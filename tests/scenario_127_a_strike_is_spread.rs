//! Scenario 127: a strike is spread. Three raids attack a force of three
//! carriers in one pulse. The first draws its carrier among those that do
//! not look lost; each raid after it takes the next.
//!
//! Proves decision 0211 as amended and measured (its section of Nov 22,
//! 2026, which governs). The data is placeholder data. It stands on
//! scenario 5b's hand calculation for the decision to strike and the bomb
//! hits.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything stated is
//! asserted exactly. If the code disagrees, the code or the rule is
//! wrong: do not change a statement here to make a test pass. A mismatch
//! goes back to the architect.
//!
//! ## The setting
//!
//! Task force 1, Allied, at (4,0): three placeholder carriers, ships 1, 2
//! and 3. Aboard ship 1, air group 1, 18 dive bombers; aboard ship 2, air
//! group 2, 18 dive bombers, and air group 3, 12 torpedo bombers; aboard
//! ship 3, air group 4, 18 dive bombers. All on the naval strike mission
//! with the default target list, on which a carrier ranks first.
//!
//! Task force 2, Japanese, stopped at (9,0), 200 nm away: three
//! placeholder carriers, ships 11, 12 and 13, and two cruisers, ships 14
//! and 15. No aircraft, no patrol.
//!
//! The Allied side starts holding a contact of task force 2 at (9,0), age
//! 0, so every group launches in the morning. The game is one day. The
//! live figures, with `strike_is_spread` present.
//!
//! ## The raids
//!
//! A raid is one ship's strike at one hex in a pulse. Three raids attack
//! task force 2 in the morning: ship 1's (group 1), ship 2's (groups 2 and
//! 3), ship 3's (group 4). Their order is that of their first groups: 1,
//! 2, 4. All want carriers.
//!
//! ## The first raid draws
//!
//! None of ships 11, 12 and 13 looks lost at the start of the morning's
//! air combat. The first raid makes one draw, d, from 0 up to but not 1,
//! and takes the ship at place floor(3 x d) in the order 11, 12, 13: under
//! one third ship 11, under two thirds ship 12, otherwise ship 13.
//!
//! ## The raids after it
//!
//! The second raid takes the next ship after the first's in the order 11,
//! 12, 13, going round; the third the next after the second's. So the
//! three raids attack three different carriers, on every seed:
//!
//! | First draws | Raid of ship 1 | Raid of ship 2 | Raid of ship 3 |
//! | --- | --- | --- | --- |
//! | ship 11 | 11 | 12 | 13 |
//! | ship 12 | 12 | 13 | 11 |
//! | ship 13 | 13 | 11 | 12 |
//!
//! Each row happens on some seed of those the test runs.
//!
//! Groups 2 and 3, of one raid, attack the same carrier. No group attacks
//! a cruiser. The later raids make no draw.
//!
//! The first raid's attack records, in its breakdown, the ships it could
//! have drawn (11, 12, 13), the draw and the ship drawn; each later
//! raid's, the ship it took by turn and the ship the raid before it took.
//!
//! Every other draw of the pulse, the fighters', the guns' and each
//! group's shots, is the draw the build without this rule makes for that
//! group and force: the test asserts each group's shots and hits are
//! those of the same seed without the rule, where the ship attacked is of
//! the same class. (The three carriers are of one class, so they are.)
//!
//! # Properties
//!
//! - **One looks lost.** With ship 12 starting with her flight deck out:
//!   the first raid draws between ships 11 and 13, under one half ship
//!   11; the second takes the other; the third goes round to the first
//!   raid's ship. Ship 12 is not attacked on any seed.
//! - **Crippled and burning.** The same with ship 12 starting with
//!   flotation damage at `crippled_flotation`, and with fire damage at
//!   `claimed_sunk_fire`, each in place of the deck.
//! - **All look lost.** With all three decks out: the draw is among the
//!   three, and the raids go round as in the table.
//! - **One afloat.** With ships 12 and 13 removed: every raid attacks ship
//!   11; the first still makes its draw, among one.
//! - **Another kind.** With group 4's target list set to rank cruisers
//!   first: the raid of ship 3 takes its own turn among the cruisers, and
//!   being the first that wants cruisers it draws between ships 14 and
//!   15; the raids of ships 1 and 2 are the first and second of those
//!   that want carriers.
//! - **A raid of two kinds.** With group 3's list set to rank cruisers
//!   first and group 2's unchanged: the raid of ship 2 takes its turn
//!   among carriers by its first group, group 2; group 3 attacks the
//!   lowest-numbered cruiser that does not look lost, as a group chooses
//!   today, and takes no turn.
//! - **Start of the step.** With a seed on which the first raid sinks its
//!   carrier: the third raid, going round, is not in question here (three
//!   carriers, three raids); with a fourth raid added (a fourth Allied
//!   carrier, ship 4, group 5) the fourth raid goes round to the first
//!   raid's carrier, sunk or not, the choice being read at the start of
//!   the step.
//! - **Off where the switch is absent.** On the frozen figures every
//!   group attacks ship 11, as before this rule, and every event of the
//!   frozen strike scenarios is byte for byte what it was.
//! - The same seed gives the same game; two worlds that differ only in
//!   what the Allied side cannot see of ships not in task force 2 give
//!   the same choice.

