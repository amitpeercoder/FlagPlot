//! Scenario 104: men put ashore score. A side with the objective "men
//! landed" at a base is scored for the men its ships land in that base's
//! hex and do not take off again, for each hundred men.
//!
//! Proves decision 0186. The arithmetic is the architect's, written before
//! the code. If the code disagrees, the code or the rule is wrong: a
//! mismatch goes back to the architect, and nobody else changes a number
//! here.
//!
//! # The sum
//!
//! For each land unit a number that starts at 0. The events are read in
//! order. Men of the unit landed in the base's hex by a ship of the side
//! raise it by those men. Men of the unit embarked by a ship of the side,
//! in any hex, lower it by those men, and not below 0.
//! count = the sum of the units' numbers.
//! points = points for each 100 men x count / 100.
//!
//! # Statements on made-up events (a log built for the test)
//!
//! Base B is in hex H. The Japanese side has the objective men landed at
//! B, 1 point for each 100 men. Ships J1 and J2 are Japanese, ship A1 is
//! Allied. A placeholder setting: no real ship or unit. Unless a
//! statement says otherwise all the men are of one unit, U.
//!
//! 1. **One landing.** J1 lands 350 men in H. Count 350, points
//!    1 x 350 / 100 = 3.5.
//! 2. **Landings add, over ships and days.** J1 lands 350 on day 1 and 600
//!    on day 2; J2 lands 152 and 48 on day 2. Count 350 + 600 + 152 + 48 =
//!    1,150, points 11.5.
//! 3. **A part of 100 scores its part.** J2 lands 48 men and nothing else
//!    is landed. Count 48, points 0.48.
//! 4. **Men taken off again were not landed.** J1 lands 350; later J2
//!    embarks 120 in H. Count 350 - 120 = 230, points 2.3.
//! 5. **Never below nothing.** Nothing is landed; J1 embarks 100 men who
//!    began the game ashore in H. Count 0, points 0, and not -1. J1 then
//!    lands those 100 in H again: count 100, points 1.
//! 6. **Another hex does not count.** J1 lands 350 in a hex beside H and
//!    nothing in H. Count 0.
//! 7. **The other side's ships do not count, either way.** J1 lands 350 in
//!    H; A1 lands 200 in H and embarks 50 there. The Japanese count is
//!    350, points 3.5. An Allied objective of men landed at B at 2 points
//!    for each 100 men counts 200 - 50 = 150, points 2 x 150 / 100 = 3.
//! 8. **Men landed and then lost still count.** J1 lands 350 in H; 200 of
//!    them are then lost in a fight. The Japanese count is 350, points
//!    3.5. An Allied objective of men lost at 1 point for each 100 men
//!    counts those 200, points 2.
//! 9. **The base may be the enemy's, and may change side.** B is Allied
//!    when J1 lands 350 in H, and Japanese at the end. Count 350.
//! 10. **Other points for each 100 men.** With 0.5 points for each 100 men
//!     and statement 2's landings: 0.5 x 1,150 / 100 = 5.75.
//! 11. **Landing the same men twice does not count them twice.** J1 lands
//!     350 of U in H. They march to hex K. J1 embarks the 350 at K: count
//!     350 - 350 = 0. J1 lands them in H again: count 350, and not 700.
//! 12. **Men who march away still count.** J1 lands 350 of U in H and they
//!     march to K and stay ashore. Count 350.
//! 13. **Another unit's men who embark do not lower it.** J1 lands 350 of
//!     U in H. Unit V, 200 men who were never landed from a ship, marches
//!     into H and J2 embarks 120 of V there. Count 350.
//! 14. **Unit by unit.** J1 lands 350 of U and 600 of V in H; J2 embarks
//!     400 of U at K, more than U landed. U's number is 0, not -50; V's is
//!     600. Count 600, points 6.
//! 15. **What the side knows** of its own progress in each of statements 1
//!     to 5 and 11 to 14 is exact, with the same count and points. The other side is
//!     told nothing of the count.
//!
//! # Properties, on logs made at random by the test's own fixed seeds
//!
//! - A further landing in H by the side's ships never lowers the count.
//! - An embarking never raises it.
//! - The points never fall when men are landed and never rise when men are
//!   embarked.
//! - Nothing the other side's ships do changes it.
//! - The count is never below 0 and never above the men the side's ships
//!   landed in H.
//!
//! # Statements on a scenario file
//!
//! 16. **The file's key.** An objective written `kind = "men_landed"`,
//!     `base = 5`, `points_per_100_men = 1` loads as men landed at base 5
//!     at 1 point for each 100 men. One that names a base the scenario
//!     does not have is refused, and so is one with points below 0, each
//!     as supply landed and men lost are refused.
//!
//! # Statements on scenario 103, file a, on its own seed (1942)
//!
//! The game is the one the second version of the orders gives (decision
//! 0184): the convoy lands 350, 350, 600, 152 and 48 men at Taivu, base 5,
//! hex (3,-4), in the night pulse of day 2, and embarks none.
//!
//! 17. **The same game.** The five daily hashes are what they are without
//!     the objective. (Objectives are not in the state and nothing reads
//!     them.) The test takes the hashes from the same file with the
//!     objective taken out, not from a number written here.
//! 18. **The count.** 350 + 350 + 600 + 152 + 48 = 1,500 men, at 1 point
//!     for each 100: 15 points.
//! 19. **The result.** Without the objective the Japanese side has 2.67
//!     points and the Allied side 42.59, an Allied margin of 39.92, a
//!     decisive victory. With it the Japanese side has 2.67 + 15 = 17.67,
//!     the Allied side 42.59 still: an Allied margin of 42.59 - 17.67 =
//!     24.92, a decisive victory. (Restated by the architect on Nov 22,
//!     2026. Before decisions 0211 and 0212 the game on this seed gave 8.87
//!     and 22.69, a margin of 13.82 without the objective and -1.18, a
//!     stalemate, with it.)
//! 20. **What the Japanese side knows** at the end of day 1: 0 men, exact.
//!     At the end of day 2 and after: 1,500 men, 15 points, exact.
//! 21. **The words.** The Japanese briefing has the line "Objective: men
//!     you land from ships at BASE and do not take off again, points for
//!     each hundred men: 1", BASE as that seat names Taivu. The Allied
//!     briefing has, among what the enemy is scored for, "men the enemy
//!     lands from ships at (3,-4) and does not take off again, points for
//!     each hundred men: 1": Taivu is not on the Allied map, so it is
//!     named by its hex and not by a number or a name (decision 0183,
//!     rule 4). The line for supply landed there names it the same way.
//!     Every number in either line is in the briefing's data.
//! 22. **File c and file d,** where the Japanese side is given no orders
//!     and the convoy does not sail: count 0, points 0.
//!
//! # Statements on a scenario for play (decision 0187, rule 22)
//!
//! 23. **The key.** `for_play = true` in a scenario's `[objectives]` table
//!     loads; with the key absent or `false` the scenario is of record.
//!     Scenario 103's four files have it, and scenario 105's four
//!     (decision 0188). No other scenario file has it. (Amended by the
//!     architect on Nov 21, 2026: the files of the same two scenarios with
//!     their oilers, decision 0206, have it too, as do the table's copies
//!     of files for play, decision 0205.)
//!     (Amended by the architect on Nov 22, 2026: scenario 114's four files,
//!     decision 0198, have it too, as do the table's copies of its files b,
//!     c and d.)
//! 24. **It changes no game.** File a's five daily hashes on seed 1942 are
//!     the same with the key as with it taken out.
//! 25. **The result's words.** `sim-cli run` on file a prints its result
//!     under "Result (the scenario's own scale, for play):". On scenario
//!     5, `05_eastern_solomons.toml`, it prints "Result (the placeholder
//!     scale of decision 0054):" as it does today.
//! 26. **The batch file's header** of file a carries `"for_play": true`;
//!     scenario 5's carries `false`.
//! 27. **`sim-cli validate` refuses a scenario for play,** saying that its
//!     results are not for the record, and exits with an error. It runs
//!     scenario 5 as it does today.
//!
//! The tests are written by the build under these statements.

