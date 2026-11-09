//! Gravity's GRF "DES": standard DES tables, all-zero key, ONE round, no half
//! swap. Because the single round only XORs f(R) into L, the operation is its
//! own inverse — the same function encrypts and decrypts.
//!
//! The four combined S-box tables are generated from the standard DES S-boxes
//! (high nibble S(2i+1), low nibble S(2i+2)), matching the tables used by
//! eAthena/rAthena `des.c` (e.g. s_table[0] starts ef 03 41 fd d8 74 1e 47).

const MASK: [u8; 8] = [0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01];

const IP_TABLE: [u8; 64] = [
    58, 50, 42, 34, 26, 18, 10, 2, 60, 52, 44, 36, 28, 20, 12, 4, 62, 54, 46, 38, 30, 22, 14, 6,
    64, 56, 48, 40, 32, 24, 16, 8, 57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43, 35, 27, 19, 11, 3,
    61, 53, 45, 37, 29, 21, 13, 5, 63, 55, 47, 39, 31, 23, 15, 7,
];

const FP_TABLE: [u8; 64] = [
    40, 8, 48, 16, 56, 24, 64, 32, 39, 7, 47, 15, 55, 23, 63, 31, 38, 6, 46, 14, 54, 22, 62, 30,
    37, 5, 45, 13, 53, 21, 61, 29, 36, 4, 44, 12, 52, 20, 60, 28, 35, 3, 43, 11, 51, 19, 59, 27,
    34, 2, 42, 10, 50, 18, 58, 26, 33, 1, 41, 9, 49, 17, 57, 25,
];

const P_TABLE: [u8; 32] = [
    16, 7, 20, 21, 29, 12, 28, 17, 1, 15, 23, 26, 5, 18, 31, 10, 2, 8, 24, 14, 32, 27, 3, 9, 19,
    13, 30, 6, 22, 11, 4, 25,
];

#[rustfmt::skip]
const SBOX: [[[u8; 16]; 4]; 8] = [
    [[14,4,13,1,2,15,11,8,3,10,6,12,5,9,0,7],[0,15,7,4,14,2,13,1,10,6,12,11,9,5,3,8],
     [4,1,14,8,13,6,2,11,15,12,9,7,3,10,5,0],[15,12,8,2,4,9,1,7,5,11,3,14,10,0,6,13]],
    [[15,1,8,14,6,11,3,4,9,7,2,13,12,0,5,10],[3,13,4,7,15,2,8,14,12,0,1,10,6,9,11,5],
     [0,14,7,11,10,4,13,1,5,8,12,6,9,3,2,15],[13,8,10,1,3,15,4,2,11,6,7,12,0,5,14,9]],
    [[10,0,9,14,6,3,15,5,1,13,12,7,11,4,2,8],[13,7,0,9,3,4,6,10,2,8,5,14,12,11,15,1],
     [13,6,4,9,8,15,3,0,11,1,2,12,5,10,14,7],[1,10,13,0,6,9,8,7,4,15,14,3,11,5,2,12]],
    [[7,13,14,3,0,6,9,10,1,2,8,5,11,12,4,15],[13,8,11,5,6,15,0,3,4,7,2,12,1,10,14,9],
     [10,6,9,0,12,11,7,13,15,1,3,14,5,2,8,4],[3,15,0,6,10,1,13,8,9,4,5,11,12,7,2,14]],
    [[2,12,4,1,7,10,11,6,8,5,3,15,13,0,14,9],[14,11,2,12,4,7,13,1,5,0,15,10,3,9,8,6],
     [4,2,1,11,10,13,7,8,15,9,12,5,6,3,0,14],[11,8,12,7,1,14,2,13,6,15,0,9,10,4,5,3]],
    [[12,1,10,15,9,2,6,8,0,13,3,4,14,7,5,11],[10,15,4,2,7,12,9,5,6,1,13,14,0,11,3,8],
     [9,14,15,5,2,8,12,3,7,0,4,10,1,13,11,6],[4,3,2,12,9,5,15,10,11,14,1,7,6,0,8,13]],
    [[4,11,2,14,15,0,8,13,3,12,9,7,5,10,6,1],[13,0,11,7,4,9,1,10,14,3,5,12,2,15,8,6],
     [1,4,11,13,12,3,7,14,10,15,6,8,0,5,9,2],[6,11,13,8,1,4,10,7,9,5,0,15,14,2,3,12]],
    [[13,2,8,4,6,15,11,1,10,9,3,14,5,0,12,7],[1,15,13,8,10,3,7,4,12,5,6,11,0,14,9,2],
     [7,11,4,1,9,12,14,2,0,6,10,13,15,3,5,8],[2,1,14,7,4,10,8,13,15,12,9,0,3,5,6,11]],
];

const fn combined_sboxes() -> [[u8; 64]; 4] {
    let mut out = [[0u8; 64]; 4];
    let mut i = 0;
    while i < 4 {
        let mut x = 0;
        while x < 64 {
            let row = ((x >> 4) & 2) | (x & 1);
            let col = (x >> 1) & 0xf;
            out[i][x] = (SBOX[2 * i][row][col] << 4) | SBOX[2 * i + 1][row][col];
            x += 1;
        }
        i += 1;
    }
    out
}

const S_TABLE: [[u8; 64]; 4] = combined_sboxes();

