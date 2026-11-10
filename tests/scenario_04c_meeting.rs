//! Scenario 4c2b: forces meet and fight in order of clock time, and a force
//! that breaks off avoids action.
//!
//! Stage 4c2b of scenario 4. It proves that two forces under way meet where
//! and when their tracks cross, that a force can steam in, fight and leave
//! within one pulse, and that a fight happens once per pair in a pulse.
//! What breaking off means is proved in `scenario_04c_readiness.rs`, where
//! one attacker meets two defenders.
//!
//! The data is placeholder data: see `data/scenarios/04c_head_on.toml` and
//! `data/scenarios/04c_in_and_out.toml`. The rule is
//! `docs/decisions/0022-meeting-and-breaking-off.md`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! Common to both parts: every ship is the placeholder cruiser, which
//! cruises at 16 kn with a top speed of 32 kn and burns 0.25 t per nm at
//! cruise speed. A force's position is the nearest hex centre, so it enters
//! a hex 20 nm before the centre: 20, 60, 100, 140 and 180 nm along a
//! straight track. At night a force sights with chance 0.5, times 0.5 if it
//! is not alert.
//!
//! ## Part 1: head on
//!
//! Force A, task force 1, Japanese, starts at (0,0) and steams for (10,0).
//! Force B, task force 2, Allied, starts at (10,0) and steams for (0,0).
//! Both cruise at 16 kn and are at battle readiness.
//!
//! Each changes position after 20, 60, 100, 140 and 180 nm, which at 16 kn
//! is 1.25, 3.75, 6.25, 8.75 and 11.25 hours into the night pulse.
//!
//! | Hour  | Force A's position | Force B's position |
//! | ----- | ------------------ | ------------------ |
//! | 0     | (0,0)              | (10,0)             |
//! | 1.25  | (1,0)              | (9,0)              |
//! | 3.75  | (2,0)              | (8,0)              |
//! | 6.25  | (3,0)              | (7,0)              |
//! | 8.75  | (4,0)              | (6,0)              |
//! | 11.25 | (5,0)              | (5,0)              |
//!
//! When two changes fall at the same moment the lower task force ID goes
//! first. At 11.25 hours force A enters (5,0): force B is still in (6,0), so
//! nothing happens. Then force B enters (5,0), where force A now is. They
//! meet in (5,0) at hour 11.25, which is 05:15 on day 1, and fight there.
//! Each ship has steamed 180 nm and burned 45 t.
//!
//! Both forces are alert, so each sights with chance 0.5 a round. In a round
//! where neither has yet sighted, each of "both", "only A", "only B" and
//! "neither" has chance 0.25. Neither ever sights in six rounds with chance
//! 0.25^6 = 0.000244. Otherwise the fight opens in one of three equally
//! likely ways, each with chance 0.999756 / 3 = 0.333252.
//!
//! | Count of runs where, over seeds 1 to 1000 | p        | Mean   | Std dev | Allowed    |
//! | ----------------------------------------- | -------- | ------ | ------- | ---------- |
//! | A has a free round                        | 0.333252 | 333.25 | 14.91   | 274 to 392 |
//! | B has a free round                        | 0.333252 | 333.25 | 14.91   | 274 to 392 |
//! | Both fire from the start                  | 0.333252 | 333.25 | 14.91   | 274 to 392 |
//! | They pass unseen                          | 0.000244 | 0.24   | 0.49    | 0 to 2     |
//!
//! In every run there is exactly one meeting in the night pulse, in (5,0) at
//! hour 11.25, and none before it.
//!
//! After the fight each force has 12 nm of the night left at 16 kn, 6 nm at
//! 8 kn, and none if it cannot move. A force that can move does not fight
//! again at the start of the morning pulse, although both may still be in
//! (5,0) then: at 192 nm force A has left (4,0) and is 32 nm toward (5,0),
//! and force B has left (6,0) and is 32 nm toward (5,0). The two fight again
//! by day only in a run where neither can move.
//!
//! ## Part 2: in and out
//!
//! Force A, task force 1, Japanese, at battle readiness, starts at (0,0) and
//! is ordered to (5,0) and then back to (0,0), at 24 kn. Force B, task force
//! 2, Allied, lies stopped in (5,0) at normal readiness, and its side holds
//! no contact, so it is not alert.
//!
//! At 24 kn burn per mile is multiplied by 1.5 squared: 0.5625 t per nm. In
//! the 12-hour night pulse force A can steam 288 nm.
//!
//! Force A enters (5,0) after 180 nm, 180 / 24 = 7.5 hours into the night
//! pulse, which is 01:30 on day 1. It meets force B and fights there. Each
//! cruiser has burned 180 x 0.5625 = 101.25 t. The chances are those of
//! scenario 4a: A sights at 0.5 and B at 0.25, so A has a free round with
//! chance 0.598331, allowed 537 to 660 runs of 1,000.
//!
//! After the fight force A has 108 nm of the night left at 24 kn. It steams
//! 20 nm on to the centre of (5,0), its first waypoint, and turns back for
//! (0,0).
//!
//! | Slowest ship's propulsion | Speed | Rest of the night   | Night total | Where it ends the night              | Position |
//! | ------------------------- | ----- | ------------------- | ----------- | ------------------------------------ | -------- |
//! | Working                   | 24 kn | 108 nm              | 288 nm      | 88 nm back: left (3,0), 8 nm on      | (3,0)    |
//! | Degraded                  | 16 kn | 108 x 16 / 24 = 72 nm | 252 nm    | 52 nm back: left (4,0), 12 nm on     | (4,0)    |
//! | Out                       | 0 kn  | 0 nm                | 180 nm      | 20 nm short of the centre of (5,0)   | (5,0)    |
//!
//! So a force that comes through with its propulsion working is two hexes
//! from the defender by the end of the night, having arrived, fought and
//! left in one pulse.
//!
//! Fuel and arrival back at (0,0), which is 400 nm in all:
//!
//! | Slowest ship's propulsion | Fuel in the night                 | Left to go by day | Arrives at hour     | Fuel by day      | Fuel left |
//! | ------------------------- | --------------------------------- | ----------------- | ------------------- | ---------------- | --------- |
//! | Working                   | 288 x 0.5625 = 162 t              | 112 nm            | 12 + 4.666666 = 16.666666 | 63 t       | 1775 t    |
//! | Degraded                  | 101.25 + 72 x 0.25 = 119.25 t     | 148 nm            | 12 + 9.25 = 21.25   | 148 x 0.25 = 37 t | 1843.75 t |
//! | Out                       | 101.25 t                          | it does not move  | never               | none             | 1898.75 t |
//!
//! The tables hold when the propulsion state of force A's slowest ship
//! afloat does not change after the night fight. In every run there is
//! exactly one fight between the two in the night pulse. There is a fight by
//! day only in a run where force A cannot move, so that it is still in (5,0)
//! at the start of a day pulse.
//!
//! # Properties
//!
//! - A pair of forces never fights twice in one pulse.
//! - A fight never begins in a hex unless both forces' positions are that
//!   hex at that moment.
//! - No force is ever further along at the end of a pulse than its speed
//!   allows: at most 192 nm in a night at 16 kn, and 288 nm at 24 kn.
//! - A replay of any seed reproduces the daily hash.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::state::{ShipLocation, ShipSystem, SystemState};
use sim_core::time::{pulse_start, Pulse};
use sim_data::Scenario;

