//! Scenario 6c2: a force two days at battle readiness is met at night by
//! one that came to it that evening.
//!
//! Stage 6c2 of scenario 6 (decision 0037). The data is
//! `data/scenarios/06c2_tired_lookouts.toml`, run with seeds 1 to 1000.
//! Everything in it is a placeholder.
//!
//! # Hand calculation
//!
//! ## The setting
//!
//! The ships, the map and the combat figures are those of scenario 4a
//! (`tests/scenario_04a_night_action.rs`): three placeholder cruisers a
//! side, no radar, and at night a force sights with chance 0.5, times 0.5
//! if it is not alert.
//!
//! - Force B, task force 2, Allied, lies stopped at (5,0) at battle
//!   readiness from the first day.
//! - Force A, task force 1, Japanese, lies at (0,0) at normal readiness. Its
//!   side holds no contact, so it is not alert. On day 3 it is ordered to
//!   battle readiness and to steam east to (10,0) at 16 kn.
//! - Both are ordered to engage and to break off at 0.5.
//!
//! The scenario runs three days. Each has a night pulse, a morning pulse
//! and an afternoon pulse.
//!
//! ## Fatigue
//!
//! At the start of each pulse a force that is alert gains 5 and one that is
//! not loses 3, not below 0 and not above 100.
//!
//! | At the start of | Force A | Force B |
//! | --------------- | ------- | ------- |
//! | Day 1, night | not alert: 0 | alert: 5 |
//! | Day 1, morning | 0 | 10 |
//! | Day 1, afternoon | 0 | 15 |
//! | Day 2, night | 0 | 20 |
//! | Day 2, morning | 0 | 25 |
//! | Day 2, afternoon | 0 | 30 |
//! | Day 3, night | alert by order: 5 | 35 |
//! | Day 3, morning | 10 | 40 |
//! | Day 3, afternoon | 15 | 45 |
//!
//! Force A at 0 has nothing to rest, and no event is recorded for it on the
//! first two days. The last two rows hold for a force with a ship afloat.
//!
//! ## The meeting
//!
//! Force A steams 180 nm at 16 kn and enters (5,0) 11.25 hours into the
//! night pulse of day 3, as in scenario 4a. The two forces fight there, in
//! the dark, with the fatigue of the start of that pulse: 5 and 35.
//!
//! Both are at battle readiness, so both are alert and neither is surprised
//! by its readiness.
//!
//! | Force | Sighting chance in each round of the night fight |
//! | ----- | ------------------------------------------------ |
//! | A | 0.5 x (100 - 5) / 100 = 0.475 |
//! | B | 0.5 x (100 - 35) / 100 = 0.325 |
//!
//! Everything else in the fight is scenario 4a's rule and is not worked
//! again here: who fires, the hits, the damage, breaking off.
//!
//! Over seeds 1 to 1000 the fight always happens, and in its first round
//! each force rolls once to sight the other:
//!
//! | Count of runs | p | Mean | Std dev | Allowed |
//! | ------------- | - | ---- | ------- | ------- |
//! | Force A sights in round 1 | 0.475 | 475 | 15.79 | 412 to 538 |
//! | Force B sights in round 1 | 0.325 | 325 | 14.81 | 266 to 384 |
//!
//! Every sighting roll of every run, in any round and any pulse, has the
//! chance the figures give for the light, times 0.5 if the force is not
//! alert at that moment, times (100 less the force's fatigue at the start
//! of that pulse) over 100. In a fight by day the base chance is 1, so on
//! the morning of day 3 force A would sight with 0.9 and force B with 0.6.
//!
//! ## Variants
//!
//! | Variant | Force A in the night fight | Force B in the night fight |
//! | ------- | -------------------------- | -------------------------- |
//! | The scenario names no fatigue file | 0.5 | 0.5. No fatigue event is recorded |
//! | Force B is at normal readiness throughout | 0.475 | Fatigue 0 and not alert: 0.5 x 0.5 = 0.25 in every roll of the night fight |
//! | Force B is ordered to battle readiness on day 3 only | 0.475 | Fatigue 5: 0.475 |
//! | Force A is at battle readiness from day 1 | Fatigue 35: 0.325 | 0.325 |
//! | Force B is at battle readiness on days 1 and 2 and ordered to normal on day 3 | 0.475 | Fatigue 30 less 3 = 27, and not alert: 0.5 x 0.5 x 0.73 = 0.1825 in every roll of the night fight |
//! | Force B starts with a fatigue of 100 | 0.475 | It stays at 100: chance 0. Force B never sights force A |
//!
//! (The two rows for a force B that is not alert first said its chance
//! doubles once it is alerted in the fight. It never rolls again after
//! that: a force that has been fired on has found its enemy, decision 0014.
//! No figure changed.)
//!
//! So at these figures two days at battle readiness still sight better
//! (0.325) than a rested force caught at normal readiness (0.25), and worse
//! than a force that came to battle readiness that evening (0.475). Standing
//! down on the last day is the worst of all (0.1825).
//!
//! ## What cannot happen
//!
//! - No fatigue is below 0 or above 100.
//! - No fatigue changes other than at the start of a pulse.
//! - No random number is drawn for fatigue. Without the fatigue file the
//!   run is, event for event, the same scenario with every sighting chance
//!   at its 4a value.
//!
//! # Properties
//!
//! - A higher fatigue never gives a higher chance to sight.
//! - More pulses at battle readiness never give a lower fatigue: force B's
//!   fatigue at the night fight is 5 when ordered on day 3, 35 from day 1.
//! - Fatigue changes no gun or torpedo chance: every salvo's chance is the
//!   one scenario 4a's rule gives for its light, surprise and fire control.
//! - Recording breakdowns changes neither the events nor the hashes.
//! - The same seed gives the same hashes twice.
//! - A scenario without the fatigue file has the hashes it had before this
//!   stage: the golden files of scenarios 1 to 6c do not change.

// The test code is written by a developer from the hand calculation above.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{Cause, Event, EventKind, ForceFatigueChange};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_core::world::SimConfig;
use sim_data::Scenario;

const SEEDS: u64 = 1000;
const MEETING: Hex = Hex::new(5, 0);
/// Written out from the hand calculation, not read from the data files.
const GUN_BASE_NIGHT: Fixed = Fixed::from_raw(30_000);
const TORPEDO_BASE_NIGHT: Fixed = Fixed::from_raw(40_000);
const GUN_BASE_DAY: Fixed = Fixed::from_raw(50_000);
const TORPEDO_BASE_DAY: Fixed = Fixed::from_raw(20_000);
const SURPRISED: Fixed = Fixed::from_raw(1_500_000);
const LIT: Fixed = Fixed::from_raw(2_000_000);
/// Fatigue gained in a pulse by a force that is alert, and lost by one that
/// is not.
const GAIN: i64 = 5;
const REST: i64 = 3;

