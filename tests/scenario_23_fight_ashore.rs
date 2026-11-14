//! Scenario 23: three places ashore. An attack on a fed defender that fails
//! and stops by fatigue; an unfed defender beaten on the first day, whose
//! base changes side; and an empty base that falls without a fight.
//!
//! Proves decision 0090 (the fight ashore, and a base that changes side)
//! against a hand calculation. The data is placeholder data: see
//! `data/scenarios/23_fight_ashore.toml`. The test runs on the frozen step 1
//! figures (`data/reference/step1`), to which the nine new ground figures
//! are added with their first values.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide, the
//! calculation gives the firing men, the shots and the chance exactly, and
//! what must follow from the hits the run rolls; the tests read the hits
//! from the run and assert the rest. Everything else is asserted exactly. If
//! the code disagrees, the code or the rule is wrong: do not change a number
//! here to make a test pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! `ground.toml`: `daily_supply_t_per_man` 0.005, `unfed_loss_share_per_day`
//! 0.05, `men_per_shot` 100, `hit_chance` 0.3, `defender_hit_chance` 0.6,
//! `men_per_hit` 20, `fatigue_per_attack_day` 20, `rest_per_day` 5,
//! `attack_fatigue_limit` 60, `beaten_share` 0.2, `captured_stock_share` 0.5.
//!
//! No air figures are loaded, so no aircraft flies and no base pays for
//! aircraft. The placeholder fighter has a strike range of 250 nm, so its
//! transfer range is 500 nm.
//!
//! ## Inputs
//!
//! A placeholder map with three coast hexes on one row: X at (0,0), Y at
//! (6,0) and Z at (10,0). The game is 6 days. Each night the men's supply is
//! charged before anything else in this scenario; the fight is after the
//! third pulse of the day.
//!
//! | Base | Side     | Hex | Supply | Fuel |
//! | ---- | -------- | --- | ------ | ---- |
//! | 1    | Allied   | X   | 1000 t | 0    |
//! | 2    | Japanese | X   | 100 t  | 0    |
//! | 3    | Japanese | Y   | 0 t    | 80 t |
//! | 4    | Allied   | Y   | 100 t  | 0    |
//! | 5    | Japanese | Z   | 60 t   | 30 t |
//!
//! | Land unit | Side     | Men  | Ashore at | Posture at the start |
//! | --------- | -------- | ---- | --------- | -------------------- |
//! | 1         | Allied   | 1000 | X         | defend               |
//! | 2         | Japanese | 600  | X         | attack               |
//! | 3         | Japanese | 300  | Y         | defend               |
//! | 4         | Allied   | 2000 | Y         | attack               |
//! | 5         | Allied   | 50   | Z         | attack               |
//!
//! Japanese air groups, all of the placeholder fighter, with no mission:
//! group 1 at base 3 with 9 ready and 2 damaged; group 2 at base 5 with 5
//! ready and 1 damaged; group 3 at base 5 with 8 ready.
//!
//! A Japanese placeholder destroyer, ship 1, lies in port at base 3, in no
//! task force. Japanese task force 1, of one placeholder destroyer, ship 2,
//! is stopped at sea at (3,1) with base 3 as its home port.
//!
//! Orders: on day 1 the Japanese side orders air group 3 to rebase to base 2.
//! Z to X is 10 hexes, 400 nm, within 500. There is no other order.
//!
//! Objectives. Allied: `base_held` base 3 for 10 points, `base_held` base 5
//! for 5, `ships_sunk` with 2 points for a destroyer, `men_lost` at 1 point
//! for each 100 men. Japanese: `base_held` base 1 for 10 points, `men_lost`
//! at 1 point for each 100 men.
//!
//! ## Z: an empty base
//!
//! Day 1 at intake: group 3 moves to base 2. It has no damaged aircraft, so
//! none is written off.
//!
//! Night of day 1: the Allied side has no base at Z. Unit 5's 50 men are
//! unfed: 50 x 0.05 = 2.5, cut to 2 lost. 48 are left.
//!
//! Day 1, ground combat: the Japanese side has no man at Z, so the hex is
//! not contested and there is no fight. Unit 5 is on attack and the Japanese
//! side has a base there and no man: it is beaten. Base 5 becomes Allied,
//! with 60 x 0.5 = 30 t of supply and 30 x 0.5 = 15 t of fuel. Air group 2 is
//! lost with 5 + 1 = 6 aircraft. Air group 3 is not there and is not
//! touched: at the end it is at base 2 with 8 ready. Unit 5 did not fire, so
//! it gains no fatigue; it stays on attack with fatigue 0 to the end.
//!
//! From the night of day 2 unit 5 is fed by base 5: 48 x 0.005 = 0.24 t a
//! night, all 48 fed.
//!
//! | Night of | Base 5 supply after |
//! | -------- | ------------------- |
//! | Day 2    | 29.76 t             |
//! | Day 3    | 29.52 t             |
//! | Day 4    | 29.28 t             |
//! | Day 5    | 29.04 t             |
//! | Day 6    | 28.8 t              |
//!
//! At the end: unit 5 has 48 men, base 5 is Allied with 28.8 t of supply and
//! 15 t of fuel.
//!
//! ## Y: an unfed defender
//!
//! Night of day 1. Base 3 has no supply: unit 3's 300 men are unfed, nothing
//! is paid, 300 x 0.05 = 15 are lost and 285 are left. Its fed share is 0.
//! Base 4 pays 2000 x 0.005 = 10 t for unit 4 and holds 90 t. Unit 4's fed
//! share is 1.
//!
//! Day 1, the fight. The Allied side attacks and the Japanese side defends.
//!
//! - Unit 4: firing men 2000 x 1 x 100/100 = 2000. 20 shots at 0.3.
//! - Unit 3: firing men 285 x 0 = 0. No shot.
//!
//! Let b1 be unit 4's hits, from 0 to 20. Unit 3 loses 20 x b1 men, or all
//! 285 if b1 is 15 or more. Unit 4 loses none. Unit 4's fatigue is 20; unit
//! 3 rests and stays at 0.
//!
//! Beaten: the Allied side attacked, and the Japanese side has at most 285
//! men against 0.2 x 2000 = 400. It is beaten whatever b1 is. Base 3 becomes
//! Allied: supply 0, fuel 80 x 0.5 = 40 t. Air group 1 is lost with 9 + 2 =
//! 11 aircraft. Ship 1 is sunk. Task force 1 has no home port.
//!
//! After day 1 the Japanese side has no base at Y. Each night what is left
//! of unit 3 is unfed, loses its men times 0.05, cut, and one man at least,
//! and its fed share is 0, so it never fires. The Allied side has two bases
//! at Y. Each night its 2000 men ask 10 t: base 3 has no supply and is asked
//! nothing, and base 4 pays 10 t. All are fed.
//!
//! While unit 3 has a man ashore the hex is contested and unit 4 fires:
//!
//! | Day | Unit 4's fatigue before | Firing men          | Shots at 0.3 | Fatigue after |
//! | --- | ----------------------- | ------------------- | ------------ | ------------- |
//! | 1   | 0                       | 2000                | 20           | 20            |
//! | 2   | 20                      | 2000 x 80/100 = 1600 | 16          | 40            |
//! | 3   | 40                      | 2000 x 60/100 = 1200 | 12          | 60            |
//! | 4   | 60                      | 2000 x 40/100 = 800  | 8           | 80, and it goes to defend |
//!
//! On a day when unit 3 has no man left there is no fight: unit 4 does not
//! fire, gains nothing and loses 5, and stays on attack. Unit 3 is destroyed
//! when its last man is lost, to a hit or unfed. The test reads each day's
//! hits and unit 3's men, and asserts each row above for the days there was
//! a fight, and the rest for the days there was none.
//!
//! Whatever the dice: unit 4 has 2000 men at the end; base 4's supply is 90,
//! 80, 70, 60, 50 and 40 t after the nights of days 1 to 6; base 3 is Allied
//! from day 1 with 0 t of supply and 40 t of fuel; the Allied side lost no
//! man at Y.
//!
//! ## X: an attack that fails
//!
//! Every night both sides are fed in full, so every fed share is 1. Base 1
//! pays 0.005 t for each Allied man there, and base 2 for each Japanese man.
//!
//! The Japanese side attacks and the Allied side defends. Let A and J be the
//! men of units 1 and 2 before a day's fight, and a and j the hits that
//! units 1 and 2 roll that day.
//!
//! | Day | Unit 2's fatigue before | Unit 2's firing men | Its chance | Unit 1's firing men | Its chance | Unit 2's fatigue after |
//! | --- | ----------------------- | ------------------- | ---------- | ------------------- | ---------- | ---------------------- |
//! | 1   | 0                       | J = 600             | 0.3        | A = 1000            | 0.6        | 20                     |
//! | 2   | 20                      | J x 80/100, cut     | 0.3        | A                   | 0.6        | 40                     |
//! | 3   | 40                      | J x 60/100, cut     | 0.3        | A                   | 0.6        | 60                     |
//! | 4   | 60                      | J x 40/100, cut     | 0.3        | A                   | 0.6        | 80, and it goes to defend |
//!
//! A unit's shots are its firing men over 100; a part of a shot is rolled at
//! that part of the chance. On day 1 unit 2 has 6 shots and unit 1 has 10.
//! After each fight unit 1 has A less 20 x j, and unit 2 has J less 20 x a,
//! or none. Both fire on the men they had before the fight.
//!
//! Unit 1 rests and stays at fatigue 0. On days 5 and 6 no unit at X is on
//! attack and there is no fight; unit 2's fatigue is 75 and then 70.
//!
//! Nobody is beaten at X. The Allied side never attacks, so the Japanese
//! side is not tested. Unit 2 has at most 6 shots a day, so unit 1 loses at
//! most 120 men a day and has at least 520 after four days, against 0.2
//! times at most 600. Base 1 is Allied and base 2 is Japanese at the end.
//!
//! If the dice destroy unit 2 before day 4, the rows after its last day do
//! not happen; the test then says so and the architect picks another seed.
//!
//! Supply at X after each night: base 1 has paid 0.005 t for each man unit 1
//! had that night, and base 2 for each man unit 2 had.
//!
//! ## The result
//!
//! - Allied `base_held`: base 3 is Allied, 10 points; base 5 is Allied, 5.
//! - Allied `ships_sunk`: ship 1, 2 points.
//! - Allied `men_lost`: the Japanese men lost over 100. They are the 300 of
//!   unit 3 less its men at the end, and the 600 of unit 2 less its men at
//!   the end.
//! - Japanese `base_held`: base 1 is not Japanese, 0 points.
//! - Japanese `men_lost`: the Allied men lost over 100: the 2 of unit 5 and
//!   the 1000 of unit 1 less its men at the end.
//!
//! Aircraft: the Japanese side lost 17, the 11 of group 1 and the 6 of group
//! 2.
//!
//! # Properties
//!
//! Beside these, the tests check what must always hold (decision 0090, "The
//! test"): a fight never creates men; more attackers never do worse over a
//! batch; men are conserved; the order of firing does not matter; a base's
//! side and the invariants agree at the end of every day; a scenario with no
//! posture order is unchanged from stage 1; and the enemy's view of a fight
//! does not show the other side's fatigue, fed share or number of units.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind, LandFatigueChange};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, EntityRef, LandUnitId, ShipId, TaskForceId};
use sim_core::objectives::{tallies, Level, Outcome};
use sim_core::orders::{Posture, RejectReason};
use sim_core::side_events::{SideEvent, SideEventKind};
use sim_core::state::{AirGroupLocation, LandUnit, ShipLocation, Side, TruthState};
use sim_data::Scenario;

