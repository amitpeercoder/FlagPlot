//! What the staff does with holds, runs set aside, supply aboard a free
//! ship, a cancelled plan and a changed base (decision 0107, step 3,
//! rulings 17 to 29). The games are scenario 36 with other commands, played
//! on the frozen step 1 figures with the doctrine's staffs passed in; the
//! architect's hand calculation of the whole is
//! `tests/scenario_41_hold_change_release.rs`, which these do not repeat.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use sim_core::command::{CommandKind, Reason, StaffDecisions};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{ShipId, TaskForceId};
use sim_core::orders::{BaseOrder, OrderKind, TaskForceOrder};
use sim_core::state::Side;
use sim_core::world::World;
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, project, staffs};

const COMMANDS_MARKER: &str = "# The Allied commander's commands";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn scenario_text() -> String {
    std::fs::read_to_string(root().join("data/scenarios/36_one_convoy.toml"))
        .expect("scenario 36 reads")
}

/// Scenario 36 as far as its commands, with `extra` before them (a base, a
/// ship's cargo) and the commands as given.
fn text(days: u32, extra: &str, commands: &str) -> String {
    let text = scenario_text();
    let head = text.split(COMMANDS_MARKER).next().expect("a head");
    let head = head.replace("days = 18", &format!("days = {days}"));
    format!("{head}\n{extra}\n{COMMANDS_MARKER}\n{commands}")
}

const INTENT: &str = "
[[command]]
day = 1
side = \"allied\"
kind = \"set_intent\"
intent = 1
base = 2
floor_days = 30
interval_days = 15

[[command]]
day = 1
side = \"allied\"
kind = \"set_limit\"
limit = \"forces\"
intent = 1
ships = [1, 2]
task_forces = []
source = 1

[[command]]
day = 1
side = \"allied\"
kind = \"set_limit\"
limit = \"reserve\"
base = 1
supply_t = 1000
fuel_t = 0

[[command]]
day = 1
side = \"allied\"
kind = \"set_leave\"
leave = \"prepare\"
given = true
";

const LAUNCH: &str = "
[[command]]
day = 1
side = \"allied\"
kind = \"set_leave\"
leave = \"launch\"
given = true
";

fn command(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
}

fn load(text: &str) -> Scenario {
    let path = root().join("data/scenarios/test_holds.toml");
    let options = LoadOptions::frozen_step1(&root());
    Scenario::from_text_with(text, &path, &options).expect("the scenario loads")
}

/// A game played to its end with the doctrine's staffs: what the staff
/// handed in each day (day 1 first), and a hook run at the staff's moment of
/// each day on the world.
struct Played {
    scenario: Scenario,
    handed: Vec<StaffDecisions>,
}

fn play(text: &str, hook: &mut dyn FnMut(u32, &World)) -> Played {
    let mut scenario = load(text);
    let held = staffs();
    let mut handed = Vec::new();
    loop {
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                if let sim_core::choices::Moment::Orders { day } = moment {
                    hook(day, world);
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        handed.push(
            scenario
                .world
                .staff_handed_in()
                .get(&Side::Allied)
                .cloned()
                .unwrap_or_default(),
        );
    }
    Played { scenario, handed }
}

impl Played {
    fn day(&self, day: u32) -> &StaffDecisions {
        &self.handed[day as usize - 1]
    }

    /// Nothing the staff handed in was refused.
    fn nothing_refused(&self) {
        let refused: Vec<_> = self
            .scenario
            .world
            .events()
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
                )
            })
            .collect();
        assert!(refused.is_empty(), "something was refused: {refused:?}");
    }
}

/// Every piece of a reason, as one text.
fn whole(reason: &Reason) -> String {
    let mut all = reason.worked.clone();
    for e in &reason.evidence {
        all.push('\n');
        all.push_str(&e.what);
    }
    for b in &reason.binding {
        all.push('\n');
        all.push_str(b);
    }
    for w in &reason.would_change {
        all.push('\n');
        all.push_str(w);
    }
    all
}

