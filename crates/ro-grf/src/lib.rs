//! Gravity GRF archives ("Master of Magic").
//!
//! * v0x102 / v0x103 — `.gpf` patches of 2003: plain file table, file names
//!   and (most) file data obfuscated with Gravity's one-round DES.
//! * v0x200 — `data.grf`: zlib-compressed file table; entries may still carry
//!   the 0x02 (mixed) / 0x04 (header) encryption flags.
//!
//! [`Grf`] reads one archive; [`Vfs`] layers several (later layers win, like
//! a patcher merging `.gpf` into `data.grf`) plus an optional loose `data/`
//! directory; [`write_grf_0x200`] writes a merged archive the client can load.
//!
//! Names are kept as raw bytes (the client's code page, CP949 for Korean
//! paths). Lookup is ASCII-case-insensitive with `/` folded to `\`.

pub mod des;
mod writer;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub use writer::{write_grf_0x200, write_grf_0x200_with_progress};


pub const HEADER_LEN: u64 = 46;
const MAGIC: &[u8] = b"Master of Magic";

pub const FLAG_FILE: u8 = 0x01;
pub const FLAG_MIXCRYPT: u8 = 0x02;
pub const FLAG_HEADERCRYPT: u8 = 0x04;

#[derive(Debug, thiserror::Error)]
pub enum GrfError {
    #[error("io: {0}")]
    Io(#[from] io::Error),
    #[error("not a GRF archive")]
    BadMagic,
    #[error("unsupported GRF version {0:#x}")]
    Version(u32),
    #[error("corrupt file table: {0}")]
    Table(&'static str),
    #[error("{name}: zlib: {err}")]
    Inflate { name: String, err: io::Error },
    #[error("not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, GrfError>;

/// One file inside an archive.
#[derive(Debug, Clone)]
pub struct Entry {
    /// Raw name bytes as stored (backslash separated, client code page).
    pub name: Vec<u8>,
    pub compressed_len: u32,
    pub compressed_len_aligned: u32,
    pub real_len: u32,
    pub flags: u8,
    /// Absolute file offset of the data.
    pub offset: u64,
}

impl Entry {
    pub fn display_name(&self) -> String {
        decode_name(&self.name)
    }
    pub fn is_file(&self) -> bool {
        self.flags & FLAG_FILE != 0
    }
}

/// Decode a raw GRF path for display (CP949, which is a superset of ASCII).
pub fn decode_name(raw: &[u8]) -> String {
    encoding_rs::EUC_KR.decode(raw).0.into_owned()
}

/// Normalised lookup key: ASCII lowercase, `/` -> `\`.
pub fn key_of(raw: &[u8]) -> Vec<u8> {
    raw.iter()
        .map(|&b| if b == b'/' { b'\\' } else { b.to_ascii_lowercase() })
        .collect()
}

pub struct Grf {
    path: PathBuf,
    version: u32,
    entries: Vec<Entry>,
    index: HashMap<Vec<u8>, usize>,
    file: Mutex<File>,
}

fn u32le(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

impl Grf {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let mut f = File::open(&path)?;
        let mut hdr = [0u8; HEADER_LEN as usize];
        f.read_exact(&mut hdr)?;
        if &hdr[..MAGIC.len()] != MAGIC {
            return Err(GrfError::BadMagic);
        }
        let table_off = u32le(&hdr, 30) as u64 + HEADER_LEN;
        let seed = u32le(&hdr, 34);
        let n2 = u32le(&hdr, 38);
        let version = u32le(&hdr, 42);
        let count = n2.checked_sub(seed).and_then(|c| c.checked_sub(7)).ok_or(GrfError::Table("count"))?;
        f.seek(SeekFrom::Start(table_off))?;
        let entries = match version >> 8 {
            0x01 => {
                let mut tbl = Vec::new();
                f.read_to_end(&mut tbl)?;
                parse_table_v1(&tbl, count)?
            }
            0x02 => {
                let mut sz = [0u8; 8];
                f.read_exact(&mut sz)?;
                let (clen, ulen) = (u32le(&sz, 0) as usize, u32le(&sz, 4) as usize);
                let mut comp = vec![0u8; clen];
                f.read_exact(&mut comp)?;
                let tbl = inflate(&comp, ulen).map_err(|err| GrfError::Inflate { name: "<file table>".into(), err })?;
                parse_table_v2(&tbl, count)?
            }
            _ => return Err(GrfError::Version(version)),
        };
        let mut index = HashMap::with_capacity(entries.len());
        for (i, e) in entries.iter().enumerate() {
            index.insert(key_of(&e.name), i);
        }
        Ok(Grf { path, version, entries, index, file: Mutex::new(f) })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
    pub fn get(&self, name: &str) -> Option<&Entry> {
        self.get_raw(name.as_bytes())
    }
    pub fn get_raw(&self, name: &[u8]) -> Option<&Entry> {
        self.index.get(&key_of(name)).map(|&i| &self.entries[i])
    }

    /// Raw stored bytes of an entry with any GRF encryption removed (still
    /// zlib-compressed). Length is `compressed_len_aligned`.
    pub fn read_compressed(&self, e: &Entry) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; e.compressed_len_aligned as usize];
        {
            let mut f = self.file.lock().unwrap();
            f.seek(SeekFrom::Start(e.offset))?;
            f.read_exact(&mut buf)?;
        }
        if e.flags & FLAG_MIXCRYPT != 0 {
            des::decode_full(&mut buf, e.compressed_len as usize);
        } else if e.flags & FLAG_HEADERCRYPT != 0 {
            des::decode_header(&mut buf);
        }
        Ok(buf)
    }

    pub fn read_entry(&self, e: &Entry) -> Result<Vec<u8>> {
        let comp = self.read_compressed(e)?;
        inflate(&comp[..e.compressed_len as usize], e.real_len as usize)
            .map_err(|err| GrfError::Inflate { name: e.display_name(), err })
    }

    pub fn read(&self, name: &str) -> Result<Vec<u8>> {
        let e = self.get(name).ok_or_else(|| GrfError::NotFound(name.into()))?;
        self.read_entry(e)
    }
}

fn inflate(comp: &[u8], expect: usize) -> io::Result<Vec<u8>> {
    let mut out = Vec::with_capacity(expect);
    flate2::read::ZlibDecoder::new(comp).read_to_end(&mut out)?;
    if out.len() != expect {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("inflated {} bytes, table says {}", out.len(), expect),
        ));
    }
    Ok(out)
}

