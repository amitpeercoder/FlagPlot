//! Scenario 132: a source the staff chooses (decision 0228).
//!
//! Written from the architect's record, word for word: the setup, the hand
//! calculation, the statements (as amended on review and by the rulings on the
//! build) and what must always hold. Nobody but the architect changes a
//! statement or an expected number; a mismatch goes back to the architect.
//!
//! # The setup and the hand calculation
//!
//! ### The setup
//!
//! - **The map.** Open sea, `q` from -3 to 9 and `r` from -1 to 9, with four coast hexes: H at (0,0), port size 0; N at (-2,0), F at (0,8) and G at (8,0), each port size 5. Each lies on an axis from H, where a leg between centres is charged 40 nm (decision 0058): N is 2 hexes, 80 nm; F and G are 8 hexes, 320 nm each.
//! - **The figures.** A ground file with `daily_supply_t_per_man = "0.005"` and no figures of the air, so the only planned use is the men ashore.
//! - **The Allied side.** Base 1 at H with 20 t of supply and land unit 1, 1,000 men ashore. Base 2 at N with 200 t and land unit 2, 2,000 men ashore; task force 2 stopped there, one placeholder troop transport (class 10), nothing aboard. Base 3 at F with 2,000 t and no men; task force 1 stopped there, two such transports, nothing aboard. Base 4 at G with 5,000 t, no men and no force. No base has a reserve.
//! - **The order.** On day 1 the commander states "hold base 1" with no limit, and gives both leaves. Ten days. No enemy acts.
//!
//! ### The hand calculation
//!
//! On the first morning, before anything is charged:
//!
//! | Base | Supply | Own use a day | Keeps (15 days) | To spare | Covers the place's 75 t | A transport force empty there | Sea miles to H |
//! | --- | --- | --- | --- | --- | --- | --- | --- |
//! | 1, H, the place | 20 t | 1,000 x 0.005 = 5 t | | | | | |
//! | 2, N | 200 t | 2,000 x 0.005 = 10 t | 150 t | 50 t | no | task force 2 | 80 |
//! | 3, F | 2,000 t | 0 | 0 | 2,000 t | yes | task force 1 | 320 |
//! | 4, G | 5,000 t | 0 | 0 | 5,000 t | yes | none | 320 |
//!
//! - **The place:** 20 t over 5 t a day is 4 days of supply. Its need is 15 x 5 = 75 t.
//! - **File a, the choice:** bases 3 and 4 cover the need and base 2 does not; of those two base 3 has a force and base 4 none. The source is base 3, with task force 1. The rule as it was (most above the reserve) gives base 4, which no ship lies at.
//! - **File c, the load.** Bases 1 and 2 alone: base 2 is the only base that may be chosen and is the source. The supply is formed on the morning of day 1 and served from the morning of day 2 (Holding a place, The supply that follows). By then base 2's men have eaten one day, 10 t: it holds 190 t, keeps 150 t and can spare 40 t. Base 1 holds 15 t that morning against a floor of 5 days at balanced, 25 t, so a run is needed at once, and its load before any cut is at least the 75 t of the interval. Cut to what the source can spare, the first run's load is **40 t**. This number rests on two readings: that the first run is ordered on the morning of day 2, and that one day's use, 10 t, has been charged at base 2 by then. If the build finds either otherwise it stops and says which; the number is the architect's to restate.
//! - **File d, nothing to spare.** Bases 1 and 2 alone, base 2 with 150 t: it keeps 150 t and can spare 0. No base may be chosen. The place lives on what it has: **4 days**.
//! - **File i, a base in the place's own hex.** Bases 1 and 2 as in file d, and base 5 at H with 900 t. Base 5 is not weighed. No base may be chosen. The hex holds 920 t against 5 t a day: **184 days**.
//!
//! # The files and the statements
//!
//! ### The files and the statements
//!
//! Each statement is tested as written. Nobody but the architect changes one or an expected number.
//!
//! File a, the setup as above:
//!
//! 1. On the morning of day 1 the staff forms "keep supplied" for base 1 under the hold, and sets its forces limit to source base 3 and task force 1.
//! 2. The reason of the choice carries, as evidence, each base weighed with its figures: base 2, 50 t to spare, less than the place's 75 t, task force 2 there, 80 nm; base 3, 2,000 t, task force 1 there, 320 nm; base 4, 5,000 t, no transport force there, 320 nm. It says the source is base 3.
//! 3. The supply has a plan of its own. The hold's plan is released on day 1 and is not failed on any day.
//! 4. A run of task force 1 sails from base 3 and unloads supply at H within the ten days. Base 4 holds 5,000 t at the end. Task force 2 is given no order.
//! 5. On every morning from day 2 the source is base 3, and `command_state` gives the hold `supplied_from` base 3, `chosen_by` `staff`.
//!
//! File b, file a without base 3 and task force 1, and with task force 3 (one such transport, nothing aboard) at base 4 from day 3:
//!
//! 6. On day 1 the source is base 4, though task force 2 lies at base 2: base 2 cannot spare the place's 75 t. The supply's forces limit names base 4 and no force.
//! 7. On day 2 the supply is served, says that no ship of its pool is afloat and sends nothing. Neither its plan nor the hold's ends.
//! 8. On day 3 the staff adds task force 3 to the supply's forces limit, and the first run is ordered on day 4 (Controlling an island, The source kept, and its pool: a force joins by a command of the morning it is found and serves from the morning after).
//!
//! File c, bases 1 and 2 alone:
//!
//! 9. The source is base 2 with task force 2. The first run is ordered on day 2 with a load of 40 t, and its reason says the load is cut to what the source can spare.
//! 10. No second run is ordered. From the morning base 2 has nothing to spare, the reason of each day the supply cannot be met says so and names no other base.
//!
//! File d, file c with base 2 at 150 t:
//!
//! 11. No supply is formed. The hold's reason says no other base of the side can spare supply, names base 2 with 150 t held and 150 t kept, and says base 1 lives on what it has: 4 days of supply today.
//!
//! File e, file a with the commander's forces limit on the hold, on day 1, naming source base 4:
//!
//! 12. The staff chooses nothing. The supply reads base 4 as its source, and `supplied_from` says base 4, `chosen_by` `commander`.
//!
//! File f, file a played for twenty days, in which on day 3 the commander names source base 2 on the hold, and on day 16 clears it:
//!
//! 13. While a run from base 3 is loading or under way the source is base 3, and the reason of each such morning names the run the change waits for.
//! 14. On the morning of day 3, the day the commander names base 2, his command being taken in before the staff is asked, the staff sets the supply's forces limit to source base 2 and task force 2, and task force 1 is no longer in it; `supplied_from` says base 2, `chosen_by` `commander`.
//! 15. The change is taken up before day 16. From day 17 the source is still base 2: the staff does not choose again.
//!
//! File g, file a with task force 3 at base 4 from day 3:
//!
//! 16. On every morning the source is base 3, though from day 3 base 4 would be the first by rule 4. The reason says the source was chosen on an earlier morning and kept.
//!
//! File h, file a with island 1 of one place, base 1, and "control island 1" stated in place of the hold:
//!
//! 17. The island's hold of base 1 is given a forces limit of source base 3 and task force 1.
//!
//! File i, file d with base 5 at H holding 900 t:
//!
//! 18. No supply is formed. The reason names base 2 as in statement 11 and does not name base 5. It says base 1 lives on what it has: 184 days of supply today.
//!
//! ### What must always hold
//!
//! Property tests beside the files, each over the setup with its figures varied:
//!
//! - **More supply at a base never puts it later in the order** of rule 4, and a transport force added at a base never does.
//! - **A base in the place's hex is never chosen,** whatever it holds.
//! - **A source the commander names is always the source.**
//! - **A run from a source the staff chose never leaves the source, on the morning it is ordered, below fifteen days of its own use or below its reserve.**
//! - **Supply is conserved** with no enemy: what leaves a source is aboard a ship, ashore at the place or eaten there.
//! - **Asked twice on the same view, the staff chooses the same base.**
//!
//! # Amended on review, Nov 23, 2026 (the hand calculation's unstated figures, the statements restated)
//!
//! ### E. The hand calculation's unstated figures
//!
//! The numbers hold on these, now stated in the setup: every skill of land units 1 and 2 is 2, so each skill's factor is 1 and a man's charge is 0.005 t a day whatever the ground file's strengths; the placeholder troop transport (class 10) has room for 3,000 t, so no load in the files is cut by the ships' room; no base has a reserve and no air figures are loaded.
//!
//! ### The statements, restated
//!
//! Every file states the key for the Allied side, but files j and k. Statements 1, 2, 4 to 6, 8 to 11 and 16 to 18 stand as written. Restated by the architect, Nov 23, 2026:
//!
//! - **3.** The supply serves the hold's plan and has none of its own. The hold's plan is released on day 1 and is not marked failed on any day.
//! - **7.** On day 2 the supply is served, says that no ship of its pool is afloat and sends nothing. The hold's plan is not marked failed.
//! - **12.** The staff chooses nothing. It sets the supply's forces limit to source base 4 and no force, since none lies there; `supplied_from` says base 4, `chosen_by` `commander`. The supply sends nothing, and the hold's plan is not marked failed.
//! - **13.** Withdrawn.
//! - **14.** (Restated by the architect, Nov 23, 2026, decision 0228, the rulings after the build, 4.) On the morning of day 3, the day the commander names base 2, his command being taken in before the staff is asked, the staff sets the supply's forces limit to source base 2 and task force 2, and task force 1 is no longer in it; `supplied_from` says base 2, `chosen_by` `commander`.
//! - **15.** From day 17 the source is still base 2 and `chosen_by` is `staff`: the staff does not choose again.
//!
//! Added:
//!
//! - **19.** File j, file a without the key: no supply is formed on any day, and the hold's reason on day 1 is the one `supply-58` gives, naming bases 2, 3 and 4 with 200 t, 2,000 t and 5,000 t of supply above their reserves.
//! - **20.** File k, file h without the key: the island's hold is given source base 4 and no force, as today.
//!
//! Added to what must always hold: two bases of a side in one hex can never together spare more than their hex can; and a side without the key gets, on any setup, the orders `supply-58` gave.
//!
//! # The architect's ruling on the build, Nov 23, 2026 (statements 21 to 25)
//!
//! The tests of intake 21 to 25 are in `crates/sim-core/src/command/tests.rs`, beside
//! those of `forces_narrow`; their statements, as the record has them:
//!
//! - **Statements.** 1, 6, 8, 9, 12 and 14 rest on the change and none is restated. Added, as tests of intake in `sim-core` beside those of `forces_narrow`:
//!   - **21.** For a side without the key, a forces limit naming a source and a task force, set on a "keep supplied" under a hold with no forces limit, is refused as wider than its parent, as today.
//!   - **22.** For a side with the key, the same limit is taken.
//!   - **23.** For a side with the key, under a hold whose forces limit names source base 4, a limit on the "keep supplied" naming source base 3 is refused as wider than its parent, and one naming base 4 and a task force is taken.
//!   - **24.** For a side with the key, a forces limit set on a cover formed under the same hold is checked as today.
//! - **Rulebook:** chapter 8, The intent tree (a limit must narrow) and The forces limit on a staff's orders, say the keyed case.
//!
//! "The engine changes in one place" above is replaced by: the engine changes in two places, the key and this check.
//!
//! - **Statements:** 4, 8, 9 and 14 rest on it and none is restated. Added, a test of intake: **25.** For a side with the key, a run of a hold's plan is checked against the forces limit of the "keep supplied" in force under the hold (taken where its ships are in that limit, refused where they are not); for a side without the key, with no limit on the hold, it is refused as having no forces limit, as today.

