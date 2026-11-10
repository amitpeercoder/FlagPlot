//! Scenario 4a: two cruiser forces meet at night. One is alert and one is not.
//!
//! Stage 4a of scenario 4. It proves the surface combat rule on placeholder
//! ships: when a fight starts, who sights whom, surprise, gun and torpedo
//! fire, damage, sinking, breaking off and what each side believes
//! afterwards. The real map (4b) and Savo Island (4c) follow.
//!
//! The data is placeholder data: see `data/scenarios/04a_night_action.toml`.
//! The rule is `docs/decisions/0014-surface-combat.md`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! Inputs, from the data files:
//!
//! - Force A, task force 1, Japanese: cruisers 1, 2 and 3. Alert. It starts
//!   at (0,0) and steams east to (10,0) at 16 kn.
//! - Force B, task force 2, Allied: cruisers 4, 5 and 6. Not alert. It lies
//!   stopped at (5,0).
//! - Both are ordered to engage and to break off at 0.5.
//! - Placeholder cruiser: 8 guns, 3 rounds a gun a minute, 100 rounds a gun,
//!   gun damage 20; 8 torpedo tubes, torpedo damage 200; toughness 400; no
//!   radar.
//! - Combat figures: rounds of 5 minutes, 6 at most. At night a force sights
//!   with chance 0.5, times 0.5 if it is not alert. At night a gun shot hits
//!   with chance 0.03 and a torpedo with 0.04. A surprised target multiplies
//!   the chance by 1.5 and a target lit by fire by 2. A gun hit lands on
//!   flotation, system or fire with chances 0.5, 0.3 and 0.2.
//!
//! ## The meeting
//!
//! Force A steams 192 nm in the night pulse: four hexes and 32 nm, so it has
//! left (4,0) and is 32 nm toward (5,0). Its position is the nearest centre,
//! (5,0), which is where force B lies. The two fight in the surface
//! engagement step of the night pulse of day 1. It is dark.
//!
//! In the morning pulse force A steams on to 288 nm, position (7,0), and in
//! the afternoon to 384 nm, position (10,0). There is no second fight. Each
//! cruiser of force A still afloat burns 384 x 0.25 = 96 t and ends the day
//! with 1904 t. Force B does not move and burns nothing.
//!
//! ## Who sights whom
//!
//! Each round a force that has not sighted the enemy rolls: force A at 0.5,
//! force B at 0.5 x 0.5 = 0.25. The rolls are independent.
//!
//! | In a round where neither has yet sighted | Chance             |
//! | ---------------------------------------- | ------------------ |
//! | Both sight                               | 0.5 x 0.25 = 0.125 |
//! | Only A sights                            | 0.5 x 0.75 = 0.375 |
//! | Only B sights                            | 0.5 x 0.25 = 0.125 |
//! | Neither sights                           | 0.5 x 0.75 = 0.375 |
//!
//! If only one force sights, it fires that round on an enemy that cannot
//! answer: a free round. The enemy has sighted from the next round. With six
//! rounds at most, the chance that neither ever sights is 0.375^6 =
//! 0.002781. The chance that the fight has opened by then is 0.997219, and
//! it opens in one of three ways, in the proportions 0.375 : 0.125 : 0.125
//! out of 0.625.
//!
//! | How the fight opens      | Chance                              |
//! | ------------------------ | ----------------------------------- |
//! | A has a free round       | 0.997219 x 0.375 / 0.625 = 0.598331 |
//! | B has a free round       | 0.997219 x 0.125 / 0.625 = 0.199444 |
//! | Both fire from the start | 0.199444                            |
//! | They pass unseen         | 0.002781                            |
//!
//! ## Detection rates over 1,000 runs
//!
//! The scenario runs with seeds 1 to 1000. Each count below follows a
//! binomial distribution and must fall within four standard deviations of
//! its mean. The seeds are fixed, so the result never varies.
//!
//! | Count of runs where         | p        | Mean   | Std dev | Allowed    |
//! | --------------------------- | -------- | ------ | ------- | ---------- |
//! | A sights in round 1         | 0.5      | 500.00 | 15.81   | 437 to 563 |
//! | B sights in round 1         | 0.25     | 250.00 | 13.69   | 196 to 304 |
//! | A has a free round          | 0.598331 | 598.33 | 15.50   | 537 to 660 |
//! | B has a free round          | 0.199444 | 199.44 | 12.64   | 149 to 249 |
//! | Both fire from the start    | 0.199444 | 199.44 | 12.64   | 149 to 249 |
//! | They pass unseen            | 0.002781 | 2.78   | 1.67    | 0 to 9     |
//!
//! The four ways a fight can open add to 1000 runs exactly.
//!
//! ## A salvo
//!
//! A cruiser with no system damage fires 8 x 3 x 5 = 120 gun shots in a
//! round. A gun hit does 100 x 20 / 400 = 5 points. A torpedo hit does 100 x
//! 200 / 400 = 50 points of flotation damage: one cripples a cruiser and two
//! sink it. Torpedoes are fired once, in the first round the ship fires.
//!
//! | Salvo                          | Shots | Chance to hit    | Expected | Hits                 |
//! | ------------------------------ | ----- | ---------------- | -------- | -------------------- |
//! | Guns, target not surprised     | 120   | 0.03             | 3.6      | 3, or 4 at 0.6       |
//! | Guns, target surprised         | 120   | 0.03 x 1.5       | 5.4      | 5, or 6 at 0.4       |
//! | Guns, target lit               | 120   | 0.03 x 2         | 7.2      | 7, or 8 at 0.2       |
//! | Torpedoes, target not surprised| 8     | 0.04             | 0.32     | 0, or 1 at 0.32      |
//! | Torpedoes, target surprised    | 8     | 0.04 x 1.5       | 0.48     | 0, or 1 at 0.48      |
//! | Torpedoes, target lit          | 8     | 0.04 x 2         | 0.64     | 0, or 1 at 0.64      |
//! | Guns, firer at 15 system damage| 102   | 0.03             | 3.06     | 3, or 4 at 0.06      |
//!
//! So in a free round every firing cruiser lands 5 or 6 gun hits on a
//! target that cannot answer and, with chance 0.48, a torpedo hit that
//! cripples it. When both fire from the start, each lands 3 or 4 gun hits
//! and a torpedo hit with chance 0.32.
//!
//! In every run and every salvo the test asserts, from the salvo's
//! breakdown: shots = 120 x (100 - the firer's system damage) / 100 for guns
//! and 8 for torpedoes; the chance is the base times the factors that apply;
//! expected hits = shots x chance; and hits are the whole part or one more.
//! A ship fires 120 shots a round at most and has 800 rounds, so ammunition
//! never limits fire in this scenario. Each ship ends with 800 less the gun
//! shots it fired.
//!
//! ## Hits over 1,000 runs
//!
//! Each salvo's fraction is one roll. Summed over every gun salvo in the
//! 1,000 runs, with H the hits, E the expected hits and f each salvo's
//! fraction, the test asserts (H - E)^2 <= 16 x the sum of f(1 - f). The
//! same holds for torpedo salvoes.
//!
//! Of all gun hits, n, the numbers on flotation, system and fire must each
//! be within four standard deviations of 0.5n, 0.3n and 0.2n: (k - np)^2 <=
//! 16np(1 - p). Every torpedo hit is on flotation.
//!
//! ## Damage, sinking and breaking off
//!
//! - Each track is the sum of the hits recorded on it, and stops at 100.
//! - A ship is sunk exactly when its flotation damage reached 100. A sunk
//!   ship is in no task force, fires in the round it sank and never after,
//!   and is never a target after.
//! - Ships are matched in ID order among those afloat at the start of the
//!   round: with three afloat on each side, ship 1 fires at 4, 2 at 5 and 3
//!   at 6, and back again. With two targets afloat the third firer takes the
//!   first target.
//! - A force breaks off at the end of a round when at least half the ships
//!   it began with are sunk or have flotation damage of 50 or more. With
//!   three ships that is two of them. One is not enough: 1/3 is below 0.5.
//! - A fight ends when a force breaks off, when one force has no ship
//!   afloat, when they pass unseen, or after round 6. The test replays each
//!   run's events and asserts the fight ended in the round these rules give,
//!   and for the reason they give.
//!
//! ## What each side believes afterwards
//!
//! - A force that sighted the enemy, or was fired on, sends one sighting
//!   report when the fight ends. It makes one contact at (5,0) or a
//!   neighbouring hex, of a kind the confusion table allows for cruisers. Its
//!   count is the enemy ships afloat at the end of the fight, or one more or
//!   one fewer, and never below one. A force that neither sighted nor was
//!   fired on sends none.
//!
//!   Corrected by the architect on review, before the code was accepted. The
//!   first version said "2 to 4 ships", which forgot that a force can lose a
//!   ship, and "a force that never sighted sends none", which left a force
//!   that was surprised and broke off in the same round with no report of
//!   who had attacked it.
//! - A force that fired claims as sunk every enemy ship that sank plus every
//!   enemy ship left afloat with fire damage of 30 or more. The claim is
//!   never less than the truth.
//! - Force B is alert at the end of every run in which it sighted or was
//!   fired on, and not alert in the runs where the forces passed unseen.
//!
//! # Properties
//!
//! Beside the numbers above, the tests check what must always hold:
//!
//! - Firing first pays. Over the runs where A had a free round, the total
//!   flotation damage to force B is greater than the total to force A. Over
//!   the runs where B had a free round, it is the other way round.
//! - More guns never mean fewer expected hits, and system damage never means
//!   more: the expected gun hits against an unsurprised, unlit target with
//!   4, 8 and 12 guns are 1.8, 3.6 and 5.4, and with 8 guns at 0, 25, 50 and
//!   100 system damage they are 3.6, 2.7, 1.8 and 0.
//! - An alert force never sights less readily than one that is not: 0.5
//!   against 0.25 at night, and 0.7 against 0.35 with radar.
//! - No ship has negative ammunition, and no track is below 0 or above 100.
//!   Every ship is in exactly one task force or is sunk.
//! - Belief holds nothing the reports did not say.
//! - A replay of any seed reproduces the daily hash.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use sim_core::events::{Cause, DamageTrack, EngagementEndReason, Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::TaskForceId;
use sim_core::refdata::ShipKind;
use sim_core::state::{ShipLocation, Side};
use sim_core::time::Pulse;
use sim_data::Scenario;

