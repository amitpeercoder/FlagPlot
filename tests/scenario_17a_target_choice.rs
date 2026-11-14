//! Which enemy ship a ship fires at (decision 0078).
//!
//! Guns go like at like: a ship fires at enemy ships of its own kind; if none
//! is afloat, at those of the nearest more important kind; if none of those,
//! at those of the most important kind afloat; and among them at the ship in
//! place (p modulo their number), where p is the firing ship's place among
//! the ships of its own kind in its force. Torpedoes go at the enemy ships of
//! the most important kind afloat, at the ship in place (i modulo their
//! number), where i is the firing ship's place in its force. Kinds in order
//! of importance: carrier, battleship, cruiser, destroyer, transport,
//! auxiliary.
//!
//! # The hand calculation (the architect's; nobody else changes a number)
//!
//! Checked by the coordinating architect on Nov 13, 2026, before any code.
//!
//! ## Common inputs
//!
//! The frozen step 1 figures and the placeholder classes
//! (`data/reference/step1`). A cruiser C has 8 guns at 3 a minute, gun damage
//! 20, 8 tubes, torpedo damage 200, toughness 400. A destroyer D has 4 guns
//! at 6 a minute, gun damage 5, 8 tubes, torpedo damage 200, toughness 100. A
//! cargo ship T (kind transport) has no guns and no tubes, toughness 300. A
//! round is 5 minutes.
//!
//! Every case is one fight by day with both forces at battle readiness, so
//! each sights in round 1 at chance 1 and nobody is surprised. Nothing burns
//! in round 1. Experience and the shares of a force not ready are off on the
//! frozen figures.
//!
//! So every gun salvo of a C or a D is 120 shots (8 x 3 x 5 and 4 x 6 x 5)
//! x 0.05 = 6 expected hits, and every torpedo salvo is 8 shots x 0.02 = 0.16
//! expected, fired once, in round 1.
//!
//! Points a hit, 100 x damage / toughness:
//!
//! | Weapon | On C | On D | On T |
//! | --- | --- | --- | --- |
//! | C's gun | 5 | 20 | 6.666666 |
//! | D's gun | 1.25 | 5 | 1.666666 |
//! | A torpedo | 50 | 200 | 66.666666 |
//!
//! Ships are numbered in ID order, force X first. p is a ship's place among
//! the ships of its own kind in its force and i its place in its force, both
//! from 0. Unless a case says otherwise only round 1 is asserted, and each
//! force's orders keep it in the fight (a break-off level of 1).
//!
//! ## Case 1: guns with the firer's own kind present
//!
//! X: C1, C2, D3, D4. Y: C5, D6, D7, D8.
//!
//! | Firer | Guns at | Why | Torpedoes at | Why |
//! | --- | --- | --- | --- | --- |
//! | C1 | C5 | Y's cruisers [C5], p 0 | C5 | Y's most important kind, cruisers [C5], i 0 |
//! | C2 | C5 | p 1, modulo 1 | C5 | i 1 |
//! | D3 | D6 | Y's destroyers [D6, D7, D8], p 0 | C5 | i 2 |
//! | D4 | D7 | p 1 | C5 | i 3 |
//! | C5 | C1 | X's cruisers [C1, C2], p 0 | C1 | X's cruisers [C1, C2], i 0 |
//! | D6 | D3 | X's destroyers [D3, D4], p 0 | C2 | i 1 |
//! | D7 | D4 | p 1 | C1 | i 2 |
//! | D8 | D3 | p 2, modulo 2 | C2 | i 3 |
//!
//! ## Case 2: guns with no own kind, and a more important kind afloat
//!
//! X: D1, D2, D3. Y: C4, C5, T6, T7.
//!
//! - X's guns: Y has no destroyer; the nearest more important kind afloat is
//!   cruiser [C4, C5]. By p 0, 1, 2: D1 at C4, D2 at C5, D3 at C4.
//! - X's torpedoes at the most important kind, cruisers, by i: C4, C5, C4.
//! - Y's guns: C4 and C5 have no cruiser in X and nothing more important, so
//!   they fire at the most important kind afloat, destroyers [D1, D2, D3]:
//!   C4 (p 0) at D1, C5 (p 1) at D2.
//! - Y's torpedoes at X's destroyers by i 0 and 1: C4 at D1, C5 at D2.
//! - T6 and T7 fire nothing, and nobody fires at a cargo ship.
//!
//! ## Case 3: guns with no own kind and nothing more important
//!
//! X: C1, C2. Y: D3, D4, T5, T6.
//!
//! - X's guns: Y has no cruiser and nothing more important, so the most
//!   important kind afloat, destroyers [D3, D4]: C1 at D3, C2 at D4. Not the
//!   cargo ships.
//! - X's torpedoes the same: C1 at D3, C2 at D4.
//! - Y's guns: D3 (p 0) and D4 (p 1) have no destroyer in X; the nearest more
//!   important kind is cruiser: D3 at C1, D4 at C2.
//! - Y's torpedoes by i: D3 at C1, D4 at C2.
//!
//! ## Case 4: torpedoes at the most important kind while the guns go elsewhere
//!
//! X: D1, D2. Y: C3, D4, D5.
//!
//! - X's guns at Y's destroyers [D4, D5]: D1 at D4, D2 at D5. 6 expected
//!   each, 5 points a hit.
//! - X's torpedoes at Y's cruisers [C3]: both at C3. 0.16 expected each, 50
//!   points a hit.
//! - So D1 and D2 each have a gun salvo and a torpedo salvo at different
//!   ships in round 1, the gun salvo recorded first.
//! - Y's guns: C3 has no cruiser in X and nothing more important: destroyers
//!   [D1, D2], p 0: D1. D4 (p 0) at D1, D5 (p 1) at D2.
//! - Y's torpedoes at X's most important kind, destroyers, by i 0, 1, 2: C3
//!   at D1, D4 at D2, D5 at D1.
//!
//! ## Case 5: more firers than targets, and fewer
//!
//! X: C1, C2, C3, C4, C5. Y: C6, C7.
//!
//! - X's guns and torpedoes (one kind, so p and i are equal): C6, C7, C6, C7,
//!   C6.
//! - Y's: C6 at C1, C7 at C2. C3, C4 and C5 are not fired on.
//!
//! This is the pairing the rule gave before decision 0078, exactly.
//!
//! ## Case 6: a target sunk in the round, and the places counted again
//!
//! This case alone alters the figures and a class, so that it is determined:
//! `gun_hit_chance_day` is 1, `system_hit_chance_per_point` is 0 (so no hit
//! knocks a system down and C1's second salvo is whole), and C1 is of a
//! cruiser class of the test's own with toughness 4000, the placeholder
//! cruiser otherwise. Both forces' break-off level is 1, so that Y, with one
//! ship of two lost, fights on.
//!
//! X: C1, T2. Y: D3, D4.
//!
//! Round 1.
//! - C1's guns: Y has no cruiser and nothing more important: destroyers [D3,
//!   D4], p 0: D3. 120 shots, 120 hits, 100 x 20 / 100 = 20 points each: D3
//!   is sunk at the round's end.
//! - C1's torpedoes at the most important kind afloat, destroyers, i 0: D3.
//! - T2 fires nothing.
//! - Y's guns: D3 (p 0) and D4 (p 1) have no destroyer in X; the nearest more
//!   important kind is cruiser [C1]: both at C1. 120 hits each, 100 x 5 /
//!   4000 = 0.125 points a hit: 240 x 0.125 = 30 points in all. C1 lives.
//! - Y's torpedoes at X's most important kind, cruisers [C1]: both at C1.
//! - D3, sunk in round 1, fired in round 1.
//!
//! Round 2. Y afloat is [D4].
//! - C1's guns at Y's destroyers [D4]: D4. 120 shots.
//! - D4, now p 0, at C1. 120 shots.
//! - No torpedo salvo: both sides fired theirs in round 1.
//!
//! ## Case 7: an unarmed ship counting for its place
//!
//! X: T1, C2, C3. Y: C4, C5, C6.
//!
//! - X's guns go by place among the firer's own kind, so the cargo ship does
//!   not shift them: C2 (p 0) at C4, C3 (p 1) at C5.
//! - X's torpedoes go by place in the force, where T1 holds place 0: C2 (i 1)
//!   at C5, C3 (i 2) at C6.
//! - Y's guns: C4, C5, C6 (p 0, 1, 2) at X's cruisers [C2, C3]: C2, C3, C2.
//! - Y's torpedoes at X's most important kind afloat, cruisers [C2, C3], by
//!   i 0, 1, 2: C2, C3, C2.
//! - The cargo ship is not fired on while a cruiser is afloat.
//!
//! ## Case 8: mixed kinds against one ship, and the order of the salvos
//!
//! X: C1, D2. Y: C3.
//!
//! - X's guns: C1 at Y's cruisers [C3]: C3. D2 has no destroyer in Y; the
//!   nearest more important kind is cruiser: C3.
//! - X's torpedoes at the most important kind, cruisers: both at C3.
//! - Y's guns: C3 (p 0) at X's cruisers [C1]: C1. Her torpedoes at X's most
//!   important kind, cruisers: C1. D2 is not fired on.
//!
//! The rule before decision 0078 gives the same targets (C1 and D2 at C3; C3
//! at C1). So the event log of this fight is the log the same scenario gave
//! before the change, byte for byte: the salvos in the order C1 gun, C1
//! torpedo, D2 gun, D2 torpedo, C3 gun, C3 torpedo, on the same streams. The
//! log and the hash were captured on main before the change and are held in
//! the test.
//!
//! ## For every case
//!
//! The test reads each `Salvo` event's firer, target, weapon, shots, chance
//! and expected hits, and the points of each hit from the damage events, and
//! asserts that no salvo exists that is not listed above.

