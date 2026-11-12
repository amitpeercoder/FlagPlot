//! Scenario 4c2a: an attacking force at battle readiness meets two defending
//! forces in one hex, one after the other.
//!
//! Stage 4c2a of scenario 4. It proves that readiness is set by order, that
//! a force at normal readiness is alert only while alerted, that a contact
//! held nearby alerts it, that a fight in its hex may alarm it, and that a
//! speed order in knots is obeyed.
//!
//! The data is placeholder data: see `data/scenarios/04c_three_forces.toml`.
//! The rule is `docs/decisions/0021-readiness-and-alarm.md`, with
//! `docs/decisions/0014-surface-combat.md` for the fight itself.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! Inputs, from the data files:
//!
//! - Force A, task force 1, Japanese: cruisers 1, 2 and 3, at battle
//!   readiness. It starts at (0,0) and is ordered to (10,0) at 24 kn.
//! - Force B, task force 2, Allied: cruisers 4, 5 and 6. Force C, task force
//!   3, Allied: cruisers 7, 8 and 9. Both lie stopped in (7,0) at normal
//!   readiness. The Allied side holds no contact.
//! - All three are ordered to engage and to break off at 0.5.
//! - Placeholder cruiser: cruises at 16 kn, top speed 32 kn, burns 0.25 t
//!   per nm at cruise speed.
//! - At night a force sights with chance 0.5, times 0.5 if it is not alert.
//! - A force at normal readiness is alerted by a contact within 3 hexes that
//!   is no more than 2 pulses old, and by a fight in its hex with chance 0.5.
//!
//! ## The speed order
//!
//! 24 kn is below the cruiser's top speed of 32 kn, so force A steams at
//! 24 kn. Fuel: 24 kn is 1.5 times cruise speed, so burn per mile is
//! multiplied by 1.5 squared, 2.25: 0.5625 t per nm.
//!
//! Force A enters (7,0), where forces B and C lie, 20 nm short of its
//! centre: after 260 nm, at 260 / 24 = 10.833333 hours into the night pulse.
//! It meets both at that moment and fights them one after the other
//! (decision 0022). By then each cruiser has burned 260 x 0.5625 = 146.25 t.
//!
//! ## Who is alert when the fights begin
//!
//! At the start of the night pulse the Allied side holds no contact, so
//! forces B and C are not alerted. Force A is at battle readiness.
//!
//! | Force | Readiness | Alerted | Alert | Sighting chance |
//! | ----- | --------- | ------- | ----- | --------------- |
//! | A     | Battle    | no      | yes   | 0.5             |
//! | B     | Normal    | no      | no    | 0.25            |
//! | C     | Normal    | no      | no    | 0.25            |
//!
//! ## The first fight: A against B
//!
//! Meetings at the same moment are taken in task force ID order, so A fights
//! B first. The
//! chances are those of scenario 4a: A has a free round with chance
//! 0.598331, B with 0.199444, both fire from the start with 0.199444, and
//! they pass unseen with 0.002781.
//!
//! ## The alarm
//!
//! If a salvo was fired in the first fight, force C has a 0.5 chance to be
//! alarmed. A salvo is fired unless the forces passed unseen, so the chance
//! that C is alarmed is 0.997219 x 0.5 = 0.498610.
//!
//! Force B is already alert by then if it sighted or was fired on. It was a
//! party to that fight and gets no alarm roll for it. Every roll is
//! recorded, whether or not it succeeds.
//!
//! If A and B passed unseen, B is still not alert when A fights C, and it is
//! then a bystander to that second fight: it gets one roll for it.
//!
//! ## The second fight: A against C
//!
//! Force A is still at 0.5. Force C sights at 0.5 if it was alarmed and at
//! 0.25 if it was not. In every run the test reads force C's chance in round
//! 1 of the second fight from the sighting event and asserts it is 0.5
//! exactly when an alarm event for C came before it, and 0.25 otherwise.
//!
//! Force A's ships fired their torpedoes in the first fight if they fired at
//! all, so no ship of force A fires torpedoes twice. A ship of force A sunk
//! in the first fight takes no part in the second.
//!
//! ## Over 1,000 runs
//!
//! The scenario runs with seeds 1 to 1000. Each count must be within four
//! standard deviations of its mean.
//!
//! | Count of runs where                     | p        | Mean   | Std dev | Allowed    |
//! | --------------------------------------- | -------- | ------ | ------- | ---------- |
//! | A has a free round against B            | 0.598331 | 598.33 | 15.50   | 537 to 660 |
//! | A and B pass unseen                     | 0.002781 | 2.78   | 1.67    | 0 to 9     |
//! | C is alarmed                            | 0.498610 | 498.61 | 15.81   | 436 to 561 |
//!
//! Of the runs where C was alarmed and the second fight took place, C sights
//! in round 1 in about half. Of those where it was not alarmed, in about a
//! quarter. With n such runs and k sightings, (k - np)^2 <= 16np(1 - p) for
//! p = 0.5 and p = 0.25 in turn.
//!
//! Force A never rolls: it is at battle readiness. Force B rolls only in a
//! run where it and A passed unseen, once, for the second fight. Force C
//! does not roll when A and B passed unseen, and never rolls more than once
//! in the night pulse.
//!
//! Corrected by the architect on review. The first version said no run has
//! an alarm event for B, which forgot that B is a bystander to the second
//! fight when the first never opened.
//!
//! ## After the fights
//!
//! Force A keeps its stragglers, the default, and steams on for (10,0) at
//! 24 kn or the best speed of its slowest ship afloat. A cruiser with
//! degraded propulsion makes 32 x 0.5 = 16 kn, which is below 24.
//!
//! At 24 kn force A had 288 - 260 = 28 nm of the night left. At 16 kn that
//! becomes 28 x 16 / 24 = 18.666666 nm, truncated to six places.
//!
//! | Slowest ship's propulsion | Speed | Rest of the night | Night total   | Fuel in the night                    |
//! | ------------------------- | ----- | ----------------- | ------------- | ------------------------------------ |
//! | Working                   | 24 kn | 28 nm             | 288 nm        | 288 x 0.5625 = 162 t                 |
//! | Degraded                  | 16 kn | 18.666666 nm      | 278.666666 nm | 146.25 + 18.666666 x 0.25 = 150.916666 t |
//! | Out                       | 0 kn  | 0 nm              | 260 nm        | 146.25 t                             |
//!
//! (18.666666 x 0.25 is 4.6666665, which truncates to 4.666666.)
//!
//! By day it steams the rest of the 400 nm to (10,0):
//!
//! | Slowest ship's propulsion | Left to go    | Takes          | Arrives at hour | Fuel by day  | Fuel left     |
//! | ------------------------- | ------------- | -------------- | --------------- | ------------ | ------------- |
//! | Working                   | 112 nm        | 4.666666 hours | 16.666666       | 63 t         | 1775 t        |
//! | Degraded                  | 121.333334 nm | 7.583333 hours | 19.583333       | 30.333333 t  | 1818.750001 t |
//! | Out                       | it does not move | never       | never           | none         | 1853.75 t     |
//!
//! For the degraded row, by pulse: the morning is 6 hours at 16 kn, 96 nm
//! and 24 t. The afternoon is the last 25.333334 nm, which takes 1.583333
//! hours and burns 25.333334 x 0.25 = 6.333333 t (6.3333335 truncated). So
//! it arrives at hour 18 + 1.583333, and has 2000 - 150.916666 - 24 -
//! 6.333333 = 1818.750001 t left.
//!
//! Hours are counted from the start of the scenario. A force that can move
//! is leaving, so it does not fight again at the start of the morning pulse.
//! If force A has a ship dead in the water it stays in (7,0) and fights both
//! forces again at the start of each day pulse. If that ship then sinks,
//! force A moves on at the speed of its slowest ship still afloat. The
//! tables hold for every ship of force A afloat at the end, whenever its
//! slowest ship had the same propulsion state from the night fights to the
//! end of the day.
//!
//! Corrected by the architect for decision 0022. Before it, the fights were
//! at the end of the night pulse and the night total was always 288 nm.
//!
//! ## Breaking off
//!
//! If force A breaks off from force B, it avoids action for the rest of the
//! pulse, and that includes force C, which it meets at the same moment. In
//! those runs force A never has a free round against C: it slips away if it
//! sights C before C sights it, and otherwise the fight ends no later than
//! the end of the first round in which A is fired on.
//!
//! It often ends sooner still. A force over its break-off level breaks off
//! at the end of a round whatever happened in it (decision 0014), and the
//! level is counted over the ships the force has when the fight begins. If
//! half or more of the ships force A brings to C are crippled, then unless
//! A slips away in round 1 the fight ends at the end of round 1: with A
//! breaking off, fired on or not, or, checked first, because C's fire sank
//! the last ship A had afloat. A ship sunk in the fight with B has left the
//! force and is not counted against it again, so a force A that lost ships
//! outright to B may be under its level when it meets C; that fight ends
//! with the first round in which A is fired on, which can be later than
//! round 1. (Corrected twice after the code was run: the first version left
//! the break-off level out, and the second took the ships lost to B to
//! count still, which was true in every run only while ships sank slowly,
//! before decision 0033.)
//!
//! ## Alertness from belief
//!
//! One run each, with the Allied side starting with a contact. The test
//! reads force B's sighting chance in round 1 of the first fight, which does
//! not depend on any roll.
//!
//! | Starting Allied contact        | Hexes from (7,0) | Age at the start of the night | B and C alerted | B's chance |
//! | ------------------------------ | ---------------- | ----------------------------- | --------------- | ---------- |
//! | None                           |                  |                               | no              | 0.25       |
//! | At (5,0), fresh                | 2                | 0                             | yes             | 0.5        |
//! | At (4,0), fresh                | 3                | 0                             | yes             | 0.5        |
//! | At (3,0), fresh                | 4                | 0                             | no              | 0.25       |
//! | At (5,0), 2 pulses old         | 2                | 2                             | yes             | 0.5        |
//! | At (5,0), 3 pulses old         | 2                | 3                             | no              | 0.25       |
//!
//! A contact 3 pulses old is still held (it is dropped at 5), so the last row
//! tests the age limit and not a dropped contact. In the rows where B and C are alerted, an event records each
//! being alerted and cites the report behind the contact, and force C needs
//! no alarm and gets no alarm roll.
//!
//! Two more single runs:
//!
//! - Force A at normal readiness, no contacts on either side: A's chance in
//!   round 1 is 0.25, the same as B's.
//! - Force A ordered to 40 kn: it steams at 32 kn, its top speed. It enters
//!   (7,0) after 260 nm, at 260 / 32 = 8.125 hours into the night pulse, and
//!   meets and fights both forces there. Before decision 0022 it passed
//!   through untouched. Burn per mile is multiplied by 4: 1 t per nm. If it
//!   comes through with its propulsion working it has 384 - 260 = 124 nm of
//!   the night left, ends the night nine hexes and 24 nm along with its
//!   position (10,0), and arrives there at hour 12.5 with 1600 t left.
//!
//! # Properties
//!
//! - A force at battle readiness is always alert. A force at normal
//!   readiness with no contact nearby, not alarmed, not fired on and not
//!   having sighted, is never alert.
//! - Being alerted never lowers a force's sighting chance.
//! - Scenario 4a keeps every number: its attacker at battle readiness and
//!   its defender at normal readiness give the chances its hand calculation
//!   was written for.
//! - A replay of any seed reproduces the daily hash.

