//! Talk to every NPC the repo's scripts define, many times, with varied
//! characters and random answers, and report every script error. Each map
//! the scripts use becomes a synthetic all-walkable 400x400 map, so no
//! client files are needed. Deterministic (fixed seed).
//!
//!   cargo test -p ro-game --test npc_crawl -- --nocapture
//!   CRAWL_FULL=1 cargo test ...   # every character state, 3 runs each (~2 min)
//!
//! Errors in source scripts that Athena 817 had too (a command its own
//! engine lacked) are expected and listed separately.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use ro_game::model::Point;
use ro_game::world::{NpcKind, NpcUnit, UnitKind};
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
    w.u32(1).str_fixed("crawler", 24).str_fixed("crawler", 24).u8(20);
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
    w.str_fixed("Crawly", 24).bytes(&[9, 1, 1, 1, 9, 9]).u8(0).u16(0).u16(1);
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

/// Character states to talk with: (class, base, job, zeny).
const STATES: &[(u16, u16, u16, u32)] = &[
    (0, 1, 1, 0),
    (0, 10, 10, 5000),
    (1, 30, 40, 100_000),
    (2, 30, 40, 100_000),
    (3, 30, 40, 100_000),
    (4, 30, 40, 100_000),
    (5, 30, 40, 100_000),
    (6, 30, 40, 100_000),
    (7, 70, 50, 1_000_000),
    (10, 70, 50, 1_000_000),
];

#[test]
fn crawl_every_npc() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let saves = std::env::temp_dir().join(format!("ro-offline-crawl-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&saves);
    let mut vfs = Vfs::new();
    vfs.add_memory_file("data\\crawlmap.gat", gat(50, 50));
    let cfg = GameConfig {
        save_dir: saves.clone(),
        script_dir: root.join("scripts"),
        db_dir: root.join("data/db"),
        start_point: Point::new("crawlmap", 25, 25),
        // everything on, so gated files are crawled too
        content: ["juno", "jobs2_2", "pvp", "events", "arena", "castle_monsters"].iter().map(|k| (k.to_string(), true)).collect(),
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
    for m in &maps {
        g.ensure_map(m);
    }
    g.notices.clear();
    let npcs: Vec<(u32, String, u16, u16, String)> = g
        .world
        .units
        .values()
        .filter(|u| matches!(u.kind, UnitKind::Npc(NpcUnit { kind: NpcKind::Script { .. }, hidden: false, .. })))
        .map(|u| (u.id, u.map.clone(), u.x, u.y, u.name.clone()))
        .collect();
    assert!(npcs.len() > 400, "only {} NPCs placed", npcs.len());
    let mut rng = Rng(0x2003_1218);
    let mut errors: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut steps = 0usize;
    let full = std::env::var_os("CRAWL_FULL").is_some();
    let (states, runs) = if full { (STATES.len(), 3) } else { (4, 1) };
    for (npc, map, x, y, name) in &npcs {
        for (si, &(class, base, job, zeny)) in STATES.iter().enumerate().take(states) {
            for run in 0..runs {
                let seed = rng.next() % 1_000_000_007;
                g.console_eval(
                    &format!(
                        "math.randomseed({seed}); player.base_level = {base}; player.job_level = {job}; player.zeny = {zeny}; job_change({class}); \
                         player.job_level = {job}; warp('{map}', {x}, {}); give_item(501, 3)",
                        y + 1
                    ),
                    me,
                );
                cap.take(3);
                let mut w = Writer::new(0x0090);
                w.u32(*npc).u8(1);
                send(&mut g, 3, w);
                for _ in 0..80 {
                    let out = cap.take(3);
                    steps += 1;
                    let mut acted = false;
                    for p in out.iter().rev() {
                        let id = u16::from_le_bytes([p[0], p[1]]);
                        match id {
                            0x00b7 => {
                                let text = Reader::new(p).seek(8).str_rest();
                                let n = text.split(':').filter(|s| !s.is_empty()).count().max(1) as u64;
                                let mut w = Writer::new(0x00b8);
                                w.u32(*npc).u8(1 + rng.below(n) as u8);
                                send(&mut g, 3, w);
                            }
                            0x00b5 => {
                                let mut w = Writer::new(0x00b9);
                                w.u32(*npc);
                                send(&mut g, 3, w);
                            }
                            0x0142 => {
                                let mut w = Writer::new(0x0143);
                                w.u32(*npc).u32(rng.below(20) as u32);
                                send(&mut g, 3, w);
                            }
                            0x01d4 => {
                                let mut w = Writer::new(0x01d5);
                                w.u32(*npc).str_nul("test");
                                send(&mut g, 3, w);
                            }
                            0x00b6 => {
                                let mut w = Writer::new(0x0146);
                                w.u32(*npc);
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
                // end anything still open, run queued events
                let mut w = Writer::new(0x0146);
                w.u32(*npc);
                send(&mut g, 3, w);
                send(&mut g, 3, Writer::new(0x00f7));
                g.tick();
                for n in g.notices.drain(..) {
                    // "script error (npc 110000123): <lua error>" -> the error itself
                    let key = match n.split_once("): ") {
                        Some((head, rest)) if head.starts_with("script error") => rest.lines().next().unwrap_or(rest).to_string(),
                        _ => n.clone(),
                    };
                    errors.entry(key).or_default().push(format!("{name} @ {map} {x},{y} state {si} run {run}"));
                }
            }
        }
    }
    let mut real = 0;
    println!("crawled {} NPCs, {steps} dialog steps", npcs.len());
    for (e, wher) in &errors {
        // source scripts using commands 817 lacked; PvP instance rooms this client doesn't have
        // (PvP is off by default)
        let expected = e.contains("athena: no command") || e.contains("is not implemented yet") || e.contains(" pvp_");
        if !expected {
            real += 1;
        }
        println!("{} x{}: {e}\n    first: {}", if expected { "expected" } else { "ERROR" }, wher.len(), wher[0]);
    }
    let _ = std::fs::remove_dir_all(&saves);
    assert_eq!(real, 0, "{real} kinds of script errors (see output)");
}
