//! Scenario 25: a unit marches to the next hex with supply, forms a dump
//! there and attacks on arrival; a march with no land link is refused; and a
//! unit that marches from a shore with no base carries nothing.
//!
//! Proves decision 0094 (marching overland) against a hand calculation. The
//! data is placeholder data: see `data/scenarios/25_march.toml`. The test
//! runs on the frozen step 1 figures (`data/reference/step1`), to which the
//! three new ground figures are added with their first values.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide, the
//! calculation gives the firing men, the shots and the chance exactly and
//! what must follow from the hits the run rolls; the tests read the hits
//! from the run and assert the rest. Everything else is asserted exactly. If
//! the code disagrees, the code or the rule is wrong: do not change a number
//! here to make a test pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! `ground.toml`: `march_days_per_hex` 3, `fatigue_per_march_day` 10,
//! `march_supply_days` 5, and from the earlier stages
//! `daily_supply_t_per_man` 0.005, `unfed_loss_share_per_day` 0.05,
//! `men_per_shot` 100, `hit_chance` 0.3, `defender_hit_chance` 0.6,
//! `men_per_hit` 20, `fatigue_per_attack_day` 20, `rest_per_day` 5,
//! `attack_fatigue_limit` 60, `beaten_share` 0.2.
//!
//! ## Inputs
//!
//! A placeholder map with four coast hexes on one row: P at (0,0), Q at
//! (1,0), R at (2,0) and S at (3,0). Its land links are P to Q and R to S.
//! Q and R are neighbours with no land link. The game is 5 days.
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Allied   | P   | 100 t  |
//! | 2    | Japanese | Q   | 50 t   |
//!
//! | Land unit | Side     | Men  | Ashore at | Posture at the start |
//! | --------- | -------- | ---- | --------- | -------------------- |
//! | 1         | Allied   | 1000 | P         | attack               |
//! | 2         | Japanese | 400  | Q         | defend               |
//! | 3         | Japanese | 100  | R         | defend               |
//!
//! Orders: on day 1, unit 1 marches to Q and unit 3 marches to S. On day 4,
//! unit 1 is ordered to march to R.
//!
//! Each night the men's supply is charged first. In the ground combat phase
//! the marches come first, then the fights, then fatigue.
//!
//! ## Unit 1: the march from P to Q
//!
//! | Day | Night: base 1 pays, and holds | Ground combat phase | Unit 1 after the day |
//! | --- | ----------------------------- | ------------------- | -------------------- |
//! | 1   | 5 t, 95 t                     | One day marched. No enemy at P: no fight | At P, fatigue 10 |
//! | 2   | 5 t, 90 t                     | Two days marched    | At P, fatigue 20     |
//! | 3   | 5 t, 85 t                     | Three days marched: it arrives at Q | At Q; see below |
//!
//! On arrival, 5 days of the charge for its men, 5 x 1000 x 0.005 = 25 t,
//! are taken from base 1, which holds 60 t. The Allied side has no base at
//! Q, so base 3 is formed there, Allied, not on the Japanese map, and holds
//! the 25 t. The march order is spent. No Allied man is at P afterwards, so
//! base 1 pays nothing more and holds 60 t at the end.
//!
//! ## Q: the fight on arrival
//!
//! Unit 2 is fed by base 2 every night: 0.005 t for each man it has, so 2 t
//! on each of the first three nights (base 2 holds 48, 46 and 44 t). Unit 2
//! rests and stays at fatigue 0.
//!
//! Day 3. The Allied side attacks and the Japanese side defends.
//!
//! - Unit 1: its fatigue before the day's change is 20. Firing men 1000 x 1
//!   x 80/100 = 800. 8 shots at 0.3.
//! - Unit 2: firing men 400. 4 shots at 0.6.
//!
//! With a the hits of unit 1 and j the hits of unit 2: unit 2 has 400 less
//! 20 x a, at least 240, and unit 1 has 1000 less 20 x j, at least 920.
//!
//! Unit 1 marched and fired on attack that day: it gains 10 and 20, and its
//! fatigue is 50. It stays on attack, since 50 is not above 60.
//!
//! Beaten: the Japanese side is tested. It has at least 240 men against 0.2
//! times at most 1000. It is not beaten on day 3.
//!
//! The ground reports of day 3: each side gets an estimate of the other's
//! men at Q, between 0.5 and 1.5 times the truth after the fight, cut.
//! Base 3 is put on the Japanese map.
//!
//! Day 4, at intake: the order for unit 1 to march to R is refused, as
//! there is no land link from Q to R.
//!
//! Night of day 4: base 3 pays 0.005 t for each man unit 1 has and all are
//! fed; base 2 pays for each man unit 2 has.
//!
//! Day 4, the fight. Unit 1's fatigue before the day's change is 50: its
//! firing men are its men times 50/100, cut, at 0.3. Unit 2 fires all its
//! men at 0.6. Unit 1 fired on attack and did not march: it gains 20, to 70,
//! which is above 60, and it goes back to defend. The Japanese side is
//! tested again by the men left: it is beaten if its men are fewer than 0.2
//! times unit 1's men, and base 2 then becomes Allied.
//!
//! Day 5. No unit at Q is on attack: no fight. Unit 1 rests to 65. A ground
//! report is made for each side if both still have men at Q.
//!
//! At the end base 3 holds 25 t less 0.005 t for each man unit 1 had on the
//! nights of days 4 and 5.
//!
//! ## Unit 3: a march from a shore with no base
//!
//! The Japanese side has no base at R or at S. Unit 3 is unfed every night
//! and loses its men times 0.05, cut, and one man at least.
//!
//! | Day | Night: lost, left | Ground combat phase                  | Unit 3 after the day |
//! | --- | ----------------- | ------------------------------------ | -------------------- |
//! | 1   | 5, 95             | One day marched                      | At R, fatigue 10     |
//! | 2   | 4.75, cut to 4, 91 | Two days marched                    | At R, fatigue 20     |
//! | 3   | 4.55, cut to 4, 87 | Three days marched: it arrives at S | At S, fatigue 30     |
//! | 4   | 4.35, cut to 4, 83 | It rests                            | At S, fatigue 25     |
//! | 5   | 4.15, cut to 4, 79 | It rests                            | At S, fatigue 20     |
//!
//! On arrival there is no base of its side at R to take supply from: it
//! carries nothing, and no base is formed at S. At the end unit 3 has 79
//! men at S and has lost 21 unfed.
//!
//! # Properties
//!
//! - Supply is conserved by a march: what leaves the bases of the hex left
//!   is what enters the base of the hex reached.
//! - Men are conserved by a march.
//! - A unit never marches where the map has no land link, and a map with no
//!   land links allows no march.
//! - The enemy's view is the same in two worlds that differ only in a march
//!   to a hex where the enemy has no men.
//! - A scenario with no march order is unchanged, apart from the hash
//!   version.

mod common;

use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind, LandFatigueChange, MarchStop, SupplyUse};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EntityRef, LandUnitId, OrderId};
use sim_core::orders::{Posture, RejectReason};
use sim_core::side_events::SideEventKind;
use sim_core::state::{Side, TruthState};
use sim_core::World;
use sim_data::Scenario;

// ------------------------------------------------------------------ Helpers

const P: Hex = Hex::new(0, 0);
const Q: Hex = Hex::new(1, 0);
const R: Hex = Hex::new(2, 0);
const S: Hex = Hex::new(3, 0);

