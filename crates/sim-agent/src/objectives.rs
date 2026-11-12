//! The OBJECTIVES part of the briefing and the result at the end of a game
//! (decision 0054).
//!
//! The part is made from a [`SideView`]: the side's own mission and
//! objectives, and its progress as far as it can know it (decision 7). Never
//! the enemy's mission or objectives. The result is handed in by whoever holds
//! the game, once the game is over (decision 8): this module only gives it
//! words and numbers, and reads no truth.

use serde::Serialize;
use sim_core::objectives::{GameResult, KnownProgress, Level, Objective, Outcome, SideResult};
use sim_core::refdata::ShipKind;
use sim_core::view::SideView;

use crate::common::{Ctx, Dec, Handle};

/// The mission, the objectives with their points, and what the side knows of
/// its progress. `mission` is `None` and `objectives` empty when the scenario
/// has no objectives. `result` is `None` until the game is over.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ObjectivesPart {
    pub mission: Option<String>,
    pub objectives: Vec<ObjectiveLine>,
    pub result: Option<ResultPart>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ObjectiveLine {
    /// `ships_sunk`, `ships_crippled`, `aircraft_destroyed`, `cargo_landed`
    /// or `base_open`.
    pub kind: &'static str,
    /// The base of `cargo_landed` and `base_open`.
    pub base: Option<Handle>,
    /// For `base_open`: true if the side wants the base shut.
    pub closed: Option<bool>,
    /// For the two ship objectives: the points for each kind of ship.
    pub ship_points: Vec<ShipKindPoints>,
    /// Points for one aircraft (`aircraft_destroyed`), one ton
    /// (`cargo_landed`) or the base being as wanted (`base_open`).
    pub points: Option<Dec>,
    /// What the side knows of its progress, from its own units and the events
    /// it may see.
    pub known: KnownLine,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ShipKindPoints {
    pub kind: &'static str,
    pub points: Dec,
}

/// Progress as the side knows it. `basis` is `exact` (its own units and
/// events: as good as the truth), `claims` (the enemy ships and aircraft its
/// own claims say it sank or shot down or destroyed on the ground, which can
/// be wrong; `points` are for the kind of ship each claim names, or `None`
/// where none does) or `not_observable` (no unit of its own and no event it
/// may see shows this: `count` is 0 and says nothing; the umpire counts it at
/// the end of the game).
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct KnownLine {
    pub basis: &'static str,
    pub count: Dec,
    pub points: Option<Dec>,
}

/// The umpire's result for both sides, told at the end of the game.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ResultPart {
    pub allied: SideResultPart,
    pub japanese: SideResultPart,
    /// The scenario's note of how the day went in history: words, not a
    /// computed score.
    pub history: Option<String>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct SideResultPart {
    pub objectives: Vec<ScoreLine>,
    pub total: Dec,
    /// This side's points less the other side's.
    pub margin: Dec,
    /// `victory`, `defeat` or `stalemate`.
    pub result: &'static str,
    /// `minor`, `major` or `decisive`; `None` for a stalemate.
    pub level: Option<&'static str>,
    /// `minor victory`, `stalemate` and so on.
    pub outcome: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ScoreLine {
    pub kind: &'static str,
    pub base: Option<Handle>,
    pub closed: Option<bool>,
    /// Ships, aircraft, tons, or 1 or 0 for a base.
    pub count: Dec,
    /// For the two ship objectives: the ships counted, by kind.
    pub ships: Vec<ShipsOfKind>,
    pub points: Dec,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ShipsOfKind {
    pub kind: &'static str,
    pub ships: u32,
}

fn ship_points(points: &sim_core::objectives::ShipPoints) -> Vec<ShipKindPoints> {
    ShipKind::ALL
        .into_iter()
        .filter_map(|kind| {
            points.0.get(&kind).map(|p| ShipKindPoints {
                kind: kind.name(),
                points: Dec(*p),
            })
        })
        .collect()
}

fn line(ctx: &Ctx<'_>, objective: &Objective, known: &KnownProgress) -> ObjectiveLine {
    let mut line = ObjectiveLine {
        kind: objective.name(),
        base: objective.base().map(|b| ctx.base(b)),
        closed: None,
        ship_points: Vec::new(),
        points: None,
        known: KnownLine {
            basis: known.basis.name(),
            count: Dec(known.count),
            points: known.points.map(Dec),
        },
    };
    match objective {
        Objective::ShipsSunk(points) | Objective::ShipsCrippled(points) => {
            line.ship_points = ship_points(points);
        }
        Objective::AircraftDestroyed { points } => line.points = Some(Dec(*points)),
        Objective::CargoLanded { points_per_t, .. } => line.points = Some(Dec(*points_per_t)),
        Objective::BaseOpen { closed, points, .. } => {
            line.closed = Some(*closed);
            line.points = Some(Dec(*points));
        }
    }
    line
}

/// The part for one side's view. The side's own mission and objectives only.
pub(crate) fn objectives_part(ctx: &Ctx<'_>, view: &SideView<'_>) -> ObjectivesPart {
    let known = view.known_progress();
    ObjectivesPart {
        mission: view.mission().map(str::to_string),
        objectives: view
            .objectives()
            .iter()
            .zip(&known)
            .map(|(objective, known)| line(ctx, objective, known))
            .collect(),
        result: None,
    }
}

fn side_result(ctx: &Ctx<'_>, result: &SideResult) -> SideResultPart {
    let (kind, level) = match result.outcome {
        Outcome::Victory(level) => ("victory", Some(level)),
        Outcome::Defeat(level) => ("defeat", Some(level)),
        Outcome::Stalemate => ("stalemate", None),
    };
    SideResultPart {
        objectives: result
            .objectives
            .iter()
            .map(|o| ScoreLine {
                kind: o.kind,
                base: o.base.map(|b| ctx.base(b)),
                closed: (o.kind == "base_open").then_some(o.closed),
                count: Dec(o.count),
                ships: o
                    .by_kind
                    .iter()
                    .map(|(kind, ships)| ShipsOfKind {
                        kind: kind.name(),
                        ships: *ships,
                    })
                    .collect(),
                points: Dec(o.points),
            })
            .collect(),
        total: Dec(result.total),
        margin: Dec(result.margin),
        result: kind,
        level: level.map(Level::name),
        outcome: result.outcome.name(),
    }
}

/// The umpire's result as it is told to a side at the end of the game. The
/// view is used for the names of bases only.
pub fn result_part(view: &SideView<'_>, result: &GameResult) -> ResultPart {
    let ctx = Ctx::new(*view);
    ResultPart {
        allied: side_result(&ctx, &result.allied),
        japanese: side_result(&ctx, &result.japanese),
        history: result.history.clone(),
    }
}
