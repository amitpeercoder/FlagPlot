//! Scenario 24: a force bombards a base on two nights, and an attack is
//! made on an opening estimate that is far too low.
//!
//! Proves decision 0092 (shells and bombs on a base and its men, and what a
//! side knows of the enemy ashore) against a hand calculation. The data is
//! placeholder data: see `data/scenarios/24_shells_and_estimates.toml`. The
//! test runs on the frozen step 1 figures (`data/reference/step1`), to which
//! the new figures are added with their first values.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide, the
//! calculation gives the shots and the chance exactly and what must follow
//! from what the run rolls; the tests read the rolls from the run's events
//! and assert the rest. Everything else is asserted exactly. If the code
//! disagrees, the code or the rule is wrong: do not change a number here to
//! make a test pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! - `surface_combat.toml`: `bombardment_minutes` 30, `bombardment_hit_chance`
//!   0.05, `bombardment_reference_gun_damage` 20,
//!   `bombardment_false_claim_chance` 0.01.
//! - `air_combat.toml`, frozen: `[base_hit]` aircraft 0.3, supply 0.3, and
//!   the new men 0.2, so 0.2 of hits land on nothing;
//!   `base_supply_destroyed_per_hit_t` 50.
//! - `ground.toml`: `men_lost_per_base_hit` 10, `report_error_low` 0.5,
//!   `report_error_high` 1.5, and the figures of stages 1 and 2
//!   (`daily_supply_t_per_man` 0.005, `men_per_shot` 100, `hit_chance` 0.3,
//!   `defender_hit_chance` 0.6, `men_per_hit` 20, `fatigue_per_attack_day`
//!   20, `attack_fatigue_limit` 60, `beaten_share` 0.2).
//! - Classes, frozen: the placeholder cruiser has 8 guns at 3 rounds a
//!   minute, 100 rounds a gun and a gun damage of 20; it cruises at 16 kn and
//!   burns 2000 t over 8000 nm, 0.25 t a mile. The placeholder destroyer has
//!   4 guns at 6 rounds a minute, 150 rounds a gun and a gun damage of 5; it
//!   cruises at 16 kn and burns 0.125 t a mile.
//!
//! ## Inputs
//!
//! A placeholder map with two coast hexes on one row, X at (0,0) and Y at
//! (8,0), and sea between. The game is 4 days.
//!
//! | Base | Side     | Hex | Supply | On the enemy's map |
//! | ---- | -------- | --- | ------ | ------------------ |
//! | 1    | Allied   | X   | 2000 t | yes                |
//! | 2    | Allied   | Y   | 1000 t | yes                |
//! | 3    | Japanese | Y   | 100 t  | no                 |
//!
//! - Allied air group 1 at base 1: placeholder fighters, 20 ready and 4
//!   damaged, no mission. Base 1 repairs no aircraft and has no guns.
//! - Allied land unit 1: 1000 men ashore at X, on defend.
//! - Allied land unit 2: 3000 men ashore at Y, on defend.
//! - Japanese land unit 3: 800 men ashore at Y, on attack from the start.
//! - Japanese task force 1, mission bombardment, at sea at (2,0): ship 1, a
//!   placeholder cruiser with 800 rounds, and ship 2, a placeholder
//!   destroyer with 600 rounds. Both are full of fuel: 2000 t and 500 t.
//! - The Japanese side's opening estimate: 300 Allied men at Y.
//!
//! Orders, Japanese: on day 1, task force 1 to (0,0) at cruise; on day 3,
//! task force 1 to (2,0) at cruise. There is no other order.
//!
//! The air supply figures are not loaded, so base 1 pays nothing for its
//! aircraft. Each night base 1 pays 0.005 t for each man of unit 1.
//!
//! ## X: the bombardment
//!
//! Day 1, night. The men's supply is charged first: base 1 pays 1000 x 0.005
//! = 5 t and holds 1995 t. Then the force steams 2 hexes, 80 nm, at 16 kn,
//! and arrives at (0,0) 5 hours into the pulse. The cruiser burns 20 t and
//! has 1980 t; the destroyer burns 10 t and has 490 t. Its position after
//! movement holds base 1, an enemy base on its map: it bombards.
//!
//! | Night | Ship      | Shells: rate x guns x 30 | Rounds aboard | Shells fired | Weighed shots        | Rounds left |
//! | ----- | --------- | ------------------------ | ------------- | ------------ | -------------------- | ----------- |
//! | 1     | Cruiser   | 3 x 8 x 30 = 720         | 800           | 720          | 720 x 20/20 = 720    | 80          |
//! | 1     | Destroyer | 6 x 4 x 30 = 720         | 600           | 600          | 600 x 5/20 = 150     | 0           |
//! | 2     | Cruiser   | 720                      | 80            | 80           | 80                   | 0           |
//! | 2     | Destroyer | 720                      | 0             | 0: it does not fire | none          | 0           |
//!
//! Every weighed shot hits at 0.05. For each hit one draw puts it on
//! aircraft (0.3), supply (0.3), men (0.2) or nothing (0.2).
//!
//! For each night, with a, s and m the hits of the whole force on aircraft,
//! supply and men, read from the run:
//!
//! - Aircraft destroyed are a, or all that are left at the base if fewer.
//!   Damaged aircraft are destroyed before ready ones. There are 24 before
//!   the first night.
//! - Supply destroyed is 50 x s t, or all that is left if less.
//! - Men lost are 10 x m, or all of unit 1 that are left if fewer.
//! - On night 1 the hits are at most 870 and on night 2 at most 80.
//!
//! The Japanese side gets one claim for each night the force fired. Its
//! aircraft claimed on the ground are the aircraft really destroyed that
//! night plus false ones, rolled from the weighed shots (870 on night 1, 80
//! on night 2) less the aircraft really destroyed, at 0.01. It is never
//! below the aircraft really destroyed. The Japanese side is told its
//! shells fired, 1320 on night 1 and 80 on night 2, and no hit.
//!
//! The Allied side gets a sighting report of the force on each night it
//! fired, and so holds a contact at (0,0) or a hex beside it. It sees the
//! shots against base 1 and where each hit landed, and is not told which
//! force or ship fired.
//!
//! Day 2. No order: the force stays at (0,0). On the night of day 2 base 1
//! pays 0.005 t for each man unit 1 has left, and the force bombards again
//! by the table's second pair of rows.
//!
//! Day 3, night. The force steams back to (2,0): 80 nm, 20 t and 10 t more,
//! so the cruiser ends with 1960 t and the destroyer with 480 t. It is not
//! in a hex with an enemy base after movement and does not bombard. It
//! would not in any case: no ship has a round left. On the nights of days 3
//! and 4 nothing is fired.
//!
//! At the end: ship 1 has 0 rounds and ship 2 has 0. Base 1's supply is 2000
//! less the four nightly charges for unit 1's men less the supply destroyed
//! on the two nights.
//!
//! ## Y: an attack on a wrong estimate
//!
//! Before day 1 the Japanese side holds an estimate of 300 Allied men at Y,
//! dated day 0. The Allied side holds no estimate, and base 3 is not on its
//! map.
//!
//! Every night both sides at Y are fed in full: base 2 pays 0.005 t for each
//! man of unit 2 and base 3 for each man of unit 3. Every fed share is 1.
//!
//! Day 1, the fight. The Japanese side attacks and the Allied side defends.
//!
//! - Unit 3: 800 firing men, 8 shots at 0.3.
//! - Unit 2: 3000 firing men, 30 shots at 0.6.
//!
//! With j the hits of unit 3 and a the hits of unit 2: unit 2 has 3000 less
//! 20 x j, which is at least 2840, and unit 3 has 800 less 20 x a, or none.
//! Unit 3's fatigue is 20.
//!
//! Beaten: only the Allied side is tested, and it has at least 2840 men
//! against 0.2 times at most 800. It is not beaten on any day. Base 2 is
//! Allied at the end and base 3 is Japanese while unit 3 has a man ashore.
//!
//! The ground reports of day 1, made after the fight if both sides still
//! have men ashore:
//!
//! - The Japanese side's estimate of Allied men at Y is replaced. With A the
//!   men unit 2 has after the fight, the new estimate is between 0.5 x A
//!   and 1.5 x A, cut to a whole man, so it is at least 1420: nearly five
//!   times the 300 it attacked on. It is dated day 1.
//! - The Allied side gets its first estimate of Japanese men at Y: between
//!   0.5 and 1.5 times the men unit 3 has after the fight, cut. Base 3 is
//!   put on the Allied map, with an event the Allied side sees.
//!
//! Each later day that both sides have men ashore at Y, unit 3 attacks again
//! with its men times 100 less its fatigue over 100, cut, as firing men
//! (fatigue 20, then 40, then 60 before the fights of days 2, 3 and 4), and
//! each side's estimate is replaced by a new report within the same bounds
//! of that day's truth. On a day unit 3 has no man left there is no fight
//! and no report, and each side keeps the estimate it had, with its day.
//!
//! If unit 3 is destroyed on day 1, no report is made at Y that day: the
//! Japanese side keeps its estimate of 300, the Allied side has none and
//! base 3 stays off its map. The test then says so and the architect picks
//! another seed.
//!
//! # Properties
//!
//! Beside these, the tests check what must always hold (decision 0092, "The
//! test"): a side's view is the same in two worlds that differ only in enemy
//! men it has no report of; a ground report always lies between the low and
//! high share of the truth; a bombardment never destroys more than is
//! there; a raid from the air on a base with no men ashore gives the hits it
//! gave before the table gained its share for men; rounds are conserved;
//! and a scenario with no bombardment mission, no men and no estimate is
//! unchanged apart from the hash version.

mod common;

use std::path::PathBuf;

use sim_core::belief::GroundEstimate;
use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EventId, ShipId};
use sim_core::side_events::{SideEvent, SideEventKind};
use sim_core::state::{AirGroupLocation, Side, TruthState};
use sim_core::time::Pulse;
use sim_core::World;
use sim_data::Scenario;

// ------------------------------------------------------------------ Helpers

const X: Hex = Hex::new(0, 0);
const Y: Hex = Hex::new(8, 0);

