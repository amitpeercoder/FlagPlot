//! The OBJECTIVES part of the briefing and the result at the end of a game
//! (decision 0054).
//!
//! The part is made from a [`SideView`]: the side's own mission and
//! objectives, and its progress as far as it can know it (decision 7); and
//! what the enemy is scored for, his objectives as the scenario gives them
//! and nothing of his progress or state (decision 0183). Never the enemy's
//! mission. The result is handed in by whoever holds
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
    /// What the enemy is scored for (decision 0183): his objectives, in the
    /// scenario's order, each with no more than its kind, its base, open or
    /// shut and its worth. Never his mission, his progress or his state.
    /// Left out where the scenario has no objectives and at a seat at a
    /// command; empty where the enemy has none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemy_objectives: Option<Vec<EnemyObjectiveLine>>,
    /// True at a seat at a task force or a base, which is not given the
    /// side's mission and objectives: they are the area command's (decision
    /// 0138, 5). Left out otherwise.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub not_given_to_this_seat: bool,
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
    /// `aircraft_destroyed`, `cargo_landed`, `base_open`, `base_held`,
    /// `men_lost` or `men_landed`.
    pub kind: &'static str,
    /// The base of `cargo_landed`, `base_open`, `base_held` and
    /// `men_landed`; a base not on the side's map is given by its hex
    /// (decision 0186, 17).
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
    /// (`base_open`, which earns the share of the days it was); the base
    /// being the side's own at the end (`base_held`); each 100 enemy men
    /// lost (`men_lost`, decision 0090, 26); or each 100 men landed
    /// (`men_landed`, decision 0186).
    pub points: Option<Dec>,
    /// For `cargo_landed` only (left out for every other kind): the points
    /// for a thousand tons, which is `points` times 1,000. The text gives the
    /// worth by this, since the worth of one ton reads as nothing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_per_1000_t: Option<Dec>,
    /// What the side knows of its progress, from its own units and the events
    /// it may see.
    pub known: KnownLine,
    /// For `base_open`: whether today would count, read from the side's own
    /// facts now (decision 0110, item 4). Left out for every other kind.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_now: Option<OpenNow>,
}

/// One objective of the enemy's (decision 0183, amended 1): what it is and
/// what it is worth, and nothing else. It has no field for a count, days
/// counted, points earned, a basis or whether a base is open now, so that
/// nothing of the enemy's progress or state can be written in it. Its fields
/// are those of an [`ObjectiveLine`] of the same name and mean the same.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct EnemyObjectiveLine {
    /// As [`ObjectiveLine::kind`].
    pub kind: &'static str,
    /// The base of `cargo_landed`, `base_open`, `base_held` and `men_landed`,
    /// by the map's number and name; a base not on the reader's map is given
    /// by its hex (decision 0186, 17).
    pub base: Option<Handle>,
    /// For `base_open`: true if the enemy is scored for the base being shut.
    pub closed: Option<bool>,
    /// For the ship objectives: the points for each kind of ship.
    pub ship_points: Vec<ShipKindPoints>,
    /// For `ships_out_of_action`: the days of the horizon. Left out for every
    /// other kind.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizon_days: Option<u32>,
    /// Points for one aircraft, one ton, the base as wanted on every day, the
    /// base held at the end, each 100 men lost or each 100 men landed; `None`
    /// for the ship objectives.
    pub points: Option<Dec>,
    /// For `cargo_landed` only (left out for every other kind): the points
    /// for a thousand tons, which is `points` times 1,000. The text gives the
    /// worth by this, since the worth of one ton reads as nothing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_per_1000_t: Option<Dec>,
}

/// Whether a base counts as open at the end of today, from the side's own
/// facts: supply at the base above 0 and an aircraft of a group at it ready
/// (`sim_core::events::base_is_open`, the test the umpire counts the day by).
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct OpenNow {
    /// Whether the base is the side's own now. When it is not, nothing
    /// counts until it is.
    pub yours: bool,
    /// For a base of the side's own: the supply at the base now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply_t: Option<Dec>,
    /// For a base of the side's own: the aircraft ready at the groups at it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aircraft_ready: Option<u32>,
    /// For a base of the side's own: whether it is open now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open: Option<bool>,
    /// For a base of the side's own: whether today would count for the
    /// objective as it is worded (open, or with `closed`, shut).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts: Option<bool>,
    /// What is missing for it to be open: `supply`, `aircraft`, or both.
    /// Left out when it is open.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub missing: Vec<&'static str>,
    /// The days the side saw this base's day end while it was its own, and
    /// the days of them that it ended open. Left out when it saw none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_seen: Option<OpenDays>,
}

