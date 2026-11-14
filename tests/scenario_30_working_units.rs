//! Scenario 30: units that work. Working units build an airfield and an air
//! group can then be flown to it; work on a port speeds the next day's
//! unloading, up to twice; work on aircraft repairs more of them the next
//! night; work on defences raises a defender's chance to hit; a fighting
//! unit set to work makes half as much and neither trains nor rehearses;
//! hungry men make less; working men do not fire and do not hold a base.
//!
//! Proves decision 0099 (units that work, and what they build) against a
//! hand calculation. The data is placeholder data: see
//! `data/scenarios/30_working_units.toml`. The frozen step 1 figures hold
//! the seven figures of this rule at the values below. The strengths of
//! experience and of the skills are 0 there, and this test leaves them so.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide (the hits
//! of a fight), the calculation gives the shots and the chance exactly; the
//! tests read the hits from the run. Everything else is asserted exactly.
//! If the code disagrees, the code or the rule is wrong: do not change a
//! number here to make a test pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! `work_per_man_day` 1, `airfield_work` 20000, `work_to_double_unloading`
//! 1000, `work_per_aircraft_repair` 200, `work_per_defence_point` 200,
//! `defences_strength` 0.5, `fighting_unit_work_share` 0.5.
//!
//! From the earlier rules: `daily_supply_t_per_man` 0.005,
//! `unfed_loss_share_per_day` 0.05, `men_per_shot` 100, `hit_chance` 0.3,
//! `defender_hit_chance` 0.6, `men_per_hit` 20, `beaten_share` 0.2,
//! `captured_stock_share` 0.5, `preparation_per_day` 5,
//! `training_days_per_level` 30. A size 0 port works cargo by lighter at 20
//! t an hour. The night pulse is 12 hours and each daylight pulse 6.
//!
//! The scenario names the wear and repair figures and not the air supply
//! figures: aircraft are repaired and cost no supply. No group has a
//! mission, so nothing flies and nothing wears.
//!
//! A unit's work in a day is its men ashore, times its fed share, times 1,
//! times its works skill over 2, and for a fighting unit times 0.5. It is
//! made in the ground combat phase, after the fights, in a hex that holds a
//! base of its side, on a day the unit did not march and its hex was not
//! contested.
//!
//! ## Inputs
//!
//! A placeholder map with five coast hexes on one axis, all clear, port
//! size 0, no land links: A at (0,0), B at (5,0), C at (10,0), D at (15,0)
//! and E at (20,0). The game is 6 days. Day d begins at hour 24 x (d - 1).
//!
//! | Base | Side     | Hex | Supply  | Airfield                 | Defences | Repairs a day |
//! | ---- | -------- | --- | ------- | ------------------------ | -------- | ------------- |
//! | 1    | Allied   | A   | 5000 t  | none, 17000 of work done | 0        | 0             |
//! | 2    | Allied   | B   | 5000 t  | yes                      | 0        | 0             |
//! | 3    | Allied   | C   | 13.5 t  | yes                      | 0        | 1             |
//! | 4    | Allied   | D   | 5000 t  | yes                      | 40       | 0             |
//! | 5    | Japanese | D   | 5000 t  | yes                      | 0        | 0             |
//! | 6    | Allied   | E   | 1000 t  | none, 5000 of work done  | 30       | 0             |
//! | 7    | Japanese | E   | 100 t   | yes                      | 0        | 0             |
//!
//! Every unit has experience 2 but unit 4, and every skill 2 but as stated.
//!
//! | Unit | Side     | Kind     | Men  | Hex | Stated                | Posture |
//! | ---- | -------- | -------- | ---- | --- | --------------------- | ------- |
//! | 1    | Allied   | working  | 1000 | A   |                       | defend  |
//! | 2    | Allied   | working  | 400  | A   | Works 4               | defend  |
//! | 3    | Allied   | working  | 500  | B   |                       | defend  |
//! | 4    | Allied   | fighting | 3000 | B   | Experience 1          | defend  |
//! | 5    | Allied   | working  | 600  | C   |                       | defend  |
//! | 6    | Allied   | fighting | 1000 | D   |                       | defend  |
//! | 7    | Allied   | working  | 2000 | D   |                       | defend  |
//! | 8    | Japanese | fighting | 1000 | D   |                       | attack  |
//! | 9    | Allied   | working  | 500  | E   |                       | defend  |
//! | 10   | Japanese | fighting | 100  | E   |                       | attack  |
//! | 11   | Allied   | fighting | 900  | A   |                       | defend  |
//! | 12   | Allied   | fighting | 600  | B   |                       | defend  |
//!
//! Forces, all Allied, all at base 2 and stopped there throughout:
//!
//! | Force | Ship                                         |
//! | ----- | -------------------------------------------- |
//! | 1     | 1, placeholder cargo ship, 1500 t of supply  |
//! | 2     | 2, placeholder cargo ship, 1000 t of supply  |
//! | 3     | 3, placeholder troop transport, empty        |
//! | 4     | 4, placeholder troop transport, empty        |
//!
//! Air groups, both Allied, neither with a mission:
//!
//! - Group 1: placeholder fighter (transfer range 500 nm), at base 2, 12
//!   ready.
//! - Group 2: placeholder flying boat, at base 3, 2 ready and 20 damaged.
//!
//! Orders:
//!
//! - Day 1: unit 3 works at the port. Unit 4 trains. Unit 5 works at the
//!   aircraft. Unit 11 works at the defences. Force 1 unloads. Force 3
//!   prepares a landing at E with unit 12 as its roster. Units 1, 2, 7 and
//!   9 are given no order.
//! - Day 2: rebase group 1 to base 1.
//! - Day 3: rebase group 1 to base 1. Unit 4 works at the port. Unit 12
//!   works at the defences.
//! - Day 4: force 2 unloads.
//! - Day 5: unit 12 stands ready. Force 4 prepares a landing at E with unit
//!   12 as its roster.
//!
//! ## At A: an airfield built, and defences dug
//!
//! Units 1 and 2 have no order. Base 1 has no airfield, so they build it.
//!
//! - Unit 1: 1000 x 1 x 2/2 = 1000 a day.
//! - Unit 2: 400 x 1 x 4/2 = 800 a day.
//!
//! | After day | Work done on the airfield | Airfield                    |
//! | --------- | ------------------------- | --------------------------- |
//! | 1         | 18800                     | none                        |
//! | 2         | 20600                     | built, in the phase of day 2 |
//! | 6         | 20600                     | built                       |
//!
//! The day's work is added whole on the day the airfield is built, and
//! nothing is added after. From day 3 units 1 and 2, still with no order,
//! work the port of base 1: 1800 a day. No ship is there and it changes
//! nothing.
//!
//! The rebase of group 1 from base 2, 5 hexes and 200 nm away, within 500:
//!
//! - Day 2, at intake: refused, as base 1 has no airfield.
//! - Day 3, at intake: accepted. Group 1 is at base 1 with 12 ready.
//!
//! Unit 11, a fighting unit at the defences: 900 x 1 x 2/2 x 0.5 = 450 a
//! day, 450 / 200 = 2.25 points a day. Defences are not cut to a whole
//! number. Base 1's defences are 2.25 after day 1 and 13.5 after day 6.
//!
//! Supply. Base 1 feeds 2300 men, 11.5 t a night, six times: 4931 t at the
//! end.
//!
//! ## At B: the port, and one activity a day
//!
//! Work on the port of base 2, and the rate of the day after:
//!
//! | Day | Unit 3 | Unit 4                        | Work on the port | Rate the next day                 |
//! | --- | ------ | ----------------------------- | ---------------- | --------------------------------- |
//! | 1   | 500    | trains                        | 500              | 20 x 1.5 = 30 t an hour           |
//! | 2   | 500    | trains                        | 500              | 30 t an hour                      |
//! | 3   | 500    | 3000 x 0.5 = 1500             | 2000             | 20 x 3, held at twice: 40 t an hour |
//! | 4   | 500    | 1500                          | 2000             | 40 t an hour                      |
//! | 5   | 500    | 1500                          | 2000             | 40 t an hour                      |
//! | 6   | 500    | 1500                          | 2000             | the game is over                  |
//!
//! Day 1 has no work from the day before: 20 t an hour.
//!
//! Force 1, 1500 t, unloading from the night pulse of day 1:
//!
//! | Day | Rate        | Night, 12 h | Morning, 6 h | Afternoon, 6 h | Landed so far |
//! | --- | ----------- | ----------- | ------------ | -------------- | ------------- |
//! | 1   | 20 t an hour | 240 t      | 120 t        | 120 t          | 480 t         |
//! | 2   | 30 t an hour | 360 t      | 180 t        | 180 t          | 1200 t        |
//! | 3   | 30 t an hour | 300 t      |              |                | 1500 t        |
//!
//! It is complete 300 / 30 = 10 hours into the night pulse of day 3, at
//! hour 58.
//!
//! Force 2, 1000 t, unloading from the night pulse of day 4 at 40 t an
//! hour: 480 t, 240 t and 240 t on day 4, 960 t. The last 40 t take 1 hour
//! of the night pulse of day 5: complete at hour 97.
//!
//! Unit 4, a fighting unit of experience 1:
//!
//! | After day | Its day was | Days of training counted | Experience |
//! | --------- | ----------- | ------------------------ | ---------- |
//! | 1         | training    | 1                        | 1          |
//! | 2         | training    | 2                        | 1          |
//! | 3 to 6    | work        | 2                        | 1          |
//!
//! Unit 12 and the two forces. Unit 12 is ashore at B, in the hex of both
//! forces.
//!
//! | After day | Unit 12's day was            | Its rehearsal, for E | Force 3's loading | Force 4's loading | Base 2's defences |
//! | --------- | ---------------------------- | -------------------- | ----------------- | ----------------- | ----------------- |
//! | 1         | rehearsal, with force 3      | 5                    | 5                 | 0                 | 0                 |
//! | 2         | rehearsal, with force 3      | 10                   | 10                | 0                 | 0                 |
//! | 3         | work: 600 x 0.5 = 300        | 10                   | 15                | 0                 | 1.5               |
//! | 4         | work                         | 10                   | 20                | 0                 | 3                 |
//! | 5         | rehearsal, with force 4      | 15                   | 20                | 5                 | 3                 |
//! | 6         | rehearsal, with force 4      | 20                   | 20                | 10                | 3                 |
//!
//! - Days 3 and 4: unit 12 works. It stays on force 3's roster, the force
//!   goes on loading and nothing is cut.
//! - Day 5: force 4's order takes unit 12 from force 3's roster, which
//!   costs force 3 nothing. Force 3's roster is empty: its order stands,
//!   it keeps its 20 and gains no more. Force 4 begins at 0. Unit 12 keeps
//!   its rehearsal of 10, which is for E, the hex of the new order.
//!
//! Supply. Base 2 feeds units 3, 4 and 12, 4100 men, 20.5 t a night, six
//! times: 123 t. It is given 1500 t and 1000 t. It holds 5000 - 123 + 2500
//! = 7377 t at the end.
//!
//! ## At C: aircraft repaired, and hungry men
//!
//! Unit 5 costs 600 x 0.005 = 3 t a night. Repair comes before the men's
//! supply in the night pulse. A night's repairs at base 3 are its own 1 and
//! one more for each whole 200 of the work on the aircraft of the day
//! before.
//!
//! | Day | Night: repairs | Group 2, ready and damaged | Night: base 3 pays | Stock | Men fed | Fed share | Men lost | Unit 5's men | Work that day |
//! | --- | -------------- | -------------------------- | ------------------ | ----- | ------- | --------- | -------- | ------------ | ------------- |
//! | 1   | 1              | 3 and 19                   | 3 t                | 10.5 t | 600    | 1         | 0        | 600          | 600           |
//! | 2   | 1 + 3 = 4      | 7 and 15                   | 3 t                | 7.5 t | 600     | 1         | 0        | 600          | 600           |
//! | 3   | 4              | 11 and 11                  | 3 t                | 4.5 t | 600     | 1         | 0        | 600          | 600           |
//! | 4   | 4              | 15 and 7                   | 3 t                | 1.5 t | 600     | 1         | 0        | 600          | 600           |
//! | 5   | 4              | 19 and 3                   | 1.5 t              | 0 t   | 300     | 0.5       | 15       | 585          | 292.5         |
//! | 6   | 1 + 1 = 2      | 21 and 1                   | nothing            | 0 t   | 0       | 0         | 29       | 556          | 0             |
//!
//! - Night of day 5: 1.5 t feeds 300 men. 300 are unfed and 300 x 0.05 =
//!   15 are lost. The day's work is 585 x 0.5 = 292.5.
//! - Night of day 6: 292.5 / 200 gives 1 more repair. Nobody is fed: 585 x
//!   0.05 = 29.25, 29 men lost.
//!
//! ## At D: defences in a fight, and working men under fire
//!
//! Night of day 1: base 4 pays 3000 x 0.005 = 15 t and holds 4985 t; base 5
//! pays 5 t and holds 4995 t. All are fed.
//!
//! Day 1. The hex is contested: both sides have fighting men ashore and
//! unit 8 is on attack.
//!
//! - Unit 8: 1000 firing men, 10 shots at 0.3.
//! - Unit 6, defending behind defences of 40: 1000 firing men, 10 shots at
//!   0.6 x (1 + 0.5 x 40/100) = 0.6 x 1.2 = 0.72.
//! - Unit 7 fires nothing.
//!
//! The men the Allied side loses, 20 for each hit, are taken from units 6
//! and 7 in proportion to their men, by the rule that serves every loss.
//! For one hit: 20 x 1000/3000 is 6 from unit 6 and 20 x 2000/3000 is 13
//! from unit 7, and the man still to take comes from unit 6: 7 and 13.
//!
//! The test of a side beaten counts unit 6's men and not unit 7's. Ten hits
//! would leave unit 6 with at least 933 men against 0.2 x 1000 = 200: the
//! Allied side is not beaten on day 1 whatever the dice give.
//!
//! Unit 7 makes no work on a day the hex is contested. On a day it is not
//! (unit 8 back on defend), unit 7, with no order at a base that has an
//! airfield, works the port of base 4: its men ashore times its fed share.
//! The tests assert that from the run, day by day, and that base 4's
//! defences are 40 at the end.
//!
//! ## At E: working men do not hold a base
//!
//! Night of day 1: base 6 pays 500 x 0.005 = 2.5 t and holds 997.5 t. Base
//! 7 pays 100 x 0.005 = 0.5 t and holds 99.5 t.
//!
//! Day 1. The Allied side has no fighting man ashore at E, so the hex is
//! not contested and there is no fight. Unit 10 is on attack and the Allied
//! side has a base there: the Allied side is beaten without a fight, and
//! base 6 changes side.
//!
//! - Base 6 keeps 997.5 x 0.5 = 498.75 t, and 498.75 t are lost.
//! - It keeps its defences of 30 and its 5000 of work done on the airfield.
//!   It still has no airfield.
//! - Unit 9 makes no work on day 1: when the work is made its side has no
//!   base in the hex. It makes none after.
//! - Unit 10 is not tired: it attacked nothing.
//!
//! From the night of day 2 the Allied side has no base at E, and unit 9 is
//! unfed, with a fed share of 0:
//!
//! | Night of day | Unfed | Lost             | Unit 9's men |
//! | ------------ | ----- | ---------------- | ------------ |
//! | 2            | 500   | 25               | 475          |
//! | 3            | 475   | 23.75, 23        | 452          |
//! | 4            | 452   | 22.6, 22         | 430          |
//! | 5            | 430   | 21.5, 21         | 409          |
//! | 6            | 409   | 20.45, 20        | 389          |
//!
//! From the night of day 2 unit 10 is fed by base 6, the lower-numbered of
//! its side's two bases there: 0.5 t a night, five times. Base 6 holds
//! 498.75 - 2.5 = 496.25 t at the end and base 7 holds 99.5 t.
//!
//! # Properties
//!
//! - No unit's day is counted twice: on no day does a unit gain two of
//!   work, training and rehearsal.
//! - Work never falls when men, fed share or works skill rise, and a day's
//!   port rate is never above twice the port's own.
//! - With no working unit, no activity order, every stated base with an
//!   airfield and no defences, a scenario gives what it gave before, apart
//!   from the hash version.
//! - The enemy's view is the same in two worlds that differ only in the
//!   other side's activities, jobs, work done and defences, where the hits
//!   are the same.