#![allow(clippy::doc_lazy_continuation, clippy::doc_markdown)]

use crate::common;

use std::collections::BTreeMap;

use crate::common::values::{at, base, force, intent};
use sim_core::command::PlanId;
use sim_core::command::{
    CommandKind, CommandState, IntentKind, Limit, Maker, PlanState, Reason, StaffDecisions,
};
use sim_core::events::EventKind;
use sim_core::ids::{BaseId, IntentId, TaskForceId};
use sim_core::orders::{OrderKind, TaskForceOrder};
use sim_core::state::{Side, TruthState};
use sim_staff::{as_staffs, staffs};

fn file() -> std::path::PathBuf {
    common::data_dir()
        .join("scenarios/a-source-the-staff-chooses/132_a_source_the_staff_chooses.toml")
}

fn text() -> String {
    std::fs::read_to_string(file()).expect("the scenario file reads")
}

/// `text` without the block that follows the mark `# MARK: name`, up to the
/// next mark.
fn without(text: &str, name: &str) -> String {
    let mark = format!("# MARK: {name}\n");
    let start = text.find(&mark).expect("the mark is there") + mark.len();
    let end = text[start..]
        .find("# MARK:")
        .map_or(text.len(), |at| start + at);
    format!("{}{}", &text[..start], &text[end..])
}

