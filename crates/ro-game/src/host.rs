//! `_rt(op, ...)`: the operations scripts can perform on the live world.
//! The friendly names scripts use are defined in prelude.lua; this is the
//! single native entry point behind them. docs/SCRIPTING.md lists them.

use serde_json::{Value, json};

use crate::game::Game;
use crate::model::{InvItem, Point};
use crate::packets;
use crate::script::{Ctx, CtxKind, Host};

fn arg<'a>(a: &'a [Value], i: usize) -> &'a Value {
    a.get(i).unwrap_or(&Value::Null)
}
fn int(a: &[Value], i: usize) -> i64 {
    match arg(a, i) {
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

impl Game {
    /// Item id from a number or an item name ("Red_Potion").
    fn item_id(&self, v: &Value) -> Result<u32, String> {
        match v {
            Value::Number(n) => n.as_u64().map(|x| x as u32).ok_or("bad item id".into()),
            Value::String(s) => match s.parse::<u32>() {
                Ok(n) => Ok(n),
                Err(_) => self.db.item_by_name(s).map(|d| d.id).ok_or(format!("unknown item {s:?}")),
            },
            _ => Err("item id expected".into()),
        }
    }

    fn need_player(&self, ctx: &Ctx) -> Result<u32, String> {
        if ctx.player != 0 && self.world.units.contains_key(&ctx.player) {
            Ok(ctx.player)
        } else {
            Err("no player in this context".into())
        }
    }

    fn player_get(&self, unit: u32, key: &str) -> Value {
        let (Some(u), Some(c)) = (self.world.units.get(&unit), self.char_of_unit(unit)) else { return Value::Null };
        let d = self.derive(c);
        let p = u.player().unwrap();
        match key {
            "name" => json!(c.name),
            "zeny" => json!(c.zeny),
            "base_level" => json!(c.base_level),
            "job_level" => json!(c.job_level),
            "job" | "class" => json!(c.class),
            "hp" => json!(c.hp),
            "sp" => json!(c.sp),
            "max_hp" => json!(d.max_hp),
            "max_sp" => json!(d.max_sp),
            "str" => json!(c.stats[0]),
            "agi" => json!(c.stats[1]),
            "vit" => json!(c.stats[2]),
            "int" => json!(c.stats[3]),
            "dex" => json!(c.stats[4]),
            "luk" => json!(c.stats[5]),
            "status_point" => json!(c.status_point),
            "skill_point" => json!(c.skill_point),
            "base_exp" => json!(c.base_exp),
            "job_exp" => json!(c.job_exp),
            "map" => json!(u.map),
            "x" => json!(u.x),
            "y" => json!(u.y),
            "sex" => json!(p.sex),
            "account_id" => json!(p.account_id),
            "char_id" => json!(c.id),
            "id" => json!(unit),
            "gm_level" => json!(p.gm_level),
            "weight" => json!(self.weight_of(c)),
            "max_weight" => json!(d.max_weight),
            "aspd_delay" => json!(d.amotion),
            "hair" => json!(c.hair),
            "hair_color" => json!(c.hair_color),
            "clothes_color" => json!(c.clothes_color),
            "save_map" => json!(c.save_point.map),
            "next_base_exp" => json!(self.exp_next(c).0),
            "next_job_exp" => json!(self.exp_next(c).1),
            "option" => json!(c.option),
            "karma" => json!(c.karma),
            "manner" => json!(c.manner),
            _ => Value::Null,
        }
    }

    fn player_set(&mut self, unit: u32, key: &str, v: i64) -> Result<(), String> {
        let c = self.char_of_unit_mut(unit).ok_or("no character")?;
        let u16v = v.clamp(0, u16::MAX as i64) as u16;
        let mut look: Option<(u8, u16)> = None;
        match key {
            "zeny" => c.zeny = v.clamp(0, u32::MAX as i64) as u32,
            "hp" => c.hp = v.max(0) as u32,
            "sp" => c.sp = v.max(0) as u32,
            "base_level" => c.base_level = u16v.clamp(1, 99),
            "job_level" => c.job_level = u16v.clamp(1, 50),
            "str" => c.stats[0] = u16v.clamp(1, 99),
            "agi" => c.stats[1] = u16v.clamp(1, 99),
            "vit" => c.stats[2] = u16v.clamp(1, 99),
            "int" => c.stats[3] = u16v.clamp(1, 99),
            "dex" => c.stats[4] = u16v.clamp(1, 99),
            "luk" => c.stats[5] = u16v.clamp(1, 99),
            "status_point" => c.status_point = u16v,
            "skill_point" => c.skill_point = u16v,
            "base_exp" => c.base_exp = v.max(0) as u64,
            "job_exp" => c.job_exp = v.max(0) as u64,
            "hair" => {
                c.hair = u16v;
                look = Some((1, u16v));
            }
            "hair_color" => {
                c.hair_color = u16v;
                look = Some((6, u16v));
            }
            "clothes_color" => {
                c.clothes_color = u16v;
                look = Some((7, u16v));
            }
            other => return Err(format!("player.{other} is read-only or unknown")),
        }
        // keep hp/sp within the (possibly new) maxima
        let c2 = self.char_of_unit(unit).cloned().unwrap();
        let d = self.derive(&c2);
        if let Some(c) = self.char_of_unit_mut(unit) {
            c.hp = c.hp.min(d.max_hp);
            c.sp = c.sp.min(d.max_sp);
        }
        if let Some((kind, val)) = look {
            self.area_send(unit, packets::look(unit, kind, val as u8), true);
        }
        self.send_full_status(unit);
        Ok(())
    }

    fn heal(&mut self, unit: u32, hp: i64, sp: i64) {
        let Some(c) = self.char_of_unit(unit).cloned() else { return };
        let d = self.derive(&c);
        if let Some(c) = self.char_of_unit_mut(unit) {
            c.hp = (c.hp as i64 + hp).clamp(0, d.max_hp as i64) as u32;
            c.sp = (c.sp as i64 + sp).clamp(0, d.max_sp as i64) as u32;
        }
        self.send_hp_sp(unit);
    }

    fn mes_target(&self, ctx: &Ctx) -> Result<(u32, u32), String> {
        let unit = self.need_player(ctx)?;
        let sid = self.sid_of(unit).ok_or("player has no session")?;
        if !matches!(ctx.kind, CtxKind::Dialog | CtxKind::Touch | CtxKind::Event) {
            return Err("dialog functions only work in NPC scripts (use notify() elsewhere)".into());
        }
        Ok((unit, sid))
    }
}

impl Host for Game {
    fn call(&mut self, ctx: &Ctx, op: &str, a: Vec<Value>) -> Result<Value, String> {
        match op {
            // ---- dialog
            "mes" => {
                let (unit, sid) = self.mes_target(ctx)?;
                self.send(sid, packets::npc_mes(ctx.npc, &text(&a, 0)));
                self.dialog_open.insert(unit, true);
            }
            "next" => {
                let (_, sid) = self.mes_target(ctx)?;
                self.send(sid, packets::npc_next(ctx.npc));
            }
            "close" => {
                let (unit, sid) = self.mes_target(ctx)?;
                self.send(sid, packets::npc_close(ctx.npc));
                self.dialog_open.insert(unit, false);
            }
            "menu" => {
                let (_, sid) = self.mes_target(ctx)?;
                let opts: Vec<String> = match arg(&a, 0) {
                    Value::Array(v) => v.iter().map(|x| x.as_str().map(String::from).unwrap_or_else(|| x.to_string())).collect(),
                    _ => vec![],
                };
                self.send(sid, packets::npc_menu(ctx.npc, &opts));
            }
            "input_number" => {
                let (_, sid) = self.mes_target(ctx)?;
                self.send(sid, packets::npc_input_number(ctx.npc));
            }
            "input_string" => {
                let (_, sid) = self.mes_target(ctx)?;
                self.send(sid, packets::npc_input_string(ctx.npc));
            }
            "cutin" => {
                let unit = self.need_player(ctx)?;
                if let Some(sid) = self.sid_of(unit) {
                    self.send(sid, packets::cutin(&text(&a, 0), int(&a, 1) as u8));
                }
            }
            // ---- player
            "player_get" => {
                let unit = self.need_player(ctx)?;
                return Ok(self.player_get(unit, &text(&a, 0)));
            }
            "player_set" => {
                let unit = self.need_player(ctx)?;
                self.player_set(unit, &text(&a, 0), int(&a, 1))?;
            }
            "var_get" | "var_set" | "var_clear" => {
                let unit = self.need_player(ctx)?;
                let key = text(&a, 0);
                let c = self.char_of_unit_mut(unit).ok_or("no character")?;
                match op {
                    "var_get" => return Ok(c.vars.get(&key).cloned().unwrap_or(Value::Null)),
                    "var_set" => match arg(&a, 1) {
                        Value::Null => {
                            c.vars.remove(&key);
                        }
                        v => {
                            c.vars.insert(key, v.clone());
                        }
                    },
                    _ => {
                        c.vars.remove(&key);
                    }
                }
            }
            "give_item" => {
                let unit = self.need_player(ctx)?;
                let id = self.item_id(arg(&a, 0))?;
                let n = int(&a, 1).max(1) as u32;
                // equipment comes piece by piece; arrows are worn too but stack
                let d = self.db.item(id).ok_or(format!("unknown item {id}"))?;
                let equip = d.is_equip() && d.kind != 10;
                let ok = if equip { (0..n).all(|_| self.add_item(unit, InvItem::new(id, 1))) } else { self.add_item(unit, InvItem::new(id, n)) };
                return Ok(json!(ok));
            }
            "take_item" => {
                let unit = self.need_player(ctx)?;
                let id = self.item_id(arg(&a, 0))?;
                return Ok(json!(self.take_item(unit, id, int(&a, 1).max(1) as u32)));
            }
            "count_item" => {
                let unit = self.need_player(ctx)?;
                let id = self.item_id(arg(&a, 0))?;
                return Ok(json!(self.count_item(unit, id)));
            }
            "heal" => {
                let unit = self.need_player(ctx)?;
                self.heal(unit, int(&a, 0), int(&a, 1));
            }
            "percent_heal" => {
                let unit = self.need_player(ctx)?;
                let c = self.char_of_unit(unit).cloned().ok_or("no character")?;
                let d = self.derive(&c);
                self.heal(unit, d.max_hp as i64 * int(&a, 0) / 100, d.max_sp as i64 * int(&a, 1) / 100);
            }

            "save_point" => {
                let unit = self.need_player(ctx)?;
                let map = text(&a, 0).to_ascii_lowercase();
                let map = map.strip_suffix(".gat").unwrap_or(&map);
                let p = Point::new(map, int(&a, 1) as u16, int(&a, 2) as u16);
                self.char_of_unit_mut(unit).ok_or("no character")?.save_point = p;
            }
            "job_change" => {
                let unit = self.need_player(ctx)?;
                let job = int(&a, 0) as u16;
                if !self.db.jobs.contains_key(&job) {
                    return Err(format!("unknown job {job}"));
                }
                if let Some(c) = self.char_of_unit_mut(unit) {
                    c.class = job;
                    c.job_level = 1;
                    c.job_exp = 0;
                }
                if let Some(u) = self.world.units.get_mut(&unit) {
                    u.class = job;
                }
                // unequip what the new job can't wear
                let bad: Vec<u16> = self
                    .char_of_unit(unit)
                    .map(|c| {
                        c.inventory
                            .iter()
                            .enumerate()
                            .filter(|(_, it)| it.equip != 0 && self.db.item(it.nameid).is_some_and(|d| d.jobs & (1 << job.min(31)) == 0))
                            .map(|(i, _)| i as u16 + packets::INV_OFFSET)
                            .collect()
                    })
                    .unwrap_or_default();
                for idx in bad {
                    self.unequip(unit, idx);
                }
                self.area_send(unit, packets::look(unit, 0, job as u8), true);
                // pc_jobchange ends in pc_calcstatus, which brings HP and SP down to the
                // new job's maxima (mod0631 pc.c:3122, 983)
                self.refresh_status(unit); // sends the status window too
                self.send_skill_list(unit);
            }
            "give_exp" => {
                let unit = self.need_player(ctx)?;
                self.gain_exp(unit, int(&a, 0).max(0) as u64, int(&a, 1).max(0) as u64);
            }
            "emotion" => {
                let who = if arg(&a, 1).as_bool().unwrap_or(false) && ctx.npc != 0 { ctx.npc } else { self.need_player(ctx)? };
                let pkt = packets::emotion(who, int(&a, 0) as u8);
                for o in self.world.units.values() {
                    if let Some(p) = o.player() {
                        if o.id == who || p.visible.contains(&who) {
                            self.send(p.sid, pkt.clone());
                        }
                    }
                }
            }
            "notify" => {
                let unit = self.need_player(ctx)?;
                self.tell(unit, &text(&a, 0));
            }
            "open_shop" => {
                let unit = self.need_player(ctx)?;
                let name = text(&a, 0);
                let npc = self.world.npc_by_name(&name).ok_or(format!("no shop named {name}"))?;
                if let Some(p) = self.world.units.get_mut(&unit).and_then(|u| u.player_mut()) {
                    p.shop = Some(npc);
                    let sid = p.sid;
                    self.send(sid, packets::shop_select(npc));
                }
            }
            // ---- world
            "spawn_mob" => {
                let map = text(&a, 0);
                if !self.ensure_map(&map) {
                    return Err(format!("unknown map {map}"));
                }
                let mob = match arg(&a, 3) {
                    Value::String(s) => self.db.mob_by_name(s).map(|m| m.id).ok_or(format!("unknown monster {s}"))?,
                    v => v.as_u64().unwrap_or(0) as u32,
                };
                let mut ids = Vec::new();
                for _ in 0..int(&a, 4).max(1) {
                    if let Some(id) = self.spawn_mob(&map, int(&a, 1) as u16, int(&a, 2) as u16, mob, None) {
                        ids.push(id);
                    }
                }
                return Ok(json!(ids));
            }
            "walkable" => {
                let map = text(&a, 0).to_ascii_lowercase();
                if !self.ensure_map(&map) {
                    return Ok(Value::Null);
                }
                return Ok(json!(self.grid(&map).is_some_and(|g| g.walkable(int(&a, 1) as i32, int(&a, 2) as i32))));
            }
            "map_size" => {
                let map = text(&a, 0).to_ascii_lowercase();
                if !self.ensure_map(&map) {
                    return Ok(Value::Null);
                }
                let g = self.grid(&map).unwrap();
                return Ok(json!([g.width, g.height]));
            }
            "nearby" => {
                // units around the player: [{id, kind, name, x, y, hp?}]
                let unit = self.need_player(ctx)?;
                let range = int(&a, 0).clamp(1, 30) as i32;
                let me = self.world.units.get(&unit).ok_or("no unit")?;
                let mut out = Vec::new();
                for id in self.world.area_ids(&me.map, me.x, me.y) {
                    let Some(u) = self.world.units.get(&id) else { continue };
                    if id == unit || crate::path::distance(me.x as i32, me.y as i32, u.x as i32, u.y as i32) > range {
                        continue;
                    }
                    let (kind, extra) = match &u.kind {
                        crate::world::UnitKind::Player(_) => ("player", json!(null)),
                        crate::world::UnitKind::Npc(_) => ("npc", json!(null)),
                        crate::world::UnitKind::Mob(m) => ("mob", json!({"mob_id": m.mob_id, "hp": m.hp, "max_hp": m.max_hp})),
                        crate::world::UnitKind::Item(fi) => ("item", json!({"item": fi.item.nameid, "amount": fi.item.amount})),
                        crate::world::UnitKind::Pet(p) => ("pet", json!({"owner": p.owner, "pet_id": p.pet_id})),
                        crate::world::UnitKind::Skill(s) => ("skill", json!({"unit_id": s.unit_id, "src": s.src, "group": s.group})),
                    };
                    out.push(json!({"id": id, "kind": kind, "name": u.name, "x": u.x, "y": u.y, "info": extra}));
                }
                return Ok(Value::Array(out));
            }
            "item_name" => {
                let id = self.item_id(arg(&a, 0))?;
                return Ok(self.db.item(id).map(|d| json!(d.display)).unwrap_or(Value::Null));
            }
            "mob_name" => {
                return Ok(self.db.mob(int(&a, 0) as u32).map(|d| json!(d.display)).unwrap_or(Value::Null));
            }
            "online" => {
                let names: Vec<String> = self.world.all_players().iter().filter_map(|p| self.world.units.get(p).map(|u| u.name.clone())).collect();
                return Ok(json!(names));
            }
            "log" => tracing::info!(target: "script", "{}", text(&a, 0)),
            "reload" => {
                return self.reload_scripts().map(Value::from).map_err(|e| format!("{e:#}"));
            }
            other => {
                if let Some(r) = self.npc_call(ctx, other, &a) {
                    return r;
                }
                return match self.battle_call(ctx, other, &a) {
                    Some(r) => r,
                    None => Err(format!("unknown runtime op {other:?}")),
                };
            }
        }
        Ok(Value::Null)
    }
}
