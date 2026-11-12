//! Stable snake_case words for the simulation's enums. Every match lists
//! every variant, so a new one fails to compile here until it is named.

use sim_core::events::{DamageTrack, Weapon};
use sim_core::orders::{AirMission, Mission, SpeedSetting, Stance};
use sim_core::refdata::AircraftRole;
use sim_core::state::{ShipSystem, Side, SystemState, TaskForceStatus};

pub(crate) fn side(side: Side) -> &'static str {
    match side {
        Side::Allied => "allied",
        Side::Japanese => "japanese",
    }
}

pub(crate) fn mission(mission: Mission) -> &'static str {
    match mission {
        Mission::Transport => "transport",
        Mission::SurfaceCombat => "surface_combat",
        Mission::Carrier => "carrier",
        Mission::Bombardment => "bombardment",
        Mission::Escort => "escort",
    }
}

/// "cruise", "full", or the knots with their unit, such as "24 kn".
pub(crate) fn speed(speed: SpeedSetting) -> String {
    match speed {
        SpeedSetting::Cruise => "cruise".to_string(),
        SpeedSetting::Full => "full".to_string(),
        SpeedSetting::Knots(knots) => format!("{knots} kn"),
    }
}

pub(crate) fn status(status: TaskForceStatus) -> &'static str {
    match status {
        TaskForceStatus::Idle => "idle",
        TaskForceStatus::Underway => "underway",
        TaskForceStatus::Arrived => "arrived",
        TaskForceStatus::OutOfFuel => "out_of_fuel",
    }
}

pub(crate) fn air_mission(mission: AirMission) -> &'static str {
    match mission {
        AirMission::Rest => "rest",
        AirMission::Search => "search",
        AirMission::NavalStrike => "naval_strike",
        AirMission::CombatAirPatrol => "combat_air_patrol",
        AirMission::BaseStrike(_) => "base_strike",
    }
}

pub(crate) fn stance(stance: Stance) -> &'static str {
    match stance {
        Stance::Engage => "engage",
        Stance::Avoid => "avoid",
    }
}

pub(crate) fn weapon(weapon: Weapon) -> &'static str {
    match weapon {
        Weapon::Gun => "guns",
        Weapon::Torpedo => "torpedoes",
        Weapon::Bomb => "bombs",
    }
}

pub(crate) fn track(track: DamageTrack) -> &'static str {
    match track {
        DamageTrack::Flotation => "flotation",
        DamageTrack::Fire => "fire",
    }
}

pub(crate) fn system(system: ShipSystem) -> &'static str {
    match system {
        ShipSystem::Propulsion => "propulsion",
        ShipSystem::FireControl => "fire_control",
        ShipSystem::MainBattery => "main_battery",
        ShipSystem::TorpedoMounts => "torpedo_mounts",
        ShipSystem::FlightDeck => "flight_deck",
    }
}

pub(crate) fn system_state(state: SystemState) -> &'static str {
    match state {
        SystemState::Working => "working",
        SystemState::Degraded => "degraded",
        SystemState::Out => "out",
    }
}

pub(crate) fn end_reason(reason: sim_core::events::EngagementEndReason) -> &'static str {
    use sim_core::events::EngagementEndReason as R;
    match reason {
        R::SlippedAway => "slipped_away",
        R::ForceSunk => "force_sunk",
        R::BrokeOff => "broke_off",
        R::PassedUnseen => "passed_unseen",
        R::MaxRounds => "max_rounds",
    }
}

pub(crate) fn role(role: AircraftRole) -> &'static str {
    match role {
        AircraftRole::Patrol => "patrol",
        AircraftRole::Bomber => "bomber",
        AircraftRole::Fighter => "fighter",
    }
}

/// A refusal as a player is shown it: the engine's own words, unchanged, and
/// for the reasons that need one a short hint of what the order needs. The
/// engine's text is what the events and the saved games hold; the hint is
/// added here only, where a refusal is shown to a player.
pub(crate) fn refusal(reason: &sim_core::orders::RejectReason) -> String {
    use sim_core::orders::RejectReason as R;
    let hint = match reason {
        R::UnderWay => {
            " (load needs the task force stopped at a base of your own side: a force that is under way, or that a route order in the same day sends under way, cannot load; give the load order on a day the force starts at the base with no route order)"
        }
        R::NoOwnBaseHere => {
            " (load needs the task force stopped in the hex of a base of your own side)"
        }
        R::NotAnEnemyBase => {
            " (a base strike needs the number of a base the enemy holds; list_bases shows them)"
        }
        R::WaypointUnreachable(_) => {
            " (measure with question sea_route tells whether ships can get from one place to another)"
        }
        R::NotAFighter => {
            " (combat_air_patrol and set_patrol_share are for fighter groups; the briefing's air group lines say which type each group is)"
        }
        _ => "",
    };
    format!("{reason}{hint}")
}
