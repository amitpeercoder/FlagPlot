//! Reach beyond the radius and running out of fuel (decision 0059, stage 1).
//!
//! # Hand calculation
//!
//! Written by the architect before the test code. The tests below assert
//! these numbers. If the code disagrees, the code or the rule is wrong: do
//! not change a number here to make a test pass.
//!
//! The scenario is `data/scenarios/12a_long_reach.toml`: scenario 5b, the
//! unopposed strike, with the enemy carrier force moved from (9,0) to
//! (11,0) and the bombing squadron given 18 pilots. Everything else is as in
//! 5b: the Allied carrier force lies in (4,0); group 1, 8 scouts, searches
//! the arc 040 to 130 out to 300 nm; group 2, 18 dive bombers, is on naval
//! strike. The dive bomber's strike radius is 250 nm.
//!
//! ## Distances and reach
//!
//! (4,0) to (11,0) is 7 hexes along an axis: 280 nm by air. That is 30 nm
//! beyond the radius. Extended reach is 250 x (1 + 0.25) = 312.5, cut to
//! 312 nm, so the target is within extended reach and outside normal reach.
//!
//! ## The search
//!
//! The scouts' chance to sight in a daylight pulse is 57.295779 x 8 x 20 /
//! (280 x 90): 9167.32464 / 25200 = 0.363782 (cut). It is 280 nm inside a
//! range of 300.
//!
//! ## With the bombing squadron on normal reach
//!
//! No strike is launched in any pulse, whatever is sighted. Nothing ditches.
//! The squadron ends with 18 aircraft and 18 pilots.
//!
//! ## With the bombing squadron on extended reach
//!
//! In the pulse the force is sighted and reported in its true hex, the
//! squadron launches all 18 at 280 nm. No fighter or gun fires (the enemy
//! has none), so 18 attack: 18 bombs at 0.2, 3.6 hits expected, as in 5b.
//!
//! All 18 come back from the target. The chance that one runs out of fuel
//! is 2 x (280 - 250) / 250: 30 / 250 = 0.12, times 2 is 0.24. Expected
//! ditched: 18 x 0.24 = 4.32. Each ditched aircraft's crew is recovered
//! with chance 0.7.
//!
//! The rolls cannot be fixed by hand, so three settings of the figures are:
//!
//! | `ditch_chance_per_radius_beyond` | `crew_recovered_short_of_home` | Chance | Ditched of 18 | Pilots lost | Squadron after |
//! | --- | --- | --- | --- | --- | --- |
//! | 0  | any | 0 | 0  | 0  | 18 aircraft, 18 pilots, less what wear writes off |
//! | 10 | 1   | 10 x 0.12 = 1.2, so 1 | 18 | 0  | 0 aircraft, 18 pilots |
//! | 10 | 0   | 1 | 18 | 18 | 0 aircraft, 0 pilots |
//!
//! With every aircraft ditched, none is rolled for wear: the wear event of
//! the strike sortie, if wear is in play, rolls 0 aircraft. The fight
//! itself (the sighting, the launch, the 18 bombs and their hits) is the
//! same at every setting.
//!
//! # Properties
//!
//! - On normal reach the log holds no launch and no ditching, at any
//!   setting.
//! - The aircraft ditched are never more than those that came back, and the
//!   pilots lost never more than the aircraft ditched.
//! - A group that flew within its radius never ditches: every scenario that
//!   existed before this rule gives the events it gave before, and only its
//!   hash lines change.
//! - Over seeds 1 to 1000 at the placeholder figures, on the runs where the
//!   strike is launched, the aircraft ditched sum to within 10 in 100 of
//!   0.24 times the aircraft that came back, and the crews recovered to
//!   within 10 in 100 of 0.7 times the aircraft ditched.
//! - The chance does not fall as the distance rises: at 290 and 300 nm
//!   (targets one and two hexes off the axis are not needed; use the
//!   function) it is 0.32 and 0.4.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use sim_core::events::{Cause, Event, EventKind, SortieKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, AircraftTypeId};
use sim_core::objectives::tallies;
use sim_core::orders::{AirGroupOrder, OrderKind, Reach};
use sim_core::side_events::SideEventKind;
use sim_core::state::Side;
use sim_core::{SimConfig, World};
use sim_data::Scenario;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const SCENARIO: &str = "12a_long_reach.toml";
const SEEDS: u64 = 1000;
const BOMBERS: AirGroupId = AirGroupId::new(2);
const HOME: Hex = Hex::new(4, 0);
const TARGET: Hex = Hex::new(11, 0);
const BOMBER_TYPE: AircraftTypeId = AircraftTypeId::new(3);

