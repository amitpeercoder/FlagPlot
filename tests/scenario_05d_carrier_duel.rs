//! Scenario 5d: two carrier forces search for and strike each other.
//!
//! Stage 5d of scenario 5. The two forces are alike. It proves that both
//! sides search, decide and strike in the same pulse, each on its own
//! belief, and that both strikes are launched before either lands. It adds
//! no rule: the rules are decisions 0012, 0025, 0026 and 0027.
//!
//! The data is placeholder data: see `data/scenarios/05d_carrier_duel.toml`.
//!
//! # Hand calculation
//!
//! Written by the architect before the test code. The tests below assert
//! these numbers. If the code disagrees, the code or the rule is wrong: do
//! not change a number here to make a test pass.
//!
//! ## The setting
//!
//! | | Allied, task force 1 at (4,0) | Japanese, task force 2 at (9,0) |
//! | --- | --- | --- |
//! | Carrier | ship 1 | ship 4 |
//! | Cruisers | ships 2 and 3 | ships 5 and 6 |
//! | Scouts, 8 | air group 1, arc 040 to 130 | air group 4, arc 220 to 310 |
//! | Dive bombers, 18 | air group 2 | air group 5 |
//! | Fighters, 12, patrol share 0.5 | air group 3 | air group 6 |
//!
//! Nothing moves. The forces are 200 nm apart, bearing 090 one way and 270
//! the other, inside each side's arc. Each force has 20 + 8 + 8 = 36
//! anti-aircraft shots while its three ships are afloat. The scenario is two
//! days: four daylight pulses.
//!
//! ## Each side on its own
//!
//! In every daylight pulse each side's scouts sight the other force with
//! chance 0.509295, as in scenario 5b, while its carrier's flight deck
//! works: with the deck degraded four scouts fly and the chance is
//! 0.254647, and with it out none fly. Each side's rolls are its own.
//!
//! A side's bombers launch in a pulse if, and only if, that side holds a
//! contact reported in that pulse or the one before and its flight deck lets
//! at least one bomber fly. (The flight deck was left out of these two
//! paragraphs when they were first written. On the first morning every deck
//! works, so the counts below stand. No chance is given for an afternoon
//! launch: it depends on what the morning did to the decks.) A report comes from a
//! search sighting or from a strike that attacked, and a strike attacks only
//! in a pulse in which it was launched. So, for a side whose deck lets it
//! launch:
//!
//! - In a morning pulse a side launches if, and only if, its own search
//!   sighted that morning. A contact from the afternoon before is too old.
//! - In an afternoon pulse it launches if, and only if, its own search
//!   sighted that afternoon, or its own strike attacked that morning. A
//!   morning strike that found nothing dropped the contact it was sent at
//!   (decision 0028; before it, a morning sighting was enough).
//! - Being struck gives a side no contact. A side that has sighted nothing
//!   launches nothing, however hard it is hit.
//!
//! A morning strike flies to the hex its side's morning report named, which
//! is the right hex with chance 0.7. So on day 1 each side attacks in the
//! morning with chance 0.509295 x 0.7 = 0.356506, and both do with chance
//! 0.356506 x 0.356506 = 0.127096.
//!
//! | Count of runs, over seeds 1 to 1000        | p        | Mean   | Std dev | Allowed    |
//! | ------------------------------------------ | -------- | ------ | ------- | ---------- |
//! | The Allied side attacks on day 1, morning  | 0.356506 | 356.51 | 15.15   | 296 to 417 |
//! | The Japanese side attacks on day 1, morning | 0.356506 | 356.51 | 15.15  | 296 to 417 |
//! | Both attack on day 1, morning              | 0.127096 | 127.10 | 10.53   | 85 to 169  |
//!
//! ## A first attack
//!
//! While every group is whole and every flight deck works, an attack by
//! either side runs like this. The attacker sends 18 bombers and 6 escorts.
//! The defender has kept 6 fighters overhead.
//!
//! | Step                    | Shots | Chance | Expected | Std dev  | Lost |
//! | ----------------------- | ----- | ------ | -------- | -------- | ---- |
//! | Escorts at the patrol   | 6     | 0.3    | 1.8      | 1.122497 | 0 to 6 of the defender's fighters |
//! | Patrol at the escorts   | 6     | 0.3    | 1.8      | 1.122497 | 0 to 6 of the attacker's fighters |
//! | Patrol at the bombers   | none: the six escorts hold all six of the patrol | | | | |
//! | Guns at the bombers     | 36    | 0.05   | 1.8      | 1.307670 | 0 to 18 bombers |
//!
//! Each shot is rolled on its own (decision 0033). The bombers that attack
//! the carrier are 18 less the guns' hits, 16.2 on average, and score 0.2
//! of their number in hits on average, 3.24, at 10 points a hit. Over the
//! runs, the hits of each roll of these first attacks are within four
//! standard deviations of the expected figure times their number. (Before
//! decision 0033 each roll lost 1 or 2 and an attack scored 3 or 4 hits.)
//!
//! ## Both at once
//!
//! In a pulse in which both sides attack, each attack is worked from the
//! aircraft and flight decks as they stood at the start of the pulse. On day
//! 1 in the morning that is the table above for both: each side sends 18
//! bombers and 6 escorts and keeps 6 overhead, whatever the other's strike
//! does to it in that pulse. A fighter group can then lose aircraft twice,
//! its patrol to the enemy's escorts and its escorts to the enemy's patrol:
//! up to 12 in all, 3.6 on average.
//!
//! A flight deck hit in a pulse changes nothing in that pulse. From the next
//! pulse a degraded deck halves each group aboard, rounded down, and a deck
//! that is out stops them all: search, strike, escort and patrol alike.
//!
//! ## Later attacks
//!
//! Every later attack is worked by the rule of scenario 5c from what each
//! group has, and the state of both flight decks, at the start of its
//! pulse. The test works every attack of every run that way, for both
//! sides.
//!
//! A carrier has a toughness of 600, so a hit does 10 points; a cruiser 400,
//! so 15. A carrier can be sunk, on the first day too now that an attack
//! can score ten hits (decision 0033), though that is rare. If it is:
//!
//! - Every air group aboard it loses the aircraft that were aboard her, with
//!   an event citing the sinking: its ready aircraft less those it had in the
//!   air in that pulse (what it flew and did not lose), and its damaged
//!   aircraft. It flies nothing afterwards.
//! - The aircraft it had in the air come home by the rule of decisions 7 to
//!   10 of 0059: there is no other carrier and no base here, so they come
//!   down in the sea beside the cruisers (a ship of their side afloat in the
//!   hex). The same is done for a carrier whose flight deck goes out in the
//!   pulse: her aircraft in the air come down in the sea, and she is afloat
//!   with none, while a deck only degraded takes them. What a group may fly
//!   still changes from the next pulse only.
//! - A strike it launched in that pulse still attacks.
//! - The force is left with its cruisers: 16 anti-aircraft shots, and no
//!   patrol. A later strike on it attacks the lower-numbered cruiser afloat.
//!
//! # Properties
//!
//! - A strike is launched only at a contact its own side holds, reported in
//!   that pulse or the one before.
//! - No side's decision reads the other side's state: a run in which only
//!   one side ever sights has strikes by that side only.
//! - In every pulse, for both sides, the bombers launched, the escorts sent
//!   and the patrol kept follow from the aircraft and flight decks at the
//!   start of the pulse.
//! - Every group's aircraft at the end are its aircraft at the start less
//!   the aircraft its loss events record (a sunk ship's, and those that came
//!   down in the sea for want of a deck), less what landed elsewhere from
//!   it, plus what landed elsewhere to it, and none goes below zero.
//! - No aircraft lands elsewhere or comes down for want of a deck unless its
//!   group's ship sank or lost her deck in that pulse; none ever for a group
//!   at a base; none in a pulse with no air combat step.
//! - Every damage event is caused by an attack, every attack by a launch,
//!   and every launch by a report to the launching side.
//! - A replay of any seed reproduces the daily hash.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{AirFirer, AirTarget, Cause, DamageTrack, Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, ContactId, EntityRef, EventId, OrderId, ShipId, TaskForceId};
use sim_core::rng::{Draw, RngKey, RngStream, Subsystem};
use sim_core::state::{AirGroupLocation, ShipLocation, ShipSystem, Side, SystemState};
use sim_core::time::Pulse;
use sim_data::Scenario;

const SEEDS: u64 = 1000;

/// One side's force: its ships, groups, the orders that set them, and where
/// it lies.
struct Camp {
    side: Side,
    force: TaskForceId,
    carrier: ShipId,
    cruisers: [ShipId; 2],
    scouts: AirGroupId,
    bombers: AirGroupId,
    fighters: AirGroupId,
    scout_orders: [u32; 2],
    bomber_order: u32,
    escort_orders: [u32; 2],
    arc: (u32, u32),
    home: Hex,
}

const ALLIED: usize = 0;
const JAPANESE: usize = 1;

const CAMPS: [Camp; 2] = [
    Camp {
        side: Side::Allied,
        force: TaskForceId::new(1),
        carrier: ShipId::new(1),
        cruisers: [ShipId::new(2), ShipId::new(3)],
        scouts: AirGroupId::new(1),
        bombers: AirGroupId::new(2),
        fighters: AirGroupId::new(3),
        scout_orders: [1, 2],
        bomber_order: 3,
        escort_orders: [4, 5],
        arc: (40, 130),
        home: Hex::new(4, 0),
    },
    Camp {
        side: Side::Japanese,
        force: TaskForceId::new(2),
        carrier: ShipId::new(4),
        cruisers: [ShipId::new(5), ShipId::new(6)],
        scouts: AirGroupId::new(4),
        bombers: AirGroupId::new(5),
        fighters: AirGroupId::new(6),
        scout_orders: [6, 7],
        bomber_order: 8,
        escort_orders: [9, 10],
        arc: (220, 310),
        home: Hex::new(9, 0),
    },
];

/// The four daylight pulses of the two days.
const DAYLIGHT: [(u32, Pulse); 4] = [
    (1, Pulse::Morning),
    (1, Pulse::Afternoon),
    (2, Pulse::Morning),
    (2, Pulse::Afternoon),
];

/// The chance one scout group of 8 sights the other force at 200 nm.
const SEARCH_CHANCE: i64 = 509_295;

/// The ships, groups and bases of the scenario as loaded, for the model of
/// where aircraft go when their deck is lost.
#[allow(clippy::type_complexity)]
fn no_deck_facts() -> &'static (
    Vec<common::no_deck::ModelShip>,
    Vec<common::no_deck::ModelGroup>,
    Vec<common::no_deck::ModelBase>,
) {
    static FACTS: OnceLock<(
        Vec<common::no_deck::ModelShip>,
        Vec<common::no_deck::ModelGroup>,
        Vec<common::no_deck::ModelBase>,
    )> = OnceLock::new();
    FACTS.get_or_init(|| common::no_deck::initial(&load().world))
}

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/05d_carrier_duel.toml")
}

fn load() -> Scenario {
    common::load(&scenario_path()).expect("scenario 5d loads")
}

fn millionths(value: i64) -> Fixed {
    Fixed::from_raw(value)
}

fn tenths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 10)
}

fn hundredths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 100)
}

fn int(value: u32) -> Fixed {
    Fixed::from_int(i64::from(value))
}

fn count(value: usize) -> i64 {
    i64::try_from(value).expect("small")
}

/// Whether k out of n is within four standard deviations of probability
/// `p` (in millionths), in exact integer arithmetic.
fn within_four_sigma(k: i64, n: i64, p: i64) -> bool {
    let (k, n, p) = (i128::from(k), i128::from(n), i128::from(p));
    let scale = 1_000_000;
    let off = k * scale - n * p;
    off * off <= 16 * n * p * (scale - p)
}

/// Flyable aircraft of a group aboard a carrier by the state of its deck:
/// all, half rounded down, none.
fn flyable(ready: u32, deck: SystemState) -> u32 {
    match deck {
        SystemState::Working => ready,
        SystemState::Degraded => ready / 2,
        SystemState::Out => 0,
    }
}

/// The hits a roll of `shots` at `chance` gives from its stream: one draw for
/// each shot (decision 0033).
fn hits_from(stream: &mut RngStream, shots: u32, chance: Fixed) -> u32 {
    (0..shots).map(|_| u32::from(stream.chance(chance))).sum()
}

