//! The event log as JSON.

use serde::Serialize;
use sim_core::entering::EntryWait;
use sim_core::events::{
    Arrival, Breakdown, Cause, Event, EventKind, FatigueChange, ForceFatigueChange,
    LandFatigueChange, ReportListener, SupplyUse,
};
use sim_core::fixed::Fixed;
use sim_core::ids::UmpireKey;
use sim_core::orders::{Cargo, RejectReason};
use sim_core::state::AirGroupLocation;
use sim_core::time::Pulse;

use super::data::{Data, Field};
use super::names;
use super::num::Num;

#[derive(Serialize)]
pub struct EventOut {
    id: u32,
    day: u32,
    pulse: Option<&'static str>,
    kind: &'static str,
    /// `Event`'s `Display`, except that a command is worded with its numbers
    /// in agreement ("1 ship") and a staff reason for a command says so.
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
    let data = describe(&event.kind, key, (event.day, event.pulse));
    EventOut {
        id: event.id.umpire_raw(key),
        day: event.day,
        pulse: event.pulse.map(|pulse| pulse.name()),
        kind: event.kind.tag(),
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
        // The truth's reason, with what the area command held (decision
        // 0136, 13).
        RejectReason::NotAsHeld { reason, held } => {
            reject_reason(reason).with("not_as_held", Field::Str(held.clone()))
        }
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
        RejectReason::AircraftCannotSearch => base("aircraft_cannot_search"),
        RejectReason::GivenAtDusk => base("given_at_dusk"),
        // Decision 0162.
        RejectReason::LevelNotAboveZero(level) => {
            base("level_not_above_zero").num("each_t", *level)
        }
        // Decision 0190.
        RejectReason::WatersOffMap(hex) => base("waters_off_map").hex("hex", *hex),
        RejectReason::WatersHexTwice(hex) => base("waters_hex_twice").hex("hex", *hex),
        RejectReason::PatrolShareOutOfRange(share) => {
            base("patrol_share_out_of_range").num("share", *share)
        }
        RejectReason::StrikeContactAgeOutOfRange { pulses, max } => {
            let data = base("strike_contact_age_out_of_range").with("pulses", Field::Int(*pulses));
            match max {
                Some(max) => data.with("max", Field::Int(i64::from(*max))),
                None => data,
            }
        }
        RejectReason::StrikeHeldShareOutOfRange(share) => {
            base("strike_held_share_out_of_range").num("share", *share)
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
        RejectReason::NoAirfield => base("no_airfield"),
        RejectReason::WorkingUnitDefends => base("working_unit_defends"),
        RejectReason::WorkingUnitWorks => base("working_unit_works"),
        RejectReason::InstructionNeedsAttack => base("instruction_needs_attack"),
        RejectReason::AirfieldTooDamaged { damage, refused_at } => base("airfield_too_damaged")
            .num("damage", *damage)
            .num("refused_at", *refused_at),
        RejectReason::AirGroupWaitsForBase => base("air_group_waits_for_base"),
        // Decision 0111.
        RejectReason::WeightNotAboveZero(weight) => {
            base("weight_not_above_zero").num("weight_t", *weight)
        }
        RejectReason::NoShipsToForm => base("no_ships_to_form"),
        RejectReason::ShipNamedTwice(ship) => base("ship_named_twice").id("ship", *ship),
        RejectReason::ShipNotInPortHere(ship) => base("ship_not_in_port_here").id("ship", *ship),
        RejectReason::RouteToFollow => base("route_to_follow"),
        RejectReason::NotAtSameBase(other) => base("not_at_same_base").id("task_force", *other),
        RejectReason::CannotJoinItself => base("cannot_join_itself"),
        RejectReason::ForceWasJoined { into } => base("force_was_joined").id("into", *into),
        RejectReason::ForceWasStoodDown => base("force_was_stood_down"),
        RejectReason::OutsideForcesLimit => base("outside_forces_limit"),
        RejectReason::Held(target) => base("held").with("target", Field::Str(target.to_string())),
        RejectReason::KeepsStationOn(charge) => base("keeps_station_on").id("charge", *charge),
        RejectReason::NotOnEscortMission => base("not_on_escort_mission"),
        RejectReason::CannotEscortItself => base("cannot_escort_itself"),
        RejectReason::ChargeIsAnEscort(charge) => base("charge_is_an_escort").id("charge", *charge),
        RejectReason::IsACharge { escort } => base("is_a_charge").id("escort", *escort),
        // Decision 0140, 2.
        RejectReason::NotYetAvailable { unit, day } => base("not_yet_available")
            .with("unit", Field::Str(unit.to_string()))
            .int("day", *day),
        // Decision 0195, 8 and 15.
        RejectReason::NotYetEntered { unit, day, waiting } => base("not_yet_entered")
            .with("unit", Field::Str(unit.to_string()))
            .int("day", *day)
            .flag("waiting", *waiting),
        // Decision 0201, 5.
        RejectReason::GiverHasNoOiler(giver) => base("giver_has_no_oiler").id("giver", *giver),
        RejectReason::CannotFuelFromItself => base("cannot_fuel_from_itself"),
        RejectReason::GiverIsFuelling(giver) => base("giver_is_fuelling").id("giver", *giver),
        RejectReason::GiverIsAnEscort(giver) => base("giver_is_an_escort").id("giver", *giver),
        RejectReason::GiverHasAnEscort { giver, escort } => base("giver_has_an_escort")
            .id("giver", *giver)
            .id("escort", *escort),
        RejectReason::IsAGiver { taker } => base("is_a_giver").id("taker", *taker),
        RejectReason::EscortCannotFuel { charge } => {
            base("escort_cannot_fuel").id("charge", *charge)
        }
        RejectReason::HasAnEscort { escort } => base("has_an_escort").id("escort", *escort),
        RejectReason::FuelsFrom(giver) => base("fuels_from").id("giver", *giver),
        RejectReason::ChargeTakesPartInFuelling(charge) => {
            base("charge_takes_part_in_fuelling").id("charge", *charge)
        }
        RejectReason::SubmarineOffTheMap => base("submarine_off_the_map"),
        RejectReason::SubmarineGoingHome => base("submarine_going_home"),
    }
}

/// Every field an event carries, under the
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

pub(super) fn describe(kind: &EventKind, key: &UmpireKey, at: (u32, Option<Pulse>)) -> Data {
    let d = Data::new();
    match kind {
        EventKind::OrderAccepted { order, target } => d
            .int("order", order.umpire_raw(key))
            .with("target", Field::Str(target.to_string())),
        EventKind::OrderRejected {
            order,
            target,
            reason,
        } => d
            .int("order", order.umpire_raw(key))
            .with("target", Field::Str(target.to_string()))
            .with("reason", Field::Obj(reject_reason(reason))),
        EventKind::TaskForceSteamed {
            task_force,
            from,
            to,
            distance_nm,
            speed_kn,
            hexes_entered,
        } => d
            .id("task_force", *task_force)
            .hex("from", *from)
            .hex("to", *to)
            .num("distance_nm", *distance_nm)
            .num("speed_kn", *speed_kn)
            .with("hexes_entered", Field::Hexes(hexes_entered.clone())),
        EventKind::FuelBurned {
            ship,
            task_force,
            burned_t,
            remaining_t,
        } => d
            .id("ship", *ship)
            .id("task_force", *task_force)
            .num("burned_t", *burned_t)
            .num("remaining_t", *remaining_t),
        EventKind::TaskForceArrived {
            task_force,
            hex,
            elapsed_hours,
        } => d
            .id("task_force", *task_force)
            .hex("hex", *hex)
            .num("elapsed_hours", *elapsed_hours),
        EventKind::TaskForceOutOfFuel { task_force, hex } => {
            d.id("task_force", *task_force).hex("hex", *hex)
        }
        EventKind::ShipRefuelled {
            ship,
            task_force,
            base,
            taken_t,
            fuel_t,
            base_fuel_t,
            force_restored,
        } => d
            .id("ship", *ship)
            .id("task_force", *task_force)
            .id("base", *base)
            .num("taken_t", *taken_t)
            .num("fuel_t", *fuel_t)
            .num("base_fuel_t", *base_fuel_t)
            .flag("force_restored", *force_restored),
        EventKind::CargoLoaded {
            ship,
            task_force,
            base,
            cargo,
            loaded_t,
            aboard_t,
            base_stock_t,
        } => cargo_named(
            d.id("ship", *ship)
                .id("task_force", *task_force)
                .id("base", *base),
            *cargo,
        )
        .num("loaded_t", *loaded_t)
        .num("aboard_t", *aboard_t)
        .num("base_stock_t", *base_stock_t),
        EventKind::CargoUnloaded {
            ship,
            task_force,
            base,
            cargo,
            unloaded_t,
            aboard_t,
            base_stock_t,
        } => cargo_named(
            d.id("ship", *ship)
                .id("task_force", *task_force)
                .id("base", *base),
            *cargo,
        )
        .num("unloaded_t", *unloaded_t)
        .num("aboard_t", *aboard_t)
        .num("base_stock_t", *base_stock_t),
        EventKind::LoadingComplete {
            task_force,
            base,
            cargo,
            elapsed_hours,
        } => cargo_named(d.id("task_force", *task_force).id("base", *base), *cargo)
            .num("elapsed_hours", *elapsed_hours),
        EventKind::UnloadingComplete {
            task_force,
            base,
            elapsed_hours,
        } => d
            .id("task_force", *task_force)
            .id("base", *base)
            .num("elapsed_hours", *elapsed_hours),
        EventKind::SearchFlown {
            air_group,
            from,
            hex,
            from_deg,
            to_deg,
            range_nm,
            aircraft,
        } => match from {
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
        EventKind::DamagedAircraftDestroyedOnGround {
            air_group,
            base,
            aircraft,
            remaining,
        } => d
            .id("air_group", *air_group)
            .id("base", *base)
            .int("aircraft", *aircraft)
            .int("remaining", *remaining),
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
            data.int("pilots_lost", *pilots_lost).int("ready", *ready)
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
        } => d
            .id("air_group", *air_group)
            .text("sortie", sortie.name())
            .num("distance_nm", *distance_nm)
            .int("aircraft", *aircraft)
            .num("chance", *chance)
            .num("expected", *expected)
            .int("worn", *worn)
            .int("written_off", *written_off)
            .int("ready", *ready)
            .int("damaged", *damaged),
        EventKind::AircraftLandedElsewhere {
            from,
            to,
            at,
            aircraft,
            pilots,
            formed,
        } => match at {
            AirGroupLocation::Base(base) => d.id("from", *from).id("to", *to).id("base", *base),
            AirGroupLocation::Ship(ship) => d.id("from", *from).id("to", *to).id("ship", *ship),
            AirGroupLocation::Lost => d.id("from", *from).id("to", *to),
        }
        .int("aircraft", *aircraft)
        .int("pilots", *pilots)
        .flag("formed", *formed),
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
            data.int("pilots_lost", *pilots_lost).int("ready", *ready)
        }
        EventKind::AircraftRepaired {
            air_group,
            at,
            repaired,
            ready,
            damaged,
        } => match at {
            AirGroupLocation::Base(base) => d.id("air_group", *air_group).id("base", *base),
            AirGroupLocation::Ship(ship) => d.id("air_group", *air_group).id("ship", *ship),
            // Lost with its base: no place (decision 0090, 20).
            AirGroupLocation::Lost => d.id("air_group", *air_group),
        }
        .int("repaired", *repaired)
        .int("ready", *ready)
        .int("damaged", *damaged),
        EventKind::AirGroupFatigue {
            air_group,
            change,
            amount,
            fatigue,
        } => match change {
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
        EventKind::AirGroupStoodDown {
            air_group,
            fatigue,
            threshold,
        } => d
            .id("air_group", *air_group)
            .num("fatigue", *fatigue)
            .num("threshold", *threshold),
        EventKind::AirGroupReturned {
            air_group,
            fatigue,
            threshold,
        } => d
            .id("air_group", *air_group)
            .num("fatigue", *fatigue)
            .num("threshold", *threshold),
        EventKind::AirGroupLost {
            air_group,
            ship,
            aircraft,
        } => d
            .id("air_group", *air_group)
            .id("ship", *ship)
            .int("aircraft", *aircraft),
        EventKind::StrikeLaunched {
            air_group,
            contact,
            hex,
            aircraft,
            distance_nm,
        } => d
            .id("air_group", *air_group)
            .int("contact", contact.umpire_raw(key))
            .hex("hex", *hex)
            .int("aircraft", *aircraft)
            .num("distance_nm", *distance_nm),
        EventKind::BaseStrikeLaunched {
            air_group,
            base,
            hex,
            aircraft,
            distance_nm,
        } => d
            .id("air_group", *air_group)
            .id("base", *base)
            .hex("hex", *hex)
            .int("aircraft", *aircraft)
            .num("distance_nm", *distance_nm),
        EventKind::BaseAntiAircraftFire {
            base,
            air_group,
            shots,
            chance,
            expected,
            hits,
            after_attack,
        } => d
            .id("base", *base)
            .id("air_group", *air_group)
            .num("shots", *shots)
            .num("chance", *chance)
            .num("expected", *expected)
            .int("hits", *hits)
            .flag_when_true("after_attack", *after_attack),
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
            recognised,
        } => d
            .id("air_group", *air_group)
            .id("base", *base)
            .num("shots", *shots)
            .num("chance", *chance)
            .num("expected", *expected)
            .int("hits", *hits)
            .int("hits_on_aircraft", *hits_on_aircraft)
            .int("hits_on_supply", *hits_on_supply)
            .int_when_nonzero("hits_on_men", *hits_on_men)
            .id_when_some("recognised", *recognised),
        EventKind::AircraftDestroyedOnGround {
            air_group,
            base,
            aircraft,
            remaining,
        } => d
            .id("air_group", *air_group)
            .id("base", *base)
            .int("aircraft", *aircraft)
            .int("remaining", *remaining),
        EventKind::BaseSupplyDestroyed {
            base,
            destroyed_t,
            supply_t,
        } => d
            .id("base", *base)
            .num("destroyed_t", *destroyed_t)
            .num("supply_t", *supply_t),
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
            d.num("asked_t", *asked_t)
                .num("paid_t", *paid_t)
                .num("left_t", *left_t)
        }
        EventKind::MenEmbarked {
            unit,
            ship,
            task_force,
            hex,
            men,
            aboard,
            ashore,
        } => d
            .id("unit", *unit)
            .id("ship", *ship)
            .id("task_force", *task_force)
            .hex("hex", *hex)
            .int("men", *men)
            .int("aboard", *aboard)
            .int("ashore", *ashore),
        EventKind::MenLanded {
            unit,
            ship,
            task_force,
            hex,
            men,
            aboard,
            ashore,
        } => d
            .id("unit", *unit)
            .id("ship", *ship)
            .id("task_force", *task_force)
            .hex("hex", *hex)
            .int("men", *men)
            .int("aboard", *aboard)
            .int("ashore", *ashore),
        EventKind::BaseFormed {
            base,
            side,
            hex,
            task_force,
        } => d
            .id("base", *base)
            .text("side", names::side(*side))
            .hex("hex", *hex)
            .id("task_force", *task_force),
        EventKind::UnitDivided {
            from,
            unit,
            task_force,
            men,
        } => d
            .id("from", *from)
            .id("unit", *unit)
            .id("task_force", *task_force)
            .int("men", *men),
        EventKind::MenLostWithShip { unit, ship, men } => {
            d.id("unit", *unit).id("ship", *ship).int("men", *men)
        }
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
            d.int("men", *men).int("fed", *fed).int("unfed", *unfed)
        }
        EventKind::MenLostUnfed {
            unit,
            hex,
            lost,
            ashore,
        } => d
            .id("unit", *unit)
            .hex("hex", *hex)
            .int("lost", *lost)
            .int("ashore", *ashore),
        EventKind::LandUnitDestroyed { unit } => d.id("unit", *unit),
        EventKind::FedShareRestored { unit } => d.id("unit", *unit),
        EventKind::GroundFight {
            hex,
            allied_attacks,
            japanese_attacks,
        } => d
            .hex("hex", *hex)
            .flag("allied_attacks", *allied_attacks)
            .flag("japanese_attacks", *japanese_attacks),
        EventKind::GroundFire {
            unit,
            hex,
            firing_men,
            shots,
            chance,
            hits,
        } => d
            .id("unit", *unit)
            .hex("hex", *hex)
            .int("firing_men", *firing_men)
            .num("shots", *shots)
            .num("chance", *chance)
            .int("hits", *hits),
        EventKind::MenLostInFight {
            unit,
            hex,
            lost,
            ashore,
        } => d
            .id("unit", *unit)
            .hex("hex", *hex)
            .int("lost", *lost)
            .int("ashore", *ashore),
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
        } => d
            .id("ship", *ship)
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
        } => d
            .id("task_force", *task_force)
            .id("base", *base)
            .hex("hex", *hex)
            .num("shells", *shells)
            .num("weighed_shots", *weighed_shots)
            .int("hits", *hits)
            .int("hits_on_aircraft", *hits_on_aircraft)
            .int("hits_on_supply", *hits_on_supply)
            .int("hits_on_men", *hits_on_men),
        EventKind::MenLostToFire {
            unit,
            base,
            hex,
            lost,
            ashore,
        } => d
            .id("unit", *unit)
            .id("base", *base)
            .hex("hex", *hex)
            .int("lost", *lost)
            .int("ashore", *ashore),
        EventKind::SubmarineReported {
            side,
            hex,
            replaced,
        } => d
            .text("side", names::side(*side))
            .hex("hex", *hex)
            .flag("replaced", *replaced),
        EventKind::SubmarineReportDropped { side, hex } => {
            d.text("side", names::side(*side)).hex("hex", *hex)
        }
        EventKind::SubmarineOrderedToStation { submarine, station } => {
            d.id("submarine", *submarine).hex("station", *station)
        }
        EventKind::SuspectedWatersTold {
            side,
            hex,
            distance,
            day,
            words,
        } => d
            .text("side", names::side(*side))
            .hex("hex", *hex)
            .int("distance", *distance)
            .int("day", *day)
            .with("words", Field::Str(words.clone())),
        EventKind::GroundReportReceived {
            side,
            hex,
            men,
            replaced,
            grade,
        } => d
            .text("side", names::side(*side))
            .hex("hex", *hex)
            .int("men", *men)
            .flag("replaced", *replaced)
            .text("grade", grade.name()),
        EventKind::ReconnaissanceFlown {
            air_group,
            from,
            hex,
            distance_nm,
            supply_t,
        } => match from {
            AirGroupLocation::Base(base) => d.id("air_group", *air_group).id("base", *base),
            AirGroupLocation::Ship(ship) => d.id("air_group", *air_group).id("ship", *ship),
            AirGroupLocation::Lost => d.id("air_group", *air_group),
        }
        .hex("hex", *hex)
        .num("distance_nm", *distance_nm)
        .num("supply_t", *supply_t),
        EventKind::GroundFoundEmpty {
            side,
            hex,
            men,
            reported_day,
        } => d
            .text("side", names::side(*side))
            .hex("hex", *hex)
            .int("men", *men)
            .int("reported_day", *reported_day),
        EventKind::MarchDay {
            unit,
            from,
            to,
            days,
            counted,
            of,
        } => d
            .id("unit", *unit)
            .hex("from", *from)
            .hex("to", *to)
            .int("days", *days)
            .num("counted", *counted)
            .int("of", *of),
        EventKind::MarchBaseFormed {
            base,
            side,
            hex,
            unit,
        } => d
            .id("base", *base)
            .text("side", names::side(*side))
            .hex("hex", *hex)
            .id("unit", *unit),
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
            match base {
                Some(base) => d.id("base", *base),
                None => d,
            }
        }
        EventKind::MarchStopped {
            unit,
            to,
            days,
            reason,
        } => d
            .id("unit", *unit)
            .hex("to", *to)
            .int("days", *days)
            .text("reason", reason.name()),
        EventKind::TaskForceHalted {
            task_force,
            hex,
            work,
            elapsed_hours,
        } => d
            .id("task_force", *task_force)
            .hex("hex", *hex)
            .flag("bombard", work.bombard)
            .flag("cargo", work.cargo)
            .flag_when_true("night", work.night)
            .num("elapsed_hours", *elapsed_hours),
        EventKind::TaskForceSteamedOn {
            task_force,
            hex,
            work,
            fired,
        } => d
            .id("task_force", *task_force)
            .hex("hex", *hex)
            .flag("bombard", work.bombard)
            .flag("cargo", work.cargo)
            .flag_when_true("night", work.night)
            .flag("fired", *fired),
        // Preparing a landing and the experience of a land unit (decisions
        // 0095 and 0097). Reasons are snake_case words, as everywhere here.
        EventKind::LoadingGained {
            task_force,
            hex,
            loading,
        } => d
            .id("task_force", *task_force)
            .hex("hex", *hex)
            .num("loading", *loading),
        EventKind::LoadingLost {
            task_force,
            hex,
            loading_before,
            loading,
            reason,
        } => d
            .id("task_force", *task_force)
            .hex("hex", *hex)
            .num("loading_before", *loading_before)
            .num("loading", *loading)
            .text("reason", names::loading_loss(*reason)),
        EventKind::RehearsalGained {
            unit,
            task_force,
            hex,
            rehearsal,
        } => d
            .id("unit", *unit)
            .id("task_force", *task_force)
            .hex("hex", *hex)
            .num("rehearsal", *rehearsal),
        EventKind::RehearsalLost {
            unit,
            hex,
            rehearsal_before,
            reason,
        } => d
            .id("unit", *unit)
            .hex("hex", *hex)
            .num("rehearsal_before", *rehearsal_before)
            .text("reason", names::rehearsal_loss(*reason)),
        EventKind::LeftRoster {
            unit,
            task_force,
            reason,
        } => d
            .id("unit", *unit)
            .id("task_force", *task_force)
            .text("reason", names::roster_left(*reason)),
        EventKind::PreparationShared {
            parent,
            new_force,
            hex,
            loading,
            units,
        } => d
            .id("parent", *parent)
            .id("new_force", *new_force)
            .hex("hex", *hex)
            .num("loading", *loading)
            .ids("units", units),
        EventKind::TrainingDay {
            unit,
            days,
            experience,
        } => d
            .id("unit", *unit)
            .int("days", *days)
            .int("experience", u32::from(*experience)),
        EventKind::UnitWorked {
            unit,
            base,
            job,
            work,
            total,
        } => d
            .id("unit", *unit)
            .id("base", *base)
            .text("job", job.name())
            .num("work", *work)
            .num("total", *total),
        EventKind::AirfieldBuilt {
            base,
            side,
            hex,
            work_done,
        } => d
            .text("side", names::side(*side))
            .id("base", *base)
            .hex("hex", *hex)
            .num("work_done", *work_done),
        EventKind::AirfieldDamaged {
            base,
            side,
            hits,
            damage,
        } => d
            .text("side", names::side(*side))
            .id("base", *base)
            .int("hits", *hits)
            .num("damage", *damage),
        EventKind::ArrivalWaiting {
            air_group,
            base,
            aircraft,
            pilots,
            damage,
        } => d
            .id("air_group", *air_group)
            .id("base", *base)
            .int("aircraft", *aircraft)
            .int("pilots", *pilots)
            .num("damage", *damage),
        EventKind::ForceEntered {
            force,
            side,
            hex,
            base,
            ships,
            air_groups,
            land_units,
        } => d
            .id("force", *force)
            .text("side", names::side(*side))
            .hex("hex", *hex)
            .id_when_some("base", *base)
            .ids("ships", ships)
            .ids("air_groups", air_groups)
            .ids("land_units", land_units),
        EventKind::SubmarineEntered { submarine, hex } => {
            d.id("submarine", *submarine).hex("hex", *hex)
        }
        EventKind::SubmarineSteamed {
            submarine,
            from,
            to,
        } => d
            .id("submarine", *submarine)
            .hex("from", *from)
            .hex("to", *to),
        EventKind::SubmarineOnStation { submarine, hex } => {
            d.id("submarine", *submarine).hex("hex", *hex)
        }
        EventKind::SubmarineLeftForHome {
            submarine,
            base,
            reason,
        } => d
            .id("submarine", *submarine)
            .id("base", *base)
            .text("reason", reason.name()),
        EventKind::SubmarineAttackChance {
            submarine,
            task_force,
            hex,
            hours,
            chance,
            attacks,
        } => d
            .id("submarine", *submarine)
            .id("task_force", *task_force)
            .hex("hex", *hex)
            .num("hours", *hours)
            .num("chance", *chance)
            .num("attacks", *attacks),
        EventKind::SubmarineSalvo {
            submarine,
            task_force,
            ship,
            hex,
            torpedoes,
            hits,
            taken_for,
        } => d
            .id("submarine", *submarine)
            .id("task_force", *task_force)
            .id("ship", *ship)
            .hex("hex", *hex)
            .int("torpedoes", *torpedoes)
            .int("hits", *hits)
            .text("taken_for", taken_for.name()),
        EventKind::SubmarineHunted {
            submarine,
            task_force,
            hex,
            held_down_until,
            damaged,
            sunk,
        } => d
            .id("submarine", *submarine)
            .id("task_force", *task_force)
            .hex("hex", *hex)
            .int("held_down_until_day", held_down_until.0)
            .text("held_down_until_pulse", held_down_until.1.name())
            .flag("damaged", *damaged)
            .flag("sunk", *sunk),
        EventKind::SubmarineSearchAttempt {
            air_group,
            submarine,
            distance_nm,
            chance,
            sighted,
        } => d
            .id("air_group", *air_group)
            .id("submarine", *submarine)
            .num("distance_nm", *distance_nm)
            .num("chance", *chance)
            .flag("sighted", *sighted),
        EventKind::SubmarineAttackedFromAir {
            air_group,
            submarine,
            hex,
            damaged,
            sunk,
        } => d
            .id("air_group", *air_group)
            .id("submarine", *submarine)
            .hex("hex", *hex)
            .flag("damaged", *damaged)
            .flag("sunk", *sunk),
        EventKind::SubmarineSunk { submarine } => d.id("submarine", *submarine),
        EventKind::SubmarineReachedHome { submarine, base } => {
            d.id("submarine", *submarine).id("base", *base)
        }
        EventKind::UnitEntered {
            unit,
            side,
            base,
            hex,
            men,
            aircraft,
        } => d
            .with("unit", Field::Str(unit.to_string()))
            .text("side", names::side(*side))
            .id("base", *base)
            .hex("hex", *hex)
            .int("men", *men)
            .int("aircraft", *aircraft),
        EventKind::EntryWaiting {
            unit,
            side,
            base,
            day,
            wait,
        } => d
            .with("unit", Field::Str(unit.to_string()))
            .text("side", names::side(*side))
            .id("base", *base)
            .int("day", *day)
            .text(
                "wait",
                match wait {
                    EntryWait::NotTheSidesBase => "not_the_sides_base",
                    EntryWait::TakesNoAircraft => "takes_no_aircraft",
                },
            ),
        EventKind::StockReceived {
            base,
            side,
            supply_t,
            fuel_t,
        } => d
            .id("base", *base)
            .text("side", names::side(*side))
            .num("supply_t", *supply_t)
            .num("fuel_t", *fuel_t),
        EventKind::MenJoined {
            unit,
            side,
            base,
            men,
            men_after,
            experience_before,
            experience_after,
            joined,
        } => d
            .id("unit", *unit)
            .text("side", names::side(*side))
            .id("base", *base)
            .int("men", *men)
            .int("men_after", *men_after)
            .int("experience_before", u32::from(*experience_before))
            .int("experience_after", u32::from(*experience_after))
            .int("joined", *joined),
        EventKind::RationsLanded {
            base,
            side,
            task_force,
            men,
            nights,
            supply_t,
        } => d
            .id("base", *base)
            .text("side", names::side(*side))
            .id("task_force", *task_force)
            .int("men", *men)
            .num("nights", *nights)
            .num("supply_t", *supply_t),
        EventKind::HungerNightsChanged {
            unit,
            hex,
            before,
            nights,
        } => d
            .id("unit", *unit)
            .hex("hex", *hex)
            .int("before", *before)
            .int("nights", *nights),
        EventKind::FightSupplyCharged {
            side,
            hex,
            men_at_grips,
            wanted_t,
            from_ammunition_t,
            from_supply_t,
            supplied_share,
        } => d
            .text("side", names::side(*side))
            .hex("hex", *hex)
            .num("men_at_grips", *men_at_grips)
            .num("wanted_t", *wanted_t)
            .num("from_ammunition_t", *from_ammunition_t)
            .num("from_supply_t", *from_supply_t)
            .num("supplied_share", *supplied_share),
        EventKind::BaseAmmunitionSpent {
            base,
            asked_t,
            paid_t,
            left_t,
        } => d
            .id("base", *base)
            .num("asked_t", *asked_t)
            .num("paid_t", *paid_t)
            .num("left_t", *left_t),
        EventKind::AmmunitionLanded {
            base,
            side,
            task_force,
            men,
            days,
            ammunition_t,
        } => d
            .id("base", *base)
            .text("side", names::side(*side))
            .id("task_force", *task_force)
            .int("men", *men)
            .num("days", *days)
            .num("ammunition_t", *ammunition_t),
        EventKind::AirfieldMended {
            unit,
            base,
            points,
            damage,
        } => d
            .id("unit", *unit)
            .id("base", *base)
            .num("points", *points)
            .num("damage", *damage),
        EventKind::UnitFellBack {
            unit,
            side,
            from,
            to,
            men,
            fatigue,
        } => d
            .id("unit", *unit)
            .text("side", names::side(*side))
            .hex("from", *from)
            .hex("to", *to)
            .int("men", *men)
            .num("fatigue", *fatigue),
        EventKind::UnitSurrendered {
            unit,
            side,
            hex,
            men,
        } => d
            .id("unit", *unit)
            .text("side", names::side(*side))
            .hex("hex", *hex)
            .int("men", *men),
        EventKind::MenSurrenderedAshore { side, hex, men } => d
            .text("side", names::side(*side))
            .hex("hex", *hex)
            .int("men", *men),
        EventKind::GroundAttackFailed {
            side,
            hex,
            weight,
            enemy_weight,
        } => d
            .text("side", names::side(*side))
            .hex("hex", *hex)
            .num("weight", *weight)
            .num("enemy_weight", *enemy_weight),
        EventKind::SpentUnitFellBack {
            unit,
            side,
            from,
            to,
            men,
            fatigue,
        } => d
            .id("unit", *unit)
            .text("side", names::side(*side))
            .hex("from", *from)
            .hex("to", *to)
            .int("men", *men)
            .num("fatigue", *fatigue),
        EventKind::FieldworksChanged {
            unit,
            change,
            fieldworks,
        } => d
            .id("unit", *unit)
            .text("change", change.name())
            .num("fieldworks", *fieldworks),
        EventKind::DefencesWorn {
            base,
            side,
            hex,
            hits,
            defences,
        } => d
            .text("side", names::side(*side))
            .id("base", *base)
            .hex("hex", *hex)
            .int("hits", *hits)
            .num("defences", *defences),
        EventKind::FightDayCounted {
            unit,
            days,
            experience,
        } => d
            .id("unit", *unit)
            .int("days", *days)
            .int("experience", u32::from(*experience)),
        EventKind::ExperienceRaised {
            unit,
            experience,
            by,
        } => d
            .id("unit", *unit)
            .int("experience", u32::from(*experience))
            .text("by", by.name()),
        EventKind::BaseFound { side, base, hex } => d
            .text("side", names::side(*side))
            .id("base", *base)
            .hex("hex", *hex),
        EventKind::LandUnitFatigue {
            unit,
            change,
            fatigue,
        } => d
            .id("unit", *unit)
            .text(
                "change",
                match change {
                    LandFatigueChange::Gained => "gained",
                    LandFatigueChange::Rested => "rested",
                    LandFatigueChange::Marched => "marched",
                    LandFatigueChange::MarchedAndAttacked => "marched_and_attacked",
                    LandFatigueChange::Landed => "landed",
                    LandFatigueChange::Strained => "strained",
                },
            )
            .num("fatigue", *fatigue),
        EventKind::LandUnitBackToDefend { unit, fatigue } => {
            d.id("unit", *unit).num("fatigue", *fatigue)
        }
        EventKind::SideBeatenAshore {
            side,
            hex,
            men,
            enemy_men,
            fought,
        } => d
            .text("side", names::side(*side))
            .hex("hex", *hex)
            .int("men", *men)
            .int("enemy_men", *enemy_men)
            .flag("fought", *fought),
        EventKind::BaseChangedSide {
            base,
            hex,
            from,
            to,
            supply_t,
            fuel_t,
            supply_lost_t,
            fuel_lost_t,
        } => d
            .id("base", *base)
            .hex("hex", *hex)
            .text("from", names::side(*from))
            .text("to", names::side(*to))
            .num("supply_t", *supply_t)
            .num("fuel_t", *fuel_t)
            .num("supply_lost_t", *supply_lost_t)
            .num("fuel_lost_t", *fuel_lost_t),
        EventKind::AirGroupLostWithBase {
            air_group,
            base,
            aircraft,
        } => d
            .id("air_group", *air_group)
            .id("base", *base)
            .int("aircraft", *aircraft),
        EventKind::ShipLostWithBase { ship, base } => d.id("ship", *ship).id("base", *base),
        EventKind::HomePortLost { task_force, base } => {
            d.id("task_force", *task_force).id("base", *base)
        }
        EventKind::StrikeMissionCleared { air_group, base } => {
            d.id("air_group", *air_group).id("base", *base)
        }
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
            d.num("elapsed_hours", *elapsed_hours)
        }
        EventKind::StrikeFoundNothing { air_group, hex } => {
            d.id("air_group", *air_group).hex("hex", *hex)
        }
        EventKind::ContactNotThere { side, contact, hex } => d
            .text("side", names::side(*side))
            .int("contact", contact.umpire_raw(key))
            .hex("hex", *hex),
        EventKind::AirAttack {
            air_group,
            task_force,
            ship,
            weapon,
            shots,
            chance,
            expected,
            hits,
            recognised,
        } => d
            .id("air_group", *air_group)
            .id("task_force", *task_force)
            .id("ship", *ship)
            .text("weapon", names::weapon(*weapon))
            .num("shots", *shots)
            .num("chance", *chance)
            .num("expected", *expected)
            .int("hits", *hits)
            .id_when_some("recognised", *recognised),
        EventKind::EscortsSent {
            air_group,
            with_air_group,
            aircraft,
        } => d
            .id("air_group", *air_group)
            .id("with_air_group", *with_air_group)
            .int("aircraft", *aircraft),
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
        } => d
            .id("firer", *firer)
            .id("target", *target)
            .text("firing", firing.name())
            .text("at", at.name())
            .num("shots", *shots)
            .num("chance", *chance)
            .num("expected", *expected)
            .int("hits", *hits)
            .flag_when_true("after_attack", *after_attack),
        EventKind::AntiAircraftFire {
            task_force,
            air_group,
            shots,
            chance,
            expected,
            hits,
            after_attack,
        } => d
            .id("task_force", *task_force)
            .id("air_group", *air_group)
            .num("shots", *shots)
            .num("chance", *chance)
            .num("expected", *expected)
            .int("hits", *hits)
            .flag_when_true("after_attack", *after_attack),
        EventKind::AircraftLost {
            air_group,
            aircraft,
            remaining,
        } => d
            .id("air_group", *air_group)
            .int("aircraft", *aircraft)
            .int("remaining", *remaining),
        EventKind::PilotsLost {
            air_group,
            lost,
            left,
        } => d
            .id("air_group", *air_group)
            .int("lost", *lost)
            .int("left", *left),
        EventKind::AircraftArrived {
            air_group,
            aircraft,
            pilots,
            ready,
            pilots_now,
            strength,
        } => d
            .id("air_group", *air_group)
            .int("aircraft", *aircraft)
            .int("pilots", *pilots)
            .int("ready", *ready)
            .int("pilots_now", *pilots_now)
            .int("strength", *strength),
        EventKind::ReplacementsReceived {
            air_group,
            base,
            aircraft,
            pilots,
            ready,
            pilots_now,
        } => d
            .id("air_group", *air_group)
            .id("base", *base)
            .int("aircraft", *aircraft)
            .int("pilots", *pilots)
            .int("ready", *ready)
            .int("pilots_now", *pilots_now),
        EventKind::SearchAttempt {
            air_group,
            task_force,
            distance_nm,
            chance,
            sighted,
        } => d
            .id("air_group", *air_group)
            .id("task_force", *task_force)
            .num("distance_nm", *distance_nm)
            .num("chance", *chance)
            .flag("sighted", *sighted),
        EventKind::ReportReceived {
            side,
            contact,
            hex,
            ship_count,
            kind,
            created,
        } => d
            .text("side", names::side(*side))
            .int("contact", contact.umpire_raw(key))
            .hex("hex", *hex)
            .int("ship_count", *ship_count)
            .text("kind", kind.name())
            .flag("created", *created),
        EventKind::ContactAged {
            side,
            contact,
            age_pulses,
        } => d
            .text("side", names::side(*side))
            .int("contact", contact.umpire_raw(key))
            .int("age_pulses", *age_pulses),
        EventKind::ContactDropped {
            side,
            contact,
            age_pulses,
        } => d
            .text("side", names::side(*side))
            .int("contact", contact.umpire_raw(key))
            .int("age_pulses", *age_pulses),
        EventKind::EngagementBegun {
            a,
            b,
            hex,
            elapsed_hours,
        } => d
            .id("a", *a)
            .id("b", *b)
            .hex("hex", *hex)
            .num("elapsed_hours", *elapsed_hours),
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
            d.flag("sighted", *sighted)
        }
        EventKind::ForceSurprised {
            round,
            task_force,
            enemy,
        } => d
            .int("round", *round)
            .id("task_force", *task_force)
            .id("enemy", *enemy),
        EventKind::Salvo {
            round,
            firer,
            target,
            weapon,
            shots,
            chance,
            expected,
            hits,
        } => d
            .int("round", *round)
            .id("firer", *firer)
            .id("target", *target)
            .text("weapon", names::weapon(*weapon))
            .num("shots", *shots)
            .num("chance", *chance)
            .num("expected", *expected)
            .int("hits", *hits),
        EventKind::ShipDamaged {
            round,
            ship,
            track,
            hits,
            points,
            value_after,
        } => d
            .int("round", *round)
            .id("ship", *ship)
            .text("track", names::track(*track))
            .int("hits", *hits)
            .num("points", *points)
            .num("value_after", *value_after),
        EventKind::AircraftFireRoll {
            ship,
            task_force,
            bombs,
            aboard,
            flying,
            share,
            chance,
            draw,
            set,
            level_after,
        } => d
            .id("ship", *ship)
            .id("task_force", *task_force)
            .int("bombs", *bombs)
            .int("aboard", *aboard)
            .int("flying", *flying)
            .num("share", *share)
            .num("chance", *chance)
            .num("draw", *draw)
            .flag("set", *set)
            .int("level_after", u32::from(*level_after)),
        EventKind::AircraftFireFought {
            ship,
            task_force,
            level_before,
            draw,
            level_after,
            lost,
        } => d
            .id("ship", *ship)
            .id_when_some("task_force", *task_force)
            .int("level_before", u32::from(*level_before))
            .num("draw", *draw)
            .int("level_after", u32::from(*level_after))
            .flag("lost", *lost),
        EventKind::ShipSunk {
            round,
            ship,
            task_force,
        } => d
            .int("round", *round)
            .id("ship", *ship)
            .id("task_force", *task_force),
        EventKind::ForceBecameAlert { round, task_force } => {
            d.int("round", *round).id("task_force", *task_force)
        }
        EventKind::ForceBrokeOff {
            round,
            task_force,
            share,
        } => d
            .int("round", *round)
            .id("task_force", *task_force)
            .num("share", *share),
        EventKind::ForceSlippedAway {
            round,
            task_force,
            enemy,
        } => d
            .int("round", *round)
            .id("task_force", *task_force)
            .id("enemy", *enemy),
        EventKind::ForcesPassedUnseen { round, a, b } => {
            d.int("round", *round).id("a", *a).id("b", *b)
        }
        EventKind::EngagementEnded {
            round,
            a,
            b,
            hex,
            reason,
        } => d
            .int("round", *round)
            .id("a", *a)
            .id("b", *b)
            .hex("hex", *hex)
            .text("reason", names::end_reason(*reason)),
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
            from_submarine,
            ground_types,
            by_unit,
            air_types,
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
            // A submarine's claim (decision 0218, 45) says the hits believed
            // scored and the kind taken for, and is marked.
            let d = if *from_submarine {
                let d = d
                    .int("claimed_hits", *claimed_hits)
                    .flag("from_submarine", true);
                match claimed_kind {
                    Some(kind) => d.text("claimed_kind", kind.name()),
                    None => d,
                }
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
            // The types claimed on the ground (decision 0179) and who claims
            // what (decision 0180), each only where the claim has any, so a
            // claim without them exports as it did.
            let d = if ground_types.is_empty() {
                d
            } else {
                d.with(
                    "ground_types",
                    Field::Objs(
                        ground_types
                            .iter()
                            .map(|(aircraft_type, aircraft)| {
                                Data::new()
                                    .int("aircraft_type", aircraft_type.raw())
                                    .int("aircraft", *aircraft)
                            })
                            .collect(),
                    ),
                )
            };
            let d = if by_unit.is_empty() {
                d
            } else {
                d.with(
                    "claimed_by",
                    Field::Objs(
                        by_unit
                            .iter()
                            .map(|line| {
                                Data::new()
                                    .with("unit", Field::Str(line.unit.to_string()))
                                    .int("shot_down", line.shot_down)
                                    .int("on_ground", line.on_ground)
                                    .int("hits", line.hits)
                            })
                            .collect(),
                    ),
                )
            };
            // The types claimed shot down (decision 0210), only where the
            // claim has any, as the types on the ground are.
            let d = if air_types.is_empty() {
                d
            } else {
                d.with(
                    "air_types",
                    Field::Objs(
                        air_types
                            .iter()
                            .map(|(aircraft_type, aircraft)| {
                                Data::new()
                                    .int("aircraft_type", aircraft_type.raw())
                                    .int("aircraft", *aircraft)
                            })
                            .collect(),
                    ),
                )
            };
            d
        }
        EventKind::SystemHit {
            round,
            ship,
            system,
            before,
            after,
        } => d
            .int("round", *round)
            .id("ship", *ship)
            .text("system", names::system(*system))
            .text("before", names::system_state(*before))
            .text("after", names::system_state(*after)),
        EventKind::ShipsDetached {
            parent,
            new_force,
            ships,
            escorts,
            reason,
        } => d
            .id("parent", *parent)
            .id("new_force", *new_force)
            .ids("ships", ships)
            .ids("escorts", escorts)
            .text("reason", names::detach_reason(*reason)),
        EventKind::ForceRetired { task_force } => d.id("task_force", *task_force),
        EventKind::ForceAlertedByContact {
            task_force,
            contact,
        } => d
            .id("task_force", *task_force)
            .int("contact", contact.umpire_raw(key)),
        EventKind::ForceStoodDown { task_force } => d.id("task_force", *task_force),
        EventKind::TaskForceFatigue {
            task_force,
            change,
            amount,
            fatigue,
        } => d
            .id("task_force", *task_force)
            .text(
                "change",
                match change {
                    ForceFatigueChange::Gained => "gained",
                    ForceFatigueChange::Rested => "rested",
                },
            )
            .num("amount", *amount)
            .num("fatigue", *fatigue),
        EventKind::TaskForceTurnedBack {
            task_force,
            from,
            toward,
            progress_nm,
            position,
        } => d
            .id("task_force", *task_force)
            .hex("from", *from)
            .hex("toward", *toward)
            .num("progress_nm", *progress_nm)
            .hex("position", *position),
        EventKind::ShipRepaired {
            ship,
            task_force,
            base,
            points,
            fire_damage,
            flotation_damage,
            restored,
            system_repair_points,
        } => d
            .id("ship", *ship)
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
        EventKind::ForceLeftForYard {
            task_force,
            base,
            passage_days,
            ships,
            days_away,
            return_day,
        } => d
            .id("task_force", *task_force)
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
        EventKind::AircraftWrittenOff {
            air_group,
            base,
            aircraft,
        } => d
            .id("air_group", *air_group)
            .id("base", *base)
            .int("aircraft", *aircraft),
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
            match pilots {
                Some(pilots) => data.int("pilots", *pilots),
                None => data,
            }
        }
        EventKind::ForceReturnedFromYard {
            task_force,
            base,
            ships,
        } => d
            .id("task_force", *task_force)
            .id("base", *base)
            .ids("ships", ships),
        EventKind::BaseAtDayEnd {
            base,
            supply_t,
            aircraft_ready,
        } => d
            .id("base", *base)
            .num("supply_t", *supply_t)
            .int("aircraft_ready", *aircraft_ready)
            .flag(
                "open",
                sim_core::events::base_is_open(*supply_t, *aircraft_ready),
            ),
        EventKind::AlarmRoll {
            task_force,
            by_a,
            by_b,
            chance,
            alarmed,
        } => d
            .id("task_force", *task_force)
            .id("by_a", *by_a)
            .id("by_b", *by_b)
            .num("chance", *chance)
            .flag("alarmed", *alarmed),
        // Decision 0111.
        EventKind::TaskForceFormed {
            task_force,
            side: _,
            base,
            ships,
        } => d
            .id("task_force", *task_force)
            .id("base", *base)
            .ids("ships", ships),
        EventKind::TaskForceJoined {
            joined,
            into,
            base,
            ships,
        } => d
            .id("joined", *joined)
            .id("into", *into)
            .id("base", *base)
            .ids("ships", ships),
        EventKind::TaskForceStoodDownInPort {
            task_force,
            base,
            ships,
        } => d
            .id("task_force", *task_force)
            .id("base", *base)
            .ids("ships", ships),
        EventKind::LoadBelowReserve {
            task_force,
            base,
            cargo,
            below_t,
            reserve_t,
        } => cargo_named(d.id("task_force", *task_force).id("base", *base), *cargo)
            .num("below_t", *below_t)
            .num("reserve_t", *reserve_t),
        EventKind::RefuellingHeldAtReserve {
            ship,
            task_force,
            hex,
            short_t,
        } => d
            .id("ship", *ship)
            .id("task_force", *task_force)
            .hex("hex", *hex)
            .num("short_t", *short_t),
        EventKind::LoadHeldAtReserve {
            task_force,
            base,
            cargo,
            aboard_t,
            reserve_t,
        } => cargo_named(d.id("task_force", *task_force).id("base", *base), *cargo)
            .num("aboard_t", *aboard_t)
            .num("reserve_t", *reserve_t),
        EventKind::CommandTaken {
            side,
            issuer,
            command,
        } => d
            .text("side", names::side(*side))
            .text("issuer", issuer_code(*issuer))
            .text("command", command.name())
            .with("detail", Field::Str(command.to_string())),
        EventKind::CommandRefused {
            side,
            issuer,
            command,
            reason,
        } => d
            .text("side", names::side(*side))
            .text("issuer", issuer_code(*issuer))
            .text("command", command.name())
            .with("detail", Field::Str(command.to_string()))
            .with("reason", Field::Str(reason.to_string())),
        EventKind::PlanDrafted { side, plan, intent } => d
            .text("side", names::side(*side))
            .int("plan", plan.raw())
            .id("intent", *intent),
        EventKind::PlanStateChanged {
            side,
            plan,
            from,
            to,
        } => d
            .text("side", names::side(*side))
            .int("plan", plan.raw())
            .text("from", from.name())
            .text("to", to.name()),
        EventKind::PlanAmended { side, plan, number } => d
            .text("side", names::side(*side))
            .int("plan", plan.raw())
            .int("number", *number),
        EventKind::StaffReason {
            side,
            order,
            reason,
            ..
        } => {
            let mut data = d.text("side", names::side(*side));
            if let Some(order) = order {
                data = data.int("order", order.umpire_raw(key));
            }
            if let Some(intent) = reason.serves.intent {
                data = data.id("intent", intent);
            }
            if let Some(plan) = reason.serves.plan {
                data = data.int("plan", plan.raw());
            }
            if let Some(amendment) = reason.serves.amendment {
                data = data.int("amendment", amendment);
            }
            if !reason.serves.statement.is_empty() {
                data = data.with("statement", Field::Str(reason.serves.statement.clone()));
            }
            // The level that decided: "area" or "base:N"; left out where a
            // reason has none (decision 0120).
            if let Some(level) = reason.serves.level {
                let level = match level {
                    sim_core::command::Addressee::Area => "area".to_string(),
                    sim_core::command::Addressee::Base(base) => format!("base:{}", base.raw()),
                    sim_core::command::Addressee::AirGroup(group) => {
                        format!("air_group:{}", group.raw())
                    }
                    sim_core::command::Addressee::TaskForce(force) => {
                        format!("task_force:{}", force.raw())
                    }
                };
                data = data.with("level", Field::Str(level));
            }
            // The intents above it, nearest first: "1 (commander): hold
            // base:1; ...", left out where it is a root.
            if !reason.serves.chain.is_empty() {
                let chain: Vec<String> = reason
                    .serves
                    .chain
                    .iter()
                    .map(|link| {
                        format!(
                            "{} ({}): {}",
                            link.intent.raw(),
                            link.maker.name(),
                            link.statement
                        )
                    })
                    .collect();
                data = data.with("chain", Field::Str(chain.join("; ")));
            }
            let evidence: Vec<String> = reason
                .evidence
                .iter()
                .map(|piece| format!("{} (day {})", piece.what, piece.day))
                .collect();
            let mut data = data
                .with("evidence", Field::Str(evidence.join("; ")))
                .with("binding", Field::Str(reason.binding.join("; ")))
                .with("worked", Field::Str(reason.worked.clone()))
                .with("would_change", Field::Str(reason.would_change.join("; ")));
            // The force whose answer the decision rests on (decision 0214);
            // written only where there is one.
            if let Some(force) = reason.answer_of {
                data = data.id("answer_of", force);
            }
            // The cautions' words (decision 0217); written only where there
            // are any.
            if !reason.cautions.is_empty() {
                let cautions: Vec<&str> =
                    reason.cautions.iter().map(|c| c.words.as_str()).collect();
                data = data.with("cautions", Field::Str(cautions.join("; ")));
            }
            // The limits it speaks of (decision 0222, rule 5), each as its
            // kind and the intent it is set on ("forces on intent:3");
            // written only where there are any.
            if !reason.limits.is_empty() {
                let limits: Vec<String> = reason
                    .limits
                    .iter()
                    .map(|l| match (l.intent, l.base) {
                        (Some(intent), _) => format!("{} on {intent}", l.kind.name()),
                        (None, Some(base)) => format!("{} of {base}", l.kind.name()),
                        (None, None) => l.kind.name().to_string(),
                    })
                    .collect();
                data = data.with("limits", Field::Str(limits.join("; ")));
            }
            data
        }
        EventKind::HeldByOrder {
            side,
            target,
            order,
        } => d
            .text("side", names::side(*side))
            .with("target", Field::Str(target.to_string()))
            .int("order", order.umpire_raw(key)),
        EventKind::StaffNotice {
            side,
            kind,
            plan,
            run,
            intent,
            pursuit,
            follow,
        } => {
            let mut data = d
                .text("side", names::side(*side))
                .text("notice", kind.name())
                .int("plan", plan.raw());
            if let Some(run) = run {
                data = data.int("run", *run);
            }
            // The intent whose words a take-a-place's `met` notice says.
            if let Some(intent) = intent {
                data = data.int("intent", intent.raw());
            }
            // What a pursuit notice says (decision 0214); written only for
            // those, so every other export is as it was.
            if let Some(pursuit) = pursuit {
                let mut out = Data::new()
                    .id("task_force", pursuit.force)
                    .int("intent", pursuit.intent.raw());
                if let Some(find) = &pursuit.find {
                    let mut found = Data::new()
                        .int("contact", find.contact)
                        .text("kind", find.kind.name())
                        .int("age", find.age)
                        .hex("reported_at", find.reported_at)
                        .hex("to", find.to);
                    if let Some(hours) = find.hours {
                        found = found.num("hours", hours);
                    }
                    out = out.with("find", Field::Obj(found));
                }
                data = data.with("pursuit", Field::Obj(out));
            }
            // What a notice of following says (decision 0220); written only for
            // those, so every other export is as it was.
            if let Some(follow) = follow {
                let mut out = Data::new()
                    .int("intent", follow.intent.raw())
                    .int("contact", follow.contact)
                    .hex("hex", follow.hex)
                    .int("day", follow.day)
                    .text("pulse", follow.pulse.name())
                    .int("ship_count", follow.ship_count)
                    .text("kind", follow.kind.name())
                    .flag("carrier", follow.carrier)
                    .flag("other_sort", follow.other_sort);
                if let Some(from) = follow.taken_from {
                    out = out.int("taken_from", from);
                }
                data = data.with("follow", Field::Obj(out));
            }
            data
        }
        EventKind::InterruptFired {
            side,
            interrupt,
            condition,
            figure,
        } => {
            let mut data = d
                .text("side", names::side(*side))
                .with("interrupt", Field::Str(interrupt.to_string()));
            if let Some(condition) = condition {
                data = data.with("condition", Field::Str(condition.to_string()));
            }
            if let Some(figure) = figure {
                data = data.num("days_of_supply", *figure);
            }
            data
        }
        EventKind::IntentFormed {
            side,
            intent,
            parent,
        } => d
            .text("side", names::side(*side))
            .int("intent", intent.raw())
            .int("parent", parent.raw()),
        EventKind::IntentWithdrawn {
            side,
            intent,
            parent,
        } => d
            .text("side", names::side(*side))
            .int("intent", intent.raw())
            .int("parent", parent.raw()),
        EventKind::StationKept {
            escort,
            charge,
            in_station,
            waypoints,
            knots,
        } => {
            let data = d
                .id("escort", *escort)
                .id("charge", *charge)
                .flag("in_station", *in_station)
                .with("waypoints", Field::Hexes(waypoints.clone()));
            match knots {
                Some(knots) => data.num("knots", *knots),
                None => data,
            }
        }
        EventKind::ShipFuelledAtSea {
            ship,
            task_force,
            oiler,
            giver,
            tons,
            fuel_t,
            oiler_cargo_fuel_t,
            restored,
        } => d
            .id("ship", *ship)
            .id("task_force", *task_force)
            .id("oiler", *oiler)
            .id("giver", *giver)
            .num("tons", *tons)
            .num("fuel_t", *fuel_t)
            .num("oiler_cargo_fuel_t", *oiler_cargo_fuel_t)
            .flag("restored", *restored),
        EventKind::FuellingBrokenOff {
            task_force,
            giver,
            hex,
        } => d
            .id("task_force", *task_force)
            .id("giver", *giver)
            .hex("hex", *hex),
        EventKind::FuelOrderEnded {
            task_force,
            giver,
            reason,
        } => d
            .id("task_force", *task_force)
            .id("giver", *giver)
            .text("reason", reason.name()),
        EventKind::ChargeLost {
            escort,
            charge,
            hex,
        } => d
            .id("escort", *escort)
            .id("charge", *charge)
            .hex("hex", *hex),
        EventKind::MeetingNotHeld {
            charge,
            enemy,
            hex,
            escorts,
        } => d
            .id("charge", *charge)
            .id("enemy", *enemy)
            .hex("hex", *hex)
            .ids("escorts", escorts),
        EventKind::ReportSent {
            side,
            sender,
            report,
            listeners,
        } => d
            .text("side", names::side(*side))
            .with("sender", Field::Str(command_word(*sender)))
            .int("report", report.umpire_raw(key))
            .with(
                "listeners",
                Field::Objs(
                    listeners
                        .iter()
                        .map(|listener| report_listener(listener, at))
                        .collect(),
                ),
            ),
        // A command's state report late or lost on its way to the area command
        // (decision 0136, 6).
        EventKind::StateReportLate {
            side,
            command,
            as_of,
        } => d
            .text("side", names::side(*side))
            .with("command", Field::Str(command_word(*command)))
            .with("as_of", Field::Str(as_of.to_string())),
        EventKind::StateReportLost {
            side,
            command,
            as_of,
        } => d
            .text("side", names::side(*side))
            .with("command", Field::Str(command_word(*command)))
            .with("as_of", Field::Str(as_of.to_string())),
        // The picture a world was built from (decision 0170, 7): the day of
        // its evening.
        EventKind::BuiltFromPicture { day } => d.int("day", *day),
        EventKind::ShipRearmed {
            ship,
            task_force,
            base,
            rounds,
            torpedoes,
            taken_t,
            rounds_aboard,
            tubes_fired,
            base_ammunition_t,
        } => d
            .id("ship", *ship)
            .id("task_force", *task_force)
            .id("base", *base)
            .int("rounds", *rounds)
            .int("torpedoes", *torpedoes)
            .num("taken_t", *taken_t)
            .num("rounds_aboard", *rounds_aboard)
            .int("tubes_fired", *tubes_fired)
            .num("base_ammunition_t", *base_ammunition_t),
    }
}

