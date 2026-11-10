//! Flag Plot data loading.
//!
//! Reads reference data, maps and scenarios from text files and turns them
//! into `sim-core` types, checking every cross-reference on the way. All file
//! access for the simulation lives here. `sim-core` never touches a file.
//!
//! File formats:
//!
//! - Ship classes: CSV, one class per row, with the weapons, toughness and
//!   radar columns of surface combat (counts are whole numbers; rate of fire,
//!   damage and toughness may be whole numbers or decimals; radar 0 or 1). Lines
//!   starting with `#` are comments.
//! - Port sizes: CSV with the header
//!   `port_size,berths,berth_rate_t_per_h,lighter_rate_t_per_h`, one row for
//!   each size 0 to 10. Lines starting with `#` are comments.
//! - Aircraft types: CSV with the header
//!   `id,name,role,search_range_nm,sweep_width_nm`, one type per row. The only
//!   role so far is `patrol`. Lines starting with `#` are comments.
//! - Search figures: TOML. A `placeholder` flag, the report, contact and
//!   notice figures, and a `[kind_confusion.<kind>]` table for each of the six
//!   ship kinds giving the chance of each reported kind.
//! - Surface combat figures: TOML. A `placeholder` flag and the figures of
//!   the surface combat rule (`docs/decisions/0014-surface-combat.md`).
//! - Maps: TOML, as rectangular regions plus single-hex overrides, or a
//!   generated CSV with the columns `q,r,kind,land_percent,lat,lon` (decision
//!   0017). The extension decides which.
//! - Scenarios: TOML, with the starting state, the seed and the scripted
//!   orders per day. A scenario file is therefore also a saved game. It names
//!   its map and reference files, and may hold bases, air groups, task
//!   forces, ships and orders. A task force may say `alert = false`, and a
//!   ship may give its `gun_rounds`. An order is addressed to exactly one of a
//!   `task_force` or an `air_group`.
//!
//! Quantities are written as whole numbers or as decimal strings such as
//! `"0.25"`. TOML floats are rejected, so no value passes through floating
//! point on its way into the simulation.

mod error;
mod map;
mod quantity;
mod reference;
mod scenario;

pub use error::DataError;
pub use map::load_map;
pub use reference::{
    load_aircraft_types, load_port_sizes, load_search_figures, load_ship_classes,
    load_surface_combat_figures,
};
pub use scenario::Scenario;
