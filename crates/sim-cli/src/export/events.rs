//! The event log as JSON.

use serde::Serialize;
use sim_core::events::{Breakdown, Cause, Event, EventKind};
use sim_core::orders::RejectReason;

use super::data::{Data, Field};
use super::names;
use super::num::Num;

#[derive(Serialize)]
pub struct EventOut {
    id: u32,
    day: u32,
    pulse: Option<&'static str>,
    kind: &'static str,
    /// Exactly `Event`'s `Display`.
    text: String,
    causes: Vec<CauseOut>,
    entities: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    breakdown: Option<BreakdownOut>,
    data: Data,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum CauseOut {
    Order(u32),
    Event(u32),
}

#[derive(Serialize)]
struct BreakdownOut {
    factors: Vec<FactorOut>,
    result: Num,
}

#[derive(Serialize)]
struct FactorOut {
    name: &'static str,
    value: Num,
}

pub fn event(event: &Event) -> EventOut {
    let (kind, data) = describe(&event.kind);
    EventOut {
        id: event.id.raw(),
        day: event.day,
        pulse: event.pulse.map(|pulse| pulse.name()),
        kind,
        text: event.to_string(),
        causes: event
            .causes
            .iter()
            .map(|cause| match cause {
                Cause::Order(id) => CauseOut::Order(id.raw()),
                Cause::Event(id) => CauseOut::Event(id.raw()),
            })
            .collect(),
        entities: event
            .kind
            .entities()
            .iter()
            .map(ToString::to_string)
            .collect(),
        breakdown: event.breakdown.as_ref().map(breakdown),
        data,
    }
}

fn breakdown(breakdown: &Breakdown) -> BreakdownOut {
    BreakdownOut {
        factors: breakdown
            .factors
            .iter()
            .map(|factor| FactorOut {
                name: factor.name,
                value: Num(factor.value),
            })
            .collect(),
        result: Num(breakdown.result),
    }
}

/// Why an order was rejected: a stable code, the log's wording and the
/// reason's own figures.
fn reject_reason(reason: &RejectReason) -> Data {
    let base = |code: &'static str| Data::new().text("code", code);
    match reason {
        RejectReason::NotYourUnit => base("not_your_unit"),
        RejectReason::EmptyRoute => base("empty_route"),
        RejectReason::WaypointOffMap(hex) => base("waypoint_off_map").hex("hex", *hex),
        RejectReason::UnderWay => base("under_way"),
        RejectReason::NoOwnBaseHere => base("no_own_base_here"),
        RejectReason::InvalidBearing(bearing) => {
            base("invalid_bearing").with("bearing", Field::Int(i64::from(*bearing)))
        }
        RejectReason::InvalidSearchRange { range_nm, max_nm } => base("invalid_search_range")
            .num("range_nm", *range_nm)
            .num("max_nm", *max_nm),
        RejectReason::BreakOffOutOfRange(level) => {
            base("break_off_out_of_range").num("level", *level)
        }
        RejectReason::KeepShareOutOfRange(share) => {
            base("keep_share_out_of_range").num("share", *share)
        }
        RejectReason::NotYourBase => base("not_your_base"),
        RejectReason::NoShipsToDetach => base("no_ships_to_detach"),
        RejectReason::ShipNotInTaskForce(ship) => base("ship_not_in_task_force").id("ship", *ship),
        RejectReason::WouldLeaveTaskForceEmpty => base("would_leave_task_force_empty"),
        RejectReason::WaypointUnreachable(hex) => base("waypoint_unreachable").hex("hex", *hex),
        RejectReason::SpeedNotAboveZero(knots) => base("speed_not_above_zero").num("knots", *knots),
    }
}

/// The stable name of an event kind and every field it carries, under the
/// field names of `EventKind`. Ids are integers, hexes `[q, r]`, quantities
/// numbers, enums snake_case words. The match names every variant.
fn describe(kind: &EventKind) -> (&'static str, Data) {
    let d = Data::new();
    match kind {
        EventKind::OrderAccepted { order, target } => (
            "order_accepted",
            d.id("order", *order)
                .with("target", Field::Str(target.to_string())),
        ),
        EventKind::OrderRejected {
            order,
            target,
            reason,
        } => (
            "order_rejected",
            d.id("order", *order)
                .with("target", Field::Str(target.to_string()))
                .with("reason", Field::Obj(reject_reason(reason))),
        ),
        EventKind::TaskForceSteamed {
            task_force,
            from,
            to,
            distance_nm,
            speed_kn,
            hexes_entered,
        } => (
            "task_force_steamed",
            d.id("task_force", *task_force)
                .hex("from", *from)
                .hex("to", *to)
                .num("distance_nm", *distance_nm)
                .num("speed_kn", *speed_kn)
                .with("hexes_entered", Field::Hexes(hexes_entered.clone())),
        ),
        EventKind::FuelBurned {
            ship,
            task_force,
            burned_t,
            remaining_t,
        } => (
            "fuel_burned",
            d.id("ship", *ship)
                .id("task_force", *task_force)
                .num("burned_t", *burned_t)
                .num("remaining_t", *remaining_t),
        ),
        EventKind::TaskForceArrived {
            task_force,
            hex,
            elapsed_hours,
        } => (
            "task_force_arrived",
            d.id("task_force", *task_force)
                .hex("hex", *hex)
                .num("elapsed_hours", *elapsed_hours),
        ),
        EventKind::TaskForceOutOfFuel { task_force, hex } => (
            "task_force_out_of_fuel",
            d.id("task_force", *task_force).hex("hex", *hex),
        ),
        EventKind::CargoLoaded {
            ship,
            task_force,
            base,
            loaded_t,
            aboard_t,
            base_stock_t,
        } => (
            "cargo_loaded",
            d.id("ship", *ship)
                .id("task_force", *task_force)
                .id("base", *base)
                .num("loaded_t", *loaded_t)
                .num("aboard_t", *aboard_t)
                .num("base_stock_t", *base_stock_t),
        ),
        EventKind::CargoUnloaded {
            ship,
            task_force,
            base,
            unloaded_t,
            aboard_t,
            base_stock_t,
        } => (
            "cargo_unloaded",
            d.id("ship", *ship)
                .id("task_force", *task_force)
                .id("base", *base)
                .num("unloaded_t", *unloaded_t)
                .num("aboard_t", *aboard_t)
                .num("base_stock_t", *base_stock_t),
        ),
        EventKind::LoadingComplete {
            task_force,
            base,
            elapsed_hours,
        } => (
            "loading_complete",
            d.id("task_force", *task_force)
                .id("base", *base)
                .num("elapsed_hours", *elapsed_hours),
        ),
        EventKind::UnloadingComplete {
            task_force,
            base,
            elapsed_hours,
        } => (
            "unloading_complete",
            d.id("task_force", *task_force)
                .id("base", *base)
                .num("elapsed_hours", *elapsed_hours),
        ),
        EventKind::SearchFlown {
            air_group,
            base,
            from_deg,
            to_deg,
            range_nm,
            aircraft,
        } => (
            "search_flown",
            d.id("air_group", *air_group)
                .id("base", *base)
                .int("from_deg", *from_deg)
                .int("to_deg", *to_deg)
                .num("range_nm", *range_nm)
                .int("aircraft", *aircraft),
        ),
        EventKind::SearchAttempt {
            air_group,
            task_force,
            distance_nm,
            chance,
            sighted,
        } => (
            "search_attempt",
            d.id("air_group", *air_group)
                .id("task_force", *task_force)
                .num("distance_nm", *distance_nm)
                .num("chance", *chance)
                .flag("sighted", *sighted),
        ),
        EventKind::ReportReceived {
            side,
            contact,
            hex,
            ship_count,
            kind,
            confidence,
            created,
        } => (
            "report_received",
            d.text("side", names::side(*side))
                .id("contact", *contact)
                .hex("hex", *hex)
                .int("ship_count", *ship_count)
                .text("kind", kind.name())
                .num("confidence", *confidence)
                .flag("created", *created),
        ),
        EventKind::SightingNoticed {
            task_force,
            air_group,
            elapsed_hours,
        } => (
            "sighting_noticed",
            d.id("task_force", *task_force)
                .id("air_group", *air_group)
                .num("elapsed_hours", *elapsed_hours),
        ),
        EventKind::ContactAged {
            side,
            contact,
            age_pulses,
            confidence,
        } => (
            "contact_aged",
            d.text("side", names::side(*side))
                .id("contact", *contact)
                .int("age_pulses", *age_pulses)
                .num("confidence", *confidence),
        ),
        EventKind::ContactDropped {
            side,
            contact,
            age_pulses,
            confidence,
        } => (
            "contact_dropped",
            d.text("side", names::side(*side))
                .id("contact", *contact)
                .int("age_pulses", *age_pulses)
                .num("confidence", *confidence),
        ),
        EventKind::EngagementBegun {
            a,
            b,
            hex,
            elapsed_hours,
        } => (
            "engagement_begun",
            d.id("a", *a)
                .id("b", *b)
                .hex("hex", *hex)
                .num("elapsed_hours", *elapsed_hours),
        ),
        EventKind::SightingRoll {
            round,
            task_force,
            enemy,
            chance,
            sighted,
        } => (
            "sighting_roll",
            d.int("round", *round)
                .id("task_force", *task_force)
                .id("enemy", *enemy)
                .num("chance", *chance)
                .flag("sighted", *sighted),
        ),
        EventKind::ForceSurprised {
            round,
            task_force,
            enemy,
        } => (
            "force_surprised",
            d.int("round", *round)
                .id("task_force", *task_force)
                .id("enemy", *enemy),
        ),
        EventKind::Salvo {
            round,
            firer,
            target,
            weapon,
            shots,
            chance,
            expected,
            hits,
        } => (
            "salvo",
            d.int("round", *round)
                .id("firer", *firer)
                .id("target", *target)
                .text("weapon", names::weapon(*weapon))
                .num("shots", *shots)
                .num("chance", *chance)
                .num("expected", *expected)
                .int("hits", *hits),
        ),
        EventKind::ShipDamaged {
            round,
            ship,
            track,
            hits,
            points,
            value_after,
        } => (
            "ship_damaged",
            d.int("round", *round)
                .id("ship", *ship)
                .text("track", names::track(*track))
                .int("hits", *hits)
                .num("points", *points)
                .num("value_after", *value_after),
        ),
        EventKind::ShipSunk {
            round,
            ship,
            task_force,
        } => (
            "ship_sunk",
            d.int("round", *round)
                .id("ship", *ship)
                .id("task_force", *task_force),
        ),
        EventKind::ForceBecameAlert { round, task_force } => (
            "force_became_alert",
            d.int("round", *round).id("task_force", *task_force),
        ),
        EventKind::ForceBrokeOff {
            round,
            task_force,
            share,
        } => (
            "force_broke_off",
            d.int("round", *round)
                .id("task_force", *task_force)
                .num("share", *share),
        ),
        EventKind::ForceSlippedAway {
            round,
            task_force,
            enemy,
        } => (
            "force_slipped_away",
            d.int("round", *round)
                .id("task_force", *task_force)
                .id("enemy", *enemy),
        ),
        EventKind::ForcesPassedUnseen { round, a, b } => (
            "forces_passed_unseen",
            d.int("round", *round).id("a", *a).id("b", *b),
        ),
        EventKind::EngagementEnded {
            round,
            a,
            b,
            hex,
            reason,
        } => (
            "engagement_ended",
            d.int("round", *round)
                .id("a", *a)
                .id("b", *b)
                .hex("hex", *hex)
                .text("reason", names::end_reason(*reason)),
        ),
        EventKind::ClaimReceived {
            side,
            hex,
            claimed_sunk,
        } => (
            "claim_received",
            d.text("side", names::side(*side))
                .hex("hex", *hex)
                .int("claimed_sunk", *claimed_sunk),
        ),
        EventKind::SystemHit {
            round,
            ship,
            system,
            before,
            after,
        } => (
            "system_hit",
            d.int("round", *round)
                .id("ship", *ship)
                .text("system", names::system(*system))
                .text("before", names::system_state(*before))
                .text("after", names::system_state(*after)),
        ),
        EventKind::ShipsDetached {
            parent,
            new_force,
            ships,
            escorts,
            reason,
        } => (
            "ships_detached",
            d.id("parent", *parent)
                .id("new_force", *new_force)
                .ids("ships", ships)
                .ids("escorts", escorts)
                .text("reason", names::detach_reason(*reason)),
        ),
        EventKind::ForceRetired { task_force } => {
            ("force_retired", d.id("task_force", *task_force))
        }
        EventKind::ForceAlertedByContact {
            task_force,
            contact,
        } => (
            "force_alerted_by_contact",
            d.id("task_force", *task_force).id("contact", *contact),
        ),
        EventKind::ForceStoodDown { task_force } => {
            ("force_stood_down", d.id("task_force", *task_force))
        }
        EventKind::AlarmRoll {
            task_force,
            by_a,
            by_b,
            chance,
            alarmed,
        } => (
            "alarm_roll",
            d.id("task_force", *task_force)
                .id("by_a", *by_a)
                .id("by_b", *by_b)
                .num("chance", *chance)
                .flag("alarmed", *alarmed),
        ),
    }
}
