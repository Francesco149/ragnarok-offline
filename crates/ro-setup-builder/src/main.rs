//! RagnarokOfflineSetupBuilder.exe: one click from nothing to the
//! all-in-one installer (docs/WINDOWS.md), without anyone redistributing a
//! modified copy of Gravity's disc.
//!
//! It downloads the preserved December 2003 euRO CD from the Internet
//! Archive (politely: src/ia.rs), checks every file on it, adds
//! ragnarok-offline to its installer exactly as tools/aio/make_aio.py does,
//! and writes the result as a disc image, a zip or a folder. The user first
//! accepts that the disc is Gravity's copyrighted software, preserved as lost
//! media. Runs on Windows XP to 11 (built by tools/xp/build.sh).
//!
//!   RagnarokOfflineSetupBuilder.exe        the window (Windows)
//!   ... --accept --format iso|zip|folder --out PATH [--disc ISO_OR_DIR]
//!       [--cache DIR] [--delete-download] [--launcher EXE] [--ddraw-dir DIR]
//!                                          the same without a window

#![cfg_attr(windows, windows_subsystem = "windows")]

mod aio;
#[cfg(windows)]
mod gui;
mod ia;
mod iso;
mod job;
mod zipw;

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::{Context, Result, bail};

pub const DISCLAIMER: &str = "\
The Ragnarok Online client on this disc is copyrighted software of Gravity Co., Ltd. \
This disc (euRO, December 2003 build, a CHIP Italia cover CD) is preserved as lost media \
on the Internet Archive. The service it was made for closed in 2010 and the disc is no \
longer sold or distributed anywhere.\r\n\r\n\
This program downloads that preserved copy from the Internet Archive onto your computer, \
adds the Ragnarok Offline single-player server to its installer, and leaves the result \
with you. Nothing of Gravity's is included in this program. The result is for your own \
personal use: do not share or sell it.\r\n\r\n\
Ragnarok Offline is an unofficial preservation project, not affiliated with or endorsed \
by Gravity, the Internet Archive, or anyone who ran euRO. It comes with no warranty \
(GNU GPL version 3 or later).";

/// Our files, as embedded at build time (build.rs) or given on the command line.
pub struct Ours {
    pub launcher: Vec<u8>,
    pub ddraw: Vec<u8>,
    pub ddraw_version: String,
    pub ddraw_ini: Vec<u8>,
    pub ddraw_license: Vec<u8>,
}

impl Ours {
    pub fn load(launcher: Option<PathBuf>, ddraw_dir: Option<PathBuf>) -> Result<Ours> {
        let launcher = match launcher {
            Some(p) => std::fs::read(&p).with_context(|| format!("reading {}", p.display()))?,
            None => embedded_launcher().map(<[u8]>::to_vec).context("this build has no RagnarokOffline.exe inside: pass --launcher")?,
        };
        let (ddraw, ddraw_license, ddraw_version) = match ddraw_dir {
            Some(d) => (
                std::fs::read(d.join("ddraw.dll"))?,
                std::fs::read(d.join("LICENSE.txt"))?,
                std::fs::read_to_string(d.join("VERSION"))?,
            ),
            None => {
                let (a, b, c) = embedded_ddraw().context("this build has no DDrawCompat inside: pass --ddraw-dir")?;
                (a.to_vec(), b.to_vec(), c.to_string())
            }
        };
        Ok(Ours {
            launcher,
            ddraw,
            // "v0.7.2" -> "0.7.2.0", the File table's version
            ddraw_version: format!("{}.0", ddraw_version.trim().trim_start_matches('v')),
            ddraw_ini: include_bytes!(env!("DDRAWCOMPAT_INI")).to_vec(),
            ddraw_license,
        })
    }

    pub fn payload(&self) -> aio::Payload<'_> {
        aio::Payload {
            launcher: &self.launcher,
            launcher_version: LAUNCHER_VERSION,
            ddraw: &self.ddraw,
            ddraw_version: &self.ddraw_version,
            ddraw_ini: &self.ddraw_ini,
            ddraw_license: &self.ddraw_license,
        }
    }
}

/// RagnarokOffline.exe's version in the MSI's File table (the workspace's).
const LAUNCHER_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), ".0");

#[cfg(embedded_launcher)]
fn embedded_launcher() -> Option<&'static [u8]> {
    Some(include_bytes!(env!("RO_SETUP_LAUNCHER_PATH")))
}
#[cfg(not(embedded_launcher))]
fn embedded_launcher() -> Option<&'static [u8]> {
    None
}

