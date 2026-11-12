//! Scenario 14a: experienced crews against green ones, at night.
//!
//! Stage 1 of decision 0064 (`docs/decisions/0064-experience.md`): a ship has
//! an experience value, 0 to 4, and three chances in a dark surface fight read
//! it. This is scenario 4a exactly (`data/scenarios/04a_night_action.toml`),
//! with `experience = 3` on force A's three ships and `experience = 1` on
//! force B's three (`data/scenarios/14a_experience.toml`). Everything not said
//! here is scenario 4a's hand calculation: `tests/scenario_04a_night_action.rs`.
//!
//! The strengths of experience are 0 in the frozen figures (step 1 had no such
//! rule). The working value of all three here is 0.4, set through an altered
//! copy of the frozen figures (`common::altered_reference`), as scenario 12a
//! sets its figures. The test has the three parts the spec asks for
//! (`docs/step2-spec.md`, "Tests while figures move", 7): the chances at the
//! working value, read from the events' breakdowns; properties over many
//! seeds; and a pinned outcome for one seed on the live figures.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! Inputs, as in scenario 4a: three placeholder cruisers a side, 8 guns at 3
//! rounds a gun a minute, 8 tubes; a round of 5 minutes, so a gun salvo is
//! 8 x 3 x 5 = 120 shots and a torpedo salvo 8; force A at battle readiness,
//! force B at normal readiness and not alert; a dark fight; night chances 0.03
//! for a gun shot and 0.04 for a torpedo; a surprised target 1.5; a lit
//! target 2. The working value of each strength is 0.4.
//!
//! ## The factors
//!
//! `1 + strength x (experience - 2)`. A: 1 + 0.4 x (3 - 2) = 1.4. B:
//! 1 + 0.4 x (1 - 2) = 0.6. Each force's mean experience is its ships' own
//! value, 3 and 1.
//!
//! ## Sighting, a round
//!
//! A: 0.5 x 1 x 1.4 = 0.7. B: 0.5 x 0.5 x 0.6 = 0.15. (Scenario 4a: 0.5 and
//! 0.25.)
//!
//! ## How a fight opens, each round until a force sights
//!
//! | Round                | Chance                |
//! | -------------------- | --------------------- |
//! | A alone sights       | 0.7 x 0.85 = 0.595    |
//! | B alone sights       | 0.3 x 0.15 = 0.045    |
//! | Both sight           | 0.7 x 0.15 = 0.105    |
//! | Neither sights       | 0.3 x 0.85 = 0.255    |
//!
//! They sum to 1. Over the six rounds, neither ever sights with 0.255^6 =
//! 0.000275. The fight opens in one of three ways, in the proportions 0.595 :
//! 0.045 : 0.105 out of 0.745:
//!
//! | How the fight opens      | Chance                                |
//! | ------------------------ | ------------------------------------- |
//! | A has the free round     | 0.595 x (1 - 0.000275) / 0.745 = 0.798438 |
//! | B has the free round     | 0.045 x 0.999725 / 0.745 = 0.060386   |
//! | Both fire from the start | 0.105 x 0.999725 / 0.745 = 0.140901   |
//! | They pass unseen         | 0.000275                              |
//!
//! The first three sum to 1 less 0.000275 (the sum of the four is 1).
//! (Scenario 4a: 0.598331, 0.199444, 0.199444, 0.002781.)
//!
//! ## Detection rates over 1,000 runs
//!
//! By scenario 4a's counting rule, a count H with mean E is allowed when
//! (H - E)^2 <= 16 x the sum of np(1 - p), that is within four standard
//! deviations. Each band is worked to the whole runs the rule allows: H is an
//! integer, so H runs from the least whole number at or above E - 4 sigma to
//! the greatest at or below E + 4 sigma.
//!
//! | Count of runs where       | p        | E      | 4 sigma | E - 4 sigma to E + 4 sigma | Allowed    |
//! | ------------------------- | -------- | ------ | ------- | -------------------------- | ---------- |
//! | A sights in round 1       | 0.7      | 700    | 57.97   | 642.03 to 757.97           | 643 to 757 |
//! | B sights in round 1       | 0.15     | 150    | 45.17   | 104.83 to 195.17           | 105 to 195 |
//! | A has the free round      | 0.798438 | 798.44 | 50.75   | 747.69 to 849.19           | 748 to 849 |
//! | B has the free round      | 0.060386 | 60.39  | 30.13   | 30.26 to 90.52             | 31 to 90   |
//! | Both fire from the start  | 0.140901 | 140.90 | 44.00   | 96.90 to 184.90            | 97 to 184  |
//!
//! (4 sigma is 4 x the square root of 1000 x p x (1 - p): 4 x sqrt(210) =
//! 57.97; 4 x sqrt(127.5) = 45.17; 4 x sqrt(160.93) = 50.75; 4 x
//! sqrt(56.74) = 30.13; 4 x sqrt(121.05) = 44.00.) The count of runs that pass
//! unseen has p = 0.000275, E = 0.27, 4 sigma = 2.1: 0 to 2. The four ways add
//! to 1000 runs exactly.
//!
//! ## A gun salvo of 120 shots
//!
//! The chance is `0.03 x surprised x lit x fire control x experience`, the
//! experience factor last.
//!
//! | Firing    | Target               | Chance                | Hits expected |
//! | --------- | -------------------- | --------------------- | ------------- |
//! | A (x 1.4) | not surprised, unlit | 0.03 x 1.4 = 0.042    | 5.04          |
//! | A         | surprised            | 0.03 x 1.5 x 1.4 = 0.063 | 7.56       |
//! | A         | lit                  | 0.03 x 2 x 1.4 = 0.084   | 10.08      |
//! | A         | surprised and lit    | 0.03 x 1.5 x 2 x 1.4 = 0.126 | 15.12  |
//! | B (x 0.6) | not surprised, unlit | 0.018                 | 2.16          |
//! | B         | surprised            | 0.027                 | 3.24          |
//! | B         | lit                  | 0.036                 | 4.32          |
//! | B         | surprised and lit    | 0.054                 | 6.48          |
//!
//! (Scenario 4a: 0.03, 3.6; 0.045, 5.4; 0.06, 7.2.) With degraded fire
//! control the gun chance is halved before the experience factor: A
//! 0.03 x 0.5 x 1.4 = 0.021.
//!
//! ## A torpedo salvo of 8
//!
//! | Firing    | Target               | Chance             | Hits expected |
//! | --------- | -------------------- | ------------------ | ------------- |
//! | A (x 1.4) | not surprised, unlit | 0.04 x 1.4 = 0.056 | 0.448         |
//! | A         | surprised            | 0.084              | 0.672         |
//! | A         | lit                  | 0.112              | 0.896         |
//! | B (x 0.6) | not surprised, unlit | 0.024              | 0.192         |
//! | B         | surprised            | 0.036              | 0.288         |
//! | B         | lit                  | 0.048              | 0.384         |
//!
//! At least one hit from A's salvo in her free round (a surprised target,
//! 0.084): 1 - (1 - 0.084)^8 = 1 - 0.916^8. 0.916^2 = 0.839056; ^4 = 0.704015;
//! ^8 = 0.495638; so 0.5044. From the start (0.056): 1 - 0.944^8. 0.944^2 =
//! 0.891136; ^4 = 0.794123; ^8 = 0.630633; so 0.3694. (The test works the same
//! sum for each of the four groups of first torpedo salvos, A and B, free
//! round and from the start, from the chance each event's breakdown gives.)
//!
//! ## A force of mixed values, for the rounding
//!
//! Ships of 3, 3 and 2 have a mean of 8 / 3 = 2.666666 in the fixed
//! arithmetic (six places, cut). At a strength of 0.4 the factor is
//! 1 + 0.4 x 0.666666 = 1.266666 (0.4 x 0.666666 = 0.2666664, cut to
//! 0.266666), and a sighting of 0.5 x 1.266666 = 0.633333. These three
//! numbers are asserted in `crates/sim-core/src/surface.rs`, in
//! `a_forces_sighting_reads_the_mean_of_its_ships_inside_the_cap_and_before_fatigue`
//! (the mean is 2_666_666 raw in `the_mean_is_over_the_ships_afloat_at_the_moment_of_the_roll`).
//!
//! ## By day, and at a strength of 0
//!
//! By day, and at a strength of 0, every chance is scenario 4a's, and no
//! breakdown names `experience_factor`.
//!
//! # The three parts
//!
//! 1. The chances above, read from the events' breakdowns at the working
//!    value, with `experience_factor` in each and its value
//!    (`the_chances_at_the_working_value_are_the_hand_calculated_ones`).
//! 2. Properties over many seeds: the counts in their bands; with all
//!    strengths 0 the event log of 14a is scenario 4a's line for line; no
//!    enemy's view of an event carries the factor; a daylight fight carries
//!    none.
//! 3. A pinned outcome for one seed on the live figures (strengths 0.25),
//!    labelled as pinned: no hand calculation can say which draw succeeds. It
//!    follows the live figures and is remade with them.

