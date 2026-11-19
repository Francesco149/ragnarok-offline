//! - embeds third_party/ddrawcompat/ddraw.dll when tools/fetch-third-party.sh
//!   has fetched it (cfg `ddrawcompat`); without it the launcher still works
//!   but cannot fix DirectX 7 on modern Windows.
//! - Windows targets: compiles assets/launcher.rc (manifest + version info)
//!   with mingw's windres and links it into the exe, and builds the
//!   Ragnarok.exe stub (assets/stub, cfg `ragnarok_stub`) to embed.

use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(ddrawcompat)");
    println!("cargo::rustc-check-cfg=cfg(euro_patch)");
    println!("cargo::rustc-check-cfg=cfg(ragnarok_stub)");
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
    build_stub(&root);
}

/// assets/stub: the Ragnarok.exe stub, always 32-bit (it runs on XP too), no
/// C runtime. gcc where there is one, else llvm-mingw's clang (the XP build).
fn build_stub(root: &std::path::Path) {
    let dir = root.join("assets").join("stub");
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let (exe, res) = (out.join("ragnarok-stub.exe"), out.join("ragnarok-stub.res.o"));
    let ok = |c: &mut Command| c.status().is_ok_and(|s| s.success());
    if !ok(Command::new("i686-w64-mingw32-windres")
        .current_dir(&dir)
        .args(["--input", "stub.rc", "--output-format=coff", "--output"])
        .arg(&res))
    {
        println!("cargo::warning=i686-w64-mingw32-windres failed: no Ragnarok.exe stub (the CD's patcher stays as it is)");
        return;
    }
    for cc in ["i686-w64-mingw32-gcc", "i686-w64-mingw32-clang"] {
        let built = ok(Command::new(cc)
            .current_dir(&dir)
            .args(["-Os", "-s", "-Wall", "-ffreestanding", "-fno-stack-protector", "-fno-asynchronous-unwind-tables"])
            .args(["-nostdlib", "-nostartfiles", "-mwindows", "-Wl,--subsystem,windows:5.01"])
            .args(["-Wl,--major-os-version,5", "-Wl,--minor-os-version,1", "-o"])
            .arg(&exe)
            .arg("stub.c")
            .arg(&res)
            .args(["-lkernel32", "-luser32"]));
        if built {
            println!("cargo::rustc-cfg=ragnarok_stub");
            println!("cargo::rustc-env=RAGNAROK_STUB={}", exe.display());
            return;
        }
    }
    println!("cargo::warning=no i686 mingw C compiler: no Ragnarok.exe stub (the CD's patcher stays as it is)");
}