fn path() -> PathBuf {
    common::data_dir().join("scenarios/24_shells_and_estimates.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// The live scenario file, on the frozen step 1 figures.
fn load() -> Scenario {
    common::load(&path()).expect("scenario 24 loads")
}

fn from(source: &str) -> Scenario {
    common::from_text(source, &path()).expect("the variant loads")
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn with_seed(source: &str, seed: u32) -> String {
    common::replace_once(source, "seed = 1942\n", &format!("seed = {seed}\n"))
}

/// What a side believed at the end of a day, as its view gives it.
#[derive(Clone, Debug)]
struct Belief {
    estimates: Vec<GroundEstimate>,
    enemy_bases: Vec<BaseId>,
    contacts: Vec<Hex>,
}

/// The truth and each side's belief at the end of a day.
struct Snapshot {
    truth: TruthState,
    allied: Belief,
    japanese: Belief,
}

fn belief_of(world: &World, side: Side) -> Belief {
    let view = world.side_view(side);
    Belief {
        estimates: view.ground_estimates().copied().collect(),
        enemy_bases: view.enemy_bases().map(|base| base.id).collect(),
        contacts: view.contacts().map(|c| c.estimated_hex).collect(),
    }
}

fn snapshot(world: &World) -> Snapshot {
    Snapshot {
        truth: world.umpire().clone(),
        allied: belief_of(world, Side::Allied),
        japanese: belief_of(world, Side::Japanese),
    }
}

/// A run of a scenario: a snapshot at the start (day 0) and at the end of
/// each day, and the scenario itself at the end, with its log.
struct Run {
    scenario: Scenario,
    days: Vec<Snapshot>,
}

impl Run {
    fn of(mut scenario: Scenario) -> Run {
        let mut days = vec![snapshot(&scenario.world)];
        while scenario.next_day().expect("a day resolves").is_some() {
            days.push(snapshot(&scenario.world));
        }
        Run { scenario, days }
    }

    fn text(source: &str) -> Run {
        Run::of(from(source))
    }

    fn seed(source: &str, seed: u32) -> Run {
        Run::text(&with_seed(source, seed))
    }

    fn events(&self) -> Vec<Event> {
        self.scenario.world.events().iter().cloned().collect()
    }

    /// The number of days run.
    fn len(&self) -> u32 {
        u32::try_from(self.days.len() - 1).expect("a few days")
    }

    /// The truth at the end of `day`; day 0 is the start.
    fn at(&self, day: u32) -> &TruthState {
        &self.days[day as usize].truth
    }

    fn belief(&self, day: u32, side: Side) -> &Belief {
        match side {
            Side::Allied => &self.days[day as usize].allied,
            Side::Japanese => &self.days[day as usize].japanese,
        }
    }

    fn ship_rounds(&self, day: u32, ship: u32) -> Fixed {
        self.at(day).ships[&ShipId::new(ship)].gun_rounds
    }

    fn base_supply(&self, day: u32, base: u32) -> Fixed {
        self.at(day).bases[&BaseId::new(base)].supply_t
    }

    fn men(&self, day: u32, unit: u32) -> u32 {
        self.at(day).land_units[&sim_core::ids::LandUnitId::new(unit)].men_ashore
    }

    /// The aircraft of air group 1: (ready, damaged).
    fn aircraft(&self, day: u32) -> (u32, u32) {
        let group = &self.at(day).air_groups[&sim_core::ids::AirGroupId::new(1)];
        (group.ready, group.damaged)
    }
}

fn men_at(truth: &TruthState, side: Side, hex: Hex) -> u32 {
    truth
        .land_units
        .values()
        .filter(|unit| unit.side == side && unit.hex == Some(hex))
        .map(|unit| unit.men_ashore)
        .sum()
}

fn factor(event: &Event, name: &str) -> Fixed {
    event
        .breakdown
        .as_ref()
        .expect("a breakdown")
        .factors
        .iter()
        .find(|f| f.name == name)
        .unwrap_or_else(|| panic!("a factor {name} in {event:?}"))
        .value
}

/// The whole number of a count of hits at most as large as `shots` rounded
/// up: a part of a shot is rolled as a whole shot at a part of the chance.
fn at_most(shots: Fixed) -> u32 {
    u32::try_from((shots.trunc()) + i64::from(shots != n(shots.trunc()))).expect("a count")
}

/// One ship's fire in a bombardment, as the log has it.
#[derive(Clone, Debug)]
struct ShipFire {
    day: u32,
    id: EventId,
    ship: u32,
    base: u32,
    shells: Fixed,
    weighed: Fixed,
    chance: Fixed,
    expected: Fixed,
    hits: u32,
    on_aircraft: u32,
    on_supply: u32,
    on_men: u32,
    rounds_left: Fixed,
    causes: Vec<Cause>,
    event: Event,
}

fn ship_fires(events: &[Event]) -> Vec<ShipFire> {
    events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::BombardmentFire {
                ship,
                base,
                shells,
                weighed_shots,
                chance,
                expected,
                hits,
                hits_on_aircraft,
                hits_on_supply,
                hits_on_men,
                rounds_left,
            } => Some(ShipFire {
                day: e.day,
                id: e.id,
                ship: ship.raw(),
                base: base.raw(),
                shells: *shells,
                weighed: *weighed_shots,
                chance: *chance,
                expected: *expected,
                hits: *hits,
                on_aircraft: *hits_on_aircraft,
                on_supply: *hits_on_supply,
                on_men: *hits_on_men,
                rounds_left: *rounds_left,
                causes: e.causes.clone(),
                event: e.clone(),
            }),
            _ => None,
        })
        .collect()
}

/// A force's bombardment in a night, as the log has it.
#[derive(Clone, Debug)]
struct Shore {
    day: u32,
    pulse: Option<Pulse>,
    id: EventId,
    base: u32,
    hex: Hex,
    shells: Fixed,
    weighed: Fixed,
    hits: u32,
    on_aircraft: u32,
    on_supply: u32,
    on_men: u32,
    causes: Vec<Cause>,
}

fn shores(events: &[Event]) -> Vec<Shore> {
    events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::ShoreBombardment {
                base,
                hex,
                shells,
                weighed_shots,
                hits,
                hits_on_aircraft,
                hits_on_supply,
                hits_on_men,
                ..
            } => Some(Shore {
                day: e.day,
                pulse: e.pulse,
                id: e.id,
                base: base.raw(),
                hex: *hex,
                shells: *shells,
                weighed: *weighed_shots,
                hits: *hits,
                on_aircraft: *hits_on_aircraft,
                on_supply: *hits_on_supply,
                on_men: *hits_on_men,
                causes: e.causes.clone(),
            }),
            _ => None,
        })
        .collect()
}

/// A claim of the bombarding side: (event, aircraft claimed on the ground).
fn bombardment_claims(events: &[Event], side: Side) -> Vec<(Event, u32)> {
    events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::ClaimReceived {
                side: s,
                from_bombardment: true,
                claimed_on_ground,
                ..
            } if *s == side => Some((e.clone(), *claimed_on_ground)),
            _ => None,
        })
        .collect()
}

/// The supply a base paid for its men on the night of `day`: what was asked
/// and what was paid.
fn men_charge(events: &[Event], day: u32, base: u32) -> Option<(Fixed, Fixed, u32)> {
    events.iter().find_map(|e| match &e.kind {
        EventKind::BaseSupplySpent {
            base: b,
            reason: sim_core::events::SupplyUse::Men,
            men,
            asked_t,
            paid_t,
            ..
        } if e.day == day && b.raw() == base => Some((*asked_t, *paid_t, *men)),
        _ => None,
    })
}

/// What each kind of limit bit in a run's bombardments: a hit on aircraft
/// destroyed fewer than a hit would, and so on.
#[derive(Default, Debug)]
struct Caps {
    aircraft: bool,
    supply: bool,
    men: bool,
    fired_nights: u32,
}

