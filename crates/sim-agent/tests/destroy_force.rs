//! The order to destroy a force, as a seat meets it through the tools
//! (decision 0220, R11 and R12, the page's check): `set_intent` with the kind
//! `destroy_force`, its refusals, the paper `command_state` gives, the words
//! of the three notices of following, and the three stops of `run` and
//! `advance`.
//!
//! The staff of these tests is a small one of their own: it hands in, on the
//! days its script names, the notices the doctrine's staff will hand in, so
//! that what the tools say of them is tested apart from the staff (the
//! doctrine's own part is tested in `tests/scenario_133_a_force_followed.rs`).
//! The game is scenario 133's game A, whose Allied side starts with contact 1,
//! two cruisers at (8,0), age 0, and whose script command is taken out: the
//! order is given through the tool. Nothing here asks a number of the staff
//! of the game: the script is the test's, and so is every number it checks.
//!
//! The script, by the coming day it is handed in on:
//!
//! | Day | Notice |
//! | --- | --- |
//! | 2 | follows contact 1, the same contact: 2 cruisers at (8,0), the morning of day 1 |
//! | 3 | follows contact 2 for 1, the same sort: 2 cruisers at (14,0), the morning of day 2 |
//! | 4 | follows contact 3 for 2, the other sort: 1 carrier at (14,0), the morning of day 3 |
//! | 5 | lost: the force last reported at (14,0), the morning of day 3 |
//! | 6 | ended: not found, the same report |

use crate::support;

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::{json, Value};
use sim_agent::{CallError, Seat, Session};
use sim_core::command::{
    Evidence, FollowNotice, NoticeBasis, NoticeKind, PlanId, Reason, Serves, Staff, StaffDecisions,
    StaffNotice,
};
use sim_core::hex::Hex;
use sim_core::ids::IntentId;
use sim_core::refdata::ShipKind;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_core::view::SideView;
use sim_data::Scenario;
use support::schemas;

fn notice(kind: NoticeKind, follow: FollowNotice) -> (NoticeKind, FollowNotice) {
    (kind, follow)
}

#[allow(clippy::too_many_arguments)]
fn follow(
    contact: u32,
    taken_from: Option<u32>,
    other_sort: bool,
    hex: (i32, i32),
    day: u32,
    pulse: Pulse,
    ship_count: u32,
    kind: ShipKind,
) -> FollowNotice {
    FollowNotice {
        intent: IntentId::new(1),
        contact,
        taken_from,
        other_sort,
        hex: Hex::new(hex.0, hex.1),
        day,
        pulse,
        ship_count,
        kind,
        carrier: kind == ShipKind::Carrier,
    }
}

/// The test's staff: hands in the script's notices on the coming day they
/// are named for, while the order (intent 1) is in force.
struct Scripted(BTreeMap<u32, Vec<(NoticeKind, FollowNotice)>>);

impl Scripted {
    fn game_a() -> Scripted {
        let mut script = BTreeMap::new();
        script.insert(
            2,
            vec![notice(
                NoticeKind::Follows,
                follow(
                    1,
                    None,
                    false,
                    (8, 0),
                    1,
                    Pulse::Morning,
                    2,
                    ShipKind::Cruiser,
                ),
            )],
        );
        script.insert(
            3,
            vec![notice(
                NoticeKind::Follows,
                follow(
                    2,
                    Some(1),
                    false,
                    (14, 0),
                    2,
                    Pulse::Morning,
                    2,
                    ShipKind::Cruiser,
                ),
            )],
        );
        script.insert(
            4,
            vec![notice(
                NoticeKind::Follows,
                follow(
                    3,
                    Some(2),
                    true,
                    (14, 0),
                    3,
                    Pulse::Morning,
                    1,
                    ShipKind::Carrier,
                ),
            )],
        );
        script.insert(
            5,
            vec![notice(
                NoticeKind::ContactLost,
                follow(
                    3,
                    None,
                    false,
                    (14, 0),
                    3,
                    Pulse::Morning,
                    1,
                    ShipKind::Carrier,
                ),
            )],
        );
        script.insert(
            6,
            vec![notice(
                NoticeKind::ForceNotFound,
                follow(
                    3,
                    None,
                    false,
                    (14, 0),
                    3,
                    Pulse::Morning,
                    1,
                    ShipKind::Carrier,
                ),
            )],
        );
        Scripted(script)
    }
}

