//! Scenario 33: who is engaged in a fight ashore, and an attack that fails.
//! A large side fires with no more men than three times the enemy's; an
//! attack whose weight of fire is the weaker fails and wears its men out;
//! tired attackers hold or fall back as their order says, and with no
//! instruction hold where they can be fed and fall back where they cannot.
//!
//! Proves decision 0105 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/33_engaged_and_failed.toml`. The frozen step 1
//! figures hold `engagement_ratio` at 1000000 and `failed_attack_fatigue`
//! at 0, so this test gives them the values below, as the test of ships'
//! experience sets its figure.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide (the
//! hits of a fight), the calculation gives the firing men, the shots, the
//! chance and the weight of fire exactly, and the rule for what follows;
//! the tests read the hits from the run. Everything else is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! `engagement_ratio` 3, `failed_attack_fatigue` 30. From the earlier
//! rules: `men_per_shot` 100, `hit_chance` 0.3, `defender_hit_chance` 0.6,
//! `men_per_hit` 20, `fatigue_per_attack_day` 20, `rest_per_day` 5,
//! `attack_fatigue_limit` 60, `beaten_share` 0.2, `beaten_fatigue` 30,
//! `march_days_per_hex` 3, `fatigue_per_march_day` 10,
//! `daily_supply_t_per_man` 0.005, `unfed_loss_share_per_day` 0.05. The
//! strengths of experience and of the skills are 0, and no base has
//! defences.
//!
//! A unit's firing men are its men ashore times its fed share, times 100
//! less its fatigue over 100, cut to a whole man. A side's firing men count
//! up to 3 times the other side's fighting men ashore in the hex at the
//! start of the fight; above that each of its units' shots are cut in
//! proportion. A side's weight of fire is the sum of its units' shots times
//! their chance, after the cut. A side that attacked and whose weight is
//! below the other's has failed that day.
//!
//! In the ground combat phase the order is: the marches, the fight, the
//! day's fatigue (with the fatigue of a failed day), the tired back to
//! defend, what a spent attacker does, the test of a side beaten.
//!
//! ## Inputs
//!
//! A placeholder map. Every hex is a coast hex and clear. The game is 6
//! days.
//!
//! - K at (0,0), with no land link.
//! - L at (10,0), with a land link to M at (10,1).
//! - N at (20,0), with a land link to O at (20,1).
//! - P at (30,0), with no land link.
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Allied   | K   | 5000 t |
//! | 2    | Japanese | K   | 1000 t |
//! | 3    | Allied   | L   | 5000 t |
//! | 4    | Japanese | L   | 10 t   |
//! | 5    | Japanese | M   | 100 t  |
//! | 6    | Allied   | N   | 5000 t |
//! | 7    | Japanese | N   | 2.5 t  |
//! | 8    | Allied   | P   | 5000 t |
//! | 9    | Japanese | P   | 1000 t |
//!
//! | Unit | Side     | Men  | Hex | Posture | Fatigue at the start | Instruction when spent |
//! | ---- | -------- | ---- | --- | ------- | -------------------- | ---------------------- |
//! | 1    | Allied   | 6000 | K   | defend  | 0                    |                        |
//! | 2    | Japanese | 1000 | K   | attack  | 0                    | none                   |
//! | 3    | Allied   | 3000 | L   | defend  | 0                    |                        |
//! | 4    | Japanese | 500  | L   | attack  | 40                   | fall back              |
//! | 5    | Allied   | 3000 | N   | defend  | 0                    |                        |
//! | 6    | Japanese | 500  | N   | attack  | 40                   | none                   |
//! | 7    | Allied   | 4000 | P   | attack  | 0                    | none                   |
//! | 8    | Japanese | 1600 | P   | defend  | 0                    |                        |
//!
//! Units 2, 4, 6 and 7 are set to attack by an order of day 1, which
//! carries the instruction of the table. There is no other order. Every
//! unit is fed in full on the night of day 1.
//!
//! ## At K: the limit, and an attack that fails twice and holds
//!
//! Day 1. The Japanese side attacks.
//!
//! - Unit 2: 1000 firing men, 10 shots at 0.3. Weight 3.
//! - Unit 1: 6000 firing men. The limit is 3 x 1000 = 3000, so its 60 shots
//!   are cut to 60 x 3000/6000 = 30, at 0.6. Weight 18.
//!
//! The Japanese attack fails: 3 is below 18. Unit 2's fatigue is 0 + 20 +
//! 30 = 50. That is not above 60: it stays on attack.
//!
//! Let J be unit 2's men after day 1 and A unit 1's, read from the run. J
//! is at least 1000 - 30 x 20 = 400, and A at least 6000 - 10 x 20 = 5800.
//!
//! Day 2.
//!
//! - Unit 2: firing men J x 50/100, cut. Its shots are those over 100, at
//!   0.3.
//! - Unit 1: A firing men, limited to 3 x J, which is below A whatever the
//!   dice gave. Its shots are 3 x J over 100, at 0.6.
//!
//! The attack fails again whatever the dice gave: unit 1's weight is 1.8 x
//! J/100 and unit 2's at most 0.15 x J/100. Unit 2's fatigue is 50 + 20 +
//! 30 = 100. It is above 60: unit 2 goes back to defend. Its order carries
//! no instruction, and base 2 at K holds supply: it holds.
//!
//! Days 3 to 6: no unit at K is on attack and there is no fight. Unit 2's
//! fatigue is 95, 90, 85 and 80.
//!
//! The Allied side never attacks at K, so the Japanese side is not tested
//! for being beaten.
//!
//! ## At L: a failed attack, and an order to fall back
//!
//! Night of day 1: base 4 pays 500 x 0.005 = 2.5 t and holds 7.5 t.
//!
//! Day 1.
//!
//! - Unit 4: firing men 500 x 60/100 = 300. 3 shots at 0.3. Weight 0.9.
//! - Unit 3: 3000 firing men, limited to 3 x 500 = 1500: 30 x 1500/3000 =
//!   15 shots at 0.6. Weight 9.
//!
//! The attack fails. Unit 4's fatigue is 40 + 20 + 30 = 90, above 60: it
//! goes back to defend. Its order says fall back, so it does, though base 4
//! holds supply: it is set to march for M, the one linked hex, which holds
//! base 5. The falling back adds 30: its fatigue is 100, the most there is.
//!
//! Let m be unit 4's men after the fight of day 1, read from the run. It is
//! at least 500 - 15 x 20 = 200.
//!
//! | Day | Night: fed                         | In the phase                | Fatigue after |
//! | --- | ---------------------------------- | --------------------------- | ------------- |
//! | 1   | by base 4, 2.5 t                   | fights, fails, falls back   | 100           |
//! | 2   | by base 4, m x 0.005 t             | marches, day 1 of 3         | 100           |
//! | 3   | by base 4, m x 0.005 t             | marches, day 2 of 3         | 100           |
//! | 4   | by base 4, m x 0.005 t             | marches, day 3: arrives at M | 100          |
//! | 5   | by base 5, m x 0.005 t             | rests                       | 95            |
//! | 6   | by base 5, m x 0.005 t             | rests                       | 90            |
//!
//! - Unit 3 is on defend and nobody attacks at L after day 1: there is no
//!   fight on days 2 to 4 while unit 4 is still there, and it loses no man
//!   after day 1.
//! - Base 4 holds 7.5 - 3 x m x 0.005 t at the end, which is 0 or more
//!   whatever the dice gave, and is still Japanese: nobody attacked it.
//! - Unit 4 carries nothing to M. Base 5 holds 100 - 2 x m x 0.005 t at
//!   the end.
//!
//! ## At N: a failed attack, no instruction, and nothing to eat
//!
//! Night of day 1: base 7 pays 2.5 t, all it has, and holds 0 t. Unit 6 is
//! fed in full.
//!
//! Day 1.
//!
//! - Unit 6: firing men 500 x 60/100 = 300. 3 shots at 0.3. Weight 0.9.
//! - Unit 5: limited to 1500: 15 shots at 0.6. Weight 9.
//!
//! The attack fails. Unit 6's fatigue is 90: it goes back to defend. Its
//! order carries no instruction. Its side's only base at N holds no supply,
//! so it cannot be fed where it stands: it falls back. O is the one linked
//! hex and holds no enemy: it marches for O, and its fatigue is 100.
//!
//! Let n be unit 6's men after the fight of day 1. On each of the nights of
//! days 2, 3 and 4 it is unfed at N and loses its men times 0.05, cut to a
//! whole man and one man at least. It arrives at O in the phase of day 4,
//! carrying nothing, and forms no base. On the nights of days 5 and 6 it is
//! unfed at O and loses the same share. The tests work each night's loss
//! from the men the run has before it. Its fatigue is 100 after days 1 to
//! 4, 95 after day 5 and 90 after day 6.
//!
//! ## At P: an attack that does not fail
//!
//! Day 1. The Allied side attacks.
//!
//! - Unit 7: 4000 firing men. The limit is 3 x 1600 = 4800, which does not
//!   cut. 40 shots at 0.3. Weight 12.
//! - Unit 8: 1600 firing men, 16 shots at 0.6. Weight 9.6.
//!
//! The attack does not fail: 12 is not below 9.6. Unit 7's fatigue is 20,
//! and nothing is added.
//!
//! The Japanese side is not beaten on day 1 whatever the dice give: unit 8
//! has at least 1600 - 40 x 20 = 800 men, and 0.2 times unit 7's men is at
//! most 800.
//!
//! On each later day the tests work both weights from the men, fed shares
//! and fatigue the run has before the fight, and assert: the limit where 3
//! times the enemy's men is below a side's firing men; that unit 7 gains 20
//! on a day it attacks and its weight is not the lower, and 50 on a day it
//! is; that it goes back to defend when its fatigue is above 60, and then
//! holds, since base 8 holds supply; and, if the Japanese side is beaten,
//! that its men surrender, P having no land link (decision 0104, 7).
//!
//! # Properties
//!
//! - A side's shots are never more than they were without the limit, and
//!   the smaller side's are never cut.
//! - With the frozen figures every scenario gives what it gave before,
//!   apart from the hash version.
//! - An instruction to hold keeps a spent attacker in the hex where the
//!   default would have it fall back: unit 6 told to hold stays at N.
//! - An instruction to fall back where no hex can be fallen back to leaves
//!   the unit in the hex, on the defence, and it does not surrender: unit 2
//!   told to fall back stays at K.
//! - The enemy's view is the same in two worlds that differ only in the
//!   instruction the other side's attack orders carry, until a unit has
//!   left the hex.

