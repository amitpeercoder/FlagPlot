//! Flag Plot data loading.
//!
//! Reads reference data, maps and scenarios from text files and turns them
//! into `sim-core` types, checking every cross-reference on the way. All file
//! access for the simulation lives here. `sim-core` never touches a file.
//!
//! File formats:
//!
//! - Ship classes: CSV, one class per row. Lines starting with `#` are
//!   comments.
//! - Maps: TOML, as rectangular regions plus single-hex overrides.
//! - Scenarios: TOML, with the starting state, the seed and the scripted
//!   orders per day. A scenario file is therefore also a saved game.
//!
//! Quantities are written as whole numbers or as decimal strings such as
//! `"0.25"`. TOML floats are rejected, so no value passes through floating
//! point on its way into the simulation.

mod error;
mod map;
mod reference;
mod scenario;

pub use error::DataError;
pub use map::load_map;
pub use reference::load_ship_classes;
pub use scenario::Scenario;