/// A command as the export words it: `area`, `task_force:N` or `base:N`.
pub(super) fn command_word(command: sim_core::pictures::CommandRef) -> String {
    match command {
        sim_core::pictures::CommandRef::Area => "area".to_string(),
        sim_core::pictures::CommandRef::TaskForce(force) => format!("task_force:{}", force.raw()),
        sim_core::pictures::CommandRef::Base(base) => format!("base:{}", base.raw()),
    }
}

/// One listener of a passed report (replay version 6): the command, the
/// road (`up`, `sideways`, `down`), how it arrives (`on_time`, `late`,
/// `never`) and, unless never, the day and pulse it arrives, worked out here
/// so that the viewer need not. A report sent in an end-of-day phase (no
/// pulse) is counted as the afternoon's last word, as the rules count it
/// (decision 0135).
fn report_listener(listener: &ReportListener, at: (u32, Option<Pulse>)) -> Data {
    let (day, pulse) = at;
    let sent = pulse.unwrap_or(Pulse::Afternoon);
    let (arrival, when) = match listener.arrival {
        Arrival::OnTime => ("on_time", Some((day, sent))),
        Arrival::Late => ("late", Some(next_pulse(day, sent))),
        Arrival::Never => ("never", None),
    };
    let data = Data::new()
        .with("command", Field::Str(command_word(listener.command)))
        .text("direction", listener.direction.name())
        .text("arrival", arrival);
    match when {
        Some((day, pulse)) => data.int("day", day).text("pulse", pulse.name()),
        None => data,
    }
}