// ------------------------------------------------------------------ Helpers

const X: Hex = Hex::new(0, 0);
const Y: Hex = Hex::new(6, 0);
const Z: Hex = Hex::new(10, 0);

fn path() -> PathBuf {
    common::data_dir().join("scenarios/23_fight_ashore.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// The live scenario file, on the frozen step 1 figures.
fn load() -> Scenario {
    common::load(&path()).expect("scenario 23 loads")
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

fn unit(id: u32) -> LandUnitId {
    LandUnitId::new(id)
}

fn base(id: u32) -> BaseId {
    BaseId::new(id)
}

/// A run of a scenario: the truth at the start and at the end of each day,
/// and the scenario itself at the end, with its log.
struct Run {
    scenario: Scenario,
    start: TruthState,
    ends: Vec<TruthState>,
}

impl Run {
    fn of(mut scenario: Scenario) -> Run {
        let start = scenario.world.umpire().clone();
        let mut ends = Vec::new();
        while scenario.next_day().expect("a day resolves").is_some() {
            ends.push(scenario.world.umpire().clone());
        }
        Run {
            scenario,
            start,
            ends,
        }
    }

    fn text(source: &str) -> Run {
        Run::of(from(source))
    }

    fn seed(seed: u32) -> Run {
        Run::text(&with_seed(&text(), seed))
    }

    fn events(&self) -> Vec<Event> {
        self.scenario.world.events().iter().cloned().collect()
    }

    /// The truth at the end of `day`; day 0 is the start.
    fn at(&self, day: u32) -> &TruthState {
        match day {
            0 => &self.start,
            _ => &self.ends[day as usize - 1],
        }
    }

    fn unit_at(&self, day: u32, id: u32) -> &LandUnit {
        &self.at(day).land_units[&unit(id)]
    }

    fn days(&self) -> u32 {
        u32::try_from(self.ends.len()).expect("a few days")
    }
}

fn with_seed(source: &str, seed: u32) -> String {
    common::replace_once(source, "seed = 1942\n", &format!("seed = {seed}\n"))
}

/// The order and its target, appended to a scenario's text.
fn with_posture_order(source: &str, day: u32, side: &str, land_unit: u32, posture: &str) -> String {
    format!(
        "{source}\n[[order]]\nday = {day}\nside = \"{side}\"\nland_unit = {land_unit}\nkind = \"set_posture\"\nposture = \"{posture}\"\n"
    )
}

/// The one event of `day` that satisfies `pick`; panics unless exactly one.
fn only_on(events: &[Event], day: u32, pick: impl Fn(&EventKind) -> bool) -> &Event {
    let found: Vec<&Event> = events
        .iter()
        .filter(|e| e.day == day && pick(&e.kind))
        .collect();
    assert_eq!(
        found.len(),
        1,
        "day {day}: exactly one such event: {found:?}"
    );
    found[0]
}

fn factors(event: &Event) -> Vec<(&'static str, Fixed)> {
    event
        .breakdown
        .as_ref()
        .expect("a breakdown")
        .factors
        .iter()
        .map(|f| (f.name, f.value))
        .collect()
}

/// One unit's fire in a fight, as the log has it.
#[derive(Clone, Debug)]
struct Fire {
    day: u32,
    unit: u32,
    hex: Hex,
    firing_men: u32,
    shots: Fixed,
    chance: Fixed,
    hits: u32,
    factors: Vec<(&'static str, Fixed)>,
}

fn fires(events: &[Event]) -> Vec<Fire> {
    events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::GroundFire {
                unit,
                hex,
                firing_men,
                shots,
                chance,
                hits,
            } => Some(Fire {
                day: e.day,
                unit: unit.raw(),
                hex: *hex,
                firing_men: *firing_men,
                shots: *shots,
                chance: *chance,
                hits: *hits,
                factors: factors(e),
            }),
            _ => None,
        })
        .collect()
}

fn fire_of(all: &[Fire], day: u32, unit: u32) -> Option<&Fire> {
    all.iter().find(|f| f.day == day && f.unit == unit)
}

/// What a unit lost in the fight of `day`: (lost, left ashore).
fn lost_in_fight(events: &[Event], day: u32, id: u32) -> Option<(u32, u32)> {
    events.iter().find_map(|e| match &e.kind {
        EventKind::MenLostInFight {
            unit, lost, ashore, ..
        } if e.day == day && unit.raw() == id => Some((*lost, *ashore)),
        _ => None,
    })
}

/// What a unit lost unfed on the night of `day`: (lost, left ashore).
fn lost_unfed(events: &[Event], day: u32, id: u32) -> Option<(u32, u32)> {
    events.iter().find_map(|e| match &e.kind {
        EventKind::MenLostUnfed {
            unit, lost, ashore, ..
        } if e.day == day && unit.raw() == id => Some((*lost, *ashore)),
        _ => None,
    })
}

fn fatigue_events(events: &[Event], day: u32, id: u32) -> Vec<(LandFatigueChange, Fixed)> {
    events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::LandUnitFatigue {
                unit,
                change,
                fatigue,
            } if e.day == day && unit.raw() == id => Some((*change, *fatigue)),
            _ => None,
        })
        .collect()
}

fn back_to_defend(events: &[Event], day: u32, id: u32) -> Vec<Fixed> {
    events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::LandUnitBackToDefend { unit, fatigue }
                if e.day == day && unit.raw() == id =>
            {
                Some(*fatigue)
            }
            _ => None,
        })
        .collect()
}

fn fight_events(events: &[Event], day: u32, hex: Hex) -> Vec<&Event> {
    events
        .iter()
        .filter(|e| {
            e.day == day && matches!(&e.kind, EventKind::GroundFight { hex: at, .. } if *at == hex)
        })
        .collect()
}

/// The men fed each night: (side, hex) of `day` to (base, men, fed, unfed).
fn fed_on(
    events: &[Event],
    day: u32,
    side: Side,
    hex: Hex,
) -> Option<(Option<BaseId>, u32, u32, u32)> {
    events.iter().find_map(|e| match &e.kind {
        EventKind::MenFed {
            side: s,
            hex: h,
            base,
            men,
            fed,
            unfed,
        } if e.day == day && *s == side && *h == hex => Some((*base, *men, *fed, *unfed)),
        _ => None,
    })
}

// ----------------------------------------------- The hand calculation: Z

#[test]
fn z_group_3_flies_to_base_2_and_loses_nothing_written_off() {
    let run = Run::of(load());
    let events = run.events();
    let rebased = only_on(
        &events,
        1,
        |k| matches!(k, EventKind::AirGroupRebased { air_group, .. } if air_group.raw() == 3),
    );
    assert!(matches!(
        &rebased.kind,
        EventKind::AirGroupRebased {
            from,
            to,
            distance_nm,
            transfer_range_nm,
            ready: 8,
            pilots: None,
            ..
        } if *from == base(5) && *to == base(2) && *distance_nm == n(400) && *transfer_range_nm == n(500)
    ));
    // It has no damaged aircraft, so none is written off.
    assert!(!events
        .iter()
        .any(|e| matches!(e.kind, EventKind::AircraftWrittenOff { .. })));
    // At the end it is at base 2 with 8 ready, and not touched when base 5
    // falls.
    for day in 1..=6 {
        let group = &run.at(day).air_groups[&AirGroupId::new(3)];
        assert_eq!(group.location, AirGroupLocation::Base(base(2)), "day {day}");
        assert_eq!((group.ready, group.damaged), (8, 0), "day {day}");
    }
    assert!(!events.iter().any(|e| matches!(&e.kind,
        EventKind::AirGroupLostWithBase { air_group, .. } if air_group.raw() == 3)));
}

#[test]
fn z_unit_5_is_unfed_on_night_1_and_base_5_falls_without_a_fight() {
    let run = Run::of(load());
    let events = run.events();
    // Night of day 1: 50 x 0.05 = 2.5, cut to 2; 48 left; the side has no
    // base at Z, so nothing is asked of one.
    assert_eq!(fed_on(&events, 1, Side::Allied, Z), Some((None, 50, 0, 50)));
    assert_eq!(lost_unfed(&events, 1, 5), Some((2, 48)));
    // The Japanese side has no man at Z: no fight, then or ever.
    for day in 1..=6 {
        assert!(fight_events(&events, day, Z).is_empty(), "day {day}");
    }
    assert!(!fires(&events).iter().any(|f| f.unit == 5));
    // Unit 5 is on attack and the Japanese side has a base and no man there:
    // beaten, without a fight.
    let beaten = only_on(
        &events,
        1,
        |k| matches!(k, EventKind::SideBeatenAshore { hex, .. } if *hex == Z),
    );
    assert_eq!(
        beaten.kind,
        EventKind::SideBeatenAshore {
            side: Side::Japanese,
            hex: Z,
            men: 0,
            enemy_men: 48,
            fought: false,
        }
    );
    // Base 5 becomes Allied with 60 x 0.5 = 30 t of supply and 30 x 0.5 =
    // 15 t of fuel.
    let changed = only_on(
        &events,
        1,
        |k| matches!(k, EventKind::BaseChangedSide { base: b, .. } if *b == base(5)),
    );
    assert_eq!(
        changed.kind,
        EventKind::BaseChangedSide {
            base: base(5),
            hex: Z,
            from: Side::Japanese,
            to: Side::Allied,
            supply_t: n(30),
            fuel_t: n(15),
            supply_lost_t: n(30),
            fuel_lost_t: n(15),
        }
    );
    assert!(changed.causes.contains(&Cause::Event(beaten.id)));
    // Air group 2 is lost with 5 + 1 = 6 aircraft.
    let lost = only_on(
        &events,
        1,
        |k| matches!(k, EventKind::AirGroupLostWithBase { air_group, .. } if air_group.raw() == 2),
    );
    assert_eq!(
        lost.kind,
        EventKind::AirGroupLostWithBase {
            air_group: AirGroupId::new(2),
            base: base(5),
            aircraft: 6,
        }
    );
    assert_eq!(lost.causes, vec![Cause::Event(changed.id)]);
    for day in 1..=6 {
        let group = &run.at(day).air_groups[&AirGroupId::new(2)];
        assert_eq!(group.location, AirGroupLocation::Lost, "day {day}");
        assert_eq!((group.ready, group.damaged), (0, 0), "day {day}");
    }
}

