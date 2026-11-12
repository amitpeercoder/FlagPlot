//! Scenario 4: Savo Island, the night of 8 to 9 August 1942.
//!
//! Stage 4c2 of scenario 4, and the first scenario with real ships. The
//! Japanese column steams down the Slot, enters the hex of Savo Island and
//! meets the southern and then the northern cruiser group there.
//!
//! The ships' figures are derived from sourced facts
//! (`data/reference/ship_classes_savo_island.csv`, decisions 0020 and 0023).
//! The map is real. What is in the scenario and what is left out is
//! `docs/decisions/0024-savo-island-scenario.md`. The combat figures in
//! `data/reference/surface_combat.toml` are placeholders, so the losses this
//! scenario produces are not a historical check yet: the spec sets the bands
//! for that in step 2. This file asserts what the rules fix and prints the
//! spread of losses beside what happened.
//!
//! # Hand calculation
//!
//! Written by the architect before the test code. The tests below assert
//! these numbers. If the code disagrees, the code or the rule is wrong: do
//! not change a number here to make a test pass.
//!
//! Quantities are cut to six decimal places at each step, as the engine
//! does (`Fixed`).
//!
//! ## The forces
//!
//! | Task force | Ships, by ID |
//! | ---------- | ------------ |
//! | 1, Japanese column, battle readiness | 1 Chokai, 2 Aoba, 3 Kako, 4 Kinugasa, 5 Furutaka, 6 Tenryu, 7 Yubari, 8 Yunagi |
//! | 2, southern group, normal readiness  | 9 Canberra, 10 Chicago, 11 Patterson, 12 Bagley |
//! | 3, northern group, normal readiness  | 13 Vincennes, 14 Quincy, 15 Astoria, 16 Helm, 17 Wilson |
//!
//! Both Allied groups lie in hex (2,-5) and do not move. The Allied side
//! holds one contact at the start, at (-1,-9), seven hexes away, so neither
//! group is alerted by it (decision 0021: three hexes at most).
//!
//! ## The approach
//!
//! Ships are charged the straight-line miles a step brings them closer to
//! their waypoint where the way is clear (decision 0058); the start was
//! worked back with that rule (decision 0024, amended).
//!
//! The column starts in (-1,-8) and is ordered to (2,-5) and back, at 25
//! knots. (-1,-8) to (2,-5) is dq 3, dr 3: 6 hexes, and the shortest sea path
//! is 6 steps, so the way is clear. The path is (0,-8), (1,-8), (1,-7),
//! (2,-7), (2,-6), (2,-5). (From (1,-8) the first direction in order, to
//! (2,-8), is closed toward (2,-7) and (1,-7), so (2,-8) is not one step
//! nearer by sea; the path goes to (1,-7).) The air distance to (2,-5) from
//! each centre, 40 times the root of dq^2 + dr^2 + dq x dr, to the nearest
//! mile, and the charge of the step from it:
//!
//! | Centre  | dq, dr | dq^2 + dr^2 + dq x dr | Miles to (2,-5) | Step charged | Entered after |
//! | ------- | ------ | --------------------- | --------------- | ------------ | ------------- |
//! | (-1,-8) | -3, -3 | 27                    | 208             | 208 - 174 = 34 | 17 nm       |
//! | (0,-8)  | -2, -3 | 19                    | 174             | 174 - 144 = 30 | 34 + 15 = 49 nm |
//! | (1,-8)  | -1, -3 | 13                    | 144             | 144 - 106 = 38 | 64 + 19 = 83 nm |
//! | (1,-7)  | -1, -2 | 7                     | 106             | 106 - 80 = 26  | 102 + 13 = 115 nm |
//! | (2,-7)  | 0, -2  | 4                     | 80              | 80 - 40 = 40   | 128 + 20 = 148 nm |
//! | (2,-6)  | 0, -1  | 1                     | 40              | 40 - 0 = 40    | 168 + 20 = 188 nm |
//!
//! The path is 34 + 30 + 38 + 26 + 40 + 40 = 208 nm. The column enters (2,-5)
//! 20 nm before its centre, after 188 nm, which at 25 knots is 188 / 25 =
//! 7.52 hours into the night pulse: 01:31 on 9 August (7 h 31.2 min after
//! 18:00). At midnight, 6 hours in, it has steamed 150 nm and is 188 - 150 =
//! 38 nm short of the hex, where the source has it 37 miles from Cape
//! Esperance. It meets the southern group, task force 2, and then the
//! northern group, task force 3, at that same moment, in that order.
//!
//! Every Japanese ship cruises at 14 knots. At 25 knots the burn per mile is
//! multiplied by (25 / 14) squared: 25 / 14 = 1.785714, squared 3.188774.
//! Fuel for a distance is distance x capacity / endurance, times that.
//!
//! | Ship       | Capacity | Endurance | 188 nm at cruise rate | Burned by 01:31 | Left      |
//! | ---------- | -------- | --------- | --------------------- | --------------- | --------- |
//! | 1 Chokai   | 2529     | 8000      | 59.4315               | 189.513621      | 2339.486379 |
//! | 2 Aoba     | 2040     | 8000      | 47.94                 | 152.869825      | 1887.130175 |
//! | 3 Kako     | 1858     | 7900      | 44.215696             | 140.993861      | 1717.006139 |
//! | 4 Kinugasa | 2040     | 8000      | 47.94                 | 152.869825      | 1887.130175 |
//! | 5 Furutaka | 1858     | 7900      | 44.215696             | 140.993861      | 1717.006139 |
//! | 6 Tenryu   | 905      | 5000      | 34.028                | 108.507601      | 796.492399  |
//! | 7 Yubari   | 902      | 3310      | 51.231419             | 163.365416      | 738.634584  |
//! | 8 Yunagi   | 413      | 3600      | 21.567777             | 68.774766       | 344.225234  |
//!
//! ## Who sees whom
//!
//! At night a force sights with chance (0.5, plus 0.2 if any ship afloat in
//! it has radar) x 0.5 if it is not alert.
//!
//! | Force                         | Radar                 | Alert | Chance a round |
//! | ----------------------------- | --------------------- | ----- | -------------- |
//! | Japanese column               | none                  | yes   | 0.5            |
//! | Southern group                | Canberra, Chicago     | no    | 0.35           |
//! | Northern group, not alarmed   | Quincy, Astoria       | no    | 0.35           |
//! | Northern group, alarmed       | Quincy, Astoria       | yes   | 0.7            |
//!
//! The southern fight, round by round until somebody sights: only the
//! Japanese 0.5 x 0.65 = 0.325, only the Allies 0.5 x 0.35 = 0.175, both
//! 0.175, neither 0.325. Neither ever sights in six rounds with chance
//! 0.325^6 = 0.001178.
//!
//! | Count of runs, over seeds 1 to 1000        | p                              | Mean  | Std dev | Allowed    |
//! | ------------------------------------------ | ------------------------------ | ----- | ------- | ---------- |
//! | The Japanese have a free round             | 0.325 / 0.675 x 0.998822 = 0.480914 | 480.91 | 15.80 | 418 to 544 |
//! | The southern group has a free round        | 0.175 / 0.675 x 0.998822 = 0.258953 | 258.95 | 13.85 | 204 to 314 |
//! | Both fire from the start                   | 0.258953                       | 258.95 | 13.85   | 204 to 314 |
//! | They pass unseen                           | 0.001178                       | 1.18  | 1.09    | 0 to 5     |
//!
//! If shots were fired in the southern fight, the northern group, which is
//! in the same hex and not alert, rolls for the alarm at 0.5 (decision
//! 0021). Among the runs with such a roll the number alarmed is within four
//! standard deviations of half. The northern group's chance in round 1 of
//! its own fight is then 0.7 if it was alarmed and 0.35 if not.
//!
//! ## The first salvos
//!
//! Ships are matched in ID order, starting again at the first when the enemy
//! has fewer ships. Gun shots in a round are guns x rounds a minute x 5. At
//! night a gun shot hits with chance 0.03 and a torpedo with 0.04, each
//! times 1.5 against a surprised target: 0.045 and 0.06. A ship fires all
//! its tubes once, in the first round it fires. A hit does 100 x the
//! weapon's damage / the target's toughness points.
//!
//! The Japanese column's free round against the southern group, when it has
//! one. Every ship's propulsion, guns and tubes are still whole.
//!
//! | Firer      | Target       | Gun shots          | Expected hits | Points a hit | Tubes | Expected hits | Points a hit |
//! | ---------- | ------------ | ------------------ | ------------- | ------------ | ----- | ------------- | ------------ |
//! | 1 Chokai   | 9 Canberra   | 10 x 2.4 x 5 = 120 | 5.4           | 5.378205     | 8     | 0.48          | 38.461538    |
//! | 2 Aoba     | 10 Chicago   | 6 x 2.4 x 5 = 72   | 3.24          | 5.794198     | 8     | 0.48          | 54.143646    |
//! | 3 Kako     | 11 Patterson | 72                 | 3.24          | 32.2891      | 8     | 0.48          | 301.724137   |
//! | 4 Kinugasa | 12 Bagley    | 72                 | 3.24          | 32.2891      | 8     | 0.48          | 301.724137   |
//! | 5 Furutaka | 9 Canberra   | 72                 | 3.24          | 5.378205     | 8     | 0.48          | 50.25641     |
//! | 6 Tenryu   | 10 Chicago   | 4 x 6 x 5 = 120    | 5.4           | 1.749539     | 6     | 0.36          | 22.099447    |
//! | 7 Yubari   | 11 Patterson | 6 x 6 x 5 = 180    | 8.1           | 9.749588     | 4     | 0.24          | 212.438423   |
//! | 8 Yunagi   | 12 Bagley    | 4 x 5.5 x 5 = 110  | 4.95          | 5.208332     | 6     | 0.36          | 123.152709   |
//!
//! The southern group's free round against the column, when it has one:
//!
//! | Firer        | Target     | Gun shots           | Expected hits | Points a hit | Tubes | Expected hits | Points a hit |
//! | ------------ | ---------- | ------------------- | ------------- | ------------ | ----- | ------------- | ------------ |
//! | 9 Canberra   | 1 Chokai   | 8 x 3.5 x 5 = 140   | 6.3           | 4.262114     | 8     | 0.48          | 29.577092    |
//! | 10 Chicago   | 2 Aoba     | 9 x 3 x 5 = 135     | 6.075         | 5.545834     | 0     |               |              |
//! | 11 Patterson | 3 Kako     | 4 x 15 x 5 = 300    | 13.5          | 1.216473     | 16    | 0.96          | 26.170734    |
//! | 12 Bagley    | 4 Kinugasa | 300                 | 13.5          | 1.175961     | 16    | 0.96          | 25.299164    |
//!
//! Each shot is rolled on its own (decision 0033), so a salvo scores
//! anything from no hit to its shots, the expected figure on average. Over
//! the runs with such a free round the total hits of each salvo in either
//! table are within four standard deviations of its expected figure times
//! the number of runs. (Before decision 0033 a salvo scored the whole part
//! of the expected figure or one more.)
//!
//! In a free round the side fired on fires nothing.
//!
//! ## After the fights
//!
//! The column is ordered to detach any ship whose propulsion is degraded or
//! out (decision 0016): a ship that can still steam is sent home to Rabaul
//! with escorts taken from the column, and a ship that cannot move is left
//! in (2,-5). If that would leave the column no ship with working
//! propulsion, the whole column retires for Rabaul instead, at the speed of
//! its slowest ship. (Added after the code was run: the first version of
//! this section left retiring out. It happens in about one run in six.) In
//! every other run the column itself, if it still has a ship, makes 25
//! knots after its fights. The night pulse is 12 hours, 300 nm at 25 knots,
//! so it had 300 - 188 = 112 nm of the night left: 20 nm on to the centre of
//! (2,-5), then 92 nm back along the path (2,-6), (2,-7), (1,-7), (1,-8),
//! (0,-8), (-1,-8). With (-1,-8) as the waypoint the steps of the way back
//! are charged 34, 30, 38, 26, 40 and 40 (air distances 208, 174, 144, 106,
//! 80, 40, 0), the same as the way in. The column passes the centre of (2,-6)
//! at 34 nm and of (2,-7) at 64 nm, and enters (1,-7) at 64 + 19 = 83 nm, so
//! it ends the night 92 - 64 = 28 nm along the 38 nm step from (2,-7) to
//! (1,-7), in hex (1,-7). It has 208 - 92 = 116 nm to go and arrives at
//! (-1,-8) 116 / 25 = 4.64 hours into the morning pulse, 12 + 4.64 = 16.64
//! hours after the start (D1 10:38).
//!
//! A ship that stays with the column to the end burns, at 25 knots:
//!
//! | Ship       | 112 nm    | 116 nm     | Left at the end |
//! | ---------- | --------- | ---------- | --------------- |
//! | 1 Chokai   | 112.901732 | 116.933936 | 2109.650711    |
//! | 2 Aoba     | 91.071385  | 94.323934  | 1701.734856    |
//! | 3 Kako     | 83.99634   | 86.996211  | 1546.013588    |
//! | 4 Kinugasa | 91.071385  | 94.323934  | 1701.734856    |
//! | 5 Furutaka | 83.99634   | 86.996211  | 1546.013588    |
//! | 6 Tenryu   | 64.642826  | 66.951498  | 664.898075     |
//! | 7 Yubari   | 97.324076  | 100.799939 | 540.510569     |
//! | 8 Yunagi   | 40.972199  | 42.435493  | 260.817542     |
//!
//! This holds in every run in which the column has a ship left after its
//! fights at 01:31 and does not retire: a force that breaks off keeps to
//! its route. A column that retires after the southern fight turns back at
//! the edge of the hex and does not meet the northern group. A force
//! sent home is under way and leaving, so it does not fight again. Ships
//! left behind form a Japanese force that stays in (2,-5). It and the
//! Allied groups do not move, so they fight again at the start of the
//! morning and afternoon pulses while both have ships afloat. The column
//! itself fights nobody after 01:31.
//!
//! # Properties
//!
//! - Each pair of forces fights at most once in a pulse.
//! - No ship has negative fuel or ammunition, and no damage track is above
//!   100.
//! - Every ship is in exactly one task force, or sunk.
//! - A ship that is sunk fires in no later round.
//! - No Japanese ship is ever given a sighting bonus for radar: the column's
//!   chance is 0.5 in every sighting roll at night.
//! - A replay of any seed reproduces the daily hash.
//!
//! # What happened, for comparison only
//!
//! Canberra, Vincennes, Quincy and Astoria were sunk or scuttled. Chicago
//! was torpedoed and survived. No destroyer of either group was lost. No
//! Japanese ship was sunk; Chokai took the most hits. The test prints how
//! often each ship is sunk over seeds 1 to 1000, and asserts nothing about
//! those counts.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{
    Cause, DamageTrack, DetachReason, EngagementEndReason, Event, EventKind, Weapon,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::state::ShipLocation;
