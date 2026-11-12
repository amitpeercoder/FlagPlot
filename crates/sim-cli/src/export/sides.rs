//! What each side may see of the log, as the engine gives it (decision 0047).
//!
//! Nothing here decides who may see what. Each side's events are exactly
//! `SideView::events()`: the engine's own rule, one exhaustive `match` in
//! `side_events.rs`. This module only writes them down, so that the viewer
//! can show a side's view without the truth.
//!
//! A side's events are numbered 1, 2, 3 in the order the engine gives them
//! (decision 0048, 14): `n` is what a side is shown, and the numbers of its
//! causes are renumbered the same way. `log_id` is the event's number in the
//! full log, for the umpire's use and for the checks; the viewer never shows
//! it in a side's view.
//!
//! The wording is the engine's: `text` is the `Display` of the side's event
//! kind, as `explain` words it for a player.

use std::collections::BTreeMap;

use serde::Serialize;
use sim_core::events::Cause;
use sim_core::side_events::{SideBreakdown, SideEvent, SideEventKind};
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_core::World;

use super::data::Data;
use super::events::{describe, CauseOut};
use super::names;
use super::num::Num;

#[derive(Serialize)]
pub struct SidesOut {
    allied: SideOut,
    japanese: SideOut,
}

#[derive(Serialize)]
pub struct SideOut {
    events: Vec<SideEventOut>,
}

#[derive(Serialize)]
pub struct SideEventOut {
    /// The number the side is shown: 1, 2, 3 in its own order.
    n: u32,
    /// The event's number in the full log.
    log_id: u32,
    day: u32,
    pulse: Option<&'static str>,
    kind: &'static str,
    /// True when the engine cut the event down: fields the side may not know
    /// are removed from `text`, `data` and `breakdown`.
    cut: bool,
    /// `#n D<day> <pulse> <the engine's wording>  <- <causes>`, all in the
    /// side's numbers.
    text: String,
    /// The causes the side may see, in its numbers.
    causes: Vec<CauseOut>,
    /// True if the event has causes the side may not see.
    hidden_causes: bool,
    entities: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    breakdown: Option<BreakdownOut>,
    data: Data,
}

#[derive(Serialize)]
struct BreakdownOut {
    factors: Vec<FactorOut>,
    /// Absent when the engine cut it: the result is the quantity that was cut.
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Num>,
}

#[derive(Serialize)]
struct FactorOut {
    name: &'static str,
    value: Num,
}

pub fn sides(world: &World) -> SidesOut {
    SidesOut {
        allied: side(world, Side::Allied),
        japanese: side(world, Side::Japanese),
    }
}

fn side(world: &World, side: Side) -> SideOut {
    SideOut {
        events: events(&world.side_view(side).events()),
    }
}

/// A side's events in its own numbers.
pub fn events(events: &[SideEvent]) -> Vec<SideEventOut> {
    let number: BTreeMap<u32, u32> = events
        .iter()
        .enumerate()
        .map(|(i, e)| {
            (
                e.id.raw(),
                u32::try_from(i + 1).expect("a side has few events"),
            )
        })
        .collect();
    events
        .iter()
        .map(|event| {
            let n = number[&event.id.raw()];
            let mut causes = Vec::new();
            let mut hidden_causes = event.hidden_causes;
            for cause in &event.causes {
                match cause {
                    Cause::Order(id) => causes.push(CauseOut::Order(id.raw())),
                    // A cause the engine lets the side see is an event it sees.
                    // If one were not, say so and drop it: never show the log's number.
                    Cause::Event(id) => match number.get(&id.raw()) {
                        Some(earlier) => causes.push(CauseOut::Event(*earlier)),
                        None => hidden_causes = true,
                    },
                }
            }
            let (kind, data, cut) = describe_side(&event.kind);
            let pulse = event.pulse.map(Pulse::name);
            let cause_text: Vec<String> = causes
                .iter()
                .map(|c| match c {
                    CauseOut::Order(id) => format!("order:{id}"),
                    CauseOut::Event(id) => format!("#{id}"),
                })
                .collect();
            let mut text = format!(
                "#{n} D{} {:<9} {}",
                event.day,
                pulse.unwrap_or("-"),
                event.kind
            );
            if !cause_text.is_empty() {
                text.push_str("  <- ");
                text.push_str(&cause_text.join(", "));
            }
            SideEventOut {
                n,
                log_id: event.id.raw(),
                day: event.day,
                pulse,
                kind,
                cut,
                text,
                causes,
                hidden_causes,
                entities: event
                    .kind
                    .entities()
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
                breakdown: event.breakdown.as_ref().map(breakdown),
                data,
            }
        })
        .collect()
}

