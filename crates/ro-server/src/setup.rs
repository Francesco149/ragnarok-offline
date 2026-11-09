//! `ragnarok-offline setup`: make a client install talk to this server
//! without touching Ragexe.exe.
//!
//! The 2003-12-18 client reads `data\clientinfo.xml` only from data.grf (a
//! loose file is ignored - tested), and the CD ships Turtle Island / Juno as
//! un-merged `.gpf` patches that euRO's patcher would have merged. Setup does
//! exactly what that patcher did, plus our clientinfo.xml:
//!
//!   data.grf  ->  data.grf.original   (kept once, never overwritten)
//!   data.grf.original + *.gpf (date order) + clientinfo.xml  ->  data.grf
//!
//! The CLI also writes `Play Offline.bat` (`Ragexe.exe 1rag1`, skipping the
//! dead patcher); the Windows launcher does the same job itself.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use ro_grf::{Grf, Vfs};
use sha2::{Digest, Sha256};

/// Ragexe.exe from the 2003 euRO CD (docs/CLIENT.md).
pub const KNOWN_RAGEXE_SHA256: &str = "a25b7efae4200919e9d12bb12bc61fb447b7f34e89348c4086d5b7c415210eca";

pub const CLIENTINFO: &str = "data\\clientinfo.xml";

/// Where the client's `registrationweb` points: the local patch server's
/// HTTP port, which serves euRO's archived front page (patchsrv.rs). Nothing
/// listening there (CLI without a patch server) just means a local error
/// page, never a request to the internet.
pub const REGISTRATION_WEB_PORT: u16 = 6950;

/// The CD's own clientinfo.xml (in data.grf.original) with only the server,
/// its display name and the registration web page changed.
pub fn clientinfo_xml(address: &str, port: u16, name: &str) -> String {
    let web = format!("http://{address}:{REGISTRATION_WEB_PORT}/");
    format!(
        "<?xml version=\"1.0\" encoding=\"euc-kr\" ?>\r\n\r\n<clientinfo>\r\n\r\n\t<servicetype>germany</servicetype>\r\n\
         \t<servertype>primary</servertype>\r\n\t<hideaccountlist></hideaccountlist>\r\n\r\n\t<connection>\r\n\
         \t\t<display>{name}</display>\r\n\t\t<desc>None</desc>\r\n\t\t<address>{address}</address>\r\n\
         \t\t<port>{port}</port>\r\n\t\t<version>1</version>\r\n\t\t<registrationweb>{web}</registrationweb>\r\n\
         \t</connection>\r\n\r\n</clientinfo>\r\n"
    )
}

/// The layers a client dir provides, in patch order.
pub fn client_vfs(dir: &Path) -> Result<Vfs> {
    let mut vfs = Vfs::new();
    let merged_marker = dir.join("data.grf.original");
    let data = dir.join("data.grf");
    if merged_marker.exists() {
        // setup already merged everything into data.grf
        vfs.push(Grf::open(&data).with_context(|| format!("opening {}", data.display()))?);
        return Ok(vfs);
    }
    vfs.push(Grf::open(&data).with_context(|| format!("opening {}", data.display()))?);
    for gpf in gpf_files(dir)? {
        vfs.push(Grf::open(&gpf).with_context(|| format!("opening {}", gpf.display()))?);
    }
    Ok(vfs)
}

fn gpf_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir)?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e.eq_ignore_ascii_case("gpf")))
        .collect();
    v.sort(); // file names start with the patch date: 2003-04-30..., 2003-12-11...
    Ok(v)
}

/// sha256 of the client's Ragexe.exe, if there is one.
pub fn ragexe_sha256(dir: &Path) -> Option<String> {
    std::fs::read(dir.join("Ragexe.exe")).ok().map(|b| hex(&Sha256::digest(b)))
}

