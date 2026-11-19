//! Scenario 93: the enemy's half of a picture, by rule. From what a side
//! holds and believes, `sim-trial` makes the enemy it will be tried
//! against: bases, men, a force for each contact, forces believed in and
//! not seen, each where it could be, and what the enemy is taken to want.
//!
//! Proves decision 0174, the third part of step 4 of step 3's stage 4,
//! against a hand calculation. The data is placeholder data: see
//! `data/scenarios/93_the_enemys_half.toml`, which the build writes to the
//! inputs below. It runs on the frozen step 1 figures. Each case is that
//! scenario with the contacts, estimates and beliefs the case names.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly, but for the one count over worlds, which has its allowed
//! range. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Inputs
//!
//! - **The map:** the placeholder map of scenario 1. Every hex is sea
//!   but the two coast hexes of the bases, and a force may be in any.
//! - **The classes,** from `ship_classes.csv`: 1, a cruiser; 2, a
//!   destroyer; 3, a transport; 4, a carrier of 36 aircraft. All cruise
//!   at 16 kn but the transport, at 10 kn. **The types,** from
//!   `aircraft_types.csv`: 3, a bomber; 4, a fighter.
//! - **A file of sides** that gives each of those classes and types to
//!   both sides.
//! - **Bases:** base 1, Allied, at (0,0); base 2, Japanese, at (8,3),
//!   on the Allied side's map.
//! - **Allied:** task force 1, one ship of class 1, at base 1, with no
//!   order. **Japanese:** nothing afloat. Nobody moves in any case.
//! - **The figures:** the merging distance is 2 hexes; a contact is
//!   dropped at 5 pulses; an enemy base is given 1,000 t of supply and
//!   500 t of fuel. A hex is 40 nm. Counted back from an evening, the
//!   pulses are the afternoon, 6 hours; the morning, 6; the night, 12.
//! - **The trial:** seed 1942, trial 1, and the worlds each case names.
//!   Unless a case says otherwise the picture is of the evening before
//!   day 1 (day 0) and the Allied side is the side tried.
//!
//! What the enemy's half is made of is read from the picture; what was
//! assumed is read from the list that comes with it.
//!
//! ## 1. Nothing held, nothing believed
//!
//! The Allied side holds no contact and believes nothing. In every world
//! the enemy's half is base 2 at (8,3) with 1,000 t of supply and 500 t
//! of fuel, and nothing else: no ship, no force, no group, no man. The
//! assumptions name the base and its two figures, and say that the enemy
//! is given no belief and no intent. Worlds 1 and 2 are the same picture.
//!
//! ## 2. A carrier contact of 5 ships
//!
//! One contact: 5 ships taken for a carrier, at (5,2), age 0.
//!
//! - **Ships,** in the order made: 900,001, class 4, the carrier;
//!   900,002 to 900,005, class 2, destroyers. Each undamaged, with the
//!   fuel its class holds, in task force 900,001.
//! - **Task force 900,001:** Japanese, at (5,2), on the carrier mission,
//!   stopped, at cruise, normal readiness, no route, no home port.
//! - **Air groups,** aboard ship 900,001: 900,001, fighters, 36 / 3 =
//!   12 aircraft of type 4; 900,002, bombers, 36 - 12 = 24 of type 3.
//!   All ready, at rest.
//! - It is at (5,2) in every world: a contact of age 0 is where it was
//!   reported.
//!
//! (Corrected by the architect on Nov 19, 2026: this said "class 4" alone.
//! The class file has a second carrier, class 5, which would then be
//! built in its place.) With the file of sides giving every carrier
//! class, 4 and 5, to the Allied side alone: the
//! carrier is not built, the force is the four destroyers, 900,001 to
//! 900,004, on the carrier mission with no air group, and the
//! assumptions say one carrier was left out for want of a class.
//!
//! ## 3. A transport contact
//!
//! One contact: 3 ships taken for a transport, at (5,2), age 0. Three
//! ships of class 3, 900,001 to 900,003, in task force 900,001 on the
//! transport mission. No destroyer.
//!
//! ## 4. A claim raises the kind; a claim of sinkings takes nothing
//!
//! One contact: 3 ships taken for a cruiser, at (5,2), age 0.
//!
//! - **As it stands:** ship 900,001 of class 1 and ships 900,002 and
//!   900,003 of class 2; task force 900,001 on the surface combat
//!   mission.
//! - **With a claim of the Allied side's,** from the air, of 1 hit on a
//!   carrier at (6,2) on day 0 (the test adds it to the side's half; a
//!   hex is 1 from the contact, inside the 2 of merging): ship 900,001
//!   is of class 4, with its two air groups of 12 and 24; ships 900,002
//!   and 900,003 are of class 2; the mission is carrier. The assumptions
//!   say the claim raised the kind.
//! - **With that claim at (8,2),** 3 hexes away: as it stands, a
//!   cruiser.
//! - **With a claim from the air of 2 ships sunk and 4 hits on a
//!   cruiser at (5,2):** three ships, as it stands. Nothing is taken
//!   away.
//!
//! ## 5. An older contact is where it could have steamed
//!
//! - **One pulse old.** One contact: 3 cruisers at (5,2), age 1. Its
//!   hours are the afternoon's 6; at 16 kn that is 96 nm, 2 steps. In
//!   each of worlds 1 to 64 the force is at a hex no more than 2 steps
//!   from (5,2), one of 19; it is not at the same hex in all of them;
//!   and world 7 asked twice gives the same hex. The assumptions give
//!   the 6 hours, the 16 kn, the 96 nm and the 2 steps.
//! - **Three pulses old,** a day. Age 3: 6 + 6 + 12 = 24 hours, 384 nm,
//!   9 steps. In each of worlds 1 to 64 the force is within 9 steps of
//!   (5,2).
//! - **Slower ships reach less far.** 3 transports at (5,2), age 1:
//!   6 hours at 10 kn, 60 nm, 1 step. In each of worlds 1 to 64 the
//!   force is within 1 step of (5,2), one of 7 hexes.
//!
//! ## 6. Two reports that may be one force
//!
//! Two contacts: the first, 3 cruisers at (2,2), age 2; the second, 2
//! cruisers at (6,2), age 0. They are 4 hexes apart. The pulses between
//! their ages are two, the afternoon and the morning: 12 hours at 16 kn,
//! 192 nm, 4 steps; with the merging distance, 6. 4 is no more than 6:
//! they may be one.
//!
//! - **In a world where they are one:** one force, at (6,2), of 3
//!   ships, one cruiser of class 1 and two destroyers.
//! - **In a world where they are two:** a force of 2 ships at (6,2), one
//!   cruiser and one destroyer; and a force of 3 ships, one cruiser and
//!   two destroyers, within 4 steps of (2,2).
//! - **Over worlds 1 to 1000** the worlds where they are one number
//!   between 437 and 563: an even chance, a mean of 500, a standard
//!   deviation of the root of 250, 15.81, and 3.94 of them either side.
//!   The assumptions of each world say which it is, and the same world
//!   asked twice is the same.
//! - **With the first at (-1,2),** 7 hexes from the second: they are two
//!   forces in every world.
//!
//! ## 7. The forces believed in and not seen
//!
//! No contact. The picture is of the evening of day 2, after two days in
//! which nothing happens. Eight worlds, 1 to 8.
//!
//! - **Believed present:** a cruiser force of 3 ships, last known at
//!   (5,2) before the first day. It is built in all 8 worlds: one
//!   cruiser and two destroyers. Its hours are 2 days, 48; at 16 kn,
//!   768 nm, 19 steps. In every world it is within 19 steps of (5,2).
//!   The assumptions give the 48 hours, the 768 nm and the 19 steps.
//! - **Believed absent:** the same force, standing absent. Built in no
//!   world. The assumptions say the side ruled it out.
//! - **Unknown:** the same force, standing unknown. Built in worlds 2,
//!   4, 6 and 8 and not in worlds 1, 3, 5 and 7: 4 of 8.
//! - **Two unknown,** that cruiser force and then a carrier force of 5
//!   ships, place not known. The first is in worlds 2, 4, 6 and 8; the
//!   second in worlds 3, 4, 7 and 8. Both in 4 and 8; neither in 1 and
//!   5. The carrier force, its place not known, is at base 2's hex,
//!   (8,3), the one base of the enemy's on the map.
//! - **A contact stands for it.** In a picture of the evening before
//!   day 1, where a contact given at the start is still held: the
//!   carrier force unknown and of no known place, and a contact of 2
//!   ships taken for a carrier at (5,2), age 0:
//!   in every world the enemy has one carrier force, the contact's, of
//!   2 ships, and the believed force is not built. The assumptions say
//!   the contact was taken for it. With the contact taken for a
//!   cruiser instead, the believed carrier force is built in its
//!   worlds, beside the contact's cruiser force.
//!
//! ## 8. The men ashore
//!
//! An estimate of the Allied side's: 600 men at (8,3). The enemy's half
//! has land unit 900,001, a fighting unit of 600 men, ashore at (8,3),
//! on defend.
//!
//! ## 9. What the enemy wants
//!
//! The case of 8, and each world built and resolved for two days with
//! both staffs.
//!
//! - **Believed to hold base 2:** at the end of day 1 the Japanese
//!   side's command state has one intent of the commander's, to hold
//!   base 2, set on day 1, and both its leaves are given.
//! - **No belief of what he wants:** the Japanese side has no intent,
//!   and its staff gives no order on either day.
//! - In both, the Japanese side's belief is empty when the world is
//!   built.
//!
//! ## 10. Each hard case gives a world
//!
//! Each of these pictures builds a world that resolves for two days with
//! both staffs, its invariants holding: case 1 (no contact); case 5's
//! day-old contact; case 6 in a world of each kind; case 4's claim of
//! two sunk; case 4's claim that raises a cruiser to a carrier (a
//! contact of the wrong kind); case 7's unknown carrier in a world that
//! has it and in one that has not (an enemy carrier never seen).
//!
//! # Properties
//!
//! - **No peek.** Scenario 68's pair of worlds that differ only in what
//!   the Allied side cannot see, each played to the evening of day 5:
//!   the Allied side's pictures of worlds 1 to 8 are equal, world for
//!   world, field for field, and so are their assumptions.
//! - **The same twice.** The same view, beliefs, seed, trial and world
//!   give the same picture and the same assumptions.
//! - **Another trial, other draws.** Case 5's contact one pulse old, in
//!   trial 2: its hexes over worlds 1 to 64 are not, world for world,
//!   those of trial 1.
//! - **Nothing of the live game is read but the view.** The crate's own
//!   source test passes: it names no umpire, no whole picture and no
//!   rebuild on a live seed.
//! - **The live game is not touched.** Making 64 pictures and building
//!   and resolving their worlds leaves the live game's hash what it was.
//! - **No live game changes** by the build: no baseline, golden file or
//!   pinned hash moves.

