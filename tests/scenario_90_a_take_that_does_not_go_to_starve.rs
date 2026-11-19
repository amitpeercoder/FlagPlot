//! Scenario 90: a take that does not go to starve. The commander says
//! "control the island". The side's men are at sea in transports, fewer
//! than the staff wants against the enemy it has been told is there, with
//! no more in reach and not the supply aboard to last until a run could
//! reach them. At balanced the staff does not lift them: it hands in
//! "cannot bring enough" and waits.
//!
//! Proves decision 0169, the second mend of step 3's stage 4, against a
//! hand calculation. It is scenario 86 with two changes: the Allied side
//! is told of 400 enemy men at B, and ship 1 has 10 t of supply aboard
//! where it had 30 t. The data is placeholder data: see
//! `data/scenarios/90_a_take_that_does_not_go_to_starve.toml`. It runs on
//! the frozen step 1 figures. No enemy man is ashore anywhere, so no die
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
//! As scenario 86's. A man eats 0.005 t a day. A hex is 40 nm and the
//! transports make 10 kn. At balanced the staff wants 2 men for each of
//! the enemy's it holds an estimate of; the margin of the check that a
//! place can be held is 5 days; a supply intent with no interval uses 15
//! days; with no latest day the staff looks 7 days ahead.
//!
//! ## Inputs
//!
//! Scenario 86's map, bases, task force, ships, units, island and
//! commands, with:
//!
//! - an estimate held by the Allied side from the start: 400 enemy men
//!   at B, (4,0);
//! - ship 1 with 10 t of supply aboard. Ship 2 has none.
//!
//! So: base 1 at A, (0,0), Allied, 1,000 t of supply; base 2 at B,
//! Japanese, with nothing at it; base 4 at C, (0,6), Allied, with
//! nothing. Task force 1, Allied, on the transport mission, ships 1 and
//! 2, at sea at (2,0), stopped, its home port base 4; unit 1, fighting,
//! 600 men, in ship 1; unit 2, working, 400 men, in ship 2. Intent 1,
//! control island 1, on day 1; no risk set, so balanced; leave to
//! prepare and to launch both given. The game is 8 days.
//!
//! ## The count, on the morning of day 1
//!
//! - **The men wanted:** 400 x 2 = 800.
//! - **The lift:** task force 1, a lift as it stands. Its fighting men
//!   are unit 1's 600. No man of the operation is ashore at B.
//!   600 < 800: the lift is short, by 200.
//! - **The men in reach:** no latest day is set, so the staff reckons to
//!   day 1 + 7 = day 8. No other transport force has a fighting unit
//!   aboard. The men in reach are the 600: too few.
//!
//! ## Whether they can be fed
//!
//! The source is base 1, named on day 1 as in scenario 86, and the hold
//! of base 2 has it.
//!
//! - **The check that the place can be held** (decision 0144), for the
//!   1,000 men the lift lands: the daily need is 1,000 x 0.005 = 5 t.
//!   The days until supply follows are 1 to land the men, 1 back to base
//!   1 (160 nm at 10 kn = 16 hours), 1 to load and 1 out: 4 days. The
//!   supply that lands with the men is 5 x (4 + 5) = 45 t and the first
//!   run 5 x 15 = 75 t: 120 t. Base 1 holds 1,000 t above its reserve
//!   and task force 1 has 6,000 t of room. Nothing is short: it can be
//!   held.
//! - **What is aboard against the days until supply follows:** 5 t x 4
//!   days = 20 t. The force lies away from its source and loads nothing:
//!   it has 10 t. 10 < 20, short by 10 t. The men cannot be fed.
//!
//! ## Day by day
//!
//! | Day | The staff | What happens |
//! | --- | --- | --- |
//! | 1 | Forms no landing under the take and gives task force 1 no order. Hands in the notice "cannot bring enough" for the take's plan | The standing interrupt "cannot bring enough" fires. Task force 1 lies at (2,0) |
//! | 2 to 8 | Each morning the same reckoning, with the day reckoned to moved on by one (day 9 on day 2, and so to day 15 on day 8). No landing, no order, no second notice | Nothing |
//!
//! The reason of each morning under the take gives: 800 men wanted, 600
//! in the lift, 600 in reach, the day reckoned to, and 10 t aboard
//! against the 20 t of the 4 days until supply follows.
//!
//! - **No staff order is given in the whole game,** to task force 1 or
//!   to a unit. None is refused.
//! - **One notice "cannot bring enough"** is handed in, on day 1, and
//!   none after.
//! - **At the end:** task force 1 is at (2,0) with unit 1's 600 men and
//!   unit 2's 400 aboard, none ashore. Base 2 is Japanese. No base has
//!   been formed at B. Base 1 holds 1,000 t and base 4 nothing.
//!
//! As the rule stood before decision 0169 the lift sailed on day 1, the
//! staff saying "at balanced the staff goes with what it has", and 1,000
//! men were ashore at B with 10 t, two days' food, and the first run due
//! on day 5.
//!
//! # Properties
//!
//! Each is the main case with the change named.
//!
//! - **Fed: it goes.** With ship 1's 30 t aboard, as in scenario 86:
//!   30 t is not less than 20 t, the place can be held, and the men can
//!   be fed. The lift is short by the same 200 men and goes: on day 1
//!   task force 1 is given its landing order with unit 1, its route to
//!   B and on to base 1, and its unload order, as in scenario 86. No
//!   notice "cannot bring enough" is handed in. The reason of the
//!   landing says it goes short and that the men can be fed.
//! - **The source too poor.** With 30 t aboard and base 1 holding
//!   100 t: base 1 is still the source, and the check finds 100 t
//!   against the 120 t wanted, short by 20 t. The place cannot be held,
//!   so the men cannot be fed: no landing, no order, one notice on day
//!   1, as in the main case.
//! - **No count.** With no estimate: nothing is wanted, nothing is
//!   short, and the lift goes on day 1 with its 10 t, as it did before
//!   the rule.
//! - **Not short.** With the estimate 300: 600 men are wanted and 600
//!   are in the lift. The lift goes on day 1 with its 10 t, and no
//!   notice "cannot bring enough" is handed in.
//! - **At press it goes.** With the risk of intent 1 set to press on day
//!   1: 400 men are wanted, the lift is not short, and it goes on day 1.
//!   With the estimate 800 as well: 800 are wanted, the lift is short
//!   by 200 and cannot be fed, and at press it goes on day 1 all the
//!   same, with no notice "cannot bring enough".
//! - **At preserve it is as it was.** With the risk of intent 1 set to
//!   preserve on day 1: 1,200 men are wanted. The take forms no landing
//!   and gives no order, as before the rule, and hands in no notice
//!   "cannot bring enough": what it hands in is what it handed in
//!   before the rule.
//! - **The commander says go.** With the risk of intent 1 set to press
//!   on day 3, and nothing else changed: on days 1 and 2 the main case;
//!   on the morning of day 3 task force 1 is given its landing order
//!   with unit 1.
//! - **More men in reach.** With a second transport force, task force 2,
//!   of one placeholder troop transport at (2,0) with a fighting unit of
//!   200 men aboard and no supply, which may be used from day 5: on day
//!   1 the men in reach are 600 + 200 = 800, the number wanted, by day
//!   5, inside day 8. The lift of task force 1 goes on day 1 with its
//!   10 t, and no notice "cannot bring enough" is handed in. Only day 1
//!   is asserted.
//! - **Out of reach.** As "More men in reach", with task force 2 usable
//!   from day 10: on days 1 and 2 the men in reach are 600 (day 10 is
//!   after day 8 and after day 9), and the take does not lift, with the
//!   one notice on day 1. On the morning of day 3 the staff reckons to
//!   day 10: the men in reach are 800, and task force 1 is given its
//!   landing order. Only days 1 to 3 are asserted.
//! - **Scenario 86 is as it was.** Scenario 86's own test passes
//!   unchanged: it holds no estimate.
//! - **The same start gives the same game,** and a game saved and
//!   replayed gives the same.

