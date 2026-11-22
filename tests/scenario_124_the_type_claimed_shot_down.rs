//! Scenario 124: the type of the aircraft claimed shot down.
//!
//! Proves decision 0210 against a hand calculation. It runs on the frozen
//! step 1 figures with chances altered, as scenarios 11 and 98 do, on
//! `06a_base_raid.toml`, which is placeholder data.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Inputs
//!
//! The base raid of scenario 11's hand calculation (see it): on three
//! mornings Japanese group 1 (bombers, aircraft type 3) and group 2
//! (escorts, type 4) raid base 2; Allied group 3 (type 4) patrols over it
//! and is the only group on patrol there; the base's guns fire. The
//! firings, from scenario 11:
//!
//! | Day | Firing | The pool fired on | Really shot down | Missed |
//! | --- | ------ | ----------------- | ---------------- | ------ |
//! | 1 | escorts at the patrol | group 3, type 4 | 3 | 3 |
//! | 1 | patrol at the escorts | group 2, type 4 | 1 | 5 |
//! | 1 | patrol at the bombers | group 1, type 3 | 2 | 4 |
//! | 1 | base guns | group 1, type 3 | 0 | 20 |
//! | 2 | escorts at the patrol | group 3, type 4 | 2 | 4 |
//! | 2 | patrol at the escorts | group 2, type 4 | 1 | 5 |
//! | 2 | patrol at the bombers | group 1, type 3 | 1 | 2 |
//! | 2 | base guns | group 1, type 3 | 0 | 20 |
//! | 3 | escorts at the patrol | group 3, type 4 | 0 | 5 |
//! | 3 | patrol at the escorts | group 2, type 4 | 1 | 4 |
//! | 3 | patrol at the bombers | group 1, type 3 | 1 | 1 |
//! | 3 | base guns | group 1, type 3 | 3 | 17 |
//!
//! "Made out" is `raid_recognised_chance`; "false" is the three false
//! claim chances of scenario 11, all at the one setting. At false 0
//! nothing false is claimed; at false 1 every miss is.
//!
//! ## The Japanese claim's types in the air, each day
//!
//! Every aircraft it claims shot down is of the patrol, type 4.
//!
//! | Day | False 0, made out 1 | False 1, made out 1 | Made out 0, either |
//! | --- | ------------------- | ------------------- | ------------------ |
//! | 1 | type 4: 3 | type 4: 6 | no type |
//! | 2 | type 4: 2 | type 4: 6 | no type |
//! | 3 | none claimed, no type | type 4: 5 | no type |
//!
//! Its types on the ground are those of scenario 98, unchanged, and the
//! one draw decides both lists: on a day it names types in the air it
//! names its types on the ground, and on a day it names neither.
//!
//! ## The Allied claim's types in the air, each day
//!
//! The patrol's fire at the escorts names type 4; its fire at the bombers
//! and the guns' fire name type 3.
//!
//! | Day | False 0, made out 1 | False 1, made out 1 | Made out 0, either |
//! | --- | ------------------- | ------------------- | ------------------ |
//! | 1 | type 3: 2, type 4: 1 | type 3: 26, type 4: 6 | no type |
//! | 2 | type 3: 1, type 4: 1 | type 3: 23, type 4: 6 | no type |
//! | 3 | type 3: 4, type 4: 1 | type 3: 22, type 4: 5 | no type |
//!
//! Day 1 at false 1: at the bombers 2 and 4, and the guns 0 and 20, are
//! 26 of type 3; at the escorts 1 and 5 are 6 of type 4; 32 in all, the
//! claim's number. Day 2: 1 and 2 and 20 are 23; 1 and 5 are 6; 29. Day
//! 3: 1 and 1, and 3 and 17, are 22; 1 and 4 are 5; 27.
//!
//! ## The words
//!
//! Day 2, false 1 and made out 1. The Japanese claim: "6 aircraft shot
//! down (6 of aircraft_type:4), 15 aircraft destroyed on the ground (15
//! of aircraft_type:3)". The Allied claim: "29 aircraft shot down (23 of
//! aircraft_type:3, 6 of aircraft_type:4)". At made out 0 both are the
//! words of scenario 11's run, as scenario 98 has them.
//!
//! ## The type is that of the group fired on
//!
//! The same three mornings with group 2, the escorts, given a fighter
//! type of its own, type 5, a copy of type 4 made for the test. The fight
//! is the same, shot for shot, which is asserted. Day 1, false 1, made
//! out 1: the Allied claim names type 3: 26 and type 5: 6, and no type
//! 4. The Japanese claim still names type 4: 6, the patrol's. (Corrected
//! by the architect on Nov 22, 2026: as first written the escorts were
//! given type 2, which in the frozen figures is a scout that does not
//! escort, so the fight was not the same.)
//!
//! ## Guns name no escort
//!
//! The same three mornings with group 3 taken out of the scenario, so
//! that nothing patrols over base 2: the escorts fire at nothing and
//! nothing fires at them. At every setting, on every day, no Allied claim
//! names type 4, and every type an Allied claim names is type 3; no
//! Japanese claim names a type in the air, since it claims nothing shot
//! down.
//!
//! ## Firing by firing
//!
//! The function is decision 0179's, used once for each firing; scenario
//! 98's table of it stands. A claim's list is the sum of its firings'
//! lists, type by type:
//!
//! | Firing | Really shot down | False | The pool | Its list |
//! | --- | --- | --- | --- | --- |
//! | first | type 3: 2 | 3 | type 2: 6, type 3: 4 | type 2: 2, type 3: 3 |
//! | second | none | 4 | type 4: 5 | type 4: 4 |
//! | the claim | | | | type 2: 2, type 3: 3, type 4: 4 |
//!
//! # Properties
//!
//! - **A type named was fired on.** Over 200 seeds of the base raid at
//!   the placeholder false chances and made out 0.75: every type in a
//!   claim's list in the air is the type of a group in a pool one of the
//!   claimant's firings of that raid was at, read from the truth; each
//!   list is empty or adds to the aircraft claimed shot down.
//! - **One draw a claim, and the old draw is the old draw.** Over those
//!   seeds, with the chance at 0.75: the fights and every claim's numbers
//!   are those at made out 0, and each Japanese claim's types on the
//!   ground are what the code gave before this change at the same seeds,
//!   which the build records from the commit before and pins.
//! - **Both lists or neither.** Over those seeds, a claim that names
//!   types on the ground and claims aircraft shot down names types in the
//!   air, and one that names none on the ground where it could names none
//!   in the air.
//! - **A surface fight's claim and a bombardment's claim name no type.**
//! - **The other side is told nothing:** neither side's events hold the
//!   other's claim, with types or without.
//! - **A claim passed between commands keeps its types, and a picture
//!   keeps them.**
//! - **The hash names the field,** and two worlds that differ only in a
//!   claim's types in the air hash differently.
//! - **The frozen figures are as they were:** no figures file is changed,
//!   on them no claim names a type, and no breakdown of a claim is other
//!   than it was.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]

