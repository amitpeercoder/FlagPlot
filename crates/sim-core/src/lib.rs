//! Flag Plot simulation core.
//!
//! This crate holds the state, the rules, the daily resolution and the event
//! log. It has no dependencies and performs no file or network access.
//!
//! The rules that keep it deterministic are in `CLAUDE.md` at the repo root.
//! In short: fixed-point arithmetic only, ordered collections only, no wall
//! clock, and all randomness through [`rng::RngKey`].

#![deny(clippy::float_arithmetic)]

pub mod bearing;
pub mod belief;
pub mod events;
pub mod fixed;
pub mod hash;
pub mod hex;
pub mod ids;
pub mod map;
pub mod orders;
pub mod refdata;
pub mod rng;
pub mod state;
pub mod time;
pub mod view;
pub mod world;

mod airstrike;
mod cargo;
mod damage;
mod invariants;
mod movement;
mod resolve;
mod search;
mod start;
mod stragglers;
mod surface;

pub use invariants::InvariantViolation;
pub use start::StartingContact;
pub use world::{DaySummary, SimConfig, SimError, World};