mod common;

use std::path::PathBuf;

use sim_core::events::{Event, EventKind, SupplyUse};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, LandUnitId, TaskForceId};
use sim_core::orders::{Job, RejectReason};
use sim_core::refdata::GroundFigures;
use sim_core::state::{AirGroupLocation, Side, TruthState};
use sim_core::works::{port_factor, unit_work};
use sim_data::Scenario;

// ------------------------------------------------------------------ Helpers

const A: Hex = Hex::new(0, 0);
const B: Hex = Hex::new(5, 0);
const C: Hex = Hex::new(10, 0);
const D: Hex = Hex::new(15, 0);
const E: Hex = Hex::new(20, 0);

fn path() -> PathBuf {
    common::data_dir().join("scenarios/30_working_units.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// The scenario on the frozen step 1 figures, which hold the seven figures
/// of this rule at the values of the calculation.
fn from(source: &str) -> Scenario {
    common::from_text(source, &path()).expect("the variant loads on the frozen figures")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn unit(raw: u32) -> LandUnitId {
    LandUnitId::new(raw)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

/// The scenario text with `extra` appended: more orders.
fn with_orders(source: &str, extra: &str) -> String {
    format!("{source}\n{extra}")
}

/// One land unit order of the Allied side as scenario text.
fn activity_order(day: u32, unit: u32, activity: &str, job: Option<&str>) -> String {
    let job = job.map_or(String::new(), |j| format!("job = \"{j}\"\n"));
    format!(
        "\n[[order]]\nday = {day}\nside = \"allied\"\nland_unit = {unit}\nkind = \"set_activity\"\nactivity = \"{activity}\"\n{job}"
    )
}

/// A run of the scenario: the truth at the start (day 0) and at the end of
/// each day, the scenario at the end, and its log.
struct Run {
    scenario: Scenario,
    days: Vec<TruthState>,
    events: Vec<Event>,
}

impl Run {
    fn of(mut scenario: Scenario) -> Run {
        let mut days = vec![scenario.world.umpire().clone()];
        while scenario.next_day().expect("a day resolves").is_some() {
            days.push(scenario.world.umpire().clone());
        }
        let events = scenario.world.events().iter().cloned().collect();
        Run {
            scenario,
            days,
            events,
        }
    }

    fn text(source: &str) -> Run {
        Run::of(from(source))
    }

    fn len(&self) -> u32 {
        u32::try_from(self.days.len() - 1).expect("a few days")
    }

    fn at(&self, day: u32) -> &TruthState {
        &self.days[day as usize]
    }

    fn unit_at(&self, day: u32, raw: u32) -> &sim_core::state::LandUnit {
        &self.at(day).land_units[&unit(raw)]
    }

    fn base_at(&self, day: u32, raw: u32) -> &sim_core::state::Base {
        &self.at(day).bases[&base(raw)]
    }

    fn loading(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).task_forces[&TaskForceId::new(raw)]
            .preparation
            .as_ref()
            .expect("preparing")
            .loading
    }

    fn on(&self, day: u32, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.day == day && pick(&e.kind))
            .collect()
    }

    fn all(&self, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events.iter().filter(|e| pick(&e.kind)).collect()
    }

    /// A base's supply after the night's feeding of day `day`.
    fn supply_after_night(&self, day: u32, raw: u32) -> Fixed {
        let found = self.on(day, |k| {
            matches!(
                k,
                EventKind::BaseSupplySpent { base: b, reason: SupplyUse::Men, .. } if b.raw() == raw
            )
        });
        assert_eq!(found.len(), 1, "base {raw} fed once on day {day}");
        match found[0].kind {
            EventKind::BaseSupplySpent { left_t, .. } => left_t,
            _ => unreachable!(),
        }
    }

    /// The `UnitWorked` events of a unit on a day: `(job, base, work, total)`.
    fn worked(&self, day: u32, raw: u32) -> Vec<(Job, u32, Fixed, Fixed)> {
        self.on(
            day,
            |k| matches!(k, EventKind::UnitWorked { unit, .. } if unit.raw() == raw),
        )
        .into_iter()
        .map(|e| match e.kind {
            EventKind::UnitWorked {
                job,
                base,
                work,
                total,
                ..
            } => (job, base.raw(), work, total),
            _ => unreachable!(),
        })
        .collect()
    }

    /// The work a unit made on a day, 0 where it has no event.
    fn work_made(&self, day: u32, raw: u32) -> Fixed {
        let found = self.worked(day, raw);
        assert!(found.len() <= 1, "unit {raw} worked once on day {day}");
        found.first().map_or(Fixed::ZERO, |w| w.2)
    }

    /// The tons a force's ship put ashore, by pulse, on `day`, with the rate
    /// of the breakdown: `(unloaded_t, rate_t_per_h)`.
    fn unloaded(&self, day: u32, raw_force: u32) -> Vec<(Fixed, Fixed)> {
        self.on(day, |k| {
            matches!(k, EventKind::CargoUnloaded { task_force, .. } if task_force.raw() == raw_force)
        })
        .into_iter()
        .map(|e| match e.kind {
            EventKind::CargoUnloaded { unloaded_t, .. } => {
                (unloaded_t, factor(e, "rate_t_per_h"))
            }
            _ => unreachable!(),
        })
        .collect()
    }

    fn fires(&self, day: u32) -> Vec<(u32, u32, Fixed, Fixed, u32)> {
        self.on(day, |k| matches!(k, EventKind::GroundFire { .. }))
            .into_iter()
            .map(|e| match e.kind {
                EventKind::GroundFire {
                    unit,
                    firing_men,
                    shots,
                    chance,
                    hits,
                    ..
                } => (unit.raw(), firing_men, shots, chance, hits),
                _ => unreachable!(),
            })
            .collect()
    }

    fn fire(&self, day: u32, raw: u32) -> (u32, Fixed, Fixed, u32) {
        let found: Vec<_> = self.fires(day).into_iter().filter(|f| f.0 == raw).collect();
        assert_eq!(found.len(), 1, "unit {raw} fired once on day {day}");
        (found[0].1, found[0].2, found[0].3, found[0].4)
    }

    /// The men lost in the fight on a day, by unit.
    fn fight_losses(&self, day: u32) -> Vec<(u32, u32)> {
        self.on(day, |k| matches!(k, EventKind::MenLostInFight { .. }))
            .into_iter()
            .map(|e| match e.kind {
                EventKind::MenLostInFight { unit, lost, .. } => (unit.raw(), lost),
                _ => unreachable!(),
            })
            .collect()
    }

    /// The `AircraftRepaired` events of a day: `(repaired, ready, damaged)`.
    fn repairs(&self, day: u32, group: u32) -> Option<(u32, u32, u32)> {
        let found = self.on(day, |k| {
            matches!(k, EventKind::AircraftRepaired { air_group, .. } if air_group.raw() == group)
        });
        assert!(found.len() <= 1);
        found.first().map(|e| match e.kind {
            EventKind::AircraftRepaired {
                repaired,
                ready,
                damaged,
                ..
            } => (repaired, ready, damaged),
            _ => unreachable!(),
        })
    }
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

/// The text of a side's view of the events of `day` and of its estimates.
fn day_digest(scenario: &Scenario, side: Side, day: u32) -> String {
    day_digest_with(scenario, side, day, true)
}

/// As [`day_digest`], with the side's estimates of the enemy ashore (which
/// are as of the end of the game) left out where `estimates` is false.
fn day_digest_with(scenario: &Scenario, side: Side, day: u32, estimates: bool) -> String {
    let view = scenario.world.side_view(side);
    let mut out = String::new();
    for e in view.events_on_day(day) {
        out += &format!(
            "{} {} {:?} {:?} {:?} {:?} {} {:?}\n",
            e.number,
            e.day,
            e.pulse,
            e.kind,
            e.order_number,
            e.causes,
            e.hidden_causes,
            e.breakdown
        );
    }
    for e in view.ground_estimates().filter(|_| estimates) {
        out += &format!("{:?} {} {}\n", e.hex, e.men, e.day);
    }
    for b in view.enemy_bases() {
        out += &format!("{} {:?} {:?} {:?}\n", b.id.raw(), b.name, b.hex, b.side);
    }
    out
}

/// A whole run's log as text, event numbers and all.
fn log_text(run: &Run) -> String {
    run.events
        .iter()
        .map(|e| format!("{e:?}\n"))
        .collect::<String>()
}

fn figures(run: &Run) -> GroundFigures {
    run.scenario
        .world
        .ref_data()
        .ground
        .expect("ground figures")
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_figures_and_the_inputs_are_the_calculations() {
    let scenario = from(&text());
    let g = scenario.world.ref_data().ground.expect("ground figures");
    assert_eq!(g.work_per_man_day, n(1));
    assert_eq!(g.airfield_work, n(20000));
    assert_eq!(g.work_to_double_unloading, n(1000));
    assert_eq!(g.work_per_aircraft_repair, n(200));
    assert_eq!(g.work_per_defence_point, n(200));
    assert_eq!(g.defences_strength, fx("0.5"));
    assert_eq!(g.fighting_unit_work_share, fx("0.5"));
    assert_eq!(g.daily_supply_t_per_man, fx("0.005"));
    assert_eq!(g.men_per_shot, n(100));
    assert_eq!(g.hit_chance, fx("0.3"));
    assert_eq!(g.defender_hit_chance, fx("0.6"));
    assert_eq!(g.preparation_per_day, n(5));
    assert_eq!(g.beaten_share, fx("0.2"));
    assert_eq!(g.captured_stock_share, fx("0.5"));
    assert_eq!(g.training_days_per_level, 30);
    assert_eq!(g.experience_fire_strength, n(0));

    let truth = scenario.world.umpire();
    let map = scenario.world.map();
    for hex in [A, B, C, D, E] {
        assert!(map.get(hex).is_some(), "{hex:?} is on the map");
    }
    // Bases: id, side, hex, supply, airfield, work done, defences, repairs.
    assert_eq!(truth.bases.len(), 7);
    for (id, side, hex, supply, airfield, work, defences, repairs) in [
        (1, Side::Allied, A, fx("5000"), false, n(17000), n(0), 0),
        (2, Side::Allied, B, fx("5000"), true, n(0), n(0), 0),
        (3, Side::Allied, C, fx("13.5"), true, n(0), n(0), 1),
        (4, Side::Allied, D, fx("5000"), true, n(0), n(40), 0),
        (5, Side::Japanese, D, fx("5000"), true, n(0), n(0), 0),
        (6, Side::Allied, E, fx("1000"), false, n(5000), n(30), 0),
        (7, Side::Japanese, E, fx("100"), true, n(0), n(0), 0),
    ] {
        let b = &truth.bases[&base(id)];
        assert_eq!(
            (
                b.side,
                b.hex,
                b.supply_t,
                b.airfield,
                b.airfield_work,
                b.defences,
                b.aircraft_repairs_per_day
            ),
            (side, hex, supply, airfield, work, defences, repairs),
            "base {id}"
        );
    }
    // Units: id, side, working, men, hex, experience, works skill, posture.
    use sim_core::orders::Posture::{Attack, Defend};
    assert_eq!(truth.land_units.len(), 12);
    for (id, side, working, men, hex, experience, works, posture) in [
        (1, Side::Allied, true, 1000, A, 2, 2, Defend),
        (2, Side::Allied, true, 400, A, 2, 4, Defend),
        (3, Side::Allied, true, 500, B, 2, 2, Defend),
        (4, Side::Allied, false, 3000, B, 1, 2, Defend),
        (5, Side::Allied, true, 600, C, 2, 2, Defend),
        (6, Side::Allied, false, 1000, D, 2, 2, Defend),
        (7, Side::Allied, true, 2000, D, 2, 2, Defend),
        (8, Side::Japanese, false, 1000, D, 2, 2, Attack),
        (9, Side::Allied, true, 500, E, 2, 2, Defend),
        (10, Side::Japanese, false, 100, E, 2, 2, Attack),
        (11, Side::Allied, false, 900, A, 2, 2, Defend),
        (12, Side::Allied, false, 600, B, 2, 2, Defend),
    ] {
        let u = &truth.land_units[&unit(id)];
        assert_eq!(
            (
                u.side,
                u.working,
                u.men_ashore,
                u.hex,
                u.experience,
                u.skills.works,
                u.posture
            ),
            (side, working, men, Some(hex), experience, works, posture),
            "unit {id}"
        );
    }
    assert_eq!(truth.task_forces.len(), 4);
    assert_eq!(truth.ships.len(), 4);
    for (raw, supply) in [(1, 1500), (2, 1000), (3, 0), (4, 0)] {
        assert_eq!(
            truth
                .ships
                .values()
                .find(|s| s.id.raw() == raw)
                .expect("ship")
                .supply_t,
            n(supply),
            "ship {raw}"
        );
    }
    let g1 = &truth.air_groups[&AirGroupId::new(1)];
    assert_eq!(
        (g1.location, g1.ready, g1.damaged),
        (AirGroupLocation::Base(base(2)), 12, 0)
    );
    let g2 = &truth.air_groups[&AirGroupId::new(2)];
    assert_eq!(
        (g2.location, g2.ready, g2.damaged),
        (AirGroupLocation::Base(base(3)), 2, 20)
    );
    // Orders by day: 6 on day 1, 1 on day 2, 3 on day 3, 1 on day 4, 2 on
    // day 5.
    let orders = scenario.scripted_orders();
    let counts: Vec<(u32, usize)> = orders.iter().map(|(d, o)| (*d, o.len())).collect();
    assert_eq!(counts, [(1, 6), (2, 1), (3, 3), (4, 1), (5, 2)]);
}

#[test]
fn at_a_the_airfield_is_built_and_group_1_is_flown_to_it_and_defences_are_dug() {
    let run = Run::text(&text());
    assert_eq!(run.len(), 6);

    // Work done on the airfield.
    for (day, done) in [(0, 17000), (1, 18800), (2, 20600), (3, 20600), (6, 20600)] {
        assert_eq!(run.base_at(day, 1).airfield_work, n(done), "day {day}");
    }
    for day in 0..=1 {
        assert!(!run.base_at(day, 1).airfield, "no airfield after day {day}");
    }
    for day in 2..=6 {
        assert!(run.base_at(day, 1).airfield, "an airfield after day {day}");
    }
    // The airfield is built in the phase of day 2, once.
    let built = run.all(|k| matches!(k, EventKind::AirfieldBuilt { .. }));
    assert_eq!(built.len(), 1);
    assert_eq!(built[0].day, 2);
    assert_eq!(
        built[0].kind,
        EventKind::AirfieldBuilt {
            base: base(1),
            side: Side::Allied,
            hex: A,
            work_done: n(20600),
        }
    );
    // Units 1 and 2 build it on days 1 and 2.
    assert_eq!(run.worked(1, 1), [(Job::Airfield, 1, n(1000), n(18000))]);
    assert_eq!(run.worked(1, 2), [(Job::Airfield, 1, n(800), n(18800))]);
    assert_eq!(run.worked(2, 1), [(Job::Airfield, 1, n(1000), n(19800))]);
    assert_eq!(run.worked(2, 2), [(Job::Airfield, 1, n(800), n(20600))]);
    // From day 3 they work the port of base 1: 1800 a day.
    for day in 3..=6 {
        assert_eq!(run.worked(day, 1), [(Job::Port, 1, n(1000), n(1000))]);
        assert_eq!(run.worked(day, 2), [(Job::Port, 1, n(800), n(1800))]);
    }

    // Group 1: refused on day 2 for want of an airfield, accepted on day 3.
    let rejected = run.all(|k| matches!(k, EventKind::OrderRejected { .. }));
    assert_eq!(rejected.len(), 1);
    assert_eq!(rejected[0].day, 2);
    assert!(matches!(
        rejected[0].kind,
        EventKind::OrderRejected {
            reason: RejectReason::NoAirfield,
            ..
        }
    ));
    for day in [0, 1, 2] {
        assert_eq!(
            run.at(day).air_groups[&AirGroupId::new(1)].location,
            AirGroupLocation::Base(base(2)),
            "day {day}"
        );
    }
    let moved = run.all(|k| matches!(k, EventKind::AirGroupRebased { .. }));
    assert_eq!(moved.len(), 1);
    assert_eq!(moved[0].day, 3);
    assert_eq!(
        moved[0].kind,
        EventKind::AirGroupRebased {
            air_group: AirGroupId::new(1),
            from: base(2),
            to: base(1),
            distance_nm: n(200),
            transfer_range_nm: n(500),
            ready: 12,
            pilots: None,
        }
    );
    for day in 3..=6 {
        let g = &run.at(day).air_groups[&AirGroupId::new(1)];
        assert_eq!(g.location, AirGroupLocation::Base(base(1)), "day {day}");
        assert_eq!(g.ready, 12, "day {day}");
    }

    // Unit 11, a fighting unit at the defences: 450 a day, 2.25 points.
    for day in 1..=6u32 {
        assert_eq!(
            run.worked(day, 11),
            [(Job::Defences, 1, n(450), fx("2.25") * n(i64::from(day)))],
            "day {day}"
        );
        assert_eq!(run.base_at(day, 1).defences, fx("2.25") * n(i64::from(day)));
    }
    assert_eq!(run.base_at(1, 1).defences, fx("2.25"));
    assert_eq!(run.base_at(6, 1).defences, fx("13.5"));

    // Supply: 2300 men, 11.5 t a night, six times.
    for day in 1..=6u32 {
        assert_eq!(
            run.supply_after_night(day, 1),
            n(5000) - fx("11.5") * n(i64::from(day))
        );
    }
    assert_eq!(run.base_at(6, 1).supply_t, n(4931));
}

#[test]
fn at_b_the_port_is_worked_and_a_unit_does_one_activity_a_day() {
    let run = Run::text(&text());
    // Work on the port of base 2, day by day: unit 3's 500, and unit 4's
    // 1500 from day 3.
    for day in 1..=6u32 {
        let unit3 = run.worked(day, 3);
        assert_eq!(unit3, [(Job::Port, 2, n(500), n(500))], "day {day}");
        let unit4 = run.worked(day, 4);
        if day < 3 {
            assert!(unit4.is_empty(), "unit 4 trains on day {day}");
        } else {
            assert_eq!(unit4, [(Job::Port, 2, n(1500), n(2000))], "day {day}");
        }
        let day_work = run.base_at(day, 2).day_work.expect("work that day");
        assert_eq!(day_work.day, day);
        assert_eq!(day_work.port, if day < 3 { n(500) } else { n(2000) });
    }

    // Force 1: 20 t an hour on day 1, 30 on days 2 and 3.
    let one = |day| run.unloaded(day, 1);
    assert_eq!(one(1), [(n(240), n(20)), (n(120), n(20)), (n(120), n(20))]);
    assert_eq!(one(2), [(n(360), n(30)), (n(180), n(30)), (n(180), n(30))]);
    assert_eq!(one(3), [(n(300), n(30))]);
    assert!(one(4).is_empty());
    let done = run.all(
        |k| matches!(k, EventKind::UnloadingComplete { task_force, .. } if task_force.raw() == 1),
    );
    assert_eq!(done.len(), 1);
    assert_eq!(done[0].day, 3);
    assert!(matches!(
        done[0].kind,
        EventKind::UnloadingComplete { elapsed_hours, .. } if elapsed_hours == n(58)
    ));

    // Force 2: 40 t an hour from day 4.
    let two = |day| run.unloaded(day, 2);
    assert_eq!(two(4), [(n(480), n(40)), (n(240), n(40)), (n(240), n(40))]);
    assert_eq!(two(5), [(n(40), n(40))]);
    let done = run.all(
        |k| matches!(k, EventKind::UnloadingComplete { task_force, .. } if task_force.raw() == 2),
    );
    assert_eq!(done.len(), 1);
    assert_eq!(done[0].day, 5);
    assert!(matches!(
        done[0].kind,
        EventKind::UnloadingComplete { elapsed_hours, .. } if elapsed_hours == n(97)
    ));
    // The breakdown names the work of the day before and the factor it
    // gives.
    let day2 = run.on(2, |k| matches!(k, EventKind::CargoUnloaded { .. }));
    assert_eq!(factor(day2[0], "port_work"), n(500));
    assert_eq!(factor(day2[0], "port_work_factor"), fx("1.5"));
    let day4 = run.on(4, |k| matches!(k, EventKind::CargoUnloaded { .. }));
    assert_eq!(factor(day4[0], "port_work"), n(2000));
    assert_eq!(factor(day4[0], "port_work_factor"), n(2));

    // Unit 4: two days of training at experience 1, then work.
    for (day, trained) in [(1u32, Some(1u32)), (2, Some(2)), (3, None)] {
        let found = run.on(
            day,
            |k| matches!(k, EventKind::TrainingDay { unit, .. } if unit.raw() == 4),
        );
        match trained {
            Some(days) => {
                assert_eq!(found.len(), 1, "day {day}");
                assert_eq!(
                    found[0].kind,
                    EventKind::TrainingDay {
                        unit: unit(4),
                        days,
                        experience: 1
                    }
                );
            }
            None => assert!(found.is_empty(), "day {day}"),
        }
    }
    for day in 4..=6 {
        assert!(run
            .on(day, |k| matches!(k, EventKind::TrainingDay { .. }))
            .is_empty());
    }
    for (day, days) in [(1, 1), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2)] {
        let u = run.unit_at(day, 4);
        assert_eq!((u.training_days, u.experience), (days, 1), "day {day}");
    }

    // Unit 12 and the two forces.
    for (day, rehearsal, loading3, loading4, defences) in [
        (1, n(5), n(5), None, n(0)),
        (2, n(10), n(10), None, n(0)),
        (3, n(10), n(15), None, fx("1.5")),
        (4, n(10), n(20), None, n(3)),
        (5, n(15), n(20), Some(n(5)), n(3)),
        (6, n(20), n(20), Some(n(10)), n(3)),
    ] {
        assert_eq!(run.unit_at(day, 12).rehearsal, rehearsal, "day {day}");
        assert_eq!(run.unit_at(day, 12).rehearsal_hex, Some(E), "day {day}");
        assert_eq!(run.loading(day, 3), loading3, "force 3, day {day}");
        if let Some(l) = loading4 {
            assert_eq!(run.loading(day, 4), l, "force 4, day {day}");
        } else {
            assert!(
                run.at(day).task_forces[&TaskForceId::new(4)]
                    .preparation
                    .is_none(),
                "force 4 has no order on day {day}"
            );
        }
        assert_eq!(run.base_at(day, 2).defences, defences, "day {day}");
    }
    // Unit 12's day: rehearsal on days 1, 2, 5 and 6, work on days 3 and 4.
    for day in 1..=6u32 {
        let rehearsed = run.on(
            day,
            |k| matches!(k, EventKind::RehearsalGained { unit, .. } if unit.raw() == 12),
        );
        let worked = run.worked(day, 12);
        if (3..=4).contains(&day) {
            assert!(rehearsed.is_empty(), "day {day}");
            assert_eq!(
                worked,
                [(Job::Defences, 2, n(300), fx("1.5") * n(i64::from(day - 2)))]
            );
        } else {
            assert_eq!(rehearsed.len(), 1, "day {day}");
            assert!(worked.is_empty(), "day {day}");
            let task_force = if day <= 2 { 3 } else { 4 };
            assert!(matches!(
                rehearsed[0].kind,
                EventKind::RehearsalGained { task_force: t, hex, .. } if t.raw() == task_force && hex == E
            ));
        }
    }
    // Days 3 and 4: unit 12 stays on force 3's roster, and nothing is cut.
    for day in 3..=4 {
        assert!(run.at(day).task_forces[&TaskForceId::new(3)]
            .preparation
            .as_ref()
            .expect("preparing")
            .roster
            .contains(&unit(12)));
    }
    assert!(run
        .on(3, |k| matches!(
            k,
            EventKind::LoadingLost { .. } | EventKind::RehearsalLost { .. }
        ))
        .is_empty());
    assert!(run
        .on(4, |k| matches!(
            k,
            EventKind::LoadingLost { .. } | EventKind::RehearsalLost { .. }
        ))
        .is_empty());
    // Day 5: force 4's order takes unit 12 from force 3's roster, at no cost
    // to force 3. Force 3's loading stops at 20.
    let left = run.all(|k| matches!(k, EventKind::LeftRoster { .. }));
    assert_eq!(left.len(), 1);
    assert_eq!(left[0].day, 5);
    assert!(matches!(
        left[0].kind,
        EventKind::LeftRoster { unit: u, task_force: t, .. } if u.raw() == 12 && t.raw() == 3
    ));
    assert!(run.at(5).task_forces[&TaskForceId::new(3)]
        .preparation
        .as_ref()
        .expect("still preparing")
        .roster
        .is_empty());
    let gained3 = run
        .all(|k| matches!(k, EventKind::LoadingGained { task_force, .. } if task_force.raw() == 3));
    assert_eq!(
        gained3.iter().map(|e| e.day).collect::<Vec<_>>(),
        [1, 2, 3, 4]
    );

    // Supply: 4100 men, 20.5 t a night: 123 t over the game, and 2500 t
    // given.
    for day in 1..=6u32 {
        assert_eq!(
            run.supply_after_night(day, 2),
            n(5000) - fx("20.5") * n(i64::from(day)) + landed_before_night(&run, day),
            "day {day}"
        );
    }
    assert_eq!(run.base_at(6, 2).supply_t, n(7377));
}

/// The tons put ashore at base 2 before the night of `day`'s feeding is
/// recorded: the night's own unloading comes after the feeding, so it is the
/// tons landed on the days before.
fn landed_before_night(run: &Run, day: u32) -> Fixed {
    let mut sum = Fixed::ZERO;
    for e in run.events.iter().filter(|e| e.day < day) {
        if let EventKind::CargoUnloaded { unloaded_t, .. } = e.kind {
            sum += unloaded_t;
        }
    }
    sum
}

#[test]
fn at_c_aircraft_are_repaired_and_hungry_men_make_less() {
    let run = Run::text(&text());
    // Night: repairs, group 2's ready and damaged.
    for (day, repaired, ready, damaged) in [
        (1, 1, 3, 19),
        (2, 4, 7, 15),
        (3, 4, 11, 11),
        (4, 4, 15, 7),
        (5, 4, 19, 3),
        (6, 2, 21, 1),
    ] {
        assert_eq!(
            run.repairs(day, 2),
            Some((repaired, ready, damaged)),
            "day {day}"
        );
        let g = &run.at(day).air_groups[&AirGroupId::new(2)];
        assert_eq!((g.ready, g.damaged), (ready, damaged), "day {day}");
    }
    // The base's stock after each night, men fed, men lost, unit 5's men
    // and the work of the day.
    for (day, stock, paid, fed, men_lost, men, work) in [
        (1, "10.5", Some("3"), 600, 0, 600, "600"),
        (2, "7.5", Some("3"), 600, 0, 600, "600"),
        (3, "4.5", Some("3"), 600, 0, 600, "600"),
        (4, "1.5", Some("3"), 600, 0, 600, "600"),
        (5, "0", Some("1.5"), 300, 15, 585, "292.5"),
        (6, "0", None, 0, 29, 556, "0"),
    ] {
        let spent = run.on(day, |k| {
            matches!(k, EventKind::BaseSupplySpent { base: b, reason: SupplyUse::Men, .. } if b.raw() == 3)
        });
        match paid {
            Some(t) => {
                assert_eq!(spent.len(), 1, "day {day}");
                assert!(matches!(
                    spent[0].kind,
                    EventKind::BaseSupplySpent { paid_t, left_t, .. }
                        if paid_t == fx(t) && left_t == fx(stock)
                ));
            }
            None => assert!(spent.is_empty(), "nothing paid on day {day}"),
        }
        assert_eq!(run.base_at(day, 3).supply_t, fx(stock), "day {day}");
        let fed_events = run.on(
            day,
            |k| matches!(k, EventKind::MenFed { hex, .. } if *hex == C),
        );
        assert_eq!(fed_events.len(), 1, "day {day}");
        assert!(matches!(
            fed_events[0].kind,
            EventKind::MenFed { fed: f, .. } if f == fed
        ));
        let lost = run.on(
            day,
            |k| matches!(k, EventKind::MenLostUnfed { unit, .. } if unit.raw() == 5),
        );
        if men_lost == 0 {
            assert!(lost.is_empty(), "day {day}");
        } else {
            assert_eq!(lost.len(), 1, "day {day}");
            assert!(matches!(
                lost[0].kind,
                EventKind::MenLostUnfed { lost: l, ashore, .. } if l == men_lost && ashore == men
            ));
        }
        assert_eq!(run.unit_at(day, 5).men_ashore, men, "day {day}");
        assert_eq!(run.work_made(day, 5), fx(work), "day {day}");
        if day >= 6 {
            assert!(run.worked(day, 5).is_empty());
        } else {
            assert_eq!(
                run.worked(day, 5),
                [(Job::Aircraft, 3, fx(work), fx(work))],
                "day {day}"
            );
        }
    }
    // Fed share: 1 on days 1 to 4, 0.5 on day 5, 0 on day 6, as the night
    // of the day left it.
    for (day, share) in [(1, "1"), (2, "1"), (3, "1"), (4, "1"), (5, "0.5"), (6, "0")] {
        assert_eq!(run.unit_at(day, 5).fed_share, fx(share), "day {day}");
    }
}

#[test]
fn at_d_defences_tell_in_a_fight_and_working_men_do_not_fire() {
    let run = Run::text(&text());
    // The night of day 1: base 4 pays 15 t, base 5 pays 5 t.
    assert_eq!(run.supply_after_night(1, 4), n(4985));
    assert_eq!(run.supply_after_night(1, 5), n(4995));
    // Day 1: unit 8 fires 10 shots at 0.3, unit 6 10 at 0.6 x 1.2.
    let (men8, shots8, chance8, hits8) = run.fire(1, 8);
    assert_eq!((men8, shots8, chance8), (1000, n(10), fx("0.3")));
    let (men6, shots6, chance6, hits6) = run.fire(1, 6);
    assert_eq!((men6, shots6, chance6), (1000, n(10), fx("0.72")));
    // Unit 7 fires nothing, on any day: a working unit fires nothing and
    // records no fire.
    for day in 1..=6 {
        assert!(
            run.fires(day).iter().all(|f| f.0 != 7),
            "unit 7 on day {day}"
        );
    }
    // The breakdown of unit 6's fire names the defences.
    let fire6 = run.on(
        1,
        |k| matches!(k, EventKind::GroundFire { unit, .. } if unit.raw() == 6),
    );
    assert_eq!(factor(fire6[0], "defences"), n(40));
    assert_eq!(factor(fire6[0], "defences_factor"), fx("1.2"));

    // The loss: 20 a hit, taken from units 6 and 7 in proportion to their
    // men, the men still to take one from each in unit order.
    for day in 1..=4u32 {
        let hits = run.fire(day, 8).3;
        let men6 = run.unit_at(day - 1, 6).men_ashore;
        let men7 = run.unit_at(day - 1, 7).men_ashore;
        let (lost6, lost7) = split_loss(20 * hits, men6, men7);
        let losses = run.fight_losses(day);
        let of = |raw: u32| {
            losses
                .iter()
                .filter(|l| l.0 == raw)
                .map(|l| l.1)
                .sum::<u32>()
        };
        assert_eq!(
            (of(6), of(7)),
            (lost6, lost7),
            "day {day}: {hits} hits on {men6} and {men7} men"
        );
        assert_eq!(of(8), 20 * run.fire(day, 6).3, "day {day}");
    }
    // Day 1: the Allied side is not beaten, whatever the dice give.
    assert!(
        run.all(|k| matches!(k, EventKind::SideBeatenAshore { side: Side::Allied, hex, .. } if *hex == D))
            .is_empty()
    );
    assert!(hits6 <= 10 && hits8 <= 10);
    assert!(run.unit_at(1, 6).men_ashore >= 933);

    // Unit 7's work: none on a contested day, men ashore times its fed
    // share on the others. Contested: while unit 8 is on attack.
    let mut worked_days = 0;
    for day in 1..=6u32 {
        let contested = run.on(
            day,
            |k| matches!(k, EventKind::GroundFight { hex, .. } if *hex == D),
        );
        let w = run.worked(day, 7);
        if contested.is_empty() {
            let u = run.unit_at(day, 7);
            let expected = Fixed::from_int(i64::from(u.men_ashore)) * u.fed_share;
            assert_eq!(w, [(Job::Port, 4, expected, expected)], "day {day}");
            worked_days += 1;
        } else {
            assert!(w.is_empty(), "contested day {day}");
        }
    }
    assert!(worked_days >= 1, "unit 7 works on some day");
    // Base 4's defences are 40 at the end, and every day.
    for day in 0..=6 {
        assert_eq!(run.base_at(day, 4).defences, n(40), "day {day}");
    }
}

/// Splits `loss` men between two units by the rule of decision 0066, 20:
/// unit i gives `loss` times its men over the total, cut to a whole man, and
/// the men still to take are taken one from each unit in unit order, round
/// again if need be.
fn split_loss(loss: u32, men_a: u32, men_b: u32) -> (u32, u32) {
    let total = men_a + men_b;
    let mut share = [loss * men_a / total, loss * men_b / total];
    let have = [men_a, men_b];
    let mut left = loss - share[0] - share[1];
    while left > 0 {
        for i in 0..2 {
            if left > 0 && share[i] < have[i] {
                share[i] += 1;
                left -= 1;
            }
        }
    }
    (share[0], share[1])
}

#[test]
fn at_e_working_men_do_not_hold_a_base() {
    let run = Run::text(&text());
    assert_eq!(run.supply_after_night(1, 6), fx("997.5"));
    assert_eq!(run.supply_after_night(1, 7), fx("99.5"));
    // Day 1: the Allied side is beaten without a fight.
    let beaten = run.all(|k| matches!(k, EventKind::SideBeatenAshore { hex, .. } if *hex == E));
    assert_eq!(beaten.len(), 1);
    assert_eq!(beaten[0].day, 1);
    assert_eq!(
        beaten[0].kind,
        EventKind::SideBeatenAshore {
            side: Side::Allied,
            hex: E,
            men: 0,
            enemy_men: 100,
            fought: false,
        }
    );
    assert!(run
        .all(|k| matches!(k, EventKind::GroundFire { hex, .. } if *hex == E))
        .is_empty());
    let changed = run.all(|k| matches!(k, EventKind::BaseChangedSide { .. }));
    assert_eq!(changed.len(), 1);
    assert_eq!(changed[0].day, 1);
    assert_eq!(
        changed[0].kind,
        EventKind::BaseChangedSide {
            base: base(6),
            hex: E,
            from: Side::Allied,
            to: Side::Japanese,
            supply_t: fx("498.75"),
            fuel_t: n(0),
            supply_lost_t: fx("498.75"),
            fuel_lost_t: n(0),
        }
    );
    for day in 1..=6 {
        let b = run.base_at(day, 6);
        assert_eq!(b.side, Side::Japanese, "day {day}");
        assert_eq!(
            (b.defences, b.airfield_work, b.airfield),
            (n(30), n(5000), false)
        );
    }
    // Unit 9 makes no work, ever; unit 10 is not tired.
    assert!(run
        .all(|k| matches!(k, EventKind::UnitWorked { unit, .. } if unit.raw() == 9))
        .is_empty());
    for day in 0..=6 {
        assert_eq!(run.unit_at(day, 10).fatigue, n(0), "day {day}");
    }
    // Unit 9's men.
    for (day, men) in [(1, 500), (2, 475), (3, 452), (4, 430), (5, 409), (6, 389)] {
        assert_eq!(run.unit_at(day, 9).men_ashore, men, "day {day}");
        assert_eq!(
            run.unit_at(day, 9).fed_share,
            if day == 1 { n(1) } else { n(0) },
            "day {day}"
        );
    }
    // Unit 10 is fed by base 6, 0.5 t a night, five times.
    assert_eq!(run.supply_after_night(2, 6), fx("498.25"));
    assert_eq!(run.base_at(6, 6).supply_t, fx("496.25"));
    assert_eq!(run.base_at(6, 7).supply_t, fx("99.5"));
    // The base at E has no airfield, so the Japanese side builds nothing.
    assert!(run
        .all(|k| matches!(k, EventKind::AirfieldBuilt { base, .. } if base.raw() == 6))
        .is_empty());
}

// ---------------------------------------------------------------- Properties

/// The count of days of each of the three a unit got on a day.
fn day_counts(run: &Run, day: u32, raw: u32) -> u32 {
    run.on(day, |k| match k {
        EventKind::UnitWorked { unit, .. }
        | EventKind::TrainingDay { unit, .. }
        | EventKind::RehearsalGained { unit, .. } => unit.raw() == raw,
        _ => false,
    })
    .len() as u32
}

fn assert_no_day_counted_twice(run: &Run, label: &str) {
    for day in 1..=run.len() {
        for raw in 1..=12 {
            assert!(
                day_counts(run, day, raw) <= 1,
                "{label}: unit {raw} is counted twice on day {day}"
            );
        }
    }
}

#[test]
fn property_no_units_day_is_counted_twice() {
    let base_text = text();
    let base_run = Run::text(&base_text);
    assert_no_day_counted_twice(&base_run, "the scenario");
    // Every unit that is counted at all is counted on a day: unit 12 has
    // one of work or rehearsal every day.
    for day in 1..=6 {
        assert_eq!(day_counts(&base_run, day, 12), 1, "unit 12, day {day}");
        assert_eq!(day_counts(&base_run, day, 3), 1, "unit 3, day {day}");
    }
    let variants: Vec<(&str, String)> = vec![
        (
            "unit 12 trains from day 1",
            with_orders(&base_text, &activity_order(1, 12, "train", None)),
        ),
        (
            "unit 4 works on day 1 and trains again on day 2",
            with_orders(
                &base_text,
                &(activity_order(1, 4, "work", Some("airfield"))
                    + &activity_order(2, 4, "train", None)),
            ),
        ),
        (
            "unit 12 works on day 1, rests on day 2 and trains on day 4",
            with_orders(
                &base_text,
                &(activity_order(1, 12, "work", Some("aircraft"))
                    + &activity_order(2, 12, "ready", None)
                    + &activity_order(4, 12, "train", None)),
            ),
        ),
        (
            "unit 12 works on every day",
            with_orders(
                &base_text,
                &(activity_order(1, 12, "work", Some("port"))
                    + &activity_order(5, 12, "work", Some("defences"))),
            ),
        ),
        (
            "unit 11 trains, unit 6 works",
            with_orders(
                &base_text,
                &(activity_order(1, 11, "train", None) + &activity_order(1, 6, "work", None)),
            ),
        ),
    ];
    for (label, source) in variants {
        let run = Run::text(&source);
        assert_no_day_counted_twice(&run, label);
        assert_ne!(log_text(&run), log_text(&base_run), "{label} differs");
    }
}

#[test]
fn property_work_never_falls_as_men_fed_share_or_skill_rise_and_a_port_never_more_than_twice() {
    let run = Run::text(&text());
    let g = figures(&run);
    let men_grid = [0u32, 1, 99, 100, 101, 400, 1000, 1001, 2000, 3000];
    let shares: Vec<Fixed> = (0..=8).map(|i| Fixed::from_ratio(i, 8)).collect();
    let skills = [0u8, 1, 2, 3, 4];
    for working in [true, false] {
        for (i, &m) in men_grid.iter().enumerate() {
            for (j, &s) in shares.iter().enumerate() {
                for (k, &w) in skills.iter().enumerate() {
                    let here = unit_work(m, s, w, working, &g);
                    assert!(here >= Fixed::ZERO);
                    if let Some(&m2) = men_grid.get(i + 1) {
                        assert!(unit_work(m2, s, w, working, &g) >= here, "men {m} to {m2}");
                    }
                    if let Some(&s2) = shares.get(j + 1) {
                        assert!(
                            unit_work(m, s2, w, working, &g) >= here,
                            "share {s} to {s2}"
                        );
                    }
                    if let Some(&w2) = skills.get(k + 1) {
                        assert!(
                            unit_work(m, s, w2, working, &g) >= here,
                            "skill {w} to {w2}"
                        );
                    }
                    // A fighting unit makes less than a working one.
                    assert!(unit_work(m, s, w, false, &g) <= unit_work(m, s, w, true, &g));
                }
            }
        }
    }
    // The port factor never falls with work and is never above 2.
    let mut last = Fixed::ZERO;
    for work in (0..=6000).step_by(50) {
        let f = port_factor(n(work), &g);
        assert!(
            f >= last && f >= Fixed::ONE && f <= n(2),
            "work {work}: {f}"
        );
        last = f;
    }
    assert_eq!(port_factor(n(10_000_000), &g), n(2));
    // Every rate of the run is at most twice the port's own 20 t an hour.
    let unloaded = run.all(|k| matches!(k, EventKind::CargoUnloaded { .. }));
    assert!(!unloaded.is_empty());
    for e in unloaded {
        let rate = factor(e, "rate_t_per_h");
        assert!(
            rate > Fixed::ZERO && rate <= n(40),
            "rate {rate} on day {}",
            e.day
        );
    }
}

/// The text of the event kinds of a log, one a line, with no number.
fn kinds_text(run: &Run) -> Vec<String> {
    run.events.iter().map(|e| format!("{}", e.kind)).collect()
}

#[test]
fn property_a_scenario_with_no_working_unit_gives_what_it_gave_before() {
    // Scenario 29 has no working unit, no activity order, bases with an
    // airfield and no defences. Its event log, read as text, is the
    // recorded baseline of before decision 0099, line for line, with the
    // hash lines left out: the strongest cheap form of "what it gave
    // before". The baseline is what `sim-cli run --events` wrote.
    let path29 = common::data_dir().join("scenarios/29_skills.toml");
    let scenario = common::load_live(&path29).expect("scenario 29 loads on the live figures");
    let run = Run::of(scenario);
    assert!(run
        .all(|k| matches!(
            k,
            EventKind::UnitWorked { .. } | EventKind::AirfieldBuilt { .. }
        ))
        .is_empty());
    let baseline = std::fs::read_to_string(
        common::data_dir().join("../crates/sim-cli/tests/run_baseline/29_skills.events.txt"),
    )
    .expect("the committed baseline reads");
    let lines: Vec<&str> = baseline.lines().filter(|l| l.starts_with('#')).collect();
    let kinds = kinds_text(&run);
    assert_eq!(lines.len(), kinds.len(), "the same number of events");
    for (i, (line, kind)) in lines.iter().zip(&kinds).enumerate() {
        // The line is "#n D<day> <pulse> <text>" with the causes after "  <- ".
        let body = line.split("  <- ").next().expect("a line");
        assert!(
            body.ends_with(kind.as_str()),
            "event {}: {kind:?} is not the end of {body:?}",
            i + 1
        );
    }
}

#[test]
fn property_the_enemys_view_is_the_same_when_only_the_other_sides_works_differ_and_the_hits_are_the_same(
) {
    let base_text = text();
    let base_run = Run::text(&base_text);
    // The Allied side's jobs, work and defences away from the fights at D
    // and E change nothing the Japanese side can see.
    let variants: Vec<(&str, String)> = vec![
        (
            "unit 11 does not dig",
            with_orders(&base_text, &activity_order(1, 11, "ready", None)),
        ),
        (
            "unit 3 digs, not the port",
            with_orders(&base_text, &activity_order(1, 3, "work", Some("defences"))),
        ),
        (
            "unit 5 works the port, not the aircraft",
            with_orders(&base_text, &activity_order(1, 5, "work", Some("port"))),
        ),
        (
            "unit 4 works the aircraft from day 1",
            with_orders(&base_text, &activity_order(1, 4, "work", Some("aircraft"))),
        ),
        (
            "the airfield at A is built on day 1",
            common::replace_once(&base_text, "airfield_work = 17000", "airfield_work = 19500"),
        ),
        (
            "base 1 starts with defences",
            common::replace_once(
                &base_text,
                "airfield_work = 17000\ndefences = 0",
                "airfield_work = 17000\ndefences = 60",
            ),
        ),
    ];
    let mut compared = 0;
    for (label, source) in &variants {
        let other = Run::text(source);
        assert_ne!(
            log_text(&other),
            log_text(&base_run),
            "{label}: the worlds differ in truth"
        );
        for day in 1..=6 {
            assert_eq!(
                base_run.fires(day),
                other.fires(day),
                "{label}: the hits are the same"
            );
            assert_eq!(
                day_digest(&base_run.scenario, Side::Japanese, day),
                day_digest(&other.scenario, Side::Japanese, day),
                "{label}, day {day}"
            );
        }
        compared += 1;
    }
    assert_eq!(compared, variants.len());

    // Defences at D, which the fight reads: compare where the hits are the
    // same, across seeds.
    let mut same_hits = 0;
    for seed in 1..=60u32 {
        let seeded = common::replace_once(&base_text, "seed = 1942", &format!("seed = {seed}"));
        let with = Run::text(&seeded);
        let without = Run::text(&common::replace_once(
            &seeded,
            "defences = 40",
            "defences = 0",
        ));
        if hits_of(&with, 1) != hits_of(&without, 1) {
            continue;
        }
        same_hits += 1;
        assert_ne!(
            with.base_at(1, 4).defences,
            without.base_at(1, 4).defences,
            "the worlds differ in truth"
        );
        assert_eq!(
            day_digest_with(&with.scenario, Side::Japanese, 1, false),
            day_digest_with(&without.scenario, Side::Japanese, 1, false),
            "seed {seed}: the defences of base 4, hits the same"
        );
    }
    assert!(same_hits >= 1, "worlds with the same hits: {same_hits}");
}

/// Each unit's hits on a day, in log order.
fn hits_of(run: &Run, day: u32) -> Vec<(u32, u32)> {
    run.fires(day).iter().map(|f| (f.0, f.4)).collect()
}