mod common;

use std::fs;
use std::path::PathBuf;

use sim_core::belief::BattleClaim;
use sim_core::claims::{air_claim_types, FiringTypes};
use sim_core::events::{Event, EventKind};
use sim_core::ids::{AirGroupId, AircraftTypeId, EntityRef};
use sim_core::side_events::SideEventKind;
use sim_core::state::Side;
use sim_core::World;
use sim_data::Scenario;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const RAID: &str = "06a_base_raid.toml";

/// The three false claim chances of scenario 11 at one setting.
const FALSE_KEYS: [&str; 3] = [
    "fighter_false_claim_chance",
    "anti_aircraft_false_claim_chance",
    "bomb_false_claim_chance",
];
const PLACEHOLDER: [&str; 3] = ["0.1", "0.05", "0.1"];
const NONE: [&str; 3] = ["0", "0", "0"];
const ALL: [&str; 3] = ["1", "1", "1"];

/// Group 2, the escorts, given aircraft type 2 in the scenario's text.
const ESCORTS_OF_TYPE_2: (&str, &str) = (
    "name = \"Placeholder escort group\"\nside = \"japanese\"\nbase = 1\naircraft_type = 4\n",
    "name = \"Placeholder escort group\"\nside = \"japanese\"\nbase = 1\naircraft_type = 2\n",
);

/// Group 3, the patrol, taken out of the scenario: its group and its order.
const NO_PATROL: [(&str, &str); 2] = [
    (
        "[[air_group]]\nid = 3\nname = \"Placeholder defending fighters\"\nside = \"allied\"\n\
         base = 2\naircraft_type = 4\nready = 12\n",
        "",
    ),
    (
        "[[order]]\nday = 1\nside = \"allied\"\nair_group = 3\nkind = \"set_air_mission\"\n\
         mission = \"combat_air_patrol\"\n",
        "",
    ),
];

