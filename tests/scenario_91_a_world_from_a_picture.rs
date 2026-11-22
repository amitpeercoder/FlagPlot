//! Scenario 91: a world built from a picture. A picture holds everything a
//! world needs on an evening; `World::from_picture` builds a world from
//! it, at that day, with the belief, the orders, the command state and the
//! counters the picture gives.
//!
//! Proves decision 0170, step 3 of step 3's stage 4, against a hand
//! calculation. It has no scenario file of its own: its pictures are made
//! in the test, from the data of scenario 1 and from live games of
//! scenarios 1 and 86. It runs on the frozen step 1 figures.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## 1. By hand: scenario 1's force, from a picture of day 4
//!
//! The picture, made in the test from scenario 1's map, reference data,
//! bases and ships, with nothing taken from a game that was played:
//!
//! - **The day:** 4.
//! - **The state:** base 1, Noumea, at (0,0) and base 2, Espiritu Santo,
//!   at (8,3), both Allied. Task force 1, Allied, on the transport
//!   mission, at base 1, with a route of one waypoint, (8,3), and the
//!   speed cruise; its route refers to order 1 and its speed to order 2.
//!   Ship 1, class 1, 2,000 t of fuel, and ship 2, class 2, 500 t, both
//!   in task force 1. No air group and no land unit.
//! - **The orders in force:** 1, Allied, a route for task force 1 to
//!   (8,3); 2, Allied, the speed cruise for task force 1. Both issued on
//!   day 1 by the commander.
//! - **Belief:** the Allied side holds one contact, 3 ships taken for
//!   cruisers at (4,2), 2 pulses old; one claim, at (4,2) on day 3 in the
//!   afternoon, of 1 hit on a cruiser from the air and nothing else; and
//!   one estimate, 200 men at (8,0), of day 2, seen from the air. The
//!   Japanese side holds nothing.
//! - **Command state:** empty for both sides, the next plan 1.
//! - **The counters:** Allied 1,000,003 for a force and 1,000,001 for an
//!   air group; Japanese 2,000,001 for each.
//! - **The stream:** seed 1942, trial 1, world 1.
//!
//! The world as built, before any day is resolved:
//!
//! - Its day is 4. It has no day in hand and no daily hash.
//! - Its log holds one event, event 1, on day 4 with no pulse: the world
//!   was built from a picture. Each side sees it.
//! - Its order book holds orders 1 and 2, as the picture gave them, both
//!   as issued on day 1.
//! - Task force 1 is at (0,0) with its route and speed, citing orders 1
//!   and 2. Ship 1 has 2,000 t and ship 2 has 500 t.
//! - The Allied side's belief: contact 1, 3 cruisers at (4,2), 2 pulses
//!   old, its last report event 1; the claim, as given, received by
//!   event 1; the estimate, 200 men at (8,0), of day 2, seen from the
//!   air, reported by event 1. The Japanese side's belief is empty.
//! - The next force the Allied side makes would be 1,000,003.
//!
//! No order is given. Days 5, 6 and 7 are resolved. The steaming is
//! scenario 1's (see its hand calculation: 16 kn, 394 nm to (8,3), the
//! cruiser burning 0.25 t a mile and the destroyer 0.125 t), with day 5
//! in the place of its day 1. Day d begins at hour 24 x (d - 1).
//!
//! | Day | Pulse     | Miles steamed | Position |
//! | --- | --------- | ------------- | -------- |
//! | 5   | Night     | 192           | (5,0)    |
//! | 5   | Morning   | 288           | (8,0)    |
//! | 5   | Afternoon | 384           | (8,3)    |
//! | 6   | Night     | 394, arrived  | (8,3)    |
//!
//! - **The end of day 5:** the world's day is 5. Ship 1 has 2,000 - 96 =
//!   1,904 t and ship 2 has 500 - 48 = 452 t.
//! - **Day 6:** the force arrives 0.625 hours into the night pulse, at
//!   hour 24 x 5 + 0.625 = 120.625. Ship 1 has 1,904 - 2.5 = 1,901.5 t
//!   and ship 2 has 452 - 1.25 = 450.75 t.
//! - **Day 7:** nothing moves and no fuel is burned. Ship 1 has 1,901.5 t
//!   and ship 2 has 450.75 t at the end.
//! - **The arrival's causes** lead back to order 1, the route, as
//!   scenario 1's do.
//! - **The first event of day 5** is event 2.
//! - **Three daily hashes** are kept, for days 5, 6 and 7.
//! - **The contact** is 2 + 3 = 5 pulses old at the end of day 5, where
//!   scenario 1's search figures still hold a contact of that age; where
//!   they drop it sooner, it is dropped on the pulse they give, as a
//!   contact of that age in a game from day 0 is. (The build reads the
//!   figure and asserts the pulse; it is not a number of this rule.)
//!
//! ## 2. Nothing lost: a world rebuilt from its own whole picture
//!
//! - **Scenario 1.** The live game is played to the evening of day 1.
//!   The umpire takes its whole picture and builds a world from it on
//!   the game's own seed with no domain. Both are resolved for days 2
//!   and 3. At the end of each of those days the built world's ships,
//!   task forces, bases and air groups are the live game's, field for
//!   field, but for the numbers of orders and events they cite. The
//!   built world's first event of day 2 is event 2.
//! - **Scenario 86,** which has a staff, a command state, land units, a
//!   landing and a base formed in play. The live game is played to the
//!   evening of day 3, each side's staff on doctrine. The umpire takes
//!   its whole picture and builds a world from it on the game's own seed
//!   with no domain. Both are played on to day 8 with the same staffs.
//!   At the end of each of days 4 to 8 the built world's ships, task
//!   forces, bases and land units are the live game's, field for field,
//!   but for the numbers of orders and events they cite; each side's
//!   command state is the live game's, but for those numbers; and the
//!   staff gives the built world the orders it gives the live game, in
//!   kind, target and day. At the end the bases hold what scenario 86's
//!   hand calculation gives: base 1 915 t, base 2 70 t, base 5 10 t.
//! - **A picture taken twice is the same picture,** and two worlds built
//!   from one picture on one stream have the same hash on every day.
//!
//! ## 3. A stream of its own
//!
//! - **A live game's draws are what they were.** Every pinned hash and
//!   run baseline of the repository is unchanged by the build.
//! - **The domain moves a draw.** For one key (seed 1942, day 5, the
//!   night pulse, one subsystem, one entity), the first draw with no
//!   domain, with trial 1 and world 1, with trial 1 and world 2, and
//!   with trial 2 and world 1 are four different numbers; and each is
//!   the same asked twice.
//!
//! ## 4. What it refuses
//!
//! Each is the picture of case 1 with one change, and each is refused
//! with words that name the fault. No world is built.
//!
//! | The change | Refused because |
//! | --- | --- |
//! | Task force 1's route refers to order 3 | an order referred to is not in the picture |
//! | A task force 1,000,003, Allied, is in the state | a unit made in play has a number at or above its side's counter |
//! | The Allied counter of forces is 1,000,000 | a counter is below its side's first number |
//! | The contact has 0 ships | a contact has no ship |
//! | The contact is at a hex off the map | a contact is off the map |
//! | The claim is of day 5 | a claim is after the picture's day |
//! | The estimate is of day 5 | an estimate is after the picture's day |
//! | A second estimate at (8,0) | two estimates for one hex |
//! | Ship 2's class is one the reference data does not have | what `World::new` refuses of it |
//! | The Allied command state holds plan 1 and its next plan is 1 | the next plan is not above the plans held |
//!
//! # Properties
//!
//! - **`World::new` is as it was.** Every scenario of the repository
//!   loads and plays as before: no baseline, golden file or pinned hash
//!   changes.
//! - **The picture is not state.** A world built from a picture and
//!   resolved holds nothing that refers to the picture: dropping the
//!   picture changes nothing, and the world's hash does not read it.
//! - **Nothing that plays can rebuild.** The whole picture of a live
//!   world and a build with no domain both ask for the umpire's key.
//!   The test that reads the agent's source still passes.

