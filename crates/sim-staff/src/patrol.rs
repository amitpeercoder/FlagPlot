//! The doctrine for "patrol the waters with submarines", at the area's level
//! (decision 0218, rules 11, 12, 34 and 54). It draws no random number and
//! keeps no state.
//!
//! Each morning, for each patrol intent in force, in priority order, whose
//! plan is released (its leaves are the same as any root's, `Day::leaves`):
//!
//! 1. **The waters** are the hexes on the map within the order's distance of
//!    its hex ([`crate::deny::waters`]).
//! 2. **The boats of the order**: those the forces limits on its chain name
//!    (their intersection), in boat number order; with none named, every boat
//!    of the side's own, in number order. A boat of place `n` in that list
//!    has station `n`.
//! 3. **The stations** are given so: the first is the centre where ships can
//!    enter it, else the navigable hex of the waters nearest the centre; each
//!    next is the navigable hex of the waters farthest from the nearest
//!    station already given; the lowest hex (q, then r) where hexes tie. When
//!    every navigable hex of the waters is a station, the stations begin
//!    again from the first. No station is on land.
//! 4. **A boat that cannot serve** (sunk, home, damaged, going home, held
//!    from the staff, held apart or before her day, or, where the order names
//!    no boat, named by the forces limit of another intent in force or used
//!    by a patrol order served before this one) is passed over, and her
//!    station goes to no one: stations do not move up.
//! 5. **The orders**: a boat whose station already is the station she would
//!    be given is given no order; the others are given the station order.
//!    One decision to give no order, with its reason, names each boat that
//!    keeps her station and each boat passed over with why.
//!
//! A boat given a station by an earlier patrol order this morning, whether
//! she was ordered or kept it, is passed over by a later one. The doctrine
//! reads the area command's picture of the side's own boats and the side's
//! command state, and nothing else.
//!
//! **The pattern of a patrol** (decision 0223 as amended on review, stage
//! A). Where the side's scenario gives it a submarine pattern, step 3 is
//! replaced by the pattern's, [`patterned`]; everything else stands. A side
//! with no pattern goes through [`stations`] as before.
//!
//! - **Lines** (`lines`): the approach is the enemy base on the side's map
//!   nearest the centre among those more than [`LINE_GAP_HEXES`] from it
//!   (lowest number on a tie). For its vector (q, r) from the centre the
//!   vector across is (-(q + 2r), 2q + r) divided by the greatest common
//!   divisor of its parts; the line is drawn from the centre along it and
//!   its opposite, segment by segment, each point at an even step rounded to
//!   the nearest hex ([`nearest_hex`]). The stations are the centre, then
//!   one farther along the first hand, then along the other, by turns, each
//!   [`LINE_SPACING_HEXES`] beyond the last, while they lie in the waters; a
//!   hex ships cannot enter is stepped over. The second line is the same
//!   line through the hex [`LINE_GAP_HEXES`] from the centre along the
//!   opposite of the approach, where that hex is in the waters. Boats left
//!   are spread as decision 0218 spreads them, counting the stations given.
//!   With no approach there is no line and the boats are spread.
//! - **Off bases** (`off_bases`): the enemy bases on the side's map in the
//!   waters or within one hex of a hex of them, nearest the centre first,
//!   lowest number on a tie; for each, the navigable hex next to it in the
//!   waters nearest the centre (lowest hex on a tie) not taken; a base with
//!   none is passed over. A second round, by the same test, to
//!   [`BOATS_PER_STATION`]; then the spread, counting the stations given.
//!   Waters with no such base are spread.

use sim_core::command::LimitKind;
use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{
    HoldTarget, Intent, IntentKind, PatrolStation, Plan, StaffOrder, SubmarinePattern,
};
use sim_core::hex::Hex;
use sim_core::ids::{EntityRef, IntentId, SubmarineId};
use sim_core::orders::{OrderKind, SubmarineOrder};
use sim_core::state::{Submarine, SubmarineCondition};
use sim_core::view::{PublicBase, SideView};

