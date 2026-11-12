//! Aircraft in the air when their deck is lost (decision 0059, stage 2).
//!
//! # Hand calculation
//!
//! Written by the architect before the test code. The tests below assert
//! these numbers. If the code disagrees, the code or the rule is wrong: do
//! not change a number here to make a test pass.
//!
//! The scenario is `data/scenarios/12c_no_deck.toml`, on the open map of
//! scenario 5b, with placeholder ships and aircraft and pilots counted for
//! every group. It is built so that nothing in it is left to a roll except
//! the crews of ditched aircraft:
//!
//! - A Japanese carrier force lies in (9,0): carrier B with group 3, 18 dive
//!   bombers, on naval strike, and group 4, 16 scouts, searching; two
//!   cruisers; and a second carrier, B2, whose only group is group 5, 12
//!   fighters, on rest.
//! - An Allied carrier force lies in (4,0), 200 nm west: carrier A with
//!   group 1, 16 scouts, searching, and group 2, 18 dive bombers, on naval
//!   strike.
//! - An Allied force of transports with no guns lies in (9,5), 200 nm from
//!   (9,0) and outside every Allied need: it is what the Japanese scouts
//!   find. The Japanese arc is laid so that it covers (9,5) and not (4,0).
//! - The figures are altered for the test so that every sighting succeeds
//!   and is reported in its true hex, every bomb hits, every hit lands on
//!   flotation, and no ship or base has an anti-aircraft gun: sixteen scouts
//!   give a chance to sight of 57.295779 x 16 x 20 / (200 x 90) = 1.018591,
//!   so 1.
//!
//! So in the morning of day 1 both sides sight and both strike. Group 2's
//! 18 bombs hit carrier B, 10 points each on a toughness of 600: 180 points
//! of flotation, and she sinks in the pulse. Group 3's 18 aircraft are over
//! the transports when she goes down. Group 4's 16 scouts are in the air
//! too. 200 nm is within the dive bomber's radius of 250, so nothing is
//! rolled for fuel.
//!
//! The scouts' type has no strike radius; a scout looks for a base within
//! its search range.
//!
//! ## Case A: another carrier with room
//!
//! Carrier B2 holds 36 aircraft and has 12 aboard, so she has room for 24.
//! Group 3, the lower number, comes first: all 18 land on her and form a
//! new group, there being no dive bombers aboard her. She then has room for
//! 6: 6 of group 4's 16 scouts land on her and form another new group, and
//! the other 10 find no base within reach and go into the sea beside the
//! cruisers and B2. Carrier B took nothing down but what was aboard: no
//! ready aircraft of groups 3 and 4 (all were flying).
//!
//! | | Landed on B2 | Ditched | Lost with carrier B |
//! | --- | --- | --- | --- |
//! | Group 3, 18 dive bombers | 18 | 0 | 0 |
//! | Group 4, 16 scouts | 6 | 10 | 0 |
//!
//! The 10 ditched crews are recovered with the figure for coming down
//! beside one's own force: at 1 none is lost, at 0 all 10.
//!
//! ## Case B: no carrier, a base within reach
//!
//! B2 is taken out of the scenario and a Japanese base is put in (12,0),
//! 120 nm from (9,0): within the dive bomber's radius of 250 and the
//! scout's search range. All 18 of group 3 and all 16 of group 4 land
//! there, each forming a new group at the base. Nothing ditches.
//!
//! ## Case C: neither
//!
//! With no B2 and no base, all 34 go into the sea. The two cruisers are
//! afloat in the hex, so the crews are recovered with the figure for one's
//! own force: at 1, 34 pilots saved; at 0, 34 lost. With the cruisers taken
//! out as well, carrier B sinks alone and the figure for open sea applies
//! instead: the test sets the two figures apart to show which is read.
//!
//! ## Case D: the deck out, the ship afloat
//!
//! The Allied strike is cut to 2 dive bombers and the figures altered so
//! that every hit also knocks a system and every system hit is on the
//! flight deck. Two hits: 20 points of flotation, and the flight deck goes
//! from working to degraded to out. Carrier B is afloat with her deck out.
//! Her aircraft come home as in case A: 18 of group 3 and 6 of group 4 onto
//! B2, 10 of group 4 into the sea beside their own ships. Groups 3 and 4
//! stay aboard carrier B with no aircraft.
//!
//! With the strike cut to 1 dive bomber the deck is degraded, not out, and
//! every aircraft lands on carrier B as before the rule.
//!
//! # Properties
//!
//! - Aircraft are conserved: for each group, the aircraft in the air when
//!   the deck was lost equal those that landed elsewhere plus those ditched
//!   (plus those out of fuel, in a scenario where any are).
//! - No carrier ends a pulse over her aircraft capacity.
//! - A group at a base, and a group whose ship is afloat with her deck
//!   working or degraded, is never moved or ditched by this rule.
//! - A ditched aircraft counts for the enemy's objective and one that lands
//!   elsewhere does not.
//! - Pilots lost are never more than aircraft ditched.
//!
//! # Pinned outcomes
//!
//! These are what the scenario's own seed (1942) gives at the working
//! figures. They are pinned, not calculated: no hand calculation can say
//! which draw succeeds. A change here means a stream or the order of draws
//! moved, and they are remade only with a deliberate change of the rule
//! (docs/step2-spec.md, "Tests while figures move", 7).
//!
//! Case A at the live figure for coming down beside one's own force (0.95),
//! seed 1942: of the 10 ditched crews of group 4, 9 are
//! recovered and 1 pilot is lost (the expectation is 9.5).
//!
//! # What the scenario says that the calculation does not
//!
//! The map is a wider copy of the open map of scenario 5b (its rows run from
//! r -3 to 3, which has no (9,5)); the Japanese arc is 110 to 200, 90
//! degrees as the chance above assumes, which covers the bearing 150 to
//! (9,5). Both strikes are repeated in the afternoon pulse (a group that did
//! not lose its aircraft launches again at the contact it holds), so the
//! tests read the morning.

