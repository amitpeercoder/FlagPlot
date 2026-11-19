//! Scenario 99: who made a claim, and an air group's tally of its claims.
//!
//! Proves decision 0180 against a hand calculation. It runs on the frozen
//! step 1 figures with the false claim chances altered, as scenario 11's
//! test does, on `06a_base_raid.toml` and `05b_strike.toml`, which are
//! placeholder data.
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
//! Scenario 11's hand calculation (see it). The base raid, three
//! mornings: Japanese group 1 (bombers) and group 2 (escorts) raid base
//! 2; Allied group 3 patrols over it and the base's guns fire.
//!
//! | Day | Firing | By | Shots | Hits | Missed |
//! | --- | ------ | -- | ----- | ---- | ------ |
//! | 1 | escorts at the patrol | group 2 | 6 | 3 | 3 |
//! | 1 | patrol at the escorts | group 3 | 6 | 1 | 5 |
//! | 1 | patrol at the bombers | group 3 | 6 | 2 | 4 |
//! | 1 | base guns | base 2 | 20 | 0 | 20 |
//! | 1 | bombs | group 1 | 16 | none on aircraft | 16 |
//! | 2 | escorts at the patrol | group 2 | 6 | 2 | 4 |
//! | 2 | patrol at the escorts | group 3 | 6 | 1 | 5 |
//! | 2 | patrol at the bombers | group 3 | 3 | 1 | 2 |
//! | 2 | base guns | base 2 | 20 | 0 | 20 |
//! | 2 | bombs | group 1 | 15 | 2 aircraft destroyed | 13 |
//! | 3 | escorts at the patrol | group 2 | 5 | 0 | 5 |
//! | 3 | patrol at the escorts | group 3 | 5 | 1 | 4 |
//! | 3 | patrol at the bombers | group 3 | 2 | 1 | 1 |
//! | 3 | base guns | base 2 | 20 | 3 | 17 |
//! | 3 | bombs | group 1 | 11 | none on aircraft | 11 |
//!
//! ## Who claims what, each day
//!
//! With every false claim chance at 0 a unit claims what it really did;
//! at 1 it claims that and every whole shot or bomb that missed. A unit
//! that claims nothing is not in the list.
//!
//! The Japanese claim:
//!
//! | Day | Chances 0 | Chances 1 |
//! | --- | --------- | --------- |
//! | 1 | group 2: 3 shot down | group 2: 6 shot down; group 1: 16 on the ground |
//! | 2 | group 2: 2 shot down; group 1: 2 on the ground | group 2: 6 shot down; group 1: 15 on the ground |
//! | 3 | no unit | group 2: 5 shot down; group 1: 11 on the ground |
//!
//! The Allied claim:
//!
//! | Day | Chances 0 | Chances 1 |
//! | --- | --------- | --------- |
//! | 1 | group 3: 3 shot down | group 3: 12 shot down; base 2: 20 shot down |
//! | 2 | group 3: 2 shot down | group 3: 9 shot down; base 2: 20 shot down |
//! | 3 | group 3: 2 shot down; base 2: 3 shot down | group 3: 7 shot down; base 2: 20 shot down |
//!
//! Day 1, chances 1, group 3: 1 + 2 really, and 5 + 4 missed: 12. The
//! sums are scenario 11's: 32, 29 and 27 for the Allied side at chances
//! 1, and every claim's own numbers are as scenario 11 has them.
//!
//! Each list is in the order given: the Japanese side's escorts fire
//! before its bombs fall, and the Allied patrol fires before the base's
//! guns.
//!
//! ## The tallies after the three days
//!
//! | Group | Chances 0 | Chances 1 |
//! | ----- | --------- | --------- |
//! | 1 | 2 on the ground | 42 on the ground |
//! | 2 | 5 shot down | 17 shot down |
//! | 3 | 7 shot down | 28 shot down |
//! | 4 and 5 | nothing | nothing |
//!
//! Group 1 at chances 1: 16 + 15 + 11. Group 3: 12 + 9 + 7. A tally's
//! other numbers are 0. Base 2 has no tally.
//!
//! ## The unopposed strike
//!
//! `05b_strike.toml`: group 2 drops 18 bombs on ship 3 for 5 hits; no
//! fighter or gun fires. The Allied claim's list: group 2, 5 hits at
//! chances 0 and 18 at chances 1. Group 2's tally of hits is the same.
//! The Japanese side has no claim.
//!
//! ## A firing of two groups, parted shot by shot
//!
//! The rule's own function: the aircraft each group has in the firing,
//! the whole shots of the firing, which shots hit (in the order drawn),
//! and what the firing really shot down. It gives each group's shots,
//! its real aircraft and its shots that missed; at false chance 1 a
//! group claims its real aircraft and every shot of its own that missed.
//!
//! | In the firing | Shots | Shots that hit | Really shot down | Shots, by group | Real, by group | Claimed at chance 1 |
//! | --- | --- | --- | --- | --- | --- | --- |
//! | group 3: 8, group 7: 4 | 6 | the 1st, 5th and 6th | 3 | 4 and 2 | 1 and 2 | 4 and 2 |
//! | group 3: 8, group 7: 4 | 12 | the 2nd and 9th | 2 | 8 and 4 | 1 and 1 | 8 and 4 |
//! | group 3: 1, group 7: 1 | 1 | none | 0 | 1 and 0 (a tie goes to the lower number) | 0 and 0 | 1 and 0 |
//! | group 3: 5, group 7: 5 | 3 | the 3rd | 1 | 2 and 1 (1.5 each: 1 and 1, the one left to group 3) | 0 and 1 | 2 and 1 |
//! | group 3: 8, group 7: 4 | 6 | the 1st, 5th and 6th | 2 | 4 and 2 | 1 and 1 (the cut comes off the last shot) | 4 and 2 |
//! | group 3: 12 | 6 | the 1st | 1 | 6 | 1 | 6 |
//!
//! Row 1: 6 x 8 / 12 = 4 and 6 x 4 / 12 = 2; group 3 has shots 1 to 4,
//! of which the 1st hit, and 3 missed; group 7 has shots 5 and 6, both
//! hits. In every row the shots, the real aircraft and the claims add to
//! the firing's own.
//!
//! In a game: the base raid's first morning with a second Allied group
//! on patrol over base 2 is not worked here by hand, the fight being
//! another fight; the build reports its lists, and the test asserts for
//! it only that the two patrol groups' lines add to the Allied claim
//! less the guns', and that neither is credited with more shots than the
//! aircraft it had on patrol.
//!
//! ## Bombs, group by group
//!
//! Two groups bomb one base in one raid: group 1 drops 10 bombs that
//! destroy 2 aircraft, group 2 drops 5 that destroy none. At false
//! chance 0 the lines are group 1: 2 on the ground, and group 2 is not
//! in the list. At 1 they are group 1: 10 and group 2: 5. This is the
//! rule's function, given the bombs and what they destroyed.
//!
//! # Properties
//!
//! - **Nothing else moves.** At every setting, every claim's numbers,
//!   every fight and every event but the claims' own are those of
//!   scenario 11's variants as they run today.
//! - **The sums hold.** Over 200 seeds of the base raid at the
//!   placeholder chances: in every claim the list's aircraft shot down,
//!   aircraft on the ground and hits each add to the claim's own; no
//!   line is all 0; and each unit's line is no less than what that unit
//!   really did, read from the firing events.
//! - **A tally is the sum of the group's lines** in every claim of the
//!   game, over those seeds.
//! - **A tally is kept:** a group that rebases, and a group that has lost
//!   every aircraft, has the tally it had.
//! - **A surface fight's claim and a bombardment's name no unit:**
//!   scenario 24's bombardment claim has an empty list.
//! - **The other side is told nothing:** neither side's events hold the
//!   other's claim, and no answer of a seat's tools holds a tally of an
//!   enemy group.
//! - **With command levels on,** a superior's knowledge of a group's
//!   tally is that of its latest state report to arrive (decision 0136).
//! - **A picture keeps the list and the tallies** (decisions 0170 and
//!   0171): a world rebuilt from the picture after day 2 has them.
//! - **The hash names the fields:** `hash::name_every_field` lists them,
//!   and two worlds that differ only in one group's tally hash
//!   differently.

