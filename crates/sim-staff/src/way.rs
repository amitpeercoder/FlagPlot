//! The way a force takes (decision 0203, stage 1): the staff chooses the
//! waypoints of a carrier force going to a station it chose (decision 0202),
//! at balanced or preserve, by how late the force may arrive, its fuel and
//! its danger by day.
//!
//! **What is weighed** (rules 10 to 12). The circles of danger are decision
//! 0202's ([`Day::circles_for`]); the danger of a hex is how many hexes
//! inside a circle it lies and one more ([`danger_of`]); the danger of a way
//! is the sum of the danger of the hexes the engine's forecast has the force
//! enter by day, but for the last.
//!
//! **The limits** (rules 13 to 15): the way arrives no later than the pulse
//! after the shortest way's at balanced and the same pulse of the day after
//! at preserve ([`latest_pulse`]), and is in fuel by decision 0202's test on
//! its own miles. Both are read from the engine's forecast.
//!
//! **The choice** (rules 16 to 22). The search proposes ways by the staff's
//! clock ([`search`], [`StaffClock`]); each is made a route by the routine
//! that keeps a route inside an area ([`crate::route::legs_inside_with`]); each
//! route is forecast by the engine, and of those within both limits, the
//! shortest way among them, the one of least danger is taken, then the fewest
//! miles charged, then the fewest waypoints, then the proposal of fewest
//! steps.
//!
//! The reasons speak in hexes, pulses and distances, never in a figure of
//! danger (rules 27 to 29). This module draws no random number and adds no
//! state.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{Intent, IntentKind, Maker, Risk};
use sim_core::fixed::Fixed;
use sim_core::hex::{Hex, HEX_NM};
use sim_core::orders::SpeedSetting;
use sim_core::state::TaskForce;
use sim_core::time::{pulse_start, Pulse};
use sim_core::view::SideView;
use sim_core::PassageEnd;

use crate::air::Day;
use crate::station::Circle;

// ---------------------------------------------------------------------------
// The pure parts.
// ---------------------------------------------------------------------------

/// The danger of `hex` (rule 11): how many hexes inside a circle it lies, and
/// one more, 1 at the rim; in more than one circle the greatest; outside
/// every circle 0.
pub fn danger_of(hex: Hex, circles: &[Circle]) -> u32 {
    circles
        .iter()
        .filter_map(|c| {
            let d = hex.distance(c.centre);
            (d <= c.hexes).then(|| c.hexes - d + 1)
        })
        .max()
        .unwrap_or(0)
}

/// Whether a pulse is by day: the morning or the afternoon (rule 8).
pub fn by_day(pulse: Pulse) -> bool {
    pulse != Pulse::Night
}

/// The place of a pulse in the count of pulses from the night of day 1.
fn pulse_index(day: u32, pulse: Pulse) -> u32 {
    let at = match pulse {
        Pulse::Night => 0,
        Pulse::Morning => 1,
        Pulse::Afternoon => 2,
    };
    day.saturating_sub(1) * 3 + at
}

/// The pulse after `(day, pulse)`.
fn next_pulse(day: u32, pulse: Pulse) -> (u32, Pulse) {
    match pulse {
        Pulse::Night => (day, Pulse::Morning),
        Pulse::Morning => (day, Pulse::Afternoon),
        Pulse::Afternoon => (day + 1, Pulse::Night),
    }
}

/// The last pulse a way may arrive in (rule 13), the shortest way arriving in
/// `(day, pulse)`: at balanced the pulse after; at preserve the same pulse of
/// the day after; at press the same pulse.
pub fn latest_pulse(risk: Risk, day: u32, pulse: Pulse) -> (u32, Pulse) {
    match risk {
        Risk::Balanced => next_pulse(day, pulse),
        Risk::Preserve => (day + 1, pulse),
        Risk::Press => (day, pulse),
    }
}