mod common;

use std::collections::BTreeSet;
use std::path::PathBuf;

use sim_core::command::{IntentKind, Maker};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, AircraftTypeId, BaseId, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::{AirMission, Mission, Posture, Readiness, SpeedSetting};
use sim_core::refdata::{ShipKind, TrialFigures};
use sim_core::state::{AirGroupLocation, ShipLocation, Side, TaskForceStatus};
use sim_core::time::Pulse;
use sim_core::world_picture::PictureClaim;
use sim_core::World;
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, staffs};
use sim_trial::{
    build_world, pictures, pictures_from_half, Assumption, Believed, Source, Trial, TrialPicture,
};

const ALLIED: Side = Side::Allied;
const JAPANESE: Side = Side::Japanese;

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn group(raw: u32) -> AirGroupId {
    AirGroupId::new(raw)
}

fn kn(n: i64) -> Fixed {
    Fixed::from_int(n)
}

fn path() -> PathBuf {
    common::data_dir().join("scenarios/93_the_enemys_half.toml")
}

/// Scenario 93 with `extra` added at the end of its file, on the frozen
/// figures (or on `options`).
fn scenario_with(extra: &str, options: &LoadOptions) -> Scenario {
    let text = std::fs::read_to_string(path()).expect("the scenario file reads");
    common::from_text_with(&format!("{text}\n{extra}"), &path(), options)
        .expect("scenario 93 loads")
}