mod common;

use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::ShipId;
use sim_core::time::Pulse;
use sim_data::{LoadOptions, Scenario};

// The placeholder classes of the frozen figures, and the test's own class 7
// for case 6.
const CRUISER: u32 = 1;
const DESTROYER: u32 = 2;
const CARGO: u32 = 3;
const TOUGH_CRUISER: u32 = 7;

const SEED: u64 = 1942;

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn n(value: i64) -> Fixed {
    Fixed::from_int(value)
}

// ---------------------------------------------------------------------
// The fight, built from text. Force X (task force 1, Japanese) lies stopped
// in (0,0). Force Y (task force 2, Allied) starts east of it and steams for
// (0,0) at cruise speed, so that the two meet in the morning pulse, by day.
// Both are at battle readiness and engage with a break-off level of 1.
// Ships are numbered in order, force X first.
// ---------------------------------------------------------------------

fn fuel_of(class: u32) -> u32 {
    match class {
        CRUISER | TOUGH_CRUISER => 2000,
        DESTROYER => 500,
        CARGO => 1000,
        other => panic!("no class {other} in this test"),
    }
}

fn name_of(class: u32) -> &'static str {
    match class {
        CRUISER | TOUGH_CRUISER => "cruiser",
        DESTROYER => "destroyer",
        CARGO => "cargo ship",
        other => panic!("no class {other} in this test"),
    }
}