mod common;

use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::ids::{EventId, TaskForceId};
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_data::Scenario;

const SEEDS: u64 = 1000;
/// Written out from the hand calculation, not read from the data files.
const A_FACTOR: Fixed = Fixed::from_raw(1_400_000);
const B_FACTOR: Fixed = Fixed::from_raw(600_000);
const A_SIGHTING: Fixed = Fixed::from_raw(700_000);
const B_SIGHTING: Fixed = Fixed::from_raw(150_000);
const GUN_BASE_NIGHT: Fixed = Fixed::from_raw(30_000);
const TORPEDO_BASE_NIGHT: Fixed = Fixed::from_raw(40_000);
const SURPRISED: Fixed = Fixed::from_raw(1_500_000);
const LIT: Fixed = Fixed::from_raw(2_000_000);

/// Force A is task force 1 (Japanese, ships 1 to 3), force B task force 2
/// (Allied, ships 4 to 6).
fn force_of_ship(ship: u32) -> usize {
    assert!(
        (1..=6).contains(&ship),
        "ship {ship} is not in the scenario"
    );
    usize::from(ship > 3)
}

fn force_index(task_force: TaskForceId) -> usize {
    assert!((1..=2).contains(&task_force.raw()));
    usize::try_from(task_force.raw() - 1).expect("small")
}