const SEEDS: u64 = 1000;
const MEETING: Hex = Hex::new(5, 0);
/// The ships of force A (task force 1) and force B (task force 2).
const SHIPS: [[u32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
/// The side of force A and of force B.
const SIDES: [Side; 2] = [Side::Japanese, Side::Allied];
const MAX_ROUNDS: u32 = 6;
/// Written out from the hand calculation, not read from the data files.
const GUN_BASE: Fixed = Fixed::from_raw(30_000);
const TORPEDO_BASE: Fixed = Fixed::from_raw(40_000);
const SURPRISED: Fixed = Fixed::from_raw(1_500_000);
const LIT: Fixed = Fixed::from_raw(2_000_000);
const GUN_HIT_POINTS: i64 = 5;
const TORPEDO_HIT_POINTS: i64 = 50;
const CRIPPLED: i64 = 50;
const CLAIMED_FIRE: i64 = 30;

fn fx(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn millionths(value: i64) -> Fixed {
    Fixed::from_raw(value)
}

/// Which force a ship belongs to at the start: 0 for A, 1 for B.
fn force_of_ship(ship: u32) -> usize {
    assert!(
        (1..=6).contains(&ship),
        "ship {ship} is not in the scenario"
    );
    usize::from(ship > 3)
}

/// Index of a task force: 0 for task force 1, 1 for task force 2.
fn force_index(task_force: TaskForceId) -> usize {
    assert!((1..=2).contains(&task_force.raw()));
    usize::try_from(task_force.raw() - 1).expect("small")
}

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/04a_night_action.toml")
}

fn load() -> Scenario {
    Scenario::load(&scenario_path()).expect("scenario 4a loads")
}

// ---------------------------------------------------------------------
// What a run leaves behind: its events, and its final state.
// ---------------------------------------------------------------------

#[derive(Clone, Debug)]
struct ShipEnd {
    side: Side,
    location: ShipLocation,
    fuel: Fixed,
    flotation: Fixed,
    system: Fixed,
    fire: Fixed,
    rounds: Fixed,
    torpedoes_ready: bool,
}

#[derive(Clone, Debug)]
struct ForceEnd {
    position: Hex,
    ships: BTreeSet<u32>,
    alert: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ContactEnd {
    id: u32,
    hex: Hex,
    count: u32,
    kind: ShipKind,
    age: u32,
    confidence: Fixed,
    last_event: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ClaimEnd {
    hex: Hex,
    day: u32,
    pulse: Pulse,
    claimed_sunk: u32,
    event: u32,
}

#[derive(Clone, Debug)]
struct RunData {
    seed: u64,
    events: Vec<Event>,
    ships: BTreeMap<u32, ShipEnd>,
    forces: BTreeMap<u32, ForceEnd>,
    contacts: BTreeMap<Side, Vec<ContactEnd>>,
    claims: BTreeMap<Side, Vec<ClaimEnd>>,
}

fn snapshot(seed: u64, scenario: &Scenario) -> RunData {
    let world = &scenario.world;
    let truth = world.umpire();
    let mut contacts = BTreeMap::new();
    let mut claims = BTreeMap::new();
    for side in Side::ALL {
        let view = world.side_view(side);
        contacts.insert(
            side,
            view.contacts()
                .map(|contact| ContactEnd {
                    id: contact.id.raw(),
                    hex: contact.estimated_hex,
                    count: contact.estimated_ship_count,
                    kind: contact.estimated_kind,
                    age: contact.age_pulses,
                    confidence: contact.confidence,
                    last_event: contact.last_report_event.raw(),
                })
                .collect::<Vec<_>>(),
        );
        claims.insert(
            side,
            view.claims()
                .iter()
                .map(|claim| ClaimEnd {
                    hex: claim.hex,
                    day: claim.day,
                    pulse: claim.pulse,
                    claimed_sunk: claim.claimed_sunk,
                    event: claim.received_event.raw(),
                })
                .collect::<Vec<_>>(),
        );
    }
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
                        side: ship.side,
                        location: ship.location,
                        fuel: ship.fuel_t,
                        flotation: ship.flotation_damage,
                        system: ship.system_damage,
                        fire: ship.fire_damage,
                        rounds: ship.gun_rounds,
                        torpedoes_ready: ship.torpedoes_ready,
                    },
                )
            })
            .collect(),
        forces: truth
            .task_forces
            .iter()
            .map(|(id, force)| {
                (
                    id.raw(),
                    ForceEnd {
                        position: force.position(),
                        ships: force.ships.iter().map(|ship| ship.raw()).collect(),
                        alert: force.alert,
                    },
                )
            })
            .collect(),
        contacts,
        claims,
    }
}

struct Batch {
    runs: Vec<RunData>,
    elapsed: Duration,
}

/// The wall clock is banned from the simulation (`clippy.toml`). The test is
/// not the simulation: it only times how long the 1,000 runs take, and the
/// time never feeds back into any result.
#[allow(clippy::disallowed_methods)]
fn start_timer() -> Instant {
    Instant::now()
}

/// Seeds 1 to 1000, run once and shared by every test below. Breakdowns are
/// recorded (the default of `SimConfig`).
fn batch() -> &'static Batch {
    static BATCH: OnceLock<Batch> = OnceLock::new();
    BATCH.get_or_init(|| {
        let base = load();
        let timer = start_timer();
        let runs = (1..=SEEDS)
            .map(|seed| {
                let mut run = base.with_seed(seed).expect("a seed can be set");
                run.run_to_end().expect("the run resolves");
                snapshot(seed, &run)
            })
            .collect();
        Batch {
            runs,
            elapsed: timer.elapsed(),
        }
    })
}

// ---------------------------------------------------------------------
// The events of a run, sorted out by kind.
// ---------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Roll {
    round: u32,
    force: usize,
    chance: Fixed,
    sighted: bool,
}

#[derive(Clone, Debug)]
struct SalvoRec {
    event: u32,
    round: u32,
    firer: u32,
    target: u32,
    weapon: Weapon,
    shots: Fixed,
    chance: Fixed,
    expected: Fixed,
    hits: u32,
    factors: Vec<(&'static str, Fixed)>,
    result: Fixed,
}

impl SalvoRec {
    fn factor(&self, name: &str) -> Fixed {
        self.factors
            .iter()
            .find(|(n, _)| *n == name)
            .unwrap_or_else(|| panic!("salvo #{} has no factor {name}", self.event))
            .1
    }
}

#[derive(Clone, Debug)]
struct DamageRec {
    round: u32,
    ship: u32,
    track: DamageTrack,
    hits: u32,
    points: Fixed,
    after: Fixed,
    cause: u32,
}

#[derive(Clone, Debug)]
struct SunkRec {
    round: u32,
    ship: u32,
    force: usize,
    cause: u32,
}

#[derive(Clone, Debug)]
struct ReportRec {
    event: u32,
    side: Side,
    contact: u32,
    hex: Hex,
    count: u32,
    kind: ShipKind,
    confidence: Fixed,
    created: bool,
    cause: u32,
}

#[derive(Clone, Debug)]
struct ClaimRec {
    event: u32,
    side: Side,
    hex: Hex,
    claimed_sunk: u32,
    cause: u32,
}

#[derive(Clone, Debug)]
struct Ended {
    event: u32,
    round: u32,
    hex: Hex,
    reason: EngagementEndReason,
}

#[derive(Clone, Debug)]
struct Parsed {
    seed: u64,
    begun_event: u32,
    ended: Ended,
    rolls: Vec<Roll>,
    surprised: Vec<(u32, usize)>,
    salvos: Vec<SalvoRec>,
    damages: Vec<DamageRec>,
    sunk: Vec<SunkRec>,
    alerts: Vec<(u32, usize)>,
    broke_off: Vec<(u32, usize, Fixed)>,
    passed_unseen: Vec<u32>,
    slipped: usize,
    /// Reports and claims after the fight, in order: (is a claim, side).
    after_fight: Vec<(bool, Side)>,
    reports: Vec<ReportRec>,
    claims: Vec<ClaimRec>,
}

fn cause_event(event: &Event) -> u32 {
    let ids: Vec<u32> = event
        .causes
        .iter()
        .filter_map(|cause| match cause {
            Cause::Event(id) => Some(id.raw()),
            Cause::Order(_) => None,
        })
        .collect();
    assert_eq!(ids.len(), 1, "#{} has one event cause", event.id.raw());
    ids[0]
}

/// Sorts a run's events. Asserts the shape every fight must have: exactly
/// one `EngagementBegun` and one `EngagementEnded`, every fight event in
/// the night pulse of day 1 and between the two, every report and claim
/// after the end.
fn parse(seed: u64, events: &[Event]) -> Parsed {
    let mut begun: Option<(u32, TaskForceId, TaskForceId, Hex)> = None;
    let mut ended: Option<Ended> = None;
    let mut parsed = Parsed {
        seed,
        begun_event: 0,
        ended: Ended {
            event: 0,
            round: 0,
            hex: MEETING,
            reason: EngagementEndReason::MaxRounds,
        },
        rolls: Vec::new(),
        surprised: Vec::new(),
        salvos: Vec::new(),
        damages: Vec::new(),
        sunk: Vec::new(),
        alerts: Vec::new(),
        broke_off: Vec::new(),
        passed_unseen: Vec::new(),
        slipped: 0,
        after_fight: Vec::new(),
        reports: Vec::new(),
        claims: Vec::new(),
    };
    for (index, event) in events.iter().enumerate() {
        let id = event.id.raw();
        assert_eq!(usize::try_from(id).expect("small"), index + 1);
        let in_fight = |begun: &Option<_>, ended: &Option<Ended>| {
            assert_eq!(
                (event.day, event.pulse),
                (1, Some(Pulse::Night)),
                "seed {seed}: fight event #{id}"
            );
            assert!(begun.is_some() && ended.is_none(), "seed {seed}: #{id}");
        };
        match &event.kind {
            EventKind::EngagementBegun { a, b, hex } => {
                assert!(begun.is_none(), "seed {seed}: a second fight");
                assert_eq!((event.day, event.pulse), (1, Some(Pulse::Night)));
                begun = Some((id, *a, *b, *hex));
            }
            EventKind::SightingRoll {
                round,
                task_force,
                enemy,
                chance,
                sighted,
            } => {
                in_fight(&begun, &ended);
                assert_eq!(force_index(*task_force) + force_index(*enemy), 1);
                let breakdown = event.breakdown.as_ref().expect("a breakdown is recorded");
                assert_eq!(breakdown.result, *chance);
                parsed.rolls.push(Roll {
                    round: *round,
                    force: force_index(*task_force),
                    chance: *chance,
                    sighted: *sighted,
                });
            }
            EventKind::ForceSurprised {
                round, task_force, ..
            } => {
                in_fight(&begun, &ended);
                parsed.surprised.push((*round, force_index(*task_force)));
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
                in_fight(&begun, &ended);
                let breakdown = event.breakdown.as_ref().expect("a breakdown is recorded");
                parsed.salvos.push(SalvoRec {
                    event: id,
                    round: *round,
                    firer: firer.raw(),
                    target: target.raw(),
                    weapon: *weapon,
                    shots: *shots,
                    chance: *chance,
                    expected: *expected,
                    hits: *hits,
                    factors: breakdown
                        .factors
                        .iter()
                        .map(|factor| (factor.name, factor.value))
                        .collect(),
                    result: breakdown.result,
                });
            }
            EventKind::ShipDamaged {
                round,
                ship,
                track,
                hits,
                points,
                value_after,
            } => {
                in_fight(&begun, &ended);
                parsed.damages.push(DamageRec {
                    round: *round,
                    ship: ship.raw(),
                    track: *track,
                    hits: *hits,
                    points: *points,
                    after: *value_after,
                    cause: cause_event(event),
                });
            }
            EventKind::ShipSunk {
                round,
                ship,
                task_force,
            } => {
                in_fight(&begun, &ended);
                parsed.sunk.push(SunkRec {
                    round: *round,
                    ship: ship.raw(),
                    force: force_index(*task_force),
                    cause: cause_event(event),
                });
            }
            EventKind::ForceBecameAlert { round, task_force } => {
                in_fight(&begun, &ended);
                parsed.alerts.push((*round, force_index(*task_force)));
            }
            EventKind::ForceBrokeOff {
                round,
                task_force,
                share,
            } => {
                in_fight(&begun, &ended);
                parsed
                    .broke_off
                    .push((*round, force_index(*task_force), *share));
            }
            EventKind::ForceSlippedAway { .. } => {
                in_fight(&begun, &ended);
                parsed.slipped += 1;
            }
            EventKind::ForcesPassedUnseen { round, .. } => {
                in_fight(&begun, &ended);
                parsed.passed_unseen.push(*round);
            }
            EventKind::EngagementEnded {
                round,
                a,
                b,
                hex,
                reason,
            } => {
                in_fight(&begun, &ended);
                let (_, begun_a, begun_b, begun_hex) = begun.expect("begun");
                assert_eq!((*a, *b, *hex), (begun_a, begun_b, begun_hex));
                ended = Some(Ended {
                    event: id,
                    round: *round,
                    hex: *hex,
                    reason: *reason,
                });
            }
            EventKind::ReportReceived {
                side,
                contact,
                hex,
                ship_count,
                kind,
                confidence,
                created,
            } => {
                let end = ended.as_ref().expect("a report follows the fight");
                parsed.after_fight.push((false, *side));
                parsed.reports.push(ReportRec {
                    event: id,
                    side: *side,
                    contact: contact.raw(),
                    hex: *hex,
                    count: *ship_count,
                    kind: *kind,
                    confidence: *confidence,
                    created: *created,
                    cause: cause_event(event),
                });
                assert_eq!(parsed.reports.last().map(|r| r.cause), Some(end.event));
            }
            EventKind::ClaimReceived {
                side,
                hex,
                claimed_sunk,
            } => {
                let end = ended.as_ref().expect("a claim follows the fight");
                parsed.after_fight.push((true, *side));
                parsed.claims.push(ClaimRec {
                    event: id,
                    side: *side,
                    hex: *hex,
                    claimed_sunk: *claimed_sunk,
                    cause: cause_event(event),
                });
                assert_eq!(parsed.claims.last().map(|c| c.cause), Some(end.event));
            }
            _ => {}
        }
    }
    let (begun_event, ..) = begun.unwrap_or_else(|| panic!("seed {seed}: no fight"));
    parsed.begun_event = begun_event;
    parsed.ended = ended.unwrap_or_else(|| panic!("seed {seed}: the fight never ended"));
    parsed
}

/// The model's tracks: damage per ship, built only from the `points` of the
/// `ShipDamaged` events, capped at 100.
#[derive(Clone, Debug)]
struct Tracks {
    flotation: BTreeMap<u32, Fixed>,
    system: BTreeMap<u32, Fixed>,
    fire: BTreeMap<u32, Fixed>,
}

impl Tracks {
    fn new() -> Tracks {
        let zero = || (1..=6).map(|ship| (ship, Fixed::ZERO)).collect();
        Tracks {
            flotation: zero(),
            system: zero(),
            fire: zero(),
        }
    }

