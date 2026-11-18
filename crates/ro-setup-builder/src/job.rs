//! One build: get the disc (download, a cached copy, or the user's own image
//! or drive), check every file on it, patch the installer, write the result
//! as a folder, a zip or a disc image.

use std::fs::{self, File};
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};

use crate::aio::{self, Payload};
use crate::{ia, iso, zipw};

/// The 9 files of the disc (iso-files.sha256, docs/PRESERVATION.md), in
/// the order they are written.
pub const CD_FILES: [(&str, &str); 9] = [
    ("0x0409.ini", "ddc0262ecaf411329b7d6b0510696e934f7f15887a9b81084ef3b1d07c7f3824"),
    ("Autorun.inf", "01a9bd59df369213b82def73380138549389a15e3c54ad4a55db6e7cd495d161"),
    ("Data1.cab", "44eff094285f6c6171862bc1e1c4c2ccd25518adbd54e4d8c34d43350649104b"),
    ("instmsia.exe", "267304efcc831e35927c1f25d610d36fb64121d108a6f4ff0168c53df01e2b16"),
    ("instmsiw.exe", "b93fdcd1136faa9a8cb73a329b2f1f5f430a150ddcec35de916e3a1539f09351"),
    ("ragnarok.ico", "ad519bc8f4f83bcf62afed13def9b2d8ab68c1d91056eab941d658114694982b"),
    ("Ragnarok Online.msi", aio::ORIGINAL_MSI_SHA256),
    ("setup.exe", "95e7aa7b4dd0ba0e086c266ae8ce2318f6093597a723fa024f612774a2e1e08d"),
    ("Setup.ini", "3a2552732cd1fbc692218e90422f4fc34dd5b3ffc33ac4bb231843a694b54499"),
];

/// The name of the zip's top folder and the default output name.
pub const OUT_NAME: &str = "RagnarokOffline-CD";

pub trait Report: Sync {
    fn status(&self, text: &str);
    fn progress(&self, done: u64, total: u64);
    fn cancelled(&self) -> bool;
}

#[derive(Debug)]
pub struct Cancelled;

impl std::fmt::Display for Cancelled {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("cancelled")
    }
}

impl std::error::Error for Cancelled {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Iso,
    Zip,
    Folder,
}

impl Format {
    pub fn default_name(self) -> String {
        match self {
            Format::Iso => format!("{OUT_NAME}.iso"),
            Format::Zip => format!("{OUT_NAME}.zip"),
            Format::Folder => OUT_NAME.to_string(),
        }
    }
}

pub enum Source {
    /// from the Internet Archive into this cache folder (or the copy already there)
    Download { cache: PathBuf, keep: bool },
    /// the user's own disc image, or a folder with the disc's files (the CD drive)
    Local(PathBuf),
}

pub struct Options {
    pub source: Source,
    pub format: Format,
    pub out: PathBuf,
}

pub struct Done {
    pub out: PathBuf,
    pub package_code: String,
    /// the downloaded image, when it was kept
    pub cached: Option<PathBuf>,
}

/// Where each file of the disc is read from.
enum Disc {
    Image { path: PathBuf, files: Vec<iso::IsoFile> },
    Dir(PathBuf),
}

struct Entry {
    name: &'static str,
    sha256: &'static str,
    size: u64,
    date: [u8; 7],
    /// replaced content (the patched MSI and Setup.ini); otherwise read from the disc
    data: Option<Vec<u8>>,
}

fn iso_date(t: std::time::SystemTime) -> [u8; 7] {
    let t = time::OffsetDateTime::from(t);
    [(t.year() - 1900).clamp(0, 255) as u8, t.month() as u8, t.day(), t.hour(), t.minute(), t.second(), 0]
}

impl Disc {
    fn open(path: &Path) -> Result<Disc> {
        if path.is_dir() {
            return Ok(Disc::Dir(path.to_path_buf()));
        }
        let mut f = File::open(path).with_context(|| format!("opening {}", path.display()))?;
        let files = iso::read_root(&mut f).with_context(|| format!("reading {}", path.display()))?;
        Ok(Disc::Image { path: path.to_path_buf(), files })
    }

    /// (reader, size, date) of a file of the disc, its name matched without case
    fn file(&self, name: &str) -> Result<(Box<dyn Read>, u64, [u8; 7])> {
        match self {
            Disc::Image { path, files } => {
                let e = files
                    .iter()
                    .find(|e| e.name.eq_ignore_ascii_case(name))
                    .with_context(|| format!("{} has no {name}: is it the 2003 euRO CD?", path.display()))?;
                let mut f = File::open(path)?;
                f.seek(SeekFrom::Start(e.offset))?;
                Ok((Box::new(f.take(e.size)), e.size, e.date))
            }
            Disc::Dir(dir) => {
                let found = fs::read_dir(dir)
                    .with_context(|| format!("reading {}", dir.display()))?
                    .filter_map(|e| e.ok())
                    .find(|e| e.file_name().to_string_lossy().eq_ignore_ascii_case(name))
                    .with_context(|| format!("{} has no {name}: is it the 2003 euRO CD?", dir.display()))?;
                let m = found.metadata()?;
                let date = m.modified().map(iso_date).unwrap_or([103, 12, 19, 0, 0, 0, 0]);
                Ok((Box::new(File::open(found.path())?), m.len(), date))
            }
        }
    }

