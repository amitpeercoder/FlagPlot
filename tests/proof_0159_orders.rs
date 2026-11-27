//! Not a test: a helper for decision 0159's proof, condition 6 ("Legible").
//! It writes the six plan orders the decision names, each exactly as a
//! seat's `order` tool answers it, into the folder named by the environment
//! variable `FLAGPILOT_0159_ORDERS_OUT`, one JSON file an order. It is
//! ignored, so `cargo test` never runs it; it is run by hand, as
//! `scripts/0159-proof/run.sh orders` runs it:
//!
//! ```text
//! FLAGPILOT_0159_ORDERS_OUT=DIR cargo test --release -p scenario-tests \
//!     --test proof_0159_orders -- --ignored --nocapture
//! ```
//!
//! The six (decision 0159, "The proof: how each condition is read", 6):
//! scenario 80 on the morning of day 2; scenario 81's main game on day 3;
//! its wait for a force on day 3; its after another place on day 3;
//! scenario 82 on day 3; scenario 79 on day 3.
//!
//! Each game is played as scenario 84's test plays it: the scenario file on
//! the frozen figures, the variants of scenario 81 made by the same text
//! changes (the constants below are checked to be word for word those of
//! `scenario_84_the_plan_as_an_order.rs`). It is played through
//! `sim_agent::Session`, the Allied seat a player's and the Japanese the
//! script's, with the doctrine's staffs and the scenario's commands, as
//! `sim-cli serve` opens a game (`serve::open`), but on the frozen figures,
//! which `serve` cannot load. The days before the morning are ended with the
//! seat's own `end_day`; then the seat calls `order` with `{"intent": N}`
//! for each N from 1 to 9, and each answer that is not a refusal is written,
//! untouched, to `<case>.intent<N>.json`. A case with more than one of the
//! commander's intents writes one file for each.
//!
//! As a check, each answer is compared with the order scenario 84's test
//! reads for the same morning: `sim_agent::plan_order` on the Allied view at
//! the staff's moment (`Moment::Orders`), in a second play of the same game.
//! Any difference (but the `inside_day` note a session adds inside a day)
//! is written to `check.txt` and fails the helper.
//!
//! It writes nothing else and reads no Markdown file.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use sim_core::choices::Moment;
use sim_core::state::Side;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs, SupplyStaff};

use crate::common;

const SOURCE_84: &str = include_str!("scenario_84_the_plan_as_an_order.rs");

const NOT_BEFORE_6: &str = "# The limit that tells the plan to wait (decision 0159, 9).\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"not_before\"\nintent = 1\nfrom_day = 6\n";
const FORCE_2: &str = "\n# Task force 2: one placeholder cruiser, stopped in port at base 1, usable\n# from day 5.\n[[task_force]]\nid = 2\nname = \"Placeholder cruiser force 2\"\nside = \"allied\"\nat_base = 1\navailable_from = 5\nmission = \"surface_combat\"\n\n[[ship]]\nid = 3\nname = \"Placeholder cruiser 3\"\nclass = 1\nside = \"allied\"\nfuel_t = 2000\ntask_force = 2\n";
const WAIT_FOR_2: &str = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"wait_for\"\nintent = 1\ntask_force = 2\n";
const PLACE_C: &str = "\n# Japanese base 4 at C: nothing at it.\n[[base]]\nid = 4\nname = \"Placeholder airfield C\"\nside = \"japanese\"\nq = 0\nr = 6\nsupply_t = 0\nfuel_t = 0\n\n# Task force 3: one placeholder troop transport, stopped in port at base 1.\n[[task_force]]\nid = 3\nname = \"Placeholder transport force 3\"\nside = \"allied\"\nat_base = 1\nmission = \"transport\"\n\n[[ship]]\nid = 4\nname = \"Placeholder troop transport 4\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000\ntask_force = 3\n\n[[land_unit]]\nid = 3\nname = \"Placeholder Third\"\nside = \"allied\"\nmen = 600\nq = 0\nr = 0\n";
const INTENT_3: &str = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 3\nintent_kind = \"take_place\"\nhex = [0, 6]\n\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 3\ntask_forces = [3]\nland_units = [3]\nsource = 1\n\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"after\"\nintent = 1\nother_intent = 3\n";

fn path_of(name: &str) -> PathBuf {
    common::data_dir().join(format!("scenarios/{name}.toml"))
}

fn text_of(name: &str) -> String {
    std::fs::read_to_string(path_of(name)).expect("the scenario file reads")
}

/// The variant texts of scenario 81, made as scenario 84's `variants` makes
/// them.
fn force_and_after() -> (String, String) {
    let s81 = text_of("81_told_to_wait");
    let without = common::replace_once(&s81, NOT_BEFORE_6, "");
    let force = format!(
        "{}\n{WAIT_FOR_2}",
        common::replace_once(
            &without,
            "# The land units",
            &format!("{FORCE_2}\n# The land units")
        )
    );
    let after = common::replace_once(
        &common::replace_once(&without, "# Task force 1:", &format!("{PLACE_C}\n# Task force 1:")),
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"prepare\"\n",
        &format!("{INTENT_3}\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"prepare\"\n"),
    );
    (force, after)
}

/// The six cases: (file stem of the output, scenario file, its text, the
/// morning).
fn cases() -> Vec<(&'static str, &'static str, String, u32)> {
    let (force, after) = force_and_after();
    vec![
        (
            "1_scenario_80_day_2",
            "80_a_take_that_waits",
            text_of("80_a_take_that_waits"),
            2,
        ),
        (
            "2_scenario_81_main_day_3",
            "81_told_to_wait",
            text_of("81_told_to_wait"),
            3,
        ),
        (
            "3_scenario_81_wait_for_a_force_day_3",
            "81_told_to_wait",
            force,
            3,
        ),
        (
            "4_scenario_81_after_another_place_day_3",
            "81_told_to_wait",
            after,
            3,
        ),
        (
            "5_scenario_82_day_3",
            "82_how_much_must_be_known",
            text_of("82_how_much_must_be_known"),
            3,
        ),
        (
            "6_scenario_79_day_3",
            "79_fed_before_it_is_taken",
            text_of("79_fed_before_it_is_taken"),
            3,
        ),
    ]
}