#[test]
fn z_unit_5_is_fed_by_base_5_from_the_night_of_day_2() {
    let run = Run::of(load());
    let events = run.events();
    let supply = ["30", "29.76", "29.52", "29.28", "29.04", "28.8"];
    for day in 1..=6u32 {
        let state = run.at(day);
        assert_eq!(
            state.bases[&base(5)].supply_t,
            fx(supply[day as usize - 1]),
            "day {day}"
        );
        assert_eq!(state.bases[&base(5)].fuel_t, n(15), "day {day}");
        assert_eq!(state.bases[&base(5)].side, Side::Allied, "day {day}");
        let unit = run.unit_at(day, 5);
        // Unit 5 did not fire: no fatigue, and it stays on attack.
        assert_eq!(unit.men_ashore, 48, "day {day}");
        assert_eq!(unit.fatigue, Fixed::ZERO, "day {day}");
        assert_eq!(unit.posture, Posture::Attack, "day {day}");
        if day >= 2 {
            assert_eq!(
                fed_on(&events, day, Side::Allied, Z),
                Some((Some(base(5)), 48, 48, 0)),
                "day {day}"
            );
            assert!(lost_unfed(&events, day, 5).is_none(), "day {day}");
        }
        assert!(fatigue_events(&events, day, 5).is_empty(), "day {day}");
    }
    let end = run.at(6);
    assert_eq!(end.land_units[&unit(5)].men_ashore, 48);
    assert_eq!(end.bases[&base(5)].supply_t, fx("28.8"));
}

// ----------------------------------------------- The hand calculation: Y

/// The hand calculation's Y, worked from the hits the run rolled, and
/// asserted against the log and the truth on every day.
#[test]
fn y_unit_4_beats_the_unfed_unit_3_day_by_day_as_the_table_says() {
    let run = Run::of(load());
    let events = run.events();
    let all = fires(&events);

    // Night of day 1: base 3 has no supply, so unit 3's 300 men are unfed and
    // nothing is paid; base 4 pays 10 t for unit 4.
    assert_eq!(
        fed_on(&events, 1, Side::Japanese, Y),
        Some((Some(base(3)), 300, 0, 300))
    );
    assert_eq!(
        fed_on(&events, 1, Side::Allied, Y),
        Some((Some(base(4)), 2000, 2000, 0))
    );

    let mut men3 = 300u32;
    let mut fatigue4: i64 = 0;
    let mut attacking = true;
    let mut destroyed_on = None;
    for day in 1..=6u32 {
        let mut ctx = format!("day {day}");
        if men3 > 0 {
            // Unfed: its men times 0.05, cut, one man at least.
            let lost = (men3 * 5 / 100).max(1);
            assert_eq!(
                lost_unfed(&events, day, 3),
                Some((lost, men3 - lost)),
                "{ctx}"
            );
            men3 -= lost;
            if men3 == 0 {
                destroyed_on = Some(day);
            }
        } else {
            assert!(lost_unfed(&events, day, 3).is_none(), "{ctx}");
        }
        let fought = men3 > 0 && attacking;
        assert_eq!(
            fight_events(&events, day, Y).len(),
            usize::from(fought),
            "{ctx}"
        );
        if fought {
            // Unit 4: firing men 2000 x (100 - fatigue) / 100, shots over
            // 100 at 0.3. Unit 3: fed share 0, so no firing man and no shot.
            let firing = u32::try_from(2000 * (100 - fatigue4) / 100).unwrap();
            let f4 = fire_of(&all, day, 4).expect("unit 4 fires");
            ctx = format!("{ctx} (unit 4 fires with {firing})");
            assert_eq!(f4.hex, Y);
            assert_eq!(f4.firing_men, firing, "{ctx}");
            assert_eq!(f4.shots, n(i64::from(firing)) / n(100), "{ctx}");
            assert_eq!(f4.chance, fx("0.3"), "{ctx}");
            assert_eq!(
                f4.factors,
                vec![
                    ("men_ashore", n(2000)),
                    ("fed_share", n(1)),
                    ("fatigue_factor", n(100 - fatigue4) / n(100)),
                    ("firing_men", n(i64::from(firing))),
                    ("men_per_shot", n(100)),
                    ("shots", n(i64::from(firing)) / n(100)),
                    ("chance", fx("0.3")),
                ],
                "{ctx}"
            );
            assert!(f4.hits <= 20);
            let f3 = fire_of(&all, day, 3).expect("unit 3 fires");
            assert_eq!(
                (f3.firing_men, f3.shots, f3.chance, f3.hits),
                (0, Fixed::ZERO, fx("0.6"), 0),
                "{ctx}"
            );
            assert_eq!(f3.factors[0], ("men_ashore", n(i64::from(men3))), "{ctx}");
            assert_eq!(f3.factors[1], ("fed_share", Fixed::ZERO), "{ctx}");
            // Unit 3 loses 20 for each hit, or all it has. Unit 4 loses none.
            let loss = (20 * f4.hits).min(men3);
            assert_eq!(
                lost_in_fight(&events, day, 3),
                (loss > 0).then_some((loss, men3 - loss)),
                "{ctx}"
            );
            assert!(lost_in_fight(&events, day, 4).is_none(), "{ctx}");
            men3 -= loss;
            if men3 == 0 && destroyed_on.is_none() {
                destroyed_on = Some(day);
            }
            // Fatigue: 20 gained a day; above 60 it goes to defend.
            fatigue4 += 20;
            assert_eq!(
                fatigue_events(&events, day, 4),
                vec![(LandFatigueChange::Gained, n(fatigue4))],
                "{ctx}"
            );
            if fatigue4 > 60 {
                attacking = false;
                assert_eq!(back_to_defend(&events, day, 4), vec![n(fatigue4)], "{ctx}");
            } else {
                assert!(back_to_defend(&events, day, 4).is_empty(), "{ctx}");
            }
            // Beaten: at most 285 men against 0.2 x 2000 = 400, whatever the
            // hits; and unit 3 rests, at 0.
            assert!(men3 < 400);
            let beaten = only_on(
                &events,
                day,
                |k| matches!(k, EventKind::SideBeatenAshore { hex, .. } if *hex == Y),
            );
            assert_eq!(
                beaten.kind,
                EventKind::SideBeatenAshore {
                    side: Side::Japanese,
                    hex: Y,
                    men: men3,
                    enemy_men: 2000,
                    fought: true,
                },
                "{ctx}"
            );
        } else {
            // No fight: unit 4 does not fire, gains nothing and loses 5.
            assert!(fire_of(&all, day, 4).is_none(), "{ctx}");
            let before = fatigue4;
            fatigue4 = (fatigue4 - 5).max(0);
            let expected = if before == fatigue4 {
                Vec::new()
            } else {
                vec![(LandFatigueChange::Rested, n(fatigue4))]
            };
            assert_eq!(fatigue_events(&events, day, 4), expected, "{ctx}");
        }
        // The end of the day.
        let u3 = run.unit_at(day, 3);
        let u4 = run.unit_at(day, 4);
        assert_eq!(u3.men_ashore, men3, "{ctx}");
        assert_eq!(u3.fatigue, Fixed::ZERO, "{ctx}");
        assert_eq!(u4.men_ashore, 2000, "{ctx}");
        assert_eq!(u4.fatigue, n(fatigue4), "{ctx}");
        assert_eq!(
            u4.posture,
            if attacking {
                Posture::Attack
            } else {
                Posture::Defend
            },
            "{ctx}"
        );
        // Base 4 paid 10 t a night: 90, 80, 70, 60, 50 and 40 t.
        assert_eq!(
            run.at(day).bases[&base(4)].supply_t,
            n(100 - 10 * i64::from(day)),
            "{ctx}"
        );
        // Base 3 is Allied from day 1 with 0 t of supply and 40 t of fuel.
        let b3 = &run.at(day).bases[&base(3)];
        assert_eq!(
            (b3.side, b3.supply_t, b3.fuel_t),
            (Side::Allied, Fixed::ZERO, n(40)),
            "{ctx}"
        );
    }
    // Unit 3 is destroyed when its last man is lost, to a hit or unfed.
    let destroyed: Vec<u32> = events
        .iter()
        .filter(|e| matches!(&e.kind, EventKind::LandUnitDestroyed { unit } if unit.raw() == 3))
        .map(|e| e.day)
        .collect();
    assert_eq!(destroyed, destroyed_on.into_iter().collect::<Vec<_>>());
    // The Allied side lost no man at Y.
    assert!(!events.iter().any(|e| matches!(&e.kind,
        EventKind::MenLostInFight { unit, .. } | EventKind::MenLostUnfed { unit, .. } if unit.raw() == 4)));
    // Day 1 is a fight: 2000 firing men, 20 shots at 0.3, and the base falls.
    let f4 = fire_of(&all, 1, 4).unwrap();
    assert_eq!((f4.firing_men, f4.shots), (2000, n(20)));
}

