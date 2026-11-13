//! Scenario 18: a ship with wing tubes fires one side at a time.
//!
//! Decision 0083 (`docs/decisions/0083-the-other-sides-torpedo-tubes.md`): a
//! class has two torpedo figures, `torpedo_tubes` (every tube aboard, T) and
//! `torpedo_tubes_one_side` (the tubes that bear on one side, B). A ship
//! fires B tubes in the first round in which she has torpedo shots to fire,
//! and the other T - B tubes in the next round in which she has torpedo shots
//! to fire, whether in the same fight or a later one. Then her torpedoes are
//! spent. Each salvo's shots are its tubes x the torpedo mount factor x
//! decision 0071's share (the share of the force in the fight the salvo is
//! fired in); its target is chosen by decision 0078 at the start of its own
//! round; the breakdown's tube count is the salvo's, B or T - B. A salvo with
//! shots above zero spends its side, however few tubes the share or a
//! degraded mount let fire. If the mounts are out when a side's turn comes,
//! nothing is fired and nothing is spent. The second side's salvo draws from
//! a stream of its own (`Draw::TorpedoOtherSide`).
//!
//! This scenario (`data/scenarios/18_torpedo_sides.toml`) is scenario 15b's
//! set-up (which is scenario 15a, which is scenario 4a) with force A's three
//! cruisers of class 8, "Placeholder cruiser with wing tubes": class 1 in
//! every figure (8 guns at 3 rounds a minute, `gun_damage` 20, toughness 400,
//! 8 tubes, `torpedo_damage` 200), except that `torpedo_tubes_one_side` is 4.
//! Force B's three cruisers are class 1 (B equal to T). Force A is at battle
//! readiness; force B is stopped, not alerted before the fight, and so not
//! ready when it begins.
//!
//! The figures are the frozen step 1 figures (`data/reference/step1`), with
//! `unready_torpedo_share` (and `unready_gun_share`, as scenario 15b has it)
//! at the working value 0.5 through an altered copy
//! (`common::altered_reference`); the experience strengths are 0.
//!
//! # Hand calculation
//!
//! Written by the architect before the code (decision 0083). The tests below
//! assert these numbers. If the code disagrees, the code or the rule is
//! wrong: do not change a number here to make a test pass.
//!
//! Figures read:
//!
//! | Figure | Value |
//! | --- | --- |
//! | `torpedo_hit_chance_night` | 0.04 |
//! | `torpedo_hit_chance_day` | 0.02 |
//! | `surprised_target_factor` | 1.5 |
//! | `degraded_factor` | 0.5 |
//! | `system_hit_chance_per_point` | 0.06 |
//! | `unready_torpedo_share` | 1 (0.5 where a case says so, by an altered copy of the frozen figures, as scenario 15b does) |
//! | Experience strengths | 0 |
//!
//! A torpedo hit from class 8 or class 1 on class 1 or 8 does
//! 100 x 200 / 400 = 50 points. Its system chance is min(1, 50 x 0.06) = 1.
//!
//! - **A. Class 8, ready, a night fight of two rounds or more, target not
//!   surprised.** Round 1, first side: tubes 4, shots 4 x 1 x 1 = 4, chance
//!   0.04, expected hits 0.16; afterwards ready, flag set. Round 2, second
//!   side, target by decision 0078 at the start of round 2: tubes 8 - 4 = 4,
//!   shots 4, chance 0.04, expected 0.16; afterwards spent, flag clear.
//! - **A'. As A, the target's force surprised in round 1.** Round 1: tubes 4,
//!   shots 4, chance 0.04 x 1.5 = 0.06, expected 0.24; ready, flag set.
//! - **B. Class 8, first fight ends after round 1; in the same pulse she meets
//!   a second enemy force.** First fight, round 1: 4 tubes, 4 shots, chance
//!   0.04, expected 0.16; ready, flag set. Second fight, her first firing
//!   round, at the most important kind afloat in place (i modulo their
//!   number): 4 tubes, 4 shots, chance 0.04, expected 0.16, drawn from the new
//!   stream keyed on her and the second force; spent. (Unit tests in
//!   `crates/sim-core/src/surface.rs`.)
//! - **B'. As B, the second meeting by day in the next pulse.** Second side:
//!   4 tubes, 4 shots, chance 0.02, expected 0.08; spent.
//! - **C. Class 8, force not ready, share 0.5, a night fight of two rounds.**
//!   Round 1: 4 tubes, shots 4 x 1 x 0.5 = 2, chance 0.04, expected 0.08;
//!   ready, flag set: the 2 unfired tubes of that side are not kept. Round 2:
//!   4 tubes, 2 shots, chance 0.04, expected 0.08; spent. Together 4, today's
//!   8 x 0.5.
//! - **D. Class 8, mounts degraded before round 1.** Round 1: 4 tubes, shots
//!   4 x 0.5 = 2, chance 0.04, expected 0.08; ready, flag set. Round 2, still
//!   degraded: 4 tubes, 2 shots, expected 0.08; spent.
//! - **D'. Class 8, mounts out after her first salvo.** Round 2: 0 shots, no
//!   salvo, no event, no draw; ready, flag set: the side waits. (Unit tests.)
//! - **E. Class 8 sunk in round 1 by fire of that round.** Round 1: 4 tubes,
//!   4 shots (a ship sunk in a round still fires in it), chance 0.04, expected
//!   0.16; sunk; no second salvo; the flag stays as it was on a sunk ship.
//!   (Unit tests.)
//! - **F. Class 1 (B equal to T).** Round 1: 8 tubes, 8 shots, chance 0.04,
//!   expected 0.32; spent, flag never set; round 2 has no torpedo salvo and
//!   the new stream is never opened. (Here for force B's ships, and in the
//!   unit tests.)
//! - **G. Class 8, a round 2 with no gun shots (ammunition gone).** Round 2:
//!   4 tubes, 4 shots, chance 0.04, expected 0.16; spent. She counts as
//!   having shots for the surprise test. (Unit tests.)
//!
//! The order of a round's draws, for class 8 in round 2 of case A: on her
//! fire stream (`Draw::Legacy`, keyed on her and the enemy force) her gun
//! salvo's 8 x 3 x 5 = 120 shot draws, then a track and a system draw for
//! each gun hit; then on `Draw::TorpedoOtherSide`, keyed on the day, the
//! pulse, her and the enemy force, 4 shot draws, then one system draw for
//! each torpedo hit. Damage lands at the end of the round in the order the
//! salvos were fired, gun salvo first, as now.
//!
//! What must not move: with class 8 in no scenario but this one, every
//! scenario on the frozen figures and every placeholder scenario gives its
//! event log byte for byte as before; only its state hashes move, by the new
//! field.
//!
//! # The three parts
//!
//! As the spec asks (`docs/step2-spec.md`, "Tests while figures move", 7):
//!
//! 1. The shots, chances and expected hits of cases A, A', C and D, read from
//!    the salvos' breakdowns at the working values.
//! 2. Properties over many seeds: every class 8 ship that fires twice fires B
//!    and then T - B, in different rounds; no class 1 ship ever fires two
//!    torpedo salvos; torpedo hits stay within four sigma of the expected
//!    hits; the sum of a class 8 ship's torpedo shots never exceeds 8 x the
//!    share.
//! 3. One pinned outcome for one seed on the live figures, labelled as
//!    pinned.
//!
//! Cases B, B', D', E, F and G are unit tests in
//! `crates/sim-core/src/surface.rs`, where a scenario cannot reach them.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::ids::TaskForceId;
use sim_core::time::Pulse;
use sim_data::{LoadOptions, Scenario};