use sim_core::time::{pulse_start, Pulse};
use sim_data::Scenario;

const SEEDS: u64 = 1000;
const SAVO: Hex = Hex::new(2, -5);
const START: Hex = Hex::new(-1, -8);
const HOME: Hex = Hex::new(-1, -8);
const JAPANESE_SHIPS: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
const SOUTHERN_SHIPS: [u32; 4] = [9, 10, 11, 12];

fn fx(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn millionths(value: i64) -> Fixed {
    Fixed::from_raw(value)
}

/// A decimal string from the hand calculation, with no floats.
fn d(text: &str) -> Fixed {
    text.parse::<Fixed>().expect("a decimal in the table")
}

fn is_allied_force(force: u32) -> bool {
    force == 2 || force == 3
}

// ---------------------------------------------------------------------
// Running the scenario.
// ---------------------------------------------------------------------

fn load() -> Scenario {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../data/scenarios")
        .join("04_savo_island.toml");
    common::load(&path).expect("the scenario loads")
}

#[derive(Clone, Debug)]
struct ShipEnd {
    location: ShipLocation,
    fuel: Fixed,
    gun_rounds: Fixed,
    flotation: Fixed,
    fire: Fixed,
}

#[derive(Clone, Debug)]
struct ForceEnd {
    ships: BTreeSet<u32>,
}

#[derive(Clone, Debug)]
struct Run {
    seed: u64,
    events: Vec<Event>,
    ships: BTreeMap<u32, ShipEnd>,
    forces: BTreeMap<u32, ForceEnd>,
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
                        gun_rounds: ship.gun_rounds,
                        flotation: ship.flotation_damage,
                        fire: ship.fire_damage,
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
                        ships: force.ships.iter().map(|s| s.raw()).collect(),
                    },
                )
            })
            .collect(),
        hashes: world.day_hashes().to_vec(),
    }
}

