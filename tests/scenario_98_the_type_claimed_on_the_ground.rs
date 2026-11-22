//! Scenario 98: the type of the aircraft claimed destroyed on the ground.
//!
//! Proves decision 0179 against a hand calculation. It runs on the frozen
//! step 1 figures with chances altered, as scenario 11's test does, on
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
//! mornings Japanese groups 1 and 2 raid base 2. The aircraft on the
//! ground there are group 4's alone, of aircraft type 3: 10 at the start
//! of day 1 and of day 2, and 8 at the start of day 3. Groups 3 and 5 are
//! in the air and have no damaged aircraft.
//!
//! What the bombs really destroy on the ground: day 1, none; day 2, 2 of
//! group 4; day 3, none. What is claimed on the ground with the false
//! claim chance of bombs at 0: 0, 2 and 0; at 1: 16, 15 and 11.
//!
//! ## The Japanese claim's types, each day
//!
//! "Made out" is `raid_recognised_chance`, the one chance of the rule;
//! "false" is the three false claim chances of scenario 11, all at the
//! one setting. The field each day is type 3 alone.
//!
//! | Day | False 0, made out 1 | False 1, made out 1 | Made out 0, either |
//! | --- | ------------------- | ------------------- | ------------------ |
//! | 1 | none claimed, no type | type 3: 16 | no type |
//! | 2 | type 3: 2 | type 3: 15 | no type |
//! | 3 | none claimed, no type | type 3: 11 | no type |
//!
//! Day 2 at false 1: the 2 really destroyed are of type 3, group 4's;
//! the 13 false are shared over the field, which is type 3 alone. On
//! days 1 and 3 every claimed aircraft is false and of type 3.
//!
//! The Allied claims, of aircraft shot down, name no type on the ground
//! at any setting. (Amended by the architect on Nov 22, 2026, with
//! decision 0210: they now name types in the air where made out, and so
//! does the Japanese claim of the patrol's aircraft; scenario 124 has
//! the numbers. As first written: "name no type at any setting".)
//!
//! The claim's words, day 2, false 1 and made out 1: they hold "15
//! aircraft destroyed on the ground" and name type 3 as decision 0167
//! names a type, with 15 against it. At made out 0 the words are those
//! of scenario 11's run, to the letter but for the clause at their end
//! that names the units (decision 0180, built with this record).
//!
//! ## The type is the group's on the ground, not the bombers'
//!
//! The same three mornings with group 4 given aircraft type 2 in the
//! scenario's text (the raiding bombers, group 1, are type 3). The fight
//! is the same, shot for shot, which is asserted. Day 2, made out 1: at
//! false 0 the claim names type 2: 2; at false 1, type 2: 15. No claim
//! names type 3.
//!
//! ## Nothing else moves
//!
//! At each setting of the false chances, between made out 0 and made out
//! 1: every claim's numbers are the same, and the event logs are the same
//! but for the claims' types and words, of both sides since decision
//! 0210, and the events that decision 0167's recognition records. With made out 0 the logs are
//! those of scenario 11's variants as they run today, but for that
//! clause.
//!
//! ## The rule's own function
//!
//! Given the aircraft really destroyed by type, the number claimed
//! falsely, and the field by type; the list when the types are made out:
//!
//! | Really destroyed | False | The field | The false, shared | The list |
//! | --- | --- | --- | --- | --- |
//! | type 3: 2 | 13 | type 3: 10 | type 3: 13 | type 3: 15 |
//! | type 3: 2 | 3 | type 2: 6, type 3: 4 | 1.8 and 1.2: 1 and 1, the one left to type 2 | type 2: 2, type 3: 3 |
//! | none | 4 | type 2: 6, type 3: 6 | 2 and 2 | type 2: 2, type 3: 2 |
//! | none | 5 | types 2, 3 and 4: 1 each | 1.67 each: 1 each, the two left to types 2 and 3 | type 2: 2, type 3: 2, type 4: 1 |
//! | type 3: 2 | 1 | type 2: 9, type 3: 1 | 0.9 and 0.1: 0 and 0, the one left to type 2 | type 2: 1, type 3: 2 |
//! | type 4: 1 | 0 | type 4: 1 | none | type 4: 1 |
//! | none | 5 | empty | none | empty, and no draw is made |
//!
//! When the types are not made out the list is empty in every row. The
//! list is in order of the type's number and holds no pair with 0; where
//! it is not empty its numbers add to the aircraft claimed.
//!
//! # Properties
//!
//! - **A bombardment's claim names no type,** at made out 1: scenario
//!   24's bombardment claim, which is of aircraft on the ground, has an
//!   empty list.
//! - **A type named was on the field.** Over 200 seeds of the base raid
//!   at the placeholder false chances and made out 0.75: every type in a
//!   Japanese claim is the type of a group that had an aircraft on the
//!   ground at base 2 at the start of that pulse, read from the truth;
//!   and each list is empty or adds to the aircraft claimed on the
//!   ground.
//! - **One draw a claim.** With the chance at 0.75, the fights and every
//!   claim's numbers over those seeds are those at made out 0.
//! - **The other side is told nothing:** the Allied side's events hold no
//!   Japanese claim, with types or without.
//! - **A claim passed between commands keeps its types** (decision 0135),
//!   and a picture keeps them (decision 0170): a world rebuilt from the
//!   picture after day 2 holds the same claim.
//! - **The hash names the field:** `hash::name_every_field` lists it, and
//!   two worlds that differ only in a claim's types hash differently.
//! - **The frozen figures are as they were:** no figures file is changed,
//!   and on them no claim names a type.

