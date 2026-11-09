//! Wire protocol of the 2003-12-18 euRO `Ragexe.exe` (eAthena `packet_ver 5`
//! era: the plain, pre-shuffle layout).
//!
//! * [`lengths`]: the packet length table, read straight out of the client
//!   (`research/extract_packet_lengths.py` -> `data/client/packet_lengths.tsv`).
//!   It is authoritative for everything the server sends; [`Writer::finish`]
//!   refuses to emit a packet whose length disagrees with it.
//! * [`Framer`]: splits the client's byte stream into packets.
//! * [`Writer`] / [`Reader`]: little-endian field access, fixed-size strings,
//!   RO's packed 3-byte position and 6-byte move encodings.
//! * [`ids`]: names for the packet ids the server uses.

pub mod ids;
pub mod lengths;

pub use lengths::{PacketLen, client_send_len, server_len};

#[derive(Debug, thiserror::Error)]
pub enum ProtoError {
    #[error("unknown packet id {0:#06x}")]
    UnknownId(u16),
    #[error("packet {id:#06x}: bad length {len}")]
    BadLength { id: u16, len: usize },
    #[error("packet {id:#06x}: read past end at {at}+{want} of {len}")]
    Short { id: u16, at: usize, want: usize, len: usize },
}

/// Stream framer for client -> server traffic.
#[derive(Default)]
pub struct Framer {
    buf: Vec<u8>,
}

impl Framer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }
    /// Next complete packet, if any. An unknown id is fatal for the stream
    /// (the client and server no longer agree on framing).
    pub fn next_packet(&mut self) -> Result<Option<Vec<u8>>, ProtoError> {
        if self.buf.len() < 2 {
            return Ok(None);
        }
        let id = u16::from_le_bytes([self.buf[0], self.buf[1]]);
        let len = match client_send_len(id) {
            Some(PacketLen::Fixed(n)) => n as usize,
            Some(PacketLen::Variable) => {
                if self.buf.len() < 4 {
                    return Ok(None);
                }
                let n = u16::from_le_bytes([self.buf[2], self.buf[3]]) as usize;
                if n < 4 {
                    return Err(ProtoError::BadLength { id, len: n });
                }
                n
            }
            None => return Err(ProtoError::UnknownId(id)),
        };
        if len < 2 {
            return Err(ProtoError::BadLength { id, len });
        }
        if self.buf.len() < len {
            return Ok(None);
        }
        let rest = self.buf.split_off(len);
        Ok(Some(std::mem::replace(&mut self.buf, rest)))
    }
}

/// Packet builder. Fixed-length packets are checked against the client's
/// table on `finish`; variable-length ones get their u16 length patched in.
pub struct Writer {
    id: u16,
    buf: Vec<u8>,
}

impl Writer {
    pub fn new(id: u16) -> Self {
        let mut buf = Vec::with_capacity(32);
        buf.extend_from_slice(&id.to_le_bytes());
        if matches!(server_len(id), Some(PacketLen::Variable)) {
            buf.extend_from_slice(&[0, 0]);
        }
        Writer { id, buf }
    }
    pub fn u8(&mut self, v: u8) -> &mut Self {
        self.buf.push(v);
        self
    }
    pub fn u16(&mut self, v: u16) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn i16(&mut self, v: i16) -> &mut Self {
        self.u16(v as u16)
    }
    pub fn u32(&mut self, v: u32) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn i32(&mut self, v: i32) -> &mut Self {
        self.u32(v as u32)
    }
    pub fn bytes(&mut self, b: &[u8]) -> &mut Self {
        self.buf.extend_from_slice(b);
        self
    }
    pub fn zeros(&mut self, n: usize) -> &mut Self {
        self.buf.resize(self.buf.len() + n, 0);
        self
    }
    /// Fixed-size, NUL-padded string field (truncated to `size - 1` bytes so
    /// it stays NUL-terminated, as the client expects).
    pub fn str_fixed(&mut self, s: &str, size: usize) -> &mut Self {
        let enc = encode_text(s);
        let n = enc.len().min(size.saturating_sub(1));
        self.buf.extend_from_slice(&enc[..n]);
        self.zeros(size - n)
    }
    /// NUL-terminated string (for variable packets).
    pub fn str_nul(&mut self, s: &str) -> &mut Self {
        self.buf.extend_from_slice(&encode_text(s));
        self.u8(0)
    }
    /// Packed cell position + facing (3 bytes).
    pub fn pos3(&mut self, x: u16, y: u16, dir: u8) -> &mut Self {
        self.bytes(&pack_pos3(x, y, dir))
    }
    /// Packed source/destination of a move (6 bytes), sub-cell 8/8.
    pub fn move6(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) -> &mut Self {
        self.bytes(&pack_move6(x0, y0, x1, y1))
    }
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Finish the packet. Panics if a fixed-length packet's size disagrees
    /// with the client's table — that is always a server bug, and sending it
    /// would desync the client's stream.
    pub fn finish(&mut self) -> Vec<u8> {
        match server_len(self.id) {
            Some(PacketLen::Fixed(n)) => assert_eq!(
                self.buf.len(),
                n as usize,
                "packet {:#06x}: built {} bytes, client expects {}",
                self.id,
                self.buf.len(),
                n
            ),
            Some(PacketLen::Variable) => {
                let n = self.buf.len() as u16;
                self.buf[2..4].copy_from_slice(&n.to_le_bytes());
            }
            None => panic!("packet {:#06x} is not in the client's table", self.id),
        }
        std::mem::take(&mut self.buf)
    }
}