fn fx(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn millionths(value: i64) -> Fixed {
    Fixed::from_raw(value)
}

/// The chance written as the hand calculation writes it: a decimal.
fn dec(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

/// (100 less fatigue) over 100, for a whole-number fatigue, in millionths.
fn fatigue_factor(fatigue: Fixed) -> Fixed {
    assert_eq!(
        fatigue.raw() % 1_000_000,
        0,
        "the fatigues here are whole numbers"
    );
    Fixed::from_raw((100 - fatigue.raw() / 1_000_000) * 10_000)
}

fn pulse_number(pulse: Pulse) -> usize {
    match pulse {
        Pulse::Night => 0,
        Pulse::Morning => 1,
        Pulse::Afternoon => 2,
    }
}

// ---------------------------------------------------------------------
// The scenario and its variants.
// ---------------------------------------------------------------------

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data")
}

fn scenario_path() -> PathBuf {
    data_dir().join("scenarios/06c2_tired_lookouts.toml")
}

fn load() -> Scenario {
    common::load(&scenario_path()).expect("scenario 6c2 loads")
}

/// The fatigue key of the scenario file, and the readiness line that puts
/// force B at battle readiness from the start.
const FATIGUE_KEY: &str = "fatigue = \"../reference/fatigue.toml\"\n";
const B_BATTLE: &str = "mission = \"surface_combat\"\nreadiness = \"battle\"\n";
const B_NORMAL: &str = "mission = \"surface_combat\"\n";
const A_START: &str = "q = 0\nr = 0\nmission = \"surface_combat\"\n";

/// A readiness order for one force on one day.
fn readiness_order(day: u32, side: &str, force: u32, readiness: &str) -> String {
    format!(
        "\n[[order]]\nday = {day}\nside = \"{side}\"\ntask_force = {force}\nkind = \"set_readiness\"\nreadiness = \"{readiness}\"\n"
    )
}

/// Loads the scenario on the frozen figures with each exact, asserted-unique
/// text replacement made in its text and in a copy of the fatigue figures,
/// and `append` added to the scenario's text. Nothing in the repository's
/// data folder is touched.
fn variant(
    tag: &str,
    scenario_edits: &[(&str, &str)],
    append: &str,
    fatigue_edits: &[(&str, &str)],
) -> Scenario {
    let path = scenario_path();
    let mut text = fs::read_to_string(&path).expect("read the scenario");
    for (original, changed) in scenario_edits {
        text = common::replace_once(&text, original, changed);
    }
    text.push_str(append);
    let options = common::altered_reference(tag, |dir| {
        common::edit_file_by(dir, "fatigue.toml", fatigue_edits);
    });
    common::from_text_with(&text, &path, &options).expect("the variant loads")
}

/// What a variant does, as the hand calculation's table gives it.
#[derive(Clone, Debug)]
struct Plan {
    /// Whether the force is at battle readiness on days 1, 2 and 3 (by its
    /// start or by an order given that day), for force A and force B.
    battle: [[bool; 3]; 2],
    /// Their fatigue at the start.
    start: [i64; 2],
    /// The figures of the fatigue file the scenario names: fatigue gained
    /// in a pulse by an alert force and lost by one that is not. `None`
    /// where it names no file.
    figures: Option<(i64, i64)>,
}

impl Plan {
    /// The scenario as it stands.
    fn base() -> Plan {
        Plan {
            battle: [[false, false, true], [true, true, true]],
            start: [0, 0],
            figures: Some((GAIN, REST)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Base,
    NoFile,
    BNormal,
    BDayThree,
    ABattleFromDayOne,
    BStandsDown,
    BFull,
    BDayTwo,
    /// A fatigue file whose two force figures are 0.
    ZeroFigures,
}

fn build(kind: Kind) -> (Scenario, Plan) {
    let mut plan = Plan::base();
    let tag = format!("{kind:?}");
    let scenario = match kind {
        Kind::Base => load(),
        Kind::NoFile => {
            plan.figures = None;
            variant(&tag, &[(FATIGUE_KEY, "")], "", &[])
        }
        Kind::BNormal => {
            plan.battle[1] = [false; 3];
            variant(&tag, &[(B_BATTLE, B_NORMAL)], "", &[])
        }
        Kind::BDayThree => {
            plan.battle[1] = [false, false, true];
            variant(
                &tag,
                &[(B_BATTLE, B_NORMAL)],
                &readiness_order(3, "allied", 2, "battle"),
                &[],
            )
        }
        Kind::ABattleFromDayOne => {
            plan.battle[0] = [true; 3];
            variant(
                &tag,
                &[(A_START, &format!("{A_START}readiness = \"battle\"\n"))],
                "",
                &[],
            )
        }
        Kind::BStandsDown => {
            plan.battle[1] = [true, true, false];
            variant(&tag, &[], &readiness_order(3, "allied", 2, "normal"), &[])
        }
        Kind::BFull => {
            plan.start[1] = 100;
            variant(
                &tag,
                &[(B_BATTLE, &format!("{B_BATTLE}fatigue = 100\n"))],
                "",
                &[],
            )
        }
        Kind::BDayTwo => {
            plan.battle[1] = [false, true, true];
            variant(
                &tag,
                &[(B_BATTLE, B_NORMAL)],
                &readiness_order(2, "allied", 2, "battle"),
                &[],
            )
        }
        Kind::ZeroFigures => {
            plan.figures = Some((0, 0));
            variant(
                &tag,
                &[],
                "",
                &[
                    (
                        "force_fatigue_per_pulse_alert = 5",
                        "force_fatigue_per_pulse_alert = 0",
                    ),
                    ("force_rest_per_pulse = 3", "force_rest_per_pulse = 0"),
                ],
            )
        }
    };
    (scenario, plan)
}

/// Force B at battle readiness from the start with a starting fatigue.
fn b_starting_at(fatigue: i64) -> Scenario {
    variant(
        &format!("start_{fatigue}"),
        &[(B_BATTLE, &format!("{B_BATTLE}fatigue = {fatigue}\n"))],
        "",
        &[],
    )
}

// ---------------------------------------------------------------------
// A run, read back through the hand calculation.
// ---------------------------------------------------------------------

#[derive(Clone, Debug)]
struct RollRec {
    day: u32,
    pulse: Pulse,
    round: u32,
    force: usize,
    chance: Fixed,
    factor: Fixed,
    sighted: bool,
    /// The fight the roll belongs to, counted from 0 in the run.
    fight: usize,
}

#[derive(Clone, Debug)]
struct FightRec {
    day: u32,
    pulse: Pulse,
    hex: Hex,
    elapsed_hours: Fixed,
}

/// One run, with what the model of the rules says about it.
#[derive(Clone, Debug)]
struct Run {
    seed: u64,
    events: Vec<Event>,
    day_hashes: Vec<u64>,
    /// The fatigue of each force at the start of each of the nine pulses,
    /// after its change: `[force][pulse index]`. `None` where the force has
    /// no ship afloat.
    fatigue: [[Option<Fixed>; 9]; 2],
    rolls: Vec<RollRec>,
    fights: Vec<FightRec>,
    /// The number of events about fatigue.
    fatigue_events: usize,
}

fn force_index(force: sim_core::ids::TaskForceId) -> usize {
    assert!((1..=2).contains(&force.raw()), "only forces 1 and 2 exist");
    usize::try_from(force.raw() - 1).expect("small")
}

/// Runs one seed and reads the events back: every fatigue event is checked
/// against the rule, and so is every sighting roll's chance.
fn run_seed(base: &Scenario, plan: &Plan, seed: u64, breakdowns: bool) -> Run {
    let mut scenario = base.with_seed(seed).expect("a seed can be set");
    scenario.world.set_config(SimConfig {
        record_breakdowns: breakdowns,
    });
    scenario.run_to_end().expect("the run resolves");
    let events: Vec<Event> = scenario.world.events().iter().cloned().collect();
    let day_hashes = scenario.world.day_hashes().to_vec();
    analyse(seed, plan, events, day_hashes)
}

fn analyse(seed: u64, plan: &Plan, events: Vec<Event>, day_hashes: Vec<u64>) -> Run {
    let mut alerted = [false; 2];
    let mut fatigue = [fx(plan.start[0]), fx(plan.start[1])];
    let mut afloat = [3u32; 2];
    let mut table = [[None; 9]; 2];
    let mut rolls = Vec::new();
    let mut fights: Vec<FightRec> = Vec::new();
    let mut fatigue_events = 0;
    let full = fx(100);

    for day in 1..=3u32 {
        let day_index = usize::try_from(day - 1).expect("small");
        for pulse in Pulse::ALL {
            let here: Vec<&Event> = events
                .iter()
                .filter(|e| e.day == day && e.pulse == Some(pulse))
                .collect();
            let pulse_index = day_index * 3 + pulse_number(pulse);

            // The start of the pulse: alert from belief, then fatigue.
            let lead = here
                .iter()
                .take_while(|e| {
                    matches!(
                        e.kind,
                        EventKind::ForceAlertedByContact { .. }
                            | EventKind::ForceStoodDown { .. }
                            | EventKind::TaskForceFatigue { .. }
                    )
                })
                .count();
            let mut seen_fatigue = false;
            for event in &here[..lead] {
                match event.kind {
                    EventKind::ForceAlertedByContact { task_force, .. } => {
                        assert!(!seen_fatigue, "seed {seed}: alert comes before fatigue");
                        alerted[force_index(task_force)] = true;
                    }
                    EventKind::ForceStoodDown { task_force } => {
                        assert!(!seen_fatigue, "seed {seed}: alert comes before fatigue");
                        alerted[force_index(task_force)] = false;
                    }
                    EventKind::TaskForceFatigue { .. } => seen_fatigue = true,
                    _ => unreachable!("the lead is made of these three"),
                }
            }
            let observed: Vec<(usize, ForceFatigueChange, Fixed, Fixed)> = here[..lead]
                .iter()
                .filter_map(|e| match e.kind {
                    EventKind::TaskForceFatigue {
                        task_force,
                        change,
                        amount,
                        fatigue,
                    } => Some((force_index(task_force), change, amount, fatigue)),
                    _ => None,
                })
                .collect();
            let mut expected = Vec::new();
            if let Some((gain, rest)) = plan.figures {
                for force in 0..2 {
                    if afloat[force] == 0 {
                        continue;
                    }
                    let alert = plan.battle[force][day_index] || alerted[force];
                    let (change, after) = if alert {
                        (
                            ForceFatigueChange::Gained,
                            (fatigue[force] + fx(gain)).min(full),
                        )
                    } else {
                        (
                            ForceFatigueChange::Rested,
                            (fatigue[force] - fx(rest)).max(Fixed::ZERO),
                        )
                    };
                    if after != fatigue[force] {
                        let amount = match change {
                            ForceFatigueChange::Gained => after - fatigue[force],
                            ForceFatigueChange::Rested => fatigue[force] - after,
                        };
                        expected.push((force, change, amount, after));
                    }
                    fatigue[force] = after;
                }
            }
            assert_eq!(
                observed, expected,
                "seed {seed} day {day} {pulse:?}: the fatigue events"
            );
            // A start-of-pulse fatigue event is at the start of the pulse
            // and nowhere else: none follows the lead (below).
            for force in 0..2 {
                assert!(
                    fatigue[force] >= Fixed::ZERO && fatigue[force] <= full,
                    "seed {seed}: fatigue within 0 to 100"
                );
                if afloat[force] > 0 {
                    table[force][pulse_index] = Some(fatigue[force]);
                }
            }
            fatigue_events += observed.len();

            // The causes of a fatigue event: the readiness order when the
            // force is at battle readiness by order, otherwise the event
            // that alerted it.
            for event in &here[..lead] {
                if let EventKind::TaskForceFatigue {
                    task_force, change, ..
                } = event.kind
                {
                    let force = force_index(task_force);
                    let orders = event
                        .causes
                        .iter()
                        .filter(|c| matches!(c, Cause::Order(_)))
                        .count();
                    let by_events = event.causes.len() - orders;
                    assert!(event.causes.len() <= 1, "seed {seed}: one cause at most");
                    if change == ForceFatigueChange::Gained
                        && plan.battle[force][day_index]
                        && force == 0
                        && day == 3
                        && plan.start == [0, 0]
                    {
                        // Force A is at battle readiness by an order on day 3.
                        assert!(orders <= 1 && by_events == 0, "seed {seed}");
                    }
                    if plan.battle[force][day_index] {
                        assert_eq!(by_events, 0, "seed {seed}: cites orders, not events");
                    }
                }
            }

            // The rest of the pulse, in order.
            let mut fight_number = fights.len();
            for event in &here[lead..] {
                match &event.kind {
                    EventKind::TaskForceFatigue { .. } => {
                        panic!("seed {seed}: fatigue changed after the start of a pulse")
                    }
                    EventKind::EngagementBegun {
                        a,
                        b,
                        hex,
                        elapsed_hours,
                    } => {
                        assert_eq!((a.raw(), b.raw()), (1, 2));
                        fights.push(FightRec {
                            day,
                            pulse,
                            hex: *hex,
                            elapsed_hours: *elapsed_hours,
                        });
                        fight_number = fights.len() - 1;
                    }
                    EventKind::ForceBecameAlert { task_force, .. } => {
                        alerted[force_index(*task_force)] = true;
                    }
                    EventKind::AlarmRoll {
                        task_force,
                        alarmed: true,
                        ..
                    } => alerted[force_index(*task_force)] = true,
                    EventKind::ShipSunk { task_force, .. } => {
                        let force = force_index(*task_force);
                        afloat[force] -= 1;
                    }
                    EventKind::ShipsDetached { .. } => {
                        panic!("seed {seed}: no force detaches stragglers here")
                    }
                    EventKind::SightingRoll {
                        round,
                        task_force,
                        chance,
                        fatigue_factor: factor,
                        sighted,
                        ..
                    } => {
                        let force = force_index(*task_force);
                        // base x not-alert x fatigue.
                        let base = if pulse == Pulse::Night {
                            dec("0.5")
                        } else {
                            Fixed::ONE
                        };
                        let alert = plan.battle[force][day_index] || alerted[force];
                        let alert_factor = if alert { Fixed::ONE } else { dec("0.5") };
                        let expected_factor = if plan.figures.is_some() {
                            fatigue_factor(fatigue[force])
                        } else {
                            Fixed::ONE
                        };
                        let expected_chance =
                            (base * alert_factor).min(Fixed::ONE) * expected_factor;
                        assert_eq!(
                            *chance, expected_chance,
                            "seed {seed} day {day} {pulse:?} round {round} force {}: chance (alert {alert}, fatigue {})",
                            force + 1,
                            fatigue[force]
                        );
                        assert_eq!(*factor, expected_factor, "seed {seed}");
                        // The factor is in the breakdown when fatigue is in
                        // play, and the breakdown's result is the chance.
                        if let Some(breakdown) = &event.breakdown {
                            assert_eq!(breakdown.result, *chance);
                            let last = breakdown.factors.last().expect("factors");
                            if plan.figures.is_some() {
                                assert_eq!(
                                    (last.name, last.value),
                                    ("fatigue_factor", expected_factor)
                                );
                                assert_eq!(breakdown.factors.len(), 4);
                            } else {
                                assert_eq!(last.name, "alert_factor");
                                assert_eq!(breakdown.factors.len(), 3);
                            }
                        }
                        // The text shows the factor only when it is not one.
                        let text = event.kind.to_string();
                        assert_eq!(
                            text.contains("fatigue factor"),
                            expected_factor != Fixed::ONE,
                            "seed {seed}: {text}"
                        );
                        rolls.push(RollRec {
                            day,
                            pulse,
                            round: *round,
                            force,
                            chance: *chance,
                            factor: *factor,
                            sighted: *sighted,
                            fight: fight_number,
                        });
                    }
                    _ => {}
                }
            }
        }
    }
    assert!(
        events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::TaskForceFatigue { .. }))
            .count()
            == fatigue_events,
        "seed {seed}: every fatigue event was at the start of a pulse"
    );
    Run {
        seed,
        events,
        day_hashes,
        fatigue: table,
        rolls,
        fights,
        fatigue_events,
    }
}

fn batch_of(kind: Kind, seeds: u64) -> Vec<Run> {
    let (scenario, plan) = build(kind);
    (1..=seeds)
        .map(|seed| run_seed(&scenario, &plan, seed, true))
        .collect()
}

/// Seeds 1 to 1000 of the scenario as it stands, run once and shared.
fn batch() -> &'static [Run] {
    static BATCH: OnceLock<Vec<Run>> = OnceLock::new();
    BATCH.get_or_init(|| batch_of(Kind::Base, SEEDS))
}

/// Seeds 1 to 1000 of a variant, run once and shared.
fn variant_batch(kind: Kind) -> &'static [Run] {
    static BATCHES: OnceLock<BTreeMap<String, Vec<Run>>> = OnceLock::new();
    let all = BATCHES.get_or_init(|| {
        [
            Kind::NoFile,
            Kind::BNormal,
            Kind::BDayThree,
            Kind::ABattleFromDayOne,
            Kind::BStandsDown,
            Kind::BFull,
            Kind::BDayTwo,
            Kind::ZeroFigures,
        ]
        .into_iter()
        .map(|kind| (format!("{kind:?}"), batch_of(kind, SEEDS)))
        .collect()
    });
    &all[&format!("{kind:?}")]
}

