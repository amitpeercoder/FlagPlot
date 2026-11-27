//! Reports between commands (decision 0135): who holds a report first, to
//! whom it is passed, when it arrives and what a late one does to a picture,
//! in the world of `pictures_tests` (Allied task force 1 at (0,0), task force
//! 2 at (1,0) and base 1 at (0,0); every ship a cruiser). Every chance in a
//! test is 0 or 1, so no outcome rests on a die. Scenario 62
//! (`tests/scenario_62_reports_between_commands.rs`) holds the proof that a
//! whole game plays as the rules say.

use crate::belief::{ApplyOutcome, BattleClaim, BeliefState, Report, SightingReport};
use crate::events::{Arrival, Cause, Direction, Event, EventKind};
use crate::fixed::Fixed;
use crate::ids::{ContactId, EntityRef, EventId};
use crate::pictures::{CommandRef, Holders};
use crate::pictures_tests::{hx, tf, world_with_levels, BASE1, TF1, TF2};
use crate::refdata::{PassingChances, ReportFigures, SearchFigures, ShipKind};
use crate::state::Side;
use crate::time::Pulse;
use crate::world::World;

const AREA: CommandRef = CommandRef::Area;
const DAY: u32 = 1;
const NOON: Pulse = Pulse::Morning;

fn base1() -> CommandRef {
    CommandRef::Base(BASE1)
}

/// A pair of chances, each 0 or 1: `(on_time, late)`.
fn chances(on_time: i64, late: i64) -> PassingChances {
    PassingChances {
        on_time: Fixed::from_int(on_time),
        late: Fixed::from_int(late),
    }
}

/// Report figures for a test: the chances of each direction and the radio
/// reach of a cruiser; every other reach as large as it needs to be.
fn figures(
    up: (i64, i64),
    sideways: (i64, i64),
    down: (i64, i64),
    cruiser_reach_nm: i64,
) -> ReportFigures {
    ReportFigures {
        up: chances(up.0, up.1),
        sideways: chances(sideways.0, sideways.1),
        down: chances(down.0, down.1),
        reach_carrier_nm: Fixed::from_int(600),
        reach_battleship_nm: Fixed::from_int(600),
        reach_cruiser_nm: Fixed::from_int(cruiser_reach_nm),
        reach_other_ship_nm: Fixed::from_int(300),
        reach_base_nm: Fixed::from_int(1000),
    }
}

/// A world with command levels on and these report figures.
fn travelling(
    up: (i64, i64),
    sideways: (i64, i64),
    down: (i64, i64),
    cruiser_reach_nm: i64,
) -> World {
    let mut world = world_with_levels(&[]);
    world.ref_data.reports = Some(figures(up, sideways, down, cruiser_reach_nm));
    world
}

const NEVER: (i64, i64) = (0, 0);
const ON_TIME: (i64, i64) = (1, 0);
const LATE: (i64, i64) = (0, 1);

/// Records an event for a report to cite, and returns its id, the report's
/// key. The kind does not matter to the passing.
pub(crate) fn record_key(world: &mut World, day: u32, pulse: Pulse) -> EventId {
    world.events.record(
        day,
        Some(pulse),
        EventKind::ReportReceived {
            side: Side::Allied,
            contact: ContactId::new(1),
            hex: hx(0, 0),
            ship_count: 1,
            kind: ShipKind::Cruiser,
            created: true,
        },
        Vec::new(),
        None,
    )
}

pub(crate) fn sighting_at(key: EventId, at: crate::hex::Hex, count: u32) -> Report {
    Report::Sighting(SightingReport {
        estimated_hex: at,
        estimated_ship_count: count,
        estimated_kind: ShipKind::Cruiser,
        received_event: key,
        age_pulses: 0,
    })
}

fn claim(key: EventId) -> Report {
    Report::BattleClaim(BattleClaim {
        hex: hx(4, 0),
        day: DAY,
        pulse: NOON,
        claimed_sunk: 1,
        claimed_hits: 0,
        claimed_kind: None,
        claimed_shot_down: 0,
        claimed_on_ground: 0,
        from_air: false,
        from_bombardment: false,
        from_submarine: false,
        received_event: key,
        ground_types: Vec::new(),
        by_unit: Vec::new(),
        air_types: Vec::new(),
    })
}

/// Makes a report as a site does: its event first, then the report applied
/// to the holders, then passed on. Returns the report's key.
pub(crate) fn make(
    world: &mut World,
    holders: &[CommandRef],
    day: u32,
    pulse: Pulse,
    build: impl Fn(EventId) -> Report,
) -> EventId {
    let key = world.events.next_id();
    let report = build(key);
    let holders = Holders::Commands {
        commands: holders.to_vec(),
        key,
        day,
        pulse: Some(pulse),
    };
    let figures = world.ref_data.search.clone();
    world.apply_report(Side::Allied, &report, &holders, &figures);
    assert_eq!(record_key(world, day, pulse), key);
    world.pass_report(Side::Allied, &report, &holders);
    key
}

