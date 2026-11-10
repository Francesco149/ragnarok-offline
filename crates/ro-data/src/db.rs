//! Server-side game database: everything the client does NOT contain.
//!
//! Every file has a `[meta]` table with a provenance [`Tier`] and a free-text
//! `source`; records may override the tier (`tier = "..."`) when a value has
//! been checked against something better. See docs/PROVENANCE.md.
//!
//! Files (all optional; missing ones load empty with a warning):
//!   data/db/items.toml  data/db/mobs.toml  data/db/exp.toml  data/db/jobs.toml
//!   data/db/skills.toml  data/db/random.toml  data/db/mapflags.toml  data/db/pets.toml
//!   data/db/mob_skills.toml

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// How much a value can be trusted, best first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Tier {
    /// Read from the client's own files or binary: confirmed.
    Client,
    /// Gravity / publisher material from the period (patch notes, guides).
    Official,
    /// Athena-family emulator data contemporary with the client (2003-12 .. 2004-05).
    Athena2003,
    /// Community measurements and documents written 2002-2004.
    Period,
    /// Later reconstructions (rAthena pre-renewal, wikis written after 2004).
    Later,
    /// Placeholder / our own estimate.
    #[default]
    Guess,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub tier: Tier,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Item {
    pub id: u32,
    /// Server/script name (Aegis-style, e.g. "Red_Potion").
    pub name: String,
    /// Display name for server messages (the client shows its own table).
    pub display: String,
    /// Athena item type: 0 healing, 2 usable, 3 etc, 4 weapon, 5 armor, 6 card,
    /// 7 pet egg, 8 pet equip, 10 ammo, 11 delayed-consume usable.
    #[serde(rename = "type")]
    pub kind: u8,
    pub buy: u32,
    pub sell: Option<u32>,
    /// In tenths, as everywhere in RO (70 = 7.0 weight).
    pub weight: u32,
    pub atk: u32,
    pub def: u32,
    pub range: u32,
    pub slots: u8,
    pub jobs: u32,
    /// 0 female, 1 male, 2 both.
    pub gender: u8,
    /// Equip location bitmask (0 = not equippable).
    pub loc: u32,
    pub weapon_level: u8,
    pub equip_level: u8,
    pub view: u32,
    /// Lua run when used (translated from `athena_use` where possible).
    pub use_script: String,
    /// Original Athena script, kept for reference / later translation.
    pub athena_use: String,
    pub athena_equip: String,
    pub tier: Option<Tier>,
}