fn set(lines: &mut [String], key: &str, value: &str) {
    let mut found = 0;
    for line in lines.iter_mut() {
        if line.starts_with(&format!("{key} = ")) {
            *line = format!("{key} = \"{value}\"");
            found += 1;
        }
    }
    assert_eq!(found, 1, "{key} is set once in the file");
}

/// The base raid on the frozen figures with the false chances and the
/// chance of making out a type set, the text edited.
fn variant(chances: [&str; 3], made_out: &str, edits: &[(&str, &str)]) -> Scenario {
    variant_with(chances, made_out, edits, None)
}

/// As [`variant`], with a line added to the test's own copy of the
/// aircraft types.
fn variant_with(
    chances: [&str; 3],
    made_out: &str,
    edits: &[(&str, &str)],
    extra_type: Option<&str>,
) -> Scenario {
    let made_out = made_out.to_string();
    let options = common::altered_reference("claim_types_air", |dir| {
        common::edit_file(dir, "air_combat.toml", |figures| {
            let mut lines: Vec<String> = figures.lines().map(str::to_string).collect();
            for (key, value) in FALSE_KEYS.iter().zip(chances) {
                set(&mut lines, key, value);
            }
            set(&mut lines, "raid_recognised_chance", &made_out);
            lines.join("\n")
        });
        if let Some(line) = extra_type {
            common::edit_file(dir, "aircraft_types.csv", |types| {
                format!("{}\n{line}\n", types.trim_end())
            });
        }
    });
    let path = PathBuf::from(ROOT).join("data/scenarios").join(RAID);
    let mut text = fs::read_to_string(&path).expect("the scenario reads");
    for (from, to) in edits {
        text = common::replace_once(&text, from, to);
    }
    common::from_text_with(&text, &path, &options).expect("the variant loads")
}

fn run(chances: [&str; 3], made_out: &str, edits: &[(&str, &str)]) -> World {
    let mut scenario = variant(chances, made_out, edits);
    scenario.run_to_end().expect("the run resolves");
    scenario.world
}

/// One claim from the air, from its event.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Claim {
    day: u32,
    side: Side,
    /// Sunk, hits, shot down, on the ground.
    numbers: [u32; 4],
    air: Vec<(u32, u32)>,
    ground: Vec<(u32, u32)>,
    text: String,
}

fn pairs(list: &[(AircraftTypeId, u32)]) -> Vec<(u32, u32)> {
    list.iter().map(|(t, n)| (t.raw(), *n)).collect()
}

fn types(list: &[(u32, u32)]) -> Vec<(AircraftTypeId, u32)> {
    list.iter()
        .map(|(t, n)| (AircraftTypeId::new(*t), *n))
        .collect()
}

fn air_claims(world: &World) -> Vec<Claim> {
    world
        .events()
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ClaimReceived {
                side,
                claimed_sunk,
                claimed_hits,
                claimed_shot_down,
                claimed_on_ground,
                from_air: true,
                ground_types,
                air_types,
                ..
            } => Some(Claim {
                day: event.day,
                side: *side,
                numbers: [
                    *claimed_sunk,
                    *claimed_hits,
                    *claimed_shot_down,
                    *claimed_on_ground,
                ],
                air: pairs(air_types),
                ground: pairs(ground_types),
                text: event.kind.to_string(),
            }),
            _ => None,
        })
        .collect()
}

fn claim_on(claims: &[Claim], side: Side, day: u32) -> &Claim {
    let mut found = claims.iter().filter(|c| c.side == side && c.day == day);
    let claim = found.next().expect("the side claims on the day");
    assert!(found.next().is_none(), "one claim a side a day");
    claim
}

/// The log with the claims taken out and the recognition of decision 0167
/// set aside: what does not move with the chance of making out a type.
fn fight_events(world: &World) -> Vec<Event> {
    world
        .events()
        .iter()
        .filter(|event| !matches!(event.kind, EventKind::ClaimReceived { .. }))
        .cloned()
        .map(|mut event| {
            match &mut event.kind {
                EventKind::BaseAttacked { recognised, .. }
                | EventKind::AirAttack { recognised, .. } => *recognised = None,
                _ => {}
            }
            event
        })
        .collect()
}

/// The fight, shot for shot: every fire event's shots, chance and hits, and
/// every aircraft lost, without the groups' types.
fn shots(world: &World) -> Vec<String> {
    world
        .events()
        .iter()
        .filter(|event| {
            matches!(
                event.kind,
                EventKind::AirFight { .. }
                    | EventKind::AntiAircraftFire { .. }
                    | EventKind::BaseAntiAircraftFire { .. }
                    | EventKind::BaseAttacked { .. }
                    | EventKind::AircraftLost { .. }
            )
        })
        .map(|event| {
            let mut event = event.clone();
            if let EventKind::BaseAttacked { recognised, .. } = &mut event.kind {
                *recognised = None;
            }
            format!("{} {:?}", event.day, event.kind)
        })
        .collect()
}