/// How far behind the first line the second lies, and the least distance
/// from the centre of the waters of an enemy base that gives the approach,
/// in hexes (decision 0223, stage A): 200 miles at 40 a hex. It rests on one
/// source and one battle (the research's rows of the Japanese lines of
/// 1942).
pub(crate) const LINE_GAP_HEXES: u32 = 5;

/// How far apart the stations of a line are, in hexes (decision 0223, stage
/// A): 30 miles is less than a hex of 40, so one. It rests on one source,
/// the Solomons lines of September 1942, with the same figure repeated in
/// 1944.
pub(crate) const LINE_SPACING_HEXES: u32 = 1;

/// How many stations an enemy base is given off it (decision 0223, stage
/// A): a judgement from rows that show one or two boats to a station.
pub(crate) const BOATS_PER_STATION: u32 = 2;

use crate::air::Day;
use crate::reasons::{hold_words, plan_for, serving};

/// One station of the waters, with how it was chosen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Station {
    pub(crate) hex: Hex,
    /// In words: "the centre of the waters", "station 2 of the waters, 2
    /// hexes from the nearest before it".
    pub(crate) how: String,
}

fn hexes(n: u32) -> String {
    if n == 1 {
        "1 hex".to_string()
    } else {
        format!("{n} hexes")
    }
}

/// The first `count` stations of `waters` about `centre` (decision 0218,
/// 54), each with how it was chosen. Fewer where the waters have no
/// navigable hex (none).
pub(crate) fn stations(
    view: &SideView<'_>,
    centre: Hex,
    waters: &[Hex],
    count: usize,
) -> Vec<Station> {
    let navigable: Vec<Hex> = waters
        .iter()
        .copied()
        .filter(|hex| view.hex(*hex).is_some_and(|data| data.is_navigable()))
        .collect();
    if navigable.is_empty() || count == 0 {
        return Vec::new();
    }
    let mut given: Vec<(Hex, String)> = Vec::new();
    // The first: the centre, else the navigable hex nearest it, the lowest
    // where they tie (`navigable` is in hex order and `min_by_key` keeps the
    // first).
    if navigable.contains(&centre) {
        given.push((centre, "the centre of the waters".to_string()));
    } else if let Some(nearest) = navigable.iter().copied().min_by_key(|h| h.distance(centre)) {
        given.push((
            nearest,
            format!(
                "the navigable hex of the waters nearest their centre ({}), {} from it",
                centre,
                hexes(nearest.distance(centre))
            ),
        ));
    }
    // Each next: the navigable hex farthest from the nearest station given;
    // the lowest where they tie.
    while given.len() < count.min(navigable.len()) {
        let mut best: Option<(u32, Hex)> = None;
        for hex in &navigable {
            if given.iter().any(|(g, _)| g == hex) {
                continue;
            }
            let near = given
                .iter()
                .map(|(g, _)| g.distance(*hex))
                .min()
                .unwrap_or(0);
            if best.is_none_or(|(far, _)| near > far) {
                best = Some((near, *hex));
            }
        }
        let Some((near, hex)) = best else { break };
        given.push((
            hex,
            format!(
                "station {} of the waters, {} from the nearest before it",
                given.len() + 1,
                hexes(near)
            ),
        ));
    }
    let unique = given.len();
    (0..count)
        .map(|n| {
            let (hex, how) = &given[n % unique];
            Station {
                hex: *hex,
                how: if n < unique {
                    how.clone()
                } else {
                    format!(
                        "station {} of the waters again, the boats being more than the hexes",
                        n % unique + 1
                    )
                },
            }
        })
        .collect()
}

/// The stations of a patrol whose side has a pattern (decision 0223, stage
/// A), with the staff's reading of the pattern in words: the approach and
/// the lines, or the bases and their stations, or why there is neither.
pub(crate) struct Patterned {
    pub(crate) stations: Vec<Station>,
    pub(crate) reading: String,
}

/// The greatest common divisor of two whole numbers, at least 1.
fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.max(1)
}