// ---------------------------------------------------------------- The harness
//
// Modelled on scenario 86's. Everything above this line is the architect's
// hand calculation; nothing below it states a number the hand calculation
// does not.

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{
    CommandKind, CommandState, IntentKind, InterruptRef, NoticeKind, Reason, StaffDecisions,
    StaffOrder, Staffs, StandingKind,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, LandUnitId};
use sim_core::orders::{OrderKind, TaskForceOrder};
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const ESTIMATE_MARKER: &str = "# ESTIMATE_MARKER\n";
const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/90_a_take_that_does_not_go_to_starve.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// The frozen step 1 figures.
fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &common::frozen()).expect("the scenario loads")
}

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    hashes: Vec<u64>,
    state: CommandState,
}

fn play(text: &str) -> Played {
    play_from(load(text), BTreeMap::new())
}

fn play_from(mut scenario: Scenario, mut handed: BTreeMap<u32, StaffDecisions>) -> Played {
    let held = staffs();
    let mut truth = vec![scenario.world.umpire().clone()];
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
    }
    Played {
        handed,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        hashes: scenario.world.day_hashes().to_vec(),
        state: scenario.world.command_state(Side::Allied).clone(),
    }
}

/// An order of the staff, plainly, with the task force it is for. Any other
/// order is `Other`, with its debug text.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Landing(u32, Hex, Vec<u32>),
    Route(u32, Vec<Hex>, Vec<Hex>),
    Unload(u32),
    Other(String),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce { target, order } => {
            let tf = target.raw();
            match order {
                TaskForceOrder::PrepareLanding { hex, roster } => {
                    Given::Landing(tf, *hex, roster.iter().map(|u| u.raw()).collect())
                }
                TaskForceOrder::SetRoute {
                    waypoints,
                    act_at,
                    stand_at,
                } if stand_at.is_empty() => {
                    Given::Route(tf, waypoints.clone(), act_at.iter().copied().collect())
                }
                TaskForceOrder::Unload => Given::Unload(tf),
                other => Given::Other(format!("task force {tf}: {other:?}")),
            }
        }
        other => Given::Other(format!("{other:?}")),
    }
}