/// The fight in the night pulse of day 3.
fn night_fight(run: &Run) -> usize {
    let found: Vec<usize> = run
        .fights
        .iter()
        .enumerate()
        .filter(|(_, f)| f.day == 3 && f.pulse == Pulse::Night)
        .map(|(index, _)| index)
        .collect();
    assert_eq!(
        found.len(),
        1,
        "seed {}: one fight on the night of day 3",
        run.seed
    );
    found[0]
}

/// The chances of one force's rolls in the night fight, in round order.
fn night_chances(run: &Run, force: usize) -> Vec<Fixed> {
    let fight = night_fight(run);
    run.rolls
        .iter()
        .filter(|r| r.fight == fight && r.force == force)
        .map(|r| r.chance)
        .collect()
}

/// Whether a force sighted by roll in round 1 of the night fight.
fn sights_in_round_1(run: &Run, force: usize) -> bool {
    let fight = night_fight(run);
    let first: Vec<&RollRec> = run
        .rolls
        .iter()
        .filter(|r| r.fight == fight && r.force == force && r.round == 1)
        .collect();
    assert_eq!(first.len(), 1, "seed {}: one roll in round 1", run.seed);
    first[0].sighted
}

// ---------------------------------------------------------------------
// Fatigue.
// ---------------------------------------------------------------------