    fn read_all(&self, name: &str, sha256: &str) -> Result<(Vec<u8>, [u8; 7])> {
        let (mut r, _, date) = self.file(name)?;
        let mut v = Vec::new();
        r.read_to_end(&mut v)?;
        if aio::sha256_hex(&v) != sha256 {
            bail!("{name} on this disc is not the one of the December 2003 euRO CD (sha256 mismatch)");
        }
        Ok((v, date))
    }
}

/// Copies with progress and cancellation, hashing what it copies.
struct Copier<'a> {
    r: &'a dyn Report,
    done: u64,
    total: u64,
    buf: Vec<u8>,
}

impl Copier<'_> {
    fn copy(&mut self, from: &mut dyn Read, to: &mut dyn Write) -> Result<String> {
        let mut h = Sha256::new();
        loop {
            if self.r.cancelled() {
                return Err(Cancelled.into());
            }
            let n = from.read(&mut self.buf)?;
            if n == 0 {
                break;
            }
            h.update(&self.buf[..n]);
            to.write_all(&self.buf[..n])?;
            self.done += n as u64;
            self.r.progress(self.done, self.total);
        }
        Ok(aio::hex(&h.finalize()))
    }

    fn entry(&mut self, disc: &Disc, e: &Entry, to: &mut dyn Write) -> Result<()> {
        match &e.data {
            Some(d) => {
                to.write_all(d)?;
                self.done += d.len() as u64;
                self.r.progress(self.done, self.total);
            }
            None => {
                let (mut from, _, _) = disc.file(e.name)?;
                if self.copy(&mut from, to)? != e.sha256 {
                    bail!("{} on this disc is not the one of the December 2003 euRO CD (sha256 mismatch)", e.name);
                }
            }
        }
        Ok(())
    }
}

fn now_iso_label() -> String {
    let t = time::OffsetDateTime::now_utc();
    format!("{:04}{:02}{:02}{:02}{:02}{:02}00", t.year(), t.month() as u8, t.day(), t.hour(), t.minute(), t.second())
}

/// A file path next to `out` for writing before the final rename.
fn part_path(out: &Path) -> PathBuf {
    let mut s = out.as_os_str().to_owned();
    s.push(".part");
    PathBuf::from(s)
}

pub fn run(o: &Options, payload: &Payload, cab_time: time::PrimitiveDateTime, r: &dyn Report) -> Result<Done> {
    // 1. the disc
    let disc_path = match &o.source {
        Source::Download { cache, .. } => ia::fetch(cache, r)?.0,
        Source::Local(p) => p.clone(),
    };
    let disc = Disc::open(&disc_path)?;

    // 2. the installer, patched
    r.status("Adding Ragnarok Offline to the installer");
    let (msi, _) = disc.read_all(aio::MSI_NAME, aio::ORIGINAL_MSI_SHA256)?;
    let (ini, _) = disc.read_all("Setup.ini", CD_FILES[8].1)?;
    let (icon, _) = disc.read_all("ragnarok.ico", CD_FILES[5].1)?;
    let patched = aio::patch(&msi, &ini, &icon, payload, cab_time)?;
    let now = iso_date(std::time::SystemTime::now());

    let mut entries = Vec::new();
    for (name, sha) in CD_FILES {
        let (data, size, date) = match name {
            aio::MSI_NAME => (Some(patched.msi.clone()), patched.msi.len() as u64, now),
            "Setup.ini" => (Some(patched.setup_ini.clone()), patched.setup_ini.len() as u64, now),
            _ => {
                let (_, size, date) = disc.file(name)?;
                (None, size, date)
            }
        };
        entries.push(Entry { name, sha256: sha, size, date, data });
    }
    let total = entries.iter().map(|e| e.size).sum();
    let mut c = Copier { r, done: 0, total, buf: vec![0u8; 1 << 20] };

    // 3. the output, written under a temporary name and renamed when complete
    if let Some(parent) = o.out.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
        }
    }
    let part = part_path(&o.out);
    let result = (|| -> Result<()> {
        match o.format {
            Format::Iso => {
                r.status("Writing the disc image");
                let files: Vec<iso::NewFile> = entries.iter().map(|e| iso::NewFile { name: e.name.into(), size: e.size, date: e.date }).collect();
                let created = now_iso_label();
                let layout = iso::layout(&files, &iso::Labels { volume: "RAGNAROK", application: "RAGNAROK OFFLINE SETUP BUILDER", created: &created })?;
                let mut w = BufWriter::with_capacity(1 << 20, File::create(&part).with_context(|| format!("creating {}", part.display()))?);
                w.write_all(&layout.header)?;
                for e in &entries {
                    c.entry(&disc, e, &mut w)?;
                    iso::write_padding(&mut w, e.size)?;
                }
                let f = w.into_inner().map_err(|e| e.into_error())?;
                if f.metadata()?.len() != layout.total_sectors as u64 * iso::SECTOR {
                    bail!("internal error: image size does not match its layout");
                }
                f.sync_all()?;
            }
            Format::Zip => {
                r.status("Writing the zip file");
                let f = BufWriter::with_capacity(1 << 20, File::create(&part).with_context(|| format!("creating {}", part.display()))?);
                let mut z = zipw::ZipWriter::new(f);
                for e in &entries {
                    z.start(&format!("{OUT_NAME}/{}", e.name), e.date)?;
                    c.entry(&disc, e, &mut z)?;
                }
                z.finish()?.into_inner().map_err(|e| e.into_error())?.sync_all()?;
            }
            Format::Folder => {
                r.status("Writing the setup folder");
                if part.exists() {
                    fs::remove_dir_all(&part)?;
                }
                fs::create_dir_all(&part).with_context(|| format!("creating {}", part.display()))?;
                for e in &entries {
                    let path = part.join(e.name);
                    let mut w = BufWriter::with_capacity(1 << 20, File::create(&path).with_context(|| format!("creating {}", path.display()))?);
                    c.entry(&disc, e, &mut w)?;
                    let f = w.into_inner().map_err(|e| e.into_error())?;
                    if e.data.is_none() {
                        set_mtime(&f, e.date);
                    }
                }
            }
        }
        Ok(())
    })();
    if let Err(e) = result {
        if o.format == Format::Folder {
            fs::remove_dir_all(&part).ok();
        } else {
            fs::remove_file(&part).ok();
        }
        return Err(e);
    }
    replace(&part, &o.out, o.format)?;

    // 4. the downloaded image: kept for next time unless the user said otherwise
    let cached = match &o.source {
        Source::Download { keep: false, .. } => {
            fs::remove_file(&disc_path).ok();
            None
        }
        Source::Download { keep: true, .. } => Some(disc_path),
        Source::Local(_) => None,
    };
    Ok(Done { out: o.out.clone(), package_code: patched.package_code, cached })
}