// ---------------------------------------------------------------------
// The one way these tests run the scenario with altered figures. The
// figures of decision 0059 are written into a copy of the air combat file
// under the build directory, the scenario is read from a copy of its text
// that names it, and nothing in the repository's data folder is touched.
// Everything below goes through `variant`, so changing how figures are
// altered means changing this function alone.
// ---------------------------------------------------------------------

/// The two figures the table of the hand calculation sets. The placeholder
/// values are the file's own.
#[derive(Clone, Copy)]
struct Figures {
    ditch_chance_per_radius_beyond: &'static str,
    crew_recovered_short_of_home: &'static str,
}

const PLACEHOLDER: Figures = Figures {
    ditch_chance_per_radius_beyond: "2",
    crew_recovered_short_of_home: "0.7",
};
/// 0 for the chance: nothing ditches whatever the crews' figure.
const NO_DITCHING: Figures = Figures {
    ditch_chance_per_radius_beyond: "0",
    crew_recovered_short_of_home: "0.7",
};
/// 10 x 0.12 is 1.2, so 1: every aircraft ditches and every crew is saved.
const ALL_DITCH_ALL_SAVED: Figures = Figures {
    ditch_chance_per_radius_beyond: "10",
    crew_recovered_short_of_home: "1",
};
/// Every aircraft ditches and every crew is lost.
const ALL_DITCH_ALL_LOST: Figures = Figures {
    ditch_chance_per_radius_beyond: "10",
    crew_recovered_short_of_home: "0",
};

/// What the variant adds to the scenario file's list of figure files.
const WEAR_AND_FATIGUE: &str =
    "air_upkeep = \"../reference/air_upkeep.toml\"\nfatigue = \"../reference/fatigue.toml\"\n";

fn variant(reach: &str, figures: Figures, extra_keys: &str) -> Scenario {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let data = PathBuf::from(ROOT).join("data");
    // Tests run in parallel: each variant has a folder of its own.
    let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "reach_{}_{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&folder).expect("make the variant folder");

    let text = fs::read_to_string(data.join("reference/air_combat.toml"))
        .expect("the air combat figures read");
    let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
    for (key, value) in [
        (
            "ditch_chance_per_radius_beyond",
            figures.ditch_chance_per_radius_beyond,
        ),
        (
            "crew_recovered_short_of_home",
            figures.crew_recovered_short_of_home,
        ),
    ] {
        let mut found = 0;
        for line in &mut lines {
            if line.starts_with(&format!("{key} = ")) {
                *line = format!("{key} = \"{value}\"");
                found += 1;
            }
        }
        assert_eq!(found, 1, "{key} is set once in the file");
    }
    let figures_path = folder.join("air_combat.toml");
    fs::write(&figures_path, lines.join("\n")).expect("write the figures");

    let scenario_path = data.join("scenarios").join(SCENARIO);
    let mut text = fs::read_to_string(&scenario_path).expect("the scenario reads");
    let original = "air_combat = \"../reference/air_combat.toml\"";
    assert_eq!(
        text.matches(original).count(),
        1,
        "the scenario names the figures"
    );
    text = text.replace(
        original,
        &format!(
            "air_combat = {:?}\n{extra_keys}",
            figures_path.to_str().expect("a path")
        ),
    );
    let order = "reach = \"extended\"";
    assert_eq!(
        text.matches(order).count(),
        1,
        "the scenario sets the reach"
    );
    text = text.replace(order, &format!("reach = \"{reach}\""));
    Scenario::from_text(&text, &scenario_path).expect("the variant loads")
}