#[test]
fn fatigue_follows_the_table_pulse_by_pulse_in_every_run() {
    // Force A, then force B, at the start of each pulse: days 1, 2 and 3.
    let a = [0, 0, 0, 0, 0, 0, 5, 10, 15];
    let b = [5, 10, 15, 20, 25, 30, 35, 40, 45];
    let (mut a_afloat, mut b_afloat) = (0, 0);
    for run in batch() {
        for pulse in 0..9 {
            // Rows up to the night fight hold for every run; the last two
            // hold for a force with a ship afloat (the table records None
            // where there is none).
            for (force, table) in [(0, a), (1, b)] {
                match run.fatigue[force][pulse] {
                    Some(value) => assert_eq!(
                        value,
                        fx(table[pulse]),
                        "seed {} force {} pulse {pulse}",
                        run.seed,
                        force + 1
                    ),
                    None => assert!(pulse >= 7, "seed {}: a force with no ship", run.seed),
                }
            }
        }
        a_afloat += usize::from(run.fatigue[0][8].is_some());
        b_afloat += usize::from(run.fatigue[1][8].is_some());
        // The fatigue events are exactly the changes: force B's nine, and
        // force A's three on day 3, for a force with a ship afloat.
        let b_events = run
            .events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::TaskForceFatigue { task_force, .. } if task_force.raw() == 2))
            .count();
        let a_events = run
            .events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::TaskForceFatigue { task_force, .. } if task_force.raw() == 1))
            .count();
        assert_eq!(run.fatigue_events, a_events + b_events, "seed {}", run.seed);
        assert!(a_events <= 3 && b_events <= 9);
        assert_eq!(
            a_events,
            run.fatigue[0][6..].iter().filter(|f| f.is_some()).count()
        );
        // Force A has no fatigue event on days 1 and 2.
        assert!(!run
            .events
            .iter()
            .any(|e| e.day < 3
                && matches!(e.kind, EventKind::TaskForceFatigue { task_force, .. } if task_force.raw() == 1)));
        // Every event of days 1 and 2 is an order or force B's fatigue.
        for event in run.events.iter().filter(|e| e.day < 3) {
            assert!(
                matches!(
                    event.kind,
                    EventKind::OrderAccepted { .. } | EventKind::TaskForceFatigue { .. }
                ),
                "seed {}: {}",
                run.seed,
                event.kind
            );
        }
    }
    eprintln!("runs with force A afloat in the afternoon of day 3: {a_afloat}");
    eprintln!("runs with force B afloat in the afternoon of day 3: {b_afloat}");
}

