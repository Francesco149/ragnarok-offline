//! A guild castle for one (scripts/npc/castle_lord.lua): the Emperium that
//! only a player's weapon hurts, taking the castle, the steward's investment
//! and Kafra, the master room's treasure on a new day, the levers, and giving
//! the castle up. Repo scripts and data, synthetic all-walkable maps.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use ro_game::model::Point;
use ro_game::{Game, GameConfig, NetEvent, Outbox, Resources, ServerKind};
use ro_grf::Vfs;
use ro_proto::{Reader, Writer};

#[derive(Clone, Default)]
struct Cap(Arc<Mutex<Vec<(u32, Vec<u8>)>>>);
impl Outbox for Cap {
    fn send(&self, sid: u32, data: Vec<u8>) {
        self.0.lock().unwrap().push((sid, data));
    }
    fn close(&self, _sid: u32) {}
}
impl Cap {
    fn take(&self, sid: u32) -> Vec<Vec<u8>> {
        let mut v = self.0.lock().unwrap();
        let (mine, rest): (Vec<_>, Vec<_>) = v.drain(..).partition(|(s, _)| *s == sid);
        *v = rest;
        mine.into_iter().map(|(_, d)| d).collect()
    }
}

fn gat(w: u32, h: u32) -> Vec<u8> {
    let mut b = b"GRAT\x01\x02".to_vec();
    b.extend_from_slice(&w.to_le_bytes());
    b.extend_from_slice(&h.to_le_bytes());
    b.resize(b.len() + (w * h * 20) as usize, 0);
    b
}

fn send(g: &mut Game, sid: u32, mut w: Writer) {
    g.handle(NetEvent::Packet { sid, data: w.finish() });
}

fn enter(g: &mut Game, cap: &Cap) -> u32 {
    g.handle(NetEvent::Connected { sid: 1, kind: ServerKind::Login });
    let mut w = Writer::new(0x0064);
    w.u32(1).str_fixed("tester", 24).str_fixed("tester", 24).u8(20);
    send(g, 1, w);
    let out = cap.take(1);
    let acc = out.iter().find(|p| p[0..2] == [0x69, 0x00]).unwrap();
    let mut r = Reader::new(acc);
    r.seek(4);
    let (l1, aid, l2) = (r.u32().unwrap(), r.u32().unwrap(), r.u32().unwrap());
    g.handle(NetEvent::Connected { sid: 2, kind: ServerKind::Char });
    let mut w = Writer::new(0x0065);
    w.u32(aid).u32(l1).u32(l2).u16(0).u8(1);
    send(g, 2, w);
    let mut w = Writer::new(0x0067);
    w.str_fixed("Tester", 24).bytes(&[9, 1, 1, 1, 9, 9]).u8(0).u16(0).u16(1);
    send(g, 2, w);
    let mut w = Writer::new(0x0066);
    w.u8(0);
    send(g, 2, w);
    let z = cap.take(2);
    let zp = z.iter().find(|p| p[0..2] == [0x71, 0x00]).unwrap();
    let cid = Reader::new(zp).u32().unwrap();
    g.handle(NetEvent::Connected { sid: 3, kind: ServerKind::Map });
    let mut w = Writer::new(0x0072);
    w.u32(aid).u32(cid).u32(l1).u32(0).u8(1);
    send(g, 3, w);
    send(g, 3, Writer::new(0x007d));
    cap.take(3);
    aid
}

struct T {
    g: Game,
    cap: Cap,
    me: u32,
}

impl T {
    fn eval(&mut self, code: &str) -> String {
        self.g.console_eval(code, self.me)
    }

    fn npc_at(&self, map: &str, x: u16, y: u16) -> u32 {
        self.g
            .world
            .units
            .values()
            .find(|u| u.map == map && u.x == x && u.y == y && !u.is_player())
            .unwrap_or_else(|| panic!("no NPC at {map} {x},{y}"))
            .id
    }

