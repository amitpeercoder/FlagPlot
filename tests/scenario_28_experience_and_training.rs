//! Scenario 28: a raw unit trains to the standard and no further; a unit one
//! day of fighting short of a step gains it; experience moves the chance a
//! shot hits; and a unit that is not fed does not train.
//!
//! Proves decision 0097 (the quality of a land unit, and training) against a
//! hand calculation. The data is placeholder data: see
//! `data/scenarios/28_experience_and_training.toml`. The frozen step 1
//! figures set `experience_fire_strength` to 0, so this test gives the four
//! figures the values below, as the test of ships' experience does.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide (the
//! hits of a fight), the calculation gives the firing men, the shots and
//! the chance exactly; the tests read the hits from the run. Everything
//! else is asserted exactly. If the code disagrees, the code or the rule is
//! wrong: do not change a number here to make a test pass. A mismatch goes
//! back to the architect.
//!
//! ## Figures
//!
//! `experience_fire_strength` 0.25, `training_days_per_level` 30,
//! `training_ceiling` 2, `fight_days_per_level` 10. From the earlier stages:
//! `daily_supply_t_per_man` 0.005, `unfed_loss_share_per_day` 0.05,
//! `men_per_shot` 100, `hit_chance` 0.3, `defender_hit_chance` 0.6,
//! `men_per_hit` 20, `fatigue_per_attack_day` 20.
//!
//! The factor of experience on a unit's chance to hit is 1 + 0.25 x
//! (experience - 2): 0.5 at 0, 0.75 at 1, 1 at 2, 1.25 at 3 and 1.5 at 4.
//!
//! ## Inputs
//!
//! A placeholder map with four coast hexes far apart and no land links: A,
//! F, G and H. The game is 62 days.
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Allied   | A   | 5000 t |
//! | 2    | Allied   | F   | 5000 t |
//! | 3    | Japanese | F   | 5000 t |
//! | 4    | Allied   | G   | 5000 t |
//! | 5    | Japanese | G   | 5000 t |
//! | 6    | Allied   | H   | 1 t    |
//!
//! | Unit | Side     | Men  | At | Experience | Days counted at the start | Posture |
//! | ---- | -------- | ---- | -- | ---------- | ------------------------- | ------- |
//! | 1    | Allied   | 1000 | A  | 0          | none                      | defend  |
//! | 2    | Allied   | 1000 | F  | 2          | 9 days of fighting        | defend  |
//! | 3    | Japanese | 600  | F  | 3          | none                      | attack  |
//! | 4    | Allied   | 500  | G  | 0          | none                      | attack  |
//! | 5    | Japanese | 500  | G  | 4          | none                      | defend  |
//! | 6    | Allied   | 100  | H  | 1          | none                      | defend  |
//!
//! Orders: on day 1, units 1 and 6 are set to train.
//!
//! A unit's day is settled in the ground combat phase, after the fights. A
//! day of training counts if all its men are ashore at a base of its side,
//! it was fed in full that night, it did not march and its hex was not
//! contested.
//!
//! ## Unit 1: training to the standard
//!
//! It is fed in full every night (5 t a night from base 1) and nothing
//! disturbs it.
//!
//! | After day | Days of training counted | Experience |
//! | --------- | ------------------------ | ---------- |
//! | 29        | 29                       | 0          |
//! | 30        | 0, the count starts again | 1         |
//! | 59        | 29                       | 1          |
//! | 60        | 0                        | 2          |
//! | 62        | 0                        | 2          |
//!
//! At 2 it is at the ceiling of training. Days 61 and 62 are spent training
//! and count nothing. Base 1 has paid 62 x 5 = 310 t and holds 4690 t.
//!
//! ## Unit 6: not fed, not trained
//!
//! Base 6 has 1 t. Nights of days 1 and 2: 100 x 0.005 = 0.5 t each, fed in
//! full; days 1 and 2 count. From the night of day 3 the base has no stock:
//! the unit is unfed, loses men by the unfed rule, and no day counts. At the
//! end it has 2 days of training counted and its experience is 1.
//!
//! Its men: 100 x 0.05 = 5 lost on the night of day 3, leaving 95; then its
//! men times 0.05, cut, and one man at least, each night after.
//!
//! ## F: a step from fighting, and the chance
//!
//! Both sides are fed in full every night. The Japanese side attacks and
//! the Allied side defends.
//!
//! Day 1.
//!
//! - Unit 3, experience 3: 600 firing men, 6 shots at 0.3 x 1.25 = 0.375.
//! - Unit 2, experience 2: 1000 firing men, 10 shots at 0.6 x 1 = 0.6.
//!
//! Both fired. Unit 2 had 9 days of fighting counted: this is its tenth, so
//! its experience rises to 3 and its count starts again at 0. Unit 3 has 1
//! day counted and stays at 3.
//!
//! Day 2, if both still have men: unit 2 now fires at 0.6 x 1.25 = 0.75.
//! Unit 3 fires at 0.375 with its men times 80/100, cut, since its fatigue
//! is 20.
//!
//! Each later day of fighting adds one to the count of each unit that
//! fired. Unit 3 attacks on four days and then goes back to defend; there
//! is no fight after that. A unit's experience at the end is what follows
//! from the days it fired: unit 2 is 3 unless it fired on ten more days,
//! which it cannot here.
//!
//! ## G: raw against the best
//!
//! Day 1. The Allied side attacks and the Japanese side defends.
//!
//! - Unit 4, experience 0: 500 firing men, 5 shots at 0.3 x 0.5 = 0.15.
//! - Unit 5, experience 4: 500 firing men, 5 shots at 0.6 x 1.5 = 0.9.
//!
//! The breakdown of each unit's fire names its experience and the factor.
//!
//! # Properties
//!
//! - Experience is never below 0 nor above 4, training never takes a unit
//!   above the ceiling, and a day counts for one thing only.
//! - With the frozen strength of 0, every scenario with land units gives
//!   what it gave before, apart from the hash version.
//! - The enemy's view of a fight is the same in two worlds that differ only
//!   in the other side's experience, where the hits are the same.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind, ExperienceSource};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EventId, LandUnitId, OrderId};
use sim_core::orders::{Activity, Posture};
use sim_core::side_events::SideEventKind;
use sim_core::state::{Side, TruthState};
use sim_data::{LoadOptions, Scenario};

