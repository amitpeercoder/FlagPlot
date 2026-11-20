//! The staff counts its side's supply lines (decision 0196, rule 10) where it
//! forecasts a base's stock. The games are scenario 36 with a `[[supply_line]]`
//! added; base 2 (the destination, at hex 5,0) holds 100 t and its men use
//! 10 t a day, with no run in the first test.
//!
//! Hand calculation, projection: the hook runs on day 1's orders, so the view's
//! day is 1 and today is day 2 in the staff's reckoning of "view day plus one"
//! only once the day has been played; at the orders moment the view's day is
//! 0 and today is day 1. Readings of days 1 to 5 without a line: 100, 90, 80,
//! 70, 60. A line of 7 t a day on days 3 and 4 lands on day 3 and day 4,
//! before each day's use: day 4 is 80 - 10 + 7 = 77 and day 5 is 77 - 10 + 7
//! = 74. So the differences are 0, 0, 0, 7, 14.
//! A line to base 1 (hex 0,0) changes nothing at hex 5,0, and a line of the
//! other side's changes nothing.
//!
//! Reason: with a line to base 2 the evidence says "holds ... and uses ... a
//! day and is given 30 t of supply a day by a supply line from day 3 to day
//! 6"; with none the words are those `supply-48` gave.

use std::path::{Path, PathBuf};

use sim_core::command::{CommandKind, StaffDecisions};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::state::Side;
use sim_core::world::World;
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, project, staffs};

const COMMANDS_MARKER: &str = "# The Allied commander's commands";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn text(days: u32, extra: &str, commands: &str) -> String {
    let text = std::fs::read_to_string(root().join("data/scenarios/36_one_convoy.toml"))
        .expect("scenario 36 reads");
    let head = text.split(COMMANDS_MARKER).next().expect("a head");
    let head = head.replace("days = 18", &format!("days = {days}"));
    format!("{head}\n{extra}\n{COMMANDS_MARKER}\n{commands}")
}

fn load(text: &str) -> Scenario {
    let path = root().join("data/scenarios/test_lines.toml");
    let options = LoadOptions::frozen_step1(&root());
    Scenario::from_text_with(text, &path, &options).expect("the scenario loads")
}

fn play(text: &str, hook: &mut dyn FnMut(u32, &World)) -> Vec<StaffDecisions> {
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
    handed
}

fn stocks(extra: &str, hex: Hex) -> Vec<(u32, Fixed)> {
    let mut seen = Vec::new();
    play(&text(6, extra, ""), &mut |day, world| {
        if day == 1 {
            let view = world.side_view(Side::Allied);
            let p = project(&view, hex, 30, 4).expect("the side's base");
            seen = p.readings.iter().map(|r| (r.day, r.stock_t)).collect();
        }
    });
    seen
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

const LINE_TO_2: &str =
    "[[supply_line]]\nbase = 2\nside = \"allied\"\nsupply_t = 7\nfirst_day = 3\nlast_day = 4\n";

#[test]
fn a_line_adds_its_tons_on_its_days_to_the_projection_of_its_base_only() {
    let hex = Hex::new(5, 0);
    let without = stocks("", hex);
    let with = stocks(LINE_TO_2, hex);
    assert_eq!(without.len(), 5);
    assert_eq!(without[0], (1, n(100)));
    assert_eq!(with[0], (1, n(100)));
    assert_eq!(with[1], without[1]);
    let diffs: Vec<Fixed> = with.iter().zip(&without).map(|(a, b)| a.1 - b.1).collect();
    assert_eq!(diffs, vec![n(0), n(0), n(0), n(7), n(14)]);

    // A line to base 1 leaves the hex of base 2 alone.
    let elsewhere =
        "[[supply_line]]\nbase = 1\nside = \"allied\"\nsupply_t = 7\nfirst_day = 3\nlast_day = 4\n";
    assert_eq!(stocks(elsewhere, hex), without);
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

fn reasons(extra: &str) -> String {
    let handed = play(&text(6, extra, INTENT), &mut |_, _| {});
    let mut all = String::new();
    for c in &handed[0].commands {
        if matches!(c.kind, CommandKind::AddRun { .. }) {
            for e in &c.reason.evidence {
                all.push_str(&e.what);
                all.push('\n');
            }
        }
    }
    all
}

#[test]
fn the_reason_names_the_line_and_is_unchanged_without_one() {
    let without = reasons("");
    assert!(without.contains(" a day\n"), "{without}");
    assert!(!without.contains("supply line"), "{without}");
    let line = "[[supply_line]]\nbase = 2\nside = \"allied\"\nsupply_t = 30\nfirst_day = 3\nlast_day = 6\n";
    let with = reasons(line);
    assert!(
        with.contains(
            " a day and is given 30 t of supply a day by a supply line from day 3 to day 6\n"
        ),
        "{with}"
    );
}