/// The hex nearest the point `from + step * i / n` (axial, `n` above 0),
/// the lowest hex (q, then r) where two are equally near. Exact in whole
/// numbers: the point's coordinates times `n` are whole, and the square of
/// the distance from a hex's centre to it, times `n` squared, is
/// `dq² + dr² + dq·dr` of the differences times `n`, the form
/// [`Hex::air_distance_nm`] uses. The nearest hex lies within one of the
/// point's own q and r rounded down, so those nine and their neighbours on
/// the far side are tried.
fn nearest_hex(from: Hex, step: (i64, i64), i: i64, n: i64) -> Hex {
    let x = i64::from(from.q) * n + step.0 * i;
    let y = i64::from(from.r) * n + step.1 * i;
    let fq = x.div_euclid(n);
    let fr = y.div_euclid(n);
    let mut best: Option<(i64, Hex)> = None;
    for q in fq - 1..=fq + 2 {
        for r in fr - 1..=fr + 2 {
            let dq = x - q * n;
            let dr = y - r * n;
            let err = dq * dq + dr * dr + dq * dr;
            let hex = Hex::new(
                i32::try_from(q).expect("a hex of the map"),
                i32::try_from(r).expect("a hex of the map"),
            );
            if best.is_none_or(|(e, h)| err < e || (err == e && hex < h)) {
                best = Some((err, hex));
            }
        }
    }
    best.expect("a nearest hex").1
}

/// The hexes of the line from `start` along `step` (a vector reduced by the
/// greatest common divisor of its parts), segment by segment: from
/// `start + k·step` to `start + (k+1)·step`, the N + 1 points at even steps
/// between two ends N hexes apart, each the nearest hex. In order, without
/// `start`, while `keep` holds; the first hex for which it fails ends the
/// line.
fn drawn(start: Hex, step: (i64, i64), keep: &dyn Fn(Hex) -> bool) -> Vec<Hex> {
    let n = i64::from(Hex::new(0, 0).distance(Hex::new(
        i32::try_from(step.0).expect("a step on the map"),
        i32::try_from(step.1).expect("a step on the map"),
    )));
    let mut found: Vec<Hex> = Vec::new();
    if n == 0 {
        return found;
    }
    // A line on the map ends within its width: no map is wider than this.
    for k in 0..4096_i64 {
        let from = Hex::new(
            start.q + i32::try_from(step.0 * k).expect("a hex of the map"),
            start.r + i32::try_from(step.1 * k).expect("a hex of the map"),
        );
        for i in 1..=n {
            let hex = nearest_hex(from, step, i, n);
            if !keep(hex) {
                return found;
            }
            if found.last() != Some(&hex) {
                found.push(hex);
            }
        }
    }
    found
}

fn hand_word(first: bool) -> &'static str {
    if first {
        "the first hand"
    } else {
        "the other hand"
    }
}

fn base_words(base: &PublicBase) -> String {
    format!("{} ({}) at {}", base.name, base.id, base.hex)
}

/// The stations of one line through `through` across `across`, in the order
/// they are given: `through`, then by turns one farther along the first
/// hand and one along the other, each at least [`LINE_SPACING_HEXES`]
/// beyond the last on its hand, while they lie in `waters`; a hex ships
/// cannot enter, or one `taken`, is stepped over. Each with its hand and its
/// distance from `through`.
fn line_stations(
    through: Hex,
    across: (i64, i64),
    in_waters: &dyn Fn(Hex) -> bool,
    navigable: &dyn Fn(Hex) -> bool,
    taken: &BTreeSet<Hex>,
) -> Vec<(Hex, Option<(bool, u32)>)> {
    let hand = |step: (i64, i64)| -> Vec<(Hex, u32)> {
        let mut last = 0;
        drawn(through, step, in_waters)
            .into_iter()
            .filter_map(|hex| {
                let far = hex.distance(through);
                if !navigable(hex) || taken.contains(&hex) || far < last + LINE_SPACING_HEXES {
                    return None;
                }
                last = far;
                Some((hex, far))
            })
            .collect()
    };
    let first = hand(across);
    let other = hand((-across.0, -across.1));
    let mut given: Vec<(Hex, Option<(bool, u32)>)> = Vec::new();
    if in_waters(through) && navigable(through) && !taken.contains(&through) {
        given.push((through, None));
    }
    let (mut a, mut b) = (first.into_iter(), other.into_iter());
    loop {
        let next_a = a.next();
        let next_b = b.next();
        if next_a.is_none() && next_b.is_none() {
            break;
        }
        if let Some((hex, far)) = next_a {
            given.push((hex, Some((true, far))));
        }
        if let Some((hex, far)) = next_b {
            given.push((hex, Some((false, far))));
        }
    }
    given
}