    fn track(&mut self, track: DamageTrack) -> &mut BTreeMap<u32, Fixed> {
        match track {
            DamageTrack::Flotation => &mut self.flotation,
            DamageTrack::System => &mut self.system,
            DamageTrack::Fire => &mut self.fire,
        }
    }

    fn afloat(&self, force: usize) -> Vec<u32> {
        SHIPS[force]
            .iter()
            .copied()
            .filter(|ship| self.flotation[ship] < fx(100))
            .collect()
    }
}

/// One round as the model sees it.
#[derive(Clone, Debug)]
struct RoundState {
    round: u32,
    /// Tracks at the start of the round, and at its end.
    start: Tracks,
    end: Tracks,
    /// Ships afloat at the start of the round, in ID order.
    afloat: [Vec<u32>; 2],
    /// The force rolls this round: it had not sighted the enemy.
    rolls: [bool; 2],
    /// The force has sighted the enemy after the sighting step.
    sighted: [bool; 2],
    /// The force is fired on this round.
    fired_on: [bool; 2],
    /// The force is surprised: it has not sighted, and the enemy fires.
    surprised: [bool; 2],
}

struct Walk {
    rounds: Vec<RoundState>,
    /// The model's value after each `ShipDamaged` event, in event order.
    after: Vec<Fixed>,
    /// The round in which each ship's flotation reached 100.
    sunk_round: BTreeMap<u32, u32>,
}

impl Walk {
    fn last(&self) -> &RoundState {
        self.rounds.last().expect("a fight has a round")
    }
}

/// Replays the damage of a fight round by round, with the sighting and
/// surprise rules written out again here.
fn walk(p: &Parsed) -> Walk {
    let mut tracks = Tracks::new();
    let mut after = Vec::new();
    let mut rounds = Vec::new();
    let mut sunk_round = BTreeMap::new();
    let mut sighted_before = [false; 2];
    let mut fired_on_before = [false; 2];
    let mut next_damage = 0;
    for round in 1..=p.ended.round {
        let start = tracks.clone();
        let afloat = [start.afloat(0), start.afloat(1)];
        let rolls = [0, 1].map(|f| !sighted_before[f] && !fired_on_before[f]);
        let success = |f: usize| {
            p.rolls
                .iter()
                .any(|r| r.round == round && r.force == f && r.sighted)
        };
        let sighted = [0, 1].map(|f| sighted_before[f] || fired_on_before[f] || success(f));
        let salvos: Vec<&SalvoRec> = p.salvos.iter().filter(|s| s.round == round).collect();
        let fired_on = [0, 1].map(|f| salvos.iter().any(|s| force_of_ship(s.target) == f));
        let fired = [0, 1].map(|f| salvos.iter().any(|s| force_of_ship(s.firer) == f));
        let surprised = [!sighted[0] && fired[1], !sighted[1] && fired[0]];
        while next_damage < p.damages.len() && p.damages[next_damage].round == round {
            let damage = &p.damages[next_damage];
            let track = tracks.track(damage.track);
            let value = std::cmp::min(track[&damage.ship] + damage.points, fx(100));
            track.insert(damage.ship, value);
            after.push(value);
            if damage.track == DamageTrack::Flotation && value >= fx(100) {
                sunk_round.entry(damage.ship).or_insert(round);
            }
            next_damage += 1;
        }
        rounds.push(RoundState {
            round,
            start,
            end: tracks.clone(),
            afloat,
            rolls,
            sighted,
            fired_on,
            surprised,
        });
        sighted_before = sighted;
        fired_on_before = fired_on;
    }
    assert_eq!(
        next_damage,
        p.damages.len(),
        "seed {}: damage after the fight ended",
        p.seed
    );
    Walk {
        rounds,
        after,
        sunk_round,
    }
}

/// How a fight opens, from the sighting rolls alone.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Opening {
    AFreeRound,
    BFreeRound,
    BothFire,
    PassUnseen,
}

fn opening(p: &Parsed) -> Opening {
    let Some(first) = p.rolls.iter().filter(|r| r.sighted).map(|r| r.round).min() else {
        return Opening::PassUnseen;
    };
    let sighted = |force: usize| {
        p.rolls
            .iter()
            .any(|r| r.round == first && r.force == force && r.sighted)
    };
    match (sighted(0), sighted(1)) {
        (true, false) => Opening::AFreeRound,
        (false, true) => Opening::BFreeRound,
        (true, true) => Opening::BothFire,
        (false, false) => unreachable!("a round with a success"),
    }
}

fn parsed_runs() -> &'static Vec<Parsed> {
    static PARSED: OnceLock<Vec<Parsed>> = OnceLock::new();
    PARSED.get_or_init(|| {
        batch()
            .runs
            .iter()
            .map(|run| parse(run.seed, &run.events))
            .collect()
    })
}

/// Whether k out of n is within four standard deviations of probability
/// `p` (in millionths), in exact integer arithmetic.
fn within_four_sigma(k: i64, n: i64, p: i64) -> bool {
    let (k, n, p) = (i128::from(k), i128::from(n), i128::from(p));
    let scale = 1_000_000;
    let off = k * scale - n * p;
    off * off <= 16 * n * p * (scale - p)
}

fn total_flotation(run: &RunData, force: usize) -> i64 {
    SHIPS[force]
        .iter()
        .map(|ship| run.ships[ship].flotation.raw())
        .sum()
}

fn ships_sunk(run: &RunData, force: usize) -> i64 {
    let count = SHIPS[force]
        .iter()
        .filter(|ship| run.ships[ship].location == ShipLocation::Sunk)
        .count();
    i64::try_from(count).expect("small")
}

// ---------------------------------------------------------------------
// The meeting.
// ---------------------------------------------------------------------