fn side_of_force(force: usize) -> Side {
    [Side::Japanese, Side::Allied][force]
}

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/14a_experience.toml")
}

fn path_of_4a() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/04a_night_action.toml")
}

/// 14a on the frozen figures altered to the working value of 0.4 for each of
/// the three strengths.
fn working() -> Scenario {
    let options = common::altered_reference("experience", |dir| {
        common::edit_file_by(
            dir,
            "surface_combat.toml",
            &[
                (
                    "experience_gun_strength = \"0\"",
                    "experience_gun_strength = \"0.4\"",
                ),
                (
                    "experience_torpedo_strength = \"0\"",
                    "experience_torpedo_strength = \"0.4\"",
                ),
                (
                    "experience_sighting_strength = \"0\"",
                    "experience_sighting_strength = \"0.4\"",
                ),
            ],
        );
    });
    Scenario::load_with(&scenario_path(), &options).expect("scenario 14a loads")
}

fn frozen(path: &std::path::Path) -> Scenario {
    common::load(path).expect("the scenario loads on the frozen figures")
}

fn run_seed(base: &Scenario, seed: u64) -> Scenario {
    let mut run = base.with_seed(seed).expect("a seed can be set");
    run.run_to_end().expect("the run resolves");
    run
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

// ---------------------------------------------------------------------
// What a run says, read from its log.
// ---------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Roll {
    pulse: Option<Pulse>,
    round: u32,
    force: usize,
    chance: Fixed,
    sighted: bool,
    factors: Vec<(&'static str, Fixed)>,
}

#[derive(Clone, Debug)]
struct Salvo {
    pulse: Option<Pulse>,
    round: u32,
    firer: u32,
    weapon: Weapon,
    shots: Fixed,
    chance: Fixed,
    expected: Fixed,
    hits: u32,
    factors: Vec<(&'static str, Fixed)>,
}

impl Salvo {
    fn factor(&self, name: &str) -> Fixed {
        self.factors
            .iter()
            .find(|(n, _)| *n == name)
            .unwrap_or_else(|| panic!("a salvo has no factor {name}"))
            .1
    }
}

fn factors_of(event: &Event) -> Vec<(&'static str, Fixed)> {
    event
        .breakdown
        .as_ref()
        .map(|b| b.factors.iter().map(|f| (f.name, f.value)).collect())
        .unwrap_or_default()
}

fn rolls_of(events: &[Event]) -> Vec<Roll> {
    events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::SightingRoll {
                round,
                task_force,
                chance,
                sighted,
                ..
            } => Some(Roll {
                pulse: event.pulse,
                round: *round,
                force: force_index(*task_force),
                chance: *chance,
                sighted: *sighted,
                factors: factors_of(event),
            }),
            _ => None,
        })
        .collect()
}

