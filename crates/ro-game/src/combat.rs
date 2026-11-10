//! Monsters, fighting, death, EXP, drops, regen.
//!
//! The *timing and packets* are protocol facts; the *numbers* (damage, hit,
//! EXP penalties, regen) are server-side and come from Lua `rules.*`
//! (scripts/rules/). An attack is one call of `rules.weapon_attack`, which
//! computes, sends the damage packet and takes the HP through the battle
//! API (battle.rs).
//!
//! Monster AI follows athena-dev-2.1.1-mod0631 `map/mob.c` (archived
//! 2003-12-18): `mob_ai_sub_hard` (targets, chase, attack, loot, random
//! walk), `mob_ai_sub_lazy` (maps nobody is on), `mobskill_use` (the skill
//! list of data/db/mob_skills.toml) and `mob_damage` (EXP share, drops, MVP
//! reward). Line numbers in comments are that file's. Tier athena2003.

use serde_json::{Value, json};

use crate::game::Game;
use crate::model::{InvItem, Point};
use crate::packets;
use crate::path;
use crate::world::{AttackIntent, DEFAULT_SPEED, MobState, MobUnit, Unit, UnitKind, Walk};

/// mob.c MIN_MOBTHINKTIME.
const MOB_THINK_MS: u64 = 100;
const DEFAULT_RESPAWN_MS: u32 = 5000;
/// map.h LOOTITEM_SIZE.
const LOOT_SIZE: usize = 20;

// Athena monster mode bits
const MD_CANMOVE: u32 = 0x01;
const MD_LOOTER: u32 = 0x02;
const MD_AGGRESSIVE: u32 = 0x04;
const MD_ASSIST: u32 = 0x08;
const MD_BOSS: u32 = 0x20;

// status changes the AI reads
const SC_INCREASEAGI: u16 = 12;
const SC_DECREASEAGI: u16 = 13;
const SC_QUAGMIRE: u16 = 8;
const SC_SILENCE: u16 = 70;

impl Game {
    /// Instantiate the monster spawns of a map (first time a player enters).
    pub(crate) fn populate_map(&mut self, map: &str) {
        if self.world.maps.get(map).is_none_or(|m| m.populated) {
            return;
        }
        self.world.maps.get_mut(map).unwrap().populated = true;
        let spawns: Vec<(usize, u32)> = self
            .scripts
            .as_ref()
            .map(|e| e.defs.spawns.iter().enumerate().filter(|(_, s)| s.map == map).map(|(i, s)| (i, s.count)).collect())
            .unwrap_or_default();
        for (i, count) in spawns {
            for _ in 0..count {
                self.spawn_from(i);
            }
        }
    }

    /// Spawn one monster of spawn group `i`.
    pub(crate) fn spawn_from(&mut self, i: usize) -> Option<u32> {
        let s = self.scripts.as_ref()?.defs.spawns.get(i)?.clone();
        let grid = self.grid(&s.map)?;
        let (mut x, mut y) = (s.x, s.y);
        let mut ok = false;
        for _ in 0..50 {
            if s.xs > 0 || s.ys > 0 {
                x = (s.x as i32 + rand_range(-(s.xs as i32), s.xs as i32)).max(0) as u16;
                y = (s.y as i32 + rand_range(-(s.ys as i32), s.ys as i32)).max(0) as u16;
            } else if s.x == 0 && s.y == 0 {
                (x, y) = crate::mapsrv::random_cell(&grid)?;
            }
            if grid.walkable(x as i32, y as i32) {
                ok = true;
                break;
            }
        }
        if !ok {
            (x, y) = crate::mapsrv::random_cell(&grid)?;
        }
        let mob = match &s.mob_name {
            Some(n) => self.db.mob_by_name(n).map(|m| m.id)?,
            None => s.mob,
        };
        self.spawn_mob(&s.map, x, y, mob, Some(i))
    }

    pub(crate) fn spawn_mob(&mut self, map: &str, x: u16, y: u16, mob_id: u32, spawn: Option<usize>) -> Option<u32> {
        let m = self.db.mob(mob_id)?.clone();
        let id = self.world.new_obj_id();
        let now = self.now();
        let skills = self.db.mob_skills.get(&mob_id).map(|v| v.len()).unwrap_or(0);
        self.world.add(Unit {
            id,
            map: map.into(),
            x,
            y,
            dir: rand::random::<u8>() % 8,
            head_dir: 0,
            speed: if m.speed > 0 { m.speed as u16 } else { DEFAULT_SPEED },
            class: mob_id as u16,
            name: m.display.clone(),
            state: 0,
            walk: None,
            kind: UnitKind::Mob(Box::new(MobUnit {
                mob_id,
                hp: m.hp.max(1),
                max_hp: m.hp.max(1),
                spawn,
                target: None,
                next_think: now + rand::random::<u64>() % 1000,
                next_attack_at: 0,
                damage_log: Default::default(),
                event: None,
                sc: Default::default(),
                def_ele: None,
                cast: None,
                canact: 0,
                master: None,
                attacked: None,
                looting: false,
                min_chase: 13,
                // mob.c:640 next_walktime = tick + rand()%50 + 5000
                next_walk: now + 5000 + rand::random::<u64>() % 50,
                move_fail: 0,
                skill_state: MobState::Idle,
                skill_used: vec![0; skills],
                loot: Vec::new(),
                master_dist: 0,
                delay_until: 0,
            })),
        });
        self.update_watchers(id, true);
        Some(id)
    }

    pub(crate) fn tick_respawns(&mut self, now: u64) {
        let (due, rest): (Vec<_>, Vec<_>) = self.respawns.drain(..).partition(|(t, _)| *t <= now);
        self.respawns = rest;
        for (_, i) in due {
            self.spawn_from(i);
        }
    }

    // ------------------------------------------------------------ player attacks

    pub(crate) fn start_attack(&mut self, unit: u32, intent: AttackIntent) {
        let Some(target) = self.world.units.get(&intent.target) else { return };
        if target.mob().is_none() || target.state == 1 {
            return;
        }
        let (tx, ty, tmap) = (target.x, target.y, target.map.clone());
        let Some(me) = self.world.units.get_mut(&unit) else { return };
        if me.map != tmap || me.state == 1 {
            return;
        }
        // pc_attack_timer: not while stunned, frozen, asleep, stone, hidden or cloaked
        let (opt1, _, option) = self.opts_of(unit);
        if opt1 > 0 || option & 6 != 0 {
            return;
        }
        let me = self.world.units.get_mut(&unit).unwrap();
        if me.state == 2 {
            me.state = 0;
        }
        let Some(p) = me.player_mut() else { return };
        p.attack = Some(intent);
        let range = self.char_of_unit(unit).map(|c| self.derive(c).range.max(1)).unwrap_or(1) as i32;
        let me = &self.world.units[&unit];
        // pc.c:2424: one cell of slack before walking closer
        if path::distance(me.x as i32, me.y as i32, tx as i32, ty as i32) > range + 1 && self.can_move(unit) {
            self.walk_toward(unit, tx, ty, range);
        }
    }

