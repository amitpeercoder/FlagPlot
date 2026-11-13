//! Scenario 15b: a force that is not ready fires part of its torpedoes.
//!
//! Decision 0071 (`docs/decisions/0071-a-force-not-ready-fires-part-of-its-torpedoes.md`):
//! a force that is not ready when a fight begins (decision 0068's `ready`,
//! unchanged) fires its torpedo shots at `unready_torpedo_share`. Torpedo
//! shots = tubes x torpedo mount factor x the share. The ship's torpedoes are
//! then spent as before (decision 0016, 6). This scenario
//! (`data/scenarios/15b_unready_torpedoes.toml`) is scenario 15a exactly,
//! which is scenario 4a: force A, three placeholder cruisers, at battle
//! readiness; force B, three, at normal readiness, stopped, and never alerted
//! before the fight. Everything not said here is the hand calculation of
//! scenario 15a (`tests/scenario_15a_unready_fire.rs`) and of scenario 4a
//! (`tests/scenario_04a_night_action.rs`).
//!
//! The share is 1 in the frozen figures (`data/reference/step1`), where the
//! rule does nothing. The working values here are `unready_gun_share` 0.5
//! and `unready_torpedo_share` 0.5, set through an altered copy of the frozen
//! figures (`common::altered_reference`), as scenario 15a sets its gun share;
//! so experience's strengths are 0 and every other number is scenario 4a's.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! The inputs, checked against the scenario and the frozen figures: class 1,
//! the placeholder cruiser, has 8 tubes; `torpedo_hit_chance_night` 0.04;
//! `torpedo_hit_chance_day` 0.02; `degraded_factor` 0.5 (a degraded mount);
//! the experience strengths 0. Every other factor is 1 in the cases below
//! (the target not surprised and not lit).
//!
//! - **Unready force, mounts sound, at night:** shots 8 x 1 x 0.5 = 4;
//!   expected hits 4 x 0.04 = 0.16. The ship's `torpedoes_ready` is false
//!   afterwards: the four tubes that did not fire are not kept.
//! - **Unready force, mounts degraded, at night:** shots 8 x 0.5 x 0.5 = 2;
//!   expected hits 2 x 0.04 = 0.08.
//! - **The same force at battle readiness by order, at night:** 8 shots;
//!   expected 8 x 0.04 = 0.32, as scenario 15a has it. The breakdown has no
//!   `unready_torpedo_share` factor.
//! - **Alerted in the pulse before (scenario 15a's case (c) set-up):** force
//!   B is alerted at the start of the night pulse by a contact held from the
//!   start, and fights in the morning pulse, by day, at the day chance 0.02.
//!   Ready B: 8 x 1 x 1 = 8 shots; expected 8 x 0.02 = 0.16; no
//!   `unready_torpedo_share` factor in the breakdown, as 15a asserts today.
//!   Its control (no contact, not ready, by day): 8 x 1 x 0.5 = 4 shots;
//!   expected 4 x 0.02 = 0.08; the factor in the breakdown. This also shows
//!   that the share holds by day, as the gun share does.
//! - **What the enemy sees:** the enemy's view of the salvo (`SalvoAgainst`)
//!   has no such factor; the firer's own view of its `Salvo` has it.
//! - **With the key at 1** (the frozen figures) the event log of 15a is
//!   unchanged byte for byte, and no breakdown names the factor.
//!
//! # The three parts
//!
//! As the spec asks (`docs/step2-spec.md`, "Tests while figures move", 7):
//!
//! 1. The chance and the shots above, read from the events' breakdowns at the
//!    working value of 0.5.
//! 2. Properties over many seeds: the torpedo hits drawn stay close to what
//!    the expected hits say (0.16 a salvo for an unready force with sound
//!    mounts, by the same four-sigma test as scenario 15a); every torpedo
//!    salvo of a force not ready names the share and has tubes x mount factor
//!    x share shots, and no salvo of a ready force names it; no enemy view
//!    carries it; and lowering the share never raises the firing force's
//!    torpedo hits.
//! 3. One pinned outcome for one seed on the live figures, labelled as pinned:
//!    no hand calculation can say which draw succeeds. It follows the live
//!    figures and is remade with them.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::ids::TaskForceId;
use sim_core::side_events::SideEventKind;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_data::{LoadOptions, Scenario};