fn scenario(extra: &str) -> Scenario {
    scenario_with(extra, &common::frozen())
}

fn contact(q: i32, r: i32, ships: u32, kind: &str, age: u32) -> String {
    format!(
        "[[contact]]\nside = \"allied\"\nq = {q}\nr = {r}\nship_count = {ships}\nkind = \
         \"{kind}\"\nage_pulses = {age}\n\n"
    )
}

fn believed_force(
    name: &str,
    kind: &str,
    ships: u32,
    standing: &str,
    at: Option<(i32, i32)>,
) -> String {
    let place = match at {
        Some((q, r)) => format!("q = {q}\nr = {r}\nday = 0\n"),
        None => String::new(),
    };
    format!(
        "[[believed_force]]\nside = \"allied\"\nname = \"{name}\"\nkind = \"{kind}\"\nships = \
         {ships}\nstanding = \"{standing}\"\n{place}placeholder = true\nnote = \"Scenario 93, \
         a case of the hand calculation.\"\n\n"
    )
}

const ESTIMATE: &str = "[[estimate]]\nside = \"allied\"\nq = 8\nr = 3\nmen = 600\n\n";

const HOLD_BASE_2: &str = "[[believed_intent]]\nside = \"allied\"\nintent_kind = \"hold\"\nbase \
                           = 2\nplaceholder = true\nnote = \"Scenario 93, case 9.\"\n\n";

fn believed(scenario: &Scenario) -> Believed {
    Believed {
        forces: scenario.believed_forces(ALLIED).cloned().collect(),
        intents: scenario.believed_intents(ALLIED).cloned().collect(),
    }
}

fn trial(trial: u32, worlds: u32) -> Trial {
    Trial {
        seed: 1942,
        trial,
        worlds,
    }
}

fn figures(scenario: &Scenario) -> TrialFigures {
    scenario
        .trial_figures()
        .cloned()
        .expect("scenario 93 names its trial's figures")
}

/// The Allied side's pictures of worlds 1 to `worlds` of trial `number`.
fn pictures_of(scenario: &Scenario, number: u32, worlds: u32) -> Vec<TrialPicture> {
    pictures(
        &scenario.world.side_view(ALLIED),
        &believed(scenario),
        &figures(scenario),
        trial(number, worlds),
    )
    .expect("the pictures are made")
}

fn made(extra: &str, worlds: u32) -> Vec<TrialPicture> {
    pictures_of(&scenario(extra), 1, worlds)
}

/// The enemy's task forces of a picture, in the order of their numbers.
fn enemy_forces(picture: &TrialPicture) -> Vec<&sim_core::state::TaskForce> {
    picture
        .picture
        .truth
        .task_forces
        .values()
        .filter(|f| f.side == JAPANESE)
        .collect()
}

/// The classes of a force's ships, in the order of their numbers.
fn classes(picture: &TrialPicture, id: TaskForceId) -> Vec<u32> {
    picture.picture.truth.task_forces[&id]
        .ships
        .iter()
        .map(|s| picture.picture.truth.ships[s].class.raw())
        .collect()
}

fn ships_of(picture: &TrialPicture, id: TaskForceId) -> Vec<u32> {
    picture.picture.truth.task_forces[&id]
        .ships
        .iter()
        .map(|s| s.raw())
        .collect()
}

fn the_only_force(picture: &TrialPicture) -> &sim_core::state::TaskForce {
    let forces = enemy_forces(picture);
    assert_eq!(
        forces.len(),
        1,
        "one enemy force in world {}",
        picture.world
    );
    forces[0]
}

/// The hexes within `steps` by sea of `from` on scenario 93's map.
fn within(scenario: &Scenario, from: Hex, steps: u32) -> BTreeSet<Hex> {
    scenario
        .world
        .map()
        .sea_distances_to(from)
        .into_iter()
        .filter(|(_, d)| *d <= steps)
        .map(|(hex, _)| hex)
        .collect()
}

fn placed(picture: &TrialPicture, id: TaskForceId) -> (u32, Fixed, Fixed, u32) {
    picture
        .assumptions
        .iter()
        .find_map(|a| match a {
            Assumption::Placed {
                force,
                hours,
                speed_kn,
                miles,
                steps,
                ..
            } if *force == id => Some((*hours, *speed_kn, *miles, *steps)),
            _ => None,
        })
        .expect("the force's place is assumed")
}

fn pair_is_one(picture: &TrialPicture) -> Option<bool> {
    picture.assumptions.iter().find_map(|a| match a {
        Assumption::Pair { one, .. } => Some(*one),
        _ => None,
    })
}

/// Builds the world of `picture` and resolves two days with both staffs.
fn built_and_resolved(scenario: &Scenario, picture: &TrialPicture) -> World {
    let mut world = build_world(
        picture,
        scenario.world.ref_data().clone(),
        scenario.world.map().clone(),
    )
    .expect("the picture builds a world");
    let held = staffs();
    for _ in 0..2 {
        world
            .resolve_day_staffed(as_staffs(&held))
            .expect("the built world resolves a day, its invariants holding");
    }
    world
}

/// Scenario 93 played with both staffs to the evening of day 2.
fn after_two_days(extra: &str) -> Scenario {
    let mut game = scenario(extra);
    let held = staffs();
    for _ in 1..=2 {
        game.next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day")
            .expect("a day exists");
    }
    assert_eq!(game.world.day(), 2);
    game
}

// ---------------------------------------------------------------------------
// 1. Nothing held, nothing believed.
// ---------------------------------------------------------------------------

