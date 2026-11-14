//! Tests of a beaten side falling back (decision 0104) on hand-built worlds.
//! The figures are the first ones: `beaten_fatigue` 30, a march of 3 days to
//! a hex, 10 fatigue a day marched, 5 a day rested.

use crate::events::{Cause, EventKind};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{BaseId, LandUnitId, OrderId};
use crate::map::{HexData, HexKind, Map};
use crate::objectives::tallies;
use crate::orders::{Activity, LandUnitOrder, OrderKind, OrderRequest, Origin, Posture};
use crate::refdata::{GroundFigures, PortSize, RefData};
use crate::state::{Base, LandUnit, March, Side, TruthState};
use crate::world::World;

const P: Hex = Hex::new(0, 0);
const Q: Hex = Hex::new(0, 1);
const R: Hex = Hex::new(1, 0);
/// Linked to nothing.
const S: Hex = Hex::new(10, 0);

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn lu(id: u32) -> LandUnitId {
    LandUnitId::new(id)
}

fn base(id: u32, side: Side, hex: Hex, supply: i64) -> Base {
    Base {
        supply_t: n(supply),
        ..Base::new(BaseId::new(id), format!("Base {id}"), side, hex)
    }
}

fn unit(id: u32, side: Side, men: u32, hex: Hex, posture: Posture, working: bool) -> LandUnit {
    LandUnit {
        posture,
        working,
        activity: if working {
            Activity::Work(None)
        } else {
            Activity::Ready
        },
        ..LandUnit::new(lu(id), format!("Unit {id}"), side, men, Some(hex))
    }
}

/// Coast hexes P, Q, R and S, with land links P to Q and P to R. Unit 1 is
/// a Japanese working unit of 300 men at P with Japanese base 1 and Allied
/// base 3; unit 2 is an Allied fighting unit of 500 men at P on attack.
/// `tweak` changes the state before the world is built.
fn world(tweak: impl FnOnce(&mut TruthState)) -> World {
    let mut map = Map::new();
    for q in -1..=11 {
        for r in -1..=2 {
            map.insert(Hex::new(q, r), HexData::open_sea());
        }
    }
    for hex in [P, Q, R, S] {
        map.insert(
            hex,
            HexData {
                kind: HexKind::Coast,
                ..HexData::open_sea()
            },
        );
    }
    map.add_land_link(P, Q).expect("link");
    map.add_land_link(P, R).expect("link");
    let mut ref_data = RefData::default();
    ref_data.port_sizes.insert(
        0,
        PortSize {
            berths: 0,
            berth_rate_t_per_h: Fixed::ZERO,
            lighter_rate_t_per_h: n(20),
        },
    );
    ref_data.ground = Some(GroundFigures::FIRST);
    ref_data.search = Some(crate::refdata::SearchFigures::from_search_toml());
    let mut truth = TruthState::default();
    for base in [
        base(1, Side::Japanese, P, 1000),
        base(2, Side::Japanese, R, 100),
        base(3, Side::Allied, P, 1000),
    ] {
        truth.bases.insert(base.id, base);
    }
    for unit in [
        unit(1, Side::Japanese, 300, P, Posture::Defend, true),
        unit(2, Side::Allied, 500, P, Posture::Attack, false),
    ] {
        truth.land_units.insert(unit.id, unit);
    }
    tweak(&mut truth);
    World::new(1942, ref_data, map, truth).expect("valid world")
}

fn events(world: &World) -> Vec<(u32, EventKind)> {
    world
        .events()
        .iter()
        .map(|event| (event.id.raw(), event.kind.clone()))
        .collect()
}

fn fell_back(world: &World, id: u32) -> Vec<EventKind> {
    events(world)
        .into_iter()
        .map(|(_, kind)| kind)
        .filter(|kind| matches!(kind, EventKind::UnitFellBack { unit, .. } if *unit == lu(id)))
        .collect()
}

fn unit_of(world: &World, id: u32) -> &LandUnit {
    &world.truth.land_units[&lu(id)]
}

fn march_order(target: u32, to: Hex) -> OrderRequest {
    OrderRequest {
        kind: OrderKind::LandUnit {
            target: lu(target),
            order: LandUnitOrder::March(to),
        },
        origin: Origin::PLAYER,
    }
}