fn add_runs(d: &StaffDecisions) -> Vec<(&BTreeSet<ShipId>, Fixed, &Reason)> {
    d.commands
        .iter()
        .filter_map(|c| match &c.kind {
            CommandKind::AddRun { run, .. } => Some((&run.ships, run.load_t, &c.reason)),
            _ => None,
        })
        .collect()
}

fn load_orders(d: &StaffDecisions) -> Vec<Fixed> {
    d.orders
        .iter()
        .filter_map(|o| match &o.kind {
            OrderKind::TaskForce {
                order: TaskForceOrder::LoadTo { weight_t, .. },
                ..
            } => Some(*weight_t),
            _ => None,
        })
        .collect()
}

fn formed(d: &StaffDecisions) -> Vec<Vec<ShipId>> {
    d.orders
        .iter()
        .filter_map(|o| match &o.kind {
            OrderKind::Base {
                order: BaseOrder::FormTaskForce { ships },
                ..
            } => Some(ships.clone()),
            _ => None,
        })
        .collect()
}

#[test]
fn a_run_not_sailed_whose_ship_is_held_is_set_aside_and_the_staff_names_the_hold() {
    // No leave to launch: run 1 is loaded and waits. Ship 1 is held on day 2.
    let commands = format!("{INTENT}{}", command(2, "kind = \"hold\"\nship = 1"));
    let played = play(&text(3, "", &commands), &mut |_, _| {});
    played.nothing_refused();

    // Day 1: ship 1 is sent, 360 t.
    let day1 = played.day(1);
    assert_eq!(formed(day1), vec![vec![ship(1)]]);
    assert_eq!(load_orders(day1), vec![n(360)]);

    // Day 2: the run is ended, its commitment freed, and ship 2 loads in its
    // place: 300 - 80 + 150 = 370 t.
    let day2 = played.day(2);
    let ended: Vec<&sim_core::command::StaffCommand> = day2
        .commands
        .iter()
        .filter(|c| matches!(c.kind, CommandKind::EndRun { .. }))
        .collect();
    assert_eq!(ended.len(), 1);
    assert_eq!(
        ended[0].reason.worked,
        "Run 1 of plan:1, the plan to keep base:2 (Placeholder base T, Allied) at 30 days of \
         supply, is ended: ship:1 is held by the commander's hold of day 2; \
         the 360 t aboard stay aboard."
    );
    assert!(day2.commands.iter().any(
        |c| matches!(&c.kind, CommandKind::FreeShips { ships, .. } if ships.contains(&ship(1)))
    ));
    let runs = add_runs(day2);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].0, &BTreeSet::from([ship(2)]));
    assert_eq!(runs[0].1, n(370));
    assert_eq!(load_orders(day2), vec![n(370)]);
    let words = whole(runs[0].2);
    assert!(
        words.contains("Without the hold, ship:1 would have been sent."),
        "{words}"
    );
    assert!(
        words.contains("ship:1 is held by the commander's hold of day 2"),
        "{words}"
    );
    assert!(words.contains("the hold on ship:1"), "{words}");
    // The staff gives no order to the held ship's force.
    let held_force = TaskForceId::new(1_000_001);
    for order in &day2.orders {
        if let OrderKind::TaskForce { target, .. } = &order.kind {
            assert_ne!(*target, held_force);
        }
    }
    // The command state shows run 1 ended on day 2 and the hold.
    let state = played.scenario.world.side_view(Side::Allied);
    let plan = &state.command_state().plans[&sim_core::command::PlanId::new(1)];
    assert_eq!(plan.runs[&1].ended, Some(2));
    assert_eq!(plan.runs.len(), 2);
}

#[test]
fn a_held_ship_is_not_free_and_the_other_ship_is_sent() {
    // Ship 1 is held on day 1, before the staff is asked.
    let commands = format!(
        "{INTENT}{LAUNCH}{}",
        command(1, "kind = \"hold\"\nship = 1")
    );
    let played = play(&text(2, "", &commands), &mut |_, _| {});
    played.nothing_refused();
    let day1 = played.day(1);
    assert_eq!(formed(day1), vec![vec![ship(2)]]);
    let runs = add_runs(day1);
    assert_eq!(runs[0].0, &BTreeSet::from([ship(2)]));
    assert!(whole(runs[0].2).contains("Without the hold, ship:1 would have been sent."));
}