fn salvos_of(events: &[Event]) -> Vec<Salvo> {
    events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::Salvo {
                round,
                firer,
                weapon,
                shots,
                chance,
                expected,
                hits,
                ..
            } => Some(Salvo {
                pulse: event.pulse,
                round: *round,
                firer: firer.raw(),
                weapon: *weapon,
                shots: *shots,
                chance: *chance,
                expected: *expected,
                hits: *hits,
                factors: factors_of(event),
            }),
            _ => None,
        })
        .collect()
}

/// How the night fight opens, from its sighting rolls alone.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Opening {
    AFreeRound,
    BFreeRound,
    BothFire,
    PassUnseen,
}

fn opening(night_rolls: &[Roll]) -> (Opening, Option<u32>) {
    let Some(first) = night_rolls
        .iter()
        .filter(|r| r.sighted)
        .map(|r| r.round)
        .min()
    else {
        return (Opening::PassUnseen, None);
    };
    let sighted = |force: usize| {
        night_rolls
            .iter()
            .any(|r| r.round == first && r.force == force && r.sighted)
    };
    let way = match (sighted(0), sighted(1)) {
        (true, false) => Opening::AFreeRound,
        (false, true) => Opening::BFreeRound,
        (true, true) => Opening::BothFire,
        (false, false) => unreachable!("a round with a success"),
    };
    (way, Some(first))
}

/// One run: its log, and what each side was shown of it.
struct Run {
    seed: u64,
    events: Vec<Event>,
    /// For each side: the log ids of the events it was shown with an
    /// `experience_factor` in the breakdown, and the factor's value.
    shown_with_factor: Vec<(Side, EventId, Fixed)>,
    /// How many `SalvoAgainst` events each side was shown with a breakdown.
    against_with_breakdown: usize,
    against_shown: usize,
}

fn run_of(base: &Scenario, seed: u64) -> Run {
    let scenario = run_seed(base, seed);
    let world = &scenario.world;
    let mut shown_with_factor = Vec::new();
    let mut against_with_breakdown = 0;
    let mut against_shown = 0;
    for side in Side::ALL {
        for event in world.side_view(side).events() {
            if let sim_core::side_events::SideEventKind::SalvoAgainst { .. } = event.kind {
                against_shown += 1;
                against_with_breakdown += usize::from(event.breakdown.is_some());
            }
            if let Some(breakdown) = &event.breakdown {
                for factor in &breakdown.factors {
                    if factor.name == "experience_factor" {
                        shown_with_factor.push((side, event.id, factor.value));
                    }
                }
            }
        }
    }
    Run {
        seed,
        events: world.events().iter().cloned().collect(),
        shown_with_factor,
        against_with_breakdown,
        against_shown,
    }
}

fn runs() -> &'static Vec<Run> {
    static RUNS: OnceLock<Vec<Run>> = OnceLock::new();
    RUNS.get_or_init(|| {
        let base = working();
        (1..=SEEDS).map(|seed| run_of(&base, seed)).collect()
    })
}

fn night(run: &Run) -> (Vec<Roll>, Vec<Salvo>) {
    let rolls = rolls_of(&run.events)
        .into_iter()
        .filter(|r| r.pulse == Some(Pulse::Night))
        .collect();
    let salvos = salvos_of(&run.events)
        .into_iter()
        .filter(|s| s.pulse == Some(Pulse::Night))
        .collect();
    (rolls, salvos)
}

/// Whether k out of n is within four standard deviations of probability `p`
/// (in millionths), in exact integer arithmetic: (k - np)^2 <= 16np(1 - p).
fn within_four_sigma(k: i64, n: i64, p: i64) -> bool {
    let (k, n, p) = (i128::from(k), i128::from(n), i128::from(p));
    let scale = 1_000_000;
    let off = k * scale - n * p;
    off * off <= 16 * n * p * (scale - p)
}