#[cfg(ddrawcompat)]
fn embedded_ddraw() -> Option<(&'static [u8], &'static [u8], &'static str)> {
    Some((
        include_bytes!(concat!(env!("DDRAWCOMPAT_DIR"), "/ddraw.dll")),
        include_bytes!(concat!(env!("DDRAWCOMPAT_DIR"), "/LICENSE.txt")),
        include_str!(concat!(env!("DDRAWCOMPAT_DIR"), "/VERSION")),
    ))
}
#[cfg(not(ddrawcompat))]
fn embedded_ddraw() -> Option<(&'static [u8], &'static [u8], &'static str)> {
    None
}

/// The date our files carry inside the installer: this build's commit date.
pub fn cab_time() -> time::PrimitiveDateTime {
    let secs: i64 = env!("BUILD_EPOCH").parse().unwrap_or(0);
    let t = time::OffsetDateTime::from_unix_timestamp(secs).unwrap_or(time::OffsetDateTime::UNIX_EPOCH);
    // cabinets hold DOS times, which start in 1980
    if t.year() < 1980 {
        return time::PrimitiveDateTime::new(time::Date::from_calendar_date(1980, time::Month::January, 1).unwrap(), time::Time::MIDNIGHT);
    }
    time::PrimitiveDateTime::new(t.date(), t.time())
}

/// Where downloads are kept: next to other per-user program data.
pub fn default_cache() -> PathBuf {
    #[cfg(windows)]
    if let Some(d) = gui::local_app_data() {
        return d.join("Ragnarok Offline").join("download");
    }
    std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))
        .unwrap_or_else(std::env::temp_dir)
        .join("ragnarok-offline")
}

struct Console {
    cancelled: std::sync::atomic::AtomicBool,
    last: AtomicU64,
}

impl job::Report for Console {
    fn status(&self, text: &str) {
        eprintln!("{text}");
    }
    fn progress(&self, done: u64, total: u64) {
        let pct = done * 100 / total.max(1);
        if self.last.swap(pct, Ordering::Relaxed) != pct && pct % 10 == 0 {
            eprintln!("  {pct}% ({} / {} MB)", done >> 20, total >> 20);
        }
    }
    fn cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}

fn cli(args: Vec<String>) -> Result<()> {
    let mut accept = false;
    let mut format = None;
    let mut out = None;
    let mut disc = None;
    let mut cache = None;
    let mut keep = true;
    let mut launcher = None;
    let mut ddraw_dir = None;
    let mut it = args.into_iter();
    while let Some(a) = it.next() {
        let mut val = || it.next().with_context(|| format!("{a} needs a value"));
        match a.as_str() {
            "--accept" => accept = true,
            "--format" => {
                format = Some(match val()?.as_str() {
                    "iso" => job::Format::Iso,
                    "zip" => job::Format::Zip,
                    "folder" => job::Format::Folder,
                    f => bail!("unknown format {f}: iso, zip or folder"),
                })
            }
            "--out" => out = Some(PathBuf::from(val()?)),
            "--disc" => disc = Some(PathBuf::from(val()?)),
            "--cache" => cache = Some(PathBuf::from(val()?)),
            "--delete-download" => keep = false,
            "--launcher" => launcher = Some(PathBuf::from(val()?)),
            "--ddraw-dir" => ddraw_dir = Some(PathBuf::from(val()?)),
            "-h" | "--help" => {
                println!("{}", include_str!("main.rs").lines().skip_while(|l| !l.contains("the window")).take_while(|l| l.starts_with("//!")).map(|l| l.trim_start_matches("//!")).collect::<Vec<_>>().join("\n"));
                return Ok(());
            }
            _ => bail!("unknown argument {a} (--help)"),
        }
    }
    if !accept {
        eprintln!("{}\n\nPass --accept to agree to the above.", DISCLAIMER.replace("\r\n", "\n"));
        std::process::exit(2);
    }
    let format = format.context("--format iso|zip|folder is required")?;
    let out = out.unwrap_or_else(|| PathBuf::from(format.default_name()));
    let ours = Ours::load(launcher, ddraw_dir)?;
    let source = match disc {
        Some(d) => job::Source::Local(d),
        None => job::Source::Download { cache: cache.unwrap_or_else(default_cache), keep },
    };
    let opts = job::Options { source, format, out };
    let con = Console { cancelled: Default::default(), last: AtomicU64::new(u64::MAX) };
    let done = job::run(&opts, &ours.payload(), cab_time(), &con)?;
    println!("{}", done.out.display());
    eprintln!("PackageCode {}", done.package_code);
    if let Some(c) = done.cached {
        eprintln!("the downloaded disc image stays in {} for next time", c.display());
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    #[cfg(windows)]
    if args.is_empty() {
        gui::run();
        return;
    }
    #[cfg(windows)]
    gui::attach_console();
    if let Err(e) = cli(args) {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}