// The tests are written by the build, to the statements above. Each is named
// for its statement. Every game here is on the live figures: no statement
// asserts a number that depends on a figure but the ones the hand
// calculation gives, which the file's own scale and game fix.
//
// What the tests crate can and cannot reach (reported by the build). The
// event log can be added to only inside `sim-core` (`EventLog::record` is
// crate-private), so a log of made-up events cannot be handed to
// `objectives::tallies`. Statements 1 to 15 are therefore tested through
// the two public pieces that read the same events with the same count:
// `MenLandedCount`'s reading in `known_progress` (events given to the side
// as `SideEvent`s seen whole, which a test can build) and `score` on a tally
// holding the count. What only `tallies` does (it picks a ship's side from
// the truth and ignores the other side's ships) cannot be tested on made-up
// events, and those parts are ignored tests that say so.

mod common;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::OnceLock;

use serde_json::{json, Value};
use sim_agent::Session;
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EventId, LandUnitId, ShipId, TaskForceId};
use sim_core::objectives::{
    known_progress, score, GameResult, KnownBasis, KnownProgress, Level, Levels, Objective,
    Objectives, Outcome, SideObjectives, Tally,
};
use sim_core::side_events::{SideEvent, SideEventKind};
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

const A: Side = Side::Allied;
const J: Side = Side::Japanese;

// ------------------------------------------------------------------ helpers

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

/// Base B, Taivu's number in scenario 103.
fn b() -> BaseId {
    BaseId::new(5)
}

/// Hex H, the hex of base B.
fn h() -> Hex {
    Hex::new(3, -4)
}

/// Hex K, beside H.
fn k() -> Hex {
    Hex::new(4, -4)
}

/// Placeholder ship J1 (Japanese), J2 (Japanese) and A1 (Allied).
fn j1() -> ShipId {
    ShipId::new(1)
}

fn j2() -> ShipId {
    ShipId::new(2)
}

fn a1() -> ShipId {
    ShipId::new(3)
}

/// Placeholder land unit U and V.
fn u() -> LandUnitId {
    LandUnitId::new(1)
}

fn v() -> LandUnitId {
    LandUnitId::new(2)
}

/// A made-up log of what one side sees whole: its own ships' events.
#[derive(Default)]
struct Log {
    events: Vec<SideEvent>,
}

impl Log {
    fn push(&mut self, day: u32, kind: EventKind) {
        let number = self.events.len() as u32 + 1;
        self.events.push(SideEvent {
            id: EventId::for_umpire(number, common::umpire_key()),
            number,
            day,
            pulse: None,
            kind: SideEventKind::Whole(kind),
            order_number: None,
            causes: Vec::new(),
            hidden_causes: false,
            breakdown: None,
        });
    }

    fn landed(&mut self, day: u32, ship: ShipId, unit: LandUnitId, hex: Hex, men: u32) {
        self.push(
            day,
            EventKind::MenLanded {
                unit,
                ship,
                task_force: TaskForceId::new(1),
                hex,
                men,
                aboard: 0,
                ashore: men,
            },
        );
    }

    fn embarked(&mut self, day: u32, ship: ShipId, unit: LandUnitId, hex: Hex, men: u32) {
        self.push(
            day,
            EventKind::MenEmbarked {
                unit,
                ship,
                task_force: TaskForceId::new(1),
                hex,
                men,
                aboard: men,
                ashore: 0,
            },
        );
    }

    fn lost_in_fight(&mut self, day: u32, unit: LandUnitId, hex: Hex, lost: u32) {
        self.push(
            day,
            EventKind::MenLostInFight {
                unit,
                hex,
                lost,
                ashore: 0,
            },
        );
    }
}

fn men_landed_objective(points: &str) -> SideObjectives {
    SideObjectives {
        mission: "Placeholder mission.".to_string(),
        objectives: vec![Objective::MenLanded {
            base: b(),
            points_per_100_men: fx(points),
        }],
    }
}

/// What the side knows of the objective, from the events, with B in H.
fn known(log: &Log, points: &str, own_bases: &[BaseId]) -> KnownProgress {
    let hexes = BTreeMap::from([(b(), h())]);
    let mut all = known_progress(
        &men_landed_objective(points),
        own_bases,
        &hexes,
        &log.events,
    );
    assert_eq!(all.len(), 1);
    all.remove(0)
}