/// The navigable hex of `navigable` not yet given that is farthest from the
/// nearest of `given`, the lowest where they tie, with that distance: the
/// next station of the spread of decision 0218, 54.
fn farthest_next(navigable: &[Hex], given: &[Hex]) -> Option<(u32, Hex)> {
    let mut best: Option<(u32, Hex)> = None;
    for hex in navigable {
        if given.contains(hex) {
            continue;
        }
        let near = given.iter().map(|g| g.distance(*hex)).min().unwrap_or(0);
        if best.is_none_or(|(far, _)| near > far) {
            best = Some((near, *hex));
        }
    }
    best
}

/// Fills `given` to `count` stations by the spread of decision 0218 counting
/// the stations already given, each said with `said` and its distance from
/// the nearest before it; then, where the boats are more than the navigable
/// hexes, begins again from the first.
fn spread_on(
    navigable: &[Hex],
    given: &mut Vec<Station>,
    count: usize,
    said: &dyn Fn(u32) -> String,
) -> Vec<Station> {
    while given.len() < count.min(navigable.len()) {
        let hexes: Vec<Hex> = given.iter().map(|s| s.hex).collect();
        let Some((near, hex)) = farthest_next(navigable, &hexes) else {
            break;
        };
        given.push(Station {
            hex,
            how: said(near),
        });
    }
    let unique = given.len();
    if unique == 0 {
        return Vec::new();
    }
    (0..count)
        .map(|n| {
            let station = &given[n % unique];
            Station {
                hex: station.hex,
                how: if n < unique {
                    station.how.clone()
                } else {
                    format!(
                        "station {} of the waters again, the boats being more than the hexes",
                        n % unique + 1
                    )
                },
            }
        })
        .collect()
}

/// The first `count` stations of `waters` about `centre` by the side's
/// `pattern` (decision 0223 as amended on review, stage A), with the
/// staff's reading of it. Fewer where the waters have no navigable hex
/// (none).
pub(crate) fn patterned(
    view: &SideView<'_>,
    pattern: SubmarinePattern,
    centre: Hex,
    waters: &[Hex],
    count: usize,
) -> Patterned {
    let navigable: Vec<Hex> = waters
        .iter()
        .copied()
        .filter(|hex| view.hex(*hex).is_some_and(|data| data.is_navigable()))
        .collect();
    if navigable.is_empty() || count == 0 {
        return Patterned {
            stations: Vec::new(),
            reading: String::new(),
        };
    }
    let mut enemy: Vec<PublicBase> = view
        .enemy_bases()
        .filter(|base| base.side != view.side())
        .collect();
    enemy.sort_by_key(|base| (base.hex.distance(centre), base.id));
    match pattern {
        SubmarinePattern::Lines => lines(view, centre, waters, &navigable, &enemy, count),
        SubmarinePattern::OffBases => off_bases(view, centre, waters, &navigable, &enemy, count),
    }
}

/// The spread of decision 0218 alone, said with why the pattern gives no
/// station of its own.
fn spread_alone(
    view: &SideView<'_>,
    centre: Hex,
    waters: &[Hex],
    count: usize,
    why: &str,
) -> Patterned {
    let stations = stations(view, centre, waters, count)
        .into_iter()
        .map(|station| Station {
            hex: station.hex,
            how: format!("{}, spread over the waters", station.how),
        })
        .collect();
    Patterned {
        stations,
        reading: why.to_string(),
    }
}

