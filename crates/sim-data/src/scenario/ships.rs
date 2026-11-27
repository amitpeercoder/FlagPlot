use crate::quantity::Quantity;
use serde::Deserialize;
use sim_core::state::SystemState;

use super::commanders::SuperiorSpec;
use super::orders::{MissionSpec, ReadinessSpec, SideSpec, StanceSpec};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct TaskForceSpec {
    pub(super) id: u32,
    pub(super) name: String,
    pub(super) side: SideSpec,
    /// Where the task force starts: exactly one of `at_base`, or both `q`
    /// and `r` for a position at sea.
    pub(super) at_base: Option<u32>,
    pub(super) q: Option<i32>,
    pub(super) r: Option<i32>,
    pub(super) mission: MissionSpec,
    /// The task force's readiness at the start (decision 0021): `normal` or
    /// `battle`. Defaults to normal.
    pub(super) readiness: Option<ReadinessSpec>,
    /// Fatigue at the start, 0 to 100 (decision 0037). Defaults to 0.
    pub(super) fatigue: Option<Quantity>,
    /// The base the force heads for when it retires or sends stragglers
    /// home. It must be a base of the force's own side. Defaults to none.
    pub(super) home_port: Option<u32>,
    /// The stance of its rules of engagement at the start, `engage` or
    /// `avoid` (decision 0124's scenarios). Defaults to engage, the rest of
    /// the rules of engagement as they default.
    pub(super) stance: Option<StanceSpec>,
    /// The command this force is placed under, if not its side's area
    /// command (decision 0134, 4). Needs `command_levels = true`.
    pub(super) superior: Option<SuperiorSpec>,
    /// Whether the force keeps radio silence from the start (decision 0136,
    /// 15). Defaults to false. It has an effect only where reports travel.
    pub(super) radio_silence: Option<bool>,
    /// The day from which the side may use it (decision 0140, 1): a whole
    /// number from 1. Without it, from day 1.
    pub(super) available_from: Option<u32>,
    /// The day the unit enters the game (decision 0195, 1): a whole number
    /// from 2 up to the scenario's `days`. Until then it is held apart, in
    /// no hex and in no count. Without it, the unit is there on day 1.
    /// The force enters at its hex, with its ships and all aboard.
    pub(super) enters: Option<u32>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ShipSpec {
    pub(super) id: u32,
    pub(super) name: String,
    pub(super) class: u32,
    pub(super) side: SideSpec,
    pub(super) fuel_t: Quantity,
    /// Supply aboard at the start, in tons. Defaults to 0.
    pub(super) supply_t: Option<Quantity>,
    /// Fuel carried as cargo at the start, in tons (decision 0046). Defaults
    /// to 0. With the supply aboard it must fit the class's cargo capacity.
    pub(super) cargo_fuel_t: Option<Quantity>,
    /// Exactly one of `task_force` and `port` must be given.
    pub(super) task_force: Option<u32>,
    pub(super) port: Option<u32>,
    /// Gun rounds aboard at the start. Defaults to the guns times the
    /// rounds each carries.
    pub(super) gun_rounds: Option<Quantity>,
    /// Torpedo tubes that have fired at the start, the highest numbered
    /// (decision 0122). Defaults to 0; at most the class's tubes. With every
    /// tube fired the ship has no torpedo ready.
    pub(super) tubes_fired: Option<u32>,
    /// The crew's experience, a whole number from 0 to 4 (decision 0064).
    /// Defaults to 2. Set here and written by no rule.
    pub(super) experience: Option<i64>,
    /// Flotation damage at the start, from 0 to below 100 (decision 0069): a
    /// ship sunk is not given this way. Defaults to 0.
    pub(super) flotation_damage: Option<Quantity>,
    /// Fire damage at the start, from 0 to 100 (decision 0069). Defaults to 0.
    pub(super) fire_damage: Option<Quantity>,
    /// The state of each named system at the start. Each defaults to
    /// working, and anything else needs a class that has the system.
    pub(super) propulsion: Option<SystemStateSpec>,
    pub(super) fire_control: Option<SystemStateSpec>,
    pub(super) main_battery: Option<SystemStateSpec>,
    pub(super) torpedo_mounts: Option<SystemStateSpec>,
    pub(super) flight_deck: Option<SystemStateSpec>,
    /// The day from which the side may use it (decision 0140, 1): a whole
    /// number from 1. Without it, from day 1.
    pub(super) available_from: Option<u32>,
    /// The day the unit enters the game (decision 0195, 1): a whole number
    /// from 2 up to the scenario's `days`. Until then it is held apart, in
    /// no hex and in no count. Without it, the unit is there on day 1.
    /// Refused: a ship enters with its force (decision 0195, 1).
    pub(super) enters: Option<u32>,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub(super) enum SystemStateSpec {
    Working,
    Degraded,
    Out,
}

impl From<SystemStateSpec> for SystemState {
    fn from(spec: SystemStateSpec) -> SystemState {
        match spec {
            SystemStateSpec::Working => SystemState::Working,
            SystemStateSpec::Degraded => SystemState::Degraded,
            SystemStateSpec::Out => SystemState::Out,
        }
    }
}
