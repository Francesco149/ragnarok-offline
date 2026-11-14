//! remove_warp() takes out an imported warp, and a warp the same file puts
//! back on that cell stays (until 2026-11-14 it went too, and both of
//! scripts/npc/castle_warps.lua's tiles led nowhere).

use std::path::PathBuf;

use ro_game::script::ScriptEngine;

fn to(e: &ScriptEngine, map: &str, x: u16, y: u16) -> Vec<(String, u16, u16)> {
    e.defs.warps.iter().filter(|w| w.map == map && (w.x, w.y) == (x, y)).map(|w| w.to.clone()).collect()
}

#[test]
fn replaced_castle_warps_stay() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../scripts");
    let e = ScriptEngine::load(&dir).expect("scripts");
    assert_eq!(to(&e, "aldeg_cas05", 188, 49), vec![("aldeg_cas05".into(), 165, 228)]);
    assert_eq!(to(&e, "prtg_cas02", 184, 44), vec![("prtg_cas02".into(), 189, 166)]);
    assert_eq!(to(&e, "prtg_cas02", 189, 162), vec![("prtg_cas02".into(), 184, 40)]);
    assert!(to(&e, "prtg_cas02", 157, 135).is_empty(), "the later map's way back is gone");
}
