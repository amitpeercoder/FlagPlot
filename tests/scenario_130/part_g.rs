//! Scenario 130, part G: a limit can say "none of this kind" (the
//! architect's ruling A). The tests of its statements G1 to G4 (see the doc
//! comment of the file above).
//!
//! # The game
//!
//! Statement 1's (`data/scenarios/a-limit-binds-what-it-names/130_a_limit_binds_what_it_names.toml`):
//! a deny of the waters within 2 hexes of (0,0) as intent 1; base B with a
//! bomber group (4) and a search group (5) in reach; carrier force C (task
//! force 1) at (10,0) with groups 1 to 3 aboard; and, for G1 and G2, a
//! second carrier force C2 (task force 3) built in the test: a placeholder
//! carrier (ship 5) at (10,-2) with a bomber group (6) aboard, home port
//! base H. With no limit the staff forms a carrier task for each force
//! (C at (3,0), C2 at (3,-2)) and, at B, the strike and the search tasks.

use super::part_a::*;
use sim_core::command::{CommandRefusal, ForceKind, IntentKind};

/// The game with the second carrier force added to the file.
fn two_carriers(t: &str) -> String {
    at(
        t,
        "forces",
        "[[task_force]]\nid = 3\nname = \"Placeholder carrier force C2\"\nside = \"allied\"\nq = 10\nr = -2\nmission = \"carrier\"\nhome_port = 1\n\n[[ship]]\nid = 5\nname = \"Placeholder carrier 5\"\nclass = 17\nside = \"allied\"\nfuel_t = 1000\ntask_force = 3\n\n[[air_group]]\nid = 6\nname = \"Placeholder dive bombers 6\"\nside = \"allied\"\nship = 5\naircraft_type = 5\nready = 12\n\n",
    )
}

fn formed_kinds(played: &Played) -> Vec<IntentKind> {
    played.formed_all(1).into_iter().map(|(_, k)| k).collect()
}

fn operate(force: u32, h: sim_core::hex::Hex) -> IntentKind {
    IntentKind::OperateCarriers {
        force: sim_core::ids::TaskForceId::new(force),
        hex: h,
    }
}

fn at_base_b(kind: &IntentKind) -> bool {
    matches!(
        kind,
        IntentKind::StrikeShips { .. } | IntentKind::SearchWaters { .. }
    )
}

/// G1: a deny whose limit names the two carrier forces and lists
/// `air_groups` in `none_of` forms no task at any air base: no strike task
/// and no search task at B, and groups 4 and 5 are given no order; the two
/// carrier forces are stationed as they are in G2's game.
#[test]
fn g1_none_of_air_groups_forms_no_task_at_any_air_base() {
    let t = forces_limit(
        &two_carriers(&text()),
        1,
        "task_forces = [1, 3]\nnone_of = [\"air_groups\"]",
    );
    let played = play(&t, 1);
    assert_eq!(played.refusals(), vec![]);
    assert_eq!(
        formed_kinds(&played),
        vec![operate(1, hex(3, 0)), operate(3, hex(3, -2))]
    );
    assert!(!formed_kinds(&played).iter().any(at_base_b));
    assert!(played.group_orders(1, 4).is_empty());
    assert!(played.group_orders(1, 5).is_empty());
}

/// G2: the same limit without `none_of` forms the tasks where a base's
/// bombers reach: the strike task and the search task at B, with the groups
/// 4 and 5 ordered, besides the two carrier tasks.
#[test]
fn g2_without_none_of_the_base_tasks_are_formed() {
    let t = forces_limit(&two_carriers(&text()), 1, "task_forces = [1, 3]");
    let played = play(&t, 1);
    assert_eq!(played.refusals(), vec![]);
    assert_eq!(
        formed_kinds(&played),
        vec![
            operate(1, hex(3, 0)),
            operate(3, hex(3, -2)),
            IntentKind::StrikeShips {
                base: sim_core::ids::BaseId::new(2)
            },
            IntentKind::SearchWaters {
                base: sim_core::ids::BaseId::new(2)
            },
        ]
    );
    assert!(!played.group_orders(1, 4).is_empty());
    assert!(!played.group_orders(1, 5).is_empty());
}

/// G3: a limit that names an air group and lists `air_groups` in `none_of`
/// is refused, with the refusal that names the kind; nothing is set, so the
/// staff forms what it forms with no limit.
#[test]
fn g3_naming_a_group_and_listing_air_groups_is_refused() {
    let t = forces_limit(
        &two_carriers(&text()),
        1,
        "air_groups = [4]\nnone_of = [\"air_groups\"]",
    );
    let played = play(&t, 1);
    assert_eq!(
        played.refusals(),
        vec![(1, CommandRefusal::NamedAndNoneOf(ForceKind::AirGroups))]
    );
    let plain = play(&two_carriers(&text()), 1);
    assert_eq!(played.formed_all(1), plain.formed_all(1));
    assert_eq!(played.said(1), plain.said(1));
}

/// G4: a limit that names nothing and lists nothing uses nothing of any
/// kind. Shown on four games, one for each kind a limit can bind:
/// - ships and air groups, in the deny: the staff forms no carrier task, no
///   task at B and no stand, and cannot meet the order;
/// - land units, in a hold of base 2 with two fighting units ashore on the
///   attack: neither is set to defend (with no limit naming land units both
///   are);
/// - air groups again, in that hold: no cover is formed from the base's
///   fighters (statement 5's game gives the cover orders), and no supply;
/// - boats, in a patrol of seven boats: no boat is ordered.
#[test]
fn g4_a_limit_naming_and_listing_nothing_uses_nothing_of_any_kind() {
    // Ships and air groups.
    let played = play(&forces_limit(&two_carriers(&text()), 1, ""), 1);
    assert_eq!(played.refusals(), vec![]);
    assert_eq!(played.formed_all(1), vec![]);
    assert!(played.cannot_meet(1));
    for g in [1, 2, 3, 4, 5, 6] {
        assert!(played.group_orders(1, g).is_empty(), "group {g}");
    }
    for f in [1, 2, 3] {
        assert!(played
            .force_orders(1, sim_core::ids::TaskForceId::new(f))
            .is_empty());
    }

    // Land units, and the base's groups.
    let hold = hold_with_men("");
    assert_eq!(units_set_to_defend(&hold), Vec::<u32>::new());
    assert!(cover_orders(&hold).is_empty());
    assert!(!hold.formed_all(1).iter().any(|(_, kind)| matches!(
        kind,
        IntentKind::Cover { .. } | IntentKind::KeepSupplied { .. }
    )));
    // With a limit naming only a source they are the staff's: both are set
    // to defend (statement 5).
    assert_eq!(
        units_set_to_defend(&hold_with_men("source = 1")),
        vec![1, 2]
    );

    // Boats.
    assert_eq!(patrol(Some("")), vec![]);
    assert_eq!(patrol(None).len(), 7);
}