/// Asserts, for every night of a run of scenario 24 or a variant of it, what
/// the hand calculation says follows from the hits the run rolls:
/// the rounds are conserved, the force's fire is its ships' added, aircraft
/// destroyed are the hits on aircraft or all that are left (damaged before
/// ready), supply is 50 t a hit or all that is left, men are 10 a hit or all
/// of unit 1 that are left, and the claim is the aircraft really destroyed
/// plus false ones. Air group 1, base 1 and unit 1 are the ones shelled.
fn check_nights(run: &Run) -> Caps {
    let events = run.events();
    let fires = ship_fires(&events);
    let all_shores = shores(&events);
    let claims = bombardment_claims(&events, Side::Japanese);
    let mut caps = Caps::default();
    for day in 1..=run.len() {
        let (before, after) = (run.at(day - 1), run.at(day));
        let night_fires: Vec<&ShipFire> = fires.iter().filter(|f| f.day == day).collect();
        let night_shores: Vec<&Shore> = all_shores.iter().filter(|s| s.day == day).collect();

        // Rounds are conserved: rounds before = rounds after + shells, and
        // nothing is fired by a ship that is not named.
        for ship in before.ships.keys() {
            let fired: Fixed = night_fires
                .iter()
                .filter(|f| f.ship == ship.raw())
                .map(|f| f.shells)
                .fold(Fixed::ZERO, |a, b| a + b);
            assert_eq!(
                before.ships[ship].gun_rounds,
                after.ships[ship].gun_rounds + fired,
                "day {day}: rounds of {ship:?} are conserved"
            );
        }
        for fire in &night_fires {
            assert_eq!(fire.event.pulse, Some(Pulse::Night));
            assert_eq!(fire.base, 1);
            assert_eq!(
                factor(&fire.event, "rounds_aboard"),
                before.ships[&ShipId::new(fire.ship)].gun_rounds
            );
            assert_eq!(
                fire.rounds_left,
                factor(&fire.event, "rounds_aboard") - fire.shells
            );
            assert_eq!(
                fire.rounds_left,
                after.ships[&ShipId::new(fire.ship)].gun_rounds
            );
            assert!(
                fire.shells > Fixed::ZERO,
                "a ship with nothing to fire is silent"
            );
            assert!(fire.rounds_left >= Fixed::ZERO);
            assert_eq!(fire.expected, fire.weighed * fire.chance);
            // Why the force is there: the order that sent it and its arrival.
            assert!(fire.causes.iter().any(|c| matches!(c, Cause::Order(_))));
            assert!(fire.causes.iter().any(|c| matches!(c, Cause::Event(_))));
            assert!(fire.hits <= at_most(fire.weighed), "{fire:?}");
            assert!(fire.on_aircraft + fire.on_supply + fire.on_men <= fire.hits);
        }

        // The base pays for its men first.
        let men_before = run.men(day - 1, 1);
        let stock_before = run.base_supply(day - 1, 1);
        let asked = fx("0.005") * n(i64::from(men_before));
        let paid = match men_charge(&events, day, 1) {
            Some((asked_t, paid_t, charged_men)) => {
                assert_eq!(charged_men, men_before);
                assert_eq!(asked_t, asked);
                paid_t
            }
            // No man or no supply: nothing is asked or paid.
            None => {
                assert!(men_before == 0 || stock_before == Fixed::ZERO, "day {day}");
                Fixed::ZERO
            }
        };
        assert_eq!(paid, asked.min(stock_before));
        let stock = run.base_supply(day - 1, 1) - paid;

        if night_fires.is_empty() {
            // Nothing fired: no force bombardment, no claim, no hit.
            assert!(night_shores.is_empty(), "day {day}");
            assert!(
                !claims.iter().any(|(e, _)| e.day == day),
                "day {day}: no claim"
            );
            assert_eq!(run.aircraft(day), run.aircraft(day - 1), "day {day}");
            assert_eq!(run.base_supply(day, 1), stock, "day {day}");
            assert_eq!(run.men(day, 1), men_before, "day {day}");
            continue;
        }
        caps.fired_nights += 1;

        // The force's fire is its ships' fire, added, once.
        assert_eq!(night_shores.len(), 1, "day {day}");
        let shore = night_shores[0];
        assert_eq!(shore.pulse, Some(Pulse::Night));
        assert_eq!((shore.base, shore.hex), (1, X));
        let sum = |pick: fn(&ShipFire) -> u32| night_fires.iter().map(|f| pick(f)).sum::<u32>();
        assert_eq!(shore.hits, sum(|f| f.hits));
        assert_eq!(shore.on_aircraft, sum(|f| f.on_aircraft));
        assert_eq!(shore.on_supply, sum(|f| f.on_supply));
        assert_eq!(shore.on_men, sum(|f| f.on_men));
        assert_eq!(
            shore.shells,
            night_fires.iter().fold(Fixed::ZERO, |a, f| a + f.shells)
        );
        assert_eq!(
            shore.weighed,
            night_fires.iter().fold(Fixed::ZERO, |a, f| a + f.weighed)
        );
        // Each ship's fire is a cause of the force's.
        for fire in &night_fires {
            assert!(shore.causes.contains(&Cause::Event(fire.id)));
        }
        if men_before == 0 {
            assert_eq!(shore.on_men, 0, "the share for men lands on nothing");
        }

        // Aircraft: damaged before ready, as many as hit or all there are.
        let (ready, damaged) = run.aircraft(day - 1);
        let on_ground = ready + damaged;
        let destroyed_damaged = shore.on_aircraft.min(damaged);
        let destroyed_ready = (shore.on_aircraft - destroyed_damaged).min(ready);
        assert_eq!(
            run.aircraft(day),
            (ready - destroyed_ready, damaged - destroyed_damaged),
            "day {day}"
        );
        let real = destroyed_damaged + destroyed_ready;
        assert_eq!(real, shore.on_aircraft.min(on_ground));
        caps.aircraft |= shore.on_aircraft > on_ground;
        let seen_destroyed: u32 = events
            .iter()
            .filter(|e| e.day == day)
            .filter_map(|e| match &e.kind {
                EventKind::AircraftDestroyedOnGround { aircraft, .. }
                | EventKind::DamagedAircraftDestroyedOnGround { aircraft, .. } => {
                    assert!(e.causes.contains(&Cause::Event(shore.id)));
                    Some(*aircraft)
                }
                _ => None,
            })
            .sum();
        assert_eq!(seen_destroyed, real, "day {day}");

        // Supply: 50 t a hit, or all that is left.
        let wanted = n(50) * n(i64::from(shore.on_supply));
        let destroyed_t = wanted.min(stock);
        caps.supply |= wanted > stock;
        assert_eq!(run.base_supply(day, 1), stock - destroyed_t, "day {day}");
        let seen_t = events.iter().find_map(|e| match &e.kind {
            EventKind::BaseSupplyDestroyed {
                destroyed_t,
                supply_t,
                ..
            } if e.day == day => {
                assert!(e.causes.contains(&Cause::Event(shore.id)));
                Some((*destroyed_t, *supply_t))
            }
            _ => None,
        });
        if destroyed_t > Fixed::ZERO {
            assert_eq!(seen_t, Some((destroyed_t, stock - destroyed_t)));
        } else {
            assert_eq!(seen_t, None);
        }

        // Men: 10 a hit, or all of unit 1 that are left.
        let lost = (10 * shore.on_men).min(men_before);
        caps.men |= 10 * shore.on_men > men_before;
        assert_eq!(run.men(day, 1), men_before - lost, "day {day}");
        let seen_men = events.iter().find_map(|e| match &e.kind {
            EventKind::MenLostToFire {
                unit,
                base,
                hex,
                lost,
                ashore,
            } if e.day == day => {
                assert_eq!((unit.raw(), base.raw(), *hex), (1, 1, X));
                assert!(e.causes.contains(&Cause::Event(shore.id)));
                Some((*lost, *ashore))
            }
            _ => None,
        });
        if lost > 0 {
            assert_eq!(seen_men, Some((lost, men_before - lost)));
        } else {
            assert_eq!(seen_men, None);
        }
        if men_before - lost == 0 {
            assert_eq!(
                after.land_units[&sim_core::ids::LandUnitId::new(1)].hex,
                None,
                "a unit with no man left is gone from the hex"
            );
        }
        // Y is untouched by fire.
        assert_eq!(run.men(day, 1) + lost, men_before);

        // The claim: one for the night, the aircraft really destroyed plus
        // false ones rolled from the weighed shots less those, at 0.01.
        let night_claims: Vec<&(Event, u32)> =
            claims.iter().filter(|(e, _)| e.day == day).collect();
        assert_eq!(night_claims.len(), 1, "day {day}: one claim");
        let (claim, claimed) = night_claims[0];
        assert_eq!(claim.pulse, Some(Pulse::Night));
        assert_eq!(factor(claim, "real_on_ground"), n(i64::from(real)));
        assert_eq!(factor(claim, "weighed_shots"), shore.weighed);
        assert_eq!(factor(claim, "false_claim_chance"), fx("0.01"));
        let false_claims = factor(claim, "false_on_ground");
        assert_eq!(
            *claimed,
            real + u32::try_from(false_claims.trunc()).unwrap()
        );
        assert!(*claimed >= real, "never below what really happened");
        assert!(
            u32::try_from(false_claims.trunc()).unwrap()
                <= at_most((shore.weighed - n(i64::from(real))).max(Fixed::ZERO))
        );
        assert!(matches!(
            &claim.kind,
            EventKind::ClaimReceived {
                side: Side::Japanese,
                hex,
                claimed_sunk: 0,
                claimed_hits: 0,
                claimed_kind: None,
                claimed_shot_down: 0,
                from_air: false,
                from_bombardment: true,
                ..
            } if *hex == X
        ));
        for fire in &night_fires {
            assert!(claim.causes.contains(&Cause::Event(fire.id)));
        }
    }
    caps
}

// ------------------------------------------------ The hand calculation: X

/// The dice of the run, read from the log: each night's hits by ship.
#[test]
fn x_each_ships_fire_is_the_table_of_the_hand_calculation() {
    let run = Run::of(load());
    let events = run.events();
    let fires = ship_fires(&events);
    let on = |day: u32, ship: u32| fires.iter().find(|f| f.day == day && f.ship == ship);

    // Night 1, cruiser: 3 x 8 x 30 = 720 shells of 800, 720 x 20/20 = 720
    // weighed shots at 0.05, 80 rounds left.
    let cruiser = on(1, 1).expect("the cruiser fires on night 1");
    assert_eq!(cruiser.shells, n(720));
    assert_eq!(cruiser.weighed, n(720));
    assert_eq!(cruiser.chance, fx("0.05"));
    assert_eq!(cruiser.rounds_left, n(80));
    for (name, value) in [
        ("rate", n(3)),
        ("guns", n(8)),
        ("minutes", n(30)),
        ("main_battery_factor", n(1)),
        ("rounds_aboard", n(800)),
        ("shells", n(720)),
        ("gun_damage", n(20)),
        ("reference_gun_damage", n(20)),
        ("weighed_shots", n(720)),
    ] {
        assert_eq!(factor(&cruiser.event, name), value, "cruiser {name}");
    }
    // Night 1, destroyer: 6 x 4 x 30 = 720 shells but 600 aboard, so 600
    // shells, 600 x 5/20 = 150 weighed shots, 0 rounds left.
    let destroyer = on(1, 2).expect("the destroyer fires on night 1");
    assert_eq!(destroyer.shells, n(600));
    assert_eq!(destroyer.weighed, n(150));
    assert_eq!(destroyer.chance, fx("0.05"));
    assert_eq!(destroyer.rounds_left, n(0));
    for (name, value) in [
        ("rate", n(6)),
        ("guns", n(4)),
        ("minutes", n(30)),
        ("rounds_aboard", n(600)),
        ("shells", n(600)),
        ("gun_damage", n(5)),
        ("reference_gun_damage", n(20)),
        ("weighed_shots", n(150)),
    ] {
        assert_eq!(factor(&destroyer.event, name), value, "destroyer {name}");
    }
    // Night 2: the cruiser's last 80, all weighed 20/20; the destroyer has
    // nothing and does not fire.
    let cruiser = on(2, 1).expect("the cruiser fires on night 2");
    assert_eq!(cruiser.shells, n(80));
    assert_eq!(cruiser.weighed, n(80));
    assert_eq!(cruiser.rounds_left, n(0));
    assert!(on(2, 2).is_none(), "the destroyer fires nothing on night 2");
    // Nights 3 and 4: nothing is fired.
    assert!(!fires.iter().any(|f| f.day >= 3));
    assert!(!shores(&events).iter().any(|s| s.day >= 3));
    assert_eq!(fires.len(), 3);

    // The force: 1320 shells and 870 weighed shots on night 1 (at most 870
    // hits), 80 and 80 on night 2 (at most 80), at the base in its hex.
    let all = shores(&events);
    assert_eq!(all.len(), 2);
    assert_eq!(
        (all[0].day, all[0].shells, all[0].weighed),
        (1, n(1320), n(870))
    );
    assert_eq!(
        (all[1].day, all[1].shells, all[1].weighed),
        (2, n(80), n(80))
    );
    assert!(all[0].hits <= 870 && all[1].hits <= 80);
    for shore in &all {
        assert_eq!((shore.base, shore.hex), (1, X));
        assert_eq!(shore.pulse, Some(Pulse::Night));
    }
    // Every hit has a place or lands on nothing; the places add to at most
    // the hits.
    for shore in &all {
        assert!(shore.on_aircraft + shore.on_supply + shore.on_men <= shore.hits);
    }
}

