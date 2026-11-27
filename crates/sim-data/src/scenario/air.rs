use crate::quantity::Quantity;
use serde::Deserialize;

use super::orders::SideSpec;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct AirGroupSpec {
    pub(super) id: u32,
    pub(super) name: String,
    pub(super) side: SideSpec,
    /// Exactly one of `base` and `ship` must be given.
    pub(super) base: Option<u32>,
    pub(super) ship: Option<u32>,
    pub(super) aircraft_type: u32,
    /// Aircraft ready to fly at the start.
    pub(super) ready: u32,
    /// Damaged aircraft at the start (decision 0035). Defaults to 0.
    pub(super) damaged: Option<u32>,
    /// Fatigue at the start, 0 to 100 (decision 0036). Defaults to 0.
    pub(super) fatigue: Option<Quantity>,
    /// Pilots at the start (decision 0038). Without it the group's pilots are
    /// not tracked and it flies as before.
    pub(super) pilots: Option<u32>,
    /// The ceiling for replacements, in aircraft and in pilots (decision 10
    /// of 0038). Defaults to the aircraft the group starts with, ready and
    /// damaged.
    pub(super) strength: Option<u32>,
    /// The oldest contact, in pulses, the group launches a naval strike on
    /// at the start (decision 0114). A whole number from 0 up to one less
    /// than the search figures' contact drop age. Without it the group uses
    /// the air combat figure. Signed so that a negative number is refused
    /// here with a message that names the range.
    pub(super) strike_contact_age_pulses: Option<i64>,
    /// The share of its aircraft the group holds back from a strike at the
    /// start, 0 to 1 (decision 0114). Defaults to 0.
    pub(super) strike_held_share: Option<Quantity>,
    /// The day from which the side may use it (decision 0140, 1): a whole
    /// number from 1. Without it, from day 1.
    pub(super) available_from: Option<u32>,
    /// The day the unit enters the game (decision 0195, 1): a whole number
    /// from 2 up to the scenario's `days`. Until then it is held apart, in
    /// no hex and in no count. Without it, the unit is there on day 1.
    /// At a base; a group aboard a ship enters with its ship's force.
    pub(super) enters: Option<u32>,
}

/// A stream of replacements for a side at a base (decision 6 of 0038).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ReplacementSpec {
    pub(super) side: SideSpec,
    pub(super) base: u32,
    pub(super) aircraft_type: u32,
    pub(super) aircraft_per_day: u32,
    pub(super) pilots_per_day: u32,
}

/// Aircraft and pilots flown in to an air group on a day (decision 0041).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ArrivalSpec {
    pub(super) day: u32,
    pub(super) air_group: u32,
    pub(super) aircraft: u32,
    /// Defaults to 0. Only for a group whose pilots are tracked.
    pub(super) pilots: Option<u32>,
}
