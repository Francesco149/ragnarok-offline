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
    gat_with(w, h, |_, _| if water { 3 } else { 0 })
}

/// A map whose cell types come from `ty(x, y)` (0 open, 1 wall, 3 water, 5 cliff).
fn gat_with(w: u32, h: u32, ty: impl Fn(u32, u32) -> u32) -> Vec<u8> {
    let mut b = b"GRAT\x01\x02".to_vec();
    b.extend_from_slice(&w.to_le_bytes());
    b.extend_from_slice(&h.to_le_bytes());
    for i in 0..w * h {
        b.extend_from_slice(&[0u8; 16]);
        b.extend_from_slice(&ty(i % w, i / w).to_le_bytes());
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
        // a cliff across the whole map at x = 28: shot over, never walked over
        vfs.add_memory_file("data\\cliffmap.gat", gat_with(50, 50, |x, _| if x == 28 { 5 } else { 0 }));
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
        // Monsters keep their kind here: many tests use Pupas as still targets, and a Pupa may
        // turn into a Creamy (NPC_METAMORPHOSIS, one chance in a thousand each time it idles).
        // `metamorphosis_hallucination_keeping` reads the real rows.
        for rows in g.db.mob_skills.values_mut() {
            rows.retain(|r| r.skill != 193);
        }
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
    // 40 + 20 x 10 = 240 s (the test clock is the real one skewed: leave room for a busy run)
    t.run(230_000);
    assert!(t.sc(10));
    t.run(11_000);
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

/// Ground skills may not be stacked (jAthena //0764, eAthena 817): a second
/// Safety Wall on the same cell fails and keeps its gemstone, a Pneuma
/// touching it fails, a trap next to a trap fails; a cell further off is
/// fine, and so is a Safety Wall under oneself.
#[test]
fn ground_skills_do_not_stack() {
    let mut t = T::new();
    t.lua("job_change(2); player.base_level = 60; player.int = 60; player.dex = 99; heal(9999, 9999); give_item(717, 5)");
    let gems = |t: &T| t.g.store.chars[&t.cid].inventory.iter().find(|i| i.nameid == 717).map(|i| i.amount).unwrap_or(0);
    let cells = |t: &T, uid: u8| t.packets(0x01c9).iter().filter(|p| p[14] == uid).count();
    t.learn("MG_SAFETYWALL", 5);
    t.cast_at("MG_SAFETYWALL", 25, 25);
    t.run(3000);
    assert_eq!(cells(&t, 0x7e), 1, "a Safety Wall under us");
    assert_eq!(gems(&t), 4);
    t.cast_at("MG_SAFETYWALL", 25, 25);
    t.run(3000);
    assert_eq!(cells(&t, 0x7e), 0, "not a second one on it");
    assert_eq!(gems(&t), 4, "and the gemstone is kept");
    t.lua("job_change(4); heal(9999, 9999)");
    t.learn("AL_PNEUMA", 1);
    t.cast_at("AL_PNEUMA", 26, 25);
    t.run(2000);
    assert_eq!(cells(&t, 0x85), 0, "no Pneuma touching it");
    t.cast_at("AL_PNEUMA", 27, 26);
    t.run(2000);
    assert_eq!(cells(&t, 0x85), 1, "two cells off is fine");
    // traps want a free cell between them
    t.lua("job_change(11); heal(9999, 9999); give_item(1065, 10)");
    t.learn("HT_LANDMINE", 5);
    t.cast_at("HT_LANDMINE", 25, 28);
    t.run(2000);
    assert_eq!(cells(&t, 0x93), 1, "a Land Mine");
    t.cast_at("HT_LANDMINE", 26, 28);
    t.run(2000);
    assert_eq!(cells(&t, 0x93), 0, "not right beside it");
    t.cast_at("HT_LANDMINE", 23, 28);
    t.run(2000);
    assert_eq!(cells(&t, 0x93), 1, "with a cell between");
}

/// Ice Wall can be broken by area skills (eAthena 817): Thunderstorm on a
/// level 1 wall wears it away well before its 8 s are up, and the cells
/// become walkable again. Signum Crucis does nothing to it.
#[test]
fn ice_wall_breaks_under_area_skills() {
    let mut t = T::new();
    t.lua("job_change(9); player.base_level = 99; player.int = 99; player.dex = 99; heal(9999, 9999)");
    t.learn("WZ_ICEWALL", 10);
    t.learn("MG_THUNDERSTORM", 10);
    let wall = |t: &mut T| -> Vec<(i64, i64)> {
        let r = t.lua("local s = {} for _, w in ipairs(icewall_targets(player.map, 0, 0, 49, 49)) do s[#s + 1] = w.x .. ':' .. w.hp end return table.concat(s, ' ')");
        r.split_whitespace().map(|p| { let (x, h) = p.split_once(':').unwrap(); (x.parse().unwrap(), h.parse().unwrap()) }).collect()
    };
    // level 10: 2200, less 50 a second
    t.skill_pos(87, 10, 30, 25);
    t.run(1000);
    let w = wall(&mut t);
    assert_eq!(w.len(), 5, "five cells: {w:?}");
    assert!(w.iter().all(|&(_, h)| (2100..2200).contains(&h)), "durability by time alone: {w:?}");
    t.skill_pos(21, 10, 30, 25);
    t.run(4000);
    let w = wall(&mut t);
    assert!(w.iter().all(|&(_, h)| h < 1900), "Thunderstorm wore it down: {w:?}");
    // level 1 (400) gives way
    t.run(30_000);
    assert!(wall(&mut t).is_empty());
    t.lua("heal(0, 9999)");
    t.skill_pos(87, 1, 30, 25);
    t.run(1000);
    assert_eq!(wall(&mut t).len(), 5);
    assert_eq!(t.lua("return _rt('cell', player.map, 30, 25)").trim(), "5", "blocked");
    t.lua("heal(0, 9999)");
    t.skill_pos(21, 10, 30, 25);
    t.run(3500);
    assert!(wall(&mut t).is_empty(), "broken");
    t.run(800); // it goes within 0.7 s
    assert_eq!(t.lua("return _rt('cell', player.map, 30, 25)").trim(), "0", "free again");
    // a monster's splash wears it too (817's monster skills go through the same skill_area_sub)
    t.run(2000);
    t.lua("heal(0, 9999)");
    t.skill_pos(87, 10, 30, 25);
    t.run(1000);
    let before = wall(&mut t);
    assert_eq!(before.len(), 5);
    t.lua("warp('testmap', 28, 25)");
    let m = t.spawn(27, 25, 1087); // Orc Hero: a blow worth measuring
    let me = t.me;
    t.lua(&format!("player.vit = 99; heal(9999, 0); skill_castend_damage_id({m}, {me}, SK.NPC_SPLASHATTACK, 10)"));
    let after = wall(&mut t);
    let worn = after.len() < before.len() || after.iter().zip(&before).any(|(a, b)| a.1 < b.1 - 20);
    assert!(worn, "the splash reached the wall: {before:?} -> {after:?}");
    t.check();
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

/// A looter keeps ten items (RODataZone's iRO page, jAthena 2004-02): the
/// eleventh and twelfth push the oldest out.
#[test]
fn looter_keeps_ten() {
    let mut t = T::new();
    let p = t.spawn(30, 30, 1002);
    for k in 0..12 {
        t.lua(&format!("_rt('drop_item', 'testmap', 31, 30, {}, 1)", 501 + k));
        t.run(3000);
    }
    let loot: Vec<u32> = t.g.world.units[&p].mob().unwrap().loot.iter().map(|i| i.nameid).collect();
    assert_eq!(loot.len(), 10, "{loot:?}");
    assert!(!loot.contains(&501) && loot.contains(&512), "the oldest went: {loot:?}");
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

/// Above 100 DEX a bow's best roll rises with DEX (RODataZone's iRO combat
/// page, 2003-12): a critical, which takes the most, takes ATK x DEX / 100.
#[test]
fn bow_roll_above_100_dex() {
    let mut t = T::new();
    t.lua("job_change(3); player.base_level = 99; player.str = 1; player.luk = 1; heal(9999, 9999)");
    t.wear(1713); // Arbalest, ATK 90, +2 DEX
    let mob = t.spawn(29, 25, 1002);
    let me = t.me;
    // the roll's part of a critical: its damage less the bow's base, DEX + (DEX/10)^2 + STR/5 + LUK/5
    let roll = |t: &mut T| -> (i64, i64) {
        let r = t.lua(&format!(
            "local d = battle.calc_weapon_attack(battle.get({me}), battle.get({mob}), 0, 0, 0, {{ sure_crit = true }}) \
             local st = battle.get({me}).st return d.damage .. ' ' .. st.dex .. ' ' .. st.str .. ' ' .. st.luk"
        ));
        let v: Vec<i64> = r.split_whitespace().map(|x| x.parse::<f64>().unwrap() as i64).collect();
        let (d, x, s, l) = (v[0], v[1], v[2], v[3]);
        (d - x - (x / 10) * (x / 10) - s / 5 - l / 5, x)
    };
    t.lua("player.dex = 98");
    assert_eq!(roll(&mut t), (90, 100), "at 100 DEX the most is the ATK");
    t.lua(&format!("player.dex = 99; skill_status_change_start({me}, SC.BLESSING, 10, 0); skill_status_change_start({me}, SC.CONCENTRATE, 10, 0)"));
    let (r, x) = roll(&mut t);
    assert!(x > 110, "DEX {x}");
    assert_eq!(r, 90 * x / 100, "above 100 the most is ATK x DEX / 100 (DEX {x})");
}

/// Auto Counter (eAthena 817, jAthena //0751): a blow from the front is
/// turned aside and answered at twice the critical rate; the status ends
/// with it. While it is up the Knight cannot walk, attack or use another
/// skill.
#[test]
fn auto_counter() {
    let mut t = T::new();
    t.lua("job_change(7); player.base_level = 60; player.str = 60; player.luk = 99; player.dex = 60; heal(9999, 9999)");
    t.learn("KN_AUTOCOUNTER", 5);
    t.learn("SM_BASH", 10);
    let z = t.spawn(26, 25, 1008); // a Pupa: it never strikes on its own
    let (me, hp, zhp) = (t.me, t.hp(), t.mob_hp(z).unwrap());
    // face east, toward the Zombie
    let stance = |t: &mut T| {
        t.lua(&format!("_rt('bl_set', {me}, 'dir', 6); skill_status_change_start({me}, SC.AUTOCOUNTER, 5, 0, {{ tick = 60000 }})"));
    };
    stance(&mut t);
    assert!(t.sc(77));
    // held: no walking, no attack, no other skill
    let mut w = Writer::new(0x0085);
    w.pos3(20, 25, 0);
    t.send(w);
    t.run(600);
    assert_eq!(t.pos(me), (25, 25), "no walking in the stance");
    let mut w = Writer::new(0x0089);
    w.u32(z).u8(7);
    t.send(w);
    t.packets(0);
    t.cast("SM_BASH", z);
    t.run(600);
    assert_eq!(t.mob_hp(z), Some(zhp), "no attack and no Bash in the stance");
    assert!(!t.packets(0x0110).is_empty(), "the Bash was refused");
    t.lua(&format!("rules.weapon_attack{{src={z}, target={me}}}"));
    assert_eq!(t.hp(), hp, "the blow was turned aside");
    assert!(!t.sc(77), "the stance ends");
    assert!(t.mob_hp(z).unwrap_or(0) < zhp, "and the Pupa was hit back");
    // LUK 99: a plain blow is critical 29.8% of the time, less the Pupa's LUK 20 x 3 (6%,
    // 817 battle.c:1751); a counter twice that, 47.6% (an earlier band of 45-75% failed one run
    // in six)
    t.packets(0);
    let (mut crits, n) = (0, 200);
    for _ in 0..n {
        t.lua(&format!("heal(9999, 9999); _rt('bl_set', {z}, 'hp', 2000)"));
        stance(&mut t);
        t.lua(&format!("rules.weapon_attack{{src={z}, target={me}}}"));
        crits += t.packets(0x008a).iter().filter(|p| u32_at(p, 2) == me && p[26] == 0x0a).count();
    }
    assert!((n * 35 / 100..=n * 60 / 100).contains(&crits), "{crits} critical counters of {n}");
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

/// Dying: 1% of the EXP the levels need is gone at once (jAthena //0513),
/// no recovery while dead, and back at the save point with 1 HP
/// (pc_setrestartvalue) and nothing more taken.
#[test]
fn death_and_return() {
    let mut t = T::new();
    let (nb, nj) = (t.g.db.exp.base[39], t.g.db.exp.job_first[29]);
    let (b0, j0) = (nb / 2, nj / 2);
    t.lua(&format!("job_change(1); player.base_level = 40; player.job_level = 30; player.base_exp = {b0}; player.job_exp = {j0}; heal(9999, 9999)"));
    let me = t.me;
    t.lua(&format!("_rt('bl_damage', 0, {me}, 99999)"));
    assert_eq!(t.hp(), 0);
    assert_eq!(t.g.world.units[&me].state, 1, "dead");
    let (b1, j1) = (b0 - nb / 100, j0 - nj / 100);
    let c = &t.g.store.chars[&t.cid];
    assert_eq!((c.base_exp, c.job_exp), (b1, j1), "taken at death");
    t.run(60_000);
    assert_eq!(t.hp(), 0, "the dead do not recover");
    let mut w = Writer::new(0x00b2);
    w.u8(0);
    t.send(w);
    t.send(Writer::new(0x007d));
    let c = &t.g.store.chars[&t.cid];
    assert_eq!((c.hp, c.base_exp, c.job_exp), (1, b1, j1));
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
/// the victim still walks, drawn as opt1 6 (this client tints it half grey);
/// then stone until the time is up: 17 s less MDEF/20 (Ragu Memo, 2003-12),
/// 16.75 s for a Poring.
#[test]
fn stone_sets_then_wears_off() {
    let mut t = T::new();
    let mob = t.spawn(27, 25, 1002);
    t.lua(&format!("skill_status_change_start({mob}, SC.STONE, 1, 0, {{ tick = 12000 }})"));
    assert_eq!(t.lua(&format!("battle.get({mob}).opt1")).trim(), "6", "still turning (817's opt1 6)");
    assert!(t.packets(0x0119).iter().any(|p| u16_at(p, 6) == 6), "the client is shown the turning (opt1 6)");
    t.run(5500);
    assert_eq!(t.lua(&format!("battle.get({mob}).opt1")).trim(), "1", "stone");
    assert!(t.packets(0x0119).iter().any(|p| u16_at(p, 6) == 1), "the client is shown the stone when it sets");
    t.run(14_000);
    assert_eq!(t.lua(&format!("battle.sc(battle.get({mob}), SC.STONE) and 1 or 0")).trim(), "1", "still stone after 19.5 s");
    t.run(4_000);
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

/// The Baphomet Card splashes a weapon blow on the cells around the target
/// (the client's text, 817's `bSplashRange`): the plant beside it is hit,
/// the one two cells off is not.
#[test]
fn baphomet_card_splashes() {
    let mut t = T::new();
    t.lua("player.base_level = 99; player.dex = 99; give_item(1201, 1); give_item(4147, 1)");
    let (card, knife) = (t.inv(4147), t.inv(1201));
    let mut w = Writer::new(0x017c);
    w.u16(card).u16(knife);
    t.send(w);
    let knife = t.inv(1201);
    let mut w = Writer::new(0x00a9);
    w.u16(knife).u16(0);
    t.send(w);
    assert_eq!(t.st("splash_range"), 1);
    let (a, b, c) = (t.spawn(26, 25, 1080), t.spawn(27, 25, 1080), t.spawn(28, 25, 1080));
    let full = t.mob_hp(c);
    let mut w = Writer::new(0x0089);
    w.u32(a).u8(7);
    t.send(w);
    t.run(3000);
    assert!(t.mob_hp(a) < full, "the target");
    assert!(t.mob_hp(b) < full, "the one beside it, by the splash");
    assert_eq!(t.mob_hp(c), full, "two cells off");
}

/// Storm Gust freezes on its third hit and passes over the frozen (817,
/// jAthena //0738): a frozen monster takes nothing from it.
#[test]
fn storm_gust_spares_the_frozen() {
    let mut t = T::new();
    t.lua("job_change(9); player.base_level = 99; player.int = 30; heal(9999, 9999)");
    let me = t.me;
    let wolf = t.spawn(26, 25, 1013);
    let hit = |t: &mut T| t.lua(&format!("return skill_attack(BF.MAGIC, {me}, {me}, {wolf}, SK.WZ_STORMGUST, 1, 0)")).trim().parse::<i64>().unwrap();
    assert!(hit(&mut t) > 0, "a first hit lands");
    hit(&mut t);
    if !t.has_sc(wolf, "FREEZE") {
        hit(&mut t);
    }
    assert!(t.has_sc(wolf, "FREEZE"), "frozen by the third hit");
    let hp = t.mob_hp(wolf);
    assert_eq!(hit(&mut t), 0);
    assert_eq!(t.mob_hp(wolf), hp, "the frozen take nothing");
}

/// EXP is shared by the part of the monster's max HP each player took off
/// (jAthena //0764 of 2004-02-01, 817), not by the damage the players dealt
/// (mod0631): a kill after something else did most of the work gives little.
/// The MVP EXP comes with the reward item, not instead of it (jAthena //0814).
#[test]
fn exp_share_by_max_hp() {
    let mut t = T::new();
    t.lua("job_change(1); player.base_level = 90; player.base_exp = 0; player.job_level = 40; player.job_exp = 0");
    t.send(Writer::new(0x007d));
    let me = t.me;
    let exp = |t: &T| (t.g.store.chars[&t.cid].base_exp, t.g.store.chars[&t.cid].job_exp);
    // a Wolf (405 HP, 45/32 EXP) alone: all of it
    let wolf = t.spawn(26, 25, 1013);
    t.lua(&format!("_rt('bl_damage', {me}, {wolf}, 9999)"));
    assert_eq!(exp(&t), (45, 32));
    // half of it taken off by something else: 200/405 of the EXP
    let wolf = t.spawn(26, 25, 1013);
    t.lua(&format!("_rt('bl_damage', 0, {wolf}, 205); _rt('bl_damage', {me}, {wolf}, 9999)"));
    assert_eq!(exp(&t), (45 + 45 * (200 * 256 / 405) / 256, 32 + 32 * (200 * 256 / 405) / 256));
    // the last point only: 1 EXP each, not nothing
    let wolf = t.spawn(26, 25, 1013);
    t.lua(&format!("_rt('bl_damage', 0, {wolf}, 404); _rt('bl_damage', {me}, {wolf}, 9999)"));
    assert_eq!(exp(&t), (45 + 22 + 1, 32 + 15 + 1));
    // Eddga alone: its EXP and the MVP EXP (6179), whatever item it gives
    let (b0, j0) = exp(&t);
    t.packets(0x010b);
    let eddga = t.spawn(26, 25, 1115);
    t.lua(&format!("_rt('bl_damage', {me}, {eddga}, 999999)"));
    assert_eq!(exp(&t), (b0 + 25025 + 6179, j0 + 12870));
    assert_eq!(t.packets(0x010b).len(), 1, "the MVP EXP shown");
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
    // a poisonous blow that misses does not ("hit by a Poisonous attack";
    // found in the client 2026-11-15, where a Poison Spore's miss set it off).
    // At AGI 255 the Pupa still hits one blow in twenty: cast again after one.
    let agi = t.lua("return player.agi");
    t.lua("player.agi = 255");
    let poison = format!("skill_attack(BF.WEAPON, {m}, {m}, {me}, SK.TF_POISON, 1, 0)");
    let mut missed = 0;
    for _ in 0..5 {
        if !t.sc(7) {
            t.lua("heal(0, 9999)");
            t.cast("AS_POISONREACT", me);
            t.run(500);
            assert!(t.sc(7), "up again");
        }
        let (before, mhp) = (t.lua("return player.hp"), t.mob_hp(m));
        t.lua(&poison);
        if t.lua("return player.hp") == before {
            missed += 1;
            assert!(t.sc(7), "a miss does not set it off");
            assert_eq!(t.mob_hp(m), mhp, "nor is it answered");
            break;
        }
    }
    assert!(missed > 0, "a blow missed");
    // a poisonous blow that lands does, once (the Pupa may still miss now and then)
    t.lua(&format!("player.agi = {agi}"));
    let hp = t.mob_hp(m).unwrap();
    for _ in 0..40 {
        if !t.sc(7) {
            break;
        }
        t.lua(&poison);
    }
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

/// Venom Splasher (THREADS.md #22): only on a poisoned enemy down to a third
/// of its HP; it marks the target, and when the mark runs out (10 s at level
/// 10) the target bursts and the enemies next to it take a quarter. A hit
/// before then undoes the mark.
#[test]
fn venom_splasher() {
    let mut t = T::new();
    t.lua("job_change(12); player.base_level = 70; player.str = 80; player.dex = 80; heal(9999, 9999)");
    t.learn("AS_SPLASHER", 10);
    let a = t.spawn(27, 25, 1008); // Pupas: they neither move nor fight back
    let b = t.spawn(28, 25, 1008); // next to it
    let c = t.spawn(30, 25, 1008); // two cells off
    let (hp_b, hp_c) = (t.mob_hp(b).unwrap(), t.mob_hp(c).unwrap());
    let fails = |t: &mut T| t.packets(0x0110).len();
    t.cap.take();
    t.cast("AS_SPLASHER", a);
    t.run(1500);
    assert!(fails(&mut t) > 0 && !t.has_sc(a, "SPLASHER"), "not on a healthy, unpoisoned target");
    t.lua(&format!("skill_status_change_start({a}, SC.POISON, 1, 0, {{tick = 60000, nodef = true}})"));
    t.lua(&format!("_rt('bl_set', {a}, 'hp', battle.max_hp(battle.get({a})) // 2)"));
    t.lua("heal(0, 9999)");
    t.cast("AS_SPLASHER", a);
    t.run(1500);
    assert!(fails(&mut t) > 0 && !t.has_sc(a, "SPLASHER"), "not above a third of its HP");
    t.lua(&format!("_rt('bl_set', {a}, 'hp', battle.max_hp(battle.get({a})) * 3 // 10)"));
    let hp_a = t.mob_hp(a).unwrap();
    t.lua("heal(0, 9999)");
    t.cast("AS_SPLASHER", a);
    t.run(500);
    assert!(t.has_sc(a, "SPLASHER"), "poisoned and down to 30%: marked");
    t.run(8000);
    assert!(t.has_sc(a, "SPLASHER"), "still counting at 8.5 s");
    assert_eq!(t.mob_hp(b), Some(hp_b), "nothing yet");
    t.run(2500);
    assert!(t.mob_hp(a).is_none_or(|h| h < hp_a), "the target bursts");
    let lost_b = hp_b - t.mob_hp(b).unwrap_or(0);
    assert!(lost_b > 0, "its neighbour is hit");
    assert_eq!(t.mob_hp(c), Some(hp_c), "two cells off is not");
    // a blow before the time is up undoes the mark
    t.kill(a);
    t.kill(b);
    t.run(3000);
    let d = t.spawn(27, 25, 1008);
    let e = t.spawn(28, 25, 1008);
    let hp_e = t.mob_hp(e).unwrap();
    t.lua(&format!("skill_status_change_start({d}, SC.POISON, 1, 0, {{tick = 60000, nodef = true}})"));
    t.lua(&format!("_rt('bl_set', {d}, 'hp', battle.max_hp(battle.get({d})) * 3 // 10)"));
    t.lua("heal(0, 9999)");
    t.cast("AS_SPLASHER", d);
    t.run(500);
    assert!(t.has_sc(d, "SPLASHER"));
    let me = t.me;
    t.lua(&format!("battle.damage({me}, {d}, 1)"));
    assert!(!t.has_sc(d, "SPLASHER"), "a hit undoes it");
    t.run(11_000);
    assert_eq!(t.mob_hp(e), Some(hp_e), "and nothing bursts");
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

/// Talkie Box: the text typed with 0x0190 is said (0x0191, under the trap's
/// own id) when the first one steps on it, its own Hunter aside, and the
/// trap goes 5 s later.
#[test]
fn talkie_box_speaks() {
    let mut t = T::new();
    t.lua("job_change(11); player.base_level = 70; player.dex = 90; heal(9999, 9999); give_item(1065, 10)");
    t.learn("HT_TALKIEBOX", 1);
    let mut w = Writer::new(0x0190);
    w.u16(1).u16(t.skill_id("HT_TALKIEBOX")).u16(28).u16(25).str_fixed("Beware of the Zombie!", 80);
    t.send(w);
    assert!(t.packets(0x01c9).iter().any(|p| p[14] == 0x99), "a Talkie Box is set");
    let cell = t.g.world.units.values().find(|u| u.skill().is_some()).map(|u| u.id).expect("its cell");
    t.run(3000);
    assert!(t.packets(0x0191).is_empty(), "silent until stepped on");
    // its own Hunter walks over it in silence (the iRO guide; jAthena //0991)
    let me = t.me;
    let mut w = Writer::new(0x0085);
    w.pos3(30, 25, 0);
    t.send(w);
    t.run(2000);
    assert_eq!(t.pos(me), (30, 25), "walked across the trap");
    assert!(t.packets(0x0191).is_empty(), "not for the caster");
    let mut w = Writer::new(0x0085);
    w.pos3(25, 25, 0);
    t.send(w);
    t.run(2000);
    let z = t.spawn(32, 25, 1015);
    t.run(4000);
    let said = t.packets(0x0191);
    assert_eq!(said.len(), 1, "it speaks once");
    assert_eq!(u32_at(&said[0], 2), cell, "under the trap's id");
    assert!(said[0][6..].starts_with(b"Beware of the Zombie!\0"), "the text typed");
    t.kill(z);
    t.run(6000);
    assert!(!t.g.world.units.contains_key(&cell), "and goes 5 s later");
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
    // the stripped Picky turns on us, and a blow would break the Dispell's cast below
    t.kill(picky);

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
    let mut rows: Vec<(u32, u16, u16, String, i32)> = t
        .g
        .db
        .mob_skills
        .values()
        .flatten()
        // rows with their own source, less mob_skills.toml's summons from jAthena //0743
        .filter(|r| r.source.as_ref().is_some_and(|s| !s.starts_with("jAthena //0743")))
        .map(|r| (r.mob, r.skill, r.lv, r.target.clone(), r.val))
        .collect();
    rows.sort();
    rows.dedup();
    for (mob, skill, lv, target, val) in rows {
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
            format!("rules.skill_castend{{src={m}, skill={skill}, lv={lv}, target=0, x={x}, y={y}, item=false, val={val}}}")
        } else {
            format!("rules.skill_castend{{src={m}, skill={skill}, lv={lv}, target={tid}, x=0, y=0, item=false, val={val}}}")
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

/// Every distinct monster skill row (skill, level, target, argument) of every file, cast once by
/// the first monster that has it, against a sturdy character: no script error, and the cast is
/// accepted. `faq_monster_skills_run` checks the FAQ's rows more closely; this one catches rows
/// of mod0631, 817 and jAthena that nothing else casts.
#[test]
fn every_monster_skill_row_runs() {
    let mut t = T::new();
    let mut seen = std::collections::HashSet::new();
    let mut rows: Vec<(u32, u16, u16, String, i32)> = Vec::new();
    // the real rows (the fixture drops Metamorphosis)
    let db = ro_data::db::Database::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/db")).expect("db");
    let mut all: Vec<_> = db.mob_skills.values().flatten().cloned().collect();
    all.sort_by_key(|r| (r.mob, r.skill, r.lv));
    for r in all {
        if r.target == "friend" {
            continue; // never cast (mobskill_use skips the line, as mod0631)
        }
        if seen.insert((r.skill, r.lv, r.target.clone(), r.val)) {
            rows.push((r.mob, r.skill, r.lv, r.target.clone(), r.val));
        }
    }
    let mut bad = Vec::new();
    for (mob, skill, lv, target, val) in rows {
        if t.hp() == 0 {
            t = T::new();
        }
        t.lua("player.base_level = 99; player.vit = 99; player.agi = 30; heal(30000, 30000); _rt('sc_clear', player.id)");
        let me = t.me;
        let Some(sk) = t.g.db.skill(skill).cloned() else {
            bad.push(format!("{mob}: skill {skill} not in the database"));
            continue;
        };
        let m = t.spawn(26, 25, mob);
        t.cap.take();
        let (tid, x, y) = if target == "self" { (m, 26, 25) } else { (me, 25, 25) };
        let call = if sk.inf & 2 != 0 {
            format!("rules.skill_castend{{src={m}, skill={skill}, lv={lv}, target=0, x={x}, y={y}, item=false, val={val}}}")
        } else {
            format!("rules.skill_castend{{src={m}, skill={skill}, lv={lv}, target={tid}, x=0, y=0, item=false, val={val}}}")
        };
        let r = t.g.console_eval(&call, me);
        let n = std::mem::take(&mut t.g.notices);
        t.run(300);
        let n2 = std::mem::take(&mut t.g.notices);
        if r.starts_with("error") || !n.is_empty() || !n2.is_empty() || r.trim() != "true" {
            bad.push(format!("{mob} {} lv{lv} {target} val{val}: {} {:?} {:?}", sk.name, r.trim(), n.first(), n2.first()));
        }
        t.kill(m);
        // slaves and summons left behind
        t.lua("for _, u in ipairs(nearby(30)) do if u.kind == 'mob' then _rt('mob_kill', u.id) end end");
        t.lua("_rt('sc_clear', player.id)");
        if t.hp() > 0 && (t.pos(me), t.g.world.units[&me].map.as_str()) != ((25, 25), "testmap") {
            t.lua("warp('testmap', 25, 25)");
            t.send(Writer::new(0x007d));
        }
    }
    assert!(bad.is_empty(), "{} rows failed:\n{}", bad.len(), bad.join("\n"));
}

/// Every item with a use effect, used once by a level-99 character whose job may use it: no
/// script error and nothing reported unimplemented. Items that open a window or ask (a
/// Magnifier, Warp items) only need to start without an error here.
#[test]
fn every_usable_item_runs() {
    let mut t = T::new();
    // the character is male: the Chocolates (gender 0) are refused it, rightly
    let mut items: Vec<(u32, String, u32)> = t.g.db.items.values().filter(|i| !i.use_script.is_empty() && i.gender != 0).map(|i| (i.id, i.display.clone(), i.jobs)).collect();
    items.sort();
    let mut bad = Vec::new();
    for (id, name, jobs) in items {
        if t.hp() == 0 || t.g.world.units[&t.me].state == 1 {
            t = T::new();
        }
        // a job the item allows (Novice first)
        let job = (0..24).find(|j| jobs == 0 || jobs & (1 << j) != 0).unwrap_or(0);
        t.lua(&format!("job_change({job}); player.base_level = 99; heal(30000, 30000); _rt('sc_clear', player.id)"));
        t.lua(&format!("give_item({id}, 1)"));
        let idx = t.inv(id);
        t.cap.take();
        let mut w = Writer::new(0x00a7);
        w.u16(idx).u32(t.me);
        t.send(w);
        let n = std::mem::take(&mut t.g.notices);
        t.run(200);
        let n2 = std::mem::take(&mut t.g.notices);
        let ack = t.cap.take().into_iter().find(|p| id_of(p) == 0x00a8);
        let ok = ack.as_ref().is_some_and(|p| p[6] == 1);
        if !n.is_empty() || !n2.is_empty() || !ok {
            bad.push(format!("{id} {name} (job {job}): used={ok} {:?} {:?}", n.first(), n2.first()));
        }
        // close what the item opened, and go back
        t.lua("for _, u in ipairs(nearby(30)) do if u.kind == 'mob' then _rt('mob_kill', u.id) end end");
        if t.hp() > 0 && (t.pos(t.me), t.g.world.units[&t.me].map.as_str()) != ((25, 25), "testmap") {
            t.lua("warp('testmap', 25, 25)");
            t.send(Writer::new(0x007d));
        }
    }
    assert!(bad.is_empty(), "{} items failed:\n{}", bad.len(), bad.join("\n"));
}

/// Three monster skills neither period emulator's code fully had: a Fabre turns into a Pupa
/// (817's Metamorphosis, its line's monster from 817), a Hallucination makes the hits drawn
/// nonsense while the HP taken stays true, and Keeping shuts a Cornutus in its shell (jAthena
/// //0850, 2004-02-29).
#[test]
fn metamorphosis_hallucination_keeping() {
    let mut t = T::new();
    let me = t.me;
    // Metamorphosis: the line carries 1008 now; half its HP stays half
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let db = ro_data::db::Database::load(&root.join("data/db")).expect("db");
    let row = db.mob_skills[&1007].iter().find(|r| r.skill == t.skill_id("NPC_METAMORPHOSIS")).cloned().unwrap();
    assert_eq!(row.val, 1008, "the Fabre's line names the Pupa (817)");
    let m = t.spawn(30, 25, 1007);
    let max = t.mob_hp(m).unwrap();
    t.lua(&format!("_rt('bl_damage', player.id, {m}, {})", max / 2));
    t.cap.take();
    let r = t.g.console_eval(&format!("rules.skill_castend{{src={m}, skill={}, lv=1, target={m}, x=0, y=0, item=false, val=1008}}", row.skill), me);
    assert_eq!(r.trim(), "true");
    assert_eq!(t.g.world.units[&m].class, 1008, "now a Pupa");
    let pupa = t.g.db.mob(1008).unwrap().hp;
    let hp = t.mob_hp(m).unwrap();
    assert!(hp > 0 && hp <= pupa / 2 + 1 && hp + 2 >= pupa / 2, "half of the Pupa's HP: {hp} of {pupa}");
    let pk = t.cap.take();
    assert!(pk.iter().any(|p| id_of(p) == 0x0080 && u32_at(p, 2) == m), "drawn anew: vanish");
    assert!(pk.iter().any(|p| matches!(id_of(p), 0x0078 | 0x007c) && u32_at(p, 2) == m), "drawn anew: appear");
    t.kill(m);

    // Hallucination: the icon, the length, and hits drawn as nonsense
    t.lua("player.base_level = 99; player.vit = 99; heal(30000, 30000)");
    let m = t.spawn(26, 25, 1101); // Baphomet Jr.
    t.lua("skill_status_change_start(player.id, SC.HALLUCINATION, 1, 0)");
    assert!(t.sc(34), "Hallucination on");
    let mut differs = false;
    for _ in 0..12 {
        t.cap.take();
        let real: i64 = t.lua(&format!("return skill_attack(BF.WEAPON, {m}, {m}, player.id, SK.NPC_PIERCINGATT, 1, 0)")).trim().parse().unwrap();
        let skill = t.skill_id("NPC_PIERCINGATT");
        let p = t.cap.take().into_iter().find(|p| id_of(p) == 0x01de && u16::from_le_bytes([p[2], p[3]]) == skill).expect("the hit is drawn");
        let shown = i32::from_le_bytes(p[24..28].try_into().unwrap()) as i64;
        if real > 0 {
            assert!(shown >= real && shown <= real * 10 + 99, "shown {shown} for {real}");
            differs |= shown != real;
        } else {
            assert_eq!(shown, 0, "a miss stays a miss");
        }
        // a hit that kills leaves the character dead under heal(): stand it up
        t.lua("if battle.get(player.id).dead then _rt('bl_stand', player.id, 30000) end heal(30000, 30000)");
    }
    assert!(differs, "some hit drawn as another number");
    t.kill(m);
    t.run(31_000);
    assert!(!t.sc(34), "level 1 lasts 30 s");

    // Keeping: the Cornutus takes next to nothing, and neither walks nor strikes for 60 s
    t.lua("player.str = 90; player.dex = 90; heal(30000, 30000)");
    let open = t.spawn(24, 27, 1067);
    let open_lost = t.bashed(open);
    t.kill(open);
    let m = t.spawn(27, 25, 1067);
    t.lua(&format!("skill_status_change_start({m}, SC.KEEPING, 1, 0)"));
    let kept_lost = t.bashed(m);
    assert!(kept_lost <= 5 && open_lost > 50, "DEF 100: {kept_lost} through the shell, {open_lost} without");
    t.lua(&format!("_rt('mob_target', {m}, player.id, 2)"));
    let me_hp = t.hp();
    t.run(5_000);
    assert_eq!(t.pos(m), (27, 25), "shut in: it does not move");
    assert_eq!(t.hp(), me_hp, "shut in: it does not strike");
    t.kill(m);
}

impl T {
    /// HP five Bashes of level 10 take from a monster.
    fn bashed(&mut self, m: u32) -> u32 {
        let before = self.mob_hp(m).unwrap();
        for _ in 0..5 {
            self.lua(&format!("skill_attack(BF.WEAPON, player.id, player.id, {m}, SK.SM_BASH, 10, 0)"));
        }
        self.run(1_000); // the HP goes when the hits land
        before - self.mob_hp(m).unwrap_or(0)
    }
}

/// iRO's boss rule (the FAQ of 2004-04): "All boss monsters will Teleport if they are attacked
/// from beyond their attacking range" -- here, from where they cannot walk to (rAthena's
/// "rudeattacked"). An Osiris shot across a cliff goes; one that can reach its attacker, or a
/// monster that is no boss, stays.
#[test]
fn boss_teleports_when_attacked_from_out_of_reach() {
    let mut t = T::on("cliffmap");
    let me = t.me;
    let shot = |t: &mut T, m: u32| {
        t.lua(&format!("skill_attack(BF.MAGIC, {me}, {me}, {m}, SK.MG_FIREBOLT, 1, 0)"));
    };
    // across the cliff: no way round, so every hit is a rude one
    let m = t.spawn(31, 25, 1038);
    shot(&mut t, m);
    assert_ne!(t.pos(m), (31, 25), "Osiris teleports away");
    t.kill(m);
    // on this side, three cells off: it can walk up, so it stays
    let m = t.spawn(25, 28, 1038);
    for _ in 0..5 {
        shot(&mut t, m);
    }
    assert_eq!(t.pos(m), (25, 28), "an Osiris that can reach its attacker stays");
    t.kill(m);
    // a monster that is no boss has no such rule
    let m = t.spawn(31, 25, 1002);
    shot(&mut t, m);
    if t.g.world.units.contains_key(&m) {
        assert_eq!(t.pos(m), (31, 25), "a Poring stays");
    }
}

/// The same rule when the way is shut by Ice Wall rather than the map: a boss walled into a
/// 3x3 box and shot from outside it teleports, as Eddga did in the client (2026-11-15).
#[test]
fn boss_in_an_ice_wall_box_teleports() {
    let mut t = T::new();
    let me = t.me;
    t.lua("job_change(9); player.base_level = 99; player.int = 99; player.dex = 99");
    t.learn("WZ_ICEWALL", 10);
    // a wall 2 cells off on each side, cast from 6 cells off: the line runs across the caster's
    for (dx, dy) in [(-1i32, 0i32), (1, 0), (0, 1), (0, -1)] {
        t.lua(&format!("heal(9999, 9999); warp('testmap', {}, {})", 25 + 6 * dx, 25 + 6 * dy));
        t.skill_pos(87, 10, (25 + 2 * dx) as u16, (25 + 2 * dy) as u16);
        t.run(1500);
    }
    let walls = t.lua("return #icewall_targets(player.map, 22, 22, 28, 28)");
    assert_eq!(walls.trim(), "16", "a closed box");
    t.lua("warp('testmap', 18, 25)");
    let m = t.spawn(25, 25, 1038);
    t.lua(&format!("skill_attack(BF.MAGIC, {me}, {me}, {m}, SK.MG_FIREBOLT, 1, 0)"));
    assert_ne!(t.pos(m), (25, 25), "Osiris teleports out of the box");
}

/// Slave summons as jAthena //0743 (2004-01-26) and 817 have them: a 3 s cast, and again 10 s
/// later once every slave is gone. Eddga calls five Bigfoot once an hour, Leib Olmai between.
#[test]
fn mvp_summons_as_jathena_0743() {
    let mut t = T::new();
    let me = t.me;
    t.lua("job_change(7); player.base_level = 99; player.vit = 99; heal(30000, 30000)");
    let eddga = t.spawn(27, 25, 1115);
    t.lua(&format!("_rt('bl_damage', {me}, {eddga}, 1)"));
    let slaves = |t: &T, kind: u32| -> Vec<u32> {
        t.g.world.units.values().filter(|u| u.mob().is_some_and(|m| m.master == Some(eddga) && m.mob_id == kind)).map(|u| u.id).collect()
    };
    // when its summon row `i` was last done with (the delay counts from the cast's end, mob.c:1945)
    let used = |t: &T, i: usize| t.g.world.units[&eddga].mob().unwrap().skill_used[i];
    let leib = t.g.db.mob_skills[&1115].iter().position(|r| r.skill == 196 && r.val == 1306).unwrap();
    // run in 100 ms steps, healing, until `kind` has five slaves; the summon's cast was seen
    // under way, 3 s long, and none came before its end. Gives the cast's start.
    let wait = |t: &mut T, kind: u32| -> u64 {
        let (mut ms, mut cast) = (0, None);
        while slaves(t, kind).len() < 5 {
            assert!(ms < 15000 && t.hp() > 0, "no five of {kind} after {ms} ms (hp {})", t.hp());
            if let Some(c) = t.g.world.units[&eddga].mob().unwrap().cast.as_ref().filter(|c| c.skill == 196) {
                if cast.is_none() {
                    assert!((2800..=3000).contains(&(c.end_at - t.g.now())), "a 3 s cast");
                    cast = Some(c.end_at);
                }
            }
            t.lua("heal(30000, 30000)");
            t.run(100);
            ms += 100;
        }
        let end = cast.expect("the summon was cast");
        assert!(t.g.now() >= end);
        end - 3000
    };
    wait(&mut t, 1060);
    assert!(slaves(&t, 1306).is_empty());
    slaves(&t, 1060).into_iter().for_each(|s| t.kill(s));
    // the Bigfoot row waits an hour: Leib Olmai
    wait(&mut t, 1306);
    let done = used(&t, leib);
    assert!(slaves(&t, 1060).is_empty());
    slaves(&t, 1306).into_iter().for_each(|s| t.kill(s));
    // again once they are gone, 10 s after the last
    let again = wait(&mut t, 1306);
    assert!(again >= done + 10000, "Leib Olmai again {} ms after", again - done);
    t.check();
    // the Turtle General calls in steps: Permeters with none left, Soldiers with two or fewer...
    let rows: Vec<(i32, i32)> = t.g.db.mob_skills[&1312].iter().filter(|r| r.skill == 196).map(|r| (r.cond_val, r.val)).collect();
    assert_eq!(rows, [(0, 1314), (2, 1316), (4, 1318), (8, 1319)]);
    // the Fur Seal no longer has the Fake Angel's summons and stone curse (mod0631's misfiled rows)
    assert!(t.g.db.mob_skills[&1317].iter().all(|r| r.skill != 196 && r.skill != 180));
    assert_eq!(t.g.db.mob_skills[&1371].iter().filter(|r| r.skill == 196).count(), 2);
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

/// A monster's ground unit seen again (THREADS.md #28): the client draws a
/// second 0x01c9 for the same unit hidden. A trap is sent again (iRO hid them
/// so) and Detect shows it; anything else stays on the client, never re-sent.
#[test]
fn others_ground_units_seen_again() {
    let mut t = T::new();
    let pupa = t.spawn(25, 20, 1008);
    t.lua(&format!("skill_unitsetting({pupa}, SK.WZ_QUAGMIRE, 5, 20, 15, 0)"));
    t.lua(&format!("skill_unitsetting({pupa}, SK.HT_SKIDTRAP, 1, 22, 32, 0)"));
    let trap = t.g.world.units.values().find(|u| u.skill().is_some_and(|s| s.unit_id == 0x90)).map(|u| u.id).expect("trap");
    let shown = t.packets(0x01c9);
    assert_eq!(shown.len(), 26, "25 Quagmire cells and the trap");
    let walk = |t: &mut T, x: u16| {
        let mut w = Writer::new(0x0085);
        w.pos3(x, 25, 0);
        t.send(w);
        t.run(6000);
        assert_eq!(t.pos(t.me), (x, 25));
    };
    walk(&mut t, 45);
    let all = t.cap.take();
    let gone: Vec<u32> = all.iter().filter(|p| id_of(p) == 0x0120).map(|p| u32_at(p, 2)).collect();
    assert_eq!(gone, vec![trap], "only the trap is taken off the client");
    walk(&mut t, 25);
    let back: Vec<u32> = t.packets(0x01c9).iter().map(|p| u32_at(p, 2)).collect();
    assert_eq!(back, vec![trap], "only the trap is sent again (and drawn hidden)");
    // Detect shows it again
    t.lua("job_change(11); player.base_level = 70; heal(9999, 9999); set_falcon()");
    t.learn("HT_DETECTING", 4);
    t.cast_at("HT_DETECTING", 22, 32);
    t.run(500);
    let shown: Vec<u32> = t.packets(0x01ac).iter().map(|p| u32_at(p, 2)).collect();
    assert_eq!(shown, vec![trap], "Detect reveals the trap");
    // a unit the client keeps out of sight is still taken off when it ends
    walk(&mut t, 45);
    t.packets(0x0120);
    t.run(30_000);
    let gone = t.packets(0x0120).len();
    assert!(gone >= 25, "the Quagmire's end reaches the client out of sight ({gone})");
}

/// Ailments as Ragu Memo's page of 2003-12 has them: a stun of about 3 s
/// whatever the skill, Blind by base level and INT, a frozen monster's DEF
/// halved and MDEF a quarter up, a poisoned one's DEF a quarter down.
#[test]
fn ailment_lengths_and_effects_of_2003() {
    let mut t = T::new();
    let mob = t.spawn(27, 25, 1002); // Poring: VIT 1, LUK 30 (DEF 0: take a Rocker, DEF 5)
    t.lua(&format!("skill_status_change_start({mob}, SC.STAN, 10, 0, {{ tick = 15000 }})"));
    assert!(t.has_sc(mob, "STAN"));
    t.run(3100);
    assert!(!t.has_sc(mob, "STAN"), "3 s less (3 + VIT + LUK/3)%, not Sonic Blow's 15 s");
    // Blind on the player: 30 s - BaseLv/10 - INT/15
    let me = t.me;
    t.lua(&format!("skill_status_change_start({me}, SC.BLIND, 1, 0, {{ tick = 10000 }})"));
    let lv: i64 = t.lua(&format!("battle.lv(battle.get({me}))")).trim().parse().unwrap();
    let int: i64 = t.lua(&format!("battle.int(battle.get({me}))")).trim().parse().unwrap();
    let want = 30000 - lv * 100 - int * 1000 / 15;
    let left: i64 = t.lua(&format!("battle.sc(battle.get({me}), SC.BLIND).left")).trim().parse().unwrap_or(-1);
    assert!((left - want).abs() < 500, "blind for {left} ms, want {want}");
    // DEF and MDEF of a frozen, then a poisoned Rocker
    let r = t.spawn(28, 25, 1052);
    let def = |t: &mut T| -> (i64, i64) {
        let s = t.lua(&format!("battle.def(battle.get({r})) .. ' ' .. battle.mdef(battle.get({r}))"));
        let v: Vec<i64> = s.trim().trim_matches('"').split(' ').map(|x| x.parse().unwrap()).collect();
        (v[0], v[1])
    };
    let (d0, m0) = def(&mut t);
    t.lua(&format!("skill_status_change_start({r}, SC.FREEZE, 10, 0, {{ tick = 20000, nodef = true }})"));
    let (d1, m1) = def(&mut t);
    assert_eq!((d1, m1), (d0 >> 1, m0 * 125 / 100), "frozen: DEF halved, MDEF +25%");
    t.lua(&format!("skill_status_change_end({r}, SC.FREEZE)"));
    t.lua(&format!("skill_status_change_start({r}, SC.POISON, 1, 0, {{ tick = 20000, nodef = true }})"));
    assert_eq!(def(&mut t).0, d0 * 75 / 100, "poisoned: DEF -25%");
}

/// Leaving the game dead and coming back: alive at the save point with the
/// restart HP, as mod0631 saved a dead character (pc_makesavestatus,
/// pc_setrestartvalue). Before, the client was shown a standing character
/// with 0 HP that could not walk. The death penalty is taken at death, 1% of
/// the EXP the level needs (jAthena //0513), and not again at the restart.
#[test]
fn logging_back_in_after_dying() {
    let mut t = T::new();
    let need = t.g.db.exp.base[39];
    let held = need / 2;
    t.lua(&format!("job_change(1); player.base_level = 40; player.base_exp = {held}; heal(9999, 9999); save_point('testmap', 10, 12)"));
    let me = t.me;
    t.lua(&format!("_rt('bl_damage', 0, {me}, 99999)"));
    assert_eq!(t.hp(), 0);
    let after = held - need / 100;
    assert_ne!(after, held - held / 100, "the level's EXP, not the EXP held");
    assert_eq!(t.g.store.chars[&t.cid].base_exp, after, "1% of the level's EXP, at death");
    t.g.handle(NetEvent::Disconnected { sid: 3 });
    // back in: login, char select, map
    let cap = t.cap.clone();
    let take = || -> Vec<Vec<u8>> { cap.0.lock().unwrap().drain(..).map(|(_, d)| d).collect() };
    t.g.handle(NetEvent::Connected { sid: 11, kind: ServerKind::Login });
    let mut w = Writer::new(0x0064);
    w.u32(20).str_fixed("battler_M", 24).str_fixed("secret", 24).u8(0);
    t.g.handle(NetEvent::Packet { sid: 11, data: w.finish() });
    let out = take();
    let acc = out.iter().find(|p| id_of(p) == 0x0069).expect("login accepted");
    let (l1, aid, l2) = (u32_at(acc, 4), u32_at(acc, 8), u32_at(acc, 12));
    t.g.handle(NetEvent::Connected { sid: 12, kind: ServerKind::Char });
    let mut w = Writer::new(0x0065);
    w.u32(aid).u32(l1).u32(l2).u16(0).u8(1);
    t.g.handle(NetEvent::Packet { sid: 12, data: w.finish() });
    let mut w = Writer::new(0x0066);
    w.u8(0);
    t.g.handle(NetEvent::Packet { sid: 12, data: w.finish() });
    t.g.handle(NetEvent::Connected { sid: 13, kind: ServerKind::Map });
    let mut w = Writer::new(0x0072);
    w.u32(aid).u32(t.cid).u32(l1).u32(0).u8(1);
    t.g.handle(NetEvent::Packet { sid: 13, data: w.finish() });
    t.g.handle(NetEvent::Packet { sid: 13, data: Writer::new(0x007d).finish() });
    let u = &t.g.world.units[&me];
    assert_eq!((u.state, u.x, u.y), (0, 10, 12), "alive, at the save point");
    let c = &t.g.store.chars[&t.cid];
    assert_eq!((c.hp, c.base_exp), (1, after), "restart HP, no second penalty");
}

/// A blind monster only notices someone next to it (Ragu Memo 2003,
/// RODataZone's Monster AI page 2004-02: sight reduced to 1 tile).
#[test]
fn a_blind_monster_sees_only_next_to_it() {
    let mut t = T::new();
    t.lua("player.base_level = 50; player.vit = 80; heal(9999, 9999)");
    let z = t.spawn(30, 25, 1015); // a Zombie five cells off
    t.lua(&format!("skill_status_change_start({z}, SC.BLIND, 1, 0, {{ tick = 60000, nodef = true }})"));
    t.lua(&format!("skill_status_change_start({z}, SC.ANKLE, 1, 0, {{ tick = 60000, nodef = true }})")); // no wandering into us
    t.run(3000);
    assert!(t.g.world.units[&z].mob().unwrap().target.is_none(), "blind: five cells is too far");
    t.lua(&format!("skill_status_change_end({z}, SC.BLIND)"));
    t.run(3000);
    assert_eq!(t.g.world.units[&z].mob().unwrap().target, Some(t.me), "seeing again: it comes");
}

/// Armour refines add two thirds of a DEF point a level (RODataZone's
/// sub-stats page of 2003-12: "+6 Plate ... 10 + 6*2/3 = 14 DEF").
#[test]
fn armour_refine_def_two_thirds() {
    let mut t = T::new();
    t.lua("player.base_level = 50");
    t.wear(2301); // Cotton Shirt
    let plain = t.st("def");
    let cid = t.cid;
    for it in t.g.store.chars.get_mut(&cid).unwrap().inventory.iter_mut().filter(|i| i.nameid == 2301) {
        it.refine = 6;
    }
    let idx = t.inv(2301);
    let mut w = Writer::new(0x00ab); // off and on again: the status is worked out anew
    w.u16(idx);
    t.send(w);
    let mut w = Writer::new(0x00a9);
    w.u16(idx).u16(0);
    t.send(w);
    assert_eq!(t.st("def"), plain + 4, "+6: four more DEF, not six");
}


impl T {
    /// Give an item with these cards in it (0x017c, one card at a time) and wear it.
    fn wear_with(&mut self, nameid: u32, cards: &[u32]) {
        self.lua(&format!("give_item({nameid}, 1)"));
        for &card in cards {
            self.lua(&format!("give_item({card}, 1)"));
            let (c, i) = (self.inv(card), self.inv(nameid));
            let mut w = Writer::new(0x017c);
            w.u16(c).u16(i);
            self.send(w);
            let ack = self.packets(0x017d);
            assert!(ack.len() == 1 && ack[0][6] == 0, "card {card} into {nameid}");
        }
        let i = self.inv(nameid);
        let mut w = Writer::new(0x00a9);
        w.u16(i).u16(0);
        self.send(w);
        let c = &self.g.store.chars[&self.cid];
        assert!(c.inventory.iter().any(|i| i.nameid == nameid && i.equip != 0), "item {nameid} was not equipped");
    }
}

/// The cards mod0631 left empty, as the client's own text describes them (scripts from jAthena
/// //0679-//0843 where they agree; tools/import/athena_import.py EXTRA_EQUIP): recovery, speed,
/// Double Attack with any weapon, HP and SP drain, close blows and spells sent back.
#[test]
fn cards_mod0631_left_empty() {
    let mut t = T::new();
    t.lua("job_change(1); player.base_level = 99; player.str = 60; player.dex = 60; player.vit = 99; heal(30000, 30000)");
    let me = t.me;

    // Moonlight Flower in Sandals: the walk a quarter faster (150 -> 112 ms a cell), DEX -2
    let dex = t.st("dex");
    t.wear_with(2402, &[4131]);
    assert_eq!(t.st("speed"), 112);
    assert_eq!(t.st("dex"), dex - 2);
    t.lua("unequip_all()");

    // Zombie in Shoes: natural HP recovery +15%, each tick (sitting: every 2 s)
    let per_tick = |t: &mut T| -> u32 {
        t.lua("player.hp = 1");
        let act = |t: &mut T, a: u8| {
            let mut w = Writer::new(0x0089);
            w.u32(0).u8(a);
            t.send(w);
        };
        act(t, 2); // sit
        let hp0 = t.hp();
        t.run(5_000);
        act(t, 3); // stand
        t.hp() - hp0
    };
    let plain = t.lua("local d = battle.get(player.id).st return 1 + math.floor(d.vit / 6) + math.floor(d.max_hp / 200)").trim().parse::<u32>().unwrap();
    let gain = per_tick(&mut t);
    assert!(gain > 0 && gain % plain == 0, "without the card: {gain} in ticks of {plain}");
    t.wear_with(2404, &[4038]);
    assert_eq!(t.st("hprecov_rate"), 115);
    let more = plain * 115 / 100;
    let gain = per_tick(&mut t);
    assert!(gain > 0 && gain % more == 0, "with the card: {gain} in ticks of {more}");
    t.lua("unequip_all()");

    // Sidewinder in a Sword: Double Attack's 5% with a weapon that is no dagger
    t.wear_with(1101, &[4117, 4115, 4134]);
    assert_eq!(t.st("double_rate"), 5);
    let pupa = t.spawn(26, 25, 1008);
    let doubles: u32 = t
        .lua(&format!("local sd, m, n = battle.get(player.id), battle.get({pupa}), 0 for i = 1, 1000 do if battle.calc_weapon_attack(sd, m, 0, 0, 0).div == 2 then n = n + 1 end end return n"))
        .trim()
        .parse()
        .unwrap();
    assert!((20..=90).contains(&doubles), "{doubles} doubles in 1000 blows");

    // Hunter Fly and Dracula in the same Sword: 3% of blows give 15% back as HP, 5% as SP
    let (hp_back, sp_back) = (t.st("hp_drain_per"), t.st("sp_drain_per"));
    assert_eq!((t.st("hp_drain_rate"), hp_back, t.st("sp_drain_rate"), sp_back), (3, 15, 3, 5));
    let r = t.lua("local sd = battle.get(player.id) local h, s = 0, 0 for i = 1, 1000 do _rt('bl_set', player.id, 'hp', 1) _rt('bl_set', player.id, 'sp', 1) battle.drain(battle.get(player.id), 1000, 0) local me = battle.get(player.id) if me.hp > 1 then h = h + 1; assert(me.hp == 151, me.hp) end if me.sp > 1 then s = s + 1; assert(me.sp == 51, me.sp) end end return h .. ' ' .. s");
    let n: Vec<u32> = r.split_whitespace().map(|x| x.parse().unwrap()).collect();
    assert!(n.iter().all(|n| (5..=70).contains(n)), "drains fired {r} times in 1000");
    // and a plain blow takes the drain path without an error
    t.lua("heal(30000, 30000)");
    let mut w = Writer::new(0x0089);
    w.u32(pupa).u8(7);
    t.send(w);
    t.run(2_000);
    t.lua(&format!("_rt('mob_kill', {pupa})"));
    t.lua("unequip_all(); heal(30000, 30000)");

    // Orc Lord in a Cotton Shirt: a close blow comes back a quarter on the striker (level 5, so
    // that the Wolf's blows land)
    t.lua("player.base_level = 5; heal(30000, 30000)");
    t.wear_with(2302, &[4135]);
    assert_eq!(t.st("short_return"), 25);
    let wolf = t.spawn(26, 25, 1013);
    let full = t.mob_hp(wolf).unwrap();
    t.lua(&format!("_rt('mob_target', {wolf}, player.id, 2)"));
    t.run(6_000);
    assert!(t.mob_hp(wolf).unwrap() < full, "the Wolf hurt by its own blows");
    t.lua(&format!("_rt('mob_kill', {wolf})"));
    t.lua("unequip_all(); player.base_level = 99; heal(30000, 30000)");

    // Maya in a Guard: three spells in ten come back on the caster; none without the card
    let bolts = |t: &mut T, m: u32| -> u32 {
        let full = t.mob_hp(m).unwrap();
        for _ in 0..40 {
            t.lua(&format!("skill_attack(BF.MAGIC, {m}, {m}, {me}, SK.MG_FIREBOLT, 1, 0) heal(30000, 30000)"));
        }
        full - t.mob_hp(m).unwrap()
    };
    let jr = t.spawn(27, 25, 1101); // Baphomet Jr.
    assert_eq!(bolts(&mut t, jr), 0, "without the card the caster is untouched");
    t.wear_with(2102, &[4146]);
    assert_eq!(t.st("magic_return"), 30);
    assert!(bolts(&mut t, jr) > 0, "some bolts came back");

    // "(Not implemented yet)", the client says: the Archer Skeleton and Doppelganger Cards do nothing
    let empty = |id: u32| t.g.db.items.get(&id).map(|i| i.athena_equip.is_empty());
    assert_eq!((empty(4094), empty(4142)), (Some(true), Some(true)));
}

/// A Hunter's trap that runs out unsprung leaves its Trap on the cell (817 skill.c:6143; an
/// iRO player of 2003: "any unused trap will be able to be picked up after it's lasting time
/// has depleted"). One that went off leaves nothing.
#[test]
fn unused_trap_leaves_its_trap() {
    let mut t = T::new();
    t.lua("job_change(11); player.base_level = 70; player.dex = 90; player.int = 40; heal(9999, 9999); give_item(1065, 10)");
    t.learn("HT_LANDMINE", 5);
    let traps_on = |t: &T, x: u16, y: u16| -> u32 {
        t.g.world.units.values().filter(|u| u.is_item() && (u.x, u.y) == (x, y)).filter_map(|u| match &u.kind {
            ro_game::world::UnitKind::Item(f) if f.item.nameid == 1065 => Some(f.item.amount),
            _ => None,
        }).sum()
    };
    // level 5 lasts 40 s; nothing walks over it
    t.cast_at("HT_LANDMINE", 25, 22);
    assert!(t.g.world.units.values().any(|u| u.skill().is_some() && (u.x, u.y) == (25, 22)), "the mine is set");
    t.run(39_000);
    assert_eq!(traps_on(&t, 25, 22), 0, "still set");
    t.run(2_000);
    assert!(!t.g.world.units.values().any(|u| u.skill().is_some()), "the mine has run out");
    assert_eq!(traps_on(&t, 25, 22), 1, "its Trap lies on the cell");
    // a mine that goes off gives nothing back
    t.run(2_000);
    t.cast_at("HT_LANDMINE", 27, 25);
    assert!(t.g.world.units.values().any(|u| u.skill().is_some() && (u.x, u.y) == (27, 25)), "the second mine is set");
    let z = t.spawn(31, 25, 1015);
    t.run(6_000);
    assert!(!t.g.world.units.values().any(|u| u.skill().is_some()), "the mine went off");
    t.kill(z);
    t.run(41_000);
    assert_eq!(traps_on(&t, 27, 25), 0, "nothing left of a sprung mine");
}

/// The Alchemist's Acid Terror, Bomb and Aid Potion (drafts from jAthena //0969 and the iRO
/// guide's Alchemist page): each takes its bottle or potion, Acid Terror hits once, Bomb burns
/// what stands in its 3x3 every second, Aid Potion heals by the potion its level throws.
#[test]
fn alchemist_acid_bomb_aid() {
    let mut t = T::new();
    t.lua("job_change(18); player.base_level = 80; player.str = 80; player.dex = 80; player.vit = 50; heal(30000, 30000)");
    for (s, lv) in [("AM_ACIDTERROR", 5), ("AM_DEMONSTRATION", 5), ("AM_POTIONPITCHER", 1)] {
        t.learn(s, lv);
    }
    t.wear(1301); // an Axe
    let me = t.me;
    let count = |t: &mut T, item: u32| -> i64 { t.lua(&format!("_rt('pc_item_count', {me}, {item})")).trim().parse::<f64>().unwrap() as i64 };

    // Acid Terror: nothing without an Acid Bottle; with one, a blow and the bottle gone
    let pupa = t.spawn(27, 25, 1008);
    let full = t.mob_hp(pupa).unwrap();
    t.cast("AM_ACIDTERROR", pupa);
    t.run(1500);
    assert_eq!(t.mob_hp(pupa), Some(full), "no bottle, no throw");
    t.lua("give_item(7136, 2)");
    let mut tries = 0;
    while t.mob_hp(pupa) == Some(full) {
        t.cast("AM_ACIDTERROR", pupa);
        t.run(2500);
        tries += 1;
        assert!(tries < 3, "Acid Terror never hit");
    }
    assert_eq!(count(&mut t, 7136), 2 - tries, "a bottle a throw");
    t.kill(pupa);

    // Bomb: a Bottle Grenade, then a 3x3 that burns a monster standing in it again and again;
    // one a cell outside is spared
    t.lua("give_item(7135, 1); player.str = 1"); // weak blows, or the first burn kills a Pupa
    let inside = t.spawn(31, 26, 1008);
    let outside = t.spawn(33, 25, 1008);
    let (hp_in, hp_out) = (t.mob_hp(inside).unwrap(), t.mob_hp(outside).unwrap());
    t.cast_at("AM_DEMONSTRATION", 31, 25);
    t.run(1500);
    assert_eq!(count(&mut t, 7135), 0);
    let mut drops = 0;
    let mut last = t.mob_hp(inside).unwrap();
    for _ in 0..6 {
        t.run(1000);
        let Some(now) = t.mob_hp(inside) else { break };
        if now < last {
            drops += 1;
        }
        last = now;
    }
    assert!(drops >= 3 || t.mob_hp(inside).is_none(), "Bomb burnt {drops} times in 6 s");
    assert!(t.mob_hp(inside).map_or(true, |h| h < hp_in));
    assert_eq!(t.mob_hp(outside), Some(hp_out), "a cell outside the 3x3");

    // Aid Potion 1 throws a Red Potion: 110% of 45-64, then the target's VIT
    t.lua("player.hp = 100");
    t.cast("AM_POTIONPITCHER", me);
    t.run(500);
    assert_eq!(t.hp(), 100, "no potion, no throw");
    t.lua("give_item(501, 1)");
    t.cast("AM_POTIONPITCHER", me);
    t.run(500);
    let healed = t.hp() as i64 - 100;
    let vit = t.st("vit");
    let (lo, hi) = (45 * 110 / 100 * (100 + vit) / 100, 64 * 110 / 100 * (100 + vit) / 100);
    assert!(healed >= lo && healed <= hi, "healed {healed}, not {lo}-{hi}");
    assert_eq!(count(&mut t, 501), 0);
    // level 5 throws a Blue Potion: SP
    t.learn("AM_POTIONPITCHER", 5);
    t.lua("give_item(505, 1); player.sp = 10");
    t.cast("AM_POTIONPITCHER", me);
    t.run(500);
    assert!(t.sp() > 10 + 40 - 1, "SP {}", t.sp());
    assert_eq!(count(&mut t, 505), 0);
}

/// Grand Cross (draft): 3 s of cast nothing breaks, a fifth of the caster's HP, then a cross of
/// 29 cells around him strikes who stands on it three times as Holy, and the caster at half.
#[test]
fn grand_cross() {
    let mut t = T::new();
    // weak, or the first blow kills a Pupa
    t.lua("job_change(14); player.base_level = 80; player.str = 1; player.int = 1; player.vit = 60; heal(30000, 30000)");
    t.learn("CR_GRANDCROSS", 1);
    t.wear(1104); // a Falchion
    let on = t.spawn(27, 25, 1008); // two cells east: on the cross
    let off = t.spawn(27, 27, 1008); // two east and two south: beside it
    let (hp_on, hp_off) = (t.mob_hp(on).unwrap(), t.mob_hp(off).unwrap());
    let hp0 = t.hp();
    t.cast_at("CR_GRANDCROSS", 25, 25);
    t.run(2700);
    assert_eq!(t.mob_hp(on), Some(hp_on), "still casting at 2.7 s");
    assert_eq!(t.hp(), hp0);
    // count the blows the Pupa on the cross takes
    let mut blows = 0;
    let mut last = hp_on;
    for _ in 0..30 {
        t.run(100);
        let Some(now) = t.mob_hp(on) else { blows += 1; break };
        if now < last {
            blows += 1;
        }
        last = now;
    }
    assert_eq!(blows, 3, "three blows");
    assert_eq!(t.mob_hp(off), Some(hp_off), "off the cross");
    // a fifth of the HP at the cast's end (costs are taken then, as mod0631 does), then his own share
    let cost = hp0 / 5;
    let hurt = hp0 - cost - t.hp();
    assert!(hurt > 0 && hurt < hp_on - last, "the caster takes less than the Pupa: {hurt}");
}

/// The Sage (drafts): Magic Rod drinks a spell aimed at him, Spell Breaker breaks a cast off,
/// Volcano and Deluge lay fields that strengthen their element on whoever stands in them (one
/// field a Sage at a time), and a Land Protector wipes what lies on it and takes no more.
#[test]
fn sage_rod_breaker_fields() {
    let mut t = T::new();
    t.lua("job_change(16); player.base_level = 80; player.int = 80; player.dex = 80; heal(30000, 30000)");
    for (s, lv) in [("SA_MAGICROD", 5), ("SA_SPELLBREAKER", 5), ("SA_VOLCANO", 5), ("SA_DELUGE", 1), ("SA_LANDPROTECTOR", 1), ("MG_FIREBOLT", 10), ("MG_SAFETYWALL", 1)] {
        t.learn(s, lv);
    }
    let me = t.me;
    let num = |t: &mut T, code: &str| -> i64 { t.lua(code).trim().parse::<f64>().unwrap() as i64 };

    // Magic Rod: a Cold Bolt at the Sage does nothing and gives him its SP (level 5: all of it)
    let pupa = t.spawn(27, 25, 1008);
    t.cast("SA_MAGICROD", me);
    t.run(100);
    assert!(t.has_sc(me, "MAGICROD"));
    t.lua("player.sp = 10");
    let (hp, cost) = (t.hp(), num(&mut t, "skill_get_sp(SK.MG_COLDBOLT, 5)"));
    assert_eq!(num(&mut t, &format!("skill_attack(BF.MAGIC, {pupa}, {pupa}, {me}, SK.MG_COLDBOLT, 5, 0)")), 0);
    assert_eq!(t.hp(), hp);
    assert_eq!(t.sp() as i64, 10 + cost);
    t.run(1500);
    assert!(!t.has_sc(me, "MAGICROD"), "1.2 s at level 5");

    // Spell Breaker (from a monster, for the test) breaks the Sage's Fire Bolt off and takes its SP
    t.lua("heal(30000, 30000)");
    let sp0 = t.sp() as i64;
    t.cast("MG_FIREBOLT", pupa);
    t.run(200);
    let bolt = num(&mut t, "skill_get_sp(SK.MG_FIREBOLT, 10)");
    assert_eq!(t.sp() as i64, sp0, "still casting");
    t.lua(&format!("skill_castend_nodamage_id({pupa}, {me}, SK.SA_SPELLBREAKER, 5)"));
    assert_eq!(t.sp() as i64, sp0 - bolt, "the bolt's SP lost");
    assert_eq!(t.lua(&format!("tostring(battle.get({me}).casting)")).trim(), "nil", "the cast broken off");
    let full = t.mob_hp(pupa);
    t.run(5000);
    assert_eq!(t.mob_hp(pupa), full, "and no bolt");

    // Volcano 5 (9x9) on the Pupa: it stands in the field, and Fire hits it 20% harder
    t.lua("give_item(715, 3); give_item(717, 1)");
    t.cast_at("SA_VOLCANO", 29, 25);
    t.run(6500);
    assert!(t.has_sc(pupa, "VOLCANO"));
    assert!(t.has_sc(me, "VOLCANO"), "the Sage at 25,25 is inside 29 +- 4");
    assert_eq!(num(&mut t, &format!("battle.calc_damage(battle.get({pupa}), 100, BF.MAGIC, battle.get({me}), 3)")), 120);
    assert_eq!(num(&mut t, &format!("battle.calc_damage(battle.get({pupa}), 100, BF.MAGIC, battle.get({me}), 1)")), 100);
    // Deluge 1 (5x5) away from both: the Sage's Volcano goes, and with it the Pupa's
    t.cast_at("SA_DELUGE", 20, 20);
    t.run(6500);
    assert!(!t.has_sc(pupa, "VOLCANO"), "one field a Sage");
    assert!(!t.has_sc(me, "VOLCANO"));

    // Land Protector 1 (7x7) over a Safety Wall wipes it; a new Safety Wall is not laid on it
    t.lua("give_item(717, 4)");
    t.cast_at("MG_SAFETYWALL", 30, 30);
    t.run(5000);
    let walls = |t: &mut T| num(t, "(function() local n = 0 for _, id in ipairs(_rt('in_area', player.map, 30, 30, 49, 49)) do end return #(skill_traps_in(player.map, 0, 0, 49, 49)) + n end)()");
    let _ = walls;
    let cells_at = |t: &mut T, x: u16, y: u16, uid: u8| -> usize {
        t.g.world.units.values().filter(|u| u.x == x && u.y == y && matches!(&u.kind, ro_game::world::UnitKind::Skill(c) if c.unit_id == uid)).count()
    };
    assert_eq!(cells_at(&mut t, 30, 30, 0x7e), 1, "the Safety Wall");
    t.cast_at("SA_LANDPROTECTOR", 31, 31);
    t.run(6500);
    assert_eq!(cells_at(&mut t, 30, 30, 0x7e), 0, "wiped");
    assert_eq!(cells_at(&mut t, 30, 30, 0x9d), 1);
    t.cast_at("MG_SAFETYWALL", 32, 32);
    t.run(5000);
    assert_eq!(cells_at(&mut t, 32, 32, 0x7e), 0, "no Safety Wall on the Land Protector");
    t.cast_at("MG_SAFETYWALL", 26, 32);
    t.run(5000);
    assert_eq!(cells_at(&mut t, 26, 32, 0x7e), 1, "beside it, yes");
}

/// A monster's armour broken by Acid Terror loses a quarter of its DEF, its weapon broken by
/// Bomb a quarter of its attack, for as long as it lives (ArislanX's iRO guide, 2004-08-20);
/// nothing breaks a player's. Acid Terror is Neutral whatever the weapon.
#[test]
fn alchemist_breaks_monster_gear() {
    let mut t = T::new();
    // STR 1, bare hands: blows too weak to kill the Orc before its armour goes
    t.lua("job_change(18); player.base_level = 80; player.str = 1; player.dex = 99; heal(30000, 30000)");
    t.learn("AM_ACIDTERROR", 5);
    let me = t.me;
    let orc = t.spawn(27, 25, 1023); // Orc Warrior: DEF 10, ATK 104-126
    let num = |t: &mut T, code: &str| -> i64 { t.lua(code).trim().parse::<f64>().unwrap() as i64 };
    let def = |t: &mut T| num(t, &format!("battle.def(battle.get({orc}))"));
    let atk = |t: &mut T| num(t, &format!("battle.atk(battle.get({orc})) + battle.atk2(battle.get({orc}))"));
    assert_eq!((def(&mut t), atk(&mut t)), (10, 230));

    // thrown until the armour goes (13% a throw at level 5)
    t.lua("give_item(7136, 200)");
    let mut throws = 0;
    while !t.has_sc(orc, "BROKENARMOR") {
        t.lua(&format!("battle.get({orc}).hp = 1400; heal(30000, 30000)")); // and the SP for the next throw
        t.cast("AM_ACIDTERROR", orc);
        t.run(1500);
        throws += 1;
        assert!(throws < 120, "Acid Terror never broke the armour");
    }
    assert_eq!(def(&mut t), 10 - 10 / 4);
    t.run(600_000);
    assert!(t.has_sc(orc, "BROKENARMOR"), "still broken ten minutes on");

    // Bomb's weapon break, a quarter off both halves of the attack
    t.lua(&format!("skill_break_mob({orc}, SC.BROKENWEAPON, 100)"));
    assert_eq!(atk(&mut t), (104 - 104 / 4) + (126 - 126 / 4));

    // a player's gear is never broken
    t.lua(&format!("skill_break_mob({me}, SC.BROKENARMOR, 100); skill_status_change_start({me}, SC.BROKENWEAPON, 1, 0)"));
    assert!(!t.has_sc(me, "BROKENARMOR") && !t.has_sc(me, "BROKENWEAPON"));

    // Neutral whatever the weapon's element
    let ele = t.lua("local w = battle.weapon_skill[SK.AM_ACIDTERROR]({ damage = 100, lv = 1, s_ele = 3, s_ele_ = 3 }); return w.s_ele * 10 + w.s_ele_");
    assert_eq!(ele.trim(), "0");
}

/// Bio Cannibalize and Sphere Mine (jAthena //0969's numbers; what they do from the iRO guide's
/// Alchemist page and euRO's text of May 2004): a Flora fights the monsters and its Alchemist
/// gets the EXP; three at most; gone when its time is up. A Marine Sphere hit by its Alchemist
/// blows up 5 s later, on the monsters only.
#[test]
fn alchemist_summons() {
    let mut t = T::new();
    t.lua("job_change(18); player.base_level = 80; player.dex = 80; heal(30000, 30000)");
    t.learn("AM_CANNIBALIZE", 5);
    t.learn("AM_SPHEREMINE", 5);
    let me = t.me;
    let count = |t: &mut T, item: u32| -> i64 { t.lua(&format!("_rt('pc_item_count', {me}, {item})")).trim().parse::<f64>().unwrap() as i64 };
    let mobs_of = |t: &T, class: u32| -> Vec<u32> {
        t.g.world.units.values().filter(|u| u.mob().is_some_and(|m| m.mob_id == class)).map(|u| u.id).collect()
    };

    // no bottle, no Flora
    t.cast_at("AM_CANNIBALIZE", 27, 27);
    t.run(500);
    assert!(mobs_of(&t, 1118).is_empty());
    t.lua("give_item(7137, 4)");
    t.cast_at("AM_CANNIBALIZE", 27, 27);
    t.run(500);
    let flora = mobs_of(&t, 1118);
    assert_eq!(flora.len(), 1);
    let flora = flora[0];
    assert_eq!(t.mob_hp(flora), Some(2210 + 5 * 200));
    assert_eq!(count(&mut t, 7137), 3);
    assert_eq!(t.lua(&format!("tostring(battle.is_enemy(battle.get({me}), battle.get({flora})))")).trim(), "false", "not the Alchemist's enemy");

    // a Thara Frog (2152 HP, passive) two cells off: the Flora (range 3) strikes it, it strikes
    // back, it dies, the EXP is the Alchemist's
    let exp0 = t.g.store.chars[&t.cid].base_exp;
    let poring = t.spawn(29, 27, 1034);
    let hp_me = t.hp();
    let mut fought_back = false;
    for _ in 0..600 {
        t.run(100);
        if t.g.world.units.get(&poring).and_then(|u| u.mob()).is_some_and(|m| m.target == Some(flora)) {
            fought_back = true;
        }
        if t.mob_hp(poring).is_none() {
            break;
        }
    }
    assert!(t.mob_hp(poring).is_none(), "the Flora killed the frog");
    assert!(fought_back, "the frog turned on the Flora");
    assert_eq!(t.hp(), hp_me, "nobody touched the Alchemist");
    assert!(t.g.store.chars[&t.cid].base_exp > exp0, "his EXP");

    // two more, then no fourth (its bottle kept)
    t.cast_at("AM_CANNIBALIZE", 20, 20);
    t.run(500);
    t.cast_at("AM_CANNIBALIZE", 21, 20);
    t.run(500);
    assert_eq!(mobs_of(&t, 1118).len(), 3);
    t.cast_at("AM_CANNIBALIZE", 22, 20);
    t.run(500);
    assert_eq!(mobs_of(&t, 1118).len(), 3);
    assert_eq!(count(&mut t, 7137), 1);
    // 30 s + 10 s a level: all three gone after 80 s, and nothing dropped
    t.run(81_000);
    assert!(mobs_of(&t, 1118).is_empty());
    assert!(!t.g.world.units.values().any(|u| u.is_item()), "a summon drops nothing");

    // Sphere Mine beside a Pupa (which stays put); the Alchemist hits it, 5 s later it blows
    t.lua("give_item(7138, 1); heal(30000, 30000)");
    let poring = t.spawn(30, 30, 1008);
    t.cast_at("AM_SPHEREMINE", 29, 30);
    t.run(500);
    let sphere = mobs_of(&t, 1142);
    assert_eq!(sphere.len(), 1);
    let sphere = sphere[0];
    assert_eq!(t.mob_hp(sphere), Some(2000));
    t.run(3000);
    assert!(t.mob_hp(sphere).is_some(), "it waits");
    let (p0, hp_me) = (t.mob_hp(poring).unwrap(), t.hp());
    t.lua(&format!("battle.damage({me}, {sphere}, 10)"));
    t.run(4000);
    assert!(t.mob_hp(sphere).is_some(), "counting down");
    assert_eq!(t.mob_hp(poring), Some(p0));
    t.run(1500);
    assert!(t.mob_hp(sphere).is_none(), "blown");
    assert!(t.mob_hp(poring).is_none_or(|h| h < p0), "the Pupa caught the blast");
    assert_eq!(t.hp(), hp_me, "the Alchemist did not");
}

/// Cast Cancel (jAthena //0969, the iRO guide's Sage page of 2004-03-27): only during a cast; the
/// spell stops and the Sage pays 2 SP and 90% (level 1) to 10% (level 5) of the spell's SP.
#[test]
fn sage_cast_cancel() {
    let mut t = T::new();
    t.lua("job_change(16); player.base_level = 80; player.int = 50; heal(30000, 30000)");
    t.learn("MG_FIREBOLT", 10);
    t.learn("SA_CASTCANCEL", 3);
    let me = t.me;
    let pupa = t.spawn(27, 25, 1008);
    let num = |t: &mut T, code: &str| -> i64 { t.lua(code).trim().parse::<f64>().unwrap() as i64 };

    // not casting: refused, nothing paid
    let sp0 = t.sp() as i64;
    t.cast("SA_CASTCANCEL", me);
    t.run(100);
    assert_eq!(t.sp() as i64, sp0);

    let bolt = num(&mut t, "skill_get_sp(SK.MG_FIREBOLT, 10)");
    t.cast("MG_FIREBOLT", pupa);
    t.run(200);
    assert_ne!(t.lua(&format!("tostring(battle.get({me}).casting)")).trim(), "nil", "casting");
    t.cast("SA_CASTCANCEL", me);
    t.run(100);
    assert_eq!(t.lua(&format!("tostring(battle.get({me}).casting)")).trim(), "nil", "cancelled");
    assert_eq!(t.sp() as i64, sp0 - 2 - bolt * 50 / 100, "2 SP and half the bolt's at level 3");
    let full = t.mob_hp(pupa);
    t.run(6000);
    assert_eq!(t.mob_hp(pupa), full, "no bolt");
    // and the Sage may cast again at once
    t.cast("MG_FIREBOLT", pupa);
    t.run(100);
    assert_ne!(t.lua(&format!("tostring(battle.get({me}).casting)")).trim(), "nil");
}

/// Frost Diver freezes 3 s a level less MDEF (the iRO guide's Mage page of 2004-04-06, mod0631),
/// not 817's 6 to 15 s: level 10 on a Pupa (MDEF 20) about 24 s.
#[test]
fn frost_diver_freezes_three_seconds_a_level() {
    let mut t = T::new();
    let me = t.me;
    let pupa = t.spawn(27, 25, 1008);
    let mdef = t.lua(&format!("battle.mdef(battle.get({pupa}))")).trim().parse::<f64>().unwrap() as i64; // before: freezing raises it
    let mut tries = 0;
    while !t.has_sc(pupa, "FREEZE") {
        t.lua(&format!("skill_additional_effect({me}, {pupa}, SK.MG_FROSTDIVER, 10, BF.MAGIC)"));
        tries += 1;
        assert!(tries < 200, "never froze");
    }
    let left = t.lua(&format!("battle.sc(battle.get({pupa}), SC.FREEZE).left")).trim().parse::<f64>().unwrap() as i64;
    let want = 30000 * (100 - mdef) / 100;
    assert!((left - want).abs() <= 200, "{left} ms, not {want}");
}

/// The Bard's and Dancer's songs (jAthena //1020 with the iRO guide's tables; behind jobs2_2):
/// played with an instrument or a whip, a 9x9 square that walks with its player, on them too
/// (battle_config.song_on_self, ours), a slow walk and no plain blow while playing, only
/// Adaptation and the arrow shots to use, SP spent as it plays, Encore at half, Dissonance
/// striking every 3 s, Don't Forget Me on a monster.
#[test]
fn bard_dancer_songs() {
    let mut t = T::new();
    t.lua("job_change(19); player.base_level = 80; player.agi = 50; player.luk = 30; player.dex = 60; heal(30000, 30000)");
    for (s, lv) in [("BA_MUSICALLESSON", 10), ("BA_WHISTLE", 10), ("BA_DISSONANCE", 5), ("BD_ADAPTATION", 1), ("BD_ENCORE", 1),
                    ("BA_FROSTJOKE", 5), ("BA_MUSICALSTRIKE", 5)] {
        t.learn(s, lv);
    }
    let me = t.me;
    let num = |t: &mut T, code: &str| -> i64 { t.lua(code).trim().parse::<f64>().unwrap() as i64 };
    let song_cells = |t: &T, uid: u8| -> Vec<(u16, u16)> {
        let mut v: Vec<(u16, u16)> = t.g.world.units.values()
            .filter(|u| matches!(&u.kind, ro_game::world::UnitKind::Skill(c) if c.unit_id == uid))
            .map(|u| (u.x, u.y)).collect();
        v.sort();
        v
    };

    // no instrument, no song
    t.cast("BA_WHISTLE", me);
    t.run(200);
    assert!(!t.has_sc(me, "DANCING"));
    t.wear(1901); // Violin
    let (flee0, speed0) = (t.st("flee"), t.st("speed"));

    // Whistle 10: 81 cells around the Bard, who stands in the song (here) and plays
    let sp0 = t.sp() as i64;
    t.cast("BA_WHISTLE", me);
    t.run(200);
    assert!(t.has_sc(me, "DANCING"));
    assert_eq!(sp0 - t.sp() as i64, 60, "Whistle 10's SP");
    let cells = song_cells(&t, 0xa7);
    assert_eq!(cells.len(), 81);
    assert_eq!((cells[0], cells[80]), ((21, 21), (29, 29)));
    assert!(t.has_sc(me, "WHISTLE"));
    // FLEE + (10 + Lessons 5 + AGI 50/10)% , walking four times slower
    assert_eq!(t.st("flee"), flee0 + flee0 * 20 / 100);
    assert_eq!(t.st("speed"), speed0 * 4);

    // the song walks with the Bard
    let mut w = Writer::new(0x0085);
    w.pos3(25, 22, 0);
    t.send(w);
    t.run(3 * 4 * speed0 as u64 + 600);
    assert_eq!(t.pos(me), (25, 22));
    let cells = song_cells(&t, 0xa7);
    assert_eq!((cells.len(), cells[0], cells[80]), (81, (21, 18), (29, 26)));
    assert!(t.has_sc(me, "WHISTLE"));

    // no plain blow, no other skill; Adaptation not in the first 5 s
    let pupa = t.spawn(27, 22, 1008);
    let full = t.mob_hp(pupa);
    let mut w = Writer::new(0x0089);
    w.u32(pupa).u8(7);
    t.send(w);
    t.run(1500);
    assert_eq!(t.mob_hp(pupa), full, "no plain blow while playing");
    t.packets(0);
    t.cast("BA_FROSTJOKE", me);
    t.run(200);
    assert!(!t.packets(0x0110).is_empty(), "Frost Joke refused while playing");
    // ... but Musical Strike, yes
    t.cast("BA_MUSICALSTRIKE", pupa);
    t.run(2500);
    assert!(t.mob_hp(pupa) < full, "Musical Strike while playing");

    // one SP every 5 s while it plays (none comes back)
    t.lua("heal(0, 30000)");
    let sp1 = t.sp() as i64;
    t.run(10_000);
    assert!((1..=3).contains(&(sp1 - t.sp() as i64)), "SP spent: {}", sp1 - t.sp() as i64);

    // Adaptation stops it: the cells go, and the Whistle with them
    t.cast("BD_ADAPTATION", me);
    t.run(200);
    assert!(!t.has_sc(me, "DANCING"));
    assert!(song_cells(&t, 0xa7).is_empty());
    assert!(!t.has_sc(me, "WHISTLE"));
    assert_eq!(t.st("speed"), speed0);

    // Encore: Whistle again, for its half (30) and Encore's own 1
    t.lua("heal(0, 30000)");
    let sp2 = t.sp() as i64;
    t.cast("BD_ENCORE", me);
    t.run(200);
    assert!(t.has_sc(me, "DANCING"));
    assert_eq!(sp2 - t.sp() as i64, 31);
    // a weapon that is no instrument stops the song
    t.lua("unequip_all()");
    t.run(200);
    assert!(!t.has_sc(me, "DANCING"));
    assert!(song_cells(&t, 0xa7).is_empty());

    // Dissonance 5 with Lessons 10: 30 + 50 + 30 a strike, at once and every 3 s, on the Pupa
    t.wear(1901);
    t.kill(pupa);
    let pupa = t.spawn(27, 22, 1008); // a fresh Pupa, 427 HP
    t.lua("heal(30000, 30000)");
    let h0 = t.mob_hp(pupa).unwrap();
    t.cast("BA_DISSONANCE", me);
    t.run(300);
    assert_eq!(t.mob_hp(pupa), Some(h0 - 110));
    t.run(3000);
    assert_eq!(t.mob_hp(pupa), Some(h0 - 220));
    assert_eq!(t.hp(), num(&mut t, "battle.get(player.id).st.max_hp") as u32, "not on its Bard");
    t.cast("BD_ADAPTATION", me);
    t.run(3000);
    t.cast("BD_ADAPTATION", me);
    t.run(200);
    assert!(!t.has_sc(me, "DANCING"));

    // the period rule: the singer is not in his own song
    t.lua("battle_config.song_on_self = false; heal(30000, 30000)");
    t.cast("BA_WHISTLE", me);
    t.run(300);
    assert!(t.has_sc(me, "DANCING") && !t.has_sc(me, "WHISTLE"));
    t.lua("battle_config.song_on_self = true");
    t.run(5000);
    t.cast("BD_ADAPTATION", me);
    t.run(200);

    // a Dancer's Don't Forget Me 10 slows a monster's walk and blows; Inc AGI does nothing under it
    t.lua("job_change(20); unequip_all(); heal(30000, 30000); player.str = 40; player.agi = 50");
    for (s, lv) in [("DC_DANCINGLESSON", 10), ("DC_DONTFORGETME", 10)] {
        t.learn(s, lv);
    }
    t.wear(1950); // Rope
    let mob = t.spawn(26, 24, 1008);
    let (lv, les) = (10, 5);
    t.cast("DC_DONTFORGETME", me);
    t.run(300);
    assert!(t.has_sc(mob, "DONTFORGETME"));
    let (slower, walk) = (3 * lv + les + 40 / 20, 2 * lv + les + 50 / 10);
    assert_eq!(num(&mut t, &format!("song_effect(SC.DONTFORGETME, battle.sc(battle.get({mob}), SC.DONTFORGETME))")), slower);
    assert_eq!(num(&mut t, &format!("select(2, song_effect(SC.DONTFORGETME, battle.sc(battle.get({mob}), SC.DONTFORGETME)))")), walk);
    assert!(!t.has_sc(me, "DONTFORGETME"), "on enemies only");
    let adelay = num(&mut t, &format!("rules.weapon_attack{{src={mob}, target={me}}}.adelay"));
    let base = num(&mut t, &format!("battle.get({mob}).db.attack_delay"));
    assert_eq!(adelay, base * (100 + slower) / 100);
}

/// Monk (a draft from jAthena //1020 and the iRO guide, scripts/rules/monk.lua): spirit
/// spheres, Absorb Spirit Sphere, Throw Spirit Sphere, Triple Attack opening the combo,
/// Chain Combo and Combo Finish in their windows, Guillotine Fist after them, Blade Stop
/// and Snap.
#[test]
fn monk_skills() {
    let mut t = T::new();
    t.lua("job_change(15); player.base_level = 80; player.dex = 60; player.agi = 40; heal(30000, 30000)");
    for (s, lv) in [("MO_CALLSPIRITS", 5), ("MO_ABSORBSPIRITS", 1), ("MO_FINGEROFFENSIVE", 5), ("MO_TRIPLEATTACK", 10),
                    ("MO_CHAINCOMBO", 5), ("MO_COMBOFINISH", 5), ("MO_EXPLOSIONSPIRITS", 5), ("MO_EXTREMITYFIST", 5),
                    ("MO_BLADESTOP", 5), ("MO_INVESTIGATE", 5), ("MO_BODYRELOCATION", 1)] {
        t.learn(s, lv);
    }
    let me = t.me;
    let num = |t: &mut T, code: &str| -> i64 { t.lua(code).trim().parse::<f64>().unwrap() as i64 };
    let balls = |t: &mut T| -> i64 { t.lua("pc_spiritball(player.id)").trim().parse::<f64>().unwrap() as i64 };
    let summon = |t: &mut T, n: i64| {
        for _ in 0..n {
            t.cast("MO_CALLSPIRITS", t.me);
            t.run(2000);
        }
    };

    // Summon Spirit Sphere: one a cast, as many as the level, then no more
    summon(&mut t, 5);
    assert_eq!(balls(&mut t), 5);
    t.packets(0);
    t.cast("MO_CALLSPIRITS", me);
    t.run(2000);
    assert_eq!(balls(&mut t), 5);
    assert!(!t.packets(0x0110).is_empty(), "a sixth sphere refused");

    // Absorb Spirit Sphere: 10 SP a sphere (the guide), all of them
    t.lua("_rt('bl_set', player.id, 'sp', 10)");
    t.cast("MO_ABSORBSPIRITS", me);
    t.run(2500);
    assert_eq!(balls(&mut t), 0);
    assert_eq!(t.sp(), 10 - 5 + 50, "5 SP spent, 50 back");
    t.lua("heal(30000, 30000)");

    // Throw Spirit Sphere 5 with three spheres: throws the three, in one blow of three hits
    summon(&mut t, 3);
    let pupa = t.spawn(28, 25, 1008);
    let full = t.mob_hp(pupa).unwrap();
    t.cast("MO_FINGEROFFENSIVE", pupa);
    t.run(5000);
    assert_eq!(balls(&mut t), 0);
    assert_eq!(num(&mut t, "finger_offensive_count[player.id]"), 3);
    assert!(t.mob_hp(pupa).unwrap() < full, "the spheres hit");
    // no spheres: refused
    let hp = t.mob_hp(pupa).unwrap();
    t.cast("MO_FINGEROFFENSIVE", pupa);
    t.run(3000);
    assert_eq!(t.mob_hp(pupa).unwrap(), hp);
    t.kill(pupa);

    // Chain Combo out of its window: nothing (a Pupa that will not die of plain blows first)
    let pupa = t.spawn(26, 25, 1008);
    if let Some(m) = t.g.world.units.get_mut(&pupa).and_then(|u| u.mob_mut()) {
        (m.max_hp, m.hp) = (1_000_000, 1_000_000);
    }
    let full = t.mob_hp(pupa).unwrap();
    t.cast("MO_CHAINCOMBO", me);
    t.run(300);
    assert_eq!(t.mob_hp(pupa).unwrap(), full);

    // Triple Attack opens the window (20% a blow at level 10): no plain blow while it lasts
    let mut w = Writer::new(0x0089);
    w.u32(pupa).u8(7);
    t.send(w);
    let mut opened = false;
    for _ in 0..400 {
        t.run(40);
        if t.has_sc(me, "COMBO") {
            opened = true;
            break;
        }
    }
    assert!(opened, "Triple Attack never came");
    assert_eq!(num(&mut t, "battle.sc(battle.get(player.id), SC.COMBO).val1"), t.skill_id("MO_TRIPLEATTACK") as i64);
    // Chain Combo strikes the one being fought, and leaves the window for Combo Finish
    t.lua("heal(30000, 30000)");
    let hp = t.mob_hp(pupa).unwrap();
    t.cast("MO_CHAINCOMBO", me);
    t.run(60);
    assert!(t.mob_hp(pupa).unwrap() < hp, "Chain Combo hit");
    assert_eq!(num(&mut t, "battle.sc(battle.get(player.id), SC.COMBO).val1"), t.skill_id("MO_CHAINCOMBO") as i64);
    // Combo Finish costs a sphere and throws the target back 5 cells
    t.lua("pc_addspiritball(player.id, 600000, 5); pc_addspiritball(player.id, 600000, 5); \
           pc_addspiritball(player.id, 600000, 5); pc_addspiritball(player.id, 600000, 5); \
           pc_addspiritball(player.id, 600000, 5)");
    t.lua("skill_status_change_start(player.id, SC.EXPLOSIONSPIRITS, 5, 0)");
    let before = t.pos(pupa);
    t.cast("MO_COMBOFINISH", me);
    t.run(60);
    assert_eq!(balls(&mut t), 4);
    let after = t.pos(pupa);
    assert_eq!(after.0 as i32 - before.0 as i32, 5, "pushed 5 cells: {before:?} -> {after:?}");
    assert_eq!(num(&mut t, "battle.sc(battle.get(player.id), SC.COMBO).val1"), t.skill_id("MO_COMBOFINISH") as i64);
    // Guillotine Fist straight after: no cast, four spheres, reaches the one thrown back; the
    // Monk lands past it, with no SP, Fury over and no SP coming back for a while
    let hp = t.mob_hp(pupa).unwrap();
    t.cast("MO_EXTREMITYFIST", pupa);
    t.run(60);
    assert_eq!(balls(&mut t), 0);
    assert!(t.mob_hp(pupa).is_none_or(|h| h < hp), "Guillotine Fist hit");
    assert_eq!(t.sp(), 0);
    assert!(!t.has_sc(me, "EXPLOSIONSPIRITS"));
    assert!(t.has_sc(me, "EXTREMITYFIST"));
    assert_eq!(t.pos(me), (after.0 + 1, after.1), "the Monk dashes through");
    t.kill(pupa);
    // without Fury, no Guillotine Fist
    t.lua("skill_status_change_end(player.id, SC.EXTREMITYFIST); heal(30000, 30000)");
    t.run(1500); // Guillotine Fist's delay
    summon(&mut t, 5);
    let (mx, my) = t.pos(me);
    let pupa = t.spawn(mx + 1, my, 1008);
    let full = t.mob_hp(pupa).unwrap();
    t.cast("MO_EXTREMITYFIST", pupa);
    t.run(5000);
    assert_eq!(t.mob_hp(pupa).unwrap(), full);
    assert_eq!(balls(&mut t), 5);
    t.kill(pupa);

    // Blade Stop: a blow while it waits holds both; the monster neither walks nor strikes;
    // Occult Impaction (level 3 and up) lets go
    let poring = t.spawn(mx + 1, my, 1002);
    t.cast("MO_BLADESTOP", me);
    t.run(100);
    assert!(t.has_sc(me, "BLADESTOP_WAIT"));
    t.lua(&format!("rules.weapon_attack({{src = {poring}, target = player.id}})"));
    assert!(t.has_sc(me, "BLADESTOP") && t.has_sc(poring, "BLADESTOP"));
    assert_eq!(balls(&mut t), 4);
    t.lua(&format!("_rt('mob_target', {poring}, player.id, 0)"));
    t.packets(0);
    t.run(4000);
    // (the Monk's HP may rise meanwhile: natural recovery. Look for the blows instead)
    let blows = t.packets(0x008a).iter().filter(|p| u32::from_le_bytes(p[2..6].try_into().unwrap()) == poring).count();
    assert_eq!(blows, 0, "the one held strikes no more");
    assert_eq!(t.pos(poring), (mx + 1, my));
    t.cast("MO_INVESTIGATE", poring);
    t.run(1500);
    assert!(!t.has_sc(me, "BLADESTOP"));
    assert!(!t.g.world.units.contains_key(&poring) || !t.has_sc(poring, "BLADESTOP"));
    t.kill(poring);
    // held again, and the monster dies: the Monk is let go within a second
    let poring = t.spawn(mx + 1, my, 1002);
    t.cast("MO_BLADESTOP", me);
    t.run(100);
    t.lua(&format!("rules.weapon_attack({{src = {poring}, target = player.id}})"));
    assert!(t.has_sc(me, "BLADESTOP"));
    t.kill(poring);
    t.run(1200);
    assert!(!t.has_sc(me, "BLADESTOP"));

    // Snap: at once to a free cell in sight, for a sphere
    let n = balls(&mut t);
    t.cast_at("MO_BODYRELOCATION", 10, 10);
    t.run(100);
    assert_eq!(t.pos(me), (10, 10));
    assert_eq!(balls(&mut t), n - 1);
}
