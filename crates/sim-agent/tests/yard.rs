//! The yard in the agent's answers (decision 0069, "The briefing and
//! `measure`", and "The order": acceptance states the day).
//!
//! A small scenario written here, on the transit map and the frozen figures
//! (yard days: 2 a point of flotation damage, 5 a system degraded, 30 a system
//! out; forward repair stops short of flotation 10 and a system takes 10
//! points). Allied base 1 is a port for the yard with a passage of 4 days;
//! base 2 mends 6 points a day and is no port for the yard.
//!
//! The hand calculation of force 1, ships 1 and 2 (the case the architect
//! gave): ship 1 has flotation damage 12.5 and one system degraded, so
//! 2 x 12.5 + 5 x 1 = 30 days; ship 2 has flotation damage 3.25 and one system
//! degraded, so 2 x 3.25 + 5 x 1 = 11.5, rounded up to 12 days. Passage 4 plus
//! the largest, 30, is 34 days away. An order for the coming day, day 1,
//! leaves on day 1 and the force is back on day 1 + 34 = 35.
//!
//! The second force is at base 2: no port for the yard, so a yard order is
//! refused. Its one ship is mended there: day 1's allowance of 6 goes 3 to
//! fire damage, 2 to flotation damage and 1 toward the degraded system.

mod support;

use serde_json::{json, Value};
use sim_agent::{briefing_for_game, measure, Question, Seat, Session};
use sim_core::state::Side;
use support::check::valid;
use support::{load_text, schemas};

const SCENARIO: &str = r#"
name = "Placeholder repair test"
placeholder = true
seed = 1942
days = 40
map = "../map/placeholder_transit.toml"
ship_classes = "../reference/ship_classes.csv"
port_sizes = "../reference/port_sizes.csv"
aircraft_types = "../reference/aircraft_types.csv"
search = "../reference/search.toml"
surface_combat = "../reference/surface_combat.toml"

[[base]]
id = 1
name = "Placeholder yard port"
side = "allied"
q = 0
r = 0
yard_passage_days = 4

[[base]]
id = 2
name = "Placeholder repair anchorage"
side = "allied"
q = 8
r = 3
ship_repair_points_per_day = 6

[[task_force]]
id = 1
name = "Placeholder yard force"
side = "allied"
at_base = 1
mission = "transport"

[[task_force]]
id = 2
name = "Placeholder repair force"
side = "allied"
at_base = 2
mission = "transport"

[[task_force]]
id = 3
name = "Placeholder enemy force"
side = "japanese"
q = 5
r = 5
mission = "transport"

[[ship]]
id = 1
name = "Placeholder cruiser 1"
class = 1
side = "allied"
fuel_t = 2000
flotation_damage = "12.5"
fire_control = "degraded"
task_force = 1

[[ship]]
id = 2
name = "Placeholder destroyer 2"
class = 2
side = "allied"
fuel_t = 500
flotation_damage = "3.25"
torpedo_mounts = "degraded"
task_force = 1

[[ship]]
id = 3
name = "Placeholder cruiser 3"
class = 1
side = "allied"
fuel_t = 2000
fire_damage = "3"
flotation_damage = "2"
fire_control = "degraded"
task_force = 2

[[ship]]
id = 4
name = "Placeholder cruiser 4"
class = 1
side = "japanese"
fuel_t = 2000
task_force = 3
"#;

fn session(days: u32) -> Session {
    let mut scenario = load_text(SCENARIO, "01_transit.toml", 1942);
    scenario.days = days;
    Session::new(
        scenario.world,
        "Placeholder repair test",
        scenario.days,
        Seat::Player,
        Seat::Script(Default::default()),
    )
}

fn measure_yard(session: &mut Session, force: &str) -> Value {
    let answer = session
        .call(
            Side::Allied,
            "measure",
            &json!({ "question": "yard", "task_force": force }),
        )
        .expect("measure is answered");
    valid(&schemas(), "measure.schema.json", &answer, "yard measure");
    answer
}

fn order(session: &mut Session, force: &str) -> Value {
    let answer = session
        .call(
            Side::Allied,
            "issue_orders",
            &json!({ "orders": [{ "unit": force, "order": { "kind": "send_to_yard" } }] }),
        )
        .expect("issue_orders is answered");
    valid(
        &schemas(),
        "issue_orders.schema.json",
        &answer,
        "yard order",
    );
    answer["results"][0].clone()
}