// ---------------------------------------------------------------------
// Part 1: the chances at the working value, from the breakdowns.
// ---------------------------------------------------------------------

#[test]
fn the_ships_have_the_experience_the_scenario_gives_them() {
    let scenario = working();
    let truth = scenario.world.umpire();
    for (id, ship) in &truth.ships {
        let expected = if id.raw() <= 3 { 3 } else { 1 };
        assert_eq!(ship.experience, expected, "ship {}", id.raw());
    }
}

#[test]
fn the_chances_at_the_working_value_are_the_hand_calculated_ones() {
    let mut seen_gun = [[false; 2]; 2];
    let mut seen_torpedo = [[false; 2]; 2];
    let mut sightings = 0;
    for run in runs() {
        let (rolls, salvos) = night(run);
        // Sighting: A 0.5 x 1 x 1.4 = 0.7, B 0.5 x 0.5 x 0.6 = 0.15, with
        // the factor after the alert factor in the breakdown.
        for roll in &rolls {
            let (chance, alert, value) = if roll.force == 0 {
                (A_SIGHTING, Fixed::ONE, A_FACTOR)
            } else {
                (B_SIGHTING, fx("0.5"), B_FACTOR)
            };
            assert_eq!(roll.chance, chance, "seed {}", run.seed);
            assert_eq!(
                roll.factors,
                vec![
                    ("base", fx("0.5")),
                    ("radar_bonus", Fixed::ZERO),
                    ("alert_factor", alert),
                    ("experience_factor", value),
                ],
                "seed {}",
                run.seed
            );
            sightings += 1;
        }
        // Fire.
        for salvo in &salvos {
            let force = force_of_ship(salvo.firer);
            let value = [A_FACTOR, B_FACTOR][force];
            assert_eq!(
                salvo.factor("experience_factor"),
                value,
                "seed {}",
                run.seed
            );
            // Last among the factors.
            assert_eq!(
                salvo.factors.last().map(|f| f.0),
                Some("experience_factor"),
                "seed {}",
                run.seed
            );
            let surprised = salvo.factor("surprised_factor");
            let lit = salvo.factor("lit_factor");
            assert!(surprised == Fixed::ONE || surprised == SURPRISED);
            assert!(lit == Fixed::ONE || lit == LIT);
            let fire_control = match salvo.weapon {
                Weapon::Gun => salvo.factor("fire_control_factor"),
                _ => Fixed::ONE,
            };
            let base = match salvo.weapon {
                Weapon::Gun => GUN_BASE_NIGHT,
                _ => TORPEDO_BASE_NIGHT,
            };
            assert_eq!(salvo.factor("base_chance"), base);
            // Multiplied in this order, the experience factor last.
            assert_eq!(
                salvo.chance,
                base * surprised * lit * fire_control * value,
                "seed {}",
                run.seed
            );
            assert_eq!(salvo.expected, salvo.shots * salvo.chance);
            assert!(salvo.hits <= u32::try_from(salvo.shots.trunc()).expect("small"));

            // The hand calculation's tables, for a full battery or full
            // tubes and fire control working.
            let (is_surprised, is_lit) = (surprised == SURPRISED, lit == LIT);
            let table_gun = [
                // A: plain, surprised, lit, both
                [
                    ("0.042", "5.04"),
                    ("0.063", "7.56"),
                    ("0.084", "10.08"),
                    ("0.126", "15.12"),
                ],
                [
                    ("0.018", "2.16"),
                    ("0.027", "3.24"),
                    ("0.036", "4.32"),
                    ("0.054", "6.48"),
                ],
            ];
            let table_torpedo = [
                [
                    ("0.056", "0.448"),
                    ("0.084", "0.672"),
                    ("0.112", "0.896"),
                    ("0.168", "1.344"),
                ],
                [
                    ("0.024", "0.192"),
                    ("0.036", "0.288"),
                    ("0.048", "0.384"),
                    ("0.072", "0.576"),
                ],
            ];
            let slot = usize::from(is_surprised) + 2 * usize::from(is_lit);
            match salvo.weapon {
                Weapon::Gun => {
                    seen_gun[force][usize::from(is_surprised || is_lit)] = true;
                    if salvo.shots == Fixed::from_int(120) && fire_control == Fixed::ONE {
                        let (chance, expected) = table_gun[force][slot];
                        assert_eq!(salvo.chance, fx(chance), "seed {}", run.seed);
                        assert_eq!(salvo.expected, fx(expected), "seed {}", run.seed);
                    }
                    if fire_control == fx("0.5") && !is_surprised && !is_lit {
                        // Halved before the experience factor: A 0.021.
                        let expected_chance = [fx("0.021"), fx("0.009")][force];
                        assert_eq!(salvo.chance, expected_chance, "seed {}", run.seed);
                    }
                }
                _ => {
                    seen_torpedo[force][usize::from(is_surprised || is_lit)] = true;
                    if salvo.shots == Fixed::from_int(8) {
                        let (chance, expected) = table_torpedo[force][slot];
                        assert_eq!(salvo.chance, fx(chance), "seed {}", run.seed);
                        assert_eq!(salvo.expected, fx(expected), "seed {}", run.seed);
                    }
                }
            }
        }
    }
    assert!(sightings > 1000, "the rolls were checked: {sightings}");
    // Both an ordinary and a surprised or lit salvo were met, for each force
    // and weapon, so the table was exercised in both its columns.
    assert_eq!(seen_gun, [[true; 2]; 2]);
    assert_eq!(seen_torpedo, [[true; 2]; 2]);
}

