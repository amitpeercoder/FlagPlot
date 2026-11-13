//! A side's own units are numbered by the side (decision 0086, part 1).
//!
//! 1. A side is shown its own ships, task forces and air groups numbered 1,
//!    2, 3... among its own units of that kind, in the order of the umpire's
//!    ids, and nothing in any answer shows how many units the enemy has: in
//!    scenarios 5 and 7, at the start and at the end of a run (when units
//!    were lost and one was made in play), the handles of each kind that a
//!    side is served are exactly 1 to n, and n + 1 is not known.
//! 2. By difference, at the level of what is served: a world that differs
//!    only by an Allied air group nobody sees gives the Japanese player the
//!    same briefing, list of forces and own air group entities, byte for
//!    byte, on every day. In the umpire's numbers the Japanese group made in
//!    play is group 6 in one world and group 901 in the other; the player is
//!    told group 4 in both. (The core's own test of this, which reads the
//!    raw ids, is `an_enemy_air_group_that_is_merely_there_changes_nothing_the_side_is_given`
//!    in `tests/scenario_09a_side_events.rs`; it is ignored until part 2.)

mod support;

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;
use sim_agent::{briefing_for_game, get_entity, list_bases, list_forces};
use sim_core::state::Side;
use sim_core::view::SideView;
use support::check::handles_in;
use support::{load, load_text, play, scenario_text};

const SIDES: [Side; 2] = [Side::Allied, Side::Japanese];

fn json<T: serde::Serialize>(value: &T) -> Value {
    serde_json::to_value(value).expect("an answer serialises")
}

/// The numbers of each kind of unit among a value's handles.
fn numbers_by_kind(value: &Value, out: &mut BTreeMap<String, BTreeSet<u32>>) {
    let mut handles = BTreeSet::new();
    handles_in(value, &mut handles);
    for handle in handles {
        let (kind, number) = handle.split_once(':').expect("a handle is kind:number");
        if matches!(kind, "ship" | "task_force" | "air_group") {
            out.entry(kind.to_string())
                .or_default()
                .insert(number.parse().expect("a number"));
        }
    }
}

/// Everything a side is asked at one moment: the briefing, the lists, and
/// each own unit by the numbers 1 to n (and n + 1, which is nobody's).
fn check_side(view: &SideView<'_>, title: &str, label: &str) {
    let mut served: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
    numbers_by_kind(
        &json(&briefing_for_game(view, title, Some(20))),
        &mut served,
    );
    numbers_by_kind(&json(&list_forces(view, None)), &mut served);
    numbers_by_kind(&json(&list_bases(view, None)), &mut served);
    let counts = [
        ("ship", view.ships().count()),
        ("task_force", view.task_forces().count()),
        ("air_group", view.air_groups().count()),
    ];
    for (kind, n) in counts {
        // Own units, in the order of the umpire's ids.
        let mut names: Vec<(u32, String)> = match kind {
            "ship" => view.ships().map(|u| (u.id.raw(), u.name.clone())).collect(),
            "task_force" => view
                .task_forces()
                .map(|u| (u.id.raw(), u.name.clone()))
                .collect(),
            _ => view
                .air_groups()
                .map(|u| (u.id.raw(), u.name.clone()))
                .collect(),
        };
        names.sort();
        for (i, (_, name)) in names.iter().enumerate() {
            let handle = format!("{kind}:{}", i + 1);
            let answer = json(&get_entity(view, &handle));
            assert_eq!(answer["kind"], "entity", "{label} {handle}");
            numbers_by_kind(&answer, &mut served);
            let own = &answer["entity"][kind];
            let own = if own.is_null() {
                // A task force or an air group is shown in its summary.
                &answer["entity"]["summary"][kind]
            } else {
                own
            };
            assert_eq!(own["handle"], handle, "{label}");
            assert_eq!(
                own["name"],
                name.as_str(),
                "{label} {handle} is the unit {i}"
            );
        }
        let past = json(&get_entity(view, &format!("{kind}:{}", n + 1)));
        assert_eq!(past["kind"], "not_known", "{label} {kind}:{}", n + 1);
        let zero = json(&get_entity(view, &format!("{kind}:0")));
        assert_eq!(zero["kind"], "not_known", "{label} {kind}:0");
        // Every number served is one of the side's own: 1 to n, and with
        // every unit asked for, each of them is served.
        let shown = served.get(kind).cloned().unwrap_or_default();
        let wanted: BTreeSet<u32> = (1..=u32::try_from(n).expect("few units")).collect();
        assert_eq!(shown, wanted, "{label}: the {kind} handles served");
    }
}