fn contacts_of(world: &World, command: CommandRef) -> Vec<(u32, u32, u32)> {
    world.commands(Side::Allied).expect("levels on")[&command]
        .picture()
        .contacts()
        .map(|c| (c.id.raw(), c.estimated_ship_count, c.age_pulses))
        .collect()
}

fn claims_of(world: &World, command: CommandRef) -> usize {
    world.commands(Side::Allied).expect("levels on")[&command]
        .picture()
        .claims()
        .len()
}

fn own_contacts(world: &World) -> usize {
    world
        .side_view(Side::Allied)
        .command(AREA)
        .map_or(0, |_| world.beliefs[&Side::Allied].contacts().count())
}

fn sent(world: &World) -> Vec<&Event> {
    world
        .events
        .iter()
        .filter(|event| matches!(event.kind, EventKind::ReportSent { .. }))
        .collect()
}

fn listeners(event: &Event) -> Vec<(CommandRef, Direction, Arrival)> {
    match &event.kind {
        EventKind::ReportSent { listeners, .. } => listeners
            .iter()
            .map(|l| (l.command, l.direction, l.arrival))
            .collect(),
        other => panic!("not a report sent: {other:?}"),
    }
}

fn sender(event: &Event) -> CommandRef {
    match &event.kind {
        EventKind::ReportSent { sender, .. } => *sender,
        other => panic!("not a report sent: {other:?}"),
    }
}

// ---------------------------------------------------------- Who holds it