fn stream_of(seed: u64, day: u32, pulse: Pulse, entity: EntityRef, other: EntityRef) -> RngStream {
    RngKey {
        seed,
        day,
        pulse: Some(pulse),
        subsystem: Subsystem::AirCombat,
        entity,
        other: Some(other),
        draw: Draw::Legacy,
    }
    .stream()
}

fn shots_text(shots: u32, chance: &str, expected: Fixed, hits: u32) -> String {
    format!(
        "{shots} shots x {chance} = {expected} expected, {hits} {}",
        if hits == 1 { "hit" } else { "hits" }
    )
}

/// Index 0 to 3 of a daylight pulse.
fn daylight_index(event: &Event) -> usize {
    DAYLIGHT
        .iter()
        .position(|&(day, pulse)| day == event.day && Some(pulse) == event.pulse)
        .expect("a daylight pulse")
}

// ---------------------------------------------------------------------
// What one run did, read from its events and final state.
// ---------------------------------------------------------------------

struct Final {
    flotation: Fixed,
    fire: Fixed,
    sunk: bool,
    systems: [SystemState; 5],
}

struct Run {
    seed: u64,
    events: Vec<Event>,
    hashes: Vec<u64>,
    ships: BTreeMap<u32, Final>,
    /// The aircraft ready at the end of each of the six groups.
    ready: BTreeMap<AirGroupId, u32>,
    positions: [Hex; 2],
}

impl Run {
    fn of(&self, keep: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|event| keep(&event.kind))
            .collect()
    }

    fn ship(&self, id: ShipId) -> &Final {
        &self.ships[&id.raw()]
    }

    fn attacks_by(&self, camp: usize) -> Vec<&Event> {
        self.of(|k| matches!(k, EventKind::AirAttack { air_group, .. } if *air_group == CAMPS[camp].bombers))
    }
}

fn run_of(base: &Scenario, seed: u64) -> Run {
    let mut run = base.with_seed(seed).expect("a seed can be set");
    run.run_to_end().expect("the run resolves");
    digest(&run, seed)
}

fn digest(run: &Scenario, seed: u64) -> Run {
    let truth = run.world.umpire();
    Run {
        seed,
        // The claims from the air (decision 0056) are not part of what this
        // scenario works out: they come after each raid is resolved and are
        // asserted in scenario_11_claims.rs.
        events: run
            .world
            .events()
            .iter()
            .filter(|e| {
                !matches!(
                    e.kind,
                    sim_core::events::EventKind::ClaimReceived { from_air: true, .. }
                )
            })
            .cloned()
            .collect(),
        hashes: run.world.day_hashes().to_vec(),
        ships: truth
            .ships
            .values()
            .map(|ship| {
                (
                    ship.id.raw(),
                    Final {
                        flotation: ship.flotation_damage,
                        fire: ship.fire_damage,
                        sunk: ship.location == ShipLocation::Sunk,
                        systems: ShipSystem::ALL.map(|system| ship.systems.get(system)),
                    },
                )
            })
            .collect(),
        ready: truth
            .air_groups
            .values()
            .map(|group| (group.id, group.ready))
            .collect(),
        positions: [
            truth.task_forces[&CAMPS[ALLIED].force].position(),
            truth.task_forces[&CAMPS[JAPANESE].force].position(),
        ],
    }
}

fn batch() -> &'static Vec<Run> {
    static BATCH: OnceLock<Vec<Run>> = OnceLock::new();
    BATCH.get_or_init(|| {
        let base = load();
        (1..=SEEDS).map(|seed| run_of(&base, seed)).collect()
    })
}

/// Every run worked by the rule, asserting every event on the way.
fn worked() -> &'static Vec<Worked> {
    static WORKED: OnceLock<Vec<Worked>> = OnceLock::new();
    WORKED.get_or_init(|| batch().iter().map(work_the_run).collect())
}

// ---------------------------------------------------------------------
// Working every pulse of a run from the state at the start of the pulse.
// ---------------------------------------------------------------------

/// A report a side received: the pulse index, whether a search made it, its
/// event, its contact and its hex.
type Held = (usize, bool, EventId, ContactId, Hex);

/// One raid's numbers, as the rule works them.
#[derive(Clone, Debug)]
struct AttackNumbers {
    /// B: the bombers launched. E: the escorts sent. P: the patrol the
    /// defender held over the hex.
    bombers: u32,
    escorts: u32,
    patrol: u32,
    /// The hits of each roll, or `None` for a roll with no shots.
    escorts_at_patrol: Option<u32>,
    patrol_at_escorts: Option<u32>,
    patrol_at_bombers: Option<u32>,
    shots_at_bombers: u32,
    gun_shots: u32,
    guns: Option<u32>,
    bombers_attacking: u32,
    hits: u32,
    ship: ShipId,
}

/// What one daylight pulse of one run did, worked from the start of it.
#[derive(Clone, Debug)]
struct PulseWork {
    index: usize,
    /// The state at the start of the pulse.
    ready: BTreeMap<AirGroupId, u32>,
    deck: [SystemState; 2],
    afloat: BTreeSet<ShipId>,
    /// Whether each side's own search sighted in this pulse.
    sighted: [bool; 2],
    /// Aircraft each side's scouts flew.
    searched: [Option<u32>; 2],
    /// Whether each side held a contact reported in this pulse or the one
    /// before.
    eligible: [bool; 2],
    /// Bombers each side launched.
    launched: [Option<u32>; 2],
    /// Escorts each side sent.
    escorts_sent: [Option<u32>; 2],
    /// Whether each side's strike found the other force.
    found: [bool; 2],
    /// Each side's attack, if its strike attacked.
    attack: [Option<AttackNumbers>; 2],
    /// Aircraft lost in the pulse by group, and the loss events that
    /// recorded them.
    lost: BTreeMap<AirGroupId, u32>,
    loss_events: BTreeMap<AirGroupId, u32>,
    /// Ships sunk in the pulse.
    sunk: Vec<ShipId>,
}

struct Worked {
    seed: u64,
    pulses: Vec<PulseWork>,
}

fn system_of(
    systems: &BTreeMap<(ShipId, ShipSystem), SystemState>,
    ship: ShipId,
    system: ShipSystem,
) -> SystemState {
    systems
        .get(&(ship, system))
        .copied()
        .unwrap_or(SystemState::Working)
}

/// Asserts one fighters' event against the rule.
#[allow(clippy::too_many_arguments)]
fn check_fight(
    seed: u64,
    event: &Event,
    firer: AirGroupId,
    target: AirGroupId,
    firing: AirFirer,
    at: AirTarget,
    shots: u32,
    hits: u32,
    causes: Vec<Cause>,
) {
    let chance = tenths(3);
    let expected = int(shots) * chance;
    assert_eq!(
        event.kind,
        EventKind::AirFight {
            firer,
            target,
            firing,
            at,
            shots: int(shots),
            chance,
            expected,
            hits
        },
        "seed {seed}"
    );
    let who = match firing {
        AirFirer::Escorts => "escorts",
        AirFirer::Patrol => "patrol",
    };
    let what = match at {
        AirTarget::Patrol => "patrol",
        AirTarget::Escorts => "escorts",
        AirTarget::Bombers => "bombers",
    };
    assert_eq!(
        event.kind.to_string(),
        format!(
            "air_group:{} {who} at the {what} of air_group:{}: {}",
            firer.raw(),
            target.raw(),
            shots_text(shots, "0.3", expected, hits)
        ),
        "seed {seed}"
    );
    assert_eq!(event.causes, causes, "seed {seed}");
    let breakdown = event.breakdown.as_ref().expect("breakdown");
    assert_eq!(breakdown.result, expected);
    let factors: Vec<(&str, Fixed)> = breakdown
        .factors
        .iter()
        .map(|f| (f.name, f.value))
        .collect();
    let (share_factors, rest) = if firing == AirFirer::Patrol {
        factors.split_at(4)
    } else {
        factors.split_at(0)
    };
    assert_eq!(
        rest,
        [("shots", int(shots)), ("hit_chance", chance)],
        "seed {seed}"
    );
    if firing == AirFirer::Patrol {
        // Each side sends one raid, so it meets the whole patrol (decision
        // 0033, 7): the share is the patrol, and the raid is all the raids.
        let names: Vec<&str> = share_factors.iter().map(|f| f.0).collect();
        assert_eq!(
            names,
            [
                "patrol",
                "raid_aircraft",
                "all_raids_aircraft",
                "patrol_share"
            ]
        );
        assert_eq!(share_factors[0].1, share_factors[3].1, "seed {seed}");
        assert_eq!(share_factors[1].1, share_factors[2].1, "seed {seed}");
        assert!(share_factors[3].1 >= int(shots), "seed {seed}");
    }
}

