//! `measure` against what then happens (decision 0049, decision 3).
//!
//! A passage is asked for before the day on which the force is ordered to
//! steam; the scenario is then run and the answer is compared, exactly, with
//! the events: the hexes entered in order, the miles, the arrival (day, pulse,
//! clock time), and each ship's fuel burned. An air reach is asked for and
//! compared with whether the group launched. The air distance is the vectors
//! of decision 0040; the sea route is the figures of
//! `tests/scenario_04b_pathfinding.rs`. Places, and what a side cannot know,
//! are tested by difference as stage 9b's are.

mod support;

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

use serde_json::Value;
use sim_agent::{measure, measure_json, MeasureAnswer, Passage, PlaceArg, Question, SpeedArg};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::TaskForceId;
use sim_core::orders::{OrderKind, OrderRequest, Origin, TaskForceOrder};
use sim_core::state::{Side, TaskForceStatus};
use sim_core::time::{ClockTime, Pulse};
use sim_core::view::SideView;
use sim_data::{LoadOptions, Scenario};
use support::check::{check_measure, sample_questions};
use support::{load, load_text, scenario_text, schemas, ROOT};

fn asked(view: &SideView<'_>, question: &Question) -> (MeasureAnswer, Value) {
    check_measure(view, question, &schemas(), "measure")
}

fn passage_question(force: u32, to: PlaceArg, speed: Option<SpeedArg>) -> Question {
    Question::Passage {
        task_force: format!("task_force:{force}"),
        to,
        speed,
    }
}

fn passage_of(answer: MeasureAnswer) -> Passage {
    match answer {
        MeasureAnswer::Passage(p) => *p,
        other => panic!("not a passage: {other:?}"),
    }
}

// ---------------------------------------------------------------------
// What happened.
// ---------------------------------------------------------------------

#[derive(Default, Debug)]
struct Observed {
    hexes: Vec<Hex>,
    steamed_nm: Fixed,
    burned: BTreeMap<String, Fixed>,
    arrived: Option<(u32, Pulse, Fixed)>,
    ran_dry: Option<(u32, Pulse, Hex)>,
}

fn observe(events: &[Event], force: TaskForceId) -> Observed {
    let mut seen = Observed::default();
    for event in events {
        match &event.kind {
            EventKind::TaskForceSteamed {
                task_force,
                distance_nm,
                hexes_entered,
                ..
            } if *task_force == force => {
                seen.hexes.extend(hexes_entered);
                seen.steamed_nm += *distance_nm;
            }
            EventKind::FuelBurned {
                ship,
                task_force,
                burned_t,
                ..
            } if *task_force == force => {
                *seen.burned.entry(ship.to_string()).or_default() += *burned_t;
            }
            EventKind::TaskForceArrived {
                task_force,
                elapsed_hours,
                ..
            } if *task_force == force => {
                seen.arrived = Some((
                    event.day,
                    event.pulse.expect("arrival is in a pulse"),
                    *elapsed_hours,
                ));
            }
            EventKind::TaskForceOutOfFuel { task_force, hex } if *task_force == force => {
                seen.ran_dry = Some((event.day, event.pulse.expect("in a pulse"), *hex));
            }
            _ => {}
        }
    }
    seen
}

/// The forecast and what happened agree exactly. Returns the number of
/// quantities compared.
fn assert_agrees(p: &Passage, seen: &Observed, label: &str) -> usize {
    let mut compared = 0;
    // The hexes entered, in order.
    let route: Vec<Hex> = p.route.iter().map(|h| Hex::new(h.q, h.r)).collect();
    assert_eq!(p.route_steps as usize, route.len(), "{label}");
    assert_eq!(
        route[..p.entered_steps as usize],
        seen.hexes[..],
        "{label}: hexes entered"
    );
    // The route ends at the destination; it is empty when the force is
    // already in that hex (a force past the middle of a leg is in the hex
    // ahead) and has only the rest of the leg to steam.
    assert_eq!(
        route
            .last()
            .copied()
            .unwrap_or(Hex::new(p.from.q, p.from.r)),
        Hex::new(p.to.place.q, p.to.place.r),
        "{label}: the route ends at the destination"
    );
    compared += 1;
    // The miles.
    assert_eq!(p.steamed_nm.0, seen.steamed_nm, "{label}: miles steamed");
    compared += 1;
    // Fuel, ship by ship.
    for line in &p.ships {
        let burned = seen
            .burned
            .get(&line.ship.handle)
            .copied()
            .unwrap_or(Fixed::ZERO);
        assert_eq!(
            line.burned_t.0, burned,
            "{label}: fuel burned by {}",
            line.ship.handle
        );
        assert_eq!(line.fuel_t.0 - line.burned_t.0, line.left_t.0, "{label}");
        compared += 1;
    }
    match (p.outcome, seen.arrived, seen.ran_dry) {
        ("arrives", Some((day, pulse, elapsed)), None) => {
            let ends = p.ends.as_ref().expect("an arrival has a moment");
            assert_eq!(ends.day, day, "{label}: arrival day");
            assert_eq!(ends.pulse, pulse.name(), "{label}: arrival pulse");
            assert_eq!(ends.elapsed_hours.0, elapsed, "{label}: arrival hours");
            assert_eq!(
                ends.clock,
                ClockTime::from_elapsed(elapsed).to_string(),
                "{label}: arrival clock"
            );
            assert_eq!(p.route_nm.0, seen.steamed_nm, "{label}: route length");
            assert_eq!(p.entered_steps, p.route_steps, "{label}");
            assert_eq!(p.short_by_nm.0, Fixed::ZERO, "{label}");
            compared += 4;
        }
        ("runs_out_of_fuel", None, Some((day, pulse, hex))) => {
            let ends = p.ends.as_ref().expect("a stop has a moment");
            assert_eq!((ends.day, ends.pulse), (day, pulse.name()), "{label}");
            let at = p.ends_at.as_ref().expect("a stop has a place");
            assert_eq!(Hex::new(at.q, at.r), hex, "{label}: where it stops");
            assert!(p.short_by_nm.0 > Fixed::ZERO, "{label}");
            assert!(p.entered_steps < p.route_steps, "{label}");
            assert_eq!(p.route_nm.0, p.steamed_nm.0 + p.short_by_nm.0, "{label}");
            for line in &p.ships {
                assert_eq!(
                    line.runs_dry_after_nm.map(|d| d.0),
                    line.runs_dry.then_some(seen.steamed_nm),
                    "{label}: {} runs dry after the miles it steamed",
                    line.ship.handle
                );
            }
            assert!(p.ships.iter().any(|l| l.runs_dry), "{label}");
            compared += 4;
        }
        other => panic!("{label}: the forecast and the run disagree on the end: {other:?}"),
    }
    compared
}

struct Setup {
    side: Side,
    force: u32,
    /// The day the force is ordered to steam; the forecast is made at the
    /// start of it.
    order_day: u32,
    to: PlaceArg,
    speed: Option<SpeedArg>,
    /// An order of the test's own, given with the day's, if the scenario
    /// scripts none on that day.
    order: Option<TaskForceOrder>,
}

fn advance_to(scenario: &mut Scenario, order_day: u32) {
    while scenario.world.day() + 1 < order_day {
        scenario
            .next_day()
            .expect("the day resolves")
            .expect("a day");
    }
}