    /// Stand next to the NPC, click it, and answer its menus with `choices`
    /// in order (1-based). Returns every menu it showed and every line it said.
    fn talk(&mut self, map: &str, x: u16, y: u16, choices: &[u8]) -> (Vec<String>, Vec<String>) {
        let npc = self.npc_at(map, x, y);
        self.eval(&format!("warp('{map}', {x}, {})", y + 1));
        self.cap.take(3);
        let mut w = Writer::new(0x0090);
        w.u32(npc).u8(1);
        send(&mut self.g, 3, w);
        let mut choices = choices.iter();
        let (mut menus, mut said) = (Vec::new(), Vec::new());
        for _ in 0..200 {
            let out = self.cap.take(3);
            said.extend(out.iter().filter(|p| p[0..2] == [0xb4, 0x00]).map(|p| Reader::new(p).seek(8).str_rest()));
            let mut acted = false;
            for p in out.iter().rev() {
                match u16::from_le_bytes([p[0], p[1]]) {
                    0x00b7 => {
                        menus.push(Reader::new(p).seek(8).str_rest());
                        let c = *choices.next().unwrap_or_else(|| panic!("unexpected menu {:?}", menus.last()));
                        let mut w = Writer::new(0x00b8);
                        w.u32(npc).u8(c);
                        send(&mut self.g, 3, w);
                    }
                    0x00b5 => {
                        let mut w = Writer::new(0x00b9);
                        w.u32(npc);
                        send(&mut self.g, 3, w);
                    }
                    0x00b6 => {
                        let mut w = Writer::new(0x0146);
                        w.u32(npc);
                        send(&mut self.g, 3, w);
                    }
                    _ => continue,
                }
                acted = true;
                break;
            }
            if !acted {
                break;
            }
        }
        self.g.tick();
        let errors: Vec<_> = self.g.notices.drain(..).filter(|n| n.contains("error")).collect();
        assert!(errors.is_empty(), "{errors:?}");
        (menus, said)
    }

    /// Come in from another map, as a player does: the client loads the map and says so (0x007d).
    fn arrive(&mut self, map: &str, x: u16, y: u16) {
        self.eval("warp('aldebaran', 100, 100)");
        send(&mut self.g, 3, Writer::new(0x007d));
        self.eval(&format!("warp('{map}', {x}, {y})"));
        send(&mut self.g, 3, Writer::new(0x007d));
        self.cap.take(3);
    }

    fn at(&mut self) -> String {
        self.eval("return player.map .. ' ' .. player.x .. ',' .. player.y")
    }
}

const MAPS: [&str; 6] = ["aldeg_cas01", "aldeg_cas02", "gld_dun02", "aldebaran", "prontera", "alde_gld"];

