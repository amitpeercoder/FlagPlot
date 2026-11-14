//! Scenario 29: what a land unit is good at. Mountain troops march faster in
//! the mountains and no faster out of them; troops raised to land from the
//! sea rehearse a landing faster; a heavy unit fires more and eats more; and
//! troops fitted to hold ground hit more often when they defend.
//!
//! Proves decision 0098 (what a land unit is good at) against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/29_skills.toml`. The frozen step 1 figures set every
//! skill's strength to 0, so this test gives the four strengths the values
//! below, as the test of ships' experience does. Experience is left at its
//! frozen strength of 0 and every unit has experience 2.
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
//! `assault_skill_strength` 0.25, `movement_skill_strength` 0.25,
//! `holding_skill_strength` 0.15, `weight_strength` 0.25. A skill's factor
//! is 1 + strength x (skill - 2). At 0.25 it is 0.5 for a skill of 0, 1 for
//! 2 and 1.5 for 4; at 0.15 it is 1.3 for a skill of 4.
//!
//! From the earlier rules: `march_days_per_hex` 3, `fatigue_per_march_day`
//! 10, `march_supply_days` 5, `rest_per_day` 5, `preparation_per_day` 5,
//! `daily_supply_t_per_man` 0.005, `men_per_shot` 100, `hit_chance` 0.3,
//! `defender_hit_chance` 0.6.
//!
//! ## Inputs
//!
//! A placeholder map. Coast hexes A at (0,0), L at (4,0), F at (6,0) and G
//! at (8,0), all clear. Land hexes M1 at (0,3) and M2 at (1,3), both
//! mountain, and P at (2,3), clear. Land links: M1 to M2, and M2 to P. The
//! game is 8 days.
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Allied   | A   | 5000 t |
//! | 2    | Allied   | M1  | 1000 t |
//! | 3    | Allied   | M2  | 1000 t |
//! | 4    | Allied   | F   | 5000 t |
//! | 5    | Japanese | F   | 5000 t |
//! | 6    | Allied   | G   | 5000 t |
//! | 7    | Japanese | G   | 5000 t |
//!
//! Every skill not named is 2.
//!
//! | Unit | Side     | Men  | Where                              | Skills            | Posture |
//! | ---- | -------- | ---- | ---------------------------------- | ----------------- | ------- |
//! | 1    | Allied   | 200  | M1                                 | Mountain 4        | defend  |
//! | 2    | Allied   | 200  | M1                                 | none              | defend  |
//! | 3    | Allied   | 200  | M1                                 | Mountain 0        | defend  |
//! | 4    | Allied   | 200  | M2                                 | Mountain 4        | defend  |
//! | 5    | Allied   | 600  | Aboard force 1, a troop transport at base 1 | Assault 4 | defend  |
//! | 6    | Allied   | 600  | Aboard force 2, the same           | none              | defend  |
//! | 7    | Allied   | 600  | Aboard force 3, the same           | Assault 0         | defend  |
//! | 8    | Allied   | 1000 | F                                  | Weight 4          | defend  |
//! | 9    | Japanese | 1000 | F                                  | Weight 0          | attack  |
//! | 10   | Allied   | 500  | G                                  | Holding 4         | defend  |
//! | 11   | Japanese | 500  | G                                  | none              | attack  |
//!
//! Orders, all on day 1: units 1, 2 and 3 march to M2; unit 4 marches to P;
//! forces 1, 2 and 3 each prepare a landing at L with the unit aboard as
//! roster. No force sails.
//!
//! ## Movement
//!
//! A day's march counts as the movement factor of a day, for the lower of
//! the unit's skills for the terrain of the hex it leaves and of the hex it
//! enters. A unit arrives when its days marched reach 3.
//!
//! | Unit | Hexes               | Skill that counts | A day counts as | Arrives, end of day | Fatigue on arrival |
//! | ---- | ------------------- | ----------------- | --------------- | ------------------- | ------------------ |
//! | 1    | Mountain to mountain | 4                | 1.5             | 2                   | 20                 |
//! | 2    | Mountain to mountain | 2                | 1               | 3                   | 30                 |
//! | 3    | Mountain to mountain | 0                | 0.5             | 6                   | 60                 |
//! | 4    | Mountain to clear   | 2, for the clear hex | 1            | 3                   | 30                 |
//!
//! Each gains 10 fatigue for each day it marches and rests 5 a day after.
//! At the end of day 8: unit 1 has 0, unit 2 has 5, unit 3 has 50 and unit 4
//! has 5.
//!
//! Each carries 5 x 200 x 0.005 = 5 t on arrival. Unit 4 finds no base at P
//! and forms base 8 there.
//!
//! Supply, night by night (each unit of 200 costs 1 t a night):
//!
//! - Base 2 at M1: nights 1 and 2, three units, 3 t each: 994 t. Day 2,
//!   unit 1 takes 5: 989. Night 3, two units: 987. Day 3, unit 2 takes 5:
//!   982. Nights 4, 5 and 6, one unit: 979. Day 6, unit 3 takes 5: 974. It
//!   holds 974 t at the end.
//! - Base 3 at M2: nights 1 and 2, unit 4: 998 t. Day 2, unit 1 brings 5:
//!   1003. Night 3, units 1 and 4: 1001. Day 3, unit 2 brings 5 and unit 4
//!   takes 5: 1001. Nights 4, 5 and 6, units 1 and 2: 995. Day 6, unit 3
//!   brings 5: 1000. Nights 7 and 8, three units: 994 t at the end.
//! - Base 8 at P: 5 t on day 3. Unit 4 costs 1 t on each of the nights of
//!   days 4 to 8: 0 t at the end, and it was fed every night.
//!
//! ## Rehearsal
//!
//! Each force lies at base 1 with its order for all 8 days. Each gains 5 of
//! loading a day: 40 at the end. A unit gains 5 of rehearsal a day times its
//! assault factor.
//!
//! | Unit | Assault skill | Rehearsal gained a day | After day 8 | Days to reach 100 |
//! | ---- | ------------- | ---------------------- | ----------- | ----------------- |
//! | 5    | 4             | 7.5                    | 60          | 14                |
//! | 6    | 2             | 5                      | 40          | 20                |
//! | 7    | 0             | 2.5                    | 20          | 40                |
//!
//! The men aboard do not eat. Base 1 holds 5000 t at the end.
//!
//! ## Weight, at F
//!
//! Weight multiplies a unit's shots and the supply each of its men costs.
//!
//! Night of day 1: unit 8 costs 1000 x 0.005 x 1.5 = 7.5 t, and base 4
//! holds 4992.5 t. Unit 9 costs 1000 x 0.005 x 0.5 = 2.5 t, and base 5
//! holds 4997.5 t. Both are fed in full.
//!
//! Day 1, the fight. The Japanese side attacks.
//!
//! - Unit 9, weight 0: 1000 firing men, 10 x 0.5 = 5 shots at 0.3.
//! - Unit 8, weight 4: 1000 firing men, 10 x 1.5 = 15 shots at 0.6.
//!
//! On each later night each unit costs its men times 0.005 times its weight
//! factor.
//!
//! ## Holding ground, at G
//!
//! Day 1. The Japanese side attacks and the Allied side defends.
//!
//! - Unit 11: 500 firing men, 5 shots at 0.3.
//! - Unit 10, holding 4: 500 firing men, 5 shots at 0.6 x 1.3 = 0.78.
//!
//! The holding skill acts only while the unit's side defends. The breakdown
//! of each unit's fire names the skill and its factor wherever one acts.
//!
//! # Properties
//!
//! - A unit with every skill at 2 gives what it gave before the rule, and
//!   with the frozen strengths of 0 every scenario with land units gives
//!   what it gave before, apart from the hash version.
//! - A higher movement skill never arrives later, and a higher assault skill
//!   never rehearses slower.
//! - The enemy's view is the same in two worlds that differ only in the
//!   other side's skills, where the hits are the same.