#[test]
fn measure_yard_gives_the_days_of_the_hand_calculation() {
    let mut session = session(40);
    let a = measure_yard(&mut session, "task_force:1");
    assert_eq!(a["kind"], "measure");
    assert_eq!(a["question"], "yard");
    assert_eq!(a["away"], false);
    assert_eq!(a["order"], "accepted");
    assert_eq!(a["refusal"], Value::Null);
    assert_eq!(a["port"]["handle"], "base:1");
    assert_eq!(a["passage_days"], 4);
    let ships = a["ships"].as_array().expect("ships");
    assert_eq!(ships.len(), 2);
    assert_eq!(ships[0]["ship"]["handle"], "ship:1");
    assert_eq!(ships[0]["flotation_damage"], "12.5");
    assert_eq!(ships[0]["systems_degraded"], 1);
    assert_eq!(ships[0]["systems_out"], 0);
    assert_eq!(ships[0]["days_of_repair"], 30);
    assert_eq!(ships[1]["ship"]["handle"], "ship:2");
    assert_eq!(ships[1]["flotation_damage"], "3.25");
    assert_eq!(ships[1]["days_of_repair"], 12);
    assert_eq!(a["days_away"], 34);
    assert_eq!(a["leaves"], 1);
    assert_eq!(a["return_day"], 35);
    assert_eq!(a["returns_before_end"], true);
    assert_eq!(a["last_day"], 40);
    // The text says the same, in words, from the data alone.
    let text = a["assumptions"].as_str().expect("assumptions");
    assert!(text.contains("no random draw"));
}

#[test]
fn measure_yard_says_it_does_not_return_when_the_game_ends_first() {
    // Back on day 35 and the game's last day is 30.
    let mut session = session(30);
    let a = measure_yard(&mut session, "task_force:1");
    assert_eq!(a["return_day"], 35);
    assert_eq!(a["returns_before_end"], false);
    assert_eq!(a["last_day"], 30);
}

#[test]
fn measure_yard_gives_the_refusal_where_there_is_no_port() {
    let mut session = session(40);
    let a = measure_yard(&mut session, "task_force:2");
    assert_eq!(a["order"], "refused");
    assert!(
        a["refusal"].as_str().expect("a reason").contains("yard"),
        "{a}"
    );
    assert_eq!(a["port"], Value::Null);
    assert_eq!(a["days_away"], Value::Null);
    assert_eq!(a["return_day"], Value::Null);
    // The same refusal as the order itself gets.
    let result = order(&mut session, "task_force:2");
    assert_eq!(result["status"], "refused");
    assert_eq!(result["reason"], a["refusal"]);
}

#[test]
fn measure_yard_is_not_known_for_a_force_that_is_not_the_sides() {
    let mut session = session(40);
    for force in ["task_force:3", "task_force:99", "ship:1"] {
        let a = session
            .call(
                Side::Allied,
                "measure",
                &json!({ "question": "yard", "task_force": force }),
            )
            .expect("answered");
        assert_eq!(a["kind"], "not_known", "{force}: {a}");
    }
}

#[test]
fn a_queued_yard_order_states_its_day_of_return() {
    let mut session = session(40);
    let queued = order(&mut session, "task_force:1");
    assert_eq!(queued["status"], "queued");
    assert_eq!(queued["day_of_return"], 35);
    assert_eq!(queued["returns"], true);

    // The same order in a game that ends on day 30 says it does not return.
    let mut short = self::session(30);
    let queued = order(&mut short, "task_force:1");
    assert_eq!(queued["day_of_return"], 35);
    assert_eq!(queued["returns"], false);

    // A refused order carries no day.
    let refused = order(&mut short, "task_force:2");
    assert_eq!(refused["status"], "refused");
    assert!(refused.get("day_of_return").is_none());
}

fn end_day(session: &mut Session) {
    session
        .call(Side::Allied, "end_day", &json!({}))
        .expect("end_day is answered");
}

fn briefing_text(session: &mut Session) -> (String, Value) {
    let text = session
        .call(Side::Allied, "get_briefing", &json!({ "text": true }))
        .expect("the briefing");
    let data = session
        .call(Side::Allied, "get_briefing", &json!({ "text": false }))
        .expect("the briefing");
    valid(&schemas(), "briefing.schema.json", &data, "yard briefing");
    (text["text"].as_str().expect("text").to_string(), data)
}