#[test]
fn case_1_with_nothing_held_and_nothing_believed_the_enemy_is_his_base() {
    let made = made("", 2);
    for picture in &made {
        let truth = &picture.picture.truth;
        let base = &truth.bases[&BaseId::new(2)];
        assert_eq!(base.side, JAPANESE);
        assert_eq!(base.hex, Hex::new(8, 3));
        assert_eq!(base.supply_t, kn(1000));
        assert_eq!(base.fuel_t, kn(500));
        assert_eq!(base.anti_aircraft, 0);
        assert!(base.airfield);
        assert_eq!(
            truth.bases.values().filter(|b| b.side == JAPANESE).count(),
            1
        );
        assert!(truth.ships.values().all(|s| s.side == ALLIED));
        assert!(truth.task_forces.values().all(|f| f.side == ALLIED));
        assert!(truth.air_groups.is_empty());
        assert!(truth.land_units.is_empty());
        assert_eq!(
            picture.assumptions,
            vec![
                Assumption::EnemyBase {
                    base: BaseId::new(2),
                    hex: Hex::new(8, 3),
                    supply_t: kn(1000),
                    fuel_t: kn(500),
                },
                Assumption::EnemyGivenNoBelief,
                Assumption::EnemyGivenNoIntent,
            ]
        );
    }
    assert_eq!(made[0].picture, made[1].picture);
    assert_eq!(made[0].assumptions, made[1].assumptions);
    assert_eq!((made[0].world, made[1].world), (1, 2));
}

// ---------------------------------------------------------------------------
// 2. A carrier contact of 5 ships.
// ---------------------------------------------------------------------------

#[test]
fn case_2_a_carrier_contact_of_five_is_a_carrier_and_four_destroyers() {
    let scenario = scenario(&contact(5, 2, 5, "carrier", 0));
    let made = pictures_of(&scenario, 1, 8);
    for picture in &made {
        let truth = &picture.picture.truth;
        let tf = the_only_force(picture);
        assert_eq!(tf.id, force(900_001));
        assert_eq!(
            ships_of(picture, tf.id),
            vec![900_001, 900_002, 900_003, 900_004, 900_005]
        );
        assert_eq!(classes(picture, tf.id), vec![4, 2, 2, 2, 2]);
        for id in &tf.ships {
            let s = &truth.ships[id];
            let figures = &scenario.world.ref_data().ship_classes[&s.class];
            assert_eq!(s.side, JAPANESE);
            assert_eq!(s.fuel_t, figures.fuel_capacity_t);
            assert_eq!(s.flotation_damage, Fixed::ZERO);
            assert_eq!(s.fire_damage, Fixed::ZERO);
            assert_eq!(s.location, ShipLocation::TaskForce(tf.id));
        }
        assert_eq!(truth.ships[&ship(900_001)].fuel_t, kn(3000));
        assert_eq!(truth.ships[&ship(900_002)].fuel_t, kn(500));
        assert_eq!(tf.side, JAPANESE);
        assert_eq!(tf.from_hex, Hex::new(5, 2));
        assert_eq!(tf.position(), Hex::new(5, 2));
        assert_eq!(tf.mission, Mission::Carrier);
        assert_eq!(tf.status, TaskForceStatus::Idle);
        assert_eq!(tf.speed, SpeedSetting::Cruise);
        assert_eq!(tf.readiness, Readiness::Normal);
        assert!(tf.route.waypoints.is_empty());
        assert_eq!(tf.home_port, None);
        let groups: Vec<_> = truth
            .air_groups
            .values()
            .filter(|g| g.side == JAPANESE)
            .collect();
        assert_eq!(groups.len(), 2);
        let fighters = &truth.air_groups[&group(900_001)];
        let bombers = &truth.air_groups[&group(900_002)];
        assert_eq!(
            (fighters.ready, fighters.aircraft_type),
            (12, AircraftTypeId::new(4))
        );
        assert_eq!(
            (bombers.ready, bombers.aircraft_type),
            (24, AircraftTypeId::new(3))
        );
        for g in [fighters, bombers] {
            assert_eq!(g.location, AirGroupLocation::Ship(ship(900_001)));
            assert_eq!(g.mission, AirMission::Rest);
            assert_eq!(g.damaged, 0);
        }
    }
}

#[test]
fn case_2_with_the_carrier_classes_the_allied_sides_alone_the_carrier_is_left_out() {
    // The hand calculation gives class 4 to the Allied side alone. The
    // frozen class file has a second carrier class, 5, which rule 8 would
    // take in its place: it is given to the Allied side alone too, so that
    // the enemy has no class of carrier.
    let options = common::altered_reference("scenario_93_no_carrier", |dir| {
        common::edit_file_by(
            dir,
            "class_sides_placeholder_scenario_93.csv",
            &[
                ("ship_class,4,both,", "ship_class,4,allied,"),
                ("ship_class,5,both,", "ship_class,5,allied,"),
            ],
        );
    });
    let scenario = scenario_with(&contact(5, 2, 5, "carrier", 0), &options);
    for picture in pictures_of(&scenario, 1, 4) {
        let tf = the_only_force(&picture);
        assert_eq!(
            ships_of(&picture, tf.id),
            vec![900_001, 900_002, 900_003, 900_004]
        );
        assert_eq!(classes(&picture, tf.id), vec![2, 2, 2, 2]);
        assert_eq!(tf.mission, Mission::Carrier);
        assert!(picture.picture.truth.air_groups.is_empty());
        assert!(picture.assumptions.contains(&Assumption::ShipsLeftOut {
            source: Source::Contact(1),
            kind: ShipKind::Carrier,
            count: 1,
        }));
    }
}

// ---------------------------------------------------------------------------
// 3. A transport contact.
// ---------------------------------------------------------------------------

#[test]
fn case_3_a_transport_contact_is_three_transports_and_no_destroyer() {
    for picture in made(&contact(5, 2, 3, "transport", 0), 4) {
        let tf = the_only_force(&picture);
        assert_eq!(tf.id, force(900_001));
        assert_eq!(ships_of(&picture, tf.id), vec![900_001, 900_002, 900_003]);
        assert_eq!(classes(&picture, tf.id), vec![3, 3, 3]);
        assert_eq!(tf.mission, Mission::Transport);
    }
}

// ---------------------------------------------------------------------------
// 4. A claim raises the kind; a claim of sinkings takes nothing.
// ---------------------------------------------------------------------------

