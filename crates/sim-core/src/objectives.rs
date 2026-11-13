//! Objectives and results (decision 0054).
//!
//! A scenario may give each side a mission and objectives. An objective is a
//! count of something the engine already records, read at the end of the
//! game, and the points one of it is worth. The result is a function of the
//! final truth and the event log. Nothing here is state: the objectives are
//! the scenario's fixed configuration, held by the world beside the
//! reference data, and are not in the state hash, because no rule reads
//! them or the result (decision 5). A replay gives the same result because
//! it gives the same truth and the same log.
//!
//! The work is in three steps, each a pure function:
//!
//! 1. [`Tally::from_truth`] counts what each side achieved, from the final
//!    truth and the event log.
//! 2. [`score`] turns the two sides' tallies into points by objective, a
//!    margin and a level. It takes plain counts, so the properties of the
//!    scale can be checked on made-up counts.
//! 3. [`known_progress`] counts the same objectives from what one side may
//!    know: its own units and the events it may see (decision 0047). It is
//!    given no truth, only a [`SideView`](crate::view::SideView) feeds it.
//!
//! Fixed point only, ordered collections only, no floats.
//!
//! # What is counted for each kind
//!
//! - `ships_sunk`: enemy ships whose location is sunk, by ship kind.
//! - `ships_crippled`: enemy ships not sunk (afloat, or in port) with
//!   propulsion out or the flight deck out, by ship kind. A degraded system
//!   does not count.
//! - `aircraft_destroyed`: the aircraft of the enemy's air groups lost for
//!   good, from six events: `AircraftLost` (shot down, in the air),
//!   `AirGroupLost` (lost with a sunk ship, ready and damaged),
//!   `AircraftDestroyedOnGround` (ready), `DamagedAircraftDestroyedOnGround`
//!   (damaged), `AircraftWorn::written_off` and `AircraftDitched::ditched`
//!   (out of fuel on the way home, decision 5 of 0059: counted for the enemy
//!   though no one claims it) and `AircraftDitchedNoDeck` (no deck, decision
//!   9 of 0059). Aircraft that land elsewhere are not lost and are not
//!   counted. Not counted: `AircraftWorn`
//!   `worn` (damaged and repaired later, not lost), `PilotsLost` (pilots, not
//!   aircraft; the aircraft's own event counts the aircraft), `AircraftRepaired`
//!   and every event that adds aircraft.
//! - `cargo_landed`: the tons in the `CargoUnloaded` events of the side's
//!   own ships at the named base, less the tons in their `CargoLoaded`
//!   events there, supply and fuel together, never below nothing (decision
//!   0067). Cargo put ashore and taken off again was not landed.
//! - `base_open`: the days on which the named base ended the day with supply
//!   above zero and at least one ready aircraft of an air group at it, from
//!   the `BaseAtDayEnd` event of each day (decision 0067). With
//!   `closed = true`, the days on which it did not. The points are the
//!   objective's points times those days over the days resolved.

use core::fmt;
use std::collections::BTreeMap;

use crate::events::{base_is_open, EventKind, EventLog};
use crate::fixed::Fixed;
use crate::ids::BaseId;
use crate::refdata::{RefData, ShipKind};
use crate::side_events::SideEvent;
use crate::state::{Ship, ShipLocation, ShipSystem, Side, SystemState, TruthState};

/// The points for each kind of ship. A kind not listed is worth nothing.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ShipPoints(pub BTreeMap<ShipKind, Fixed>);

impl ShipPoints {
    pub fn of(&self, kind: ShipKind) -> Fixed {
        self.0.get(&kind).copied().unwrap_or(Fixed::ZERO)
    }
}

/// One objective of one side.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Objective {
    /// Enemy ships sunk, points for each kind of ship.
    ShipsSunk(ShipPoints),
    /// Enemy ships afloat at the end with propulsion or the flight deck out,
    /// points for each kind of ship.
    ShipsCrippled(ShipPoints),
    /// Enemy aircraft lost for good, points for one aircraft.
    AircraftDestroyed { points: Fixed },
    /// Tons of supply and fuel the side unloaded at a named base, points for
    /// one ton.
    CargoLanded { base: BaseId, points_per_t: Fixed },
    /// 1 if a named base has supply above zero and an aircraft ready at the
    /// end (`closed` false), or the opposite (`closed` true).
    BaseOpen {
        base: BaseId,
        closed: bool,
        points: Fixed,
    },
}

impl Objective {
    /// The kind's name as the scenario file and the briefing write it.
    pub const fn name(&self) -> &'static str {
        match self {
            Objective::ShipsSunk(_) => "ships_sunk",
            Objective::ShipsCrippled(_) => "ships_crippled",
            Objective::AircraftDestroyed { .. } => "aircraft_destroyed",
            Objective::CargoLanded { .. } => "cargo_landed",
            Objective::BaseOpen { .. } => "base_open",
        }
    }

    /// The base the objective names, if it names one.
    pub const fn base(&self) -> Option<BaseId> {
        match self {
            Objective::CargoLanded { base, .. } | Objective::BaseOpen { base, .. } => Some(*base),
            _ => None,
        }
    }
}

/// A side's mission and objectives.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SideObjectives {
    /// One or two sentences.
    pub mission: String,
    pub objectives: Vec<Objective>,
}

/// The margins that make a victory, rising and above zero.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Levels {
    pub minor: Fixed,
    pub major: Fixed,
    pub decisive: Fixed,
}

impl Levels {
    /// `None` if the levels are not above zero and rising.
    pub fn new(minor: Fixed, major: Fixed, decisive: Fixed) -> Option<Levels> {
        (Fixed::ZERO < minor && minor < major && major < decisive).then_some(Levels {
            minor,
            major,
            decisive,
        })
    }
}

/// The scenario's objectives for both sides, with the levels and the note of
/// how the day went in history.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Objectives {
    pub allied: SideObjectives,
    pub japanese: SideObjectives,
    pub levels: Levels,
    /// Words, not a computed score (decision 9). Shown with the result.
    pub history: Option<String>,
}

impl Objectives {
    pub fn side(&self, side: Side) -> &SideObjectives {
        match side {
            Side::Allied => &self.allied,
            Side::Japanese => &self.japanese,
        }
    }

    /// Names of the bases the objectives refer to, for a loader to check.
    pub fn bases_named(&self) -> Vec<BaseId> {
        Side::ALL
            .into_iter()
            .flat_map(|side| {
                self.side(side)
                    .objectives
                    .iter()
                    .filter_map(Objective::base)
            })
            .collect()
    }
}

/// How far a side won or lost.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Level {
    Minor,
    Major,
    Decisive,
}

