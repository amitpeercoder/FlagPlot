//! Scenario 66: units made ready, and a base built up. The staff trains a
//! raw unit and then rehearses it, rehearses a sound one from the first
//! day, loads the transports, builds an airfield, and says when the
//! landing is ready. Nobody gives a unit its activity by hand.
//!
//! Proves decision 0141, step 2 of stage 3, against a hand calculation.
//! The data is placeholder data: see
//! `data/scenarios/66_units_made_ready.toml`. It runs on the frozen step 1
//! figures, with ground figures of its own that differ from the frozen
//! ones in three: `preparation_per_day` 25, `training_days_per_level` 3,
//! `airfield_work` 4000. There is no Japanese unit, so no die is rolled.
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
//! A force in port with its roster gains 25 of loading a day, to 100. A
//! unit that stands ready on that roster gains 25 of rehearsal a day, to
//! 100 (the assault skill's strength is 0 on the frozen figures). A unit
//! that trains counts a day, and at 3 its experience rises by one and the
//! count starts again; the standard is 2 and training goes no higher. A
//! man of a working unit of works skill 2 makes 1 of work a day; a
//! fighting unit set to work makes half that. An airfield takes 4,000.
//! Each is settled at the end of the day.
//!
//! ## Inputs
//!
//! A placeholder map. Allied base 1 at a coast hex, (0,0), with a port,
//! no airfield and no work done on one, and supply for every man for the
//! whole game. The game is 12 days.
//!
//! Task force 1, Allied: two placeholder transports, stopped in port at
//! base 1. Ashore at base 1, every man fed in full:
//!
//! | Unit | Kind | Men | Experience |
//! | --- | --- | --- | --- |
//! | 1, "Placeholder Raw" | fighting | 800 | 1 |
//! | 2, "Placeholder Sound" | fighting | 800 | 2 |
//! | 3, "Placeholder Builders" | working, works skill 2 | 1,000 | 2 |
//!
//! Every unit's activity at the start is what the rules give it: ready
//! for units 1 and 2, work for unit 3 with no job named.
//!
//! The Allied side's commands of day 1: intent 1, ready a landing by task
//! force 1 at (6,0), a coast hex, with a forces limit of units 1 and 2,
//! no risk set, so balanced, and no date; intent 2, build up base 1,
//! wanting an airfield; leave to prepare given for both.
//!
//! ## Day by day
//!
//! | Day | Orders in the morning | Loading after | Unit 1 after | Unit 2 rehearsal after | Airfield work after |
//! | --- | --- | --- | --- | --- | --- |
//! | 1 | `prepare_landing` at (6,0), roster 1 and 2; unit 1, train; unit 3, work on the airfield | 25 | trains: 1 day counted | 25 | 1,000 |
//! | 2 | none | 50 | 2 days | 50 | 2,000 |
//! | 3 | none | 75 | 3 days: experience 2, count 0 | 75 | 3,000 |
//! | 4 | unit 1, ready | 100 | rehearsal 25 | 100 | 4,000: the airfield is built |
//! | 5 | none | 100 | 50 | 100 | |
//! | 6 | none | 100 | 75 | 100 | |
//! | 7 | none | 100 | 100 | 100 | |
//! | 8 to 12 | none | 100 | 100 | 100 | |
//!
//! Four orders in all: three on day 1 and one on day 4.
//!
//! - **Day 1.** Unit 1 is at experience 1, below the standard of 2: at
//!   balanced with no date it trains first. Unit 2 is at the standard and
//!   stands ready already: no order. Unit 3 is given the airfield by name.
//! - **Day 4.** Unit 1 is at the standard: it is set to stand ready, and
//!   from that day rehearses.
//! - **Day 5.** The base has an airfield. Intent 2's one want is met: the
//!   staff gives no order, says so, and hands in the notice that intent 2
//!   is met, on that morning and on no other.
//! - **Day 8.** The loading is 100 and both units' rehearsal is 100: the
//!   staff hands in the notice that intent 1 is ready, on that morning
//!   and on no other.
//!
//! The reason for unit 1's order of day 1 gives its experience, 1, against
//! the standard, 2; 0 days counted of 3; and 4 days of rehearsal to come.
//! The reason for unit 3's order gives 4,000 of work to do, 1,000 a day,
//! 4 days.
//!
//! No staff order is refused.
//!
//! # Properties
//!
//! - **Press.** With intent 1 at press: unit 1 is not set to train. Day 1
//!   has two orders, the landing order and unit 3's job. Both units'
//!   rehearsal and the loading are 50 after day 2, and the notice that
//!   intent 1 is ready is handed in on the morning of day 3.
//! - **Preserve the force, and a raw unit.** With intent 1 at preserve the
//!   force and unit 1 at experience 0: it trains for 6 days, is at the
//!   standard after day 6, is set to stand ready on the morning of day 7,
//!   and its rehearsal is 100 after day 10. The notice that intent 1 is
//!   ready is handed in on the morning of day 11.
//! - **A date, at balanced.** With a date of the morning of day 6: on the
//!   morning of day 1 five days are left and unit 1's rehearsal needs 4:
//!   it trains. On the morning of day 2 four are left and it needs 4: it
//!   is set to stand ready, with 1 day of training counted and its
//!   experience still 1. Its rehearsal is 100 after day 5, and the notice
//!   that intent 1 is ready is handed in on the morning of day 6.
//! - **A date, at preserve the force.** With the same date and preserve
//!   the force: unit 1 trains to the standard as in the main case, and the
//!   task is ready on the morning of day 8. On the morning of day 1 the
//!   staff hands in the notice that it cannot be ready by its date: 2
//!   days late, and what binds is unit 1's training.
//! - **A fighting unit put to work.** With intent 2's forces limit naming
//!   units 2 and 3, and intent 1's naming unit 1 alone: unit 2 is set to
//!   work on the airfield on day 1 and makes 400 a day, half of 800. The
//!   work after days 1, 2 and 3 is 1,400, 2,800 and 4,200: the airfield
//!   is built on day 3.
//! - **One task a unit.** With unit 2 in both forces limits: intent 1,
//!   the higher, has it; it rehearses, and intent 2's reason says so.
//! - **A want already met.** With an airfield at base 1 at the start:
//!   intent 2 gives no order and its notice is handed in on day 1.
//! - **Defences to a number.** With intent 2 wanting an airfield and then
//!   defences of 10 points: from the morning of day 5 unit 3 is given the
//!   defences, one order, and makes 5 points a day at 200 of work a
//!   point: 10 after day 6, and the notice is handed in on the morning of
//!   day 7.
//! - **Without leave to prepare** no order is given and the reason says it
//!   waits for leave.
//! - **A hold and an override.** With unit 1 held from day 1 it is given
//!   no order and the staff says it plans without it. A commander's
//!   `set_activity` to unit 2 on day 2 holds it, and the staff gives it no
//!   order after.
//! - **Before its day.** With unit 2 available from day 3: it is given no
//!   order before day 3 and is not on the roster of the landing order of
//!   day 1; on day 3 the staff gives the landing order again with both
//!   units, and the loading is halved by the unit added, as decision 0095
//!   has it.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons;
//!   every scenario built before this one gives the events it gave.

