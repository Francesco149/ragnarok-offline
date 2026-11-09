//! The game: sessions on the three "servers" (login, char, map), auth
//! hand-off between them, timers, script reloads.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use ro_data::{Database, MapGrid};
use ro_grf::Vfs;
use ro_proto::{Reader, ids::*};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::model::{Character, InvItem, Point};
use crate::packets::{self, CharEntry};
use crate::script::{Ctx, ScriptEngine, Wait};
use crate::store::Store;
use crate::world::{MapInst, World};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerKind {
    Login,
    Char,
    Map,
}

pub enum NetEvent {
    Connected { sid: u32, kind: ServerKind },
    Packet { sid: u32, data: Vec<u8> },
    Disconnected { sid: u32 },
}

/// Where outgoing bytes go (implemented by the network layer).
pub trait Outbox {
    fn send(&self, sid: u32, data: Vec<u8>);
    fn close(&self, sid: u32);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GameConfig {
    /// Address the client is told to connect to for char/map servers.
    pub public_ip: [u8; 4],
    pub char_port: u16,
    pub map_port: u16,
    pub server_name: String,
    pub save_dir: PathBuf,
    pub script_dir: PathBuf,
    pub db_dir: PathBuf,
    /// Unknown IDs create an account on first login ("_M"/"_F" suffix picks sex).
    pub auto_register: bool,
    pub default_sex: u8,
    /// GM level for auto-registered accounts (99 = all @commands).
    pub new_account_gm_level: u8,
    pub start_point: Point,
    pub start_zeny: u32,
    /// (item id, amount) given to new characters.
    pub start_items: Vec<(u32, u32)>,
    /// Rates in percent (100 = official 1x).
    pub base_exp_rate: u32,
    pub job_exp_rate: u32,
    pub drop_rate: u32,
    /// Autosave interval, seconds.
    pub autosave_secs: u64,
    /// Content switches scripts test with `content("name")`. Defaults are
    /// euRO's state in Jan-Feb 2004 (docs/DEVIATIONS.md): juno and jobs2_2
    /// opened 2004-06-09; pvp needs other players; events are seasonal.
    pub content: std::collections::BTreeMap<String, bool>,
}

pub fn default_content() -> std::collections::BTreeMap<String, bool> {
    [("juno", false), ("jobs2_2", false), ("pvp", false), ("events", false)]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            public_ip: [127, 0, 0, 1],
            char_port: 6121,
            map_port: 5121,
            server_name: "Offline".into(),
            save_dir: "saves".into(),
            script_dir: "scripts".into(),
            db_dir: "data/db".into(),
            auto_register: true,
            default_sex: 1,
            new_account_gm_level: 99,
            start_point: Point::new("new_zone01", 53, 111),
            start_zeny: 0,
            start_items: vec![(1201, 1), (2301, 1)],
            base_exp_rate: 100,
            job_exp_rate: 100,
            drop_rate: 100,
            autosave_secs: 120,
            content: default_content(),
        }
    }
}

/// Client-derived resources.
pub struct Resources {
    pub vfs: Vfs,
    /// Maps the client has a .gat for (lowercase, no extension).
    pub maps: HashSet<String>,
    /// SP per level by Aegis skill name, from the client's
    /// `data\leveluseskillspamount.txt` (tier client).
    pub skill_sp: HashMap<String, Vec<u16>>,
}

impl Resources {
    pub fn new(vfs: Vfs) -> Self {
        let mut maps = HashSet::new();
        for src in vfs.merged() {
            let name = ro_grf::decode_name(src.name()).to_ascii_lowercase();
            if let Some(stem) = name.strip_prefix("data\\").and_then(|n| n.strip_suffix(".gat")) {
                if !stem.contains('\\') {
                    maps.insert(stem.to_string());
                }
            }
        }
        let skill_sp = vfs.read("data\\leveluseskillspamount.txt").map(|b| parse_skill_sp(&b)).unwrap_or_default();
        Resources { vfs, maps, skill_sp }
    }
}

