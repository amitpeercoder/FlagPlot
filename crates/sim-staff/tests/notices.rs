//! The notices the supply staff hands in beside its orders, commands and
//! reasons (decision 0107, step 4, rulings 34 and 35): cannot meet, waits
//! for a leave, a run ended. The games are the scenario files 36, 40a, 40b,
//! 41a and 41b (and 36 with the leave to prepare withheld), played on the
//! frozen step 1 figures with the doctrine's staffs passed in. The days
//! each gives are the architect's; nothing here is tuned to pass.
//!
//! - 36: no notice on any day.
//! - 40a: plan 2 cannot meet on days 1 to 4 and plan 1 on days 10 to 13,
//!   and nothing else; a load cut that leaves the reading at or above the
//!   floor (intent 1, day 1) is no notice.
//! - 40b: plan 2 cannot meet on each of days 1 to 5 (day 1 by a cut load,
//!   resting on its `add_run`; days 2 to 5 with no run, on the no-order),
//!   plan 1 on none.
//! - 41a: plan 1 waits for leave to launch on day 2 only.
//! - A run set aside (41a with ship 1 held on day 2, and 41b) gives a
//!   `RunEnded` notice for that run on that day.
//! - Leave to prepare withheld: `WaitsForLeave(Prepare)` each day.
//! - Each notice's basis is a reason of the same plan.
//! - The standing interrupts fire on the first day of each run of notices.

use std::path::{Path, PathBuf};

use sim_core::command::Leave;
use sim_core::command::{
    InterruptRef, NoticeBasis, NoticeKind, PlanId, Reason, StaffDecisions, StaffNotice,
    StandingKind, StandingSubject,
};
use sim_core::events::EventKind;
use sim_core::state::Side;
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, staffs};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn file(name: &str) -> String {
    std::fs::read_to_string(root().join("data/scenarios").join(name)).expect("scenario reads")
}

fn plan(raw: u32) -> PlanId {
    PlanId::new(raw)
}

struct Played {
    handed: Vec<StaffDecisions>,
    /// The `StaffNotice` events: day, kind, plan, run.
    events: Vec<(u32, NoticeKind, PlanId, Option<u32>)>,
    /// The standing interrupts that fired: day, kind, subject.
    fired: Vec<(u32, StandingKind, StandingSubject)>,
}

fn play(name: &str, text: &str) -> Played {
    let path = root().join("data/scenarios").join(name);
    let options = LoadOptions::frozen_step1(&root());
    let mut scenario = Scenario::from_text_with(text, &path, &options).expect("scenario loads");
    let held = staffs();
    let mut handed = Vec::new();
    while scenario
        .next_day_watched(as_staffs(&held), &mut |_, _| {})
        .expect("a day resolves")
        .is_some()
    {
        handed.push(
            scenario
                .world
                .staff_handed_in()
                .get(&Side::Allied)
                .cloned()
                .unwrap_or_default(),
        );
    }
    let mut events = Vec::new();
    let mut fired = Vec::new();
    for event in scenario.world.events().iter() {
        match &event.kind {
            EventKind::StaffNotice {
                kind, plan, run, ..
            } => events.push((event.day, *kind, *plan, *run)),
            EventKind::InterruptFired {
                interrupt: InterruptRef::Standing { kind, subject },
                ..
            } => fired.push((event.day, *kind, *subject)),
            _ => {}
        }
    }
    Played {
        handed,
        events,
        fired,
    }
}

impl Played {
    fn day(&self, day: u32) -> &StaffDecisions {
        &self.handed[day as usize - 1]
    }

    /// The days on which `plan` has a notice of `kind`.
    fn days_of(&self, kind: NoticeKind, plan: PlanId) -> Vec<u32> {
        (1..=self.handed.len() as u32)
            .filter(|d| {
                self.day(*d)
                    .notices
                    .iter()
                    .any(|n| n.kind == kind && n.plan == plan)
            })
            .collect()
    }

    fn all_notices(&self) -> Vec<(u32, StaffNotice)> {
        let mut found = Vec::new();
        for d in 1..=self.handed.len() as u32 {
            for n in &self.day(d).notices {
                found.push((d, *n));
            }
        }
        found
    }