// ---------------------------------------------------------------- The harness

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use sim_core::command::{NoticeKind, Reason, StaffDecisions, StaffOrder};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, LandUnitId, TaskForceId};
use sim_core::orders::{Activity, Job, LandUnitOrder, OrderKind, TaskForceOrder};
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/66_units_made_ready.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn unit(raw: u32) -> LandUnitId {
    LandUnitId::new(raw)
}

fn intent(raw: u32) -> IntentId {
    IntentId::new(raw)
}

const LANDING: Hex = Hex { q: 6, r: 0 };

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// The scenario's own ground figures: the frozen ones with three changed.
fn figures() -> sim_data::LoadOptions {
    common::altered_reference("s66", |dir| {
        common::edit_file_by(
            dir,
            "ground.toml",
            &[
                ("preparation_per_day = 5\n", "preparation_per_day = 25\n"),
                (
                    "training_days_per_level = 30\n",
                    "training_days_per_level = 3\n",
                ),
                ("airfield_work = 20000\n", "airfield_work = 4000\n"),
            ],
        );
    })
}

/// A game played to its end with the doctrine's staffs: what the Allied staff
/// handed in each day, the truth at the end of each day (index 0 is the
/// start) and the log.
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
}

fn play(text: &str) -> Played {
    let mut scenario: Scenario =
        common::from_text_with(text, &path(), &figures()).expect("the scenario loads");
    let held = staffs();
    let mut handed = BTreeMap::new();
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
    }
}

