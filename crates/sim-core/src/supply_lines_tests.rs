//! Supply lines (decision 0196): stock from off the map, in a world built
//! by hand. The world is the open sea of `pictures_tests`, with its Allied
//! base 1 and Japanese base 2.

use crate::events::{Event, EventKind};
use crate::fixed::Fixed;
use crate::hash::hash_world;
use crate::ids::BaseId;
use crate::invariants::{check, InvariantViolation};
use crate::pictures_tests::{world as test_world, BASE1};
use crate::side_events::SideEventKind;
use crate::state::{Side, SupplyLine};
use crate::time::Pulse;
use crate::world::World;

const BASE2: BaseId = BaseId::new(2);

fn t(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn line(side: Side, base: BaseId, supply: &str, fuel: &str, first: u32, last: u32) -> SupplyLine {
    SupplyLine {
        side,
        base,
        supply_t: t(supply),
        fuel_t: t(fuel),
        first_day: first,
        last_day: last,
    }
}

fn stock_events(world: &World) -> Vec<Event> {
    world
        .events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::StockReceived { .. }))
        .cloned()
        .collect()
}

fn stock(world: &World, base: BaseId) -> (Fixed, Fixed) {
    let b = &world.truth.bases[&base];
    (b.supply_t, b.fuel_t)
}

