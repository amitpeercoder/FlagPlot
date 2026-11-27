//! Scenario 87: the plan order of a take the staff formed. The commander
//! says "control the island"; the take of its airfield is the staff's.
//! The staff hands in a plan order for that take as it does for a take
//! of the commander's, and the seat's tool answers for it.
//!
//! Proves decision 0164 against a hand calculation. It has no scenario
//! file of its own: it runs scenario 76's main game as scenario 76's own
//! test runs it, and reads the order at the staff's hour of the morning
//! named, before that morning's decisions are made, as scenario 84 reads
//! its orders.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The game
//!
//! Scenario 76's main game (its own hand calculation has every day).
//! Intent 1 is the commander's, control island 1. Intent 2 is the
//! staff's, take the place at B, (4,0), with a plan of its own. Intent 8
//! is the staff's, ready a landing by task force 1 at B, under intent 2.
//! Leave to prepare and leave to launch are given. The side holds no
//! estimate and no count of the enemy at B.
//!
//! The landing is begun on day 1 and is ready on day 2 (loading 100 and
//! rehearsal 100, at 100 a day). The take's plan is released by the
//! decisions of the morning of day 2. Supply is loaded on day 2 and unit
//! 1 on day 3; the force sails on day 4 and its first men are ashore
//! that day. Base 2 is the side's at the end of day 4, and the plan is
//! marked complete by the decisions of the morning of day 5.
//!
//! ## The order of intent 2
//!
//! | Morning | Plan | Prepare | Move | Land | Waits for | Men wanted, to hand | Expects to be ashore |
//! | --- | --- | --- | --- | --- | --- | --- | --- |
//! | 2 | preparing | under way | to come | to come | nothing | none | day 4 |
//! | 3 | released | done | under way | to come | nothing | none | day 4 |
//! | 4 | released | done | under way | to come | nothing | none | day 4 |
//! | 5 | released | done | done | done | nothing | none | none: it is ashore |
//!
//! The mission is the take's statement, as the staff's reasons give it.
//! On each of the four mornings its one task is intent 8, ready a
//! landing, in the phase prepare.
//!
//! The need to know: on the morning of day 2, the plan preparing, the
//! order gives it as the order of any take that is preparing does. The
//! risk, balanced, asks a count of the enemy at B; the commander asked
//! none; none is held; nobody is looking and no force was moved to look;
//! it is not met. Nothing of the side's can look, so the take does not
//! wait on it, and it is not among what the order waits for. On the
//! mornings of days 3, 4 and 5 the plan is released and none is asked.
//! (Restated by the architect on Nov 19, 2026: the first writing had "no
//! need to know is asked" on every morning.)
//!
//! The day it expects, by the sums of a latest day (the day it starts
//! from, plus the days to embark, plus the days of passage, less 1):
//!
//! - **Morning 2,** preparing: it starts from the day the landing is
//!   ready, day 2. To embark: 1 day for the supply and 1 for unit 1,
//!   ashore at A. The passage, 16 hours, is 1 day. 2 + 2 + 1 - 1 = 4.
//! - **Morning 3,** released: it starts from today, day 3. The supply
//!   was loaded on day 2, so 1 day to embark, for unit 1. 3 + 1 + 1 - 1
//!   = 4.
//! - **Morning 4:** unit 1 is aboard, so no day to embark. 4 + 0 + 1 - 1
//!   = 4.
//! - **Morning 5:** unit 1 is ashore at B and B is the side's: no day.
//!
//! ## The order of intent 1
//!
//! On the morning of day 2 the order of intent 1, control island 1, is
//! what it was before this decision: it has no phases, no men and no
//! day, and intents 2 and 8 are among its tasks.
//!
//! ## The orders together
//!
//! On the morning of day 2 the staff hands in two orders, of intent 1
//! and then of intent 2, and no other.
//!
//! # Properties
//!
//! - **The tool answers for the take.** The seat's tool `order` answers
//!   for intent 2 with the order above, and for intent 1 as before.
//! - **It refuses the rest.** It refuses intent 3, the staff's hold of
//!   base 2, and intent 8, the staff's landing, with the words it
//!   refuses any intent the side does not have.
//! - **Nothing is stored.** Asking for the order of intent 2 on every
//!   morning changes no daily hash of scenario 76's main game.
//! - **Nothing more is told.** The order of intent 2 is made from the
//!   seat's own view: two worlds that differ only in what the Allied
//!   side cannot see and has not been told give the same order.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::Value;
use sim_core::choices::Moment;
use sim_core::command::{
    IntentKind, Knowledge, Maker, NeedToKnow, OrderTask, Phase, PhaseEnd, PhaseName, PhaseStage,
    PlanId, PlanOrder, PlanState, Staff,
};
use sim_core::hex::Hex;
use sim_core::ids::{IntentId, TaskForceId};
use sim_core::state::Side;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs, SupplyStaff};