/// 0x1xx: `u32 namefield_len; u8[2] ?; u8[namefield_len-6] encrypted name; ...;
/// u32 clen_obf; u32 clen_aligned_obf; u32 real_len; u8 flags; u32 offset`.
fn parse_table_v1(tbl: &[u8], count: u32) -> Result<Vec<Entry>> {
    let mut out = Vec::with_capacity(count as usize);
    let mut ofs = 0usize;
    for _ in 0..count {
        if ofs + 4 > tbl.len() {
            return Err(GrfError::Table("truncated v1 entry"));
        }
        let field = u32le(tbl, ofs) as usize;
        let ofs2 = ofs + field + 4;
        if ofs2 + 17 > tbl.len() || field < 6 {
            return Err(GrfError::Table("v1 entry overflows table"));
        }
        let flags = tbl[ofs2 + 12];
        if flags & FLAG_FILE != 0 {
            let mut name = tbl[ofs + 6..ofs + 6 + (field - 6)].to_vec();
            des::decode_filename(&mut name);
            if let Some(z) = name.iter().position(|&b| b == 0) {
                name.truncate(z);
            }
            let real_len = u32le(tbl, ofs2 + 8);
            let clen = u32le(tbl, ofs2).wrapping_sub(real_len).wrapping_sub(715);
            let clen_aligned = u32le(tbl, ofs2 + 4).wrapping_sub(37579);
            let mut flags = flags;
            flags |= if full_encrypt(&name) { FLAG_MIXCRYPT } else { FLAG_HEADERCRYPT };
            out.push(Entry {
                name,
                compressed_len: clen,
                compressed_len_aligned: clen_aligned,
                real_len,
                flags,
                offset: u32le(tbl, ofs2 + 13) as u64 + HEADER_LEN,
            });
        }
        ofs = ofs2 + 17;
    }
    Ok(out)
}

/// .gnd/.gat/.act/.str in 0x1xx archives only have their header encrypted.
fn full_encrypt(name: &[u8]) -> bool {
    let lower = key_of(name);
    ![b".gnd".as_slice(), b".gat", b".act", b".str"].iter().any(|ext| lower.ends_with(ext))
}