/// The result `score` gives for a tally that holds `count` men landed at B
/// for the side, the other side having nothing: the side's line.
fn scored(side: Side, count: u32, points: &str) -> (Fixed, Fixed) {
    let empty = || SideObjectives {
        mission: "Placeholder mission.".to_string(),
        objectives: Vec::new(),
    };
    let (allied, japanese) = match side {
        A => (men_landed_objective(points), empty()),
        J => (empty(), men_landed_objective(points)),
    };
    let objectives = Objectives {
        allied,
        japanese,
        levels: Levels::new(n(3), n(8), n(16)).expect("levels"),
        horizon_days: 1,
        history: None,
        for_play: false,
        base_hexes: BTreeMap::from([(b(), h())]),
    };
    let mut tally = Tally::default();
    if count > 0 {
        tally.men_landed.insert(b(), count);
    }
    let result = score(&objectives, &BTreeMap::from([(side, tally)]));
    let line = &result.side(side).objectives[0];
    assert_eq!(line.kind, "men_landed");
    (line.count, line.points)
}

/// The count and points the log gives, checked as exact and checked against
/// the points `score` gives for that count.
fn counted(log: &Log, points: &str) -> (Fixed, Fixed) {
    let known = known(log, points, &[b()]);
    assert_eq!(known.basis, KnownBasis::Exact);
    assert_eq!(known.base, Some(b()));
    let points_known = known.points.expect("points of an exact count");
    let count = known
        .count
        .to_string()
        .parse::<i64>()
        .expect("a whole count");
    let (score_count, score_points) = scored(J, count as u32, points);
    assert_eq!(score_count, known.count);
    assert_eq!(score_points, points_known);
    (known.count, points_known)
}

/// Statement 2's landings.
fn statement_2_log() -> Log {
    let mut log = Log::default();
    log.landed(1, j1(), u(), h(), 350);
    log.landed(2, j1(), u(), h(), 600);
    log.landed(2, j2(), u(), h(), 152);
    log.landed(2, j2(), u(), h(), 48);
    log
}

// ------------------------------------------- Statements on made-up events

#[test]
fn statement_01_one_landing() {
    let mut log = Log::default();
    log.landed(1, j1(), u(), h(), 350);
    assert_eq!(counted(&log, "1"), (n(350), fx("3.5")));
}

#[test]
fn statement_02_landings_add_over_ships_and_days() {
    assert_eq!(counted(&statement_2_log(), "1"), (n(1150), fx("11.5")));
}

#[test]
fn statement_03_a_part_of_100_scores_its_part() {
    let mut log = Log::default();
    log.landed(1, j2(), u(), h(), 48);
    assert_eq!(counted(&log, "1"), (n(48), fx("0.48")));
}

#[test]
fn statement_04_men_taken_off_again_were_not_landed() {
    let mut log = Log::default();
    log.landed(1, j1(), u(), h(), 350);
    log.embarked(1, j2(), u(), h(), 120);
    assert_eq!(counted(&log, "1"), (n(230), fx("2.3")));
}

#[test]
fn statement_05_never_below_nothing() {
    // The 100 men began ashore in H: nothing was landed, then J1 embarks
    // them.
    let mut log = Log::default();
    log.embarked(1, j1(), u(), h(), 100);
    assert_eq!(counted(&log, "1"), (n(0), n(0)));
    // J1 then lands those 100 in H again.
    log.landed(2, j1(), u(), h(), 100);
    assert_eq!(counted(&log, "1"), (n(100), n(1)));
}

#[test]
fn statement_06_another_hex_does_not_count() {
    let mut log = Log::default();
    log.landed(1, j1(), u(), k(), 350);
    assert_eq!(counted(&log, "1").0, n(0));
}

/// What can be tested here of the statement: each side's objective counts
/// the events of its own ships, given to it as it would see them. That
/// `tallies` keeps one side's ships out of the other's count is the ignored
/// test below.
#[test]
fn statement_07_the_other_sides_ships_do_not_count_either_way() {
    let mut japanese = Log::default();
    japanese.landed(1, j1(), u(), h(), 350);
    assert_eq!(counted(&japanese, "1"), (n(350), fx("3.5")));
    // The Allied objective, 2 points for each 100 men, over A1's events.
    let mut allied = Log::default();
    allied.landed(1, a1(), v(), h(), 200);
    allied.embarked(1, a1(), v(), h(), 50);
    assert_eq!(counted(&allied, "2"), (n(150), n(3)));
}

#[test]
#[ignore = "the tests crate cannot make an event log (EventLog::record is crate-private), so `tallies` cannot be given J1's, J2's and A1's events together: that a side's count ignores the other side's ships is not testable here; sim-core's own unit test men_landed_are_counted_for_the_side_of_the_ship_in_the_base_hex_alone covers it through tallies"]
fn statement_07_through_tallies_the_other_sides_ships_do_not_count() {
    todo!("a log with J1 landing 350 in H and A1 landing 200 and embarking 50 there, read by tallies: Japanese 350, Allied 150")
}

#[test]
fn statement_08_men_landed_and_then_lost_still_count() {
    let mut log = Log::default();
    log.landed(1, j1(), u(), h(), 350);
    log.lost_in_fight(2, u(), h(), 200);
    assert_eq!(counted(&log, "1"), (n(350), fx("3.5")));
    // An Allied objective of men lost at 1 point for each 100 men counts
    // those 200: the score of a tally with 200 enemy men lost.
    let objectives = Objectives {
        allied: SideObjectives {
            mission: "Placeholder mission.".to_string(),
            objectives: vec![Objective::MenLost {
                points_per_100_men: n(1),
            }],
        },
        japanese: men_landed_objective("1"),
        levels: Levels::new(n(3), n(8), n(16)).expect("levels"),
        horizon_days: 1,
        history: None,
        for_play: false,
        base_hexes: BTreeMap::from([(b(), h())]),
    };
    let allied = Tally {
        men_lost: 200,
        ..Tally::default()
    };
    let japanese = Tally {
        men_landed: BTreeMap::from([(b(), 350)]),
        ..Tally::default()
    };
    let result = score(&objectives, &BTreeMap::from([(A, allied), (J, japanese)]));
    assert_eq!(result.allied.objectives[0].count, n(200));
    assert_eq!(result.allied.objectives[0].points, n(2));
    assert_eq!(result.japanese.objectives[0].count, n(350));
    assert_eq!(result.japanese.objectives[0].points, fx("3.5"));
}

#[test]
fn statement_09_the_base_may_be_the_enemys_and_may_change_side() {
    let mut log = Log::default();
    log.landed(1, j1(), u(), h(), 350);
    // B is Allied when the men land and Japanese at the end: the count is
    // the same whichever side holds it.
    for own in [&[][..], &[b()][..]] {
        let known = known(&log, "1", own);
        assert_eq!((known.count, known.points), (n(350), Some(fx("3.5"))));
    }
    assert_eq!(counted(&log, "1").0, n(350));
}

