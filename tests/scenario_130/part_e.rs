//! Scenario 130, part E: the tests of its statements (see the doc comment
//! of the file above): supply ranked with its order (rule 6).
//!
//! # The game
//!
//! `data/scenarios/a-limit-binds-what-it-names/130_two_holds.toml`: base 1
//! at S, the source, with 1,000 t; base 2 at T (5,0) and base 3 at U (0,5),
//! each with 10 t and 200 men who eat 1 t a day; one transport force, task
//! force 1, one fast transport of 30 t of cargo, in port at base 1. The
//! Allied commander holds base 2 (intent 1) and base 3 (intent 2), each with
//! a forces limit of task force 1 and source base 1, at the balanced risk
//! (a floor of 5 days). The staff forms "keep supplied" under each hold on
//! the first morning (intents 3 and 4, in the hold's number order), and the
//! one transport serves one of them at a time. What the games show is the
//! destination of the first run: the first route the staff gives task force
//! one. With no rank anywhere it is T, on day 6, the lower hold's place (the
//! run is called on day 5, loaded that day and sails on day 6); the second
//! run, to U, is called on day 9 when the ship is free.

use super::common;
use super::part_a::*;
use sim_core::state::Side;

pub fn holds_path() -> std::path::PathBuf {
    common::data_dir().join("scenarios/a-limit-binds-what-it-names/130_two_holds.toml")
}

pub fn holds_text() -> String {
    std::fs::read_to_string(holds_path()).expect("the scenario file reads")
}

fn play_holds(text: &str, days: u32) -> Played {
    play_at(text, &holds_path(), &common::frozen(), days, Side::Allied)
}

/// A rank on `intent`, given on `day`.
fn rank(text: &str, day: u32, intent: u32, rank: u32) -> String {
    at(
        text,
        "commands",
        &command(
            day,
            &format!(
                "kind = \"set_limit\"\nlimit = \"priority\"\nintent = {intent}\nrank = {rank}"
            ),
        ),
    )
}

/// The hexes task force 1 is first routed to, in order of the days, each
/// with its day.
fn routes(played: &Played, days: u32) -> Vec<(u32, sim_core::hex::Hex)> {
    (1..=days)
        .filter_map(|day| {
            played
                .route_of(day, sim_core::ids::TaskForceId::new(1))
                .map(|w| (day, w[0]))
        })
        .collect()
}

/// The setting is what the doc comment says: the staff forms one supply
/// under each hold, and with no rank the runs go to T then U.
#[test]
fn the_file_is_the_setting_the_doc_comment_describes() {
    let played = play_holds(&holds_text(), 14);
    let formed: Vec<_> = played
        .formed_all(1)
        .into_iter()
        .map(|(p, k)| (p.raw(), format!("{k:?}")))
        .collect();
    assert_eq!(
        formed,
        vec![
            (
                1,
                "KeepSupplied { base: BaseId(2), floor_days: 5, interval_days: None }".to_string()
            ),
            (
                2,
                "KeepSupplied { base: BaseId(3), floor_days: 5, interval_days: None }".to_string()
            ),
        ]
    );
}

/// Statement 21: two orders to hold two places, each with supply the staff
/// forms under it, ships enough for one run: with the second order ranked
/// first, its place is supplied first. The run to U sails on day 6, and T's
/// follows when the ship is free, on day 10.
#[test]
fn statement_21_the_second_order_ranked_first_is_supplied_first() {
    let t = rank(&holds_text(), 1, 2, 1);
    let played = play_holds(&t, 14);
    let routes = routes(&played, 14);
    assert_eq!(
        routes.first(),
        Some(&(6, hex(0, 5))),
        "the first run goes to U: {routes:?}"
    );
    assert_eq!(
        routes.iter().map(|(_, h)| *h).collect::<Vec<_>>(),
        vec![hex(0, 5), hex(5, 0)],
        "then T: {routes:?}"
    );
}

/// Statement 22: neither ranked, the lower number first, as before.
#[test]
fn statement_22_neither_ranked_the_lower_number_first() {
    let played = play_holds(&holds_text(), 14);
    let routes = routes(&played, 14);
    assert_eq!(routes.first(), Some(&(6, hex(5, 0))), "{routes:?}");
    assert_eq!(
        routes.iter().map(|(_, h)| *h).collect::<Vec<_>>(),
        vec![hex(5, 0), hex(0, 5)],
        "{routes:?}"
    );
}

/// Statement 23: the commander ranks the supply the staff formed under hold
/// 2 (intent 4) first, on day 2, and ranks the holds above them second
/// (hold 1) and third (hold 2). Read up the tree from intent 4 its own rank
/// 1 is first, before hold 2's rank 3, and intent 3 reads hold 1's rank 2:
/// so intent 4, U, is served first. (Were the rank above read first, intent
/// 4 would read 3 and intent 3 would read 2, and T would be first, as it is
/// with no rank at all.)
#[test]
fn statement_23_a_rank_on_the_supply_is_read_before_one_above() {
    let t = rank(&holds_text(), 1, 1, 2);
    let t = rank(&t, 1, 2, 3);
    let t = rank(&t, 2, 4, 1);
    let played = play_holds(&t, 14);
    let supply = played
        .scenario
        .world
        .side_view(Side::Allied)
        .command_state()
        .clone();
    assert_eq!(
        supply.priority_read_up(sim_core::ids::IntentId::new(4)),
        Some(1)
    );
    assert_eq!(
        supply.priority_read_up(sim_core::ids::IntentId::new(3)),
        Some(2)
    );
    let routes = routes(&played, 14);
    assert_eq!(routes.first(), Some(&(6, hex(0, 5))), "{routes:?}");
}
