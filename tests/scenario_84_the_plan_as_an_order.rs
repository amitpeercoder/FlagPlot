//! Scenario 84: the plan as an order. For each intent of the commander's
//! the staff hands in an order: the mission, the phases with what ends
//! each and what each waits for, what the plan needs and has, and the
//! tasks under it. It is read here on mornings of scenarios 79 to 82,
//! whose games are already worked by hand, so no new game is played.
//!
//! Proves decision 0159, rule E (items 15 to 17), step 6 of the opening
//! of stage 4, against a hand calculation. It has no scenario file of its
//! own: it runs scenarios 79, 80, 81 and 82 as their own tests run them,
//! and reads the order at the staff's hour of the morning named, before
//! that morning's decisions are made.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The order
//!
//! An order is worked when it is asked for, from the side's command
//! state and the staff's own sums, and is stored nowhere. It has:
//!
//! - **the intent** it is of, and its **mission** in words (the intent's
//!   statement, as the staff's reasons give it);
//! - **the plan** and its state;
//! - **the phases,** in order, each with a name, the condition that ends
//!   it, and whether it is done, under way or to come;
//! - **what it waits for:** the things that must be so before the phase
//!   under way can end, each with its kind (men, leave, a day, a force,
//!   an intent, knowledge), whether it is met, and the day it is expected
//!   to be met where the staff can work one;
//! - **what it needs and has:** the men wanted and the men to hand, where
//!   a count of the enemy is held;
//! - **the day it expects** to be ashore, where it can work one, by the
//!   sums the take uses for a latest day;
//! - **the need to know,** as step 4 gives it;
//! - **the tasks** under it: the staff's intents, each with its phase.
//!
//! The phases of a "take a place": **prepare**, which ends when the plan
//! is released; **move**, which ends when the first man is ashore at the
//! place; **land**, which ends when the place is the side's. The phases
//! of a "hold" of a base that is not the side's: **wait**, which ends
//! when the base is the side's; then **hold**, which does not end. A
//! "hold" of a base that is the side's has the one phase, hold.
//!
//! A day on which a task had no order is not in the order.
//!
//! ## Scenario 80, a take that waits (intent 1)
//!
//! | Morning | Plan | Prepare | Move | Land | Waits for | Men wanted, to hand | Expects to be ashore |
//! | --- | --- | --- | --- | --- | --- | --- | --- |
//! | 2 | draft | to come | to come | to come | men: not met, expected day 4 | 1,200, 600 | day 8 |
//! | 5 | preparing | under way | to come | to come | nothing unmet | 1,200, 1,200 | day 8 |
//! | 6 | released | done | under way | to come | nothing | 1,200, 1,200 | day 8 |
//! | 9 | released | done | done | done | nothing | none: no count is held | none: it is ashore |
//!
//! On morning 2 the take has no task under it. On mornings 5 and 6 its
//! one task is intent 2, ready a landing. The men landed on day 8 found
//! no enemy at B and said so, which cleared the count the side held, and
//! the order gives men only where a count is held: on morning 9 it gives
//! none (restated by the architect on Nov 19, 2026; the first table had
//! 1,200 and 1,200). On morning 9 base 2 is the
//! side's and the plan is not yet marked complete: that is done by the
//! staff later that morning.
//!
//! The day it expects: a landing begun on day 4 is ready on day 5, takes
//! 3 days to embark and sails on day 8, and the passage is inside a day.
//!
//! With a latest day of 7 (the property of scenario 80), on morning 2:
//! the plan is a draft, it waits for men, not met, with no day expected,
//! and no day ashore is expected.
//!
//! ## Scenario 81, told to wait (intent 1)
//!
//! | Case | Morning | Plan | Prepare | Waits for | Expects to be ashore |
//! | --- | --- | --- | --- | --- | --- |
//! | Main | 3 | preparing | under way | a day: not before day 6, not met, expected day 6 | day 9 |
//! | Main | 7 | released | done | nothing | day 9 |
//! | Leave withheld | 3 | preparing | under way | a day, as above; leave to launch, not met, no day | none |
//! | Wait for a force | 3 | preparing | under way | a force: task force 2, not met, expected day 5 | day 8 |
//! | After another place | 3 | preparing | under way | an intent: intent 3, not met, no day | none |
//!
//! No count is held in scenario 81, so the order gives no men wanted. A
//! thing with no day expected leaves the day ashore without one.
//!
//! ## Scenario 82, how much must be known (intent 1)
//!
//! | Morning | Plan | Prepare | Waits for | The need to know |
//! | --- | --- | --- | --- | --- |
//! | 2 | preparing | under way | knowledge: met | 150, seen from the air, of day 1, 1 day old; met |
//! | 3 | preparing | under way | knowledge: not met, no day | the same count, 2 days old; not met; air group 1 looking since day 3 |
//! | 4 | preparing | under way | knowledge: met | 150, of day 3, 1 day old; met |
//! | 5 | released | done | nothing | not asked: the plan is released |
//!
//! The men wanted and to hand are 300 and 600 on mornings 3, 4 and 5.
//!
//! The need to know is step 4's, which names the group the staff sends
//! that morning: on morning 3, air group 1, as scenario 82's own table
//! has it (restated by the architect on Nov 19, 2026; the first table
//! had "nobody looking yet"). The plan's state, phases and tasks are as
//! they stand before the morning's decisions.
//!
//! ## Scenario 79, fed before it is taken (intent 1, a hold)
//!
//! On the morning of day 3: the phases are wait, under way, ending when
//! base 2 is the side's, and hold, to come. It waits for nothing the
//! staff can give a day to. Its one task is intent 2, keep base 3
//! supplied. No men are wanted and no day is expected.
//!
//! # Properties
//!
//! - **Nothing more is told.** Each order is made from the seat's own
//!   view: two worlds that differ only in what the Allied side cannot
//!   see and has not been told give the same orders.
//! - **Nothing is stored.** Asking for an order changes no hash; a game
//!   in which it is asked on every morning has the daily hashes of the
//!   game in which it is never asked.
//! - **A seat's tool gives it.** The tool answers for an intent of the
//!   commander's by its number, and for a take the staff formed (decision
//!   0164), and refuses any other intent of the staff's and
//!   an intent that is not the side's with the reason it refuses any
//!   intent the side does not have.
//! - **No "no order".** In none of the orders above is there a line for a
//!   task that was given no order.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::belief::GroundGrade;
use sim_core::choices::Moment;
use sim_core::command::{
    Awaited, CountRange, HeldCount, IntentKind, Knowledge, Leave, Looking, MenCount, NeedToKnow,
    OrderTask, Phase, PhaseEnd, PhaseName, PhaseStage, PlanId, PlanOrder, PlanState, Staff, Waited,
    WaitedFor,
};
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId, TaskForceId};
use sim_core::state::Side;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs, SupplyStaff};