impl Staff for Scripted {
    fn decide(&self, view: SideView<'_>) -> StaffDecisions {
        let coming = view.day() + 1;
        let Some(list) = self.0.get(&coming) else {
            return StaffDecisions::default();
        };
        if !view
            .command_state()
            .intents_in_force()
            .any(|intent| intent.id == IntentId::new(1))
        {
            return StaffDecisions::default();
        }
        StaffDecisions {
            no_orders: vec![Reason {
                serves: Serves {
                    intent: Some(IntentId::new(1)),
                    plan: Some(PlanId::new(1)),
                    amendment: None,
                    statement: String::new(),
                    chain: Vec::new(),
                    level: None,
                },
                evidence: vec![Evidence {
                    what: "a report".to_string(),
                    day: coming - 1,
                }],
                binding: Vec::new(),
                worked: "the test's script".to_string(),
                would_change: Vec::new(),
                answer_of: None,
                cautions: Vec::new(),
                limits: Vec::new(),
            }],
            notices: list
                .iter()
                .map(|(kind, follow)| StaffNotice {
                    kind: *kind,
                    plan: PlanId::new(1),
                    run: None,
                    rests_on: NoticeBasis::NoOrder(0),
                    pursuit: None,
                    follow: Some(*follow),
                })
                .collect(),
            ..StaffDecisions::default()
        }
    }
}

/// A staff that does nothing.
struct Quiet;

impl Staff for Quiet {
    fn decide(&self, _view: SideView<'_>) -> StaffDecisions {
        StaffDecisions::default()
    }
}

fn game_a() -> Scenario {
    let path = Path::new(support::ROOT).join("data/scenarios/a-force-followed/133_a_followed.toml");
    let text = std::fs::read_to_string(&path).expect("game A reads");
    let mut scenario = Scenario::from_text_with(&text, &path, &support::frozen())
        .expect("game A loads")
        .with_seed(1942)
        .expect("a seed can be set");
    // The order is given through the tool, not by the script.
    scenario.replace_commands(BTreeMap::new());
    scenario
}