/// How far east of (0,0) force Y starts, in hexes. A force enters a hex 20
/// nm before its centre and a hex is 40 nm, so force Y enters (0,0) after
/// 40 x hexes - 20 nm: 220 nm from 6 hexes at 16 kn is 13.75 hours, and 140
/// nm from 4 hexes at 10 kn (a cargo ship in the force) is 14 hours, both
/// after the 12 hours of the night pulse.
fn start_hexes(y: &[u32]) -> i32 {
    if y.contains(&CARGO) {
        4
    } else {
        6
    }
}

/// The scenario text of a fight between force X and force Y, each a list of
/// ship classes.
fn scenario_text(x: &[u32], y: &[u32]) -> String {
    let mut text = format!(
        "name = \"Scenario 17a: which enemy ship a ship fires at\"\n\
         placeholder = true\n\
         seed = {SEED}\n\
         days = 1\n\
         map = \"../map/placeholder_night_action.toml\"\n\
         ship_classes = \"../reference/ship_classes.csv\"\n\
         port_sizes = \"../reference/port_sizes.csv\"\n\
         aircraft_types = \"../reference/aircraft_types.csv\"\n\
         search = \"../reference/search.toml\"\n\
         surface_combat = \"../reference/surface_combat.toml\"\n\n"
    );
    text.push_str(
        "[[task_force]]\nid = 1\nname = \"Placeholder force X\"\nside = \"japanese\"\nq = 0\nr = 0\nmission = \"surface_combat\"\nreadiness = \"battle\"\n\n",
    );
    text.push_str(&format!(
        "[[task_force]]\nid = 2\nname = \"Placeholder force Y\"\nside = \"allied\"\nq = {}\nr = 0\nmission = \"surface_combat\"\nreadiness = \"battle\"\n\n",
        start_hexes(y)
    ));
    let mut id = 0;
    for (force, side, classes) in [(1, "japanese", x), (2, "allied", y)] {
        for class in classes {
            id += 1;
            text.push_str(&format!(
                "[[ship]]\nid = {id}\nname = \"Placeholder {} {id}\"\nclass = {class}\nside = \"{side}\"\nfuel_t = {}\ntask_force = {force}\n\n",
                name_of(*class),
                fuel_of(*class)
            ));
        }
    }
    text.push_str(
        "[[order]]\nday = 1\nside = \"japanese\"\ntask_force = 1\nkind = \"set_rules_of_engagement\"\nstance = \"engage\"\nbreak_off = \"1\"\n\n",
    );
    text.push_str(
        "[[order]]\nday = 1\nside = \"allied\"\ntask_force = 2\nkind = \"set_route\"\nwaypoints = [[0, 0]]\n\n",
    );
    text.push_str(
        "[[order]]\nday = 1\nside = \"allied\"\ntask_force = 2\nkind = \"set_speed\"\nspeed = \"cruise\"\n\n",
    );
    text.push_str(
        "[[order]]\nday = 1\nside = \"allied\"\ntask_force = 2\nkind = \"set_rules_of_engagement\"\nstance = \"engage\"\nbreak_off = \"1\"\n",
    );
    text
}

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/17a_target_choice.toml")
}