use crate::common;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::ids::ShipId;
use sim_core::time::Pulse;
use sim_core::world::SimConfig;
use sim_data::{LoadOptions, Scenario};

const SEEDS: u64 = 1000;
const CARRIERS: [u32; 3] = [11, 12, 13];

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/127_a_strike_is_spread.toml")
}

fn text() -> String {
    fs::read_to_string(scenario_path()).expect("read the scenario")
}

// ---------------------------------------------------------------------
// The figures. The scenario runs on the live figures, which carry
// `strike_is_spread`. The build without the rule is a copy of the live
// figures with that one line taken out, in a folder of its own under the
// build directory, as `common::altered_reference` makes one of the frozen
// figures. Nothing in the data folder is touched.
// ---------------------------------------------------------------------

/// A reference root that is the live figures (`data/reference`, its files and
/// not its frozen folder) with changes made by `alter`.
fn live_copy(alter: impl FnOnce(&Path)) -> LoadOptions {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "reference_live_{}_{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&root).expect("make the reference folder");
    for entry in fs::read_dir(common::data_dir().join("reference")).expect("the live folder reads")
    {
        let entry = entry.expect("an entry reads");
        let name = entry.file_name();
        if entry.path().is_dir() || name.to_string_lossy().ends_with(".md") {
            continue;
        }
        fs::copy(entry.path(), root.join(&name)).expect("copy a live file");
    }
    alter(&root);
    LoadOptions::with_reference_root(root)
}

/// The live figures: with the switch as they have it or taken out (the build
/// without this rule), and with a placeholder torpedo bomber added to the
/// aircraft types as type 5 or not.
fn live_figures(switch: bool, torpedo_bomber: bool) -> LoadOptions {
    live_copy(|dir| {
        if !switch {
            common::edit_file(dir, "air_combat.toml", |text| {
                common::replace_once(&text, "strike_is_spread = true\n", "")
            });
        }
        if torpedo_bomber {
            common::edit_file(dir, "aircraft_types.csv", |mut text| {
                if !text.ends_with('\n') {
                    text.push('\n');
                }
                text.push_str("5,Placeholder torpedo bomber,bomber,0,0,250,torpedo,60,0,0\n");
                text
            });
        }
    })
}

fn build(options: &LoadOptions, change: impl FnOnce(String) -> String) -> Scenario {
    common::from_text_with(&change(text()), &scenario_path(), options).expect("the variant loads")
}

/// The scenario on the live figures, which hold the switch. The assertions
/// made on it are choices, not numbers that depend on a figure.
fn live(change: impl FnOnce(String) -> String) -> Scenario {
    build(&LoadOptions::default(), change)
}

fn torpedo_variant(text: String) -> String {
    common::replace_once(
        &text,
        "name = \"Placeholder strike squadron 3\"\nside = \"allied\"\nship = 2\naircraft_type = 3",
        "name = \"Placeholder strike squadron 3\"\nside = \"allied\"\nship = 2\naircraft_type = 5",
    )
}

// ---------------------------------------------------------------------
// What one run did.
// ---------------------------------------------------------------------

struct Attack {
    group: u32,
    ship: u32,
    shots: Fixed,
    chance: Fixed,
    expected: Fixed,
    hits: u32,
    factors: Vec<(&'static str, Fixed)>,
}

impl Attack {
    fn factor(&self, name: &str) -> Vec<Fixed> {
        self.factors
            .iter()
            .filter(|(factor, _)| *factor == name)
            .map(|(_, value)| *value)
            .collect()
    }