fn posture_order(target: u32, posture: Posture) -> OrderRequest {
    OrderRequest {
        kind: OrderKind::LandUnit {
            target: lu(target),
            order: LandUnitOrder::SetPosture {
                posture,
                when_spent: None,
            },
        },
        origin: Origin::PLAYER,
    }
}

#[test]
fn hex_order_puts_q_before_r() {
    assert!(Q < R);
}

#[test]
fn a_base_of_its_side_is_preferred_to_an_earlier_empty_hex() {
    let mut world = world(|_| {});
    world.resolve_day().expect("day 1");
    // Q comes first and holds nothing; R holds base 2.
    let fell = fell_back(&world, 1);
    assert_eq!(fell.len(), 1);
    let EventKind::UnitFellBack {
        side,
        from,
        to,
        men,
        fatigue,
        ..
    } = fell[0].clone()
    else {
        unreachable!()
    };
    assert_eq!((side, from, to, men), (Side::Japanese, P, R, 300));
    assert_eq!(fatigue, n(30));
    let unit = unit_of(&world, 1);
    assert_eq!(unit.fatigue, n(30));
    assert_eq!(unit.posture, Posture::Defend);
    let march = unit.march.expect("a march");
    assert_eq!((march.to, march.days), (R, 0));
    assert!(matches!(march.cause, Cause::Event(_)));
    // The event cites the test that beat the side.
    let beaten = events(&world)
        .into_iter()
        .find(|(_, kind)| matches!(kind, EventKind::SideBeatenAshore { .. }))
        .expect("beaten")
        .0;
    let event = world
        .events()
        .iter()
        .find(|event| matches!(event.kind, EventKind::UnitFellBack { .. }))
        .expect("event");
    assert_eq!(
        event.causes,
        vec![Cause::Event(crate::ids::EventId::new(beaten))]
    );
    // The march's cause is that event.
    assert_eq!(march.cause, Cause::Event(event.id));
}

#[test]
fn it_carries_nothing_and_forms_no_base() {
    let mut world = world(|_| {});
    for _ in 0..4 {
        world.resolve_day().expect("a day");
    }
    let unit = unit_of(&world, 1);
    assert_eq!(unit.hex, Some(R));
    assert!(unit.march.is_none());
    let arrived = events(&world)
        .into_iter()
        .find_map(|(_, kind)| match kind {
            EventKind::MarchArrived {
                unit,
                supply_t,
                base,
                ..
            } if unit == lu(1) => Some((supply_t, base)),
            _ => None,
        })
        .expect("arrived");
    assert_eq!(arrived, (Fixed::ZERO, None));
    assert!(!events(&world)
        .iter()
        .any(|(_, kind)| matches!(kind, EventKind::MarchBaseFormed { .. })));
    // Base 2 holds what it held; base 1, taken, was not drawn on.
    assert_eq!(world.truth.bases.len(), 3);
    // 30 at the defeat, then 10 for each of three days marched less nothing
    // rested: 60 on the day of arrival.
    assert_eq!(unit.fatigue, n(60));
}

#[test]
fn with_no_base_it_goes_to_the_first_hex_with_no_enemy_fighting_men() {
    // No Japanese base at R; an Allied fighting unit at Q; an Allied working
    // unit at R: Q is passed over, R's working men do not count.
    let mut world = world(|truth| {
        truth.bases.remove(&BaseId::new(2));
        for unit in [
            unit(5, Side::Allied, 100, Q, Posture::Defend, false),
            unit(6, Side::Allied, 100, R, Posture::Defend, true),
        ] {
            truth.land_units.insert(unit.id, unit);
        }
    });
    world.resolve_day().expect("day 1");
    let unit = unit_of(&world, 1);
    assert_eq!(unit.march.expect("a march").to, R);
}

