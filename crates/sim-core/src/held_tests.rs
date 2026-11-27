//! What an area command holds of the commands under it (decision 0136, steps
//! 1 to 3), in the world of `pictures_tests` (Allied task force 1 at (0,0),
//! task force 2 at (1,0) and base 1 at (0,0); Japanese task force 9 and base
//! 2). Every chance in a test is 0 or 1, so no outcome rests on a die.
//! Scenario 63 (`tests/scenario_63_a_superiors_knowledge.rs`) holds the proof
//! that a whole game plays as the rules say.

use crate::events::EventKind;
use crate::fixed::Fixed;
use crate::held::{AsOf, HeldState};
use crate::orders::{OrderKind, OrderRequest, Origin, TaskForceOrder};
use crate::pictures::CommandRef;
use crate::pictures_tests::{hx, tf, world_with_levels, BASE1, TF1, TF2};
use crate::refdata::{PassingChances, ReportFigures};
use crate::state::Side;
use crate::time::Pulse;
use crate::world::World;

fn chances(on_time: i64, late: i64) -> PassingChances {
    PassingChances {
        on_time: Fixed::from_int(on_time),
        late: Fixed::from_int(late),
    }
}

/// A world with command levels on, report figures whose chance of a state
/// report (the `up` chances) are as given, and holding begun.
fn holding(up: (i64, i64)) -> World {
    let mut world = world_with_levels(&[]);
    world.ref_data.reports = Some(ReportFigures {
        up: chances(up.0, up.1),
        sideways: chances(1, 0),
        down: chances(1, 0),
        reach_carrier_nm: Fixed::from_int(600),
        reach_battleship_nm: Fixed::from_int(600),
        reach_cruiser_nm: Fixed::from_int(600),
        reach_other_ship_nm: Fixed::from_int(300),
        reach_base_nm: Fixed::from_int(1000),
    });
    world.begin_holding();
    world
}

const ON_TIME: (i64, i64) = (1, 0);
const LATE: (i64, i64) = (0, 1);
const NEVER: (i64, i64) = (0, 0);

fn base1() -> CommandRef {
    CommandRef::Base(BASE1)
}

fn held_of(world: &World, command: CommandRef) -> &HeldState {
    &world.held(Side::Allied).expect("holding")[&command]
}

fn position(state: &HeldState) -> Option<crate::hex::Hex> {
    state.task_force().map(crate::state::TaskForce::position)
}

fn move_force(world: &mut World, to: crate::hex::Hex) {
    world
        .truth
        .task_forces
        .get_mut(&TF1)
        .expect("force 1")
        .from_hex = to;
}

fn state_events(world: &World) -> Vec<&EventKind> {
    world
        .events()
        .iter()
        .map(|event| &event.kind)
        .filter(|kind| {
            matches!(
                kind,
                EventKind::StateReportLate { .. } | EventKind::StateReportLost { .. }
            )
        })
        .collect()
}

#[test]
fn every_command_is_held_exactly_as_of_the_start() {
    let world = holding(ON_TIME);
    let held = world.held(Side::Allied).expect("holding");
    assert_eq!(
        held.keys().copied().collect::<Vec<_>>(),
        vec![tf(TF1), tf(TF2), base1()]
    );
    for state in held.values() {
        assert_eq!(state.as_of(), AsOf::Start);
    }
    assert_eq!(held_of(&world, tf(TF1)).ships().count(), 2);
    assert_eq!(position(held_of(&world, tf(TF1))), Some(hx(0, 0)));
    assert!(held_of(&world, base1()).base().is_some());
    // The enemy's area command holds its own.
    assert!(world.held(Side::Japanese).is_some());
}

#[test]
fn nothing_is_held_with_command_levels_off_or_no_report_figures() {
    let plain = crate::pictures_tests::world();
    assert!(plain.held(Side::Allied).is_none());
    let mut no_figures = world_with_levels(&[]);
    no_figures.begin_holding();
    assert!(no_figures.held(Side::Allied).is_none());
    // And nothing is made or drawn at a report moment.
    no_figures.make_state_reports(1, Pulse::Night);
    assert!(state_events(&no_figures).is_empty());
}

