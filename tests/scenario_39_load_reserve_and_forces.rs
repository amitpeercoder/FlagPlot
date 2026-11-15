//! Scenario 39: a load up to a weight, a reserve at a base, and forces
//! formed, joined and stood down. A refuelling stops at the fuel reserve
//! with a second force refuelling the same morning; a load ordered by the
//! staff stops at the supply reserve and one ordered by the commander does
//! not; a weight is shared among a force's ships in ship order.
//!
//! Proves decision 0111 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/39_load_reserve_and_forces.toml`. It runs on
//! the frozen step 1 figures. There is no enemy and no random draw.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! A size 1 port has one berth, at 50 t an hour, and lighters at 20 t an
//! hour; a size 0 port works by lighter at 20 t an hour. Berths are given
//! out at the start of each pulse, to forces in number order, to a ship
//! that has cargo to move. The placeholder cargo ship: 3000 t of cargo,
//! 1000 t of fuel. The placeholder fast transport with cargo: 30 t of
//! cargo. A day is a night pulse of 12 hours and two daylight pulses of 6.
//! A ship of a force stopped at a base of its side takes the fuel it lacks
//! in the pulse, in force order, after its cargo (decision 0112; restated by
//! the architect on Nov 15, 2026: it was at the start of a day, before the
//! day's orders. On these figures no fuelling rate is given, so a ship is
//! filled in the first pulse. No quantity moved).
//!
//! ## Inputs
//!
//! A placeholder map: S at (0,0), a coast hex with port size 1, and T at
//! (10,0), a coast hex with port size 0. The game is 4 days. Everything is
//! Allied.
//!
//! | Base | Hex | Supply | Fuel  | Supply reserve | Fuel reserve |
//! | ---- | --- | ------ | ----- | -------------- | ------------ |
//! | 1    | S   | 1000 t | 500 t | 400 t          | 300 t        |
//! | 2    | T   | 100 t  | 0     | 0              | 0            |
//!
//! | Force | Ships                                               | Where          |
//! | ----- | --------------------------------------------------- | -------------- |
//! | 1     | 1, cargo ship, 850 t of fuel                        | stopped at base 1 |
//! | 2     | 2, cargo ship, 900 t of fuel                        | stopped at base 1 |
//! | 3     | 3 and 4, fast transports with cargo, full of fuel   | stopped at base 2 |
//!
//! Ships 5 and 6, cargo ships full of fuel, lie in port at base 2 in no
//! force. No ship has cargo aboard.
//!
//! Orders:
//!
//! - Day 1: force 1 loads supply up to 250 t, by the commander's order.
//!   Force 2 loads supply up to 500 t, by an order whose origin is the
//!   staff. Force 3 loads supply up to 50 t. Ships 5 and 6 are formed into
//!   a force.
//! - Day 2: force 1 loads supply up to 450 t, by the commander's order.
//!   Force 3 is joined to the force formed on day 1.
//! - Day 3: that force loads supply up to 60 t.
//! - Day 4: that force is stood down.
//!
//! ## At S: a reserve
//!
//! Day 1, the refuelling, in the night pulse. Base 1 holds 500 t of fuel and may give 200 t,
//! down to its reserve of 300.
//!
//! - Force 1: ship 1 wants 150 t and takes it. Base 1 holds 350 t.
//! - Force 2: ship 2 wants 100 t. Base 1 may give 50 t: ship 2 takes 50 and
//!   has 950 t. Base 1 holds 300 t, its reserve.
//!
//! Day 1, the loads. Base 1 holds 1000 t of supply; its reserve is 400.
//!
//! | Pulse     | Force 1, to 250 t, the commander's       | Force 2, to 500 t, the staff's              | Base 1 after |
//! | --------- | ---------------------------------------- | ------------------------------------------- | ------------ |
//! | Night     | Has the berth. 250 t in 5 hours: complete | By lighter: 20 x 12 = 240 t                | 510 t        |
//! | Morning   | Nothing to move                          | Has the berth. It may take 510 - 400 = 110 t: loaded in 2.2 hours. 350 t aboard. Complete: the stock it may draw on is gone | 400 t |
//! | Afternoon |                                          |                                             | 400 t        |
//!
//! In the night force 1 is worked first and leaves 750 t; force 2 may then
//! take up to 350 t and takes its 240. Force 2's load is complete with 350
//! t aboard, not 500: the staff's load never takes the stock below 400.
//!
//! Day 2, the refuelling. Base 1 is at its fuel reserve. Ship 2 wants 50 t
//! and takes none. It was recorded as held at the reserve on day 1, 50 t
//! short, and is not recorded again while nothing changes.
//!
//! Day 2, force 1's second order, the commander's. Force 1 has 250 t aboard
//! and is to have 450: it wants 200 t more. Base 1 holds nothing above its
//! reserve. The order is accepted, and its answer says it may take 200 t
//! below the reserve. In the night pulse it has the berth and loads 200 t
//! in 4 hours. Base 1 holds 200 t.
//!
//! At the end: base 1 holds 200 t of supply and 300 t of fuel. Ship 1 has
//! 450 t of supply and 1000 t of fuel. Ship 2 has 350 t of supply and 950 t
//! of fuel.
//!
//! ## At T: a weight shared, and forces formed, joined and stood down
//!
//! Day 1. Force 3 loads up to 50 t, by lighter, both ships at once. Ship 3
//! is filled first: 30 t, its room, in 1.5 hours. Ship 4 takes the other 20
//! t in 1 hour. Base 2 holds 50 t. The load is complete.
//!
//! Ships 5 and 6 are formed into a force. It takes the first number of the
//! side's made series (decision 0086), and the side sees it as its next
//! force: stopped at base 2, with no mission, at cruise speed, with base 2
//! as its home port.
//!
//! Day 2. Force 3 is joined to the formed force. The formed force has
//! ships 3, 4, 5 and 6, with the 30 t and 20 t aboard ships 3 and 4. Force
//! 3 has ceased to be, and an order to it is refused.
//!
//! Day 3. The formed force loads up to 60 t. It has 50 t aboard and wants
//! 10 more. Its ships are taken in ship order: ship 3 is full; ship 4 has
//! room for 10 and takes them, in half an hour. Ships 5 and 6 take nothing.
//! Base 2 holds 40 t.
//!
//! Day 4. The formed force is stood down. Ships 3, 4, 5 and 6 lie in port
//! at base 2 in no force, with 30, 30, 0 and 0 t aboard. The formed force
//! has ceased to be.
//!
//! # Properties
//!
//! - With no weight, no reserve and none of the three orders, every
//!   scenario gives what it gave before, apart from the hash version.
//! - A load with a weight never puts more aboard a force than the weight,
//!   and never more in a ship than its room.
//! - Refuelling never takes a base's fuel below its reserve, and a load
//!   whose origin is the staff never takes its supply below its reserve,
//!   on any day of a run in which the men's supply is also drawn from the
//!   base.
//! - Where a side has two bases in a hex, each ship of a force that
//!   refuels is served by the lowest-numbered base that holds fuel above
//!   its reserve, down to that reserve; a ship that base leaves short does
//!   not go on to the next (decision 0045's one base for a ship).
//! - Forming, joining and standing down are each refused at sea, at a base
//!   of the other side, for a force that is under way, and for a force
//!   away at the yard; a join is refused between forces at different bases.
//! - The enemy's view is the same in two worlds that differ only in which
//!   of the other side's ships are in which force at a base.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, OrderId, ShipId, TaskForceId};
use sim_core::orders::{
    BaseOrder, Cargo, Mission, OrderKind, RejectReason, SpeedSetting, TaskForceOrder,
};
use sim_core::state::{
    Base, RulesOfEngagement, Ship, ShipLocation, Side, TaskForce, TaskForceStatus, TruthState,
};
use sim_core::time::{pulse_start, Pulse};
use sim_data::Scenario;