    /// The reason a notice rests on.
    fn reason_of(&self, day: u32, notice: &StaffNotice) -> &Reason {
        let d = self.day(day);
        match notice.rests_on {
            NoticeBasis::Command(i) => &d.commands[i].reason,
            NoticeBasis::NoOrder(i) => &d.no_orders[i],
        }
    }

    fn fired_days(&self, kind: StandingKind, plan: PlanId) -> Vec<u32> {
        self.fired
            .iter()
            .filter(|(_, k, s)| *k == kind && *s == StandingSubject::Plan(plan))
            .map(|(d, _, _)| *d)
            .collect()
    }

    /// Every notice rests on a reason of its own plan, and the engine's
    /// events are the notices handed in, in order.
    fn check_every_notice(&self) {
        for (day, notice) in self.all_notices() {
            let reason = self.reason_of(day, &notice);
            assert_eq!(
                reason.serves.plan,
                Some(notice.plan),
                "day {day}: {notice:?} rests on a reason of another plan"
            );
        }
        let handed: Vec<_> = self
            .all_notices()
            .iter()
            .map(|(d, n)| (*d, n.kind, n.plan, n.run))
            .collect();
        assert_eq!(handed, self.events, "the events are the notices handed in");
    }
}

fn cannot_meet(p: &Played, raw: u32) -> Vec<u32> {
    p.days_of(NoticeKind::CannotMeet, plan(raw))
}

#[test]
fn scenario_36_gives_no_notice() {
    let p = play("36_one_convoy.toml", &file("36_one_convoy.toml"));
    assert_eq!(p.handed.len(), 18);
    assert!(p.all_notices().is_empty());
    assert!(p.events.is_empty());
    assert!(p.fired.is_empty());
}

#[test]
fn scenario_40a_cannot_meet_on_the_days_the_hand_calculation_gives() {
    let name = "40a_one_pool_two_hexes.toml";
    let p = play(name, &file(name));
    assert_eq!(p.handed.len(), 16);
    assert_eq!(cannot_meet(&p, 2), vec![1, 2, 3, 4]);
    assert_eq!(cannot_meet(&p, 1), vec![10, 11, 12, 13]);
    // Nothing else, and every notice is a cannot-meet with no run.
    for (_, n) in p.all_notices() {
        assert_eq!(n.kind, NoticeKind::CannotMeet);
        assert_eq!(n.run, None);
    }
    assert_eq!(p.all_notices().len(), 8);
    // Day 1: intent 1's load is cut (36 t to 30 t) and still meets its
    // floor: no notice. Plan 2's notice has no run, so it rests on the
    // no-order.
    assert!(p.day(1).notices.iter().all(|n| n.plan == plan(2)));
    // The standing interrupt fires on the first day of each run.
    assert_eq!(p.fired_days(StandingKind::CannotMeet, plan(2)), vec![1]);
    assert_eq!(p.fired_days(StandingKind::CannotMeet, plan(1)), vec![10]);
    p.check_every_notice();
}

#[test]
fn scenario_40b_plan_2_cannot_meet_every_day_and_plan_1_never() {
    let name = "40b_one_stock_two_plans.toml";
    let p = play(name, &file(name));
    assert_eq!(p.handed.len(), 5);
    assert_eq!(cannot_meet(&p, 2), vec![1, 2, 3, 4, 5]);
    assert_eq!(cannot_meet(&p, 1), Vec::<u32>::new());
    assert_eq!(p.all_notices().len(), 5);
    // Day 1 rests on the run's add_run (a cut load); days 2 to 5 on a
    // no-order. On day 2 the staff dispatches run 1 and also needs a run it
    // cannot have: its no-order is about the next run.
    let day1 = p.day(1).notices[0];
    assert!(matches!(day1.rests_on, NoticeBasis::Command(i)
        if matches!(p.day(1).commands[i].kind, sim_core::command::CommandKind::AddRun { plan: pl, .. } if pl == plan(2))));
    for d in 2..=5 {
        assert!(
            matches!(p.day(d).notices[0].rests_on, NoticeBasis::NoOrder(_)),
            "day {d}"
        );
    }
    let day2 = p.day(2).notices[0];
    assert!(p.reason_of(2, &day2).worked.starts_with("No new run: "));
    assert!(!p.day(2).orders.is_empty());
    assert_eq!(p.fired_days(StandingKind::CannotMeet, plan(2)), vec![1]);
    p.check_every_notice();
}