/// Works every daylight pulse of `run` by the rule of the hand calculation
/// from the aircraft and flight decks at the start of the pulse, asserts every
/// event of the pulse against it, and returns what was worked. State is carried
/// from pulse to pulse from the events of the pulses before.
#[allow(clippy::too_many_lines)]
fn work_the_run(run: &Run) -> Worked {
    let seed = run.seed;
    let mut ready: BTreeMap<AirGroupId, u32> = BTreeMap::new();
    for camp in &CAMPS {
        ready.insert(camp.scouts, 8);
        ready.insert(camp.bombers, 18);
        ready.insert(camp.fighters, 12);
    }
    let mut systems: BTreeMap<(ShipId, ShipSystem), SystemState> = BTreeMap::new();
    let mut afloat: BTreeSet<ShipId> = (1..=6).map(ShipId::new).collect();
    let mut damage: BTreeMap<ShipId, (Fixed, Fixed)> = (1..=6)
        .map(|id| (ShipId::new(id), (Fixed::ZERO, Fixed::ZERO)))
        .collect();
    // Reports each side received: (pulse index, by search, event, contact, hex).
    let mut held: [Vec<Held>; 2] = [Vec::new(), Vec::new()];
    let mut pulses: Vec<PulseWork> = Vec::new();
    let flight_deck = ShipSystem::FlightDeck;

    for (index, &(day, pulse)) in DAYLIGHT.iter().enumerate() {
        let evs: Vec<&Event> = run
            .events
            .iter()
            .filter(|event| event.day == day && event.pulse == Some(pulse))
            .collect();
        let ready0 = ready.clone();
        let afloat0 = afloat.clone();
        let deck0 = [
            system_of(&systems, CAMPS[ALLIED].carrier, flight_deck),
            system_of(&systems, CAMPS[JAPANESE].carrier, flight_deck),
        ];
        // The events that say each side's groups flew, for the causes of the
        // homecoming (decisions 7 to 10 of 0059).
        let mut flown_events: [Option<EventId>; 2] = [None; 2];
        let mut sent_events: [Option<EventId>; 2] = [None; 2];
        let mut work = PulseWork {
            index,
            ready: ready0.clone(),
            deck: deck0,
            afloat: afloat0.clone(),
            sighted: [false; 2],
            searched: [None; 2],
            eligible: [false; 2],
            launched: [None; 2],
            escorts_sent: [None; 2],
            found: [false; 2],
            attack: [None, None],
            lost: BTreeMap::new(),
            loss_events: BTreeMap::new(),
            sunk: Vec::new(),
        };
        // Every event of the pulse must be one of the kinds worked below.
        for event in &evs {
            assert!(
                matches!(
                    event.kind,
                    EventKind::SearchFlown { .. }
                        | EventKind::SearchAttempt { .. }
                        | EventKind::ReportReceived { .. }
                        | EventKind::StrikeLaunched { .. }
                        | EventKind::EscortsSent { .. }
                        | EventKind::AirFight { .. }
                        | EventKind::AntiAircraftFire { .. }
                        | EventKind::AircraftLost { .. }
                        | EventKind::AirAttack { .. }
                        | EventKind::StrikeFoundNothing { .. }
                        | EventKind::ContactNotThere { .. }
                        | EventKind::ShipDamaged { .. }
                        | EventKind::SystemHit { .. }
                        | EventKind::ShipSunk { .. }
                        | EventKind::AirGroupLost { .. }
                        | EventKind::AircraftLandedElsewhere { .. }
                        | EventKind::AircraftDitchedNoDeck { .. }
                        | EventKind::ContactAged { .. }
                ),
                "seed {seed}: unexpected event {}",
                event.kind
            );
        }

        // ---- The search: each side's scouts, on their own. ----
        for (c, camp) in CAMPS.iter().enumerate() {
            let enemy = &CAMPS[1 - c];
            let scouts_fly = flyable(ready0[&camp.scouts], deck0[c]);
            let flown: Vec<&Event> = evs
                .iter()
                .copied()
                .filter(|e| matches!(e.kind, EventKind::SearchFlown { air_group, .. } if air_group == camp.scouts))
                .collect();
            let attempts: Vec<&Event> = evs
                .iter()
                .copied()
                .filter(|e| matches!(e.kind, EventKind::SearchAttempt { air_group, .. } if air_group == camp.scouts))
                .collect();
            if scouts_fly == 0 {
                assert!(flown.is_empty() && attempts.is_empty(), "seed {seed}");
                continue;
            }
            work.searched[c] = Some(scouts_fly);
            assert_eq!(flown.len(), 1, "seed {seed}");
            flown_events[c] = Some(flown[0].id);
            assert_eq!(
                flown[0].kind,
                EventKind::SearchFlown {
                    air_group: camp.scouts,
                    from: AirGroupLocation::Ship(camp.carrier),
                    hex: camp.home,
                    from_deg: camp.arc.0,
                    to_deg: camp.arc.1,
                    range_nm: Fixed::from_int(300),
                    aircraft: scouts_fly,
                },
                "seed {seed} pulse {index}"
            );
            assert_eq!(
                flown[0].causes,
                vec![
                    Cause::Order(OrderId::for_umpire(
                        camp.scout_orders[0],
                        common::umpire_key()
                    )),
                    Cause::Order(OrderId::for_umpire(
                        camp.scout_orders[1],
                        common::umpire_key()
                    ))
                ]
            );
            assert_eq!(attempts.len(), 1, "seed {seed}");
            // Eight scouts sight with 0.509295; four, half of it.
            let EventKind::SearchAttempt {
                task_force,
                distance_nm,
                chance,
                sighted,
                ..
            } = attempts[0].kind
            else {
                unreachable!()
            };
            assert_eq!(task_force, enemy.force);
            assert_eq!(distance_nm, Fixed::from_int(200));
            assert_eq!(
                chance,
                millionths(SEARCH_CHANCE * i64::from(scouts_fly) / 8),
                "seed {seed} pulse {index}"
            );
            assert_eq!(attempts[0].causes, vec![Cause::Event(flown[0].id)]);
            let reports: Vec<&Event> = evs
                .iter()
                .copied()
                .filter(|e| {
                    matches!(e.kind, EventKind::ReportReceived { .. })
                        && e.causes == vec![Cause::Event(attempts[0].id)]
                })
                .collect();
            assert_eq!(reports.len(), usize::from(sighted), "seed {seed}");
            work.sighted[c] = sighted;
            if let Some(report) = reports.first() {
                let EventKind::ReportReceived {
                    side, contact, hex, ..
                } = report.kind
                else {
                    unreachable!()
                };
                assert_eq!(side, camp.side);
                held[c].push((index, true, report.id, contact, hex));
            }
        }

        // ---- The decisions: each side's bombers on its own reports. ----
        let launches: Vec<&Event> = evs
            .iter()
            .copied()
            .filter(|e| matches!(e.kind, EventKind::StrikeLaunched { .. }))
            .collect();
        let mut launch_of: [Option<&Event>; 2] = [None, None];
        for (c, camp) in CAMPS.iter().enumerate() {
            // A contact reported in this pulse (a search is before the
            // decision) or in the morning of the same day, when this is the
            // afternoon.
            let eligible: Vec<&Held> = held[c]
                .iter()
                .filter(|r| r.0 == index || (r.0 + 1 == index && index % 2 == 1))
                .collect();
            // The same, by the hand calculation: its own search sighted this
            // pulse, or, in an afternoon, its own strike attacked that
            // morning (one that found nothing dropped the contact, decision
            // 0028).
            let by_search = work.sighted[c]
                || (index % 2 == 1
                    && pulses
                        .last()
                        .is_some_and(|before| before.attack[c].is_some()));
            assert_eq!(
                !eligible.is_empty(),
                by_search,
                "seed {seed} pulse {index}: a side holds a contact to strike only from its own search or its own attack"
            );
            work.eligible[c] = !eligible.is_empty();
            let can_fly = flyable(ready0[&camp.bombers], deck0[c]);
            let capable = afloat0.contains(&camp.carrier) && can_fly > 0;
            let mine: Vec<&Event> = launches
                .iter()
                .copied()
                .filter(|e| matches!(e.kind, EventKind::StrikeLaunched { air_group, .. } if air_group == camp.bombers))
                .collect();
            assert_eq!(
                mine.len(),
                usize::from(work.eligible[c] && capable),
                "seed {seed} pulse {index}: side {c} launches if, and only if, it holds a contact and its bombers can fly"
            );
            let Some(launch) = mine.first().copied() else {
                continue;
            };
            let EventKind::StrikeLaunched {
                contact,
                hex,
                aircraft,
                distance_nm,
                ..
            } = launch.kind
            else {
                unreachable!()
            };
            assert_eq!(aircraft, can_fly, "seed {seed}: all that can fly go");
            assert_eq!(
                distance_nm,
                Fixed::from_int(i64::from(camp.home.air_distance_nm(hex)))
            );
            assert!(distance_nm <= Fixed::from_int(250));
            assert_eq!(launch.causes.len(), 2, "seed {seed}");
            assert_eq!(
                launch.causes[0],
                Cause::Order(OrderId::for_umpire(camp.bomber_order, common::umpire_key()))
            );
            let Cause::Event(report_id) = launch.causes[1] else {
                panic!("seed {seed}: a launch cites a report");
            };
            assert!(
                eligible
                    .iter()
                    .any(|r| r.2 == report_id && r.3 == contact && r.4 == hex),
                "seed {seed}: the launch cites a report of its own side, this pulse or the one before"
            );
            work.launched[c] = Some(aircraft);
            launch_of[c] = Some(launch);
        }

        // ---- The air combat step, worked on the start-of-pulse state. ----
        let split = |c: usize| -> (u32, u32) {
            let fighters = CAMPS[c].fighters;
            if ready0[&fighters] == 0 {
                return (0, 0);
            }
            let can = flyable(ready0[&fighters], deck0[c]);
            let patrol = can / 2;
            (patrol, can - patrol)
        };
        let air: Vec<&Event> = evs
            .iter()
            .copied()
            .filter(|e| {
                matches!(
                    e.kind,
                    EventKind::EscortsSent { .. }
                        | EventKind::AirFight { .. }
                        | EventKind::AntiAircraftFire { .. }
                        | EventKind::AircraftLost { .. }
                )
            })
            .collect();
        let outs: Vec<&Event> = evs
            .iter()
            .copied()
            .filter(|e| {
                matches!(
                    e.kind,
                    EventKind::AirAttack { .. } | EventKind::StrikeFoundNothing { .. }
                )
            })
            .collect();
        // The order of the log: launches, then each raid's escorts, fights,
        // guns and losses, then each strike's finding.
        if let (Some(last_launch), Some(first_air)) =
            (launches.last(), air.first().or(outs.first()))
        {
            assert!(last_launch.id < first_air.id, "seed {seed}");
        }
        if let (Some(last_air), Some(first_out)) = (air.last(), outs.first()) {
            assert!(last_air.id < first_out.id, "seed {seed}");
        }
        let mut air_it = air.iter().copied();
        let mut now = ready0.clone();
        let mut loss_ids: BTreeMap<AirGroupId, Vec<EventId>> = BTreeMap::new();
        // The bombers each raid has left to attack with, and whether it found
        // the other force.
        let mut raid_left: [Option<(u32, bool)>; 2] = [None, None];
        let mut numbers: [Option<AttackNumbers>; 2] = [None, None];
        for (c, camp) in CAMPS.iter().enumerate() {
            let Some(launch) = launch_of[c] else {
                continue;
            };
            let d = 1 - c;
            let enemy = &CAMPS[d];
            let b = work.launched[c].expect("launched");
            let EventKind::StrikeLaunched { hex, .. } = launch.kind else {
                unreachable!()
            };
            let found = hex == enemy.home;
            work.found[c] = found;
            let (_, e) = split(c);
            let (p, _) = split(d);
            // The escorts go with the raid, whether or not it finds anything.
            let mut sent_id = None;
            if e > 0 {
                let sent = air_it.next().expect("escorts sent");
                assert_eq!(
                    sent.kind,
                    EventKind::EscortsSent {
                        air_group: camp.fighters,
                        with_air_group: camp.bombers,
                        aircraft: e
                    },
                    "seed {seed} pulse {index}"
                );
                assert_eq!(
                    sent.kind.to_string(),
                    format!(
                        "air_group:{} sent {e} escorts with air_group:{}",
                        camp.fighters.raw(),
                        camp.bombers.raw()
                    )
                );
                assert_eq!(
                    sent.causes,
                    vec![
                        Cause::Order(OrderId::for_umpire(
                            camp.escort_orders[0],
                            common::umpire_key()
                        )),
                        Cause::Order(OrderId::for_umpire(
                            camp.escort_orders[1],
                            common::umpire_key()
                        )),
                        Cause::Event(launch.id)
                    ],
                    "seed {seed}"
                );
                sent_id = Some(sent.id);
                sent_events[c] = Some(sent.id);
                work.escorts_sent[c] = Some(e);
            }
            if !found {
                raid_left[c] = Some((b, false));
                continue;
            }

            // The fights, on the counts at the start of the pulse. Each
            // roll: (event, the group that loses, the aircraft taken).
            let chance = tenths(3);
            let mut rolls: Vec<(EventId, AirGroupId, u32)> = Vec::new();
            let mut escorts_stream = stream_of(
                seed,
                day,
                pulse,
                EntityRef::AirGroup(camp.fighters),
                EntityRef::TaskForce(enemy.force),
            );
            let mut patrol_stream = stream_of(
                seed,
                day,
                pulse,
                EntityRef::AirGroup(enemy.fighters),
                EntityRef::AirGroup(camp.bombers),
            );
            let (mut e_p, mut p_e, mut p_b) = (None, None, None);
            if e > 0 && p > 0 {
                let hits = hits_from(&mut escorts_stream, e, chance);
                let event = air_it.next().expect("escorts at the patrol");
                check_fight(
                    seed,
                    event,
                    camp.fighters,
                    enemy.fighters,
                    AirFirer::Escorts,
                    AirTarget::Patrol,
                    e,
                    hits,
                    vec![
                        Cause::Event(launch.id),
                        Cause::Event(sent_id.expect("escorts")),
                    ],
                );
                e_p = Some(hits);
                rolls.push((event.id, enemy.fighters, hits.min(p)));
            }
            let at_escorts = p.min(e);
            if p > 0 && e > 0 {
                let hits = hits_from(&mut patrol_stream, at_escorts, chance);
                let event = air_it.next().expect("patrol at the escorts");
                check_fight(
                    seed,
                    event,
                    enemy.fighters,
                    camp.fighters,
                    AirFirer::Patrol,
                    AirTarget::Escorts,
                    at_escorts,
                    hits,
                    vec![
                        Cause::Event(launch.id),
                        Cause::Event(sent_id.expect("escorts")),
                    ],
                );
                p_e = Some(hits);
                rolls.push((event.id, camp.fighters, hits.min(e)));
            }
            let at_bombers = p - at_escorts;
            let mut bombers_left = b;
            if at_bombers > 0 {
                let hits = hits_from(&mut patrol_stream, at_bombers, chance);
                let event = air_it.next().expect("patrol at the bombers");
                check_fight(
                    seed,
                    event,
                    enemy.fighters,
                    camp.bombers,
                    AirFirer::Patrol,
                    AirTarget::Bombers,
                    at_bombers,
                    hits,
                    vec![Cause::Event(launch.id)],
                );
                p_b = Some(hits);
                let taken = hits.min(b);
                bombers_left -= taken;
                rolls.push((event.id, camp.bombers, taken));
            }

            // The guns of every ship afloat in the force, at the bombers left.
            let gun_shots: u32 = afloat0
                .iter()
                .map(|ship| {
                    if *ship == enemy.carrier {
                        20
                    } else if enemy.cruisers.contains(ship) {
                        8
                    } else {
                        0
                    }
                })
                .sum();
            let mut gun_hits = None;
            if gun_shots > 0 && bombers_left > 0 {
                let lowest = std::iter::once(enemy.carrier)
                    .chain(enemy.cruisers)
                    .find(|ship| afloat0.contains(ship))
                    .expect("a ship afloat");
                let mut stream = stream_of(
                    seed,
                    day,
                    pulse,
                    EntityRef::Ship(lowest),
                    EntityRef::AirGroup(camp.bombers),
                );
                let hits = hits_from(&mut stream, gun_shots, hundredths(5));
                let expected = int(gun_shots) * hundredths(5);
                let event = air_it.next().expect("the guns");
                assert_eq!(
                    event.kind,
                    EventKind::AntiAircraftFire {
                        task_force: enemy.force,
                        air_group: camp.bombers,
                        shots: int(gun_shots),
                        chance: hundredths(5),
                        expected,
                        hits,
                        after_attack: false,
                    },
                    "seed {seed} pulse {index}"
                );
                assert_eq!(
                    event.kind.to_string(),
                    format!(
                        "task_force:{} guns at the raid of air_group:{}: {}",
                        enemy.force.raw(),
                        camp.bombers.raw(),
                        shots_text(gun_shots, "0.05", expected, hits)
                    )
                );
                assert_eq!(event.causes, vec![Cause::Event(launch.id)]);
                assert_eq!(
                    event.breakdown.as_ref().expect("breakdown").result,
                    expected
                );
                gun_hits = Some(hits);
                let taken = hits.min(bombers_left);
                bombers_left -= taken;
                rolls.push((event.id, camp.bombers, taken));
            }

            // The losses, one per roll that cost aircraft, taken off what
            // each group has as they are recorded.
            for (cause, group, taken) in rolls {
                let lost = taken.min(now[&group]);
                if lost == 0 {
                    continue;
                }
                let remaining = now[&group] - lost;
                now.insert(group, remaining);
                let event = air_it.next().expect("a loss");
                assert_eq!(
                    event.kind,
                    EventKind::AircraftLost {
                        air_group: group,
                        aircraft: lost,
                        remaining
                    },
                    "seed {seed} pulse {index}"
                );
                assert_eq!(
                    event.kind.to_string(),
                    format!(
                        "air_group:{} lost {lost} aircraft, {remaining} left",
                        group.raw()
                    )
                );
                assert_eq!(event.causes, vec![Cause::Event(cause)], "seed {seed}");
                loss_ids.entry(group).or_default().push(event.id);
                *work.lost.entry(group).or_default() += lost;
                *work.loss_events.entry(group).or_default() += 1;
            }
            raid_left[c] = Some((bombers_left, true));

            // The target: the carrier while afloat, else the lower-numbered
            // cruiser afloat.
            let ship = std::iter::once(enemy.carrier)
                .chain(enemy.cruisers)
                .find(|ship| afloat0.contains(ship))
                .expect("a ship afloat");
            numbers[c] = Some(AttackNumbers {
                bombers: b,
                escorts: e,
                patrol: p,
                escorts_at_patrol: e_p,
                patrol_at_escorts: p_e,
                patrol_at_bombers: p_b,
                shots_at_bombers: at_bombers,
                gun_shots,
                guns: gun_hits,
                bombers_attacking: bombers_left,
                hits: 0,
                ship,
            });
        }
        assert!(
            air_it.next().is_none(),
            "seed {seed} pulse {index}: nothing else in the air"
        );

        // ---- What each strike found, in group order. ----
        let mut out_it = outs.iter().copied();
        let mut attack_ids: [Option<EventId>; 2] = [None, None];
        // The finding of each strike that found nothing, with the contact it
        // was sent at.
        let mut nothing_ids: [Option<(EventId, ContactId)>; 2] = [None, None];
        for (c, camp) in CAMPS.iter().enumerate() {
            let Some(launch) = launch_of[c] else {
                continue;
            };
            let EventKind::StrikeLaunched { hex, contact, .. } = launch.kind else {
                unreachable!()
            };
            let (left, found) = raid_left[c].expect("a raid");
            if !found {
                let event = out_it.next().expect("found nothing");
                assert_eq!(
                    event.kind,
                    EventKind::StrikeFoundNothing {
                        air_group: camp.bombers,
                        hex
                    },
                    "seed {seed}"
                );
                assert_eq!(
                    event.kind.to_string(),
                    format!("air_group:{} found nothing at {hex}", camp.bombers.raw())
                );
                assert_eq!(event.causes, vec![Cause::Event(launch.id)]);
                nothing_ids[c] = Some((event.id, contact));
                continue;
            }
            // Fighters and guns never take all the bombers.
            assert!(left > 0, "seed {seed} pulse {index}");
            let attack = numbers[c].as_mut().expect("worked");
            let enemy = &CAMPS[1 - c];
            let mut stream = stream_of(
                seed,
                day,
                pulse,
                EntityRef::AirGroup(camp.bombers),
                EntityRef::TaskForce(enemy.force),
            );
            let hits = hits_from(&mut stream, left, tenths(2));
            let expected = int(left) * tenths(2);
            let event = out_it.next().expect("an attack");
            assert_eq!(
                event.kind,
                EventKind::AirAttack {
                    air_group: camp.bombers,
                    task_force: enemy.force,
                    ship: attack.ship,
                    weapon: Weapon::Bomb,
                    shots: int(left),
                    chance: tenths(2),
                    expected,
                    hits
                },
                "seed {seed} pulse {index}"
            );
            assert_eq!(
                event.kind.to_string(),
                format!(
                    "air_group:{} bombs at ship:{}: {}",
                    camp.bombers.raw(),
                    attack.ship.raw(),
                    shots_text(left, "0.2", expected, hits)
                )
            );
            // It cites its launch and the losses of its own bombers.
            let mut causes = vec![Cause::Event(launch.id)];
            causes.extend(
                loss_ids
                    .get(&camp.bombers)
                    .into_iter()
                    .flatten()
                    .copied()
                    .map(Cause::Event),
            );
            assert_eq!(event.causes, causes, "seed {seed}");
            attack.hits = hits;
            attack_ids[c] = Some(event.id);
        }
        assert!(out_it.next().is_none(), "seed {seed}");

        // ---- Damage lands, and the reports go out, in group order. ----
        let last_out = outs.last().map(|e| e.id);
        let tail: Vec<&Event> = evs
            .iter()
            .copied()
            .filter(|e| {
                last_out.is_some_and(|last| e.id > last)
                    && matches!(
                        e.kind,
                        EventKind::ShipDamaged { .. }
                            | EventKind::SystemHit { .. }
                            | EventKind::ShipSunk { .. }
                            | EventKind::AirGroupLost { .. }
                            | EventKind::ReportReceived { .. }
                            | EventKind::ContactNotThere { .. }
                    )
            })
            .collect();
        let mut tail_it = tail.iter().copied().peekable();
        // The aircraft in the air at the end of the air combat step: what
        // each group flew less what it lost in the step (decision 7 of 0059).
        let mut in_air: BTreeMap<AirGroupId, u32> = BTreeMap::new();
        for (c, camp) in CAMPS.iter().enumerate() {
            let (patrol, _) = split(c);
            let escorts = work.escorts_sent[c].unwrap_or(0);
            let flew = [
                (camp.scouts, work.searched[c].unwrap_or(0)),
                (camp.bombers, work.launched[c].unwrap_or(0)),
                (camp.fighters, patrol + escorts),
            ];
            for (group, flown) in flew {
                in_air.insert(group, flown - (ready0[&group] - now[&group]));
            }
        }
        let mut deck_losses: Vec<common::no_deck::DeckLoss> = Vec::new();
        for (c, camp) in CAMPS.iter().enumerate() {
            // A strike that found nothing reports it, and its side drops the
            // contact it was sent at: only its own side's reports come
            // between, so the contact still names the hex.
            if let Some((nothing, contact)) = nothing_ids[c] {
                let launch = launch_of[c].expect("a launch");
                let EventKind::StrikeLaunched { hex, .. } = launch.kind else {
                    unreachable!()
                };
                let event = tail_it.next().expect("a contact dropped");
                assert_eq!(
                    event.kind,
                    EventKind::ContactNotThere {
                        side: camp.side,
                        contact,
                        hex
                    },
                    "seed {seed} pulse {index}"
                );
                assert_eq!(event.causes, vec![Cause::Event(nothing)], "seed {seed}");
                assert!(
                    held[c].iter().any(|r| r.3 == contact),
                    "seed {seed}: the side held the contact"
                );
                held[c].retain(|r| r.3 != contact);
                continue;
            }
            let Some(attack_id) = attack_ids[c] else {
                continue;
            };
            let attack = numbers[c].as_ref().expect("worked");
            let enemy = &CAMPS[1 - c];
            let ship = attack.ship;
            let hits = attack.hits;
            let per_hit = if ship == enemy.carrier {
                Fixed::from_int(10)
            } else {
                Fixed::from_int(15)
            };
            let mut total = 0;
            let mut last_track: Option<DamageTrack> = None;
            while let Some(event) = tail_it.peek().copied() {
                let EventKind::ShipDamaged {
                    round,
                    ship: damaged,
                    track,
                    hits: track_hits,
                    points,
                    value_after,
                } = event.kind
                else {
                    break;
                };
                assert_eq!(event.causes, vec![Cause::Event(attack_id)], "seed {seed}");
                tail_it.next();
                assert_eq!(round, 0, "damage from the air carries no round");
                assert_eq!(damaged, ship);
                assert!(track_hits > 0);
                // Flotation is recorded before fire.
                assert!(last_track.is_none_or(|before| (before as usize) < (track as usize)));
                last_track = Some(track);
                assert_eq!(points, per_hit * i64::from(track_hits), "seed {seed}");
                let (flotation, fire) = damage[&ship];
                let before = match track {
                    DamageTrack::Flotation => flotation,
                    DamageTrack::Fire => fire,
                };
                let after = (before + points).min(Fixed::from_int(100));
                assert_eq!(value_after, after, "seed {seed}");
                damage.insert(
                    ship,
                    match track {
                        DamageTrack::Flotation => (after, fire),
                        DamageTrack::Fire => (flotation, after),
                    },
                );
                total += track_hits;
                if track == DamageTrack::Flotation
                    && before < Fixed::from_int(100)
                    && after >= Fixed::from_int(100)
                {
                    // The ship sinks, and takes every air group aboard it.
                    let sunk = tail_it.next().expect("a sinking");
                    assert_eq!(
                        sunk.kind,
                        EventKind::ShipSunk {
                            round: 0,
                            ship,
                            task_force: enemy.force
                        },
                        "seed {seed}"
                    );
                    assert_eq!(sunk.causes, vec![Cause::Event(event.id)]);
                    afloat.remove(&ship);
                    work.sunk.push(ship);
                    deck_losses.push(common::no_deck::DeckLoss {
                        ship: ship.raw(),
                        hex: enemy.home,
                        cause: sunk.id,
                    });
                    if ship == enemy.carrier {
                        for group in [enemy.scouts, enemy.bombers, enemy.fighters] {
                            // Decision 10 of 0059: what was aboard goes down
                            // with her; the aircraft in the air are kept.
                            let aboard = now[&group] - in_air[&group];
                            let lost = tail_it.next().expect("an air group lost with its ship");
                            assert_eq!(
                                lost.kind,
                                EventKind::AirGroupLost {
                                    air_group: group,
                                    ship,
                                    aircraft: aboard
                                },
                                "seed {seed}"
                            );
                            assert_eq!(
                                lost.kind.to_string(),
                                format!(
                                    "air_group:{} lost with ship:{}: {} aircraft",
                                    group.raw(),
                                    ship.raw(),
                                    aboard
                                )
                            );
                            assert_eq!(lost.causes, vec![Cause::Event(sunk.id)]);
                            now.insert(group, in_air[&group]);
                        }
                    }
                }
            }
            assert_eq!(total, hits, "seed {seed}: every hit is on a track");
            // The systems hit, in hit order, each citing the attack.
            let mut system_hits = 0;
            while let Some(event) = tail_it.peek().copied() {
                let EventKind::SystemHit {
                    round,
                    ship: hit,
                    system,
                    before,
                    after,
                } = event.kind
                else {
                    break;
                };
                tail_it.next();
                assert_eq!(event.causes, vec![Cause::Event(attack_id)], "seed {seed}");
                assert_eq!(round, 0);
                assert_eq!(hit, ship);
                assert_eq!(before, system_of(&systems, ship, system), "seed {seed}");
                if system == ShipSystem::FlightDeck {
                    // A carrier's deck steps down; a cruiser has none.
                    if ship == enemy.carrier {
                        assert_eq!(after, before.step_down());
                    } else {
                        assert_eq!(after, before);
                    }
                } else {
                    assert!(after == before || after == before.step_down());
                }
                // (A hit on a ship already sunk puts no deck out: she has none.)
                if system == ShipSystem::FlightDeck
                    && after == SystemState::Out
                    && before != SystemState::Out
                    && afloat.contains(&ship)
                {
                    deck_losses.push(common::no_deck::DeckLoss {
                        ship: ship.raw(),
                        hex: enemy.home,
                        cause: event.id,
                    });
                }
                systems.insert((ship, system), after);
                system_hits += 1;
            }
            assert!(system_hits <= hits, "seed {seed}");
            // The report, citing the attack.
            let report = tail_it.next().expect("a report");
            let EventKind::ReportReceived {
                side, contact, hex, ..
            } = report.kind
            else {
                panic!("seed {seed}: expected the report, found {}", report.kind);
            };
            assert_eq!(side, camp.side);
            assert_eq!(report.causes, vec![Cause::Event(attack_id)]);
            held[c].push((index, false, report.id, contact, hex));
        }
        assert!(tail_it.next().is_none(), "seed {seed} pulse {index}");

        // ---- The aircraft whose deck is lost come home (decisions 7 to 10
        // of 0059): no other carrier and no base here, so they come down in
        // the sea, beside the cruisers if one is afloat. ----
        {
            let (mut ships, mut model_groups, bases) = no_deck_facts().clone();
            for ship in &mut ships {
                ship.afloat = afloat.contains(&ShipId::new(ship.id));
                ship.deck_out =
                    system_of(&systems, ShipId::new(ship.id), flight_deck) == SystemState::Out;
            }
            for group in &mut model_groups {
                let id = AirGroupId::new(group.id);
                group.ready = now[&id];
                group.in_air = in_air[&id];
            }
            let homing: Vec<&Event> = evs
                .iter()
                .copied()
                .filter(|e| {
                    matches!(
                        e.kind,
                        EventKind::AircraftLandedElsewhere { .. }
                            | EventKind::AircraftDitchedNoDeck { .. }
                    )
                })
                .collect();
            let expected =
                common::no_deck::homecoming(&ships, &mut model_groups, &bases, &deck_losses);
            assert_eq!(homing.len(), expected.len(), "seed {seed} pulse {index}");
            for (event, (home, closing)) in homing.iter().zip(&expected) {
                let source = AirGroupId::new(home.source());
                let camp = CAMPS
                    .iter()
                    .position(|camp| [camp.scouts, camp.bombers, camp.fighters].contains(&source))
                    .expect("a group of a camp");
                // The events that say the group flew, in the order its
                // sorties are noted (a patrol before its escorts).
                let mut causes: Vec<Cause> = closing.map(Cause::Event).into_iter().collect();
                let flew: Vec<Cause> = if source == CAMPS[camp].scouts {
                    flown_events[camp].map(Cause::Event).into_iter().collect()
                } else if source == CAMPS[camp].bombers {
                    launch_of[camp]
                        .map(|l| Cause::Event(l.id))
                        .into_iter()
                        .collect()
                } else {
                    let mut flew = Vec::new();
                    if split(camp).0 > 0 {
                        flew.push(Cause::Order(OrderId::for_umpire(
                            CAMPS[camp].escort_orders[0],
                            common::umpire_key(),
                        )));
                        flew.push(Cause::Order(OrderId::for_umpire(
                            CAMPS[camp].escort_orders[1],
                            common::umpire_key(),
                        )));
                    }
                    flew.extend(sent_events[camp].map(Cause::Event));
                    flew
                };
                for cause in flew {
                    if !causes.contains(&cause) {
                        causes.push(cause);
                    }
                }
                let kind = match *home {
                    common::no_deck::Home::Landed {
                        from,
                        to,
                        at,
                        aircraft,
                        formed,
                    } => EventKind::AircraftLandedElsewhere {
                        from: AirGroupId::new(from),
                        to: AirGroupId::new(to),
                        at,
                        aircraft,
                        pilots: 0,
                        formed,
                    },
                    common::no_deck::Home::NoDeck {
                        group,
                        hex,
                        aircraft,
                        beside_own_force,
                    } => EventKind::AircraftDitchedNoDeck {
                        air_group: AirGroupId::new(group),
                        hex,
                        aircraft,
                        beside_own_force,
                        // The frozen figures: both crew figures are 1.
                        chance: Fixed::ONE,
                        crews_recovered: None,
                        pilots_lost: 0,
                        ready: model_groups
                            .iter()
                            .find(|g| g.id == group)
                            .map(|g| g.ready)
                            .expect("the group"),
                    },
                };
                // `ready` of a ditching is what the group has after all its
                // aircraft went: the model has worked the whole pulse.
                assert_eq!(event.kind, kind, "seed {seed} pulse {index}");
                assert_eq!(event.causes, causes, "seed {seed} pulse {index}");
            }
            for group in &model_groups {
                now.insert(AirGroupId::new(group.id), group.ready);
            }
        }

        work.attack = [0, 1].map(|c| numbers[c].clone().filter(|_| attack_ids[c].is_some()));
        ready = now;
        pulses.push(work);
    }

    // The end: every group's aircraft are those the events leave, the damage
    // is what the events add up to, and the decks and ships are as left.
    assert_eq!(run.ready, ready, "seed {seed}");
    for id in 1..=6u32 {
        let ship = ShipId::new(id);
        let last = run.ship(ship);
        assert_eq!(
            (last.flotation, last.fire),
            damage[&ship],
            "seed {seed}, ship {id}"
        );
        assert_eq!(last.sunk, !afloat.contains(&ship), "seed {seed}");
        for (slot, system) in ShipSystem::ALL.into_iter().enumerate() {
            assert_eq!(
                last.systems[slot],
                system_of(&systems, ship, system),
                "seed {seed}, ship {id}"
            );
        }
    }
    Worked { seed, pulses }
}

