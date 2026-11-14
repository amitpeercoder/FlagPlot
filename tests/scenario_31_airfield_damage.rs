//! Scenario 31: an airfield damaged and mended. A damaged field flies a
//! share of its aircraft; working units mend it and nothing mends itself; a
//! badly damaged field takes no aircraft, and an arrival due at it waits;
//! bombs can land on an airfield and cannot land on one that is not there.
//!
//! Proves decision 0103 (an airfield damaged and mended) against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/31_airfield_damage.toml`. The frozen step 1 figures set
//! `[base_hit] airfield` to 0, so this test gives it the value below, as
//! the test of ships' experience sets its figure. The three ground figures
//! are in the frozen figures at the values below.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide (the
//! hits of a raid and where each lands), the calculation gives the rule
//! exactly and the tests read the hits from the run. Everything else is
//! asserted exactly. If the code disagrees, the code or the rule is wrong:
//! do not change a number here to make a test pass. A mismatch goes back to
//! the architect.
//!
//! ## Figures
//!
//! `[base_hit] airfield` 0.2, `airfield_damage_per_hit` 2,
//! `work_per_airfield_damage_point` 50, `airfield_landing_refused_at` 50.
//! From decision 0099: `work_per_man_day` 1, `fighting_unit_work_share`
//! 0.5. From the earlier rules: `daily_supply_t_per_man` 0.005.
//!
//! The scenario names no wear, fatigue or air supply figures: nothing
//! wears, tires or costs supply but the men.
//!
//! Of a group at a base, the aircraft that may fly in a pulse are its ready
//! aircraft times 100 less the field's damage, over 100, cut to a whole
//! aircraft. The damage is read at the start of the pulse. Work is made in
//! the ground combat phase, at the end of the day, so a day's mending shows
//! in the next day's flying.
//!
//! ## Inputs
//!
//! A placeholder map with four coast hexes: A at (0,0), B at (5,0) and C
//! at (10,0) on one axis, and D at (5,4), four hexes from B. The game is 6
//! days.
//!
//! | Base | Side     | Hex | Supply | Airfield | Damage at the start |
//! | ---- | -------- | --- | ------ | -------- | ------------------- |
//! | 1    | Allied   | A   | 5000 t | yes      | 30                  |
//! | 2    | Allied   | B   | 5000 t | yes      | 60                  |
//! | 3    | Japanese | C   | 5000 t | yes      | 0                   |
//! | 4    | Japanese | D   | 5000 t | none     | none                |
//!
//! | Unit | Side   | Kind     | Men  | Hex |
//! | ---- | ------ | -------- | ---- | --- |
//! | 1    | Allied | working  | 500  | A   |
//! | 2    | Allied | fighting | 1000 | B   |
//!
//! Air groups, all Allied:
//!
//! - Group 1: placeholder flying boat, at base 1, 12 ready, on search.
//! - Group 2: placeholder fighter (transfer range 500 nm), at base 1, 12
//!   ready, no mission. Base 2 is 200 nm away.
//! - Group 3: placeholder fighter, at base 2, 4 ready, no mission.
//! - Group 4: the placeholder bomber (type 3, strike radius 250 nm), at
//!   base 2, on a strike at base 3, which is within its strike radius.
//! - Group 5: the same bomber, at base 2, on a strike at base 4, which is
//!   within its strike radius.
//!
//! An arrival: 6 aircraft and 6 pilots for group 3, due on day 2.
//!
//! Orders:
//!
//! - Day 1: rebase group 2 to base 2. Unit 1 is given no order.
//! - Day 2: unit 2 works at the airfield.
//! - Day 3: rebase group 2 to base 2.
//! - Day 4: rebase group 2 to base 2.
//!
//! ## At A: a share flies, and a working unit mends
//!
//! Unit 1 has no order. The field of base 1 is built and damaged, so its
//! job is the airfield: 500 of work a day, 500 / 50 = 10 points mended.
//!
//! | Day | Damage through the day | Group 1 may fly | Damage after the day's work | Unit 1's job that day |
//! | --- | ---------------------- | --------------- | --------------------------- | --------------------- |
//! | 1   | 30                     | 12 x 70/100 = 8.4: 8 | 20                     | airfield              |
//! | 2   | 20                     | 9.6: 9          | 10                          | airfield              |
//! | 3   | 10                     | 10.8: 10        | 0                           | airfield              |
//! | 4   | 0                      | 12              | 0                           | port                  |
//! | 5   | 0                      | 12              | 0                           | port                  |
//! | 6   | 0                      | 12              | 0                           | port                  |
//!
//! From day 4 the field is whole and unit 1, still with no order, works the
//! port. Group 2 is at base 1 until day 4 and may fly 8, 9 and 10 of its 12
//! on days 1 to 3.
//!
//! Base 1 feeds 500 men, 2.5 t a night, six times: 4985 t at the end.
//!
//! ## At B: nothing mends itself, a level that refuses, an arrival that waits
//!
//! Unit 2 is a fighting unit. From day 2 it works at the airfield: 1000 x
//! 0.5 = 500 of work a day, 10 points mended. On day 1 nobody works on the
//! field and its damage stays at 60.
//!
//! | Day | Damage through the day | Night: the arrival | Group 3, ready | Group 3 may fly | Damage after the day's work |
//! | --- | ---------------------- | ------------------ | -------------- | --------------- | --------------------------- |
//! | 1   | 60                     | not due            | 4              | 4 x 40/100 = 1.6: 1 | 60                      |
//! | 2   | 60                     | due, waits: 60 is at or above 50 | 4 | 1               | 50                          |
//! | 3   | 50                     | waits: 50 is at or above 50 | 4     | 4 x 50/100 = 2  | 40                          |
//! | 4   | 40                     | delivered: 6 aircraft | 10          | 10 x 60/100 = 6 | 30                          |
//! | 5   | 30                     |                    | 10             | 7               | 20                          |
//! | 6   | 20                     |                    | 10             | 8               | 10                          |
//!
//! The rebase of group 2 to base 2:
//!
//! - Day 1, at intake: the damage is 60. Refused, as the field takes no
//!   aircraft.
//! - Day 3, at intake: the damage is 50. Refused: at the level is refused.
//! - Day 4, at intake: the damage is 40. Accepted. Group 2 is at base 2
//!   with 12 ready and flies nothing that day (decision 0091). On days 5
//!   and 6 it may fly 12 x 70/100 = 8.4: 8, and 12 x 80/100 = 9.6: 9.
//!
//! The bombers at base 2 are held by the same share. Of each group's ready
//! aircraft r at the start of a daylight pulse, r times 100 less the damage,
//! over 100, cut to a whole aircraft, fly the strike. The tests read r from
//! the run and assert the aircraft that flew.
//!
//! Base 2 feeds 1000 men, 5 t a night, six times: 4970 t at the end.
//!
//! ## At C and D: where bombs land
//!
//! Each hit of a raid on base 3 makes one draw for where it lands. With the
//! figure at 0.2 the airfield's share comes after those of aircraft, supply
//! and men in the table, and a hit on the airfield adds 2 to the damage of
//! base 3, up to 100. The tests read each raid's hits on the airfield from
//! the run and assert that base 3's damage after the raid is the damage
//! before it plus twice those hits. Nobody works on base 3's field: its
//! damage never falls.
//!
//! Base 4 has no airfield. The share counts for nothing there: no hit of
//! any raid on base 4 lands on an airfield, base 4 has no damage at the
//! end, and the raid's breakdown does not name the share.
//!
//! # Properties
//!
//! - Damage is never below 0 or above 100, and with no unit working on a
//!   field it never falls.
//! - The aircraft that may fly never rise when damage rises, and are never
//!   more than the ready aircraft.
//! - An arrival that waits is delivered once and whole, and none is lost.
//! - With the share at 0 and no damage stated, every scenario gives what it
//!   gave before, apart from the hash version.
//! - The enemy's view is the same in two worlds that differ only in the
//!   other side's airfield damage and the work done to mend it, where the
//!   hits are the same; and a raid's claim says nothing of the field.