fn batch() -> &'static Vec<Run> {
    static BATCH: OnceLock<Vec<Run>> = OnceLock::new();
    BATCH.get_or_init(|| {
        let base = load();
        (1..=SEEDS).map(|seed| run_scenario(&base, seed)).collect()
    })
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
    hexes: Vec<Hex>,
}

#[derive(Clone, Debug)]
struct Roll {
    event: u32,
    round: u32,
    force: u32,
    chance: Fixed,
    sighted: bool,
}

#[derive(Clone, Debug)]
struct Damage {
    track: DamageTrack,
    hits: u32,
    points: Fixed,
    after: Fixed,
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
    damages: Vec<Damage>,
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
    rolls: Vec<Roll>,
    salvos: Vec<SalvoRec>,
    end: Option<EngagementEndReason>,
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

    fn first_round(&self) -> Option<u32> {
        self.rolls
            .iter()
            .filter(|r| r.sighted)
            .map(|r| r.round)
            .min()
    }

    fn opening(&self) -> Opening {
        let Some(first) = self.first_round() else {
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

    fn salvos_in(&self, round: u32) -> Vec<&SalvoRec> {
        self.salvos.iter().filter(|s| s.round == round).collect()
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
                    rolls: Vec::new(),
                    salvos: Vec::new(),
                    end: None,
                }),
                EventKind::SightingRoll {
                    round,
                    task_force,
                    chance,
                    sighted,
                    ..
                } => fights
                    .last_mut()
                    .expect("a roll inside a fight")
                    .rolls
                    .push(Roll {
                        event: event.id.raw(),
                        round: *round,
                        force: task_force.raw(),
                        chance: *chance,
                        sighted: *sighted,
                    }),
                EventKind::Salvo {
                    round,
                    firer,
                    target,
                    weapon,
                    shots,
                    chance,
                    expected,
                    hits,
                } => fights
                    .last_mut()
                    .expect("a salvo inside a fight")
                    .salvos
                    .push(SalvoRec {
                        event: event.id.raw(),
                        round: *round,
                        firer: firer.raw(),
                        target: target.raw(),
                        weapon: *weapon,
                        shots: *shots,
                        chance: *chance,
                        expected: *expected,
                        hits: *hits,
                        damages: Vec::new(),
                    }),
                EventKind::ShipDamaged {
                    track,
                    hits,
                    points,
                    value_after,
                    ..
                } => {
                    let cause = event.causes.iter().find_map(|c| match c {
                        Cause::Event(id) => Some(id.raw()),
                        Cause::Order(_) => None,
                    });
                    let fight = fights.last_mut().expect("damage inside a fight");
                    let salvo = fight
                        .salvos
                        .iter_mut()
                        .find(|s| Some(s.event) == cause)
                        .expect("damage names its salvo");
                    salvo.damages.push(Damage {
                        track: *track,
                        hits: *hits,
                        points: *points,
                        after: *value_after,
                    });
                }
                EventKind::EngagementEnded { reason, .. } => {
                    fights.last_mut().expect("an end of a fight").end = Some(*reason);
                }
                _ => {}
            }
        }
        fights
    }

    fn night_fights(&self) -> Vec<Fight> {
        self.fights().into_iter().filter(Fight::night).collect()
    }

    /// The fight of the column and the southern group: the first of the run.
    fn southern_fight(&self) -> Fight {
        let fights = self.fights();
        let first = fights.into_iter().next().expect("a fight");
        assert_eq!(
            (first.a, first.b),
            (1, 2),
            "seed {}: first fight",
            self.seed
        );
        first
    }

    fn northern_fight(&self) -> Option<Fight> {
        self.night_fights()
            .into_iter()
            .find(|f| (f.a, f.b) == (1, 3))
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
                    hexes_entered,
                } if task_force.raw() == force => Some(Step {
                    event: event.id.raw(),
                    pulse: event.pulse.expect("a step has a pulse"),
                    from: *from,
                    to: *to,
                    distance: *distance_nm,
                    speed: *speed_kn,
                    hexes: hexes_entered.clone(),
                }),
                _ => None,
            })
            .collect()
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

    /// Fuel burned by a ship in events after `after`, in the night pulse or
    /// not.
    fn burned(&self, ship: u32, after: u32, night: bool) -> Fixed {
        self.events
            .iter()
            .filter(|e| e.id.raw() > after && (e.pulse == Some(Pulse::Night)) == night)
            .filter_map(|e| match &e.kind {
                EventKind::FuelBurned {
                    ship: s, burned_t, ..
                } if s.raw() == ship => Some(*burned_t),
                _ => None,
            })
            .fold(Fixed::ZERO, |sum, t| sum + t)
    }

    /// Whether `force` was alert in the night before event `before`: it
    /// became alert in a fight, or passed its alarm roll.
    fn alert_before(&self, force: u32, before: u32) -> bool {
        self.events.iter().any(|e| {
            e.id.raw() < before
                && match &e.kind {
                    EventKind::ForceBecameAlert { task_force, .. }
                    | EventKind::ForceAlertedByContact { task_force, .. } => {
                        task_force.raw() == force
                    }
                    EventKind::AlarmRoll {
                        task_force,
                        alarmed,
                        ..
                    } => task_force.raw() == force && *alarmed,
                    _ => false,
                }
        })
    }

    fn alarm_rolls_for_3(&self) -> Vec<(u32, Fixed, bool)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::AlarmRoll {
                    task_force,
                    by_a,
                    by_b,
                    chance,
                    alarmed,
                } if task_force.raw() == 3 && by_a.raw() == 1 && by_b.raw() == 2 => {
                    Some((e.id.raw(), *chance, *alarmed))
                }
                _ => None,
            })
            .collect()
    }

    fn sunk(&self, ship: u32) -> bool {
        self.ships[&ship].location == ShipLocation::Sunk
    }

    /// Ships of the column that remain in task force 1 at the end.
    fn column_remaining(&self) -> BTreeSet<u32> {
        self.forces
            .get(&1)
            .map(|f| f.ships.clone())
            .unwrap_or_default()
    }

    fn detachments(&self, reason: DetachReason) -> Vec<(u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::ShipsDetached {
                    new_force,
                    reason: r,
                    ..
                } if *r == reason => Some((e.id.raw(), new_force.raw())),
                _ => None,
            })
            .collect()
    }
}

