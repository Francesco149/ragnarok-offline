//! The client's `#`-separated text tables (`data\*.txt` in the GRF).
//! Format: `key#value#` records, `//` comments; values may span lines.

use std::collections::HashMap;

/// Decode client text. These English tables are effectively Windows-1252.
pub fn decode(b: &[u8]) -> String {
    encoding_rs::WINDOWS_1252.decode(b).0.into_owned()
}

/// Split a `#` table into fields (comments and blank lines dropped).
pub fn fields(b: &[u8]) -> Vec<String> {
    let text = decode(b);
    let cleaned: String = text
        .lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    cleaned.split('#').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
}

/// `id#name#` pairs, e.g. idnum2itemdisplaynametable.txt. Underscores in
/// display names stand for spaces.
pub fn id_names(b: &[u8]) -> HashMap<u32, String> {
    let f = fields(b);
    let mut out = HashMap::new();
    for pair in f.chunks(2) {
        if let [k, v] = pair {
            if let Ok(id) = k.parse::<u32>() {
                out.insert(id, v.replace('_', " "));
            }
        }
    }
    out
}

/// `map.rsw#Display Name#` pairs from mapnametable.txt, keyed by map name
/// without extension.
pub fn map_names(b: &[u8]) -> HashMap<String, String> {
    let f = fields(b);
    let mut out = HashMap::new();
    for pair in f.chunks(2) {
        if let [k, v] = pair {
            let key = k.trim_end_matches(".rsw").trim_end_matches(".gat").to_ascii_lowercase();
            out.insert(key, v.clone());
        }
    }
    out
}