const SEEDS: u64 = 1000;
const MEETING: Hex = Hex::new(5, 0);
const ORIGIN: Hex = Hex::new(0, 0);
const A_SHIPS: [u32; 3] = [1, 2, 3];
const B_SHIPS: [u32; 3] = [4, 5, 6];

fn fx(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn millionths(value: i64) -> Fixed {
    Fixed::from_raw(value)
}

fn pulse_index(pulse: Pulse) -> usize {
    match pulse {
        Pulse::Night => 0,
        Pulse::Morning => 1,
        Pulse::Afternoon => 2,
    }
}

const PULSES: [Pulse; 3] = [Pulse::Night, Pulse::Morning, Pulse::Afternoon];

// ---------------------------------------------------------------------
// Running a scenario.
// ---------------------------------------------------------------------

fn load(file: &str) -> Scenario {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../data/scenarios")
        .join(file);
    Scenario::load(&path).expect("the scenario loads")
}

#[derive(Clone, Debug)]
struct ShipEnd {
    location: ShipLocation,
    fuel: Fixed,
}

#[derive(Clone, Debug)]
struct Run {
    seed: u64,
    events: Vec<Event>,
    ships: BTreeMap<u32, ShipEnd>,
    positions: BTreeMap<u32, Hex>,
    hashes: Vec<u64>,
}

fn run_scenario(scenario: &Scenario, seed: u64) -> Run {
    let mut run = scenario.with_seed(seed).expect("a seed can be set");
    run.run_to_end().expect("the run resolves");
    let world = &run.world;
    let truth = world.umpire();
    let events: Vec<Event> = world.events().iter().cloned().collect();
    for (index, event) in events.iter().enumerate() {
        assert_eq!(usize::try_from(event.id.raw()).expect("small"), index + 1);
    }
    Run {
        seed,
        events,
        ships: truth
            .ships
            .iter()
            .map(|(id, ship)| {
                (
                    id.raw(),
                    ShipEnd {
                        location: ship.location,
                        fuel: ship.fuel_t,
                    },
                )
            })
            .collect(),
        positions: truth
            .task_forces
            .iter()
            .map(|(id, force)| (id.raw(), force.position()))
            .collect(),
        hashes: world.day_hashes().to_vec(),
    }
}

fn batch_of(file: &str) -> Vec<Run> {
    let base = load(file);
    (1..=SEEDS).map(|seed| run_scenario(&base, seed)).collect()
}

fn head_on() -> &'static Vec<Run> {
    static BATCH: OnceLock<Vec<Run>> = OnceLock::new();
    BATCH.get_or_init(|| batch_of("04c_head_on.toml"))
}