fn lines(
    view: &SideView<'_>,
    centre: Hex,
    waters: &[Hex],
    navigable: &[Hex],
    enemy: &[PublicBase],
    count: usize,
) -> Patterned {
    let Some(base) = enemy
        .iter()
        .find(|base| base.hex.distance(centre) > LINE_GAP_HEXES)
    else {
        let why = match enemy.first() {
            Some(nearest) => format!(
                "There is no line: no enemy base on the side's map lies more than {} from the centre {centre}, so there is no approach (the nearest, {}, is {} from it).",
                hexes(LINE_GAP_HEXES),
                base_words(nearest),
                hexes(nearest.hex.distance(centre))
            ),
            None => format!(
                "There is no line: the side's map has no enemy base, so there is no approach to the centre {centre}."
            ),
        };
        return spread_alone(view, centre, waters, count, &why);
    };
    let in_waters = |hex: Hex| waters.binary_search(&hex).is_ok();
    let is_navigable = |hex: Hex| navigable.binary_search(&hex).is_ok();
    let v = (
        i64::from(base.hex.q) - i64::from(centre.q),
        i64::from(base.hex.r) - i64::from(centre.r),
    );
    let raw = (-(v.0 + 2 * v.1), 2 * v.0 + v.1);
    let g = gcd(raw.0, raw.1);
    let across = (raw.0 / g, raw.1 / g);
    let gv = gcd(v.0, v.1);
    let away = (-v.0 / gv, -v.1 / gv);
    let from = base_words(base);
    let mut given: Vec<Station> = Vec::new();
    let mut taken: BTreeSet<Hex> = BTreeSet::new();
    for (hex, place) in line_stations(centre, across, &in_waters, &is_navigable, &taken) {
        if given.len() >= count {
            break;
        }
        taken.insert(hex);
        given.push(Station {
            hex,
            how: match place {
                None => format!("the centre of the first line across the approach from {from}"),
                Some((first, far)) => format!(
                    "on the first line across the approach from {from}, {} from its centre on {}",
                    hexes(far),
                    hand_word(first)
                ),
            },
        });
    }
    let first_count = given.len();
    // The second line's hex: the one LINE_GAP_HEXES from the centre along
    // the approach away from the enemy, by the same drawing.
    let gap = drawn(centre, away, &|hex| hex.distance(centre) <= LINE_GAP_HEXES)
        .into_iter()
        .find(|hex| hex.distance(centre) == LINE_GAP_HEXES);
    let second_hex = gap.filter(|hex| in_waters(*hex));
    let mut second_count = 0;
    if given.len() < count {
        if let Some(through) = second_hex {
            for (hex, place) in line_stations(through, across, &in_waters, &is_navigable, &taken) {
                if given.len() >= count {
                    break;
                }
                taken.insert(hex);
                second_count += 1;
                given.push(Station {
                    hex,
                    how: match place {
                        None => format!(
                            "the centre of the second line across the approach from {from}, {} behind the first, away from the enemy",
                            hexes(LINE_GAP_HEXES)
                        ),
                        Some((first, far)) => format!(
                            "on the second line across the approach from {from}, {} from its centre at {through} on {}",
                            hexes(far),
                            hand_word(first)
                        ),
                    },
                });
            }
        }
    }
    let second_words = match (gap, second_hex) {
        (_, Some(through)) => format!("the second line runs through {through}"),
        (Some(outside), None) => format!(
            "the second line would run through {outside}, which is outside the waters, so there is none"
        ),
        (None, None) => "there is no hex for a second line".to_string(),
    };
    let reading = format!(
        "The approach is from {from}, {} from the centre {centre}, the nearest enemy base more than {} from it. The first line runs through {centre} across it and has {first_count} station(s) in the waters; {second_words}{}.",
        hexes(base.hex.distance(centre)),
        hexes(LINE_GAP_HEXES),
        if second_hex.is_some() {
            format!(", with {second_count} station(s) given on it")
        } else {
            String::new()
        }
    );
    let full = if second_hex.is_some() {
        "the lines being full"
    } else {
        "the line being full"
    };
    let stations = spread_on(navigable, &mut given, count, &|near| {
        format!(
            "spread over the waters, {full}: the navigable hex farthest from the nearest station given, {} from it",
            hexes(near)
        )
    });
    Patterned { stations, reading }
}

