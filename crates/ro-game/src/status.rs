//! Derived stats, EXP, levels, stat points.
//!
//! Formulas are server-side, so they live in Lua (`scripts/rules/status.lua`,
//! sourced from Athena mod0631 `map/pc.c`, 2003-12-18). The Rust fallbacks
//! below are only used when no rule is defined.

use serde_json::{Value, json};

use crate::game::Game;
use crate::model::Character;
use crate::packets::{self, CharView, FullStatus};

// status parameter ids (0x00b0 / 0x00b1 / 0x0141 / 0x00be)
pub const SP_SPEED: u16 = 0;
pub const SP_BASEEXP: u16 = 1;
pub const SP_JOBEXP: u16 = 2;
pub const SP_HP: u16 = 5;
pub const SP_MAXHP: u16 = 6;
pub const SP_SP: u16 = 7;
pub const SP_MAXSP: u16 = 8;
pub const SP_STATUSPOINT: u16 = 9;
pub const SP_BASELEVEL: u16 = 11;
pub const SP_SKILLPOINT: u16 = 12;
pub const SP_STR: u16 = 13;
pub const SP_ZENY: u16 = 20;
pub const SP_NEXTBASEEXP: u16 = 22;
pub const SP_NEXTJOBEXP: u16 = 23;
pub const SP_WEIGHT: u16 = 24;
pub const SP_MAXWEIGHT: u16 = 25;
pub const SP_USTR: u16 = 32;
pub const SP_ASPD: u16 = 53;
pub const SP_JOBLEVEL: u16 = 55;

#[derive(Debug, Clone, Default)]
pub struct Derived {
    pub max_hp: u32,
    pub max_sp: u32,
    pub atk: u32,
    pub atk2: u32,
    pub matk_min: u32,
    pub matk_max: u32,
    pub def: u32,
    pub def2: u32,
    pub mdef: u32,
    pub mdef2: u32,
    pub hit: u32,
    pub flee: u32,
    pub flee2: u32,
    /// In tenths of a percent.
    pub crit: u32,
    /// Attack delay in ms ("amotion"); client ASPD = (2000 - amotion) / 10.
    pub amotion: u32,
    pub max_weight: u32,
    pub range: u32,
    /// Walk speed, ms per cell.
    pub speed: u32,
    /// How long a hit holds the character (damage motion), ms.
    pub dmotion: u32,
    /// Stat bonuses (equipment, job, status changes): str..luk, shown as "+n".
    pub bonus: [i32; 6],
}

/// A Lua number as u64 (Lua 5.4 hands back floats from `^` and `/`).
pub(crate) fn lua_u64(v: &Value) -> Option<u64> {
    v.as_u64().or_else(|| v.as_i64().map(|i| i.max(0) as u64)).or_else(|| v.as_f64().map(|f| f.max(0.0) as u64))
}

impl Derived {
    fn from_lua(v: &Value) -> Option<Derived> {
        let o = v.as_object()?;
        let g = |k: &str| o.get(k).and_then(lua_u64).unwrap_or(0) as u32;
        Some(Derived {
            max_hp: g("max_hp").max(1),
            max_sp: g("max_sp").max(1),
            atk: g("atk"),
            atk2: g("atk2"),
            matk_min: g("matk_min"),
            matk_max: g("matk_max"),
            def: g("def"),
            def2: g("def2"),
            mdef: g("mdef"),
            mdef2: g("mdef2"),
            hit: g("hit"),
            flee: g("flee"),
            flee2: g("flee2"),
            crit: g("crit"),
            amotion: g("amotion").max(20),
            max_weight: g("max_weight"),
            range: g("range").max(1),
            speed: match g("speed") {
                0 => 150,
                v => v,
            },
            dmotion: g("dmotion"),
            bonus: ["bstr", "bagi", "bvit", "bint", "bdex", "bluk"].map(|k| o.get(k).and_then(Value::as_i64).unwrap_or(0) as i32),
        })
    }
}

/// Status point cost to raise a stat from `v` (pre-renewal).
pub fn stat_cost(v: u16) -> u8 {
    ((v.saturating_sub(1)) / 10 + 2) as u8
}