/// Asks for the passage, runs the scenario until the force has arrived or
/// run dry, and checks the answer against the events.
fn compare(mut scenario: Scenario, setup: &Setup, label: &str) -> (Passage, usize) {
    advance_to(&mut scenario, setup.order_day);
    let force_id = TaskForceId::new(setup.force);
    let view = scenario.world.side_view(setup.side);
    let (answer, _) = asked(
        &view,
        &passage_question(setup.force, setup.to.clone(), setup.speed),
    );
    let passage = passage_of(answer);
    assert_eq!(passage.order, "accepted", "{label}");
    let first = scenario.world.events().len();
    if let Some(order) = &setup.order {
        scenario.world.submit_orders(
            setup.side,
            vec![OrderRequest {
                kind: OrderKind::TaskForce {
                    target: force_id,
                    order: order.clone(),
                },
                origin: Origin::PLAYER,
            }],
        );
    }
    loop {
        if scenario.next_day().expect("the day resolves").is_none() {
            break;
        }
        let status = scenario.world.umpire().task_forces[&force_id].status;
        if matches!(
            status,
            TaskForceStatus::Arrived | TaskForceStatus::OutOfFuel
        ) {
            break;
        }
    }
    let events = &scenario.world.events().from_index(first);
    let seen = observe(events, force_id);
    let compared = assert_agrees(&passage, &seen, label);
    (passage, compared)
}

fn setup(side: Side, force: u32, order_day: u32, q: i32, r: i32) -> Setup {
    Setup {
        side,
        force,
        order_day,
        to: PlaceArg::hex(q, r),
        speed: None,
        order: None,
    }
}

// ---------------------------------------------------------------------
// Passages against the scenarios.
// ---------------------------------------------------------------------

#[test]
fn a_passage_is_what_then_happens_in_the_scenarios() {
    let mut compared = 0;
    let mut passages = 0;
    let cases: Vec<(&str, Setup, &str)> = vec![
        (
            "01_transit.toml",
            setup(Side::Allied, 1, 1, 8, 3),
            "01 transit",
        ),
        (
            "01_transit.toml",
            Setup {
                speed: Some(SpeedArg::Cruise),
                ..setup(Side::Allied, 1, 1, 8, 3)
            },
            "01 transit at the speed asked",
        ),
        (
            "01b_refuel.toml",
            setup(Side::Allied, 1, 1, 8, 3),
            "01b refuel, out",
        ),
        (
            "01b_refuel.toml",
            setup(Side::Allied, 1, 3, 0, 0),
            "01b refuel, back",
        ),
        (
            "02_supply_run.toml",
            setup(Side::Allied, 1, 2, 8, 0),
            "02 supply run",
        ),
        (
            "04b_santo_to_lunga.toml",
            setup(Side::Allied, 1, 1, 2, -4),
            "04b Santo to Lunga",
        ),
        (
            "04b_round_the_island.toml",
            setup(Side::Allied, 1, 1, 8, 0),
            "04b round the island",
        ),
        (
            "07_full_slice.toml",
            setup(Side::Allied, 5, 2, 2, -4),
            "07 the convoy",
        ),
        (
            "07_full_slice.toml",
            setup(Side::Allied, 9, 2, 4, 14),
            "07 the dry destroyer",
        ),
    ];
    for (name, setup, label) in cases {
        let (p, n) = compare(load(name, 1942), &setup, label);
        println!(
            "{label}: {} hexes, {} nm, {} {}",
            p.route_steps,
            p.route_nm,
            p.outcome,
            p.ends.as_ref().map_or(String::new(), |m| m.clock.clone())
        );
        compared += n;
        passages += 1;
    }
    println!("{passages} passages, {compared} quantities compared");
    assert_eq!(passages, 9);
}

#[test]
fn the_dry_destroyer_is_named_with_the_miles_it_steams() {
    // Ship 45 has 5 t of fuel and a destroyer burns 500 / 4000 = 0.125 t a
    // mile at cruise speed, so it has 40 nm in it, and the force stops there
    // of the 80 nm to (4,14) (the scenario's own comment says it steams 80;
    // the destination is 80 nm off, and the run, which the forecast matches
    // event by event, steams 40).
    let scenario = load("07_full_slice.toml", 1942);
    let (p, _) = compare(scenario, &setup(Side::Allied, 9, 2, 4, 14), "dry destroyer");
    assert_eq!(p.outcome, "runs_out_of_fuel");
    let dry: Vec<&sim_agent::ShipFuel> = p.ships.iter().filter(|s| s.runs_dry).collect();
    assert_eq!(dry.len(), 1);
    assert_eq!(dry[0].ship.handle, "ship:45");
    assert_eq!(
        dry[0].runs_dry_after_nm.map(|d| d.0),
        Some(Fixed::from_int(40))
    );
    assert_eq!(p.steamed_nm.0, Fixed::from_int(40));
    assert_eq!(p.route_nm.0, Fixed::from_int(80));
    assert_eq!(p.short_by_nm.0, Fixed::from_int(40));
    assert_eq!(dry[0].left_t.0, Fixed::ZERO);
    let mut scenario = load("07_full_slice.toml", 1942);
    advance_to(&mut scenario, 2);
    let text = measure(
        &scenario.world.side_view(Side::Allied),
        &passage_question(9, PlaceArg::hex(4, 14), None),
    )
    .to_text();
    assert!(text.contains("Runs out of fuel"), "{text}");
    assert!(text.contains("runs dry after 40 miles"), "{text}");
}

/// The scenario 1 text with the destroyer given little fuel.
fn little_fuel(tons: i64) -> Scenario {
    let text = scenario_text("01_transit.toml");
    assert!(text.contains("fuel_t = 500"));
    load_text(
        &text.replace("fuel_t = 500", &format!("fuel_t = {tons}")),
        "01_transit.toml",
        1942,
    )
}

#[test]
fn a_ship_that_runs_dry_on_the_way_is_named_by_the_forecast_and_the_run() {
    // The destroyer burns a eighth of a ton a mile; the route is 394 nm
    // (decision 0058), so a destroyer with 45 t, which is 360 nm, runs dry.
    let mut dry_cases = 0;
    for tons in [20, 33, 45, 100] {
        let (p, _) = compare(
            little_fuel(tons),
            &setup(Side::Allied, 1, 1, 8, 3),
            &format!("01 with {tons} t in the destroyer"),
        );
        if p.outcome == "arrives" {
            assert!(
                Fixed::from_int(tons * 8) >= p.route_nm.0,
                "{tons} t is enough for the route"
            );
            continue;
        }
        assert_eq!(p.outcome, "runs_out_of_fuel", "{tons} t");
        let dry: Vec<_> = p.ships.iter().filter(|s| s.runs_dry).collect();
        assert_eq!(dry.len(), 1);
        assert_eq!(dry[0].ship.handle, "ship:2");
        assert_eq!(dry[0].runs_dry_after_nm.map(|d| d.0), Some(p.steamed_nm.0));
        assert!(p.steamed_nm.0 < p.route_nm.0);
        println!("{tons} t: dry after {} of {} nm", p.steamed_nm, p.route_nm);
        dry_cases += 1;
    }
    assert!(dry_cases >= 3);
}