use crate::common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const B: Hex = Hex { q: 4, r: 0 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/76_controlling_an_island.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// The frozen step 1 figures, as scenario 76's own test loads it.
fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &common::frozen()).expect("the scenario loads")
}

/// The words the seat's tool refuses an intent the side does not have in.
fn refusal(n: u32) -> String {
    format!("intent:{n} is not an intent in force (command_state lists the intents)")
}

/// Scenario 76's main game, played as its own test plays it, with the
/// Allied plan orders of each morning read at the staff's moment
/// (`Moment::Orders`: the commander's commands taken in, before the staff
/// is asked), each order's answer as the seat's tool gives it from the same
/// view, and the tool's answer to each other intent of the staff's in force.
struct Played {
    orders: BTreeMap<u32, Vec<PlanOrder>>,
    answers: BTreeMap<u32, BTreeMap<u32, Value>>,
    /// The tool's answer to each staff intent in force that has no order,
    /// by morning and intent: its refusal's words, or the answer it gave.
    others: BTreeMap<u32, BTreeMap<u32, Result<String, Value>>>,
    /// The plan of intent 2 as the command state has it on each morning.
    plan_of_2: BTreeMap<u32, PlanId>,
    /// Every statement of intent 2 the staff's reasons give, on any day.
    statements_of_2: Vec<String>,
    hashes: Vec<u64>,
}

fn play(text: &str, ask: bool) -> Played {
    let mut scenario = load(text);
    let held = staffs();
    let mut played = Played {
        orders: BTreeMap::new(),
        answers: BTreeMap::new(),
        others: BTreeMap::new(),
        plan_of_2: BTreeMap::new(),
        statements_of_2: Vec::new(),
        hashes: Vec::new(),
    };
    loop {
        let day = scenario.world.day() + 1;
        let mut morning: Option<Vec<PlanOrder>> = None;
        let mut answers = BTreeMap::new();
        let mut others = BTreeMap::new();
        let mut plan_of_2 = None;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                if !ask {
                    return;
                }
                if let Moment::Orders { .. } = moment {
                    let view = world.side_view(Side::Allied);
                    let orders = SupplyStaff.plan_orders(view);
                    for o in &orders {
                        let answer = sim_agent::plan_order(&view, &SupplyStaff, o.intent.raw())
                            .expect("the tool answers for an intent that has an order");
                        answers.insert(o.intent.raw(), answer);
                    }
                    let state = view.command_state();
                    plan_of_2 = state.intents.get(&IntentId::new(2)).map(|i| i.plan);
                    for i in state.intents_in_force() {
                        if orders.iter().any(|o| o.intent == i.id) {
                            continue;
                        }
                        let n = i.id.raw();
                        let got = match sim_agent::plan_order(&view, &SupplyStaff, n) {
                            Err(e) => Ok(e.message),
                            Ok(a) => Err(a),
                        };
                        others.insert(n, got);
                    }
                    morning = Some(orders);
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        if let Some(handed) = scenario.world.staff_handed_in().get(&Side::Allied) {
            let reasons = handed
                .orders
                .iter()
                .map(|o| &o.reason)
                .chain(handed.commands.iter().map(|c| &c.reason))
                .chain(handed.no_orders.iter());
            for r in reasons {
                if r.serves.intent == Some(IntentId::new(2)) && !r.serves.statement.is_empty() {
                    played.statements_of_2.push(r.serves.statement.clone());
                }
                for link in &r.serves.chain {
                    if link.intent == IntentId::new(2) {
                        played.statements_of_2.push(link.statement.clone());
                    }
                }
            }
        }
        if ask {
            played
                .orders
                .insert(day, morning.expect("the staff's moment was passed"));
            played.answers.insert(day, answers);
            played.others.insert(day, others);
            if let Some(plan) = plan_of_2 {
                played.plan_of_2.insert(day, plan);
            }
        }
    }
    played.hashes = scenario.world.day_hashes().to_vec();
    played
}

impl Played {
    /// The one order of a morning for `intent`.
    fn order(&self, day: u32, intent: u32) -> &PlanOrder {
        self.orders[&day]
            .iter()
            .find(|o| o.intent.raw() == intent)
            .unwrap_or_else(|| panic!("an order for intent {intent} on morning {day}"))
    }
}

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text(), true))
}