/// Whether `(day, pulse)` is no later than `(last_day, last_pulse)`.
pub fn no_later(day: u32, pulse: Pulse, last: (u32, Pulse)) -> bool {
    pulse_index(day, pulse) <= pulse_index(last.0, last.1)
}

/// The staff's clock (rule 9): a force that leaves at the start of the night
/// pulse of `first_day` at `knots`. It only proposes; nothing is decided on
/// it.
#[derive(Clone, Copy, Debug)]
pub struct StaffClock {
    pub first_day: u32,
    pub knots: Fixed,
}

impl StaffClock {
    /// The pulse in which the force has steamed `nm` miles: an hour on the
    /// line between two pulses belongs to the earlier, as in the engine.
    pub fn pulse_at_nm(&self, nm: Fixed) -> (u32, Pulse) {
        let start = pulse_start(self.first_day, Pulse::Night);
        let mut day = self.first_day;
        // A passage the staff weighs is of a few days; this ends it always.
        for _ in 0..4000 {
            for pulse in Pulse::ALL {
                let end = pulse_start(day, pulse) + Fixed::from_int(pulse.hours()) - start;
                if nm <= end * self.knots {
                    return (day, pulse);
                }
            }
            day += 1;
        }
        (day, Pulse::Night)
    }

    /// The latest the force enters the hex of step `n` (rule 9), in miles
    /// steamed: `n` times 40 nm less 20.
    pub fn latest_nm(n: u32) -> Fixed {
        Fixed::from_int(HEX_NM * i64::from(n) - HEX_NM / 2)
    }

    /// The earliest the force enters `hex` from `start` (rule 9), in miles
    /// steamed: the straight line less 20 nm.
    pub fn earliest_nm(start: Hex, hex: Hex) -> Fixed {
        Fixed::from_int(i64::from(start.air_distance_nm(hex)) - HEX_NM / 2)
    }

    /// Whether the hex of step `n`, `hex`, counts as entered by night: both
    /// its hours fall in the same night.
    pub fn by_night(&self, start: Hex, hex: Hex, n: u32) -> bool {
        let early = self.pulse_at_nm(Self::earliest_nm(start, hex));
        let late = self.pulse_at_nm(Self::latest_nm(n));
        early == late && early.1 == Pulse::Night
    }

    /// The most steps whose latest hour falls no later than the end of
    /// `last` (rule 17).
    pub fn most_steps(&self, last: (u32, Pulse)) -> u32 {
        let start = pulse_start(self.first_day, Pulse::Night);
        let end = pulse_start(last.0, last.1) + Fixed::from_int(last.1.hours()) - start;
        let nm = end * self.knots + Fixed::from_int(HEX_NM / 2);
        let steps = (nm / Fixed::from_int(HEX_NM)).trunc();
        u32::try_from(steps.max(0)).unwrap_or(0)
    }
}

/// One way the search proposes (rule 17): the count of steps it was found
/// for and the hexes it enters, in order, the last the station.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Proposal {
    pub steps: u32,
    pub hexes: Vec<Hex>,
}

