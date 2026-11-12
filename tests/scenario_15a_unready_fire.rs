//! Scenario 15a: a force that is not ready fires part of its guns.
//!
//! Decision 0068 (`docs/decisions/0068-a-force-not-at-battle-readiness-fires-with-part-of-its-guns.md`):
//! a force that is not ready when a fight begins fires its guns at one share,
//! `unready_gun_share`, for the whole of that fight. Ready is decided once,
//! when the fight begins: the force's readiness is battle by order, or it was
//! alert before the present pulse began. This is scenario 4a exactly
//! (`data/scenarios/04a_night_action.toml`; the file is
//! `data/scenarios/15a_unready_fire.toml`): force A at battle readiness, force
//! B at normal readiness, stopped, and never alerted before the fight.
//! Everything not said here is scenario 4a's hand calculation:
//! `tests/scenario_04a_night_action.rs`.
//!
//! The share is 1 in the frozen figures (step 1 had no such rule; at 1 it does
//! nothing). The working value here is 0.5, set through an altered copy of the
//! frozen figures (`common::altered_reference`), as scenario 14a sets its
//! strengths; so experience's strengths are 0 and every other number is
//! scenario 4a's. The test has the three parts the spec asks for
//! (`docs/step2-spec.md`, "Tests while figures move", 7): the shots and
//! expected hits at the working value, read from the events' breakdowns;
//! properties over many seeds; and a pinned outcome for one seed on the live
//! figures.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! New scenario `data/scenarios/15a_unready_fire.toml`: scenario 4a exactly
//! (force A, three placeholder cruisers, at battle readiness; force B, three,
//! at normal readiness, stopped, and never alerted before the fight). New test
//! `tests/scenario_15a_unready_fire.rs`, registered. The working value of
//! `unready_gun_share` is 0.5, set through an altered copy of the frozen
//! figures (so experience's strengths are 0 and every other number is
//! scenario 4a's).
//!
//! - A full gun salvo is 8 guns x 3 rounds a gun a minute x 5 minutes = 120
//!   shots a ship. A torpedo salvo is 8.
//! - Force A is ready: 120 shots a ship a round; 120 x 0.03 = 3.6 hits
//!   expected; at a surprised target 120 x 0.045 = 5.4; at a lit target
//!   120 x 0.06 = 7.2.
//! - Force B is not ready, in every round of the fight: 120 x 0.5 = 60 shots
//!   a ship; 60 x 0.03 = 1.8 hits expected; at a surprised target 60 x 0.045 =
//!   2.7; at a lit target 60 x 0.06 = 3.6. Being fired on, sighting and
//!   becoming alert during the fight do not make her ready in it: round 6 is
//!   as round 1.
//! - B's torpedoes are as scenario 4a's: 8 a ship, 8 x 0.04 = 0.32 expected,
//!   0.48 at a surprised target.
//! - A ship of B with her main battery degraded: 120 x 0.5 (the battery) x 0.5
//!   (the share) = 30 shots; with it out, none.
//! - Sighting and how the fight opens are scenario 4a's to the last digit: A
//!   0.5, B 0.25; A has the free round with 0.598331, B with 0.199444, both
//!   from the start with 0.199444, neither ever with 0.002781.
//! - Three more cases, each a small edit of the scenario text inside the test,
//!   with the numbers by the rule:
//!   (b) force B ordered to battle readiness: ready, 120 shots a ship, and the
//!   log is scenario 4a's with B at battle readiness.
//!   (c) force B at normal readiness but alert since before the pulse of the
//!   fight: ready, 120 shots.
//!   (d) force B at normal readiness, alerted in the SAME pulse before the
//!   fight (by the alarm from another fight in the hex, as scenario 4c has it,
//!   or by being in an earlier fight of that pulse): not ready, 60 shots a
//!   ship, while her sighting is the alert one.
//! - With the share at 1 (the frozen figure) the event log of 15a is scenario
//!   4a's line for line.
//!
//! ## The set-ups of cases (c) and (d) as built (the developer's, for the
//! architect's approval)
//!
//! The rules make a force alert in an earlier pulse only by a contact held at
//! the start of that pulse (alert from belief, decision 0021) or by a fight
//! of that pulse; at the start of the scenario's first pulse nothing is
//! earlier. So case (c) moves force B to (7,0), 280 nm from force A, and gives
//! the Allied side one contact at (7,0), held from the start (a contact is
//! fresh for 2 pulses). At the start of the night pulse that contact alerts B
//! (decision 0021, 3): B is alerted in the night pulse. Force A enters (7,0)
//! 20 nm short of its centre after 260 nm: 192 nm in the night pulse and 68 nm,
//! 4.25 hours, into the morning pulse. The fight is in the MORNING pulse, by
//! day, and B was alerted in the night pulse, an earlier one: it is ready.
//! Everything of the fight is by day: both force sight in round 1 for sure (a
//! day sighting is 1, and B is alert), nobody is surprised, a gun shot hits
//! with 0.05 and a torpedo with 0.02, and fire lights nothing. A full gun
//! salvo is 120 shots, 120 x 0.05 = 6 hits expected a ship for each force;
//! torpedoes 8 x 0.02 = 0.16. As a control the same text without the contact:
//! B is not alerted, not ready, sights at 1 x 0.5 = 0.5 (not alert), and fires
//! 60 shots (60 x 0.05 = 3 expected at a target that is not surprised).
//!
//! Case (d) adds a third force, C (task force 3, ships 7 to 9, Allied, normal
//! readiness, stopped at (5,0) with B), with the alarm chance of the altered
//! figures at 1 instead of 0.5 so that the alarm is sure, as in scenario 4c.
//! Force A fights B first (task force ID order), at the same moment it
//! enters (5,0), and then C. If a salvo was fired in the first fight, C is
//! alarmed by it (chance 1) and is alerted in the night pulse, the pulse of
//! its fight: not ready. It fires 60 shots a ship in each round of its fight
//! with A and sights with 0.5 (an alert force) in round 1. If A and B passed
//! unseen (0.002781) there is no alarm, C is not alert, and sights with
//! 0.25: but C is not ready either, and fires 60. So in every run C's gun
//! salvos are 60 x the battery factor. B fires 60 in the first fight as in
//! scenario 4a, the alarm having nothing to do with it.
//!
//! ## Detection rates over 1,000 runs
//!
//! The sighting is scenario 4a's, and so are its bands (4a counts each as
//! within four standard deviations of its mean). The rule touches no sighting
//! stream, so the test asserts more than the bands: with the same seed the
//! night fight opens in the same way and in the same round as in scenario 4a.
//!
//! | Count of runs where         | p        | Allowed    |
//! | --------------------------- | -------- | ---------- |
//! | A sights in round 1         | 0.5      | 437 to 563 |
//! | B sights in round 1         | 0.25     | 196 to 304 |
//! | A has a free round          | 0.598331 | 537 to 660 |
//! | B has a free round          | 0.199444 | 149 to 249 |
//! | Both fire from the start    | 0.199444 | 149 to 249 |
//! | They pass unseen            | 0.002781 | 0 to 9     |
//!
//! ## The shots, read from the breakdowns
//!
//! In every night salvo of every run: A's gun shots are 120 x the main
//! battery factor, B's are 120 x the main battery factor x 0.5, and
//! `unready_gun_share` is in B's gun salvos, straight after
//! `main_battery_factor`, and in none of A's and in no torpedo salvo. With the
//! battery working, the table above: A 120 shots, 3.6 / 5.4 / 7.2 / 10.8
//! expected (not surprised and unlit, surprised, lit, both); B 60 shots, 1.8 /
//! 2.7 / 3.6 / 5.4; with the battery degraded B 30 shots, 0.9 / 1.35 / 1.8 /
//! 2.7. Torpedoes 8, 0.32 / 0.48 / 0.64 / 0.96.
//!
//! Over the 1,000 runs, B's gun shots in the night fight, summed, are half of
//! what the same rounds give at full fire: the sum over B's night gun salvos
//! of shots, doubled, is exactly the sum over the same salvos of guns x rate x
//! minutes x main battery factor (the figures of each salvo's own breakdown).
//! For force A the two sums are equal. The sums are printed.
//!
//! # The three parts
//!
//! 1. The shots and expected hits above, read from the events' breakdowns at
//!    the working value
//!    (`the_shots_and_chances_at_the_working_value_are_the_hand_calculated_ones`).
//! 2. Properties over many seeds: every gun salvo of a force that was not ready
//!    when its fight began has shots equal to its ship's full salvo x battery
//!    factor x the share (before the ammunition cap), in every round; every
//!    salvo of a ready force has no such factor; no enemy's view of an event
//!    carries the factor; with the share at 1 the log is 4a's; and the cases
//!    (b), (c) and (d).
//! 3. A pinned outcome for one seed on the live figures (share 0.5, strengths
//!    of experience 0.25, all ships at 2), labelled as pinned: no hand
//!    calculation can say which draw succeeds. It follows the live figures and
//!    is remade with them.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::ids::{EventId, TaskForceId};
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_data::{LoadOptions, Scenario};