#[test]
fn a_force_that_starts_between_two_hexes_is_forecast_from_where_it_really_is() {
    // Scenario 1's transit force ends day 1 with 30 nm of the leg from (8,2)
    // to (8,3), which is charged 40, steamed, so it is in (8,3), between two
    // centres (decision 0058: the round-the-island force of scenario 4b now
    // arrives on day 1, so it no longer stands between hexes at the end of
    // it). Orders to different places make it go on or turn back; the
    // forecast makes the engine's choice (decision 0005, 5) and the run
    // agrees.
    let mut turned = 0;
    let mut went_on = 0;
    for (q, r) in [(8, 3), (8, 0), (0, 0), (8, 2), (4, 1), (6, 3)] {
        let mut scenario = load("01_transit.toml", 1942);
        advance_to(&mut scenario, 2);
        let force = scenario.world.umpire().task_forces[&TaskForceId::new(1)].clone();
        assert!(force.toward_hex.is_some(), "the force is between two hexes");
        let label = format!("transit, to ({q},{r}) from between hexes");
        let (turns, centre) = force
            .reroute_centre(scenario.world.map(), Some(Hex::new(q, r)))
            .expect("reachable");
        let (p, _) = compare(
            scenario,
            &Setup {
                order: Some(TaskForceOrder::SetRoute(vec![Hex::new(q, r)])),
                ..setup(Side::Allied, 1, 2, q, r)
            },
            &label,
        );
        assert!(p.between_hexes, "{label}");
        assert_eq!(
            (p.route_starts_at.q, p.route_starts_at.r, p.turns_back),
            (centre.q, centre.r, turns),
            "{label}"
        );
        if p.turns_back {
            turned += 1;
        } else {
            went_on += 1;
        }
    }
    assert!(
        turned > 0 && went_on > 0,
        "{turned} turned, {went_on} went on"
    );
}

#[test]
fn a_force_ordered_back_mid_voyage_in_the_full_slice_is_forecast_by_the_turn_back_rule() {
    // Scenario 7 sends force A north on day 5 and back on day 6, when it is
    // 24 nm along a leg between two hexes (see the scenario's comment).
    let scenario = load("07_full_slice.toml", 1942);
    let (p, _) = compare(
        scenario,
        &setup(Side::Japanese, 6, 6, -8, 10),
        "07 force A turned back",
    );
    assert!(p.between_hexes);
    println!(
        "turned back: {}, starts from ({},{})",
        p.turns_back, p.route_starts_at.q, p.route_starts_at.r
    );
}

#[test]
fn a_speed_asked_for_is_limited_as_the_engine_limits_it() {
    // The cruiser (class 1) steams at 16 kn cruise and 32 kn full, the
    // destroyer (class 2) at its own figures. The slowest ship sets the
    // speed, and a speed in knots is limited by each ship's top speed.
    let view_of = |name: &str| load(name, 1942);
    let scenario = view_of("01_transit.toml");
    let view = scenario.world.side_view(Side::Allied);
    let speed_of = |speed: Option<SpeedArg>| {
        let (answer, _) = asked(&view, &passage_question(1, PlaceArg::hex(8, 3), speed));
        let p = passage_of(answer);
        (
            p.speed.knots.0,
            p.speed.slowest_ship.as_ref().map(|s| s.handle.clone()),
            p,
        )
    };
    let (cruise, _, p_cruise) = speed_of(Some(SpeedArg::Cruise));
    let (full, _, p_full) = speed_of(Some(SpeedArg::Full));
    let (ten, ship, _) = speed_of(Some(SpeedArg::Knots(Fixed::from_int(10))));
    let (huge, _, _) = speed_of(Some(SpeedArg::Knots(Fixed::from_int(99))));
    assert!(ten < cruise && cruise < full, "{ten} {cruise} {full}");
    assert_eq!(ten, Fixed::from_int(10));
    assert!(huge <= full, "a speed above every ship's top is limited");
    assert!(ship.is_some());
    // A faster passage takes fewer hours and burns at least as much fuel.
    assert!(p_full.steaming_hours.0 < p_cruise.steaming_hours.0);
    for (a, b) in p_full.ships.iter().zip(&p_cruise.ships) {
        assert!(a.burned_t.0 >= b.burned_t.0, "{}", a.ship.handle);
    }
    // Speeds given as text or whole numbers read the same.
    let q = |speed: &str| {
        Question::from_json(&format!(
            r#"{{"question":"passage","task_force":"task_force:1","to":{{"q":8,"r":3}},"speed":{speed}}}"#
        ))
    };
    assert_eq!(
        q("\"cruise\"").unwrap(),
        passage_question(1, PlaceArg::hex(8, 3), Some(SpeedArg::Cruise))
    );
    assert_eq!(
        q("10").unwrap(),
        passage_question(
            1,
            PlaceArg::hex(8, 3),
            Some(SpeedArg::Knots(Fixed::from_int(10)))
        )
    );
    assert!(q("\"10.5\"").is_ok());
    assert!(q("0").is_err());
    assert!(q("-3").is_err());
    assert!(q("10.5").is_err(), "a JSON float is no speed");
    assert!(q("\"fast\"").is_err());
}

#[test]
fn a_force_ordered_where_it_already_is_would_have_its_order_refused() {
    let scenario = load("04b_round_the_island.toml", 1942);
    let view = scenario.world.side_view(Side::Allied);
    let (answer, _) = asked(&view, &passage_question(1, PlaceArg::hex(0, 0), None));
    let p = passage_of(answer);
    assert_eq!(p.order, "refused");
    assert_eq!(p.refusal, Some("already_there"));
    assert_eq!(p.outcome, "does_not_sail");
    assert!(p.route.is_empty() && p.ends.is_none());
    // Land, or a hex no ship can reach: no route.
    let (answer, _) = asked(&view, &passage_question(1, PlaceArg::hex(4, 0), None));
    let p = passage_of(answer);
    assert_eq!((p.order, p.refusal), ("refused", Some("no_route")));
    let text = p.to_text();
    assert!(text.contains("refused, no route"), "{text}");
}

// ---------------------------------------------------------------------
// Air reach against the launches.
// ---------------------------------------------------------------------

fn launches(events: &[Event]) -> Vec<(String, Hex, Option<Fixed>)> {
    events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::StrikeLaunched {
                air_group,
                hex,
                distance_nm,
                ..
            }
            | EventKind::BaseStrikeLaunched {
                air_group,
                hex,
                distance_nm,
                ..
            } => Some((air_group.to_string(), *hex, Some(*distance_nm))),
            EventKind::EscortsSent { air_group, .. } => {
                Some((air_group.to_string(), Hex::new(0, 0), None))
            }
            _ => None,
        })
        .collect()
}

