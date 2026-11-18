//! What the builder puts into the installer, embedded at build time:
//! - RO_SETUP_LAUNCHER: path to the RagnarokOffline.exe to ship, the 32-bit
//!   one that also runs on XP (tools/xp/build.sh builds it first and sets
//!   this). Without it the builder needs --launcher at run time.
//! - DDrawCompat (third_party/ddrawcompat, tools/fetch-third-party.sh) and
//!   ro-launcher's DDrawCompat.ini.
//! - BUILD_DATE: the commit date (or SOURCE_DATE_EPOCH), the date our files
//!   carry in the installer's cabinet.
//! - Windows: assets/builder.rc (manifest + version info).

use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(embedded_launcher)");
    println!("cargo::rustc-check-cfg=cfg(ddrawcompat)");
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo::rerun-if-env-changed=RO_SETUP_LAUNCHER");
    println!("cargo::rerun-if-env-changed=SOURCE_DATE_EPOCH");
    println!("cargo::rerun-if-changed=assets");
    if let Ok(p) = std::env::var("RO_SETUP_LAUNCHER") {
        let p = PathBuf::from(p).canonicalize().expect("RO_SETUP_LAUNCHER does not exist");
        println!("cargo::rerun-if-changed={}", p.display());
        println!("cargo::rustc-cfg=embedded_launcher");
        println!("cargo::rustc-env=RO_SETUP_LAUNCHER_PATH={}", p.display());
    }
    let ddc = root.join("../../third_party/ddrawcompat");
    println!("cargo::rerun-if-changed={}", ddc.display());
    if ["ddraw.dll", "LICENSE.txt", "VERSION"].iter().all(|f| ddc.join(f).exists()) {
        let ddc = ddc.canonicalize().unwrap();
        println!("cargo::rustc-cfg=ddrawcompat");
        println!("cargo::rustc-env=DDRAWCOMPAT_DIR={}", ddc.display());
    } else {
        println!("cargo::warning=third_party/ddrawcompat missing (run tools/fetch-third-party.sh): the builder will need --ddraw-dir");
    }
    let ini = root.join("../ro-launcher/assets/DDrawCompat.ini").canonicalize().unwrap();
    println!("cargo::rustc-env=DDRAWCOMPAT_INI={}", ini.display());

    let epoch = std::env::var("SOURCE_DATE_EPOCH").ok().or_else(|| {
        let out = Command::new("git").args(["log", "-1", "--format=%ct"]).current_dir(&root).output().ok()?;
        out.status.success().then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
    });
    println!("cargo::rustc-env=BUILD_EPOCH={}", epoch.filter(|s| !s.is_empty()).unwrap_or_else(|| "0".into()));

    let target = std::env::var("TARGET").unwrap();
    if !target.contains("windows") {
        return;
    }
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("builder.res.o");
    let arch = target.split('-').next().unwrap();
    // llvm-mingw has llvm-windres as <triple>-windres too
    let windres = format!("{arch}-w64-mingw32-windres");
    let status = Command::new(&windres)
        .current_dir(root.join("assets"))
        .args(["--input", "builder.rc", "--output-format=coff", "--output"])
        .arg(&out)
        .status();
    match status {
        Ok(s) if s.success() => println!("cargo::rustc-link-arg-bins={}", out.display()),
        _ => println!("cargo::warning={windres} failed: no manifest/version resource in the exe"),
    }
}
