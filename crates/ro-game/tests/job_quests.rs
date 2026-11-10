//! Can each job be reached? A character with the level its guild asks for,
//! and the things the quest's scripts count, talks to that quest's NPCs with
//! random answers until the job changes (the crawler in npc_crawl.rs only
//! looks for script errors, not for dead ends). Cells that warp or trap on
//! touch are stepped on like the others. Monsters a quest calls are killed
//! by the GM command. Repo scripts, synthetic all-walkable maps, no client
//! files. Deterministic (fixed seed).
//!
//!   cargo test -p ro-game --test job_quests -- --nocapture

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

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: u64) -> u64 {
        if n == 0 { 0 } else { self.next() % n }
    }
}

fn enter(g: &mut Game, cap: &Cap) -> u32 {
    g.handle(NetEvent::Connected { sid: 1, kind: ServerKind::Login });
    let mut w = Writer::new(0x0064);
    w.u32(1).str_fixed("quester", 24).str_fixed("quester", 24).u8(20);
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
    w.str_fixed("Quester", 24).bytes(&[9, 1, 1, 1, 9, 9]).u8(0).u16(0).u16(1);
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

/// The quest's NPCs (map, x, y, touch area) and the item ids its scripts
/// count, read from the script files themselves.
fn read_quest(root: &std::path::Path, files: &[&str]) -> (Vec<(String, u16, u16, bool)>, Vec<u32>) {
    let mut npcs = Vec::new();
    let mut items = Vec::new();
    for f in files {
        let text = std::fs::read_to_string(root.join("scripts").join(f)).unwrap_or_else(|e| panic!("{f}: {e}"));
        let lines: Vec<&str> = text.lines().collect();
        for (i, l) in lines.iter().enumerate() {
            let t = l.trim_start();
            if let Some(rest) = t.strip_prefix("map = \"") {
                let map = rest.split('"').next().unwrap().to_string();
                let num = |k: &str| -> Option<u16> {
                    let at = t.find(k)? + k.len();
                    t[at..].split(|c: char| !c.is_ascii_digit()).next()?.parse().ok()
                };
                if let (Some(x), Some(y)) = (num(" x = "), num(" y = ")) {
                    let touch = lines.get(i + 1).is_some_and(|n| n.contains("touch = {"));
                    if map != "-" {
                        npcs.push((map, x, y, touch));
                    }
                }
            }
            for part in l.split("count_item(").skip(1) {
                if let Ok(id) = part.split(')').next().unwrap().trim().parse::<u32>() {
                    if !items.contains(&id) {
                        items.push(id);
                    }
                }
            }
        }
    }
    (npcs, items)
}

struct Quest {
    name: &'static str,
    files: &'static [&'static str],
    from: u16,
    job_level: u16,
    to: u16,
    /// Items to start with; None = 30 of everything the scripts count.
    gifts: Option<&'static [(u32, u32)]>,
    /// Numbers to type when asked for one (else something under 30).
    numbers: &'static [u32],
}

const QUESTS: &[Quest] = &[
    Quest { name: "Swordman", files: &["import/npc/jobs/swordsman.lua"], from: 0, job_level: 10, to: 1, gifts: None, numbers: &[] },
    // the materials a Novice would bring, and the four numbers the guild's book gives
    Quest {
        name: "Mage",
        files: &["import/npc/jobs/magician.lua"],
        from: 0,
        job_level: 10,
        to: 2,
        gifts: Some(&[(909, 30), (914, 30), (519, 5)]),
        numbers: &[8472, 3735, 2750, 5429],
    },
    Quest { name: "Archer", files: &["import/npc/jobs/archer.lua"], from: 0, job_level: 10, to: 3, gifts: None, numbers: &[] },
    Quest { name: "Acolyte", files: &["import/npc/jobs/acolyte.lua"], from: 0, job_level: 10, to: 4, gifts: None, numbers: &[] },
    // nothing in hand; the eight serial numbers the Chief may name
    Quest {
        name: "Merchant",
        files: &["import/npc/jobs/merchant.lua"],
        from: 0,
        job_level: 10,
        to: 5,
        gifts: Some(&[]),
        numbers: &[2485741, 2328137, 2989396, 2191737, 3012685, 3487372, 3318702, 3543625],
    },
    Quest { name: "Thief", files: &["import/npc/jobs/thief.lua"], from: 0, job_level: 10, to: 6, gifts: None, numbers: &[] },
    Quest { name: "Knight", files: &["import/npc/jobs/knight.lua"], from: 1, job_level: 50, to: 7, gifts: None, numbers: &[] },
    Quest { name: "Priest", files: &["import/npc/jobs/priest.lua"], from: 4, job_level: 50, to: 8, gifts: None, numbers: &[] },
    Quest { name: "Wizard", files: &["import/npc/jobs/wizard.lua"], from: 2, job_level: 50, to: 9, gifts: None, numbers: &[] },
    Quest { name: "Blacksmith", files: &["import/npc/jobs/blacksmith.lua"], from: 5, job_level: 50, to: 10, gifts: None, numbers: &[] },
    Quest { name: "Hunter", files: &["import/npc/jobs/hunter.lua"], from: 3, job_level: 50, to: 11, gifts: None, numbers: &[] },
    Quest { name: "Assassin", files: &["import/npc/jobs/assassin.lua"], from: 6, job_level: 50, to: 12, gifts: None, numbers: &[] },
];