use std::fs;
use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, ShipId};
use sim_core::objectives::tallies;
use sim_core::side_events::SideEventKind;
use sim_core::state::{AirGroupLocation, Side};
use sim_core::time::Pulse;
use sim_core::World;
use sim_data::Scenario;

mod common;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const CARRIER_B: ShipId = ShipId::new(3);
const CARRIER_B2: ShipId = ShipId::new(6);
const BOMBERS: AirGroupId = AirGroupId::new(3);
const SCOUTS: AirGroupId = AirGroupId::new(4);

/// What a variant changes: the case's cuts of the scenario's text and the
/// figures that differ from the frozen ones.
#[derive(Clone, Copy)]
struct Case<'a> {
    /// Text of the scenario cut out (each starts at a `[[` line).
    cut: &'a [&'a str],
    /// Text added at the end of the scenario.
    add: &'a str,
    /// Allied strike aircraft (18 in the file).
    allied_bombers: u32,
    /// `crew_recovered_beside_own_force` and `crew_recovered_open_sea`.
    beside: &'a str,
    open: &'a str,
    /// Whether every hit also knocks the flight deck.
    deck_hits: bool,
}

const CASE_A: Case = Case {
    cut: &[],
    add: "",
    allied_bombers: 18,
    beside: "1",
    open: "0",
    deck_hits: false,
};

const CUT_B2: [&str; 2] = ["[[ship]]\nid = 6\n", "[[air_group]]\nid = 5\n"];
const BASE_AT_12: &str =
    "\n[[base]]\nid = 1\nname = \"Placeholder airfield\"\nside = \"japanese\"\nq = 12\nr = 0\nsupply_t = 1000\n";