fn claim(q: i32, r: i32, sunk: u32, hits: u32, kind: ShipKind) -> PictureClaim {
    PictureClaim {
        hex: Hex::new(q, r),
        day: 0,
        pulse: Pulse::Afternoon,
        claimed_sunk: sunk,
        claimed_hits: hits,
        claimed_kind: Some(kind),
        claimed_shot_down: 0,
        claimed_on_ground: 0,
        from_air: true,
        from_bombardment: false,
    }
}

/// Case 4's pictures, with `claims` added to the Allied side's half.
fn with_claims(claims: &[PictureClaim], worlds: u32) -> Vec<TrialPicture> {
    let scenario = scenario(&contact(5, 2, 3, "cruiser", 0));
    let view = scenario.world.side_view(ALLIED);
    let mut half = view.own_half().expect("a half");
    half.belief.claims.extend_from_slice(claims);
    pictures_from_half(
        &half,
        &view,
        &believed(&scenario),
        &figures(&scenario),
        trial(1, worlds),
    )
    .expect("the pictures are made")
}

fn assert_a_cruiser_as_it_stands(picture: &TrialPicture) {
    let tf = the_only_force(picture);
    assert_eq!(ships_of(picture, tf.id), vec![900_001, 900_002, 900_003]);
    assert_eq!(classes(picture, tf.id), vec![1, 2, 2]);
    assert_eq!(tf.mission, Mission::SurfaceCombat);
    assert!(picture.picture.truth.air_groups.is_empty());
    assert!(!picture
        .assumptions
        .iter()
        .any(|a| matches!(a, Assumption::KindRaised { .. })));
}

#[test]
fn case_4_a_cruiser_contact_as_it_stands() {
    for picture in with_claims(&[], 4) {
        assert_a_cruiser_as_it_stands(&picture);
    }
}

#[test]
fn case_4_a_claim_on_a_carrier_one_hex_away_raises_the_cruiser_to_a_carrier() {
    for picture in with_claims(&[claim(6, 2, 0, 1, ShipKind::Carrier)], 4) {
        let truth = &picture.picture.truth;
        let tf = the_only_force(&picture);
        assert_eq!(ships_of(&picture, tf.id), vec![900_001, 900_002, 900_003]);
        assert_eq!(classes(&picture, tf.id), vec![4, 2, 2]);
        assert_eq!(tf.mission, Mission::Carrier);
        assert_eq!(truth.air_groups[&group(900_001)].ready, 12);
        assert_eq!(truth.air_groups[&group(900_002)].ready, 24);
        assert!(picture.assumptions.contains(&Assumption::KindRaised {
            contact: 1,
            claim: 1,
            from: ShipKind::Cruiser,
            to: ShipKind::Carrier,
        }));
    }
}

#[test]
fn case_4_a_claim_three_hexes_away_raises_nothing() {
    for picture in with_claims(&[claim(8, 2, 0, 1, ShipKind::Carrier)], 4) {
        assert_a_cruiser_as_it_stands(&picture);
    }
}

#[test]
fn case_4_a_claim_of_two_sunk_takes_nothing_away() {
    for picture in with_claims(&[claim(5, 2, 2, 4, ShipKind::Cruiser)], 4) {
        assert_a_cruiser_as_it_stands(&picture);
    }
}

// ---------------------------------------------------------------------------
// 5. An older contact is where it could have steamed.
// ---------------------------------------------------------------------------

#[test]
fn case_5_a_contact_one_pulse_old_is_within_two_steps() {
    let scenario = scenario(&contact(5, 2, 3, "cruiser", 1));
    let reach = within(&scenario, Hex::new(5, 2), 2);
    assert_eq!(reach.len(), 19);
    let made = pictures_of(&scenario, 1, 64);
    let mut hexes = BTreeSet::new();
    for picture in &made {
        let tf = the_only_force(picture);
        assert!(
            reach.contains(&tf.from_hex),
            "world {}: {}",
            picture.world,
            tf.from_hex
        );
        assert!(Hex::new(5, 2).distance(tf.from_hex) <= 2);
        hexes.insert(tf.from_hex);
        assert_eq!(placed(picture, tf.id), (6, kn(16), kn(96), 2));
    }
    assert!(hexes.len() > 1, "not at one hex in every world");
    let again = pictures_of(&scenario, 1, 7);
    assert_eq!(again[6], made[6]);
    assert_eq!(
        the_only_force(&again[6]).from_hex,
        the_only_force(&made[6]).from_hex
    );
}

#[test]
fn case_5_a_contact_a_day_old_is_within_nine_steps() {
    let scenario = scenario(&contact(5, 2, 3, "cruiser", 3));
    for picture in pictures_of(&scenario, 1, 64) {
        let tf = the_only_force(&picture);
        assert!(Hex::new(5, 2).distance(tf.from_hex) <= 9);
        assert!(within(&scenario, Hex::new(5, 2), 9).contains(&tf.from_hex));
        assert_eq!(placed(&picture, tf.id), (24, kn(16), kn(384), 9));
    }
}

#[test]
fn case_5_slower_ships_reach_less_far() {
    let scenario = scenario(&contact(5, 2, 3, "transport", 1));
    let reach = within(&scenario, Hex::new(5, 2), 1);
    assert_eq!(reach.len(), 7);
    for picture in pictures_of(&scenario, 1, 64) {
        let tf = the_only_force(&picture);
        assert!(reach.contains(&tf.from_hex));
        assert_eq!(placed(&picture, tf.id), (6, kn(10), kn(60), 1));
    }
}

// ---------------------------------------------------------------------------
// 6. Two reports that may be one force.
// ---------------------------------------------------------------------------

fn case_6(first_q: i32) -> Scenario {
    scenario(&format!(
        "{}{}",
        contact(first_q, 2, 3, "cruiser", 2),
        contact(6, 2, 2, "cruiser", 0)
    ))
}