    /// Walk to a cell within `range` of (tx, ty) (the target's own cell is taken).
    fn walk_toward(&mut self, unit: u32, tx: u16, ty: u16, range: i32) {
        let Some(u) = self.world.units.get(&unit) else { return };
        let (x, y) = (u.x as i32, u.y as i32);
        let Some(grid) = self.grid(&u.map) else { return };
        let (dx, dy) = ((x - tx as i32).signum(), (y - ty as i32).signum());
        let r = range.max(1);
        // the cell on our side of the target, `range` cells out; else the target cell itself
        for k in (1..=r).rev() {
            let (cx, cy) = (tx as i32 + dx * k, ty as i32 + dy * k);
            if grid.walkable(cx, cy) && self.walk_to(unit, cx as u16, cy as u16) {
                return;
            }
        }
        self.walk_to(unit, tx, ty);
    }

    pub(crate) fn tick_combat(&mut self, now: u64) {
        // players
        let attackers: Vec<u32> = self
            .world
            .units
            .values()
            .filter(|u| u.player().is_some_and(|p| p.attack.is_some() && p.next_attack_at <= now))
            .map(|u| u.id)
            .collect();
        for id in attackers {
            self.player_attack_step(id, now);
        }
        // monsters: the real AI near players, the lazy one elsewhere
        let mobs: Vec<(u32, bool)> = self
            .world
            .units
            .values()
            .filter(|u| u.mob().is_some_and(|m| m.next_think <= now))
            .map(|u| (u.id, self.world.maps.get(&u.map).is_some_and(|m| m.players > 0)))
            .collect();
        for (id, near) in mobs {
            if near {
                self.mob_think(id, now);
            } else if let Some(m) = self.world.units.get_mut(&id).and_then(|u| u.mob_mut()) {
                // nobody on the map: nothing to do until someone comes
                m.next_think = now + MOB_THINK_MS * 10;
                m.target = None;
                m.attacked = None;
            }
        }
    }

    fn player_attack_step(&mut self, id: u32, now: u64) {
        let Some(intent) = self.world.units.get(&id).and_then(|u| u.player()).and_then(|p| p.attack) else { return };
        let target_ok = self.world.units.get(&intent.target).is_some_and(|t| t.mob().is_some() && t.state != 1 && t.map == self.world.units[&id].map);
        let (opt1, _, option) = self.opts_of(id);
        if !target_ok || opt1 > 0 || option & 6 != 0 || self.world.units[&id].state == 1 {
            self.world.units.get_mut(&id).unwrap().player_mut().unwrap().attack = None;
            return;
        }
        // pc.c:2419 pc_attack_timer: no attack during a skill's delay; the
        // client is told and the attack is dropped
        let p = self.world.units[&id].player().unwrap();
        if p.canact > now || p.cast.is_some() {
            let sid = p.sid;
            self.world.units.get_mut(&id).unwrap().player_mut().unwrap().attack = None;
            self.send(sid, packets::skill_fail(1, 0, 4));
            return;
        }
        let Some(c) = self.char_of_unit(id).cloned() else { return };
        let d = self.derive(&c);
        let (me, t) = (&self.world.units[&id], &self.world.units[&intent.target]);
        let (mx, my, tx, ty, walking) = (me.x, me.y, t.x, t.y, me.walk.is_some());
        let dist = path::distance(mx as i32, my as i32, tx as i32, ty as i32);
        if dist > d.range.max(1) as i32 + 1 {
            if !walking && self.can_move(id) {
                self.walk_toward(id, tx, ty, d.range.max(1) as i32);
            }
            return;
        }
        if walking {
            // in range: stop where we are
            self.stop_walking(id);
        }
        // face the target (pc.c:2429)
        let (dx, dy) = (tx as i32 - mx as i32, ty as i32 - my as i32);
        if (dx, dy) != (0, 0) {
            let dir = path::dir_of(dx, dy);
            let u = self.world.units.get_mut(&id).unwrap();
            u.dir = dir;
            u.head_dir = 0;
        }
        let r = self.rule_call("weapon_attack", json!({"src": id, "target": intent.target}));
        // pc.c:2458: the next attack comes after twice the "aspd" value
        let delay = r.as_ref().and_then(|v| v.get("adelay")).and_then(crate::status::lua_u64).unwrap_or(d.amotion as u64 * 2).max(100);
        if let Some(p) = self.world.units.get_mut(&id).and_then(|u| u.player_mut()) {
            p.next_attack_at = now + delay;
            if !intent.continuous || r == Some(Value::Bool(false)) {
                p.attack = None;
            }
        }
    }

    /// Athena mob_damage, the part before death. Returns true when it died.
    pub(crate) fn damage_mob(&mut self, mob: u32, by: u32, dmg: u32) -> bool {
        let by_player = self.world.units.get(&by).is_some_and(|u| u.is_player());
        let now = self.now();
        let flinch = self.world.units.get(&mob).and_then(|u| u.mob()).and_then(|m| self.db.mob(m.mob_id)).map(|d| d.damage_motion as u64).unwrap_or(0);
        let Some(u) = self.world.units.get_mut(&mob) else { return false };
        // mob.c:1549: a hit stops its walk and holds it for its damage motion
        let stopped = u.walk.take().is_some();
        let (x, y) = (u.x, u.y);
        let Some(m) = u.mob_mut() else { return false };
        if stopped {
            m.delay_until = now + flinch;
        }
        let dmg = dmg.min(m.hp);
        if by_player {
            *m.damage_log.entry(by).or_default() += dmg;
            if m.attacked.is_none() {
                m.attacked = Some(by);
            }
        }
        m.hp -= dmg;
        let dead = m.hp == 0;
        if stopped && !dead {
            self.area_send(mob, packets::stop_move(mob, x, y), false);
        }
        if dead {
            self.mob_dies(mob);
        }
        dead
    }

