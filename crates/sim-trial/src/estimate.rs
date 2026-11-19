//! The estimate (decision 0176, as amended on review): what a trial says of
//! one intent of the side tried, read from its worlds as played.
//!
//! It speaks in counts of worlds and never in chances, and in places, days,
//! men, ships and aircraft and never in points. It is data, with a rendering
//! in words for a reader ([`Estimate`]'s `Display`), and reading it changes
//! nothing.
//!
//! Reads: each world's record (its number, its assumptions, each day's
//! bases as the side saw them at the day's end, the plan orders of its first
//! day, why it could not be played); and each world a trial played, through
//! the views of its two sides (rule 6): the side's own command state for the
//! intent's kind, its own events and ships for what it lost, and the other
//! side's for what the trial's enemy lost. A world of a trial is the trial's
//! own invention: reading its enemy's view is not reading the enemy. This is
//! one of the two parts of the crate that take a view of a world it built
//! (amended 14); the other is the play, which records the day's bases and
//! plan orders.

use core::fmt;
use std::collections::BTreeSet;

use sim_core::believed::Standing;
use sim_core::command::{IntentKind, Leave, Maker, PlanOrder, WaitedFor};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId};
use sim_core::refdata::{AircraftRole, ShipKind};
use sim_core::repair::days_of_repair;
use sim_core::side_events::SideEventKind;
use sim_core::state::{ShipLocation, Side};
use sim_core::view::SideView;

use crate::assumption::{Assumption, NotMade, SetAside, Source};
use crate::enemy::other;
use crate::play::{BasesAt, Failed, Played, PlayedDay, PlayedWorld};

/// The end of an intent (decision 0176, 4 and amended 2): the places that
/// must be the side's. For "take a place", its hex; for "control an island",
/// every named place of the island; for "hold", its base's hex.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct End {
    pub places: Vec<Hex>,
}

/// The end of `intent` of the view's side, where it has one an estimate can
/// read: `None` for an intent of another kind, an intent the side does not
/// hold, an island the map does not have or that has no place, and a held
/// base that is on no map of the side's.
pub fn end_of(view: &SideView<'_>, intent: IntentId) -> Option<End> {
    let kind = view.command_state().intents.get(&intent)?.kind;
    end_of_kind(view, kind)
}

pub(crate) fn end_of_kind(view: &SideView<'_>, kind: IntentKind) -> Option<End> {
    let places = match kind {
        IntentKind::TakePlace { hex, .. } => vec![hex],
        IntentKind::ControlIsland { island, .. } => {
            let places: Vec<Hex> = view.island(island)?.places.iter().map(|p| p.hex).collect();
            if places.is_empty() {
                return None;
            }
            places
        }
        IntentKind::Hold { base } => vec![base_hex(view, base)?],
        _ => return None,
    };
    Some(End { places })
}

fn base_hex(view: &SideView<'_>, base: BaseId) -> Option<Hex> {
    view.bases()
        .find(|b| b.id == base)
        .map(|b| b.hex)
        .or_else(|| view.enemy_bases().find(|b| b.id == base).map(|b| b.hex))
}

/// Whether an end is met where bases stand as `bases` says (decision 0176,
/// amended 2): every place of it is the side's, the side holding a base in
/// its hex and no base of the enemy's being there on its map. The staff's
/// own test of a take met. One function, so that it may be applied to a
/// live game's view ([`BasesAt::of`]) as to a world's record.
pub fn met(end: &End, bases: &BasesAt) -> bool {
    end.places
        .iter()
        .all(|hex| bases.own.contains(hex) && !bases.enemy.contains(hex))
}

/// The first day an end is met in a world's days, and whether it was lost
/// again: met at the end of some day and not at the end of the last
/// (amended 4). An end met on the last day is kept.
pub(crate) fn met_in(days: &[PlayedDay], end: &End) -> (Option<u32>, bool) {
    let first = days
        .iter()
        .find(|day| met(end, &day.bases))
        .map(|day| day.day);
    let kept = days.last().is_some_and(|day| met(end, &day.bases));
    (first, first.is_some() && !kept)
}

/// What a side lost over the days of a world played (decision 0176, 8, and
/// amended 12): each its own count from its own events and its own ships.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Losses {
    /// Men lost to the enemy: in a fight, to fire, with a ship, by
    /// surrender.
    pub men_to_enemy: u32,
    /// Men lost unfed.
    pub men_unfed: u32,
    /// Ships sunk, and lost with a base.
    pub ships_sunk: u32,
    /// Ships afloat at the end whose days of repair are 1 or more, a ship in
    /// a yard among them.
    pub ships_out_of_action: u32,
    /// Aircraft lost, by the list the objectives count.
    pub aircraft: u32,
}

impl Losses {
    /// What a side lost, read from its own view of a world: its own events
    /// and its own ships.
    fn of(view: &SideView<'_>) -> Losses {
        let mut losses = Losses::default();
        let mut sunk = BTreeSet::new();
        for event in view.events() {
            let SideEventKind::Whole(kind) = event.kind else {
                continue;
            };
            match kind {
                EventKind::MenLostInFight { lost, .. } | EventKind::MenLostToFire { lost, .. } => {
                    losses.men_to_enemy += lost;
                }
                EventKind::MenLostWithShip { men, .. } | EventKind::UnitSurrendered { men, .. } => {
                    losses.men_to_enemy += men;
                }
                EventKind::MenLostUnfed { lost, .. } => losses.men_unfed += lost,
                EventKind::ShipSunk { ship, .. } | EventKind::ShipLostWithBase { ship, .. } => {
                    sunk.insert(ship);
                }
                EventKind::AircraftLost { aircraft, .. }
                | EventKind::AirGroupLost { aircraft, .. }
                | EventKind::AircraftDestroyedOnGround { aircraft, .. }
                | EventKind::DamagedAircraftDestroyedOnGround { aircraft, .. }
                | EventKind::AircraftWorn {
                    written_off: aircraft,
                    ..
                }
                | EventKind::AircraftDitched {
                    ditched: aircraft, ..
                }
                | EventKind::AircraftDitchedNoDeck { aircraft, .. }
                | EventKind::AircraftWrittenOff { aircraft, .. }
                | EventKind::AirGroupLostWithBase { aircraft, .. } => losses.aircraft += aircraft,
                _ => {}
            }
        }
        losses.ships_sunk = u32::try_from(sunk.len()).expect("a count of ships");
        let figures = view.surface_combat_figures();
        let out = view
            .ships()
            .filter(|ship| match ship.location {
                ShipLocation::Sunk => false,
                ShipLocation::Yard(_) => true,
                ShipLocation::TaskForce(_) | ShipLocation::Port(_) => {
                    figures.is_some_and(|figures| days_of_repair(ship, figures) >= 1)
                }
            })
            .count();
        losses.ships_out_of_action = u32::try_from(out).expect("a count of ships");
        losses
    }
}

