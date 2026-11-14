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
        TaskForceStatus::Halted => "halted",
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

/// A fight's ending as a side is told it (decision 0085).
pub(crate) fn side_end_reason(reason: sim_core::side_events::SideEndReason) -> &'static str {
    use sim_core::side_events::SideEndReason as R;
    match reason {
        R::SlippedAway => "slipped_away",
        R::ForceSunk => "force_sunk",
        R::BrokeOff => "broke_off",
        R::PassedUnseen => "passed_unseen",
        R::MaxRounds => "max_rounds",
        R::ContactLost => "contact_lost",
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
            " (load (of supply or fuel) and send_to_yard need the task force stopped at a base of your own side: a force that is under way, or that a route order in the same day sends under way, cannot take them; give the order on a day the force starts at the base with no route order)"
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
        R::NotYourLandUnit => {
            " (a load with a unit needs the handle of a land unit of your own side, written land_unit:N; list_forces shows them)"
        }
        R::NoMenAshoreHere => {
            " (a load with a unit takes the unit's men that are ashore in the hex where the task force is stopped; the unit's line says where its men are)"
        }
        R::NoTroopCapacity => {
            " (a load with a unit needs a ship in the task force that can carry men, such as a transport)"
        }
        R::AirGroupAboardShip => {
            " (rebase is for a group at a base of your side; a group aboard a ship cannot be rebased)"
        }
        R::AirGroupAwayAtYard => {
            " (rebase is for a group at a base of your side; a group aboard a ship cannot be rebased, at the yard or not)"
        }
        R::AlreadyAtBase => {
            " (rebase needs another base of your side than the one the group is at)"
        }
        R::BeyondTransferRange { .. } => {
            " (the transfer range is twice the longer of the aircraft type's search range and strike radius; measure with question air_reach gives it, and a nearer base can be used first, the move made over two days)"
        }
        R::LandUnitDestroyed => {
            " (a destroyed unit has no man left, ashore or aboard, and takes no order)"
        }
        R::NoMenAshore => {
            " (set_posture is for a unit with men ashore, and so is march; a unit still aboard ship has none to order until it lands)"
        }
        R::AirGroupLostWithBase => {
            " (the group was lost when the base it was at changed side; it has no place and takes no order)"
        }
        R::NoLandLink(_) => {
            " (a land unit marches only to a neighbouring hex that the map joins to its own by land, or to its own hex to stop; the unit's line in the briefing and get_entity lists the hexes it can march to under march_to, and a map with no land links allows no march)"
        }
        R::MarkNotOnRoute(_) => {
            " (act_at marks waypoints of the same set_route: each mark must be one of the route's waypoints, and the last waypoint needs no mark since a force stops there anyway)"
        }
        R::AlreadyMovedToday => {
            " (a group moves once a day; the order that moved it was given earlier in this day)"
        }
        _ => "",
    };
    format!("{reason}{hint}")
}

#[cfg(test)]
mod tests {
    use super::refusal;
    use sim_core::orders::RejectReason;

    #[test]
    fn the_refusals_of_a_posture_order_and_of_a_lost_group_have_their_hints() {
        let destroyed = refusal(&RejectReason::LandUnitDestroyed);
        assert!(
            destroyed.starts_with("the land unit is destroyed (a destroyed unit has no man left"),
            "{destroyed}"
        );
        let none_ashore = refusal(&RejectReason::NoMenAshore);
        assert!(
            none_ashore.starts_with(
                "the land unit has no men ashore (set_posture is for a unit with men ashore"
            ),
            "{none_ashore}"
        );
        let no_link = refusal(&RejectReason::NoLandLink(sim_core::hex::Hex::new(1, 0)));
        assert!(
            no_link.starts_with("no land link joins the land unit's hex to")
                && no_link.contains("march_to"),
            "{no_link}"
        );
        let mark = refusal(&RejectReason::MarkNotOnRoute(sim_core::hex::Hex::new(2, 1)));
        assert!(
            mark.contains("is not one of the route's waypoints (act_at marks")
                && mark.contains("the last waypoint needs no mark"),
            "{mark}"
        );
        let lost = refusal(&RejectReason::AirGroupLostWithBase);
        assert!(
            lost.starts_with(
                "the air group was lost with its base (the group was lost when the base"
            ),
            "{lost}"
        );
    }
}