mod common;

use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind, LandFatigueChange, SupplyUse};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, LandUnitId};
use sim_core::orders::Posture;
use sim_core::state::{LandUnit, Side, TruthState};
use sim_data::{LoadOptions, Scenario};

const K: Hex = Hex::new(0, 0);
const L: Hex = Hex::new(10, 0);
const M: Hex = Hex::new(10, 1);
const N: Hex = Hex::new(20, 0);
const O: Hex = Hex::new(20, 1);
const P: Hex = Hex::new(30, 0);

fn path() -> PathBuf {
    common::data_dir().join("scenarios/33_engaged_and_failed.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// The frozen step 1 figures with the two figures of this calculation: the
/// ratio 3 and the fatigue of a failed day 30.
fn figures(tag: &str) -> LoadOptions {
    common::altered_reference(tag, |dir| {
        common::edit_file_by(
            dir,
            "ground.toml",
            &[
                ("engagement_ratio = 1000000", "engagement_ratio = 3"),
                ("failed_attack_fatigue = 0", "failed_attack_fatigue = 30"),
            ],
        );
    })
}

fn from(source: &str) -> Scenario {
    common::from_text_with(source, &path(), &figures("s33")).expect("the scenario loads")
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

    /// The men of `side` ashore at `hex` at the end of `day`.
    fn men_at(&self, day: u32, side: Side, hex: Hex) -> u32 {
        self.at(day)
            .land_units
            .values()
            .filter(|u| u.side == side && u.hex == Some(hex))
            .map(|u| u.men_ashore)
            .sum()
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

    fn index_of(&self, event: &Event) -> usize {
        self.events
            .iter()
            .position(|e| e.id == event.id)
            .expect("the event is in the log")
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

    /// The `GroundAttackFailed` event of a side at a hex on a day.
    fn failed(&self, day: u32, side: Side, hex: Hex) -> Option<&Event> {
        let found = self.on(day, |k| {
            matches!(k, EventKind::GroundAttackFailed { side: s, hex: h, .. } if *s == side && *h == hex)
        });
        assert!(found.len() <= 1);
        found.first().copied()
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

    fn spent_fell_back(&self, raw: u32) -> Vec<&Event> {
        self.all(|k| matches!(k, EventKind::SpentUnitFellBack { unit, .. } if unit.raw() == raw))
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

    fn last_march_day(&self, raw: u32) -> &Event {
        self.all(|k| matches!(k, EventKind::MarchDay { unit, .. } if unit.raw() == raw))
            .last()
            .copied()
            .expect("it marched")
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

    /// No unit ever surrendered and no side was beaten.
    fn nobody_surrendered_or_beaten(&self) {
        assert!(self
            .all(|k| matches!(
                k,
                EventKind::UnitSurrendered { .. }
                    | EventKind::MenSurrenderedAshore { .. }
                    | EventKind::SideBeatenAshore { .. }
                    | EventKind::BaseChangedSide { .. }
            ))
            .is_empty());
    }
}

/// The text of `source` with unit `raw`'s attack order carrying `when_spent`.
fn with_instruction(source: &str, raw: u32, when_spent: &str) -> String {
    let order = format!("land_unit = {raw}\nkind = \"set_posture\"\nposture = \"attack\"\n");
    common::replace_once(
        source,
        &order,
        &format!("{order}when_spent = \"{when_spent}\"\n"),
    )
}

/// The same with the instruction taken out.
fn without_instruction(source: &str, raw: u32) -> String {
    common::replace_once(
        source,
        &format!(
            "land_unit = {raw}\nkind = \"set_posture\"\nposture = \"attack\"\nwhen_spent = \"fall_back\"\n"
        ),
        &format!("land_unit = {raw}\nkind = \"set_posture\"\nposture = \"attack\"\n"),
    )
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_figures_and_the_inputs_are_the_calculations() {
    let run = Run::live();
    let figures = run.scenario.world.ref_data().ground.expect("ground");
    assert_eq!(figures.engagement_ratio, n(3));
    assert_eq!(figures.failed_attack_fatigue, n(30));
    assert_eq!(figures.men_per_shot, n(100));
    assert_eq!(figures.hit_chance, fx("0.3"));
    assert_eq!(figures.defender_hit_chance, fx("0.6"));
    assert_eq!(figures.men_per_hit, n(20));
    assert_eq!(figures.fatigue_per_attack_day, n(20));
    assert_eq!(figures.rest_per_day, n(5));
    assert_eq!(figures.attack_fatigue_limit, n(60));
    assert_eq!(figures.beaten_share, fx("0.2"));
    assert_eq!(figures.beaten_fatigue, n(30));
    assert_eq!(figures.march_days_per_hex, 3);
    assert_eq!(figures.fatigue_per_march_day, n(10));
    assert_eq!(figures.daily_supply_t_per_man, fx("0.005"));
    assert_eq!(figures.unfed_loss_share_per_day, fx("0.05"));
    assert_eq!(figures.experience_fire_strength, n(0));

    assert_eq!(run.scenario.days, 6);
    let start = run.at(0);
    assert_eq!(start.land_units.len(), 8);
    assert_eq!(start.bases.len(), 9);
    let bases = [
        (1, Side::Allied, K, n(5000)),
        (2, Side::Japanese, K, n(1000)),
        (3, Side::Allied, L, n(5000)),
        (4, Side::Japanese, L, n(10)),
        (5, Side::Japanese, M, n(100)),
        (6, Side::Allied, N, n(5000)),
        (7, Side::Japanese, N, fx("2.5")),
        (8, Side::Allied, P, n(5000)),
        (9, Side::Japanese, P, n(1000)),
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
        (1, Side::Allied, 6000, K, 0),
        (2, Side::Japanese, 1000, K, 0),
        (3, Side::Allied, 3000, L, 0),
        (4, Side::Japanese, 500, L, 40),
        (5, Side::Allied, 3000, N, 0),
        (6, Side::Japanese, 500, N, 40),
        (7, Side::Allied, 4000, P, 0),
        (8, Side::Japanese, 1600, P, 0),
    ];
    for (id, side, men, hex, fatigue) in units {
        let u = run.unit_at(0, id);
        assert_eq!(
            (u.side, u.working, u.men_ashore, u.hex, u.fatigue),
            (side, false, men, Some(hex), n(fatigue)),
            "unit {id}"
        );
        // Every unit starts on the defence: the attacks are by order.
        assert_eq!(u.posture, Posture::Defend, "unit {id}");
        assert!(u.march.is_none());
    }
    // The land links the calculation leans on.
    let map = run.scenario.world.map();
    assert!(map.land_neighbours(K).is_empty());
    assert_eq!(map.land_neighbours(L), vec![M]);
    assert_eq!(map.land_neighbours(N), vec![O]);
    assert_eq!(map.land_neighbours(M), vec![L]);
    assert_eq!(map.land_neighbours(O), vec![N]);
    assert!(map.land_neighbours(P).is_empty());
    // The orders of day 1 set units 2, 4, 6 and 7 to attack: each fight of
    // day 1 has the attacker the table names, and no other fights.
    let flags = |hex: Hex| match run.fights(1, hex)[0].kind {
        EventKind::GroundFight {
            allied_attacks,
            japanese_attacks,
            ..
        } => (allied_attacks, japanese_attacks),
        _ => unreachable!(),
    };
    assert_eq!(flags(K), (false, true));
    assert_eq!(flags(L), (false, true));
    assert_eq!(flags(N), (false, true));
    assert_eq!(flags(P), (true, false));
    assert_eq!(
        run.on(1, |k| matches!(k, EventKind::GroundFight { .. }))
            .len(),
        4
    );
    // Every unit is fed in full on the night of day 1.
    for (hex, side, men) in [
        (K, Side::Allied, 6000),
        (K, Side::Japanese, 1000),
        (L, Side::Allied, 3000),
        (L, Side::Japanese, 500),
        (N, Side::Allied, 3000),
        (N, Side::Japanese, 500),
        (P, Side::Allied, 4000),
        (P, Side::Japanese, 1600),
    ] {
        let fed = run.fed(1, side, hex).expect("fed");
        assert_eq!((fed.1, fed.2, fed.3), (men, men, 0), "{side:?} at {hex:?}");
    }
}

// --------------------------------------------------------------------- At K

#[test]
fn k_day_1_the_japanese_attack_is_limited_fails_and_costs_50_fatigue() {
    let run = Run::live();
    let fight = run.fights(1, K);
    assert_eq!(fight.len(), 1);
    // Unit 2: 1000 firing men, 10 shots at 0.3. Unit 1: 6000 firing men,
    // cut to 3 x 1000: 30 shots at 0.6.
    let f2 = run.fire(1, K, 2);
    assert_eq!((f2.firing, f2.shots, f2.chance), (1000, n(10), fx("0.3")));
    let f1 = run.fire(1, K, 1);
    assert_eq!((f1.firing, f1.shots, f1.chance), (6000, n(30), fx("0.6")));
    assert_eq!(run.fires(1, K).len(), 2);
    // The attack fails: 3 is below 18.
    let failed = run.failed(1, Side::Japanese, K).expect("the attack fails");
    assert_eq!(
        failed.kind,
        EventKind::GroundAttackFailed {
            side: Side::Japanese,
            hex: K,
            weight: n(3),
            enemy_weight: n(18),
        }
    );
    assert!(run.failed(1, Side::Allied, K).is_none());
    // Each loss is 20 a hit.
    assert_eq!(run.lost_in_fight(1, 1), 20 * f2.hits);
    assert_eq!(run.lost_in_fight(1, 2), 20 * f1.hits);
    assert_eq!(run.men(1, 1), 6000 - 20 * f2.hits);
    assert_eq!(run.men(1, 2), 1000 - 20 * f1.hits);
    // J is at least 400 and A at least 5800.
    assert!(run.men(1, 2) >= 400 && run.men(1, 1) >= 5800);
    // The fatigue is 0 + 20 + 30 = 50, not above 60: it stays on attack.
    assert_eq!(run.fatigue(1, 2), vec![(LandFatigueChange::Gained, n(50))]);
    assert_eq!(run.unit_at(1, 2).fatigue, n(50));
    assert_eq!(run.unit_at(1, 2).posture, Posture::Attack);
    assert!(run.back_to_defend(2).iter().all(|e| e.day == 2));
    // The failed attack is in the cause of the fatigue.
    let gained = run.on(
        1,
        |k| matches!(k, EventKind::LandUnitFatigue { unit, .. } if unit.raw() == 2),
    );
    let factors: Vec<(&str, Fixed)> = gained[0]
        .breakdown
        .as_ref()
        .expect("a breakdown")
        .factors
        .iter()
        .map(|f| (f.name, f.value))
        .collect();
    assert!(factors.contains(&("failed_attack_fatigue", n(30))));
    assert!(factors.contains(&("fatigue_per_attack_day", n(20))));
    assert!(gained[0].causes.contains(&Cause::Event(failed.id)));
}

#[test]
fn k_day_2_the_attack_fails_again_unit_2_is_at_100_and_holds() {
    let run = Run::live();
    let j = run.men(1, 2);
    let a = run.men(1, 1);
    // Fed in full on the night of day 2.
    assert_eq!(run.fed(2, Side::Japanese, K), Some((Some(2), j, j, 0)));
    assert_eq!(run.fed(2, Side::Allied, K), Some((Some(1), a, a, 0)));
    // Unit 2: J x 50/100 firing men, shots over 100, at 0.3.
    let f2 = run.fire(2, K, 2);
    assert_eq!(f2.firing, share(j, 50, 100));
    assert_eq!(
        (f2.shots, f2.chance),
        (shots_of(share(j, 50, 100)), fx("0.3"))
    );
    // Unit 1: A firing men, limited to 3 x J, which is below A: 3J over
    // 100 shots at 0.6.
    assert!(3 * j < a);
    let f1 = run.fire(2, K, 1);
    assert_eq!(f1.firing, a);
    assert_eq!((f1.shots, f1.chance), (shots_of(3 * j), fx("0.6")));
    // The attack fails whatever the dice gave.
    let w2 = f2.shots * fx("0.3");
    let w1 = f1.shots * fx("0.6");
    assert!(w2 < w1);
    assert!(w1 >= shots_of(3 * j) * fx("0.6"));
    let failed = run.failed(2, Side::Japanese, K).expect("fails again");
    assert_eq!(
        failed.kind,
        EventKind::GroundAttackFailed {
            side: Side::Japanese,
            hex: K,
            weight: w2,
            enemy_weight: w1,
        }
    );
    // Unit 1's weight is 1.8 x J/100 and unit 2's at most 0.15 x J/100.
    assert_eq!(w1, fx("1.8") * shots_of(j));
    assert!(w2 <= fx("0.15") * shots_of(j));
    // Fatigue 50 + 20 + 30 = 100: back to defend, and it holds.
    assert_eq!(run.fatigue(2, 2), vec![(LandFatigueChange::Gained, n(100))]);
    let back = run.back_to_defend(2);
    assert_eq!(back.len(), 1);
    assert_eq!(back[0].day, 2);
    assert_eq!(
        back[0].kind,
        EventKind::LandUnitBackToDefend {
            unit: unit(2),
            fatigue: n(100)
        }
    );
    assert!(run.back_to_defend(1).is_empty());
    assert!(run.spent_fell_back(2).is_empty());
    assert!(run.marched_days(2).is_empty());
    let u = run.unit_at(2, 2);
    assert_eq!(
        (u.posture, u.hex, u.fatigue),
        (Posture::Defend, Some(K), n(100))
    );
    assert!(u.march.is_none());
    assert_eq!(run.men(2, 2), j - 20 * f1.hits);
    assert_eq!(run.men(2, 1), a - 20 * f2.hits);
}

#[test]
fn k_days_3_to_6_there_is_no_fight_and_unit_2_rests_95_90_85_80() {
    let run = Run::live();
    for day in 3..=6u32 {
        assert!(run.fights(day, K).is_empty(), "day {day}");
        assert!(run.fires(day, K).is_empty());
        assert!(run.failed(day, Side::Japanese, K).is_none());
        assert_eq!(run.lost_in_fight(day, 1), 0);
        assert_eq!(run.lost_in_fight(day, 2), 0);
        let fatigue = n(100 - 5 * i64::from(day - 2));
        assert_eq!(run.unit_at(day, 2).fatigue, fatigue, "day {day}");
        assert_eq!(
            run.fatigue(day, 2),
            vec![(LandFatigueChange::Rested, fatigue)]
        );
        assert_eq!(run.unit_at(day, 2).posture, Posture::Defend);
        assert_eq!(run.unit_at(day, 2).hex, Some(K));
        // Fed by base 2 on each night: nobody is lost unfed.
        let men = run.men(day - 1, 2);
        assert_eq!(
            run.fed(day, Side::Japanese, K),
            Some((Some(2), men, men, 0))
        );
        assert_eq!(run.lost_unfed(day, 2), None);
        assert_eq!(run.men(day, 2), men);
        assert_eq!(run.men(day, 1), run.men(day - 1, 1));
    }
    // The Allied side never attacks at K: the Japanese side is not tested.
    assert!(run
        .all(|k| matches!(k, EventKind::SideBeatenAshore { hex, .. } if *hex == K))
        .is_empty());
    assert_eq!(run.at(6).bases[&base(2)].side, Side::Japanese);
    assert_eq!(run.at(6).bases[&base(1)].side, Side::Allied);
    assert!(run.unit_at(6, 2).men_ashore > 0);
}

// --------------------------------------------------------------------- At L

#[test]
fn l_day_1_the_attack_fails_and_unit_4_falls_back_though_base_4_holds_supply() {
    let run = Run::live();
    assert_eq!(run.fed(1, Side::Japanese, L), Some((Some(4), 500, 500, 0)));
    let f4 = run.fire(1, L, 4);
    assert_eq!((f4.firing, f4.shots, f4.chance), (300, n(3), fx("0.3")));
    let f3 = run.fire(1, L, 3);
    assert_eq!((f3.firing, f3.shots, f3.chance), (3000, n(15), fx("0.6")));
    let failed = run.failed(1, Side::Japanese, L).expect("fails");
    assert_eq!(
        failed.kind,
        EventKind::GroundAttackFailed {
            side: Side::Japanese,
            hex: L,
            weight: fx("0.9"),
            enemy_weight: n(9),
        }
    );
    assert_eq!(run.lost_in_fight(1, 3), 20 * f4.hits);
    assert_eq!(run.lost_in_fight(1, 4), 20 * f3.hits);
    let m = run.men(1, 4);
    assert_eq!(m, 500 - 20 * f3.hits);
    assert!(m >= 200);
    assert_eq!(run.men(1, 3), 3000 - 20 * f4.hits);
    // Fatigue 40 + 20 + 30 = 90, above 60: back to defend; then it falls
    // back, tired by 30 more: 100.
    assert_eq!(run.fatigue(1, 4), vec![(LandFatigueChange::Gained, n(90))]);
    let back = run.back_to_defend(4);
    assert_eq!(back.len(), 1);
    assert_eq!(back[0].day, 1);
    assert_eq!(
        back[0].kind,
        EventKind::LandUnitBackToDefend {
            unit: unit(4),
            fatigue: n(90)
        }
    );
    let fell = run.spent_fell_back(4);
    assert_eq!(fell.len(), 1);
    let fell = fell[0];
    assert_eq!(fell.day, 1);
    assert_eq!(
        fell.kind,
        EventKind::SpentUnitFellBack {
            unit: unit(4),
            side: Side::Japanese,
            from: L,
            to: M,
            men: m,
            fatigue: n(100),
        }
    );
    assert!(fell.causes.contains(&Cause::Event(back[0].id)));
    // The order of the phase: the fight, the fatigue, back to defend, what a
    // spent attacker does.
    let fatigue_event = run.on(
        1,
        |k| matches!(k, EventKind::LandUnitFatigue { unit, .. } if unit.raw() == 4),
    )[0];
    let last_fire = run
        .on(
            1,
            |k| matches!(k, EventKind::GroundFire { hex, .. } if *hex == L),
        )
        .into_iter()
        .map(|e| run.index_of(e))
        .max()
        .unwrap();
    assert!(last_fire < run.index_of(fatigue_event));
    assert!(run.index_of(fatigue_event) < run.index_of(back[0]));
    assert!(run.index_of(back[0]) < run.index_of(fell));
    // The fall back is a march for M, the one linked hex, set but not begun.
    let u = run.unit_at(1, 4);
    let march = u.march.as_ref().expect("a march");
    assert_eq!((march.to, march.days), (M, 0));
    assert_eq!(march.cause, Cause::Event(fell.id));
    assert_eq!(
        (u.posture, u.hex, u.fatigue),
        (Posture::Defend, Some(L), n(100))
    );
    assert_eq!(u.men_ashore, m);
}

#[test]
fn l_unit_4_marches_days_2_to_4_arrives_at_m_carrying_nothing_and_rests() {
    let run = Run::live();
    let m = run.men(1, 4);
    let fell = run.spent_fell_back(4)[0];
    // Fatigue after each day: 100, 100, 100, 100, 95, 90.
    for (day, fatigue) in [(1, 100), (2, 100), (3, 100), (4, 100), (5, 95), (6, 90)] {
        assert_eq!(run.unit_at(day, 4).fatigue, n(fatigue), "day {day}");
        // No man is lost after day 1.
        assert_eq!(run.men(day, 4), m, "day {day}");
        assert_eq!(run.lost_unfed(day, 4), None);
    }
    assert_eq!(run.fatigue(5, 4), vec![(LandFatigueChange::Rested, n(95))]);
    assert_eq!(run.fatigue(6, 4), vec![(LandFatigueChange::Rested, n(90))]);
    // Fed by base 4 on nights 2 to 4 (m x 0.005 t), by base 5 on 5 and 6.
    for day in 2..=4 {
        assert_eq!(run.fed(day, Side::Japanese, L), Some((Some(4), m, m, 0)));
    }
    for day in 5..=6 {
        assert_eq!(run.fed(day, Side::Japanese, M), Some((Some(5), m, m, 0)));
        assert!(run.fed(day, Side::Japanese, L).is_none());
    }
    // The march: days 1, 2 and 3 of 3, each caused by the falling back.
    assert_eq!(
        run.marched_days(4),
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
                |k| matches!(k, EventKind::MarchDay { unit, .. } if unit.raw() == 4)
            )
            .len(),
            1
        );
    }
    // At L until the phase of day 4, then at M.
    for day in 1..=3 {
        assert_eq!(run.unit_at(day, 4).hex, Some(L), "day {day}");
        assert!(run.unit_at(day, 4).march.is_some());
    }
    for day in 4..=6 {
        assert_eq!(run.unit_at(day, 4).hex, Some(M), "day {day}");
        assert!(run.unit_at(day, 4).march.is_none());
    }
    let (arrived, to, men, supply_t, formed) = run.arrived(4).expect("arrives");
    assert_eq!(arrived.day, 4);
    assert_eq!(arrived.pulse, None);
    assert_eq!((to, men, supply_t, formed), (M, m, n(0), None));
    assert_eq!(arrived.causes, vec![Cause::Event(run.last_march_day(4).id)]);
    assert!(run
        .all(|k| matches!(k, EventKind::MarchBaseFormed { .. }))
        .is_empty());
    // Nobody surrenders and no side is beaten.
    run.nobody_surrendered_or_beaten();
}

#[test]
fn l_no_fight_on_days_2_to_6_and_unit_3_loses_no_man_after_day_1() {
    let run = Run::live();
    for day in 2..=6 {
        assert!(run.fights(day, L).is_empty(), "day {day}");
        assert!(run.fights(day, M).is_empty(), "day {day}");
        assert_eq!(run.lost_in_fight(day, 3), 0);
        assert_eq!(run.men(day, 3), run.men(1, 3));
        assert_eq!(run.unit_at(day, 3).posture, Posture::Defend);
    }
}

#[test]
fn l_base_4_holds_7_5_less_3_m_x_0_005_and_base_5_holds_100_less_2_m_x_0_005() {
    let run = Run::live();
    let m = run.men(1, 4);
    let per_night = Fixed::from_int(i64::from(m)) * fx("0.005");
    assert_eq!(run.supply_at(0, 4), fx("10"));
    assert_eq!(run.supply_at(1, 4), fx("7.5"));
    assert_eq!(run.supply_at(2, 4), fx("7.5") - per_night);
    assert_eq!(run.supply_at(3, 4), fx("7.5") - per_night * 2);
    assert_eq!(run.supply_at(4, 4), fx("7.5") - per_night * 3);
    assert_eq!(run.supply_at(6, 4), fx("7.5") - per_night * 3);
    assert!(run.supply_at(6, 4) >= n(0));
    assert_eq!(run.at(6).bases[&base(4)].side, Side::Japanese);
    // Unit 4 carries nothing to M.
    assert_eq!(run.supply_at(4, 5), n(100));
    assert_eq!(run.supply_at(5, 5), n(100) - per_night);
    assert_eq!(run.supply_at(6, 5), n(100) - per_night * 2);
    assert_eq!(run.at(6).bases[&base(5)].side, Side::Japanese);
    assert_eq!(run.at(6).bases.len(), 9);
}

// --------------------------------------------------------------------- At N

#[test]
fn n_day_1_the_attack_fails_and_unit_6_falls_back_for_want_of_food() {
    let run = Run::live();
    // Base 7 pays 2.5 t, all it has: unit 6 is fed in full, and the base is
    // left with nothing.
    assert_eq!(run.fed(1, Side::Japanese, N), Some((Some(7), 500, 500, 0)));
    assert_eq!(run.supply_at(1, 7), n(0));
    let f6 = run.fire(1, N, 6);
    assert_eq!((f6.firing, f6.shots, f6.chance), (300, n(3), fx("0.3")));
    let f5 = run.fire(1, N, 5);
    assert_eq!((f5.firing, f5.shots, f5.chance), (3000, n(15), fx("0.6")));
    let failed = run.failed(1, Side::Japanese, N).expect("fails");
    assert_eq!(
        failed.kind,
        EventKind::GroundAttackFailed {
            side: Side::Japanese,
            hex: N,
            weight: fx("0.9"),
            enemy_weight: n(9),
        }
    );
    assert_eq!(run.lost_in_fight(1, 5), 20 * f6.hits);
    let nn = run.men(1, 6);
    assert_eq!(nn, 500 - 20 * f5.hits);
    assert!(nn >= 200);
    assert_eq!(run.men(1, 5), 3000 - 20 * f6.hits);
    assert_eq!(run.fatigue(1, 6), vec![(LandFatigueChange::Gained, n(90))]);
    let back = run.back_to_defend(6);
    assert_eq!(back.len(), 1);
    assert_eq!(
        back[0].kind,
        EventKind::LandUnitBackToDefend {
            unit: unit(6),
            fatigue: n(90)
        }
    );
    // No instruction, and base 7 holds nothing: it falls back to O.
    let fell = run.spent_fell_back(6);
    assert_eq!(fell.len(), 1);
    assert_eq!(
        fell[0].kind,
        EventKind::SpentUnitFellBack {
            unit: unit(6),
            side: Side::Japanese,
            from: N,
            to: O,
            men: nn,
            fatigue: n(100),
        }
    );
    assert!(fell[0].causes.contains(&Cause::Event(back[0].id)));
    let u = run.unit_at(1, 6);
    let march = u.march.as_ref().expect("a march");
    assert_eq!((march.to, march.days), (O, 0));
    assert_eq!(march.cause, Cause::Event(fell[0].id));
    assert_eq!(
        (u.posture, u.hex, u.fatigue),
        (Posture::Defend, Some(N), n(100))
    );
}

#[test]
fn n_unit_6_is_unfed_loses_a_twentieth_a_night_and_arrives_at_o_on_day_4_with_nothing() {
    let run = Run::live();
    let fell = run.spent_fell_back(6)[0];
    let mut men = run.men(1, 6);
    // The nights of days 2 to 4 at N, 5 and 6 at O.
    for day in 2..=6u32 {
        let hex = if day <= 4 { N } else { O };
        let loss = (share(men, 5, 100)).max(1);
        let after = men - loss;
        assert_eq!(
            run.fed(day, Side::Japanese, hex),
            Some((if day <= 4 { Some(7) } else { None }, men, 0, men)),
            "day {day}"
        );
        assert_eq!(run.lost_unfed(day, 6), Some((loss, after)), "day {day}");
        assert_eq!(run.men(day, 6), after, "day {day}");
        men = after;
    }
    // The men's trail, as the dice gave it.
    // Fatigue: 100 after days 1 to 4, 95 after day 5, 90 after day 6.
    for (day, fatigue) in [(1, 100), (2, 100), (3, 100), (4, 100), (5, 95), (6, 90)] {
        assert_eq!(run.unit_at(day, 6).fatigue, n(fatigue), "day {day}");
    }
    assert_eq!(run.fatigue(5, 6), vec![(LandFatigueChange::Rested, n(95))]);
    assert_eq!(run.fatigue(6, 6), vec![(LandFatigueChange::Rested, n(90))]);
    // The march.
    assert_eq!(
        run.marched_days(6),
        vec![
            (1, 3, Cause::Event(fell.id)),
            (2, 3, Cause::Event(fell.id)),
            (3, 3, Cause::Event(fell.id)),
        ]
    );
    for day in 1..=3 {
        assert_eq!(run.unit_at(day, 6).hex, Some(N), "day {day}");
    }
    for day in 4..=6 {
        assert_eq!(run.unit_at(day, 6).hex, Some(O), "day {day}");
        assert!(run.unit_at(day, 6).march.is_none());
    }
    let (arrived, to, arrived_men, supply_t, formed) = run.arrived(6).expect("arrives");
    assert_eq!(arrived.day, 4);
    assert_eq!(arrived.pulse, None);
    assert_eq!((to, supply_t, formed), (O, n(0), None));
    assert_eq!(arrived_men, run.men(4, 6));
    assert_eq!(arrived.causes, vec![Cause::Event(run.last_march_day(6).id)]);
    assert!(run
        .all(|k| matches!(k, EventKind::MarchBaseFormed { .. }))
        .is_empty());
    // Base 7 has nothing left to give.
    for day in 1..=6 {
        assert_eq!(run.supply_at(day, 7), n(0));
    }
    assert_eq!(run.at(6).bases.len(), 9);
    run.nobody_surrendered_or_beaten();
}

#[test]
fn n_no_fight_after_day_1_and_unit_5_loses_no_man_after_it() {
    let run = Run::live();
    for day in 2..=6 {
        assert!(run.fights(day, N).is_empty(), "day {day}");
        assert!(run.fights(day, O).is_empty(), "day {day}");
        assert_eq!(run.men(day, 5), run.men(1, 5));
    }
}

// --------------------------------------------------------------------- At P

#[test]
fn p_day_1_the_attack_does_not_fail_and_adds_nothing() {
    let run = Run::live();
    let f7 = run.fire(1, P, 7);
    assert_eq!((f7.firing, f7.shots, f7.chance), (4000, n(40), fx("0.3")));
    let f8 = run.fire(1, P, 8);
    assert_eq!((f8.firing, f8.shots, f8.chance), (1600, n(16), fx("0.6")));
    // 12 against 9.6: not below.
    assert_eq!(f7.shots * f7.chance, n(12));
    assert_eq!(f8.shots * f8.chance, fx("9.6"));
    assert!(run.failed(1, Side::Allied, P).is_none());
    assert!(run.failed(1, Side::Japanese, P).is_none());
    assert_eq!(run.lost_in_fight(1, 7), 20 * f8.hits);
    assert_eq!(run.lost_in_fight(1, 8), 20 * f7.hits);
    // Fatigue 20, nothing added.
    assert_eq!(run.fatigue(1, 7), vec![(LandFatigueChange::Gained, n(20))]);
    assert_eq!(run.unit_at(1, 7).fatigue, n(20));
    assert_eq!(run.unit_at(1, 7).posture, Posture::Attack);
    // The Japanese side is not beaten on day 1: unit 8 has at least 800 men
    // and 0.2 times unit 7's is at most 800.
    assert!(run.men(1, 8) >= 800);
    assert!(share(run.men(1, 7), 20, 100) <= 800);
    assert!(run
        .on(1, |k| matches!(k, EventKind::SideBeatenAshore { .. }))
        .is_empty());
}

#[test]
fn p_every_later_day_by_the_rule_from_the_run() {
    let run = Run::live();
    let mut fights = 0;
    for day in 1..=6u32 {
        let attacking = day == 1 || run.unit_at(day - 1, 7).posture == Posture::Attack;
        if !attacking {
            // No attack at P: no fight, no failed attack, the rest of the day.
            assert!(run.fights(day, P).is_empty(), "day {day}");
            assert!(run.failed(day, Side::Allied, P).is_none());
            assert_eq!(run.men(day, 7), run.men(day - 1, 7));
            assert_eq!(run.men(day, 8), run.men(day - 1, 8));
            let before = run.unit_at(day - 1, 7).fatigue;
            assert_eq!(run.unit_at(day, 7).fatigue, before - n(5), "day {day}");
            assert_eq!(
                run.fatigue(day, 7),
                vec![(LandFatigueChange::Rested, before - n(5))]
            );
            continue;
        }
        fights += 1;
        assert_eq!(run.fights(day, P).len(), 1, "day {day}");
        // The men, fed shares and fatigue the run has before the fight.
        let (m7, m8) = (run.men(day - 1, 7), run.men(day - 1, 8));
        let (fat7, fat8) = (
            run.unit_at(day - 1, 7).fatigue.trunc() as u32,
            run.unit_at(day - 1, 8).fatigue.trunc() as u32,
        );
        assert_eq!(fat8, 0);
        // Fed in full at P on every night.
        assert_eq!(run.fed(day, Side::Allied, P), Some((Some(8), m7, m7, 0)));
        assert_eq!(run.fed(day, Side::Japanese, P), Some((Some(9), m8, m8, 0)));
        let firing7 = share(m7, 100 - fat7, 100);
        let firing8 = share(m8, 100 - fat8, 100);
        // The limit: 3 times the enemy's fighting men.
        let (limit7, limit8) = (3 * m8, 3 * m7);
        let shots = |firing: u32, limit: u32| {
            if firing > limit {
                shots_of(limit)
            } else {
                shots_of(firing)
            }
        };
        let (s7, s8) = (shots(firing7, limit7), shots(firing8, limit8));
        let f7 = run.fire(day, P, 7);
        let f8 = run.fire(day, P, 8);
        assert_eq!(
            (f7.firing, f7.shots, f7.chance),
            (firing7, s7, fx("0.3")),
            "unit 7, day {day}"
        );
        assert_eq!(
            (f8.firing, f8.shots, f8.chance),
            (firing8, s8, fx("0.6")),
            "unit 8, day {day}"
        );
        let (w7, w8) = (s7 * fx("0.3"), s8 * fx("0.6"));
        // Is the limit acting? Where it is, the cut is as the figure says.
        if firing7 > limit7 {
            assert_eq!(f7.shots, shots_of(3 * m8));
        }
        if firing8 > limit8 {
            assert_eq!(f8.shots, shots_of(3 * m7));
        }
        // Losses are 20 a hit.
        assert_eq!(run.lost_in_fight(day, 7), 20 * f8.hits);
        assert_eq!(run.lost_in_fight(day, 8), 20 * f7.hits);
        assert_eq!(run.men(day, 7), m7 - 20 * f8.hits);
        assert_eq!(run.men(day, 8), m8 - 20 * f7.hits);
        // Failure, and what it costs.
        let failed = run.failed(day, Side::Allied, P);
        assert!(run.failed(day, Side::Japanese, P).is_none());
        let gain = if w7 < w8 {
            assert_eq!(
                failed.expect("the attack fails").kind,
                EventKind::GroundAttackFailed {
                    side: Side::Allied,
                    hex: P,
                    weight: w7,
                    enemy_weight: w8,
                },
                "day {day}"
            );
            50
        } else {
            assert!(failed.is_none(), "day {day}");
            20
        };
        let after = (fat7 + gain).min(100);
        assert_eq!(
            run.fatigue(day, 7),
            vec![(LandFatigueChange::Gained, n(i64::from(after)))],
            "day {day}"
        );
        assert_eq!(run.unit_at(day, 7).fatigue, n(i64::from(after)));
        // Back to defend above 60, and then it holds: base 8 holds supply.
        if after > 60 {
            assert_eq!(run.unit_at(day, 7).posture, Posture::Defend);
            let back = run.back_to_defend(7);
            assert_eq!(back.len(), 1);
            assert_eq!(back[0].day, day);
            assert!(run.supply_at(day, 8) > n(0));
            assert!(run.spent_fell_back(7).is_empty());
            assert_eq!(run.unit_at(day, 7).hex, Some(P));
            assert!(run.unit_at(day, 7).march.is_none());
        } else {
            assert_eq!(run.unit_at(day, 7).posture, Posture::Attack);
        }
        // Beaten: a side whose men are below 0.2 of the enemy's, after the
        // fight. P has no land link: its men surrender.
        let (a7, a8) = (run.men(day, 7), run.men(day, 8));
        let beaten = run.on(
            day,
            |k| matches!(k, EventKind::SideBeatenAshore { side: Side::Japanese, hex, .. } if *hex == P),
        );
        if 5 * a8 < a7 {
            assert_eq!(beaten.len(), 1, "day {day}");
            assert_eq!(
                run.on(
                    day,
                    |k| matches!(k, EventKind::MenSurrenderedAshore { hex, .. } if *hex == P)
                )
                .len(),
                1
            );
        } else if 5 * a8 > a7 {
            assert!(beaten.is_empty(), "day {day}");
        }
    }
    assert!(fights >= 1);
    // The fights stop when unit 7 goes back to defend: it did.
    assert!(!run.back_to_defend(7).is_empty());
    // Every later day it rests 5.
    let last = run.back_to_defend(7)[0].day;
    for day in last + 1..=6 {
        assert_eq!(
            run.unit_at(day, 7).fatigue,
            run.unit_at(day - 1, 7).fatigue - n(5)
        );
    }
}

// ------------------------------------------------------ Supplies, all hexes

#[test]
fn every_base_pays_the_men_it_feeds_each_night_and_ends_as_the_calculation_says() {
    let run = Run::live();
    let each = [
        (1, Side::Allied, K),
        (2, Side::Japanese, K),
        (3, Side::Allied, L),
        (4, Side::Japanese, L),
        (5, Side::Japanese, M),
        (6, Side::Allied, N),
        (7, Side::Japanese, N),
        (8, Side::Allied, P),
        (9, Side::Japanese, P),
    ];
    for (id, side, hex) in each {
        for day in 1..=6u32 {
            // The night charges the men at the hex as the day before left
            // them (units on a march are still where they began).
            let men = run.men_at(day - 1, side, hex);
            let wanted = Fixed::from_int(i64::from(men)) * fx("0.005");
            let before = run.supply_at(day - 1, id);
            let paid = std::cmp::min(wanted, before);
            assert_eq!(
                run.supply_at(day, id),
                before - paid,
                "base {id} on the night of day {day}"
            );
        }
    }
    // The ends the calculation names.
    assert_eq!(run.supply_at(6, 7), n(0));
    let spent: Vec<_> = run.all(|k| {
        matches!(
            k,
            EventKind::BaseSupplySpent {
                reason: SupplyUse::March,
                ..
            }
        )
    });
    assert!(spent.is_empty(), "nothing is carried by a march");
}

#[test]
fn nobody_surrenders_at_k_l_or_n_and_no_base_changes_side() {
    let run = Run::live();
    run.nobody_surrendered_or_beaten();
    for raw in 1..=8 {
        assert!(run.unit_at(6, raw).men_ashore > 0, "unit {raw}");
    }
}

// ----------------------------------------------------------------- Properties

/// The fighting men of the other side in the hex at the start of a fight, as
/// the end of the day and the day's losses give them (no unit enters or
/// leaves a fought hex in the phase).
fn enemy_fighting_men(run: &Run, day: u32, hex: Hex, side: Side) -> u32 {
    let enemy = match side {
        Side::Allied => Side::Japanese,
        Side::Japanese => Side::Allied,
    };
    let lost: u32 = run
        .at(day)
        .land_units
        .iter()
        .filter(|(_, u)| u.side == enemy && u.hex == Some(hex))
        .map(|(id, _)| run.lost_in_fight(day, id.raw()))
        .sum();
    run.men_at(day, enemy, hex) + lost
}

#[test]
fn property_shots_are_never_more_than_without_the_limit_and_the_smaller_sides_are_never_cut() {
    let run = Run::live();
    let mut cut = 0;
    let mut uncut = 0;
    for e in &run.events {
        if let EventKind::GroundFire {
            unit,
            hex,
            firing_men,
            shots,
            ..
        } = e.kind
        {
            let without = shots_of(firing_men);
            assert!(shots <= without, "never more than without the limit");
            let side = run.at(0).land_units[&unit].side;
            let limit = 3 * enemy_fighting_men(&run, e.day, hex, side);
            if firing_men <= limit {
                assert_eq!(shots, without, "not above the limit: not cut");
                uncut += 1;
            } else {
                assert_eq!(shots, shots_of(limit), "above the limit: cut to it");
                cut += 1;
            }
        }
    }
    // Both kinds of fire are in the run: the larger side cut (K, L, N and
    // day 2 of K), and the smaller side whole.
    assert!(cut >= 4, "{cut}");
    assert!(uncut >= 8, "{uncut}");
    // The smaller side at each fight of day 1 is whole.
    for (hex, small) in [(K, 2), (L, 4), (N, 6), (P, 8)] {
        let f = run.fire(1, hex, small);
        assert_eq!(f.shots, shots_of(f.firing));
    }
}

#[test]
fn property_on_the_frozen_figures_no_shot_is_cut_and_no_failed_day_is_charged() {
    // The scenario on the unchanged frozen figures: ratio 1000000 and 0.
    let run = Run::of(common::load(&path()).expect("loads on the frozen figures"));
    let figures = run.scenario.world.ref_data().ground.expect("ground");
    assert_eq!(figures.engagement_ratio, n(1_000_000));
    assert_eq!(figures.failed_attack_fatigue, n(0));
    let mut fires = 0;
    for e in &run.events {
        if let EventKind::GroundFire {
            firing_men, shots, ..
        } = e.kind
        {
            assert_eq!(shots, shots_of(firing_men), "no shot is cut");
            fires += 1;
        }
    }
    assert!(fires >= 8);
    // A day's attack costs exactly 20, and nothing for a failed day.
    let mut gains = 0;
    for e in &run.events {
        if let EventKind::LandUnitFatigue {
            unit,
            change: LandFatigueChange::Gained,
            fatigue,
        } = e.kind
        {
            let before = run.at(e.day - 1).land_units[&unit].fatigue;
            assert_eq!(fatigue, (before + n(20)).min(n(100)), "day {}", e.day);
            gains += 1;
        }
    }
    assert!(gains >= 4);
    // The first fights, on the frozen figures, are the same as on the live
    // figures that day: the figures act only after the fight is dealt.
    let live = Run::live();
    for hex in [K, L, N, P] {
        assert_eq!(run.fights(1, hex).len(), 1);
        assert_eq!(live.fights(1, hex).len(), 1);
    }
    // Where no unit is limited the fire is the same on both figures: P.
    let (a, b) = (run.fire(1, P, 7), live.fire(1, P, 7));
    assert_eq!((a.firing, a.shots, a.hits), (b.firing, b.shots, b.hits));
}

#[test]
fn property_an_instruction_to_hold_keeps_unit_6_at_n() {
    let held = with_instruction(&text(), 6, "hold");
    let run = Run::text(&held);
    let base_run = Run::live();
    // The day-1 fight is the same: the instruction is read after it.
    for raw in [5, 6] {
        let (a, b) = (run.fire(1, N, raw), base_run.fire(1, N, raw));
        assert_eq!((a.firing, a.shots, a.hits), (b.firing, b.shots, b.hits));
    }
    assert!(run.failed(1, Side::Japanese, N).is_some());
    assert_eq!(run.back_to_defend(6).len(), 1);
    assert!(run.spent_fell_back(6).is_empty());
    assert!(run.marched_days(6).is_empty());
    assert!(run.arrived(6).is_none());
    // Tired 90 after day 1, resting 5 a day thereafter, at N on the defence.
    for day in 1..=6u32 {
        let u = run.unit_at(day, 6);
        assert_eq!(u.hex, Some(N), "day {day}");
        assert_eq!(u.posture, Posture::Defend);
        assert!(u.march.is_none());
        assert_eq!(u.fatigue, n(90 - 5 * i64::from(day - 1)), "day {day}");
    }
    // It is unfed there each night and loses a twentieth of its men.
    let mut men = run.men(1, 6);
    for day in 2..=6u32 {
        let loss = share(men, 5, 100).max(1);
        assert_eq!(
            run.fed(day, Side::Japanese, N),
            Some((Some(7), men, 0, men))
        );
        assert_eq!(run.lost_unfed(day, 6), Some((loss, men - loss)));
        men -= loss;
    }
    run.nobody_surrendered_or_beaten();
}

#[test]
fn property_an_instruction_to_fall_back_where_there_is_no_hex_leaves_unit_2_at_k() {
    let told = with_instruction(&text(), 2, "fall_back");
    let run = Run::text(&told);
    let base_run = Run::live();
    // The same two fights, as the instruction is read after them.
    for day in 1..=2 {
        for raw in [1, 2] {
            let (a, b) = (run.fire(day, K, raw), base_run.fire(day, K, raw));
            assert_eq!((a.firing, a.shots, a.hits), (b.firing, b.shots, b.hits));
        }
    }
    // It goes back to defend on day 2 and has nowhere to go: it stays.
    let back = run.back_to_defend(2);
    assert_eq!(back.len(), 1);
    assert_eq!(back[0].day, 2);
    assert!(run.spent_fell_back(2).is_empty());
    assert!(run.marched_days(2).is_empty());
    for day in 2..=6u32 {
        let u = run.unit_at(day, 2);
        assert_eq!((u.hex, u.posture), (Some(K), Posture::Defend), "day {day}");
        assert!(u.march.is_none());
        assert!(u.men_ashore > 0);
    }
    assert_eq!(run.unit_at(6, 2).fatigue, n(80));
    // It does not surrender, and its side is not beaten.
    run.nobody_surrendered_or_beaten();
    assert_eq!(
        run.unit_at(6, 2).men_ashore,
        base_run.unit_at(6, 2).men_ashore
    );
}

/// The text of a side's view at the end of each day: its events, contacts,
/// claims, what it holds of the ground, and every question it can ask with
/// the answer.
fn views(source: &str, side: Side, days: usize) -> Vec<String> {
    let mut scenario = from(source);
    let mut out = Vec::new();
    while out.len() < days && scenario.next_day().expect("a day").is_some() {
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
        for b in view.enemy_bases() {
            text += &format!("{} {:?} {:?} {:?}\n", b.id.raw(), b.name, b.hex, b.side);
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
fn property_the_enemys_view_is_the_same_whichever_instruction_the_attack_orders_carry() {
    // World one: as written. World two: unit 4 told to hold (not fall back),
    // unit 6 told to hold, unit 2 told to fall back. The Allied side sees no
    // instruction and no tiredness. Until a unit has left the hex (unit 4
    // and unit 6 leave in the phase of day 2 and arrive in that of day 4;
    // their men are in the hex until then), the Allied view is the same.
    let one = text();
    let two = with_instruction(
        &with_instruction(&without_instruction(&one, 4), 4, "hold"),
        6,
        "hold",
    );
    let two = with_instruction(&two, 2, "fall_back");
    let (a, b) = (Run::text(&one), Run::text(&two));
    // The worlds differ: units 4 and 6 go in one and stay in the other.
    assert_ne!(a.scenario.world.day_hashes(), b.scenario.world.day_hashes());
    assert_eq!(a.unit_at(6, 4).hex, Some(M));
    assert_eq!(b.unit_at(6, 4).hex, Some(L));
    assert_eq!(a.unit_at(6, 6).hex, Some(O));
    assert_eq!(b.unit_at(6, 6).hex, Some(N));
    // The Allied view is the same on days 1, 2 and 3.
    let seen_one = views(&one, Side::Allied, 3);
    let seen_two = views(&two, Side::Allied, 3);
    assert_eq!(seen_one.len(), 3);
    assert_eq!(seen_one, seen_two);
    assert!(seen_one[2].contains("->"));
    // Nothing of the failed attacks' instruction or falling back is in it.
    for text in &seen_one {
        // (The Allied side's own unit 7 fails and goes back to defend on day
        // 3: it is told of that, and of no unit of the other side's.)
        assert!(!text.contains("SpentUnitFellBack"));
        assert!(!text.contains("LandUnitId(4)") && !text.contains("LandUnitId(6)"));
    }
    // The Japanese view differs: it sees its own units go back to defend or
    // fall back.
    assert_ne!(
        views(&one, Side::Japanese, 3),
        views(&two, Side::Japanese, 3)
    );
}

/// The days on which `side`'s attack failed, at any hex.
fn failed_days(run: &Run, side: Side) -> Vec<u32> {
    run.all(|k| matches!(k, EventKind::GroundAttackFailed { side: s, .. } if *s == side))
        .iter()
        .map(|e| e.day)
        .collect()
}

/// The men each unit of `side` lost in fights, by day.
fn fight_losses(run: &Run, side: Side) -> Vec<(u32, u32)> {
    run.all(|k| matches!(k, EventKind::MenLostInFight { .. }))
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::MenLostInFight { unit, lost, .. }
                if run.at(0).land_units[&unit].side == side =>
            {
                Some((e.day, lost))
            }
            _ => None,
        })
        .collect()
}

#[test]
fn property_the_attacker_sees_the_same_whatever_the_defenders_fatigue_and_units() {
    // The architect's amendment following decision 0105, 9: the attacker is
    // told on which days its own attack failed, which reads the defender's
    // weight of fire, and nothing else of it. Two worlds that differ only in
    // the defender at P (one regiment of 1600 men rested, or two of 800 at
    // fatigue 20), matched on the hits of each side (the men each side lost)
    // and on the days the Allied attack failed, give the Allied side the
    // same view. Each day of dice must match, so the worlds are compared over
    // the first two days, on the seeds where they do.
    let split = |source: &str| {
        let one_regiment = "men = 1600\nq = 30\nr = 0\nposture = \"defend\"\n";
        assert!(source.contains(one_regiment));
        source.replace(
            one_regiment,
            "men = 800\nq = 30\nr = 0\nposture = \"defend\"\nfatigue = 20\n\n[[land_unit]]\nid = 9\nname = \"Placeholder Japanese second regiment at P\"\nside = \"japanese\"\nmen = 800\nq = 30\nr = 0\nposture = \"defend\"\nfatigue = 20\n",
        )
    };
    let mut matched = 0;
    const DAYS: u32 = 2;
    for seed in 1..=400 {
        let one = text().replace("seed = 1942", &format!("seed = {seed}"));
        let two = split(&one);
        let (a, b) = (Run::text(&one), Run::text(&two));
        let japanese_men = |run: &Run| {
            (1..=DAYS)
                .map(|day| run.men_at(day, Side::Japanese, P))
                .collect::<Vec<_>>()
        };
        let early = |list: Vec<(u32, u32)>| -> Vec<(u32, u32)> {
            list.into_iter().filter(|(day, _)| *day <= DAYS).collect()
        };
        let early_failed =
            |list: Vec<u32>| -> Vec<u32> { list.into_iter().filter(|day| *day <= DAYS).collect() };
        if early(fight_losses(&a, Side::Allied)) != early(fight_losses(&b, Side::Allied))
            || japanese_men(&a) != japanese_men(&b)
            || early_failed(failed_days(&a, Side::Allied))
                != early_failed(failed_days(&b, Side::Allied))
        {
            continue;
        }
        matched += 1;
        assert_ne!(a.scenario.world.day_hashes(), b.scenario.world.day_hashes());
        assert_eq!(
            views(&one, Side::Allied, DAYS as usize),
            views(&two, Side::Allied, DAYS as usize),
            "seed {seed}"
        );
    }
    assert!(matched >= 3, "only {matched} seeds matched in 400");
}

#[test]
fn property_the_attacker_is_told_of_its_failure_by_the_hex_and_the_day_only() {
    // What the attacker is given of a failed attack: the hex and the day, no
    // weight, no men and no unit of the enemy; the enemy is given nothing.
    let source = text();
    let mut scenario = from(&source);
    scenario.run_to_end().expect("runs");
    let japanese = scenario.world.side_view(Side::Japanese);
    let failed: Vec<_> = japanese
        .events()
        .into_iter()
        .filter(|e| format!("{:?}", e.kind).starts_with("GroundAttackFailed"))
        .collect();
    // K on days 1 and 2, L and N on day 1.
    assert_eq!(failed.len(), 4, "{failed:?}");
    for e in &failed {
        let text = format!("{:?}", e.kind);
        assert!(text.starts_with("GroundAttackFailed { hex: Hex"), "{text}");
        assert!(!text.contains("weight") && !text.contains("LandUnitId"));
        assert!(e.breakdown.is_none());
    }
    let allied = scenario.world.side_view(Side::Allied);
    let allied_failed: Vec<_> = allied
        .events()
        .into_iter()
        .filter(|e| format!("{:?}", e.kind).starts_with("GroundAttackFailed"))
        .collect();
    // The Allied side is told only of its own failure, at P.
    assert!(allied_failed
        .iter()
        .all(|e| format!("{:?}", e.kind) == format!("GroundAttackFailed {{ hex: {P:?} }}")));
}