#[test]
fn statement_10_other_points_for_each_100_men() {
    assert_eq!(counted(&statement_2_log(), "0.5"), (n(1150), fx("5.75")));
}

#[test]
fn statement_11_landing_the_same_men_twice_does_not_count_them_twice() {
    let mut log = Log::default();
    log.landed(1, j1(), u(), h(), 350);
    // They march to K; J1 embarks the 350 at K.
    log.embarked(2, j1(), u(), k(), 350);
    assert_eq!(counted(&log, "1").0, n(0));
    // J1 lands them in H again.
    log.landed(3, j1(), u(), h(), 350);
    assert_eq!(counted(&log, "1").0, n(350));
}

#[test]
fn statement_12_men_who_march_away_still_count() {
    // The march is not an event of the sea: nothing is embarked.
    let mut log = Log::default();
    log.landed(1, j1(), u(), h(), 350);
    assert_eq!(counted(&log, "1").0, n(350));
}

#[test]
fn statement_13_another_units_men_who_embark_do_not_lower_it() {
    let mut log = Log::default();
    log.landed(1, j1(), u(), h(), 350);
    // V, 200 men never landed from a ship, marches into H; J2 embarks 120
    // of V there.
    log.embarked(2, j2(), v(), h(), 120);
    assert_eq!(counted(&log, "1").0, n(350));
}

#[test]
fn statement_14_unit_by_unit() {
    let mut log = Log::default();
    log.landed(1, j1(), u(), h(), 350);
    log.landed(1, j1(), v(), h(), 600);
    // J2 embarks 400 of U at K, more than U landed.
    log.embarked(2, j2(), u(), k(), 400);
    assert_eq!(counted(&log, "1"), (n(600), n(6)));
}

/// What the side knows of its own progress in statements 1 to 5 and 11 to
/// 14 is exact, with the same count and points; the enemy's objective line
/// has no count (the briefing's data shows it, below).
#[test]
fn statement_15_what_the_side_knows_is_exact_with_the_same_count_and_points() {
    type Make = fn(&mut Log);
    let cases: Vec<(&str, Make, i64, &str)> = vec![
        ("1", |l| l.landed(1, j1(), u(), h(), 350), 350, "3.5"),
        ("2", |l| *l = statement_2_log(), 1150, "11.5"),
        ("3", |l| l.landed(1, j2(), u(), h(), 48), 48, "0.48"),
        (
            "4",
            |l| {
                l.landed(1, j1(), u(), h(), 350);
                l.embarked(1, j2(), u(), h(), 120);
            },
            230,
            "2.3",
        ),
        (
            "5",
            |l| {
                l.embarked(1, j1(), u(), h(), 100);
                l.landed(2, j1(), u(), h(), 100);
            },
            100,
            "1",
        ),
        (
            "11",
            |l| {
                l.landed(1, j1(), u(), h(), 350);
                l.embarked(2, j1(), u(), k(), 350);
                l.landed(3, j1(), u(), h(), 350);
            },
            350,
            "3.5",
        ),
        ("12", |l| l.landed(1, j1(), u(), h(), 350), 350, "3.5"),
        (
            "13",
            |l| {
                l.landed(1, j1(), u(), h(), 350);
                l.embarked(2, j2(), v(), h(), 120);
            },
            350,
            "3.5",
        ),
        (
            "14",
            |l| {
                l.landed(1, j1(), u(), h(), 350);
                l.landed(1, j1(), v(), h(), 600);
                l.embarked(2, j2(), u(), k(), 400);
            },
            600,
            "6",
        ),
    ];
    for (which, make, men, points) in cases {
        let mut log = Log::default();
        make(&mut log);
        let known = known(&log, "1", &[b()]);
        assert_eq!(known.basis, KnownBasis::Exact, "statement {which}");
        assert_eq!(known.count, n(men), "statement {which}");
        assert_eq!(known.points, Some(fx(points)), "statement {which}");
        // The umpire's count for the same men is the same.
        assert_eq!(
            scored(J, men as u32, "1"),
            (n(men), fx(points)),
            "statement {which}"
        );
    }
    // The other side is told nothing of the count: the enemy's line in the
    // briefing has the kind, the base, and the worth, and no count or
    // known part.
    let mut s = scripted('a');
    let briefing = call(&mut s, A, "get_briefing", json!({"text": false}));
    let enemy = briefing["objectives"]["enemy_objectives"]
        .as_array()
        .expect("the enemy's objectives are listed");
    let line = enemy
        .iter()
        .find(|l| l["kind"] == "men_landed")
        .expect("the enemy's men landed objective is listed");
    for key in line.as_object().expect("an object").keys() {
        assert!(
            ["kind", "base", "closed", "ship_points", "points"].contains(&key.as_str()),
            "the enemy's line has {key}"
        );
    }
    assert!(line.get("known").is_none() && line.get("count").is_none());
}

// ------------------------------------------------------------- Properties

/// A small deterministic generator (xorshift64), the test's own.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
}