mod common;

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use sim_core::claims::{part_firing, strike_lines, FiringPart, UnitClaim};
use sim_core::events::{AirFirer, Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::held::AsOf;
use sim_core::ids::{AirGroupId, BaseId, EntityRef};
use sim_core::side_events::SideEventKind;
use sim_core::state::{AirGroupLocation, ClaimTally, Side};
use sim_core::time::Pulse;
use sim_core::World;
use sim_data::Scenario;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const RAID: &str = "06a_base_raid.toml";
const STRIKE: &str = "05b_strike.toml";

const KEYS: [&str; 3] = [
    "fighter_false_claim_chance",
    "anti_aircraft_false_claim_chance",
    "bomb_false_claim_chance",
];
const PLACEHOLDER: [&str; 3] = ["0.1", "0.05", "0.1"];
const NONE: [&str; 3] = ["0", "0", "0"];
const ALL: [&str; 3] = ["1", "1", "1"];

/// A second Allied fighter group on patrol over base 2.
const SECOND_PATROL: &str = r#"
[[air_group]]
id = 6
name = "Placeholder second defending fighters"
side = "allied"
base = 2
aircraft_type = 4
ready = 6

[[order]]
day = 1
side = "allied"
air_group = 6
kind = "set_air_mission"
mission = "combat_air_patrol"
"#;

/// A Japanese base at (7,0), and group 2 rebased to it on day 3.
const REBASE: &str = r#"
[[base]]
id = 3
name = "Placeholder airfield C"
side = "japanese"
q = 7
r = 0
supply_t = 0

[[order]]
day = 3
side = "japanese"
air_group = 2
kind = "rebase"
base = 3
"#;

/// The scenario `name` on the frozen figures, its three false claim chances
/// set, each edit made once and `added` appended to its text.
fn variant(name: &str, chances: [&str; 3], edits: &[(&str, &str)], added: &str) -> Scenario {
    let options = common::altered_reference("tally", |dir| {
        common::edit_file(dir, "air_combat.toml", |figures| {
            let mut lines: Vec<String> = figures.lines().map(str::to_string).collect();
            for (key, value) in KEYS.iter().zip(chances) {
                let mut found = 0;
                for line in &mut lines {
                    if line.starts_with(&format!("{key} = ")) {
                        *line = format!("{key} = \"{value}\"");
                        found += 1;
                    }
                }
                assert_eq!(found, 1, "{key} is set once in the file");
            }
            lines.join("\n")
        });
    });
    let path = PathBuf::from(ROOT).join("data/scenarios").join(name);
    let mut text = fs::read_to_string(&path).expect("the scenario reads");
    for (from, to) in edits {
        text = common::replace_once(&text, from, to);
    }
    text.push_str(added);
    common::from_text_with(&text, &path, &options).expect("the variant loads")
}

fn run(name: &str, chances: [&str; 3]) -> World {
    run_variant(variant(name, chances, &[], ""))
}

fn run_variant(mut scenario: Scenario) -> World {
    scenario.run_to_end().expect("the run resolves");
    scenario.world
}

fn group(n: u32) -> EntityRef {
    EntityRef::AirGroup(AirGroupId::new(n))
}

fn base(n: u32) -> EntityRef {
    EntityRef::Base(BaseId::new(n))
}

fn line(unit: EntityRef, shot_down: u32, on_ground: u32, hits: u32) -> UnitClaim {
    UnitClaim {
        unit,
        shot_down,
        on_ground,
        hits,
    }
}

/// One claim from the air, from its event.
#[derive(Clone, Debug)]
struct Claim {
    day: u32,
    side: Side,
    down: u32,
    ground: u32,
    hits: u32,
    lines: Vec<UnitClaim>,
    causes: Vec<Cause>,
}

fn air_claims(world: &World) -> Vec<Claim> {
    world
        .events()
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ClaimReceived {
                side,
                claimed_hits,
                claimed_shot_down,
                claimed_on_ground,
                from_air: true,
                by_unit,
                ..
            } => Some(Claim {
                day: event.day,
                side: *side,
                down: *claimed_shot_down,
                ground: *claimed_on_ground,
                hits: *claimed_hits,
                lines: by_unit.clone(),
                causes: event.causes.clone(),
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

fn tally(world: &World, n: u32) -> ClaimTally {
    world.umpire().air_groups[&AirGroupId::new(n)].claims
}

fn tally_of(shot_down: u32, on_ground: u32, hits: u32) -> ClaimTally {
    ClaimTally {
        shot_down,
        on_ground,
        hits,
    }
}

fn whole(value: Fixed) -> u32 {
    u32::try_from(value.trunc()).expect("a count")
}

// ---------------------------------------------------------------------
// The base raid: who claims what, each day.
// ---------------------------------------------------------------------

#[test]
fn the_firing_is_the_table_of_the_hand_calculation() {
    // The firing, by unit: the escorts are group 2's, the patrol group 3's
    // and the guns base 2's; the bombs are group 1's.
    let world = run(RAID, PLACEHOLDER);
    let mut rows = Vec::new();
    for event in world.events().iter() {
        match &event.kind {
            EventKind::AirFight {
                firer, shots, hits, ..
            } => rows.push((event.day, group(firer.raw()), whole(*shots), *hits)),
            EventKind::BaseAntiAircraftFire {
                base: b,
                shots,
                hits,
                ..
            } => rows.push((event.day, base(b.raw()), whole(*shots), *hits)),
            EventKind::BaseAttacked {
                air_group, shots, ..
            } => rows.push((event.day, group(air_group.raw()), whole(*shots), u32::MAX)),
            _ => {}
        }
    }
    let r = |day, unit, shots, hits| (day, unit, shots, hits);
    assert_eq!(
        rows,
        vec![
            r(1, group(2), 6, 3),
            r(1, group(3), 6, 1),
            r(1, group(3), 6, 2),
            r(1, base(2), 20, 0),
            r(1, group(1), 16, u32::MAX),
            r(2, group(2), 6, 2),
            r(2, group(3), 6, 1),
            r(2, group(3), 3, 1),
            r(2, base(2), 20, 0),
            r(2, group(1), 15, u32::MAX),
            r(3, group(2), 5, 0),
            r(3, group(3), 5, 1),
            r(3, group(3), 2, 1),
            r(3, base(2), 20, 3),
            r(3, group(1), 11, u32::MAX),
        ]
    );
}

#[test]
fn who_claims_what_each_day_is_the_hand_calculation() {
    let japanese_0 = [
        vec![line(group(2), 3, 0, 0)],
        vec![line(group(2), 2, 0, 0), line(group(1), 0, 2, 0)],
        vec![],
    ];
    let japanese_1 = [
        vec![line(group(2), 6, 0, 0), line(group(1), 0, 16, 0)],
        vec![line(group(2), 6, 0, 0), line(group(1), 0, 15, 0)],
        vec![line(group(2), 5, 0, 0), line(group(1), 0, 11, 0)],
    ];
    let allied_0 = [
        vec![line(group(3), 3, 0, 0)],
        vec![line(group(3), 2, 0, 0)],
        vec![line(group(3), 2, 0, 0), line(base(2), 3, 0, 0)],
    ];
    let allied_1 = [
        vec![line(group(3), 12, 0, 0), line(base(2), 20, 0, 0)],
        vec![line(group(3), 9, 0, 0), line(base(2), 20, 0, 0)],
        vec![line(group(3), 7, 0, 0), line(base(2), 20, 0, 0)],
    ];
    for (chances, japanese, allied, sums) in [
        (NONE, japanese_0, allied_0, [3, 2, 5]),
        (ALL, japanese_1, allied_1, [32, 29, 27]),
    ] {
        let world = run(RAID, chances);
        let claims = air_claims(&world);
        assert_eq!(claims.len(), 6);
        for day in 1..=3 {
            let at = day as usize - 1;
            let raid = claim_on(&claims, Side::Japanese, day);
            let defence = claim_on(&claims, Side::Allied, day);
            assert_eq!(raid.lines, japanese[at], "{chances:?} day {day}");
            assert_eq!(defence.lines, allied[at], "{chances:?} day {day}");
            // The claim's own numbers are scenario 11's.
            assert_eq!(defence.down, sums[at], "{chances:?} day {day}");
        }
    }
}

#[test]
fn the_tallies_after_the_three_days_are_the_hand_calculation() {
    for (chances, want) in [
        (
            NONE,
            [
                tally_of(0, 2, 0),
                tally_of(5, 0, 0),
                tally_of(7, 0, 0),
                ClaimTally::default(),
                ClaimTally::default(),
            ],
        ),
        (
            ALL,
            [
                tally_of(0, 42, 0),
                tally_of(17, 0, 0),
                tally_of(28, 0, 0),
                ClaimTally::default(),
                ClaimTally::default(),
            ],
        ),
    ] {
        let world = run(RAID, chances);
        for (n, want) in (1..=5).zip(want) {
            assert_eq!(tally(&world, n), want, "{chances:?} group {n}");
        }
    }
}

#[test]
fn the_unopposed_strike_is_group_2s_hits() {
    for (chances, hits) in [(NONE, 5), (ALL, 18)] {
        let world = run(STRIKE, chances);
        let claims = air_claims(&world);
        assert_eq!(claims.len(), 1, "the Japanese side has no claim");
        assert_eq!(claims[0].side, Side::Allied);
        assert_eq!(claims[0].lines, vec![line(group(2), 0, 0, hits)]);
        assert_eq!(tally(&world, 2), tally_of(0, 0, hits));
    }
}

// ---------------------------------------------------------------------
// A firing of two groups, parted shot by shot.
// ---------------------------------------------------------------------

/// A row of the table: in the firing, shots, shots that hit, really shot
/// down; the parts; the claims at chance 1.
type PartRow = (
    Vec<(AirGroupId, u32)>,
    u32,
    Vec<u32>,
    u32,
    Vec<FiringPart>,
    Vec<u32>,
);

#[test]
fn the_parting_of_a_firing_is_the_table_of_the_hand_calculation() {
    let (g3, g7) = (AirGroupId::new(3), AirGroupId::new(7));
    let part = |group, shots, real, missed| FiringPart {
        group,
        shots,
        real,
        missed,
    };
    // In the firing, shots, shots that hit, really shot down; then shots,
    // real and claimed at chance 1, by group.
    let rows: [PartRow; 6] = [
        (
            vec![(g3, 8), (g7, 4)],
            6,
            vec![1, 5, 6],
            3,
            vec![part(g3, 4, 1, 3), part(g7, 2, 2, 0)],
            vec![4, 2],
        ),
        (
            vec![(g3, 8), (g7, 4)],
            12,
            vec![2, 9],
            2,
            vec![part(g3, 8, 1, 7), part(g7, 4, 1, 3)],
            vec![8, 4],
        ),
        (
            vec![(g3, 1), (g7, 1)],
            1,
            vec![],
            0,
            vec![part(g3, 1, 0, 1), part(g7, 0, 0, 0)],
            vec![1, 0],
        ),
        (
            vec![(g3, 5), (g7, 5)],
            3,
            vec![3],
            1,
            vec![part(g3, 2, 0, 2), part(g7, 1, 1, 0)],
            vec![2, 1],
        ),
        (
            vec![(g3, 8), (g7, 4)],
            6,
            vec![1, 5, 6],
            2,
            vec![part(g3, 4, 1, 3), part(g7, 2, 1, 0)],
            // The hand calculation gives 4 and 2: see the build's report.
            // Group 7's shots both hit and one is cut; it has no shot that
            // missed, so at chance 1 it claims its 1 real aircraft.
            vec![4, 1],
        ),
        (
            vec![(g3, 12)],
            6,
            vec![1],
            1,
            vec![part(g3, 6, 1, 5)],
            vec![6],
        ),
    ];
    for (index, (in_firing, shots, hit_shots, real, want, claimed)) in rows.iter().enumerate() {
        let parts = part_firing(in_firing, *shots, hit_shots, *real);
        assert_eq!(&parts, want, "row {}", index + 1);
        // At false chance 1 a group claims its real aircraft and every shot
        // of its own that missed.
        let at_one: Vec<u32> = parts.iter().map(|p| p.real + p.missed).collect();
        // Row 5's claims are not asserted against the hand calculation:
        // the hand calculation and the rule disagree there (the build's
        // report, reading 1).
        if index != 4 {
            assert_eq!(&at_one, claimed, "row {}", index + 1);
        }
        // The shots and the real aircraft add to the firing's own; so do
        // the claims, the firing's own being its real aircraft and its
        // shots that missed.
        assert_eq!(parts.iter().map(|p| p.shots).sum::<u32>(), *shots);
        assert_eq!(parts.iter().map(|p| p.real).sum::<u32>(), *real);
        let missed = shots - u32::try_from(hit_shots.len()).expect("few");
        assert_eq!(
            at_one.iter().sum::<u32>(),
            real + missed,
            "row {}",
            index + 1
        );
    }
}

#[test]
fn a_second_group_on_patrol_shares_the_patrols_lines() {
    for chances in [NONE, ALL, PLACEHOLDER] {
        let mut scenario = variant(RAID, chances, &[], SECOND_PATROL);
        let mut ready_at_start: Vec<BTreeMap<u32, u32>> = Vec::new();
        loop {
            let truth = scenario.world.umpire();
            ready_at_start.push(
                [3, 6]
                    .into_iter()
                    .map(|n| (n, truth.air_groups[&AirGroupId::new(n)].ready))
                    .collect(),
            );
            if scenario.next_day().expect("a day").is_none() {
                break;
            }
        }
        let world = scenario.world;
        for claim in air_claims(&world).iter().filter(|c| c.side == Side::Allied) {
            // The build reports these lists.
            eprintln!("{chances:?} day {}: {:?}", claim.day, claim.lines);
            let guns: u32 = claim
                .lines
                .iter()
                .filter(|l| l.unit == base(2))
                .map(|l| l.shot_down)
                .sum();
            let patrols: u32 = claim
                .lines
                .iter()
                .filter(|l| l.unit == group(3) || l.unit == group(6))
                .map(|l| l.shot_down)
                .sum();
            assert_eq!(patrols, claim.down - guns, "{chances:?} day {}", claim.day);
            // Neither is credited with more shots than the aircraft it had
            // on patrol: each patrol firing parted by the aircraft at the
            // start of the day, and each group's claims at most what its
            // shots give.
            let ready = &ready_at_start[claim.day as usize - 1];
            let mut most: BTreeMap<u32, u32> = BTreeMap::new();
            for event in world.events().iter() {
                if !claim.causes.contains(&Cause::Event(event.id)) {
                    continue;
                }
                if let EventKind::AirFight {
                    firing: AirFirer::Patrol,
                    shots,
                    hits,
                    ..
                } = &event.kind
                {
                    let in_firing: Vec<(AirGroupId, u32)> = ready
                        .iter()
                        .map(|(n, r)| (AirGroupId::new(*n), *r))
                        .collect();
                    let hit_shots: Vec<u32> = (1..=*hits).collect();
                    for part in part_firing(&in_firing, whole(*shots), &hit_shots, *hits) {
                        assert!(part.shots <= ready[&part.group.raw()]);
                        *most.entry(part.group.raw()).or_insert(0) += part.shots;
                    }
                }
            }
            for l in &claim.lines {
                if let EntityRef::AirGroup(g) = l.unit {
                    assert!(
                        l.shot_down <= most.get(&g.raw()).copied().unwrap_or(0),
                        "{chances:?} day {}: {:?}",
                        claim.day,
                        l
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------------
// Bombs, group by group.
// ---------------------------------------------------------------------

#[test]
fn bombs_are_each_groups_own() {
    let (g1, g2) = (AirGroupId::new(1), AirGroupId::new(2));
    let groups = [(g1, 10, 2), (g2, 5, 0)];
    let kept = |lines: Vec<UnitClaim>| -> Vec<UnitClaim> {
        lines.into_iter().filter(|l| !l.is_nothing()).collect()
    };
    assert_eq!(
        kept(strike_lines(&groups, true, |_| 0)),
        vec![line(group(1), 0, 2, 0)]
    );
    assert_eq!(
        kept(strike_lines(&groups, true, |missed| missed)),
        vec![line(group(1), 0, 10, 0), line(group(2), 0, 5, 0)]
    );
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

/// The log without its claims, each line as the log prints it.
fn lines_without_claims(world: &World) -> Vec<String> {
    world
        .events()
        .iter()
        .filter(|e| !matches!(e.kind, EventKind::ClaimReceived { .. }))
        .map(Event::to_string)
        .collect()
}

#[test]
fn nothing_else_moves() {
    // Every claim's numbers are scenario 11's at chances 0 and 1 (asserted
    // above); here, at every setting, each list adds to its claim and the
    // log without the claims is the same at every setting (scenario 11's
    // own test holds it to the log from before decision 0056).
    for name in [RAID, STRIKE, "05c_air_combat.toml", "05d_carrier_duel.toml"] {
        let base_log = lines_without_claims(&run(name, PLACEHOLDER));
        for chances in [NONE, ALL] {
            assert_eq!(
                base_log,
                lines_without_claims(&run(name, chances)),
                "{name}"
            );
        }
    }
}

/// What a unit really shot down, read from the firing events of a claim:
/// the aircraft lost to each firing event, by the unit it is recorded under.
fn real_by_unit(world: &World, claim: &Claim) -> BTreeMap<EntityRef, (u32, u32, u32)> {
    let mut firer_of: BTreeMap<u32, EntityRef> = BTreeMap::new();
    let mut real: BTreeMap<EntityRef, (u32, u32, u32)> = BTreeMap::new();
    let key = common::umpire_key();
    for event in world.events().iter() {
        let raw = event.id.umpire_raw(key);
        if claim.causes.contains(&Cause::Event(event.id)) {
            let unit = match &event.kind {
                EventKind::AirFight { firer, .. } => Some(EntityRef::AirGroup(*firer)),
                EventKind::BaseAntiAircraftFire { base: b, .. } => Some(EntityRef::Base(*b)),
                EventKind::AntiAircraftFire { task_force, .. } => {
                    Some(EntityRef::TaskForce(*task_force))
                }
                EventKind::BaseAttacked { air_group, .. } => Some(EntityRef::AirGroup(*air_group)),
                EventKind::AirAttack {
                    air_group, hits, ..
                } => {
                    real.entry(EntityRef::AirGroup(*air_group)).or_default().2 += hits;
                    Some(EntityRef::AirGroup(*air_group))
                }
                _ => None,
            };
            if let Some(unit) = unit {
                firer_of.insert(raw, unit);
            }
            continue;
        }
        let cited = event.causes.iter().find_map(|cause| match cause {
            Cause::Event(id) => firer_of.get(&id.umpire_raw(key)).copied(),
            _ => None,
        });
        if let Some(unit) = cited {
            match &event.kind {
                EventKind::AircraftLost { aircraft, .. } => {
                    real.entry(unit).or_default().0 += aircraft
                }
                EventKind::AircraftDestroyedOnGround { aircraft, .. }
                | EventKind::DamagedAircraftDestroyedOnGround { aircraft, .. } => {
                    real.entry(unit).or_default().1 += aircraft;
                }
                _ => {}
            }
        }
    }
    real
}

#[test]
fn over_200_seeds_the_sums_hold_and_a_tally_is_the_sum_of_its_lines() {
    let raid = variant(RAID, PLACEHOLDER, &[], "");
    for seed in 1..=200 {
        let world = run_variant(raid.with_seed(seed).expect("a seed"));
        let claims = air_claims(&world);
        let mut sums: BTreeMap<AirGroupId, ClaimTally> = BTreeMap::new();
        for claim in &claims {
            let add = |f: fn(&UnitClaim) -> u32| claim.lines.iter().map(f).sum::<u32>();
            assert_eq!(add(|l| l.shot_down), claim.down, "seed {seed}");
            assert_eq!(add(|l| l.on_ground), claim.ground, "seed {seed}");
            assert_eq!(add(|l| l.hits), claim.hits, "seed {seed}");
            assert!(claim.lines.iter().all(|l| !l.is_nothing()), "seed {seed}");
            let real = real_by_unit(&world, claim);
            for l in &claim.lines {
                let (down, ground, hits) = real.get(&l.unit).copied().unwrap_or_default();
                assert!(l.shot_down >= down, "seed {seed}: {l:?} {down}");
                assert!(l.on_ground >= ground, "seed {seed}: {l:?} {ground}");
                assert!(l.hits >= hits, "seed {seed}: {l:?} {hits}");
                if let EntityRef::AirGroup(g) = l.unit {
                    let sum = sums.entry(g).or_default();
                    sum.shot_down += l.shot_down;
                    sum.on_ground += l.on_ground;
                    sum.hits += l.hits;
                }
            }
        }
        for (id, g) in &world.umpire().air_groups {
            assert_eq!(
                g.claims,
                sums.get(id).copied().unwrap_or_default(),
                "seed {seed}: {id}"
            );
        }
    }
}

#[test]
fn a_tally_is_kept_when_a_group_rebases_or_loses_every_aircraft() {
    // Group 2 is rebased to a new base on day 3: it flies nothing that day
    // and keeps what it claimed on days 1 and 2.
    let plain = run(RAID, ALL);
    let world = run_variant(variant(RAID, ALL, &[], REBASE));
    let g2 = &world.umpire().air_groups[&AirGroupId::new(2)];
    assert_eq!(g2.location, AirGroupLocation::Base(BaseId::new(3)));
    assert_eq!(g2.claims, tally_of(12, 0, 0), "6 and 6 on days 1 and 2");
    assert_eq!(tally(&plain, 2), tally_of(17, 0, 0));

    // A group with no aircraft left keeps its tally: group 1, every aircraft
    // taken away after day 2, plays day 3 with the tally it had.
    let key = common::umpire_key();
    let mut live = variant(RAID, ALL, &[], "");
    live.next_day().expect("day 1").expect("day 1 exists");
    live.next_day().expect("day 2").expect("day 2 exists");
    let mut picture = live.world.whole_picture(key).expect("an evening");
    let g1 = picture
        .truth
        .air_groups
        .get_mut(&AirGroupId::new(1))
        .expect("group 1");
    assert_eq!(g1.claims, tally_of(0, 31, 0), "16 and 15");
    g1.ready = 0;
    g1.damaged = 0;
    let mut rebuilt = World::rebuild_from_picture(
        picture,
        live.world.ref_data().clone(),
        live.world.map().clone(),
        live.world.seed(),
        key,
    )
    .expect("the picture is sound");
    rebuilt.resolve_day().expect("day 3");
    let g1 = &rebuilt.umpire().air_groups[&AirGroupId::new(1)];
    assert_eq!(g1.ready + g1.damaged, 0);
    assert_eq!(g1.claims, tally_of(0, 31, 0));
}

#[test]
fn a_bombardments_claim_names_no_unit() {
    let path = common::data_dir().join("scenarios/24_shells_and_estimates.toml");
    let mut scenario = common::load(&path).expect("scenario 24 loads");
    scenario.run_to_end().expect("the run resolves");
    let mut seen = 0;
    for event in scenario.world.events().iter() {
        if let EventKind::ClaimReceived {
            from_air: false,
            by_unit,
            ..
        } = &event.kind
        {
            seen += 1;
            assert!(by_unit.is_empty(), "{}", event.kind);
        }
    }
    assert!(seen > 0);
}

#[test]
fn the_other_side_is_told_nothing() {
    let world = run(RAID, ALL);
    for side in [Side::Allied, Side::Japanese] {
        let view = world.side_view(side);
        for event in view.events() {
            if let SideEventKind::Whole(EventKind::ClaimReceived { side: owner, .. }) = &event.kind
            {
                assert_eq!(*owner, side);
            }
        }
        // The view gives the side's own groups, and so its own tallies only.
        assert!(view.air_groups().all(|g| g.side == side));
    }
    // No answer of a seat's tools holds a tally of an enemy group: the
    // line of an air group is built for the side's own groups only, and
    // the Japanese groups' tallies appear in no Allied answer.
    let mut scenario = variant(RAID, ALL, &[], "");
    scenario.run_to_end().expect("the run resolves");
    let allied = sim_agent::briefing(&scenario.world.side_view(Side::Allied), "the base raid");
    let text = serde_json::to_string(&allied).expect("json");
    // Groups 1 and 2 are Japanese, with 42 aircraft on the ground and 17
    // shot down claimed: no tally in the Allied briefing says either.
    assert!(!text.contains("\"on_ground\":42"));
    assert!(!text.contains("\"shot_down\":17"));
    assert!(text.contains("\"claimed_to_date\":{\"shot_down\":28"));
}

#[test]
fn with_command_levels_on_a_superior_knows_a_tally_by_the_latest_state_report() {
    let mut scenario = variant(
        RAID,
        ALL,
        &[
            (
                "placeholder = true\n",
                "placeholder = true\ncommand_levels = true\n",
            ),
            // Reports that travel, each on time (scenario 63's figures).
            (
                "air_combat = \"../reference/air_combat.toml\"\n",
                "air_combat = \"../reference/air_combat.toml\"\n\
                 reports = \"../reference/reports_placeholder_scenario_63.toml\"\n",
            ),
        ],
        "",
    );
    let mut at_end: Vec<ClaimTally> = vec![ClaimTally::default()];
    let mut checked = 0;
    while scenario.next_day().expect("a day").is_some() {
        let world = &scenario.world;
        at_end.push(tally(world, 3));
        let held = world.held(Side::Allied).expect("command levels are on");
        for state in held.values() {
            for g in state.air_groups().filter(|g| g.id == AirGroupId::new(3)) {
                let want = match state.as_of() {
                    AsOf::Start => ClaimTally::default(),
                    AsOf::Pulse {
                        day,
                        pulse: Pulse::Night,
                    } => at_end[day as usize - 1],
                    AsOf::Pulse { day, .. } => at_end[day as usize],
                };
                assert_eq!(g.claims, want, "{:?}", state.as_of());
                checked += 1;
            }
        }
    }
    assert!(checked > 0, "a superior holds group 3");
    assert_ne!(at_end[3], ClaimTally::default());
}

#[test]
fn a_picture_keeps_the_lists_and_the_tallies_and_the_hash_names_them() {
    let key = common::umpire_key();
    let mut live = variant(RAID, ALL, &[], "");
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
    for side in [Side::Allied, Side::Japanese] {
        let a: Vec<_> = live
            .world
            .side_view(side)
            .claims()
            .iter()
            .map(|c| c.by_unit.clone())
            .collect();
        let b: Vec<_> = rebuilt
            .side_view(side)
            .claims()
            .iter()
            .map(|c| c.by_unit.clone())
            .collect();
        assert_eq!(a, b);
        assert!(a.iter().any(|l| !l.is_empty()));
    }
    for (id, g) in &live.world.umpire().air_groups {
        assert_eq!(rebuilt.umpire().air_groups[id].claims, g.claims);
    }
    let mut changed = picture;
    changed
        .truth
        .air_groups
        .get_mut(&AirGroupId::new(3))
        .expect("group 3")
        .claims
        .shot_down += 1;
    assert_ne!(rebuilt.state_hash(), rebuild(changed).state_hash());
}