fn next_pulse(day: u32, pulse: Pulse) -> (u32, Pulse) {
    match pulse {
        Pulse::Night => (day, Pulse::Morning),
        Pulse::Morning => (day, Pulse::Afternoon),
        Pulse::Afternoon => (day + 1, Pulse::Night),
    }
}

/// An issuer's code in the replay export.
fn issuer_code(issuer: sim_core::orders::Issuer) -> &'static str {
    match issuer {
        sim_core::orders::Issuer::Player => "player",
        sim_core::orders::Issuer::Staff => "staff",
        sim_core::orders::Issuer::LocalCommander => "local_commander",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sim_core::events::Direction;
    use sim_core::pictures::CommandRef;

    fn listener(arrival: Arrival) -> ReportListener {
        ReportListener {
            command: CommandRef::Area,
            direction: Direction::Up,
            arrival,
        }
    }

    fn text(data: &Data) -> String {
        serde_json::to_string(data).expect("data serialises")
    }

    /// An arrival is the moment of sending (on time), the next pulse (a
    /// pulse late, which from the afternoon is the next day's night), or no
    /// moment at all (never). A report sent in an end-of-day phase has no
    /// pulse and is counted as the afternoon's.
    #[test]
    fn a_listeners_arrival_is_the_pulse_it_is_counted_from() {
        let at = |arrival, day, pulse| text(&report_listener(&listener(arrival), (day, pulse)));
        assert_eq!(
            at(Arrival::OnTime, 2, Some(Pulse::Morning)),
            r#"{"command":"area","direction":"up","arrival":"on_time","day":2,"pulse":"morning"}"#
        );
        assert_eq!(
            at(Arrival::Late, 2, Some(Pulse::Night)),
            r#"{"command":"area","direction":"up","arrival":"late","day":2,"pulse":"morning"}"#
        );
        assert_eq!(
            at(Arrival::Late, 2, Some(Pulse::Morning)),
            r#"{"command":"area","direction":"up","arrival":"late","day":2,"pulse":"afternoon"}"#
        );
        assert_eq!(
            at(Arrival::Late, 2, Some(Pulse::Afternoon)),
            r#"{"command":"area","direction":"up","arrival":"late","day":3,"pulse":"night"}"#
        );
        assert_eq!(
            at(Arrival::Late, 2, None),
            r#"{"command":"area","direction":"up","arrival":"late","day":3,"pulse":"night"}"#
        );
        assert_eq!(
            at(Arrival::OnTime, 2, None),
            r#"{"command":"area","direction":"up","arrival":"on_time","day":2,"pulse":"afternoon"}"#
        );
        assert_eq!(
            at(Arrival::Never, 2, Some(Pulse::Night)),
            r#"{"command":"area","direction":"up","arrival":"never"}"#
        );
    }
}
