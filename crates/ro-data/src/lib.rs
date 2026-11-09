//! Client file formats and the server-side game database.
//!
//! * [`gat`]: walkability grids from the client's `.gat` files (confirmed:
//!   the client walks on exactly these cells).
//! * [`tables`]: the client's `#`-separated text tables (item names, map names).
//! * [`db`]: server-only game data (items, monsters, jobs, EXP) in TOML, every
//!   table tagged with a provenance [`db::Tier`] so reconstructed values are
//!   never mistaken for confirmed ones.

pub mod db;
pub mod gat;
pub mod tables;

pub use db::{Database, Tier};
pub use gat::MapGrid;