fn setup(tag: &str) -> T {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let saves = std::env::temp_dir().join(format!("ro-offline-castle-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&saves);
    let mut vfs = Vfs::new();
    vfs.add_memory_file("data\\testmap.gat", gat(50, 50));
    let cfg = GameConfig {
        save_dir: saves,
        script_dir: root.join("scripts"),
        db_dir: root.join("data/db"),
        start_point: Point::new("testmap", 25, 25),
        ..Default::default()
    };
    let cap = Cap::default();
    let mut g = Game::new(cfg, Resources::new(vfs), Box::new(cap.clone())).expect("game");
    for m in MAPS {
        g.res.vfs.add_memory_file(&format!("data\\{m}.gat"), gat(300, 300));
        g.res.maps.insert(m.to_string());
    }
    let me = enter(&mut g, &cap);
    for m in MAPS {
        g.ensure_map(m);
    }
    // the castles' scripts ran before their maps existed: run their OnInit again
    g.reload_scripts().expect("reload");
    g.notices.clear();
    let mut t = T { g, cap, me };
    // the castles read the wall clock, which may fall in Sunday's siege: pin it to a Wednesday
    // noon and end the siege the real one started
    t.eval("castle_now = function() return os.time{year=2026, month=11, day=11, hour=12} end");
    t.eval("castle_siege_end(); g.siege_done = 0");
    t.cap.take(3);
    t
}

const EMP: &str = "Agit_aldeg_cas01::OnAgitBreak";
const BOX: &str = "Agit_aldeg_cas01::OnBoxOpen";

#[test]
fn castle_of_one() {
    let mut t = setup("one");
    let me = t.me;
    assert_eq!(t.eval("return castle_owner('aldeg_cas01')"), "0");
    assert_eq!(t.eval(&format!("return mob_count('aldeg_cas01', '{EMP}')")), "1", "an ownerless castle has its Emperium");
    assert!(t.eval("return mob_count('aldeg_cas01', 'All')").parse::<u32>().unwrap() > 1, "and its garrison");

    // the steward of an ownerless castle waits for a master
    let (_, said) = t.talk("aldeg_cas01", 218, 175, &[]);
    assert!(said.iter().any(|l| l.contains("waiting for my master")), "{said:?}");
    // no one else may pull the dungeon lever
    t.talk("aldeg_cas01", 211, 179, &[1]);
    assert_eq!(t.at(), "aldeg_cas01 211,180");

    // only a player's weapon hurts the Emperium (817 battle.c:1153)
    t.eval("warp('aldeg_cas01', 216, 26)");
    let emp = t.eval("for _, u in ipairs(nearby(5)) do if u.name == 'Emperium' then return u.id end end");
    let hit = |t: &mut T, flag: &str| {
        t.eval(&format!("return battle.calc_damage(battle.get({emp}), 100, {flag}, battle.get(player.id))"))
    };
    assert_eq!(hit(&mut t, "BF.WEAPON | BF.SHORT | BF.NORMAL"), "100");
    assert_eq!(hit(&mut t, "BF.WEAPON | BF.SHORT | BF.SKILL"), "0", "a skill does nothing to it");
    assert_eq!(hit(&mut t, "BF.MAGIC | BF.LONG | BF.SKILL"), "0");

    // breaking it takes the castle
    t.eval("player.zeny = 1000000");
    t.eval(&format!("battle.damage(player.id, {emp}, 10000000)"));
    for _ in 0..3 {
        t.g.tick();
    }
    assert_eq!(t.eval("return castle_owner('aldeg_cas01')"), t.eval("return player.char_id"));
    assert_eq!(t.eval(&format!("return mob_count('aldeg_cas01', '{EMP}')")), "0");

    // two investments a day: 5,000 then 20,000 (jAthena //0974), then no more
    t.talk("aldeg_cas01", 218, 175, &[2, 1]);
    assert_eq!(t.eval("return player.zeny"), "995000");
    t.talk("aldeg_cas01", 218, 175, &[2, 1]);
    assert_eq!(t.eval("return player.zeny"), "975000");
    let (menus, _) = t.talk("aldeg_cas01", 218, 175, &[2]);
    assert_eq!(menus.len(), 1, "a third investment is refused: {menus:?}");
    assert_eq!(t.eval("return g.castle_aldeg_cas01_eco"), "2");

    // the Kafra comes with a contract
    t.talk("aldeg_cas01", 218, 175, &[5, 1]);
    assert_eq!(t.eval("return player.zeny"), "965000");
    let (menus, _) = t.talk("aldeg_cas01", 218, 170, &[4]);
    assert_eq!(menus[0], "Open Storage:Teleport Service:Use Cart Service:Cancel");

    // no treasure on the day of the capture; the next day's comes when the master walks in
    t.arrive("aldeg_cas01", 100, 100);
    assert_eq!(t.eval(&format!("return mob_count('aldeg_cas01', '{BOX}')")), "0");
    t.eval("castle_today = function() return 'tomorrow' end");
    t.arrive("aldeg_cas01", 100, 101);
    // commerce 2: 2 + 7 / 10 = 2 of the first box, 2 + 2 / 10 = 2 of the second
    assert_eq!(t.eval(&format!("return mob_count('aldeg_cas01', '{BOX}')")), "4");
    assert_eq!(t.eval("return g['castle_aldeg_cas01_te']"), "0", "and investments start again");
    t.arrive("aldeg_cas01", 100, 102);
    assert_eq!(t.eval(&format!("return mob_count('aldeg_cas01', '{BOX}')")), "4", "once a day");

    // the master room and its lever out, the lever into the guild dungeon
    t.talk("aldeg_cas01", 218, 175, &[6, 1]);
    assert_eq!(t.at(), "aldeg_cas01 119,223");
    t.talk("aldeg_cas01", 123, 223, &[1]);
    assert_eq!(t.at(), "aldeg_cas01 218,176");
    t.talk("aldeg_cas01", 211, 179, &[1]);
    assert_eq!(t.at(), "gld_dun02 32,122");

    // giving it up: the treasure goes, the Emperium and the garrison come back
    t.eval("kill_monster('aldeg_cas01', 'All')");
    t.eval("castle_today = function() return 'the day after' end");
    t.arrive("aldeg_cas01", 100, 100);
    assert_eq!(t.eval(&format!("return mob_count('aldeg_cas01', '{BOX}')")), "4");
    assert_eq!(t.eval("return mob_count('aldeg_cas01', 'All')"), "4", "only the treasure is left");
    t.talk("aldeg_cas01", 218, 175, &[7, 1, 2]);
    assert_eq!(t.eval("return castle_owner('aldeg_cas01')"), "0");
    assert_eq!(t.eval(&format!("return mob_count('aldeg_cas01', '{BOX}')")), "0");
    assert_eq!(t.eval(&format!("return mob_count('aldeg_cas01', '{EMP}')")), "1");
    assert!(t.eval("return mob_count('aldeg_cas01', 'All')").parse::<u32>().unwrap() > 1, "the garrison is back");
    let _ = me;
}

/// The castle's flag on its grounds says who holds it and takes the master
/// home; the master recovers twice as fast in the castle.
#[test]
fn flag_and_home_regen() {
    let mut t = setup("flag");
    let (_, said) = t.talk("alde_gld", 257, 153, &[]);
    assert!(said.iter().any(|l| l.contains("Kingdom of Al De Baran")), "{said:?}");
    assert!(said.iter().any(|l| l.contains("isn't taken by any guild yet")), "{said:?}");

    // someone else's castle: the flag names its master
    t.eval("g.castle_aldeg_cas01 = 999999; g['castle_aldeg_cas01_name$'] = 'Somebody'");
    let (menus, said) = t.talk("alde_gld", 257, 153, &[]);
    assert!(menus.is_empty());
    assert!(said.iter().any(|l| l.contains("Its master is ^ff0000Somebody")), "{said:?}");
    t.eval("g.castle_aldeg_cas01 = 0; g['castle_aldeg_cas01_name$'] = ''");

    // taken: the master recovers faster there
    t.arrive("aldeg_cas01", 216, 26);
    assert_eq!(t.eval("return tmp.regen_rate"), "0");
    let emp = t.eval("for _, u in ipairs(nearby(5)) do if u.name == 'Emperium' then return u.id end end");
    t.eval(&format!("battle.damage(player.id, {emp}, 10000000)"));
    for _ in 0..3 {
        t.g.tick();
    }
    assert_eq!(t.eval("return castle_owner('aldeg_cas01')"), t.eval("return player.char_id"));
    assert_eq!(t.eval("return tmp.regen_rate"), "200");
    let iv = |t: &mut T, rate: u32| {
        t.eval(&format!("local r = rules.regen({{max_hp = 1000, max_sp = 100, vit = 0, int = 0, regen_rate = {rate}}}) return r.hp_interval .. ' ' .. r.sp_interval"))
    };
    assert_eq!(iv(&mut t, 100), "4000 8000");
    assert_eq!(iv(&mut t, 200), "2000 4000");

    // away from it, the usual pace; the flag takes the master home, beside the steward
    t.arrive("alde_gld", 250, 150);
    assert_eq!(t.eval("return tmp.regen_rate"), "0");
    let (menus, _) = t.talk("alde_gld", 257, 153, &[1]);
    assert_eq!(menus, ["Yes:No"]);
    assert_eq!(t.at(), "aldeg_cas01 218,176");
    send(&mut t.g, 3, Writer::new(0x007d));
    assert_eq!(t.eval("return tmp.regen_rate"), "200");

    // and the server's clock follows: two of the usual 4 s ticks in 4 s (vit 1: 1 HP a tick)
    t.eval("kill_monster('aldeg_cas01', 'All')");
    t.eval("player.hp = 1");
    t.run(4100);
    assert_eq!(t.eval("return player.hp"), "3");

    // given up: no longer
    t.talk("aldeg_cas01", 218, 175, &[7, 1, 2]);
    t.eval("player.hp = 1");
    t.run(4100);
    assert_eq!(t.eval("return player.hp"), "2");
    assert_eq!(t.eval("return tmp.regen_rate"), "0");
}

/// The boxes drop Ragu Memo's jRO table (data/db/treasure.toml), not mod0631's.
#[test]
fn treasure_boxes_drop_the_jro_table() {
    let t = setup("box");
    let first = t.g.db.mob(1324).unwrap();
    assert_eq!(first.drops.len(), 16);
    assert!(first.drops.iter().any(|d| d.item == 732 && d.rate == 4000), "3 Carat Diamond 1/2.5");
    assert!(first.drops.iter().any(|d| d.item == 7073 && d.rate == 40), "Freyja's Jewel 1/250");
    let last = t.g.db.mob(1363).unwrap();
    assert!(last.drops.iter().any(|d| d.item == 7092 && d.rate == 40), "Prontera 5's material");
}

impl T {
    /// Break an ownerless castle's Emperium as the player.
    fn capture(&mut self, map: &str, x: u16, y: u16) {
        self.eval(&format!("warp('{map}', {x}, {})", y + 2));
        let emp = self.emperium(map);
        self.eval(&format!("battle.damage(player.id, {emp}, 10000000)"));
        self.ticks(3);
    }

    fn emperium(&mut self, map: &str) -> String {
        self.eval(&format!("return mob_ids('{map}', 'Agit_{map}::OnAgitBreak')[1]"))
    }

    fn ticks(&mut self, n: u32) {
        for _ in 0..n {
            self.g.tick();
        }
    }

    /// Let `ms` of game time pass.
    fn run(&mut self, ms: u64) {
        for _ in 0..ms / 50 {
            self.g.skew(50);
            self.g.tick();
        }
    }

    /// Ids of the monsters on `map` on siege side `side`.
    fn side(&mut self, map: &str, side: u8) -> Vec<u32> {
        let s = self.eval(&format!(
            "local t = {{}} for _, id in ipairs(mob_ids('{map}', 'All')) do local u = battle.get(id) \
             if u and u.side == {side} then t[#t + 1] = id end end return table.concat(t, ',')"
        ));
        s.split(',').filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect()
    }
}

/// A held castle under attack: its old garrison comes back at the siege hour
/// and goes for the Emperium; guardians and defence help; a broken Emperium
/// loses the castle, which can be taken back before the siege is over.
#[test]
fn counter_siege() {
    let mut t = setup("siege");
    t.eval("player.zeny = 1000000");
    t.capture("aldeg_cas01", 216, 24);
    assert_eq!(t.eval("return castle_owner('aldeg_cas01')"), t.eval("return player.char_id"));
    t.eval("kill_monster('aldeg_cas01', 'All')");
    assert_eq!(t.emperium("aldeg_cas01"), "nil", "a held castle has no Emperium outside a siege");

    // defence 1 (10,000z), a Soldier Guardian in the first slot (10,000z)
    t.talk("aldeg_cas01", 218, 175, &[3, 1]);
    assert_eq!(t.eval("return player.zeny .. ' ' .. g.castle_aldeg_cas01_def"), "990000 1");
    let (menus, _) = t.talk("aldeg_cas01", 218, 175, &[4, 1, 1]);
    assert!(menus[1].starts_with("Soldier Guardian - Installable:Archer Guardian - Installable"), "{menus:?}");
    let guards = t.side("aldeg_cas01", 2);
    assert_eq!(guards.len(), 1);
    let g = guards[0];
    // Soldier Guardian 15,670 HP + 2,000 a defence level
    assert_eq!(t.eval(&format!("local u = battle.get({g}) return u.max_hp .. ' ' .. u.class")), "17670 1287");
    let (menus, _) = t.talk("aldeg_cas01", 218, 175, &[4, 1]);
    assert!(menus[1].starts_with("Soldier Guardian - (17670 / 17670)"), "{menus:?}");

    // an MVP of the old garrison with a summoned slave (no event: seen in the client,
    // Garm's Sasquatches stayed by the Emperium out of the siege, 2026-11-15)
    let garm = t.eval("return monster('aldeg_cas01', 100, 100, '--ja--', 1252, 1)[1]");
    let slave = t.eval(&format!("return _rt('mob_spawn', 'aldeg_cas01', 101, 100, 1243, {garm})"));
    assert_ne!(slave, "nil");

    // the castle's halves joined only by portals, as on the real maps: a wall across
    // row 100; the Emperium (216,24) lies south of it, the portal 206,188 leads to 216,50
    t.eval("for x = 0, 299 do _rt('set_cell', 'aldeg_cas01', x, 100, false) end");

    // the siege: a defended Emperium (68,430 + 2,000), the garrison as attackers
    t.eval("castle_siege_start(castle_now() + 7200)");
    t.ticks(3);
    let gone = |t: &mut T, id: &str| t.eval(&format!("return battle.get({id}) == nil")) == "true";
    assert!(gone(&mut t, &garm) && gone(&mut t, &slave), "the old garrison goes, slaves and all");
    let emp = t.emperium("aldeg_cas01");
    assert_eq!(t.eval(&format!("local u = battle.get({emp}) return u.max_hp .. ' ' .. u.side")), "70430 2");
    let attackers = t.side("aldeg_cas01", 1);
    assert!(attackers.len() > 20, "the garrison attacks: {}", attackers.len());
    // none starts next to it: they come along the corridors, at random times in the first hour
    let near = t.eval(&format!(
        "local n = 0 for _, id in ipairs(mob_ids('aldeg_cas01', 'All')) do local u = battle.get(id) \
         if u.side == 1 and math.max(math.abs(u.x - 216), math.abs(u.y - 24)) < 20 then n = n + 1 end end return n"
    ));
    assert_eq!(near, "0");
    // the master's own blows do nothing to it
    assert_eq!(t.eval(&format!("return battle.calc_damage(battle.get({emp}), 100, BF.WEAPON | BF.SHORT, battle.get(player.id))")), "0");

    // an attacker next to the Emperium goes for it; one next to the guardian is fought
    // (monsters think only while a player is on their map)
    t.arrive("aldeg_cas01", 100, 100);
    let raydric = |t: &mut T, x: u16, y: u16| -> String {
        t.eval(&format!("local id = monster('aldeg_cas01', {x}, {y}, '--ja--', 1163, 1)[1] mob_side(id, 1) return id"))
    };
    let r1 = raydric(&mut t, 217, 24);
    let gx = t.eval(&format!("local u = battle.get({g}) return u.x .. ',' .. u.y"));
    let (gx, gy) = gx.split_once(',').map(|(a, b)| (a.parse::<u16>().unwrap(), b.parse::<u16>().unwrap())).unwrap();
    let r2 = raydric(&mut t, gx + 1, gy);
    // one far off, setting out now, walks toward the Emperium, through the portal
    let r3 = raydric(&mut t, 216, 120);
    t.eval(&format!("mob_march({r3}, 0)"));
    // a slave summoned by an attacker attacks too
    let s3 = t.eval(&format!("return _rt('mob_spawn', 'aldeg_cas01', 217, 120, 1243, {r3})"));
    assert_eq!(t.eval(&format!("return battle.get({s3}).side")), "1");
    t.run(15000);
    let hp = |t: &mut T, id: &str| t.eval(&format!("local u = battle.get({id}) return u and (u.max_hp - u.hp) or 'dead'"));
    assert_ne!(hp(&mut t, &emp), "0", "the Emperium is hit");
    assert_ne!(hp(&mut t, &r2), "0", "the guardian fights back");
    let y3: i32 = t.eval(&format!("local u = battle.get({r3}) return u and u.y or 0")).parse().unwrap();
    assert!(y3 < 100, "the marcher is on its way: y {y3}");
    let _ = r1;

    // the attackers break it: the castle is theirs, its guardians and Kafra gone
    t.eval(&format!("battle.damage({r1}, {emp}, 10000000)"));
    t.ticks(3);
    assert_eq!(t.eval("return castle_owner('aldeg_cas01')"), "0");
    assert!(t.side("aldeg_cas01", 2).is_empty());
    assert!(t.side("aldeg_cas01", 1).is_empty(), "the attackers are the garrison again");
    assert_eq!(t.eval("return g.castle_aldeg_cas01_gd1"), "0");

    // taken back before the siege ends: a new Emperium to hold, the rest attack it
    t.capture("aldeg_cas01", 216, 24);
    assert_eq!(t.eval("return castle_owner('aldeg_cas01')"), t.eval("return player.char_id"));
    let emp = t.emperium("aldeg_cas01");
    assert_eq!(t.eval(&format!("return battle.get({emp}).side")), "2");
    assert!(!t.side("aldeg_cas01", 1).is_empty());

    // the siege ends with the Emperium standing: the castle held, the attackers gone
    t.eval("castle_siege_end()");
    t.ticks(2);
    assert_eq!(t.eval("return castle_owner('aldeg_cas01')"), t.eval("return player.char_id"));
    assert_eq!(t.emperium("aldeg_cas01"), "nil");
    assert!(t.side("aldeg_cas01", 1).is_empty());
}

/// Sieges come on Sunday at 18:00 for two hours (euRO, April 2004).
#[test]
fn siege_on_sunday_evening() {
    let mut t = setup("sunday");
    let at = |t: &mut T, h: u32, m: u32| {
        t.eval(&format!("castle_now = function() return os.time{{year=2026, month=11, day=15, hour={h}, min={m}}} end"));
        t.eval("do_event('castle_siege::OnTimer60000')");
        t.ticks(2);
        t.eval("return g.siege_on")
    };
    assert_eq!(at(&mut t, 17, 59), "0");
    assert_eq!(at(&mut t, 18, 0), "1");
    assert_eq!(at(&mut t, 19, 30), "1");
    assert_eq!(at(&mut t, 20, 0), "0");
    assert_eq!(at(&mut t, 20, 1), "0", "once a window");
    // a Saturday evening is quiet
    t.eval("castle_now = function() return os.time{year=2026, month=11, day=14, hour=18, min=30} end");
    t.eval("do_event('castle_siege::OnTimer60000')");
    t.ticks(2);
    assert_eq!(t.eval("return g.siege_on"), "0");
}