/// Logs of the side's own ships' events at random, one event added at a
/// time, checking after each addition.
fn each_step(mut check: impl FnMut(&Log, Option<(bool, bool)>, Fixed, Fixed, Fixed, u32)) {
    for seed in 1..=300u64 {
        let mut rng = Rng(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
        let mut log = Log::default();
        let (mut count, mut points) = (n(0), n(0));
        let mut landed_in_h = 0u32;
        for step in 0..40 {
            let unit = LandUnitId::new(1 + rng.below(3) as u32);
            let men = 1 + rng.below(500) as u32;
            let hex = if rng.below(2) == 0 { h() } else { k() };
            let ship = if rng.below(2) == 0 { j1() } else { j2() };
            let landing = rng.below(3) != 0;
            let day = 1 + step / 10;
            if landing {
                log.landed(day, ship, unit, hex, men);
            } else {
                log.embarked(day, ship, unit, hex, men);
            }
            let in_h = landing && hex == h();
            if in_h {
                landed_in_h += men;
            }
            let (new_count, new_points) = counted(&log, "1");
            // Points are the count over 100 at 1 for each 100.
            assert_eq!(new_points, new_count.mul_div(n(1), n(100)));
            let kind = Some((landing, in_h));
            check(
                &log,
                kind,
                count,
                new_count,
                new_points - points,
                landed_in_h,
            );
            count = new_count;
            points = new_points;
        }
    }
}

#[test]
fn property_a_landing_in_h_never_lowers_the_count() {
    each_step(|_, kind, before, after, _, _| {
        if matches!(kind, Some((true, true))) {
            assert!(after >= before, "{before} then {after}");
        }
    });
}

#[test]
fn property_an_embarking_never_raises_the_count() {
    each_step(|_, kind, before, after, _, _| {
        if matches!(kind, Some((false, _))) {
            assert!(after <= before, "{before} then {after}");
        }
    });
}

#[test]
fn property_the_points_never_fall_when_men_are_landed_and_never_rise_when_embarked() {
    each_step(|_, kind, _, _, change, _| match kind {
        Some((true, _)) => assert!(change >= Fixed::ZERO, "{change}"),
        Some((false, _)) => assert!(change <= Fixed::ZERO, "{change}"),
        None => {}
    });
}

#[test]
#[ignore = "the tests crate cannot make an event log, so `tallies` cannot be given the other side's ships' events: that nothing they do changes a side's count is not testable here (what a side is given to count is its own ships' events, which every property above uses); sim-core's own unit test men_taken_off_lower_the_count_unit_by_unit_and_never_below_nothing covers it through tallies"]
fn property_nothing_the_other_sides_ships_do_changes_it() {
    todo!("random logs with the other side's landings and embarkings added, read by tallies: the side's count and points are as they were")
}

#[test]
fn property_the_count_is_never_below_0_and_never_above_the_men_landed_in_h() {
    each_step(|_, _, _, after, _, landed_in_h| {
        assert!(after >= Fixed::ZERO, "{after}");
        assert!(
            after <= n(i64::from(landed_in_h)),
            "{after} of {landed_in_h}"
        );
    });
}

// ------------------------------------------- Statements on a scenario file

fn five_days(letter: char) -> PathBuf {
    common::data_dir().join(format!(
        "scenarios/five-days/01{letter}_eastern_solomons.toml"
    ))
}

fn five_days_text(letter: char) -> String {
    std::fs::read_to_string(five_days(letter)).expect("the scenario file reads")
}

/// The Japanese men landed objective's text in the five-days files.
const MEN_LANDED_BLOCK: &str =
    "[[objectives.japanese.objective]]\nkind = \"men_landed\"\nbase = 5\npoints_per_100_men = 1\n";

fn load_text(text: &str, letter: char) -> Result<Scenario, sim_data::DataError> {
    Scenario::from_text(text, &five_days(letter))
}

fn refused(text: &str) -> String {
    match load_text(text, 'a') {
        Ok(_) => panic!("the scenario loaded, and was to be refused"),
        Err(error) => error.to_string(),
    }
}

#[test]
fn statement_16_the_files_key() {
    let text = five_days_text('a');
    let scenario = load_text(&text, 'a').expect("the file loads");
    let japanese = &scenario
        .world
        .objectives()
        .expect("objectives")
        .japanese
        .objectives;
    assert_eq!(
        japanese.last(),
        Some(&Objective::MenLanded {
            base: BaseId::new(5),
            points_per_100_men: n(1),
        })
    );
    // A base the scenario does not have.
    let message = refused(&common::replace_once(
        &text,
        MEN_LANDED_BLOCK,
        &MEN_LANDED_BLOCK.replace("base = 5", "base = 77"),
    ));
    assert!(message.contains("unknown base:77"), "{message}");
    // Points below 0.
    let message = refused(&common::replace_once(
        &text,
        MEN_LANDED_BLOCK,
        &MEN_LANDED_BLOCK.replace("points_per_100_men = 1", "points_per_100_men = -1"),
    ));
    assert!(message.contains("negative"), "{message}");
    // Each as supply landed and men lost are refused.
    let cargo = "kind = \"cargo_landed\"\nbase = 5\npoints_per_t = \"0.005\"\n";
    let message = refused(&common::replace_once(
        &text,
        cargo,
        &cargo.replace("base = 5", "base = 77"),
    ));
    assert!(message.contains("unknown base:77"), "{message}");
    let message = refused(&common::replace_once(
        &text,
        cargo,
        &cargo.replace("\"0.005\"", "\"-0.005\""),
    ));
    assert!(message.contains("negative"), "{message}");
    let lost = "kind = \"men_lost\"\npoints_per_100_men = 1\n";
    let message = refused(&text.replacen(lost, &lost.replace("= 1", "= -1"), 1));
    assert!(message.contains("negative"), "{message}");
}

// ------------------------------------------ Statements on scenario 103 a

/// A game played to its end by the doctrine's staffs, as `batch` plays one.
struct Game {
    hashes: Vec<u64>,
    result: GameResult,
    /// Each man landing and embarking of the game: (day, pulse, side of the
    /// ship, hex, men, is a landing).
    men: Vec<(u32, Option<Pulse>, Side, Hex, u32, bool)>,
}

fn play_text(text: &str, letter: char) -> Game {
    let mut scenario = load_text(text, letter).expect("the scenario loads");
    let held = staffs();
    let mut hashes = Vec::new();
    while let Some(summary) = scenario
        .next_day_watched(as_staffs(&held), &mut |_, _| {})
        .expect("a day resolves")
    {
        hashes.push(summary.state_hash);
    }
    let world = &scenario.world;
    let mut men = Vec::new();
    for event in world.events().iter() {
        match event.kind {
            EventKind::MenLanded {
                ship, hex, men: m, ..
            } => {
                let side = world.umpire().ships[&ship].side;
                men.push((event.day, event.pulse, side, hex, m, true));
            }
            EventKind::MenEmbarked {
                ship, hex, men: m, ..
            } => {
                let side = world.umpire().ships[&ship].side;
                men.push((event.day, event.pulse, side, hex, m, false));
            }
            _ => {}
        }
    }
    Game {
        hashes,
        result: world.result().expect("a result"),
        men,
    }
}

/// File a as it is, on seed 1942.
fn game_a() -> &'static Game {
    static GAME: OnceLock<Game> = OnceLock::new();
    GAME.get_or_init(|| play_text(&five_days_text('a'), 'a'))
}

/// File a with the objective taken out.
fn game_a_without_the_objective() -> &'static Game {
    static GAME: OnceLock<Game> = OnceLock::new();
    GAME.get_or_init(|| {
        let text = common::replace_once(&five_days_text('a'), MEN_LANDED_BLOCK, "");
        assert!(!text.contains("men_landed\"\nbase"));
        play_text(&text, 'a')
    })
}