#[test]
fn each_side_is_served_its_own_units_numbered_from_1_in_scenarios_5_and_7() {
    for name in ["05_eastern_solomons.toml", "07_full_slice.toml"] {
        let mut scenario = load(name, 1942);
        let title = scenario.name.clone();
        // The start.
        for side in SIDES {
            let view = scenario.world.side_view(side);
            check_side(&view, &title, &format!("{name} {side:?} at the start"));
        }
        // After a run: ships sunk and groups lost are still numbered, and
        // the groups made in play come last.
        let groups_at_start: Vec<usize> = SIDES
            .iter()
            .map(|s| scenario.world.side_view(*s).air_groups().count())
            .collect();
        scenario.run_to_end().expect("the run resolves");
        let mut made = 0;
        for (i, side) in SIDES.into_iter().enumerate() {
            let view = scenario.world.side_view(side);
            made += view.air_groups().count() - groups_at_start[i];
            check_side(&view, &title, &format!("{name} {side:?} at the end"));
        }
        if name == "05_eastern_solomons.toml" {
            assert!(made > 0, "a group was made in play, so it was numbered");
        }
    }
}

/// What the Japanese player is served, as text, on one day.
fn served_to_japan(view: &SideView<'_>, title: &str) -> Vec<(String, String)> {
    let mut out = vec![
        (
            "briefing".to_string(),
            serde_json::to_string(&briefing_for_game(view, title, Some(3))).unwrap(),
        ),
        (
            "list_forces".to_string(),
            serde_json::to_string(&list_forces(view, None)).unwrap(),
        ),
    ];
    for number in 1..=view.air_groups().count() {
        let handle = format!("air_group:{number}");
        out.push((
            handle.clone(),
            serde_json::to_string(&get_entity(view, &handle)).unwrap(),
        ));
    }
    out
}

#[test]
fn an_allied_air_group_nobody_sees_changes_nothing_the_japanese_player_is_served() {
    // Scenario 12c with Japanese carrier B's flight deck degraded: its scouts
    // come back to a deck they cannot land on and land aboard carrier B2,
    // which holds no scouts, so a new group is formed for them. The second
    // world adds an empty, resting Allied group (id 900) aboard the Allied
    // carrier, which flies nothing and is never reported.
    let name = "12c_no_deck.toml";
    let carrier_b = "name = \"Placeholder carrier B\"\nclass = 4\nside = \"japanese\"\nfuel_t = 3000\ntask_force = 2\n";
    let original = scenario_text(name);
    assert!(original.contains(carrier_b), "the scenario is as expected");
    let text = original.replace(
        carrier_b,
        &format!("{carrier_b}flight_deck = \"degraded\"\n"),
    );
    let with_group = format!(
        "{text}\n[[air_group]]\nid = 900\nname = \"Placeholder unseen empty Allied group\"\nside = \"allied\"\nship = 1\naircraft_type = 4\nready = 0\npilots = 0\n"
    );
    let mut served: Vec<Vec<Vec<(String, String)>>> = Vec::new();
    let mut japanese_groups = Vec::new();
    for text in [&text, &with_group] {
        let mut scenario = load_text(text, name, 1942);
        let title = scenario.name.clone();
        let mut days = Vec::new();
        play(&mut scenario, |s| {
            days.push(served_to_japan(&s.world.side_view(Side::Japanese), &title));
        });
        japanese_groups.push(
            scenario
                .world
                .side_view(Side::Japanese)
                .air_groups()
                .count(),
        );
        served.push(days);
    }
    // The control formed a group: the Japanese side began with three.
    assert!(
        japanese_groups[0] > 3,
        "no Japanese group was formed in the control, so nothing was compared"
    );
    assert_eq!(japanese_groups[0], japanese_groups[1]);
    assert_eq!(served[0].len(), served[1].len());
    let mut compared = 0;
    for (day, (one, two)) in served[0].iter().zip(&served[1]).enumerate() {
        assert_eq!(one.len(), two.len(), "day {day}");
        for ((key, a), (_, b)) in one.iter().zip(two) {
            assert_eq!(
                a, b,
                "day {day}: {key} differs with an Allied group nobody sees"
            );
            compared += 1;
        }
    }
    assert!(compared > 10);
}

