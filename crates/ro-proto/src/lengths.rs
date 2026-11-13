//! Packet lengths of the 2003-12-18 client.
//!
//! Source of truth: `data/client/packet_lengths.tsv`, extracted from
//! `Ragexe.exe` by `research/extract_packet_lengths.py` (the client's own
//! `std::map<int,int>` initialiser). The client uses that table to frame what
//! it *receives*, so it is authoritative for every server -> client packet.
//!
//! What the client *sends* is built by separate code; a few ids are sent
//! variable-length although the receive table lists a fixed size
//! (`CLIENT_SEND_OVERRIDES`). Read from the exe's send code (docs/PROTOCOL.md:
//! 0x01d5 at 0x513dd0, 0x019c at 0x511e7c); every era server frames them as
//! variable too, and kRO fixed its table on 2004-12-13 (research/notes/emulators.md §4).

use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketLen {
    Fixed(u16),
    Variable,
}

const TABLE_TSV: &str = include_str!("../../../data/client/packet_lengths.tsv");
const MAX_ID: usize = 0x200;

/// (id, how the client actually sends it)
const CLIENT_SEND_OVERRIDES: &[(u16, PacketLen)] = &[
    (0x01d5, PacketLen::Variable), // CZ_INPUT_EDITDLGSTR: u16 len, u32 npc, char[] text
    (0x019c, PacketLen::Variable), // CZ_LOCALBROADCAST:   u16 len, char[] text
];

fn table() -> &'static [Option<PacketLen>; MAX_ID] {
    static T: OnceLock<[Option<PacketLen>; MAX_ID]> = OnceLock::new();
    T.get_or_init(|| {
        let mut t = [None; MAX_ID];
        for line in TABLE_TSV.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut cols = line.split('\t');
            let id = cols.next().and_then(|s| u16::from_str_radix(s.trim_start_matches("0x"), 16).ok());
            let len = cols.next().and_then(|s| s.parse::<i32>().ok());
            match (id, len) {
                (Some(id), Some(-1)) => t[id as usize] = Some(PacketLen::Variable),
                (Some(id), Some(n)) if n >= 2 => t[id as usize] = Some(PacketLen::Fixed(n as u16)),
                _ => panic!("bad packet_lengths.tsv line: {line}"),
            }
        }
        t
    })
}

/// Length of a packet the server sends, per the client's receive table.
pub fn server_len(id: u16) -> Option<PacketLen> {
    table().get(id as usize).copied().flatten()
}

/// Length of a packet as the client sends it.
pub fn client_send_len(id: u16) -> Option<PacketLen> {
    CLIENT_SEND_OVERRIDES
        .iter()
        .find(|(i, _)| *i == id)
        .map(|(_, l)| *l)
        .or_else(|| server_len(id))
}

/// Number of ids in the table (for diagnostics).
pub fn known_ids() -> usize {
    table().iter().filter(|e| e.is_some()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_loads_and_has_landmarks() {
        assert_eq!(known_ids(), 386);
        assert_eq!(server_len(0x0064), Some(PacketLen::Fixed(55)));
        assert_eq!(server_len(0x0069), Some(PacketLen::Variable));
        assert_eq!(server_len(0x0072), Some(PacketLen::Fixed(19)));
        assert_eq!(server_len(0x0085), Some(PacketLen::Fixed(5)));
        assert_eq!(server_len(0x0078), Some(PacketLen::Fixed(52)));
        assert_eq!(client_send_len(0x01d5), Some(PacketLen::Variable));
        assert_eq!(server_len(0x0200), None);
    }
}