fn path() -> PathBuf {
    common::data_dir().join("scenarios/25_march.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn from(source: &str) -> Scenario {
    common::from_text(source, &path()).expect("the variant loads")
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

/// What a side believed at the end of a day, as its view gives it.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Belief {
    /// Each ground estimate: hex, men, day.
    estimates: Vec<(Hex, u32, u32)>,
    enemy_bases: Vec<BaseId>,
    own_bases: Vec<BaseId>,
}

struct Snapshot {
    truth: TruthState,
    allied: Belief,
    japanese: Belief,
}

fn belief_of(world: &World, side: Side) -> Belief {
    let view = world.side_view(side);
    Belief {
        estimates: view
            .ground_estimates()
            .map(|e| (e.hex, e.men, e.day))
            .collect(),
        enemy_bases: view.enemy_bases().map(|b| b.id).collect(),
        own_bases: view.bases().map(|b| b.id).collect(),
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
    events: Vec<Event>,
}

impl Run {
    fn of(mut scenario: Scenario) -> Run {
        let mut days = vec![snapshot(&scenario.world)];
        while scenario.next_day().expect("a day resolves").is_some() {
            days.push(snapshot(&scenario.world));
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

    fn men(&self, day: u32, raw: u32) -> u32 {
        self.at(day).land_units[&unit(raw)].men_ashore
    }

    fn hex(&self, day: u32, raw: u32) -> Option<Hex> {
        self.at(day).land_units[&unit(raw)].hex
    }

    fn fatigue(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).land_units[&unit(raw)].fatigue
    }

    fn supply(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).bases[&base(raw)].supply_t
    }

    fn march_days(&self, day: u32, raw: u32) -> Option<u32> {
        self.at(day).land_units[&unit(raw)].march.map(|m| m.days)
    }

    /// The events of `day` that satisfy `pick`, in log order.
    fn on(&self, day: u32, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.day == day && pick(&e.kind))
            .collect()
    }

    /// The one event of `day` that satisfies `pick`; panics unless exactly one.
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

    fn marches(&self) -> Vec<&Event> {
        self.all(is_march_event)
    }
}

fn is_march_event(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::MarchDay { .. }
            | EventKind::MarchBaseFormed { .. }
            | EventKind::MarchArrived { .. }
            | EventKind::MarchStopped { .. }
    )
}

/// The one fire of a unit on a day: (firing men, shots, chance, hits).
fn fire(run: &Run, day: u32, raw: u32) -> (u32, Fixed, Fixed, u32) {
    let e = run.one(
        day,
        |k| matches!(k, EventKind::GroundFire { unit: u, .. } if u.raw() == raw),
    );
    match e.kind {
        EventKind::GroundFire {
            firing_men,
            shots,
            chance,
            hits,
            ..
        } => (firing_men, shots, chance, hits),
        _ => unreachable!(),
    }
}

fn fatigue_event(run: &Run, day: u32, raw: u32) -> &Event {
    run.one(
        day,
        |k| matches!(k, EventKind::LandUnitFatigue { unit: u, .. } if u.raw() == raw),
    )
}

fn fatigue_change(run: &Run, day: u32, raw: u32) -> LandFatigueChange {
    match fatigue_event(run, day, raw).kind {
        EventKind::LandUnitFatigue { change, .. } => change,
        _ => unreachable!(),
    }
}

/// The ground reports of `day` for `side`: (men, replaced).
fn reports(run: &Run, day: u32, side: Side) -> Vec<(u32, bool)> {
    run.on(
        day,
        |k| matches!(k, EventKind::GroundReportReceived { side: s, .. } if *s == side),
    )
    .iter()
    .map(|e| match e.kind {
        EventKind::GroundReportReceived { men, replaced, .. } => (men, replaced),
        _ => unreachable!(),
    })
    .collect()
}

/// A report of `men` is a share of between 0.5 and 1.5 of the truth, cut.
fn in_report_bounds(men: u32, truth: u32) -> bool {
    men >= truth / 2 && men <= truth * 3 / 2
}

/// The order, of `day`, that was refused for a unit, with its reason.
fn refusals(run: &Run, day: u32) -> Vec<(u32, RejectReason)> {
    run.on(day, |k| matches!(k, EventKind::OrderRejected { .. }))
        .iter()
        .map(|e| match &e.kind {
            EventKind::OrderRejected {
                target: EntityRef::LandUnit(u),
                reason,
                ..
            } => (u.raw(), reason.clone()),
            other => panic!("a refusal for something else: {other:?}"),
        })
        .collect()
}

/// The order accepted on `day` for a unit.
fn accepted(run: &Run, day: u32, raw: u32) -> OrderId {
    let e = run.one(day, |k| {
        matches!(k, EventKind::OrderAccepted { target: EntityRef::LandUnit(u), .. } if u.raw() == raw)
    });
    match e.kind {
        EventKind::OrderAccepted { order, .. } => order,
        _ => unreachable!(),
    }
}

/// The text with a march order appended.
fn with_march(source: &str, day: u32, side: &str, land_unit: u32, hex: (i32, i32)) -> String {
    format!(
        "{source}\n[[order]]\nday = {day}\nside = \"{side}\"\nland_unit = {land_unit}\nkind = \"march\"\nhex = [{}, {}]\n",
        hex.0, hex.1
    )
}

/// The three orders of the scenario's text, each by its day and unit.
const ORDER_1: &str =
    "[[order]]\nday = 1\nside = \"allied\"\nland_unit = 1\nkind = \"march\"\nhex = [1, 0]\n";
const ORDER_3: &str =
    "[[order]]\nday = 1\nside = \"japanese\"\nland_unit = 3\nkind = \"march\"\nhex = [3, 0]\n";
const ORDER_4: &str =
    "[[order]]\nday = 4\nside = \"allied\"\nland_unit = 1\nkind = \"march\"\nhex = [2, 0]\n";

fn without(source: &str, order: &str) -> String {
    common::replace_once(source, order, "")
}

fn without_orders(source: &str) -> String {
    without(&without(&without(source, ORDER_1), ORDER_3), ORDER_4)
}

/// A base appended: id, side, hex, supply.
fn with_base(source: &str, id: u32, side: &str, hex: Hex, supply: u32) -> String {
    format!(
        "{source}\n[[base]]\nid = {id}\nname = \"Placeholder base {id}\"\nside = \"{side}\"\nq = {}\nr = {}\nsupply_t = {supply}\n",
        hex.q, hex.r
    )
}

/// The map of scenario 25 with its land links removed, written to a file of
/// its own, and the scenario text pointing at it.
fn without_land_links(source: &str) -> String {
    let map = std::fs::read_to_string(common::data_dir().join("map/placeholder_march.toml"))
        .expect("the map reads");
    let bare = map.split("[[land_link]]").next().expect("a first part");
    assert!(!bare.contains("land_link"));
    let file = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("map_scenario_25_no_links.toml");
    std::fs::write(&file, bare).expect("write the bare map");
    common::replace_once(
        source,
        "map = \"../map/placeholder_march.toml\"",
        &format!("map = {:?}", file.to_string_lossy()),
    )
}