/// Whether two lists hold the same orders, whatever the order they stand in.
fn same(a: &[Given], b: &[Given]) -> bool {
    let key = |v: &[Given]| {
        let mut k: Vec<String> = v.iter().map(|g| format!("{g:?}")).collect();
        k.sort();
        k
    };
    key(a) == key(b)
}

/// Scenario 86's orders of day 1: the landing order with unit 1, the route
/// to B and on to base 1, acting at B, and the unload order.
fn lift_orders() -> Vec<Given> {
    vec![
        Given::Landing(1, B, vec![1]),
        Given::Route(1, vec![B, A], vec![B]),
        Given::Unload(1),
    ]
}

impl Played {
    fn days(&self) -> u32 {
        self.handed.len() as u32
    }

    fn orders(&self, day: u32) -> Vec<Given> {
        self.handed[&day].orders.iter().map(given).collect()
    }

    fn all_orders(&self) -> Vec<(u32, Given)> {
        (1..=self.days())
            .flat_map(|d| self.orders(d).into_iter().map(move |g| (d, g)))
            .collect()
    }

    /// Every "ready a landing" the staff formed, by day.
    fn landings_formed(&self) -> Vec<u32> {
        (1..=self.days())
            .flat_map(|d| {
                self.handed[&d]
                    .commands
                    .iter()
                    .filter(|c| {
                        matches!(
                            c.kind,
                            CommandKind::FormIntent {
                                kind: IntentKind::ReadyLanding { .. },
                                ..
                            }
                        )
                    })
                    .map(move |_| d)
            })
            .collect()
    }

    /// Every notice "cannot bring enough" handed in: (day, plan).
    fn cannot_bring_enough(&self) -> Vec<(u32, u32)> {
        (1..=self.days())
            .flat_map(|d| {
                self.handed[&d]
                    .notices
                    .iter()
                    .filter(|nt| nt.kind == NoticeKind::CannotBringEnough)
                    .map(move |nt| (d, nt.plan.raw()))
            })
            .collect()
    }

