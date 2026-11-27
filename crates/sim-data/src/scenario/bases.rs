use crate::quantity::Quantity;
use serde::Deserialize;

use super::commanders::SuperiorSpec;
use super::orders::SideSpec;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct BaseSpec {
    pub(super) id: u32,
    pub(super) name: String,
    pub(super) side: SideSpec,
    pub(super) q: i32,
    pub(super) r: i32,
    /// Supply stock at the start, in tons. Defaults to 0.
    pub(super) supply_t: Option<Quantity>,
    /// Fuel stock for ships at the start, in tons (decision 0045). Defaults
    /// to 0. May not be negative.
    pub(super) fuel_t: Option<Quantity>,
    /// The supply the staff's own loads leave in the base (decision 0111).
    /// Defaults to 0. May not be negative.
    pub(super) supply_reserve_t: Option<Quantity>,
    /// The fuel refuelling leaves in the base (decision 0111). Defaults to
    /// 0. May not be negative.
    pub(super) fuel_reserve_t: Option<Quantity>,
    /// Ammunition for ships at the start, in tons (decision 0122). Defaults
    /// to 0. May not be negative.
    pub(super) ammunition_t: Option<Quantity>,
    /// Anti-aircraft shots at a raid on the base (decision 0034). Defaults
    /// to 0.
    pub(super) anti_aircraft: Option<u32>,
    /// Damaged aircraft the base repairs each day (decision 0035). Defaults
    /// to 0.
    pub(super) aircraft_repairs_per_day: Option<u32>,
    /// The points of ship damage the base mends in a day, shared by the
    /// forces of its side stopped there (decision 0069). Defaults to 0. May
    /// not be negative.
    pub(super) ship_repair_points_per_day: Option<Quantity>,
    /// Makes the base a port for the yard, with a passage there and back of
    /// this many days (decision 0069). A whole number, at least 1. Defaults
    /// to none.
    pub(super) yard_passage_days: Option<i64>,
    /// Whether the base is on the enemy's map at the start (decision 0092,
    /// "As built"). Defaults to true. A base off it cannot be bombarded or
    /// struck until a ground report finds it.
    pub(super) on_map: Option<bool>,
    /// Whether the base has an airfield (decision 5 of 0099). Defaults to
    /// true.
    pub(super) airfield: Option<bool>,
    /// The work already done on the airfield, 0 or more (decision 5 of
    /// 0099). Only for a base with `airfield = false`. Defaults to 0.
    pub(super) airfield_work: Option<Quantity>,
    /// The base's defences at the start, 0 to 100 (decision 4 of 0099).
    /// Defaults to 0.
    pub(super) defences: Option<Quantity>,
    /// The damage to the airfield at the start, 0 to 100 (decision 0103).
    /// Only for a base with an airfield. Defaults to 0.
    pub(super) airfield_damage: Option<Quantity>,
    /// The command this base is placed under, if not its side's area
    /// command (decision 0134, 4). Needs `command_levels = true`.
    pub(super) superior: Option<SuperiorSpec>,
}
