//! The path finder against the client's own: `data/client_paths.json` holds
//! synthetic grids, start and target cells, and the path the exe's
//! `CPathFinder` (Ragexe 2003-12-18, `0x4d2300`) returned for each, run
//! under an emulator (docs/CLIENT.md, "The path finder"). `null`: the client
//! found no path (none, or more than its 149 nodes needed). Set
//! `CLIENT_PATHS=<file>` to check another file of the same form.

use ro_data::MapGrid;
use ro_game::path::{self, DIRS, client_search};

fn check(file: &str) -> usize {
    let text = std::fs::read_to_string(file).unwrap_or_else(|e| panic!("{file}: {e}"));
    let cases: serde_json::Value = serde_json::from_str(&text).unwrap();
    let mut n = 0;
    let mut bad = Vec::new();
    for (k, c) in cases.as_array().unwrap().iter().enumerate() {
        let (w, h) = (c["w"].as_u64().unwrap() as u16, c["h"].as_u64().unwrap() as u16);
        let mut g = MapGrid::open("t", w, h);
        for (i, ch) in c["cells"].as_str().unwrap().chars().enumerate() {
            if ch == '1' {
                g.set_walkable((i % w as usize) as i32, (i / w as usize) as i32, false);
            }
        }
        for p in c["pairs"].as_array().unwrap() {
            let v: Vec<i32> = (0..4).map(|i| p[i].as_i64().unwrap() as i32).collect();
            let want = p[4].as_str().map(|s| s.bytes().map(|b| b - b'0').collect::<Vec<u8>>());
            let got = client_search(&g, v[0], v[1], v[2], v[3]);
            if let Some(dirs) = &got {
                let (mut x, mut y) = (v[0], v[1]);
                for d in dirs {
                    x += DIRS[*d as usize].0;
                    y += DIRS[*d as usize].1;
                }
                assert_eq!((x, y), (v[2], v[3]), "case {k}: path does not end at the target");
            }
            if got != want {
                bad.push(format!("case {k} {v:?}: client {want:?}, ours {got:?}"));
            }
            n += 1;
        }
    }
    assert!(bad.is_empty(), "{} of {n} paths differ:\n{}", bad.len(), bad[..bad.len().min(10)].join("\n"));
    n
}

/// `data/client_walk_times.json`: rows that also carry a sub-cell, a speed
/// and the move's tick, and the time the exe gave each cell of the path.
#[test]
fn walk_times_match_the_client() {
    let file = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/client_walk_times.json");
    let cases: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(file).unwrap()).unwrap();
    let (mut n, mut bad) = (0, Vec::new());
    for c in cases.as_array().unwrap() {
        for p in c["pairs"].as_array().unwrap() {
            let v: Vec<i64> = (0..8).map(|i| p[i].as_i64().unwrap()).collect();
            let (Some(dirs), Some(want)) = (p[8].as_str(), p[9].as_array()) else { continue };
            let dirs: Vec<u8> = dirs.bytes().map(|b| b - b'0').collect();
            let start = v[7] as u32;
            let got: Vec<u32> = path::client_times(v[0] as i32, v[1] as i32, (v[4] as u8, v[5] as u8), &dirs, v[6] as u32)
                .iter()
                .map(|t| start.wrapping_add(*t))
                .collect();
            let want: Vec<u32> = want.iter().map(|t| t.as_u64().unwrap() as u32).collect();
            if got != want {
                bad.push(format!("{v:?} {dirs:?}: client {want:?}, ours {got:?}"));
            }
            n += 1;
        }
    }
    assert!(n > 500);
    assert!(bad.is_empty(), "{} of {n} differ:\n{}", bad.len(), bad[..bad.len().min(10)].join("\n"));
}

#[test]
fn paths_match_the_client() {
    let n = check(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/client_paths.json"));
    assert!(n > 1000);
    if let Ok(f) = std::env::var("CLIENT_PATHS") {
        check(&f);
    }
}