    fn spread_factors(&self) -> Vec<&'static str> {
        self.factors
            .iter()
            .map(|(name, _)| *name)
            .filter(|name| name.starts_with("spread_"))
            .collect()
    }

    /// What `without` the rule must show of the group's attack: the same
    /// shots, chance, expected hits and hits.
    fn same_shots(&self, other: &Attack) -> bool {
        (self.shots, self.chance, self.expected, self.hits)
            == (other.shots, other.chance, other.expected, other.hits)
    }
}

struct Run {
    seed: u64,
    events: Vec<Event>,
    /// The ships sunk in the morning (after its attacks are made: a raid's
    /// damage is applied when the step's attacks have been drawn).
    sunk_in_the_morning: BTreeSet<u32>,
    /// The attacks of the morning, in the order of the log.
    morning: Vec<Attack>,
}

fn run_of(scenario: &Scenario, seed: u64) -> Run {
    let mut run = scenario.with_seed(seed).expect("a seed can be set");
    run.world.set_config(SimConfig {
        record_breakdowns: true,
    });
    run.run_to_end().expect("the run resolves");
    let events: Vec<Event> = run.world.events().iter().cloned().collect();
    let mut morning = Vec::new();
    let mut sunk_in_the_morning = BTreeSet::new();
    for event in &events {
        match event.kind {
            EventKind::ShipSunk { ship, .. } => {
                if event.pulse == Some(Pulse::Morning) {
                    sunk_in_the_morning.insert(ship.raw());
                }
            }
            EventKind::AirAttack {
                air_group,
                ship,
                shots,
                chance,
                expected,
                hits,
                ..
            } if event.pulse == Some(Pulse::Morning) => morning.push(Attack {
                group: air_group.raw(),
                ship: ship.raw(),
                shots,
                chance,
                expected,
                hits,
                factors: event
                    .breakdown
                    .as_ref()
                    .map(|breakdown| {
                        breakdown
                            .factors
                            .iter()
                            .map(|factor| (factor.name, factor.value))
                            .collect()
                    })
                    .unwrap_or_default(),
            }),
            _ => {}
        }
    }
    Run {
        seed,
        events,
        sunk_in_the_morning,
        morning,
    }
}

impl Run {
    fn attack_of(&self, group: u32) -> &Attack {
        let found: Vec<&Attack> = self
            .morning
            .iter()
            .filter(|attack| attack.group == group)
            .collect();
        assert_eq!(
            found.len(),
            1,
            "seed {}: group {group} attacks once",
            self.seed
        );
        found[0]
    }

    /// The ship each group attacked in the morning.
    fn ships(&self) -> BTreeMap<u32, u32> {
        self.morning
            .iter()
            .map(|attack| (attack.group, attack.ship))
            .collect()
    }
}

fn number(ship: u32) -> Fixed {
    Fixed::from_int(i64::from(ship))
}

/// The place `floor(n x d)` a draw `d` takes among `n`.
fn place(draw: Fixed, n: usize) -> usize {
    let n = i64::try_from(n).expect("few");
    usize::try_from(n * draw.raw() / Fixed::SCALE).expect("a place")
}

/// The next ship after `after` in `pool`, going round.
fn next_in(pool: &[u32], after: u32) -> u32 {
    pool.iter()
        .copied()
        .find(|ship| *ship > after)
        .unwrap_or(pool[0])
}