// ---------------------------------------------------------------------
// The approach.
// ---------------------------------------------------------------------

/// Burned by 01:31 and left, from the table in the hand calculation.
const APPROACH: [(u32, &str, &str); 8] = [
    (1, "189.513621", "2339.486379"),
    (2, "152.869825", "1887.130175"),
    (3, "140.993861", "1717.006139"),
    (4, "152.869825", "1887.130175"),
    (5, "140.993861", "1717.006139"),
    (6, "108.507601", "796.492399"),
    (7, "163.365416", "738.634584"),
    (8, "68.774766", "344.225234"),
];

#[test]
fn the_column_steams_188_nm_at_25_kn_into_2_m5() {
    for run in batch() {
        let seed = run.seed;
        let steps = run.steps(1);
        let first = &steps[0];
        assert_eq!(first.pulse, Pulse::Night, "seed {seed}");
        assert_eq!(first.from, START, "seed {seed}");
        assert_eq!(first.to, SAVO, "seed {seed}");
        assert_eq!(first.distance, fx(188), "seed {seed}");
        assert_eq!(first.speed, fx(25), "seed {seed}");
        assert_eq!(
            first.hexes,
            vec![
                Hex::new(0, -8),
                Hex::new(1, -8),
                Hex::new(1, -7),
                Hex::new(2, -7),
                Hex::new(2, -6),
                Hex::new(2, -5)
            ],
            "seed {seed}"
        );
        // The eight fuel figures, in ship order, right after the step.
        let burns: Vec<(u32, Fixed, Fixed, u32)> = run
            .events
            .iter()
            .filter(|e| e.id.raw() > first.event)
            .map_while(|e| match &e.kind {
                EventKind::FuelBurned {
                    ship,
                    burned_t,
                    remaining_t,
                    ..
                } => Some((ship.raw(), *burned_t, *remaining_t, e.id.raw())),
                _ => None,
            })
            .collect();
        assert_eq!(burns.len(), 8, "seed {seed}");
        for (burn, (ship, burned, left)) in
            burns.iter().zip(APPROACH.iter().map(|r| (r.0, r.1, r.2)))
        {
            assert_eq!(burn.0, ship, "seed {seed}");
            assert_eq!(burn.1, d(burned), "seed {seed}: ship {ship}");
            assert_eq!(burn.2, d(left), "seed {seed}: ship {ship}");
        }
    }
}