fn command(day: u32, body: &str) -> String {
    format!("[[command]]\nday = {day}\nside = \"allied\"\n{body}\n\n")
}

/// The commander's forces limit on the hold, naming source `source`.
fn names_source(text: &str, day: u32, source: u32) -> String {
    at(
        text,
        "commands",
        &command(
            day,
            &format!("kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nsource = {source}"),
        ),
    )
}

/// Task force 3, one transport with nothing aboard, at base 4 (G), usable
/// from day `from`.
fn with_force_3(text: &str, from: u32) -> String {
    at(
        text,
        "forces",
        &format!(
            "[[task_force]]\nid = 3\nname = \"Placeholder transport force 3\"\nside = \"allied\"\nat_base = 4\nmission = \"transport\"\navailable_from = {from}\n\n[[ship]]\nid = 4\nname = \"Placeholder troop transport 4\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000\ntask_force = 3\n\n"
        ),
    )
}

/// File b: file a without base 3 and task force 1, with task force 3.
fn file_b() -> String {
    with_force_3(&without(&without(&text(), "base 3"), "force 1"), 3)
}

/// File c: bases 1 and 2 alone.
fn file_c() -> String {
    without(&without(&without(&text(), "base 3"), "base 4"), "force 1")
}

/// File d: file c with base 2 at 150 t.
fn file_d() -> String {
    common::replace_once(&file_c(), "supply_t = 200", "supply_t = 150")
}