#[test]
fn eastern_solomons_the_reach_of_the_carrier_groups_agrees_with_their_launches() {
    // The American carriers do not move in scenario 5, so the view at the
    // start of the day is where they launch from: for each Allied group and
    // each hex any Allied strike was sent to in seeds 1 to 20, the answer is
    // within for every hex it was sent to and, wherever the answer is beyond,
    // the group never launched there.
    let name = "05_eastern_solomons.toml";
    let mut targets: Vec<Hex> = Vec::new();
    let mut sent: BTreeMap<(String, Hex), Fixed> = BTreeMap::new();
    let mut japanese: Vec<(u32, Hex, Fixed, Vec<Event>)> = Vec::new();
    for seed in 1..=20 {
        let mut scenario = load(name, seed);
        scenario.run_to_end().expect("the run resolves");
        let allied: Vec<String> = scenario
            .world
            .side_view(Side::Allied)
            .air_groups()
            .map(|g| g.id.to_string())
            .collect();
        for (group, hex, distance) in launches(scenario.world.events().from_index(0)) {
            let Some(distance) = distance else { continue };
            if allied.contains(&group) {
                sent.insert((group, hex), distance);
                if !targets.contains(&hex) {
                    targets.push(hex);
                }
            } else {
                japanese.push((seed as u32, hex, distance, Vec::new()));
            }
        }
    }
    assert!(!sent.is_empty());
    let scenario = load(name, 1);
    let view = scenario.world.side_view(Side::Allied);
    let (mut within, mut beyond) = (0, 0);
    for group in view.air_groups() {
        let (answer, _) = asked(
            &view,
            &Question::AirReach {
                air_group: group.id.to_string(),
                to: targets.iter().map(|h| PlaceArg::hex(h.q, h.r)).collect(),
            },
        );
        let MeasureAnswer::AirReach(reach) = answer else {
            panic!("not an air reach")
        };
        for line in &reach.places {
            let hex = Hex::new(line.to.place.q, line.to.place.r);
            let launched = sent.get(&(group.id.to_string(), hex));
            if let Some(distance) = launched {
                assert!(line.within, "{} launched at {hex:?}", group.id);
                assert_eq!(*distance, Fixed::from_int(i64::from(line.distance_nm)));
                within += 1;
            }
            if !line.within {
                assert!(launched.is_none(), "{} launched beyond its reach", group.id);
                beyond += 1;
            }
        }
    }
    println!("05 Allied: {within} launches within reach, {beyond} (group, hex) pairs beyond");
    assert!(within > 0 && beyond > 0);
    let _ = japanese;
}

#[test]
fn eastern_solomons_the_japanese_launches_are_at_the_distance_measure_gives_from_where_the_carrier_was(
) {
    // The Japanese carriers steam, so their groups are not where the view at
    // the start of the day puts them when they launch. Their position at the
    // launch is rebuilt from the steaming events, and the air distance that
    // `measure` gives from there to the hex sent to is the event's distance,
    // within the group's radius.
    let name = "05_eastern_solomons.toml";
    let mut checked = 0;
    for seed in 1..=20 {
        let mut scenario = load(name, seed);
        let view = scenario.world.side_view(Side::Japanese);
        let mut position: BTreeMap<String, Hex> = view
            .task_forces()
            .map(|f| (f.id.to_string(), f.position()))
            .collect();
        let carrier_of: BTreeMap<String, String> = view
            .air_groups()
            .filter_map(|g| match g.location {
                sim_core::state::AirGroupLocation::Ship(ship) => {
                    let force =
                        view.ships()
                            .find(|s| s.id == ship)
                            .and_then(|s| match s.location {
                                sim_core::state::ShipLocation::TaskForce(f) => Some(f.to_string()),
                                _ => None,
                            })?;
                    Some((g.id.to_string(), force))
                }
                sim_core::state::AirGroupLocation::Base(_) => None,
            })
            .collect();
        let radius: BTreeMap<String, Fixed> = view
            .air_groups()
            .filter_map(|g| {
                view.aircraft_type(g.aircraft_type)
                    .map(|t| (g.id.to_string(), t.strike_range_nm))
            })
            .collect();
        scenario.run_to_end().expect("the run resolves");
        let view = scenario.world.side_view(Side::Japanese);
        for event in scenario.world.events().from_index(0) {
            match &event.kind {
                EventKind::TaskForceSteamed { task_force, to, .. } => {
                    if position.contains_key(&task_force.to_string()) {
                        position.insert(task_force.to_string(), *to);
                    }
                }
                EventKind::StrikeLaunched {
                    air_group,
                    hex,
                    distance_nm,
                    ..
                } => {
                    let group = air_group.to_string();
                    let Some(force) = carrier_of.get(&group) else {
                        continue;
                    };
                    let origin = position[force];
                    let (answer, _) = asked(
                        &view,
                        &Question::AirDistance {
                            from: PlaceArg::hex(origin.q, origin.r),
                            to: PlaceArg::hex(hex.q, hex.r),
                        },
                    );
                    let MeasureAnswer::AirDistance(d) = answer else {
                        panic!("not an air distance")
                    };
                    assert_eq!(Fixed::from_int(i64::from(d.distance_nm)), *distance_nm);
                    assert!(*distance_nm <= radius[&group], "{group}");
                    checked += 1;
                }
                _ => {}
            }
        }
    }
    println!("05 Japanese: {checked} launches' distances agree");
    assert!(checked > 0);
}

const RABAUL: (i32, i32) = (-5, -13);
const HENDERSON: (i32, i32) = (2, -4);

/// The frozen reference folder with `types` applied to a copy of the aircraft
/// types the Rabaul scenario names, under the build directory.
fn variant_options(tag: &str, types: &dyn Fn(String) -> String) -> LoadOptions {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let frozen = Path::new(ROOT).join(sim_data::FROZEN_REFERENCE_DIR);
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "measure_{tag}_{}_{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&root).expect("make the folder");
    for entry in fs::read_dir(&frozen).expect("the frozen folder reads") {
        let entry = entry.expect("an entry");
        if entry.file_name().to_string_lossy().ends_with(".md") {
            continue;
        }
        fs::copy(entry.path(), root.join(entry.file_name())).expect("copy a frozen file");
    }
    let types_file = root.join("aircraft_types_rabaul_guadalcanal.csv");
    let original = fs::read_to_string(&types_file).expect("read the types");
    fs::write(&types_file, types(original)).expect("write the types");
    LoadOptions::with_reference_root(root)
}

fn replace_once(text: String, old: &str, new: &str) -> String {
    assert_eq!(text.matches(old).count(), 1, "{old}");
    text.replace(old, new)
}

fn rabaul_variant(a6m2: i64, g4m1: i64, seed: u64) -> Scenario {
    let options = variant_options(&format!("a{a6m2}_g{g4m1}"), &|text| {
        let text = replace_once(
            text,
            ",fighter,556,20,556,none,",
            &format!(",fighter,{a6m2},20,{a6m2},none,"),
        );
        replace_once(
            text,
            ",bomber,611,20,611,bomb,",
            &format!(",bomber,{g4m1},20,{g4m1},bomb,"),
        )
    });
    Scenario::from_text_with(
        &scenario_text("06_rabaul_guadalcanal.toml"),
        &support::scenario_path("06_rabaul_guadalcanal.toml"),
        &options,
    )
    .expect("the variant loads")
    .with_seed(seed)
    .expect("a seed")
}

