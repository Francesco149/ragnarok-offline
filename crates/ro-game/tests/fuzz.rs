//! Packet fuzz: whatever a client sends, the server must not panic.
//!
//! Every id of the client's length table is sent many times to a logged-in
//! map session (and to login and char sessions), framed the way the network
//! layer frames it (the table's fixed length, or a length word), with a
//! payload of plausible values: ids of units that exist, inventory indices,
//! small numbers, coordinates near the player, text. The player stands
//! among monsters, a script NPC, a shop and items on the floor, carries
//! items and zeny, knows every skill and has a cart, so most handlers get
//! past their first check. A fixed seed keeps it repeatable; FUZZ_SEED and
//! FUZZ_ROUNDS change it.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use ro_game::model::Point;
use ro_game::{Game, GameConfig, NetEvent, Outbox, Resources, ServerKind};
use ro_grf::Vfs;
use ro_proto::lengths::{client_send_len, PacketLen};
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
    fn take(&self) -> Vec<Vec<u8>> {
        self.0.lock().unwrap().drain(..).map(|(_, d)| d).collect()
    }
}

fn find(pkts: &[Vec<u8>], id: u16) -> Option<&Vec<u8>> {
    pkts.iter().find(|p| p.len() >= 2 && u16::from_le_bytes([p[0], p[1]]) == id)
}

fn gat(w: u32, h: u32) -> Vec<u8> {
    let mut b = b"GRAT\x01\x02".to_vec();
    b.extend_from_slice(&w.to_le_bytes());
    b.extend_from_slice(&h.to_le_bytes());
    b.resize(b.len() + (w * h * 20) as usize, 0);
    b
}

/// xorshift64*: small, seedable, good enough.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n.max(1)
    }
    fn pick<T: Copy>(&mut self, v: &[T]) -> T {
        v[self.below(v.len() as u64) as usize]
    }
}

const NPCS: &str = r#"
npc { map = "testmap", x = 27, y = 25, dir = 4, sprite = 97, name = "Talker",
  on_click = function()
    mes "[Talker]" next()
    local c = menu("Number", "Text", "Items", "Warp", "Bye")
    if c == 1 then mes(tostring(input_number())) close()
    elseif c == 2 then mes(input_string()) close()
    elseif c == 3 then give_item(501, 2) take_item(501, 1) player.zeny = player.zeny + 10 close()
    elseif c == 4 then warp("testmap", 24, 24) close()
    else close() end
  end }
shop { map = "testmap", x = 23, y = 25, dir = 4, sprite = 83, name = "Seller", items = { { 501 }, { 1201 }, { 1750 }, { 2301, 5 } } }
"#;

/// A floor under the kinds of server packet a default run draws: 43 to 48
/// measured (the clock and the monsters make runs differ; the test prints it).
const MIN_REPLY_KINDS: usize = 35;

/// What a client may send next when the server sends the first id.
const FOLLOW_UPS: &[(u16, &[u16])] = &[
    (0x00b5, &[0x00b9, 0x00b9, 0x0146]),         // next button
    (0x00b6, &[0x0146]),                         // close button
    (0x00b7, &[0x00b8]),                         // menu
    (0x0142, &[0x0143]),                         // number box
    (0x01d4, &[0x01d5]),                         // text box
    (0x00c4, &[0x00c5]),                         // shop: buy or sell
    (0x00c6, &[0x00c8]),                         // buy list
    (0x00c7, &[0x00c9]),                         // sell list
    (0x00f8, &[0x00f3, 0x00f5, 0x00f7]),         // storage closed
    (0x00f2, &[0x00f3, 0x00f5, 0x00f7, 0x00f3]), // storage open
    (0x0177, &[0x0178]),                         // items to identify
    (0x017b, &[0x017c]),                         // items a card fits
    (0x01ad, &[0x01ae]),                         // arrows to make
    (0x018d, &[0x018e]),                         // things to forge
    (0x011c, &[0x011b]),                         // warp portal places
    (0x01a6, &[0x01a7]),                         // pet eggs
    (0x0122, &[0x0126, 0x0127, 0x0128, 0x0129]), // cart contents
    (0x0123, &[0x0126, 0x0127, 0x0128, 0x0129]),
];