fn take_phases(prepare: PhaseStage, moving: PhaseStage, land: PhaseStage) -> Vec<Phase> {
    vec![
        Phase {
            name: PhaseName::Prepare,
            ends: PhaseEnd::Released,
            stage: prepare,
        },
        Phase {
            name: PhaseName::Move,
            ends: PhaseEnd::FirstManAshore(B),
            stage: moving,
        },
        Phase {
            name: PhaseName::Land,
            ends: PhaseEnd::PlaceIsTheSides(B),
            stage: land,
        },
    ]
}

use PhaseStage::{Done, ToCome, UnderWay};

/// Intent 8, ready a landing by task force 1 at B, in the phase prepare.
fn landing_task() -> Vec<OrderTask> {
    vec![OrderTask {
        intent: IntentId::new(8),
        kind: IntentKind::ReadyLanding {
            force: TaskForceId::new(1),
            hex: B,
            by_day: None,
        },
        maker: sim_core::command::Maker::Staff,
        phase: Some(PhaseName::Prepare),
    }]
}

/// One row of the table of the order of intent 2: everything but the
/// mission, which `the_mission_is_the_takes_statement` asserts.
fn assert_row(
    day: u32,
    state: PlanState,
    phases: (PhaseStage, PhaseStage, PhaseStage),
    expected_ashore: Option<u32>,
) {
    let g = main_game();
    let o = g.order(day, 2);
    assert_eq!(o.intent, IntentId::new(2), "morning {day}");
    assert_eq!(
        o.plan, g.plan_of_2[&day],
        "morning {day}: the take's own plan"
    );
    assert_eq!(o.plan_state, state, "morning {day}");
    assert_eq!(
        o.phases,
        take_phases(phases.0, phases.1, phases.2),
        "morning {day}"
    );
    // Waits for nothing.
    assert!(o.waits_for.is_empty(), "morning {day}: {:?}", o.waits_for);
    // Men wanted and to hand: none.
    assert_eq!(o.men, None, "morning {day}");
    // Its one task is intent 8, ready a landing, in the phase prepare.
    assert_eq!(o.tasks, landing_task(), "morning {day}");
    assert_eq!(o.expected_ashore, expected_ashore, "morning {day}");
    // The need to know: on the morning of day 2, a count asked by the
    // risk, nothing held, nobody looking, not met; on the others none.
    let want = (day == 2).then(|| NeedToKnow {
        plan: g.plan_of_2[&day],
        intent: IntentId::new(2),
        place: Hex::new(4, 0),
        asked: Knowledge::Count,
        asked_from: None,
        held: None,
        looking: None,
        moved_to_look: None,
        met: false,
    });
    assert_eq!(o.need_to_know, want, "morning {day}");
}

