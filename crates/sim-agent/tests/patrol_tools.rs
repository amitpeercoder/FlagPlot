//! The command tools and lists of the patrol of submarines (decision 0218,
//! stage 3), over scenario 129: seven placeholder Japanese boats lie at the
//! Japanese base at (12,0). The numbers of the boats are the side's own,
//! 1 to 7, in the order of the scenario's ids.
//!
//! Hand calculation: the intent set is intent 1, a patrol of the waters
//! within 2 hexes of (5,0); naming boats 3 and 1 in a forces limit makes the
//! limit's boats [1, 3] (the side's numbers, in order); a hold on boat 2 is a
//! hold whose target is boat 2; the Allied side, told one suspected water at
//! (4,0) of 1 hex on day 1, lists that and no boat of the Japanese side.

use crate::support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use support::check::valid;
use support::{load_text, schemas};

const NAME: &str = "129_submarines_the_staff_and_the_patrol.toml";
const BOATS_COMMANDS: &str = "# The Japanese commander's commands";

fn text(extra: &str) -> String {
    let file = support::scenario_text(NAME);
    let at = file.find(BOATS_COMMANDS).expect("the commands begin");
    format!("{}{extra}", &file[..at])
}

fn session(extra: &str) -> Session {
    let scenario = load_text(&text(extra), NAME, 1942);
    Session::new(
        scenario.world,
        "Patrol",
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
    .with_staffs(sim_staff::staffs())
}

fn call(session: &mut Session, side: Side, tool: &str, args: Value) -> Value {
    let answer = session.call(side, tool, &args).expect("the tool answers");
    let kind = answer["kind"].as_str().expect("a kind").to_string();
    valid(&schemas(), &format!("{kind}.schema.json"), &answer, tool);
    answer
}

#[test]
fn a_patrol_intent_is_queued_and_shown_with_its_waters() {
    let mut s = session("");
    let a = call(
        &mut s,
        Side::Japanese,
        "set_intent",
        json!({"intent_kind": "patrol", "hex": [5, 0], "distance": 2}),
    );
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["command"]["intent_kind"], "patrol");
    assert_eq!(a["command"]["hex"], json!([5, 0]));
    assert_eq!(a["command"]["distance"], 2);
    let state = call(&mut s, Side::Japanese, "command_state", json!({}));
    let intent = &state["intents"][0];
    assert_eq!(intent["kind"], "patrol");
    assert_eq!(intent["hex"], json!([5, 0]));
    assert_eq!(intent["distance"], 2);
    // A patrol takes no base, and a hex and distance are not for a keep.
    let error = s
        .call(
            Side::Japanese,
            "set_intent",
            &json!({"intent_kind": "patrol", "hex": [5, 0], "distance": 2, "base": 1}),
        )
        .expect_err("refused");
    assert!(error.message.contains("patrol"), "{error}");
}

#[test]
fn a_forces_limit_names_boats_by_the_sides_numbers() {
    let mut s = session("");
    call(
        &mut s,
        Side::Japanese,
        "set_intent",
        json!({"intent_kind": "patrol", "hex": [5, 0], "distance": 2}),
    );
    let a = call(
        &mut s,
        Side::Japanese,
        "set_limit",
        json!({"limit": "forces", "intent": 1, "submarines": [3, 1]}),
    );
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["command"]["submarines"], json!([1, 3]), "{a}");
    let state = call(&mut s, Side::Japanese, "command_state", json!({}));
    let intent = &state["intents"][0];
    assert_eq!(intent["limits"]["forces"]["submarines"], json!([1, 3]));
    assert_eq!(
        intent["limits_read_up"]["submarines_binding"],
        json!([1, 3])
    );
    // A boat the side does not have is refused in the engine's words.
    let a = call(
        &mut s,
        Side::Japanese,
        "set_limit",
        json!({"limit": "forces", "intent": 1, "submarines": [9]}),
    );
    assert_eq!(a["status"], "refused", "{a}");
    assert!(a["reason"].as_str().unwrap().contains("submarine"), "{a}");
}