    fn notices(&self, day: u32) -> Vec<NoticeKind> {
        self.handed[&day].notices.iter().map(|nt| nt.kind).collect()
    }

    /// The days on which the Allied standing interrupt "cannot bring
    /// enough" fired.
    fn cannot_bring_enough_fired(&self) -> Vec<u32> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::InterruptFired {
                    side: Side::Allied,
                    interrupt:
                        InterruptRef::Standing {
                            kind: StandingKind::CannotBringEnough,
                            ..
                        },
                    ..
                } => Some(e.day),
                _ => None,
            })
            .collect()
    }

    /// Every reason of `day` that serves intent `of`, as text.
    fn reasons_serving(&self, day: u32, of: u32) -> String {
        let d = &self.handed[&day];
        let mut all: Vec<&Reason> = d.commands.iter().map(|c| &c.reason).collect();
        all.extend(d.orders.iter().map(|o| &o.reason));
        all.extend(d.no_orders.iter());
        all.into_iter()
            .filter(|r| r.serves.intent == Some(IntentId::new(of)))
            .map(text_of)
            .collect::<Vec<_>>()
            .join(" || ")
    }

    /// The reason of the landing formed on `day`: the command that forms it.
    fn landing_reason(&self, day: u32) -> String {
        self.handed[&day]
            .commands
            .iter()
            .filter(|c| {
                matches!(
                    c.kind,
                    CommandKind::FormIntent {
                        kind: IntentKind::ReadyLanding { .. },
                        ..
                    }
                )
            })
            .map(|c| text_of(&c.reason))
            .collect::<Vec<_>>()
            .join(" || ")
    }

    fn unit_state(&self, day: u32, u: u32) -> &sim_core::state::LandUnit {
        &self.truth[day as usize].land_units[&LandUnitId::new(u)]
    }

    fn base(&self, day: u32, b: u32) -> Option<&sim_core::state::Base> {
        self.truth[day as usize].bases.get(&BaseId::new(b))
    }

    fn refused(&self) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
                )
            })
            .collect()
    }

    /// The plan of the take, intent 2.
    fn take_plan(&self) -> u32 {
        self.state.intents[&IntentId::new(2)].plan.raw()
    }
}

fn text_of(reason: &Reason) -> String {
    format!(
        "{} | {} | {}",
        reason.worked,
        reason.binding.join("; "),
        reason
            .evidence
            .iter()
            .map(|e| e.what.clone())
            .collect::<Vec<_>>()
            .join("; ")
    )
}

fn assert_says(all: &str, parts: &[&str]) {
    for part in parts {
        assert!(all.contains(part), "the reason says {part:?}: {all}");
    }
}

/// A command table of the Allied side, added to the end of the file.
fn command_table(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
}

fn risk_on(t: &str, day: u32, level: &str) -> String {
    format!(
        "{t}{}",
        command_table(
            day,
            &format!("kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{level}\"")
        )
    )
}

/// The estimate changed to `men`.
fn estimate(t: &str, men: u32) -> String {
    changed(
        t,
        "q = 4\nr = 0\nmen = 400\n",
        &format!("q = 4\nr = 0\nmen = {men}\n"),
    )
}

/// No estimate held.
fn no_estimate(t: &str) -> String {
    changed(
        t,
        "[[estimate]]\nside = \"allied\"\nq = 4\nr = 0\nmen = 400\n",
        "",
    )
}

/// Ship 1 with 30 t of supply aboard, as in scenario 86.
fn thirty_aboard(t: &str) -> String {
    changed(t, "supply_t = 10\n", "supply_t = 30\n")
}