/// The lowest, the middle and the highest of some worlds' values (decision
/// 0176, 10): the middle is the value at place ceil(n / 2) of the values in
/// order, counting from 1, never an average of two.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Spread {
    pub lowest: u32,
    pub middle: u32,
    pub highest: u32,
}

impl Spread {
    /// The spread of `values`, `None` where there are none.
    pub fn of(values: &[u32]) -> Option<Spread> {
        let mut sorted = values.to_vec();
        sorted.sort_unstable();
        let n = sorted.len();
        if n == 0 {
            return None;
        }
        Some(Spread {
            lowest: sorted[0],
            middle: sorted[n.div_ceil(2) - 1],
            highest: sorted[n - 1],
        })
    }
}

/// The five costs, each as the lowest, the middle and the highest over some
/// worlds (decision 0176, 8 and 9).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Costs {
    pub men_to_enemy: Spread,
    pub men_unfed: Spread,
    pub ships_sunk: Spread,
    pub ships_out_of_action: Spread,
    pub aircraft: Spread,
}

impl Costs {
    /// The costs over `losses`, `None` where there are none.
    fn of(losses: &[Losses]) -> Option<Costs> {
        let spread =
            |read: fn(&Losses) -> u32| Spread::of(&losses.iter().map(read).collect::<Vec<u32>>());
        Some(Costs {
            men_to_enemy: spread(|l| l.men_to_enemy)?,
            men_unfed: spread(|l| l.men_unfed)?,
            ships_sunk: spread(|l| l.ships_sunk)?,
            ships_out_of_action: spread(|l| l.ships_out_of_action)?,
            aircraft: spread(|l| l.aircraft)?,
        })
    }
}

/// One world's own figures (amended 10): its number, the day its end was
/// first met or none, whether it was lost again, what each side lost, and
/// what its picture assumed.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WorldFigures {
    pub number: u32,
    pub met_day: Option<u32>,
    pub lost_again: bool,
    /// What the side tried lost.
    pub side: Losses,
    /// What the trial's enemy lost: the trial's own reckoning, not a claim
    /// and not a report.
    pub enemy: Losses,
    pub assumptions: Vec<Assumption>,
}

/// How it went, in counts of worlds (decision 0176, 7 and amended 4), each
/// by the worlds' numbers: the worlds; those in which the end was met and
/// kept, with the earliest, the middle and the latest day it was first met;
/// those in which it was met and lost again; those in which it was not met
/// by the last day. For an intent with no end, only the worlds.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Went {
    pub worlds: Vec<u32>,
    pub met_kept: Vec<u32>,
    pub met_days: Option<Spread>,
    pub lost_again: Vec<u32>,
    pub not_met: Vec<u32>,
}

impl Went {
    fn of(figures: &[&WorldFigures], has_end: bool) -> Went {
        let worlds = figures.iter().map(|w| w.number).collect();
        if !has_end {
            return Went {
                worlds,
                met_kept: Vec::new(),
                met_days: None,
                lost_again: Vec::new(),
                not_met: Vec::new(),
            };
        }
        let kept: Vec<&&WorldFigures> = figures
            .iter()
            .filter(|w| w.met_day.is_some() && !w.lost_again)
            .collect();
        Went {
            worlds,
            met_kept: kept.iter().map(|w| w.number).collect(),
            met_days: Spread::of(&kept.iter().filter_map(|w| w.met_day).collect::<Vec<u32>>()),
            lost_again: figures
                .iter()
                .filter(|w| w.lost_again)
                .map(|w| w.number)
                .collect(),
            not_met: figures
                .iter()
                .filter(|w| w.met_day.is_none())
                .map(|w| w.number)
                .collect(),
        }
    }

    /// The counts of rule 7 and the days met, without the worlds' numbers:
    /// what two sides of a parting are compared by.
    fn counts(&self) -> (usize, usize, usize, Option<Spread>) {
        (
            self.met_kept.len(),
            self.lost_again.len(),
            self.not_met.len(),
            self.met_days,
        )
    }
}

/// What a thing assumed is of (amended 9): an enemy base, the men of an
/// estimate, a believed force or body of men, a contact, a pair; or the
/// enemy's belief and intents. Not the numbers given to the things made.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AssumedOf {
    EnemyBase {
        base: BaseId,
        hex: Hex,
        supply_t: Fixed,
        fuel_t: Fixed,
    },
    MenAshore {
        hex: Hex,
        men: u32,
    },
    /// A believed force, by its place in the side's list.
    BelievedForce {
        place: u32,
        name: String,
    },
    /// Believed men, by their place in the side's list.
    BelievedMen {
        place: u32,
        name: String,
        hex: Hex,
        men: u32,
        standing: Standing,
    },
    /// A contact, by its place in the side's list.
    Contact {
        place: u32,
    },
    /// Two contacts that may be one force: one in the worlds it was made
    /// in, two in the others.
    Pair {
        first: u32,
        second: u32,
    },
    EnemyGivenNoBelief,
    EnemyIntent {
        kind: IntentKind,
    },
    EnemyGivenNoIntent,
}