#[test]
fn with_enemy_fighting_men_in_every_linked_hex_the_unit_surrenders() {
    let mut world = world(|truth| {
        truth.bases.remove(&BaseId::new(2));
        for unit in [
            unit(5, Side::Allied, 100, Q, Posture::Defend, false),
            unit(6, Side::Allied, 100, R, Posture::Defend, false),
        ] {
            truth.land_units.insert(unit.id, unit);
        }
    });
    world.resolve_day().expect("day 1");
    assert!(fell_back(&world, 1).is_empty());
    let surrendered: Vec<_> = events(&world)
        .into_iter()
        .filter(|(_, kind)| matches!(kind, EventKind::UnitSurrendered { .. }))
        .collect();
    assert_eq!(surrendered.len(), 1);
    assert!(matches!(
        surrendered[0].1,
        EventKind::UnitSurrendered {
            unit,
            side: Side::Japanese,
            hex: P,
            men: 300
        } if unit == lu(1)
    ));
    let unit = unit_of(&world, 1);
    assert_eq!((unit.men_ashore, unit.hex), (0, None));
}

#[test]
fn a_unit_on_an_island_of_one_hex_surrenders_destroyed_and_the_men_are_lost() {
    let mut world = world(|truth| {
        for base in [
            base(7, Side::Japanese, S, 1000),
            base(8, Side::Allied, S, 1000),
        ] {
            truth.bases.insert(base.id, base);
        }
        for unit in [
            unit(6, Side::Japanese, 200, S, Posture::Defend, true),
            unit(7, Side::Allied, 100, S, Posture::Attack, false),
        ] {
            truth.land_units.insert(unit.id, unit);
        }
    });
    world.resolve_day().expect("day 1");
    let log = events(&world);
    let at = log
        .iter()
        .position(|(_, kind)| {
            matches!(kind, EventKind::UnitSurrendered { unit, men: 200, .. } if *unit == lu(6))
        })
        .expect("surrendered");
    assert!(matches!(
        log[at + 1].1,
        EventKind::LandUnitDestroyed { unit } if unit == lu(6)
    ));
    assert_eq!(unit_of(&world, 6).men_ashore, 0);
    // The men count as men lost on the objectives scale: for the Allied
    // side, the 200 of unit 6 (and nothing is counted for the Japanese).
    let tallies = tallies(&world.truth, world.events(), world.ref_data());
    assert_eq!(tallies[&Side::Allied].men_lost, 200);
    assert_eq!(tallies[&Side::Japanese].men_lost, 0);
}

#[test]
fn the_fatigue_of_defeat_is_capped_at_100() {
    let mut world = world(|truth| {
        truth.land_units.get_mut(&lu(1)).expect("unit 1").fatigue = n(90);
    });
    world.resolve_day().expect("day 1");
    // Rested to 85 in the day's step, then 30 more, to 100 at most.
    assert_eq!(unit_of(&world, 1).fatigue, n(100));
}

#[test]
fn a_unit_already_marching_is_left_as_it_is() {
    let mut world = world(|truth| {
        truth.land_units.get_mut(&lu(1)).expect("unit 1").march = Some(March {
            to: Q,
            days: 0,
            counted: Fixed::ZERO,
            cause: Cause::Order(OrderId::new(1)),
        });
    });
    world.resolve_day().expect("day 1");
    assert!(fell_back(&world, 1).is_empty());
    let unit = unit_of(&world, 1);
    let march = unit.march.expect("still marching");
    assert_eq!((march.to, march.cause), (Q, Cause::Order(OrderId::new(1))));
    // It marched a day and gained no fatigue of defeat.
    assert_eq!(unit.fatigue, n(10));
}

#[test]
fn a_march_order_the_next_day_replaces_the_falling_back() {
    let mut world = world(|_| {});
    world.resolve_day().expect("day 1");
    assert_eq!(unit_of(&world, 1).march.expect("march").to, R);
    world.submit_orders(Side::Japanese, vec![march_order(1, Q)]);
    world.resolve_day().expect("day 2");
    let march = unit_of(&world, 1).march.expect("march");
    assert_eq!((march.to, march.days), (Q, 1));
    assert!(matches!(march.cause, Cause::Order(_)));
}