// ---------------------------------------------------------------- The order of intent 2

#[test]
fn morning_2_preparing() {
    assert_row(2, PlanState::Preparing, (UnderWay, ToCome, ToCome), Some(4));
}

#[test]
fn morning_3_released() {
    assert_row(3, PlanState::Released, (Done, UnderWay, ToCome), Some(4));
}

#[test]
fn morning_4_released() {
    assert_row(4, PlanState::Released, (Done, UnderWay, ToCome), Some(4));
}

#[test]
fn morning_5_ashore() {
    // None: it is ashore.
    assert_row(5, PlanState::Released, (Done, Done, Done), None);
}

#[test]
fn the_mission_is_the_takes_statement() {
    let g = main_game();
    assert!(
        !g.statements_of_2.is_empty(),
        "the staff's reasons state intent 2"
    );
    for day in 2..=5 {
        let o = g.order(day, 2);
        assert!(
            g.statements_of_2.contains(&o.mission),
            "morning {day}: {:?} against {:?}",
            o.mission,
            g.statements_of_2
        );
    }
}

// ---------------------------------------------------------------- The order of intent 1

#[test]
fn the_order_of_intent_1_on_morning_2_is_as_before() {
    let o = main_game().order(2, 1);
    assert_eq!(o.intent, IntentId::new(1));
    assert!(o.phases.is_empty(), "{:?}", o.phases);
    assert_eq!(o.men, None);
    assert_eq!(o.expected_ashore, None);
    let tasks: Vec<IntentId> = o.tasks.iter().map(|t| t.intent).collect();
    for n in [2, 8] {
        assert!(tasks.contains(&IntentId::new(n)), "{tasks:?}");
    }
}

// ---------------------------------------------------------------- The orders together

#[test]
fn on_morning_2_two_orders_of_intents_1_and_2() {
    let intents: Vec<u32> = main_game().orders[&2]
        .iter()
        .map(|o| o.intent.raw())
        .collect();
    assert_eq!(intents, vec![1, 2]);
}

// ---------------------------------------------------------------- Properties

#[test]
fn the_tool_answers_for_the_take() {
    let g = main_game();
    let mut needs = Vec::new();
    for day in 2..=5 {
        let o = g.order(day, 2);
        let a = &g.answers[&day][&2];
        assert_eq!(a["kind"], "plan_order", "morning {day}");
        assert_eq!(a["intent"], 2, "morning {day}");
        assert_eq!(a["mission"], serde_json::json!(o.mission), "morning {day}");
        assert_eq!(a["plan"]["number"], o.plan.raw(), "morning {day}");
        assert_eq!(a["plan"]["state"], o.plan_state.name(), "morning {day}");
        let stages: Vec<(String, String)> = a["phases"]
            .as_array()
            .expect("phases")
            .iter()
            .map(|p| (p["name"].to_string(), p["stage"].to_string()))
            .collect();
        let want: Vec<(String, String)> = o
            .phases
            .iter()
            .map(|p| {
                (
                    format!("{:?}", p.name.name()),
                    format!("{:?}", p.stage.name()),
                )
            })
            .collect();
        assert_eq!(stages, want, "morning {day}");
        assert_eq!(a["waits_for"], serde_json::json!([]), "morning {day}");
        assert!(a["men"].is_null(), "morning {day}");
        if !a["need_to_know"].is_null() {
            needs.push((day, a["need_to_know"].clone()));
        }
        assert_eq!(
            a["expected_ashore"],
            serde_json::json!(o.expected_ashore),
            "morning {day}"
        );
        let tasks: Vec<(u64, String)> = a["tasks"]
            .as_array()
            .expect("tasks")
            .iter()
            .map(|t| {
                (
                    t["intent"].as_u64().expect("a number"),
                    t["phase"].as_str().unwrap_or("").to_string(),
                )
            })
            .collect();
        assert_eq!(tasks, vec![(8, "prepare".to_string())], "morning {day}");
        let text: Vec<&str> = a["text"]
            .as_array()
            .expect("text")
            .iter()
            .filter_map(Value::as_str)
            .collect();
        let ashore = match o.expected_ashore {
            Some(d) => format!("It expects to be ashore on day {d}."),
            None => "Its men are ashore.".to_string(),
        };
        assert!(text.contains(&ashore.as_str()), "morning {day}: {text:?}");
    }
    // And for intent 1, as before: an order whose answer is the order's.
    let a = &g.answers[&2][&1];
    assert_eq!(a["kind"], "plan_order");
    assert_eq!(a["intent"], 1);
    assert_eq!(a["phases"], serde_json::json!([]));
    assert!(a["expected_ashore"].is_null());
    // The need to know is given on the morning of day 2 and on no other,
    // and it is not met.
    assert_eq!(needs.len(), 1, "{needs:?}");
    assert_eq!(needs[0].0, 2, "{needs:?}");
    assert_eq!(needs[0].1["met"], serde_json::json!(false), "{needs:?}");
}