#[test]
fn the_forces_meet_once_at_five_zero_in_the_night_pulse() {
    let batch = batch();
    assert_eq!(batch.runs.len(), 1000);
    let own = load();
    assert!(own.placeholder);
    assert_eq!(own.days, 1);
    for run in &batch.runs {
        let seed = run.seed;
        let begun: Vec<&Event> = run
            .events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::EngagementBegun { .. }))
            .collect();
        assert_eq!(begun.len(), 1, "seed {seed}: one fight in the day");
        assert_eq!((begun[0].day, begun[0].pulse), (1, Some(Pulse::Night)));
        assert_eq!(
            begun[0].kind,
            EventKind::EngagementBegun {
                a: TaskForceId::new(1),
                b: TaskForceId::new(2),
                hex: MEETING,
            }
        );
        let ends = run
            .events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::EngagementEnded { .. }))
            .count();
        assert_eq!(ends, 1, "seed {seed}");

        // Afterwards.
        let a_afloat: Vec<u32> = SHIPS[0]
            .iter()
            .copied()
            .filter(|s| run.ships[s].location != ShipLocation::Sunk)
            .collect();
        let steps: Vec<(Pulse, Hex, Hex, Fixed)> = run
            .events
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::TaskForceSteamed {
                    task_force,
                    from,
                    to,
                    distance_nm,
                    ..
                } if task_force.raw() == 1 => {
                    Some((event.pulse.expect("a pulse"), *from, *to, *distance_nm))
                }
                _ => None,
            })
            .collect();
        if !a_afloat.is_empty() {
            assert_eq!(run.forces[&1].position, Hex::new(10, 0), "seed {seed}");
            assert_eq!(
                steps,
                vec![
                    (Pulse::Night, Hex::new(0, 0), Hex::new(5, 0), fx(192)),
                    (Pulse::Morning, Hex::new(5, 0), Hex::new(7, 0), fx(96)),
                    (Pulse::Afternoon, Hex::new(7, 0), Hex::new(10, 0), fx(96)),
                ],
                "seed {seed}"
            );
            for ship in &a_afloat {
                assert_eq!(run.ships[ship].fuel, fx(1904), "seed {seed} ship {ship}");
            }
        } else {
            assert_eq!(steps.len(), 1, "seed {seed}: a sunk force steams no more");
        }
        assert_eq!(run.forces[&2].position, MEETING, "seed {seed}");
        for ship in SHIPS[1] {
            if run.ships[&ship].location != ShipLocation::Sunk {
                assert_eq!(run.ships[&ship].fuel, fx(2000), "seed {seed} ship {ship}");
            }
        }
    }
}

// ---------------------------------------------------------------------
// Detection.
// ---------------------------------------------------------------------

#[test]
fn detection_falls_where_the_hand_calculation_says() {
    let (mut a1, mut b1) = (0i64, 0i64);
    let mut ways = [0i64; 4];
    for p in parsed_runs() {
        // A force rolls once a round until it sights, at 0.5 for A and 0.25
        // for B, and never rolls after.
        let w = walk(p);
        let mut rolled = [0u32; 2];
        for roll in &p.rolls {
            let expected = if roll.force == 0 {
                millionths(500_000)
            } else {
                millionths(250_000)
            };
            assert_eq!(roll.chance, expected, "seed {}", p.seed);
            rolled[roll.force] += 1;
        }
        for (f, rolled_f) in rolled.iter().enumerate() {
            let expected_rolls = w.rounds.iter().filter(|r| r.rolls[f]).count();
            assert_eq!(
                usize::try_from(*rolled_f).expect("small"),
                expected_rolls,
                "seed {} force {f}",
                p.seed
            );
            for st in &w.rounds {
                let rolls_now = p
                    .rolls
                    .iter()
                    .filter(|r| r.round == st.round && r.force == f)
                    .count();
                assert_eq!(rolls_now, usize::from(st.rolls[f]), "seed {}", p.seed);
            }
        }
        a1 += i64::from(
            p.rolls
                .iter()
                .any(|r| r.round == 1 && r.force == 0 && r.sighted),
        );
        b1 += i64::from(
            p.rolls
                .iter()
                .any(|r| r.round == 1 && r.force == 1 && r.sighted),
        );
        let way = opening(p);
        ways[match way {
            Opening::AFreeRound => 0,
            Opening::BFreeRound => 1,
            Opening::BothFire => 2,
            Opening::PassUnseen => 3,
        }] += 1;
        // Passing unseen is what the end of the fight says too.
        assert_eq!(
            way == Opening::PassUnseen,
            p.ended.reason == EngagementEndReason::PassedUnseen,
            "seed {}",
            p.seed
        );
    }
    eprintln!("A sights in round 1: {a1}");
    eprintln!("B sights in round 1: {b1}");
    eprintln!(
        "A has a free round: {}, B has a free round: {}, both fire from the start: {}, pass unseen: {}",
        ways[0], ways[1], ways[2], ways[3]
    );
    assert!((437..=563).contains(&a1), "A sights in round 1: {a1}");
    assert!((196..=304).contains(&b1), "B sights in round 1: {b1}");
    assert!((537..=660).contains(&ways[0]), "A free round: {}", ways[0]);
    assert!((149..=249).contains(&ways[1]), "B free round: {}", ways[1]);
    assert!((149..=249).contains(&ways[2]), "both fire: {}", ways[2]);
    assert!((0..=9).contains(&ways[3]), "pass unseen: {}", ways[3]);
    assert_eq!(ways.iter().sum::<i64>(), 1000);
}

// ---------------------------------------------------------------------
// Salvos.
// ---------------------------------------------------------------------

#[test]
fn every_salvo_follows_the_figures() {
    let mut count = 0usize;
    for p in parsed_runs() {
        let w = walk(p);
        let seed = p.seed;

        // ForceSurprised events are exactly the surprised forces.
        for st in &w.rounds {
            for f in 0..2 {
                let events = p
                    .surprised
                    .iter()
                    .filter(|(round, force)| *round == st.round && *force == f)
                    .count();
                assert_eq!(events, usize::from(st.surprised[f]), "seed {seed}");
            }
        }
        assert_eq!(
            p.surprised.len(),
            w.rounds
                .iter()
                .map(|st| st.surprised.iter().filter(|s| **s).count())
                .sum::<usize>()
        );

        for salvo in &p.salvos {
            count += 1;
            let st = &w.rounds[usize::try_from(salvo.round - 1).expect("small")];
            let target_force = force_of_ship(salvo.target);
            assert_eq!(force_of_ship(salvo.firer), 1 - target_force);
            let surprised = st.surprised[target_force];
            let lit = st.start.fire[&salvo.target] > Fixed::ZERO;
            let (base, names): (Fixed, [&str; 4]) = match salvo.weapon {
                Weapon::Gun => (GUN_BASE, ["guns", "rate", "minutes", "system_damage"]),
                Weapon::Torpedo => (TORPEDO_BASE, ["tubes", "rate", "minutes", "system_damage"]),
            };
            let surprised_factor = if surprised { SURPRISED } else { Fixed::ONE };
            let lit_factor = if lit { LIT } else { Fixed::ONE };

            // The breakdown's factors are named, in this order, and carry
            // the figures the hand calculation gives.
            let listed: Vec<&str> = salvo.factors.iter().map(|(name, _)| *name).collect();
            assert_eq!(
                listed,
                vec![
                    names[0],
                    names[1],
                    names[2],
                    names[3],
                    "base_chance",
                    "surprised_factor",
                    "lit_factor"
                ],
                "seed {seed}"
            );
            assert_eq!(salvo.factor("base_chance"), base, "seed {seed}");
            assert_eq!(
                salvo.factor("surprised_factor"),
                surprised_factor,
                "seed {seed}"
            );
            assert_eq!(salvo.factor("lit_factor"), lit_factor, "seed {seed}");
            assert_eq!(salvo.result, salvo.expected);

            // Shots.
            match salvo.weapon {
                Weapon::Gun => {
                    let system = st.start.system[&salvo.firer];
                    assert_eq!(salvo.factor("guns"), fx(8));
                    assert_eq!(salvo.factor("rate"), fx(3));
                    assert_eq!(salvo.factor("minutes"), fx(5));
                    assert_eq!(salvo.factor("system_damage"), system, "seed {seed}");
                    assert_eq!(
                        salvo.shots,
                        fx(120).mul_div(fx(100) - system, fx(100)),
                        "seed {seed}: shots"
                    );
                    assert!(salvo.shots > Fixed::ZERO);
                }
                Weapon::Torpedo => {
                    assert_eq!(salvo.factor("tubes"), fx(8));
                    assert_eq!(salvo.factor("rate"), fx(1));
                    assert_eq!(salvo.factor("minutes"), fx(1));
                    assert_eq!(salvo.factor("system_damage"), Fixed::ZERO);
                    assert_eq!(salvo.shots, fx(8));
                }
            }

            // Chance, expected hits, hits.
            assert_eq!(
                salvo.chance,
                base * surprised_factor * lit_factor,
                "seed {seed} salvo #{}",
                salvo.event
            );
            assert_eq!(salvo.expected, salvo.shots * salvo.chance, "seed {seed}");
            let whole = salvo.expected.trunc();
            let fraction = salvo.expected.raw() - whole * 1_000_000;
            let hits = i64::from(salvo.hits);
            if fraction == 0 {
                assert_eq!(hits, whole, "seed {seed}");
            } else {
                assert!(hits == whole || hits == whole + 1, "seed {seed}");
            }
        }
    }
    eprintln!("salvos checked: {count}");
    assert!(count > 10_000);
}

#[test]
fn each_ship_ends_with_800_rounds_less_the_gun_shots_it_fired() {
    for (run, p) in batch().runs.iter().zip(parsed_runs()) {
        for ship in 1..=6u32 {
            let shots: Fixed = p
                .salvos
                .iter()
                .filter(|s| s.firer == ship && s.weapon == Weapon::Gun)
                .fold(Fixed::ZERO, |sum, s| sum + s.shots);
            let end = &run.ships[&ship];
            assert_eq!(end.rounds, fx(800) - shots, "seed {} ship {ship}", run.seed);
            assert!(end.rounds >= Fixed::ZERO);
            // Torpedoes are gone exactly when the ship fired them.
            let torpedoed = p
                .salvos
                .iter()
                .any(|s| s.firer == ship && s.weapon == Weapon::Torpedo);
            assert_eq!(
                end.torpedoes_ready, !torpedoed,
                "seed {} ship {ship}",
                run.seed
            );
        }
    }
}

// ---------------------------------------------------------------------
// Hits over 1,000 runs.
// ---------------------------------------------------------------------

