use crate::quantity::Quantity;
use serde::Deserialize;

use super::orders::SideSpec;

/// A supply line: tons of supply and fuel a day that appear at a base from
/// off the map, on the days from `first_day` to `last_day` (decision 0196).
/// Its side may differ from the base's side at the start.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SupplyLineSpec {
    pub(super) base: u32,
    /// Tons of supply a day. Defaults to 0.
    pub(super) supply_t: Option<Quantity>,
    /// Tons of fuel a day. Defaults to 0.
    pub(super) fuel_t: Option<Quantity>,
    /// Defaults to 1.
    pub(super) first_day: Option<u32>,
    /// Defaults to the game's last day.
    pub(super) last_day: Option<u32>,
    /// Defaults to the base's side at the start.
    pub(super) side: Option<SideSpec>,
}

/// A stream of replacement men: so many men a day who join the land units
/// of its side ashore at a base that are below their establishment
/// (decision 0197).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MenReplacementSpec {
    pub(super) base: u32,
    /// Men a day, a whole number above 0.
    pub(super) men: u32,
    /// Defaults to 1; nothing is given on day 1.
    pub(super) first_day: Option<u32>,
    /// Defaults to the game's last day.
    pub(super) last_day: Option<u32>,
    /// Defaults to the base's side at the start.
    pub(super) side: Option<SideSpec>,
}
