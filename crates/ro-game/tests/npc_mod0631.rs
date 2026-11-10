//! The NPCs taken from mod0631 in research rounds 2 and 3, on their success
//! paths: Kafra points (earned by teleporting, spent in the head office), the
//! Thief quest skills from Alcouskou, a Monster Tamer's recipe, and the
//! Novice's First Aid and Play Dead, and the Peco Peco keeper. The crawler
//! (npc_crawl.rs) only takes random paths through them. Repo scripts,
//! synthetic all-walkable maps, no client files.

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
            .find(|u| u.map == map && u.x == x && u.y == y)
            .unwrap_or_else(|| panic!("no NPC at {map} {x},{y}"))
            .id
    }

    /// Stand next to the NPC, click it, and answer its menus with `choices`
    /// in order (1-based). Returns every menu it showed.
    fn talk(&mut self, map: &str, x: u16, y: u16, choices: &[u8]) -> Vec<String> {
        let npc = self.npc_at(map, x, y);
        self.eval(&format!("warp('{map}', {x}, {})", y + 1));
        self.cap.take(3);
        let mut w = Writer::new(0x0090);
        w.u32(npc).u8(1);
        send(&mut self.g, 3, w);
        let mut choices = choices.iter();
        let mut menus = Vec::new();
        for _ in 0..200 {
            let out = self.cap.take(3);
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
        let errors: Vec<_> = self.g.notices.drain(..).filter(|n| n.contains("script error")).collect();
        assert!(errors.is_empty(), "{errors:?}");
        menus
    }
}

const MAPS: [&str; 9] =
    ["prontera", "izlude", "aldeba_in", "moc_prydb1", "payon", "alberta_in", "morocc", "prt_in", "prt_castle"];

fn setup(tag: &str) -> T {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let saves = std::env::temp_dir().join(format!("ro-offline-m631-{tag}-{}", std::process::id()));
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
        g.res.vfs.add_memory_file(&format!("data\\{m}.gat"), gat(400, 400));
        g.res.maps.insert(m.to_string());
    }
    let me = enter(&mut g, &cap);
    for m in MAPS {
        g.ensure_map(m);
    }
    g.notices.clear();
    T { g, cap, me }
}

#[test]
fn kafra_points() {
    let mut t = setup("kafra_points");
    t.eval("player.zeny = 5000");
    // Prontera, centre: Use Teleport Service -> Izlude, 900z
    let menus = t.talk("prontera", 146, 89, &[3, 1]);
    assert!(menus[1].starts_with("Izlude -> 900 z"), "{menus:?}");
    assert_eq!(t.eval("return player.map"), "izlude");
    assert_eq!(t.eval("return player.zeny"), "4100");
    assert_eq!(t.eval("return v.KF_PIT"), "90", "a tenth of the fare (mod0631)");
    // storage earns nothing (Izlude: Save, Use Storage, ...)
    t.eval("give_skill(1, 9, 0)");
    t.talk("izlude", 124, 115, &[2]);
    assert_eq!(t.eval("return v.KF_PIT"), "90");
    // the counter in the head office: 200p is too much, 100p buys 7 Sweet Potatoes
    t.eval("v.KF_PIT = 190");
    let menus = t.talk("aldeba_in", 79, 161, &[2]);
    assert!(menus[0].starts_with("100p = Sweet Potato 7 ea:200p = Sweet Potato 15 ea"), "{menus:?}");
    assert_eq!(t.eval("return count_item(516)"), "0");
    assert_eq!(t.eval("return v.KF_PIT"), "190");
    t.talk("aldeba_in", 79, 161, &[1]);
    assert_eq!(t.eval("return count_item(516)"), "7");
    assert_eq!(t.eval("return v.KF_PIT"), "90");
    // second page: Red Potions from 1100p
    t.eval("v.KF_PIT = 1100");
    let menus = t.talk("aldeba_in", 79, 161, &[10, 1]);
    assert!(menus[1].starts_with("1100p = Red Potion 7 ea"), "{menus:?}");
    assert_eq!(t.eval("return count_item(501)"), "7");
    assert_eq!(t.eval("return v.KF_PIT"), "0");
}