/// Checks (H - E)^2 <= 16 x the sum of f(1 - f), in raw millionths.
fn salvo_sums_hold(salvos: &[&SalvoRec]) -> (i128, i128, i128, bool) {
    let mut hits = 0i128;
    let mut expected = 0i128;
    let mut variance = 0i128;
    for salvo in salvos {
        hits += i128::from(salvo.hits) * 1_000_000;
        expected += i128::from(salvo.expected.raw());
        let fraction = i128::from(salvo.expected.raw() % 1_000_000);
        variance += fraction * (1_000_000 - fraction);
    }
    let off = hits - expected;
    (hits, expected, variance, off * off <= 16 * variance)
}

#[test]
fn hits_and_where_they_land_stay_within_four_sigma() {
    let all: Vec<&SalvoRec> = parsed_runs().iter().flat_map(|p| p.salvos.iter()).collect();
    let guns: Vec<&SalvoRec> = all
        .iter()
        .copied()
        .filter(|s| s.weapon == Weapon::Gun)
        .collect();
    let torpedoes: Vec<&SalvoRec> = all
        .iter()
        .copied()
        .filter(|s| s.weapon == Weapon::Torpedo)
        .collect();
    for (name, salvos) in [("gun", &guns), ("torpedo", &torpedoes)] {
        let (hits, expected, variance, holds) = salvo_sums_hold(salvos);
        eprintln!(
            "{name} salvos {}: hits {} expected {} (millionths: H {hits}, E {expected}, sum f(1-f) {variance})",
            salvos.len(),
            hits / 1_000_000,
            Fixed::from_raw(i64::try_from(expected).expect("fits"))
        );
        assert!(holds, "{name} hits are outside four sigma");
    }

    // Where the hits land. Hits are counted from the damage events and tied
    // back to the weapon of the salvo that caused them.
    let mut gun_hits = 0i64;
    let mut by_track = [0i64; 3];
    let mut torpedo_hits = 0i64;
    let mut torpedo_on_flotation = 0i64;
    for p in parsed_runs() {
        let weapon_of: BTreeMap<u32, (Weapon, u32)> = p
            .salvos
            .iter()
            .map(|s| (s.event, (s.weapon, s.hits)))
            .collect();
        for damage in &p.damages {
            let (weapon, _) = weapon_of[&damage.cause];
            let hits = i64::from(damage.hits);
            match weapon {
                Weapon::Gun => {
                    gun_hits += hits;
                    by_track[match damage.track {
                        DamageTrack::Flotation => 0,
                        DamageTrack::System => 1,
                        DamageTrack::Fire => 2,
                    }] += hits;
                }
                Weapon::Torpedo => {
                    torpedo_hits += hits;
                    if damage.track == DamageTrack::Flotation {
                        torpedo_on_flotation += hits;
                    }
                }
            }
        }
        // Each salvo's hits are all accounted for on the tracks.
        for salvo in &p.salvos {
            let landed: u32 = p
                .damages
                .iter()
                .filter(|d| d.cause == salvo.event)
                .map(|d| d.hits)
                .sum();
            assert_eq!(landed, salvo.hits, "seed {} salvo #{}", p.seed, salvo.event);
        }
    }
    let salvo_gun_hits: i64 = guns.iter().map(|s| i64::from(s.hits)).sum();
    let salvo_torpedo_hits: i64 = torpedoes.iter().map(|s| i64::from(s.hits)).sum();
    assert_eq!(gun_hits, salvo_gun_hits);
    assert_eq!(torpedo_hits, salvo_torpedo_hits);
    eprintln!(
        "gun hits {gun_hits}: flotation {}, system {}, fire {}; torpedo hits {torpedo_hits}, on flotation {torpedo_on_flotation}",
        by_track[0], by_track[1], by_track[2]
    );
    assert_eq!(by_track.iter().sum::<i64>(), gun_hits);
    for (name, k, p) in [
        ("flotation", by_track[0], 500_000),
        ("system", by_track[1], 300_000),
        ("fire", by_track[2], 200_000),
    ] {
        assert!(
            within_four_sigma(k, gun_hits, p),
            "{name}: {k} of {gun_hits}"
        );
    }
    assert_eq!(torpedo_on_flotation, torpedo_hits);
}

// ---------------------------------------------------------------------
// Damage, sinking, matching.
// ---------------------------------------------------------------------

#[test]
fn damage_adds_up_and_ships_sink_exactly_when_flotation_reaches_100() {
    for (run, p) in batch().runs.iter().zip(parsed_runs()) {
        let seed = p.seed;
        let w = walk(p);
        let end = &w.last().end;

        // Each damage event: its cause is a salvo at that ship, its points
        // are hits x the damage of one hit, and its value_after is the
        // running total, capped at 100.
        let weapon_of: BTreeMap<u32, &SalvoRec> = p.salvos.iter().map(|s| (s.event, s)).collect();
        for (index, damage) in p.damages.iter().enumerate() {
            let salvo = weapon_of[&damage.cause];
            assert_eq!(salvo.target, damage.ship, "seed {seed}");
            assert_eq!(salvo.round, damage.round, "seed {seed}");
            let per_hit = match salvo.weapon {
                Weapon::Gun => GUN_HIT_POINTS,
                Weapon::Torpedo => {
                    assert_eq!(damage.track, DamageTrack::Flotation);
                    TORPEDO_HIT_POINTS
                }
            };
            assert!(damage.hits > 0);
            assert_eq!(damage.points, fx(per_hit * i64::from(damage.hits)));
            assert_eq!(damage.after, w.after[index], "seed {seed} damage {index}");
            assert!(damage.after >= Fixed::ZERO && damage.after <= fx(100));
        }

        // Final tracks equal the model's: min(100, sum of points).
        for ship in 1..=6u32 {
            let ship_end = &run.ships[&ship];
            for (track, model, name) in [
                (ship_end.flotation, &end.flotation, "flotation"),
                (ship_end.system, &end.system, "system"),
                (ship_end.fire, &end.fire, "fire"),
            ] {
                let points: i64 = p
                    .damages
                    .iter()
                    .filter(|d| d.ship == ship && d.track.name() == name)
                    .map(|d| d.points.raw())
                    .sum();
                let capped = points.min(100_000_000);
                assert_eq!(track.raw(), capped, "seed {seed} ship {ship} {name}");
                assert_eq!(track, model[&ship], "seed {seed} ship {ship} {name}");
            }
            // Sunk exactly when flotation reached 100.
            let sunk = ship_end.flotation == fx(100);
            assert_eq!(
                ship_end.location == ShipLocation::Sunk,
                sunk,
                "seed {seed} ship {ship}"
            );
            assert_eq!(w.sunk_round.contains_key(&ship), sunk);
            let in_a_force = run.forces.values().any(|f| f.ships.contains(&ship));
            assert_eq!(in_a_force, !sunk, "seed {seed} ship {ship}");
        }

        // The ShipSunk events: one per sunk ship, in the round its flotation
        // reached 100, caused by the damage event that took it there.
        assert_eq!(p.sunk.len(), w.sunk_round.len(), "seed {seed}");
        for sunk in &p.sunk {
            assert_eq!(
                w.sunk_round.get(&sunk.ship),
                Some(&sunk.round),
                "seed {seed}"
            );
            assert_eq!(sunk.force, force_of_ship(sunk.ship));
            let event = run
                .events
                .get(usize::try_from(sunk.cause - 1).expect("small"))
                .expect("the cause exists");
            match &event.kind {
                EventKind::ShipDamaged {
                    ship,
                    track,
                    value_after,
                    round,
                    ..
                } => {
                    assert_eq!(ship.raw(), sunk.ship);
                    assert_eq!(*track, DamageTrack::Flotation);
                    assert_eq!(*value_after, fx(100));
                    assert_eq!(*round, sunk.round);
                }
                other => panic!("seed {seed}: a ship sank because of {other:?}"),
            }
        }

        // Nothing fires from or at a ship in a round after it sank, and
        // nothing is damaged after.
        for salvo in &p.salvos {
            for ship in [salvo.firer, salvo.target] {
                if let Some(round) = w.sunk_round.get(&ship) {
                    assert!(salvo.round <= *round, "seed {seed}: ship {ship}");
                }
            }
        }
        for damage in &p.damages {
            if let Some(round) = w.sunk_round.get(&damage.ship) {
                assert!(damage.round <= *round, "seed {seed}");
            }
        }
        for salvo in &p.salvos {
            let st = &w.rounds[usize::try_from(salvo.round - 1).expect("small")];
            assert!(st.afloat[force_of_ship(salvo.firer)].contains(&salvo.firer));
            assert!(st.afloat[force_of_ship(salvo.target)].contains(&salvo.target));
        }
    }
}

#[test]
fn ships_are_matched_in_id_order_among_those_afloat() {
    for p in parsed_runs() {
        let seed = p.seed;
        let w = walk(p);
        let mut fired_before: BTreeSet<u32> = BTreeSet::new();
        for st in &w.rounds {
            let salvos: Vec<&SalvoRec> = p.salvos.iter().filter(|s| s.round == st.round).collect();

            // Order: lower force first, ship ID order, guns before torpedoes.
            let keys: Vec<(usize, u32, u8)> = salvos
                .iter()
                .map(|s| {
                    (
                        force_of_ship(s.firer),
                        s.firer,
                        u8::from(s.weapon == Weapon::Torpedo),
                    )
                })
                .collect();
            let mut sorted = keys.clone();
            sorted.sort_unstable();
            assert_eq!(keys, sorted, "seed {seed} round {}", st.round);

            for f in 0..2 {
                let own = &st.afloat[f];
                let enemy = &st.afloat[1 - f];
                let firers: BTreeSet<u32> = salvos
                    .iter()
                    .filter(|s| force_of_ship(s.firer) == f)
                    .map(|s| s.firer)
                    .collect();
                if !st.sighted[f] {
                    assert!(firers.is_empty(), "seed {seed}: fires before it sighted");
                    continue;
                }
                for (index, ship) in own.iter().enumerate() {
                    let target = enemy[index % enemy.len()];
                    let guns: Vec<&&SalvoRec> = salvos
                        .iter()
                        .filter(|s| s.firer == *ship && s.weapon == Weapon::Gun)
                        .collect();
                    let torpedoes: Vec<&&SalvoRec> = salvos
                        .iter()
                        .filter(|s| s.firer == *ship && s.weapon == Weapon::Torpedo)
                        .collect();
                    // A gun salvo exists unless system damage is 100.
                    let silent = st.start.system[ship] == fx(100);
                    assert_eq!(guns.len(), usize::from(!silent), "seed {seed} ship {ship}");
                    // Torpedoes go in the first round the ship fires.
                    let first = !fired_before.contains(ship);
                    assert_eq!(
                        torpedoes.len(),
                        usize::from(first),
                        "seed {seed} ship {ship}"
                    );
                    for salvo in guns.iter().chain(torpedoes.iter()) {
                        assert_eq!(
                            salvo.target, target,
                            "seed {seed} round {} ship {ship}",
                            st.round
                        );
                    }
                }
                fired_before.extend(own.iter().copied());
                // Only ships afloat at the start fired.
                assert!(firers.iter().all(|s| own.contains(s)));
            }
        }
    }
}