// ------------------------------------------------------------------ Helpers

const S: Hex = Hex::new(0, 0);
const T: Hex = Hex::new(10, 0);

/// The number the engine gives the force formed on day 1: the first of the
/// Allied made series (decision 0086).
const FORMED: u32 = 1_000_001;

fn path() -> PathBuf {
    common::data_dir().join("scenarios/39_load_reserve_and_forces.toml")
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

fn tf(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn ship_id(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn base_id(raw: u32) -> BaseId {
    BaseId::new(raw)
}

/// The scenario's text down to its orders: the bases, forces and ships.
fn head() -> String {
    let text = text();
    let marker = "# Day 1: three loads";
    assert_eq!(text.matches(marker).count(), 1);
    text.split(marker).next().expect("a head").to_string()
}

/// The text of one allied order for `day`, to `target` (a line such as
/// `task_force = 1`), with the lines of `body`.
fn order(day: u32, target: &str, body: &str) -> String {
    format!("[[order]]\nday = {day}\nside = \"allied\"\n{target}\n{body}\n\n")
}

/// The scenario's text with every order block for which `drop` says so left
/// out. The blocks are cut at `[[order]]`.
fn without_orders(text: &str, drop: impl Fn(&str) -> bool) -> String {
    let mut parts = text.split("[[order]]");
    let mut out = parts.next().expect("a head").to_string();
    for block in parts {
        if !drop(block) {
            out += "[[order]]";
            out += block;
        }
    }
    out
}

/// The orders that form, join or stand down, or go to the formed force.
fn is_new_order(block: &str) -> bool {
    block.contains("form_task_force")
        || block.contains("kind = \"join\"")
        || block.contains("kind = \"stand_down\"")
        || block.contains("task_force = 1000001")
}

/// What a side holds at this moment: every event it may see, as its view
/// gives them (by its own numbers: the umpire's event number is left out),
/// and its contacts.
fn side_digest(scenario: &Scenario, side: Side) -> String {
    let view = scenario.world.side_view(side);
    let mut out = String::new();
    for event in view.events() {
        out += &format!(
            "{} {} {:?} {:?} {:?} {:?} {}\n",
            event.number,
            event.day,
            event.pulse,
            event.kind,
            event.order_number,
            event.causes,
            event.hidden_causes
        );
    }
    for contact in view.contacts() {
        out += &format!("{contact:?}\n");
    }
    out
}

/// A run of a scenario: the truth at the start (day 0) and at the end of
/// each day, the Japanese side's view at the end of each day, the log and
/// the orders.
struct Run {
    days: Vec<TruthState>,
    japanese: Vec<String>,
    events: Vec<Event>,
    orders: BTreeMap<OrderId, (u32, OrderKind)>,
}

impl Run {
    fn of(mut scenario: Scenario) -> Run {
        let mut days = vec![scenario.world.umpire().clone()];
        let mut japanese = vec![side_digest(&scenario, Side::Japanese)];
        while scenario.next_day().expect("a day resolves").is_some() {
            days.push(scenario.world.umpire().clone());
            japanese.push(side_digest(&scenario, Side::Japanese));
        }
        Run {
            events: scenario.world.events().iter().cloned().collect(),
            orders: scenario
                .world
                .orders()
                .map(|order| (order.id, (order.day_issued, order.kind.clone())))
                .collect(),
            days,
            japanese,
        }
    }

    fn text(source: &str) -> Run {
        Run::of(from(source))
    }

    fn at(&self, day: u32) -> &TruthState {
        &self.days[day as usize]
    }

    fn ship(&self, day: u32, raw: u32) -> &Ship {
        &self.at(day).ships[&ship_id(raw)]
    }

    fn force(&self, day: u32, raw: u32) -> &TaskForce {
        &self.at(day).task_forces[&tf(raw)]
    }

    fn base(&self, day: u32, raw: u32) -> &Base {
        &self.at(day).bases[&base_id(raw)]
    }

    fn ships_of(&self, day: u32, raw: u32) -> Vec<u32> {
        self.force(day, raw).ships.iter().map(|s| s.raw()).collect()
    }

    fn all(&self, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events.iter().filter(|e| pick(&e.kind)).collect()
    }

    /// The events of `day` (any pulse) that satisfy `pick`, in log order.
    fn on_day(&self, day: u32, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.day == day && pick(&e.kind))
            .collect()
    }

    /// The one such event of `day` and `pulse`; panics unless exactly one.
    fn one(&self, day: u32, pulse: Pulse, pick: impl Fn(&EventKind) -> bool) -> &Event {
        let found: Vec<&Event> = self
            .events
            .iter()
            .filter(|e| e.day == day && e.pulse == Some(pulse) && pick(&e.kind))
            .collect();
        assert_eq!(
            found.len(),
            1,
            "day {day} {pulse:?}: one such event: {found:?}"
        );
        found[0]
    }

    /// The one event of `day` (any pulse, usually none) that satisfies `pick`.
    fn one_on_day(&self, day: u32, pick: impl Fn(&EventKind) -> bool) -> &Event {
        let found = self.on_day(day, pick);
        assert_eq!(found.len(), 1, "day {day}: one such event: {found:?}");
        found[0]
    }

    /// A ship's `CargoLoaded` of the pulse: (loaded, aboard, base stock).
    fn loaded(&self, day: u32, pulse: Pulse, ship: u32) -> (Fixed, Fixed, Fixed) {
        match self
            .one(
                day,
                pulse,
                |k| matches!(k, EventKind::CargoLoaded { ship: s, .. } if s.raw() == ship),
            )
            .kind
        {
            EventKind::CargoLoaded {
                loaded_t,
                aboard_t,
                base_stock_t,
                ..
            } => (loaded_t, aboard_t, base_stock_t),
            _ => unreachable!(),
        }
    }

    /// The ships that loaded in the pulse, in log order.
    fn loaders(&self, day: u32, pulse: Pulse) -> Vec<u32> {
        self.events
            .iter()
            .filter(|e| e.day == day && e.pulse == Some(pulse))
            .filter_map(|e| match e.kind {
                EventKind::CargoLoaded { ship, .. } => Some(ship.raw()),
                _ => None,
            })
            .collect()
    }

    /// The elapsed hours of a force's `LoadingComplete` in the pulse.
    fn complete(&self, day: u32, pulse: Pulse, force: u32) -> Fixed {
        match self
            .one(day, pulse, |k| {
                matches!(k, EventKind::LoadingComplete { task_force, .. } if task_force.raw() == force)
            })
            .kind
        {
            EventKind::LoadingComplete { elapsed_hours, .. } => elapsed_hours,
            _ => unreachable!(),
        }
    }

    /// The `ShipRefuelled` events of a day: (ship, base, taken, fuel aboard,
    /// base fuel).
    fn refuelled(&self, day: u32) -> Vec<(u32, u32, Fixed, Fixed, Fixed)> {
        self.on_day(day, |k| matches!(k, EventKind::ShipRefuelled { .. }))
            .iter()
            .map(|e| match e.kind {
                EventKind::ShipRefuelled {
                    ship,
                    base,
                    taken_t,
                    fuel_t,
                    base_fuel_t,
                    ..
                } => (ship.raw(), base.raw(), taken_t, fuel_t, base_fuel_t),
                _ => unreachable!(),
            })
            .collect()
    }

    /// The `RefuellingHeldAtReserve` events of a day: (ship, short).
    fn held_refuelling(&self, day: u32) -> Vec<(u32, Fixed)> {
        self.on_day(day, |k| {
            matches!(k, EventKind::RefuellingHeldAtReserve { .. })
        })
        .iter()
        .map(|e| match e.kind {
            EventKind::RefuellingHeldAtReserve { ship, short_t, .. } => (ship.raw(), short_t),
            _ => unreachable!(),
        })
        .collect()
    }

    /// Why the one order of `day` that `pick` names was refused.
    fn refusal(&self, day: u32, pick: impl Fn(&OrderKind) -> bool) -> RejectReason {
        let ids: Vec<OrderId> = self
            .orders
            .iter()
            .filter(|(_, (issued, kind))| *issued == day && pick(kind))
            .map(|(id, _)| *id)
            .collect();
        assert_eq!(ids.len(), 1, "day {day}: one such order");
        let found: Vec<&RejectReason> = self
            .events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::OrderRejected { order, reason, .. } if *order == ids[0] => Some(reason),
                _ => None,
            })
            .collect();
        assert_eq!(
            found.len(),
            1,
            "day {day}: the order was refused: {:?}",
            ids[0]
        );
        found[0].clone()
    }

    /// Whether the one order of `day` that `pick` names was accepted.
    fn accepted(&self, day: u32, pick: impl Fn(&OrderKind) -> bool) -> bool {
        let ids: Vec<OrderId> = self
            .orders
            .iter()
            .filter(|(_, (issued, kind))| *issued == day && pick(kind))
            .map(|(id, _)| *id)
            .collect();
        assert_eq!(ids.len(), 1, "day {day}: one such order");
        self.events
            .iter()
            .any(|e| matches!(e.kind, EventKind::OrderAccepted { order, .. } if order == ids[0]))
    }
}

