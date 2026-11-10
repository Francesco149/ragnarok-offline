//! Units on maps: players, NPCs, monsters, floor items.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::Arc;

use ro_data::MapGrid;

use crate::model::InvItem;
use crate::packets::{CharView, UnitLook};

/// Cells a player sees around itself (Athena `area_size` default 14).
pub const AREA: i32 = 14;

/// Default walk speed: ms per cell (client & Athena DEFAULT_WALK_SPEED).
pub const DEFAULT_SPEED: u16 = 150;

pub const NPC_ID_START: u32 = 110_000_000; // Athena npc ids
pub const OBJ_ID_START: u32 = 1_000; // mobs and floor items (below player ids)

#[derive(Debug, Clone)]
pub struct Walk {
    pub path: Vec<u8>,
    pub pos: usize,
    /// When the unit enters the next cell (ms tick).
    pub next_at: u64,
    pub to: (u16, u16),
    /// New destination requested mid-walk; applied at the next cell.
    pub retarget: Option<(u16, u16)>,
}

#[derive(Debug, Clone)]
pub struct PlayerUnit {
    pub sid: u32,
    pub char_id: u32,
    pub account_id: u32,
    pub sex: u8,
    pub view: CharView,
    pub visible: HashSet<u32>,
    /// Client finished loading the map (0x007d received).
    pub loaded: bool,
    pub attack: Option<AttackIntent>,
    pub next_attack_at: u64,
    pub last_regen: u64,
    /// Natural regen clocks (Athena hp_sub / sp_sub), ms.
    pub hp_acc: u64,
    pub sp_acc: u64,
    /// Shop NPC the player has open (for buy/sell packets).
    pub shop: Option<u32>,
    /// Kafra storage window is open.
    pub storage_open: bool,
    /// Monster class a taming item was used for (waiting for the client's target).
    pub catch_class: Option<u32>,
    /// This player's pet unit, if out.
    pub pet_unit: Option<u32>,
    pub gm_level: u8,
    /// Skill being cast (Athena skilltimer).
    pub cast: Option<crate::battle::Cast>,
    /// No skill or attack before this tick (Athena canact_tick).
    pub canact: u64,
    /// No walking before this tick (Athena canmove_tick).
    pub canmove: u64,
    /// Skill granted by an item being used (Athena skillitem): (skill, level).
    pub skill_item: Option<(u16, u16)>,
    /// Skill waiting for a map choice (Teleport, Warp Portal): (skill, level, x, y).
    pub skill_menu: Option<(u16, u16, u16, u16)>,
    /// Natural-recovery skill clocks (Athena inchealhptick / inchealsptick / inchealspirittick).
    pub inc_hp_acc: u64,
    pub inc_sp_acc: u64,
    pub spirit_acc: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct AttackIntent {
    pub target: u32,
    pub continuous: bool,
}

#[derive(Debug, Clone)]
pub enum NpcKind {
    Script { def: usize },
    Warp { to: (String, u16, u16) },
    Shop { def: usize },
}

#[derive(Debug, Clone)]
pub struct NpcUnit {
    pub kind: NpcKind,
    pub touch: Option<(u16, u16)>,
    /// Not shown to clients (Athena sprite -1: touch areas, event NPCs).
    pub hidden: bool,
}

#[derive(Debug, Clone)]
pub struct MobUnit {
    pub mob_id: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub spawn: Option<usize>,
    pub target: Option<u32>,
    pub next_think: u64,
    pub next_attack_at: u64,
    /// Damage taken per player unit (EXP share).
    pub damage_log: HashMap<u32, u32>,
    /// Script event run when it dies ("Npc::OnMyMobDead"); script-spawned mobs only.
    pub event: Option<String>,
    /// Status changes in effect (Athena sc_data).
    pub sc: std::collections::BTreeMap<u16, crate::model::StatusChange>,
    /// Defence element when a skill changed it (element type + level*20).
    pub def_ele: Option<u16>,
    pub cast: Option<crate::battle::Cast>,
    pub canact: u64,
    /// Summoner (Athena master_id), for slaves.
    pub master: Option<u32>,
    /// Who hit it last (Athena attacked_id): the AI turns on them.
    pub attacked: Option<u32>,
    /// The target is a floor item (looters), not something to attack.
    pub looting: bool,
    /// Give up the chase beyond this many cells (Athena min_chase).
    pub min_chase: i32,
    /// No random walk before this tick (Athena next_walktime).
    pub next_walk: u64,
    pub move_fail: u32,
    /// What it is doing, for skill conditions (Athena skillstate).
    pub skill_state: MobState,
    /// When each line of its skill list was last used.
    pub skill_used: Vec<u64>,
    /// Items a looter picked up (dropped again when it dies).
    pub loot: Vec<InvItem>,
    /// Distance to its master at the last check (Athena master_dist).
    pub master_dist: i32,
}

/// Athena MSS_*: the state monster skills are conditioned on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MobState {
    #[default]
    Idle,
    Walk,
    Attack,
    Dead,
    Loot,
    Chase,
}