/// The words of a claim without the clause of decision 0180 at its end.
fn before_the_units(text: &str) -> &str {
    text.split("; claimed by ").next().expect("words")
}

// ---------------------------------------------------------------------
// The Japanese claim's types in the air, each day.
// ---------------------------------------------------------------------

/// Scenario 98's types on the ground, unchanged (its table).
fn ground_of_98(chances: [&str; 3], made_out: &str, day: u32) -> Vec<(u32, u32)> {
    if made_out == "0" {
        return Vec::new();
    }
    match (chances == ALL, day) {
        (false, 2) => vec![(3, 2)],
        (false, _) => Vec::new(),
        (true, 1) => vec![(3, 16)],
        (true, 2) => vec![(3, 15)],
        (true, _) => vec![(3, 11)],
    }
}

#[test]
fn the_japanese_claims_types_in_the_air_are_the_hand_calculation() {
    let none: Vec<(u32, u32)> = Vec::new();
    for (chances, made_out, want) in [
        (NONE, "1", [vec![(4, 3)], vec![(4, 2)], none.clone()]),
        (ALL, "1", [vec![(4, 6)], vec![(4, 6)], vec![(4, 5)]]),
        (NONE, "0", [none.clone(), none.clone(), none.clone()]),
        (ALL, "0", [none.clone(), none.clone(), none.clone()]),
    ] {
        let world = run(chances, made_out, &[]);
        let claims = air_claims(&world);
        for day in 1..=3 {
            let at = day as usize - 1;
            let claim = claim_on(&claims, Side::Japanese, day);
            assert_eq!(claim.air, want[at], "{chances:?} {made_out} day {day}");
            // Day 3 at false 0: none claimed shot down.
            if chances == NONE && day == 3 {
                assert_eq!(claim.numbers[2], 0);
            }
            // Its types on the ground are scenario 98's, unchanged, and the
            // one draw decides both lists.
            assert_eq!(
                claim.ground,
                ground_of_98(chances, made_out, day),
                "{chances:?} {made_out} day {day}"
            );
            if claim.numbers[2] > 0 && claim.numbers[3] > 0 {
                assert_eq!(claim.air.is_empty(), claim.ground.is_empty());
            }
        }
    }
}

// ---------------------------------------------------------------------
// The Allied claim's types in the air, each day.
// ---------------------------------------------------------------------

#[test]
fn the_allied_claims_types_in_the_air_are_the_hand_calculation() {
    let none: Vec<(u32, u32)> = Vec::new();
    for (chances, made_out, want) in [
        (
            NONE,
            "1",
            [
                vec![(3, 2), (4, 1)],
                vec![(3, 1), (4, 1)],
                vec![(3, 4), (4, 1)],
            ],
        ),
        (
            ALL,
            "1",
            [
                vec![(3, 26), (4, 6)],
                vec![(3, 23), (4, 6)],
                vec![(3, 22), (4, 5)],
            ],
        ),
        (NONE, "0", [none.clone(), none.clone(), none.clone()]),
        (ALL, "0", [none.clone(), none.clone(), none.clone()]),
    ] {
        let world = run(chances, made_out, &[]);
        let claims = air_claims(&world);
        for day in 1..=3 {
            let at = day as usize - 1;
            let claim = claim_on(&claims, Side::Allied, day);
            assert_eq!(claim.air, want[at], "{chances:?} {made_out} day {day}");
            assert!(claim.ground.is_empty());
        }
        if chances == ALL {
            // 32, 29 and 27 in all, the claim's number.
            for (day, all) in [(1, 32), (2, 29), (3, 27)] {
                assert_eq!(claim_on(&claims, Side::Allied, day).numbers[2], all);
            }
        }
    }
}

// ---------------------------------------------------------------------
// The words.
// ---------------------------------------------------------------------

