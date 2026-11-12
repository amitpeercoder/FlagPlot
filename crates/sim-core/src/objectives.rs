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
//!   good, from five events: `AircraftLost` (shot down, in the air),
//!   `AirGroupLost` (lost with a sunk ship, ready and damaged),
//!   `AircraftDestroyedOnGround` (ready), `DamagedAircraftDestroyedOnGround`
//!   (damaged) and `AircraftWorn::written_off`. Not counted: `AircraftWorn`
//!   `worn` (damaged and repaired later, not lost), `PilotsLost` (pilots, not
//!   aircraft; the aircraft's own event counts the aircraft), `AircraftRepaired`
//!   and every event that adds aircraft.
//! - `cargo_landed`: `CargoUnloaded` events of the side's own ships at the
//!   named base, supply and fuel tons together.
//! - `base_open`: the named base's supply above zero and at least one ready
//!   aircraft of an air group at it, read from the final truth.

use core::fmt;
use std::collections::BTreeMap;

use crate::events::{EventKind, EventLog};
use crate::fixed::Fixed;
use crate::ids::BaseId;
use crate::refdata::{RefData, ShipKind};
use crate::side_events::SideEvent;
use crate::state::{
    AirGroupLocation, Ship, ShipLocation, ShipSystem, Side, SystemState, TruthState,
};

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
    /// Tons unloaded by the side's ships, by base.
    pub cargo_t: BTreeMap<BaseId, Fixed>,
    /// Whether each base has supply above zero and an aircraft ready.
    pub open: BTreeMap<BaseId, bool>,
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
            _ => {}
        }
    }

    // Bases: open or not, for both sides, whichever side holds them.
    for base in truth.bases.values() {
        let ready: u32 = truth
            .air_groups
            .values()
            .filter(|g| g.location == AirGroupLocation::Base(base.id))
            .map(|g| g.ready)
            .sum();
        let open = base.supply_t > Fixed::ZERO && ready > 0;
        for tally in out.values_mut() {
            tally.open.insert(base.id, open);
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
    /// Ships, aircraft, tons, or 1 or 0 for a base.
    pub count: Fixed,
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
                    score.count = tally.cargo_t.get(base).copied().unwrap_or(Fixed::ZERO);
                    score.points = score.count * *points_per_t;
                }
                Objective::BaseOpen {
                    base,
                    closed,
                    points,
                } => {
                    let open = tally.open.get(base).copied().unwrap_or(false);
                    score.closed = *closed;
                    score.count = if open != *closed {
                        Fixed::ONE
                    } else {
                        Fixed::ZERO
                    };
                    score.points = score.count * *points;
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
    /// From the claims the side received (`ClaimReceived`): the number of
    /// enemy ships it believes sunk, of no known kind, so with no points.
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
    /// Ships, aircraft, tons, or 1 or 0 for a base. 0 when not observable.
    pub count: Fixed,
    /// Points of the known count, where the kind of what was counted is
    /// known: `None` for claims and for what is not observable.
    pub points: Option<Fixed>,
}

/// A side's own base and what it holds, as the side sees it. The view hands
/// over its own bases and air groups only.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OwnBase {
    pub id: BaseId,
    pub supply_t: Fixed,
    pub aircraft_ready: u32,
}

