//! Battle layer: what the Lua battle and skill rules act through.
//!
//! The *rules* (damage formulas, what each skill and status change does,
//! ground-skill behaviour) are reconstructed from period emulators and live
//! in `scripts/rules/` (battle.lua, skill.lua, status_change.lua). This file
//! is the part the client can confirm or that is plain bookkeeping: packets,
//! timers, where units stand, who is casting, HP/SP, the status-change
//! store and the option words the client draws from it.
//!
//! Lua reaches it through `_rt` operations (`bl`, `bl_damage`, `sc_set`,
//! `clif_*`, `su_new`, ...), wrapped by the `battle` API in battle.lua. The
//! game calls into Lua with [`Game::rule_call`]: `rules.weapon_attack`,
//! `rules.skill_use`, `rules.skill_castend`, `rules.battle_tick`.
//!
//! One rule: a host operation never calls back into a world-bound script
//! (the `_rt` binding is not re-entrant). Anything a host operation wants
//! scripted is queued with [`Game::defer_rule`] and runs when the current
//! script call returns.

use std::collections::BTreeMap;

use serde_json::{Value, json};

use crate::game::Game;
use crate::model::{InvItem, Slots, StatusChange};
use crate::packets;
use crate::path;
use crate::script::{Ctx, CtxKind};
use crate::world::{DEFAULT_SPEED, Unit, UnitKind};

// Athena status-change numbers the option words are made of (mod0631
// map/skill.h; the words themselves are the client's: opt1 1 stone 2 frozen
// 3 stunned 4 asleep; opt2 bit0 poison 1 curse 2 silence 3 confusion(unused)
// 4 blind; option 1 sight 2 hide 4 cloak).
pub const SC_STONE: u16 = 64;
pub const SC_SLEEP: u16 = 67;
pub const SC_POISON: u16 = 68;
pub const SC_BLIND: u16 = 72;
pub const SC_HIDDING: u16 = 4;
pub const SC_CLOAKING: u16 = 5;
pub const SC_SIGHT: u16 = 75;
pub const SC_RUWACH: u16 = 76;
pub const SC_ANKLE: u16 = 94;
pub const SC_TRICKDEAD: u16 = 29;
pub const SC_ENDURE: u16 = 1;

/// How often `rules.battle_tick` runs (Athena skill_unit_timer: 100 ms).
const BATTLE_TICK_MS: u64 = 100;

/// A skill being cast (Athena skilltimer + skillid/lv/target/x/y).
#[derive(Debug, Clone)]
pub struct Cast {
    pub skill: u16,
    pub lv: u16,
    pub target: u32,
    pub x: u16,
    pub y: u16,
    pub end_at: u64,
    /// Damage interrupts it (Athena state.skillcastcancel).
    pub cancelable: bool,
    /// Skill argument of a monster skill line (monster to summon, emotion).
    pub val: i64,
    /// Which line of the monster's skill list this is (-1: none).
    pub line: i32,
}

/// One cell of a ground skill. Its group and behaviour are the rules'.
#[derive(Debug, Clone)]
pub struct SkillCell {
    pub group: u32,
    /// What the client draws (0x7e safety wall, 0x7f fire wall, ...).
    pub unit_id: u8,
    pub src: u32,
}

impl SkillCell {
    /// One of the Hunter's traps (0x8f-0x99 without Venom Dust's 0x92), as
    /// the rules' `TRAPS` in skill_units.lua.
    pub fn is_trap(&self) -> bool {
        matches!(self.unit_id, 0x8f..=0x91 | 0x93..=0x99)
    }

    /// The client draws a second 0x01c9 for a unit placed by someone else
    /// hidden, whatever its kind (`0x52a9a6`, `0x5a91f0`; cleared by 0x0091).
    /// iRO hid traps so ("go off screen and come back", THREADS.md #28); for
    /// anything else that would make a Fire Wall vanish, so the server keeps
    /// such a unit on the client out of sight instead of sending it again.
    pub fn kept_for(&self, viewer: u32) -> bool {
        self.src != viewer && !self.is_trap()
    }
}

#[derive(Default)]
pub struct BattleState {
    /// Units with status changes (so expiry does not scan the world).
    pub sc_units: std::collections::BTreeSet<u32>,
    /// Rules to run once the current script call returns: (name, argument).
    pub deferred: Vec<(String, Value)>,
    /// Skill uses asked for by a script (@useskill): (unit, skill, level, target, x, y).
    pub skill_requests: Vec<(u32, u16, u16, u32, u16, u16)>,
    pub last_tick: u64,
    /// Cells an Ice Wall made unwalkable: (map, x, y).
    pub walls: Vec<(String, u16, u16)>,
}

fn arg(a: &[Value], i: usize) -> &Value {
    a.get(i).unwrap_or(&Value::Null)
}
fn int(a: &[Value], i: usize) -> i64 {
    num(arg(a, i))
}
fn num(v: &Value) -> i64 {
    match v {
        Value::Number(n) => n.as_i64().unwrap_or_else(|| n.as_f64().unwrap_or(0.0) as i64),
        Value::String(s) => s.parse().unwrap_or(0),
        Value::Bool(b) => *b as i64,
        _ => 0,
    }
}
fn text(a: &[Value], i: usize) -> String {
    match arg(a, i) {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        v => v.to_string(),
    }
}
fn field(t: &Value, k: &str) -> i64 {
    t.get(k).map(num).unwrap_or(0)
}

impl Game {
    // ------------------------------------------------------------ calling the rules

    /// Call `rules.<name>(arg)` with the world bound. Deferred rules queued
    /// during the call run afterwards.
    pub(crate) fn rule_call(&mut self, name: &str, arg: Value) -> Option<Value> {
        let engine = self.scripts.clone()?;
        if engine.busy() {
            // a host op asked for a rule: run it when the script returns
            self.battle.deferred.push((name.to_string(), arg));
            return None;
        }
        let r = engine.rule_host(self, Ctx::new(0, 0, CtxKind::Event), name, &arg);
        let out = match r {
            Ok(v) => v,
            Err(e) => {
                self.notice(format!("{e:#}"));
                None
            }
        };
        self.run_deferred();
        out
    }

    pub(crate) fn defer_rule(&mut self, name: &str, arg: Value) {
        self.battle.deferred.push((name.to_string(), arg));
    }

    /// Run what host operations queued: after a rule call, after each
    /// packet and on every tick.
    pub(crate) fn run_deferred(&mut self) {
        if self.scripts.as_ref().is_none_or(|e| e.busy()) {
            return;
        }
        for (unit, skill, lv, target, x, y) in std::mem::take(&mut self.battle.skill_requests) {
            self.skill_request(unit, skill, lv, target, x, y);
        }
        if self.battle.deferred.is_empty() {
            return;
        }
        for _ in 0..16 {
            if self.battle.deferred.is_empty() {
                break;
            }
            for (name, arg) in std::mem::take(&mut self.battle.deferred) {
                self.rule_call(&name, arg);
            }
        }
    }

