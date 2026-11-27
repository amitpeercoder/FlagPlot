//! Scenario 134: the estimate of the enemy's boats, as a reading (decision
//! 0223 as amended on review, stage D1, with the architect's rulings in its
//! section "Stage D1: the statements, as written").
//!
//! # The architect's statements
//!
//! Written by the architect on Nov 23, 2026, before any code. Nobody else
//! changes a statement, a hex or a count here; a mismatch goes back to the
//! architect. The areas and counts are worked by hand below.
//!
//! ## Figures
//!
//! The frozen step 1 figures with `position_error_chance` 0, through
//! `common::altered_reference`, so that a report of a submarine names her
//! own hex. The submarine figures are the scenario's own
//! (`submarines_placeholder_scenario_134.toml`): scenario 128's with
//! `submarine_surfaced_share` 1 and `air_attack_damages_chance` 0;
//! `submarine_report_drop_age_pulses` is 6. The doctrine's figure
//! `estimate_join_hexes` is 2, a judgement, in the code. The data is
//! placeholder data.
//!
//! Hexes are axial (q, r); the distance between two is (|dq| + |dr| +
//! |dq + dr|) / 2; hex order is by q, then r. A hex is 40 nm.
//!
//! ## The game
//!
//! `data/scenarios/the-estimate-of-boats/134_the_estimate_of_boats.toml`,
//! 4 days, on scenario 131's map. Japanese base 1 at (-9,0). Allied base 2
//! at (8,0) with group 1, 100 flying boats; Allied base 3 at (4,0), with
//! nothing. Eight Japanese boats, each on her station, numbered in the
//! order of their hexes:
//!
//! | Boat | Hex | From base 2 | Bearing from base 2 |
//! | --- | --- | --- | --- |
//! | 1 | (-4,2) | 445 nm | 261 |
//! | 2 | (-4,3) | 433 nm | 256 |
//! | 3 | (-3,3) | 394 nm | 255 |
//! | 4 | (-2,2) | 367 nm | 259 |
//! | 5 | (-1,2) | 327 nm | 258 |
//! | 6 | (0,-1) | 342 nm | 276 |
//! | 7 | (0,0) | 320 nm | 270 |
//! | 8 | (6,-6) | 288 nm | 316 |
//!
//! Group 1's orders: day 1, the search mission and the whole circle to 600
//! nm; day 2, the arc 280 to 330; day 3, rest. No commander's command is
//! given and neither side has a staff's order to give. No Allied ship is at
//! sea, so no boat attacks.
//!
//! **What is reported.** A boat on her station in a searching group's arc
//! and range is sighted at the chance of sighting a force there times the
//! surfaced share. With 100 flying boats the first is 57.295779 x 100 x 40
//! = 229,183.116 over the distance times the arc's width, at most 445 x 360
//! = 160,200: 1. The share is 1. So each boat in the arc is reported in
//! each daylight pulse the group flies, at her own hex, and is never
//! damaged. A side holds one report for a hex, the newer.
//!
//! - Day 1, morning and afternoon: all eight, the whole circle.
//! - Day 2, morning and afternoon: boat 8 alone. Her bearing, 316, is
//!   inside 280 to 330; the nearest of the others, boat 6 on 276, is 4
//!   degrees outside it.
//! - Days 3 and 4: none.
//!
//! **The ages.** A report made in the afternoon of day 1 is 0 pulses old at
//! the end of that day, 3 at the end of day 2, and reaches 6 at the end of
//! the afternoon of day 3, when it is dropped. Boat 8's report of the
//! afternoon of day 2 is dropped at the end of the afternoon of day 4.
//!
//! | After day | Reports held |
//! | --- | --- |
//! | 1 | all eight hexes, each of day 1 |
//! | 2 | all eight; (6,-6) of day 2, the others of day 1 |
//! | 3 | (6,-6), of day 2 |
//! | 4 | none |
//!
//! ## The areas, worked by hand
//!
//! The reports are taken in hex order. A report within 2 hexes of the first
//! report of an area joins it, the earliest-begun where there are two; else
//! it begins an area. A report is counted where it is more than one hex
//! from every report counted before it in its area.
//!
//! | Report | To the first report of A, (-4,2) | Of B, (-1,2) | Of C, (0,-1) | So | Counted |
//! | --- | --- | --- | --- | --- | --- |
//! | (-4,2) | | | | begins A | yes: A is 1 |
//! | (-4,3) | 1 | | | joins A | no: 1 from (-4,2) |
//! | (-3,3) | 2 | | | joins A | yes, 2 from (-4,2): A is 2 |
//! | (-2,2) | 2 | | | joins A | no: 1 from (-3,3) |
//! | (-1,2) | 3 | | | begins B, though it is next to (-2,2) of A | yes: B is 1 |
//! | (0,-1) | 4 | 3 | | begins C | yes: C is 1 |
//! | (0,0) | 4 | 2 | 1 | joins B, the earlier begun of B and C | yes, 2 from (-1,2): B is 2 |
//! | (6,-6) | 10 | 8 | 6 | begins D | yes: D is 1 |
//!
//! The distances: (-4,3) from (-4,2) is (0 + 1 + 1) / 2 = 1. (-3,3) from
//! (-4,2): (1 + 1 + 2) / 2 = 2. (-2,2) from (-4,2): (2 + 0 + 2) / 2 = 2,
//! and from (-3,3): (1 + 1 + 0) / 2 = 1. (-1,2) from (-4,2): 3. (0,-1) from
//! (-4,2): (4 + 3 + 1) / 2 = 4, and from (-1,2): (1 + 3 + 2) / 2 = 3. (0,0)
//! from (-4,2): (4 + 2 + 2) / 2 = 4; from (-1,2): (1 + 2 + 1) / 2 = 2; from
//! (0,-1): 1. (6,-6) from (-4,2): (10 + 8 + 2) / 2 = 10; from (-1,2):
//! (7 + 8 + 1) / 2 = 8; from (0,-1): (6 + 5 + 1) / 2 = 6.
//!
//! ## Statements
//!
//! 1. **After day 1** the Allied side's estimate has four areas, in the
//!    order they were begun, each on the basis "reported":
//!
//!    | Area | Centre | Its reports, in hex order | Count | Days |
//!    | --- | --- | --- | --- | --- |
//!    | A | (-4,2) | (-4,2), (-4,3), (-3,3), (-2,2) | about 2 | 1 |
//!    | B | (-1,2) | (-1,2), (0,0) | about 2 | 1 |
//!    | C | (0,-1) | (0,-1) | about 1 | 1 |
//!    | D | (6,-6) | (6,-6) | about 1 | 1 |
//!
//!    Each area is told with its centre, the hex of its first report, and
//!    the distance 2.
//! 2. **No area joins another.** (-1,2) lies next to (-2,2), a report of
//!    area A, and begins area B: it is 3 hexes from A's first report.
//! 3. **The earlier area takes a report within 2 of two.** (0,0) is 2 from
//!    B's first report and 1 from C's, and joins B.
//! 4. **The count is said "about",** never "at least", in every word a
//!    seat is given, and a count is a whole number of 1 or more.
//! 5. **After day 2** the areas are those of statement 1 but for area D's
//!    day, which is 2.
//! 6. **After day 3** there is one area: D, centre (6,-6), about 1, day 2.
//!    **After day 4** there is none, and the estimate holds nothing.
//! 7. **By habit** (the game with `[side.allied]` and
//!    `expects_boats_off_own_bases = true`): beside the reported areas of
//!    each day the estimate names, on the basis "by habit" and with no
//!    count, Allied base 2 at (8,0) and Allied base 3 at (4,0), in base
//!    number order, as waters where boats may be expected. They are named
//!    on every day, after day 4 too, when no report is held. In the game
//!    as filed, where the scenario says nothing, none is named.
//! 8. **The other side.** The Japanese side holds no report and its
//!    estimate holds nothing, in the game as filed and in the game of
//!    statement 7. In the game with `[side.japanese]` and
//!    `expects_boats_off_own_bases = true` its estimate names Japanese base
//!    1 at (-9,0) by habit and nothing else, and the Allied side's is that
//!    of statements 1, 5 and 6.
//! 9. **Told in `list_contacts`.** The Allied seat's `list_contacts` gives
//!    the estimate as `submarine_estimate`, beside `submarine_reports` and
//!    `suspected_waters`, which are what they were: after day 1 the eight
//!    reports, each with its place, day and age, and the four areas of
//!    statement 1. `submarine_estimate` is left out when the estimate holds
//!    nothing. With an area given to the tool, the areas are those whose
//!    centre lies in it, and a base named by habit is given where its hex
//!    lies in it: after day 1, for the hexes within 2 of (0,0), areas B and
//!    C and no other.
//! 10. **Suspected waters stand beside it** (the game with one
//!     `[[suspected_waters]]` entry for the Allied side, the waters within
//!     3 of (-4,2), told on day 1): the entry is given as it was, and the
//!     estimate is that of statement 1: nothing is joined to it or counted
//!     from it.
//! 11. **One command's picture** (the game with `command_levels = true`
//!     and `reports_placeholder_scenario_134.toml`, in which no report
//!     arrives at another command). After day 1 the estimate read from base
//!     2's picture is that of statement 1, and the estimate read from the
//!     area command's picture has no reported area. The Allied seat at the
//!     area is given no `submarine_estimate`; with the habit of statement 7
//!     it is given the two bases and no reported area.
//! 12. **A reading and no more.** The estimate is kept nowhere: the state
//!     layout, the state hash, the saved game and the event log of the game
//!     are what they are with the reading never asked for, and asking for
//!     it twice gives the same answer. No event is recorded for it.
//! 13. **It drives nothing.** No order, command, reason or notice of any
//!     staff differs for it: the comparison of every run on file, live and
//!     frozen, shows no difference from the commit this is built on, the
//!     hashes included. The staff's regard for submarine waters reads the
//!     reports and the suspected waters as it did (scenario 129's and
//!     scenario 131's statements hold as written).
//! 14. **Nothing it cannot see.** The game with a ninth Japanese boat at
//!     (-10,8), 625 nm from base 2 and never reported, and the game with
//!     boat 1's salvoes cut to 1, give the Allied side the estimate of
//!     statements 1, 5 and 6 on every day. The function that makes the
//!     estimate takes a picture, the side's own bases and the side's own
//!     doctrine, and neither a world nor the umpire's view.
//! 15. **The frozen figures are untouched** and no rule's figure file gains
//!     a key: `estimate_join_hexes` is a constant, marked a judgement where
//!     it is written.
//!
//! If the game cannot be set up as written (a seat's `list_contacts` read
//! at a base's command, a boat sighted at a chance of 1), the build says so
//! and it comes back to the architect; no statement is changed to fit.

