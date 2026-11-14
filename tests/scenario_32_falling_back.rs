//! Scenario 32: a beaten side falls back. Its units march out of the hex to
//! a hex with a base of their side, or else to one with no enemy fighting
//! men, tired by the defeat and carrying nothing; men with nowhere to go
//! surrender.
//!
//! Proves decision 0104 (a beaten side falls back) against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/32_falling_back.toml`. It runs on the frozen step 1
//! figures, which hold `beaten_fatigue` at the value below.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide (the
//! hits of the one fight), the calculation gives the shots and the chance
//! exactly and the rule for what follows; the tests read the hits from the
//! run. Everything else is asserted exactly. If the code disagrees, the
//! code or the rule is wrong: do not change a number here to make a test
//! pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! `beaten_fatigue` 30. From the earlier rules: `march_days_per_hex` 3,
//! `fatigue_per_march_day` 10, `rest_per_day` 5, `daily_supply_t_per_man`
//! 0.005, `unfed_loss_share_per_day` 0.05, `captured_stock_share` 0.5,
//! `beaten_share` 0.2, `men_per_shot` 100, `hit_chance` 0.3,
//! `defender_hit_chance` 0.6, `men_per_hit` 20, `work_per_man_day` 1.
//!
//! In the ground combat phase the marches come first, then the fights, the
//! fatigue of the day, the test of a side beaten and the bases that change
//! side. A unit set to fall back on day d therefore marches first on day
//! d + 1 and arrives in the phase of day d + 3. The fatigue of defeat is
//! added when the unit is set to fall back, after the day's fatigue step.
//!
//! ## Inputs
//!
//! A placeholder map. Every hex is a coast hex and clear. The game is 6
//! days.
//!
//! - Group one: P at (0,0), with land links to Q at (0,1) and to R at
//!   (1,0). In hex order Q comes before R.
//! - Group two: T at (10,0), with land links to U at (10,1) and to V at
//!   (11,0). In hex order U comes before V.
//! - S at (20,0), with no land link.
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Japanese | P   | 1000 t |
//! | 2    | Japanese | R   | 100 t  |
//! | 3    | Allied   | P   | 1000 t |
//! | 4    | Japanese | T   | 1000 t |
//! | 5    | Allied   | T   | 5000 t |
//! | 6    | Allied   | U   | 1000 t |
//! | 7    | Japanese | S   | 1000 t |
//! | 8    | Allied   | S   | 1000 t |
//!
//! Every base has an airfield.
//!
//! | Unit | Side     | Kind     | Men  | Hex | Posture |
//! | ---- | -------- | -------- | ---- | --- | ------- |
//! | 1    | Japanese | working  | 300  | P   | defend  |
//! | 2    | Allied   | fighting | 500  | P   | attack  |
//! | 3    | Japanese | fighting | 1000 | T   | defend  |
//! | 4    | Allied   | fighting | 6000 | T   | attack  |
//! | 5    | Allied   | fighting | 100  | U   | defend  |
//! | 6    | Japanese | working  | 200  | S   | defend  |
//! | 7    | Allied   | fighting | 100  | S   | attack  |
//!
//! The Allied side has a `men_lost` objective that counts Japanese men
//! lost.
//!
//! Orders: on day 2, units 2 and 4 are set to defend.
//!
//! ## At P: falling back to a base, with no fight
//!
//! Night of day 1: base 1 pays 300 x 0.005 = 1.5 t and holds 998.5 t. Base
//! 3 pays 2.5 t.
//!
//! Day 1. The Japanese side has no fighting man at P, so the hex is not
//! contested. Unit 2 is on attack and the Japanese side has a base there:
//! it is beaten without a fight. Base 1 changes side and keeps 998.5 x 0.5
//! = 499.25 t.
//!
//! Unit 1 falls back. Q comes first in hex order and holds nothing; R holds
//! base 2, a base of its side, and a base is preferred: it marches for R.
//! Its fatigue is 0 after the day's rest, and the defeat adds 30.
//!
//! | Day | Night: fed          | Men lost unfed    | Men  | In the phase            | Fatigue after |
//! | --- | ------------------- | ----------------- | ---- | ----------------------- | ------------- |
//! | 1   | by base 1, in full  | 0                 | 300  | beaten, set to fall back | 30           |
//! | 2   | unfed: no base at P | 300 x 0.05 = 15   | 285  | marches, day 1 of 3     | 40            |
//! | 3   | unfed               | 14.25: 14         | 271  | marches, day 2 of 3     | 50            |
//! | 4   | unfed               | 13.55: 13         | 258  | marches, day 3: arrives at R | 60       |
//! | 5   | by base 2, in full  | 0                 | 258  | works                   | 55            |
//! | 6   | by base 2, in full  | 0                 | 258  | works                   | 50            |
//!
//! - It carries nothing to R: its side has no base at P to draw from.
//! - Base 2 pays 258 x 0.005 = 1.29 t on each of the nights of days 5 and
//!   6, and holds 97.42 t at the end.
//! - At R on days 5 and 6 unit 1, with no order at a base that has an
//!   airfield, works the port: 258 of work each day.
//! - On days 2 to 4 it makes no work: it marches.
//! - Base 1, now Allied, and base 3 feed unit 2. Base 1 is the lower
//!   numbered and pays 2.5 t on each of the nights of days 2 to 6: it holds
//!   499.25 - 12.5 = 486.75 t at the end. Base 3 holds 997.5 t.
//!
//! ## At T: falling back after a fight, away from the enemy
//!
//! Day 1, the fight. The Allied side attacks.
//!
//! - Unit 4: 6000 firing men, 60 shots at 0.3.
//! - Unit 3: 1000 firing men, 10 shots at 0.6.
//!
//! The Japanese side is beaten whatever the dice give: unit 3 has at most
//! 1000 men, and unit 4 has at least 6000 - 10 x 20 = 5800, of which 0.2 is
//! 1160. Base 4 changes side.
//!
//! Let m be unit 3's men after the fight, read from the run.
//!
//! - If m is 0 the unit is destroyed and nothing falls back.
//! - If m is above 0, unit 3 falls back. U comes first in hex order and
//!   holds Allied fighting men: it is passed over. V holds no one: unit 3
//!   marches for V. Its fatigue is 0 after the day's rest and the defeat
//!   adds 30. It goes on the defence, as it was.
//! - On day 2 unit 4 is set to defend, so there is no fight on days 2 to 4
//!   while unit 3 is still at T.
//! - On each of the nights of days 2, 3 and 4 it is unfed at T and loses
//!   its men times 0.05, cut to a whole man and one man at least. On the
//!   nights of days 5 and 6 it is unfed at V, where it has no base and
//!   brought nothing, and loses the same share. The tests work each night's
//!   loss from the men the run has before it.
//! - Its fatigue is 40, 50 and 60 after days 2, 3 and 4. It arrives at V in
//!   the phase of day 4 and forms no base there. Its fatigue is 55 and 50
//!   after days 5 and 6.
//!
//! ## At S: nowhere to go
//!
//! Day 1. The Japanese side has no fighting man at S. Unit 7 is on attack:
//! the Japanese side is beaten without a fight and base 7 changes side.
//!
//! S has no land link. Unit 6 surrenders in the same phase: its 200 men are
//! lost and the unit is destroyed. Both sides are told, with the 200 men.
//! The men the Allied `men_lost` objective counts rise by 200.
//!
//! # Properties
//!
//! - Men are conserved: a unit's men before it falls back equal its men on
//!   arrival plus those lost on the way, each loss with its event.
//! - A unit that falls back never arrives in a hex that held enemy fighting
//!   men when it set out, unless that hex held a base of its side.
//! - A march order given to a unit on the day after it is set to fall back
//!   replaces the falling back, and an attack order to it is taken.
//! - A scenario in which no side is beaten in a hex with a land link, and
//!   no beaten side is left with men and nowhere to go, gives what it gave
//!   before, apart from the hash version. A scenario that does differ is
//!   listed for the architect before its baseline is remade.
//! - The enemy's view is the same in two worlds that differ only in where
//!   the beaten side's units fall back to.