#[test]
fn y_base_3_changes_side_on_day_1_with_its_group_ship_and_force() {
    let run = Run::of(load());
    let events = run.events();
    let beaten = only_on(
        &events,
        1,
        |k| matches!(k, EventKind::SideBeatenAshore { hex, .. } if *hex == Y),
    );
    let changed = only_on(
        &events,
        1,
        |k| matches!(k, EventKind::BaseChangedSide { base: b, .. } if *b == base(3)),
    );
    // Supply 0 and fuel 80 x 0.5 = 40 t.
    assert_eq!(
        changed.kind,
        EventKind::BaseChangedSide {
            base: base(3),
            hex: Y,
            from: Side::Japanese,
            to: Side::Allied,
            supply_t: Fixed::ZERO,
            fuel_t: n(40),
            supply_lost_t: Fixed::ZERO,
            fuel_lost_t: n(40),
        }
    );
    assert!(changed.causes.contains(&Cause::Event(beaten.id)));
    // Air group 1 is lost with 9 + 2 = 11 aircraft.
    let group = only_on(
        &events,
        1,
        |k| matches!(k, EventKind::AirGroupLostWithBase { air_group, .. } if air_group.raw() == 1),
    );
    assert_eq!(
        group.kind,
        EventKind::AirGroupLostWithBase {
            air_group: AirGroupId::new(1),
            base: base(3),
            aircraft: 11,
        }
    );
    assert_eq!(group.causes, vec![Cause::Event(changed.id)]);
    // Ship 1 is sunk.
    let ship = only_on(&events, 1, |k| {
        matches!(k, EventKind::ShipLostWithBase { .. })
    });
    assert_eq!(
        ship.kind,
        EventKind::ShipLostWithBase {
            ship: ShipId::new(1),
            base: base(3),
        }
    );
    // Task force 1 has no home port.
    let port = only_on(&events, 1, |k| matches!(k, EventKind::HomePortLost { .. }));
    assert_eq!(
        port.kind,
        EventKind::HomePortLost {
            task_force: TaskForceId::new(1),
            base: base(3),
        }
    );
    assert_eq!(port.causes, vec![Cause::Event(changed.id)]);
    assert_eq!(
        run.start.task_forces[&TaskForceId::new(1)].home_port,
        Some(base(3))
    );
    for day in 1..=6 {
        let end = run.at(day);
        assert_eq!(
            end.ships[&ShipId::new(1)].location,
            ShipLocation::Sunk,
            "day {day}"
        );
        assert_eq!(
            end.task_forces[&TaskForceId::new(1)].home_port,
            None,
            "day {day}"
        );
        assert_eq!(
            end.air_groups[&AirGroupId::new(1)].location,
            AirGroupLocation::Lost,
            "day {day}"
        );
        // Ship 2, at sea, is not touched.
        assert!(!matches!(
            end.ships[&ShipId::new(2)].location,
            ShipLocation::Sunk
        ));
    }
    // The base changed side once, and it is on both maps.
    assert_eq!(
        events
            .iter()
            .filter(
                |e| matches!(&e.kind, EventKind::BaseChangedSide { base: b, .. } if *b == base(3))
            )
            .count(),
        1
    );
    assert!(run.at(1).bases[&base(3)].on_map);
}

// ----------------------------------------------- The hand calculation: X

/// The hand calculation's X, worked from the hits the run rolled, and
/// asserted against the log and the truth on every day.
#[test]
fn x_unit_2_attacks_and_fails_day_by_day_as_the_table_says() {
    let run = Run::of(load());
    let events = run.events();
    let all = fires(&events);

    let (mut a, mut j) = (1000u32, 600u32);
    let mut fatigue2: i64 = 0;
    let mut attacking = true;
    let (mut supply1, mut supply2) = (n(1000), n(100));
    let per_man = fx("0.005");
    for day in 1..=6u32 {
        let ctx = format!("day {day}, unit 1 has {a}, unit 2 has {j}");
        // Both sides are fed in full every night; each base pays 0.005 t a
        // man.
        assert_eq!(
            fed_on(&events, day, Side::Allied, X),
            Some((Some(base(1)), a, a, 0)),
            "{ctx}"
        );
        assert_eq!(
            fed_on(&events, day, Side::Japanese, X),
            Some((Some(base(2)), j, j, 0)),
            "{ctx}"
        );
        supply1 -= per_man * n(i64::from(a));
        supply2 -= per_man * n(i64::from(j));
        if attacking {
            assert_eq!(fight_events(&events, day, X).len(), 1, "{ctx}");
            let fight = fight_events(&events, day, X)[0];
            assert_eq!(
                fight.kind,
                EventKind::GroundFight {
                    hex: X,
                    allied_attacks: false,
                    japanese_attacks: true,
                },
                "{ctx}"
            );
            // Unit 2: J x (100 - fatigue) / 100 firing men, cut, at 0.3.
            let firing2 = u32::try_from(i64::from(j) * (100 - fatigue2) / 100).unwrap();
            let f2 = fire_of(&all, day, 2).expect("unit 2 fires");
            assert_eq!(
                (f2.hex, f2.firing_men, f2.chance),
                (X, firing2, fx("0.3")),
                "{ctx}"
            );
            assert_eq!(f2.shots, n(i64::from(firing2)) / n(100), "{ctx}");
            assert_eq!(
                f2.factors,
                vec![
                    ("men_ashore", n(i64::from(j))),
                    ("fed_share", n(1)),
                    ("fatigue_factor", n(100 - fatigue2) / n(100)),
                    ("firing_men", n(i64::from(firing2))),
                    ("men_per_shot", n(100)),
                    ("shots", n(i64::from(firing2)) / n(100)),
                    ("chance", fx("0.3")),
                ],
                "{ctx}"
            );
            // Unit 1: A firing men at 0.6.
            let f1 = fire_of(&all, day, 1).expect("unit 1 fires");
            assert_eq!(
                (f1.hex, f1.firing_men, f1.chance),
                (X, a, fx("0.6")),
                "{ctx}"
            );
            assert_eq!(f1.shots, n(i64::from(a)) / n(100), "{ctx}");
            assert_eq!(
                f1.factors,
                vec![
                    ("men_ashore", n(i64::from(a))),
                    ("fed_share", n(1)),
                    ("fatigue_factor", n(1)),
                    ("firing_men", n(i64::from(a))),
                    ("men_per_shot", n(100)),
                    ("shots", n(i64::from(a)) / n(100)),
                    ("chance", fx("0.6")),
                ],
                "{ctx}"
            );
            if day == 1 {
                // On day 1 unit 2 has 6 shots and unit 1 has 10.
                assert_eq!((f2.shots, f1.shots), (n(6), n(10)));
            }
            // After the fight unit 1 has A less 20 x j and unit 2 has J less
            // 20 x a, both having fired on the men they had before.
            let (a_before, j_before) = (a, j);
            a -= (20 * f2.hits).min(a_before);
            j -= (20 * f1.hits).min(j_before);
            assert_eq!(
                lost_in_fight(&events, day, 1),
                (a_before > a).then_some((a_before - a, a)),
                "{ctx}"
            );
            assert_eq!(
                lost_in_fight(&events, day, 2),
                (j_before > j).then_some((j_before - j, j)),
                "{ctx}"
            );
            if day <= 3 {
                assert!(
                    j > 0,
                    "unit 2 was destroyed on day {day}, before day 4, by the dice of seed 1942: \
                     the rows of the hand calculation after its last day do not happen. The \
                     architect picks another seed"
                );
            }
            fatigue2 += 20;
            assert_eq!(
                fatigue_events(&events, day, 2),
                vec![(LandFatigueChange::Gained, n(fatigue2))],
                "{ctx}"
            );
            if fatigue2 > 60 {
                attacking = false;
                assert_eq!(day, 4, "{ctx}");
                assert_eq!(back_to_defend(&events, day, 2), vec![n(80)], "{ctx}");
            } else {
                assert!(back_to_defend(&events, day, 2).is_empty(), "{ctx}");
            }
            // Unit 1 rests and stays at fatigue 0: nothing changes, nothing
            // is recorded.
            assert!(fatigue_events(&events, day, 1).is_empty(), "{ctx}");
        } else {
            // On days 5 and 6 no unit at X is on attack: no fight; unit 2's
            // fatigue is 75 and then 70.
            assert!(fight_events(&events, day, X).is_empty(), "{ctx}");
            assert!(fire_of(&all, day, 2).is_none(), "{ctx}");
            fatigue2 -= 5;
            assert_eq!(
                fatigue_events(&events, day, 2),
                vec![(LandFatigueChange::Rested, n(fatigue2))],
                "{ctx}"
            );
        }
        // The end of the day.
        let end = run.at(day);
        assert_eq!(end.land_units[&unit(1)].men_ashore, a, "{ctx}");
        assert_eq!(end.land_units[&unit(2)].men_ashore, j, "{ctx}");
        assert_eq!(end.land_units[&unit(1)].fatigue, Fixed::ZERO, "{ctx}");
        assert_eq!(end.land_units[&unit(2)].fatigue, n(fatigue2), "{ctx}");
        assert_eq!(end.land_units[&unit(1)].posture, Posture::Defend, "{ctx}");
        assert_eq!(
            end.land_units[&unit(2)].posture,
            if attacking {
                Posture::Attack
            } else {
                Posture::Defend
            },
            "{ctx}"
        );
        assert_eq!(end.bases[&base(1)].supply_t, supply1, "{ctx}");
        assert_eq!(end.bases[&base(2)].supply_t, supply2, "{ctx}");
    }
    // Unit 2 has at most 6 shots a day, so unit 1 loses at most 120 men a day
    // and has at least 520 after four days.
    assert!(run.unit_at(4, 1).men_ashore >= 520);
    assert_eq!(fatigue2, 70);
    assert_eq!(run.unit_at(5, 2).fatigue, n(75));
    // Unit 2 goes to defend on day 4 at fatigue 80, and its order is spent.
    assert_eq!(run.unit_at(4, 2).fatigue, n(80));
    assert_eq!(run.unit_at(4, 2).posture_order, None);
    // Nobody is beaten at X. Base 1 is Allied and base 2 is Japanese.
    assert!(!events.iter().any(|e| matches!(&e.kind,
        EventKind::SideBeatenAshore { hex, .. } if *hex == X)));
    let end = run.at(6);
    assert_eq!(end.bases[&base(1)].side, Side::Allied);
    assert_eq!(end.bases[&base(2)].side, Side::Japanese);
    assert_eq!(
        end.air_groups[&AirGroupId::new(3)].location,
        AirGroupLocation::Base(base(2))
    );
}