/// Base 5 at `q`, `r` with `supply` tons.
fn base_5(text: &str, q: i32, r: i32, supply: u32) -> String {
    at(
        text,
        "bases",
        &format!(
            "[[base]]\nid = 5\nname = \"Placeholder base 5\"\nside = \"allied\"\nq = {q}\nr = {r}\nsupply_t = {supply}\n\n"
        ),
    )
}

/// File h: file a with island 1 of one place, base 1, and "control island 1"
/// in place of the hold.
fn file_h() -> String {
    let t = common::replace_once(
        &text(),
        "intent_kind = \"hold\"\nbase = 1",
        "intent_kind = \"control_island\"\nisland = 1",
    );
    at(
        &t,
        "islands",
        "[[island]]\nnumber = 1\nname = \"Placeholder island\"\nplaces = [{ base = 1 }]\n\n",
    )
}

/// Without the key.
fn no_key(text: &str) -> String {
    without(text, "side")
}

struct Played {
    /// What the Allied staff handed in on each day.
    handed: BTreeMap<u32, StaffDecisions>,
    /// The Allied command state at the end of each day; index 0 is the start.
    states: Vec<CommandState>,
    truth: Vec<TruthState>,
    refusals: Vec<String>,
}

fn play(text: &str, days: u32) -> Played {
    let mut scenario =
        common::from_text_with(text, &file(), &common::frozen()).expect("the scenario loads");
    let held = staffs();
    let mut handed = BTreeMap::new();
    let mut states = vec![scenario.world.command_state(Side::Allied).clone()];
    let mut truth = vec![scenario.world.umpire().clone()];
    loop {
        let day = scenario.world.day() + 1;
        if day > days {
            break;
        }
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
        states.push(scenario.world.command_state(Side::Allied).clone());
        truth.push(scenario.world.umpire().clone());
    }
    let refusals = scenario
        .world
        .events()
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::CommandRefused {
                reason, command, ..
            } => Some(format!("{reason:?} {command:?}")),
            _ => None,
        })
        .collect();
    Played {
        handed,
        states,
        truth,
        refusals,
    }
}
fn words(reason: &Reason) -> String {
    let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
    format!("{} | {}", reason.worked, evidence.join(" | "))
}

impl Played {
    /// The supply formed on `day`, if any: its number is 2 in these files.
    fn formed(&self, day: u32) -> Vec<IntentKind> {
        self.handed[&day]
            .commands
            .iter()
            .filter_map(|c| match c.kind {
                CommandKind::FormIntent {
                    kind: kind @ IntentKind::KeepSupplied { .. },
                    ..
                } => Some(kind),
                _ => None,
            })
            .collect()
    }

    /// The forces limits the staff set on `day`: intent, source, task forces.
    fn limits(&self, day: u32) -> Vec<(IntentId, Option<BaseId>, Vec<TaskForceId>)> {
        self.handed[&day]
            .commands
            .iter()
            .filter_map(|c| match &c.kind {
                CommandKind::SetLimit(Limit::Forces { intent, limit }) => Some((
                    *intent,
                    limit.source,
                    limit.task_forces.iter().copied().collect(),
                )),
                _ => None,
            })
            .collect()
    }

    /// The first day the staff set a limit on the supply (intent 2).
    fn first_limit(&self) -> Option<(u32, Option<BaseId>, Vec<TaskForceId>)> {
        self.handed.keys().find_map(|day| {
            self.limits(*day)
                .into_iter()
                .find(|(i, _, _)| *i == intent(2))
                .map(|(_, s, f)| (*day, s, f))
        })
    }