#[test]
fn the_first_fight_is_with_task_force_2_at_hour_7_52_and_nobody_is_alert() {
    for run in batch() {
        let seed = run.seed;
        let fights = run.fights();
        let first = &fights[0];
        assert_eq!((first.a, first.b), (1, 2), "seed {seed}");
        assert_eq!(first.hex, SAVO, "seed {seed}");
        assert_eq!(first.elapsed, millionths(7_520_000), "seed {seed}");
        assert!(first.night(), "seed {seed}");
        // The Allied contact is seven hexes away: no alert before the fight.
        let alert_before = run.events.iter().any(|e| {
            e.id.raw() < first.event
                && matches!(
                    e.kind,
                    EventKind::ForceBecameAlert { .. } | EventKind::ForceAlertedByContact { .. }
                )
        });
        assert!(
            !alert_before,
            "seed {seed}: an alert before the first fight"
        );
        // The Allied round-1 sighting chance is 0.35.
        let round_1: Vec<&Roll> = first
            .rolls
            .iter()
            .filter(|r| r.round == 1 && r.force == 2)
            .collect();
        assert_eq!(round_1.len(), 1, "seed {seed}");
        assert_eq!(round_1[0].chance, d("0.35"), "seed {seed}");
        // The northern fight, if any, is at the same moment and after.
        for fight in run.night_fights().iter().filter(|f| (f.a, f.b) == (1, 3)) {
            assert_eq!(fight.elapsed, millionths(7_520_000), "seed {seed}");
            assert!(fight.event > first.event, "seed {seed}");
        }
    }
}

// ---------------------------------------------------------------------
// Who sees whom.
// ---------------------------------------------------------------------

#[test]
fn every_night_sighting_chance_matches_the_table() {
    for run in batch() {
        let seed = run.seed;
        for fight in run.night_fights() {
            for roll in &fight.rolls {
                let expected = if !is_allied_force(roll.force) {
                    d("0.5")
                } else if run.alert_before(roll.force, roll.event) {
                    d("0.7")
                } else {
                    d("0.35")
                };
                assert_eq!(
                    roll.chance,
                    expected,
                    "seed {seed}: force {} round {} in fight {:?}",
                    roll.force,
                    roll.round,
                    (fight.a, fight.b)
                );
            }
        }
        // The northern group's chance in round 1 of its own fight is 0.7 if
        // it was alarmed, else 0.35.
        if let Some(fight) = run.northern_fight() {
            let alarmed = run.alarm_rolls_for_3().iter().any(|r| r.2);
            let roll = fight
                .rolls
                .iter()
                .find(|r| r.round == 1 && r.force == 3)
                .expect("a round-1 roll");
            let want = if alarmed { d("0.7") } else { d("0.35") };
            assert_eq!(roll.chance, want, "seed {seed}");
        }
    }
}

#[test]
fn the_opening_counts_of_the_southern_fight_fall_in_their_bands() {
    let mut counts = [0usize; 4];
    for run in batch() {
        let slot = match run.southern_fight().opening() {
            Opening::AFreeRound => 0,
            Opening::BFreeRound => 1,
            Opening::BothFire => 2,
            Opening::PassUnseen => 3,
        };
        counts[slot] += 1;
    }
    eprintln!(
        "southern fight: Japanese free {}, southern free {}, both {}, unseen {}",
        counts[0], counts[1], counts[2], counts[3]
    );
    assert!(
        (418..=544).contains(&counts[0]),
        "Japanese free {}",
        counts[0]
    );
    assert!(
        (204..=314).contains(&counts[1]),
        "southern free {}",
        counts[1]
    );
    assert!((204..=314).contains(&counts[2]), "both {}", counts[2]);
    assert!(counts[3] <= 5, "unseen {}", counts[3]);
    assert_eq!(counts.iter().sum::<usize>(), 1000);
}

#[test]
fn the_alarm_is_rolled_once_when_shots_were_fired_in_the_southern_fight() {
    let mut rolls = 0i64;
    let mut alarmed = 0i64;
    for run in batch() {
        let seed = run.seed;
        let fight = run.southern_fight();
        let alarm = run.alarm_rolls_for_3();
        if fight.salvos.is_empty() {
            assert!(alarm.is_empty(), "seed {seed}: an alarm roll without shots");
        } else {
            assert_eq!(alarm.len(), 1, "seed {seed}");
            assert_eq!(alarm[0].1, d("0.5"), "seed {seed}");
            if let Some(north) = run.northern_fight() {
                assert!(
                    alarm[0].0 < north.event,
                    "seed {seed}: alarm before the fight"
                );
            }
            rolls += 1;
            alarmed += i64::from(alarm[0].2);
        }
    }
    eprintln!("alarm: {alarmed} alarmed in {rolls} rolls");
    // Within four standard deviations of half: |a - n/2| <= 4 x sqrt(n)/2.
    assert!(
        (2 * alarmed - rolls).pow(2) <= 16 * rolls,
        "{alarmed} of {rolls}"
    );
}