/// Asserts the choices of the raids that want one kind of ship, in order,
/// each given as the groups that make it (the first of them is the raid's
/// first group), over `pool` (the ships of the kind that do not look lost, or
/// all of it where every one does, by ship number). `after` is the ship the
/// raid before them took, none for the first of the kind. Returns the ship
/// each raid took.
fn check_raids(run: &Run, pool: &[u32], raids: &[&[u32]]) -> Vec<u32> {
    let mut taken: Vec<u32> = Vec::new();
    for raid in raids {
        let first = run.attack_of(raid[0]);
        let ship = match taken.last() {
            None => {
                // The first raid draws: each ship it could have drawn, the
                // draw and the ship drawn are in the breakdown.
                let could: Vec<Fixed> = pool.iter().map(|ship| number(*ship)).collect();
                assert_eq!(
                    first.factor("spread_could_draw"),
                    could,
                    "seed {}",
                    run.seed
                );
                let draw = first.factor("spread_draw");
                assert_eq!(draw.len(), 1, "seed {}", run.seed);
                assert!(draw[0] >= Fixed::ZERO && draw[0] < Fixed::ONE);
                let ship = pool[place(draw[0], pool.len())];
                assert_eq!(first.factor("spread_ship_drawn"), vec![number(ship)]);
                assert!(first.factor("spread_ship_by_turn").is_empty());
                assert!(first.factor("spread_after_ship").is_empty());
                ship
            }
            Some(after) => {
                // A later raid takes the next ship, going round, and makes
                // no draw.
                let ship = next_in(pool, *after);
                assert_eq!(first.factor("spread_ship_by_turn"), vec![number(ship)]);
                assert_eq!(first.factor("spread_after_ship"), vec![number(*after)]);
                assert!(first.factor("spread_could_draw").is_empty());
                assert!(first.factor("spread_draw").is_empty());
                assert!(first.factor("spread_ship_drawn").is_empty());
                ship
            }
        };
        // Every group of the raid attacks the raid's ship, with the same
        // record of how it was chosen.
        for group in *raid {
            let attack = run.attack_of(*group);
            assert_eq!(attack.ship, ship, "seed {} group {group}", run.seed);
            assert_eq!(attack.spread_factors(), first.spread_factors());
            assert_eq!(
                attack
                    .factors
                    .iter()
                    .filter(|(n, _)| n.starts_with("spread_"))
                    .count(),
                first
                    .factors
                    .iter()
                    .filter(|(n, _)| n.starts_with("spread_"))
                    .count()
            );
        }
        taken.push(ship);
    }
    taken
}

/// What the build without the rule gives for the same seed: each group's
/// shots, chance, expected hits and hits are the same, at a ship of the same
/// class (every ship it can attack here is a carrier).
fn same_as_without(with: &Run, without: &Run) {
    assert_eq!(
        with.morning.len(),
        without.morning.len(),
        "seed {}",
        with.seed
    );
    for attack in &with.morning {
        let other = without.attack_of(attack.group);
        assert!(
            attack.same_shots(other),
            "seed {} group {}: with the rule {:?} shots, {} hits; without, {:?} shots, {} hits",
            with.seed,
            attack.group,
            attack.shots,
            attack.hits,
            other.shots,
            other.hits
        );
        assert!(CARRIERS.contains(&attack.ship) && CARRIERS.contains(&other.ship));
        assert!(other.spread_factors().is_empty());
    }
}

// ---------------------------------------------------------------------
// The setting and the raids.
// ---------------------------------------------------------------------

#[test]
fn the_setting_is_as_the_hand_calculation_says() {
    // The torpedo bombers of group 3 are a copy of the live types with one
    // added: the live file has none (see the scenario file's head).
    for scenario in [
        live(|text| text),
        build(&live_figures(true, true), torpedo_variant),
    ] {
        let truth = scenario.world.umpire();
        assert_eq!(scenario.days, 1);
        let ships = |force: u32| -> Vec<u32> {
            truth.task_forces[&sim_core::ids::TaskForceId::new(force)]
                .ships
                .iter()
                .map(|ship| ship.raw())
                .collect()
        };
        assert_eq!(ships(1), vec![1, 2, 3]);
        assert_eq!(ships(2), vec![11, 12, 13, 14, 15]);
        let group = |id: u32| &truth.air_groups[&sim_core::ids::AirGroupId::new(id)];
        let aboard = |id: u32| match group(id).location {
            sim_core::state::AirGroupLocation::Ship(ship) => ship.raw(),
            other => panic!("group {id} is at {other:?}"),
        };
        assert_eq!(
            [1, 2, 3, 4].map(|id| (aboard(id), group(id).ready)),
            [(1, 18), (2, 18), (2, 12), (3, 18)]
        );
        assert!(truth
            .air_groups
            .values()
            .all(|group| group.side == sim_core::state::Side::Allied));
        let air = scenario.world.ref_data().air_combat.as_ref().expect("held");
        assert!(air.strike_is_spread, "the live figures carry the switch");
        // The Allied side holds one contact, at (9,0), and every group is
        // allowed to strike on it in the morning.
        let allied = scenario.world.side_view(sim_core::state::Side::Allied);
        assert_eq!(allied.contacts().count(), 1);
    }
}

