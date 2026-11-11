//! A whole play session over the real protocol, in-process: login, char
//! creation, map entry, walking, chat, @commands, NPC-free combat, EXP,
//! loot. Uses a synthetic 50x50 map, so no client files are needed.
//! The repo's scripts/ and data/db/ are loaded as-is.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

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

fn id_of(p: &[u8]) -> u16 {
    u16::from_le_bytes([p[0], p[1]])
}
fn find(pkts: &[Vec<u8>], id: u16) -> Option<&Vec<u8>> {
    pkts.iter().find(|p| p.len() >= 2 && id_of(p) == id)
}

fn gat(w: u32, h: u32) -> Vec<u8> {
    let mut b = b"GRAT\x01\x02".to_vec();
    b.extend_from_slice(&w.to_le_bytes());
    b.extend_from_slice(&h.to_le_bytes());
    b.resize(b.len() + (w * h * 20) as usize, 0); // heights 0.0, type 0 = walkable
    b
}

fn game() -> (Game, Cap) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let saves = std::env::temp_dir().join(format!("ro-offline-test-{}", std::process::id()));
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
    let g = Game::new(cfg, Resources::new(vfs), Box::new(cap.clone())).expect("game");
    (g, cap)
}

fn pump(g: &mut Game, ms: u64) {
    let end = Instant::now() + Duration::from_millis(ms);
    while Instant::now() < end {
        g.tick();
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn send(g: &mut Game, sid: u32, mut w: Writer) {
    g.handle(NetEvent::Packet { sid, data: w.finish() });
}

#[test]
fn full_session() {
    let (mut g, cap) = game();

    // ---- login server: unknown id auto-registers
    g.handle(NetEvent::Connected { sid: 1, kind: ServerKind::Login });
    let mut w = Writer::new(0x0064);
    w.u32(1).str_fixed("botuser", 24).str_fixed("botpass", 24).u8(20);
    send(&mut g, 1, w);
    let out = cap.take(1);
    let acc = find(&out, 0x0069).expect("login accepted");
    let mut r = Reader::new(acc);
    r.seek(4);
    let (l1, aid, l2) = (r.u32().unwrap(), r.u32().unwrap(), r.u32().unwrap());
    assert_eq!(aid, 2_000_000);
    assert_eq!((acc.len() - 47) % 32, 0, "server records are 32 bytes");

    // wrong password is refused
    g.handle(NetEvent::Connected { sid: 9, kind: ServerKind::Login });
    let mut w = Writer::new(0x0064);
    w.u32(1).str_fixed("botuser", 24).str_fixed("nope", 24).u8(20);
    send(&mut g, 9, w);
    assert_eq!(find(&cap.take(9), 0x006a).unwrap()[2], 1);

    // ---- char server
    g.handle(NetEvent::Connected { sid: 2, kind: ServerKind::Char });
    let mut w = Writer::new(0x0065);
    w.u32(aid).u32(l1).u32(l2).u16(0).u8(1);
    send(&mut g, 2, w);
    let out = cap.take(2);
    assert_eq!(out[0], aid.to_le_bytes().to_vec(), "raw account id first (Athena-era quirk)");
    assert_eq!(id_of(&out[1]), 0x006b);

    // invalid stats are refused, valid ones accepted
    let mut w = Writer::new(0x0067);
    w.str_fixed("Botty", 24).bytes(&[9, 9, 9, 9, 9, 9]).u8(0).u16(0).u16(1);
    send(&mut g, 2, w);
    assert_eq!(find(&cap.take(2), 0x006e).unwrap()[2], 0xff);
    let mut w = Writer::new(0x0067);
    w.str_fixed("Botty", 24).bytes(&[9, 1, 1, 1, 9, 9]).u8(0).u16(0).u16(1);
    send(&mut g, 2, w);
    let made = cap.take(2);
    assert!(find(&made, 0x006d).is_some(), "char created");

    let mut w = Writer::new(0x0066);
    w.u8(0);
    send(&mut g, 2, w);
    let zone = cap.take(2);
    let z = find(&zone, 0x0071).expect("zone server");
    let mut r = Reader::new(z);
    let cid = r.u32().unwrap();
    assert_eq!(r.str_fixed(16).unwrap(), "testmap.gat");

    // ---- map server
    g.handle(NetEvent::Connected { sid: 3, kind: ServerKind::Map });
    let mut w = Writer::new(0x0072);
    w.u32(aid).u32(cid).u32(l1).u32(0).u8(1);
    send(&mut g, 3, w);
    let out = cap.take(3);
    assert_eq!(out[0], aid.to_le_bytes().to_vec());
    let (x, y, _) = Reader::new(find(&out, 0x0073).unwrap()).seek(6).pos3().unwrap();
    assert_eq!((x, y), (25, 25));
    send(&mut g, 3, Writer::new(0x007d));
    let out = cap.take(3);
    for id in [0x00bd, 0x00a3, 0x00a4, 0x010f] {
        assert!(find(&out, id).is_some(), "initial packet {id:#06x}");
    }

    // ---- walk 3 cells east (150 ms/cell)
    let mut w = Writer::new(0x0085);
    w.pos3(28, 25, 0);
    send(&mut g, 3, w);
    assert!(find(&cap.take(3), 0x0087).is_some(), "walk accepted");
    pump(&mut g, 700);
    let u = g.world.units.get(&aid).unwrap();
    assert_eq!((u.x, u.y), (28, 25));

    // ---- chat + @command (Lua)
    let mut w = Writer::new(0x008c);
    w.str_nul("Botty : hello");
    send(&mut g, 3, w);
    assert!(find(&cap.take(3), 0x008e).is_some());
    let mut w = Writer::new(0x008c);
    w.str_nul("Botty : @where");
    send(&mut g, 3, w);
    let o = cap.take(3);
    assert!(Reader::new(find(&o, 0x008e).unwrap()).seek(4).str_rest().contains("testmap 28,25"));

    // ---- spawn a Poring next to us and kill it (continuous attack)
    // a new character wears its Knife and Cotton Shirt (mod0631 char/char.c:408-415)
    assert_eq!(g.console_eval("equip(4).name .. ' / ' .. equip(2).name", aid), "Knife / Cotton Shirt");
    let r = g.console_eval("spawn_mob('testmap', 29, 25, 1002, 1)[1]", aid);
    let mob: u32 = r.trim().parse().expect("mob id");
    let mut w = Writer::new(0x0089);
    w.u32(mob).u8(7);
    send(&mut g, 3, w);
    let mut dead = false;
    let mut hits = 0;
    for _ in 0..60 {
        pump(&mut g, 250);
        let o = cap.take(3);
        hits += o.iter().filter(|p| id_of(p) == 0x008a).count();
        if o.iter().any(|p| id_of(p) == 0x0080 && p[6] == 1 && u32::from_le_bytes(p[2..6].try_into().unwrap()) == mob) {
            dead = true;
            break;
        }
    }
    assert!(dead, "poring died after {hits} actions");
    let c = g.store.chars.get(&cid).unwrap();
    assert!(c.base_exp > 0 || c.base_level > 1, "got base exp");

    // ---- save on logout
    g.handle(NetEvent::Disconnected { sid: 3 });
    let saved = std::fs::read_to_string(g.cfg.save_dir.join(format!("chars/{cid}.json"))).unwrap();
    assert!(saved.contains("\"testmap\""));
}

#[test]
fn script_reload_keeps_world_running() {
    let (mut g, _cap) = game();
    let msg = g.reload_scripts().unwrap();
    assert!(msg.contains("npcs"), "{msg}");
    assert_eq!(g.console_eval("1 + 1", 0), "2");
    assert!(g.console_eval("this is not lua", 0).starts_with("error"));
}