/// `NAME#\nsp#\nsp#...\n@` blocks (the client's own format).
pub fn parse_skill_sp(data: &[u8]) -> HashMap<String, Vec<u16>> {
    let text = String::from_utf8_lossy(data);
    let mut out = HashMap::new();
    let mut cur: Option<(String, Vec<u16>)> = None;
    for tok in text.split(['#', '\n', '\r']).map(str::trim).filter(|t| !t.is_empty()) {
        if tok == "@" {
            if let Some((name, v)) = cur.take() {
                out.insert(name, v);
            }
        } else if let Ok(n) = tok.parse::<u16>() {
            if let Some((_, v)) = &mut cur {
                v.push(n);
            }
        } else {
            if let Some((name, v)) = cur.take() {
                out.insert(name, v);
            }
            cur = Some((tok.to_ascii_uppercase(), Vec::new()));
        }
    }
    if let Some((name, v)) = cur {
        out.insert(name, v);
    }
    out
}

#[derive(Debug, Clone)]
pub(crate) enum State {
    Login,
    Char { account_id: Option<u32> },
    Map { #[allow(dead_code)] account_id: u32, char_id: u32, unit: u32 },
    Closed,
}

pub(crate) struct Session {
    pub kind: ServerKind,
    pub state: State,
    pub md5_key: Option<Vec<u8>>,
}

pub(crate) struct Dialog {
    pub thread: mlua::Thread,
    pub npc: u32,
    /// NPC definition index (script::NO_DEF if none).
    pub def: u32,
    pub wait: Wait,
    /// A message is showing that still needs a close button.
    pub open: bool,
}

pub struct Game {
    pub cfg: GameConfig,
    pub res: Resources,
    pub db: Database,
    pub store: Store,
    pub(crate) out: Box<dyn Outbox>,
    pub(crate) sessions: HashMap<u32, Session>,
    /// account id -> (login1, login2, sex): login -> char hand-off
    login_tokens: HashMap<u32, (u32, u32, u8)>,
    /// char id -> (account id, login1): char -> map hand-off
    map_tokens: HashMap<u32, (u32, u32)>,
    pub world: World,
    pub(crate) scripts: Option<Rc<ScriptEngine>>,
    pub(crate) dialogs: HashMap<u32, Dialog>,
    /// Per player: an NPC message is showing in the current script step.
    pub(crate) dialog_open: HashMap<u32, bool>,
    pub(crate) respawns: Vec<(u64, usize)>,
    /// NPC runtime state: events, timers, temp variables (npcrt.rs).
    pub(crate) npcrt: crate::npcrt::NpcRuntime,
    start: Instant,
    last_autosave: u64,
    /// Messages for the admin console / logs (script errors etc.).
    pub notices: Vec<String>,
}

impl Game {
    pub fn new(cfg: GameConfig, res: Resources, out: Box<dyn Outbox>) -> Result<Self> {
        let db = Database::load(&cfg.db_dir)?;
        for w in &db.warnings {
            warn!("{w}");
        }
        info!(
            "db: {} items ({:?}), {} monsters ({:?}), {} jobs",
            db.items.len(),
            db.items_meta.tier,
            db.mobs.len(),
            db.mobs_meta.tier,
            db.jobs.len()
        );
        let store = Store::open(&cfg.save_dir)?;
        info!("saves: {} accounts, {} characters in {}", store.accounts.len(), store.chars.len(), cfg.save_dir.display());
        let mut g = Game {
            cfg,
            res,
            db,
            store,
            out,
            sessions: HashMap::new(),
            login_tokens: HashMap::new(),
            map_tokens: HashMap::new(),
            world: World::new(),
            scripts: None,
            dialogs: HashMap::new(),
            dialog_open: HashMap::new(),
            respawns: Vec::new(),
            npcrt: Default::default(),
            start: Instant::now(),
            last_autosave: 0,
            notices: Vec::new(),
        };
        g.reload_scripts()?;
        Ok(g)
    }