#[test]
fn scenario_41a_plan_1_waits_for_leave_to_launch_on_day_2_only() {
    let name = "41a_hold_change_release.toml";
    let p = play(name, &file(name));
    assert_eq!(p.handed.len(), 27);
    assert_eq!(
        p.days_of(NoticeKind::WaitsForLeave(Leave::Launch), plan(1)),
        vec![2]
    );
    assert!(p.days_of(NoticeKind::CannotMeet, plan(1)).is_empty());
    assert!(p
        .days_of(NoticeKind::WaitsForLeave(Leave::Prepare), plan(1))
        .is_empty());
    assert_eq!(p.all_notices().len(), 1);
    assert_eq!(p.fired_days(StandingKind::WaitsForLeave, plan(1)), vec![2]);
    p.check_every_notice();
}

#[test]
fn a_run_set_aside_gives_a_run_ended_notice_for_that_run_that_day() {
    // 41a with ship 1 held on day 2 in place of day 8.
    let name = "41a_hold_change_release.toml";
    let text = file(name);
    let hold = "day = 8\nside = \"allied\"\nkind = \"hold\"";
    assert!(text.contains(hold), "the hold command is where it was");
    let text = text.replace(hold, "day = 2\nside = \"allied\"\nkind = \"hold\"");
    let p = play(name, &text);
    assert_eq!(p.days_of(NoticeKind::RunEnded, plan(1)), vec![2]);
    let ended: Vec<_> = p
        .day(2)
        .notices
        .iter()
        .filter(|n| n.kind == NoticeKind::RunEnded)
        .collect();
    assert_eq!(ended.len(), 1);
    assert_eq!(ended[0].run, Some(1));
    let NoticeBasis::Command(i) = ended[0].rests_on else {
        panic!("a run ended rests on its end_run command");
    };
    assert!(matches!(
        p.day(2).commands[i].kind,
        sim_core::command::CommandKind::EndRun { run: 1, .. }
    ));
    p.check_every_notice();
}

#[test]
fn an_override_that_sets_a_run_aside_gives_a_run_ended_notice() {
    let name = "41b_an_override.toml";
    let p = play(name, &file(name));
    assert_eq!(p.days_of(NoticeKind::RunEnded, plan(1)), vec![2]);
    let n = p
        .day(2)
        .notices
        .iter()
        .find(|n| n.kind == NoticeKind::RunEnded)
        .expect("a notice");
    assert_eq!(n.run, Some(1));
    p.check_every_notice();
}

#[test]
fn leave_to_prepare_withheld_gives_waits_for_leave_to_prepare_each_day() {
    // Scenario 36 with leave to prepare withheld at the start.
    let name = "36_one_convoy.toml";
    let text = file(name);
    let given = "leave = \"prepare\"\ngiven = true";
    assert!(text.contains(given), "scenario 36 gives leave to prepare");
    let text = text.replace(given, "leave = \"prepare\"\ngiven = false");
    let p = play(name, &text);
    let days: Vec<u32> = (1..=p.handed.len() as u32).collect();
    assert_eq!(
        p.days_of(NoticeKind::WaitsForLeave(Leave::Prepare), plan(1)),
        days
    );
    for (d, n) in p.all_notices() {
        assert_eq!(n.kind, NoticeKind::WaitsForLeave(Leave::Prepare), "day {d}");
        assert!(matches!(n.rests_on, NoticeBasis::NoOrder(_)));
    }
    assert_eq!(p.fired_days(StandingKind::WaitsForLeave, plan(1)), vec![1]);
    p.check_every_notice();
}