/// Cuts the block that begins with `start` (up to the next `[[` line).
fn cut(text: &str, start: &str) -> String {
    let at = text
        .find(start)
        .unwrap_or_else(|| panic!("{start:?} is in the text"));
    let rest = &text[at + start.len()..];
    let end = rest.find("\n[[").map_or(rest.len(), |i| i + 1);
    format!("{}{}", &text[..at], &rest[end..])
}

fn variant(case: Case, seed: u64) -> Scenario {
    let options = common::altered_reference("no_deck", |dir| {
        // Every sighting reported in its true hex.
        common::edit_file_by(
            dir,
            "search.toml",
            &[(
                "position_error_chance = \"0.3\"",
                "position_error_chance = \"0\"",
            )],
        );
        // Every bomb hits and every hit lands on flotation; a system hit
        // only where the case asks for it.
        let per_point = if case.deck_hits { "1" } else { "0" };
        common::edit_file_by(
            dir,
            "surface_combat.toml",
            &[
                ("gun_hit_flotation = \"0.75\"", "gun_hit_flotation = \"1\""),
                (
                    "system_hit_chance_per_point = \"0.06\"",
                    &format!("system_hit_chance_per_point = \"{per_point}\""),
                ),
            ],
        );
        let mut air = vec![
            (
                "bomb_hit_chance = \"0.2\"".to_string(),
                "bomb_hit_chance = \"1\"".to_string(),
            ),
            (
                "crew_recovered_beside_own_force = \"1\"".to_string(),
                format!("crew_recovered_beside_own_force = \"{}\"", case.beside),
            ),
            (
                "crew_recovered_open_sea = \"1\"".to_string(),
                format!("crew_recovered_open_sea = \"{}\"", case.open),
            ),
        ];
        if case.deck_hits {
            for (from, to) in [
                ("flight_deck = \"0.5\"", "flight_deck = \"1\""),
                ("propulsion = \"0.2\"", "propulsion = \"0\""),
                ("fire_control = \"0.1\"", "fire_control = \"0\""),
                ("main_battery = \"0.2\"", "main_battery = \"0\""),
            ] {
                air.push((from.to_string(), to.to_string()));
            }
        }
        let pairs: Vec<(&str, &str)> = air.iter().map(|(a, b)| (a.as_str(), b.as_str())).collect();
        common::edit_file_by(dir, "air_combat.toml", &pairs);
    });
    let path = PathBuf::from(ROOT).join("data/scenarios/12c_no_deck.toml");
    let mut text = fs::read_to_string(&path).expect("the scenario reads");
    for start in case.cut {
        text = cut(&text, start);
    }
    if case.allied_bombers != 18 {
        text = common::replace_once(
            &text,
            "ready = 18\npilots = 18\n\n[[air_group]]\nid = 3\n",
            &format!(
                "ready = {n}\npilots = {n}\n\n[[air_group]]\nid = 3\n",
                n = case.allied_bombers
            ),
        );
    }
    text.push_str(case.add);
    common::from_text_with(&text, &path, &options)
        .expect("the variant loads")
        .with_seed(seed)
        .expect("a seed can be set")
}

fn run(case: Case, seed: u64) -> World {
    let mut scenario = variant(case, seed);
    scenario.run_to_end().expect("the run resolves");
    scenario.world
}

fn events_of(world: &World, keep: impl Fn(&EventKind) -> bool) -> Vec<Event> {
    world
        .events()
        .iter()
        .filter(|event| keep(&event.kind))
        .cloned()
        .collect()
}

fn morning(events: Vec<Event>) -> Vec<Event> {
    events
        .into_iter()
        .filter(|e| e.pulse == Some(Pulse::Morning))
        .collect()
}

/// (from, to, at, aircraft, pilots, formed) of each landing in the morning.
fn landings(world: &World) -> Vec<(u32, u32, AirGroupLocation, u32, u32, bool)> {
    morning(events_of(world, |k| {
        matches!(k, EventKind::AircraftLandedElsewhere { .. })
    }))
    .iter()
    .map(|e| match e.kind {
        EventKind::AircraftLandedElsewhere {
            from,
            to,
            at,
            aircraft,
            pilots,
            formed,
        } => (from.raw(), to.raw(), at, aircraft, pilots, formed),
        _ => unreachable!(),
    })
    .collect()
}