fn run_seed(base: &Scenario, seed: u64) -> World {
    let mut scenario = base.with_seed(seed).expect("a seed can be set");
    scenario.run_to_end().expect("the run resolves");
    scenario.world
}

fn runs(reach: &str, figures: Figures, extra_keys: &str) -> Vec<World> {
    let base = variant(reach, figures, extra_keys);
    (1..=SEEDS).map(|seed| run_seed(&base, seed)).collect()
}

// ---------------------------------------------------------------------
// What a run says, read from its log.
// ---------------------------------------------------------------------

fn events_of(world: &World, keep: impl Fn(&EventKind) -> bool) -> Vec<Event> {
    world
        .events()
        .iter()
        .filter(|event| keep(&event.kind))
        .cloned()
        .collect()
}

fn launches(world: &World) -> Vec<Event> {
    events_of(world, |k| matches!(k, EventKind::StrikeLaunched { .. }))
}

fn ditchings(world: &World) -> Vec<Event> {
    events_of(world, |k| matches!(k, EventKind::AircraftDitched { .. }))
}

fn pilot_losses(world: &World) -> Vec<Event> {
    events_of(world, |k| matches!(k, EventKind::PilotsLost { .. }))
}

/// One `AircraftDitched` event, read.
#[derive(Clone, Debug)]
struct Ditching {
    id: u32,
    sortie: SortieKind,
    distance: i64,
    radius: i64,
    aircraft: u32,
    chance: Fixed,
    expected: Fixed,
    ditched: u32,
    crews_recovered: Option<u32>,
    pilots_lost: u32,
    ready: u32,
    causes: Vec<Cause>,
}

fn read(event: &Event) -> Ditching {
    match &event.kind {
        EventKind::AircraftDitched {
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
            ..
        } => Ditching {
            id: event.id.raw(),
            sortie: *sortie,
            distance: distance_nm.trunc(),
            radius: radius_nm.trunc(),
            aircraft: *aircraft,
            chance: *chance,
            expected: *expected,
            ditched: *ditched,
            crews_recovered: *crews_recovered,
            pilots_lost: *pilots_lost,
            ready: *ready,
            causes: event.causes.clone(),
        },
        other => panic!("not a ditching: {other:?}"),
    }
}

/// (distance in miles, hex) of each strike launch of the run.
fn launch_hexes(world: &World) -> Vec<(i64, Hex)> {
    launches(world)
        .iter()
        .map(|event| match event.kind {
            EventKind::StrikeLaunched {
                hex, distance_nm, ..
            } => (distance_nm.trunc(), hex),
            _ => unreachable!(),
        })
        .collect()
}

fn bombers_ready(world: &World) -> u32 {
    world.umpire().air_groups[&BOMBERS].ready
}

fn bombers_pilots(world: &World) -> Option<u32> {
    world.umpire().air_groups[&BOMBERS].pilots
}

fn hundredths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 100)
}

/// Whether `k` is within 10 in 100 of `expected` (a relative tolerance of 10
/// in 100 of the expected count, both in hundredths).
fn within_ten_in_a_hundred(k: i64, expected_hundredths: i64) -> bool {
    // |k - expected| <= expected / 10, all in hundredths.
    let off = (k * 100 - expected_hundredths).abs();
    off * 10 <= expected_hundredths
}

// ---------------------------------------------------------------------
// The setting, the reach and the search.
// ---------------------------------------------------------------------

