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

/// An item list kept as Athena keeps its arrays (inventory, cart, storage):
/// an item keeps its index while it is held, a slot whose item id is 0 is
/// empty, and a new item takes the first empty slot. The client is told of
/// each change by index and never sees the list closed up (mod0631
/// pc_delitem clears the slot; the indices are reassigned only when the
/// character is loaded again).
pub trait Slots {
    /// The item in slot `i`, if the slot holds one.
    fn slot(&self, i: usize) -> Option<&InvItem>;
    fn slot_mut(&mut self, i: usize) -> Option<&mut InvItem>;
    /// The held items with their slots.
    fn held(&self) -> Box<dyn Iterator<Item = (usize, &InvItem)> + '_>;
    /// How many slots hold an item.
    fn count_held(&self) -> usize;
    /// Put an item in the first empty slot (or a new one); returns the slot.
    fn put(&mut self, it: InvItem) -> usize;
    /// Empty slot `i` (trailing empty slots are dropped).
    fn clear_slot(&mut self, i: usize);
    /// Close up the empty slots (on loading a character).
    fn compact(&mut self);
}

impl Slots for Vec<InvItem> {
    fn slot(&self, i: usize) -> Option<&InvItem> {
        self.get(i).filter(|it| it.nameid != 0)
    }
    fn slot_mut(&mut self, i: usize) -> Option<&mut InvItem> {
        self.get_mut(i).filter(|it| it.nameid != 0)
    }
    fn held(&self) -> Box<dyn Iterator<Item = (usize, &InvItem)> + '_> {
        Box::new(self.iter().enumerate().filter(|(_, it)| it.nameid != 0))
    }
    fn count_held(&self) -> usize {
        self.iter().filter(|it| it.nameid != 0).count()
    }
    fn put(&mut self, it: InvItem) -> usize {
        match self.iter().position(|s| s.nameid == 0) {
            Some(i) => {
                self[i] = it;
                i
            }
            None => {
                self.push(it);
                self.len() - 1
            }
        }
    }
    fn clear_slot(&mut self, i: usize) {
        if let Some(s) = self.get_mut(i) {
            *s = InvItem::default();
        }
        while self.last().is_some_and(|s| s.nameid == 0) {
            self.pop();
        }
    }
    fn compact(&mut self) {
        self.retain(|it| it.nameid != 0 && it.amount != 0);
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
    /// Warp Portal destinations saved with /memo (Athena memo_point, up to 3).
    pub memo: Vec<Point>,
    /// Status changes in effect, by Athena SC type (not saved: lost on logout, as in Athena).
    #[serde(skip)]
    pub sc: BTreeMap<u16, StatusChange>,
}

/// One status change (Athena sc_data): until = server tick (ms).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StatusChange {
    pub until: u64,
    pub val1: i64,
    pub val2: i64,
    pub val3: i64,
    pub val4: i64,
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
            memo: Vec::new(),
            sc: BTreeMap::new(),
        }
    }
}

pub const STR: usize = 0;
pub const AGI: usize = 1;
pub const VIT: usize = 2;
pub const INT: usize = 3;
pub const DEX: usize = 4;
pub const LUK: usize = 5;