#[test]
fn the_enemy_is_told_nothing_of_a_falling_back_and_the_unit_of_a_surrender() {
    let mut world = world(|truth| {
        for base in [
            base(7, Side::Japanese, S, 1000),
            base(8, Side::Allied, S, 1000),
        ] {
            truth.bases.insert(base.id, base);
        }
        for unit in [
            unit(6, Side::Japanese, 200, S, Posture::Defend, true),
            unit(7, Side::Allied, 100, S, Posture::Attack, false),
        ] {
            truth.land_units.insert(unit.id, unit);
        }
    });
    world.resolve_day().expect("day 1");
    let allied = world.side_view(Side::Allied).events();
    let japanese = world.side_view(Side::Japanese).events();
    let text = |events: &[crate::side_events::SideEvent]| -> Vec<String> {
        events.iter().map(|event| event.kind.to_string()).collect()
    };
    let (allied, japanese) = (text(&allied), text(&japanese));
    // The fall back of unit 1 is the Japanese side's only.
    assert!(japanese.iter().any(|line| line.contains("fell back")));
    assert!(!allied.iter().any(|line| line.contains("fell back")));
    // The surrender: the Japanese side sees its unit, the Allied side the
    // hex and the men.
    assert!(japanese
        .iter()
        .any(|line| line.contains("land_unit:6") && line.contains("surrendered")));
    assert!(allied
        .iter()
        .any(|line| line == "200 enemy men surrendered at (10,0)"));
    assert!(!allied.iter().any(|line| line.contains("land_unit:6")));
}

#[test]
fn a_fighting_unit_beaten_in_a_fight_falls_back_on_the_defence_and_an_attack_order_is_taken_while_it_marches(
) {
    // Unit 1 is a fighting unit of 1,000 men against 6,000 on attack: the
    // Japanese side is beaten whatever the dice give.
    let mut world = world(|truth| {
        let unit = truth.land_units.get_mut(&lu(1)).expect("unit 1");
        unit.working = false;
        unit.activity = Activity::Ready;
        unit.men_ashore = 1000;
        truth.land_units.get_mut(&lu(2)).expect("unit 2").men_ashore = 6000;
    });
    world.resolve_day().expect("day 1");
    let unit = unit_of(&world, 1);
    assert!(unit.men_ashore > 0, "the dice left it men");
    assert_eq!(fell_back(&world, 1).len(), 1);
    assert_eq!(unit.march.expect("march").to, R);
    assert_eq!(unit.posture, Posture::Defend);
    // The countermand of its posture: taken, and the march goes on.
    world.submit_orders(Side::Japanese, vec![posture_order(1, Posture::Attack)]);
    world.resolve_day().expect("day 2");
    let unit = unit_of(&world, 1);
    assert_eq!(unit.posture, Posture::Attack);
    let march = unit.march.expect("the march goes on");
    assert_eq!((march.to, march.days), (R, 1));
    assert!(matches!(march.cause, Cause::Event(_)));
}

/// What the Allied side is told on day 1 when the Japanese units in S, a hex
/// linked to nothing, are beaten and surrender, whatever their number.
fn allied_view_of_a_surrender(japanese_men: &[u32]) -> Vec<String> {
    let mut world = world(|truth| {
        for base in [
            base(7, Side::Japanese, S, 1000),
            base(8, Side::Allied, S, 1000),
        ] {
            truth.bases.insert(base.id, base);
        }
        for (i, men) in japanese_men.iter().enumerate() {
            let unit = unit(
                6 + u32::try_from(i).unwrap(),
                Side::Japanese,
                *men,
                S,
                Posture::Defend,
                true,
            );
            truth.land_units.insert(unit.id, unit);
        }
        let attacker = unit(20, Side::Allied, 100, S, Posture::Attack, false);
        truth.land_units.insert(attacker.id, attacker);
    });
    world.resolve_day().expect("day 1");
    let surrendered: u32 = events(&world)
        .iter()
        .filter_map(|(_, kind)| match kind {
            EventKind::UnitSurrendered { men, .. } => Some(*men),
            _ => None,
        })
        .sum();
    assert_eq!(surrendered, japanese_men.iter().sum::<u32>());
    world
        .side_view(Side::Allied)
        .events()
        .iter()
        .map(|event| event.kind.to_string())
        .collect()
}

#[test]
fn the_enemy_is_told_of_a_surrender_once_whatever_the_number_of_units() {
    let one = allied_view_of_a_surrender(&[400]);
    let two = allied_view_of_a_surrender(&[200, 200]);
    assert_eq!(one, two);
    let told: Vec<&String> = one
        .iter()
        .filter(|line| line.contains("surrendered"))
        .collect();
    assert_eq!(told.len(), 1);
    assert_eq!(told[0], "400 enemy men surrendered at (10,0)");
}