/// A fast transport at base 1 (class 11, room for 200 men), in a force of
/// its own, from the placeholder transports file, as scenario 21 has them.
fn with_transport(source: &str) -> String {
    let source = common::replace_once(
        source,
        "ship_classes = \"../reference/ship_classes.csv\"",
        "ship_classes = [\n    \"../reference/ship_classes.csv\",\n    \"../reference/ship_classes_placeholder_transports.csv\",\n]",
    );
    format!(
        "{source}\n[[task_force]]\nid = 1\nname = \"Placeholder transport\"\nside = \"allied\"\nat_base = 1\nmission = \"transport\"\n\n[[ship]]\nid = 1\nname = \"Placeholder fast transport\"\nclass = 11\nside = \"allied\"\nfuel_t = 500\ntask_force = 1\n"
    )
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_march_of_unit_1_from_p_to_q() {
    let run = Run::text(&text());
    assert_eq!(run.len(), 5);
    let order = accepted(&run, 1, 1);

    // Where unit 1 is, how tired and how far it has marched, day by day.
    assert_eq!(run.hex(0, 1), Some(P));
    for (day, hex, fatigue, marched) in [
        (1, P, n(10), Some(1)),
        (2, P, n(20), Some(2)),
        (3, Q, n(50), None),
    ] {
        assert_eq!(run.hex(day, 1), Some(hex), "day {day}");
        assert_eq!(run.fatigue(day, 1), fatigue, "day {day}");
        assert_eq!(run.march_days(day, 1), marched, "day {day}");
    }
    // Days 1 and 2: no man lost, no fight at P.
    assert_eq!(run.men(1, 1), 1000);
    assert_eq!(run.men(2, 1), 1000);
    assert!(run
        .on(1, |k| matches!(k, EventKind::GroundFight { .. }))
        .is_empty());
    assert!(run
        .on(2, |k| matches!(k, EventKind::GroundFight { .. }))
        .is_empty());

    // Each day marched is an event that cites the order.
    for day in 1..=3 {
        let e = run.one(
            day,
            |k| matches!(k, EventKind::MarchDay { unit: u, .. } if u.raw() == 1),
        );
        assert_eq!(
            e.kind,
            EventKind::MarchDay {
                unit: unit(1),
                from: P,
                to: Q,
                days: day,
                of: 3,
            }
        );
        assert_eq!(e.causes, vec![Cause::Order(order)]);
    }

    // Base 1 pays 5 t each night, and nothing after the arrival.
    for (day, left) in [(1, 95), (2, 90), (3, 60), (4, 60), (5, 60)] {
        assert_eq!(run.supply(day, 1), n(left), "day {day}");
    }
    for (day, left) in [(1, 95), (2, 90), (3, 85)] {
        let spent = run.one(day, |k| {
            matches!(k, EventKind::BaseSupplySpent { base: b, reason: SupplyUse::Men, .. } if b.raw() == 1)
        });
        assert_eq!(
            spent.kind,
            EventKind::BaseSupplySpent {
                base: base(1),
                reason: SupplyUse::Men,
                aircraft: 0,
                men: 1000,
                asked_t: n(5),
                paid_t: n(5),
                left_t: n(left),
            }
        );
    }
    // After the arrival no Allied man is at P: base 1 pays nothing more.
    for day in 4..=5 {
        assert!(run
            .on(
                day,
                |k| matches!(k, EventKind::BaseSupplySpent { base: b, .. } if b.raw() == 1)
            )
            .is_empty());
    }

    // The arrival: 25 t taken from base 1 (60 t left), base 3 formed at Q.
    let day_event = run.one(
        3,
        |k| matches!(k, EventKind::MarchDay { unit: u, .. } if u.raw() == 1),
    );
    let taken = run.one(3, |k| {
        matches!(
            k,
            EventKind::BaseSupplySpent {
                reason: SupplyUse::March,
                ..
            }
        )
    });
    assert_eq!(
        taken.kind,
        EventKind::BaseSupplySpent {
            base: base(1),
            reason: SupplyUse::March,
            aircraft: 0,
            men: 1000,
            asked_t: n(25),
            paid_t: n(25),
            left_t: n(60),
        }
    );
    assert_eq!(taken.causes, vec![Cause::Event(day_event.id)]);
    let formed = run.one(3, |k| matches!(k, EventKind::MarchBaseFormed { .. }));
    assert_eq!(
        formed.kind,
        EventKind::MarchBaseFormed {
            base: base(3),
            side: Side::Allied,
            hex: Q,
            unit: unit(1),
        }
    );
    assert_eq!(
        formed.causes,
        vec![Cause::Event(day_event.id), Cause::Event(taken.id)]
    );
    let arrived = run.one(
        3,
        |k| matches!(k, EventKind::MarchArrived { unit: u, .. } if u.raw() == 1),
    );
    assert_eq!(
        arrived.kind,
        EventKind::MarchArrived {
            unit: unit(1),
            from: P,
            to: Q,
            men: 1000,
            supply_t: n(25),
            base: Some(base(3)),
        }
    );
    assert_eq!(
        arrived.causes,
        vec![
            Cause::Event(day_event.id),
            Cause::Event(taken.id),
            Cause::Event(formed.id)
        ]
    );
    // Nine march events in the game: unit 1's three days, base and arrival,
    // and unit 3's three days and arrival. None is a stop.
    assert_eq!(run.marches().len(), 9);

    // Base 3: Allied, at Q, formed with 25 t, and not on the map the Japanese
    // side has until the ground report of day 3 finds it.
    assert!(!run.at(2).bases.contains_key(&base(3)));
    let truth = &run.at(3).bases[&base(3)];
    assert_eq!(
        (truth.side, truth.hex, truth.on_map),
        (Side::Allied, Q, true)
    );
    assert_eq!(run.supply(3, 3), n(25));
    let found = run.one(3, |k| matches!(k, EventKind::BaseFound { .. }));
    assert_eq!(
        found.kind,
        EventKind::BaseFound {
            side: Side::Japanese,
            base: base(3),
            hex: Q,
        }
    );
    assert!(found.id > formed.id);
    let report = run.one(3, |k| {
        matches!(
            k,
            EventKind::GroundReportReceived {
                side: Side::Japanese,
                ..
            }
        )
    });
    assert_eq!(found.causes, vec![Cause::Event(report.id)]);
    assert!(run.belief(3, Side::Japanese).enemy_bases.contains(&base(3)));
    assert!(run.belief(3, Side::Allied).own_bases.contains(&base(3)));
    assert!(!run.belief(3, Side::Allied).enemy_bases.contains(&base(3)));
    assert_eq!(
        run.all(|k| matches!(k, EventKind::BaseFound { .. })).len(),
        1,
        "the Japanese side finds it once, and the Allied side finds nothing"
    );

    // The march order is spent: no march at the end, and no day marched on
    // days 4 and 5.
    for day in 4..=5 {
        assert_eq!(run.march_days(day, 1), None);
        assert!(run
            .on(
                day,
                |k| matches!(k, EventKind::MarchDay { unit: u, .. } if u.raw() == 1)
            )
            .is_empty());
    }
}

#[test]
fn unit_2_is_fed_by_base_2_and_the_fight_on_day_3_follows_the_calculation() {
    let run = Run::text(&text());
    // Unit 2 is fed 2 t a night on the first three nights and stays rested.
    for (day, left) in [(1, 48), (2, 46), (3, 44)] {
        assert_eq!(run.supply(day, 2), n(left), "day {day}");
        assert_eq!(run.fatigue(day, 2), Fixed::ZERO, "day {day}");
    }
    assert_eq!(run.men(2, 2), 400);
    assert_eq!(run.at(3).land_units[&unit(2)].fed_share, Fixed::ONE);

    // Day 3: the Allied side attacks and the Japanese side defends.
    let fight = run.one(3, |k| matches!(k, EventKind::GroundFight { .. }));
    assert_eq!(
        fight.kind,
        EventKind::GroundFight {
            hex: Q,
            allied_attacks: true,
            japanese_attacks: false,
        }
    );
    let (men1, shots1, chance1, a) = fire(&run, 3, 1);
    let (men2, shots2, chance2, j) = fire(&run, 3, 2);
    assert_eq!((men1, shots1, chance1), (800, n(8), fx("0.3")));
    assert_eq!((men2, shots2, chance2), (400, n(4), fx("0.6")));
    assert!(a <= 8 && j <= 4);
    // Unit 2 has 400 less 20 a, at least 240; unit 1 has 1000 less 20 j, at
    // least 920.
    assert_eq!(run.men(3, 2), 400 - 20 * a);
    assert_eq!(run.men(3, 1), 1000 - 20 * j);
    assert!(run.men(3, 2) >= 240 && run.men(3, 1) >= 920);

    // Unit 1 marched and fired on attack: 20 + 10 on top of 20, so 50, on
    // attack still. Its change cites its fire and its day marched.
    assert_eq!(run.fatigue(3, 1), n(50));
    assert_eq!(
        fatigue_change(&run, 3, 1),
        LandFatigueChange::MarchedAndAttacked
    );
    let fatigue = fatigue_event(&run, 3, 1);
    let fired = run.one(
        3,
        |k| matches!(k, EventKind::GroundFire { unit: u, .. } if u.raw() == 1),
    );
    let day_event = run.one(
        3,
        |k| matches!(k, EventKind::MarchDay { unit: u, .. } if u.raw() == 1),
    );
    assert!(fatigue.causes.contains(&Cause::Event(fired.id)));
    assert!(fatigue.causes.contains(&Cause::Event(day_event.id)));
    assert_eq!(run.at(3).land_units[&unit(1)].posture, Posture::Attack);
    assert!(run
        .on(3, |k| matches!(k, EventKind::LandUnitBackToDefend { .. }))
        .is_empty());

    // Days 1 and 2: a day marched gains 10 in place of rest, citing the day.
    for day in 1..=2 {
        assert_eq!(fatigue_change(&run, day, 1), LandFatigueChange::Marched);
        let day_event = run.one(
            day,
            |k| matches!(k, EventKind::MarchDay { unit: u, .. } if u.raw() == 1),
        );
        assert_eq!(
            fatigue_event(&run, day, 1).causes,
            vec![Cause::Event(day_event.id)]
        );
    }

    // Not beaten on day 3: at least 240 men against 0.2 of at most 1000. Base
    // 2 stays Japanese.
    assert!(run
        .on(3, |k| matches!(k, EventKind::SideBeatenAshore { .. }))
        .is_empty());
    assert!(run
        .on(3, |k| matches!(k, EventKind::BaseChangedSide { .. }))
        .is_empty());
    assert_eq!(run.at(3).bases[&base(2)].side, Side::Japanese);

    // The ground reports of day 3, between 0.5 and 1.5 times the truth after
    // the fight, cut; the first of the game.
    let allied = reports(&run, 3, Side::Allied);
    let japanese = reports(&run, 3, Side::Japanese);
    assert_eq!(allied.len(), 1);
    assert_eq!(japanese.len(), 1);
    assert!(!allied[0].1 && !japanese[0].1);
    assert!(in_report_bounds(allied[0].0, run.men(3, 2)));
    assert!(in_report_bounds(japanese[0].0, run.men(3, 1)));
    assert_eq!(
        run.belief(3, Side::Allied).estimates,
        vec![(Q, allied[0].0, 3)]
    );
    assert_eq!(
        run.belief(3, Side::Japanese).estimates,
        vec![(Q, japanese[0].0, 3)]
    );
}

#[test]
fn day_4_and_day_5_follow_the_calculation() {
    let run = Run::text(&text());

    // Day 4, at intake: the order to march to R is refused, as no land link
    // joins Q to R. Nothing changes for unit 1.
    assert_eq!(refusals(&run, 4), vec![(1, RejectReason::NoLandLink(R))]);
    assert_eq!(run.march_days(4, 1), None);
    assert_eq!(run.hex(4, 1), Some(Q));
    assert_eq!(run.hex(5, 1), Some(Q));

    // The night of day 4: base 3 pays 0.005 t for each man of unit 1, all fed;
    // base 2 pays for each man of unit 2.
    let (m1, m2) = (run.men(3, 1), run.men(3, 2));
    let paid3 = fx("0.005") * i64::from(m1);
    let paid2 = fx("0.005") * i64::from(m2);
    let spent3 = run.one(
        4,
        |k| matches!(k, EventKind::BaseSupplySpent { base: b, .. } if b.raw() == 3),
    );
    assert_eq!(
        spent3.kind,
        EventKind::BaseSupplySpent {
            base: base(3),
            reason: SupplyUse::Men,
            aircraft: 0,
            men: m1,
            asked_t: paid3,
            paid_t: paid3,
            left_t: n(25) - paid3,
        }
    );
    let spent2 = run.one(
        4,
        |k| matches!(k, EventKind::BaseSupplySpent { base: b, .. } if b.raw() == 2),
    );
    assert_eq!(
        spent2.kind,
        EventKind::BaseSupplySpent {
            base: base(2),
            reason: SupplyUse::Men,
            aircraft: 0,
            men: m2,
            asked_t: paid2,
            paid_t: paid2,
            left_t: run.supply(3, 2) - paid2,
        }
    );
    assert_eq!(run.at(4).land_units[&unit(1)].fed_share, Fixed::ONE);
    assert_eq!(run.at(4).land_units[&unit(2)].fed_share, Fixed::ONE);

    // Day 4, the fight: unit 1 fires its men times 50/100, cut, at 0.3; unit 2
    // fires all its men at 0.6.
    let (men1, shots1, chance1, a) = fire(&run, 4, 1);
    let (men2, shots2, chance2, j) = fire(&run, 4, 2);
    assert_eq!(men1, m1 * 50 / 100);
    assert_eq!(chance1, fx("0.3"));
    assert_eq!(shots1 * 100, n(i64::from(men1)));
    assert_eq!(men2, m2);
    assert_eq!(chance2, fx("0.6"));
    assert_eq!(shots2 * 100, n(i64::from(men2)));
    assert_eq!(run.men(4, 2), m2 - 20 * a);
    assert_eq!(run.men(4, 1), m1 - 20 * j);

    // Unit 1 fired on attack and did not march: 50 and 20 is 70, above 60, so
    // it goes back to defend.
    assert_eq!(fatigue_change(&run, 4, 1), LandFatigueChange::Gained);
    assert_eq!(run.fatigue(4, 1), n(70));
    let back = run.one(4, |k| matches!(k, EventKind::LandUnitBackToDefend { .. }));
    assert_eq!(
        back.kind,
        EventKind::LandUnitBackToDefend {
            unit: unit(1),
            fatigue: n(70),
        }
    );
    assert_eq!(run.at(4).land_units[&unit(1)].posture, Posture::Defend);

    // Beaten: the Japanese side is tested by the men left, and base 2 becomes
    // Allied if and only if it is beaten. The Allied side is not tested.
    let (left1, left2) = (run.men(4, 1), run.men(4, 2));
    let beaten = n(i64::from(left2)) < fx("0.2") * i64::from(left1) || left2 == 0;
    let events = run.on(4, |k| matches!(k, EventKind::SideBeatenAshore { .. }));
    assert_eq!(events.len(), usize::from(beaten));
    let expected = if beaten { Side::Allied } else { Side::Japanese };
    assert_eq!(run.at(4).bases[&base(2)].side, expected);
    assert_eq!(
        run.on(4, |k| matches!(k, EventKind::BaseChangedSide { .. }))
            .len(),
        usize::from(beaten)
    );

    // Day 5: no unit at Q is on attack: no fight, and unit 1 rests to 65.
    assert!(run
        .on(5, |k| matches!(k, EventKind::GroundFight { .. }))
        .is_empty());
    assert!(run
        .on(5, |k| matches!(k, EventKind::GroundFire { .. }))
        .is_empty());
    assert_eq!(fatigue_change(&run, 5, 1), LandFatigueChange::Rested);
    assert_eq!(run.fatigue(5, 1), n(65));
    assert_eq!(run.men(5, 1), left1);

    // A ground report for each side on each of days 4 and 5, if both still
    // have men at Q; the second is a replacement.
    for (day, men1, men2) in [(4, left1, left2), (5, run.men(5, 1), run.men(5, 2))] {
        let both = men1 > 0 && men2 > 0;
        let allied = reports(&run, day, Side::Allied);
        let japanese = reports(&run, day, Side::Japanese);
        assert_eq!(allied.len(), usize::from(both), "day {day}");
        assert_eq!(japanese.len(), usize::from(both), "day {day}");
        if both {
            assert!(allied[0].1 && japanese[0].1, "replacing the estimate held");
            assert!(in_report_bounds(allied[0].0, men2), "day {day}");
            assert!(in_report_bounds(japanese[0].0, men1), "day {day}");
        }
    }

    // At the end base 3 holds 25 t less 0.005 t for each man unit 1 had on the
    // nights of days 4 and 5; base 2 paid for unit 2's men on the same nights.
    let rate = fx("0.005");
    assert_eq!(
        run.supply(5, 3),
        n(25) - rate * i64::from(m1) - rate * i64::from(left1)
    );
    assert_eq!(run.supply(4, 3), n(25) - rate * i64::from(m1));
    assert_eq!(run.supply(5, 1), n(60));
}

#[test]
fn unit_3_marches_from_a_shore_with_no_base_and_carries_nothing() {
    let run = Run::text(&text());
    let order = accepted(&run, 1, 3);
    for (day, men, hex, fatigue, march) in [
        (1, 95, R, 10, Some(1)),
        (2, 91, R, 20, Some(2)),
        (3, 87, S, 30, None),
        (4, 83, S, 25, None),
        (5, 79, S, 20, None),
    ] {
        assert_eq!(run.men(day, 3), men, "day {day}");
        assert_eq!(run.hex(day, 3), Some(hex), "day {day}");
        assert_eq!(run.fatigue(day, 3), n(fatigue), "day {day}");
        assert_eq!(run.march_days(day, 3), march, "day {day}");
    }
    // Losses unfed, each night: 5, 4, 4, 4, 4 (4.75, 4.55, 4.35 and 4.15 cut).
    let lost: Vec<u32> = run
        .all(|k| matches!(k, EventKind::MenLostUnfed { unit: u, .. } if u.raw() == 3))
        .iter()
        .map(|e| match e.kind {
            EventKind::MenLostUnfed { lost, .. } => lost,
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(lost, vec![5, 4, 4, 4, 4]);
    assert_eq!(100 - lost.iter().sum::<u32>(), 79);
    // Each day marched cites the order; no day is marched after the arrival.
    let days = run.all(|k| matches!(k, EventKind::MarchDay { unit: u, .. } if u.raw() == 3));
    assert_eq!(days.len(), 3);
    for e in &days {
        assert_eq!(e.causes, vec![Cause::Order(order)]);
    }
    // The arrival carries nothing and forms no base; no supply was taken.
    let arrived = run.one(
        3,
        |k| matches!(k, EventKind::MarchArrived { unit: u, .. } if u.raw() == 3),
    );
    assert_eq!(
        arrived.kind,
        EventKind::MarchArrived {
            unit: unit(3),
            from: R,
            to: S,
            men: 87,
            supply_t: Fixed::ZERO,
            base: None,
        }
    );
    assert_eq!(arrived.causes, vec![Cause::Event(days[2].id)]);
    assert_eq!(run.at(5).bases.len(), 3, "bases 1, 2 and 3 only");
    assert!(run.at(5).bases.values().all(|b| b.hex != S && b.hex != R));
    assert_eq!(
        run.all(|k| matches!(k, EventKind::MarchBaseFormed { .. }))
            .len(),
        1
    );
    assert_eq!(
        run.all(|k| matches!(
            k,
            EventKind::BaseSupplySpent {
                reason: SupplyUse::March,
                ..
            }
        ))
        .len(),
        1
    );
    // It fought nobody.
    assert!(run
        .all(|k| matches!(k, EventKind::GroundFire { unit: u, .. } if u.raw() == 3))
        .is_empty());
    // 79 men at S, 21 lost unfed.
    assert_eq!(run.men(5, 3), 79);
    assert_eq!(100 - run.men(5, 3), 21);
}

#[test]
fn only_the_allied_side_sees_the_allied_march_and_nobody_sees_the_others() {
    let run = Run::text(&text());
    let kinds_seen = |side: Side| -> Vec<&'static str> {
        run.scenario
            .world
            .side_view(side)
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                SideEventKind::Whole(EventKind::MarchDay { unit, .. }) => {
                    Some(if unit.raw() == 1 { "day 1" } else { "day 3" })
                }
                SideEventKind::Whole(EventKind::MarchBaseFormed { .. }) => Some("formed"),
                SideEventKind::Whole(EventKind::MarchArrived { unit, .. }) => {
                    Some(if unit.raw() == 1 {
                        "arrived 1"
                    } else {
                        "arrived 3"
                    })
                }
                SideEventKind::Whole(EventKind::MarchStopped { .. }) => Some("stopped"),
                _ => None,
            })
            .collect()
    };
    assert_eq!(
        kinds_seen(Side::Allied),
        vec!["day 1", "day 1", "day 1", "formed", "arrived 1"]
    );
    assert_eq!(
        kinds_seen(Side::Japanese),
        vec!["day 3", "day 3", "day 3", "arrived 3"]
    );
}