/// Every unit token of a text, as (kind, number).
fn unit_tokens(text: &str) -> Vec<(String, u32)> {
    let mut found = Vec::new();
    for kind in ["ship", "task_force", "air_group"] {
        let mut rest = text;
        while let Some(at) = rest.find(&format!("{kind}:")) {
            let before_ok = rest[..at]
                .chars()
                .last()
                .is_none_or(|c| !(c.is_ascii_alphanumeric() || c == '_'));
            let after = &rest[at + kind.len() + 1..];
            let digits: String = after.chars().take_while(char::is_ascii_digit).collect();
            if before_ok && !digits.is_empty() {
                found.push((kind.to_string(), digits.parse().expect("a number")));
            }
            rest = after;
        }
    }
    found
}

#[test]
fn the_text_of_an_event_names_a_sides_own_units_by_the_sides_numbers() {
    use sim_agent::Explainer;
    let mut checked = 0;
    for name in ["05_eastern_solomons.toml", "07_full_slice.toml"] {
        let mut scenario = load(name, 1942);
        scenario.run_to_end().expect("the run resolves");
        for side in SIDES {
            let view = scenario.world.side_view(side);
            let own = |kind: &str| match kind {
                "ship" => view.ships().count(),
                "task_force" => view.task_forces().count(),
                _ => view.air_groups().count(),
            };
            let explainer = Explainer::new(&view);
            for number in explainer.event_numbers() {
                let answer = json(&explainer.explain(number));
                for node in answer["nodes"].as_array().expect("nodes") {
                    let Some(text) = node["text"].as_str() else {
                        continue;
                    };
                    for (kind, n) in unit_tokens(text) {
                        assert!(
                            n >= 1 && (n as usize) <= own(&kind),
                            "{name} {side:?} event {number}: {kind}:{n} in {text:?}"
                        );
                        checked += 1;
                    }
                }
            }
        }
    }
    assert!(
        checked > 100,
        "{checked} unit tokens were found in event text"
    );
}

#[test]
fn a_detach_names_ships_by_the_sides_numbers_and_a_refusal_shows_the_number_written() {
    use serde_json::json;
    use support::play::session_of;
    let scenario = load("07_full_slice.toml", 1942);
    let view = scenario.world.side_view(Side::Allied);
    // A force with two ships afloat, one of them to detach, and a ship of
    // another force.
    let force = view
        .task_forces()
        .find(|f| f.ships.len() >= 2)
        .expect("a force of two ships");
    let force_unit = support::force_handle(&view, force.id);
    let inside = support::ship_handle(&view, *force.ships.iter().next().expect("a ship"));
    let outside = view
        .ships()
        .find(|s| !force.ships.contains(&s.id))
        .map(|s| support::ship_handle(&view, s.id))
        .expect("a ship of another force");
    let number = |handle: &str| -> u32 { handle[5..].parse().expect("a ship number") };
    let (mut session, _) = session_of(scenario, &[Side::Allied]);
    let mut detach = |ship: u32| {
        let answer = session
            .call(
                Side::Allied,
                "issue_orders",
                &json!({"orders": [{"unit": force_unit, "order": {"kind": "detach", "ships": [ship]}}], "clear": true}),
            )
            .expect("answered");
        answer["results"][0].clone()
    };
    assert_eq!(detach(number(&inside))["status"], "queued");
    for ship in [number(&outside), 9999] {
        let result = detach(ship);
        assert_eq!(result["status"], "refused");
        let reason = result["reason"].as_str().expect("a reason");
        assert!(
            reason.starts_with(&format!("ship:{ship} is not afloat in the task force")),
            "{reason}"
        );
    }
    // A number beyond the side's own is refused as any other ship not in the
    // force: whoever's ship it would be, the answer is one.
    let beyond = number(&outside).max(number(&inside)) + 100;
    let a = detach(beyond);
    let b = detach(9999);
    assert_eq!(
        a["reason"]
            .as_str()
            .unwrap()
            .replace(&beyond.to_string(), "N"),
        b["reason"].as_str().unwrap().replace("9999", "N")
    );
}
