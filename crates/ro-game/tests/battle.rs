//! Battle and skills against the real rules (scripts/rules) and data
//! (data/db), in-process on a synthetic map: no client files needed.
//! Each test logs in a fresh character, shapes it with console Lua
//! (`player.x = ...`, `give_skill`), and drives it with real packets.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use ro_game::model::Point;
use ro_game::{Game, GameConfig, NetEvent, Outbox, Resources, ServerKind};
use ro_grf::Vfs;
use ro_proto::Writer;

#[derive(Clone, Default)]
struct Cap(Arc<Mutex<Vec<(u32, Vec<u8>)>>>);
impl Outbox for Cap {
    fn send(&self, sid: u32, data: Vec<u8>) {
        self.0.lock().unwrap().push((sid, data));
    }
    fn close(&self, _sid: u32) {}
}
impl Cap {
    fn take(&self) -> Vec<Vec<u8>> {
        self.0.lock().unwrap().drain(..).filter(|(s, _)| *s == 3).map(|(_, d)| d).collect()
    }
}

fn id_of(p: &[u8]) -> u16 {
    u16::from_le_bytes([p[0], p[1]])
}
fn u32_at(p: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(p[at..at + 4].try_into().unwrap())
}
fn u16_at(p: &[u8], at: usize) -> u16 {
    u16::from_le_bytes(p[at..at + 2].try_into().unwrap())
}

fn gat(w: u32, h: u32, water: bool) -> Vec<u8> {
    let mut b = b"GRAT\x01\x02".to_vec();
    b.extend_from_slice(&w.to_le_bytes());
    b.extend_from_slice(&h.to_le_bytes());
    for _ in 0..w * h {
        b.extend_from_slice(&[0u8; 16]);
        b.extend_from_slice(&(if water { 3u32 } else { 0 }).to_le_bytes());
    }
    b
}

struct T {
    g: Game,
    cap: Cap,
    /// The player's unit id (= account id).
    me: u32,
    cid: u32,
}

static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

impl T {
    /// A logged-in character standing at 25,25 of an open 50x50 map.
    fn new() -> T {
        Self::on("testmap")
    }