#[test]
fn a_hold_that_leaves_no_ship_free_is_what_binds() {
    let commands = format!(
        "{INTENT}{LAUNCH}{}{}",
        command(1, "kind = \"hold\"\nship = 1"),
        command(1, "kind = \"hold\"\nship = 2")
    );
    let played = play(&text(1, "", &commands), &mut |_, _| {});
    let day1 = played.day(1);
    assert!(day1.orders.is_empty());
    let [reason] = &day1.no_orders[..] else {
        panic!("one reason")
    };
    let words = whole(reason);
    assert!(words.contains("no ship of the pool is free"), "{words}");
    assert!(words.contains("the hold on ship:1"), "{words}");
    assert!(
        words.contains("Without the hold, ship:1 would have been sent."),
        "{words}"
    );
}

#[test]
fn a_free_ship_with_supply_aboard_carries_it_and_is_given_no_load_order() {
    // Ship 1 lies in port with 360 t aboard: the sum gives 360 t, so nothing
    // is to be taken.
    let extra = "";
    let text = text(1, extra, &format!("{INTENT}{LAUNCH}"));
    let text = text.replace(
        "fuel_t = 1000\nport = 1\n\n[[ship]]\nid = 2",
        "fuel_t = 1000\nsupply_t = 360\nport = 1\n\n[[ship]]\nid = 2",
    );
    let played = play(&text, &mut |_, _| {});
    played.nothing_refused();
    let day1 = played.day(1);
    let runs = add_runs(day1);
    assert_eq!(runs[0].0, &BTreeSet::from([ship(1)]));
    assert_eq!(runs[0].1, n(360));
    assert!(load_orders(day1).is_empty(), "no load order is given");
    let words = whole(runs[0].2);
    assert!(words.contains("carries the 360 t it has aboard"), "{words}");
    assert!(words.contains("no load order is needed"), "{words}");
    assert!(!words.contains("by the sum"), "{words}");
}

#[test]
fn a_sum_below_what_is_aboard_carries_what_is_aboard() {
    // 150 t is the sum's load after the first run; with 500 t aboard on day
    // 1 the sum is 360 t, below what is aboard.
    let text = text(1, "", &format!("{INTENT}{LAUNCH}")).replace(
        "fuel_t = 1000\nport = 1\n\n[[ship]]\nid = 2",
        "fuel_t = 1000\nsupply_t = 500\nport = 1\n\n[[ship]]\nid = 2",
    );
    let played = play(&text, &mut |_, _| {});
    let day1 = played.day(1);
    let runs = add_runs(day1);
    assert_eq!(runs[0].1, n(500));
    assert!(load_orders(day1).is_empty());
    let words = whole(runs[0].2);
    assert!(words.contains("carries the 500 t it has aboard"), "{words}");
    assert!(words.contains("its load is 360 t by the sum"), "{words}");
    assert!(words.contains("no load order is needed"), "{words}");
}

#[test]
fn some_supply_aboard_is_topped_up_by_a_load_order_for_the_difference() {
    // 100 t aboard, 360 t by the sum: 260 t are taken and the order names
    // the weight aboard in all.
    let text = text(1, "", &format!("{INTENT}{LAUNCH}")).replace(
        "fuel_t = 1000\nport = 1\n\n[[ship]]\nid = 2",
        "fuel_t = 1000\nsupply_t = 100\nport = 1\n\n[[ship]]\nid = 2",
    );
    let played = play(&text, &mut |_, _| {});
    let day1 = played.day(1);
    let runs = add_runs(day1);
    assert_eq!(runs[0].1, n(360));
    assert_eq!(load_orders(day1), vec![n(360)]);
}

/// A map like scenario 36's with T at (9,0): 360 nm from S.
fn far_map_text() -> String {
    let map = std::fs::read_to_string(root().join("data/map/placeholder_one_convoy.toml"))
        .expect("the map reads");
    let map = map.replace("q = [-2, 7]", "q = [-2, 11]").replace(
        "q = 5\nr = 0\nkind = \"coast\"",
        "q = 9\nr = 0\nkind = \"coast\"",
    );
    assert!(map.contains("q = 9"));
    map
}

