//! prepare / is_prepared / unprepare on a synthetic client folder: what the
//! Windows launcher and the all-in-one installer's custom actions rely on.

use std::path::PathBuf;

use ro_grf::{Grf, Vfs};
use ro_server::setup;

fn tmpdir(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("ro-setup-test-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn grf_with(path: &std::path::Path, files: &[(&str, &[u8])]) {
    let mut v = Vfs::new();
    for (n, d) in files {
        v.add_memory_file(n, d.to_vec());
    }
    ro_grf::write_grf_0x200(&v, path).unwrap();
}

#[test]
fn prepare_merges_patches_and_unprepare_restores() {
    let dir = tmpdir("roundtrip");
    grf_with(&dir.join("data.grf"), &[("data\\base.txt", b"base"), ("data\\shared.txt", b"old")]);
    // a .gpf patch layer (0x200 here; the real ones are 0x102) overriding one file
    grf_with(&dir.join("2003-12-11gdata_test.gpf"), &[("data\\shared.txt", b"new"), ("data\\patch.txt", b"patch")]);
    let original = std::fs::read(dir.join("data.grf")).unwrap();

    assert!(!setup::is_prepared(&dir, "127.0.0.1", 6900));
    let mut calls = 0;
    let p = setup::prepare(&dir, "127.0.0.1", 6900, "Offline", &mut |_, _| calls += 1).unwrap();
    assert!(calls > 0);
    assert_eq!(p.layers, vec!["2003-12-11gdata_test.gpf".to_string()]);
    assert!(setup::is_prepared(&dir, "127.0.0.1", 6900));
    assert!(!setup::is_prepared(&dir, "127.0.0.1", 7000));
    assert_eq!(std::fs::read(dir.join("data.grf.original")).unwrap(), original);

    let g = Grf::open(dir.join("data.grf")).unwrap();
    assert_eq!(g.read("data\\base.txt").unwrap(), b"base");
    assert_eq!(g.read("data\\shared.txt").unwrap(), b"new", "patch layer wins");
    assert_eq!(g.read("data\\patch.txt").unwrap(), b"patch");
    let xml = String::from_utf8(g.read(setup::CLIENTINFO).unwrap()).unwrap();
    assert!(xml.contains("<address>127.0.0.1</address>") && xml.contains("<port>6900</port>"));
    drop(g);

    // idempotent: a second run rebuilds from data.grf.original
    setup::prepare(&dir, "127.0.0.1", 6900, "Offline", &mut |_, _| {}).unwrap();
    assert_eq!(std::fs::read(dir.join("data.grf.original")).unwrap(), original);

    assert!(setup::unprepare(&dir).unwrap());
    assert_eq!(std::fs::read(dir.join("data.grf")).unwrap(), original, "CD's data.grf is back, byte for byte");
    assert!(!dir.join("data.grf.original").exists());
    assert!(!setup::unprepare(&dir).unwrap(), "nothing to undo the second time");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn prepare_refuses_a_folder_without_data_grf() {
    let dir = tmpdir("empty");
    assert!(setup::prepare(&dir, "127.0.0.1", 6900, "Offline", &mut |_, _| {}).is_err());
    let _ = std::fs::remove_dir_all(&dir);
}