#[test]
fn a_report_on_time_is_held_at_once_and_is_no_event() {
    let mut world = holding(ON_TIME);
    move_force(&mut world, hx(3, 0));
    world.make_state_reports(1, Pulse::Night);
    let night = AsOf::Pulse {
        day: 1,
        pulse: Pulse::Night,
    };
    assert_eq!(held_of(&world, tf(TF1)).as_of(), night);
    assert_eq!(position(held_of(&world, tf(TF1))), Some(hx(3, 0)));
    assert_eq!(held_of(&world, base1()).as_of(), night);
    assert!(state_events(&world).is_empty());
}

#[test]
fn a_late_report_is_an_event_and_arrives_at_the_end_of_the_next_pulse() {
    let mut world = holding(LATE);
    move_force(&mut world, hx(3, 0));
    world.make_state_reports(1, Pulse::Night);
    // Three late events, one for each command of the side, and the enemy's
    // two besides; the held state is still the start's.
    assert_eq!(position(held_of(&world, tf(TF1))), Some(hx(0, 0)));
    assert_eq!(held_of(&world, tf(TF1)).as_of(), AsOf::Start);
    let late: Vec<_> = state_events(&world)
        .into_iter()
        .filter(|kind| {
            matches!(
                kind,
                EventKind::StateReportLate {
                    side: Side::Allied,
                    ..
                }
            )
        })
        .collect();
    assert_eq!(late.len(), 3);
    assert_eq!(world.state_reports_in_flight.len(), 5);
    // The next pulse's moment: the night's report arrives first, then the
    // morning's own is drawn late again.
    world.make_state_reports(1, Pulse::Morning);
    assert_eq!(
        held_of(&world, tf(TF1)).as_of(),
        AsOf::Pulse {
            day: 1,
            pulse: Pulse::Night
        }
    );
    assert_eq!(position(held_of(&world, tf(TF1))), Some(hx(3, 0)));
    assert_eq!(world.state_reports_in_flight.len(), 5);
}

#[test]
fn a_lost_report_is_an_event_and_changes_nothing() {
    let mut world = holding(NEVER);
    move_force(&mut world, hx(3, 0));
    world.make_state_reports(1, Pulse::Night);
    assert_eq!(held_of(&world, tf(TF1)).as_of(), AsOf::Start);
    assert!(world.state_reports_in_flight.is_empty());
    let lost = state_events(&world)
        .into_iter()
        .filter(|kind| {
            matches!(
                kind,
                EventKind::StateReportLost {
                    side: Side::Allied,
                    ..
                }
            )
        })
        .count();
    assert_eq!(lost, 3);
    let event = world
        .events()
        .iter()
        .find(|event| matches!(event.kind, EventKind::StateReportLost { .. }))
        .expect("an event");
    let names: Vec<&str> = event
        .breakdown
        .as_ref()
        .expect("a breakdown")
        .factors
        .iter()
        .map(|factor| factor.name)
        .collect();
    assert_eq!(names, ["on_time_chance", "late_chance", "draw"]);
    assert!(event.causes.is_empty());
    assert!(event
        .kind
        .to_string()
        .ends_with("of the night of day 1 is lost"));
}

#[test]
fn an_older_report_arriving_after_a_newer_one_changes_nothing() {
    let mut world = holding(ON_TIME);
    // A morning report held; then a night's arrives late.
    move_force(&mut world, hx(4, 0));
    world.make_state_reports(1, Pulse::Morning);
    let stale = {
        let mut state = held_of(&world, tf(TF1)).clone();
        state.as_of = AsOf::Pulse {
            day: 1,
            pulse: Pulse::Night,
        };
        state.task_force.as_mut().expect("record").from_hex = hx(9, 0);
        state
    };
    world.hold_reported(Side::Allied, tf(TF1), stale);
    assert_eq!(position(held_of(&world, tf(TF1))), Some(hx(4, 0)));
}

#[test]
fn the_area_command_reads_what_it_holds_and_every_other_command_the_truth() {
    let mut world = holding(NEVER);
    move_force(&mut world, hx(3, 0));
    world.make_state_reports(1, Pulse::Night);
    let view = world.side_view(Side::Allied);
    let area = view.known_to(CommandRef::Area);
    let seen: Vec<_> = area
        .view()
        .task_forces()
        .map(|force| (force.id, force.position()))
        .collect();
    assert_eq!(seen, vec![(TF1, hx(0, 0)), (TF2, hx(1, 0))]);
    assert_eq!(area.view().held_as_of(tf(TF1)), Some(AsOf::Start));
    // Its own units are as they are for a command.
    let own = view.known_to(tf(TF1));
    let seen: Vec<_> = own
        .view()
        .task_forces()
        .map(|force| force.position())
        .collect();
    assert_eq!(seen, vec![hx(3, 0), hx(1, 0)]);
    assert_eq!(own.view().held_as_of(tf(TF1)), None);
    assert_eq!(view.held_as_of(tf(TF1)), None);
    // The enemy is untouched.
    assert_eq!(
        area.view().enemy_bases().count(),
        view.enemy_bases().count()
    );
    assert_eq!(view.pulse_just_ended(), AsOf::Start);
}