#[test]
fn thief_quest_skills() {
    let mut t = setup("thief_quest_skills");
    // Sono-san is gone from Morroc; Alcouskou stands in the Thief guild
    assert!(!t.g.world.units.values().any(|u| u.name == "Sono-san"));
    // a Novice only gets the explanation, no menu
    assert!(t.talk("moc_prydb1", 154, 128, &[]).is_empty());

    // Stone Fling: 2 Garlets, 2 Scells, Thief job 15
    t.eval("job_change(6); player.job_level = 14; give_item(910, 2); give_item(911, 2)");
    t.talk("moc_prydb1", 154, 128, &[4]);
    assert_eq!(t.eval("return skill_level(152)"), "0", "job level 14 is too low");
    t.eval("player.job_level = 40");
    t.talk("moc_prydb1", 154, 128, &[4]);
    assert_eq!(t.eval("return skill_level(152)"), "1");
    assert_eq!(t.eval("return count_item(910) + count_item(911)"), "0");

    // Find Stone: Zargon, Bear's Footskin, 5 Spawns
    t.eval("give_item(912, 1); give_item(948, 1); give_item(908, 5)");
    t.talk("moc_prydb1", 154, 128, &[3]);
    assert_eq!(t.eval("return skill_level(151)"), "1");

    // Back Slide: 20 Grasshopper's Legs
    t.eval("give_item(940, 20)");
    t.talk("moc_prydb1", 154, 128, &[2]);
    assert_eq!(t.eval("return skill_level(150)"), "1");
    assert_eq!(t.eval("return count_item(940)"), "0");

    // Sand Attack: 5 Fine Grit, then the bag from Payon, then back
    t.eval("give_item(7041, 5)");
    t.talk("moc_prydb1", 154, 128, &[1]);
    assert_eq!(t.eval("return v.skill_thief_1"), "1");
    assert_eq!(t.eval("return count_item(7041)"), "0");
    t.talk("payon", 30, 120, &[]);
    assert_eq!(t.eval("return count_item(7042)"), "0", "no materials yet");
    t.eval("give_item(952, 1); give_item(1055, 1); give_item(1025, 1)");
    t.talk("payon", 30, 120, &[]);
    assert_eq!(t.eval("return count_item(7042)"), "1");
    t.talk("moc_prydb1", 154, 128, &[1]);
    assert_eq!(t.eval("return skill_level(149)"), "1");
    assert_eq!(t.eval("return count_item(7042)"), "0");
    assert_eq!(t.eval("return v.skill_thief_1"), "0");
}

#[test]
fn monster_tamer() {
    let mut t = setup("monster_tamer");
    // Alberta: Monster Juice from 1 Animal Gore and 2 Apples
    t.eval("give_item(702, 1); give_item(512, 1)");
    let menus = t.talk("alberta_in", 173, 76, &[2]);
    assert!(menus[0].contains("Monster Juice") && menus[0].contains("Wild Flower"), "{menus:?}");
    assert_eq!(t.eval("return count_item(626)"), "0", "one Apple short");
    t.eval("give_item(512, 1)");
    t.talk("alberta_in", 173, 76, &[2]);
    assert_eq!(t.eval("return count_item(626)"), "1");
    assert_eq!(t.eval("return count_item(702) + count_item(512)"), "0");
    // Al De Baran: Rocker Glasses from 2 Zargons and a Ribbon (6th recipe)
    t.eval("give_item(912, 2); give_item(2208, 1)");
    t.talk("aldeba_in", 168, 175, &[7]);
    assert_eq!(t.eval("return count_item(10014)"), "1");
}