impl Item {
    pub fn sell_price(&self) -> u32 {
        self.sell.unwrap_or(self.buy / 2)
    }
    pub fn is_equip(&self) -> bool {
        self.loc != 0 && matches!(self.kind, 4 | 5 | 8 | 10)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Drop {
    pub item: u32,
    /// Chance in 1/10000.
    pub rate: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Mob {
    pub id: u32,
    pub name: String,
    pub display: String,
    pub level: u16,
    pub hp: u32,
    pub sp: u32,
    pub base_exp: u32,
    pub job_exp: u32,
    pub range: u16,
    pub atk1: u32,
    pub atk2: u32,
    pub def: u32,
    pub mdef: u32,
    pub str: u16,
    pub agi: u16,
    pub vit: u16,
    pub int: u16,
    pub dex: u16,
    pub luk: u16,
    pub view_range: u16,
    pub chase_range: u16,
    /// 0 small, 1 medium, 2 large.
    pub size: u8,
    pub race: u8,
    /// element*10 + level, Athena style (21 = water 1... see docs).
    pub element: u8,
    /// Athena mode bitmask (0x01 can move, 0x02 looter, 0x04 aggressive, 0x08 assist,
    /// 0x10 cast-sensor, 0x20 boss, 0x40 plant, 0x80 can attack).
    pub mode: u32,
    /// ms per cell.
    pub speed: u32,
    pub attack_delay: u32,
    pub attack_motion: u32,
    pub damage_motion: u32,
    pub drops: Vec<Drop>,
    pub mvp_exp: u32,
    pub mvp_drops: Vec<Drop>,
    pub tier: Option<Tier>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ExpTables {
    pub meta: Meta,
    /// Base EXP to reach level n+2 from n+1 (index 0 = 1 -> 2).
    pub base: Vec<u64>,
    pub job_novice: Vec<u64>,
    pub job_first: Vec<u64>,
    pub job_second: Vec<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Job {
    pub id: u16,
    pub name: String,
    /// Weight limit base, in tenths (Athena job_db1 col 1).
    pub weight: u32,
    pub hp_coef: f64,
    pub sp_coef: u32,
    /// Base attack delay per weapon type (ms * 2? see docs), Athena job_db1.
    pub aspd_base: Vec<u32>,
    /// Stat bonus gained at each job level: 1 str .. 6 luk, 0 none.
    pub job_bonus: Vec<u8>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Skill {
    pub id: u16,
    /// Aegis name ("SM_BASH"); the client keys its own tables by it.
    pub name: String,
    pub max: u8,
    /// Target type: 0 passive, 1 enemy, 2 place, 4 self, 16 friend.
    pub inf: u16,
    /// Cells; negative = weapon range.
    pub range: i32,
    pub hit: u8,
    pub element: u8,
    pub nk: u8,
    /// SP per level (index 0 = level 1).
    pub sp: Vec<u16>,
    /// Hits per level (empty = 1).
    pub num: Vec<u16>,
    /// Cast time and after-cast delay per level, ms (empty = 0).
    pub cast: Vec<u32>,
    pub delay: Vec<u32>,
    /// How long the skill and its side effect last, ms per level (817's cast_db).
    pub time: Vec<u32>,
    pub time2: Vec<u32>,
    /// Monster skill (Athena inf2 & 2).
    pub npc: bool,
    /// Learnt only from NPC quests, never with skill points.
    pub quest: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TreeEntry {
    pub job: u16,
    pub skill: u16,
    pub max: u8,
    /// (skill, level) prerequisites.
    pub need: Vec<(u16, u8)>,
}

/// One line of a monster's skill list (Athena mob_skill_db).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MobSkill {
    pub mob: u32,
    /// any, idle, walk, attack, dead, loot, chase
    pub state: String,
    pub skill: u16,
    pub lv: u16,
    /// Chance per 1000 each time the monster thinks.
    pub rate: u32,
    pub cast: u32,
    pub delay: u32,
    /// Damage interrupts the cast.
    pub cancel: bool,
    /// target, self, friend
    pub target: String,
    pub cond: String,
    pub cond_val: i32,
    /// Skill argument: monster to summon, emotion.
    pub val: i32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct MobSkillFile {
    #[serde(default)]
    meta: Meta,
    #[serde(default)]
    skill: Vec<MobSkill>,
}

/// A random item source (`getitem -flag`): Old Blue Box, Old Violet Box,
/// Old Card Album, Gift Box, Old Scroll.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ItemBox {
    pub flag: u8,
    pub name: String,
    /// Given when the random pick fails (0 = nothing).
    pub default: u32,
    /// (item id, chance per 1,000,000 of keeping a pick)
    pub items: Vec<(u32, u32)>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Branch {
    /// Monsters a Dead Branch may summon.
    pub mobs: Vec<u32>,
}

/// A tameable monster (Athena pet_db).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PetDef {
    pub mob: u32,
    pub name: String,
    /// Taming item, egg, accessory and food item ids.
    pub item: u32,
    pub egg: u32,
    pub accessory: u32,
    pub food: u32,
    /// Hunger points (of 100) one feeding restores.
    pub fullness: i32,
    /// Seconds per hunger point lost.
    pub hungry_delay: u32,
    pub r_hungry: i32,
    pub r_full: i32,
    /// Intimacy (of 1000) when caught.
    pub intimate: i32,
    pub die: i32,
    /// Base catch chance in 1/100 percent.
    pub capture: i32,
    pub speed: u16,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct PetFile {
    #[serde(default)]
    meta: Meta,
    #[serde(default)]
    pet: Vec<PetDef>,
}

/// Per-map rules (Athena map flags).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MapFlags {
    pub name: String,
    /// noteleport, nobranch, nopenalty, nosave, nomemo, pvp
    pub flags: Vec<String>,
    /// For nosave: "map,x,y", or "SavePoint".
    pub nosave: Option<String>,
}

impl MapFlags {
    pub fn has(&self, flag: &str) -> bool {
        self.flags.iter().any(|f| f == flag)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct MapFlagFile {
    #[serde(default)]
    meta: Meta,
    #[serde(default)]
    map: Vec<MapFlags>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct RandomFile {
    #[serde(default)]
    meta: Meta,
    #[serde(default)]
    r#box: Vec<ItemBox>,
    #[serde(default)]
    branch: Branch,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct SkillFile {
    #[serde(default)]
    meta: Meta,
    #[serde(default)]
    skill: Vec<Skill>,
    #[serde(default)]
    tree: Vec<TreeEntry>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct ItemFile {
    #[serde(default)]
    meta: Meta,
    #[serde(default)]
    item: Vec<Item>,
}
#[derive(Debug, Default, Serialize, Deserialize)]
struct MobFile {
    #[serde(default)]
    meta: Meta,
    #[serde(default)]
    mob: Vec<Mob>,
}
#[derive(Debug, Default, Serialize, Deserialize)]
struct JobFile {
    #[serde(default)]
    meta: Meta,
    #[serde(default)]
    job: Vec<Job>,
}

#[derive(Default)]
pub struct Database {
    pub items: HashMap<u32, Item>,
    pub items_by_name: HashMap<String, u32>,
    pub items_meta: Meta,
    pub mobs: HashMap<u32, Mob>,
    pub mobs_by_name: HashMap<String, u32>,
    pub mobs_meta: Meta,
    pub exp: ExpTables,
    pub jobs: HashMap<u16, Job>,
    pub jobs_meta: Meta,
    pub skills: HashMap<u16, Skill>,
    pub skills_by_name: HashMap<String, u16>,
    /// Skill tree per job, in file order.
    pub skill_tree: HashMap<u16, Vec<TreeEntry>>,
    pub skills_meta: Meta,
    pub boxes: Vec<ItemBox>,
    pub branch: Branch,
    pub mapflags: HashMap<String, MapFlags>,
    pub pets: Vec<PetDef>,
    /// Monster skill lists by monster id, in file order.
    pub mob_skills: HashMap<u32, Vec<MobSkill>>,
    pub warnings: Vec<String>,
}

fn read_toml<T: for<'de> Deserialize<'de> + Default>(path: &Path, warnings: &mut Vec<String>) -> Result<T> {
    match std::fs::read_to_string(path) {
        Ok(s) => toml::from_str(&s).with_context(|| format!("parsing {}", path.display())),
        Err(_) => {
            warnings.push(format!("{} missing; using empty table", path.display()));
            Ok(T::default())
        }
    }
}

impl Database {
    pub fn load(dir: &Path) -> Result<Self> {
        let mut db = Database::default();
        let items: ItemFile = read_toml(&dir.join("items.toml"), &mut db.warnings)?;
        db.items_meta = items.meta;
        for it in items.item {
            db.items_by_name.insert(it.name.to_ascii_lowercase(), it.id);
            db.items.insert(it.id, it);
        }
        let mobs: MobFile = read_toml(&dir.join("mobs.toml"), &mut db.warnings)?;
        db.mobs_meta = mobs.meta;
        for m in mobs.mob {
            db.mobs_by_name.insert(m.name.to_ascii_lowercase(), m.id);
            db.mobs.insert(m.id, m);
        }
        db.exp = read_toml(&dir.join("exp.toml"), &mut db.warnings)?;
        let jobs: JobFile = read_toml(&dir.join("jobs.toml"), &mut db.warnings)?;
        db.jobs_meta = jobs.meta;
        for j in jobs.job {
            db.jobs.insert(j.id, j);
        }
        let skills: SkillFile = read_toml(&dir.join("skills.toml"), &mut db.warnings)?;
        db.skills_meta = skills.meta;
        for sk in skills.skill {
            db.skills_by_name.insert(sk.name.to_ascii_uppercase(), sk.id);
            db.skills.insert(sk.id, sk);
        }
        for t in skills.tree {
            db.skill_tree.entry(t.job).or_default().push(t);
        }
        let random: RandomFile = read_toml(&dir.join("random.toml"), &mut db.warnings)?;
        db.boxes = random.r#box;
        db.branch = random.branch;
        let flags: MapFlagFile = read_toml(&dir.join("mapflags.toml"), &mut db.warnings)?;
        db.mapflags = flags.map.into_iter().map(|m| (m.name.clone(), m)).collect();
        db.pets = read_toml::<PetFile>(&dir.join("pets.toml"), &mut db.warnings)?.pet;
        for ms in read_toml::<MobSkillFile>(&dir.join("mob_skills.toml"), &mut db.warnings)?.skill {
            db.mob_skills.entry(ms.mob).or_default().push(ms);
        }
        Ok(db)
    }

    pub fn pet_by_mob(&self, mob: u32) -> Option<&PetDef> {
        self.pets.iter().find(|p| p.mob == mob)
    }

    /// Does `map` have Athena map flag `flag`?
    pub fn map_flag(&self, map: &str, flag: &str) -> bool {
        self.mapflags.get(map).is_some_and(|m| m.has(flag))
    }

    pub fn skill(&self, id: u16) -> Option<&Skill> {
        self.skills.get(&id)
    }

    pub fn item(&self, id: u32) -> Option<&Item> {
        self.items.get(&id)
    }
    pub fn item_by_name(&self, name: &str) -> Option<&Item> {
        self.items_by_name.get(&name.to_ascii_lowercase()).and_then(|id| self.items.get(id))
    }
    pub fn mob(&self, id: u32) -> Option<&Mob> {
        self.mobs.get(&id)
    }
    pub fn mob_by_name(&self, name: &str) -> Option<&Mob> {
        self.mobs_by_name.get(&name.to_ascii_lowercase()).and_then(|id| self.mobs.get(id))
    }
}
