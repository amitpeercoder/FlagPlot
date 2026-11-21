//! Scenario 117: the way a force takes. A carrier force the staff has
//! stationed is given a way that keeps it out of the enemy's reach by
//! day, within how late its risk lets it arrive and what its fuel allows.
//!
//! Proves decision 0203, stage 1. The statements are the architect's,
//! written before the code. A mismatch goes back to the architect, and
//! nobody else changes a statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on a placeholder map of open
//! sea from q -16 to 16 and r -16 to 16, with no real ship. Hexes are 40
//! nm and are named (q,r). It is scenario 116's setting with a wider map
//! and the force further off.
//!
//! Side A has a carrier force C: one carrier and two destroyers, on the
//! carrier mission, stopped at sea at (16,0). Aboard are a bomber group
//! whose normal strike radius is 210 nm, a fighter group and a group that
//! searches. C cruises at 15 kn and has a full speed of 30 kn. Each
//! ship's class has a fuel capacity of 1,000 t and an endurance of 4,000
//! nm, so it burns 0.25 t a mile at cruise and four times that at full
//! speed. C has no home port unless a statement gives it one: base H at
//! (10,2). A has no aircraft at any base.
//!
//! Side B has what each statement gives it. A circle of danger is 15
//! hexes round an enemy base on A's map and 8 hexes round a contact
//! reported as a carrier and three pulses old or less; a hex at exactly
//! that distance is inside. The danger of a hex is how many hexes inside
//! a circle it lies and one more.
//!
//! A day is the night, 18:00 to 06:00, then the morning and the
//! afternoon, six hours each. A route given by the staff is steamed from
//! the start of the night. In twelve hours C makes 180 nm at cruise and
//! 360 nm at full speed; in six, 90 nm and 180 nm.
//!
//! The engine charges a run with no land in the way by the straight line
//! to the waypoint it steers for, to the nearest mile: 40 nm a step along
//! a line of the grid, less off it. It has a force enter a hex half-way
//! along the step into it. A hex entered at the very hour a pulse ends is
//! entered in that pulse.
//!
//! In each statement A's commander has given leave to launch, the order
//! has no forces limit, and C's commander is steady.
//!
//! # Statements: with no enemy known, as today
//!
//! The order is "deny the waters within 2 hexes of (0,0)". By decision
//! 0202 the station is (3,0), 13 hexes from C: 520 nm, 34.7 hours at
//! cruise, more than 12, and 17.3 at full speed.
//!
//! 1. **One point, as today.** At balanced and at press C is given the
//!    route (3,0) and full speed; at preserve the route (3,0) and cruise.
//!    The reason says nothing of a way.
//! 2. **Routed each morning, as today.** At preserve C is still under way
//!    on the morning of day 2, and is given the route (3,0) again that
//!    morning. A way that does not bend is not kept.
//!
//! # Statements: a carrier contact across the way, at preserve
//!
//! A holds a fresh contact reported as a carrier at (13,-7), K. On C's
//! row, r = 0, the hexes from (5,0) to (14,0) are inside K's circle:
//! (5,0) and (14,0) at 8 hexes from K, the eight between at 7. On the row
//! r = 1 the hexes from (5,1) to (13,1) are inside, at 8. The row r = 2
//! is clear. (3,0) is 10 hexes from K and outside, so it is still the
//! station. The order is at preserve, so C closes on nothing and goes at
//! cruise.
//!
//! 3. **The shortest way, by the engine's clock.** Along r = 0 each step
//!    is 40 nm, and C enters the hex of step n after 40n less 20 nm:
//!    hours 1.3, 4, 6.7, 9.3 and 12 for steps 1 to 5, so (15,0) to (11,0)
//!    are entered in the night of day 1; 14.7 and 17.3 for (10,0) and
//!    (9,0), the morning; 20 and 22.7 for (8,0) and (7,0), the afternoon;
//!    25.3 on for (6,0) to (3,0), the night of day 2. It arrives after
//!    34.7 hours, in the night of day 2. By day it enters four hexes
//!    inside K's circle, (10,0), (9,0), (8,0) and (7,0), each 7 hexes
//!    from K; by night six, (14,0) to (11,0), (6,0) and (5,0).
//! 4. **How late it may be.** At preserve the way may arrive no later
//!    than the night of day 3, the same pulse of the day after.
//! 5. **The way taken.** C is given the route (10,2), (6,2), (3,0) and
//!    cruise.
//!    - Its three legs are charged 212 nm ((16,0) to (10,2) is 40 times
//!      the root of 28, 211.7), 160 nm and 174 nm ((6,2) to (3,0) is 40
//!      times the root of 19, 174.4): 546 nm, 26 more than the shortest.
//!    - The engine steams it through (15,0), (14,0), (13,0), (12,0),
//!      (11,1), (10,2), (9,2), (8,2), (7,2), (6,2), (6,1), (6,0), (5,0),
//!      (4,0), (3,0).
//!    - It arrives after 36.4 hours, in the morning of day 2, a pulse
//!      later than the shortest way and inside the limit.
//!    - It enters (15,0) to (11,1) in the night of day 1, (10,2) and
//!      (9,2) in the morning, (8,2), (7,2) and (6,2) in the afternoon,
//!      and (6,1) to (3,0) in the night of day 2. By day it enters no hex
//!      inside K's circle. By night it enters seven: (14,0), (13,0),
//!      (12,0), (11,1), (6,1), (6,0) and (5,0).
//! 6. **The reason.** The reason for the route says it arrives in the
//!    morning of day 2 where the shortest way arrives in the night of day
//!    2; that the shortest way enters four hexes inside the reach of the
//!    contact by day, the nearest 7 hexes from it, and six by night; and
//!    that the way taken enters none by day and seven by night. It names
//!    the contact by its number and its age. It gives no figure of
//!    danger.
//! 7. **Kept.** On the morning of day 2 C is under way, in (6,2), and the
//!    staff gives it no route and no speed. Its route still ends at
//!    (3,0), and it arrives there in the morning of day 2.
//! 8. **The search on the way.** On the morning of day 1 C's search arc
//!    takes in the way ahead for a day at 15 kn, nine steps: (15,0),
//!    (14,0), (13,0), (12,0), (11,1), (10,2), (9,2), (8,2), (7,2), and
//!    the station (3,0), with the waters of the deny. On the morning of
//!    day 2 it takes in (6,1), (6,0), (5,0), (4,0) and (3,0).
//! 9. **A contact grown old casts no circle.** With K four pulses old on
//!    the morning of day 1, C is given the route (3,0), as in statement
//!    1.
//!
//! # Statements: fuel, at preserve
//!
//! Statement 5's setting, with H at (10,2) C's home port. From (3,0) to H
//! is 327 nm, 81.75 t at cruise. The margin at preserve is 1.5.
//!
//! 10. **What each way takes.** The way of statement 5: 546 nm at cruise,
//!     136.5 t, and 81.75 t home, 218.25 t, and 327.4 t with the margin.
//!     The shortest way: 130 t and 81.75 t, 211.75 t, and 317.6 t with
//!     the margin.
//! 11. **In fuel, the way of statement 5.** With every ship at 330 t C is
//!     given the route (10,2), (6,2), (3,0).
//! 12. **Fuel takes the clearest way away.** With every ship at 325 t the
//!     way of statement 5 is not in fuel. C is given the route (10,1),
//!     (6,1), (3,0): its legs are charged 223 nm ((16,0) to (10,1) is 40
//!     times the root of 31, 222.7), 160 nm and 144 nm ((6,1) to (3,0) is
//!     40 times the root of 13, 144.2), 527 nm; 131.75 t and 81.75 t,
//!     213.5 t, and 320.25 t with the margin. It arrives after 35.1
//!     hours, in the night of day 2. By day it enters four hexes inside
//!     K's circle, (10,1), (9,1), (8,1) and (7,1), each on its rim 8
//!     hexes from K, where the shortest way enters four that are each 7
//!     from it. The reason says so, and that a clearer way is out of
//!     fuel.
//! 13. **No clearer way in fuel.** With every ship at 319 t only the
//!     shortest way is in fuel. C is given the route (3,0), and is routed
//!     each morning as today. The reason says the way lies in the reach
//!     of the contact by day and that no clearer way is in fuel.
//!
//! # Statements: an enemy base beside the way
//!
//! The order is "deny the waters within 2 hexes of (-10,0)". B has a base
//! E at (9,-15) on A's map, and A holds no contact. On C's row the hexes
//! from (-6,0) to (9,0) are inside E's circle, each at 15 hexes, its rim.
//! The row r = 1 is clear. By decision 0202 the station is (-7,0): it is
//! 16 hexes from E and outside, and 23 hexes from C, the least passage of
//! any candidate. The shortest way is 920 nm: 61.3 hours at cruise and
//! 30.7 at full speed.
//!
//! 14. **At press, as today.** C is given the route (-7,0) and full
//!     speed, and the reason says nothing of a way.
//! 15. **At balanced, the shortest way by the engine's clock.** At full
//!     speed C enters the hex of step n after 40n less 20 nm at 30 kn:
//!     steps 1 to 9, (15,0) to (7,0), in the night of day 1; steps 10 to
//!     14, (6,0) to (2,0), in the morning (step 14 at hour 18 exactly);
//!     steps 15 to 18, (1,0) to (-2,0), in the afternoon; steps 19 to 23
//!     in the night of day 2. It arrives after 30.7 hours, in the night
//!     of day 2. By day it enters nine hexes inside E's circle, (6,0) to
//!     (-2,0), each 15 hexes from E; by night seven, (9,0), (8,0), (7,0)
//!     and (-3,0) to (-6,0). It may arrive no later than the morning of
//!     day 2.
//! 16. **At balanced, the way taken.** C is given the route (6,1),
//!     (-3,1), (-7,0) and full speed.
//!     - Its legs are charged 382 nm ((16,0) to (6,1) is 40 times the
//!       root of 91, 381.6), 360 nm and 183 nm ((-3,1) to (-7,0) is 40
//!       times the root of 21, 183.3): 925 nm, 5 more than the shortest.
//!     - The engine steams it along r = 0 to (7,0), then (6,1) and along
//!       r = 1 to (-3,1), then (-3,0) and along r = 0 to (-7,0).
//!     - It enters (7,0) after 327.5 nm, hour 10.9, in the night; (6,1)
//!       after 362 nm, hour 12.1, in the morning; (-2,1) after 682 nm,
//!       hour 22.7, in the afternoon; (-3,1) after 722 nm, hour 24.1, in
//!       the night of day 2.
//!     - It arrives after 30.8 hours, in the night of day 2, the pulse
//!       the shortest way arrives in.
//!     - By day it enters no hex inside E's circle. By night it enters
//!       seven: (9,0), (8,0), (7,0), (-3,0), (-4,0), (-5,0) and (-6,0).
//! 17. **At balanced, kept and reasoned.** The reason says both ways
//!     arrive in the night of day 2; that the shortest enters nine hexes
//!     inside the reach of E by day, the nearest 15 hexes from it; and
//!     that the way taken enters none by day. On the morning of day 2 C
//!     is given no route and no speed.
//! 18. **At preserve.** C goes at cruise. The shortest way arrives after
//!     61.3 hours, in the morning of day 3, and by day enters seven hexes
//!     inside E's circle: (9,0), (8,0) and (7,0) on day 1 and (1,0) to
//!     (-2,0) on day 2. C is given the route (9,1), (-3,1), (-7,0) and
//!     cruise: 925 nm (the legs 262, 480 and 183; (16,0) to (9,1) is 40
//!     times the root of 43, 262.3), arriving after 61.7 hours in the
//!     morning of day 3. By day it enters no hex inside E's circle.
//! 19. **In the dark, the shortest way.** With C at (0,0), inside E's
//!     circle, the station is still (-7,0), 7 hexes and 280 nm: 9.3 hours
//!     at full speed. At balanced every hex of the shortest way is
//!     entered in the night of day 1, the six inside E's circle among
//!     them. C is given the route (-7,0), as today, and the reason says
//!     nothing of a way.
//!
//! # Statements: where nothing is chosen
//!
//! 20. **A station of the commander's.** With no deny, and a task of the
//!     commander's own for C to operate carriers from (3,0), at preserve
//!     and with K held as in statement 5, C is given the route (3,0): the
//!     way is chosen only for a station the staff chose.
//! 21. **Home for fuel.** With C at its station (3,0), H its home port,
//!     K held and every ship at 110 t: at balanced the passage home, 81.75
//!     t times 1.2, is 98.1 t, and C stays. With the order set to preserve
//!     on day 2 the passage home needs 122.6 t, C is short, and it is
//!     routed home by the one point H. The reason says nothing of a way.
//!     (Corrected by the architect on Nov 21, 2026: as first written it
//!     took statement 11's setting, in which a force at its station is
//!     never short, since its way out was weighed with the same margin.)
//!
//! # Properties
//!
//! - **No game on file moves but scenario 116's.** Every other scenario
//!   and recorded game gives the events and the daily hashes it gave
//!   before. Scenario 116's statements all hold.
//! - **The forecast only reports.** The pulse of each hex in a route's
//!   forecast changes no hash and no event.
//! - **Nothing is drawn.** The same view gives the same way.
//! - **The staff reads its own side.** Nothing in a reason for a way
//!   names what an enemy base holds, and no reason gives a figure of
//!   danger.
//! - **A day is not much slower.** `sim-cli time` on a day in which such
//!   a route is given is measured before and after.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{Reason, Risk, StaffDecisions};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{IntentId, TaskForceId};
use sim_core::orders::{OrderKind, SpeedSetting, TaskForceOrder};
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_core::{PassageEnd, PassageForecast};
use sim_data::Scenario;
use sim_staff::station::Circle;
use sim_staff::way::{latest_pulse, weigh_hexes, Inside};
use sim_staff::{as_staffs, staffs};