fn in_and_out() -> &'static Vec<Run> {
    static BATCH: OnceLock<Vec<Run>> = OnceLock::new();
    BATCH.get_or_init(|| batch_of("04c_in_and_out.toml"))
}

// ---------------------------------------------------------------------
// Reading the events of a run.
// ---------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Step {
    event: u32,
    pulse: Pulse,
    from: Hex,
    to: Hex,
    distance: Fixed,
    speed: Fixed,
}

#[derive(Clone, Debug)]
struct Roll {
    round: u32,
    force: u32,
    sighted: bool,
}

#[derive(Clone, Debug)]
struct Fight {
    event: u32,
    day: u32,
    pulse: Pulse,
    a: u32,
    b: u32,
    hex: Hex,
    elapsed: Fixed,
    first_cause: Option<u32>,
    rolls: Vec<Roll>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Opening {
    AFreeRound,
    BFreeRound,
    BothFire,
    PassUnseen,
}

impl Fight {
    fn night(&self) -> bool {
        self.day == 1 && self.pulse == Pulse::Night
    }

    /// How the fight opens, from the sighting rolls alone.
    fn opening(&self) -> Opening {
        let Some(first) = self
            .rolls
            .iter()
            .filter(|r| r.sighted)
            .map(|r| r.round)
            .min()
        else {
            return Opening::PassUnseen;
        };
        let sighted = |force: u32| {
            self.rolls
                .iter()
                .any(|r| r.round == first && r.force == force && r.sighted)
        };
        match (sighted(self.a), sighted(self.b)) {
            (true, false) => Opening::AFreeRound,
            (false, true) => Opening::BFreeRound,
            (true, true) => Opening::BothFire,
            (false, false) => unreachable!("a round with a success"),
        }
    }
}

impl Run {
    fn fights(&self) -> Vec<Fight> {
        let mut fights: Vec<Fight> = Vec::new();
        for event in &self.events {
            match &event.kind {
                EventKind::EngagementBegun {
                    a,
                    b,
                    hex,
                    elapsed_hours,
                } => fights.push(Fight {
                    event: event.id.raw(),
                    day: event.day,
                    pulse: event.pulse.expect("a fight has a pulse"),
                    a: a.raw(),
                    b: b.raw(),
                    hex: *hex,
                    elapsed: *elapsed_hours,
                    first_cause: event.causes.first().and_then(|cause| match cause {
                        Cause::Event(id) => Some(id.raw()),
                        Cause::Order(_) => None,
                    }),
                    rolls: Vec::new(),
                }),
                EventKind::SightingRoll {
                    round,
                    task_force,
                    sighted,
                    ..
                } => fights
                    .last_mut()
                    .expect("a roll inside a fight")
                    .rolls
                    .push(Roll {
                        round: *round,
                        force: task_force.raw(),
                        sighted: *sighted,
                    }),
                _ => {}
            }
        }
        fights
    }

    fn night_fights(&self) -> Vec<Fight> {
        self.fights().into_iter().filter(Fight::night).collect()
    }