/// (group, aircraft ditched, beside own force, chance, crews recovered,
/// pilots lost) of each ditching for want of a deck in the morning.
#[allow(clippy::type_complexity)]
fn no_deck_ditchings(world: &World) -> Vec<(u32, u32, bool, Fixed, Option<u32>, u32)> {
    morning(events_of(world, |k| {
        matches!(k, EventKind::AircraftDitchedNoDeck { .. })
    }))
    .iter()
    .map(|e| match e.kind {
        EventKind::AircraftDitchedNoDeck {
            air_group,
            aircraft,
            beside_own_force,
            chance,
            crews_recovered,
            pilots_lost,
            ..
        } => (
            air_group.raw(),
            aircraft,
            beside_own_force,
            chance,
            crews_recovered,
            pilots_lost,
        ),
        _ => unreachable!(),
    })
    .collect()
}

/// The aircraft each group lost with its ship, in the morning.
fn lost_with_ship(world: &World) -> Vec<(u32, u32)> {
    morning(events_of(world, |k| {
        matches!(k, EventKind::AirGroupLost { .. })
    }))
    .iter()
    .map(|e| match e.kind {
        EventKind::AirGroupLost {
            air_group,
            aircraft,
            ..
        } => (air_group.raw(), aircraft),
        _ => unreachable!(),
    })
    .collect()
}

fn ship_sunk(world: &World, ship: ShipId) -> bool {
    morning(events_of(
        world,
        |k| matches!(k, EventKind::ShipSunk { ship: s, .. } if *s == ship),
    ))
    .len()
        == 1
}

fn at_ship(ship: u32) -> AirGroupLocation {
    AirGroupLocation::Ship(ShipId::new(ship))
}

fn at_base(base: u32) -> AirGroupLocation {
    AirGroupLocation::Base(sim_core::ids::BaseId::new(base))
}

// ---------------------------------------------------------------------
// The setting.
// ---------------------------------------------------------------------

#[test]
fn the_figures_of_the_setting_are_what_the_calculation_uses() {
    let scenario = variant(CASE_A, 1942);
    let data = scenario.world.ref_data();
    let class = &data.ship_classes[&sim_core::ids::ShipClassId::new(4)];
    assert_eq!(class.toughness, Fixed::from_int(600));
    assert_eq!(class.aircraft_capacity, 36);
    let dive = &data.aircraft_types[&sim_core::ids::AircraftTypeId::new(3)];
    assert_eq!(dive.attack_damage, Fixed::from_int(60));
    assert_eq!(dive.strike_range_nm, Fixed::from_int(250));
    let scout = &data.aircraft_types[&sim_core::ids::AircraftTypeId::new(2)];
    assert_eq!(scout.search_range_nm, Fixed::from_int(300));
    assert_eq!(scout.strike_range_nm, Fixed::ZERO);
    // Bombs of 60 on a toughness of 600 do 10 points: 18 do 180.
    assert_eq!(
        sim_core::fixed::Fixed::from_int(100).mul_div(dive.attack_damage, class.toughness),
        Fixed::from_int(10)
    );
    assert_eq!(Hex::new(4, 0).air_distance_nm(Hex::new(9, 0)), 200);
    assert_eq!(Hex::new(9, 0).air_distance_nm(Hex::new(14, 0)), 200);
    assert_eq!(Hex::new(9, 0).air_distance_nm(Hex::new(12, 0)), 120);
}