#[test]
fn x_the_force_steams_to_x_burns_fuel_and_steams_back_on_day_3() {
    let run = Run::of(load());
    let events = run.events();
    // The men's supply is charged first: base 1 pays 1000 x 0.005 = 5 t.
    let (asked, paid, men) = men_charge(&events, 1, 1).expect("charged");
    assert_eq!((asked, paid, men), (n(5), n(5), 1000));
    // At the start, before any day: ships full of fuel, rounds as given.
    let force = |day: u32| &run.at(day).task_forces[&sim_core::ids::TaskForceId::new(1)];
    assert_eq!(force(0).position(), Hex::new(2, 0));
    assert_eq!(run.at(0).ships[&ShipId::new(1)].fuel_t, n(2000));
    assert_eq!(run.at(0).ships[&ShipId::new(2)].fuel_t, n(500));
    assert_eq!(run.ship_rounds(0, 1), n(800));
    assert_eq!(run.ship_rounds(0, 2), n(600));
    // 2 hexes, 80 nm, at 16 kn: the cruiser burns 20 t, the destroyer 10 t.
    // They stay at (0,0) on day 2 and burn nothing.
    for day in 1..=2 {
        assert_eq!(force(day).position(), X, "day {day}");
        assert_eq!(
            run.at(day).ships[&ShipId::new(1)].fuel_t,
            n(1980),
            "day {day}"
        );
        assert_eq!(
            run.at(day).ships[&ShipId::new(2)].fuel_t,
            n(490),
            "day {day}"
        );
    }
    // The hex it ends the night in holds base 1, an enemy base on its map.
    assert_eq!(
        run.at(0).bases[&BaseId::new(1)].hex,
        X,
        "base 1 is in the hex the force steams to"
    );
    // Day 3, night: 80 nm back to (2,0), 20 t and 10 t more.
    for day in 3..=4 {
        assert_eq!(force(day).position(), Hex::new(2, 0), "day {day}");
        assert_eq!(
            run.at(day).ships[&ShipId::new(1)].fuel_t,
            n(1960),
            "day {day}"
        );
        assert_eq!(
            run.at(day).ships[&ShipId::new(2)].fuel_t,
            n(480),
            "day {day}"
        );
    }
    // Rounds at the end of each day: 80 and 0 after day 1, then 0 and 0.
    assert_eq!(run.ship_rounds(1, 1), n(80));
    assert_eq!(run.ship_rounds(1, 2), n(0));
    for day in 2..=4 {
        assert_eq!(run.ship_rounds(day, 1), n(0), "day {day}");
        assert_eq!(run.ship_rounds(day, 2), n(0), "day {day}");
    }
}

#[test]
fn x_what_follows_from_the_hits_the_run_rolls() {
    let run = Run::of(load());
    let caps = check_nights(&run);
    assert_eq!(caps.fired_nights, 2);

    // Base 1's end supply: 2000 less the four nightly charges for unit 1's
    // men (1000, then what is left after each night) less the supply
    // destroyed on the two nights.
    let events = run.events();
    let all = shores(&events);
    let mut men = 1000u32;
    let mut supply = n(2000);
    for day in 1..=4u32 {
        let charge = fx("0.005") * n(i64::from(men));
        assert_eq!(men_charge(&events, day, 1).map(|c| c.1), Some(charge));
        supply -= charge;
        if let Some(shore) = all.iter().find(|s| s.day == day) {
            supply -= (n(50) * n(i64::from(shore.on_supply))).min(supply);
            men -= (10 * shore.on_men).min(men);
        }
        assert_eq!(run.base_supply(day, 1), supply, "day {day}");
        assert_eq!(run.men(day, 1), men, "day {day}");
    }
    // The aircraft are 24 before the first night, and fewer after.
    assert_eq!(run.aircraft(0), (20, 4));
    // The hits are at most 870 and 80, and the aircraft destroyed are the
    // hits on aircraft or all.
    let night_1 = &all[0];
    let (ready, damaged) = run.aircraft(1);
    assert_eq!(
        24 - (ready + damaged),
        night_1.on_aircraft.min(24),
        "aircraft destroyed on night 1"
    );
    assert_eq!(damaged, 4 - night_1.on_aircraft.min(4), "damaged go first");
}

#[test]
fn x_the_japanese_side_claims_aircraft_it_may_not_have_destroyed() {
    let run = Run::of(load());
    let events = run.events();
    let claims = bombardment_claims(&events, Side::Japanese);
    // One claim for each night the force fired, none for the other two.
    assert_eq!(
        claims.iter().map(|(e, _)| e.day).collect::<Vec<_>>(),
        [1, 2]
    );
    let all = shores(&events);
    for ((claim, claimed), shore) in claims.iter().zip(&all) {
        let real = factor(claim, "real_on_ground");
        let weighed = factor(claim, "weighed_shots");
        assert_eq!(weighed, if shore.day == 1 { n(870) } else { n(80) });
        let false_claims = factor(claim, "false_on_ground");
        assert_eq!(n(i64::from(*claimed)), real + false_claims);
        assert!(n(i64::from(*claimed)) >= real);
        assert_eq!(
            claim.breakdown.as_ref().unwrap().result,
            n(i64::from(*claimed))
        );
        // The real aircraft destroyed that night.
        let (ready, damaged) = (run.aircraft(shore.day - 1), run.aircraft(shore.day));
        let destroyed = (ready.0 + ready.1) - (damaged.0 + damaged.1);
        assert_eq!(real, n(i64::from(destroyed)));
    }
    // The side's view holds the two claims, as claims of a bombardment.
    let view = run.scenario.world.side_view(Side::Japanese);
    let held = view.claims();
    assert_eq!(held.len(), 2);
    for ((claim, claimed), held) in claims.iter().zip(held) {
        assert!(held.from_bombardment && !held.from_air);
        assert_eq!(held.claimed_on_ground, *claimed);
        assert_eq!(
            (held.claimed_sunk, held.claimed_hits, held.claimed_shot_down),
            (0, 0, 0)
        );
        assert_eq!(held.claimed_kind, None);
        assert_eq!(held.hex, X);
        assert_eq!((held.day, held.pulse), (claim.day, Pulse::Night));
    }
}

fn is_whole(kind: &SideEventKind, pick: fn(&EventKind) -> bool) -> bool {
    matches!(kind, SideEventKind::Whole(inner) if pick(inner))
}

#[test]
fn x_the_japanese_side_is_told_its_shells_and_no_hit() {
    let run = Run::of(load());
    let view = run.scenario.world.side_view(Side::Japanese);
    let events = view.events();
    let events_of =
        |day: u32| -> Vec<&SideEvent> { events.iter().filter(|e| e.day == day).collect() };

    let mut ship_shells = Vec::new();
    let mut force_shells = Vec::new();
    for event in &events {
        match &event.kind {
            SideEventKind::BombardmentFire {
                ship,
                base,
                shells,
                weighed_shots,
                rounds_left,
            } => {
                assert_eq!(base.raw(), 1);
                ship_shells.push((event.day, ship.raw(), *shells, *weighed_shots, *rounds_left));
                // The chance and the hits are not given, nor a result.
                let breakdown = event.breakdown.as_ref().expect("its own factors");
                assert!(breakdown.factors.iter().all(|f| f.name != "chance"));
                assert_eq!(breakdown.result, None);
            }
            SideEventKind::ShoreBombardment {
                base,
                hex,
                shells,
                weighed_shots,
                ..
            } => {
                assert_eq!((base.raw(), *hex), (1, X));
                force_shells.push((event.day, *shells, *weighed_shots));
                assert!(event.breakdown.is_none());
            }
            other => {
                // Nothing the Japanese side sees carries a hit or a place a
                // hit landed, or a thing of the Allied side's destroyed.
                assert!(
                    !matches!(
                        other,
                        SideEventKind::ShoreBombardmentAgainst { .. }
                            | SideEventKind::Whole(EventKind::BombardmentFire { .. })
                            | SideEventKind::Whole(EventKind::ShoreBombardment { .. })
                            | SideEventKind::Whole(EventKind::AircraftDestroyedOnGround { .. })
                            | SideEventKind::Whole(
                                EventKind::DamagedAircraftDestroyedOnGround { .. }
                            )
                            | SideEventKind::Whole(EventKind::BaseSupplyDestroyed { .. })
                            | SideEventKind::Whole(EventKind::MenLostToFire { .. })
                    ),
                    "the Japanese side is not told {other:?}"
                );
            }
        }
    }
    assert_eq!(
        ship_shells,
        vec![
            (1, 1, n(720), n(720), n(80)),
            (1, 2, n(600), n(150), n(0)),
            (2, 1, n(80), n(80), n(0)),
        ]
    );
    assert_eq!(force_shells, vec![(1, n(1320), n(870)), (2, n(80), n(80))]);
    // Told on the night, and not on days 3 and 4.
    assert!(events_of(3)
        .iter()
        .chain(events_of(4).iter())
        .all(|e| !matches!(
            e.kind,
            SideEventKind::BombardmentFire { .. } | SideEventKind::ShoreBombardment { .. }
        )));
    for event in events.iter().filter(|e| {
        matches!(
            e.kind,
            SideEventKind::BombardmentFire { .. } | SideEventKind::ShoreBombardment { .. }
        )
    }) {
        assert_eq!(event.pulse, Some(Pulse::Night));
    }
    // The claim is seen whole, with no breakdown: the real and false parts
    // are the umpire's.
    let claims: Vec<&SideEvent> = events
        .iter()
        .filter(|e| is_whole(&e.kind, |k| matches!(k, EventKind::ClaimReceived { .. })))
        .collect();
    assert_eq!(claims.len(), 2);
    assert!(claims.iter().all(|c| c.breakdown.is_none()));
}