struct Run {
    events: Vec<Event>,
    hash: u64,
}

fn run(text: &str, options: &LoadOptions) -> Run {
    let mut scenario = Scenario::from_text_with(text, &scenario_path(), options)
        .expect("the scenario loads on the frozen figures");
    let summaries = scenario.run_to_end().expect("the run resolves");
    assert_eq!(summaries.len(), 1, "one day");
    Run {
        events: scenario.world.events().iter().cloned().collect(),
        hash: summaries[0].state_hash,
    }
}

// ---------------------------------------------------------------------
// What the hand calculation says, and the check of a run against it.
// ---------------------------------------------------------------------

/// One salvo the hand calculation lists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Spec {
    firer: u32,
    target: u32,
    weapon: Weapon,
}

fn gun(firer: u32, target: u32) -> Spec {
    Spec {
        firer,
        target,
        weapon: Weapon::Gun,
    }
}

fn torpedo(firer: u32, target: u32) -> Spec {
    Spec {
        firer,
        target,
        weapon: Weapon::Torpedo,
    }
}

/// A round's salvos from rows of (firer, gun target, torpedo target): each
/// ship's gun salvo, then its torpedo salvo, in ship order.
fn fire(rows: &[(u32, u32, u32)]) -> Vec<Spec> {
    rows.iter()
        .flat_map(|&(firer, guns, torpedoes)| [gun(firer, guns), torpedo(firer, torpedoes)])
        .collect()
}

/// The figures a case reads: the chance of one gun shot hit and the
/// expected hits of a gun salvo (120 shots). Torpedo salvos are 8 shots at
/// 0.02, 0.16 expected, in every case.
#[derive(Clone, Copy)]
struct Figures {
    gun_chance: Fixed,
    gun_expected: Fixed,
}

/// The frozen figures: 0.05, so 6 expected.
fn day_figures() -> Figures {
    Figures {
        gun_chance: fx("0.05"),
        gun_expected: n(6),
    }
}

/// Points a hit, from the hand calculation's table (100 x damage /
/// toughness): the class of the firer, of the target, and the weapon.
fn points_a_hit(weapon: Weapon, firer: u32, target: u32) -> Fixed {
    match (weapon, firer, target) {
        (Weapon::Gun, CRUISER, CRUISER) => n(5),
        (Weapon::Gun, CRUISER | TOUGH_CRUISER, DESTROYER) => n(20),
        (Weapon::Gun, DESTROYER, CRUISER) => fx("1.25"),
        (Weapon::Gun, DESTROYER, DESTROYER) => n(5),
        (Weapon::Gun, DESTROYER, TOUGH_CRUISER) => fx("0.125"),
        (Weapon::Torpedo, _, CRUISER) => n(50),
        (Weapon::Torpedo, _, DESTROYER) => n(200),
        (Weapon::Torpedo, _, TOUGH_CRUISER) => n(5),
        other => panic!("the hand calculation has no points for {other:?}"),
    }
}

