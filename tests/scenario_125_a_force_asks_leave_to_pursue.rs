//! Scenario 125: a force asks leave to go after what it has found. A
//! carrier at her hex holds a contact of a carrier beyond a night's
//! steaming and her bombers' reach. The staff asks leave to pursue and the
//! clock stops in the evening, before the day is played. On the
//! commander's yes she sails that night, stays while the contact is in
//! reach, and goes back when it is gone.
//!
//! Proves decision 0214 against a hand calculation. It stands on scenario
//! 58's for the closing rule and on scenario 55's for the orders to the
//! groups, and uses scenario 58's map, ship, groups and figures: the
//! frozen step 1 figures with scenario 53's table of aircraft for each
//! ship; no upkeep, wear, fatigue or supply figures. The data is
//! placeholder data. There is no enemy unit: nothing is struck and no die
//! is rolled.
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
//! As scenario 58: a hex is 40 nm; a night pulse of 12 hours and two
//! daylight pulses of 6; the placeholder carrier makes 16 kn at cruise and
//! 30 kn at most; the placeholder dive bomber has a strike radius of 250
//! nm; `strike_contact_max_age_pulses` 0; a contact gains a pulse of age
//! at the end of each pulse and is dropped at 5; the closing rule takes a
//! contact 3 pulses old or less. `pursuit_nights` 2.
//!
//! A night at full speed is 360 nm, so the closing rule as it stands takes
//! a contact within 360 and 250, 610 nm. With the bound lifted the night
//! is taken as two, 720 nm, and the test is 720 and 250, 970 nm.
//!
//! ## Inputs
//!
//! Scenario 58's, with these differences. The game is 4 days. The Allied
//! side starts holding one contact: 1 ship, a carrier, at (16,0), age 0,
//! 640 nm from the force. The Allied side's commands of day 1: intent 1,
//! operate task force 1 from (0,0); its risk, balanced; leave to prepare
//! and leave to launch both given; leave to pursue, ask.
//!
//! ## Day 1, the asking
//!
//! The rule as it stands: the contact is fresh and a carrier, which
//! balanced closes on, and 640 nm away against 610. It is too far, she has
//! nothing in reach, and she closes on nothing. So a find is looked for.
//! Worked again with the night as two: 640 is within 970. The contact is
//! the find. Along the row, (9,0) is 280 nm from the contact and (10,0) is
//! 240: she would close to (10,0), 400 nm from her, 13 hours and a third
//! at 30 kn (asserted as 400 divided by 30 in the game's own arithmetic).
//!
//! The staff hands in the day's orders as the rule as it stands gives
//! them, with no route and no speed order to the force (scenario 58's
//! property "Too far" at balanced, order for order), and one notice: task
//! force 1 asks leave to pursue; contact 1, a carrier, age 0, reported at
//! (16,0); to (10,0); the hours above; the order left, intent 1. The
//! standing interrupt for it fires, its subject task force 1.
//!
//! A seat that runs time is stopped here: the answer says it stopped
//! before day 1, for that interrupt, and no pulse of day 1 has been
//! resolved (the day is 1, the hour 0, the force at (0,0)).
//!
//! ## Day 1, the answer
//!
//! The commander answers yes for task force 1 and contact 1 at that stop.
//! It is taken in at this same dusk, before the staff is asked. The staff
//! now gives the force what scenario 58 gives at balanced when it closes,
//! order for order, with the route to (10,0): full speed and that route
//! among them. The reason of the route says it is a pursuit, by the
//! commander's leave, and that intent 1 waits; the event of the answer is
//! among the route order's causes. There is no notice of asking and none
//! that the force pursues. The side's leaves by answer: task force 1.
//!
//! She sails at the start of the night, is at (9,0) at hour 12, reaches
//! (10,0) at hour 13 and a third, in the morning, and stops. Group 3
//! launches nothing on any day: the contact is never 0 pulses old in a
//! daylight pulse. At the end of day 1 the contact is 3 pulses old.
//!
//! ## Day 2
//!
//! The contact is 3 pulses old, fresh, and 240 nm from (10,0): it is in
//! reach, and she stays. No order to the force, no notice. The leave
//! stands, since she has a contact. The contact is 4 pulses old at the end
//! of the night and is dropped at the end of the morning, at 5.
//!
//! ## Day 3
//!
//! The side holds no contact. She has neither a find nor a contact to
//! close on: the leave ends, and the staff hands in the notice that task
//! force 1's pursuit is ended. She goes back as scenario 58's day 2: two
//! orders, the route to (0,0), 400 nm, at the full speed she has, and
//! group 2's arc, the whole circle. She sails at hour 48 and arrives at
//! hour 61 and a third. The side's leaves by answer: none.
//!
//! ## Day 4
//!
//! She is at her hex. No order and no notice.
//!
//! ## At the end
//!
//! The force is at (0,0), stopped, having steamed 800 nm at 30 kn. Her
//! fuel is asserted as scenario 58's is: she burnt what a force of the
//! same ship ordered over the same two passages at full speed burns. The game as answered records
//! one notice of this rule: the pursuit ended, on day 3. The asking is not
//! recorded in it: it was shown at the stop as what the staff would hand
//! in, and the answer came before the staff was asked. A game not answered
//! records the asking (property "Not answered").
//!
//! # Properties
//!
//! - **Not answered.** With the interrupt switched off and no answer: day
//!   1 has the asking and no route. At dusk of day 2 the contact is 3
//!   pulses old and still 640 nm away: the find stands and the staff hands
//!   in the asking again. On day 3 there is no contact and no notice. She
//!   never leaves (0,0). The interrupt switched off gave no leave.
//! - **Refused.** With a no at the stop: no route. At dusk of day 2 the
//!   find is not asked about again, the refusal being kept for contact 1.
//!   She never leaves (0,0).
//! - **Go.** With the leave to pursue at go: no asking, no interrupt and
//!   no stop; a notice that task force 1 pursues, with the same contact,
//!   hex and hours; the same orders and the same passage as the game as
//!   answered; the reason says the leave was taken. On day 3 the notice
//!   that the pursuit is ended.
//! - **Stay.** With the leave at stay: no notice on any day, and every
//!   order of every day is what the staff gave before this rule.
//! - **The default.** With no leave to pursue stated: where launching
//!   waits for the commander by default it is ask, and where the leaves
//!   are given because there is no commander it is go.
//! - **Within a night.** With the contact at (12,0), 480 nm: she closes as
//!   scenario 58 at balanced, to (6,0), with no notice at ask, go or stay.
//! - **Beyond two nights.** With the contact at (25,0), 1000 nm against
//!   970: no find, no notice, at ask or go.
//! - **A kind the risk does not close on.** With the contact a cruiser at
//!   (16,0): at balanced there is no find. At press, with ask, there is,
//!   and the asking names it a cruiser.
//! - **Outside the area.** With an area limit on intent 1 that does not
//!   hold (16,0): no find, at ask or go.
//! - **Something in reach.** With a second contact, a carrier at (5,0),
//!   200 nm: she has a contact in reach, the rule as it stands stays, and
//!   no find is looked for.
//! - **The waters first:** withdrawn by the architect on Nov 22, 2026; the
//!   closing rule prefers the waters only among contacts within a night, so no
//!   such find arises.
//! - **Seen again under a new number.** In the game as answered, with a
//!   new contact handed to the side at dusk of day 3, contact 2, a carrier
//!   at (26,0), 640 nm from (10,0): the leave stands without a new asking
//!   and she closes to (20,0). With that contact at (22,0), 480 nm, she
//!   closes to (16,0) by the rule as it stands. (If a test cannot hand a
//!   side a contact after the start, the build says so and this comes back
//!   to the architect.)
//! - **A force covering a landing** has no find, and **a held force** has
//!   none and is left alone.
//! - **With command levels on,** the asking is handed in only where the
//!   commander's own picture holds the contact.
//! - The same start gives the same orders and notices; two worlds that
//!   differ only in what the Allied side cannot see give the same.
//! - Every staffed scenario built before this one, with the leave at stay
//!   for every order, gives the orders it gave.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::choices::Moment;
use sim_core::command::{
    Addressee, CommandKind, Maker, NoticeKind, PursuitNotice, Reason, StaffDecisions, StaffNotice,
    StaffOrder, StandingKind, StandingSubject,
};
use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, IntentId, ShipId, TaskForceId};
use sim_core::orders::{
    AirGroupOrder, AirMission, OrderKind, Readiness, SpeedSetting, Stance, StragglerInstruction,
    TaskForceOrder,
};
use sim_core::refdata::ShipKind;
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const CONTACT_MARKER: &str = "# The contact";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/125_a_force_asks_leave_to_pursue.toml")
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