const SEEDS: u64 = 1000;
const VARIANT_SEEDS: u64 = 300;
/// Written out from the hand calculation, not read from the data files.
const SHARE: Fixed = Fixed::from_raw(500_000);
const TORPEDO_BASE_NIGHT: Fixed = Fixed::from_raw(40_000);
const SURPRISED: Fixed = Fixed::from_raw(1_500_000);
const LIT: Fixed = Fixed::from_raw(2_000_000);
const DEGRADED: Fixed = Fixed::from_raw(500_000);
const FACTOR: &str = "unready_torpedo_share";

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/18_torpedo_sides.toml")
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

/// The frozen figures with the gun and torpedo shares at 0.5.
fn working_options() -> LoadOptions {
    common::altered_reference("torpedo_sides", |dir| {
        common::edit_file_by(
            dir,
            "surface_combat.toml",
            &[
                ("unready_gun_share = \"1\"", "unready_gun_share = \"0.5\""),
                (
                    "unready_torpedo_share = \"1\"",
                    "unready_torpedo_share = \"0.5\"",
                ),
            ],
        );
    })
}

fn text_of(path: &std::path::Path) -> String {
    std::fs::read_to_string(path).expect("the scenario file reads")
}

fn working() -> Scenario {
    Scenario::load_with(&scenario_path(), &working_options()).expect("scenario 18 loads")
}