// ---------------------------------------------------------------- The harness

fn path() -> PathBuf {
    common::data_dir().join("scenarios/the-way-a-force-takes/117_the_way_a_force_takes.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn hex(q: i32, r: i32) -> Hex {
    Hex { q, r }
}

fn c() -> TaskForceId {
    TaskForceId::new(1)
}

/// Adds `what` at the mark `# MARK: name`; the mark stays.
fn at(text: &str, name: &str, what: &str) -> String {
    let mark = format!("# MARK: {name}\n");
    common::replace_once(text, &mark, &format!("{what}{mark}"))
}

fn command(day: u32, body: &str) -> String {
    format!("[[command]]\nday = {day}\nside = \"allied\"\n{body}\n\n")
}

/// The risk set on intent 1 on `day`.
fn risk_on(text: &str, day: u32, level: &str) -> String {
    at(
        text,
        "commands",
        &command(
            day,
            &format!("kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{level}\""),
        ),
    )
}

fn at_risk(text: &str, level: &str) -> String {
    risk_on(text, 1, level)
}

/// C stopped at `(q, r)` instead of (16,0).
fn c_at(text: &str, q: i32, r: i32) -> String {
    common::replace_once(
        text,
        "q = 16\nr = 0\nmission = \"carrier\"",
        &format!("q = {q}\nr = {r}\nmission = \"carrier\""),
    )
}

/// C with base H for its home port.
fn home_h(text: &str) -> String {
    common::replace_once(
        text,
        "mission = \"carrier\"\n",
        "mission = \"carrier\"\nhome_port = 1\n",
    )
}

/// The deny's waters within 2 hexes of `(q, r)` instead of (0,0).
fn waters_at(text: &str, q: i32, r: i32) -> String {
    common::replace_once(
        text,
        "hex = [0, 0]\ndistance = 2",
        &format!("hex = [{q}, {r}]\ndistance = 2"),
    )
}

/// Every ship of C at `tons` instead of 1,000.
fn all_fuel(text: &str, tons: i64) -> String {
    let mut t = text.to_string();
    for id in 1..=3 {
        let name = if id == 1 { "carrier" } else { "destroyer" };
        let class = if id == 1 { 17 } else { 18 };
        let head =
            format!("name = \"Placeholder {name} {id}\"\nclass = {class}\nside = \"allied\"\n");
        t = common::replace_once(
            &t,
            &format!("{head}fuel_t = 1000"),
            &format!("{head}fuel_t = {tons}"),
        );
    }
    t
}

/// B's base E, base 2, at (9,-15).
fn base_e(text: &str) -> String {
    at(
        text,
        "bases",
        "[[base]]\nid = 2\nname = \"Placeholder base E\"\nside = \"japanese\"\nq = 9\nr = -15\nsupply_t = 0\n\n",
    )
}

/// A contact A holds at `(q, r)`, of one ship reported as a carrier, `age`
/// pulses old.
fn contact(text: &str, q: i32, r: i32, age: u32) -> String {
    at(
        text,
        "contacts",
        &format!(
            "[[contact]]\nside = \"allied\"\nq = {q}\nr = {r}\nship_count = 1\nkind = \"carrier\"\nage_pulses = {age}\n\n"
        ),
    )
}

/// The setting of the statements of a carrier contact: K at (13,-7).
fn with_k(level: &str) -> String {
    at_risk(&contact(&text(), 13, -7, 0), level)
}

/// The setting of the statements of an enemy base: E at (9,-15) and the
/// waters round (-10,0).
fn with_e(level: &str) -> String {
    at_risk(&waters_at(&base_e(&text()), -10, 0), level)
}

fn k_circle() -> Vec<Circle> {
    vec![Circle {
        centre: hex(13, -7),
        hexes: 8,
        words: String::new(),
    }]
}

fn e_circle() -> Vec<Circle> {
    vec![Circle {
        centre: hex(9, -15),
        hexes: 15,
        words: String::new(),
    }]
}

fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &common::frozen()).expect("the scenario loads")
}

