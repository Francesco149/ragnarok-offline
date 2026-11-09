//! Game simulation for the ragnarok-offline server.
//!
//! One [`Game`] owns everything (accounts, the world, scripts) and is driven
//! by a single thread: network tasks feed it [`NetEvent`]s, a timer calls
//! [`Game::tick`]. No locks, deterministic order.

pub mod game;
pub mod model;
pub mod packets;
pub mod path;
pub mod script;
pub mod store;
pub mod world;

mod cart;
mod combat;
mod host;
mod items;
mod mapsrv;
mod npcrt;
mod skills;
mod status;
mod storage;

pub use game::{Game, GameConfig, NetEvent, Outbox, Resources, ServerKind};