/// An order of the staff, plainly.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Landing(u32, Hex, Vec<u32>),
    Activity(u32, Activity),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce {
            target,
            order: TaskForceOrder::PrepareLanding { hex, roster },
        } => Given::Landing(target.raw(), *hex, roster.iter().map(|u| u.raw()).collect()),
        OrderKind::LandUnit {
            target,
            order: LandUnitOrder::SetActivity(activity),
        } => Given::Activity(target.raw(), *activity),
        other => panic!("the doctrine gave an order of another kind: {other:?}"),
    }
}

fn train(u: u32) -> Given {
    Given::Activity(u, Activity::Train)
}

fn ready(u: u32) -> Given {
    Given::Activity(u, Activity::Ready)
}

fn work(u: u32, job: Job) -> Given {
    Given::Activity(u, Activity::Work(Some(job)))
}

fn landing(roster: &[u32]) -> Given {
    Given::Landing(1, LANDING, roster.to_vec())
}

impl Played {
    fn days(&self) -> u32 {
        self.handed.len() as u32
    }

    fn orders(&self, day: u32) -> Vec<Given> {
        self.handed[&day].orders.iter().map(given).collect()
    }

    /// Every order of the game with its day.
    fn all_orders(&self) -> Vec<(u32, Given)> {
        (1..=self.days())
            .flat_map(|d| self.orders(d).into_iter().map(move |g| (d, g)))
            .collect()
    }

    fn order_reason(&self, day: u32, wanted: &Given) -> &Reason {
        let found = self.handed[&day]
            .orders
            .iter()
            .find(|o| given(o) == *wanted)
            .unwrap_or_else(|| panic!("day {day} has the order {wanted:?}"));
        &found.reason
    }

    /// The days on which a notice of `kind` was handed in for the plan of
    /// `intent` (each intent here is a root with its own plan, numbered as
    /// the intent is).
    fn notice_days(&self, kind: NoticeKind, of: u32) -> Vec<u32> {
        (1..=self.days())
            .filter(|d| {
                self.handed[d]
                    .notices
                    .iter()
                    .any(|n| n.kind == kind && n.plan.raw() == of)
            })
            .collect()
    }

    /// The reason a notice of `kind` on `day` rests on.
    fn notice_reason(&self, day: u32, kind: NoticeKind) -> &Reason {
        let decisions = &self.handed[&day];
        let notice = decisions
            .notices
            .iter()
            .find(|n| n.kind == kind)
            .unwrap_or_else(|| panic!("day {day} has a {kind:?} notice"));
        match notice.rests_on {
            sim_core::command::NoticeBasis::NoOrder(i) => &decisions.no_orders[i],
            sim_core::command::NoticeBasis::Command(i) => &decisions.commands[i].reason,
        }
    }