struct F {
    g: Game,
    cap: Cap,
    aid: u32,
    next_sid: u32,
    map_sid: u32,
    /// ids worth putting into a packet: the player, units around, items on the floor
    ids: Vec<u32>,
}

impl F {
    fn new() -> F {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let base = std::env::temp_dir().join(format!("ro-offline-fuzz-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        // the repo's rules and commands with our own two NPCs: no imported world to load
        let scripts = base.join("scripts");
        std::fs::create_dir_all(scripts.join("rules")).unwrap();
        for f in std::fs::read_dir(root.join("scripts/rules")).unwrap().flatten() {
            std::fs::copy(f.path(), scripts.join("rules").join(f.file_name())).unwrap();
        }
        std::fs::copy(root.join("scripts/commands.lua"), scripts.join("commands.lua")).unwrap();
        std::fs::write(scripts.join("fuzz.lua"), NPCS).unwrap();
        let mut vfs = Vfs::new();
        vfs.add_memory_file("data\\testmap.gat", gat(50, 50));
        let mut content = ro_game::game::default_content();
        content.insert("castle_monsters".into(), false);
        let cfg = GameConfig {
            content,
            save_dir: base.join("saves"),
            script_dir: scripts,
            db_dir: root.join("data/db"),
            start_point: Point::new("testmap", 25, 25),
            ..Default::default()
        };
        let cap = Cap::default();
        let g = Game::new(cfg, Resources::new(vfs), Box::new(cap.clone())).expect("game");
        let mut f = F { g, cap, aid: 0, next_sid: 1, map_sid: 0, ids: vec![] };
        f.enter(true);
        f
    }

    fn send(&mut self, sid: u32, data: Vec<u8>) {
        let shown = data.iter().take(48).map(|b| format!("{b:02x}")).collect::<String>();
        let r = catch_unwind(AssertUnwindSafe(|| self.g.handle(NetEvent::Packet { sid, data })));
        assert!(r.is_ok(), "the server panicked on packet {shown}");
    }

    /// Log in, pick (or first make) the character, enter the map.
    fn enter(&mut self, first: bool) {
        let (s1, s2, s3) = (self.next_sid, self.next_sid + 1, self.next_sid + 2);
        self.next_sid += 3;
        self.g.handle(NetEvent::Connected { sid: s1, kind: ServerKind::Login });
        let mut w = Writer::new(0x0064);
        w.u32(1).str_fixed("fuzzer", 24).str_fixed("fuzzpass", 24).u8(20);
        self.send(s1, w.finish());
        let out = self.cap.take();
        let mut r = Reader::new(find(&out, 0x0069).expect("login accepted"));
        r.seek(4);
        let (l1, aid, l2) = (r.u32().unwrap(), r.u32().unwrap(), r.u32().unwrap());
        self.g.handle(NetEvent::Disconnected { sid: s1 });
        self.g.handle(NetEvent::Connected { sid: s2, kind: ServerKind::Char });
        let mut w = Writer::new(0x0065);
        w.u32(aid).u32(l1).u32(l2).u16(0).u8(1);
        self.send(s2, w.finish());
        if first {
            let mut w = Writer::new(0x0067);
            w.str_fixed("Fuzzy", 24).bytes(&[5, 5, 5, 5, 5, 5]).u8(0).u16(1).u16(1);
            self.send(s2, w.finish());
        }
        self.cap.take();
        let mut w = Writer::new(0x0066);
        w.u8(0);
        self.send(s2, w.finish());
        let out = self.cap.take();
        let cid = Reader::new(find(&out, 0x0071).expect("character chosen")).u32().unwrap();
        self.g.handle(NetEvent::Disconnected { sid: s2 });
        self.g.handle(NetEvent::Connected { sid: s3, kind: ServerKind::Map });
        let mut w = Writer::new(0x0072);
        w.u32(aid).u32(cid).u32(l1).u32(0).u8(1);
        self.send(s3, w.finish());
        self.send(s3, Writer::new(0x007d).finish());
        self.cap.take();
        self.aid = aid;
        self.map_sid = s3;
    }

    fn in_world(&self) -> bool {
        self.g.world.units.get(&self.aid).is_some_and(|u| u.player().is_some())
    }

    /// Put the player back in a busy spot: alive, rich, skilled, with company.
    fn stage(&mut self) {
        let lua = r#"
            warp("testmap", 25, 25)
            player.zeny = 500000
            if player.base_level < 60 then player.base_level = 60 end
            heal(30000, 30000)
            set_option(8)
            for _, it in ipairs({ { 501, 30 }, { 1201, 1 }, { 2301, 1 }, { 1750, 200 }, { 4001, 1 }, { 602, 3 }, { 611, 2 }, { 9001, 1 }, { 619, 1 }, { 1101, 1 }, { 2101, 1 }, { 717, 5 } }) do
              if count_item(it[1]) < it[2] then give_item(it[1], it[2]) end
            end
            local n = 0
            for _, u in ipairs(nearby(12)) do if u.kind == "mob" then n = n + 1 end end
            if n < 6 then spawn_mob("testmap", 26, 26, 1002, 3) spawn_mob("testmap", 24, 23, 1063, 2) spawn_mob("testmap", 28, 27, 1113, 1) end
        "#;
        let r = self.g.console_eval(lua, self.aid);
        assert!(!r.starts_with("error"), "staging: {r}");
        self.ids = self.g.world.units.keys().copied().collect();
        self.ids.push(self.aid);
        self.ids.sort(); // the map's order changes from run to run
        self.cap.take();
    }

    fn payload(&mut self, rng: &mut Rng, n: usize) -> Vec<u8> {
        const TEXT: &[&str] = &["Fuzzy : hello", "@where", "@item 501 5", "@allskill", "@heal", "@warp testmap 20 20", "Poring", "", "/memo", "Fuzzy", "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"];
        let mut b = Vec::with_capacity(n + 8);
        while b.len() < n {
            match rng.below(12) {
                0..=2 => {
                    let id = if self.ids.is_empty() { 0 } else { self.ids[rng.below(self.ids.len() as u64) as usize] };
                    b.extend_from_slice(&id.to_le_bytes());
                }
                3 => b.extend_from_slice(&(rng.below(12) as u16 + 2).to_le_bytes()), // an inventory index
                4 => b.extend_from_slice(&(rng.pick(&[1u16, 2, 5, 10, 28, 51, 142, 501, 1201, 1750, 0xffff, 30000]).to_le_bytes())),
                5 => b.extend_from_slice(&rng.pick(&[0u32, 1, 2, 100, 0x7fff_ffff, 0xffff_ffff, 1_000_000_000]).to_le_bytes()),
                6 => b.extend_from_slice(&ro_proto::pack_pos3(20 + rng.below(12) as u16, 20 + rng.below(12) as u16, rng.below(8) as u8)),
                7 => b.extend_from_slice(rng.pick(TEXT).as_bytes()),
                8 => b.push(0),
                9 => b.push(rng.below(256) as u8),
                _ => b.push(rng.below(12) as u8),
            }
        }
        b.truncate(n);
        b
    }

    fn packet(&mut self, rng: &mut Rng, id: u16) -> Option<Vec<u8>> {
        let mut p = id.to_le_bytes().to_vec();
        match client_send_len(id)? {
            PacketLen::Fixed(n) => {
                let body = self.payload(rng, n as usize - 2);
                p.extend_from_slice(&body);
            }
            PacketLen::Variable => {
                let n = rng.pick(&[0usize, 1, 2, 4, 6, 8, 12, 24, 40, 80, 200]);
                let body = self.payload(rng, n);
                p.extend_from_slice(&((n + 4) as u16).to_le_bytes());
                p.extend_from_slice(&body);
            }
        }
        Some(p)
    }
}

#[test]
fn no_packet_panics_the_server() {
    let seed = std::env::var("FUZZ_SEED").ok().and_then(|s| s.parse().ok()).unwrap_or(0x5eed_2003_1218u64);
    let rounds: u32 = std::env::var("FUZZ_ROUNDS").ok().and_then(|s| s.parse().ok()).unwrap_or(40);
    let mut rng = Rng(seed);
    let mut f = F::new();
    let ids: Vec<u16> = (0u16..0x200).filter(|&i| client_send_len(i).is_some()).collect();
    assert!(ids.len() > 300, "the length table is loaded");
    let mut sent = 0u32;
    let mut replies = std::collections::BTreeSet::new();
    for _ in 0..rounds {
        for &id in &ids {
            if id == 0x0072 {
                continue; // entering again is what `enter` does
            }
            if !f.in_world() {
                f.g.handle(NetEvent::Disconnected { sid: f.map_sid });
                f.enter(false);
            }
            if sent % 97 == 0 {
                f.stage();
            }
            let Some(p) = f.packet(&mut rng, id) else { continue };
            let sid = f.map_sid;
            f.send(sid, p);
            sent += 1;
            if sent % 13 == 0 {
                let r = catch_unwind(AssertUnwindSafe(|| {
                    f.g.skew(rng.pick(&[20u64, 150, 600, 3000]));
                    f.g.tick();
                }));
                assert!(r.is_ok(), "the server panicked in a tick after {sent} packets (seed {seed})");
            }
            // answer what the server asks, so that dialogues, shops and storage get past their first step
            let mut depth = 0;
            loop {
                let got = f.cap.take();
                replies.extend(got.iter().filter(|p| p.len() >= 2).map(|p| u16::from_le_bytes([p[0], p[1]])));
                // (what to send, the unit the server's packet named: an NPC window answers to its NPC)
                let follow: Vec<(u16, Option<u32>)> = got
                    .iter()
                    .filter(|p| p.len() >= 2)
                    .filter_map(|p| {
                        let asked = u16::from_le_bytes([p[0], p[1]]);
                        let about = if asked == 0x00b7 { p.get(4..8) } else { p.get(2..6) }.map(|b| u32::from_le_bytes(b.try_into().unwrap()));
                        FOLLOW_UPS.iter().find(|(a, _)| *a == asked).map(|(_, a)| (rng.pick(a), about))
                    })
                    .collect();
                depth += 1;
                if follow.is_empty() || depth > 6 || !f.in_world() {
                    break;
                }
                for (id, about) in follow {
                    if let Some(mut p) = f.packet(&mut rng, id) {
                        let at = if matches!(client_send_len(id), Some(PacketLen::Variable)) { 4 } else { 2 };
                        if let (Some(about), true, true) = (about, p.len() >= at + 4, rng.below(4) != 0) {
                            p[at..at + 4].copy_from_slice(&about.to_le_bytes());
                        }
                        let sid = f.map_sid;
                        f.send(sid, p);
                        sent += 1;
                    }
                }
            }
        }
    }
    // the fuzz is only worth something if it gets past the handlers' first checks
    eprintln!("{sent} packets drew {} kinds of reply: {}", replies.len(), replies.iter().map(|i| format!("{i:04x}")).collect::<Vec<_>>().join(" "));
    assert!(replies.len() >= MIN_REPLY_KINDS, "only {} kinds of reply: the payloads no longer reach the handlers", replies.len());
    // the same ids against the login and char servers, logged out and in
    for kind in [ServerKind::Login, ServerKind::Char] {
        let sid = f.next_sid;
        f.next_sid += 1;
        f.g.handle(NetEvent::Connected { sid, kind });
        for _ in 0..4 {
            for &id in &ids {
                if let Some(p) = f.packet(&mut rng, id) {
                    f.send(sid, p);
                }
            }
        }
        f.cap.take();
    }
    // Rule errors are not crashes, but each is an input a rule did not expect.
    let mut errors: Vec<String> = f.g.notices.iter().filter(|n| n.contains("error")).map(|n| n.chars().take(300).collect()).collect();
    errors.sort();
    errors.dedup();
    assert!(errors.is_empty(), "{} kinds of script error after {sent} packets (seed {seed}):\n{}", errors.len(), errors.join("\n"));
    // and the player can still be talked to
    if !f.in_world() {
        f.enter(false);
    }
    assert!(f.g.console_eval("1 + 1", f.aid) == "2");
}