    /// The runs added on each day: (day, load).
    fn runs(&self) -> Vec<(u32, sim_core::fixed::Fixed)> {
        self.handed
            .iter()
            .flat_map(|(day, d)| {
                d.commands.iter().filter_map(move |c| match &c.kind {
                    CommandKind::AddRun { run, .. } => Some((*day, run.load_t)),
                    _ => None,
                })
            })
            .collect()
    }

    fn reasons(&self, day: u32) -> Vec<String> {
        let d = &self.handed[&day];
        d.commands
            .iter()
            .map(|c| &c.reason)
            .chain(d.orders.iter().map(|o| &o.reason))
            .chain(d.no_orders.iter())
            .map(words)
            .collect()
    }

    fn said(&self, day: u32) -> String {
        self.reasons(day).join("\n")
    }

    fn force_orders(&self, day: u32, f: u32) -> Vec<TaskForceOrder> {
        self.handed[&day]
            .orders
            .iter()
            .filter_map(|o| match &o.kind {
                OrderKind::TaskForce { target, order } if *target == force(f) => {
                    Some(order.clone())
                }
                _ => None,
            })
            .collect()
    }

    /// The hold's plan was never marked failed or ended.
    fn hold_plan_never_ended(&self) -> bool {
        self.states.iter().all(|s| {
            s.plans
                .get(&PlanId::new(1))
                .is_none_or(|p| !p.state.ended())
        })
    }

    fn supplied_from(&self, morning: u32) -> Option<(BaseId, Maker)> {
        self.states[(morning - 1) as usize].supplied_from(intent(1))
    }

    fn supply_of(&self, day: u32, base: u32) -> sim_core::fixed::Fixed {
        self.truth[day as usize].bases[&sim_core::ids::BaseId::new(base)].supply_t
    }
}

fn t(n: i64) -> sim_core::fixed::Fixed {
    sim_core::fixed::Fixed::from_int(n)
}

// ------------------------------------------------------------------- File a

#[test]
fn nothing_is_refused_in_file_a() {
    let played = play(&text(), 10);
    assert_eq!(played.refusals, Vec::<String>::new());
}

/// Statement 1.
#[test]
fn statement_1_the_supply_is_formed_and_given_source_base_3_and_task_force_1() {
    let played = play(&text(), 1);
    assert_eq!(
        played.formed(1),
        vec![IntentKind::KeepSupplied {
            base: base(1),
            floor_days: 5,
            interval_days: None
        }]
    );
    assert_eq!(
        played.limits(1),
        vec![(intent(2), Some(base(3)), vec![force(1)])]
    );
}

/// Statement 2.
#[test]
fn statement_2_the_reason_weighs_each_base_with_its_figures() {
    let played = play(&text(), 1);
    let formed = played.handed[&1]
        .commands
        .iter()
        .find(|c| matches!(c.kind, CommandKind::FormIntent { .. }))
        .expect("the supply is formed");
    let said = words(&formed.reason);
    for part in [
        "base:2 (Placeholder base N): 50 t to spare, less than the place's need of 75 t, task_force:2 stopped there with nothing aboard, 80 nm from the place",
        "base:3 (Placeholder base F): 2000 t to spare, enough for the place's need of 75 t, task_force:1 stopped there with nothing aboard, 320 nm from the place",
        "base:4 (Placeholder base G): 5000 t to spare, enough for the place's need of 75 t, no transport force stopped there with nothing aboard, 320 nm from the place",
        "the source is base:3 (Placeholder base F)",
    ] {
        assert!(said.contains(part), "{part}\n{said}");
    }
    let evidence: Vec<&str> = formed
        .reason
        .evidence
        .iter()
        .map(|e| e.what.as_str())
        .collect();
    assert!(evidence
        .iter()
        .any(|e| e.starts_with("base:2 (Placeholder base N): 50 t to spare")));
    assert!(evidence
        .iter()
        .any(|e| e.starts_with("base:4 (Placeholder base G): 5000 t to spare")));
}

/// Statement 3.
#[test]
fn statement_3_the_supply_serves_the_holds_plan() {
    let played = play(&text(), 10);
    let first = &played.states[1];
    assert_eq!(first.plans[&PlanId::new(1)].state, PlanState::Released);
    assert_eq!(first.plans.len(), 1, "the supply has no plan of its own");
    assert_eq!(first.intents[&intent(2)].plan, PlanId::new(1));
    assert!(played.hold_plan_never_ended());
}