/// An order to task force `raw` that is `pick`.
fn to_force(kind: &OrderKind, raw: u32, pick: impl Fn(&TaskForceOrder) -> bool) -> bool {
    matches!(kind, OrderKind::TaskForce { target, order } if target.raw() == raw && pick(order))
}

fn is_join(order: &TaskForceOrder) -> bool {
    matches!(order, TaskForceOrder::JoinTo(_))
}

fn is_stand_down(order: &TaskForceOrder) -> bool {
    matches!(order, TaskForceOrder::StandDown)
}

fn is_load(order: &TaskForceOrder) -> bool {
    matches!(
        order,
        TaskForceOrder::Load | TaskForceOrder::LoadTo { .. } | TaskForceOrder::LoadFuel
    )
}

fn forms(kind: &OrderKind, base: u32, ships: &[u32]) -> bool {
    matches!(kind, OrderKind::Base { target, order: BaseOrder::FormTaskForce { ships: listed } }
        if target.raw() == base
            && listed.iter().map(|s| s.raw()).collect::<Vec<u32>>() == ships)
}

/// A log as comparable text, with causes.
fn digest(run: &Run) -> Vec<String> {
    run.events
        .iter()
        .map(|e| {
            format!(
                "{:?} {} {:?} {:?} {:?}",
                e.id, e.day, e.pulse, e.kind, e.causes
            )
        })
        .collect()
}

/// The truth at the end of every day as comparable text.
fn truths(run: &Run) -> Vec<String> {
    run.days.iter().map(|d| format!("{d:?}")).collect()
}

// ------------------------------------------------------- The hand calculation

#[test]
fn refuelling_stops_at_the_fuel_reserve_and_a_second_force_takes_what_is_left() {
    let run = Run::text(&text());
    // Day 1: base 1 holds 500 t and may give 200 t, down to its reserve of
    // 300. Ship 1 wants 150 and takes it; ship 2 wants 100 and takes 50.
    assert_eq!(
        run.refuelled(1),
        vec![
            (1, 1, n(150), n(1000), n(350)),
            (2, 1, n(50), n(950), n(300)),
        ]
    );
    assert_eq!(run.held_refuelling(1), vec![(2, n(50))]);
    // Day 2: base 1 is at its fuel reserve. Ship 2 wants 50 and takes none:
    // nothing is taken, and the event says it is 50 t short.
    assert!(run.refuelled(2).is_empty());
    assert_eq!(run.held_refuelling(2), vec![(2, n(50))]);
    // Nothing is taken on any later day either.
    assert!(run.refuelled(3).is_empty() && run.refuelled(4).is_empty());
    // The held event cites the ship's refuelling on day 1 and no cause on
    // day 2 (nothing was taken).
    let held_1 = run.one_on_day(1, |k| {
        matches!(k, EventKind::RefuellingHeldAtReserve { .. })
    });
    match &held_1.kind {
        EventKind::RefuellingHeldAtReserve {
            ship,
            task_force,
            hex,
            short_t,
        } => {
            assert_eq!(
                (ship.raw(), task_force.raw(), *hex, *short_t),
                (2, 2, S, n(50))
            );
        }
        _ => unreachable!(),
    }
    // The truth: base 1's fuel is 300 from day 1 to the end; ship 2 has 950.
    for day in 1..=4 {
        assert_eq!(run.base(day, 1).fuel_t, n(300));
        assert_eq!(run.ship(day, 1).fuel_t, n(1000));
        assert_eq!(run.ship(day, 2).fuel_t, n(950));
    }
}

#[test]
fn the_loads_at_s_follow_the_calculation_pulse_by_pulse() {
    let run = Run::text(&text());
    let night = pulse_start(1, Pulse::Night);
    let morning = pulse_start(1, Pulse::Morning);

    // Night: force 1 has the berth: 250 t in 5 hours, complete. Force 2 is
    // by lighter: 20 x 12 = 240 t. Base 1 after: 510 t.
    assert_eq!(run.loaded(1, Pulse::Night, 1), (n(250), n(250), n(750)));
    assert_eq!(run.complete(1, Pulse::Night, 1), night + n(5));
    assert_eq!(run.loaded(1, Pulse::Night, 2), (n(240), n(240), n(510)));
    assert_eq!(run.loaders(1, Pulse::Night), vec![1, 2, 3, 4]);
    assert!(run
        .on_day(
            1,
            |k| matches!(k, EventKind::LoadingComplete { task_force, .. } if task_force.raw() == 2)
        )
        .iter()
        .all(|e| e.pulse != Some(Pulse::Night)));

    // Morning: force 1 has nothing to move. Force 2 has the berth and may
    // take 510 - 400 = 110 t: loaded in 110 / 50 = 2.2 hours, 350 t aboard,
    // base 1 at 400 t. It is complete, and the event says why.
    assert_eq!(run.loaders(1, Pulse::Morning), vec![2]);
    assert_eq!(run.loaded(1, Pulse::Morning, 2), (n(110), n(350), n(400)));
    assert_eq!(
        run.complete(1, Pulse::Morning, 2),
        morning + Fixed::from_ratio(11, 5)
    );
    let held = run.one(1, Pulse::Morning, |k| {
        matches!(k, EventKind::LoadHeldAtReserve { .. })
    });
    match held.kind {
        EventKind::LoadHeldAtReserve {
            task_force,
            base,
            cargo,
            aboard_t,
            reserve_t,
        } => assert_eq!(
            (task_force.raw(), base.raw(), cargo, aboard_t, reserve_t),
            (2, 1, Cargo::Supply, n(350), n(400))
        ),
        _ => unreachable!(),
    }

    // Afternoon: nothing moves; base 1 holds 400 t.
    assert!(run.loaders(1, Pulse::Afternoon).is_empty());
    assert_eq!(run.base(1, 1).supply_t, n(400));
    assert_eq!(run.ship(1, 1).supply_t, n(250));
    assert_eq!(run.ship(1, 2).supply_t, n(350));
    // The staff's load is not the commander's: no answer about a reserve was
    // given to either on day 1.
    assert!(run
        .on_day(1, |k| matches!(k, EventKind::LoadBelowReserve { .. }))
        .is_empty());
}

