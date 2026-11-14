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

const MAPS: [&str; 5] = ["aldeg_cas01", "aldeg_cas02", "gld_dun02", "aldebaran", "prontera"];

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
    T { g, cap, me }
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
    t.talk("aldeg_cas01", 218, 175, &[3, 1]);
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
    t.talk("aldeg_cas01", 218, 175, &[4, 1]);
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
    t.talk("aldeg_cas01", 218, 175, &[5, 1, 2]);
    assert_eq!(t.eval("return castle_owner('aldeg_cas01')"), "0");
    assert_eq!(t.eval(&format!("return mob_count('aldeg_cas01', '{BOX}')")), "0");
    assert_eq!(t.eval(&format!("return mob_count('aldeg_cas01', '{EMP}')")), "1");
    assert!(t.eval("return mob_count('aldeg_cas01', 'All')").parse::<u32>().unwrap() > 1, "the garrison is back");
    let _ = me;
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