    fn steps(&self, force: u32) -> Vec<Step> {
        self.events
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::TaskForceSteamed {
                    task_force,
                    from,
                    to,
                    distance_nm,
                    speed_kn,
                    ..
                } if task_force.raw() == force => Some(Step {
                    event: event.id.raw(),
                    pulse: event.pulse.expect("a step has a pulse"),
                    from: *from,
                    to: *to,
                    distance: *distance_nm,
                    speed: *speed_kn,
                }),
                _ => None,
            })
            .collect()
    }

    /// What one ship burned in the night pulse before event `before`.
    fn night_burn_before(&self, ship: u32, before: u32) -> Fixed {
        self.events
            .iter()
            .filter(|e| e.id.raw() < before && e.pulse == Some(Pulse::Night))
            .filter_map(|e| match &e.kind {
                EventKind::FuelBurned {
                    ship: s, burned_t, ..
                } if s.raw() == ship => Some(*burned_t),
                _ => None,
            })
            .fold(Fixed::ZERO, |sum, t| sum + t)
    }

    /// What one ship burned in the pulses other than the night.
    fn burn_after_night(&self, ship: u32) -> Fixed {
        self.events
            .iter()
            .filter(|e| e.pulse.is_some_and(|p| p != Pulse::Night))
            .filter_map(|e| match &e.kind {
                EventKind::FuelBurned {
                    ship: s, burned_t, ..
                } if s.raw() == ship => Some(*burned_t),
                _ => None,
            })
            .fold(Fixed::ZERO, |sum, t| sum + t)
    }

    fn arrivals(&self, force: u32) -> Vec<(Hex, Fixed)> {
        self.events
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::TaskForceArrived {
                    task_force,
                    hex,
                    elapsed_hours,
                } if task_force.raw() == force => Some((*hex, *elapsed_hours)),
                _ => None,
            })
            .collect()
    }

    /// Whether the ship was afloat after the events that came before the
    /// event numbered `until` (exclusive).
    fn afloat_before(&self, ship: u32, until: u32) -> bool {
        !self.events.iter().any(|e| {
            e.id.raw() < until
                && matches!(&e.kind, EventKind::ShipSunk { ship: s, .. } if s.raw() == ship)
        })
    }

    /// The propulsion of the slowest of `ships` still afloat, from the events
    /// before the event numbered `until` (exclusive). `None` with none afloat.
    fn slowest_before(&self, ships: &[u32], until: u32) -> Option<SystemState> {
        ships
            .iter()
            .copied()
            .filter(|ship| self.afloat_before(*ship, until))
            .map(|ship| {
                self.events
                    .iter()
                    .rfind(|e| {
                        e.id.raw() < until
                            && matches!(&e.kind, EventKind::SystemHit { ship: s, system, .. }
                                if s.raw() == ship && *system == ShipSystem::Propulsion)
                    })
                    .map_or(SystemState::Working, |e| match &e.kind {
                        EventKind::SystemHit { after, .. } => *after,
                        _ => unreachable!("a system hit"),
                    })
            })
            .max()
    }

    /// The same at the end of a pulse of day 1.
    fn slowest_at_end_of(&self, ships: &[u32], pulse: Pulse) -> Option<SystemState> {
        let until = self
            .events
            .iter()
            .find(|e| e.pulse.is_some_and(|p| pulse_index(p) > pulse_index(pulse)))
            .map_or(u32::MAX, |e| e.id.raw());
        self.slowest_before(ships, until)
    }

    /// The slowest ship's propulsion at the start of a pulse.
    fn slowest_at_start_of(&self, ships: &[u32], pulse: Pulse) -> Option<SystemState> {
        let until = self
            .events
            .iter()
            .find(|e| {
                e.pulse
                    .is_some_and(|p| pulse_index(p) >= pulse_index(pulse))
            })
            .map_or(u32::MAX, |e| e.id.raw());
        self.slowest_before(ships, until)
    }

    fn afloat_at_end(&self, ships: &[u32]) -> Vec<u32> {
        ships
            .iter()
            .copied()
            .filter(|ship| self.ships[ship].location != ShipLocation::Sunk)
            .collect()
    }
}

// ---------------------------------------------------------------------
// Properties common to both scenarios.
// ---------------------------------------------------------------------

