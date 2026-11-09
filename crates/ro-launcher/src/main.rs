//! RagnarokOffline.exe - drop it into a Ragnarok Online folder (the one with
//! Ragexe.exe) and double-click it.
//!
//! First run, it prepares the folder once (UAC prompt only if the folder is
//! read-only, e.g. under Program Files):
//!   - merges the CD's patches + our clientinfo.xml into data.grf
//!     (data.grf.original keeps the CD's file; see ro_server::setup)
//!   - Windows (not Wine): puts DDrawCompat's ddraw.dll + DDrawCompat.ini next
//!     to Ragexe.exe (DirectX 7 fixes, XP-style text; docs/WINDOWS.md)
//!   - writes the client's settings if its Setup.exe never ran
//!
//! Every run: unpacks the server's scripts/database into a data folder
//! (`<game>\offline` if the game folder is writable, else
//! `%LOCALAPPDATA%\Ragnarok Offline`), starts the server in-process, then
//! starts the game the way the CD did: through its patcher, `Ragnarok.exe`,
//! which fetches euRO's notice and patch list - now from our local patch
//! server (an archived snapshot, ro_server::patchsrv) - and offers
//! "Start Game". Saves + stops the server when the client exits.
//!
//!   RagnarokOffline.exe               play
//!   RagnarokOffline.exe --direct      skip the patcher, start Ragexe.exe 1rag1
//!   RagnarokOffline.exe --settings    the client's own Setup.exe
//!   RagnarokOffline.exe --prepare     prepare only (elevated relaunch, installers)
//!   RagnarokOffline.exe --unprepare   put the CD's data.grf back (uninstallers)
//!   ... --quiet                       no dialogs (exit code 0 = ok)

#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(windows)]
mod win;
#[cfg(windows)]
use win as plat;
#[cfg(not(windows))]
mod other;
#[cfg(not(windows))]
use other as plat;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Context, Result, bail};
use include_dir::{Dir, include_dir};
use ro_server::config::{Config, TEMPLATE};
use ro_server::{server, setup};
use sha2::{Digest, Sha256};
use tracing::{info, warn};

static SCRIPTS: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../scripts");
static DB: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../data/db");
#[cfg(ddrawcompat)]
static DDRAWCOMPAT: Option<&[u8]> = Some(include_bytes!(env!("DDRAWCOMPAT_DLL")));
#[cfg(not(ddrawcompat))]
static DDRAWCOMPAT: Option<&[u8]> = None;
const DDRAWCOMPAT_INI: &str = include_str!("../assets/DDrawCompat.ini");

/// What euRO's wpatch.euro-ro.net/patch/ served, as the Wayback Machine has it
/// (fetched by tools/fetch-third-party.sh; docs/WINDOWS.md "The patcher").
/// (file name, contents); patch2.txt is the archived list, served through
/// patchsrv::patch_list_for_level.
#[cfg(euro_patch)]
static EURO_PATCH: Option<[(&str, &[u8]); 4]> = Some([
    ("patch_allow.txt", include_bytes!(env!("EURO_PATCH_ALLOW"))),
    ("notice_patch.txt", include_bytes!(env!("EURO_PATCH_NOTICE"))),
    ("notice_close.txt", include_bytes!(env!("EURO_PATCH_CLOSE"))),
    ("patch2.txt", include_bytes!(env!("EURO_PATCH_LIST"))),
]);
#[cfg(not(euro_patch))]
static EURO_PATCH: Option<[(&str, &[u8]); 4]> = None;
/// euRO's front page (2004-02-21), served at / for the client's registrationweb.
#[cfg(euro_patch)]
static EURO_WEB_INDEX: Option<&[u8]> = Some(include_bytes!(env!("EURO_WEB_INDEX")));
#[cfg(not(euro_patch))]
static EURO_WEB_INDEX: Option<&[u8]> = None;