impl Game {
    /// Equipped items, with db data, as Lua sees them: what Athena's
    /// pc_calcstatus walks over (item, refine, cards, each one's bonus script).
    fn equip_summary(&self, c: &Character) -> Value {
        let mut equips = Vec::new();
        for it in c.inventory.iter().filter(|i| i.equip != 0) {
            let Some(d) = self.db.item(it.nameid) else { continue };
            // card[0] 0x00ff = forged weapon: card[1] = star crumbs << 8 | element
            let forged = it.cards[0] == 0x00ff;
            let cards: Vec<Value> = if forged || it.cards[0] == 0xff00 {
                vec![]
            } else {
                it.cards
                    .iter()
                    .take(d.slots as usize)
                    .filter(|&&c| c != 0)
                    .filter_map(|&c| self.db.item(c as u32))
                    .map(|cd| json!({"id": cd.id, "script": cd.athena_equip}))
                    .collect()
            };
            equips.push(json!({
                "id": d.id, "type": d.kind, "atk": d.atk, "def": d.def, "loc": it.equip, "refine": it.refine, "weight": d.weight,
                "wlv": d.weapon_level, "view": d.view, "range": d.range, "script": d.athena_equip, "cards": cards,
                "star": if forged { it.cards[1] >> 8 } else { 0 }, "forged_ele": if forged { it.cards[1] & 0x0f } else { 0 },
            }));
        }
        json!({"items": equips})
    }

    pub(crate) fn char_lua(&self, c: &Character) -> Value {
        let job = self.db.jobs.get(&c.class);
        let cart_weight: u32 = c.cart.iter().map(|i| self.db.item(i.nameid).map(|d| d.weight).unwrap_or(0) * i.amount).sum();
        json!({
            "class": c.class, "base_level": c.base_level, "job_level": c.job_level,
            "str": c.stats[0], "agi": c.stats[1], "vit": c.stats[2],
            "int": c.stats[3], "dex": c.stats[4], "luk": c.stats[5],
            "hp": c.hp, "sp": c.sp, "zeny": c.zeny, "option": c.option,
            "weight": self.weight_of(c), "cart_weight": cart_weight,
            "job": job.map(|j| json!({"weight": j.weight, "hp_coef": j.hp_coef, "sp_coef": j.sp_coef,
                                       "aspd_base": j.aspd_base, "job_bonus": j.job_bonus})).unwrap_or(Value::Null),
            "equip": self.equip_summary(c),
            "skills": c.skills.iter().filter(|(_, lv)| **lv > 0).map(|(k, v)| (k.to_string(), json!(v))).collect::<serde_json::Map<_, _>>(),
            "sc": c.sc.iter().map(|(k, v)| (k.to_string(), json!({"val1": v.val1, "val2": v.val2, "val3": v.val3, "val4": v.val4}))).collect::<serde_json::Map<_, _>>(),
        })
    }

    pub(crate) fn derive(&self, c: &Character) -> Derived {
        if let Some(engine) = &self.scripts {
            // status_view: the fields the server keeps, out of rules.status's full table
            match engine.rule("status_view", &self.char_lua(c)) {
                Ok(Some(v)) => match Derived::from_lua(&v) {
                    Some(d) => return d,
                    None => self.soft_notice(format!("rules.status must return a table, got {v}")),
                },
                Ok(None) => {}
                Err(e) => self.soft_notice(format!("{e:#}")),
            }
        }
        fallback_derive(c)
    }

    /// Note a rule error from a read-only path (shown once, on the next tick).
    pub(crate) fn soft_notice(&self, msg: String) {
        let mut n = self.soft_notices.borrow_mut();
        if !n.contains(&msg) && n.len() < 20 {
            n.push(msg);
        }
    }

    pub(crate) fn char_view(&self, c: &Character) -> CharView {
        let mut v = CharView::default();
        for it in c.inventory.iter().filter(|i| i.equip != 0) {
            let Some(d) = self.db.item(it.nameid) else { continue };
            let view = d.view as u16;
            if it.equip & 0x02 != 0 {
                v.weapon = view;
            }
            if it.equip & 0x20 != 0 && d.kind == 5 {
                v.shield = view;
            }
            if it.equip & 0x100 != 0 {
                v.head_top = view;
            }
            if it.equip & 0x200 != 0 {
                v.head_mid = view;
            }
            if it.equip & 0x01 != 0 {
                v.head_bottom = view;
            }
        }
        v
    }

