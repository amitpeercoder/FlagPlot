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
    /// The margins (a side's points less the enemy's) that make a minor, a
    /// major and a decisive victory or defeat, from day 1. `None` when the
    /// scenario has no objectives.
    pub levels: Option<LevelsPart>,
    pub result: Option<ResultPart>,
}

/// The scenario's levels (decision 4 of 0054): a margin of at least `minor`
/// is a minor victory, at least `major` a major one, at least `decisive` a
/// decisive one; the same margins against a side are defeats; a margin
/// between minus `minor` and `minor` is a stalemate.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct LevelsPart {
    pub minor: Dec,
    pub major: Dec,
    pub decisive: Dec,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ObjectiveLine {
    /// `ships_sunk`, `ships_crippled`, `ships_out_of_action`,
    /// `aircraft_destroyed`, `cargo_landed` or `base_open`.
    pub kind: &'static str,
    /// The base of `cargo_landed` and `base_open`.
    pub base: Option<Handle>,
    /// For `base_open`: true if the side wants the base shut.
    pub closed: Option<bool>,
    /// For the ship objectives: the points for each kind of ship (for
    /// `ships_out_of_action`, the points for a ship away for the whole
    /// horizon).
    pub ship_points: Vec<ShipKindPoints>,
    /// For `ships_out_of_action`: the days of the horizon its shares are of
    /// (decision 0088). Left out of the answer for every other kind, so that
    /// no earlier answer changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizon_days: Option<u32>,
    /// Points for one aircraft (`aircraft_destroyed`), one ton
    /// (`cargo_landed`) or the base being as wanted on every day
    /// (`base_open`, which earns the share of the days it was).
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
    /// Ships, aircraft, tons (unloaded less loaded back), or for a base of
    /// the side's own the days it ended as wanted.
    pub count: Dec,
    /// For `base_open` at a base of the side's own: the days counted, which
    /// `count` is out of.
    pub days: Option<u32>,
    pub points: Option<Dec>,
}

/// The umpire's result for both sides, told at the end of the game.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ResultPart {
    pub allied: SideResultPart,
    pub japanese: SideResultPart,
    /// What happened in the real battle, not in this game: the scenario's
    /// note of how the day went in history, in words and not a computed
    /// score. It is not this game's result.
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
    /// `minor`, `major` or `decisive`; `None` for a stalemate, which has no
    /// level (`outcome` says it in words).
    pub level: Option<&'static str>,
    /// `minor victory`, `stalemate` and so on.
    pub outcome: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ScoreLine {
    pub kind: &'static str,
    pub base: Option<Handle>,
    pub closed: Option<bool>,
    /// Ships, aircraft, tons (unloaded less loaded back), or for a base the
    /// days it ended as wanted.
    pub count: Dec,
    /// For `base_open`: the days counted, which `count` is out of.
    pub days: Option<u32>,
    /// For the ship objectives: the ships counted, by kind.
    pub ships: Vec<ShipsOfKind>,
    pub points: Dec,
    /// For `ships_out_of_action`: the days of the horizon the shares are of.
    /// Left out of the answer for every other kind.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizon_days: Option<u32>,
    /// For `ships_out_of_action`: each ship counted, in ship number order,
    /// with the days it was out and the points they earned. Left out of the
    /// answer when empty.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ships_out: Vec<ShipOutLine>,
}

/// One ship's line of a `ships_out_of_action` score (decision 0088).
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ShipOutLine {
    /// The side's own handle for a ship of its own, `ship:7`, with no name;
    /// for an enemy ship, which a side has no handle for (decision 0086),
    /// `enemy_ship:3`, the place of the ship in this list, from 1.
    pub ship: String,
    pub kind: &'static str,
    /// Her days of repair still to run at the end of the game.
    pub at_end: u32,
    /// The days of repair she left for a yard with.
    pub left_with: u32,
    /// `at_end` plus `left_with`.
    pub days_out: u32,
    /// `min(days_out, horizon) / horizon`.
    pub share: Dec,
    /// The kind's points times the share.
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

fn line(
    ctx: &Ctx<'_>,
    objective: &Objective,
    known: &KnownProgress,
    horizon_days: Option<u32>,
) -> ObjectiveLine {
    let mut line = ObjectiveLine {
        kind: objective.name(),
        base: objective.base().map(|b| ctx.base(b)),
        closed: None,
        ship_points: Vec::new(),
        horizon_days: None,
        points: None,
        known: KnownLine {
            basis: known.basis.name(),
            count: Dec(known.count),
            days: known.days,
            points: known.points.map(Dec),
        },
    };
    match objective {
        Objective::ShipsSunk(points) | Objective::ShipsCrippled(points) => {
            line.ship_points = ship_points(points);
        }
        Objective::ShipsOutOfAction(points) => {
            line.ship_points = ship_points(points);
            line.horizon_days = horizon_days;
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
            .map(|(objective, known)| line(ctx, objective, known, view.horizon_days()))
            .collect(),
        levels: view.levels().map(|l| LevelsPart {
            minor: Dec(l.minor),
            major: Dec(l.major),
            decisive: Dec(l.decisive),
        }),
        result: None,
    }
}

/// What a side calls a ship in the result: its own number for a ship of its
/// own, and for an enemy ship her place in the list of ships counted, which
/// tells it no more than the count does.
fn ship_label(ctx: &Ctx<'_>, ship: sim_core::ids::ShipId, place: usize) -> String {
    match ctx.ship_number(ship) {
        Some(number) => format!("ship:{number}"),
        None => format!("enemy_ship:{}", place + 1),
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
                days: o.days,
                ships: o
                    .by_kind
                    .iter()
                    .map(|(kind, ships)| ShipsOfKind {
                        kind: kind.name(),
                        ships: *ships,
                    })
                    .collect(),
                points: Dec(o.points),
                horizon_days: o.horizon_days,
                ships_out: o
                    .ships_out
                    .iter()
                    .enumerate()
                    .map(|(place, s)| ShipOutLine {
                        ship: ship_label(ctx, s.ship, place),
                        kind: s.kind.name(),
                        at_end: s.at_end,
                        left_with: s.left_with,
                        days_out: s.days_out,
                        share: Dec(s.share),
                        points: Dec(s.points),
                    })
                    .collect(),
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