#[test]
fn a_commanders_second_load_may_take_the_stock_below_the_reserve() {
    let run = Run::text(&text());
    // Day 2: force 1 has 250 t and is to have 450: it wants 200 t more. Base
    // 1 holds nothing above its reserve. The order is accepted, and its
    // answer says it may take 200 t below the reserve of 400.
    let order = |kind: &OrderKind| {
        to_force(kind, 1, |o| {
            *o == TaskForceOrder::LoadTo {
                cargo: Cargo::Supply,
                weight_t: n(450),
            }
        })
    };
    assert!(run.accepted(2, order));
    let answer = run.one_on_day(2, |k| matches!(k, EventKind::LoadBelowReserve { .. }));
    match answer.kind {
        EventKind::LoadBelowReserve {
            task_force,
            base,
            cargo,
            below_t,
            reserve_t,
        } => assert_eq!(
            (task_force.raw(), base.raw(), cargo, below_t, reserve_t),
            (1, 1, Cargo::Supply, n(200), n(400))
        ),
        _ => unreachable!(),
    }
    // In the night pulse it has the berth and loads 200 t in 4 hours; base 1
    // holds 200 t.
    assert_eq!(run.loaded(2, Pulse::Night, 1), (n(200), n(450), n(200)));
    assert_eq!(
        run.complete(2, Pulse::Night, 1),
        pulse_start(2, Pulse::Night) + n(4)
    );
    assert_eq!(run.loaders(2, Pulse::Night), vec![1]);
    assert!(run.loaders(2, Pulse::Morning).is_empty());
    assert!(run.loaders(2, Pulse::Afternoon).is_empty());
    assert_eq!(run.base(2, 1).supply_t, n(200));

    // At the end: base 1 holds 200 t of supply and 300 t of fuel. Ship 1 has
    // 450 t of supply and 1000 t of fuel; ship 2 has 350 t and 950 t.
    let end = 4;
    assert_eq!(run.base(end, 1).supply_t, n(200));
    assert_eq!(run.base(end, 1).fuel_t, n(300));
    assert_eq!(
        (run.ship(end, 1).supply_t, run.ship(end, 1).fuel_t),
        (n(450), n(1000))
    );
    assert_eq!(
        (run.ship(end, 2).supply_t, run.ship(end, 2).fuel_t),
        (n(350), n(950))
    );
    // Nothing more moves at S on days 3 and 4.
    for day in [3, 4] {
        assert!(run
            .on_day(
                day,
                |k| matches!(k, EventKind::CargoLoaded { base, .. } if base.raw() == 1)
            )
            .is_empty());
    }
}

#[test]
fn a_weight_is_shared_between_the_ships_of_a_force_in_ship_order() {
    let run = Run::text(&text());
    let night = pulse_start(1, Pulse::Night);
    // Day 1 at T, by lighter, both ships at once. Ship 3 is filled first:
    // 30 t, its room. Ship 4 takes the other 20 t. Base 2 holds 50 t. The
    // load is complete; it takes 30 / 20 = 1.5 hours for ship 3.
    assert_eq!(run.loaded(1, Pulse::Night, 3), (n(30), n(30), n(70)));
    assert_eq!(run.loaded(1, Pulse::Night, 4), (n(20), n(20), n(50)));
    assert_eq!(
        run.complete(1, Pulse::Night, 3),
        night + Fixed::from_ratio(3, 2)
    );
    assert_eq!(run.base(1, 2).supply_t, n(50));
    assert!(run.loaders(1, Pulse::Morning).iter().all(|s| *s == 2));
    assert!(run.loaders(1, Pulse::Afternoon).is_empty());
    // Ships 5 and 6 are in no force: nothing is loaded into them on day 1.
    assert_eq!(run.ship(1, 5).supply_t, n(0));
    assert_eq!(run.ship(1, 6).supply_t, n(0));

    // Day 3: the formed force loads up to 60 t. It has 50 t aboard and
    // wants 10 more: ship 3 is full; ship 4 has room for 10 and takes them
    // in half an hour. Ships 5 and 6 take nothing. Base 2 holds 40 t.
    assert_eq!(run.loaders(3, Pulse::Night), vec![4]);
    assert!(run.loaders(3, Pulse::Morning).is_empty());
    assert!(run.loaders(3, Pulse::Afternoon).is_empty());
    assert_eq!(run.loaded(3, Pulse::Night, 4), (n(10), n(30), n(40)));
    assert_eq!(
        run.complete(3, Pulse::Night, FORMED),
        pulse_start(3, Pulse::Night) + Fixed::from_ratio(1, 2)
    );
    assert_eq!(run.base(2, 2).supply_t, n(50));
    assert_eq!(run.base(3, 2).supply_t, n(40));
    assert_eq!(run.base(4, 2).supply_t, n(40));
}

#[test]
fn ships_in_port_are_formed_into_the_sides_next_force() {
    let mut scenario = from(&text());
    scenario.next_day().expect("day 1 resolves");
    let truth = scenario.world.umpire().clone();

    // The event: the first number of the Allied made series.
    let formed: Vec<&Event> = scenario
        .world
        .events()
        .iter()
        .filter(|e| matches!(e.kind, EventKind::TaskForceFormed { .. }))
        .collect();
    assert_eq!(formed.len(), 1);
    match &formed[0].kind {
        EventKind::TaskForceFormed {
            task_force,
            side,
            base,
            ships,
        } => {
            assert_eq!(task_force.raw(), 1_000_001);
            assert_eq!(task_force.raw(), FORMED);
            assert_eq!(*side, Side::Allied);
            assert_eq!(base.raw(), 2);
            assert_eq!(
                ships.iter().map(|s| s.raw()).collect::<Vec<_>>(),
                vec![5, 6]
            );
        }
        _ => unreachable!(),
    }
    assert_eq!(formed[0].day, 1);

    // The handle the side sees: the agent numbers a side's own forces by
    // their position in the side's view, in id order, from 1
    // (`force_number` in crates/sim-agent/src/common.rs). The formed force
    // is the side's force 4.
    let view = scenario.world.side_view(Side::Allied);
    let ids: Vec<TaskForceId> = view.task_forces().map(|t| t.id).collect();
    assert_eq!(ids, vec![tf(1), tf(2), tf(3), tf(FORMED)]);
    let position = ids.iter().position(|id| *id == tf(FORMED)).expect("seen") + 1;
    assert_eq!(position, 4);

    // Stopped at base 2 at T with no route, at cruise speed, with base 2 as
    // its home port and the engagement settings a force has when a scenario
    // gives it none. Its mission is Transport (decision 0111, 12: no rule
    // but bombardment reads a mission).
    let force = &truth.task_forces[&tf(FORMED)];
    assert_eq!(force.status, TaskForceStatus::Idle);
    assert_eq!(force.position(), T);
    assert_eq!(force.toward_hex, None);
    assert_eq!(force.mission, Mission::Transport);
    assert_eq!(force.speed, SpeedSetting::Cruise);
    assert_eq!(force.home_port, Some(base_id(2)));
    assert_eq!(force.rules_of_engagement, RulesOfEngagement::DEFAULT);
    assert_eq!(force.route_order, None);
    assert_eq!(force.speed_order, None);
    assert_eq!(force.rules_order, None);
    assert_eq!(
        force.ships.iter().map(|s| s.raw()).collect::<Vec<_>>(),
        vec![5, 6]
    );
    for raw in [5, 6] {
        assert_eq!(
            truth.ships[&ship_id(raw)].location,
            ShipLocation::TaskForce(tf(FORMED))
        );
    }
    // Force 3 is as it was: ships 3 and 4 still lie with it.
    assert_eq!(truth.task_forces[&tf(3)].ships.len(), 2);
}