/// What group `group` of the Japanese side measures to Henderson Field, and
/// whether it launched there, in each of seeds 1 to 20.
fn rabaul_reach(a6m2: i64, g4m1: i64, group: u32) -> (bool, u32, Fixed, usize) {
    let mut reach = None;
    let mut launched = 0;
    for seed in 1..=20 {
        let mut scenario = rabaul_variant(a6m2, g4m1, seed);
        if seed == 1 {
            let view = scenario.world.side_view(Side::Japanese);
            let (answer, _) = asked(
                &view,
                &Question::AirReach {
                    air_group: format!("air_group:{group}"),
                    to: vec![PlaceArg::hex(HENDERSON.0, HENDERSON.1)],
                },
            );
            let MeasureAnswer::AirReach(r) = answer else {
                panic!("not an air reach")
            };
            assert_eq!(r.places[0].distance_nm, 556);
            reach = Some((
                r.places[0].within,
                r.places[0].distance_nm,
                r.radius_nm.0,
                r,
            ));
            if group == 2 {
                let r = &reach.as_ref().unwrap().3;
                assert_eq!(r.radius_is, "escort");
                assert_eq!(r.escort_radius_nm, Some(r.radius_nm));
                assert!(r.to_text().contains("also its escort radius"));
            }
        }
        scenario.run_to_end().expect("the run resolves");
        let events = launches(scenario.world.events().from_index(0));
        launched += events
            .iter()
            .filter(|(g, _, _)| *g == format!("air_group:{group}"))
            .count();
    }
    let (within, distance, radius, _) = reach.expect("asked");
    (within, distance, radius, launched)
}

#[test]
fn rabaul_to_henderson_is_556_and_the_radius_variants_flip_the_answer_as_they_flip_the_launches() {
    // Group 1, the G4M1s, strikes Henderson Field; group 2, the A6M2s,
    // escorts (decision 0030). The radius comparison is `<=`: the A6M2 at 556
    // reaches 556 with nothing to spare.
    let mut cases = 0;
    // The scenario as it is: both reach.
    let (within, distance, radius, bombers) = rabaul_reach(556, 611, 1);
    assert_eq!(
        (within, distance, radius),
        (true, 556, Fixed::from_int(611))
    );
    assert!(bombers > 0, "the G4M1s launch raids");
    let (within, distance, radius, escorts) = rabaul_reach(556, 611, 2);
    assert_eq!(
        (within, distance, radius),
        (true, 556, Fixed::from_int(556))
    );
    assert!(escorts > 0, "the A6M2s fly escort");
    cases += 2;
    // The G4M1 at 513: beyond, and no raid is ever launched.
    let (within, _, radius, bombers) = rabaul_reach(556, 513, 1);
    assert_eq!((within, radius), (false, Fixed::from_int(513)));
    assert_eq!(bombers, 0, "no raid is launched at 513");
    cases += 1;
    // The A6M2 at 261: beyond, and no escort is ever sent.
    let (within, _, radius, escorts) = rabaul_reach(261, 611, 2);
    assert_eq!((within, radius), (false, Fixed::from_int(261)));
    assert_eq!(escorts, 0, "no escort is sent at 261");
    cases += 1;
    // One mile less than the distance is the edge: beyond. Exactly equal is
    // within (checked above at 556).
    let (within, _, _, escorts) = rabaul_reach(555, 611, 2);
    assert!(!within);
    assert_eq!(escorts, 0);
    cases += 1;
    println!("{cases} reach cases agree with the launches, 20 seeds each");
}

// ---------------------------------------------------------------------
// Air distance and the sea route.
// ---------------------------------------------------------------------

fn air(view: &SideView<'_>, from: PlaceArg, to: PlaceArg) -> u32 {
    let (answer, _) = asked(view, &Question::AirDistance { from, to });
    match answer {
        MeasureAnswer::AirDistance(a) => a.distance_nm,
        other => panic!("{other:?}"),
    }
}

#[test]
fn the_air_distance_has_the_vectors_of_decision_0040_and_is_symmetric() {
    let scenario = load("06_rabaul_guadalcanal.toml", 1942);
    let view = scenario.world.side_view(Side::Allied);
    let o = Hex::new(0, 0);
    // The vectors of the decision's test in hex.rs, on the real map's hexes
    // where the map has them.
    let vectors = [
        (Hex::new(1, -6), 223),
        (Hex::new(5, -6), 223),
        (Hex::new(1, -9), 342),
        (Hex::new(5, -9), 312),
        (Hex::new(3, 0), 120),
        (Hex::new(0, 4), 160),
    ];
    for (hex, nm) in vectors {
        if view.hex(hex).is_none() || view.hex(o).is_none() {
            continue;
        }
        let a = air(&view, PlaceArg::hex(o.q, o.r), PlaceArg::hex(hex.q, hex.r));
        let b = air(&view, PlaceArg::hex(hex.q, hex.r), PlaceArg::hex(o.q, o.r));
        assert_eq!((a, b), (nm, nm), "{hex:?}");
        assert_eq!(a, o.air_distance_nm(hex));
    }
    // Rabaul to Henderson Field, the decision's example, by hex and by unit.
    assert_eq!(
        air(
            &view,
            PlaceArg::hex(RABAUL.0, RABAUL.1),
            PlaceArg::hex(HENDERSON.0, HENDERSON.1)
        ),
        556
    );
    assert_eq!(
        air(
            &view,
            PlaceArg::hex(HENDERSON.0, HENDERSON.1),
            PlaceArg::hex(RABAUL.0, RABAUL.1)
        ),
        556
    );
    // Along an axis it is the hex count times 40, and a place to itself is 0.
    for (a, b) in [((2, -4), (6, -4)), ((2, -4), (2, 0)), ((2, -4), (6, -8))] {
        let (a, b) = (Hex::new(a.0, a.1), Hex::new(b.0, b.1));
        if view.hex(a).is_some() && view.hex(b).is_some() {
            assert_eq!(
                air(&view, PlaceArg::hex(a.q, a.r), PlaceArg::hex(b.q, b.r)),
                40 * a.distance(b)
            );
        }
    }
    assert_eq!(air(&view, PlaceArg::hex(2, -4), PlaceArg::hex(2, -4)), 0);
    // Every pair of the side's bases and forces, both ways round.
    for a in ["base:1"] {
        for b in view.air_groups().map(|g| g.id.to_string()) {
            let one = air(&view, PlaceArg::unit(a), PlaceArg::unit(&b));
            let two = air(&view, PlaceArg::unit(&b), PlaceArg::unit(a));
            assert_eq!(one, two, "{a} {b}");
        }
    }
}

fn route(view: &SideView<'_>, from: (i32, i32), to: (i32, i32)) -> sim_agent::SeaRoute {
    let (answer, _) = asked(
        view,
        &Question::SeaRoute {
            from: PlaceArg::hex(from.0, from.1),
            to: PlaceArg::hex(to.0, to.1),
        },
    );
    match answer {
        MeasureAnswer::SeaRoute(r) => r,
        other => panic!("{other:?}"),
    }
}

