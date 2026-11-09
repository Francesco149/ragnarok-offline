//! Monsters, fighting, death, EXP, drops, regen.
//!
//! The *timing and packets* are protocol facts; the *numbers* (damage, hit,
//! EXP penalties, regen) are server-side and come from Lua `rules.*`
//! (scripts/rules/combat.lua) with crude Rust fallbacks. Monster AI here is a
//! simplified Athena-style reconstruction: see docs/ROADMAP.md.

use serde_json::{Value, json};

use crate::game::Game;
use crate::model::{InvItem, Point};
use crate::packets;
use crate::path;
use crate::world::{AttackIntent, DEFAULT_SPEED, MobUnit, Unit, UnitKind, Walk};

const MOB_THINK_MS: u64 = 100;
const DEFAULT_RESPAWN_MS: u32 = 5000;

// Athena monster mode bits
const MD_CANMOVE: u32 = 0x01;
const MD_AGGRESSIVE: u32 = 0x04;
const MD_CANATTACK: u32 = 0x80;

#[derive(Debug, Default)]
struct Hit {
    damage: u32,
    /// "hit" | "crit" | "miss"
    kind: String,
}

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
        self.spawn_mob(&s.map, x, y, s.mob, Some(i))
    }

    pub(crate) fn spawn_mob(&mut self, map: &str, x: u16, y: u16, mob_id: u32, spawn: Option<usize>) -> Option<u32> {
        let m = self.db.mob(mob_id)?.clone();
        let id = self.world.new_obj_id();
        let now = self.now();
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
        if me.state == 2 {
            me.state = 0;
        }
        let Some(p) = me.player_mut() else { return };
        p.attack = Some(intent);
        let range = self.char_of_unit(unit).map(|c| self.derive(c).range.max(1)).unwrap_or(1) as i32;
        let me = &self.world.units[&unit];
        if path::distance(me.x as i32, me.y as i32, tx as i32, ty as i32) > range {
            self.walk_to(unit, tx, ty);
        }
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
        // monsters on maps with players
        let mobs: Vec<u32> = self
            .world
            .units
            .values()
            .filter(|u| u.mob().is_some_and(|m| m.next_think <= now))
            .filter(|u| self.world.maps.get(&u.map).is_some_and(|m| m.players > 0))
            .map(|u| u.id)
            .collect();
        for id in mobs {
            self.mob_think(id, now);
        }
    }

    fn player_attack_step(&mut self, id: u32, now: u64) {
        let Some(intent) = self.world.units.get(&id).and_then(|u| u.player()).and_then(|p| p.attack) else { return };
        let target_ok = self.world.units.get(&intent.target).is_some_and(|t| t.mob().is_some() && t.state != 1);
        if !target_ok {
            self.world.units.get_mut(&id).unwrap().player_mut().unwrap().attack = None;
            return;
        }
        let Some(c) = self.char_of_unit(id).cloned() else { return };
        let d = self.derive(&c);
        let (me, t) = (&self.world.units[&id], &self.world.units[&intent.target]);
        let mob = t.mob().unwrap().clone();
        let dist = path::distance(me.x as i32, me.y as i32, t.x as i32, t.y as i32);
        if dist > d.range.max(1) as i32 {
            if me.walk.is_none() {
                let (tx, ty) = (t.x, t.y);
                self.walk_to(id, tx, ty);
            }
            return;
        }
        if me.walk.is_some() {
            // in range: stop where we are
            let (x, y) = (me.x, me.y);
            self.world.units.get_mut(&id).unwrap().walk = None;
            self.area_send(id, packets::stop_move(id, x, y), true);
        }
        let mdb = self.db.mob(mob.mob_id).cloned().unwrap_or_default();
        let hit = self.roll_attack(
            json!({"kind": "player", "char": self.char_lua(&c), "derived": derive_json(&d)}),
            json!({"kind": "mob", "mob": mdb}),
        );
        let tick = self.tick32();
        let kind = match hit.kind.as_str() {
            "crit" => 10,
            "miss" => 0,
            _ => 0,
        };
        let dmg = if hit.kind == "miss" { 0 } else { hit.damage };
        self.area_send(
            id,
            packets::action(id, intent.target, tick, d.amotion, mdb.damage_motion, dmg.min(32000) as u16, 1, kind, 0),
            true,
        );
        {
            let p = self.world.units.get_mut(&id).unwrap().player_mut().unwrap();
            p.next_attack_at = now + d.amotion as u64;
            if !intent.continuous {
                p.attack = None;
            }
        }
        self.damage_mob(intent.target, id, dmg);
    }

    fn roll_attack(&self, attacker: Value, defender: Value) -> Hit {
        if let Some(engine) = &self.scripts {
            match engine.rule("attack", &json!({"attacker": attacker, "defender": defender})) {
                Ok(Some(v)) => {
                    return Hit {
                        damage: v.get("damage").and_then(crate::status::lua_u64).unwrap_or(0) as u32,
                        kind: v.get("kind").and_then(|k| k.as_str()).unwrap_or("hit").to_string(),
                    };
                }
                Ok(None) => {}
                Err(e) => tracing::warn!("{e:#}"),
            }
        }
        Hit { damage: 1 + rand::random::<u32>() % 10, kind: "hit".into() }
    }

    fn damage_mob(&mut self, mob: u32, by: u32, dmg: u32) {
        let Some(u) = self.world.units.get_mut(&mob) else { return };
        let Some(m) = u.mob_mut() else { return };
        *m.damage_log.entry(by).or_default() += dmg;
        if m.target.is_none() {
            m.target = Some(by);
        }
        m.hp = m.hp.saturating_sub(dmg);
        if m.hp == 0 {
            self.mob_dies(mob);
        }
    }

    fn mob_dies(&mut self, mob: u32) {
        let Some(u) = self.world.units.get(&mob).cloned() else { return };
        let Some(m) = u.mob() else { return };
        let mdb = self.db.mob(m.mob_id).cloned().unwrap_or_default();
        self.despawn(mob, 1);
        // EXP: whole amount to everyone who hit it (single player: you)
        for (&pid, _) in &m.damage_log {
            if self.world.units.get(&pid).is_some_and(|p| p.is_player()) {
                let base = mdb.base_exp as u64 * self.cfg.base_exp_rate as u64 / 100;
                let job = mdb.job_exp as u64 * self.cfg.job_exp_rate as u64 / 100;
                self.gain_exp(pid, base, job);
            }
        }
        // drops (Athena: rate/10000, scaled by drop_rate)
        for d in &mdb.drops {
            let rate = (d.rate as u64 * self.cfg.drop_rate as u64 / 100).min(10000) as u32;
            if rand::random::<u32>() % 10000 < rate && self.db.item(d.item).is_some() {
                let (dx, dy) = (rand_range(-1, 1), rand_range(-1, 1));
                let (x, y) = ((u.x as i32 + dx).max(0) as u16, (u.y as i32 + dy).max(0) as u16);
                let (x, y) = if self.grid(&u.map).is_some_and(|g| g.walkable(x as i32, y as i32)) { (x, y) } else { (u.x, u.y) };
                self.drop_floor_item(&u.map, x, y, InvItem::new(d.item, 1), true);
            }
        }
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

    fn mob_think(&mut self, id: u32, now: u64) {
        let Some(u) = self.world.units.get(&id) else { return };
        let Some(m) = u.mob() else { return };
        let mdb = self.db.mob(m.mob_id).cloned().unwrap_or_default();
        let mode = mdb.mode; // 0 = plant-like: never moves or attacks
        let (map, x, y) = (u.map.clone(), u.x, u.y);
        let mut target = m.target;
        // drop targets that are gone, dead, or too far
        if let Some(t) = target {
            let keep = self.world.units.get(&t).is_some_and(|tu| {
                tu.map == map && tu.state != 1 && path::distance(x as i32, y as i32, tu.x as i32, tu.y as i32) <= mdb.chase_range.max(12) as i32
            });
            if !keep {
                target = None;
            }
        }
        // aggressive monsters pick the nearest player in sight
        if target.is_none() && mode & MD_AGGRESSIVE != 0 {
            let view = mdb.view_range.max(1) as i32;
            target = self
                .world
                .players_on(&map)
                .into_iter()
                .filter_map(|pid| self.world.units.get(&pid))
                .filter(|p| p.state != 1 && path::distance(x as i32, y as i32, p.x as i32, p.y as i32) <= view)
                .min_by_key(|p| path::distance(x as i32, y as i32, p.x as i32, p.y as i32))
                .map(|p| p.id);
        }
        let walking = u.walk.is_some();
        {
            let m = self.world.units.get_mut(&id).unwrap().mob_mut().unwrap();
            m.target = target;
            m.next_think = now + MOB_THINK_MS;
        }
        match target {
            Some(t) if mode & MD_CANATTACK != 0 => {
                let tu = &self.world.units[&t];
                let (tx, ty) = (tu.x, tu.y);
                let dist = path::distance(x as i32, y as i32, tx as i32, ty as i32);
                if dist <= mdb.range.max(1) as i32 {
                    if walking {
                        self.world.units.get_mut(&id).unwrap().walk = None;
                        self.area_send(id, packets::stop_move(id, x, y), false);
                    }
                    let next = self.world.units[&id].mob().unwrap().next_attack_at;
                    if next <= now {
                        self.mob_attack(id, t, &mdb, now);
                    }
                } else if mode & MD_CANMOVE != 0 && !walking {
                    self.mob_walk(id, &map, x, y, tx, ty);
                }
            }
            _ => {
                // idle: wander now and then (Athena random walk, simplified)
                if mode & MD_CANMOVE != 0 && !walking && rand::random::<u32>() % 60 == 0 {
                    let (dx, dy) = (rand_range(-5, 5), rand_range(-5, 5));
                    let (nx, ny) = ((x as i32 + dx).max(0) as u16, (y as i32 + dy).max(0) as u16);
                    self.mob_walk(id, &map, x, y, nx, ny);
                }
            }
        }
    }

    fn mob_walk(&mut self, id: u32, map: &str, x0: u16, y0: u16, x1: u16, y1: u16) {
        let Some(grid) = self.grid(map) else { return };
        let Some(dirs) = path::find(&grid, x0 as i32, y0 as i32, x1 as i32, y1 as i32) else { return };
        if dirs.is_empty() {
            return;
        }
        let now = self.now();
        let tick = self.tick32();
        let u = self.world.units.get_mut(&id).unwrap();
        let first = crate::mapsrv::step_cost(u.speed, dirs[0]) / 2;
        u.walk = Some(Walk { path: dirs, pos: 0, next_at: now + first as u64, to: (x1, y1), retarget: None });
        self.area_send(id, packets::unit_move(id, x0, y0, x1, y1, tick), false);
    }

    fn mob_attack(&mut self, id: u32, target: u32, mdb: &ro_data::db::Mob, now: u64) {
        let Some(c) = self.char_of_unit(target).cloned() else { return };
        let d = self.derive(&c);
        let hit = self.roll_attack(
            json!({"kind": "mob", "mob": mdb}),
            json!({"kind": "player", "char": self.char_lua(&c), "derived": derive_json(&d)}),
        );
        let dmg = if hit.kind == "miss" { 0 } else { hit.damage };
        let tick = self.tick32();
        self.area_send(
            id,
            packets::action(id, target, tick, mdb.attack_motion, 0, dmg.min(32000) as u16, 1, if hit.kind == "crit" { 10 } else { 0 }, 0),
            false,
        );
        if let Some(m) = self.world.units.get_mut(&id).and_then(|u| u.mob_mut()) {
            m.next_attack_at = now + mdb.attack_delay.max(500) as u64;
        }
        if dmg > 0 {
            let dead = {
                let Some(c) = self.char_of_unit_mut(target) else { return };
                c.hp = c.hp.saturating_sub(dmg);
                c.hp == 0
            };
            if let Some(u) = self.world.units.get_mut(&target) {
                if u.state == 2 {
                    u.state = 0;
                }
            }
            self.send_hp_sp(target);
            if dead {
                self.player_dies(target);
            }
        }
    }

    fn player_dies(&mut self, unit: u32) {
        if let Some(u) = self.world.units.get_mut(&unit) {
            u.state = 1;
            u.walk = None;
            if let Some(p) = u.player_mut() {
                p.attack = None;
            }
        }
        self.dialogs.remove(&unit);
        self.area_send(unit, packets::vanish(unit, 1), true);
        // death EXP penalty from rules (Athena: 1% of next level, not novices)
        let penalty = self.char_of_unit(unit).cloned().and_then(|c| {
            let (nb, nj) = self.exp_next(&c);
            self.scripts.as_ref().and_then(|e| {
                e.rule("death_penalty", &json!({"class": c.class, "next_base": nb, "next_job": nj})).ok().flatten()
            })
        });
        if let Some(v) = penalty {
            let b = v.get("base").and_then(crate::status::lua_u64).unwrap_or(0);
            let j = v.get("job").and_then(crate::status::lua_u64).unwrap_or(0);
            if let Some(c) = self.char_of_unit_mut(unit) {
                c.base_exp = c.base_exp.saturating_sub(b);
                c.job_exp = c.job_exp.saturating_sub(j);
            }
        }
    }

    /// "Return to save point" after death.
    pub(crate) fn respawn_player(&mut self, unit: u32) {
        let Some(c) = self.char_of_unit(unit).cloned() else { return };
        let d = self.derive(&c);
        let hp = self
            .scripts
            .as_ref()
            .and_then(|e| e.rule("respawn_hp", &json!({"class": c.class, "max_hp": d.max_hp})).ok().flatten())
            .and_then(|v| crate::status::lua_u64(&v))
            .unwrap_or(1) as u32;
        if let Some(cm) = self.char_of_unit_mut(unit) {
            if cm.hp == 0 {
                cm.hp = hp.max(1);
            }
        }
        if let Some(u) = self.world.units.get_mut(&unit) {
            u.state = 0;
        }
        let Point { map, x, y } = c.save_point.clone();
        self.warp_player(unit, &map, x, y);
        self.send_hp_sp(unit);
    }

    // ------------------------------------------------------------ regen

    /// Natural regen, Athena mod0631 model: a clock runs while not walking
    /// (HP) / always (SP), sitting runs it twice as fast, and every
    /// `hp_interval` / `sp_interval` ms of clock heals the rule's amount.
    pub(crate) fn tick_regen(&mut self, now: u64) {
        let due: Vec<u32> = self
            .world
            .units
            .values()
            .filter(|u| u.player().is_some_and(|p| p.loaded && now >= p.last_regen + 500) && u.state != 1)
            .map(|u| u.id)
            .collect();
        for id in due {
            let sitting = self.world.units[&id].state == 2;
            let walking = self.world.units[&id].walk.is_some();
            let Some(c) = self.char_of_unit(id).cloned() else { continue };
            let d = self.derive(&c);
            let r = self
                .scripts
                .as_ref()
                .and_then(|e| {
                    e.rule("regen", &json!({"max_hp": d.max_hp, "max_sp": d.max_sp, "vit": c.stats[2], "int": c.stats[3]}))
                        .ok()
                        .flatten()
                })
                .unwrap_or(Value::Null);
            let get = |k: &str, def: u64| r.get(k).and_then(crate::status::lua_u64).unwrap_or(def);
            let (hp_amt, sp_amt) = (get("hp", 0) as u32, get("sp", 0) as u32);
            let (hp_int, sp_int) = (get("hp_interval", 4000).max(100), get("sp_interval", 8000).max(100));
            let sit_factor = get("sit_factor", 2);
            let (mut heal_hp, mut heal_sp) = (0u32, 0u32);
            {
                let p = self.world.units.get_mut(&id).unwrap().player_mut().unwrap();
                let mut a = now - p.last_regen;
                p.last_regen = now;
                if sitting {
                    a *= sit_factor;
                }
                if walking || c.hp >= d.max_hp {
                    p.hp_acc = 0;
                } else {
                    p.hp_acc += a;
                    while p.hp_acc >= hp_int {
                        p.hp_acc -= hp_int;
                        heal_hp += hp_amt;
                    }
                }
                if c.sp >= d.max_sp {
                    p.sp_acc = 0;
                } else {
                    p.sp_acc += a;
                    while p.sp_acc >= sp_int {
                        p.sp_acc -= sp_int;
                        heal_sp += sp_amt;
                    }
                }
            }
            if heal_hp == 0 && heal_sp == 0 {
                continue;
            }
            let cm = self.char_of_unit_mut(id).unwrap();
            cm.hp = (cm.hp + heal_hp).min(d.max_hp);
            cm.sp = (cm.sp + heal_sp).min(d.max_sp);
            self.send_hp_sp(id);
        }
    }
}

fn derive_json(d: &crate::status::Derived) -> Value {
    json!({"max_hp": d.max_hp, "atk": d.atk, "atk2": d.atk2, "def": d.def, "def2": d.def2,
           "hit": d.hit, "flee": d.flee, "flee2": d.flee2, "crit": d.crit, "amotion": d.amotion})
}

pub(crate) fn rand_range(a: i32, b: i32) -> i32 {
    if b <= a {
        return a;
    }
    a + (rand::random::<u32>() % (b - a + 1) as u32) as i32
}