/// Every attack of every run: (the run, the pulse, the side that attacked,
/// its numbers).
fn all_attacks() -> Vec<(
    &'static Worked,
    &'static PulseWork,
    usize,
    &'static AttackNumbers,
)> {
    let mut all = Vec::new();
    for run in worked() {
        for pulse in &run.pulses {
            for (c, attack) in pulse.attack.iter().enumerate() {
                if let Some(attack) = attack {
                    all.push((run, pulse, c, attack));
                }
            }
        }
    }
    all
}

fn attacked(run: &Run, camp: usize, pulse_index: usize) -> bool {
    run.attacks_by(camp)
        .iter()
        .any(|event| daylight_index(event) == pulse_index)
}

// ---------------------------------------------------------------------
// The setting.
// ---------------------------------------------------------------------

#[test]
fn the_setting_is_as_the_hand_calculation_says_and_nothing_moves() {
    let scenario = load();
    assert_eq!(scenario.days, 2);
    let truth = scenario.world.umpire();
    let ships = |force: TaskForceId| -> Vec<u32> {
        truth.task_forces[&force]
            .ships
            .iter()
            .map(|ship| ship.raw())
            .collect()
    };
    assert_eq!(ships(CAMPS[ALLIED].force), vec![1, 2, 3]);
    assert_eq!(ships(CAMPS[JAPANESE].force), vec![4, 5, 6]);
    for camp in &CAMPS {
        assert_eq!(truth.task_forces[&camp.force].position(), camp.home);
        // The aircraft: 8 scouts, 18 dive bombers, 12 fighters, aboard the
        // carrier.
        for (group, aircraft) in [(camp.scouts, 8), (camp.bombers, 18), (camp.fighters, 12)] {
            assert_eq!(truth.air_groups[&group].ready, aircraft);
            assert_eq!(
                truth.air_groups[&group].location,
                AirGroupLocation::Ship(camp.carrier)
            );
        }
    }
    // The forces are 200 nm apart.
    assert_eq!(CAMPS[ALLIED].home.distance(CAMPS[JAPANESE].home), 5);
    // 20 + 8 + 8 = 36 anti-aircraft shots for each force while all three are
    // afloat, and a carrier has a toughness of 600 and a cruiser 400.
    let classes = scenario.world.ref_data();
    for camp in &CAMPS {
        let class = |ship: ShipId| &classes.ship_classes[&truth.ships[&ship].class];
        assert_eq!(class(camp.carrier).anti_aircraft, 20);
        assert_eq!(class(camp.cruisers[0]).anti_aircraft, 8);
        assert_eq!(class(camp.cruisers[1]).anti_aircraft, 8);
        assert_eq!(
            class(camp.carrier).anti_aircraft
                + class(camp.cruisers[0]).anti_aircraft
                + class(camp.cruisers[1]).anti_aircraft,
            36
        );
        assert_eq!(class(camp.carrier).toughness, Fixed::from_int(600));
        assert_eq!(class(camp.cruisers[0]).toughness, Fixed::from_int(400));
    }
    // A hit does 100 x 60 / 600 = 10 points on a carrier and 15 on a cruiser.
    let air = classes.air_combat.as_ref().expect("air combat figures");
    assert_eq!(air.fighter_hit_chance, tenths(3));
    assert_eq!(air.anti_aircraft_hit_chance, hundredths(5));
    assert_eq!(air.bomb_hit_chance, tenths(2));
    assert_eq!(air.strike_contact_max_age_pulses, 0);
    assert_eq!(
        Fixed::from_int(100 * 60) / Fixed::from_int(600),
        Fixed::from_int(10)
    );
    assert_eq!(
        Fixed::from_int(100 * 60) / Fixed::from_int(400),
        Fixed::from_int(15)
    );
    for run in batch() {
        assert_eq!(run.positions, [CAMPS[ALLIED].home, CAMPS[JAPANESE].home]);
    }
    // The orders: on day 1 each side's scouts search, its bombers strike and
    // its fighters keep half overhead.
    let mut scenario = load();
    scenario.run_to_end().expect("runs");
    let truth = scenario.world.umpire();
    for camp in &CAMPS {
        assert_eq!(truth.air_groups[&camp.fighters].patrol_share, tenths(5));
        assert_eq!(
            truth.air_groups[&camp.fighters].mission,
            sim_core::orders::AirMission::CombatAirPatrol
        );
        assert_eq!(
            truth.air_groups[&camp.bombers].mission,
            sim_core::orders::AirMission::NavalStrike
        );
        assert_eq!(
            truth.air_groups[&camp.scouts].mission,
            sim_core::orders::AirMission::Search
        );
    }
}