mod common;

use std::fs;
use std::path::PathBuf;

use sim_core::belief::BattleClaim;
use sim_core::claims::{ground_claim_types, UnitClaim};
use sim_core::events::{claim_text, Event, EventKind};
use sim_core::ids::{AirGroupId, AircraftTypeId, BaseId, EntityRef};
use sim_core::side_events::SideEventKind;
use sim_core::state::{AirGroupLocation, Side};
use sim_core::World;
use sim_data::Scenario;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const RAID: &str = "06a_base_raid.toml";

/// The three false claim chances of scenario 11 at one setting, and the
/// chance that the types are made out.
const FALSE_KEYS: [&str; 3] = [
    "fighter_false_claim_chance",
    "anti_aircraft_false_claim_chance",
    "bomb_false_claim_chance",
];
const PLACEHOLDER: [&str; 3] = ["0.1", "0.05", "0.1"];
const NONE: [&str; 3] = ["0", "0", "0"];
const ALL: [&str; 3] = ["1", "1", "1"];

/// Group 4 given aircraft type 2 in the scenario's text.
const GROUP_4_OF_TYPE_2: (&str, &str) = (
    "base = 2\naircraft_type = 3\nready = 10\n",
    "base = 2\naircraft_type = 2\nready = 10\n",
);

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
    let made_out = made_out.to_string();
    let options = common::altered_reference("claim_types", |dir| {
        common::edit_file(dir, "air_combat.toml", |figures| {
            let mut lines: Vec<String> = figures.lines().map(str::to_string).collect();
            for (key, value) in FALSE_KEYS.iter().zip(chances) {
                set(&mut lines, key, value);
            }
            set(&mut lines, "raid_recognised_chance", &made_out);
            lines.join("\n")
        });
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
    numbers: [u32; 4],
    types: Vec<(u32, u32)>,
    by_unit: Vec<UnitClaim>,
    text: String,
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
                by_unit,
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
                types: ground_types.iter().map(|(t, n)| (t.raw(), *n)).collect(),
                by_unit: by_unit.clone(),
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

/// The words of a claim without the clause of decision 0180 at its end.
fn before_the_units(text: &str) -> &str {
    text.split("; claimed by ").next().expect("words")
}

fn types(list: &[(u32, u32)]) -> Vec<(AircraftTypeId, u32)> {
    list.iter()
        .map(|(t, n)| (AircraftTypeId::new(*t), *n))
        .collect()
}

// ---------------------------------------------------------------------
// The Japanese claim's types, each day.
// ---------------------------------------------------------------------

#[test]
fn the_types_claimed_each_day_are_the_hand_calculation() {
    let none: Vec<(u32, u32)> = Vec::new();
    for (chances, made_out, ground, want) in [
        (
            NONE,
            "1",
            [0, 2, 0],
            [none.clone(), vec![(3, 2)], none.clone()],
        ),
        (
            ALL,
            "1",
            [16, 15, 11],
            [vec![(3, 16)], vec![(3, 15)], vec![(3, 11)]],
        ),
        (
            NONE,
            "0",
            [0, 2, 0],
            [none.clone(), none.clone(), none.clone()],
        ),
        (
            ALL,
            "0",
            [16, 15, 11],
            [none.clone(), none.clone(), none.clone()],
        ),
    ] {
        let world = run(chances, made_out, &[]);
        let claims = air_claims(&world);
        assert_eq!(claims.len(), 6, "{chances:?} {made_out}");
        for day in 1..=3 {
            let at = day as usize - 1;
            let raid = claim_on(&claims, Side::Japanese, day);
            assert_eq!(
                raid.numbers[3], ground[at],
                "{chances:?} {made_out} day {day}"
            );
            assert_eq!(raid.types, want[at], "{chances:?} {made_out} day {day}");
            // The Allied claims, of aircraft shot down, name no type.
            assert!(claim_on(&claims, Side::Allied, day).types.is_empty());
        }
    }
}

#[test]
fn the_words_of_day_2_name_the_type_after_the_number() {
    let made = run(ALL, "1", &[]);
    let claim = claim_on(&air_claims(&made), Side::Japanese, 2).clone();
    assert!(claim.text.contains("15 aircraft destroyed on the ground"));
    assert_eq!(
        before_the_units(&claim.text),
        // Since decision 0210 the claim names the patrol's type in the air as
        // well (scenario 124 has the words).
        "Japanese claim at (5,0): 6 aircraft shot down (6 of aircraft_type:4), 15 aircraft \
         destroyed on the ground (15 of aircraft_type:3)"
    );
    // At made out 0 the words are scenario 11's, to the letter, but for the
    // clause that names the units.
    for chances in [NONE, ALL, PLACEHOLDER] {
        let world = run(chances, "0", &[]);
        for event in world.events().iter() {
            if let EventKind::ClaimReceived {
                side,
                hex,
                claimed_sunk,
                claimed_hits,
                claimed_kind,
                claimed_shot_down,
                claimed_on_ground,
                from_air,
                from_bombardment,
                ground_types,
                ..
            } = &event.kind
            {
                assert!(ground_types.is_empty());
                let text = event.kind.to_string();
                assert_eq!(
                    before_the_units(&text),
                    claim_text(
                        *side,
                        *hex,
                        *claimed_sunk,
                        *claimed_hits,
                        *claimed_kind,
                        *claimed_shot_down,
                        *claimed_on_ground,
                        *from_air,
                        *from_bombardment,
                        false,
                        &[],
                        &[],
                        &[],
                    )
                );
            }
        }
    }
    let unmade = run(ALL, "0", &[]);
    let claim = claim_on(&air_claims(&unmade), Side::Japanese, 2).clone();
    assert_eq!(
        before_the_units(&claim.text),
        "Japanese claim at (5,0): 6 aircraft shot down, 15 aircraft destroyed on the ground"
    );
}

// ---------------------------------------------------------------------
// The type is the group's on the ground, not the bombers'.
// ---------------------------------------------------------------------

#[test]
fn the_type_named_is_the_group_on_the_grounds_not_the_bombers() {
    for (chances, want) in [(NONE, vec![(2, 2)]), (ALL, vec![(2, 15)])] {
        let plain = run(chances, "1", &[]);
        let other = run(chances, "1", &[GROUP_4_OF_TYPE_2]);
        // The fight is the same, shot for shot.
        assert_eq!(fight_events(&plain), fight_events(&other), "{chances:?}");
        let claims = air_claims(&other);
        assert_eq!(claim_on(&claims, Side::Japanese, 2).types, want);
        for claim in &claims {
            assert!(
                claim.types.iter().all(|(t, _)| *t != 3),
                "{chances:?}: no claim names type 3: {}",
                claim.text
            );
        }
    }
}

// ---------------------------------------------------------------------
// Nothing else moves.
// ---------------------------------------------------------------------

#[test]
fn nothing_else_moves_with_the_chance_of_making_out_a_type() {
    for chances in [NONE, ALL, PLACEHOLDER] {
        let made = run(chances, "1", &[]);
        let unmade = run(chances, "0", &[]);
        assert_eq!(fight_events(&made), fight_events(&unmade), "{chances:?}");
        assert_eq!(made.umpire(), unmade.umpire(), "{chances:?}");
        let (a, b) = (air_claims(&made), air_claims(&unmade));
        assert_eq!(a.len(), b.len());
        for (a, b) in a.iter().zip(&b) {
            assert_eq!((a.day, a.side, a.numbers), (b.day, b.side, b.numbers));
            assert_eq!(a.by_unit, b.by_unit, "{chances:?}");
            // Since decision 0210 the Allied claims' types in the air and
            // their words move with the chance; their types on the ground,
            // none, do not.
            if a.side == Side::Allied {
                assert_eq!(
                    a.types, b.types,
                    "{chances:?}: the Allied claims do not move"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------
// The rule's own function.
// ---------------------------------------------------------------------

/// A row of the table: really destroyed, false, the field, the list.
type Row = (
    &'static [(u32, u32)],
    u32,
    &'static [(u32, u32)],
    &'static [(u32, u32)],
);

#[test]
fn the_rules_function_is_the_table_of_the_hand_calculation() {
    let rows: [Row; 7] = [
        (&[(3, 2)], 13, &[(3, 10)], &[(3, 15)]),
        (&[(3, 2)], 3, &[(2, 6), (3, 4)], &[(2, 2), (3, 3)]),
        (&[], 4, &[(2, 6), (3, 6)], &[(2, 2), (3, 2)]),
        (&[], 5, &[(2, 1), (3, 1), (4, 1)], &[(2, 2), (3, 2), (4, 1)]),
        (&[(3, 2)], 1, &[(2, 9), (3, 1)], &[(2, 1), (3, 2)]),
        (&[(4, 1)], 0, &[(4, 1)], &[(4, 1)]),
        (&[], 5, &[], &[]),
    ];
    for (real, false_claimed, field, want) in rows {
        let list = ground_claim_types(&types(real), false_claimed, &types(field));
        assert_eq!(list, types(want), "{real:?} {false_claimed} {field:?}");
        // In order of the type's number, no pair with 0, adding to the
        // aircraft claimed where it is not empty.
        assert!(list.windows(2).all(|pair| pair[0].0 < pair[1].0));
        assert!(list.iter().all(|(_, n)| *n > 0));
        if !list.is_empty() {
            let claimed: u32 = real.iter().map(|(_, n)| n).sum::<u32>() + false_claimed;
            assert_eq!(list.iter().map(|(_, n)| n).sum::<u32>(), claimed);
        }
    }
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn a_bombardments_claim_names_no_type() {
    let options = common::altered_reference("claim_types_bombardment", |dir| {
        common::edit_file(dir, "air_combat.toml", |figures| {
            let mut lines: Vec<String> = figures.lines().map(str::to_string).collect();
            set(&mut lines, "raid_recognised_chance", "1");
            lines.join("\n")
        });
    });
    let path = common::data_dir().join("scenarios/24_shells_and_estimates.toml");
    let text = fs::read_to_string(&path).expect("scenario 24 reads");
    let mut scenario = common::from_text_with(&text, &path, &options).expect("it loads");
    scenario.run_to_end().expect("the run resolves");
    let mut bombardments = 0;
    for event in scenario.world.events().iter() {
        if let EventKind::ClaimReceived {
            from_bombardment: true,
            claimed_on_ground,
            ground_types,
            by_unit,
            ..
        } = &event.kind
        {
            bombardments += 1;
            assert!(*claimed_on_ground > 0 || ground_types.is_empty());
            assert!(ground_types.is_empty(), "{}", event.kind);
            assert!(by_unit.is_empty(), "{}", event.kind);
        }
    }
    assert!(bombardments > 0, "scenario 24 has a bombardment's claim");
}

/// The types of the groups with an aircraft at base 2, read from the truth
/// at the start of a day: every aircraft on the ground there at the start of
/// the morning pulse is of one of them.
fn types_at_base_2(world: &World) -> Vec<AircraftTypeId> {
    world
        .umpire()
        .air_groups
        .values()
        .filter(|g| g.location == AirGroupLocation::Base(BaseId::new(2)) && g.ready + g.damaged > 0)
        .map(|g| g.aircraft_type)
        .collect()
}

#[test]
fn over_200_seeds_a_type_named_was_on_the_field_and_one_draw_moves_nothing_else() {
    let made = variant(PLACEHOLDER, "0.75", &[]);
    let unmade = variant(PLACEHOLDER, "0", &[]);
    let mut named = 0;
    for seed in 1..=200 {
        let mut a = made.with_seed(seed).expect("a seed");
        let mut b = unmade.with_seed(seed).expect("a seed");
        let mut fields: Vec<Vec<AircraftTypeId>> = Vec::new();
        loop {
            fields.push(types_at_base_2(&a.world));
            if a.next_day().expect("a day").is_none() {
                break;
            }
        }
        b.run_to_end().expect("the run resolves");
        for claim in air_claims(&a.world) {
            if claim.side != Side::Japanese {
                assert!(claim.types.is_empty());
                continue;
            }
            let field = &fields[claim.day as usize - 1];
            for (t, _) in &claim.types {
                assert!(
                    field.contains(&AircraftTypeId::new(*t)),
                    "seed {seed}: {}",
                    claim.text
                );
            }
            if !claim.types.is_empty() {
                named += 1;
                let sum: u32 = claim.types.iter().map(|(_, n)| n).sum();
                assert_eq!(sum, claim.numbers[3], "seed {seed}: {}", claim.text);
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
            assert_eq!(x.by_unit, y.by_unit);
        }
    }
    assert!(named > 0, "some claim over the seeds names a type");
}

#[test]
fn the_other_side_is_told_nothing_of_the_claim() {
    let world = run(ALL, "1", &[]);
    for (side, other) in [
        (Side::Allied, Side::Japanese),
        (Side::Japanese, Side::Allied),
    ] {
        for event in world.side_view(side).events() {
            if let SideEventKind::Whole(EventKind::ClaimReceived { side: owner, .. }) = &event.kind
            {
                assert_eq!(
                    *owner,
                    side,
                    "{} sees no {} claim",
                    side.name(),
                    other.name()
                );
            }
        }
    }
}

fn same_claim(a: &BattleClaim, b: &BattleClaim) -> bool {
    (
        a.day,
        a.pulse,
        a.claimed_on_ground,
        &a.ground_types,
        &a.by_unit,
    ) == (
        b.day,
        b.pulse,
        b.claimed_on_ground,
        &b.ground_types,
        &b.by_unit,
    )
}

#[test]
fn a_claim_passed_between_commands_and_a_picture_keep_the_types() {
    // Command levels on (decision 0134): every command that holds the
    // Japanese claim of day 2 holds it with its types.
    let mut scenario = variant(
        ALL,
        "1",
        &[(
            "placeholder = true\n",
            "placeholder = true\ncommand_levels = true\n",
        )],
    );
    scenario.run_to_end().expect("the run resolves");
    let view = scenario.world.side_view(Side::Japanese);
    let mut held = 0;
    for command in view.commands() {
        let picture = view.command(command).expect("a listed command has a view");
        for claim in picture.claims() {
            if claim.day == 2 && claim.claimed_on_ground > 0 {
                held += 1;
                assert_eq!(claim.ground_types, types(&[(3, 15)]), "{command:?}");
            }
        }
    }
    assert!(
        held > 1,
        "the claim is passed to more than one command: {held}"
    );

    // A picture after day 2 keeps the claim, and the hash names the field.
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
    let (a, b) = (
        live.world.side_view(Side::Japanese).claims().to_vec(),
        rebuilt.side_view(Side::Japanese).claims().to_vec(),
    );
    assert_eq!(a.len(), b.len());
    assert!(a.iter().zip(&b).all(|(a, b)| same_claim(a, b)));
    assert!(b.iter().any(|c| c.ground_types == types(&[(3, 15)])));

    let mut changed = picture;
    let japanese = changed.beliefs.get_mut(&Side::Japanese).expect("a belief");
    let claim = japanese
        .claims
        .iter_mut()
        .find(|c| !c.ground_types.is_empty())
        .expect("a claim with types");
    claim.ground_types = types(&[(2, claim.claimed_on_ground)]);
    assert_ne!(rebuilt.state_hash(), rebuild(changed).state_hash());
}

#[test]
fn a_picture_whose_claim_names_an_enemy_unit_is_refused() {
    let key = common::umpire_key();
    let mut live = variant(ALL, "1", &[]);
    live.next_day().expect("day 1").expect("day 1 exists");
    let mut picture = live.world.whole_picture(key).expect("an evening");
    let japanese = picture.beliefs.get_mut(&Side::Japanese).expect("a belief");
    japanese.claims[0].by_unit[0].unit = EntityRef::AirGroup(AirGroupId::new(3));
    let refused = World::rebuild_from_picture(
        picture,
        live.world.ref_data().clone(),
        live.world.map().clone(),
        live.world.seed(),
        key,
    );
    assert!(refused.is_err(), "group 3 is Allied");
}

#[test]
fn on_the_frozen_figures_no_claim_names_a_type() {
    let frozen = fs::read_to_string(common::frozen_root().join("air_combat.toml"))
        .expect("the frozen figures read");
    assert!(frozen.contains("raid_recognised_chance = \"0\""));
    for name in [
        RAID,
        "05b_strike.toml",
        "05c_air_combat.toml",
        "05d_carrier_duel.toml",
    ] {
        let mut scenario =
            common::load(&PathBuf::from(ROOT).join("data/scenarios").join(name)).expect("it loads");
        scenario.run_to_end().expect("the run resolves");
        for claim in air_claims(&scenario.world) {
            assert!(claim.types.is_empty(), "{name}: {}", claim.text);
        }
    }
}