/// The engine's forecast of C given `waypoints` at `speed`, from the start.
fn forecast(text: &str, waypoints: &[Hex], speed: SpeedSetting) -> PassageForecast {
    load(text)
        .world
        .side_view(Side::Allied)
        .forecast_route(c(), waypoints, Some(speed))
        .expect("C is the side's own")
}

/// The pulse the forecast enters `h` in.
fn entered(f: &PassageForecast, h: Hex) -> (u32, Pulse) {
    let at = f
        .hexes
        .iter()
        .position(|x| *x == h)
        .expect("the hex is entered");
    f.entered_in[at]
}

fn arrives(f: &PassageForecast) -> (u32, Pulse, Fixed) {
    match f.end {
        PassageEnd::Arrives {
            day,
            pulse,
            elapsed_hours,
        } => (day, pulse, elapsed_hours),
        ref other => panic!("does not arrive: {other:?}"),
    }
}

fn inside(f: &PassageForecast, circles: &[Circle]) -> Inside {
    weigh_hexes(&f.hexes, &f.entered_in, circles).1[0]
}

/// The charged miles of each leg from (16,0) through `waypoints`.
fn legs(text: &str, waypoints: &[Hex]) -> Vec<u32> {
    let s = load(text);
    let view = s.world.side_view(Side::Allied);
    let mut here = hex(16, 0);
    waypoints
        .iter()
        .map(|w| {
            let nm = view.sea_charged_nm(here, *w).expect("a leg by sea");
            here = *w;
            nm
        })
        .collect()
}