fn load(name: &str, text: &str) -> Scenario {
    common::from_text_with(text, &path_of(name), &common::frozen()).expect("the scenario loads")
}

/// The answers of the seat's `order` tool on `morning`, by intent, and the
/// refusals' words.
fn by_the_tool(name: &str, text: &str, morning: u32) -> (BTreeMap<u32, Value>, Vec<String>) {
    let mut scenario = load(name, text);
    assert!(
        scenario.scripted_orders().is_empty() && scenario.scripted_orders_inside_day().is_empty(),
        "{name}: the scenario has scripted orders, which this helper does not hand in"
    );
    let commands = scenario.scripted_commands().clone();
    scenario.replace_commands(BTreeMap::new());
    let title = scenario.name.clone();
    let mut session = sim_agent::Session::new(
        scenario.world,
        &title,
        scenario.days,
        sim_agent::Seat::Player,
        sim_agent::Seat::Script(BTreeMap::new()),
    )
    .with_staffs(staffs())
    .with_commands(commands);
    for _ in 1..morning {
        let a = session
            .call(Side::Allied, "end_day", &json!({}))
            .expect("end_day answers");
        assert_eq!(a["resolved"], true, "{name}: {a}");
    }
    assert_eq!(session.day(), morning - 1);
    let mut answers = BTreeMap::new();
    let mut refused = Vec::new();
    for intent in 1..=9u32 {
        match session.call(Side::Allied, "order", &json!({ "intent": intent })) {
            Ok(answer) => {
                answers.insert(intent, answer);
            }
            Err(error) => {
                assert_eq!(
                    error.message,
                    format!("intent:{intent} is not an intent in force (command_state lists the intents)"),
                    "{name}: an unexpected refusal"
                );
                refused.push(intent.to_string());
            }
        }
    }
    (answers, refused)
}

/// The orders as scenario 84's test reads them: `plan_order` on the Allied
/// view at the staff's moment of `morning`.
fn as_scenario_84_reads(name: &str, text: &str, morning: u32) -> BTreeMap<u32, Value> {
    let mut scenario = load(name, text);
    let held = staffs();
    let mut out = BTreeMap::new();
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                if day != morning {
                    return;
                }
                if let Moment::Orders { .. } = moment {
                    let view = world.side_view(Side::Allied);
                    for intent in 1..=9u32 {
                        if let Ok(answer) = sim_agent::plan_order(&view, &SupplyStaff, intent) {
                            out.insert(intent, answer);
                        }
                    }
                }
            })
            .expect("a day resolves");
        if summary.is_none() || day >= morning {
            break;
        }
    }
    out
}

fn without_inside_day(v: &Value) -> Value {
    let mut v = v.clone();
    if let Some(map) = v.as_object_mut() {
        map.remove("inside_day");
    }
    v
}

#[test]
#[ignore = "a helper for decision 0159's proof, run by hand: see the file's comment"]
fn write_the_six_plan_orders() {
    for constant in [NOT_BEFORE_6, FORCE_2, WAIT_FOR_2, PLACE_C, INTENT_3] {
        let quoted = format!("{constant:?}");
        assert!(
            SOURCE_84.contains(&quoted),
            "a constant here is not word for word scenario 84's: {quoted}"
        );
    }
    let out = std::env::var("FLAGPILOT_0159_ORDERS_OUT")
        .expect("FLAGPILOT_0159_ORDERS_OUT names the folder to write into");
    let out = Path::new(&out);
    std::fs::create_dir_all(out).expect("the folder can be made");
    let mut check = String::new();
    let mut differ = 0;
    for (stem, name, text, morning) in cases() {
        let (answers, refused) = by_the_tool(name, &text, morning);
        assert!(
            !answers.is_empty(),
            "{stem}: the tool answered for no intent"
        );
        let read = as_scenario_84_reads(name, &text, morning);
        writeln!(
            check,
            "{stem}: {name}, morning {morning}: answered for intent(s) {:?}; refused, as not in force: {}",
            answers.keys().collect::<Vec<_>>(),
            refused.join(", ")
        )
        .unwrap();
        if answers.keys().collect::<Vec<_>>() != read.keys().collect::<Vec<_>>() {
            differ += 1;
            writeln!(
                check,
                "  DIFFERS: scenario 84's reading has intent(s) {:?}",
                read.keys().collect::<Vec<_>>()
            )
            .unwrap();
        }
        for (intent, answer) in &answers {
            let file = out.join(format!("{stem}.intent{intent}.json"));
            let mut body = serde_json::to_string_pretty(answer).expect("JSON");
            body.push('\n');
            std::fs::write(&file, body).expect("the file is written");
            let same = read.get(intent).map(without_inside_day) == Some(without_inside_day(answer));
            if !same {
                differ += 1;
            }
            writeln!(
                check,
                "  intent {intent}: {} ; {}",
                file.display(),
                if same {
                    "the same as scenario 84's reading"
                } else {
                    "DIFFERS from scenario 84's reading"
                }
            )
            .unwrap();
        }
    }
    std::fs::write(out.join("check.txt"), &check).expect("check.txt is written");
    print!("{check}");
    assert_eq!(
        differ, 0,
        "an answer differs from scenario 84's reading: see check.txt"
    );
}