/// What else was assumed of a thing, with the count of worlds it was so in.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AssumedNote {
    /// A force made for it was placed by reach: the count of worlds, and no
    /// list of hexes.
    PlacedByReach { worlds: u32 },
    /// A force made for it, its place not known, was put at an enemy base.
    AtEnemyBase { worlds: u32 },
    /// A contact stood for the believed force, which was not made.
    TakenForContact { contact: u32, worlds: u32 },
    /// No force was made for it.
    NotMade { why: NotMade, worlds: u32 },
    /// A claim raised a contact's kind.
    KindRaised {
        claim: u32,
        from: ShipKind,
        to: ShipKind,
        worlds: u32,
    },
    /// Ships left out, for the enemy has no class of the kind.
    ShipsLeftOut {
        kind: ShipKind,
        count: u32,
        worlds: u32,
    },
    /// A carrier's air group left out.
    GroupLeftOut {
        role: AircraftRole,
        aircraft: u32,
        worlds: u32,
    },
    /// Believed absent: the side ruled it out.
    RuledOut { worlds: u32 },
    /// Believed men set aside, for the side knows better (amended 6).
    SetAside { why: SetAside, worlds: u32 },
}

impl AssumedNote {
    fn worlds_mut(&mut self) -> &mut u32 {
        match self {
            AssumedNote::PlacedByReach { worlds }
            | AssumedNote::AtEnemyBase { worlds }
            | AssumedNote::TakenForContact { worlds, .. }
            | AssumedNote::NotMade { worlds, .. }
            | AssumedNote::KindRaised { worlds, .. }
            | AssumedNote::ShipsLeftOut { worlds, .. }
            | AssumedNote::GroupLeftOut { worlds, .. }
            | AssumedNote::RuledOut { worlds }
            | AssumedNote::SetAside { worlds, .. } => worlds,
        }
    }

    fn same_as(&self, other: &AssumedNote) -> bool {
        let mut a = self.clone();
        let mut b = other.clone();
        *a.worlds_mut() = 0;
        *b.worlds_mut() = 0;
        a == b
    }
}

/// One line of what was assumed (decision 0176, 13, and amended 9): what it
/// is of, the worlds it was made in, for a thing of unknown standing the
/// worlds it was not (for a pair, the worlds it was two forces), and what
/// else was assumed of it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Assumed {
    pub of: AssumedOf,
    pub made_in: Vec<u32>,
    pub not_in: Vec<u32>,
    pub notes: Vec<AssumedNote>,
}

/// What a parting is by (amended 8): a believed force or body of men of
/// unknown standing, by its place in the side's list, or a pair of reports
/// that is one force in some worlds and two in others.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PartedBy {
    BelievedForce {
        place: u32,
        name: String,
    },
    BelievedMen {
        place: u32,
        name: String,
    },
    /// With: the worlds in which the pair is one force; without: two.
    Pair {
        first: u32,
        second: u32,
    },
}

/// One side of a parting: its worlds and its counts (in `went`), and the
/// five costs of each side over its worlds.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PartSide {
    pub went: Went,
    pub side: Option<Costs>,
    pub enemy: Option<Costs>,
}

/// Where the worlds part by a thing the side did not know (amended 8): the
/// worlds with it and the worlds without, side by side. Given only where the
/// two differ in a count of rule 7, the days met, or a cost.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Parting {
    pub by: PartedBy,
    pub with: PartSide,
    pub without: PartSide,
}

/// An intent's end met and kept in some worlds of a trial (amended 11): the
/// worlds, and the middle of the days it was first met in them.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Kept {
    pub worlds: Vec<u32>,
    pub middle_day: Option<u32>,
}

/// Why another plan is named as displaced (amended 11).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Displaced {
    /// It is met and kept in fewer worlds with the course.
    InFewerWorlds,
    /// In as many worlds, and its middle day met is later with the course.
    Later,
}

/// Another intent of the side's with an end (amended 11): met and kept with
/// the course and without, and whether it is displaced.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OtherPlan {
    pub intent: IntentId,
    pub kind: IntentKind,
    pub with_course: Kept,
    pub without: Kept,
    pub displaced: Option<Displaced>,
}

/// An estimate of one intent of the side tried (decision 0176), read from a
/// trial as played and, where given, a trial of the same worlds with an
/// empty course. Data; its `Display` renders it in words for a reader.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Estimate {
    /// The side tried; `None` where the trial had no picture.
    pub side: Option<Side>,
    pub intent: IntentId,
    /// The intent's kind, as the side holds it in the first world played;
    /// `None` where it holds no such intent in any.
    pub kind: Option<IntentKind>,
    /// Its end, where it has one an estimate can read.
    pub end: Option<End>,
    /// The count of worlds the trial was asked for.
    pub asked: u32,
    /// The last day of the horizon, as played; `None` where no world was.
    pub last_day: Option<u32>,
    /// How it went, over the worlds played.
    pub went: Went,
    /// The worlds that could not be played, counted apart and in none of the
    /// other counts.
    pub could_not: Vec<Failed>,
    /// What it cost the side, over the worlds played; `None` where none was.
    pub side_costs: Option<Costs>,
    /// What the trial's enemy lost: the trial's own reckoning, not a claim,
    /// not a report.
    pub enemy_costs: Option<Costs>,
    /// The intent's plan order (amended 5), as the side's staff worked it at
    /// the end of the first day in world 1, or the first world with one; at
    /// the end of the day the course was taken in, where that is later
    /// (decision 0177, 6).
    pub plan_order: Option<PlanOrder>,
    /// The world whose plan order it is.
    pub plan_order_world: Option<u32>,
    /// The day at whose end it was recorded: the day the course was taken
    /// in, or the first day played.
    pub plan_order_day: Option<u32>,
    /// The worlds whose plan order of the intent differs from it.
    pub plan_order_differs_in: Vec<u32>,
    /// What was assumed, grouped by what it is of.
    pub assumed: Vec<Assumed>,
    /// The leaves the course gave that the side had not given.
    pub leaves_given: Vec<Leave>,
    /// The count of worlds is not a power of two: the things of unknown
    /// standing were not tried evenly.
    pub uneven: bool,
    /// Where the worlds part.
    pub partings: Vec<Parting>,
    /// The side's other intents with an end, with the course and without;
    /// `None` where no trial of things as they stand was given.
    pub other_plans: Option<Vec<OtherPlan>>,
    /// Each world played's own figures, in the order of their numbers.
    pub worlds: Vec<WorldFigures>,
}