/// The search (rule 17). For each count of steps `n` from `least` to `most`,
/// the way from `start` to `station` of `n` steps or fewer with the least
/// danger, then the fewest steps, where `cost(hex, k)` is the danger of
/// entering `hex` at step `k` and the station costs nothing. It runs on the
/// steps `can_step` allows, over the hexes whose fewest steps from the start
/// and to the station (`from_start`, `to_station`) sum to `most` or less. It
/// is worked backward from the station; the way is built forward from the
/// start, taking at each step the first direction in [`Hex::DIRECTIONS`]
/// order that keeps the least. A way may pass a hex twice or double back.
#[allow(clippy::too_many_arguments)]
pub fn search(
    start: Hex,
    station: Hex,
    least: u32,
    most: u32,
    from_start: &BTreeMap<Hex, u32>,
    to_station: &BTreeMap<Hex, u32>,
    can_step: &dyn Fn(Hex, usize) -> bool,
    cost: &dyn Fn(Hex, u32) -> u32,
) -> Vec<Proposal> {
    if start == station {
        return Vec::new();
    }
    let hexes: Vec<Hex> = from_start
        .iter()
        .filter_map(|(h, a)| {
            let b = to_station.get(h)?;
            (a + b <= most).then_some(*h)
        })
        .collect();
    let index: BTreeMap<Hex, usize> = hexes.iter().enumerate().map(|(i, h)| (*h, i)).collect();
    let (Some(&s), Some(&t)) = (index.get(&start), index.get(&station)) else {
        return Vec::new();
    };
    let next: Vec<Vec<usize>> = hexes
        .iter()
        .map(|h| {
            h.neighbors()
                .into_iter()
                .enumerate()
                .filter(|(d, _)| can_step(*h, *d))
                .filter_map(|(_, n)| index.get(&n).copied())
                .collect()
        })
        .collect();
    // The danger of entering each hex at each step, worked out once.
    let steps = usize::try_from(most).unwrap_or(0);
    let mut costs: Vec<Vec<u64>> = vec![vec![0; hexes.len()]; steps + 1];
    for (k, row) in costs.iter_mut().enumerate().skip(1) {
        let k32 = u32::try_from(k).unwrap_or(u32::MAX);
        for (i, h) in hexes.iter().enumerate() {
            if i != t && from_start[h] <= k32 {
                row[i] = u64::from(cost(*h, k32));
            }
        }
    }
    let mut proposals = Vec::new();
    for n in least..=most {
        let n_us = usize::try_from(n).unwrap_or(0);
        // value[k][i]: the least (danger, steps) still to come from hex i
        // after k steps, arriving by step n.
        let mut value: Vec<Vec<Option<(u64, u32)>>> = vec![vec![None; hexes.len()]; n_us + 1];
        for row in &mut value {
            row[t] = Some((0, 0));
        }
        for k in (0..n_us).rev() {
            let k32 = u32::try_from(k).unwrap_or(u32::MAX);
            for i in 0..hexes.len() {
                if i == t {
                    continue;
                }
                let h = hexes[i];
                if from_start[&h] > k32 || to_station[&h] + k32 > n {
                    continue;
                }
                let mut best: Option<(u64, u32)> = None;
                for &j in &next[i] {
                    if let Some((d, st)) = value[k + 1][j] {
                        let c = (d + costs[k + 1][j], st + 1);
                        if best.is_none_or(|b| c < b) {
                            best = Some(c);
                        }
                    }
                }
                value[k][i] = best;
            }
        }
        let Some(_) = value[0][s] else {
            continue;
        };
        let mut way = Vec::new();
        let mut at = s;
        let mut k = 0;
        while at != t {
            let want = value[k][at].expect("a hex on the way has a value");
            let chosen = next[at]
                .iter()
                .copied()
                .find(|&j| {
                    value[k + 1][j].is_some_and(|(d, st)| (d + costs[k + 1][j], st + 1) == want)
                })
                .expect("one direction keeps the least");
            way.push(hexes[chosen]);
            at = chosen;
            k += 1;
        }
        proposals.push(Proposal {
            steps: n,
            hexes: way,
        });
    }
    proposals
}

/// What a forecast way enters inside one circle (rule 27): by day, by night,
/// and the nearest it passes to the circle's centre by day.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Inside {
    pub by_day: u32,
    pub by_night: u32,
    pub nearest_by_day: Option<u32>,
}

/// One route weighed (rule 19): its waypoints and what the engine's forecast
/// gives of it.
#[derive(Clone, Debug)]
pub(crate) struct Weighed {
    pub(crate) waypoints: Vec<Hex>,
    /// The proposal's count of steps; the shortest way's for it.
    pub(crate) steps: u32,
    pub(crate) arrives: (u32, Pulse),
    pub(crate) miles: Fixed,
    pub(crate) danger: u64,
    pub(crate) in_limit: bool,
    pub(crate) in_fuel: bool,
    pub(crate) fuel_words: String,
    /// For each circle, what the way enters inside it.
    pub(crate) inside: Vec<Inside>,
    /// The hexes the engine has the force enter, in order.
    pub(crate) hexes: Vec<Hex>,
}