    /// Every 100 ms: status changes that ran out, then the rules' own timers
    /// (ground skills, delayed hits).
    pub(crate) fn tick_battle(&mut self, now: u64) {
        self.tick_casts(now);
        if now < self.battle.last_tick + BATTLE_TICK_MS {
            return;
        }
        self.battle.last_tick = now;
        let mut due = Vec::new();
        let mut gone = Vec::new();
        for &u in &self.battle.sc_units {
            match self.sc_map(u) {
                Some(m) if !m.is_empty() => {
                    for (k, s) in m {
                        if s.until <= now {
                            due.push(json!([u, k]));
                        }
                    }
                }
                _ => gone.push(u),
            }
        }
        for u in gone {
            self.battle.sc_units.remove(&u);
        }
        if self.world.all_players().is_empty() && due.is_empty() {
            return;
        }
        self.rule_call("battle_tick", json!({"now": now, "sc_due": due}));
    }

    // ------------------------------------------------------------ status changes (store)

    pub(crate) fn sc_map(&self, unit: u32) -> Option<&BTreeMap<u16, StatusChange>> {
        let u = self.world.units.get(&unit)?;
        match &u.kind {
            UnitKind::Player(p) => self.store.chars.get(&p.char_id).map(|c| &c.sc),
            UnitKind::Mob(m) => Some(&m.sc),
            _ => None,
        }
    }

    pub(crate) fn sc_map_mut(&mut self, unit: u32) -> Option<&mut BTreeMap<u16, StatusChange>> {
        let u = self.world.units.get_mut(&unit)?;
        match &mut u.kind {
            UnitKind::Player(p) => self.store.chars.get_mut(&p.char_id).map(|c| &mut c.sc),
            UnitKind::Mob(m) => Some(&mut m.sc),
            _ => None,
        }
    }

    pub(crate) fn has_sc(&self, unit: u32, kind: u16) -> bool {
        self.sc_map(unit).is_some_and(|m| m.contains_key(&kind))
    }

    /// (opt1, opt2, option) as the client draws them.
    pub(crate) fn opts_of(&self, unit: u32) -> (u16, u16, u16) {
        let base = self.char_of_unit(unit).map(|c| c.option as u16).unwrap_or(0);
        let Some(sc) = self.sc_map(unit) else { return (0, 0, base) };
        // stone only counts once it has set (val2 = 1 while still turning: eAthena 817)
        let opt1 = (SC_STONE..=SC_SLEEP)
            .find(|k| sc.get(k).is_some_and(|s| *k != SC_STONE || s.val2 == 0))
            .map(|k| k - SC_STONE + 1)
            .unwrap_or(0);
        let mut opt2 = 0;
        for k in SC_POISON..=SC_BLIND {
            if sc.contains_key(&k) {
                opt2 |= 1 << (k - SC_POISON);
            }
        }
        let mut option = base & !7;
        if sc.contains_key(&SC_SIGHT) || sc.contains_key(&SC_RUWACH) {
            option |= 1;
        }
        if sc.contains_key(&SC_HIDDING) {
            option |= 2;
        }
        if sc.contains_key(&SC_CLOAKING) {
            option |= 4;
        }
        (opt1, opt2, option)
    }

    /// Turning to stone (eAthena 817 skill.c:5705, its opt1 6): the five
    /// seconds before Stone Curse sets. Walking goes on; attacks and skills
    /// do not. (Not sent: whether this client knows opt1 6 is THREADS.md #21.)
    pub(crate) fn turning_stone(&self, unit: u32) -> bool {
        self.sc_map(unit).is_some_and(|sc| sc.get(&SC_STONE).is_some_and(|s| s.val2 != 0))
    }

    /// Stone, frozen, stunned, asleep, or turning to stone: no attack, no skill.
    pub(crate) fn cannot_act(&self, unit: u32) -> bool {
        self.opts_of(unit).0 > 0 || self.turning_stone(unit)
    }

    /// Icon number for a status change, as mod0631 sends it (skill.c:4017):
    /// the type itself below 64, a few 2-2 ones by table.
    fn sc_icon(kind: u16) -> Option<u16> {
        match kind {
            0..=63 => Some(kind),
            86 => Some(0x56),
            87 => Some(0x57),
            99 => Some(0x44),
            98 => Some(0x5a),
            95 => Some(0x5b),
            96 => Some(0x5c),
            97 => Some(0x5d),
            _ => None,
        }
    }

    /// Store a status change (replacing one of the same type) and tell the
    /// clients: icon, option words, and the player's recomputed status.
    pub(crate) fn sc_set(&mut self, unit: u32, kind: u16, sc: StatusChange) {
        let before = self.opts_of(unit);
        let is_player = self.world.units.get(&unit).is_some_and(|u| u.is_player());
        let Some(m) = self.sc_map_mut(unit) else { return };
        let fresh = m.insert(kind, sc).is_none();
        self.battle.sc_units.insert(unit);
        if is_player && fresh {
            if let Some(icon) = Self::sc_icon(kind) {
                self.area_send(unit, packets::status_icon(icon, unit, true), true);
            }
        }
        self.after_sc_change(unit, before);
    }

    /// Only the values and the end time change (timers re-arming themselves).
    pub(crate) fn sc_touch(&mut self, unit: u32, kind: u16, f: impl FnOnce(&mut StatusChange)) {
        let before = self.opts_of(unit);
        if let Some(s) = self.sc_map_mut(unit).and_then(|m| m.get_mut(&kind)) {
            f(s);
        }
        // a value can change what is drawn (stone setting: 817 skill.c:5113)
        if self.opts_of(unit) != before {
            self.after_sc_change(unit, before);
        }
    }

    pub(crate) fn sc_end(&mut self, unit: u32, kind: u16) -> bool {
        let before = self.opts_of(unit);
        let is_player = self.world.units.get(&unit).is_some_and(|u| u.is_player());
        let had = self.sc_map_mut(unit).and_then(|m| m.remove(&kind)).is_some();
        if had {
            if is_player {
                if let Some(icon) = Self::sc_icon(kind) {
                    self.area_send(unit, packets::status_icon(icon, unit, false), true);
                }
            }
            self.after_sc_change(unit, before);
        }
        had
    }

    fn after_sc_change(&mut self, unit: u32, before: (u16, u16, u16)) {
        let now = self.opts_of(unit);
        if now != before {
            let karma = self.char_of_unit(unit).map(|c| c.karma as u8).unwrap_or(0);
            self.area_send(unit, packets::state_change(unit, now.0, now.1, now.2, karma), true);
            if now.0 != 0 || now.2 & 6 != 0 {
                // stone, frozen, stunned, asleep, hidden: stop walking and attacking
                self.stop_walking(unit);
                self.stop_attack(unit);
            }
        }
        if self.world.units.get(&unit).is_some_and(|u| u.is_player()) {
            self.refresh_status(unit);
        }
    }

    /// Death and logout end every status change (Athena status_change_clear).
    pub(crate) fn sc_clear(&mut self, unit: u32) {
        let all: Vec<u16> = self.sc_map(unit).map(|m| m.keys().copied().collect()).unwrap_or_default();
        for k in all {
            self.sc_end(unit, k);
        }
    }

    /// Recompute a player's status after anything that feeds it changed:
    /// walk speed on the unit, the status window, HP/SP caps.
    pub(crate) fn refresh_status(&mut self, unit: u32) {
        let Some(c) = self.char_of_unit(unit).cloned() else { return };
        let d = self.derive(&c);
        if let Some(cm) = self.char_of_unit_mut(unit) {
            cm.hp = cm.hp.min(d.max_hp);
            cm.sp = cm.sp.min(d.max_sp);
        }
        if let Some(u) = self.world.units.get_mut(&unit) {
            u.speed = d.speed.clamp(20, 1000) as u16;
        }
        self.send_full_status(unit);
        if let Some(sid) = self.sid_of(unit) {
            self.send(sid, packets::attack_range(d.range as u16));
        }
    }