/// The worlds of a trial that were played, each whole to its horizon.
fn played_worlds(played: &Played) -> Vec<&PlayedWorld> {
    played
        .worlds
        .iter()
        .filter(|w| w.failed.is_none() && w.world.is_some())
        .collect()
}

/// The estimate of `intent` of the side tried, from a trial as played
/// (decision 0176), and, where given, a trial of the same worlds with an
/// empty course, things as they stand, to compare its other plans with.
pub fn estimate(played: &Played, intent: IntentId, as_they_stand: Option<&Played>) -> Estimate {
    let side = played.side;
    let worlds = played_worlds(played);
    // The intent's kind and end, as the side holds it in the first world
    // played that has it.
    let found = side.and_then(|side| {
        worlds.iter().find_map(|w| {
            let view = w.world.as_ref()?.side_view(side);
            let kind = view.command_state().intents.get(&intent)?.kind;
            Some((kind, end_of_kind(&view, kind)))
        })
    });
    let kind = found.as_ref().map(|(kind, _)| *kind);
    let end = found.and_then(|(_, end)| end);
    let figures: Vec<WorldFigures> = side
        .map(|side| {
            worlds
                .iter()
                .map(|w| world_figures(w, side, end.as_ref()))
                .collect()
        })
        .unwrap_or_default();
    let all: Vec<&WorldFigures> = figures.iter().collect();
    let went = Went::of(&all, end.is_some());
    let side_losses: Vec<Losses> = figures.iter().map(|w| w.side).collect();
    let enemy_losses: Vec<Losses> = figures.iter().map(|w| w.enemy).collect();

    // The plan order (amended 5): world 1's, or the first world's that has
    // one, and the worlds whose differs. It is the one recorded at the end
    // of the day the course was taken in, where the course has a command
    // and that day was played (decision 0177, 6); otherwise the first day's.
    let orders_of = |w: &PlayedWorld| -> Option<Vec<PlanOrder>> {
        match &w.taken_in {
            Some(taken) => Some(taken.plan_orders.clone()),
            None => w.plan_orders.clone(),
        }
    };
    let order_of = |w: &PlayedWorld| -> Option<PlanOrder> {
        orders_of(w)?
            .into_iter()
            .find(|order| order.intent == intent)
    };
    let reference = played.worlds.iter().find(|w| orders_of(w).is_some());
    let plan_order = reference.and_then(order_of);
    let plan_order_differs_in = played
        .worlds
        .iter()
        .filter(|w| orders_of(w).is_some() && order_of(w) != plan_order)
        .map(|w| w.number)
        .collect();

    let other_plans = as_they_stand.map(|stand| other_plans(played, stand, intent));
    Estimate {
        side,
        intent,
        kind,
        end: end.clone(),
        asked: u32::try_from(played.worlds.len()).expect("a count of worlds"),
        last_day: worlds
            .iter()
            .filter_map(|w| w.days.last().map(|d| d.day))
            .max(),
        went,
        could_not: played.failed().into_iter().cloned().collect(),
        side_costs: Costs::of(&side_losses),
        enemy_costs: Costs::of(&enemy_losses),
        plan_order,
        plan_order_world: reference.map(|w| w.number),
        plan_order_day: reference.and_then(|w| {
            w.taken_in
                .as_ref()
                .map(|taken| taken.day)
                .or_else(|| w.days.first().map(|d| d.day))
        }),
        plan_order_differs_in,
        assumed: assumed(&figures),
        leaves_given: played.leaves_given.clone(),
        uneven: played.uneven,
        partings: partings(&figures, end.is_some()),
        other_plans,
        worlds: figures,
    }
}

/// One world's own figures, read from its record and its two sides' views.
fn world_figures(w: &PlayedWorld, side: Side, end: Option<&End>) -> WorldFigures {
    let (met_day, lost_again) = end.map_or((None, false), |end| met_in(&w.days, end));
    let (own, enemy) = w.world.as_ref().map_or_else(
        || (Losses::default(), Losses::default()),
        |built| {
            (
                Losses::of(&built.side_view(side)),
                Losses::of(&built.side_view(other(side))),
            )
        },
    );
    WorldFigures {
        number: w.number,
        met_day,
        lost_again,
        side: own,
        enemy,
        assumptions: w.assumptions.clone(),
    }
}

/// The side's other intents with an end (amended 11): the commander's
/// intents in the first world played of the trial of things as they stand,
/// other than `intent`, each met and kept with the course and without.
fn other_plans(played: &Played, stand: &Played, intent: IntentId) -> Vec<OtherPlan> {
    let Some(side) = stand.side.or(played.side) else {
        return Vec::new();
    };
    let without = played_worlds(stand);
    let with = played_worlds(played);
    let Some(first) = without.iter().find_map(|w| w.world.as_ref()) else {
        return Vec::new();
    };
    let view = first.side_view(side);
    let mut out = Vec::new();
    for held in view.command_state().intents.values() {
        if held.id == intent || held.maker != Maker::Commander {
            continue;
        }
        let Some(end) = end_of_kind(&view, held.kind) else {
            continue;
        };
        out.push(other_plan(held.id, held.kind, &end, &with, &without));
    }
    out
}