/// The speed no force may exceed, in knots, and the starting hexes.
fn assert_properties(runs: &[Run], top_speed_kn: i64, starts: [(u32, Hex); 2]) {
    for run in runs {
        let seed = run.seed;
        let fights = run.fights();

        // A pair never fights twice in one pulse.
        let mut seen = BTreeMap::new();
        for fight in &fights {
            let key = (fight.day, pulse_index(fight.pulse), fight.a, fight.b);
            assert!(
                seen.insert(key, fight.event).is_none(),
                "seed {seed}: {:?} fight twice in one pulse",
                (fight.a, fight.b)
            );
        }

        // A fight begins in a hex only if both forces' positions are that hex
        // at that moment.
        let mut positions: BTreeMap<u32, Hex> = starts.iter().copied().collect();
        let mut steps: Vec<(u32, Step)> = Vec::new();
        for (force, _) in starts {
            steps.extend(run.steps(force).into_iter().map(|s| (force, s)));
        }
        steps.sort_by_key(|(_, s)| s.event);
        let mut cursor = 0;
        for fight in &fights {
            while cursor < steps.len() && steps[cursor].1.event < fight.event {
                positions.insert(steps[cursor].0, steps[cursor].1.to);
                cursor += 1;
            }
            assert_eq!(positions[&fight.a], fight.hex, "seed {seed}: force a");
            assert_eq!(positions[&fight.b], fight.hex, "seed {seed}: force b");
        }

        // No force is further along at the end of a pulse than its speed
        // allows.
        for (force, _) in starts {
            for pulse in PULSES {
                let steps: Vec<Step> = run
                    .steps(force)
                    .into_iter()
                    .filter(|s| s.pulse == pulse)
                    .collect();
                let total = steps.iter().fold(Fixed::ZERO, |sum, s| sum + s.distance);
                assert!(
                    total <= fx(top_speed_kn * pulse.hours()),
                    "seed {seed}: force {force} steamed {total:?} nm in {pulse:?}"
                );
                assert!(
                    steps.iter().all(|s| s.speed <= fx(top_speed_kn)),
                    "seed {seed}: force {force} too fast"
                );
            }
        }
    }
}

fn assert_replays(file: &str, runs: &[Run]) {
    let base = load(file);
    let mut all: Vec<Vec<u64>> = Vec::new();
    for seed in [1u64, 2, 77, 537, 1942] {
        let hashes: Vec<Vec<u64>> = (0..2).map(|_| run_scenario(&base, seed).hashes).collect();
        assert_eq!(hashes[0].len(), 1);
        assert_eq!(hashes[0], hashes[1], "seed {seed}");
        all.push(hashes[0].clone());
    }
    assert!(all.windows(2).any(|pair| pair[0] != pair[1]));
    for run in runs.iter().take(3) {
        assert_eq!(run.hashes, run_scenario(&base, run.seed).hashes);
    }
}

#[test]
fn properties_hold_head_on() {
    assert_properties(head_on(), 16, [(1, ORIGIN), (2, Hex::new(10, 0))]);
}

#[test]
fn properties_hold_in_and_out() {
    assert_properties(in_and_out(), 24, [(1, ORIGIN), (2, MEETING)]);
}

#[test]
fn a_replay_reproduces_the_daily_hash_head_on() {
    assert_replays("04c_head_on.toml", head_on());
}

#[test]
fn a_replay_reproduces_the_daily_hash_in_and_out() {
    assert_replays("04c_in_and_out.toml", in_and_out());
}

// ---------------------------------------------------------------------
// Part 1: head on.
// ---------------------------------------------------------------------

/// The night fight of a head-on run: exactly one, at hour 11.25 in (5,0),
/// and the first of the run.
fn head_on_fight(run: &Run) -> Fight {
    let seed = run.seed;
    let fights = run.fights();
    let night: Vec<&Fight> = fights.iter().filter(|f| f.night()).collect();
    assert_eq!(
        night.len(),
        1,
        "seed {seed}: one meeting in the night pulse"
    );
    assert_eq!(fights[0].event, night[0].event, "seed {seed}: none earlier");
    let fight = night[0].clone();
    assert_eq!((fight.a, fight.b), (1, 2), "seed {seed}");
    assert_eq!(fight.hex, MEETING, "seed {seed}");
    assert_eq!(fight.elapsed, millionths(11_250_000), "seed {seed}");
    fight
}

#[test]
fn head_on_they_meet_once_in_5_0_at_hour_11_25() {
    for run in head_on() {
        let fight = head_on_fight(run);
        // The meeting is caused by force B entering (5,0) second: B's steaming
        // is the fight's first cause.
        let cause = fight.first_cause.expect("the fight has an event cause");
        let b_steps = run.steps(2);
        assert_eq!(cause, b_steps[0].event, "seed {}", run.seed);
    }
}