mod common;

use std::path::PathBuf;

use sim_core::choices::air::{AirAnswer, AirQuestion};
use sim_core::choices::{Answer, Moment, PulsePhase, Question};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, EntityRef, LandUnitId};
use sim_core::orders::{Job, RejectReason};
use sim_core::state::{AirGroupLocation, Side, TruthState};
use sim_core::time::Pulse;
use sim_core::World;
use sim_data::Scenario;

// ------------------------------------------------------------------ Helpers

const A: Hex = Hex::new(0, 0);
const B: Hex = Hex::new(5, 0);
const C: Hex = Hex::new(10, 0);
const D: Hex = Hex::new(5, 4);

/// The days of the game.
const DAYS: u32 = 6;

fn path() -> PathBuf {
    common::data_dir().join("scenarios/31_airfield_damage.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

fn group(raw: u32) -> AirGroupId {
    AirGroupId::new(raw)
}

fn unit(raw: u32) -> LandUnitId {
    LandUnitId::new(raw)
}

/// The scenario on the frozen step 1 figures with `[base_hit] airfield` set
/// to `share`, as the doc comment says.
fn load(source: &str, share: &str) -> Scenario {
    let options = common::altered_reference("scenario31", |dir| {
        common::edit_file_by(
            dir,
            "air_combat.toml",
            &[("airfield = \"0\"", &format!("airfield = \"{share}\""))],
        );
    });
    common::from_text_with(source, &path(), &options).expect("the variant loads")
}

/// The scenario text with the damage of base 1 at the start taken out.
fn without_base_1_damage(source: &str) -> String {
    common::replace_once(source, "airfield_damage = 30\n", "")
}

/// The scenario text with no damage stated on either field.
fn without_damage(source: &str) -> String {
    common::replace_once(&without_base_1_damage(source), "airfield_damage = 60\n", "")
}

/// One land unit order of the Allied side as scenario text.
fn work_order(day: u32, unit: u32, job: &str) -> String {
    format!(
        "\n[[order]]\nday = {day}\nside = \"allied\"\nland_unit = {unit}\nkind = \"set_activity\"\nactivity = \"work\"\njob = \"{job}\"\n"
    )
}

/// What a side's view says of the aircraft that may fly, asked at a moment.
#[derive(Clone, Debug)]
struct Seen {
    day: u32,
    pulse: Pulse,
    /// Which of the moments of the pulse: "begins", "search", "strikes".
    moment: &'static str,
    group: u32,
    able: u32,
    ready: u32,
    /// The damage of the airfield of the group's base, 0 aboard no base.
    damage: Fixed,
}

/// A run of the scenario: the truth at the start (day 0) and at the end of
/// each day, the log, the questions asked in each daylight pulse, and the
/// text of each side's view at the end of each day.
struct Run {
    scenario: Scenario,
    truths: Vec<TruthState>,
    events: Vec<Event>,
    seen: Vec<Seen>,
    japanese: Vec<String>,
    allied: Vec<String>,
}

/// The text of a side's view of the events of `day`, its contacts, its
/// claims, its enemy bases, and each question it can ask with its answer.
fn digest(world: &World, side: Side, day: u32) -> String {
    let view = world.side_view(side);
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
    for c in view.contacts() {
        out += &format!("{c:?}\n");
    }
    for c in view.claims() {
        out += &format!("{}\n", c.text(side));
    }
    for b in view.enemy_bases() {
        out += &format!("{} {:?} {:?} {:?}\n", b.id.raw(), b.name, b.hex, b.side);
    }
    for q in view.questions() {
        out += &format!("{q:?} -> {:?}\n", view.ask(&q));
    }
    out
}

impl Run {
    fn of(source: &str, share: &str, last_day: u32) -> Run {
        let mut scenario = load(source, share);
        let script = scenario.scripted_orders().clone();
        let mut truths = vec![scenario.world.umpire().clone()];
        let mut seen: Vec<Seen> = Vec::new();
        let mut japanese = vec![String::new()];
        let mut allied = vec![String::new()];
        for day in 1..=last_day.min(scenario.days) {
            for (side, request) in script.get(&day).cloned().unwrap_or_default() {
                scenario.world.submit_orders(side, vec![request]);
            }
            scenario
                .world
                .resolve_day_watched(&mut |world, moment| {
                    let (day, pulse, label) = match moment {
                        Moment::PulseBegins { day, pulse } => (day, pulse, "begins"),
                        Moment::Phase {
                            day,
                            pulse,
                            phase: PulsePhase::AirSearch,
                        } => (day, pulse, "search"),
                        Moment::Phase {
                            day,
                            pulse,
                            phase: PulsePhase::StrikeDecisions,
                        } => (day, pulse, "strikes"),
                        _ => return,
                    };
                    if pulse == Pulse::Night {
                        return;
                    }
                    for g in 1..=5 {
                        let answer = world.side_view(Side::Allied).ask(&Question::Air(
                            AirQuestion::AircraftAbleToFly { group: group(g) },
                        ));
                        let Some(Answer::Air(AirAnswer::AircraftAbleToFly(able))) = answer else {
                            panic!("group {g} answers how many may fly, got {answer:?}");
                        };
                        let truth = world.umpire();
                        let at = &truth.air_groups[&group(g)];
                        let damage = match at.location {
                            AirGroupLocation::Base(b) => truth.bases[&b].airfield_damage,
                            _ => Fixed::ZERO,
                        };
                        seen.push(Seen {
                            day,
                            pulse,
                            moment: label,
                            group: g,
                            able,
                            ready: at.ready,
                            damage,
                        });
                    }
                })
                .expect("the day resolves");
            truths.push(scenario.world.umpire().clone());
            japanese.push(digest(&scenario.world, Side::Japanese, day));
            allied.push(digest(&scenario.world, Side::Allied, day));
        }
        let events = scenario.world.events().iter().cloned().collect();
        Run {
            scenario,
            truths,
            events,
            seen,
            japanese,
            allied,
        }
    }

    /// The scenario as it is, on the share of the doc comment.
    fn base() -> Run {
        Run::of(&text(), "0.2", DAYS)
    }

    fn at(&self, day: u32) -> &TruthState {
        &self.truths[day as usize]
    }

    /// A base's damage at the end of `day` (0: the start).
    fn damage(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).bases[&base(raw)].airfield_damage
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

    /// The aircraft of `group` that may fly in `pulse` of `day`: the answers
    /// of the three moments of the pulse must be one.
    fn able(&self, day: u32, pulse: Pulse, g: u32) -> u32 {
        let found: Vec<&Seen> = self
            .seen
            .iter()
            .filter(|s| s.day == day && s.pulse == pulse && s.group == g)
            .collect();
        assert_eq!(
            found.len(),
            3,
            "three moments, day {day} {pulse:?} group {g}"
        );
        assert!(
            found.iter().all(|s| s.able == found[0].able),
            "one answer at the three moments, day {day} {pulse:?} group {g}: {found:?}"
        );
        found[0].able
    }

    /// The aircraft of `group` that may fly on `day`: the same in both
    /// daylight pulses.
    fn able_on(&self, day: u32, g: u32) -> u32 {
        let morning = self.able(day, Pulse::Morning, g);
        let afternoon = self.able(day, Pulse::Afternoon, g);
        if g <= 3 {
            // Nothing a group of these three does changes what may fly.
            assert_eq!(morning, afternoon, "day {day} group {g}");
        }
        morning
    }

    /// The ready aircraft of `group` at the start of the morning pulse of
    /// `day`.
    fn ready_at_morning(&self, day: u32, g: u32) -> u32 {
        self.seen
            .iter()
            .find(|s| s.day == day && s.pulse == Pulse::Morning && s.group == g)
            .expect("asked")
            .ready
    }

    /// The `UnitWorked` events of a unit on a day: `(job, base, work)`.
    fn worked(&self, day: u32, raw: u32) -> Vec<(Job, u32, Fixed)> {
        self.on(
            day,
            |k| matches!(k, EventKind::UnitWorked { unit, .. } if unit.raw() == raw),
        )
        .into_iter()
        .map(|e| match e.kind {
            EventKind::UnitWorked {
                job, base, work, ..
            } => (job, base.raw(), work),
            _ => unreachable!(),
        })
        .collect()
    }

    /// The `AirfieldMended` events of a day: `(unit, base, points, damage)`.
    fn mended(&self, day: u32) -> Vec<(u32, u32, Fixed, Fixed)> {
        self.on(day, |k| matches!(k, EventKind::AirfieldMended { .. }))
            .into_iter()
            .map(|e| match e.kind {
                EventKind::AirfieldMended {
                    unit,
                    base,
                    points,
                    damage,
                } => (unit.raw(), base.raw(), points, damage),
                _ => unreachable!(),
            })
            .collect()
    }

    /// The `AirfieldDamaged` events of a day: `(base, hits, damage)`.
    fn damaged(&self, day: u32) -> Vec<(u32, u32, Fixed)> {
        self.on(day, |k| matches!(k, EventKind::AirfieldDamaged { .. }))
            .into_iter()
            .map(|e| match e.kind {
                EventKind::AirfieldDamaged {
                    base, hits, damage, ..
                } => (base.raw(), hits, damage),
                _ => unreachable!(),
            })
            .collect()
    }

    /// The bombers' launch on `day`: `(pulse, aircraft)` of group `g`.
    fn launched(&self, day: u32, g: u32) -> Vec<(Option<Pulse>, u32)> {
        self.on(
            day,
            |k| matches!(k, EventKind::BaseStrikeLaunched { air_group, .. } if air_group.raw() == g),
        )
        .into_iter()
        .map(|e| match e.kind {
            EventKind::BaseStrikeLaunched { aircraft, .. } => (e.pulse, aircraft),
            _ => unreachable!(),
        })
        .collect()
    }

    /// The raid of group `g` on `day`: its `BaseAttacked` event.
    fn attack(&self, day: u32, g: u32) -> &Event {
        let found = self.on(
            day,
            |k| matches!(k, EventKind::BaseAttacked { air_group, .. } if air_group.raw() == g),
        );
        assert_eq!(found.len(), 1, "group {g} attacked once on day {day}");
        found[0]
    }

    /// `(hits, on aircraft, on supply, on men)` of a raid.
    fn hits(&self, day: u32, g: u32) -> (u32, u32, u32, u32) {
        match self.attack(day, g).kind {
            EventKind::BaseAttacked {
                hits,
                hits_on_aircraft,
                hits_on_supply,
                hits_on_men,
                ..
            } => (hits, hits_on_aircraft, hits_on_supply, hits_on_men),
            _ => unreachable!(),
        }
    }

    /// The hits of the raids of `day` that landed on the airfield of base
    /// `raw`, from the `AirfieldDamaged` events.
    fn airfield_hits(&self, day: u32, raw: u32) -> u32 {
        self.damaged(day)
            .into_iter()
            .filter(|d| d.0 == raw)
            .map(|d| d.1)
            .sum()
    }

    /// The aircraft group 1's search flew on `day`, by pulse.
    fn searches(&self, day: u32) -> Vec<(Option<Pulse>, u32)> {
        self.on(
            day,
            |k| matches!(k, EventKind::SearchFlown { air_group, .. } if air_group.raw() == 1),
        )
        .into_iter()
        .map(|e| match e.kind {
            EventKind::SearchFlown { aircraft, .. } => (e.pulse, aircraft),
            _ => unreachable!(),
        })
        .collect()
    }

    /// The answers to the Allied orders of `day` for air group `g`:
    /// `Ok(())` for accepted, `Err(reason)` for rejected.
    fn rebase_answer(&self, day: u32, g: u32) -> Result<(), RejectReason> {
        let mut answers = Vec::new();
        for e in self.on(day, |_| true) {
            match &e.kind {
                EventKind::OrderAccepted {
                    target: EntityRef::AirGroup(a),
                    ..
                } if a.raw() == g => answers.push(Ok(())),
                EventKind::OrderRejected {
                    target: EntityRef::AirGroup(a),
                    reason,
                    ..
                } if a.raw() == g => answers.push(Err(reason.clone())),
                _ => {}
            }
        }
        assert_eq!(answers.len(), 1, "one answer for group {g} on day {day}");
        answers.remove(0)
    }

    /// `(day, damage)` of each `ArrivalWaiting` event.
    fn waiting(&self) -> Vec<(u32, Option<Pulse>, Fixed)> {
        self.all(|k| matches!(k, EventKind::ArrivalWaiting { .. }))
            .into_iter()
            .map(|e| match e.kind {
                EventKind::ArrivalWaiting { damage, .. } => (e.day, e.pulse, damage),
                _ => unreachable!(),
            })
            .collect()
    }

    /// `(day, pulse, aircraft, pilots, ready, pilots_now, strength)` of each
    /// arrival delivered.
    fn arrived(&self) -> Vec<Arrived> {
        self.all(|k| matches!(k, EventKind::AircraftArrived { .. }))
            .into_iter()
            .map(|e| match e.kind {
                EventKind::AircraftArrived {
                    aircraft,
                    pilots,
                    ready,
                    pilots_now,
                    strength,
                    ..
                } => (
                    e.day, e.pulse, aircraft, pilots, ready, pilots_now, strength,
                ),
                _ => unreachable!(),
            })
            .collect()
    }
}

/// `(day, pulse, aircraft, pilots, ready, pilots_now, strength)` of an arrival.
type Arrived = (u32, Option<Pulse>, u32, u32, u32, u32, u32);

fn has_factor(event: &Event, name: &str) -> bool {
    event
        .breakdown
        .as_ref()
        .expect("a breakdown")
        .factors
        .iter()
        .any(|f| f.name == name)
}

/// Whole aircraft of `ready` times 100 less `damage` over 100, cut.
fn share(ready: u32, damage: i64) -> u32 {
    u32::try_from(i64::from(ready) * (100 - damage) / 100).expect("a count")
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_figures_and_the_inputs_are_the_calculations() {
    let scenario = load(&text(), "0.2");
    let figures = scenario.world.ref_data();
    let g = figures.ground.expect("ground figures");
    assert_eq!(g.airfield_damage_per_hit, n(2));
    assert_eq!(g.work_per_airfield_damage_point, n(50));
    assert_eq!(g.airfield_landing_refused_at, n(50));
    assert_eq!(g.work_per_man_day, n(1));
    assert_eq!(g.fighting_unit_work_share, fx("0.5"));
    assert_eq!(g.daily_supply_t_per_man, fx("0.005"));
    assert_eq!(
        figures
            .air_combat
            .as_ref()
            .expect("air combat")
            .base_hit_airfield,
        fx("0.2")
    );
    // No wear, fatigue or air supply figures.
    assert!(figures.fatigue.is_none());
    assert!(figures.air_supply.is_none());
    assert!(figures.air_upkeep.is_none());

    let truth = scenario.world.umpire();
    let map = scenario.world.map();
    for hex in [A, B, C, D] {
        assert!(map.get(hex).is_some(), "{hex:?} is on the map");
    }
    assert_eq!(A.air_distance_nm(B), 200);
    assert_eq!(B.air_distance_nm(C), 200);
    assert_eq!(B.air_distance_nm(D), 160);
    assert_eq!(scenario.days, DAYS);
    // Bases: id, side, hex, supply, airfield, damage.
    assert_eq!(truth.bases.len(), 4);
    for (id, side, hex, supply, airfield, damage) in [
        (1, Side::Allied, A, n(5000), true, n(30)),
        (2, Side::Allied, B, n(5000), true, n(60)),
        (3, Side::Japanese, C, n(5000), true, n(0)),
        (4, Side::Japanese, D, n(5000), false, n(0)),
    ] {
        let b = &truth.bases[&base(id)];
        assert_eq!(
            (b.side, b.hex, b.supply_t, b.airfield, b.airfield_damage),
            (side, hex, supply, airfield, damage),
            "base {id}"
        );
    }
    // Units: id, working, men, hex, works skill.
    assert_eq!(truth.land_units.len(), 2);
    for (id, working, men, hex) in [(1, true, 500, A), (2, false, 1000, B)] {
        let u = &truth.land_units[&unit(id)];
        assert_eq!(
            (u.side, u.working, u.men_ashore, u.hex, u.skills.works),
            (Side::Allied, working, men, Some(hex), 2),
            "unit {id}"
        );
    }
    // Air groups: id, base, ready, pilots, aircraft type.
    assert_eq!(truth.air_groups.len(), 5);
    for (id, at, ready, pilots, kind) in [
        (1, 1, 12, None, 1),
        (2, 1, 12, None, 4),
        (3, 2, 4, Some(4), 4),
        (4, 2, 20, None, 3),
        (5, 2, 20, None, 3),
    ] {
        let a = &truth.air_groups[&group(id)];
        assert_eq!(
            (
                a.location,
                a.ready,
                a.damaged,
                a.pilots,
                a.aircraft_type.raw()
            ),
            (AirGroupLocation::Base(base(at)), ready, 0, pilots, kind),
            "group {id}"
        );
        assert_eq!(a.side, Side::Allied);
    }
    // The bombers reach C and D from B.
    let radius = figures.reach_nm(
        truth.air_groups[&group(4)].aircraft_type,
        sim_core::orders::Reach::Normal,
    );
    assert!(radius >= n(200));
    // The arrival.
    assert_eq!(truth.arrivals.len(), 1);
    let arrival = &truth.arrivals[0];
    assert_eq!(
        (
            arrival.day,
            arrival.air_group,
            arrival.aircraft,
            arrival.pilots,
            arrival.waiting
        ),
        (2, group(3), 6, 6, false)
    );
    // Orders by day: 5 on day 1, 1 on each of days 2 to 4.
    let counts: Vec<(u32, usize)> = scenario
        .scripted_orders()
        .iter()
        .map(|(d, o)| (*d, o.len()))
        .collect();
    assert_eq!(counts, vec![(1, 5), (2, 1), (3, 1), (4, 1)]);
}

#[test]
fn at_a_damage_goes_30_20_10_0_and_group_1_may_fly_8_9_10_12_12_12() {
    let run = Run::base();
    // Damage through the day, which is that of the end of the day before.
    for (day, through, after, may_fly) in [
        (1, 30, 20, 8),
        (2, 20, 10, 9),
        (3, 10, 0, 10),
        (4, 0, 0, 12),
        (5, 0, 0, 12),
        (6, 0, 0, 12),
    ] {
        assert_eq!(run.damage(day - 1, 1), n(through), "day {day} through");
        assert_eq!(run.damage(day, 1), n(after), "day {day} after");
        assert_eq!(run.able_on(day, 1), may_fly, "group 1 on day {day}");
        // The share of the doc comment: 12 x (100 - damage) / 100, cut.
        assert_eq!(may_fly, share(12, through), "day {day}");
        // And the engine flew that many: the search of both daylight pulses.
        assert_eq!(
            run.searches(day),
            vec![
                (Some(Pulse::Morning), may_fly),
                (Some(Pulse::Afternoon), may_fly)
            ],
            "group 1's search on day {day}"
        );
    }
}

#[test]
fn unit_1_mends_the_airfield_10_points_a_day_for_3_days_then_works_the_port() {
    let run = Run::base();
    for day in 1..=3 {
        assert_eq!(
            run.worked(day, 1),
            vec![(Job::Airfield, 1, n(500))],
            "day {day}"
        );
    }
    for day in 4..=6 {
        assert_eq!(
            run.worked(day, 1),
            vec![(Job::Port, 1, n(500))],
            "day {day}"
        );
    }
    // Mended 10, 10, 10 and then nothing: damage 20, 10, 0.
    for (day, damage) in [(1, 20), (2, 10), (3, 0)] {
        let unit_1: Vec<_> = run.mended(day).into_iter().filter(|m| m.0 == 1).collect();
        assert_eq!(unit_1, vec![(1, 1, n(10), n(damage))], "day {day}");
    }
    for day in 4..=6 {
        assert!(run.mended(day).iter().all(|m| m.0 != 1), "day {day}");
    }
}

#[test]
fn group_2_is_refused_on_days_1_and_3_and_accepted_on_day_4() {
    let run = Run::base();
    assert_eq!(
        run.rebase_answer(1, 2),
        Err(RejectReason::AirfieldTooDamaged {
            damage: n(60),
            refused_at: n(50)
        })
    );
    assert_eq!(
        run.rebase_answer(3, 2),
        Err(RejectReason::AirfieldTooDamaged {
            damage: n(50),
            refused_at: n(50)
        })
    );
    assert_eq!(run.rebase_answer(4, 2), Ok(()));
    assert_eq!(
        RejectReason::AirfieldTooDamaged {
            damage: n(60),
            refused_at: n(50)
        }
        .to_string(),
        "the airfield's damage is 60, and a field at 50 or more takes no aircraft"
    );
    // Refused: group 2 stays at base 1 through day 3 with its 12 ready.
    for day in 1..=3 {
        let g = &run.at(day).air_groups[&group(2)];
        assert_eq!((g.location, g.ready), (AirGroupLocation::Base(base(1)), 12));
    }
    // Accepted on day 4: at base 2 with 12 ready, moved once.
    let moves = run.all(|k| matches!(k, EventKind::AirGroupRebased { .. }));
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0].day, 4);
    match moves[0].kind {
        EventKind::AirGroupRebased {
            air_group,
            from,
            to,
            distance_nm,
            ready,
            ..
        } => assert_eq!(
            (air_group, from, to, distance_nm, ready),
            (group(2), base(1), base(2), n(200), 12)
        ),
        _ => unreachable!(),
    }
    let g = &run.at(4).air_groups[&group(2)];
    assert_eq!((g.location, g.ready), (AirGroupLocation::Base(base(2)), 12));
    // The order is in the log as refused or accepted, and no other order of
    // any day was refused.
    let refused = run.all(|k| matches!(k, EventKind::OrderRejected { .. }));
    assert_eq!(refused.len(), 2);
}

#[test]
fn group_2_may_fly_8_9_10_then_nothing_on_the_day_it_moves_then_8_and_9() {
    let run = Run::base();
    for (day, may_fly) in [(1, 8), (2, 9), (3, 10), (4, 0), (5, 8), (6, 9)] {
        assert_eq!(run.able_on(day, 2), may_fly, "group 2 on day {day}");
    }
    // 12 x 70/100 = 8.4 and 12 x 80/100 = 9.6 at base 2 on days 5 and 6.
    assert_eq!(share(12, 30), 8);
    assert_eq!(share(12, 20), 9);
}

#[test]
fn at_b_damage_goes_60_60_50_40_30_20_10_and_unit_2_mends_from_day_2() {
    let run = Run::base();
    for (day, through, after) in [
        (1, 60, 60),
        (2, 60, 50),
        (3, 50, 40),
        (4, 40, 30),
        (5, 30, 20),
        (6, 20, 10),
    ] {
        assert_eq!(run.damage(day - 1, 2), n(through), "day {day} through");
        assert_eq!(run.damage(day, 2), n(after), "day {day} after");
    }
    // Nobody works the field on day 1.
    assert!(run.worked(1, 2).is_empty());
    assert!(run.mended(1).iter().all(|m| m.1 != 2));
    // From day 2 unit 2 works at the airfield at half: 1000 x 0.5 = 500.
    for (day, damage) in [(2, 50), (3, 40), (4, 30), (5, 20), (6, 10)] {
        assert_eq!(
            run.worked(day, 2),
            vec![(Job::Airfield, 2, n(500))],
            "day {day}"
        );
        let unit_2: Vec<_> = run.mended(day).into_iter().filter(|m| m.0 == 2).collect();
        assert_eq!(unit_2, vec![(2, 2, n(10), n(damage))], "day {day}");
    }
}

#[test]
fn group_3_is_ready_4_and_may_fly_1_1_2_6_7_8() {
    let run = Run::base();
    for (day, ready, may_fly) in [
        (1, 4, 1),
        (2, 4, 1),
        (3, 4, 2),
        (4, 10, 6),
        (5, 10, 7),
        (6, 10, 8),
    ] {
        assert_eq!(run.ready_at_morning(day, 3), ready, "ready on day {day}");
        assert_eq!(run.able_on(day, 3), may_fly, "group 3 on day {day}");
    }
}

#[test]
fn the_arrival_waits_on_the_nights_of_days_2_and_3_and_comes_whole_on_the_night_of_day_4() {
    let run = Run::base();
    assert_eq!(
        run.waiting(),
        vec![
            (2, Some(Pulse::Night), n(60)),
            (3, Some(Pulse::Night), n(50))
        ]
    );
    assert_eq!(
        run.arrived(),
        vec![(4, Some(Pulse::Night), 6, 6, 10, 10, 10)]
    );
    // Group 3: 4 ready until the night of day 4, then 10.
    for day in 1..=3 {
        assert_eq!(run.at(day).air_groups[&group(3)].ready, 4, "day {day}");
    }
    for day in 4..=6 {
        let g = &run.at(day).air_groups[&group(3)];
        assert_eq!(
            (g.ready, g.pilots, g.strength),
            (10, Some(10), 10),
            "day {day}"
        );
    }
    assert!(run.at(3).arrivals[0].waiting);
    assert!(run.at(4).arrivals.is_empty() || !run.at(4).arrivals[0].waiting);
}

#[test]
fn the_bombers_launch_the_share_of_their_ready_aircraft_the_field_allows() {
    let run = Run::base();
    for day in 1..=DAYS {
        let damage = run.damage(day - 1, 2);
        let damage_whole = damage.trunc();
        for g in [4, 5] {
            let r = run.ready_at_morning(day, g);
            let expected = share(r, damage_whole);
            let launched = run.launched(day, g);
            assert_eq!(
                launched,
                vec![(Some(Pulse::Morning), expected)],
                "group {g} on day {day}: r = {r}, damage {damage}"
            );
            assert_eq!(run.able(day, Pulse::Morning, g), expected);
            eprintln!(
                "scenario 31: day {day} group {g}: r = {r}, damage at base 2 {damage}, launched {expected}"
            );
        }
    }
}

#[test]
fn the_raids_on_base_3_add_twice_the_hits_on_its_airfield_and_its_damage_never_falls() {
    let run = Run::base();
    let mut before = n(0);
    assert_eq!(run.damage(0, 3), before);
    for day in 1..=DAYS {
        let (hits, on_aircraft, on_supply, on_men) = run.hits(day, 4);
        let airfield = run.airfield_hits(day, 3);
        // Base 3 has no men ashore, and the rest landed on nothing.
        assert_eq!(on_men, 0);
        assert!(
            on_aircraft + on_supply + on_men + airfield <= hits,
            "day {day}: {hits} hits, {on_aircraft} {on_supply} {on_men} {airfield}"
        );
        let expected = (before + n(2 * i64::from(airfield))).min(n(100));
        let after = run.damage(day, 3);
        assert_eq!(after, expected, "day {day}");
        assert!(after >= before, "never falling, day {day}");
        // The event says the damage after.
        match run.damaged(day).as_slice() {
            [] => assert_eq!(airfield, 0),
            [(3, h, damage)] => assert_eq!((*h, *damage), (airfield, after)),
            other => panic!("day {day}: {other:?}"),
        }
        assert!(run.mended(day).iter().all(|m| m.1 != 3), "nobody mends it");
        eprintln!(
            "scenario 31: day {day} raid on base 3: {hits} hits, {on_aircraft} on aircraft, {on_supply} on supply, {on_men} on men, {airfield} on the airfield; base 3 damage {after}"
        );
        before = after;
    }
    // The raid's breakdown names the share where it counts.
    for day in 1..=DAYS {
        assert!(has_factor(run.attack(day, 4), "hit_on_airfield_chance"));
        let breakdown = run.attack(day, 4).breakdown.as_ref().expect("breakdown");
        let share = breakdown
            .factors
            .iter()
            .find(|f| f.name == "hit_on_airfield_chance")
            .expect("named");
        assert_eq!(share.value, fx("0.2"));
    }
}

#[test]
fn base_4_has_no_airfield_so_no_hit_lands_on_one() {
    let run = Run::base();
    for day in 1..=DAYS {
        let (hits, on_aircraft, on_supply, on_men) = run.hits(day, 5);
        assert!(on_aircraft + on_supply + on_men <= hits);
        assert_eq!(run.airfield_hits(day, 4), 0, "day {day}");
        assert_eq!(run.damage(day, 4), n(0), "day {day}");
        assert!(!has_factor(run.attack(day, 5), "hit_on_airfield_chance"));
        eprintln!(
            "scenario 31: day {day} raid on base 4: {hits} hits, {on_aircraft} on aircraft, {on_supply} on supply, {on_men} on men"
        );
    }
    assert!(!run
        .all(|k| matches!(k, EventKind::AirfieldDamaged { base, .. } if base.raw() == 4))
        .iter()
        .any(|_| true));
    assert!(!run.at(DAYS).bases[&base(4)].airfield);
}

#[test]
fn base_3s_damage_at_each_days_end_is_printed() {
    let run = Run::base();
    let line: Vec<String> = (0..=DAYS).map(|d| run.damage(d, 3).to_string()).collect();
    eprintln!("scenario 31: base 3 damage at the end of days 0 to 6: {line:?}");
}

#[test]
fn base_1_feeds_500_men_and_base_2_1000_men_six_nights() {
    let run = Run::base();
    assert_eq!(run.at(DAYS).bases[&base(1)].supply_t, n(4985));
    assert_eq!(run.at(DAYS).bases[&base(2)].supply_t, n(4970));
    // 2.5 t a night.
    assert_eq!(run.at(1).bases[&base(1)].supply_t, fx("4997.5"));
    assert_eq!(run.at(1).bases[&base(2)].supply_t, n(4995));
}

// ---------------------------------------------------------------- Properties

#[test]
fn damage_stays_within_0_and_100_and_with_no_worker_never_falls() {
    let run = Run::base();
    for raw in 1..=4 {
        let mut before = run.damage(0, raw);
        for day in 1..=DAYS {
            let after = run.damage(day, raw);
            assert!(after >= n(0) && after <= n(100), "base {raw} day {day}");
            // Bases 3 and 4 have no worker: their damage never falls.
            if raw >= 3 {
                assert!(after >= before, "base {raw} day {day}");
            }
            before = after;
        }
    }
}

#[test]
fn with_nobody_on_either_field_damage_stays_and_the_aircraft_that_may_fly_stay() {
    // Unit 1 to the port on day 1 and unit 2 told to the port on day 2 (a
    // later order for a day replaces the airfield one in force).
    let idle = format!(
        "{}{}{}",
        text(),
        work_order(1, 1, "port"),
        work_order(2, 2, "port")
    );
    let run = Run::of(&idle, "0.2", DAYS);
    for day in 1..=DAYS {
        assert_eq!(run.damage(day, 1), n(30), "base 1 day {day}");
        assert_eq!(run.damage(day, 2), n(60), "base 2 day {day}");
        assert!(run.mended(day).is_empty(), "day {day}");
        assert_eq!(run.able_on(day, 1), 8, "group 1 on day {day}");
        assert_eq!(run.able_on(day, 3), 1, "group 3 on day {day}");
    }
}

#[test]
fn the_aircraft_that_may_fly_are_never_more_than_ready_and_never_rise_as_damage_rises() {
    let run = Run::base();
    for s in &run.seen {
        assert!(s.able <= s.ready, "{s:?}");
        // Group 2 flies nothing on day 4, the day it moves; every other
        // group of every moment is the share of its ready aircraft.
        if (s.group, s.day) != (2, 4) {
            assert_eq!(
                s.able,
                share(s.ready, s.damage.trunc()),
                "{} on day {} {:?}: {s:?}",
                s.moment,
                s.day,
                s.pulse
            );
        }
    }
    // Group 1 and group 2 at base 1 on day 1, for starts from 0 to 100.
    let source = text();
    let mut last: Option<(u32, u32)> = None;
    for damage in (0..=100).step_by(5) {
        let variant = common::replace_once(
            &source,
            "airfield_damage = 30\n",
            &format!("airfield_damage = {damage}\n"),
        );
        let run = Run::of(&variant, "0.2", 1);
        let (g1, g2) = (run.able_on(1, 1), run.able_on(1, 2));
        assert_eq!(g1, share(12, damage), "damage {damage}");
        assert_eq!(g2, share(12, damage), "damage {damage}");
        assert!(g1 <= 12 && g2 <= 12);
        if let Some((p1, p2)) = last {
            assert!(g1 <= p1 && g2 <= p2, "damage {damage} gives more than less");
        }
        last = Some((g1, g2));
    }
    assert_eq!(last, Some((0, 0)), "at 100 nothing flies");
}

#[test]
fn the_arrival_is_delivered_once_and_whole_or_waits_whole_and_none_is_lost() {
    // As it is: delivered once.
    let run = Run::base();
    assert_eq!(run.arrived().len(), 1);
    assert_eq!(run.waiting().len(), 2);
    // A field that is whole: delivered the night it is due, never waiting.
    let whole = common::replace_once(&text(), "airfield_damage = 60\n", "");
    let run = Run::of(&whole, "0.2", DAYS);
    assert!(run.waiting().is_empty());
    assert_eq!(
        run.arrived(),
        vec![(2, Some(Pulse::Night), 6, 6, 10, 10, 10)]
    );
    // A field at 100 that unit 2 cannot mend below 50 in the game: waits
    // every night from day 2 to 6, whole, and is never delivered or lost.
    let wrecked =
        common::replace_once(&text(), "airfield_damage = 60\n", "airfield_damage = 100\n");
    let run = Run::of(&wrecked, "0.2", DAYS);
    assert!(run.arrived().is_empty());
    assert_eq!(
        run.waiting(),
        vec![
            (2, Some(Pulse::Night), n(100)),
            (3, Some(Pulse::Night), n(90)),
            (4, Some(Pulse::Night), n(80)),
            (5, Some(Pulse::Night), n(70)),
            (6, Some(Pulse::Night), n(60)),
        ]
    );
    let g = &run.at(DAYS).air_groups[&group(3)];
    assert_eq!((g.ready, g.pilots), (4, Some(4)));
    let arrival = &run.at(DAYS).arrivals[0];
    assert_eq!(
        (arrival.aircraft, arrival.pilots, arrival.waiting),
        (6, 6, true)
    );
    // And every rebase to it is refused on every day it is ordered.
    for day in [1, 3] {
        assert!(run.rebase_answer(day, 2).is_err(), "day {day}");
    }
}

#[test]
fn with_the_share_at_0_and_no_damage_stated_nothing_of_the_rule_happens() {
    let run = Run::of(&without_damage(&text()), "0", DAYS);
    for e in &run.events {
        assert!(
            !matches!(
                e.kind,
                EventKind::AirfieldDamaged { .. }
                    | EventKind::AirfieldMended { .. }
                    | EventKind::ArrivalWaiting { .. }
            ),
            "{e:?}"
        );
        if let EventKind::OrderRejected { reason, .. } = &e.kind {
            assert!(
                !matches!(reason, RejectReason::AirfieldTooDamaged { .. }),
                "{e:?}"
            );
        }
    }
    for day in 1..=DAYS {
        for raw in 1..=4 {
            assert_eq!(run.damage(day, raw), n(0));
        }
        // Whole fields: every ready aircraft may fly, but group 2, which the
        // order of day 1 moves (accepted at a whole field) and which flies
        // nothing on the day it moves.
        assert_eq!(run.able_on(1, 2), 0);
        for g in (1..=5).filter(|g| !(day == 1 && *g == 2)) {
            let found: Vec<&Seen> = run
                .seen
                .iter()
                .filter(|s| s.day == day && s.group == g)
                .collect();
            assert!(found.iter().all(|s| s.able == s.ready), "{found:?}");
        }
    }
    assert_eq!(
        run.arrived(),
        vec![(2, Some(Pulse::Night), 6, 6, 10, 10, 10)]
    );
    // And the raids' breakdown does not name the share at 0.
    assert!(!has_factor(run.attack(1, 4), "hit_on_airfield_chance"));
}

#[test]
fn the_japanese_view_is_the_same_whatever_the_allied_field_damage_and_work() {
    let base_run = Run::base();
    // Base 1 starts whole.
    let whole_1 = Run::of(&without_base_1_damage(&text()), "0.2", DAYS);
    // Base 1 starts at 30 and its unit is sent to the port: nobody mends it.
    let unmended = Run::of(
        &format!("{}{}", text(), work_order(1, 1, "port")),
        "0.2",
        DAYS,
    );
    for (name, other) in [("whole", &whole_1), ("unmended", &unmended)] {
        // The Allied view is not the same: the test would prove nothing if
        // it were.
        assert_ne!(base_run.allied, other.allied, "{name}: the worlds differ");
        // The hits are the same.
        for day in 1..=DAYS {
            for g in [4, 5] {
                assert_eq!(
                    base_run.hits(day, g),
                    other.hits(day, g),
                    "{name} day {day}"
                );
            }
        }
        assert_eq!(base_run.japanese, other.japanese, "{name}");
    }
}

#[test]
fn the_enemy_sees_none_of_a_field_and_a_raids_claim_says_nothing_of_it() {
    let run = Run::base();
    let allied = run.scenario.world.side_view(Side::Allied);
    let japanese = run.scenario.world.side_view(Side::Japanese);
    // The Allied side is never told of the damage to the Japanese field.
    for e in allied.events() {
        assert!(
            !matches!(
                &e.kind,
                sim_core::side_events::SideEventKind::Whole(EventKind::AirfieldDamaged { .. })
            ),
            "{e:?}"
        );
    }
    // The Japanese side sees the damage to its own field, whole, on each
    // day the raid hit it, and none of the Allied mending or waiting.
    let own: u32 = japanese
        .events()
        .iter()
        .filter(|e| {
            matches!(
                &e.kind,
                sim_core::side_events::SideEventKind::Whole(EventKind::AirfieldDamaged { .. })
            )
        })
        .count()
        .try_into()
        .expect("a count");
    let days_hit = (1..=DAYS).filter(|d| run.airfield_hits(*d, 3) > 0).count();
    assert_eq!(own as usize, days_hit);
    for e in japanese.events() {
        assert!(
            !matches!(
                &e.kind,
                sim_core::side_events::SideEventKind::Whole(
                    EventKind::AirfieldMended { .. } | EventKind::ArrivalWaiting { .. }
                )
            ),
            "{e:?}"
        );
    }
    // A raid's claim: hits, kind, aircraft, nothing of the field.
    assert!(!allied.claims().is_empty(), "the raids made claims");
    for claim in allied.claims() {
        let words = format!("{:?} {}", claim, claim.text(Side::Allied)).to_lowercase();
        assert!(!words.contains("airfield"), "{words}");
        assert!(!words.contains("damage"), "{words}");
        assert!(!words.contains("field"), "{words}");
    }
}