/// One other plan, met and kept with the course and without.
pub(crate) fn other_plan(
    intent: IntentId,
    kind: IntentKind,
    end: &End,
    with: &[&PlayedWorld],
    without: &[&PlayedWorld],
) -> OtherPlan {
    let with_course = kept(with, end);
    let without = kept(without, end);
    let displaced = if with_course.worlds.len() < without.worlds.len() {
        Some(Displaced::InFewerWorlds)
    } else if matches!(
        (with_course.middle_day, without.middle_day),
        (Some(a), Some(b)) if a > b
    ) {
        Some(Displaced::Later)
    } else {
        None
    };
    OtherPlan {
        intent,
        kind,
        with_course,
        without,
        displaced,
    }
}

/// The worlds in which an end is met and kept, and the middle of their first
/// days met.
fn kept(worlds: &[&PlayedWorld], end: &End) -> Kept {
    let mut numbers = Vec::new();
    let mut days = Vec::new();
    for w in worlds {
        if let (Some(day), false) = met_in(&w.days, end) {
            numbers.push(w.number);
            days.push(day);
        }
    }
    Kept {
        worlds: numbers,
        middle_day: Spread::of(&days).map(|spread| spread.middle),
    }
}

/// What a thing assumed is of, and what was made of it, for one assumption;
/// `None` for one that names a force made by its number, which is read with
/// the force's source.
enum Read {
    Made(AssumedOf),
    NotIn(AssumedOf),
    Note(AssumedOf, AssumedNote),
}

fn source_of(source: &Source) -> AssumedOf {
    match source {
        Source::Contact(place) => AssumedOf::Contact { place: *place },
        Source::Pair(first, second) => AssumedOf::Pair {
            first: *first,
            second: *second,
        },
        Source::Believed(place, name) => AssumedOf::BelievedForce {
            place: *place,
            name: name.clone(),
        },
    }
}

/// What was assumed, grouped by what it is of (amended 9), in the order of
/// first appearance over the worlds in the order of their numbers.
fn assumed(figures: &[WorldFigures]) -> Vec<Assumed> {
    let mut lines: Vec<Assumed> = Vec::new();
    for w in figures {
        // The source of each force made in this world, by its number, for
        // the assumption that places it.
        let mut made_for = Vec::new();
        for assumption in &w.assumptions {
            let read = match assumption {
                Assumption::EnemyBase {
                    base,
                    hex,
                    supply_t,
                    fuel_t,
                } => Read::Made(AssumedOf::EnemyBase {
                    base: *base,
                    hex: *hex,
                    supply_t: *supply_t,
                    fuel_t: *fuel_t,
                }),
                Assumption::MenAshore { hex, men, .. } => Read::Made(AssumedOf::MenAshore {
                    hex: *hex,
                    men: *men,
                }),
                Assumption::KindRaised {
                    contact,
                    claim,
                    from,
                    to,
                } => Read::Note(
                    AssumedOf::Contact { place: *contact },
                    AssumedNote::KindRaised {
                        claim: *claim,
                        from: *from,
                        to: *to,
                        worlds: 1,
                    },
                ),
                Assumption::Pair { first, second, one } => {
                    let of = AssumedOf::Pair {
                        first: *first,
                        second: *second,
                    };
                    if *one {
                        Read::Made(of)
                    } else {
                        Read::NotIn(of)
                    }
                }
                Assumption::ShipsLeftOut {
                    source,
                    kind,
                    count,
                } => Read::Note(
                    source_of(source),
                    AssumedNote::ShipsLeftOut {
                        kind: *kind,
                        count: *count,
                        worlds: 1,
                    },
                ),
                Assumption::GroupLeftOut {
                    source,
                    role,
                    aircraft,
                } => Read::Note(
                    source_of(source),
                    AssumedNote::GroupLeftOut {
                        role: *role,
                        aircraft: *aircraft,
                        worlds: 1,
                    },
                ),
                Assumption::ForceMade { force, source, .. } => {
                    made_for.push((*force, source_of(source)));
                    match source {
                        // A pair's line is made in the worlds it is one.
                        Source::Pair(..) => continue,
                        _ => Read::Made(source_of(source)),
                    }
                }
                Assumption::ForceNotMade { source, why } => Read::Note(
                    source_of(source),
                    AssumedNote::NotMade {
                        why: *why,
                        worlds: 1,
                    },
                ),
                Assumption::Placed { force, .. } | Assumption::AtEnemyBase { force, .. } => {
                    let Some((_, of)) = made_for.iter().find(|(made, _)| made == force) else {
                        continue;
                    };
                    let note = if matches!(assumption, Assumption::Placed { .. }) {
                        AssumedNote::PlacedByReach { worlds: 1 }
                    } else {
                        AssumedNote::AtEnemyBase { worlds: 1 }
                    };
                    Read::Note(of.clone(), note)
                }
                Assumption::RuledOut { place, name } => Read::Note(
                    AssumedOf::BelievedForce {
                        place: *place,
                        name: name.clone(),
                    },
                    AssumedNote::RuledOut { worlds: 1 },
                ),
                Assumption::UnknownNotInWorld { place, name } => {
                    Read::NotIn(AssumedOf::BelievedForce {
                        place: *place,
                        name: name.clone(),
                    })
                }
                Assumption::TakenForContact {
                    place,
                    name,
                    contact,
                } => Read::Note(
                    AssumedOf::BelievedForce {
                        place: *place,
                        name: name.clone(),
                    },
                    AssumedNote::TakenForContact {
                        contact: *contact,
                        worlds: 1,
                    },
                ),
                Assumption::EnemyGivenNoBelief => Read::Made(AssumedOf::EnemyGivenNoBelief),
                Assumption::EnemyIntent { kind } => {
                    Read::Made(AssumedOf::EnemyIntent { kind: *kind })
                }
                Assumption::EnemyGivenNoIntent => Read::Made(AssumedOf::EnemyGivenNoIntent),
                Assumption::BelievedMenMade {
                    place,
                    name,
                    hex,
                    men,
                    standing,
                    ..
                } => Read::Made(AssumedOf::BelievedMen {
                    place: *place,
                    name: name.clone(),
                    hex: *hex,
                    men: *men,
                    standing: *standing,
                }),
                Assumption::BelievedMenRuledOut {
                    place,
                    name,
                    hex,
                    men,
                } => Read::Note(
                    AssumedOf::BelievedMen {
                        place: *place,
                        name: name.clone(),
                        hex: *hex,
                        men: *men,
                        standing: Standing::Absent,
                    },
                    AssumedNote::RuledOut { worlds: 1 },
                ),
                Assumption::BelievedMenNotInWorld {
                    place,
                    name,
                    hex,
                    men,
                } => Read::NotIn(AssumedOf::BelievedMen {
                    place: *place,
                    name: name.clone(),
                    hex: *hex,
                    men: *men,
                    standing: Standing::Unknown,
                }),
                Assumption::BelievedMenSetAside {
                    place,
                    name,
                    hex,
                    men,
                    standing,
                    why,
                } => Read::Note(
                    AssumedOf::BelievedMen {
                        place: *place,
                        name: name.clone(),
                        hex: *hex,
                        men: *men,
                        standing: *standing,
                    },
                    AssumedNote::SetAside {
                        why: *why,
                        worlds: 1,
                    },
                ),
            };
            let of = match &read {
                Read::Made(of) | Read::NotIn(of) | Read::Note(of, _) => of.clone(),
            };
            let at = match lines.iter().position(|line| line.of == of) {
                Some(at) => at,
                None => {
                    lines.push(Assumed {
                        of,
                        made_in: Vec::new(),
                        not_in: Vec::new(),
                        notes: Vec::new(),
                    });
                    lines.len() - 1
                }
            };
            let line = &mut lines[at];
            match read {
                Read::Made(_) => {
                    if line.made_in.last() != Some(&w.number) {
                        line.made_in.push(w.number);
                    }
                }
                Read::NotIn(_) => {
                    if line.not_in.last() != Some(&w.number) {
                        line.not_in.push(w.number);
                    }
                }
                Read::Note(_, note) => match line.notes.iter_mut().find(|n| n.same_as(&note)) {
                    Some(held) => *held.worlds_mut() += 1,
                    None => line.notes.push(note),
                },
            }
        }
    }
    lines
}