const SEEDS: u64 = 1000;
/// Written out from the hand calculation, not read from the data files.
const SHARE: Fixed = Fixed::from_raw(500_000);
const GUN_BASE_NIGHT: Fixed = Fixed::from_raw(30_000);
const TORPEDO_BASE_NIGHT: Fixed = Fixed::from_raw(40_000);
const SURPRISED: Fixed = Fixed::from_raw(1_500_000);
const LIT: Fixed = Fixed::from_raw(2_000_000);
const FULL_SALVO: i64 = 120;

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/15a_unready_fire.toml")
}

fn path_of_4a() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/04a_night_action.toml")
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

/// The frozen figures with the share at the working value of 0.5, and the
/// alarm chance at `alarm` if one is given.
fn working_options(alarm: Option<&str>) -> LoadOptions {
    common::altered_reference("unready", |dir| {
        common::edit_file_by(
            dir,
            "surface_combat.toml",
            &[("unready_gun_share = \"1\"", "unready_gun_share = \"0.5\"")],
        );
        if let Some(alarm) = alarm {
            common::edit_file_by(
                dir,
                "surface_combat.toml",
                &[(
                    "alarm_chance = \"0.5\"",
                    &format!("alarm_chance = \"{alarm}\""),
                )],
            );
        }
    })
}

fn text_of(path: &std::path::Path) -> String {
    std::fs::read_to_string(path).expect("the scenario file reads")
}

fn working() -> Scenario {
    Scenario::load_with(&scenario_path(), &working_options(None)).expect("scenario 15a loads")
}

fn frozen(path: &std::path::Path) -> Scenario {
    common::load(path).expect("the scenario loads on the frozen figures")
}

fn from_text(text: &str, options: &LoadOptions) -> Scenario {
    Scenario::from_text_with(text, &scenario_path(), options).expect("the variant loads")
}

fn run_seed(base: &Scenario, seed: u64) -> Scenario {
    let mut run = base.with_seed(seed).expect("a seed can be set");
    run.run_to_end().expect("the run resolves");
    run
}

/// Task force 1 is ships 1 to 3, task force 2 ships 4 to 6, task force 3
/// (case (d) only) ships 7 to 9.
fn force_of_ship(ship: u32) -> u32 {
    assert!(
        (1..=9).contains(&ship),
        "ship {ship} is not in the scenario"
    );
    (ship - 1) / 3 + 1
}

fn side_of_force(force: u32) -> Side {
    if force == 1 {
        Side::Japanese
    } else {
        Side::Allied
    }
}

