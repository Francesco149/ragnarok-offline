use std::fs::File;
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::path::Path;

use crate::{HEADER_LEN, Result, Source, Vfs};

/// Write every file visible through `vfs` (top layer wins) into a new GRF
/// v0x200 archive at `out`. Entry data is copied still-compressed; GRF
/// encryption is removed (entries are written with flags = 0x01), which every
/// client that reads 0x200 accepts.
///
/// Returns the number of entries written. Streams one entry at a time, so
/// memory use is bounded by the largest single entry.
pub fn write_grf_0x200(vfs: &Vfs, out: &Path) -> Result<usize> {
    write_grf_0x200_with_progress(vfs, out, &mut |_, _| {})
}

/// [`write_grf_0x200`], calling `progress(done, total)` (entries) as it goes.
pub fn write_grf_0x200_with_progress(vfs: &Vfs, out: &Path, progress: &mut dyn FnMut(usize, usize)) -> Result<usize> {
    let merged = vfs.merged();
    let total = merged.len();
    let f = File::create(out)?;
    let mut w = BufWriter::with_capacity(1 << 20, f);
    w.write_all(&[0u8; HEADER_LEN as usize])?;
    let mut table = Vec::with_capacity(merged.len() * 64);
    let mut pos: u64 = 0; // relative to end of header
    for (i, src) in merged.iter().enumerate() {
        if i % 256 == 0 {
            progress(i, total);
        }
        let (data, real_len) = match src {
            Source::Archive { layer, entry } => {
                let mut d = vfs.layers()[*layer].read_compressed(entry)?;
                d.truncate(entry.compressed_len as usize);
                (d, entry.real_len)
            }
            Source::Memory { data, .. } => {
                let mut enc = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
                enc.write_all(data)?;
                (enc.finish()?, data.len() as u32)
            }
        };
        w.write_all(&data)?;
        table.extend_from_slice(src.name());
        table.push(0);
        table.extend_from_slice(&(data.len() as u32).to_le_bytes());
        table.extend_from_slice(&(data.len() as u32).to_le_bytes());
        table.extend_from_slice(&real_len.to_le_bytes());
        table.push(crate::FLAG_FILE);
        table.extend_from_slice(&(pos as u32).to_le_bytes());
        pos += data.len() as u64;
        if pos > u32::MAX as u64 {
            return Err(std::io::Error::other("archive exceeds 4 GiB").into());
        }
    }
    let table_off = pos as u32;
    let mut enc = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    enc.write_all(&table)?;
    let comp = enc.finish()?;
    w.write_all(&(comp.len() as u32).to_le_bytes())?;
    w.write_all(&(table.len() as u32).to_le_bytes())?;
    w.write_all(&comp)?;
    let mut hdr = Vec::with_capacity(HEADER_LEN as usize);
    hdr.extend_from_slice(b"Master of Magic\0");
    hdr.extend_from_slice(&[0u8; 14]); // key: zero, as in the original data.grf
    hdr.extend_from_slice(&table_off.to_le_bytes());
    hdr.extend_from_slice(&0u32.to_le_bytes()); // seed
    hdr.extend_from_slice(&(merged.len() as u32 + 7).to_le_bytes());
    hdr.extend_from_slice(&0x200u32.to_le_bytes());
    w.seek(SeekFrom::Start(0))?;
    w.write_all(&hdr)?;
    w.flush()?;
    progress(total, total);
    Ok(merged.len())
}