/// The checks of "The raids": the choices of three raids, on every seed.
fn check_the_table(scenario: &Scenario, without: &Scenario) -> BTreeSet<u32> {
    let mut first_ships = BTreeSet::new();
    for seed in 1..=SEEDS {
        let run = run_of(scenario, seed);
        let reference = run_of(without, seed);
        assert_eq!(
            run.morning.len(),
            4,
            "seed {seed}: four groups launch in the morning"
        );
        let taken = check_raids(&run, &CARRIERS, &[&[1], &[2, 3], &[4]]);
        // Three different carriers, on every seed, in the order of the table.
        let first = taken[0];
        assert_eq!(
            taken,
            vec![
                first,
                next_in(&CARRIERS, first),
                next_in(&CARRIERS, next_in(&CARRIERS, first))
            ]
        );
        assert_eq!(taken.iter().collect::<BTreeSet<_>>().len(), 3);
        first_ships.insert(first);
        // The table, row by row.
        let row = match first {
            11 => (11, 12, 13),
            12 => (12, 13, 11),
            _ => (13, 11, 12),
        };
        assert_eq!(
            (
                run.attack_of(1).ship,
                run.attack_of(2).ship,
                run.attack_of(4).ship
            ),
            row
        );
        // Groups 2 and 3 attack the same carrier, and no group a cruiser.
        assert_eq!(run.attack_of(2).ship, run.attack_of(3).ship);
        assert!(run.morning.iter().all(|attack| attack.ship <= 13));
        // The later raids make no draw: the one draw is the first raid's.
        assert_eq!(
            run.morning
                .iter()
                .filter(|attack| !attack.factor("spread_draw").is_empty())
                .map(|attack| attack.group)
                .collect::<Vec<_>>(),
            vec![1]
        );
        // Everything else is what the build without the rule makes.
        same_as_without(&run, &reference);
    }
    first_ships
}

#[test]
fn three_raids_take_three_different_carriers_in_the_order_of_the_table_on_every_seed() {
    let without = build(&live_figures(false, false), |text| text);
    let first_ships = check_the_table(&live(|text| text), &without);
    // Each row happens on some seed.
    assert_eq!(first_ships, CARRIERS.into_iter().collect::<BTreeSet<_>>());
}

#[test]
fn the_same_holds_with_torpedo_bombers_in_group_3() {
    let without = build(&live_figures(false, true), torpedo_variant);
    let scenario = build(&live_figures(true, true), torpedo_variant);
    let first_ships = check_the_table(&scenario, &without);
    assert_eq!(first_ships, CARRIERS.into_iter().collect::<BTreeSet<_>>());
    // Group 3's attack is a torpedo attack, at the carrier of its raid.
    let run = run_of(&scenario, 1);
    let torpedo = run.events.iter().any(|event| {
        matches!(
            event.kind,
            EventKind::AirAttack { air_group, weapon: sim_core::events::Weapon::Torpedo, .. }
                if air_group.raw() == 3
        )
    });
    assert!(torpedo, "group 3 attacks with torpedoes");
}

// ---------------------------------------------------------------------
// The variants: ships that look lost.
// ---------------------------------------------------------------------

/// A line added to the block of the ship named `name` (a name that appears
/// once in the text).
fn ship_gets(text: String, name: &str, line: &str) -> String {
    let heading = format!("name = \"{name}\"\n");
    common::replace_once(&text, &heading, &format!("{heading}{line}\n"))
}

const DECK_OUT: &str = "flight_deck = \"out\"";

/// Runs `scenario` on every seed and checks the three carrier raids over
/// `pool`, which no ship outside it is attacked by. Returns the first
/// ships taken and the ships taken by the second raid and the third, by
/// the first ship.
fn check_pool(scenario: &Scenario, pool: &[u32]) -> BTreeMap<u32, (u32, u32)> {
    let mut rows = BTreeMap::new();
    for seed in 1..=SEEDS {
        let run = run_of(scenario, seed);
        assert_eq!(run.morning.len(), 4, "seed {seed}");
        let taken = check_raids(&run, pool, &[&[1], &[2, 3], &[4]]);
        for attack in &run.morning {
            assert!(
                pool.contains(&attack.ship),
                "seed {seed}: group {} attacked ship {}, outside {pool:?}",
                attack.group,
                attack.ship
            );
        }
        rows.insert(taken[0], (taken[1], taken[2]));
    }
    rows
}