#[test]
fn head_on_each_ship_burned_45_t_before_the_fight() {
    for run in head_on() {
        let seed = run.seed;
        let fight = head_on_fight(run);
        for force in [1u32, 2] {
            let before: Vec<Step> = run
                .steps(force)
                .into_iter()
                .filter(|s| s.event < fight.event)
                .collect();
            assert_eq!(before.len(), 1, "seed {seed}: one segment before the fight");
            assert_eq!(before[0].distance, fx(180), "seed {seed}");
            assert_eq!(before[0].speed, fx(16), "seed {seed}");
            assert_eq!(before[0].to, MEETING, "seed {seed}");
        }
        assert_eq!(run.steps(1)[0].from, ORIGIN, "seed {seed}");
        assert_eq!(run.steps(2)[0].from, Hex::new(10, 0), "seed {seed}");
        for ship in A_SHIPS.iter().chain(&B_SHIPS) {
            assert_eq!(
                run.night_burn_before(*ship, fight.event),
                fx(45),
                "seed {seed}: ship {ship}"
            );
        }
    }
}

#[test]
fn head_on_the_opening_counts_fall_in_the_allowed_bands() {
    let mut counts = [0usize; 4];
    for run in head_on() {
        let slot = match head_on_fight(run).opening() {
            Opening::AFreeRound => 0,
            Opening::BFreeRound => 1,
            Opening::BothFire => 2,
            Opening::PassUnseen => 3,
        };
        counts[slot] += 1;
    }
    eprintln!(
        "head on: A free {}, B free {}, both {}, unseen {}",
        counts[0], counts[1], counts[2], counts[3]
    );
    for count in &counts[..3] {
        assert!((274..=392).contains(count), "count {count}");
    }
    assert!(counts[3] <= 2, "unseen {}", counts[3]);
    assert_eq!(counts.iter().sum::<usize>(), 1000);
}

#[test]
fn head_on_the_rest_of_the_night_follows_each_forces_propulsion() {
    let mut rows = [0usize; 3];
    for run in head_on() {
        let seed = run.seed;
        let fight = head_on_fight(run);
        for (force, ships) in [(1u32, A_SHIPS), (2, B_SHIPS)] {
            let Some(state) = run.slowest_at_end_of(&ships, Pulse::Night) else {
                continue;
            };
            let after: Vec<Step> = run
                .steps(force)
                .into_iter()
                .filter(|s| s.event > fight.event && s.pulse == Pulse::Night)
                .collect();
            let (nm, speed) = match state {
                SystemState::Working => (12, 16),
                SystemState::Degraded => (6, 8),
                SystemState::Out => (0, 0),
            };
            rows[match state {
                SystemState::Working => 0,
                SystemState::Degraded => 1,
                SystemState::Out => 2,
            }] += 1;
            if nm == 0 {
                assert!(after.is_empty(), "seed {seed}: force {force} is stopped");
            } else {
                assert_eq!(after.len(), 1, "seed {seed}: force {force}");
                assert_eq!(after[0].distance, fx(nm), "seed {seed}: force {force}");
                assert_eq!(after[0].speed, fx(speed), "seed {seed}: force {force}");
                assert_eq!(after[0].from, MEETING, "seed {seed}");
                assert_eq!(after[0].to, MEETING, "seed {seed}");
            }
        }
    }
    eprintln!("head on, rest of night rows (working, degraded, out): {rows:?}");
    assert!(rows.iter().all(|n| *n > 0), "every row is met");
}

/// A fight at the start of a day pulse happens exactly when neither force
/// can move, and there is no other fight by day.
#[test]
fn head_on_they_fight_by_day_only_when_neither_can_move() {
    let mut day_fights = 0;
    for run in head_on() {
        let seed = run.seed;
        let fights = run.fights();
        for pulse in [Pulse::Morning, Pulse::Afternoon] {
            let start = pulse_start(1, pulse);
            let a = run.slowest_at_start_of(&A_SHIPS, pulse);
            let b = run.slowest_at_start_of(&B_SHIPS, pulse);
            let neither_moves = a == Some(SystemState::Out) && b == Some(SystemState::Out);
            let here: Vec<&Fight> = fights
                .iter()
                .filter(|f| f.day == 1 && f.pulse == pulse)
                .collect();
            if neither_moves {
                day_fights += 1;
                assert_eq!(here.len(), 1, "seed {seed}: {pulse:?}");
                assert_eq!(here[0].elapsed, start, "seed {seed}");
                assert_eq!(here[0].hex, MEETING, "seed {seed}");
            } else {
                assert!(
                    here.is_empty(),
                    "seed {seed}: {pulse:?} fight with a force that can move"
                );
            }
        }
        // No fight at the start of the morning if either force can move.
        let morning_start = pulse_start(1, Pulse::Morning);
        if run.slowest_at_end_of(&A_SHIPS, Pulse::Night) != Some(SystemState::Out)
            || run.slowest_at_end_of(&B_SHIPS, Pulse::Night) != Some(SystemState::Out)
        {
            assert!(
                fights.iter().all(|f| f.elapsed != morning_start),
                "seed {seed}"
            );
        }
    }
    eprintln!("head on: pulses with a day fight: {day_fights}");
}