// ------------------------------------------------------------------ Variants

/// Base 4 appended at P, Allied, beside base 1, with base 1 set to `first`.
fn two_bases_at_p(first: u32, second: u32) -> String {
    let source = common::replace_once(&text(), "supply_t = 100", &format!("supply_t = {first}"));
    with_base(&source, 4, "allied", P, second)
}

/// The variants that march, to run every property over.
fn marching_runs() -> Vec<(&'static str, Run)> {
    vec![
        ("the scenario", Run::text(&text())),
        ("stock short in base 1", Run::text(&two_bases_at_p(18, 100))),
        ("stock short in both", Run::text(&two_bases_at_p(18, 12))),
        (
            "a base at Q already",
            Run::text(&with_base(
                &with_base(&text(), 5, "allied", Q, 10),
                6,
                "allied",
                Q,
                7,
            )),
        ),
        ("men aboard", Run::text(&aboard_source())),
        (
            "a march stopped on day 2",
            Run::text(&with_march(&text(), 2, "allied", 1, (0, 0))),
        ),
    ]
}

/// Unit 1 with a fast transport at P that takes 200 of its men aboard on day
/// 1, marching all the while.
fn aboard_source() -> String {
    format!(
        "{}\n[[order]]\nday = 1\nside = \"allied\"\ntask_force = 1\nkind = \"load\"\nunit = 1\ncargo = \"none\"\n",
        with_transport(&text())
    )
}