mod common;

use std::path::PathBuf;

use sim_core::events::{Event, EventKind, SupplyUse};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, LandUnitId, TaskForceId};
use sim_core::state::{Side, TruthState};
use sim_data::{LoadOptions, Scenario};

// ------------------------------------------------------------------ Helpers

const F: Hex = Hex::new(6, 0);
const G: Hex = Hex::new(8, 0);
const M1: Hex = Hex::new(0, 3);
const M2: Hex = Hex::new(1, 3);
const P: Hex = Hex::new(2, 3);

fn path() -> PathBuf {
    common::data_dir().join("scenarios/29_skills.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// The frozen figures of step 1 with the four strengths of the calculation.
fn working() -> LoadOptions {
    common::altered_reference("skills", |dir| {
        common::edit_file_by(
            dir,
            "ground.toml",
            &[
                (
                    "assault_skill_strength = 0",
                    "assault_skill_strength = \"0.25\"",
                ),
                (
                    "movement_skill_strength = 0",
                    "movement_skill_strength = \"0.25\"",
                ),
                (
                    "holding_skill_strength = 0",
                    "holding_skill_strength = \"0.15\"",
                ),
                ("weight_strength = 0", "weight_strength = \"0.25\""),
            ],
        );
    })
}

fn from(source: &str) -> Scenario {
    common::from_text_with(source, &path(), &working()).expect("the variant loads")
}

fn from_frozen(source: &str) -> Scenario {
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

/// The scenario text with every `skills = ...` line removed: every unit is
/// the standard unit.
fn without_skills(source: &str) -> String {
    source
        .lines()
        .filter(|line| !line.starts_with("skills ="))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

/// The scenario text with unit `raw`'s skills line replaced by `skills` (an
/// inline table's body, or empty for none).
fn with_skills(source: &str, raw: u32, skills: &str) -> String {
    let parts: Vec<&str> = source.split("\n[[land_unit]]\n").collect();
    let mut rebuilt = vec![parts[0].to_string()];
    for part in &parts[1..] {
        if !part.starts_with(&format!("id = {raw}\n")) {
            rebuilt.push((*part).to_string());
            continue;
        }
        // The unit's own text runs to the next table.
        let end = part.find("\n[[").unwrap_or(part.len());
        let (unit_text, rest) = part.split_at(end);
        let mut kept: Vec<String> = unit_text
            .lines()
            .filter(|l| !l.starts_with("skills ="))
            .map(str::to_string)
            .collect();
        if !skills.is_empty() {
            kept.push(format!("skills = {{ {skills} }}"));
        }
        rebuilt.push(kept.join("\n") + rest);
    }
    rebuilt.join("\n[[land_unit]]\n")
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

    fn frozen(source: &str) -> Run {
        Run::of(from_frozen(source))
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

    fn supply(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).bases[&base(raw)].supply_t
    }

    fn loading(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).task_forces[&TaskForceId::new(raw)]
            .preparation
            .as_ref()
            .expect("preparing")
            .loading
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

    fn on(&self, day: u32, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.day == day && pick(&e.kind))
            .collect()
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

    fn fire_event(&self, day: u32, raw: u32) -> (Fire, &Event) {
        let mut found: Vec<(Fire, &Event)> = self
            .fires(day)
            .into_iter()
            .filter(|(f, _)| f.unit == raw)
            .collect();
        assert_eq!(found.len(), 1, "unit {raw} fired once on day {day}");
        found.remove(0)
    }

    fn fire(&self, day: u32, raw: u32) -> Fire {
        self.fire_event(day, raw).0
    }

    /// The day a unit arrived from its march.
    fn arrival_day(&self, raw: u32) -> Option<u32> {
        let found =
            self.all(|k| matches!(k, EventKind::MarchArrived { unit, .. } if unit.raw() == raw));
        assert!(found.len() <= 1, "at most one arrival");
        found.first().map(|e| e.day)
    }

    /// `left_t` of each night's feeding of the men of `raw`'s base, in order.
    fn nights(&self, raw: u32) -> Vec<Fixed> {
        self.all(|k| {
            matches!(
                k,
                EventKind::BaseSupplySpent { base: b, reason: SupplyUse::Men, .. } if b.raw() == raw
            )
        })
        .into_iter()
        .map(|e| match e.kind {
            EventKind::BaseSupplySpent { left_t, .. } => left_t,
            _ => unreachable!(),
        })
        .collect()
    }
}

fn has_factor(event: &Event, name: &str) -> bool {
    event
        .breakdown
        .as_ref()
        .expect("a breakdown")
        .factors
        .iter()
        .any(|f| f.name == name)
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

/// A whole run's log as text, event numbers and all.
fn log_text(run: &Run) -> String {
    run.events
        .iter()
        .map(|e| format!("{e:?}\n"))
        .collect::<String>()
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_figures_and_the_inputs_are_the_calculations() {
    let scenario = from(&text());
    let g = scenario.world.ref_data().ground.expect("ground figures");
    assert_eq!(g.assault_skill_strength, fx("0.25"));
    assert_eq!(g.movement_skill_strength, fx("0.25"));
    assert_eq!(g.holding_skill_strength, fx("0.15"));
    assert_eq!(g.weight_strength, fx("0.25"));
    assert_eq!(g.march_days_per_hex, 3);
    assert_eq!(g.fatigue_per_march_day, n(10));
    assert_eq!(g.march_supply_days, n(5));
    assert_eq!(g.rest_per_day, n(5));
    assert_eq!(g.preparation_per_day, n(5));
    assert_eq!(g.daily_supply_t_per_man, fx("0.005"));
    assert_eq!(g.men_per_shot, n(100));
    assert_eq!(g.hit_chance, fx("0.3"));
    assert_eq!(g.defender_hit_chance, fx("0.6"));
    assert_eq!(g.experience_fire_strength, n(0));

    let truth = scenario.world.umpire();
    // The map: terrain and land links.
    let map = scenario.world.map();
    use sim_core::map::Terrain;
    for (hex, terrain) in [
        (Hex::new(0, 0), Terrain::Clear),
        (Hex::new(4, 0), Terrain::Clear),
        (F, Terrain::Clear),
        (G, Terrain::Clear),
        (M1, Terrain::Mountain),
        (M2, Terrain::Mountain),
        (P, Terrain::Clear),
    ] {
        assert_eq!(
            map.get(hex).and_then(|d| d.terrain),
            Some(terrain),
            "{hex:?}"
        );
    }
    assert!(map.has_land_link(M1, M2));
    assert!(map.has_land_link(M2, P));
    assert!(!map.has_land_link(M1, P));

    // Bases: id, side, hex, supply.
    assert_eq!(truth.bases.len(), 7);
    for (id, side, hex, supply) in [
        (1, Side::Allied, Hex::new(0, 0), 5000),
        (2, Side::Allied, M1, 1000),
        (3, Side::Allied, M2, 1000),
        (4, Side::Allied, F, 5000),
        (5, Side::Japanese, F, 5000),
        (6, Side::Allied, G, 5000),
        (7, Side::Japanese, G, 5000),
    ] {
        let b = &truth.bases[&base(id)];
        assert_eq!(
            (b.side, b.hex, b.supply_t),
            (side, hex, n(supply)),
            "base {id}"
        );
    }

    // Units: id, side, men, hex (None: aboard), the skills as (assault,
    // holding, weight, clear, jungle, mountain, atoll), the posture.
    use sim_core::orders::Posture::{Attack, Defend};
    assert_eq!(truth.land_units.len(), 11);
    for (id, side, men, hex, skills, posture) in [
        (
            1,
            Side::Allied,
            200,
            Some(M1),
            [2, 2, 2, 2, 2, 4, 2],
            Defend,
        ),
        (
            2,
            Side::Allied,
            200,
            Some(M1),
            [2, 2, 2, 2, 2, 2, 2],
            Defend,
        ),
        (
            3,
            Side::Allied,
            200,
            Some(M1),
            [2, 2, 2, 2, 2, 0, 2],
            Defend,
        ),
        (
            4,
            Side::Allied,
            200,
            Some(M2),
            [2, 2, 2, 2, 2, 4, 2],
            Defend,
        ),
        (5, Side::Allied, 600, None, [4, 2, 2, 2, 2, 2, 2], Defend),
        (6, Side::Allied, 600, None, [2, 2, 2, 2, 2, 2, 2], Defend),
        (7, Side::Allied, 600, None, [0, 2, 2, 2, 2, 2, 2], Defend),
        (
            8,
            Side::Allied,
            1000,
            Some(F),
            [2, 2, 4, 2, 2, 2, 2],
            Defend,
        ),
        (
            9,
            Side::Japanese,
            1000,
            Some(F),
            [2, 2, 0, 2, 2, 2, 2],
            Attack,
        ),
        (
            10,
            Side::Allied,
            500,
            Some(G),
            [2, 4, 2, 2, 2, 2, 2],
            Defend,
        ),
        (
            11,
            Side::Japanese,
            500,
            Some(G),
            [2, 2, 2, 2, 2, 2, 2],
            Attack,
        ),
    ] {
        let u = &truth.land_units[&unit(id)];
        let s = &u.skills;
        assert_eq!(
            (u.side, u.hex, u.posture, u.experience),
            (side, hex, posture, 2),
            "unit {id}"
        );
        assert_eq!(
            [s.assault, s.holding, s.weight, s.clear, s.jungle, s.mountain, s.atoll],
            skills,
            "unit {id}"
        );
        if hex.is_some() {
            assert_eq!(u.men_ashore, men, "unit {id}");
        }
    }
    // The men: ashore, or aboard a force's one troop transport.
    for id in 5..=7 {
        assert_eq!(
            truth.land_units[&unit(id)].men_ashore,
            0,
            "unit {id} is aboard"
        );
    }
    assert_eq!(truth.task_forces.len(), 3);
    assert_eq!(truth.ships.len(), 3);
    // Orders: all on day 1: four marches and three prepare-landings.
    let orders = scenario.scripted_orders();
    assert_eq!(orders.len(), 1);
    assert_eq!(orders[&1].len(), 7);
}

#[test]
fn movement_follows_the_lower_skill_of_the_two_hexes() {
    let run = Run::text(&text());
    assert_eq!(run.len(), 8);
    // Day by day, the days marched and what each day counted as: unit 1
    // (mountain 4, mountain to mountain) 1.5 a day, unit 2 (2) 1 a day, unit
    // 3 (0) 0.5 a day, unit 4 (mountain to clear, 2 for the clear hex) 1 a
    // day.
    for (raw, from, to, per_day, arrives) in [
        (1, M1, M2, fx("1.5"), 2),
        (2, M1, M2, n(1), 3),
        (3, M1, M2, fx("0.5"), 6),
        (4, M2, P, n(1), 3),
    ] {
        let days = run.all(|k| matches!(k, EventKind::MarchDay { unit, .. } if unit.raw() == raw));
        assert_eq!(
            days.len(),
            arrives,
            "unit {raw}: one MarchDay a day marched"
        );
        for (i, e) in days.iter().enumerate() {
            let marched = u32::try_from(i + 1).expect("few");
            assert_eq!(e.day, marched);
            assert_eq!(
                e.kind,
                EventKind::MarchDay {
                    unit: unit(raw),
                    from,
                    to,
                    days: marched,
                    counted: per_day * n(i64::from(marched)),
                    of: 3,
                },
                "unit {raw} day {marched}"
            );
        }
        // It arrives at the end of the day its counted days reach 3 and not
        // before.
        assert_eq!(run.arrival_day(raw), Some(arrives as u32), "unit {raw}");
        let last = days.last().expect("a march");
        assert!(matches!(last.kind, EventKind::MarchDay { counted, .. } if counted >= n(3)));
        if days.len() > 1 {
            let before = days[days.len() - 2];
            assert!(matches!(before.kind, EventKind::MarchDay { counted, .. } if counted < n(3)));
        }
        // Where it stands at the end of each day.
        for day in 1..=8u32 {
            let at = run.unit_at(day, raw).hex;
            let expected = if day >= arrives as u32 { to } else { from };
            assert_eq!(at, Some(expected), "unit {raw} end of day {day}");
        }
    }
    // The marching unit's order is the cause of its first day.
    // Fatigue: 10 a day marched, 5 a day rested after.
    for (raw, arrives, on_arrival) in [(1, 2, 20), (2, 3, 30), (3, 6, 60), (4, 3, 30)] {
        assert_eq!(
            run.unit_at(arrives, raw).fatigue,
            n(on_arrival),
            "unit {raw} on arrival"
        );
        for day in 1..=arrives.min(8) {
            assert_eq!(run.unit_at(day, raw).fatigue, n(10 * i64::from(day)));
        }
    }
    for (raw, at_end) in [(1, 0), (2, 5), (3, 50), (4, 5)] {
        assert_eq!(
            run.unit_at(8, raw).fatigue,
            n(at_end),
            "unit {raw} end of day 8"
        );
    }
    // Each carries 5 x 200 x 0.005 = 5 t on arrival.
    for raw in [1, 2, 3, 4] {
        let arrived =
            run.all(|k| matches!(k, EventKind::MarchArrived { unit, .. } if unit.raw() == raw));
        assert_eq!(arrived.len(), 1);
        assert!(
            matches!(arrived[0].kind, EventKind::MarchArrived { men: 200, supply_t, .. } if supply_t == n(5)),
            "unit {raw}: {:?}",
            arrived[0].kind
        );
        let spent = run.all(|k| {
            matches!(
                k,
                EventKind::BaseSupplySpent {
                    reason: SupplyUse::March,
                    men: 200,
                    ..
                }
            )
        });
        assert_eq!(spent.len(), 4);
    }
    // Unit 4 finds no base at P and forms base 8 there, on day 3.
    let formed = run.all(|k| matches!(k, EventKind::MarchBaseFormed { .. }));
    assert_eq!(formed.len(), 1);
    assert_eq!(formed[0].day, 3);
    assert!(matches!(
        formed[0].kind,
        EventKind::MarchBaseFormed { base: b, side: Side::Allied, hex, unit: u }
            if b == base(8) && hex == P && u == unit(4)
    ));
    assert_eq!(run.at(8).bases[&base(8)].hex, P);
    assert_eq!(run.at(8).bases.len(), 8);
}

#[test]
fn the_supply_of_the_marching_units_night_by_night() {
    let run = Run::text(&text());
    // Base 2 at M1, each night the units there are fed at 1 t each: nights
    // 1 and 2 three units, night 3 two, nights 4, 5 and 6 one.
    assert_eq!(
        run.nights(2),
        [997, 994, 987, 981, 980, 979].map(n).to_vec(),
        "base 2, night by night"
    );
    // Base 3 at M2: nights 1 and 2 unit 4; night 3 units 1 and 4; nights 4,
    // 5 and 6 units 1 and 2; nights 7 and 8 three units.
    assert_eq!(
        run.nights(3),
        [999, 998, 1001, 999, 997, 995, 997, 994].map(n).to_vec(),
        "base 3, night by night"
    );
    // Base 8 at P: 5 t on day 3, 1 t on each of the nights of days 4 to 8.
    assert_eq!(
        run.nights(8),
        [4, 3, 2, 1, 0].map(n).to_vec(),
        "base 8, night by night"
    );
    // Unit 4 was fed every night: at P the men were fed in full on days 4 to
    // 8 and none were lost unfed.
    for day in 4..=8 {
        let fed = run.on(
            day,
            |k| matches!(k, EventKind::MenFed { hex, .. } if *hex == P),
        );
        assert_eq!(fed.len(), 1, "day {day}");
        assert!(matches!(
            fed[0].kind,
            EventKind::MenFed { men: 200, fed: 200, unfed: 0, base: Some(b), .. } if b == base(8)
        ));
    }
    assert!(run
        .all(|k| matches!(k, EventKind::MenLostUnfed { .. }))
        .is_empty());
    // The ledger at the end of each day.
    let ledger: [(u32, [i64; 8]); 3] = [
        (2, [997, 989, 982, 981, 980, 974, 974, 974]),
        (3, [999, 1003, 1001, 999, 997, 1000, 997, 994]),
        (8, [0, 0, 5, 4, 3, 2, 1, 0]),
    ];
    for (b, by_day) in ledger {
        for (i, expected) in by_day.iter().enumerate() {
            let day = u32::try_from(i + 1).expect("few");
            if b == 8 && day < 3 {
                continue;
            }
            assert_eq!(
                run.supply(day, b),
                n(*expected),
                "base {b} end of day {day}"
            );
        }
    }
    assert_eq!(run.supply(8, 2), n(974));
    assert_eq!(run.supply(8, 3), n(994));
    assert_eq!(run.supply(8, 8), n(0));
}

#[test]
fn assault_skill_sets_the_rehearsal_gained_each_day() {
    let run = Run::text(&text());
    // The three forces lie at base 1 with their order the whole 8 days: each
    // gains 5 of loading a day, and each unit 5 x its assault factor of
    // rehearsal.
    for (raw, gained) in [(5, fx("7.5")), (6, n(5)), (7, fx("2.5"))] {
        for day in 1..=8u32 {
            assert_eq!(
                run.unit_at(day, raw).rehearsal,
                gained * n(i64::from(day)),
                "unit {raw} day {day}"
            );
        }
        let events =
            run.all(|k| matches!(k, EventKind::RehearsalGained { unit, .. } if unit.raw() == raw));
        assert_eq!(events.len(), 8);
        assert!(matches!(
            events[0].kind,
            EventKind::RehearsalGained { rehearsal, .. } if rehearsal == gained
        ));
    }
    for (raw, after) in [(5, 60), (6, 40), (7, 20)] {
        assert_eq!(run.unit_at(8, raw).rehearsal, n(after), "unit {raw}");
    }
    for force in 1..=3 {
        for day in 1..=8u32 {
            assert_eq!(run.loading(day, force), n(5 * i64::from(day)));
        }
        assert_eq!(run.loading(8, force), n(40));
    }
    // The men aboard do not eat: base 1 holds 5000 t throughout.
    for day in 1..=8 {
        assert_eq!(run.supply(day, 1), n(5000));
    }
    assert!(run.nights(1).is_empty());
}

#[test]
fn days_to_reach_100_are_14_20_and_40() {
    // The same scenario for 40 days.
    let long = text().replace("days = 8\n", "days = 40\n");
    assert!(long.contains("days = 40"));
    let run = Run::text(&long);
    assert_eq!(run.len(), 40);
    for (raw, first) in [(5, 14u32), (6, 20), (7, 40)] {
        let found = (1..=40u32)
            .find(|day| run.unit_at(*day, raw).rehearsal >= n(100))
            .expect("it reaches 100");
        assert_eq!(found, first, "unit {raw}");
        assert_eq!(run.unit_at(first, raw).rehearsal, n(100));
        assert!(run.unit_at(first - 1, raw).rehearsal < n(100));
    }
}

#[test]
fn weight_at_f_is_shots_and_supply() {
    let run = Run::text(&text());
    // Night of day 1: unit 8 costs 1000 x 0.005 x 1.5 = 7.5 t; unit 9 costs
    // 1000 x 0.005 x 0.5 = 2.5 t. Both fed in full.
    assert_eq!(run.nights(4)[0], fx("4992.5"));
    assert_eq!(run.nights(5)[0], fx("4997.5"));
    assert_eq!(run.supply(1, 4), fx("4992.5"));
    assert_eq!(run.supply(1, 5), fx("4997.5"));
    let fed = run.on(
        1,
        |k| matches!(k, EventKind::MenFed { hex, .. } if *hex == F),
    );
    assert_eq!(fed.len(), 2);
    for e in fed {
        assert!(matches!(
            e.kind,
            EventKind::MenFed {
                men: 1000,
                fed: 1000,
                unfed: 0,
                ..
            }
        ));
    }
    // Day 1, the fight. The Japanese side attacks.
    let fight = run.on(
        1,
        |k| matches!(k, EventKind::GroundFight { hex, .. } if *hex == F),
    );
    assert_eq!(fight.len(), 1);
    assert!(matches!(
        fight[0].kind,
        EventKind::GroundFight {
            allied_attacks: false,
            japanese_attacks: true,
            ..
        }
    ));
    let nine = run.fire(1, 9);
    assert_eq!(
        (nine.hex, nine.firing_men, nine.shots, nine.chance),
        (F, 1000, n(5), fx("0.3"))
    );
    let eight = run.fire(1, 8);
    assert_eq!(
        (eight.hex, eight.firing_men, eight.shots, eight.chance),
        (F, 1000, n(15), fx("0.6"))
    );
    assert!(eight.hits <= 15 && nine.hits <= 5);
    // The breakdown names weight and its factor where one acts (both), and
    // no holding skill (the unit that defends has a holding skill of 2).
    let (_, e8) = run.fire_event(1, 8);
    let (_, e9) = run.fire_event(1, 9);
    assert_eq!(factor(e8, "weight"), n(4));
    assert_eq!(factor(e8, "weight_factor"), fx("1.5"));
    assert_eq!(factor(e9, "weight"), n(0));
    assert_eq!(factor(e9, "weight_factor"), fx("0.5"));
    for e in [e8, e9] {
        assert!(!has_factor(e, "holding_skill") && !has_factor(e, "holding_factor"));
        assert!(!has_factor(e, "side_chance"));
    }
    // On each later night a unit costs its men times 0.005 times its weight
    // factor, and each fight's shots are its firing men over 100 times the
    // weight factor.
    let cost = |men: u32, weight: Fixed| n(i64::from(men)) * fx("0.005") * weight;
    let mut checked = 0;
    for day in 2..=run.len() {
        let before_8 = run.unit_at(day - 1, 8).men_ashore;
        let before_9 = run.unit_at(day - 1, 9).men_ashore;
        assert_eq!(
            run.supply(day - 1, 4) - run.supply_after_night(day, 4),
            cost(before_8, fx("1.5")),
            "unit 8, night of day {day}"
        );
        assert_eq!(
            run.supply(day - 1, 5) - run.supply_after_night(day, 5),
            cost(before_9, fx("0.5")),
            "unit 9, night of day {day}"
        );
        checked += 1;
    }
    assert_eq!(checked, 7);
    for day in 1..=run.len() {
        for (raw, weight, chance) in [(8, fx("1.5"), fx("0.6")), (9, fx("0.5"), fx("0.3"))] {
            let Some((fire, _)) = run.fires(day).into_iter().find(|(f, _)| f.unit == raw) else {
                continue;
            };
            assert_eq!(fire.chance, chance, "unit {raw} day {day}");
            assert_eq!(
                fire.shots,
                n(i64::from(fire.firing_men)) / n(100) * weight,
                "unit {raw} day {day}"
            );
        }
    }
    report(&run, "F", &[8, 9]);
}

#[test]
fn holding_ground_at_g_only_while_defending() {
    let run = Run::text(&text());
    let fight = run.on(
        1,
        |k| matches!(k, EventKind::GroundFight { hex, .. } if *hex == G),
    );
    assert_eq!(fight.len(), 1);
    assert!(matches!(
        fight[0].kind,
        EventKind::GroundFight {
            allied_attacks: false,
            japanese_attacks: true,
            ..
        }
    ));
    let eleven = run.fire(1, 11);
    assert_eq!(
        (eleven.hex, eleven.firing_men, eleven.shots, eleven.chance),
        (G, 500, n(5), fx("0.3"))
    );
    let ten = run.fire(1, 10);
    assert_eq!(
        (ten.hex, ten.firing_men, ten.shots, ten.chance),
        (G, 500, n(5), fx("0.78"))
    );
    assert!(ten.hits <= 5 && eleven.hits <= 5);
    // The breakdown names the skill and its factor for unit 10 and neither
    // for unit 11, whose skill is 2, and no weight for either.
    let (_, e10) = run.fire_event(1, 10);
    let (_, e11) = run.fire_event(1, 11);
    assert_eq!(factor(e10, "holding_skill"), n(4));
    assert_eq!(factor(e10, "holding_factor"), fx("1.3"));
    assert_eq!(factor(e10, "side_chance"), fx("0.6"));
    assert_eq!(factor(e10, "chance"), fx("0.78"));
    assert!(!has_factor(e11, "holding_skill") && !has_factor(e11, "holding_factor"));
    for e in [e10, e11] {
        assert!(!has_factor(e, "weight") && !has_factor(e, "weight_factor"));
    }
    // Every day it fires: the same chances, and shots at 1 a hundred men.
    for day in 1..=run.len() {
        for (raw, chance) in [(10, fx("0.78")), (11, fx("0.3"))] {
            let Some((fire, _)) = run.fires(day).into_iter().find(|(f, _)| f.unit == raw) else {
                continue;
            };
            assert_eq!(fire.chance, chance, "unit {raw} day {day}");
            assert_eq!(fire.shots, n(i64::from(fire.firing_men)) / n(100));
        }
    }
    // Supply at G on the first night: standard weight, 500 x 0.005 = 2.5 t.
    assert_eq!(run.supply(1, 6), fx("4997.5"));
    assert_eq!(run.supply(1, 7), fx("4997.5"));
    report(&run, "G", &[10, 11]);
}

fn report(run: &Run, place: &str, units: &[u32]) {
    for day in 1..=run.len() {
        for raw in units {
            if let Some((f, _)) = run.fires(day).into_iter().find(|(f, _)| f.unit == *raw) {
                eprintln!(
                    "DICE {place} day {day} unit {raw}: firing men {}, shots {}, chance {}, hits {}",
                    f.firing_men, f.shots, f.chance, f.hits
                );
            }
        }
    }
}

// --------------------------------------------------------------- Properties

#[test]
fn property_a_unit_with_every_skill_at_2_is_as_it_was() {
    let plain = without_skills(&text());
    // Every skill at 2: the working strengths do nothing, against the frozen
    // strengths of 0.
    let working = Run::text(&plain);
    let frozen = Run::frozen(&plain);
    assert_eq!(log_text(&working), log_text(&frozen));
    assert_eq!(
        working.scenario.world.state_hash(),
        frozen.scenario.world.state_hash()
    );
    // With the frozen strengths of 0 the scenario as written gives what it
    // gives with every skill line removed.
    let as_written = Run::frozen(&text());
    assert_eq!(log_text(&as_written), log_text(&frozen));
    // (The skills are state and are in the hash: the unit that has them
    // hashes differently from the standard unit, though the log is the same.)
    assert_ne!(
        as_written.scenario.world.state_hash(),
        frozen.scenario.world.state_hash()
    );
    // And the skills do change the run on the working strengths.
    let skilled = Run::text(&text());
    assert_ne!(log_text(&skilled), log_text(&working));
    assert_ne!(
        skilled.scenario.world.state_hash(),
        working.scenario.world.state_hash()
    );
}

#[test]
fn property_a_higher_movement_skill_never_arrives_later_and_a_higher_assault_skill_never_rehearses_slower(
) {
    // Unit 2's mountain skill from 0 to 4 on the mountain strip.
    let mut arrivals = Vec::new();
    for skill in 0..=4u32 {
        let run = Run::text(&with_skills(&text(), 2, &format!("mountain = {skill}")));
        arrivals.push(run.arrival_day(2));
    }
    // Factors 0.5, 0.75, 1, 1.25, 1.5: days of 3 / factor, up.
    assert_eq!(arrivals, [Some(6), Some(4), Some(3), Some(3), Some(2)]);
    for pair in arrivals.windows(2) {
        assert!(pair[1] <= pair[0], "never later: {arrivals:?}");
    }
    // Unit 4 leaves the mountains for the clear hex: the lower of the two
    // skills counts, so a clear skill of 0 slows it and a mountain skill of
    // 4 alone does not help.
    let mut by_clear = Vec::new();
    for skill in 0..=4u32 {
        let run = Run::text(&with_skills(
            &text(),
            4,
            &format!("mountain = 4, clear = {skill}"),
        ));
        by_clear.push(run.arrival_day(4));
    }
    assert_eq!(by_clear, [Some(6), Some(4), Some(3), Some(3), Some(2)]);

    // Unit 6's assault skill from 0 to 4.
    let mut rehearsed = Vec::new();
    for skill in 0..=4u32 {
        let run = Run::text(&with_skills(&text(), 6, &format!("assault = {skill}")));
        rehearsed.push(run.unit_at(8, 6).rehearsal);
    }
    assert_eq!(rehearsed, [n(20), fx("30"), n(40), fx("50"), n(60)]);
    for pair in rehearsed.windows(2) {
        assert!(pair[1] >= pair[0], "never slower: {rehearsed:?}");
    }
}

#[test]
fn property_the_enemys_view_is_the_same_when_only_the_other_sides_skills_differ_and_the_hits_are_the_same(
) {
    let base_run = Run::text(&text());
    let mut compared = 0;
    // Allied movement and assault skills change no fire, so every day's
    // Japanese view is the same.
    for (raw, skills) in [
        (1, "mountain = 0"),
        (3, "mountain = 4"),
        (4, "mountain = 0, clear = 4"),
        (5, "assault = 0"),
        (6, "assault = 4"),
        (7, "assault = 4"),
    ] {
        let other = Run::text(&with_skills(&text(), raw, skills));
        assert_ne!(
            log_text(&other),
            log_text(&base_run),
            "unit {raw} {skills}: the worlds differ in truth"
        );
        for day in 1..=8 {
            assert_eq!(
                day_digest(&base_run.scenario, Side::Japanese, day),
                day_digest(&other.scenario, Side::Japanese, day),
                "unit {raw} {skills}, day {day}"
            );
        }
        compared += 1;
    }
    // Allied weight and holding skills change the shots, so the hits may
    // differ: compare the worlds where unit 8's and unit 10's hits are the
    // same, on day 1, seen by the Japanese side.
    let mut same_hits = 0;
    for (raw, key, fired) in [(8, "weight", 8u32), (10, "holding", 10u32)] {
        let hits = base_run.fire(1, fired).hits;
        for skill in 0..=4u32 {
            let other = Run::text(&with_skills(&text(), raw, &format!("{key} = {skill}")));
            if other.fire(1, fired).hits != hits {
                continue;
            }
            same_hits += 1;
            assert_eq!(
                day_digest(&base_run.scenario, Side::Japanese, 1),
                day_digest(&other.scenario, Side::Japanese, 1),
                "unit {raw} {key} {skill}"
            );
        }
    }
    // The other way: the Japanese unit's weight, seen by the Allied side,
    // where unit 9's hits are the same.
    let hits = base_run.fire(1, 9).hits;
    for skill in 0..=4u32 {
        let other = Run::text(&with_skills(&text(), 9, &format!("weight = {skill}")));
        if other.fire(1, 9).hits != hits {
            continue;
        }
        same_hits += 1;
        assert_eq!(
            day_digest(&base_run.scenario, Side::Allied, 1),
            day_digest(&other.scenario, Side::Allied, 1),
            "unit 9 weight {skill}"
        );
    }
    assert_eq!(compared, 6);
    assert!(same_hits >= 3, "worlds with the same hits: {same_hits}");
}