#[test]
fn fatigue_events_cite_the_readiness_order_for_a_force_ordered_to_battle() {
    let run = &batch()[0];
    let a: Vec<&Event> = run
        .events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::TaskForceFatigue { task_force, .. } if task_force.raw() == 1))
        .collect();
    assert!(!a.is_empty());
    for event in &a {
        let [Cause::Order(order)] = event.causes[..] else {
            panic!(
                "force A's fatigue cites its readiness order: {:?}",
                event.causes
            );
        };
        // The third order of the scenario is the readiness order.
        assert_eq!(order.umpire_raw(common::umpire_key()), 3);
    }
    // Force B started at battle readiness: no order, no cause.
    for event in run
        .events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::TaskForceFatigue { task_force, .. } if task_force.raw() == 2))
    {
        assert!(event.causes.is_empty(), "{:?}", event.causes);
    }
}

// ---------------------------------------------------------------------
// The meeting and the night fight.
// ---------------------------------------------------------------------

#[test]
fn the_forces_meet_at_five_nought_at_11_25_hours_into_the_night_of_day_3() {
    for run in batch() {
        let fight = night_fight(run);
        let begun = &run.fights[fight];
        assert_eq!(begun.hex, MEETING, "seed {}", run.seed);
        // Elapsed hours since the scenario began: two days of 24, then
        // 11.25 hours into the night pulse, which starts at hour 0.
        assert_eq!(begun.elapsed_hours, dec("59.25"), "seed {}", run.seed);
        // It is the first fight of the run.
        assert_eq!(fight, 0);
        // Force A steamed 180 nm at 16 kn from (0,0) to (5,0) to get there.
        let steps: Vec<_> = run
            .events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceSteamed {
                    task_force,
                    from,
                    to,
                    distance_nm,
                    speed_kn,
                    ..
                } if task_force.raw() == 1 && e.day == 3 && e.pulse == Some(Pulse::Night) => {
                    Some((*from, *to, *distance_nm, *speed_kn))
                }
                _ => None,
            })
            .collect();
        assert_eq!(
            steps[0],
            (Hex::new(0, 0), MEETING, fx(180), fx(16)),
            "seed {}",
            run.seed
        );
        // Force A does not move before day 3.
        assert!(!run
            .events
            .iter()
            .any(|e| e.day < 3 && matches!(e.kind, EventKind::TaskForceSteamed { .. })));
    }
}

#[test]
fn the_night_fight_sighting_chances_are_0_475_and_0_325_in_every_round() {
    for run in batch() {
        let a = night_chances(run, 0);
        let b = night_chances(run, 1);
        assert!(!a.is_empty() && !b.is_empty(), "seed {}", run.seed);
        assert!(a.iter().all(|c| *c == dec("0.475")), "seed {}", run.seed);
        assert!(b.iter().all(|c| *c == dec("0.325")), "seed {}", run.seed);
        // Each rolls once in round 1.
        let fight = night_fight(run);
        for force in 0..2 {
            let in_round_1 = run
                .rolls
                .iter()
                .filter(|r| r.fight == fight && r.force == force && r.round == 1)
                .count();
            assert_eq!(in_round_1, 1, "seed {}", run.seed);
        }
        // The factor of the fatigue is 0.95 and 0.65.
        let fight_rolls = run.rolls.iter().filter(|r| r.fight == fight);
        for roll in fight_rolls {
            let expected = if roll.force == 0 {
                dec("0.95")
            } else {
                dec("0.65")
            };
            assert_eq!(roll.factor, expected, "seed {}", run.seed);
        }
    }
}

#[test]
fn the_round_1_sightings_stay_within_the_allowed_bands() {
    let runs = batch();
    let a = runs.iter().filter(|r| sights_in_round_1(r, 0)).count();
    let b = runs.iter().filter(|r| sights_in_round_1(r, 1)).count();
    eprintln!("force A sights in round 1 in {a} runs, force B in {b}");
    assert!(
        (412..=538).contains(&a),
        "force A sights in round 1 in {a} runs"
    );
    assert!(
        (266..=384).contains(&b),
        "force B sights in round 1 in {b} runs"
    );
}

#[test]
fn a_fight_by_day_uses_a_base_chance_of_one() {
    // In a fight by day the chance is 1 x the not-alert factor x the
    // fatigue factor: on the morning of day 3, force A at fatigue 10 sights
    // with 0.9 and force B at fatigue 40 with 0.6. Every such roll in the
    // shared batch is checked by `analyse`; here the ones that happen are
    // read.
    let mut seen = 0;
    for run in batch() {
        for roll in run
            .rolls
            .iter()
            .filter(|r| r.pulse == Pulse::Morning && r.day == 3)
        {
            seen += 1;
            let expected = if roll.force == 0 {
                dec("0.9")
            } else {
                dec("0.6")
            };
            assert_eq!(roll.chance, expected, "seed {}", run.seed);
        }
    }
    eprintln!("rolls on the morning of day 3 in the batch: {seen}");
}

// ---------------------------------------------------------------------
// Variants.
// ---------------------------------------------------------------------

/// The chances of both forces in every roll of the night fight of each run
/// of a variant are the figures of the table (force A, force B).
fn assert_night_chances(kind: Kind, a: &str, b: &str) {
    for run in variant_batch(kind) {
        assert!(
            night_chances(run, 0).iter().all(|c| *c == dec(a)),
            "{kind:?} seed {}: force A",
            run.seed
        );
        assert!(
            night_chances(run, 1).iter().all(|c| *c == dec(b)),
            "{kind:?} seed {}: force B",
            run.seed
        );
    }
}