#[test]
fn case_6_two_reports_are_one_force_in_some_worlds_and_two_in_others() {
    let scenario = case_6(2);
    let made = pictures_of(&scenario, 1, 1000);
    let reach = within(&scenario, Hex::new(2, 2), 4);
    let mut ones = 0;
    for picture in &made {
        let one = pair_is_one(picture).expect("the pair is named in every world");
        let forces = enemy_forces(picture);
        if one {
            ones += 1;
            assert_eq!(forces.len(), 1);
            assert_eq!(forces[0].from_hex, Hex::new(6, 2));
            assert_eq!(classes(picture, forces[0].id), vec![1, 2, 2]);
            assert_eq!(forces[0].mission, Mission::SurfaceCombat);
        } else {
            assert_eq!(forces.len(), 2);
            let at_six: Vec<_> = forces
                .iter()
                .filter(|f| f.from_hex == Hex::new(6, 2))
                .collect();
            let two = forces
                .iter()
                .find(|f| f.ships.len() == 2)
                .expect("a force of 2");
            let three = forces
                .iter()
                .find(|f| f.ships.len() == 3)
                .expect("a force of 3");
            assert_eq!(two.from_hex, Hex::new(6, 2));
            assert!(!at_six.is_empty());
            assert_eq!(classes(picture, two.id), vec![1, 2]);
            assert_eq!(classes(picture, three.id), vec![1, 2, 2]);
            assert!(reach.contains(&three.from_hex), "world {}", picture.world);
        }
    }
    println!("case 6: one force in {ones} of 1000 worlds");
    assert!(
        (437..=563).contains(&ones),
        "{ones} worlds of 1000 are one force"
    );
    let again = pictures_of(&scenario, 1, 1000);
    assert_eq!(again, made, "the same worlds asked twice are the same");
}

#[test]
fn case_6_seven_hexes_apart_they_are_two_forces_in_every_world() {
    for picture in pictures_of(&case_6(-1), 1, 64) {
        assert_eq!(pair_is_one(&picture), None);
        assert_eq!(enemy_forces(&picture).len(), 2);
    }
}

// ---------------------------------------------------------------------------
// 7. The forces believed in and not seen.
// ---------------------------------------------------------------------------

const CRUISERS: &str = "Placeholder cruiser force";
const CARRIERS: &str = "Placeholder carrier force";

fn cruisers(standing: &str) -> String {
    believed_force(CRUISERS, "cruiser", 3, standing, Some((5, 2)))
}

fn carriers_of_no_place() -> String {
    believed_force(CARRIERS, "carrier", 5, "unknown", None)
}

/// The worlds of 1 to 8 with an enemy force of `ships` ships.
fn worlds_with(made: &[TrialPicture], ships: usize) -> Vec<u32> {
    made.iter()
        .filter(|p| enemy_forces(p).iter().any(|f| f.ships.len() == ships))
        .map(|p| p.world)
        .collect()
}

#[test]
fn case_7_a_force_believed_present_is_in_every_world_within_its_reach() {
    let scenario = after_two_days(&cruisers("present"));
    let made = pictures_of(&scenario, 1, 8);
    for picture in &made {
        assert_eq!(picture.picture.day, 2);
        let tf = the_only_force(picture);
        assert_eq!(classes(picture, tf.id), vec![1, 2, 2]);
        assert!(Hex::new(5, 2).distance(tf.from_hex) <= 19);
        assert_eq!(placed(picture, tf.id), (48, kn(16), kn(768), 19));
    }
}

#[test]
fn case_7_a_force_believed_absent_is_in_no_world() {
    let scenario = after_two_days(&cruisers("absent"));
    for picture in pictures_of(&scenario, 1, 8) {
        assert!(enemy_forces(&picture).is_empty());
        assert!(picture.assumptions.contains(&Assumption::RuledOut {
            place: 1,
            name: CRUISERS.to_string()
        }));
    }
}

#[test]
fn case_7_a_force_of_unknown_standing_is_in_half_the_worlds() {
    let scenario = after_two_days(&cruisers("unknown"));
    let made = pictures_of(&scenario, 1, 8);
    assert_eq!(worlds_with(&made, 3), vec![2, 4, 6, 8]);
}

#[test]
fn case_7_two_unknown_forces_are_tried_in_every_combination() {
    let scenario = after_two_days(&format!(
        "{}{}",
        cruisers("unknown"),
        carriers_of_no_place()
    ));
    let made = pictures_of(&scenario, 1, 8);
    assert_eq!(worlds_with(&made, 3), vec![2, 4, 6, 8]);
    assert_eq!(worlds_with(&made, 5), vec![3, 4, 7, 8]);
    for picture in &made {
        let forces = enemy_forces(picture);
        match picture.world {
            4 | 8 => assert_eq!(forces.len(), 2),
            1 | 5 => assert!(forces.is_empty()),
            _ => assert_eq!(forces.len(), 1),
        }
        if let Some(carrier) = forces.iter().find(|f| f.ships.len() == 5) {
            assert_eq!(carrier.from_hex, Hex::new(8, 3));
            assert_eq!(carrier.mission, Mission::Carrier);
            assert!(picture.assumptions.contains(&Assumption::AtEnemyBase {
                force: carrier.id,
                base: BaseId::new(2),
                hex: Hex::new(8, 3),
            }));
        }
    }
}

#[test]
fn case_7_a_contact_stands_for_a_believed_force_of_its_kind() {
    let made = made(
        &format!(
            "{}{}",
            carriers_of_no_place(),
            contact(5, 2, 2, "carrier", 0)
        ),
        8,
    );
    for picture in &made {
        let tf = the_only_force(picture);
        assert_eq!(tf.ships.len(), 2);
        assert_eq!(tf.from_hex, Hex::new(5, 2));
        assert_eq!(tf.mission, Mission::Carrier);
        let taken = Assumption::TakenForContact {
            place: 1,
            name: CARRIERS.to_string(),
            contact: 1,
        };
        // In the worlds the unknown force is in, the contact is taken for
        // it; in the others it is not in the world at all.
        assert_eq!(
            picture.assumptions.contains(&taken),
            picture.world % 2 == 0,
            "world {}",
            picture.world
        );
    }
}