/// A line of 30 t of supply and 12.5 t of fuel on days 3 to 6, to an Allied
/// base: on day 3 it adds exactly those, and records one `StockReceived`
/// with no cause. On days 2 and 7 it gives nothing and records nothing. If
/// the base is the other side's it gives nothing and records nothing. Two
/// lines to one base both give, one event each, in the state's order. Day 1
/// works, unlike the replacements, which begin on day 2.
#[test]
fn a_line_gives_on_its_days_to_a_base_of_its_side_and_records_one_event_each() {
    let mut world = test_world();
    world
        .truth
        .supply_lines
        .push(line(Side::Allied, BASE1, "30", "12.5", 3, 6));
    let before = stock(&world, BASE1);
    world.receive_supply_lines(2, Pulse::Night);
    assert_eq!(stock(&world, BASE1), before);
    assert!(stock_events(&world).is_empty());
    world.receive_supply_lines(3, Pulse::Night);
    assert_eq!(
        stock(&world, BASE1),
        (before.0 + Fixed::from_int(30), before.1 + t("12.5"))
    );
    let events = stock_events(&world);
    assert_eq!(events.len(), 1);
    assert!(events[0].causes.is_empty());
    assert_eq!((events[0].day, events[0].pulse), (3, Some(Pulse::Night)));
    assert_eq!(
        events[0].kind,
        EventKind::StockReceived {
            base: BASE1,
            side: Side::Allied,
            supply_t: Fixed::from_int(30),
            fuel_t: t("12.5"),
        }
    );
    let after = stock(&world, BASE1);
    world.receive_supply_lines(7, Pulse::Night);
    assert_eq!(stock(&world, BASE1), after);
    assert_eq!(stock_events(&world).len(), 1);

    // The base changes side: the line gives nothing and keeps nothing.
    world.truth.bases.get_mut(&BASE1).expect("base").side = Side::Japanese;
    world.receive_supply_lines(4, Pulse::Night);
    assert_eq!(stock(&world, BASE1), after);
    assert_eq!(stock_events(&world).len(), 1);

    // Two lines to one base, both giving, in the state's order; and day 1.
    let mut two = test_world();
    two.truth
        .supply_lines
        .push(line(Side::Allied, BASE1, "5", "0", 1, 1));
    two.truth
        .supply_lines
        .push(line(Side::Allied, BASE1, "7", "1", 1, 2));
    let start = stock(&two, BASE1);
    two.receive_supply_lines(1, Pulse::Night);
    assert_eq!(
        stock(&two, BASE1),
        (start.0 + Fixed::from_int(12), start.1 + Fixed::ONE)
    );
    let events = stock_events(&two);
    assert_eq!(events.len(), 2);
    let given: Vec<Fixed> = events
        .iter()
        .map(|e| match e.kind {
            EventKind::StockReceived { supply_t, .. } => supply_t,
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(given, vec![Fixed::from_int(5), Fixed::from_int(7)]);
}

/// In resolved days, night N's `StockReceived` is recorded in the night
/// pulse of day N, before the day's end record of the bases, which then
/// holds the tons: with 0 held at the start and a line of 4 t of supply and
/// 1.5 t of fuel on days 1 to 2, the base holds 4 t at the end of day 1 and
/// 8 t at the end of day 2 (no aircraft or men draw on it in this world;
/// the ships in port refuel from the fuel, so only the supply is exact).
/// The line draws no random number: another seed gives the same events and
/// stock.
#[test]
fn a_resolved_day_gives_at_night_before_the_day_end_and_draws_no_random_number() {
    let build = |seed: u64| {
        let source = test_world();
        let mut truth = source.truth.clone();
        truth
            .supply_lines
            .push(line(Side::Allied, BASE1, "4", "1.5", 1, 2));
        World::new(seed, source.ref_data.clone(), source.map.clone(), truth).expect("valid")
    };
    let mut world = build(1);
    let start = stock(&world, BASE1);
    world.resolve_day().expect("day 1");
    let day_end = |world: &World, day: u32| {
        world
            .events
            .iter()
            .find(|e| {
                e.day == day
                    && matches!(e.kind, EventKind::BaseAtDayEnd { base, .. } if base == BASE1)
            })
            .expect("a day-end record")
            .clone()
    };
    let first = stock_events(&world);
    assert_eq!(first.len(), 1);
    assert_eq!((first[0].day, first[0].pulse), (1, Some(Pulse::Night)));
    assert!(first[0].id < day_end(&world, 1).id);
    match day_end(&world, 1).kind {
        EventKind::BaseAtDayEnd { supply_t, .. } => {
            assert_eq!(supply_t, start.0 + Fixed::from_int(4));
        }
        _ => unreachable!(),
    }
    world.resolve_day().expect("day 2");
    world.resolve_day().expect("day 3");
    let all = stock_events(&world);
    assert_eq!(
        all.iter().map(|e| (e.day, e.pulse)).collect::<Vec<_>>(),
        vec![(1, Some(Pulse::Night)), (2, Some(Pulse::Night))]
    );
    // The fuel is drawn on by the ships in port, which refuel from it
    // (their tanks are empty in this world), so only the supply is exact.
    assert_eq!(stock(&world, BASE1).0, start.0 + Fixed::from_int(8));
    let mut other = build(99);
    for _ in 0..3 {
        other.resolve_day().expect("resolves");
    }
    assert_eq!(stock(&other, BASE1), stock(&world, BASE1));
    let kinds = |w: &World| {
        stock_events(w)
            .into_iter()
            .map(|e| (e.day, e.pulse, e.kind))
            .collect::<Vec<_>>()
    };
    assert_eq!(kinds(&other), kinds(&world));
}

/// A world with no line hashes as it did; one line changes the hash; and
/// lines that differ only in fuel, or only in last day, hash differently.
#[test]
fn supply_lines_are_hashed_only_where_there_are_some() {
    let mut world = test_world();
    let none = hash_world(&world);
    world.truth.supply_lines.clear();
    assert_eq!(hash_world(&world), none);
    let hashed = |fuel: &str, last: u32| {
        let mut w = test_world();
        w.truth
            .supply_lines
            .push(line(Side::Allied, BASE1, "30", fuel, 3, last));
        hash_world(&w)
    };
    let base = hashed("12.5", 6);
    assert_ne!(base, none);
    assert_ne!(hashed("12.75", 6), base);
    assert_ne!(hashed("12.5", 7), base);
    assert_eq!(hashed("12.5", 6), base);
}

/// A side's view gives the side its own lines only: a line of the Allied
/// side to a Japanese-held base is the Allied side's, not the Japanese
/// side's.
#[test]
fn a_side_view_gives_a_side_its_own_lines_only() {
    let mut world = test_world();
    let own = line(Side::Allied, BASE1, "1", "0", 1, 2);
    let held_by_enemy = line(Side::Allied, BASE2, "2", "0", 1, 2);
    let japanese = line(Side::Japanese, BASE2, "3", "0", 1, 2);
    world.truth.supply_lines = vec![own, held_by_enemy, japanese];
    let allied: Vec<SupplyLine> = world
        .side_view(Side::Allied)
        .supply_lines()
        .copied()
        .collect();
    assert_eq!(allied, vec![own, held_by_enemy]);
    let jp: Vec<SupplyLine> = world
        .side_view(Side::Japanese)
        .supply_lines()
        .copied()
        .collect();
    assert_eq!(jp, vec![japanese]);
}

/// A line to an unknown base, or with its last day before its first, or
/// giving nothing, is unsound; a sound line is not.
#[test]
fn an_unsound_supply_line_is_an_invariant_violation() {
    let violations = |l: SupplyLine| {
        let mut w = test_world();
        w.truth.supply_lines.push(l);
        check(&w.truth, &w.map, &w.ref_data)
    };
    let unsound = |v: &[InvariantViolation]| {
        v.iter()
            .any(|x| matches!(x, InvariantViolation::SupplyLineUnsound(_)))
    };
    assert!(!unsound(&violations(line(
        Side::Allied,
        BASE1,
        "30",
        "0",
        3,
        6
    ))));
    assert!(!unsound(&violations(line(
        Side::Allied,
        BASE1,
        "30",
        "0",
        3,
        3
    ))));
    assert!(unsound(&violations(line(
        Side::Allied,
        BaseId::new(99),
        "30",
        "0",
        3,
        6
    ))));
    assert!(unsound(&violations(line(
        Side::Allied,
        BASE1,
        "30",
        "0",
        6,
        3
    ))));
    assert!(unsound(&violations(line(
        Side::Allied,
        BASE1,
        "0",
        "0",
        1,
        3
    ))));
}

/// The line's side sees the `StockReceived` whole; the other side's events
/// do not hold it at all.
#[test]
fn only_the_lines_side_sees_the_stock_received() {
    let mut world = test_world();
    world
        .truth
        .supply_lines
        .push(line(Side::Allied, BASE1, "30", "12.5", 1, 1));
    world.receive_supply_lines(1, Pulse::Night);
    let allied = world.side_view(Side::Allied).events();
    let seen: Vec<_> = allied
        .iter()
        .filter(|e| {
            matches!(
                &e.kind,
                SideEventKind::Whole(EventKind::StockReceived { .. })
            )
        })
        .collect();
    assert_eq!(seen.len(), 1);
    assert!(seen[0].causes.is_empty());
    let japanese = world.side_view(Side::Japanese).events();
    assert!(japanese.iter().all(|e| !matches!(
        &e.kind,
        SideEventKind::Whole(EventKind::StockReceived { .. })
    )));
    assert!(japanese.is_empty());
}
