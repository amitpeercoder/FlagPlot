use serde::Deserialize;

use super::orders::SideSpec;

/// A submarine (decision 0218): a unit of her own kind with a station.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SubmarineSpec {
    pub(super) id: u32,
    pub(super) name: String,
    pub(super) side: SideSpec,
    pub(super) class: u32,
    /// The hex she starts in.
    pub(super) q: i32,
    pub(super) r: i32,
    /// Her station. Without it, her hex. A station that is not her hex
    /// means she starts on passage to it.
    pub(super) station_q: Option<i32>,
    pub(super) station_r: Option<i32>,
    /// The base she goes home to: required.
    pub(super) home: u32,
    /// The day she leaves for home (rule 3): a whole number from 1.
    pub(super) leave_day: Option<u32>,
    /// The day she enters the game (decision 0218, 33): a whole number from
    /// 2 up to the scenario's `days`. Until then she is held apart, in no
    /// hex for any rule.
    pub(super) enters: Option<u32>,
    /// The day from which her side may use her: a whole number from 1.
    pub(super) available_from: Option<u32>,
    /// Salvoes at the start: from 0, at most her class's. Without it, her
    /// class's.
    pub(super) salvoes: Option<u32>,
    /// `sound` or `damaged`. A damaged boat starts going home. Without it,
    /// sound.
    pub(super) condition: Option<String>,
    /// Pulses she starts held down, counted from the first, the night:
    /// 0 for none. Without it, 0.
    pub(super) held_down_pulses: Option<u32>,
}