#[test]
fn a_force_is_joined_and_another_stood_down_and_each_ceases_to_be() {
    // Days 1 to 4 as the file has them, then a fifth day with an order to
    // force 3 on day 3 and to the formed force on day 5, to see them refused.
    let source = text().replace("days = 4", "days = 5")
        + &order(
            3,
            "task_force = 3",
            "kind = \"set_speed\"\nspeed = \"cruise\"",
        )
        + &order(
            5,
            "task_force = 1000001",
            "kind = \"set_speed\"\nspeed = \"cruise\"",
        );
    let run = Run::text(&source);

    // Day 2: force 3 is joined to the formed force. It has ships 3, 4, 5 and
    // 6, with the 30 t and 20 t aboard ships 3 and 4. Force 3 has ceased to
    // be.
    let joined = run.one_on_day(2, |k| matches!(k, EventKind::TaskForceJoined { .. }));
    match &joined.kind {
        EventKind::TaskForceJoined {
            joined,
            into,
            base,
            ships,
        } => {
            assert_eq!((joined.raw(), into.raw(), base.raw()), (3, FORMED, 2));
            assert_eq!(
                ships.iter().map(|s| s.raw()).collect::<Vec<_>>(),
                vec![3, 4]
            );
        }
        _ => unreachable!(),
    }
    assert_eq!(run.ships_of(1, FORMED), vec![5, 6]);
    assert_eq!(run.ships_of(1, 3), vec![3, 4]);
    assert_eq!(run.ships_of(2, FORMED), vec![3, 4, 5, 6]);
    assert!(run.ships_of(2, 3).is_empty());
    assert_eq!(
        (run.ship(2, 3).supply_t, run.ship(2, 4).supply_t),
        (n(30), n(20))
    );
    for raw in [3, 4, 5, 6] {
        assert_eq!(
            run.ship(2, raw).location,
            ShipLocation::TaskForce(tf(FORMED))
        );
    }
    // The receiving force keeps its place, speed and settings.
    let force = run.force(2, FORMED);
    assert_eq!((force.status, force.position()), (TaskForceStatus::Idle, T));
    // An order to force 3 is refused, naming the force it was joined to.
    assert_eq!(
        run.refusal(3, |k| to_force(k, 3, |_| true)),
        RejectReason::ForceWasJoined { into: tf(FORMED) }
    );

    // Day 4: the formed force is stood down. Ships 3, 4, 5 and 6 lie in port
    // at base 2 in no force, with 30, 30, 0 and 0 t aboard. The formed force
    // has ceased to be, and an order to it is refused.
    assert_eq!(run.ships_of(3, FORMED), vec![3, 4, 5, 6]);
    assert_eq!(
        (run.ship(3, 3).supply_t, run.ship(3, 4).supply_t),
        (n(30), n(30))
    );
    let stood = run.one_on_day(4, |k| {
        matches!(k, EventKind::TaskForceStoodDownInPort { .. })
    });
    match &stood.kind {
        EventKind::TaskForceStoodDownInPort {
            task_force,
            base,
            ships,
        } => {
            assert_eq!((task_force.raw(), base.raw()), (FORMED, 2));
            assert_eq!(
                ships.iter().map(|s| s.raw()).collect::<Vec<_>>(),
                vec![3, 4, 5, 6]
            );
        }
        _ => unreachable!(),
    }
    assert!(run.ships_of(4, FORMED).is_empty());
    for (raw, aboard) in [(3, 30), (4, 30), (5, 0), (6, 0)] {
        let ship = run.ship(4, raw);
        assert_eq!(ship.location, ShipLocation::Port(base_id(2)));
        assert_eq!(ship.supply_t, n(aboard));
        assert_eq!(ship.fuel_t, if raw <= 4 { n(500) } else { n(1000) });
    }
    assert_eq!(
        run.refusal(5, |k| to_force(k, FORMED, |_| true)),
        RejectReason::ForceWasStoodDown
    );
    // The stood-down ships work no cargo, refuel or move on day 5.
    assert!(run
        .on_day(5, |k| matches!(
            k,
            EventKind::CargoLoaded { .. }
                | EventKind::ShipRefuelled { .. }
                | EventKind::FuelBurned { .. }
        ))
        .is_empty());
}

// ------------------------------------------------------------- The properties

/// Property 1. With no weight, no reserve and none of the three orders every
/// scenario gives what it gave before, apart from the hash version. That is
/// proved for every scenario by the run baselines
/// (`crates/sim-cli/tests/run_baseline`) and `scripts/compare-scenarios.sh`,
/// which compare each scenario's whole run, event by event, with and without
/// this change. Here: scenario 39 with every weight, reserve and new order
/// left out runs as it does with the reserve keys present at 0, and records
/// none of the new events.
#[test]
fn with_no_weight_no_reserve_and_no_new_order_nothing_new_happens() {
    let plain = without_orders(&text(), |b| is_new_order(b) || b.contains("task_force = 3"));
    // The loads lose their weights and fill the room instead.
    let plain = plain
        .replace("weight_t = 250\n", "")
        .replace("weight_t = 500\n", "")
        .replace("weight_t = 450\n", "");
    let without_keys = plain
        .replace("supply_reserve_t = 400\n", "")
        .replace("fuel_reserve_t = 300\n", "");
    assert!(!without_keys.contains("weight_t") && !without_keys.contains("reserve_t"));
    let with_zero = common::replace_once(
        &without_keys,
        "supply_t = 1000\nfuel_t = 500\n",
        "supply_t = 1000\nfuel_t = 500\nsupply_reserve_t = 0\nfuel_reserve_t = 0\n",
    );
    assert!(with_zero.contains("fuel_reserve_t = 0"));

    let a = Run::text(&without_keys);
    let b = Run::text(&with_zero);
    assert_eq!(digest(&a), digest(&b));
    assert_eq!(truths(&a), truths(&b));
    assert_eq!(a.japanese, b.japanese);

    // None of the new events, and nothing refused.
    assert!(a
        .all(|k| matches!(
            k,
            EventKind::TaskForceFormed { .. }
                | EventKind::TaskForceJoined { .. }
                | EventKind::TaskForceStoodDownInPort { .. }
                | EventKind::LoadBelowReserve { .. }
                | EventKind::RefuellingHeldAtReserve { .. }
                | EventKind::LoadHeldAtReserve { .. }
                | EventKind::OrderRejected { .. }
        ))
        .is_empty());
    // With no reserve the refuelling takes the whole of what is wanted (150
    // t and 100 t: base 1 holds 500 t) and a load takes what the room and
    // the stock allow: the two ships between them take the stock of 1000 t
    // (the commander's load first, in force order, since both are at the
    // berth or by lighter at once and the stock is shared as the night goes).
    assert_eq!(
        a.refuelled(1),
        vec![
            (1, 1, n(150), n(1000), n(350)),
            (2, 1, n(100), n(1000), n(250)),
        ]
    );
    let taken = a.base(1, 1).supply_t;
    assert_eq!(taken, n(0), "with no reserve the stock is taken to nothing");
    assert_eq!(
        a.ship(1, 1).supply_t + a.ship(1, 2).supply_t,
        n(1000),
        "the two loads are the stock between them"
    );
}

