//! - embeds third_party/ddrawcompat/ddraw.dll when tools/fetch-third-party.sh
//!   has fetched it (cfg `ddrawcompat`); without it the launcher still works
//!   but cannot fix DirectX 7 on modern Windows.
//! - Windows targets: compiles assets/launcher.rc (manifest + version info)
//!   with mingw's windres and links it into the exe.

use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(ddrawcompat)");
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