fn group(raw: u32) -> AirGroupId {
    AirGroupId::new(raw)
}

fn intent(raw: u32) -> IntentId {
    IntentId::new(raw)
}

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn hex(q: i32, r: i32) -> Hex {
    Hex { q, r }
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// A command table of the Allied side.
fn command_table(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
}

fn with(extra: &str) -> String {
    format!("{}{}", text(), extra)
}

/// The scenario's own figures: scenario 53's table of aircraft for each ship,
/// added after the frozen file's empty `[strike_aircraft_per_ship]` table, and
/// `pursuit_nights` of `nights` in place of the frozen file's 1.
fn figures_with(nights: u32) -> sim_data::LoadOptions {
    common::altered_reference(&format!("s125n{nights}"), |dir| {
        common::edit_file(dir, "air_combat.toml", |text| {
            let text = common::replace_once(
                &text,
                "pursuit_nights = 1",
                &format!("pursuit_nights = {nights}"),
            );
            format!("{text}\ncruiser = 12\ndestroyer = 6\ntransport = 6\nauxiliary = 4\n")
        })
    })
}

fn figures() -> sim_data::LoadOptions {
    figures_with(2)
}

/// A game played to its end with the doctrine's staffs: what the Allied staff
/// handed in each day, the truth at the end of each day (index 0 is the
/// start), the log and the side's command state at the end.
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    state: sim_core::command::CommandState,
    /// The side's leaves by answer at the end of each day (index 0 is the
    /// start).
    answered: Vec<Vec<TaskForceId>>,
}

fn play(text: &str) -> Played {
    play_on(text, &figures())
}

fn play_on(text: &str, figures: &sim_data::LoadOptions) -> Played {
    let mut scenario: Scenario =
        common::from_text_with(text, &path(), figures).expect("the scenario loads");
    let held = staffs();
    let mut handed = BTreeMap::new();
    let mut truth = vec![scenario.world.umpire().clone()];
    let mut answered = vec![scenario
        .world
        .command_state(Side::Allied)
        .leaves_by_answer()
        .collect()];
    loop {
        let day = scenario.world.day() + 1;
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
        answered.push(
            scenario
                .world
                .command_state(Side::Allied)
                .leaves_by_answer()
                .collect(),
        );
    }
    Played {
        handed,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        state: scenario.world.command_state(Side::Allied).clone(),
        answered,
    }
}

fn mission(m: AirMission) -> AirGroupOrder {
    AirGroupOrder::SetAirMission(m)
}

fn sizing(on: bool) -> AirGroupOrder {
    AirGroupOrder::SetStrikeSizing { on }
}

fn arc(from_deg: i32, to_deg: i32) -> AirGroupOrder {
    AirGroupOrder::SetSearchArc {
        from_deg,
        to_deg,
        range_nm: n(300),
    }
}

fn share(s: &str) -> AirGroupOrder {
    AirGroupOrder::SetPatrolShare { share: fx(s) }
}

const STRIKE: AirMission = AirMission::NavalStrike;
const CAP: AirMission = AirMission::CombatAirPatrol;
const SEARCH: AirMission = AirMission::Search;

/// An order of the staff: to a force or to an air group.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Force(u32, TaskForceOrder),
    Air(u32, AirGroupOrder),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce { target, order } => Given::Force(target.raw(), order.clone()),
        OrderKind::AirGroup { target, order } => Given::Air(target.raw(), order.clone()),
        other => panic!("the doctrine gave an order of another kind: {other:?}"),
    }
}

fn rules(stance: Stance, break_off: &str) -> TaskForceOrder {
    TaskForceOrder::SetRulesOfEngagement {
        stance,
        break_off: fx(break_off),
        stragglers: StragglerInstruction::Keep,
        keep_share: Fixed::ZERO,
    }
}

const BATTLE: TaskForceOrder = TaskForceOrder::SetReadiness(Readiness::Battle);

fn route(waypoints: &[(i32, i32)]) -> TaskForceOrder {
    TaskForceOrder::SetRoute {
        waypoints: waypoints.iter().map(|(q, r)| hex(*q, *r)).collect(),
        act_at: Default::default(),
        stand_at: Default::default(),
    }
}