/// Talk to (or step on) the quest's NPCs until the class changes. Returns
/// the number of conversations it took, or None.
fn play(q: &Quest, seed: u64, limit: usize) -> Result<Option<usize>, String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let saves = std::env::temp_dir().join(format!("ro-offline-jobq-{}-{}-{seed}", q.name, std::process::id()));
    let _ = std::fs::remove_dir_all(&saves);
    let mut vfs = Vfs::new();
    vfs.add_memory_file("data\\questmap.gat", gat(50, 50));
    let cfg = GameConfig {
        save_dir: saves.clone(),
        script_dir: root.join("scripts"),
        db_dir: root.join("data/db"),
        start_point: Point::new("questmap", 25, 25),
        ..Default::default()
    };
    let cap = Cap::default();
    let mut g = Game::new(cfg, Resources::new(vfs), Box::new(cap.clone())).expect("game");
    let mut maps: Vec<String> = g.script_maps();
    maps.sort();
    maps.dedup();
    for m in &maps {
        g.res.vfs.add_memory_file(&format!("data\\{m}.gat"), gat(400, 400));
        g.res.maps.insert(m.clone());
    }
    let me = enter(&mut g, &cap);
    let (npcs, items) = read_quest(&root, q.files);
    assert!(!npcs.is_empty(), "{}: no NPCs", q.name);
    for (m, ..) in &npcs {
        g.ensure_map(m);
    }
    let mut rng = Rng(seed);
    let gifts: String = match q.gifts {
        Some(g) => g.iter().map(|(id, n)| format!("give_item({id}, {n}) ")).collect(),
        None => items.iter().map(|id| format!("give_item({id}, 30) ")).collect(),
    };
    g.console_eval(
        &format!(
            "math.randomseed({seed}); player.base_level = 60; player.zeny = 500000; job_change({}); player.job_level = {}; \
             give_skill(1, 9, 0); {gifts}",
            q.from, q.job_level
        ),
        me,
    );
    g.notices.clear();
    let mut result = None;
    for n in 0..limit {
        let (map, x, y, touch) = &npcs[rng.below(npcs.len() as u64) as usize];
        // talk only to who is near enough to be reached on foot: the test maps
        // are entered through the quest, so prefer the map the player is on
        let here = g.console_eval("return player.map", me);
        if &here != map && npcs.iter().any(|n| n.0 == here) && rng.below(4) != 0 {
            continue;
        }
        let npc = g.world.units.values().find(|u| &u.map == map && u.x == *x && u.y == *y && u.player().is_none() && u.mob().is_none()).map(|u| u.id);
        let Some(npc) = npc else { continue };
        if *touch {
            // walk onto it: one cell away, then a step
            g.console_eval(&format!("warp('{map}', {x}, {y})"), me);
        } else {
            g.console_eval(&format!("warp('{map}', {x}, {})", y + 1), me);
            cap.take(3);
            let mut w = Writer::new(0x0090);
            w.u32(npc).u8(1);
            send(&mut g, 3, w);
        }
        for _ in 0..120 {
            let out = cap.take(3);
            let mut acted = false;
            for p in out.iter().rev() {
                match u16::from_le_bytes([p[0], p[1]]) {
                    0x00b7 => {
                        let text = Reader::new(p).seek(8).str_rest();
                        let k = text.split(':').filter(|s| !s.is_empty()).count().max(1) as u64;
                        let npc = Reader::new(p).seek(4).u32().unwrap();
                        let mut w = Writer::new(0x00b8);
                        w.u32(npc).u8(1 + rng.below(k) as u8);
                        send(&mut g, 3, w);
                    }
                    0x00b5 => {
                        let npc = Reader::new(p).seek(2).u32().unwrap();
                        let mut w = Writer::new(0x00b9);
                        w.u32(npc);
                        send(&mut g, 3, w);
                    }
                    0x0142 => {
                        let npc = Reader::new(p).seek(2).u32().unwrap();
                        let mut w = Writer::new(0x0143);
                        let n = if q.numbers.is_empty() { rng.below(30) as u32 } else { q.numbers[rng.below(q.numbers.len() as u64) as usize] };
                        w.u32(npc).u32(n);
                        send(&mut g, 3, w);
                    }
                    0x00b6 => {
                        let npc = Reader::new(p).seek(2).u32().unwrap();
                        let mut w = Writer::new(0x0146);
                        w.u32(npc);
                        send(&mut g, 3, w);
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
        send(&mut g, 3, Writer::new(0x007d));
        g.tick();
        let errors: Vec<_> = g.notices.drain(..).filter(|n| n.contains("script error")).collect();
        if !errors.is_empty() {
            let _ = std::fs::remove_dir_all(&saves);
            return Err(errors.join("; "));
        }
        if g.console_eval("return player.class", me) == q.to.to_string() {
            result = Some(n + 1);
            break;
        }
    }
    let _ = std::fs::remove_dir_all(&saves);
    Ok(result)
}

#[test]
fn every_job_can_be_reached() {
    let mut stuck = Vec::new();
    let only = std::env::var("QUEST").ok();
    for q in QUESTS.iter().filter(|q| only.as_deref().is_none_or(|o| o == q.name)) {
        // three characters each: a script error on any of them counts
        let mut reached = None;
        let mut error = None;
        for s in 1..=3 {
            match play(q, 0x2003_1218 + s, 1500) {
                Ok(n) => reached = reached.or(n),
                Err(e) => error = Some(e),
            }
        }
        match (reached, error) {
            (_, Some(e)) => {
                println!("{:<10} SCRIPT ERROR {e}", q.name);
                stuck.push(q.name);
            }
            (Some(n), None) => println!("{:<10} reached after {n} conversations", q.name),
            (None, None) => {
                println!("{:<10} NOT reached", q.name);
                stuck.push(q.name);
            }
        }
    }
    assert!(stuck.is_empty(), "no way into: {stuck:?}");
}