#[test]
fn case_7_a_contact_of_another_kind_stands_for_nothing() {
    let made = made(
        &format!(
            "{}{}",
            carriers_of_no_place(),
            contact(5, 2, 2, "cruiser", 0)
        ),
        8,
    );
    assert_eq!(worlds_with(&made, 5), vec![2, 4, 6, 8]);
    for picture in &made {
        let forces = enemy_forces(picture);
        assert!(forces
            .iter()
            .any(|f| f.ships.len() == 2 && f.mission == Mission::SurfaceCombat));
        assert_eq!(forces.len(), if picture.world % 2 == 0 { 2 } else { 1 });
    }
}

// ---------------------------------------------------------------------------
// 8. The men ashore.
// ---------------------------------------------------------------------------

#[test]
fn case_8_an_estimate_is_a_fighting_unit_of_its_men_on_defend() {
    for picture in made(ESTIMATE, 2) {
        let units = &picture.picture.truth.land_units;
        assert_eq!(units.len(), 1);
        let unit = &units[&LandUnitId::new(900_001)];
        assert_eq!(unit.side, JAPANESE);
        assert_eq!(unit.men_ashore, 600);
        assert_eq!(unit.hex, Some(Hex::new(8, 3)));
        assert_eq!(unit.posture, Posture::Defend);
        assert!(!unit.working);
        assert!(picture.assumptions.contains(&Assumption::MenAshore {
            unit: LandUnitId::new(900_001),
            hex: Hex::new(8, 3),
            men: 600,
        }));
    }
}

// ---------------------------------------------------------------------------
// 9. What the enemy wants.
// ---------------------------------------------------------------------------

#[test]
fn case_9_an_enemy_believed_to_hold_base_2_is_given_that_intent_and_his_leaves() {
    let scenario = scenario(&format!("{ESTIMATE}{HOLD_BASE_2}"));
    for picture in pictures_of(&scenario, 1, 4) {
        assert!(picture.assumptions.contains(&Assumption::EnemyIntent {
            kind: IntentKind::Hold {
                base: BaseId::new(2)
            }
        }));
        let mut world = build_world(
            &picture,
            scenario.world.ref_data().clone(),
            scenario.world.map().clone(),
        )
        .expect("the world builds");
        let japanese = world.side_view(JAPANESE);
        assert_eq!(japanese.contacts().count(), 0);
        assert!(japanese.claims().is_empty());
        assert_eq!(japanese.ground_estimates().count(), 0);
        let held = staffs();
        world.resolve_day_staffed(as_staffs(&held)).expect("day 1");
        let command = world.command_state(JAPANESE);
        let roots: Vec<_> = command
            .intents
            .values()
            .filter(|intent| intent.parent.is_none())
            .collect();
        assert_eq!(roots.len(), 1);
        assert_eq!(
            roots[0].kind,
            IntentKind::Hold {
                base: BaseId::new(2)
            }
        );
        assert_eq!(roots[0].maker, Maker::Commander);
        assert_eq!(roots[0].set_day, 1);
        assert!(command.leaves.prepare && command.leaves.launch);
        world.resolve_day_staffed(as_staffs(&held)).expect("day 2");
    }
}

