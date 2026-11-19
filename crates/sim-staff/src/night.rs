//! A run by night (decision 0145, step 5 of stage 3, first part): where an
//! enemy base on the side's map lies within 15 hexes of the place a run goes
//! to and not in that place's own hex, a supply run of "keep a base
//! supplied" and a lift of "take a place" are timed, at preserve the force
//! and at balanced, to arrive in the night.
//!
//! - **A place the side's own fighters are based at, or a hex next to it**
//!   (item 2a and ruling 6), is run to by day, at every risk: read from the
//!   side's own groups.
//! - **The hour it would arrive** (item 4) is the engine's forecast of the
//!   passage from the source, sailing at the first hour of a day, at cruising
//!   speed and then at full. It arrives in the night when its hours, less the
//!   whole days in them, are fewer than 12; the night's hours (item 6) are 12
//!   less that.
//! - **The speed** (item 5) is the first of the two that arrives in the night
//!   with room for the load and for which every ship has the fuel of the
//!   route out and home at that speed, by the engine's measure.
//! - **The night's limit** (item 7): no ship carries more than the place's
//!   rate for it times the night's hours. A supply run's load names a level,
//!   the least of the ships' limits, and each ship takes no more than the
//!   level less its supply aboard; the load is cut to what the levels allow
//!   (decision 0162, 5 to 7, replacing item 8 for a supply run). A lift's
//!   load is still cut until the first ship filled carries no more than its
//!   limit, and each after it likewise (item 8; decision 0162, 9).
//!
//! The pure parts are [`night_hours`], [`weight_within`], [`spread`] and
//! [`most_within`]; the rest reads the side's view and the engine's measures
//! through it.

use sim_core::command::Risk;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, ShipId, TaskForceId};
use sim_core::orders::SpeedSetting;
use sim_core::refdata::AircraftRole;
use sim_core::state::AirGroupLocation;
use sim_core::view::SideView;
use sim_core::PassageEnd;

use crate::reasons::base_words;
use crate::stand::{ShipFuel, NIGHT_HOURS, THREAT_HEXES};

/// The night's hours left on arrival after a passage of `hours` from the
/// first hour of a day (items 4 and 6): `None` where it arrives by day.
pub(crate) fn night_hours(hours: Fixed) -> Option<Fixed> {
    let day = Fixed::from_int(24);
    let whole = (hours / day).trunc();
    let into = hours - Fixed::from_int(whole) * day;
    let night = Fixed::from_int(NIGHT_HOURS);
    (into < night).then(|| night - into)
}

/// The most a load to a weight may take so that no ship holds more than its
/// limit (item 8): each line is a ship's supply aboard, its room to take
/// more and its limit, in ship order. The weight fills ship after ship; at
/// the first ship whose supply and room together pass its limit it stops at
/// the limit.
pub(crate) fn weight_within(lines: &[(Fixed, Fixed, Fixed)]) -> Fixed {
    let mut most = Fixed::ZERO;
    for (aboard, room, limit) in lines {
        if *aboard + *room <= *limit {
            most += *room;
        } else {
            most += (*limit - *aboard).max(Fixed::ZERO);
            break;
        }
    }
    most
}

/// What each ship takes of a load of `take_t` with a level of `level_t`
/// (decision 0162, 1), as the engine fills it: each line is a ship's supply
/// aboard and its room to take more, in ship order; each takes the least of
/// its room, what is left of the load and the level less its supply aboard,
/// not below 0.
pub(crate) fn spread(lines: &[(Fixed, Fixed)], take_t: Fixed, level_t: Fixed) -> Vec<Fixed> {
    let mut left = take_t;
    lines
        .iter()
        .map(|(aboard, room)| {
            let part = (*room)
                .min(left)
                .min((level_t - *aboard).max(Fixed::ZERO))
                .max(Fixed::ZERO);
            left -= part;
            part
        })
        .collect()
}