fn full() -> Given {
    Given::Force(1, TaskForceOrder::SetSpeed(SpeedSetting::Full))
}

fn is_speed(o: &Given) -> bool {
    matches!(o, Given::Force(_, TaskForceOrder::SetSpeed(_)))
}

fn is_route(o: &Given) -> bool {
    matches!(o, Given::Force(_, TaskForceOrder::SetRoute { .. }))
}

impl Played {
    fn decisions(&self, day: u32) -> &StaffDecisions {
        &self.handed[&day]
    }

    fn orders(&self, day: u32) -> Vec<Given> {
        self.decisions(day).orders.iter().map(given).collect()
    }

    fn orders_to_force(&self, day: u32, raw: u32) -> Vec<TaskForceOrder> {
        self.orders(day)
            .into_iter()
            .filter_map(|g| match g {
                Given::Force(f, o) if f == raw => Some(o),
                _ => None,
            })
            .collect()
    }

    fn all_orders_given(&self) -> Vec<(u32, Vec<Given>)> {
        self.handed
            .keys()
            .map(|day| (*day, self.orders(*day)))
            .collect()
    }

    fn all_orders(&self) -> Vec<(u32, &StaffOrder)> {
        self.handed
            .iter()
            .flat_map(|(day, d)| d.orders.iter().map(move |o| (*day, o)))
            .collect()
    }

    fn notices(&self, day: u32) -> &[StaffNotice] {
        &self.decisions(day).notices
    }

    fn notice_kinds(&self, day: u32) -> Vec<NoticeKind> {
        self.notices(day).iter().map(|n| n.kind).collect()
    }

    fn no_notice_on_any_day(&self) -> bool {
        self.handed.values().all(|d| d.notices.is_empty())
    }

    fn all(&self, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events.iter().filter(|e| pick(&e.kind)).collect()
    }

    fn nothing_refused(&self) {
        let refused = self.all(|k| {
            matches!(
                k,
                EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
            )
        });
        assert!(refused.is_empty(), "something was refused: {refused:?}");
    }

    fn all_launches(&self) -> Vec<(u32, Hex, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::StrikeLaunched {
                    air_group,
                    hex,
                    aircraft,
                    ..
                } => Some((air_group.raw(), *hex, *aircraft)),
                _ => None,
            })
            .collect()
    }

    fn force_at_end(&self) -> &sim_core::state::TaskForce {
        &self.truth.last().expect("a state").task_forces[&force(1)]
    }

    /// The force's position at the end of each day (index 0 is the start).
    fn positions(&self) -> Vec<Hex> {
        self.truth
            .iter()
            .map(|t| t.task_forces[&force(1)].position())
            .collect()
    }

    fn never_leaves_its_hex(&self) {
        for (day, at) in self.positions().into_iter().enumerate() {
            assert_eq!(at, hex(0, 0), "end of day {day}");
        }
    }

    fn arrivals(&self) -> Vec<(Hex, Fixed)> {
        arrivals_in(&self.events)
    }

    fn interrupts_fired(&self) -> Vec<&Event> {
        self.all(|k| {
            matches!(
                k,
                EventKind::InterruptFired {
                    interrupt: sim_core::command::InterruptRef::Standing { .. },
                    ..
                }
            )
        })
    }
}

fn arrivals_in(events: &[Event]) -> Vec<(Hex, Fixed)> {
    events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::TaskForceArrived {
                hex, elapsed_hours, ..
            } => Some((*hex, *elapsed_hours)),
            _ => None,
        })
        .collect()
}

fn level(reason: &Reason) -> Option<Addressee> {
    reason.serves.level
}

/// A reason's text: its calculation and its evidence.
fn said(reason: &Reason) -> String {
    let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
    format!("{} | {}", reason.worked, evidence.join(" | "))
}

/// The reason of the order of a day that is `wanted`.
fn reason_of<'a>(played: &'a Played, day: u32, wanted: &Given) -> &'a Reason {
    &played
        .decisions(day)
        .orders
        .iter()
        .find(|o| &given(o) == wanted)
        .unwrap_or_else(|| panic!("day {day} has no order {wanted:?}"))
        .reason
}