// ---------------------------------------------------------------------
// Part 2: properties over many seeds.
// ---------------------------------------------------------------------

#[test]
fn the_counts_of_how_the_fight_opens_are_within_their_bands() {
    let (mut a1, mut b1) = (0, 0);
    let mut ways = [0i64; 4];
    for run in runs() {
        let (rolls, _) = night(run);
        a1 += i64::from(
            rolls
                .iter()
                .any(|r| r.round == 1 && r.force == 0 && r.sighted),
        );
        b1 += i64::from(
            rolls
                .iter()
                .any(|r| r.round == 1 && r.force == 1 && r.sighted),
        );
        let (way, _) = opening(&rolls);
        ways[match way {
            Opening::AFreeRound => 0,
            Opening::BFreeRound => 1,
            Opening::BothFire => 2,
            Opening::PassUnseen => 3,
        }] += 1;
    }
    eprintln!("A sights in round 1: {a1} (643..=757)");
    eprintln!("B sights in round 1: {b1} (105..=195)");
    eprintln!("A has a free round: {} (748..=849)", ways[0]);
    eprintln!("B has a free round: {} (31..=90)", ways[1]);
    eprintln!("both fire from the start: {} (97..=184)", ways[2]);
    eprintln!("pass unseen: {} (0..=2)", ways[3]);
    assert!((643..=757).contains(&a1), "A sights in round 1: {a1}");
    assert!((105..=195).contains(&b1), "B sights in round 1: {b1}");
    assert!((748..=849).contains(&ways[0]), "A free round: {}", ways[0]);
    assert!((31..=90).contains(&ways[1]), "B free round: {}", ways[1]);
    assert!((97..=184).contains(&ways[2]), "both fire: {}", ways[2]);
    assert!((0..=2).contains(&ways[3]), "pass unseen: {}", ways[3]);
    assert_eq!(ways.iter().sum::<i64>(), 1000);
    // The same bands by the rule itself, in exact arithmetic.
    assert!(within_four_sigma(a1, 1000, 700_000));
    assert!(within_four_sigma(b1, 1000, 150_000));
    assert!(within_four_sigma(ways[0], 1000, 798_438));
    assert!(within_four_sigma(ways[1], 1000, 60_386));
    assert!(within_four_sigma(ways[2], 1000, 140_901));
}

/// 1 - (1 - q)^8, in millionths: the chance that a salvo of 8 tubes at a
/// chance q lands at least one hit.
fn at_least_one_of_eight(chance: Fixed) -> i64 {
    let miss = Fixed::ONE - chance;
    let mut all = Fixed::ONE;
    for _ in 0..8 {
        all = all * miss;
    }
    (Fixed::ONE - all).raw()
}

