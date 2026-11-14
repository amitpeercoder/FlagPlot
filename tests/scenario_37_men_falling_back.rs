//! Scenario 37: men who are falling back are fired on only when the side
//! that beat them is on attack. A fight another unit of their own side
//! starts costs them nothing and does not count them; a victor who goes on
//! attacking fires on them and counts them, as before.
//!
//! Proves decision 0109, correction 1, against a hand calculation. The data
//! is placeholder data: see `data/scenarios/37_men_falling_back.toml`. It
//! runs on the frozen step 1 figures and sets `engagement_ratio` to 3, as
//! the test of scenario 33 does; `failed_attack_fatigue` is left at 0.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide (the
//! hits of a fight), the calculation gives the firing men, the shots after
//! the limit and the chance exactly, and the rule for what follows; the
//! tests read the hits from the run. Everything else is asserted exactly.
//! If the code disagrees, the code or the rule is wrong: do not change a
//! number here to make a test pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! `engagement_ratio` 3. From the earlier rules: `men_per_shot` 100,
//! `hit_chance` 0.3, `defender_hit_chance` 0.6, `men_per_hit` 20,
//! `fatigue_per_attack_day` 20, `fatigue_per_march_day` 10, `rest_per_day`
//! 5, `attack_fatigue_limit` 60, `beaten_share` 0.2, `beaten_fatigue` 30,
//! `march_days_per_hex` 3, `march_supply_days` 5, `daily_supply_t_per_man`
//! 0.005, `unfed_loss_share_per_day` 0.05, `captured_stock_share` 0.5.
//!
//! In the ground combat phase the marches come first, then the fight, the
//! day's fatigue, the test of a side beaten and the bases that change side.
//! So a unit that arrives by march fights that day with the fatigue of the
//! days before, and the fatigue of the day's march is added after the
//! fight.
//!
//! ## Inputs
//!
//! A placeholder map. Every hex is a coast hex and clear. The game is 6
//! days.
//!
//! - P at (0,0), with a land link to R at (1,0).
//! - T at (10,0), with a land link to U at (11,0).
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Japanese | P   | 1000 t |
//! | 2    | Japanese | R   | 100 t  |
//! | 3    | Allied   | P   | 5000 t |
//! | 4    | Japanese | T   | 1000 t |
//! | 5    | Japanese | U   | 100 t  |
//! | 6    | Allied   | T   | 5000 t |
//!
//! | Unit | Side     | Men  | Hex | Posture |
//! | ---- | -------- | ---- | --- | ------- |
//! | 1    | Japanese | 1000 | P   | defend  |
//! | 2    | Allied   | 6000 | P   | attack  |
//! | 3    | Japanese | 300  | R   | attack  |
//! | 4    | Japanese | 1000 | T   | defend  |
//! | 5    | Allied   | 6000 | T   | attack  |
//!
//! All are fighting units. Orders: on day 1, unit 3 marches to P. On day
//! 2, unit 2 is set to defend. Unit 5 is given no order and stays on
//! attack.
//!
//! ## At P: a fight their own side starts
//!
//! Day 1. The Allied side attacks. Unit 3 is at R and is not in the fight.
//!
//! - Unit 2: 6000 firing men, limited to 3 x 1000 = 3000: 30 shots at 0.3.
//! - Unit 1: 1000 firing men, 10 shots at 0.6.
//!
//! Let b be unit 2's hits and a unit 1's. Unit 1 has 1000 - 20 x b men,
//! at least 400; unit 2 has 6000 - 20 x a. The Japanese side is beaten
//! whatever the dice give: at most 1000 men against 0.2 times at least
//! 5800. Base 1 becomes Allied. Unit 1 falls back: R is the one linked hex
//! and holds base 2. Its fatigue is 0 after the day's rest, and the defeat
//! adds 30.
//!
//! Day 2. Unit 2 is set to defend at intake. No unit at P is on attack:
//! there is no fight. Unit 1 is unfed that night, since its side has no
//! base at P, and loses its men times 0.05, cut to a whole man: its fed
//! share is 0. It marches, day 1 of 3. Unit 2 rests: its fatigue goes from
//! 20 to 15.
//!
//! Day 3. Unit 1 is unfed again and marches, day 2 of 3. Unit 3 arrives at
//! P on its third day of march and puts down the 300 x 0.005 x 5 = 7.5 t it
//! carried: a Japanese base is formed at P. It is on attack, so the hex is
//! contested.
//!
//! - Unit 3: fed in full at R the night before; its fatigue at the fight is
//!   20, from two days of march. Firing men 300 x 80/100 = 240: 2.4 shots
//!   at 0.3.
//! - Unit 1 is falling back, and the Allied side is not on attack in the
//!   hex. So it is not counted among the Japanese fighting men for the
//!   limit, and it takes no share of the fight's losses. Its fed share is
//!   0, so it fires nothing in any case, and by the correction it would
//!   not fire if it could.
//! - Unit 2: its fatigue is 15, so its firing men are its men times 85/100,
//!   cut, far above the limit of 3 x 300 = 900: 9 shots at 0.6.
//!
//! Let c be unit 2's hits on day 3. Unit 3 loses 20 x c men, at most 180.
//! Unit 1 loses none in the fight: the only men it loses on days 2 to 4
//! are the unfed. Without the correction it would have shared the 20 x c
//! in proportion to its men, and the limit would have been three times the
//! men of both units.
//!
//! Unit 3's fatigue after day 3 is 20 + 10 for the march + 20 for the
//! attack: 50. That is not above 60, and it stays on attack.
//!
//! Day 4. Unit 1 marches its third day and arrives at R, where base 2
//! feeds it from the night of day 5. If unit 3 has a man left the fight
//! goes on between it and unit 2, with the limit at three times unit 3's
//! men; the tests work each later day by the rule from the men and fatigue
//! the run has.
//!
//! Supply at R: base 2 pays 1.5 t for unit 3 on each of the nights of days
//! 1, 2 and 3, and gives it 7.5 t on the day it arrives at P: 88 t after
//! day 3.
//!
//! ## At T: a victor who goes on attacking
//!
//! Day 1 is as at P: unit 5 is limited to 30 shots at 0.3, unit 4 fires 10
//! at 0.6, the Japanese side is beaten, base 4 becomes Allied and unit 4
//! falls back toward U with fatigue 30. Unit 5's fatigue is 20.
//!
//! Days 2 and 3. Unit 5 is still on attack and the hex is contested. Unit
//! 4 is falling back and the side that beat it is on attack: it is counted
//! and it is fired on, as before the correction.
//!
//! - Let m be unit 4's men before the day's fight, after that night's
//!   unfed loss. Its fed share is 0: it fires nothing.
//! - Unit 5's firing men are its men times 80/100 on day 2 and 60/100 on
//!   day 3, cut, far above the limit of 3 x m: its shots are 3 x m / 100,
//!   at 0.3.
//! - Unit 4 loses 20 for each hit, or all it has. It is already marching:
//!   beaten again, it is left as it is.
//!
//! Unit 5's fatigue is 40 after day 2 and 60 after day 3. 60 is not above
//! the limit: it stays on attack.
//!
//! Day 4. The marches come first: unit 4, if it has a man left, arrives at
//! U before the fight. No Japanese man is at T: there is no fight, and
//! unit 5 rests to 55.
//!
//! # Properties
//!
//! - A unit that is not falling back shares in every fight in its hex, as
//!   before: with unit 1 never beaten (unit 2 on defend from day 1) and
//!   unit 3 attacking on day 3, units 1 and 3 share the losses in
//!   proportion and both count for the limit.
//! - With the frozen `engagement_ratio`, and no unit falling back in a hex
//!   where its own side attacks, every scenario gives what it gave before,
//!   apart from the hash version. A scenario that does differ is listed
//!   for the architect before its baseline is remade.
//! - The enemy's view is the same in two worlds that differ only in
//!   whether a unit it cannot see is falling back or standing, where the
//!   hits are the same.

