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
//! and writes `Play Offline.bat` (`Ragexe.exe 1rag1`, skipping the dead patcher).

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use ro_grf::{Grf, Vfs};
use sha2::{Digest, Sha256};

/// Ragexe.exe from the 2003 euRO CD (docs/CLIENT.md).
pub const KNOWN_RAGEXE_SHA256: &str = "a25b7efae4200919e9d12bb12bc61fb447b7f34e89348c4086d5b7c415210eca";

pub fn clientinfo_xml(address: &str, port: u16, name: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="euc-kr" ?>
<clientinfo>
	<servicetype>germany</servicetype>
	<servertype>primary</servertype>
	<connection>
		<display>{name}</display>
		<desc>ragnarok-offline</desc>
		<address>{address}</address>
		<port>{port}</port>
		<version>1</version>
	</connection>
</clientinfo>
"#
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

pub fn run(dir: &Path, address: &str, port: u16, name: &str) -> Result<()> {
    let data = dir.join("data.grf");
    let original = dir.join("data.grf.original");
    if !data.exists() {
        bail!("{} not found - point --client at the folder with data.grf and Ragexe.exe", data.display());
    }
    let exe = dir.join("Ragexe.exe");
    if exe.exists() {
        let h = hex(&Sha256::digest(std::fs::read(&exe)?));
        if h == KNOWN_RAGEXE_SHA256 {
            println!("Ragexe.exe: the 2003-12-18 euRO client (known hash)");
        } else {
            println!("warning: Ragexe.exe sha256 {h} is not the 2003-12-18 euRO client this server targets");
        }
    }
    if !original.exists() {
        println!("keeping the CD's data.grf as {}", original.display());
        std::fs::rename(&data, &original)?;
    }
    let mut vfs = Vfs::new();
    vfs.push(Grf::open(&original)?);
    let gpfs = gpf_files(dir)?;
    for g in &gpfs {
        println!("patch layer: {}", g.file_name().unwrap().to_string_lossy());
        vfs.push(Grf::open(g)?);
    }
    let xml = clientinfo_xml(address, port, name);
    vfs.add_memory_file("data\\clientinfo.xml", xml.into_bytes());
    let tmp = dir.join("data.grf.tmp");
    println!("writing merged data.grf (this copies ~550 MB) ...");
    let n = ro_grf::write_grf_0x200(&vfs, &tmp)?;
    if data.exists() {
        std::fs::remove_file(&data)?;
    }
    std::fs::rename(&tmp, &data)?;
    println!("data.grf: {n} files, clientinfo.xml -> {address}:{port}");
    let bat = dir.join("Play Offline.bat");
    std::fs::write(&bat, "@echo off\r\ncd /d \"%~dp0\"\r\nstart \"\" Ragexe.exe 1rag1\r\n")?;
    println!("wrote {} (starts Ragexe.exe 1rag1, skipping the old patcher)", bat.display());
    println!("to undo: delete data.grf and rename data.grf.original back to data.grf");
    Ok(())
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