/// 0x200: `name\0; u32 clen; u32 clen_aligned; u32 real_len; u8 flags; u32 offset`.
fn parse_table_v2(tbl: &[u8], count: u32) -> Result<Vec<Entry>> {
    let mut out = Vec::with_capacity(count as usize);
    let mut p = 0usize;
    for _ in 0..count {
        let z = tbl[p..].iter().position(|&b| b == 0).ok_or(GrfError::Table("unterminated name"))? + p;
        if z + 1 + 17 > tbl.len() {
            return Err(GrfError::Table("v2 entry overflows table"));
        }
        let q = z + 1;
        let e = Entry {
            name: tbl[p..z].to_vec(),
            compressed_len: u32le(tbl, q),
            compressed_len_aligned: u32le(tbl, q + 4),
            real_len: u32le(tbl, q + 8),
            flags: tbl[q + 12],
            offset: u32le(tbl, q + 13) as u64 + HEADER_LEN,
        };
        if e.is_file() {
            out.push(e);
        }
        p = q + 17;
    }
    Ok(out)
}

/// Layered view over several archives (+ optional loose directory).
///
/// Layer order = patch order: `push` later archives last; lookups search from
/// the top. A loose directory (the client's `data\` folder) is searched first
/// when present, mirroring clients that read loose files before the GRF.
#[derive(Default)]
pub struct Vfs {
    loose: Option<PathBuf>,
    layers: Vec<Grf>,
    /// In-memory files on top of every archive (e.g. a generated clientinfo.xml).
    overlay: HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)>, // key -> (raw name, data)
}

/// Where a file visible through a [`Vfs`] comes from.
pub enum Source<'a> {
    Archive { layer: usize, entry: &'a Entry },
    Memory { name: &'a [u8], data: &'a [u8] },
}

impl Source<'_> {
    pub fn name(&self) -> &[u8] {
        match self {
            Source::Archive { entry, .. } => &entry.name,
            Source::Memory { name, .. } => name,
        }
    }
}

impl Vfs {
    pub fn new() -> Self {
        Self::default()
    }
    /// `root` is the directory that *contains* `data\` (the client dir).
    pub fn set_loose_root(&mut self, root: impl Into<PathBuf>) {
        self.loose = Some(root.into());
    }
    pub fn push(&mut self, grf: Grf) {
        self.layers.push(grf);
    }
    pub fn layers(&self) -> &[Grf] {
        &self.layers
    }
    /// Put a file above every archive (name uses `\\` separators).
    pub fn add_memory_file(&mut self, name: &str, data: Vec<u8>) {
        let raw = name.replace('/', "\\").into_bytes();
        self.overlay.insert(key_of(&raw), (raw, data));
    }

    pub fn read(&self, name: &str) -> Result<Vec<u8>> {
        if let Some((_, d)) = self.overlay.get(&key_of(name.as_bytes())) {
            return Ok(d.clone());
        }
        if let Some(root) = &self.loose {
            let rel: PathBuf = name.split(['\\', '/']).collect();
            if let Ok(b) = std::fs::read(root.join(rel)) {
                return Ok(b);
            }
        }
        for g in self.layers.iter().rev() {
            if let Some(e) = g.get(name) {
                return g.read_entry(e);
            }
        }
        Err(GrfError::NotFound(name.into()))
    }

    pub fn exists(&self, name: &str) -> bool {
        self.overlay.contains_key(&key_of(name.as_bytes()))
            || self.layers.iter().any(|g| g.get(name).is_some())
    }

    /// Every distinct file, top-most source winning, sorted by name.
    pub fn merged(&self) -> Vec<Source<'_>> {
        let mut seen: HashMap<Vec<u8>, Source<'_>> = HashMap::new();
        for (layer, g) in self.layers.iter().enumerate() {
            for entry in g.entries() {
                seen.insert(key_of(&entry.name), Source::Archive { layer, entry });
            }
        }
        for (k, (name, data)) in &self.overlay {
            seen.insert(k.clone(), Source::Memory { name, data });
        }
        let mut v: Vec<_> = seen.into_values().collect();
        v.sort_by(|a, b| a.name().cmp(b.name()));
        v
    }
}