#[test]
fn the_words_of_day_2_give_the_types_after_the_number() {
    let made = run(ALL, "1", &[]);
    let claims = air_claims(&made);
    assert_eq!(
        before_the_units(&claim_on(&claims, Side::Japanese, 2).text),
        "Japanese claim at (5,0): 6 aircraft shot down (6 of aircraft_type:4), 15 aircraft \
         destroyed on the ground (15 of aircraft_type:3)"
    );
    assert_eq!(
        before_the_units(&claim_on(&claims, Side::Allied, 2).text),
        "Allied claim over (5,0): 29 aircraft shot down (23 of aircraft_type:3, 6 of \
         aircraft_type:4)"
    );
    // At made out 0 both are the words of scenario 11's run, as scenario
    // 98 has them.
    let unmade = run(ALL, "0", &[]);
    let claims = air_claims(&unmade);
    assert_eq!(
        before_the_units(&claim_on(&claims, Side::Japanese, 2).text),
        "Japanese claim at (5,0): 6 aircraft shot down, 15 aircraft destroyed on the ground"
    );
    assert_eq!(
        before_the_units(&claim_on(&claims, Side::Allied, 2).text),
        "Allied claim over (5,0): 29 aircraft shot down"
    );
}

// ---------------------------------------------------------------------
// The type is that of the group fired on.
// ---------------------------------------------------------------------

/// The statement as corrected: the escorts are given a fighter type of
/// their own, type 5, a copy of type 4 in the test's own copy of the frozen
/// types (a placeholder). It also shows why type 2 would not do: the frozen
/// type 2 is a scout, a group of it sends no escort, and nothing fights in
/// the air.
#[test]
fn the_type_named_is_that_of_the_group_fired_on() {
    let plain = run(ALL, "1", &[]);
    let scouts = run(ALL, "1", &[ESCORTS_OF_TYPE_2]);
    assert_ne!(shots(&plain), shots(&scouts), "type 2 does not escort");
    assert!(!scouts.events().iter().any(|e| matches!(
        e.kind,
        EventKind::AirFight {
            at: sim_core::events::AirTarget::Escorts,
            ..
        } | EventKind::AirFight {
            firing: sim_core::events::AirFirer::Escorts,
            ..
        }
    )));
    for claim in air_claims(&scouts) {
        assert!(claim.air.iter().all(|(t, _)| *t == 3), "{}", claim.text);
    }

    let fighter_b = "5,Placeholder fighter B,fighter,0,0,250,none,0,0,0";
    let mut other = variant_with(
        ALL,
        "1",
        &[(
            ESCORTS_OF_TYPE_2.0,
            &ESCORTS_OF_TYPE_2
                .1
                .replace("aircraft_type = 2", "aircraft_type = 5"),
        )],
        Some(fighter_b),
    );
    other.run_to_end().expect("the run resolves");
    assert_eq!(shots(&plain), shots(&other.world), "the fight is the same");
    let claims = air_claims(&other.world);
    let allied = claim_on(&claims, Side::Allied, 1);
    assert_eq!(allied.air, vec![(3, 26), (5, 6)], "{}", allied.text);
    assert!(allied.air.iter().all(|(t, _)| *t != 4));
    assert_eq!(claim_on(&claims, Side::Japanese, 1).air, vec![(4, 6)]);
}

// ---------------------------------------------------------------------
// Guns name no escort.
// ---------------------------------------------------------------------

#[test]
fn guns_name_no_escort() {
    for chances in [NONE, ALL, PLACEHOLDER] {
        for made_out in ["0", "0.75", "1"] {
            let world = run(chances, made_out, &NO_PATROL);
            let claims = air_claims(&world);
            for claim in &claims {
                match claim.side {
                    Side::Allied => assert!(
                        claim.air.iter().all(|(t, _)| *t == 3),
                        "{chances:?} {made_out}: {}",
                        claim.text
                    ),
                    Side::Japanese => {
                        assert_eq!(claim.numbers[2], 0, "{}", claim.text);
                        assert!(claim.air.is_empty(), "{}", claim.text);
                    }
                }
            }
        }
    }
    // At made out 1 and false 1 the guns' claims do name type 3.
    let world = run(ALL, "1", &NO_PATROL);
    assert!(air_claims(&world)
        .iter()
        .any(|c| c.side == Side::Allied && !c.air.is_empty()));
}

// ---------------------------------------------------------------------
// Firing by firing.
// ---------------------------------------------------------------------

