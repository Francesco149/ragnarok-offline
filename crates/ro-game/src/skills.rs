//! Skills, the part NPC scripts and job changes need: the skill tree, skill
//! points, the skill window (0x010f / 0x010e / 0x0112), quest skills given by
//! NPCs, skill reset, and Basic Skill gating of sitting and emotes.
//! Using skills is not reconstructed yet (docs/ROADMAP.md).
//!
//! Tree rules follow athena-817 `pc_calc_skilltree` / `pc_skillup` (a skill is
//! available once its prerequisites are met; one point per level). One
//! difference, on purpose: quest skills (skills.toml `quest`) can't be bought
//! with points. 817 allowed it (`quest_skill_learn: yes`); the official
//! servers didn't (docs/DEVIATIONS.md).

use std::collections::HashMap;

use crate::game::Game;
use crate::packets::{self, SkillEntry};

pub const NV_BASIC: u16 = 1;

impl Game {
    /// SP of `skill` at `level` (>= 1): the client's own table first (tier
    /// client), else skills.toml.
    pub(crate) fn skill_sp(&self, skill: u16, level: u8) -> u16 {
        let Some(sk) = self.db.skill(skill) else { return 0 };
        let lv = level.max(1) as usize;
        if let Some(v) = self.res.skill_sp.get(&sk.name).and_then(|t| t.get(lv - 1)) {
            return *v;
        }
        sk.sp.get(lv - 1).copied().unwrap_or(0)
    }

    /// Skill id -> (level, learnable max) the character can see: Athena
    /// pc_calc_skilltree plus every skill it already has (quest skills).
    pub(crate) fn skill_window(&self, char_id: u32) -> Vec<(u16, u8, u8)> {
        let Some(c) = self.store.chars.get(&char_id) else { return vec![] };
        let tree = self.db.skill_tree.get(&c.class).cloned().unwrap_or_default();
        let level = |id: u16| c.skills.get(&id).copied().unwrap_or(0);
        let mut avail: HashMap<u16, u8> = HashMap::new();
        loop {
            let mut changed = false;
            for t in &tree {
                if avail.contains_key(&t.skill) {
                    continue;
                }
                if self.db.skill(t.skill).is_some_and(|s| s.quest) && level(t.skill) == 0 {
                    continue;
                }
                if t.need.iter().all(|&(id, lv)| level(id) >= lv) {
                    avail.insert(t.skill, t.max);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        for (&id, &lv) in &c.skills {
            if lv > 0 {
                let max = self.db.skill(id).map(|s| s.max).unwrap_or(lv);
                avail.entry(id).or_insert(max);
            }
        }
        let mut out: Vec<(u16, u8, u8)> = avail.into_iter().map(|(id, max)| (id, level(id), max)).collect();
        out.sort();
        out
    }

    pub(crate) fn send_skill_list(&mut self, unit: u32) {
        let (Some(sid), Some(cid)) = (self.sid_of(unit), self.world.units.get(&unit).and_then(|u| u.player()).map(|p| p.char_id)) else {
            return;
        };
        let points = self.store.chars.get(&cid).map(|c| c.skill_point).unwrap_or(0);
        let entries: Vec<SkillEntry> = self
            .skill_window(cid)
            .into_iter()
            .filter_map(|(id, lv, max)| {
                let sk = self.db.skill(id)?;
                Some(SkillEntry {
                    id,
                    inf: sk.inf,
                    level: lv as u16,
                    sp: if lv > 0 { self.skill_sp(id, lv) } else { 0 },
                    range: self.skill_range(sk.range),
                    name: sk.name.clone(),
                    upgradable: points > 0 && lv < max && !sk.quest,
                })
            })
            .collect();
        self.send(sid, packets::skill_list(&entries));
    }

    fn skill_range(&self, r: i32) -> u16 {
        // Athena: negative = weapon range - (r + 1); weapon range is 1 for now
        if r < 0 { (1 - (r + 1)).max(1) as u16 } else { r as u16 }
    }

    /// 0x0112: spend one skill point.
    pub(crate) fn skill_up(&mut self, unit: u32, skill: u16) {
        let Some(cid) = self.world.units.get(&unit).and_then(|u| u.player()).map(|p| p.char_id) else { return };
        let Some(&(_, lv, max)) = self.skill_window(cid).iter().find(|(id, _, _)| *id == skill) else { return };
        let quest = self.db.skill(skill).is_none_or(|s| s.quest);
        let Some(c) = self.store.chars.get_mut(&cid) else { return };
        if c.skill_point == 0 || lv >= max || quest {
            return;
        }
        c.skill_point -= 1;
        c.skills.insert(skill, lv + 1);
        let sp = self.skill_sp(skill, lv + 1);
        let range = self.db.skill(skill).map(|s| self.skill_range(s.range)).unwrap_or(0);
        if let Some(sid) = self.sid_of(unit) {
            self.send(sid, packets::skill_update(skill, (lv + 1) as u16, sp, range, lv + 1 < max));
        }
        self.send_full_status(unit);
        self.send_skill_list(unit);
    }

    /// Script `skill(id, level)`: set a skill's level (quest skills). Athena flag 1
    /// (temporary card skills) is treated like 0: there is no card system yet.
    pub(crate) fn give_skill(&mut self, unit: u32, skill: u16, level: u8) -> Result<(), String> {
        if self.db.skill(skill).is_none() {
            return Err(format!("unknown skill {skill}"));
        }
        let c = self.char_of_unit_mut(unit).ok_or("no character")?;
        c.skills.insert(skill, level);
        self.send_skill_list(unit);
        Ok(())
    }

    pub(crate) fn skill_level(&self, unit: u32, skill: u16) -> u8 {
        self.char_of_unit(unit).and_then(|c| c.skills.get(&skill).copied()).unwrap_or(0)
    }

    /// Athena pc_resetskill: points back for every skill but quest skills.
    pub(crate) fn reset_skills(&mut self, unit: u32) {
        let quest: Vec<u16> = self.db.skills.values().filter(|s| s.quest).map(|s| s.id).collect();
        if let Some(c) = self.char_of_unit_mut(unit) {
            let mut back = 0u16;
            c.skills.retain(|id, lv| {
                if quest.contains(id) {
                    true
                } else {
                    back += *lv as u16;
                    false
                }
            });
            c.skill_point += back;
        }
        self.send_full_status(unit);
        self.send_skill_list(unit);
    }

    /// Basic Skill gate (Athena basic_skill_check): 1 trade, 2 emotion, 3 sit,
    /// 4 chat room, 5 party join, 7 party create. Sends the client's failure
    /// message (0x0110 with btype) when too low.
    pub(crate) fn basic_skill_ok(&mut self, unit: u32, need: u8, btype: u16) -> bool {
        if self.skill_level(unit, NV_BASIC) >= need {
            return true;
        }
        if let Some(sid) = self.sid_of(unit) {
            self.send(sid, packets::skill_fail(NV_BASIC, btype, 0));
        }
        false
    }
}