// ---------------------------------------------------------------------
// Ending.
// ---------------------------------------------------------------------

/// Ships of a force sunk or crippled at the end of a round, out of the
/// three it began with.
fn lost(tracks: &Tracks, force: usize) -> i64 {
    let count = SHIPS[force]
        .iter()
        .filter(|ship| tracks.flotation[ship] >= fx(CRIPPLED))
        .count();
    i64::try_from(count).expect("small")
}

/// The end the rules give, found by replaying the rounds: the round, the
/// reason and the forces that break off. Written out again here from
/// decision 14 and the hand calculation, using only the model's tracks.
fn expected_end(p: &Parsed, w: &Walk) -> Option<(u32, EngagementEndReason, Vec<usize>)> {
    let ever_sighted = p.rolls.iter().any(|r| r.sighted) || !p.salvos.is_empty();
    for st in &w.rounds {
        // (1) A force with no ship afloat.
        if st.end.afloat(0).is_empty() || st.end.afloat(1).is_empty() {
            return Some((st.round, EngagementEndReason::ForceSunk, Vec::new()));
        }
        // (2) Break-off: at least half the three ships sunk or crippled.
        let broke: Vec<usize> = (0..2)
            .filter(|f| i128::from(lost(&st.end, *f)) * 1_000_000 >= 500_000 * 3)
            .collect();
        if !broke.is_empty() {
            return Some((st.round, EngagementEndReason::BrokeOff, broke));
        }
        // (3) The last round.
        if st.round == MAX_ROUNDS {
            let reason = if ever_sighted {
                EngagementEndReason::MaxRounds
            } else {
                EngagementEndReason::PassedUnseen
            };
            return Some((st.round, reason, Vec::new()));
        }
    }
    None
}

#[test]
fn each_fight_ends_in_the_round_and_for_the_reason_the_rules_give() {
    for p in parsed_runs() {
        let seed = p.seed;
        let w = walk(p);
        let (round, reason, broke) =
            expected_end(p, &w).unwrap_or_else(|| panic!("seed {seed}: the model never ends"));
        assert_eq!(p.ended.round, round, "seed {seed}");
        assert_eq!(p.ended.reason, reason, "seed {seed}");
        assert_eq!(p.ended.hex, MEETING);
        assert_eq!(p.slipped, 0, "nobody avoids in this scenario");

        // Nothing happens in a round after the end: every rolled, fired or
        // damaged round is within the fight.
        for roll in &p.rolls {
            assert!(roll.round <= round, "seed {seed}");
        }

        // The ForceBrokeOff events name exactly the forces the model says,
        // in the last round, with the share of ships lost.
        let named: Vec<usize> = p.broke_off.iter().map(|(_, f, _)| *f).collect();
        assert_eq!(named, broke, "seed {seed}");
        for (event_round, force, share) in &p.broke_off {
            assert_eq!(*event_round, round);
            assert_eq!(
                *share,
                Fixed::from_ratio(lost(&w.last().end, *force), 3),
                "seed {seed}"
            );
        }

        // ForcesPassedUnseen: only in a fight that passes unseen, in round 6.
        let unseen = reason == EngagementEndReason::PassedUnseen;
        assert_eq!(
            p.passed_unseen,
            if unseen { vec![MAX_ROUNDS] } else { vec![] }
        );
        if unseen {
            assert!(p.salvos.is_empty());
            assert!(p.rolls.iter().all(|r| !r.sighted));
            assert_eq!(p.rolls.len(), 12, "two forces rolling in six rounds");
        }
    }
}

// ---------------------------------------------------------------------
// Reports and claims.
// ---------------------------------------------------------------------

#[test]
fn reports_and_claims_say_what_the_fight_says() {
    let base = load();
    let figures = base
        .world
        .ref_data()
        .search
        .clone()
        .expect("search figures are loaded");
    let cruiser_row: BTreeSet<ShipKind> = figures.kind_confusion[&ShipKind::Cruiser]
        .keys()
        .copied()
        .collect();
    assert_eq!(
        cruiser_row,
        BTreeSet::from([
            ShipKind::Cruiser,
            ShipKind::Battleship,
            ShipKind::Destroyer,
            ShipKind::Carrier
        ])
    );
    let mut outside_two_to_four = 0;
    for (run, p) in batch().runs.iter().zip(parsed_runs()) {
        let seed = p.seed;
        let w = walk(p);
        let last = w.last();

        // Order after the fight: for force A then B, the sighting report if
        // it sighted, then the claim if it fired.
        let mut expected_order: Vec<(bool, Side)> = Vec::new();
        for (f, side) in SIDES.iter().enumerate() {
            if last.sighted[f] || last.fired_on[f] {
                expected_order.push((false, *side));
            }
            if p.salvos.iter().any(|s| force_of_ship(s.firer) == f) {
                expected_order.push((true, *side));
            }
        }
        assert_eq!(p.after_fight, expected_order, "seed {seed}");

        for (f, side) in SIDES.iter().copied().enumerate() {
            let enemy = 1 - f;
            let reports: Vec<&ReportRec> = p.reports.iter().filter(|r| r.side == side).collect();
            let contacts = &run.contacts[&side];
            assert_eq!(
                reports.len(),
                usize::from(last.sighted[f] || last.fired_on[f]),
                "seed {seed}"
            );
            assert_eq!(contacts.len(), reports.len(), "seed {seed}");

            // Belief on the other side holds nothing from this side's report.
            if let Some(report) = reports.first() {
                let afloat = last.end.afloat(enemy).len();
                let truth = if afloat == 0 { 3 } else { afloat };
                let truth = u32::try_from(truth).expect("small");
                let allowed = [truth.saturating_sub(1).max(1), truth, truth + 1];
                assert!(
                    allowed.contains(&report.count),
                    "seed {seed}: {}",
                    report.count
                );
                if !(2..=4).contains(&report.count) {
                    outside_two_to_four += 1;
                }
                assert!(
                    report.hex.distance(MEETING) <= 1,
                    "seed {seed}: report at {:?}",
                    report.hex
                );
                assert!(cruiser_row.contains(&report.kind), "seed {seed}");
                assert_eq!(report.confidence, figures.initial_confidence);
                assert!(report.created);
                assert_eq!(report.cause, p.ended.event);

                // The contact is that report, aged by the two pulses left
                // in the day (confidence 0.8 x 0.8 x 0.8).
                let contact = &contacts[0];
                assert_eq!(contact.id, report.contact);
                assert_eq!(contact.hex, report.hex);
                assert_eq!(contact.count, report.count);
                assert_eq!(contact.kind, report.kind);
                assert_eq!(contact.last_event, report.event);
                assert_eq!(contact.age, 2);
                assert_eq!(contact.confidence, millionths(512_000));
            }

            // Claims: a force that fired claims the enemy ships that sank,
            // and those left afloat with fire damage of 30 or more.
            let fired = p.salvos.iter().any(|s| force_of_ship(s.firer) == f);
            let claims: Vec<&ClaimRec> = p.claims.iter().filter(|c| c.side == side).collect();
            assert_eq!(claims.len(), usize::from(fired), "seed {seed}");
            let seen = &run.claims[&side];
            assert_eq!(seen.len(), claims.len(), "seed {seed}");
            if let Some(claim) = claims.first() {
                let sunk = 3 - last.end.afloat(enemy).len();
                let burning = last
                    .end
                    .afloat(enemy)
                    .iter()
                    .filter(|ship| last.end.fire[ship] >= fx(CLAIMED_FIRE))
                    .count();
                let expected = u32::try_from(sunk + burning).expect("small");
                assert_eq!(claim.claimed_sunk, expected, "seed {seed}");
                assert!(claim.claimed_sunk >= u32::try_from(sunk).expect("small"));
                assert_eq!(claim.hex, MEETING);
                assert_eq!(claim.cause, p.ended.event);
                // Through the side's view.
                assert_eq!(
                    seen[0],
                    ClaimEnd {
                        hex: MEETING,
                        day: 1,
                        pulse: Pulse::Night,
                        claimed_sunk: expected,
                        event: claim.event,
                    }
                );
                // The truth: ships really sunk, from the ships' own state.
                let truly_sunk = ships_sunk(run, enemy);
                assert!(i64::from(claim.claimed_sunk) >= truly_sunk, "seed {seed}");
            }
        }
    }
    eprintln!("reports whose count is outside 2 to 4: {outside_two_to_four}");
}

// ---------------------------------------------------------------------
// Alertness.
// ---------------------------------------------------------------------