#[test]
fn the_target_is_280_nm_out_30_beyond_the_radius_and_within_extended_reach_only() {
    assert_eq!(HOME.air_distance_nm(TARGET), 280);
    let scenario = variant("extended", PLACEHOLDER, "");
    let ref_data = scenario.world.ref_data();
    let radius = ref_data.aircraft_types[&BOMBER_TYPE].strike_range_nm;
    assert_eq!(radius, Fixed::from_int(250));
    assert_eq!(
        ref_data.reach_nm(BOMBER_TYPE, Reach::Normal),
        Fixed::from_int(250)
    );
    // 250 x 1.25 is 312.5, cut to 312.
    assert_eq!(
        ref_data.reach_nm(BOMBER_TYPE, Reach::Extended),
        Fixed::from_int(312)
    );
    assert!(HOME.air_distance_within(TARGET, ref_data.reach_nm(BOMBER_TYPE, Reach::Extended)));
    assert!(!HOME.air_distance_within(TARGET, ref_data.reach_nm(BOMBER_TYPE, Reach::Normal)));
    // The scenario's own order puts the squadron on extended reach.
    let group = &scenario.world.umpire().air_groups[&BOMBERS];
    assert_eq!(group.reach, Reach::Normal, "until the order is applied");
    assert_eq!(group.pilots, Some(18));
    let world = run_seed(&scenario, 1);
    assert_eq!(world.umpire().air_groups[&BOMBERS].reach, Reach::Extended);
    let order = world.umpire().air_groups[&BOMBERS]
        .reach_order
        .expect("the order that set it");
    assert!(matches!(
        world.order(order).expect("the order is kept").kind,
        OrderKind::AirGroup {
            order: AirGroupOrder::SetReach {
                reach: Reach::Extended
            },
            ..
        }
    ));
}

#[test]
fn the_scouts_chance_to_sight_at_280_nm_is_the_hand_calculation() {
    let world = run_seed(&variant("extended", PLACEHOLDER, ""), 1);
    let attempts = events_of(&world, |k| matches!(k, EventKind::SearchAttempt { .. }));
    assert_eq!(attempts.len(), 2, "a morning and an afternoon search");
    for event in attempts {
        match event.kind {
            EventKind::SearchAttempt {
                distance_nm,
                chance,
                ..
            } => {
                assert_eq!(distance_nm, Fixed::from_int(280));
                // 9167.32464 / 25200, cut to millionths.
                assert_eq!(chance, Fixed::from_raw(363_782));
            }
            _ => unreachable!(),
        }
    }
}

// ---------------------------------------------------------------------
// Normal reach.
// ---------------------------------------------------------------------

#[test]
fn on_normal_reach_nothing_strikes_the_target_and_nothing_ditches_at_any_setting() {
    // The hand calculation says no strike is launched in any pulse, whatever
    // is sighted. MISMATCH, reported to the architect: a sighting report
    // names a neighbouring hex with chance 0.05 each, and (10,0) is 240 nm
    // out, within the radius, so a report that lands there launches a strike
    // on normal reach (it finds nothing: the force is at (11,0)). The test
    // asserts what holds: no launch at the target hex, none beyond the
    // radius, no ditching, and the squadron whole.
    let mut launched_at_all = 0;
    for figures in [PLACEHOLDER, ALL_DITCH_ALL_LOST] {
        for world in runs("normal", figures, "") {
            for (distance, hex) in launch_hexes(&world) {
                assert!(distance <= 250, "a launch at {distance} nm on normal reach");
                assert_ne!(hex, TARGET);
                launched_at_all += 1;
            }
            assert!(ditchings(&world).is_empty());
            assert!(pilot_losses(&world).is_empty());
            assert_eq!(bombers_ready(&world), 18);
            assert_eq!(bombers_pilots(&world), Some(18));
        }
    }
    // The strikes that do launch are the report errors at (10,0).
    assert!(launched_at_all > 0, "{launched_at_all}");
}