// ---------------------------------------------------------------------
// What a run says, read from its log.
// ---------------------------------------------------------------------

type Factors = Vec<(&'static str, Fixed)>;

fn factors_of(event: &Event) -> Factors {
    event
        .breakdown
        .as_ref()
        .map(|b| b.factors.iter().map(|f| (f.name, f.value)).collect())
        .unwrap_or_default()
}

fn named(factors: &Factors, name: &str) -> Option<Fixed> {
    factors.iter().find(|(n, _)| *n == name).map(|(_, v)| *v)
}

#[derive(Clone, Debug)]
struct Roll {
    pulse: Option<Pulse>,
    round: u32,
    force: u32,
    chance: Fixed,
    sighted: bool,
}

#[derive(Clone, Debug)]
struct Salvo {
    day: u32,
    pulse: Option<Pulse>,
    round: u32,
    firer: u32,
    weapon: Weapon,
    shots: Fixed,
    chance: Fixed,
    expected: Fixed,
    hits: u32,
    factors: Factors,
    /// Whether the firer's force was ready when the fight of this salvo
    /// began, worked from the log by [`readiness_of_salvos`], not from the
    /// salvo's own breakdown.
    ready: bool,
}

impl Salvo {
    fn factor(&self, name: &str) -> Fixed {
        named(&self.factors, name).unwrap_or_else(|| panic!("a salvo has no factor {name}"))
    }

    fn night(&self) -> bool {
        self.pulse == Some(Pulse::Night)
    }
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
                force: task_force.raw(),
                chance: *chance,
                sighted: *sighted,
            }),
            _ => None,
        })
        .collect()
}