#[test]
fn force_b_is_alert_when_it_sighted_or_was_fired_on() {
    let (mut alert_runs, mut unseen_runs) = (0, 0);
    for (run, p) in batch().runs.iter().zip(parsed_runs()) {
        let seed = p.seed;
        let w = walk(p);
        // A is alert from the start and stays so.
        assert!(run.forces[&1].alert, "seed {seed}");
        let b_sighted = p.rolls.iter().any(|r| r.force == 1 && r.sighted);
        let b_fired_on = p.salvos.iter().any(|s| force_of_ship(s.target) == 1);
        let expected = b_sighted || b_fired_on;
        assert_eq!(run.forces[&2].alert, expected, "seed {seed}");
        if p.ended.reason == EngagementEndReason::PassedUnseen {
            assert!(!run.forces[&2].alert, "seed {seed}");
            unseen_runs += 1;
        }
        alert_runs += i64::from(expected);

        // One ForceBecameAlert for B, in the first round it sighted or was
        // fired on, and none for A.
        let first = w
            .rounds
            .iter()
            .find(|st| {
                st.fired_on[1]
                    || p.rolls
                        .iter()
                        .any(|r| r.round == st.round && r.force == 1 && r.sighted)
            })
            .map(|st| st.round);
        let expected_events: Vec<(u32, usize)> =
            first.map(|round| (round, 1)).into_iter().collect();
        assert_eq!(p.alerts, expected_events, "seed {seed}");
    }
    eprintln!("runs in which B ends alert: {alert_runs}, passed unseen (not alert): {unseen_runs}");
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn firing_first_pays() {
    let (mut a_free, mut b_free) = ([0i64; 2], [0i64; 2]);
    for (run, p) in batch().runs.iter().zip(parsed_runs()) {
        let damage = [total_flotation(run, 0), total_flotation(run, 1)];
        match opening(p) {
            Opening::AFreeRound => {
                a_free[0] += damage[0];
                a_free[1] += damage[1];
            }
            Opening::BFreeRound => {
                b_free[0] += damage[0];
                b_free[1] += damage[1];
            }
            _ => {}
        }
    }
    eprintln!(
        "A free round: flotation damage to A {}, to B {}; B free round: to A {}, to B {}",
        Fixed::from_raw(a_free[0]),
        Fixed::from_raw(a_free[1]),
        Fixed::from_raw(b_free[0]),
        Fixed::from_raw(b_free[1])
    );
    assert!(a_free[1] > a_free[0], "A's free rounds damage B more");
    assert!(b_free[0] > b_free[1], "B's free rounds damage A more");
}

/// Copies the data into a folder under the build directory, makes each
/// exact, asserted-unique text replacement in the named copied file, and
/// loads the copied scenario. Nothing in the repository's data folder is
/// touched. Each edit is (file under data/, original text, changed text).
fn variant(tag: &str, edits: &[(&str, &str, &str)]) -> Scenario {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data");
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("night_action_{tag}"));
    for folder in ["scenarios", "map", "reference"] {
        fs::create_dir_all(root.join(folder)).expect("make the variant folder");
    }
    for file in [
        "scenarios/04a_night_action.toml",
        "map/placeholder_night_action.toml",
        "reference/ship_classes.csv",
        "reference/port_sizes.csv",
        "reference/aircraft_types.csv",
        "reference/search.toml",
        "reference/surface_combat.toml",
    ] {
        let mut text = fs::read_to_string(data.join(file)).expect("read a data file");
        for (target, original, changed) in edits.iter().filter(|(target, ..)| *target == file) {
            assert_eq!(
                text.matches(original).count(),
                1,
                "{target}: the text {original:?} appears exactly once"
            );
            text = text.replace(original, changed);
        }
        fs::write(root.join(file), text).expect("write a variant file");
    }
    for (target, ..) in edits {
        assert!(
            [
                "scenarios/04a_night_action.toml",
                "reference/ship_classes.csv",
                "reference/surface_combat.toml"
            ]
            .contains(target),
            "unknown file {target}"
        );
    }
    Scenario::load(&root.join("scenarios/04a_night_action.toml")).expect("the variant loads")
}

const CLASSES: &str = "reference/ship_classes.csv";
const COMBAT: &str = "reference/surface_combat.toml";
const SCENARIO: &str = "scenarios/04a_night_action.toml";
const CRUISER_ROW: &str = "1,Placeholder cruiser,cruiser,16,32,2000,8000,0,8,3,100,20,8,200,400,0";

/// The cruiser row with another gun count, gun damage and radar flag.
fn cruiser_row(guns: u32, gun_damage: u32, radar: u32) -> String {
    format!(
        "1,Placeholder cruiser,cruiser,16,32,2000,8000,0,{guns},3,100,{gun_damage},8,200,400,{radar}"
    )
}

fn run_events(scenario: &Scenario, seed: u64) -> Vec<Event> {
    let mut run = scenario.with_seed(seed).expect("a seed can be set");
    run.run_to_end().expect("the variant runs");
    run.world.events().iter().cloned().collect()
}

/// The first seed in 1 to 1000 in which both forces sight in round 1, so
/// both fire in round 1 at an unsurprised target that is not yet lit.
fn both_sight_in_round_one() -> u64 {
    parsed_runs()
        .iter()
        .find(|p| {
            [0, 1].iter().all(|f| {
                p.rolls
                    .iter()
                    .any(|r| r.round == 1 && r.force == *f && r.sighted)
            })
        })
        .map(|p| p.seed)
        .expect("some seed has both sighting in round 1")
}

#[test]
fn more_guns_never_mean_fewer_expected_hits() {
    let seed = both_sight_in_round_one();
    let mut found = Vec::new();
    for guns in [4u32, 8, 12] {
        let row = cruiser_row(guns, 20, 0);
        let scenario = variant(&format!("guns_{guns}"), &[(CLASSES, CRUISER_ROW, &row)]);
        let events = run_events(&scenario, seed);
        let p = parse(seed, &events);
        // Round 1: every cruiser fires at an unsurprised, unlit target.
        let round_one: Vec<&SalvoRec> = p
            .salvos
            .iter()
            .filter(|s| s.round == 1 && s.weapon == Weapon::Gun)
            .collect();
        assert_eq!(round_one.len(), 6);
        for salvo in &round_one {
            assert_eq!(salvo.factor("surprised_factor"), Fixed::ONE);
            assert_eq!(salvo.factor("lit_factor"), Fixed::ONE);
            assert_eq!(salvo.factor("system_damage"), Fixed::ZERO);
            assert_eq!(salvo.factor("guns"), fx(i64::from(guns)));
            found.push((guns, salvo.expected));
        }
    }
    let expected = |guns: u32| -> Vec<Fixed> {
        found
            .iter()
            .filter(|(g, _)| *g == guns)
            .map(|(_, e)| *e)
            .collect()
    };
    assert_eq!(expected(4), vec![millionths(1_800_000); 6]);
    assert_eq!(expected(8), vec![millionths(3_600_000); 6]);
    assert_eq!(expected(12), vec![millionths(5_400_000); 6]);
}

#[test]
fn system_damage_never_means_more_expected_hits() {
    // From the 1,000 runs: every gun salvo from a ship with 0, 25 or 50
    // system damage at an unsurprised, unlit target.
    let mut seen: BTreeMap<i64, BTreeSet<Fixed>> = BTreeMap::new();
    let mut all_pairs: BTreeSet<(i64, Fixed)> = BTreeSet::new();
    for p in parsed_runs() {
        for salvo in p.salvos.iter().filter(|s| s.weapon == Weapon::Gun) {
            if salvo.factor("surprised_factor") == Fixed::ONE
                && salvo.factor("lit_factor") == Fixed::ONE
            {
                let system = salvo.factor("system_damage").trunc();
                seen.entry(system).or_default().insert(salvo.expected);
                all_pairs.insert((system, salvo.expected));
            }
        }
    }
    for (system, expected) in [(0, 3_600_000), (25, 2_700_000), (50, 1_800_000)] {
        assert_eq!(
            seen.get(&system),
            Some(&BTreeSet::from([millionths(expected)])),
            "system damage {system}"
        );
    }
    // Never more with more damage: the expected hits fall as damage rises.
    let pairs: Vec<(i64, Fixed)> = all_pairs.into_iter().collect();
    assert!(pairs.windows(2).all(|w| w[0].0 < w[1].0 && w[0].1 > w[1].1));

    // At 100 system damage a ship has no shots, so no gun salvo. A variant
    // puts every gun hit on the system track at 25 points a hit, so ships
    // reach 100 in a round or two.
    let mut at_100 = 0;
    let mut silent_checked = 0;
    for seed in 1..=60u64 {
        let scenario = variant(
            "all_system",
            &[
                (CLASSES, CRUISER_ROW, &cruiser_row(8, 100, 0)),
                (
                    COMBAT,
                    "gun_hit_flotation = \"0.5\"",
                    "gun_hit_flotation = \"0\"",
                ),
                (COMBAT, "gun_hit_system = \"0.3\"", "gun_hit_system = \"1\""),
                (COMBAT, "gun_hit_fire = \"0.2\"", "gun_hit_fire = \"0\""),
            ],
        );
        let events = run_events(&scenario, seed);
        let p = parse(seed, &events);
        let w = walk(&p);
        for st in &w.rounds {
            for ship in 1..=6u32 {
                let system = st.start.system[&ship];
                let guns = p
                    .salvos
                    .iter()
                    .filter(|s| s.round == st.round && s.firer == ship && s.weapon == Weapon::Gun)
                    .count();
                if system == fx(100) {
                    at_100 += 1;
                    if st.afloat[force_of_ship(ship)].contains(&ship)
                        && st.sighted[force_of_ship(ship)]
                    {
                        silent_checked += 1;
                        assert_eq!(guns, 0, "seed {seed}: ship {ship} at 100 fires guns");
                    }
                }
            }
        }
        for salvo in p.salvos.iter().filter(|s| s.weapon == Weapon::Gun) {
            let system = salvo.factor("system_damage");
            assert_eq!(
                salvo.shots,
                fx(120).mul_div(fx(100) - system, fx(100)),
                "seed {seed}"
            );
        }
    }
    assert!(
        at_100 > 0 && silent_checked > 0,
        "no ship reached 100 system damage"
    );
}

#[test]
fn an_alert_force_never_sights_less_readily() {
    let seed = both_sight_in_round_one();
    // (A not alert, B not alert, radar) -> round 1 chances of A and B.
    let a_not_alert = (
        "q = 0\nr = 0\nmission = \"surface_combat\"",
        "q = 0\nr = 0\nmission = \"surface_combat\"\nalert = false",
    );
    let b_alert = ("alert = false", "alert = true");
    let mut table: Vec<(bool, bool, [Fixed; 2])> = Vec::new();
    for (flipped, radar) in [(false, false), (true, false), (false, true), (true, true)] {
        let mut edits: Vec<(&str, &str, &str)> = Vec::new();
        let row = cruiser_row(8, 20, 1);
        if flipped {
            // B first: A's edit adds a second `alert = false` to the text.
            edits.push((SCENARIO, b_alert.0, b_alert.1));
            edits.push((SCENARIO, a_not_alert.0, a_not_alert.1));
        }
        if radar {
            edits.push((CLASSES, CRUISER_ROW, &row));
        }
        let scenario = variant(&format!("sight_{flipped}_{radar}"), &edits);
        let events = run_events(&scenario, seed);
        let p = parse(seed, &events);
        let chance = |force: usize| {
            p.rolls
                .iter()
                .find(|r| r.round == 1 && r.force == force)
                .expect("every force rolls in round 1")
                .chance
        };
        table.push((flipped, radar, [chance(0), chance(1)]));
    }
    // Radar adds 0.2 to the base, then not being alert halves the chance.
    assert_eq!(
        table,
        vec![
            (false, false, [millionths(500_000), millionths(250_000)]),
            (true, false, [millionths(250_000), millionths(500_000)]),
            (false, true, [millionths(700_000), millionths(350_000)]),
            (true, true, [millionths(350_000), millionths(700_000)]),
        ]
    );
    for (_, _, [a, b]) in &table {
        assert!(a != b);
    }
}