// ---------------------------------------------------------------------
// The first salvos.
// ---------------------------------------------------------------------

/// Firer, target, gun shots, expected hits, points a hit, tubes, expected
/// hits, points a hit.
type Row = (
    u32,
    u32,
    i64,
    &'static str,
    &'static str,
    i64,
    &'static str,
    &'static str,
);

const JAPANESE_FIRST: [Row; 8] = [
    (1, 9, 120, "5.4", "5.378205", 8, "0.48", "38.461538"),
    (2, 10, 72, "3.24", "5.794198", 8, "0.48", "54.143646"),
    (3, 11, 72, "3.24", "32.2891", 8, "0.48", "301.724137"),
    (4, 12, 72, "3.24", "32.2891", 8, "0.48", "301.724137"),
    (5, 9, 72, "3.24", "5.378205", 8, "0.48", "50.25641"),
    (6, 10, 120, "5.4", "1.749539", 6, "0.36", "22.099447"),
    (7, 11, 180, "8.1", "9.749588", 4, "0.24", "212.438423"),
    (8, 12, 110, "4.95", "5.208332", 6, "0.36", "123.152709"),
];

const SOUTHERN_FIRST: [Row; 4] = [
    (9, 1, 140, "6.3", "4.262114", 8, "0.48", "29.577092"),
    (10, 2, 135, "6.075", "5.545834", 0, "", ""),
    (11, 3, 300, "13.5", "1.216473", 16, "0.96", "26.170734"),
    (12, 4, 300, "13.5", "1.175961", 16, "0.96", "25.299164"),
];

/// One salvo against one row of a table. `seed` is for messages.
fn check_salvo(
    seed: u64,
    salvo: &SalvoRec,
    target: u32,
    shots: Fixed,
    chance: &str,
    expected: &str,
    pph: &str,
) {
    let who = format!(
        "seed {seed}: ship {} {:?} round {}",
        salvo.firer, salvo.weapon, salvo.round
    );
    assert_eq!(salvo.target, target, "{who}");
    assert_eq!(salvo.shots, shots, "{who}");
    assert_eq!(salvo.chance, d(chance), "{who}");
    assert_eq!(salvo.expected, d(expected), "{who}");
    assert_eq!(salvo.shots * salvo.chance, salvo.expected, "{who}");
    // Each shot is rolled on its own (decision 0033): the hits are some
    // number of the shots; the spread is checked over the runs.
    assert!(
        Fixed::from_int(i64::from(salvo.hits)) <= shots,
        "{who}: {} hits of {expected} expected",
        salvo.hits
    );
    // The damage events of the salvo carry the table's points a hit.
    let total: u32 = salvo.damages.iter().map(|x| x.hits).sum();
    assert_eq!(total, salvo.hits, "{who}: hits on the tracks");
    for damage in &salvo.damages {
        assert_eq!(
            damage.points,
            d(pph) * i64::from(damage.hits),
            "{who}: {} hits on {:?}",
            damage.hits,
            damage.track
        );
        // A track is capped at 100, but the points reported are not.
        assert!(damage.after <= fx(100), "{who}");
    }
}

/// Hits, expected hits (millionths) and variance (1e-12) of one salvo of a
/// table, summed over the runs.
#[derive(Default)]
struct Totals {
    hits: i128,
    expected: i128,
    variance: i128,
}

type Sums = BTreeMap<(u32, Weapon), Totals>;

fn add_totals(sums: &mut Sums, salvo: &SalvoRec) {
    let totals = sums.entry((salvo.firer, salvo.weapon)).or_default();
    let p = i128::from(salvo.chance.raw());
    totals.hits += i128::from(salvo.hits);
    totals.expected += i128::from(salvo.expected.raw());
    // n x p x (1 - p), with n whole shots and p in millionths.
    totals.variance += i128::from(salvo.shots.trunc()) * p * (1_000_000 - p);
}

/// Over the runs, the hits of each salvo are within four standard
/// deviations of the expected figure times the runs: (H - E)^2 <= 16 x var.
fn check_four_sigma(sums: &Sums) {
    for ((firer, weapon), t) in sums {
        let diff = t.hits * 1_000_000 - t.expected;
        assert!(
            diff * diff <= 16 * t.variance,
            "ship {firer} {weapon:?}: {} hits, {} expected (millionths), variance {}",
            t.hits,
            t.expected,
            t.variance
        );
    }
}

fn check_free_round(seed: u64, fight: &Fight, rows: &[Row], sums: &mut Sums) {
    let round = fight.first_round().expect("somebody sighted");
    let in_round = fight.salvos_in(round);
    let mut expected_count = 0;
    for row in rows {
        let (firer, target, gun_shots, gun_exp, gun_pph, tubes, torp_exp, torp_pph) = *row;
        let guns: Vec<&&SalvoRec> = in_round
            .iter()
            .filter(|s| s.firer == firer && s.weapon == Weapon::Gun)
            .collect();
        assert_eq!(guns.len(), 1, "seed {seed}: ship {firer} guns");
        check_salvo(
            seed,
            guns[0],
            target,
            fx(gun_shots),
            "0.045",
            gun_exp,
            gun_pph,
        );
        add_totals(sums, guns[0]);
        expected_count += 1;
        let torps: Vec<&&SalvoRec> = in_round
            .iter()
            .filter(|s| s.firer == firer && s.weapon == Weapon::Torpedo)
            .collect();
        if tubes == 0 {
            assert!(torps.is_empty(), "seed {seed}: ship {firer} has no tubes");
        } else {
            assert_eq!(torps.len(), 1, "seed {seed}: ship {firer} torpedoes");
            check_salvo(
                seed,
                torps[0],
                target,
                fx(tubes),
                "0.06",
                torp_exp,
                torp_pph,
            );
            add_totals(sums, torps[0]);
            expected_count += 1;
        }
    }
    // Nobody else fires in the round: the side fired on fires nothing.
    assert_eq!(
        in_round.len(),
        expected_count,
        "seed {seed}: salvos in the free round"
    );
}