use crate::common;

use std::path::PathBuf;

use serde_json::{json, Value};
use sim_agent::{list_contacts, Post, Seat, Session};
use sim_core::command::SubmarineDoctrine;
use sim_core::hex::Hex;
use sim_core::ids::BaseId;
use sim_core::pictures::CommandRef;
use sim_core::state::Side;
use sim_core::submarine_estimate::{submarine_estimate, SubmarineEstimate};
use sim_core::World;
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, staffs};

// ---------------------------------------------------------------------
// The games, each made from the text of the file.
// ---------------------------------------------------------------------

fn path() -> PathBuf {
    common::data_dir().join("scenarios/the-estimate-of-boats/134_the_estimate_of_boats.toml")
}

fn file_text() -> String {
    std::fs::read_to_string(path()).expect("the game's file reads")
}

/// The frozen figures with `position_error_chance` 0.
fn options() -> LoadOptions {
    common::altered_reference("134", |dir| {
        common::edit_file(dir, "search.toml", |text| {
            common::replace_once(
                &text,
                "position_error_chance = \"0.3\"",
                "position_error_chance = \"0\"",
            )
        });
    })
}

/// The file's text with one thing added at the place a game says.
#[derive(Clone, Default)]
struct Game {
    /// `[side.allied]` with the habit.
    allied_habit: bool,
    /// `[side.japanese]` with the habit.
    japanese_habit: bool,
    /// One `[[suspected_waters]]` entry for the Allied side.
    waters: bool,
    command_levels: bool,
    ninth_boat: bool,
    salvoes_cut: bool,
}