/// Every salvo of the log, with whether its force was ready as its fight
/// began. The rule is worked here from the events alone, as the decision
/// states it: a force is ready if it is at battle readiness (the forces in
/// `battle`, by order or from the start) or was alert before the pulse of the
/// fight began. A force is alert from the event that alerted it (a contact at
/// the start of a pulse, being sighted or fired on in a fight, or the alarm)
/// until it stands down.
fn salvos_of(events: &[Event], battle: &[u32]) -> Vec<Salvo> {
    let mut alert_since: BTreeMap<u32, (u32, Pulse)> = BTreeMap::new();
    let mut ready_in_fight: Option<BTreeMap<u32, bool>> = None;
    let mut salvos = Vec::new();
    for event in events {
        match &event.kind {
            EventKind::ForceAlertedByContact { task_force, .. }
            | EventKind::ForceBecameAlert { task_force, .. }
            | EventKind::AlarmRoll {
                task_force,
                alarmed: true,
                ..
            } => {
                let pulse = event.pulse.expect("an alert comes in a pulse");
                alert_since
                    .entry(task_force.raw())
                    .or_insert((event.day, pulse));
            }
            EventKind::ForceStoodDown { task_force } => {
                alert_since.remove(&task_force.raw());
            }
            EventKind::EngagementBegun { a, b, .. } => {
                let now = (event.day, event.pulse.expect("a fight is in a pulse"));
                let ready = |id: &TaskForceId| {
                    battle.contains(&id.raw())
                        || alert_since.get(&id.raw()).is_some_and(|since| *since < now)
                };
                ready_in_fight = Some([a, b].into_iter().map(|id| (id.raw(), ready(id))).collect());
            }
            EventKind::Salvo {
                round,
                firer,
                weapon,
                shots,
                chance,
                expected,
                hits,
                ..
            } => {
                let ready = ready_in_fight
                    .as_ref()
                    .expect("a salvo is in a fight")
                    .get(&force_of_ship(firer.raw()))
                    .copied()
                    .expect("the firer's force is in the fight");
                salvos.push(Salvo {
                    day: event.day,
                    pulse: event.pulse,
                    round: *round,
                    firer: firer.raw(),
                    weapon: *weapon,
                    shots: *shots,
                    chance: *chance,
                    expected: *expected,
                    hits: *hits,
                    factors: factors_of(event),
                    ready,
                });
            }
            _ => {}
        }
    }
    salvos
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
    let sighted = |force: u32| {
        night_rolls
            .iter()
            .any(|r| r.round == first && r.force == force && r.sighted)
    };
    let way = match (sighted(1), sighted(2)) {
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
    /// `unready_gun_share` in the breakdown, and the factor's value.
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
                    if factor.name == "unready_gun_share" {
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

fn night_rolls(run: &Run) -> Vec<Roll> {
    rolls_of(&run.events)
        .into_iter()
        .filter(|r| r.pulse == Some(Pulse::Night))
        .collect()
}

fn night_salvos(run: &Run, battle: &[u32]) -> Vec<Salvo> {
    salvos_of(&run.events, battle)
        .into_iter()
        .filter(Salvo::night)
        .collect()
}

/// Whether k out of n is within four standard deviations of probability `p`
/// (in millionths), in exact integer arithmetic: (k - np)^2 <= 16np(1 - p).
fn within_four_sigma(k: i64, n: i64, p: i64) -> bool {
    let (k, n, p) = (i128::from(k), i128::from(n), i128::from(p));
    let scale = 1_000_000;
    let off = k * scale - n * p;
    off * off <= 16 * n * p * (scale - p)
}

/// The full gun salvo of a cruiser: 8 guns x 3 rounds a gun a minute x 5
/// minutes, times the factor.
fn full_salvo(battery_factor: Fixed) -> Fixed {
    Fixed::from_int(FULL_SALVO) * battery_factor
}

// ---------------------------------------------------------------------
// Part 1: the shots and chances at the working value, from the breakdowns.
// ---------------------------------------------------------------------

#[test]
fn the_shots_and_chances_at_the_working_value_are_the_hand_calculated_ones() {
    // Gun cells seen: [A, B full battery] x [plain, surprised, lit, both]
    // and B with a degraded battery.
    let mut seen_gun = [[false; 4]; 2];
    let mut seen_b_degraded = false;
    let mut seen_torpedo = [[false; 4]; 2];
    let gun_expected = [
        // A, 120 shots
        ["3.6", "5.4", "7.2", "10.8"],
        // B, 60 shots
        ["1.8", "2.7", "3.6", "5.4"],
    ];
    let b_degraded_expected = ["0.9", "1.35", "1.8", "2.7"];
    let torpedo_expected = ["0.32", "0.48", "0.64", "0.96"];
    let mut b_salvos_in_round_6 = 0;
    for run in runs() {
        for salvo in night_salvos(run, &[1]) {
            let force = usize::from(force_of_ship(salvo.firer) == 2);
            let surprised = salvo.factor("surprised_factor");
            let lit = salvo.factor("lit_factor");
            assert!(surprised == Fixed::ONE || surprised == SURPRISED);
            assert!(lit == Fixed::ONE || lit == LIT);
            let slot = usize::from(surprised == SURPRISED) + 2 * usize::from(lit == LIT);
            match salvo.weapon {
                Weapon::Gun => {
                    let battery = salvo.factor("main_battery_factor");
                    let fire_control = salvo.factor("fire_control_factor");
                    assert_eq!(salvo.factor("base_chance"), GUN_BASE_NIGHT);
                    // Force A: 120 x the battery factor, and no share.
                    // Force B: 120 x the battery factor x 0.5, and the share
                    // in the breakdown, straight after the battery factor.
                    let names: Vec<&str> = salvo.factors.iter().map(|f| f.0).collect();
                    if force == 0 {
                        assert_eq!(salvo.shots, full_salvo(battery), "seed {}", run.seed);
                        assert!(!names.contains(&"unready_gun_share"));
                    } else {
                        assert_eq!(
                            salvo.shots,
                            full_salvo(battery) * SHARE,
                            "seed {} round {}",
                            run.seed,
                            salvo.round
                        );
                        assert_eq!(salvo.factor("unready_gun_share"), SHARE);
                        assert_eq!(
                            &names[..6],
                            [
                                "guns",
                                "rate",
                                "minutes",
                                "main_battery_factor",
                                "unready_gun_share",
                                "base_chance"
                            ],
                            "seed {}",
                            run.seed
                        );
                        b_salvos_in_round_6 += usize::from(salvo.round == 6);
                    }
                    // The chance is the base times the factors that apply;
                    // there is no experience factor on the frozen figures.
                    assert_eq!(
                        salvo.chance,
                        GUN_BASE_NIGHT * surprised * lit * fire_control,
                        "seed {}",
                        run.seed
                    );
                    assert_eq!(salvo.expected, salvo.shots * salvo.chance);
                    assert!(salvo.hits <= u32::try_from(salvo.shots.trunc()).expect("small"));
                    if fire_control == Fixed::ONE {
                        if battery == Fixed::ONE {
                            seen_gun[force][slot] = true;
                            assert_eq!(
                                salvo.expected,
                                fx(gun_expected[force][slot]),
                                "seed {}",
                                run.seed
                            );
                            assert_eq!(
                                salvo.shots,
                                Fixed::from_int([120, 60][force]),
                                "seed {}",
                                run.seed
                            );
                        } else if force == 1 && battery == fx("0.5") {
                            seen_b_degraded = true;
                            assert_eq!(salvo.shots, Fixed::from_int(30));
                            assert_eq!(salvo.expected, fx(b_degraded_expected[slot]));
                        }
                    }
                    // A main battery that is out fires no salvo at all.
                    assert!(salvo.shots > Fixed::ZERO);
                }
                Weapon::Torpedo => {
                    // Torpedoes are untouched: no share, whoever fires.
                    assert_eq!(named(&salvo.factors, "unready_gun_share"), None);
                    assert_eq!(salvo.factor("base_chance"), TORPEDO_BASE_NIGHT);
                    if salvo.factor("torpedo_mount_factor") == Fixed::ONE {
                        seen_torpedo[force][slot] = true;
                        assert_eq!(salvo.shots, Fixed::from_int(8), "seed {}", run.seed);
                        assert_eq!(salvo.expected, fx(torpedo_expected[slot]));
                    }
                }
                Weapon::Bomb => panic!("a surface fight fires no bombs"),
            }
        }
    }
    // Each cell of the table was met, for each force, but one: a target is
    // surprised only in the first round and lit only by fire from an earlier
    // one, so no night salvo has both.
    for cells in &seen_gun {
        assert_eq!(cells[..3], [true; 3]);
        assert!(!cells[3]);
    }
    assert!(
        seen_b_degraded,
        "a ship of B with her battery degraded fired"
    );
    assert!(seen_torpedo[0][0] && seen_torpedo[0][1] && seen_torpedo[1][0] && seen_torpedo[1][1]);
    // B fired in round 6 in some runs: the share holds to the last round.
    assert!(
        b_salvos_in_round_6 > 0,
        "some fight went to round 6 with B firing"
    );
}

// ---------------------------------------------------------------------
// Part 2: properties over many seeds.
// ---------------------------------------------------------------------

#[test]
fn the_night_fight_opens_as_in_scenario_4a_for_the_same_seed_and_in_its_bands() {
    let b = frozen(&path_of_4a());
    let (mut a1, mut b1) = (0, 0);
    let mut ways = [0i64; 4];
    for run in runs() {
        let rolls = night_rolls(run);
        a1 += i64::from(
            rolls
                .iter()
                .any(|r| r.round == 1 && r.force == 1 && r.sighted),
        );
        b1 += i64::from(
            rolls
                .iter()
                .any(|r| r.round == 1 && r.force == 2 && r.sighted),
        );
        let (way, first) = opening(&rolls);
        ways[match way {
            Opening::AFreeRound => 0,
            Opening::BFreeRound => 1,
            Opening::BothFire => 2,
            Opening::PassUnseen => 3,
        }] += 1;
        // The rule touches no sighting stream: scenario 4a's night fight with
        // the same seed opens the same way, in the same round, with the same
        // chances.
        let reference = run_of(&b, run.seed);
        let their_rolls = night_rolls(&reference);
        assert_eq!(opening(&their_rolls), (way, first), "seed {}", run.seed);
        let first_round: Vec<_> = rolls
            .iter()
            .filter(|r| r.round == 1)
            .map(|r| (r.force, r.chance, r.sighted))
            .collect();
        let their_first: Vec<_> = their_rolls
            .iter()
            .filter(|r| r.round == 1)
            .map(|r| (r.force, r.chance, r.sighted))
            .collect();
        assert_eq!(first_round, their_first, "seed {}", run.seed);
        // The chances: A 0.5, B 0.25, in every roll.
        for roll in &rolls {
            let chance = if roll.force == 1 { "0.5" } else { "0.25" };
            assert_eq!(roll.chance, fx(chance), "seed {}", run.seed);
        }
    }
    eprintln!("A sights in round 1: {a1} (437..=563)");
    eprintln!("B sights in round 1: {b1} (196..=304)");
    eprintln!("A has a free round: {} (537..=660)", ways[0]);
    eprintln!("B has a free round: {} (149..=249)", ways[1]);
    eprintln!("both fire from the start: {} (149..=249)", ways[2]);
    eprintln!("pass unseen: {} (0..=9)", ways[3]);
    assert!((437..=563).contains(&a1), "A sights in round 1: {a1}");
    assert!((196..=304).contains(&b1), "B sights in round 1: {b1}");
    assert!((537..=660).contains(&ways[0]), "A free round: {}", ways[0]);
    assert!((149..=249).contains(&ways[1]), "B free round: {}", ways[1]);
    assert!((149..=249).contains(&ways[2]), "both fire: {}", ways[2]);
    assert!((0..=9).contains(&ways[3]), "pass unseen: {}", ways[3]);
    assert_eq!(ways.iter().sum::<i64>(), 1000);
    assert!(within_four_sigma(a1, 1000, 500_000));
    assert!(within_four_sigma(b1, 1000, 250_000));
    assert!(within_four_sigma(ways[0], 1000, 598_331));
    assert!(within_four_sigma(ways[1], 1000, 199_444));
    assert!(within_four_sigma(ways[2], 1000, 199_444));
}

/// Worked from the log alone: the ammunition each ship has left, so that the
/// cap, which the share comes before, can be taken into account exactly.
fn check_every_salvo_against_the_rule(run: &Run, battle: &[u32], share: Fixed) {
    let mut ammunition: BTreeMap<u32, Fixed> = BTreeMap::new();
    for salvo in salvos_of(&run.events, battle) {
        let ship = salvo.firer;
        if salvo.weapon == Weapon::Torpedo {
            assert_eq!(
                named(&salvo.factors, "unready_gun_share"),
                None,
                "seed {}",
                run.seed
            );
            continue;
        }
        let left = ammunition.entry(ship).or_insert(Fixed::from_int(800));
        let battery = salvo.factor("main_battery_factor");
        // Ready force: no factor, the full salvo. Not ready: the factor, and
        // the full salvo x the battery factor x the share, before the cap.
        let uncapped = if salvo.ready {
            assert_eq!(
                named(&salvo.factors, "unready_gun_share"),
                None,
                "seed {} ship {ship} round {}: a ready force's salvo names the share",
                run.seed,
                salvo.round
            );
            full_salvo(battery)
        } else {
            assert_eq!(
                named(&salvo.factors, "unready_gun_share"),
                Some(share),
                "seed {} ship {ship} round {}: a force not ready names the share",
                run.seed,
                salvo.round
            );
            full_salvo(battery) * share
        };
        let expected_shots = uncapped.min(*left);
        assert_eq!(
            salvo.shots, expected_shots,
            "seed {} ship {ship} day {} {:?} round {}",
            run.seed, salvo.day, salvo.pulse, salvo.round
        );
        *left -= salvo.shots;
    }
}

#[test]
fn every_gun_salvo_of_a_force_not_ready_is_the_full_salvo_times_battery_times_the_share() {
    // Over 1,000 runs, every salvo, every round, every fight (the later
    // fights by day with the stragglers too). B is not ready in the night
    // fight; in a later fight it is ready if it was alert since before that
    // pulse (the log says which). A is at battle readiness throughout.
    let mut not_ready = 0;
    let mut ready_b = 0;
    for run in runs() {
        check_every_salvo_against_the_rule(run, &[1], SHARE);
        for salvo in salvos_of(&run.events, &[1]) {
            if salvo.weapon == Weapon::Gun && force_of_ship(salvo.firer) == 2 {
                if salvo.ready {
                    ready_b += 1;
                } else {
                    not_ready += 1;
                }
            }
            // A is always ready, B is not in the night fight.
            if force_of_ship(salvo.firer) == 1 {
                assert!(salvo.ready);
            }
            if salvo.night() && force_of_ship(salvo.firer) == 2 {
                assert!(!salvo.ready);
            }
        }
    }
    eprintln!("B gun salvos fired not ready: {not_ready}, ready (later fights): {ready_b}");
    assert!(not_ready > 3000, "the rule was checked: {not_ready}");
}

#[test]
fn b_fires_half_of_what_the_same_rounds_give_at_full_fire_and_a_all_of_it() {
    // The check, exactly. Over the 1,000 runs and over every gun salvo of the
    // night fight, force B's `shots` summed, doubled, equals the sum over the
    // same salvos of guns x rate x minutes x main battery factor (each
    // salvo's own breakdown figures: what the same round would fire at full
    // fire, with the battery as it was). For force A the two sums are equal.
    let (mut b_shots, mut b_full, mut a_shots, mut a_full) = (0i64, 0i64, 0i64, 0i64);
    for run in runs() {
        for salvo in night_salvos(run, &[1]) {
            if salvo.weapon != Weapon::Gun {
                continue;
            }
            let full = (salvo.factor("guns")
                * salvo.factor("rate")
                * salvo.factor("minutes")
                * salvo.factor("main_battery_factor"))
            .raw();
            if force_of_ship(salvo.firer) == 2 {
                b_shots += salvo.shots.raw();
                b_full += full;
            } else {
                a_shots += salvo.shots.raw();
                a_full += full;
            }
        }
    }
    eprintln!(
        "B: shots {}, at full fire {} (doubled shots {}); A: shots {}, at full fire {}",
        Fixed::from_raw(b_shots),
        Fixed::from_raw(b_full),
        Fixed::from_raw(2 * b_shots),
        Fixed::from_raw(a_shots),
        Fixed::from_raw(a_full)
    );
    assert!(b_full > 0 && a_full > 0);
    assert_eq!(2 * b_shots, b_full);
    assert_eq!(a_shots, a_full);
}

#[test]
fn the_hits_stay_within_four_sigma_of_what_the_chances_expect() {
    // Over every night salvo: (H - E)^2 <= 16 x the sum of np(1 - p), for
    // each force and weapon, in exact arithmetic (n and p are the event's).
    // Index: force x 2 + weapon (0 gun, 1 torpedo).
    let mut hits = [0i128; 4];
    let mut expected6 = [0i128; 4];
    let mut variance12 = [0i128; 4];
    for run in runs() {
        for salvo in night_salvos(run, &[1]) {
            let force = usize::from(force_of_ship(salvo.firer) == 2);
            let slot = force * 2 + usize::from(salvo.weapon == Weapon::Torpedo);
            let n = i128::from(salvo.shots.trunc());
            assert_eq!(salvo.shots, Fixed::from_int(salvo.shots.trunc()), "whole");
            let p = i128::from(salvo.chance.raw());
            hits[slot] += i128::from(salvo.hits);
            expected6[slot] += n * p;
            variance12[slot] += n * p * (1_000_000 - p);
        }
    }
    for slot in 0..4 {
        let off6 = hits[slot] * 1_000_000 - expected6[slot];
        assert!(
            off6 * off6 <= 16 * variance12[slot],
            "slot {slot}: hits {} expected (millionths) {}",
            hits[slot],
            expected6[slot]
        );
    }
}

#[test]
fn with_the_share_at_1_the_event_log_is_scenario_4as_line_for_line() {
    // The frozen figures hold 1, so 15a on them must be 4a on them: the same
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
            .all(|e| named(&factors_of(e), "unready_gun_share").is_none()));
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
            // The event is the side's own gun salvo, and the value is its own.
            match &event.kind {
                EventKind::Salvo { firer, weapon, .. } => {
                    assert_eq!(*weapon, Weapon::Gun, "seed {}", run.seed);
                    assert_eq!(
                        *side,
                        side_of_force(force_of_ship(firer.raw())),
                        "seed {}",
                        run.seed
                    );
                    assert_eq!(side_of_force(force_of_ship(firer.raw())), Side::Allied);
                }
                other => panic!(
                    "seed {}: a side was shown the factor in {other:?}",
                    run.seed
                ),
            }
            assert_eq!(*value, SHARE, "seed {}", run.seed);
            shown += 1;
        }
        // The side fired on is given no breakdown at all.
        assert_eq!(run.against_with_breakdown, 0, "seed {}", run.seed);
        against += run.against_shown;
    }
    assert!(shown > 1000, "the views were checked: {shown}");
    assert!(against > 1000, "salvos against were shown: {against}");
}