    pub(crate) fn mob_dies(&mut self, mob: u32) {
        let Some(u) = self.world.units.get(&mob).cloned() else { return };
        let Some(m) = u.mob() else { return };
        let mdb = self.db.mob(m.mob_id).cloned().unwrap_or_default();
        self.despawn(mob, 1);
        if let Some(ev) = m.event.clone().filter(|e| !e.is_empty()) {
            // Athena: the killer is attached (the last player to hit it)
            let killer = m.damage_log.keys().copied().find(|p| self.world.units.contains_key(p)).unwrap_or(0);
            self.queue_event(&ev, killer);
        }
        // EXP (mob.c:1604): each attacker on the map gets its share of the
        // damage, +10% of the total per attacker beyond the first, at most double
        let here: Vec<(u32, u32)> = m
            .damage_log
            .iter()
            .filter(|(p, _)| self.world.units.get(p).is_some_and(|pu| pu.is_player() && pu.map == u.map))
            .map(|(p, d)| (*p, *d))
            .collect();
        let count = m.damage_log.keys().filter(|p| self.world.units.contains_key(p)).count() as u64;
        let sum: u64 = match here.iter().map(|(_, d)| *d as u64).sum() {
            0 => mdb.hp.max(1) as u64,
            s => s,
        };
        // mob.c:2323: every monster is worth 1 EXP at least; the rate comes before the share
        let base_all = (mdb.base_exp as u64 * self.cfg.base_exp_rate as u64 / 100).max(1);
        let job_all = (mdb.job_exp as u64 * self.cfg.job_exp_rate as u64 / 100).max(1);
        for &(pid, dmg) in &here {
            let per = (dmg as u64 * 256 * (9 + count) / 10 / sum).clamp(1, 512);
            self.gain_exp(pid, base_all * per / 256, job_all * per / 256);
        }
        // drops (Athena: rate/10000, scaled by drop_rate)
        let drop_at = |g: &Game| {
            let (dx, dy) = (rand_range(-1, 1), rand_range(-1, 1));
            let (x, y) = ((u.x as i32 + dx).max(0) as u16, (u.y as i32 + dy).max(0) as u16);
            if g.grid(&u.map).is_some_and(|gr| gr.walkable(x as i32, y as i32)) { (x, y) } else { (u.x, u.y) }
        };
        for d in &mdb.drops {
            let rate = (d.rate as u64 * self.cfg.drop_rate as u64 / 100).min(10000) as u32;
            // mob.c:1669: dropped unless the rate is below the roll
            if rate > 0 && rand::random::<u32>() % 10000 <= rate && self.db.item(d.item).is_some() {
                let (x, y) = drop_at(self);
                let mut it = InvItem::new(d.item, 1);
                // equipment falls unidentified (mob.c mob_delay_item_drop: identify = !itemdb_isequip3)
                if self.db.item(d.item).is_some_and(|i| matches!(i.kind, 4 | 5)) {
                    it.identify = false;
                }
                self.drop_floor_item(&u.map, x, y, it, true);
            }
        }
        // what a looter had picked up
        for it in &m.loot {
            let (x, y) = drop_at(self);
            self.drop_floor_item(&u.map, x, y, it.clone(), true);
        }
        // MVP (mob.c:1703): the biggest damage dealer gets one reward item, or the MVP EXP
        if mdb.mvp_exp > 0 {
            if let Some(&(mvp, _)) = here.iter().max_by_key(|(_, d)| *d) {
                self.area_send(mvp, packets::mvp_effect(mvp), true);
                let mut got = false;
                for d in &mdb.mvp_drops {
                    if d.item == 0 || d.rate < rand::random::<u32>() % 10000 || self.db.item(d.item).is_none() {
                        continue;
                    }
                    if let Some(sid) = self.sid_of(mvp) {
                        self.send(sid, packets::mvp_item(d.item as u16));
                    }
                    let heavy = self.char_of_unit(mvp).is_some_and(|c| self.weight_of(c) * 2 > self.derive(c).max_weight);
                    if heavy || !self.add_item(mvp, InvItem::new(d.item, 1)) {
                        if let Some((map, x, y)) = self.world.units.get(&mvp).map(|p| (p.map.clone(), p.x, p.y)) {
                            self.drop_floor_item(&map, x, y, InvItem::new(d.item, 1), true);
                        }
                    }
                    got = true;
                    break;
                }
                if !got {
                    if let Some(sid) = self.sid_of(mvp) {
                        self.send(sid, packets::mvp_exp(mdb.mvp_exp));
                    }
                    self.gain_exp(mvp, mdb.mvp_exp as u64 * self.cfg.base_exp_rate as u64 / 100, 0);
                }
            }
        }
        // its slaves die with it (mob.c:1496 mob_deleteslave: a death like any
        // other, with drops, and EXP for whoever had hit them)
        let slaves: Vec<u32> = self.world.units.values().filter(|s| s.mob().is_some_and(|sm| sm.master == Some(mob))).map(|s| s.id).collect();
        for s in slaves {
            if let Some(sm) = self.world.units.get_mut(&s).and_then(|u| u.mob_mut()) {
                sm.hp = 0;
                sm.master = None;
            }
            self.mob_dies(s);
        }
        self.defer_rule("unit_gone", json!({"bl": mob}));
        if let Some(i) = m.spawn {
            let s = self.scripts.as_ref().and_then(|e| e.defs.spawns.get(i)).cloned();
            if let Some(s) = s {
                let base = if s.delay > 0 { s.delay } else { DEFAULT_RESPAWN_MS };
                let extra = if s.delay2 > 0 { rand::random::<u32>() % s.delay2 } else { 0 };
                let at = self.now() + (base + extra) as u64;
                self.respawns.push((at, i));
            }
        }
    }

    // ------------------------------------------------------------ monster AI

    /// Can this monster see `target` as prey? Hiding, cloaking and playing
    /// dead fool everything but bosses, insects and demons (mob.c:746).
    fn mob_sees(&self, mdb: &ro_data::db::Mob, target: u32) -> bool {
        let Some(t) = self.world.units.get(&target) else { return false };
        if t.state == 1 || !t.is_player() {
            return false;
        }
        if mdb.mvp_exp > 0 || mdb.mode & MD_BOSS != 0 {
            return true;
        }
        if self.has_sc(target, crate::battle::SC_TRICKDEAD) {
            return false;
        }
        let (_, _, option) = self.opts_of(target);
        option & 6 == 0 || mdb.race == 4 || mdb.race == 6
    }

    /// Athena mob_target: something drew the monster's attention.
    pub(crate) fn mob_target(&mut self, mob: u32, src: u32, dist: i32) {
        let Some(m) = self.world.units.get(&mob).and_then(|u| u.mob()) else { return };
        let Some(mdb) = self.db.mob(m.mob_id).cloned() else { return };
        if mdb.mode == 0 {
            return;
        }
        // already locked on and not minded to switch (aggressive ones do, 25%)
        if m.target.is_some() && !m.looting && (mdb.mode & MD_AGGRESSIVE == 0 || rand::random::<u32>() % 100 > 25) {
            return;
        }
        if self.mob_sees(&mdb, src) {
            if let Some(m) = self.world.units.get_mut(&mob).and_then(|u| u.mob_mut()) {
                m.target = Some(src);
                m.looting = false;
                m.min_chase = (dist + 13).min(26);
            }
        }
    }

    /// mob_unlocktarget.
    fn mob_unlock(&mut self, id: u32, now: u64) {
        if let Some(m) = self.world.units.get_mut(&id).and_then(|u| u.mob_mut()) {
            m.target = None;
            m.looting = false;
            m.skill_state = MobState::Idle;
            m.next_walk = now + 3000 + rand::random::<u64>() % 3000;
        }
    }