impl Game {
    fn text(&self) -> String {
        let mut text = file_text();
        let mut sides = String::new();
        if self.allied_habit {
            sides.push_str("[side.allied]\nexpects_boats_off_own_bases = true\n\n");
        }
        if self.japanese_habit {
            sides.push_str("[side.japanese]\nexpects_boats_off_own_bases = true\n\n");
        }
        if !sides.is_empty() {
            text = common::replace_once(
                &text,
                "# The side tables\n",
                &format!("# The side tables\n{sides}"),
            );
        }
        if self.command_levels {
            text = common::replace_once(
                &text,
                "days = 4\n",
                "days = 4\ncommand_levels = true\nreports = \"../../reference/reports_placeholder_scenario_134.toml\"\n",
            );
        }
        if self.waters {
            text.push_str(
                "\n[[suspected_waters]]\nside = \"allied\"\nq = -4\nr = 2\ndistance = 3\nday = 1\nwords = \"Placeholder: the staff believes enemy submarines work in these waters.\"\nrows = \"Placeholder: no row.\"\nreading = \"Placeholder: made up to try the estimate.\"\n",
            );
        }
        if self.ninth_boat {
            text.push_str(
                "\n[[submarine]]\nid = 9\nname = \"Placeholder submarine 9\"\nside = \"japanese\"\nclass = 6900\nq = -10\nr = 8\nhome = 1\n",
            );
        }
        if self.salvoes_cut {
            text = common::replace_once(
                &text,
                "class = 6900\nq = -4\nr = 2\nhome = 1\n",
                "class = 6900\nq = -4\nr = 2\nhome = 1\nsalvoes = 1\n",
            );
        }
        text
    }