/// File a with the key taken out.
fn game_a_without_the_key() -> &'static Game {
    static GAME: OnceLock<Game> = OnceLock::new();
    GAME.get_or_init(|| {
        let text = common::replace_once(
            &five_days_text('a'),
            "[objectives]\nfor_play = true\n",
            "[objectives]\n",
        );
        play_text(&text, 'a')
    })
}

fn game_of(letter: char) -> &'static Game {
    static C: OnceLock<Game> = OnceLock::new();
    static D: OnceLock<Game> = OnceLock::new();
    match letter {
        'c' => C.get_or_init(|| play_text(&five_days_text('c'), 'c')),
        'd' => D.get_or_init(|| play_text(&five_days_text('d'), 'd')),
        _ => unreachable!("file c or d"),
    }
}

fn men_landed_line(game: &Game) -> (Fixed, Fixed) {
    let line = game
        .result
        .japanese
        .objectives
        .iter()
        .find(|o| o.kind == "men_landed")
        .expect("the Japanese side's men landed objective");
    (line.count, line.points)
}

#[test]
fn statement_17_the_same_game() {
    let with = game_a();
    let without = game_a_without_the_objective();
    assert_eq!(with.hashes.len(), 5);
    assert_eq!(with.hashes, without.hashes);
    // The objective is in the one result and not the other.
    assert!(without
        .result
        .japanese
        .objectives
        .iter()
        .all(|o| o.kind != "men_landed"));
}

#[test]
fn statement_18_the_count() {
    // The convoy lands 350, 350, 600, 152 and 48 men at Taivu in the night
    // pulse of day 2, and embarks none.
    let game = game_a();
    let japanese: Vec<_> = game.men.iter().filter(|m| m.2 == J).collect();
    assert!(japanese.iter().all(|m| m.5), "a Japanese man embarked");
    let mut landed: Vec<u32> = japanese.iter().map(|m| m.4).collect();
    landed.sort();
    assert_eq!(landed, [48, 152, 350, 350, 600]);
    for m in &japanese {
        assert_eq!((m.0, m.1, m.3), (2, Some(Pulse::Night), h()), "{m:?}");
    }
    assert_eq!(landed.iter().sum::<u32>(), 1500);
    assert_eq!(men_landed_line(game), (n(1500), n(15)));
}

#[test]
fn statement_19_the_result() {
    let without = &game_a_without_the_objective().result;
    assert_eq!(without.japanese.total, fx("2.67"));
    assert_eq!(without.allied.total, fx("42.59"));
    assert_eq!(without.allied.margin, fx("39.92"));
    assert_eq!(without.allied.outcome, Outcome::Victory(Level::Decisive));
    let with = &game_a().result;
    assert_eq!(with.japanese.total, fx("17.67"));
    assert_eq!(with.allied.total, fx("42.59"));
    assert_eq!(with.allied.margin, fx("24.92"));
    assert_eq!(with.allied.outcome, Outcome::Victory(Level::Decisive));
    assert_eq!(with.japanese.outcome, Outcome::Defeat(Level::Decisive));
}

// --- The briefing's reading of file a (statements 20 and 21)

fn path_text(path: &Path) -> String {
    path.to_str().expect("a path in words").to_string()
}

/// A game opened as `serve --watched` opens one, both seats the script's.
fn scripted(letter: char) -> Session {
    sim_cli::game::open(
        &path_text(&five_days(letter)),
        None,
        BTreeMap::new(),
        &BTreeMap::new(),
    )
    .expect("the game opens")
    .0
    .watched()
}

fn call(s: &mut Session, side: Side, tool: &str, args: Value) -> Value {
    s.watch(side, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} at {side:?}: {}", e.message))
}

/// The watcher steps until `day` is resolved and the clock stands at dusk.
fn to_day(s: &mut Session, day: u32) {
    while s.day() < day || s.open_day().is_some() {
        let stepped = s.step();
        stepped
            .answer
            .unwrap_or_else(|e| panic!("the step is refused: {}", e.message));
    }
}

/// The objective line of `kind` among the reader's own.
fn own_line<'a>(data: &'a Value, kind: &str) -> &'a Value {
    data["objectives"]["objectives"]
        .as_array()
        .expect("the side's objectives are listed")
        .iter()
        .find(|l| l["kind"] == kind)
        .unwrap_or_else(|| panic!("an own objective of {kind}"))
}

fn dec(value: &Value) -> Fixed {
    value
        .as_str()
        .expect("a decimal in words")
        .parse()
        .expect("a decimal")
}

#[test]
fn statement_20_what_the_japanese_side_knows() {
    let mut s = scripted('a');
    to_day(&mut s, 1);
    let data = call(&mut s, J, "get_briefing", json!({"text": false}));
    let known = &own_line(&data, "men_landed")["known"];
    assert_eq!(known["basis"], "exact", "{known}");
    assert_eq!(dec(&known["count"]), n(0), "{known}");
    assert_eq!(dec(&known["points"]), n(0), "{known}");
    for day in 2..=5 {
        to_day(&mut s, day);
        let data = call(&mut s, J, "get_briefing", json!({"text": false}));
        let known = &own_line(&data, "men_landed")["known"];
        assert_eq!(known["basis"], "exact", "day {day}: {known}");
        assert_eq!(dec(&known["count"]), n(1500), "day {day}: {known}");
        assert_eq!(dec(&known["points"]), n(15), "day {day}: {known}");
    }
}

/// The lines of the text's OBJECTIVES part.
fn objectives_lines(text: &str) -> Vec<&str> {
    text.lines()
        .skip_while(|l| *l != "OBJECTIVES")
        .skip(1)
        .take_while(|l| !l.is_empty())
        .collect()
}