#[test]
fn no_ammunition_is_negative_no_track_leaves_0_to_100_and_every_ship_has_one_place() {
    for (run, p) in batch().runs.iter().zip(parsed_runs()) {
        let seed = run.seed;
        for (ship, end) in &run.ships {
            assert!(
                end.rounds >= Fixed::ZERO && end.rounds <= fx(800),
                "seed {seed}"
            );
            for track in [end.flotation, end.system, end.fire] {
                assert!(track >= Fixed::ZERO && track <= fx(100), "seed {seed}");
            }
            let homes: Vec<u32> = run
                .forces
                .iter()
                .filter(|(_, force)| force.ships.contains(ship))
                .map(|(id, _)| *id)
                .collect();
            match end.location {
                ShipLocation::TaskForce(task_force) => {
                    assert_eq!(homes, vec![task_force.raw()], "seed {seed} ship {ship}");
                    assert_eq!(task_force.raw(), if *ship <= 3 { 1 } else { 2 });
                }
                ShipLocation::Sunk => assert!(homes.is_empty(), "seed {seed} ship {ship}"),
                ShipLocation::Port(_) => panic!("seed {seed}: ship {ship} in port"),
            }
            assert_eq!(end.side, SIDES[force_of_ship(*ship)]);
        }
        // Along the way: the model never holds a track outside 0 to 100 or
        // ammunition below 0.
        let w = walk(p);
        for st in &w.rounds {
            for tracks in [&st.start, &st.end] {
                for map in [&tracks.flotation, &tracks.system, &tracks.fire] {
                    assert!(map.values().all(|v| *v >= Fixed::ZERO && *v <= fx(100)));
                }
            }
        }
        // Belief holds nothing the reports did not say: each contact traces
        // to a report and each claim to a claim event.
        for side in Side::ALL {
            for contact in &run.contacts[&side] {
                let report = p
                    .reports
                    .iter()
                    .find(|r| r.event == contact.last_event)
                    .expect("a contact comes from a report");
                assert_eq!(
                    (
                        report.side,
                        report.contact,
                        report.hex,
                        report.count,
                        report.kind
                    ),
                    (side, contact.id, contact.hex, contact.count, contact.kind)
                );
            }
            for claim in &run.claims[&side] {
                let sent = p
                    .claims
                    .iter()
                    .find(|c| c.event == claim.event)
                    .expect("a claim comes from a claim event");
                assert_eq!(
                    (sent.side, sent.hex, sent.claimed_sunk),
                    (side, claim.hex, claim.claimed_sunk)
                );
            }
        }
    }
}

#[test]
fn a_replay_reproduces_the_daily_hash() {
    let base = load();
    let mut all_hashes: Vec<Vec<u64>> = Vec::new();
    for seed in [1u64, 2, 77, 1942] {
        let hashes: Vec<Vec<u64>> = (0..2)
            .map(|_| {
                let mut run = base.with_seed(seed).expect("a seed can be set");
                run.run_to_end().expect("the run resolves");
                run.world.day_hashes().to_vec()
            })
            .collect();
        assert_eq!(hashes[0].len(), 1);
        assert_eq!(hashes[0], hashes[1], "seed {seed}");
        all_hashes.push(hashes[0].clone());
    }
    // The seed matters: different seeds do not all give the same history.
    assert!(all_hashes.windows(2).any(|pair| pair[0] != pair[1]));

    // The file's own seed gives the same hashes as a seeded copy.
    let mut own = load();
    own.run_to_end().expect("the run resolves");
    let mut copy = base.with_seed(1942).expect("a seed can be set");
    copy.run_to_end().expect("the run resolves");
    assert_eq!(own.world.day_hashes(), copy.world.day_hashes());
}

// ---------------------------------------------------------------------
// What the designer needs to see: run with --nocapture.
// ---------------------------------------------------------------------

#[test]
fn print_the_summary() {
    let batch = batch();
    let parsed = parsed_runs();
    let runs = i64::try_from(batch.runs.len()).expect("small");
    eprintln!("== Scenario 4a summary over seeds 1 to {SEEDS} ==");
    eprintln!("the {runs} runs took {:?} in this build", batch.elapsed);

    // The six counts.
    let mut a1 = 0;
    let mut b1 = 0;
    let mut ways = [0i64; 4];
    for p in parsed {
        a1 += i64::from(
            p.rolls
                .iter()
                .any(|r| r.round == 1 && r.force == 0 && r.sighted),
        );
        b1 += i64::from(
            p.rolls
                .iter()
                .any(|r| r.round == 1 && r.force == 1 && r.sighted),
        );
        ways[match opening(p) {
            Opening::AFreeRound => 0,
            Opening::BFreeRound => 1,
            Opening::BothFire => 2,
            Opening::PassUnseen => 3,
        }] += 1;
    }
    eprintln!("A sights in round 1: {a1} (437..=563)");
    eprintln!("B sights in round 1: {b1} (196..=304)");
    eprintln!("A has a free round: {} (537..=660)", ways[0]);
    eprintln!("B has a free round: {} (149..=249)", ways[1]);
    eprintln!("both fire from the start: {} (149..=249)", ways[2]);
    eprintln!("pass unseen: {} (0..=9)", ways[3]);

    // Reason by round.
    let reasons = [
        EngagementEndReason::ForceSunk,
        EngagementEndReason::BrokeOff,
        EngagementEndReason::MaxRounds,
        EngagementEndReason::PassedUnseen,
        EngagementEndReason::SlippedAway,
    ];
    eprintln!("how the fights ended, by reason and round:");
    eprintln!("  {:<24} r1   r2   r3   r4   r5   r6  total", "reason");
    for reason in reasons {
        let mut by_round = [0i64; 6];
        for p in parsed.iter().filter(|p| p.ended.reason == reason) {
            by_round[usize::try_from(p.ended.round - 1).expect("small")] += 1;
        }
        eprintln!(
            "  {:<24} {:>3}  {:>3}  {:>3}  {:>3}  {:>3}  {:>3}  {:>5}",
            reason.name(),
            by_round[0],
            by_round[1],
            by_round[2],
            by_round[3],
            by_round[4],
            by_round[5],
            by_round.iter().sum::<i64>()
        );
    }

    // Ships sunk per side, runs with a sinking.
    for (f, side) in SIDES.iter().enumerate() {
        let sunk: i64 = batch.runs.iter().map(|r| ships_sunk(r, f)).sum();
        let with_sinking = batch.runs.iter().filter(|r| ships_sunk(r, f) > 0).count();
        eprintln!(
            "ships sunk, force {} ({}): {sunk} in {with_sinking} runs",
            if f == 0 { "A" } else { "B" },
            side.name()
        );
    }

    // Average flotation damage per side by how the fight opened.
    for (name, way) in [
        ("A free round", Opening::AFreeRound),
        ("B free round", Opening::BFreeRound),
        ("both fire from the start", Opening::BothFire),
    ] {
        let members: Vec<&RunData> = batch
            .runs
            .iter()
            .zip(parsed)
            .filter(|(_, p)| opening(p) == way)
            .map(|(r, _)| r)
            .collect();
        let n = i64::try_from(members.len()).expect("small");
        let totals = [0, 1].map(|f| members.iter().map(|r| total_flotation(r, f)).sum::<i64>());
        eprintln!(
            "average flotation damage, {name} ({n} runs): force A {} / {n} = {}, force B {} / {n} = {}",
            Fixed::from_raw(totals[0]),
            Fixed::from_raw(totals[0] / n),
            Fixed::from_raw(totals[1]),
            Fixed::from_raw(totals[1] / n)
        );
    }

    // One seed where both fired from the start and the fight lasted 3+ rounds.
    let example = parsed
        .iter()
        .find(|p| opening(p) == Opening::BothFire && p.ended.round >= 3);
    match example {
        Some(p) => eprintln!(
            "example: seed {} - both fired from the start, the fight ended in round {} ({})",
            p.seed,
            p.ended.round,
            p.ended.reason.name()
        ),
        None => eprintln!("example: no such seed"),
    }

    // Claims and reports.
    let mut claims = [0i64; 2];
    let mut truth = [0i64; 2];
    let mut over = 0;
    let mut burning_claimed = 0i64;
    let mut sighted_not_reported = 0;
    let mut reports = [0i64; 2];
    let mut contacts_at_end = 0;
    for (run, p) in batch.runs.iter().zip(parsed) {
        let w = walk(p);
        let last = w.last();
        for f in 0..2 {
            let enemy = 1 - f;
            for claim in p.claims.iter().filter(|c| c.side == SIDES[f]) {
                claims[f] += i64::from(claim.claimed_sunk);
                truth[f] += ships_sunk(run, enemy);
                over += i64::from(i64::from(claim.claimed_sunk) > ships_sunk(run, enemy));
                burning_claimed += i64::try_from(
                    last.end
                        .afloat(enemy)
                        .iter()
                        .filter(|s| last.end.fire[s] >= fx(CLAIMED_FIRE))
                        .count(),
                )
                .expect("small");
            }
            reports[f] += i64::try_from(p.reports.iter().filter(|r| r.side == SIDES[f]).count())
                .expect("small");
            let fired_on = p.salvos.iter().any(|s| force_of_ship(s.target) == f);
            let rolled_success = p.rolls.iter().any(|r| r.force == f && r.sighted);
            if (fired_on || rolled_success) && !last.sighted[f] {
                sighted_not_reported += 1;
            }
            contacts_at_end += run.contacts[&SIDES[f]].len();
        }
    }
    eprintln!(
        "claims: A claimed {} sunk (truth {}), B claimed {} sunk (truth {}); runs where a claim exceeded the truth: {over}",
        claims[0], truth[0], claims[1], truth[1]
    );
    eprintln!("enemy ships claimed only for fire damage of 30 or more: {burning_claimed}");
    eprintln!(
        "sighting reports: A sent {}, B sent {}; contacts held at the end: {contacts_at_end}",
        reports[0], reports[1]
    );
    eprintln!(
        "force-runs fired on but with no round left to sight in, so no report: {sighted_not_reported}"
    );
    let surprised_rounds: usize = parsed.iter().map(|p| p.surprised.len()).sum();
    let salvos: usize = parsed.iter().map(|p| p.salvos.len()).sum();
    let sunk_total: usize = parsed.iter().map(|p| p.sunk.len()).sum();
    eprintln!("salvos {salvos}, surprised forces {surprised_rounds}, ships sunk {sunk_total}");
}