#[test]
fn firing_by_firing_is_the_table_of_the_hand_calculation() {
    let first = FiringTypes {
        real: types(&[(3, 2)]),
        false_claimed: 3,
        pool: types(&[(2, 6), (3, 4)]),
    };
    let second = FiringTypes {
        real: Vec::new(),
        false_claimed: 4,
        pool: types(&[(4, 5)]),
    };
    assert_eq!(
        air_claim_types(std::slice::from_ref(&first)),
        types(&[(2, 2), (3, 3)])
    );
    assert_eq!(
        air_claim_types(std::slice::from_ref(&second)),
        types(&[(4, 4)])
    );
    assert_eq!(
        air_claim_types(&[first, second]),
        types(&[(2, 2), (3, 3), (4, 4)])
    );
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

/// The types of the enemy's groups each side's firings of a day were at,
/// read from the truth: the groups the day's fire events name, of the side
/// that did not fire. A firing names its first target group, and the
/// escorts' fire at the patrol its first patrol group; in this scenario the
/// patrol is one group, so this is every group of every pool.
fn pools_fired_on(world: &World) -> Vec<(u32, Side, AircraftTypeId)> {
    let truth = world.umpire();
    let mut pools = Vec::new();
    let mut add = |day: u32, group: AirGroupId| {
        let g = &truth.air_groups[&group];
        // The pool's side is the side fired on; the claimant is the other.
        let claimant = match g.side {
            Side::Allied => Side::Japanese,
            Side::Japanese => Side::Allied,
        };
        pools.push((day, claimant, g.aircraft_type));
    };
    for event in world.events().iter() {
        match &event.kind {
            EventKind::AirFight { target, .. } => add(event.day, *target),
            EventKind::AntiAircraftFire { air_group, .. }
            | EventKind::BaseAntiAircraftFire { air_group, .. } => add(event.day, *air_group),
            _ => {}
        }
    }
    pools
}

/// What the commit before this change gave, recorded and pinned: each
/// Japanese claim's types on the ground over seeds 1 to 200 at made out
/// 0.75 (by seed and day), and every claim's breakdown on the frozen
/// figures (by scenario and the claim's place in its log).
/// The pins: the types on the ground by seed and day, the breakdowns by
/// scenario and claim.
type Pins = (
    std::collections::BTreeMap<(u64, u32), String>,
    std::collections::BTreeMap<(String, usize), String>,
);

fn before() -> Pins {
    let text = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("golden/scenario_124_before.txt"),
    )
    .expect("the pins read");
    let mut ground = std::collections::BTreeMap::new();
    let mut breakdowns = std::collections::BTreeMap::new();
    for line in text.lines().filter(|l| !l.starts_with('#')) {
        let mut words = line.splitn(4, ' ');
        let kind = words.next().expect("a kind");
        let a = words.next().expect("a first key").to_string();
        let b = words.next().expect("a second key").to_string();
        let rest = words.next().unwrap_or("").to_string();
        match kind {
            "ground" => {
                ground.insert(
                    (a.parse().expect("a seed"), b.parse().expect("a day")),
                    rest,
                );
            }
            "breakdown" => {
                breakdowns.insert((a, b.parse().expect("an index")), rest);
            }
            other => panic!("unknown pin {other}"),
        }
    }
    (ground, breakdowns)
}

fn ground_words(list: &[(u32, u32)]) -> String {
    list.iter()
        .map(|(t, n)| format!("{t}:{n}"))
        .collect::<Vec<_>>()
        .join(",")
}

#[test]
fn over_200_seeds_a_type_named_was_fired_on_and_one_draw_moves_nothing_else() {
    let made = variant(PLACEHOLDER, "0.75", &[]);
    let unmade = variant(PLACEHOLDER, "0", &[]);
    let (ground_before, _) = before();
    let (mut named, mut both, mut neither) = (0, 0, 0);
    for seed in 1..=200 {
        let mut a = made.with_seed(seed).expect("a seed");
        let mut b = unmade.with_seed(seed).expect("a seed");
        a.run_to_end().expect("the run resolves");
        b.run_to_end().expect("the run resolves");
        let pools = pools_fired_on(&a.world);
        for claim in air_claims(&a.world) {
            // A type named was fired on.
            for (t, _) in &claim.air {
                assert!(
                    pools.contains(&(claim.day, claim.side, AircraftTypeId::new(*t))),
                    "seed {seed}: {}",
                    claim.text
                );
            }
            if !claim.air.is_empty() {
                named += 1;
                let sum: u32 = claim.air.iter().map(|(_, n)| n).sum();
                assert_eq!(sum, claim.numbers[2], "seed {seed}: {}", claim.text);
            }
            if claim.side == Side::Japanese {
                // The old draw is the old draw.
                assert_eq!(
                    ground_before.get(&(seed, claim.day)),
                    Some(&ground_words(&claim.ground)),
                    "seed {seed} day {}",
                    claim.day
                );
                // Both lists or neither: base 2 always has group 4 on its
                // field, so a claim on the ground could name its types.
                if claim.numbers[2] > 0 && claim.numbers[3] > 0 {
                    assert_eq!(
                        claim.air.is_empty(),
                        claim.ground.is_empty(),
                        "seed {seed}: {}",
                        claim.text
                    );
                    if claim.air.is_empty() {
                        neither += 1;
                    } else {
                        both += 1;
                    }
                }
            }
        }
        // One draw a claim: the fights and every claim's numbers are those
        // at made out 0.
        assert_eq!(
            fight_events(&a.world),
            fight_events(&b.world),
            "seed {seed}"
        );
        let (x, y) = (air_claims(&a.world), air_claims(&b.world));
        assert_eq!(x.len(), y.len());
        for (x, y) in x.iter().zip(&y) {
            assert_eq!((x.day, x.side, x.numbers), (y.day, y.side, y.numbers));
        }
    }
    let pinned = ground_before.len();
    assert_eq!(pinned, 600, "three Japanese claims a seed are pinned");
    assert!(
        named > 0 && both > 0 && neither > 0,
        "{named} {both} {neither}"
    );
}

