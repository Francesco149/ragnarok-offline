//! The NPC runtime the imported Athena NPCs rely on, over the real protocol:
//! a converter-style NPC (menu + goto, the three kinds of variables, events,
//! an NPC timer, an OnInit), overriding an NPC by `src`, the skill tree and
//! skill points, Basic Skill gating, and Kafra storage. Synthetic map and a
//! throwaway script directory; the repo's rules and data/db are used.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

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

fn id_of(p: &[u8]) -> u16 {
    u16::from_le_bytes([p[0], p[1]])
}
fn find(pkts: &[Vec<u8>], id: u16) -> Option<&Vec<u8>> {
    pkts.iter().find(|p| p.len() >= 2 && id_of(p) == id)
}
fn texts(pkts: &[Vec<u8>]) -> Vec<String> {
    pkts.iter().filter(|p| id_of(p) == 0x00b4).map(|p| Reader::new(p).seek(8).str_rest()).collect()
}

fn gat(w: u32, h: u32) -> Vec<u8> {
    let mut b = b"GRAT\x01\x02".to_vec();
    b.extend_from_slice(&w.to_le_bytes());
    b.extend_from_slice(&h.to_le_bytes());
    b.resize(b.len() + (w * h * 20) as usize, 0);
    b
}

const TEST_NPCS: &str = r#"
-- shaped like tools/import/athena_npc.py output
npc {
  map = "testmap", x = 26, y = 25, dir = 4, sprite = 97, name = "Tester",
  id = "tester", events = { "OnInit", "OnPing", "OnTimer100" }, src = "test:tester",
  on_click = function(at)
    local c
    if at then
      if at == "OnInit" then goto OnInit
      elseif at == "OnPing" then goto OnPing
      elseif at == "OnTimer100" then goto OnTimer100
      end
    end
    mes "[Tester]"
    v.visits = v.visits + 1
    tmp.t = tmp.t + 2
    g.count = g.count + 1
    c = menu("Count", "Ping", "-")
    if c == 1 then goto LA elseif c == 2 then goto LB end
    mes "fell through"
    close()
    do return end
    ::LA::
    mes("visits " .. v.visits .. " tmp " .. tmp.t .. " global " .. g.count)
    close()
    do return end
    ::LB::
    do_event("tester::OnPing")
    close()
    do return end
    ::OnInit::
    g.inited = g.inited + 1
    do return end
    ::OnPing::
    v.pinged = 1
    npc_timer_start()
    do return end
    ::OnTimer100::
    g.timer = 1
  end,
}

npc { map = "testmap", x = 30, y = 30, sprite = 97, name = "Old", src = "test:old",
  on_click = function() mes "old" close() end }

-- a hand-written override replacing it, like scripts/npc/ does for imports
npc { map = "testmap", x = 30, y = 30, sprite = 97, name = "New", replaces = "test:old",
  on_click = function() mes "new" close() end }

-- code around an imported NPC, like scripts/npc/jobs.lua
npc { map = "testmap", x = 32, y = 30, sprite = 97, name = "Wrapped", src = "test:wrapped",
  on_click = function() mes "inner" close() end }
wrap_npc("test:wrapped", function(orig)
  return function(at) mes "outer" return orig(at) end
end)

-- an endless loop must not freeze the server
npc { map = "testmap", x = 20, y = 20, sprite = 97, name = "Loop",
  on_click = function() ::again:: v.n = 1 goto again end }

-- invisible touch area (Athena sprite -1): runs on_click when stepped on
npc { map = "testmap", x = 40, y = 25, sprite = -1, name = "Trap", touch = { 1, 1 },
  on_click = function() g.trapped = g.trapped + 1 end }
"#;