// ---------------------------------------------------------------------
// Each side on its own.
// ---------------------------------------------------------------------

#[test]
fn each_side_launches_when_it_holds_a_contact_its_own_search_reported_and_never_otherwise() {
    // `work_the_run` asserts, pulse by pulse and for each side, that its
    // bombers launch if, and only if, it holds a contact reported in that
    // pulse or the one before, that the contact is its own search's, and
    // that its search is its own rolls.
    let mut morning_launches = [0i64; 2];
    let mut afternoon_launches = [0i64; 2];
    let mut afternoon_working = [0i64; 2];
    // Pulses in which a side launched nothing though the other struck it.
    let mut struck_without_a_contact = [0i64; 2];
    let mut struck_and_never_sighted = [0i64; 2];
    for run in worked() {
        let never_sighted: [bool; 2] =
            [0, 1].map(|c| run.pulses.iter().all(|pulse| !pulse.sighted[c]));
        for pulse in &run.pulses {
            for c in 0..2 {
                // The hand calculation's own statement of the rule.
                let by_hand = if pulse.index % 2 == 0 {
                    pulse.sighted[c]
                } else {
                    pulse.sighted[c] || run.pulses[pulse.index - 1].attack[c].is_some()
                };
                assert_eq!(pulse.eligible[c], by_hand, "seed {}", run.seed);
                let can_launch = pulse.deck[c] != SystemState::Out
                    && pulse.afloat.contains(&CAMPS[c].carrier)
                    && pulse.ready[&CAMPS[c].bombers] > 0;
                assert_eq!(
                    pulse.launched[c].is_some(),
                    by_hand && can_launch,
                    "seed {} pulse {}",
                    run.seed,
                    pulse.index
                );
                if pulse.index == 0 {
                    morning_launches[c] += i64::from(pulse.launched[c].is_some());
                }
                if pulse.index == 1 && pulse.deck[c] == SystemState::Working {
                    afternoon_launches[c] += i64::from(pulse.launched[c].is_some());
                    afternoon_working[c] += 1;
                }
                // Being struck gives no contact: a side that holds none
                // launches nothing in a pulse in which the other strikes it.
                if !by_hand && pulse.attack[1 - c].is_some() {
                    assert!(pulse.launched[c].is_none());
                    struck_without_a_contact[c] += 1;
                }
            }
        }
        for c in 0..2 {
            if never_sighted[c] && run.pulses.iter().any(|pulse| pulse.attack[1 - c].is_some()) {
                struck_and_never_sighted[c] += 1;
                assert!(
                    run.pulses.iter().all(|pulse| pulse.launched[c].is_none()),
                    "seed {}: a side that sighted nothing launches nothing",
                    run.seed
                );
            }
        }
    }
    eprintln!(
        "launches in the first morning {morning_launches:?}, first afternoon {afternoon_launches:?}; struck with no contact in a pulse {struck_without_a_contact:?}, struck and sighted nothing all run {struck_and_never_sighted:?}"
    );
    for c in 0..2 {
        assert!(struck_without_a_contact[c] > 50);
        assert!(struck_and_never_sighted[c] > 5);
        // A morning launch is a morning sighting, with chance 0.509295 (every
        // deck works on the first morning). An afternoon launch is a
        // sighting that afternoon or an attack that morning, and a side
        // whose deck is hurt launches less or not at all, so the afternoon
        // band is taken over the runs in which this side's deck was working
        // at the start of the afternoon, 0.684234 for the hand calculation's
        // case of working decks. ASSUMED: that a side's deck working then is
        // independent of whether its own scouts sight and its own strike
        // attacked (the deck is hit only by the other side's strike, whose
        // launch rests on the other side's own sightings). The launch rule
        // itself, whatever the decks, is asserted pulse by pulse above, and
        // the search chance by deck in `work_the_run`.
        assert!(within_four_sigma(morning_launches[c], 1000, SEARCH_CHANCE));
        assert!(afternoon_working[c] > 300, "{afternoon_working:?}");
        assert!(
            within_four_sigma(afternoon_launches[c], afternoon_working[c], 684_234),
            "side {c}: {} of {} runs with a working deck",
            afternoon_launches[c],
            afternoon_working[c]
        );
    }
}