#[test]
fn statement_21_the_words() {
    let mut s = scripted('a');
    // Japanese briefing.
    let data = call(&mut s, J, "get_briefing", json!({"text": false}));
    let text = call(&mut s, J, "get_briefing", json!({"text": true}));
    let text = text["text"].as_str().expect("the text");
    let own = objectives_lines(text);
    let line = own
        .iter()
        .find(|l| l.starts_with("- Objective: men you land"))
        .unwrap_or_else(|| panic!("the Japanese line: {own:#?}"));
    let prefix = "- Objective: men you land from ships at ";
    let suffix = " and do not take off again, points for each hundred men: 1";
    assert!(line.starts_with(prefix) && line.ends_with(suffix), "{line}");
    let base = &line[prefix.len()..line.len() - suffix.len()];
    // BASE is as this seat names Taivu: as its supply objective does.
    let supply = own
        .iter()
        .find(|l| l.contains("supply and fuel you unload at "))
        .unwrap_or_else(|| panic!("the supply line: {own:#?}"));
    assert!(
        supply.contains(&format!("unload at {base} and")),
        "{base}: {supply}"
    );
    // Every number in the line is in the data.
    let own_data = own_line(&data, "men_landed");
    assert_eq!(dec(&own_data["points"]), n(1));
    assert_eq!(own_data["base"]["handle"], "base:5");
    assert!(base.contains("base:5"), "{base}");

    // Allied briefing: Taivu is not on its map, so it is named by its hex.
    let data = call(&mut s, A, "get_briefing", json!({"text": false}));
    let text = call(&mut s, A, "get_briefing", json!({"text": true}));
    let text = text["text"].as_str().expect("the text");
    let allied = objectives_lines(text);
    let scored_for = allied
        .iter()
        .position(|l| l.starts_with("- The enemy is scored for:"))
        .expect("the enemy's objectives are listed");
    let block: Vec<&&str> = allied[scored_for + 1..]
        .iter()
        .take_while(|l| l.starts_with("  - "))
        .collect();
    let men = block
        .iter()
        .find(|l| l.contains("men the enemy lands from ships"))
        .unwrap_or_else(|| panic!("the enemy's men landed line: {block:#?}"));
    assert!(
        men.contains("men the enemy lands from ships at (3,-4) and does not take off again, points for each hundred men: 1"),
        "{men}"
    );
    let supply = block
        .iter()
        .find(|l| l.contains("supply and fuel the enemy unloads at "))
        .unwrap_or_else(|| panic!("the enemy's supply line: {block:#?}"));
    assert!(supply.contains("unloads at (3,-4) and"), "{supply}");
    for line in [men, supply] {
        assert!(
            !line.contains("base:5") && !line.contains("Taivu"),
            "{line}"
        );
        assert!(!line.contains("Placeholder Japanese stock"), "{line}");
    }
    // Every number in either line is in the data.
    let enemy = data["objectives"]["enemy_objectives"]
        .as_array()
        .expect("the enemy's objectives");
    for kind in ["men_landed", "cargo_landed"] {
        let line = enemy
            .iter()
            .find(|l| l["kind"] == kind)
            .unwrap_or_else(|| panic!("{kind} in the data"));
        assert_eq!(line["base"]["handle"], "(3,-4)", "{line}");
        assert!(line["base"]["name"].is_null(), "{line}");
    }
    let men_data = enemy.iter().find(|l| l["kind"] == "men_landed").unwrap();
    assert_eq!(dec(&men_data["points"]), n(1));
}

#[test]
fn statement_22_file_c_and_file_d() {
    for letter in ['c', 'd'] {
        assert_eq!(
            men_landed_line(game_of(letter)),
            (n(0), n(0)),
            "file {letter}"
        );
    }
}

// ------------------------------- Statements on a scenario for play (0187)

fn all_scenario_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .expect("the folder reads")
        .map(|e| e.expect("an entry").path())
        .collect();
    entries.sort();
    for path in entries {
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if path.is_dir() {
            if !name.starts_with("tmp-") {
                all_scenario_files(&path, out);
            }
        } else if name.ends_with(".toml") {
            out.push(path);
        }
    }
}

/// Statement 23 as built with decision 0205: the eight files have the key,
/// and so do the table's copies of six of them (files b, c and d of each) in
/// data/scenarios/table-hunger, each its source but for the line that names
/// the ground figures (decision 0205, 3). No other scenario file has it.
#[test]
fn statement_23_the_key() {
    // Loads with the key; absent or false, of record.
    let text = five_days_text('a');
    let with = load_text(&text, 'a').expect("loads");
    assert!(with.world.objectives().expect("objectives").for_play);
    let absent = common::replace_once(&text, "[objectives]\nfor_play = true\n", "[objectives]\n");
    let absent = load_text(&absent, 'a').expect("loads");
    assert!(!absent.world.objectives().expect("objectives").for_play);
    let false_ = common::replace_once(
        &text,
        "[objectives]\nfor_play = true\n",
        "[objectives]\nfor_play = false\n",
    );
    let false_ = load_text(&false_, 'a').expect("loads");
    assert!(!false_.world.objectives().expect("objectives").for_play);
    // Scenario 103's four files have it, and load.
    for letter in ['a', 'b', 'c', 'd'] {
        let scenario = common::load_live(&five_days(letter)).expect("loads");
        assert!(
            scenario.world.objectives().expect("objectives").for_play,
            "file {letter}"
        );
    }
    // No other scenario file has it: every file under data/scenarios (the
    // tmp- folders are someone's trials) is read as text for a line that
    // writes the key. Scenario 5 is loaded as well.
    let mut files = Vec::new();
    all_scenario_files(&common::data_dir().join("scenarios"), &mut files);
    assert!(files.len() > 100, "{} files", files.len());
    let mut with_key = Vec::new();
    // The one file left out is the script's base, which is not played itself
    // ("base_eastern_solomons.toml", the text the four files are made from)
    // and carries the key as they do: it is not a scenario file in the
    // statement's sense (reported by the build).
    for file in files.iter().filter(|f| {
        let name = f.file_name().and_then(|n| n.to_str());
        name != Some("base_eastern_solomons.toml")
            && name != Some("base_coral_sea.toml")
            && name != Some("base_eastern_solomons_oilers.toml")
            && name != Some("base_coral_sea_oilers.toml")
            && name != Some("base_coral_sea_both_oilers.toml")
            && name != Some("base_midway.toml")
    }) {
        let text = std::fs::read_to_string(file).expect("reads");
        if text.lines().any(|l| l.trim_start().starts_with("for_play")) {
            with_key.push(file.strip_prefix(common::data_dir()).unwrap().to_owned());
        }
    }
    let mut want: Vec<PathBuf> = ['a', 'b', 'c', 'd']
        .iter()
        .flat_map(|l| {
            [
                PathBuf::from(format!("scenarios/coral-sea/01{l}_coral_sea.toml")),
                PathBuf::from(format!("scenarios/five-days/01{l}_eastern_solomons.toml")),
                // The same two scenarios with their oilers (decision 0206).
                PathBuf::from(format!("scenarios/coral-sea/02{l}_coral_sea_oilers.toml")),
                PathBuf::from(format!(
                    "scenarios/five-days/02{l}_eastern_solomons_oilers.toml"
                )),
                // The Coral Sea with both its oilers (decision 0207).
                PathBuf::from(format!(
                    "scenarios/coral-sea/03{l}_coral_sea_both_oilers.toml"
                )),
                // Midway (decision 0198).
                PathBuf::from(format!("scenarios/midway/01{l}_midway.toml")),
            ]
        })
        .collect();
    // The table's copies (decision 0205, 3).
    for l in ['b', 'c', 'd'] {
        want.push(PathBuf::from(format!(
            "scenarios/table-hunger/03{l}_coral_sea_both_oilers.toml"
        )));
        want.push(PathBuf::from(format!(
            "scenarios/table-hunger/02{l}_eastern_solomons_oilers.toml"
        )));
    }
    for l in ['b', 'c', 'd'] {
        want.push(PathBuf::from(format!(
            "scenarios/table-hunger/01{l}_midway.toml"
        )));
    }
    want.sort();
    with_key.sort();
    assert_eq!(with_key, want);
    let five = common::load_live(&common::data_dir().join("scenarios/05_eastern_solomons.toml"))
        .expect("scenario 5 loads");
    assert!(!five.world.objectives().expect("objectives").for_play);
}