#[test]
fn the_morning_is_both_sides_sighting_and_striking_and_carrier_b_sinking() {
    let world = run(CASE_A, 1942);
    let chances: Vec<Fixed> = morning(events_of(&world, |k| {
        matches!(k, EventKind::SearchAttempt { .. })
    }))
    .iter()
    .map(|e| match e.kind {
        EventKind::SearchAttempt {
            chance, sighted, ..
        } => {
            assert!(sighted);
            chance
        }
        _ => unreachable!(),
    })
    .collect();
    // 57.295779 x 16 x 20 / (200 x 90) = 1.018591: a chance of 1.
    assert_eq!(chances, [Fixed::ONE, Fixed::ONE]);
    assert!(ship_sunk(&world, CARRIER_B));
    assert!(!ship_sunk(&world, CARRIER_B2));
    // Group 2's 18 bombs hit: 10 points each.
    let attacks = morning(events_of(
        &world,
        |k| matches!(k, EventKind::AirAttack { air_group, .. } if air_group.raw() == 2),
    ));
    assert_eq!(attacks.len(), 1);
    assert!(matches!(
        attacks[0].kind,
        EventKind::AirAttack { shots, hits: 18, .. } if shots == Fixed::from_int(18)
    ));
    // Nothing is rolled for fuel: 200 nm is within the radius.
    assert!(events_of(&world, |k| matches!(k, EventKind::AircraftDitched { .. })).is_empty());
}

// ---------------------------------------------------------------------
// Case A.
// ---------------------------------------------------------------------

#[test]
fn case_a_the_bombers_land_on_b2_the_scouts_split_and_ten_go_into_the_sea() {
    let world = run(CASE_A, 1942);
    // Carrier B took down no ready aircraft of groups 3 and 4: all were
    // flying.
    assert_eq!(lost_with_ship(&world), [(3, 0), (4, 0)]);
    assert_eq!(
        landings(&world),
        [
            (3, 6, at_ship(6), 18, 18, true),
            (4, 7, at_ship(6), 6, 6, true),
        ]
    );
    let ditched = no_deck_ditchings(&world);
    assert_eq!(ditched.len(), 1);
    let (group, aircraft, beside, chance, recovered, lost) = ditched[0];
    assert_eq!((group, aircraft, beside), (4, 10, true));
    assert_eq!(chance, Fixed::ONE);
    // At 1 none of the 10 crews is lost.
    assert_eq!((recovered, lost), (Some(10), 0));
    // The groups of the ship that sank end with no aircraft, and the new
    // groups are numbered one above the highest, in order.
    let groups = &world.umpire().air_groups;
    assert_eq!(groups[&BOMBERS].ready, 0);
    assert_eq!(groups[&SCOUTS].ready, 0);
    assert_eq!(
        groups[&AirGroupId::new(6)].aircraft_type,
        groups[&BOMBERS].aircraft_type
    );
    assert_eq!(
        groups[&AirGroupId::new(7)].aircraft_type,
        groups[&SCOUTS].aircraft_type
    );
    assert_eq!(
        groups[&AirGroupId::new(6)].mission,
        sim_core::orders::AirMission::Rest
    );
    // A formed group: named from the old one and its place, on the side of
    // the old one, with the aircraft that landed as its strength, reach
    // normal and no search arc.
    let formed = &groups[&AirGroupId::new(6)];
    assert_eq!(
        formed.name,
        "Placeholder Japanese dive bombers landed at ship:6"
    );
    assert_eq!(formed.side, Side::Japanese);
    assert_eq!(formed.location, at_ship(6));
    assert_eq!(formed.strength, 18);
    assert_eq!(formed.reach, sim_core::orders::Reach::Normal);
    assert!(formed.search.is_none());
    assert_eq!(groups[&AirGroupId::new(7)].strength, 6);
}