#[test]
fn without_the_fatigue_file_both_sight_with_0_5_and_no_fatigue_is_recorded() {
    assert_night_chances(Kind::NoFile, "0.5", "0.5");
    for run in variant_batch(Kind::NoFile) {
        assert_eq!(run.fatigue_events, 0, "seed {}", run.seed);
        assert!(
            !run.events
                .iter()
                .any(|e| matches!(e.kind, EventKind::TaskForceFatigue { .. })),
            "seed {}",
            run.seed
        );
        // Every sighting roll is at its 4a value, in every pulse: the base
        // by light, times 0.5 if not alert.
        assert!(run.rolls.iter().all(|r| r.factor == Fixed::ONE));
        for roll in &run.rolls {
            let ok = [dec("0.5"), dec("0.25"), Fixed::ONE].contains(&roll.chance);
            assert!(ok, "seed {}: {}", run.seed, roll.chance);
        }
    }
}

#[test]
fn force_b_at_normal_readiness_throughout_sights_with_0_25_in_the_night_fight() {
    // Fatigue 0 and not alert: 0.5 x 0.5. B only rolls while it has not
    // sighted, and a force that is fired on has sighted, so it is never
    // alerted in the middle of a round it rolls in: every roll of the
    // night fight is 0.25.
    assert_night_chances(Kind::BNormal, "0.475", "0.25");
    for run in variant_batch(Kind::BNormal) {
        // B never gained: it was not alert before the fight.
        for pulse in 0..6 {
            assert_eq!(
                run.fatigue[1][pulse],
                Some(Fixed::ZERO),
                "seed {}",
                run.seed
            );
        }
        assert_eq!(run.fatigue[1][6], Some(Fixed::ZERO));
    }
}

#[test]
fn force_b_ordered_to_battle_on_day_3_only_has_fatigue_5() {
    assert_night_chances(Kind::BDayThree, "0.475", "0.475");
    for run in variant_batch(Kind::BDayThree) {
        assert_eq!(run.fatigue[1][6], Some(fx(5)), "seed {}", run.seed);
    }
}

#[test]
fn force_a_at_battle_readiness_from_day_1_has_fatigue_35() {
    assert_night_chances(Kind::ABattleFromDayOne, "0.325", "0.325");
    for run in variant_batch(Kind::ABattleFromDayOne) {
        assert_eq!(run.fatigue[0][6], Some(fx(35)), "seed {}", run.seed);
        assert_eq!(run.fatigue[1][6], Some(fx(35)), "seed {}", run.seed);
    }
}

#[test]
fn force_b_standing_down_on_day_3_is_the_worst_of_all() {
    // Fatigue 30 less 3 is 27, and not alert: 0.5 x 0.5 x 0.73 = 0.1825.
    assert_night_chances(Kind::BStandsDown, "0.475", "0.1825");
    for run in variant_batch(Kind::BStandsDown) {
        assert_eq!(run.fatigue[1][5], Some(fx(30)), "seed {}", run.seed);
        assert_eq!(run.fatigue[1][6], Some(fx(27)), "seed {}", run.seed);
        // It is the lowest of the night chances in the variants.
        assert!(night_chances(run, 1).iter().all(|c| *c < dec("0.25")));
    }
}

#[test]
fn a_force_that_starts_at_100_stays_at_100_and_never_sights() {
    assert_night_chances(Kind::BFull, "0.475", "0");
    for run in variant_batch(Kind::BFull) {
        // It stays at 100: an alert force at 100 gains nothing and records
        // no event.
        assert!(
            !run.events.iter().any(|e| matches!(
                e.kind,
                EventKind::TaskForceFatigue { task_force, .. } if task_force.raw() == 2
            )),
            "seed {}",
            run.seed
        );
        for pulse in 0..9 {
            if let Some(fatigue) = run.fatigue[1][pulse] {
                assert_eq!(fatigue, fx(100), "seed {}", run.seed);
            }
        }
        // Force B never sights force A by rolling, in any pulse, in any
        // round: every one of its rolls has chance 0.
        for roll in run.rolls.iter().filter(|r| r.force == 1) {
            assert_eq!(roll.chance, Fixed::ZERO, "seed {}", run.seed);
            assert!(!roll.sighted, "seed {}", run.seed);
        }
    }
}

#[test]
fn two_days_at_battle_readiness_still_sight_better_than_a_rested_force_caught_unready() {
    // Force B's round 1 chance in the night fight, by variant: 0.325 after
    // two days at battle readiness, more than 0.25 for a rested force caught
    // at normal readiness and less than 0.475 for one that came to battle
    // readiness that evening; 0.1825 for standing down on the last day is
    // the worst of all.
    let first = |runs: &[Run]| night_chances(&runs[0], 1)[0];
    let two_days = first(batch());
    let rested = first(variant_batch(Kind::BNormal));
    let evening = first(variant_batch(Kind::BDayThree));
    let stood_down = first(variant_batch(Kind::BStandsDown));
    assert_eq!(
        [two_days, rested, evening, stood_down],
        [dec("0.325"), dec("0.25"), dec("0.475"), dec("0.1825")]
    );
    assert!(stood_down < rested && rested < two_days && two_days < evening);
}

/// Ships sunk, by side of the force: (force A, force B).
fn sunk(run: &Run) -> [usize; 2] {
    let mut sunk = [0; 2];
    for event in &run.events {
        if let EventKind::ShipSunk { task_force, .. } = event.kind {
            sunk[force_index(task_force)] += 1;
        }
    }
    sunk
}

#[test]
fn what_the_runs_show() {
    // Not a rule: the figures of the report. Ships sunk per run, on average.
    for (label, runs) in [
        ("with the file", batch()),
        ("without the file", variant_batch(Kind::NoFile)),
        ("force B at normal readiness", variant_batch(Kind::BNormal)),
        ("force B ordered on day 3", variant_batch(Kind::BDayThree)),
        (
            "force A at battle from day 1",
            variant_batch(Kind::ABattleFromDayOne),
        ),
        (
            "force B stands down on day 3",
            variant_batch(Kind::BStandsDown),
        ),
        ("force B starts at 100", variant_batch(Kind::BFull)),
    ] {
        let total = runs.iter().fold([0usize; 2], |acc, run| {
            let s = sunk(run);
            [acc[0] + s[0], acc[1] + s[1]]
        });
        let a_round_1 = runs.iter().filter(|r| sights_in_round_1(r, 0)).count();
        let b_round_1 = runs.iter().filter(|r| sights_in_round_1(r, 1)).count();
        let mut other_b = BTreeSet::new();
        for run in runs.iter() {
            for roll in run
                .rolls
                .iter()
                .filter(|r| r.force == 1 && r.pulse != Pulse::Night)
            {
                other_b.insert((roll.day, roll.pulse, roll.chance.to_string()));
            }
        }
        eprintln!(
            "{label}: A sunk {:.3}, B sunk {:.3} per run; round 1 sightings A {a_round_1} B {b_round_1}; B's rolls by day: {other_b:?}",
            total[0] as f64 / SEEDS as f64,
            total[1] as f64 / SEEDS as f64
        );
    }
}