    /// mob.c:203 mob_get_speed.
    fn mob_speed(&self, id: u32, base: u32) -> u16 {
        let mut a = if base > 0 { base } else { DEFAULT_SPEED as u32 };
        if self.has_sc(id, SC_INCREASEAGI) {
            a -= a * 25 / 100;
        }
        if self.has_sc(id, SC_DECREASEAGI) {
            a = a * 125 / 100;
        }
        if self.has_sc(id, SC_QUAGMIRE) {
            a = a * 3 / 2;
        }
        a.clamp(20, 2000) as u16
    }

    /// mob_can_reach (mob.c:708): within `range` cells and a walkable path.
    fn mob_can_reach(&self, map: &str, x: i32, y: i32, tx: i32, ty: i32, range: i32) -> bool {
        if range > 0 && range < path::distance(x, y, tx, ty) {
            return false;
        }
        if (x, y) == (tx, ty) {
            return true;
        }
        self.grid(map).is_some_and(|g| path::find(&g, x, y, tx, ty).is_some())
    }

    /// mob_ai_sub_hard (mob.c:1050).
    fn mob_think(&mut self, id: u32, now: u64) {
        let Some(u) = self.world.units.get(&id) else { return };
        let Some(m) = u.mob() else { return };
        let Some(mdb) = self.db.mob(m.mob_id).cloned() else { return };
        let mode = mdb.mode; // 0 = plant-like: never moves or attacks
        let (map, x, y) = (u.map.clone(), u.x as i32, u.y as i32);
        let speed = self.mob_speed(id, mdb.speed);
        {
            let u = self.world.units.get_mut(&id).unwrap();
            u.speed = speed;
            let m = u.mob_mut().unwrap();
            m.next_think = now + MOB_THINK_MS;
            // casting
            if m.cast.is_some() {
                if now > m.next_walk + MOB_THINK_MS {
                    m.next_walk = now;
                }
                return;
            }
            if mode == 0 {
                m.target = None;
            }
        }
        // stone, frozen, stunned, asleep; or flinching from a hit (MS_DELAY)
        if self.opts_of(id).0 > 0 || self.world.units[&id].mob().unwrap().delay_until > now {
            return;
        }
        let m = self.world.units[&id].mob().unwrap();
        let (attacked, master, class) = (m.attacked, m.master, m.mob_id);
        let dist_to = |g: &Game, t: u32| g.world.units.get(&t).filter(|tu| tu.map == map).map(|tu| path::distance(x, y, tu.x as i32, tu.y as i32));

        // The hit monster's kind comes to help (mob.c:1086, 850): every one
        // of its class within 13 cells that can reach the attacker, itself
        // included, turns on him, unless it is busy fighting while this one is too.
        if let (Some(a), true) = (attacked, mode & MD_ASSIST != 0) {
            if let Some((ax, ay)) = self.world.units.get(&a).filter(|p| p.is_player() && p.map == map).map(|p| (p.x as i32, p.y as i32)) {
                let me_idle = m.target.is_none() || m.looting; // md->state.targettype == NONE_ATTACKABLE
                let friends: Vec<(u32, i32, i32)> = self
                    .world
                    .maps
                    .get(&map)
                    .map(|mi| {
                        mi.units
                            .iter()
                            .filter_map(|i| self.world.units.get(i))
                            .filter(|f| (f.x as i32 - x).abs() <= 13 && (f.y as i32 - y).abs() <= 13)
                            .filter(|f| f.mob().is_some_and(|fm| fm.mob_id == class && (fm.target.is_none() || me_idle)))
                            .map(|f| (f.id, f.x as i32, f.y as i32))
                            .collect()
                    })
                    .unwrap_or_default();
                for (f, fx, fy) in friends {
                    if !self.mob_can_reach(&map, fx, fy, ax, ay, 12) {
                        continue;
                    }
                    if let Some(fm) = self.world.units.get_mut(&f).and_then(|u| u.mob_mut()) {
                        fm.target = Some(a);
                        fm.looting = false;
                        fm.min_chase = 13;
                    }
                }
            }
        }

        // whoever hit it (mob.c:1099); aggressive ones switch a quarter of the time
        let m = self.world.units[&id].mob().unwrap();
        let mut retaliate = false;
        if let (Some(a), true) = (attacked, mode > 0) {
            if m.target.is_none() || m.looting || (mode & MD_AGGRESSIVE != 0 && rand::random::<u32>() % 100 < 25) {
                let d = dist_to(self, a).filter(|d| *d < 32);
                let m = self.world.units.get_mut(&id).unwrap().mob_mut().unwrap();
                m.attacked = None;
                if let Some(d) = d {
                    m.target = Some(a);
                    m.looting = false;
                    m.min_chase = (d + 13).min(26);
                    retaliate = true;
                }
            }
        }

        // a slave follows its master and takes its target (mob.c:885)
        let mut master_check = false;
        if let Some((mx, my, mtarget)) = master.and_then(|mid| self.world.units.get(&mid)).filter(|mu| mu.map == map).map(|mu| {
            (mu.x as i32, mu.y as i32, mu.mob().and_then(|mm| mm.target.filter(|_| !mm.looting)))
        }) {
            let old = self.world.units[&id].mob().unwrap().master_dist;
            let dist = path::distance(x, y, mx, my);
            self.world.units.get_mut(&id).unwrap().mob_mut().unwrap().master_dist = dist;
            if old < 6 && dist > 15 {
                // the master was near a moment ago: jump after it (mob_warp)
                self.mob_teleport(id, mx as u16, my as u16);
                if let Some(m) = self.world.units.get_mut(&id).and_then(|u| u.mob_mut()) {
                    m.attacked = None;
                    m.skill_state = MobState::Idle;
                    m.next_think = now + MOB_THINK_MS;
                }
                return;
            }
            let m = self.world.units[&id].mob().unwrap();
            let free = m.target.is_none() || m.looting;
            if free && self.can_move(id) && self.world.units[&id].walk.is_none() && dist < 15 {
                if dist > 4 {
                    // toward the master, stopping short by up to three cells
                    for i in 0..10 {
                        let (dx, dy) = if i <= 5 {
                            let step = |d: i32| d - d.signum() * (rand_range(0, d.abs().min(3) - 1).max(0) + 1).min(d.abs());
                            (step(mx - x), step(my - y))
                        } else {
                            (mx - x + rand_range(-3, 3), my - y + rand_range(-3, 3))
                        };
                        if (dx, dy) != (0, 0) && self.mob_walk(id, &map, x, y, x + dx, y + dy, false) {
                            break;
                        }
                    }
                }
                // (within 4 cells mod0631 picks a cell around the master, adds
                // the master's position to it twice, and so never walks: the
                // slave stands. mob.c:934)
                if let Some(m) = self.world.units.get_mut(&id).and_then(|u| u.mob_mut()) {
                    m.next_walk = now + 500;
                }
                master_check = true;
            }
            if let (Some(t), true) = (mtarget, free) {
                if self.mob_sees(&mdb, t) {
                    let d = dist_to(self, t).unwrap_or(0);
                    let m = self.world.units.get_mut(&id).unwrap().mob_mut().unwrap();
                    m.target = Some(t);
                    m.looting = false;
                    m.min_chase = 5 + d;
                    master_check = true;
                }
            }
        }

        // aggressive monsters look for players within 9 cells, any of them equally (mob.c:785)
        let m = self.world.units[&id].mob().unwrap();
        if (m.target.is_none() || m.looting) && mode & MD_AGGRESSIVE != 0 && !master_check {
            let mut n = 0u32;
            let mut pick = None;
            for pid in self.world.players_on(&map) {
                let p = &self.world.units[&pid];
                let (px, py) = (p.x as i32, p.y as i32);
                if p.state == 1 || path::distance(x, y, px, py) >= 9 || !self.mob_sees(&mdb, pid) {
                    continue;
                }
                if !self.mob_can_reach(&map, x, y, px, py, 12) {
                    continue;
                }
                n += 1;
                if rand::random::<u32>() % 1000 < 1000 / n {
                    pick = Some(pid);
                }
            }
            if let Some(pid) = pick {
                let m = self.world.units.get_mut(&id).unwrap().mob_mut().unwrap();
                m.target = Some(pid);
                m.looting = false;
                m.min_chase = 13;
            }
        }

        // looters look for items within 9 cells they can walk to (mob.c:822)
        let m = self.world.units[&id].mob().unwrap();
        if m.target.is_none() && mode & MD_LOOTER != 0 && !master_check {
            let items: Vec<(u32, i32, i32)> = self
                .world
                .maps
                .get(&map)
                .map(|mi| mi.units.iter().filter_map(|i| self.world.units.get(i)).filter(|i| i.is_item()).map(|i| (i.id, i.x as i32, i.y as i32)).collect())
                .unwrap_or_default();
            let mut n = 0u32;
            let mut pick = None;
            for (iid, ix, iy) in items {
                if path::distance(x, y, ix, iy) >= 9 || !self.mob_can_reach(&map, x, y, ix, iy, 12) {
                    continue;
                }
                n += 1;
                if rand::random::<u32>() % 1000 < 1000 / n {
                    pick = Some(iid);
                }
            }
            if let Some(iid) = pick {
                let m = self.world.units.get_mut(&id).unwrap().mob_mut().unwrap();
                m.target = Some(iid);
                m.looting = true;
                m.min_chase = 13;
            }
        }

        let m = self.world.units[&id].mob().unwrap();
        let (target, looting, min_chase, next_walk) = (m.target, m.looting, m.min_chase, m.next_walk);
        let walk_to = self.world.units[&id].walk.as_ref().map(|w| (w.to.0 as i32, w.to.1 as i32));
        if let Some(t) = target {
            let Some(tu) = self.world.units.get(&t).filter(|tu| tu.map == map) else {
                self.mob_unlock(id, now);
                return;
            };
            let (tx, ty) = (tu.x as i32, tu.y as i32);
            let dist = path::distance(x, y, tx, ty);
            if !looting {
                // mob.c:1152: hiding and playing dead fool all but bosses, insects and demons
                let fooled = mode & MD_BOSS == 0 && {
                    let (_, _, option) = self.opts_of(t);
                    self.has_sc(t, crate::battle::SC_TRICKDEAD) || (option & 6 != 0 && mdb.race != 4 && mdb.race != 6)
                };
                if dist >= min_chase || fooled || !tu.is_player() {
                    self.mob_unlock(id, now);
                } else if dist > mdb.range.max(1) as i32 {
                    // out of reach: chase
                    if mode & MD_CANMOVE == 0 {
                        self.mob_unlock(id, now);
                        return;
                    }
                    if !self.can_move(id) {
                        return;
                    }
                    self.world.units.get_mut(&id).unwrap().mob_mut().unwrap().skill_state = MobState::Chase;
                    self.mobskill_use(id, now, "");
                    if self.world.units.get(&id).is_none_or(|u| u.mob().is_none_or(|m| m.cast.is_some())) {
                        return;
                    }
                    // already walking: keep on once the half second since the last
                    // re-path is over, or when the path ends next to the target (mob.c:1174)
                    if let Some((wx, wy)) = walk_to {
                        if next_walk < now || path::distance(wx, wy, tx, ty) < 2 {
                            return;
                        }
                    }
                    if !self.mob_can_reach(&map, x, y, tx, ty, min_chase.max(13)) {
                        // no way there (an Ice Wall?): let go
                        self.mob_unlock(id, now);
                        return;
                    }
                    self.world.units.get_mut(&id).unwrap().mob_mut().unwrap().next_walk = now + 500;
                    // first straight at the cell next to the target, then at random ones around it
                    let mut ok = false;
                    let (mut dx, mut dy) = (0, 0);
                    for i in 0..5 {
                        (dx, dy) = if i == 0 {
                            (tx - x - (tx - x).signum(), ty - y - (ty - y).signum())
                        } else {
                            (tx - x + rand_range(-1, 1), ty - y + rand_range(-1, 1))
                        };
                        if self.mob_walk(id, &map, x, y, x + dx, y + dy, false) {
                            ok = true;
                            break;
                        }
                    }
                    if !ok {
                        // attacked from where it cannot go: two steps back (mob.c:1209)
                        let back = |d: i32| if d < 0 { 2 } else if d > 0 { -2 } else { 0 };
                        self.mob_walk(id, &map, x, y, x + back(dx), y + back(dy), false);
                    }
                } else {
                    // in reach: attack
                    self.world.units.get_mut(&id).unwrap().mob_mut().unwrap().skill_state = MobState::Attack;
                    if walk_to.is_some() {
                        self.stop_walking(id);
                    }
                    // mob.c:407 mob_changestate(MS_ATTACK): when the next attack is
                    // due within 2 s, then; else at once, or after its attack
                    // motion if it has just turned on who hit it
                    let next = self.world.units[&id].mob().unwrap().next_attack_at;
                    if next > now && next < now + 2000 {
                        return;
                    }
                    if retaliate {
                        self.world.units.get_mut(&id).unwrap().mob_mut().unwrap().next_attack_at = now + mdb.attack_motion as u64;
                        return;
                    }
                    self.mob_attack(id, t, &mdb, now);
                }
            } else {
                // loot (mob.c:1237)
                if !tu.is_item() || dist >= min_chase {
                    self.mob_unlock(id, now);
                } else if dist > 0 {
                    if mode & MD_CANMOVE == 0 {
                        self.mob_unlock(id, now);
                        return;
                    }
                    if !self.can_move(id) {
                        return;
                    }
                    self.world.units.get_mut(&id).unwrap().mob_mut().unwrap().skill_state = MobState::Loot;
                    self.mobskill_use(id, now, "");
                    if self.world.units.get(&id).is_none_or(|u| u.mob().is_none_or(|m| m.cast.is_some())) {
                        return;
                    }
                    if let Some((wx, wy)) = walk_to {
                        if next_walk < now || (wx, wy) == (tx, ty) {
                            return; // on its way
                        }
                    }
                    self.world.units.get_mut(&id).unwrap().mob_mut().unwrap().next_walk = now + 500;
                    if !self.mob_walk(id, &map, x, y, tx, ty, false) {
                        self.mob_unlock(id, now);
                    }
                } else {
                    // on the item: take it. A full bag lets the oldest go (monster_loot_type 0)
                    if walk_to.is_some() {
                        self.stop_walking(id);
                    }
                    if let Some(UnitKind::Item(fi)) = self.world.units.get(&t).map(|i| i.kind.clone()) {
                        let m = self.world.units.get_mut(&id).unwrap().mob_mut().unwrap();
                        if m.loot.len() >= LOOT_SIZE {
                            m.loot.remove(0);
                        }
                        m.loot.push(fi.item);
                        self.despawn(t, 0);
                    }
                    self.mob_unlock(id, now);
                }
            }
            return;
        }

        // idle / walking skills
        if self.mobskill_use(id, now, "") {
            return;
        }
        // random walk (not slaves that stay with their master)
        let m = self.world.units[&id].mob().unwrap();
        if mode & MD_CANMOVE != 0 && self.can_move(id) && (m.master.is_none() || m.master_dist > 10) && m.next_walk < now {
            self.mob_random_walk(id, &map, x, y, now);
            return;
        }
        if walk_to.is_none() {
            self.world.units.get_mut(&id).unwrap().mob_mut().unwrap().skill_state = MobState::Idle;
        }
    }