#[test]
fn the_sea_route_is_the_path_of_scenario_4b_pathfinding() {
    // Round the island (tests/scenario_04b_pathfinding.rs, part 1): south of
    // the island, ten steps, 374 nm charged (five detour steps of 40, then
    // 35, 33, 26, 40 and 40: decision 0058).
    let scenario = load("04b_round_the_island.toml", 1942);
    let view = scenario.world.side_view(Side::Allied);
    let r = route(&view, (0, 0), (8, 0));
    let hexes: Vec<(i32, i32)> = r.hexes.iter().map(|h| (h.q, h.r)).collect();
    assert_eq!(
        hexes,
        vec![
            (1, 0),
            (2, 0),
            (3, 0),
            (3, 1),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 2),
            (7, 1),
            (8, 0)
        ]
    );
    assert_eq!((r.steps, r.distance_nm), (Some(10), Some(374)));
    assert!(r.route_found);
    // Land has no route, and a hex off the map is not known.
    let none = route(&view, (0, 0), (4, 0));
    assert!(!none.route_found && none.hexes.is_empty());
    assert_eq!((none.steps, none.distance_nm), (None, None));
    assert!(none.to_text().contains("none, ships cannot get"));
    // The same hex: a route of no steps.
    let same = route(&view, (0, 0), (0, 0));
    assert!(same.route_found && same.hexes.is_empty());
    assert_eq!(same.steps, Some(0));

    // The real map: Espiritu Santo to Lunga Point is 16 steps, 640 nm
    // ((7,6) to (2,-4)), and Rabaul to Lunga Point 16 steps ((-5,-13) to
    // (2,-4)); Noumea to Espiritu Santo 13 steps, 520 nm ((0,18) to (7,6)).
    let scenario = load("04b_santo_to_lunga.toml", 1942);
    let view = scenario.world.side_view(Side::Allied);
    // Steps and the miles a ship is charged for them (decision 0058): 629,
    // 556 and 456 (the tests/scenario_04b_pathfinding.rs table).
    for (from, to, steps, miles) in [
        ((7, 6), (2, -4), 16, 629),
        ((-5, -13), (2, -4), 16, 556),
        ((0, 18), (7, 6), 13, 456),
    ] {
        let r = route(&view, from, to);
        assert_eq!(r.steps, Some(steps), "{from:?} to {to:?}");
        assert_eq!(r.distance_nm, Some(miles));
        assert_eq!(
            r.hexes.last().map(|h| (h.q, h.r)),
            Some(to),
            "the path ends at the target"
        );
        // Each step is to a neighbour, and is the map's own next step.
        let mut here = Hex::new(from.0, from.1);
        for h in &r.hexes {
            let next = Hex::new(h.q, h.r);
            assert_eq!(here.distance(next), 1);
            here = next;
        }
        // Never shorter than the straight line (decision 0017).
        assert!(
            r.distance_nm.unwrap()
                >= Hex::new(from.0, from.1).air_distance_nm(Hex::new(to.0, to.1))
        );
    }
    // The forecast steams the same hexes: the passage of the force that is
    // there is its route, and the sea route agrees with it.
    let (answer, _) = asked(&view, &passage_question(1, PlaceArg::hex(2, -4), None));
    let p = passage_of(answer);
    let r = route(&view, (7, 6), (2, -4));
    assert_eq!(p.route, r.hexes);
    assert_eq!(p.route_nm.0, Fixed::from_int(629));
    assert_eq!(r.distance_nm, Some(629));
}

// ---------------------------------------------------------------------
// Places.
// ---------------------------------------------------------------------

#[test]
fn every_kind_of_place_resolves_to_its_hex() {
    let mut scenario = load("05_eastern_solomons.toml", 1942);
    scenario.run_to_end().expect("the run resolves");
    let view = scenario.world.side_view(Side::Allied);
    let force = view
        .task_forces()
        .find(|f| f.id.raw() == 1)
        .expect("a force");
    let ship = view
        .ships()
        .find(
            |s| matches!(s.location, sim_core::state::ShipLocation::TaskForce(f) if f == force.id),
        )
        .expect("a ship in it");
    let group = view
        .air_groups()
        .find(|g| g.location == sim_core::state::AirGroupLocation::Ship(ship.id))
        .expect("an air group aboard");
    let base = view.bases().next().expect("a base");
    let contact = view.contacts().next().expect("a contact");
    let anchor = PlaceArg::hex(0, 0);
    let from = |arg: PlaceArg| {
        let (answer, _) = asked(
            &view,
            &Question::AirDistance {
                from: arg,
                to: anchor.clone(),
            },
        );
        match answer {
            MeasureAnswer::AirDistance(a) => a,
            other => panic!("{other:?}"),
        }
    };
    let cases = [
        (PlaceArg::hex(7, -5), (7, -5), None),
        (
            PlaceArg::unit(&force.id.to_string()),
            (force.position().q, force.position().r),
            Some(force.id.to_string()),
        ),
        (
            PlaceArg::unit(&ship.id.to_string()),
            (force.position().q, force.position().r),
            Some(ship.id.to_string()),
        ),
        (
            PlaceArg::unit(&group.id.to_string()),
            (force.position().q, force.position().r),
            Some(group.id.to_string()),
        ),
        (
            PlaceArg::unit(&base.id.to_string()),
            (base.hex.q, base.hex.r),
            Some(base.id.to_string()),
        ),
        (
            PlaceArg::unit(&contact.id.to_string()),
            (contact.estimated_hex.q, contact.estimated_hex.r),
            Some(contact.id.to_string()),
        ),
    ];
    for (arg, hex, unit) in cases {
        let a = from(arg.clone());
        assert_eq!((a.from.place.q, a.from.place.r), hex, "{arg:?}");
        assert_eq!(a.from.unit.map(|u| u.handle), unit, "{arg:?}");
    }
    // The base's place carries the base's name, as elsewhere.
    assert_eq!(
        from(PlaceArg::unit(&base.id.to_string())).from.place.name,
        Some(base.name.clone())
    );
}

#[test]
fn a_contact_is_the_hex_the_side_believes_not_the_truth() {
    // Find a moment when a contact's estimate is not where the unit truly is,
    // and show that measure uses the estimate.
    let mut found = 0;
    for name in [
        "05_eastern_solomons.toml",
        "07_full_slice.toml",
        "04a_night_action.toml",
    ] {
        for seed in 1..=6 {
            let mut scenario = load(name, seed);
            while scenario.next_day().expect("resolves").is_some() {
                for side in [Side::Allied, Side::Japanese] {
                    let view = scenario.world.side_view(side);
                    let truth = scenario.world.umpire();
                    for contact in view.contacts() {
                        // The nearest enemy force of the same side's enemy.
                        let enemy_hexes: Vec<Hex> = truth
                            .task_forces
                            .values()
                            .filter(|f| f.side != side)
                            .map(|f| f.position())
                            .collect();
                        if enemy_hexes.contains(&contact.estimated_hex) {
                            continue;
                        }
                        // The estimate is off every enemy force: the hex the
                        // answer shows must be the estimate.
                        let (answer, _) = asked(
                            &view,
                            &Question::AirDistance {
                                from: PlaceArg::unit(&contact.id.to_string()),
                                to: PlaceArg::hex(contact.estimated_hex.q, contact.estimated_hex.r),
                            },
                        );
                        let MeasureAnswer::AirDistance(a) = answer else {
                            panic!("not an air distance")
                        };
                        assert_eq!(
                            (a.from.place.q, a.from.place.r),
                            (contact.estimated_hex.q, contact.estimated_hex.r)
                        );
                        assert_eq!(a.distance_nm, 0);
                        found += 1;
                    }
                }
                if found > 0 {
                    break;
                }
            }
            if found > 0 {
                break;
            }
        }
        if found > 0 {
            break;
        }
    }
    assert!(found > 0, "a contact whose estimate is not an enemy's hex");
}