#[test]
fn the_newest_word_wins_for_a_ship_two_reports_name() {
    let mut world = holding(ON_TIME);
    // Ship 2 moves from force 1 to force 2 and only force 2 reports it.
    world
        .truth
        .task_forces
        .get_mut(&TF1)
        .expect("force")
        .ships
        .remove(&crate::ids::ShipId::new(2));
    world
        .truth
        .task_forces
        .get_mut(&TF2)
        .expect("force")
        .ships
        .insert(crate::ids::ShipId::new(2));
    world
        .truth
        .ships
        .get_mut(&crate::ids::ShipId::new(2))
        .expect("ship")
        .location = crate::state::ShipLocation::TaskForce(TF2);
    // Force 1's report is lost, force 2's arrives (a lost report of one
    // command cannot be set apart by figures, so hold one by hand).
    let state = crate::held::state_of(
        &world.truth,
        tf(TF2),
        AsOf::Pulse {
            day: 1,
            pulse: Pulse::Night,
        },
    );
    world.hold_reported(Side::Allied, tf(TF2), state);
    let view = world.side_view(Side::Allied);
    let area = view.known_to(CommandRef::Area);
    let ship = area
        .view()
        .ships()
        .find(|ship| ship.id == crate::ids::ShipId::new(2))
        .expect("ship");
    assert_eq!(ship.location, crate::state::ShipLocation::TaskForce(TF2));
}

#[test]
fn a_silent_force_makes_no_report_until_it_breaks_silence() {
    let mut world = holding(ON_TIME);
    world.radio_silent.insert(TF1);
    move_force(&mut world, hx(3, 0));
    world.make_state_reports(1, Pulse::Night);
    assert_eq!(held_of(&world, tf(TF1)).as_of(), AsOf::Start);
    assert_eq!(
        held_of(&world, tf(TF2)).as_of(),
        AsOf::Pulse {
            day: 1,
            pulse: Pulse::Night
        }
    );
    // Breaking silence this pulse: it reports, and the set is cleared after.
    world.broke_silence.insert(TF1);
    world.make_state_reports(1, Pulse::Morning);
    assert_eq!(
        held_of(&world, tf(TF1)).as_of(),
        AsOf::Pulse {
            day: 1,
            pulse: Pulse::Morning
        }
    );
    assert!(world.broke_silence.is_empty());
}

#[test]
fn a_force_that_ceased_to_be_makes_one_last_report_of_a_record_with_no_ships() {
    let mut world = holding(ON_TIME);
    // Force 2 is stood down: its ship lies in port in no force.
    world
        .truth
        .task_forces
        .get_mut(&TF2)
        .expect("force")
        .ships
        .clear();
    world
        .truth
        .ships
        .get_mut(&crate::ids::ShipId::new(5))
        .expect("ship")
        .location = crate::state::ShipLocation::Port(BASE1);
    world.end_gone_commands();
    world.make_state_reports(1, Pulse::Night);
    let state = held_of(&world, tf(TF2));
    assert_eq!(state.ships().count(), 0);
    assert!(state.task_force().expect("record").ships.is_empty());
    assert!(world.gone_commands.is_empty());
}

#[test]
fn a_command_formed_is_held_as_of_now_and_a_base_changing_side_moves_its_hold() {
    let mut world = holding(ON_TIME);
    let new = crate::ids::TaskForceId::new(1_000_001);
    let mut force = crate::state::TaskForce::at(
        new,
        "New".to_string(),
        Side::Allied,
        hx(0, 0),
        crate::orders::Mission::Transport,
    );
    force.ships.insert(crate::ids::ShipId::new(3));
    world.truth.task_forces.insert(new, force);
    world
        .truth
        .ships
        .get_mut(&crate::ids::ShipId::new(3))
        .expect("ship")
        .location = crate::state::ShipLocation::TaskForce(new);
    world.command_formed(new, base1());
    let state = held_of(&world, tf(new));
    assert_eq!(state.as_of(), AsOf::Start);
    assert_eq!(state.ships().count(), 1);
    // The base is lost to the other side.
    world.truth.bases.get_mut(&BASE1).expect("base").side = Side::Japanese;
    world.base_changed_side(BASE1, Side::Allied);
    assert!(!world
        .held(Side::Allied)
        .expect("holding")
        .contains_key(&base1()));
    assert!(world
        .held(Side::Japanese)
        .expect("holding")
        .contains_key(&base1()));
}