    /// mob_randomwalk (mob.c:1009): a cell within 12 (fewer after failures)
    /// that a straight walk reaches, then 3-6 s rest.
    fn mob_random_walk(&mut self, id: u32, map: &str, x: i32, y: i32, now: u64) {
        let fail = self.world.units[&id].mob().unwrap().move_fail as i32;
        let d = (12 - fail).max(5);
        let mut walked = false;
        for _ in 0..20 {
            let (nx, ny) = (x + rand_range(-d, d), y + rand_range(-d, d));
            if (nx, ny) != (x, y) && self.mob_walk(id, map, x, y, nx, ny, true) {
                walked = true;
                break;
            }
        }
        let speed = self.world.units[&id].speed as u64;
        let steps: u64 = self.world.units[&id].walk.as_ref().map(|w| w.path.iter().map(|d| if path::is_diagonal(*d) { speed * 14 / 10 } else { speed }).sum()).unwrap_or(0);
        let m = self.world.units.get_mut(&id).unwrap().mob_mut().unwrap();
        // (after 1000 rounds without a cell mod0631 respawns the monster; here it keeps trying)
        m.move_fail = if walked { 0 } else { m.move_fail + 1 };
        m.next_walk = now + 3000 + rand::random::<u64>() % 3000 + steps;
        m.skill_state = MobState::Walk;
    }