/// The progress of one side's objectives from its own bases and the events
/// it may see (`events` is [`SideView::events`](crate::view::SideView::events)).
/// Reads no truth.
pub fn known_progress(
    objectives: &SideObjectives,
    own_bases: &[OwnBase],
    events: &[SideEvent],
) -> Vec<KnownProgress> {
    use crate::side_events::SideEventKind;
    // Tons unloaded at each base by the side's own ships: a side sees its
    // own unloading whole.
    let mut cargo: BTreeMap<BaseId, Fixed> = BTreeMap::new();
    let mut claimed: u32 = 0;
    for event in events {
        if let SideEventKind::Whole(kind) = &event.kind {
            match kind {
                EventKind::CargoUnloaded {
                    base, unloaded_t, ..
                } => {
                    *cargo.entry(*base).or_insert(Fixed::ZERO) += *unloaded_t;
                }
                EventKind::ClaimReceived { claimed_sunk, .. } => claimed += *claimed_sunk,
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
                points: None,
            };
            match objective {
                Objective::ShipsSunk(_) => {
                    if claimed > 0 {
                        known.basis = KnownBasis::Claims;
                        known.count = Fixed::from_int(i64::from(claimed));
                    }
                }
                Objective::ShipsCrippled(_) | Objective::AircraftDestroyed { .. } => {}
                Objective::CargoLanded { base, points_per_t } => {
                    known.basis = KnownBasis::Exact;
                    known.count = cargo.get(base).copied().unwrap_or(Fixed::ZERO);
                    known.points = Some(known.count * *points_per_t);
                }
                Objective::BaseOpen {
                    base,
                    closed,
                    points,
                } => {
                    known.closed = *closed;
                    if let Some(own) = own_bases.iter().find(|b| b.id == *base) {
                        let open = own.supply_t > Fixed::ZERO && own.aircraft_ready > 0;
                        known.basis = KnownBasis::Exact;
                        known.count = if open != *closed {
                            Fixed::ONE
                        } else {
                            Fixed::ZERO
                        };
                        known.points = Some(known.count * *points);
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
    use crate::ids::{AirGroupId, AircraftTypeId, ShipClassId, ShipId, TaskForceId};
    use crate::orders::{AirMission, Cargo};
    use crate::side_events::SideEventKind;
    use crate::state::{AirGroup, Base, ShipSystems};

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
            id: BaseId::new(id),
            name: format!("Base {id}"),
            side,
            hex: Hex::new(0, 0),
            supply_t: n(supply),
            anti_aircraft: 0,
            aircraft_repairs_per_day: 0,
            fuel_t: Fixed::ZERO,
        }
    }

    fn group(id: u32, side: Side, base: u32, ready: u32, damaged: u32) -> AirGroup {
        AirGroup {
            id: AirGroupId::new(id),
            name: format!("Group {id}"),
            side,
            location: AirGroupLocation::Base(BaseId::new(base)),
            aircraft_type: AircraftTypeId::new(1),
            ready,
            damaged,
            mission: AirMission::Rest,
            search: None,
            mission_order: None,
            arc_order: None,
            patrol_share: Fixed::ONE,
            share_order: None,
            fatigue: Fixed::ZERO,
            rest_above: None,
            rest_order: None,
            stood_down: false,
            pilots: None,
            strength: ready + damaged,
        }
    }

    fn ship(id: u32, side: Side) -> Ship {
        Ship {
            id: ShipId::new(id),
            name: format!("Ship {id}"),
            class: ShipClassId::new(1),
            side,
            fuel_t: Fixed::ZERO,
            supply_t: Fixed::ZERO,
            cargo_fuel_t: Fixed::ZERO,
            location: ShipLocation::TaskForce(TaskForceId::new(1)),
            flotation_damage: Fixed::ZERO,
            fire_damage: Fixed::ZERO,
            gun_rounds: Fixed::ZERO,
            torpedoes_ready: false,
            systems: ShipSystems::ALL_WORKING,
        }
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
        let owners = crate::side_events::Owners::from_state(&truth, &Default::default());
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

    /// Four bases, as the objectives see them at the end of a game:
    /// 1 has supply and a ready aircraft (open); 2 has no supply (shut);
    /// 3 has supply and only damaged aircraft (shut); 4 has supply and no
    /// aircraft (shut).
    fn open_scenario() -> TruthState {
        let mut truth = TruthState::default();
        truth.bases.insert(BaseId::new(1), base(1, Side::Allied, 5));
        truth.bases.insert(BaseId::new(2), base(2, Side::Allied, 0));
        truth
            .bases
            .insert(BaseId::new(3), base(3, Side::Japanese, 5));
        truth
            .bases
            .insert(BaseId::new(4), base(4, Side::Japanese, 5));
        for (id, base_id, ready, damaged) in [(1, 1, 3, 0), (2, 2, 3, 0), (3, 3, 0, 4)] {
            let side = truth.bases[&BaseId::new(base_id)].side;
            truth.air_groups.insert(
                AirGroupId::new(id),
                group(id, side, base_id, ready, damaged),
            );
        }
        truth
    }

    #[test]
    fn base_open_is_one_with_supply_and_an_aircraft_ready_and_closed_reads_the_other_way() {
        let truth = open_scenario();
        let tallies = tallies(&truth, &EventLog::default(), &RefData::default());
        // (base, closed, points of 7 expected for the base being as wanted)
        for (base_id, closed, expected) in [
            (1, false, 7),
            (1, true, 0),
            (2, false, 0),
            (2, true, 7),
            (3, false, 0),
            (3, true, 7),
            (4, false, 0),
            (4, true, 7),
        ] {
            let objectives = one_side(Objective::BaseOpen {
                base: BaseId::new(base_id),
                closed,
                points: n(7),
            });
            let result = score(&objectives, &tallies);
            let line = &result.allied.objectives[0];
            assert_eq!(
                line.points,
                n(expected),
                "base {base_id} closed {closed}: {line:?}"
            );
            assert_eq!(
                line.count,
                if expected > 0 {
                    Fixed::ONE
                } else {
                    Fixed::ZERO
                }
            );
            assert_eq!(line.closed, closed);
        }
        // Either side's tally says the same of a base, whoever holds it.
        assert_eq!(tallies[&Side::Allied].open, tallies[&Side::Japanese].open);
    }

    #[test]
    fn a_side_knows_whether_its_own_base_is_open_and_nothing_of_the_enemys() {
        let objectives = SideObjectives {
            mission: "Placeholder mission.".to_string(),
            objectives: vec![
                Objective::BaseOpen {
                    base: BaseId::new(1),
                    closed: false,
                    points: n(7),
                },
                Objective::BaseOpen {
                    base: BaseId::new(1),
                    closed: true,
                    points: n(7),
                },
                Objective::BaseOpen {
                    base: BaseId::new(3),
                    closed: true,
                    points: n(7),
                },
            ],
        };
        let own = [OwnBase {
            id: BaseId::new(1),
            supply_t: n(5),
            aircraft_ready: 3,
        }];
        let known = known_progress(&objectives, &own, &[]);
        assert_eq!(
            known
                .iter()
                .map(|k| (k.basis, k.count, k.points))
                .collect::<Vec<_>>(),
            vec![
                (KnownBasis::Exact, Fixed::ONE, Some(n(7))),
                (KnownBasis::Exact, Fixed::ZERO, Some(Fixed::ZERO)),
                // Base 3 is the enemy's: nothing of it is seen.
                (KnownBasis::NotObservable, Fixed::ZERO, None),
            ]
        );
        // Out of supply: shut.
        let own = [OwnBase {
            id: BaseId::new(1),
            supply_t: Fixed::ZERO,
            aircraft_ready: 3,
        }];
        assert_eq!(known_progress(&objectives, &own, &[])[0].count, Fixed::ZERO);
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
            day: 1,
            pulse: None,
            kind: SideEventKind::Whole(EventKind::ClaimReceived {
                side: Side::Allied,
                hex: Hex::new(0, 0),
                claimed_sunk: 2,
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
}