fn session(inside_day: bool) -> Session {
    let scenario = game_a();
    let title = scenario.name.clone();
    let mut staffs: BTreeMap<Side, Box<dyn Staff + Send>> = BTreeMap::new();
    staffs.insert(Side::Allied, Box::new(Scripted::game_a()));
    staffs.insert(Side::Japanese, Box::new(Quiet));
    let session = Session::new(
        scenario.world,
        &title,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
    .with_staffs(staffs);
    if inside_day {
        session.with_orders_inside_a_day(true, Default::default())
    } else {
        session
    }
}

struct Player {
    session: Session,
    schemas: support::validator::Schemas,
}

impl Player {
    fn new(inside_day: bool) -> Player {
        Player {
            session: session(inside_day),
            schemas: schemas(),
        }
    }

    fn try_call(&mut self, tool: &str, args: Value) -> Result<Value, CallError> {
        let result = self.session.call(Side::Allied, tool, &args);
        let answer = match &result {
            Ok(answer) => answer.clone(),
            Err(error) => error.to_json(),
        };
        let kind = answer["kind"].as_str().expect("an answer has a kind");
        let errors = self.schemas.check(&format!("{kind}.schema.json"), &answer);
        assert!(errors.is_empty(), "{tool} {args}: {errors:?}\n{answer}");
        result
    }

    fn call(&mut self, tool: &str, args: Value) -> Value {
        self.try_call(tool, args)
            .unwrap_or_else(|e| panic!("{tool}: {e}"))
    }

    /// The order, given with contact 1 and 2 hexes round.
    fn give_the_order(&mut self) -> Value {
        let a = self.call(
            "set_intent",
            json!({"intent": 1, "intent_kind": "destroy_force", "contact": 1, "distance": 2}),
        );
        assert_eq!(a["status"], "queued", "{a}");
        a
    }

    /// Command state's paper of intent 1.
    fn paper(&mut self) -> Value {
        let state = self.call("command_state", json!({}));
        let intent = state["intents"]
            .as_array()
            .expect("intents")
            .iter()
            .find(|intent| intent["number"] == 1)
            .unwrap_or_else(|| panic!("intent 1 in {state}"))
            .clone();
        intent
    }
}

fn last_day(a: &Value) -> u64 {
    a["days_resolved"]["last"].as_u64().expect("a last day")
}

// -------------------------------------------------------------------------
// Giving it, and what refuses it.
// -------------------------------------------------------------------------

#[test]
fn a_contact_the_picture_does_not_hold_is_refused_at_dusk_and_none_is_named() {
    let mut p = Player::new(false);
    let a = p.call(
        "set_intent",
        json!({"intent_kind": "destroy_force", "contact": 9, "distance": 2}),
    );
    assert_eq!(a["status"], "refused", "{a}");
    let reason = a["reason"].as_str().expect("a reason");
    assert!(reason.contains("not held"), "{reason}");
    // None named: the contact is not held.
    let b = p.call(
        "set_intent",
        json!({"intent_kind": "destroy_force", "distance": 2}),
    );
    assert_eq!(b["status"], "refused", "{b}");
    assert!(
        b["reason"]
            .as_str()
            .expect("a reason")
            .contains("the contact is not held"),
        "{b}"
    );
    // Contact 0 is no contact.
    let c = p.call(
        "set_intent",
        json!({"intent_kind": "destroy_force", "contact": 0, "distance": 2}),
    );
    assert_eq!(c["status"], "refused", "{c}");
    // Nothing was queued by any.
    assert_eq!(c["queued"], 0);
}

#[test]
fn the_keys_of_the_order_are_a_contact_a_distance_and_search_only_and_no_hex() {
    let mut p = Player::new(false);
    for args in [
        json!({"intent_kind": "destroy_force", "contact": 1, "distance": 2, "hex": [8, 0]}),
        json!({"intent_kind": "destroy_force", "contact": 1, "distance": 2, "base": 1}),
        json!({"intent_kind": "deny", "contact": 1, "hex": [8, 0], "distance": 2}),
        json!({"intent_kind": "deny", "search_only": true, "hex": [8, 0], "distance": 2}),
        json!({"contact": 1, "base": 1}),
    ] {
        let error = p
            .try_call("set_intent", args.clone())
            .expect_err(&format!("{args} is refused as a call"));
        assert!(
            error.message.contains("is not an argument")
                || error.message.contains("only for intent_kind"),
            "{error}"
        );
    }
    // A distance is required, and whole.
    assert!(p
        .try_call(
            "set_intent",
            json!({"intent_kind": "destroy_force", "contact": 1})
        )
        .is_err());
    // Search only left out is false; given true it is kept.
    let a = p.call(
        "set_intent",
        json!({"intent": 1, "intent_kind": "destroy_force", "contact": 1, "distance": 0, "search_only": true}),
    );
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["command"]["search_only"], true);
    assert_eq!(a["command"]["distance"], 0);
    assert_eq!(a["command"]["contact"], 1);
    let b = p.call(
        "set_intent",
        json!({"intent": 2, "intent_kind": "destroy_force", "contact": 1, "distance": 4}),
    );
    assert_eq!(b["status"], "queued", "{b}");
    assert_eq!(b["command"]["search_only"], false);
}

#[test]
fn it_is_given_by_the_contacts_number_and_said_by_its_report_never_by_a_number() {
    let mut p = Player::new(false);
    let a = p.give_the_order();
    let text = a["text"].as_str().expect("words");
    assert!(text.contains("2 cruisers"), "{text}");
    assert!(text.contains("(8,0)"), "{text}");
    assert!(!text.contains("contact"), "{text}");
    assert!(!text.contains("intent"), "{text}");
}