/// Property 2. A load with a weight never puts more aboard a force than the
/// weight, and never more in a ship than its room. The weights are 1, 25,
/// 30, 31, 59, 61 and 100000 on the loads of force 1 (class 3, room 3000 t,
/// the stock 1000 t) and force 3 (two ships of room 30 t). Nothing is invented
/// beyond these.
#[test]
fn a_load_never_puts_more_aboard_than_its_weight_or_a_ships_room() {
    let weights = [1_i64, 25, 30, 31, 59, 61, 100_000];
    for weight in weights {
        let source = common::replace_once(
            &common::replace_once(
                &text(),
                "task_force = 1\nkind = \"load\"\nweight_t = 250\n",
                &format!("task_force = 1\nkind = \"load\"\nweight_t = {weight}\n"),
            ),
            "task_force = 3\nkind = \"load\"\nweight_t = 50\n",
            &format!("task_force = 3\nkind = \"load\"\nweight_t = {weight}\n"),
        );
        let run = Run::text(&source);

        // After every `CargoLoaded` no ship is over its room: class 3 holds
        // 3000 t and class 12 holds 30 t, of supply and fuel cargo together.
        for event in run.all(|k| matches!(k, EventKind::CargoLoaded { .. })) {
            if let EventKind::CargoLoaded { ship, aboard_t, .. } = event.kind {
                let room = if ship.raw() <= 2 || ship.raw() >= 5 {
                    3000
                } else {
                    30
                };
                assert!(aboard_t <= n(room), "weight {weight}: {event:?}");
                assert!(aboard_t > n(0), "a load puts something aboard");
            }
        }
        for day in 1..=4 {
            for raw in 1..=6 {
                let ship = run.ship(day, raw);
                let room = if raw <= 2 || raw >= 5 { 3000 } else { 30 };
                assert!(ship.supply_t + ship.cargo_fuel_t <= n(room));
            }
            // At the end of each day no force has more aboard than its
            // weight: force 1's is the weight of the day-1 order, then 450;
            // force 2's is 500; force 3's is the weight and the formed
            // force's 60 (it joined with what force 3 had, at most 60).
            let aboard = |ships: &[u32]| {
                ships
                    .iter()
                    .fold(n(0), |sum, raw| sum + run.ship(day, *raw).supply_t)
            };
            let force_1 = if day == 1 {
                n(weight)
            } else {
                n(weight).max(n(450))
            };
            assert!(aboard(&[1]) <= force_1, "weight {weight} day {day}");
            assert!(aboard(&[2]) <= n(500), "weight {weight} day {day}");
            if day == 1 {
                assert!(aboard(&[3, 4]) <= n(weight), "weight {weight} day {day}");
            } else {
                assert!(aboard(&[3, 4, 5, 6]) <= n(60), "weight {weight} day {day}");
            }
        }
        // Force 3 fills ship 3 first, then ship 4: the amounts are exact.
        let first = weight.min(30);
        let second = (weight - 30).clamp(0, 30);
        assert_eq!(run.ship(1, 3).supply_t, n(first), "weight {weight}");
        assert_eq!(run.ship(1, 4).supply_t, n(second), "weight {weight}");
        // Force 1 never has more than the weight and never more than the
        // stock allowed: base 1 holds 1000 t, and never less than 0.
        assert!(run.ship(1, 1).supply_t <= n(weight.min(1000)));
        assert!(run.base(1, 1).supply_t >= n(0));
    }
}

/// Property 3. Refuelling never takes a base's fuel below its reserve and a
/// load whose origin is the staff never takes its supply below its reserve,
/// on any day of a run in which the men's supply is also drawn from the base.
///
/// Chosen for the property: 20 000 men ashore at S, who draw 0.005 t each a
/// night (the frozen ground figure), 100 t a night, and staff loads of force
/// 2 on days 1, 2 and 3 (to 500, 700 and 900 t). The commander's load of
/// force 1 on day 2 is left out so that only the men take the stock below its
/// reserve.
#[test]
fn a_reserve_holds_for_refuelling_and_the_staffs_load_while_men_eat_from_the_base() {
    let source = head().replace(
        "surface_combat = \"../reference/surface_combat.toml\"\n",
        "surface_combat = \"../reference/surface_combat.toml\"\nground = \"../reference/ground.toml\"\n",
    ) + "[[land_unit]]\nid = 1\nname = \"Placeholder men at S\"\nside = \"allied\"\nmen = 20000\nq = 0\nr = 0\nposture = \"defend\"\n\n"
        + &order(1, "task_force = 1", "kind = \"load\"\nweight_t = 250")
        + &order(1, "task_force = 2\nissuer = \"staff\"", "kind = \"load\"\nweight_t = 500")
        + &order(2, "task_force = 2\nissuer = \"staff\"", "kind = \"load\"\nweight_t = 700")
        + &order(3, "task_force = 2\nissuer = \"staff\"", "kind = \"load\"\nweight_t = 900");
    let run = Run::text(&source);

    // Every refuelling leaves the base's fuel at or above its reserve (base
    // 1's is 300), and some refuelling goes exactly to it.
    let refuellings = run.all(|k| matches!(k, EventKind::ShipRefuelled { .. }));
    assert!(!refuellings.is_empty());
    for event in &refuellings {
        if let EventKind::ShipRefuelled {
            base, base_fuel_t, ..
        } = event.kind
        {
            let reserve = if base.raw() == 1 { n(300) } else { n(0) };
            assert!(base_fuel_t >= reserve, "{event:?}");
        }
    }
    assert!(refuellings.iter().any(
        |e| matches!(e.kind, EventKind::ShipRefuelled { base_fuel_t, .. } if base_fuel_t == n(300))
    ));

    // Every load of the staff's force leaves the supply at or above the
    // reserve of 400.
    let staff = run
        .all(|k| matches!(k, EventKind::CargoLoaded { task_force, .. } if task_force.raw() == 2));
    assert!(!staff.is_empty());
    for event in &staff {
        if let EventKind::CargoLoaded { base_stock_t, .. } = event.kind {
            assert!(base_stock_t >= n(400), "{event:?}");
        }
    }
    // The property is not empty: the men's draw takes the supply below the
    // reserve on some day (the base's supply at the end of a day is under
    // 400), and the staff's load still held the line before that.
    let lowest = (1..=4)
        .map(|day| run.base(day, 1).supply_t)
        .min()
        .expect("days");
    assert!(
        lowest < n(400),
        "the men take the supply below the reserve: {lowest:?}"
    );
    assert!(
        run.on_day(1, |k| matches!(k, EventKind::LoadHeldAtReserve { .. }))
            .len()
            == 1
    );
    // The men are charged first in the night: day 1 starts the night with
    // 1000 - 100 = 900 t, force 1 takes 250 (650 t), force 2 takes 240 by
    // lighter (410 t) and 10 t in the morning, to the reserve: 250 t aboard,
    // not 350. On days 2 and 3 the men have taken the base below its reserve
    // and the staff's loads take nothing: force 2 has the 250 t at the end.
    assert_eq!(run.loaded(1, Pulse::Night, 2), (n(240), n(240), n(410)));
    assert_eq!(run.loaded(1, Pulse::Morning, 2), (n(10), n(250), n(400)));
    assert_eq!(run.ship(4, 2).supply_t, n(250));
    assert_eq!(
        run.all(
            |k| matches!(k, EventKind::CargoLoaded { task_force, .. } if task_force.raw() == 2)
        )
        .len(),
        2
    );
    assert_eq!(
        [1, 2, 3, 4].map(|day| run.base(day, 1).supply_t),
        [n(400), n(300), n(200), n(100)]
    );
    // The commander's force 1 was not capped: it has its 250 t from day 1.
    assert_eq!(run.ship(4, 1).supply_t, n(250));
}