#[test]
fn one_looks_lost_with_her_flight_deck_out_she_is_not_attacked_and_the_others_take_turns() {
    let scenario = live(|text| ship_gets(text, "Placeholder carrier B2", DECK_OUT));
    assert_eq!(
        scenario.world.umpire().ships[&ShipId::new(12)]
            .systems
            .flight_deck,
        sim_core::state::SystemState::Out
    );
    // Under one half ship 11, otherwise ship 13; the second takes the other;
    // the third goes round to the first raid's ship.
    let rows = check_pool(&scenario, &[11, 13]);
    assert_eq!(rows, BTreeMap::from([(11, (13, 11)), (13, (11, 13))]));
}

#[test]
fn one_looks_lost_crippled_or_burning_she_is_not_attacked_either() {
    // Flotation damage at crippled_flotation, and fire damage at
    // claimed_sunk_fire, each in place of the deck.
    let base = live(|text| text);
    let surface = base.world.ref_data().surface_combat.clone();
    for (line, label) in [
        (
            format!("flotation_damage = \"{}\"", surface.crippled_flotation),
            "crippled",
        ),
        (
            format!("fire_damage = \"{}\"", surface.claimed_sunk_fire),
            "burning",
        ),
    ] {
        let scenario = live(|text| ship_gets(text, "Placeholder carrier B2", &line));
        let rows = check_pool(&scenario, &[11, 13]);
        assert_eq!(
            rows,
            BTreeMap::from([(11, (13, 11)), (13, (11, 13))]),
            "{label}"
        );
    }
}

#[test]
fn all_look_lost_the_draw_is_among_the_three_and_the_raids_go_round() {
    let scenario = live(|text| {
        let text = ship_gets(text, "Placeholder carrier B1", DECK_OUT);
        let text = ship_gets(text, "Placeholder carrier B2", DECK_OUT);
        ship_gets(text, "Placeholder carrier B3", DECK_OUT)
    });
    let rows = check_pool(&scenario, &CARRIERS);
    assert_eq!(
        rows,
        BTreeMap::from([(11, (12, 13)), (12, (13, 11)), (13, (11, 12))])
    );
}

#[test]
fn one_afloat_every_raid_attacks_her_and_the_first_still_makes_its_draw() {
    let gone = |id: u32, name: &str| {
        format!(
            "[[ship]]\nid = {id}\nname = \"{name}\"\nclass = 4\nside = \"japanese\"\nfuel_t = 3000\ntask_force = 2\n\n"
        )
    };
    let scenario = live(|text| {
        let text = common::replace_once(&text, &gone(12, "Placeholder carrier B2"), "");
        common::replace_once(&text, &gone(13, "Placeholder carrier B3"), "")
    });
    let rows = check_pool(&scenario, &[11]);
    assert_eq!(rows, BTreeMap::from([(11, (11, 11))]));
    // The first raid's breakdown holds the draw among one.
    let run = run_of(&scenario, 1);
    let first = run.attack_of(1);
    assert_eq!(first.factor("spread_could_draw"), vec![number(11)]);
    assert_eq!(first.factor("spread_draw").len(), 1);
}

// ---------------------------------------------------------------------
// The variants: raids that want another kind.
// ---------------------------------------------------------------------