    // ------------------------------------------------------------ moving and stopping

    pub(crate) fn stop_walking(&mut self, unit: u32) {
        let Some(u) = self.world.units.get_mut(&unit) else { return };
        if u.walk.take().is_some() {
            let (x, y) = (u.x, u.y);
            self.area_send(unit, packets::stop_move(unit, x, y), true);
        }
    }

    pub(crate) fn stop_attack(&mut self, unit: u32) {
        if let Some(u) = self.world.units.get_mut(&unit) {
            match &mut u.kind {
                UnitKind::Player(p) => p.attack = None,
                UnitKind::Mob(m) => m.target = None,
                _ => {}
            }
        }
    }

    /// May this unit walk right now? (clif_parse_WalkToXY / mob.c:774,1079)
    pub(crate) fn can_move(&self, unit: u32) -> bool {
        let Some(u) = self.world.units.get(&unit) else { return false };
        if u.state == 1 {
            return false;
        }
        let now = self.now();
        let (opt1, _, option) = self.opts_of(unit);
        // (playing dead: ours. clif_parse_WalkToXY does not check it, so in
        // mod0631 one could walk about while "dead")
        if opt1 > 0 || self.has_sc(unit, SC_ANKLE) || self.has_sc(unit, SC_TRICKDEAD) {
            return false;
        }
        match &u.kind {
            UnitKind::Player(p) => {
                // SA_FREECAST (278) lets a Sage walk while casting; RG_TUNNELDRIVE (213) while hidden
                let skill = |id: u16| self.store.chars.get(&p.char_id).and_then(|c| c.skills.get(&id)).copied().unwrap_or(0);
                if p.cast.is_some() && skill(278) == 0 {
                    return false;
                }
                if p.canmove > now {
                    return false;
                }
                if option & 2 != 0 && skill(213) == 0 {
                    return false;
                }
                true
            }
            UnitKind::Mob(m) => m.cast.is_none(),
            _ => true,
        }
    }

    /// Push a unit up to `count` cells in direction (dx, dy), stopping at the
    /// first blocked cell (Athena skill_blown / path_blownpos). Returns the
    /// cell it ends on.
    pub(crate) fn blow(&mut self, unit: u32, dx: i32, dy: i32, count: i32) -> (u16, u16) {
        let Some(u) = self.world.units.get(&unit) else { return (0, 0) };
        let (map, mut x, mut y) = (u.map.clone(), u.x as i32, u.y as i32);
        let Some(grid) = self.grid(&map) else { return (x as u16, y as u16) };
        for _ in 0..count.max(0) {
            let (nx, ny) = (x + dx, y + dy);
            let ok = grid.walkable(nx, ny) && (dx == 0 || dy == 0 || (grid.walkable(nx, y) && grid.walkable(x, ny)));
            if !ok {
                break;
            }
            (x, y) = (nx, ny);
        }
        let u = self.world.units.get_mut(&unit).unwrap();
        u.walk = None;
        u.x = x as u16;
        u.y = y as u16;
        if u.is_player() {
            self.refresh_view(unit, false);
        } else {
            self.update_watchers(unit, false);
        }
        (x as u16, y as u16)
    }

    // ------------------------------------------------------------ damage and healing

    /// Take HP off a player or monster (Athena battle_damage). Returns true
    /// when it died.
    pub(crate) fn bl_damage(&mut self, src: u32, target: u32, damage: u32) -> bool {
        if damage == 0 {
            return false;
        }
        let Some(t) = self.world.units.get(&target) else { return false };
        if t.state == 1 {
            return false;
        }
        match &t.kind {
            UnitKind::Mob(m) => {
                if m.cast.as_ref().is_some_and(|c| c.cancelable) {
                    self.cast_cancel(target);
                }
                self.damage_mob(target, src, damage)
            }
            UnitKind::Player(p) => {
                // a Phen Card (4077) keeps the cast going (battle.c:556)
                if p.cast.as_ref().is_some_and(|c| c.cancelable) && !self.has_card(target, 4077) {
                    self.cast_cancel(target);
                }
                self.damage_player(target, damage)
            }
            _ => false,
        }
    }

    /// Athena pc_damage: stand up, stop walking unless enduring, lose HP, die.
    fn damage_player(&mut self, target: u32, damage: u32) -> bool {
        let endure = self.has_sc(target, SC_ENDURE);
        if let Some(u) = self.world.units.get_mut(&target) {
            if u.state == 2 {
                u.state = 0;
                let tick = self.tick32();
                self.area_send(target, packets::action(target, 0, tick, 0, 0, 0, 0, 3, 0), true);
            }
        }
        if !endure {
            // pc.c:2249 pc_stop_walking(sd, 3): stopped, and held for the damage motion
            self.stop_walking(target);
            let dmotion = self.char_of_unit(target).map(|c| self.derive(c).dmotion as u64).unwrap_or(0);
            let now = self.now();
            if let Some(p) = self.world.units.get_mut(&target).and_then(|u| u.player_mut()) {
                p.canmove = p.canmove.max(now + dmotion);
            }
        }
        let dead = {
            let Some(c) = self.char_of_unit_mut(target) else { return false };
            c.hp = c.hp.saturating_sub(damage);
            c.hp == 0
        };
        if let Some(sid) = self.sid_of(target) {
            let hp = self.char_of_unit(target).map(|c| c.hp).unwrap_or(0);
            self.send(sid, packets::par(crate::status::SP_HP, hp));
        }
        if dead {
            self.player_dies(target);
        } else {
            self.defer_rule("pc_damaged", json!({"bl": target}));
        }
        dead
    }

    /// Give HP/SP to a player or HP to a monster; returns the HP actually healed.
    pub(crate) fn bl_heal(&mut self, target: u32, hp: i64, sp: i64) -> i64 {
        let Some(t) = self.world.units.get_mut(&target) else { return 0 };
        if t.state == 1 {
            return 0;
        }
        match &mut t.kind {
            UnitKind::Mob(m) => {
                let before = m.hp;
                m.hp = (m.hp as i64 + hp).clamp(1, m.max_hp as i64) as u32;
                m.hp as i64 - before as i64
            }
            UnitKind::Player(_) => {
                let Some(c) = self.char_of_unit(target).cloned() else { return 0 };
                let d = self.derive(&c);
                let (nh, ns) = ((c.hp as i64 + hp).clamp(0, d.max_hp as i64) as u32, (c.sp as i64 + sp).clamp(0, d.max_sp as i64) as u32);
                if let Some(cm) = self.char_of_unit_mut(target) {
                    cm.hp = nh.max(1);
                    cm.sp = ns;
                }
                if let Some(sid) = self.sid_of(target) {
                    self.send(sid, packets::par(crate::status::SP_HP, nh.max(1)));
                    self.send(sid, packets::par(crate::status::SP_SP, ns));
                }
                nh as i64 - c.hp as i64
            }
            _ => 0,
        }
    }