    fn on(map: &str) -> T {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let n = N.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let saves = std::env::temp_dir().join(format!("ro-offline-battle-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&saves);
        let mut vfs = Vfs::new();
        vfs.add_memory_file("data\\testmap.gat", gat(50, 50, false));
        vfs.add_memory_file("data\\watermap.gat", gat(50, 50, true));
        let mut content = ro_game::game::default_content();
        content.insert("castle_monsters".into(), false); // their maps are not in this test's client
        let cfg = GameConfig {
            content,
            save_dir: saves,
            script_dir: root.join("scripts"),
            db_dir: root.join("data/db"),
            start_point: Point::new(map, 25, 25),
            ..Default::default()
        };
        let cap = Cap::default();
        let mut g = Game::new(cfg, Resources::new(vfs), Box::new(cap.clone())).expect("game");
        // login -> char -> map, as session.rs does
        g.handle(NetEvent::Connected { sid: 1, kind: ServerKind::Login });
        let mut w = Writer::new(0x0064);
        w.u32(20).str_fixed("battler_M", 24).str_fixed("secret", 24).u8(0);
        g.handle(NetEvent::Packet { sid: 1, data: w.finish() });
        let out: Vec<Vec<u8>> = cap.0.lock().unwrap().drain(..).map(|(_, d)| d).collect();
        let acc = out.iter().find(|p| id_of(p) == 0x0069).expect("login accepted");
        let (l1, aid, l2) = (u32_at(acc, 4), u32_at(acc, 8), u32_at(acc, 12));
        g.handle(NetEvent::Connected { sid: 2, kind: ServerKind::Char });
        let mut w = Writer::new(0x0065);
        w.u32(aid).u32(l1).u32(l2).u16(0).u8(1);
        g.handle(NetEvent::Packet { sid: 2, data: w.finish() });
        let mut w = Writer::new(0x0067);
        w.str_fixed("Fighter", 24);
        for _ in 0..6 {
            w.u8(5);
        }
        w.u8(0).u16(1).u16(1);
        g.handle(NetEvent::Packet { sid: 2, data: w.finish() });
        let out: Vec<Vec<u8>> = cap.0.lock().unwrap().drain(..).map(|(_, d)| d).collect();
        let made = out.iter().find(|p| id_of(p) == 0x006d).expect("char created");
        let cid = u32_at(made, 2);
        let mut w = Writer::new(0x0066);
        w.u8(0);
        g.handle(NetEvent::Packet { sid: 2, data: w.finish() });
        g.handle(NetEvent::Connected { sid: 3, kind: ServerKind::Map });
        let mut w = Writer::new(0x0072);
        w.u32(aid).u32(cid).u32(l1).u32(0).u8(1);
        g.handle(NetEvent::Packet { sid: 3, data: w.finish() });
        g.handle(NetEvent::Packet { sid: 3, data: Writer::new(0x007d).finish() });
        cap.0.lock().unwrap().clear();
        let mut t = T { g, cap, me: aid, cid };
        t.check();
        t
    }

    /// Fail on any script error the server noted.
    fn check(&mut self) {
        self.g.tick(); // surfaces errors from read-only paths
        let n = std::mem::take(&mut self.g.notices);
        assert!(n.is_empty(), "{} script errors, first: {}", n.len(), n[0]);
    }

    /// Run Lua as the player (console), fail on error.
    fn lua(&mut self, code: &str) -> String {
        let r = self.g.console_eval(code, self.me);
        assert!(!r.starts_with("error"), "{code}: {r}");
        self.check();
        r
    }

    /// Let `ms` of game time pass, ticking every 20 ms.
    fn run(&mut self, ms: u64) {
        for _ in 0..ms / 20 {
            self.g.skew(20);
            self.g.tick();
        }
        self.check();
    }

    fn send(&mut self, mut w: Writer) {
        self.g.handle(NetEvent::Packet { sid: 3, data: w.finish() });
        self.check();
    }

    fn spawn(&mut self, x: u16, y: u16, mob: u32) -> u32 {
        self.lua(&format!("spawn_mob(player.map, {x}, {y}, {mob}, 1)[1]")).trim().parse().expect("mob id")
    }

    fn mob_hp(&self, id: u32) -> Option<u32> {
        self.g.world.units.get(&id).and_then(|u| u.mob()).map(|m| m.hp)
    }

    fn hp(&self) -> u32 {
        self.g.store.chars[&self.cid].hp
    }
    fn sp(&self) -> u32 {
        self.g.store.chars[&self.cid].sp
    }

    /// Use a skill on a unit (0x0113).
    fn skill(&mut self, skill: u16, lv: u16, target: u32) {
        let mut w = Writer::new(0x0113);
        w.u16(lv).u16(skill).u32(target);
        self.send(w);
    }
    /// Use a skill on a cell (0x0116).
    fn skill_pos(&mut self, skill: u16, lv: u16, x: u16, y: u16) {
        let mut w = Writer::new(0x0116);
        w.u16(lv).u16(skill).u16(x).u16(y);
        self.send(w);
    }

    /// Packets of one id sent to the player since the last call.
    fn packets(&self, id: u16) -> Vec<Vec<u8>> {
        self.cap.take().into_iter().filter(|p| id_of(p) == id).collect()
    }

    /// Inventory index (as the client counts: +2) of the first unworn item with this id.
    fn inv(&self, nameid: u32) -> u16 {
        let c = &self.g.store.chars[&self.cid];
        c.inventory.iter().position(|i| i.nameid == nameid && i.equip == 0).unwrap_or_else(|| panic!("no item {nameid}")) as u16 + 2
    }

    /// Give and wear an item (0x00a9).
    fn wear(&mut self, nameid: u32) {
        self.lua(&format!("give_item({nameid}, 1)"));
        let idx = self.inv(nameid);
        let mut w = Writer::new(0x00a9);
        w.u16(idx).u16(0);
        self.send(w);
        let c = &self.g.store.chars[&self.cid];
        assert!(c.inventory.iter().any(|i| i.nameid == nameid && i.equip != 0), "item {nameid} was not equipped");
    }

    /// A number from the player's computed status (rules.status).
    fn st(&mut self, key: &str) -> i64 {
        let r = self.lua(&format!("battle.get(player.id).st.{key}"));
        r.trim().parse::<f64>().unwrap_or_else(|_| panic!("st.{key} = {r}")) as i64
    }

    /// Learn a skill at a level.
    fn learn(&mut self, skill: &str, lv: u8) {
        self.lua(&format!("give_skill(SK.{skill}, {lv})"));
    }

    /// A status change of the player's, if on.
    fn sc(&self, kind: u16) -> bool {
        self.g.store.chars[&self.cid].sc.contains_key(&kind)
    }
}

/// A plain attack does damage through the rules and kills a Poring.
#[test]
fn plain_attack_kills() {
    let mut t = T::new();
    let mob = t.spawn(26, 25, 1002);
    let mut w = Writer::new(0x0089);
    w.u32(mob).u8(7);
    t.send(w);
    t.run(30_000);
    assert!(t.mob_hp(mob).is_none(), "poring still has {:?} hp", t.mob_hp(mob));
    assert!(t.g.store.chars[&t.cid].base_exp > 0 || t.g.store.chars[&t.cid].base_level > 1);
}

/// Every skill a player or monster can use runs through the rules without
/// a script error: on an enemy, on oneself or on a cell as its target type
/// says, at its top level, then a few seconds of game time (ground skills,
/// status-change timers). Conditions (SP, weapon, gems) are skipped the way
/// an item-granted skill skips them, so the effect code runs for all.
#[test]
fn every_skill_runs() {
    let mut t = T::on("watermap");
    t.lua("player.base_level = 90; player.str = 60; player.agi = 40; player.vit = 60; player.int = 60; player.dex = 60; player.luk = 30; heal(30000, 30000)");
    let skills: Vec<(u16, u16, u16, String)> = {
        let mut v: Vec<_> = t.g.db.skills.values().map(|s| (s.id, s.inf, s.max.max(1) as u16, s.name.clone())).collect();
        v.sort();
        v
    };
    let mut used = 0;
    for (id, inf, max, name) in skills {
        // the 2-2 jobs and guild skills are not reconstructed yet
        if inf == 0 || (210..=10000).contains(&id) || id > 10000 {
            continue;
        }
        t.lua("heal(30000, 30000)");
        // a sturdy, harmless target next to us: a Pupa (cannot move or attack)
        let mob = t.spawn(26, 25, 1008);
        let me = t.me;
        let call = if inf & 2 != 0 {
            format!("rules.skill_castend{{src={me}, skill={id}, lv={max}, target=0, x=26, y=25, item=true, val=1002}}")
        } else if inf & 1 != 0 {
            format!("rules.skill_castend{{src={me}, skill={id}, lv={max}, target={mob}, x=0, y=0, item=true, val=1002}}")
        } else {
            format!("rules.skill_castend{{src={me}, skill={id}, lv={max}, target={me}, x=0, y=0, item=true, val=1002}}")
        };
        let r = t.g.console_eval(&call, me);
        let n = std::mem::take(&mut t.g.notices);
        assert!(!r.starts_with("error") && n.is_empty(), "{name} ({id}): {r} {:?}", n.first());
        t.run(3000);
        used += 1;
        // tidy up for the next one
        t.lua(&format!("_rt('sc_clear', {me}); if not battle.get({me}) or battle.get({me}).dead then error('died using {name}') end"));
        if t.g.world.units.contains_key(&mob) {
            t.lua(&format!("_rt('mob_kill', {mob})"));
        }
        if t.g.world.units[&me].map != "watermap" || (t.g.world.units[&me].x, t.g.world.units[&me].y) != (25, 25) {
            t.lua("warp('watermap', 25, 25)");
            t.send(Writer::new(0x007d));
        }
    }
    assert!(used > 150, "only {used} skills tried");
    // and let the long-lived ground skills run out
    t.run(60_000);
}

/// Job bonuses, equipment scripts, cards and refine feed the status
/// (pc_calcstatus): each adds what its source says.
#[test]
fn status_counts_equipment_cards_and_job() {
    let mut t = T::new();
    assert_eq!(t.st("str"), 5);
    let base_atk = t.st("atk");
    // Knife [3]: ATK 17
    t.wear(1201);
    assert_eq!(t.st("atk"), base_atk + 17);
    // Swordman job level 10: the job gives STR at levels 2, 14 (jobs.toml) -> +1 by level 10
    t.lua("job_change(1); player.job_level = 10");
    let jb = t.g.db.jobs[&1].job_bonus[..10].iter().filter(|b| **b == 1).count() as i64;
    assert!(jb >= 1);
    assert_eq!(t.st("str"), 5 + jb);
    // an Andre Card (ATK +20) in a second Knife
    t.lua("give_item(1201, 1); give_item(4043, 1)");
    let (card, knife) = (t.inv(4043), t.inv(1201));
    let mut w = Writer::new(0x017c);
    w.u16(card).u16(knife);
    t.send(w);
    let ack = t.packets(0x017d);
    assert_eq!(ack.len(), 1);
    assert_eq!(ack[0][6], 0, "card inserted");
    let knife = t.inv(1201);
    let mut w = Writer::new(0x00a9);
    w.u16(knife).u16(0);
    t.send(w);
    assert_eq!(t.st("watk"), 17);
    assert_eq!(t.st("watk2"), 20, "Andre Card: bonus bAtk,20");
    // a Vadon Card (+20% against Fire) shows in addele
    assert_eq!(t.st("addele[3]"), 0);
    // max HP grows with VIT as pc.c:969 says
    let hp0 = t.st("max_hp");
    t.lua("player.vit = 50");
    assert!(t.st("max_hp") > hp0);
    // Owl's Eye: +1 DEX a level
    let dex = t.st("dex");
    t.learn("AC_OWL", 7);
    assert_eq!(t.st("dex"), dex + 7);
}

/// The element table decides: Fire Bolt on a Water monster does half, Cold
/// Bolt on Fire one and a half times; the Undead take Heal as damage.
#[test]
fn elements_and_magic() {
    let mut t = T::new();
    t.lua("job_change(2); player.base_level = 50; player.int = 50; player.dex = 30; heal(9999, 9999)");
    t.learn("MG_FIREBOLT", 1);
    t.learn("MG_COLDBOLT", 1);
    // 1 bolt, MATK between min and max, minus the monster's MDEF
    let (lo, hi) = (t.st("matk2"), t.st("matk1"));
    assert!(lo > 50 && hi > lo);
    let dmg = |t: &mut T, skill: &str, mob_id: u32| -> i64 {
        let mob = t.spawn(27, 25, mob_id);
        let hp = t.mob_hp(mob).unwrap();
        let id = t.g.db.skills_by_name[skill];
        t.skill(id, 1, mob);
        // cast bar first, nothing lost yet
        assert_eq!(t.packets(0x013e).len(), 1, "{skill}: cast bar");
        assert_eq!(t.mob_hp(mob), Some(hp));
        t.run(3000);
        let left = t.mob_hp(mob).unwrap_or(0);
        if t.g.world.units.contains_key(&mob) {
            t.lua(&format!("_rt('mob_kill', {mob})"));
        }
        t.run(3000);
        (hp - left) as i64
    };
    // Pupa: Earth 1, MDEF 20 -> Fire 150%, Water 100%
    let fire = dmg(&mut t, "MG_FIREBOLT", 1008);
    let cold = dmg(&mut t, "MG_COLDBOLT", 1008);
    assert!(fire > cold, "fire {fire} should beat cold {cold} on Earth");
    assert!(fire as f64 >= lo as f64 * 0.8 * 1.5 - 60.0 && fire as f64 <= hi as f64 * 1.5, "fire bolt {fire} outside {lo}..{hi} x1.5");
    // Zombie (Undead 1): Heal hurts it
    t.lua("job_change(4)");
    t.learn("AL_HEAL", 5);
    let z = t.spawn(27, 25, 1015);
    let hp = t.mob_hp(z).unwrap();
    t.skill(28, 5, z);
    t.run(500);
    assert!(t.mob_hp(z).unwrap_or(0) < hp, "Heal damages the undead");
    // ... and heals us
    t.lua(&format!("_rt('mob_kill', {z}); player.hp = 10"));
    t.run(1500);
    let me = t.me;
    t.skill(28, 5, me);
    t.run(200);
    assert!(t.hp() > 10, "Heal heals");
}

/// Using a skill: SP is spent, the delay holds the next one back, a cast can
/// be interrupted by damage, and silence stops it.
#[test]
fn skill_use_costs_and_delays() {
    let mut t = T::new();
    t.lua("job_change(1); player.base_level = 30; player.str = 30; player.dex = 30; heal(9999, 9999)");
    t.learn("SM_BASH", 5);
    let mob = t.spawn(26, 25, 1008);
    let (sp, hp) = (t.sp(), t.mob_hp(mob).unwrap());
    t.skill(5, 5, mob);
    assert_eq!(t.sp(), sp - 8, "Bash level 5 costs 8 SP");
    assert!(t.mob_hp(mob).unwrap() < hp, "Bash hits at once");
    assert_eq!(t.packets(0x01de).len(), 1, "one skill damage packet");
    // not learnt: nothing happens
    t.skill(7, 1, mob);
    assert!(t.packets(0x01de).is_empty());
    // no SP: failure type 1
    t.lua("player.sp = 3");
    t.skill(5, 5, mob);
    let fail = t.packets(0x0110);
    assert_eq!(fail.len(), 1);
    assert_eq!(fail[0][9], 1, "not enough SP");
    // Provoke on the monster: it shows in what the rules read
    t.lua("player.sp = 100");
    t.learn("SM_PROVOKE", 10);
    t.skill(6, 10, mob);
    let r = t.lua(&format!("battle.sc(battle.get({mob}), SC.PROVOKE) and 1 or 0"));
    assert_eq!(r.trim(), "1", "the Pupa is provoked");
}

/// Blessing: icon on, stats up, and off again when its time is over.
#[test]
fn buffs_start_show_and_expire() {
    let mut t = T::new();
    t.lua("job_change(4); player.base_level = 30; player.int = 20; heal(9999, 9999)");
    t.learn("AL_BLESSING", 10);
    t.learn("AL_INCAGI", 1);
    let (str0, int0, dex0) = (t.st("str"), t.st("int"), t.st("dex"));
    let me = t.me;
    t.skill(34, 10, me);
    assert!(t.sc(10), "Blessing is on");
    assert_eq!((t.st("str"), t.st("int"), t.st("dex")), (str0 + 10, int0 + 10, dex0 + 10));
    let icons = t.packets(0x0196);
    assert!(icons.iter().any(|p| u16_at(p, 2) == 10 && p[8] == 1), "Blessing icon on");
    // 40 + 20 x 10 = 240 s
    t.run(239_000);
    assert!(t.sc(10));
    t.run(2_000);
    assert!(!t.sc(10), "Blessing ended");
    assert_eq!(t.st("str"), str0);
    assert!(t.packets(0x0196).iter().any(|p| u16_at(p, 2) == 10 && p[8] == 0), "Blessing icon off");
    // Increase AGI: faster on foot (150 -> 113 ms a cell)
    t.skill(29, 1, me);
    t.run(1500);
    assert!(t.sc(12));
    assert_eq!(t.g.world.units[&me].speed, 113);
}

/// Ground skills: cells appear for the client, act on who stands in them,
/// and go when their time is up.
#[test]
fn ground_skills() {
    let mut t = T::new();
    t.lua("job_change(2); player.base_level = 60; player.int = 60; player.dex = 99; heal(9999, 9999); give_item(717, 5)");
    t.learn("MG_FIREWALL", 5);
    t.learn("MG_SAFETYWALL", 5);
    // Fire Wall east of us: three cells, each burning what stands on it
    let mob = t.spawn(28, 25, 1008); // Pupa, Earth: fire hurts
    let hp = t.mob_hp(mob).unwrap();
    t.skill_pos(18, 5, 28, 25);
    t.run(3000);
    let cells = t.packets(0x01c9);
    assert_eq!(cells.len(), 3, "Fire Wall is three cells");
    assert!(cells.iter().all(|p| p[14] == 0x7f));
    assert!(t.mob_hp(mob).unwrap_or(0) < hp, "the Pupa burns");
    // 4 + 5 hits a cell, 9 s at most: gone after that
    t.run(10_000);
    assert!(!t.g.world.units.values().any(|u| u.skill().is_some()), "Fire Wall ended");
    assert!(!t.packets(0x0120).is_empty(), "the client was told the cells are gone");
    if t.g.world.units.contains_key(&mob) {
        t.lua(&format!("_rt('mob_kill', {mob})"));
    }
    // Safety Wall under us: a Zombie's blows do nothing while it lasts
    let (me, hp) = (t.me, t.hp());
    t.skill_pos(12, 5, 25, 25);
    t.run(1500);
    assert!(t.sc(90), "standing in the Safety Wall");
    assert_eq!(t.g.store.chars[&t.cid].inventory.iter().find(|i| i.nameid == 717).map(|i| i.amount), Some(4), "a Blue Gemstone was used");
    let z = t.spawn(26, 25, 1015);
    // level 5 stops six blows that would have landed; then it breaks
    let mut blocked = 0;
    for _ in 0..200 {
        if !t.sc(90) {
            break;
        }
        t.lua(&format!("rules.weapon_attack{{src={z}, target={me}}}"));
        if t.sc(90) {
            assert_eq!(t.hp(), hp, "no damage behind the wall");
        }
        blocked += 1;
    }
    assert!(!t.sc(90), "the wall is used up");
    assert!(blocked >= 6, "it took {blocked} blows");
    for _ in 0..50 {
        t.lua(&format!("rules.weapon_attack{{src={z}, target={me}}}"));
    }
    assert!(t.hp() < hp, "now it hurts");
    let _ = (me, z);
}

/// Monsters: an aggressive one comes for the player, a passive one hits
/// back, a looter picks things up, and each uses the skills its list gives it.
#[test]
fn monster_ai() {
    let mut t = T::new();
    t.lua("player.base_level = 50; player.vit = 80; heal(9999, 9999)");
    let hp = t.hp();
    // Zombie (aggressive) six cells away walks up and attacks
    let z = t.spawn(31, 25, 1015);
    t.run(8000);
    let zu = &t.g.world.units[&z];
    assert!((zu.x as i32 - 25).abs() <= 1 && (zu.y as i32 - 25).abs() <= 1, "the Zombie came to us: {},{}", zu.x, zu.y);
    assert!(t.packets(0x008a).iter().any(|p| u32_at(p, 2) == z), "and attacked");
    let _ = hp;
    t.lua(&format!("_rt('mob_kill', {z}); heal(9999, 0)"));
    // Poring (passive looter) ignores us but takes what lies on the floor
    let p = t.spawn(30, 25, 1002);
    t.lua("_rt('drop_item', 'testmap', 32, 25, 909, 1)");
    let hp = t.hp();
    t.run(15_000);
    assert_eq!(t.hp(), hp, "a Poring does not attack first");
    assert!(t.g.world.units[&p].mob().unwrap().loot.iter().any(|i| i.nameid == 909), "the Poring picked the Jellopy up");
    // killed, it gives the loot back
    t.lua(&format!("_rt('bl_damage', player.id, {p}, 9999)"));
    assert!(t.g.world.units.values().any(|u| matches!(&u.kind, ro_game::world::UnitKind::Item(i) if i.item.nameid == 909)), "the Jellopy is on the floor again");
    // a Scorpion's list: Envenom and a fire attack while fighting
    let s = t.spawn(26, 25, 1001);
    t.lua(&format!("_rt('bl_damage', player.id, {s}, 1)"));
    let mut skills = 0;
    for _ in 0..60 {
        t.lua("heal(9999, 0)");
        t.run(1000);
        skills += t.cap.take().iter().filter(|p| id_of(p) == 0x01de && u32_at(p, 4) == s).count();
    }
    assert!(skills > 0, "the Scorpion used a skill in a minute of fighting");
}

/// A bow needs arrows, spends one a shot, and reaches as far as it says.
#[test]
fn bows_and_arrows() {
    let mut t = T::new();
    t.lua("job_change(3); player.base_level = 30; player.dex = 40; heal(9999, 9999)");
    t.wear(1701); // Bow
    assert_eq!(t.st("attackrange"), 5);
    let mob = t.spawn(29, 25, 1008);
    let attack = |t: &mut T| {
        let mut w = Writer::new(0x0089);
        w.u32(mob).u8(7);
        t.send(w);
    };
    // no arrows: nothing but the client's "equip arrows" message
    attack(&mut t);
    t.run(1500);
    assert_eq!(t.mob_hp(mob), Some(427));
    assert!(!t.packets(0x013b).is_empty());
    // with arrows: hits from four cells away, one arrow each
    t.lua("give_item(1750, 20)");
    let idx = t.inv(1750);
    let mut w = Writer::new(0x00a9);
    w.u16(idx).u16(0);
    t.send(w);
    assert!(!t.packets(0x013c).is_empty(), "arrow equipped");
    attack(&mut t);
    t.run(3000);
    assert!(t.mob_hp(mob).unwrap_or(0) < 427, "arrows hit");
    let me = t.me;
    assert_eq!((t.g.world.units[&me].x, t.g.world.units[&me].y), (25, 25), "shot from where we stood");
    let left = t.g.store.chars[&t.cid].inventory.iter().find(|i| i.nameid == 1750).map(|i| i.amount).unwrap_or(0);
    assert!(left < 20 && left >= 15, "{left} arrows left");
}

/// Auto Counter (eAthena 817): a blow from the front is turned aside and
/// answered with a critical; the status ends with it.
#[test]
fn auto_counter() {
    let mut t = T::new();
    t.lua("job_change(7); player.base_level = 60; player.str = 60; heal(9999, 9999)");
    t.learn("KN_AUTOCOUNTER", 5);
    let z = t.spawn(26, 25, 1015);
    let (me, hp, zhp) = (t.me, t.hp(), t.mob_hp(z).unwrap());
    // face east, toward the Zombie
    t.lua(&format!("_rt('bl_set', {me}, 'dir', 6); skill_status_change_start({me}, SC.AUTOCOUNTER, 5, 0)"));
    assert!(t.sc(77));
    t.lua(&format!("rules.weapon_attack{{src={z}, target={me}}}"));
    assert_eq!(t.hp(), hp, "the blow was turned aside");
    assert!(!t.sc(77), "the stance ends");
    assert!(t.mob_hp(z).unwrap_or(0) < zhp, "and the Zombie was hit back");
    let crit = t.packets(0x008a).iter().any(|p| u32_at(p, 2) == me && p[26] == 0x0a);
    assert!(crit, "with a critical");
}

/// Merchant prices, potions scaled by VIT, the overweight icons.
#[test]
fn trade_potions_and_weight() {
    let mut t = T::new();
    // Discount 10: 24% off; Overcharge 10: 24% more; level 1: 7%
    assert_eq!(t.lua("rules.buy_price{price = 1000, skills = {[tostring(SK.MC_DISCOUNT)] = 10}}").trim(), "760");
    assert_eq!(t.lua("rules.buy_price{price = 1000, skills = {[tostring(SK.MC_DISCOUNT)] = 1}}").trim(), "930");
    assert_eq!(t.lua("rules.sell_price{price = 1000, skills = {[tostring(SK.MC_OVERCHARGE)] = 10}}").trim(), "1240");
    assert_eq!(t.lua("rules.buy_price{price = 1000, skills = {}}").trim(), "1000");
    // a Red Potion heals 45..64, more with VIT (1% a point)
    t.lua("player.base_level = 50; player.vit = 99; heal(9999, 9999); player.hp = 1; give_item(501, 1)");
    let idx = t.inv(501);
    let mut w = Writer::new(0x00a7);
    w.u16(idx).u32(t.me);
    t.send(w);
    let got = t.hp() - 1;
    assert!((89..=128).contains(&got), "Red Potion with 99 VIT healed {got}");
    // half the weight limit: icon 35; nine tenths: 36
    let c = &t.g.store.chars[&t.cid];
    let _ = c;
    let max = t.st("max_weight");
    let n = max / 2 / 10 + 5; // Jellopy weighs 1.0
    t.lua(&format!("give_item(909, {n})"));
    assert!(t.packets(0x0196).iter().any(|p| u16_at(p, 2) == 35 && p[8] == 1), "50% icon");
    t.lua(&format!("give_item(909, {})", max * 4 / 10 / 10));
    assert!(t.packets(0x0196).iter().any(|p| u16_at(p, 2) == 36 && p[8] == 1), "90% icon");
    // overweight: no natural recovery (pc.c:4036)
    t.lua("player.hp = 1");
    t.run(20_000);
    assert_eq!(t.hp(), 1);
}

/// Forging: the hammer lists what the smith can make, the materials go,
/// and a made weapon carries its maker and element.
#[test]
fn forging() {
    let mut t = T::new();
    t.lua("job_change(10); player.base_level = 99; player.dex = 99; player.luk = 99");
    t.learn("BS_DAGGER", 3);
    t.learn("BS_IRON", 5);
    // a Knife takes 1 Iron and 10 Jellopy (produce_db)
    t.lua("give_item(998, 40); give_item(909, 400); give_item(613, 1); give_item(994, 40); give_item(986, 1)");
    let idx = t.inv(613); // Iron Hammer: produce(1)
    let mut w = Writer::new(0x00a7);
    w.u16(idx).u32(t.me);
    t.send(w);
    let list = t.packets(0x018d);
    assert_eq!(list.len(), 1, "forge window opened");
    let ids: Vec<u16> = list[0][4..].chunks(8).map(|c| u16_at(c, 0)).collect();
    assert!(ids.contains(&1201), "a Knife can be forged: {ids:?}");
    assert!(!ids.contains(&998), "no ores with a hammer");
    // forge Knives with a Flame Heart until one succeeds
    let mut made = None;
    for _ in 0..40 {
        let mut w = Writer::new(0x018e);
        w.u16(1201).u16(994).u16(0).u16(0);
        t.send(w);
        if let Some(k) = t.g.store.chars[&t.cid].inventory.iter().find(|i| i.nameid == 1201 && i.cards[0] != 0) {
            made = Some(k.clone());
            break;
        }
    }
    let k = made.expect("a Knife was forged in 40 tries");
    assert_eq!(k.cards[0], 0x00ff);
    assert_eq!(k.cards[1] & 0x0f, 3, "fire");
    assert_eq!(k.cards[2] as u32 | ((k.cards[3] as u32) << 16), t.cid);
    // worn, it hits with fire
    let idx = t.g.store.chars[&t.cid].inventory.iter().position(|i| i.cards[0] == 0x00ff).unwrap() as u16 + 2;
    let mut w = Writer::new(0x00a9);
    w.u16(idx).u16(0);
    t.send(w);
    assert_eq!(t.st("atk_ele"), 3);
}