/// Where the force is at some moments of the game: "1 Morning" is the start
/// of the morning of day 1.
fn force_by_moment(text: &str) -> BTreeMap<String, Hex> {
    let mut held: BTreeMap<String, Hex> = BTreeMap::new();
    let mut scenario: Scenario =
        common::from_text_with(text, &path(), &figures()).expect("the scenario loads");
    let staffs = staffs();
    loop {
        let summary = scenario
            .next_day_watched(as_staffs(&staffs), &mut |world, moment| {
                if let Moment::PulseBegins { day, pulse } = moment {
                    held.insert(
                        format!("{day} {pulse:?}"),
                        world.umpire().task_forces[&force(1)].position(),
                    );
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
    }
    held
}

// ------------------------------------------------------------ The variants

const YES: &str = "kind = \"answer_pursuit\"\ntask_force = 1\ncontact = 1\nallow = true";
const NO: &str = "kind = \"answer_pursuit\"\ntask_force = 1\ncontact = 1\nallow = false";
const INTERRUPT_OFF: &str =
    "kind = \"set_interrupt\"\nstanding = \"asks_leave_to_pursue\"\non = false";

fn pursuit_leave(text: &str, leave: &str) -> String {
    changed(
        text,
        "intent = 1\nleave = \"ask\"",
        &format!("intent = 1\nleave = \"{leave}\""),
    )
}

fn without_pursuit_setting(text: &str) -> String {
    changed(
        text,
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_pursuit\"\nintent = 1\nleave = \"ask\"\n",
        "",
    )
}

fn at_risk(text: &str, risk: &str) -> String {
    changed(text, "risk = \"balanced\"", &format!("risk = \"{risk}\""))
}

fn contact_at(text: &str, q: i32) -> String {
    changed(text, "q = 16\nr = 0", &format!("q = {q}\nr = 0"))
}

fn cruiser_contact(text: &str) -> String {
    changed(text, "kind = \"carrier\"", "kind = \"cruiser\"")
}

/// The scenario with a second contact entered after the first.
fn with_second_contact(text: &str, q: i32, kind: &str) -> String {
    let at = text.find(COMMANDS_MARKER).expect("the commands");
    format!(
        "{}[[contact]]\nside = \"allied\"\nq = {q}\nr = 0\nship_count = 1\nkind = \"{kind}\"\nage_pulses = 0\n\n{}",
        &text[..at],
        &text[at..]
    )
}

fn levels_on(text: &str) -> String {
    changed(
        text,
        "placeholder = true",
        "command_levels = true\nplaceholder = true\nreports = \"../reference/reports_placeholder_scenario_62.toml\"",
    )
}

fn answered_text() -> String {
    with(&command_table(1, YES))
}

fn balanced_air_orders() -> Vec<Given> {
    vec![
        Given::Air(1, mission(CAP)),
        Given::Air(1, share("0.5")),
        Given::Air(2, mission(SEARCH)),
        Given::Air(2, arc(50, 120)),
        Given::Air(3, mission(STRIKE)),
        Given::Air(3, sizing(true)),
    ]
}

/// Scenario 58's "Too far" at balanced: no route, no speed order.
fn too_far_orders() -> Vec<Given> {
    [
        vec![
            Given::Force(1, BATTLE),
            Given::Force(1, rules(Stance::Avoid, "0.5")),
        ],
        balanced_air_orders(),
    ]
    .concat()
}

/// Scenario 58's balanced closing orders, with the route to `(q, 0)`.
fn closing_orders(q: i32) -> Vec<Given> {
    [
        vec![
            Given::Force(1, BATTLE),
            Given::Force(1, rules(Stance::Avoid, "0.5")),
            full(),
            Given::Force(1, route(&[(q, 0)])),
        ],
        balanced_air_orders(),
    ]
    .concat()
}

fn going_back_orders() -> Vec<Given> {
    vec![Given::Force(1, route(&[(0, 0)])), Given::Air(2, arc(0, 0))]
}

fn hours() -> Fixed {
    n(400) / n(30)
}

fn asking_notice(day_age: u32, kind: ShipKind) -> StaffNotice {
    let find = sim_core::command::PursuitFind {
        contact: 1,
        kind,
        age: day_age,
        reported_at: hex(16, 0),
        to: hex(10, 0),
        hours: Some(hours()),
    };
    StaffNotice {
        kind: NoticeKind::AsksLeaveToPursue,
        plan: sim_core::command::PlanId::new(1),
        run: None,
        rests_on: sim_core::command::NoticeBasis::NoOrder(0),
        pursuit: Some(PursuitNotice {
            force: force(1),
            intent: intent(1),
            find: Some(find),
        }),
    }
}

/// The notice's content without the index of the reason it rests on.
fn content(n: &StaffNotice) -> (NoticeKind, Option<PursuitNotice>) {
    (n.kind, n.pursuit)
}

fn find_of(n: &StaffNotice) -> sim_core::command::PursuitFind {
    n.pursuit.expect("a pursuit notice").find.expect("a find")
}

// ----------------------------------------------------------- A game on a seat

struct SeatGame {
    stop: Value,
    end: Value,
    handed: BTreeMap<u32, StaffDecisions>,
    events: Vec<Event>,
    hash_before: u64,
    hash_after_stop: u64,
    day_after_stop: u32,
}

fn session_for(text: &str) -> Session {
    let mut scenario: Scenario =
        common::from_text_with(text, &path(), &figures()).expect("the scenario loads");
    let commands = scenario.scripted_commands().clone();
    scenario.replace_commands(BTreeMap::new());
    let title = scenario.name.clone();
    Session::new(
        scenario.world,
        &title,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
    .with_staffs(staffs())
    .with_commands(commands)
}

fn call(session: &mut Session, tool: &str, args: Value) -> Value {
    session
        .call(Side::Allied, tool, &args)
        .unwrap_or_else(|e| panic!("{tool}: {e}"))
}

/// Run to the first stop, answer (or not), run to the end.
fn seat_game(text: &str, answer: Option<bool>) -> SeatGame {
    let mut session = session_for(text);
    let hash_before = session.state_hash();
    let stop = call(&mut session, "run", json!({"days": 4}));
    let hash_after_stop = session.state_hash();
    let day_after_stop = session.day();
    if let Some(allow) = answer {
        let q = call(
            &mut session,
            "answer_pursuit",
            json!({"task_force": 1, "contact": 1, "allow": allow}),
        );
        assert_eq!(q["status"], "queued", "{q}");
    }
    let end = if session.is_over() {
        stop.clone()
    } else {
        call(&mut session, "run", json!({"days": 4}))
    };
    let handed = session
        .staff_handed_in()
        .iter()
        .filter_map(|(d, m)| m.get(&Side::Allied).map(|x| (*d, x.clone())))
        .collect();
    let events = session.into_world().events().iter().cloned().collect();
    SeatGame {
        stop,
        end,
        handed,
        events,
        hash_before,
        hash_after_stop,
        day_after_stop,
    }
}

fn counts(a: &Value) -> (u64, u64, u64) {
    let d = &a["days_resolved"];
    (
        d["first"].as_u64().unwrap(),
        d["last"].as_u64().unwrap(),
        d["count"].as_u64().unwrap(),
    )
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_commands_of_the_file_are_the_ones_these_tests_vary() {
    let t = text();
    let at = t.find(COMMANDS_MARKER).expect("the marker");
    let c = &t[at..];
    assert_eq!(c.matches("[[command]]").count(), 5);
    assert!(c.contains("intent_kind = \"operate_carriers\"\ntask_force = 1\nhex = [0, 0]\n"));
    assert!(c.contains("limit = \"risk\"\nintent = 1\nrisk = \"balanced\"\n"));
    assert!(c.contains("kind = \"set_pursuit\"\nintent = 1\nleave = \"ask\"\n"));
    assert!(!c.contains("given = false"));
    assert!(t.contains(CONTACT_MARKER));
    assert_eq!(t.matches("[[contact]]").count(), 1);
    assert!(t.contains("days = 4"));
}

#[test]
fn the_figures_are_those_of_the_hand_calculation() {
    assert_eq!(hex(0, 0).distance(hex(16, 0)) * 40, 640);
    assert_eq!(hex(9, 0).distance(hex(16, 0)) * 40, 280);
    assert_eq!(hex(10, 0).distance(hex(16, 0)) * 40, 240);
    assert_eq!(hex(0, 0).distance(hex(10, 0)) * 40, 400);
    let nights = std::fs::read_to_string(common::frozen_root().join("air_combat.toml"))
        .expect("the frozen figures read");
    assert!(nights.contains("pursuit_nights = 1"));
}

#[test]
fn day_1_the_asking_the_orders_are_scenario_58s_too_far_and_one_notice_asks() {
    let played = play(&text());
    played.nothing_refused();
    assert_eq!(played.orders(1), too_far_orders());
    assert!(!played.orders(1).iter().any(is_route));
    assert!(!played.orders(1).iter().any(is_speed));
    let notices = played.notices(1);
    assert_eq!(notices.len(), 1, "{notices:?}");
    let want = asking_notice(0, ShipKind::Carrier);
    assert_eq!(content(&notices[0]), content(&want));
    let find = find_of(&notices[0]);
    assert_eq!(find.hours, Some(n(400) / n(30)));
    // The standing interrupt fires, its subject task force 1.
    let fired = played.interrupts_fired();
    assert_eq!(fired.len(), 1, "{fired:?}");
    assert_eq!(fired[0].day, 1);
    match &fired[0].kind {
        EventKind::InterruptFired {
            side, interrupt, ..
        } => {
            assert_eq!(*side, Side::Allied);
            assert_eq!(
                *interrupt,
                sim_core::command::InterruptRef::Standing {
                    kind: StandingKind::AsksLeaveToPursue,
                    subject: StandingSubject::TaskForce(force(1)),
                }
            );
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn the_stop_a_run_stops_before_day_1_with_nothing_resolved() {
    let game = seat_game(&text(), None);
    let a = &game.stop;
    assert_eq!(a["stopped"], "asks_leave", "{a}");
    assert_eq!(counts(a), (1, 0, 0), "{a}");
    assert_eq!(a["day"], 0);
    let asking = a["asking"].as_array().expect("an asking");
    assert_eq!(asking.len(), 1, "{a}");
    let one = &asking[0];
    assert_eq!(one["task_force"], 1);
    assert_eq!(one["contact"], 1);
    assert_eq!(one["intent"], 1);
    assert_eq!(one["kind"], "carrier");
    assert_eq!(one["age_pulses"], 0);
    assert_eq!(one["reported_at"], json!({"q": 16, "r": 0}));
    assert_eq!(one["to"], json!({"q": 10, "r": 0}));
    assert_eq!(one["hours"], json!(hours().to_string()));
    // No pulse of day 1 was resolved: day 0, the world as it was.
    assert_eq!(game.day_after_stop, 0);
    assert_eq!(game.hash_before, game.hash_after_stop);
}

#[test]
fn the_answer_through_a_seat_gives_the_orders_the_passages_and_the_one_notice() {
    let game = seat_game(&text(), Some(true));
    assert_eq!(game.stop["stopped"], "asks_leave");
    assert_eq!(game.end["stopped"], "game_over", "{}", game.end);
    assert_eq!(counts(&game.end), (1, 4, 4), "{}", game.end);
    assert_eq!(game.end["game_over"], true);
    assert_eq!(game.end["fired"], json!([]));
    // The same staff decisions as the game with the answer scripted.
    let twin = play(&answered_text());
    assert_eq!(game.handed, twin.handed);
    let orders = |day: u32| -> Vec<Given> { game.handed[&day].orders.iter().map(given).collect() };
    assert_eq!(orders(1), closing_orders(10));
    assert!(orders(2).iter().all(|o| !matches!(o, Given::Force(..))));
    assert_eq!(orders(3), going_back_orders());
    assert!(orders(4).is_empty());
    // The route's reason and its causes.
    let route_reason = game.handed[&1]
        .orders
        .iter()
        .find(|o| is_route(&given(o)))
        .expect("a route")
        .reason
        .clone();
    let words = said(&route_reason);
    assert!(words.contains("pursues"), "{words}");
    assert!(words.contains("the commander's leave"), "{words}");
    assert!(words.contains("intent:1 waits"), "{words}");
    assert_eq!(route_reason.answer_of, Some(force(1)));
    let answers: Vec<&Event> = game
        .events
        .iter()
        .filter(|e| {
            matches!(
                &e.kind,
                EventKind::CommandTaken {
                    command: CommandKind::AnswerPursuit { .. },
                    ..
                }
            )
        })
        .collect();
    assert_eq!(answers.len(), 1);
    let route_order = game
        .events
        .iter()
        .find_map(|e| match &e.kind {
            EventKind::StaffReason {
                order: Some(o),
                reason,
                ..
            } if reason.worked.contains("is routed to (10,0)") => Some(*o),
            _ => None,
        })
        .expect("the route order's reason is recorded");
    let cited = game
        .events
        .iter()
        .filter(|e| match &e.kind {
            EventKind::StaffReason { order, .. } => *order == Some(route_order),
            EventKind::OrderAccepted { order, .. } => *order == route_order,
            _ => false,
        })
        .any(|e| e.causes.contains(&Cause::Event(answers[0].id)));
    assert!(
        cited,
        "the answer's event is not among the route order's causes"
    );
    // No asking and no "pursues"; one notice: the pursuit ended, on day 3.
    let notices: Vec<(u32, NoticeKind)> = game
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::StaffNotice { side, kind, .. }
                if *side == Side::Allied
                    && matches!(
                        kind,
                        NoticeKind::AsksLeaveToPursue
                            | NoticeKind::Pursues
                            | NoticeKind::PursuitEnded
                    ) =>
            {
                Some((e.day, *kind))
            }
            _ => None,
        })
        .collect();
    assert_eq!(notices, vec![(3, NoticeKind::PursuitEnded)]);
    assert!(game.handed[&3]
        .notices
        .iter()
        .all(|x| x.kind == NoticeKind::PursuitEnded));
}

#[test]
fn the_answered_game_day_by_day() {
    let t = answered_text();
    let played = play(&t);
    played.nothing_refused();
    // Day 1.
    assert_eq!(played.orders(1), closing_orders(10));
    assert_eq!(played.orders(1).len(), 10);
    assert!(played.notices(1).is_empty());
    assert_eq!(played.answered[1], vec![force(1)]);
    assert!(played.interrupts_fired().is_empty());
    // Day 2: no order to the force, no notice; the leave stands.
    assert!(played.orders_to_force(2, 1).is_empty());
    assert!(played.notices(2).is_empty());
    assert_eq!(played.answered[2], vec![force(1)]);
    // Day 3: the pursuit is ended and she goes back.
    assert_eq!(played.notice_kinds(3), vec![NoticeKind::PursuitEnded]);
    let ended = played.notices(3)[0].pursuit.expect("a pursuit notice");
    assert_eq!(ended.force, force(1));
    assert_eq!(ended.find, None);
    assert_eq!(played.orders(3), going_back_orders());
    assert_eq!(played.orders(3).len(), 2);
    assert!(played.answered[3].is_empty());
    // Day 4: nothing.
    assert!(played.orders_to_force(4, 1).is_empty());
    assert!(played.notices(4).is_empty());
    // Passages.
    assert_eq!(
        played.arrivals(),
        vec![
            (hex(10, 0), n(400) / n(30)),
            (hex(0, 0), n(48) + n(400) / n(30))
        ]
    );
    let at = force_by_moment(&t);
    assert_eq!(at["1 Night"], hex(0, 0));
    assert_eq!(at["1 Morning"], hex(9, 0));
    assert_eq!(at["1 Afternoon"], hex(10, 0));
    assert_eq!(at["2 Night"], hex(10, 0));
    assert_eq!(at["3 Night"], hex(10, 0));
    assert_eq!(at["3 Afternoon"], hex(0, 0));
    assert_eq!(at["4 Night"], hex(0, 0));
    // Group 3 launches nothing on any day.
    assert!(played.all_launches().is_empty());
    // At the end.
    let f = played.force_at_end();
    assert_eq!(f.position(), hex(0, 0));
    assert_eq!(f.toward_hex, None);
    assert_eq!(f.speed, SpeedSetting::Full);
    assert_eq!(f.route.next_index, f.route.waypoints.len(), "{:?}", f.route);
    assert!(!matches!(
        f.status,
        sim_core::state::TaskForceStatus::Underway | sim_core::state::TaskForceStatus::Halted
    ));
    // 800 nm steamed at 30 kn: 80/3 hours of steaming.
    let steamed: Fixed = played
        .all(|k| matches!(k, EventKind::TaskForceSteamed { .. }))
        .iter()
        .map(|e| match &e.kind {
            EventKind::TaskForceSteamed { from, to, .. } => n(i64::from(from.air_distance_nm(*to))),
            _ => unreachable!(),
        })
        .fold(Fixed::ZERO, |a, b| a + b);
    assert_eq!(steamed, n(800));
    // One notice of this rule in the game: the pursuit ended, on day 3.
    let kinds: Vec<(u32, NoticeKind)> = played
        .all(|k| {
            matches!(
                k,
                EventKind::StaffNotice {
                    kind: NoticeKind::AsksLeaveToPursue
                        | NoticeKind::Pursues
                        | NoticeKind::PursuitEnded,
                    ..
                }
            )
        })
        .iter()
        .map(|e| match &e.kind {
            EventKind::StaffNotice { kind, .. } => (e.day, *kind),
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(kinds, vec![(3, NoticeKind::PursuitEnded)]);
    // Every order is decided at the level of the force, but the sizing.
    for (day, order) in played.all_orders() {
        if given(order) == Given::Air(3, sizing(true)) {
            assert_eq!(level(&order.reason), Some(Addressee::AirGroup(group(3))));
        } else {
            assert_eq!(
                level(&order.reason),
                Some(Addressee::TaskForce(force(1))),
                "day {day}"
            );
        }
        assert_eq!(order.intent, intent(1));
        let served = order.reason.serves.intent.expect("an intent");
        assert_eq!(served, intent(1));
        assert_eq!(played.state.intents[&served].maker, Maker::Commander,);
    }
}

#[test]
fn she_burns_what_the_same_ship_ordered_over_the_same_passages_burns() {
    let by_staff = play(&answered_text());
    let t = text();
    let at = t.find(COMMANDS_MARKER).expect("the marker");
    let plain = format!(
        "{}{}",
        &t[..at],
        "[[order]]\nday = 1\nside = \"allied\"\ntask_force = 1\nkind = \"set_speed\"\nspeed = \"full\"\n\n[[order]]\nday = 1\nside = \"allied\"\ntask_force = 1\nkind = \"set_route\"\nwaypoints = [[10, 0]]\n\n[[order]]\nday = 3\nside = \"allied\"\ntask_force = 1\nkind = \"set_route\"\nwaypoints = [[0, 0]]\n"
    );
    let by_orders = play(&plain);
    by_orders.nothing_refused();
    assert_eq!(by_orders.force_at_end().position(), hex(0, 0));
    let a = by_staff.truth.last().expect("a state").ships[&ship(1)].fuel_t;
    let b = by_orders.truth.last().expect("a state").ships[&ship(1)].fuel_t;
    assert_eq!(a, b);
    assert!(a < n(3000), "{a}");
}

// --------------------------------------------------------------- Properties

#[test]
fn not_answered_the_asking_is_repeated_at_dusk_of_day_2_and_she_never_leaves() {
    let played = play(&with(&command_table(1, INTERRUPT_OFF)));
    played.nothing_refused();
    assert_eq!(played.orders(1), too_far_orders());
    assert_eq!(played.notices(1).len(), 1);
    assert_eq!(
        content(&played.notices(1)[0]),
        content(&asking_notice(0, ShipKind::Carrier))
    );
    assert_eq!(played.notices(2).len(), 1);
    assert_eq!(
        content(&played.notices(2)[0]),
        content(&asking_notice(3, ShipKind::Carrier))
    );
    assert!(played.notices(3).is_empty());
    assert!(played.notices(4).is_empty());
    assert!(played.orders_to_force(2, 1).is_empty());
    played.never_leaves_its_hex();
    assert!(played.answered.iter().all(|a| a.is_empty()));
    assert!(played.interrupts_fired().is_empty());
}

#[test]
fn refused_the_find_is_not_asked_about_again_and_she_never_leaves() {
    let played = play(&with(&command_table(1, NO)));
    played.nothing_refused();
    for day in 1..=4 {
        assert!(!played.orders(day).iter().any(is_route), "day {day}");
        assert!(!played.orders(day).iter().any(is_speed), "day {day}");
    }
    assert!(played.notices(2).is_empty());
    played.never_leaves_its_hex();
    // The same through a seat: a no at the stop.
    let game = seat_game(&text(), Some(false));
    assert_eq!(game.stop["stopped"], "asks_leave");
    for (day, d) in &game.handed {
        let orders: Vec<Given> = d.orders.iter().map(given).collect();
        assert!(!orders.iter().any(is_route), "day {day}");
    }
    assert!(game.handed[&2].notices.is_empty());
}

#[test]
fn go_no_asking_no_stop_a_pursues_notice_and_the_answered_games_orders() {
    let t = pursuit_leave(&text(), "go");
    let played = play(&t);
    let answered = play(&answered_text());
    played.nothing_refused();
    assert_eq!(played.notice_kinds(1), vec![NoticeKind::Pursues]);
    let find = find_of(&played.notices(1)[0]);
    let want = find_of(&asking_notice(0, ShipKind::Carrier));
    assert_eq!(find, want);
    assert_eq!(played.notices(1)[0].pursuit.unwrap().force, force(1));
    assert!(played.notices(2).is_empty());
    assert_eq!(played.notice_kinds(3), vec![NoticeKind::PursuitEnded]);
    assert!(played.notices(4).is_empty());
    assert!(played.interrupts_fired().is_empty());
    assert_eq!(played.all_orders_given(), answered.all_orders_given());
    assert_eq!(played.arrivals(), answered.arrivals());
    let r = said(reason_of(&played, 1, &Given::Force(1, route(&[(10, 0)]))));
    assert!(r.contains("the leave to pursue was taken"), "{r}");
    // No stop through a seat.
    let game = seat_game(&t, None);
    assert_eq!(game.stop["stopped"], "game_over", "{}", game.stop);
    assert_eq!(counts(&game.stop), (1, 4, 4));
}

#[test]
fn stay_gives_no_notice_and_every_order_the_rule_off_gives() {
    let stay = play(&pursuit_leave(&text(), "stay"));
    let off = play_on(&text(), &figures_with(1));
    stay.nothing_refused();
    assert!(stay.no_notice_on_any_day());
    assert!(off.no_notice_on_any_day());
    assert_eq!(stay.all_orders_given(), off.all_orders_given());
    assert_eq!(stay.orders(1), too_far_orders());
    stay.never_leaves_its_hex();
}

#[test]
fn the_default_is_go_where_launching_is_given_and_ask_where_it_waits() {
    let given_text = without_pursuit_setting(&text());
    let played = play(&given_text);
    assert_eq!(played.notice_kinds(1), vec![NoticeKind::Pursues]);
    assert_eq!(
        played.state.pursue_read_up(intent(1)).0,
        sim_core::command::PursueLeave::Go
    );
    let withheld = changed(
        &given_text,
        "leave = \"launch\"\ngiven = true",
        "leave = \"launch\"\ngiven = false",
    );
    let played = play(&withheld);
    assert_eq!(
        played.state.pursue_read_up(intent(1)).0,
        sim_core::command::PursueLeave::Ask
    );
}

#[test]
fn the_default_with_launching_withheld_the_staff_asks() {
    // With leave to launch withheld the commander releases the plan himself:
    // two approvals on day 1 (draft to preparing, preparing to released).
    let withheld = changed(
        &without_pursuit_setting(&text()),
        "leave = \"launch\"\ngiven = true",
        "leave = \"launch\"\ngiven = false",
    );
    let t = format!(
        "{withheld}{}{}",
        command_table(1, "kind = \"approve\"\nplan = 1"),
        command_table(1, "kind = \"approve\"\nplan = 1")
    );
    let played = play(&t);
    played.nothing_refused();
    assert_eq!(played.notice_kinds(1), vec![NoticeKind::AsksLeaveToPursue]);
    assert_eq!(find_of(&played.notices(1)[0]).to, hex(10, 0));
    assert!(!played.orders(1).iter().any(is_route));
}

#[test]
fn within_a_night_she_closes_as_scenario_58_with_no_notice_at_ask_go_or_stay() {
    for leave in ["ask", "go", "stay"] {
        let played = play(&contact_at(&pursuit_leave(&text(), leave), 12));
        played.nothing_refused();
        assert_eq!(played.orders(1), closing_orders(6), "{leave}");
        assert!(played.no_notice_on_any_day(), "{leave}");
        assert_eq!(played.arrivals()[0], (hex(6, 0), n(8)), "{leave}");
    }
}

#[test]
fn beyond_two_nights_there_is_no_find_and_no_notice() {
    for leave in ["ask", "go"] {
        let played = play(&contact_at(&pursuit_leave(&text(), leave), 25));
        assert!(played.no_notice_on_any_day(), "{leave}");
        assert_eq!(played.orders(1), too_far_orders(), "{leave}");
        played.never_leaves_its_hex();
    }
}

#[test]
fn a_kind_the_risk_does_not_close_on_balanced_none_press_asks_and_names_a_cruiser() {
    let balanced = play(&cruiser_contact(&text()));
    assert!(balanced.no_notice_on_any_day());
    assert!(!balanced.orders(1).iter().any(is_route));
    let press = play(&cruiser_contact(&at_risk(&text(), "press")));
    assert_eq!(press.notice_kinds(1), vec![NoticeKind::AsksLeaveToPursue]);
    let find = find_of(&press.notices(1)[0]);
    assert_eq!(find.kind, ShipKind::Cruiser);
    assert_eq!(find.contact, 1);
    assert_eq!(find.to, hex(10, 0));
    assert!(!press.orders(1).iter().any(is_route));
}

#[test]
fn outside_the_area_there_is_no_find_at_ask_or_go() {
    for leave in ["ask", "go"] {
        let t = with(&command_table(
            1,
            "kind = \"set_limit\"\nlimit = \"area\"\nintent = 1\nhexes = [[0, 0], [1, 0], [2, 0]]",
        ));
        let played = play(&pursuit_leave(&t, leave));
        assert!(played.no_notice_on_any_day(), "{leave}");
        assert!(!played.orders(1).iter().any(is_route), "{leave}");
        played.never_leaves_its_hex();
    }
}

#[test]
fn something_in_reach_the_rule_as_it_stands_stays_and_no_find_is_looked_for() {
    let played = play(&with_second_contact(&text(), 5, "carrier"));
    assert!(played.no_notice_on_any_day());
    assert!(!played.orders(1).iter().any(is_route));
    played.never_leaves_its_hex();
}

#[test]
#[ignore = "the test cannot hand a side a contact after the start: BeliefState::apply_report is pub(crate) in sim-core, a scenario's [[contact]] has no day, and only a real enemy ship sighted by the scouts would give one; reported to the architect"]
fn seen_again_under_a_new_number_the_pursuit_goes_on() {
    // Answered game, a contact 2 at (26,0) handed to the side at dusk of day 3
    // keeps the leave and she closes to (20,0); at (22,0) she closes to (16,0).
    unimplemented!("no way to give a side a contact after the start");
}

#[test]
#[ignore = "not built: a force covering a landing needs a take intent, a landing force, land and a map wide enough for a contact beyond 610 nm (scenario 73's map is 400 nm long); reported to the architect"]
fn a_force_covering_a_landing_has_no_find() {
    unimplemented!("scenario 73's map is too small for a find");
}

#[test]
fn a_held_force_has_no_find_and_is_left_alone() {
    let played = play(&with(&command_table(1, "kind = \"hold\"\ntask_force = 1")));
    assert!(played.all_orders().is_empty());
    assert!(played.no_notice_on_any_day());
    played.never_leaves_its_hex();
}

#[test]
fn with_command_levels_on_the_asking_is_handed_in_only_where_the_commanders_picture_holds_the_contact(
) {
    let told = play(&levels_on(&text()));
    assert_eq!(told.notice_kinds(1), vec![NoticeKind::AsksLeaveToPursue]);
    assert_eq!(find_of(&told.notices(1)[0]).contact, 1);
    let own = play(&changed(
        &levels_on(&text()),
        "age_pulses = 0",
        "age_pulses = 0\nseen_by = { task_force = 1 }",
    ));
    assert!(own.notices(1).is_empty(), "{:?}", own.notices(1));
}

#[test]
fn the_same_start_gives_the_same_orders_and_notices() {
    let a = play(&answered_text());
    let b = play(&answered_text());
    assert_eq!(a.handed, b.handed);
    assert_eq!(a.state, b.state);
    assert_eq!(a.events.len(), b.events.len());
    let a = play(&text());
    let b = play(&text());
    assert_eq!(a.handed, b.handed);
}

#[test]
fn two_worlds_that_differ_only_in_what_the_allied_side_cannot_see_give_the_same_decisions() {
    let hidden = format!(
        "{}\n[[base]]\nid = 3\nname = \"Placeholder base 3\"\nside = \"japanese\"\nq = 10\nr = 5\nsupply_t = 50\non_map = false\n\n[[air_group]]\nid = 6\nname = \"Placeholder Japanese fighters 6\"\nside = \"japanese\"\nbase = 3\naircraft_type = 4\nready = 20\n",
        text()
    );
    let plain = play(&text());
    let other = play(&hidden);
    assert_ne!(
        plain.truth.last().map(|t| t.bases.len()),
        other.truth.last().map(|t| t.bases.len()),
        "the two worlds do not differ"
    );
    assert_eq!(plain.handed, other.handed);
    assert_eq!(plain.state, other.state);
}

/// Every order of a staff, by day and side, as text.
fn orders_by_day_and_side(
    text: &str,
    path: &std::path::Path,
    figures: &sim_data::LoadOptions,
) -> Option<Vec<String>> {
    let mut scenario: Scenario = common::from_text_with(text, path, figures).ok()?;
    let held = staffs();
    let mut out = Vec::new();
    loop {
        let day = scenario.world.day() + 1;
        match scenario.next_day_watched(as_staffs(&held), &mut |_, _| {}) {
            Ok(Some(_)) => {}
            Ok(None) => break,
            Err(_) => return None,
        }
        for side in Side::ALL {
            if let Some(d) = scenario.world.staff_handed_in().get(&side) {
                for o in &d.orders {
                    out.push(format!("{day} {side:?} {:?} {:?}", o.kind, o.intent));
                }
            }
        }
    }
    Some(out)
}

/// The scenario's text with the leave to pursue set at stay on every intent
/// it states, on the day and for the side that states it.
fn with_stay_everywhere(text: &str) -> String {
    let mut extra = String::new();
    for table in text.split("[[command]]").skip(1) {
        if !table.contains("kind = \"set_intent\"") {
            continue;
        }
        let field = |key: &str| -> Option<String> {
            table
                .lines()
                .find_map(|l| l.strip_prefix(&format!("{key} = ")).map(str::to_string))
        };
        let (Some(day), Some(side), Some(intent)) = (field("day"), field("side"), field("intent"))
        else {
            continue;
        };
        extra += &format!(
            "\n[[command]]\nday = {day}\nside = {side}\nkind = \"set_pursuit\"\nintent = {intent}\nleave = \"stay\"\n"
        );
    }
    format!("{text}{extra}")
}

/// Chosen: every staffed scenario built before this one (a file with
/// commands, numbered below 125, of at most 12 days, that loads on the
/// frozen figures) is played twice on the frozen figures: with
/// `pursuit_nights` 2 and the leave at stay on every intent it states, and
/// with `pursuit_nights` 1 (the rule off, as the frozen figures have it). The
/// orders of both sides are the same, day by day.
#[test]
fn every_staffed_scenario_built_before_this_one_with_the_leave_at_stay_gives_the_orders_it_gave() {
    let dir = common::data_dir().join("scenarios");
    let mut names: Vec<_> = std::fs::read_dir(&dir)
        .expect("the scenarios read")
        .map(|e| e.expect("an entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "toml"))
        .collect();
    names.sort();
    let stay_figures = figures_with(2);
    let off_figures = figures_with(1);
    let mut compared = Vec::new();
    for path in names {
        let file = path.file_name().unwrap().to_string_lossy().to_string();
        let number: u32 = file
            .chars()
            .take_while(char::is_ascii_digit)
            .collect::<String>()
            .parse()
            .unwrap_or(999);
        if number >= 125 {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("reads");
        if !text.contains("[[command]]") {
            continue;
        }
        let days: u32 = text
            .lines()
            .find_map(|l| l.strip_prefix("days = "))
            .and_then(|d| d.trim().parse().ok())
            .unwrap_or(999);
        if days > 12 {
            continue;
        }
        let Some(off) = orders_by_day_and_side(&text, &path, &off_figures) else {
            continue;
        };
        let stay = orders_by_day_and_side(&with_stay_everywhere(&text), &path, &stay_figures)
            .unwrap_or_else(|| panic!("{file} loads with the rule on and not off"));
        assert_eq!(stay, off, "{file}");
        compared.push(file);
    }
    assert!(compared.len() >= 10, "{compared:?}");
}
