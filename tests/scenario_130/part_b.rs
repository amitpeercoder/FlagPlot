//! Scenario 130, part B: the tests of its statements (see the doc comment
//! of the file above): clearing (rule 3), and a forces limit that names
//! nothing (statement 12).
//!
//! # The game
//!
//! Statement 1's: `data/scenarios/a-limit-binds-what-it-names/130_a_limit_binds_what_it_names.toml`
//! (its header says what it holds): a deny of the waters within 2 hexes of
//! (0,0) as intent 1; base B with a bomber group (4) and a search group (5)
//! in reach; carrier force C (task force 1) at (10,0). With no limit the
//! staff forms, on day 1, a carrier task for C at (3,0), a task to strike
//! ships from B and a task to search the waters from B. The limit the games
//! set is a forces limit that names the air group 4 and nothing else: it
//! binds the air groups, so group 5 is not used and no search task is formed
//! at B (statement 3). The leaves are given on day 1, so the plan is
//! approved on day 1; a command of day 2 (taken before the staff's morning
//! of day 2) is therefore on a plan in force, an amendment.

use super::part_a::*;
use sim_core::command::{CommandKind, CommandRefusal, Limit};
use sim_core::events::EventKind;
use sim_core::ids::{IntentId, TaskForceId};
use sim_core::state::Side;

const CLEAR_FORCES: &str = "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\ncleared = true";
const CLEAR_AREA: &str = "kind = \"set_limit\"\nlimit = \"area\"\nintent = 1\ncleared = true";
const SET_GROUP_4: &str = "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nair_groups = [4]";
const SET_AREA: &str =
    "kind = \"set_limit\"\nlimit = \"area\"\nintent = 1\nhexes = [[0, 0], [1, 0]]";

/// `body` on `day`.
fn on(text: &str, day: u32, body: &str) -> String {
    at(text, "commands", &command(day, body))
}

/// The limits on intent 1 at the end of the game.
fn deny_limits(played: &Played) -> (Option<sim_core::command::ForcesLimit>, Option<usize>) {
    let state = played
        .scenario
        .world
        .side_view(Side::Allied)
        .command_state()
        .clone();
    let limits = &state.intents[&IntentId::new(1)].limits;
    (limits.forces.clone(), limits.area.as_ref().map(|a| a.len()))
}

/// What statement 1's game has on day 1: the tasks, the orders to groups 4
/// and 5 and to C, and the words of every reason.
fn same_as_statement_1(played: &Played, day: u32) {
    let plain = play(&text(), 1);
    assert_eq!(played.formed_all(day), plain.formed_all(1));
    for g in [4, 5, 1, 2, 3] {
        assert_eq!(played.group_orders(day, g), plain.group_orders(1, g), "{g}");
    }
    assert_eq!(
        played.force_orders(day, TaskForceId::new(1)),
        plain.force_orders(1, TaskForceId::new(1))
    );
    assert_eq!(played.said(day), plain.said(1));
}

/// The commands taken in the game that clear a forces limit or an area.
fn clearings(played: &Played) -> Vec<(u32, Limit)> {
    played
        .scenario
        .world
        .events()
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::CommandTaken {
                command: CommandKind::SetLimit(limit),
                ..
            } if matches!(
                limit,
                Limit::ForcesCleared { .. } | Limit::AreaCleared { .. }
            ) =>
            {
                Some((e.day, limit.clone()))
            }
            _ => None,
        })
        .collect()
}

/// Statement 9, the forces limit set and then cleared, both on day 1 before
/// the staff's morning: the order is as if none had been set. The staff
/// forms what statement 1 has: the same three tasks, the same orders to the
/// groups and C, and every reason in the same words. The clearing is a
/// command taken like the setting, and no command is refused.
#[test]
fn statement_9_a_forces_limit_set_and_cleared_leaves_the_order_as_it_was() {
    let t = with(&with(&text(), SET_GROUP_4), CLEAR_FORCES);
    let played = play(&t, 1);
    same_as_statement_1(&played, 1);
    assert_eq!(deny_limits(&played), (None, None));
    assert_eq!(
        clearings(&played),
        vec![(
            1,
            Limit::ForcesCleared {
                intent: IntentId::new(1)
            }
        )]
    );
    assert_eq!(played.refusals(), vec![]);
}