#[test]
fn a_cancelled_plan_sends_a_force_on_passage_out_home_once() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("sim_staff_holds");
    std::fs::create_dir_all(&dir).expect("a directory");
    let map = dir.join("far_map.toml");
    std::fs::write(&map, far_map_text()).expect("the map is written");
    let commands = format!(
        "{INTENT}{LAUNCH}{}",
        command(3, "kind = \"cancel_plan\"\nplan = 1")
    );
    let text = text(5, "", &commands)
        .replace(
            "map = \"../map/placeholder_one_convoy.toml\"",
            &format!("map = {:?}", map.display().to_string()),
        )
        .replace("q = 5\nr = 0", "q = 9\nr = 0");
    let played = play(&text, &mut |_, _| {});
    played.nothing_refused();

    // The force sails on day 2 and is out at sea on day 3.
    let home = Hex::new(0, 0);
    let routes = |d: &StaffDecisions| -> Vec<(TaskForceId, Vec<Hex>, usize)> {
        d.orders
            .iter()
            .filter_map(|o| match &o.kind {
                OrderKind::TaskForce {
                    target,
                    order: TaskForceOrder::SetRoute { waypoints, act_at },
                } => Some((*target, waypoints.clone(), act_at.len())),
                _ => None,
            })
            .collect()
    };
    let day3 = played.day(3);
    assert_eq!(
        routes(day3),
        vec![(TaskForceId::new(1_000_001), vec![home], 0)]
    );
    // No new unload order: its own stands.
    assert!(!day3.orders.iter().any(|o| matches!(
        o.kind,
        OrderKind::TaskForce {
            order: TaskForceOrder::Unload,
            ..
        }
    )));
    let reason = &day3
        .orders
        .iter()
        .find(|o| {
            matches!(
                o.kind,
                OrderKind::TaskForce {
                    order: TaskForceOrder::SetRoute { .. },
                    ..
                }
            )
        })
        .expect("the route order")
        .reason;
    assert_eq!(
        reason.worked,
        "plan:1 is cancelled: task_force:1000001 on passage out to base:2 is sent home to \
         base:1; its cargo stays aboard and its unload order stands, so it is put back \
         ashore at base:1."
    );
    // The next morning the force is on passage home: nothing is given again.
    for day in 4..=5 {
        let d = played.day(day);
        assert!(routes(d).is_empty(), "day {day}");
        assert!(d.orders.is_empty(), "day {day}");
        let [no_order] = &d.no_orders[..] else {
            panic!("one reason on day {day}")
        };
        assert!(
            no_order.worked.contains(
                "plan:1, the plan to keep base:2 (Placeholder base T, Allied) at 30 days \
                     of supply, is cancelled, so the staff gives no order for intent:1"
            ),
            "{}",
            no_order.worked
        );
    }
}