mod common;

use std::collections::{BTreeMap, BTreeSet};

use sim_core::belief::GroundGrade;
use sim_core::command::{CommandState, Plan, PlanId, PlanState, StaffDecisions};
use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{EntityRef, IntentId, ShipClassId, ShipId, TaskForceId};
use sim_core::map::Map;
use sim_core::orders::{OrderKind, Origin, SpeedSetting, TaskForceOrder};
use sim_core::refdata::{RefData, ShipKind};
use sim_core::rng::{Domain, Draw, RngKey, Stream, Subsystem};
use sim_core::side_events::SideEventKind;
use sim_core::state::{Route, Side, TaskForceStatus, TruthState};
use sim_core::time::Pulse;
use sim_core::world_picture::{
    PictureBelief, PictureClaim, PictureContact, PictureCounters, PictureEstimate, PictureOrder,
    WorldPicture,
};
use sim_core::{Staffs, World};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

const TASK_FORCE: TaskForceId = TaskForceId::new(1);
const CRUISER: ShipId = ShipId::new(1);
const DESTROYER: ShipId = ShipId::new(2);
const STREAM: Stream = Stream {
    seed: 1942,
    domain: Domain { trial: 1, world: 1 },
};

fn scenario(name: &str) -> Scenario {
    common::load(&common::data_dir().join("scenarios").join(name)).expect("the scenario loads")
}

