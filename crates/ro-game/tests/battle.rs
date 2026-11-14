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
        t.lua("unequip_all()"); // a new character wears the Knife and the Cotton Shirt; these tests start bare
        t.cap.0.lock().unwrap().clear();
        t.g.skew(5100); // past the five seconds in which monsters ignore a newcomer
        t.check();
        t
    }

    /// Fail on any script error the server noted.
    fn check(&mut self) {
        self.g.tick(); // surfaces errors from read-only paths
        // a rule named a skill the database does not have
        let missing = self.g.console_eval("(function() local t = {} for k in pairs(SK_MISSING) do t[#t + 1] = k end return table.concat(t, ' ') end)()", self.me);
        assert!(missing.trim().is_empty(), "unknown skill names in the rules: {missing}");
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
        // guild skills are not reconstructed. Most 2-2 skills are not either, but
        // they must run without a script error all the same
        if inf == 0 || id > 10000 {
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
        // and once more the way a player uses it: the request (range, cast
        // time) and the cast end with its conditions (SP, weapon, items),
        // which may well refuse
        t.lua("heal(30000, 30000)");
        for call in [call.replace("rules.skill_castend", "rules.skill_use"), call.clone()] {
            let call = call.replace("item=true", "item=false");
            let r = t.g.console_eval(&call, me);
            let n = std::mem::take(&mut t.g.notices);
            assert!(!r.starts_with("error") && n.is_empty(), "{name} ({id}) as a player: {r} {:?}", n.first());
        }
        t.run(1000);
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
        let me = t.me;
        assert_eq!(t.packets(0x013e).iter().filter(|p| u32_at(p, 2) == me).count(), 1, "{skill}: cast bar");
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

/// Five seconds of peace after entering a map (mod0631 ghost_time), ended
/// by the first thing the player does.
#[test]
fn newcomers_are_left_alone() {
    let mut t = T::new();
    t.lua("player.base_level = 50; player.vit = 80; heal(9999, 9999); give_item(501, 1)");
    let hp = t.hp();
    t.lua("warp('testmap', 25, 25)");
    t.send(Writer::new(0x007d));
    let z = t.spawn(26, 25, 1015); // a Zombie on the next cell
    // (run() also lets real time pass, so stay well inside the five seconds)
    t.run(2500);
    assert_eq!(t.hp(), hp, "half-way in: the Zombie has not noticed");
    assert!(t.g.world.units[&z].mob().unwrap().target.is_none());
    t.run(4000);
    // (its blows may miss: look at what it is after, not at our HP)
    assert_eq!(t.g.world.units[&z].mob().unwrap().target, Some(t.me), "after five it turns on us");
    // again, but drinking a potion ends the peace at once
    t.lua(&format!("_rt('mob_kill', {z}); heal(9999, 0); warp('testmap', 25, 25)"));
    t.send(Writer::new(0x007d));
    let z = t.spawn(26, 25, 1015);
    t.run(200);
    assert!(t.g.world.units[&z].mob().unwrap().target.is_none());
    let mut w = Writer::new(0x00a7);
    w.u16(2).u32(t.me);
    t.send(w);
    t.run(1000);
    assert!(t.g.world.units[&z].mob().unwrap().target.is_some(), "the potion gave us away");
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

/// A job change with less SP than the old job's brings SP down to the new
/// maximum at once (mod0631 pc_jobchange -> pc_calcstatus), and the recovery
/// tick never subtracts below what it had (it panicked on an SP above its
/// maximum in the client rig, 2026-11-13).
#[test]
fn recovery_with_sp_above_max() {
    let mut t = T::new();
    t.lua("job_change(9); player.base_level = 70; player.int = 90; heal(9999, 9999)");
    let high = t.st("max_sp");
    t.lua("job_change(1)"); // a Swordman: less SP than a Wizard, more HP to recover
    let low = t.st("max_sp");
    assert!(low < high);
    assert_eq!(t.sp() as i64, low, "SP comes down with the job change");
    t.run(15_000);
    assert_eq!(t.sp() as i64, low);
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

/// The monsters' own skills are wired to their names (they were not: the
/// skill table's comments name them in free text).
#[test]
fn monster_skill_names_resolve() {
    let mut t = T::new();
    for name in ["NPC_COMBOATTACK", "NPC_FIREATTACK", "NPC_RANDOMATTACK"] {
        assert_eq!(t.lua(&format!("battle.weapon_skill[SK.{name}] ~= nil")).trim(), "true", "{name}");
    }
    assert_eq!(t.lua("SK.NPC_POISON").trim(), "176");
    assert_eq!(t.lua("SK.NPC_SUMMONSLAVE").trim(), "196");
    // a three-hit combo attack shows three hits
    t.lua("player.base_level = 50; player.vit = 1; heal(9999, 9999)");
    let s = t.spawn(26, 25, 1001);
    let me = t.me;
    t.lua(&format!("skill_castend_damage_id({s}, {me}, SK.NPC_COMBOATTACK, 3)"));
    let hit = t.packets(0x01de);
    assert_eq!(hit.len(), 1);
    assert_eq!(u16_at(&hit[0], 30), skill_num(&mut t, "NPC_COMBOATTACK", 3), "hit count from the skill table");
}

fn skill_num(t: &mut T, name: &str, lv: u16) -> u16 {
    t.lua(&format!("skill_get_num(SK.{name}, {lv})")).trim().parse().unwrap()
}

/// Dying: no recovery while dead, and on returning to the save point 1% of
/// the EXP held is gone and the character has 1 HP (pc_setrestartvalue).
#[test]
fn death_and_return() {
    let mut t = T::new();
    t.lua("job_change(1); player.base_level = 20; player.base_exp = 10000; player.job_exp = 5000; heal(9999, 9999)");
    let me = t.me;
    t.lua(&format!("_rt('bl_damage', 0, {me}, 99999)"));
    assert_eq!(t.hp(), 0);
    assert_eq!(t.g.world.units[&me].state, 1, "dead");
    assert_eq!(t.g.store.chars[&t.cid].base_exp, 10000, "nothing lost yet");
    t.run(60_000);
    assert_eq!(t.hp(), 0, "the dead do not recover");
    let mut w = Writer::new(0x00b2);
    w.u8(0);
    t.send(w);
    t.send(Writer::new(0x007d));
    let c = &t.g.store.chars[&t.cid];
    assert_eq!((c.hp, c.base_exp, c.job_exp), (1, 9900, 4950));
    // a minute of death is not paid out in one go afterwards
    t.run(600);
    assert!(t.hp() <= 1 + 20, "regen burst after respawn: {} HP", t.hp());
    // a Novice comes back with half its HP and loses nothing
    t.lua("job_change(0); player.base_exp = 10000");
    t.lua(&format!("_rt('bl_damage', 0, {me}, 99999)"));
    let mut w = Writer::new(0x00b2);
    w.u8(0);
    t.send(w);
    t.send(Writer::new(0x007d));
    let max = t.st("max_hp") as u32;
    let c = &t.g.store.chars[&t.cid];
    assert_eq!((c.hp, c.base_exp), (max / 2, 10000));
}

/// Stone Curse (eAthena 817): five seconds of turning to stone, during which
/// the victim still acts; then stone until the time is up.
#[test]
fn stone_sets_then_wears_off() {
    let mut t = T::new();
    let mob = t.spawn(27, 25, 1002);
    t.lua(&format!("skill_status_change_start({mob}, SC.STONE, 1, 0, {{ tick = 12000 }})"));
    assert_eq!(t.lua(&format!("battle.get({mob}).opt1")).trim(), "6", "still turning (817's opt1 6)");
    assert!(t.packets(0x0119).is_empty(), "nothing drawn yet");
    t.run(5500);
    assert_eq!(t.lua(&format!("battle.get({mob}).opt1")).trim(), "1", "stone");
    assert!(t.packets(0x0119).iter().any(|p| u16_at(p, 6) == 1), "the client is shown the stone when it sets");
    t.run(13_000);
    assert_eq!(t.lua(&format!("battle.sc(battle.get({mob}), SC.STONE) and 1 or 0")).trim(), "0", "worn off");
}

/// EXP levels a character up: base level with status points, job level
/// with a skill point (the tables of data/db/exp.toml must have loaded).
#[test]
fn exp_levels_up() {
    let mut t = T::new();
    assert!(t.g.db.exp.base.len() >= 98 && t.g.db.exp.base[0] == 9, "base EXP table: {:?}", &t.g.db.exp.base.iter().take(3).collect::<Vec<_>>());
    assert_eq!(t.g.db.exp.job_novice[0], 10);
    let c = &t.g.store.chars[&t.cid];
    let (sp0, kp0) = (c.status_point, c.skill_point);
    // 9 base EXP for level 2, 10 job EXP for job level 2
    t.lua("give_exp(9, 10)");
    let c = &t.g.store.chars[&t.cid];
    assert_eq!((c.base_level, c.job_level), (2, 2));
    assert_eq!(c.status_point, sp0 + (2 + 14) / 5);
    assert_eq!(c.skill_point, kp0 + 1);
    assert!(!t.packets(0x019b).is_empty(), "level-up effect");
    // a base level heals fully (pc.c:2531)
    t.lua("player.hp = 1");
    t.lua("give_exp(16, 0)");
    let max = t.st("max_hp") as u32;
    assert_eq!((t.g.store.chars[&t.cid].base_level, t.hp()), (3, max));
    // the dead gain nothing
    let me = t.me;
    t.lua(&format!("_rt('bl_damage', 0, {me}, 99999); give_exp(1000, 1000)"));
    assert_eq!(t.g.store.chars[&t.cid].base_level, 3);
    let mut w = Writer::new(0x00b2);
    w.u8(0);
    t.send(w);
    t.send(Writer::new(0x007d));
    // a Poring's 2 EXP (mobs.toml) by killing it
    let before = t.g.store.chars[&t.cid].base_exp;
    let mob = t.spawn(26, 25, 1002);
    let me = t.me;
    t.lua(&format!("_rt('bl_damage', {me}, {mob}, 9999)"));
    assert_eq!(t.g.store.chars[&t.cid].base_exp, before + 2);
}

impl T {
    /// Is this status change on the unit (monster or player)?
    fn has_sc(&mut self, id: u32, sc: &str) -> bool {
        self.g.world.units.contains_key(&id) && self.lua(&format!("battle.sc(battle.get({id}), SC.{sc}) and 1 or 0")).trim() == "1"
    }
    fn pos(&self, id: u32) -> (u16, u16) {
        let u = &self.g.world.units[&id];
        (u.x, u.y)
    }
    fn skill_id(&self, name: &str) -> u16 {
        self.g.db.skills_by_name[name]
    }
    /// Use a learnt skill by name on a unit, at its learnt level.
    fn cast(&mut self, name: &str, target: u32) {
        let id = self.skill_id(name);
        let lv = self.g.store.chars[&self.cid].skills.get(&id).copied().unwrap_or(1) as u16;
        self.skill(id, lv, target);
    }
    fn cast_at(&mut self, name: &str, x: u16, y: u16) {
        let id = self.skill_id(name);
        let lv = self.g.store.chars[&self.cid].skills.get(&id).copied().unwrap_or(1) as u16;
        self.skill_pos(id, lv, x, y);
    }
    fn kill(&mut self, mob: u32) {
        if self.g.world.units.contains_key(&mob) {
            self.lua(&format!("_rt('mob_kill', {mob})"));
        }
    }
}

/// Thief: Hiding costs SP over time and shakes monsters off, Envenom
/// poisons and the poison bites, Detoxify cures, Steal takes a drop once.
#[test]
fn thief_skills() {
    let mut t = T::new();
    t.lua("job_change(6); player.base_level = 60; player.dex = 99; player.str = 50; player.vit = 50; heal(9999, 9999)");
    for (s, lv) in [("TF_HIDING", 10), ("TF_POISON", 10), ("TF_DETOXIFY", 1), ("TF_STEAL", 10)] {
        t.learn(s, lv);
    }
    let me = t.me;
    // a Zombie comes for us; hidden, it lets go
    let z = t.spawn(28, 25, 1015);
    t.run(3000);
    assert_eq!(t.g.world.units[&z].mob().unwrap().target, Some(me));
    let sp = t.sp();
    t.cast("TF_HIDING", me);
    assert!(t.sc(4), "hidden");
    assert_eq!(t.sp(), sp - 10);
    assert!(t.packets(0x0119).iter().any(|p| u16_at(p, 10) & 2 != 0), "the client is told (option 2)");
    // level 10: one SP every 13 s
    let sp = t.sp();
    t.run(27_000);
    // (the test clock is the real one skewed: under load a third tick can fall in)
    assert!((2..=3).contains(&(sp - t.sp())), "SP went from {sp} to {}", t.sp());
    // (a monster that just used a skill rests one attack delay before it looks again)
    assert_eq!(t.g.world.units[&z].mob().unwrap().target, None, "the Zombie lost us");
    // no walking while hidden
    let mut w = Writer::new(0x0085);
    w.pos3(20, 25, 0);
    t.send(w);
    t.run(600);
    assert_eq!(t.pos(me), (25, 25));
    // out again
    t.run(3000);
    t.cast("TF_HIDING", me);
    assert!(!t.sc(4));
    t.kill(z);
    // Envenom until it poisons (10 + 2/level %, less the Pupa's VIT)
    let mut p = t.spawn(26, 25, 1008);
    let mut poisoned = false;
    for _ in 0..60 {
        if t.mob_hp(p).is_none() {
            p = t.spawn(26, 25, 1008);
        }
        t.lua("heal(0, 9999)");
        t.cast("TF_POISON", p);
        t.run(300);
        if t.has_sc(p, "POISON") {
            poisoned = true;
            break;
        }
    }
    assert!(poisoned, "Envenom poisons");
    let hp = t.mob_hp(p).unwrap();
    t.run(5000);
    assert!(t.mob_hp(p).unwrap() < hp, "poison takes HP");
    t.cast("TF_DETOXIFY", p);
    t.run(300);
    assert!(!t.has_sc(p, "POISON"), "Detoxify cures");
    t.kill(p);
    // Poison React answers one poisonous blow (an Envenom thrown back at us), once
    t.lua("job_change(12)");
    t.learn("AS_POISONREACT", 5);
    let me = t.me;
    let m = t.spawn(26, 25, 1008); // a Pupa (the answer is of the Poison element: not an Undead)
    t.cast("AS_POISONREACT", me);
    t.run(500);
    assert!(t.sc(7), "Poison React is up");
    let hp = t.mob_hp(m).unwrap();
    t.lua(&format!("rules.weapon_attack{{src={m}, target={me}}}"));
    assert!(t.sc(7), "a plain blow does not set it off");
    assert_eq!(t.mob_hp(m), Some(hp));
    t.lua(&format!("skill_attack(BF.WEAPON, {m}, {m}, {me}, SK.TF_POISON, 1, 0)"));
    assert!(!t.sc(7), "a poisonous one does, once");
    assert!(t.mob_hp(m).is_none_or(|h| h < hp), "and is answered");
    t.kill(m);
    t.lua("heal(9999, 9999)");
    t.run(2000);
    // Steal from a Poring: sooner or later an item, and then never again
    let before: u32 = t.g.store.chars[&t.cid].inventory.iter().map(|i| i.amount).sum();
    let p = t.spawn(26, 25, 1002);
    let mut got = false;
    for _ in 0..200 {
        t.lua("heal(0, 9999)");
        t.cast("TF_STEAL", p);
        t.run(200);
        let now: u32 = t.g.store.chars[&t.cid].inventory.iter().map(|i| i.amount).sum();
        if now > before {
            got = true;
            break;
        }
    }
    assert!(got, "stole something");
    let after: u32 = t.g.store.chars[&t.cid].inventory.iter().map(|i| i.amount).sum();
    for _ in 0..50 {
        t.lua("heal(0, 9999)");
        t.cast("TF_STEAL", p);
        t.run(200);
    }
    let end: u32 = t.g.store.chars[&t.cid].inventory.iter().map(|i| i.amount).sum();
    assert_eq!(end, after, "a monster is robbed once");
}

/// Acolyte and Priest: Teleport, Warp Portal with a memo point, Pneuma,
/// Sanctuary, Kyrie Eleison.
#[test]
fn acolyte_skills() {
    let mut t = T::new();
    t.lua("job_change(8); player.base_level = 70; player.int = 80; player.dex = 99; heal(9999, 9999); give_item(717, 20)");
    for (s, lv) in [("AL_TELEPORT", 2), ("AL_WARP", 4), ("AL_PNEUMA", 1), ("PR_SANCTUARY", 5), ("PR_KYRIE", 10), ("AL_HEAL", 10)] {
        t.learn(s, lv);
    }
    let me = t.me;
    // Teleport 2 offers Random and the save point
    t.skill(26, 2, me);
    let menu = t.packets(0x011c);
    assert_eq!(menu.len(), 1);
    assert!(menu[0][4..10].starts_with(b"Random"));
    let mut w = Writer::new(0x011b);
    w.u16(26).str_fixed("Random", 16);
    t.send(w);
    assert_ne!(t.pos(me), (25, 25), "teleported");
    t.lua("warp('testmap', 25, 25)");
    t.run(1500);
    // /memo here, walk off, open a portal back
    t.send(Writer::new(0x011d));
    assert_eq!(t.packets(0x011e)[0][2], 0, "memo saved");
    assert_eq!(t.g.store.chars[&t.cid].memo.len(), 1);
    t.lua("warp('watermap', 10, 10)");
    t.send(Writer::new(0x007d));
    t.run(500);
    t.cast_at("AL_WARP", 12, 10);
    t.run(1500);
    let menu = t.packets(0x011c);
    assert_eq!(menu.len(), 1, "portal destinations");
    let names = String::from_utf8_lossy(&menu[0][4..]).to_string();
    assert!(names.contains("testmap.gat"), "{names}");
    let mut w = Writer::new(0x011b);
    w.u16(27).str_fixed("testmap.gat", 16);
    t.send(w);
    assert!(t.packets(0x01c9).iter().any(|p| p[14] == 0x81), "the portal is opening");
    t.run(2500);
    assert!(t.packets(0x01c9).iter().any(|p| p[14] == 0x80), "the portal is open");
    // step in
    let mut w = Writer::new(0x0085);
    w.pos3(12, 10, 0);
    t.send(w);
    t.run(1500);
    assert_eq!(t.g.world.units[&me].map, "testmap", "the portal took us back");
    t.send(Writer::new(0x007d));
    assert_eq!(t.pos(me), (25, 25));
    // Sanctuary heals who stands in it
    t.lua("player.hp = 10");
    t.cast_at("PR_SANCTUARY", 25, 25);
    t.run(8000);
    assert!(t.hp() > 400, "Sanctuary healed to {}", t.hp());
    t.run(20_000);
    // Kyrie Eleison takes blows: level 10 = 10 hits or 30% of max HP
    t.lua("heal(9999, 9999)");
    t.cast("PR_KYRIE", me);
    t.run(2500);
    assert!(t.sc(19));
    let hp = t.hp();
    let z = t.spawn(26, 25, 1015);
    t.lua(&format!("for i = 1, 4 do rules.weapon_attack{{src={z}, target={me}}} end"));
    assert_eq!(t.hp(), hp, "Kyrie holds");
    t.lua(&format!("for i = 1, 60 do rules.weapon_attack{{src={z}, target={me}}} end"));
    assert!(!t.sc(19), "Kyrie is spent");
    assert!(t.hp() < hp);
}

/// Swordman, Knight, Merchant: Magnum Break around the swordman, Endure,
/// Auto Berserk, Pierce by size, Mammonite's zeny.
#[test]
fn fighter_skills() {
    let mut t = T::new();
    t.lua("job_change(7); player.base_level = 70; player.str = 80; player.dex = 80; player.vit = 60; heal(9999, 9999)");
    for (s, lv) in [("SM_MAGNUM", 10), ("SM_ENDURE", 10), ("SM_AUTOBERSERK", 1), ("KN_PIERCE", 10), ("SM_BASH", 10), ("SM_PROVOKE", 10)] {
        t.learn(s, lv);
    }
    let me = t.me;
    // Magnum Break: the target and those within a cell of us, thrown back two cells
    let a = t.spawn(26, 25, 1008);
    let b = t.spawn(24, 26, 1008);
    let far = t.spawn(30, 25, 1008);
    t.cast("SM_MAGNUM", a);
    assert!(t.mob_hp(a).unwrap_or(0) < 427 && t.mob_hp(b).unwrap_or(0) < 427, "both next to us are hit");
    assert_eq!(t.mob_hp(far), Some(427), "not the one five cells off");
    for m in [a, b, far] {
        t.kill(m);
    }
    // Pierce: one hit on small, three on large (Baphomet Jr is small; an Orc Lady medium)
    t.wear(1401); // Javelin
    t.run(2500);
    let small = t.spawn(26, 25, 1008); // Pupa: small
    t.cast("KN_PIERCE", small);
    let hits = t.packets(0x01de);
    assert_eq!(u16_at(&hits[0], 30), 1, "Pierce hits a small monster once");
    t.kill(small);
    let medium = t.spawn(26, 25, 1015); // Zombie: medium
    t.cast("KN_PIERCE", medium);
    let hits = t.packets(0x01de);
    assert_eq!(u16_at(&hits[0], 30), 2, "twice a medium one");
    t.kill(medium);
    // Auto Berserk: under a quarter of the HP, Provoke 10 on ourselves
    let max = t.st("max_hp");
    t.lua(&format!("_rt('bl_damage', 0, {me}, {})", max * 8 / 10));
    assert!(t.sc(0), "berserk");
    t.lua("heal(9999, 0)");
    t.run(2500);
    assert!(!t.sc(0), "calm again above a quarter");
    // Mammonite: 100 zeny a level, refused without
    t.lua("job_change(5); player.zeny = 1500");
    t.learn("MC_MAMMONITE", 10);
    let m = t.spawn(26, 25, 1008);
    t.cast("MC_MAMMONITE", m);
    assert_eq!(t.g.store.chars[&t.cid].zeny, 500);
    t.run(2500);
    t.packets(0);
    let m = t.spawn(26, 25, 1008);
    t.cast("MC_MAMMONITE", m);
    assert_eq!(t.g.store.chars[&t.cid].zeny, 500, "not enough zeny");
    assert!(t.packets(0x0110).iter().any(|p| p[9] == 5), "the client is told why");
}

/// Ankle Snare stays while other monsters move near it: only the caught one
/// walking off ends it (jAthena //1046; mod0631 and 817 ended it on anyone's
/// step within 5 cells, which in the client took a snare away in a second).
#[test]
fn ankle_snare_outlasts_passers_by() {
    let mut t = T::new();
    t.lua("job_change(11); player.base_level = 70; player.dex = 90; heal(9999, 9999); give_item(1065, 10)");
    t.learn("HT_ANKLESNARE", 5);
    let snares = |t: &T| t.g.world.units.values().filter(|u| u.skill().is_some() && (u.x, u.y) == (28, 25)).count();
    t.cast_at("HT_ANKLESNARE", 28, 25);
    t.run(1000);
    assert_eq!(snares(&t), 1, "set");
    // a Zombie walks about 3 cells off the trap without stepping on it
    let passer = t.spawn(31, 28, 1002);
    t.lua(&format!("_rt('walk_to', {passer}, 25, 28)"));
    t.run(4000);
    assert_eq!(snares(&t), 1, "a monster walking by does not take an unsprung snare away");
    t.kill(passer);
    let z = t.spawn(32, 25, 1015);
    t.run(5000);
    assert!(t.has_sc(z, "ANKLE"), "snared");
    let passer = t.spawn(31, 29, 1002);
    t.lua(&format!("_rt('walk_to', {passer}, 25, 29)"));
    t.run(4000);
    assert!(t.has_sc(z, "ANKLE"), "still snared while another walks by");
    assert_eq!(snares(&t), 1, "and the trap is still there");
}

/// Sightrasher: only under Sight, which it ends; burns and pushes back what
/// stands on the eight lines around the wizard, not what stands between them.
#[test]
fn sightrasher() {
    let mut t = T::new();
    t.lua("job_change(9); player.base_level = 70; player.int = 80; player.dex = 99; heal(9999, 9999)");
    t.learn("MG_SIGHT", 1);
    t.learn("WZ_SIGHTRASHER", 10);
    let me = t.me;
    let on_line = t.spawn(28, 25, 1008); // Pupa, Earth: fire hurts; does not move
    let off_line = t.spawn(27, 28, 1008); // 2 east, 3 north: off the lines
    let (hp_on, hp_off) = (t.mob_hp(on_line).unwrap(), t.mob_hp(off_line).unwrap());
    t.cast("WZ_SIGHTRASHER", me);
    t.run(1500);
    assert_eq!(t.mob_hp(on_line), Some(hp_on), "nothing without Sight");
    assert!(t.packets(0x0110).iter().any(|p| u16::from_le_bytes([p[2], p[3]]) == 81), "the client is told it failed");
    t.run(2500);
    t.cast("MG_SIGHT", me);
    t.run(1500);
    assert!(t.has_sc(me, "SIGHT"));
    t.cast("WZ_SIGHTRASHER", me);
    t.run(1500);
    assert!(t.mob_hp(on_line).is_none_or(|h| h < hp_on), "the Pupa on the line burns");
    if t.g.world.units.contains_key(&on_line) {
        assert!(t.pos(on_line).0 > 28, "and is pushed back: {:?}", t.pos(on_line));
    }
    assert_eq!(t.mob_hp(off_line), Some(hp_off), "the one off the lines does not");
    assert!(!t.has_sc(me, "SIGHT"), "Sight is spent");
}

/// The client asks for a skill from one cell beyond its range (Remove Trap,
/// range 2, from 3 cells) and the server lets it through (jAthena //1082);
/// two cells beyond is still too far.
#[test]
fn skill_range_one_cell_more() {
    let mut t = T::new();
    t.lua("job_change(11); player.base_level = 70; player.dex = 90; heal(9999, 9999); give_item(1065, 10)");
    for (s, lv) in [("HT_LANDMINE", 5), ("HT_REMOVETRAP", 1)] {
        t.learn(s, lv);
    }
    let at = |t: &T, x: u16, y: u16| -> Vec<u32> { t.g.world.units.values().filter(|u| u.skill().is_some() && (u.x, u.y) == (x, y)).map(|u| u.id).collect() };
    t.cast_at("HT_LANDMINE", 28, 22);
    t.run(2000);
    t.cast_at("HT_LANDMINE", 21, 27);
    t.run(2000);
    let (three, four) = (at(&t, 28, 22)[0], at(&t, 21, 27)[0]);
    t.cast("HT_REMOVETRAP", four);
    t.run(1000);
    assert_eq!(at(&t, 21, 27).len(), 1, "four cells is too far");
    t.cast("HT_REMOVETRAP", three);
    t.run(1000);
    assert!(at(&t, 28, 22).is_empty(), "three cells is near enough");
}

/// Hunter: traps wait for a monster; Land Mine goes off, Ankle Snare holds.
#[test]
fn hunter_traps() {
    let mut t = T::new();
    t.lua("job_change(11); player.base_level = 70; player.dex = 90; player.int = 40; heal(9999, 9999); give_item(1065, 10)");
    for (s, lv) in [("HT_LANDMINE", 5), ("HT_ANKLESNARE", 5), ("HT_BLITZBEAT", 5), ("HT_FALCON", 1), ("HT_REMOVETRAP", 1), ("HT_SPRINGTRAP", 5)] {
        t.learn(s, lv);
    }
    let me = t.me;
    t.cast_at("HT_LANDMINE", 27, 25);
    assert!(t.packets(0x01c9).iter().any(|p| p[14] == 0x93), "a Land Mine is set");
    assert_eq!(t.g.store.chars[&t.cid].inventory.iter().find(|i| i.nameid == 1065).map(|i| i.amount), Some(9), "one Trap used");
    // a Zombie walks at us over the mine
    let z = t.spawn(31, 25, 1015);
    let hp = t.mob_hp(z).unwrap();
    t.run(6000);
    assert!(t.mob_hp(z).is_none_or(|h| h < hp), "the mine went off under it");
    assert!(!t.g.world.units.values().any(|u| u.skill().is_some()), "and is gone");
    t.kill(z);
    t.lua("heal(9999, 9999)");
    // Ankle Snare holds the next one where it stepped
    t.cast_at("HT_ANKLESNARE", 28, 25);
    let z = t.spawn(32, 25, 1015);
    t.run(5000);
    assert!(t.has_sc(z, "ANKLE"), "snared");
    let at = t.pos(z);
    t.run(3000);
    assert_eq!(t.pos(z), at, "it does not move");
    assert!((at.0 as i32 - 28).abs() <= 1, "caught at the trap: {at:?}");
    // Remove Trap takes a trap up again and gives the Trap back; Spring Trap needs the falcon
    let traps = |t: &T| t.g.store.chars[&t.cid].inventory.iter().find(|i| i.nameid == 1065).map(|i| i.amount).unwrap_or(0);
    let trap_cells = |t: &T| -> Vec<u32> { t.g.world.units.values().filter(|u| u.skill().is_some() && (u.x, u.y) == (23, 25)).map(|u| u.id).collect() };
    t.run(2000);
    t.cast_at("HT_LANDMINE", 23, 25);
    let n = traps(&t);
    let cell = trap_cells(&t)[0];
    t.run(2000);
    t.cast("HT_REMOVETRAP", cell);
    assert!(trap_cells(&t).is_empty(), "the trap is taken up");
    assert_eq!(traps(&t), n + 1, "and the Trap is back");
    t.run(2000);
    t.cast_at("HT_LANDMINE", 23, 25);
    let cell = trap_cells(&t)[0];
    t.run(2000);
    t.cast("HT_SPRINGTRAP", cell);
    assert_eq!(trap_cells(&t).len(), 1, "no falcon, no Spring Trap");
    // Blitz Beat needs the falcon
    t.packets(0);
    t.cast("HT_BLITZBEAT", z);
    assert!(t.packets(0x01de).is_empty(), "no falcon, no Blitz Beat");
    t.lua("set_falcon()");
    t.run(2000);
    let hp = t.mob_hp(z).unwrap();
    t.cast("HT_BLITZBEAT", z);
    t.run(2000);
    assert!(t.mob_hp(z).is_none_or(|h| h < hp), "the falcon strikes");
    t.run(2000);
    let n = traps(&t);
    t.cast("HT_SPRINGTRAP", cell);
    assert!(trap_cells(&t).is_empty(), "the falcon springs the trap");
    assert_eq!(traps(&t), n, "nothing comes back");
    let _ = me;
}

/// Sandman: the one who springs it and those on the cells around may fall
/// asleep (the iRO guide's "an enemy and near enemies"), nobody further off.
#[test]
fn sandman_reaches_the_neighbours() {
    let mut t = T::new();
    t.lua("job_change(11); player.base_level = 70; player.dex = 90; heal(9999, 9999); give_item(1065, 10)");
    t.learn("HT_SANDMAN", 5);
    // Ant Eggs do not move: eight around the trap's cell, three further off;
    // a Zombie walks at us over the trap
    let near: Vec<u32> = [(27, 24), (28, 24), (29, 24), (27, 26), (28, 26), (29, 26), (27, 25), (29, 25)].iter().map(|(x, y)| t.spawn(*x, *y, 1097)).collect();
    let far: Vec<u32> = (0..3).map(|i| t.spawn(32, 22 + i, 1097)).collect();
    t.cast_at("HT_SANDMAN", 28, 25);
    assert!(t.packets(0x01c9).iter().any(|p| p[14] == 0x95), "a Sandman is set");
    let z = t.spawn(32, 25, 1015);
    t.run(6000);
    assert!(!t.g.world.units.values().any(|u| u.skill().is_some()), "the trap went off");
    let asleep = |t: &mut T, ids: &[u32]| ids.iter().filter(|id| t.has_sc(**id, "SLEEP")).count();
    let n = asleep(&mut t, &near);
    assert!(n >= 1, "one of the eight around it sleeps (55% each): {n}");
    assert_eq!(asleep(&mut t, &far), 0, "three cells away nobody does");
    let _ = z;
}

/// A monster with a long attack delay (a Zombie: 2612 ms) keeps to it, and
/// a living player cannot "return to the save point".
#[test]
fn slow_attackers_keep_their_delay() {
    let mut t = T::new();
    t.lua("player.base_level = 90; player.vit = 90; heal(9999, 9999)");
    let me = t.me;
    let z = t.spawn(26, 25, 1015);
    t.lua(&format!("_rt('mob_target', {z}, {me}, 0)"));
    t.run(1000);
    t.packets(0);
    t.run(13_000);
    let blows = t.packets(0x008a).iter().filter(|p| u32_at(p, 2) == z).count();
    // (fewer when it uses a skill instead: that costs it a delay as well)
    assert!((1..=6).contains(&blows), "{blows} blows in 13 s at 2.6 s a blow");
    // alive: restart type 0 does nothing
    let at = t.pos(me);
    let mut w = Writer::new(0x00b2);
    w.u8(0);
    t.send(w);
    assert_eq!(t.pos(me), at);
    assert_eq!(t.g.world.units[&me].map, "testmap");
}

/// Numbers sharpened from the iRO guide's skill pages of spring 2004
/// (research/THREADS.md #22): Poison React lasts 30 s, the falcon's reach
/// for Spring Trap grows with the level, a trap that has gone off cannot be
/// taken up, Brandish Spear also hits beside the knight and pushes two cells.
#[test]
fn guide_numbers() {
    let mut t = T::new();
    let me = t.me;
    // Poison React: 30 s (817's table says half an hour)
    t.lua("job_change(12); player.base_level = 70; heal(9999, 9999)");
    t.learn("AS_POISONREACT", 5);
    t.cast("AS_POISONREACT", me);
    t.run(20000);
    assert!(t.sc(7), "Poison React still up after 20 s");
    t.run(15000);
    assert!(!t.sc(7), "and gone after 35 s");

    // Spring Trap: 4 cells at level 1, 8 at level 5
    t.lua("job_change(11); player.dex = 90; heal(9999, 9999); give_item(1065, 10)");
    for (s, lv) in [("HT_LANDMINE", 5), ("HT_FALCON", 1), ("HT_REMOVETRAP", 1), ("HT_SPRINGTRAP", 1), ("HT_ANKLESNARE", 5)] {
        t.learn(s, lv);
    }
    t.lua("set_falcon()");
    let cells_at = |t: &T, x: u16, y: u16| -> Vec<u32> { t.g.world.units.values().filter(|u| u.skill().is_some() && (u.x, u.y) == (x, y)).map(|u| u.id).collect() };
    t.cast_at("HT_LANDMINE", 27, 25);
    t.run(2000);
    let cell = cells_at(&t, 27, 25)[0];
    t.lua("warp('testmap', 20, 25)");
    t.run(1000);
    t.cast("HT_SPRINGTRAP", cell);
    t.run(1000);
    assert_eq!(cells_at(&t, 27, 25).len(), 1, "seven cells is too far for level 1");
    t.learn("HT_SPRINGTRAP", 5);
    t.cast("HT_SPRINGTRAP", cell);
    t.run(1000);
    assert!(cells_at(&t, 27, 25).is_empty(), "and within reach at level 5");

    // Remove Trap: not once the trap has caught something
    t.lua("warp('testmap', 25, 25); heal(9999, 9999)");
    t.run(2000);
    t.cast_at("HT_ANKLESNARE", 27, 25);
    t.run(1000);
    let z = t.spawn(27, 25, 1015);
    t.run(1500);
    assert!(t.has_sc(z, "ANKLE"), "snared");
    let cell = cells_at(&t, 27, 25)[0];
    let traps = t.inv(1065);
    t.cast("HT_REMOVETRAP", cell);
    t.run(500);
    assert_eq!(cells_at(&t, 27, 25).len(), 1, "a sprung trap stays");
    assert_eq!(t.inv(1065), traps, "and no Trap comes back");
    t.kill(z);
    t.run(25000);

    // Brandish Spear: the cell beside the knight is in area 1; the target is pushed two cells
    t.lua("job_change(7); player.str = 60; heal(9999, 9999)");
    for (s, lv) in [("KN_RIDING", 1), ("KN_SPEARMASTERY", 10), ("KN_BRANDISHSPEAR", 1)] {
        t.learn(s, lv);
    }
    t.lua("set_riding()");
    t.wear(1401); // a Javelin
    let front = t.spawn(26, 25, 1015);
    let beside = t.spawn(25, 26, 1015);
    let behind = t.spawn(24, 25, 1015);
    t.lua(&format!("for _, id in ipairs{{{front}, {beside}, {behind}}} do _rt('bl_set', id, 'hp', 500000) end"));
    let hp = |t: &T, id: u32| t.mob_hp(id).unwrap();
    let (a, b, c) = (hp(&t, front), hp(&t, beside), hp(&t, behind));
    t.cast("KN_BRANDISHSPEAR", front);
    // the Zombie walks back at once: note how far it was thrown
    let mut far = 0;
    for _ in 0..30 {
        t.run(100);
        far = far.max(t.pos(front).0);
    }
    assert!(hp(&t, front) < a, "the target is hit");
    assert!(hp(&t, beside) < b, "and the one beside the knight");
    assert_eq!(hp(&t, behind), c, "not the one behind");
    assert_eq!(far, 28, "pushed two cells");
    t.check();
}

/// The Crusader's shield skills, the first of the 2-2 jobs' to be drafted:
/// Guard and Defender as eAthena 817 has them (the iRO guide's tables),
/// Shield Reflect from the guide alone, the shield's own weight in a
/// Shield Boomerang. All of them want a shield.
#[test]
fn crusader_shield_skills() {
    let mut t = T::new();
    t.lua("job_change(14); player.base_level = 80; player.str = 80; player.dex = 80; player.vit = 50; heal(30000, 30000)");
    for (s, lv) in [("CR_AUTOGUARD", 10), ("CR_DEFENDER", 5), ("CR_REFLECTSHIELD", 10), ("CR_SHIELDCHARGE", 5), ("CR_SHIELDBOOMERANG", 5)] {
        t.learn(s, lv);
    }
    let me = t.me;
    let calc = |t: &mut T, flags: &str| -> i64 {
        t.lua(&format!("battle.calc_damage(battle.get({me}), 1000, {flags})")).trim().parse::<f64>().unwrap() as i64
    };
    // no shield, no Guard
    t.cast("CR_AUTOGUARD", me);
    t.run(1500);
    assert!(!t.sc(58));
    t.wear(2101); // a Guard: 30 of weight
    assert_eq!(t.st("shield_atk"), 30);

    // Guard 10 costs 30 SP and stops three weapon hits in ten; magic passes
    let sp = t.sp();
    t.cast("CR_AUTOGUARD", me);
    t.run(1500);
    assert!(t.sc(58));
    assert_eq!(sp - t.sp(), 30);
    let blocked = (0..1000).filter(|_| calc(&mut t, "BF.WEAPON | BF.SHORT") == 0).count();
    assert!((230..370).contains(&blocked), "{blocked} of 1000 blocked at 30%");
    assert_eq!(calc(&mut t, "BF.MAGIC | BF.LONG"), 1000);
    // off again, for nothing
    let sp = t.sp();
    t.cast("CR_AUTOGUARD", me);
    t.run(1500);
    assert!(!t.sc(58));
    assert_eq!(t.sp(), sp);

    // Defender 5: four fifths off a ranged weapon hit, nothing off a close one; slower hands and feet
    let (aspd, speed) = (t.st("aspd"), t.st("speed"));
    t.cast("CR_DEFENDER", me);
    t.run(1500);
    assert!(t.sc(62));
    assert_eq!(calc(&mut t, "BF.WEAPON | BF.LONG"), 200);
    assert_eq!(calc(&mut t, "BF.WEAPON | BF.SHORT"), 1000);
    assert_eq!(calc(&mut t, "BF.MAGIC | BF.LONG"), 1000);
    assert_eq!(t.st("aspd"), aspd * 110 / 100);
    assert_eq!(t.st("speed"), speed * 130 / 100);

    // Shield Reflect 10: two fifths of a close blow go back to who dealt it
    t.cast("CR_REFLECTSHIELD", me);
    t.run(1500);
    assert!(t.sc(59));
    let z = t.spawn(26, 25, 1015);
    let zhp = t.mob_hp(z).unwrap();
    t.lua(&format!("battle.reflect(battle.get({z}), battle.get({me}), 500, BF.WEAPON | BF.LONG)"));
    assert_eq!(t.mob_hp(z).unwrap(), zhp, "an arrow is not thrown back");
    t.lua(&format!("battle.reflect(battle.get({z}), battle.get({me}), 500, BF.WEAPON | BF.SHORT)"));
    assert_eq!(t.mob_hp(z).unwrap(), zhp - 200);
    t.kill(z);

    // a Shield Boomerang reaches eleven cells at level 5 (six in mod0631) and takes the Zombie's HP down
    let (x, y) = t.pos(me);
    let far = t.spawn(x + 10, y, 1015);
    let hp = t.mob_hp(far).unwrap();
    t.cast("CR_SHIELDBOOMERANG", far);
    t.run(1500);
    assert!(t.mob_hp(far).unwrap_or(0) < hp, "hit at ten cells");
    t.kill(far);

    // the shield comes off: all three stances end, and Smite has nothing to hit with
    let idx = t.g.store.chars[&t.cid].inventory.iter().position(|i| i.nameid == 2101 && i.equip != 0).unwrap() as u16 + 2;
    let mut w = Writer::new(0x00ab);
    w.u16(idx);
    t.send(w);
    t.run(200);
    assert!(!t.sc(62) && !t.sc(59));
    let near = t.spawn(26, 25, 1015);
    let hp = t.mob_hp(near).unwrap();
    t.cast("CR_SHIELDCHARGE", near);
    t.run(1500);
    assert_eq!(t.mob_hp(near).unwrap(), hp);
}

/// Drafts of 2-2 skills that need no packet this client lacks: Providence, the Sage's
/// endows and Dispell, the Rogue's Strips, the Alchemist's Chemical Protections.
#[test]
fn endows_strips_protections_dispell() {
    let mut t = T::new();
    t.lua("job_change(16); player.base_level = 80; player.int = 60; player.dex = 60; heal(30000, 30000)");
    for (s, lv) in [("CR_PROVIDENCE", 5), ("SA_FLAMELAUNCHER", 5), ("SA_DISPELL", 5), ("RG_STRIPARMOR", 5), ("RG_STRIPWEAPON", 5), ("RG_STRIPSHIELD", 5), ("RG_STRIPHELM", 5), ("AM_CP_ARMOR", 2)] {
        t.learn(s, lv);
    }
    t.lua("give_item(990, 3); give_item(715, 2); give_item(716, 2); give_item(7139, 2)");
    let me = t.me;
    let count = |t: &mut T, item: u32| -> i64 { t.lua(&format!("_rt('pc_item_count', {me}, {item})")).trim().parse::<f64>().unwrap() as i64 };
    let castend = |t: &mut T, skill: &str, lv: u8, target: u32| {
        t.lua(&format!("rules.skill_castend{{src={me}, skill=SK.{skill}, lv={lv}, target={target}, x=0, y=0, item=true, val=0}}"));
    };

    // Providence: three seconds of cast (817, the guide), then on
    t.cast("CR_PROVIDENCE", me);
    t.run(1500);
    assert!(!t.sc(61), "still casting");
    t.run(3000);
    assert!(t.sc(61));

    // Endow Blaze: nothing to endow on bare hands; on a Knife the blows turn to Fire
    castend(&mut t, "SA_FLAMELAUNCHER", 5, me);
    assert!(!t.sc(98));
    t.wear(1201);
    t.cast("SA_FLAMELAUNCHER", me);
    t.run(4500);
    assert!(t.sc(98), "level 5 does not fail");
    assert_eq!(count(&mut t, 990), 2, "one Red Blood");
    assert_eq!(t.lua(&format!("battle.attack_element(battle.get({me}))")).trim().parse::<f64>().unwrap() as i64, 3);
    // it is still on long after mod0631's minute and a half
    t.run(200_000);
    assert!(t.sc(98));
    // level 1 fails one time in five, and the Knife is gone
    let mut tries = 0;
    let knives = t.g.store.chars[&t.cid].inventory.iter().filter(|i| i.nameid == 1201).count();
    while t.g.store.chars[&t.cid].inventory.iter().any(|i| i.nameid == 1201 && i.equip != 0) {
        castend(&mut t, "SA_FLAMELAUNCHER", 1, me);
        tries += 1;
        assert!(tries < 200, "an endow of level 1 never failed");
    }
    assert_eq!(t.g.store.chars[&t.cid].inventory.iter().filter(|i| i.nameid == 1201).count(), knives - 1, "destroyed, not taken off");

    // Strips on a monster, for a minute: Weapon a tenth off its attack (the iRO guide),
    // Armor 40% off its VIT, Shield 15% off its DEF, Helm 40% off its INT (euRO's text)
    let egg = t.spawn(26, 25, 1048);
    let num = |t: &mut T, id: u32, f: &str| -> i64 { t.lua(&format!("battle.{f}(battle.get({id}))")).trim().parse::<f64>().unwrap() as i64 };
    let (def, atk2, vit, def2) = (num(&mut t, egg, "def"), num(&mut t, egg, "atk2"), num(&mut t, egg, "vit"), num(&mut t, egg, "def2"));
    assert!(def >= 10 && atk2 >= 10 && vit >= 3);
    let mut tries = 0;
    while !(t.has_sc(egg, "STRIPARMOR") && t.has_sc(egg, "STRIPWEAPON")) {
        castend(&mut t, "RG_STRIPARMOR", 5, egg);
        castend(&mut t, "RG_STRIPWEAPON", 5, egg);
        tries += 1;
        assert!(tries < 300, "a Strip of level 5 never took");
    }
    assert!(tries > 1 || !t.has_sc(egg, "STRIPSHIELD"));
    assert_eq!(num(&mut t, egg, "def"), def, "Strip Armor leaves DEF alone");
    assert_eq!(num(&mut t, egg, "vit"), vit - vit * 40 / 100);
    assert_eq!(num(&mut t, egg, "def2"), def2 - def2 * 40 / 100);
    assert_eq!(num(&mut t, egg, "atk2"), atk2 - atk2 / 10);
    let picky = t.spawn(27, 25, 1050); // a Picky with its shell: DEF 20, INT 10
    let (def, int, mdef2) = (num(&mut t, picky, "def"), num(&mut t, picky, "int"), num(&mut t, picky, "mdef2"));
    assert!(def >= 20 && int >= 10);
    let mut tries = 0;
    while !(t.has_sc(picky, "STRIPSHIELD") && t.has_sc(picky, "STRIPHELM")) {
        castend(&mut t, "RG_STRIPSHIELD", 5, picky);
        castend(&mut t, "RG_STRIPHELM", 5, picky);
        tries += 1;
        assert!(tries < 300, "a Strip of level 5 never took");
    }
    assert_eq!(num(&mut t, picky, "def"), def - def * 15 / 100);
    assert_eq!(num(&mut t, picky, "int"), int - int * 40 / 100);
    assert_eq!(num(&mut t, picky, "mdef2"), mdef2 - mdef2 * 40 / 100);

    // Dispell 5 always works: a Yellow and a Red Gemstone, and the monster's blessing is gone
    t.lua(&format!("skill_status_change_start({egg}, SC.INCREASEAGI, 5, 0)"));
    assert!(t.has_sc(egg, "INCREASEAGI"));
    t.cast("SA_DISPELL", egg);
    t.run(3500);
    assert!(!t.has_sc(egg, "INCREASEAGI"));
    assert_eq!((count(&mut t, 715), count(&mut t, 716)), (1, 1));
    t.run(61_000);
    assert!(!t.has_sc(egg, "STRIPARMOR"), "a Strip lasts a minute");

    // Chemical Protection: a Coating Wax, two minutes at level 2, and no Strip takes the armour
    t.wear(2301);
    let worn = |t: &T| t.g.store.chars[&t.cid].inventory.iter().any(|i| i.nameid == 2301 && i.equip != 0);
    t.cast("AM_CP_ARMOR", me);
    t.run(1500);
    assert!(t.sc(56));
    assert_eq!(count(&mut t, 7139), 1);
    for _ in 0..200 {
        t.lua(&format!("skill_castend_nodamage_id({egg}, {me}, SK.RG_STRIPARMOR, 5)"));
    }
    assert!(worn(&t) && !t.sc(52));
    t.run(100_000);
    assert!(t.sc(56), "still on after 100 s");
    t.run(30_000);
    assert!(!t.sc(56), "gone after 130 s");
    // unprotected, the armour comes off
    let mut tries = 0;
    while worn(&t) {
        t.lua(&format!("skill_castend_nodamage_id({egg}, {me}, SK.RG_STRIPARMOR, 5)"));
        tries += 1;
        assert!(tries < 300, "the armour never came off");
    }
    assert!(t.sc(52));
    t.check();
}

/// The monster skills added from the April-2004 FAQ (data/db/mob_skills_faq.toml):
/// each runs with its monster as the caster, the way the AI ends a cast.
#[test]
fn faq_monster_skills_run() {
    let mut t = T::new();
    let mut rows: Vec<(u32, u16, u16, String)> = t
        .g
        .db
        .mob_skills
        .values()
        .flatten()
        .filter(|r| r.source.is_some())
        .map(|r| (r.mob, r.skill, r.lv, r.target.clone()))
        .collect();
    rows.sort();
    rows.dedup();
    for (mob, skill, lv, target) in rows {
        if t.hp() == 0 {
            t = T::new();
        }
        t.lua("player.base_level = 99; player.vit = 99; player.agi = 30; heal(30000, 30000)");
        let me = t.me;
        let sk = t.g.db.skill(skill).cloned().expect("skill");
        t.lua("heal(30000, 30000); _rt('sc_clear', player.id)");
        let m = t.spawn(26, 25, mob);
        t.cap.take();
        let hp = t.hp();
        let (tid, x, y) = match target.as_str() {
            "self" => (m, 26, 25),
            _ => (me, 25, 25),
        };
        let call = if sk.inf & 2 != 0 {
            format!("rules.skill_castend{{src={m}, skill={skill}, lv={lv}, target=0, x={x}, y={y}, item=false, val=0}}")
        } else {
            format!("rules.skill_castend{{src={m}, skill={skill}, lv={lv}, target={tid}, x=0, y=0, item=false, val=0}}")
        };
        let r = t.g.console_eval(&call, me);
        let n = std::mem::take(&mut t.g.notices);
        assert!(!r.starts_with("error") && n.is_empty(), "{} by {mob}: {r} {:?}", sk.name, n.first());
        t.run(500);
        let mut ids: Vec<String> = t.cap.take().iter().map(|p| format!("{:04x}", id_of(p))).collect();
        ids.sort();
        ids.dedup();
        let scs = |t: &mut T, u: u32| if t.g.world.units.contains_key(&u) { t.lua(&format!("(function() local s = {{}} for k in pairs(battle.get({u}).sc) do s[#s+1] = k end return table.concat(s, ',') end)()")).trim().to_string() } else { "gone".into() };
        let (msc, psc) = (scs(&mut t, m), scs(&mut t, me));
        let line = format!("{mob} {} lv{lv} {target}: r={} hp {} -> {} mob_sc=[{msc}] my_sc=[{psc}] pk={}", sk.name, r.trim(), hp, t.hp(), ids.join(","));
        assert_eq!(r.trim(), "true", "{line}");
        assert!(!ids.is_empty(), "nothing reached the client: {line}");
        t.kill(m);
        t.lua("_rt('sc_clear', player.id)");
        if t.hp() > 0 && (t.pos(me), t.g.world.units[&me].map.as_str()) != ((25, 25), "testmap") {
            t.lua("warp('testmap', 25, 25)");
            t.send(Writer::new(0x007d));
        }
    }
}

/// A monster's element change takes the element its row means: mod0631's skill_db makes 168 Ghost
/// and 169 Shadow (its names say the opposite), and it routes them as support skills only since
/// jAthena //0850; before that they did nothing.
#[test]
fn monster_element_change() {
    let mut t = T::new();
    for (mob, skill, ele) in [(1151, 168, 8), (1267, 169, 7), (1243, 162, 1)] {
        let m = t.spawn(26, 25, mob);
        let r = t.lua(&format!("rules.skill_castend{{src={m}, skill={skill}, lv=1, target={m}, x=0, y=0, item=false, val=0}}"));
        assert_eq!(r.trim(), "true");
        assert_eq!(t.lua(&format!("battle.elem_type(battle.get({m}))")).trim(), ele.to_string(), "{mob} with {skill}");
        t.kill(m);
    }
}