#[test]
fn the_japanese_free_round_matches_the_first_table() {
    let mut checked = 0;
    let mut sums = Sums::new();
    for run in batch() {
        let fight = run.southern_fight();
        if fight.opening() == Opening::AFreeRound {
            check_free_round(run.seed, &fight, &JAPANESE_FIRST, &mut sums);
            checked += 1;
        }
    }
    eprintln!("Japanese free rounds checked: {checked}");
    assert!(checked > 0);
    check_four_sigma(&sums);
}

#[test]
fn the_southern_free_round_matches_the_second_table() {
    let mut checked = 0;
    let mut sums = Sums::new();
    for run in batch() {
        let fight = run.southern_fight();
        if fight.opening() == Opening::BFreeRound {
            check_free_round(run.seed, &fight, &SOUTHERN_FIRST, &mut sums);
            checked += 1;
        }
    }
    eprintln!("southern free rounds checked: {checked}");
    assert!(checked > 0);
    check_four_sigma(&sums);
}

// ---------------------------------------------------------------------
// After the fights.
// ---------------------------------------------------------------------

/// Burned in 112 nm, in 116 nm, and left at the end.
const FINISH: [(u32, &str, &str, &str); 8] = [
    (1, "112.901732", "116.933936", "2109.650711"),
    (2, "91.071385", "94.323934", "1701.734856"),
    (3, "83.99634", "86.996211", "1546.013588"),
    (4, "91.071385", "94.323934", "1701.734856"),
    (5, "83.99634", "86.996211", "1546.013588"),
    (6, "64.642826", "66.951498", "664.898075"),
    (7, "97.324076", "100.799939", "540.510569"),
    (8, "40.972199", "42.435493", "260.817542"),
];

/// `skip_retired` leaves out the runs in which task force 1 retired.
fn check_column_after_the_fights() {
    let mut checked = 0;
    for run in batch() {
        let seed = run.seed;
        let remaining = run.column_remaining();
        if remaining.is_empty() {
            continue;
        }
        let retired = run
            .events
            .iter()
            .any(|e| matches!(e.kind, EventKind::ForceRetired { .. }));
        if retired {
            continue;
        }
        checked += 1;
        let last_fight = run
            .night_fights()
            .iter()
            .filter(|f| f.a == 1 || f.b == 1)
            .map(|f| f.event)
            .max()
            .expect("a fight");
        let steps = run.steps(1);
        let night_after: Vec<&Step> = steps
            .iter()
            .filter(|s| s.pulse == Pulse::Night && s.event > last_fight)
            .collect();
        let night_nm = night_after
            .iter()
            .fold(Fixed::ZERO, |sum, s| sum + s.distance);
        assert_eq!(night_nm, fx(112), "seed {seed}: rest of the night");
        assert!(
            night_after.iter().all(|s| s.speed == fx(25)),
            "seed {seed}: speed"
        );
        assert_eq!(
            night_after.last().map(|s| s.to),
            Some(Hex::new(1, -7)),
            "seed {seed}"
        );
        let day: Vec<&Step> = steps.iter().filter(|s| s.pulse != Pulse::Night).collect();
        assert!(
            day.iter().all(|s| s.speed == fx(25)),
            "seed {seed}: day speed"
        );
        assert_eq!(
            day.iter().fold(Fixed::ZERO, |sum, s| sum + s.distance),
            fx(116),
            "seed {seed}: by day"
        );
        assert_eq!(
            run.arrivals(1),
            vec![(HOME, millionths(16_640_000))],
            "seed {seed}"
        );
        for row in FINISH.iter().filter(|r| remaining.contains(&r.0)) {
            let ship = row.0;
            assert_eq!(
                run.burned(ship, last_fight, true),
                d(row.1),
                "seed {seed}: ship {ship} 112 nm"
            );
            assert_eq!(
                run.burned(ship, last_fight, false),
                d(row.2),
                "seed {seed}: ship {ship} 116 nm"
            );
            assert_eq!(
                run.ships[&ship].fuel,
                d(row.3),
                "seed {seed}: ship {ship} left"
            );
        }
        // Task force 1 fights nobody after 01:31.
        for fight in run.fights().iter().filter(|f| f.a == 1 || f.b == 1) {
            assert!(fight.night(), "seed {seed}");
            assert_eq!(fight.elapsed, millionths(7_520_000), "seed {seed}");
        }
    }
    eprintln!("columns with a ship left: {checked}");
    assert!(checked > 0);
}

#[test]
fn left_behind_forces_fight_only_allies_at_day_starts_and_sent_home_forces_never() {
    for run in batch() {
        let seed = run.seed;
        let fights = run.fights();
        for (detached, force) in run.detachments(DetachReason::LeftBehind) {
            for fight in fights.iter().filter(|f| f.a == force || f.b == force) {
                assert!(fight.event > detached, "seed {seed}");
                assert_eq!(fight.day, 1, "seed {seed}");
                assert!(
                    fight.pulse != Pulse::Night
                        && fight.elapsed == pulse_start(fight.day, fight.pulse),
                    "seed {seed}: force {force} fought at {:?}",
                    fight.elapsed
                );
                let other = if fight.a == force { fight.b } else { fight.a };
                assert!(is_allied_force(other), "seed {seed}");
            }
        }
        for (detached, force) in run.detachments(DetachReason::SentHome) {
            assert!(
                fights
                    .iter()
                    .all(|f| !(f.event > detached && (f.a == force || f.b == force))),
                "seed {seed}: sent-home force {force} fought"
            );
        }
    }
}

/// Every fight in this scenario is in the hex of Savo Island: the Allied
/// groups never leave it, and a meeting is held only where both forces are.
/// A column that retires after the southern fight turns back out of the hex
/// and does not meet the northern group.
#[test]
fn every_fight_is_in_the_hex_of_savo_island() {
    let mut retired_and_missed_the_northern_group = 0;
    for run in batch() {
        let seed = run.seed;
        let fights = run.fights();
        for fight in &fights {
            assert_eq!(fight.hex, Hex::new(2, -5), "seed {seed}: a fight elsewhere");
        }
        let night_with_3 = fights
            .iter()
            .any(|f| f.pulse == Pulse::Night && (f.a, f.b) == (1, 3));
        if !night_with_3 {
            retired_and_missed_the_northern_group += 1;
        }
    }
    assert!(
        retired_and_missed_the_northern_group > 0,
        "in some run the column does not meet the northern group"
    );
}