    /// Is this card in any worn equipment?
    pub(crate) fn has_card(&self, unit: u32, card: u16) -> bool {
        self.char_of_unit(unit).is_some_and(|c| c.inventory.iter().any(|i| i.equip != 0 && i.cards[0] != 0x00ff && i.cards.contains(&card)))
    }

    // ------------------------------------------------------------ casting

    /// Client asked to use a skill (0x0113 on a target, 0x0116 on a cell).
    pub(crate) fn skill_request(&mut self, unit: u32, skill: u16, lv: u16, target: u32, x: u16, y: u16) {
        let Some(sid) = self.sid_of(unit) else { return };
        let now = self.now();
        let Some(p) = self.world.units.get(&unit).and_then(|u| u.player()) else { return };
        if p.canact > now || p.cast.is_some() {
            // clif_parse_UseSkillToId: "skill delay" failure
            self.send(sid, packets::skill_fail(skill, 0, 4));
            return;
        }
        let item = p.skill_item;
        if self.dialogs.contains_key(&unit) {
            return;
        }
        self.end_ghost(unit); // clif.c:5414, 5446
        let known = self.skill_level(unit, skill) as u16;
        let lv = match item {
            Some((s, l)) if s == skill => lv.min(l).max(1),
            _ if known > 0 => lv.min(known).max(1),
            _ => return,
        };
        let from_item = item.is_some_and(|(s, _)| s == skill);
        let r = self.rule_call("skill_use", json!({"src": unit, "skill": skill, "lv": lv, "target": target, "x": x, "y": y, "item": from_item}));
        let Some(r) = r.filter(|v| v.is_object()) else { return };
        self.begin_cast(unit, skill, lv, field(&r, "target") as u32, x, y, field(&r, "cast") as u64, field(&r, "delay") as u64, r.get("cancelable").and_then(Value::as_bool).unwrap_or(true), r.get("forcecast").and_then(Value::as_bool).unwrap_or(false), 0, -1);
    }