impl Level {
    pub const fn name(self) -> &'static str {
        match self {
            Level::Minor => "minor",
            Level::Major => "major",
            Level::Decisive => "decisive",
        }
    }
}

/// A side's result.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outcome {
    Victory(Level),
    Defeat(Level),
    Stalemate,
}

impl Outcome {
    /// The other side's result.
    pub const fn mirror(self) -> Outcome {
        match self {
            Outcome::Victory(level) => Outcome::Defeat(level),
            Outcome::Defeat(level) => Outcome::Victory(level),
            Outcome::Stalemate => Outcome::Stalemate,
        }
    }

    /// `decisive victory`, `minor defeat`, `stalemate` and so on.
    pub fn name(self) -> String {
        match self {
            Outcome::Victory(level) => format!("{} victory", level.name()),
            Outcome::Defeat(level) => format!("{} defeat", level.name()),
            Outcome::Stalemate => "stalemate".to_string(),
        }
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name())
    }
}

/// The level of a margin (decision 4). A margin of at least a level is that
/// level, so a margin exactly equal to one is it and a margin just below is
/// the level under. A margin strictly between minus `minor` and `minor` is a
/// stalemate. A defeat is the victory of the other side at the same level.
pub fn outcome_of(margin: Fixed, levels: &Levels) -> Outcome {
    let size = if margin.is_negative() {
        Fixed::ZERO - margin
    } else {
        margin
    };
    let level = if size >= levels.decisive {
        Level::Decisive
    } else if size >= levels.major {
        Level::Major
    } else if size >= levels.minor {
        Level::Minor
    } else {
        return Outcome::Stalemate;
    };
    if margin.is_negative() {
        Outcome::Defeat(level)
    } else {
        Outcome::Victory(level)
    }
}

/// What one side achieved, counted from the truth and the event log.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Tally {
    /// Enemy ships sunk, by kind.
    pub sunk: BTreeMap<ShipKind, u32>,
    /// Enemy ships not sunk with propulsion or flight deck out, by kind.
    pub crippled: BTreeMap<ShipKind, u32>,
    /// Enemy aircraft lost for good.
    pub aircraft: u32,
    /// Tons unloaded by the side's ships less tons they loaded, by base.
    /// It can be below zero here; the score never is.
    pub cargo_t: BTreeMap<BaseId, Fixed>,
    /// For each base, the days counted and the days it ended open.
    pub base_days: BTreeMap<BaseId, BaseDays>,
}

/// The days a base was counted at their end, and on how many it was open:
/// supply above zero and an aircraft ready (decision 0067).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BaseDays {
    pub days: u32,
    pub open: u32,
}

impl BaseDays {
    /// Adds one day's end.
    pub fn note(&mut self, supply_t: Fixed, aircraft_ready: u32) {
        self.days += 1;
        if base_is_open(supply_t, aircraft_ready) {
            self.open += 1;
        }
    }

    /// The days the base was as the objective wants it: open, or with
    /// `closed`, shut.
    pub fn as_wanted(self, closed: bool) -> u32 {
        if closed {
            self.days - self.open
        } else {
            self.open
        }
    }

    /// The points of a `base_open` objective: its points times the days as
    /// wanted over the days counted. Nothing before a day is counted.
    pub fn points(self, closed: bool, points: Fixed) -> Fixed {
        if self.days == 0 {
            return Fixed::ZERO;
        }
        points * i64::from(self.as_wanted(closed)) / Fixed::from_int(i64::from(self.days))
    }
}

/// The tons that count as landed: never below nothing.
fn landed(net_t: Fixed) -> Fixed {
    if net_t.is_negative() {
        Fixed::ZERO
    } else {
        net_t
    }
}

/// The tallies of both sides, from the final truth and the log.
pub fn tallies(truth: &TruthState, log: &EventLog, ref_data: &RefData) -> BTreeMap<Side, Tally> {
    let mut out: BTreeMap<Side, Tally> = Side::ALL
        .into_iter()
        .map(|side| (side, Tally::default()))
        .collect();

    // Ships: a side's tally holds the enemy's losses.
    for ship in truth.ships.values() {
        let Some(kind) = ref_data.ship_classes.get(&ship.class).map(|c| c.kind) else {
            continue;
        };
        let taker = other(ship.side);
        let tally = out.get_mut(&taker).expect("both sides");
        if ship.location == ShipLocation::Sunk {
            *tally.sunk.entry(kind).or_insert(0) += 1;
        } else if is_crippled(ship) {
            *tally.crippled.entry(kind).or_insert(0) += 1;
        }
    }

    // Aircraft and cargo, from the log.
    for event in log.iter() {
        match &event.kind {
            EventKind::AircraftLost {
                air_group,
                aircraft,
                ..
            }
            | EventKind::AirGroupLost {
                air_group,
                aircraft,
                ..
            }
            | EventKind::AircraftDestroyedOnGround {
                air_group,
                aircraft,
                ..
            }
            | EventKind::DamagedAircraftDestroyedOnGround {
                air_group,
                aircraft,
                ..
            }
            | EventKind::AircraftWorn {
                air_group,
                written_off: aircraft,
                ..
            }
            | EventKind::AircraftDitched {
                air_group,
                ditched: aircraft,
                ..
            }
            | EventKind::AircraftDitchedNoDeck {
                air_group,
                aircraft,
                ..
            } => {
                if let Some(group) = truth.air_groups.get(air_group) {
                    out.get_mut(&other(group.side))
                        .expect("both sides")
                        .aircraft += *aircraft;
                }
            }
            EventKind::CargoUnloaded {
                ship,
                base,
                unloaded_t,
                ..
            } => {
                if let Some(ship) = truth.ships.get(ship) {
                    let total = out
                        .get_mut(&ship.side)
                        .expect("both sides")
                        .cargo_t
                        .entry(*base)
                        .or_insert(Fixed::ZERO);
                    *total += *unloaded_t;
                }
            }
            // Cargo taken off again at the base was not landed.
            EventKind::CargoLoaded {
                ship,
                base,
                loaded_t,
                ..
            } => {
                if let Some(ship) = truth.ships.get(ship) {
                    let total = out
                        .get_mut(&ship.side)
                        .expect("both sides")
                        .cargo_t
                        .entry(*base)
                        .or_insert(Fixed::ZERO);
                    *total -= *loaded_t;
                }
            }
            // Bases: each day's end, for both sides, whichever side holds
            // them.
            EventKind::BaseAtDayEnd {
                base,
                supply_t,
                aircraft_ready,
            } => {
                for tally in out.values_mut() {
                    tally
                        .base_days
                        .entry(*base)
                        .or_default()
                        .note(*supply_t, *aircraft_ready);
                }
            }
            _ => {}
        }
    }
    out
}