#[test]
fn case_a_at_a_crew_figure_of_0_all_ten_crews_are_lost() {
    let world = run(
        Case {
            beside: "0",
            ..CASE_A
        },
        1942,
    );
    let ditched = no_deck_ditchings(&world);
    assert_eq!(ditched[0].4, Some(0));
    assert_eq!(ditched[0].5, 10);
    let lost = morning(events_of(
        &world,
        |k| matches!(k, EventKind::PilotsLost { air_group, .. } if air_group.raw() == 4),
    ));
    assert_eq!(lost.len(), 1);
    assert!(matches!(
        lost[0].kind,
        EventKind::PilotsLost { lost: 10, .. }
    ));
}

// ---------------------------------------------------------------------
// Case B: a base within reach, no carrier.
// ---------------------------------------------------------------------

const CASE_B: Case = Case {
    cut: &CUT_B2,
    add: BASE_AT_12,
    ..CASE_A
};

#[test]
fn case_b_all_34_land_at_the_base_each_forming_a_group_and_nothing_ditches() {
    let world = run(CASE_B, 1942);
    assert!(ship_sunk(&world, CARRIER_B));
    assert_eq!(
        landings(&world),
        [
            (3, 5, at_base(1), 18, 18, true),
            (4, 6, at_base(1), 16, 16, true),
        ]
    );
    assert!(no_deck_ditchings(&world).is_empty());
    assert_eq!(lost_with_ship(&world), [(3, 0), (4, 0)]);
}

// ---------------------------------------------------------------------
// Case C: neither.
// ---------------------------------------------------------------------

const CASE_C: Case = Case {
    cut: &CUT_B2,
    add: "",
    ..CASE_A
};

#[test]
fn case_c_all_34_go_into_the_sea_beside_the_cruisers_at_the_figure_for_ones_own_force() {
    for (beside, saved) in [("1", true), ("0", false)] {
        let world = run(
            Case {
                beside,
                open: if saved { "0" } else { "1" },
                ..CASE_C
            },
            1942,
        );
        assert!(landings(&world).is_empty());
        let ditched = no_deck_ditchings(&world);
        assert_eq!(ditched.len(), 2);
        for (row, aircraft) in ditched.iter().zip([18, 16]) {
            assert_eq!((row.1, row.2), (aircraft, true));
            // At 1, every pilot saved; at 0, every pilot lost, though the
            // open-sea figure is the other way round.
            assert_eq!(
                (row.4, row.5),
                if saved {
                    (Some(aircraft), 0)
                } else {
                    (Some(0), aircraft)
                }
            );
        }
    }
}

#[test]
fn case_c_with_the_cruisers_out_as_well_the_open_sea_figure_is_read() {
    let case = Case {
        cut: &[
            "[[ship]]\nid = 6\n",
            "[[air_group]]\nid = 5\n",
            "[[ship]]\nid = 4\n",
            "[[ship]]\nid = 5\n",
        ],
        ..CASE_A
    };
    for (beside, open, saved) in [("0", "1", true), ("1", "0", false)] {
        let world = run(
            Case {
                beside,
                open,
                ..case
            },
            1942,
        );
        let ditched = no_deck_ditchings(&world);
        assert_eq!(ditched.len(), 2);
        for (row, aircraft) in ditched.iter().zip([18, 16]) {
            // Carrier B sank alone: no ship of its side is in the hex.
            assert_eq!((row.1, row.2), (aircraft, false));
            assert_eq!(
                (row.4, row.5),
                if saved {
                    (Some(aircraft), 0)
                } else {
                    (Some(0), aircraft)
                }
            );
        }
    }
}

// ---------------------------------------------------------------------
// Case D: the deck out, the ship afloat.
// ---------------------------------------------------------------------

const CASE_D: Case = Case {
    allied_bombers: 2,
    deck_hits: true,
    ..CASE_A
};