#[test]
fn property_supply_is_conserved_by_a_march() {
    for (name, run) in marching_runs() {
        let arrivals = run.all(|k| matches!(k, EventKind::MarchArrived { .. }));
        assert!(!arrivals.is_empty(), "{name}");
        for arrived in arrivals {
            let EventKind::MarchArrived { supply_t, base, .. } = arrived.kind else {
                unreachable!()
            };
            let Cause::Event(day_event) = arrived.causes[0] else {
                panic!("{name}: the arrival cites the day's march first")
            };
            // What leaves the bases of the hex left: the spends the march
            // caused. What enters: the arrival's supply.
            let spends: Vec<&Event> = run
                .on(arrived.day, |k| {
                    matches!(
                        k,
                        EventKind::BaseSupplySpent {
                            reason: SupplyUse::March,
                            ..
                        }
                    )
                })
                .into_iter()
                .filter(|e| e.causes.contains(&Cause::Event(day_event)))
                .collect();
            let mut left = Fixed::ZERO;
            for spend in &spends {
                let EventKind::BaseSupplySpent {
                    base: b,
                    paid_t,
                    left_t,
                    ..
                } = spend.kind
                else {
                    unreachable!()
                };
                left += paid_t;
                assert_eq!(run.supply(arrived.day, b.raw()), left_t, "{name}");
                assert!(spend.id < arrived.id, "{name}");
                assert!(arrived.causes.contains(&Cause::Event(spend.id)), "{name}");
            }
            assert_eq!(left, supply_t, "{name}: what leaves is what enters");
            assert_eq!(base.is_some(), supply_t > Fixed::ZERO, "{name}");
            if let Some(b) = base {
                // Its stock before: after the night's charge, or what it held.
                let before = run
                    .events
                    .iter()
                    .filter(|e| e.day == arrived.day && e.id < arrived.id)
                    .filter_map(|e| match e.kind {
                        EventKind::BaseSupplySpent {
                            base: s, left_t, ..
                        } if s == b => Some(left_t),
                        _ => None,
                    })
                    .next_back()
                    .unwrap_or_else(|| {
                        run.at(arrived.day - 1)
                            .bases
                            .get(&b)
                            .map_or(Fixed::ZERO, |base| base.supply_t)
                    });
                assert_eq!(
                    run.supply(arrived.day, b.raw()),
                    before + supply_t,
                    "{name}: base {b:?}"
                );
            }
        }
    }
}

