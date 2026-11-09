//! NPC runtime: what Athena-era NPC scripts need beyond the dialog window.
//!
//! * variables: temporary per character (`tmp.x`, Athena `@x`), server-wide
//!   (`g.x`, Athena `$x`, saved in saves/world.json), per account (`acct.x`);
//!   permanent per character (`v.x`) live in host.rs (`var_get`/`var_set`).
//! * events: an NPC's `on_click(label)` entered at an "On..." label, queued by
//!   `do_event("Npc::Label")`, `OnInit` after every (re)load, NPC timers
//!   (`OnTimer<ms>`, athena-817 inittimer/stoptimer), player timers
//!   (`add_timer`), and `OnMyMobDead`-style monster events.
//! * enabling/disabling NPCs by id, script-spawned monsters, announcements,
//!   area/map warps, minimap marks, equipment and refining, options
//!   (cart/falcon/peco), stat reset.
//!
//! The Lua side is in prelude.lua; the generated NPCs call it through the
//! Athena-named wrappers listed in tools/import/athena_npc.py.

use std::collections::{HashMap, HashSet, VecDeque};

use serde_json::{Value, json};

use crate::game::Game;
use crate::packets;
use crate::script::{Ctx, CtxKind, NO_DEF, Outcome};
use crate::world::{NpcKind, NpcUnit, UnitKind};

/// Equipment slot numbers used by scripts (athena-817 script.c `equip[]`):
/// 1 head top, 2 armor, 3 left hand, 4 right hand, 5 garment, 6 shoes,
/// 7 accessory 1, 8 accessory 2, 9 head mid, 10 head low.
pub const EQUIP_SLOTS: [u32; 10] = [0x0100, 0x0010, 0x0020, 0x0002, 0x0004, 0x0040, 0x0008, 0x0080, 0x0200, 0x0001];

pub const OPT_CART: u32 = 0x0008;
pub const OPT_FALCON: u32 = 0x0010;
pub const OPT_RIDING: u32 = 0x0020;
pub const CART_BITS: u32 = 0x0008 | 0x0080 | 0x0100 | 0x0200 | 0x0400;
const MC_PUSHCART: u16 = 39;
const HT_FALCON: u16 = 127;
const KN_RIDING: u16 = 63;

#[derive(Debug, Clone)]
pub struct NpcTimer {
    pub started: u64,
    /// OnTimer labels already fired (in ascending order of their time).
    pub fired: usize,
}

#[derive(Debug, Clone)]
pub struct PlayerTimer {
    pub at: u64,
    pub unit: u32,
    pub event: String,
}

#[derive(Default)]
pub struct NpcRuntime {
    /// NPC ids switched off with disable_npc().
    pub disabled: HashSet<String>,
    pub timers: HashMap<String, NpcTimer>,
    pub player_timers: Vec<PlayerTimer>,
    /// (npc id, label, player unit or 0)
    pub queue: VecDeque<(String, String, u32)>,
    /// Temporary character variables, by player unit (gone at logout).
    pub temp: HashMap<u32, HashMap<String, Value>>,
    /// Unimplemented builtins already reported once.
    pub reported: HashSet<String>,
    pub globals_dirty: bool,
}