fn off_bases(
    view: &SideView<'_>,
    centre: Hex,
    waters: &[Hex],
    navigable: &[Hex],
    enemy: &[PublicBase],
    count: usize,
) -> Patterned {
    let bases: Vec<&PublicBase> = enemy
        .iter()
        .filter(|base| waters.iter().any(|hex| hex.distance(base.hex) <= 1))
        .collect();
    if bases.is_empty() {
        return spread_alone(
            view,
            centre,
            waters,
            count,
            "No enemy base on the side's map lies in the waters or within a hex of them, so the boats are spread over the waters.",
        );
    }
    let mut given: Vec<Station> = Vec::new();
    let mut taken: BTreeSet<Hex> = BTreeSet::new();
    let mut had: BTreeMap<sim_core::ids::BaseId, u32> = BTreeMap::new();
    // Why a base was given fewer stations than its turns asked for, said
    // when its turn found no hex.
    let mut short: BTreeMap<sim_core::ids::BaseId, String> = BTreeMap::new();
    for round in 1..=BOATS_PER_STATION {
        for base in &bases {
            if given.len() >= count || short.contains_key(&base.id) {
                continue;
            }
            let Some(hex) = base
                .hex
                .neighbors()
                .into_iter()
                .filter(|hex| navigable.binary_search(hex).is_ok() && !taken.contains(hex))
                .min_by_key(|hex| (hex.distance(centre), *hex))
            else {
                let next_to = base
                    .hex
                    .neighbors()
                    .into_iter()
                    .filter(|hex| navigable.binary_search(hex).is_ok())
                    .count();
                let n = had.get(&base.id).copied().unwrap_or(0);
                let which = match n {
                    0 => "",
                    1 => " second",
                    _ => " further",
                };
                short.insert(
                    base.id,
                    if u32::try_from(next_to).unwrap_or(u32::MAX) <= n {
                        format!("no{which} hex next to it that ships can enter lies in the waters")
                    } else {
                        format!(
                            "each{which} hex next to it that ships can enter in the waters is taken"
                        )
                    },
                );
                continue;
            };
            taken.insert(hex);
            *had.entry(base.id).or_insert(0) += 1;
            given.push(Station {
                hex,
                how: format!(
                    "{} {}, the navigable hex next to it in the waters nearest the centre, {} from it{}",
                    if round == 1 { "off" } else { "a second station off" },
                    base_words(base),
                    hexes(hex.distance(centre)),
                    if round == 1 { "" } else { ", of those not taken" }
                ),
            });
        }
    }
    let listed: Vec<String> = bases
        .iter()
        .map(|base| {
            let n = had.get(&base.id).copied().unwrap_or(0);
            let words = base_words(base);
            match (n, short.get(&base.id)) {
                (0, Some(why)) => format!("{words} is passed over: {why}"),
                (n, Some(why)) => {
                    format!("{words} is given {n} station(s) and not {BOATS_PER_STATION}: {why}")
                }
                (n, None) if n < BOATS_PER_STATION => {
                    format!("{words} is given {n} station(s), the boats being fewer")
                }
                (n, None) => format!("{words} is given {n} station(s)"),
            }
        })
        .collect();
    let reading = format!(
        "The enemy bases on the side's map in the waters or within a hex of them, nearest the centre {centre} first: {}.",
        listed.join("; ")
    );
    let stations = spread_on(navigable, &mut given, count, &|near| {
        format!(
            "spread over the waters, every base in or next to them having its stations: the navigable hex farthest from the nearest station given, {} from it",
            hexes(near)
        )
    });
    Patterned { stations, reading }
}