// ------------------------------------------------------- The hand calculation: result

#[test]
fn the_result_is_what_the_hand_calculation_says() {
    let run = Run::of(load());
    let end = run.at(6);
    let men3 = end.land_units[&unit(3)].men_ashore;
    let men2 = end.land_units[&unit(2)].men_ashore;
    let men1 = end.land_units[&unit(1)].men_ashore;
    let japanese_lost = (300 - men3) + (600 - men2);
    let allied_lost = 2 + (1000 - men1);
    let result = run
        .scenario
        .world
        .result()
        .expect("the scenario has objectives");

    // Allied: base_held base 3, 10 points; base 5, 5; ships_sunk, ship 1, 2
    // points; men_lost, the Japanese men lost over 100.
    let allied = &result.allied.objectives;
    assert_eq!(allied.len(), 4);
    assert_eq!(
        (allied[0].kind, allied[0].base, allied[0].points),
        ("base_held", Some(base(3)), n(10))
    );
    assert_eq!(
        (allied[1].kind, allied[1].base, allied[1].points),
        ("base_held", Some(base(5)), n(5))
    );
    assert_eq!((allied[2].kind, allied[2].points), ("ships_sunk", n(2)));
    assert_eq!(allied[2].count, n(1));
    assert_eq!(allied[3].kind, "men_lost");
    assert_eq!(allied[3].count, n(i64::from(japanese_lost)));
    assert_eq!(allied[3].points, n(i64::from(japanese_lost)) * fx("0.01"));

    // Japanese: base_held base 1 is not Japanese, 0 points; men_lost, the
    // Allied men lost over 100: the 2 of unit 5 and the 1000 of unit 1 less
    // its men at the end.
    let japanese = &result.japanese.objectives;
    assert_eq!(japanese.len(), 2);
    assert_eq!(
        (japanese[0].kind, japanese[0].base, japanese[0].points),
        ("base_held", Some(base(1)), Fixed::ZERO)
    );
    assert_eq!(japanese[1].kind, "men_lost");
    assert_eq!(japanese[1].count, n(i64::from(allied_lost)));
    assert_eq!(japanese[1].points, n(i64::from(allied_lost)) * fx("0.01"));

    let allied_total = n(10 + 5 + 2) + allied[3].points;
    let japanese_total = japanese[1].points;
    assert_eq!(result.allied.total, allied_total);
    assert_eq!(result.japanese.total, japanese_total);
    assert_eq!(result.allied.margin, allied_total - japanese_total);
    assert_eq!(result.japanese.margin, japanese_total - allied_total);
    assert!(result.allied.margin >= n(20), "decisive is at 20");
    assert_eq!(result.allied.outcome, Outcome::Victory(Level::Decisive));
    assert_eq!(result.japanese.outcome, Outcome::Defeat(Level::Decisive));

    // Aircraft: the Japanese side lost 17, the 11 of group 1 and the 6 of
    // group 2, and they count for the Allied tally.
    let tally = tallies(
        run.scenario.world.umpire(),
        run.scenario.world.events(),
        run.scenario.world.ref_data(),
    );
    assert_eq!(tally[&Side::Allied].aircraft, 17);
    assert_eq!(tally[&Side::Japanese].aircraft, 0);
    assert_eq!(tally[&Side::Allied].men_lost, japanese_lost);
    assert_eq!(tally[&Side::Japanese].men_lost, allied_lost);
}

// ------------------------------------------------------------ Properties

/// Scenario 23 at each of `count` seeds.
fn seed_runs(count: u32) -> Vec<(u32, Run)> {
    (1..=count).map(|seed| (seed, Run::seed(seed))).collect()
}

/// The men of a unit that began as it did, counted at the end of `day`: those
/// ashore and aboard, and every man lost by any cause up to the end of the
/// day.
fn men_accounted(run: &Run, events: &[Event], day: u32, id: u32) -> u32 {
    let state = run.at(day);
    let mut total = state.land_units[&unit(id)].men_ashore + state.men_aboard(unit(id));
    for event in events.iter().filter(|e| e.day <= day) {
        match &event.kind {
            EventKind::MenLostUnfed { unit, lost, .. }
            | EventKind::MenLostInFight { unit, lost, .. }
                if unit.raw() == id =>
            {
                total += lost;
            }
            EventKind::MenLostWithShip { unit, men, .. } if unit.raw() == id => total += men,
            _ => {}
        }
    }
    total
}

#[test]
fn property_a_fight_never_creates_men() {
    for (seed, run) in seed_runs(40) {
        let events = run.events();
        for day in 1..=run.days() {
            // No unit has more men after a day than before it.
            for (id, after) in &run.at(day).land_units {
                let before = &run.at(day - 1).land_units[id];
                assert!(
                    after.men_ashore <= before.men_ashore,
                    "seed {seed}, day {day}: {id:?} had {} and has {}",
                    before.men_ashore,
                    after.men_ashore
                );
            }
            // Each side's men in each hex: no more after a fight than before.
            for side in Side::ALL {
                for hex in [X, Y, Z] {
                    let men = |state: &TruthState| -> u32 {
                        state
                            .land_units
                            .values()
                            .filter(|u| u.side == side && u.hex == Some(hex))
                            .map(|u| u.men_ashore)
                            .sum()
                    };
                    assert!(
                        men(run.at(day)) <= men(run.at(day - 1)),
                        "seed {seed}, day {day}, {side:?} at {hex}"
                    );
                }
            }
        }
        // And in the log: what a unit lost and what it kept add to what it
        // had at the start of the fight, and it lost no more than that.
        let all = fires(&events);
        for event in &events {
            if let EventKind::MenLostInFight {
                unit, lost, ashore, ..
            } = &event.kind
            {
                let had = fire_of(&all, event.day, unit.raw())
                    .expect("a unit that lost men in a fight fired in it")
                    .factors[0]
                    .1;
                assert_eq!(
                    n(i64::from(lost + ashore)),
                    had,
                    "seed {seed}, day {}",
                    event.day
                );
                assert!(*lost > 0);
            }
        }
    }
}

#[test]
fn property_men_are_conserved_with_the_fight_as_a_cause_of_loss() {
    let mut fought = 0;
    for (seed, run) in seed_runs(40) {
        let events = run.events();
        fought += events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::MenLostInFight { .. }))
            .count();
        for id in 1..=5 {
            let began = run.start.land_units[&unit(id)].men_ashore + run.start.men_aboard(unit(id));
            for day in 1..=run.days() {
                assert_eq!(
                    men_accounted(&run, &events, day, id),
                    began,
                    "seed {seed}, day {day}, unit {id}: men at the start = men ashore + aboard + lost"
                );
            }
        }
    }
    // The fight really was a cause of loss in this batch.
    assert!(fought > 100, "{fought} losses in fights");
}

#[test]
fn property_the_order_of_firing_does_not_matter() {
    // Both sides fire on the strengths they had at the start of the fight:
    // each unit's firing men are worked from its men before the day's fight,
    // after the night's unfed loss, and never after the other side's hits;
    // and the losses are the other side's hits times 20, capped at the men
    // there, taken from the start strengths.
    for (seed, run) in seed_runs(40) {
        let events = run.events();
        let all = fires(&events);
        for day in 1..=run.days() {
            let ctx = |what: &str| format!("seed {seed}, day {day}: {what}");
            // The men of each unit with men ashore at the start of the fight.
            let mut start: BTreeMap<u32, (Side, Hex, u32)> = BTreeMap::new();
            for (id, before) in &run.at(day - 1).land_units {
                let lost = lost_unfed(&events, day, id.raw()).map_or(0, |(lost, _)| lost);
                let men = before.men_ashore - lost;
                if let (true, Some(hex)) = (men > 0, before.hex) {
                    start.insert(id.raw(), (before.side, hex, men));
                }
            }
            let contested: Vec<Hex> = [X, Y, Z]
                .into_iter()
                .filter(|hex| !fight_events(&events, day, *hex).is_empty())
                .collect();
            for hex in &contested {
                let units_here: Vec<u32> = start
                    .iter()
                    .filter(|(_, (_, at, _))| at == hex)
                    .map(|(id, _)| *id)
                    .collect();
                // Every unit with men at the start fired, including those the
                // other side's hits then killed.
                let mut fired_here: Vec<u32> = all
                    .iter()
                    .filter(|f| f.day == day && f.hex == *hex)
                    .map(|f| f.unit)
                    .collect();
                fired_here.sort();
                assert_eq!(fired_here, units_here, "{}", ctx("who fired"));
                let mut hits = BTreeMap::new();
                for id in &units_here {
                    let (side, _, men) = start[id];
                    let fire = fire_of(&all, day, *id).unwrap();
                    // Fed share from the night's charge, 1 or 0 here;
                    // fatigue from the end of the day before.
                    let (_, charged, fed, _) = fed_on(&events, day, side, *hex).expect("fed");
                    assert_eq!(
                        charged,
                        start_men_of_side(&start, side, *hex)
                            + unfed_lost_of_side(&events, day, &run, side, *hex),
                        "{}",
                        ctx("the charge")
                    );
                    let share = if fed == charged { 1 } else { 0 };
                    assert!(fed == charged || fed == 0, "{}", ctx("a whole or no share"));
                    let fatigue = run.at(day - 1).land_units[&unit(*id)].fatigue;
                    let expected =
                        u32::try_from((i64::from(men) * share * (100 - fatigue.trunc())) / 100)
                            .unwrap();
                    assert_eq!(fire.firing_men, expected, "{}", ctx(&format!("unit {id}")));
                    *hits.entry(side).or_insert(0u32) += fire.hits;
                }
                // The losses.
                for side in Side::ALL {
                    let enemy = if side == Side::Allied {
                        Side::Japanese
                    } else {
                        Side::Allied
                    };
                    let there = start_men_of_side(&start, side, *hex);
                    let expected = (20 * hits.get(&enemy).copied().unwrap_or(0)).min(there);
                    let mut lost = 0;
                    for id in start
                        .iter()
                        .filter(|(_, (s, h, _))| *s == side && h == hex)
                        .map(|(id, _)| *id)
                    {
                        let part = lost_in_fight(&events, day, id).map_or(0, |(l, _)| l);
                        assert!(
                            part <= start[&id].2,
                            "{}",
                            ctx("a part is at most the unit's men")
                        );
                        lost += part;
                    }
                    assert_eq!(lost, expected, "{}", ctx(&format!("{side:?} at {hex}")));
                }
            }
            // Nothing fired in a hex with no fight.
            assert_eq!(
                all.iter().filter(|f| f.day == day).count(),
                start
                    .values()
                    .filter(|(_, hex, _)| contested.contains(hex))
                    .count(),
                "{}",
                ctx("fire only where there was a fight")
            );
        }
    }
}