#[test]
fn the_hits_stay_within_four_sigma_of_what_the_chances_expect() {
    // Over every night salvo: (H - E)^2 <= 16 x the sum of np(1 - p), for
    // each force and weapon, in exact arithmetic (n and p are the event's).
    // Index: force x 2 + weapon (0 gun, 1 torpedo).
    let mut hits = [0i128; 4];
    let mut expected6 = [0i128; 4];
    let mut variance12 = [0i128; 4];
    let mut shots = [0i128; 4];
    // The first torpedo salvos, grouped by how the fight opened: whether at
    // least one hit landed.
    let mut first_torpedo: Vec<((usize, bool), i64, i64, i64)> = Vec::new();
    for run in runs() {
        let (rolls, salvos) = night(run);
        let (way, first_round) = opening(&rolls);
        for salvo in &salvos {
            let force = force_of_ship(salvo.firer);
            let slot = force * 2 + usize::from(salvo.weapon == Weapon::Torpedo);
            let n = i128::from(salvo.shots.trunc());
            assert_eq!(salvo.shots, Fixed::from_int(salvo.shots.trunc()), "whole");
            let p = i128::from(salvo.chance.raw());
            hits[slot] += i128::from(salvo.hits);
            shots[slot] += n;
            expected6[slot] += n * p;
            variance12[slot] += n * p * (1_000_000 - p);
            if salvo.weapon == Weapon::Torpedo && Some(salvo.round) == first_round {
                let free = matches!(
                    (way, force),
                    (Opening::AFreeRound, 0) | (Opening::BFreeRound, 1)
                );
                let key = (force, free);
                let landed = i64::from(salvo.hits > 0);
                let p = at_least_one_of_eight(salvo.chance);
                match first_torpedo.iter_mut().find(|(k, ..)| *k == key) {
                    Some((_, runs_n, successes, expected_p)) => {
                        *runs_n += 1;
                        *successes += landed;
                        // Within a group the chance is one value.
                        assert_eq!(*expected_p, p, "seed {}", run.seed);
                    }
                    None => first_torpedo.push((key, 1, landed, p)),
                }
            }
        }
    }
    for slot in 0..4 {
        let off6 = hits[slot] * 1_000_000 - expected6[slot];
        assert!(
            off6 * off6 <= 16 * variance12[slot],
            "slot {slot}: hits {} expected (millionths) {} of {} shots",
            hits[slot],
            expected6[slot],
            shots[slot]
        );
    }
    assert_eq!(first_torpedo.len(), 4, "all four groups of first salvos");
    for ((force, free), n, successes, p) in first_torpedo {
        eprintln!("first torpedo salvos, force {force}, free round {free}: {successes} of {n} landed at least one hit (p {p})");
        assert!(
            within_four_sigma(successes, n, p),
            "force {force}, free {free}: {successes} of {n}, p {p}"
        );
    }
    // More experience never means fewer hits: A (3) lands more of its shots
    // than B (1), gun and torpedo.
    for weapon in 0..2 {
        let rate = |force: usize| hits[force * 2 + weapon] * 1_000_000 / shots[force * 2 + weapon];
        assert!(
            rate(0) > rate(1),
            "weapon {weapon}: {} v {}",
            rate(0),
            rate(1)
        );
    }
}

#[test]
fn with_every_strength_at_0_the_event_log_is_scenario_4as_line_for_line() {
    // The frozen figures hold 0, so 14a on them must be 4a on them: the same
    // events, with the same breakdowns (none names the factor).
    let a = frozen(&scenario_path());
    let b = frozen(&path_of_4a());
    for seed in 1..=100 {
        let a = run_seed(&a, seed);
        let b = run_seed(&b, seed);
        let (a, b): (Vec<Event>, Vec<Event>) = (
            a.world.events().iter().cloned().collect(),
            b.world.events().iter().cloned().collect(),
        );
        assert_eq!(a.len(), b.len(), "seed {seed}");
        for (x, y) in a.iter().zip(&b) {
            assert_eq!(x, y, "seed {seed}");
        }
        assert!(a
            .iter()
            .all(|e| !factors_of(e).iter().any(|f| f.0 == "experience_factor")));
    }
}