/// The days of a base's end the side saw (decision 0110, item 4).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
pub struct OpenDays {
    pub days: u32,
    pub open: u32,
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
    /// Ships, aircraft, tons (unloaded less loaded back), men landed, or for
    /// a base of the side's own the days it ended as wanted.
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
    /// Ships, aircraft, tons (unloaded less loaded back), men landed, or for
    /// a base the days it ended as wanted.
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
        base: objective.base().map(|b| objective_base(ctx, b)),
        closed: None,
        ship_points: Vec::new(),
        horizon_days: None,
        points: None,
        points_per_1000_t: None,
        known: KnownLine {
            basis: known.basis.name(),
            count: Dec(known.count),
            days: known.days,
            points: known.points.map(Dec),
        },
        open_now: None,
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
        Objective::CargoLanded { points_per_t, .. } => {
            line.points = Some(Dec(*points_per_t));
            line.points_per_1000_t = Some(Dec(*points_per_t * 1000));
        }
        Objective::BaseOpen {
            base,
            closed,
            points,
        } => {
            line.closed = Some(*closed);
            line.points = Some(Dec(*points));
            line.open_now = Some(open_now(ctx, *base, *closed));
        }
        Objective::BaseHeld { points, .. } => line.points = Some(Dec(*points)),
        Objective::MenLost { points_per_100_men }
        | Objective::MenLanded {
            points_per_100_men, ..
        } => line.points = Some(Dec(*points_per_100_men)),
    }
    line
}

/// The base an objective names, as the reading side writes it (decision
/// 0186, 17): by the map's number and name when the base is on its map, own
/// or enemy; by its hex, `(q,r)` with no name, when it is not and the hex is
/// known; otherwise as `Ctx::base` has it.
fn objective_base(ctx: &Ctx<'_>, base: sim_core::ids::BaseId) -> Handle {
    if ctx.bases.contains_key(&base) || ctx.enemy_bases.contains_key(&base) {
        return ctx.base(base);
    }
    match ctx.view.objective_base_hex(base) {
        Some(hex) => Handle {
            handle: hex.to_string(),
            name: None,
        },
        None => ctx.base(base),
    }
}

/// One objective of the enemy's, from the scenario's configuration alone
/// (decision 0183): nothing of the side's or the enemy's state is read.
fn enemy_line(
    ctx: &Ctx<'_>,
    objective: &Objective,
    horizon_days: Option<u32>,
) -> EnemyObjectiveLine {
    let mut line = EnemyObjectiveLine {
        kind: objective.name(),
        base: objective.base().map(|b| objective_base(ctx, b)),
        closed: None,
        ship_points: Vec::new(),
        horizon_days: None,
        points: None,
        points_per_1000_t: None,
    };
    match objective {
        Objective::ShipsSunk(points) | Objective::ShipsCrippled(points) => {
            line.ship_points = ship_points(points);
        }
        Objective::ShipsOutOfAction(points) => {
            line.ship_points = ship_points(points);
            line.horizon_days = horizon_days;
        }
        Objective::AircraftDestroyed { points } | Objective::BaseHeld { points, .. } => {
            line.points = Some(Dec(*points))
        }
        Objective::CargoLanded { points_per_t, .. } => {
            line.points = Some(Dec(*points_per_t));
            line.points_per_1000_t = Some(Dec(*points_per_t * 1000));
        }
        Objective::BaseOpen { closed, points, .. } => {
            line.closed = Some(*closed);
            line.points = Some(Dec(*points));
        }
        Objective::MenLost { points_per_100_men }
        | Objective::MenLanded {
            points_per_100_men, ..
        } => line.points = Some(Dec(*points_per_100_men)),
    }
    line
}

/// Whether the base would count as open today, from the side's own facts.
fn open_now(ctx: &Ctx<'_>, base_id: sim_core::ids::BaseId, closed: bool) -> OpenNow {
    use sim_core::events::{base_is_open, EventKind};
    use sim_core::side_events::SideEventKind;
    let Some(base) = ctx.bases.get(&base_id) else {
        return OpenNow {
            yours: false,
            supply_t: None,
            aircraft_ready: None,
            open: None,
            counts: None,
            missing: Vec::new(),
            days_seen: None,
        };
    };
    let ready: u32 = ctx
        .groups
        .values()
        .filter(|g| g.location == sim_core::state::AirGroupLocation::Base(base_id))
        .map(|g| g.ready)
        .sum();
    let open = base_is_open(base.supply_t, ready);
    let mut missing = Vec::new();
    if base.supply_t <= sim_core::fixed::Fixed::ZERO {
        missing.push("supply");
    }
    if ready == 0 {
        missing.push("aircraft");
    }
    let mut seen = OpenDays { days: 0, open: 0 };
    for event in ctx.view.events() {
        if let SideEventKind::Whole(EventKind::BaseAtDayEnd {
            base,
            supply_t,
            aircraft_ready,
        }) = &event.kind
        {
            if *base == base_id {
                seen.days += 1;
                if base_is_open(*supply_t, *aircraft_ready) {
                    seen.open += 1;
                }
            }
        }
    }
    OpenNow {
        yours: true,
        supply_t: Some(Dec(base.supply_t)),
        aircraft_ready: Some(ready),
        open: Some(open),
        counts: Some(open != closed),
        missing,
        days_seen: (seen.days > 0).then_some(seen),
    }
}

/// The part for one side's view: the side's own mission and objectives, and
/// the enemy's objectives without his mission (decision 0183).
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
        enemy_objectives: view.enemy_objectives().map(|list| {
            list.iter()
                .map(|objective| enemy_line(ctx, objective, view.horizon_days()))
                .collect()
        }),
        not_given_to_this_seat: view.seat().is_some(),
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
                base: o.base.map(|b| objective_base(ctx, b)),
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