// ---------------------------------------------------------------------
// Extended reach: the three settings of the figures.
// ---------------------------------------------------------------------

/// The ditching event of the run's first strike, if that strike was
/// launched at the target hex: all 18 aircraft, 280 nm out, the case of the
/// hand calculation's table. (A report that names a neighbouring hex can
/// make the first strike one at another distance, which leaves fewer
/// aircraft for a later one.)
fn first_at_target(world: &World) -> Option<Ditching> {
    let launch = launches(world).into_iter().next()?;
    if !matches!(launch.kind, EventKind::StrikeLaunched { hex, .. } if hex == TARGET) {
        return None;
    }
    let found: Vec<Ditching> = ditchings(world)
        .iter()
        .map(read)
        .filter(|d| d.causes == vec![Cause::Event(launch.id)])
        .collect();
    assert_eq!(found.len(), 1, "one ditching event for the launch");
    Some(found[0].clone())
}

#[test]
fn with_no_chance_nothing_ditches_and_the_squadron_ends_whole() {
    let mut at_target_runs = 0;
    for world in runs("extended", NO_DITCHING, "") {
        for event in ditchings(&world) {
            let d = read(&event);
            assert_eq!(d.chance, Fixed::ZERO);
            assert_eq!(d.ditched, 0);
            assert_eq!(d.crews_recovered, Some(0));
            assert_eq!(d.pilots_lost, 0);
        }
        assert!(pilot_losses(&world).is_empty());
        assert_eq!(bombers_ready(&world), 18);
        assert_eq!(bombers_pilots(&world), Some(18));
        if let Some(d) = first_at_target(&world) {
            at_target_runs += 1;
            assert_eq!((d.distance, d.radius, d.aircraft), (280, 250, 18));
        }
    }
    assert!(at_target_runs > 100, "{at_target_runs}");
}

#[test]
fn with_every_aircraft_ditched_and_every_crew_saved_the_squadron_is_gone_and_its_pilots_are_not() {
    let mut at_target_runs = 0;
    for world in runs("extended", ALL_DITCH_ALL_SAVED, "") {
        if let Some(d) = first_at_target(&world) {
            at_target_runs += 1;
            // 10 x 0.12 is 1.2, so the chance is 1.
            assert_eq!(d.chance, Fixed::ONE);
            assert_eq!((d.aircraft, d.ditched), (18, 18));
            assert_eq!((d.crews_recovered, d.pilots_lost), (Some(18), 0));
            assert_eq!(d.ready, 0);
            assert_eq!(bombers_ready(&world), 0);
            assert_eq!(bombers_pilots(&world), Some(18));
            assert!(pilot_losses(&world).is_empty());
        }
    }
    assert!(at_target_runs > 100, "{at_target_runs}");
}

#[test]
fn with_every_aircraft_ditched_and_every_crew_lost_the_squadron_and_its_pilots_are_gone() {
    let mut at_target_runs = 0;
    for world in runs("extended", ALL_DITCH_ALL_LOST, "") {
        if let Some(d) = first_at_target(&world) {
            at_target_runs += 1;
            assert_eq!(d.chance, Fixed::ONE);
            assert_eq!((d.aircraft, d.ditched), (18, 18));
            assert_eq!((d.crews_recovered, d.pilots_lost), (Some(0), 18));
            assert_eq!(d.ready, 0);
            assert_eq!(bombers_ready(&world), 0);
            assert_eq!(bombers_pilots(&world), Some(0));
            // Recorded as the pilots rule records it, caused by the ditching.
            let lost = pilot_losses(&world);
            assert_eq!(lost.len(), 1);
            assert_eq!(
                lost[0].causes,
                vec![Cause::Event(sim_core::ids::EventId::new(d.id))]
            );
            assert!(matches!(
                lost[0].kind,
                EventKind::PilotsLost {
                    lost: 18,
                    left: 0,
                    ..
                }
            ));
        }
    }
    assert!(at_target_runs > 100, "{at_target_runs}");
}