// ---------------------------------------------------------------------
// What cannot happen.
// ---------------------------------------------------------------------

#[test]
fn a_fatigue_file_of_zero_figures_is_the_same_run_as_none_event_for_event() {
    // With both force figures 0 nothing changes: no fatigue event, every
    // factor is one, and every event, cause and hash is the run of the
    // scenario with no file. (The breakdown of a roll alone has one more
    // factor, listed as the fatigue factor.)
    let without = variant_batch(Kind::NoFile);
    let zero = variant_batch(Kind::ZeroFigures);
    for (a, b) in without.iter().zip(zero) {
        assert_eq!(a.events.len(), b.events.len(), "seed {}", a.seed);
        for (x, y) in a.events.iter().zip(&b.events) {
            assert_eq!((x.id, x.day, x.pulse), (y.id, y.day, y.pulse));
            assert_eq!(x.kind, y.kind, "seed {}", a.seed);
            assert_eq!(x.causes, y.causes, "seed {}", a.seed);
        }
        assert_eq!(a.day_hashes, b.day_hashes, "seed {}", a.seed);
    }
}

#[test]
fn with_the_file_the_run_differs_from_the_run_without_it_only_as_the_rolls_make_it() {
    // The draws of a sighting roll are keyed on the seed, the day, the
    // pulse and the two forces, and not on the chance, so for a seed in
    // which every roll comes out as it did without the file everything but
    // the fatigue events and the chance of each roll is the same run, with
    // the events renumbered.
    let without = variant_batch(Kind::NoFile);
    let mut same = 0;
    for (with, plain) in batch().iter().zip(without) {
        let outcomes = |run: &Run| -> Vec<(u32, Pulse, u32, usize, bool)> {
            run.rolls
                .iter()
                .map(|r| (r.day, r.pulse, r.round, r.force, r.sighted))
                .collect()
        };
        if outcomes(with) != outcomes(plain) {
            continue;
        }
        same += 1;
        let kept: Vec<&Event> = with
            .events
            .iter()
            .filter(|e| !matches!(e.kind, EventKind::TaskForceFatigue { .. }))
            .collect();
        assert_eq!(kept.len(), plain.events.len(), "seed {}", with.seed);
        let renumbered: BTreeMap<u32, u32> = kept
            .iter()
            .zip(&plain.events)
            .map(|(e, p)| {
                (
                    e.id.umpire_raw(common::umpire_key()),
                    p.id.umpire_raw(common::umpire_key()),
                )
            })
            .collect();
        for (e, p) in kept.iter().zip(&plain.events) {
            assert_eq!((e.day, e.pulse), (p.day, p.pulse), "seed {}", with.seed);
            let (mut left, mut right) = (e.kind.clone(), p.kind.clone());
            for kind in [&mut left, &mut right] {
                if let EventKind::SightingRoll {
                    chance,
                    fatigue_factor,
                    ..
                } = kind
                {
                    *chance = Fixed::ZERO;
                    *fatigue_factor = Fixed::ONE;
                }
            }
            assert_eq!(left, right, "seed {}", with.seed);
            let causes: Vec<Cause> = e
                .causes
                .iter()
                .map(|cause| match cause {
                    Cause::Event(id) => Cause::Event(sim_core::ids::EventId::for_umpire(
                        renumbered[&id.umpire_raw(common::umpire_key())],
                        common::umpire_key(),
                    )),
                    order => *order,
                })
                .collect();
            assert_eq!(causes, p.causes, "seed {}", with.seed);
        }
    }
    eprintln!("seeds in which every roll came out as without the file: {same} of {SEEDS}");
    assert!(same > 500, "most seeds roll the same: {same}");
}

#[test]
fn fatigue_changes_no_gun_or_torpedo_chance() {
    // Every salvo's chance is what scenario 4a's rule gives for its light,
    // surprise, fire and fire control: base x surprised x lit (x fire
    // control for guns), with no fatigue among the factors. Checked in the
    // scenario and in the variant where force B is the more tired.
    let mut salvos = 0usize;
    for run in batch().iter().chain(variant_batch(Kind::ABattleFromDayOne)) {
        let mut fight = usize::MAX;
        let mut surprised: BTreeSet<(usize, u32, usize)> = BTreeSet::new();
        // Surprise comes before the salvos of its round.
        for event in &run.events {
            match &event.kind {
                EventKind::EngagementBegun { .. } => fight = fight.wrapping_add(1),
                EventKind::ForceSurprised {
                    round, task_force, ..
                } => {
                    surprised.insert((fight, *round, force_index(*task_force)));
                }
                EventKind::Salvo {
                    round,
                    target,
                    weapon,
                    chance,
                    ..
                } => {
                    salvos += 1;
                    let target_force = usize::from(target.raw() > 3);
                    let dark = event.pulse == Some(Pulse::Night);
                    let breakdown = event.breakdown.as_ref().expect("a breakdown");
                    let factor = |name: &str| {
                        breakdown
                            .factors
                            .iter()
                            .find(|f| f.name == name)
                            .map(|f| f.value)
                            .unwrap_or_else(|| panic!("seed {}: no factor {name}", run.seed))
                    };
                    assert!(
                        !breakdown.factors.iter().any(|f| f.name.contains("fatigue")),
                        "seed {}: fatigue is not a factor of a salvo",
                        run.seed
                    );
                    let base = match (weapon, dark) {
                        (sim_core::events::Weapon::Gun, true) => GUN_BASE_NIGHT,
                        (sim_core::events::Weapon::Gun, false) => GUN_BASE_DAY,
                        (sim_core::events::Weapon::Torpedo, true) => TORPEDO_BASE_NIGHT,
                        (sim_core::events::Weapon::Torpedo, false) => TORPEDO_BASE_DAY,
                        (sim_core::events::Weapon::Bomb, _) => unreachable!("no bombs here"),
                    };
                    assert_eq!(factor("base_chance"), base, "seed {}", run.seed);
                    let was_surprised = surprised.contains(&(fight, *round, target_force));
                    let surprised_factor = if was_surprised { SURPRISED } else { Fixed::ONE };
                    assert_eq!(
                        factor("surprised_factor"),
                        surprised_factor,
                        "seed {}",
                        run.seed
                    );
                    let lit = factor("lit_factor");
                    assert!(
                        lit == Fixed::ONE || (lit == LIT && dark),
                        "seed {}: lit factor {lit}",
                        run.seed
                    );
                    let mut expected = base * surprised_factor * lit;
                    if *weapon == sim_core::events::Weapon::Gun {
                        let control = factor("fire_control_factor");
                        assert!(
                            [Fixed::ONE, millionths(500_000), millionths(250_000)]
                                .contains(&control),
                            "seed {}: fire control factor {control}",
                            run.seed
                        );
                        expected = expected * control;
                    }
                    assert_eq!(
                        *chance,
                        expected,
                        "seed {} salvo #{}",
                        run.seed,
                        event.id.umpire_raw(common::umpire_key())
                    );
                }
                _ => {}
            }
        }
    }
    eprintln!("salvos checked: {salvos}");
    assert!(salvos > 1000);
}