/// Little-endian reader over one framed packet.
pub struct Reader<'a> {
    id: u16,
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(pkt: &'a [u8]) -> Self {
        let id = u16::from_le_bytes([pkt[0], pkt[1]]);
        Reader { id, buf: pkt, pos: 2 }
    }
    pub fn id(&self) -> u16 {
        self.id
    }
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    pub fn seek(&mut self, at: usize) -> &mut Self {
        self.pos = at;
        self
    }
    pub fn remaining(&self) -> usize {
        self.buf.len().saturating_sub(self.pos)
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8], ProtoError> {
        if self.pos + n > self.buf.len() {
            return Err(ProtoError::Short { id: self.id, at: self.pos, want: n, len: self.buf.len() });
        }
        let s = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }
    pub fn u8(&mut self) -> Result<u8, ProtoError> {
        Ok(self.take(1)?[0])
    }
    pub fn u16(&mut self) -> Result<u16, ProtoError> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }
    pub fn u32(&mut self) -> Result<u32, ProtoError> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    pub fn bytes(&mut self, n: usize) -> Result<&'a [u8], ProtoError> {
        self.take(n)
    }
    pub fn rest(&mut self) -> &'a [u8] {
        let s = &self.buf[self.pos.min(self.buf.len())..];
        self.pos = self.buf.len();
        s
    }
    /// Fixed-size string field, cut at the first NUL.
    pub fn str_fixed(&mut self, size: usize) -> Result<String, ProtoError> {
        let raw = self.take(size)?;
        let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
        Ok(decode_text(&raw[..end]))
    }
    /// Rest of the packet as a string, cut at the first NUL.
    pub fn str_rest(&mut self) -> String {
        let raw = self.rest();
        let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
        decode_text(&raw[..end])
    }
    pub fn pos3(&mut self) -> Result<(u16, u16, u8), ProtoError> {
        let b = self.take(3)?;
        Ok(unpack_pos3(b.try_into().unwrap()))
    }
}

/// Text on the wire. `servicetype germany` makes this client a Western
/// (Windows-1252) client; Korean resource names only ever travel as GRF
/// paths, not as chat/NPC text.
pub fn encode_text(s: &str) -> Vec<u8> {
    encoding_rs::WINDOWS_1252.encode(s).0.into_owned()
}
pub fn decode_text(b: &[u8]) -> String {
    encoding_rs::WINDOWS_1252.decode(b).0.into_owned()
}

pub fn pack_pos3(x: u16, y: u16, dir: u8) -> [u8; 3] {
    [(x >> 2) as u8, ((x << 6) as u8) | (((y >> 4) & 0x3f) as u8), ((y << 4) as u8) | (dir & 0x0f)]
}

pub fn unpack_pos3(b: [u8; 3]) -> (u16, u16, u8) {
    let x = ((b[0] as u16) << 2) | ((b[1] as u16) >> 6);
    let y = (((b[1] as u16) & 0x3f) << 4) | ((b[2] as u16) >> 4);
    (x, y, b[2] & 0x0f)
}

pub fn pack_move6(x0: u16, y0: u16, x1: u16, y1: u16) -> [u8; 6] {
    [
        (x0 >> 2) as u8,
        ((x0 << 6) as u8) | (((y0 >> 4) & 0x3f) as u8),
        ((y0 << 4) as u8) | (((x1 >> 6) & 0x0f) as u8),
        ((x1 << 2) as u8) | (((y1 >> 8) & 0x03) as u8),
        y1 as u8,
        (8 << 4) | 8,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos3_roundtrip() {
        for &(x, y, d) in &[(0, 0, 0), (156, 191, 4), (399, 399, 7), (1023, 1023, 15)] {
            assert_eq!(unpack_pos3(pack_pos3(x, y, d)), (x, y, d));
        }
    }

    #[test]
    fn framer_splits_fixed_and_variable() {
        let mut f = Framer::new();
        // 0x007d (2 bytes) + 0x008c global chat (variable) + half of 0x0085 (5)
        f.push(&[0x7d, 0x00, 0x8c, 0x00, 0x07, 0x00, b'h', b'i', 0, 0x85, 0x00, 1]);
        assert_eq!(f.next_packet().unwrap().unwrap(), vec![0x7d, 0]);
        assert_eq!(f.next_packet().unwrap().unwrap().len(), 7);
        assert!(f.next_packet().unwrap().is_none());
        f.push(&[2, 3]);
        assert_eq!(f.next_packet().unwrap().unwrap().len(), 5);
    }

    #[test]
    #[should_panic(expected = "client expects")]
    fn writer_rejects_wrong_fixed_length() {
        Writer::new(0x007f).u32(1).u8(0).finish();
    }

    #[test]
    fn writer_patches_variable_length() {
        let p = Writer::new(0x008e).str_nul("hello").finish();
        assert_eq!(u16::from_le_bytes([p[2], p[3]]) as usize, p.len());
    }
}