impl MobState {
    pub fn name(self) -> &'static str {
        match self {
            MobState::Idle => "idle",
            MobState::Walk => "walk",
            MobState::Attack => "attack",
            MobState::Dead => "dead",
            MobState::Loot => "loot",
            MobState::Chase => "chase",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FloorItemUnit {
    pub item: InvItem,
    pub expire_at: u64,
    pub subx: u8,
    pub suby: u8,
}

/// A pet out of its egg, following its owner.
#[derive(Debug, Clone)]
pub struct PetUnit {
    pub owner: u32,
    pub pet_id: u32,
    /// Accessory item id (drawn as the head-bottom slot).
    pub equip: u32,
    /// Next hunger tick (ms).
    pub next_hunger: u64,
}

#[derive(Debug, Clone)]
pub enum UnitKind {
    /// One cell of a ground skill (Athena skill_unit).
    Skill(crate::battle::SkillCell),
    Player(Box<PlayerUnit>),
    Pet(PetUnit),
    Npc(NpcUnit),
    Mob(Box<MobUnit>),
    Item(FloorItemUnit),
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: u32,
    pub map: String,
    pub x: u16,
    pub y: u16,
    pub dir: u8,
    pub head_dir: u8,
    pub speed: u16,
    pub class: u16,
    pub name: String,
    /// 0 standing, 1 dead, 2 sitting
    pub state: u8,
    pub walk: Option<Walk>,
    pub kind: UnitKind,
}

impl Unit {
    pub fn player(&self) -> Option<&PlayerUnit> {
        match &self.kind {
            UnitKind::Player(p) => Some(p),
            _ => None,
        }
    }
    pub fn player_mut(&mut self) -> Option<&mut PlayerUnit> {
        match &mut self.kind {
            UnitKind::Player(p) => Some(p),
            _ => None,
        }
    }
    pub fn mob(&self) -> Option<&MobUnit> {
        match &self.kind {
            UnitKind::Mob(m) => Some(m),
            _ => None,
        }
    }
    pub fn mob_mut(&mut self) -> Option<&mut MobUnit> {
        match &mut self.kind {
            UnitKind::Mob(m) => Some(m),
            _ => None,
        }
    }
    pub fn is_player(&self) -> bool {
        matches!(self.kind, UnitKind::Player(_))
    }
    pub fn is_item(&self) -> bool {
        matches!(self.kind, UnitKind::Item(_))
    }
    pub fn skill(&self) -> Option<&crate::battle::SkillCell> {
        match &self.kind {
            UnitKind::Skill(s) => Some(s),
            _ => None,
        }
    }
    pub fn skill_mut(&mut self) -> Option<&mut crate::battle::SkillCell> {
        match &mut self.kind {
            UnitKind::Skill(s) => Some(s),
            _ => None,
        }
    }
    /// Players and monsters: what skills and attacks can hit.
    pub fn is_fighter(&self) -> bool {
        matches!(self.kind, UnitKind::Player(_) | UnitKind::Mob(_))
    }
    /// Look for 0x0078/0x0079/0x007b.
    pub fn look(&self, extra: Option<(&crate::model::Character, CharView)>) -> UnitLook {
        let mut l = UnitLook {
            id: self.id,
            speed: self.speed,
            class: self.class,
            head_dir: self.head_dir as u16,
            x: self.x,
            y: self.y,
            dir: self.dir,
            state: self.state,
            ..Default::default()
        };
        if let UnitKind::Pet(p) = &self.kind {
            // Athena clif_pet0078: hair 0x14 tells the client it is a pet
            l.hair = 0x14;
            l.head_bottom = p.equip as u16;
        }
        if let (UnitKind::Player(p), Some((c, view))) = (&self.kind, extra) {
            l.sex = p.sex;
            l.hair = c.hair;
            l.hair_color = c.hair_color;
            l.clothes_color = c.clothes_color;
            l.weapon = view.weapon;
            l.shield = view.shield;
            l.head_top = view.head_top;
            l.head_mid = view.head_mid;
            l.head_bottom = view.head_bottom;
            l.option = c.option as u16;
            l.karma = c.karma as u8;
            l.manner = c.manner as u16;
        }
        l
    }
}

pub struct MapInst {
    pub grid: Arc<MapGrid>,
    pub units: BTreeSet<u32>,
    pub players: usize,
    /// Monster spawns have been instantiated (lazily, on first visit).
    pub populated: bool,
}

impl Unit {
    /// Clients are told about this unit (hidden script NPCs are server-side only).
    pub fn shown(&self) -> bool {
        !matches!(&self.kind, UnitKind::Npc(NpcUnit { hidden: true, .. }))
    }
}

pub fn in_area(a: &Unit, b: &Unit) -> bool {
    a.map == b.map && (a.x as i32 - b.x as i32).abs() <= AREA && (a.y as i32 - b.y as i32).abs() <= AREA
}

#[derive(Default)]
pub struct World {
    pub maps: HashMap<String, MapInst>,
    pub units: HashMap<u32, Unit>,
    next_npc: u32,
    next_obj: u32,
}

impl World {
    pub fn new() -> Self {
        World { next_npc: NPC_ID_START, next_obj: OBJ_ID_START, ..Default::default() }
    }
    pub fn new_npc_id(&mut self) -> u32 {
        self.next_npc += 1;
        self.next_npc
    }
    pub fn new_obj_id(&mut self) -> u32 {
        self.next_obj += 1;
        if self.next_obj >= crate::store::FIRST_CHAR_ID {
            self.next_obj = OBJ_ID_START + 1;
        }
        while self.units.contains_key(&self.next_obj) {
            self.next_obj += 1;
        }
        self.next_obj
    }

    pub fn add(&mut self, u: Unit) {
        if let Some(m) = self.maps.get_mut(&u.map) {
            m.units.insert(u.id);
            if u.is_player() {
                m.players += 1;
            }
        }
        self.units.insert(u.id, u);
    }

    pub fn remove(&mut self, id: u32) -> Option<Unit> {
        let u = self.units.remove(&id)?;
        if let Some(m) = self.maps.get_mut(&u.map) {
            m.units.remove(&id);
            if u.is_player() {
                m.players = m.players.saturating_sub(1);
            }
        }
        Some(u)
    }

    /// Ids of units on `map` within the view area of (x, y).
    pub fn area_ids(&self, map: &str, x: u16, y: u16) -> Vec<u32> {
        let Some(m) = self.maps.get(map) else { return vec![] };
        m.units
            .iter()
            .copied()
            .filter(|id| {
                self.units.get(id).is_some_and(|u| {
                    (u.x as i32 - x as i32).abs() <= AREA && (u.y as i32 - y as i32).abs() <= AREA
                })
            })
            .collect()
    }

    /// Player units on `map` that can see (x, y).
    pub fn viewers(&self, map: &str, x: u16, y: u16) -> Vec<u32> {
        let Some(m) = self.maps.get(map) else { return vec![] };
        if m.players == 0 {
            return vec![];
        }
        m.units
            .iter()
            .copied()
            .filter(|id| {
                self.units.get(id).is_some_and(|u| {
                    u.is_player()
                        && (u.x as i32 - x as i32).abs() <= AREA
                        && (u.y as i32 - y as i32).abs() <= AREA
                })
            })
            .collect()
    }

    pub fn players_on(&self, map: &str) -> Vec<u32> {
        let Some(m) = self.maps.get(map) else { return vec![] };
        m.units.iter().copied().filter(|id| self.units.get(id).is_some_and(|u| u.is_player())).collect()
    }

    pub fn all_players(&self) -> Vec<u32> {
        self.units.values().filter(|u| u.is_player()).map(|u| u.id).collect()
    }

    pub fn npc_by_name(&self, name: &str) -> Option<u32> {
        self.units
            .values()
            .find(|u| matches!(u.kind, UnitKind::Npc(_)) && u.name.eq_ignore_ascii_case(name))
            .map(|u| u.id)
    }
}