/// Why a boat cannot serve a patrol order this morning, if she cannot.
fn cannot_serve(
    day: &Day<'_, '_>,
    boat: &Submarine,
    patrol: &Intent,
    named: bool,
    used: &BTreeMap<SubmarineId, IntentId>,
) -> Option<String> {
    let today = day.today;
    match boat.condition {
        SubmarineCondition::Sunk => return Some("sunk".to_string()),
        SubmarineCondition::Home => return Some("home".to_string()),
        SubmarineCondition::Damaged | SubmarineCondition::Sound => {}
    }
    if !day.area.may_use(EntityRef::Submarine(boat.id)) {
        return Some(match (boat.enters_day, boat.available_from) {
            (Some(enters), _) if enters > today => format!("enters on day {enters}"),
            (_, Some(from)) => format!("may be used from day {from}"),
            (Some(enters), None) => format!("enters on day {enters}"),
            (None, None) => "may not be used".to_string(),
        });
    }
    if boat.condition == SubmarineCondition::Damaged {
        return Some("damaged".to_string());
    }
    if boat.passage.is_some_and(|passage| passage.home) {
        return Some("going home".to_string());
    }
    let target = HoldTarget::Submarine(boat.id);
    if let Some(hold) = day.state.hold_of(target) {
        return Some(format!(
            "held by the commander: {}",
            hold_words(target, *hold)
        ));
    }
    if let Some(earlier) = used.get(&boat.id) {
        return Some(format!("used by {earlier}"));
    }
    if !named {
        if let Some(other) = day
            .state
            .intents_in_force()
            .filter(|other| other.id != patrol.id)
            .find(|other| {
                other
                    .limits
                    .forces
                    .as_ref()
                    .is_some_and(|forces| forces.submarines.contains(&boat.id))
            })
        {
            return Some(format!("used by {}", other.id));
        }
    }
    None
}

impl Day<'_, '_> {
    /// The patrol doctrine for the morning (decision 0218, 11, 12, 34 and
    /// 54).
    pub(crate) fn patrols(&mut self) {
        let ends = self.in_priority_order(|i| matches!(i.kind, IntentKind::Patrol { .. }));
        let mut used: BTreeMap<SubmarineId, IntentId> = BTreeMap::new();
        for id in ends {
            let Some(end) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            if !self.released(&end) {
                continue;
            }
            let Some(plan) = self.plan_of(&end) else {
                continue;
            };
            self.patrol(&end, &plan, &mut used);
        }
    }