#[test]
fn the_three_counts_of_the_first_morning_are_in_their_bands() {
    let mut allied = 0;
    let mut japanese = 0;
    let mut both = 0;
    for run in batch() {
        let a = attacked(run, ALLIED, 0);
        let j = attacked(run, JAPANESE, 0);
        allied += i64::from(a);
        japanese += i64::from(j);
        both += i64::from(a && j);
    }
    eprintln!("day 1 morning: Allied {allied}, Japanese {japanese}, both {both}");
    assert!((296..=417).contains(&allied), "{allied}");
    assert!((296..=417).contains(&japanese), "{japanese}");
    assert!((85..=169).contains(&both), "{both}");
}

// ---------------------------------------------------------------------
// A first attack.
// ---------------------------------------------------------------------

#[test]
fn a_first_attack_by_either_side_meets_6_escorts_6_patrol_and_36_gun_shots_by_the_table() {
    let mut firsts = 0i64;
    // Hits summed over the attacks, and the bombers that attacked.
    let mut e_p_hits = 0i64;
    let mut p_e_hits = 0i64;
    let mut gun_hits = 0i64;
    let mut bomb_hits = 0i64;
    let mut bombers_attacking = 0i64;
    let mut attacking = BTreeSet::new();
    let mut by_side = [0i64; 2];
    for (run, pulse, c, attack) in all_attacks() {
        // While every group is whole and every flight deck works.
        let whole = pulse.deck == [SystemState::Working; 2]
            && CAMPS.iter().all(|camp| {
                pulse.ready[&camp.bombers] == 18
                    && pulse.ready[&camp.fighters] == 12
                    && pulse.ready[&camp.scouts] == 8
            })
            && pulse.afloat.len() == 6;
        if !whole {
            continue;
        }
        // The attacker sends 18 bombers and 6 escorts; the defender has kept
        // 6 fighters overhead.
        assert_eq!(
            (attack.bombers, attack.escorts, attack.patrol),
            (18, 6, 6),
            "seed {}",
            run.seed
        );
        // The patrol's 6 are all fired at the escorts: none at the bombers.
        assert_eq!(attack.shots_at_bombers, 0);
        assert!(attack.patrol_at_bombers.is_none());
        let hits = |roll: Option<u32>| roll.expect("the roll was made");
        let (e_p, p_e, g) = (
            hits(attack.escorts_at_patrol),
            hits(attack.patrol_at_escorts),
            hits(attack.guns),
        );
        // Each roll is shot by shot (decision 0033): 6 shots at 0.3 for the
        // escorts and for the patrol, the guns 36 shots at 0.05. Each scores
        // anything from none to its shots.
        assert_eq!(attack.gun_shots, 36);
        assert!(e_p <= 6 && p_e <= 6 && g <= 36, "seed {}", run.seed);
        // The bombers that attack are 18 less the guns' hits (a group cannot
        // lose more than it has), and each drops one bomb at 0.2.
        assert_eq!(attack.bombers_attacking, 18 - g.min(18));
        assert!(attack.hits <= attack.bombers_attacking, "seed {}", run.seed);
        // The target is the carrier, which does 10 points a hit.
        assert_eq!(attack.ship, CAMPS[1 - c].carrier);
        attacking.insert(attack.bombers_attacking);
        firsts += 1;
        by_side[c] += 1;
        e_p_hits += i64::from(e_p);
        p_e_hits += i64::from(p_e);
        gun_hits += i64::from(g);
        bomb_hits += i64::from(attack.hits);
        bombers_attacking += i64::from(attack.bombers_attacking);
    }
    eprintln!(
        "whole-state attacks {firsts} (Allied {}, Japanese {}): escorts' hits {e_p_hits}, patrol's hits {p_e_hits}, guns' hits {gun_hits}, bomb hits {bomb_hits} of {bombers_attacking} bombers; attacking {} to {}",
        by_side[0],
        by_side[1],
        attacking.first().expect("an attack"),
        attacking.last().expect("an attack")
    );
    assert!(firsts > 400);
    assert!(by_side[0] > 200 && by_side[1] > 200);
    // Over the runs each roll's hits are within four standard deviations of
    // shots x chance: the shots of all the attacks are independent draws at
    // one chance, so their hits are a binomial count of all the shots.
    assert!(
        within_four_sigma(e_p_hits, 6 * firsts, 300_000),
        "{e_p_hits} of {firsts} x 6"
    );
    assert!(
        within_four_sigma(p_e_hits, 6 * firsts, 300_000),
        "{p_e_hits} of {firsts} x 6"
    );
    assert!(
        within_four_sigma(gun_hits, 36 * firsts, 50_000),
        "{gun_hits} of {firsts} x 36"
    );
    assert!(
        within_four_sigma(bomb_hits, bombers_attacking, 200_000),
        "{bomb_hits} of {bombers_attacking}"
    );
    // 18 less 1.8, 16.2 on average; the spread is more than 16 and 17.
    assert!(attacking.len() > 2);
}

// ---------------------------------------------------------------------
// Both at once.
// ---------------------------------------------------------------------

#[test]
fn when_both_sides_attack_in_one_pulse_each_attack_follows_the_start_of_the_pulse() {
    let mut both_pulses = 0;
    let mut first_morning = 0;
    let mut two_causes = 0;
    let mut loss_totals = BTreeSet::new();
    let mut first_morning_lost = 0i64;
    let mut first_morning_groups = 0i64;
    let mut patrol_shot_down_still_fires = 0;
    for run in worked() {
        for pulse in &run.pulses {
            let (Some(a), Some(j)) = (&pulse.attack[ALLIED], &pulse.attack[JAPANESE]) else {
                continue;
            };
            both_pulses += 1;
            // Each side's numbers are those the start of the pulse gives
            // (asserted for every event by `work_the_run`): its bombers, its
            // escorts and the patrol the other holds are worked from the
            // aircraft and decks before either strike lands.
            let from_start = |c: usize| {
                let camp = &CAMPS[c];
                let enemy = &CAMPS[1 - c];
                let b = flyable(pulse.ready[&camp.bombers], pulse.deck[c]);
                let f = flyable(pulse.ready[&camp.fighters], pulse.deck[c]);
                let p = flyable(pulse.ready[&enemy.fighters], pulse.deck[1 - c]) / 2;
                (b, f - f / 2, p)
            };
            assert_eq!(
                (a.bombers, a.escorts, a.patrol),
                from_start(ALLIED),
                "seed {}",
                run.seed
            );
            assert_eq!(
                (j.bombers, j.escorts, j.patrol),
                from_start(JAPANESE),
                "seed {}",
                run.seed
            );
            // Both strikes were launched before either landed: both launches
            // precede every event of the air combat step.
            let events = &batch()[usize::try_from(run.seed - 1).expect("small")].events;
            let in_pulse: Vec<&Event> = events
                .iter()
                .filter(|e| {
                    e.day == DAYLIGHT[pulse.index].0 && e.pulse == Some(DAYLIGHT[pulse.index].1)
                })
                .collect();
            let last_launch = in_pulse
                .iter()
                .filter(|e| matches!(e.kind, EventKind::StrikeLaunched { .. }))
                .map(|e| e.id)
                .max()
                .expect("launches");
            let first_air = in_pulse
                .iter()
                .filter(|e| {
                    matches!(
                        e.kind,
                        EventKind::EscortsSent { .. }
                            | EventKind::AirFight { .. }
                            | EventKind::AntiAircraftFire { .. }
                            | EventKind::AircraftLost { .. }
                            | EventKind::AirAttack { .. }
                            | EventKind::StrikeFoundNothing { .. }
                    )
                })
                .map(|e| e.id)
                .min()
                .expect("the air step");
            assert!(last_launch < first_air, "seed {}", run.seed);
            assert_eq!(
                in_pulse
                    .iter()
                    .filter(|e| matches!(e.kind, EventKind::StrikeLaunched { .. }))
                    .count(),
                2
            );
            // A fighter group can lose aircraft twice in the pulse, its
            // patrol to the enemy's escorts and its escorts to the enemy's
            // patrol.
            for camp in &CAMPS {
                let events = pulse.loss_events.get(&camp.fighters).copied().unwrap_or(0);
                let lost = pulse.lost.get(&camp.fighters).copied().unwrap_or(0);
                if events == 2 {
                    two_causes += 1;
                }
                if pulse.index == 0 {
                    // Decision 0033: each cause costs what its 6 shots at 0.3
                    // hit, 0 to 6, 12 at most in all and 3.6 on average. A
                    // cause that hit nothing records no loss event. The
                    // group lost to the other side's escorts (its patrol) and
                    // to the other side's patrol (its own escorts), as worked.
                    let c = CAMPS
                        .iter()
                        .position(|other| other.fighters == camp.fighters)
                        .expect("a camp");
                    let to_escorts = pulse.attack[1 - c]
                        .as_ref()
                        .and_then(|attack| attack.escorts_at_patrol)
                        .expect("the other side's escorts fired");
                    let to_patrol = pulse.attack[c]
                        .as_ref()
                        .and_then(|attack| attack.patrol_at_escorts)
                        .expect("the other side's patrol fired");
                    assert_eq!(lost, to_escorts + to_patrol, "seed {}", run.seed);
                    assert_eq!(
                        events,
                        u32::from(to_escorts > 0) + u32::from(to_patrol > 0),
                        "seed {}",
                        run.seed
                    );
                    assert!(lost <= 12, "seed {} {lost}", run.seed);
                    first_morning_lost += i64::from(lost);
                    first_morning_groups += 1;
                    loss_totals.insert(lost);
                }
            }
            if pulse.index == 0 {
                first_morning += 1;
                // On day 1 in the morning each side sends 18 bombers and 6
                // escorts and keeps 6 overhead, whatever the other's strike
                // does to it in that pulse.
                for attack in [a, j] {
                    assert_eq!((attack.bombers, attack.escorts, attack.patrol), (18, 6, 6));
                    assert_eq!(attack.gun_shots, 36);
                    assert_eq!(attack.shots_at_bombers, 0);
                    // The patrol fires its full 6 at the escorts even though
                    // the other raid's escorts hit it in the same pulse.
                    patrol_shot_down_still_fires += 1;
                }
            }
        }
    }
    eprintln!(
        "pulses with both attacking {both_pulses}, of them the first morning {first_morning}; fighter groups losing aircraft to two causes in a pulse {two_causes}; first-morning losses of a group {loss_totals:?}"
    );
    assert!(both_pulses > 100);
    assert!(first_morning > 50);
    assert!(two_causes > 100);
    assert_eq!(patrol_shot_down_still_fires, 2 * first_morning);
    // Each fighter group faces 12 shots at 0.3 on the first morning: its
    // losses summed over the runs are within four standard deviations of 3.6
    // for each, and they are not always 2 to 4.
    assert_eq!(first_morning_groups, 2 * first_morning);
    assert!(
        within_four_sigma(first_morning_lost, 12 * first_morning_groups, 300_000),
        "{first_morning_lost} of {first_morning_groups} groups x 12"
    );
    assert!(loss_totals.len() > 3, "{loss_totals:?}");
}