    /// The no-order reasons of `day` serving `of`.
    fn no_orders(&self, day: u32, of: u32) -> Vec<&Reason> {
        self.handed[&day]
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(intent(of)))
            .collect()
    }

    fn loading(&self, day: u32) -> Fixed {
        self.truth[day as usize].task_forces[&TaskForceId::new(1)]
            .preparation
            .as_ref()
            .map_or(Fixed::ZERO, |p| p.loading)
    }

    fn unit_state(&self, day: u32, u: u32) -> &sim_core::state::LandUnit {
        &self.truth[day as usize].land_units[&unit(u)]
    }

    fn rehearsal(&self, day: u32, u: u32) -> Fixed {
        self.unit_state(day, u).rehearsal
    }

    fn airfield_work(&self, day: u32) -> Fixed {
        self.truth[day as usize].bases[&BaseId::new(1)].airfield_work
    }

    fn has_airfield(&self, day: u32) -> bool {
        self.truth[day as usize].bases[&BaseId::new(1)].airfield
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

fn assert_says(reason: &Reason, parts: &[&str]) {
    let all = text_of(reason);
    for part in parts {
        assert!(all.contains(part), "the reason says {part:?}: {all}");
    }
}

/// The scenario's text with `extra` added after the commander's commands.
fn with_commands(extra: &str) -> String {
    format!("{}\n{extra}", text())
}

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

// ---------------------------------------------------------------- The hand calculation

#[test]
fn day_by_day_the_loading_the_units_and_the_airfield() {
    let g = main_game();
    assert_eq!(g.days(), 12);
    let loading = [25, 50, 75, 100, 100, 100, 100, 100, 100, 100, 100, 100];
    let unit2 = [25, 50, 75, 100, 100, 100, 100, 100, 100, 100, 100, 100];
    for day in 1..=12u32 {
        let i = (day - 1) as usize;
        assert_eq!(g.loading(day), n(loading[i]), "loading after day {day}");
        assert_eq!(g.rehearsal(day, 2), n(unit2[i]), "unit 2 after day {day}");
    }
    // Unit 1: trains days 1 to 3, then rehearses.
    for (day, days, experience) in [(1, 1, 1), (2, 2, 1), (3, 0, 2)] {
        let u = g.unit_state(day, 1);
        assert_eq!(
            (u.training_days, u.experience),
            (days, experience),
            "day {day}"
        );
        assert_eq!(u.rehearsal, Fixed::ZERO, "day {day}");
    }
    for (day, rehearsal) in [(4, 25), (5, 50), (6, 75), (7, 100), (8, 100), (12, 100)] {
        assert_eq!(g.rehearsal(day, 1), n(rehearsal), "unit 1 after day {day}");
        assert_eq!(g.unit_state(day, 1).experience, 2);
    }
    for (day, done) in [(1, 1000), (2, 2000), (3, 3000), (4, 4000)] {
        assert_eq!(
            g.airfield_work(day),
            n(done),
            "airfield work after day {day}"
        );
    }
    assert!(!g.has_airfield(3));
    assert!(g.has_airfield(4), "the airfield is built on day 4");
}

#[test]
fn four_orders_in_all() {
    let g = main_game();
    assert_eq!(
        g.orders(1),
        vec![landing(&[1, 2]), train(1), work(3, Job::Airfield)]
    );
    assert_eq!(g.orders(4), vec![ready(1)]);
    assert_eq!(g.all_orders().len(), 4);
    assert!(g.refused().is_empty(), "no staff order is refused");
}

#[test]
fn intent_2_is_met_on_day_5_and_only_then_noticed() {
    let g = main_game();
    assert_eq!(g.notice_days(NoticeKind::Met, 2), vec![5]);
    for day in 5..=12 {
        assert!(
            !g.all_orders()
                .iter()
                .any(|(d, o)| *d == day && *o == work(3, Job::Airfield)),
            "no order for intent 2 on day {day}"
        );
        assert_eq!(
            g.no_orders(day, 2).len(),
            1,
            "the staff says so on day {day}"
        );
    }
}

#[test]
fn intent_1_is_ready_on_day_8_and_only_then_noticed() {
    let g = main_game();
    assert_eq!(g.notice_days(NoticeKind::Ready, 1), vec![8]);
    assert!(g.notice_days(NoticeKind::Late, 1).is_empty());
}

#[test]
fn the_reasons_carry_the_numbers() {
    let g = main_game();
    assert_says(
        g.order_reason(1, &train(1)),
        &[
            "experience 1",
            "standard of 2",
            "0 of the 3 days of training",
            "needs 4 days",
        ],
    );
    assert_says(
        g.order_reason(1, &work(3, Job::Airfield)),
        &["4000 of work to do", "1000 a day", "4 days"],
    );
}

// ---------------------------------------------------------------- Properties

#[test]
fn press() {
    let g = play(&with_commands(
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"press\"\n",
    ));
    assert_eq!(g.orders(1), vec![landing(&[1, 2]), work(3, Job::Airfield)]);
    assert!(!g.all_orders().iter().any(|(_, o)| *o == train(1)));
    assert_eq!(g.loading(2), n(50));
    assert_eq!(g.rehearsal(2, 1), n(50));
    assert_eq!(g.rehearsal(2, 2), n(50));
    assert_eq!(g.notice_days(NoticeKind::Ready, 1), vec![3]);
}

#[test]
fn preserve_the_force_and_a_raw_unit() {
    let t = changed(&text(), "experience = 1\n", "experience = 0\n");
    let g = play(&format!(
        "{t}\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"preserve\"\n"
    ));
    assert!(g.orders(1).contains(&train(1)));
    for day in 1..=5 {
        assert!(g.unit_state(day, 1).experience < 2, "below after day {day}");
    }
    assert_eq!(
        g.unit_state(6, 1).experience,
        2,
        "at the standard after day 6"
    );
    assert_eq!(g.orders(7), vec![ready(1)]);
    assert_eq!(g.rehearsal(10, 1), n(100));
    assert_eq!(g.rehearsal(9, 1), n(75));
    assert_eq!(g.notice_days(NoticeKind::Ready, 1), vec![11]);
}

fn dated(t: &str) -> String {
    changed(t, "hex = [6, 0]\n", "hex = [6, 0]\nby_day = 6\n")
}

#[test]
fn a_date_at_balanced() {
    let g = play(&dated(&text()));
    assert!(g.orders(1).contains(&train(1)));
    assert_eq!(g.orders(2), vec![ready(1)]);
    let u = g.unit_state(1, 1);
    assert_eq!((u.training_days, u.experience), (1, 1));
    assert_eq!(g.unit_state(2, 1).training_days, 1);
    assert_eq!(g.rehearsal(5, 1), n(100));
    assert_eq!(g.rehearsal(4, 1), n(75));
    assert_eq!(g.notice_days(NoticeKind::Ready, 1), vec![6]);
}

#[test]
fn a_date_at_preserve_the_force() {
    let g = play(&format!(
        "{}\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"preserve\"\n",
        dated(&text())
    ));
    assert!(g.orders(1).contains(&train(1)));
    assert_eq!(g.orders(4), vec![ready(1)]);
    assert_eq!(g.notice_days(NoticeKind::Ready, 1), vec![8]);
    assert_eq!(g.notice_days(NoticeKind::Late, 1), vec![1]);
    let reason = g.notice_reason(1, NoticeKind::Late);
    assert_says(reason, &["2 days late", "the training of land_unit:1"]);
    assert_eq!(
        reason.binding,
        vec!["the training of land_unit:1 (Placeholder Raw)"]
    );
}

#[test]
fn a_fighting_unit_put_to_work() {
    let t = changed(&text(), "land_units = [1, 2]\n", "land_units = [1]\n");
    let g = play(&format!(
        "{t}\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\nland_units = [2, 3]\n"
    ));
    assert!(g.orders(1).contains(&work(2, Job::Airfield)));
    assert!(g.orders(1).contains(&work(3, Job::Airfield)));
    assert!(g.orders(1).contains(&landing(&[1])));
    for (day, done) in [(1, 1400), (2, 2800), (3, 4200)] {
        assert_eq!(g.airfield_work(day), n(done), "after day {day}");
    }
    assert!(!g.has_airfield(2));
    assert!(g.has_airfield(3));
    // Unit 2's own day of work: 400, half of 800.
    let made = g
        .events
        .iter()
        .find_map(|e| match &e.kind {
            EventKind::UnitWorked { unit: u, work, .. } if *u == unit(2) && e.day == 1 => {
                Some(*work)
            }
            _ => None,
        })
        .expect("unit 2 worked on day 1");
    assert_eq!(made, n(400));
}

#[test]
fn one_task_a_unit() {
    let g = play(&with_commands(
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\nland_units = [2, 3]\n",
    ));
    assert!(g.orders(1).contains(&landing(&[1, 2])));
    assert!(!g
        .all_orders()
        .iter()
        .any(|(_, o)| matches!(o, Given::Activity(2, _))));
    assert_eq!(g.rehearsal(1, 2), n(25), "unit 2 rehearses");
    assert_says(
        g.order_reason(1, &work(3, Job::Airfield)),
        &["land_unit:2 (Placeholder Sound) serves intent:1"],
    );
}

#[test]
fn a_want_already_met() {
    let t = changed(&text(), "airfield = false\n", "");
    let g = play(&t);
    assert!(!g
        .all_orders()
        .iter()
        .any(|(_, o)| matches!(o, Given::Activity(3, _))));
    assert_eq!(g.notice_days(NoticeKind::Met, 2), vec![1]);
}

#[test]
fn defences_to_a_number() {
    let t = changed(
        &text(),
        "wants = [\"airfield\"]\n",
        "wants = [\"airfield\", \"defences\"]\ndefences = 10\n",
    );
    let g = play(&t);
    let to_defences: Vec<u32> = g
        .all_orders()
        .into_iter()
        .filter(|(_, o)| *o == work(3, Job::Defences))
        .map(|(d, _)| d)
        .collect();
    assert_eq!(to_defences, vec![5]);
    let defences = |day: u32| g.truth[day as usize].bases[&BaseId::new(1)].defences;
    assert_eq!(defences(5), n(5));
    assert_eq!(defences(6), n(10));
    assert_eq!(g.notice_days(NoticeKind::Met, 2), vec![7]);
}

#[test]
fn without_leave_to_prepare() {
    let t = changed(&text(), "given = true\n", "given = false\n");
    let g = play(&t);
    assert!(g.all_orders().is_empty());
    for of in [1, 2] {
        let says = g.handed[&1]
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(intent(of)))
            .any(|r| {
                r.worked.contains("waits for him")
                    && r.binding.iter().any(|b| b.contains("leave to prepare"))
            });
        assert!(says, "intent {of}'s reason says it waits for leave");
    }
}

