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
pub mod believed;
pub mod choices;
pub mod claims;
pub mod command;
pub mod commander;
pub mod condition;
pub mod entering;
#[cfg(test)]
mod entering_tests;
pub mod events;
pub mod fixed;
pub mod hash;
pub mod held;
pub mod hex;
pub mod ids;
pub mod map;
pub mod objectives;
pub mod orders;
pub mod pictures;
pub mod rearm;
pub mod refdata;
pub mod repair;
mod replacement_men;
#[cfg(test)]
mod replacement_men_tests;
pub mod rng;
pub mod side_events;
pub mod state;
#[cfg(test)]
mod supply_lines_tests;
pub mod time;
pub mod view;
pub mod world;
pub mod world_picture;

mod airstrike;
mod available;
mod bombardment;
mod cargo;
mod cover;
mod damage;
mod escort;
mod estimate;
#[cfg(test)]
mod falling_back_tests;
mod forming;
mod fuel_at_sea;
mod ground;
mod ground_combat;
#[cfg(test)]
mod held_tests;
mod invariants;
mod landing;
mod march;
mod movement;
mod passing;
#[cfg(test)]
mod passing_tests;
#[cfg(test)]
mod pictures_tests;
mod preparation;
#[cfg(test)]
mod preparation_tests;
mod rebase;
mod reconnaissance;
mod refuel;
mod resolve;
mod search;
mod start;
mod stragglers;
mod submarine;
pub mod submarine_estimate;
#[cfg(test)]
mod submarine_tests;
mod surface;
mod suspected_waters;
mod upkeep;
pub mod works;

pub use airstrike::{NavalStrike, StrikeChoice, StrikeSizing};
pub use command::{Proposal, Staff, StaffCommand, StaffDecisions, Staffs};
pub use estimate::StartingEstimate;
pub use invariants::InvariantViolation;
pub use movement::{PassageEnd, PassageForecast, ShipPassage};
pub use start::StartingContact;
pub use view::DaysOfSupply;
pub use world::{DayInHand, DayPart, DaySummary, SimConfig, SimError, World};