#[test]
fn it_refuses_the_rest() {
    let g = main_game();
    // Intent 3, the staff's hold of base 2, and intent 8, the staff's
    // landing, on the morning of day 2.
    for n in [3, 8] {
        assert_eq!(
            g.others[&2].get(&n),
            Some(&Ok(refusal(n))),
            "intent {n} on morning 2"
        );
    }
    // Every staff intent in force that is not a take is refused on every
    // morning, in the words an intent the side does not have is refused in.
    for (day, others) in &g.others {
        for (n, got) in others {
            assert_eq!(got, &Ok(refusal(*n)), "intent {n} on morning {day}");
        }
    }
    // And every order is of the commander's intent or of a take the staff
    // formed.
    for (day, orders) in &g.orders {
        for o in orders {
            assert!(
                o.intent == IntentId::new(1) || o.intent == IntentId::new(2),
                "morning {day}: an order of intent {}",
                o.intent.raw()
            );
        }
    }
}

#[test]
fn nothing_is_stored() {
    let never = play(&text(), false);
    assert_eq!(never.hashes, main_game().hashes);
}

#[test]
fn nothing_more_is_told() {
    // An enemy base off the Allied map, unseen and untold: two worlds that
    // differ only in its supply give the same orders, as scenario 76's own
    // test varies it.
    let hidden = |supply: u32| {
        common::replace_once(
            &text(),
            COMMANDS_MARKER,
            &format!(
                "[[base]]\nid = 9\nname = \"Placeholder base 9\"\nside = \"japanese\"\nq = 3\nr = 4\nsupply_t = {supply}\non_map = false\n\n{COMMANDS_MARKER}"
            ),
        )
    };
    let a = play(&hidden(50), true);
    let b = play(&hidden(400), true);
    assert_ne!(a.hashes, b.hashes, "the two worlds do not differ");
    assert_eq!(a.orders, b.orders);
    assert_eq!(a.answers, b.answers);
    assert_eq!(a.orders, main_game().orders);
}

#[test]
fn the_staff_formed_the_take() {
    // The reading this file rests on: intent 2 is the staff's take of B.
    let mut scenario = load(&text());
    let held = staffs();
    scenario
        .next_day_watched(as_staffs(&held), &mut |_, _| {})
        .expect("a day resolves");
    let state = scenario.world.command_state(Side::Allied);
    let two = &state.intents[&IntentId::new(2)];
    assert_eq!(two.maker, Maker::Staff);
    assert!(matches!(two.kind, IntentKind::TakePlace { hex, .. } if hex == B));
}