/// The check over the runs in which the column did not retire.
#[test]
fn a_column_that_did_not_retire_steams_112_then_116_nm_at_25_kn_and_arrives_at_hour_16_64() {
    check_column_after_the_fights();
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn a_pair_fights_at_most_once_in_a_pulse() {
    for run in batch() {
        let mut seen = BTreeSet::new();
        for f in run.fights() {
            let key = (f.day, f.pulse.code(), f.a.min(f.b), f.a.max(f.b));
            assert!(seen.insert(key), "seed {}: {key:?} twice", run.seed);
        }
    }
}

#[test]
fn no_negative_fuel_or_ammunition_and_no_track_above_100() {
    for run in batch() {
        let seed = run.seed;
        for (id, ship) in &run.ships {
            assert!(!ship.fuel.is_negative(), "seed {seed}: ship {id} fuel");
            assert!(
                !ship.gun_rounds.is_negative(),
                "seed {seed}: ship {id} rounds"
            );
            assert!(
                ship.flotation <= fx(100) && ship.fire <= fx(100),
                "seed {seed}: ship {id}"
            );
        }
        for event in &run.events {
            match &event.kind {
                EventKind::FuelBurned { remaining_t, .. } => {
                    assert!(!remaining_t.is_negative(), "seed {seed}");
                }
                EventKind::ShipDamaged { value_after, .. } => {
                    assert!(*value_after <= fx(100), "seed {seed}");
                }
                _ => {}
            }
        }
    }
}

#[test]
fn every_ship_is_in_exactly_one_task_force_or_sunk() {
    for run in batch() {
        let seed = run.seed;
        for (id, ship) in &run.ships {
            let holders: Vec<u32> = run
                .forces
                .iter()
                .filter(|(_, f)| f.ships.contains(id))
                .map(|(fid, _)| *fid)
                .collect();
            match ship.location {
                ShipLocation::TaskForce(force) => {
                    assert_eq!(holders, vec![force.raw()], "seed {seed}: ship {id}");
                }
                ShipLocation::Sunk => assert!(holders.is_empty(), "seed {seed}: ship {id}"),
                ShipLocation::Port(_) => panic!("seed {seed}: ship {id} in port"),
            }
        }
    }
}

#[test]
fn a_sunk_ship_fires_in_no_later_round() {
    for run in batch() {
        let sunk: Vec<(u32, u32)> = run
            .events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::ShipSunk { ship, .. } => Some((ship.raw(), e.id.raw())),
                _ => None,
            })
            .collect();
        for e in &run.events {
            if let EventKind::Salvo { firer, .. } = &e.kind {
                for (ship, at) in &sunk {
                    assert!(
                        !(ship == &firer.raw() && e.id.raw() > *at),
                        "seed {}: sunk ship {ship} fired",
                        run.seed
                    );
                }
            }
        }
    }
}

#[test]
fn no_japanese_sighting_roll_at_night_has_a_radar_bonus() {
    for run in batch() {
        for e in &run.events {
            if let EventKind::SightingRoll {
                task_force, chance, ..
            } = &e.kind
            {
                if e.pulse == Some(Pulse::Night) && !is_allied_force(task_force.raw()) {
                    assert_eq!(*chance, d("0.5"), "seed {}", run.seed);
                }
            }
        }
    }
}

#[test]
fn a_replay_reproduces_the_daily_hash() {
    let base = load();
    let mut all: Vec<Vec<u64>> = Vec::new();
    for seed in (1..=1000u64).step_by(50) {
        let again = run_scenario(&base, seed);
        let first = &batch()[usize::try_from(seed - 1).expect("small")];
        assert_eq!(first.seed, seed);
        assert_eq!(first.hashes.len(), 1);
        assert_eq!(first.hashes, again.hashes, "seed {seed}");
        all.push(again.hashes);
    }
    assert!(all.len() >= 20);
    assert!(all.windows(2).any(|pair| pair[0] != pair[1]));
}

// ---------------------------------------------------------------------
// The summary.
// ---------------------------------------------------------------------

#[test]
fn print_the_summary() {
    let runs = batch();
    let names = [
        "Chokai",
        "Aoba",
        "Kako",
        "Kinugasa",
        "Furutaka",
        "Tenryu",
        "Yubari",
        "Yunagi",
        "Canberra",
        "Chicago",
        "Patterson",
        "Bagley",
        "Vincennes",
        "Quincy",
        "Astoria",
        "Helm",
        "Wilson",
    ];
    println!("Savo Island over seeds 1 to {SEEDS}: how often each ship ends sunk");
    for (index, name) in names.iter().enumerate() {
        let ship = u32::try_from(index + 1).expect("small");
        let count = runs.iter().filter(|r| r.sunk(ship)).count();
        println!("  {ship:>2} {name:<10} {count:>4}");
    }
    let mut japanese = [0usize; 9];
    let mut allied = [0usize; 10];
    for run in runs {
        let j = JAPANESE_SHIPS.iter().filter(|s| run.sunk(**s)).count();
        let a = (9..=17u32).filter(|s| run.sunk(*s)).count();
        japanese[j] += 1;
        allied[a] += 1;
    }
    println!("Japanese ships sunk (0 to 8): {japanese:?}");
    println!("Allied ships sunk (0 to 9): {allied:?}");
    let mut ends: BTreeMap<String, usize> = BTreeMap::new();
    for run in runs {
        for fight in run.night_fights() {
            let key = format!("{} v {}: {:?}", fight.a, fight.b, fight.end);
            *ends.entry(key).or_insert(0) += 1;
        }
    }
    println!("How the night fights ended:");
    for (key, count) in &ends {
        println!("  {key:<40} {count:>4}");
    }
    println!(
        "Historically: Canberra, Vincennes, Quincy and Astoria were sunk or scuttled; Chicago was torpedoed and survived; no destroyer of either group was lost; no Japanese ship was sunk."
    );
    let _ = SOUTHERN_SHIPS;
}