/// Property 4. Where a side has two bases in a hex, a force that refuels
/// draws on them in base order, each down to its reserve. A second Allied
/// base, 3, at S with 200 t of fuel and a reserve of 150. Ship 1 has 700 t
/// (wants 300) and ship 2 has 500 t (wants 500), chosen for the property.
/// Base 1 may give 200 t, base 3 may give 50 t. Each ship is served by the
/// lowest-numbered base with fuel above its reserve (`refuel_in_port`): ship 1
/// takes 200 from base 1 (300 left, its reserve) and is 100 t short; ship 2,
/// served next, finds base 1 at its reserve and takes 50 from base 3 (150
/// left, its reserve) and is 450 t short. A ship does not go on from one base
/// to the next.
#[test]
fn a_force_refuels_from_the_bases_of_its_hex_in_base_order_each_down_to_its_reserve() {
    let source = head()
        .replace("fuel_t = 850\n", "fuel_t = 700\n")
        .replace("fuel_t = 900\n", "fuel_t = 500\n")
        + "[[base]]\nid = 3\nname = \"Placeholder second base at S, Allied\"\nside = \"allied\"\nq = 0\nr = 0\nfuel_t = 200\nfuel_reserve_t = 150\n";
    let run = Run::text(&source);
    assert_eq!(
        run.refuelled(1),
        vec![
            (1, 1, n(200), n(900), n(300)),
            (2, 3, n(50), n(550), n(150)),
        ]
    );
    assert_eq!(run.held_refuelling(1), vec![(1, n(100)), (2, n(450))]);
    assert_eq!(run.base(1, 1).fuel_t, n(300));
    assert_eq!(run.base(1, 3).fuel_t, n(150));
    assert_eq!(run.ship(1, 1).fuel_t, n(900));
    assert_eq!(run.ship(1, 2).fuel_t, n(550));
    // Later days take nothing more: both bases are at their reserves.
    for day in 2..=4 {
        assert!(run.refuelled(day).is_empty());
    }
}

/// Property 5, at sea and at a base of the other side: a force at sea, one at
/// a Japanese base, a ship in a force and a base of the other side or none.
/// A Japanese base 4 at (5,0), allied forces 4 at (3,0) at sea and 5 at
/// (5,0), with ships 7 and 8.
#[test]
fn forming_joining_and_standing_down_are_refused_at_sea_and_at_the_enemys_base() {
    let source = head()
        + "[[base]]\nid = 4\nname = \"Placeholder base, Japanese\"\nside = \"japanese\"\nq = 5\nr = 0\nsupply_t = 10\n\n\
           [[task_force]]\nid = 4\nname = \"Placeholder force at sea\"\nside = \"allied\"\nq = 3\nr = 0\nmission = \"transport\"\n\n\
           [[ship]]\nid = 7\nname = \"Placeholder cargo ship 7\"\nclass = 3\nside = \"allied\"\nfuel_t = 1000\ntask_force = 4\n\n\
           [[task_force]]\nid = 5\nname = \"Placeholder force at the Japanese base\"\nside = \"allied\"\nq = 5\nr = 0\nmission = \"transport\"\n\n\
           [[ship]]\nid = 8\nname = \"Placeholder cargo ship 8\"\nclass = 3\nside = \"allied\"\nfuel_t = 1000\ntask_force = 5\n\n"
        + &order(1, "task_force = 4", "kind = \"stand_down\"")
        + &order(1, "task_force = 4", "kind = \"join\"\ninto = 1")
        + &order(1, "task_force = 1", "kind = \"join\"\ninto = 4")
        + &order(1, "task_force = 5", "kind = \"stand_down\"")
        + &order(1, "task_force = 5", "kind = \"join\"\ninto = 1")
        + &order(1, "task_force = 1", "kind = \"join\"\ninto = 5")
        + &order(1, "base = 1", "kind = \"form_task_force\"\nships = [7]")
        + &order(1, "base = 1", "kind = \"form_task_force\"\nships = [8]")
        + &order(1, "base = 1", "kind = \"form_task_force\"\nships = [2]")
        + &order(1, "base = 4", "kind = \"form_task_force\"\nships = [5, 6]")
        + &order(1, "base = 99", "kind = \"form_task_force\"\nships = [5, 6]");
    let run = Run::text(&source);
    let no_base = RejectReason::NoOwnBaseHere;
    // At sea: no base of the side is here.
    assert_eq!(run.refusal(1, |k| to_force(k, 4, is_stand_down)), no_base);
    assert_eq!(run.refusal(1, |k| to_force(k, 4, is_join)), no_base);
    // The force to join at sea, or at the enemy's base, is refused too.
    assert_eq!(
        run.refusal(1, |k| to_force(k, 1, |o| *o
            == TaskForceOrder::JoinTo(tf(4)))),
        no_base
    );
    // At a base of the other side: the force is not at a base of its own.
    assert_eq!(run.refusal(1, |k| to_force(k, 5, is_stand_down)), no_base);
    assert_eq!(run.refusal(1, |k| to_force(k, 5, is_join)), no_base);
    assert_eq!(
        run.refusal(1, |k| to_force(k, 1, |o| *o
            == TaskForceOrder::JoinTo(tf(5)))),
        no_base
    );
    // Forming: a ship at sea in a force, a ship at the enemy's base in a
    // force and a ship in a force at the base itself are not ships in port in
    // no force.
    assert_eq!(
        run.refusal(1, |k| forms(k, 1, &[7])),
        RejectReason::ShipNotInPortHere(ship_id(7))
    );
    assert_eq!(
        run.refusal(1, |k| forms(k, 1, &[8])),
        RejectReason::ShipNotInPortHere(ship_id(8))
    );
    assert_eq!(
        run.refusal(1, |k| forms(k, 1, &[2])),
        RejectReason::ShipNotInPortHere(ship_id(2))
    );
    // A base of the other side, or one that does not exist, is not the
    // side's base.
    assert_eq!(
        run.refusal(1, |k| forms(k, 4, &[5, 6])),
        RejectReason::NotYourBase
    );
    assert_eq!(
        run.refusal(1, |k| forms(k, 99, &[5, 6])),
        RejectReason::NotYourBase
    );
    // Nothing was formed, joined or stood down.
    assert!(run
        .all(|k| matches!(
            k,
            EventKind::TaskForceFormed { .. }
                | EventKind::TaskForceJoined { .. }
                | EventKind::TaskForceStoodDownInPort { .. }
        ))
        .is_empty());
}

/// Property 5, a force under way: force 1 is given a route to T (400 nm, 40
/// hours at the 10 knots of its class) on day 1, and on day 1 a stand-down
/// after it; on day 2 a stand-down and joins are given to it and to forces
/// that would join it.
#[test]
fn forming_joining_and_standing_down_are_refused_for_a_force_under_way() {
    let source = head()
        + &order(
            1,
            "task_force = 1",
            "kind = \"set_route\"\nwaypoints = [[10, 0]]",
        )
        + &order(1, "task_force = 1", "kind = \"stand_down\"")
        + &order(2, "task_force = 1", "kind = \"stand_down\"")
        + &order(2, "task_force = 1", "kind = \"join\"\ninto = 2")
        + &order(2, "task_force = 2", "kind = \"join\"\ninto = 1")
        + &order(2, "task_force = 3", "kind = \"join\"\ninto = 1")
        + &order(2, "base = 1", "kind = \"form_task_force\"\nships = [1]");
    let run = Run::text(&source);
    // The route was taken: force 1 is on its way at the end of day 1 and
    // arrives in the morning of day 2, after the day's orders were taken in.
    assert!(run.accepted(1, |k| matches!(k, OrderKind::TaskForce { target, order: TaskForceOrder::SetRoute { .. } } if target.raw() == 1)));
    assert_eq!(run.force(1, 1).status, TaskForceStatus::Underway);
    // The same day, after the route order, it is no longer stopped.
    assert_eq!(
        run.refusal(1, |k| to_force(k, 1, is_stand_down)),
        RejectReason::UnderWay
    );
    // The next day: a force under way, joined or joined to.
    assert_eq!(
        run.refusal(2, |k| to_force(k, 1, is_stand_down)),
        RejectReason::UnderWay
    );
    assert_eq!(
        run.refusal(2, |k| to_force(k, 1, is_join)),
        RejectReason::UnderWay
    );
    assert_eq!(
        run.refusal(2, |k| to_force(k, 2, is_join)),
        RejectReason::UnderWay
    );
    assert_eq!(
        run.refusal(2, |k| to_force(k, 3, is_join)),
        RejectReason::UnderWay
    );
    assert_eq!(
        run.refusal(2, |k| forms(k, 1, &[1])),
        RejectReason::ShipNotInPortHere(ship_id(1))
    );
}

