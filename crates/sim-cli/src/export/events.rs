//! The event log as JSON.

use serde::Serialize;
use sim_core::events::{
    Breakdown, Cause, Event, EventKind, FatigueChange, ForceFatigueChange, LandFatigueChange,
    SupplyUse,
};
use sim_core::fixed::Fixed;
use sim_core::ids::UmpireKey;
use sim_core::orders::{Cargo, RejectReason};
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
pub(super) enum CauseOut {
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

pub fn event(event: &Event, key: &UmpireKey) -> EventOut {
    let (kind, data) = describe(&event.kind, key);
    EventOut {
        id: event.id.umpire_raw(key),
        day: event.day,
        pulse: event.pulse.map(|pulse| pulse.name()),
        kind,
        text: event.to_string(),
        causes: event
            .causes
            .iter()
            .map(|cause| match cause {
                Cause::Order(id) => CauseOut::Order(id.umpire_raw(key)),
                Cause::Event(id) => CauseOut::Event(id.umpire_raw(key)),
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
        RejectReason::NotACoastHex(hex) => base("not_a_coast_hex").hex("hex", *hex),
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
        RejectReason::NoShipAfloat => base("no_ship_afloat"),
        RejectReason::AirGroupLost => base("air_group_lost"),
        RejectReason::LandUnitDestroyed => base("land_unit_destroyed"),
        RejectReason::NoMenAshore => base("no_men_ashore"),
        RejectReason::AirGroupLostWithBase => base("air_group_lost_with_base"),
        RejectReason::TargetListEmpty => base("target_list_empty"),
        RejectReason::OutOfFuel => base("out_of_fuel"),
        RejectReason::AwayAtYard => base("away_at_yard"),
        RejectReason::NotAYardPort => base("not_a_yard_port"),
        RejectReason::TargetKindRepeated(kind) => {
            base("target_kind_repeated").text("kind", kind.name())
        }
        RejectReason::NotYourLandUnit => base("not_your_land_unit"),
        RejectReason::NoMenAshoreHere => base("no_men_ashore_here"),
        RejectReason::NoTroopCapacity => base("no_troop_capacity"),
        RejectReason::AirGroupAboardShip => base("air_group_aboard_ship"),
        RejectReason::AirGroupAwayAtYard => base("air_group_away_at_yard"),
        RejectReason::AlreadyAtBase => base("already_at_base"),
        RejectReason::BeyondTransferRange {
            distance_nm,
            range_nm,
        } => base("beyond_transfer_range")
            .num("distance_nm", *distance_nm)
            .num("range_nm", *range_nm),
        RejectReason::AlreadyMovedToday => base("already_moved_today"),
        RejectReason::NoLandLink(hex) => base("no_land_link").hex("hex", *hex),
        RejectReason::MarkNotOnRoute(hex) => base("mark_not_on_route").hex("hex", *hex),
    }
}

/// The stable name of an event kind and every field it carries, under the
/// field names of `EventKind`. Ids are integers, hexes `[q, r]`, quantities
/// numbers, enums snake_case words. The match names every variant.
/// Names the cargo in an event's data, for fuel only: the data of a supply
/// event is what it was before fuel became a cargo (decision 0046).
fn cargo_named(data: Data, cargo: Cargo) -> Data {
    match cargo {
        Cargo::Supply => data,
        Cargo::Fuel => data.text("cargo", cargo.name()),
    }
}

pub(super) fn describe(kind: &EventKind, key: &UmpireKey) -> (&'static str, Data) {
    let d = Data::new();
    match kind {
        EventKind::OrderAccepted { order, target } => (
            "order_accepted",
            d.int("order", order.umpire_raw(key))
                .with("target", Field::Str(target.to_string())),
        ),
        EventKind::OrderRejected {
            order,
            target,
            reason,
        } => (
            "order_rejected",
            d.int("order", order.umpire_raw(key))
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
        EventKind::ShipRefuelled {
            ship,
            task_force,
            base,
            taken_t,
            fuel_t,
            base_fuel_t,
            force_restored,
        } => (
            "ship_refuelled",
            d.id("ship", *ship)
                .id("task_force", *task_force)
                .id("base", *base)
                .num("taken_t", *taken_t)
                .num("fuel_t", *fuel_t)
                .num("base_fuel_t", *base_fuel_t)
                .flag("force_restored", *force_restored),
        ),
        EventKind::CargoLoaded {
            ship,
            task_force,
            base,
            cargo,
            loaded_t,
            aboard_t,
            base_stock_t,
        } => (
            "cargo_loaded",
            cargo_named(
                d.id("ship", *ship)
                    .id("task_force", *task_force)
                    .id("base", *base),
                *cargo,
            )
            .num("loaded_t", *loaded_t)
            .num("aboard_t", *aboard_t)
            .num("base_stock_t", *base_stock_t),
        ),
        EventKind::CargoUnloaded {
            ship,
            task_force,
            base,
            cargo,
            unloaded_t,
            aboard_t,
            base_stock_t,
        } => (
            "cargo_unloaded",
            cargo_named(
                d.id("ship", *ship)
                    .id("task_force", *task_force)
                    .id("base", *base),
                *cargo,
            )
            .num("unloaded_t", *unloaded_t)
            .num("aboard_t", *aboard_t)
            .num("base_stock_t", *base_stock_t),
        ),
        EventKind::LoadingComplete {
            task_force,
            base,
            cargo,
            elapsed_hours,
        } => (
            "loading_complete",
            cargo_named(d.id("task_force", *task_force).id("base", *base), *cargo)
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
                // Lost with its base: no place (decision 0090, 20).
                AirGroupLocation::Lost => d.id("air_group", *air_group),
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
        EventKind::AircraftDitched {
            air_group,
            sortie,
            distance_nm,
            radius_nm,
            aircraft,
            chance,
            expected,
            ditched,
            crews_recovered,
            pilots_lost,
            ready,
        } => {
            let data = d
                .id("air_group", *air_group)
                .text("sortie", sortie.name())
                .num("distance_nm", *distance_nm)
                .num("radius_nm", *radius_nm)
                .int("aircraft", *aircraft)
                .num("chance", *chance)
                .num("expected", *expected)
                .int("ditched", *ditched);
            // Left out where the group's pilots are not counted.
            let data = match crews_recovered {
                Some(recovered) => data.int("crews_recovered", *recovered),
                None => data,
            };
            (
                "aircraft_ditched",
                data.int("pilots_lost", *pilots_lost).int("ready", *ready),
            )
        }
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
        EventKind::AircraftLandedElsewhere {
            from,
            to,
            at,
            aircraft,
            pilots,
            formed,
        } => (
            "aircraft_landed_elsewhere",
            match at {
                AirGroupLocation::Base(base) => d.id("from", *from).id("to", *to).id("base", *base),
                AirGroupLocation::Ship(ship) => d.id("from", *from).id("to", *to).id("ship", *ship),
                AirGroupLocation::Lost => d.id("from", *from).id("to", *to),
            }
            .int("aircraft", *aircraft)
            .int("pilots", *pilots)
            .flag("formed", *formed),
        ),
        EventKind::AircraftDitchedNoDeck {
            air_group,
            hex,
            aircraft,
            beside_own_force,
            chance,
            crews_recovered,
            pilots_lost,
            ready,
        } => {
            let data = d
                .id("air_group", *air_group)
                .hex("hex", *hex)
                .int("aircraft", *aircraft)
                .flag("beside_own_force", *beside_own_force)
                .num("chance", *chance);
            // Left out where the group's pilots are not counted.
            let data = match crews_recovered {
                Some(recovered) => data.int("crews_recovered", *recovered),
                None => data,
            };
            (
                "aircraft_ditched_no_deck",
                data.int("pilots_lost", *pilots_lost).int("ready", *ready),
            )
        }
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
                // Lost with its base: no place (decision 0090, 20).
                AirGroupLocation::Lost => d.id("air_group", *air_group),
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
                .int("contact", contact.umpire_raw(key))
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
            after_attack,
        } => (
            "base_anti_aircraft_fire",
            d.id("base", *base)
                .id("air_group", *air_group)
                .num("shots", *shots)
                .num("chance", *chance)
                .num("expected", *expected)
                .int("hits", *hits)
                .flag_when_true("after_attack", *after_attack),
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
            hits_on_men,
        } => (
            "base_attacked",
            d.id("air_group", *air_group)
                .id("base", *base)
                .num("shots", *shots)
                .num("chance", *chance)
                .num("expected", *expected)
                .int("hits", *hits)
                .int("hits_on_aircraft", *hits_on_aircraft)
                .int("hits_on_supply", *hits_on_supply)
                .int_when_nonzero("hits_on_men", *hits_on_men),
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
        EventKind::BaseSupplySpent {
            base,
            reason,
            aircraft,
            men,
            asked_t,
            paid_t,
            left_t,
        } => {
            let d = d
                .id("base", *base)
                .text("reason", reason.name())
                .int("aircraft", *aircraft);
            // `men` is written only for the men's supply, so that every file
            // made before land units is unchanged (decision 0066).
            let d = if matches!(reason, SupplyUse::Men | SupplyUse::March) {
                d.int("men", *men)
            } else {
                d
            };
            (
                "base_supply_spent",
                d.num("asked_t", *asked_t)
                    .num("paid_t", *paid_t)
                    .num("left_t", *left_t),
            )
        }
        EventKind::MenEmbarked {
            unit,
            ship,
            task_force,
            hex,
            men,
            aboard,
            ashore,
        } => (
            "men_embarked",
            d.id("unit", *unit)
                .id("ship", *ship)
                .id("task_force", *task_force)
                .hex("hex", *hex)
                .int("men", *men)
                .int("aboard", *aboard)
                .int("ashore", *ashore),
        ),
        EventKind::MenLanded {
            unit,
            ship,
            task_force,
            hex,
            men,
            aboard,
            ashore,
        } => (
            "men_landed",
            d.id("unit", *unit)
                .id("ship", *ship)
                .id("task_force", *task_force)
                .hex("hex", *hex)
                .int("men", *men)
                .int("aboard", *aboard)
                .int("ashore", *ashore),
        ),
        EventKind::BaseFormed {
            base,
            side,
            hex,
            task_force,
        } => (
            "base_formed",
            d.id("base", *base)
                .text("side", names::side(*side))
                .hex("hex", *hex)
                .id("task_force", *task_force),
        ),
        EventKind::UnitDivided {
            from,
            unit,
            task_force,
            men,
        } => (
            "unit_divided",
            d.id("from", *from)
                .id("unit", *unit)
                .id("task_force", *task_force)
                .int("men", *men),
        ),
        EventKind::MenLostWithShip { unit, ship, men } => (
            "men_lost_with_ship",
            d.id("unit", *unit).id("ship", *ship).int("men", *men),
        ),
        EventKind::MenFed {
            side,
            hex,
            base,
            men,
            fed,
            unfed,
        } => {
            let d = d.text("side", names::side(*side)).hex("hex", *hex);
            let d = match base {
                Some(base) => d.id("base", *base),
                None => d,
            };
            (
                "men_fed",
                d.int("men", *men).int("fed", *fed).int("unfed", *unfed),
            )
        }
        EventKind::MenLostUnfed {
            unit,
            hex,
            lost,
            ashore,
        } => (
            "men_lost_unfed",
            d.id("unit", *unit)
                .hex("hex", *hex)
                .int("lost", *lost)
                .int("ashore", *ashore),
        ),
        EventKind::LandUnitDestroyed { unit } => ("land_unit_destroyed", d.id("unit", *unit)),
        EventKind::FedShareRestored { unit } => ("fed_share_restored", d.id("unit", *unit)),
        EventKind::GroundFight {
            hex,
            allied_attacks,
            japanese_attacks,
        } => (
            "ground_fight",
            d.hex("hex", *hex)
                .flag("allied_attacks", *allied_attacks)
                .flag("japanese_attacks", *japanese_attacks),
        ),
        EventKind::GroundFire {
            unit,
            hex,
            firing_men,
            shots,
            chance,
            hits,
        } => (
            "ground_fire",
            d.id("unit", *unit)
                .hex("hex", *hex)
                .int("firing_men", *firing_men)
                .num("shots", *shots)
                .num("chance", *chance)
                .int("hits", *hits),
        ),
        EventKind::MenLostInFight {
            unit,
            hex,
            lost,
            ashore,
        } => (
            "men_lost_in_fight",
            d.id("unit", *unit)
                .hex("hex", *hex)
                .int("lost", *lost)
                .int("ashore", *ashore),
        ),
        EventKind::BombardmentFire {
            ship,
            base,
            shells,
            weighed_shots,
            chance,
            expected,
            hits,
            hits_on_aircraft,
            hits_on_supply,
            hits_on_men,
            rounds_left,
        } => (
            "bombardment_fire",
            d.id("ship", *ship)
                .id("base", *base)
                .num("shells", *shells)
                .num("weighed_shots", *weighed_shots)
                .num("chance", *chance)
                .num("expected", *expected)
                .int("hits", *hits)
                .int("hits_on_aircraft", *hits_on_aircraft)
                .int("hits_on_supply", *hits_on_supply)
                .int("hits_on_men", *hits_on_men)
                .num("rounds_left", *rounds_left),
        ),
        EventKind::ShoreBombardment {
            task_force,
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
            d.id("task_force", *task_force)
                .id("base", *base)
                .hex("hex", *hex)
                .num("shells", *shells)
                .num("weighed_shots", *weighed_shots)
                .int("hits", *hits)
                .int("hits_on_aircraft", *hits_on_aircraft)
                .int("hits_on_supply", *hits_on_supply)
                .int("hits_on_men", *hits_on_men),
        ),
        EventKind::MenLostToFire {
            unit,
            base,
            hex,
            lost,
            ashore,
        } => (
            "men_lost_to_fire",
            d.id("unit", *unit)
                .id("base", *base)
                .hex("hex", *hex)
                .int("lost", *lost)
                .int("ashore", *ashore),
        ),
        EventKind::GroundReportReceived {
            side,
            hex,
            men,
            replaced,
        } => (
            "ground_report_received",
            d.text("side", names::side(*side))
                .hex("hex", *hex)
                .int("men", *men)
                .flag("replaced", *replaced),
        ),
        EventKind::MarchDay {
            unit,
            from,
            to,
            days,
            counted,
            of,
        } => (
            "march_day",
            d.id("unit", *unit)
                .hex("from", *from)
                .hex("to", *to)
                .int("days", *days)
                .num("counted", *counted)
                .int("of", *of),
        ),
        EventKind::MarchBaseFormed {
            base,
            side,
            hex,
            unit,
        } => (
            "march_base_formed",
            d.id("base", *base)
                .text("side", names::side(*side))
                .hex("hex", *hex)
                .id("unit", *unit),
        ),
        EventKind::MarchArrived {
            unit,
            from,
            to,
            men,
            supply_t,
            base,
        } => {
            let d = d
                .id("unit", *unit)
                .hex("from", *from)
                .hex("to", *to)
                .int("men", *men)
                .num("supply_t", *supply_t);
            let d = match base {
                Some(base) => d.id("base", *base),
                None => d,
            };
            ("march_arrived", d)
        }
        EventKind::MarchStopped {
            unit,
            to,
            days,
            reason,
        } => (
            "march_stopped",
            d.id("unit", *unit)
                .hex("to", *to)
                .int("days", *days)
                .text("reason", reason.name()),
        ),
        EventKind::TaskForceHalted {
            task_force,
            hex,
            work,
            elapsed_hours,
        } => (
            "task_force_halted",
            d.id("task_force", *task_force)
                .hex("hex", *hex)
                .flag("bombard", work.bombard)
                .flag("cargo", work.cargo)
                .num("elapsed_hours", *elapsed_hours),
        ),
        EventKind::TaskForceSteamedOn {
            task_force,
            hex,
            work,
            fired,
        } => (
            "task_force_steamed_on",
            d.id("task_force", *task_force)
                .hex("hex", *hex)
                .flag("bombard", work.bombard)
                .flag("cargo", work.cargo)
                .flag("fired", *fired),
        ),
        // Preparing a landing and the experience of a land unit (decisions
        // 0095 and 0097). Reasons are snake_case words, as everywhere here.
        EventKind::LoadingGained {
            task_force,
            hex,
            loading,
        } => (
            "loading_gained",
            d.id("task_force", *task_force)
                .hex("hex", *hex)
                .num("loading", *loading),
        ),
        EventKind::LoadingLost {
            task_force,
            hex,
            loading_before,
            loading,
            reason,
        } => (
            "loading_lost",
            d.id("task_force", *task_force)
                .hex("hex", *hex)
                .num("loading_before", *loading_before)
                .num("loading", *loading)
                .text("reason", names::loading_loss(*reason)),
        ),
        EventKind::RehearsalGained {
            unit,
            task_force,
            hex,
            rehearsal,
        } => (
            "rehearsal_gained",
            d.id("unit", *unit)
                .id("task_force", *task_force)
                .hex("hex", *hex)
                .num("rehearsal", *rehearsal),
        ),
        EventKind::RehearsalLost {
            unit,
            hex,
            rehearsal_before,
            reason,
        } => (
            "rehearsal_lost",
            d.id("unit", *unit)
                .hex("hex", *hex)
                .num("rehearsal_before", *rehearsal_before)
                .text("reason", names::rehearsal_loss(*reason)),
        ),
        EventKind::LeftRoster {
            unit,
            task_force,
            reason,
        } => (
            "left_roster",
            d.id("unit", *unit)
                .id("task_force", *task_force)
                .text("reason", names::roster_left(*reason)),
        ),
        EventKind::PreparationShared {
            parent,
            new_force,
            hex,
            loading,
            units,
        } => (
            "preparation_shared",
            d.id("parent", *parent)
                .id("new_force", *new_force)
                .hex("hex", *hex)
                .num("loading", *loading)
                .ids("units", units),
        ),
        EventKind::TrainingDay {
            unit,
            days,
            experience,
        } => (
            "training_day",
            d.id("unit", *unit)
                .int("days", *days)
                .int("experience", u32::from(*experience)),
        ),
        EventKind::FightDayCounted {
            unit,
            days,
            experience,
        } => (
            "fight_day_counted",
            d.id("unit", *unit)
                .int("days", *days)
                .int("experience", u32::from(*experience)),
        ),
        EventKind::ExperienceRaised {
            unit,
            experience,
            by,
        } => (
            "experience_raised",
            d.id("unit", *unit)
                .int("experience", u32::from(*experience))
                .text("by", by.name()),
        ),
        EventKind::BaseFound { side, base, hex } => (
            "base_found",
            d.text("side", names::side(*side))
                .id("base", *base)
                .hex("hex", *hex),
        ),
        EventKind::LandUnitFatigue {
            unit,
            change,
            fatigue,
        } => (
            "land_unit_fatigue",
            d.id("unit", *unit)
                .text(
                    "change",
                    match change {
                        LandFatigueChange::Gained => "gained",
                        LandFatigueChange::Rested => "rested",
                        LandFatigueChange::Marched => "marched",
                        LandFatigueChange::MarchedAndAttacked => "marched_and_attacked",
                        LandFatigueChange::Landed => "landed",
                    },
                )
                .num("fatigue", *fatigue),
        ),
        EventKind::LandUnitBackToDefend { unit, fatigue } => (
            "land_unit_back_to_defend",
            d.id("unit", *unit).num("fatigue", *fatigue),
        ),
        EventKind::SideBeatenAshore {
            side,
            hex,
            men,
            enemy_men,
            fought,
        } => (
            "side_beaten_ashore",
            d.text("side", names::side(*side))
                .hex("hex", *hex)
                .int("men", *men)
                .int("enemy_men", *enemy_men)
                .flag("fought", *fought),
        ),
        EventKind::BaseChangedSide {
            base,
            hex,
            from,
            to,
            supply_t,
            fuel_t,
            supply_lost_t,
            fuel_lost_t,
        } => (
            "base_changed_side",
            d.id("base", *base)
                .hex("hex", *hex)
                .text("from", names::side(*from))
                .text("to", names::side(*to))
                .num("supply_t", *supply_t)
                .num("fuel_t", *fuel_t)
                .num("supply_lost_t", *supply_lost_t)
                .num("fuel_lost_t", *fuel_lost_t),
        ),
        EventKind::AirGroupLostWithBase {
            air_group,
            base,
            aircraft,
        } => (
            "air_group_lost_with_base",
            d.id("air_group", *air_group)
                .id("base", *base)
                .int("aircraft", *aircraft),
        ),
        EventKind::ShipLostWithBase { ship, base } => {
            ("ship_lost_with_base", d.id("ship", *ship).id("base", *base))
        }
        EventKind::HomePortLost { task_force, base } => (
            "home_port_lost",
            d.id("task_force", *task_force).id("base", *base),
        ),
        EventKind::StrikeMissionCleared { air_group, base } => (
            "strike_mission_cleared",
            d.id("air_group", *air_group).id("base", *base),
        ),
        EventKind::UnitLoadingComplete {
            task_force,
            unit,
            base,
            cargo,
            elapsed_hours,
        } => {
            let d = d.id("task_force", *task_force).id("unit", *unit);
            let d = match base {
                Some(base) => d.id("base", *base),
                None => d,
            };
            let d = match cargo {
                Some(cargo) => d.text("cargo", cargo.name()),
                None => d.text("cargo", "none"),
            };
            (
                "unit_loading_complete",
                d.num("elapsed_hours", *elapsed_hours),
            )
        }
        EventKind::StrikeFoundNothing { air_group, hex } => (
            "strike_found_nothing",
            d.id("air_group", *air_group).hex("hex", *hex),
        ),
        EventKind::ContactNotThere { side, contact, hex } => (
            "contact_not_there",
            d.text("side", names::side(*side))
                .int("contact", contact.umpire_raw(key))
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
            after_attack,
        } => (
            "air_fight",
            d.id("firer", *firer)
                .id("target", *target)
                .text("firing", firing.name())
                .text("at", at.name())
                .num("shots", *shots)
                .num("chance", *chance)
                .num("expected", *expected)
                .int("hits", *hits)
                .flag_when_true("after_attack", *after_attack),
        ),
        EventKind::AntiAircraftFire {
            task_force,
            air_group,
            shots,
            chance,
            expected,
            hits,
            after_attack,
        } => (
            "anti_aircraft_fire",
            d.id("task_force", *task_force)
                .id("air_group", *air_group)
                .num("shots", *shots)
                .num("chance", *chance)
                .num("expected", *expected)
                .int("hits", *hits)
                .flag_when_true("after_attack", *after_attack),
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
        EventKind::AircraftArrived {
            air_group,
            aircraft,
            pilots,
            ready,
            pilots_now,
            strength,
        } => (
            "aircraft_arrived",
            d.id("air_group", *air_group)
                .int("aircraft", *aircraft)
                .int("pilots", *pilots)
                .int("ready", *ready)
                .int("pilots_now", *pilots_now)
                .int("strength", *strength),
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
                .int("contact", contact.umpire_raw(key))
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
                .int("contact", contact.umpire_raw(key))
                .int("age_pulses", *age_pulses),
        ),
        EventKind::ContactDropped {
            side,
            contact,
            age_pulses,
        } => (
            "contact_dropped",
            d.text("side", names::side(*side))
                .int("contact", contact.umpire_raw(key))
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
            fatigue_factor,
            sighted,
        } => {
            let d = d
                .int("round", *round)
                .id("task_force", *task_force)
                .id("enemy", *enemy)
                .num("chance", *chance);
            // Left out where fatigue is not in play (decision 0037).
            let d = if *fatigue_factor == Fixed::ONE {
                d
            } else {
                d.num("fatigue_factor", *fatigue_factor)
            };
            ("sighting_roll", d.flag("sighted", *sighted))
        }
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
            claimed_hits,
            claimed_kind,
            claimed_shot_down,
            claimed_on_ground,
            from_air,
            from_bombardment,
        } => {
            let d = d
                .text("side", names::side(*side))
                .hex("hex", *hex)
                .int("claimed_sunk", *claimed_sunk);
            // A bombardment's claim (decision 0092, 17) says aircraft
            // destroyed on the ground and is marked; the others export as
            // they always did.
            let d = if *from_bombardment {
                d.int("claimed_on_ground", *claimed_on_ground)
                    .flag("from_bombardment", true)
            } else {
                d
            };
            // A claim from the air (decision 0056) adds its own figures; a
            // surface fight's claim exports as it always did.
            let d = if *from_air {
                let d = d
                    .int("claimed_hits", *claimed_hits)
                    .int("claimed_shot_down", *claimed_shot_down)
                    .int("claimed_on_ground", *claimed_on_ground);
                match claimed_kind {
                    Some(kind) => d.text("claimed_kind", kind.name()),
                    None => d,
                }
            } else {
                d
            };
            ("claim_received", d)
        }
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
            d.id("task_force", *task_force)
                .int("contact", contact.umpire_raw(key)),
        ),
        EventKind::ForceStoodDown { task_force } => {
            ("force_stood_down", d.id("task_force", *task_force))
        }
        EventKind::TaskForceFatigue {
            task_force,
            change,
            amount,
            fatigue,
        } => (
            "task_force_fatigue",
            d.id("task_force", *task_force)
                .text(
                    "change",
                    match change {
                        ForceFatigueChange::Gained => "gained",
                        ForceFatigueChange::Rested => "rested",
                    },
                )
                .num("amount", *amount)
                .num("fatigue", *fatigue),
        ),
        EventKind::TaskForceTurnedBack {
            task_force,
            from,
            toward,
            progress_nm,
            position,
        } => (
            "task_force_turned_back",
            d.id("task_force", *task_force)
                .hex("from", *from)
                .hex("toward", *toward)
                .num("progress_nm", *progress_nm)
                .hex("position", *position),
        ),
        EventKind::ShipRepaired {
            ship,
            task_force,
            base,
            points,
            fire_damage,
            flotation_damage,
            restored,
            system_repair_points,
        } => (
            "ship_repaired",
            d.id("ship", *ship)
                .id("task_force", *task_force)
                .id("base", *base)
                .num("points", *points)
                .num("fire_damage", *fire_damage)
                .num("flotation_damage", *flotation_damage)
                .with(
                    "restored",
                    Field::Texts(
                        restored
                            .iter()
                            .map(|system| names::system(*system))
                            .collect(),
                    ),
                )
                .num("system_repair_points", *system_repair_points),
        ),
        EventKind::ForceLeftForYard {
            task_force,
            base,
            passage_days,
            ships,
            days_away,
            return_day,
        } => (
            "force_left_for_yard",
            d.id("task_force", *task_force)
                .id("base", *base)
                .int("passage_days", *passage_days)
                // The ships and, in the same order, each one's days of repair.
                .with(
                    "ships",
                    Field::Ids(ships.iter().map(|(ship, _)| ship.raw()).collect()),
                )
                .with(
                    "days_of_repair",
                    Field::Ids(ships.iter().map(|(_, days)| *days).collect()),
                )
                .int("days_away", *days_away)
                .int("return_day", *return_day),
        ),
        EventKind::AircraftWrittenOff {
            air_group,
            base,
            aircraft,
        } => (
            "aircraft_written_off",
            d.id("air_group", *air_group)
                .id("base", *base)
                .int("aircraft", *aircraft),
        ),
        EventKind::AirGroupRebased {
            air_group,
            from,
            to,
            distance_nm,
            transfer_range_nm,
            ready,
            pilots,
        } => {
            let data = d
                .id("air_group", *air_group)
                .id("from", *from)
                .id("to", *to)
                .num("distance_nm", *distance_nm)
                .num("transfer_range_nm", *transfer_range_nm)
                .int("ready", *ready);
            // Left out where the group's pilots are not counted.
            let data = match pilots {
                Some(pilots) => data.int("pilots", *pilots),
                None => data,
            };
            ("air_group_rebased", data)
        }
        EventKind::ForceReturnedFromYard {
            task_force,
            base,
            ships,
        } => (
            "force_returned_from_yard",
            d.id("task_force", *task_force)
                .id("base", *base)
                .ids("ships", ships),
        ),
        EventKind::BaseAtDayEnd {
            base,
            supply_t,
            aircraft_ready,
        } => (
            "base_at_day_end",
            d.id("base", *base)
                .num("supply_t", *supply_t)
                .int("aircraft_ready", *aircraft_ready)
                .flag(
                    "open",
                    sim_core::events::base_is_open(*supply_t, *aircraft_ready),
                ),
        ),
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