mod common;

fn path_of(name: &str) -> PathBuf {
    common::data_dir().join(format!("scenarios/{name}.toml"))
}

fn text_of(name: &str) -> String {
    std::fs::read_to_string(path_of(name)).expect("the scenario file reads")
}

fn load(name: &str, text: &str) -> Scenario {
    common::from_text_with(text, &path_of(name), &common::frozen()).expect("the scenario loads")
}

/// A game played as its own test plays it, with the Allied plan orders of
/// each morning read at the staff's moment (`Moment::Orders`: the
/// commander's commands taken in, before the staff is asked), and each
/// order's answer as the seat's tool gives it, from the same view.
struct Played {
    orders: BTreeMap<u32, Vec<PlanOrder>>,
    /// The tool's answer for each order of each morning, by intent.
    answers: BTreeMap<u32, BTreeMap<u32, serde_json::Value>>,
    /// The tool's refusals of each morning: of intent 1 asked on the
    /// Japanese side, of each staff intent that is not a take, and of
    /// intent 99.
    refusals: BTreeMap<u32, Vec<(Side, u32, String)>>,
    /// The commander's intents in force on each morning.
    commanders: BTreeMap<u32, Vec<u32>>,
    /// The takes the staff formed in force on each morning.
    staff_takes: BTreeMap<u32, Vec<u32>>,
    hashes: Vec<u64>,
}