#[test]
fn x_the_allied_side_sees_the_shots_and_where_they_landed_and_not_who_fired() {
    let run = Run::of(load());
    let umpire = run.events();
    let all = shores(&umpire);
    let view = run.scenario.world.side_view(Side::Allied);
    let events = view.events();
    let against: Vec<&SideEvent> = events
        .iter()
        .filter(|e| matches!(e.kind, SideEventKind::ShoreBombardmentAgainst { .. }))
        .collect();
    assert_eq!(against.len(), 2);
    for (event, shore) in against.iter().zip(&all) {
        let SideEventKind::ShoreBombardmentAgainst {
            base,
            hex,
            shells,
            weighed_shots,
            hits,
            hits_on_aircraft,
            hits_on_supply,
            hits_on_men,
        } = &event.kind
        else {
            unreachable!()
        };
        assert_eq!((base.raw(), *hex, event.day), (1, X, shore.day));
        assert_eq!(event.pulse, Some(Pulse::Night));
        assert_eq!((*shells, *weighed_shots), (shore.shells, shore.weighed));
        assert_eq!(
            (*hits, *hits_on_aircraft, *hits_on_supply, *hits_on_men),
            (shore.hits, shore.on_aircraft, shore.on_supply, shore.on_men)
        );
        assert!(event.breakdown.is_none());
    }
    assert_eq!(
        against.iter().map(|e| e.day).collect::<Vec<_>>(),
        [1, 2],
        "none on days 3 and 4"
    );
    // The Allied side is told neither the force nor a ship, nor the ships'
    // fire.
    assert!(!events.iter().any(|e| matches!(
        e.kind,
        SideEventKind::ShoreBombardment { .. } | SideEventKind::BombardmentFire { .. }
    )));
    // It sees its own losses whole: the aircraft, the supply and the men.
    let whole: Vec<&EventKind> = events
        .iter()
        .filter_map(|e| match &e.kind {
            SideEventKind::Whole(kind) => Some(kind),
            _ => None,
        })
        .collect();
    let seen = |pick: fn(&EventKind) -> bool| whole.iter().filter(|k| pick(k)).count();
    let truth_seen = |pick: fn(&Event) -> bool| umpire.iter().filter(|e| pick(e)).count();
    assert_eq!(
        seen(|k| matches!(k, EventKind::BaseSupplyDestroyed { .. })),
        truth_seen(|e| matches!(e.kind, EventKind::BaseSupplyDestroyed { .. }))
    );
    assert_eq!(
        seen(|k| matches!(k, EventKind::MenLostToFire { .. })),
        truth_seen(|e| matches!(e.kind, EventKind::MenLostToFire { .. }))
    );
    assert_eq!(
        seen(|k| matches!(
            k,
            EventKind::AircraftDestroyedOnGround { .. }
                | EventKind::DamagedAircraftDestroyedOnGround { .. }
        )),
        truth_seen(|e| matches!(
            e.kind,
            EventKind::AircraftDestroyedOnGround { .. }
                | EventKind::DamagedAircraftDestroyedOnGround { .. }
        ))
    );
    // It is told of no claim of the Japanese side's.
    assert!(!whole
        .iter()
        .any(|k| matches!(k, EventKind::ClaimReceived { .. })));
}

#[test]
fn x_the_allied_side_holds_a_contact_at_x_or_beside_it_after_each_night_it_was_shelled() {
    let run = Run::of(load());
    let view = run.scenario.world.side_view(Side::Allied);
    let events = view.events();
    for day in [1u32, 2] {
        let reports: Vec<&SideEvent> = events
            .iter()
            .filter(|e| {
                e.day == day
                    && e.pulse == Some(Pulse::Night)
                    && matches!(
                        &e.kind,
                        SideEventKind::Whole(EventKind::ReportReceived { .. })
                    )
            })
            .collect();
        assert_eq!(
            reports.len(),
            1,
            "day {day}: a sighting report on the night"
        );
        let SideEventKind::Whole(EventKind::ReportReceived { side, hex, .. }) = &reports[0].kind
        else {
            unreachable!()
        };
        assert_eq!(*side, Side::Allied);
        assert!(
            hex.distance(X) <= 1,
            "day {day}: the report names {hex} for a force at {X}"
        );
        // The report follows the bombardment it reports.
        let bombarded = events
            .iter()
            .find(|e| {
                e.day == day && matches!(e.kind, SideEventKind::ShoreBombardmentAgainst { .. })
            })
            .expect("bombarded");
        assert!(reports[0].number > bombarded.number);
    }
    // After each night it holds a contact at X or beside it.
    for day in [1u32, 2] {
        assert!(
            run.belief(day, Side::Allied)
                .contacts
                .iter()
                .any(|hex| hex.distance(X) <= 1),
            "day {day}: a contact at X or beside it"
        );
    }
    // No report of the force on any other night: the Allied side's only
    // sightings of it are the nights it fired. On day 1 the report is of the
    // contact the side now holds.
    let sightings = events
        .iter()
        .filter(|e| {
            matches!(
                &e.kind,
                SideEventKind::Whole(EventKind::ReportReceived { .. })
            )
        })
        .count();
    assert_eq!(sightings, 2);
}

// ------------------------------------------------- The hand calculation: Y

#[test]
fn y_the_opening_estimates_and_the_map_before_day_1() {
    let scenario = load();
    let japanese = scenario.world.side_view(Side::Japanese);
    let estimate = japanese.ground_estimate(Y).expect("an opening estimate");
    assert_eq!((estimate.men, estimate.day, estimate.hex), (300, 0, Y));
    assert_eq!(japanese.ground_estimates().count(), 1);
    let allied = scenario.world.side_view(Side::Allied);
    assert!(allied.ground_estimate(Y).is_none());
    assert_eq!(allied.ground_estimates().count(), 0);
    // Base 3 is not on the Allied map; bases 1 and 2 are on the Japanese.
    assert!(!allied.enemy_bases().any(|b| b.id.raw() == 3));
    assert_eq!(allied.enemy_bases().count(), 0);
    let seen: Vec<u32> = japanese.enemy_bases().map(|b| b.id.raw()).collect();
    assert_eq!(seen, [1, 2]);
    // The opening estimate is an event of day 0 the Japanese side sees; the
    // Allied side does not.
    let events = scenario.world.events();
    let opening: Vec<&Event> = events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::GroundReportReceived { .. }))
        .collect();
    assert_eq!(opening.len(), 1);
    assert_eq!(opening[0].day, 0);
    assert!(opening[0].causes.is_empty());
    assert!(matches!(
        opening[0].kind,
        EventKind::GroundReportReceived {
            side: Side::Japanese,
            men: 300,
            replaced: false,
            ..
        }
    ));
    assert!(!allied.events().iter().any(|e| matches!(
        &e.kind,
        SideEventKind::Whole(EventKind::GroundReportReceived { .. })
    )));
    assert!(japanese.events().iter().any(|e| matches!(
        &e.kind,
        SideEventKind::Whole(EventKind::GroundReportReceived { men: 300, .. })
    )));
    // Base 3 is off the Allied map in the truth.
    assert!(!scenario.world.umpire().bases[&BaseId::new(3)].on_map);
}