// ------------------------------------------------------------------ Helpers

const F: Hex = Hex::new(10, 0);
const G: Hex = Hex::new(20, 0);

fn path() -> PathBuf {
    common::data_dir().join("scenarios/28_experience_and_training.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// The frozen figures of step 1 with `experience_fire_strength` 0.25. The
/// frozen `training_days_per_level` 30, `training_ceiling` 2 and
/// `fight_days_per_level` 10 are the calculation's values already.
fn working() -> LoadOptions {
    common::altered_reference("experience", |dir| {
        common::edit_file_by(
            dir,
            "ground.toml",
            &[(
                "experience_fire_strength = 0",
                "experience_fire_strength = \"0.25\"",
            )],
        );
    })
}

fn from(source: &str) -> Scenario {
    common::from_text_with(source, &path(), &working()).expect("the variant loads")
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

fn with(source: &str, block: &str) -> String {
    format!("{source}\n{block}")
}

fn with_order(source: &str, day: u32, side: &str, target: &str, kind: &str, keys: &str) -> String {
    with(
        source,
        &format!(
            "[[order]]\nday = {day}\nside = \"{side}\"\n{target}\nkind = \"{kind}\"\n{keys}\n"
        ),
    )
}

/// One fire of a unit in a day's fight.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Fire {
    day: u32,
    unit: u32,
    hex: Hex,
    firing_men: u32,
    shots: Fixed,
    chance: Fixed,
    hits: u32,
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

    fn men(&self, day: u32, raw: u32) -> u32 {
        self.unit_at(day, raw).men_ashore
    }

    fn supply(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).bases[&base(raw)].supply_t
    }

    fn on(&self, day: u32, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.day == day && pick(&e.kind))
            .collect()
    }

    fn one(&self, day: u32, pick: impl Fn(&EventKind) -> bool) -> &Event {
        let found = self.on(day, pick);
        assert_eq!(
            found.len(),
            1,
            "day {day}: exactly one such event: {found:?}"
        );
        found[0]
    }

    fn all(&self, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events.iter().filter(|e| pick(&e.kind)).collect()
    }

    /// Every fire of `day`, in log order.
    fn fires(&self, day: u32) -> Vec<(Fire, &Event)> {
        self.on(day, |k| matches!(k, EventKind::GroundFire { .. }))
            .into_iter()
            .map(|e| match e.kind {
                EventKind::GroundFire {
                    unit,
                    hex,
                    firing_men,
                    shots,
                    chance,
                    hits,
                } => (
                    Fire {
                        day,
                        unit: unit.raw(),
                        hex,
                        firing_men,
                        shots,
                        chance,
                        hits,
                    },
                    e,
                ),
                _ => unreachable!(),
            })
            .collect()
    }

    fn fire(&self, day: u32, raw: u32) -> Fire {
        let found: Vec<Fire> = self
            .fires(day)
            .into_iter()
            .map(|(f, _)| f)
            .filter(|f| f.unit == raw)
            .collect();
        assert_eq!(found.len(), 1, "unit {raw} fired once on day {day}");
        found[0].clone()
    }

    fn fired_days(&self, raw: u32) -> Vec<u32> {
        (1..=self.len())
            .filter(|day| self.fires(*day).iter().any(|(f, _)| f.unit == raw))
            .collect()
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

fn has_factor(event: &Event, name: &str) -> bool {
    event
        .breakdown
        .as_ref()
        .is_some_and(|b| b.factors.iter().any(|f| f.name == name))
}

fn caused_by_event(event: &Event, cause: EventId) -> bool {
    event.causes.contains(&Cause::Event(cause))
}

fn caused_by_order(event: &Event, order: OrderId) -> bool {
    event.causes.contains(&Cause::Order(order))
}

/// What a unit has after `fired` days of fighting that count, from `days`
/// counted at `experience`: a step every 10 days, none at 4.
fn after_fighting(experience: u32, days: u32, fired: u32) -> (u32, u32) {
    let (mut experience, mut days) = (experience, days);
    for _ in 0..fired {
        if experience < 4 {
            days += 1;
            if days == 10 {
                experience += 1;
                days = 0;
            }
        }
    }
    (experience, days)
}

/// The Allied view's text of the events of `day`, by its own numbers.
fn day_digest(scenario: &Scenario, side: Side, day: u32) -> String {
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
    for e in view.ground_estimates() {
        out += &format!("{:?} {} {}\n", e.hex, e.men, e.day);
    }
    out
}

fn is_new_event(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::TrainingDay { .. }
            | EventKind::FightDayCounted { .. }
            | EventKind::ExperienceRaised { .. }
    )
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_figures_and_the_inputs_are_the_calculations() {
    let scenario = from(&text());
    let g = scenario.world.ref_data().ground.expect("ground figures");
    assert_eq!(g.experience_fire_strength, fx("0.25"));
    assert_eq!(g.training_days_per_level, 30);
    assert_eq!(g.training_ceiling, 2);
    assert_eq!(g.fight_days_per_level, 10);
    assert_eq!(g.daily_supply_t_per_man, fx("0.005"));
    assert_eq!(g.unfed_loss_share_per_day, fx("0.05"));
    assert_eq!(g.men_per_shot, n(100));
    assert_eq!(g.hit_chance, fx("0.3"));
    assert_eq!(g.defender_hit_chance, fx("0.6"));
    assert_eq!(g.men_per_hit, n(20));
    assert_eq!(g.fatigue_per_attack_day, n(20));
    let truth = scenario.world.umpire();
    // Bases: id, side, hex, supply.
    for (id, side, q, supply) in [
        (1, Side::Allied, 0, n(5000)),
        (2, Side::Allied, 10, n(5000)),
        (3, Side::Japanese, 10, n(5000)),
        (4, Side::Allied, 20, n(5000)),
        (5, Side::Japanese, 20, n(5000)),
        (6, Side::Allied, 30, n(1)),
    ] {
        let b = &truth.bases[&base(id)];
        assert_eq!((b.side, b.hex, b.supply_t), (side, Hex::new(q, 0), supply));
    }
    // Units: id, side, men, hex, experience, days of fighting, posture.
    for (id, side, men, q, experience, fight_days, posture) in [
        (1, Side::Allied, 1000, 0, 0, 0, Posture::Defend),
        (2, Side::Allied, 1000, 10, 2, 9, Posture::Defend),
        (3, Side::Japanese, 600, 10, 3, 0, Posture::Attack),
        (4, Side::Allied, 500, 20, 0, 0, Posture::Attack),
        (5, Side::Japanese, 500, 20, 4, 0, Posture::Defend),
        (6, Side::Allied, 100, 30, 1, 0, Posture::Defend),
    ] {
        let u = &truth.land_units[&unit(id)];
        assert_eq!(
            (
                u.side,
                u.men_ashore,
                u.hex,
                u.experience,
                u.fight_days,
                u.posture
            ),
            (
                side,
                men,
                Some(Hex::new(q, 0)),
                experience,
                fight_days,
                posture
            )
        );
        assert_eq!((u.training_days, u.activity), (0, Activity::Ready));
    }
    // No land link joins any two hexes: the four hexes are far apart.
    let map = scenario.world.map();
    for (a, b) in [(0, 10), (10, 20), (20, 30)] {
        assert!(!map.has_land_link(Hex::new(a, 0), Hex::new(b, 0)));
    }
    // Orders: on day 1, units 1 and 6 are set to train.
    let orders = scenario.scripted_orders();
    assert_eq!(orders.len(), 1);
    assert_eq!(orders[&1].len(), 2);
}

#[test]
fn unit_1_trains_to_the_standard_and_no_further() {
    let run = Run::text(&text());
    assert_eq!(run.len(), 62);
    let order = {
        let e = run.one(1, |k| {
            matches!(k, EventKind::OrderAccepted { target, .. } if *target == sim_core::ids::EntityRef::LandUnit(unit(1)))
        });
        match e.kind {
            EventKind::OrderAccepted { order, .. } => order,
            _ => unreachable!(),
        }
    };
    // After day: days of training counted, experience.
    for (day, days, experience) in [(29, 29, 0), (30, 0, 1), (59, 29, 1), (60, 0, 2), (62, 0, 2)] {
        let u = run.unit_at(day, 1);
        assert_eq!(
            (u.training_days, u.experience as u32),
            (days, experience),
            "day {day}"
        );
    }
    // Every day in between follows from it.
    for day in 1..=62u32 {
        let (days, experience) = match day {
            1..=29 => (day, 0),
            30 => (0, 1),
            31..=59 => (day - 30, 1),
            _ => (0, 2),
        };
        let u = run.unit_at(day, 1);
        assert_eq!(
            (u.training_days, u.experience as u32),
            (days, experience),
            "day {day}"
        );
        assert_eq!(u.activity, Activity::Train);
        assert_eq!(u.men_ashore, 1000);
    }
    // 60 days of training are recorded, and none on days 61 and 62 at the
    // ceiling, where the days are spent and count nothing.
    let trained = run.all(|k| matches!(k, EventKind::TrainingDay { unit, .. } if unit.raw() == 1));
    assert_eq!(trained.len(), 60);
    for day in [61, 62] {
        assert!(run
            .on(day, |k| matches!(k, EventKind::TrainingDay { .. }))
            .is_empty());
    }
    for e in &trained {
        // Each cites the activity order and that night's feeding of the side
        // in its hex.
        assert!(caused_by_order(e, order));
        let fed = run.on(e.day, |k| {
            matches!(k, EventKind::MenFed { side: Side::Allied, hex, .. } if *hex == Hex::new(0, 0))
        });
        assert_eq!(fed.len(), 1);
        assert!(caused_by_event(e, fed[0].id));
        let EventKind::TrainingDay {
            days, experience, ..
        } = e.kind
        else {
            unreachable!()
        };
        // 1 to 30 at experience 0, then 1 to 30 at 1.
        let expected_days = if e.day <= 30 { e.day } else { e.day - 30 };
        assert_eq!(
            (days, u32::from(experience)),
            (expected_days, u32::from(e.day > 30))
        );
        assert_eq!(factor(e, "days_before"), n(i64::from(days) - 1));
        assert_eq!(factor(e, "training_days_per_level"), n(30));
    }
    // The two steps, citing the days that earned them.
    let raised =
        run.all(|k| matches!(k, EventKind::ExperienceRaised { unit, .. } if unit.raw() == 1));
    assert_eq!(raised.len(), 2);
    for (e, (day, experience)) in raised.iter().zip([(30, 1), (60, 2)]) {
        assert_eq!(e.day, day);
        assert_eq!(
            e.kind,
            EventKind::ExperienceRaised {
                unit: unit(1),
                experience,
                by: ExperienceSource::Training,
            }
        );
        let day_event = run.on(
            day,
            |k| matches!(k, EventKind::TrainingDay { unit, days: 30, .. } if unit.raw() == 1),
        );
        assert_eq!(day_event.len(), 1);
        assert!(caused_by_event(e, day_event[0].id));
    }
    // Base 1 has paid 62 x 5 = 310 t and holds 4690 t.
    assert_eq!(run.supply(62, 1), n(4690));
    for day in 1..=62 {
        assert_eq!(run.supply(day, 1), n(5000 - 5 * i64::from(day)));
    }
}

#[test]
fn unit_6_is_not_fed_and_so_not_trained() {
    let run = Run::text(&text());
    // Nights of days 1 and 2: 100 x 0.005 = 0.5 t each, fed in full. The base
    // is out of supply from the night of day 3.
    assert_eq!(run.supply(0, 6), n(1));
    assert_eq!(run.supply(1, 6), fx("0.5"));
    assert_eq!(run.supply(2, 6), n(0));
    assert_eq!(run.supply(3, 6), n(0));
    for day in [1, 2] {
        let fed = run.one(
            day,
            |k| matches!(k, EventKind::MenFed { hex, .. } if *hex == Hex::new(30, 0)),
        );
        assert!(matches!(
            fed.kind,
            EventKind::MenFed {
                fed: 100,
                unfed: 0,
                ..
            }
        ));
        assert_eq!(run.unit_at(day, 6).training_days, day);
        assert_eq!(
            run.one(
                day,
                |k| matches!(k, EventKind::TrainingDay { unit, .. } if unit.raw() == 6)
            )
            .kind,
            EventKind::TrainingDay {
                unit: unit(6),
                days: day,
                experience: 1,
            }
        );
    }
    // No day counts after that.
    assert!(run
        .all(|k| matches!(k, EventKind::TrainingDay { unit, .. } if unit.raw() == 6))
        .iter()
        .all(|e| e.day <= 2));
    // The men: 100 x 0.05 = 5 lost on the night of day 3, leaving 95; then
    // its men times 0.05, cut, and one man at least, each night after.
    let mut model = 100u32;
    let mut destroyed_on = None;
    assert_eq!((run.men(1, 6), run.men(2, 6)), (100, 100));
    for day in 3..=62u32 {
        if model > 0 {
            model -= std::cmp::max(1, model * 5 / 100);
            if model == 0 {
                destroyed_on = Some(day);
            }
        }
        assert_eq!(run.men(day, 6), model, "day {day}");
    }
    assert_eq!(run.men(3, 6), 95);
    let lost = run.one(
        3,
        |k| matches!(k, EventKind::MenLostUnfed { unit, .. } if unit.raw() == 6),
    );
    assert_eq!(
        lost.kind,
        EventKind::MenLostUnfed {
            unit: unit(6),
            hex: Hex::new(30, 0),
            lost: 5,
            ashore: 95,
        }
    );
    assert_eq!(
        run.all(|k| matches!(k, EventKind::LandUnitDestroyed { unit, .. } if unit.raw() == 6))
            .first()
            .map(|e| e.day),
        destroyed_on
    );
    // At the end it has 2 days of training counted and its experience is 1.
    let end = run.unit_at(62, 6);
    assert_eq!((end.training_days, end.experience), (2, 1));
}

#[test]
fn f_on_day_1_and_2_follows_the_calculation() {
    let run = Run::text(&text());
    // Day 1. Unit 3, experience 3: 600 firing men, 6 shots at 0.3 x 1.25 =
    // 0.375. Unit 2, experience 2: 1000 firing men, 10 shots at 0.6 x 1 =
    // 0.6.
    let three = run.fire(1, 3);
    assert_eq!(
        (three.hex, three.firing_men, three.shots, three.chance),
        (F, 600, n(6), fx("0.375"))
    );
    let two = run.fire(1, 2);
    assert_eq!(
        (two.hex, two.firing_men, two.shots, two.chance),
        (F, 1000, n(10), fx("0.6"))
    );
    // The hits are the dice's, never more than the shots.
    assert!(three.hits <= 6 && two.hits <= 10);
    // Both fired. Unit 2 had 9 days counted: this is its tenth, so its
    // experience rises to 3 and its count starts again at 0. Unit 3 has 1
    // day counted and stays at 3.
    let counted = run.one(
        1,
        |k| matches!(k, EventKind::FightDayCounted { unit, .. } if unit.raw() == 2),
    );
    assert_eq!(
        counted.kind,
        EventKind::FightDayCounted {
            unit: unit(2),
            days: 10,
            experience: 2,
        }
    );
    assert_eq!(factor(counted, "days_before"), n(9));
    assert_eq!(factor(counted, "fight_days_per_level"), n(10));
    let raised = run.one(
        1,
        |k| matches!(k, EventKind::ExperienceRaised { unit, .. } if unit.raw() == 2),
    );
    assert_eq!(
        raised.kind,
        EventKind::ExperienceRaised {
            unit: unit(2),
            experience: 3,
            by: ExperienceSource::Fighting,
        }
    );
    assert!(caused_by_event(raised, counted.id));
    let (two_fire, three_fire) = (
        run.one(
            1,
            |k| matches!(k, EventKind::GroundFire { unit, .. } if unit.raw() == 2),
        ),
        run.one(
            1,
            |k| matches!(k, EventKind::GroundFire { unit, .. } if unit.raw() == 3),
        ),
    );
    assert!(caused_by_event(counted, two_fire.id));
    let three_counted = run.one(
        1,
        |k| matches!(k, EventKind::FightDayCounted { unit, .. } if unit.raw() == 3),
    );
    assert_eq!(
        three_counted.kind,
        EventKind::FightDayCounted {
            unit: unit(3),
            days: 1,
            experience: 3,
        }
    );
    assert!(caused_by_event(three_counted, three_fire.id));
    let (u2, u3) = (run.unit_at(1, 2), run.unit_at(1, 3));
    assert_eq!((u2.experience, u2.fight_days), (3, 0));
    assert_eq!((u3.experience, u3.fight_days), (3, 1));
    // The losses are the enemy's hits at 20 men a hit.
    assert_eq!(run.men(1, 2), 1000 - 20 * three.hits);
    assert_eq!(run.men(1, 3), 600 - 20 * two.hits);

    // Day 2, if both still have men: unit 2 now fires at 0.6 x 1.25 = 0.75,
    // with all its men. Unit 3 fires at 0.375 with its men times 80/100,
    // cut, since its fatigue is 20.
    assert!(run.men(1, 2) > 0 && run.men(1, 3) > 0);
    assert_eq!(run.unit_at(1, 3).fatigue, n(20));
    let two = run.fire(2, 2);
    assert_eq!(two.firing_men, run.men(1, 2));
    assert_eq!(two.chance, fx("0.75"));
    assert_eq!(two.shots, Fixed::from_ratio(i64::from(two.firing_men), 100));
    let three = run.fire(2, 3);
    assert_eq!(three.firing_men, run.men(1, 3) * 80 / 100);
    assert_eq!(three.chance, fx("0.375"));
    assert_eq!(
        three.shots,
        Fixed::from_ratio(i64::from(three.firing_men), 100)
    );
    assert_eq!(
        factor(
            run.fires(2)
                .iter()
                .find(|(f, _)| f.unit == 3)
                .expect("fired")
                .1,
            "fatigue_factor"
        ),
        fx("0.8")
    );
}

#[test]
fn each_later_day_of_fighting_adds_one_to_the_count_of_each_unit_that_fired() {
    let run = Run::text(&text());
    // Unit 3 attacks on four days and then goes back to defend: there is no
    // fight after that. Its fatigue is above the limit of 60 after day 4.
    let fights: Vec<u32> = (1..=62).filter(|d| !run.fires(*d).is_empty()).collect();
    assert_eq!(fights, vec![1, 2, 3, 4]);
    assert_eq!(
        run.on(
            4,
            |k| matches!(k, EventKind::LandUnitBackToDefend { unit, .. } if unit.raw() == 3)
        )
        .len(),
        1
    );
    assert_eq!(run.unit_at(4, 3).posture, Posture::Defend);
    for (raw, start_experience, start_days) in [(2, 2, 9), (3, 3, 0), (4, 0, 0), (5, 4, 0)] {
        let fired = run.fired_days(raw);
        assert_eq!(fired, vec![1, 2, 3, 4], "unit {raw}");
        // A day is counted for each that fired, while experience is below 4.
        let mut experience = start_experience;
        let mut days = start_days;
        for day in 1..=4 {
            let (e, d) = after_fighting(experience, days, 1);
            let counted = run.on(
                day,
                |k| matches!(k, EventKind::FightDayCounted { unit, .. } if unit.raw() == raw),
            );
            if experience < 4 {
                assert_eq!(counted.len(), 1, "unit {raw} day {day}");
                assert_eq!(
                    counted[0].kind,
                    EventKind::FightDayCounted {
                        unit: unit(raw),
                        days: if d == 0 { 10 } else { d },
                        experience: experience as u8,
                    }
                );
            } else {
                assert!(counted.is_empty(), "unit {raw} at 4 counts nothing");
            }
            experience = e;
            days = d;
            let u = run.unit_at(day, raw);
            assert_eq!(
                (u.experience as u32, u.fight_days),
                (e, d),
                "unit {raw} day {day}"
            );
        }
        // At the end: what follows from the days it fired.
        let end = run.unit_at(62, raw);
        assert_eq!(
            (end.experience as u32, end.fight_days),
            after_fighting(start_experience, start_days, fired.len() as u32)
        );
    }
    // Unit 2 is 3 unless it fired on ten more days, which it cannot here.
    assert_eq!(run.unit_at(62, 2).experience, 3);
    assert_eq!(run.unit_at(62, 2).fight_days, 3);
    assert_eq!(run.unit_at(62, 3).fight_days, 4);
    assert_eq!(run.unit_at(62, 4).fight_days, 4);
    assert_eq!(run.unit_at(62, 5).fight_days, 0);
    assert_eq!(run.unit_at(62, 5).experience, 4);
    // Only units 1 and 2 ever rose.
    let raised: Vec<u32> = run
        .all(|k| matches!(k, EventKind::ExperienceRaised { .. }))
        .iter()
        .map(|e| match e.kind {
            EventKind::ExperienceRaised { unit, .. } => unit.raw(),
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(raised, vec![2, 1, 1]);
}

#[test]
fn every_fire_follows_the_men_the_fatigue_the_posture_and_the_experience() {
    let run = Run::text(&text());
    for day in 1..=4u32 {
        let fires = run.fires(day);
        assert_eq!(fires.len(), 4, "day {day}");
        for (fire, event) in fires {
            let before = run.unit_at(day - 1, fire.unit);
            // Firing men: the men ashore, cut by the fatigue.
            let fatigue = before.fatigue.trunc() as u32;
            assert_eq!(fire.firing_men, before.men_ashore * (100 - fatigue) / 100);
            assert_eq!(
                fire.shots,
                Fixed::from_ratio(i64::from(fire.firing_men), 100)
            );
            // The side's chance by posture, times 1 + 0.25 x (experience - 2).
            let side_chance = if before.posture == Posture::Attack {
                fx("0.3")
            } else {
                fx("0.6")
            };
            let experience = i64::from(before.experience);
            let factor_of_experience = n(1) + fx("0.25") * Fixed::from_int(experience - 2);
            assert_eq!(fire.chance, side_chance * factor_of_experience);
            // The breakdown names each.
            assert_eq!(factor(event, "side_chance"), side_chance);
            assert_eq!(factor(event, "experience"), n(experience));
            assert_eq!(factor(event, "experience_factor"), factor_of_experience);
            assert_eq!(factor(event, "chance"), fire.chance);
            assert!(fire.hits as i64 <= fire.shots.trunc() + 1);
        }
        // Each side's loss is the other's hits at 20 men a hit.
        for (a, b) in [(2, 3), (4, 5)] {
            let (fa, fb) = (run.fire(day, a), run.fire(day, b));
            let before = |raw: u32| run.men(day - 1, raw);
            assert_eq!(
                run.men(day, a),
                before(a) - std::cmp::min(before(a), 20 * fb.hits)
            );
            assert_eq!(
                run.men(day, b),
                before(b) - std::cmp::min(before(b), 20 * fa.hits)
            );
        }
    }
}

#[test]
fn g_raw_against_the_best_on_day_1() {
    let run = Run::text(&text());
    // Unit 4, experience 0: 500 firing men, 5 shots at 0.3 x 0.5 = 0.15. Unit
    // 5, experience 4: 500 firing men, 5 shots at 0.6 x 1.5 = 0.9.
    let four = run.fire(1, 4);
    assert_eq!(
        (four.hex, four.firing_men, four.shots, four.chance),
        (G, 500, n(5), fx("0.15"))
    );
    let five = run.fire(1, 5);
    assert_eq!(
        (five.hex, five.firing_men, five.shots, five.chance),
        (G, 500, n(5), fx("0.9"))
    );
    // The breakdown of each unit's fire names its experience and the factor.
    let by_unit = |raw: u32| {
        run.fires(1)
            .into_iter()
            .find(|(f, _)| f.unit == raw)
            .expect("fired")
            .1
    };
    let (e4, e5) = (by_unit(4), by_unit(5));
    assert_eq!(factor(e4, "side_chance"), fx("0.3"));
    assert_eq!(factor(e4, "experience"), n(0));
    assert_eq!(factor(e4, "experience_factor"), fx("0.5"));
    assert_eq!(factor(e4, "chance"), fx("0.15"));
    assert_eq!(factor(e5, "side_chance"), fx("0.6"));
    assert_eq!(factor(e5, "experience"), n(4));
    assert_eq!(factor(e5, "experience_factor"), fx("1.5"));
    assert_eq!(factor(e5, "chance"), fx("0.9"));
    // Unit 4 counted a day at 0; unit 5 at 4 counts none.
    assert_eq!(run.unit_at(1, 4).fight_days, 1);
    assert_eq!(run.unit_at(1, 5).fight_days, 0);
    assert!(run
        .all(|k| matches!(k, EventKind::FightDayCounted { unit, .. } if unit.raw() == 5))
        .is_empty());
}

// ------------------------------------------------------------- The properties

#[test]
fn property_experience_is_in_0_to_4_training_never_passes_the_ceiling_and_a_day_counts_for_one_thing(
) {
    // The scenario, and a variant where unit 2, with no experience and 9 days
    // of fighting, is set to train on day 1 while it fights, and goes on
    // training when the fighting is over (no fight after day 4).
    let variant = common::replace_once(
        &text(),
        "experience = 2\nfight_days = 9\n",
        "experience = 0\nfight_days = 9\n",
    );
    let variant = with_order(
        &variant,
        1,
        "allied",
        "land_unit = 2",
        "set_activity",
        "activity = \"train\"",
    );
    for (name, run) in [
        ("scenario", Run::text(&text())),
        ("variant", Run::text(&variant)),
    ] {
        for day in 0..=run.len() {
            for u in run.at(day).land_units.values() {
                assert!(u.experience <= 4, "{name} {:?} day {day}", u.id);
                assert!(u.training_days < 30, "{name}");
                assert!(u.fight_days < 10, "{name}");
                // Training never takes a unit above the ceiling of 2: only a
                // unit that started there, or fought up, is above it.
                if u.id.raw() == 1 || u.id.raw() == 6 || u.id.raw() == 2 && name == "variant" {
                    let fought = run.all(|k| {
                        matches!(k, EventKind::ExperienceRaised { unit, by: ExperienceSource::Fighting, .. } if *unit == u.id)
                    });
                    let fought_up = fought.iter().filter(|e| e.day <= day).count() as u8;
                    let start = run.at(0).land_units[&u.id].experience;
                    assert!(
                        u.experience <= std::cmp::max(2, start + fought_up),
                        "{name} {:?} day {day}",
                        u.id
                    );
                }
            }
        }
        // A day counts for one thing only: at most one of training, a day of
        // fighting and rehearsal for each unit each day.
        let mut counts: BTreeMap<(u32, u32), u32> = BTreeMap::new();
        for e in &run.events {
            let who = match e.kind {
                EventKind::TrainingDay { unit, .. }
                | EventKind::FightDayCounted { unit, .. }
                | EventKind::RehearsalGained { unit, .. } => unit.raw(),
                _ => continue,
            };
            *counts.entry((e.day, who)).or_default() += 1;
        }
        assert!(counts.values().all(|c| *c == 1), "{name}: {counts:?}");
    }
    let run = Run::text(&variant);
    // In the variant: unit 2 fought on days 1 to 4, and so trained on none of
    // them (its hex was contested), and trains on day 5 onward.
    let training: Vec<u32> = run
        .all(|k| matches!(k, EventKind::TrainingDay { unit, .. } if unit.raw() == 2))
        .iter()
        .map(|e| e.day)
        .collect();
    assert_eq!(training.first(), Some(&5));
    assert!(training.iter().all(|d| *d >= 5));
    for day in 1..=4 {
        assert_eq!(
            run.on(
                day,
                |k| matches!(k, EventKind::FightDayCounted { unit, .. } if unit.raw() == 2)
            )
            .len(),
            1
        );
    }
    // It trains at experience 1 up to the ceiling of 2 and stops: 30 days at 1.
    assert_eq!(run.unit_at(62, 2).experience, 2);
    assert_eq!(run.unit_at(62, 2).training_days, 0);
}

#[test]
fn property_with_the_frozen_strength_of_0_experience_does_not_move_a_shot() {
    // The frozen figures as they are: strength 0. Training and fighting still
    // count (the other three figures are the same), but every shot is at the
    // side's chance, whatever the unit's experience, and the breakdown
    // names none of the new factors.
    let run = Run::of(common::load(&path()).expect("loads on the frozen figures"));
    let g = run.scenario.world.ref_data().ground.expect("figures");
    assert_eq!(g.experience_fire_strength, n(0));
    let altered = Run::text(&text());
    let mut shot = 0;
    for day in 1..=run.len() {
        for (fire, event) in run.fires(day) {
            shot += 1;
            let attacker = run.unit_at(day - 1, fire.unit).posture == Posture::Attack;
            assert_eq!(
                fire.chance,
                if attacker {
                    g.hit_chance
                } else {
                    g.defender_hit_chance
                },
                "unit {} day {day}",
                fire.unit
            );
            assert!(!has_factor(event, "experience_factor"));
            assert!(!has_factor(event, "side_chance"));
        }
    }
    assert!(shot >= 4, "at least the four fires of day 1: {shot}");
    // The first day's shots are the same men and shots as on the working
    // figures; only the chance differs.
    for raw in 2..=5 {
        let (a, b) = (run.fire(1, raw), altered.fire(1, raw));
        assert_eq!((a.firing_men, a.shots), (b.firing_men, b.shots));
    }
    // Training is not a switch: unit 1's table is the same.
    for day in [29, 30, 59, 60, 62] {
        assert_eq!(
            (
                run.unit_at(day, 1).training_days,
                run.unit_at(day, 1).experience
            ),
            (
                altered.unit_at(day, 1).training_days,
                altered.unit_at(day, 1).experience
            )
        );
    }
}

#[test]
fn property_the_enemys_view_of_a_fight_is_the_same_when_only_the_other_sides_experience_differs_and_the_hits_are_the_same(
) {
    // Unit 5 (Japanese, experience 4) in one world and at another experience
    // in the other. The Allied side's view of day 1 is the same where unit 5
    // scored the same hits. The same the other way round: unit 4 (Allied,
    // experience 0) and the Japanese view.
    let unit_5 = "men = 500\nq = 20\nr = 0\nposture = \"defend\"\nexperience = 4\n";
    let unit_4 = "men = 500\nq = 20\nr = 0\nposture = \"attack\"\nexperience = 0\n";
    let with_experience = |block: &str, experience: u32| {
        common::replace_once(
            &text(),
            block,
            &block.replace(
                &block[block.rfind("experience = ").expect("a key")..],
                &format!("experience = {experience}\n"),
            ),
        )
    };
    let base_run = Run::text(&text());
    let mut compared = 0;
    for (block, raw, viewer) in [(unit_5, 5, Side::Allied), (unit_4, 4, Side::Japanese)] {
        let hits = base_run.fire(1, raw).hits;
        for experience in 0..=4u32 {
            if i64::from(experience) == i64::from(base_run.unit_at(0, raw).experience) {
                continue;
            }
            let other = Run::text(&with_experience(block, experience));
            // The chance differs, so the hits may: compare the worlds where
            // they do not.
            assert_ne!(other.fire(1, raw).chance, base_run.fire(1, raw).chance);
            if other.fire(1, raw).hits != hits {
                continue;
            }
            compared += 1;
            assert_eq!(
                day_digest(&base_run.scenario, viewer, 1),
                day_digest(&other.scenario, viewer, 1),
                "unit {raw} at experience {experience}, seen by {viewer:?}"
            );
        }
    }
    assert!(
        compared >= 2,
        "at least two pairs of worlds had the same hits: {compared}"
    );
}

#[test]
fn property_a_side_sees_its_own_training_and_fighting_days_and_never_the_enemys() {
    let run = Run::text(&text());
    let seen = |side: Side| -> Vec<(u32, u32)> {
        run.scenario
            .world
            .side_view(side)
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                SideEventKind::Whole(EventKind::TrainingDay { unit, .. })
                | SideEventKind::Whole(EventKind::FightDayCounted { unit, .. })
                | SideEventKind::Whole(EventKind::ExperienceRaised { unit, .. }) => {
                    Some((e.day, unit.raw()))
                }
                _ => None,
            })
            .collect()
    };
    let units = |side: Side| -> std::collections::BTreeSet<u32> {
        seen(side).into_iter().map(|(_, u)| u).collect()
    };
    assert_eq!(units(Side::Allied), [1, 2, 4, 6].into_iter().collect());
    assert_eq!(units(Side::Japanese), [3].into_iter().collect());
    // Every such event of the log is seen by exactly its own side, whole.
    let in_log = run.all(is_new_event).len();
    assert_eq!(
        seen(Side::Allied).len() + seen(Side::Japanese).len(),
        in_log
    );
    // The own side's cut of a fire has the shots and not the chance or hits.
    for e in run.scenario.world.side_view(Side::Allied).events() {
        if let SideEventKind::GroundFire {
            firing_men, shots, ..
        } = e.kind
        {
            assert!(firing_men > 0 && !shots.is_zero());
            assert!(e.breakdown.as_ref().is_none_or(|b| !b
                .factors
                .iter()
                .any(|f| f.name == "side_chance" || f.name == "chance")));
        }
    }
}