/// Does `dir`'s data.grf already carry our clientinfo.xml for `address:port`
/// (exactly: a changed template means the folder is prepared again)?
pub fn is_prepared(dir: &Path, address: &str, port: u16, name: &str) -> bool {
    if !dir.join("data.grf.original").exists() {
        return false;
    }
    let Ok(g) = Grf::open(dir.join("data.grf")) else { return false };
    g.read(CLIENTINFO).is_ok_and(|xml| xml == clientinfo_xml(address, port, name).as_bytes())
}

/// What [`prepare`] did.
pub struct Prepared {
    pub entries: usize,
    pub layers: Vec<String>,
}

/// Merge the patches and our clientinfo.xml into `dir`'s data.grf (see the
/// module docs). Idempotent: the CD's data.grf is kept once as
/// data.grf.original and every run rebuilds data.grf from it.
pub fn prepare(dir: &Path, address: &str, port: u16, name: &str, progress: &mut dyn FnMut(usize, usize)) -> Result<Prepared> {
    let data = dir.join("data.grf");
    let original = dir.join("data.grf.original");
    if !data.exists() && !original.exists() {
        bail!("{} not found - this is not a Ragnarok Online folder (it needs data.grf and Ragexe.exe)", data.display());
    }
    if !original.exists() {
        std::fs::rename(&data, &original).with_context(|| format!("renaming {} to data.grf.original", data.display()))?;
    }
    let mut vfs = Vfs::new();
    vfs.push(Grf::open(&original)?);
    let mut layers = Vec::new();
    for g in gpf_files(dir)? {
        layers.push(g.file_name().unwrap().to_string_lossy().into_owned());
        vfs.push(Grf::open(&g)?);
    }
    vfs.add_memory_file(CLIENTINFO, clientinfo_xml(address, port, name).into_bytes());
    let tmp = dir.join("data.grf.tmp");
    let entries = ro_grf::write_grf_0x200_with_progress(&vfs, &tmp, progress)
        .with_context(|| format!("writing {}", tmp.display()))?;
    drop(vfs); // close data.grf.original before touching data.grf
    if data.exists() {
        std::fs::remove_file(&data)?;
    }
    std::fs::rename(&tmp, &data)?;
    Ok(Prepared { entries, layers })
}

/// Undo [`prepare`]: put the CD's data.grf back.
pub fn unprepare(dir: &Path) -> Result<bool> {
    let original = dir.join("data.grf.original");
    if !original.exists() {
        return Ok(false);
    }
    let data = dir.join("data.grf");
    if data.exists() {
        std::fs::remove_file(&data)?;
    }
    let _ = std::fs::remove_file(dir.join("data.grf.tmp"));
    std::fs::rename(&original, &data)?;
    Ok(true)
}

/// The CLI `setup` command.
pub fn run(dir: &Path, address: &str, port: u16, name: &str) -> Result<()> {
    if !dir.join("data.grf").exists() && !dir.join("data.grf.original").exists() {
        bail!("{} not found - point --client at the folder with data.grf and Ragexe.exe", dir.join("data.grf").display());
    }
    match ragexe_sha256(dir) {
        Some(h) if h == KNOWN_RAGEXE_SHA256 => println!("Ragexe.exe: the 2003-12-18 euRO client (known hash)"),
        Some(h) => println!("warning: Ragexe.exe sha256 {h} is not the 2003-12-18 euRO client this server targets"),
        None => {}
    }
    if !dir.join("data.grf.original").exists() {
        println!("keeping the CD's data.grf as {}", dir.join("data.grf.original").display());
    }
    println!("writing merged data.grf (this copies ~550 MB) ...");
    let p = prepare(dir, address, port, name, &mut |_, _| {})?;
    for l in &p.layers {
        println!("patch layer: {l}");
    }
    println!("data.grf: {} files, clientinfo.xml -> {address}:{port}", p.entries);
    let bat = dir.join("Play Offline.bat");
    std::fs::write(&bat, "@echo off\r\ncd /d \"%~dp0\"\r\nstart \"\" Ragexe.exe 1rag1\r\n")?;
    println!("wrote {} (starts Ragexe.exe 1rag1, skipping the old patcher)", bat.display());
    println!("to undo: delete data.grf and rename data.grf.original back to data.grf");
    Ok(())
}

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