#[test]
fn case_9_an_enemy_of_whom_nothing_is_believed_has_no_intent_and_gives_no_order() {
    let scenario = scenario(ESTIMATE);
    for picture in pictures_of(&scenario, 1, 4) {
        assert!(picture
            .assumptions
            .contains(&Assumption::EnemyGivenNoIntent));
        let mut world = build_world(
            &picture,
            scenario.world.ref_data().clone(),
            scenario.world.map().clone(),
        )
        .expect("the world builds");
        let japanese = world.side_view(JAPANESE);
        assert_eq!(japanese.contacts().count(), 0);
        assert!(japanese.claims().is_empty());
        assert_eq!(japanese.ground_estimates().count(), 0);
        let held = staffs();
        for _ in 0..2 {
            world.resolve_day_staffed(as_staffs(&held)).expect("a day");
            assert!(world.command_state(JAPANESE).intents.is_empty());
            let handed = world.staff_handed_in().get(&JAPANESE);
            assert!(
                handed.is_none_or(|decisions| decisions.orders.is_empty()),
                "the Japanese staff gives no order"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 10. Each hard case gives a world.
// ---------------------------------------------------------------------------

#[test]
fn case_10_each_hard_case_builds_a_world_that_resolves() {
    // Case 1: no contact.
    let plain = scenario("");
    for picture in pictures_of(&plain, 1, 2) {
        built_and_resolved(&plain, &picture);
    }
    // Case 5's contact a day old.
    let old = scenario(&contact(5, 2, 3, "cruiser", 3));
    for picture in pictures_of(&old, 1, 4) {
        built_and_resolved(&old, &picture);
    }
    // Case 6, a world of each kind.
    let two = case_6(2);
    let made = pictures_of(&two, 1, 16);
    let one = made
        .iter()
        .find(|p| pair_is_one(p) == Some(true))
        .expect("a world of one");
    let apart = made
        .iter()
        .find(|p| pair_is_one(p) == Some(false))
        .expect("a world of two");
    built_and_resolved(&two, one);
    built_and_resolved(&two, apart);
    // Case 4's claim of two sunk, and its claim that raises the kind.
    let cruiser = scenario(&contact(5, 2, 3, "cruiser", 0));
    for claims in [
        [claim(5, 2, 2, 4, ShipKind::Cruiser)],
        [claim(6, 2, 0, 1, ShipKind::Carrier)],
    ] {
        for picture in with_claims(&claims, 2) {
            built_and_resolved(&cruiser, &picture);
        }
    }
    // Case 7's unknown carrier, in a world that has it and one that has not.
    let unseen = after_two_days(&format!(
        "{}{}",
        cruisers("unknown"),
        carriers_of_no_place()
    ));
    let made = pictures_of(&unseen, 1, 8);
    for world in [3, 1] {
        let picture = &made[world - 1];
        let has = enemy_forces(picture).iter().any(|f| f.ships.len() == 5);
        assert_eq!(has, world == 3);
        let built = built_and_resolved(&unseen, picture);
        assert_eq!(built.day(), 4);
    }
}

// ---------------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------------

/// Scenario 68 with base 2's supply set to `supply_t`, played with both
/// staffs to the evening of day 5 (as scenario 92's test plays it).
fn scenario_68_to_day_five(supply_t: u32) -> World {
    let path = common::data_dir().join("scenarios/68_ashore.toml");
    let text = std::fs::read_to_string(&path).expect("the scenario file reads");
    let text = common::replace_once(
        &text,
        "name = \"Placeholder airfield B\"\nside = \"japanese\"\nq = 4\nr = 0\nsupply_t = 200\n",
        &format!(
            "name = \"Placeholder airfield B\"\nside = \"japanese\"\nq = 4\nr = 0\nsupply_t = \
             {supply_t}\n"
        ),
    );
    let mut game = common::from_text(&text, &path).expect("the scenario loads");
    let held = staffs();
    for _ in 1..=5 {
        game.next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day")
            .expect("a day exists");
    }
    game.world
}

#[test]
fn property_no_peek_what_the_side_cannot_see_gives_it_the_same_pictures() {
    let base = scenario_68_to_day_five(200);
    let more = scenario_68_to_day_five(300);
    let b = BaseId::new(2);
    assert_ne!(
        base.umpire().bases[&b].supply_t,
        more.umpire().bases[&b].supply_t
    );
    let figures = TrialFigures {
        placeholder: true,
        enemy_base_supply_t: kn(1000),
        enemy_base_fuel_t: kn(500),
    };
    let make = |world: &World| {
        pictures(
            &world.side_view(ALLIED),
            &Believed::default(),
            &figures,
            trial(1, 8),
        )
        .expect("the pictures are made")
    };
    let (one, other) = (make(&base), make(&more));
    assert_eq!(one.len(), 8);
    for (a, b) in one.iter().zip(&other) {
        assert_eq!(a.picture, b.picture, "world {}", a.world);
        assert_eq!(a.assumptions, b.assumptions, "world {}", a.world);
    }
    // The enemy is made of what the side holds: its estimate ashore.
    assert!(one[0]
        .assumptions
        .iter()
        .any(|a| matches!(a, Assumption::MenAshore { .. })));
}

#[test]
fn property_the_same_inputs_give_the_same_pictures_twice() {
    let extra = format!(
        "{}{}{}{}",
        contact(5, 2, 3, "cruiser", 1),
        contact(2, 2, 2, "transport", 2),
        carriers_of_no_place(),
        ESTIMATE
    );
    let scenario = scenario(&extra);
    assert_eq!(pictures_of(&scenario, 1, 16), pictures_of(&scenario, 1, 16));
    let again = self::scenario(&extra);
    assert_eq!(pictures_of(&scenario, 1, 16), pictures_of(&again, 1, 16));
}

#[test]
fn property_another_trial_draws_other_hexes() {
    let scenario = scenario(&contact(5, 2, 3, "cruiser", 1));
    let hexes = |number| -> Vec<Hex> {
        pictures_of(&scenario, number, 64)
            .iter()
            .map(|p| the_only_force(p).from_hex)
            .collect()
    };
    assert_ne!(hexes(1), hexes(2));
}

#[test]
fn property_the_trial_names_no_umpire_whole_picture_or_rebuild() {
    // The crate's own source test (`crates/sim-trial/tests/source.rs`) bans
    // more; this repeats its three central words here, so that the scenario
    // says what it proves.
    let src = common::data_dir().join("../crates/sim-trial/src");
    let mut files = 0;
    for entry in std::fs::read_dir(src).expect("the source reads") {
        let path = entry.expect("an entry").path();
        let text = std::fs::read_to_string(&path).expect("a source file reads");
        files += 1;
        for line in text.lines().filter(|l| !l.trim_start().starts_with("//")) {
            for word in ["umpire", "whole_picture", "rebuild_from_picture"] {
                assert!(!line.contains(word), "{} names {word}", path.display());
            }
        }
    }
    assert!(files > 0);
}

#[test]
fn property_making_and_playing_pictures_leaves_the_live_game_as_it_was() {
    let mut live = scenario(&format!(
        "{}{}{}",
        contact(5, 2, 5, "carrier", 1),
        carriers_of_no_place(),
        ESTIMATE
    ));
    let before = live.world.state_hash();
    let made = pictures_of(&live, 1, 64);
    for picture in &made {
        built_and_resolved(&live, picture);
    }
    assert_eq!(live.world.state_hash(), before);
    // And the live game plays on as it would have.
    let mut untouched = scenario(&format!(
        "{}{}{}",
        contact(5, 2, 5, "carrier", 1),
        carriers_of_no_place(),
        ESTIMATE
    ));
    let held = staffs();
    for _ in 1..=2 {
        live.next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day")
            .expect("a day exists");
        untouched
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day")
            .expect("a day exists");
        assert_eq!(live.world.state_hash(), untouched.world.state_hash());
    }
}

#[test]
fn a_trial_is_refused_where_the_side_reaches_the_made_numbers_or_a_belief_is_not_its_own() {
    // Rule 20: a ship of the side's own numbered 900,001.
    let text = std::fs::read_to_string(path()).expect("the scenario file reads");
    let text = common::replace_once(&text, "[[ship]]\nid = 1\n", "[[ship]]\nid = 900001\n");
    let numbered = common::from_text(&text, &path()).expect("the scenario loads");
    let refused = pictures(
        &numbered.world.side_view(ALLIED),
        &Believed::default(),
        &figures(&numbered),
        trial(1, 1),
    )
    .expect_err("a ship numbered 900,001 refuses the trial");
    assert!(refused.0.contains("ship:900001"), "{refused}");

    // A belief of the other side's is not the tried side's.
    let scenario = scenario(&cruisers("present"));
    let mut beliefs = believed(&scenario);
    beliefs.forces[0].side = JAPANESE;
    let refused = pictures(
        &scenario.world.side_view(ALLIED),
        &beliefs,
        &figures(&scenario),
        trial(1, 1),
    )
    .expect_err("a Japanese belief refuses an Allied trial");
    assert!(refused.0.contains("the Japanese side's"), "{refused}");
}
