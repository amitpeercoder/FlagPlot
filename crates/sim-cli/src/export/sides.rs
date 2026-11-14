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

use serde::Serialize;
use sim_core::fixed::Fixed;
use sim_core::ids::UmpireKey;
use sim_core::side_events::{SideBreakdown, SideCause, SideEventKind};
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
    /// The side's own land units, known exactly (decision 0066, 21). Left
    /// out when the side has none.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    land_units: Vec<LandUnitOut>,
}

/// One of the side's own land units, known exactly (decision 0066, 21).
#[derive(Serialize)]
pub struct LandUnitOut {
    id: u32,
    name: String,
    men_ashore: u32,
    hex: Option<(i32, i32)>,
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
        events: events(world, side),
        land_units: world
            .side_view(side)
            .land_units()
            .map(|unit| LandUnitOut {
                id: unit.id.raw(),
                name: unit.name.clone(),
                men_ashore: unit.men_ashore,
                hex: unit.hex.map(|hex| (hex.q, hex.r)),
            })
            .collect(),
    }
}

/// A side's events in its own numbers. The file is the umpire's, so an order
/// cause names the order by its place in the order book, as the full log's
/// export does; the viewer's text says it the same way.
pub fn events(world: &World, side: Side) -> Vec<SideEventOut> {
    let key = world.umpire_key();
    let view = world.side_view(side);
    let events = view.events();
    events
        .iter()
        .map(|event| {
            let n = event.number;
            let mut causes = Vec::new();
            for cause in &event.causes {
                match cause {
                    SideCause::Order(own) => causes.push(CauseOut::Order(
                        view.order_by_number(*own)
                            .expect("a cause the side sees is one of its own orders")
                            .id
                            .umpire_raw(&key),
                    )),
                    SideCause::Event(earlier) => causes.push(CauseOut::Event(*earlier)),
                }
            }
            let hidden_causes = event.hidden_causes;
            let (kind, data, cut) = describe_side(&event.kind, &key);
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
                log_id: event.id.umpire_raw(&key),
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
fn describe_side(kind: &SideEventKind, key: &UmpireKey) -> (&'static str, Data, bool) {
    let d = Data::new();
    match kind {
        SideEventKind::Whole(kind) => {
            let (code, data) = describe(kind, key);
            (code, data, false)
        }
        SideEventKind::Salvo {
            round,
            firer,
            weapon,
            shots,
        } => (
            "salvo",
            d.int("round", *round)
                .id("firer", *firer)
                .text("weapon", names::weapon(*weapon))
                .num("shots", *shots),
            true,
        ),
        SideEventKind::SalvoAgainst {
            round,
            target,
            weapon,
        } => (
            "salvo",
            d.int("round", *round)
                .id("target", *target)
                .text("weapon", names::weapon(*weapon)),
            true,
        ),
        SideEventKind::AirAttackAgainst {
            task_force,
            ship,
            shots,
        } => (
            "air_attack",
            d.id("task_force", *task_force)
                .id("ship", *ship)
                .num("shots", *shots),
            true,
        ),
        SideEventKind::AirAttack {
            air_group,
            weapon,
            shots,
        } => (
            "air_attack",
            d.id("air_group", *air_group)
                .text("weapon", names::weapon(*weapon))
                .num("shots", *shots),
            true,
        ),
        SideEventKind::AirFight {
            firer,
            firing,
            at,
            shots,
            after_attack,
        } => (
            "air_fight",
            d.id("firer", *firer)
                .text("firing", firing.name())
                .text("at", at.name())
                .num("shots", *shots)
                .flag_when_true("after_attack", *after_attack),
            true,
        ),
        SideEventKind::AntiAircraftFire {
            task_force,
            shots,
            after_attack,
        } => (
            "anti_aircraft_fire",
            d.id("task_force", *task_force)
                .num("shots", *shots)
                .flag_when_true("after_attack", *after_attack),
            true,
        ),
        SideEventKind::BaseAntiAircraftFire {
            base,
            shots,
            after_attack,
        } => (
            "base_anti_aircraft_fire",
            d.id("base", *base)
                .num("shots", *shots)
                .flag_when_true("after_attack", *after_attack),
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
        SideEventKind::BaseAttackedAgainst {
            base,
            shots,
            chance,
            expected,
            hits,
            hits_on_aircraft,
            hits_on_supply,
            hits_on_men,
        } => (
            "base_attacked",
            d.id("base", *base)
                .num("shots", *shots)
                .num("chance", *chance)
                .num("expected", *expected)
                .int("hits", *hits)
                .int("hits_on_aircraft", *hits_on_aircraft)
                .int("hits_on_supply", *hits_on_supply)
                .int_when_nonzero("hits_on_men", *hits_on_men),
            true,
        ),
        SideEventKind::SearchAttempt { air_group } => {
            ("search_attempt", d.id("air_group", *air_group), true)
        }
        SideEventKind::EngagementBegun {
            task_force,
            hex,
            elapsed_hours,
        } => (
            "engagement_begun",
            d.id("task_force", *task_force)
                .hex("hex", *hex)
                .num("elapsed_hours", *elapsed_hours),
            true,
        ),
        SideEventKind::EngagementEnded {
            round,
            task_force,
            hex,
            reason,
        } => (
            "engagement_ended",
            d.int("round", *round)
                .id("task_force", *task_force)
                .hex("hex", *hex)
                .text("reason", names::side_end_reason(*reason)),
            true,
        ),
        SideEventKind::SightingRoll {
            round,
            task_force,
            chance,
            fatigue_factor,
        } => {
            let d = d
                .int("round", *round)
                .id("task_force", *task_force)
                .num("chance", *chance);
            // Left out where fatigue is not in play (decision 0037).
            let d = if *fatigue_factor == Fixed::ONE {
                d
            } else {
                d.num("fatigue_factor", *fatigue_factor)
            };
            ("sighting_roll", d, true)
        }
        SideEventKind::ForceSurprised { round, task_force } => (
            "force_surprised",
            d.int("round", *round).id("task_force", *task_force),
            true,
        ),
        SideEventKind::ForceSlippedAway { round, task_force } => (
            "force_slipped_away",
            d.int("round", *round).id("task_force", *task_force),
            true,
        ),
        SideEventKind::AlarmRoll { task_force, chance } => (
            "alarm_roll",
            d.id("task_force", *task_force).num("chance", *chance),
            true,
        ),
        SideEventKind::GroundFight { hex } => ("ground_fight", d.hex("hex", *hex), true),
        SideEventKind::GroundFire {
            unit,
            hex,
            firing_men,
            shots,
        } => (
            "ground_fire",
            d.id("unit", *unit)
                .hex("hex", *hex)
                .int("firing_men", *firing_men)
                .num("shots", *shots),
            true,
        ),
        SideEventKind::BombardmentFire {
            ship,
            base,
            shells,
            weighed_shots,
            rounds_left,
        } => (
            "bombardment_fire",
            d.id("ship", *ship)
                .id("base", *base)
                .num("shells", *shells)
                .num("weighed_shots", *weighed_shots)
                .num("rounds_left", *rounds_left),
            true,
        ),
        SideEventKind::ShoreBombardment {
            task_force,
            base,
            hex,
            shells,
            weighed_shots,
        } => (
            "shore_bombardment",
            d.id("task_force", *task_force)
                .id("base", *base)
                .hex("hex", *hex)
                .num("shells", *shells)
                .num("weighed_shots", *weighed_shots),
            true,
        ),
        SideEventKind::ShoreBombardmentAgainst {
            base,
            hex,
            shells,
            weighed_shots,
            hits,
            hits_on_aircraft,
            hits_on_supply,
            hits_on_men,
        } => (
            "shore_bombardment",
            d.id("base", *base)
                .hex("hex", *hex)
                .num("shells", *shells)
                .num("weighed_shots", *weighed_shots)
                .int("hits", *hits)
                .int("hits_on_aircraft", *hits_on_aircraft)
                .int("hits_on_supply", *hits_on_supply)
                .int("hits_on_men", *hits_on_men),
            true,
        ),
        SideEventKind::BaseChangedSide {
            base,
            hex,
            from,
            to,
        } => (
            "base_changed_side",
            d.id("base", *base)
                .hex("hex", *hex)
                .text("from", names::side(*from))
                .text("to", names::side(*to)),
            true,
        ),
    }
}
