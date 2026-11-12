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
//!   `id,name,role,search_range_nm,sweep_width_nm,strike_range_nm,attack_weapon,attack_damage,pilot_armour,self_sealing_tanks`,
//!   one type per row. The roles are `patrol` and `bomber`; the attack weapon
//!   is `none`, `bomb` or `torpedo` (decision 0026); the last two columns are
//!   `0` or `1` and required (decision 0029). Lines starting with `#`
//!   are comments.
//! - Search figures: TOML. A `placeholder` flag, the report and contact figures, and a `[kind_confusion.<kind>]` table for each of the six
//!   ship kinds giving the chance of each reported kind.
//! - Surface combat figures: TOML. A `placeholder` flag and the figures of
//!   the surface combat rule (`docs/decisions/0014-surface-combat.md`).
//! - Air combat figures: TOML, named by the scenario's optional `air_combat`
//!   key (`docs/decisions/0026-air-strikes.md`). Without it no air group can be
//!   ordered to strike.
//! - Air upkeep figures: TOML, named by the scenario's optional `air_upkeep`
//!   key (`docs/decisions/0035-wear-repair-and-operational-losses.md`). Without
//!   it no aircraft is worn or repaired.
//! - Fatigue figures: TOML, named by the scenario's optional `fatigue` key
//!   (`docs/decisions/0036-fatigue-and-rest.md`). Without it no air group
//!   tires or rests.
//! - Base supply figures: TOML, named by the scenario's optional `air_supply`
//!   key (`docs/decisions/0039-base-supply-consumption.md`). Without it
//!   nothing is spent and no base is out of supply.
//! - Maps: TOML, as rectangular regions plus single-hex overrides, or a
//!   generated CSV with the columns `q,r,kind,land_percent,lat,lon` (decision
//!   0017). The extension decides which.
//! - Scenarios: TOML, with the starting state, the seed and the scripted
//!   orders per day. A scenario file is therefore also a saved game. It names
//!   its map and reference files, and may hold bases, air groups, task
//!   forces, ships and orders. `ship_classes` and `aircraft_types` may each
//!   name one file or a list of files; an id in two files is an error naming
//!   both (decision 0043). A task force may say `readiness = "battle"`
//!   (the default is `"normal"`), and a
//!   ship may give its `gun_rounds`. An order is addressed to exactly one of a
//!   `task_force` or an `air_group`.
//!
//!   A scenario may also have an `[objectives]` table: the three levels, an
//!   optional `history` note, and for each side a `mission` and a list of
//!   `[[objectives.<side>.objective]]` of the kinds `ships_sunk`,
//!   `ships_crippled`, `aircraft_destroyed`, `cargo_landed` and `base_open`
//!   (decision 0054, "As built").
//!
//! A scenario names its reference files by path. A caller may read them from
//! another directory instead with [`LoadOptions`] and `Scenario::load_with`:
//! each reference file the scenario names is then looked up by its file name
//! under `LoadOptions::reference_root`, and one that is missing there is an
//! error that names it. The map is read where the scenario names it. The
//! hand-calculation tests use `data/reference/step1`, the figures frozen at
//! the end of step 1 (`LoadOptions::frozen_step1`); the run baselines and
//! everything else read the live files. The files a scenario records as its
//! inputs (`Scenario::data_files`) are the ones actually read.
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
    load_air_combat_figures, load_air_supply_figures, load_air_upkeep_figures, load_aircraft_types,
    load_fatigue_figures, load_port_sizes, load_search_figures, load_ship_classes,
    load_surface_combat_figures,
};
pub use scenario::{LoadOptions, Scenario, FROZEN_REFERENCE_DIR};