    #[allow(clippy::too_many_lines)]
    fn patrol(&mut self, end: &Intent, plan: &Plan, used: &mut BTreeMap<SubmarineId, IntentId>) {
        let IntentKind::Patrol { hex, distance } = end.kind else {
            return;
        };
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        let area = self.area;
        let waters = crate::deny::waters(area, hex, distance);
        let waters_text = format!(
            "the waters within {distance} {} of {hex} ({} hex(es) on the map)",
            if distance == 1 { "hex" } else { "hexes" },
            waters.len()
        );
        // The boats of the order, in boat number order.
        let named_set = self.state.submarines_on_chain(end.id);
        let named = named_set.is_some();
        let mut boats: Vec<Submarine> = area
            .own_submarines()
            .filter(|boat| named_set.as_ref().is_none_or(|set| set.contains(&boat.id)))
            .cloned()
            .collect();
        boats.sort_by_key(|boat| boat.id);
        if boats.is_empty() {
            let reason = serving(end, plan)
                .evidence(
                    format!("{} has no submarine of the side's to use", end.id),
                    today,
                )
                .worked(format!(
                    "{} has no submarine of the side's own to use, so the staff gives no station.",
                    plan_for(area, end, plan)
                ))
                .would_change("a boat of the side's own, or a forces limit that names one")
                .speaks_of(&self.state, LimitKind::Forces)
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        }
        // Decision 0223, stage A: the side's pattern, where its scenario
        // gives one; else the spread of decision 0218, as before.
        let (stations, reading) = match area.submarine_doctrine().pattern {
            None => (stations(area, hex, &waters, boats.len()), None),
            Some(pattern) => {
                let patterned = patterned(area, pattern, hex, &waters, boats.len());
                (patterned.stations, Some(patterned.reading))
            }
        };
        if in_scope && !stations.is_empty() {
            if let Some(reading) = &reading {
                self.lines.push(format!("{}: {reading}", end.id));
            }
        }
        if stations.is_empty() {
            let reason = serving(end, plan)
                .evidence(
                    format!("{waters_text}: no hex of them is one ships can enter"),
                    today,
                )
                .worked(format!(
                    "{} has no hex in {waters_text} that ships can enter, so the staff gives no station.",
                    plan_for(area, end, plan)
                ))
                .would_change("waters with a hex that ships can enter")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        }
        let mut orders: Vec<StaffOrder> = Vec::new();
        let mut keeps: Vec<String> = Vec::new();
        let mut passed: Vec<String> = Vec::new();
        for (index, boat) in boats.iter().enumerate() {
            let place = u32::try_from(index + 1).unwrap_or(u32::MAX);
            let station = &stations[index];
            let who = format!("{} ({})", boat.id, boat.name);
            if let Some(why) = cannot_serve(self, boat, end, named, used) {
                passed.push(format!(
                    "{who} is passed over, {why}, and her place {place} station {} goes to no one",
                    station.hex
                ));
                continue;
            }
            used.insert(boat.id, end.id);
            if in_scope {
                self.patrol_stations.push(PatrolStation {
                    boat: boat.id,
                    hex: station.hex,
                    place,
                });
            }
            if boat.station == station.hex {
                keeps.push(format!(
                    "{} ({}) keeps her station at {}",
                    boat.id, boat.name, station.hex
                ));
                if in_scope {
                    self.lines
                        .push(format!("{} keeps her station at {}.", boat.id, station.hex));
                }
                continue;
            }
            let mut builder = serving(end, plan)
                .evidence(
                    format!(
                        "{who} is in place {place} of the boats of {}; she is at {} with her station at {}",
                        end.id, boat.hex, boat.station
                    ),
                    today,
                )
                .binding(waters_text.clone());
            builder = match &reading {
                None => builder
                    .worked(format!(
                        "{} patrols {waters_text}. {who} is in place {place} of the boats it may use, so her station is {}, {}: the staff orders her to the station.",
                        end.id, station.hex, station.how
                    ))
                    .would_change("another boat in a place before hers, or a hold on her")
                    .would_change("the waters or the boats the order names"),
                // Decision 0223, stage A: the station by the side's pattern,
                // with the staff's reading of it.
                Some(reading) => builder
                    .worked(format!(
                        "{} patrols {waters_text}. {who} is in place {place} of the boats it may use, so her station is {}, {}: the staff orders her to the station. {reading}",
                        end.id, station.hex, station.how
                    ))
                    .would_change("another boat in a place before hers, or a hold on her")
                    .would_change("the waters or the boats the order names")
                    .would_change("the enemy bases on the side's map")
                    .speaks_of(&self.state, LimitKind::Forces),
            };
            let reason = builder.done();
            if in_scope {
                self.lines.push(format!(
                    "{} would be ordered to station {} ({}).",
                    boat.id, station.hex, station.how
                ));
            }
            orders.push(StaffOrder {
                kind: OrderKind::Submarine {
                    target: boat.id,
                    order: SubmarineOrder::Station(station.hex),
                },
                intent: end.id,
                reason,
            });
        }
        self.target(in_scope).orders.extend(orders);
        if keeps.is_empty() && passed.is_empty() {
            return;
        }
        let mut parts = Vec::new();
        if !keeps.is_empty() {
            parts.push(keeps.join("; "));
        }
        if !passed.is_empty() {
            parts.push(passed.join("; "));
        }
        let reason = serving(end, plan)
            .evidence(
                format!(
                    "{} keeps {} boat(s) on their stations and passes over {}",
                    end.id,
                    keeps.len(),
                    passed.len()
                ),
                today,
            )
            .worked(format!(
                "{} patrols {waters_text}. {}. The station of a boat passed over goes to no one.",
                end.id,
                parts.join(". ")
            ))
            .would_change("a boat that could serve, or a hold released")
            .done();
        self.target(in_scope).no_orders.push(reason);
    }
}
