//! The event log as JSON.

use serde::Serialize;
use sim_core::events::{Breakdown, Cause, Event, EventKind, FatigueChange};
use sim_core::orders::RejectReason;
use sim_core::state::AirGroupLocation;

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
        RejectReason::NoAirCombatFigures => base("no_air_combat_figures"),
        RejectReason::AircraftAttackWithNothing => base("aircraft_attack_with_nothing"),
        RejectReason::NotAFighter => base("not_a_fighter"),
        RejectReason::PatrolShareOutOfRange(share) => {
            base("patrol_share_out_of_range").num("share", *share)
        }
        RejectReason::NotAnEnemyBase => base("not_an_enemy_base"),
        RejectReason::FatigueOutOfRange(fatigue) => {
            base("fatigue_out_of_range").num("fatigue", *fatigue)
        }
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
            from,
            hex,
            from_deg,
            to_deg,
            range_nm,
            aircraft,
        } => (
            "search_flown",
            match from {
                AirGroupLocation::Base(base) => d.id("air_group", *air_group).id("base", *base),
                AirGroupLocation::Ship(ship) => d.id("air_group", *air_group).id("ship", *ship),
            }
            .hex("hex", *hex)
            .int("from_deg", *from_deg)
            .int("to_deg", *to_deg)
            .num("range_nm", *range_nm)
            .int("aircraft", *aircraft),
        ),
        EventKind::DamagedAircraftDestroyedOnGround {
            air_group,
            base,
            aircraft,
            remaining,
        } => (
            "damaged_aircraft_destroyed_on_ground",
            d.id("air_group", *air_group)
                .id("base", *base)
                .int("aircraft", *aircraft)
                .int("remaining", *remaining),
        ),
        EventKind::AircraftWorn {
            air_group,
            sortie,
            distance_nm,
            aircraft,
            chance,
            expected,
            worn,
            written_off,
            ready,
            damaged,
        } => (
            "aircraft_worn",
            d.id("air_group", *air_group)
                .text("sortie", sortie.name())
                .num("distance_nm", *distance_nm)
                .int("aircraft", *aircraft)
                .num("chance", *chance)
                .num("expected", *expected)
                .int("worn", *worn)
                .int("written_off", *written_off)
                .int("ready", *ready)
                .int("damaged", *damaged),
        ),
        EventKind::AircraftRepaired {
            air_group,
            at,
            repaired,
            ready,
            damaged,
        } => (
            "aircraft_repaired",
            match at {
                AirGroupLocation::Base(base) => d.id("air_group", *air_group).id("base", *base),
                AirGroupLocation::Ship(ship) => d.id("air_group", *air_group).id("ship", *ship),
            }
            .int("repaired", *repaired)
            .int("ready", *ready)
            .int("damaged", *damaged),
        ),
        EventKind::AirGroupFatigue {
            air_group,
            change,
            amount,
            fatigue,
        } => (
            "air_group_fatigue",
            match change {
                FatigueChange::Gained {
                    sortie,
                    distance_nm,
                } => d
                    .id("air_group", *air_group)
                    .text("change", "gained")
                    .text("sortie", sortie.name())
                    .num("distance_nm", *distance_nm),
                FatigueChange::Rested => d.id("air_group", *air_group).text("change", "rested"),
            }
            .num("amount", *amount)
            .num("fatigue", *fatigue),
        ),
        EventKind::AirGroupStoodDown {
            air_group,
            fatigue,
            threshold,
        } => (
            "air_group_stood_down",
            d.id("air_group", *air_group)
                .num("fatigue", *fatigue)
                .num("threshold", *threshold),
        ),
        EventKind::AirGroupReturned {
            air_group,
            fatigue,
            threshold,
        } => (
            "air_group_returned",
            d.id("air_group", *air_group)
                .num("fatigue", *fatigue)
                .num("threshold", *threshold),
        ),
        EventKind::AirGroupLost {
            air_group,
            ship,
            aircraft,
        } => (
            "air_group_lost",
            d.id("air_group", *air_group)
                .id("ship", *ship)
                .int("aircraft", *aircraft),
        ),
        EventKind::StrikeLaunched {
            air_group,
            contact,
            hex,
            aircraft,
            distance_nm,
        } => (
            "strike_launched",
            d.id("air_group", *air_group)
                .id("contact", *contact)
                .hex("hex", *hex)
                .int("aircraft", *aircraft)
                .num("distance_nm", *distance_nm),
        ),
        EventKind::BaseStrikeLaunched {
            air_group,
            base,
            hex,
            aircraft,
            distance_nm,
        } => (
            "base_strike_launched",
            d.id("air_group", *air_group)
                .id("base", *base)
                .hex("hex", *hex)
                .int("aircraft", *aircraft)
                .num("distance_nm", *distance_nm),
        ),
        EventKind::BaseAntiAircraftFire {
            base,
            air_group,
            shots,
            chance,
            expected,
            hits,
        } => (
            "base_anti_aircraft_fire",
            d.id("base", *base)
                .id("air_group", *air_group)
                .num("shots", *shots)
                .num("chance", *chance)
                .num("expected", *expected)
                .int("hits", *hits),
        ),
        EventKind::BaseAttacked {
            air_group,
            base,
            shots,
            chance,
            expected,
            hits,
            hits_on_aircraft,
            hits_on_supply,
        } => (
            "base_attacked",
            d.id("air_group", *air_group)
                .id("base", *base)
                .num("shots", *shots)
                .num("chance", *chance)
                .num("expected", *expected)
                .int("hits", *hits)
                .int("hits_on_aircraft", *hits_on_aircraft)
                .int("hits_on_supply", *hits_on_supply),
        ),
        EventKind::AircraftDestroyedOnGround {
            air_group,
            base,
            aircraft,
            remaining,
        } => (
            "aircraft_destroyed_on_ground",
            d.id("air_group", *air_group)
                .id("base", *base)
                .int("aircraft", *aircraft)
                .int("remaining", *remaining),
        ),
        EventKind::BaseSupplyDestroyed {
            base,
            destroyed_t,
            supply_t,
        } => (
            "base_supply_destroyed",
            d.id("base", *base)
                .num("destroyed_t", *destroyed_t)
                .num("supply_t", *supply_t),
        ),
        EventKind::StrikeFoundNothing { air_group, hex } => (
            "strike_found_nothing",
            d.id("air_group", *air_group).hex("hex", *hex),
        ),
        EventKind::ContactNotThere { side, contact, hex } => (
            "contact_not_there",
            d.text("side", names::side(*side))
                .id("contact", *contact)
                .hex("hex", *hex),
        ),
        EventKind::AirAttack {
            air_group,
            task_force,
            ship,
            weapon,
            shots,
            chance,
            expected,
            hits,
        } => (
            "air_attack",
            d.id("air_group", *air_group)
                .id("task_force", *task_force)
                .id("ship", *ship)
                .text("weapon", names::weapon(*weapon))
                .num("shots", *shots)
                .num("chance", *chance)
                .num("expected", *expected)
                .int("hits", *hits),
        ),
        EventKind::EscortsSent {
            air_group,
            with_air_group,
            aircraft,
        } => (
            "escorts_sent",
            d.id("air_group", *air_group)
                .id("with_air_group", *with_air_group)
                .int("aircraft", *aircraft),
        ),
        EventKind::AirFight {
            firer,
            target,
            firing,
            at,
            shots,
            chance,
            expected,
            hits,
        } => (
            "air_fight",
            d.id("firer", *firer)
                .id("target", *target)
                .text("firing", firing.name())
                .text("at", at.name())
                .num("shots", *shots)
                .num("chance", *chance)
                .num("expected", *expected)
                .int("hits", *hits),
        ),
        EventKind::AntiAircraftFire {
            task_force,
            air_group,
            shots,
            chance,
            expected,
            hits,
        } => (
            "anti_aircraft_fire",
            d.id("task_force", *task_force)
                .id("air_group", *air_group)
                .num("shots", *shots)
                .num("chance", *chance)
                .num("expected", *expected)
                .int("hits", *hits),
        ),
        EventKind::AircraftLost {
            air_group,
            aircraft,
            remaining,
        } => (
            "aircraft_lost",
            d.id("air_group", *air_group)
                .int("aircraft", *aircraft)
                .int("remaining", *remaining),
        ),
        EventKind::PilotsLost {
            air_group,
            lost,
            left,
        } => (
            "pilots_lost",
            d.id("air_group", *air_group)
                .int("lost", *lost)
                .int("left", *left),
        ),
        EventKind::ReplacementsReceived {
            air_group,
            base,
            aircraft,
            pilots,
            ready,
            pilots_now,
        } => (
            "replacements_received",
            d.id("air_group", *air_group)
                .id("base", *base)
                .int("aircraft", *aircraft)
                .int("pilots", *pilots)
                .int("ready", *ready)
                .int("pilots_now", *pilots_now),
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
            created,
        } => (
            "report_received",
            d.text("side", names::side(*side))
                .id("contact", *contact)
                .hex("hex", *hex)
                .int("ship_count", *ship_count)
                .text("kind", kind.name())
                .flag("created", *created),
        ),
        EventKind::ContactAged {
            side,
            contact,
            age_pulses,
        } => (
            "contact_aged",
            d.text("side", names::side(*side))
                .id("contact", *contact)
                .int("age_pulses", *age_pulses),
        ),
        EventKind::ContactDropped {
            side,
            contact,
            age_pulses,
        } => (
            "contact_dropped",
            d.text("side", names::side(*side))
                .id("contact", *contact)
                .int("age_pulses", *age_pulses),
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