/// Move the finished output into place. An existing output is replaced only
/// if it looks like ours (a folder holding nothing but the disc's files, or a
/// file of the same kind).
fn replace(part: &Path, out: &Path, format: Format) -> Result<()> {
    if out.exists() {
        if format == Format::Folder {
            let ours = fs::read_dir(out)?
                .filter_map(|e| e.ok())
                .all(|e| CD_FILES.iter().any(|(n, _)| e.file_name().to_string_lossy().eq_ignore_ascii_case(n)));
            if !out.is_dir() || !ours {
                fs::remove_dir_all(part).ok();
                bail!("{} already exists and holds other files: choose another folder", out.display());
            }
            fs::remove_dir_all(out)?;
        } else {
            if out.is_dir() {
                fs::remove_file(part).ok();
                bail!("{} is a folder", out.display());
            }
            fs::remove_file(out)?;
        }
    }
    fs::rename(part, out).with_context(|| format!("renaming {} to {}", part.display(), out.display()))
}

fn set_mtime(f: &File, date: [u8; 7]) {
    let Ok(month) = time::Month::try_from(date[1]) else { return };
    let Ok(d) = time::Date::from_calendar_date(1900 + date[0] as i32, month, date[2]) else { return };
    let Ok(t) = time::Time::from_hms(date[3], date[4], date[5]) else { return };
    let offset = time::UtcOffset::from_whole_seconds(date[6] as i8 as i32 * 15 * 60).unwrap_or(time::UtcOffset::UTC);
    let when = time::PrimitiveDateTime::new(d, t).assume_offset(offset);
    let _ = f.set_modified(when.into());
}

/// Free bytes on the volume holding `path` (or its nearest existing parent).
pub fn free_space(path: &Path) -> Option<u64> {
    let mut p = path;
    while !p.exists() {
        p = p.parent()?;
    }
    free_space_at(p)
}

#[cfg(windows)]
fn free_space_at(p: &Path) -> Option<u64> {
    use std::os::windows::ffi::OsStrExt;
    let w: Vec<u16> = p.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
    let mut free = 0u64;
    let ok = unsafe { windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW(w.as_ptr(), &mut free, std::ptr::null_mut(), std::ptr::null_mut()) };
    (ok != 0).then_some(free)
}

#[cfg(not(windows))]
fn free_space_at(_: &Path) -> Option<u64> {
    None
}

/// Bytes the build will write: the output, plus the download if there is none yet.
pub fn space_needed(o: &Options) -> (u64, u64) {
    let out = ia::ISO_SIZE + (8 << 20);
    let dl = match &o.source {
        Source::Download { cache, .. } => {
            let have = cache.join(ia::FILE).metadata().map(|m| m.len()).unwrap_or(0).max(cache.join(format!("{}.part", ia::FILE)).metadata().map(|m| m.len()).unwrap_or(0));
            ia::ISO_SIZE.saturating_sub(have)
        }
        Source::Local(_) => 0,
    };
    (out, dl)
}