#[test]
fn a_boat_is_held_and_released_and_a_stranger_is_refused() {
    let mut s = session("");
    let a = call(&mut s, Side::Japanese, "hold", json!({"submarine": 2}));
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["command"], json!({"kind": "hold", "submarine": 2}));
    assert!(a["text"].as_str().unwrap().contains("submarine:2"), "{a}");
    let a = call(&mut s, Side::Japanese, "release", json!({"submarine": 2}));
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["command"], json!({"kind": "release", "submarine": 2}));
    // A boat that is not held cannot be released.
    let a = call(&mut s, Side::Japanese, "release", json!({"submarine": 3}));
    assert_eq!(a["status"], "refused", "{a}");
    assert!(a["reason"].as_str().unwrap().contains("submarine:3"), "{a}");
    // Not a boat of the side's: refused, in the number the side wrote.
    let a = call(&mut s, Side::Japanese, "hold", json!({"submarine": 99}));
    assert_eq!(a["status"], "refused", "{a}");
    assert_eq!(a["command"]["submarine"], 99, "{a}");
    // The Allied side has no boat at all.
    let a = call(&mut s, Side::Allied, "hold", json!({"submarine": 1}));
    assert_eq!(a["status"], "refused", "{a}");
    // One target only.
    let error = s
        .call(Side::Japanese, "hold", &json!({"submarine": 1, "ship": 1}))
        .expect_err("refused");
    assert!(error.message.contains("only one"), "{error}");
}