#[test]
fn novice_quest_skills() {
    let mut t = setup("novice_quest_skills");
    // First Aid: Novice job level 4 (Nami's own words, RODataZone)
    t.eval("player.job_level = 3");
    assert!(t.talk("prt_in", 234, 133, &[]).is_empty(), "job level 3: she is dozing");
    t.eval("player.job_level = 4");
    t.talk("prt_in", 234, 133, &[2]);
    assert_eq!(t.eval("return v.skill_nov"), "0", "sneaked away");
    t.talk("prt_in", 234, 133, &[1]);
    assert_eq!(t.eval("return v.skill_nov"), "1");
    // the bandages are a flag, from the Nursing Instructor in the castle
    t.talk("prt_castle", 175, 146, &[]);
    assert_eq!(t.eval("return v.skill_nov"), "2");
    t.eval("give_item(507, 3); give_item(705, 2)");
    t.talk("prt_in", 234, 133, &[1]);
    assert_eq!(t.eval("return skill_level(142)"), "0", "one Clover short");
    t.eval("give_item(705, 1)");
    t.talk("prt_in", 234, 133, &[1]);
    assert_eq!(t.eval("return skill_level(142)"), "1");
    assert_eq!(t.eval("return count_item(507) + count_item(705)"), "0");
    assert_eq!(t.eval("return v.skill_nov"), "3");

    // Play Dead: job level 7, First Aid, and a run to the castle and back
    t.talk("prt_in", 73, 87, &[]);
    assert_eq!(t.eval("return v.skill_nov"), "3", "job level 4 is too low");
    t.eval("player.job_level = 7");
    t.talk("prt_in", 73, 87, &[]);
    assert_eq!(t.eval("return v.skill_nov"), "4");
    assert_eq!(t.eval("return tmp.nov_run"), "1");
    t.talk("prt_castle", 175, 146, &[]);
    assert_eq!(t.eval("return count_item(7039)"), "1");
    assert_eq!(t.eval("return v.skill_nov"), "5");
    // the ten minutes run out: round again, without the old tag
    t.eval("do_event('nov_playdead::OnLate')");
    t.g.tick();
    assert_eq!(t.eval("return tmp.nov_run"), "0");
    t.talk("prt_in", 73, 87, &[]);
    assert_eq!(t.eval("return skill_level(143)"), "0");
    assert_eq!(t.eval("return v.skill_nov"), "4");
    assert_eq!(t.eval("return count_item(7039)"), "0");
    // in time
    t.talk("prt_castle", 175, 146, &[]);
    t.talk("prt_in", 73, 87, &[]);
    assert_eq!(t.eval("return skill_level(143)"), "1");
    assert_eq!(t.eval("return count_item(7039)"), "0");
    assert_eq!(t.eval("return v.skill_nov"), "6");
}

#[test]
fn peco_keeper() {
    let mut t = setup("peco_keeper");
    t.eval("player.zeny = 6000; job_change(1); player.job_level = 50");
    // a Swordman is turned away
    assert!(t.talk("prontera", 55, 350, &[]).is_empty());
    t.eval("job_change(7)");
    t.talk("prontera", 55, 350, &[2]);
    assert_eq!(t.eval("return player.zeny"), "6000", "never mind");
    // without Riding the money goes and no bird comes: the keeper says so
    t.talk("prontera", 55, 350, &[1, 1]);
    assert_eq!(t.eval("return player.zeny"), "3500");
    assert_eq!(t.eval("return check_option(0x20)"), "0");
    t.eval("give_skill(63, 1, 0)");
    t.talk("prontera", 55, 350, &[1, 1]);
    assert_eq!(t.eval("return player.zeny"), "1000");
    assert_eq!(t.eval("return check_option(0x20)"), "1");
    // 1000z is not enough for another
    t.talk("prontera", 55, 350, &[1]);
    assert_eq!(t.eval("return player.zeny"), "1000");
    // the Crusaders' keeper comes with the 2-2 jobs
    assert!(!t.g.world.units.values().any(|u| u.map == "prontera" && u.x == 232 && u.y == 318));
}