fn tons(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn raw_event(event: &Event) -> u32 {
    event.id.umpire_raw(common::umpire_key())
}

/// Case 1's picture, made from scenario 1's map, reference data, bases and
/// ships as it loads, before any day is played: nothing is taken from a
/// game that was played.
fn case_one() -> (WorldPicture, RefData, Map) {
    let loaded = scenario("01_transit.toml");
    let world = loaded.world;
    let mut truth: TruthState = world.umpire().clone();
    assert_eq!(truth.day, 0, "nothing of scenario 1 is played");
    truth.day = 4;
    let force = truth
        .task_forces
        .get_mut(&TASK_FORCE)
        .expect("task force 1");
    force.route = Route {
        waypoints: vec![Hex::new(8, 3)],
        next_index: 0,
        act_at: BTreeSet::new(),
        stand_at: BTreeSet::new(),
    };
    force.status = TaskForceStatus::Underway;
    force.speed = SpeedSetting::Cruise;
    force.route_order = Some(WorldPicture::order_at(1));
    force.speed_order = Some(WorldPicture::order_at(2));
    assert_eq!(truth.ships[&CRUISER].fuel_t, tons(2000));
    assert_eq!(truth.ships[&DESTROYER].fuel_t, tons(500));
    assert!(truth.air_groups.is_empty() && truth.land_units.is_empty());

    let allied = PictureBelief {
        contacts: vec![PictureContact {
            hex: Hex::new(4, 2),
            ship_count: 3,
            kind: ShipKind::Cruiser,
            age_pulses: 2,
        }],
        claims: vec![PictureClaim {
            hex: Hex::new(4, 2),
            day: 3,
            pulse: Pulse::Afternoon,
            claimed_sunk: 0,
            claimed_hits: 1,
            claimed_kind: Some(ShipKind::Cruiser),
            claimed_shot_down: 0,
            claimed_on_ground: 0,
            from_air: true,
            from_bombardment: false,
            from_submarine: false,
            ground_types: Vec::new(),
            by_unit: Vec::new(),
            air_types: Vec::new(),
        }],
        estimates: vec![PictureEstimate {
            hex: Hex::new(8, 0),
            men: 200,
            day: 2,
            grade: GroundGrade::SeenFromAir,
        }],
        submarine_reports: Vec::new(),
        suspected_waters: Vec::new(),
    };
    let picture = WorldPicture {
        day: 4,
        truth,
        beliefs: BTreeMap::from([
            (Side::Allied, allied),
            (Side::Japanese, PictureBelief::default()),
        ]),
        command: BTreeMap::from([
            (Side::Allied, CommandState::new()),
            (Side::Japanese, CommandState::new()),
        ]),
        orders: vec![
            PictureOrder {
                side: Side::Allied,
                day_issued: 1,
                kind: route_order(),
                origin: Origin::PLAYER,
            },
            PictureOrder {
                side: Side::Allied,
                day_issued: 1,
                kind: speed_order(),
                origin: Origin::PLAYER,
            },
        ],
        counters: BTreeMap::from([
            (
                Side::Allied,
                PictureCounters {
                    next_task_force: 1_000_003,
                    next_air_group: 1_000_001,
                },
            ),
            (
                Side::Japanese,
                PictureCounters {
                    next_task_force: 2_000_001,
                    next_air_group: 2_000_001,
                },
            ),
        ]),
        objectives: None,
        // Decision 0194, 11: a picture may carry the game's days; this one,
        // made by hand, carries none, as before.
        game_days: None,
    };
    (picture, world.ref_data().clone(), world.map().clone())
}

fn route_order() -> OrderKind {
    OrderKind::TaskForce {
        target: TASK_FORCE,
        order: TaskForceOrder::route(vec![Hex::new(8, 3)]),
    }
}

fn speed_order() -> OrderKind {
    OrderKind::TaskForce {
        target: TASK_FORCE,
        order: TaskForceOrder::SetSpeed(SpeedSetting::Cruise),
    }
}

fn built() -> World {
    let (picture, ref_data, map) = case_one();
    World::from_picture(picture, ref_data, map, STREAM).expect("case 1's picture is sound")
}

// ---------------------------------------------------------------------------
// 1. By hand.
// ---------------------------------------------------------------------------

#[test]
fn case_one_as_built_before_any_day() {
    let key = common::umpire_key();
    let world = built();

    // Its day is 4, with no day in hand and no daily hash.
    assert_eq!(world.day(), 4);
    assert_eq!(world.day_in_hand(), None);
    assert!(world.day_hashes().is_empty());
    assert_eq!(world.seed(), 1942);
    assert_eq!(world.domain(), Some(Domain { trial: 1, world: 1 }));

    // One event, event 1, on day 4 with no pulse: the picture.
    let log: Vec<&Event> = world.events().iter().collect();
    assert_eq!(log.len(), 1);
    assert_eq!(raw_event(log[0]), 1);
    assert_eq!(log[0].day, 4);
    assert_eq!(log[0].pulse, None);
    assert_eq!(log[0].kind, EventKind::BuiltFromPicture { day: 4 });
    assert!(log[0].causes.is_empty());
    // Each side sees it.
    for side in Side::ALL {
        let seen = world.side_view(side).events();
        assert_eq!(seen.len(), 1, "{side:?}");
        assert_eq!(seen[0].id, log[0].id);
        assert!(matches!(
            seen[0].kind,
            SideEventKind::Whole(EventKind::BuiltFromPicture { day: 4 })
        ));
    }

    // The order book: orders 1 and 2, as given, both of day 1.
    let orders: Vec<_> = world.orders().collect();
    assert_eq!(orders.len(), 2);
    assert_eq!(orders[0].id.umpire_raw(key), 1);
    assert_eq!(orders[0].kind, route_order());
    assert_eq!(orders[1].id.umpire_raw(key), 2);
    assert_eq!(orders[1].kind, speed_order());
    for order in &orders {
        assert_eq!(order.side, Side::Allied);
        assert_eq!(order.day_issued, 1);
        assert_eq!(order.origin, Origin::PLAYER);
    }

    // Task force 1 at (0,0), its route and speed citing orders 1 and 2.
    let truth = world.umpire();
    let force = &truth.task_forces[&TASK_FORCE];
    assert_eq!(force.position(), Hex::new(0, 0));
    assert_eq!(force.route.waypoints, vec![Hex::new(8, 3)]);
    assert_eq!(force.status, TaskForceStatus::Underway);
    assert_eq!(force.speed, SpeedSetting::Cruise);
    assert_eq!(force.route_order.map(|o| o.umpire_raw(key)), Some(1));
    assert_eq!(force.speed_order.map(|o| o.umpire_raw(key)), Some(2));
    assert_eq!(truth.ships[&CRUISER].fuel_t, tons(2000));
    assert_eq!(truth.ships[&DESTROYER].fuel_t, tons(500));

    // The Allied side's belief, every entry reported by event 1.
    let allied = world.side_view(Side::Allied);
    let contacts: Vec<_> = allied.contacts().collect();
    assert_eq!(contacts.len(), 1);
    assert_eq!(allied.contact_number(contacts[0].id), 1);
    assert_eq!(contacts[0].estimated_hex, Hex::new(4, 2));
    assert_eq!(contacts[0].estimated_ship_count, 3);
    assert_eq!(contacts[0].estimated_kind, ShipKind::Cruiser);
    assert_eq!(contacts[0].age_pulses, 2);
    assert_eq!(contacts[0].last_report_event, log[0].id);
    let claims = allied.claims();
    assert_eq!(claims.len(), 1);
    let claim = &claims[0];
    assert_eq!(
        (claim.hex, claim.day, claim.pulse),
        (Hex::new(4, 2), 3, Pulse::Afternoon)
    );
    assert_eq!(
        (
            claim.claimed_sunk,
            claim.claimed_hits,
            claim.claimed_kind,
            claim.claimed_shot_down,
            claim.claimed_on_ground,
            claim.from_air,
            claim.from_bombardment
        ),
        (0, 1, Some(ShipKind::Cruiser), 0, 0, true, false)
    );
    assert_eq!(claim.received_event, log[0].id);
    let estimates: Vec<_> = allied.ground_estimates().collect();
    assert_eq!(estimates.len(), 1);
    assert_eq!(
        (estimates[0].hex, estimates[0].men, estimates[0].day),
        (Hex::new(8, 0), 200, 2)
    );
    assert_eq!(estimates[0].grade, GroundGrade::SeenFromAir);
    assert_eq!(estimates[0].report_event, log[0].id);

    // The Japanese side's belief is empty.
    let japanese = world.side_view(Side::Japanese);
    assert_eq!(japanese.contacts().count(), 0);
    assert!(japanese.claims().is_empty());
    assert_eq!(japanese.ground_estimates().count(), 0);

    // The next force the Allied side makes would be 1,000,003.
    assert_eq!(allied.next_task_force_id(), TaskForceId::new(1_000_003));

    // The command states are empty, each its next plan 1.
    for side in Side::ALL {
        assert_eq!(world.command_state(side), &CommandState::new());
    }
}

/// The steaming of days 5 to 7: scenario 1's, with day 5 in the place of
/// its day 1.
#[test]
fn case_one_steams_as_scenario_one_does() {
    let key = common::umpire_key();
    let mut world = built();
    let ref_data = world.ref_data().clone();

    // Day 5.
    let day5 = world.resolve_day().expect("day 5 resolves");
    assert_eq!(day5.day, 5);
    // The first event of day 5 is event 2.
    assert_eq!(day5.first_event, 1);
    assert_eq!(
        raw_event(&world.events().from_index(day5.first_event)[0]),
        2
    );
    let steamed = |world: &World, day: u32| -> Vec<(Pulse, Fixed, Hex)> {
        world
            .events()
            .iter()
            .filter(|event| event.day == day)
            .filter_map(|event| match &event.kind {
                EventKind::TaskForceSteamed {
                    task_force,
                    distance_nm,
                    to,
                    ..
                } if *task_force == TASK_FORCE => {
                    Some((event.pulse.expect("a pulse"), *distance_nm, *to))
                }
                _ => None,
            })
            .collect()
    };
    // Night 192 nm to (5,0); morning 96 nm, 288 in all, to (8,0);
    // afternoon 96 nm, 384 in all, to (8,3).
    assert_eq!(
        steamed(&world, 5),
        vec![
            (Pulse::Night, tons(192), Hex::new(5, 0)),
            (Pulse::Morning, tons(96), Hex::new(8, 0)),
            (Pulse::Afternoon, tons(96), Hex::new(8, 3)),
        ]
    );
    assert_eq!(world.day(), 5);
    {
        let truth = world.umpire();
        assert_eq!(truth.task_forces[&TASK_FORCE].position(), Hex::new(8, 3));
        assert_eq!(truth.ships[&CRUISER].fuel_t, tons(1904));
        assert_eq!(truth.ships[&DESTROYER].fuel_t, tons(452));
    }

    // The contact: 2 pulses old in the picture, a pulse older at the end
    // of each pulse with no fresh report. Scenario 1's search figures drop
    // a contact that reaches 5 pulses: the figure is read, and the contact
    // is dropped on the pulse at which it reaches it, the afternoon of day
    // 5, aged 5, as a contact of that age in a game from day 0 is.
    let drop_age = ref_data
        .search
        .as_ref()
        .expect("scenario 1 has search figures")
        .contact_drop_age_pulses;
    assert_eq!(drop_age, 5);
    let contact_events: Vec<(Pulse, u32, bool, Vec<Cause>)> = world
        .events()
        .iter()
        .filter(|event| event.day == 5)
        .filter_map(|event| match &event.kind {
            EventKind::ContactAged {
                side: Side::Allied,
                age_pulses,
                ..
            } => Some((
                event.pulse.expect("a pulse"),
                *age_pulses,
                false,
                event.causes.clone(),
            )),
            EventKind::ContactDropped {
                side: Side::Allied,
                age_pulses,
                ..
            } => Some((
                event.pulse.expect("a pulse"),
                *age_pulses,
                true,
                event.causes.clone(),
            )),
            _ => None,
        })
        .collect();
    let first = world.events().iter().next().expect("event 1").id;
    assert_eq!(
        contact_events,
        vec![
            (Pulse::Night, 3, false, vec![Cause::Event(first)]),
            (Pulse::Morning, 4, false, vec![Cause::Event(first)]),
            (Pulse::Afternoon, 5, true, vec![Cause::Event(first)]),
        ]
    );
    assert_eq!(world.side_view(Side::Allied).contacts().count(), 0);
    // The claim and the estimate never age.
    assert_eq!(world.side_view(Side::Allied).claims().len(), 1);
    assert_eq!(world.side_view(Side::Allied).ground_estimates().count(), 1);

    // Day 6: 10 nm remain; the force arrives 0.625 hours into the night,
    // at hour 120.625.
    let day6 = world.resolve_day().expect("day 6 resolves");
    assert_eq!(day6.day, 6);
    assert_eq!(
        steamed(&world, 6),
        vec![(Pulse::Night, tons(10), Hex::new(8, 3))]
    );
    let hour = Fixed::from_ratio(965, 8);
    let arrival = world
        .events()
        .iter()
        .find(|event| matches!(event.kind, EventKind::TaskForceArrived { .. }))
        .expect("the force arrived")
        .clone();
    assert_eq!((arrival.day, arrival.pulse), (6, Some(Pulse::Night)));
    assert_eq!(
        arrival.kind,
        EventKind::TaskForceArrived {
            task_force: TASK_FORCE,
            hex: Hex::new(8, 3),
            elapsed_hours: hour,
        }
    );
    {
        let truth = world.umpire();
        let force = &truth.task_forces[&TASK_FORCE];
        assert_eq!(force.status, TaskForceStatus::Arrived);
        assert_eq!(force.arrived_at_hours, Some(hour));
        assert_eq!(truth.ships[&CRUISER].fuel_t, Fixed::from_ratio(3803, 2));
        assert_eq!(truth.ships[&DESTROYER].fuel_t, Fixed::from_ratio(1803, 4));
    }

    // Day 7: nothing moves and no fuel is burned.
    let day7 = world.resolve_day().expect("day 7 resolves");
    assert_eq!(day7.day, 7);
    assert!(world
        .events()
        .from_index(day7.first_event)
        .iter()
        .all(|event| !matches!(
            event.kind,
            EventKind::TaskForceSteamed { .. } | EventKind::FuelBurned { .. }
        )));
    {
        let truth = world.umpire();
        assert_eq!(truth.ships[&CRUISER].fuel_t, Fixed::from_ratio(3803, 2));
        assert_eq!(truth.ships[&DESTROYER].fuel_t, Fixed::from_ratio(1803, 4));
    }

    // Three daily hashes, for days 5, 6 and 7.
    assert_eq!(world.day_hashes().len(), 3);
    assert_eq!(
        world.day_hashes(),
        [day5.state_hash, day6.state_hash, day7.state_hash]
    );

    // The arrival's causes lead back to order 1, the route, as scenario
    // 1's do: through the movement to the two standing orders.
    let [Cause::Event(steamed_id)] = arrival.causes[..] else {
        panic!("the arrival is caused by one event: {:?}", arrival.causes);
    };
    assert!(matches!(
        world.events().get(steamed_id).expect("in the log").kind,
        EventKind::TaskForceSteamed { .. }
    ));
    let orders: Vec<u32> = world
        .events()
        .trace(arrival.id)
        .into_iter()
        .filter_map(|cause| match cause {
            Cause::Order(id) => Some(id.umpire_raw(key)),
            Cause::Event(_) => None,
        })
        .collect();
    assert_eq!(orders, vec![1, 2]);
    let route = world.orders().next().expect("order 1");
    assert_eq!(route.kind, route_order());
    assert!(world
        .events()
        .involving(EntityRef::TaskForce(TASK_FORCE))
        .all(|event| event.day >= 5));
}

// ---------------------------------------------------------------------------
// 2. Nothing lost.
// ---------------------------------------------------------------------------

/// A unit's fields in words. An order's and an event's number show as
/// `_` outside the engine, so two units that differ only in the numbers of
/// the orders and events they cite read the same.
fn words<T: std::fmt::Debug>(value: &T) -> String {
    format!("{value:?}")
}

fn same_units(live: &World, built: &World, day: u32, land_units: bool) {
    let (a, b) = (live.umpire(), built.umpire());
    assert_eq!(a.day, b.day, "day {day}");
    assert_eq!(words(&a.ships), words(&b.ships), "the ships, day {day}");
    assert_eq!(
        words(&a.task_forces),
        words(&b.task_forces),
        "the task forces, day {day}"
    );
    assert_eq!(words(&a.bases), words(&b.bases), "the bases, day {day}");
    assert_eq!(
        words(&a.air_groups),
        words(&b.air_groups),
        "the air groups, day {day}"
    );
    if land_units {
        assert_eq!(
            words(&a.land_units),
            words(&b.land_units),
            "the land units, day {day}"
        );
    }
}

#[test]
fn scenario_one_rebuilt_from_its_whole_picture_plays_on_as_it_did() {
    let key = common::umpire_key();
    let mut live = scenario("01_transit.toml");
    live.next_day().expect("day 1").expect("day 1 exists");
    let picture = live.world.whole_picture(key).expect("an evening");
    // A picture taken twice is the same picture.
    assert_eq!(picture, live.world.whole_picture(key).expect("an evening"));
    assert_eq!(picture.day, 1);
    assert_eq!(picture.orders.len(), 2);
    let mut rebuilt = World::rebuild_from_picture(
        picture,
        live.world.ref_data().clone(),
        live.world.map().clone(),
        live.world.seed(),
        key,
    )
    .expect("the whole picture is sound");
    assert_eq!(rebuilt.domain(), None);
    same_units(&live.world, &rebuilt, 1, true);

    for day in 2..=3 {
        live.next_day().expect("a day").expect("a day exists");
        let summary = rebuilt.resolve_day().expect("a day");
        assert_eq!(summary.day, day);
        if day == 2 {
            // The built world's first event of day 2 is event 2.
            assert_eq!(summary.first_event, 1);
            assert_eq!(
                raw_event(&rebuilt.events().from_index(summary.first_event)[0]),
                2
            );
        }
        same_units(&live.world, &rebuilt, day, true);
    }
}

/// Scenario 86, played by each side's staff on doctrine: stopped on the
/// evening of day 3, rebuilt by the umpire from its whole picture on its
/// own seed, and both played on to day 8 with the same staffs.
#[test]
fn scenario_eighty_six_rebuilt_from_its_whole_picture_plays_on_as_it_did() {
    let key = common::umpire_key();
    let mut live = scenario("86_fed_with_the_lift.toml");
    let held = staffs();
    for _ in 1..=3 {
        live.next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day")
            .expect("a day exists");
    }
    let picture = live.world.whole_picture(key).expect("an evening");
    assert_eq!(picture, live.world.whole_picture(key).expect("an evening"));
    assert_eq!(picture.day, 3);
    let mut rebuilt = World::rebuild_from_picture(
        picture,
        live.world.ref_data().clone(),
        live.world.map().clone(),
        live.world.seed(),
        key,
    )
    .expect("the whole picture is sound");
    same_units(&live.world, &rebuilt, 3, true);
    let held_built = staffs();
    let staff_words = |world: &World| -> String {
        words(
            &world
                .staff_handed_in()
                .iter()
                .map(|(side, decisions)| (*side, decisions.clone()))
                .collect::<BTreeMap<Side, StaffDecisions>>(),
        )
    };

    for day in 4..=8 {
        live.next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day")
            .expect("a day exists");
        let staffs_built: Staffs<'_> = as_staffs(&held_built);
        let summary = rebuilt.resolve_day_staffed(staffs_built).expect("a day");
        assert_eq!(summary.day, day);
        same_units(&live.world, &rebuilt, day, true);
        for side in Side::ALL {
            assert_eq!(
                words(live.world.command_state(side)),
                words(rebuilt.command_state(side)),
                "the {side:?} command state, day {day}"
            );
        }
        // The staff gives the built world the orders it gives the live
        // game, in kind, target and day.
        assert_eq!(
            staff_words(&live.world),
            staff_words(&rebuilt),
            "what the staffs handed in, day {day}"
        );
        // Days 4 and 5 are the days the staff orders the run (one order on
        // day 4, the load; two on day 5, the route and the unload).
        let given = rebuilt
            .staff_handed_in()
            .get(&Side::Allied)
            .map_or(0, |decisions| decisions.orders.len());
        assert_eq!(
            given,
            match day {
                4 => 1,
                5 => 2,
                _ => 0,
            },
            "the staff's orders, day {day}"
        );
    }

    // The bases hold what scenario 86's hand calculation gives.
    let supply = |base: u32| rebuilt.umpire().bases[&sim_core::ids::BaseId::new(base)].supply_t;
    assert_eq!(supply(1), tons(915));
    assert_eq!(supply(2), tons(70));
    assert_eq!(supply(5), tons(10));
}

/// Two worlds built from one picture on one stream have the same hash on
/// every day; the picture is not state, so dropping it changes nothing.
#[test]
fn one_picture_on_one_stream_gives_one_world() {
    let (picture, ref_data, map) = case_one();
    let mut kept =
        World::from_picture(picture.clone(), ref_data.clone(), map.clone(), STREAM).expect("sound");
    let mut other = World::from_picture(picture.clone(), ref_data, map, STREAM).expect("sound");
    drop(picture);
    assert_eq!(kept.state_hash(), other.state_hash());
    for _ in 5..=7 {
        let a = kept.resolve_day().expect("a day");
        let b = other.resolve_day().expect("a day");
        assert_eq!(a.state_hash, b.state_hash);
    }
    assert_eq!(kept.day_hashes(), other.day_hashes());
}

// ---------------------------------------------------------------------------
// 3. A stream of its own.
// ---------------------------------------------------------------------------

#[test]
fn the_domain_moves_a_draw() {
    let first = |domain: Option<Domain>| {
        RngKey {
            seed: 1942,
            day: 5,
            pulse: Some(Pulse::Night),
            subsystem: Subsystem::Search,
            entity: EntityRef::TaskForce(TASK_FORCE),
            other: None,
            draw: Draw::Legacy,
            domain,
        }
        .stream()
        .next_u64()
    };
    let draws = [
        first(None),
        first(Some(Domain { trial: 1, world: 1 })),
        first(Some(Domain { trial: 1, world: 2 })),
        first(Some(Domain { trial: 2, world: 1 })),
    ];
    for (i, a) in draws.iter().enumerate() {
        for b in &draws[i + 1..] {
            assert_ne!(a, b);
        }
    }
    assert_eq!(draws[0], first(None));
    assert_eq!(draws[1], first(Some(Domain { trial: 1, world: 1 })));
    assert_eq!(draws[2], first(Some(Domain { trial: 1, world: 2 })));
    assert_eq!(draws[3], first(Some(Domain { trial: 2, world: 1 })));
}

/// A world built from a picture draws in its domain, and the umpire's
/// rebuild on the live seed draws what the live game draws: the domain is
/// the only difference in their hashes.
#[test]
fn a_built_world_hashes_its_domain_and_a_rebuild_does_not() {
    let key = common::umpire_key();
    let (picture, ref_data, map) = case_one();
    let in_domain =
        World::from_picture(picture.clone(), ref_data.clone(), map.clone(), STREAM).expect("sound");
    let rebuilt =
        World::rebuild_from_picture(picture, ref_data, map, STREAM.seed, key).expect("sound");
    assert_eq!(rebuilt.domain(), None);
    assert_ne!(in_domain.state_hash(), rebuilt.state_hash());
}

// ---------------------------------------------------------------------------
// 4. What it refuses.
// ---------------------------------------------------------------------------

fn refused(change: impl FnOnce(&mut WorldPicture)) -> String {
    let (mut picture, ref_data, map) = case_one();
    change(&mut picture);
    match World::from_picture(picture, ref_data, map, STREAM) {
        Ok(_) => panic!("the picture was not refused"),
        Err(error) => error.to_string(),
    }
}

fn says(words: &str, fault: &str) {
    assert!(
        words.contains(fault),
        "refused with {words:?}, not {fault:?}"
    );
}

fn allied(picture: &mut WorldPicture) -> &mut PictureBelief {
    picture
        .beliefs
        .get_mut(&Side::Allied)
        .expect("the Allied belief")
}

#[test]
fn an_order_referred_to_must_be_in_the_picture() {
    let words = refused(|p| {
        p.truth
            .task_forces
            .get_mut(&TASK_FORCE)
            .unwrap()
            .route_order = Some(WorldPicture::order_at(3));
    });
    says(&words, "task_force:1's route order refers to order 3");
    says(&words, "not in the picture's list of 2 orders");
}

#[test]
fn a_unit_made_in_play_must_be_below_its_sides_counter() {
    let words = refused(|p| {
        let mut made = p.truth.task_forces[&TASK_FORCE].clone();
        made.id = TaskForceId::new(1_000_003);
        made.ships = BTreeSet::new();
        made.route_order = None;
        made.speed_order = None;
        made.status = TaskForceStatus::Idle;
        made.route = Route::default();
        p.truth.task_forces.insert(made.id, made);
    });
    says(
        &words,
        "task_force:1000003, made in play, has a number at or above the Allied counter of task \
         forces, 1000003",
    );
}

#[test]
fn a_counter_may_not_be_below_its_sides_first_number() {
    let words = refused(|p| {
        p.counters.get_mut(&Side::Allied).unwrap().next_task_force = 1_000_000;
    });
    says(
        &words,
        "the Allied counter of task forces, 1000000, is below its side's first number, 1000001",
    );
}

#[test]
fn a_contact_must_have_a_ship() {
    let words = refused(|p| allied(p).contacts[0].ship_count = 0);
    says(&words, "the Allied contact 1 has no ship");
}

#[test]
fn a_contact_must_be_on_the_map() {
    let words = refused(|p| allied(p).contacts[0].hex = Hex::new(500, 500));
    says(&words, "the Allied contact 1 is at (500,500), off the map");
}

#[test]
fn a_claim_may_not_be_after_the_pictures_day() {
    let words = refused(|p| allied(p).claims[0].day = 5);
    says(
        &words,
        "the Allied claim 1 is of day 5, after the picture's day, 4",
    );
}

#[test]
fn an_estimate_may_not_be_after_the_pictures_day() {
    let words = refused(|p| allied(p).estimates[0].day = 5);
    says(
        &words,
        "the Allied estimate at (8,0) is of day 5, after the picture's day, 4",
    );
}

#[test]
fn a_belief_holds_one_estimate_for_a_hex() {
    let words = refused(|p| {
        let again = allied(p).estimates[0];
        allied(p)
            .estimates
            .push(PictureEstimate { men: 300, ..again });
    });
    says(&words, "the Allied side holds two estimates for (8,0)");
}

#[test]
fn what_world_new_refuses_a_picture_is_refused() {
    let words = refused(|p| {
        p.truth.ships.get_mut(&DESTROYER).unwrap().class = ShipClassId::new(99);
    });
    says(&words, "ship:2 has unknown class ship_class:99");
}

#[test]
fn the_next_plan_must_be_above_the_plans_held() {
    let plan = Plan {
        id: PlanId::new(1),
        intent: IntentId::new(1),
        state: PlanState::Draft,
        baseline: None,
        amendments: Vec::new(),
        runs: BTreeMap::new(),
    };
    let words = refused(|p| {
        let state = p.command.get_mut(&Side::Allied).unwrap();
        state.plans.insert(plan.id, plan.clone());
        state.next_plan = 1;
    });
    says(
        &words,
        "the Allied next plan, 1, is not above plan:1, which it holds",
    );
}
