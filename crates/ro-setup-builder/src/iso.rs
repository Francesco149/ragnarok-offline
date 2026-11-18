//! Just enough ISO 9660 + Joliet for this disc: its 9 files sit in the root
//! directory, so only the root is read and only a flat root is written.
//!
//! Reading prefers the Joliet tree (long, mixed-case names such as
//! "Ragnarok Online.msi"); the primary tree is the fallback.
//! Writing produces what `xorriso -as mkisofs -V RAGNAROK -J -joliet-long`
//! made for the first all-in-one images: a primary tree with ISO 9660 names
//! (upper case, `;1`), a Joliet tree with the real names, no Rock Ridge,
//! one file after another from sector 25 on.

use std::io::{self, Read, Seek, SeekFrom, Write};

use anyhow::{Context, Result, bail};

pub const SECTOR: u64 = 2048;

/// A file in the root directory of an image.
#[derive(Clone, Debug)]
pub struct IsoFile {
    pub name: String,
    /// byte offset of its data in the image
    pub offset: u64,
    pub size: u64,
    /// ISO 9660 recording date: years since 1900, month, day, hour, minute,
    /// second, offset from GMT in 15-minute steps
    pub date: [u8; 7],
}

fn u32_le(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

fn read_sector<R: Read + Seek>(r: &mut R, lba: u64, buf: &mut [u8]) -> io::Result<()> {
    r.seek(SeekFrom::Start(lba * SECTOR))?;
    r.read_exact(buf)
}

/// The files in the root directory (sub-directories are skipped).
pub fn read_root<R: Read + Seek>(r: &mut R) -> Result<Vec<IsoFile>> {
    let mut primary = None;
    let mut joliet = None;
    let mut vd = vec![0u8; SECTOR as usize];
    for lba in 16..64 {
        read_sector(r, lba, &mut vd).context("reading the volume descriptors")?;
        if &vd[1..6] != b"CD001" {
            bail!("not an ISO 9660 image (no volume descriptor at sector {lba})");
        }
        match vd[0] {
            1 => primary = Some(vd[156..190].to_vec()),
            // UCS-2 escape sequences %/@, %/C, %/E
            2 if vd[88] == b'%' && vd[89] == b'/' && matches!(vd[90], b'@' | b'C' | b'E') => joliet = Some(vd[156..190].to_vec()),
            255 => break,
            _ => {}
        }
    }
    let (root, is_joliet) = match (joliet, primary) {
        (Some(j), _) => (j, true),
        (None, Some(p)) => (p, false),
        _ => bail!("no primary volume descriptor"),
    };
    let (lba, len) = (u32_le(&root, 2) as u64, u32_le(&root, 10) as u64);
    if len > 1 << 20 {
        bail!("root directory too large ({len} bytes)");
    }
    let mut dir = vec![0u8; len as usize];
    r.seek(SeekFrom::Start(lba * SECTOR))?;
    r.read_exact(&mut dir).context("reading the root directory")?;

    let mut files = Vec::new();
    let mut at = 0usize;
    while at < dir.len() {
        let rec_len = dir[at] as usize;
        if rec_len == 0 {
            // records never cross a sector: skip to the next one
            at = (at / SECTOR as usize + 1) * SECTOR as usize;
            continue;
        }
        if at + rec_len > dir.len() || rec_len < 34 {
            bail!("damaged directory record at {at}");
        }
        let rec = &dir[at..at + rec_len];
        at += rec_len;
        let name_len = rec[32] as usize;
        let raw = &rec[33..33 + name_len];
        if rec[25] & 2 != 0 || raw == [0] || raw == [1] {
            continue; // directory, "." or ".."
        }
        if rec[25] & 0x80 != 0 {
            bail!("multi-extent files are not supported");
        }
        let mut name = if is_joliet {
            let units: Vec<u16> = raw.chunks_exact(2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect();
            String::from_utf16_lossy(&units)
        } else {
            String::from_utf8_lossy(raw).into_owned()
        };
        if let Some(i) = name.rfind(';') {
            name.truncate(i);
        }
        if !is_joliet {
            name = name.trim_end_matches('.').to_string();
        }
        files.push(IsoFile {
            name,
            offset: u32_le(rec, 2) as u64 * SECTOR,
            size: u32_le(rec, 10) as u64,
            date: rec[18..25].try_into().unwrap(),
        });
    }
    Ok(files)
}

// ---------------------------------------------------------------- writing

/// A file to put in the root of a new image.
pub struct NewFile {
    pub name: String,
    pub size: u64,
    pub date: [u8; 7],
}

pub struct Labels<'a> {
    pub volume: &'a str,
    pub application: &'a str,
    /// creation time as 16 ASCII digits YYYYMMDDHHMMSScc (GMT)
    pub created: &'a str,
}

fn both16(v: u16) -> [u8; 4] {
    let (l, b) = (v.to_le_bytes(), v.to_be_bytes());
    [l[0], l[1], b[0], b[1]]
}

fn both32(v: u32) -> [u8; 8] {
    let (l, b) = (v.to_le_bytes(), v.to_be_bytes());
    [l[0], l[1], l[2], l[3], b[0], b[1], b[2], b[3]]
}

/// ISO 9660 name: d-characters (A-Z 0-9 _), one dot, ";1".
fn iso_name(name: &str) -> Vec<u8> {
    let up = name.to_ascii_uppercase();
    let (stem, ext) = match up.rfind('.') {
        Some(i) => (&up[..i], &up[i + 1..]),
        None => (&up[..], ""),
    };
    let clean = |s: &str| -> String { s.chars().map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' }).collect() };
    let mut stem = clean(stem);
    stem.truncate(30 - ext.len().min(30));
    format!("{stem}.{};1", clean(ext)).into_bytes()
}

fn joliet_name(name: &str) -> Vec<u8> {
    // -joliet-long allows 103 characters
    name.encode_utf16().take(103).flat_map(|u| u.to_be_bytes()).collect()
}

fn dir_record(lba: u32, size: u32, date: [u8; 7], is_dir: bool, name: &[u8]) -> Vec<u8> {
    let mut r = vec![0u8; 33];
    r[2..10].copy_from_slice(&both32(lba));
    r[10..18].copy_from_slice(&both32(size));
    r[18..25].copy_from_slice(&date);
    r[25] = if is_dir { 2 } else { 0 };
    r[28..32].copy_from_slice(&both16(1));
    r[32] = name.len() as u8;
    r.extend_from_slice(name);
    if r.len() % 2 == 1 {
        r.push(0);
    }
    r[0] = r.len() as u8;
    r
}

/// Directory contents: ".", "..", then the records, none crossing a sector.
fn directory(self_lba: u32, sectors: u32, date: [u8; 7], records: &[Vec<u8>]) -> Vec<u8> {
    let size = sectors * SECTOR as u32;
    let mut out = Vec::new();
    for rec in [dir_record(self_lba, size, date, true, &[0]), dir_record(self_lba, size, date, true, &[1])].iter().chain(records) {
        let room = SECTOR as usize - out.len() % SECTOR as usize;
        if rec.len() > room {
            out.resize(out.len() + room, 0);
        }
        out.extend_from_slice(rec);
    }
    out.resize(size as usize, 0);
    out
}

fn dir_sectors(records: &[Vec<u8>]) -> u32 {
    // "." and ".." are 34 bytes each
    let mut used = 68usize;
    let mut sectors = 1;
    for r in records {
        if used + r.len() > SECTOR as usize {
            sectors += 1;
            used = 0;
        }
        used += r.len();
    }
    sectors
}

fn path_table(root_lba: u32, big_endian: bool) -> Vec<u8> {
    let mut t = vec![1, 0];
    t.extend_from_slice(&if big_endian { root_lba.to_be_bytes() } else { root_lba.to_le_bytes() });
    t.extend_from_slice(&if big_endian { 1u16.to_be_bytes() } else { 1u16.to_le_bytes() });
    t.extend_from_slice(&[0, 0]); // root's name (one zero byte) + pad
    t
}

fn pad_field(dst: &mut [u8], s: &str, joliet: bool) {
    if joliet {
        let units: Vec<u16> = s.encode_utf16().collect();
        for (i, c) in dst.chunks_exact_mut(2).enumerate() {
            c.copy_from_slice(&units.get(i).copied().unwrap_or(0x20).to_be_bytes());
        }
    } else {
        for (i, b) in dst.iter_mut().enumerate() {
            *b = s.as_bytes().get(i).copied().unwrap_or(b' ');
        }
    }
}

/// The image is laid out as:
///   0-15 system area, 16 primary, 17 Joliet, 18 terminator,
///   19-22 path tables (primary L/M, Joliet L/M), then the two root
///   directories, then the files in the order given, each on a new sector.
/// Returns the header (everything before the first file) and each file's
/// first sector. The caller writes the header, then each file's data
/// padded to a whole sector.
pub struct Layout {
    pub header: Vec<u8>,
    pub total_sectors: u32,
}

pub fn layout(files: &[NewFile], labels: &Labels) -> Result<Layout> {
    // names sorted as ISO 9660 wants each directory sorted, per tree
    let mut primary: Vec<(Vec<u8>, usize)> = files.iter().enumerate().map(|(i, f)| (iso_name(&f.name), i)).collect();
    let mut joliet: Vec<(Vec<u8>, usize)> = files.iter().enumerate().map(|(i, f)| (joliet_name(&f.name), i)).collect();
    primary.sort();
    joliet.sort();
    for w in primary.windows(2).chain(joliet.windows(2)) {
        if w[0].0 == w[1].0 {
            bail!("two files map to the same ISO 9660 name: {} and {}", files[w[0].1].name, files[w[1].1].name);
        }
    }

    // sector numbers: directories first (their size depends only on names)
    let fake = |names: &[(Vec<u8>, usize)]| -> Vec<Vec<u8>> { names.iter().map(|(n, _)| dir_record(0, 0, [0; 7], false, n)).collect() };
    let p_secs = dir_sectors(&fake(&primary));
    let j_secs = dir_sectors(&fake(&joliet));
    let p_root = 23u32;
    let j_root = p_root + p_secs;
    let mut next = j_root + j_secs;
    let mut lba = vec![0u32; files.len()];
    for (i, f) in files.iter().enumerate() {
        if f.size > u32::MAX as u64 {
            bail!("{} is too large for one ISO 9660 extent", f.name);
        }
        lba[i] = next;
        next += f.size.div_ceil(SECTOR) as u32;
    }
    let total = next;

    let root_date = files.iter().map(|f| f.date).max().unwrap_or([0; 7]);
    let recs = |names: &[(Vec<u8>, usize)]| -> Vec<Vec<u8>> {
        names.iter().map(|(n, i)| dir_record(lba[*i], files[*i].size as u32, files[*i].date, false, n)).collect()
    };
    let p_dir = directory(p_root, p_secs, root_date, &recs(&primary));
    let j_dir = directory(j_root, j_secs, root_date, &recs(&joliet));

    let mut header = vec![0u8; (16 * SECTOR) as usize];
    let created = format!("{:0<16}", labels.created);
    for (kind, root, secs, ltab) in [(1u8, p_root, p_secs, 19u32), (2u8, j_root, j_secs, 21u32)] {
        let joliet = kind == 2;
        let mut vd = vec![0u8; SECTOR as usize];
        vd[0] = kind;
        vd[1..6].copy_from_slice(b"CD001");
        vd[6] = 1;
        pad_field(&mut vd[8..40], "", joliet);
        pad_field(&mut vd[40..72], labels.volume, joliet);
        vd[80..88].copy_from_slice(&both32(total));
        if joliet {
            vd[88..91].copy_from_slice(b"%/E"); // UCS-2 level 3
        }
        vd[120..124].copy_from_slice(&both16(1));
        vd[124..128].copy_from_slice(&both16(1));
        vd[128..132].copy_from_slice(&both16(SECTOR as u16));
        vd[132..140].copy_from_slice(&both32(10));
        vd[140..144].copy_from_slice(&ltab.to_le_bytes());
        vd[148..152].copy_from_slice(&(ltab + 1).to_be_bytes());
        let root_rec = dir_record(root, secs * SECTOR as u32, root_date, true, &[0]);
        vd[156..190].copy_from_slice(&root_rec);
        for (a, b) in [(190, 318), (318, 446), (446, 574)] {
            pad_field(&mut vd[a..b], "", joliet);
        }
        pad_field(&mut vd[574..702], labels.application, joliet);
        for (a, b) in [(702, 739), (739, 776), (776, 813)] {
            pad_field(&mut vd[a..b], "", joliet);
            if joliet {
                vd[b - 1] = 0x20; // odd-length fields: the last byte is a lone space
            }
        }
        for (at, s) in [(813, created.as_str()), (830, created.as_str()), (847, "0000000000000000"), (864, "0000000000000000")] {
            vd[at..at + 16].copy_from_slice(s.as_bytes());
            vd[at + 16] = 0;
        }
        vd[881] = 1;
        header.extend_from_slice(&vd);
    }
    let mut term = vec![0u8; SECTOR as usize];
    term[0] = 255;
    term[1..6].copy_from_slice(b"CD001");
    term[6] = 1;
    header.extend_from_slice(&term);
    for (root, be) in [(p_root, false), (p_root, true), (j_root, false), (j_root, true)] {
        let mut s = path_table(root, be);
        s.resize(SECTOR as usize, 0);
        header.extend_from_slice(&s);
    }
    header.extend_from_slice(&p_dir);
    header.extend_from_slice(&j_dir);
    assert_eq!(header.len() as u64, lba.first().copied().unwrap_or(total) as u64 * SECTOR);
    Ok(Layout { header, total_sectors: total })
}

/// Zero bytes to put after a file of `size` bytes.
pub fn padding(size: u64) -> usize {
    ((SECTOR - size % SECTOR) % SECTOR) as usize
}

pub fn write_padding<W: Write>(w: &mut W, size: u64) -> io::Result<()> {
    w.write_all(&vec![0u8; padding(size)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn round_trip() {
        let files = vec![
            NewFile { name: "Ragnarok Online.msi".into(), size: 5000, date: [103, 12, 19, 11, 31, 6, 4] },
            NewFile { name: "setup.exe".into(), size: 2048, date: [103, 12, 19, 0, 0, 0, 0] },
            NewFile { name: "0x0409.ini".into(), size: 0, date: [102, 5, 2, 0, 0, 0, 0] },
        ];
        let l = layout(&files, &Labels { volume: "RAGNAROK", application: "TEST", created: "2026111800000000" }).unwrap();
        let mut img = l.header.clone();
        for (i, f) in files.iter().enumerate() {
            img.extend(std::iter::repeat_n(i as u8 + 1, f.size as usize));
            img.extend(vec![0u8; padding(f.size)]);
        }
        assert_eq!(img.len() as u64, l.total_sectors as u64 * SECTOR);
        let got = read_root(&mut Cursor::new(&img)).unwrap();
        assert_eq!(got.len(), 3);
        for (i, f) in files.iter().enumerate() {
            let g = got.iter().find(|g| g.name == f.name).unwrap();
            assert_eq!((g.size, g.date), (f.size, f.date));
            let data = &img[g.offset as usize..(g.offset + g.size) as usize];
            assert!(data.iter().all(|&b| b == i as u8 + 1));
        }
        assert_eq!(iso_name("Ragnarok Online.msi"), b"RAGNAROK_ONLINE.MSI;1");
    }
}