/// Property 5, a force away at the yard: base 1 is a port for the yard (a
/// passage of 6 days, longer than the game) and force 1 is sent there on day
/// 1; its orders, a join to it and a join of it on day 2 are refused, and so
/// is the forming of its ship.
#[test]
fn forming_joining_and_standing_down_are_refused_for_a_force_away_at_the_yard() {
    let source = common::replace_once(
        &head(),
        "fuel_reserve_t = 300\n",
        "fuel_reserve_t = 300\nyard_passage_days = 6\n",
    ) + &order(1, "task_force = 1", "kind = \"send_to_yard\"")
        + &order(2, "task_force = 1", "kind = \"stand_down\"")
        + &order(2, "task_force = 1", "kind = \"join\"\ninto = 2")
        + &order(2, "task_force = 2", "kind = \"join\"\ninto = 1")
        + &order(2, "base = 1", "kind = \"form_task_force\"\nships = [1]");
    let run = Run::text(&source);
    assert!(run.accepted(1, |k| to_force(k, 1, |o| *o == TaskForceOrder::SendToYard)));
    assert_eq!(run.ship(1, 1).location, ShipLocation::Yard(tf(1)));
    assert_eq!(
        run.refusal(2, |k| to_force(k, 1, is_stand_down)),
        RejectReason::AwayAtYard
    );
    assert_eq!(
        run.refusal(2, |k| to_force(k, 1, is_join)),
        RejectReason::AwayAtYard
    );
    assert_eq!(
        run.refusal(2, |k| to_force(k, 2, is_join)),
        RejectReason::AwayAtYard
    );
    assert_eq!(
        run.refusal(2, |k| forms(k, 1, &[1])),
        RejectReason::ShipNotInPortHere(ship_id(1))
    );
}

/// Property 5, a join between forces at different bases, and the refusals
/// that need no other place: a force joined to itself or to one that is not
/// the side's, a forming order with no ship, a ship twice, a ship not in port
/// there, and a weight that is not above 0.
#[test]
fn a_join_between_forces_at_different_bases_and_the_other_refusals() {
    let source = head()
        + &order(1, "task_force = 3", "kind = \"join\"\ninto = 1")
        + &order(1, "task_force = 1", "kind = \"join\"\ninto = 3")
        + &order(1, "task_force = 1", "kind = \"join\"\ninto = 1")
        + &order(1, "task_force = 1", "kind = \"join\"\ninto = 99")
        + &order(1, "base = 2", "kind = \"form_task_force\"\nships = []")
        + &order(
            1,
            "base = 2",
            "kind = \"form_task_force\"\nships = [5, 6, 5]",
        )
        + &order(1, "base = 2", "kind = \"form_task_force\"\nships = [5, 9]")
        + &order(1, "base = 1", "kind = \"form_task_force\"\nships = [5]")
        + &order(1, "base = 2", "kind = \"form_task_force\"\nships = [3, 5]")
        + &order(1, "task_force = 2", "kind = \"load\"\nweight_t = 0")
        + &order(1, "task_force = 3", "kind = \"load\"\nweight_t = -5");
    let run = Run::text(&source);
    assert_eq!(
        run.refusal(1, |k| to_force(k, 3, is_join)),
        RejectReason::NotAtSameBase(tf(1))
    );
    assert_eq!(
        run.refusal(1, |k| to_force(k, 1, |o| *o
            == TaskForceOrder::JoinTo(tf(3)))),
        RejectReason::NotAtSameBase(tf(3))
    );
    assert_eq!(
        run.refusal(1, |k| to_force(k, 1, |o| *o
            == TaskForceOrder::JoinTo(tf(1)))),
        RejectReason::CannotJoinItself
    );
    assert_eq!(
        run.refusal(1, |k| to_force(k, 1, |o| *o
            == TaskForceOrder::JoinTo(tf(99)))),
        RejectReason::NotYourUnit
    );
    assert_eq!(
        run.refusal(1, |k| forms(k, 2, &[])),
        RejectReason::NoShipsToForm
    );
    assert_eq!(
        run.refusal(1, |k| forms(k, 2, &[5, 6, 5])),
        RejectReason::ShipNamedTwice(ship_id(5))
    );
    assert_eq!(
        run.refusal(1, |k| forms(k, 2, &[5, 9])),
        RejectReason::ShipNotInPortHere(ship_id(9))
    );
    assert_eq!(
        run.refusal(1, |k| forms(k, 1, &[5])),
        RejectReason::ShipNotInPortHere(ship_id(5))
    );
    assert_eq!(
        run.refusal(1, |k| forms(k, 2, &[3, 5])),
        RejectReason::ShipNotInPortHere(ship_id(3))
    );
    assert_eq!(
        run.refusal(1, |k| to_force(k, 2, is_load)),
        RejectReason::WeightNotAboveZero(n(0))
    );
    assert_eq!(
        run.refusal(1, |k| to_force(k, 3, is_load)),
        RejectReason::WeightNotAboveZero(n(-5))
    );
    // Nothing was formed or joined.
    assert!(run
        .all(|k| matches!(
            k,
            EventKind::TaskForceFormed { .. } | EventKind::TaskForceJoined { .. }
        ))
        .is_empty());
}

/// Property 6. The enemy's view is the same in two worlds that differ only
/// in which of the other side's ships are in which force at a base: the
/// scenario as it is (ships 5 and 6 formed into a force, force 3 joined to it
/// and it stood down), one with no forming, joining or standing down, and one
/// that only forms. The Japanese side has a base far away (base 9 at (12,2),
/// with 50 t) to see with, and no aircraft, so no dice are thrown.
#[test]
fn the_enemys_view_is_the_same_whichever_ships_are_in_which_force() {
    let japanese = "[[base]]\nid = 9\nname = \"Placeholder base, Japanese\"\nside = \"japanese\"\nq = 12\nr = 2\nsupply_t = 50\n\n";
    let with = |tail: String| head() + japanese + &tail;
    let tail = |keep: &dyn Fn(&str) -> bool| {
        let all = text();
        let from = all.find("# Day 1: three loads").expect("the orders");
        without_orders(&all[from..], |b| !keep(b) && is_new_order(b))
    };
    let everything = Run::text(&with(tail(&|_| true)));
    let nothing = Run::text(&with(tail(&|_| false)));
    let only_forming = Run::text(&with(tail(&|b| b.contains("form_task_force"))));

    // The worlds do differ for the Allies: forces and ships are where they
    // are.
    assert_eq!(everything.ships_of(4, FORMED), Vec::<u32>::new());
    assert!(!nothing.at(4).task_forces.contains_key(&tf(FORMED)));
    assert_eq!(only_forming.ships_of(4, FORMED), vec![5, 6]);
    assert_ne!(truths(&everything), truths(&nothing));

    // The Japanese side's view is the same on every day, and it is not empty.
    assert_eq!(everything.japanese, nothing.japanese);
    assert_eq!(everything.japanese, only_forming.japanese);
    assert!(everything.japanese.last().expect("days").contains("base"));
    // The Japanese side sees none of the new events.
    assert!(!everything
        .japanese
        .iter()
        .any(|view| view.contains("TaskForceFormed")
            || view.contains("TaskForceJoined")
            || view.contains("TaskForceStoodDownInPort")
            || view.contains("LoadBelowReserve")
            || view.contains("HeldAtReserve")));
}