#[test]
fn a_base_formed_by_a_march_is_held_as_it_stood_before_its_supply_was_put_into_it() {
    // Decision 0237, rule 3 and ruling 5: the hook runs before the supply
    // carried is put into the base.
    let mut world = holding(ON_TIME);
    let id = crate::ids::BaseId::new(3);
    world.truth.bases.insert(
        id,
        crate::state::Base::new(id, "Dump at (1,1)".to_string(), Side::Allied, hx(1, 1)),
    );
    world.base_formed_by_march(id, hx(0, 0));
    let new = CommandRef::Base(id);
    let held = held_of(&world, new);
    assert_eq!(held.as_of(), AsOf::Start);
    assert_eq!(held.base().expect("a base").supply_t, Fixed::ZERO);
    // 5 t is put into the base in truth, as the march does after the hook.
    world.truth.bases.get_mut(&id).expect("base").supply_t = Fixed::from_int(5);
    assert_eq!(
        held_of(&world, new).base().expect("a base").supply_t,
        Fixed::ZERO
    );
    assert!(!world
        .held(Side::Japanese)
        .expect("holding")
        .contains_key(&new));
}

#[test]
fn radio_silence_is_an_order_and_a_setting_that_is_part_of_the_hash() {
    let mut world = holding(ON_TIME);
    let before = world.state_hash();
    let order = |on| OrderRequest {
        kind: OrderKind::TaskForce {
            target: TF1,
            order: TaskForceOrder::SetRadioSilence { on },
        },
        origin: Origin::PLAYER,
    };
    assert_eq!(world.check_order(Side::Allied, &order(true)), Ok(()));
    world.take_in_order(1, None, Side::Allied, order(true), Vec::new());
    assert!(world.radio_silence(TF1));
    assert_eq!(world.side_view(Side::Allied).radio_silence(TF1), Some(true));
    assert_eq!(
        world.side_view(Side::Allied).radio_silence(TF2),
        Some(false)
    );
    // The enemy's force is not the side's to ask about.
    assert_eq!(
        world
            .side_view(Side::Allied)
            .radio_silence(crate::pictures_tests::TF9),
        None
    );
    assert_ne!(world.state_hash(), before);
    world.take_in_order(1, None, Side::Allied, order(false), Vec::new());
    assert!(!world.radio_silence(TF1));
    assert_eq!(world.state_hash(), before_orders_only(&world, before));
}

/// The orders received are hashed by count, so the hash after two orders
/// differs from `before` by that alone: rebuild it from a world that took the
/// same two orders to compare.
fn before_orders_only(world: &World, _before: u64) -> u64 {
    let mut other = holding(ON_TIME);
    let order = |on| OrderRequest {
        kind: OrderKind::TaskForce {
            target: TF1,
            order: TaskForceOrder::SetRadioSilence { on },
        },
        origin: Origin::PLAYER,
    };
    other.take_in_order(1, None, Side::Allied, order(true), Vec::new());
    other.take_in_order(1, None, Side::Allied, order(false), Vec::new());
    let _ = world;
    other.state_hash()
}

#[test]
fn what_is_held_and_what_is_in_flight_is_part_of_the_hash() {
    let a = holding(ON_TIME);
    let mut b = holding(ON_TIME);
    assert_eq!(a.state_hash(), b.state_hash());
    move_force(&mut b, hx(3, 0));
    b.make_state_reports(1, Pulse::Night);
    assert_ne!(a.state_hash(), b.state_hash());
    let mut late = holding(LATE);
    let before = late.state_hash();
    late.make_state_reports(1, Pulse::Night);
    assert_ne!(late.state_hash(), before);
    assert!(!late.state_reports_in_flight.is_empty());
}

// ---- Phase 2: breaking silence, and an order that meets a different world ----