#[test]
fn a_run_sailed_lands_at_the_old_base_and_a_loading_run_at_the_new_one() {
    // Base 3 at (3,0), the side's, with no use. On day 2 the intent's base
    // is changed to base 3: run 1 (loading on day 1, sailing on day 2 with
    // its route marked to act at T) is still loading when the day begins.
    let extra = "
[[base]]
id = 3
name = \"Placeholder base V, Allied\"
side = \"allied\"
q = 3
r = 0
supply_t = 0
";
    let change = command(
        2,
        "kind = \"set_intent\"\nintent = 1\nbase = 3\nfloor_days = 30\ninterval_days = 15",
    );
    let change3 = command(
        3,
        "kind = \"set_intent\"\nintent = 1\nbase = 3\nfloor_days = 30\ninterval_days = 15",
    );
    let t_hex = Hex::new(5, 0);
    let v_hex = Hex::new(3, 0);
    let stock_on = |p: &sim_core::command::SupplyProjection, day: u32| {
        p.readings
            .iter()
            .find(|r| r.day == day)
            .map(|r| r.stock_t)
            .expect("a reading")
    };

    // Changed on day 2: the loading run is counted at the new base.
    let mut seen2 = None;
    let _ = play(
        &text(2, extra, &format!("{INTENT}{LAUNCH}{change}")),
        &mut |day, world| {
            if day == 2 {
                let view = world.side_view(Side::Allied);
                let at_v = project(&view, v_hex, 30, 5).expect("base 3 is the side's");
                let at_t = project(&view, t_hex, 30, 5).expect("base 2 is the side's");
                seen2 = Some((stock_on(&at_v, 4), stock_on(&at_t, 4)));
            }
        },
    );
    let (v4, t4) = seen2.expect("day 2 was reached");
    assert!(v4 >= n(300), "the loaded run lands at base 3: {v4}");
    // At T nothing lands: only the planned use of the men.
    assert!(t4 < n(100), "nothing lands at base 2: {t4}");

    // Changed on day 3, after the run has sailed (day 2) toward T with its
    // route marked to act at T: it lands at the old base.
    let mut seen3 = None;
    let _ = play(
        &text(3, extra, &format!("{INTENT}{LAUNCH}{change3}")),
        &mut |day, world| {
            if day == 3 {
                let view = world.side_view(Side::Allied);
                let at_v = project(&view, v_hex, 30, 5).expect("base 3 is the side's");
                seen3 = Some(stock_on(&at_v, 5));
            }
        },
    );
    assert_eq!(seen3, Some(n(0)), "a sailed run does not land at base 3");
}

#[test]
fn a_hold_a_changed_floor_and_a_release_follow_the_hand_calculation() {
    let commands = format!(
        "{INTENT}{}{}{}{}",
        command(3, "kind = \"approve\"\nplan = 1"),
        command(8, "kind = \"hold\"\nship = 1"),
        command(
            10,
            "kind = \"set_intent\"\nintent = 1\nbase = 2\nfloor_days = 40\ninterval_days = 15"
        ),
        command(17, "kind = \"release\"\nship = 1"),
    );
    let played = play(&text(27, "", &commands), &mut |_, _| {});
    played.nothing_refused();

    // Day 1: made ready, ship 1 loads 360 t. Day 2: it waits for leave.
    assert_eq!(add_runs(played.day(1))[0].1, n(360));
    assert!(played.day(2).orders.is_empty());
    // Day 3: released, run 1 is timed again to sail the same day.
    assert!(played.day(3).commands.iter().any(|c| matches!(
        &c.kind,
        CommandKind::TimeRun { days, .. } if days.sail == 3 && days.free == 6
    )));
    // Day 8: ship 1 is held and no run is needed.
    assert!(played.day(8).orders.is_empty());
    // Day 10: the floor is 40 days; ship 1 is held, so ship 2 carries 190 t,
    // and the reason serves amendment 1 and says what ship 1 would have done.
    let d10 = played.day(10);
    let runs = add_runs(d10);
    assert_eq!(runs[0].0, &BTreeSet::from([ship(2)]));
    assert_eq!(runs[0].1, n(190));
    assert_eq!(runs[0].2.serves.amendment, Some(1));
    let words = whole(runs[0].2);
    assert!(
        words.contains("ship:1 is held by the commander's hold of day 8"),
        "{words}"
    );
    assert!(
        words.contains("Without the hold, ship:1 would have been sent."),
        "{words}"
    );
    // Days 11 to 24: nothing but ship 2's sailing; day 17, ship 1 released.
    assert!(played.day(17).orders.is_empty());
    // Day 25: ship 1, in the force formed on day 1, loads 150 t.
    let d25 = played.day(25);
    let runs = add_runs(d25);
    assert_eq!(runs[0].0, &BTreeSet::from([ship(1)]));
    assert_eq!(runs[0].1, n(150));
    assert_eq!(load_orders(d25), vec![n(150)]);
    assert!(formed(d25).is_empty());
    assert!(!whole(runs[0].2).contains("held"));
}