    pub(crate) fn weight_of(&self, c: &Character) -> u32 {
        c.inventory.iter().map(|i| self.db.item(i.nameid).map(|d| d.weight).unwrap_or(0) * i.amount).sum()
    }

    /// EXP needed for the next base / job level (0 = max level).
    pub(crate) fn exp_next(&self, c: &Character) -> (u64, u64) {
        let base = self.db.exp.base.get(c.base_level as usize - 1).copied().unwrap_or(0);
        let table = match c.class {
            0 => &self.db.exp.job_novice,
            1..=6 => &self.db.exp.job_first,
            _ => &self.db.exp.job_second,
        };
        let job = table.get(c.job_level as usize - 1).copied().unwrap_or(0);
        (base, job)
    }

    pub(crate) fn char_of_unit(&self, unit: u32) -> Option<&Character> {
        let cid = self.world.units.get(&unit)?.player()?.char_id;
        self.store.chars.get(&cid)
    }
    pub(crate) fn char_of_unit_mut(&mut self, unit: u32) -> Option<&mut Character> {
        let cid = self.world.units.get(&unit)?.player()?.char_id;
        self.store.chars.get_mut(&cid)
    }

    /// Everything the status window shows.
    pub(crate) fn send_full_status(&mut self, unit: u32) {
        let Some(sid) = self.sid_of(unit) else { return };
        let Some(c) = self.char_of_unit(unit).cloned() else { return };
        let d = self.derive(&c);
        let (nb, nj) = self.exp_next(&c);
        let speed = d.speed;
        let costs: Vec<u8> = c.stats.iter().map(|&s| stat_cost(s)).collect();
        let fs = FullStatus {
            status_point: c.status_point,
            stats: c.stats.map(|s| s.min(255) as u8),
            costs: costs.clone().try_into().unwrap(),
            atk: d.atk as u16,
            atk2: d.atk2 as u16,
            matk_max: d.matk_max as u16,
            matk_min: d.matk_min as u16,
            def: d.def as u16,
            def2: d.def2 as u16,
            mdef: d.mdef as u16,
            mdef2: d.mdef2 as u16,
            hit: d.hit as u16,
            flee: d.flee as u16,
            flee2: d.flee2 as u16,
            crit: (d.crit / 10) as u16,
            aspd: d.amotion as u16,
        };
        let w = self.weight_of(&c);
        let mut out = vec![
            packets::full_status(&fs),
            packets::par(SP_SPEED, speed),
            packets::par(SP_HP, c.hp),
            packets::par(SP_MAXHP, d.max_hp),
            packets::par(SP_SP, c.sp),
            packets::par(SP_MAXSP, d.max_sp),
            packets::par(SP_BASELEVEL, c.base_level as u32),
            packets::par(SP_JOBLEVEL, c.job_level as u32),
            packets::par(SP_STATUSPOINT, c.status_point as u32),
            packets::par(SP_SKILLPOINT, c.skill_point as u32),
            packets::par(SP_WEIGHT, w),
            packets::par(SP_MAXWEIGHT, d.max_weight),
            packets::par(SP_ASPD, d.amotion),
            packets::long_par(SP_BASEEXP, c.base_exp.min(u32::MAX as u64) as u32),
            packets::long_par(SP_JOBEXP, c.job_exp.min(u32::MAX as u64) as u32),
            packets::long_par(SP_NEXTBASEEXP, nb.min(u32::MAX as u64) as u32),
            packets::long_par(SP_NEXTJOBEXP, nj.min(u32::MAX as u64) as u32),
            packets::long_par(SP_ZENY, c.zeny),
        ];
        for i in 0..6u16 {
            out.push(packets::stat_par((SP_STR + i) as u32, c.stats[i as usize] as u32, d.bonus[i as usize] as u32));
            out.push(packets::stat_cost(SP_USTR + i, costs[i as usize]));
        }
        for p in out {
            self.send(sid, p);
        }
    }

    pub(crate) fn send_hp_sp(&mut self, unit: u32) {
        let Some(sid) = self.sid_of(unit) else { return };
        let Some(c) = self.char_of_unit(unit).cloned() else { return };
        let d = self.derive(&c);
        self.send(sid, packets::par(SP_HP, c.hp));
        self.send(sid, packets::par(SP_MAXHP, d.max_hp));
        self.send(sid, packets::par(SP_SP, c.sp));
        self.send(sid, packets::par(SP_MAXSP, d.max_sp));
    }