#[test]
fn statement_24_it_changes_no_game() {
    let with = game_a();
    let without = game_a_without_the_key();
    assert_eq!(with.hashes.len(), 5);
    assert_eq!(with.hashes, without.hashes);
}

// --- sim-cli, run as a program (statements 25 to 27)

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const FILE_A: &str = "data/scenarios/five-days/01a_eastern_solomons.toml";
const SCENARIO_5: &str = "data/scenarios/05_eastern_solomons.toml";

/// The `sim-cli` program. The tests crate holds `sim-cli` as a library, so
/// cargo does not build the program for it and gives no path to it: it is
/// built here, once, in the profile this test runs in, in the same target
/// folder.
fn sim_cli() -> &'static PathBuf {
    static BINARY: OnceLock<PathBuf> = OnceLock::new();
    BINARY.get_or_init(|| {
        let exe = std::env::current_exe().expect("the test's path");
        let release = exe.components().any(|c| c.as_os_str() == "release");
        let target = std::env::var("CARGO_TARGET_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(ROOT).join("target"));
        let mut build = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()));
        build
            .args(["build", "--quiet", "-p", "sim-cli"])
            .current_dir(ROOT);
        if release {
            build.arg("--release");
        }
        let built = build.output().expect("cargo starts");
        assert!(
            built.status.success(),
            "sim-cli did not build: {}",
            String::from_utf8_lossy(&built.stderr)
        );
        target
            .join(if release { "release" } else { "debug" })
            .join("sim-cli")
    })
}

fn binary(args: &[&str]) -> Output {
    Command::new(sim_cli())
        .args(args)
        .current_dir(ROOT)
        .output()
        .expect("the program starts")
}

fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

#[test]
fn statement_25_the_results_words() {
    let result = binary(&["run", FILE_A]);
    assert!(result.status.success(), "{}", stderr(&result));
    let text = stdout(&result);
    assert!(
        text.contains("Result (the scenario's own scale, for play):"),
        "{text}"
    );
    assert!(!text.contains("placeholder scale of decision 0054"));
    let result = binary(&["run", SCENARIO_5]);
    assert!(result.status.success(), "{}", stderr(&result));
    let text = stdout(&result);
    assert!(
        text.contains("Result (the placeholder scale of decision 0054):"),
        "{text}"
    );
    assert!(!text.contains("for play"));
}

/// A batch of `runs` games of `scenario`, written to a file of its own.
fn batch_of(scenario: &str, name: &str, runs: &str) -> (PathBuf, Value) {
    let out = std::env::temp_dir().join(format!(
        "flagplot-scenario-104-{}-{name}.json",
        std::process::id()
    ));
    let result = binary(&[
        "batch",
        scenario,
        "--runs",
        runs,
        "--label",
        "scenario-104-test",
        "--out",
        &out.to_string_lossy(),
    ]);
    assert!(result.status.success(), "{}", stderr(&result));
    let value = serde_json::from_slice(&std::fs::read(&out).expect("batch file")).expect("JSON");
    (out, value)
}

#[test]
fn statement_26_the_batch_files_header() {
    let (path, header) = batch_of(FILE_A, "a", "1");
    assert_eq!(header["for_play"], true);
    let _ = std::fs::remove_file(path);
    let (path, header) = batch_of(SCENARIO_5, "five", "1");
    assert_eq!(header["for_play"], false);
    let _ = std::fs::remove_file(path);
}

#[test]
fn statement_27_validate_refuses_a_scenario_for_play() {
    let measures = std::env::temp_dir().join(format!(
        "flagplot-scenario-104-{}-empty-measures.toml",
        std::process::id()
    ));
    std::fs::write(&measures, "").expect("measures");
    let report = std::env::temp_dir().join(format!(
        "flagplot-scenario-104-{}-report.md",
        std::process::id()
    ));
    let judge = |batch: &Path, measures: &Path| {
        binary(&[
            "validate",
            &batch.to_string_lossy(),
            &measures.to_string_lossy(),
            "--out",
            &report.to_string_lossy(),
        ])
    };
    let (path, _) = batch_of(FILE_A, "refused", "1");
    let refused = judge(&path, &measures);
    assert!(!refused.status.success(), "{}", stdout(&refused));
    let message = stderr(&refused);
    assert!(message.contains("is a scenario for play"), "{message}");
    assert!(
        message.contains("its results are not for the record"),
        "{message}"
    );
    assert!(!report.exists(), "a report was written");
    // Scenario 5 is judged as it is today, against its own measures.
    let five_measures = PathBuf::from(ROOT).join("data/validation/eastern_solomons.toml");
    let (five, _) = batch_of(SCENARIO_5, "judged", "2");
    let judged = judge(&five, &five_measures);
    assert!(judged.status.success(), "{}", stderr(&judged));
    assert!(report.exists(), "scenario 5 was not judged");
    for file in [path, five, measures, report] {
        let _ = std::fs::remove_file(file);
    }
}