/// The events of the first fight of the run: from its `EngagementBegun` to
/// the next one, or to the end of the log.
fn first_fight(events: &[Event]) -> &[Event] {
    let start = events
        .iter()
        .position(|e| matches!(e.kind, EventKind::EngagementBegun { .. }))
        .expect("a fight begins");
    let end = events[start + 1..]
        .iter()
        .position(|e| matches!(e.kind, EventKind::EngagementBegun { .. }))
        .map_or(events.len(), |at| start + 1 + at);
    &events[start..end]
}

/// Checks the first fight of the run against the hand calculation: it is by
/// day, in (0,0), and each round in `rounds` (round 1 first) has exactly the
/// salvos listed, in order, each with its shots, chance and expected hits,
/// its damage events naming the target and the points a hit the table gives.
/// `classes` are the classes of ships 1, 2, ... in order.
fn check_rounds(run: &Run, classes: &[u32], figures: Figures, rounds: &[Vec<Spec>]) {
    let fight = first_fight(&run.events);
    match &fight[0].kind {
        EventKind::EngagementBegun { hex, .. } => assert_eq!(*hex, Hex::new(0, 0)),
        _ => unreachable!(),
    }
    assert_eq!(fight[0].pulse, Some(Pulse::Morning), "a fight by day");

    let class_of = |ship: ShipId| classes[usize::try_from(ship.raw()).expect("an index") - 1];
    for (index, expected) in rounds.iter().enumerate() {
        let round = u32::try_from(index + 1).expect("a round number");
        let salvos: Vec<&Event> = fight
            .iter()
            .filter(|e| matches!(e.kind, EventKind::Salvo { round: r, .. } if r == round))
            .collect();
        let seen: Vec<Spec> = salvos
            .iter()
            .map(|e| match &e.kind {
                EventKind::Salvo {
                    firer,
                    target,
                    weapon,
                    ..
                } => Spec {
                    firer: firer.raw(),
                    target: target.raw(),
                    weapon: *weapon,
                },
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(&seen, expected, "the salvos of round {round}");

        for event in salvos {
            let EventKind::Salvo {
                firer,
                target,
                weapon,
                shots,
                chance,
                expected: expected_hits,
                hits,
                ..
            } = &event.kind
            else {
                unreachable!()
            };
            let what = format!(
                "{weapon:?} salvo of ship {} at ship {}",
                firer.raw(),
                target.raw()
            );
            match weapon {
                Weapon::Gun => {
                    assert_eq!(*shots, n(120), "{what}: shots");
                    assert_eq!(*chance, figures.gun_chance, "{what}: chance");
                    assert_eq!(*expected_hits, figures.gun_expected, "{what}: expected");
                }
                Weapon::Torpedo => {
                    assert_eq!(*shots, n(8), "{what}: shots");
                    assert_eq!(*chance, fx("0.02"), "{what}: chance");
                    assert_eq!(*expected_hits, fx("0.16"), "{what}: expected");
                }
                Weapon::Bomb => panic!("a surface fight fires no bombs"),
            }
            // The damage this salvo did: events that cite it.
            let per_hit = points_a_hit(*weapon, class_of(*firer), class_of(*target));
            let mut landed = 0;
            for damage in fight
                .iter()
                .filter(|e| e.causes.contains(&Cause::Event(event.id)))
            {
                if let EventKind::ShipDamaged {
                    ship, hits, points, ..
                } = &damage.kind
                {
                    assert_eq!(ship, target, "{what}: the ship damaged");
                    assert_eq!(*points, n(i64::from(*hits)) * per_hit, "{what}: points");
                    landed += hits;
                }
            }
            assert_eq!(landed, *hits, "{what}: the hits that landed");
        }
    }
}

// ---------------------------------------------------------------------
// The cases.
// ---------------------------------------------------------------------

fn run_frozen(x: &[u32], y: &[u32]) -> (Run, Vec<u32>) {
    let run = run(&scenario_text(x, y), &common::frozen());
    (run, x.iter().chain(y).copied().collect())
}

#[test]
fn case_1_guns_with_the_firers_own_kind_present() {
    let (x, y) = (
        [CRUISER, CRUISER, DESTROYER, DESTROYER],
        [CRUISER, DESTROYER, DESTROYER, DESTROYER],
    );
    let (run, classes) = run_frozen(&x, &y);
    // Ships: X C1 C2 D3 D4, Y C5 D6 D7 D8.
    let round_1 = fire(&[
        (1, 5, 5),
        (2, 5, 5),
        (3, 6, 5),
        (4, 7, 5),
        (5, 1, 1),
        (6, 3, 2),
        (7, 4, 1),
        (8, 3, 2),
    ]);
    check_rounds(&run, &classes, day_figures(), &[round_1]);
}

#[test]
fn case_2_guns_with_no_own_kind_and_a_more_important_kind_afloat() {
    let (x, y) = (
        [DESTROYER, DESTROYER, DESTROYER],
        [CRUISER, CRUISER, CARGO, CARGO],
    );
    let (run, classes) = run_frozen(&x, &y);
    // Ships: X D1 D2 D3, Y C4 C5 T6 T7. The cargo ships fire nothing.
    let round_1 = fire(&[(1, 4, 4), (2, 5, 5), (3, 4, 4), (4, 1, 1), (5, 2, 2)]);
    check_rounds(&run, &classes, day_figures(), &[round_1]);
}

#[test]
fn case_3_guns_with_no_own_kind_and_nothing_more_important() {
    let (x, y) = ([CRUISER, CRUISER], [DESTROYER, DESTROYER, CARGO, CARGO]);
    let (run, classes) = run_frozen(&x, &y);
    // Ships: X C1 C2, Y D3 D4 T5 T6.
    let round_1 = fire(&[(1, 3, 3), (2, 4, 4), (3, 1, 1), (4, 2, 2)]);
    check_rounds(&run, &classes, day_figures(), &[round_1]);
}

#[test]
fn case_4_torpedoes_at_the_most_important_kind_while_the_guns_go_elsewhere() {
    let (x, y) = ([DESTROYER, DESTROYER], [CRUISER, DESTROYER, DESTROYER]);
    let (run, classes) = run_frozen(&x, &y);
    // Ships: X D1 D2, Y C3 D4 D5. D1 and D2 fire guns at Y's destroyers and
    // torpedoes at Y's cruiser.
    let round_1 = fire(&[(1, 4, 3), (2, 5, 3), (3, 1, 1), (4, 1, 2), (5, 2, 1)]);
    check_rounds(&run, &classes, day_figures(), &[round_1]);
}

#[test]
fn case_5_more_firers_than_targets_and_fewer() {
    let (x, y) = (
        [CRUISER, CRUISER, CRUISER, CRUISER, CRUISER],
        [CRUISER, CRUISER],
    );
    let (run, classes) = run_frozen(&x, &y);
    // Ships: X C1 to C5, Y C6 C7. C3, C4 and C5 are not fired on.
    let round_1 = fire(&[
        (1, 6, 6),
        (2, 7, 7),
        (3, 6, 6),
        (4, 7, 7),
        (5, 6, 6),
        (6, 1, 1),
        (7, 2, 2),
    ]);
    check_rounds(&run, &classes, day_figures(), &[round_1]);
}

#[test]
fn case_6_a_target_sunk_in_the_round_and_the_places_counted_again() {
    // This case alone alters the figures and a class: a gun shot hits at
    // chance 1 by day, a hit knocks no system down, and C1 is of a cruiser
    // class of the test's own with toughness 4000.
    let options = common::altered_reference("target_choice_case_6", |dir| {
        common::edit_file_by(
            dir,
            "surface_combat.toml",
            &[
                (
                    "gun_hit_chance_day = \"0.05\"",
                    "gun_hit_chance_day = \"1\"",
                ),
                (
                    "system_hit_chance_per_point = \"0.06\"",
                    "system_hit_chance_per_point = \"0\"",
                ),
            ],
        );
        common::edit_file(dir, "ship_classes.csv", |mut text| {
            if !text.ends_with('\n') {
                text.push('\n');
            }
            text.push_str(
                "7,Placeholder tough cruiser,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,4000,0,0,0\n",
            );
            text
        });
    });
    let (x, y) = ([TOUGH_CRUISER, CARGO], [DESTROYER, DESTROYER]);
    let run = run(&scenario_text(&x, &y), &options);
    let classes: Vec<u32> = x.iter().chain(&y).copied().collect();
    let figures = Figures {
        gun_chance: n(1),
        gun_expected: n(120),
    };
    // Ships: X C1 T2, Y D3 D4.
    let round_1 = fire(&[(1, 3, 3), (3, 1, 1), (4, 1, 1)]);
    // Round 2: D3 is sunk and the places are counted again. Both fired their
    // torpedoes in round 1.
    let round_2 = vec![gun(1, 4), gun(4, 1)];
    check_rounds(&run, &classes, figures, &[round_1, round_2]);

    // D3 is sunk at the end of round 1, and nothing else is sunk in it.
    let fight = first_fight(&run.events);
    let sunk_in_round_1: Vec<u32> = fight
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::ShipSunk { round: 1, ship, .. } => Some(ship.raw()),
            _ => None,
        })
        .collect();
    assert_eq!(sunk_in_round_1, vec![3]);
}