/// Statement 4.
#[test]
fn statement_4_a_run_of_task_force_1_unloads_at_h_and_task_force_2_is_given_no_order() {
    let played = play(&text(), 10);
    let sails = played.handed.keys().any(|d| {
        played.force_orders(*d, 1).iter().any(|o| {
            matches!(o, TaskForceOrder::SetRoute { waypoints, act_at, .. }
                if waypoints.first() == Some(&sim_core::hex::Hex::new(0, 0))
                    && act_at.contains(&sim_core::hex::Hex::new(0, 0)))
        })
    });
    assert!(sails, "a run of task force 1 goes to H");
    let unloads = played
        .handed
        .keys()
        .any(|d| played.force_orders(*d, 1).contains(&TaskForceOrder::Unload));
    assert!(unloads);
    // The supply reaches H: its stock rises on some day.
    assert!((1..=10).any(|d| played.supply_of(d, 1) > played.supply_of(d - 1, 1)));
    assert_eq!(played.supply_of(10, 4), t(5000));
    assert!(played
        .handed
        .keys()
        .all(|d| played.force_orders(*d, 2).is_empty()));
}

/// Statement 5.
#[test]
fn statement_5_every_morning_from_day_2_the_source_is_base_3_chosen_by_the_staff() {
    let played = play(&text(), 10);
    for morning in 2..=10 {
        assert_eq!(
            played.supplied_from(morning),
            Some((base(3), Maker::Staff)),
            "morning {morning}"
        );
    }
}

// ------------------------------------------------------------------- File b

/// Statement 6.
#[test]
fn statement_6_base_4_is_the_source_though_task_force_2_lies_at_base_2() {
    let played = play(&file_b(), 1);
    assert_eq!(played.limits(1), vec![(intent(2), Some(base(4)), vec![])]);
}

/// Statement 7.
#[test]
fn statement_7_the_supply_says_no_ship_of_its_pool_is_afloat_and_sends_nothing() {
    let played = play(&file_b(), 2);
    assert!(played
        .said(2)
        .contains("No ship of the pool of intent:2 is afloat"));
    assert!(played.runs().is_empty());
    assert!(played.hold_plan_never_ended());
}

/// Statement 8.
#[test]
fn statement_8_task_force_3_joins_on_day_3_and_the_first_run_is_ordered_on_day_4() {
    let played = play(&file_b(), 6);
    assert_eq!(
        played.limits(3),
        vec![(intent(2), Some(base(4)), vec![force(3)])]
    );
    assert_eq!(played.runs().first().map(|(d, _)| *d), Some(4));
    assert!(played.hold_plan_never_ended());
    assert_eq!(played.refusals, Vec::<String>::new());
}

// ------------------------------------------------------------------- File c

/// Statement 9.
#[test]
fn statement_9_the_source_is_base_2_and_the_first_load_is_40_t() {
    let played = play(&file_c(), 10);
    assert_eq!(
        played.first_limit(),
        Some((1, Some(base(2)), vec![force(2)]))
    );
    let runs = played.runs();
    assert_eq!(runs.first(), Some(&(2, t(40))));
    let said = played.said(2);
    assert!(
        said.contains("the load is cut to what the source can spare"),
        "{said}"
    );
}

/// Statement 10.
#[test]
fn statement_10_no_second_run_and_no_other_base_named() {
    let played = play(&file_c(), 10);
    assert_eq!(played.runs().len(), 1);
    let mut spoke = 0;
    for day in 3..=10 {
        let said = played.said(day);
        if said.contains("It cannot be met") {
            spoke += 1;
            assert!(
                said.contains("no other base of the side can spare supply"),
                "day {day}: {said}"
            );
            assert!(!said.contains("to spare;"), "{said}");
        }
    }
    assert!(spoke > 0, "the supply could not be met on some day");
}

// ------------------------------------------------------------------- File d

/// Statement 11.
#[test]
fn statement_11_no_supply_is_formed_and_base_1_lives_on_what_it_has() {
    let played = play(&file_d(), 10);
    assert!(played.handed.keys().all(|d| played.formed(*d).is_empty()));
    let said = played.said(1);
    for part in [
        "no other base of the side can spare supply",
        "base:2 (Placeholder base N) holds 150 t and keeps 150 t",
        "base:1 (Placeholder base H) lives on what it has: 4 days of supply today",
    ] {
        assert!(said.contains(part), "{part}\n{said}");
    }
}

// ------------------------------------------------------------------- File e