// ---------------------------------------------------------------------
// The other ways to be ready, and not: cases (b), (c) and (d).
// ---------------------------------------------------------------------

/// (b) Force B ordered to battle readiness on day 1.
fn text_b_ordered_battle() -> String {
    let mut text = text_of(&scenario_path());
    text.push_str(
        "\n[[order]]\nday = 1\nside = \"allied\"\ntask_force = 2\nkind = \"set_readiness\"\nreadiness = \"battle\"\n",
    );
    text
}

/// Scenario 4a with B at battle readiness from the start, as a standing
/// readiness and not by an order.
fn text_b_at_battle_from_the_start() -> String {
    common::replace_once(
        &text_of(&scenario_path()),
        "q = 5\nr = 0\nmission = \"surface_combat\"\n",
        "q = 5\nr = 0\nmission = \"surface_combat\"\nreadiness = \"battle\"\n",
    )
}

#[test]
fn b_ordered_to_battle_readiness_is_ready_and_the_log_is_4a_with_b_at_battle_readiness() {
    let by_order = from_text(&text_b_ordered_battle(), &working_options(None));
    let at_start_working = from_text(&text_b_at_battle_from_the_start(), &working_options(None));
    let at_start_frozen = from_text(
        &text_b_at_battle_from_the_start(),
        &LoadOptions::frozen_step1(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")),
    );
    let mut b_gun_salvos = 0;
    for seed in 1..=300 {
        let ordered = run_of(&by_order, seed);
        // 120 shots a ship x the battery factor (or the rounds left) in every
        // round, no factor: every salvo of both forces is a ready force's.
        check_every_salvo_against_the_rule(&ordered, &[1, 2], SHARE);
        for salvo in salvos_of(&ordered.events, &[1, 2]) {
            assert!(salvo.ready, "seed {seed}");
            if salvo.weapon == Weapon::Gun {
                assert_eq!(named(&salvo.factors, "unready_gun_share"), None);
                if salvo.night() && salvo.round == 1 {
                    assert_eq!(salvo.shots, full_salvo(salvo.factor("main_battery_factor")));
                }
                b_gun_salvos += usize::from(force_of_ship(salvo.firer) == 2);
            }
        }
        // The rolls and salvos are scenario 4a's with B at battle readiness
        // (on the frozen figures, where the share is 1), seed for seed.
        let reference = run_of(&at_start_frozen, seed);
        let summary = |run: &Run| {
            (
                rolls_of(&run.events)
                    .iter()
                    .map(|r| (r.round, r.force, r.chance, r.sighted))
                    .collect::<Vec<_>>(),
                salvos_of(&run.events, &[1, 2])
                    .iter()
                    .map(|s| (s.round, s.firer, s.weapon, s.shots, s.chance, s.hits))
                    .collect::<Vec<_>>(),
            )
        };
        assert_eq!(summary(&ordered), summary(&reference), "seed {seed}");
        // And a force at battle readiness from the start: the whole log is
        // the same at the working share and at 1, line for line.
        let started = run_of(&at_start_working, seed);
        assert_eq!(started.events, reference.events, "seed {seed}");
    }
    assert!(b_gun_salvos > 300, "B fired: {b_gun_salvos}");
}

/// (c) Force B at normal readiness, alert since before the pulse of the
/// fight: B at (7,0), 280 nm from A, with the Allied side holding one contact
/// there from the start. See the doc comment.
fn text_c(with_contact: bool) -> String {
    let mut text = common::replace_once(
        &text_of(&scenario_path()),
        "q = 5\nr = 0\nmission = \"surface_combat\"\n",
        "q = 7\nr = 0\nmission = \"surface_combat\"\n",
    );
    if with_contact {
        text.push_str(
            "\n[[contact]]\nside = \"allied\"\nq = 7\nr = 0\nship_count = 3\nkind = \"cruiser\"\n",
        );
    }
    text
}

#[test]
fn b_alert_since_before_the_pulse_is_ready_and_fires_the_full_salvo() {
    let alerted = from_text(&text_c(true), &working_options(None));
    let control = from_text(&text_c(false), &working_options(None));
    let (mut with_contact, mut without) = (0, 0);
    for seed in 1..=300 {
        // With the contact: B is alerted at the start of the night pulse, the
        // fight is in the morning, by day. B is ready: 120 x the battery
        // factor, no factor; sighting 1 (day) x 1 (alert) in round 1.
        let run = run_of(&alerted, seed);
        let first_fight_pulse = fight_pulse(&run.events);
        assert_eq!(first_fight_pulse, Pulse::Morning, "seed {seed}");
        let alert_events: Vec<&Event> = run
            .events
            .iter()
            .filter(|e| {
                matches!(&e.kind, EventKind::ForceAlertedByContact { task_force, .. } if task_force.raw() == 2)
            })
            .collect();
        assert_eq!(alert_events.len(), 1, "seed {seed}");
        assert_eq!(alert_events[0].pulse, Some(Pulse::Night));
        let salvos = salvos_of(&run.events, &[1]);
        let first_fight: Vec<&Salvo> = salvos
            .iter()
            .filter(|s| s.pulse == Some(Pulse::Morning) && s.day == 1)
            .collect();
        for salvo in &first_fight {
            assert!(salvo.ready, "seed {seed}");
            assert_eq!(named(&salvo.factors, "unready_gun_share"), None);
            if salvo.weapon == Weapon::Gun {
                assert_eq!(salvo.shots, full_salvo(salvo.factor("main_battery_factor")));
                // By day: 0.05, and no one is surprised.
                assert_eq!(salvo.factor("base_chance"), fx("0.05"));
                assert_eq!(salvo.factor("surprised_factor"), Fixed::ONE);
                if salvo.shots == Fixed::from_int(120)
                    && salvo.factor("fire_control_factor") == Fixed::ONE
                {
                    assert_eq!(salvo.expected, fx("6"));
                    with_contact += 1;
                }
            } else {
                assert_eq!(salvo.shots, Fixed::from_int(8));
                assert_eq!(salvo.expected, fx("0.16"));
            }
        }
        let round_1: Vec<Roll> = rolls_of(&run.events)
            .into_iter()
            .filter(|r| r.pulse == Some(Pulse::Morning) && r.round == 1)
            .collect();
        for roll in &round_1 {
            assert_eq!(roll.chance, Fixed::ONE, "seed {seed}: both sight for sure");
            assert!(roll.sighted);
        }

        // The control, no contact: not alerted, not ready, sights at 0.5, and
        // fires 60 x the battery factor in every round.
        let run = run_of(&control, seed);
        let salvos = salvos_of(&run.events, &[1]);
        let rolls: Vec<Roll> = rolls_of(&run.events)
            .into_iter()
            .filter(|r| r.pulse == Some(Pulse::Morning) && r.force == 2 && r.round == 1)
            .collect();
        assert_eq!(rolls.len(), 1, "seed {seed}");
        assert_eq!(rolls[0].chance, fx("0.5"));
        for salvo in salvos.iter().filter(|s| {
            s.pulse == Some(Pulse::Morning) && s.day == 1 && force_of_ship(s.firer) == 2
        }) {
            assert!(!salvo.ready, "seed {seed}");
            if salvo.weapon == Weapon::Gun {
                assert_eq!(
                    salvo.shots,
                    full_salvo(salvo.factor("main_battery_factor")) * SHARE,
                    "seed {seed}"
                );
                assert_eq!(salvo.factor("unready_gun_share"), SHARE);
                without += 1;
            }
        }
        check_every_salvo_against_the_rule(&run, &[1], SHARE);
    }
    assert!(with_contact > 300, "{with_contact}");
    assert!(without > 300, "{without}");
}

/// The pulse of the first fight of a log.
fn fight_pulse(events: &[Event]) -> Pulse {
    events
        .iter()
        .find(|e| matches!(e.kind, EventKind::EngagementBegun { .. }))
        .and_then(|e| e.pulse)
        .expect("a fight began")
}

/// (d) Force B at normal readiness, alerted in the pulse of the fight by the
/// alarm of another fight in the hex: a third force, C, ships 7 to 9, lies in
/// (5,0) with B. See the doc comment.
fn text_d() -> String {
    let mut text = text_of(&scenario_path());
    text.push_str(
        "
[[task_force]]
id = 3
name = \"Placeholder second defending force\"
side = \"allied\"
q = 5
r = 0
mission = \"surface_combat\"

[[order]]
day = 1
side = \"allied\"
task_force = 3
kind = \"set_rules_of_engagement\"
stance = \"engage\"
break_off = \"0.5\"
",
    );
    for ship in 7..=9 {
        text.push_str(&format!(
            "
[[ship]]
id = {ship}
name = \"Placeholder cruiser C{}\"
class = 1
side = \"allied\"
fuel_t = 2000
task_force = 3
",
            ship - 6
        ));
    }
    text
}

#[test]
fn a_force_alarmed_in_the_pulse_is_not_ready_and_fires_part_of_its_guns_with_the_alert_sighting() {
    let scenario = from_text(&text_d(), &working_options(Some("1")));
    let (mut c_salvos, mut alarmed_runs, mut c_fights, mut unseen) = (0, 0, 0, 0);
    for seed in 1..=300 {
        let run = run_of(&scenario, seed);
        check_every_salvo_against_the_rule(&run, &[1], SHARE);
        let alarms: Vec<&Event> = run
            .events
            .iter()
            .filter(|e| {
                matches!(&e.kind, EventKind::AlarmRoll { task_force, alarmed: true, .. } if task_force.raw() == 3)
            })
            .collect();
        // The sighting of C in round 1 of its fight: 0.5 if the alarm came
        // before it (an alert force), 0.25 if it did not.
        let c_rolls: Vec<&Event> = run
            .events
            .iter()
            .filter(|e| {
                e.pulse == Some(Pulse::Night) && matches!(&e.kind, EventKind::SightingRoll { task_force, round: 1, .. } if task_force.raw() == 3)
            })
            .collect();
        for roll in &c_rolls {
            let alarmed_before = alarms.iter().any(|alarm| alarm.id < roll.id);
            let EventKind::SightingRoll { chance, .. } = roll.kind else {
                unreachable!();
            };
            assert_eq!(
                chance,
                if alarmed_before {
                    fx("0.5")
                } else {
                    fx("0.25")
                },
                "seed {seed}"
            );
            assert_eq!(
                named(&factors_of(roll), "alert_factor"),
                Some(if alarmed_before {
                    Fixed::ONE
                } else {
                    fx("0.5")
                })
            );
            c_fights += 1;
        }
        alarmed_runs += i64::from(!alarms.is_empty());
        unseen += i64::from(alarms.is_empty());
        // C's gun salvos in the night: 60 x the battery factor, in every
        // round, alarmed or not (never ready), with the factor named. (A
        // later fight by day is checked against the rule above.)
        for salvo in salvos_of(&run.events, &[1]) {
            if force_of_ship(salvo.firer) == 3 && salvo.weapon == Weapon::Gun && salvo.night() {
                assert!(!salvo.ready, "seed {seed}");
                assert_eq!(salvo.factor("unready_gun_share"), SHARE);
                if salvo.round == 1 || salvo.shots > Fixed::ZERO {
                    c_salvos += 1;
                }
            }
        }
        // If C was alarmed, it was alerted by that event.
        if let Some(alarm) = alarms.first() {
            assert_eq!(alarm.pulse, Some(Pulse::Night));
        }
    }
    eprintln!("(d): {alarmed_runs} runs with the alarm, {unseen} without; C fought in {c_fights}");
    assert!(alarmed_runs > 290, "the alarm is sure: {alarmed_runs}");
    assert!(c_salvos > 1000, "C fired: {c_salvos}");
    assert!(c_fights > 250, "C's fight was run: {c_fights}");
}

// ---------------------------------------------------------------------
// Part 3: a pinned outcome on the live figures.
// ---------------------------------------------------------------------

/// PINNED, not calculated. On the live figures (the share 0.5, experience's
/// strengths 0.25 with every ship at 2, so a factor of 1), seed 1942, the
/// scenario's own. No hand calculation can say which draw succeeds; this
/// catches a stream that moved, which the properties above would miss. It
/// follows the live figures: when a figure in
/// `data/reference/surface_combat.toml` is tuned, or a rule is changed on
/// purpose, it is remade with the change, and never to make a test pass.
#[test]
fn pinned_outcome_of_seed_1942_on_the_live_figures() {
    let base = common::load_live(&scenario_path()).expect("scenario 15a loads on live figures");
    let run = run_of(&base, 1942);
    let rolls = night_rolls(&run);
    let (way, first_round) = opening(&rolls);
    let salvos = night_salvos(&run, &[1]);
    let summary = Pinned {
        way,
        first_round,
        rolls: rolls.len(),
        salvos: salvos.len(),
        b_gun_shots: salvos
            .iter()
            .filter(|s| s.weapon == Weapon::Gun && force_of_ship(s.firer) == 2)
            .map(|s| s.shots.trunc())
            .sum(),
        hits_a: hits_of(&salvos, 1),
        hits_b: hits_of(&salvos, 2),
        events: run.events.len(),
    };
    eprintln!("{summary:?}");
    assert_eq!(summary, PINNED);
}

fn hits_of(salvos: &[Salvo], force: u32) -> u32 {
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
    b_gun_shots: i64,
    hits_a: u32,
    hits_b: u32,
    events: usize,
}

const PINNED: Pinned = Pinned {
    way: Opening::AFreeRound,
    first_round: Some(1),
    rolls: 2,
    salvos: 15,
    b_gun_shots: 120,
    hits_a: 38,
    hits_b: 4,
    events: 74,
};