    fn scenario(&self) -> Scenario {
        common::from_text_with(&self.text(), &path(), &options()).expect("the game loads")
    }
}

/// The world after `days` days of the game, played with the doctrine's
/// staffs.
fn world_after(game: &Game, days: u32) -> World {
    let held = staffs();
    let mut scenario = game.scenario();
    for _ in 0..days {
        scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves")
            .expect("the game has the day");
    }
    scenario.world
}

fn hex(q: i32, r: i32) -> Hex {
    Hex::new(q, r)
}

/// The estimate of one side's own picture, as the tool's function makes it.
fn estimate_of(world: &World, side: Side) -> SubmarineEstimate {
    let view = world.side_view(side);
    let bases: Vec<(BaseId, Hex)> = view.bases().map(|b| (b.id, b.hex)).collect();
    submarine_estimate(&view.seat_picture(), &bases, view.submarine_doctrine())
}

/// An area as a row: centre, reports, count, days.
type Row = ((i32, i32), Vec<(i32, i32)>, u32, Vec<u32>);

fn rows(estimate: &SubmarineEstimate) -> Vec<Row> {
    estimate
        .areas
        .iter()
        .map(|a| {
            (
                (a.centre.q, a.centre.r),
                a.reports.iter().map(|h| (h.q, h.r)).collect(),
                a.count,
                a.days.clone(),
            )
        })
        .collect()
}

fn habit(estimate: &SubmarineEstimate) -> Vec<(u32, (i32, i32))> {
    estimate
        .by_habit
        .iter()
        .map(|w| (w.base.raw(), (w.hex.q, w.hex.r)))
        .collect()
}

/// The areas of statement 1, with the day of area D given.
fn four_areas(day_of_d: u32) -> Vec<Row> {
    vec![
        (
            (-4, 2),
            vec![(-4, 2), (-4, 3), (-3, 3), (-2, 2)],
            2,
            vec![1],
        ),
        ((-1, 2), vec![(-1, 2), (0, 0)], 2, vec![1]),
        ((0, -1), vec![(0, -1)], 1, vec![1]),
        ((6, -6), vec![(6, -6)], 1, vec![day_of_d]),
    ]
}