/// The most a supply run may take with a level of `level_t` (decision 0162,
/// 5): the sum, over its ships, of the least of a ship's room and the level
/// less its supply aboard, not below 0.
pub(crate) fn most_within(lines: &[(Fixed, Fixed)], level_t: Fixed) -> Fixed {
    lines
        .iter()
        .map(|(aboard, room)| (*room).min((level_t - *aboard).max(Fixed::ZERO)))
        .fold(Fixed::ZERO, |sum, part| sum + part)
}

/// The enemy base that makes a run go by night (item 2): the nearest on the
/// side's map within 15 hexes of `place` and not in its hex, the lowest
/// number on a tie; with its hexes.
pub(crate) fn enemy_base_near(view: &SideView<'_>, place: Hex) -> Option<(BaseId, u32)> {
    let side = view.side();
    view.enemy_bases()
        .filter(|b| b.side != side && b.hex != place)
        .map(|b| (b.hex.distance(place), b.id))
        .filter(|(hexes, _)| *hexes <= THREAT_HEXES)
        .min()
        .map(|(hexes, id)| (id, hexes))
}

/// Item 2a (with ruling 6): a base of the side's own in `place`'s hex or in a
/// hex next to it with a fighter group of the side's based at it, whatever
/// its state: the lowest numbered such base and the lowest numbered group.
/// Read from the side's own groups only.
pub(crate) fn own_fighters_at(view: &SideView<'_>, place: Hex) -> Option<(BaseId, AirGroupId)> {
    let bases: Vec<BaseId> = view
        .bases()
        .filter(|b| b.hex.distance(place) <= 1)
        .map(|b| b.id)
        .collect();
    view.air_groups()
        .filter_map(|g| match g.location {
            AirGroupLocation::Base(base) if bases.contains(&base) => {
                let kind = view.aircraft_type(g.aircraft_type)?;
                (kind.role == AircraftRole::Fighter).then_some((base, g.id))
            }
            _ => None,
        })
        .min()
}

/// Whether a run at `risk` to `place` goes by night, and the base that makes
/// it (items 2, 2a and 3). At press it goes as before, and so it does to a
/// place where the side's own fighters are based (item 2a).
pub(crate) fn applies(view: &SideView<'_>, risk: Risk, place: Hex) -> Option<(BaseId, u32)> {
    if risk == Risk::Press || own_fighters_at(view, place).is_some() {
        return None;
    }
    enemy_base_near(view, place)
}

/// One speed tried (items 4 and 5).
#[derive(Clone, Debug)]
pub(crate) struct Tried {
    pub(crate) speed: SpeedSetting,
    /// The hours of the passage out, where it arrives.
    pub(crate) hours: Option<Fixed>,
    /// The night's hours left on arrival, where it arrives in the night.
    pub(crate) night: Option<Fixed>,
    /// Each ship's fuel aboard against the burn out and home at this speed.
    pub(crate) fuel: Vec<ShipFuel>,
}

impl Tried {
    pub(crate) fn has_fuel(&self) -> bool {
        !self.fuel.iter().any(|f| f.short())
    }