/// Statement 12.
#[test]
fn statement_12_the_commanders_source_is_used_and_nothing_is_sent() {
    let played = play(&names_source(&text(), 1, 4), 10);
    assert_eq!(played.limits(1), vec![(intent(2), Some(base(4)), vec![])]);
    assert!(played
        .said(1)
        .contains("the commander's forces limit names"));
    for morning in 2..=10 {
        assert_eq!(
            played.supplied_from(morning),
            Some((base(4), Maker::Commander)),
            "morning {morning}"
        );
    }
    assert!(played.runs().is_empty());
    assert!(played.hold_plan_never_ended());
}

// ------------------------------------------------------------------- File f

fn file_f() -> String {
    let t = names_source(
        &common::replace_once(&text(), "days = 10", "days = 20"),
        3,
        2,
    );
    at(
        &t,
        "commands",
        &command(
            16,
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\ncleared = true",
        ),
    )
}

/// Statement 14.
#[test]
fn statement_14_a_source_the_commander_names_is_taken_up_and_the_pool_made_anew() {
    let played = play(&file_f(), 20);
    let change = played
        .limits(3)
        .into_iter()
        .find(|(i, s, _)| *i == intent(2) && *s == Some(base(2)))
        .map(|(_, _, f)| (3, f))
        .expect("the limit is set on the morning of day 3");
    assert_eq!(change.1, vec![force(2)], "task force 1 is no longer in it");
    assert_eq!(
        played.states[change.0 as usize].supplied_from(intent(1)),
        Some((base(2), Maker::Commander))
    );
}

/// Statement 15.
#[test]
fn statement_15_from_day_17_the_source_is_still_base_2_chosen_by_the_staff() {
    let played = play(&file_f(), 20);
    for morning in 17..=20 {
        assert_eq!(
            played.supplied_from(morning),
            Some((base(2), Maker::Staff)),
            "morning {morning}"
        );
    }
}

// ------------------------------------------------------------------- File g

/// Statement 16.
#[test]
fn statement_16_the_source_chosen_on_an_earlier_morning_is_kept() {
    let played = play(&with_force_3(&text(), 3), 10);
    for morning in 2..=10 {
        assert_eq!(
            played.supplied_from(morning),
            Some((base(3), Maker::Staff)),
            "morning {morning}"
        );
    }
    for day in 3..=10 {
        assert!(
            played
                .said(day)
                .contains("chosen on an earlier morning and kept"),
            "day {day}"
        );
    }
}

// ------------------------------------------------------------------- File h

/// Statement 17.
#[test]
fn statement_17_the_islands_hold_is_given_source_base_3_and_task_force_1() {
    let played = play(&file_h(), 1);
    assert!(
        played
            .limits(1)
            .contains(&(intent(2), Some(base(3)), vec![force(1)])),
        "{:?}",
        played.limits(1)
    );
}

// ------------------------------------------------------------------- File i

/// Statement 18.
#[test]
fn statement_18_a_base_in_the_places_own_hex_is_not_weighed() {
    let played = play(&base_5(&file_d(), 0, 0, 900), 10);
    assert!(played.handed.keys().all(|d| played.formed(*d).is_empty()));
    let said = played.said(1);
    assert!(
        said.contains("base:2 (Placeholder base N) holds 150 t and keeps 150 t"),
        "{said}"
    );
    assert!(!said.contains("base:5"), "{said}");
    assert!(said.contains("184 days of supply today"), "{said}");
}

// ------------------------------------------------------------- Files j and k

/// Statement 19.
#[test]
fn statement_19_without_the_key_no_supply_is_formed_and_the_reason_is_supply_58s() {
    let played = play(&no_key(&text()), 10);
    assert!(played.handed.keys().all(|d| played.formed(*d).is_empty()));
    let said = played.said(1);
    for part in [
        "base:2 (Placeholder base N), 200 t of supply above its reserve",
        "base:3 (Placeholder base F), 2000 t of supply above its reserve",
        "base:4 (Placeholder base G), 5000 t of supply above its reserve",
    ] {
        assert!(said.contains(part), "{part}\n{said}");
    }
}

/// Statement 20.
#[test]
fn statement_20_without_the_key_the_island_gives_source_base_4_and_no_force() {
    let played = play(&no_key(&file_h()), 1);
    assert!(
        played
            .limits(1)
            .contains(&(intent(2), Some(base(4)), vec![])),
        "{:?}",
        played.limits(1)
    );
}

// ------------------------------------------------------- What must always hold

/// The source the staff gives the supply on day 1 of `text`.
fn source_of(text: &str) -> Option<BaseId> {
    play(text, 1).first_limit().and_then(|(_, s, _)| s)
}

fn base_4_with(text: &str, supply: u32) -> String {
    common::replace_once(text, "supply_t = 5000", &format!("supply_t = {supply}"))
}