mod common;

use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind, LandFatigueChange};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, LandUnitId};
use sim_core::orders::Posture;
use sim_core::state::{LandUnit, Side, TruthState};
use sim_data::{LoadOptions, Scenario};

const P: Hex = Hex::new(0, 0);
const R: Hex = Hex::new(1, 0);
const T: Hex = Hex::new(10, 0);
const U: Hex = Hex::new(11, 0);

fn path() -> PathBuf {
    common::data_dir().join("scenarios/37_men_falling_back.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// The frozen step 1 figures with the ratio of this calculation, 3.
/// `failed_attack_fatigue` stays at the frozen 0.
fn figures(tag: &str) -> LoadOptions {
    common::altered_reference(tag, |dir| {
        common::edit_file_by(
            dir,
            "ground.toml",
            &[("engagement_ratio = 1000000", "engagement_ratio = 3")],
        );
    })
}

fn from(source: &str) -> Scenario {
    common::from_text_with(source, &path(), &figures("s37")).expect("the scenario loads")
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

/// `men` over 100: the shots of that many firing men, with no limit.
fn shots_of(men: u32) -> Fixed {
    Fixed::from_ratio(i64::from(men), 100)
}

fn unit(raw: u32) -> LandUnitId {
    LandUnitId::new(raw)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

/// Whole men: `men` times the share `num` over `den`, cut.
fn share(men: u32, num: u32, den: u32) -> u32 {
    men * num / den
}

struct Fire {
    unit: u32,
    firing: u32,
    shots: Fixed,
    chance: Fixed,
    hits: u32,
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

    fn men(&self, day: u32, raw: u32) -> u32 {
        self.unit_at(day, raw).men_ashore
    }

    fn supply_at(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).bases[&base(raw)].supply_t
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

    fn fights(&self, day: u32, hex: Hex) -> Vec<&Event> {
        self.on(
            day,
            |k| matches!(k, EventKind::GroundFight { hex: h, .. } if *h == hex),
        )
    }

    /// The fire of the day's fight at a hex, in the order of the log.
    fn fires(&self, day: u32, hex: Hex) -> Vec<Fire> {
        self.on(
            day,
            |k| matches!(k, EventKind::GroundFire { hex: h, .. } if *h == hex),
        )
        .into_iter()
        .map(|e| match e.kind {
            EventKind::GroundFire {
                unit,
                firing_men,
                shots,
                chance,
                hits,
                ..
            } => Fire {
                unit: unit.raw(),
                firing: firing_men,
                shots,
                chance,
                hits,
            },
            _ => unreachable!(),
        })
        .collect()
    }

    fn fire(&self, day: u32, hex: Hex, raw: u32) -> Fire {
        let mut found: Vec<Fire> = self
            .fires(day, hex)
            .into_iter()
            .filter(|f| f.unit == raw)
            .collect();
        assert_eq!(found.len(), 1, "unit {raw} fires once on day {day}");
        found.remove(0)
    }

    /// What a unit lost in the fight of a day.
    fn lost_in_fight(&self, day: u32, raw: u32) -> u32 {
        self.on(
            day,
            |k| matches!(k, EventKind::MenLostInFight { unit, .. } if unit.raw() == raw),
        )
        .iter()
        .map(|e| match e.kind {
            EventKind::MenLostInFight { lost, .. } => lost,
            _ => unreachable!(),
        })
        .sum()
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

    /// The `MenFed` of a side at a hex on the night of `day`:
    /// `(base, men, fed, unfed)`.
    fn fed(&self, day: u32, side: Side, hex: Hex) -> Option<(Option<u32>, u32, u32, u32)> {
        let found = self.on(
            day,
            |k| matches!(k, EventKind::MenFed { side: s, hex: h, .. } if *s == side && *h == hex),
        );
        assert!(found.len() <= 1);
        found.first().map(|e| match e.kind {
            EventKind::MenFed {
                base,
                men,
                fed,
                unfed,
                ..
            } => (base.map(|b| b.raw()), men, fed, unfed),
            _ => unreachable!(),
        })
    }

    /// The day's fatigue events of a unit.
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

    fn back_to_defend(&self, raw: u32) -> Vec<&Event> {
        self.all(|k| matches!(k, EventKind::LandUnitBackToDefend { unit, .. } if unit.raw() == raw))
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

    /// `MarchArrived` of a unit: the event, its destination, men, supply
    /// carried and base formed.
    fn arrived(&self, raw: u32) -> Option<(&Event, Hex, u32, Fixed, Option<BaseId>)> {
        let found =
            self.all(|k| matches!(k, EventKind::MarchArrived { unit, .. } if unit.raw() == raw));
        assert!(found.len() <= 1);
        found.first().map(|e| match e.kind {
            EventKind::MarchArrived {
                to,
                men,
                supply_t,
                base,
                ..
            } => (*e, to, men, supply_t, base),
            _ => unreachable!(),
        })
    }
}

/// The factor of that name in an event's breakdown.
fn factor(event: &Event, name: &str) -> Option<Fixed> {
    event
        .breakdown
        .as_ref()
        .expect("a breakdown")
        .factors
        .iter()
        .find(|f| f.name == name)
        .map(|f| f.value)
}

/// The `GroundFire` event of a unit on a day.
fn fire_event(run: &Run, day: u32, raw: u32) -> &Event {
    let found = run.on(
        day,
        |k| matches!(k, EventKind::GroundFire { unit, .. } if unit.raw() == raw),
    );
    assert_eq!(found.len(), 1, "unit {raw} fires once on day {day}");
    found[0]
}

/// The `UnitFellBack` events of a unit: the first falling back, by decision
/// 0104, is the only one of unit 1 and unit 4.
fn fell_back(run: &Run, raw: u32) -> Vec<&Event> {
    run.all(|k| matches!(k, EventKind::UnitFellBack { unit, .. } if unit.raw() == raw))
}

/// The men a unit loses unfed on a night: its men times 0.05, cut to a whole
/// man, one at least.
fn unfed_loss(men: u32) -> u32 {
    (men * 5 / 100).max(1)
}

/// `take_in_proportion` as `ground.rs` has it: the loss shared among the
/// units in proportion to their men, the parts cut, the rest given a man at
/// a time in unit order to units with men left.
fn in_proportion(loss: u32, men: &[u32]) -> Vec<u32> {
    let total: u64 = men.iter().map(|m| u64::from(*m)).sum();
    let loss = u64::from(loss).min(total);
    let mut parts: Vec<u64> = men.iter().map(|m| loss * u64::from(*m) / total).collect();
    let mut left = loss - parts.iter().sum::<u64>();
    while left > 0 {
        for (part, m) in parts.iter_mut().zip(men) {
            if left == 0 {
                break;
            }
            if *part < u64::from(*m) {
                *part += 1;
                left -= 1;
            }
        }
    }
    parts.into_iter().map(|p| p as u32).collect()
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_figures_and_the_inputs_are_the_calculations() {
    let run = Run::live();
    let figures = run.scenario.world.ref_data().ground.expect("ground");
    assert_eq!(figures.engagement_ratio, n(3));
    assert_eq!(figures.failed_attack_fatigue, n(0));
    assert_eq!(figures.men_per_shot, n(100));
    assert_eq!(figures.hit_chance, fx("0.3"));
    assert_eq!(figures.defender_hit_chance, fx("0.6"));
    assert_eq!(figures.men_per_hit, n(20));
    assert_eq!(figures.fatigue_per_attack_day, n(20));
    assert_eq!(figures.fatigue_per_march_day, n(10));
    assert_eq!(figures.rest_per_day, n(5));
    assert_eq!(figures.attack_fatigue_limit, n(60));
    assert_eq!(figures.beaten_share, fx("0.2"));
    assert_eq!(figures.beaten_fatigue, n(30));
    assert_eq!(figures.march_days_per_hex, 3);
    assert_eq!(figures.march_supply_days, n(5));
    assert_eq!(figures.daily_supply_t_per_man, fx("0.005"));
    assert_eq!(figures.unfed_loss_share_per_day, fx("0.05"));
    assert_eq!(figures.captured_stock_share, fx("0.5"));
    assert_eq!(figures.experience_fire_strength, n(0));

    assert_eq!(run.scenario.days, 6);
    let start = run.at(0);
    assert_eq!(start.land_units.len(), 5);
    assert_eq!(start.bases.len(), 6);
    let bases = [
        (1, Side::Japanese, P, n(1000)),
        (2, Side::Japanese, R, n(100)),
        (3, Side::Allied, P, n(5000)),
        (4, Side::Japanese, T, n(1000)),
        (5, Side::Japanese, U, n(100)),
        (6, Side::Allied, T, n(5000)),
    ];
    for (id, side, hex, supply) in bases {
        let b = &start.bases[&base(id)];
        assert_eq!(
            (b.side, b.hex, b.supply_t),
            (side, hex, supply),
            "base {id}"
        );
    }
    let units = [
        (1, Side::Japanese, 1000, P, Posture::Defend),
        (2, Side::Allied, 6000, P, Posture::Attack),
        (3, Side::Japanese, 300, R, Posture::Attack),
        (4, Side::Japanese, 1000, T, Posture::Defend),
        (5, Side::Allied, 6000, T, Posture::Attack),
    ];
    for (id, side, men, hex, posture) in units {
        let u = run.unit_at(0, id);
        assert_eq!(
            (u.side, u.working, u.men_ashore, u.hex, u.posture, u.fatigue),
            (side, false, men, Some(hex), posture, n(0)),
            "unit {id}"
        );
        assert!(u.march.is_none());
    }
    // The land links the calculation leans on, and no other.
    let map = run.scenario.world.map();
    assert_eq!(map.land_neighbours(P), vec![R]);
    assert_eq!(map.land_neighbours(R), vec![P]);
    assert_eq!(map.land_neighbours(T), vec![U]);
    assert_eq!(map.land_neighbours(U), vec![T]);
    // Fights on day 1 at P and T only, the Allied side attacking.
    for hex in [P, T] {
        match run.fights(1, hex)[0].kind {
            EventKind::GroundFight {
                allied_attacks,
                japanese_attacks,
                ..
            } => assert_eq!((allied_attacks, japanese_attacks), (true, false)),
            _ => unreachable!(),
        }
    }
    assert_eq!(
        run.on(1, |k| matches!(k, EventKind::GroundFight { .. }))
            .len(),
        2
    );
}

// --------------------------------------------------------------------- At P

/// What the fight at P on day 1 gave: unit 2's hits, b, and unit 1's, a.
fn day_1_hits(run: &Run, hex: Hex, attacker: u32, defender: u32) -> (u32, u32) {
    let (f_attacker, f_defender) = (run.fire(1, hex, attacker), run.fire(1, hex, defender));
    (f_attacker.hits, f_defender.hits)
}

#[test]
fn p_day_1_unit_2_fires_30_shots_unit_1_10_and_the_japanese_side_is_beaten() {
    let run = Run::live();
    // Unit 3 is at R and is not in the fight.
    assert_eq!(run.fights(1, P).len(), 1);
    assert_eq!(run.fires(1, P).len(), 2);
    let f2 = run.fire(1, P, 2);
    assert_eq!((f2.firing, f2.shots, f2.chance), (6000, n(30), fx("0.3")));
    let f1 = run.fire(1, P, 1);
    assert_eq!((f1.firing, f1.shots, f1.chance), (1000, n(10), fx("0.6")));
    // The limit cut unit 2 from 60 shots to 30: 3 x 1000 = 3000 engaged.
    let e2 = fire_event(&run, 1, 2);
    assert_eq!(factor(e2, "shots_before_limit"), Some(n(60)));
    assert_eq!(factor(e2, "side_firing_men"), Some(n(6000)));
    assert_eq!(factor(e2, "engaged_men"), Some(n(3000)));
    let (b, a) = day_1_hits(&run, P, 2, 1);
    assert_eq!((b, a), (f2.hits, f1.hits));
    // Unit 1 has 1000 - 20 b men, at least 400; unit 2 has 6000 - 20 a.
    assert_eq!(run.lost_in_fight(1, 1), 20 * b);
    assert_eq!(run.lost_in_fight(1, 2), 20 * a);
    assert_eq!(run.men(1, 1), 1000 - 20 * b);
    assert_eq!(run.men(1, 2), 6000 - 20 * a);
    assert!(run.men(1, 1) >= 400);
    // The Japanese side is beaten whatever the dice give.
    let beaten = run.all(|k| matches!(k, EventKind::SideBeatenAshore { hex, .. } if *hex == P));
    assert_eq!(beaten.len(), 1);
    assert_eq!(
        beaten[0].kind,
        EventKind::SideBeatenAshore {
            side: Side::Japanese,
            hex: P,
            men: run.men(1, 1),
            enemy_men: run.men(1, 2),
            fought: true,
        }
    );
    // Base 1 becomes Allied, on day 1, and base 2 and base 3 do not change.
    let changed =
        run.all(|k| matches!(k, EventKind::BaseChangedSide { base, .. } if base.raw() == 1));
    assert_eq!(changed.len(), 1);
    assert_eq!(changed[0].day, 1);
    assert_eq!(run.at(1).bases[&base(1)].side, Side::Allied);
    assert_eq!(run.at(6).bases[&base(1)].side, Side::Allied);
    assert_eq!(run.at(6).bases[&base(2)].side, Side::Japanese);
    assert_eq!(run.at(6).bases[&base(3)].side, Side::Allied);
    // Unit 1 falls back to R, the one linked hex, which holds base 2. Its
    // fatigue is 0 after the day's rest, and the defeat adds 30.
    let fell = fell_back(&run, 1);
    assert_eq!(fell.len(), 1);
    assert_eq!(fell[0].day, 1);
    assert_eq!(
        fell[0].kind,
        EventKind::UnitFellBack {
            unit: unit(1),
            side: Side::Japanese,
            from: P,
            to: R,
            men: run.men(1, 1),
            fatigue: n(30),
        }
    );
    assert_eq!(run.unit_at(1, 1).fatigue, n(30));
    let march = run.unit_at(1, 1).march.as_ref().expect("a march");
    assert_eq!(march.to, R);
    assert_eq!(march.cause, Cause::Event(fell[0].id));
    // Unit 2's fatigue is 20.
    assert_eq!(run.unit_at(1, 2).fatigue, n(20));
    // Unit 3 marched the first day of three toward P and is not in the
    // fight; it is at R, fed by base 2.
    assert_eq!(run.unit_at(1, 3).hex, Some(R));
    assert_eq!(run.men(1, 3), 300);
    assert_eq!(run.marched_days(3)[0].0, 1);
    assert_eq!(run.fed(1, Side::Japanese, R), Some((Some(2), 300, 300, 0)));
    assert_eq!(run.fed(1, Side::Allied, P), Some((Some(3), 6000, 6000, 0)));
    assert_eq!(
        run.fed(1, Side::Japanese, P),
        Some((Some(1), 1000, 1000, 0))
    );
}

#[test]
fn p_day_2_there_is_no_fight_unit_1_is_unfed_and_marches_and_unit_2_rests_to_15() {
    let run = Run::live();
    // Unit 2 is set to defend at intake: no unit at P is on attack.
    assert_eq!(run.unit_at(1, 2).posture, Posture::Attack);
    assert_eq!(run.unit_at(2, 2).posture, Posture::Defend);
    assert!(run.fights(2, P).is_empty());
    assert!(run.fires(2, P).is_empty());
    assert_eq!(run.lost_in_fight(2, 1), 0);
    assert_eq!(run.lost_in_fight(2, 2), 0);
    // Unit 1 is unfed that night: its side has no base at P. It loses its
    // men times 0.05 and its fed share is 0.
    let men = run.men(1, 1);
    let lost = unfed_loss(men);
    assert_eq!(run.fed(2, Side::Japanese, P), Some((None, men, 0, men)));
    assert_eq!(run.lost_unfed(2, 1), Some((lost, men - lost)));
    assert_eq!(run.men(2, 1), men - lost);
    assert_eq!(run.unit_at(2, 1).fed_share, n(0));
    // It marches, day 1 of 3, its fatigue 30 + 10.
    let marched = run.marched_days(1);
    assert_eq!(marched[0].0, 1);
    assert_eq!(marched[0].1, 3);
    assert!(matches!(marched[0].2, Cause::Event(_)));
    assert_eq!(run.unit_at(2, 1).fatigue, n(40));
    // Unit 2 rests from 20 to 15, and is fed in full.
    assert_eq!(run.fatigue(2, 2), vec![(LandFatigueChange::Rested, n(15))]);
    assert_eq!(run.unit_at(2, 2).fatigue, n(15));
    let men2 = run.men(1, 2);
    assert_eq!(run.fed(2, Side::Allied, P), Some((Some(1), men2, men2, 0)));
    assert_eq!(run.men(2, 2), men2);
    // Unit 3 marches its second day.
    assert_eq!(run.unit_at(2, 3).hex, Some(R));
    assert_eq!(run.fed(2, Side::Japanese, R), Some((Some(2), 300, 300, 0)));
}

#[test]
fn p_day_3_unit_1_is_out_of_the_fight_and_unit_2_is_limited_to_900() {
    let run = Run::live();
    // Unit 1 is unfed again and marches, day 2 of 3.
    let men1 = run.men(2, 1);
    let lost = unfed_loss(men1);
    assert_eq!(run.fed(3, Side::Japanese, P), Some((None, men1, 0, men1)));
    assert_eq!(run.lost_unfed(3, 1), Some((lost, men1 - lost)));
    assert_eq!(run.marched_days(1)[1].0, 2);
    assert_eq!(run.unit_at(3, 1).fatigue, n(50));
    assert_eq!(run.unit_at(3, 1).fed_share, n(0));
    // Unit 3 arrives at P on its third day of march and puts down the
    // 300 x 0.005 x 5 = 7.5 t it carried: a Japanese base is formed at P.
    let (_, to, men, carried, formed) = run.arrived(3).expect("unit 3 arrives");
    assert_eq!((to, men, carried), (P, 300, fx("7.5")));
    let formed = formed.expect("a base is formed");
    assert_eq!(run.at(3).bases[&formed].side, Side::Japanese);
    assert_eq!(run.at(3).bases[&formed].hex, P);
    assert_eq!(run.marched_days(3).last().expect("marched").0, 3);
    assert!(matches!(run.marched_days(3)[0].2, Cause::Order(_)));
    // Supply at R: base 2 paid 1.5 t on each of the nights of days 1, 2 and
    // 3 and gave 7.5 t when unit 3 left: 88 t after day 3.
    assert_eq!(run.supply_at(2, 2), fx("97"));
    assert_eq!(run.supply_at(3, 2), n(88));
    assert_eq!(run.supply_at(1, 2), fx("98.5"));

    // The fight: the Japanese side attacks, and the Allied side does not.
    let fight = run.fights(3, P);
    assert_eq!(fight.len(), 1);
    assert_eq!(
        fight[0].kind,
        EventKind::GroundFight {
            hex: P,
            allied_attacks: false,
            japanese_attacks: true,
        }
    );
    // Unit 3: fed in full at R the night before; its fatigue at the fight
    // is 20, from two days of march: 240 firing men, 2.4 shots at 0.3.
    assert_eq!(run.fed(3, Side::Japanese, R), Some((Some(2), 300, 300, 0)));
    assert_eq!(run.unit_at(2, 3).fatigue, n(20));
    let f3 = run.fire(3, P, 3);
    assert_eq!(
        (f3.firing, f3.shots, f3.chance),
        (240, fx("2.4"), fx("0.3"))
    );
    // Unit 2: its fatigue is 15, so its firing men are its men times
    // 85/100, far above the limit of 3 x 300 = 900: 9 shots at 0.6.
    assert_eq!(run.unit_at(2, 2).fatigue, n(15));
    let men2 = run.men(2, 2);
    let f2 = run.fire(3, P, 2);
    assert_eq!(f2.firing, share(men2, 85, 100));
    assert_eq!((f2.shots, f2.chance), (n(9), fx("0.6")));
    let e2 = fire_event(&run, 3, 2);
    assert_eq!(factor(e2, "engaged_men"), Some(n(900)));
    assert_eq!(factor(e2, "side_firing_men"), Some(n(i64::from(f2.firing))));
    assert_eq!(factor(e2, "shots_before_limit"), Some(shots_of(f2.firing)));
    // Unit 1 is falling back and the Allied side is not on attack in the
    // hex: it does not fire, it is not counted, and it takes no share of the
    // losses.
    assert!(matches!(
        run.unit_at(2, 1).march.as_ref().expect("marching").cause,
        Cause::Event(_)
    ));
    assert_eq!(run.unit_at(2, 1).hex, Some(P));
    assert_eq!(run.fires(3, P).len(), 2);
    assert!(run.fires(3, P).iter().all(|f| f.unit != 1));
    assert_eq!(run.lost_in_fight(3, 1), 0);
    // The only men it lost on days 2 to 4 are the unfed.
    assert_eq!(run.men(3, 1), men1 - lost);
    // Let c be unit 2's hits on day 3: unit 3 loses 20 c, at most 180.
    let c = f2.hits;
    assert!(c <= 9);
    assert_eq!(run.lost_in_fight(3, 3), 20 * c);
    assert_eq!(run.men(3, 3), 300 - 20 * c);
    assert_eq!(run.lost_in_fight(3, 2), 20 * f3.hits);
    assert_eq!(run.men(3, 2), men2 - 20 * f3.hits);
    // Unit 3's fatigue after day 3 is 20 + 10 for the march + 20 for the
    // attack: 50. It is not above 60 and stays on attack.
    assert_eq!(
        run.fatigue(3, 3),
        vec![(LandFatigueChange::MarchedAndAttacked, n(50))]
    );
    assert_eq!(run.unit_at(3, 3).fatigue, n(50));
    assert_eq!(run.unit_at(3, 3).posture, Posture::Attack);
    assert!(run.back_to_defend(3).iter().all(|e| e.day != 3));
    // Unit 2 rests: 15 to 10.
    assert_eq!(run.unit_at(3, 2).fatigue, n(10));
    // Unit 1's fatigue goes on: 40 + 10.
    assert_eq!(run.fatigue(3, 1), vec![(LandFatigueChange::Marched, n(50))]);
    // The Japanese side is not tested: the Allied side did not attack.
    assert!(run
        .on(
            3,
            |k| matches!(k, EventKind::SideBeatenAshore { hex, .. } if *hex == P)
        )
        .is_empty());
}

#[test]
fn p_day_3_without_the_correction_unit_1_would_have_shared_the_loss_and_the_limit() {
    // The same day's numbers read the other way: the limit is 3 x 300 and
    // not 3 x (300 + unit 1's men), and the whole loss falls on unit 3.
    let run = Run::live();
    let men1 = run.men(2, 1) - unfed_loss(run.men(2, 1));
    let e2 = fire_event(&run, 3, 2);
    assert_ne!(
        factor(e2, "engaged_men"),
        Some(n(3 * (300 + i64::from(men1))))
    );
    assert_eq!(factor(e2, "engaged_men"), Some(n(3 * 300)));
    let c = run.fire(3, P, 2).hits;
    assert_eq!(run.lost_in_fight(3, 3), 20 * c);
}

#[test]
fn p_day_4_unit_1_arrives_at_r_and_the_fight_goes_on_between_units_3_and_2() {
    let run = Run::live();
    // Unit 1 marches its third day and arrives at R, where base 2 feeds it
    // from the night of day 5.
    let (_, to, men, carried, formed) = run.arrived(1).expect("unit 1 arrives");
    assert_eq!(to, R);
    assert_eq!(men, run.men(3, 1));
    assert_eq!((carried, formed), (n(0), None));
    assert_eq!(run.arrived(1).expect("arrives").0.day, 4);
    assert_eq!(run.unit_at(4, 1).hex, Some(R));
    assert_eq!(run.fed(5, Side::Japanese, R), Some((Some(2), men, men, 0)));
    assert!(run.unit_at(4, 1).march.is_none());
    assert_eq!(run.fatigue(4, 1), vec![(LandFatigueChange::Marched, n(60))]);
    assert_eq!(run.fatigue(5, 1), vec![(LandFatigueChange::Rested, n(55))]);
    // Unit 3 has men left (at least 120) and is on attack: the fight goes
    // on, with the limit at three times unit 3's men. Unit 1 is not in the
    // hex.
    let m3 = run.men(3, 3);
    assert!(m3 >= 120);
    assert_eq!(run.unit_at(3, 3).posture, Posture::Attack);
    assert_eq!(run.fights(4, P).len(), 1);
    assert_eq!(run.fires(4, P).len(), 2);
    // Unit 3: fed in full by the dump, fatigue 50 at the fight.
    assert_eq!(run.unit_at(3, 3).fatigue, n(50));
    let f3 = run.fire(4, P, 3);
    assert_eq!(f3.firing, share(m3, 50, 100));
    assert_eq!((f3.shots, f3.chance), (shots_of(f3.firing), fx("0.3")));
    // Unit 2: fatigue 10, its firing men far above the limit 3 x m3.
    let m2 = run.men(3, 2);
    let f2 = run.fire(4, P, 2);
    assert_eq!(f2.firing, share(m2, 90, 100));
    assert!(f2.firing > 3 * m3);
    assert_eq!((f2.shots, f2.chance), (shots_of(3 * m3), fx("0.6")));
    // Each loss is 20 a hit, unit 3 to no fewer than none.
    assert_eq!(run.lost_in_fight(4, 3), (20 * f2.hits).min(m3));
    assert_eq!(run.lost_in_fight(4, 2), 20 * f3.hits);
    assert_eq!(run.men(4, 3), m3 - (20 * f2.hits).min(m3));
    // Unit 3's fatigue is 50 + 20 = 70 (above 60): it goes back to defend.
    assert_eq!(run.fatigue(4, 3), vec![(LandFatigueChange::Gained, n(70))]);
    assert_eq!(run.back_to_defend(3).len(), 1);
    assert_eq!(run.back_to_defend(3)[0].day, 4);
    assert_eq!(run.unit_at(4, 3).posture, Posture::Defend);
    assert_eq!(run.unit_at(4, 2).fatigue, n(5));
}

#[test]
fn p_days_5_and_6_nobody_is_on_attack_so_there_is_no_fight() {
    let run = Run::live();
    for day in [5, 6] {
        assert!(run.fights(day, P).is_empty(), "day {day}");
        assert!(run.fires(day, P).is_empty());
        assert_eq!(run.lost_in_fight(day, 2), 0);
        assert_eq!(run.lost_in_fight(day, 3), 0);
    }
    assert_eq!(run.unit_at(5, 2).fatigue, n(0));
    assert_eq!(run.fatigue(5, 3), vec![(LandFatigueChange::Rested, n(65))]);
    assert_eq!(run.fatigue(6, 3), vec![(LandFatigueChange::Rested, n(60))]);
}

// --------------------------------------------------------------------- At T

#[test]
fn t_day_1_is_as_at_p_base_4_goes_over_and_unit_4_falls_back_toward_u() {
    let run = Run::live();
    let f5 = run.fire(1, T, 5);
    assert_eq!((f5.firing, f5.shots, f5.chance), (6000, n(30), fx("0.3")));
    let f4 = run.fire(1, T, 4);
    assert_eq!((f4.firing, f4.shots, f4.chance), (1000, n(10), fx("0.6")));
    assert_eq!(run.lost_in_fight(1, 4), 20 * f5.hits);
    assert_eq!(run.lost_in_fight(1, 5), 20 * f4.hits);
    assert_eq!(run.men(1, 4), 1000 - 20 * f5.hits);
    assert_eq!(run.men(1, 5), 6000 - 20 * f4.hits);
    assert_eq!(run.unit_at(1, 5).fatigue, n(20));
    let beaten = run.all(|k| matches!(k, EventKind::SideBeatenAshore { hex, .. } if *hex == T));
    assert_eq!(beaten[0].day, 1);
    assert_eq!(run.at(1).bases[&base(4)].side, Side::Allied);
    assert_eq!(run.at(6).bases[&base(5)].side, Side::Japanese);
    let fell = fell_back(&run, 4);
    assert_eq!(fell.len(), 1);
    assert_eq!(
        fell[0].kind,
        EventKind::UnitFellBack {
            unit: unit(4),
            side: Side::Japanese,
            from: T,
            to: U,
            men: run.men(1, 4),
            fatigue: n(30),
        }
    );
    assert_eq!(run.unit_at(1, 4).march.as_ref().expect("march").to, U);
}

/// Days 2 and 3 at T: unit 5 is still on attack and the hex is contested;
/// unit 4 is falling back and the side that beat it is on attack, so it is
/// counted and fired on, as before.
fn t_day(run: &Run, day: u32, rested_share: u32, fatigue_5_after: i64) {
    let before = run.men(day - 1, 4);
    let lost = unfed_loss(before);
    // m is unit 4's men before the day's fight, after that night's loss.
    let m = before - lost;
    assert!(m > 0);
    assert_eq!(
        run.fed(day, Side::Japanese, T),
        Some((None, before, 0, before))
    );
    assert_eq!(run.lost_unfed(day, 4), Some((lost, m)));
    assert_eq!(run.unit_at(day, 4).fed_share, n(0));
    assert_eq!(run.fights(day, T).len(), 1);
    assert_eq!(run.fires(day, T).len(), 2);
    // Unit 4 is counted and fires nothing: its fed share is 0.
    let f4 = run.fire(day, T, 4);
    assert_eq!((f4.firing, f4.shots, f4.hits), (0, n(0), 0));
    // Unit 5's firing men are its men times rested/100, far above the limit
    // of 3 m: its shots are 3 m over 100, at 0.3.
    let m5 = run.men(day - 1, 5);
    let f5 = run.fire(day, T, 5);
    assert_eq!(f5.firing, share(m5, rested_share, 100));
    assert!(f5.firing > 3 * m);
    assert_eq!((f5.shots, f5.chance), (shots_of(3 * m), fx("0.3")));
    let e5 = fire_event(run, day, 5);
    assert_eq!(factor(e5, "engaged_men"), Some(n(3 * i64::from(m))));
    // Unit 4 loses 20 for each hit, or all it has; and is fired on.
    assert_eq!(run.lost_in_fight(day, 4), (20 * f5.hits).min(m));
    assert_eq!(run.men(day, 4), m - (20 * f5.hits).min(m));
    assert_eq!(run.lost_in_fight(day, 5), 0);
    assert_eq!(run.unit_at(day, 5).fatigue, n(fatigue_5_after));
    // Beaten again, it is left as it is: no new falling back.
    assert_eq!(fell_back(run, 4).len(), 1);
    assert_eq!(
        run.on(
            day,
            |k| matches!(k, EventKind::SideBeatenAshore { hex, .. } if *hex == T)
        )
        .len(),
        1
    );
    assert_eq!(run.unit_at(day, 4).hex, Some(T));
}

#[test]
fn t_days_2_and_3_unit_4_is_counted_and_fired_on_as_before() {
    let run = Run::live();
    t_day(&run, 2, 80, 40);
    t_day(&run, 3, 60, 60);
    // 60 is not above the limit: unit 5 stays on attack.
    assert_eq!(run.unit_at(3, 5).posture, Posture::Attack);
    assert!(run.back_to_defend(5).is_empty());
}

#[test]
fn t_day_4_unit_4_arrives_at_u_before_the_fight_there_is_none_and_unit_5_rests_to_55() {
    let run = Run::live();
    let (event, to, men, carried, formed) = run.arrived(4).expect("unit 4 arrives");
    assert_eq!((event.day, to, carried, formed), (4, U, n(0), None));
    assert_eq!(men, run.men(3, 4) - unfed_loss(run.men(3, 4)));
    assert_eq!(run.unit_at(4, 4).hex, Some(U));
    assert!(run.fights(4, T).is_empty());
    assert!(run.fires(4, T).is_empty());
    assert_eq!(run.unit_at(4, 5).fatigue, n(55));
    assert_eq!(run.fatigue(4, 5), vec![(LandFatigueChange::Rested, n(55))]);
    assert_eq!(run.unit_at(4, 5).posture, Posture::Attack);
}

// --------------------------------------------------------------- Properties

/// The scenario with unit 2 on defend from day 1, so that unit 1 is never
/// beaten and no unit of the Japanese side is falling back at P.
fn unit_2_defends(source: &str) -> String {
    common::replace_once(
        source,
        "name = \"Placeholder Allied division at P\"\nside = \"allied\"\nmen = 6000\nq = 0\nr = 0\nposture = \"attack\"",
        "name = \"Placeholder Allied division at P\"\nside = \"allied\"\nmen = 6000\nq = 0\nr = 0\nposture = \"defend\"",
    )
}

#[test]
fn property_a_unit_not_falling_back_shares_in_every_fight_and_counts_for_the_limit() {
    let run = Run::text(&unit_2_defends(&text()));
    // Nobody falls back at P; unit 1 is never beaten.
    assert!(fell_back(&run, 1).is_empty());
    assert!(run.fights(1, P).is_empty());
    assert!(run.fights(2, P).is_empty());
    assert_eq!(run.men(2, 1), 1000);
    assert_eq!(run.at(2).bases[&base(1)].side, Side::Japanese);
    assert_eq!(run.unit_at(2, 1).fed_share, n(1));
    // Unit 3 attacks on day 3, on its third day of march.
    assert_eq!(run.arrived(3).expect("arrives").0.day, 3);
    assert_eq!(run.fights(3, P).len(), 1);
    let m1 = run.men(2, 1);
    let m3 = run.men(2, 3);
    assert_eq!((m1, m3), (1000, 300));
    // Both count for the limit: 3 x (unit 1's men + unit 3's men).
    let e2 = fire_event(&run, 3, 2);
    assert_eq!(factor(e2, "engaged_men"), Some(n(3 * i64::from(m1 + m3))));
    let f2 = run.fire(3, P, 2);
    assert_eq!(f2.shots, shots_of(3 * (m1 + m3)));
    // Both fire: 1000 men and 240 men.
    let f1 = run.fire(3, P, 1);
    assert_eq!((f1.firing, f1.shots), (1000, n(10)));
    let f3 = run.fire(3, P, 3);
    assert_eq!((f3.firing, f3.shots), (240, fx("2.4")));
    // And share the losses in proportion to their men.
    let c = f2.hits;
    let parts = in_proportion(20 * c, &[m1, m3]);
    assert_eq!(run.lost_in_fight(3, 1), parts[0]);
    assert_eq!(run.lost_in_fight(3, 3), parts[1]);
    assert_eq!(parts[0] + parts[1], 20 * c);
    assert_eq!(run.men(3, 1), m1 - parts[0]);
    assert_eq!(run.men(3, 3), m3 - parts[1]);
}

// Property: "with the frozen engagement_ratio, and no unit falling back in a
// hex where its own side attacks, every scenario gives what it gave before,
// apart from the hash version." This is checked by the team lead with
// `scripts/compare-scenarios.sh`; no test here.

// Property: "the enemy's view is the same in two worlds that differ only in
// whether a unit it cannot see is falling back or standing, where the hits
// are the same." A falling-back unit is made by setting a `March` on a unit
// of a world, which the scenario data cannot do and an integration test has
// no public way to do; it is a unit test of sim-core, in
// `crates/sim-core/src/ground_combat/tests.rs`:
// `the_enemys_view_is_the_same_whether_a_unit_is_falling_back_or_standing`.