#[test]
fn a_surface_fights_claim_and_a_bombardments_claim_name_no_type() {
    let options = common::altered_reference("claim_types_air_sea", |dir| {
        common::edit_file(dir, "air_combat.toml", |figures| {
            let mut lines: Vec<String> = figures.lines().map(str::to_string).collect();
            set(&mut lines, "raid_recognised_chance", "1");
            lines.join("\n")
        });
    });
    let mut seen = [0, 0];
    for name in ["24_shells_and_estimates.toml", "04a_night_action.toml"] {
        let path = common::data_dir().join("scenarios").join(name);
        let text = fs::read_to_string(&path).expect("the scenario reads");
        let mut scenario = common::from_text_with(&text, &path, &options).expect("it loads");
        scenario.run_to_end().expect("the run resolves");
        for event in scenario.world.events().iter() {
            if let EventKind::ClaimReceived {
                from_air: false,
                from_bombardment,
                air_types,
                ground_types,
                ..
            } = &event.kind
            {
                seen[usize::from(*from_bombardment)] += 1;
                assert!(air_types.is_empty(), "{name}: {}", event.kind);
                assert!(ground_types.is_empty(), "{name}: {}", event.kind);
            }
        }
    }
    assert!(
        seen[0] > 0 && seen[1] > 0,
        "a surface fight's and a bombardment's: {seen:?}"
    );
}

#[test]
fn the_other_side_is_told_nothing_of_the_claim() {
    let world = run(ALL, "1", &[]);
    for side in [Side::Allied, Side::Japanese] {
        for event in world.side_view(side).events() {
            if let SideEventKind::Whole(EventKind::ClaimReceived { side: owner, .. }) = &event.kind
            {
                assert_eq!(*owner, side, "{} sees no claim of the other", side.name());
            }
        }
    }
}

fn same_claim(a: &BattleClaim, b: &BattleClaim) -> bool {
    (
        a.day,
        a.pulse,
        a.claimed_shot_down,
        &a.air_types,
        &a.ground_types,
    ) == (
        b.day,
        b.pulse,
        b.claimed_shot_down,
        &b.air_types,
        &b.ground_types,
    )
}

#[test]
fn a_claim_passed_between_commands_and_a_picture_keep_the_types() {
    // Command levels on (decision 0134): every command that holds a claim
    // of day 2 holds it with its types in the air.
    let mut scenario = variant(
        ALL,
        "1",
        &[(
            "placeholder = true\n",
            "placeholder = true\ncommand_levels = true\n",
        )],
    );
    scenario.run_to_end().expect("the run resolves");
    let mut held = 0;
    for (side, want) in [
        (Side::Japanese, types(&[(4, 6)])),
        (Side::Allied, types(&[(3, 23), (4, 6)])),
    ] {
        let view = scenario.world.side_view(side);
        for command in view.commands() {
            let picture = view.command(command).expect("a listed command has a view");
            for claim in picture.claims() {
                if claim.day == 2 && claim.claimed_shot_down > 0 {
                    held += 1;
                    assert_eq!(claim.air_types, want, "{side:?} {command:?}");
                }
            }
        }
    }
    assert!(
        held > 2,
        "the claims are passed to more than one command: {held}"
    );

    // A picture after day 2 keeps the claims, and the hash names the field.
    let key = common::umpire_key();
    let mut live = variant(ALL, "1", &[]);
    live.next_day().expect("day 1").expect("day 1 exists");
    live.next_day().expect("day 2").expect("day 2 exists");
    let picture = live.world.whole_picture(key).expect("an evening");
    let rebuild = |picture| {
        World::rebuild_from_picture(
            picture,
            live.world.ref_data().clone(),
            live.world.map().clone(),
            live.world.seed(),
            key,
        )
        .expect("the whole picture is sound")
    };
    let rebuilt = rebuild(picture.clone());
    for side in [Side::Japanese, Side::Allied] {
        let (a, b) = (
            live.world.side_view(side).claims().to_vec(),
            rebuilt.side_view(side).claims().to_vec(),
        );
        assert_eq!(a.len(), b.len());
        assert!(a.iter().zip(&b).all(|(a, b)| same_claim(a, b)));
    }
    assert!(rebuilt
        .side_view(Side::Allied)
        .claims()
        .iter()
        .any(|c| c.air_types == types(&[(3, 23), (4, 6)])));

    // Two worlds that differ only in a claim's types in the air hash
    // differently.
    let mut changed = picture;
    let allied = changed.beliefs.get_mut(&Side::Allied).expect("a belief");
    let claim = allied
        .claims
        .iter_mut()
        .find(|c| !c.air_types.is_empty())
        .expect("a claim with types in the air");
    claim.air_types = types(&[(2, claim.claimed_shot_down)]);
    assert_ne!(rebuilt.state_hash(), rebuild(changed).state_hash());
}