#[test]
fn case_d_with_two_bombers_the_deck_is_out_and_the_aircraft_come_home_as_in_case_a() {
    let world = run(CASE_D, 1942);
    assert!(!ship_sunk(&world, CARRIER_B));
    // 20 points of flotation, the deck working -> degraded -> out.
    let hits = morning(events_of(
        &world,
        |k| matches!(k, EventKind::SystemHit { ship, .. } if *ship == CARRIER_B),
    ));
    assert_eq!(hits.len(), 2);
    assert!(matches!(
        hits[1].kind,
        EventKind::SystemHit {
            before: sim_core::state::SystemState::Degraded,
            after: sim_core::state::SystemState::Out,
            ..
        }
    ));
    assert!(lost_with_ship(&world).is_empty());
    assert_eq!(
        landings(&world),
        [
            (3, 6, at_ship(6), 18, 18, true),
            (4, 7, at_ship(6), 6, 6, true),
        ]
    );
    let ditched = no_deck_ditchings(&world);
    assert_eq!(ditched.len(), 1);
    assert_eq!((ditched[0].0, ditched[0].1, ditched[0].2), (4, 10, true));
    // The landings cite the system hit that put the deck out.
    let landed = morning(events_of(&world, |k| {
        matches!(k, EventKind::AircraftLandedElsewhere { .. })
    }));
    assert!(landed[0].causes.contains(&Cause::Event(hits[1].id)));
    // Groups 3 and 4 stay aboard carrier B with no aircraft.
    let groups = &world.umpire().air_groups;
    for id in [BOMBERS, SCOUTS] {
        assert_eq!(groups[&id].location, at_ship(3));
        assert_eq!(groups[&id].ready, 0);
    }
}

#[test]
fn case_d_with_one_bomber_the_deck_is_degraded_and_every_aircraft_lands_on_b() {
    let world = run(
        Case {
            allied_bombers: 1,
            ..CASE_D
        },
        1942,
    );
    assert!(landings(&world).is_empty());
    assert!(no_deck_ditchings(&world).is_empty());
    assert!(lost_with_ship(&world).is_empty());
    // One hit: the deck is degraded at the end of the morning (the
    // afternoon's hit, which the tests do not read, takes it further).
    let hits = morning(events_of(
        &world,
        |k| matches!(k, EventKind::SystemHit { ship, .. } if *ship == CARRIER_B),
    ));
    assert_eq!(hits.len(), 1);
    assert!(matches!(
        hits[0].kind,
        EventKind::SystemHit {
            after: sim_core::state::SystemState::Degraded,
            ..
        }
    ));
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

/// The aircraft in the air of each source group in the morning: what its
/// strike launched, or its scouts searched with.
fn in_the_air(world: &World, group: AirGroupId) -> u32 {
    morning(events_of(world, |_| true))
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::StrikeLaunched {
                air_group,
                aircraft,
                ..
            } if air_group == group => Some(aircraft),
            EventKind::SearchFlown {
                air_group,
                aircraft,
                ..
            } if air_group == group => Some(aircraft),
            _ => None,
        })
        .sum()
}

#[test]
fn aircraft_are_conserved_and_pilots_lost_are_never_more_than_aircraft_ditched() {
    for case in [CASE_A, CASE_B, CASE_C, CASE_D] {
        for seed in 1..=40 {
            let world = run(
                Case {
                    beside: "0.5",
                    ..case
                },
                seed,
            );
            for group in [3, 4] {
                let id = AirGroupId::new(group);
                let landed: u32 = landings(&world)
                    .iter()
                    .filter(|l| l.0 == group)
                    .map(|l| l.3)
                    .sum();
                let ditched: u32 = no_deck_ditchings(&world)
                    .iter()
                    .filter(|d| d.0 == group)
                    .map(|d| d.1)
                    .sum();
                let lost_aboard: u32 = lost_with_ship(&world)
                    .iter()
                    .filter(|l| l.0 == group)
                    .map(|l| l.1)
                    .sum();
                assert_eq!(
                    in_the_air(&world, id),
                    landed + ditched,
                    "group {group} seed {seed}"
                );
                assert_eq!(lost_aboard, 0);
            }
            for row in no_deck_ditchings(&world) {
                assert!(row.5 <= row.1);
            }
        }
    }
}