/// Statement 9, cleared after the staff has worked under the limit: set on
/// day 1, the staff forms the carrier task and the strike task and no
/// search task (statement 3); cleared on day 2, it forms the search task on
/// the morning of day 2 and gives group 5 the orders statement 1 gives it:
/// the order is as if none had been set.
#[test]
fn statement_9_cleared_on_a_later_day_the_staff_forms_what_statement_1_has() {
    let t = on(&with(&text(), SET_GROUP_4), 2, CLEAR_FORCES);
    let played = play(&t, 2);
    assert_eq!(
        kinds_of(&played, 1),
        vec![
            "OperateCarriers { force: TaskForceId(1), hex: Hex { q: 3, r: 0 } }",
            "StrikeShips { base: BaseId(2) }"
        ]
    );
    assert_eq!(
        kinds_of(&played, 2),
        vec!["SearchWaters { base: BaseId(2) }"]
    );
    let plain = play(&text(), 1);
    assert_eq!(played.group_orders(2, 5), plain.group_orders(1, 5));
    assert_eq!(deny_limits(&played), (None, None));
}

fn kinds_of(played: &Played, day: u32) -> Vec<String> {
    played
        .formed_all(day)
        .into_iter()
        .map(|(_, k)| format!("{k:?}"))
        .collect()
}

/// Statement 9, an amendment of a plan in force as the setting is: the plan
/// is approved on day 1, and on day 2 a clearing amends it once, as a
/// setting does; a clearing before the plan is approved amends nothing, as
/// a setting before it does not.
#[test]
fn statement_9_a_clearing_is_an_amendment_of_a_plan_in_force() {
    let set = play(&on(&text(), 2, SET_GROUP_4), 2);
    assert_eq!(
        set.amendments(),
        vec![(2, sim_core::command::PlanId::new(1))]
    );
    let cleared = play(&on(&with(&text(), SET_GROUP_4), 2, CLEAR_FORCES), 2);
    assert_eq!(
        cleared.amendments(),
        vec![(2, sim_core::command::PlanId::new(1))]
    );
    assert_eq!(
        clearings(&cleared),
        vec![(
            2,
            Limit::ForcesCleared {
                intent: IntentId::new(1)
            }
        )]
    );
    // Before the approval, neither amends.
    let early = play(&with(&with(&text(), SET_GROUP_4), CLEAR_FORCES), 1);
    assert_eq!(early.amendments(), vec![]);
}

/// Statement 9, refused for an order not the side's: a clearing of intent 9,
/// which the side does not have, is refused with the refusal a setting on it
/// gets, nothing changes, and the staff forms what statement 1 has.
#[test]
fn statement_9_a_clearing_is_refused_for_an_order_not_the_sides() {
    let clear_9 = "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 9\ncleared = true";
    let set_9 = "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 9\nair_groups = [4]";
    let cleared = play(&with(&text(), clear_9), 1);
    let set = play(&with(&text(), set_9), 1);
    assert_eq!(
        cleared.refusals(),
        vec![(1, CommandRefusal::NoSuchIntent(IntentId::new(9)))]
    );
    assert_eq!(cleared.refusals(), set.refusals());
    same_as_statement_1(&cleared, 1);
    assert_eq!(clearings(&cleared), vec![]);
    // On a plan in force the same.
    let later = play(&on(&text(), 2, clear_9), 2);
    assert_eq!(
        later.refusals(),
        vec![(2, CommandRefusal::NoSuchIntent(IntentId::new(9)))]
    );
    assert_eq!(later.amendments(), vec![]);
}