/// Where the worlds part (amended 8): by each believed force or body of men
/// of unknown standing, and by each pair that is one force in some worlds
/// and two in others, in the order of first appearance; each given where its
/// two sides differ.
fn partings(figures: &[WorldFigures], has_end: bool) -> Vec<Parting> {
    // What each world lacks, or holds as one force.
    let mut keys: Vec<PartedBy> = Vec::new();
    for w in figures {
        for assumption in &w.assumptions {
            let key = match assumption {
                Assumption::UnknownNotInWorld { place, name } => PartedBy::BelievedForce {
                    place: *place,
                    name: name.clone(),
                },
                Assumption::BelievedMenNotInWorld { place, name, .. } => PartedBy::BelievedMen {
                    place: *place,
                    name: name.clone(),
                },
                Assumption::Pair { first, second, .. } => PartedBy::Pair {
                    first: *first,
                    second: *second,
                },
                _ => continue,
            };
            if !keys.contains(&key) {
                keys.push(key);
            }
        }
    }
    let mut out = Vec::new();
    for key in keys {
        let without_it = |w: &WorldFigures| {
            w.assumptions
                .iter()
                .any(|assumption| match (&key, assumption) {
                    (
                        PartedBy::BelievedForce { place, .. },
                        Assumption::UnknownNotInWorld { place: p, .. },
                    )
                    | (
                        PartedBy::BelievedMen { place, .. },
                        Assumption::BelievedMenNotInWorld { place: p, .. },
                    ) => place == p,
                    (
                        PartedBy::Pair { first, second },
                        Assumption::Pair {
                            first: f,
                            second: s,
                            one: false,
                        },
                    ) => first == f && second == s,
                    _ => false,
                })
        };
        let (without, with): (Vec<&WorldFigures>, Vec<&WorldFigures>) =
            figures.iter().partition(|w| without_it(w));
        if with.is_empty() || without.is_empty() {
            continue;
        }
        let with = part_side(&with, has_end);
        let without = part_side(&without, has_end);
        let differ = with.went.counts() != without.went.counts()
            || with.side != without.side
            || with.enemy != without.enemy;
        if differ {
            out.push(Parting {
                by: key,
                with,
                without,
            });
        }
    }
    out
}

fn part_side(worlds: &[&WorldFigures], has_end: bool) -> PartSide {
    let side: Vec<Losses> = worlds.iter().map(|w| w.side).collect();
    let enemy: Vec<Losses> = worlds.iter().map(|w| w.enemy).collect();
    PartSide {
        went: Went::of(worlds, has_end),
        side: Costs::of(&side),
        enemy: Costs::of(&enemy),
    }
}

// ---------------------------------------------------------------- Words

/// "2 of 4 worlds (worlds 2 and 4)", "none of 4 worlds".
struct OfWorlds<'a>(&'a [u32], usize);

impl fmt::Display for OfWorlds<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (worlds, of) = (self.0, self.1);
        let noun = if of == 1 { "world" } else { "worlds" };
        if worlds.is_empty() {
            return write!(f, "none of {of} {noun}");
        }
        write!(f, "{} of {of} {noun} (", worlds.len())?;
        write!(
            f,
            "{} {})",
            if worlds.len() == 1 { "world" } else { "worlds" },
            list(worlds)
        )
    }
}