fn setup() -> (Game, Cap, PathBuf) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let base = std::env::temp_dir().join(format!("ro-offline-npcrt-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let scripts = base.join("scripts");
    std::fs::create_dir_all(scripts.join("rules")).unwrap();
    for f in std::fs::read_dir(root.join("scripts/rules")).unwrap().flatten() {
        std::fs::copy(f.path(), scripts.join("rules").join(f.file_name())).unwrap();
    }
    std::fs::write(scripts.join("test.lua"), TEST_NPCS).unwrap();
    let mut vfs = Vfs::new();
    vfs.add_memory_file("data\\testmap.gat", gat(50, 50));
    vfs.add_memory_file("data\\job_sword1.gat", gat(50, 50)); // noteleport in data/db/mapflags.toml
    let cfg = GameConfig {
        save_dir: base.join("saves"),
        script_dir: scripts,
        db_dir: root.join("data/db"),
        start_point: Point::new("testmap", 25, 25),
        ..Default::default()
    };
    let cap = Cap::default();
    let g = Game::new(cfg, Resources::new(vfs), Box::new(cap.clone())).expect("game");
    (g, cap, base)
}

fn send(g: &mut Game, sid: u32, mut w: Writer) {
    g.handle(NetEvent::Packet { sid, data: w.finish() });
}

fn pump(g: &mut Game, ms: u64) {
    let end = Instant::now() + Duration::from_millis(ms);
    while Instant::now() < end {
        g.tick();
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// Log in, make a character, enter the map. Returns the player unit (= account id).
fn enter(g: &mut Game, cap: &Cap) -> u32 {
    g.handle(NetEvent::Connected { sid: 1, kind: ServerKind::Login });
    let mut w = Writer::new(0x0064);
    w.u32(1).str_fixed("npcuser", 24).str_fixed("npcpass", 24).u8(20);
    send(g, 1, w);
    let out = cap.take(1);
    let mut r = Reader::new(find(&out, 0x0069).unwrap());
    r.seek(4);
    let (l1, aid, l2) = (r.u32().unwrap(), r.u32().unwrap(), r.u32().unwrap());
    g.handle(NetEvent::Connected { sid: 2, kind: ServerKind::Char });
    let mut w = Writer::new(0x0065);
    w.u32(aid).u32(l1).u32(l2).u16(0).u8(1);
    send(g, 2, w);
    cap.take(2);
    let mut w = Writer::new(0x0067);
    w.str_fixed("Npcy", 24).bytes(&[9, 1, 1, 1, 9, 9]).u8(0).u16(0).u16(1);
    send(g, 2, w);
    cap.take(2);
    let mut w = Writer::new(0x0066);
    w.u8(0);
    send(g, 2, w);
    let z = cap.take(2);
    let cid = Reader::new(find(&z, 0x0071).unwrap()).u32().unwrap();
    g.handle(NetEvent::Connected { sid: 3, kind: ServerKind::Map });
    let mut w = Writer::new(0x0072);
    w.u32(aid).u32(cid).u32(l1).u32(0).u8(1);
    send(g, 3, w);
    send(g, 3, Writer::new(0x007d));
    aid
}

fn npc_named(g: &Game, name: &str) -> Option<u32> {
    g.world
        .units
        .values()
        .find(|u| u.name == name && matches!(u.kind, UnitKind::Npc(NpcUnit { kind: NpcKind::Script { .. }, .. })))
        .map(|u| u.id)
}

fn click(g: &mut Game, npc: u32) {
    let mut w = Writer::new(0x0090);
    w.u32(npc).u8(1);
    send(g, 3, w);
}
fn choose(g: &mut Game, npc: u32, n: u8) {
    let mut w = Writer::new(0x00b8);
    w.u32(npc).u8(n);
    send(g, 3, w);
}
fn close_dialog(g: &mut Game, npc: u32) {
    let mut w = Writer::new(0x0146);
    w.u32(npc);
    send(g, 3, w);
}

#[test]
fn npc_runtime() {
    let (mut g, cap, base) = setup();
    let me = enter(&mut g, &cap);
    let first = cap.take(3);
    assert_eq!(g.console_eval("g.inited", me), "1", "OnInit ran once at load");

    // the menu, goto and the three variable kinds
    let tester = npc_named(&g, "Tester").expect("tester placed");
    click(&mut g, tester);
    assert!(find(&cap.take(3), 0x00b7).is_some(), "menu shown");
    choose(&mut g, tester, 1);
    let t = texts(&cap.take(3));
    assert!(t.iter().any(|s| s == "visits 1 tmp 2 global 1"), "{t:?}");
    close_dialog(&mut g, tester);
    click(&mut g, tester);
    choose(&mut g, tester, 1);
    assert!(texts(&cap.take(3)).iter().any(|s| s == "visits 2 tmp 4 global 2"));
    close_dialog(&mut g, tester);
    // "-" falls through to the next statement
    click(&mut g, tester);
    choose(&mut g, tester, 3);
    assert!(texts(&cap.take(3)).iter().any(|s| s == "fell through"));
    close_dialog(&mut g, tester);

    // do_event -> OnPing (player attached) -> NPC timer -> OnTimer100
    click(&mut g, tester);
    choose(&mut g, tester, 2);
    close_dialog(&mut g, tester);
    pump(&mut g, 300);
    assert_eq!(g.console_eval("v.pinged", me), "1");
    assert_eq!(g.console_eval("g.timer", me), "1");

    // wrap_npc runs hand-written code first, then the original
    let wrapped = npc_named(&g, "Wrapped").expect("wrapped npc");
    click(&mut g, wrapped);
    assert!(texts(&cap.take(3)).ends_with(&["outer".to_string(), "inner".to_string()]));
    close_dialog(&mut g, wrapped);

    // the loop guard ends a runaway script; the server goes on
    let looper = npc_named(&g, "Loop").expect("loop npc");
    click(&mut g, looper);
    assert!(g.notices.iter().any(|n| n.contains("ran too long")), "{:?}", g.notices);
    cap.take(3);
    click(&mut g, tester);
    assert!(find(&cap.take(3), 0x00b7).is_some(), "still talking after the runaway script");
    close_dialog(&mut g, tester);

    // overrides by src, invisible touch NPCs
    let new = npc_named(&g, "New").expect("override placed");
    assert!(npc_named(&g, "Old").is_none(), "replaced NPC is gone");
    click(&mut g, new);
    assert!(texts(&cap.take(3)).iter().any(|s| s == "new"));
    close_dialog(&mut g, new);
    let trap = npc_named(&g, "Trap").expect("trap placed");
    assert!(!first.iter().any(|p| p.len() >= 6 && u32::from_le_bytes(p[2..6].try_into().unwrap()) == trap), "hidden NPCs aren't sent");
    let mut w = Writer::new(0x0085);
    w.pos3(40, 25, 0);
    send(&mut g, 3, w);
    pump(&mut g, 3000);
    assert_eq!(g.console_eval("g.trapped", me), "1");

    // skills: a novice sees Basic Skill, can't sit without level 3, spends points
    let first_list = find(&first, 0x010f).expect("skill list");
    assert_eq!(u16::from_le_bytes([first_list[4], first_list[5]]), 1, "NV_BASIC listed");
    let mut w = Writer::new(0x0089);
    w.u32(0).u8(2);
    send(&mut g, 3, w);
    assert!(find(&cap.take(3), 0x0110).is_some(), "sitting needs Basic Skill 3");
    g.console_eval("player.skill_point = 3", me);
    for _ in 0..3 {
        let mut w = Writer::new(0x0112);
        w.u16(1);
        send(&mut g, 3, w);
    }
    assert_eq!(g.console_eval("skill_level(1)", me), "3");
    assert_eq!(g.console_eval("player.skill_point", me), "0");
    let mut w = Writer::new(0x0089);
    w.u32(0).u8(2);
    send(&mut g, 3, w);
    assert!(find(&cap.take(3), 0x0110).is_none(), "now it can sit");
    // quest skills come from NPCs, not points
    g.console_eval("player.skill_point = 1", me);
    let mut w = Writer::new(0x0112);
    w.u16(142);
    send(&mut g, 3, w);
    assert_eq!(g.console_eval("skill_level(142)", me), "0");
    g.console_eval("give_skill(142, 1)", me);
    assert_eq!(g.console_eval("skill_level(142)", me), "1");

    // Kafra storage: open, put 3 of 5 potions, take them back, close (saved)
    g.console_eval("give_item(501, 5)", me);
    cap.take(3);
    g.console_eval("open_storage()", me);
    assert!(find(&cap.take(3), 0x00f2).is_some(), "storage opened");
    let slot = g.store.chars.values().next().unwrap().inventory.iter().position(|i| i.nameid == 501).unwrap() as u16;
    let mut w = Writer::new(0x00f3);
    w.u16(slot + 2).u32(3);
    send(&mut g, 3, w);
    assert!(find(&cap.take(3), 0x00f4).is_some());
    assert_eq!(g.console_eval("count_item(501)", me), "2");
    let mut w = Writer::new(0x00f5);
    w.u16(1).u32(3);
    send(&mut g, 3, w);
    assert!(find(&cap.take(3), 0x00f6).is_some());
    assert_eq!(g.console_eval("count_item(501)", me), "5");
    send(&mut g, 3, Writer::new(0x00f7));
    assert!(find(&cap.take(3), 0x00f8).is_some());

    // usable items translated from the era's item scripts
    let use_item = |g: &mut Game, cap: &Cap, nameid: u32| {
        g.console_eval(&format!("give_item({nameid}, 1)"), me);
        cap.take(3);
        let slot = g.store.chars.values().next().unwrap().inventory.iter().position(|i| i.nameid == nameid).unwrap() as u16;
        let mut w = Writer::new(0x00a7);
        w.u16(slot + 2).u32(me);
        send(g, 3, w);
    };
    let items_before = g.store.chars.values().next().unwrap().inventory.len();
    use_item(&mut g, &cap, 603); // Old Blue Box
    let inv = &g.store.chars.values().next().unwrap().inventory;
    assert!(inv.len() >= items_before && !inv.iter().any(|i| i.nameid == 603), "box opened into an item");
    let mobs_near = |g: &Game| g.world.units.values().filter(|u| u.mob().is_some() && u.map == "testmap").count();
    let before = mobs_near(&g);
    use_item(&mut g, &cap, 604); // Dead Branch
    assert_eq!(mobs_near(&g), before + 1, "a branch monster appeared");
    g.console_eval("save_point('testmap', 10, 10); warp('testmap', 30, 40)", me);
    use_item(&mut g, &cap, 602); // Butterfly Wing
    let u = g.world.units.get(&me).unwrap();
    assert_eq!((u.x, u.y), (10, 10), "back at the save point");

    // map flags: no Fly Wing on the Swordsman course, and the wing is kept
    g.console_eval("warp('job_sword1', 20, 20)", me);
    use_item(&mut g, &cap, 601); // Fly Wing
    let u = g.world.units.get(&me).unwrap();
    assert_eq!((u.map.as_str(), u.x, u.y), ("job_sword1", 20, 20), "no teleport here");
    assert!(find(&cap.take(3), 0x0189).is_some(), "the client's 'unable to teleport' message");
    assert_eq!(g.console_eval("count_item(601)", me), "1", "the wing is not used up");

    // pushcart: renting needs the Pushcart skill; potions go in and out
    assert_eq!(g.console_eval("set_cart()", me), "false", "no cart without Pushcart");
    g.console_eval("give_skill(39, 1); give_item(501, 10)", me);
    assert_eq!(g.console_eval("set_cart()", me), "true");
    assert!(find(&cap.take(3), 0x0121).is_some(), "cart counts sent");
    let slot = g.store.chars.values().next().unwrap().inventory.iter().position(|i| i.nameid == 501).unwrap() as u16;
    let mut w = Writer::new(0x0126);
    w.u16(slot + 2).u32(4);
    send(&mut g, 3, w);
    assert!(find(&cap.take(3), 0x0124).is_some());
    assert_eq!(g.store.chars.values().next().unwrap().cart.iter().map(|i| i.amount).sum::<u32>(), 4);
    let mut w = Writer::new(0x0127);
    w.u16(2).u32(4);
    send(&mut g, 3, w);
    assert!(find(&cap.take(3), 0x0125).is_some());
    assert!(g.store.chars.values().next().unwrap().cart.is_empty());
    send(&mut g, 3, Writer::new(0x012a));
    assert_eq!(g.console_eval("check_option(8)", me), "0", "cart taken off");

    // pets: tame a Poring with Unripe Apples (mod0631 rate ~20%), hatch, feed, egg again
    g.console_eval("warp('testmap', 25, 25)", me);
    let mut caught = false;
    for _ in 0..60 {
        let mob: u32 = g.console_eval("spawn_mob('testmap', 26, 25, 1002, 1)[1]", me).trim().parse().unwrap();
        use_item(&mut g, &cap, 619); // Unripe Apple
        assert!(find(&cap.take(3), 0x019e).is_some(), "taming mode");
        let mut w = Writer::new(0x019f);
        w.u32(mob);
        send(&mut g, 3, w);
        let r = cap.take(3);
        if find(&r, 0x01a0).is_some_and(|p| p[2] == 1) {
            caught = true;
            break;
        }
        g.console_eval(&format!("kill_monster('testmap', 'All')"), me);
        let _ = mob;
    }
    assert!(caught, "a Poring was tamed in 60 tries");
    let egg = g.store.chars.values().next().unwrap().inventory.iter().position(|i| i.nameid == 9001).expect("Poring egg") as u16;
    use_item(&mut g, &cap, 643); // Pet Incubator
    let list = cap.take(3);
    let eggs = find(&list, 0x01a6).expect("egg list");
    assert_eq!(u16::from_le_bytes([eggs[4], eggs[5]]), egg + 2);
    let mut w = Writer::new(0x01a7);
    w.u16(egg + 2);
    send(&mut g, 3, w);
    assert!(find(&cap.take(3), 0x01a2).is_some(), "pet status after hatching");
    assert!(g.world.units.values().any(|u| matches!(u.kind, UnitKind::Pet(_))), "pet out");
    g.console_eval("give_item(531, 1)", me); // Apple Juice
    cap.take(3);
    let mut w = Writer::new(0x01a1);
    w.u8(1);
    send(&mut g, 3, w);
    assert!(find(&cap.take(3), 0x01a3).is_some_and(|p| p[2] == 1), "fed");
    let mut w = Writer::new(0x01a1);
    w.u8(3);
    send(&mut g, 3, w);
    assert!(!g.world.units.values().any(|u| matches!(u.kind, UnitKind::Pet(_))), "back in the egg");
    assert_eq!(g.console_eval("count_item(9001)", me), "1");

    // server-wide variables persist
    g.save_all();
    let world = std::fs::read_to_string(base.join("saves/world.json")).unwrap();
    assert!(world.contains("\"count\": 5"), "{world}");
    let _ = std::fs::remove_dir_all(Path::new(&base));
}