/// What the hand calculation says of Y, for every day and every seed:
/// the fights, the losses, the reports and what each side holds after them.
/// Units 2 (Allied) and 3 (Japanese) are the ones at Y.
fn check_y(run: &Run) {
    let events = run.events();
    let unit = |day: u32, id: u32| &run.at(day).land_units[&sim_core::ids::LandUnitId::new(id)];
    let mut last: [Option<GroundEstimate>; 2] = [
        run.belief(0, Side::Allied).estimates.first().copied(),
        run.belief(0, Side::Japanese).estimates.first().copied(),
    ];
    let mut base_found = 0;
    for day in 1..=run.len() {
        let (a0, j0) = (
            men_at(run.at(day - 1), Side::Allied, Y),
            men_at(run.at(day - 1), Side::Japanese, Y),
        );
        let day_events: Vec<&Event> = events.iter().filter(|e| e.day == day).collect();

        // Each night both sides at Y are fed in full.
        for (side, base, men) in [(Side::Allied, 2u32, a0), (Side::Japanese, 3, j0)] {
            let asked = men_charge(&events, day, base).map_or(Fixed::ZERO, |c| c.0);
            assert_eq!(
                asked,
                fx("0.005") * n(i64::from(men)),
                "day {day} base {base}"
            );
            let fed = day_events.iter().find_map(|e| match &e.kind {
                EventKind::MenFed {
                    side: s,
                    hex,
                    men,
                    fed,
                    unfed,
                    ..
                } if *s == side && *hex == Y => Some((*men, *fed, *unfed)),
                _ => None,
            });
            if men > 0 {
                assert_eq!(fed, Some((men, men, 0)), "day {day} {side:?} fed in full");
            }
        }

        let fights: Vec<&&Event> = day_events
            .iter()
            .filter(|e| matches!(&e.kind, EventKind::GroundFight { hex, .. } if *hex == Y))
            .collect();
        let fires: Vec<(u32, u32, Fixed, Fixed, u32)> = day_events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::GroundFire {
                    unit,
                    firing_men,
                    shots,
                    chance,
                    hits,
                    ..
                } => Some((unit.raw(), *firing_men, *shots, *chance, *hits)),
                _ => None,
            })
            .collect();
        let (a1, j1);
        let fight_id;
        if a0 > 0 && j0 > 0 {
            // The Japanese side attacks and the Allied side defends.
            assert_eq!(fights.len(), 1, "day {day}: a fight");
            assert!(matches!(
                fights[0].kind,
                EventKind::GroundFight {
                    allied_attacks: false,
                    japanese_attacks: true,
                    ..
                }
            ));
            fight_id = Some(fights[0].id);
            // Unit 3: its men less its fatigue's share, cut: 8 shots at 0.3
            // on day 1. Unit 2: all its men, at 0.6.
            let fatigue = unit(day - 1, 3).fatigue;
            assert_eq!(
                fatigue,
                n(20 * i64::from(day - 1)),
                "day {day} unit 3's fatigue"
            );
            let firing_3 = u32::try_from(i64::from(j0) * (100 - fatigue.trunc()) / 100).unwrap();
            let shots = |men: u32| Fixed::from_raw(i64::from(men) * 10_000);
            let f2 = fires.iter().find(|f| f.0 == 2).expect("unit 2 fires");
            let f3 = fires.iter().find(|f| f.0 == 3).expect("unit 3 fires");
            assert_eq!((f2.1, f2.2, f2.3), (a0, shots(a0), fx("0.6")), "day {day}");
            assert_eq!(
                (f3.1, f3.2, f3.3),
                (firing_3, shots(firing_3), fx("0.3")),
                "day {day}"
            );
            assert!(f2.4 <= at_most(f2.2) && f3.4 <= at_most(f3.2));
            // Losses: 20 a hit, from the other side, no more than it has.
            a1 = a0 - (20 * f3.4).min(a0);
            j1 = j0 - (20 * f2.4).min(j0);
            assert_eq!(unit(day, 2).men_ashore, a1, "day {day}");
            assert_eq!(unit(day, 3).men_ashore, j1, "day {day}");
            if day == 1 {
                // j is at most 8 hits, a at most 30.
                assert!(f3.4 <= 8 && f2.4 <= 30);
                assert!(a1 >= 2840, "unit 2 has at least 2840");
                assert!(
                    j1 > 0,
                    "unit 3 is not destroyed on day 1: pick another seed"
                );
            }
            if j1 > 0 {
                assert_eq!(unit(day, 3).fatigue, fatigue + n(20), "day {day}");
            }
            assert_eq!(unit(day, 2).fatigue, n(0));
            // Not beaten: the Allied side has far more than 0.2 x 800.
            assert!(a1 > j0 / 5);
        } else {
            // No fight, no fire, no report; the men are as they were.
            assert!(fights.is_empty() && fires.is_empty(), "day {day}: no fight");
            fight_id = None;
            (a1, j1) = (
                men_at(run.at(day), Side::Allied, Y),
                men_at(run.at(day), Side::Japanese, Y),
            );
            assert_eq!((a1, j1), (a0, j0));
        }

        // The reports, after the fight, if both sides still have men.
        let reports: Vec<&&Event> = day_events
            .iter()
            .filter(|e| matches!(&e.kind, EventKind::GroundReportReceived { .. }))
            .collect();
        if a1 > 0 && j1 > 0 {
            assert_eq!(reports.len(), 2, "day {day}: both sides report");
            for (index, side) in [Side::Allied, Side::Japanese].into_iter().enumerate() {
                let (report, truth) = match side {
                    Side::Allied => (reports[0], j1),
                    Side::Japanese => (reports[1], a1),
                };
                let EventKind::GroundReportReceived {
                    side: s,
                    hex,
                    men,
                    replaced,
                } = &report.kind
                else {
                    unreachable!()
                };
                assert_eq!((*s, *hex), (side, Y));
                assert_eq!(*replaced, last[index].is_some(), "day {day} {side:?}");
                // The report is the truth after the fight, times an error
                // between 0.5 and 1.5, cut to a whole man.
                assert_eq!(factor(report, "true_men"), n(i64::from(truth)));
                assert_eq!(factor(report, "report_error_low"), fx("0.5"));
                assert_eq!(factor(report, "report_error_high"), fx("1.5"));
                let error = factor(report, "error");
                assert!(error >= fx("0.5") && error <= fx("1.5"));
                assert_eq!(i64::from(*men), (n(i64::from(truth)) * error).trunc());
                assert!(*men >= truth / 2, "day {day}: {men} against truth {truth}");
                assert!(i64::from(*men) <= (n(i64::from(truth)) * fx("1.5")).trunc());
                assert_eq!(report.pulse, None);
                assert_eq!(
                    report.causes,
                    fight_id.map(Cause::Event).into_iter().collect::<Vec<_>>()
                );
                // What the side holds is that report, dated the day.
                let held = run
                    .belief(day, side)
                    .estimates
                    .iter()
                    .find(|e| e.hex == Y)
                    .copied()
                    .expect("an estimate");
                assert_eq!(
                    (held.men, held.day, held.report_event),
                    (*men, day, report.id)
                );
                last[index] = Some(held);
            }
            // Base 3 is put on the Allied map by the Allied report, once.
            let found: Vec<&&Event> = day_events
                .iter()
                .filter(|e| matches!(e.kind, EventKind::BaseFound { .. }))
                .collect();
            if day == 1 {
                assert_eq!(found.len(), 1);
                assert!(matches!(
                    found[0].kind,
                    EventKind::BaseFound { side: Side::Allied, hex, .. } if hex == Y
                ));
                assert_eq!(found[0].causes, vec![Cause::Event(reports[0].id)]);
                base_found += 1;
            } else {
                assert!(found.is_empty(), "day {day}: base 3 is found once");
            }
        } else {
            // No fight is needed and no report is made: each side keeps the
            // estimate it had, with its day.
            assert!(reports.is_empty(), "day {day}: no report");
            for (index, side) in [Side::Allied, Side::Japanese].into_iter().enumerate() {
                let held = run
                    .belief(day, side)
                    .estimates
                    .iter()
                    .find(|e| e.hex == Y)
                    .copied();
                assert_eq!(held, last[index], "day {day} {side:?} keeps its estimate");
            }
        }

        // The map: base 3 is on the Allied map from day 1; base 2 is Allied
        // always, and base 3 Japanese while unit 3 has a man ashore.
        let found_by_now = run
            .belief(day, Side::Allied)
            .enemy_bases
            .iter()
            .any(|b| b.raw() == 3);
        assert_eq!(found_by_now, base_found > 0, "day {day}");
        assert!(run.at(day).bases[&BaseId::new(3)].on_map);
        assert_eq!(run.at(day).bases[&BaseId::new(2)].side, Side::Allied);
        if j1 > 0 {
            assert_eq!(run.at(day).bases[&BaseId::new(3)].side, Side::Japanese);
        }
    }
    assert_eq!(run.at(run.len()).bases[&BaseId::new(2)].side, Side::Allied);
    // Y never changes a base's side in these figures.
    assert!(!events.iter().any(|e| matches!(
        e.kind,
        EventKind::BaseChangedSide { .. } | EventKind::SideBeatenAshore { .. }
    )));
}

#[test]
fn y_an_attack_on_an_estimate_of_300_finds_3000_and_fails() {
    let run = Run::of(load());
    check_y(&run);
    let events = run.events();
    // Day 1's reports, which the hand calculation bounds.
    let reports: Vec<(u32, Side, u32)> = events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::GroundReportReceived { side, men, .. } if e.day >= 1 => {
                Some((e.day, *side, *men))
            }
            _ => None,
        })
        .collect();
    let japanese_day_1 = reports
        .iter()
        .find(|(d, s, _)| *d == 1 && *s == Side::Japanese)
        .expect("the Japanese side's report of day 1")
        .2;
    assert!(
        japanese_day_1 >= 1420,
        "nearly five times the 300 it attacked on"
    );
    assert!(run.belief(0, Side::Japanese).estimates[0].men == 300);
    // The Japanese estimate was replaced on day 1 and carries the day.
    let held = run.belief(1, Side::Japanese).estimates[0];
    assert_eq!((held.men, held.day, held.hex), (japanese_day_1, 1, Y));
    // Allied: its first estimate on day 1, base 3 on its map, with an event
    // it sees and the Japanese side does not.
    let held = run.belief(1, Side::Allied).estimates[0];
    assert_eq!((held.day, held.hex), (1, Y));
    assert!(run
        .belief(1, Side::Allied)
        .enemy_bases
        .iter()
        .any(|b| b.raw() == 3));
    assert!(!run
        .belief(0, Side::Allied)
        .enemy_bases
        .iter()
        .any(|b| b.raw() == 3));
    let allied = run.scenario.world.side_view(Side::Allied).events();
    let japanese = run.scenario.world.side_view(Side::Japanese).events();
    let found = |events: &[SideEvent]| {
        events
            .iter()
            .filter(|e| matches!(&e.kind, SideEventKind::Whole(EventKind::BaseFound { .. })))
            .count()
    };
    assert_eq!(found(&allied), 1);
    assert_eq!(found(&japanese), 0);
    // Each side sees its own reports and never the other's; and no
    // breakdown of one (it holds the truth and the error drawn).
    for (events, side) in [(&allied, Side::Allied), (&japanese, Side::Japanese)] {
        for event in events.iter() {
            if let SideEventKind::Whole(EventKind::GroundReportReceived { side: s, .. }) =
                &event.kind
            {
                assert_eq!(*s, side);
                assert!(event.breakdown.is_none());
            }
        }
    }
    let count = |events: &[SideEvent]| {
        events
            .iter()
            .filter(|e| {
                matches!(
                    &e.kind,
                    SideEventKind::Whole(EventKind::GroundReportReceived { .. })
                )
            })
            .count()
    };
    let truth_reports = |side: Side| {
        events
            .iter()
            .filter(|e| matches!(&e.kind, EventKind::GroundReportReceived { side: s, .. } if *s == side))
            .count()
    };
    assert_eq!(count(&allied), truth_reports(Side::Allied));
    assert_eq!(count(&japanese), truth_reports(Side::Japanese));
    // Both ground units at Y have the aircraft-free air group of X
    // untouched by anything at Y.
    assert_eq!(
        run.at(0).air_groups[&sim_core::ids::AirGroupId::new(1)].location,
        AirGroupLocation::Base(BaseId::new(1))
    );
}

#[test]
fn y_on_a_day_unit_3_is_gone_there_is_no_fight_and_no_report() {
    // Seed 1942 destroys unit 3 on day 3 (the run in the architect's
    // reference); the test reads it from the run and does not require it.
    let run = Run::of(load());
    let events = run.events();
    let gone = (1..=run.len()).find(|&day| run.men(day, 3) == 0);
    let Some(gone) = gone else {
        panic!("unit 3 is not destroyed by day 4 on seed 1942: the hand calculation's later rows need a seed where it is");
    };
    for day in gone + 1..=run.len() {
        assert!(!events.iter().any(|e| e.day == day
            && matches!(
                e.kind,
                EventKind::GroundFight { .. }
                    | EventKind::GroundFire { .. }
                    | EventKind::GroundReportReceived { .. }
            )));
    }
    // On the day it is destroyed in the fight there is no report either, and
    // the estimates are the last ones, with their days.
    assert!(!events
        .iter()
        .any(|e| e.day == gone && matches!(e.kind, EventKind::GroundReportReceived { .. })));
    for side in [Side::Allied, Side::Japanese] {
        let held = run.belief(run.len(), side).estimates[0];
        assert_eq!(
            held.day,
            gone - 1,
            "{side:?} keeps the estimate of day {}",
            gone - 1
        );
        assert_eq!(held, run.belief(gone - 1, side).estimates[0]);
    }
    // Base 2 is Allied at the end.
    assert_eq!(run.at(run.len()).bases[&BaseId::new(2)].side, Side::Allied);
}