/// Task force 2: one placeholder troop transport at (2,0) with a fighting
/// unit of 200 men aboard and no supply, which may be used from `day`.
fn force_2_from(t: &str, day: u32) -> String {
    let force = format!(
        "# Task force 2: one placeholder troop transport, usable from day {day}.\n[[task_force]]\nid = 2\nname = \"Placeholder transport force 2\"\nside = \"allied\"\nq = 2\nr = 0\nhome_port = 4\navailable_from = {day}\nmission = \"transport\"\n\n[[ship]]\nid = 3\nname = \"Placeholder troop transport 3\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000\ntask_force = 2\n\n[[land_unit]]\nid = 3\nname = \"Placeholder Infantry 3\"\nside = \"allied\"\nmen = 200\ntask_force = 2\n\n"
    );
    changed(t, ESTIMATE_MARKER, &format!("{force}{ESTIMATE_MARKER}"))
}

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

/// The lift goes on day 1 as in scenario 86: its orders, no notice "cannot
/// bring enough".
fn goes_on_day_1(g: &Played) {
    assert!(
        same(&g.orders(1), &lift_orders()),
        "{:?} against {:?}",
        g.orders(1),
        lift_orders()
    );
    assert_eq!(g.landings_formed().first(), Some(&1));
    assert!(
        g.cannot_bring_enough().is_empty(),
        "{:?}",
        g.cannot_bring_enough()
    );
}

/// The take does not lift on `days`: no landing, no order, and the one
/// notice on day 1.
fn does_not_lift(g: &Played, days: std::ops::RangeInclusive<u32>) {
    for day in days {
        assert!(g.orders(day).is_empty(), "day {day}: {:?}", g.orders(day));
        assert!(
            !g.landings_formed().contains(&day),
            "day {day}: {:?}",
            g.landings_formed()
        );
    }
    let notices = g.cannot_bring_enough();
    assert_eq!(notices.len(), 1, "{notices:?}");
    assert_eq!(notices[0].0, 1, "{notices:?}");
}

// ---------------------------------------------------------------- The hand calculation

#[test]
fn the_file_is_scenario_86_with_the_two_changes() {
    let t = text();
    let s86 =
        std::fs::read_to_string(common::data_dir().join("scenarios/86_fed_with_the_lift.toml"))
            .expect("scenario 86 reads");
    let body = |s: &str| s[s.find("seed = ").expect("a seed")..].to_string();
    let ours = body(&t);
    let theirs = body(&s86);
    // Ship 1's 30 t made 10 t, and the estimate of 400 men at B added.
    let back = changed(&ours, "supply_t = 10\n", "supply_t = 30\n");
    let back = changed(
        &back,
        "# The Allied side's opening estimate of the enemy's men at B.\n# ESTIMATE_MARKER\n[[estimate]]\nside = \"allied\"\nq = 4\nr = 0\nmen = 400\n\n",
        "",
    );
    let back = changed(
        &back,
        "which has 10 t of supply",
        "which has 30 t of supply",
    );
    assert_eq!(back, theirs);
    assert_eq!(t.matches(COMMANDS_MARKER).count(), 1);
    assert_eq!(t.matches(ESTIMATE_MARKER).count(), 1);
    assert!(t.contains("days = 8"));
    assert!(t.contains("placeholder = true"));
    assert!(!t.contains("[[order]]"));
}

