//! `.gat` ("GRAT") cell files: `u8[4] "GRAT", u8 major, u8 minor, u32 w,
//! u32 h`, then `w*h` cells of `f32[4] heights, u32 type`, row-major from
//! (0,0) bottom-left. Cell types as the Athena family maps them:
//! 0 walk, 1 wall, 2 walk(?), 3 walk+water, 4 walk(?), 5 cliff (shoot-only),
//! 6 walk(?).

use anyhow::{Result, bail};

pub const WALK: u8 = 1;
pub const SHOOT: u8 = 2;
pub const WATER: u8 = 4;

#[derive(Clone)]
pub struct MapGrid {
    pub name: String,
    pub width: u16,
    pub height: u16,
    cells: Vec<u8>,
}

impl MapGrid {
    pub fn from_gat(name: &str, b: &[u8]) -> Result<Self> {
        if b.len() < 14 || &b[..4] != b"GRAT" {
            bail!("{name}: not a GAT file");
        }
        let w = u32::from_le_bytes(b[6..10].try_into()?) as usize;
        let h = u32::from_le_bytes(b[10..14].try_into()?) as usize;
        if w == 0 || h == 0 || w > 1024 || h > 1024 || b.len() < 14 + w * h * 20 {
            bail!("{name}: bad GAT size {w}x{h} ({} bytes)", b.len());
        }
        let mut cells = Vec::with_capacity(w * h);
        for i in 0..w * h {
            let off = 14 + i * 20 + 16;
            let t = u32::from_le_bytes(b[off..off + 4].try_into()?) & 0xff;
            cells.push(match t {
                0 | 2 | 4 | 6 => WALK | SHOOT,
                1 => 0,
                3 => WALK | SHOOT | WATER,
                5 => SHOOT,
                _ => 0,
            });
        }
        Ok(MapGrid { name: name.to_string(), width: w as u16, height: h as u16, cells })
    }

    /// A blank all-walkable grid (tests).
    pub fn open(name: &str, width: u16, height: u16) -> Self {
        MapGrid { name: name.into(), width, height, cells: vec![WALK | SHOOT; width as usize * height as usize] }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as u32) < self.width as u32 && (y as u32) < self.height as u32
    }
    pub fn flags(&self, x: i32, y: i32) -> u8 {
        if !self.in_bounds(x, y) {
            return 0;
        }
        self.cells[y as usize * self.width as usize + x as usize]
    }
    pub fn walkable(&self, x: i32, y: i32) -> bool {
        self.flags(x, y) & WALK != 0
    }
    pub fn set_walkable(&mut self, x: i32, y: i32, walk: bool) {
        if self.in_bounds(x, y) {
            let c = &mut self.cells[y as usize * self.width as usize + x as usize];
            if walk {
                *c |= WALK
            } else {
                *c &= !WALK
            }
        }
    }
    /// Set a cell's flags outright (WALK | SHOOT | WATER; 0 = a wall).
    pub fn set_flags(&mut self, x: i32, y: i32, flags: u8) {
        if self.in_bounds(x, y) {
            self.cells[y as usize * self.width as usize + x as usize] = flags;
        }
    }
    pub fn walkable_count(&self) -> usize {
        self.cells.iter().filter(|&&c| c & WALK != 0).count()
    }
}