fn hours(n: i64, d: i64) -> Fixed {
    Fixed::from_ratio(n, d)
}

/// A game played with the doctrine's staffs: what the Allied staff handed in
/// each day, and the truth at the end of each day (index 0 is the start).
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
}

fn play(text: &str, days: u32) -> Played {
    let mut scenario = load(text);
    let held = staffs();
    let mut handed = BTreeMap::new();
    let mut truth = vec![scenario.world.umpire().clone()];
    for day in 1..=days {
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        handed.insert(
            day,
            scenario
                .world
                .staff_handed_in()
                .get(&Side::Allied)
                .cloned()
                .unwrap_or_default(),
        );
        truth.push(scenario.world.umpire().clone());
    }
    Played { handed, truth }
}

/// A reason's text: its calculation, its evidence and its bindings.
fn words(reason: &Reason) -> String {
    let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
    format!(
        "{} | {} | {}",
        reason.worked,
        evidence.join(" | "),
        reason.binding.join(" | ")
    )
}

impl Played {
    fn decisions(&self, day: u32) -> &StaffDecisions {
        &self.handed[&day]
    }

    fn force_orders(&self, day: u32) -> Vec<TaskForceOrder> {
        self.decisions(day)
            .orders
            .iter()
            .filter_map(|o| match &o.kind {
                OrderKind::TaskForce { target, order } if *target == c() => Some(order.clone()),
                _ => None,
            })
            .collect()
    }

    fn route_of(&self, day: u32) -> Option<Vec<Hex>> {
        self.force_orders(day).into_iter().find_map(|o| match o {
            TaskForceOrder::SetRoute { waypoints, .. } => Some(waypoints),
            _ => None,
        })
    }

    fn speed_of(&self, day: u32) -> Option<SpeedSetting> {
        self.force_orders(day).into_iter().find_map(|o| match o {
            TaskForceOrder::SetSpeed(s) => Some(s),
            _ => None,
        })
    }

    /// The reason of the route given on `day`.
    fn route_reason(&self, day: u32) -> String {
        self.decisions(day)
            .orders
            .iter()
            .find(|o| {
                matches!(
                    &o.kind,
                    OrderKind::TaskForce {
                        order: TaskForceOrder::SetRoute { .. },
                        ..
                    }
                )
            })
            .map(|o| words(&o.reason))
            .unwrap_or_default()
    }

    /// The part of the route's reason that speaks of the way.
    fn way_words(&self, day: u32) -> String {
        let r = self.route_reason(day);
        r.find("The way:")
            .map_or_else(String::new, |i| r[i..].to_string())
    }