    /// mob_walktoxy. `easy`: only a straight path, no way around obstacles.
    #[allow(clippy::too_many_arguments)]
    fn mob_walk(&mut self, id: u32, map: &str, x0: i32, y0: i32, x1: i32, y1: i32, easy: bool) -> bool {
        let Some(grid) = self.grid(map) else { return false };
        let found = if easy { path::find_easy(&grid, x0, y0, x1, y1) } else { path::find(&grid, x0, y0, x1, y1) };
        let Some(dirs) = found else { return false };
        if dirs.is_empty() {
            return false;
        }
        let now = self.now();
        let tick = self.tick32();
        let u = self.world.units.get_mut(&id).unwrap();
        let first = crate::mapsrv::step_cost(u.speed, dirs[0]) / 2;
        u.walk = Some(Walk { path: dirs, pos: 0, next_at: now + first as u64, to: (x1 as u16, y1 as u16), retarget: None });
        self.area_send(id, packets::unit_move(id, x0 as u16, y0 as u16, x1 as u16, y1 as u16, tick), false);
        true
    }

    /// mob.c:314 mob_attack: a skill of the attack state, or a plain hit.
    fn mob_attack(&mut self, id: u32, target: u32, mdb: &ro_data::db::Mob, now: u64) {
        let lose = |g: &mut Game| {
            if let Some(m) = g.world.units.get_mut(&id).and_then(|u| u.mob_mut()) {
                m.target = None;
            }
        };
        let Some((tx, ty, dead)) = self.world.units.get(&target).map(|t| (t.x as i32, t.y as i32, t.state == 1)) else {
            lose(self);
            return;
        };
        let (x, y) = (self.world.units[&id].x as i32, self.world.units[&id].y as i32);
        if dead || path::distance(x, y, tx, ty) >= 13 || !self.mob_sees(mdb, target) {
            lose(self);
            return;
        }
        // one cell of slack for a monster that can walk (mob.c:345)
        let range = mdb.range.max(1) as i32 + (mdb.mode & MD_CANMOVE != 0) as i32;
        if path::distance(x, y, tx, ty) > range {
            return;
        }
        // face the target
        if (tx - x, ty - y) != (0, 0) {
            self.world.units.get_mut(&id).unwrap().dir = path::dir_of(tx - x, ty - y);
        }
        if let Some(m) = self.world.units.get_mut(&id).and_then(|u| u.mob_mut()) {
            m.min_chase = 13;
            m.skill_state = MobState::Attack;
        }
        if self.mobskill_use(id, now, "") {
            return;
        }
        let r = self.rule_call("weapon_attack", json!({"src": id, "target": target}));
        let delay = r.as_ref().and_then(|v| v.get("adelay")).and_then(crate::status::lua_u64).unwrap_or(mdb.attack_delay as u64).max(100);
        if let Some(m) = self.world.units.get_mut(&id).and_then(|u| u.mob_mut()) {
            m.next_attack_at = now + delay;
        }
    }