#[test]
fn stock_short_across_two_bases_is_taken_in_base_order() {
    // Base 1 holds 18 t, base 4 holds 100 t, both at P. Base 1 pays the three
    // nights (5 t each) and has 3 t left when unit 1 arrives; it gives its 3 t
    // and base 4 the other 22 t of the 25 t.
    let run = Run::text(&two_bases_at_p(18, 100));
    assert_eq!(run.supply(2, 1), n(8));
    assert_eq!(run.supply(2, 4), n(100));
    let spends = run.all(|k| {
        matches!(
            k,
            EventKind::BaseSupplySpent {
                reason: SupplyUse::March,
                ..
            }
        )
    });
    assert_eq!(spends.len(), 2);
    assert_eq!(
        spends[0].kind,
        EventKind::BaseSupplySpent {
            base: base(1),
            reason: SupplyUse::March,
            aircraft: 0,
            men: 1000,
            asked_t: n(25),
            paid_t: n(3),
            left_t: Fixed::ZERO,
        }
    );
    assert_eq!(
        spends[1].kind,
        EventKind::BaseSupplySpent {
            base: base(4),
            reason: SupplyUse::March,
            aircraft: 0,
            men: 1000,
            asked_t: n(22),
            paid_t: n(22),
            left_t: n(78),
        }
    );
    // The base formed is number 5 and holds the 25 t.
    let arrived = run.one(
        3,
        |k| matches!(k, EventKind::MarchArrived { unit: u, .. } if u.raw() == 1),
    );
    assert_eq!(
        arrived.kind,
        EventKind::MarchArrived {
            unit: unit(1),
            from: P,
            to: Q,
            men: 1000,
            supply_t: n(25),
            base: Some(base(5)),
        }
    );
    assert_eq!(run.supply(3, 5), n(25));
    // Both stocks short: base 4 holds 12 t, and gives all of it.
    let run = Run::text(&two_bases_at_p(18, 12));
    let arrived = run.one(
        3,
        |k| matches!(k, EventKind::MarchArrived { unit: u, .. } if u.raw() == 1),
    );
    assert_eq!(
        arrived.kind,
        EventKind::MarchArrived {
            unit: unit(1),
            from: P,
            to: Q,
            men: 1000,
            supply_t: n(15),
            base: Some(base(5)),
        }
    );
    assert_eq!(run.supply(3, 1), Fixed::ZERO);
    assert_eq!(run.supply(3, 4), Fixed::ZERO);
    assert_eq!(run.supply(3, 5), n(15));
}

#[test]
fn property_men_are_conserved_by_a_march() {
    for (name, run) in marching_runs() {
        // A march moves a unit's men and loses none: the men arriving are the
        // men ashore before the march.
        for arrived in run.all(|k| matches!(k, EventKind::MarchArrived { .. })) {
            let EventKind::MarchArrived {
                unit: u, to, men, ..
            } = arrived.kind
            else {
                unreachable!()
            };
            let ashore_before = run
                .events
                .iter()
                .filter(|e| e.day == arrived.day && e.id < arrived.id)
                .filter_map(|e| match e.kind {
                    EventKind::MenLostUnfed {
                        unit: lost, ashore, ..
                    } if lost == u => Some(ashore),
                    _ => None,
                })
                .next_back()
                .unwrap_or(run.men(arrived.day - 1, u.raw()));
            assert_eq!(men, ashore_before, "{name}");
            assert!(run.hex(arrived.day, u.raw()) == Some(to), "{name}");
        }
        // And over the whole game every unit's men, ashore and aboard, change
        // only by the men lost unfed or in a fight.
        for day in 1..=run.len() {
            for id in run.at(0).land_units.keys() {
                let total =
                    |d: u32| run.at(d).land_units[id].men_ashore + run.at(d).men_aboard(*id);
                let lost: u32 = run
                    .on(day, |k| {
                        matches!(k, EventKind::MenLostUnfed { unit: u, .. }
                            | EventKind::MenLostInFight { unit: u, .. } if u == id)
                    })
                    .iter()
                    .map(|e| match e.kind {
                        EventKind::MenLostUnfed { lost, .. }
                        | EventKind::MenLostInFight { lost, .. } => lost,
                        _ => unreachable!(),
                    })
                    .sum();
                assert_eq!(
                    total(day),
                    total(day - 1) - lost,
                    "{name}, day {day}, {id:?}"
                );
            }
        }
    }
}

#[test]
fn property_no_march_where_the_map_has_no_land_link() {
    // In every run, each day marched, and each arrival, is along a link.
    for (name, run) in marching_runs() {
        let map = run.scenario.world.map();
        for e in run.marches() {
            match e.kind {
                EventKind::MarchDay { from, to, .. } | EventKind::MarchArrived { from, to, .. } => {
                    assert!(map.has_land_link(from, to), "{name}: {from:?} to {to:?}")
                }
                _ => {}
            }
        }
    }
    // A map with no land links allows no march: every order is refused and
    // nobody moves, whatever the orders.
    let run = Run::text(&without_land_links(&text()));
    assert_eq!(
        refusals(&run, 1),
        vec![
            (1, RejectReason::NoLandLink(Q)),
            (3, RejectReason::NoLandLink(S))
        ]
    );
    assert_eq!(refusals(&run, 4), vec![(1, RejectReason::NoLandLink(R))]);
    assert!(run.marches().is_empty());
    for day in 0..=run.len() {
        assert_eq!(run.hex(day, 1), Some(P));
        assert_eq!(run.hex(day, 2), Some(Q));
        assert_eq!(run.hex(day, 3), Some(R));
        for raw in 1..=3 {
            assert_eq!(run.march_days(day, raw), None);
        }
    }
    assert_eq!(run.at(5).bases.len(), 2, "no base formed");
    // Even an order to the unit's own hex is not a march, on a map with none.
    let source = with_march(&without_land_links(&text()), 2, "allied", 1, (0, 0));
    let run = Run::text(&source);
    assert!(refusals(&run, 2).is_empty());
    assert!(run.marches().is_empty());
}