#[test]
fn fatigue_changes_no_speed_or_route_before_the_fight() {
    // The first leg of force A on the night of day 3 is the same with and
    // without the file: 180 nm at 16 kn, and fuel burned the same.
    for (with, without) in batch().iter().zip(variant_batch(Kind::NoFile)).take(200) {
        let first_steps = |run: &Run| -> Vec<EventKind> {
            run.events
                .iter()
                .filter(|e| {
                    e.day == 3
                        && e.pulse == Some(Pulse::Night)
                        && matches!(
                            e.kind,
                            EventKind::TaskForceSteamed { .. } | EventKind::FuelBurned { .. }
                        )
                })
                .take(4)
                .map(|e| e.kind.clone())
                .collect()
        };
        assert_eq!(
            first_steps(with),
            first_steps(without),
            "seed {}",
            with.seed
        );
    }
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn a_higher_fatigue_never_gives_a_higher_chance_to_sight() {
    // Force B at battle readiness from the start, with a starting fatigue
    // from 0 to 100: at the night fight its fatigue is the start plus 35,
    // 100 at most, and its chance 0.5 x (100 less that) over 100.
    let mut last: Option<(i64, Fixed)> = None;
    for start in [0, 10, 20, 30, 40, 50, 55, 60, 80, 100] {
        let scenario = b_starting_at(start);
        let mut plan = Plan::base();
        plan.start[1] = start;
        let run = run_seed(&scenario, &plan, 1, true);
        let chance = night_chances(&run, 1)[0];
        let fatigue = (start + 35).min(100);
        assert_eq!(
            chance,
            Fixed::from_raw(500_000) * fatigue_factor(fx(fatigue)),
            "start {start}"
        );
        if let Some((_, before)) = last {
            assert!(chance <= before, "start {start}");
        }
        last = Some((start, chance));
    }
    assert_eq!(last.expect("ran").1, Fixed::ZERO);
}

#[test]
fn more_pulses_at_battle_readiness_never_give_a_lower_fatigue() {
    // Force B's fatigue at the start of the night fight: 0 at normal
    // readiness throughout, 5 ordered on day 3, 20 on day 2, 35 from day 1.
    let at_fight = |runs: &[Run]| runs[0].fatigue[1][6].expect("afloat");
    let fatigues = [
        at_fight(variant_batch(Kind::BNormal)),
        at_fight(variant_batch(Kind::BDayThree)),
        at_fight(variant_batch(Kind::BDayTwo)),
        at_fight(batch()),
    ];
    assert_eq!(fatigues, [fx(0), fx(5), fx(20), fx(35)]);
    assert!(fatigues.windows(2).all(|pair| pair[0] <= pair[1]));
    // Every run of each variant agrees.
    for kind in [Kind::BNormal, Kind::BDayThree, Kind::BDayTwo] {
        let first = at_fight(variant_batch(kind));
        assert!(variant_batch(kind)
            .iter()
            .all(|r| r.fatigue[1][6] == Some(first)));
    }
}

#[test]
fn recording_breakdowns_changes_neither_the_events_nor_the_hashes() {
    let (scenario, plan) = build(Kind::Base);
    for seed in 1..=40 {
        let with = run_seed(&scenario, &plan, seed, true);
        let without = run_seed(&scenario, &plan, seed, false);
        assert_eq!(with.day_hashes, without.day_hashes, "seed {seed}");
        assert_eq!(with.events.len(), without.events.len());
        for (x, y) in with.events.iter().zip(&without.events) {
            assert_eq!((x.id, x.day, x.pulse), (y.id, y.day, y.pulse));
            assert_eq!(x.kind, y.kind, "seed {seed}");
            assert_eq!(x.causes, y.causes, "seed {seed}");
            assert!(y.breakdown.is_none());
        }
    }
}

#[test]
fn the_same_seed_gives_the_same_hashes_twice_and_seeds_differ() {
    let (scenario, plan) = build(Kind::Base);
    let mut all = Vec::new();
    for seed in 1..=30 {
        let first = run_seed(&scenario, &plan, seed, true);
        let second = run_seed(&scenario, &plan, seed, true);
        assert_eq!(first.day_hashes, second.day_hashes, "seed {seed}");
        assert_eq!(first.day_hashes.len(), 3);
        all.push(first.day_hashes);
    }
    assert!(all.windows(2).any(|pair| pair[0] != pair[1]));
    // The file's own seed gives the same hashes as a seeded copy.
    let own = {
        let mut run = load();
        run.run_to_end().expect("runs");
        run.world.day_hashes().to_vec()
    };
    let copy = {
        let mut run = load().with_seed(1942).expect("a seed");
        run.run_to_end().expect("runs");
        run.world.day_hashes().to_vec()
    };
    assert_eq!(own, copy);
}

#[test]
fn a_side_sees_its_own_force_fatigue_and_never_the_enemys() {
    let mut scenario = load();
    scenario.run_to_end().expect("runs");
    for (side, own) in [(Side::Allied, 2), (Side::Japanese, 1)] {
        let view = scenario.world.side_view(side);
        let forces: Vec<_> = view.task_forces().collect();
        assert_eq!(forces.len(), 1);
        assert_eq!(forces[0].id.raw(), own);
        let fatigue = forces[0].fatigue;
        assert!(fatigue >= Fixed::ZERO && fatigue <= fx(100));
        assert_eq!(
            fatigue,
            scenario.world.umpire().task_forces[&forces[0].id].fatigue
        );
    }
}

#[test]
fn a_tired_force_hashes_differently_from_a_rested_one() {
    // Force B is tired after the first pulse, so the first day's hash with
    // the file differs from the run without it; the zero-figures run, in
    // which nothing is tired, has the hashes of the run without the file
    // (`a_fatigue_file_of_zero_figures_is_the_same_run_as_none_event_for_event`).
    let none = &variant_batch(Kind::NoFile)[0].day_hashes;
    let tired = &batch()[0].day_hashes;
    assert!(none.iter().zip(tired).all(|(a, b)| a != b));
}