fn play(name: &str, text: &str, ask: bool) -> Played {
    let mut scenario = load(name, text);
    let held = staffs();
    let mut played = Played {
        orders: BTreeMap::new(),
        answers: BTreeMap::new(),
        refusals: BTreeMap::new(),
        commanders: BTreeMap::new(),
        staff_takes: BTreeMap::new(),
        hashes: Vec::new(),
    };
    loop {
        let day = scenario.world.day() + 1;
        let mut morning: Option<Vec<PlanOrder>> = None;
        let mut answers = BTreeMap::new();
        let mut refusals = Vec::new();
        let mut commanders = Vec::new();
        let mut staff_takes = Vec::new();
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
                    // The staff's intents that are not a take: a take the
                    // staff formed has an order (decision 0164) and is
                    // answered above.
                    let staff_intents: Vec<u32> = view
                        .command_state()
                        .intents_in_force()
                        .filter(|i| i.maker == sim_core::command::Maker::Staff)
                        .filter(|i| !matches!(i.kind, IntentKind::TakePlace { .. }))
                        .map(|i| i.id.raw())
                        .collect();
                    for number in staff_intents.into_iter().chain([99]) {
                        let refused = sim_agent::plan_order(&view, &SupplyStaff, number)
                            .expect_err("refused");
                        refusals.push((Side::Allied, number, refused.message));
                    }
                    let japanese = world.side_view(Side::Japanese);
                    let refused =
                        sim_agent::plan_order(&japanese, &SupplyStaff, 1).expect_err("refused");
                    refusals.push((Side::Japanese, 1, refused.message));
                    for i in view.command_state().intents_in_force() {
                        match i.maker {
                            sim_core::command::Maker::Commander => commanders.push(i.id.raw()),
                            sim_core::command::Maker::Staff => {
                                if matches!(i.kind, IntentKind::TakePlace { .. }) {
                                    staff_takes.push(i.id.raw());
                                }
                            }
                        }
                    }
                    morning = Some(orders);
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        if ask {
            played
                .orders
                .insert(day, morning.expect("the staff's moment was passed"));
            played.answers.insert(day, answers);
            played.refusals.insert(day, refusals);
            played.commanders.insert(day, commanders);
            played.staff_takes.insert(day, staff_takes);
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

const NOT_BEFORE_6: &str = "# The limit that tells the plan to wait (decision 0159, 9).\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"not_before\"\nintent = 1\nfrom_day = 6\n";
const FORCE_2: &str = "\n# Task force 2: one placeholder cruiser, stopped in port at base 1, usable\n# from day 5.\n[[task_force]]\nid = 2\nname = \"Placeholder cruiser force 2\"\nside = \"allied\"\nat_base = 1\navailable_from = 5\nmission = \"surface_combat\"\n\n[[ship]]\nid = 3\nname = \"Placeholder cruiser 3\"\nclass = 1\nside = \"allied\"\nfuel_t = 2000\ntask_force = 2\n";
const WAIT_FOR_2: &str = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"wait_for\"\nintent = 1\ntask_force = 2\n";
const PLACE_C: &str = "\n# Japanese base 4 at C: nothing at it.\n[[base]]\nid = 4\nname = \"Placeholder airfield C\"\nside = \"japanese\"\nq = 0\nr = 6\nsupply_t = 0\nfuel_t = 0\n\n# Task force 3: one placeholder troop transport, stopped in port at base 1.\n[[task_force]]\nid = 3\nname = \"Placeholder transport force 3\"\nside = \"allied\"\nat_base = 1\nmission = \"transport\"\n\n[[ship]]\nid = 4\nname = \"Placeholder troop transport 4\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000\ntask_force = 3\n\n[[land_unit]]\nid = 3\nname = \"Placeholder Third\"\nside = \"allied\"\nmen = 600\nq = 0\nr = 0\n";
const INTENT_3: &str = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 3\nintent_kind = \"take_place\"\nhex = [0, 6]\n\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 3\ntask_forces = [3]\nland_units = [3]\nsource = 1\n\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"after\"\nintent = 1\nother_intent = 3\n";

fn variants() -> Vec<(&'static str, &'static str, String)> {
    let s80 = text_of("80_a_take_that_waits");
    let s81 = text_of("81_told_to_wait");
    let without = common::replace_once(&s81, NOT_BEFORE_6, "");
    let late = common::replace_once(
        &s80,
        "intent_kind = \"take_place\"\nhex = [4, 0]\n",
        "intent_kind = \"take_place\"\nhex = [4, 0]\nby_day = 7\n",
    );
    let withheld = common::replace_once(
        &s81,
        "leave = \"launch\"\ngiven = true\n",
        "leave = \"launch\"\ngiven = false\n",
    );
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
    vec![
        ("80 late", "80_a_take_that_waits", late),
        ("81 withheld", "81_told_to_wait", withheld),
        ("81 force", "81_told_to_wait", force),
        ("81 after", "81_told_to_wait", after),
    ]
}

const B: Hex = Hex { q: 4, r: 0 };

fn games() -> &'static BTreeMap<&'static str, Played> {
    static GAMES: std::sync::OnceLock<BTreeMap<&'static str, Played>> = std::sync::OnceLock::new();
    GAMES.get_or_init(|| {
        let mut all = BTreeMap::new();
        for (key, name) in [
            ("79", "79_fed_before_it_is_taken"),
            ("80", "80_a_take_that_waits"),
            ("81", "81_told_to_wait"),
            ("82", "82_how_much_must_be_known"),
        ] {
            all.insert(key, play(name, &text_of(name), true));
        }
        for (key, name, text) in variants() {
            all.insert(key, play(name, &text, true));
        }
        all
    })
}

fn game(key: &str) -> &'static Played {
    &games()[key]
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

fn not_met(thing: WaitedFor, expected_day: Option<u32>) -> Waited {
    Waited {
        thing,
        met: false,
        expected_day,
    }
}

fn met(thing: WaitedFor) -> Waited {
    Waited {
        thing,
        met: true,
        expected_day: None,
    }
}

fn men(wanted: u32, to_hand: u32) -> Option<MenCount> {
    Some(MenCount { wanted, to_hand })
}

/// Intent 2, ready a landing by task force 1 at B, in the prepare phase.
fn landing_task() -> Vec<OrderTask> {
    vec![OrderTask {
        intent: IntentId::new(2),
        kind: IntentKind::ReadyLanding {
            force: TaskForceId::new(1),
            hex: B,
            by_day: None,
        },
        phase: Some(PhaseName::Prepare),
    }]
}

/// The order's state, plan, mission and intent, as every row has them.
fn assert_head(o: &PlanOrder, state: PlanState) {
    assert_eq!(o.intent, IntentId::new(1));
    assert_eq!(o.plan, PlanId::new(1));
    assert_eq!(o.plan_state, state);
    assert_eq!(o.mission, "take the place at (4,0)");
}

// ---------------------------------------------------------------- Scenario 80

#[test]
fn scenario_80_morning_2_a_draft_that_waits_for_men() {
    let o = game("80").order(2, 1);
    assert_head(o, PlanState::Draft);
    assert_eq!(o.phases, take_phases(ToCome, ToCome, ToCome));
    assert_eq!(
        o.waits_for,
        vec![not_met(
            WaitedFor::Men {
                wanted: 1200,
                to_hand: 600
            },
            Some(4)
        )]
    );
    assert_eq!(o.men, men(1200, 600));
    assert_eq!(o.expected_ashore, Some(8));
    // On morning 2 the take has no task under it.
    assert!(o.tasks.is_empty(), "{:?}", o.tasks);
}

#[test]
fn scenario_80_morning_5_preparing_with_nothing_unmet() {
    let o = game("80").order(5, 1);
    assert_head(o, PlanState::Preparing);
    assert_eq!(o.phases, take_phases(UnderWay, ToCome, ToCome));
    assert!(o.waits_for.iter().all(|w| w.met), "{:?}", o.waits_for);
    assert_eq!(o.men, men(1200, 1200));
    assert_eq!(o.expected_ashore, Some(8));
    assert_eq!(o.tasks, landing_task());
}

#[test]
fn scenario_80_morning_6_released() {
    let o = game("80").order(6, 1);
    assert_head(o, PlanState::Released);
    assert_eq!(o.phases, take_phases(Done, UnderWay, ToCome));
    assert!(o.waits_for.is_empty(), "{:?}", o.waits_for);
    assert_eq!(o.men, men(1200, 1200));
    assert_eq!(o.expected_ashore, Some(8));
    assert_eq!(o.tasks, landing_task());
}

#[test]
fn scenario_80_morning_9_the_place_is_the_sides() {
    let o = game("80").order(9, 1);
    // Base 2 is the side's and the plan is not yet marked complete.
    assert_head(o, PlanState::Released);
    assert_eq!(o.phases, take_phases(Done, Done, Done));
    assert!(o.waits_for.is_empty(), "{:?}", o.waits_for);
    assert_eq!(o.expected_ashore, None);
    assert_eq!(o.tasks, landing_task());
}

#[test]
fn scenario_80_morning_9_men_wanted_and_to_hand() {
    // No count is held: the men ashore found no enemy at B.
    assert_eq!(game("80").order(9, 1).men, None);
}

#[test]
fn scenario_80_a_latest_day_of_7() {
    let o = game("80 late").order(2, 1);
    assert_eq!(o.plan_state, PlanState::Draft);
    assert_eq!(o.mission, "take the place at (4,0) by the end of day 7");
    assert_eq!(
        o.waits_for,
        vec![not_met(
            WaitedFor::Men {
                wanted: 1200,
                to_hand: 600
            },
            None
        )]
    );
    assert_eq!(o.expected_ashore, None);
}

// ---------------------------------------------------------------- Scenario 81

#[test]
fn scenario_81_main() {
    let g = game("81");
    let o = g.order(3, 1);
    assert_head(o, PlanState::Preparing);
    assert_eq!(o.phases, take_phases(UnderWay, ToCome, ToCome));
    assert_eq!(o.waits_for, vec![not_met(WaitedFor::Day(6), Some(6))]);
    assert_eq!(o.expected_ashore, Some(9));
    assert_eq!(o.men, None);
    let o = g.order(7, 1);
    assert_head(o, PlanState::Released);
    assert_eq!(o.phases[0].stage, Done);
    assert!(o.waits_for.is_empty(), "{:?}", o.waits_for);
    assert_eq!(o.expected_ashore, Some(9));
    assert_eq!(o.men, None);
}

#[test]
fn scenario_81_leave_withheld() {
    let o = game("81 withheld").order(3, 1);
    assert_head(o, PlanState::Preparing);
    assert_eq!(o.phases[0].stage, UnderWay);
    assert_eq!(
        o.waits_for,
        vec![
            not_met(WaitedFor::Day(6), Some(6)),
            not_met(WaitedFor::Leave(Leave::Launch), None),
        ]
    );
    assert_eq!(o.expected_ashore, None);
    assert_eq!(o.men, None);
}

#[test]
fn scenario_81_wait_for_a_force() {
    let o = game("81 force").order(3, 1);
    assert_head(o, PlanState::Preparing);
    assert_eq!(o.phases[0].stage, UnderWay);
    assert_eq!(
        o.waits_for,
        vec![not_met(
            WaitedFor::Force(Awaited::TaskForce(TaskForceId::new(2))),
            Some(5)
        )]
    );
    assert_eq!(o.expected_ashore, Some(8));
    assert_eq!(o.men, None);
}

#[test]
fn scenario_81_after_another_place() {
    let o = game("81 after").order(3, 1);
    assert_head(o, PlanState::Preparing);
    assert_eq!(o.phases[0].stage, UnderWay);
    assert_eq!(
        o.waits_for,
        vec![not_met(WaitedFor::Intent(IntentId::new(3)), None)]
    );
    assert_eq!(o.expected_ashore, None);
    assert_eq!(o.men, None);
}

// ---------------------------------------------------------------- Scenario 82

const ASKED: Knowledge = Knowledge::RecentCount(1);

fn seen_150(day: u32, age_days: u32) -> Option<HeldCount> {
    Some(HeldCount {
        men: 150,
        grade: GroundGrade::SeenFromAir,
        day,
        age_days,
        could_be: Some(CountRange {
            lowest: 150,
            highest: Some(150),
        }),
    })
}

fn need(held: Option<HeldCount>, looking: Option<Looking>, met: bool) -> Option<NeedToKnow> {
    Some(NeedToKnow {
        plan: PlanId::new(1),
        intent: IntentId::new(1),
        place: B,
        asked: ASKED,
        asked_from: Some(IntentId::new(1)),
        held,
        looking,
        moved_to_look: None,
        met,
    })
}

#[test]
fn scenario_82_morning_2() {
    let o = game("82").order(2, 1);
    assert_head(o, PlanState::Preparing);
    assert_eq!(o.phases, take_phases(UnderWay, ToCome, ToCome));
    assert_eq!(o.waits_for, vec![met(WaitedFor::Knowledge(ASKED))]);
    assert_eq!(o.need_to_know, need(seen_150(1, 1), None, true));
}

#[test]
fn scenario_82_morning_3() {
    let o = game("82").order(3, 1);
    assert_head(o, PlanState::Preparing);
    assert_eq!(o.phases, take_phases(UnderWay, ToCome, ToCome));
    assert_eq!(
        o.waits_for,
        vec![not_met(WaitedFor::Knowledge(ASKED), None)]
    );
    assert_eq!(o.men, men(300, 600));
}

#[test]
fn scenario_82_morning_3_the_need_to_know() {
    // The same count, 2 days old; not met; air group 1 looking since day 3.
    let o = game("82").order(3, 1);
    assert_eq!(
        o.need_to_know,
        need(
            seen_150(1, 2),
            Some(Looking {
                air_group: AirGroupId::new(1),
                since_day: Some(3),
            }),
            false
        )
    );
}

#[test]
fn scenario_82_morning_4() {
    let o = game("82").order(4, 1);
    assert_head(o, PlanState::Preparing);
    assert_eq!(o.phases, take_phases(UnderWay, ToCome, ToCome));
    assert_eq!(o.waits_for, vec![met(WaitedFor::Knowledge(ASKED))]);
    assert_eq!(o.need_to_know, need(seen_150(3, 1), None, true));
    assert_eq!(o.men, men(300, 600));
}

#[test]
fn scenario_82_morning_5() {
    let o = game("82").order(5, 1);
    assert_head(o, PlanState::Released);
    assert_eq!(o.phases, take_phases(Done, UnderWay, ToCome));
    assert!(o.waits_for.is_empty(), "{:?}", o.waits_for);
    // Not asked: the plan is released.
    assert_eq!(o.need_to_know, None);
    assert_eq!(o.men, men(300, 600));
}

// ---------------------------------------------------------------- Scenario 79

#[test]
fn scenario_79_a_hold_that_waits() {
    let o = game("79").order(3, 1);
    assert_eq!(o.intent, IntentId::new(1));
    assert_eq!(
        o.phases,
        vec![
            Phase {
                name: PhaseName::Wait,
                ends: PhaseEnd::BaseIsTheSides(BaseId::new(2)),
                stage: UnderWay,
            },
            Phase {
                name: PhaseName::Hold,
                ends: PhaseEnd::Never,
                stage: ToCome,
            },
        ]
    );
    // It waits for nothing the staff can give a day to.
    assert!(o.waits_for.is_empty(), "{:?}", o.waits_for);
    assert_eq!(o.tasks.len(), 1, "{:?}", o.tasks);
    assert_eq!(o.tasks[0].intent, IntentId::new(2));
    assert!(
        matches!(
            o.tasks[0].kind,
            IntentKind::KeepSupplied { base, .. } if base == BaseId::new(3)
        ),
        "{:?}",
        o.tasks[0]
    );
    assert_eq!(o.men, None);
    assert_eq!(o.expected_ashore, None);
}

// ---------------------------------------------------------------- Properties

#[test]
fn nothing_more_is_told() {
    // An enemy unit far from B, unseen and untold: two worlds that differ
    // only in its size give the same orders.
    let name = "82_how_much_must_be_known";
    let with = |men: u32| {
        common::replace_once(
            &text_of(name),
            "# The Allied commander's commands",
            &format!(
                "[[land_unit]]\nid = 9\nname = \"Placeholder enemy\"\nside = \"japanese\"\nmen = {men}\nq = 0\nr = 6\n\n# The Allied commander's commands"
            ),
        )
    };
    let a = play(name, &with(100), true);
    let b = play(name, &with(300), true);
    assert_eq!(a.orders, b.orders);
    assert_eq!(a.answers, b.answers);
    assert_eq!(a.orders, game("82").orders);
}

#[test]
fn nothing_is_stored() {
    for (name, key) in [
        ("80_a_take_that_waits", "80"),
        ("82_how_much_must_be_known", "82"),
    ] {
        let never = play(name, &text_of(name), false);
        assert_eq!(never.hashes, game(key).hashes, "{name}");
    }
}

#[test]
fn a_seats_tool_gives_it() {
    let refusal =
        |n: u32| format!("intent:{n} is not an intent in force (command_state lists the intents)");
    for (key, g) in games() {
        for (day, orders) in &g.orders {
            for o in orders {
                let a = &g.answers[day][&o.intent.raw()];
                assert_eq!(a["kind"], "plan_order", "{key} {day}");
                assert_eq!(a["intent"], o.intent.raw(), "{key} {day}");
                assert_eq!(a["plan"]["state"], o.plan_state.name(), "{key} {day}");
                assert_eq!(a["expected_ashore"], serde_json::json!(o.expected_ashore));
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
                assert_eq!(stages, want, "{key} {day}");
                assert_eq!(
                    a["waits_for"].as_array().expect("waits").len(),
                    o.waits_for.len()
                );
                assert_eq!(a["men"].is_null(), o.men.is_none());
            }
            for (side, number, words) in &g.refusals[day] {
                assert_eq!(*words, refusal(*number), "{key} {day} {side:?}");
            }
        }
    }
    // A staff intent is among those refused: intent 2 of scenario 80.
    assert!(game("80").refusals[&5]
        .iter()
        .any(|(side, n, _)| *side == Side::Allied && *n == 2));
    // Every order answered is of an intent of the commander's or of a take
    // the staff formed: no other intent of the staff's has one.
    for (key, g) in games() {
        for (day, orders) in &g.orders {
            for o in orders {
                assert!(
                    g.commanders[day].contains(&o.intent.raw())
                        || g.staff_takes[day].contains(&o.intent.raw()),
                    "{key} {day}: intent {}",
                    o.intent.raw()
                );
            }
        }
    }
}

#[test]
fn no_line_for_a_task_given_no_order() {
    for (key, g) in games() {
        for (day, answers) in &g.answers {
            for a in answers.values() {
                let all = a.to_string().to_lowercase();
                assert!(!all.contains("no order"), "{key} {day}: {all}");
            }
        }
    }
    // The order names no reason: its tasks are the staff's intents in
    // force, each by number, kind and phase, and nothing else.
    for g in games().values() {
        for orders in g.orders.values() {
            for o in orders {
                for t in &o.tasks {
                    assert_ne!(t.intent, o.intent);
                }
            }
        }
    }
}