/// The men of `side` in `hex` in a start-of-fight table.
fn start_men_of_side(start: &BTreeMap<u32, (Side, Hex, u32)>, side: Side, hex: Hex) -> u32 {
    start
        .values()
        .filter(|(s, h, _)| *s == side && *h == hex)
        .map(|(_, _, men)| *men)
        .sum()
}

/// The men of `side` in `hex` lost unfed on the night of `day`.
fn unfed_lost_of_side(events: &[Event], day: u32, run: &Run, side: Side, hex: Hex) -> u32 {
    run.at(day - 1)
        .land_units
        .values()
        .filter(|u| u.side == side && u.hex == Some(hex))
        .map(|u| lost_unfed(events, day, u.id.raw()).map_or(0, |(lost, _)| lost))
        .sum()
}

#[test]
fn property_a_bases_side_and_the_invariants_agree_at_the_end_of_every_day() {
    // The engine checks its invariants at the end of every day and fails the
    // day if one is broken; the runs below are made of days that passed. This
    // test also states the two the record names.
    let mut changed = 0;
    let mut sources = vec![text()];
    sources.push(with_posture_order(&text(), 1, "allied", 4, "defend"));
    sources.push(with_posture_order(&text(), 1, "allied", 1, "attack"));
    for source in sources {
        for seed in 1..=20 {
            let run = Run::text(&with_seed(&source, seed));
            for day in 1..=run.days() {
                let state = run.at(day);
                for group in state.air_groups.values() {
                    if let AirGroupLocation::Base(b) = group.location {
                        assert_eq!(state.bases[&b].side, group.side, "seed {seed}, day {day}");
                    }
                }
                for force in state.task_forces.values() {
                    if let Some(port) = force.home_port {
                        assert_eq!(
                            state.bases[&port].side, force.side,
                            "seed {seed}, day {day}"
                        );
                    }
                }
                for ship in state.ships.values() {
                    if let ShipLocation::Port(b) = ship.location {
                        assert_eq!(state.bases[&b].side, ship.side, "seed {seed}, day {day}");
                    }
                }
                // A base's side is what the log says it is: the side it began
                // with, changed by each BaseChangedSide so far.
                for (id, base) in &state.bases {
                    let mut side = run.start.bases[id].side;
                    for event in run.events().iter().filter(|e| e.day <= day) {
                        if let EventKind::BaseChangedSide { base: b, to, .. } = &event.kind {
                            if b == id {
                                side = *to;
                            }
                        }
                    }
                    assert_eq!(base.side, side, "seed {seed}, day {day}, base {id:?}");
                }
            }
            changed += run
                .events()
                .iter()
                .filter(|e| matches!(e.kind, EventKind::BaseChangedSide { .. }))
                .count();
        }
    }
    assert!(changed >= 60, "{changed} changes of side");
}

/// A scenario of one hex, Y, with a Japanese defender of 600 men and `men`
/// Allied attackers, each side fed from a base of its own with plenty of
/// supply, over `days` days at `seed`.
fn contest(men: u32, seed: u32, days: u32) -> String {
    let source = text();
    let header = &source[..source.find("[[base]]").expect("the first base")];
    let header = common::replace_once(header, "seed = 1942\n", &format!("seed = {seed}\n"));
    let header = common::replace_once(&header, "days = 6\n", &format!("days = {days}\n"));
    format!(
        "{header}\
[[base]]\nid = 1\nname = \"Placeholder Allied base at Y\"\nside = \"allied\"\nq = 6\nr = 0\nsupply_t = 1000\n\n\
[[base]]\nid = 2\nname = \"Placeholder Japanese base at Y\"\nside = \"japanese\"\nq = 6\nr = 0\nsupply_t = 1000\n\n\
[[land_unit]]\nid = 1\nname = \"Placeholder attackers\"\nside = \"allied\"\nmen = {men}\nq = 6\nr = 0\nposture = \"attack\"\n\n\
[[land_unit]]\nid = 2\nname = \"Placeholder defenders\"\nside = \"japanese\"\nmen = 600\nq = 6\nr = 0\n"
    )
}

/// The day the defender is first beaten, or `days + 1` if never.
fn day_beaten(men: u32, seed: u32, days: u32) -> u32 {
    let run = Run::text(&contest(men, seed, days));
    run.events()
        .iter()
        .find_map(|e| match &e.kind {
            EventKind::SideBeatenAshore {
                side: Side::Japanese,
                ..
            } => Some(e.day),
            _ => None,
        })
        .unwrap_or(days + 1)
}

/// More attackers never do worse: a defender of 600 fed men, at one hex, is
/// attacked by 300, 1000, 1500, 1700, 1900, 2100 and 2500 fed men, each over
/// the same 300 seeds and 8 days, and the mean day the defender is beaten
/// (never counted as day 9) does not rise as the attackers grow. The sizes
/// are chosen to straddle where the defender starts to fall (about 1500 to
/// 2100 men, where 0.2 of the attackers' men is near what 600 men lose in a
/// day or two), so that the means differ from step to step and the order is
/// a real check. 300 seeds make each mean steady to a few hundredths of a
/// day; the whole batch, 2,100 runs of 8 days, takes under a second in
/// release (about 0.7 s) and a few seconds in a debug build.
#[test]
fn property_more_attackers_never_do_worse_over_a_batch() {
    const SEEDS: u32 = 300;
    const DAYS: u32 = 8;
    let sizes = [300u32, 1000, 1500, 1700, 1900, 2100, 2500];
    let means: Vec<f64> = sizes
        .iter()
        .map(|men| {
            let total: u32 = (1..=SEEDS).map(|seed| day_beaten(*men, seed, DAYS)).sum();
            f64::from(total) / f64::from(SEEDS)
        })
        .collect();
    for pair in means.windows(2).zip(sizes.windows(2)) {
        assert!(
            pair.0[1] <= pair.0[0],
            "mean day beaten rose from {} at {} attackers to {} at {}: {means:?}",
            pair.0[0],
            pair.1[0],
            pair.0[1],
            pair.1[1]
        );
    }
    // And it is not vacuous: the smallest forces never win, and each step
    // from 1500 up makes the win come sooner.
    assert_eq!(means[0], f64::from(DAYS + 1), "{means:?}");
    assert!(means[2] < means[0], "{means:?}");
    assert!(means[3] < means[2] - 1.0, "{means:?}");
    assert!(means[4] < means[3] - 1.0, "{means:?}");
    assert!(means[5] < means[4] - 1.0, "{means:?}");
    assert!(means[6] < 2.0, "{means:?}");
}

// ------------------------------------------- A scenario with no posture order

fn is_ground_kind(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::FedShareRestored { .. }
            | EventKind::GroundFight { .. }
            | EventKind::GroundFire { .. }
            | EventKind::MenLostInFight { .. }
            | EventKind::LandUnitFatigue { .. }
            | EventKind::LandUnitBackToDefend { .. }
            | EventKind::SideBeatenAshore { .. }
            | EventKind::BaseChangedSide { .. }
            | EventKind::AirGroupLostWithBase { .. }
            | EventKind::ShipLostWithBase { .. }
            | EventKind::HomePortLost { .. }
            | EventKind::StrikeMissionCleared { .. }
    )
}

fn path_21() -> PathBuf {
    common::data_dir().join("scenarios/21_landing.toml")
}

fn text_21() -> String {
    std::fs::read_to_string(path_21()).expect("scenario 21 reads")
}

fn run_21(source: &str) -> Run {
    Run::of(common::from_text(source, &path_21()).expect("the variant of scenario 21 loads"))
}

#[test]
fn property_a_scenario_with_no_posture_order_is_unchanged_from_stage_1() {
    // Scenario 21 loads the ground figures, has land units and no posture
    // order. No event of the kinds decision 0090 added appears, and every unit
    // stays on defend with fatigue 0 and no standing order, the whole game.
    // (Its events are compared byte for byte by the run baselines.)
    let run = run_21(&text_21());
    assert!(run.scenario.world.ref_data().ground.is_some());
    assert!(!run.start.land_units.is_empty());
    assert_eq!(run.days(), 7);
    let new: Vec<String> = run
        .events()
        .iter()
        .filter(|e| is_ground_kind(&e.kind))
        .map(|e| e.kind.to_string())
        .collect();
    assert!(new.is_empty(), "{new:?}");
    for day in 0..=run.days() {
        for unit in run.at(day).land_units.values() {
            assert_eq!(unit.posture, Posture::Defend, "day {day} {:?}", unit.id);
            assert_eq!(unit.fatigue, Fixed::ZERO, "day {day} {:?}", unit.id);
            assert_eq!(unit.posture_order, None, "day {day} {:?}", unit.id);
        }
        for base in run.at(day).bases.values() {
            assert_eq!(base.side, Side::Allied, "day {day} {:?}", base.id);
        }
    }
}

// ------------------------------------------------- The posture order's refusals

/// Posture orders accepted (order, unit) and rejected (unit, reason).
type PostureAnswers = (Vec<(sim_core::ids::OrderId, u32)>, Vec<(u32, RejectReason)>);

/// The answers to the posture orders of `day`.
fn posture_answers(events: &[Event], day: u32) -> PostureAnswers {
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();
    for event in events.iter().filter(|e| e.day == day) {
        match &event.kind {
            EventKind::OrderAccepted {
                order,
                target: EntityRef::LandUnit(u),
            } => accepted.push((*order, u.raw())),
            EventKind::OrderRejected {
                target: EntityRef::LandUnit(u),
                reason,
                ..
            } => rejected.push((u.raw(), reason.clone())),
            _ => {}
        }
    }
    (accepted, rejected)
}