#[test]
fn no_landing_is_formed_and_no_order_is_given_in_the_whole_game() {
    let g = main_game();
    assert_eq!(g.days(), 8);
    assert!(g.landings_formed().is_empty(), "{:?}", g.landings_formed());
    assert!(g.all_orders().is_empty(), "{:?}", g.all_orders());
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn one_notice_cannot_bring_enough_on_day_1_and_none_after() {
    let g = main_game();
    let notices = g.cannot_bring_enough();
    assert_eq!(notices.len(), 1, "{notices:?}");
    assert_eq!(notices[0].0, 1);
    // It is for the take's plan.
    assert_eq!(notices[0].1, g.take_plan());
    assert!(matches!(
        g.state.intents[&IntentId::new(2)].kind,
        IntentKind::TakePlace { hex: B, .. }
    ));
    assert!(g.notices(1).contains(&NoticeKind::CannotBringEnough));
    for day in 2..=8 {
        assert!(
            !g.notices(day).contains(&NoticeKind::CannotBringEnough),
            "day {day}: {:?}",
            g.notices(day)
        );
    }
    // The standing interrupt fires on day 1.
    assert_eq!(g.cannot_bring_enough_fired(), vec![1]);
}

#[test]
fn the_notice_is_for_the_take_of_the_place() {
    let g = main_game();
    // The take is intent 2, as in scenario 86; the notice rests on its plan.
    let reasons = g.reasons_serving(1, 2);
    assert_says(
        &reasons,
        &["hands in the notice that it cannot bring enough"],
    );
}

#[test]
fn the_reason_of_each_morning_gives_the_count_and_the_supply() {
    let g = main_game();
    for day in 1..=8 {
        let reasons = g.reasons_serving(day, 2);
        assert_says(
            &reasons,
            &[
                "The staff wants 800 men; the lift by task_force:1 gives 600 fighting men: 600, short by 200",
                &format!(
                    "The men in reach by day {}, the 7 days the staff looks ahead: 600 (no other force with fighting men aboard can be used by then), fewer than the 800 wanted",
                    day + 7
                ),
                "it can be held",
                "10 t against the daily need of 5 t for the 4 days until supply follows, 5 t x 4 = 20 t, short by 10 t: the men cannot be fed",
                "At balanced the staff does not land men it cannot feed",
            ],
        );
        // The check that the place can be held, for the 1,000 men.
        assert_says(
            &reasons,
            &[
                "the daily need of the 1000 men chosen is 5 t",
                "the days until supply follows are 1 to land the men",
                "1 back to base:1, 1 to load and 1 out: 4 days",
                "the supply that lands with the men is 5 t x (4 + 5) = 45 t",
                "the first run is 5 t x 15 = 75 t",
                "base:1 holds 1000 t above its reserve against 45 t + 75 t = 120 t",
                "task_force:1's room is 6000 t",
            ],
        );
        let notice_words = reasons.contains("hands in the notice that it cannot bring enough");
        assert_eq!(notice_words, day == 1, "day {day}: {reasons}");
    }
}

#[test]
fn at_the_end_the_men_are_aboard_and_base_2_is_japanese() {
    let g = main_game();
    let last = g.truth.last().expect("the end");
    let tf1 = last
        .task_forces
        .get(&sim_core::ids::TaskForceId::new(1))
        .expect("task force 1");
    assert_eq!(tf1.position(), Hex { q: 2, r: 0 });
    let aboard = |unit: u32| -> u32 {
        last.ships
            .values()
            .filter_map(|s| s.troops)
            .filter(|t| t.unit.raw() == unit)
            .map(|t| t.men)
            .sum()
    };
    assert_eq!(aboard(1), 600);
    assert_eq!(aboard(2), 400);
    assert_eq!(g.unit_state(8, 1).men_ashore, 0);
    assert_eq!(g.unit_state(8, 2).men_ashore, 0);
    assert_eq!(g.base(8, 2).expect("base 2").side, Side::Japanese);
    // No base is formed at B.
    assert!(!g.events.iter().any(|e| matches!(
        &e.kind,
        EventKind::BaseFormed { hex, .. } if *hex == B
    )));
    assert_eq!(g.base(8, 1).expect("base 1").supply_t, n(1000));
    assert_eq!(g.base(8, 4).expect("base 4").supply_t, n(0));
}

// ---------------------------------------------------------------- The properties

#[test]
fn fed_it_goes() {
    let g = play(&thirty_aboard(&text()));
    goes_on_day_1(&g);
    assert_says(
        &g.landing_reason(1),
        &[
            "The staff wants 800 men; the lift by task_force:1 gives 600 fighting men: 600, short by 200",
            "30 t against the daily need of 5 t for the 4 days until supply follows, 5 t x 4 = 20 t, at least the 20 t they eat: the men can be fed",
            "So it goes short: its men can be fed where they wait.",
        ],
    );
}

#[test]
fn the_source_too_poor() {
    let t = changed(&thirty_aboard(&text()), "supply_t = 1000", "supply_t = 100");
    let g = play(&t);
    does_not_lift(&g, 1..=8);
    assert!(g.all_orders().is_empty(), "{:?}", g.all_orders());
    assert_says(
        &g.reasons_serving(1, 2),
        &[
            "base:1 has 100 t above its reserve against the 45 t + 75 t = 120 t wanted, short by 20 t",
            "So the men cannot be fed.",
        ],
    );
}

#[test]
fn no_count() {
    let g = play(&no_estimate(&text()));
    goes_on_day_1(&g);
}

#[test]
fn not_short() {
    let g = play(&estimate(&text(), 300));
    goes_on_day_1(&g);
}

#[test]
fn at_press_it_goes() {
    let g = play(&risk_on(&text(), 1, "press"));
    goes_on_day_1(&g);
    let g = play(&risk_on(&estimate(&text(), 800), 1, "press"));
    goes_on_day_1(&g);
}

#[test]
fn at_preserve_it_is_as_it_was() {
    let g = play(&risk_on(&text(), 1, "preserve"));
    assert!(g.all_orders().is_empty(), "{:?}", g.all_orders());
    assert!(g.landings_formed().is_empty(), "{:?}", g.landings_formed());
    assert!(
        g.cannot_bring_enough().is_empty(),
        "{:?}",
        g.cannot_bring_enough()
    );
    // What it hands in is what it handed in before the rule: at preserve
    // the force, too few men is no landing, with the notice that it cannot
    // meet the intent. (Read off the code as it stood before decision 0169,
    // on Nov 19, 2026: that notice is handed in on each of the 8 mornings.)
    for day in 1..=8 {
        assert_says(
            &g.reasons_serving(day, 2),
            &["At preserve the force the staff lands only with the men wanted: the ships can carry 600 men against the 1200 wanted"],
        );
    }
    let cannot_meet: Vec<u32> = (1..=8)
        .filter(|d| g.notices(*d).contains(&NoticeKind::CannotMeet))
        .collect();
    assert_eq!(cannot_meet, (1..=8).collect::<Vec<_>>());
}

#[test]
fn the_commander_says_go() {
    let g = play(&risk_on(&text(), 3, "press"));
    does_not_lift(&g, 1..=2);
    assert!(same(&g.orders(3), &lift_orders()), "{:?}", g.orders(3));
}

#[test]
fn more_men_in_reach() {
    let g = play(&force_2_from(&text(), 5));
    goes_on_day_1(&g);
}

#[test]
fn out_of_reach() {
    let g = play(&force_2_from(&text(), 10));
    does_not_lift(&g, 1..=2);
    assert_says(
        &g.reasons_serving(2, 2),
        &["The men in reach by day 9, the 7 days the staff looks ahead: 600"],
    );
    assert!(same(&g.orders(3), &lift_orders()), "{:?}", g.orders(3));
}

#[test]
fn the_same_start_gives_the_same_game() {
    let a = play(&text());
    assert_eq!(a.handed, main_game().handed);
    assert_eq!(a.hashes, main_game().hashes);
}

/// A game rebuilt from what was handed in to day `upto`, as a saved game
/// replays.
fn rebuilt(handed: &BTreeMap<u32, StaffDecisions>, upto: u32) -> Scenario {
    let mut scenario = load(&text());
    for day in 1..=upto {
        scenario
            .world
            .submit_staff(Side::Allied, handed[&day].clone());
        scenario
            .next_day_watched(Staffs::none(), &mut |_, _| {})
            .expect("a day resolves")
            .expect("a day to resolve");
    }
    scenario
}

#[test]
fn saved_and_replayed() {
    let through = main_game();
    let at_4 = rebuilt(&through.handed, 4);
    assert_eq!(at_4.world.day_hashes(), &through.hashes[..4]);
    let first: BTreeMap<u32, StaffDecisions> =
        (1..=4).map(|d| (d, through.handed[&d].clone())).collect();
    let resumed = play_from(at_4, first);
    assert_eq!(resumed.hashes, through.hashes);
    assert_eq!(resumed.handed, through.handed);
}