fn other(side: Side) -> Side {
    match side {
        Side::Allied => Side::Japanese,
        Side::Japanese => Side::Allied,
    }
}

fn is_crippled(ship: &Ship) -> bool {
    ship.systems.get(ShipSystem::Propulsion) == SystemState::Out
        || ship.systems.get(ShipSystem::FlightDeck) == SystemState::Out
}

/// One objective's count and points.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectiveScore {
    /// `ships_sunk`, `ships_crippled`, `aircraft_destroyed`, `cargo_landed`
    /// or `base_open`.
    pub kind: &'static str,
    pub base: Option<BaseId>,
    /// For `base_open`: whether the side wanted the base shut.
    pub closed: bool,
    /// Ships, aircraft, tons, or for a base the days it was as wanted.
    pub count: Fixed,
    /// For `base_open`: the days counted, which `count` is out of.
    pub days: Option<u32>,
    /// For the two ship objectives: ships by kind.
    pub by_kind: BTreeMap<ShipKind, u32>,
    pub points: Fixed,
}

/// One side's result.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SideResult {
    pub objectives: Vec<ObjectiveScore>,
    pub total: Fixed,
    /// This side's points less the enemy's.
    pub margin: Fixed,
    pub outcome: Outcome,
}

/// The umpire's result for both sides.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameResult {
    pub allied: SideResult,
    pub japanese: SideResult,
    pub history: Option<String>,
}

impl GameResult {
    pub fn side(&self, side: Side) -> &SideResult {
        match side {
            Side::Allied => &self.allied,
            Side::Japanese => &self.japanese,
        }
    }
}

fn ship_count(by_kind: &BTreeMap<ShipKind, u32>, points: &ShipPoints) -> (Fixed, Fixed) {
    let mut count = Fixed::ZERO;
    let mut total = Fixed::ZERO;
    for (kind, ships) in by_kind {
        let n = Fixed::from_int(i64::from(*ships));
        count += n;
        total += n * points.of(*kind);
    }
    (count, total)
}

/// One side's points by objective for one tally.
fn score_side(side: &SideObjectives, tally: &Tally) -> Vec<ObjectiveScore> {
    side.objectives
        .iter()
        .map(|objective| {
            let mut score = ObjectiveScore {
                kind: objective.name(),
                base: objective.base(),
                closed: false,
                count: Fixed::ZERO,
                days: None,
                by_kind: BTreeMap::new(),
                points: Fixed::ZERO,
            };
            match objective {
                Objective::ShipsSunk(points) => {
                    (score.count, score.points) = ship_count(&tally.sunk, points);
                    score.by_kind = tally.sunk.clone();
                }
                Objective::ShipsCrippled(points) => {
                    (score.count, score.points) = ship_count(&tally.crippled, points);
                    score.by_kind = tally.crippled.clone();
                }
                Objective::AircraftDestroyed { points } => {
                    score.count = Fixed::from_int(i64::from(tally.aircraft));
                    score.points = score.count * *points;
                }
                Objective::CargoLanded { base, points_per_t } => {
                    score.count = landed(tally.cargo_t.get(base).copied().unwrap_or(Fixed::ZERO));
                    score.points = score.count * *points_per_t;
                }
                Objective::BaseOpen {
                    base,
                    closed,
                    points,
                } => {
                    let days = tally.base_days.get(base).copied().unwrap_or_default();
                    score.closed = *closed;
                    score.count = Fixed::from_int(i64::from(days.as_wanted(*closed)));
                    score.days = Some(days.days);
                    score.points = days.points(*closed, *points);
                }
            }
            score
        })
        .collect()
}

/// Points, margin and level for both sides from their tallies. Pure: the
/// same objectives and tallies give the same result.
pub fn score(objectives: &Objectives, tallies: &BTreeMap<Side, Tally>) -> GameResult {
    let empty = Tally::default();
    let tally = |side: Side| tallies.get(&side).unwrap_or(&empty);
    let allied = score_side(&objectives.allied, tally(Side::Allied));
    let japanese = score_side(&objectives.japanese, tally(Side::Japanese));
    let sum =
        |scores: &[ObjectiveScore]| scores.iter().fold(Fixed::ZERO, |total, s| total + s.points);
    let (allied_total, japanese_total) = (sum(&allied), sum(&japanese));
    let allied_margin = allied_total - japanese_total;
    let outcome = outcome_of(allied_margin, &objectives.levels);
    GameResult {
        allied: SideResult {
            objectives: allied,
            total: allied_total,
            margin: allied_margin,
            outcome,
        },
        japanese: SideResult {
            objectives: japanese,
            total: japanese_total,
            margin: Fixed::ZERO - allied_margin,
            outcome: outcome.mirror(),
        },
        history: objectives.history.clone(),
    }
}

/// The result of a game from the final truth and the log. For the umpire
/// only.
pub fn result(
    objectives: &Objectives,
    truth: &TruthState,
    log: &EventLog,
    ref_data: &RefData,
) -> GameResult {
    score(objectives, &tallies(truth, log, ref_data))
}

// ---------------------------------------------------------------------
// What a side knows of its progress.
// ---------------------------------------------------------------------

/// How a side's known count was made.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KnownBasis {
    /// From the side's own units and events: as exact as the truth.
    Exact,
    /// From the claims the side received (`ClaimReceived`, decision 0056):
    /// the number of enemy ships it believes sunk, and of enemy aircraft it
    /// believes shot down or destroyed on the ground. The claims can be
    /// wrong, so the count can be above the truth. A claim of ships from the
    /// air has the kind the attackers took the contact for, and gives points
    /// for it; a surface fight's claim has no kind and gives none.
    Claims,
    /// No unit of the side's and no event it may see shows this: the count
    /// is 0 and says nothing. The umpire counts it at the end.
    NotObservable,
}

impl KnownBasis {
    pub const fn name(self) -> &'static str {
        match self {
            KnownBasis::Exact => "exact",
            KnownBasis::Claims => "claims",
            KnownBasis::NotObservable => "not_observable",
        }
    }
}

/// One objective's progress as far as its side can know it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KnownProgress {
    pub kind: &'static str,
    pub base: Option<BaseId>,
    pub closed: bool,
    pub basis: KnownBasis,
    /// Ships, aircraft, tons, or for a base the days it was as wanted. 0
    /// when not observable.
    pub count: Fixed,
    /// For `base_open` at a base of the side's own: the days counted, which
    /// `count` is out of.
    pub days: Option<u32>,
    /// Points of the known count, where the kind of what was counted is
    /// known: for ships claimed sunk, those of the claims that name a kind
    /// (`None` if none does); for aircraft claimed, the count times the
    /// points for one; `None` for what is not observable.
    pub points: Option<Fixed>,
}