fn permute(src: &[u8; 8], table: &[u8; 64]) -> [u8; 8] {
    let mut out = [0u8; 8];
    for (i, &t) in table.iter().enumerate() {
        let j = (t - 1) as usize;
        if src[j >> 3] & MASK[j & 7] != 0 {
            out[i >> 3] |= MASK[i & 7];
        }
    }
    out
}

fn round(b: &mut [u8; 8]) {
    // E: right half (b[4..8]) -> eight 6-bit values
    let e = [
        ((b[7] << 5) | (b[4] >> 3)) & 0x3f,
        ((b[4] << 1) | (b[5] >> 7)) & 0x3f,
        ((b[4] << 5) | (b[5] >> 3)) & 0x3f,
        ((b[5] << 1) | (b[6] >> 7)) & 0x3f,
        ((b[5] << 5) | (b[6] >> 3)) & 0x3f,
        ((b[6] << 1) | (b[7] >> 7)) & 0x3f,
        ((b[6] << 5) | (b[7] >> 3)) & 0x3f,
        ((b[7] << 1) | (b[4] >> 7)) & 0x3f,
    ];
    // S-boxes (key is zero, so no XOR) -> 32 bits
    let mut s = [0u8; 4];
    for i in 0..4 {
        s[i] = (S_TABLE[i][e[2 * i] as usize] & 0xf0) | (S_TABLE[i][e[2 * i + 1] as usize] & 0x0f);
    }
    // P permutation, XOR into the left half
    let mut p = [0u8; 4];
    for (i, &t) in P_TABLE.iter().enumerate() {
        let j = (t - 1) as usize;
        if s[j >> 3] & MASK[j & 7] != 0 {
            p[i >> 3] |= MASK[i & 7];
        }
    }
    for i in 0..4 {
        b[i] ^= p[i];
    }
}

/// Decrypt (== encrypt) one 8-byte block in place.
pub fn crypt_block(block: &mut [u8]) {
    let mut b: [u8; 8] = block[..8].try_into().unwrap();
    b = permute(&b, &IP_TABLE);
    round(&mut b);
    b = permute(&b, &FP_TABLE);
    block[..8].copy_from_slice(&b);
}

fn substitution(x: u8) -> u8 {
    match x {
        0x00 => 0x2b,
        0x2b => 0x00,
        0x6c => 0x80,
        0x80 => 0x6c,
        0x01 => 0x68,
        0x68 => 0x01,
        0x48 => 0x77,
        0x77 => 0x48,
        0x60 => 0xff,
        0xff => 0x60,
        0xb9 => 0xc0,
        0xc0 => 0xb9,
        0xfe => 0xeb,
        0xeb => 0xfe,
        other => other,
    }
}

fn shuffle_dec(b: &mut [u8]) {
    let s: [u8; 8] = b[..8].try_into().unwrap();
    b[0] = s[3];
    b[1] = s[4];
    b[2] = s[6];
    b[3] = s[0];
    b[4] = s[1];
    b[5] = s[2];
    b[6] = s[5];
    b[7] = substitution(s[7]);
}

/// "Mixed" encryption (entry flag 0x02): first 20 blocks DES, then every
/// `cycle`-th block DES and every 8th of the others shuffled. `entry_len` is
/// the compressed (unaligned) length, whose decimal digit count picks the cycle.
pub fn decode_full(buf: &mut [u8], entry_len: usize) {
    let mut digits = 1usize;
    let mut i = 10usize;
    while i <= entry_len {
        digits += 1;
        i = i.saturating_mul(10);
    }
    let cycle = if digits < 3 {
        1
    } else if digits < 5 {
        digits + 1
    } else if digits < 7 {
        digits + 9
    } else {
        digits + 15
    };
    let nblocks = buf.len() / 8;
    for i in 0..nblocks.min(20) {
        crypt_block(&mut buf[i * 8..]);
    }
    let mut j = 0;
    for i in 20..nblocks {
        if i % cycle == 0 {
            crypt_block(&mut buf[i * 8..]);
            continue;
        }
        if j == 7 {
            shuffle_dec(&mut buf[i * 8..]);
            j = 0;
        }
        j += 1;
    }
}

/// Header-only encryption (entry flag 0x04, or 0x1xx .gnd/.gat/.act/.str).
pub fn decode_header(buf: &mut [u8]) {
    let nblocks = buf.len() / 8;
    for i in 0..nblocks.min(20) {
        crypt_block(&mut buf[i * 8..]);
    }
}

/// GRF 0x1xx file names: nibble-swap every byte, then DES each 8-byte block.
pub fn decode_filename(buf: &mut [u8]) {
    for chunk in buf.chunks_mut(8) {
        if chunk.len() < 8 {
            break;
        }
        for b in chunk.iter_mut() {
            *b = b.rotate_left(4);
        }
        crypt_block(chunk);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sbox_matches_reference_prefix() {
        assert_eq!(&S_TABLE[0][..8], &[0xef, 0x03, 0x41, 0xfd, 0xd8, 0x74, 0x1e, 0x47]);
    }

    #[test]
    fn round_is_involution() {
        let orig = *b"ragnarok";
        let mut b = orig;
        crypt_block(&mut b);
        assert_ne!(b, orig);
        crypt_block(&mut b);
        assert_eq!(b, orig);
    }
}