use crate::command::{Addressee, Intent, IntentKind, Maker, PlanId};
use crate::ids::{IntentId, ShipId};
use crate::orders::{Issuer, RejectReason, SpeedSetting};
use crate::passing_tests::{make, sighting_at};

fn area_contacts(world: &World) -> Vec<u32> {
    world.commands(Side::Allied).expect("on")[&CommandRef::Area]
        .picture()
        .contacts()
        .map(|contact| contact.age_pulses)
        .collect()
}

/// Ages every picture as the end of a pulse does.
fn end_pulse(world: &mut World, day: u32, pulse: Pulse) {
    world.age_contacts_at_end_of_pulse(day, pulse);
}

#[test]
fn a_silent_force_sends_a_sighting_when_it_breaks_silence_as_old_as_it_is() {
    let mut world = holding(ON_TIME);
    world.radio_silent.insert(TF1);
    // Pulse 3 is the night of day 1, pulse 5 its afternoon. The sighting is
    // the silent force's own and is kept.
    make(&mut world, &[tf(TF1)], 1, Pulse::Night, |key| {
        sighting_at(key, hx(6, 2), 2)
    });
    assert_eq!(world.unsent_reports[&TF1].len(), 1);
    assert!(area_contacts(&world).is_empty());
    end_pulse(&mut world, 1, Pulse::Night);
    end_pulse(&mut world, 1, Pulse::Morning);
    assert!(
        area_contacts(&world).is_empty(),
        "nothing after pulses 3 and 4"
    );
    // The strike launch calls `break_silence(force, day, pulse)` for the force
    // of the launching group; the same hook is called here, a launch being
    // too heavy to build in this world.
    world.break_silence(TF1, 1, Pulse::Afternoon);
    assert_eq!(area_contacts(&world), vec![2], "heard at age 2");
    assert!(world.unsent_reports.is_empty());
    assert!(world.broke_silence.contains(&TF1));
    // A second call in the same pulse sends nothing more.
    let sent = |world: &World| {
        world
            .events()
            .iter()
            .filter(|e| matches!(e.kind, EventKind::ReportSent { .. }))
            .count()
    };
    let before = sent(&world);
    world.break_silence(TF1, 1, Pulse::Afternoon);
    assert_eq!(sent(&world), before);
}

#[test]
fn a_report_older_than_a_contact_may_be_is_not_sent_and_silence_off_drops_what_was_kept() {
    let mut world = holding(ON_TIME);
    world.radio_silent.insert(TF1);
    make(&mut world, &[tf(TF1)], 1, Pulse::Night, |key| {
        sighting_at(key, hx(6, 2), 2)
    });
    // Five pulses old (the drop age): not sent.
    world.break_silence(TF1, 2, Pulse::Afternoon);
    assert!(area_contacts(&world).is_empty());
    // Turning silence off by order drops what was kept.
    let mut world = holding(ON_TIME);
    world.radio_silent.insert(TF1);
    make(&mut world, &[tf(TF1)], 1, Pulse::Night, |key| {
        sighting_at(key, hx(6, 2), 2)
    });
    let off = OrderRequest {
        kind: OrderKind::TaskForce {
            target: TF1,
            order: TaskForceOrder::SetRadioSilence { on: false },
        },
        origin: Origin::PLAYER,
    };
    world.take_in_order(1, None, Side::Allied, off, Vec::new());
    assert!(world.unsent_reports.is_empty());
    assert!(!world.radio_silence(TF1));
}

#[test]
fn a_force_that_breaks_silence_makes_its_state_report_that_pulse_and_is_silent_again_after() {
    let mut world = holding(NEVER);
    world.radio_silent.insert(TF1);
    let lost = |world: &World, force: Option<crate::ids::TaskForceId>| {
        world
            .events()
            .iter()
            .filter(|e| match (&e.kind, force) {
                (
                    EventKind::StateReportLost {
                        side: Side::Allied,
                        command,
                        ..
                    },
                    Some(f),
                ) => *command == tf(f),
                (
                    EventKind::StateReportLost {
                        side: Side::Allied, ..
                    },
                    None,
                ) => true,
                _ => false,
            })
            .count()
    };
    world.make_state_reports(1, Pulse::Night);
    assert_eq!(lost(&world, None), 2, "the base and force 2 only");
    assert_eq!(lost(&world, Some(TF1)), 0);
    world.break_silence(TF1, 1, Pulse::Morning);
    world.make_state_reports(1, Pulse::Morning);
    assert_eq!(lost(&world, Some(TF1)), 1);
    world.make_state_reports(1, Pulse::Afternoon);
    assert_eq!(lost(&world, Some(TF1)), 1, "silent again");
    assert_eq!(lost(&world, None), 2 + 3 + 2);
}