fn target_cruisers(text: String, group: u32) -> String {
    let mut text = text;
    text.push_str(&format!(
        "
[[order]]
day = 1
side = \"allied\"
air_group = {group}
kind = \"set_target_priority\"
kinds = [\"cruiser\", \"carrier\"]
"
    ));
    text
}

#[test]
fn another_kind_takes_its_own_turn_among_the_cruisers() {
    let scenario = live(|text| target_cruisers(text, 4));
    let mut cruisers_taken = BTreeSet::new();
    let mut rows = BTreeSet::new();
    for seed in 1..=SEEDS {
        let run = run_of(&scenario, seed);
        // The raid of ship 3, first of those that want cruisers, draws
        // between ships 14 and 15.
        let cruiser = check_raids(&run, &[14, 15], &[&[4]]);
        cruisers_taken.insert(cruiser[0]);
        // The raids of ships 1 and 2 are the first and second of those that
        // want carriers.
        let carriers = check_raids(&run, &CARRIERS, &[&[1], &[2, 3]]);
        rows.insert((carriers[0], carriers[1]));
        assert!(CARRIERS.contains(&run.attack_of(1).ship));
        assert!(CARRIERS.contains(&run.attack_of(2).ship));
        assert!(CARRIERS.contains(&run.attack_of(3).ship));
    }
    assert_eq!(cruisers_taken, BTreeSet::from([14, 15]));
    assert_eq!(rows, BTreeSet::from([(11, 12), (12, 13), (13, 11)]));
}

#[test]
fn a_raid_of_two_kinds_takes_its_turn_by_its_first_group_and_the_other_takes_none() {
    // Group 3's list ranks cruisers first, group 2's is unchanged.
    let scenario = live(|text| target_cruisers(text, 3));
    let mut rows = BTreeSet::new();
    for seed in 1..=SEEDS {
        let run = run_of(&scenario, seed);
        // The raid of ship 2 takes its turn among the carriers by its first
        // group, group 2; the raid of ship 3 follows it.
        let taken = check_raids(&run, &CARRIERS, &[&[1], &[2], &[4]]);
        rows.insert((taken[0], taken[1], taken[2]));
        // Group 3 attacks the lowest-numbered cruiser that does not look
        // lost, as a group chooses today, and takes no turn.
        let attack = run.attack_of(3);
        assert_eq!(attack.ship, 14, "seed {seed}");
        assert!(attack.spread_factors().is_empty(), "seed {seed}");
        // Its raid is not asked of the cruisers' pool: no draw among them.
        assert!(attack.factor("spread_could_draw").is_empty());
    }
    assert_eq!(
        rows,
        BTreeSet::from([(11, 12, 13), (12, 13, 11), (13, 11, 12)])
    );
}

// ---------------------------------------------------------------------
// The start of the step.
// ---------------------------------------------------------------------

#[test]
fn a_fourth_raid_goes_round_to_the_first_raids_carrier_sunk_or_not() {
    // A fourth Allied carrier, ship 4, with group 5 aboard, ordered as the
    // others; and the three Japanese carriers start with flotation damage
    // of 85 so that two hits on it sink a carrier. Each looks lost: the
    // first raid draws among the three, and the raids go round.
    let scenario = live(|text| {
        let mut text = text;
        for name in ["B1", "B2", "B3"] {
            text = ship_gets(
                text,
                &format!("Placeholder carrier {name}"),
                "flotation_damage = 85",
            );
        }
        text.push_str(
            "
[[ship]]
id = 4
name = \"Placeholder carrier A4\"
class = 4
side = \"allied\"
fuel_t = 3000
task_force = 1

[[air_group]]
id = 5
name = \"Placeholder bombing squadron 5\"
side = \"allied\"
ship = 4
aircraft_type = 3
ready = 18

[[order]]
day = 1
side = \"allied\"
air_group = 5
kind = \"set_air_mission\"
mission = \"naval_strike\"

[[order]]
day = 1
side = \"allied\"
air_group = 5
kind = \"set_strike_contact_age\"
pulses = 1
",
        );
        text
    });
    let (mut sunk_first, mut seen_first) = (0, BTreeSet::new());
    for seed in 1..=SEEDS {
        let run = run_of(&scenario, seed);
        assert_eq!(run.morning.len(), 5, "seed {seed}");
        let taken = check_raids(&run, &CARRIERS, &[&[1], &[2, 3], &[4], &[5]]);
        // Three carriers, four raids: the fourth is the first's.
        assert_eq!(taken[3], taken[0], "seed {seed}");
        assert_eq!(run.attack_of(5).ship, run.attack_of(1).ship);
        seen_first.insert(taken[0]);
        // The choice is read at the start of the step: the fourth raid is at
        // her whether or not an earlier raid has sunk her by then.
        if run.sunk_in_the_morning.contains(&taken[0]) {
            sunk_first += 1;
        }
    }
    assert!(
        sunk_first > 0,
        "the first raid's carrier was sunk before the fourth raid on some seed"
    );
    assert_eq!(seen_first, CARRIERS.into_iter().collect::<BTreeSet<_>>());
}

// ---------------------------------------------------------------------
// Off where the switch is absent.
// ---------------------------------------------------------------------

#[test]
fn on_the_frozen_figures_every_group_attacks_ship_11_as_before_the_rule() {
    let frozen = build(&common::frozen(), |text| text);
    assert!(
        !frozen
            .world
            .ref_data()
            .air_combat
            .as_ref()
            .expect("held")
            .strike_is_spread
    );
    for seed in 1..=SEEDS {
        let run = run_of(&frozen, seed);
        assert_eq!(run.morning.len(), 4);
        for attack in &run.morning {
            assert_eq!(attack.ship, 11, "seed {seed} group {}", attack.group);
            assert!(attack.spread_factors().is_empty());
        }
        assert_eq!(run.attack_of(1).ship, 11);
    }
}

#[test]
fn on_the_live_figures_with_the_switch_taken_out_the_groups_attack_as_they_did() {
    let without = build(&live_figures(false, false), |text| text);
    for seed in 1..=300 {
        let run = run_of(&without, seed);
        for attack in &run.morning {
            assert_eq!(attack.ship, 11);
            assert!(attack.spread_factors().is_empty());
        }
    }
}

/// A digest of every event of a scenario's runs on the frozen figures, for
/// seeds 1 to 100: FNV-1a over the debug text of each event, ids, causes
/// and all, in order.
fn digest_of_frozen_runs(name: &str) -> (u64, usize) {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../data/scenarios/{name}.toml"));
    let base = common::load(&path).expect("the scenario loads on the frozen figures");
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    let mut count = 0;
    for seed in 1..=100u64 {
        let mut run = base.with_seed(seed).expect("a seed can be set");
        run.run_to_end().expect("the run resolves");
        for event in run.world.events().iter() {
            for byte in format!("{event:?}").bytes() {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
            }
            count += 1;
        }
    }
    (hash, count)
}

#[test]
fn on_the_frozen_figures_every_event_of_the_frozen_strike_scenarios_is_what_it_was() {
    // Digests taken from the build without the rules of decisions 0211 and
    // 0212 (the commit before their engine change, 9fc04a5c), the same on
    // the build with them: scenarios 5c, 5d and 13a, which strike ships,
    // over seeds 1 to 100 (scenario 5b's is in scenario 126's test).
    // Remade for decision 0218, stage 2: every claim's record gains
    // `from_submarine`, false here; with it taken out each digest is the one
    // before (5c 0x3461_0bac_438e_b822, 5d 0xa52b_1ec2_391e_ccda; 13a has no
    // claim and did not move).
    assert_eq!(
        digest_of_frozen_runs("05c_air_combat"),
        (0xdef5_44eb_0a67_4d72, 2576)
    );
    assert_eq!(
        digest_of_frozen_runs("05d_carrier_duel"),
        (0x3575_c47f_9867_9f3a, 7191)
    );
    assert_eq!(
        digest_of_frozen_runs("13a_target_priority"),
        (0x9bf7_94ee_684f_0cf5, 2400)
    );
}

// ---------------------------------------------------------------------
// The same seed, and what the Allied side cannot see.
// ---------------------------------------------------------------------

#[test]
fn the_same_seed_gives_the_same_game_and_unseen_ships_change_no_choice() {
    let scenario = live(|text| text);
    // Another Japanese force, far from task force 2 and seen by nobody,
    // with a carrier and a cruiser: ships not in task force 2.
    let with_more = live(|mut text| {
        text.push_str(
            "
[[task_force]]
id = 3
name = \"Placeholder unseen force\"
side = \"japanese\"
q = 9
r = -3
mission = \"carrier\"

[[ship]]
id = 16
name = \"Placeholder carrier B6\"
class = 4
side = \"japanese\"
fuel_t = 3000
task_force = 3

[[ship]]
id = 17
name = \"Placeholder cruiser B7\"
class = 1
side = \"japanese\"
fuel_t = 2000
task_force = 3
",
        );
        text
    });
    for seed in 1..=300 {
        let run = run_of(&scenario, seed);
        let again = run_of(&scenario, seed);
        assert_eq!(run.events, again.events, "seed {seed}");
        let other = run_of(&with_more, seed);
        assert_eq!(run.ships(), other.ships(), "seed {seed}");
        for (a, b) in run.morning.iter().zip(&other.morning) {
            assert_eq!(a.group, b.group);
            let spread = |attack: &Attack| -> Vec<(&'static str, Fixed)> {
                attack
                    .factors
                    .iter()
                    .filter(|(name, _)| name.starts_with("spread_"))
                    .copied()
                    .collect()
            };
            assert_eq!(spread(a), spread(b), "seed {seed}");
        }
    }
}