#[test]
fn a_contact_off_by_a_hex_is_measured_at_the_estimate() {
    // Built by hand on scenario 5: the Japanese carriers are at known hexes,
    // and the Allied side is given a starting contact one hex away from the
    // truth. The answer shows the estimate.
    let text = scenario_text("05_eastern_solomons.toml");
    let scenario = load_text(&text, "05_eastern_solomons.toml", 1942);
    let truth = scenario.world.umpire();
    let enemy = truth
        .task_forces
        .values()
        .find(|f| f.side == Side::Japanese)
        .expect("an enemy force");
    let real = enemy.position();
    let off = Hex::new(real.q + 1, real.r);
    // The scenario's own contacts: use the view of a run, which holds the
    // contacts the search makes, and find one whose estimate differs from
    // every enemy hex by at least a hex. A contact's last report is at the
    // true hex or one beside it, so which seed has one beside depends on the
    // scenario's orders; with Ryujo struck by six groups (the Eastern
    // Solomons orders of Nov 12, 2026) the first seed whose held contact is
    // a hex off is searched for, among the first 50 (seed 1 no longer is).
    let mut checked = 0;
    for seed in 1..=50 {
        let mut scenario = load_text(&text, "05_eastern_solomons.toml", seed);
        scenario.run_to_end().expect("runs");
        let view = scenario.world.side_view(Side::Allied);
        let truth = scenario.world.umpire();
        for contact in view.contacts() {
            let nearest = truth
                .task_forces
                .values()
                .filter(|f| f.side == Side::Japanese)
                .map(|f| f.position().distance(contact.estimated_hex))
                .min()
                .unwrap_or(0);
            let (answer, data) = asked(
                &view,
                &Question::AirDistance {
                    from: PlaceArg::unit(&contact.id.to_string()),
                    to: PlaceArg::hex(real.q, real.r),
                },
            );
            let MeasureAnswer::AirDistance(a) = answer else {
                panic!("not an air distance")
            };
            // The distance is measured from the estimate, whatever the truth.
            assert_eq!(a.distance_nm, contact.estimated_hex.air_distance_nm(real));
            assert_eq!(data["from"]["place"]["q"], contact.estimated_hex.q);
            checked += usize::from(nearest > 0);
        }
        if checked > 0 {
            break;
        }
    }
    let _ = off;
    assert!(
        checked > 0,
        "a contact whose estimate is a hex or more from every enemy force"
    );
}

#[test]
fn what_the_side_does_not_know_is_one_and_the_same_answer() {
    for name in [
        "05_eastern_solomons.toml",
        "07_full_slice.toml",
        "06a_base_raid.toml",
    ] {
        let mut scenario = load(name, 1942);
        scenario.run_to_end().expect("the run resolves");
        let truth_units = {
            let truth = scenario.world.umpire();
            let mut enemy: Vec<(Side, String)> = Vec::new();
            for side in [Side::Allied, Side::Japanese] {
                let theirs = |s: Side| s != side;
                enemy.extend(
                    truth
                        .ships
                        .values()
                        .filter(|u| theirs(u.side))
                        .map(|u| (side, u.id.to_string())),
                );
                enemy.extend(
                    truth
                        .task_forces
                        .values()
                        .filter(|u| theirs(u.side))
                        .map(|u| (side, u.id.to_string())),
                );
                enemy.extend(
                    truth
                        .air_groups
                        .values()
                        .filter(|u| theirs(u.side))
                        .map(|u| (side, u.id.to_string())),
                );
                // An enemy base is a place the side knows (decision 0050, as
                // amended): see `an_enemy_base_is_a_place_by_its_public_facts`.
            }
            enemy
        };
        for side in [Side::Allied, Side::Japanese] {
            let view = scenario.world.side_view(side);
            let mut handles: Vec<String> = truth_units
                .iter()
                .filter(|(s, _)| *s == side)
                .map(|(_, h)| h.clone())
                .collect();
            for kind in [
                "ship",
                "task_force",
                "air_group",
                "base",
                "contact",
                "pilot",
            ] {
                handles.push(format!("{kind}:99999"));
                handles.push(format!("{kind}:0"));
            }
            handles.extend(
                ["", "banana", "ship:", "ship:-1", "ship:x", ":3", "ship:1:2"].map(String::from),
            );
            // A force of its own if it has one; else the question is asked
            // of a number no side has (not known all the same).
            let own_force = view
                .task_forces()
                .next()
                .map_or_else(|| "task_force:99998".to_string(), |f| f.id.to_string());
            let own_group = view.air_groups().next().map(|g| g.id.to_string());
            let own_base = view
                .bases()
                .next()
                .map(|b| b.id.to_string())
                .or_else(|| view.task_forces().next().map(|f| f.id.to_string()))
                .or_else(|| own_group.clone())
                .expect("a unit");
            let off_map = PlaceArg::hex(100_000, 100_000);
            let mut places: Vec<PlaceArg> = handles.iter().map(|h| PlaceArg::unit(h)).collect();
            places.push(off_map.clone());
            places.push(PlaceArg::hex(i32::MIN, i32::MAX));
            let mut answers: Vec<String> = Vec::new();
            for place in &places {
                let known = PlaceArg::unit(&own_base);
                for question in [
                    Question::AirDistance {
                        from: place.clone(),
                        to: known.clone(),
                    },
                    Question::AirDistance {
                        from: known.clone(),
                        to: place.clone(),
                    },
                    Question::SeaRoute {
                        from: place.clone(),
                        to: known.clone(),
                    },
                    Question::SeaRoute {
                        from: known.clone(),
                        to: place.clone(),
                    },
                    Question::Passage {
                        task_force: own_force.clone(),
                        to: place.clone(),
                        speed: None,
                    },
                    Question::Passage {
                        task_force: match place {
                            PlaceArg::Unit(h) => h.clone(),
                            PlaceArg::Hex { .. } => "task_force:99999".to_string(),
                        },
                        to: known.clone(),
                        speed: Some(SpeedArg::Full),
                    },
                ] {
                    let (_, data) = asked(&view, &question);
                    // Every one names an enemy's unit, a missing one, a hex off
                    // the map or a bad handle.
                    assert_eq!(data["kind"], "not_known", "{name} {side:?} {question:?}");
                    answers.push(data.to_string());
                }
                if let Some(group) = &own_group {
                    let (_, data) = asked(
                        &view,
                        &Question::AirReach {
                            air_group: group.clone(),
                            to: vec![place.clone()],
                        },
                    );
                    assert_eq!(data["kind"], "not_known", "{name} {side:?} {place:?}");
                    answers.push(data.to_string());
                }
            }
            // An enemy group, a missing group and a bad handle asked about.
            for h in &handles {
                let question = Question::AirReach {
                    air_group: h.clone(),
                    to: vec![PlaceArg::unit(&own_base)],
                };
                let (_, data) = asked(&view, &question);
                assert_eq!(data["kind"], "not_known", "{name} {side:?} {h:?}");
                answers.push(data.to_string());
            }
            let first = answers.first().expect("some answers").clone();
            assert!(first.contains("not_known"));
            for a in &answers {
                assert_eq!(a, &first, "{name} {side:?}");
            }
            // It says nothing of the question.
            assert!(
                !first.contains("99999") && !first.contains("ship:"),
                "{first}"
            );
            // Known things are known.
            let (known, _) = asked(
                &view,
                &Question::AirDistance {
                    from: PlaceArg::unit(&own_base),
                    to: PlaceArg::unit(&own_base),
                },
            );
            assert!(matches!(known, MeasureAnswer::AirDistance(_)));
        }
    }
}