fn breakdown(breakdown: &SideBreakdown) -> BreakdownOut {
    BreakdownOut {
        factors: breakdown
            .factors
            .iter()
            .map(|factor| FactorOut {
                name: factor.name,
                value: Num(factor.value),
            })
            .collect(),
        result: breakdown.result.map(Num),
    }
}

/// The kind's code, its data, and whether it was cut. A whole event is
/// described as the full log describes it; a cut one has only the fields the
/// engine left it.
fn describe_side(kind: &SideEventKind) -> (&'static str, Data, bool) {
    let d = Data::new();
    match kind {
        SideEventKind::Whole(kind) => {
            let (code, data) = describe(kind);
            (code, data, false)
        }
        SideEventKind::Salvo {
            round,
            firer,
            target,
            weapon,
            shots,
        } => (
            "salvo",
            d.int("round", *round)
                .id("firer", *firer)
                .id("target", *target)
                .text("weapon", names::weapon(*weapon))
                .num("shots", *shots),
            true,
        ),
        SideEventKind::SalvoAgainst {
            round,
            firer,
            target,
            weapon,
        } => (
            "salvo",
            d.int("round", *round)
                .id("firer", *firer)
                .id("target", *target)
                .text("weapon", names::weapon(*weapon)),
            true,
        ),
        SideEventKind::AirAttackAgainst {
            air_group,
            task_force,
            ship,
            shots,
        } => (
            "air_attack",
            d.id("air_group", *air_group)
                .id("task_force", *task_force)
                .id("ship", *ship)
                .num("shots", *shots),
            true,
        ),
        SideEventKind::AirAttack {
            air_group,
            task_force,
            ship,
            weapon,
            shots,
        } => (
            "air_attack",
            d.id("air_group", *air_group)
                .id("task_force", *task_force)
                .id("ship", *ship)
                .text("weapon", names::weapon(*weapon))
                .num("shots", *shots),
            true,
        ),
        SideEventKind::AirFight {
            firer,
            target,
            firing,
            at,
            shots,
        } => (
            "air_fight",
            d.id("firer", *firer)
                .id("target", *target)
                .text("firing", firing.name())
                .text("at", at.name())
                .num("shots", *shots),
            true,
        ),
        SideEventKind::AntiAircraftFire {
            task_force,
            air_group,
            shots,
        } => (
            "anti_aircraft_fire",
            d.id("task_force", *task_force)
                .id("air_group", *air_group)
                .num("shots", *shots),
            true,
        ),
        SideEventKind::BaseAntiAircraftFire {
            base,
            air_group,
            shots,
        } => (
            "base_anti_aircraft_fire",
            d.id("base", *base)
                .id("air_group", *air_group)
                .num("shots", *shots),
            true,
        ),
        SideEventKind::BaseAttacked {
            air_group,
            base,
            shots,
        } => (
            "base_attacked",
            d.id("air_group", *air_group)
                .id("base", *base)
                .num("shots", *shots),
            true,
        ),
        SideEventKind::SearchAttempt {
            air_group,
            task_force,
        } => (
            "search_attempt",
            d.id("air_group", *air_group).id("task_force", *task_force),
            true,
        ),
    }
}