#[test]
fn the_fight_is_the_same_at_every_setting_of_the_figures() {
    // The sighting, the launch, the 18 bombs and their hits come from
    // streams the ditching never touches. Compare the text of the events of
    // the fight, run by run, at the four settings.
    let fight = |world: &World| -> Vec<String> {
        world
            .events()
            .iter()
            .filter(|event| {
                !matches!(
                    event.kind,
                    EventKind::AircraftDitched { .. } | EventKind::PilotsLost { .. }
                )
            })
            .map(|event| event.kind.to_string())
            .collect()
    };
    let placeholder = runs("extended", PLACEHOLDER, "");
    for other in [NO_DITCHING, ALL_DITCH_ALL_SAVED, ALL_DITCH_ALL_LOST] {
        let there = runs("extended", other, "");
        for (a, b) in placeholder.iter().zip(&there) {
            // A later strike differs by the aircraft the first one lost, so
            // compare the events up to the second launch.
            let (a, b) = (fight(a), fight(b));
            let cut = |lines: &[String]| -> Vec<String> {
                let mut launches = 0;
                lines
                    .iter()
                    .filter(|line| !line.contains("aged to"))
                    .take_while(|line| {
                        if line.contains("launched") {
                            launches += 1;
                        }
                        launches < 2
                    })
                    .cloned()
                    .collect()
            };
            assert_eq!(cut(&a), cut(&b));
        }
    }
}

// ---------------------------------------------------------------------
// The placeholder figures over seeds 1 to 1000.
// ---------------------------------------------------------------------

#[test]
fn at_the_placeholder_figures_the_ditchings_follow_the_chance_and_the_crews_follow_theirs() {
    let worlds = runs("extended", PLACEHOLDER, "");
    let (mut came_back, mut ditched, mut recovered) = (0_i64, 0_i64, 0_i64);
    for world in &worlds {
        for event in ditchings(world) {
            let d = read(&event);
            if d.distance != 280 {
                continue;
            }
            // The first sortie of the day is 18; a second one, if the
            // squadron strikes again, is whatever it has left.
            assert_eq!(d.sortie, SortieKind::Strike);
            assert_eq!(d.chance, hundredths(24));
            assert_eq!(
                d.expected,
                Fixed::from_int(i64::from(d.aircraft)) * hundredths(24)
            );
            came_back += i64::from(d.aircraft);
            ditched += i64::from(d.ditched);
            recovered += i64::from(d.crews_recovered.expect("pilots are counted"));
        }
    }
    assert!(came_back > 5000, "{came_back}");
    // Within 10 in 100 of 0.24 of the aircraft that came back, and of 0.7 of
    // the aircraft ditched.
    assert!(
        within_ten_in_a_hundred(ditched, came_back * 24),
        "{ditched} ditched of {came_back}"
    );
    assert!(
        within_ten_in_a_hundred(recovered, ditched * 70),
        "{recovered} recovered of {ditched}"
    );
}