#[test]
fn a_hold_and_an_override() {
    let g = play(&with_commands(concat!(
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"hold\"\nland_unit = 1\n\n",
        "[[order]]\nday = 2\nside = \"allied\"\nland_unit = 2\nkind = \"set_activity\"\nactivity = \"train\"\n",
    )));
    assert!(!g
        .all_orders()
        .iter()
        .any(|(_, o)| matches!(o, Given::Activity(1, _))));
    assert!(g.orders(1).contains(&landing(&[2])));
    let says = g.handed[&1]
        .orders
        .iter()
        .map(|o| &o.reason)
        .chain(g.handed[&1].no_orders.iter())
        .filter(|r| r.serves.intent == Some(intent(1)))
        .any(|r| {
            text_of(r).contains("the staff plans without it") && text_of(r).contains("land_unit:1")
        });
    assert!(says, "the staff says it plans without unit 1");
    // Unit 2 is held from day 2 and given nothing after.
    assert!(!g
        .all_orders()
        .iter()
        .any(|(d, o)| *d >= 2 && matches!(o, Given::Activity(2, _))));
    assert_eq!(g.unit_state(2, 2).activity, Activity::Train);
    assert_eq!(g.unit_state(12, 2).activity, Activity::Train);
    assert!(g.refused().is_empty());
}

