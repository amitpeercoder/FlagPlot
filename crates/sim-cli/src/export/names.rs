//! Stable snake_case words for the simulation's enums. Every match lists
//! every variant, so a new one fails to compile here until it is named.

use sim_core::events::{DamageTrack, DetachReason, EngagementEndReason, Weapon};
use sim_core::map::{HexKind, Terrain};
use sim_core::orders::{AirMission, Issuer, Mission, SpeedSetting, Stance};
use sim_core::state::{ShipSystem, Side, SystemState, TaskForceStatus};

pub fn side(side: Side) -> &'static str {
    match side {
        Side::Allied => "allied",
        Side::Japanese => "japanese",
    }
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
    }
}

pub fn air_mission(mission: AirMission) -> &'static str {
    match mission {
        AirMission::Rest => "rest",
        AirMission::Search => "search",
        AirMission::NavalStrike => "naval_strike",
        AirMission::CombatAirPatrol => "combat_air_patrol",
        AirMission::BaseStrike(_) => "base_strike",
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
    match weapon {
        Weapon::Gun => "gun",
        Weapon::Torpedo => "torpedo",
        Weapon::Bomb => "bomb",
    }
}

pub fn track(track: DamageTrack) -> &'static str {
    match track {
        DamageTrack::Flotation => "flotation",
        DamageTrack::Fire => "fire",
    }
}

pub fn end_reason(reason: EngagementEndReason) -> &'static str {
    match reason {
        EngagementEndReason::SlippedAway => "slipped_away",
        EngagementEndReason::ForceSunk => "force_sunk",
        EngagementEndReason::BrokeOff => "broke_off",
        EngagementEndReason::PassedUnseen => "passed_unseen",
        EngagementEndReason::MaxRounds => "max_rounds",
    }
}

pub fn detach_reason(reason: DetachReason) -> &'static str {
    match reason {
        DetachReason::SentHome => "sent_home",
        DetachReason::LeftBehind => "left_behind",
        DetachReason::Ordered => "ordered",
    }
}

pub fn system(system: ShipSystem) -> &'static str {
    match system {
        ShipSystem::Propulsion => "propulsion",
        ShipSystem::FireControl => "fire_control",
        ShipSystem::MainBattery => "main_battery",
        ShipSystem::TorpedoMounts => "torpedo_mounts",
        ShipSystem::FlightDeck => "flight_deck",
    }
}

pub fn system_state(state: SystemState) -> &'static str {
    match state {
        SystemState::Working => "working",
        SystemState::Degraded => "degraded",
        SystemState::Out => "out",
    }
}