#[test]
fn a_posture_order_for_the_enemys_unit_is_refused_as_not_your_land_unit() {
    let source = with_posture_order(&text(), 1, "allied", 2, "defend");
    let run = Run::text(&source);
    let events = run.events();
    let (accepted, rejected) = posture_answers(&events, 1);
    assert!(accepted.is_empty());
    assert_eq!(rejected, vec![(2, RejectReason::NotYourLandUnit)]);
    // Nothing changed: unit 2 attacks on day 1 as it began, with no order.
    assert_eq!(run.unit_at(1, 2).fatigue, n(20));
    assert_eq!(run.unit_at(1, 2).posture_order, None);
    assert!(!fight_events(&events, 1, X).is_empty());
    // The same for a unit that does not exist.
    let source = with_posture_order(&text(), 1, "allied", 77, "defend");
    let events = Run::text(&source).events();
    assert_eq!(
        posture_answers(&events, 1).1,
        vec![(77, RejectReason::NotYourLandUnit)]
    );
}

#[test]
fn a_posture_order_for_a_destroyed_unit_is_refused_as_destroyed() {
    // Unit 3 has lost its last man by the end of day 3 (the dice of seed
    // 1942, read here and not assumed).
    let source = with_posture_order(&text(), 4, "japanese", 3, "attack");
    let run = Run::text(&source);
    assert_eq!(run.unit_at(3, 3).men_ashore, 0);
    assert_eq!(run.at(3).men_aboard(unit(3)), 0);
    let events = run.events();
    let (accepted, rejected) = posture_answers(&events, 4);
    assert!(accepted.is_empty());
    assert_eq!(rejected, vec![(3, RejectReason::LandUnitDestroyed)]);
    assert_eq!(run.unit_at(6, 3).posture, Posture::Defend);
    assert_eq!(run.unit_at(6, 3).posture_order, None);
}

#[test]
fn a_posture_order_for_a_unit_with_men_aboard_and_none_ashore_is_refused() {
    // Scenario 21: units 1 and 2 are aboard task force 1 on day 1; unit 3 is
    // ashore at the bare shore.
    let mut source = with_posture_order(&text_21(), 1, "allied", 1, "attack");
    source = with_posture_order(&source, 1, "allied", 3, "attack");
    let run = run_21(&source);
    let events = run.events();
    let (accepted, rejected) = posture_answers(&events, 1);
    assert_eq!(rejected, vec![(1, RejectReason::NoMenAshore)]);
    assert_eq!(accepted.len(), 1);
    assert_eq!(accepted[0].1, 3);
    assert_eq!(run.unit_at(1, 1).posture, Posture::Defend);
    assert_eq!(run.unit_at(1, 1).posture_order, None);
    assert_eq!(run.unit_at(1, 3).posture, Posture::Attack);
    assert_eq!(run.unit_at(1, 3).posture_order, Some(accepted[0].0));
}

#[test]
fn an_accepted_posture_order_sets_the_posture_and_the_order_and_the_fight_cites_it() {
    // The Allied regiment at X is ordered to attack on day 1: both sides
    // attack there, and the fight cites the order.
    let source = with_posture_order(&text(), 1, "allied", 1, "attack");
    let run = Run::text(&source);
    let events = run.events();
    let (accepted, rejected) = posture_answers(&events, 1);
    assert!(rejected.is_empty());
    assert_eq!(accepted.len(), 1);
    let order = accepted[0].0;
    assert_eq!(accepted[0].1, 1);
    let after = run.unit_at(1, 1);
    assert_eq!(after.posture, Posture::Attack);
    assert_eq!(after.posture_order, Some(order));
    let fights = fight_events(&events, 1, X);
    assert_eq!(fights.len(), 1);
    assert_eq!(
        fights[0].kind,
        EventKind::GroundFight {
            hex: X,
            allied_attacks: true,
            japanese_attacks: true,
        }
    );
    // The Japanese unit began on attack with no order, so the fight cites
    // the one order.
    assert_eq!(fights[0].causes, vec![Cause::Order(order)]);
    // Unit 1 now fires at 0.3 as an attacker and gains fatigue; unit 2 too.
    let all = fires(&events);
    assert_eq!(fire_of(&all, 1, 1).unwrap().chance, fx("0.3"));
    assert_eq!(fire_of(&all, 1, 2).unwrap().chance, fx("0.3"));
    assert_eq!(after.fatigue, n(20));
    // The world the order is given to answers to the same order on the order
    // book.
    assert!(run.scenario.world.orders().any(|o| o.id == order));

    // An order to defend holds a unit that began on attack: at Y nobody
    // attacks, so there is no fight there all game and base 3 stays
    // Japanese.
    let source = with_posture_order(&text(), 1, "allied", 4, "defend");
    let run = Run::text(&source);
    let events = run.events();
    let (accepted, rejected) = posture_answers(&events, 1);
    assert!(rejected.is_empty());
    assert_eq!(run.unit_at(1, 4).posture, Posture::Defend);
    assert_eq!(run.unit_at(1, 4).posture_order, Some(accepted[0].0));
    for day in 1..=6 {
        assert!(fight_events(&events, day, Y).is_empty(), "day {day}");
        assert_eq!(
            run.at(day).bases[&base(3)].side,
            Side::Japanese,
            "day {day}"
        );
    }
    assert!(!events
        .iter()
        .any(|e| matches!(&e.kind, EventKind::BaseChangedSide { base: b, .. } if *b == base(3))));
}

// --------------------------------------------------- The enemy's view of a fight

/// Everything a side may see at the end of each day, as text: its events with
/// their numbers, causes and breakdowns (their ids are opaque and print as
/// `_`), its bases, the enemy bases on its map, its land units, ships and
/// contacts.
fn view_by_day(source: &str, side: Side) -> Vec<String> {
    let mut scenario = from(source);
    let mut seen = Vec::new();
    while scenario.next_day().expect("a day").is_some() {
        let view = scenario.world.side_view(side);
        let events: Vec<String> = view.events().iter().map(side_event_text).collect();
        let bases: Vec<String> = view.bases().map(|b| format!("{b:?}")).collect();
        let enemy_bases: Vec<String> = view.enemy_bases().map(|b| format!("{b:?}")).collect();
        let units: Vec<String> = view.land_units().map(|u| format!("{u:?}")).collect();
        let ships: Vec<String> = view.ships().map(|s| format!("{s:?}")).collect();
        seen.push(format!(
            "day {}: events {events:?}\nbases {bases:?}\nenemy bases {enemy_bases:?}\nunits {units:?}\nships {ships:?}\ncontacts {}",
            scenario.world.day(),
            view.contacts().count()
        ));
    }
    seen
}

fn side_event_text(e: &SideEvent) -> String {
    format!(
        "{} D{} {:?} {:?} order {:?} causes {:?} hidden {} breakdown {:?}",
        e.number, e.day, e.pulse, e.kind, e.order_number, e.causes, e.hidden_causes, e.breakdown
    )
}

/// A fight at Y over `days` days: 1000 Allied men on attack, fed from a base
/// with plenty of supply, against the given Japanese land units and a
/// Japanese base with `japanese_supply` t.
fn view_world(japanese_units: &[(u32, u32, u32)], japanese_supply: u32, seed: u32) -> String {
    let source = text();
    let header = &source[..source.find("[[base]]").expect("the first base")];
    let header = common::replace_once(header, "seed = 1942\n", &format!("seed = {seed}\n"));
    let header = common::replace_once(&header, "days = 6\n", "days = 3\n");
    let mut out = format!(
        "{header}\
[[base]]\nid = 1\nname = \"Placeholder Allied base at Y\"\nside = \"allied\"\nq = 6\nr = 0\nsupply_t = 1000\n\n\
[[base]]\nid = 2\nname = \"Placeholder Japanese base at Y\"\nside = \"japanese\"\nq = 6\nr = 0\nsupply_t = {japanese_supply}\n\n\
[[land_unit]]\nid = 1\nname = \"Placeholder attackers\"\nside = \"allied\"\nmen = 1000\nq = 6\nr = 0\nposture = \"attack\"\n\n"
    );
    for (id, men, fatigue) in japanese_units {
        out.push_str(&format!(
            "[[land_unit]]\nid = {id}\nname = \"Placeholder defenders {id}\"\nside = \"japanese\"\nmen = {men}\nq = 6\nr = 0\nfatigue = {fatigue}\n\n"
        ));
    }
    out
}

fn japanese_men_by_day(run: &Run) -> Vec<u32> {
    (1..=run.days())
        .map(|day| {
            run.at(day)
                .land_units
                .values()
                .filter(|u| u.side == Side::Japanese)
                .map(|u| u.men_ashore)
                .sum()
        })
        .collect()
}

fn allied_losses_by_day(run: &Run) -> Vec<Option<(u32, u32)>> {
    let events = run.events();
    (1..=run.days())
        .map(|day| lost_in_fight(&events, day, 1))
        .collect()
}