    /// Every reason of a day as one text.
    fn said(&self, day: u32) -> String {
        let d = self.decisions(day);
        d.orders
            .iter()
            .map(|o| &o.reason)
            .chain(d.no_orders.iter())
            .chain(d.commands.iter().map(|c| &c.reason))
            .map(words)
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn position(&self, day: usize) -> Hex {
        self.truth[day].task_forces[&c()].position()
    }

    fn station(&self) -> Option<Hex> {
        self.decisions(1)
            .commands
            .iter()
            .find_map(|c| match c.kind {
                sim_core::command::CommandKind::FormIntent {
                    kind: sim_core::command::IntentKind::OperateCarriers { hex, .. },
                    ..
                } => Some(hex),
                _ => None,
            })
    }
}

const RISKS: [&str; 3] = ["preserve", "balanced", "press"];

// ---------------------------------------------------------------- The setting

#[test]
fn the_file_is_the_setting_the_doc_comment_describes() {
    let s = load(&text());
    let t = s.world.umpire();
    let force = &t.task_forces[&c()];
    assert_eq!(force.position(), hex(16, 0));
    assert_eq!(force.home_port, None);
    assert_eq!(force.ships.len(), 3);
    assert_eq!(t.bases[&sim_core::ids::BaseId::new(1)].hex, hex(10, 2));
    let view = s.world.side_view(Side::Allied);
    for ship in &force.ships {
        let class = view.ship_class(t.ships[ship].class).expect("a class");
        assert_eq!(class.cruise_speed_kn, Fixed::from_int(15));
        assert_eq!(class.max_speed_kn, Fixed::from_int(30));
        assert_eq!(class.fuel_capacity_t, Fixed::from_int(1000));
        assert_eq!(class.endurance_nm, Fixed::from_int(4000));
    }
    for (q, r) in [(-16, -16), (16, 16), (-16, 16), (16, -16)] {
        assert!(view.hex(hex(q, r)).is_some_and(|h| h.is_navigable()));
    }
}

// ------------------------------------------------- Statements: no enemy known

/// Statement 1.
#[test]
fn statement_1_one_point_as_today() {
    for level in RISKS {
        let played = play(&at_risk(&text(), level), 1);
        assert_eq!(played.station(), Some(hex(3, 0)), "{level}");
        assert_eq!(played.route_of(1), Some(vec![hex(3, 0)]), "{level}");
        let speed = played.speed_of(1);
        if level == "preserve" {
            assert_eq!(speed, None, "{level}: cruise, as it has");
            assert_eq!(
                played.truth[1].task_forces[&c()].speed,
                SpeedSetting::Cruise
            );
        } else {
            assert_eq!(speed, Some(SpeedSetting::Full), "{level}");
        }
        assert!(
            played.way_words(1).is_empty(),
            "{level}: {}",
            played.route_reason(1)
        );
    }
}

/// Statement 2.
#[test]
fn statement_2_routed_each_morning_as_today() {
    let played = play(&at_risk(&text(), "preserve"), 2);
    assert_ne!(played.position(1), hex(3, 0));
    assert_eq!(played.route_of(2), Some(vec![hex(3, 0)]));
}

// --------------------------------------- Statements: a carrier contact, preserve

/// Statement 3.
#[test]
fn statement_3_the_shortest_way_by_the_engines_clock() {
    let f = forecast(&with_k("preserve"), &[hex(3, 0)], SpeedSetting::Cruise);
    let row: Vec<Hex> = (3..=15).rev().map(|q| hex(q, 0)).collect();
    assert_eq!(f.hexes, row);
    for q in 11..=15 {
        assert_eq!(entered(&f, hex(q, 0)), (1, Pulse::Night), "({q},0)");
    }
    for q in [10, 9] {
        assert_eq!(entered(&f, hex(q, 0)), (1, Pulse::Morning), "({q},0)");
    }
    for q in [8, 7] {
        assert_eq!(entered(&f, hex(q, 0)), (1, Pulse::Afternoon), "({q},0)");
    }
    for q in 3..=6 {
        assert_eq!(entered(&f, hex(q, 0)), (2, Pulse::Night), "({q},0)");
    }
    assert_eq!(arrives(&f), (2, Pulse::Night, hours(104, 3)));
    let i = inside(&f, &k_circle());
    assert_eq!((i.by_day, i.nearest_by_day, i.by_night), (4, Some(7), 6));
}

/// Statement 4.
#[test]
fn statement_4_how_late_it_may_be() {
    assert_eq!(
        latest_pulse(Risk::Preserve, 2, Pulse::Night),
        (3, Pulse::Night)
    );
}

fn statement_5_route() -> Vec<Hex> {
    vec![hex(10, 2), hex(6, 2), hex(3, 0)]
}

/// Statement 5.
#[test]
fn statement_5_the_way_taken() {
    let t = with_k("preserve");
    let played = play(&t, 1);
    assert_eq!(played.station(), Some(hex(3, 0)));
    assert_eq!(played.route_of(1), Some(statement_5_route()));
    assert_eq!(played.speed_of(1), None, "cruise, as it has");
    assert_eq!(legs(&t, &statement_5_route()), vec![212, 160, 174]);
    let f = forecast(&t, &statement_5_route(), SpeedSetting::Cruise);
    assert_eq!(f.route_nm, Fixed::from_int(546));
    let way = [
        (15, 0),
        (14, 0),
        (13, 0),
        (12, 0),
        (11, 1),
        (10, 2),
        (9, 2),
        (8, 2),
        (7, 2),
        (6, 2),
        (6, 1),
        (6, 0),
        (5, 0),
        (4, 0),
        (3, 0),
    ];
    assert_eq!(
        f.hexes,
        way.iter().map(|(q, r)| hex(*q, *r)).collect::<Vec<_>>()
    );
    assert_eq!(arrives(&f), (2, Pulse::Morning, hours(364, 10)));
    for (h, p) in [
        ((11, 1), (1, Pulse::Night)),
        ((10, 2), (1, Pulse::Morning)),
        ((9, 2), (1, Pulse::Morning)),
        ((8, 2), (1, Pulse::Afternoon)),
        ((6, 2), (1, Pulse::Afternoon)),
        ((6, 1), (2, Pulse::Night)),
        ((3, 0), (2, Pulse::Night)),
    ] {
        assert_eq!(entered(&f, hex(h.0, h.1)), p, "{h:?}");
    }
    let i = inside(&f, &k_circle());
    assert_eq!((i.by_day, i.by_night), (0, 7));
}

/// Statement 6.
#[test]
fn statement_6_the_reason() {
    let played = play(&with_k("preserve"), 1);
    let said = played.way_words(1);
    assert!(
        said.contains("it arrives in the morning of day 2"),
        "{said}"
    );
    assert!(
        said.contains("contact 1, reported as a carrier, 0 pulse(s) old, at (13,-7)"),
        "{said}"
    );
    assert!(
        said.contains("where the shortest way, the one point (3,0), 520 nm at cruise, arrives in the night of day 2 and enters, inside the reach of contact 1, reported as a carrier, 0 pulse(s) old, at (13,-7), 4 hexes by day, the nearest 7 hexes from it, and 6 by night"),
        "{said}"
    );
    assert!(said.contains("none by day, and 7 by night"), "{said}");
    assert!(!said.contains("danger"), "{said}");
}

/// Statement 7.
#[test]
fn statement_7_kept() {
    let played = play(&with_k("preserve"), 2);
    assert_eq!(played.position(1), hex(6, 2));
    assert_eq!(played.route_of(2), None);
    assert_eq!(played.speed_of(2), None);
    assert!(played
        .said(2)
        .contains("the way is kept, so it is given no route and no speed"));
    assert_eq!(
        played.truth[1].task_forces[&c()].route.waypoints.last(),
        Some(&hex(3, 0))
    );
    let arrived = played.truth[2].task_forces[&c()].arrived_at_hours;
    assert_eq!(arrived, Some(hours(364, 10)));
    assert_eq!(played.position(2), hex(3, 0));
}

/// Statement 8.
#[test]
fn statement_8_the_search_on_the_way() {
    let played = play(&with_k("preserve"), 2);
    let said = played.said(1);
    for (q, r) in [
        (15, 0),
        (14, 0),
        (13, 0),
        (12, 0),
        (11, 1),
        (10, 2),
        (9, 2),
        (8, 2),
        (7, 2),
        (3, 0),
    ] {
        assert!(
            said.contains(&format!("the way ahead to the station (3,0) at ({q},{r})")),
            "({q},{r}): {said}"
        );
    }
    for (q, r) in [(6, 2), (6, 1), (11, 0)] {
        assert!(
            !said.contains(&format!("the way ahead to the station (3,0) at ({q},{r})")),
            "({q},{r})"
        );
    }
    assert!(said.contains("the waters to deny at (0,0)"));
    let said = played.said(2);
    for (q, r) in [(6, 1), (6, 0), (5, 0), (4, 0), (3, 0)] {
        assert!(
            said.contains(&format!("the way ahead to the station (3,0) at ({q},{r})")),
            "({q},{r}): {said}"
        );
    }
    assert!(!said.contains("the way ahead to the station (3,0) at (7,2)"));
}

/// Statement 9.
#[test]
fn statement_9_a_contact_grown_old_casts_no_circle() {
    let t = at_risk(&contact(&text(), 13, -7, 4), "preserve");
    let played = play(&t, 1);
    assert_eq!(played.route_of(1), Some(vec![hex(3, 0)]));
    assert!(played.way_words(1).is_empty());
}

// --------------------------------------------------- Statements: fuel, preserve

fn fuelled(tons: i64) -> String {
    all_fuel(&home_h(&with_k("preserve")), tons)
}

/// Statement 10.
#[test]
fn statement_10_what_each_way_takes() {
    let played = play(&fuelled(330), 1);
    let said = played.said(1);
    assert!(
        said.contains("ship:1 needs 327.375 t (136.5 t out and 81.75 t home, times 1.5)"),
        "{said}"
    );
    assert!(
        said.contains("ship:1 needs 317.625 t (130 t out and 81.75 t home, times 1.5)"),
        "{said}"
    );
    assert!(
        said.contains("home to base:1 (Placeholder base H), 327 nm at cruise"),
        "{said}"
    );
}

/// Statement 11.
#[test]
fn statement_11_in_fuel_the_way_of_statement_5() {
    assert_eq!(
        play(&fuelled(330), 1).route_of(1),
        Some(statement_5_route())
    );
}

/// Statement 12.
#[test]
fn statement_12_fuel_takes_the_clearest_way_away() {
    let t = fuelled(325);
    let played = play(&t, 1);
    let route = vec![hex(10, 1), hex(6, 1), hex(3, 0)];
    assert_eq!(played.route_of(1), Some(route.clone()));
    assert_eq!(legs(&t, &route), vec![223, 160, 144]);
    let f = forecast(&t, &route, SpeedSetting::Cruise);
    assert_eq!(f.route_nm, Fixed::from_int(527));
    assert_eq!(arrives(&f), (2, Pulse::Night, hours(527, 15)));
    let i = inside(&f, &k_circle());
    assert_eq!((i.by_day, i.nearest_by_day), (4, Some(8)));
    for q in 7..=10 {
        assert!(Pulse::Night != entered(&f, hex(q, 1)).1, "({q},1)");
    }
    let said = played.way_words(1);
    assert!(
        said.contains("ship:1 needs 320.25 t (131.75 t out and 81.75 t home, times 1.5)"),
        "{said}"
    );
    assert!(
        said.contains("4 hexes by day, the nearest 8 hexes from it"),
        "{said}"
    );
    assert!(
        said.contains("4 hexes by day, the nearest 7 hexes from it"),
        "{said}"
    );
    assert!(
        said.contains("A clearer way that arrives in time is out of fuel."),
        "{said}"
    );
}

/// Statement 13.
#[test]
fn statement_13_no_clearer_way_in_fuel() {
    let played = play(&fuelled(319), 2);
    assert_eq!(played.route_of(1), Some(vec![hex(3, 0)]));
    let said = played.way_words(1);
    assert!(
        said.contains("it lies in the enemy's reach by day"),
        "{said}"
    );
    assert!(
        said.contains("no clearer way that arrives in time is in fuel"),
        "{said}"
    );
    assert_eq!(played.route_of(2), Some(vec![hex(3, 0)]));
}

// ------------------------------------------ Statements: an enemy base beside it

/// Statement 14.
#[test]
fn statement_14_at_press_as_today() {
    let played = play(&with_e("press"), 1);
    assert_eq!(played.station(), Some(hex(-7, 0)));
    assert_eq!(played.route_of(1), Some(vec![hex(-7, 0)]));
    assert_eq!(played.speed_of(1), Some(SpeedSetting::Full));
    assert!(played.way_words(1).is_empty());
}

/// Statement 15.
#[test]
fn statement_15_at_balanced_the_shortest_way() {
    let f = forecast(&with_e("balanced"), &[hex(-7, 0)], SpeedSetting::Full);
    assert_eq!(f.hexes.len(), 23);
    for q in 7..=15 {
        assert_eq!(entered(&f, hex(q, 0)), (1, Pulse::Night), "({q},0)");
    }
    for q in 2..=6 {
        assert_eq!(entered(&f, hex(q, 0)), (1, Pulse::Morning), "({q},0)");
    }
    for q in -2..=1 {
        assert_eq!(entered(&f, hex(q, 0)), (1, Pulse::Afternoon), "({q},0)");
    }
    for q in -7..=-3 {
        assert_eq!(entered(&f, hex(q, 0)), (2, Pulse::Night), "({q},0)");
    }
    assert_eq!(arrives(&f), (2, Pulse::Night, hours(92, 3)));
    let i = inside(&f, &e_circle());
    assert_eq!((i.by_day, i.nearest_by_day, i.by_night), (9, Some(15), 7));
    assert_eq!(
        latest_pulse(Risk::Balanced, 2, Pulse::Night),
        (2, Pulse::Morning)
    );
}

fn statement_16_route() -> Vec<Hex> {
    vec![hex(6, 1), hex(-3, 1), hex(-7, 0)]
}

/// Statement 16.
#[test]
fn statement_16_at_balanced_the_way_taken() {
    let t = with_e("balanced");
    let played = play(&t, 1);
    assert_eq!(played.route_of(1), Some(statement_16_route()));
    assert_eq!(played.speed_of(1), Some(SpeedSetting::Full));
    assert_eq!(legs(&t, &statement_16_route()), vec![382, 360, 183]);
    let f = forecast(&t, &statement_16_route(), SpeedSetting::Full);
    assert_eq!(f.route_nm, Fixed::from_int(925));
    let mut way: Vec<Hex> = (7..=15).rev().map(|q| hex(q, 0)).collect();
    way.extend((-3..=6).rev().map(|q| hex(q, 1)));
    way.extend((-7..=-3).rev().map(|q| hex(q, 0)));
    assert_eq!(f.hexes, way);
    assert_eq!(entered(&f, hex(7, 0)), (1, Pulse::Night));
    assert_eq!(entered(&f, hex(6, 1)), (1, Pulse::Morning));
    assert_eq!(entered(&f, hex(-2, 1)), (1, Pulse::Afternoon));
    assert_eq!(entered(&f, hex(-3, 1)), (2, Pulse::Night));
    assert_eq!(arrives(&f), (2, Pulse::Night, hours(185, 6)));
    let i = inside(&f, &e_circle());
    assert_eq!((i.by_day, i.by_night), (0, 7));
}

/// Statement 17.
#[test]
fn statement_17_at_balanced_kept_and_reasoned() {
    let played = play(&with_e("balanced"), 2);
    let said = played.way_words(1);
    assert!(said.contains("it arrives in the night of day 2"), "{said}");
    assert!(
        said.contains("920 nm at full speed, arrives in the night of day 2"),
        "{said}"
    );
    assert!(
        said.contains(
            "base:2 (Placeholder base E) at (9,-15), 9 hexes by day, the nearest 15 hexes from it"
        ),
        "{said}"
    );
    assert!(
        said.contains("base:2 (Placeholder base E) at (9,-15), none by day"),
        "{said}"
    );
    assert_eq!(played.route_of(2), None);
    assert_eq!(played.speed_of(2), None);
}

/// Statement 18.
#[test]
fn statement_18_at_preserve() {
    let t = with_e("preserve");
    let f = forecast(&t, &[hex(-7, 0)], SpeedSetting::Cruise);
    assert_eq!(arrives(&f), (3, Pulse::Morning, hours(184, 3)));
    let by_day: Vec<Hex> = f
        .hexes
        .iter()
        .zip(&f.entered_in)
        .take(f.hexes.len() - 1)
        .filter(|(h, (_, p))| *p != Pulse::Night && h.distance(hex(9, -15)) <= 15)
        .map(|(h, _)| *h)
        .collect();
    let mut want = vec![hex(9, 0), hex(8, 0), hex(7, 0)];
    want.extend((-2..=1).rev().map(|q| hex(q, 0)));
    assert_eq!(by_day, want);
    let played = play(&t, 1);
    let route = vec![hex(9, 1), hex(-3, 1), hex(-7, 0)];
    assert_eq!(played.route_of(1), Some(route.clone()));
    assert_eq!(played.speed_of(1), None, "cruise, as it has");
    assert_eq!(legs(&t, &route), vec![262, 480, 183]);
    let f = forecast(&t, &route, SpeedSetting::Cruise);
    assert_eq!(f.route_nm, Fixed::from_int(925));
    assert_eq!(arrives(&f), (3, Pulse::Morning, hours(185, 3)));
    assert_eq!(inside(&f, &e_circle()).by_day, 0);
}

/// Statement 19.
#[test]
fn statement_19_in_the_dark_the_shortest_way() {
    let t = c_at(&with_e("balanced"), 0, 0);
    let played = play(&t, 1);
    assert_eq!(played.station(), Some(hex(-7, 0)));
    assert_eq!(played.route_of(1), Some(vec![hex(-7, 0)]));
    assert!(played.way_words(1).is_empty(), "{}", played.route_reason(1));
    let f = forecast(&t, &[hex(-7, 0)], SpeedSetting::Full);
    assert!(f.entered_in.iter().all(|p| *p == (1, Pulse::Night)));
    let i = inside(&f, &e_circle());
    assert_eq!((i.by_day, i.by_night), (0, 6));
}

// --------------------------------------------- Statements: nothing is chosen

/// Statement 20.
#[test]
fn statement_20_a_station_of_the_commanders() {
    let t = common::replace_once(
        &contact(&text(), 13, -7, 0),
        "intent_kind = \"deny\"\nhex = [0, 0]\ndistance = 2",
        "intent_kind = \"operate_carriers\"\ntask_force = 1\nhex = [3, 0]",
    );
    let played = play(&at_risk(&t, "preserve"), 1);
    assert_eq!(played.station(), None);
    assert_eq!(played.route_of(1), Some(vec![hex(3, 0)]));
    assert!(played.way_words(1).is_empty());
}

/// Statement 21. C waits at its station (3,0) with 110 t a ship, in fuel at
/// balanced (81.75 t home, times 1.2, is 98.1 t); on day 2 the order is set
/// to preserve, at which the passage home needs 122.625 t, so it is short.
#[test]
fn statement_21_home_for_fuel() {
    let t = all_fuel(&home_h(&c_at(&contact(&text(), 13, -7, 0), 3, 0)), 110);
    let t = risk_on(&at_risk(&t, "balanced"), 2, "preserve");
    let played = play(&t, 2);
    assert_eq!(played.station(), Some(hex(3, 0)));
    assert_eq!(played.route_of(1), None);
    assert_eq!(played.route_of(2), Some(vec![hex(10, 2)]));
    assert!(played.way_words(2).is_empty());
}

// ---------------------------------------------------------------- Properties

/// The forecast's pulses are one for each hex entered, and a game played
/// twice gives the same hashes: the report changes nothing.
#[test]
fn property_the_forecast_only_reports() {
    let t = with_k("preserve");
    let f = forecast(&t, &statement_5_route(), SpeedSetting::Cruise);
    assert_eq!(f.entered_in.len(), f.hexes.len());
    let a = play(&t, 2);
    let b = play(&t, 2);
    for d in 0..=2 {
        assert_eq!(a.truth[d], b.truth[d]);
    }
}

/// The way draws no random number: the same view gives the same way,
/// whatever the seed.
#[test]
fn property_nothing_is_drawn() {
    let t = with_k("preserve");
    let a = play(&t, 1);
    let b = play(&common::replace_once(&t, "seed = 1942", "seed = 7"), 1);
    assert_eq!(a.route_of(1), b.route_of(1));
    assert_eq!(a.route_reason(1), b.route_reason(1));
}

/// A reason for a way names no holding of an enemy base and gives no figure
/// of danger.
#[test]
fn property_the_staff_reads_its_own_side() {
    for t in [
        with_e("balanced"),
        with_e("preserve"),
        with_k("preserve"),
        fuelled(319),
    ] {
        let played = play(&t, 1);
        let said = played.way_words(1);
        assert!(!said.is_empty());
        assert!(!said.contains("danger"), "{said}");
        for word in ["supply", "aircraft", "fuel_t"] {
            assert!(!said.contains(word), "{word}: {said}");
        }
    }
}

#[test]
fn the_intent_is_the_deny() {
    let played = play(&with_k("preserve"), 1);
    assert!(played
        .decisions(1)
        .commands
        .iter()
        .any(|c| matches!(c.kind, sim_core::command::CommandKind::FormIntent { parent, .. } if parent == IntentId::new(1))));
}