const SEEDS: u64 = 1000;
/// Written out from the hand calculation, not read from the data files.
const SHARE: Fixed = Fixed::from_raw(500_000);
const TORPEDO_BASE_NIGHT: Fixed = Fixed::from_raw(40_000);
const TORPEDO_BASE_DAY: Fixed = Fixed::from_raw(20_000);
const SURPRISED: Fixed = Fixed::from_raw(1_500_000);
const LIT: Fixed = Fixed::from_raw(2_000_000);
const FACTOR: &str = "unready_torpedo_share";

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/15b_unready_torpedoes.toml")
}

fn path_of(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../data/scenarios")
        .join(name)
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

/// The frozen figures with the gun share at 0.5 and the torpedo share at
/// `torpedo_share`.
fn options_with(torpedo_share: &str) -> LoadOptions {
    common::altered_reference("unready_torpedo", |dir| {
        common::edit_file_by(
            dir,
            "surface_combat.toml",
            &[
                ("unready_gun_share = \"1\"", "unready_gun_share = \"0.5\""),
                (
                    "unready_torpedo_share = \"1\"",
                    &format!("unready_torpedo_share = \"{torpedo_share}\""),
                ),
            ],
        );
    })
}

fn text_of(path: &std::path::Path) -> String {
    std::fs::read_to_string(path).expect("the scenario file reads")
}

fn working() -> Scenario {
    Scenario::load_with(&scenario_path(), &options_with("0.5")).expect("scenario 15b loads")
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

/// Task force 1 is ships 1 to 3, task force 2 ships 4 to 6.
fn force_of_ship(ship: u32) -> u32 {
    assert!(
        (1..=6).contains(&ship),
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
    /// began, worked from the log by [`salvos_of`], not from the salvo's own
    /// breakdown.
    ready: bool,
}

impl Salvo {
    fn factor(&self, name: &str) -> Fixed {
        named(&self.factors, name).unwrap_or_else(|| panic!("a salvo has no factor {name}"))
    }

    fn night(&self) -> bool {
        self.pulse == Some(Pulse::Night)
    }

    fn names(&self) -> Vec<&'static str> {
        self.factors.iter().map(|f| f.0).collect()
    }
}

/// Every salvo of the log, with whether its force was ready as its fight
/// began. The rule is worked here from the events alone, as decision 0068
/// states it: a force is ready if it is at battle readiness (the forces in
/// `battle`, by order or from the start) or was alert before the pulse of the
/// fight began.
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

/// One run: its log, and what each side was shown of it.
struct Run {
    seed: u64,
    events: Vec<Event>,
    /// For each side: the events it was shown with the factor in the
    /// breakdown, and the factor's value.
    shown_with_factor: Vec<(Side, sim_core::ids::EventId, Fixed)>,
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
            if let SideEventKind::SalvoAgainst { .. } = event.kind {
                against_shown += 1;
                against_with_breakdown += usize::from(event.breakdown.is_some());
            }
            if let Some(breakdown) = &event.breakdown {
                for factor in &breakdown.factors {
                    if factor.name == FACTOR {
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

fn night_salvos(run: &Run, battle: &[u32]) -> Vec<Salvo> {
    salvos_of(&run.events, battle)
        .into_iter()
        .filter(Salvo::night)
        .collect()
}

fn torpedo_salvos(salvos: Vec<Salvo>) -> Vec<Salvo> {
    salvos
        .into_iter()
        .filter(|s| s.weapon == Weapon::Torpedo)
        .collect()
}

/// Whether k out of n is within four standard deviations of probability `p`
/// (in millionths), in exact integer arithmetic: (k - np)^2 <= 16np(1 - p).
#[allow(dead_code)]
fn within_four_sigma(k: i64, n: i64, p: i64) -> bool {
    let (k, n, p) = (i128::from(k), i128::from(n), i128::from(p));
    let scale = 1_000_000;
    let off = k * scale - n * p;
    off * off <= 16 * n * p * (scale - p)
}

/// The names of a torpedo salvo's first five factors, the sixth being the
/// base chance for a force not ready.
const NOT_READY_NAMES: [&str; 6] = [
    "tubes",
    "rate",
    "minutes",
    "torpedo_mount_factor",
    FACTOR,
    "base_chance",
];

/// Checks a torpedo salvo against the rule, whatever the figures: shots are
/// tubes x mount factor x share (1 for a ready force), and the factor is in
/// the breakdown straight after the mount factor if and only if the force was
/// not ready.
fn check_torpedo_salvo(salvo: &Salvo, share: Fixed, seed: u64) {
    let tubes = salvo.factor("tubes");
    let mount = salvo.factor("torpedo_mount_factor");
    if salvo.ready {
        assert_eq!(named(&salvo.factors, FACTOR), None, "seed {seed}");
        assert_eq!(salvo.shots, tubes * mount, "seed {seed}");
    } else {
        assert_eq!(
            salvo.names()[..6],
            NOT_READY_NAMES,
            "seed {seed} ship {} round {}",
            salvo.firer,
            salvo.round
        );
        assert_eq!(salvo.factor(FACTOR), share, "seed {seed}");
        assert_eq!(salvo.shots, tubes * mount * share, "seed {seed}");
    }
    assert_eq!(salvo.expected, salvo.shots * salvo.chance, "seed {seed}");
}

// ---------------------------------------------------------------------
// Part 1: the shots and chances at the working value, from the breakdowns.
// ---------------------------------------------------------------------

#[test]
fn the_shots_and_chances_at_the_working_value_are_the_hand_calculated_ones() {
    // Cells seen: [A, B with sound mounts, B with degraded mounts] x
    // [plain, surprised, lit].
    let mut seen = [[false; 3]; 3];
    // Not ready, sound mounts, plain: 4 x 0.04 = 0.16; surprised 4 x 0.06 =
    // 0.24; lit 4 x 0.08 = 0.32. Ready: 8 x 0.04 = 0.32, 0.48, 0.64. Not
    // ready, degraded mounts: 2 x 0.04 = 0.08, 0.12, 0.16.
    let expected = [
        ["0.32", "0.48", "0.64"],
        ["0.16", "0.24", "0.32"],
        ["0.08", "0.12", "0.16"],
    ];
    let shots = [8, 4, 2];
    for run in runs() {
        for salvo in torpedo_salvos(night_salvos(run, &[1])) {
            let force = force_of_ship(salvo.firer);
            let surprised = salvo.factor("surprised_factor");
            let lit = salvo.factor("lit_factor");
            assert!(surprised == Fixed::ONE || surprised == SURPRISED);
            assert!(lit == Fixed::ONE || lit == LIT);
            assert!(!(surprised == SURPRISED && lit == LIT));
            assert_eq!(salvo.factor("base_chance"), TORPEDO_BASE_NIGHT);
            assert_eq!(
                salvo.chance,
                TORPEDO_BASE_NIGHT * surprised * lit,
                "seed {}",
                run.seed
            );
            // Force A is ready, force B is not, at night.
            assert_eq!(salvo.ready, force == 1, "seed {}", run.seed);
            check_torpedo_salvo(&salvo, SHARE, run.seed);
            let slot = usize::from(surprised == SURPRISED) + 2 * usize::from(lit == LIT);
            let mount = salvo.factor("torpedo_mount_factor");
            let row = if force == 1 {
                (mount == Fixed::ONE).then_some(0)
            } else if mount == Fixed::ONE {
                Some(1)
            } else if mount == fx("0.5") {
                Some(2)
            } else {
                None
            };
            if let Some(row) = row {
                assert_eq!(
                    salvo.shots,
                    Fixed::from_int(shots[row]),
                    "seed {}",
                    run.seed
                );
                assert_eq!(salvo.expected, fx(expected[row][slot]), "seed {}", run.seed);
                seen[row][slot] = true;
            }
        }
    }
    for (row, cells) in seen.iter().enumerate() {
        eprintln!(
            "row {row}: plain {} surprised {} lit {}",
            cells[0], cells[1], cells[2]
        );
    }
    // The unsurprised, unlit cell of each row was met; B's with sound mounts
    // and the surprised cell of A and B too.
    assert!(seen[0][0] && seen[1][0] && seen[2][0]);
    assert!(seen[0][1] && seen[1][1]);
}

// ---------------------------------------------------------------------
// Part 2: properties over many seeds.
// ---------------------------------------------------------------------

#[test]
fn every_torpedo_salvo_of_a_force_not_ready_is_tubes_times_mount_times_the_share() {
    let (mut not_ready, mut ready) = (0, 0);
    for run in runs() {
        let mut fired: BTreeMap<u32, u32> = BTreeMap::new();
        for salvo in torpedo_salvos(salvos_of(&run.events, &[1])) {
            check_torpedo_salvo(&salvo, SHARE, run.seed);
            if salvo.ready {
                ready += 1;
            } else {
                not_ready += 1;
            }
            // A is always ready, and B is not in the night fight.
            if force_of_ship(salvo.firer) == 1 {
                assert!(salvo.ready);
            }
            if salvo.night() && force_of_ship(salvo.firer) == 2 {
                assert!(!salvo.ready);
            }
            *fired.entry(salvo.firer).or_default() += 1;
        }
        // The torpedoes are spent after the first round fired, at a share or
        // not: no ship fires a second torpedo salvo in the run.
        assert!(fired.values().all(|n| *n == 1), "seed {}", run.seed);
    }
    eprintln!("torpedo salvos not ready: {not_ready}, ready: {ready}");
    assert!(not_ready > 1000, "the rule was checked: {not_ready}");
    assert!(ready > 1000, "ready salvos were checked: {ready}");
}

#[test]
fn the_guns_keep_their_own_share_and_no_gun_salvo_names_the_torpedo_share() {
    for run in runs() {
        for salvo in salvos_of(&run.events, &[1]) {
            if salvo.weapon == Weapon::Gun {
                assert_eq!(named(&salvo.factors, FACTOR), None, "seed {}", run.seed);
                assert_eq!(
                    named(&salvo.factors, "unready_gun_share").is_some(),
                    !salvo.ready,
                    "seed {}",
                    run.seed
                );
            }
        }
    }
}

#[test]
fn the_torpedo_hits_stay_within_four_sigma_of_what_the_chances_expect() {
    // Over every night torpedo salvo: (H - E)^2 <= 16 x the sum of np(1 - p),
    // for each force, in exact arithmetic (n and p are the event's). The shots
    // are whole here (4, 2 or 8), so n is exact.
    let mut hits = [0i128; 2];
    let mut expected6 = [0i128; 2];
    let mut variance12 = [0i128; 2];
    for run in runs() {
        for salvo in torpedo_salvos(night_salvos(run, &[1])) {
            let slot = usize::from(force_of_ship(salvo.firer) == 2);
            let n = i128::from(salvo.shots.trunc());
            assert_eq!(salvo.shots, Fixed::from_int(salvo.shots.trunc()), "whole");
            let p = i128::from(salvo.chance.raw());
            hits[slot] += i128::from(salvo.hits);
            expected6[slot] += n * p;
            variance12[slot] += n * p * (1_000_000 - p);
        }
    }
    for slot in 0..2 {
        let off6 = hits[slot] * 1_000_000 - expected6[slot];
        assert!(
            off6 * off6 <= 16 * variance12[slot],
            "slot {slot}: hits {} expected (millionths) {}",
            hits[slot],
            expected6[slot]
        );
        eprintln!(
            "force {}: torpedo hits {}, expected {}",
            slot + 1,
            hits[slot],
            Fixed::from_raw(i64::try_from(expected6[slot]).expect("small"))
        );
    }
}

#[test]
fn lowering_the_torpedo_share_never_raises_the_firing_forces_torpedo_hits() {
    // The hits of a salvo are the whole part of the expected hits and one roll
    // on the fraction; all of a force's ships fire their torpedoes in their
    // first firing round from the same state, so the draw is in the same
    // place in the stream and a smaller fraction can only lose the roll.
    let full = Scenario::load_with(&scenario_path(), &options_with("1")).expect("loads");
    let part = working();
    let b_hits = |run: &Run| -> u32 {
        torpedo_salvos(salvos_of(&run.events, &[1]))
            .iter()
            .filter(|s| force_of_ship(s.firer) == 2)
            .map(|s| s.hits)
            .sum()
    };
    let (mut total_full, mut total_part) = (0, 0);
    for seed in 1..=300 {
        let (with_full, with_part) = (run_of(&full, seed), run_of(&part, seed));
        let (f, p) = (b_hits(&with_full), b_hits(&with_part));
        assert!(
            p <= f,
            "seed {seed}: B's torpedo hits {p} at 0.5 against {f} at 1"
        );
        total_full += f;
        total_part += p;
    }
    eprintln!("B's torpedo hits over 300 seeds: {total_part} at 0.5, {total_full} at 1");
    assert!(total_part < total_full);
}

#[test]
fn with_the_torpedo_share_at_1_the_event_log_is_scenario_15as_and_4as_line_for_line() {
    // The frozen figures hold 1, so 15b on them must be 15a on them, and 4a:
    // the same events, with the same breakdowns (none names the factor).
    let b = frozen(&scenario_path());
    let a = frozen(&path_of("15a_unready_fire.toml"));
    let reference = frozen(&path_of("04a_night_action.toml"));
    for seed in 1..=100 {
        let log = |scenario: &Scenario| -> Vec<Event> {
            run_seed(scenario, seed)
                .world
                .events()
                .iter()
                .cloned()
                .collect()
        };
        let (mine, of_a, of_4a) = (log(&b), log(&a), log(&reference));
        assert_eq!(mine.len(), of_a.len(), "seed {seed}");
        assert_eq!(mine.len(), of_4a.len(), "seed {seed}");
        for ((x, y), z) in mine.iter().zip(&of_a).zip(&of_4a) {
            assert_eq!(x, y, "seed {seed}");
            assert_eq!(x, z, "seed {seed}");
        }
        assert!(mine.iter().all(|e| named(&factors_of(e), FACTOR).is_none()));
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
            // The event is the side's own torpedo salvo, and the value is its
            // own.
            match &event.kind {
                EventKind::Salvo { firer, weapon, .. } => {
                    assert_eq!(*weapon, Weapon::Torpedo, "seed {}", run.seed);
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
// The other ways to be ready, and not.
// ---------------------------------------------------------------------

/// (b) Force B ordered to battle readiness on day 1.
fn text_b_ordered_battle() -> String {
    let mut text = text_of(&scenario_path());
    text.push_str(
        "\n[[order]]\nday = 1\nside = \"allied\"\ntask_force = 2\nkind = \"set_readiness\"\nreadiness = \"battle\"\n",
    );
    text
}

#[test]
fn b_ordered_to_battle_readiness_is_ready_and_fires_all_its_tubes() {
    let by_order = from_text(&text_b_ordered_battle(), &options_with("0.5"));
    let (mut b_plain, mut b_salvos) = (0, 0);
    for seed in 1..=300 {
        let run = run_of(&by_order, seed);
        for salvo in torpedo_salvos(salvos_of(&run.events, &[1, 2])) {
            assert!(salvo.ready, "seed {seed}");
            check_torpedo_salvo(&salvo, SHARE, seed);
            if salvo.night() && force_of_ship(salvo.firer) == 2 {
                b_salvos += 1;
                let (surprised, lit) =
                    (salvo.factor("surprised_factor"), salvo.factor("lit_factor"));
                assert_eq!(salvo.chance, TORPEDO_BASE_NIGHT * surprised * lit);
                if salvo.factor("torpedo_mount_factor") == Fixed::ONE {
                    assert_eq!(salvo.shots, Fixed::from_int(8), "seed {seed}");
                    if surprised == Fixed::ONE && lit == Fixed::ONE {
                        assert_eq!(salvo.expected, fx("0.32"), "seed {seed}");
                        b_plain += 1;
                    }
                }
            }
        }
    }
    eprintln!("(b): B night torpedo salvos {b_salvos}, plain {b_plain}");
    assert!(b_plain > 50, "{b_plain}");
}

/// (c) Force B at normal readiness, alert since before the pulse of the
/// fight: B at (7,0), 280 nm from A, with the Allied side holding one contact
/// there from the start (scenario 15a's set-up).
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
fn b_alert_since_before_the_pulse_fires_8_by_day_and_the_control_fires_4() {
    let alerted = from_text(&text_c(true), &options_with("0.5"));
    let control = from_text(&text_c(false), &options_with("0.5"));
    let (mut with_contact, mut without) = (0, 0);
    for seed in 1..=300 {
        // With the contact: B is alerted in the night pulse, the fight is in
        // the morning, by day. B is ready: 8 shots, 8 x 0.02 = 0.16, no
        // factor.
        let run = run_of(&alerted, seed);
        for salvo in torpedo_salvos(salvos_of(&run.events, &[1])) {
            if salvo.pulse != Some(Pulse::Morning) || salvo.day != 1 {
                continue;
            }
            assert!(salvo.ready, "seed {seed}");
            check_torpedo_salvo(&salvo, SHARE, seed);
            assert_eq!(salvo.factor("base_chance"), TORPEDO_BASE_DAY);
            assert_eq!(salvo.factor("surprised_factor"), Fixed::ONE);
            if salvo.factor("torpedo_mount_factor") == Fixed::ONE {
                assert_eq!(salvo.shots, Fixed::from_int(8), "seed {seed}");
                assert_eq!(salvo.expected, fx("0.16"), "seed {seed}");
                if force_of_ship(salvo.firer) == 2 {
                    with_contact += 1;
                }
            }
        }
        // The control, no contact: not ready: 4 shots, 4 x 0.02 = 0.08, the
        // factor named.
        let run = run_of(&control, seed);
        for salvo in torpedo_salvos(salvos_of(&run.events, &[1])) {
            if salvo.pulse != Some(Pulse::Morning)
                || salvo.day != 1
                || force_of_ship(salvo.firer) != 2
            {
                continue;
            }
            assert!(!salvo.ready, "seed {seed}");
            check_torpedo_salvo(&salvo, SHARE, seed);
            assert_eq!(salvo.factor("base_chance"), TORPEDO_BASE_DAY);
            if salvo.factor("torpedo_mount_factor") == Fixed::ONE {
                assert_eq!(salvo.shots, Fixed::from_int(4), "seed {seed}");
                assert_eq!(salvo.expected, fx("0.08"), "seed {seed}");
                without += 1;
            }
        }
    }
    eprintln!("(c): ready B torpedo salvos {with_contact}, control {without}");
    assert!(with_contact > 300, "{with_contact}");
    assert!(without > 300, "{without}");
}

// ---------------------------------------------------------------------
// Part 3: a pinned outcome on the live figures.
// ---------------------------------------------------------------------

/// PINNED, not calculated. On the live figures (both shares 0.5, experience's
/// strengths 0.25 with every ship at 2, so a factor of 1), seed 1942, the
/// scenario's own. No hand calculation can say which draw succeeds; this
/// catches a stream that moved, which the properties above would miss. It
/// follows the live figures: when a figure in
/// `data/reference/surface_combat.toml` is tuned, or a rule is changed on
/// purpose, it is remade with the change, and never to make a test pass.
#[test]
fn pinned_outcome_of_seed_1942_on_the_live_figures() {
    let base = common::load_live(&scenario_path()).expect("scenario 15b loads on live figures");
    let run = run_of(&base, 1942);
    let salvos = night_salvos(&run, &[1]);
    let torpedoes = torpedo_salvos(salvos.clone());
    let summary = Pinned {
        salvos: salvos.len(),
        b_torpedo_shots: torpedoes
            .iter()
            .filter(|s| force_of_ship(s.firer) == 2)
            .map(|s| s.shots.trunc())
            .sum(),
        torpedo_hits_a: hits_of(&torpedoes, 1),
        torpedo_hits_b: hits_of(&torpedoes, 2),
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
    salvos: usize,
    b_torpedo_shots: i64,
    torpedo_hits_a: u32,
    torpedo_hits_b: u32,
    hits_a: u32,
    hits_b: u32,
    events: usize,
}

// Remade on Nov 13, 2026 with tuning round 2 (surprised_target_factor 1.5 to
// 2): A's hits in its free round rose; nothing else of the pin moved.
const PINNED: Pinned = Pinned {
    salvos: 15,
    b_torpedo_shots: 10,
    torpedo_hits_a: 1,
    torpedo_hits_b: 0,
    hits_a: 42,
    hits_b: 4,
    events: 74,
};