#[test]
fn a_boat_held_is_in_the_commanders_hands_and_the_rest_in_the_staffs() {
    let extra = "[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_intent\"\nintent = 1\nintent_kind = \"patrol\"\nhex = [5, 0]\ndistance = 2\n\n\
                 [[command]]\nday = 1\nside = \"japanese\"\nkind = \"hold\"\nsubmarine = 2\n";
    let mut scenario = load_text(&text(extra), NAME, 1942);
    scenario.next_day().expect("day 1");
    let mut s = Session::new(
        scenario.world,
        "Patrol",
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
    .with_staffs(sim_staff::staffs());
    let forces = call(&mut s, Side::Japanese, "list_forces", json!({}));
    let boats = forces["submarines"].as_array().expect("boats");
    assert_eq!(boats.len(), 7);
    for boat in boats {
        let number = boat["submarine"]["handle"].as_str().unwrap();
        let hands = &boat["hands"];
        if number == "submarine:2" {
            assert_eq!(hands["by"], "commander", "{boat}");
            assert!(hands["words"].as_str().unwrap().contains("hold command"));
        } else {
            assert_eq!(hands["by"], "staff", "{boat}");
            assert_eq!(hands["intent"], 1, "{boat}");
        }
    }
    let state = call(&mut s, Side::Japanese, "command_state", json!({}));
    assert_eq!(
        state["holds"][0]["target"],
        json!({"submarine": 2}),
        "{state}"
    );
    assert!(state["holds"][0]["text"]
        .as_str()
        .unwrap()
        .contains("submarine 2"));
}

#[test]
fn propose_and_preview_give_the_stations_beside_the_words() {
    let mut s = session("");
    call(
        &mut s,
        Side::Japanese,
        "set_intent",
        json!({"intent_kind": "patrol", "hex": [5, 0], "distance": 2}),
    );
    let proposal = call(&mut s, Side::Japanese, "propose", json!({"intent": 1}));
    let stations = proposal["stations"].as_array().expect("stations");
    assert!(!stations.is_empty(), "{proposal}");
    assert_eq!(stations[0]["submarine"], "submarine:1", "{proposal}");
    assert_eq!(stations[0]["place"], 1);
    assert!(stations[0]["hex"]["q"].is_i64());
    assert!(stations[0]["name"].as_str().is_some());
    // The places are 1, 2, 3, ... in order.
    for (i, station) in stations.iter().enumerate() {
        assert_eq!(station["place"], json!(i + 1));
    }
    // A preview of a change names the stations too, on both copies.
    let a = call(
        &mut s,
        Side::Japanese,
        "preview",
        json!({"command": "set_limit", "limit": "forces", "intent": 1, "submarines": [2, 3]}),
    );
    assert_eq!(a["valid"], true, "{a}");
    for copy in ["as_things_stand", "with_change"] {
        assert!(a[copy]["stations"].as_array().is_some(), "{copy}: {a}");
    }
    let boats = |copy: &str| -> Vec<Value> {
        a[copy]["stations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s["submarine"].clone())
            .collect()
    };
    assert_eq!(
        boats("with_change"),
        vec![json!("submarine:2"), json!("submarine:3")]
    );
    assert_eq!(boats("as_things_stand").len(), 7);
}

#[test]
fn the_waters_the_side_suspects_are_listed_and_no_enemy_boat_is() {
    let waters = "[[suspected_waters]]\nside = \"allied\"\nq = 4\nr = 0\ndistance = 1\nday = 1\n\
        words = \"Placeholder belief: enemy boats work these waters\"\n\
        rows = \"placeholder, no row\"\nreading = \"Placeholder reading, the architect's\"\n";
    let mut s = session(waters);
    let allied = call(&mut s, Side::Allied, "list_contacts", json!({}));
    assert_eq!(allied["contacts"], json!([]));
    let told = allied["suspected_waters"].as_array().expect("waters");
    assert_eq!(told.len(), 1, "{allied}");
    assert_eq!(told[0]["place"]["q"], 4);
    assert_eq!(told[0]["distance_hexes"], 1);
    assert_eq!(told[0]["told_day"], 1);
    assert!(allied.get("submarine_reports").is_none(), "{allied}");
    // An area that does not hold the centre leaves it out.
    let far = call(
        &mut s,
        Side::Allied,
        "list_contacts",
        json!({"area": {"q": 12, "r": 0, "radius_nm": 10}}),
    );
    assert!(far.get("suspected_waters").is_none(), "{far}");
    // The Japanese side was told nothing and sees no boat of its own here.
    let japanese = call(&mut s, Side::Japanese, "list_contacts", json!({}));
    assert!(japanese.get("suspected_waters").is_none(), "{japanese}");
    assert!(japanese.get("submarine_reports").is_none(), "{japanese}");
    assert_eq!(japanese["contacts"], json!([]));
}

/// Decision 0221, the staff's reading: a preview of an order not yet given
/// carries in `with_change` the proposal's tree, what it would hand in and
/// its reason, the same as `propose` gives once the order is queued; the copy
/// as things stand, with no such order, carries none of them.
#[test]
fn a_preview_of_a_draft_carries_the_proposal_as_propose_gives_it() {
    let mut s = session("");
    let draft = json!({"intent_kind": "patrol", "hex": [5, 0], "distance": 2});
    let mut asked = draft.clone();
    asked["command"] = json!("set_intent");
    let a = call(&mut s, Side::Japanese, "preview", asked);
    assert_eq!(a["valid"], true, "{a}");
    let with = &a["with_change"];
    for key in ["tree", "would_hand_in", "reason"] {
        assert!(with.get(key).is_some(), "{key}: {a}");
        assert!(a["as_things_stand"].get(key).is_none(), "{key}: {a}");
    }
    call(&mut s, Side::Japanese, "set_intent", draft);
    let proposal = call(&mut s, Side::Japanese, "propose", json!({"intent": 1}));
    assert_eq!(with["tree"], proposal["tree"], "{a}\n{proposal}");
    assert_eq!(
        with["would_hand_in"], proposal["would_hand_in"],
        "{a}\n{proposal}"
    );
    assert_eq!(with["reason"], proposal["reason"], "{a}\n{proposal}");
}