#[test]
fn a_group_whose_ship_is_afloat_with_her_deck_working_or_degraded_is_never_moved() {
    // Carrier A's groups (1 and 2) and the fighters on B2 are never touched
    // in any case, and in case D with one bomber nor are carrier B's.
    for case in [CASE_A, CASE_B, CASE_C, CASE_D] {
        let world = run(case, 1942);
        for row in landings(&world) {
            assert!(row.0 == 3 || row.0 == 4);
        }
        for row in no_deck_ditchings(&world) {
            assert!(row.0 == 3 || row.0 == 4);
        }
    }
}

#[test]
fn a_ditched_aircraft_counts_for_the_enemy_and_one_that_lands_elsewhere_does_not() {
    let world = run(CASE_A, 1942);
    let tally = tallies(world.umpire(), world.events(), world.ref_data());
    // The Allied side is credited with the 10 ditched scouts and not with
    // the 24 that landed (and with what its own afternoon strike shoots down
    // or sinks aboard B2: count only the morning's ditching).
    let ditched: u32 = no_deck_ditchings(&world).iter().map(|d| d.1).sum();
    assert_eq!(ditched, 10);
    let all: u32 = events_of(&world, |k| {
        matches!(
            k,
            EventKind::AircraftLost { .. }
                | EventKind::AirGroupLost { .. }
                | EventKind::AircraftDestroyedOnGround { .. }
                | EventKind::DamagedAircraftDestroyedOnGround { .. }
                | EventKind::AircraftWorn { .. }
                | EventKind::AircraftDitched { .. }
                | EventKind::AircraftDitchedNoDeck { .. }
        )
    })
    .iter()
    .map(|e| match e.kind {
        EventKind::AircraftLost {
            air_group,
            aircraft,
            ..
        }
        | EventKind::AirGroupLost {
            air_group,
            aircraft,
            ..
        }
        | EventKind::AircraftDestroyedOnGround {
            air_group,
            aircraft,
            ..
        }
        | EventKind::DamagedAircraftDestroyedOnGround {
            air_group,
            aircraft,
            ..
        }
        | EventKind::AircraftDitchedNoDeck {
            air_group,
            aircraft,
            ..
        } if world.umpire().air_groups[&air_group].side == Side::Japanese => aircraft,
        _ => 0,
    })
    .sum();
    assert_eq!(tally[&Side::Allied].aircraft, all);
    assert!(all >= 10);
    // A landing changes no tally: the log with the landings removed counts
    // the same (they are not among the kinds counted).
    assert!(landings(&world).len() == 2);
}

#[test]
fn a_side_sees_its_own_landings_and_ditchings_whole_and_the_enemy_none() {
    let world = run(CASE_A, 1942);
    let count = |side: Side| {
        world
            .side_view(side)
            .events()
            .iter()
            .filter(|e| {
                matches!(
                    &e.kind,
                    SideEventKind::Whole(
                        EventKind::AircraftLandedElsewhere { .. }
                            | EventKind::AircraftDitchedNoDeck { .. }
                    )
                )
            })
            .count()
    };
    assert!(count(Side::Japanese) >= 3);
    assert_eq!(count(Side::Allied), 0);
}

// ---------------------------------------------------------------------
// Pinned outcomes.
// ---------------------------------------------------------------------

#[test]
fn pinned_the_crews_of_the_ten_ditched_scouts_of_seed_1942_at_the_live_figure() {
    let world = run(
        Case {
            beside: "0.95",
            open: "0.5",
            ..CASE_A
        },
        1942,
    );
    let ditched = no_deck_ditchings(&world);
    assert_eq!(ditched.len(), 1);
    assert_eq!(ditched[0].1, 10);
    assert_eq!(ditched[0].3, Fixed::from_ratio(95, 100));
    assert_eq!(ditched[0].4, Some(9), "crews recovered of the 10");
    assert_eq!(ditched[0].5, 1, "pilots lost");
}