#[test]
fn the_chance_does_not_fall_as_the_distance_rises_and_is_0_32_and_0_4_at_290_and_300() {
    let scenario = variant("extended", PLACEHOLDER, "");
    let figures = scenario
        .world
        .ref_data()
        .air_combat
        .as_ref()
        .expect("the figures");
    let radius = Fixed::from_int(250);
    let chance = |nm: i64| figures.ditch_chance(Fixed::from_int(nm), radius);
    assert_eq!(chance(250), Fixed::ZERO);
    assert_eq!(chance(280), hundredths(24));
    assert_eq!(chance(290), hundredths(32));
    assert_eq!(chance(300), hundredths(40));
    let mut last = Fixed::ZERO;
    for nm in 0..=600 {
        assert!(chance(nm) >= last, "{nm} nm");
        last = chance(nm);
    }
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn the_aircraft_ditched_are_never_more_than_came_back_nor_the_pilots_lost_more_than_ditched() {
    for figures in [PLACEHOLDER, ALL_DITCH_ALL_LOST, ALL_DITCH_ALL_SAVED] {
        for world in runs("extended", figures, "") {
            for event in ditchings(&world) {
                let d = read(&event);
                assert!(d.ditched <= d.aircraft);
                assert!(d.pilots_lost <= d.ditched);
                assert!(d.crews_recovered.expect("counted") + d.pilots_lost <= d.aircraft);
                assert_eq!(
                    d.crews_recovered.expect("counted") + d.pilots_lost,
                    d.ditched
                );
                // Only a sortie beyond the radius rolls, and it is the
                // type's radius it is measured against.
                assert!(d.distance > d.radius);
                assert_eq!(d.radius, 250);
                assert_eq!(d.sortie, SortieKind::Strike);
            }
            assert!(bombers_pilots(&world).expect("counted") <= 18);
            assert!(bombers_ready(&world) <= 18);
        }
    }
}

#[test]
fn a_ditching_cites_the_launch_and_the_launch_cites_the_reach_order() {
    for world in runs("extended", PLACEHOLDER, "").into_iter().take(200) {
        let group = &world.umpire().air_groups[&BOMBERS];
        let reach_order = group.reach_order.expect("the order");
        for event in ditchings(&world) {
            assert_eq!(event.causes.len(), 1);
            let Cause::Event(launch) = event.causes[0] else {
                panic!("a ditching is caused by an event");
            };
            let launch = world.events().get(launch).expect("the launch");
            assert!(matches!(launch.kind, EventKind::StrikeLaunched { .. }));
            // On extended reach the launch cites the order that gave it.
            assert!(launch.causes.contains(&Cause::Order(reach_order)));
        }
    }
}

#[test]
fn an_aircraft_that_ditched_is_not_worn_and_the_sortie_still_counts_as_flown() {
    // With the wear and fatigue figures in play: wear rolls the aircraft that
    // came back and did not ditch, and a group that ditched them all is tired
    // all the same.
    for world in runs("extended", PLACEHOLDER, WEAR_AND_FATIGUE)
        .into_iter()
        .take(300)
    {
        for event in ditchings(&world) {
            let d = read(&event);
            let worn: Vec<u32> = events_of(
                &world,
                |k| matches!(k, EventKind::AircraftWorn { air_group, .. } if *air_group == BOMBERS),
            )
            .iter()
            .filter(|e| e.causes == event.causes)
            .map(|e| match e.kind {
                EventKind::AircraftWorn { aircraft, .. } => aircraft,
                _ => unreachable!(),
            })
            .collect();
            match d.aircraft - d.ditched {
                0 => assert!(worn.is_empty(), "wear rolls no aircraft"),
                left => assert_eq!(worn, vec![left]),
            }
        }
    }
    for world in runs("extended", ALL_DITCH_ALL_SAVED, WEAR_AND_FATIGUE)
        .into_iter()
        .take(300)
    {
        if let Some(d) = first_at_target(&world) {
            // No wear event follows the launch of that strike.
            assert!(events_of(&world, |k| matches!(
                k,
                EventKind::AircraftWorn { air_group, .. } if *air_group == BOMBERS
            ))
            .iter()
            .all(|worn| !worn.causes.contains(&d.causes[0])));
            let tired = events_of(
                &world,
                |k| matches!(k, EventKind::AirGroupFatigue { air_group, .. } if *air_group == BOMBERS),
            );
            assert!(!tired.is_empty(), "a group that flew is tired");
        }
    }
}

#[test]
fn the_enemy_counts_the_aircraft_ditched_as_destroyed_though_no_one_claims_them() {
    for world in runs("extended", ALL_DITCH_ALL_LOST, "")
        .into_iter()
        .take(200)
    {
        let ditched: u32 = ditchings(&world).iter().map(|e| read(e).ditched).sum();
        let tally = tallies(world.umpire(), world.events(), world.ref_data());
        // Nothing else of the squadron is lost in this scenario.
        assert_eq!(tally[&Side::Japanese].aircraft, ditched);
        assert_eq!(tally[&Side::Allied].aircraft, 0);
        // And the Japanese claim nothing of it: no claim of the day says
        // aircraft shot down.
        for event in events_of(&world, |k| matches!(k, EventKind::ClaimReceived { .. })) {
            if let EventKind::ClaimReceived {
                claimed_shot_down, ..
            } = event.kind
            {
                assert_eq!(claimed_shot_down, 0);
            }
        }
    }
}

#[test]
fn a_side_sees_its_own_ditchings_whole_and_the_enemy_sees_none() {
    let mut checked = 0;
    for world in runs("extended", PLACEHOLDER, "").into_iter().take(200) {
        let total = ditchings(&world).len();
        let seen = |side: Side| {
            world
                .side_view(side)
                .events()
                .iter()
                .filter(|event| {
                    matches!(
                        &event.kind,
                        SideEventKind::Whole(EventKind::AircraftDitched { .. })
                    )
                })
                .count()
        };
        assert_eq!(seen(Side::Allied), total);
        assert_eq!(seen(Side::Japanese), 0);
        checked += total;
    }
    assert!(checked > 0);
}

#[test]
fn recording_the_breakdowns_changes_neither_the_ditchings_nor_the_hashes() {
    let base = variant("extended", PLACEHOLDER, "");
    for seed in 1..=50 {
        let with = run_seed(&base, seed);
        let mut scenario = base.with_seed(seed).expect("a seed");
        scenario.world.set_config(SimConfig {
            record_breakdowns: false,
        });
        scenario.run_to_end().expect("the run resolves");
        let without = scenario.world;
        assert_eq!(with.day_hashes(), without.day_hashes());
        assert_eq!(ditchings(&with).len(), ditchings(&without).len());
        for event in ditchings(&with) {
            let breakdown = event.breakdown.as_ref().expect("a breakdown");
            let factor = |name: &str| {
                breakdown
                    .factors
                    .iter()
                    .find(|f| f.name == name)
                    .unwrap_or_else(|| panic!("the factor {name}"))
                    .value
            };
            let d = read(&event);
            assert_eq!(factor("distance_nm"), Fixed::from_int(d.distance));
            assert_eq!(factor("radius_nm"), Fixed::from_int(d.radius));
            assert_eq!(factor("chance"), d.chance);
            assert_eq!(breakdown.result, d.expected);
        }
    }
}

#[test]
fn a_replay_of_a_seed_reproduces_the_daily_hash() {
    let base = variant("extended", PLACEHOLDER, "");
    for seed in [1, 2, 3, 77] {
        assert_eq!(
            run_seed(&base, seed).day_hashes(),
            run_seed(&base, seed).day_hashes()
        );
    }
    // The reach is state: a squadron on normal reach has another hash.
    let normal = variant("normal", PLACEHOLDER, "");
    assert_ne!(
        run_seed(&base, 1).day_hashes(),
        run_seed(&normal, 1).day_hashes()
    );
}

#[test]
fn no_scenario_that_gives_no_reach_order_ditches_anything() {
    // The golden files cover every scenario's whole log. Here the 5b strike,
    // whose bombers strike out to their radius, ditches nothing.
    let path = PathBuf::from(ROOT).join("data/scenarios/05b_strike.toml");
    for seed in 1..=100 {
        let mut scenario = Scenario::load(&path)
            .expect("5b loads")
            .with_seed(seed)
            .expect("a seed");
        scenario.run_to_end().expect("the run resolves");
        assert!(ditchings(&scenario.world).is_empty());
    }
}
