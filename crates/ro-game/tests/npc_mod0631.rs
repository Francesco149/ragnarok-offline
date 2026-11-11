//! The NPCs taken from mod0631 in research rounds 2 and 3, on their success
//! paths: Kafra points (earned by teleporting, spent in the head office), the
//! Thief quest skills from Alcouskou, a Monster Tamer's recipe, and the
//! Novice's First Aid and Play Dead, the Peco Peco keeper, the dye chain, Turtle
//! Island, the doomed swords and the doll child. The crawler
//! (npc_crawl.rs) only takes random paths through them. Also the Wizard test
//! (817's script on the Turtle Island patch's job_wiz), which needs right
//! answers and a clock. Repo scripts,
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
    /// What the NPC of the last `talk` said, line by line.
    said: Vec<String>,
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
        let mut choices = choices.iter();
        self.talk_by(map, x, y, |menu| *choices.next().unwrap_or_else(|| panic!("unexpected menu {menu:?}")))
    }

    /// The same, with the closure choosing from each menu it is shown.
    fn talk_by(&mut self, map: &str, x: u16, y: u16, mut pick: impl FnMut(&str) -> u8) -> Vec<String> {
        let npc = self.npc_at(map, x, y);
        self.eval(&format!("warp('{map}', {x}, {})", y + 1));
        self.cap.take(3);
        let mut w = Writer::new(0x0090);
        w.u32(npc).u8(1);
        send(&mut self.g, 3, w);
        let mut menus = Vec::new();
        self.said.clear();
        for _ in 0..200 {
            let out = self.cap.take(3);
            self.said.extend(out.iter().filter(|p| p[0..2] == [0xb4, 0x00]).map(|p| Reader::new(p).seek(8).str_rest()));
            let mut acted = false;
            for p in out.iter().rev() {
                match u16::from_le_bytes([p[0], p[1]]) {
                    0x00b7 => {
                        menus.push(Reader::new(p).seek(8).str_rest());
                        let c = pick(menus.last().unwrap());
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

const MAPS: [&str; 25] = [
    "prontera", "izlude", "aldeba_in", "moc_prydb1", "payon", "alberta_in", "morocc", "prt_in", "prt_castle", "geffen",
    "geffen_in", "morocc_in", "alberta", "tur_dun01", "tur_dun02", "izlude_in", "alb2trea", "pay_fild08",
    "gef_tower", "job_wiz", "new_zone02", "new_zone03", "new_zone04", "payon_in02",
    "izlu2dun",
];

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
    T { g, cap, me, said: Vec::new() }
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

#[test]
fn dye_chain() {
    let mut t = setup("dye_chain");
    t.eval("player.zeny = 20000; give_item(970, 2); give_item(971, 1); give_item(972, 1); give_item(713, 3)");
    // the chemist does not know us yet
    assert!(t.talk("geffen_in", 141, 140, &[]).is_empty());
    assert_eq!(t.eval("return count_item(973)"), "0");
    // Louitz sees the solutions and names him; Aure Dupon says where he lives
    let menus = t.talk("alberta_in", 133, 55, &[2]);
    assert_eq!(menus[0], "Talk:Talk about the solutions:Cancel");
    assert_eq!(t.eval("return v.MOLGENSTAIN"), "1");
    t.talk("geffen", 181, 114, &[2]);
    assert_eq!(t.eval("return v.MOLGENSTAIN"), "2");
    // first visit: he only giggles; then a Counteragent (3000z) and a Mixture (4000z)
    t.talk("geffen_in", 141, 140, &[2]);
    assert_eq!(t.eval("return v.MOLGENSTAIN"), "3");
    assert_eq!(t.eval("return count_item(973)"), "0");
    t.talk("geffen_in", 141, 140, &[2, 1, 1]);
    t.talk("geffen_in", 141, 140, &[2, 2, 1]);
    assert_eq!(t.eval("return count_item(973) .. ' ' .. count_item(974) .. ' ' .. player.zeny"), "1 1 13000");
    assert_eq!(t.eval("return count_item(970) + count_item(971) + count_item(972)"), "0");
    assert_eq!(t.eval("return count_item(713)"), "1");
    // talking to Louitz again does not undo the introduction
    t.eval("give_item(971, 1)");
    t.talk("alberta_in", 133, 55, &[2]);
    assert_eq!(t.eval("return v.MOLGENSTAIN"), "3");
    // Morroc: Orange (5th) needs 20 Red and 20 Yellow Herbs, both solvents, a bottle, 5000z
    t.eval("give_item(507, 20); give_item(508, 19)");
    t.talk("morocc_in", 146, 99, &[2, 5, 1]);
    assert_eq!(t.eval("return count_item(980)"), "0", "one Yellow Herb short");
    t.eval("give_item(508, 1)");
    t.talk("morocc_in", 146, 99, &[2, 5, 1]);
    assert_eq!(t.eval("return count_item(980) .. ' ' .. player.zeny"), "1 8000");
    assert_eq!(t.eval("return count_item(507) + count_item(508) + count_item(973) + count_item(974) + count_item(713)"), "0");
    // Prontera: the hairdresser takes the dye and 5000z; orange is the client's hair palette 3
    t.talk("prt_in", 243, 168, &[1, 4]);
    assert_eq!(t.eval("return player.hair_color .. ' ' .. count_item(980) .. ' ' .. player.zeny"), "3 0 3000");
    // no dye, no dyeing; and the clothes dyer is busy
    t.talk("prt_in", 243, 168, &[1, 1]);
    assert_eq!(t.eval("return player.hair_color .. ' ' .. player.zeny"), "3 3000");
    t.talk("prt_in", 284, 168, &[2]);
    assert_eq!(t.eval("return player.clothes_color"), "0");
}

/// 817's sailor sold passage to Amatsu and Gonryun, which this client does
/// not have: the fare went and the ship did not.
#[test]
fn alberta_sailor() {
    let mut t = setup("alberta_sailor");
    t.eval("player.zeny = 1000");
    let menus = t.talk("alberta", 189, 151, &[2]);
    // 817's 800z passage to Turtle Island is gone: Gotanblue sails there (turtle_island)
    assert_eq!(menus[0], "Sunken Ship -> 250 Zeny.:Izlude Marina -> 500 Zeny:Quit");
    assert_eq!(t.eval("return player.map .. ' ' .. player.zeny"), "izlude 500");
}

#[test]
fn turtle_island() {
    let mut t = setup("turtle_island");
    t.eval("player.zeny = 25000");
    // Gotanblue has nothing to offer a stranger
    assert!(t.talk("alberta", 247, 122, &[]).is_empty());
    // the old man in the inn sends us to him
    t.talk("alberta_in", 23, 104, &[2]);
    assert_eq!(t.eval("return v.turtle_npc"), "1");
    // thinking better of it costs nothing; then 10000z to the beach
    t.talk("alberta", 247, 122, &[2, 2]);
    assert_eq!(t.eval("return player.map .. ' ' .. player.zeny"), "alberta 25000");
    let menus = t.talk("alberta", 247, 122, &[2, 1]);
    assert_eq!(menus[1], "Turtle Island -> 10000 z:Never mind");
    assert_eq!(t.eval("return player.map .. ' ' .. player.x .. ' ' .. player.y .. ' ' .. player.zeny"), "tur_dun01 157 39 15000");
    // the things on the island are scenery to someone who has not read the logbook
    t.talk("tur_dun01", 160, 81, &[]);
    assert_eq!(t.eval("return count_item(513)"), "0");
    // he waits on the beach and takes us home for nothing
    t.talk("tur_dun01", 165, 29, &[1]);
    assert_eq!(t.eval("return player.map .. ' ' .. player.x .. ' ' .. player.y .. ' ' .. player.zeny"), "alberta 241 115 15000");
    // 9999z is not enough
    t.eval("player.zeny = 9999");
    t.talk("alberta", 247, 122, &[2, 1]);
    assert_eq!(t.eval("return player.map .. ' ' .. player.zeny"), "alberta 9999");

    // the logbook: its first reading is the wreck, the next starts one of two errands
    t.talk("alberta", 248, 94, &[]);
    assert_eq!(t.eval("return v.kame_quest"), "1");
    let (mut bananas, mut gems) = (0, 0);
    for _ in 0..40 {
        t.eval("v.kame_quest = 1");
        t.talk("alberta", 248, 94, &[]);
        match t.eval("return v.kame_quest").as_str() {
            "2" => {
                // the key fits the rock only
                t.talk("tur_dun02", 132, 251, &[]);
                assert_eq!(t.eval("return v.kame_quest"), "2");
                t.talk("tur_dun01", 160, 81, &[]);
                bananas += 1;
                assert_eq!(t.eval("return count_item(513)"), (bananas * 5).to_string());
            }
            "3" => {
                // root, crystal mound, pillar, in that order
                t.talk("tur_dun02", 49, 156, &[]);
                assert_eq!(t.eval("return v.kame_quest"), "3");
                t.talk("tur_dun02", 132, 251, &[]);
                t.talk("tur_dun02", 46, 125, &[]);
                assert_eq!(t.eval("return v.kame_quest"), "5");
                t.talk("tur_dun02", 49, 156, &[]);
                gems += 1;
                assert_eq!(t.eval("return count_item(716)"), gems.to_string());
            }
            other => panic!("the logbook left kame_quest at {other}"),
        }
        assert_eq!(t.eval("return v.kame_quest"), "6");
        // a finished errand: the things are scenery again
        t.talk("tur_dun01", 160, 81, &[]);
        t.talk("tur_dun02", 49, 156, &[]);
        assert_eq!(t.eval("return count_item(513) .. ' ' .. count_item(716)"), format!("{} {gems}", bananas * 5));
        if bananas > 0 && gems > 0 {
            break;
        }
    }
    assert!(bananas > 0 && gems > 0, "both errands should come up in 40 readings");
    // as in mod0631, reading the logbook after an errand begins again
    t.talk("alberta", 248, 94, &[]);
    assert_eq!(t.eval("return v.kame_quest"), "1");
    // the scholar and the boy only talk
    t.talk("alberta", 248, 89, &[1]);
    t.talk("alberta", 91, 70, &[]);
    t.talk("alberta_in", 23, 104, &[1, 1]);
}

#[test]
fn moving_hp_recovery() {
    let mut t = setup("moving_hp");
    t.eval("job_change(1); player.job_level = 35; give_item(713, 520); give_item(1058, 1)");
    // without the doll he asks again
    assert!(t.talk("izlude_in", 175, 130, &[]).is_empty());
    assert_eq!(t.eval("return skill_level(144)"), "0");
    t.eval("give_item(752, 1)");
    t.talk("izlude_in", 175, 130, &[2]);
    assert_eq!(t.eval("return skill_level(144) .. ' ' .. count_item(713)"), "0 520");
    t.talk("izlude_in", 175, 130, &[1]);
    // what he asked for is what he takes; no armour is needed, the Moth Wings stay
    assert_eq!(t.eval("return skill_level(144) .. ' ' .. count_item(713) .. ' ' .. count_item(752) .. ' ' .. count_item(1058)"), "1 20 0 1");
    assert!(t.talk("izlude_in", 175, 130, &[]).is_empty());
}

#[test]
fn cursed_swords() {
    let mut t = setup("cursed_swords");
    let state = "return v.Ma_sword .. ' ' .. v.ex_sword .. ' ' .. v.orga_sword .. ' ' .. v.mys_sword";
    // strangers get silence from the smith and from Gatu
    assert!(t.talk("pay_fild08", 218, 283, &[]).is_empty());
    t.talk("prt_in", 163, 12, &[]);
    assert_eq!(t.eval(state), "0 0 0 0");
    // a rumour, then Gatu, then the smith names what he has heard of
    t.talk("morocc", 281, 178, &[]);
    assert_eq!(t.eval(state), "1 0 1 0");
    assert!(t.talk("pay_fild08", 218, 283, &[]).is_empty());
    t.talk("prt_in", 163, 12, &[]);
    assert_eq!(t.eval(state), "2 0 1 0");
    let menus = t.talk("pay_fild08", 218, 283, &[1]);
    assert_eq!(menus, ["Grimtooth"]);
    assert_eq!(t.eval(state), "2 0 2 0");
    // a second and third rumour do not send us back to Gatu, nor undo the recipe
    t.talk("izlude_in", 173, 88, &[]);
    t.talk("prontera", 197, 188, &[]);
    t.talk("morocc", 281, 178, &[]);
    assert_eq!(t.eval(state), "2 1 2 1");
    // one ruby short: nothing is taken
    t.eval("give_item(724, 4); give_item(7023, 10); give_item(7002, 100); give_item(7022, 1); give_item(714, 3)");
    let menus = t.talk("pay_fild08", 218, 283, &[2]);
    assert_eq!(menus, ["Executioner:Grimtooth:Mysteltain"]);
    assert_eq!(t.eval("return count_item(1237) .. ' ' .. count_item(7002)"), "0 100");
    t.eval("give_item(724, 1)");
    t.talk("pay_fild08", 218, 283, &[2]);
    assert_eq!(
        t.eval("return count_item(1237) .. ' ' .. count_item(724) .. ' ' .. count_item(7023) .. ' ' .. count_item(7002) .. ' ' .. count_item(7022) .. ' ' .. count_item(714)"),
        "1 0 0 0 0 2"
    );
    // the other two: the tale first, the sword on the next visit
    t.eval("give_item(7017, 2); give_item(1008, 3); give_item(7024, 10); give_item(609, 50)");
    t.eval("give_item(7018, 2); give_item(7019, 1); give_item(7020, 1); give_item(7021, 1)");
    t.talk("pay_fild08", 218, 283, &[1]);
    t.talk("pay_fild08", 218, 283, &[3]);
    assert_eq!(t.eval("return count_item(1169) .. ' ' .. count_item(1138)"), "0 0");
    t.talk("pay_fild08", 218, 283, &[1]);
    t.talk("pay_fild08", 218, 283, &[3]);
    // one Young Twig is taken, not mod0631's hundred
    assert_eq!(
        t.eval("return count_item(1169) .. ' ' .. count_item(1138) .. ' ' .. count_item(7018) .. ' ' .. count_item(609) .. ' ' .. count_item(714)"),
        "1 1 1 0 0"
    );
}

#[test]
fn doll_child() {
    let mut t = setup("doll_child");
    // only talking; then offering a doll we do not have
    t.talk("alberta", 117, 135, &[1]);
    let menus = t.talk("alberta", 117, 135, &[2, 9]);
    assert_eq!(menus[1], "Poring Doll:Chonchon Doll:Puppet:Rocker Doll:Spore Doll:Osiris Doll:Baphomet Doll:Racoon Doll:Yoyo Doll");
    assert_eq!(t.eval("return count_item(608)"), "0");
    // each doll for its sweet
    let trades = [(741, 529, 1), (742, 530, 1), (740, 530, 2), (752, 532, 7), (743, 538, 5), (751, 522, 2), (750, 526, 2), (754, 539, 3), (753, 608, 1)];
    for (i, (doll, gift, have)) in trades.iter().enumerate() {
        t.eval(&format!("give_item({doll}, 2)"));
        t.talk("alberta", 117, 135, &[2, i as u8 + 1]);
        assert_eq!(t.eval(&format!("return count_item({doll}) .. ' ' .. count_item({gift})")), format!("1 {have}"));
    }
    // the same for a girl
    t.eval("player.sex = 0");
    t.talk("alberta", 117, 135, &[2, 1]);
    assert_eq!(t.eval("return count_item(741) .. ' ' .. count_item(529)"), "0 2");
}

/// Raul's questions by the answers offered, and the right one (scripts/npc/wizard_test.lua).
const WIZARD_KEY: [(&str, u8); 35] = [
    ("Fire Bolt Lv 4:Fire Ball Lv 5:Sight Lv 1:Napalm Beat Lv 4", 4),
    ("Changes to Water:Changes to Earth:Changes to Fire:Changes to Wind", 1),
    ("1.6x:1.7x:2x:20x", 2),
    ("Red Blood:Blue Gem:Yellow Gem:Red Gem", 4),
    ("Napalm Beat Lv 4:Soul Strike Lv 5:SP Recovery Lv 6:Ice Wall Lv 7", 3),
    ("14:21:28:35", 2),
    ("dmg 18% SP 1.5%:dmg 18% SP 2%:dmg 24% SP 1.5%:dmg 24% SP 2%", 2),
    ("40 sp, 6 hits:35 sp, 6 hits:40 sp, 7 hits:35 sp, 7 hits", 3),
    ("84:74:64:54", 2),
    ("Lightning Bolt:Fire Bolt:Cold Bolt:Sight", 1),
    ("Thief Bug:Peco Peco:Pupa:Mace Kobold", 3),
    ("Flora:Giearth:Golem:Myst", 1),
    ("Elder Willow:Evil Druid:Magnolia:Marc", 2),
    ("125%:150%:175%:200%", 4),
    ("Baby Desert Wolf:Familiar:Tie:Don't know", 1),
    ("Poporing:Roda Frog:Smokie:Poison Spore", 2),
    ("Knife Goblin:Mace Goblin:Flail Goblin:Hammer Goblin", 4),
    ("Horong:Chonchon:Andre:Caramel", 4),
    ("Poring:Mastering:Ghostring:Spore", 3),
    ("Drake:Megalodon:Deviace:Khalitzburg", 3),
    ("INT:AGI:DEX:VIT", 1),
    ("Water:Earth:Fire:Wind", 2),
    ("Weak physical abilities:Distance attacks:Good money maker:High magic def", 3),
    ("Prontera:Morroc:Alberta:Geffen", 4),
    ("Andre Larva Card:Soldier Andre Card:Baby Desert Wolf Card:Elder Willow Card", 2),
    ("Close range combat:High dodge:Dance skills:Magical power", 4),
    ("8:7:6:5", 4),
    ("Knife:Cap:Sandals:Eye of Dullahan", 2),
    ("Yellow Gemstone:Red Gemstone:Blue Gemstone:Red Blood", 1),
    ("Marduk Card:Magnolia Card:Willow Card:Maya Card", 2),
    ("Mantis:Cunner Tooth:Giearth:Caramel", 2),
    ("Yoyo:Bigfoot:Metaller:Zerom", 4),
    ("Marina:Vitata:Scorpion:Giearth", 1),
    ("Cold Bolt:Fire Bolt:Lightning Bolt:Stone Curse", 3),
    ("Hydra:Mandragora:Greatest General:Frilldora", 4),
];

impl T {
    /// Let some game time pass, in milliseconds.
    fn run(&mut self, ms: u64) {
        for _ in 0..ms / 250 {
            self.g.skew(250);
            self.g.tick();
        }
        let errors: Vec<_> = self.g.notices.drain(..).filter(|n| n.contains("script error") || n.contains("event")).collect();
        assert!(errors.is_empty(), "{errors:?}");
    }

    /// Kill every monster on the player's map, by the player's hand.
    fn slay(&mut self) -> usize {
        let map = self.eval("return player.map");
        let mobs: Vec<u32> = self.g.world.units.values().filter(|u| u.map == map && u.mob().is_some()).map(|u| u.id).collect();
        for id in &mobs {
            self.eval(&format!("battle.damage(player.id, {id}, 999999)"));
        }
        self.run(500);
        mobs.len()
    }
}

/// The training ground where 817's script is broken (scripts/npc/novice_result.lua):
/// the forest Helper saves outside the forest and counts the tries, Leo Von
/// Frisch's right answers count once, the subsidy follows mod0631's table,
/// and "I will follow the result of the test" leads through a guide room.
#[test]
fn training_ground_result() {
    let mut t = setup("training_ground_result");
    // the first course with every answer right ('insert' - /Sit is the last)
    t.talk("new_zone02", 84, 115, &[1, 2, 3, 3, 2, 4, 1]);
    assert_eq!(t.eval("return v.nov_1st_cos"), "6", "six right answers, six points");
    assert_eq!(t.eval("return v.nov_1_2_cos_c"), "7");
    // between his lesson and the NPC Master's he has a line (817: none at all)
    t.talk("new_zone02", 84, 115, &[]);
    // a lesson cut short after the last question can be finished
    t.eval("v.nov_1_2_cos_c = 6");
    let herbs: i64 = t.eval("return count_item(507)").parse().unwrap();
    t.talk("new_zone02", 84, 115, &[]);
    assert_eq!(t.eval("return v.nov_1_2_cos_c"), "7");
    assert_eq!(t.eval("return count_item(507)").parse::<i64>().unwrap(), herbs + 4);
    // hearing him again later does not add points
    t.eval("v.nov_1_2_cos_c = 9; v.nov_1st_cos = 3");
    t.talk("new_zone02", 84, 115, &[2, 1, 2, 3, 3, 2, 4, 1]);
    assert_eq!(t.eval("return v.nov_1st_cos"), "3");
    t.eval("v.nov_1st_cos = 6");

    // the Guide in the hall knows where each step leads (817: one answer for all)
    for (step, word) in [(0, "Northwest"), (7, "Southeast"), (8, "right next him"), (9, "southwest")] {
        t.eval(&format!("v.nov_1_2_cos_c = {step}"));
        t.talk("new_zone02", 100, 113, &[1]);
        let said = t.said.join(" ");
        assert!(said.contains(word), "step {step}: {said}");
    }

    // the forest: saved beside the Helper, each return is a try, the fifth may be the last
    t.eval("v.nov_1_2_cos_c = 10");
    t.talk("new_zone02", 16, 183, &[1]);
    assert_eq!(t.eval("return player.map .. ' ' .. v.nov_1_2_cos_c .. ' ' .. v.nov_2nd_tries"), "new_zone03 11 0");
    assert_eq!(t.eval("return player.save_map"), "new_zone02", "knocked out, you wake beside the Helper");
    t.talk("new_zone02", 16, 183, &[1]);
    assert_eq!(t.eval("return player.map .. ' ' .. v.nov_2nd_tries"), "new_zone03 1");

    // the personality test came out for the Archer and the Thief alike
    t.eval("v.nov_3_thief = 5; v.nov_3_archer = 5; v.nov_3_swordman = 2");
    let zeny: i64 = t.eval("return player.zeny").parse().unwrap();
    let herbs: i64 = t.eval("return count_item(507)").parse().unwrap();
    t.talk("new_zone04", 60, 149, &[2]);
    // 6 + 9 points: 150z and three Red Herbs (mod0631 npc_job_newbie.txt:2270)
    assert_eq!(t.eval("return player.zeny").parse::<i64>().unwrap(), zeny + 150);
    assert_eq!(t.eval("return count_item(507)").parse::<i64>().unwrap(), herbs + 3);
    assert_eq!(t.eval("return player.map .. ' ' .. player.x .. ' ' .. player.y"), "new_zone04 60 176", "the Archer guide room: the first of two equal scores wins");
    // its attendant: three Red Potions, 50z, and on to the Archer guild
    t.talk("new_zone04", 59, 186, &[]);
    assert_eq!(t.eval("return count_item(501)"), "3");
    assert_eq!(t.eval("return player.zeny").parse::<i64>().unwrap(), zeny + 200);
    assert_eq!(t.eval("return player.map"), "payon_in02");

    // never knocked out and all six right: 16 points, 300z and five Red Herbs;
    // choosing for oneself goes straight to town
    let mut t = setup("training_ground_result_own");
    t.eval("v.nov_1st_cos = 6");
    t.talk("new_zone04", 60, 149, &[1, 4]);
    assert_eq!(t.eval("return player.map"), "geffen");
    assert_eq!(t.eval("return count_item(507) .. ' ' .. player.zeny"), "5 300");

    // the fifth time back from the forest it may be given up: no points for it
    let mut t = setup("training_ground_give_up");
    t.eval("v.nov_1_2_cos_c = 11; v.nov_2nd_tries = 4");
    t.talk("new_zone02", 16, 183, &[1]);
    assert_eq!(t.eval("return player.map .. ' ' .. v.nov_2nd_tries"), "new_zone04 11");
    t.talk("new_zone04", 60, 149, &[1, 1]);
    assert_eq!(t.eval("return count_item(507) .. ' ' .. player.zeny"), "1 100", "0 points: one Red Herb, 100z");
}

/// The Merchant test's parcels (scripts/npc/merchant_delivery.lua and the
/// source fixes to 817's merchant.txt): each of the four places takes the
/// box of the errand it was sent and no other, Byalan's Kafra among them;
/// Kay changes a wrong parcel; the letter pays only when it was delivered.
#[test]
fn merchant_parcels() {
    let mut t = setup("merchant_parcels");
    let places = [("prontera", 248, 42, 1073), ("geffen_in", 155, 122, 1075), ("morocc_in", 140, 102, 1077), ("izlu2dun", 106, 58, 1079)];
    for (i, (map, x, y, receipt)) in places.iter().enumerate() {
        for errand in 0..2u32 {
            let (right, other, receipt) = (1081 + errand, 1082 - errand, receipt + errand);
            let state = format!("v.job_merchant_q = 5; v.job_merchant_q2 = {}; v.job_merchant_q3 = {}", 2 * i as u32 + 1 + errand, i + 1);
            for wrong in [1083, other] {
                t.eval(&format!("{state}; give_item({wrong}, 1)"));
                t.talk_by(map, *x, *y, |_| 1);
                assert_eq!(t.eval(&format!("return count_item({receipt}) .. ' ' .. v.job_merchant_q .. ' ' .. count_item({wrong})")), "0 5 1", "{map}, errand {errand}: box {wrong} is not the one ordered");
                t.eval(&format!("take_item({wrong}, 1)"));
            }
            t.eval(&format!("{state}; give_item({right}, 1)"));
            t.talk_by(map, *x, *y, |_| 1);
            assert_eq!(t.eval(&format!("return count_item({receipt}) .. ' ' .. v.job_merchant_q .. ' ' .. count_item({right})")), "1 7 0", "{map}, errand {errand}: the right box gets its receipt");
            // delivered: she has nothing more to take, and stays a Kafra to everyone else
            t.talk_by(map, *x, *y, |_| 1);
            assert_eq!(t.eval("return v.job_merchant_q"), "7");
            t.eval(&format!("take_item({receipt}, 1)"));
        }
    }
    // Byalan: the Chief's letter goes with the parcel, and only then does he pay for it
    let fruit: i64 = t.eval("return count_item(512) + count_item(513)").parse().unwrap();
    t.eval("v.job_merchant_q = 6; v.job_merchant_q2 = 8; v.job_merchant_q3 = 4; v.nov_3_merchant = 0; give_item(1082, 1); give_item(1072, 1)");
    t.talk_by("izlu2dun", 106, 58, |_| 1);
    assert_eq!(t.eval("return count_item(1080) .. ' ' .. count_item(1072) .. ' ' .. v.nov_3_merchant .. ' ' .. v.job_merchant_q"), "1 0 11 8");
    assert_eq!(t.eval("return count_item(512) + count_item(513)").parse::<i64>().unwrap(), fruit + 3, "her small gift");

    // off the errand she is the Kafra who sends you to Izlude for 900z, if you have them
    t.eval("v.job_merchant_q = 0; v.job_merchant_q3 = 0; player.zeny = 899; v.KF_PIT = 0");
    t.talk("prontera", 248, 42, &[1]);
    assert_eq!(t.eval("return player.map .. ' ' .. player.zeny"), "prontera 899");
    t.eval("player.zeny = 1000");
    t.talk("prontera", 248, 42, &[1]);
    assert_eq!(t.eval("return player.map .. ' ' .. player.zeny .. ' ' .. v.KF_PIT"), "izlude 100 90");

    // Byalan's Kafra, off the errand, is an ordinary Kafra: she saves, and stores for 35z
    t.eval("player.zeny = 100");
    let menus = t.talk("izlu2dun", 106, 58, &[1]);
    assert!(menus[0].starts_with("Save:Use Storage:Rent a Cart"), "{menus:?}");
    assert_eq!(t.eval("return player.save_map"), "izlu2dun");

    // Kay changes a parcel with the wrong serial number (817: the branch could not be reached)
    t.eval("v.job_merchant_q = 5; v.job_merchant_q2 = 1; v.job_merchant_q3 = 1; give_item(1083, 1)");
    let menus = t.talk("alberta_in", 28, 29, &[1, 1]);
    assert!(menus[1].starts_with("Please Exchange the Product."), "{menus:?}");
    assert_eq!(t.eval("return count_item(1083) .. ' ' .. v.job_merchant_q .. ' ' .. v.job_merchant_q3"), "0 4 0");
}

/// The Thief test's weighing (source fix to 817's thief.txt:429): nine Orange
/// Net Mushrooms and nothing else are 27 points, a pass.
#[test]
fn thief_mushrooms_of_one_kind() {
    let mut t = setup("thief_mushrooms");
    t.eval("player.job_level = 10; give_skill(1, 9); v.job_thief_q = 3; give_item(1069, 9)");
    t.talk_by("moc_prydb1", 42, 133, |_| 1);
    assert_eq!(t.eval("return player.class"), "6", "Thief");
}

/// The Archer test (source fixes to 817's archer.txt): the trunks handed in
/// are taken, all of them, and a passing score is read out, not "0 / 40".
#[test]
fn archer_trunks() {
    let mut t = setup("archer_trunks");
    t.eval("player.job_level = 10; give_skill(1, 9); v.job_archer_q = 1; give_item(1066, 6); give_item(1019, 3)");
    t.talk_by("payon_in02", 64, 71, |menu| if menu.starts_with("Err") { 2 } else { 1 });
    assert_eq!(t.eval("return player.class"), "3", "Archer");
    assert_eq!(t.eval("return count_item(1066) + count_item(1019)"), "0", "no trunk left to sell");
    let said = t.said.join(" ");
    assert!(said.contains("^0000FF33^000000 / 40"), "six fine trunks and three plain ones are 33 points: {said}");
    assert_eq!(t.eval("return count_item(1702) .. ' ' .. count_item(1750)"), "1 100", "the guild's bow and arrows");
}

fn wizard_answer(menu: &str, right: bool) -> u8 {
    match WIZARD_KEY.iter().find(|(m, _)| *m == menu) {
        Some((_, a)) if right => *a,
        Some((_, a)) => a % 4 + 1,
        // not a question: take the second entry ("I want to continue", "Please go on")
        None => 2,
    }
}

#[test]
fn wizard_test() {
    let mut t = setup("wizard_test");
    let (clerk, raul, helper) = (("gef_tower", 111, 37), ("gef_tower", 102, 24), ("job_wiz", 50, 165));
    let q = "return v.job_wizard_q";
    // a Mage of job level 39 may not sign
    t.eval("math.randomseed(7); job_change(2); player.job_level = 39; player.skill_point = 0");
    t.talk(clerk.0, clerk.1, clerk.2, &[1, 1]);
    assert_eq!(t.eval(q), "0");
    // at 40 she names one of two lists; Raul will not test before they are brought
    t.eval("player.job_level = 40");
    t.talk(clerk.0, clerk.1, clerk.2, &[1, 1]);
    let list = t.eval(q);
    assert!(list == "1" || list == "2", "{list}");
    t.talk(raul.0, raul.1, raul.2, &[]);
    t.talk(clerk.0, clerk.1, clerk.2, &[]);
    assert_eq!(t.eval(q), list);
    // one short of the last thing on the list is not enough (the sources' "and")
    if list == "1" {
        t.eval("give_item(716, 10); give_item(717, 10); give_item(715, 9)");
    } else {
        t.eval("give_item(991, 5); give_item(993, 5); give_item(990, 5); give_item(992, 4)");
    }
    t.talk(clerk.0, clerk.1, clerk.2, &[]);
    assert_eq!(t.eval(q), list);
    t.eval(if list == "1" { "give_item(715, 1)" } else { "give_item(992, 1)" });
    t.talk(clerk.0, clerk.1, clerk.2, &[]);
    assert_eq!(t.eval(q), "3");
    assert_eq!(t.eval("return count_item(716) + count_item(717) + count_item(715) + count_item(991) + count_item(993) + count_item(990) + count_item(992)"), "0");

    // the questions: two wrong fails, the hint costs an Apple Juice, one wrong passes
    let mut wrong = 2;
    t.talk_by(raul.0, raul.1, raul.2, |m| {
        let known = WIZARD_KEY.iter().any(|(k, _)| *k == m);
        let right = !(known && wrong > 0);
        if known && wrong > 0 {
            wrong -= 1;
        }
        wizard_answer(m, right)
    });
    assert_eq!(t.eval(q), "4");
    t.eval("give_item(531, 1)");
    t.talk(clerk.0, clerk.1, clerk.2, &[1]);
    assert_eq!(t.eval("return count_item(531)"), "0");
    let mut wrong = 1;
    let menus = t.talk_by(raul.0, raul.1, raul.2, |m| {
        let known = WIZARD_KEY.iter().any(|(k, _)| *k == m);
        let right = !(known && wrong > 0);
        if known && wrong > 0 {
            wrong -= 1;
        }
        wizard_answer(m, right)
    });
    assert_eq!(menus.len(), 11, "{menus:?}");
    assert_eq!(t.eval(q), "5");

    // into the waiting room; the Helper lets us into the Room of Water
    t.talk(raul.0, raul.1, raul.2, &[2, 1]);
    assert_eq!(t.eval("return player.map .. ' ' .. player.x .. ' ' .. player.y .. ' ' .. v.job_wizard_q"), "job_wiz 57 154 6");
    // lounging first: we stay, and he remembers that we have heard the rules
    let menus = t.talk(helper.0, helper.1, helper.2, &[2]);
    assert_eq!(menus, ["I am ready.:I'd like to lounge a bit first."]);
    assert_eq!(t.eval("return player.x .. ' ' .. player.y .. ' ' .. tmp.jobwiz_lounge"), "50 166 1");
    t.talk(helper.0, helper.1, helper.2, &[1]);
    assert_eq!(t.eval("return player.x .. ' ' .. player.y"), "114 169");
    // running out of time puts us back in Geffen, the monsters gone
    t.run(185_000);
    assert_eq!(t.eval("return player.map .. ' ' .. mob_count('job_wiz', 'jobwiz_1::OnKilled')"), "geffen 0");
    // Raul asks five questions before a second try: three right is not enough
    let mut wrong = 2;
    t.talk_by(raul.0, raul.1, raul.2, |m| {
        let known = WIZARD_KEY.iter().any(|(k, _)| *k == m);
        let right = !(known && wrong > 0);
        if known && wrong > 0 {
            wrong -= 1;
        }
        wizard_answer(m, right)
    });
    assert_eq!(t.eval("return player.map"), "gef_tower");
    t.talk_by(raul.0, raul.1, raul.2, |m| if m.starts_with("Let's start!") { 1 } else { wizard_answer(m, true) });
    assert_eq!(t.eval("return player.map"), "job_wiz");

    // three rooms: the monsters, then the gate guards, then on
    t.talk(helper.0, helper.1, helper.2, &[1]);
    for (room, mobs, guards, then) in [(1, 8, 5, "116 97"), (2, 10, 7, "46 99"), (3, 8, 3, "46 99")] {
        t.run(4000);
        assert_eq!(t.eval(&format!("return mob_count('job_wiz', 'jobwiz_{room}::OnKilled')")), mobs.to_string(), "room {room}");
        if room == 2 {
            t.eval("player.hp = 10");
        }
        assert_eq!(t.slay(), mobs, "room {room}");
        assert_eq!(t.eval(&format!("return mob_count('job_wiz', 'jobwiz_{room}g::OnKilled')")), guards.to_string(), "room {room}");
        assert_eq!(t.slay(), guards, "room {room}");
        if room < 3 {
            assert_eq!(t.eval("return player.x .. ' ' .. player.y"), then, "after room {room}");
        }
        if room == 2 {
            assert_eq!(t.eval("return player.hp == player.max_hp"), "true", "healed on entering the Room of Fire");
        }
    }
    assert_eq!(t.eval(q), "7");
    t.run(6000);
    assert_eq!(t.eval("return player.map .. ' ' .. player.x .. ' ' .. player.y"), "gef_tower 110 30");
    // no timer of the test is left to throw anyone out
    t.run(200_000);
    assert_eq!(t.eval("return player.map"), "gef_tower");
    // skill points must be spent; then the job and six Blue Potions
    t.eval("player.skill_point = 1");
    t.talk(clerk.0, clerk.1, clerk.2, &[]);
    assert_eq!(t.eval("return player.class"), "2");
    t.eval("player.skill_point = 0");
    t.talk(clerk.0, clerk.1, clerk.2, &[]);
    assert_eq!(t.eval("return player.class .. ' ' .. count_item(505) .. ' ' .. v.job_wizard_q"), "9 6 0");
}