    pub(crate) fn raise_stat(&mut self, unit: u32, kind: u16) {
        let Some(sid) = self.sid_of(unit) else { return };
        if !(SP_STR..SP_STR + 6).contains(&kind) {
            return;
        }
        let i = (kind - SP_STR) as usize;
        let Some(c) = self.char_of_unit_mut(unit) else { return };
        let cost = stat_cost(c.stats[i]) as u16;
        let ok = c.stats[i] < 99 && c.status_point >= cost;
        if ok {
            c.status_point -= cost;
            c.stats[i] += 1;
        }
        let v = c.stats[i] as u8;
        self.send(sid, packets::stat_up_ack(kind, ok, v));
        if ok {
            self.send_full_status(unit);
        }
    }

    /// Add EXP (already rate-adjusted); handles level ups (mod0631 pc.c
    /// pc_gainexp, pc_checkbaselevelup, pc_checkjoblevelup): a base level
    /// gives (level + 14) / 5 status points (22 at 99) and heals fully, a job
    /// level one skill point. The dead gain nothing.
    pub(crate) fn gain_exp(&mut self, unit: u32, base: u64, job: u64) {
        if self.sid_of(unit).is_none() || self.world.units.get(&unit).is_none_or(|u| u.state == 1) {
            return;
        }
        let Some(c) = self.char_of_unit_mut(unit) else { return };
        c.base_exp += base;
        c.job_exp += job;
        let (mut base_ups, mut job_ups) = (0, 0);
        loop {
            let Some(c) = self.char_of_unit(unit) else { return };
            let (nb, _) = self.exp_next(c);
            if nb == 0 || c.base_exp < nb || c.base_level >= 99 {
                break;
            }
            let c = self.char_of_unit_mut(unit).unwrap();
            c.base_exp -= nb;
            c.base_level += 1;
            c.status_point += if c.base_level >= 99 { 22 } else { (c.base_level + 14) / 5 };
            base_ups += 1;
        }
        loop {
            let Some(c) = self.char_of_unit(unit) else { return };
            let (_, nj) = self.exp_next(c);
            if nj == 0 || c.job_exp < nj {
                break;
            }
            let c = self.char_of_unit_mut(unit).unwrap();
            c.job_exp -= nj;
            c.job_level += 1;
            c.skill_point += 1;
            job_ups += 1;
        }
        if base_ups > 0 {
            // pc_heal(sd, max_hp, max_sp) with the new level's maxima
            let c = self.char_of_unit(unit).cloned().unwrap();
            let d = self.derive(&c);
            if let Some(cm) = self.char_of_unit_mut(unit) {
                cm.hp = d.max_hp;
                cm.sp = d.max_sp;
            }
        }
        for _ in 0..base_ups {
            self.area_send(unit, packets::misc_effect(unit, 0), true);
        }
        for _ in 0..job_ups {
            self.area_send(unit, packets::misc_effect(unit, 1), true);
        }
        self.send_full_status(unit);
        if job_ups > 0 {
            // the new point can be spent: the skill window's arrows
            self.send_skill_list(unit);
        }
    }
}

pub(crate) fn fallback_derive(c: &Character) -> Derived {
    let [s, a, v, i, d, l] = c.stats.map(|x| x as u32);
    let bl = c.base_level as u32;
    Derived {
        max_hp: ((35 + bl * 5) * (100 + v) / 100).max(1),
        max_sp: (10 * (100 + i) / 100).max(1),
        atk: s + (s / 10) * (s / 10) + d / 5 + l / 5,
        matk_max: i + (i / 5) * (i / 5),
        matk_min: i + (i / 7) * (i / 7),
        def2: v,
        mdef2: i,
        hit: d + bl,
        flee: a + bl,
        flee2: l / 10 + 1,
        crit: l * 3 / 10 + 1,
        amotion: (500 - (a * 4 + d) * 500 / 1000).max(100),
        max_weight: 20000 + s * 300,
        range: 1,
        speed: 150,
        dmotion: 400,
        ..Default::default()
    }
}
