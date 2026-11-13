//! A ship's experience in what a side is told (decision 0064, 12).
//!
//! Through `get_entity` (a ship, and the force that holds it) a side sees the
//! experience of its own ships when it is not the standard 2, and never of an
//! enemy ship: asked about one, it is told "not known", the one answer given
//! for everything it does not know. No list or briefing carries the word.
//!
//! Scenario 4 (Savo Island) has the values: ship 10 (Chicago) 3, ship 13
//! (Vincennes) 1, ship 15 (Astoria) 2 on the Allied side; ship 1 (Chokai) 3 on
//! the Japanese. Those are the scenario's numbers; a side is shown its own
//! units numbered from 1 (decision 0086), so the Allied ships 9 to 17 are
//! `ship:1` to `ship:9` and Chicago is `ship:2`, Vincennes `ship:5`, Astoria
//! `ship:7`; the Allied task force 3 is `task_force:2`.

mod support;

use serde_json::Value;
use sim_agent::{briefing, get_entity, list_bases, list_contacts, list_forces};
use sim_core::state::Side;
use support::load;

fn entity(scenario: &sim_data::Scenario, side: Side, handle: &str) -> Value {
    let view = scenario.world.side_view(side);
    serde_json::to_value(get_entity(&view, handle)).expect("serialises")
}

#[test]
fn a_side_sees_its_own_ships_experience_when_it_is_not_2_and_never_an_enemys() {
    let scenario = load("04_savo_island.toml", 1942);

    // Own ships: 3 and 1 are shown, the standard 2 is left out.
    assert_eq!(
        entity(&scenario, Side::Allied, "ship:2")["entity"]["experience"],
        3
    );
    assert_eq!(
        entity(&scenario, Side::Allied, "ship:5")["entity"]["experience"],
        1
    );
    let astoria = entity(&scenario, Side::Allied, "ship:7");
    assert_eq!(astoria["entity"]["ship"]["handle"], "ship:7");
    assert!(astoria["entity"].get("experience").is_none());
    assert_eq!(
        entity(&scenario, Side::Japanese, "ship:1")["entity"]["experience"],
        3
    );

    // The force's ships: the same rule, line by line. The Allied task force
    // 3 of the scenario, `task_force:2`, holds Vincennes (1), Quincy (1),
    // Astoria (2), Helm (3) and Wilson (2).
    let force = entity(&scenario, Side::Allied, "task_force:2");
    let lines = force["entity"]["ships"].as_array().expect("ship lines");
    let experience_of = |handle: &str| {
        lines
            .iter()
            .find(|line| line["ship"]["handle"] == handle)
            .unwrap_or_else(|| panic!("{handle} is in the force"))
            .get("experience")
            .cloned()
    };
    assert_eq!(experience_of("ship:5"), Some(Value::from(1)));
    assert_eq!(experience_of("ship:6"), Some(Value::from(1)));
    assert_eq!(experience_of("ship:8"), Some(Value::from(3)));
    assert_eq!(experience_of("ship:7"), None);
    assert_eq!(experience_of("ship:9"), None);

    // A side has no handle for an enemy ship: a number above the side's own
    // count (the Allied side has 9 ships, the Japanese 8; decision 0086) is
    // not known, the same document as for a ship that does not exist, with
    // nothing of the question.
    let absent = entity(&scenario, Side::Allied, "ship:999");
    for (side, handle) in [
        (Side::Allied, "ship:10"),
        (Side::Allied, "ship:13"),
        (Side::Japanese, "ship:9"),
        (Side::Japanese, "ship:13"),
    ] {
        let answer = entity(&scenario, side, handle);
        assert!(answer.get("entity").is_none(), "{handle}");
        assert!(!answer.to_string().contains("experience"), "{handle}");
        if side == Side::Allied {
            assert_eq!(answer, absent, "{handle}");
        }
    }
}

#[test]
fn no_list_or_briefing_carries_experience() {
    let scenario = load("04_savo_island.toml", 1942);
    for side in [Side::Allied, Side::Japanese] {
        let view = scenario.world.side_view(side);
        let texts = [
            serde_json::to_string(&briefing(&view, &scenario.name)).expect("serialises"),
            serde_json::to_string(&list_forces(&view, None)).expect("serialises"),
            serde_json::to_string(&list_bases(&view, None)).expect("serialises"),
            serde_json::to_string(&list_contacts(&view, None)).expect("serialises"),
        ];
        for text in texts {
            assert!(!text.contains("experience"), "{side:?}");
        }
    }
}