fn sink_force_1(world: &mut World) {
    world
        .truth
        .task_forces
        .get_mut(&TF1)
        .expect("force")
        .ships
        .clear();
    for raw in [1, 2] {
        world
            .truth
            .ships
            .get_mut(&ShipId::new(raw))
            .expect("ship")
            .location = crate::state::ShipLocation::Sunk;
    }
}

fn speed_order(origin: Origin) -> OrderRequest {
    OrderRequest {
        kind: OrderKind::TaskForce {
            target: TF1,
            order: TaskForceOrder::SetSpeed(SpeedSetting::Full),
        },
        origin,
    }
}

#[test]
fn an_order_from_the_area_to_a_force_sunk_in_truth_but_held_afloat_is_refused_as_not_as_held() {
    let mut world = holding(ON_TIME);
    world.make_state_reports(1, Pulse::Morning);
    sink_force_1(&mut world);
    // The truth refuses: nothing is afloat in it.
    let plain = world.validate_order_on_truth(&crate::orders::Order {
        id: crate::ids::OrderId::new(0),
        side: Side::Allied,
        day_issued: 2,
        kind: speed_order(Origin::PLAYER).kind,
        origin: Origin::PLAYER,
    });
    let reason = plain.expect_err("the truth refuses");
    let refused = world
        .check_order(Side::Allied, &speed_order(Origin::PLAYER))
        .expect_err("refused");
    assert!(refused.is_not_as_held());
    assert_eq!(
        refused.to_string(),
        format!(
            "{reason}; the area command held task_force:1 at (0,0) with 2 ships afloat, as of the morning of day 1"
        )
    );
    // At intake too: the order is rejected with the same words.
    world.take_in_order(
        2,
        None,
        Side::Allied,
        speed_order(Origin::PLAYER),
        Vec::new(),
    );
    let rejected = world
        .events()
        .iter()
        .find_map(|e| match &e.kind {
            EventKind::OrderRejected { reason, .. } => Some(reason.clone()),
            _ => None,
        })
        .expect("rejected");
    assert!(rejected.is_not_as_held());
}

#[test]
fn an_order_from_a_force_level_intent_or_with_no_reports_is_refused_as_before() {
    let mut world = holding(ON_TIME);
    world.make_state_reports(1, Pulse::Morning);
    sink_force_1(&mut world);
    // A staff order whose intent is addressed to the force.
    let intent = IntentId::new(1);
    world
        .command
        .get_mut(&Side::Allied)
        .expect("state")
        .intents
        .insert(
            intent,
            Intent {
                id: intent,
                kind: IntentKind::Hold { base: BASE1 },
                parent: None,
                maker: Maker::Staff,
                addressee: Addressee::TaskForce(TF1),
                limits: Default::default(),
                set_day: 1,
                plan: PlanId::new(1),
                withdrawn_day: None,
            },
        );
    let staff = Origin {
        issuer: Issuer::Staff,
        intent: Some(intent),
    };
    let refused = world
        .check_order(Side::Allied, &speed_order(staff))
        .expect_err("refused");
    assert!(!refused.is_not_as_held(), "{refused}");
    // The same intent addressed to the area is wrapped.
    world
        .command
        .get_mut(&Side::Allied)
        .expect("state")
        .intents
        .get_mut(&intent)
        .expect("intent")
        .addressee = Addressee::Area;
    assert!(world
        .check_order(Side::Allied, &speed_order(staff))
        .expect_err("refused")
        .is_not_as_held());
    // Reports not travelling: refused as before.
    let mut plain = world_with_levels(&[]);
    plain
        .truth
        .task_forces
        .get_mut(&TF1)
        .expect("force")
        .ships
        .clear();
    for raw in [1, 2] {
        plain
            .truth
            .ships
            .get_mut(&ShipId::new(raw))
            .expect("ship")
            .location = crate::state::ShipLocation::Sunk;
    }
    let refused = plain
        .check_order(Side::Allied, &speed_order(Origin::PLAYER))
        .expect_err("refused");
    assert!(!matches!(refused, RejectReason::NotAsHeld { .. }));
}