#[test]
fn property_the_enemy_sees_the_same_fight_whatever_the_other_sides_fatigue_and_units() {
    // Part 1, with no dice to match: the Japanese side has no base to feed it
    // (a base with no supply), so none of its men is fed, its fed share is 0
    // and no man of it fires, in either world. The worlds differ in the
    // Japanese side's fatigue (60 against 0) and in its number of units (one
    // of 400 men against two, of 250 and 150). Its men are the same in both
    // (the unfed loss is the hex's, whoever holds the men) and the Allied
    // side's losses are the same, none.
    let one = view_world(&[(2, 400, 60)], 0, 1942);
    let two = view_world(&[(2, 250, 0), (3, 150, 0)], 0, 1942);
    let (a, b) = (Run::text(&one), Run::text(&two));
    assert_ne!(a.scenario.world.day_hashes(), b.scenario.world.day_hashes());
    assert_eq!(japanese_men_by_day(&a), japanese_men_by_day(&b));
    assert!(japanese_men_by_day(&a)[2] < 400);
    assert_eq!(allied_losses_by_day(&a), allied_losses_by_day(&b));
    assert!(allied_losses_by_day(&a).iter().all(Option::is_none));
    assert!(fires(&a.events())
        .iter()
        .filter(|f| f.unit != 1)
        .all(|f| f.firing_men == 0));
    let seen_one = view_by_day(&one, Side::Allied);
    let seen_two = view_by_day(&two, Side::Allied);
    assert_eq!(seen_one.len(), 3);
    assert_eq!(seen_one, seen_two);
    // The Japanese side's own view does differ: it sees its own units.
    assert_ne!(
        view_by_day(&one, Side::Japanese),
        view_by_day(&two, Side::Japanese)
    );
    // Nothing of the Japanese units is in the Allied events: no unit, no
    // fatigue, no fed share, no going back to defend.
    let mut checked = 0;
    for event in Run::text(&one)
        .scenario
        .world
        .side_view(Side::Allied)
        .events()
    {
        let text = side_event_text(&event);
        for word in [
            "LandUnitId(2)",
            "LandUnitId(3)",
            "fed_share\", value: 0",
            "LandUnitBackToDefend",
            "SideBeatenAshore",
        ] {
            assert!(!text.contains(word), "{word}: {text}");
        }
        checked += 1;
    }
    assert!(checked > 5);

    // Part 2, with the Japanese side fed and firing, and the dice matched:
    // the Japanese side's fatigue (0 against 40) and its number of units (one
    // of 400 men against two of 200) differ, and the seeds are those, found by
    // search, where it rolls the same hits every day, so that the Allied
    // side's losses are the same and so are the Japanese men. The Allied view
    // is then the same.
    let mut matched = 0;
    for seed in 1..=300 {
        let one = view_world(&[(2, 400, 0)], 1000, seed);
        let two = view_world(&[(2, 200, 40), (3, 200, 40)], 1000, seed);
        let (a, b) = (Run::text(&one), Run::text(&two));
        if allied_losses_by_day(&a) != allied_losses_by_day(&b)
            || japanese_men_by_day(&a) != japanese_men_by_day(&b)
        {
            continue;
        }
        matched += 1;
        // The worlds really differ.
        assert_ne!(a.scenario.world.day_hashes(), b.scenario.world.day_hashes());
        assert_ne!(
            fires(&a.events())
                .iter()
                .map(|f| f.firing_men)
                .collect::<Vec<_>>(),
            fires(&b.events())
                .iter()
                .map(|f| f.firing_men)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            view_by_day(&one, Side::Allied),
            view_by_day(&two, Side::Allied),
            "seed {seed}"
        );
    }
    assert!(matched >= 3, "only {matched} seeds matched in 300");
}

// ----------------------------------------- A base formed by a landing, taken

#[test]
fn a_base_formed_by_a_landing_and_taken_comes_onto_both_maps() {
    // Scenario 21: the Allied convoy lands units 1 and 2 at (8,0) and forms
    // base 2 there, a base on the Allied side's map only. A fed Japanese
    // force of 6,000 men on attack is already ashore there, with a base of
    // its own (base 3), and the Allied men, 1,000 at most, are beaten.
    let source = common::replace_once(
        &text_21(),
        "[[task_force]]\nid = 1\n",
        "[[base]]\nid = 3\nname = \"Placeholder Japanese base\"\nside = \"japanese\"\nq = 8\nr = 0\nsupply_t = 500\n\n\
[[land_unit]]\nid = 9\nname = \"Placeholder Japanese force\"\nside = \"japanese\"\nmen = 6000\nq = 8\nr = 0\nposture = \"attack\"\n\n\
[[task_force]]\nid = 1\n",
    );
    let run = run_21(&source);
    let events = run.events();
    let formed = events
        .iter()
        .find(|e| matches!(&e.kind, EventKind::BaseFormed { .. }))
        .expect("the landing forms a base");
    // The scenario's bases are 1 and 3, so the landing's base is made 4.
    let EventKind::BaseFormed {
        base: formed_id, ..
    } = &formed.kind
    else {
        unreachable!()
    };
    let formed_id = *formed_id;
    assert_eq!(formed_id, base(4));
    // It was on the Allied side's map only.
    assert_eq!(formed.day, 2);
    let day_before = run.at(1);
    assert!(!day_before.bases.contains_key(&formed_id));
    let changed = events
        .iter()
        .find(|e| matches!(&e.kind, EventKind::BaseChangedSide { base: b, .. } if *b == formed_id))
        .expect("the Japanese force takes the base");
    assert!(changed.id > formed.id);
    assert!(matches!(
        &changed.kind,
        EventKind::BaseChangedSide {
            from: Side::Allied,
            to: Side::Japanese,
            ..
        }
    ));
    // At the end it is Japanese, on the map, and on both sides' lists.
    let end = &run.at(7).bases[&formed_id];
    assert_eq!(end.side, Side::Japanese);
    assert!(end.on_map);
    let world = &run.scenario.world;
    let japanese = world.side_view(Side::Japanese);
    assert!(japanese.bases().any(|b| b.id == formed_id));
    assert!(!japanese.enemy_bases().any(|b| b.id == formed_id));
    let allied = world.side_view(Side::Allied);
    assert!(!allied.bases().any(|b| b.id == formed_id));
    let seen: Vec<_> = allied.enemy_bases().filter(|b| b.id == formed_id).collect();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].side, Side::Japanese);
    assert_eq!(seen[0].hex, Hex::new(8, 0));
    // The former holder is told it changed side; the new holder too.
    for view in [&allied, &japanese] {
        assert!(view.events().iter().any(|e| matches!(
            &e.kind,
            SideEventKind::BaseChangedSide { base: b, .. } if *b == formed_id
        ) || matches!(
            &e.kind,
            SideEventKind::Whole(EventKind::BaseChangedSide { base: b, .. }) if *b == formed_id
        )));
    }
}

// ----------------------------------------- Decision 24: a base's past

fn named(event: &SideEvent, id: BaseId) -> bool {
    event.kind.entities().contains(&EntityRef::Base(id))
}

fn is_change_of(event: &SideEvent, id: BaseId) -> bool {
    matches!(
        &event.kind,
        SideEventKind::BaseChangedSide { base: b, .. }
            | SideEventKind::Whole(EventKind::BaseChangedSide { base: b, .. }) if *b == id
    )
}

#[test]
fn a_bases_events_are_seen_by_the_side_that_held_it_when_they_were_recorded() {
    let run = Run::of(load());
    let world = &run.scenario.world;
    let japanese = world.side_view(Side::Japanese).events();
    let allied = world.side_view(Side::Allied).events();

    // The Japanese side still sees its own night of day 1 at base 3: its
    // 300 men at Y were not fed from it, none paid.
    let fed = japanese
        .iter()
        .find(|e| {
            matches!(&e.kind, SideEventKind::Whole(EventKind::MenFed {
                side: Side::Japanese, hex, base: Some(b), men: 300, fed: 0, unfed: 300 })
                if *hex == Y && *b == base(3))
        })
        .expect("the Japanese side keeps its day 1 night at base 3");
    assert_eq!(fed.day, 1);
    // It never sees an end-of-day count or a supply spent of base 3 or base
    // 5: after the change they are the Allied side's, and before it there was
    // none to see (the count comes after the ground combat phase).
    for id in [base(3), base(5)] {
        for event in japanese.iter().filter(|e| named(e, id)) {
            let ok = matches!(
                &event.kind,
                SideEventKind::Whole(EventKind::MenFed { .. })
                    | SideEventKind::Whole(EventKind::AirGroupLostWithBase { .. })
                    | SideEventKind::Whole(EventKind::ShipLostWithBase { .. })
                    | SideEventKind::Whole(EventKind::HomePortLost { .. })
                    | SideEventKind::Whole(EventKind::AirGroupRebased { .. })
                    | SideEventKind::BaseChangedSide { .. }
            );
            assert!(ok, "{:?}", event.kind);
        }
    }
    // It sees base 3 and base 5 change side, as the enemy's.
    assert!(japanese.iter().any(|e| is_change_of(e, base(3))));
    assert!(japanese.iter().any(|e| is_change_of(e, base(5))));

    // The Allied side sees no event of base 3 or base 5 from before the
    // change: the first event it sees that names either is the change, and
    // after it the end-of-day counts of the base, now its own.
    for id in [base(3), base(5)] {
        let first = allied
            .iter()
            .position(|e| named(e, id))
            .expect("the Allied side sees its new base");
        assert!(is_change_of(&allied[first], id), "{:?}", allied[first].kind);
        assert!(allied
            .iter()
            .filter(|e| named(e, id))
            .skip(1)
            .all(|e| matches!(
                &e.kind,
                SideEventKind::Whole(EventKind::BaseAtDayEnd { .. })
                    | SideEventKind::Whole(EventKind::BaseSupplySpent { .. })
                    | SideEventKind::Whole(EventKind::MenFed { .. })
            )));
    }
    // The day 1 night's Japanese charge at base 3 is not in the Allied view.
    assert!(!allied.iter().any(|e| matches!(
        &e.kind,
        SideEventKind::Whole(EventKind::MenFed {
            side: Side::Japanese,
            ..
        })
    )));
}

#[test]
fn a_bases_change_of_side_does_not_renumber_what_its_old_side_had_been_shown() {
    // The Japanese view's events before the fight at Y are the same, by
    // number, cause and text, in the run where base 3 changes side and in the
    // run where it does not (unit 4 ordered to defend, so nobody attacks at
    // Y).
    let changes = Run::of(load());
    let holds = Run::text(&with_posture_order(&text(), 1, "allied", 4, "defend"));
    assert_eq!(
        changes.at(1).bases[&base(3)].side,
        Side::Allied,
        "the base changes side in one run"
    );
    assert_eq!(
        holds.at(1).bases[&base(3)].side,
        Side::Japanese,
        "and not in the other"
    );
    let a = changes.scenario.world.side_view(Side::Japanese).events();
    let b = holds.scenario.world.side_view(Side::Japanese).events();
    let fight_at_y = a
        .iter()
        .position(|e| matches!(&e.kind, SideEventKind::GroundFight { hex } if *hex == Y))
        .expect("the Japanese side sees the fight at Y");
    assert!(fight_at_y >= 10, "the night and the fight at X come first");
    let before = |events: &[SideEvent]| -> Vec<String> {
        events[..fight_at_y].iter().map(side_event_text).collect()
    };
    assert_eq!(before(&a), before(&b));
    assert_eq!(a[..fight_at_y].last().unwrap().number as usize, fight_at_y);
    // And the two runs do differ from then on.
    assert_ne!(
        a.iter().map(side_event_text).collect::<Vec<_>>(),
        b.iter().map(side_event_text).collect::<Vec<_>>()
    );
}