// ------------------------------------------------------------- Properties

/// The same scenario on many seeds, and with other men at Y and X: the
/// hand calculation's formulas hold whatever the dice.
#[test]
fn p_the_formulas_hold_on_many_seeds() {
    let source = text();
    for seed in 1..=40u32 {
        let run = Run::seed(&source, seed);
        check_nights(&run);
        check_y(&run);
    }
}

#[test]
fn p_a_ground_report_always_lies_between_the_low_and_high_share_of_the_truth() {
    let mut reports = 0;
    let mut variants: Vec<String> = vec![text()];
    // Other men at Y: the cut to a whole man is then of other counts.
    for (men_3, men_2) in [(777, 3001), (123, 999), (1, 50), (5000, 4999)] {
        let source = common::replace_once(
            &common::replace_once(&text(), "men = 800\n", &format!("men = {men_3}\n")),
            "men = 3000\n",
            &format!("men = {men_2}\n"),
        );
        variants.push(source);
    }
    for source in &variants {
        for seed in 1..=25u32 {
            let run = Run::seed(source, seed);
            for event in run.events().iter() {
                let EventKind::GroundReportReceived { side, men, .. } = &event.kind else {
                    continue;
                };
                if event.day == 0 {
                    continue;
                }
                // The truth, recomputed from the state after the fight.
                let other = match side {
                    Side::Allied => Side::Japanese,
                    Side::Japanese => Side::Allied,
                };
                let truth = men_at(run.at(event.day), other, Y);
                assert_eq!(factor(event, "true_men"), n(i64::from(truth)));
                let low = (n(i64::from(truth)) * fx("0.5")).trunc();
                let high = (n(i64::from(truth)) * fx("1.5")).trunc();
                assert!(
                    i64::from(*men) >= low && i64::from(*men) <= high,
                    "seed {seed} day {}: {men} is not between {low} and {high} of {truth}",
                    event.day
                );
                reports += 1;
            }
        }
    }
    assert!(reports > 400, "{reports} reports were checked");
}

#[test]
fn p_a_bombardment_never_destroys_more_than_is_there() {
    // Few aircraft, little supply and few men, so that the caps bite.
    let source = common::replace_once(
        &text(),
        "ready = 20\ndamaged = 4\n",
        "ready = 2\ndamaged = 1\n",
    );
    let source = common::replace_once(&source, "supply_t = 2000\n", "supply_t = 120\n");
    let source = common::replace_once(&source, "men = 1000\n", "men = 25\n");
    let mut caps = Caps::default();
    for seed in 1..=30u32 {
        let run = Run::seed(&source, seed);
        let c = check_nights(&run);
        caps.aircraft |= c.aircraft;
        caps.supply |= c.supply;
        caps.men |= c.men;
        // Never more than there is, at the end of any day.
        for day in 0..=run.len() {
            let (ready, damaged) = run.aircraft(day);
            assert!(ready <= 2 && damaged <= 1);
            assert!(run.base_supply(day, 1) >= Fixed::ZERO);
            assert!(run.men(day, 1) <= 25);
        }
        // Unit 1 is gone from the hex when no man is left.
        let unit = &run.at(run.len()).land_units[&sim_core::ids::LandUnitId::new(1)];
        assert_eq!(unit.hex.is_none(), unit.men_ashore == 0);
    }
    assert!(
        caps.aircraft && caps.supply && caps.men,
        "every cap bit on some seed: {caps:?}"
    );
}

#[test]
fn p_rounds_are_conserved_with_other_magazines() {
    // The cruiser has fewer rounds than a night's fire, and the destroyer
    // more: shells are the rounds that are there, and never more.
    let source = common::replace_once(&text(), "gun_rounds = 800\n", "gun_rounds = 500\n");
    let source = common::replace_once(&source, "gun_rounds = 600\n", "gun_rounds = 719\n");
    for seed in 1..=10u32 {
        let run = Run::seed(&source, seed);
        check_nights(&run);
        let fires = ship_fires(&run.events());
        let cruiser = fires.iter().find(|f| f.day == 1 && f.ship == 1).unwrap();
        assert_eq!(
            (cruiser.shells, cruiser.weighed, cruiser.rounds_left),
            (n(500), n(500), n(0))
        );
        let destroyer = fires.iter().find(|f| f.day == 1 && f.ship == 2).unwrap();
        assert_eq!(
            (destroyer.shells, destroyer.weighed, destroyer.rounds_left),
            (n(719), fx("179.75"), n(0))
        );
        // Night 2: nothing left to fire.
        assert!(!fires.iter().any(|f| f.day == 2));
    }
}

/// The map of scenario 24 with a third coast hex Z at (4,0), written to a
/// folder of its own, for a variant whose extra men stand where nothing else
/// is.
fn map_with_z() -> PathBuf {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("scenario_24_map_z");
    std::fs::create_dir_all(&folder).expect("a folder");
    let original = common::data_dir().join("map/placeholder_shells_and_estimates.toml");
    let mut map = std::fs::read_to_string(original).expect("the map reads");
    map.push_str("\n[[hex]]\nq = 4\nr = 0\nkind = \"coast\"\nport_size = 3\nanchorage = true\n");
    let target = folder.join(format!("map_z_{}.toml", std::process::id()));
    std::fs::write(&target, map).expect("the map writes");
    target
}

/// Everything a side's view holds that could say whether an enemy unit is at
/// a hex it has no report of, taken as text with the opaque event ids left
/// out. The side's own numbers of events stay.
fn digest(world: &World, side: Side) -> String {
    let view = world.side_view(side);
    let mut out = String::new();
    for event in view.events() {
        out.push_str(&format!(
            "{} {} {:?} {:?} {:?} {:?} {} {:?}\n",
            event.number,
            event.day,
            event.pulse,
            event.kind,
            event.order_number,
            event.causes,
            event.hidden_causes,
            event.breakdown
        ));
    }
    for contact in view.contacts() {
        out.push_str(&format!(
            "contact {:?} {} {} {:?} {}\n",
            contact.id,
            contact.estimated_hex,
            contact.estimated_ship_count,
            contact.estimated_kind,
            contact.age_pulses
        ));
    }
    for claim in view.claims() {
        out.push_str(&format!(
            "claim {} {} {:?} {} {} {} {:?} {} {} {} {}\n",
            claim.hex,
            claim.day,
            claim.pulse,
            claim.claimed_sunk,
            claim.claimed_hits,
            claim.claimed_shot_down,
            claim.claimed_kind,
            claim.claimed_on_ground,
            claim.from_air,
            claim.from_bombardment,
            u32::from(claim.claimed_on_ground > 0)
        ));
    }
    for estimate in view.ground_estimates() {
        out.push_str(&format!(
            "estimate {} {} {}\n",
            estimate.hex, estimate.men, estimate.day
        ));
    }
    for base in view.enemy_bases() {
        out.push_str(&format!("enemy base {:?}\n", base));
    }
    for base in view.bases() {
        out.push_str(&format!("base {:?}\n", base));
    }
    for unit in view.land_units() {
        out.push_str(&format!("unit {:?}\n", unit));
    }
    out
}

/// The digest of a side after each day of a run of `source`.
fn digests(source: &str, side: Side) -> Vec<String> {
    let mut scenario = from(source);
    let mut out = vec![digest(&scenario.world, side)];
    while scenario.next_day().expect("a day resolves").is_some() {
        out.push(digest(&scenario.world, side));
    }
    out
}

#[test]
fn p_a_sides_view_is_the_same_in_two_worlds_that_differ_only_in_enemy_men_it_has_no_report_of() {
    let map = map_with_z();
    let with_map = common::replace_once(
        &text(),
        "map = \"../map/placeholder_shells_and_estimates.toml\"",
        &format!("map = \"{}\"", map.display()),
    );
    let unit = |id: u32, side: &str, men: u32, posture: &str| {
        format!(
            "\n[[land_unit]]\nid = {id}\nname = \"Placeholder extra unit\"\nside = \"{side}\"\nmen = {men}\nq = 4\nr = 0\nposture = \"{posture}\"\n"
        )
    };
    for (viewer, enemy) in [("allied", "japanese"), ("japanese", "allied")] {
        let side = if viewer == "allied" {
            Side::Allied
        } else {
            Side::Japanese
        };
        for seed in [1942u32, 7, 99] {
            let base = with_seed(&with_map, seed);
            let without = digests(&base, side);
            for (men, posture) in [(100, "defend"), (700, "attack"), (2000, "defend")] {
                let other = format!("{base}{}", unit(4, enemy, men, posture));
                let with = digests(&other, side);
                assert_eq!(
                    without, with,
                    "{viewer}'s view with {men} {enemy} men at a hex it has no report of, seed {seed}"
                );
            }
        }
    }
    // Control: the view does differ when the men are in a hex with a report
    // (unit 3's men at Y change what the Allied side is told of them).
    let few = common::replace_once(&text(), "men = 800\n", "men = 200\n");
    assert_ne!(
        digests(&text(), Side::Allied),
        digests(&few, Side::Allied),
        "the control: men it has a report of change its view"
    );
}