// ---------------------------------------------------------------------
// Later attacks and the flight deck.
// ---------------------------------------------------------------------

#[test]
fn every_attack_of_every_run_is_worked_by_the_rule_from_the_start_of_its_pulse() {
    // `work_the_run` has asserted every event of every pulse of every run.
    let attacks = all_attacks();
    let mut per_pulse = [[0i64; 2]; 4];
    for (_, pulse, c, attack) in &attacks {
        per_pulse[pulse.index][*c] += 1;
        assert!(attack.bombers_attacking <= attack.bombers);
        // The bombers that attack are those that flew less those lost to
        // the patrol's fire at the bombers and to the guns.
        assert_eq!(
            attack.bombers_attacking,
            attack.bombers
                - attack.patrol_at_bombers.unwrap_or(0).min(attack.bombers)
                - attack.guns.unwrap_or(0).min(
                    attack.bombers - attack.patrol_at_bombers.unwrap_or(0).min(attack.bombers)
                )
        );
    }
    eprintln!("attacks per pulse [Allied, Japanese]: {per_pulse:?}");
    assert!(attacks.len() > 1500);
    for pulse in per_pulse {
        assert!(pulse[0] > 100 && pulse[1] > 100);
    }
}

#[test]
fn a_hit_deck_halves_or_stops_search_strike_escort_and_patrol_from_the_next_pulse_only() {
    let mut search_halved = 0;
    let mut search_stopped = 0;
    let mut strike_halved = 0;
    let mut strike_stopped = 0;
    let mut escort_halved = 0;
    let mut escort_stopped = 0;
    let mut patrol_halved = 0;
    let mut patrol_stopped = 0;
    let mut hit_and_still_full = 0;
    // What is new in decisions 7 to 10 of 0059: a deck that goes OUT in a
    // pulse sends the aircraft its groups have in the air in that pulse
    // elsewhere (here, to the sea), citing the hit; a deck only degraded
    // takes them, and nothing is recorded for it.
    let (mut out_with_aircraft_in_the_air, mut degraded_only) = (0, 0);
    for run in batch() {
        for event in &run.events {
            let EventKind::SystemHit {
                ship,
                system: ShipSystem::FlightDeck,
                after,
                ..
            } = event.kind
            else {
                continue;
            };
            let Some(camp) = CAMPS.iter().find(|camp| camp.carrier == ship) else {
                continue;
            };
            let same_pulse = |e: &&Event| (e.day, e.pulse) == (event.day, event.pulse);
            let flew = run.events.iter().filter(same_pulse).any(|e| {
                matches!(
                    e.kind,
                    EventKind::SearchFlown { air_group, .. }
                        | EventKind::StrikeLaunched { air_group, .. }
                        | EventKind::EscortsSent { air_group, .. }
                        if [camp.scouts, camp.bombers, camp.fighters].contains(&air_group)
                )
            });
            let homing = run.events.iter().filter(same_pulse).any(|e| {
                matches!(e.kind, EventKind::AircraftDitchedNoDeck { air_group, .. }
                    if [camp.scouts, camp.bombers, camp.fighters].contains(&air_group))
            });
            let lost_in_pulse = run.events.iter().filter(same_pulse).any(|e| {
                matches!(e.kind, EventKind::ShipSunk { ship: sunk, .. } if sunk == ship)
                    || matches!(e.kind, EventKind::SystemHit {
                        ship: hit, system: ShipSystem::FlightDeck, after: SystemState::Out, ..
                    } if hit == ship)
            });
            if after == SystemState::Out && flew && homing {
                out_with_aircraft_in_the_air += 1;
            }
            if after == SystemState::Degraded && !lost_in_pulse {
                assert!(
                    !homing,
                    "seed {}: a degraded deck takes its aircraft",
                    run.seed
                );
                degraded_only += 1;
            }
        }
    }
    assert!(out_with_aircraft_in_the_air > 0 && degraded_only > 0);
    for run in worked() {
        for (i, pulse) in run.pulses.iter().enumerate() {
            for c in 0..2 {
                let camp = &CAMPS[c];
                let enemy = &CAMPS[1 - c];
                let ready = |group: AirGroupId| pulse.ready[&group];
                match pulse.deck[c] {
                    SystemState::Degraded => {
                        if ready(camp.scouts) > 0 {
                            assert_eq!(pulse.searched[c], Some(ready(camp.scouts) / 2));
                            search_halved += 1;
                        }
                        if let Some(aircraft) = pulse.launched[c] {
                            assert_eq!(aircraft, ready(camp.bombers) / 2);
                            strike_halved += 1;
                        }
                        if let Some(escorts) = pulse.escorts_sent[c] {
                            let can = ready(camp.fighters) / 2;
                            assert_eq!(escorts, can - can / 2);
                            // Fewer than a working deck sends: strictly so
                            // unless the group is down to two aircraft, when
                            // one goes either way.
                            let full = ready(camp.fighters) - ready(camp.fighters) / 2;
                            assert!(escorts <= full);
                            if ready(camp.fighters) != 2 {
                                assert!(escorts < full);
                            }
                            escort_halved += 1;
                        }
                    }
                    SystemState::Out => {
                        if ready(camp.scouts) > 0 {
                            assert_eq!(pulse.searched[c], None);
                            search_stopped += 1;
                        }
                        if pulse.eligible[c] && ready(camp.bombers) > 0 {
                            assert_eq!(pulse.launched[c], None);
                            strike_stopped += 1;
                        }
                        if ready(camp.fighters) > 0 {
                            // Nothing goes with a raid that did not leave.
                            assert_eq!(pulse.escorts_sent[c], None);
                            escort_stopped += 1;
                        }
                    }
                    SystemState::Working => {}
                }
                // The patrol the defender can still fly.
                if let Some(attack) = &pulse.attack[c] {
                    let held = ready(enemy.fighters);
                    match pulse.deck[1 - c] {
                        SystemState::Degraded if held > 0 => {
                            assert_eq!(attack.patrol, held / 2 / 2);
                            patrol_halved += 1;
                        }
                        SystemState::Out if held > 0 => {
                            assert_eq!(attack.patrol, 0);
                            assert!(attack.escorts_at_patrol.is_none());
                            assert!(attack.patrol_at_escorts.is_none());
                            assert!(attack.patrol_at_bombers.is_none());
                            patrol_stopped += 1;
                        }
                        _ => {}
                    }
                }
                // A hit changes nothing in its own pulse: a side whose deck
                // was hit in this pulse flew and struck at full strength.
                if let Some(next) = run.pulses.get(i + 1) {
                    if next.deck[c] != pulse.deck[c] {
                        if let Some(attack) = &pulse.attack[c] {
                            assert_eq!(attack.bombers, flyable(ready(camp.bombers), pulse.deck[c]));
                            if pulse.deck[c] == SystemState::Working {
                                assert_eq!(attack.bombers, ready(camp.bombers));
                                hit_and_still_full += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    eprintln!(
        "search halved {search_halved} stopped {search_stopped}; strike halved {strike_halved} stopped {strike_stopped}; escorts halved {escort_halved} stopped {escort_stopped}; patrol halved {patrol_halved} stopped {patrol_stopped}; hit and still full {hit_and_still_full}"
    );
    assert!(search_halved > 0 && search_stopped > 0);
    assert!(strike_halved > 0 && strike_stopped > 0);
    assert!(escort_halved > 0 && escort_stopped > 0);
    assert!(patrol_halved > 0 && patrol_stopped > 0);
    assert!(hit_and_still_full > 0);
}

// ---------------------------------------------------------------------
// Sinking.
// ---------------------------------------------------------------------

#[test]
fn a_carrier_sunk_takes_its_groups_with_it_and_its_force_is_left_with_its_cruisers() {
    let mut sinkings = 0;
    let mut day_1_sinkings = 0;
    let mut runs_with_a_day_1_sinking = BTreeSet::new();
    let mut sinking_pulse_strike_attacked = 0;
    let mut later_attacks_on_cruisers = 0;
    for run in worked() {
        for pulse in &run.pulses {
            for ship in &pulse.sunk {
                let Some(victim) = CAMPS.iter().position(|camp| camp.carrier == *ship) else {
                    continue;
                };
                sinkings += 1;
                if pulse.index < 2 {
                    day_1_sinkings += 1;
                    runs_with_a_day_1_sinking.insert(run.seed);
                }
                let camp = &CAMPS[victim];
                // The loss events of the groups aboard, citing the sinking,
                // were asserted in `work_the_run`. After it every group
                // aboard has no aircraft and flies nothing.
                for later in &run.pulses[pulse.index + 1..] {
                    for group in [camp.scouts, camp.bombers, camp.fighters] {
                        assert_eq!(later.ready[&group], 0, "seed {}", run.seed);
                    }
                    assert_eq!(later.searched[victim], None);
                    assert_eq!(later.launched[victim], None);
                    assert_eq!(later.escorts_sent[victim], None);
                    // The force has 16 shots and no patrol; a strike on it
                    // attacks the lower-numbered cruiser afloat.
                    if let Some(attack) = &later.attack[1 - victim] {
                        assert_eq!(attack.patrol, 0);
                        let cruisers: Vec<ShipId> = camp
                            .cruisers
                            .iter()
                            .copied()
                            .filter(|ship| later.afloat.contains(ship))
                            .collect();
                        assert_eq!(
                            attack.gun_shots,
                            8 * u32::try_from(cruisers.len()).expect("2")
                        );
                        if cruisers.len() == 2 {
                            assert_eq!(attack.gun_shots, 16);
                        }
                        assert_eq!(attack.ship, cruisers[0]);
                        later_attacks_on_cruisers += 1;
                    }
                }
                // A strike the carrier launched in the pulse it sank still
                // attacks.
                if pulse.launched[victim].is_some() && pulse.found[victim] {
                    assert!(pulse.attack[victim].is_some(), "seed {}", run.seed);
                    sinking_pulse_strike_attacked += 1;
                }
            }
        }
    }
    eprintln!(
        "carriers sunk {sinkings}, on day 1 {day_1_sinkings} (in {} runs); sunk in a pulse in which their own strike attacked {sinking_pulse_strike_attacked}; later attacks on cruisers {later_attacks_on_cruisers}",
        runs_with_a_day_1_sinking.len()
    );
    // A carrier can be sunk on either day (decision 0033: ten hits do it),
    // though rarely: well under one run in ten sinks one on the first day.
    assert!(sinkings > 0);
    assert!(sinking_pulse_strike_attacked > 0);
    assert!(later_attacks_on_cruisers > 0);
    assert!(runs_with_a_day_1_sinking.len() < 100);
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn a_strike_is_launched_only_at_a_contact_its_own_side_holds_reported_this_pulse_or_the_one_before()
{
    let mut launches = 0;
    for run in batch() {
        let by_id: BTreeMap<EventId, &Event> = run.events.iter().map(|e| (e.id, e)).collect();
        for launch in run.of(|k| matches!(k, EventKind::StrikeLaunched { .. })) {
            let EventKind::StrikeLaunched {
                air_group,
                contact,
                hex,
                ..
            } = launch.kind
            else {
                unreachable!()
            };
            let camp = CAMPS
                .iter()
                .find(|camp| camp.bombers == air_group)
                .expect("a bomber group");
            let Cause::Event(report) = launch.causes[1] else {
                panic!("a launch cites a report");
            };
            let report = by_id[&report];
            let EventKind::ReportReceived {
                side,
                contact: reported,
                hex: reported_at,
                ..
            } = report.kind
            else {
                panic!("seed {}: a launch cites a report", run.seed);
            };
            assert_eq!(side, camp.side, "seed {}", run.seed);
            assert_eq!((reported, reported_at), (contact, hex));
            // Reported in this pulse or the one before.
            let (then, now) = (daylight_index(report), daylight_index(launch));
            assert!(
                then == now || (then + 1 == now && now % 2 == 1),
                "seed {}: launched in pulse {now} on a report of pulse {then}",
                run.seed
            );
            launches += 1;
        }
    }
    assert!(launches > 1500);
}

#[test]
fn no_sides_decision_reads_the_other_sides_state() {
    // A run in which only one side ever sights has strikes by that side only.
    let mut only_one = [0, 0];
    for run in worked() {
        let sighted: [bool; 2] = [0, 1].map(|c| run.pulses.iter().any(|pulse| pulse.sighted[c]));
        for c in 0..2 {
            if sighted[c] && !sighted[1 - c] {
                only_one[c] += 1;
                for pulse in &run.pulses {
                    assert!(pulse.launched[1 - c].is_none(), "seed {}", run.seed);
                    assert!(pulse.attack[1 - c].is_none(), "seed {}", run.seed);
                    assert!(pulse.escorts_sent[1 - c].is_none(), "seed {}", run.seed);
                }
            }
        }
    }
    eprintln!(
        "runs in which only the Allied side sights {}, only the Japanese {}",
        only_one[0], only_one[1]
    );
    assert!(only_one[0] > 20 && only_one[1] > 20);
}

#[test]
fn in_every_pulse_for_both_sides_the_launch_the_escorts_and_the_patrol_follow_from_the_start_of_the_pulse(
) {
    // `work_the_run` derives every one of them from the aircraft and decks
    // at the start of the pulse and asserts the events against them; here
    // the summary of that work is held to the same statement.
    let mut pulses = 0;
    for run in worked() {
        for pulse in &run.pulses {
            for (c, camp) in CAMPS.iter().enumerate() {
                let enemy = &CAMPS[1 - c];
                let can =
                    |group: AirGroupId, side: usize| flyable(pulse.ready[&group], pulse.deck[side]);
                if let Some(bombers) = pulse.launched[c] {
                    assert_eq!(bombers, can(camp.bombers, c));
                    let escorts = can(camp.fighters, c) - can(camp.fighters, c) / 2;
                    assert_eq!(pulse.escorts_sent[c], (escorts > 0).then_some(escorts));
                }
                if let Some(attack) = &pulse.attack[c] {
                    assert_eq!(attack.patrol, can(enemy.fighters, 1 - c) / 2);
                    assert_eq!(attack.bombers, can(camp.bombers, c));
                    pulses += 1;
                }
            }
        }
    }
    assert!(pulses > 1500);
}

#[test]
fn every_groups_aircraft_at_the_end_are_those_at_the_start_less_its_recorded_losses() {
    for run in batch() {
        for camp in &CAMPS {
            for (group, start) in [(camp.scouts, 8u32), (camp.bombers, 18), (camp.fighters, 12)] {
                let mut left = start;
                for event in &run.events {
                    match event.kind {
                        EventKind::AircraftLost {
                            air_group,
                            aircraft,
                            remaining,
                        } if air_group == group => {
                            left = left.checked_sub(aircraft).expect("none goes below zero");
                            assert_eq!(left, remaining, "seed {}", run.seed);
                        }
                        // A sunk ship takes what was aboard; the aircraft in
                        // the air are kept until they come home (decisions 7
                        // to 10 of 0059).
                        EventKind::AirGroupLost {
                            air_group,
                            aircraft,
                            ..
                        } if air_group == group => {
                            left = left
                                .checked_sub(aircraft)
                                .expect("a sunk ship takes no more than is left");
                        }
                        // What came down in the sea is lost, and what landed
                        // elsewhere leaves the group (and joins another).
                        EventKind::AircraftDitchedNoDeck {
                            air_group,
                            aircraft,
                            ready,
                            ..
                        } if air_group == group => {
                            left = left.checked_sub(aircraft).expect("none goes below zero");
                            assert_eq!(left, ready, "seed {}", run.seed);
                        }
                        EventKind::AircraftLandedElsewhere { from, aircraft, .. }
                            if from == group =>
                        {
                            left = left.checked_sub(aircraft).expect("none goes below zero");
                        }
                        EventKind::AircraftLandedElsewhere { to, aircraft, .. } if to == group => {
                            left += aircraft;
                        }
                        _ => {}
                    }
                }
                assert_eq!(
                    run.ready[&group],
                    left,
                    "seed {}, group {}",
                    run.seed,
                    group.raw()
                );
                assert!(run.ready[&group] <= start);
            }
        }
    }
}

#[test]
fn aircraft_come_home_elsewhere_only_when_their_deck_sank_or_went_out_in_the_pulse() {
    // Decisions 7 to 10 of 0059. A deck only degraded takes its aircraft (the
    // model in `work_the_run` expects no event for it); a deck that is out
    // or sunk sends the aircraft in the air to the sea here, where there is
    // no other carrier and no base. Nothing of the kind ever happens to a
    // group whose ship neither sank nor lost her deck in the pulse.
    let ship_of = |group: AirGroupId| {
        CAMPS
            .iter()
            .find(|camp| [camp.scouts, camp.bombers, camp.fighters].contains(&group))
            .map(|camp| camp.carrier)
    };
    let mut homecomings = 0;
    for run in batch() {
        homecomings += common::no_deck::assert_only_for_lost_decks(&run.events, ship_of);
        // Every one here comes down in the sea, never landed elsewhere.
        assert!(run
            .events
            .iter()
            .all(|e| !matches!(e.kind, EventKind::AircraftLandedElsewhere { .. })));
    }
    assert!(
        homecomings > 0,
        "some deck goes out with aircraft in the air"
    );
}

#[test]
fn every_damage_event_is_caused_by_an_attack_every_attack_by_a_launch_and_every_launch_by_a_report()
{
    let mut damage_events = 0;
    for run in batch() {
        let by_id: BTreeMap<EventId, &Event> = run.events.iter().map(|e| (e.id, e)).collect();
        let cause_of = |event: &Event| -> &Event {
            assert_eq!(event.causes.len(), 1, "seed {}", run.seed);
            let Cause::Event(id) = event.causes[0] else {
                panic!("seed {}: an event cites an event", run.seed);
            };
            by_id[&id]
        };
        for event in &run.events {
            match event.kind {
                EventKind::ShipDamaged { .. } | EventKind::SystemHit { .. } => {
                    assert!(matches!(cause_of(event).kind, EventKind::AirAttack { .. }));
                    damage_events += 1;
                }
                EventKind::ShipSunk { .. } => {
                    assert!(matches!(
                        cause_of(event).kind,
                        EventKind::ShipDamaged { .. }
                    ));
                }
                EventKind::AirGroupLost { .. } => {
                    assert!(matches!(cause_of(event).kind, EventKind::ShipSunk { .. }));
                }
                EventKind::AirAttack { air_group, .. } => {
                    let Cause::Event(launch) = event.causes[0] else {
                        panic!("an attack cites its launch");
                    };
                    assert!(matches!(
                        by_id[&launch].kind,
                        EventKind::StrikeLaunched { air_group: launched, .. } if launched == air_group
                    ));
                    for cause in &event.causes[1..] {
                        let Cause::Event(loss) = cause else {
                            panic!("an attack cites events");
                        };
                        assert!(matches!(
                            by_id[loss].kind,
                            EventKind::AircraftLost { air_group: lost, .. } if lost == air_group
                        ));
                    }
                }
                EventKind::StrikeLaunched { air_group, .. } => {
                    let Cause::Event(report) = event.causes[1] else {
                        panic!("a launch cites a report");
                    };
                    let camp = CAMPS
                        .iter()
                        .find(|camp| camp.bombers == air_group)
                        .expect("a bomber group");
                    assert!(matches!(
                        by_id[&report].kind,
                        EventKind::ReportReceived { side, .. } if side == camp.side
                    ));
                }
                EventKind::ReportReceived { side, .. } => {
                    // A report is caused by a search sighting or an attack,
                    // both of its own side's groups.
                    let camp = CAMPS.iter().find(|camp| camp.side == side).expect("a side");
                    match cause_of(event).kind {
                        EventKind::SearchAttempt {
                            air_group, sighted, ..
                        } => assert!(air_group == camp.scouts && sighted),
                        EventKind::AirAttack { air_group, .. } => {
                            assert_eq!(air_group, camp.bombers);
                        }
                        ref other => panic!("seed {}: a report cites {other}", run.seed),
                    }
                }
                _ => {}
            }
        }
    }
    assert!(damage_events > 3000);
}

#[test]
fn a_replay_reproduces_every_daily_hash() {
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
        assert_eq!(hashes[0].len(), 2);
        assert_eq!(hashes[0], hashes[1], "seed {seed}");
        all_hashes.push(hashes[0].clone());
    }
    // The seed matters.
    assert!(all_hashes.windows(2).any(|pair| pair[0] != pair[1]));
    // The batch holds the same hashes as a fresh run of the same seed.
    let mut again = base.with_seed(77).expect("a seed can be set");
    again.run_to_end().expect("the run resolves");
    assert_eq!(batch()[76].seed, 77);
    assert_eq!(batch()[76].hashes, again.world.day_hashes());
    // The file's own seed gives the same hashes as a seeded copy.
    let mut own = load();
    own.run_to_end().expect("the run resolves");
    let mut copy = base.with_seed(1942).expect("a seed can be set");
    copy.run_to_end().expect("the run resolves");
    assert_eq!(own.world.day_hashes(), copy.world.day_hashes());
}

// ---------------------------------------------------------------------
// The summary, for the architect. It asserts nothing.
// ---------------------------------------------------------------------

#[test]
fn print_the_summary() {
    let mut attacks = [[0i64; 2]; 4];
    let mut both = [0i64; 4];
    let mut sunk = [0i64; 2];
    let mut cruisers_sunk = 0;
    let mut left: BTreeMap<AirGroupId, i64> = BTreeMap::new();
    for run in worked() {
        for pulse in &run.pulses {
            for (total, attack) in attacks[pulse.index].iter_mut().zip(&pulse.attack) {
                *total += i64::from(attack.is_some());
            }
            both[pulse.index] += i64::from(pulse.attack.iter().all(Option::is_some));
            for ship in &pulse.sunk {
                match CAMPS.iter().position(|camp| camp.carrier == *ship) {
                    Some(victim) => sunk[victim] += 1,
                    None => cruisers_sunk += 1,
                }
            }
        }
    }
    for run in batch() {
        for (group, ready) in &run.ready {
            *left.entry(*group).or_default() += i64::from(*ready);
        }
    }
    eprintln!("SUMMARY over seeds 1 to {SEEDS}");
    for (index, (day, pulse)) in DAYLIGHT.iter().enumerate() {
        eprintln!(
            "  day {day} {pulse:?}: attacks Allied {}, Japanese {}, both in one pulse {}",
            attacks[index][0], attacks[index][1], both[index]
        );
    }
    eprintln!(
        "  carriers sunk: Allied {}, Japanese {}; cruisers sunk {cruisers_sunk}",
        sunk[0], sunk[1]
    );
    for (group, total) in &left {
        eprintln!(
            "  air_group:{} mean aircraft left {}.{:03}",
            group.raw(),
            total / count(batch().len()),
            total * 1000 / count(batch().len()) % 1000
        );
    }
}