#[test]
fn a_hex_off_the_map_and_a_wrong_kind_of_unit_are_not_known_but_a_land_hex_is() {
    let scenario = load("04b_round_the_island.toml", 1942);
    let view = scenario.world.side_view(Side::Allied);
    // Land is on the map.
    assert_eq!(air(&view, PlaceArg::hex(4, 0), PlaceArg::hex(0, 0)), 160);
    // A ship is not a task force, and a force is not an air group.
    let (a, _) = asked(
        &view,
        &Question::Passage {
            task_force: "ship:1".to_string(),
            to: PlaceArg::hex(8, 0),
            speed: None,
        },
    );
    assert!(matches!(a, MeasureAnswer::NotKnown(_)));
    let (a, _) = asked(
        &view,
        &Question::AirReach {
            air_group: "task_force:1".to_string(),
            to: vec![],
        },
    );
    assert!(matches!(a, MeasureAnswer::NotKnown(_)));
    // The JSON entry: a bad question is an error, a hex off the map is not
    // known.
    let ok = measure_json(
        &view,
        r#"{"question":"air_distance","from":{"q":0,"r":0},"to":{"q":9999,"r":0}}"#,
    )
    .expect("a question");
    assert!(ok.contains("not_known"), "{ok}");
    assert!(measure_json(&view, r#"{"question":"nope"}"#).is_err());
    assert!(measure_json(&view, "[]").is_err());
    assert!(measure_json(
        &view,
        r#"{"question":"air_distance","from":{"q":0,"r":0},"to":{"q":1,"r":0},"extra":1}"#
    )
    .is_err());
}

// ---------------------------------------------------------------------
// By difference, as stage 9b's.
// ---------------------------------------------------------------------

#[test]
fn the_answers_do_not_change_with_what_the_side_cannot_know() {
    // The five differences are made in tests/leaks.rs, which also compares
    // every measure answer in its sample. Here the one difference that is
    // easiest to state: two worlds with the same own units and different
    // enemy fuel give byte-identical answers to a sample of questions.
    let name = "07_full_slice.toml";
    let text = scenario_text(name);
    let edited = text.replace(
        "side = \"japanese\"\nfuel_t = 2000",
        "side = \"japanese\"\nfuel_t = 1999",
    );
    assert_ne!(text, edited, "the edit changes the scenario");
    let mut compared = 0;
    for (a, b) in [(text.clone(), edited)] {
        let (mut one, mut two) = (load_text(&a, name, 1), load_text(&b, name, 1));
        for _ in 0..4 {
            // The Japanese ships' fuel is the Japanese side's own knowledge:
            // only the Allied side may be compared.
            for side in [Side::Allied] {
                let (v1, v2) = (one.world.side_view(side), two.world.side_view(side));
                for (key, q) in sample_questions(&v1, 6, 3) {
                    let x = serde_json::to_string(&measure(&v1, &q)).unwrap();
                    let y = serde_json::to_string(&measure(&v2, &q)).unwrap();
                    assert_eq!(x, y, "{key}");
                    compared += 1;
                }
            }
            one.next_day().expect("resolves");
            two.next_day().expect("resolves");
        }
    }
    println!("{compared} answers compared");
    assert!(compared > 100);
}

#[test]
fn every_answer_has_a_text_and_text_for_a_passage_names_what_it_assumes() {
    let scenario = load("02_supply_run.toml", 1942);
    let mut scenario = scenario;
    advance_to(&mut scenario, 2);
    let view = scenario.world.side_view(Side::Allied);
    for (key, q) in sample_questions(&view, 4, 3) {
        let (answer, data) = asked(&view, &q);
        let text = answer.to_text();
        assert!(text.starts_with("FLAG PLOT MEASURE"), "{key}: {text}");
        assert!(!text.contains("odds") && !text.contains("chance"), "{key}");
        let _ = data;
    }
    let text = measure(&view, &passage_question(1, PlaceArg::hex(8, 0), None)).to_text();
    println!("{text}");
    assert!(text.contains("Assumes:"));
    assert!(text.contains("it meets nobody and fights nothing"));
}

#[test]
fn the_passage_of_the_convoy_and_the_reach_of_the_g4m1s_as_text() {
    let mut scenario = load("02_supply_run.toml", 1942);
    advance_to(&mut scenario, 2);
    let view = scenario.world.side_view(Side::Allied);
    let text = measure(&view, &passage_question(1, PlaceArg::hex(8, 0), None)).to_text();
    println!("---- passage of the convoy in scenario 2\n{text}");
    let scenario = load("06_rabaul_guadalcanal.toml", 1942);
    let view = scenario.world.side_view(Side::Japanese);
    let text = measure(
        &view,
        &Question::AirReach {
            air_group: "air_group:1".to_string(),
            to: vec![PlaceArg::hex(HENDERSON.0, HENDERSON.1)],
        },
    )
    .to_text();
    println!("---- air reach of the G4M1 group in scenario 6\n{text}");
}

#[test]
fn an_enemy_base_is_a_place_by_its_public_facts() {
    let scenario = load("06_rabaul_guadalcanal.toml", 1);
    let truth = scenario.world.umpire();
    let enemy = truth
        .bases
        .values()
        .find(|b| b.side == Side::Japanese)
        .expect("a Japanese base")
        .clone();
    let view = scenario.world.side_view(Side::Allied);
    let own = view.bases().next().expect("own base").id.to_string();
    let schemas = schemas();
    let handle = enemy.id.to_string();
    let (_, data) = check_measure(
        &view,
        &Question::AirDistance {
            from: PlaceArg::unit(&own),
            to: PlaceArg::unit(&handle),
        },
        &schemas,
        "air distance to an enemy base",
    );
    assert_eq!(data["kind"], "measure");
    assert_eq!(data["to"]["unit"]["handle"], handle.as_str());
    assert_eq!(data["to"]["unit"]["name"], enemy.name.as_str());
    assert_eq!(data["to"]["place"]["q"], enemy.hex.q);
    // A base number that does not exist is still not known.
    let (_, data) = check_measure(
        &view,
        &Question::AirDistance {
            from: PlaceArg::unit(&own),
            to: PlaceArg::unit("base:99999"),
        },
        &schemas,
        "air distance to no base",
    );
    assert_eq!(data["kind"], "not_known");
    // The reach of an own air group to the enemy base names it.
    let group = view.air_groups().next().expect("a group").id.to_string();
    let (_, data) = check_measure(
        &view,
        &Question::AirReach {
            air_group: group,
            to: vec![PlaceArg::unit(&handle)],
        },
        &schemas,
        "air reach to an enemy base",
    );
    assert_eq!(data["kind"], "measure");
}