/// The CD's patcher, Ragnarok.exe 2003-12-12 (installed-files.sha256).
const KNOWN_PATCHER_SHA256: &str = "91eafb955fcbdf61a18bc1acfbd1e5aa2e127c25127072ddfe3f6e7b191c63c2";
/// Its host name slots in .data (64-byte fields; fixed image base 0x400000,
/// no relocations), read by the HTTP ("http://%s%s/%s") and FTP
/// (InternetConnectA, port 21) code.
const PATCHER_WPATCH: (usize, &[u8]) = (0x41d210, b"wpatch.euro-ro.net\0");
const PATCHER_FPATCH: (usize, &[u8]) = (0x41d250, b"fpatch.euro-ro.net\0");
/// The CD's patch.inf: the last patch number the disc already contains.
const CD_PATCH_LEVEL: u32 = 303;
const PATCH_LIST_HEADER: &str = "// ragnarok-offline: euRO's list as archived on 2004-02-21 (web.archive.org); \
entries after this CD's level (303) are commented out, their files were not archived";

const ADDRESS: &str = "127.0.0.1";
const LOGIN_PORT: u16 = 6900;
const VERSION: &str = env!("CARGO_PKG_VERSION");

static QUIET: AtomicBool = AtomicBool::new(false);

pub enum RegValue {
    Dword(u32),
    Str(&'static str),
    Bin(&'static [u8]),
}

fn error(msg: &str) {
    tracing::error!("{msg}");
    if !QUIET.load(Ordering::Relaxed) {
        plat::error(msg);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    QUIET.store(args.iter().any(|a| a == "--quiet"), Ordering::Relaxed);
    let code = match run(&args) {
        Ok(code) => code,
        Err(e) => {
            error(&format!("{e:#}"));
            1
        }
    };
    std::process::exit(code);
}

fn game_dir() -> Result<PathBuf> {
    let exe = std::env::current_exe().context("locating RagnarokOffline.exe")?;
    let dir = exe.parent().context("exe has no folder")?.to_path_buf();
    if !dir.join("Ragexe.exe").exists() {
        bail!(
            "Ragexe.exe is not in {}.\n\nPut RagnarokOffline.exe into your Ragnarok Online folder \
             (the one with Ragexe.exe and data.grf) and start it from there.",
            dir.display()
        );
    }
    Ok(dir)
}

fn run(args: &[String]) -> Result<i32> {
    let dir = game_dir()?;
    let has = |a: &str| args.iter().any(|x| x == a);
    if has("--prepare") {
        log_to(&std::env::temp_dir().join("RagnarokOffline-prepare.log"));
        prepare(&dir)?;
        return Ok(0);
    }
    if has("--unprepare") {
        log_to(&std::env::temp_dir().join("RagnarokOffline-prepare.log"));
        let restored = setup::unprepare(&dir)?;
        info!("unprepare {}: restored data.grf = {restored}", dir.display());
        return Ok(0);
    }
    if has("--settings") {
        std::process::Command::new(dir.join("Setup.exe")).current_dir(&dir).status().context("starting Setup.exe")?;
        return Ok(0);
    }
    play(&dir, has("--direct"))
}

fn play(dir: &Path, direct: bool) -> Result<i32> {
    match setup::ragexe_sha256(dir) {
        Some(h) if h == setup::KNOWN_RAGEXE_SHA256 => {}
        _ => {
            if !QUIET.load(Ordering::Relaxed)
                && !plat::ask(
                    "This Ragexe.exe is not the 2003-12-18 euRO client this server was made for, \
                     so it will probably not work.\n\nTry anyway?",
                )
            {
                return Ok(1);
            }
        }
    }
    let writable = dir_writable(dir);
    let data = if writable { dir.join("offline") } else { plat::local_appdata().join("Ragnarok Offline") };
    unpack(&data)?;
    log_to(&data.join("logs").join("launcher.log"));
    info!("RagnarokOffline {VERSION}: game {} data {}", dir.display(), data.display());
    if needs_prepare(dir) {
        if writable {
            prepare(dir)?;
        } else {
            info!("{} is read-only; asking for administrator rights to prepare it", dir.display());
            let code = plat::run_elevated("--prepare")?;
            if code != 0 || needs_prepare(dir) {
                bail!("preparing {} failed (see %TEMP%\\RagnarokOffline-prepare.log)", dir.display());
            }
        }
    }
    if !plat::client_settings_exist() {
        let wher = plat::write_client_settings(&default_settings())?;
        info!("wrote first-run client settings ({wher})");
    }

    let cfg_path = data.join("ragnarok-offline.toml");
    let mut cfg = Config::load(&cfg_path)?;
    cfg.client_dir = dir.to_path_buf();
    let use_patcher = cfg!(windows) && EURO_PATCH.is_some() && !direct && patcher_ok(dir);
    if use_patcher {
        cfg.patch_server.enabled = true;
        cfg.patch_server.dir = data.join("patchsrv");
    }
    let http_port = cfg.patch_server.http_port;
    let opts = server::Options { ctrl_c: false, log_dir: data.join("logs") };
    let started = plat::starting("Starting Ragnarok Offline...", move || server::start(cfg, opts));
    let handle = match started {
        Ok(h) => Some(h),
        Err(e) if addr_in_use(&e) => {
            warn!("{e:#}; assuming a server is already running and starting only the client");
            None
        }
        Err(e) => return Err(e),
    };
    let started = std::time::Instant::now();
    let via_patcher = use_patcher && handle.as_ref().is_some_and(|h| h.patch_server);
    let status = if via_patcher {
        match run_patcher(dir, http_port) {
            Ok(()) => Ok(()),
            Err(e) => {
                warn!("patcher: {e:#}; starting the client directly");
                run_client(dir)
            }
        }
    } else {
        run_client(dir)
    };
    info!("game exited after {:?}: {status:?}", started.elapsed());
    if let Some(h) = handle {
        h.shutdown();
    }
    status?;
    Ok(0)
}

fn run_client(dir: &Path) -> Result<()> {
    let mut c = spawn_client(dir)?;
    plat::allow_foreground(c.id());
    c.wait().context("waiting for Ragexe.exe")?;
    Ok(())
}

/// Is the CD's patcher there, unmodified?
fn patcher_ok(dir: &Path) -> bool {
    std::fs::read(dir.join("Ragnarok.exe")).is_ok_and(|b| setup::hex(&Sha256::digest(b)) == KNOWN_PATCHER_SHA256)
}

/// The CD's patcher, pointed at our patch server: its two host names are
/// replaced in memory before it runs (the exe on disk stays as it is), so it
/// fetches euRO's notice and list from 127.0.0.1, finds nothing missing and
/// starts Ragexe.exe 1rag1 when you click "Start Game". Returns when the
/// patcher and the game it started have both exited.
fn run_patcher(dir: &Path, http_port: u16) -> Result<()> {
    let web = format!("127.0.0.1:{http_port}").into_bytes(); // the port rides in the URL
    let ftp = b"127.0.0.1".to_vec(); // InternetConnectA always uses port 21
    info!("starting the CD's patcher (wpatch -> {}, fpatch -> 127.0.0.1:21)", String::from_utf8_lossy(&web));
    plat::run_patched(&dir.join("Ragnarok.exe"), dir, &[(PATCHER_WPATCH.0, PATCHER_WPATCH.1, web), (PATCHER_FPATCH.0, PATCHER_FPATCH.1, ftp)])
}

fn spawn_client(dir: &Path) -> Result<std::process::Child> {
    #[cfg(windows)]
    let mut cmd = std::process::Command::new(dir.join("Ragexe.exe"));
    #[cfg(not(windows))]
    let mut cmd = {
        let mut c = std::process::Command::new("wine");
        c.arg("Ragexe.exe");
        c
    };
    cmd.arg("1rag1").current_dir(dir);
    cmd.spawn().with_context(|| format!("starting {}", dir.join("Ragexe.exe").display()))
}

fn addr_in_use(e: &anyhow::Error) -> bool {
    e.chain().any(|c| c.downcast_ref::<std::io::Error>().is_some_and(|io| io.kind() == std::io::ErrorKind::AddrInUse))
}

/// Can we create files in `dir`? (Our manifest turns off UAC file
/// virtualization, so this is the real answer for Program Files.)
fn dir_writable(dir: &Path) -> bool {
    let probe = dir.join(".ragnarok-offline-write-test");
    let ok = std::fs::write(&probe, b"").is_ok();
    let _ = std::fs::remove_file(&probe);
    ok
}

fn wants_ddrawcompat() -> bool {
    cfg!(windows) && DDRAWCOMPAT.is_some() && !plat::is_wine()
}

fn needs_prepare(dir: &Path) -> bool {
    !setup::is_prepared(dir, ADDRESS, LOGIN_PORT, "Offline")
        || (wants_ddrawcompat() && (!dir.join("ddraw.dll").exists() || !dir.join("DDrawCompat.ini").exists()))
}

/// Everything that needs write access to the game folder.
fn prepare(dir: &Path) -> Result<()> {
    info!("preparing {}", dir.display());
    if !setup::is_prepared(dir, ADDRESS, LOGIN_PORT, "Offline") {
        let work = |report: &(dyn Fn(usize, usize) + Sync)| {
            setup::prepare(dir, ADDRESS, LOGIN_PORT, "Offline", &mut |d, t| report(d, t))
        };
        let p = if QUIET.load(Ordering::Relaxed) {
            work(&|_, _| {})
        } else {
            plat::progress("Preparing your Ragnarok Online folder for offline play (once)...", work)
        }?;
        info!("data.grf: {} files, patch layers {:?}", p.entries, p.layers);
    }
    if wants_ddrawcompat() {
        let dll = dir.join("ddraw.dll");
        match std::fs::read(&dll) {
            Ok(b) if b == DDRAWCOMPAT.unwrap() => {}
            Ok(b) => warn!("{} exists and is not our DDrawCompat (sha256 {}); leaving it alone", dll.display(), setup::hex(&Sha256::digest(b))),
            Err(_) => {
                std::fs::write(&dll, DDRAWCOMPAT.unwrap()).with_context(|| format!("writing {}", dll.display()))?;
                info!("installed DDrawCompat as {}", dll.display());
            }
        }
        let ini = dir.join("DDrawCompat.ini");
        if !ini.exists() {
            std::fs::write(&ini, DDRAWCOMPAT_INI.replace('\n', "\r\n"))?;
        }
    }
    // when we are elevated, HKLM is ours to fill (like Setup.exe run as admin on XP)
    if !plat::client_settings_exist()
        && let Ok(w) = plat::write_client_settings(&default_settings())
    {
        info!("wrote first-run client settings ({w})");
    }
    Ok(())
}

/// What the client's own Setup.exe writes when you click Ok (observed on
/// Windows 11 with DDrawCompat), with 800x600 x16 picked. Fullscreen is
/// emulated by DDrawCompat (no real mode change); under Wine stay windowed.
/// MODECNT = index of 800x600 in Setup.exe's list given our
/// SupportedResolutions (640x480, 800x600, ...).
fn default_settings() -> Vec<(&'static str, RegValue)> {
    use RegValue::*;
    const HAL: &[u8] = &[0xe0, 0x3d, 0xe6, 0x84, 0xaa, 0x46, 0xcf, 0x11, 0x81, 0x6f, 0x00, 0x00, 0xc0, 0x20, 0x15, 0x6e];
    let fullscreen = wants_ddrawcompat() as u32;
    vec![
        ("ISFULLSCREENMODE", Dword(fullscreen)),
        ("WIDTH", Dword(800)),
        ("HEIGHT", Dword(600)),
        ("BITPERPIXEL", Dword(16)),
        ("DEVICECNT", Dword(0)),
        ("MODECNT", Dword(1)),
        ("ISVOODOO", Dword(0)),
        ("ISLIGHTMAP", Dword(1)),
        ("DEVICENAME", Str("Direct3D HAL")),
        ("GUIDDEVICE", Bin(HAL)),
        ("GUIDDRIVER", Bin(&[0; 16])),
        ("SOUNDMODE", Dword(1)),
        ("SPEAKERTYPE", Dword(0)),
        ("DIGITALRATETYPE", Dword(0)),
        ("DIGITALBITSTYPE", Dword(0)),
        ("PROVIDERNAME", Str("No Provider")),
        ("SPRITEMODE", Dword(2)),
        ("TEXTUREMODE", Dword(2)),
        ("FOG", Dword(1)),
        ("NUMSAMPLETYPE", Dword(2)),
    ]
}

/// Unpack the server's scripts and database into `data`. Scripts are yours
/// to edit: only missing files are written. The database is generated data
/// and is refreshed whenever the launcher version changes.
fn unpack(data: &Path) -> Result<()> {
    std::fs::create_dir_all(data).with_context(|| format!("creating {}", data.display()))?;
    let stamp = data.join("data").join("db").join(".version");
    let fresh_db = std::fs::read_to_string(&stamp).map(|v| v.trim() != VERSION).unwrap_or(true);
    extract(&SCRIPTS, &data.join("scripts"), false)?;
    extract(&DB, &data.join("data").join("db"), fresh_db)?;
    if let Some(index) = EURO_WEB_INDEX {
        let p = data.join("patchsrv").join("index.html");
        if !p.exists() {
            std::fs::create_dir_all(data.join("patchsrv"))?;
            std::fs::write(&p, index)?;
        }
    }
    if let Some(files) = &EURO_PATCH {
        // the patch server's files; yours to edit (e.g. your own notice): written once
        let d = data.join("patchsrv").join("patch");
        std::fs::create_dir_all(&d)?;
        for (name, bytes) in files {
            let p = d.join(name);
            if p.exists() {
                continue;
            }
            if *name == "patch2.txt" {
                let list = ro_server::patchsrv::patch_list_for_level(&String::from_utf8_lossy(bytes), CD_PATCH_LEVEL, PATCH_LIST_HEADER);
                std::fs::write(&p, list)?;
            } else {
                std::fs::write(&p, bytes)?;
            }
        }
    }
    std::fs::write(&stamp, VERSION)?;
    let cfg = data.join("ragnarok-offline.toml");
    if !cfg.exists() {
        let t = TEMPLATE.replace(
            "client_dir = \"client\"",
            "client_dir = \"..\"   # ignored by RagnarokOffline.exe: it always uses its own folder",
        );
        std::fs::write(&cfg, t.replace('\n', "\r\n"))?;
    }
    Ok(())
}

fn extract(dir: &Dir, to: &Path, overwrite: bool) -> Result<()> {
    for f in dir.files() {
        let p = to.join(f.path()); // include_dir paths are relative to the embedded root
        if overwrite || !p.exists() {
            if let Some(parent) = p.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&p, f.contents()).with_context(|| format!("writing {}", p.display()))?;
        }
    }
    for d in dir.dirs() {
        extract(d, to, overwrite)?;
    }
    Ok(())
}

fn log_to(path: &Path) {
    let Some(parent) = path.parent() else { return };
    let _ = std::fs::create_dir_all(parent);
    let Ok(f) = std::fs::OpenOptions::new().create(true).append(true).open(path) else { return };
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with_ansi(false)
        .with_target(false)
        .with_writer(std::sync::Mutex::new(f))
        .try_init();
}