#[test]
fn more_supply_or_a_force_at_a_base_never_puts_it_later() {
    // File b's bases with task force 3 usable from day 1 and base 4's stock
    // varied: once base 4 is chosen, more supply keeps it chosen.
    let mut chosen = Vec::new();
    for supply in [100, 1000, 3000, 5000, 20000] {
        let t = base_4_with(
            &with_force_3(&without(&without(&text(), "base 3"), "force 1"), 1),
            supply,
        );
        chosen.push(source_of(&t).map(BaseId::raw));
    }
    let first_4 = chosen.iter().position(|c| *c == Some(4));
    if let Some(at) = first_4 {
        assert!(chosen[at..].iter().all(|c| *c == Some(4)), "{chosen:?}");
    }
    // A force added at base 4: where it was chosen without, it is with.
    for supply in [100, 1000, 5000] {
        let bare = base_4_with(&without(&without(&text(), "base 3"), "force 1"), supply);
        let with = with_force_3(&bare, 1);
        if source_of(&bare) == Some(base(4)) {
            assert_eq!(source_of(&with), Some(base(4)), "supply {supply}");
        }
    }
}

#[test]
fn a_base_in_the_places_hex_is_never_chosen() {
    for supply in [10, 900, 100_000] {
        let t = base_5(&text(), 0, 0, supply);
        assert_ne!(source_of(&t), Some(base(5)), "supply {supply}");
    }
}

#[test]
fn a_source_the_commander_names_is_always_the_source() {
    for named in [2, 3, 4] {
        let played = play(&names_source(&text(), 1, named), 3);
        assert_eq!(
            played.supplied_from(3).map(|(b, m)| (b.raw(), m)),
            Some((named, Maker::Commander))
        );
    }
}

#[test]
fn a_run_from_a_staff_source_never_takes_it_below_what_it_keeps() {
    for stock in [200, 260, 400, 1000] {
        let t = common::replace_once(&file_c(), "supply_t = 200", &format!("supply_t = {stock}"));
        let played = play(&t, 10);
        for (day, load) in played.runs() {
            let morning = played.supply_of(day - 1, 2);
            let spare = (morning - t_(150)).max(sim_core::fixed::Fixed::ZERO);
            assert!(load <= spare, "stock {stock}, day {day}: {load} > {spare}");
        }
    }
}

fn t_(n: i64) -> sim_core::fixed::Fixed {
    t(n)
}

#[test]
fn two_bases_in_one_hex_never_spare_more_than_their_hex() {
    // Base 5 at N beside base 2: the hex holds 300 t and keeps 150.
    let t = base_5(&file_c(), -2, 0, 100);
    let played = play(&t, 10);
    for (day, load) in played.runs() {
        let hex_stock = played.supply_of(day - 1, 2) + played.supply_of(day - 1, 5);
        let spare = (hex_stock - t_(150)).max(sim_core::fixed::Fixed::ZERO);
        assert!(load <= spare, "day {day}: {load} > {spare}");
    }
}

#[test]
fn supply_is_never_made() {
    let played = play(&text(), 10);
    let total = |d: usize| {
        let truth = &played.truth[d];
        let bases = truth
            .bases
            .values()
            .fold(sim_core::fixed::Fixed::ZERO, |s, b| s + b.supply_t);
        truth
            .ships
            .values()
            .fold(bases, |s, ship| s + ship.supply_t)
    };
    for d in 1..=10 {
        assert!(total(d) <= total(d - 1), "day {d}");
        assert!(total(d - 1) - total(d) <= t(15), "day {d}");
    }
}

#[test]
fn asked_twice_the_staff_chooses_the_same_base() {
    let a = play(&text(), 2);
    let b = play(&text(), 2);
    assert_eq!(a.limits(1), b.limits(1));
    assert_eq!(a.said(1), b.said(1));
}

#[test]
fn a_side_without_the_key_gets_the_orders_it_got_before_whatever_the_setup() {
    // The key stated false is the key left out.
    for variant in [text(), file_b(), file_c(), file_h()] {
        let left_out = no_key(&variant);
        let stated_false = common::replace_once(
            &variant,
            "staff_chooses_supply_sources = true",
            "staff_chooses_supply_sources = false",
        );
        let (a, b) = (play(&left_out, 6), play(&stated_false, 6));
        for day in 1..=6 {
            assert_eq!(a.said(day), b.said(day), "day {day}");
            assert_eq!(a.limits(day), b.limits(day), "day {day}");
        }
    }
}