// ---------------------------------------------------------------------
// Part 2: in and out.
// ---------------------------------------------------------------------

fn in_and_out_fight(run: &Run) -> Fight {
    let seed = run.seed;
    let night = run.night_fights();
    assert_eq!(night.len(), 1, "seed {seed}: one fight in the night");
    let fight = night[0].clone();
    assert_eq!((fight.a, fight.b), (1, 2), "seed {seed}");
    assert_eq!(fight.hex, MEETING, "seed {seed}");
    assert_eq!(fight.elapsed, millionths(7_500_000), "seed {seed}");
    fight
}

#[test]
fn in_and_out_they_meet_once_in_5_0_at_hour_7_5_with_101_25_t_burned() {
    for run in in_and_out() {
        let seed = run.seed;
        let fight = in_and_out_fight(run);
        assert_eq!(
            run.fights()[0].event,
            fight.event,
            "seed {seed}: none earlier"
        );
        // A's move into (5,0) caused the meeting.
        let a_steps = run.steps(1);
        assert_eq!(fight.first_cause, Some(a_steps[0].event), "seed {seed}");
        assert_eq!(a_steps[0].distance, fx(180), "seed {seed}");
        assert_eq!(a_steps[0].speed, fx(24), "seed {seed}");
        assert_eq!(a_steps[0].from, ORIGIN, "seed {seed}");
        assert_eq!(a_steps[0].to, MEETING, "seed {seed}");
        assert!(a_steps[0].event < fight.event);
        for ship in A_SHIPS {
            assert_eq!(
                run.night_burn_before(ship, fight.event),
                millionths(101_250_000),
                "seed {seed}: ship {ship}"
            );
        }
        // Force B lies stopped: it steams nowhere.
        assert!(run.steps(2).is_empty(), "seed {seed}");
        for ship in B_SHIPS {
            assert_eq!(run.ships[&ship].fuel, fx(2000), "seed {seed}");
        }
    }
}

#[test]
fn in_and_out_a_has_a_free_round_in_537_to_660_runs() {
    let free = in_and_out()
        .iter()
        .filter(|run| in_and_out_fight(run).opening() == Opening::AFreeRound)
        .count();
    eprintln!("in and out: A free round {free}");
    assert!((537..=660).contains(&free), "count {free}");
}

#[derive(Default, Debug)]
struct Rows {
    rows: [usize; 3],
    changed: usize,
    none_afloat: usize,
}