    /// Start a cast decided by `rules.skill_use` / the monster skill rules:
    /// casting bar (0x013e), timers, and the cast end when there is no cast time.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn begin_cast(&mut self, unit: u32, skill: u16, lv: u16, target: u32, x: u16, y: u16, cast_ms: u64, delay_ms: u64, cancelable: bool, forcecast: bool, val: i64, line: i32) {
        let now = self.now();
        let element = self.db.skill(skill).map(|s| s.element as u32).unwrap_or(0);
        // The client shouts the skill's name over a caster and says "Unknown
        // Skill !!" for one its name table lacks: every monster-only skill
        // (an Ant Egg hatching) and every 2-2 skill on this client. Those
        // cast without the bar; the cast still takes its time (ours: what
        // the official server sent for them is not known, THREADS.md #23).
        let named = self.res.skill_names.is_empty() || self.db.skill(skill).is_some_and(|s| self.res.skill_names.contains(&s.name));
        if (cast_ms > 0 || forcecast) && named {
            self.area_send(unit, packets::skill_casting(unit, target, x, y, skill, element, cast_ms as u32), true);
        }
        let cast = Cast { skill, lv, target, x, y, end_at: now + cast_ms, cancelable: cancelable && cast_ms > 0, val, line };
        // a player stops to cast (but a Sage with Free Cast); a monster's walk runs on
        if self.world.units.get(&unit).is_some_and(|u| u.is_player()) && self.skill_level(unit, 278) == 0 {
            self.stop_walking(unit);
        }
        if let Some(u) = self.world.units.get_mut(&unit) {
            match &mut u.kind {
                UnitKind::Player(p) => {
                    p.attack = if matches!(skill, 272 | 273) { p.attack } else { None }; // combo skills keep the target
                    p.canact = now + cast_ms + delay_ms;
                    p.canmove = now;
                    p.cast = Some(cast);
                }
                UnitKind::Mob(m) => {
                    m.canact = now + cast_ms + delay_ms;
                    m.cast = Some(cast);
                }
                _ => return,
            }
        }
        if cast_ms == 0 {
            self.finish_cast(unit);
        }
    }

    fn tick_casts(&mut self, now: u64) {
        let due: Vec<u32> = self
            .world
            .units
            .values()
            .filter(|u| match &u.kind {
                UnitKind::Player(p) => p.cast.as_ref().is_some_and(|c| c.end_at <= now),
                UnitKind::Mob(m) => m.cast.as_ref().is_some_and(|c| c.end_at <= now),
                _ => false,
            })
            .map(|u| u.id)
            .collect();
        for id in due {
            self.finish_cast(id);
        }
    }

    fn take_cast(&mut self, unit: u32) -> Option<Cast> {
        match &mut self.world.units.get_mut(&unit)?.kind {
            UnitKind::Player(p) => p.cast.take(),
            UnitKind::Mob(m) => m.cast.take(),
            _ => None,
        }
    }

    fn finish_cast(&mut self, unit: u32) {
        let Some(c) = self.take_cast(unit) else { return };
        // mob.c:1936, 1945: after a skill on a target the monster rests for
        // one attack delay, and the line's own delay starts again here
        let now = self.now();
        let adelay = self.world.units.get(&unit).and_then(|u| u.mob()).and_then(|m| self.db.mob(m.mob_id)).map(|d| d.attack_delay as u64);
        if let (Some(adelay), Some(m)) = (adelay, self.world.units.get_mut(&unit).and_then(|u| u.mob_mut())) {
            if c.target != 0 {
                m.next_think = m.next_think.max(now + adelay);
            }
            if let Some(used) = usize::try_from(c.line).ok().and_then(|i| m.skill_used.get_mut(i)) {
                *used = now.max(1);
            }
        }
        let from_item = self.world.units.get(&unit).and_then(|u| u.player()).and_then(|p| p.skill_item).is_some_and(|(s, _)| s == c.skill);
        if from_item {
            if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
                p.skill_item = None;
            }
        }
        let r = self.rule_call(
            "skill_castend",
            json!({"src": unit, "skill": c.skill, "lv": c.lv, "target": c.target, "x": c.x, "y": c.y, "item": from_item, "val": c.val}),
        );
        // a failed use gives the delay back (skill_castend_id: canact_tick = tick)
        if r == Some(Value::Bool(false)) {
            let now = self.now();
            if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
                p.canact = now;
                p.canmove = now;
            }
        }
    }

    /// Athena skill_castcancel.
    pub(crate) fn cast_cancel(&mut self, unit: u32) {
        if self.take_cast(unit).is_some() {
            let now = self.now();
            if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
                p.canact = now;
                p.canmove = now;
            }
            self.area_send(unit, packets::skill_cast_cancel(unit), true);
        }
    }

    // ------------------------------------------------------------ ground skill cells

    fn su_new(&mut self, map: &str, x: u16, y: u16, unit_id: u8, src: u32, group: u32) -> u32 {
        let id = self.world.new_obj_id();
        self.world.add(Unit {
            id,
            map: map.into(),
            x,
            y,
            dir: 0,
            head_dir: 0,
            speed: DEFAULT_SPEED,
            class: unit_id as u16,
            name: String::new(),
            state: 0,
            walk: None,
            kind: UnitKind::Skill(SkillCell { group, unit_id, src }),
        });
        self.update_watchers(id, true);
        id
    }

    /// Make a cell (un)walkable for an Ice Wall and tell the clients (0x0192).
    fn set_cell(&mut self, map: &str, x: u16, y: u16, walkable: bool) {
        let Some(m) = self.world.maps.get_mut(map) else { return };
        let mut g = (*m.grid).clone();
        g.set_walkable(x as i32, y as i32, walkable);
        m.grid = std::sync::Arc::new(g);
        self.battle.walls.retain(|w| !(w.0 == map && w.1 == x && w.2 == y));
        if !walkable {
            self.battle.walls.push((map.to_string(), x, y));
        }
        // A freed cell gets its own type back (3 for water), as Athena sends
        // the gat's; the client stores whatever it is given (0x518cd0).
        let freed = if m.grid.flags(x as i32, y as i32) & ro_data::gat::WATER != 0 { 3 } else { 0 };
        let pkt = packets::change_map_cell(x, y, if walkable { freed } else { 5 }, map);
        for pid in self.world.players_on(map) {
            if let Some(sid) = self.sid_of(pid) {
                self.send(sid, pkt.clone());
            }
        }
    }

    /// Ice Wall cells on `map`, for a client that just loaded it.
    pub(crate) fn send_walls(&self, unit: u32) {
        let (Some(sid), Some(u)) = (self.sid_of(unit), self.world.units.get(&unit)) else { return };
        for (m, x, y) in &self.battle.walls {
            if *m == u.map {
                self.send(sid, packets::change_map_cell(*x, *y, 5, m));
            }
        }
    }

    // ------------------------------------------------------------ what Lua sees of a unit

    fn sc_json(&self, unit: u32) -> Value {
        let now = self.now();
        let m: serde_json::Map<String, Value> = self
            .sc_map(unit)
            .map(|m| {
                m.iter()
                    .map(|(k, s)| (k.to_string(), json!({"val1": s.val1, "val2": s.val2, "val3": s.val3, "val4": s.val4, "left": s.until.saturating_sub(now)})))
                    .collect()
            })
            .unwrap_or_default();
        Value::Object(m)
    }

    /// Snapshot of a player or monster for the rules (Athena block_list +
    /// map_session_data / mob_data).
    pub(crate) fn bl_json(&self, id: u32) -> Value {
        let Some(u) = self.world.units.get(&id) else { return Value::Null };
        let (opt1, opt2, option) = self.opts_of(id);
        let opt1 = if self.turning_stone(id) { 6 } else { opt1 };
        let now = self.now();
        let mut o = json!({
            "id": id, "map": u.map, "x": u.x, "y": u.y, "dir": u.dir, "dead": u.state == 1, "sit": u.state == 2,
            "walking": u.walk.is_some(), "speed": u.speed, "opt1": opt1, "opt2": opt2, "option": option,
            "sc": self.sc_json(id), "now": now,
        });
        let m = o.as_object_mut().unwrap();
        match &u.kind {
            UnitKind::Player(p) => {
                let Some(c) = self.store.chars.get(&p.char_id) else { return Value::Null };
                m.insert("kind".into(), json!("player"));
                m.insert("name".into(), json!(c.name));
                m.insert("char_id".into(), json!(c.id));
                m.insert("sex".into(), json!(p.sex));
                m.insert("hp".into(), json!(c.hp));
                m.insert("sp".into(), json!(c.sp));
                m.insert("casting".into(), json!(p.cast.as_ref().map(|c| c.skill)));
                m.insert("canact".into(), json!(p.canact.saturating_sub(now)));
                m.insert("attacking".into(), json!(p.attack.map(|a| a.target)));
                m.insert("char".into(), self.char_lua(c));
            }
            UnitKind::Mob(mob) => {
                let Some(d) = self.db.mob(mob.mob_id) else { return Value::Null };
                m.insert("kind".into(), json!("mob"));
                m.insert("name".into(), json!(d.display));
                m.insert("hp".into(), json!(mob.hp));
                m.insert("max_hp".into(), json!(mob.max_hp));
                m.insert("class".into(), json!(mob.mob_id));
                m.insert("def_ele".into(), json!(mob.def_ele));
                m.insert("master".into(), json!(mob.master));
                m.insert("target".into(), json!(mob.target));
                m.insert("casting".into(), json!(mob.cast.as_ref().map(|c| c.skill)));
                m.insert("state".into(), json!(mob.skill_state.name()));
                m.insert("db".into(), serde_json::to_value(d).unwrap_or(Value::Null));
            }
            _ => return Value::Null,
        }
        o
    }

    // ------------------------------------------------------------ the operations

    /// `_rt` operations of the battle API. None = not one of ours.
    pub(crate) fn battle_call(&mut self, _ctx: &Ctx, op: &str, a: &[Value]) -> Option<Result<Value, String>> {
        let id = |i: usize| int(a, i) as u32;
        Some(Ok(match op {
            "bl" => self.bl_json(id(0)),
            "now" => json!(self.now()),
            "bl_damage" => json!(self.bl_damage(id(0), id(1), int(a, 2).max(0) as u32)),
            "bl_heal" => json!(self.bl_heal(id(0), int(a, 1), int(a, 2))),
            "bl_set" => {
                let (unit, key, v) = (id(0), text(a, 1), int(a, 2));
                match key.as_str() {
                    "sp" | "hp" | "zeny" => {
                        if let Some(c) = self.char_of_unit_mut(unit) {
                            match key.as_str() {
                                "sp" => c.sp = v.max(0) as u32,
                                "hp" => c.hp = v.max(1) as u32,
                                _ => c.zeny = v.clamp(0, u32::MAX as i64) as u32,
                            }
                            let (hp, sp, zeny) = (c.hp, c.sp, c.zeny);
                            if let Some(sid) = self.sid_of(unit) {
                                self.send(sid, packets::par(crate::status::SP_HP, hp));
                                self.send(sid, packets::par(crate::status::SP_SP, sp));
                                if key == "zeny" {
                                    self.send(sid, packets::long_par(crate::status::SP_ZENY, zeny));
                                }
                            }
                        } else if let Some(m) = self.world.units.get_mut(&unit).and_then(|u| u.mob_mut()) {
                            if key == "hp" {
                                m.hp = (v.max(1) as u32).min(m.max_hp);
                            }
                        }
                    }
                    "def_ele" => {
                        if let Some(m) = self.world.units.get_mut(&unit).and_then(|u| u.mob_mut()) {
                            m.def_ele = Some(v as u16);
                        }
                    }
                    "dir" => {
                        if let Some(u) = self.world.units.get_mut(&unit) {
                            u.dir = (v & 7) as u8;
                        }
                    }
                    "canact" | "canmove" => {
                        let at = self.now() + v.max(0) as u64;
                        if let Some(u) = self.world.units.get_mut(&unit) {
                            match &mut u.kind {
                                UnitKind::Player(p) if key == "canact" => p.canact = at,
                                UnitKind::Player(p) => p.canmove = at,
                                UnitKind::Mob(m) if key == "canact" => m.canact = at,
                                _ => {}
                            }
                        }
                    }
                    other => return Some(Err(format!("bl_set: unknown key {other}"))),
                }
                Value::Null
            }
            "bl_stand" => {
                // back on its feet (Resurrection): hp given by the caller
                let unit = id(0);
                if let Some(u) = self.world.units.get_mut(&unit) {
                    u.state = 0;
                }
                if let Some(c) = self.char_of_unit_mut(unit) {
                    c.hp = int(a, 1).max(1) as u32;
                }
                self.set_ghost(unit); // skill.c:1219
                self.area_send(unit, packets::resurrection(unit, 0), true);
                self.send_hp_sp(unit);
                Value::Null
            }
            "in_area" => {
                // players and monsters alive in a rectangle
                let map = text(a, 0);
                let (x0, y0, x1, y1) = (int(a, 1), int(a, 2), int(a, 3), int(a, 4));
                let ids: Vec<u32> = self
                    .world
                    .maps
                    .get(&map)
                    .map(|m| {
                        m.units
                            .iter()
                            .filter_map(|i| self.world.units.get(i))
                            .filter(|u| u.is_fighter() && u.state != 1)
                            .filter(|u| (x0..=x1).contains(&(u.x as i64)) && (y0..=y1).contains(&(u.y as i64)))
                            .map(|u| u.id)
                            .collect()
                    })
                    .unwrap_or_default();
                json!(ids)
            }
            "cell" => {
                // 0 walkable, 1 wall, 3 water, 5 shoot-over only
                let map = text(a, 0);
                match self.grid(&map) {
                    Some(g) => {
                        let f = g.flags(int(a, 1) as i32, int(a, 2) as i32);
                        json!(if f & ro_data::gat::WATER != 0 { 3 } else if f & ro_data::gat::WALK != 0 { 0 } else if f & ro_data::gat::SHOOT != 0 { 5 } else { 1 })
                    }
                    None => json!(1),
                }
            }
            "set_cell" => {
                self.set_cell(&text(a, 0), int(a, 1) as u16, int(a, 2) as u16, arg(a, 3).as_bool().unwrap_or(true));
                Value::Null
            }
            "map_flag" => json!(self.db.map_flag(&text(a, 0), &text(a, 1))),
            "can_shoot" => {
                // a free straight line between two cells (battle_check_range)
                let map = text(a, 0);
                json!(self.grid(&map).is_some_and(|g| path::can_shoot(&g, int(a, 1) as i32, int(a, 2) as i32, int(a, 3) as i32, int(a, 4) as i32)))
            }
            "reachable" => {
                let map = text(a, 0);
                json!(self.grid(&map).is_some_and(|g| path::find(&g, int(a, 1) as i32, int(a, 2) as i32, int(a, 3) as i32, int(a, 4) as i32).is_some()))
            }
            // ---- status changes
            "sc_set" => {
                let now = self.now();
                let sc = StatusChange { until: now + int(a, 6).max(0) as u64, val1: int(a, 2), val2: int(a, 3), val3: int(a, 4), val4: int(a, 5) };
                self.sc_set(id(0), int(a, 1) as u16, sc);
                Value::Null
            }
            "sc_touch" => {
                // re-arm: new values (nil keeps) and time
                let now = self.now();
                let (v1, v2, v3, ms, v4) = (arg(a, 2).clone(), arg(a, 3).clone(), arg(a, 4).clone(), int(a, 5).max(0) as u64, arg(a, 6).clone());
                self.sc_touch(id(0), int(a, 1) as u16, |s| {
                    if !v1.is_null() {
                        s.val1 = num(&v1);
                    }
                    if !v2.is_null() {
                        s.val2 = num(&v2);
                    }
                    if !v3.is_null() {
                        s.val3 = num(&v3);
                    }
                    if !v4.is_null() {
                        s.val4 = num(&v4);
                    }
                    s.until = now + ms;
                });
                Value::Null
            }
            "sc_del" => json!(self.sc_end(id(0), int(a, 1) as u16)),
            "sc_clear" => {
                self.sc_clear(id(0));
                Value::Null
            }
            // ---- packets
            "clif_damage" => {
                // src, dst, sdelay, ddelay, damage, div, type, damage2
                let tick = self.tick32();
                let pkt = packets::action(id(0), id(1), tick, int(a, 2) as u32, int(a, 3) as u32, int(a, 4).clamp(0, 32000) as u16, int(a, 5) as u16, int(a, 6) as u8, int(a, 7).clamp(0, 32000) as u16);
                self.area_send(id(0), pkt, true);
                Value::Null
            }
            "clif_skill_damage" | "clif_skill_damage2" => {
                // src, dst, sdelay, ddelay, damage, div, skill, lv, type
                let (src, dst) = (id(0), id(1));
                let tick = self.tick32();
                let mut kind = int(a, 8);
                if kind <= 0 {
                    kind = self.db.skill(int(a, 6) as u16).map(|s| s.hit as i64).unwrap_or(6);
                }
                if self.has_sc(dst, SC_ENDURE) && self.world.units.get(&dst).is_some_and(|u| u.is_player()) {
                    kind = 9;
                }
                let pkt = if op == "clif_skill_damage" {
                    packets::skill_damage(int(a, 6) as u16, src, dst, tick, int(a, 2) as u32, int(a, 3) as u32, int(a, 4) as i32, int(a, 7) as u16, int(a, 5) as u16, kind as u8)
                } else {
                    let (x, y) = self.world.units.get(&dst).map(|u| (u.x, u.y)).unwrap_or((0, 0));
                    packets::skill_damage_blown(int(a, 6) as u16, src, dst, tick, int(a, 2) as u32, int(a, 3) as u32, x, y, int(a, 4).clamp(0, 32000) as u16, int(a, 7) as u16, int(a, 5) as u16, kind as u8)
                };
                // a ground cell is no viewer anchor: send around the target
                let anchor = if self.world.units.get(&src).is_some_and(|u| u.is_fighter()) { src } else { dst };
                self.area_send(anchor, pkt.clone(), true);
                if anchor == src && dst != src {
                    // the target sees its own hit even when the source is out of its view
                    if let Some(sid) = self.sid_of(dst) {
                        if !self.world.units.get(&dst).and_then(|u| u.player()).is_some_and(|p| p.visible.contains(&src)) {
                            self.send(sid, pkt);
                        }
                    }
                }
                Value::Null
            }
            "clif_skill_nodamage" => {
                // src, dst, skill, heal, fail
                let pkt = packets::skill_nodamage(int(a, 2) as u16, int(a, 3).clamp(0, 32000) as u16, id(1), id(0), int(a, 4) as u8);
                let anchor = if self.world.units.get(&id(0)).is_some_and(|u| u.is_fighter()) { id(0) } else { id(1) };
                self.area_send(anchor, pkt, true);
                Value::Null
            }
            "clif_skill_poseffect" => {
                let tick = self.tick32();
                self.area_send(id(0), packets::skill_poseffect(int(a, 1) as u16, id(0), int(a, 2) as u16, int(a, 3) as u16, int(a, 4) as u16, tick), true);
                Value::Null
            }
            "clif_skill_fail" => {
                if let Some(sid) = self.sid_of(id(0)) {
                    self.send(sid, packets::skill_fail(int(a, 1) as u16, int(a, 3) as u16, int(a, 2) as u8));
                }
                Value::Null
            }
            "clif_heal" => {
                if let Some(sid) = self.sid_of(id(0)) {
                    self.send(sid, packets::heal_notice(int(a, 1) as u16, int(a, 2).clamp(0, 32000) as u16));
                }
                Value::Null
            }
            "clif_emotion" => {
                self.area_send(id(0), packets::emotion(id(0), int(a, 1) as u8), true);
                Value::Null
            }
            "clif_misceffect" => {
                self.area_send(id(0), packets::misc_effect(id(0), int(a, 1) as u32), true);
                Value::Null
            }
            "clif_fixpos" => {
                if let Some((x, y)) = self.world.units.get(&id(0)).map(|u| (u.x, u.y)) {
                    self.area_send(id(0), packets::stop_move(id(0), x, y), true);
                }
                Value::Null
            }
            "clif_estimation" => {
                // WZ_ESTIMATION (clif_skill_estimation): elements = damage % of each attack element 1..9
                if let (Some(sid), Some(m)) = (self.sid_of(id(0)), self.world.units.get(&id(1)).and_then(|u| u.mob())) {
                    if let Some(d) = self.db.mob(m.mob_id) {
                        let fix: Vec<u8> = arg(a, 2).as_array().map(|v| v.iter().map(|x| num(x).clamp(0, 255) as u8).collect()).unwrap_or_default();
                        self.send(sid, packets::estimation(d.id as u16, d.level, d.size as u16, m.hp, d.def.min(100) as u16, d.race as u16, d.mdef.min(99) as u16, (d.element % 10) as u16, &fix));
                    }
                }
                Value::Null
            }
            "clif_warppoint" => {
                // Teleport / Warp Portal destination list (0x011c); remembers the pending skill
                let unit = id(0);
                let maps: Vec<String> = arg(a, 3).as_array().map(|v| v.iter().map(|x| x.as_str().unwrap_or("").to_string()).collect()).unwrap_or_default();
                if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
                    p.skill_menu = Some((int(a, 1) as u16, int(a, 2) as u16, int(a, 4) as u16, int(a, 5) as u16));
                    let sid = p.sid;
                    self.send(sid, packets::skill_warp_point(int(a, 1) as u16, &maps));
                }
                Value::Null
            }
            "clif_map_info" => {
                if let Some(sid) = self.sid_of(id(0)) {
                    self.send(sid, packets::map_info(int(a, 1) as u16));
                }
                Value::Null
            }
            "clif_message" => {
                self.tell(id(0), &text(a, 1));
                Value::Null
            }
            // ---- units
            "blow" => {
                let (x, y) = self.blow(id(0), int(a, 1).signum() as i32, int(a, 2).signum() as i32, int(a, 3) as i32);
                json!([x, y])
            }
            "stop_walking" => {
                self.stop_walking(id(0));
                Value::Null
            }
            "stop_attack" => {
                self.stop_attack(id(0));
                Value::Null
            }
            "cast_cancel" => {
                self.cast_cancel(id(0));
                Value::Null
            }
            "walk_to" => json!(self.walk_to(id(0), int(a, 1) as u16, int(a, 2) as u16)),
            "move_pos" => {
                // instant move on the same map (Athena pc_movepos)
                let unit = id(0);
                if let Some(u) = self.world.units.get_mut(&unit) {
                    u.walk = None;
                    u.x = int(a, 1) as u16;
                    u.y = int(a, 2) as u16;
                    let player = u.is_player();
                    self.area_send(unit, packets::stop_move(unit, int(a, 1) as u16, int(a, 2) as u16), true);
                    if player {
                        self.refresh_view(unit, false);
                    } else {
                        self.update_watchers(unit, false);
                    }
                }
                Value::Null
            }
            "su_new" => json!(self.su_new(&text(a, 0), int(a, 1) as u16, int(a, 2) as u16, int(a, 3) as u8, id(4), id(5))),
            "su_reveal" => {
                if self.world.units.get(&id(0)).and_then(|u| u.skill()).is_some_and(|s| s.is_trap()) {
                    self.area_send(id(0), packets::skill_unit_reveal(id(0)), false);
                }
                Value::Null
            }
            "su_del" => {
                if self.world.units.get(&id(0)).is_some_and(|u| u.skill().is_some()) {
                    self.despawn(id(0), 0);
                }
                Value::Null
            }
            // ---- players
            "pc_warp" => {
                let unit = id(0);
                let map = text(a, 1);
                json!(if map.is_empty() || map == "Random" {
                    let here = self.world.units.get(&unit).map(|u| u.map.clone()).unwrap_or_default();
                    self.warp_player(unit, &here, 0, 0)
                } else if map == "SavePoint" {
                    match self.char_of_unit(unit).map(|c| c.save_point.clone()) {
                        Some(p) => self.warp_player(unit, &p.map, p.x, p.y),
                        None => false,
                    }
                } else {
                    self.warp_player(unit, map.trim_end_matches(".gat"), int(a, 2) as u16, int(a, 3) as u16)
                })
            }
            "pc_item_count" => json!(self.count_item(id(0), int(a, 1) as u32)),
            "pc_equip" => {
                // wear the first unworn item of this id, as a double click in the
                // inventory would (the @equip command): true if it went on
                let unit = id(0);
                let want = int(a, 1) as u32;
                let idx = self.char_of_unit(unit).and_then(|c| c.inventory.iter().position(|it| it.nameid == want && it.equip == 0));
                match idx {
                    Some(i) => {
                        self.equip(unit, i as u16 + packets::INV_OFFSET);
                        json!(self.char_of_unit(unit).is_some_and(|c| c.inventory.slot(i).is_some_and(|it| it.equip != 0)))
                    }
                    None => json!(false),
                }
            }
            "pc_item_take" => json!(self.take_item(id(0), int(a, 1) as u32, int(a, 2).max(1) as u32)),
            "pc_item_add" => {
                // unit, item, amount, cards, identified. Falls to the floor when
                // it cannot be carried (skill.c AL_HOLYWATER)
                let unit = id(0);
                let mut item = InvItem::new(int(a, 1) as u32, int(a, 2).max(1) as u32);
                if let Some(cards) = arg(a, 3).as_array() {
                    for (i, c) in cards.iter().take(4).enumerate() {
                        item.cards[i] = num(c) as u16;
                    }
                }
                if arg(a, 4).as_bool() == Some(false) {
                    item.identify = false;
                }
                if !self.add_item(unit, item.clone()) {
                    if let Some((map, x, y)) = self.world.units.get(&unit).map(|u| (u.map.clone(), u.x, u.y)) {
                        self.drop_floor_item(&map, x, y, item, true);
                    }
                }
                Value::Null
            }
            "pc_gain_exp" => {
                self.gain_exp(id(0), int(a, 1).max(0) as u64, int(a, 2).max(0) as u64);
                Value::Null
            }
            "pc_memo" => json!(self.char_of_unit(id(0)).map(|c| c.memo.iter().map(|p| json!({"map": p.map, "x": p.x, "y": p.y})).collect::<Vec<_>>())),
            "item_skill" => {
                // an item lends a skill for one use (clif_item_skill): id, lv, name, inf, sp, range
                let unit = _ctx.player;
                if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
                    p.skill_item = Some((int(a, 0) as u16, int(a, 1) as u16));
                    let sid = p.sid;
                    self.send(sid, packets::item_skill(int(a, 0) as u16, int(a, 3) as u16, int(a, 1) as u16, int(a, 4) as u16, int(a, 5) as u16, &text(a, 2)));
                }
                Value::Null
            }
            "pc_save_point" => json!(self.char_of_unit(id(0)).map(|c| json!({"map": c.save_point.map, "x": c.save_point.x, "y": c.save_point.y}))),
            "skill_request" => {
                // use a skill as if the client had asked (0x0113 / 0x0116): unit, skill, lv, target, x, y
                self.battle.skill_requests.push((id(0), int(a, 1) as u16, int(a, 2) as u16, id(3), int(a, 4) as u16, int(a, 5) as u16));
                Value::Null
            }
            "pc_produce_list" => {
                // forge window (0x018d): the items that can be made now
                let unit = id(0);
                let ids: Vec<u16> = arg(a, 1).as_array().map(|v| v.iter().map(|x| num(x) as u16).collect()).unwrap_or_default();
                let cid = self.char_of_unit(unit).map(|c| c.id).unwrap_or(0);
                if let Some(sid) = self.sid_of(unit) {
                    self.send(sid, packets::produce_list(&ids, cid));
                }
                Value::Null
            }
            "clif_produceeffect" => {
                let (unit, cid, name) = (id(0), self.char_of_unit(id(0)).map(|c| c.id).unwrap_or(0), self.char_of_unit(id(0)).map(|c| c.name.clone()).unwrap_or_default());
                if let Some(sid) = self.sid_of(unit) {
                    self.send(sid, packets::char_name(cid, &name));
                    self.send(sid, packets::produce_effect(int(a, 1) as u16, int(a, 2) as u16));
                }
                Value::Null
            }
            "pc_identify_list" => {
                self.send_identify_list(id(0));
                Value::Null
            }
            "pc_arrow_list" => {
                // AC_MAKINGARROW: items that can be turned into arrows (0x01ad)
                let unit = id(0);
                let ids: Vec<u16> = arg(a, 1).as_array().map(|v| v.iter().map(|x| num(x) as u16).collect()).unwrap_or_default();
                if let Some(sid) = self.sid_of(unit) {
                    self.send(sid, packets::arrow_create_list(&ids));
                }
                Value::Null
            }
            "pc_unequip" => {
                // take off whatever is worn at these equip positions
                let (unit, mask) = (id(0), int(a, 1) as u32);
                let idx: Vec<u16> = self
                    .char_of_unit(unit)
                    .map(|c| c.inventory.iter().enumerate().filter(|(_, i)| i.equip & mask != 0).map(|(i, _)| i as u16 + packets::INV_OFFSET).collect())
                    .unwrap_or_default();
                for i in &idx {
                    self.unequip(unit, *i);
                }
                json!(!idx.is_empty())
            }
            "pc_destroy_equip" => {
                // whatever is worn at these equip positions is lost for good
                let (unit, mask) = (id(0), int(a, 1) as u32);
                let mut lost = Vec::new();
                while let Some(i) = self.char_of_unit(unit).and_then(|c| c.inventory.iter().position(|i| i.equip & mask != 0)) {
                    lost.push(self.char_of_unit(unit).map(|c| c.inventory[i].nameid).unwrap_or(0));
                    self.unequip(unit, i as u16 + packets::INV_OFFSET);
                    if self.char_of_unit(unit).is_some_and(|c| c.inventory.slot(i).is_some_and(|it| it.equip & mask != 0)) {
                        break; // it would not come off
                    }
                    self.remove_item(unit, i, 1);
                }
                json!(lost)
            }
            "pc_ammo_use" => {
                // one arrow off the quiver; false = none equipped
                let unit = id(0);
                let slot = self.char_of_unit(unit).and_then(|c| c.inventory.iter().position(|i| i.equip & 0x8000 != 0));
                match slot {
                    Some(i) => {
                        let left = self.char_of_unit(unit).map(|c| c.inventory[i].amount).unwrap_or(0);
                        self.remove_item(unit, i, 1);
                        if left <= 1 {
                            self.refresh_status(unit);
                        }
                        json!(true)
                    }
                    None => {
                        if let Some(sid) = self.sid_of(unit) {
                            self.send(sid, packets::arrow_fail(0));
                        }
                        json!(false)
                    }
                }
            }
            "pc_set_option" => {
                // cart, falcon, peco bits (the hide/sight bits come from status changes)
                self.set_option(id(0), int(a, 1) as u32);
                Value::Null
            }
            // ---- monsters
            "mob_target" => {
                // Athena mob_target: the monster turns on `src`
                self.mob_target(id(0), id(1), int(a, 2) as i32);
                Value::Null
            }
            "mob_event" => {
                // a hit landed: counter skills (mobskill_event)
                self.mobskill_event(id(0), &text(a, 1));
                Value::Null
            }
            "mob_warp" => {
                let mob = id(0);
                if let Some(u) = self.world.units.get(&mob) {
                    let map = u.map.clone();
                    if let Some((x, y)) = self.grid(&map).and_then(|g| crate::mapsrv::random_cell(&g)) {
                        self.mob_teleport(mob, x, y);
                    }
                }
                Value::Null
            }
            "mob_spawn" => {
                // map, x, y, class, master -> id
                let master = id(4);
                let r = self.spawn_mob(&text(a, 0), int(a, 1) as u16, int(a, 2) as u16, int(a, 3) as u32, None);
                if let (Some(nid), true) = (r, master != 0) {
                    if let Some(m) = self.world.units.get_mut(&nid).and_then(|u| u.mob_mut()) {
                        m.master = Some(master);
                    }
                }
                json!(r)
            }
            "mob_kill" => {
                // dies without giving anything (NPC_SUICIDE, self-destruction)
                let mob = id(0);
                if let Some(m) = self.world.units.get_mut(&mob).and_then(|u| u.mob_mut()) {
                    m.damage_log.clear();
                    m.hp = 0;
                    self.mob_dies(mob);
                }
                Value::Null
            }
            "mob_db" => self.db.mob(int(a, 0) as u32).and_then(|d| serde_json::to_value(d).ok()).unwrap_or(Value::Null),
            "item_db" => self
                .db
                .item(int(a, 0) as u32)
                .map(|d| json!({"id": d.id, "name": d.name, "type": d.kind, "weight": d.weight, "buy": d.buy, "sell": d.sell_price(), "slots": d.slots, "loc": d.loc, "wlv": d.weapon_level}))
                .unwrap_or(Value::Null),
            "drop_item" => {
                // map, x, y, item, amount
                self.drop_floor_item(&text(a, 0), int(a, 1) as u16, int(a, 2) as u16, InvItem::new(int(a, 3) as u32, int(a, 4).max(1) as u32), true);
                Value::Null
            }
            _ => return None,
        }))
    }

    /// Put a monster on another cell of its map (teleport: vanish, appear).
    pub(crate) fn mob_teleport(&mut self, mob: u32, x: u16, y: u16) {
        let Some(mut u) = self.world.remove(mob) else { return };
        let pkt = packets::vanish(mob, 3);
        let watchers: Vec<(u32, u32)> = self.world.units.values().filter_map(|o| o.player().filter(|p| p.visible.contains(&mob)).map(|p| (o.id, p.sid))).collect();
        for (oid, sid) in watchers {
            self.send(sid, pkt.clone());
            if let Some(p) = self.world.units.get_mut(&oid).and_then(|o| o.player_mut()) {
                p.visible.remove(&mob);
            }
        }
        u.x = x;
        u.y = y;
        u.walk = None;
        if let Some(m) = u.mob_mut() {
            m.target = None;
            m.looting = false;
        }
        self.world.add(u);
        self.update_watchers(mob, true);
    }
}