#[test]
fn a_released_ship_that_was_set_aside_carries_what_it_has_aboard() {
    // Ship 1 held on day 2 and released on day 8: run 1 is set aside with
    // 360 t aboard. The next run (needed on day 17) is ship 1's: its sum is
    // 150 t, it carries the 360 t aboard and is given no load order.
    let commands = format!(
        "{INTENT}{}{}{}",
        command(3, "kind = \"approve\"\nplan = 1"),
        command(2, "kind = \"hold\"\nship = 1"),
        command(8, "kind = \"release\"\nship = 1"),
    );
    let played = play(&text(18, "", &commands), &mut |_, _| {});
    played.nothing_refused();
    // Day 2: run 1 ended, ship 2 loads 370 t in its place.
    let d2 = played.day(2);
    assert!(d2
        .commands
        .iter()
        .any(|c| matches!(c.kind, CommandKind::EndRun { run: 1, .. })));
    assert_eq!(add_runs(d2)[0].1, n(370));
    // Day 17: ship 1 again, carrying its 360 t, with no load order.
    let d17 = played.day(17);
    let runs = add_runs(d17);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].0, &BTreeSet::from([ship(1)]));
    assert_eq!(runs[0].1, n(360));
    assert!(load_orders(d17).is_empty());
    let words = whole(runs[0].2);
    assert!(words.contains("carries the 360 t it has aboard"), "{words}");
    assert!(words.contains("its load is 150 t by the sum"), "{words}");
    assert!(words.contains("no load order is needed"), "{words}");
    // It sails on day 18: the route order is given then.
    assert!(played.day(18).orders.iter().any(|o| matches!(
        o.kind,
        OrderKind::TaskForce {
            order: TaskForceOrder::SetRoute { .. },
            ..
        }
    )));
}

#[test]
fn a_held_ship_in_a_force_of_two_keeps_both_and_a_third_ship_is_sent() {
    // Ships 1 and 2 lie in force 1 from the start; ship 3 lies loose in
    // port. Ship 1 is held, so the force is not free: ship 3 carries the
    // run, and the reason says the force would have been sent.
    let base = scenario_text();
    let head = base.split(COMMANDS_MARKER).next().expect("a head");
    let head = head
        .replace("days = 18", "days = 1")
        .replace(
            "[[ship]]\nid = 1\nname = \"Placeholder cargo ship 1\"\nclass = 3\nside = \"allied\"\nfuel_t = 1000\nport = 1",
            "[[task_force]]\nid = 1\nname = \"Placeholder convoy\"\nside = \"allied\"\nat_base = 1\nmission = \"transport\"\n\n[[ship]]\nid = 1\nname = \"Placeholder cargo ship 1\"\nclass = 3\nside = \"allied\"\nfuel_t = 1000\ntask_force = 1",
        )
        .replace(
            "[[ship]]\nid = 2\nname = \"Placeholder cargo ship 2\"\nclass = 3\nside = \"allied\"\nfuel_t = 1000\nport = 1",
            "[[ship]]\nid = 2\nname = \"Placeholder cargo ship 2\"\nclass = 3\nside = \"allied\"\nfuel_t = 1000\ntask_force = 1\n\n[[ship]]\nid = 3\nname = \"Placeholder cargo ship 3\"\nclass = 3\nside = \"allied\"\nfuel_t = 1000\nport = 1",
        );
    assert!(head.contains("id = 3"));
    let commands = format!(
        "{INTENT}{LAUNCH}{}",
        command(1, "kind = \"hold\"\nship = 1")
    )
    .replace("ships = [1, 2]", "ships = [1, 2, 3]");
    let text = format!("{head}\n{COMMANDS_MARKER}\n{commands}");
    let played = play(&text, &mut |_, _| {});
    played.nothing_refused();
    let day1 = played.day(1);
    let runs = add_runs(day1);
    assert_eq!(runs[0].0, &BTreeSet::from([ship(3)]));
    let words = whole(runs[0].2);
    assert!(
        words.contains("Without the hold, ship:1, ship:2 would have been sent."),
        "{words}"
    );
    assert!(
        words.contains("ship:1 is held by the commander's hold of day 1"),
        "{words}"
    );
}