    pub(crate) fn short_words(&self) -> String {
        self.fuel
            .iter()
            .filter(|f| f.short())
            .map(|f| format!("{} for the passage out and home", f.words()))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

pub(crate) fn speed_name(speed: SpeedSetting) -> &'static str {
    match speed {
        SpeedSetting::Full => "full speed",
        _ => "cruising speed",
    }
}

/// The two speeds tried for `force`, lying at the source, to `place` and
/// home through `waypoints` (the route out and back), by the engine's own
/// forecast: the hours of the passage to `place`, and the fuel of the whole
/// route at the speed (item 5).
pub(crate) fn try_force(
    view: &SideView<'_>,
    force: TaskForceId,
    place: Hex,
    waypoints: &[Hex],
) -> Vec<Tried> {
    [SpeedSetting::Cruise, SpeedSetting::Full]
        .into_iter()
        .map(|speed| {
            let hours = view
                .forecast_route(force, &[place], Some(speed))
                .and_then(|f| match f.end {
                    PassageEnd::Arrives { .. } => Some(f.hours),
                    _ => None,
                });
            let fuel = view
                .forecast_route(force, waypoints, Some(speed))
                .map(|f| {
                    f.ships
                        .iter()
                        .map(|line| {
                            // A ship the measure runs dry burns only what it
                            // has: its need is the burn of the whole route at
                            // the speed, by the rule the measure burns by
                            // (chapter 2, Fuel burn), so that a reason gives
                            // it against what is aboard.
                            let need = if line.runs_dry {
                                burn_of(view, line.ship, f.route_nm, f.speed_kn)
                                    .unwrap_or(line.burned_t)
                            } else {
                                line.burned_t
                            };
                            ShipFuel {
                                ship: line.ship,
                                aboard: line.fuel_t,
                                need,
                                runs_dry: false,
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();
            Tried {
                speed,
                hours,
                night: hours.and_then(night_hours),
                fuel,
            }
        })
        .collect()
}

/// The burn of `miles` at `knots` for `ship` (chapter 2, Fuel burn): the
/// cruise rate a mile, times the square of the speed over the cruise speed
/// above it.
fn burn_of(view: &SideView<'_>, ship: ShipId, miles: Fixed, knots: Fixed) -> Option<Fixed> {
    let found = view.ships().find(|s| s.id == ship)?;
    let class = view.ship_class(found.class)?;
    let base = miles.mul_div(class.fuel_capacity_t, class.endurance_nm);
    Some(if knots > class.cruise_speed_kn {
        let ratio = knots / class.cruise_speed_kn;
        base * ratio * ratio
    } else {
        base
    })
}

/// The two speeds tried for ships that lie in no one force, by the rules
/// the forecast steams by on an open route: the passage of `out_nm` at the
/// slowest ship's speed, and the burn of `round_nm` at that speed (chapter 2,
/// Fuel burn).
pub(crate) fn try_ships(
    view: &SideView<'_>,
    ships: &[ShipId],
    out_nm: u32,
    round_nm: u32,
) -> Vec<Tried> {
    let found: Vec<_> = view
        .ships()
        .filter(|s| ships.contains(&s.id))
        .filter_map(|s| view.ship_class(s.class).map(|c| (s.id, s.fuel_t, c)))
        .collect();
    [SpeedSetting::Cruise, SpeedSetting::Full]
        .into_iter()
        .map(|speed| {
            let knots = found
                .iter()
                .map(|(_, _, c)| match speed {
                    SpeedSetting::Full => c.max_speed_kn,
                    _ => c.cruise_speed_kn,
                })
                .min()
                .unwrap_or(Fixed::ZERO);
            let hours = (knots > Fixed::ZERO).then(|| Fixed::from_int(i64::from(out_nm)) / knots);
            let fuel = found
                .iter()
                .map(|(id, aboard, c)| {
                    let base = Fixed::from_int(i64::from(round_nm))
                        .mul_div(c.fuel_capacity_t, c.endurance_nm);
                    let need = if knots > c.cruise_speed_kn {
                        let ratio = knots / c.cruise_speed_kn;
                        base * ratio * ratio
                    } else {
                        base
                    };
                    ShipFuel {
                        ship: *id,
                        aboard: *aboard,
                        need,
                        runs_dry: false,
                    }
                })
                .collect();
            Tried {
                speed,
                hours,
                night: hours.and_then(night_hours),
                fuel,
            }
        })
        .collect()
}

/// The choice of item 5 among `tried`, for ships whose loads (in ship order,
/// with each ship's limit-giving rate) are `loads`: the first speed that
/// arrives in the night, has the fuel and has room for every ship's load;
/// where none has room, the one of those that arrive in the night with the
/// fuel that has the longest night, whose limit the load is then cut to.
/// `None` where no speed arrives in the night with the fuel.
pub(crate) fn choose<'t>(tried: &'t [Tried], loads: &[(Fixed, Fixed)]) -> Option<&'t Tried> {
    choose_by(tried, |night| {
        loads.iter().all(|(load, rate)| *load <= *rate * night)
    })
}

/// [`choose`] with the test of room for the load given as `fits`, which is
/// asked with a speed's night hours (decision 0162, 7: a supply run reads
/// it on its load as spread).
pub(crate) fn choose_by(tried: &[Tried], fits: impl Fn(Fixed) -> bool) -> Option<&Tried> {
    let eligible: Vec<&Tried> = tried
        .iter()
        .filter(|t| t.night.is_some() && t.has_fuel())
        .collect();
    let fits = |t: &Tried| fits(t.night.unwrap_or(Fixed::ZERO));
    eligible
        .iter()
        .find(|t| fits(t))
        .or_else(|| eligible.iter().max_by_key(|t| t.night))
        .copied()
}

/// The words of the two speeds tried: "at cruising speed the passage is 15
/// hours, arriving by day; at full speed 6.666666 hours, arriving in the
/// night with 5.333334 hours left".
pub(crate) fn tried_words(tried: &[Tried]) -> String {
    tried
        .iter()
        .map(|t| {
            let when = match (t.hours, t.night) {
                (None, _) => "it does not arrive".to_string(),
                (Some(h), Some(n)) => {
                    format!("{h} hours, arriving in the night with {n} hours of it left")
                }
                (Some(h), None) => format!("{h} hours, arriving by day"),
            };
            let fuel = if t.has_fuel() {
                String::new()
            } else {
                format!(" ({}: passed over)", t.short_words())
            };
            format!("at {} the passage is {when}{fuel}", speed_name(t.speed))
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// The words of the enemy base that sends a run by night.
pub(crate) fn base_near_words(view: &SideView<'_>, base: BaseId, hexes: u32, place: Hex) -> String {
    format!(
        "the enemy's {} lies {hexes} hexes from {place}, within {THREAT_HEXES} and not in its hex, so the run goes by night",
        base_words(view, base)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_night_left_on_arrival() {
        assert_eq!(night_hours(Fixed::from_int(15)), None);
        assert_eq!(
            night_hours(Fixed::from_int(240) / Fixed::from_int(36)),
            Some(Fixed::from_int(12) - Fixed::from_int(240) / Fixed::from_int(36))
        );
        assert_eq!(night_hours(Fixed::from_int(30)), Some(Fixed::from_int(6)));
        assert_eq!(night_hours(Fixed::from_int(12)), None);
    }

    #[test]
    fn a_load_spread_under_a_level() {
        let t = Fixed::from_int;
        // Decision 0162's order alone: 10 t, nothing and 45 t aboard, 45 t to
        // take, a level of 40 t.
        let lines = [(t(10), t(40)), (t(0), t(50)), (t(45), t(5))];
        assert_eq!(spread(&lines, t(45), t(40)), vec![t(30), t(15), t(0)]);
        // Scenario 85: three empty ships of 50 t, a level of 40 t.
        let empty = [(t(0), t(50)), (t(0), t(50)), (t(0), t(50))];
        assert_eq!(most_within(&empty, t(40)), t(120));
        assert_eq!(spread(&empty, t(150), t(40)), vec![t(40), t(40), t(40)]);
    }

    #[test]
    fn a_weight_stops_at_the_first_ship_over_its_limit() {
        let limit = Fixed::from_ratio(64, 3);
        let thirty = Fixed::from_int(30);
        assert_eq!(
            weight_within(&[(Fixed::ZERO, thirty, limit), (Fixed::ZERO, thirty, limit)]),
            limit
        );
        let big = Fixed::from_int(100);
        assert_eq!(
            weight_within(&[(Fixed::ZERO, thirty, big), (Fixed::ZERO, thirty, big)]),
            Fixed::from_int(60)
        );
    }
}