#[test]
fn no_enemy_view_of_an_event_carries_the_factor() {
    let mut shown = 0;
    let mut against = 0;
    for run in runs() {
        for (side, id, value) in &run.shown_with_factor {
            let event = run
                .events
                .iter()
                .find(|e| e.id == *id)
                .expect("the event is in the log");
            // The event is the side's own salvo or its own force's sighting
            // roll, and the value is its own.
            let (own_force, own_value) = match &event.kind {
                EventKind::Salvo { firer, .. } => {
                    let force = force_of_ship(firer.raw());
                    (force, [A_FACTOR, B_FACTOR][force])
                }
                EventKind::SightingRoll { task_force, .. } => {
                    let force = force_index(*task_force);
                    (force, [A_FACTOR, B_FACTOR][force])
                }
                other => panic!(
                    "seed {}: a side was shown the factor in {other:?}",
                    run.seed
                ),
            };
            assert_eq!(*side, side_of_force(own_force), "seed {}", run.seed);
            assert_eq!(*value, own_value, "seed {}", run.seed);
            shown += 1;
        }
        // The side fired on is given no breakdown at all.
        assert_eq!(run.against_with_breakdown, 0, "seed {}", run.seed);
        against += run.against_shown;
    }
    assert!(shown > 1000, "the views were checked: {shown}");
    assert!(against > 1000, "salvos against were shown: {against}");
}

#[test]
fn a_daylight_fight_carries_no_experience_factor() {
    // A force whose slowest ship is dead in the water stays in the hex and
    // fights again by day (scenario 4a). Those fights are by day: nothing in
    // them reads experience, on the working figures.
    let mut day_salvos = 0;
    let mut day_rolls = 0;
    for run in runs() {
        for roll in rolls_of(&run.events) {
            if roll.pulse != Some(Pulse::Night) {
                day_rolls += 1;
                assert!(
                    !roll.factors.iter().any(|f| f.0 == "experience_factor"),
                    "seed {}",
                    run.seed
                );
            }
        }
        for salvo in salvos_of(&run.events) {
            if salvo.pulse != Some(Pulse::Night) {
                day_salvos += 1;
                assert!(
                    !salvo.factors.iter().any(|f| f.0 == "experience_factor"),
                    "seed {}",
                    run.seed
                );
                // The plain day chance, whatever the firer's crew.
                let (base, torpedo) = (fx("0.05"), fx("0.02"));
                let fire_control = match salvo.weapon {
                    Weapon::Gun => salvo.factor("fire_control_factor"),
                    _ => Fixed::ONE,
                };
                let base = if salvo.weapon == Weapon::Gun {
                    base
                } else {
                    torpedo
                };
                assert_eq!(salvo.chance, base * fire_control, "seed {}", run.seed);
            }
        }
    }
    assert!(day_salvos > 0 && day_rolls > 0, "some run fought by day");
}

// ---------------------------------------------------------------------
// Part 3: a pinned outcome on the live figures.
// ---------------------------------------------------------------------

/// PINNED, not calculated. On the live figures (strengths 0.25, so A's factor
/// is 1.25 and B's 0.75), seed 1942, the scenario's own. No hand calculation
/// can say which draw succeeds; this catches a stream that moved, which the
/// properties above would miss. It follows the live figures: when a
/// strength in `data/reference/surface_combat.toml` is tuned, or a rule is
/// changed on purpose, it is remade with the change, and never to make a test
/// pass.
#[test]
fn pinned_outcome_of_seed_1942_on_the_live_figures() {
    let base = common::load_live(&scenario_path()).expect("scenario 14a loads on live figures");
    let run = run_of(&base, 1942);
    let (rolls, salvos) = night(&run);
    let (way, first_round) = opening(&rolls);
    let summary = Pinned {
        way,
        first_round,
        rolls: rolls.len(),
        salvos: salvos.len(),
        hits_a: hits_of(&salvos, 0),
        hits_b: hits_of(&salvos, 1),
        events: run.events.len(),
    };
    eprintln!("{summary:?}");
    assert_eq!(summary, PINNED);
}

fn hits_of(salvos: &[Salvo], force: usize) -> u32 {
    salvos
        .iter()
        .filter(|s| force_of_ship(s.firer) == force)
        .map(|s| s.hits)
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Pinned {
    way: Opening,
    first_round: Option<u32>,
    rolls: usize,
    salvos: usize,
    hits_a: u32,
    hits_b: u32,
    events: usize,
}

const PINNED: Pinned = Pinned {
    way: Opening::AFreeRound,
    first_round: Some(1),
    rolls: 2,
    salvos: 15,
    hits_a: 44,
    hits_b: 5,
    events: 75,
};