mod common;

use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind, LandFatigueChange, SupplyUse};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, LandUnitId};
use sim_core::objectives::tallies;
use sim_core::orders::{Job, Posture};
use sim_core::side_events::SideEventKind;
use sim_core::state::{LandUnit, Side, TruthState};
use sim_data::Scenario;

// ------------------------------------------------------------------ Helpers

const P: Hex = Hex::new(0, 0);
const Q: Hex = Hex::new(0, 1);
const R: Hex = Hex::new(1, 0);
const T: Hex = Hex::new(10, 0);
const U: Hex = Hex::new(10, 1);
const V: Hex = Hex::new(11, 0);
const S: Hex = Hex::new(20, 0);

fn path() -> PathBuf {
    common::data_dir().join("scenarios/32_falling_back.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// The scenario on the frozen step 1 figures, which hold `beaten_fatigue` at
/// 30 as the calculation says.
fn from(source: &str) -> Scenario {
    common::from_text(source, &path()).expect("the variant loads on the frozen figures")
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn unit(raw: u32) -> LandUnitId {
    LandUnitId::new(raw)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

/// A run of a scenario: the truth at the start (day 0) and at the end of
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

    fn live() -> Run {
        Run::text(&text())
    }

    fn at(&self, day: u32) -> &TruthState {
        &self.days[day as usize]
    }

    fn unit_at(&self, day: u32, raw: u32) -> &LandUnit {
        &self.at(day).land_units[&unit(raw)]
    }

    fn supply_at(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).bases[&base(raw)].supply_t
    }

    /// The events of `day` that satisfy `pick`.
    fn on(&self, day: u32, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.day == day && pick(&e.kind))
            .collect()
    }

    fn all(&self, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events.iter().filter(|e| pick(&e.kind)).collect()
    }

    fn only(&self, pick: impl Fn(&EventKind) -> bool) -> &Event {
        let found = self.all(pick);
        assert_eq!(found.len(), 1, "exactly one such event: {found:?}");
        found[0]
    }

    fn index_of(&self, event: &Event) -> usize {
        self.events
            .iter()
            .position(|e| e.id == event.id)
            .expect("the event is in the log")
    }

    /// The `SideBeatenAshore` event of a side at a hex.
    fn beaten(&self, side: Side, hex: Hex) -> &Event {
        self.only(|k| {
            matches!(k, EventKind::SideBeatenAshore { side: s, hex: h, .. } if *s == side && *h == hex)
        })
    }

    fn fell_back(&self, raw: u32) -> Option<&Event> {
        let found =
            self.all(|k| matches!(k, EventKind::UnitFellBack { unit, .. } if unit.raw() == raw));
        assert!(found.len() <= 1, "a unit falls back once here");
        found.first().copied()
    }

    /// What a unit lost unfed on the night of `day`.
    fn lost_unfed(&self, day: u32, raw: u32) -> Option<(u32, u32)> {
        let found = self.on(
            day,
            |k| matches!(k, EventKind::MenLostUnfed { unit, .. } if unit.raw() == raw),
        );
        assert!(found.len() <= 1);
        found.first().map(|e| match e.kind {
            EventKind::MenLostUnfed { lost, ashore, .. } => (lost, ashore),
            _ => unreachable!(),
        })
    }

    /// A base's supply after the night's feeding of `day`, and what it paid.
    fn night_paid(&self, day: u32, raw: u32) -> Option<(Fixed, Fixed)> {
        let found = self.on(day, |k| {
            matches!(
                k,
                EventKind::BaseSupplySpent { base: b, reason: SupplyUse::Men, .. } if b.raw() == raw
            )
        });
        assert!(found.len() <= 1);
        found.first().map(|e| match e.kind {
            EventKind::BaseSupplySpent { paid_t, left_t, .. } => (paid_t, left_t),
            _ => unreachable!(),
        })
    }

    /// The day-by-day fatigue events of a unit on `day`.
    fn fatigue(&self, day: u32, raw: u32) -> Vec<(LandFatigueChange, Fixed)> {
        self.on(
            day,
            |k| matches!(k, EventKind::LandUnitFatigue { unit, .. } if unit.raw() == raw),
        )
        .into_iter()
        .map(|e| match e.kind {
            EventKind::LandUnitFatigue {
                change, fatigue, ..
            } => (change, fatigue),
            _ => unreachable!(),
        })
        .collect()
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

    fn marched_days(&self, raw: u32) -> Vec<(u32, u32, Cause)> {
        self.all(|k| matches!(k, EventKind::MarchDay { unit, .. } if unit.raw() == raw))
            .into_iter()
            .map(|e| match e.kind {
                EventKind::MarchDay { days, of, .. } => (days, of, e.causes[0]),
                _ => unreachable!(),
            })
            .collect()
    }

    /// The last `MarchDay` event of a unit.
    fn last_march_day(&self, raw: u32) -> &Event {
        self.all(|k| matches!(k, EventKind::MarchDay { unit, .. } if unit.raw() == raw))
            .last()
            .copied()
            .expect("it marched")
    }

    /// `MarchArrived` of a unit: the event, its destination and men.
    fn arrived(&self, raw: u32) -> Option<(&Event, Hex, u32)> {
        let found =
            self.all(|k| matches!(k, EventKind::MarchArrived { unit, .. } if unit.raw() == raw));
        assert!(found.len() <= 1);
        found.first().map(|e| match e.kind {
            EventKind::MarchArrived { to, men, .. } => (*e, to, men),
            _ => unreachable!(),
        })
    }
}

/// The scenario text with `extra` appended: more orders or units.
fn with(source: &str, extra: &str) -> String {
    format!("{source}\n{extra}")
}

fn with_seed(source: &str, seed: u32) -> String {
    common::replace_once(source, "seed = 1942\n", &format!("seed = {seed}\n"))
}

fn order(day: u32, side: &str, unit: u32, body: &str) -> String {
    format!("\n[[order]]\nday = {day}\nside = \"{side}\"\nland_unit = {unit}\n{body}\n")
}

/// A fighting Allied unit on defence at a hex, as scenario text.
fn allied_unit_at(id: u32, hex: Hex) -> String {
    format!(
        "\n[[land_unit]]\nid = {id}\nname = \"Placeholder Allied unit {id}\"\nside = \"allied\"\nmen = 100\nq = {}\nr = {}\nposture = \"defend\"\n",
        hex.q, hex.r
    )
}

/// The scenario with base 2 (the Japanese base at R) taken out.
fn without_base_2(source: &str) -> String {
    let start = source.find("[[base]]\nid = 2\n").expect("base 2");
    let end = source[start + 1..].find("[[base]]").expect("base 3") + start + 1;
    format!("{}{}", &source[..start], &source[end..])
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_figures_and_the_inputs_are_the_calculations() {
    let run = Run::live();
    let figures = run.scenario.world.ref_data().ground.expect("ground");
    assert_eq!(figures.beaten_fatigue, n(30));
    assert_eq!(figures.march_days_per_hex, 3);
    assert_eq!(figures.fatigue_per_march_day, n(10));
    assert_eq!(figures.rest_per_day, n(5));
    assert_eq!(figures.daily_supply_t_per_man, fx("0.005"));
    assert_eq!(figures.unfed_loss_share_per_day, fx("0.05"));
    assert_eq!(figures.captured_stock_share, fx("0.5"));
    assert_eq!(figures.beaten_share, fx("0.2"));
    assert_eq!(figures.men_per_shot, n(100));
    assert_eq!(figures.hit_chance, fx("0.3"));
    assert_eq!(figures.defender_hit_chance, fx("0.6"));
    assert_eq!(figures.men_per_hit, n(20));
    assert_eq!(figures.work_per_man_day, n(1));

    let start = run.at(0);
    assert_eq!(start.land_units.len(), 7);
    assert_eq!(start.bases.len(), 8);
    let bases = [
        (1, Side::Japanese, P, n(1000)),
        (2, Side::Japanese, R, n(100)),
        (3, Side::Allied, P, n(1000)),
        (4, Side::Japanese, T, n(1000)),
        (5, Side::Allied, T, n(5000)),
        (6, Side::Allied, U, n(1000)),
        (7, Side::Japanese, S, n(1000)),
        (8, Side::Allied, S, n(1000)),
    ];
    for (id, side, hex, supply) in bases {
        let b = &start.bases[&base(id)];
        assert_eq!(
            (b.side, b.hex, b.supply_t),
            (side, hex, supply),
            "base {id}"
        );
        assert!(b.airfield, "base {id} has an airfield");
    }
    let units = [
        (1, Side::Japanese, true, 300, P, Posture::Defend),
        (2, Side::Allied, false, 500, P, Posture::Attack),
        (3, Side::Japanese, false, 1000, T, Posture::Defend),
        (4, Side::Allied, false, 6000, T, Posture::Attack),
        (5, Side::Allied, false, 100, U, Posture::Defend),
        (6, Side::Japanese, true, 200, S, Posture::Defend),
        (7, Side::Allied, false, 100, S, Posture::Attack),
    ];
    for (id, side, working, men, hex, posture) in units {
        let u = run.unit_at(0, id);
        assert_eq!(
            (u.side, u.working, u.men_ashore, u.hex, u.posture),
            (side, working, men, Some(hex), posture),
            "unit {id}"
        );
    }
    // The hex order the calculation leans on.
    assert!(Q < R);
    assert!(U < V);
    let map = run.scenario.world.map();
    assert_eq!(map.land_neighbours(P), vec![Q, R]);
    assert_eq!(map.land_neighbours(T), vec![U, V]);
    assert!(map.land_neighbours(S).is_empty());
    // The Allied objective counts Japanese men lost.
    assert_eq!(run.scenario.days, 6);
}

#[test]
fn p_base_1_holds_998_5_then_changes_side_with_499_25() {
    let run = Run::live();
    assert_eq!(run.night_paid(1, 1), Some((fx("1.5"), fx("998.5"))));
    assert_eq!(run.night_paid(1, 3), Some((fx("2.5"), fx("997.5"))));
    let beaten = run.beaten(Side::Japanese, P);
    match beaten.kind {
        EventKind::SideBeatenAshore {
            men,
            enemy_men,
            fought,
            ..
        } => assert_eq!((men, enemy_men, fought), (0, 500, false)),
        _ => unreachable!(),
    }
    // No fight at P.
    assert!(run
        .all(|k| matches!(k, EventKind::GroundFight { hex, .. } if *hex == P))
        .is_empty());
    let changed =
        run.only(|k| matches!(k, EventKind::BaseChangedSide { base, .. } if base.raw() == 1));
    match changed.kind {
        EventKind::BaseChangedSide {
            hex,
            from,
            to,
            supply_t,
            supply_lost_t,
            ..
        } => {
            assert_eq!((hex, from, to), (P, Side::Japanese, Side::Allied));
            assert_eq!(supply_t, fx("499.25"));
            assert_eq!(supply_lost_t, fx("499.25"));
        }
        _ => unreachable!(),
    }
    assert!(changed.causes.contains(&Cause::Event(beaten.id)));
    assert_eq!(run.at(1).bases[&base(1)].side, Side::Allied);
    assert_eq!(run.supply_at(1, 1), fx("499.25"));
}

#[test]
fn p_unit_1_falls_back_to_r_and_not_q_tired_by_the_defeat() {
    let run = Run::live();
    let beaten = run.beaten(Side::Japanese, P);
    let fell = run.fell_back(1).expect("unit 1 falls back");
    // In the same phase as the test, after the base changed side.
    assert_eq!(fell.day, 1);
    assert_eq!(fell.pulse, None);
    assert_eq!(fell.causes, vec![Cause::Event(beaten.id)]);
    let changed =
        run.only(|k| matches!(k, EventKind::BaseChangedSide { base, .. } if base.raw() == 1));
    assert!(run.index_of(changed) < run.index_of(fell));
    assert!(run.index_of(beaten) < run.index_of(changed));
    assert_eq!(
        fell.kind,
        EventKind::UnitFellBack {
            unit: unit(1),
            side: Side::Japanese,
            from: P,
            to: R,
            men: 300,
            fatigue: n(30),
        }
    );
    // The fatigue of the defeat is on the event, with its breakdown.
    let factors: Vec<(&str, Fixed)> = fell
        .breakdown
        .as_ref()
        .expect("a breakdown")
        .factors
        .iter()
        .map(|f| (f.name, f.value))
        .collect();
    assert_eq!(
        factors,
        vec![("fatigue_before", n(0)), ("beaten_fatigue", n(30))]
    );
    // It is set to march for R and has marched no day.
    let u = run.unit_at(1, 1);
    let march = u.march.as_ref().expect("a march");
    assert_eq!((march.to, march.days), (R, 0));
    assert_eq!(u.posture, Posture::Defend);
    assert_eq!(u.hex, Some(P));
    assert_eq!(u.men_ashore, 300);
}

#[test]
fn p_unit_1s_men_fatigue_and_march_go_day_by_day_as_the_table_says() {
    let run = Run::live();
    let fatigue = [30, 40, 50, 60, 55, 50];
    let men = [300, 285, 271, 258, 258, 258];
    for day in 1..=6u32 {
        let u = run.unit_at(day, 1);
        assert_eq!(
            u.fatigue,
            n(fatigue[day as usize - 1]),
            "fatigue, day {day}"
        );
        assert_eq!(u.men_ashore, men[day as usize - 1], "men, day {day}");
    }
    // Each loss with its event.
    assert_eq!(run.lost_unfed(1, 1), None);
    assert_eq!(run.lost_unfed(2, 1), Some((15, 285)));
    assert_eq!(run.lost_unfed(3, 1), Some((14, 271)));
    assert_eq!(run.lost_unfed(4, 1), Some((13, 258)));
    assert_eq!(run.lost_unfed(5, 1), None);
    assert_eq!(run.lost_unfed(6, 1), None);
    // The night: unfed at P with no base of its side, on days 2 to 4.
    for day in 2..=4 {
        let fed = run.on(
            day,
            |k| matches!(k, EventKind::MenFed { side: Side::Japanese, hex, .. } if *hex == P),
        );
        assert_eq!(fed.len(), 1, "day {day}");
        match fed[0].kind {
            EventKind::MenFed {
                base, fed, unfed, ..
            } => {
                assert_eq!((base, fed), (None, 0));
                assert_eq!(unfed, men[day as usize - 2]);
            }
            _ => unreachable!(),
        }
    }
    // The marching days: day 1 of 3, 2 and 3, each caused by the falling back.
    let fell = run.fell_back(1).expect("falls back");
    assert_eq!(
        run.marched_days(1),
        vec![
            (1, 3, Cause::Event(fell.id)),
            (2, 3, Cause::Event(fell.id)),
            (3, 3, Cause::Event(fell.id)),
        ]
    );
    for day in 2..=4 {
        assert_eq!(
            run.on(
                day,
                |k| matches!(k, EventKind::MarchDay { unit, .. } if unit.raw() == 1)
            )
            .len(),
            1
        );
    }
    // Where it is: at P until the phase of day 4.
    for day in 1..=3 {
        assert_eq!(run.unit_at(day, 1).hex, Some(P), "day {day}");
        assert!(run.unit_at(day, 1).march.is_some());
    }
    for day in 4..=6 {
        assert_eq!(run.unit_at(day, 1).hex, Some(R), "day {day}");
        assert!(run.unit_at(day, 1).march.is_none());
    }
    // The fatigue events: the day's rest, then the day's march.
    assert_eq!(run.fatigue(2, 1), vec![(LandFatigueChange::Marched, n(40))]);
    assert_eq!(run.fatigue(3, 1), vec![(LandFatigueChange::Marched, n(50))]);
    assert_eq!(run.fatigue(4, 1), vec![(LandFatigueChange::Marched, n(60))]);
    assert_eq!(run.fatigue(5, 1), vec![(LandFatigueChange::Rested, n(55))]);
    assert_eq!(run.fatigue(6, 1), vec![(LandFatigueChange::Rested, n(50))]);
}

#[test]
fn p_unit_1_arrives_at_r_in_the_phase_of_day_4_carrying_nothing_and_forms_no_base() {
    let run = Run::live();
    let fell = run.fell_back(1).expect("falls back");
    let (arrived, to, men) = run.arrived(1).expect("arrives");
    assert_eq!(arrived.day, 4);
    assert_eq!(arrived.pulse, None);
    assert_eq!((to, men), (R, 258));
    // The arrival cites the day's march, which cites the falling back.
    let last_day = run.last_march_day(1);
    assert_eq!(arrived.causes, vec![Cause::Event(last_day.id)]);
    assert_eq!(last_day.causes, vec![Cause::Event(fell.id)]);
    match arrived.kind {
        EventKind::MarchArrived {
            from,
            supply_t,
            base,
            ..
        } => {
            assert_eq!(from, P);
            assert_eq!(supply_t, n(0));
            assert_eq!(base, None);
        }
        _ => unreachable!(),
    }
    // No base is formed, by either unit that fell back; base 1's stock and
    // base 2's are untouched by a march.
    assert!(run
        .all(|k| matches!(k, EventKind::MarchBaseFormed { .. }))
        .is_empty());
    assert_eq!(run.at(6).bases.len(), 8);
    assert!(run
        .all(|k| matches!(
            k,
            EventKind::BaseSupplySpent {
                reason: SupplyUse::March,
                ..
            }
        ))
        .is_empty());
    // Base 2 is where it was and holds what it held until the nights of
    // days 5 and 6.
    for day in 1..=4 {
        assert_eq!(run.supply_at(day, 2), n(100), "day {day}");
        assert_eq!(run.night_paid(day, 2), None, "day {day}");
    }
}

#[test]
fn p_base_2_pays_1_29_t_on_the_nights_of_days_5_and_6_and_holds_97_42() {
    let run = Run::live();
    assert_eq!(run.night_paid(5, 2), Some((fx("1.29"), fx("98.71"))));
    assert_eq!(run.night_paid(6, 2), Some((fx("1.29"), fx("97.42"))));
    assert_eq!(run.supply_at(6, 2), fx("97.42"));
    for day in [5, 6] {
        let fed = run.on(
            day,
            |k| matches!(k, EventKind::MenFed { side: Side::Japanese, hex, .. } if *hex == R),
        );
        assert_eq!(fed.len(), 1);
        match fed[0].kind {
            EventKind::MenFed {
                base,
                men,
                fed,
                unfed,
                ..
            } => assert_eq!((base, men, fed, unfed), (Some(base_id(2)), 258, 258, 0)),
            _ => unreachable!(),
        }
    }
}

fn base_id(raw: u32) -> BaseId {
    base(raw)
}

#[test]
fn p_unit_1_works_the_port_258_on_days_5_and_6_and_makes_no_work_on_days_2_to_4() {
    let run = Run::live();
    for day in 1..=4 {
        assert!(run.worked(day, 1).is_empty(), "day {day}");
    }
    assert_eq!(run.worked(5, 1), vec![(Job::Port, 2, n(258), n(258))]);
    assert_eq!(run.worked(6, 1), vec![(Job::Port, 2, n(258), n(258))]);
}

#[test]
fn p_base_1_pays_2_5_a_night_for_unit_2_and_ends_with_486_75_and_base_3_with_997_5() {
    let run = Run::live();
    for day in 2..=6 {
        let paid = run.night_paid(day, 1).expect("base 1 feeds unit 2");
        assert_eq!(paid.0, fx("2.5"), "day {day}");
        assert_eq!(
            run.night_paid(day, 3),
            None,
            "base 3 pays nothing on day {day}"
        );
    }
    assert_eq!(run.supply_at(6, 1), fx("486.75"));
    assert_eq!(run.supply_at(6, 3), fx("997.5"));
    // Unit 2 loses nothing and is fed on every night.
    for day in 1..=6 {
        assert_eq!(run.unit_at(day, 2).men_ashore, 500);
        assert_eq!(run.lost_unfed(day, 2), None);
    }
}

// --------------------------------------------------------------------- At T

/// What the fight at T on day 1 gave: the hits unit 4 and unit 3 scored.
fn fight_at_t(run: &Run) -> (u32, u32) {
    let fires = run.on(
        1,
        |k| matches!(k, EventKind::GroundFire { hex, .. } if *hex == T),
    );
    assert_eq!(fires.len(), 2);
    let mut hits = (None, None);
    for e in fires {
        match e.kind {
            EventKind::GroundFire {
                unit: u,
                firing_men,
                shots,
                chance,
                hits: h,
                ..
            } => match u.raw() {
                4 => {
                    assert_eq!((firing_men, shots, chance), (6000, n(60), fx("0.3")));
                    hits.0 = Some(h);
                }
                3 => {
                    assert_eq!((firing_men, shots, chance), (1000, n(10), fx("0.6")));
                    hits.1 = Some(h);
                }
                other => panic!("unit {other} fired at T"),
            },
            _ => unreachable!(),
        }
    }
    (hits.0.expect("unit 4 fired"), hits.1.expect("unit 3 fired"))
}

/// Unit 3's men after the fight on day 1, as the run has them.
fn m_of(run: &Run) -> u32 {
    let (hits_by_4, _) = fight_at_t(run);
    1000u32.saturating_sub(20 * hits_by_4)
}

/// The assertions of the calculation at T for one run of the scenario.
fn check_t(run: &Run) -> (u32, Vec<u32>) {
    let (hits_by_4, hits_by_3) = fight_at_t(run);
    assert!(hits_by_4 <= 60 && hits_by_3 <= 10);
    let m = m_of(run);
    // Unit 4 lost 20 men a hit of unit 3's.
    let lost_4 = run.on(
        1,
        |k| matches!(k, EventKind::MenLostInFight { unit, .. } if unit.raw() == 4),
    );
    let expected_lost_4 = 20 * hits_by_3;
    if expected_lost_4 > 0 {
        assert_eq!(lost_4.len(), 1);
        match lost_4[0].kind {
            EventKind::MenLostInFight { lost, ashore, .. } => {
                assert_eq!((lost, ashore), (expected_lost_4, 6000 - expected_lost_4))
            }
            _ => unreachable!(),
        }
    }
    // The Japanese side is beaten whatever the dice give, and base 4 changes side.
    let beaten = run.beaten(Side::Japanese, T);
    match beaten.kind {
        EventKind::SideBeatenAshore {
            men,
            enemy_men,
            fought,
            ..
        } => {
            assert!(fought);
            assert_eq!(men, m);
            assert_eq!(enemy_men, 6000 - expected_lost_4);
            assert!(enemy_men >= 5800);
            // 0.2 of the enemy's men is at least 1160: unit 3 never has more.
            assert!(m < 1160);
        }
        _ => unreachable!(),
    }
    let changed =
        run.only(|k| matches!(k, EventKind::BaseChangedSide { base, .. } if base.raw() == 4));
    match changed.kind {
        EventKind::BaseChangedSide {
            from, to, supply_t, ..
        } => {
            assert_eq!((from, to), (Side::Japanese, Side::Allied));
            assert_eq!(supply_t, fx("497.5"));
        }
        _ => unreachable!(),
    }
    assert_eq!(run.at(1).bases[&base(4)].side, Side::Allied);

    if m == 0 {
        // Destroyed: nothing falls back.
        assert!(run.fell_back(3).is_none());
        assert!(
            run.on(
                1,
                |k| matches!(k, EventKind::LandUnitDestroyed { unit } if unit.raw() == 3)
            )
            .len()
                == 1
        );
        assert_eq!(run.unit_at(1, 3).men_ashore, 0);
        return (m, vec![0]);
    }

    // Falls back to V, passing U, tired by the defeat.
    let fell = run.fell_back(3).expect("unit 3 falls back");
    assert_eq!(fell.day, 1);
    assert_eq!(fell.causes, vec![Cause::Event(beaten.id)]);
    assert!(run.index_of(changed) < run.index_of(fell));
    assert_eq!(
        fell.kind,
        EventKind::UnitFellBack {
            unit: unit(3),
            side: Side::Japanese,
            from: T,
            to: V,
            men: m,
            fatigue: n(30),
        }
    );
    // No fight at T on days 2 to 4, with unit 4 on the defence.
    for day in 2..=4 {
        assert!(
            run.on(day, |k| matches!(k, EventKind::GroundFight { .. }))
                .is_empty(),
            "no fight on day {day}"
        );
        assert_eq!(run.unit_at(day, 4).posture, Posture::Defend);
    }
    // The nights: unfed, each loss its men times 0.05 cut, one at least.
    let mut men = m;
    let fatigue = [30, 40, 50, 60, 55, 50];
    let mut trail = vec![m];
    assert_eq!(run.unit_at(1, 3).men_ashore, m);
    assert_eq!(run.unit_at(1, 3).fatigue, n(fatigue[0]));
    for day in 2..=6u32 {
        let loss = ((men * 5) / 100).max(1);
        let expected = men.saturating_sub(loss);
        if men > 0 {
            assert_eq!(
                run.lost_unfed(day, 3),
                Some((loss.min(men), expected)),
                "day {day}: unit 3 with {men} men"
            );
        }
        men = expected;
        trail.push(men);
        assert_eq!(run.unit_at(day, 3).men_ashore, men, "men, day {day}");
        if men > 0 {
            assert_eq!(
                run.unit_at(day, 3).fatigue,
                n(fatigue[day as usize - 1]),
                "fatigue, day {day}"
            );
        }
        let expected_hex = if day < 4 { T } else { V };
        if men > 0 {
            assert_eq!(
                run.unit_at(day, 3).hex,
                Some(expected_hex),
                "hex, day {day}"
            );
        }
    }
    if men > 0 || run.unit_at(4, 3).men_ashore > 0 {
        // It arrives at V in the phase of day 4 and forms no base there.
        let (arrived, to, arrived_men) = run.arrived(3).expect("arrives");
        assert_eq!((arrived.day, arrived.pulse, to), (4, None, V));
        assert_eq!(arrived_men, trail[3]);
        let last_day = run.last_march_day(3);
        assert_eq!(arrived.causes, vec![Cause::Event(last_day.id)]);
        assert_eq!(last_day.causes, vec![Cause::Event(fell.id)]);
        match arrived.kind {
            EventKind::MarchArrived { supply_t, base, .. } => {
                assert_eq!((supply_t, base), (n(0), None))
            }
            _ => unreachable!(),
        }
    }
    assert!(run
        .all(|k| matches!(k, EventKind::MarchBaseFormed { .. }))
        .is_empty());
    assert_eq!(run.at(6).bases.len(), 8);
    // At V it is unfed, on the nights of days 5 and 6.
    for day in 5..=6 {
        if trail[day as usize - 1] > 0 {
            let fed = run.on(
                day,
                |k| matches!(k, EventKind::MenFed { side: Side::Japanese, hex, .. } if *hex == V),
            );
            assert_eq!(fed.len(), 1, "day {day}");
            match fed[0].kind {
                EventKind::MenFed { base, fed, .. } => assert_eq!((base, fed), (None, 0)),
                _ => unreachable!(),
            }
        }
    }
    // The Allied unit at U was passed over and was not touched.
    assert_eq!(run.unit_at(6, 5).men_ashore, 100);
    (m, trail)
}

#[test]
fn t_the_fight_is_60_shots_at_0_3_against_10_at_0_6_and_unit_3_falls_back_to_v_passing_u() {
    let run = Run::live();
    let (a, j) = fight_at_t(&run);
    eprintln!("the fight at T on day 1: unit 4 scored {a} hits, unit 3 scored {j} hits");
    let (m, trail) = check_t(&run);
    eprintln!(
        "unit 3's men after the fight: {m}; at the end of days 2 to 6: {:?}",
        &trail[1..]
    );
}

#[test]
fn t_the_calculation_holds_for_a_hundred_seeds() {
    let mut destroyed = 0;
    let mut survived = 0;
    for seed in 1..=100 {
        let run = Run::text(&with_seed(&text(), seed));
        if check_t(&run).0 == 0 {
            destroyed += 1;
        } else {
            survived += 1;
        }
    }
    eprintln!("100 seeds: unit 3 survived the fight in {survived}, was destroyed in {destroyed}");
    assert!(survived > 0);
}

#[test]
fn t_a_unit_3_the_fight_destroys_is_not_set_to_fall_back() {
    // 100 men in place of 1000: the 60 shots at 0.3 leave it none.
    let source = common::replace_once(
        &text(),
        "men = 1000\nq = 10\nr = 0\nposture = \"defend\"",
        "men = 100\nq = 10\nr = 0\nposture = \"defend\"",
    );
    let run = Run::text(&source);
    assert_eq!(run.unit_at(1, 3).men_ashore, 0);
    assert!(run.fell_back(3).is_none());
    assert!(run.arrived(3).is_none());
    assert_eq!(
        run.on(
            1,
            |k| matches!(k, EventKind::LandUnitDestroyed { unit } if unit.raw() == 3)
        )
        .len(),
        1
    );
    assert_eq!(run.at(1).bases[&base(4)].side, Side::Allied);
    assert!(run.unit_at(1, 3).march.is_none());
}

// --------------------------------------------------------------------- At S

#[test]
fn s_base_7_changes_side_and_unit_6_surrenders_with_200_men_in_the_same_phase() {
    let run = Run::live();
    let beaten = run.beaten(Side::Japanese, S);
    assert_eq!(beaten.day, 1);
    let changed =
        run.only(|k| matches!(k, EventKind::BaseChangedSide { base, .. } if base.raw() == 7));
    assert_eq!(run.at(1).bases[&base(7)].side, Side::Allied);
    let surrendered = run.only(|k| matches!(k, EventKind::UnitSurrendered { .. }));
    assert_eq!(
        surrendered.kind,
        EventKind::UnitSurrendered {
            unit: unit(6),
            side: Side::Japanese,
            hex: S,
            men: 200,
        }
    );
    assert_eq!(
        (surrendered.day, surrendered.pulse),
        (beaten.day, beaten.pulse)
    );
    // One event tells the hex's surrender, citing the test that beat the
    // side; the unit's own surrender cites it.
    let together = run.only(|k| matches!(k, EventKind::MenSurrenderedAshore { .. }));
    assert_eq!(
        together.kind,
        EventKind::MenSurrenderedAshore {
            side: Side::Japanese,
            hex: S,
            men: 200,
        }
    );
    assert_eq!(together.causes, vec![Cause::Event(beaten.id)]);
    assert_eq!(surrendered.causes, vec![Cause::Event(together.id)]);
    assert!(run.index_of(changed) < run.index_of(together));
    assert!(run.index_of(together) < run.index_of(surrendered));
    // Destroyed, with no men aboard.
    let destroyed =
        run.only(|k| matches!(k, EventKind::LandUnitDestroyed { unit } if unit.raw() == 6));
    assert_eq!(destroyed.day, 1);
    assert!(run.index_of(surrendered) < run.index_of(destroyed));
    let u = run.unit_at(1, 6);
    assert_eq!((u.men_ashore, u.hex), (0, None));
    // It did not fall back and did not march, and lost nothing unfed.
    assert!(run.fell_back(6).is_none());
    assert!(run.marched_days(6).is_empty());
    assert_eq!(run.lost_unfed(2, 6), None);
}

#[test]
fn s_both_sides_are_told_of_the_surrender_with_200_men() {
    let run = Run::live();
    let japanese = run.scenario.world.side_view(Side::Japanese);
    let seen: Vec<_> = japanese
        .events()
        .into_iter()
        .filter(|e| {
            matches!(
                &e.kind,
                SideEventKind::Whole(EventKind::UnitSurrendered { .. })
            )
        })
        .collect();
    assert_eq!(seen.len(), 1);
    assert_eq!(
        seen[0].kind,
        SideEventKind::Whole(EventKind::UnitSurrendered {
            unit: unit(6),
            side: Side::Japanese,
            hex: S,
            men: 200,
        })
    );
    let allied = run.scenario.world.side_view(Side::Allied);
    let told: Vec<_> = allied
        .events()
        .into_iter()
        .filter(|e| matches!(e.kind, SideEventKind::EnemySurrendered { .. }))
        .collect();
    assert_eq!(told.len(), 1);
    assert_eq!(
        told[0].kind,
        SideEventKind::EnemySurrendered { hex: S, men: 200 }
    );
    assert!(
        told[0].hidden_causes,
        "it cites the test that beat the enemy"
    );
    // The Allied side is never told of a falling back, nor of the unit.
    for e in allied.events() {
        let shown = format!("{:?}", e.kind);
        assert!(!shown.contains("UnitFellBack"), "{shown}");
        assert!(!shown.contains("UnitSurrendered"), "{shown}");
    }
    // The Japanese side is told of its own units falling back, and the
    // Allied side's own units never fall back.
    let fell: Vec<_> = japanese
        .events()
        .into_iter()
        .filter(|e| {
            matches!(
                &e.kind,
                SideEventKind::Whole(EventKind::UnitFellBack { .. })
            )
        })
        .collect();
    assert_eq!(fell.len(), 1 + usize::from(m_of(&run) > 0));
}

#[test]
fn s_the_allied_men_lost_count_rises_by_200() {
    let run = Run::live();
    let tally = tallies(
        run.scenario.world.umpire(),
        run.scenario.world.events(),
        run.scenario.world.ref_data(),
    );
    // Men the Japanese side lost, as the Allied objective counts them: the
    // surrender, the fight at T and the unfed losses.
    let surrendered = 200;
    let in_fight: u32 = run
        .all(|k| matches!(k, EventKind::MenLostInFight { unit, .. } if run.unit_side(*unit) == Side::Japanese))
        .iter()
        .map(|e| match e.kind {
            EventKind::MenLostInFight { lost, .. } => lost,
            _ => 0,
        })
        .sum();
    let unfed: u32 = run
        .all(|k| matches!(k, EventKind::MenLostUnfed { unit, .. } if run.unit_side(*unit) == Side::Japanese))
        .iter()
        .map(|e| match e.kind {
            EventKind::MenLostUnfed { lost, .. } => lost,
            _ => 0,
        })
        .sum();
    assert_eq!(
        tally[&Side::Allied].men_lost,
        surrendered + in_fight + unfed
    );
    // And without the surrender it would be 200 fewer: the same world with a
    // land link at S is not built here, so the check is by the events: the
    // count includes the 200 and the Japanese side's tally holds the Allied
    // men lost (the fight only).
    let allied_in_fight: u32 = run
        .all(|k| matches!(k, EventKind::MenLostInFight { unit, .. } if run.unit_side(*unit) == Side::Allied))
        .iter()
        .map(|e| match e.kind {
            EventKind::MenLostInFight { lost, .. } => lost,
            _ => 0,
        })
        .sum();
    assert_eq!(tally[&Side::Japanese].men_lost, allied_in_fight);
}

impl Run {
    fn unit_side(&self, id: LandUnitId) -> Side {
        self.at(0).land_units[&id].side
    }
}

// --------------------------------------------------------------- Properties

#[test]
fn property_men_are_conserved_for_a_falling_back_unit_each_loss_with_its_event() {
    for seed in 1..=60 {
        let run = Run::text(&with_seed(&text(), seed));
        for raw in [1, 3] {
            let Some(fell) = run.fell_back(raw) else {
                continue;
            };
            let (set_day, men_before) = match fell.kind {
                EventKind::UnitFellBack { men, .. } => (fell.day, men),
                _ => unreachable!(),
            };
            let arrived = run.arrived(raw);
            // Count the losses between the setting out and the arrival (or
            // the end): each has its event, and the men on the truth follow.
            let last_day = arrived.map_or(run.scenario.days, |(e, _, _)| e.day);
            let lost: u32 = (set_day + 1..=last_day)
                .filter_map(|d| run.lost_unfed(d, raw))
                .map(|(lost, _)| lost)
                .sum();
            let on_hand = run.unit_at(last_day, raw).men_ashore;
            assert_eq!(men_before, on_hand + lost, "seed {seed}, unit {raw}");
            if let Some((_, _, men)) = arrived {
                assert_eq!(men, on_hand, "seed {seed}, unit {raw}");
            }
            // Every loss event is a MenLostUnfed of the unit, none other.
            let other = run
                .all(|k| matches!(k, EventKind::MenLostInFight { unit, .. } if unit.raw() == raw));
            assert!(other.iter().all(|e| e.day == 1), "no fight after day 1");
        }
    }
}

/// The Allied fighting men ashore in a hex, in the truth at the end of `day`.
fn allied_fighting_men_at(run: &Run, day: u32, hex: Hex) -> u32 {
    run.at(day)
        .land_units
        .values()
        .filter(|u| u.side == Side::Allied && !u.working && u.hex == Some(hex))
        .map(|u| u.men_ashore)
        .sum()
}

#[test]
fn property_a_unit_that_falls_back_never_arrives_where_enemy_fighting_men_stood_unless_a_base_of_its_side_was_there(
) {
    // Variants of the scenario's group one with enemy units at Q and R and a
    // base of the side at R or at neither, and the same at T with seeds.
    let q = allied_unit_at(8, Q);
    let r = allied_unit_at(9, R);
    let variants: Vec<(&str, String)> = vec![
        ("base at R", text()),
        ("enemy at Q, base at R", with(&text(), &q)),
        ("no base, enemy at Q", with(&without_base_2(&text()), &q)),
        ("no base, enemy at R", with(&without_base_2(&text()), &r)),
        ("no base, no enemy", without_base_2(&text())),
        ("enemy at R, base at R", with(&text(), &r)),
        (
            "no base, enemy at both",
            with(&with(&without_base_2(&text()), &q), &r),
        ),
    ];
    let mut surrendered_at_p = 0;
    for (name, source) in variants {
        for seed in [1942, 7, 99] {
            let run = Run::text(&with_seed(&source, seed));
            let falls = run.all(|k| matches!(k, EventKind::UnitFellBack { .. }));
            for e in &falls {
                match e.kind {
                    EventKind::UnitFellBack { from, to, side, .. } => {
                        assert_eq!(side, Side::Japanese);
                        assert_ne!(from, to);
                        // Allied units are all static here, so the end of the
                        // day is as good as the moment it set out. Unless the
                        // hex held a base of its side when it set out, it is
                        // free of enemy fighting men.
                        let had_base = run
                            .at(e.day - 1)
                            .bases
                            .values()
                            .any(|b| b.side == Side::Japanese && b.hex == to);
                        if !had_base {
                            assert_eq!(
                                allied_fighting_men_at(&run, e.day, to),
                                0,
                                "{name}, seed {seed}: fell back to {to}"
                            );
                        }
                    }
                    _ => unreachable!(),
                }
            }
            // Where it went: a base of its side before an empty hex, the
            // first in hex order of each.
            if let Some(fell) = run.fell_back(1) {
                let to = match fell.kind {
                    EventKind::UnitFellBack { to, .. } => to,
                    _ => unreachable!(),
                };
                let expected = match name {
                    // The exception: a base of the side comes first, though
                    // enemy fighting men stand at R.
                    "base at R" | "enemy at Q, base at R" | "enemy at R, base at R" => R,
                    "no base, no enemy" | "no base, enemy at R" => Q,
                    "no base, enemy at Q" => R,
                    other => panic!("{other}"),
                };
                assert_eq!(to, expected, "{name}, seed {seed}");
            } else {
                // Nowhere to go: it surrendered, with its 300 men.
                assert_eq!(name, "no base, enemy at both");
                surrendered_at_p += 1;
                let s = run.all(|k| {
                    matches!(k, EventKind::UnitSurrendered { unit, hex, men, .. } if unit.raw() == 1 && *hex == P && *men == 300)
                });
                assert_eq!(s.len(), 1);
                assert_eq!(run.unit_at(1, 1).men_ashore, 0);
            }
        }
    }
    assert_eq!(surrendered_at_p, 3);
}

#[test]
fn a_base_of_the_side_is_preferred_even_where_enemy_fighting_men_stand() {
    // The exception the property allows: base 2 at R comes before the free
    // hex Q though an Allied unit stands at R, because a hex that holds a
    // base of the unit's side is a place to fall back to.
    let run = Run::text(&with(&text(), &allied_unit_at(9, R)));
    let fell = run.fell_back(1).expect("falls back");
    match fell.kind {
        EventKind::UnitFellBack { to, .. } => assert_eq!(to, R),
        _ => unreachable!(),
    }
}

#[test]
fn property_a_march_order_the_day_after_replaces_the_falling_back() {
    // Day 2: unit 1 is ordered to march to Q, though it fell back for R.
    let extra = order(2, "japanese", 1, "kind = \"march\"\nhex = [0, 1]");
    let run = Run::text(&with(&text(), &extra));
    let fell = run.fell_back(1).expect("it fell back on day 1");
    match fell.kind {
        EventKind::UnitFellBack { to, .. } => assert_eq!(to, R),
        _ => unreachable!(),
    }
    let accepted = run.on(2, |k| matches!(k, EventKind::OrderAccepted { .. }));
    assert!(accepted
        .iter()
        .any(|e| matches!(e.kind, EventKind::OrderAccepted { target: sim_core::ids::EntityRef::LandUnit(u), .. } if u.raw() == 1)));
    // The march is the order's: its days are caused by the order, and it ends
    // at Q, with the same men as the falling back would have left it.
    let days = run.marched_days(1);
    assert_eq!(days.len(), 3, "{days:?}");
    assert!(days
        .iter()
        .all(|(_, _, cause)| matches!(cause, Cause::Order(_))));
    let (arrived, to, men) = run.arrived(1).expect("arrives");
    assert_eq!(to, Q);
    assert_eq!(arrived.day, 4);
    let last_day = run.last_march_day(1);
    assert_eq!(arrived.causes, vec![Cause::Event(last_day.id)]);
    assert!(matches!(last_day.causes[0], Cause::Order(_)));
    assert_eq!(men, 258);
    assert_eq!(run.unit_at(6, 1).hex, Some(Q));
    // Nothing of the old march is left: it never reached R.
    assert!(run
        .all(
            |k| matches!(k, EventKind::MarchArrived { to, unit, .. } if *to == R && unit.raw() == 1)
        )
        .is_empty());
    // Unfed at Q, where its side has no base, on the nights of days 5 and 6.
    assert_eq!(run.night_paid(5, 2), None);
    assert!(run.lost_unfed(5, 1).is_some());
}

#[test]
fn property_an_attack_order_the_day_after_is_taken() {
    // Day 2: unit 3 is ordered to attack at T, where it still is. It has men
    // if the dice left it any: look for a seed where it does.
    let extra = order(
        2,
        "japanese",
        3,
        "kind = \"set_posture\"\nposture = \"attack\"",
    );
    let run = Run::text(&with(&text(), &extra));
    assert!(m_of(&run) > 0, "unit 3 survives the fight with seed 1942");
    let accepted = run.on(2, |k| matches!(k, EventKind::OrderAccepted { .. }));
    assert!(accepted
        .iter()
        .any(|e| matches!(e.kind, EventKind::OrderAccepted { target: sim_core::ids::EntityRef::LandUnit(u), .. } if u.raw() == 3)));
    // The order is taken: the Japanese side attacks at T on day 2, with its
    // unit still there (it marched a day of three).
    let fights = run.on(
        2,
        |k| matches!(k, EventKind::GroundFight { hex, .. } if *hex == T),
    );
    assert_eq!(fights.len(), 1);
    match fights[0].kind {
        EventKind::GroundFight {
            japanese_attacks, ..
        } => assert!(japanese_attacks),
        _ => unreachable!(),
    }
    let fell = run.fell_back(3).expect("it fell back on day 1");
    assert!(matches!(fell.kind, EventKind::UnitFellBack { .. }));
}

/// The text of an event.
fn shown(events: &[&Event]) -> Vec<String> {
    events.iter().map(|e| e.kind.to_string()).collect()
}

#[test]
fn property_a_scenario_with_no_side_beaten_is_what_it_was_whatever_the_figure() {
    // Every unit on the defence from the start: no attack, so no side is
    // beaten anywhere. The rule does not act: the log has no falling back and
    // no surrender, and the world is the same with the figure at 0 as at 30.
    let calm = text().replace("posture = \"attack\"", "posture = \"defend\"");
    assert!(!calm.contains("posture = \"attack\""));
    let at_30 = Run::text(&calm);
    let at_0 = Run::of(
        common::from_text_with(
            &calm,
            &path(),
            &common::altered_reference("beaten_fatigue_0", |dir| {
                common::edit_file_by(
                    dir,
                    "ground.toml",
                    &[("beaten_fatigue = 30", "beaten_fatigue = 0")],
                );
            }),
        )
        .expect("loads with the figure at 0"),
    );
    for run in [&at_30, &at_0] {
        assert!(run
            .all(|k| matches!(
                k,
                EventKind::UnitFellBack { .. }
                    | EventKind::UnitSurrendered { .. }
                    | EventKind::SideBeatenAshore { .. }
                    | EventKind::BaseChangedSide { .. }
            ))
            .is_empty());
        assert!(run
            .all(|k| matches!(k, EventKind::MarchDay { .. }))
            .is_empty());
    }
    assert_eq!(
        shown(&at_30.all(|_| true)),
        shown(&at_0.all(|_| true)),
        "the same events with the figure changed"
    );
    assert_eq!(
        at_30.scenario.world.day_hashes().len(),
        at_0.scenario.world.day_hashes().len()
    );
    // Every unit stays where it stood, none marched and none is tired by
    // defeat: the Japanese men stay at P, T and S and are fed by their bases.
    for raw in 1..=7 {
        assert_eq!(at_30.unit_at(6, raw).hex, at_30.unit_at(0, raw).hex);
        assert!(at_30.unit_at(6, raw).march.is_none());
    }
}

#[test]
fn property_a_side_beaten_where_no_land_link_leads_out_is_no_different_from_surrender_alone() {
    // A beaten side with men and nowhere to go surrenders: here at S, and
    // at P when both exits hold enemy fighting men. Men that surrender are
    // counted once, as men lost.
    let source = with(
        &with(&without_base_2(&text()), &allied_unit_at(8, Q)),
        &allied_unit_at(9, R),
    );
    let run = Run::text(&source);
    let surrendered = run.all(|k| matches!(k, EventKind::UnitSurrendered { .. }));
    assert_eq!(surrendered.len(), 2);
    let men: u32 = surrendered
        .iter()
        .map(|e| match e.kind {
            EventKind::UnitSurrendered { men, .. } => men,
            _ => 0,
        })
        .sum();
    assert_eq!(men, 500);
    let tally = tallies(
        run.scenario.world.umpire(),
        run.scenario.world.events(),
        run.scenario.world.ref_data(),
    );
    let in_fight_and_unfed: u32 = run
        .all(|k| {
            matches!(
                k,
                EventKind::MenLostInFight { unit, .. } | EventKind::MenLostUnfed { unit, .. }
                    if run.unit_side(*unit) == Side::Japanese
            )
        })
        .iter()
        .map(|e| match e.kind {
            EventKind::MenLostInFight { lost, .. } | EventKind::MenLostUnfed { lost, .. } => lost,
            _ => 0,
        })
        .sum();
    assert_eq!(tally[&Side::Allied].men_lost, 500 + in_fight_and_unfed);
}

// ------------------------------------------------ The enemy's view is the same

/// The text of a side's view at the end of each day: its events, contacts,
/// claims, what it holds of the ground (the hex, the men and the day, not the
/// report's place in the full log), its enemy bases when `with_bases` is true,
/// and every question it can ask with the answer.
fn views(source: &str, side: Side, with_bases: bool) -> Vec<String> {
    let mut scenario = from(source);
    let mut out = Vec::new();
    while scenario.next_day().expect("a day").is_some() {
        let view = scenario.world.side_view(side);
        let mut text = String::new();
        for e in view.events() {
            text += &format!(
                "{} D{} {:?} {:?} {:?} {:?} {} {:?}\n",
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
            text += &format!("{c:?}\n");
        }
        for c in view.claims() {
            text += &format!("{}\n", c.text(side));
        }
        for g in view.ground_estimates() {
            text += &format!("estimate {:?} {} {}\n", g.hex, g.men, g.day);
        }
        if with_bases {
            for b in view.enemy_bases() {
                text += &format!("{} {:?} {:?} {:?}\n", b.id.raw(), b.name, b.hex, b.side);
            }
        }
        for b in view.bases() {
            text += &format!("{b:?}\n");
        }
        for u in view.land_units() {
            text += &format!("{u:?}\n");
        }
        for q in view.questions() {
            text += &format!("{q:?} -> {:?}\n", view.ask(&q));
        }
        out.push(text);
    }
    out
}

#[test]
fn property_the_enemys_view_is_the_same_whichever_way_the_beaten_side_falls_back() {
    // World one: unit 1 falls back to R, where base 2 stands. World two: the
    // Japanese base at R is gone and unit 1 falls back to Q, the first hex
    // in hex order with no Allied fighting man. The Allied side has no unit in
    // Q or R, so it sees nothing of either march. (The Allied side's list of
    // enemy bases is not part of the comparison: base 2 is a public fact and
    // it is the one thing the two worlds differ in beside where unit 1 goes.)
    let one = text();
    let two = without_base_2(&text());
    let (a, b) = (Run::text(&one), Run::text(&two));
    // The worlds differ, and unit 1 went to different hexes.
    assert_ne!(a.scenario.world.day_hashes(), b.scenario.world.day_hashes());
    assert_eq!(a.unit_at(6, 1).hex, Some(R));
    assert_eq!(b.unit_at(6, 1).hex, Some(Q));
    assert_ne!(a.unit_at(6, 1).men_ashore, b.unit_at(6, 1).men_ashore);
    // The Allied view is the same on each day.
    let seen_one = views(&one, Side::Allied, false);
    let seen_two = views(&two, Side::Allied, false);
    assert_eq!(seen_one.len(), 6);
    assert_eq!(seen_one, seen_two);
    // Including the Allied side's questions, which are there to compare.
    assert!(seen_one[5].contains("->"));
    // The Japanese view does differ: it sees its own unit's march.
    assert_ne!(
        views(&one, Side::Japanese, false),
        views(&two, Side::Japanese, false)
    );
    // Nothing of the march is in the Allied events.
    for text in &seen_one {
        for word in [
            "UnitFellBack",
            "MarchDay",
            "MarchArrived",
            "LandUnitId(1)",
            "(1, 0)",
            "(0, 1)",
        ] {
            assert!(!text.contains(word), "{word}: {text}");
        }
    }
}

#[test]
fn property_the_enemys_view_is_the_same_whether_unit_3_is_fed_at_v_or_not() {
    // Where unit 3 goes at T cannot be hidden by a base at U (the Allied
    // side holds U): the two worlds differ in a Japanese base at V, which
    // would bring unit 3 to V in either case. The check is that the Allied
    // view of the fight and its ground reports is the same when unit 3's
    // later fate (unfed at V, or fed at a base there) differs.
    let one = text();
    let two = with(
        &one,
        "\n[[base]]\nid = 9\nname = \"Placeholder base 9, Japanese, at V\"\nside = \"japanese\"\nq = 11\nr = 0\nsupply_t = 1000\n",
    );
    let (a, b) = (Run::text(&one), Run::text(&two));
    assert_eq!(a.unit_at(6, 3).hex, b.unit_at(6, 3).hex);
    assert_ne!(a.unit_at(6, 3).men_ashore, b.unit_at(6, 3).men_ashore);
    assert_eq!(
        views(&one, Side::Allied, false),
        views(&two, Side::Allied, false)
    );
}
