//! What a group can fly in a pulse: the flight deck, fatigue, pilots,
//! supply and a damaged airfield (decisions 0026, 0036, 0038, 0039 and 0103).

use super::*;

/// How many aircraft of `group` can be flown this pulse (decision 14 of 0026
/// and decision 2 of 0036): its ready aircraft times the flight deck's factor
/// (all of them from a base, or from a ship whose deck works; the degraded
/// factor if it is degraded; none if it is out) times 100 less its fatigue
/// over 100, multiplied in fixed point and rounded down once; for a group at
/// a base, also times 100 less the damage of its airfield (`field_damage`,
/// the base's at the start of the pulse, 0 for a group aboard a ship) over 100
/// (decision 4 of 0103), between the deck's factor and the fatigue's. A
/// stood-down group flies none. Without the fatigue figures (`fatigue_loaded` false) a
/// group is never tired or stood down. A group whose pilots are tracked
/// starts from the smaller of its ready aircraft and its pilots (decision 2
/// of 0038). A group at a base with no supply at the start of the pulse
/// (`base_grounded`) flies none (decision 7 of 0039). A group that moved to
/// another base by order today flies none (decision 0091, 6): `day` is the
/// day being resolved.
///
/// `flight_deck` is the state of the flight deck of the ship the group is
/// aboard, `None` if it is not aboard a ship that exists; it matters only
/// with the surface combat figures, without which no system can be damaged.
/// The inputs are the group's own and the figures: this is the one function
/// the engine and a side's view both call (decision 0100, 1).
pub(crate) fn able_to_fly(
    group: &AirGroup,
    base_grounded: bool,
    field_damage: Fixed,
    flight_deck: Option<SystemState>,
    surface_combat: Option<&SurfaceCombatFigures>,
    fatigue_loaded: bool,
    day: u32,
) -> u32 {
    // A base with no supply at the start of the pulse flies nothing
    // (decision 7 of 0039). Groups aboard ships are never affected.
    if matches!(group.location, AirGroupLocation::Base(_)) && base_grounded {
        return 0;
    }
    // A group that moved to another base today flies nothing (decision
    // 0091, 6).
    if group.rebased_day == Some(day) {
        return 0;
    }
    let crewed = group
        .pilots
        .map_or(group.ready, |pilots| group.ready.min(pilots));
    let mut able = Fixed::from_int(i64::from(crewed));
    if let (AirGroupLocation::Ship(_), Some(state)) = (group.location, flight_deck) {
        // Without the figures no system can be damaged (World::new).
        if let Some(figures) = surface_combat {
            able = able * figures.system_factor(ShipSystem::FlightDeck, state);
        }
    }
    if matches!(group.location, AirGroupLocation::Base(_)) && field_damage > Fixed::ZERO {
        let hundred = Fixed::from_int(100);
        able = able.mul_div(hundred - field_damage.min(hundred), hundred);
    }
    if fatigue_loaded {
        if group.stood_down {
            return 0;
        }
        let hundred = Fixed::from_int(100);
        able = able.mul_div(hundred - group.fatigue, hundred);
    }
    u32::try_from(able.trunc()).expect("a share of the aircraft is not negative")
}

impl World {
    /// Whether a group's search is flown in this pulse (decision 0131, 1):
    /// it is on the search mission and, with search once a day set, it is
    /// not the afternoon. Whether it has aircraft, light and an arc is for
    /// the caller.
    pub fn searches_in(&self, group: &AirGroup, pulse: Pulse) -> bool {
        group.mission == AirMission::Search && !(group.search_once && pulse == Pulse::Afternoon)
    }

    /// The aircraft of `group` that can be flown this pulse (decision 14 of
    /// 0026 and decision 2 of 0036): its ready aircraft times the flight
    /// deck's factor (all of them from a base, or from a ship whose deck
    /// works; the degraded factor if it is degraded; none if it is out) times
    /// 100 less its fatigue over 100, multiplied in fixed point and rounded
    /// down once. A stood-down group flies none. Without the fatigue
    /// figures a group is never tired or stood down. A group whose pilots
    /// are tracked starts from the smaller of its ready aircraft and its
    /// pilots (decision 2 of 0038). A group at a base with no supply at the
    /// start of the pulse flies none (decision 7 of 0039). A group that
    /// moved to another base by order today flies none (decision 0091, 6).
    /// A group at a base with a damaged airfield flies a share of them
    /// (decision 4 of 0103), by the damage at the start of the pulse.
    pub(crate) fn aircraft_able_to_fly(&self, group: &AirGroup) -> u32 {
        let base_grounded = match group.location {
            AirGroupLocation::Base(base) => self.grounded_bases.contains(&base),
            _ => false,
        };
        let field_damage = match group.location {
            AirGroupLocation::Base(base) => {
                self.field_damage.get(&base).copied().unwrap_or(Fixed::ZERO)
            }
            _ => Fixed::ZERO,
        };
        let flight_deck = match group.location {
            AirGroupLocation::Ship(ship) => self
                .truth
                .ships
                .get(&ship)
                .map(|ship| ship.deck_for_flying()),
            _ => None,
        };
        let able = able_to_fly(
            group,
            base_grounded,
            field_damage,
            flight_deck,
            self.ref_data.surface_combat.as_ref(),
            self.ref_data.fatigue.is_some(),
            self.truth.day + 1,
        );
        self.trace_choice(group.side, || {
            (
                Question::Air(AirQuestion::AircraftAbleToFly { group: group.id }),
                Answer::Air(AirAnswer::AircraftAbleToFly(able)),
            )
        });
        able
    }
}