/// Statement 10, the same for an area: an area of two hexes set and then
/// cleared leaves the order with no area and the staff forms what statement
/// 1 has; a clearing of an area is a command taken like the setting and an
/// amendment of a plan in force; refused for an order not the side's. (A
/// deny reads no area: that is why nothing the staff forms depends on it,
/// and the area is shown by the order's limits.)
#[test]
fn statement_10_an_area_set_and_cleared_leaves_the_order_as_it_was() {
    let set = play(&with(&text(), SET_AREA), 1);
    assert_eq!(deny_limits(&set), (None, Some(2)));
    let t = with(&with(&text(), SET_AREA), CLEAR_AREA);
    let played = play(&t, 1);
    assert_eq!(deny_limits(&played), (None, None));
    same_as_statement_1(&played, 1);
    assert_eq!(
        clearings(&played),
        vec![(
            1,
            Limit::AreaCleared {
                intent: IntentId::new(1)
            }
        )]
    );
    assert_eq!(played.refusals(), vec![]);

    // On a plan in force, an amendment as the setting is.
    let later = play(&on(&with(&text(), SET_AREA), 2, CLEAR_AREA), 2);
    assert_eq!(deny_limits(&later), (None, None));
    assert_eq!(
        later.amendments(),
        vec![(2, sim_core::command::PlanId::new(1))]
    );

    // Refused for an order not the side's.
    let refused = play(
        &with(
            &text(),
            "kind = \"set_limit\"\nlimit = \"area\"\nintent = 9\ncleared = true",
        ),
        1,
    );
    assert_eq!(
        refused.refusals(),
        vec![(1, CommandRefusal::NoSuchIntent(IntentId::new(9)))]
    );
}

/// Statement 11: clearing a forces limit or an area that is not set changes
/// nothing and is not refused: no refusal, no limit on the order after, the
/// staff forms what statement 1 has, in the same words, with the same
/// orders. (On a plan in force, the clearing is taken as an amendment all
/// the same, as a setting is; that is reported with the statements.)
#[test]
fn statement_11_clearing_what_is_not_set_changes_nothing() {
    for body in [CLEAR_FORCES, CLEAR_AREA] {
        let played = play(&with(&text(), body), 1);
        assert_eq!(played.refusals(), vec![], "{body}");
        assert_eq!(deny_limits(&played), (None, None), "{body}");
        same_as_statement_1(&played, 1);
        // On day 2, on a plan in force: not refused, nothing set.
        let later = play(&on(&text(), 2, body), 2);
        assert_eq!(later.refusals(), vec![], "{body}");
        assert_eq!(deny_limits(&later), (None, None), "{body}");
        assert_eq!(later.formed_all(2), vec![], "{body}");
    }
}

/// Statement 12: a forces limit that names nothing is "use nothing", not
/// cleared. The limit is set with no unit, no source and no kind listed:
/// it is on the order after (not none), the staff forms no task for the
/// order, not at B and not for C either, hands in that it cannot meet the
/// order, and its reason says no base the order may use has a bomber or a
/// group that can search in reach: "the side has no base it may use". The
/// list of limits a reason carries is not asserted here (part D proves
/// it).
#[test]
fn statement_12_a_limit_naming_nothing_is_use_nothing_and_not_cleared() {
    let played = play(
        &with(
            &text(),
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1",
        ),
        1,
    );
    let (forces, _) = deny_limits(&played);
    assert!(forces.expect("the limit is set").names_nothing());
    assert_eq!(played.formed_all(1), vec![]);
    assert!(played.group_orders(1, 4).is_empty());
    assert!(played.group_orders(1, 5).is_empty());
    assert_eq!(played.route_of(1, TaskForceId::new(1)), None);
    assert!(played.cannot_meet(1));
    let said = played.said(1);
    assert!(
        said.contains(
            "No base intent:1 may use has a bomber whose normal reach, or a group whose search range, touches the waters within 2 hexes of (0,0) (19 hex(es) on the map): the side has no base it may use. The staff forms no task and cannot meet intent:1."
        ),
        "{said}"
    );
}
