//! Stable snake_case words for the simulation's enums. Every match lists
//! every variant, so a new one fails to compile here until it is named. A
//! word for a type that an event holds is written once, in `sim-core`'s
//! `field_value.rs`, and the function here calls it (decision 0235, ruling 4).

use sim_core::events::Weapon;
use sim_core::map::{HexKind, Terrain};
use sim_core::orders::{AirMission, Issuer, Mission, SpeedSetting, Stance};
use sim_core::side_events::SideEndReason;
use sim_core::state::{ShipSystem, Side, SystemState, TaskForceStatus};

pub fn side(side: Side) -> &'static str {
    sim_core::field_value::side_word(side)
}

pub fn hex_kind(kind: HexKind) -> &'static str {
    match kind {
        HexKind::DeepSea => "deep_sea",
        HexKind::ShallowSea => "shallow_sea",
        HexKind::Coast => "coast",
        HexKind::Land => "land",
    }
}

pub fn terrain(terrain: Terrain) -> &'static str {
    match terrain {
        Terrain::Clear => "clear",
        Terrain::Jungle => "jungle",
        Terrain::Mountain => "mountain",
        Terrain::Atoll => "atoll",
    }
}

pub fn mission(mission: Mission) -> &'static str {
    match mission {
        Mission::Transport => "transport",
        Mission::SurfaceCombat => "surface_combat",
        Mission::Carrier => "carrier",
        Mission::Bombardment => "bombardment",
        Mission::Escort => "escort",
    }
}

/// A speed setting as text: "cruise", "full", or the knots with their unit,
/// such as "24 kn".
pub fn speed(speed: SpeedSetting) -> String {
    match speed {
        SpeedSetting::Cruise => "cruise".to_string(),
        SpeedSetting::Full => "full".to_string(),
        SpeedSetting::Knots(knots) => format!("{knots} kn"),
    }
}

pub fn status(status: TaskForceStatus) -> &'static str {
    match status {
        TaskForceStatus::Idle => "idle",
        TaskForceStatus::Underway => "underway",
        TaskForceStatus::Arrived => "arrived",
        TaskForceStatus::OutOfFuel => "out_of_fuel",
        TaskForceStatus::Halted => "halted",
    }
}

pub fn air_mission(mission: AirMission) -> &'static str {
    match mission {
        AirMission::Rest => "rest",
        AirMission::Search => "search",
        AirMission::NavalStrike => "naval_strike",
        AirMission::CombatAirPatrol => "combat_air_patrol",
        AirMission::BaseStrike(_) => "base_strike",
        AirMission::Reconnaissance(_) => "reconnaissance",
    }
}

pub fn stance(stance: Stance) -> &'static str {
    match stance {
        Stance::Engage => "engage",
        Stance::Avoid => "avoid",
    }
}

pub fn issuer(issuer: Issuer) -> &'static str {
    match issuer {
        Issuer::Player => "player",
        Issuer::Staff => "staff",
        Issuer::LocalCommander => "local_commander",
    }
}

pub fn weapon(weapon: Weapon) -> &'static str {
    sim_core::field_value::weapon_word(weapon)
}

/// A fight's ending as a side is told it (decision 0085).
pub fn side_end_reason(reason: SideEndReason) -> &'static str {
    match reason {
        SideEndReason::SlippedAway => "slipped_away",
        SideEndReason::ForceSunk => "force_sunk",
        SideEndReason::BrokeOff => "broke_off",
        SideEndReason::PassedUnseen => "passed_unseen",
        SideEndReason::MaxRounds => "max_rounds",
        SideEndReason::ContactLost => "contact_lost",
    }
}

pub fn system(system: ShipSystem) -> &'static str {
    sim_core::field_value::system_word(system)
}

pub fn system_state(state: SystemState) -> &'static str {
    sim_core::field_value::system_state_word(state)
}