/// The danger of a forecast way and what it enters of each circle (rules 12
/// and 27): every hex entered but the last.
pub fn weigh_hexes(
    hexes: &[Hex],
    entered_in: &[(u32, Pulse)],
    circles: &[Circle],
) -> (u64, Vec<Inside>) {
    let mut danger = 0u64;
    let mut inside = vec![Inside::default(); circles.len()];
    let count = hexes.len().saturating_sub(1);
    for (hex, (_, pulse)) in hexes.iter().zip(entered_in).take(count) {
        let day = by_day(*pulse);
        if day {
            danger += u64::from(danger_of(*hex, circles));
        }
        for (c, seen) in circles.iter().zip(inside.iter_mut()) {
            let d = hex.distance(c.centre);
            if d > c.hexes {
                continue;
            }
            if day {
                seen.by_day += 1;
                seen.nearest_by_day = Some(seen.nearest_by_day.map_or(d, |n| n.min(d)));
            } else {
                seen.by_night += 1;
            }
        }
    }
    (danger, inside)
}

/// The words of a pulse: "the morning of day 2".
pub fn pulse_text(day: u32, pulse: Pulse) -> String {
    format!("the {} of day {day}", pulse.name())
}

/// The waypoints in words: "(10,2), (6,2), (3,0)".
fn points_text(points: &[Hex]) -> String {
    points
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

/// What a way enters inside the circles, in words (rule 27): for each circle
/// it enters, the hexes by day, the nearest by day, and the hexes by night.
fn inside_text(inside: &[Inside], circles: &[Circle]) -> String {
    let parts: Vec<String> = circles
        .iter()
        .zip(inside)
        .filter(|(_, i)| i.by_day + i.by_night > 0)
        .map(|(c, i)| {
            let day = match (i.by_day, i.nearest_by_day) {
                (0, _) | (_, None) => "none by day".to_string(),
                (n, Some(d)) => format!(
                    "{n} {} by day, the nearest {d} hexes from it",
                    if n == 1 { "hex" } else { "hexes" }
                ),
            };
            let night = match i.by_night {
                0 => "none by night".to_string(),
                n => format!("{n} by night"),
            };
            format!("inside the reach of {}, {day}, and {night}", c.words)
        })
        .collect();
    if parts.is_empty() {
        "enters no hex inside the enemy's reach".to_string()
    } else {
        format!("enters, {}", parts.join("; "))
    }
}

/// The hexes of the engine's path from `here` through `waypoints` in turn
/// (decision 0203, 25): what the search on the way reads.
pub(crate) fn path_through(view: &SideView<'_>, here: Hex, waypoints: &[Hex]) -> Vec<Hex> {
    let mut path = Vec::new();
    let mut at = here;
    for w in waypoints {
        path.extend(view.sea_path(at, *w).unwrap_or_default());
        at = *w;
    }
    path
}

// ---------------------------------------------------------------------------
// The morning.
// ---------------------------------------------------------------------------

/// The way chosen for a force bound for its station.
#[derive(Clone, Debug)]
pub(crate) struct WayChoice {
    /// The waypoints given: the station alone where the shortest way is
    /// taken.
    pub(crate) waypoints: Vec<Hex>,
    /// Whether the way bends: more than the one point of the station.
    pub(crate) bent: bool,
    /// What the reason says of the way, where it says anything (rules 27,
    /// 28 and 21).
    pub(crate) words: Option<String>,
    pub(crate) evidence: Vec<String>,
}

/// A carrier task's force is on a way the staff gave and keeps (rule 24):
/// under way on a route of more than the one point of its station, the last
/// its station.
pub(crate) fn on_kept_way(force: &TaskForce, station: Hex) -> bool {
    !crate::stand::stopped(force)
        && force.route.waypoints.len() > 1
        && force.route.waypoints.last() == Some(&station)
        && force.route.next_index < force.route.waypoints.len()
}

/// The waypoints that remain of a kept way.
pub(crate) fn remaining(force: &TaskForce) -> &[Hex] {
    let at = force.route.next_index.min(force.route.waypoints.len());
    &force.route.waypoints[at..]
}

impl Day<'_, '_> {
    /// Whether the way of `task`'s force to its station is the staff's to
    /// choose (rules 1, 2 and 5): a carrier task the staff chose a station
    /// for, at balanced or preserve.
    pub(crate) fn chooses_way(&self, task: &Intent, risk: Risk) -> bool {
        risk != Risk::Press
            && task.maker == Maker::Staff
            && matches!(task.kind, IntentKind::OperateCarriers { .. })
            && self.station_chosen(task)
    }

    /// Rules 6 to 22 and 27 to 29: the way `force` takes to `station` at
    /// `speed` (rule 7), or `None` where nothing is chosen (rule 6: the
    /// shortest way does not arrive by the engine's forecast).
    #[allow(clippy::too_many_lines)]
    pub(crate) fn way_to_station(
        &self,
        task: &Intent,
        force: &TaskForce,
        station: Hex,
        risk: Risk,
        speed: SpeedSetting,
    ) -> Option<WayChoice> {
        let view = self.view;
        let id = force.id;
        let end = task.parent.and_then(|p| self.state.intents.get(&p))?;
        let aim = self.aim_of(end)?;
        let circles = self.circles_for(&aim);
        let shortest = view.forecast_route(id, &[station], Some(speed))?;
        if shortest.refusal.is_some() {
            return None;
        }
        let PassageEnd::Arrives { day, pulse, .. } = shortest.end else {
            return None;
        };
        let plain = WayChoice {
            waypoints: vec![station],
            bent: false,
            words: None,
            evidence: Vec::new(),
        };
        if circles.is_empty() {
            // No circle: no hex has danger, and the search proposes the
            // engine's own way alone (rule 17).
            return Some(plain);
        }
        let last = latest_pulse(risk, day, pulse);
        let start = shortest.starts_at;
        let knots = shortest.speed_kn;
        let clock = StaffClock {
            first_day: view.day() + 1,
            knots,
        };
        let from_start = view.sea_distances_to(start);
        let to_station = view.sea_distances_to(station);
        let least = to_station.get(&start).copied()?;
        let most = clock.most_steps(last).max(least);
        let can_step = |h: Hex, d: usize| view.can_step(h, d);
        let cost = |h: Hex, k: u32| {
            if clock.by_night(start, h, k) {
                0
            } else {
                danger_of(h, &circles)
            }
        };
        let proposals = search(
            start,
            station,
            least,
            most,
            &from_start,
            &to_station,
            &can_step,
            &cost,
        );
        // Each proposal made a route (rule 18); the shortest way first.
        let mut routes: Vec<(Vec<Hex>, u32)> = vec![(vec![station], least)];
        let mut fields = crate::route::Fields::default();
        for p in &proposals {
            let mut area: BTreeSet<Hex> = p.hexes.iter().copied().collect();
            area.insert(start);
            let Some(points) =
                crate::route::legs_inside_with(view, &mut fields, start, station, &area)
            else {
                continue;
            };
            if !routes.iter().any(|(w, _)| *w == points) {
                routes.push((points, p.steps));
            }
        }
        // Each route forecast by the engine (rule 19), and weighed.
        let (home, home_field) = self.fuel_home(force);
        let full = self.weighed_full(force);
        let mut weighed: Vec<Weighed> = Vec::new();
        for (points, steps) in routes {
            let forecast = if points == [station] {
                shortest.clone()
            } else {
                match view.forecast_route(id, &points, Some(speed)) {
                    Some(f) if f.refusal.is_none() => f,
                    _ => continue,
                }
            };
            let PassageEnd::Arrives { day, pulse, .. } = forecast.end else {
                continue;
            };
            let (danger, inside) = weigh_hexes(&forecast.hexes, &forecast.entered_in, &circles);
            let fuel = self.fuel_read(
                force,
                station,
                (forecast.route_nm, forecast.speed_kn, speed),
                risk,
                home,
                home_field.as_ref(),
                full,
            );
            weighed.push(Weighed {
                waypoints: points,
                steps,
                arrives: (day, pulse),
                miles: forecast.route_nm,
                danger,
                in_limit: no_later(day, pulse, last),
                in_fuel: fuel.in_fuel,
                fuel_words: fuel.words,
                inside,
                hexes: forecast.hexes,
            });
        }
        let short = weighed.first().cloned()?;
        // The choice (rule 20); where none is in fuel, the shortest (rule 21).
        let chosen = weighed
            .iter()
            .filter(|w| w.in_limit && w.in_fuel)
            .min_by_key(|w| (w.danger, w.miles, w.waypoints.len(), w.steps))
            .cloned();
        let speed_text = match speed {
            SpeedSetting::Full => "full speed",
            _ => "cruise",
        };
        let limit_text = format!(
            "at the {} risk it may arrive no later than {}",
            risk.name(),
            pulse_text(last.0, last.1)
        );
        let short_text = format!(
            "the shortest way, the one point {station}, {} nm at {speed_text}, arrives in {} and {}",
            short.miles,
            pulse_text(short.arrives.0, short.arrives.1),
            inside_text(&short.inside, &circles)
        );
        let evidence = vec![short_text.clone(), limit_text.clone()];
        let Some(chosen) = chosen else {
            let words = format!(
                "The way: no way of {id} to {station} is in fuel ({}), so the shortest way is taken, as today: {short_text}.",
                short.fuel_words
            );
            return Some(WayChoice {
                words: Some(words),
                evidence,
                ..plain
            });
        };
        // A clearer way that arrives in time and is out of fuel.
        let clearer_out = weighed
            .iter()
            .any(|w| w.in_limit && !w.in_fuel && w.danger < chosen.danger);
        if chosen.waypoints == [station] {
            if short.danger == 0 {
                return Some(plain);
            }
            let why = if clearer_out {
                "no clearer way that arrives in time is in fuel"
            } else {
                "no way that arrives in time is clearer"
            };
            let words = format!(
                "The way: {short_text}; it lies in the enemy's reach by day, and it is taken because {why} ({limit_text})."
            );
            return Some(WayChoice {
                words: Some(words),
                evidence,
                ..plain
            });
        }
        let out_text = if clearer_out {
            " A clearer way that arrives in time is out of fuel."
        } else {
            ""
        };
        let words = format!(
            "The way: {id} is given the route {} at {speed_text}, {} nm, through {}: it arrives in {} and {}, where {short_text}; {limit_text}. Fuel for the way: {}.{out_text}",
            points_text(&chosen.waypoints),
            chosen.miles,
            points_text(&chosen.hexes),
            pulse_text(chosen.arrives.0, chosen.arrives.1),
            inside_text(&chosen.inside, &circles),
            chosen.fuel_words
        );
        Some(WayChoice {
            waypoints: chosen.waypoints,
            bent: true,
            words: Some(words),
            evidence,
        })
    }
}

impl Day<'_, '_> {
    /// Rules 1, 16 to 24: where `route` is the doctrine's route of `force` to
    /// its station `hex` alone, at a speed, and the way is the staff's to
    /// choose, the way chosen: its waypoints, and the words of the way joined
    /// to the route's reason. Where the shortest way is taken the route is as
    /// it was (rule 23), its reason gaining words only where that way lies in
    /// danger by day (rule 28) or no way is in fuel (rule 21).
    pub(crate) fn choose_the_way(
        &self,
        task: &Intent,
        force: &TaskForce,
        hex: Hex,
        risk: Risk,
        route: &mut Option<(Vec<Hex>, Option<SpeedSetting>, sim_core::command::Reason)>,
    ) {
        if !self.chooses_way(task, risk) {
            return;
        }
        let Some((points, Some(speed), reason)) = route.as_mut() else {
            return;
        };
        if points.as_slice() != [hex] {
            return;
        }
        let Some(choice) = self.way_to_station(task, force, hex, risk, *speed) else {
            return;
        };
        if let Some(words) = choice.words {
            reason.worked = format!("{} {words}", reason.worked);
            for what in choice.evidence {
                reason.evidence.push(sim_core::command::Evidence {
                    what,
                    day: self.today,
                });
            }
            reason
                .binding
                .push("the way to a station the staff chose: the least time by day inside the enemy's reach, within how late the risk lets it arrive and in fuel".to_string());
        }
        if choice.bent {
            *points = choice.waypoints;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn circle(q: i32, r: i32, hexes: u32) -> Circle {
        Circle {
            centre: Hex::new(q, r),
            hexes,
            words: String::new(),
        }
    }

    #[test]
    fn the_danger_of_a_hex() {
        let c = [circle(9, -15, 15), circle(13, -7, 8)];
        assert_eq!(danger_of(Hex::new(9, -15), &c), 16);
        assert_eq!(danger_of(Hex::new(13, -7), &c), 9);
        assert_eq!(danger_of(Hex::new(9, 0), &c), 2);
        assert_eq!(danger_of(Hex::new(-6, 0), &c), 1);
        assert_eq!(danger_of(Hex::new(-7, 0), &c), 0);
    }

    #[test]
    fn the_limits() {
        assert_eq!(
            latest_pulse(Risk::Balanced, 2, Pulse::Afternoon),
            (3, Pulse::Night)
        );
        assert_eq!(
            latest_pulse(Risk::Preserve, 2, Pulse::Night),
            (3, Pulse::Night)
        );
        assert!(no_later(2, Pulse::Morning, (2, Pulse::Morning)));
        assert!(!no_later(2, Pulse::Afternoon, (2, Pulse::Morning)));
    }

    #[test]
    fn the_staffs_clock() {
        let clock = StaffClock {
            first_day: 1,
            knots: Fixed::from_int(15),
        };
        // Step 5 at 180 nm, hour 12, is the night; step 6 the morning.
        assert_eq!(
            clock.pulse_at_nm(StaffClock::latest_nm(5)),
            (1, Pulse::Night)
        );
        assert_eq!(
            clock.pulse_at_nm(StaffClock::latest_nm(6)),
            (1, Pulse::Morning)
        );
        // The night of day 3 ends at hour 60, 900 nm: 23 steps.
        assert_eq!(clock.most_steps((3, Pulse::Night)), 23);
    }

    #[test]
    fn a_search_with_no_danger_is_the_engines_way() {
        let start = Hex::new(4, 0);
        let station = Hex::new(0, 0);
        let field = |t: Hex| -> BTreeMap<Hex, u32> {
            (-6..=6)
                .flat_map(|q| (-6..=6).map(move |r| Hex::new(q, r)))
                .map(|h| (h, h.distance(t)))
                .collect()
        };
        let ways = search(
            start,
            station,
            4,
            6,
            &field(start),
            &field(station),
            &|_, _| true,
            &|_, _| 0,
        );
        assert_eq!(ways.len(), 3);
        for w in ways {
            assert_eq!(
                w.hexes,
                vec![
                    Hex::new(3, 0),
                    Hex::new(2, 0),
                    Hex::new(1, 0),
                    Hex::new(0, 0)
                ]
            );
        }
    }
}