#[test]
fn inside_a_day_it_is_checked_at_once_and_queued_for_the_dusk() {
    let mut p = Player::new(true);
    // From dusk to 0600 of day 1.
    let a = p.call("advance", json!({"pulses": 1}));
    assert_eq!(a["clock"]["at"], "0600", "{a}");
    let refused = p.call(
        "set_intent",
        json!({"intent_kind": "destroy_force", "contact": 7, "distance": 2}),
    );
    assert_eq!(refused["status"], "refused", "{refused}");
    assert!(refused["reason"].as_str().unwrap().contains("not held"));
    let none = p.call(
        "set_intent",
        json!({"intent_kind": "destroy_force", "distance": 2}),
    );
    assert_eq!(none["status"], "refused", "{none}");
    let queued = p.give_the_order();
    assert_eq!(queued["for_day"], 2, "{queued}");
}

#[test]
fn the_other_side_cannot_name_a_contact_of_the_first() {
    let scenario = game_a();
    let title = scenario.name.clone();
    let mut staffs: BTreeMap<Side, Box<dyn Staff + Send>> = BTreeMap::new();
    staffs.insert(Side::Allied, Box::new(Quiet));
    staffs.insert(Side::Japanese, Box::new(Quiet));
    let mut session = Session::new(
        scenario.world,
        &title,
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
    .with_staffs(staffs);
    // The Japanese side holds no contact: contact 1 of the Allied picture is
    // not one of its own.
    let a = session
        .call(
            Side::Japanese,
            "set_intent",
            &json!({"intent_kind": "destroy_force", "contact": 1, "distance": 2}),
        )
        .expect("an answer");
    assert_eq!(a["status"], "refused", "{a}");
    assert!(a["reason"].as_str().unwrap().contains("not held"), "{a}");
}

// -------------------------------------------------------------------------
// The paper.
// -------------------------------------------------------------------------

#[test]
fn the_paper_follows_the_order_through_a_taking_a_loss_and_its_end() {
    let mut p = Player::new(false);
    p.give_the_order();
    // As given: the contact is held, as the picture has it.
    let intent = p.paper();
    assert_eq!(intent["kind"], "destroy_force");
    assert_eq!(intent["contact"], 1);
    assert_eq!(intent["search_only"], false);
    assert_eq!(intent["distance"], 2);
    let paper = &intent["following"];
    assert_eq!(paper["held"], true, "{paper}");
    assert_eq!(paper["hex"], json!([8, 0]));
    assert_eq!(paper["ships"], 2);
    assert_eq!(paper["taken_for"], "cruiser");
    assert_eq!(paper["carrier"], false);
    assert_eq!(paper["age_pulses"], 0);
    assert_eq!(paper["reported"], json!({"day": 0, "pulse": "afternoon"}));
    assert_eq!(paper["takings"], json!([]));
    assert_eq!(paper["lost"], false);
    assert_eq!(
        paper["ring"],
        json!({"place": paper["place"], "hex": [8, 0], "distance": 2, "dashed": false})
    );

    // Dusk after day 3: contact 2 taken for 1 on day 3, of the same sort.
    let a = p.call("run", json!({"days": 3}));
    assert_eq!(last_day(&a), 3, "{a}");
    let paper = p.paper()["following"].clone();
    assert_eq!(paper["contact"], 2, "{paper}");
    assert_eq!(
        paper["held"], false,
        "the picture holds no contact 2: {paper}"
    );
    assert_eq!(paper["hex"], json!([14, 0]));
    assert_eq!(paper["ships"], 2);
    assert_eq!(paper["taken_for"], "cruiser");
    assert_eq!(paper["reported"], json!({"day": 2, "pulse": "morning"}));
    let takings = paper["takings"].as_array().expect("takings");
    assert_eq!(takings.len(), 1, "{paper}");
    assert_eq!(takings[0]["day"], 3);
    assert_eq!(takings[0]["taken_from"], 1);
    assert_eq!(takings[0]["contact"], 2);
    assert_eq!(takings[0]["other_sort"], false);
    assert!(takings[0].get("was_carrier").is_none());
    // A report that only refreshed the contact (day 2) adds no line.

    // Dusk after day 4: the other sort, with what it was and what it is.
    p.call("run", json!({"days": 1}));
    let paper = p.paper()["following"].clone();
    assert_eq!(paper["contact"], 3);
    assert_eq!(paper["carrier"], true);
    assert_eq!(paper["taken_for"], "carrier");
    assert_eq!(paper["ships"], 1);
    let takings = paper["takings"].as_array().expect("takings");
    assert_eq!(takings.len(), 2, "{paper}");
    assert_eq!(takings[1]["day"], 4);
    assert_eq!(takings[1]["other_sort"], true);
    assert_eq!(takings[1]["carrier"], true);
    assert_eq!(takings[1]["was_carrier"], false);
    let lines: Vec<&str> = paper["text"]
        .as_array()
        .expect("text")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    assert!(
        lines
            .iter()
            .any(|line| line.contains("We now take the force for a carrier force")),
        "{lines:?}"
    );

    // Dusk after day 5: lost, with the last place and day, the ring dashed.
    p.call("run", json!({"days": 1}));
    let paper = p.paper()["following"].clone();
    assert_eq!(paper["lost"], true, "{paper}");
    assert_eq!(paper["last_day"], 3);
    assert_eq!(paper["last_place"]["q"], 14);
    assert_eq!(paper["last_place"]["r"], 0);
    assert_eq!(paper["ring"]["dashed"], true);
    assert_eq!(paper["ring"]["hex"], json!([14, 0]));
    assert_eq!(paper["ring"]["distance"], 2);
    let lines: Vec<&str> = paper["text"]
        .as_array()
        .expect("text")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    assert!(
        lines
            .iter()
            .any(|line| line.contains("We have lost the force")),
        "{lines:?}"
    );

    // Dusk after day 6: ended, and the paper has gone.
    p.call("run", json!({"days": 1}));
    let intent = p.paper();
    assert_eq!(intent["in_force"], false, "{intent}");
    assert!(intent.get("following").is_none(), "{intent}");
}

#[test]
fn no_word_of_the_paper_or_the_notices_is_a_handle_an_intent_or_a_rule() {
    let mut p = Player::new(false);
    p.give_the_order();
    p.call("run", json!({"days": 5}));
    let paper = p.paper()["following"].clone();
    let report = p.call("report", json!({}));
    let mut said: Vec<String> = paper["text"]
        .as_array()
        .expect("text")
        .iter()
        .filter_map(|line| line.as_str().map(str::to_string))
        .collect();
    let matters = report["matters"].as_array().expect("matters");
    let follows: Vec<&Value> = matters
        .iter()
        .filter(|m| matches!(m["kind"].as_str(), Some("follows" | "contact_lost")))
        .collect();
    assert_eq!(follows.len(), 2, "{report}");
    for matter in &follows {
        assert_eq!(matter["order"], 1);
        said.push(matter["words"].as_str().expect("words").to_string());
    }
    assert!(said.len() >= 4, "{said:?}");
    for line in &said {
        assert!(
            line.starts_with("We ") || line.starts_with("On day "),
            "{line}"
        );
        for banned in [
            "contact", "intent", "his ", "points", "victory", "margin", "score", "knots",
        ] {
            assert!(!line.contains(banned), "{banned} in {line}");
        }
        assert!(!line.contains("contact:"), "{line}");
    }
}

// -------------------------------------------------------------------------
// The stops.
// -------------------------------------------------------------------------

#[test]
fn a_run_stops_after_the_day_a_notice_was_handed_in_only_where_asked() {
    // Off unless asked for: the seven days are played through.
    let mut p = Player::new(false);
    p.give_the_order();
    let a = p.call("run", json!({"days": 7}));
    assert_eq!(a["stopped"], "game_over", "{a}");
    assert!(a.get("stop_on").is_none());
    assert!(a.get("told").is_none());

    // The taking of the same sort (day 3) stops nothing; the other sort
    // (day 4) stops after that day, naming the notice.
    let mut p = Player::new(false);
    p.give_the_order();
    let a = p.call("run", json!({"days": 7, "stop_on": ["follows_other_sort"]}));
    assert_eq!(a["stopped"], "told", "{a}");
    assert_eq!(last_day(&a), 4, "{a}");
    assert_eq!(a["stop_on"], json!(["follows_other_sort"]));
    assert_eq!(a["told"][0]["thing"], "follows_other_sort");
    let words = a["told"][0]["words"][0]
        .as_str()
        .expect("the notice's words");
    assert!(
        words.starts_with("We now take the force for a carrier force"),
        "{words}"
    );
    assert!(a["summary"]
        .as_str()
        .unwrap()
        .contains("follows_other_sort"));
    assert_eq!(a["fired"], json!([]), "no interrupt fires: {a}");

    // Lost stops after day 5, ended after day 6.
    let mut p = Player::new(false);
    p.give_the_order();
    let a = p.call("run", json!({"days": 7, "stop_on": ["contact_lost"]}));
    assert_eq!(a["stopped"], "told", "{a}");
    assert_eq!(last_day(&a), 5, "{a}");
    assert_eq!(a["told"][0]["thing"], "contact_lost");
    let mut p = Player::new(false);
    p.give_the_order();
    let a = p.call("run", json!({"days": 7, "stop_on": ["force_not_found"]}));
    assert_eq!(a["stopped"], "told", "{a}");
    assert_eq!(last_day(&a), 6, "{a}");
    assert_eq!(a["told"][0]["thing"], "force_not_found");

    // All three asked: the first stop is day 4, then 5, then 6.
    let mut p = Player::new(false);
    p.give_the_order();
    let all =
        json!({"days": 7, "stop_on": ["follows_other_sort", "contact_lost", "force_not_found"]});
    for day in [4, 5, 6] {
        let a = p.call("run", all.clone());
        assert_eq!(a["stopped"], "told", "{a}");
        assert_eq!(last_day(&a), day, "{a}");
    }
    // None stops before a day: the clock stood at dusk each time and the
    // call played a day.
    let a = p.call("run", all.clone());
    assert_eq!(a["stopped"], "game_over", "{a}");
    assert_eq!(last_day(&a), 7);

    // Only the three notices are things run can stop on.
    let mut p = Player::new(false);
    assert!(p
        .try_call("run", json!({"days": 1, "stop_on": ["contact"]}))
        .is_err());
    assert!(p
        .try_call("run", json!({"days": 1, "stop_on": "contact_lost"}))
        .is_err());
}

#[test]
fn advance_stops_at_the_end_of_the_first_pulse_of_the_day_the_notice_was_handed_in() {
    let mut p = Player::new(true);
    p.give_the_order();
    // Off unless asked for: three days are passed in nine pulses.
    let a = p.call("advance", json!({"pulses": 9}));
    assert_eq!(a["stopped"], "pulses", "{a}");

    // The taking of the same sort on day 3 stops nothing; the other sort
    // stops at 0600 of day 4.
    let mut p = Player::new(true);
    p.give_the_order();
    let a = p.call(
        "advance",
        json!({"pulses": 60, "stop_on": ["follows_other_sort"]}),
    );
    assert_eq!(a["stopped"], "told", "{a}");
    assert_eq!(a["clock"]["day"], 4, "{a}");
    assert_eq!(a["clock"]["at"], "0600", "{a}");
    assert_eq!(a["told"][0]["thing"], "follows_other_sort");
    assert!(a["told"][0]["words"][0]
        .as_str()
        .unwrap()
        .starts_with("We now take"));
    assert_eq!(a["fired"], json!([]));

    let mut p = Player::new(true);
    p.give_the_order();
    let a = p.call(
        "advance",
        json!({"pulses": 60, "stop_on": ["contact_lost"]}),
    );
    assert_eq!(a["clock"]["day"], 5, "{a}");
    assert_eq!(a["clock"]["at"], "0600", "{a}");
    let mut p = Player::new(true);
    p.give_the_order();
    let a = p.call(
        "advance",
        json!({"pulses": 60, "stop_on": ["force_not_found"]}),
    );
    assert_eq!(a["clock"]["day"], 6, "{a}");
    assert_eq!(a["clock"]["at"], "0600", "{a}");
}