fn in_and_out_tables() -> Rows {
    let mut out = Rows::default();
    for run in in_and_out() {
        let seed = run.seed;
        let fight = in_and_out_fight(run);
        let night = run.slowest_at_end_of(&A_SHIPS, Pulse::Night);
        let morning = run.slowest_at_end_of(&A_SHIPS, Pulse::Morning);
        let day = run.slowest_at_end_of(&A_SHIPS, Pulse::Afternoon);
        let Some(state) = night else {
            out.none_afloat += 1;
            continue;
        };
        if morning != night || day != night {
            out.changed += 1;
            continue;
        }
        // The force has not split: the straggler instruction is the default.
        let forces: Vec<u32> = run.positions.keys().copied().collect();
        assert_eq!(forces, vec![1, 2], "seed {seed}: no force was made");
        let afloat = run.afloat_at_end(&A_SHIPS);
        let night_steps: Vec<Step> = run
            .steps(1)
            .into_iter()
            .filter(|s| s.pulse == Pulse::Night)
            .collect();
        let night_total = night_steps
            .iter()
            .fold(Fixed::ZERO, |sum, s| sum + s.distance);
        let after: Vec<Step> = run
            .steps(1)
            .into_iter()
            .filter(|s| s.pulse != Pulse::Night)
            .collect();
        let arrivals = run.arrivals(1);
        let after_fight: Vec<&Step> = night_steps
            .iter()
            .filter(|s| s.event > fight.event)
            .collect();

        // No fight by day unless A cannot move.
        let day_fights = run.fights().len() - 1;
        match state {
            SystemState::Working => {
                out.rows[0] += 1;
                assert_eq!(after_fight.len(), 1, "seed {seed}");
                assert_eq!(after_fight[0].distance, fx(108), "seed {seed}");
                assert_eq!(after_fight[0].speed, fx(24), "seed {seed}");
                assert_eq!(after_fight[0].from, MEETING, "seed {seed}");
                assert_eq!(after_fight[0].to, Hex::new(3, 0), "seed {seed}");
                assert_eq!(night_total, fx(288), "seed {seed}");
                assert_eq!(day_fights, 0, "seed {seed}");
                assert!(after.iter().all(|s| s.speed == fx(24)), "seed {seed}");
                assert_eq!(
                    after.iter().fold(Fixed::ZERO, |sum, s| sum + s.distance),
                    fx(112),
                    "seed {seed}"
                );
                assert_eq!(
                    arrivals,
                    vec![(ORIGIN, millionths(16_666_666))],
                    "seed {seed}"
                );
                for ship in &afloat {
                    assert_eq!(
                        run.burn_after_night(*ship),
                        fx(63),
                        "seed {seed} ship {ship}"
                    );
                    assert_eq!(run.ships[ship].fuel, fx(1775), "seed {seed} ship {ship}");
                }
            }
            SystemState::Degraded => {
                out.rows[1] += 1;
                assert_eq!(after_fight.len(), 1, "seed {seed}");
                assert_eq!(after_fight[0].distance, fx(72), "seed {seed}");
                assert_eq!(after_fight[0].speed, fx(16), "seed {seed}");
                assert_eq!(after_fight[0].to, Hex::new(4, 0), "seed {seed}");
                assert_eq!(night_total, fx(252), "seed {seed}");
                assert_eq!(day_fights, 0, "seed {seed}");
                assert!(after.iter().all(|s| s.speed == fx(16)), "seed {seed}");
                assert_eq!(
                    after.iter().fold(Fixed::ZERO, |sum, s| sum + s.distance),
                    fx(148),
                    "seed {seed}"
                );
                assert_eq!(
                    arrivals,
                    vec![(ORIGIN, fx(21) + millionths(250_000))],
                    "seed {seed}"
                );
                for ship in &afloat {
                    assert_eq!(
                        run.burn_after_night(*ship),
                        fx(37),
                        "seed {seed} ship {ship}"
                    );
                    assert_eq!(
                        run.ships[ship].fuel,
                        fx(1843) + millionths(750_000),
                        "seed {seed} ship {ship}"
                    );
                }
            }
            SystemState::Out => {
                out.rows[2] += 1;
                assert!(after_fight.is_empty(), "seed {seed}");
                assert_eq!(night_total, fx(180), "seed {seed}");
                assert_eq!(
                    night_steps.last().map(|s| s.to),
                    Some(MEETING),
                    "seed {seed}"
                );
                assert!(arrivals.is_empty(), "seed {seed}: never arrives");
                assert!(after.is_empty(), "seed {seed}: it does not move");
                assert_eq!(run.positions[&1], MEETING, "seed {seed}");
                for ship in &afloat {
                    assert_eq!(run.burn_after_night(*ship), Fixed::ZERO, "seed {seed}");
                    assert_eq!(
                        run.ships[ship].fuel,
                        fx(1898) + millionths(750_000),
                        "seed {seed} ship {ship}"
                    );
                }
            }
        }
    }
    out
}

#[test]
fn in_and_out_the_tables_hold_for_every_run_of_each_kind() {
    let out = in_and_out_tables();
    eprintln!("in and out tables: {out:?}");
    assert!(out.rows.iter().all(|n| *n > 0), "every row is met: {out:?}");
}

/// There is a fight by day only where A cannot move, and then at the start
/// of the day pulse.
#[test]
fn in_and_out_a_fights_by_day_only_when_it_cannot_move() {
    for run in in_and_out() {
        let seed = run.seed;
        let fights = run.fights();
        for pulse in [Pulse::Morning, Pulse::Afternoon] {
            let a = run.slowest_at_start_of(&A_SHIPS, pulse);
            let b = run.slowest_at_start_of(&B_SHIPS, pulse);
            let here: Vec<&Fight> = fights
                .iter()
                .filter(|f| f.day == 1 && f.pulse == pulse)
                .collect();
            if a == Some(SystemState::Out) && b.is_some() {
                assert_eq!(here.len(), 1, "seed {seed}: {pulse:?}");
                assert_eq!(here[0].elapsed, pulse_start(1, pulse), "seed {seed}");
                assert_eq!(here[0].hex, MEETING, "seed {seed}");
            } else {
                assert!(here.is_empty(), "seed {seed}: {pulse:?}");
            }
        }
    }
}