fn from_text(text: &str) -> Scenario {
    Scenario::from_text_with(text, &scenario_path(), &working_options()).expect("variant loads")
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
    pulse: Option<Pulse>,
    round: u32,
    firer: u32,
    target: u32,
    weapon: Weapon,
    shots: Fixed,
    chance: Fixed,
    expected: Fixed,
    hits: u32,
    factors: Factors,
    /// Whether the firer's force was ready when the fight of this salvo
    /// began (decision 0068), worked from the log by [`salvos_of`].
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

/// Every salvo of the log, with whether its force was ready as its fight
/// began, worked from the events alone as decision 0068 states it: a force is
/// ready if it is at battle readiness (`battle`) or was alert before the
/// pulse of the fight began.
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
                target,
                weapon,
                shots,
                chance,
                expected,
                hits,
            } => {
                let ready = ready_in_fight
                    .as_ref()
                    .expect("a salvo is in a fight")
                    .get(&force_of_ship(firer.raw()))
                    .copied()
                    .expect("the firer's force is in the fight");
                salvos.push(Salvo {
                    pulse: event.pulse,
                    round: *round,
                    firer: firer.raw(),
                    target: target.raw(),
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

struct Run {
    seed: u64,
    events: Vec<Event>,
}

fn run_of(base: &Scenario, seed: u64) -> Run {
    let scenario = run_seed(base, seed);
    Run {
        seed,
        events: scenario.world.events().iter().cloned().collect(),
    }
}

fn runs() -> &'static Vec<Run> {
    static RUNS: OnceLock<Vec<Run>> = OnceLock::new();
    RUNS.get_or_init(|| {
        let base = working();
        (1..=SEEDS).map(|seed| run_of(&base, seed)).collect()
    })
}

fn torpedo_salvos(run: &Run) -> Vec<Salvo> {
    salvos_of(&run.events, &[1])
        .into_iter()
        .filter(|s| s.weapon == Weapon::Torpedo)
        .collect()
}

/// The torpedo salvos of each ship of the run, in the order fired.
fn by_ship(salvos: Vec<Salvo>) -> BTreeMap<u32, Vec<Salvo>> {
    let mut map: BTreeMap<u32, Vec<Salvo>> = BTreeMap::new();
    for salvo in salvos {
        map.entry(salvo.firer).or_default().push(salvo);
    }
    map
}

/// Checks a torpedo salvo against the rule, whatever the figures: shots are
/// the salvo's tubes x mount factor x the share (1 for a ready force), and
/// the expected hits are shots x chance.
fn check_torpedo_salvo(salvo: &Salvo, seed: u64) {
    let tubes = salvo.factor("tubes");
    let mount = salvo.factor("torpedo_mount_factor");
    if salvo.ready {
        assert_eq!(named(&salvo.factors, FACTOR), None, "seed {seed}");
        assert_eq!(salvo.shots, tubes * mount, "seed {seed}");
    } else {
        assert_eq!(salvo.factor(FACTOR), SHARE, "seed {seed}");
        assert_eq!(salvo.shots, tubes * mount * SHARE, "seed {seed}");
    }
    assert_eq!(salvo.expected, salvo.shots * salvo.chance, "seed {seed}");
}

// ---------------------------------------------------------------------
// Part 1: the cases, from the breakdowns.
// ---------------------------------------------------------------------

#[test]
fn cases_a_and_a_prime_class_8_ready_fires_4_and_then_4_at_0_04() {
    // Force A (ships 1 to 3, class 8) is ready, so its share is 1. Every one
    // of its night torpedo salvos has 4 tubes and 4 shots at a chance of
    // 0.04 x the surprised factor x the lit factor: expected 0.16 plain,
    // 0.24 against a surprised force, 0.32 against a lit ship.
    // Cells: [first side, second side] x [plain, surprised, lit].
    let mut seen = [[false; 3]; 2];
    let expected = [["0.16", "0.24", "0.32"], ["0.16", "0.24", "0.32"]];
    for run in runs() {
        let per_ship = by_ship(
            torpedo_salvos(run)
                .into_iter()
                .filter(|s| force_of_ship(s.firer) == 1)
                .collect(),
        );
        for (ship, salvos) in &per_ship {
            // A ship's salvos: B then T - B, at most two.
            assert!(salvos.len() <= 2, "seed {} ship {ship}", run.seed);
            for (index, salvo) in salvos.iter().enumerate() {
                // Only the night fight's salvos are in the cells (the first
                // side may be fired in a later fight, by day).
                if !salvo.night() {
                    continue;
                }
                assert!(salvo.ready, "seed {}", run.seed);
                assert_eq!(salvo.factor("tubes"), Fixed::from_int(4));
                let mount = salvo.factor("torpedo_mount_factor");
                if mount != Fixed::ONE {
                    continue;
                }
                assert_eq!(salvo.shots, Fixed::from_int(4), "seed {}", run.seed);
                let surprised = salvo.factor("surprised_factor");
                let lit = salvo.factor("lit_factor");
                assert!(surprised == Fixed::ONE || surprised == SURPRISED);
                assert!(lit == Fixed::ONE || lit == LIT);
                assert!(!(surprised == SURPRISED && lit == LIT));
                assert_eq!(salvo.factor("base_chance"), TORPEDO_BASE_NIGHT);
                assert_eq!(salvo.chance, TORPEDO_BASE_NIGHT * surprised * lit);
                let slot = usize::from(surprised == SURPRISED) + 2 * usize::from(lit == LIT);
                // The columns are plain, surprised, lit.
                let column = match slot {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    _ => continue,
                };
                assert_eq!(
                    salvo.expected,
                    fx(expected[index.min(1)][column]),
                    "seed {} ship {ship} salvo {index}",
                    run.seed
                );
                seen[index.min(1)][column] = true;
            }
        }
    }
    eprintln!("cells seen: {seen:?}");
    // Case A: both sides, plain. Case A': the surprised first side.
    assert!(seen[0][0] && seen[1][0], "A: first and second side, plain");
    assert!(seen[0][1], "A': the first side against a surprised force");
}

#[test]
fn force_b_class_1_fires_its_8_tubes_once_at_the_share_and_never_two_salvos() {
    // Case F, in the scenario: force B's class 1 cruisers have B equal to T.
    // At night, not ready, share 0.5: 8 tubes, 8 x 1 x 0.5 = 4 shots, chance
    // 0.04 (x surprised x lit), expected 0.16 plain. The flag is never set,
    // so no class 1 ship ever fires two torpedo salvos.
    let mut plain = 0;
    for run in runs() {
        let per_ship = by_ship(
            torpedo_salvos(run)
                .into_iter()
                .filter(|s| force_of_ship(s.firer) == 2)
                .collect(),
        );
        for (ship, salvos) in &per_ship {
            assert_eq!(salvos.len(), 1, "seed {} ship {ship}", run.seed);
            let salvo = &salvos[0];
            assert_eq!(salvo.factor("tubes"), Fixed::from_int(8));
            check_torpedo_salvo(salvo, run.seed);
            let (surprised, lit) = (salvo.factor("surprised_factor"), salvo.factor("lit_factor"));
            if salvo.night()
                && salvo.factor("torpedo_mount_factor") == Fixed::ONE
                && surprised == Fixed::ONE
                && lit == Fixed::ONE
            {
                assert_eq!(salvo.shots, Fixed::from_int(4), "seed {}", run.seed);
                assert_eq!(salvo.expected, fx("0.16"), "seed {}", run.seed);
                plain += 1;
            }
        }
    }
    assert!(plain > 100, "{plain}");
}

/// Case C: force B's cruisers are class 8 too, so force B (not ready, share
/// 0.5) fires wing tubes.
fn text_c() -> String {
    let text = text_of(&scenario_path());
    let old = "class = 1\nside = \"allied\"";
    assert_eq!(text.matches(old).count(), 3);
    text.replace(old, "class = 8\nside = \"allied\"")
}

#[test]
fn case_c_a_force_not_ready_fires_2_and_then_2_and_together_4() {
    let base = from_text(&text_c());
    let (mut first, mut second) = (0, 0);
    for seed in 1..=VARIANT_SEEDS {
        let run = run_of(&base, seed);
        let per_ship = by_ship(
            torpedo_salvos(&run)
                .into_iter()
                .filter(|s| force_of_ship(s.firer) == 2 && s.night())
                .collect(),
        );
        for (ship, salvos) in &per_ship {
            let mut total = Fixed::ZERO;
            for (index, salvo) in salvos.iter().enumerate() {
                assert!(!salvo.ready, "seed {seed}");
                assert_eq!(salvo.factor("tubes"), Fixed::from_int(4));
                check_torpedo_salvo(salvo, seed);
                total += salvo.shots;
                if salvo.factor("torpedo_mount_factor") != Fixed::ONE {
                    continue;
                }
                assert_eq!(salvo.shots, Fixed::from_int(2), "seed {seed} ship {ship}");
                let (surprised, lit) =
                    (salvo.factor("surprised_factor"), salvo.factor("lit_factor"));
                if surprised == Fixed::ONE && lit == Fixed::ONE {
                    assert_eq!(salvo.chance, fx("0.04"));
                    assert_eq!(salvo.expected, fx("0.08"), "seed {seed}");
                    if index == 0 {
                        first += 1;
                    } else {
                        second += 1;
                    }
                }
            }
            // Together 4, today's 8 x 0.5, at most.
            assert!(total <= Fixed::from_int(4), "seed {seed} ship {ship}");
        }
    }
    eprintln!("(C) plain salvos: first side {first}, second side {second}");
    assert!(first > 100 && second > 50, "{first} {second}");
}

/// Case D: force A's mounts are degraded before round 1.
fn text_d() -> String {
    let text = text_of(&scenario_path());
    let old = "class = 8\nside = \"japanese\"";
    assert_eq!(text.matches(old).count(), 3);
    text.replace(
        old,
        "class = 8\nside = \"japanese\"\ntorpedo_mounts = \"degraded\"",
    )
}

#[test]
fn case_d_degraded_mounts_fire_2_and_then_2() {
    let base = from_text(&text_d());
    let (mut first, mut second) = (0, 0);
    for seed in 1..=VARIANT_SEEDS {
        let run = run_of(&base, seed);
        let per_ship = by_ship(
            torpedo_salvos(&run)
                .into_iter()
                .filter(|s| force_of_ship(s.firer) == 1 && s.night())
                .collect(),
        );
        for (ship, salvos) in &per_ship {
            for (index, salvo) in salvos.iter().enumerate() {
                assert_eq!(salvo.factor("tubes"), Fixed::from_int(4));
                check_torpedo_salvo(salvo, seed);
                // Round 1 is before any hit: the mounts are as set. A later
                // hit may knock them out (no salvo) but never mend them.
                assert_eq!(
                    salvo.factor("torpedo_mount_factor"),
                    DEGRADED,
                    "seed {seed} ship {ship}"
                );
                assert_eq!(salvo.shots, Fixed::from_int(2), "seed {seed}");
                let (surprised, lit) =
                    (salvo.factor("surprised_factor"), salvo.factor("lit_factor"));
                if surprised == Fixed::ONE && lit == Fixed::ONE {
                    assert_eq!(salvo.expected, fx("0.08"), "seed {seed}");
                    if index == 0 {
                        first += 1;
                    } else {
                        second += 1;
                    }
                }
            }
        }
    }
    eprintln!("(D) plain salvos: first side {first}, second side {second}");
    assert!(first > 100 && second > 50, "{first} {second}");
}

// ---------------------------------------------------------------------
// Part 2: properties over many seeds.
// ---------------------------------------------------------------------

#[test]
fn every_class_8_ship_that_fires_twice_fires_b_then_t_minus_b_in_different_rounds() {
    let (mut twice, mut once) = (0, 0);
    for run in runs() {
        for (ship, salvos) in by_ship(torpedo_salvos(run)) {
            let class_8 = force_of_ship(ship) == 1;
            if !class_8 {
                // No class 1 ship ever fires two torpedo salvos.
                assert_eq!(salvos.len(), 1, "seed {} ship {ship}", run.seed);
                assert_eq!(salvos[0].factor("tubes"), Fixed::from_int(8));
                continue;
            }
            assert!(salvos.len() <= 2, "seed {} ship {ship}", run.seed);
            // The tubes of a class 8 ship's salvos: 4, then 4.
            for salvo in &salvos {
                assert_eq!(salvo.factor("tubes"), Fixed::from_int(4));
                check_torpedo_salvo(salvo, run.seed);
            }
            if let [first, second] = &salvos[..] {
                twice += 1;
                // Different rounds, or a later fight (a later pulse).
                assert!(
                    (second.pulse, second.round) != (first.pulse, first.round),
                    "seed {} ship {ship}",
                    run.seed
                );
                // Both sides' shots are whole, 4 x mount x share, never more
                // than 8 in all (tubes x mount x share, with a share of 1 for
                // a ready force).
                assert!(first.shots + second.shots <= Fixed::from_int(8));
            } else {
                once += 1;
            }
        }
    }
    eprintln!("class 8 ships firing twice: {twice}, once: {once}");
    assert!(twice > 1500, "the rule was checked: {twice}");
}

#[test]
fn the_sum_of_a_class_8_ships_torpedo_shots_never_exceeds_8_times_the_share() {
    // Force B in case C is not ready at night (share 0.5): over a night fight
    // a class 8 ship's torpedo shots sum to at most 8 x 0.5 = 4. Every
    // salvo's tubes x mount x share is at most 4 x 0.5 = 2 there.
    let base = from_text(&text_c());
    let mut ships = 0;
    for seed in 1..=VARIANT_SEEDS {
        let run = run_of(&base, seed);
        for (ship, salvos) in by_ship(
            torpedo_salvos(&run)
                .into_iter()
                .filter(|s| force_of_ship(s.firer) == 2 && !s.ready)
                .collect(),
        ) {
            let total = salvos.iter().fold(Fixed::ZERO, |sum, s| sum + s.shots);
            assert!(
                total <= Fixed::from_int(8) * SHARE,
                "seed {seed} ship {ship}"
            );
            ships += 1;
        }
    }
    assert!(ships > 200, "{ships}");
    // And force A, ready (share 1), in the scenario itself: at most 8.
    for run in runs() {
        for (ship, salvos) in by_ship(
            torpedo_salvos(run)
                .into_iter()
                .filter(|s| force_of_ship(s.firer) == 1)
                .collect(),
        ) {
            let total = salvos.iter().fold(Fixed::ZERO, |sum, s| sum + s.shots);
            assert!(total <= Fixed::from_int(8), "seed {} ship {ship}", run.seed);
        }
    }
}

#[test]
fn the_torpedo_hits_stay_within_four_sigma_of_what_the_chances_expect() {
    // Over every night torpedo salvo of force A: (H - E)^2 <= 16 x the sum of
    // np(1 - p), in exact arithmetic (n and p are the event's). The shots are
    // whole here (4), so n is exact.
    let (mut hits, mut expected6, mut variance12) = (0i128, 0i128, 0i128);
    for run in runs() {
        for salvo in torpedo_salvos(run)
            .into_iter()
            .filter(|s| force_of_ship(s.firer) == 1)
        {
            let n = i128::from(salvo.shots.trunc());
            assert_eq!(salvo.shots, Fixed::from_int(salvo.shots.trunc()), "whole");
            let p = i128::from(salvo.chance.raw());
            hits += i128::from(salvo.hits);
            expected6 += n * p;
            variance12 += n * p * (1_000_000 - p);
        }
    }
    let off6 = hits * 1_000_000 - expected6;
    assert!(
        off6 * off6 <= 16 * variance12,
        "hits {hits} expected (millionths) {expected6}"
    );
    eprintln!(
        "force A: torpedo hits {hits}, expected {}",
        Fixed::from_raw(i64::try_from(expected6).expect("small"))
    );
}

#[test]
fn a_second_salvo_is_aimed_at_force_b_and_never_at_a_ship_of_force_a() {
    // The target is chosen by decision 0078 at the start of its own round:
    // always an enemy ship.
    for run in runs().iter().take(200) {
        for salvo in torpedo_salvos(run) {
            assert_ne!(
                force_of_ship(salvo.target),
                force_of_ship(salvo.firer),
                "seed {}",
                run.seed
            );
        }
    }
}

// ---------------------------------------------------------------------
// Part 3: a pinned outcome on the live figures.
// ---------------------------------------------------------------------

/// PINNED, not calculated. On the live figures, seed 1942, the scenario's
/// own. No hand calculation can say which draw succeeds; this catches a
/// stream that moved, which the properties above would miss. It follows the
/// live figures: when a figure in `data/reference/surface_combat.toml` is
/// tuned, or a rule is changed on purpose, it is remade with the change, and
/// never to make a test pass.
#[test]
fn pinned_outcome_of_seed_1942_on_the_live_figures() {
    let base = common::load_live(&scenario_path()).expect("scenario 18 loads on live figures");
    let run = run_of(&base, 1942);
    let salvos = salvos_of(&run.events, &[1]);
    let torpedoes: Vec<Salvo> = salvos
        .iter()
        .filter(|s| s.weapon == Weapon::Torpedo)
        .cloned()
        .collect();
    let summary = Pinned {
        salvos: salvos.len(),
        torpedo_salvos_a: torpedoes
            .iter()
            .filter(|s| force_of_ship(s.firer) == 1)
            .count(),
        torpedo_shots_a: torpedoes
            .iter()
            .filter(|s| force_of_ship(s.firer) == 1)
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
    torpedo_salvos_a: usize,
    torpedo_shots_a: i64,
    torpedo_hits_a: u32,
    torpedo_hits_b: u32,
    hits_a: u32,
    hits_b: u32,
    events: usize,
}

// Pinned on Nov 13, 2026, when the scenario was built (decision 0083).
const PINNED: Pinned = Pinned {
    salvos: 18,
    torpedo_salvos_a: 6,
    torpedo_shots_a: 24,
    torpedo_hits_a: 0,
    torpedo_hits_b: 0,
    hits_a: 40,
    hits_b: 4,
    events: 73,
};
