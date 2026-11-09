//! - embeds third_party/ddrawcompat/ddraw.dll when tools/fetch-third-party.sh
//!   has fetched it (cfg `ddrawcompat`); without it the launcher still works
//!   but cannot fix DirectX 7 on modern Windows.
//! - Windows targets: compiles assets/launcher.rc (manifest + version info)
//!   with mingw's windres and links it into the exe.

use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(ddrawcompat)");
    println!("cargo::rustc-check-cfg=cfg(euro_patch)");
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let ddc = root.join("../../third_party/ddrawcompat/ddraw.dll");
    println!("cargo::rerun-if-changed={}", ddc.display());
    println!("cargo::rerun-if-changed=assets");
    if ddc.exists() {
        println!("cargo::rustc-cfg=ddrawcompat");
        println!("cargo::rustc-env=DDRAWCOMPAT_DLL={}", ddc.canonicalize().unwrap().display());
    } else {
        println!("cargo::warning=third_party/ddrawcompat/ddraw.dll missing (run tools/fetch-third-party.sh); building without DDrawCompat");
    }

    // the euRO patch server snapshot (Wayback captures), for the local patch server
    let ep = root.join("../../third_party/euro-patch");
    println!("cargo::rerun-if-changed={}", ep.display());
    let files = [
        ("EURO_PATCH_ALLOW", "patch_allow.txt.20040718192102"),
        ("EURO_PATCH_NOTICE", "notice_patch.txt.20040718124248"),
        ("EURO_PATCH_CLOSE", "notice_close.txt.20040718054701"),
        ("EURO_PATCH_LIST", "patch2.txt.20040221010205"),
        ("EURO_WEB_INDEX", "index.html.20040221010012"),
    ];
    if files.iter().all(|(_, f)| ep.join(f).exists()) {
        println!("cargo::rustc-cfg=euro_patch");
        for (env, f) in files {
            println!("cargo::rustc-env={env}={}", ep.join(f).canonicalize().unwrap().display());
        }
    } else {
        println!("cargo::warning=third_party/euro-patch missing (run tools/fetch-third-party.sh); the launcher will skip the CD's patcher");
    }

    let target = std::env::var("TARGET").unwrap();
    if !target.contains("windows") {
        return;
    }
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("launcher.res.o");
    let windres = if target.contains("gnu") {
        format!("{}-w64-mingw32-windres", target.split('-').next().unwrap())
    } else {
        "windres".into()
    };
    let status = Command::new(&windres)
        .current_dir(root.join("assets"))
        .args(["--input", "launcher.rc", "--output-format=coff", "--output"])
        .arg(&out)
        .status();
    match status {
        Ok(s) if s.success() => println!("cargo::rustc-link-arg-bins={}", out.display()),
        _ => println!("cargo::warning={windres} failed: no manifest/version resource in the exe"),
    }
}