#[test]
fn before_its_day() {
    let t = changed(
        &text(),
        "experience = 2\n",
        "experience = 2\navailable_from = 3\n",
    );
    let g = play(&t);
    assert_eq!(g.orders(1)[0], landing(&[1]));
    for day in 1..=2 {
        assert!(!g
            .all_orders()
            .iter()
            .any(|(d, o)| *d == day && matches!(o, Given::Activity(2, _))));
    }
    assert_eq!(g.orders(3), vec![landing(&[1, 2])]);
    // 25 and 50 after days 1 and 2; halved to 25 on day 3 and 50 after it.
    assert_eq!(g.loading(2), n(50));
    assert_eq!(g.loading(3), n(50));
    let lost = g.events.iter().any(|e| {
        e.day == 3
            && matches!(&e.kind, EventKind::LoadingLost { loading_before, loading, .. }
                if *loading_before == n(50) && *loading == n(25))
    });
    assert!(lost, "the loading is halved by the unit added on day 3");
}

#[test]
fn the_same_start_gives_the_same_orders() {
    let a = play(&text());
    let b = play(&text());
    assert_eq!(a.handed, b.handed);
}

#[test]
fn what_the_side_cannot_see_changes_nothing() {
    let enemy = |men: u32| {
        format!(
            "[[land_unit]]\nid = 9\nname = \"Placeholder enemy\"\nside = \"japanese\"\nmen = {men}\nq = 6\nr = 0\n"
        )
    };
    let place = |men: u32| {
        changed(
            &text(),
            COMMANDS_MARKER,
            &format!("{}\n{COMMANDS_MARKER}", enemy(men)),
        )
    };
    let a = play(&place(100));
    let b = play(&place(300));
    assert_eq!(a.handed, b.handed);
}

#[test]
fn a_game_with_neither_task_gives_nothing() {
    let cut = text()
        .split(COMMANDS_MARKER)
        .next()
        .expect("a part")
        .to_string();
    let g = play(&cut);
    for day in 1..=12 {
        assert!(g.handed[&day].is_empty(), "nothing handed in on day {day}");
    }
    assert_eq!(g.unit_state(12, 1).activity, Activity::Ready);
    let roster: BTreeSet<LandUnitId> = BTreeSet::new();
    assert!(g.truth[12].task_forces[&TaskForceId::new(1)]
        .preparation
        .as_ref()
        .is_none_or(|p| p.roster == roster));
}