#[test]
fn only_the_holder_and_the_side_get_a_report_nothing_passes() {
    let mut world = travelling(NEVER, NEVER, NEVER, 600);
    make(&mut world, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    assert_eq!(contacts_of(&world, tf(TF1)), vec![(1, 2, 0)]);
    for other in [AREA, tf(TF2), base1()] {
        assert!(contacts_of(&world, other).is_empty(), "{other}");
    }
    assert_eq!(own_contacts(&world), 1, "the side's own gets it at once");
    // One passing, to the superior and the two within reach, none arriving.
    let events = sent(&world);
    assert_eq!(events.len(), 1);
    assert_eq!(sender(events[0]), tf(TF1));
    assert_eq!(
        listeners(events[0]),
        vec![
            (AREA, Direction::Up, Arrival::Never),
            (tf(TF2), Direction::Sideways, Arrival::Never),
            (base1(), Direction::Sideways, Arrival::Never),
        ]
    );
}

#[test]
fn a_holder_that_is_not_a_command_is_the_area_command() {
    let mut world = travelling(NEVER, NEVER, NEVER, 600);
    make(
        &mut world,
        &[tf(crate::ids::TaskForceId::new(77))],
        DAY,
        NOON,
        |key| sighting_at(key, hx(5, 0), 2),
    );
    assert_eq!(contacts_of(&world, AREA), vec![(1, 2, 0)]);
    assert!(contacts_of(&world, tf(TF1)).is_empty());
}

#[test]
fn up_on_time_reaches_the_area_command_which_sends_it_down_but_not_back() {
    let mut world = travelling(ON_TIME, NEVER, NEVER, 600);
    let key = make(&mut world, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    assert_eq!(contacts_of(&world, AREA), vec![(1, 2, 0)]);
    assert!(contacts_of(&world, tf(TF2)).is_empty());
    let events = sent(&world);
    assert_eq!(events.len(), 2);
    // The area command's passing: down to every command but the one that
    // sent it, which is task force 1.
    assert_eq!(sender(events[1]), AREA);
    assert_eq!(
        listeners(events[1]),
        vec![
            (tf(TF2), Direction::Down, Arrival::Never),
            (base1(), Direction::Down, Arrival::Never),
        ]
    );
    // Caused by the report, and by the passing that brought it to the area.
    assert_eq!(
        events[1].causes,
        vec![Cause::Event(key), Cause::Event(events[0].id)]
    );
    assert_eq!(events[0].causes, vec![Cause::Event(key)]);
}

#[test]
fn the_area_command_as_holder_sends_down_with_nobody_excluded() {
    let mut world = travelling(NEVER, NEVER, ON_TIME, 600);
    make(&mut world, &[AREA], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    let events = sent(&world);
    assert_eq!(events.len(), 1);
    assert_eq!(sender(events[0]), AREA);
    assert_eq!(
        listeners(events[0]),
        vec![
            (tf(TF1), Direction::Down, Arrival::OnTime),
            (tf(TF2), Direction::Down, Arrival::OnTime),
            (base1(), Direction::Down, Arrival::OnTime),
        ]
    );
    for command in [tf(TF1), tf(TF2), base1()] {
        assert_eq!(contacts_of(&world, command).len(), 1, "{command}");
    }
    // Its causes are the report alone.
    assert_eq!(events[0].causes.len(), 1);
}

#[test]
fn sideways_reaches_exactly_as_far_as_the_radio_does() {
    // Task force 2 is 40 nm from task force 1.
    let mut at = travelling(NEVER, ON_TIME, NEVER, 40);
    make(&mut at, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    assert_eq!(contacts_of(&at, tf(TF2)).len(), 1, "at exactly the reach");
    assert_eq!(contacts_of(&at, base1()).len(), 1);

    let mut beyond = travelling(NEVER, ON_TIME, NEVER, 39);
    make(&mut beyond, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    assert!(contacts_of(&beyond, tf(TF2)).is_empty());
    assert_eq!(contacts_of(&beyond, base1()).len(), 1, "base 1 is at 0 nm");
    let events = sent(&beyond);
    assert_eq!(
        listeners(events[0]),
        vec![
            (AREA, Direction::Up, Arrival::Never),
            (base1(), Direction::Sideways, Arrival::OnTime),
        ]
    );
}

#[test]
fn a_base_sends_sideways_by_its_own_reach() {
    let mut world = travelling(NEVER, ON_TIME, NEVER, 1);
    // A task force of the Allied side 400 nm from the base: its own reach (1
    // nm for a cruiser here) could not reach the base, the base's can reach
    // it.
    world
        .truth
        .task_forces
        .get_mut(&TF2)
        .expect("force")
        .from_hex = hx(10, 0);
    make(&mut world, &[base1()], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    assert_eq!(contacts_of(&world, base1()).len(), 1);
    assert_eq!(contacts_of(&world, tf(TF2)).len(), 1);
    // The area command has no place: it is never a sideways listener.
    let listed = listeners(sent(&world)[0]);
    assert!(!listed.contains(&(AREA, Direction::Sideways, Arrival::OnTime)));
}

#[test]
fn what_a_command_was_only_told_it_does_not_pass_on() {
    let mut world = travelling(NEVER, ON_TIME, ON_TIME, 600);
    make(&mut world, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    for event in sent(&world) {
        assert!(
            sender(event) == tf(TF1) || sender(event) == AREA,
            "only the holder and the area command send"
        );
    }
}

// ---------------------------------------------------- The area's limit

fn give_intent(
    world: &mut World,
    number: u32,
    addressee: crate::command::Addressee,
    area: Option<&[crate::hex::Hex]>,
) {
    use crate::command::{Intent, IntentKind, IntentLimits, Maker, PlanId};
    let limits = IntentLimits {
        area: area.map(|hexes| hexes.iter().copied().collect()),
        ..IntentLimits::default()
    };
    let id = crate::ids::IntentId::new(number);
    world
        .command
        .get_mut(&Side::Allied)
        .expect("side")
        .intents
        .insert(
            id,
            Intent {
                id,
                kind: IntentKind::KeepSupplied {
                    base: BASE1,
                    floor_days: 1,
                    interval_days: None,
                },
                parent: None,
                maker: Maker::Commander,
                addressee,
                limits,
                set_day: 1,
                plan: PlanId::new(1),
                withdrawn_day: None,
            },
        );
}

#[test]
fn the_area_command_tells_down_what_bears_on_a_commands_intents() {
    use crate::command::Addressee;
    let mut world = travelling(ON_TIME, NEVER, ON_TIME, 600);
    // Base 1's intent has an area that holds the report's hex, task force
    // 2's does not, and task force 1 has none addressed to it (and is the
    // sender, so is not told in any case).
    give_intent(
        &mut world,
        1,
        Addressee::Base(BASE1),
        Some(&[hx(5, 0), hx(6, 0)]),
    );
    give_intent(&mut world, 2, Addressee::TaskForce(TF2), Some(&[hx(9, 9)]));
    make(&mut world, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    let down = listeners(sent(&world)[1]);
    assert_eq!(down, vec![(base1(), Direction::Down, Arrival::OnTime)]);
    assert!(contacts_of(&world, tf(TF2)).is_empty());
    assert_eq!(contacts_of(&world, base1()).len(), 1);
}

#[test]
fn an_intent_with_no_area_or_one_in_force_without_a_limit_bears_everything() {
    use crate::command::Addressee;
    let mut world = travelling(ON_TIME, NEVER, ON_TIME, 600);
    give_intent(&mut world, 1, Addressee::TaskForce(TF2), Some(&[hx(9, 9)]));
    // A second intent for the same command with no area: every report bears.
    give_intent(&mut world, 2, Addressee::TaskForce(TF2), None);
    make(&mut world, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    assert_eq!(contacts_of(&world, tf(TF2)).len(), 1);
}

// ------------------------------------------------------ When it arrives

#[test]
fn a_report_a_pulse_late_is_applied_at_the_start_of_the_next_pulse() {
    let mut world = travelling(NEVER, LATE, NEVER, 600);
    make(&mut world, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    assert!(contacts_of(&world, tf(TF2)).is_empty());
    assert_eq!(world.reports_in_flight.len(), 2, "task force 2 and base 1");
    assert_eq!(
        world.reports_in_flight[0].arrives,
        (DAY, Pulse::Afternoon),
        "the next pulse"
    );
    // Not before then.
    world.deliver_late_reports(DAY, NOON);
    assert!(contacts_of(&world, tf(TF2)).is_empty());
    world.deliver_late_reports(DAY, Pulse::Afternoon);
    assert!(world.reports_in_flight.is_empty());
    // A sighting is as old as the pulses since it was made.
    assert_eq!(contacts_of(&world, tf(TF2)), vec![(1, 2, 1)]);
    assert_eq!(contacts_of(&world, base1()), vec![(1, 2, 1)]);
}

#[test]
fn an_afternoon_report_arrives_in_the_next_nights_pulse() {
    let mut world = travelling(NEVER, LATE, NEVER, 600);
    make(&mut world, &[tf(TF1)], DAY, Pulse::Afternoon, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    assert_eq!(world.reports_in_flight[0].arrives, (DAY + 1, Pulse::Night));
    world.deliver_late_reports(DAY + 1, Pulse::Night);
    assert_eq!(contacts_of(&world, tf(TF2)), vec![(1, 2, 1)]);
}

#[test]
fn late_reports_are_applied_in_the_order_of_their_events() {
    let mut world = travelling(NEVER, LATE, NEVER, 600);
    // Two sightings of the same place, made in the same pulse: the later
    // event is the newer word and stands, whichever is sent first.
    let first = make(&mut world, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 3)
    });
    let second = make(&mut world, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 4)
    });
    assert!(first < second);
    // Put the later one first in the list of those in flight.
    world.reports_in_flight.reverse();
    world.deliver_late_reports(DAY, Pulse::Afternoon);
    let held = contacts_of(&world, tf(TF2));
    assert_eq!(held.len(), 1);
    assert_eq!(held[0].1, 4, "the later event's count stands");
}

#[test]
fn a_command_that_has_ceased_to_be_hears_nothing_late() {
    let mut world = travelling(NEVER, LATE, NEVER, 600);
    make(&mut world, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    world
        .chain
        .as_mut()
        .expect("on")
        .get_mut(&Side::Allied)
        .expect("side")
        .remove(&tf(TF2));
    world.deliver_late_reports(DAY, Pulse::Afternoon);
    assert_eq!(contacts_of(&world, base1()).len(), 1);
    assert!(world.reports_in_flight.is_empty());
}

#[test]
fn a_report_reaching_a_command_by_two_roads_is_heard_once() {
    // Sideways and down, both on time, to the same listener: a claim is
    // stored once.
    let mut world = travelling(ON_TIME, ON_TIME, ON_TIME, 600);
    make(&mut world, &[tf(TF1)], DAY, NOON, claim);
    assert_eq!(claims_of(&world, tf(TF1)), 1);
    assert_eq!(claims_of(&world, AREA), 1);
    assert_eq!(claims_of(&world, tf(TF2)), 1);
    assert_eq!(claims_of(&world, base1()), 1);
    // Both roads were drawn and recorded.
    let events = sent(&world);
    assert!(listeners(events[0])
        .iter()
        .any(|l| l.0 == tf(TF2) && l.1 == Direction::Sideways));
    assert!(listeners(events[1])
        .iter()
        .any(|l| l.0 == tf(TF2) && l.1 == Direction::Down));
    // And a late copy of it arriving after is heard and changes nothing.
    world
        .reports_in_flight
        .push(crate::passing::ReportInFlight {
            side: Side::Allied,
            report: claim(events[0].causes_key()),
            key: events[0].causes_key(),
            made: (DAY, NOON),
            sender: tf(TF1),
            direction: Direction::Sideways,
            listener: tf(TF2),
            sent_event: events[0].id,
            arrives: (DAY, Pulse::Afternoon),
        });
    world.deliver_late_reports(DAY, Pulse::Afternoon);
    assert_eq!(claims_of(&world, tf(TF2)), 1);
}

#[test]
fn a_report_sent_to_one_command_sideways_and_down_takes_two_numbers() {
    // The ruling after the build on item 11: the second road is a second
    // chance. Task force 2 is sent the report sideways by task force 1 and
    // down by the area command, in the same pulse; the two draws differ.
    let mut world = travelling(ON_TIME, ON_TIME, ON_TIME, 600);
    make(&mut world, &[tf(TF1)], DAY, NOON, claim);
    let events = sent(&world);
    let roll_to = |event: &Event, direction: Direction| {
        let at = listeners(event)
            .iter()
            .position(|l| l.0 == tf(TF2) && l.1 == direction)
            .expect("task force 2 is a listener");
        event.breakdown.as_ref().expect("breakdowns are on").factors[at * 3 + 2].value
    };
    let sideways = roll_to(events[0], Direction::Sideways);
    let down = roll_to(events[1], Direction::Down);
    assert_ne!(sideways, down);
    // Each is the first number of its own stream: up and sideways on
    // `ReportPassed`, down on `ReportPassedDown`, the key otherwise the same.
    let key = events[0].causes_key();
    let first = |draw: crate::rng::Draw| {
        let mut stream = world.rng_pair(
            DAY,
            Some(NOON),
            crate::rng::Subsystem::Intelligence,
            draw,
            EntityRef::Event(key),
            EntityRef::TaskForce(TF2),
        );
        Fixed::from_raw(i64::try_from(stream.below(Fixed::SCALE.unsigned_abs())).expect("fits"))
    };
    assert_eq!(sideways, first(crate::rng::Draw::ReportPassed));
    assert_eq!(down, first(crate::rng::Draw::ReportPassedDown));
}

trait CausesKey {
    fn causes_key(&self) -> EventId;
}

impl CausesKey for Event {
    /// The first cause of a "report sent": the report's event.
    fn causes_key(&self) -> EventId {
        match self.causes[0] {
            Cause::Event(id) => id,
            Cause::Order(_) => panic!("a report's event"),
        }
    }
}

#[test]
fn the_area_command_hearing_a_report_late_sends_it_down_then() {
    let mut world = travelling(LATE, NEVER, ON_TIME, 600);
    make(&mut world, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    assert!(contacts_of(&world, AREA).is_empty());
    assert_eq!(sent(&world).len(), 1);
    world.deliver_late_reports(DAY, Pulse::Afternoon);
    // The area command has it at age 1 and has sent it down, to the commands
    // under it but task force 1, in the pulse it heard it.
    assert_eq!(contacts_of(&world, AREA), vec![(1, 2, 1)]);
    let events = sent(&world);
    assert_eq!(events.len(), 2);
    assert_eq!(events[1].pulse, Some(Pulse::Afternoon));
    assert_eq!(
        listeners(events[1]),
        vec![
            (tf(TF2), Direction::Down, Arrival::OnTime),
            (base1(), Direction::Down, Arrival::OnTime),
        ]
    );
    assert_eq!(contacts_of(&world, tf(TF2)), vec![(1, 2, 1)]);
    assert!(contacts_of(&world, tf(TF1)).iter().all(|c| c.2 == 0));
}

// -------------------------------------------------- What a late one does

#[test]
fn a_late_sighting_is_one_pulse_old_then_one_and_then_two_as_the_makers_is() {
    let mut world = travelling(NEVER, LATE, NEVER, 600);
    make(&mut world, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    // The end of the pulse it was made in: the maker's contact is 0.
    world.age_contacts_at_end_of_pulse(DAY, NOON);
    assert_eq!(contacts_of(&world, tf(TF1)), vec![(1, 2, 0)]);
    // The start of the next pulse.
    world.deliver_late_reports(DAY, Pulse::Afternoon);
    assert_eq!(contacts_of(&world, tf(TF2)), vec![(1, 2, 1)]);
    // The end of it: the maker's is 1 and the listener's is 1 as well.
    world.age_contacts_at_end_of_pulse(DAY, Pulse::Afternoon);
    assert_eq!(contacts_of(&world, tf(TF1)), vec![(1, 2, 1)]);
    assert_eq!(contacts_of(&world, tf(TF2)), vec![(1, 2, 1)]);
    // The end of the next: both 2.
    world.age_contacts_at_end_of_pulse(DAY + 1, Pulse::Night);
    assert_eq!(contacts_of(&world, tf(TF1)), vec![(1, 2, 2)]);
    assert_eq!(contacts_of(&world, tf(TF2)), vec![(1, 2, 2)]);
}

#[test]
fn newer_word_stands() {
    let figures = SearchFigures::from_search_toml();
    let mut belief = BeliefState::new(Side::Allied);
    // A contact made by the event 5, 2 ships of one kind, age 0.
    let newer = Report::Sighting(SightingReport {
        estimated_hex: hx(3, 0),
        estimated_ship_count: 2,
        estimated_kind: ShipKind::Destroyer,
        received_event: EventId::new(5),
        age_pulses: 0,
    });
    assert!(matches!(
        belief.apply_report(&newer, &figures),
        ApplyOutcome::Contact(_)
    ));
    // A sighting of the event 4 of the same place arrives a pulse late.
    let older = Report::Sighting(SightingReport {
        estimated_hex: hx(3, 0),
        estimated_ship_count: 1,
        estimated_kind: ShipKind::Cruiser,
        received_event: EventId::new(4),
        age_pulses: 1,
    });
    assert_eq!(
        belief.apply_report(&older, &figures),
        ApplyOutcome::Unheeded
    );
    let held: Vec<_> = belief.contacts().collect();
    assert_eq!(held.len(), 1);
    assert_eq!(held[0].estimated_ship_count, 2);
    assert_eq!(held[0].estimated_kind, ShipKind::Destroyer);
    assert_eq!(held[0].age_pulses, 0);
    assert_eq!(held[0].last_report_event, EventId::new(5));
}

#[test]
fn nothing_there_by_number_for_the_holder_and_by_hex_for_the_rest_the_sides_own_too() {
    let mut world = travelling(ON_TIME, NEVER, NEVER, 600);
    // Every picture and the side's own hold contact 1 at (5,0).
    let figures_search = world.ref_data.search.clone();
    let key = world.events.next_id();
    let sighting = sighting_at(key, hx(5, 0), 2);
    world.apply_report(Side::Allied, &sighting, &Holders::Every, &figures_search);
    assert_eq!(record_key(&mut world, 0, Pulse::Night), key);
    world.age_contacts_at_end_of_pulse(0, Pulse::Night);
    // Task force 1's strike found nothing; it names contact 1 and the hex.
    let outcome_event = world.events.next_id();
    let report = Report::NothingThere {
        contact: Some(ContactId::new(1)),
        hex: hx(5, 0),
    };
    let holders = Holders::Commands {
        commands: vec![tf(TF1)],
        key: outcome_event,
        day: DAY,
        pulse: Some(NOON),
    };
    let dropped = world.apply_report(Side::Allied, &report, &holders, &figures_search);
    assert_eq!(dropped, ApplyOutcome::Dropped, "the side's own, by hex");
    assert_eq!(record_key(&mut world, DAY, NOON), outcome_event);
    // The holder and the side's own dropped it; the others have not yet.
    assert!(contacts_of(&world, tf(TF1)).is_empty());
    assert_eq!(own_contacts(&world), 0);
    for other in [AREA, tf(TF2), base1()] {
        assert_eq!(contacts_of(&world, other).len(), 1, "{other}");
    }
    // Passed up on time, the area command drops it, by hex.
    world.pass_report(Side::Allied, &report, &holders);
    assert!(contacts_of(&world, AREA).is_empty());
    assert_eq!(contacts_of(&world, tf(TF2)).len(), 1);
}

#[test]
fn nothing_there_by_hex_drops_the_lowest_numbered_contact_at_the_hex() {
    let figures = SearchFigures::from_search_toml();
    let mut belief = BeliefState::new(Side::Allied);
    let at = hx(3, 0);
    let sight = |event: u32| {
        Report::Sighting(SightingReport {
            estimated_hex: at,
            estimated_ship_count: 1,
            estimated_kind: ShipKind::Cruiser,
            received_event: EventId::new(event),
            age_pulses: 0,
        })
    };
    belief.apply_report(&sight(1), &figures);
    // Age the first past the age a sighting may refresh a contact at (the
    // pulse it was made in does not age it), so that a second sighting of
    // the same place makes contact 2.
    for _ in 0..=figures.fresh_contact_max_age_pulses + 1 {
        belief.age_contacts(&figures);
    }
    belief.apply_report(&sight(2), &figures);
    belief.age_contacts(&figures);
    assert_eq!(belief.contacts().count(), 2);
    let gone = belief.apply_report(
        &Report::NothingThere {
            contact: None,
            hex: at,
        },
        &figures,
    );
    assert_eq!(gone, ApplyOutcome::Dropped);
    let left: Vec<u32> = belief.contacts().map(|c| c.id.raw()).collect();
    assert_eq!(left, vec![2], "contact 1 is the lowest numbered");
    // A hex it does not name changes nothing.
    assert_eq!(
        belief.apply_report(
            &Report::NothingThere {
                contact: None,
                hex: hx(9, 9)
            },
            &figures
        ),
        ApplyOutcome::Unchanged
    );
}

// ------------------------------------------------ Without report figures

#[test]
fn with_no_report_figures_every_report_reaches_every_command_at_once() {
    let mut world = world_with_levels(&[]);
    assert!(world.ref_data.reports.is_none());
    assert!(!world.side_view(Side::Allied).reports_travel());
    let before = world.events.len();
    make(&mut world, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    for command in [AREA, tf(TF1), tf(TF2), base1()] {
        assert_eq!(contacts_of(&world, command), vec![(1, 2, 0)], "{command}");
    }
    // Only the report's own event: no "report sent", nothing in flight.
    assert_eq!(world.events.len(), before + 1);
    assert!(sent(&world).is_empty());
    assert!(world.reports_in_flight.is_empty());
    // "Nothing there" reaches every picture by its number.
    let key = world.events.next_id();
    let report = Report::NothingThere {
        contact: Some(ContactId::new(1)),
        hex: hx(5, 0),
    };
    let figures_search = world.ref_data.search.clone();
    world.apply_report(
        Side::Allied,
        &report,
        &Holders::Commands {
            commands: vec![tf(TF1)],
            key,
            day: DAY,
            pulse: Some(NOON),
        },
        &figures_search,
    );
    for command in [AREA, tf(TF1), tf(TF2), base1()] {
        assert!(contacts_of(&world, command).is_empty(), "{command}");
    }
}

#[test]
fn with_command_levels_off_only_the_sides_own_belief_changes_and_nothing_is_sent() {
    let mut world = crate::pictures_tests::world();
    world.ref_data.reports = Some(figures(ON_TIME, ON_TIME, ON_TIME, 600));
    assert!(!world.side_view(Side::Allied).reports_travel());
    let key = world.events.next_id();
    let report = sighting_at(key, hx(5, 0), 2);
    let holders = Holders::Commands {
        commands: vec![tf(TF1)],
        key,
        day: DAY,
        pulse: Some(NOON),
    };
    let figures_search = world.ref_data.search.clone();
    world.apply_report(Side::Allied, &report, &holders, &figures_search);
    record_key(&mut world, DAY, NOON);
    world.pass_report(Side::Allied, &report, &holders);
    assert_eq!(world.beliefs[&Side::Allied].contacts().count(), 1);
    assert!(sent(&world).is_empty());
}

// ------------------------------------------------ The event and its view

#[test]
fn a_report_sent_has_its_causes_and_its_breakdown_and_one_draw_for_each_listener() {
    let mut world = travelling((1, 0), (0, 1), (0, 0), 600);
    let key = make(&mut world, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    let events = sent(&world);
    let first = events[0];
    assert_eq!(first.causes, vec![Cause::Event(key)]);
    let breakdown = first.breakdown.as_ref().expect("breakdowns are on");
    // Three listeners, three factors each, in listener order.
    assert_eq!(breakdown.factors.len(), 9);
    let names: Vec<&str> = breakdown.factors.iter().map(|f| f.name).collect();
    assert_eq!(&names[..3], ["on_time_chance", "late_chance", "draw"]);
    // Up on time (1, 0); then sideways late (0, 1) twice.
    assert_eq!(breakdown.factors[0].value, Fixed::ONE);
    assert_eq!(breakdown.factors[1].value, Fixed::ZERO);
    assert_eq!(breakdown.factors[3].value, Fixed::ZERO);
    assert_eq!(breakdown.factors[4].value, Fixed::ONE);
    // The result: listeners that arrive, on time or late.
    assert_eq!(breakdown.result, Fixed::from_int(3));
    // The draw is the die: in [0, 1).
    for at in [2, 5, 8] {
        let roll = breakdown.factors[at].value;
        assert!(roll >= Fixed::ZERO && roll < Fixed::ONE);
    }
    // The event names the task forces and bases it is about.
    let entities = first.kind.entities();
    assert!(entities.contains(&EntityRef::TaskForce(TF1)));
    assert!(entities.contains(&EntityRef::TaskForce(TF2)));
    assert!(entities.contains(&EntityRef::Base(BASE1)));
}

#[test]
fn a_report_sent_is_the_sides_own_and_the_enemy_sees_nothing_of_it() {
    let mut world = travelling(ON_TIME, NEVER, NEVER, 600);
    make(&mut world, &[tf(TF1)], DAY, NOON, |key| {
        sighting_at(key, hx(5, 0), 2)
    });
    let count = |side: Side| {
        world
            .side_view(side)
            .events()
            .iter()
            .filter(|event| {
                matches!(
                    event.kind,
                    crate::side_events::SideEventKind::Whole(EventKind::ReportSent { .. })
                )
            })
            .count()
    };
    assert_eq!(count(Side::Allied), 2);
    assert_eq!(count(Side::Japanese), 0);
}

#[test]
fn the_views_say_whether_reports_travel() {
    let world = travelling(NEVER, NEVER, NEVER, 600);
    let view = world.side_view(Side::Allied);
    assert!(view.reports_travel());
    assert!(view.command(AREA).expect("area").reports_travel());
    let off = crate::pictures_tests::world();
    assert!(!off.side_view(Side::Allied).reports_travel());
    assert!(!off
        .side_view(Side::Allied)
        .command(AREA)
        .expect("area")
        .reports_travel());
}

// --------------------------------------------- State, hash and pruning

#[test]
fn a_report_in_flight_and_a_heard_entry_are_in_the_state_hash() {
    let mut world = travelling(NEVER, LATE, NEVER, 600);
    make(&mut world, &[tf(TF1)], DAY, NOON, claim);
    assert!(!world.reports_in_flight.is_empty());
    let with = world.state_hash();
    let kept = std::mem::take(&mut world.reports_in_flight);
    assert_ne!(with, world.state_hash(), "a report in flight is hashed");
    // Each field of one counts.
    for change in [
        |f: &mut crate::passing::ReportInFlight| f.arrives.1 = Pulse::Night,
        |f: &mut crate::passing::ReportInFlight| f.listener = CommandRef::Area,
        |f: &mut crate::passing::ReportInFlight| f.direction = Direction::Down,
        |f: &mut crate::passing::ReportInFlight| f.made.0 += 1,
    ] {
        let mut changed = kept.clone();
        change(&mut changed[0]);
        world.reports_in_flight = changed;
        assert_ne!(with, world.state_hash());
    }
    world.reports_in_flight = kept;
    assert_eq!(with, world.state_hash());
    // A heard entry.
    let before = world.state_hash();
    world
        .chain
        .as_mut()
        .expect("on")
        .get_mut(&Side::Allied)
        .expect("side")
        .get_mut(&tf(TF2))
        .expect("command")
        .heard
        .insert(EventId::new(99), (DAY, NOON));
    assert_ne!(before, world.state_hash(), "a heard entry is hashed");
}

#[test]
fn what_a_command_heard_is_forgotten_after_as_long_as_a_contact_is_kept() {
    let mut world = travelling(NEVER, NEVER, NEVER, 600);
    make(&mut world, &[tf(TF1)], DAY, NOON, claim);
    let heard = |world: &World| {
        world.commands(Side::Allied).expect("on")[&tf(TF1)]
            .heard()
            .len()
    };
    assert_eq!(heard(&world), 1);
    // Made in the morning of day 1; a drop age of 5 pulses.
    world.prune_heard(DAY, Pulse::Afternoon, 5);
    world.prune_heard(DAY + 1, Pulse::Night, 5);
    world.prune_heard(DAY + 1, Pulse::Morning, 5);
    world.prune_heard(DAY + 1, Pulse::Afternoon, 5);
    assert_eq!(heard(&world), 1, "four pulses on, still held");
    world.prune_heard(DAY + 2, Pulse::Night, 5);
    assert_eq!(heard(&world), 0, "five pulses on, forgotten");
}

#[test]
fn a_command_formed_by_copy_has_heard_what_the_picture_it_copies_has() {
    let mut world = travelling(NEVER, NEVER, NEVER, 600);
    let key = make(&mut world, &[tf(TF1)], DAY, NOON, claim);
    let new = crate::ids::TaskForceId::new(30);
    world.truth.task_forces.insert(
        new,
        crate::state::TaskForce::at(
            new,
            "New".to_string(),
            Side::Allied,
            hx(0, 0),
            crate::orders::Mission::Transport,
        ),
    );
    world
        .truth
        .task_forces
        .get_mut(&new)
        .expect("force")
        .ships
        .insert(crate::ids::ShipId::new(1));
    world.command_formed(new, tf(TF1));
    let heard = world.commands(Side::Allied).expect("on")[&tf(new)]
        .heard()
        .clone();
    assert!(heard.contains_key(&key));
}

#[test]
fn the_draw_and_the_entity_kind_have_codes_of_their_own() {
    assert_eq!(crate::rng::Draw::ReportPassed.code(), 9);
    assert_eq!(crate::rng::Draw::ReportPassedDown.code(), 10);
    let event = EntityRef::Event(EventId::new(3));
    assert_eq!(event.kind_code(), 8);
    assert_eq!(event.raw_id(), 3);
    assert_eq!(event.stable_key(), (8u64 << 32) | 3);
    assert_eq!(Direction::Up.code(), 1);
    assert_eq!(Direction::Sideways.code(), 2);
    assert_eq!(Direction::Down.code(), 3);
    assert_eq!(Arrival::OnTime.code(), 1);
    assert_eq!(Arrival::Late.code(), 2);
    assert_eq!(Arrival::Never.code(), 3);
}

#[test]
fn the_same_start_gives_the_same_passing() {
    let run = || {
        let mut world = travelling((1, 0), (0, 1), (1, 0), 600);
        make(&mut world, &[tf(TF1)], DAY, NOON, claim);
        sent(&world)
            .iter()
            .map(|event| event.kind.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(run(), run());
}

// ------------------------------------------- A contact of its own (decision 0137)

/// The contacts of `command`'s picture marked as its own, by number.
fn own_of(world: &World, command: CommandRef) -> Vec<u32> {
    world.commands(Side::Allied).expect("on")[&command]
        .own()
        .iter()
        .map(|c| c.raw())
        .collect()
}

#[test]
fn a_report_is_its_holders_own_and_told_to_the_rest() {
    let mut world = travelling(ON_TIME, ON_TIME, ON_TIME, 600);
    make(&mut world, &[tf(TF1)], DAY, NOON, |k| {
        sighting_at(k, hx(4, 0), 1)
    });
    assert_eq!(own_of(&world, tf(TF1)), vec![1]);
    for told in [AREA, tf(TF2), base1()] {
        assert_eq!(
            world.commands(Side::Allied).expect("on")[&told]
                .picture()
                .contacts()
                .count(),
            1,
            "{told}"
        );
        assert!(own_of(&world, told).is_empty(), "{told}");
    }
    // A report of the same contact sent to task force 1 by task force 2
    // makes it one task force 1 was told of.
    make(&mut world, &[tf(TF2)], DAY, NOON, |k| {
        sighting_at(k, hx(4, 0), 2)
    });
    assert!(own_of(&world, tf(TF1)).is_empty());
    assert_eq!(own_of(&world, tf(TF2)), vec![1]);
}

#[test]
fn without_report_figures_the_holders_mark_it_their_own_and_a_dropped_contact_loses_it() {
    let mut world = world_with_levels(&[]);
    make(&mut world, &[tf(TF1)], DAY, NOON, |k| {
        sighting_at(k, hx(4, 0), 1)
    });
    assert_eq!(own_of(&world, tf(TF1)), vec![1]);
    assert!(own_of(&world, tf(TF2)).is_empty());
    assert!(own_of(&world, AREA).is_empty());
    let figures = world.ref_data.search.clone();
    for _ in 0..figures.contact_drop_age_pulses + 1 {
        world.age_pictures(Side::Allied, &figures);
    }
    assert!(own_of(&world, tf(TF1)).is_empty());
}

#[test]
fn a_contact_one_command_saw_is_in_its_picture_alone() {
    let mut world = travelling(ON_TIME, ON_TIME, ON_TIME, 600);
    let key = record_key(&mut world, DAY, NOON);
    let figures = world.ref_data.search.clone();
    world.apply_report(
        Side::Allied,
        &sighting_at(key, hx(4, 0), 1),
        &Holders::Only(tf(TF2)),
        &figures,
    );
    assert_eq!(own_of(&world, tf(TF2)), vec![1]);
    for other in [AREA, tf(TF1), base1()] {
        assert_eq!(
            world.commands(Side::Allied).expect("on")[&other]
                .picture()
                .contacts()
                .count(),
            0,
            "{other}"
        );
    }
    assert_eq!(world.beliefs[&Side::Allied].contacts().count(), 1);
}