#[test]
fn the_hash_counts_the_types_in_the_air() {
    // The state's layout was raised for the new list (decision 0057).
    assert_eq!(sim_core::hash::STATE_VERSION, b"flagplot-state-v54");
    let made = run(ALL, "1", &[]);
    let unmade = run(ALL, "0", &[]);
    // The fights are the same and the types differ: so do the hashes.
    assert_eq!(fight_events(&made), fight_events(&unmade));
    assert_ne!(made.day_hashes(), unmade.day_hashes());
}

#[test]
fn on_the_frozen_figures_no_claim_names_a_type_and_no_breakdown_moves() {
    let frozen = fs::read_to_string(common::frozen_root().join("air_combat.toml"))
        .expect("the frozen figures read");
    assert!(frozen.contains("raid_recognised_chance = \"0\""));
    let (_, breakdowns_before) = before();
    let mut runs: Vec<(String, Scenario)> = Vec::new();
    for name in [
        RAID,
        "05b_strike.toml",
        "05c_air_combat.toml",
        "05d_carrier_duel.toml",
        "19a_guns_and_the_attack.toml",
    ] {
        runs.push((
            name.to_string(),
            common::load(&PathBuf::from(ROOT).join("data/scenarios").join(name)).expect("it loads"),
        ));
    }
    runs.push(("06a_false_1_made_out_0".to_string(), variant(ALL, "0", &[])));
    let mut compared = 0;
    for (name, mut scenario) in runs {
        scenario.run_to_end().expect("the run resolves");
        for claim in air_claims(&scenario.world) {
            assert!(claim.air.is_empty(), "{name}: {}", claim.text);
            assert!(claim.ground.is_empty(), "{name}: {}", claim.text);
        }
        let mut index = 0;
        for event in scenario.world.events().iter() {
            if let EventKind::ClaimReceived { .. } = &event.kind {
                let factors: Vec<String> = event
                    .breakdown
                    .iter()
                    .flat_map(|b| {
                        b.factors
                            .iter()
                            .map(|f| format!("{}={}", f.name, f.value))
                            .chain([format!("result={}", b.result)])
                    })
                    .collect();
                assert_eq!(
                    breakdowns_before.get(&(name.clone(), index)),
                    Some(&factors.join(" ")),
                    "{name}, claim {index}"
                );
                index += 1;
                compared += 1;
            }
        }
    }
    assert_eq!(compared, breakdowns_before.len(), "every pinned breakdown");
}

#[test]
fn a_picture_whose_claim_names_an_enemy_unit_is_still_refused() {
    // The new list names types, not units: decision 0180's check stands.
    let key = common::umpire_key();
    let mut live = variant(ALL, "1", &[]);
    live.next_day().expect("day 1").expect("day 1 exists");
    let mut picture = live.world.whole_picture(key).expect("an evening");
    let allied = picture.beliefs.get_mut(&Side::Allied).expect("a belief");
    allied.claims[0].by_unit[0].unit = EntityRef::AirGroup(AirGroupId::new(1));
    let refused = World::rebuild_from_picture(
        picture,
        live.world.ref_data().clone(),
        live.world.map().clone(),
        live.world.seed(),
        key,
    );
    assert!(refused.is_err(), "group 1 is Japanese");
}
