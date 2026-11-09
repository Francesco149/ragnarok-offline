//! Persistent game state: accounts and characters, saved as JSON.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Account {
    pub id: u32,
    pub userid: String,
    /// Plain text: this is a single-player server on your own machine.
    pub password: String,
    /// 0 female, 1 male (the client picks sprites from this).
    pub sex: u8,
    pub gm_level: u8,
    /// Character ids by slot (0..=8).
    pub slots: BTreeMap<u8, u32>,
    /// Kafra storage, shared by the account's characters.
    pub storage: Vec<InvItem>,
    /// Persistent per-account script variables.
    pub vars: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Point {
    pub map: String,
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(map: &str, x: u16, y: u16) -> Self {
        Point { map: map.to_string(), x, y }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(default)]
pub struct InvItem {
    pub nameid: u32,
    pub amount: u32,
    /// Equip location mask the item is worn in (0 = not worn).
    pub equip: u32,
    pub identify: bool,
    pub refine: u8,
    pub attribute: u8,
    pub cards: [u16; 4],
}

impl InvItem {
    pub fn new(nameid: u32, amount: u32) -> Self {
        InvItem { nameid, amount, identify: true, ..Default::default() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Character {
    pub id: u32,
    pub account_id: u32,
    pub slot: u8,
    pub name: String,
    pub class: u16,
    pub base_level: u16,
    pub job_level: u16,
    pub base_exp: u64,
    pub job_exp: u64,
    pub zeny: u32,
    /// str, agi, vit, int, dex, luk
    pub stats: [u16; 6],
    pub status_point: u16,
    pub skill_point: u16,
    pub hp: u32,
    pub sp: u32,
    pub hair: u16,
    pub hair_color: u16,
    pub clothes_color: u16,
    pub option: u32,
    pub karma: u32,
    pub manner: i32,
    pub last_point: Point,
    pub save_point: Point,
    pub inventory: Vec<InvItem>,
    /// Pushcart contents (kept when the cart is taken off).
    pub cart: Vec<InvItem>,
    pub skills: BTreeMap<u16, u8>,
    /// Persistent script variables (quest progress etc.), set from Lua.
    pub vars: BTreeMap<String, serde_json::Value>,
    /// Pets this character owns, by pet id (in eggs or out).
    pub pets: BTreeMap<u32, Pet>,
    /// The pet out of its egg (0 = none).
    pub active_pet: u32,
}

/// A tamed monster (Athena s_pet).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(default)]
pub struct Pet {
    pub id: u32,
    pub mob: u32,
    pub name: String,
    pub level: u16,
    /// Accessory item id worn (0 = none).
    pub equip: u32,
    /// 0..=1000
    pub intimate: i32,
    /// 0..=100
    pub hungry: i32,
    pub renamed: bool,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            id: 0,
            account_id: 0,
            slot: 0,
            name: String::new(),
            class: 0,
            base_level: 1,
            job_level: 1,
            base_exp: 0,
            job_exp: 0,
            zeny: 0,
            stats: [1; 6],
            status_point: 0,
            skill_point: 0,
            hp: 40,
            sp: 11,
            hair: 1,
            hair_color: 0,
            clothes_color: 0,
            option: 0,
            karma: 0,
            manner: 0,
            last_point: Point::new("prontera", 156, 191),
            save_point: Point::new("prontera", 156, 191),
            inventory: Vec::new(),
            cart: Vec::new(),
            skills: BTreeMap::new(),
            vars: BTreeMap::new(),
            pets: BTreeMap::new(),
            active_pet: 0,
        }
    }
}

pub const STR: usize = 0;
pub const AGI: usize = 1;
pub const VIT: usize = 2;
pub const INT: usize = 3;
pub const DEX: usize = 4;
pub const LUK: usize = 5;