#[test]
fn the_briefing_says_who_is_away_and_when_it_is_back() {
    let mut session = session(40);
    // Before the order nothing is away.
    let (text, data) = briefing_text(&mut session);
    assert!(!text.contains("AWAY AT THE YARD"), "{text}");
    assert!(data.get("away_at_yard").is_none());

    assert_eq!(order(&mut session, "task_force:1")["status"], "queued");
    end_day(&mut session);

    let (text, data) = briefing_text(&mut session);
    assert!(text.contains("AWAY AT THE YARD"), "{text}");
    assert!(
        text.contains(
            "task_force:1 \"Placeholder yard force\" at (0,0) Placeholder yard port: away at the yard, 2 ships"
        ),
        "{text}"
    );
    assert!(text.contains("back on day 35"), "{text}");
    let away = &data["away_at_yard"][0];
    assert_eq!(away["task_force"]["handle"], "task_force:1");
    assert_eq!(away["port"]["handle"], "base:1");
    assert_eq!(away["return_day"], 35);
    assert_eq!(away["returns_before_end"], true);
    assert_eq!(away["ships"].as_array().expect("ships").len(), 2);
    let line = &data["task_forces"][0];
    assert_eq!(line["task_force"]["handle"], "task_force:1");
    assert_eq!(line["status"], "away_at_yard");
    assert_eq!(line["ships_afloat"], 2);

    // Base 2 mended ship 3 on day 1: 6 points (3 fire, 2 flotation, 1 toward
    // the system), and she still has her system degraded.
    assert!(text.contains("Ships repaired yesterday:"), "{text}");
    assert!(text.contains("still degraded: fire control"), "{text}");
    let repaired = &data["bases"][1]["repaired_yesterday"][0];
    assert_eq!(repaired["ship"]["handle"], "ship:3");
    assert_eq!(repaired["fire_damage"], "0");
    assert_eq!(repaired["flotation_damage"], "0");
    assert_eq!(repaired["systems_degraded"], json!(["fire_control"]));

    // The force away is asked about: its port and day of return.
    let a = measure_yard(&mut session, "task_force:1");
    assert_eq!(a["away"], true);
    assert_eq!(a["port"]["handle"], "base:1");
    assert_eq!(a["return_day"], 35);
    assert_eq!(a["returns_before_end"], true);
    // It takes no order until then.
    let again = order(&mut session, "task_force:1");
    assert_eq!(again["status"], "refused", "{again}");
}

#[test]
fn a_force_that_returned_yesterday_is_said_so_under_its_base() {
    let mut session = session(40);
    assert_eq!(order(&mut session, "task_force:1")["status"], "queued");
    // Days 1 to 34 are played; the force is back at the start of day 35, and
    // the briefing for day 36 reports it.
    for _ in 1..=35 {
        end_day(&mut session);
    }
    let (text, data) = briefing_text(&mut session);
    assert!(text.contains("Returned yesterday from the yard"), "{text}");
    assert!(!text.contains("AWAY AT THE YARD"), "{text}");
    let back = &data["bases"][0]["returned_yesterday"][0];
    assert_eq!(back["task_force"]["handle"], "task_force:1");
    assert!(data.get("away_at_yard").is_none());
    assert_eq!(data["task_forces"][0]["status"], "idle");
    // Back, it can be ordered again.
    let a = measure_yard(&mut session, "task_force:1");
    assert_eq!(a["away"], false);
    assert_eq!(a["order"], "accepted");
    assert_eq!(a["days_away"], 4);
}

#[test]
fn the_enemy_is_told_nothing_of_the_yard() {
    let mut session = session(40);
    assert_eq!(order(&mut session, "task_force:1")["status"], "queued");
    for _ in 1..=2 {
        end_day(&mut session);
    }
    let world = session.into_world();
    let view = world.side_view(Side::Japanese);
    let b = briefing_for_game(&view, "Placeholder repair test", Some(40));
    let text = b.to_text().to_lowercase();
    let data = serde_json::to_string(&b)
        .expect("serialises")
        .to_lowercase();
    for word in ["yard", "away", "repaired", "returned"] {
        assert!(
            !text.contains(word),
            "{word} in the enemy's briefing: {text}"
        );
    }
    assert!(!data.contains("yard"), "{data}");
    assert!(b.away_at_yard.is_empty());
    assert!(b
        .bases
        .iter()
        .all(|base| base.repaired_yesterday.is_empty()));
    // Its own force is not the Allied one, and asking of the Allied handles
    // gives the one answer of everything the side does not know.
    let mut answers = Vec::new();
    for force in ["task_force:2", "task_force:3", "task_force:99"] {
        let question = Question::Yard {
            task_force: force.to_string(),
        };
        answers.push(serde_json::to_string(&measure(&view, &question)).expect("serialises"));
    }
    assert!(answers.iter().all(|a| a == &answers[0]), "{answers:?}");
    assert!(answers[0].contains("not_known"), "{answers:?}");
    // Its own force it may ask of, and gets a refusal-free plan-less answer:
    // no port of its own for the yard, nothing of the Allied force.
    let own = serde_json::to_string(&measure(
        &view,
        &Question::Yard {
            task_force: "task_force:1".to_string(),
        },
    ))
    .expect("serialises");
    assert!(own.contains("\"question\":\"yard\""), "{own}");
    assert!(!own.contains("base:1"), "{own}");
}