    /// Server tick in ms (what the client sees in 0x007f etc.).
    pub fn now(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
    pub fn tick32(&self) -> u32 {
        self.now() as u32
    }

    pub(crate) fn send(&self, sid: u32, data: Vec<u8>) {
        self.out.send(sid, data);
    }

    pub fn notice(&mut self, msg: impl Into<String>) {
        let m = msg.into();
        warn!("{m}");
        self.notices.push(m);
        if self.notices.len() > 200 {
            self.notices.drain(..100);
        }
    }

    // ------------------------------------------------------------ maps

    /// Load a map's cells on demand. False if the client has no such map.
    pub fn ensure_map(&mut self, name: &str) -> bool {
        let name = name.to_ascii_lowercase();
        if self.world.maps.contains_key(&name) {
            return true;
        }
        if !self.res.maps.contains(&name) {
            return false;
        }
        match self.res.vfs.read(&format!("data\\{name}.gat")) {
            Ok(b) => match MapGrid::from_gat(&name, &b) {
                Ok(grid) => {
                    self.world.maps.insert(
                        name.clone(),
                        MapInst { grid: Arc::new(grid), units: Default::default(), players: 0, populated: false },
                    );
                    self.place_static_units(&name);
                    true
                }
                Err(e) => {
                    self.notice(format!("map {name}: {e}"));
                    false
                }
            },
            Err(e) => {
                self.notice(format!("map {name}: {e}"));
                false
            }
        }
    }

    pub fn grid(&self, map: &str) -> Option<Arc<MapGrid>> {
        self.world.maps.get(map).map(|m| m.grid.clone())
    }

    // ------------------------------------------------------------ scripts

    /// (Re)load all scripts. On failure the previous scripts stay active.
    pub fn reload_scripts(&mut self) -> Result<String> {
        let opts = crate::script::LoadOptions { content: self.cfg.content.clone() };
        let engine = match ScriptEngine::load_with(&self.cfg.script_dir, &opts) {
            Ok(e) => e,
            Err(e) => {
                let msg = format!("script load failed (keeping previous scripts): {e:#}");
                self.notice(msg.clone());
                if self.scripts.is_none() {
                    return Err(e);
                }
                return Ok(msg);
            }
        };
        // close open dialogs: their coroutines belong to the old state
        let open: Vec<u32> = self.dialogs.keys().copied().collect();
        for pid in open {
            if let Some(d) = self.dialogs.remove(&pid) {
                if let Some(sid) = self.sid_of(pid) {
                    self.send(sid, packets::npc_close(d.npc));
                }
            }
        }
        // drop every scripted unit (npcs, warps, shops, spawned mobs) ...
        let doomed: Vec<u32> = self
            .world
            .units
            .values()
            .filter(|u| !u.is_player() && !u.is_item())
            .map(|u| u.id)
            .collect();
        for id in doomed {
            self.despawn(id, 0);
        }
        self.respawns.clear();
        let summary = format!(
            "scripts: {} files, {} npcs, {} warps, {} shops, {} spawn groups, {} commands",
            engine.defs.files.len(),
            engine.defs.npcs.len(),
            engine.defs.warps.len(),
            engine.defs.shops.len(),
            engine.defs.spawns.len(),
            engine.defs.commands.len()
        );
        self.scripts = Some(Rc::new(engine));
        // fresh scripts, fresh runtime state (variables survive: they're saved data)
        self.npcrt.disabled.clear();
        self.npcrt.timers.clear();
        self.npcrt.queue.clear();
        self.queue_init_events();
        self.run_events();
        // ... and put the new ones back on every loaded map
        let maps: Vec<String> = self.world.maps.keys().cloned().collect();
        for m in maps {
            if let Some(mi) = self.world.maps.get_mut(&m) {
                mi.populated = false;
            }
            self.place_static_units(&m);
            if self.world.maps.get(&m).is_some_and(|mi| mi.players > 0) {
                self.populate_map(&m);
            }
        }
        // players see the new world
        for pid in self.world.all_players() {
            self.refresh_view(pid, true);
        }
        info!("{summary}");
        Ok(summary)
    }

    // ------------------------------------------------------------ events

    pub fn handle(&mut self, ev: NetEvent) {
        match ev {
            NetEvent::Connected { sid, kind } => {
                let state = match kind {
                    ServerKind::Login => State::Login,
                    ServerKind::Char => State::Char { account_id: None },
                    ServerKind::Map => State::Closed,
                };
                self.sessions.insert(sid, Session { kind, state, md5_key: None });
            }
            NetEvent::Disconnected { sid } => self.disconnect(sid),
            NetEvent::Packet { sid, data } => {
                let Some(kind) = self.sessions.get(&sid).map(|s| s.kind) else { return };
                let r = match kind {
                    ServerKind::Login => self.login_packet(sid, &data),
                    ServerKind::Char => self.char_packet(sid, &data),
                    ServerKind::Map => self.map_packet(sid, &data),
                };
                if let Err(e) = r {
                    warn!("session {sid}: packet {:02x}{:02x}: {e:#}", data[1], data[0]);
                }
            }
        }
    }

    fn disconnect(&mut self, sid: u32) {
        let Some(s) = self.sessions.remove(&sid) else { return };
        if let State::Map { unit, char_id, .. } = s.state {
            self.leave_world(unit, char_id, 2);
        }
    }

    pub(crate) fn sid_of(&self, unit: u32) -> Option<u32> {
        self.world.units.get(&unit).and_then(|u| u.player()).map(|p| p.sid)
    }

    // ------------------------------------------------------------ login server

    fn login_packet(&mut self, sid: u32, pkt: &[u8]) -> Result<()> {
        let mut r = Reader::new(pkt);
        match r.id() {
            CA_LOGIN => {
                let _version = r.u32()?;
                let userid = r.str_fixed(24)?;
                let password = r.str_fixed(24)?;
                let _clienttype = r.u8()?;
                self.login(sid, &userid, |acc_pw| acc_pw == password, &password);
            }
            CA_REQ_HASH => {
                let key: Vec<u8> = (0..16).map(|_| rand::random::<u8>() % 94 + 33).collect();
                let mut w = ro_proto::Writer::new(AC_ACK_HASH);
                w.bytes(&key);
                self.send(sid, w.finish());
                if let Some(s) = self.sessions.get_mut(&sid) {
                    s.md5_key = Some(key);
                }
            }
            CA_LOGIN2 => {
                let _version = r.u32()?;
                let userid = r.str_fixed(24)?;
                let hash = r.bytes(16)?.to_vec();
                let key = self.sessions.get(&sid).and_then(|s| s.md5_key.clone()).unwrap_or_default();
                let check = move |pw: &str| {
                    use md5::{Digest, Md5};
                    let a = Md5::digest([key.as_slice(), pw.as_bytes()].concat());
                    let b = Md5::digest([pw.as_bytes(), key.as_slice()].concat());
                    a.as_slice() == hash || b.as_slice() == hash
                };
                // auto-register cannot learn a password from a hash
                self.login(sid, &userid, check, "");
            }
            CZ_PING => {}
            other => debug!("login: unhandled {other:#06x}"),
        }
        Ok(())
    }

    fn login(&mut self, sid: u32, userid: &str, check: impl Fn(&str) -> bool, plain_pw: &str) {
        let (mut name, mut sex) = (userid.to_string(), self.cfg.default_sex);
        if let Some(base) = userid.strip_suffix("_M").or_else(|| userid.strip_suffix("_m")) {
            name = base.into();
            sex = 1;
        } else if let Some(base) = userid.strip_suffix("_F").or_else(|| userid.strip_suffix("_f")) {
            name = base.into();
            sex = 0;
        }
        let account_id = match self.store.account_by_userid(&name) {
            Some(a) if check(&a.password) => a.id,
            Some(_) => {
                info!("login: wrong password for {name}");
                self.send(sid, packets::refuse_login(1));
                return;
            }
            None if self.cfg.auto_register && name.len() >= 4 && !plain_pw.is_empty() => {
                match self.store.create_account(&name, plain_pw, sex, self.cfg.new_account_gm_level) {
                    Ok(id) => {
                        info!("login: registered new account {name} ({id})");
                        id
                    }
                    Err(e) => {
                        warn!("register {name}: {e:#}");
                        self.send(sid, packets::refuse_login(3));
                        return;
                    }
                }
            }
            None => {
                self.send(sid, packets::refuse_login(0));
                return;
            }
        };
        let sex = self.store.accounts[&account_id].sex;
        let (l1, l2) = (rand::random::<u32>() | 1, rand::random::<u32>() | 1);
        self.login_tokens.insert(account_id, (l1, l2, sex));
        let ip = u32::from_le_bytes(self.cfg.public_ip);
        let users = self.world.all_players().len() as u16;
        let name = self.cfg.server_name.clone();
        info!("login: {userid} ok (account {account_id})");
        self.send(sid, packets::accept_login(l1, account_id, l2, sex, &[(ip, self.cfg.char_port, &name, users)]));
    }

    // ------------------------------------------------------------ char server

    fn char_entries(&self, account_id: u32) -> Vec<(u32, crate::packets::CharView, u32, u32)> {
        self.store
            .chars_of(account_id)
            .iter()
            .map(|c| {
                let d = self.derive(c);
                (c.id, self.char_view(c), d.max_hp, d.max_sp)
            })
            .collect()
    }

    fn send_char_list(&self, sid: u32, account_id: u32) {
        let entries = self.char_entries(account_id);
        let list: Vec<CharEntry> = entries
            .iter()
            .map(|(id, view, hp, sp)| CharEntry { ch: &self.store.chars[id], view: *view, max_hp: *hp, max_sp: *sp })
            .collect();
        self.send(sid, packets::char_list(&list));
    }

    fn char_packet(&mut self, sid: u32, pkt: &[u8]) -> Result<()> {
        let mut r = Reader::new(pkt);
        let account = match self.sessions.get(&sid).map(|s| s.state.clone()) {
            Some(State::Char { account_id }) => account_id,
            _ => None,
        };
        match (r.id(), account) {
            (CH_ENTER, _) => {
                let aid = r.u32()?;
                let l1 = r.u32()?;
                let l2 = r.u32()?;
                let _ = r.u16()?;
                let _sex = r.u8()?;
                // Athena-era quirk: raw account id, no packet header.
                self.send(sid, aid.to_le_bytes().to_vec());
                match self.login_tokens.get(&aid) {
                    Some(&(a, b, _)) if a == l1 && b == l2 => {
                        if let Some(s) = self.sessions.get_mut(&sid) {
                            s.state = State::Char { account_id: Some(aid) };
                        }
                        self.send_char_list(sid, aid);
                    }
                    _ => {
                        self.send(sid, packets::refuse_enter(0));
                        self.out.close(sid);
                    }
                }
            }
            (CH_MAKE_CHAR, Some(aid)) => {
                let name = r.str_fixed(24)?;
                let mut stats = [0u16; 6];
                for s in stats.iter_mut() {
                    *s = r.u8()? as u16;
                }
                let slot = r.u8()?;
                let hair_color = r.u16()?;
                let hair = r.u16()?;
                self.make_char(sid, aid, name.trim(), stats, slot, hair_color, hair);
            }
            (CH_DELETE_CHAR, Some(aid)) => {
                let cid = r.u32()?;
                let ok = self.store.chars.get(&cid).is_some_and(|c| c.account_id == aid);
                if ok && self.store.delete_char(cid).is_ok() {
                    info!("char {cid} deleted");
                    self.send(sid, packets::char_deleted());
                } else {
                    self.send(sid, packets::char_delete_refused(0));
                }
            }
            (CH_SELECT_CHAR, Some(aid)) => {
                let slot = r.u8()?;
                let cid = self.store.accounts.get(&aid).and_then(|a| a.slots.get(&slot).copied());
                let Some(cid) = cid else {
                    self.send(sid, packets::refuse_enter(1));
                    return Ok(());
                };
                let l1 = self.login_tokens.get(&aid).map(|t| t.0).unwrap_or(0);
                self.map_tokens.insert(cid, (aid, l1));
                let p = self.valid_point(cid);
                if let Some(c) = self.store.chars.get_mut(&cid) {
                    c.last_point = p.clone();
                }
                let ip = u32::from_le_bytes(self.cfg.public_ip);
                info!("char {cid} -> map {} {},{}", p.map, p.x, p.y);
                self.send(sid, packets::zone_server(cid, &p.map, ip, self.cfg.map_port));
            }
            (CZ_PING, _) => {}
            (id, _) => debug!("char: unhandled {id:#06x}"),
        }
        Ok(())
    }

    /// Where a character can actually stand: last point, else save point, else start.
    fn valid_point(&mut self, cid: u32) -> Point {
        let (last, save) = match self.store.chars.get(&cid) {
            Some(c) => (c.last_point.clone(), c.save_point.clone()),
            None => return self.cfg.start_point.clone(),
        };
        for p in [last, save, self.cfg.start_point.clone(), Point::new("prontera", 156, 191)] {
            if self.ensure_map(&p.map) && self.grid(&p.map).is_some_and(|g| g.walkable(p.x as i32, p.y as i32)) {
                return p;
            }
        }
        self.cfg.start_point.clone()
    }

    #[allow(clippy::too_many_arguments)]
    fn make_char(&mut self, sid: u32, aid: u32, name: &str, stats: [u16; 6], slot: u8, hair_color: u16, hair: u16) {
        // Creation rules of the period client/server: 4+ chars, unique name,
        // free slot, stats 1..9 with str+int, agi+luk, vit+dex each = 10.
        let valid_stats = stats.iter().all(|&s| (1..=9).contains(&s))
            && stats[0] + stats[3] == 10
            && stats[1] + stats[5] == 10
            && stats[2] + stats[4] == 10;
        let slot_free = self.store.accounts.get(&aid).is_some_and(|a| !a.slots.contains_key(&slot)) && slot < 9;
        if name.len() < 4 || self.store.name_taken(name) {
            self.send(sid, packets::char_create_refused(0x00));
            return;
        }
        if !valid_stats || !slot_free || hair > 23 || hair_color > 8 {
            self.send(sid, packets::char_create_refused(0xff));
            return;
        }
        let mut c = Character {
            account_id: aid,
            slot,
            name: name.into(),
            stats,
            hair,
            hair_color,
            zeny: self.cfg.start_zeny,
            last_point: self.cfg.start_point.clone(),
            save_point: self.cfg.start_point.clone(),
            ..Default::default()
        };
        for &(id, n) in &self.cfg.start_items {
            if self.db.item(id).is_some() {
                c.inventory.push(InvItem::new(id, n));
            }
        }
        let d = self.derive(&c);
        c.hp = d.max_hp;
        c.sp = d.max_sp;
        match self.store.create_char(c) {
            Ok(cid) => {
                info!("char created: {name} ({cid}) slot {slot}");
                let c = &self.store.chars[&cid];
                let entry = CharEntry { ch: c, view: self.char_view(c), max_hp: d.max_hp, max_sp: d.max_sp };
                self.send(sid, packets::char_created(&entry));
            }
            Err(e) => {
                warn!("create char: {e:#}");
                self.send(sid, packets::char_create_refused(0xff));
            }
        }
    }

    // ------------------------------------------------------------ map server entry

    pub(crate) fn map_enter(&mut self, sid: u32, aid: u32, cid: u32, l1: u32) -> Result<()> {
        // Athena-era quirk: raw account id first.
        self.send(sid, aid.to_le_bytes().to_vec());
        let ok = self.map_tokens.get(&cid).is_some_and(|&(a, l)| a == aid && l == l1)
            && self.store.chars.get(&cid).is_some_and(|c| c.account_id == aid);
        if !ok {
            self.send(sid, packets::refuse_enter(0));
            self.out.close(sid);
            return Ok(());
        }
        self.map_tokens.remove(&cid);
        // already in the world (reconnect)? drop the old unit
        if let Some(old) = self.world.units.values().find(|u| u.player().is_some_and(|p| p.char_id == cid)).map(|u| u.id) {
            self.leave_world(old, cid, 2);
        }
        let p = self.valid_point(cid);
        let gm = self.store.accounts.get(&aid).map(|a| a.gm_level).unwrap_or(0);
        let sex = self.store.accounts.get(&aid).map(|a| a.sex).unwrap_or(1);
        let unit = aid; // players use the account id as their unit id (Athena)
        self.spawn_player(sid, unit, aid, cid, sex, gm, &p);
        if let Some(s) = self.sessions.get_mut(&sid) {
            s.state = State::Map { account_id: aid, char_id: cid, unit };
        }
        self.send(sid, packets::accept_enter(self.tick32(), p.x, p.y, 4));
        Ok(())
    }

    /// Where a character on `pt` comes back next login: a nosave map sends it
    /// to its point (Athena pc_makesavestatus).
    fn storable_point(&self, char_id: u32, pt: Point) -> Point {
        let Some(to) = self.db.mapflags.get(&pt.map).filter(|m| m.has("nosave")).and_then(|m| m.nosave.clone()) else { return pt };
        if to.eq_ignore_ascii_case("SavePoint") {
            return self.store.chars.get(&char_id).map(|c| c.save_point.clone()).unwrap_or(pt);
        }
        let p: Vec<&str> = to.split(',').collect();
        match (p.first(), p.get(1).and_then(|x| x.parse().ok()), p.get(2).and_then(|x| x.parse().ok())) {
            (Some(m), Some(x), Some(y)) => Point::new(m, x, y),
            _ => pt,
        }
    }

    pub(crate) fn leave_world(&mut self, unit: u32, char_id: u32, vanish_kind: u8) {
        if let Some(pt) = self.world.units.get(&unit).map(|u| Point::new(&u.map, u.x, u.y)) {
            let pt = self.storable_point(char_id, pt);
            if let Some(c) = self.store.chars.get_mut(&char_id) {
                c.last_point = pt;
            }
        }
        self.dialogs.remove(&unit);
        self.pet_owner_left(unit);
        self.player_left(unit);
        self.despawn(unit, vanish_kind);
        if let Err(e) = self.store.save_char(char_id) {
            warn!("save {char_id}: {e:#}");
        }
    }

    // ------------------------------------------------------------ timers

    pub fn tick(&mut self) {
        let now = self.now();
        self.tick_walks(now);
        self.tick_combat(now);
        self.tick_regen(now);
        self.tick_items(now);
        self.tick_respawns(now);
        self.tick_npc_timers(now);
        self.tick_pets(now);
        self.tick_status_changes(now);
        self.run_events();
        if now - self.last_autosave >= self.cfg.autosave_secs * 1000 {
            self.last_autosave = now;
            self.save_all();
        }
    }

    pub fn save_all(&mut self) {
        self.save_globals();
        let players: Vec<(u32, u32)> = self
            .world
            .units
            .values()
            .filter_map(|u| u.player().map(|p| (u.id, p.char_id)))
            .collect();
        for (unit, cid) in players {
            if let Some(pt) = self.world.units.get(&unit).map(|u| Point::new(&u.map, u.x, u.y)) {
                let pt = self.storable_point(cid, pt);
                if let Some(c) = self.store.chars.get_mut(&cid) {
                    c.last_point = pt;
                }
            }
            if let Err(e) = self.store.save_char(cid) {
                warn!("autosave {cid}: {e:#}");
            }
        }
    }

    /// Run Lua as the given player (admin console). `player` 0 = none.
    pub fn console_eval(&mut self, code: &str, player: u32) -> String {
        let Some(engine) = self.scripts.clone() else { return "no scripts loaded".into() };
        let ctx = Ctx::new(player, 0, crate::script::CtxKind::Console);
        let r = engine.eval(self, ctx, code);
        match r {
            Ok(s) => s,
            Err(e) => format!("error: {e:#}"),
        }
    }

    /// Every map name the loaded scripts refer to (one entry per reference).
    pub fn script_maps(&self) -> Vec<String> {
        let Some(e) = &self.scripts else { return vec![] };
        let d = &e.defs;
        d.npcs.iter().filter(|n| n.map != "-").map(|n| n.map.clone())
            .chain(d.warps.iter().flat_map(|w| [w.map.clone(), w.to.0.clone()]))
            .chain(d.shops.iter().map(|s| s.map.clone()))
            .chain(d.spawns.iter().map(|s| s.map.clone()))
            .collect()
    }

    /// First online player unit (convenience for the console).
    pub fn first_player(&self) -> u32 {
        self.world.all_players().into_iter().min().unwrap_or(0)
    }
}