mod common;

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{Cause, EngagementEndReason, Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::state::{ShipLocation, ShipSystem, Side, SystemState};
use sim_core::time::Pulse;
use sim_data::Scenario;

const SEEDS: u64 = 1000;
/// The seed in the scenario file.
const FILE_SEED: u64 = 1942;
const SCENARIO_FILE: &str = "scenarios/04c_three_forces.toml";
/// The ships of force A, B and C (task forces 1, 2 and 3).
const A_SHIPS: [u32; 3] = [1, 2, 3];
const HEX_NM: i64 = 40;
/// Where forces B and C lie, and where A is after the night.
const MEETING: Hex = Hex::new(7, 0);
const DESTINATION: Hex = Hex::new(10, 0);

fn fx(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn millionths(value: i64) -> Fixed {
    Fixed::from_raw(value)
}

/// 0.5 and 0.25: the night sighting chance with and without the alert
/// factor, written out from the hand calculation.
const HALF: Fixed = Fixed::from_raw(500_000);
const QUARTER: Fixed = Fixed::from_raw(250_000);

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/04c_three_forces.toml")
}

fn load() -> Scenario {
    common::load(&scenario_path()).expect("scenario 4c2a loads")
}

fn pulse_index(pulse: Pulse) -> usize {
    match pulse {
        Pulse::Night => 0,
        Pulse::Morning => 1,
        Pulse::Afternoon => 2,
    }
}

// ---------------------------------------------------------------------
// What a run leaves behind: its events, its final state, its hashes.
// ---------------------------------------------------------------------

#[derive(Clone, Debug)]
struct ShipEnd {
    location: ShipLocation,
    fuel: Fixed,
    propulsion: SystemState,
}

#[derive(Clone, Debug)]
struct RunData {
    seed: u64,
    events: Vec<Event>,
    ships: BTreeMap<u32, ShipEnd>,
    positions: BTreeMap<u32, Hex>,
    hashes: Vec<u64>,
}

fn snapshot(seed: u64, scenario: &Scenario) -> RunData {
    let world = &scenario.world;
    let truth = world.umpire();
    RunData {
        seed,
        events: world.events().iter().cloned().collect(),
        ships: truth
            .ships
            .iter()
            .map(|(id, ship)| {
                (
                    id.raw(),
                    ShipEnd {
                        location: ship.location,
                        fuel: ship.fuel_t,
                        propulsion: ship.systems.propulsion,
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

fn run_scenario(scenario: &Scenario, seed: u64) -> RunData {
    let mut run = scenario.with_seed(seed).expect("a seed can be set");
    run.run_to_end().expect("the run resolves");
    snapshot(seed, &run)
}

/// Seeds 1 to 1000, run once and shared by every test below. Breakdowns are
/// recorded (the default of `SimConfig`).
fn batch() -> &'static Vec<(RunData, Parsed)> {
    static BATCH: OnceLock<Vec<(RunData, Parsed)>> = OnceLock::new();
    BATCH.get_or_init(|| {
        let base = load();
        (1..=SEEDS)
            .map(|seed| {
                let run = run_scenario(&base, seed);
                let parsed = parse(seed, &run.events);
                (run, parsed)
            })
            .collect()
    })
}

// ---------------------------------------------------------------------
// The events of a run, sorted out by kind.
// ---------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Roll {
    event: u32,
    round: u32,
    force: u32,
    base: Fixed,
    radar_bonus: Fixed,
    alert_factor: Fixed,
    chance: Fixed,
    sighted: bool,
}

#[derive(Clone, Debug)]
struct SalvoRec {
    round: u32,
    firer: u32,
    weapon: Weapon,
}

#[derive(Clone, Debug)]
struct Fight {
    day: u32,
    pulse: Pulse,
    begun: u32,
    a: u32,
    b: u32,
    ended: u32,
    ended_round: u32,
    reason: EngagementEndReason,
    /// Elapsed hours since the scenario began, from `EngagementBegun`.
    elapsed_hours: Fixed,
    rolls: Vec<Roll>,
    salvos: Vec<SalvoRec>,
    /// (round, ship) for each ship that sank in this fight.
    sunk: Vec<(u32, u32)>,
    /// The forces that broke off, and the forces that slipped away.
    broke_off: Vec<u32>,
    slipped: Vec<u32>,
}

impl Fight {
    fn night(&self) -> bool {
        self.day == 1 && self.pulse == Pulse::Night
    }

    fn round_one(&self, force: u32) -> &Roll {
        self.rolls
            .iter()
            .find(|r| r.round == 1 && r.force == force)
            .unwrap_or_else(|| panic!("force {force} rolls in round 1 of fight #{}", self.begun))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AlertKind {
    Contact(u32),
    Became,
    Alarmed(u32, u32),
    StoodDown,
}

/// One `AlarmRoll` event, whether or not it succeeded.
#[derive(Clone, Debug)]
struct AlarmRollRec {
    event: u32,
    day: u32,
    pulse: Option<Pulse>,
    force: u32,
    by: (u32, u32),
    chance: Fixed,
    alarmed: bool,
    causes: Vec<u32>,
}

#[derive(Clone, Debug)]
struct AlertRec {
    event: u32,
    day: u32,
    pulse: Option<Pulse>,
    force: u32,
    kind: AlertKind,
    causes: Vec<u32>,
}

#[derive(Clone, Debug)]
struct StepRec {
    pulse: Pulse,
    task_force: u32,
    from: Hex,
    to: Hex,
    distance: Fixed,
    speed: Fixed,
}

#[derive(Clone, Debug)]
struct BurnRec {
    pulse: Pulse,
    ship: u32,
    task_force: u32,
    burned: Fixed,
    remaining: Fixed,
}

#[derive(Clone, Debug)]
struct SystemHitRec {
    event: u32,
    pulse: Pulse,
    ship: u32,
    system: ShipSystem,
    after: SystemState,
}

#[derive(Clone, Debug)]
struct Report {
    event: u32,
    side: Side,
    contact: u32,
}

#[derive(Clone, Debug)]
struct Parsed {
    seed: u64,
    fights: Vec<Fight>,
    alerts: Vec<AlertRec>,
    alarm_rolls: Vec<AlarmRollRec>,
    steps: Vec<StepRec>,
    burns: Vec<BurnRec>,
    arrivals: Vec<(u32, Hex, Fixed)>,
    system_hits: Vec<SystemHitRec>,
    /// Each sunk ship: (fight index, round, pulse).
    sunk: BTreeMap<u32, (usize, u32, Pulse)>,
    day_zero_reports: Vec<Report>,
}

fn event_causes(event: &Event) -> Vec<u32> {
    event
        .causes
        .iter()
        .filter_map(|cause| match cause {
            Cause::Event(id) => Some(id.raw()),
            Cause::Order(_) => None,
        })
        .collect()
}

fn factor(event: &Event, name: &str) -> Fixed {
    event
        .breakdown
        .as_ref()
        .expect("a breakdown is recorded")
        .factors
        .iter()
        .find(|f| f.name == name)
        .unwrap_or_else(|| panic!("event #{} has no factor {name}", event.id.raw()))
        .value
}

fn parse(seed: u64, events: &[Event]) -> Parsed {
    let mut p = Parsed {
        seed,
        fights: Vec::new(),
        alerts: Vec::new(),
        alarm_rolls: Vec::new(),
        steps: Vec::new(),
        burns: Vec::new(),
        arrivals: Vec::new(),
        system_hits: Vec::new(),
        sunk: BTreeMap::new(),
        day_zero_reports: Vec::new(),
    };
    let mut open = false;
    for (index, event) in events.iter().enumerate() {
        let id = event.id.raw();
        assert_eq!(usize::try_from(id).expect("small"), index + 1);
        let pulse = event.pulse;
        match &event.kind {
            EventKind::EngagementBegun {
                a,
                b,
                elapsed_hours,
                ..
            } => {
                assert!(!open, "seed {seed}: a fight inside a fight");
                open = true;
                p.fights.push(Fight {
                    day: event.day,
                    pulse: pulse.expect("a fight has a pulse"),
                    begun: id,
                    a: a.raw(),
                    b: b.raw(),
                    ended: 0,
                    ended_round: 0,
                    reason: EngagementEndReason::MaxRounds,
                    elapsed_hours: *elapsed_hours,
                    rolls: Vec::new(),
                    salvos: Vec::new(),
                    sunk: Vec::new(),
                    broke_off: Vec::new(),
                    slipped: Vec::new(),
                });
            }
            EventKind::ForceBrokeOff { task_force, .. } => {
                assert!(open, "seed {seed}: a break-off outside a fight");
                p.fights
                    .last_mut()
                    .expect("a fight")
                    .broke_off
                    .push(task_force.raw());
            }
            EventKind::ForceSlippedAway { task_force, .. } => {
                assert!(open, "seed {seed}: a slip outside a fight");
                p.fights
                    .last_mut()
                    .expect("a fight")
                    .slipped
                    .push(task_force.raw());
            }
            EventKind::EngagementEnded { round, reason, .. } => {
                assert!(open, "seed {seed}: a fight ended that never began");
                open = false;
                let fight = p.fights.last_mut().expect("a fight");
                fight.ended = id;
                fight.reason = *reason;
                fight.ended_round = *round;
            }
            EventKind::SightingRoll {
                round,
                task_force,
                chance,
                sighted,
                ..
            } => {
                assert!(open, "seed {seed}: a roll outside a fight");
                assert_eq!(
                    factor(event, "alert_factor")
                        * (factor(event, "base") + factor(event, "radar_bonus")),
                    *chance,
                    "seed {seed}"
                );
                let roll = Roll {
                    event: id,
                    round: *round,
                    force: task_force.raw(),
                    base: factor(event, "base"),
                    radar_bonus: factor(event, "radar_bonus"),
                    alert_factor: factor(event, "alert_factor"),
                    chance: *chance,
                    sighted: *sighted,
                };
                p.fights.last_mut().expect("a fight").rolls.push(roll);
            }
            EventKind::Salvo {
                round,
                firer,
                weapon,
                ..
            } => {
                assert!(open, "seed {seed}: a salvo outside a fight");
                p.fights.last_mut().expect("a fight").salvos.push(SalvoRec {
                    round: *round,
                    firer: firer.raw(),
                    weapon: *weapon,
                });
            }
            EventKind::ShipSunk { round, ship, .. } => {
                assert!(open, "seed {seed}: a sinking outside a fight");
                let fights = p.fights.len();
                p.fights
                    .last_mut()
                    .expect("a fight")
                    .sunk
                    .push((*round, ship.raw()));
                p.sunk.insert(
                    ship.raw(),
                    (fights - 1, *round, pulse.expect("a sinking has a pulse")),
                );
            }
            EventKind::ForceAlertedByContact {
                task_force,
                contact,
            } => p.alerts.push(AlertRec {
                event: id,
                day: event.day,
                pulse,
                force: task_force.raw(),
                kind: AlertKind::Contact(contact.raw()),
                causes: event_causes(event),
            }),
            EventKind::ForceBecameAlert { task_force, .. } => p.alerts.push(AlertRec {
                event: id,
                day: event.day,
                pulse,
                force: task_force.raw(),
                kind: AlertKind::Became,
                causes: event_causes(event),
            }),
            EventKind::AlarmRoll {
                task_force,
                by_a,
                by_b,
                chance,
                alarmed,
            } => {
                p.alarm_rolls.push(AlarmRollRec {
                    event: id,
                    day: event.day,
                    pulse,
                    force: task_force.raw(),
                    by: (by_a.raw(), by_b.raw()),
                    chance: *chance,
                    alarmed: *alarmed,
                    causes: event_causes(event),
                });
                // Only a successful roll alerts the force.
                if *alarmed {
                    p.alerts.push(AlertRec {
                        event: id,
                        day: event.day,
                        pulse,
                        force: task_force.raw(),
                        kind: AlertKind::Alarmed(by_a.raw(), by_b.raw()),
                        causes: event_causes(event),
                    });
                }
            }
            EventKind::ForceStoodDown { task_force } => p.alerts.push(AlertRec {
                event: id,
                day: event.day,
                pulse,
                force: task_force.raw(),
                kind: AlertKind::StoodDown,
                causes: event_causes(event),
            }),
            EventKind::TaskForceSteamed {
                task_force,
                from,
                to,
                distance_nm,
                speed_kn,
                ..
            } => {
                assert!(!open, "seed {seed}: movement inside a fight");
                p.steps.push(StepRec {
                    pulse: pulse.expect("a step has a pulse"),
                    task_force: task_force.raw(),
                    from: *from,
                    to: *to,
                    distance: *distance_nm,
                    speed: *speed_kn,
                });
            }
            EventKind::FuelBurned {
                ship,
                task_force,
                burned_t,
                remaining_t,
            } => p.burns.push(BurnRec {
                pulse: pulse.expect("a burn has a pulse"),
                ship: ship.raw(),
                task_force: task_force.raw(),
                burned: *burned_t,
                remaining: *remaining_t,
            }),
            EventKind::TaskForceArrived {
                task_force,
                hex,
                elapsed_hours,
            } => p.arrivals.push((task_force.raw(), *hex, *elapsed_hours)),
            EventKind::SystemHit {
                ship,
                system,
                after,
                ..
            } => p.system_hits.push(SystemHitRec {
                event: id,
                pulse: pulse.expect("a system hit has a pulse"),
                ship: ship.raw(),
                system: *system,
                after: *after,
            }),
            EventKind::ReportReceived { side, contact, .. } if event.day == 0 => {
                p.day_zero_reports.push(Report {
                    event: id,
                    side: *side,
                    contact: contact.raw(),
                });
            }
            _ => {}
        }
    }
    assert!(!open, "seed {seed}: the last fight never ended");
    p
}

impl Parsed {
    fn night_fights(&self) -> Vec<&Fight> {
        self.fights.iter().filter(|f| f.night()).collect()
    }

    /// The first fight of the night: A against B.
    fn first(&self) -> &Fight {
        self.night_fights()[0]
    }

    /// The second fight of the night, A against C, if A had a ship afloat.
    fn second(&self) -> Option<&Fight> {
        self.night_fights().get(1).copied()
    }

    /// Alarm rolls for one force, successful or not, in the night pulse of
    /// day 1.
    fn night_rolls(&self, force: u32) -> Vec<&AlarmRollRec> {
        self.alarm_rolls
            .iter()
            .filter(|r| r.force == force && r.day == 1 && r.pulse == Some(Pulse::Night))
            .collect()
    }

    /// Alarm rolls for one force in any pulse.
    fn rolls(&self, force: u32) -> Vec<&AlarmRollRec> {
        self.alarm_rolls
            .iter()
            .filter(|r| r.force == force)
            .collect()
    }

    /// Successful alarm events for one force, in the night pulse of day 1.
    fn night_alarms(&self, force: u32) -> Vec<&AlertRec> {
        self.alerts
            .iter()
            .filter(|a| {
                a.force == force
                    && a.day == 1
                    && a.pulse == Some(Pulse::Night)
                    && matches!(a.kind, AlertKind::Alarmed(..))
            })
            .collect()
    }

    /// Successful alarm events for one force in any pulse.
    fn alarms(&self, force: u32) -> Vec<&AlertRec> {
        self.alerts
            .iter()
            .filter(|a| a.force == force && matches!(a.kind, AlertKind::Alarmed(..)))
            .collect()
    }

    /// Whether an alert event for the force came before `event` in the same
    /// pulse.
    fn alerted_before(&self, force: u32, event: u32, day: u32, pulse: Pulse) -> bool {
        self.alerts.iter().any(|a| {
            a.force == force
                && a.event < event
                && a.day == day
                && a.pulse == Some(pulse)
                && a.kind != AlertKind::StoodDown
        })
    }

    /// Ships of force A afloat at the end of a pulse of day 1.
    fn a_afloat(&self, upto: Pulse) -> Vec<u32> {
        A_SHIPS
            .iter()
            .copied()
            .filter(|ship| {
                self.sunk
                    .get(ship)
                    .is_none_or(|(_, _, pulse)| pulse_index(*pulse) > pulse_index(upto))
            })
            .collect()
    }

    /// The propulsion of A's slowest ship afloat at the end of a pulse.
    fn a_slowest(&self, upto: Pulse) -> Option<SystemState> {
        self.a_afloat(upto)
            .iter()
            .map(|ship| {
                self.system_hits
                    .iter()
                    .rfind(|h| {
                        h.ship == *ship
                            && h.system == ShipSystem::Propulsion
                            && pulse_index(h.pulse) <= pulse_index(upto)
                    })
                    .map_or(SystemState::Working, |h| h.after)
            })
            .max()
    }

    /// The propulsion of A's slowest ship afloat between the two night
    /// fights: after the fight with B and before the fight with C. The same
    /// as after the night when A has no second night fight.
    fn a_slowest_between_night_fights(&self) -> Option<SystemState> {
        let Some(second) = self.second() else {
            return self.a_slowest(Pulse::Night);
        };
        A_SHIPS
            .iter()
            .filter(|ship| self.sunk.get(ship).is_none_or(|(index, ..)| *index > 0))
            .map(|ship| {
                self.system_hits
                    .iter()
                    .rfind(|h| {
                        h.ship == *ship
                            && h.system == ShipSystem::Propulsion
                            && h.event < second.begun
                    })
                    .map_or(SystemState::Working, |h| h.after)
            })
            .max()
    }

    fn a_arrival(&self) -> Option<(Hex, Fixed)> {
        let arrivals: Vec<_> = self.arrivals.iter().filter(|a| a.0 == 1).collect();
        assert!(arrivals.len() <= 1, "seed {}: A arrives twice", self.seed);
        arrivals.first().map(|a| (a.1, a.2))
    }
}

/// How a fight opens, from the sighting rolls alone (as in scenario 4a).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Opening {
    AFreeRound,
    BFreeRound,
    BothFire,
    PassUnseen,
}

fn opening(fight: &Fight) -> Opening {
    let Some(first) = fight
        .rolls
        .iter()
        .filter(|r| r.sighted)
        .map(|r| r.round)
        .min()
    else {
        return Opening::PassUnseen;
    };
    let sighted = |force: u32| {
        fight
            .rolls
            .iter()
            .any(|r| r.round == first && r.force == force && r.sighted)
    };
    match (sighted(fight.a), sighted(fight.b)) {
        (true, false) => Opening::AFreeRound,
        (false, true) => Opening::BFreeRound,
        (true, true) => Opening::BothFire,
        (false, false) => unreachable!("a round with a success"),
    }
}

/// Whether k out of n is within four standard deviations of probability
/// `p` (in millionths), in exact integer arithmetic.
fn within_four_sigma(k: i64, n: i64, p: i64) -> bool {
    let (k, n, p) = (i128::from(k), i128::from(n), i128::from(p));
    let scale = 1_000_000;
    let off = k * scale - n * p;
    off * off <= 16 * n * p * (scale - p)
}

// ---------------------------------------------------------------------
// Variants: the scenario with edits to its text.
// ---------------------------------------------------------------------

/// Loads the scenario on the frozen figures with each exact, asserted-unique
/// text replacement made in its text. Nothing in the repository's data folder
/// is touched. Each edit is (file under data/, original text, changed text);
/// only the scenario file may be named.
fn variant(_tag: &str, edits: &[(&str, &str, &str)]) -> Scenario {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../data")
        .join(SCENARIO_FILE);
    let mut text = fs::read_to_string(&path).expect("read the scenario");
    for (target, original, changed) in edits {
        assert_eq!(*target, SCENARIO_FILE, "unknown file {target}");
        text = common::replace_once(&text, original, changed);
    }
    common::from_text(&text, &path).expect("the variant loads")
}

/// The anchor in the scenario file before which a contact block goes.
const CONTACT_ANCHOR: &str = "# Day 1, Allied: hold position, and fight.";

fn contact_block(q: i32, r: i32, age: u32) -> String {
    format!(
        "[[contact]]\nside = \"allied\"\nq = {q}\nr = {r}\nship_count = 3\nkind = \"cruiser\"\nage_pulses = {age}\n\n{CONTACT_ANCHOR}"
    )
}

// ---------------------------------------------------------------------
// 1. The speed order.
// ---------------------------------------------------------------------

/// Where force A enters (7,0): 20 nm short of its centre, after 260 nm
/// (decision 0022).
const ENTRY_NM: i64 = 260;

/// The night pulse's movement and burn of force A, asserted against the
/// hand calculation. Used for every run. Force A steams 260 nm at 24 kn to
/// enter (7,0), burning 260 x 0.5625 = 146.25 t a ship, and fights there.
/// What it does for the rest of the night depends on its slowest ship afloat
/// after the fights: 28 nm at 24 kn, 28 x 16 / 24 = 18.666666 nm at 16 kn, or
/// nothing.
fn assert_night_steaming(p: &Parsed, final_positions: &BTreeMap<u32, Hex>) {
    let seed = p.seed;
    let night: Vec<&StepRec> = p.steps.iter().filter(|s| s.pulse == Pulse::Night).collect();
    assert!(
        night.iter().all(|s| s.task_force == 1),
        "seed {seed}: only force A moves"
    );
    // 24 kn for 260 nm: it enters (7,0) 20 nm short of its centre, and its
    // position is (7,0).
    assert_eq!(night[0].from, Hex::new(0, 0), "seed {seed}");
    assert_eq!(night[0].to, MEETING, "seed {seed}");
    assert_eq!(night[0].distance, fx(ENTRY_NM), "seed {seed}");
    assert_eq!(night[0].speed, fx(24), "seed {seed}");
    assert_eq!(7 * HEX_NM - ENTRY_NM, 20);
    // The rest of the night, by the slowest ship afloat after the fights.
    //
    // The distance is scaled at each fight by new speed over old, so it is
    // exact only if the slowest ship's state did not change between the two
    // night fights (the hand calculation's condition). If it did, only the
    // speed is checked.
    let after = p.a_slowest(Pulse::Night);
    let same_between = p.a_slowest_between_night_fights() == after;
    let rest = match after {
        Some(SystemState::Working) => Some((fx(28), fx(24))),
        Some(SystemState::Degraded) => Some((millionths(18_666_666), fx(16))),
        Some(SystemState::Out) | None => None,
    };
    match rest {
        Some((distance, speed)) => {
            assert_eq!(night.len(), 2, "seed {seed}: the rest of the night");
            assert_eq!(night[1].from, MEETING, "seed {seed}");
            assert_eq!(night[1].to, MEETING, "seed {seed}");
            if same_between {
                assert_eq!(night[1].distance, distance, "seed {seed}");
            }
            assert_eq!(night[1].speed, speed, "seed {seed}");
        }
        None => assert_eq!(night.len(), 1, "seed {seed}: no steaming after the fights"),
    }
    // 146.25 t burned and 1853.75 t left, for each cruiser of A, at the
    // meeting.
    let after_night = p.a_afloat(Pulse::Night);
    for ship in A_SHIPS {
        let burns: Vec<&BurnRec> = p
            .burns
            .iter()
            .filter(|b| b.ship == ship && b.pulse == Pulse::Night)
            .collect();
        assert_eq!(burns[0].task_force, 1, "seed {seed}");
        assert_eq!(burns[0].burned, millionths(146_250_000), "seed {seed}");
        assert_eq!(
            burns[0].remaining,
            millionths(1_853_750_000),
            "seed {seed} ship {ship}"
        );
        // A ship afloat after the fights burns for the rest of the night
        // too: 15.75 t at 24 kn, 4.666666 t at 16 kn.
        match (rest, after_night.contains(&ship)) {
            (Some(_), true) if !same_between => {
                assert_eq!(burns.len(), 2, "seed {seed} ship {ship}");
            }
            (Some((_, speed)), true) => {
                assert_eq!(burns.len(), 2, "seed {seed} ship {ship}");
                let (burned, left) = if speed == fx(24) {
                    (millionths(15_750_000), fx(1838))
                } else {
                    (millionths(4_666_666), millionths(1_849_083_334))
                };
                assert_eq!(burns[1].burned, burned, "seed {seed} ship {ship}");
                assert_eq!(burns[1].remaining, left, "seed {seed} ship {ship}");
            }
            _ => assert_eq!(burns.len(), 1, "seed {seed} ship {ship}"),
        }
    }
    // B and C never move and burn nothing.
    assert!(p.steps.iter().all(|s| s.task_force == 1), "seed {seed}");
    assert!(p.burns.iter().all(|b| b.task_force == 1), "seed {seed}");
    assert_eq!(final_positions[&2], MEETING, "seed {seed}");
    assert_eq!(final_positions[&3], MEETING, "seed {seed}");
}

#[test]
fn force_a_enters_the_hex_after_260_nm_at_24_kn_and_burns_146_25_t_each() {
    let default = load();
    let default_run = run_scenario(&default, FILE_SEED);
    let parsed = parse(FILE_SEED, &default_run.events);
    assert_night_steaming(&parsed, &default_run.positions);
    for (run, p) in batch() {
        assert_night_steaming(p, &run.positions);
    }
}

// ---------------------------------------------------------------------
// 2, 3 and 5. Who is alert when the fights begin, and the order of pairs.
// ---------------------------------------------------------------------

#[test]
fn a_is_alert_and_b_is_not_when_the_first_fight_begins() {
    for (_, p) in batch() {
        let seed = p.seed;
        let first = p.first();
        let a = first.round_one(1);
        assert_eq!(
            (a.base, a.radar_bonus, a.alert_factor, a.chance),
            (HALF, Fixed::ZERO, Fixed::ONE, HALF),
            "seed {seed}: A's round 1"
        );
        let b = first.round_one(2);
        assert_eq!(
            (b.base, b.radar_bonus, b.alert_factor, b.chance),
            (HALF, Fixed::ZERO, HALF, QUARTER),
            "seed {seed}: B's round 1"
        );
        if let Some(second) = p.second() {
            let a = second.round_one(1);
            assert_eq!(
                (a.alert_factor, a.chance),
                (Fixed::ONE, HALF),
                "seed {seed}: A's round 1 against C"
            );
        }
    }
}

#[test]
fn the_night_pairs_are_a_against_b_then_a_against_c_unless_a_has_no_ship_afloat() {
    let mut skipped = 0;
    for (run, p) in batch() {
        let seed = p.seed;
        let pairs: Vec<(u32, u32)> = p.night_fights().iter().map(|f| (f.a, f.b)).collect();
        // The first EngagementBegun of the night is (1,2).
        let first_begun = run
            .events
            .iter()
            .find(|e| matches!(e.kind, EventKind::EngagementBegun { .. }))
            .expect("a fight");
        match &first_begun.kind {
            EventKind::EngagementBegun { a, b, .. } => {
                assert_eq!((a.raw(), b.raw()), (1, 2), "seed {seed}");
            }
            _ => unreachable!(),
        }
        assert_eq!(first_begun.pulse, Some(Pulse::Night), "seed {seed}");
        // A fight with C is skipped only if A has no ship afloat when the
        // first fight is over.
        let afloat_after_first = A_SHIPS
            .iter()
            .filter(|ship| p.sunk.get(ship).is_none_or(|(index, ..)| *index > 0))
            .count();
        // A ship sunk later (index > 0) is still afloat after the first fight.
        if afloat_after_first == 0 {
            skipped += 1;
            assert_eq!(pairs, vec![(1, 2)], "seed {seed}");
        } else {
            assert_eq!(pairs, vec![(1, 2), (1, 3)], "seed {seed}");
        }
        // Fights in the night come in pair order, whichever way they end.
        assert!(
            p.fights
                .windows(2)
                .all(|w| w[0].begun < w[1].begun && w[0].ended < w[1].begun),
            "seed {seed}"
        );
    }
    eprintln!("runs with no second night fight because A had no ship afloat: {skipped}");
}

#[test]
fn c_sights_at_half_if_alarmed_before_the_second_fight_and_at_a_quarter_if_not() {
    let (mut with_alarm, mut without) = (0, 0);
    for (_, p) in batch() {
        let seed = p.seed;
        let Some(second) = p.second() else { continue };
        let roll = second.round_one(3);
        let alarmed = p
            .night_alarms(3)
            .iter()
            .any(|alarm| alarm.event < roll.event);
        let expected = if alarmed { HALF } else { QUARTER };
        let factor = if alarmed { Fixed::ONE } else { HALF };
        assert_eq!(roll.chance, expected, "seed {seed}: C's round 1");
        assert_eq!(roll.alert_factor, factor, "seed {seed}: C's alert factor");
        if alarmed {
            with_alarm += 1;
        } else {
            without += 1;
        }
    }
    eprintln!("second fights where C was alarmed {with_alarm}, not alarmed {without}");
    assert!(with_alarm > 0 && without > 0);
}

// ---------------------------------------------------------------------
// 4. Over 1,000 runs.
// ---------------------------------------------------------------------

struct Counts {
    a_free: i64,
    unseen: i64,
    alarmed: i64,
    /// (n, k) where C was alarmed and the second fight took place, and C
    /// sighted in round 1 in k of them. Then for not alarmed.
    alarmed_cond: (i64, i64),
    quiet_cond: (i64, i64),
}

fn counts() -> Counts {
    let mut c = Counts {
        a_free: 0,
        unseen: 0,
        alarmed: 0,
        alarmed_cond: (0, 0),
        quiet_cond: (0, 0),
    };
    for (_, p) in batch() {
        let first = p.first();
        let way = opening(first);
        c.a_free += i64::from(way == Opening::AFreeRound);
        c.unseen += i64::from(way == Opening::PassUnseen);
        // C rolls exactly when a salvo was fired in the first fight, once,
        // at 0.5; it is alarmed if that roll succeeded.
        let rolls = p.night_rolls(3);
        let alarmed = rolls.iter().any(|r| r.alarmed);
        c.alarmed += i64::from(alarmed);
        if let Some(second) = p.second() {
            let sighted = i64::from(
                second
                    .rolls
                    .iter()
                    .any(|r| r.round == 1 && r.force == 3 && r.sighted),
            );
            let cell = if alarmed {
                &mut c.alarmed_cond
            } else {
                &mut c.quiet_cond
            };
            cell.0 += 1;
            cell.1 += sighted;
        }
    }
    c
}

#[test]
fn the_counts_fall_where_the_hand_calculation_says() {
    let c = counts();
    assert!(
        (537..=660).contains(&c.a_free),
        "A free round: {}",
        c.a_free
    );
    assert!((0..=9).contains(&c.unseen), "pass unseen: {}", c.unseen);
    assert!((436..=561).contains(&c.alarmed), "C alarmed: {}", c.alarmed);
    // (k - np)^2 <= 16np(1 - p), exactly.
    let (n, k) = c.alarmed_cond;
    assert!(n > 0, "some second fights with C alarmed");
    assert!(
        within_four_sigma(k, n, 500_000),
        "C alarmed, sights in round 1: {k} of {n}"
    );
    let (n, k) = c.quiet_cond;
    assert!(n > 0, "some second fights with C not alarmed");
    assert!(
        within_four_sigma(k, n, 250_000),
        "C not alarmed, sights in round 1: {k} of {n}"
    );
}

#[test]
fn the_alarm_never_rolls_for_a_or_b_and_c_rolls_once_after_a_fight_with_fire() {
    for (_, p) in batch() {
        let seed = p.seed;
        assert!(p.rolls(1).is_empty(), "seed {seed}: an alarm roll for A");
        assert!(p.alarms(2).is_empty(), "seed {seed}: an alarm for B");
        // B was a party to the first fight and is alert if it sighted or was
        // fired on. It is a bystander only to the second fight (A against C),
        // and not alert only if the first passed unseen. Then it rolls once,
        // citing the end of the second fight.
        let b_rolls = p.rolls(2);
        if opening(p.first()) == Opening::PassUnseen {
            if let Some(second) = p.second().filter(|second| !second.salvos.is_empty()) {
                assert_eq!(b_rolls.len(), 1, "seed {seed}: B's roll");
                assert_eq!(b_rolls[0].by, (1, 3), "seed {seed}");
                assert_eq!(b_rolls[0].causes, vec![second.ended], "seed {seed}");
                assert_eq!(b_rolls[0].chance, HALF, "seed {seed}");
            }
        } else {
            assert!(b_rolls.is_empty(), "seed {seed}: B rolled though alert");
        }
        let rolls = p.night_rolls(3);
        assert!(rolls.len() <= 1, "seed {seed}: more than one roll for C");
        // C rolls in no other pulse either.
        assert_eq!(rolls.len(), p.rolls(3).len(), "seed {seed}");
        // Successes are the rolls whose `alarmed` field is true, and only
        // they are alerts.
        let alarms = p.night_alarms(3);
        let successes = rolls.iter().filter(|r| r.alarmed).count();
        assert_eq!(alarms.len(), successes, "seed {seed}");
        assert_eq!(p.alarms(3).len(), successes, "seed {seed}");
        let first = p.first();
        // C rolls exactly when a salvo was fired in the first fight.
        assert_eq!(
            rolls.len(),
            usize::from(!first.salvos.is_empty()),
            "seed {seed}"
        );
        if let Some(roll) = rolls.first() {
            // No roll when A and B passed unseen: at least one salvo was
            // fired in the first fight.
            assert_ne!(opening(first), Opening::PassUnseen, "seed {seed}");
            // The roll is at 0.5, cites the end of that fight and names its
            // forces.
            assert_eq!(roll.chance, HALF, "seed {seed}");
            assert_eq!(roll.causes, vec![first.ended], "seed {seed}");
            assert_eq!(roll.by, (1, 2), "seed {seed}");
            assert!(roll.event > first.ended, "seed {seed}");
            if let Some(second) = p.second() {
                assert!(roll.event < second.begun, "seed {seed}");
            }
            if roll.alarmed {
                assert_eq!(alarms[0].event, roll.event, "seed {seed}");
                assert_eq!(alarms[0].kind, AlertKind::Alarmed(1, 2), "seed {seed}");
                assert_eq!(alarms[0].causes, vec![first.ended], "seed {seed}");
            }
        }
        if opening(first) == Opening::PassUnseen {
            assert!(first.salvos.is_empty(), "seed {seed}");
            assert!(rolls.is_empty(), "seed {seed}");
        }
    }
}

// ---------------------------------------------------------------------
// 6. Torpedoes.
// ---------------------------------------------------------------------

#[test]
fn no_ship_of_a_fires_torpedoes_twice_and_a_sunk_ship_fires_no_more() {
    for (_, p) in batch() {
        let seed = p.seed;
        for ship in A_SHIPS {
            let torpedoes: usize = p
                .fights
                .iter()
                .flat_map(|f| f.salvos.iter())
                .filter(|s| s.firer == ship && s.weapon == Weapon::Torpedo)
                .count();
            assert!(torpedoes <= 1, "seed {seed} ship {ship}: {torpedoes}");
        }
        // A ship that sank fired in the round it sank at the latest.
        for (index, fight) in p.fights.iter().enumerate() {
            for salvo in &fight.salvos {
                if let Some((sunk_fight, sunk_round, _)) = p.sunk.get(&salvo.firer) {
                    assert!(
                        (index, salvo.round) <= (*sunk_fight, *sunk_round),
                        "seed {seed}: ship {} fires after it sank",
                        salvo.firer
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------------
// 6b. When the fights begin.
// ---------------------------------------------------------------------

#[test]
fn the_night_fights_begin_after_260_nm_at_hour_10_833333_and_day_fights_at_the_start_of_their_pulse(
) {
    // A enters (7,0) after 260 nm at 24 kn: 260 / 24 = 10.833333 hours into
    // the night pulse, which starts at hour 0 (truncated to six places). It
    // meets B and C at that moment. A force that is dead in the water lies
    // in the hex at the start of a pulse: the morning pulse begins at hour
    // 12 and the afternoon pulse at hour 18.
    for (_, p) in batch() {
        for fight in &p.fights {
            let hours = match (fight.day, fight.pulse) {
                (1, Pulse::Night) => millionths(10_833_333),
                (1, Pulse::Morning) => fx(12),
                (1, Pulse::Afternoon) => fx(18),
                other => panic!("seed {}: a fight in {other:?}", p.seed),
            };
            assert_eq!(fight.elapsed_hours, hours, "seed {}", p.seed);
        }
    }
    // The default run at the scenario's own seed agrees.
    let run = run_scenario(&load(), FILE_SEED);
    let p = parse(FILE_SEED, &run.events);
    assert_eq!(p.first().elapsed_hours, millionths(10_833_333));
}

// ---------------------------------------------------------------------
// 6c. Breaking off.
// ---------------------------------------------------------------------

/// What the "Breaking off" section says, counted for the summary.
#[derive(Default)]
struct BreakingOff {
    /// Pulses in which A broke off from B and then met C.
    cases: usize,
    slipped: usize,
    broke_off_in_round_1: usize,
    /// Pulses in which A met C without having broken off from B, and had a
    /// free round against C.
    free_rounds_otherwise: usize,
}

/// A free round for force A: a round in which A fires and the other force
/// has not yet sighted it. (A round in which the other force has sighted A
/// and cannot fire, its guns being out, is not a free round.)
fn a_has_a_free_round(fight: &Fight) -> bool {
    (1..=6).any(|round| {
        let a_fired = fight
            .salvos
            .iter()
            .any(|s| s.round == round && A_SHIPS.contains(&s.firer));
        let a_is_sighted = fight
            .rolls
            .iter()
            .any(|r| r.round <= round && r.force != 1 && r.sighted);
        a_fired && !a_is_sighted
    })
}

fn breaking_off() -> BreakingOff {
    let mut out = BreakingOff::default();
    for (_, p) in batch() {
        let seed = p.seed;
        for pulse in [Pulse::Night, Pulse::Morning, Pulse::Afternoon] {
            let in_pulse: Vec<&Fight> = p.fights.iter().filter(|f| f.pulse == pulse).collect();
            let (Some(ab), Some(ac)) = (
                in_pulse.iter().find(|f| (f.a, f.b) == (1, 2)),
                in_pulse.iter().find(|f| (f.a, f.b) == (1, 3)),
            ) else {
                continue;
            };
            if !ab.broke_off.contains(&1) {
                out.free_rounds_otherwise += usize::from(a_has_a_free_round(ac));
                continue;
            }
            out.cases += 1;
            // A avoids C: it never has a free round against it.
            assert!(
                !a_has_a_free_round(ac),
                "seed {seed} {pulse:?}: A has a free round against C"
            );
            // The fight ends with the first round in which A is fired on,
            // if there is one.
            let first_fire_on_a = ac
                .salvos
                .iter()
                .filter(|s| !A_SHIPS.contains(&s.firer))
                .map(|s| s.round)
                .min();
            if let Some(round) = first_fire_on_a {
                assert_eq!(
                    ac.ended_round, round,
                    "seed {seed} {pulse:?}: ends with the first round A is fired on"
                );
            }
            // Either A slips away, with no fire and no break-off, or C's
            // fire sinks its last ship, or the fight ends with A breaking
            // off, fired on or not.
            if ac.reason == EngagementEndReason::SlippedAway {
                out.slipped += 1;
                assert_eq!(ac.slipped, vec![1], "seed {seed}");
                assert!(ac.salvos.is_empty(), "seed {seed}");
                assert!(ac.broke_off.is_empty(), "seed {seed}");
            } else if ac.reason == EngagementEndReason::ForceSunk {
                // C's fire left A with no ship afloat: that ending is
                // checked before the break-off.
                assert!(!ac.salvos.is_empty(), "seed {seed} {pulse:?}");
                assert!(ac.slipped.is_empty(), "seed {seed}");
            } else {
                assert_eq!(
                    ac.reason,
                    EngagementEndReason::BrokeOff,
                    "seed {seed} {pulse:?}"
                );
                assert!(ac.broke_off.contains(&1), "seed {seed}: A breaks off");
                assert!(ac.slipped.is_empty(), "seed {seed}");
                out.broke_off_in_round_1 += usize::from(ac.ended_round == 1);
            }
        }
    }
    out
}

#[test]
fn a_force_that_breaks_off_from_b_avoids_c_for_the_rest_of_the_pulse() {
    let out = breaking_off();
    assert!(
        out.cases > 0,
        "A breaks off from B and meets C in some runs"
    );
    assert!(out.slipped > 0, "some seed has A slip away from C");
    assert!(
        out.broke_off_in_round_1 > 0,
        "some seed has A break off in round 1"
    );
    // Without the rule A would have a free round against C in some of the
    // other runs: it still does where it did not break off from B.
    assert!(
        out.free_rounds_otherwise > 0,
        "A has a free round against C when it has not broken off"
    );
}

// ---------------------------------------------------------------------
// 7. After the night.
// ---------------------------------------------------------------------

/// What the table of "After the night" rows found, for the summary.
#[derive(Default)]
struct AfterNight {
    rows: [usize; 3],
    changed: usize,
    /// Of those, runs where the state changed between the two night fights.
    changed_between: usize,
    none_afloat: usize,
    freed: usize,
}

fn after_the_night() -> AfterNight {
    let mut out = AfterNight::default();
    for (run, p) in batch() {
        let seed = p.seed;
        let night = p.a_slowest(Pulse::Night);
        let morning = p.a_slowest(Pulse::Morning);
        let day = p.a_slowest(Pulse::Afternoon);

        // The events agree with the final state.
        let final_worst = A_SHIPS
            .iter()
            .filter(|ship| run.ships[ship].location != ShipLocation::Sunk)
            .map(|ship| run.ships[ship].propulsion)
            .max();
        assert_eq!(day, final_worst, "seed {seed}: propulsion from the events");

        let Some(state) = night else {
            out.none_afloat += 1;
            continue;
        };
        // The hand calculation holds when the slowest ship's state is the
        // same from the night fights to the end of the day, between the two
        // night fights too.
        if morning == night && day == night && p.a_slowest_between_night_fights() != night {
            out.changed_between += 1;
            continue;
        }
        if morning != night || day != night {
            out.changed += 1;
            out.freed += usize::from(day.is_some() && day != night);
            continue;
        }
        let after: Vec<&StepRec> = p.steps.iter().filter(|s| s.pulse != Pulse::Night).collect();
        let afloat: Vec<u32> = A_SHIPS
            .iter()
            .copied()
            .filter(|ship| run.ships[ship].location != ShipLocation::Sunk)
            .collect();
        let night_steps: Vec<&StepRec> =
            p.steps.iter().filter(|s| s.pulse == Pulse::Night).collect();
        let night_nm = night_steps
            .iter()
            .fold(Fixed::ZERO, |sum, s| sum + s.distance);
        let night_burn = |ship: u32| {
            p.burns
                .iter()
                .filter(|b| b.ship == ship && b.pulse == Pulse::Night)
                .fold(Fixed::ZERO, |sum, b| sum + b.burned)
        };
        let post_night_burn = |ship: u32| {
            p.burns
                .iter()
                .filter(|b| b.ship == ship && b.pulse != Pulse::Night)
                .fold(Fixed::ZERO, |sum, b| sum + b.burned)
        };
        match state {
            SystemState::Working => {
                out.rows[0] += 1;
                // The night: 28 nm to go at 24 kn, 288 nm in all, 162 t.
                assert_eq!(night_nm, fx(288), "seed {seed}: night total");
                assert!(after.iter().all(|s| s.speed == fx(24)), "seed {seed}");
                assert_eq!(
                    after.iter().fold(Fixed::ZERO, |sum, s| sum + s.distance),
                    fx(112),
                    "seed {seed}: 112 nm to go"
                );
                // 112 nm at 24 kn takes 4.666666 hours: arrives at hour
                // 16.666666 at (10,0).
                assert_eq!(
                    p.a_arrival(),
                    Some((DESTINATION, millionths(16_666_666))),
                    "seed {seed}"
                );
                assert_eq!(run.positions[&1], DESTINATION, "seed {seed}");
                for ship in &afloat {
                    assert_eq!(night_burn(*ship), fx(162), "seed {seed} ship {ship}");
                    assert_eq!(post_night_burn(*ship), fx(63), "seed {seed} ship {ship}");
                    assert_eq!(run.ships[ship].fuel, fx(1775), "seed {seed} ship {ship}");
                }
            }
            SystemState::Degraded => {
                out.rows[1] += 1;
                // The night: 18.666666 nm to go at 16 kn, 278.666666 nm in
                // all, 150.916666 t. Then 96 nm in the morning and the last
                // 25.333334 nm in the afternoon: 121.333334 nm, 30.333333 t
                // (24 t and 6.333333 t).
                assert_eq!(night_nm, millionths(278_666_666), "seed {seed}");
                assert!(after.iter().all(|s| s.speed == fx(16)), "seed {seed}");
                assert_eq!(
                    after.iter().fold(Fixed::ZERO, |sum, s| sum + s.distance),
                    millionths(121_333_334),
                    "seed {seed}: 121.333334 nm to go"
                );
                let pieces: Vec<(Pulse, Fixed)> =
                    after.iter().map(|s| (s.pulse, s.distance)).collect();
                assert_eq!(
                    pieces,
                    vec![
                        (Pulse::Morning, fx(96)),
                        (Pulse::Afternoon, millionths(25_333_334))
                    ],
                    "seed {seed}"
                );
                assert_eq!(
                    p.a_arrival(),
                    Some((DESTINATION, millionths(19_583_333))),
                    "seed {seed}"
                );
                assert_eq!(run.positions[&1], DESTINATION, "seed {seed}");
                for ship in &afloat {
                    assert_eq!(
                        night_burn(*ship),
                        millionths(150_916_666),
                        "seed {seed} ship {ship}"
                    );
                    assert_eq!(
                        post_night_burn(*ship),
                        millionths(30_333_333),
                        "seed {seed} ship {ship}"
                    );
                    assert_eq!(
                        run.ships[ship].fuel,
                        millionths(1_818_750_001),
                        "seed {seed} ship {ship}"
                    );
                }
            }
            SystemState::Out => {
                out.rows[2] += 1;
                // The night total is the 260 nm to the meeting, 146.25 t.
                // Speed 0: it does not move, burns nothing and never arrives.
                assert_eq!(night_nm, fx(260), "seed {seed}: night total");
                assert!(after.is_empty(), "seed {seed}: A steams dead in the water");
                assert_eq!(p.a_arrival(), None, "seed {seed}");
                assert_eq!(run.positions[&1], MEETING, "seed {seed}");
                for ship in &afloat {
                    assert_eq!(night_burn(*ship), millionths(146_250_000), "seed {seed}");
                    assert_eq!(post_night_burn(*ship), Fixed::ZERO, "seed {seed}");
                    assert_eq!(
                        run.ships[ship].fuel,
                        millionths(1_853_750_000),
                        "seed {seed} ship {ship}"
                    );
                }
            }
        }
    }
    out
}

#[test]
fn after_the_night_a_steams_on_by_its_slowest_ship() {
    let out = after_the_night();
    assert!(out.rows.iter().all(|n| *n > 0), "every row is met");
}

// ---------------------------------------------------------------------
// 8. Alertness from belief.
// ---------------------------------------------------------------------

/// One row of the "Alertness from belief" table: the contact (q, r, age),
/// the hexes from (7,0), whether B and C are alerted and B's chance.
struct BeliefRow {
    label: &'static str,
    contact: Option<(i32, i32, u32)>,
    alerted: bool,
    b_chance: Fixed,
}

fn belief_rows() -> [BeliefRow; 6] {
    [
        BeliefRow {
            label: "none",
            contact: None,
            alerted: false,
            b_chance: QUARTER,
        },
        BeliefRow {
            label: "(5,0) fresh",
            contact: Some((5, 0, 0)),
            alerted: true,
            b_chance: HALF,
        },
        BeliefRow {
            label: "(4,0) fresh",
            contact: Some((4, 0, 0)),
            alerted: true,
            b_chance: HALF,
        },
        BeliefRow {
            label: "(3,0) fresh",
            contact: Some((3, 0, 0)),
            alerted: false,
            b_chance: QUARTER,
        },
        BeliefRow {
            label: "(5,0) 2 pulses old",
            contact: Some((5, 0, 2)),
            alerted: true,
            b_chance: HALF,
        },
        BeliefRow {
            label: "(5,0) 3 pulses old",
            contact: Some((5, 0, 3)),
            alerted: false,
            b_chance: QUARTER,
        },
    ]
}

/// The scenario with one more starting Allied contact, if any.
fn with_contact(tag: usize, contact: Option<(i32, i32, u32)>) -> Scenario {
    match contact {
        None => load(),
        Some((q, r, age)) => variant(
            &format!("contact_{tag}"),
            &[(SCENARIO_FILE, CONTACT_ANCHOR, &contact_block(q, r, age))],
        ),
    }
}

struct BeliefResult {
    b_chance: Fixed,
    c_chance: Option<Fixed>,
}

fn run_belief_row(index: usize, row: &BeliefRow) -> BeliefResult {
    let scenario = with_contact(index, row.contact);
    // Held at day 0: the contact is held before the first pulse, at its age.
    let view = scenario.world.side_view(Side::Allied);
    let held: Vec<_> = view.contacts().collect();
    match row.contact {
        None => assert!(held.is_empty(), "{}", row.label),
        Some((q, r, age)) => {
            assert_eq!(held.len(), 1, "{}", row.label);
            assert_eq!(held[0].estimated_hex, Hex::new(q, r), "{}", row.label);
            assert_eq!(held[0].age_pulses, age, "{}", row.label);
            assert_eq!(held[0].estimated_ship_count, 3, "{}", row.label);
        }
    }
    assert!(
        scenario
            .world
            .side_view(Side::Japanese)
            .contacts()
            .next()
            .is_none(),
        "{}: the Japanese hold no contact",
        row.label
    );
    let run = run_scenario(&scenario, FILE_SEED);
    let p = parse(FILE_SEED, &run.events);
    assert_night_steaming(&p, &run.positions);
    let first = p.first();
    let b = first.round_one(2);
    let (factor, chance) = if row.alerted {
        (Fixed::ONE, HALF)
    } else {
        (HALF, QUARTER)
    };
    assert_eq!(b.chance, row.b_chance, "{}", row.label);
    assert_eq!(
        (b.alert_factor, b.chance),
        (factor, chance),
        "{}",
        row.label
    );
    assert_eq!(first.round_one(1).chance, HALF, "{}: A", row.label);

    let contact_alerts: Vec<&AlertRec> = p
        .alerts
        .iter()
        .filter(|a| matches!(a.kind, AlertKind::Contact(_)))
        .collect();
    let night_start = |a: &&AlertRec| a.day == 1 && a.pulse == Some(Pulse::Night);
    // At the start of the night nothing is held but the starting contact, if
    // any. Later pulses hold what the fights reported, and every force near
    // it is alerted by it at the start of the pulse, A included.
    let stood_down = p
        .alerts
        .iter()
        .filter(night_start)
        .any(|a| a.kind == AlertKind::StoodDown);
    if row.alerted {
        let report = p
            .day_zero_reports
            .iter()
            .find(|r| r.side == Side::Allied)
            .expect("a report on day 0 for the starting contact");
        assert_eq!(p.day_zero_reports.len(), 1, "{}", row.label);
        // B and C are each alerted at the start of the night, citing the
        // day-0 report behind the contact; A is not (it holds no contact).
        for force in [2, 3] {
            let mine: Vec<&&AlertRec> = contact_alerts
                .iter()
                .filter(|a| a.force == force && night_start(a))
                .collect();
            assert_eq!(mine.len(), 1, "{}: force {force}", row.label);
            assert_eq!(
                mine[0].kind,
                AlertKind::Contact(report.contact),
                "{}",
                row.label
            );
            assert_eq!(mine[0].causes, vec![report.event], "{}", row.label);
            // Alerted at the start of the pulse: before any fight begins.
            assert!(mine[0].event < first.begun, "{}", row.label);
        }
        assert!(
            contact_alerts
                .iter()
                .all(|a| a.force != 1 || !night_start(a)),
            "{}: A alerted by a contact",
            row.label
        );
        // C needs no alarm and gets no alarm roll. Nor does B, or A.
        for force in [1, 2, 3] {
            assert!(p.rolls(force).is_empty(), "{}: an alarm roll", row.label);
        }
    } else {
        assert!(!contact_alerts.iter().any(night_start), "{}", row.label);
        assert!(!stood_down, "{}", row.label);
    }
    BeliefResult {
        b_chance: b.chance,
        c_chance: p.second().map(|second| {
            let c = second.round_one(3);
            if row.alerted {
                assert_eq!(c.chance, HALF, "{}: C", row.label);
            } else {
                let alarmed = p.night_alarms(3).iter().any(|a| a.event < c.event);
                assert_eq!(
                    c.chance,
                    if alarmed { HALF } else { QUARTER },
                    "{}",
                    row.label
                );
            }
            c.chance
        }),
    }
}

#[test]
fn a_contact_held_nearby_and_recent_alerts_b_and_c() {
    let rows = belief_rows();
    let results: Vec<BeliefResult> = rows
        .iter()
        .enumerate()
        .map(|(index, row)| run_belief_row(index, row))
        .collect();
    // Being alerted never lowers a force's sighting chance: compare each row
    // with the row with no contact.
    for (row, result) in rows.iter().zip(&results) {
        assert!(result.b_chance >= results[0].b_chance, "{}", row.label);
        if row.alerted {
            assert!(result.b_chance > results[0].b_chance, "{}", row.label);
        } else {
            assert_eq!(result.b_chance, results[0].b_chance, "{}", row.label);
        }
        eprintln!(
            "belief row {:<20} B's chance {} C's chance in the second fight {}",
            row.label,
            result.b_chance,
            result
                .c_chance
                .map_or("none".to_string(), |c| c.to_string())
        );
    }
}

// ---------------------------------------------------------------------
// 9. Two further single runs.
// ---------------------------------------------------------------------

#[test]
fn a_at_normal_readiness_sights_at_a_quarter_like_b() {
    let scenario = variant(
        "normal",
        &[(
            SCENARIO_FILE,
            "readiness = \"battle\"",
            "readiness = \"normal\"",
        )],
    );
    let run = run_scenario(&scenario, FILE_SEED);
    let p = parse(FILE_SEED, &run.events);
    let first = p.first();
    let a = first.round_one(1);
    let b = first.round_one(2);
    assert_eq!((a.alert_factor, a.chance), (HALF, QUARTER));
    assert_eq!((b.alert_factor, b.chance), (HALF, QUARTER));
    assert_eq!(a.chance, b.chance);
    // And lower than at battle readiness.
    let battle = run_scenario(&load(), FILE_SEED);
    let bp = parse(FILE_SEED, &battle.events);
    assert!(a.chance < bp.first().round_one(1).chance);
}

#[test]
fn a_ordered_to_40_kn_steams_at_32_kn_and_fights_both_forces_in_passing() {
    let scenario = variant(
        "forty",
        &[(SCENARIO_FILE, "knots = \"24\"", "knots = \"40\"")],
    );
    // The hand calculation is for the force that comes through with its
    // propulsion working. With each shot rolled (decision 0033) the scenario's
    // own seed no longer gives that, so the test takes the first seed from
    // the file's that does.
    let (seed, run) = (FILE_SEED..FILE_SEED + 100)
        .map(|seed| (seed, run_scenario(&scenario, seed)))
        .find(|(_, run)| {
            A_SHIPS.iter().all(|ship| {
                run.ships[ship].propulsion == SystemState::Working
                    && run.ships[ship].location != ShipLocation::Sunk
            })
        })
        .expect("a seed in which A comes through undamaged");
    eprintln!("40 kn: seed {seed}");
    let p = parse(seed, &run.events);
    // 32 kn, the top speed. It enters (7,0) after 260 nm, 260 / 32 = 8.125
    // hours into the night pulse, and meets and fights both forces there,
    // one after the other.
    let hours = millionths(8_125_000);
    assert_eq!(p.fights.len(), 2, "two fights, both at the meeting");
    for (fight, enemy) in p.fights.iter().zip([2, 3]) {
        assert_eq!((fight.a, fight.b), (1, enemy));
        assert_eq!((fight.day, fight.pulse), (1, Pulse::Night));
        assert_eq!(fight.elapsed_hours, hours);
    }
    // Burn per mile is multiplied by 4: 1 t a nm, so 260 t at the meeting.
    // At this seed no ship of A is sunk or loses propulsion (the seed was
    // chosen for it), so it comes
    // through at 32 kn with 384 - 260 = 124 nm of the night left, ends the
    // night nine hexes and 24 nm along, with its position (10,0).
    for ship in A_SHIPS {
        assert_eq!(run.ships[&ship].propulsion, SystemState::Working);
        assert_ne!(run.ships[&ship].location, ShipLocation::Sunk);
    }
    let night: Vec<&StepRec> = p.steps.iter().filter(|s| s.pulse == Pulse::Night).collect();
    assert_eq!(night.len(), 2);
    assert!(night.iter().all(|s| s.task_force == 1 && s.speed == fx(32)));
    assert_eq!(
        (night[0].from, night[0].to, night[0].distance),
        (Hex::new(0, 0), MEETING, fx(260))
    );
    assert_eq!(
        (night[1].from, night[1].to, night[1].distance),
        (MEETING, DESTINATION, fx(124))
    );
    assert!(p
        .steps
        .iter()
        .all(|s| s.task_force == 1 && s.speed == fx(32)));
    // The 16 nm that remain take half an hour: it arrives at hour 12.5.
    assert_eq!(p.a_arrival(), Some((DESTINATION, millionths(12_500_000))));
    assert_eq!(run.positions[&1], DESTINATION);
    // 1 t a nm: 260 t to the meeting, 124 t for the rest of the night, 16 t
    // after, 400 t in all, 1600 t left.
    for ship in A_SHIPS {
        let night_burn: Vec<&BurnRec> = p
            .burns
            .iter()
            .filter(|b| b.ship == ship && b.pulse == Pulse::Night)
            .collect();
        assert_eq!(night_burn.len(), 2);
        assert_eq!(night_burn[0].burned, fx(260));
        assert_eq!(night_burn[1].burned, fx(124));
        let total = p
            .burns
            .iter()
            .filter(|b| b.ship == ship)
            .fold(Fixed::ZERO, |sum, b| sum + b.burned);
        assert_eq!(total, fx(400), "ship {ship}");
        assert_eq!(run.ships[&ship].fuel, fx(1600), "ship {ship}");
    }
}

// ---------------------------------------------------------------------
// 10. Properties.
// ---------------------------------------------------------------------

#[test]
fn a_force_at_battle_readiness_is_always_alert() {
    let mut rolls = 0;
    for (_, p) in batch() {
        for fight in &p.fights {
            for roll in fight.rolls.iter().filter(|r| r.force == 1) {
                rolls += 1;
                assert_eq!(roll.alert_factor, Fixed::ONE, "seed {}", p.seed);
            }
        }
    }
    eprintln!("sighting rolls of A checked: {rolls}");
    assert!(rolls > 1000);
}

#[test]
fn a_normal_force_is_alert_only_after_an_event_that_pulse() {
    let (mut alert_rolls, mut quiet_rolls) = (0, 0);
    for (_, p) in batch() {
        for fight in p.night_fights() {
            for roll in fight.rolls.iter().filter(|r| r.force != 1) {
                // Nobody holds a contact at the start of the night, so an
                // alert comes only from an event earlier in the pulse:
                // sighting or being fired on (in an earlier round) or the
                // alarm.
                let alerted = p.alerted_before(roll.force, roll.event, 1, Pulse::Night);
                let expected = if alerted { Fixed::ONE } else { HALF };
                assert_eq!(
                    roll.alert_factor, expected,
                    "seed {} force {} round {}",
                    p.seed, roll.force, roll.round
                );
                if alerted {
                    alert_rolls += 1;
                } else {
                    quiet_rolls += 1;
                }
                // Alert or not, the chance is the base times the factor.
                assert_eq!(roll.chance, roll.base * expected);
            }
        }
        // An alert event for a force in the night is always for B or C and
        // never for a force at battle readiness.
        for alert in p.alerts.iter().filter(|a| a.pulse == Some(Pulse::Night)) {
            assert!(alert.force != 1, "seed {}", p.seed);
            assert!(
                !matches!(alert.kind, AlertKind::Contact(_) | AlertKind::StoodDown),
                "seed {}: no belief at the start of the night",
                p.seed
            );
        }
    }
    eprintln!("normal-readiness rolls: {quiet_rolls} not alert, {alert_rolls} alerted earlier");
    assert!(quiet_rolls > 0);
}

#[test]
fn scenario_4a_keeps_its_numbers() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/04a_night_action.toml");
    let mut scenario = common::load(&path).expect("scenario 4a loads");
    scenario.run_to_end().expect("the run resolves");
    let p = parse(
        0,
        &scenario.world.events().iter().cloned().collect::<Vec<_>>(),
    );
    let first = p.first();
    let (a, b) = (first.round_one(1), first.round_one(2));
    assert_eq!((a.alert_factor, a.chance), (Fixed::ONE, HALF));
    assert_eq!((b.alert_factor, b.chance), (HALF, QUARTER));
}

#[test]
fn a_replay_reproduces_the_daily_hash() {
    let base = load();
    let mut all: Vec<Vec<u64>> = Vec::new();
    for seed in [1u64, 2, 77, 537, 1942] {
        let runs: Vec<Vec<u64>> = (0..2).map(|_| run_scenario(&base, seed).hashes).collect();
        assert_eq!(runs[0].len(), 1);
        assert_eq!(runs[0], runs[1], "seed {seed}");
        all.push(runs[0].clone());
    }
    assert!(all.windows(2).any(|pair| pair[0] != pair[1]));
    // The batch's runs are the same as fresh ones.
    for (run, _) in batch().iter().take(3) {
        assert_eq!(run.hashes, run_scenario(&base, run.seed).hashes);
    }
}

// ---------------------------------------------------------------------
// What the designer needs to see: run with --nocapture.
// ---------------------------------------------------------------------

#[test]
fn print_the_summary() {
    eprintln!("== Scenario 4c2a summary over seeds 1 to {SEEDS} ==");
    let c = counts();
    eprintln!("A has a free round against B: {} (537..=660)", c.a_free);
    eprintln!("A and B pass unseen: {} (0..=9)", c.unseen);
    eprintln!("C alarmed: {} (436..=561)", c.alarmed);
    let (n, k) = c.alarmed_cond;
    eprintln!(
        "C alarmed and second fight took place: n {n}, C sighted in round 1 in k {k} (p 0.5, four sigma: (k - np)^2 <= 16np(1-p): {})",
        within_four_sigma(k, n, 500_000)
    );
    let (n, k) = c.quiet_cond;
    eprintln!(
        "C not alarmed and second fight took place: n {n}, k {k} (p 0.25: {})",
        within_four_sigma(k, n, 250_000)
    );
    let out = after_the_night();
    eprintln!(
        "after the night, slowest ship of A held its state to the end of the day: working {}, degraded {}, out {}; state changed by the end of the day {} (of them freed by a sinking {}); no ship of A afloat after the night {}",
        out.rows[0], out.rows[1], out.rows[2], out.changed, out.freed, out.none_afloat
    );
    eprintln!(
        "state changed only between the two night fights (the tables do not apply): {}",
        out.changed_between
    );
    let bo = breaking_off();
    eprintln!(
        "A broke off from B and met C: {} pulses (slipped away in round 1: {}, broke off at the end of round 1: {}); A had a free round against C when it had not broken off: {}",
        bo.cases, bo.slipped, bo.broke_off_in_round_1, bo.free_rounds_otherwise
    );
    let second_fights = batch().iter().filter(|(_, p)| p.second().is_some()).count();
    let day_fights: usize = batch()
        .iter()
        .map(|(_, p)| p.fights.iter().filter(|f| !f.night()).count())
        .sum();
    eprintln!(
        "runs with a second night fight: {second_fights}; fights by day in all: {day_fights}"
    );
    let describe = |seed: u64| {
        let (_, p) = &batch()[usize::try_from(seed - 1).expect("small")];
        let first = p.first();
        format!(
            "seed {seed}: first fight {:?}, ended ({}) in round {}, salvos {}",
            opening(first),
            first.reason.name(),
            first.ended_round,
            first.salvos.len()
        )
    };
    let alarmed = batch()
        .iter()
        .find(|(_, p)| !p.night_alarms(3).is_empty())
        .map(|(run, _)| run.seed);
    let quiet = batch()
        .iter()
        .find(|(_, p)| p.night_alarms(3).is_empty() && opening(p.first()) != Opening::PassUnseen)
        .map(|(run, _)| run.seed);
    let unseen = batch()
        .iter()
        .find(|(_, p)| opening(p.first()) == Opening::PassUnseen)
        .map(|(run, _)| run.seed);
    for (label, seed) in [
        ("C alarmed", alarmed),
        ("C not alarmed (a salvo was fired)", quiet),
        ("A and B passed unseen", unseen),
    ] {
        eprintln!(
            "example, {label}: {}",
            seed.map_or("no such seed".to_string(), describe)
        );
    }
}