/// What a side's claims add up to (decision 0056).
#[derive(Default)]
struct Claimed {
    /// Enemy ships believed sunk, by any claim.
    sunk: u32,
    /// Of those, by the kind of ship the attackers took the contact for, for
    /// the claims that name one.
    sunk_by_kind: BTreeMap<ShipKind, u32>,
    /// A claim says the side attacked ships: it names a kind.
    attacked_ships: bool,
    /// Enemy aircraft believed shot down or destroyed on the ground.
    aircraft: u32,
    /// The side has a claim from the air.
    from_air: bool,
}

impl Claimed {
    fn note(&mut self, sunk: u32, kind: Option<ShipKind>, aircraft: u32, from_air: bool) {
        self.sunk += sunk;
        if let Some(kind) = kind {
            self.attacked_ships = true;
            *self.sunk_by_kind.entry(kind).or_insert(0) += sunk;
        }
        self.aircraft += aircraft;
        self.from_air |= from_air;
    }
}

/// The progress of one side's objectives from the events it may see
/// (`events` is [`SideView::events`](crate::view::SideView::events)), and
/// which bases are its own. Reads no truth.
pub fn known_progress(
    objectives: &SideObjectives,
    own_bases: &[BaseId],
    events: &[SideEvent],
) -> Vec<KnownProgress> {
    use crate::side_events::SideEventKind;
    // Tons unloaded at each base by the side's own ships, less the tons they
    // loaded there: a side sees its own cargo events whole.
    let mut cargo: BTreeMap<BaseId, Fixed> = BTreeMap::new();
    // Each day's end at the side's own bases, which it sees whole.
    let mut base_days: BTreeMap<BaseId, BaseDays> = BTreeMap::new();
    let mut claimed = Claimed::default();
    for event in events {
        if let SideEventKind::Whole(kind) = &event.kind {
            match kind {
                EventKind::CargoUnloaded {
                    base, unloaded_t, ..
                } => {
                    *cargo.entry(*base).or_insert(Fixed::ZERO) += *unloaded_t;
                }
                EventKind::CargoLoaded { base, loaded_t, .. } => {
                    *cargo.entry(*base).or_insert(Fixed::ZERO) -= *loaded_t;
                }
                EventKind::BaseAtDayEnd {
                    base,
                    supply_t,
                    aircraft_ready,
                } => base_days
                    .entry(*base)
                    .or_default()
                    .note(*supply_t, *aircraft_ready),
                EventKind::ClaimReceived {
                    claimed_sunk,
                    claimed_kind,
                    claimed_shot_down,
                    claimed_on_ground,
                    from_air,
                    ..
                } => claimed.note(
                    *claimed_sunk,
                    *claimed_kind,
                    *claimed_shot_down + *claimed_on_ground,
                    *from_air,
                ),
                _ => {}
            }
        }
    }
    objectives
        .objectives
        .iter()
        .map(|objective| {
            let mut known = KnownProgress {
                kind: objective.name(),
                base: objective.base(),
                closed: false,
                basis: KnownBasis::NotObservable,
                count: Fixed::ZERO,
                days: None,
                points: None,
            };
            match objective {
                Objective::ShipsSunk(points) => {
                    // A side that has been told of ships, even of none sunk
                    // by an attack it knows it made, has a claim count.
                    if claimed.sunk > 0 || claimed.attacked_ships {
                        known.basis = KnownBasis::Claims;
                        known.count = Fixed::from_int(i64::from(claimed.sunk));
                        if !claimed.sunk_by_kind.is_empty() {
                            known.points = Some(
                                claimed
                                    .sunk_by_kind
                                    .iter()
                                    .map(|(kind, n)| points.of(*kind) * i64::from(*n))
                                    .fold(Fixed::ZERO, |sum, value| sum + value),
                            );
                        }
                    }
                }
                Objective::ShipsCrippled(_) => {}
                Objective::AircraftDestroyed { points } => {
                    // Any claim from the air tells the side something of
                    // aircraft, if only that it claims none.
                    if claimed.from_air {
                        known.basis = KnownBasis::Claims;
                        known.count = Fixed::from_int(i64::from(claimed.aircraft));
                        known.points = Some(known.count * *points);
                    }
                }
                Objective::CargoLanded { base, points_per_t } => {
                    known.basis = KnownBasis::Exact;
                    known.count = landed(cargo.get(base).copied().unwrap_or(Fixed::ZERO));
                    known.points = Some(known.count * *points_per_t);
                }
                Objective::BaseOpen {
                    base,
                    closed,
                    points,
                } => {
                    known.closed = *closed;
                    if own_bases.contains(base) {
                        let days = base_days.get(base).copied().unwrap_or_default();
                        known.basis = KnownBasis::Exact;
                        known.count = Fixed::from_int(i64::from(days.as_wanted(*closed)));
                        known.days = Some(days.days);
                        known.points = Some(days.points(*closed, *points));
                    }
                }
            }
            known
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventKind;
    use crate::hex::Hex;
    use crate::ids::{ShipClassId, ShipId, TaskForceId};
    use crate::orders::Cargo;
    use crate::side_events::SideEventKind;
    use crate::state::Base;

    fn fx(text: &str) -> Fixed {
        text.parse().expect("a decimal")
    }

    fn n(value: i64) -> Fixed {
        Fixed::from_int(value)
    }

    fn levels() -> Levels {
        Levels::new(n(3), n(8), n(16)).expect("levels")
    }

    #[test]
    fn levels_must_be_above_zero_and_rising() {
        assert!(Levels::new(n(0), n(8), n(16)).is_none());
        assert!(Levels::new(n(3), n(3), n(16)).is_none());
        assert!(Levels::new(n(3), n(8), n(8)).is_none());
        assert!(Levels::new(n(9), n(8), n(16)).is_none());
        assert!(Levels::new(n(3), n(8), n(16)).is_some());
    }

    #[test]
    fn a_margin_equal_to_a_level_is_that_level_and_just_below_is_the_one_under() {
        let l = levels();
        let below = |x: Fixed| x - Fixed::from_raw(1);
        let minus = |x: Fixed| Fixed::ZERO - x;
        assert_eq!(outcome_of(n(16), &l), Outcome::Victory(Level::Decisive));
        assert_eq!(outcome_of(below(n(16)), &l), Outcome::Victory(Level::Major));
        assert_eq!(outcome_of(n(8), &l), Outcome::Victory(Level::Major));
        assert_eq!(outcome_of(below(n(8)), &l), Outcome::Victory(Level::Minor));
        assert_eq!(outcome_of(n(3), &l), Outcome::Victory(Level::Minor));
        // Strictly between minus minor and minor: a stalemate, edges not
        // included.
        assert_eq!(outcome_of(below(n(3)), &l), Outcome::Stalemate);
        assert_eq!(outcome_of(Fixed::ZERO, &l), Outcome::Stalemate);
        assert_eq!(outcome_of(minus(below(n(3))), &l), Outcome::Stalemate);
        // The same boundaries for a defeat.
        assert_eq!(outcome_of(minus(n(3)), &l), Outcome::Defeat(Level::Minor));
        assert_eq!(
            outcome_of(minus(below(n(8))), &l),
            Outcome::Defeat(Level::Minor)
        );
        assert_eq!(outcome_of(minus(n(8)), &l), Outcome::Defeat(Level::Major));
        assert_eq!(
            outcome_of(minus(below(n(16))), &l),
            Outcome::Defeat(Level::Major)
        );
        assert_eq!(
            outcome_of(minus(n(16)), &l),
            Outcome::Defeat(Level::Decisive)
        );
        assert_eq!(outcome_of(fx("7.75"), &l), Outcome::Victory(Level::Minor));
        assert_eq!(outcome_of(fx("-7.75"), &l), Outcome::Defeat(Level::Minor));
        assert_eq!(
            Outcome::Victory(Level::Major).mirror(),
            Outcome::Defeat(Level::Major)
        );
        assert_eq!(Outcome::Stalemate.mirror(), Outcome::Stalemate);
    }

    // -----------------------------------------------------------------
    // Made-up worlds for cargo_landed and base_open.
    // -----------------------------------------------------------------

    fn base(id: u32, side: Side, supply: i64) -> Base {
        Base {
            supply_t: n(supply),
            ..Base::new(BaseId::new(id), format!("Base {id}"), side, Hex::new(0, 0))
        }
    }

    fn ship(id: u32, side: Side) -> Ship {
        Ship::new(
            ShipId::new(id),
            format!("Ship {id}"),
            ShipClassId::new(1),
            side,
            ShipLocation::TaskForce(TaskForceId::new(1)),
        )
    }

    fn unloaded(log: &mut EventLog, ship: u32, base: u32, cargo: Cargo, tons: i64) {
        log.record(
            1,
            None,
            EventKind::CargoUnloaded {
                ship: ShipId::new(ship),
                task_force: TaskForceId::new(1),
                base: BaseId::new(base),
                cargo,
                unloaded_t: n(tons),
                aboard_t: Fixed::ZERO,
                base_stock_t: Fixed::ZERO,
            },
            Vec::new(),
            None,
        );
    }

    fn cargo_scenario() -> (TruthState, EventLog) {
        let mut truth = TruthState::default();
        truth.bases.insert(BaseId::new(1), base(1, Side::Allied, 5));
        truth
            .bases
            .insert(BaseId::new(2), base(2, Side::Japanese, 5));
        truth.ships.insert(ShipId::new(1), ship(1, Side::Allied));
        truth.ships.insert(ShipId::new(2), ship(2, Side::Japanese));
        let mut log = EventLog::default();
        // The Allied ship lands 100 t of supply and 40 t of fuel at base 1,
        // and 10 t at base 2. The Japanese ship lands 7 t at base 1.
        unloaded(&mut log, 1, 1, Cargo::Supply, 100);
        unloaded(&mut log, 1, 1, Cargo::Fuel, 40);
        unloaded(&mut log, 1, 2, Cargo::Supply, 10);
        unloaded(&mut log, 2, 1, Cargo::Supply, 7);
        (truth, log)
    }

    fn one_side(objective: Objective) -> Objectives {
        let empty = || SideObjectives {
            mission: "Placeholder mission.".to_string(),
            objectives: Vec::new(),
        };
        let mut allied = empty();
        allied.objectives.push(objective);
        Objectives {
            allied,
            japanese: empty(),
            levels: levels(),
            history: None,
        }
    }

    #[test]
    fn cargo_landed_counts_the_tons_of_supply_and_fuel_the_side_unloaded_at_the_named_base() {
        let (truth, log) = cargo_scenario();
        let ref_data = RefData::default();
        let tallies = tallies(&truth, &log, &ref_data);
        assert_eq!(tallies[&Side::Allied].cargo_t[&BaseId::new(1)], n(140));
        assert_eq!(tallies[&Side::Allied].cargo_t[&BaseId::new(2)], n(10));
        assert_eq!(tallies[&Side::Japanese].cargo_t[&BaseId::new(1)], n(7));

        // 140 t at half a point a ton is 70 points; the 10 t at base 2 and
        // the Japanese 7 t are not counted.
        let objectives = one_side(Objective::CargoLanded {
            base: BaseId::new(1),
            points_per_t: fx("0.5"),
        });
        let result = score(&objectives, &tallies);
        let line = &result.allied.objectives[0];
        assert_eq!(
            (line.kind, line.count, line.points),
            ("cargo_landed", n(140), n(70))
        );
        assert_eq!(result.allied.total, n(70));
        assert_eq!(result.allied.margin, n(70));
        assert_eq!(result.japanese.margin, Fixed::ZERO - n(70));
        assert_eq!(result.allied.outcome, Outcome::Victory(Level::Decisive));
        assert_eq!(result.japanese.outcome, Outcome::Defeat(Level::Decisive));
        let other = one_side(Objective::CargoLanded {
            base: BaseId::new(2),
            points_per_t: Fixed::ONE,
        });
        assert_eq!(score(&other, &tallies).allied.total, n(10));
    }

    #[test]
    fn what_a_side_knows_of_cargo_landed_is_its_own_unloading_whole() {
        let (truth, log) = cargo_scenario();
        let owners = crate::side_events::Owners::from_state(&truth, &Default::default(), 0);
        let allied_events = crate::side_events::project(Side::Allied, log.from_index(0), &owners);
        let japanese_events =
            crate::side_events::project(Side::Japanese, log.from_index(0), &owners);
        let objectives = one_side(Objective::CargoLanded {
            base: BaseId::new(1),
            points_per_t: fx("0.5"),
        });
        let known = known_progress(&objectives.allied, &[], &allied_events);
        assert_eq!(known.len(), 1);
        assert_eq!(known[0].basis, KnownBasis::Exact);
        assert_eq!(known[0].count, n(140));
        assert_eq!(known[0].points, Some(n(70)));
        // The other side's events hold none of the Allied unloading.
        let known = known_progress(&objectives.allied, &[], &japanese_events);
        assert_eq!(known[0].count, n(7), "only what its own ships landed");
    }

    // -----------------------------------------------------------------
    // Decision 0067: cargo net of what was loaded back, and a base open by
    // the share of days.
    //
    // # Hand calculation
    //
    // Written by the architect before the code, from the worked case of
    // decision 0067. A game of four days. Base 1 is Allied. The Allied side
    // holds `cargo_landed` at base 1 at 0.005 points a ton and `base_open`
    // at base 1 for 10 points; the Japanese side holds `base_open` at base 1
    // with `closed = true` for 10 points.
    //
    // | | Day 1 | Day 2 | Day 3 | Day 4 |
    // | --- | --- | --- | --- | --- |
    // | Unloaded at base 1 by Allied ship 1, tons | 1,000 | 0 | 1,000 | 0 |
    // | Loaded at base 1 by Allied ship 1, tons | 0 | 1,000 | 0 | 0 |
    // | Supply at the end of the day, tons | 1,000 | 0 | 1,000 | 1,000 |
    // | Ready aircraft at the end of the day | 3 | 3 | 0 | 2 |
    // | Open | yes | no | no | yes |
    //
    // - Cargo landed: 1,000 + 1,000 unloaded, less 1,000 loaded, is 1,000
    //   tons. 1,000 x 0.005 = 5 points. (Before the decision: 2,000 tons, 10
    //   points.)
    // - Base open: open on days 1 and 4, 2 days of 4. 10 x 2 / 4 = 5 points
    //   for the Allied side. Shut on days 2 and 3, 2 days of 4: 5 points for
    //   the Japanese side. The two sum to the 10 of the objective.
    // - A side that loads more at a base than it unloads there has landed
    //   nothing: 300 unloaded and 500 loaded is 0 tons, not -200.
    // - Three days, open on one: 10 x 1 / 3 = 3.333333 for the side that
    //   wants it open and 10 x 2 / 3 = 6.666666 for the side that wants it
    //   shut (fixed point keeps six places and cuts the rest).
    // - No day resolved: 0 days of 0, and 0 points for either side.
    // -----------------------------------------------------------------

    fn loaded(log: &mut EventLog, day: u32, ship: u32, base: u32, tons: i64) {
        log.record(
            day,
            None,
            EventKind::CargoLoaded {
                ship: ShipId::new(ship),
                task_force: TaskForceId::new(1),
                base: BaseId::new(base),
                cargo: Cargo::Supply,
                loaded_t: n(tons),
                aboard_t: n(tons),
                base_stock_t: Fixed::ZERO,
            },
            Vec::new(),
            None,
        );
    }

    fn day_end(log: &mut EventLog, day: u32, base: u32, supply: i64, ready: u32) {
        log.record(
            day,
            None,
            EventKind::BaseAtDayEnd {
                base: BaseId::new(base),
                supply_t: n(supply),
                aircraft_ready: ready,
            },
            Vec::new(),
            None,
        );
    }

    /// The worked case of decision 0067, as a truth and a log.
    fn worked_case() -> (TruthState, EventLog) {
        let mut truth = TruthState::default();
        truth
            .bases
            .insert(BaseId::new(1), base(1, Side::Allied, 1000));
        truth.ships.insert(ShipId::new(1), ship(1, Side::Allied));
        let mut log = EventLog::default();
        unloaded(&mut log, 1, 1, Cargo::Supply, 1000);
        day_end(&mut log, 1, 1, 1000, 3);
        loaded(&mut log, 2, 1, 1, 1000);
        day_end(&mut log, 2, 1, 0, 3);
        unloaded(&mut log, 1, 1, Cargo::Supply, 1000);
        day_end(&mut log, 3, 1, 1000, 0);
        day_end(&mut log, 4, 1, 1000, 2);
        (truth, log)
    }

    fn worked_objectives() -> Objectives {
        Objectives {
            allied: SideObjectives {
                mission: "Placeholder mission.".to_string(),
                objectives: vec![
                    Objective::CargoLanded {
                        base: BaseId::new(1),
                        points_per_t: fx("0.005"),
                    },
                    Objective::BaseOpen {
                        base: BaseId::new(1),
                        closed: false,
                        points: n(10),
                    },
                ],
            },
            japanese: SideObjectives {
                mission: "Placeholder mission.".to_string(),
                objectives: vec![Objective::BaseOpen {
                    base: BaseId::new(1),
                    closed: true,
                    points: n(10),
                }],
            },
            levels: levels(),
            history: None,
        }
    }

    #[test]
    fn the_worked_case_of_decision_0067_scores_net_cargo_and_the_share_of_days() {
        let (truth, log) = worked_case();
        let result = score(
            &worked_objectives(),
            &tallies(&truth, &log, &RefData::default()),
        );
        let cargo = &result.allied.objectives[0];
        assert_eq!(
            (cargo.kind, cargo.count, cargo.points, cargo.days),
            ("cargo_landed", n(1000), n(5), None)
        );
        let open = &result.allied.objectives[1];
        assert_eq!(
            (open.kind, open.count, open.points, open.days),
            ("base_open", n(2), n(5), Some(4))
        );
        let shut = &result.japanese.objectives[0];
        assert!(shut.closed);
        assert_eq!((shut.count, shut.points, shut.days), (n(2), n(5), Some(4)));
        // The two sides' objectives on the one base sum to the whole.
        assert_eq!(open.points + shut.points, n(10));
        assert_eq!(result.allied.total, n(10));
        assert_eq!(result.allied.margin, n(5));
    }

    #[test]
    fn cargo_loaded_back_beyond_what_was_landed_is_nothing_landed_and_never_less() {
        let (truth, _) = worked_case();
        let mut log = EventLog::default();
        unloaded(&mut log, 1, 1, Cargo::Supply, 300);
        loaded(&mut log, 1, 1, 1, 500);
        let tallies = tallies(&truth, &log, &RefData::default());
        let result = score(&worked_objectives(), &tallies);
        let cargo = &result.allied.objectives[0];
        assert_eq!((cargo.count, cargo.points), (Fixed::ZERO, Fixed::ZERO));
        // Loading at another base takes nothing from this one.
        let mut log = EventLog::default();
        unloaded(&mut log, 1, 1, Cargo::Supply, 300);
        loaded(&mut log, 1, 1, 2, 500);
        let tallies = self::tallies(&truth, &log, &RefData::default());
        assert_eq!(
            score(&worked_objectives(), &tallies).allied.objectives[0].count,
            n(300)
        );
    }

    #[test]
    fn a_base_is_open_on_a_day_with_supply_and_a_ready_aircraft_and_on_no_other() {
        // Day by day: both, no supply, no ready aircraft, neither.
        let mut days = BaseDays::default();
        for (supply, ready) in [(5, 3), (0, 3), (5, 0), (0, 0)] {
            days.note(n(supply), ready);
        }
        assert_eq!(days, BaseDays { days: 4, open: 1 });
        assert_eq!((days.as_wanted(false), days.as_wanted(true)), (1, 3));
        // One day of three, in fixed point.
        let thirds = BaseDays { days: 3, open: 1 };
        assert_eq!(thirds.points(false, n(10)), fx("3.333333"));
        assert_eq!(thirds.points(true, n(10)), fx("6.666666"));
        // No day resolved: nothing for either side.
        let none = BaseDays::default();
        assert_eq!(none.points(false, n(10)), Fixed::ZERO);
        assert_eq!(none.points(true, n(10)), Fixed::ZERO);
    }

    #[test]
    fn either_sides_tally_says_the_same_of_a_base_whoever_holds_it() {
        let (truth, log) = worked_case();
        let tallies = tallies(&truth, &log, &RefData::default());
        assert_eq!(
            tallies[&Side::Allied].base_days,
            tallies[&Side::Japanese].base_days
        );
        assert_eq!(
            tallies[&Side::Allied].base_days[&BaseId::new(1)],
            BaseDays { days: 4, open: 2 }
        );
        // A base with no day counted scores nothing, open or shut.
        let other = one_side(Objective::BaseOpen {
            base: BaseId::new(9),
            closed: true,
            points: n(7),
        });
        let line = &score(&other, &tallies).allied.objectives[0];
        assert_eq!((line.count, line.points, line.days), (n(0), n(0), Some(0)));
    }

    #[test]
    fn a_side_knows_the_days_of_its_own_base_and_its_net_cargo_and_nothing_of_the_enemys() {
        let (truth, log) = worked_case();
        let owners = crate::side_events::Owners::from_state(&truth, &Default::default(), 0);
        let allied = crate::side_events::project(Side::Allied, log.from_index(0), &owners);
        let japanese = crate::side_events::project(Side::Japanese, log.from_index(0), &owners);
        let objectives = worked_objectives();
        // The Allied side: exact, and the umpire's own numbers.
        let known = known_progress(&objectives.allied, &[BaseId::new(1)], &allied);
        assert_eq!(
            known
                .iter()
                .map(|k| (k.basis, k.count, k.days, k.points))
                .collect::<Vec<_>>(),
            vec![
                (KnownBasis::Exact, n(1000), None, Some(n(5))),
                (KnownBasis::Exact, n(2), Some(4), Some(n(5))),
            ]
        );
        // The Japanese side is given none of base 1's days: it is not its
        // base.
        assert!(japanese.is_empty());
        let known = known_progress(&objectives.japanese, &[], &japanese);
        assert_eq!(
            (
                known[0].basis,
                known[0].count,
                known[0].days,
                known[0].points
            ),
            (KnownBasis::NotObservable, Fixed::ZERO, None, None)
        );
        // Before any day is resolved a side's own base has 0 days of 0.
        let known = known_progress(&objectives.allied, &[BaseId::new(1)], &[]);
        assert_eq!(
            (
                known[1].basis,
                known[1].count,
                known[1].days,
                known[1].points
            ),
            (KnownBasis::Exact, Fixed::ZERO, Some(0), Some(Fixed::ZERO))
        );
    }

    #[test]
    fn claims_count_ships_of_no_known_kind_and_nothing_else_of_the_enemys_losses_is_seen() {
        let objectives = SideObjectives {
            mission: "Placeholder mission.".to_string(),
            objectives: vec![
                Objective::ShipsSunk(ShipPoints::default()),
                Objective::ShipsCrippled(ShipPoints::default()),
                Objective::AircraftDestroyed { points: Fixed::ONE },
            ],
        };
        let claim = SideEvent {
            id: crate::ids::EventId::new(1),
            number: 1,
            order_number: None,
            day: 1,
            pulse: None,
            kind: SideEventKind::Whole(EventKind::ClaimReceived {
                side: Side::Allied,
                hex: Hex::new(0, 0),
                claimed_sunk: 2,
                claimed_hits: 0,
                claimed_kind: None,
                claimed_shot_down: 0,
                claimed_on_ground: 0,
                from_air: false,
            }),
            causes: Vec::new(),
            hidden_causes: false,
            breakdown: None,
        };
        let known = known_progress(&objectives, &[], &[claim]);
        assert_eq!(
            (known[0].basis, known[0].count, known[0].points),
            (KnownBasis::Claims, n(2), None)
        );
        assert_eq!(known[1].basis, KnownBasis::NotObservable);
        assert_eq!(known[2].basis, KnownBasis::NotObservable);
        assert_eq!(known[2].count, Fixed::ZERO);
    }

    fn air_claim(
        id: u32,
        sunk: u32,
        kind: Option<ShipKind>,
        shot_down: u32,
        on_ground: u32,
    ) -> SideEvent {
        SideEvent {
            id: crate::ids::EventId::new(id),
            number: id,
            order_number: None,
            day: 1,
            pulse: None,
            kind: SideEventKind::Whole(EventKind::ClaimReceived {
                side: Side::Allied,
                hex: Hex::new(0, 0),
                claimed_sunk: sunk,
                claimed_hits: if kind.is_some() { 3 } else { 0 },
                claimed_kind: kind,
                claimed_shot_down: shot_down,
                claimed_on_ground: on_ground,
                from_air: true,
            }),
            causes: Vec::new(),
            hidden_causes: false,
            breakdown: None,
        }
    }

    #[test]
    fn claims_from_the_air_count_ships_with_points_for_their_kind_and_aircraft_shot_down_or_destroyed(
    ) {
        let ships = ShipPoints(BTreeMap::from([
            (ShipKind::Carrier, n(10)),
            (ShipKind::Cruiser, n(4)),
        ]));
        let objectives = SideObjectives {
            mission: "Placeholder mission.".to_string(),
            objectives: vec![
                Objective::ShipsSunk(ships),
                Objective::ShipsCrippled(ShipPoints::default()),
                Objective::AircraftDestroyed {
                    points: "0.05".parse().expect("a decimal"),
                },
            ],
        };
        // A raid on a carrier claims one sunk, one on a cruiser claims none,
        // and a raid on a base claims aircraft on the ground.
        let events = [
            air_claim(1, 1, Some(ShipKind::Carrier), 4, 0),
            air_claim(2, 0, Some(ShipKind::Cruiser), 0, 0),
            air_claim(3, 0, None, 2, 20),
        ];
        let known = known_progress(&objectives, &[], &events);
        assert_eq!(
            (known[0].basis, known[0].count, known[0].points),
            (KnownBasis::Claims, n(1), Some(n(10)))
        );
        assert_eq!(known[1].basis, KnownBasis::NotObservable);
        // 4 + 0 + 2 shot down and 20 on the ground.
        assert_eq!(
            (known[2].basis, known[2].count, known[2].points),
            (
                KnownBasis::Claims,
                n(26),
                Some("1.3".parse().expect("a decimal"))
            )
        );
    }

    #[test]
    fn a_claim_that_names_a_kind_makes_a_count_even_of_no_ship_sunk_and_one_that_does_not_makes_none(
    ) {
        let objectives = SideObjectives {
            mission: "Placeholder mission.".to_string(),
            objectives: vec![
                Objective::ShipsSunk(ShipPoints::default()),
                Objective::AircraftDestroyed { points: Fixed::ONE },
            ],
        };
        // Aircraft only: nothing is known of ships; no ship was attacked.
        let known = known_progress(&objectives, &[], &[air_claim(1, 0, None, 3, 0)]);
        assert_eq!(known[0].basis, KnownBasis::NotObservable);
        assert_eq!((known[1].basis, known[1].count), (KnownBasis::Claims, n(3)));
        // A raid on ships that claims none sunk: a count of 0, from claims.
        let known = known_progress(
            &objectives,
            &[],
            &[air_claim(1, 0, Some(ShipKind::Carrier), 0, 0)],
        );
        assert_eq!(
            (known[0].basis, known[0].count, known[0].points),
            (KnownBasis::Claims, Fixed::ZERO, Some(Fixed::ZERO))
        );
        // A claim from the air of no aircraft is still a claim of none.
        assert_eq!(
            (known[1].basis, known[1].count),
            (KnownBasis::Claims, Fixed::ZERO)
        );
    }

    // -----------------------------------------------------------------
    // The scale, on made-up counts.
    // -----------------------------------------------------------------

    fn scale() -> Objectives {
        let ships = || {
            ShipPoints(BTreeMap::from([
                (ShipKind::Carrier, n(10)),
                (ShipKind::Battleship, n(8)),
                (ShipKind::Cruiser, n(4)),
                (ShipKind::Destroyer, n(1)),
                (ShipKind::Transport, n(2)),
                (ShipKind::Auxiliary, n(1)),
            ]))
        };
        let side = || SideObjectives {
            mission: "Placeholder mission.".to_string(),
            objectives: vec![
                Objective::ShipsSunk(ships()),
                Objective::ShipsCrippled(ships()),
                Objective::AircraftDestroyed { points: fx("0.05") },
            ],
        };
        Objectives {
            allied: side(),
            japanese: side(),
            levels: levels(),
            history: None,
        }
    }

    fn tally(carriers: u32, destroyers: u32, crippled: u32, aircraft: u32) -> Tally {
        Tally {
            sunk: BTreeMap::from([
                (ShipKind::Carrier, carriers),
                (ShipKind::Destroyer, destroyers),
            ]),
            crippled: BTreeMap::from([(ShipKind::Carrier, crippled)]),
            aircraft,
            ..Tally::default()
        }
    }

    fn pair(allied: Tally, japanese: Tally) -> BTreeMap<Side, Tally> {
        BTreeMap::from([(Side::Allied, allied), (Side::Japanese, japanese)])
    }

    #[test]
    fn the_margins_of_the_two_sides_sum_to_zero_and_their_levels_mirror() {
        let objectives = scale();
        for a in 0..4 {
            for b in 0..4 {
                for (c, d) in [(0, 0), (3, 60), (1, 5)] {
                    let result = score(&objectives, &pair(tally(a, b, 1, c), tally(b, a, 0, d)));
                    assert_eq!(result.allied.margin + result.japanese.margin, Fixed::ZERO);
                    assert_eq!(result.allied.outcome.mirror(), result.japanese.outcome);
                    assert_eq!(
                        result.allied.margin,
                        result.allied.total - result.japanese.total
                    );
                }
            }
        }
    }

    #[test]
    fn sinking_one_more_enemy_ship_or_losing_one_fewer_never_lowers_a_margin() {
        let objectives = scale();
        for a in 0..4 {
            for b in 0..4 {
                for aircraft in [0, 7, 60] {
                    let margin = |allied: Tally, japanese: Tally| {
                        score(&objectives, &pair(allied, japanese)).allied.margin
                    };
                    let before = margin(tally(a, 0, 0, aircraft), tally(b, 0, 0, 0));
                    // One more enemy carrier sunk by the Allied side.
                    assert!(margin(tally(a + 1, 0, 0, aircraft), tally(b, 0, 0, 0)) >= before);
                    // One of the Allied side's own carriers fewer lost (the
                    // Japanese tally holds the Allied losses).
                    if b > 0 {
                        assert!(margin(tally(a, 0, 0, aircraft), tally(b - 1, 0, 0, 0)) >= before);
                    }
                    // One more enemy aircraft destroyed, one more crippled.
                    assert!(margin(tally(a, 0, 0, aircraft + 1), tally(b, 0, 0, 0)) >= before);
                    assert!(margin(tally(a, 0, 1, aircraft), tally(b, 0, 0, 0)) >= before);
                }
            }
        }
    }

    #[test]
    fn a_ship_at_the_yard_is_afloat_and_counts_as_crippled_by_the_damage_she_left_with() {
        use crate::refdata::{ShipClass, ShipKind};
        let class = ShipClass {
            id: ShipClassId::new(1),
            name: "Cruiser".to_string(),
            kind: ShipKind::Cruiser,
            cruise_speed_kn: n(16),
            max_speed_kn: n(20),
            fuel_capacity_t: n(2000),
            endurance_nm: n(8000),
            cargo_capacity_t: Fixed::ZERO,
            main_guns: 0,
            gun_rounds_per_min: Fixed::ZERO,
            gun_rounds_per_gun: 0,
            gun_damage: Fixed::ZERO,
            torpedo_tubes: 0,
            torpedo_tubes_one_side: 0,
            torpedo_damage: Fixed::ZERO,
            toughness: n(100),
            radar: false,
            aircraft_capacity: 0,
            anti_aircraft: 0,
        };
        let mut ref_data = RefData::default();
        ref_data.ship_classes.insert(class.id, class);
        let mut truth = TruthState::default();
        // Ship 1 is at the yard with her propulsion out; ship 2 is at the
        // yard with nothing out; ship 3 is sunk.
        let mut crippled = ship(1, Side::Japanese);
        crippled.location = ShipLocation::Yard(TaskForceId::new(1));
        crippled.systems.propulsion = SystemState::Out;
        let mut whole = ship(2, Side::Japanese);
        whole.location = ShipLocation::Yard(TaskForceId::new(1));
        let mut sunk = ship(3, Side::Japanese);
        sunk.location = ShipLocation::Sunk;
        for ship in [crippled, whole, sunk] {
            truth.ships.insert(ship.id, ship);
        }
        let tallies = tallies(&truth, &EventLog::default(), &ref_data);
        let taken = &tallies[&Side::Allied];
        assert_eq!(taken.sunk.get(&ShipKind::Cruiser), Some(&1));
        assert_eq!(taken.crippled.get(&ShipKind::Cruiser), Some(&1));
    }
}