fn arg(a: &[Value], i: usize) -> &Value {
    a.get(i).unwrap_or(&Value::Null)
}
fn int(a: &[Value], i: usize) -> i64 {
    match arg(a, i) {
        Value::Number(n) => n.as_i64().unwrap_or_else(|| n.as_f64().unwrap_or(0.0) as i64),
        Value::String(s) => s.trim().parse().unwrap_or(0),
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
fn map_name(a: &[Value], i: usize) -> String {
    let m = text(a, i).to_ascii_lowercase();
    m.strip_suffix(".gat").map(String::from).unwrap_or(m)
}

impl Game {
    // ------------------------------------------------------------ events

    /// Queue "Npc::Label" to run (with `player` attached, 0 = none). A bare
    /// "Npc" is an event NPC (sprite -1), run from the top; for a monster's
    /// death event without one, Athena tries "Npc::OnMyMobDead" (npc.c npc_event).
    pub(crate) fn queue_event(&mut self, event: &str, player: u32) -> bool {
        let (npc, label) = event.split_once("::").unwrap_or((event, ""));
        self.npcrt.queue.push_back((npc.to_string(), label.to_string(), player));
        true
    }

    fn def_by_id(&self, id: &str) -> Option<usize> {
        let engine = self.scripts.as_ref()?;
        // Athena keeps the last NPC registered under a name
        engine.defs.npcs.iter().rposition(|n| n.id == id)
    }

    /// Unit of an NPC definition on a loaded map, if placed.
    pub(crate) fn unit_of_def(&self, def: usize) -> Option<u32> {
        self.world.units.values().find(|u| matches!(&u.kind, UnitKind::Npc(NpcUnit { kind: NpcKind::Script { def: d }, .. }) if *d == def)).map(|u| u.id)
    }

    /// Run queued events; ones for a player busy in a dialog wait their turn
    /// (Athena npc_event_dequeue).
    pub(crate) fn run_events(&mut self) {
        let mut waiting = VecDeque::new();
        let mut budget = 256;
        while let Some((npc, label, player)) = self.npcrt.queue.pop_front() {
            budget -= 1;
            if budget == 0 {
                self.notice("event queue: too many events in one tick (loop?)");
                break;
            }
            if player != 0 && !self.world.units.contains_key(&player) {
                continue;
            }
            if player != 0 && self.dialogs.contains_key(&player) {
                waiting.push_back((npc, label, player));
                continue;
            }
            self.run_event(&npc, &label, player);
        }
        waiting.append(&mut self.npcrt.queue);
        self.npcrt.queue = waiting;
    }

    fn run_event(&mut self, npc: &str, label: &str, player: u32) {
        let Some(engine) = self.scripts.clone() else { return };
        let Some(def) = self.def_by_id(npc) else {
            self.notice(format!("event {npc}::{label}: no such NPC"));
            return;
        };
        let d = &engine.defs.npcs[def];
        let label = if label.is_empty() {
            match (d.sprite, d.events.iter().any(|e| e == "OnMyMobDead")) {
                (None, _) => "",
                (Some(_), true) => "OnMyMobDead",
                _ => {
                    self.notice(format!("event {npc}: not an event NPC"));
                    return;
                }
            }
        } else {
            label
        };
        if !label.is_empty() && !d.events.iter().any(|e| e == label) {
            self.notice(format!("event {npc}::{label}: the NPC has no such label"));
            return;
        }
        let Some(f) = d.on_click.clone() else { return };
        let at = (!label.is_empty()).then_some(label);
        let unit = self.unit_of_def(def).unwrap_or(0);
        if player != 0 {
            self.start_dialog(player, unit, def, &f, CtxKind::Event, at);
            return;
        }
        let ctx = Ctx { player: 0, npc: unit, kind: CtxKind::Event, def: def as u32 };
        match engine.start(self, ctx, &f, at) {
            Ok((_, Outcome::Done(_))) => {}
            Ok((_, Outcome::Suspended(_))) => self.notice(format!("event {npc}::{label}: waits for a player but has none")),
            Err(e) => self.notice(format!("event {npc}::{label}: {e:#}")),
        }
    }

    /// After a (re)load: every NPC's OnInit, in load order.
    pub(crate) fn queue_init_events(&mut self) {
        let Some(engine) = self.scripts.clone() else { return };
        for n in &engine.defs.npcs {
            if n.events.iter().any(|e| e == "OnInit") {
                self.npcrt.queue.push_back((n.id.clone(), "OnInit".into(), 0));
            }
        }
    }

    pub(crate) fn tick_npc_timers(&mut self, now: u64) {
        let Some(engine) = self.scripts.clone() else { return };
        let ids: Vec<String> = self.npcrt.timers.keys().cloned().collect();
        for id in ids {
            let Some(def) = self.def_by_id(&id) else {
                self.npcrt.timers.remove(&id);
                continue;
            };
            let mut labels: Vec<(u64, &String)> = engine.defs.npcs[def]
                .events
                .iter()
                .filter_map(|e| e.strip_prefix("OnTimer").and_then(|n| n.parse::<u64>().ok()).map(|n| (n, e)))
                .collect();
            labels.sort();
            let Some(t) = self.npcrt.timers.get(&id).cloned() else { continue };
            let mut fired = t.fired;
            while fired < labels.len() && t.started + labels[fired].0 <= now {
                self.npcrt.queue.push_back((id.clone(), labels[fired].1.clone(), 0));
                fired += 1;
            }
            if fired >= labels.len() {
                self.npcrt.timers.remove(&id);
            } else if let Some(t) = self.npcrt.timers.get_mut(&id) {
                t.fired = fired;
            }
        }
        let (due, rest): (Vec<_>, Vec<_>) = self.npcrt.player_timers.drain(..).partition(|t| t.at <= now);
        self.npcrt.player_timers = rest;
        for t in due {
            self.queue_event(&t.event, t.unit);
        }
    }

    /// enable_npc / disable_npc (by id; Athena npcname_db).
    pub(crate) fn set_npc_enabled(&mut self, id: &str, on: bool) {
        if on {
            if !self.npcrt.disabled.remove(id) {
                return;
            }
            let Some(def) = self.def_by_id(id) else { return };
            let map = self.scripts.as_ref().map(|e| e.defs.npcs[def].map.clone()).unwrap_or_default();
            if self.world.maps.contains_key(&map) && self.unit_of_def(def).is_none() {
                self.place_npc_def(def);
            }
        } else {
            self.npcrt.disabled.insert(id.to_string());
            let Some(engine) = self.scripts.clone() else { return };
            let units: Vec<u32> = self
                .world
                .units
                .values()
                .filter(|u| matches!(&u.kind, UnitKind::Npc(NpcUnit { kind: NpcKind::Script { def }, .. }) if engine.defs.npcs[*def].id == id))
                .map(|u| u.id)
                .collect();
            for u in units {
                self.despawn(u, 0);
            }
        }
    }

    pub(crate) fn player_left(&mut self, unit: u32) {
        self.npcrt.temp.remove(&unit);
        self.npcrt.player_timers.retain(|t| t.unit != unit);
        self.npcrt.queue.retain(|(_, _, p)| *p != unit);
    }

    // ------------------------------------------------------------ monsters

    /// Athena mob_once_spawn (mod0631): map "this" = the player's map; x/y <= 0 =
    /// the player's cell (random cell without a player); name "--ja--"/"--en--" =
    /// db name.
    #[allow(clippy::too_many_arguments)]
    fn script_monster(&mut self, player: u32, map: &str, x0: i64, y0: i64, x1: i64, y1: i64, name: &str, mob: &Value, count: i64, event: &str) -> Result<Value, String> {
        let here = self.world.units.get(&player).map(|u| (u.map.clone(), u.x as i64, u.y as i64));
        let map = if map == "this" { here.as_ref().map(|h| h.0.clone()).unwrap_or_default() } else { map.to_string() };
        let map = map.as_str();
        let (x0, y0) = match &here {
            Some((m, px, py)) if m == map && x1 <= x0 && y1 <= y0 => (if x0 <= 0 { *px } else { x0 }, if y0 <= 0 { *py } else { y0 }),
            _ => (x0, y0),
        };
        if !self.ensure_map(map) {
            return Err(format!("unknown map {map}"));
        }
        let mob_id = match mob {
            Value::String(s) => match s.parse::<u32>() {
                Ok(n) => n,
                Err(_) => self.db.mob_by_name(s).map(|m| m.id).ok_or(format!("unknown monster {s}"))?,
            },
            v => v.as_u64().unwrap_or(0) as u32,
        };
        let grid = self.grid(map).ok_or("no grid")?;
        let mut ids = Vec::new();
        for _ in 0..count.max(1) {
            let mut cell = None;
            for _ in 0..50 {
                let (x, y) = if x1 > x0 || y1 > y0 {
                    (crate::combat::rand_range(x0 as i32, x1 as i32) as i64, crate::combat::rand_range(y0 as i32, y1 as i32) as i64)
                } else {
                    (x0, y0)
                };
                if x > 0 && y > 0 && grid.walkable(x as i32, y as i32) {
                    cell = Some((x as u16, y as u16));
                    break;
                }
                if x0 > 0 && x1 <= x0 && y1 <= y0 {
                    break; // a fixed cell that isn't walkable: Athena spawns there anyway
                }
            }
            let (x, y) = match cell {
                Some(c) => c,
                None if x0 > 0 && y0 > 0 => (x0 as u16, y0 as u16),
                None => crate::mapsrv::random_cell(&grid).ok_or("no walkable cell")?,
            };
            if let Some(id) = self.spawn_mob(map, x, y, mob_id, None) {
                if let Some(u) = self.world.units.get_mut(&id) {
                    if !name.is_empty() && name != "--ja--" && name != "--en--" {
                        u.name = name.to_string();
                    }
                    if let Some(m) = u.mob_mut() {
                        m.event = Some(event.to_string());
                    }
                }
                ids.push(id);
            }
        }
        Ok(json!(ids))
    }

    fn script_mobs(&self, map: &str, event: &str) -> Vec<u32> {
        let Some(mi) = self.world.maps.get(map) else { return vec![] };
        mi.units
            .iter()
            .filter(|id| {
                self.world.units.get(id).and_then(|u| u.mob()).is_some_and(|m| match &m.event {
                    Some(e) => event == "All" || e == event,
                    None => false,
                })
            })
            .copied()
            .collect()
    }

    // ------------------------------------------------------------ equipment

    /// Inventory slot worn in script equip slot `num` (1..=10).
    fn equip_slot(&self, unit: u32, num: i64) -> Option<usize> {
        let mask = *EQUIP_SLOTS.get((num - 1) as usize)?;
        self.char_of_unit(unit)?.inventory.iter().position(|it| it.equip & mask != 0)
    }

    fn equip_info(&self, unit: u32, num: i64) -> Value {
        let Some(slot) = self.equip_slot(unit, num) else { return Value::Null };
        let it = &self.char_of_unit(unit).unwrap().inventory[slot];
        let d = self.db.item(it.nameid);
        let wlv = d.map(|d| d.weapon_level).unwrap_or(0);
        let refinable = d.is_some_and(|d| matches!(d.kind, 4 | 5)) && (1..=6).contains(&num);
        json!({
            "id": it.nameid,
            "name": d.map(|d| d.display.clone()).unwrap_or_default(),
            "refine": it.refine,
            "identified": it.identify,
            "weapon_level": wlv,
            "refinable": refinable,
            "kind": d.map(|d| d.kind).unwrap_or(0),
        })
    }

    /// successrefitem / failedrefitem (athena-817 script.c).
    fn refine_result(&mut self, unit: u32, num: i64, success: bool) {
        let Some(slot) = self.equip_slot(unit, num) else { return };
        let Some(sid) = self.sid_of(unit) else { return };
        let idx = slot as u16 + packets::INV_OFFSET;
        if success {
            let refine = {
                let c = self.char_of_unit_mut(unit).unwrap();
                c.inventory[slot].refine = (c.inventory[slot].refine + 1).min(10);
                c.inventory[slot].refine
            };
            self.send(sid, packets::refine_result(0, idx, refine as u16));
            self.send_inventory(unit);
            self.send_full_status(unit);
            self.area_send(unit, packets::misc_effect(unit, 3), true);
        } else {
            let refine = self.char_of_unit(unit).map(|c| c.inventory[slot].refine).unwrap_or(0);
            self.unequip(unit, idx);
            self.send(sid, packets::refine_result(1, idx, refine as u16));
            self.remove_item(unit, slot, 1);
            self.area_send(unit, packets::misc_effect(unit, 2), true);
        }
    }

    // ------------------------------------------------------------ options

    pub(crate) fn set_option(&mut self, unit: u32, option: u32) {
        let Some(c) = self.char_of_unit_mut(unit) else { return };
        let had_cart = c.option & CART_BITS != 0;
        c.option = option;
        let (opt, karma) = (option as u16, c.karma as u8);
        self.area_send(unit, packets::state_change(unit, 0, 0, opt, karma), true);
        if option & CART_BITS != 0 && !had_cart {
            self.send_cart(unit);
        }
    }

    /// Athena pc_resetstate: every stat back to 1, points refunded.
    fn reset_stats(&mut self, unit: u32) {
        let Some(c) = self.char_of_unit_mut(unit) else { return };
        let sumsp = |a: i64| a * ((a - 2) / 10 + 2) - 5 * ((a - 2) / 10) * ((a - 2) / 10) - 6 * ((a - 2) / 10) - 2;
        let add: i64 = c.stats.iter().map(|&s| sumsp(s as i64)).sum();
        c.status_point = (c.status_point as i64 + add).clamp(0, u16::MAX as i64) as u16;
        for s in c.stats.iter_mut() {
            *s = 1;
        }
        self.send_full_status(unit);
    }

    // ------------------------------------------------------------ host ops

    /// Script ops of the NPC runtime; None = not ours.
    pub(crate) fn npc_call(&mut self, ctx: &Ctx, op: &str, a: &[Value]) -> Option<Result<Value, String>> {
        let player = || if ctx.player != 0 && self.world.units.contains_key(&ctx.player) { Ok(ctx.player) } else { Err("no player in this context".to_string()) };
        let r: Result<Value, String> = match op {
            // ---- variables
            "tvar_get" => player().map(|u| self.npcrt.temp.get(&u).and_then(|m| m.get(&text(a, 0))).cloned().unwrap_or(Value::Null)),
            "tvar_set" => player().map(|u| {
                let m = self.npcrt.temp.entry(u).or_default();
                match arg(a, 1) {
                    Value::Null => m.remove(&text(a, 0)),
                    v => m.insert(text(a, 0), v.clone()),
                };
                Value::Null
            }),
            "gvar_get" => Ok(self.store.globals.get(&text(a, 0)).cloned().unwrap_or(Value::Null)),
            "gvar_set" => {
                match arg(a, 1) {
                    Value::Null => self.store.globals.remove(&text(a, 0)),
                    v => self.store.globals.insert(text(a, 0), v.clone()),
                };
                self.npcrt.globals_dirty = true;
                Ok(Value::Null)
            }
            "avar_get" | "avar_set" => player().and_then(|u| {
                let aid = self.world.units[&u].player().map(|p| p.account_id).unwrap_or(0);
                let acc = self.store.accounts.get_mut(&aid).ok_or("no account")?;
                if op == "avar_get" {
                    return Ok(acc.vars.get(&text(a, 0)).cloned().unwrap_or(Value::Null));
                }
                match arg(a, 1) {
                    Value::Null => acc.vars.remove(&text(a, 0)),
                    v => acc.vars.insert(text(a, 0), v.clone()),
                };
                let _ = self.store.save_account(aid);
                Ok(Value::Null)
            }),
            // ---- events and timers
            "do_event" => {
                self.queue_event(&text(a, 0), ctx.player);
                Ok(Value::Null)
            }
            "add_timer" => player().map(|u| {
                let at = self.now() + int(a, 0).max(0) as u64;
                self.npcrt.player_timers.push(crate::npcrt::PlayerTimer { at, unit: u, event: text(a, 1) });
                Value::Null
            }),
            "del_timer" => player().map(|u| {
                let ev = text(a, 0);
                self.npcrt.player_timers.retain(|t| !(t.unit == u && t.event == ev));
                Value::Null
            }),
            "add_timer_count" => player().map(|u| {
                let ev = text(a, 0);
                let ms = int(a, 1);
                for t in self.npcrt.player_timers.iter_mut().filter(|t| t.unit == u && t.event == ev) {
                    t.at = (t.at as i64 + ms).max(0) as u64;
                }
                Value::Null
            }),
            "npc_timer" => {
                // npc_timer(on, [npc id]): athena-817 inittimer (on) / stoptimer (off)
                let id = match arg(a, 1) {
                    Value::String(s) => Some(s.clone()),
                    _ if ctx.def != NO_DEF => self.scripts.as_ref().map(|e| e.defs.npcs[ctx.def as usize].id.clone()),
                    _ => None,
                };
                match id {
                    Some(id) if int(a, 0) != 0 => {
                        let now = self.now();
                        self.npcrt.timers.insert(id, crate::npcrt::NpcTimer { started: now, fired: 0 });
                        Ok(Value::Null)
                    }
                    Some(id) => {
                        self.npcrt.timers.remove(&id);
                        Ok(Value::Null)
                    }
                    None => Err("npc_timer: no NPC in this context".into()),
                }
            }
            "enable_npc" | "disable_npc" => {
                self.set_npc_enabled(&text(a, 0), op == "enable_npc");
                Ok(Value::Null)
            }
            // ---- monsters
            "monster" => {
                let event = text(a, 6);
                self.script_monster(ctx.player, &map_name(a, 0), int(a, 1), int(a, 2), 0, 0, &text(a, 3), arg(a, 4), int(a, 5), &event)
            }
            "area_monster" => {
                let event = text(a, 8);
                self.script_monster(ctx.player, &map_name(a, 0), int(a, 1), int(a, 2), int(a, 3), int(a, 4), &text(a, 5), arg(a, 6), int(a, 7), &event)
            }
            "kill_monster" => {
                for id in self.script_mobs(&map_name(a, 0), &text(a, 1)) {
                    self.despawn(id, 0);
                }
                Ok(Value::Null)
            }
            "mob_count" => Ok(json!(self.script_mobs(&map_name(a, 0), &text(a, 1)).len())),
            // ---- messages, users, warps
            "announce" => {
                // flag & 0x0f: 0 everyone, 1 map, 2 area, 3 self; 0x10 blue (Athena clif_GMmessage)
                let flag = int(a, 1);
                let msg = if flag & 0x10 != 0 { format!("blue{}", text(a, 0)) } else { text(a, 0) };
                let pkt = packets::broadcast(&msg);
                let targets: Vec<u32> = match (flag & 0x0f, ctx.player) {
                    (3, p) if p != 0 => vec![p],
                    (1 | 2, p) if p != 0 => {
                        let m = self.world.units.get(&p).map(|u| u.map.clone()).unwrap_or_default();
                        self.world.players_on(&m)
                    }
                    _ => self.world.all_players(),
                };
                for t in targets {
                    if let Some(sid) = self.sid_of(t) {
                        self.send(sid, pkt.clone());
                    }
                }
                Ok(Value::Null)
            }
            "map_announce" | "area_announce" => {
                let (map, msg, flag) =
                    if op == "map_announce" { (map_name(a, 0), text(a, 1), int(a, 2)) } else { (map_name(a, 0), text(a, 5), int(a, 6)) };
                let area = (op == "area_announce").then(|| (int(a, 1), int(a, 2), int(a, 3), int(a, 4)));
                let msg = if flag & 0x10 != 0 { format!("blue{msg}") } else { msg };
                let pkt = packets::broadcast(&msg);
                for p in self.world.players_on(&map) {
                    let inside = match (area, self.world.units.get(&p)) {
                        (Some((x0, y0, x1, y1)), Some(u)) => (x0..=x1).contains(&(u.x as i64)) && (y0..=y1).contains(&(u.y as i64)),
                        _ => true,
                    };
                    if let (true, Some(sid)) = (inside, self.sid_of(p)) {
                        self.send(sid, pkt.clone());
                    }
                }
                Ok(Value::Null)
            }
            "get_users" => {
                // flag & 7: 0 = players on this map, 1 = everyone online
                let all = self.world.all_players();
                if int(a, 0) & 7 == 1 {
                    Ok(json!(all.len()))
                } else {
                    let m = self.world.units.get(&ctx.player).or_else(|| self.world.units.get(&ctx.npc)).map(|u| u.map.clone()).unwrap_or_default();
                    Ok(json!(self.world.players_on(&m).len()))
                }
            }
            "map_users" => Ok(json!(self.world.players_on(&map_name(a, 0)).len())),
            "area_users" => {
                let (map, x0, y0, x1, y1) = (map_name(a, 0), int(a, 1), int(a, 2), int(a, 3), int(a, 4));
                let n = self
                    .world
                    .players_on(&map)
                    .into_iter()
                    .filter(|p| self.world.units.get(p).is_some_and(|u| (x0..=x1).contains(&(u.x as i64)) && (y0..=y1).contains(&(u.y as i64))))
                    .count();
                Ok(json!(n))
            }
            "area_warp" => {
                let (map, x0, y0, x1, y1) = (map_name(a, 0), int(a, 1), int(a, 2), int(a, 3), int(a, 4));
                let (to, tx, ty) = (map_name(a, 5), int(a, 6), int(a, 7));
                let who: Vec<u32> = self
                    .world
                    .players_on(&map)
                    .into_iter()
                    .filter(|p| self.world.units.get(p).is_some_and(|u| (x0..=x1).contains(&(u.x as i64)) && (y0..=y1).contains(&(u.y as i64))))
                    .collect();
                for p in who {
                    self.script_warp(p, &to, tx, ty);
                }
                Ok(Value::Null)
            }
            "map_warp" => {
                let (from, to, x, y) = (map_name(a, 0), map_name(a, 1), int(a, 2), int(a, 3));
                for p in self.world.players_on(&from) {
                    self.script_warp(p, &to, x, y);
                }
                Ok(Value::Null)
            }
            "warp" => player().and_then(|u| {
                let map = text(a, 0);
                // Fly Wing & co: not on noteleport maps (Athena pc_randomwarp); false = refused
                let here = self.world.units.get(&u).map(|x| x.map.clone()).unwrap_or_default();
                if map.eq_ignore_ascii_case("Random") && self.db.map_flag(&here, "noteleport") {
                    if let Some(sid) = self.sid_of(u) {
                        self.send(sid, packets::map_info(0));
                    }
                    return Ok(json!(false));
                }
                if self.script_warp(u, &map, int(a, 1), int(a, 2)) { Ok(json!(true)) } else { Err(format!("cannot warp to {map}")) }
            }),
            "viewpoint" => player().map(|u| {
                if let Some(sid) = self.sid_of(u) {
                    let pkt = packets::viewpoint(ctx.npc, int(a, 0) as u32, int(a, 1) as u32, int(a, 2) as u32, int(a, 3) as u8, int(a, 4) as u32);
                    self.send(sid, pkt);
                }
                Value::Null
            }),
            "npc_emotion" => {
                let who = if ctx.npc != 0 { ctx.npc } else { ctx.player };
                if who != 0 {
                    self.area_send(who, packets::emotion(who, int(a, 0).clamp(0, 255) as u8), true);
                }
                Ok(Value::Null)
            }
            // ---- player
            "check_weight" => player().and_then(|u| {
                let id = self.script_item_id(arg(a, 0))?;
                let w = self.db.item(id).map(|d| d.weight).unwrap_or(0) as i64 * int(a, 1).max(0);
                let c = self.char_of_unit(u).ok_or("no character")?;
                let ok = self.weight_of(c) as i64 + w <= self.derive(c).max_weight as i64;
                Ok(json!(ok as i32))
            }),
            "equip_info" => player().map(|u| self.equip_info(u, int(a, 0))),
            "refine_result" => player().map(|u| {
                self.refine_result(u, int(a, 0), arg(a, 1).as_bool().unwrap_or(false));
                Value::Null
            }),
            "option_get" => player().map(|u| json!(self.char_of_unit(u).map(|c| c.option).unwrap_or(0))),
            "option_set" => player().map(|u| {
                self.set_option(u, int(a, 0).max(0) as u32);
                Value::Null
            }),
            "set_cart" | "set_falcon" | "set_riding" => player().map(|u| {
                // Athena pc_setcart/pc_setfalcon/pc_setriding: only with the skill
                let (skill, bit) = match op {
                    "set_cart" => (MC_PUSHCART, OPT_CART),
                    "set_falcon" => (HT_FALCON, OPT_FALCON),
                    _ => (KN_RIDING, OPT_RIDING),
                };
                let ok = self.skill_level(u, skill) > 0;
                if ok {
                    let opt = self.char_of_unit(u).map(|c| c.option).unwrap_or(0);
                    let opt = if bit == OPT_CART { (opt & !CART_BITS) | bit } else { opt | bit };
                    self.set_option(u, opt);
                }
                json!(ok)
            }),
            "reset_stats" => player().map(|u| {
                self.reset_stats(u);
                Value::Null
            }),
            "reset_skills" => player().map(|u| {
                self.reset_skills(u);
                Value::Null
            }),
            "skill_level" => player().map(|u| json!(self.skill_level(u, int(a, 0) as u16))),
            "give_skill" => player().and_then(|u| self.give_skill(u, int(a, 0) as u16, int(a, 1).clamp(0, 10) as u8).map(|_| Value::Null)),
            "item_box" => player().map(|u| {
                // Athena itemdb_searchrandomid: up to 2000 random picks, each kept with chance per/1e6
                let flag = int(a, 0) as u8;
                let pick = self.db.boxes.iter().find(|b| b.flag == flag).map(|b| {
                    let mut got = b.default;
                    for _ in 0..2000 {
                        if b.items.is_empty() {
                            break;
                        }
                        let (id, per) = b.items[rand::random::<usize>() % b.items.len()];
                        if rand::random::<u32>() % 1_000_000 < per {
                            got = id;
                            break;
                        }
                    }
                    got
                });
                match pick {
                    Some(id) if id != 0 && self.db.item(id).is_some() => {
                        self.add_item(u, crate::model::InvItem::new(id, 1));
                        json!(id)
                    }
                    _ => json!(0),
                }
            }),
            "dead_branch" => player().and_then(|u| {
                let here = self.world.units.get(&u).map(|x| x.map.clone()).unwrap_or_default();
                if self.db.map_flag(&here, "nobranch") {
                    return Ok(json!(false));
                }
                // mod0631 mob_once_spawn class 0: random 1000..1999 on the branch list
                // whose level is not above the player's, else a Poring
                let lv = self.char_of_unit(u).map(|c| c.base_level).unwrap_or(1);
                let list: Vec<u32> = self.db.branch.mobs.iter().copied().filter(|m| self.db.mob(*m).is_some_and(|d| d.level <= lv && d.hp > 0)).collect();
                let mob = if list.is_empty() { 1002 } else { list[rand::random::<usize>() % list.len()] };
                self.script_monster(u, "this", 0, 0, 0, 0, "--ja--", &json!(mob), 1, "")
            }),
            "catch_pet" => player().map(|u| {
                self.catch_pet_start(u, int(a, 0).max(0) as u32);
                Value::Null
            }),
            "birth_pet" => player().map(|u| {
                self.pet_egg_list(u);
                Value::Null
            }),
            "unequip_all" => player().map(|u| {
                // Athena releaseall: take off everything worn
                let worn: Vec<u16> = self
                    .char_of_unit(u)
                    .map(|c| c.inventory.iter().enumerate().filter(|(_, it)| it.equip != 0).map(|(i, _)| i as u16 + packets::INV_OFFSET).collect())
                    .unwrap_or_default();
                for idx in worn {
                    self.unequip(u, idx);
                }
                Value::Null
            }),
            "open_storage" => player().map(|u| {
                self.open_storage(u);
                Value::Null
            }),
            "unimplemented" => {
                let name = text(a, 0);
                if self.npcrt.reported.insert(name.clone()) {
                    let at = self.scripts.as_ref().and_then(|e| e.defs.npcs.get(ctx.def as usize)).and_then(|d| d.src.clone()).unwrap_or_default();
                    self.notice(format!("script: {name} is not implemented yet (first use: {at})"));
                }
                Ok(Value::Null)
            }
            _ => return None,
        };
        Some(r)
    }

    /// Script warp: Athena special targets "SavePoint" and "Random".
    pub(crate) fn script_warp(&mut self, unit: u32, map: &str, x: i64, y: i64) -> bool {
        let m = map.strip_suffix(".gat").unwrap_or(map);
        if m.eq_ignore_ascii_case("SavePoint") {
            let Some(p) = self.char_of_unit(unit).map(|c| c.save_point.clone()) else { return false };
            return self.warp_player(unit, &p.map, p.x, p.y);
        }
        if m.eq_ignore_ascii_case("Random") {
            let Some(cur) = self.world.units.get(&unit).map(|u| u.map.clone()) else { return false };
            return self.warp_player(unit, &cur, 0, 0);
        }
        self.warp_player(unit, m, x.max(0) as u16, y.max(0) as u16)
    }

    fn script_item_id(&self, v: &Value) -> Result<u32, String> {
        match v {
            Value::Number(n) => n.as_u64().map(|x| x as u32).ok_or("bad item id".into()),
            Value::String(s) => match s.parse::<u32>() {
                Ok(n) => Ok(n),
                Err(_) => self.db.item_by_name(s).map(|d| d.id).ok_or(format!("unknown item {s}")),
            },
            _ => Err("item id expected".into()),
        }
    }

    pub(crate) fn save_globals(&mut self) {
        if self.npcrt.globals_dirty {
            self.npcrt.globals_dirty = false;
            if let Err(e) = self.store.save_globals() {
                tracing::warn!("save world.json: {e:#}");
            }
        }
    }
}