fn one_area_d() -> Vec<Row> {
    vec![((6, -6), vec![(6, -6)], 1, vec![2])]
}

/// The tool's answer, as the Allied seat is given it after `days` days. A
/// seat at a post is placed before the first day and the game is played
/// through the session (the Allied seat the scenario's own script, the
/// Japanese seat ending each day); without a post the world is played with
/// the staffs and the session opened on it.
fn tool_answer(game: &Game, days: u32, post: Option<Post>, args: Value) -> Value {
    let name = "placeholder scenario 134".to_string();
    let mut session = match post {
        None => Session::new(
            world_after(game, days),
            &name,
            4,
            Seat::Player,
            Seat::Player,
        ),
        Some(post) => {
            let scenario = game.scenario();
            let mut session =
                Session::new(scenario.world.clone(), &name, 4, Seat::Player, Seat::Player)
                    .with_staffs(staffs())
                    .with_post(Side::Allied, post)
                    .expect("the seat is placed");
            for day in 1..=days {
                // The Allied seat gives the scenario's own orders of the
                // day, by the number its seat names its group.
                let orders: Vec<Value> = scenario
                    .scripted_orders()
                    .get(&day)
                    .into_iter()
                    .flatten()
                    .filter(|(who, _)| *who == Side::Allied)
                    .map(|(_, request)| {
                        json!({
                            "unit": sim_agent::unit_text(&request.kind),
                            "order": sim_agent::order_to_json(&request.kind),
                        })
                    })
                    .collect();
                let issued = session
                    .call(Side::Allied, "issue_orders", &json!({ "orders": orders }))
                    .unwrap_or_else(|e| panic!("issue_orders: {}", e.message));
                assert!(
                    issued["results"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .all(|r| r["status"] != "refused"),
                    "{issued}"
                );
                for side in [Side::Allied, Side::Japanese] {
                    let ended = session
                        .call(side, "end_day", &json!({}))
                        .unwrap_or_else(|e| panic!("end_day: {}", e.message));
                    assert!(ended.get("resolved").is_some(), "{ended}");
                }
            }
            session
        }
    };
    session
        .call(Side::Allied, "list_contacts", &args)
        .unwrap_or_else(|e| panic!("list_contacts {args}: {}", e.message))
}

// ---------------------------------------------------------------------
// The statements.
// ---------------------------------------------------------------------

#[test]
fn after_day_1_four_areas_with_their_counts() {
    // Statement 1, in the game as filed.
    let world = world_after(&Game::default(), 1);
    let estimate = estimate_of(&world, Side::Allied);
    assert_eq!(rows(&estimate), four_areas(1));
    assert!(estimate.by_habit.is_empty());
    // Each area is told with its centre and the distance 2, on the basis
    // "reported", in the order they were begun.
    let answer = tool_answer(&Game::default(), 1, None, json!({}));
    let areas = answer["submarine_estimate"]["areas"]
        .as_array()
        .unwrap_or_else(|| panic!("{answer}"));
    assert_eq!(areas.len(), 4);
    let centres: Vec<(i64, i64)> = areas
        .iter()
        .map(|a| {
            (
                a["place"]["q"].as_i64().unwrap(),
                a["place"]["r"].as_i64().unwrap(),
            )
        })
        .collect();
    assert_eq!(centres, vec![(-4, 2), (-1, 2), (0, -1), (6, -6)]);
    for area in areas {
        assert_eq!(area["distance_hexes"], 2, "{area}");
        assert_eq!(area["basis"], "reported", "{area}");
    }
    let counts: Vec<i64> = areas.iter().map(|a| a["count"].as_i64().unwrap()).collect();
    assert_eq!(counts, vec![2, 2, 1, 1]);
    let first_reports: Vec<(i64, i64)> = areas[0]["reports"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| (p["q"].as_i64().unwrap(), p["r"].as_i64().unwrap()))
        .collect();
    assert_eq!(first_reports, vec![(-4, 2), (-4, 3), (-3, 3), (-2, 2)]);
    assert_eq!(areas[0]["days"], json!([1]));
}

#[test]
fn no_area_joins_another_and_the_earlier_takes_a_report_within_two() {
    let estimate = estimate_of(&world_after(&Game::default(), 1), Side::Allied);
    let areas = rows(&estimate);
    // Statement 2: (-1,2), next to (-2,2) of area A, begins B.
    assert_eq!(areas[0].1, vec![(-4, 2), (-4, 3), (-3, 3), (-2, 2)]);
    assert_eq!(areas[1].0, (-1, 2));
    assert_eq!(hex(-1, 2).distance(hex(-4, 2)), 3);
    assert_eq!(hex(-1, 2).distance(hex(-2, 2)), 1);
    // Statement 3: (0,0) is 2 from B's first report and 1 from C's, and
    // joins B, the earlier begun.
    assert_eq!(hex(0, 0).distance(hex(-1, 2)), 2);
    assert_eq!(hex(0, 0).distance(hex(0, -1)), 1);
    assert!(areas[1].1.contains(&(0, 0)));
    assert_eq!(areas[2].1, vec![(0, -1)]);
}

#[test]
fn the_count_is_said_about() {
    let answer = tool_answer(&Game::default(), 1, None, json!({}));
    let text = answer["submarine_estimate"].to_string();
    assert!(!text.contains("at least"), "{text}");
    for area in answer["submarine_estimate"]["areas"].as_array().unwrap() {
        let count = area["count"].as_u64().unwrap();
        assert!(count >= 1, "{area}");
        let words = area["words"].as_str().unwrap();
        assert!(words.contains(&format!("about {count}")), "{words}");
    }
    // In every word a seat is given, of the game with the habit too.
    let habit_answer = tool_answer(
        &Game {
            allied_habit: true,
            ..Game::default()
        },
        1,
        None,
        json!({}),
    );
    assert!(
        !habit_answer.to_string().contains("at least"),
        "{habit_answer}"
    );
}

#[test]
fn the_days_of_the_reports_and_the_areas_as_reports_are_dropped() {
    let game = Game::default();
    // Statement 5.
    let after_2 = estimate_of(&world_after(&game, 2), Side::Allied);
    assert_eq!(rows(&after_2), four_areas(2));
    // Statement 6: one area, D, of day 2; then none.
    let after_3 = estimate_of(&world_after(&game, 3), Side::Allied);
    assert_eq!(rows(&after_3), one_area_d());
    let after_4 = estimate_of(&world_after(&game, 4), Side::Allied);
    assert!(after_4.is_empty());
    let answer = tool_answer(&game, 4, None, json!({}));
    assert!(answer.get("submarine_estimate").is_none(), "{answer}");
}

#[test]
fn by_habit_a_sides_own_bases_where_its_scenario_says_so() {
    // Statement 7: bases 2 and 3 on every day, after day 4 too.
    let game = Game {
        allied_habit: true,
        ..Game::default()
    };
    let bases = vec![(2, (8, 0)), (3, (4, 0))];
    for day in 0..=4 {
        let estimate = estimate_of(&world_after(&game, day), Side::Allied);
        assert_eq!(habit(&estimate), bases, "day {day}");
        let expected = match day {
            0 => vec![],
            1 => four_areas(1),
            2 => four_areas(2),
            3 => one_area_d(),
            _ => vec![],
        };
        assert_eq!(rows(&estimate), expected, "day {day}");
    }
    // Told with a basis and no count, and left out of the game as filed.
    let answer = tool_answer(&game, 4, None, json!({}));
    let by_habit = answer["submarine_estimate"]["by_habit"].as_array().unwrap();
    assert_eq!(by_habit.len(), 2, "{answer}");
    for line in by_habit {
        assert_eq!(line["basis"], "by habit");
        assert!(line.get("count").is_none());
    }
    assert!(answer["submarine_estimate"].get("areas").is_none());
    let filed = tool_answer(&Game::default(), 0, None, json!({}));
    assert!(filed.get("submarine_estimate").is_none(), "{filed}");
}

#[test]
fn the_other_side_is_given_nothing_of_it() {
    for game in [
        Game::default(),
        Game {
            allied_habit: true,
            ..Game::default()
        },
    ] {
        for day in 0..=4 {
            let estimate = estimate_of(&world_after(&game, day), Side::Japanese);
            assert!(estimate.is_empty(), "day {day}");
        }
    }
    // The Japanese side's habit names its base 1 and nothing else, and the
    // Allied side's estimate is that of statements 1, 5 and 6.
    let game = Game {
        japanese_habit: true,
        ..Game::default()
    };
    for (day, areas) in [
        (1, four_areas(1)),
        (2, four_areas(2)),
        (3, one_area_d()),
        (4, vec![]),
    ] {
        let world = world_after(&game, day);
        let japanese = estimate_of(&world, Side::Japanese);
        assert!(japanese.areas.is_empty(), "day {day}");
        assert_eq!(habit(&japanese), vec![(1, (-9, 0))], "day {day}");
        let allied = estimate_of(&world, Side::Allied);
        assert_eq!(rows(&allied), areas, "day {day}");
        assert!(allied.by_habit.is_empty());
    }
}

#[test]
fn told_in_list_contacts_beside_the_reports_and_the_suspected_waters() {
    let answer = tool_answer(&Game::default(), 1, None, json!({}));
    let reports = answer["submarine_reports"].as_array().unwrap();
    assert_eq!(reports.len(), 8, "{answer}");
    for report in reports {
        assert_eq!(report["day"], 1);
        assert!(report.get("age_pulses").is_some());
    }
    assert!(answer["submarine_estimate"]["areas"].is_array());
    assert!(answer.get("suspected_waters").is_none());
    // With an area given: the areas whose centre lies in it. The hexes
    // within 2 of (0,0) are 80 nm; B (-1,2) is 2 from (0,0), C (0,-1) is 1.
    let in_area = tool_answer(
        &Game::default(),
        1,
        None,
        json!({"area": {"q": 0, "r": 0, "radius_nm": 80}}),
    );
    let centres: Vec<(i64, i64)> = in_area["submarine_estimate"]["areas"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| {
            (
                a["place"]["q"].as_i64().unwrap(),
                a["place"]["r"].as_i64().unwrap(),
            )
        })
        .collect();
    assert_eq!(centres, vec![(-1, 2), (0, -1)], "{in_area}");
    // A base named by habit is given where its hex lies in the area.
    let habit_game = Game {
        allied_habit: true,
        ..Game::default()
    };
    let near_base_3 = tool_answer(
        &habit_game,
        1,
        None,
        json!({"area": {"q": 4, "r": 0, "radius_nm": 40}}),
    );
    let bases = near_base_3["submarine_estimate"]["by_habit"]
        .as_array()
        .unwrap();
    assert_eq!(bases.len(), 1, "{near_base_3}");
    assert_eq!(bases[0]["place"]["q"], 4);
    assert!(near_base_3["submarine_estimate"].get("areas").is_none());
    // Statement 10: the suspected waters stand beside it, as they were, and
    // the estimate is that of statement 1.
    let with_waters = Game {
        waters: true,
        ..Game::default()
    };
    let beside = tool_answer(&with_waters, 1, None, json!({}));
    let waters = beside["suspected_waters"].as_array().unwrap();
    assert_eq!(waters.len(), 1, "{beside}");
    assert_eq!(waters[0]["distance_hexes"], 3);
    assert_eq!(waters[0]["place"]["q"], -4);
    assert_eq!(beside["submarine_estimate"], answer["submarine_estimate"]);
    let estimate = estimate_of(&world_after(&with_waters, 1), Side::Allied);
    assert_eq!(rows(&estimate), four_areas(1));
    // The tool's answer is valid as the schema has it, and says version 58 (decision 0232 raised it from 57, and before that from 56 and 55).
    assert_eq!(answer["version"], 58);
}

#[test]
fn one_commands_picture_and_nothing_else() {
    let game = Game {
        command_levels: true,
        ..Game::default()
    };
    let world = world_after(&game, 1);
    let view = world.side_view(Side::Allied);
    let doctrine = view.submarine_doctrine();
    let bases: Vec<(BaseId, Hex)> = view.bases().map(|b| (b.id, b.hex)).collect();
    // Base 2's picture: statement 1. The area command's: no reported area.
    let at_base = view
        .command(CommandRef::Base(BaseId::new(2)))
        .expect("base 2 is a command");
    assert_eq!(
        rows(&submarine_estimate(&at_base, &bases, doctrine)),
        four_areas(1)
    );
    let at_area = view.command(CommandRef::Area).expect("the area command");
    assert!(submarine_estimate(&at_area, &bases, doctrine)
        .areas
        .is_empty());
    // The seat at the area is given no submarine_estimate.
    let seat = tool_answer(&game, 1, Some(Post::Area), json!({}));
    assert!(seat.get("submarine_estimate").is_none(), "{seat}");
    assert!(seat.get("submarine_reports").is_none(), "{seat}");
    // With the habit it is given the two bases and no reported area.
    let with_habit = Game {
        allied_habit: true,
        command_levels: true,
        ..Game::default()
    };
    let seat = tool_answer(&with_habit, 1, Some(Post::Area), json!({}));
    let estimate = &seat["submarine_estimate"];
    assert!(estimate.get("areas").is_none(), "{seat}");
    assert_eq!(estimate["by_habit"].as_array().unwrap().len(), 2, "{seat}");
    // The seat at base 2 is given the areas of statement 1.
    let at_two = tool_answer(
        &game,
        1,
        Some(Post::At(CommandRef::Base(BaseId::new(2)))),
        json!({}),
    );
    assert_eq!(
        at_two["submarine_estimate"]["areas"]
            .as_array()
            .unwrap()
            .len(),
        4,
        "{at_two}"
    );
}

#[test]
fn a_reading_kept_nowhere_that_drives_nothing() {
    let game = Game {
        allied_habit: true,
        ..Game::default()
    };
    // Statement 12: the game played with the estimate asked every day and
    // the game played without it are the same, hash, layout and log.
    let silent = world_after(&game, 4);
    let held = staffs();
    let mut scenario = game.scenario();
    let mut asked = Vec::new();
    for _ in 0..4 {
        scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves")
            .expect("the day");
        for side in [Side::Allied, Side::Japanese] {
            let view = scenario.world.side_view(side);
            let first = serde_json::to_value(list_contacts(&view, None)).unwrap();
            let second = serde_json::to_value(list_contacts(&view, None)).unwrap();
            // Asking twice gives the same answer.
            assert_eq!(first, second);
            asked.push(first);
        }
    }
    assert_eq!(scenario.world.day_hashes(), silent.day_hashes());
    assert_eq!(scenario.world.state_hash(), silent.state_hash());
    assert_eq!(scenario.world.events().len(), silent.events().len());
    // Statement 13: the game with the habit plays the staff's decisions of
    // the game as filed.
    let filed = world_after(&Game::default(), 4);
    assert_eq!(filed.staff_handed_in(), silent.staff_handed_in());
    let orders = |w: &World| {
        w.events()
            .iter()
            .filter(|e| format!("{:?}", e.kind).contains("Order"))
            .count()
    };
    assert_eq!(orders(&filed), orders(&silent));
}

#[test]
fn nothing_the_side_cannot_see_changes_it() {
    let expected = [
        (1, four_areas(1)),
        (2, four_areas(2)),
        (3, one_area_d()),
        (4, vec![]),
    ];
    let games = [
        Game {
            ninth_boat: true,
            ..Game::default()
        },
        Game {
            salvoes_cut: true,
            ..Game::default()
        },
    ];
    for game in &games {
        for (day, areas) in &expected {
            let world = world_after(game, *day);
            let estimate = estimate_of(&world, Side::Allied);
            assert_eq!(&rows(&estimate), areas, "day {day}");
        }
    }
    // The function takes a picture, the side's own bases and the side's own
    // doctrine: its signature has no world and no umpire's view.
    let _typed: fn(
        &sim_core::view::CommandView<'_>,
        &[(BaseId, Hex)],
        SubmarineDoctrine,
    ) -> SubmarineEstimate = submarine_estimate;
}
