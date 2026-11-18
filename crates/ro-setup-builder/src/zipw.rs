//! A minimal zip writer: stored (uncompressed) entries, sizes under 4 GiB.
//! Data1.cab is already compressed and makes up 98% of the bytes, so
//! deflating would only cost time. Each entry's CRC and sizes are written
//! back into its local header once its data is known.

use std::io::{self, Seek, SeekFrom, Write};

pub struct ZipWriter<W: Write + Seek> {
    w: W,
    central: Vec<u8>,
    count: u16,
    open: Option<Open>,
}

struct Open {
    header_at: u64,
    name: Vec<u8>,
    dos: (u16, u16),
    crc: crc32fast::Hasher,
    size: u64,
}

/// MS-DOS date and time from an ISO 9660 recording date (local time as
/// recorded on the disc; DOS times have no zone).
pub fn dos_time(d: [u8; 7]) -> (u16, u16) {
    let year = (1900 + d[0] as u16).clamp(1980, 2107);
    let date = ((year - 1980) << 9) | ((d[1] as u16 & 15) << 5) | (d[2] as u16 & 31);
    let time = ((d[3] as u16 & 31) << 11) | ((d[4] as u16 & 63) << 5) | (d[5] as u16 / 2);
    (time, date)
}

impl<W: Write + Seek> ZipWriter<W> {
    pub fn new(w: W) -> Self {
        ZipWriter { w, central: Vec::new(), count: 0, open: None }
    }

    /// Start an entry; write its bytes with `write`.
    pub fn start(&mut self, name: &str, date: [u8; 7]) -> io::Result<()> {
        self.finish_entry()?;
        let header_at = self.w.stream_position()?;
        let name = name.as_bytes().to_vec();
        let dos = dos_time(date);
        let mut h = Vec::with_capacity(30 + name.len());
        h.extend_from_slice(&0x04034b50u32.to_le_bytes());
        h.extend_from_slice(&10u16.to_le_bytes()); // version needed: 1.0 (stored)
        h.extend_from_slice(&0u16.to_le_bytes()); // flags
        h.extend_from_slice(&0u16.to_le_bytes()); // stored
        h.extend_from_slice(&dos.0.to_le_bytes());
        h.extend_from_slice(&dos.1.to_le_bytes());
        h.extend_from_slice(&[0; 12]); // crc, sizes: written back in finish_entry
        h.extend_from_slice(&(name.len() as u16).to_le_bytes());
        h.extend_from_slice(&0u16.to_le_bytes());
        h.extend_from_slice(&name);
        self.w.write_all(&h)?;
        self.open = Some(Open { header_at, name, dos, crc: crc32fast::Hasher::new(), size: 0 });
        Ok(())
    }

    fn finish_entry(&mut self) -> io::Result<()> {
        let Some(e) = self.open.take() else { return Ok(()) };
        if e.size > u32::MAX as u64 {
            return Err(io::Error::other("zip entry over 4 GiB"));
        }
        let crc = e.crc.finalize();
        let end = self.w.stream_position()?;
        self.w.seek(SeekFrom::Start(e.header_at + 14))?;
        let mut f = Vec::with_capacity(12);
        f.extend_from_slice(&crc.to_le_bytes());
        f.extend_from_slice(&(e.size as u32).to_le_bytes());
        f.extend_from_slice(&(e.size as u32).to_le_bytes());
        self.w.write_all(&f)?;
        self.w.seek(SeekFrom::Start(end))?;

        let c = &mut self.central;
        c.extend_from_slice(&0x02014b50u32.to_le_bytes());
        c.extend_from_slice(&20u16.to_le_bytes()); // made by: MS-DOS, 2.0
        c.extend_from_slice(&10u16.to_le_bytes());
        c.extend_from_slice(&0u16.to_le_bytes());
        c.extend_from_slice(&0u16.to_le_bytes());
        c.extend_from_slice(&e.dos.0.to_le_bytes());
        c.extend_from_slice(&e.dos.1.to_le_bytes());
        c.extend_from_slice(&f);
        c.extend_from_slice(&(e.name.len() as u16).to_le_bytes());
        c.extend_from_slice(&[0; 8]); // extra, comment, disk, internal attrs
        c.extend_from_slice(&0u32.to_le_bytes()); // external attrs
        c.extend_from_slice(&(e.header_at as u32).to_le_bytes());
        c.extend_from_slice(&e.name);
        self.count += 1;
        Ok(())
    }

    pub fn finish(mut self) -> io::Result<W> {
        self.finish_entry()?;
        let at = self.w.stream_position()?;
        if at > u32::MAX as u64 {
            return Err(io::Error::other("zip over 4 GiB"));
        }
        self.w.write_all(&self.central)?;
        let mut e = Vec::with_capacity(22);
        e.extend_from_slice(&0x06054b50u32.to_le_bytes());
        e.extend_from_slice(&[0; 4]);
        e.extend_from_slice(&self.count.to_le_bytes());
        e.extend_from_slice(&self.count.to_le_bytes());
        e.extend_from_slice(&(self.central.len() as u32).to_le_bytes());
        e.extend_from_slice(&(at as u32).to_le_bytes());
        e.extend_from_slice(&0u16.to_le_bytes());
        self.w.write_all(&e)?;
        self.w.flush()?;
        Ok(self.w)
    }
}

impl<W: Write + Seek> Write for ZipWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let e = self.open.as_mut().ok_or_else(|| io::Error::other("no zip entry started"))?;
        let n = self.w.write(buf)?;
        e.crc.update(&buf[..n]);
        e.size += n as u64;
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.w.flush()
    }
}