    /// mobskill_use (mob.c:2126): go down the monster's skill list; the
    /// first line whose delay, state, condition and chance all pass is cast.
    /// `event` is the condition that just happened ("closedattacked",
    /// "skillused:15", ...) or "" for the regular check. True when a skill
    /// was used.
    pub(crate) fn mobskill_use(&mut self, id: u32, now: u64, event: &str) -> bool {
        let Some(m) = self.world.units.get(&id).and_then(|u| u.mob()) else { return false };
        // ours: one cast at a time (in mod0631 a counter skill could start
        // over a cast in progress and leave both timers running)
        if m.cast.is_some() {
            return false;
        }
        let Some(list) = self.db.mob_skills.get(&m.mob_id).cloned() else { return false };
        let state = m.skill_state.name();
        let (ev_name, ev_val) = event.split_once(':').unwrap_or((event, ""));
        for (i, ms) in list.iter().enumerate() {
            let m = self.world.units[&id].mob().unwrap();
            let used = m.skill_used.get(i).copied().unwrap_or(0);
            if used != 0 && now < used + ms.delay as u64 {
                continue;
            }
            if ms.state != "any" && ms.state != state {
                continue;
            }
            let flag = if !ev_name.is_empty() && ms.cond == ev_name {
                ms.cond != "skillused" || ev_val.parse::<i32>().ok() == Some(ms.cond_val)
            } else {
                match ms.cond.as_str() {
                    "always" => true,
                    "myhpltmaxrate" => (m.hp as u64) < m.max_hp as u64 * ms.cond_val.max(0) as u64 / 100,
                    "slavelt" => self.mob_slaves(id) < ms.cond_val,
                    "slavele" => self.mob_slaves(id) <= ms.cond_val,
                    "attackpcgt" => self.mob_targeted(id) > ms.cond_val,
                    "attackpcge" => self.mob_targeted(id) >= ms.cond_val,
                    _ => false,
                }
            };
            if !flag || rand::random::<u32>() % 1000 >= ms.rate {
                continue;
            }
            let Some(sk) = self.db.skill(ms.skill).cloned() else { continue };
            let m = self.world.units[&id].mob().unwrap();
            let (me_x, me_y) = (self.world.units[&id].x, self.world.units[&id].y);
            let tgt = match ms.target.as_str() {
                "target" => m.target.filter(|_| !m.looting),
                "self" => Some(id),
                _ => continue, // "friend": not implemented in mod0631 either (the line is skipped)
            };
            // from here a failure ends the search (mobskill_use_id returning 0)
            let Some(tgt) = tgt else { return false };
            let Some((tx, ty)) = self.world.units.get(&tgt).map(|t| (t.x, t.y)) else { return false };
            // silenced, stunned, hidden: no skills (mob.c:2011)
            let (opt1, _, option) = self.opts_of(id);
            if opt1 > 0 || option & 6 != 0 || self.has_sc(id, SC_SILENCE) {
                return false;
            }
            // range of the skill itself; a weapon-range skill (below 0) reaches as the monster does
            let range = if sk.range < 0 { mdb_range(self, m.mob_id) - (sk.range + 1) } else { sk.range };
            if range > 0 && path::distance(me_x as i32, me_y as i32, tx as i32, ty as i32) > range {
                return false;
            }
            if let Some(m) = self.world.units.get_mut(&id).and_then(|u| u.mob_mut()) {
                if let Some(u) = m.skill_used.get_mut(i) {
                    *u = now.max(1);
                }
            }
            let place = sk.inf & 2 != 0;
            self.begin_cast(id, ms.skill, ms.lv, if place { 0 } else { tgt }, if place { tx } else { 0 }, if place { ty } else { 0 }, ms.cast as u64, 0, ms.cancel, false, ms.val as i64, i as i32);
            return true;
        }
        false
    }

    /// mobskill_event: a hit landed on the monster (flag: short or long range).
    pub(crate) fn mobskill_event(&mut self, id: u32, event: &str) {
        let now = self.now();
        if self.world.units.get(&id).is_some_and(|u| u.state != 1) {
            self.mobskill_use(id, now, event);
        }
    }

    /// mob_countslave (mob.c:1823): its slaves within 20 cells.
    fn mob_slaves(&self, id: u32) -> i32 {
        let Some((map, x, y)) = self.world.units.get(&id).map(|u| (u.map.clone(), u.x as i32, u.y as i32)) else { return 0 };
        self.world
            .units
            .values()
            .filter(|u| u.map == map && (u.x as i32 - x).abs() <= 20 && (u.y as i32 - y).abs() <= 20)
            .filter(|u| u.mob().is_some_and(|m| m.master == Some(id)))
            .count() as i32
    }

    /// Players attacking this monster right now (mob_counttargeted).
    fn mob_targeted(&self, id: u32) -> i32 {
        self.world.units.values().filter(|u| u.player().is_some_and(|p| p.attack.is_some_and(|a| a.target == id))).count() as i32
    }