#[test]
fn case_7_an_unarmed_ship_counting_for_its_place() {
    let (x, y) = ([CARGO, CRUISER, CRUISER], [CRUISER, CRUISER, CRUISER]);
    let (run, classes) = run_frozen(&x, &y);
    // Ships: X T1 C2 C3, Y C4 C5 C6. The cargo ship is not fired on.
    let round_1 = fire(&[(2, 4, 5), (3, 5, 6), (4, 2, 2), (5, 3, 3), (6, 2, 2)]);
    check_rounds(&run, &classes, day_figures(), &[round_1]);
}

/// The event log of case 8 as `sim-cli run --events` prints it, captured on
/// main before decision 0078 was built, and its hash. The hash was
/// 0x7baddf6faffed244 until the state went to version 7 (decision 0083, a
/// ship's flag for the side of her torpedo tubes fired), and 0x8339eb579d472d65
/// until the state went to version 8 (decision 0086, the counters of units
/// made in play); the log was unchanged. Remade Nov 13, 2026: battle
/// readiness is no longer alerted by a contact (differences.md row 2), so the
/// two alert events are gone from the log, the events after them are
/// numbered two lower, and the hash is 0xc1809065942fe453 no longer. The hash
/// was 0x69d03b89e5b56452 until the state went to version 9 (decision 0069,
/// ship repair) and five figures were added to surface_combat.toml; the log
/// was unchanged. It was 0x8b7cd91a2f608ed5 until the state went to version
/// 10 (decision 0066, land units); the log was unchanged again. It was
/// 0x8411a9fd58a3680b until the state went to version 11 (decision 0091,
/// rebasing); the log was unchanged again. It was 0x3f24661c88fb42de until
/// the state went to version 12 (decision 0090, the fight ashore); the log
/// was unchanged again. It was 0x6c4ecebc0c9f0fbd until the state went to
/// version 13 (decision 0092, shells and the enemy ashore); the log was
/// unchanged again. It was 0x6db87308007b7ab8 until the state went to version
/// 14 (decision 0094, marching overland); the log was unchanged again.
const CASE_8_LOG: &str = include_str!("golden/17a_case_8_events.txt");
const CASE_8_HASH: u64 = 0x37c66440a1926f4b;

fn log_text(events: &[Event]) -> String {
    let mut text = String::new();
    for event in events {
        text.push_str(&event.to_string());
        text.push('\n');
    }
    text
}

#[test]
fn case_8_mixed_kinds_against_one_ship_and_the_order_of_the_salvos() {
    let (x, y) = ([CRUISER, DESTROYER], [CRUISER]);
    let (run, classes) = run_frozen(&x, &y);
    // Ships: X C1 D2, Y C3.
    let round_1 = fire(&[(1, 3, 3), (2, 3, 3), (3, 1, 1)]);
    check_rounds(&run, &classes, day_figures(), &[round_1]);

    // The rule before decision 0078 gives these targets, so the log is the
    // log it gave, byte for byte, and so is the hash.
    assert_eq!(log_text(&run.events), CASE_8_LOG);
    assert_eq!(run.hash, CASE_8_HASH);
}