#[test]
fn the_enemys_view_is_the_same_in_two_worlds_that_differ_only_in_a_march_to_a_hex_without_enemy() {
    // Unit 3 marches from R to S where the Allied side has no man, or does not.
    let marching = Run::text(&text());
    let staying = Run::text(&without(&text(), ORDER_3));
    assert_eq!(marching.hex(5, 3), Some(S));
    assert_eq!(staying.hex(5, 3), Some(R));
    assert!(!staying
        .marches()
        .iter()
        .any(|e| matches!(e.kind, EventKind::MarchDay { unit: u, .. } if u.raw() == 3)));
    // The Allied beliefs, day by day, are equal.
    for day in 0..=5 {
        assert_eq!(
            marching.belief(day, Side::Allied),
            staying.belief(day, Side::Allied),
            "day {day}"
        );
    }
    // And its whole view at the end: own units, own bases, estimates, enemy
    // bases and every event it sees, by its own numbers (not the log's).
    let digest = |run: &Run| -> String {
        let view = run.scenario.world.side_view(Side::Allied);
        let mut out = String::new();
        for u in view.land_units() {
            out += &format!("{u:?}\n");
        }
        for b in view.bases() {
            out += &format!("{b:?}\n");
        }
        for e in view.ground_estimates() {
            out += &format!("{:?} {} {}\n", e.hex, e.men, e.day);
        }
        for b in view.enemy_bases() {
            out += &format!("{b:?}\n");
        }
        for e in view.events() {
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
        out
    };
    assert_eq!(digest(&marching), digest(&staying));
    assert!(
        digest(&marching).contains("MarchArrived"),
        "the Allied side sees its own march"
    );
    // The worlds did differ: the Japanese side sees unit 3 march in one.
    let japanese = |run: &Run| run.scenario.world.side_view(Side::Japanese).events().len();
    assert!(japanese(&marching) > japanese(&staying));
}
// ------------------------------------------------------------ The refusals

#[test]
fn an_order_for_a_unit_that_is_not_the_sides_is_refused_as_not_your_land_unit() {
    // The Japanese unit 2, ordered by the Allied side; and a unit that does
    // not exist.
    for raw in [2, 77] {
        let run = Run::text(&with_march(&text(), 1, "allied", raw, (0, 0)));
        assert_eq!(
            refusals(&run, 1),
            vec![(raw, RejectReason::NotYourLandUnit)]
        );
        // Nothing moved, and the orders of the scenario went on.
        assert_eq!(run.hex(5, 2), Some(Q));
        assert_eq!(run.march_days(1, 2), None);
        assert_eq!(run.hex(3, 1), Some(Q));
    }
    // And the other way: the Japanese side orders the Allied unit 1.
    let run = Run::text(&with_march(&text(), 2, "japanese", 1, (0, 0)));
    assert_eq!(refusals(&run, 2), vec![(1, RejectReason::NotYourLandUnit)]);
    assert_eq!(run.hex(3, 1), Some(Q), "unit 1 marched on");
}

#[test]
fn an_order_for_a_destroyed_unit_is_refused_as_destroyed() {
    // Unit 3 has one man and is unfed: it loses its last man on the night of
    // day 1, with its march order standing, and the march stops for want of a
    // man ashore.
    let source = common::replace_once(&text(), "men = 100\nq = 2", "men = 1\nq = 2");
    let run = Run::text(&with_march(&source, 2, "japanese", 3, (3, 0)));
    assert_eq!(run.men(1, 3), 0);
    assert_eq!(run.at(1).men_aboard(unit(3)), 0);
    assert_eq!(
        run.on(1, |k| matches!(k, EventKind::LandUnitDestroyed { .. }))
            .len(),
        1
    );
    let stopped = run.one(1, |k| matches!(k, EventKind::MarchStopped { .. }));
    assert_eq!(
        stopped.kind,
        EventKind::MarchStopped {
            unit: unit(3),
            to: S,
            days: 0,
            reason: MarchStop::NoMenAshore,
        }
    );
    assert_eq!(run.march_days(1, 3), None);
    assert_eq!(
        refusals(&run, 2),
        vec![(3, RejectReason::LandUnitDestroyed)]
    );
    assert!(run
        .all(|k| matches!(k, EventKind::MarchDay { unit: u, .. } if u.raw() == 3))
        .is_empty());
}

#[test]
fn an_order_for_a_unit_with_every_man_aboard_ships_is_refused_as_no_men_ashore() {
    // Unit 4, 100 men, is aboard the transport at P and none are ashore.
    let source = format!(
        "{}\n[[land_unit]]\nid = 4\nname = \"Placeholder company aboard\"\nside = \"allied\"\nmen = 100\ntask_force = 1\n",
        with_transport(&text())
    );
    let source = with_march(&source, 1, "allied", 4, (1, 0));
    let run = Run::text(&source);
    assert_eq!(run.at(0).men_aboard(unit(4)), 100);
    assert_eq!(refusals(&run, 1), vec![(4, RejectReason::NoMenAshore)]);
    assert_eq!(run.march_days(1, 4), None);
    assert_eq!(run.at(5).men_aboard(unit(4)), 100);
    // The reason holds whatever hex is named, linked or not.
    let source = with_march(&source, 2, "allied", 4, (7, 7));
    assert_eq!(
        refusals(&Run::text(&source), 2),
        vec![(4, RejectReason::NoMenAshore)]
    );
}

#[test]
fn an_order_to_a_hex_no_land_link_joins_is_refused_as_no_land_link() {
    // On day 4 unit 1 at Q is ordered to R, a neighbour with no link: the
    // scenario's own order, refused.
    let run = Run::text(&text());
    assert_eq!(refusals(&run, 4), vec![(1, RejectReason::NoLandLink(R))]);
    // On day 2 unit 1, marching, is ordered to a hex that is not a neighbour
    // (R), a neighbour with no link ((0,1), open sea) and a hex off the map:
    // each is refused with its own hex, and the march goes on.
    let source = with_march(&text(), 2, "allied", 1, (2, 0));
    let source = with_march(&source, 2, "allied", 1, (0, 1));
    let source = with_march(&source, 2, "allied", 1, (7, 7));
    let run = Run::text(&source);
    assert_eq!(
        refusals(&run, 2),
        vec![
            (1, RejectReason::NoLandLink(R)),
            (1, RejectReason::NoLandLink(Hex::new(0, 1))),
            (1, RejectReason::NoLandLink(Hex::new(7, 7))),
        ]
    );
    assert_eq!(run.march_days(2, 1), Some(2));
    assert_eq!(run.hex(3, 1), Some(Q));
    assert_eq!(run.supply(3, 3), n(25));
}

// ------------------------------------------------- A march stopped by an order

#[test]
fn an_order_to_the_units_own_hex_stops_the_march() {
    let run = Run::text(&with_march(&text(), 2, "allied", 1, (0, 0)));
    let order = {
        let e = run.one(2, |k| {
            matches!(k, EventKind::OrderAccepted { target: EntityRef::LandUnit(u), .. } if u.raw() == 1)
        });
        match e.kind {
            EventKind::OrderAccepted { order, .. } => order,
            _ => unreachable!(),
        }
    };
    // It had marched one day. The stop is recorded, and cites the new order.
    let stopped = run.one(2, |k| matches!(k, EventKind::MarchStopped { .. }));
    assert_eq!(
        stopped.kind,
        EventKind::MarchStopped {
            unit: unit(1),
            to: Q,
            days: 1,
            reason: MarchStop::Ordered,
        }
    );
    assert_eq!(stopped.causes, vec![Cause::Order(order)]);
    // No day marched after, no arrival, no base, no supply moved.
    assert!(run
        .all(|k| matches!(k, EventKind::MarchArrived { unit: u, .. } if u.raw() == 1))
        .is_empty());
    assert_eq!(
        run.all(|k| matches!(k, EventKind::MarchDay { unit: u, .. } if u.raw() == 1))
            .len(),
        1
    );
    assert!(run
        .all(|k| matches!(k, EventKind::MarchBaseFormed { .. }))
        .is_empty());
    assert!(run
        .all(|k| matches!(
            k,
            EventKind::BaseSupplySpent {
                reason: SupplyUse::March,
                ..
            }
        ))
        .is_empty());
    assert_eq!(run.at(5).bases.len(), 2);
    // The unit stays at P and rests from then on: 10, 5, 0, 0, 0. Base 1 pays
    // 5 t on each of the five nights.
    for (day, fatigue, supply) in [(1, 10, 95), (2, 5, 90), (3, 0, 85), (4, 0, 80), (5, 0, 75)] {
        assert_eq!(run.hex(day, 1), Some(P), "day {day}");
        assert_eq!(run.fatigue(day, 1), n(fatigue), "day {day}");
        assert_eq!(run.supply(day, 1), n(supply), "day {day}");
        // One day marched stands in the unit until the order stops it on day 2.
        assert_eq!(run.march_days(day, 1), (day == 1).then_some(1), "day {day}");
        assert_eq!(run.men(day, 1), 1000);
    }
    assert_eq!(fatigue_change(&run, 2, 1), LandFatigueChange::Rested);
    // No fight at all: the unit never reached Q.
    assert!(run
        .all(|k| matches!(k, EventKind::GroundFight { .. }))
        .is_empty());
    // An order to the own hex of a unit that is not marching changes nothing
    // and records no stop.
    let run = Run::text(&with_march(
        &without_orders(&text()),
        2,
        "allied",
        1,
        (0, 0),
    ));
    assert!(run.marches().is_empty());
    assert_eq!(run.march_days(2, 1), None);
}

// --------------------------------------------------- Men aboard and ashore

#[test]
fn a_unit_with_men_aboard_ships_marches_only_its_men_ashore() {
    // 200 of unit 1's 1000 men go aboard the transport at P on the first
    // night. Its march goes on with the 800 ashore.
    let run = Run::text(&aboard_source());
    for day in 1..=5 {
        assert_eq!(run.at(day).men_aboard(unit(1)), 200, "day {day}");
    }
    let ship = &run.at(5).ships[&sim_core::ids::ShipId::new(1)];
    assert_eq!(ship.troops.map(|t| (t.unit.raw(), t.men)), Some((1, 200)));
    assert_eq!(run.men(1, 1), 800);
    assert_eq!(run.men(2, 1), 800);
    // The arrival brings the 800, and the supply carried is for the 800: 5
    // days at 0.005 t is 20 t, from base 1, into base 3.
    let arrived = run.one(
        3,
        |k| matches!(k, EventKind::MarchArrived { unit: u, .. } if u.raw() == 1),
    );
    assert_eq!(
        arrived.kind,
        EventKind::MarchArrived {
            unit: unit(1),
            from: P,
            to: Q,
            men: 800,
            supply_t: n(20),
            base: Some(base(3)),
        }
    );
    assert_eq!(run.supply(3, 3), n(20));
    // Base 1: 100 less 5 (night 1, 1000 men), 4 and 4 (800 men), less the 20 t.
    assert_eq!(run.supply(3, 1), n(67));
    // Only the men ashore are at Q; the men aboard are aboard still, and the
    // ship is where it was. No new unit was made.
    assert_eq!(run.hex(3, 1), Some(Q));
    assert_eq!(run.at(5).land_units.len(), 3);
    assert!(run
        .all(|k| matches!(k, EventKind::UnitDivided { .. }))
        .is_empty());
    assert!(run
        .all(|k| matches!(k, EventKind::MenLanded { .. }))
        .is_empty());
    // It fires on arrival with the men ashore only: 800 times 80/100.
    let (men, shots, _, a) = fire(&run, 3, 1);
    assert_eq!(men, 640);
    assert_eq!(shots * 100, n(640));
    assert!(a <= 7, "6.4 shots are at most 7 hits");
    let (_, _, _, j) = fire(&run, 3, 2);
    assert_eq!(run.men(3, 1), 800 - 20 * j);
    assert_eq!(run.at(3).men_aboard(unit(1)), 200);
}

// ------------------------------------------------ Arriving where a base is

#[test]
fn a_unit_arriving_where_its_side_has_a_base_adds_the_supply_to_the_lowest_numbered() {
    // Allied bases 5 and 6 are already at Q, with 10 t and 7 t. The 25 t go
    // to base 5, and no base is formed.
    let source = with_base(&with_base(&text(), 5, "allied", Q, 10), 6, "allied", Q, 7);
    let run = Run::text(&source);
    let arrived = run.one(
        3,
        |k| matches!(k, EventKind::MarchArrived { unit: u, .. } if u.raw() == 1),
    );
    assert_eq!(
        arrived.kind,
        EventKind::MarchArrived {
            unit: unit(1),
            from: P,
            to: Q,
            men: 1000,
            supply_t: n(25),
            base: Some(base(5)),
        }
    );
    assert!(run
        .all(|k| matches!(k, EventKind::MarchBaseFormed { .. }))
        .is_empty());
    assert_eq!(run.at(5).bases.len(), 4, "bases 1, 2, 5 and 6");
    assert_eq!(run.supply(2, 5), n(10));
    assert_eq!(run.supply(3, 5), n(35));
    assert_eq!(run.supply(3, 6), n(7));
    assert_eq!(run.supply(3, 1), n(60));
    // The arrival cites the march and the supply taken, and no formation.
    let day_event = run.one(
        3,
        |k| matches!(k, EventKind::MarchDay { unit: u, .. } if u.raw() == 1),
    );
    let taken = run.one(3, |k| {
        matches!(
            k,
            EventKind::BaseSupplySpent {
                reason: SupplyUse::March,
                ..
            }
        )
    });
    assert_eq!(
        arrived.causes,
        vec![Cause::Event(day_event.id), Cause::Event(taken.id)]
    );
    // The base was on the map from the start: nothing to find, so no
    // BaseFound for it; the day 3 report finds nothing.
    assert!(run
        .all(|k| matches!(k, EventKind::BaseFound { .. }))
        .is_empty());
    assert!(run.belief(0, Side::Japanese).enemy_bases.contains(&base(5)));
}

// ----------------------------------------------- No march order, no change

/// Runs a scenario file and returns its log; asserts no unit ever has a march
/// and no event of decision 0094 is recorded.
fn assert_no_march_in(path: PathBuf) {
    let mut scenario = common::load(&path).expect("the scenario loads");
    let mut days = 0;
    while scenario.next_day().expect("a day resolves").is_some() {
        days += 1;
        for unit in scenario.world.umpire().land_units.values() {
            assert_eq!(unit.march, None, "{path:?}, day {days}");
        }
    }
    assert!(days > 0);
    for event in scenario.world.events().iter() {
        assert!(!is_march_event(&event.kind), "{path:?}: {event}");
        assert!(
            !matches!(
                event.kind,
                EventKind::BaseSupplySpent {
                    reason: SupplyUse::March,
                    ..
                } | EventKind::LandUnitFatigue {
                    change: LandFatigueChange::Marched | LandFatigueChange::MarchedAndAttacked,
                    ..
                }
            ),
            "{path:?}: {event}"
        );
    }
}

#[test]
fn property_a_scenario_with_no_march_order_is_unchanged() {
    // The scenarios that have land units and no march order never march: no
    // unit has a march, and no event of the march is recorded. (Their events
    // are compared byte for byte by the run baselines, and the hash version is
    // the one thing that moves their final hashes.)
    for name in ["21_landing", "23_fight_ashore", "24_shells_and_estimates"] {
        assert_no_march_in(common::data_dir().join(format!("scenarios/{name}.toml")));
    }
    // Scenario 25 with its three orders taken out: nobody moves, no base is
    // formed, and each night base 1 pays 5 t.
    let run = Run::text(&without_orders(&text()));
    assert!(run.marches().is_empty());
    for day in 0..=5 {
        assert_eq!(run.hex(day, 1), Some(P));
        assert_eq!(run.hex(day, 2), Some(Q));
        assert_eq!(run.hex(day, 3), Some(R));
        for raw in 1..=3 {
            assert_eq!(run.march_days(day, raw), None);
        }
    }
    assert_eq!(run.supply(5, 1), n(75));
    assert_eq!(run.at(5).bases.len(), 2);
    assert!(run
        .all(|k| matches!(k, EventKind::GroundFight { .. }))
        .is_empty());
    assert!(refusals(&run, 4).is_empty());
}