    pub(crate) fn player_dies(&mut self, unit: u32) {
        if let Some(u) = self.world.units.get_mut(&unit) {
            u.state = 1;
            u.walk = None;
            if let Some(p) = u.player_mut() {
                p.attack = None;
                p.cast = None;
            }
        }
        self.dialogs.remove(&unit);
        self.sc_clear(unit);
        self.area_send(unit, packets::vanish(unit, 1), true);
        // the pet minds (pc.c:2832)
        self.pet_owner_died(unit);
        // (the EXP penalty is taken on the way back to the save point: respawn_player)
        let here = here_map(&self.world, unit);
        if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
            p.died_on = Some(here);
        }
        self.send_full_status(unit);
    }

    /// "Return to save point" after death (pc.c:208 pc_setrestartvalue):
    /// the HP and SP to come back with, and the death penalty, both from the
    /// rules. The penalty is not taken on a `nopenalty` map.
    pub(crate) fn respawn_player(&mut self, unit: u32) {
        let Some(c) = self.char_of_unit(unit).cloned() else { return };
        let d = self.derive(&c);
        let died_on = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()).and_then(|p| p.died_on.take());
        let was_dead = c.hp == 0;
        if was_dead {
            let (nb, nj) = self.exp_next(&c);
            let free = died_on.as_deref().is_some_and(|m| self.db.map_flag(m, "nopenalty"));
            let r = self
                .scripts
                .as_ref()
                .and_then(|e| {
                    e.rule(
                        "respawn",
                        &json!({"class": c.class, "max_hp": d.max_hp, "max_sp": d.max_sp, "sp": c.sp, "base_exp": c.base_exp, "job_exp": c.job_exp,
                                "next_base": nb, "next_job": nj, "nopenalty": free, "osiris": self.has_card(unit, 4144)}),
                    )
                    .ok()
                    .flatten()
                })
                .unwrap_or(Value::Null);
            let get = |k: &str, def: u64| r.get(k).and_then(crate::status::lua_u64).unwrap_or(def);
            let (hp, sp, base, job) = (get("hp", 1), get("sp", c.sp as u64), get("base", 0), get("job", 0));
            if let Some(cm) = self.char_of_unit_mut(unit) {
                cm.hp = (hp as u32).clamp(1, d.max_hp);
                cm.sp = (sp as u32).min(d.max_sp);
                cm.base_exp = cm.base_exp.saturating_sub(base);
                cm.job_exp = cm.job_exp.saturating_sub(job);
            }
        }
        if let Some(u) = self.world.units.get_mut(&unit) {
            u.state = 0;
        }
        let Point { map, x, y } = c.save_point.clone();
        self.warp_player(unit, &map, x, y);
        self.send_full_status(unit);
    }

    // ------------------------------------------------------------ regen

    /// Natural regen, Athena mod0631 model (pc.c pc_natural_heal_*): clocks
    /// that run while standing (HP) or always (SP), sitting twice as fast;
    /// every `hp_interval` / `sp_interval` ms of clock heals the rule's
    /// amount. The recovery skills (Increase HP/SP Recovery, Spiritual
    /// Cadence) run on clocks of their own. What stops or changes it
    /// (overweight, hiding, Magnificat, Moving HP Recovery) is the rule's.
    pub(crate) fn tick_regen(&mut self, now: u64) {
        let due: Vec<u32> = self
            .world
            .units
            .values()
            .filter(|u| u.player().is_some_and(|p| now >= p.last_regen + 500))
            .map(|u| u.id)
            .collect();
        for id in due {
            // dead, or still loading the map: the clocks stand at zero (pc.c:4036)
            let u = &self.world.units[&id];
            if u.state == 1 || !u.player().is_some_and(|p| p.loaded) {
                let p = self.world.units.get_mut(&id).unwrap().player_mut().unwrap();
                p.last_regen = now;
                (p.hp_acc, p.sp_acc, p.inc_hp_acc, p.inc_sp_acc, p.spirit_acc) = (0, 0, 0, 0, 0);
                continue;
            }
            let sitting = self.world.units[&id].state == 2;
            let walking = self.world.units[&id].walk.is_some();
            let Some(c) = self.char_of_unit(id).cloned() else { continue };
            let d = self.derive(&c);
            let (_, _, option) = self.opts_of(id);
            let r = self
                .scripts
                .as_ref()
                .and_then(|e| {
                    e.rule(
                        "regen",
                        &json!({"max_hp": d.max_hp, "max_sp": d.max_sp, "vit": c.stats[2] as i32 + d.bonus[2], "int": c.stats[3] as i32 + d.bonus[3],
                                "sitting": sitting, "walking": walking, "weight": self.weight_of(&c), "max_weight": d.max_weight,
                                "hiding": option & 6 != 0, "skills": c.skills.iter().map(|(k, v)| (k.to_string(), json!(v))).collect::<serde_json::Map<_, _>>(),
                                "sc": c.sc.keys().map(|k| (k.to_string(), json!(true))).collect::<serde_json::Map<_, _>>()}),
                    )
                    .ok()
                    .flatten()
                })
                .unwrap_or(Value::Null);
            let get = |k: &str, def: u64| r.get(k).and_then(crate::status::lua_u64).unwrap_or(def);
            let (hp_amt, sp_amt) = (get("hp", 0) as u32, get("sp", 0) as u32);
            let (hp_int, sp_int) = (get("hp_interval", 4000).max(100), get("sp_interval", 8000).max(100));
            let (hp_clock, sp_clock) = (get("hp_clock", 100), get("sp_clock", 100)); // percent of real time
            let (inc_hp, inc_sp, inc_int) = (get("inc_hp", 0) as u32, get("inc_sp", 0) as u32, get("inc_interval", 10000).max(100));
            let (sp_hp, sp_sp, sp_int2) = (get("spirit_hp", 0) as u32, get("spirit_sp", 0) as u32, get("spirit_interval", 10000).max(100));
            let (mut heal_hp, mut heal_sp) = (0u32, 0u32);
            let (mut note_hp, mut note_sp) = (0u32, 0u32);
            {
                let p = self.world.units.get_mut(&id).unwrap().player_mut().unwrap();
                let a = now - p.last_regen;
                p.last_regen = now;
                let (ah, asp) = (a * hp_clock / 100, a * sp_clock / 100);
                if ah == 0 || c.hp >= d.max_hp {
                    p.hp_acc = 0;
                    p.inc_hp_acc = 0;
                } else {
                    p.hp_acc += ah;
                    while p.hp_acc >= hp_int {
                        p.hp_acc -= hp_int;
                        heal_hp += hp_amt;
                    }
                    // Increase HP Recovery only counts time standing still
                    if walking || inc_hp == 0 {
                        p.inc_hp_acc = 0;
                    } else {
                        p.inc_hp_acc += ah;
                        while p.inc_hp_acc >= inc_int {
                            p.inc_hp_acc -= inc_int;
                            note_hp += inc_hp;
                        }
                    }
                }
                if c.sp >= d.max_sp || (hp_clock == 0 && sp_clock == 0) {
                    // full, or nothing recovers (overweight, hidden): the clock is zeroed
                    p.sp_acc = 0;
                    p.inc_sp_acc = 0;
                } else {
                    p.sp_acc += asp;
                    while p.sp_acc >= sp_int && asp > 0 {
                        p.sp_acc -= sp_int;
                        heal_sp += sp_amt;
                    }
                    if walking || inc_sp == 0 || asp == 0 {
                        p.inc_sp_acc = 0;
                    } else {
                        p.inc_sp_acc += asp;
                        while p.inc_sp_acc >= inc_int {
                            p.inc_sp_acc -= inc_int;
                            note_sp += inc_sp;
                        }
                    }
                }
                if (sp_hp == 0 && sp_sp == 0) || (c.hp >= d.max_hp && c.sp >= d.max_sp) {
                    p.spirit_acc = 0;
                } else {
                    p.spirit_acc += a;
                    while p.spirit_acc >= sp_int2 {
                        p.spirit_acc -= sp_int2;
                        note_hp += sp_hp;
                        note_sp += sp_sp;
                    }
                }
            }
            if heal_hp == 0 && heal_sp == 0 && note_hp == 0 && note_sp == 0 {
                continue;
            }
            let cm = self.char_of_unit_mut(id).unwrap();
            let (hp0, sp0) = (cm.hp, cm.sp);
            cm.hp = (cm.hp + heal_hp + note_hp).min(d.max_hp);
            cm.sp = (cm.sp + heal_sp + note_sp).min(d.max_sp);
            let (got_hp, got_sp) = ((cm.hp - hp0).saturating_sub(heal_hp), (cm.sp - sp0).saturating_sub(heal_sp));
            if let Some(sid) = self.sid_of(id) {
                // the skills' recovery shows as a number (clif_heal)
                if note_hp > 0 && got_hp > 0 {
                    self.send(sid, packets::heal_notice(crate::status::SP_HP, got_hp.min(32000) as u16));
                }
                if note_sp > 0 && got_sp > 0 {
                    self.send(sid, packets::heal_notice(crate::status::SP_SP, got_sp.min(32000) as u16));
                }
            }
            self.send_hp_sp(id);
        }
    }
}

/// A monster's own reach (its attack range).
fn mdb_range(g: &Game, mob_id: u32) -> i32 {
    g.db.mob(mob_id).map(|d| d.range.max(1) as i32).unwrap_or(1)
}

fn here_map(w: &crate::world::World, unit: u32) -> String {
    w.units.get(&unit).map(|u| u.map.clone()).unwrap_or_default()
}

pub(crate) fn rand_range(a: i32, b: i32) -> i32 {
    if b <= a {
        return a;
    }
    a + (rand::random::<u32>() % (b - a + 1) as u32) as i32
}