#[test]
fn p_a_raid_on_a_base_with_no_men_ashore_gives_the_hits_it_gave_before() {
    let source = std::fs::read_to_string(common::data_dir().join("scenarios/06a_base_raid.toml"))
        .expect("scenario 6a reads");
    let path = common::data_dir().join("scenarios/06a_base_raid.toml");
    let men_0 = common::altered_reference("men0", |dir| {
        common::edit_file_by(dir, "air_combat.toml", &[("men = \"0.2\"", "men = \"0\"")]);
    });
    // With and without the ground figures loaded: with them, a base has men
    // to hit if its side has any ashore, and here nobody has.
    let with_ground = common::replace_once(
        &source,
        "air_combat = \"../reference/air_combat.toml\"\n",
        "air_combat = \"../reference/air_combat.toml\"\nground = \"../reference/ground.toml\"\n",
    );
    let mut raids = 0;
    let mut hits = 0;
    for text in [&source, &with_ground] {
        for seed in 1..=20u32 {
            let seeded = common::replace_once(text, "seed = 1942\n", &format!("seed = {seed}\n"));
            let run = |options: &sim_data::LoadOptions| {
                let mut scenario =
                    common::from_text_with(&seeded, &path, options).expect("6a loads");
                while scenario.next_day().expect("a day resolves").is_some() {}
                scenario
            };
            let with_men_share = run(&common::frozen());
            let without = run(&men_0);
            let a: Vec<Event> = with_men_share.world.events().iter().cloned().collect();
            let b: Vec<Event> = without.world.events().iter().cloned().collect();
            assert_eq!(a, b, "seed {seed}: identical logs");
            assert_eq!(with_men_share.world.umpire(), without.world.umpire());
            for event in &a {
                if let EventKind::BaseAttacked {
                    hits: h,
                    hits_on_men,
                    ..
                } = &event.kind
                {
                    assert_eq!(*hits_on_men, 0);
                    raids += 1;
                    hits += h;
                }
            }
        }
    }
    assert!(raids > 0 && hits > 0, "{raids} raids, {hits} hits");
}

#[test]
fn p_a_scenario_with_no_bombardment_no_men_and_no_estimate_is_unchanged() {
    // The figures of decision 0092 are taken out of the frozen files (the
    // file as it stood before the decision) and put back: the logs, the
    // truth and the hash are the same.
    fn drop_keys(text: String, keys: &[&str]) -> String {
        text.lines()
            .filter(|line| {
                !keys
                    .iter()
                    .any(|key| line.trim_start().starts_with(&format!("{key} =")))
            })
            .map(|line| format!("{line}\n"))
            .collect()
    }
    let without = common::altered_reference("no_0092", |dir| {
        common::edit_file(dir, "surface_combat.toml", |t| {
            drop_keys(
                t,
                &[
                    "bombardment_minutes",
                    "bombardment_hit_chance",
                    "bombardment_reference_gun_damage",
                    "bombardment_false_claim_chance",
                ],
            )
        });
        common::edit_file(dir, "air_combat.toml", |t| drop_keys(t, &["men"]));
        common::edit_file(dir, "ground.toml", |t| {
            drop_keys(
                t,
                &[
                    "men_lost_per_base_hit",
                    "report_error_low",
                    "report_error_high",
                ],
            )
        });
    });
    // The control: the old files really have none of the new figures.
    {
        let path = common::data_dir().join("scenarios/06a_base_raid.toml");
        let source = std::fs::read_to_string(&path).expect("the scenario reads");
        let with_ground = common::replace_once(
            &source,
            "air_combat = \"../reference/air_combat.toml\"\n",
            "air_combat = \"../reference/air_combat.toml\"\nground = \"../reference/ground.toml\"\n",
        );
        let old = common::from_text_with(&with_ground, &path, &without).expect("loads");
        let figures = old.world.ref_data();
        assert!(figures
            .surface_combat
            .as_ref()
            .unwrap()
            .bombardment
            .is_none());
        assert_eq!(
            figures.air_combat.as_ref().unwrap().base_hit_men,
            Fixed::ZERO
        );
        let ground = figures.ground.as_ref().unwrap();
        assert_eq!(ground.men_lost_per_base_hit, Fixed::ZERO);
        assert!(ground.report_error.is_none());
        let new = common::from_text_with(&with_ground, &path, &common::frozen()).expect("loads");
        let figures = new.world.ref_data();
        assert!(figures
            .surface_combat
            .as_ref()
            .unwrap()
            .bombardment
            .is_some());
        assert_ne!(
            figures.air_combat.as_ref().unwrap().base_hit_men,
            Fixed::ZERO
        );
    }
    for file in ["06a_base_raid.toml", "04a_night_action.toml"] {
        let path = common::data_dir().join("scenarios").join(file);
        let source = std::fs::read_to_string(&path).expect("the scenario reads");
        let mut variants = vec![source.clone()];
        if source.contains("air_combat = \"../reference/air_combat.toml\"\n") {
            // Also with the ground figures named, and no man in the game.
            variants.push(common::replace_once(
                &source,
                "air_combat = \"../reference/air_combat.toml\"\n",
                "air_combat = \"../reference/air_combat.toml\"\nground = \"../reference/ground.toml\"\n",
            ));
        }
        for text in &variants {
            for seed in [1942u32, 1, 2, 3, 4] {
                let seeded = text.replacen("seed = 1942\n", &format!("seed = {seed}\n"), 1);
                let run = |options: &sim_data::LoadOptions| {
                    let mut scenario =
                        common::from_text_with(&seeded, &path, options).expect("loads");
                    while scenario.next_day().expect("a day resolves").is_some() {}
                    scenario
                };
                let with = run(&common::frozen());
                let old = run(&without);
                let a: Vec<Event> = with.world.events().iter().cloned().collect();
                let b: Vec<Event> = old.world.events().iter().cloned().collect();
                assert!(!a.is_empty());
                assert_eq!(a, b, "{file} seed {seed}: identical logs");
                assert_eq!(
                    with.world.umpire(),
                    old.world.umpire(),
                    "{file} seed {seed}"
                );
                assert_eq!(
                    with.world.state_hash(),
                    old.world.state_hash(),
                    "{file} seed {seed}"
                );
            }
        }
    }
}

// ------------------------------------------------------------------ Edges

/// A run of a variant of scenario 24 that never bombards: no ship fires, the
/// base is as it was but for the nightly charge for its men, and the Japanese
/// side is told nothing and claims nothing.
fn assert_no_bombardment(run: &Run) {
    let events = run.events();
    assert!(ship_fires(&events).is_empty());
    assert!(shores(&events).is_empty());
    assert!(bombardment_claims(&events, Side::Japanese).is_empty());
    assert!(!events.iter().any(|e| matches!(
        e.kind,
        EventKind::MenLostToFire { .. } | EventKind::BaseSupplyDestroyed { .. }
    )));
    check_nights(run);
    assert_eq!(run.aircraft(run.len()), (20, 4));
    assert_eq!(run.men(run.len(), 1), 1000);
    assert_eq!(run.ship_rounds(run.len(), 1), n(800));
    assert_eq!(run.ship_rounds(run.len(), 2), n(600));
    let japanese = run.scenario.world.side_view(Side::Japanese).events();
    assert!(!japanese.iter().any(|e| matches!(
        e.kind,
        SideEventKind::BombardmentFire { .. } | SideEventKind::ShoreBombardment { .. }
    )));
    let allied = run.scenario.world.side_view(Side::Allied).events();
    assert!(!allied
        .iter()
        .any(|e| matches!(e.kind, SideEventKind::ShoreBombardmentAgainst { .. })));
}

#[test]
fn e_a_force_that_passes_through_the_hex_in_the_night_does_not_bombard() {
    // Its route goes beyond (0,0), to (-1,0): at the end of the night it is
    // not in the hex.
    let source = common::replace_once(&text(), "waypoints = [[0, 0]]", "waypoints = [[-1, 0]]");
    let run = Run::text(&source);
    let force = &run.at(1).task_forces[&sim_core::ids::TaskForceId::new(1)];
    assert_eq!(
        force.position(),
        Hex::new(-1, 0),
        "the force passed X in the night"
    );
    assert_no_bombardment(&run);
    // And on no night does it end in X: day 3 sends it to (2,0), through X.
    for day in 1..=run.len() {
        assert_ne!(
            run.at(day).task_forces[&sim_core::ids::TaskForceId::new(1)].position(),
            X,
            "day {day}"
        );
    }
}

#[test]
fn e_a_base_off_the_sides_map_is_not_bombarded() {
    let source = common::replace_once(
        &text(),
        "supply_t = 2000\n",
        "supply_t = 2000\non_map = false\n",
    );
    let run = Run::text(&source);
    // The force is at (0,0) all the same.
    assert_eq!(
        run.at(1).task_forces[&sim_core::ids::TaskForceId::new(1)].position(),
        X
    );
    assert_no_bombardment(&run);
}

#[test]
fn e_a_force_not_on_the_bombardment_mission_does_not_bombard() {
    let source = common::replace_once(&text(), "mission = \"bombardment\"", "mission = \"escort\"");
    let run = Run::text(&source);
    assert_eq!(
        run.at(1).task_forces[&sim_core::ids::TaskForceId::new(1)].position(),
        X
    );
    assert_no_bombardment(&run);
}

#[test]
fn e_a_ship_with_its_main_battery_out_fires_nothing_and_the_rest_of_the_force_does() {
    let source = common::replace_once(
        &text(),
        "gun_rounds = 800\n",
        "gun_rounds = 800\nmain_battery = \"out\"\n",
    );
    let run = Run::text(&source);
    let events = run.events();
    let fires = ship_fires(&events);
    // The cruiser fires nothing and keeps its rounds; the destroyer fires
    // its 600 on night 1 and has nothing for night 2.
    assert!(!fires.iter().any(|f| f.ship == 1));
    assert_eq!(fires.len(), 1);
    assert_eq!(
        (
            fires[0].day,
            fires[0].ship,
            fires[0].shells,
            fires[0].weighed
        ),
        (1, 2, n(600), n(150))
    );
    assert_eq!(run.ship_rounds(4, 1), n(800));
    assert_eq!(run.ship_rounds(4, 2), n(0));
    let all = shores(&events);
    assert_eq!(all.len(), 1);
    assert_eq!((all[0].shells, all[0].weighed), (n(600), n(150)));
    check_nights(&run);
    // The claim and the sighting still follow, from the one ship that fired.
    assert_eq!(bombardment_claims(&events, Side::Japanese).len(), 1);

    // A degraded battery fires the degraded share of its guns.
    let source = common::replace_once(
        &text(),
        "gun_rounds = 800\n",
        "gun_rounds = 800\nmain_battery = \"degraded\"\n",
    );
    let run = Run::text(&source);
    let factor_degraded = run
        .scenario
        .world
        .ref_data()
        .surface_combat
        .as_ref()
        .unwrap()
        .degraded_factor;
    let fires = ship_fires(&run.events());
    let cruiser = fires
        .iter()
        .find(|f| f.day == 1 && f.ship == 1)
        .expect("fires");
    assert_eq!(cruiser.shells, n(720) * factor_degraded);
    assert_eq!(cruiser.weighed, cruiser.shells);
    check_nights(&run);
}