/// "1", "1 and 3", "1, 2 and 4".
fn list(numbers: &[u32]) -> String {
    match numbers {
        [] => String::new(),
        [one] => one.to_string(),
        [rest @ .., last] => format!(
            "{} and {last}",
            rest.iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn spread_words(spread: Spread) -> String {
    format!("{}, {}, {}", spread.lowest, spread.middle, spread.highest)
}

fn costs_words(costs: &Costs) -> String {
    format!(
        "men lost to the enemy {}; men lost unfed {}; ships sunk {}; ships out of action at the \
         end {}; aircraft lost {}",
        spread_words(costs.men_to_enemy),
        spread_words(costs.men_unfed),
        spread_words(costs.ships_sunk),
        spread_words(costs.ships_out_of_action),
        spread_words(costs.aircraft)
    )
}

pub(crate) fn intent_words(kind: IntentKind) -> String {
    match kind {
        IntentKind::TakePlace { hex, .. } => format!("take the place at {hex}"),
        IntentKind::ControlIsland { island, .. } => format!("control island {island}"),
        IntentKind::Hold { base } => format!("hold base {}", base.raw()),
        IntentKind::KeepSupplied { base, .. } => format!("keep base {} supplied", base.raw()),
        other => format!("{other:?}"),
    }
}

fn went_words(went: &Went, last_day: Option<u32>) -> String {
    let of = went.worlds.len();
    let mut out = format!("met and kept in {}", OfWorlds(&went.met_kept, of));
    if let Some(days) = went.met_days {
        out.push_str(&format!(
            ": the earliest day {}, the middle day {}, the latest day {}",
            days.lowest, days.middle, days.highest
        ));
    }
    out.push_str(&format!(
        "; met and lost again in {}; not met",
        OfWorlds(&went.lost_again, of)
    ));
    if let Some(day) = last_day {
        out.push_str(&format!(" by day {day}"));
    }
    out.push_str(&format!(" in {}", OfWorlds(&went.not_met, of)));
    out
}

fn assumed_words(line: &Assumed) -> String {
    let what = match &line.of {
        AssumedOf::EnemyBase {
            base,
            hex,
            supply_t,
            fuel_t,
        } => format!(
            "base {} of the enemy's at {hex}, with {supply_t} t of supply and {fuel_t} t of fuel",
            base.raw()
        ),
        AssumedOf::MenAshore { hex, men } => {
            format!("{men} men at {hex}, from the side's estimate")
        }
        AssumedOf::BelievedForce { place, name } => {
            format!("believed force {place} ({name})")
        }
        AssumedOf::BelievedMen {
            place,
            name,
            hex,
            men,
            standing,
        } => format!(
            "{men} men believed at {hex}, believed men {place} ({name}), standing {}",
            standing.name()
        ),
        AssumedOf::Contact { place } => format!("contact {place}"),
        AssumedOf::Pair { first, second } => {
            format!("contacts {first} and {second}, one force")
        }
        AssumedOf::EnemyGivenNoBelief => "the enemy given no belief".to_string(),
        AssumedOf::EnemyIntent { kind } => format!("the enemy taken to intend {kind:?}"),
        AssumedOf::EnemyGivenNoIntent => "the enemy given no intent".to_string(),
    };
    let mut out = what;
    if !line.made_in.is_empty() {
        out.push_str(&format!(", in {} worlds", line.made_in.len()));
    }
    if !line.not_in.is_empty() {
        out.push_str(&format!(", not in {}", line.not_in.len()));
    }
    for note in &line.notes {
        let words = match note {
            AssumedNote::PlacedByReach { worlds } => {
                format!("placed by its reach in {worlds} worlds")
            }
            AssumedNote::AtEnemyBase { worlds } => {
                format!("put at a base of the enemy's in {worlds} worlds")
            }
            AssumedNote::TakenForContact { contact, worlds } => {
                format!("contact {contact} taken for it in {worlds} worlds")
            }
            AssumedNote::NotMade { why, worlds } => format!(
                "not made in {worlds} worlds: {}",
                match why {
                    NotMade::NoShip => "no ship of it built",
                    NotMade::NowhereToBe => "nowhere ships can be within its reach",
                    NotMade::NoEnemyBase => "no base of the enemy's to be at",
                }
            ),
            AssumedNote::KindRaised {
                claim,
                from,
                to,
                worlds,
            } => format!(
                "raised from a {} to a {} by claim {claim} in {worlds} worlds",
                from.name(),
                to.name()
            ),
            AssumedNote::ShipsLeftOut {
                kind,
                count,
                worlds,
            } => format!("{count} {} left out in {worlds} worlds", kind.name()),
            AssumedNote::GroupLeftOut {
                role,
                aircraft,
                worlds,
            } => {
                format!("an air group of {aircraft} {role:?} aircraft left out in {worlds} worlds")
            }
            AssumedNote::RuledOut { worlds } => {
                format!("believed absent, ruled out in {worlds} worlds")
            }
            AssumedNote::SetAside { why, worlds } => match why {
                SetAside::CountSince { day } => format!(
                    "set aside in {worlds} worlds: the side has a count of the place since, of \
                     day {day}"
                ),
                SetAside::PlaceIsTheSides => {
                    format!("set aside in {worlds} worlds: the place is the side's")
                }
            },
        };
        out.push_str("; ");
        out.push_str(&words);
    }
    out
}

fn plan_order_words(order: &PlanOrder) -> String {
    let mut parts = Vec::new();
    if let Some(men) = order.men {
        parts.push(format!(
            "men wanted {}, to hand {}",
            men.wanted, men.to_hand
        ));
    }
    let waits: Vec<String> = order
        .waits_for
        .iter()
        .filter(|w| !w.met)
        .map(|w| match w.thing {
            WaitedFor::Men { wanted, to_hand } => {
                format!("men, {wanted} wanted and {to_hand} to hand")
            }
            WaitedFor::Leave(leave) => format!("leave to {}", leave.name()),
            WaitedFor::Day(day) => format!("day {day}"),
            WaitedFor::Force(awaited) => format!("{awaited:?}"),
            WaitedFor::Intent(id) => format!("intent {}", id.raw()),
            WaitedFor::Knowledge(knowledge) => format!("{knowledge:?}"),
        })
        .collect();
    if waits.is_empty() {
        parts.push("it waits for nothing".to_string());
    } else {
        parts.push(format!("it waits for {}", waits.join(", ")));
    }
    match order.expected_ashore {
        Some(day) => parts.push(format!("the day expected ashore is day {day}")),
        None => parts.push("no day expected ashore".to_string()),
    }
    if let Some(need) = &order.need_to_know {
        parts.push(format!(
            "must be known: {:?}, {}",
            need.asked,
            if need.met { "held" } else { "not held" }
        ));
    }
    if !order.tasks.is_empty() {
        let tasks: Vec<String> = order
            .tasks
            .iter()
            .map(|task| format!("intent {}, {}", task.intent.raw(), intent_words(task.kind)))
            .collect();
        parts.push(format!("its tasks: {}", tasks.join("; ")));
    }
    parts.join("; ")
}

fn part_side_words(part: &PartSide) -> String {
    let of = part.went.worlds.len();
    let mut out = format!(
        "worlds {}: {} worlds; met and kept in {}",
        list(&part.went.worlds),
        of,
        part.went.met_kept.len()
    );
    if let Some(days) = part.went.met_days {
        out.push_str(&format!(
            ", on day {}, {} and {}",
            days.lowest, days.middle, days.highest
        ));
    }
    out.push_str(&format!(
        ", lost again in {}, not met in {}",
        part.went.lost_again.len(),
        part.went.not_met.len()
    ));
    if let Some(costs) = &part.side {
        out.push_str(&format!("; the side lost {}", costs_words(costs)));
    }
    if let Some(costs) = &part.enemy {
        out.push_str(&format!("; the enemy lost {}", costs_words(costs)));
    }
    out
}

impl fmt::Display for Estimate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side = self.side.map_or("the", Side::name);
        let what = self.kind.map_or_else(
            || "an intent the side does not hold".to_string(),
            intent_words,
        );
        writeln!(
            f,
            "The estimate of intent {} of the {side} side, {what}, from a trial of {} worlds.",
            self.intent.raw(),
            self.asked
        )?;
        let played = self.went.worlds.len();
        if played == 0 {
            writeln!(
                f,
                "None of {} worlds could be played: there is no day and no cost to give.",
                self.asked
            )?;
        } else {
            write!(f, "How it went: {played} of {} worlds played. ", self.asked)?;
            if self.end.is_some() {
                writeln!(f, "Its end: {}.", went_words(&self.went, self.last_day))?;
            } else {
                writeln!(
                    f,
                    "The intent has no end an estimate can read: only its costs are given."
                )?;
            }
        }
        if !self.could_not.is_empty() {
            let numbers: Vec<u32> = self.could_not.iter().map(|failed| failed.world).collect();
            writeln!(
                f,
                "Could not be played: {} of {} worlds (worlds {}), counted in none of the rest.",
                numbers.len(),
                self.asked,
                list(&numbers)
            )?;
        }
        if let Some(costs) = &self.side_costs {
            writeln!(
                f,
                "What it cost the side, lowest, middle and highest over {played} worlds: {}.",
                costs_words(costs)
            )?;
        }
        if let Some(costs) = &self.enemy_costs {
            writeln!(
                f,
                "What the trial's enemy lost, the trial's own reckoning and not a claim or a \
                 report: {}.",
                costs_words(costs)
            )?;
        }
        if let (Some(order), Some(world)) = (&self.plan_order, self.plan_order_world) {
            let day = self
                .plan_order_day
                .map_or_else(|| "its first day".to_string(), |day| format!("day {day}"));
            write!(
                f,
                "What it needs and has, from world {world}'s plan order at the end of {day}: {}.",
                plan_order_words(order)
            )?;
            if self.plan_order_differs_in.is_empty() {
                writeln!(f, " The plan order is the same in every world.")?;
            } else {
                writeln!(
                    f,
                    " It differs in {} worlds (worlds {}).",
                    self.plan_order_differs_in.len(),
                    list(&self.plan_order_differs_in)
                )?;
            }
        }
        if !self.assumed.is_empty() || !self.leaves_given.is_empty() {
            writeln!(f, "What was assumed:")?;
            for line in &self.assumed {
                writeln!(f, "- {}.", assumed_words(line))?;
            }
            for leave in &self.leaves_given {
                writeln!(
                    f,
                    "- leave to {}, given by the course and not by the side.",
                    leave.name()
                )?;
            }
        }
        if self.uneven {
            writeln!(
                f,
                "The count of worlds is not a power of two: what the side did not know was not \
                 tried evenly."
            )?;
        }
        for parting in &self.partings {
            let by = match &parting.by {
                PartedBy::BelievedForce { place, name } => {
                    format!("believed force {place} ({name})")
                }
                PartedBy::BelievedMen { place, name } => format!("believed men {place} ({name})"),
                PartedBy::Pair { first, second } => {
                    format!("contacts {first} and {second}, one force or two")
                }
            };
            writeln!(
                f,
                "Where the worlds part, by {by}. With: {}. Without: {}.",
                part_side_words(&parting.with),
                part_side_words(&parting.without)
            )?;
        }
        match &self.other_plans {
            None => writeln!(f, "No trial of things as they stand was given.")?,
            Some(plans) if plans.is_empty() => {
                writeln!(f, "The side has no other plan with an end.")?;
            }
            Some(plans) => {
                for plan in plans {
                    write!(
                        f,
                        "Intent {}, {}: met and kept in {} worlds with the course and {} without",
                        plan.intent.raw(),
                        intent_words(plan.kind),
                        plan.with_course.worlds.len(),
                        plan.without.worlds.len()
                    )?;
                    match plan.displaced {
                        Some(Displaced::InFewerWorlds) => {
                            writeln!(f, ": displaced, met in fewer worlds.")?;
                        }
                        Some(Displaced::Later) => writeln!(
                            f,
                            ": displaced, met later, the middle day {} against {}.",
                            plan.with_course.middle_day.unwrap_or(0),
                            plan.without.middle_day.unwrap_or(0)
                        )?,
                        None => writeln!(f, ".")?,
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "estimate_tests.rs"]
mod tests;
