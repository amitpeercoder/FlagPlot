//! Scenario 122: hungry men are slow to mend. A unit's condition falls
//! with each night of hunger and rises a night at a time once it is fed in
//! full, and last night's meal counts for half.
//!
//! Proves decision 0208. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The setting
//!
//! Scenario 119's, in a file of its own: a placeholder scenario on a
//! placeholder map, with no real unit. Its ground figures are in a file of
//! its own, the frozen step 1 figures with five added: the daily supply
//! for a man is 0.005 t, `rations_landed_nights` is 3,
//! `hungry_below_share` is 0.6, `hunger_nights_before_loss` is 14,
//! `hunger_weakness_per_night` is 0.01 and `unfed_fighting_share` is 0.5.
//! Every land unit has the standard weight and skills, and no fatigue
//! unless a statement gives it.
//!
//! Side A has a base Q, with no aircraft, and a land unit V of 1,000 men
//! ashore there at the start. The men's charge at Q is 5 t a night. Q's
//! stock is what each statement gives it. Side B has nothing ashore unless
//! a statement says so. "After night N" is as in scenario 119: read at the
//! start of day N's morning.
//!
//! The words: a unit's condition is 1 less its nights of hunger times
//! 0.01, not below 0. Its share from last night's meal is 0.5 plus 0.5
//! times its fed share. Its fighting share is the one times the other.
//! "Would fire" is what side A is told of its own unit: the men ashore
//! times the fighting share times 100 less the fatigue over 100, cut to a
//! whole man.
//!
//! "The rule of 0205" is the same scenario on a ground file with the
//! three figures of decision 0205 and without the last two.
//!
//! # Statements: the fall
//!
//! 1. **A night on nothing halves a unit and no more.** With no stock at
//!    Q, after the first night V's fed share is 0 and its count 1: its
//!    condition is 0.99, its share from the meal 0.5, its fighting share
//!    0.495, and 495 men would fire. By the rule of 0205 none would.
//! 2. **Each night takes a hundredth more.** After the fifth night the
//!    count is 5: condition 0.95, fighting share 0.475, and 475 would
//!    fire. After the fourteenth, 0.86 times 0.5, 0.43: 430.
//! 3. **Men still die as before.** On the fifteenth night V loses 50 men
//!    and has 950, as in scenario 119. Its count is 15: 950 times 0.85
//!    times 0.5 is 403.75, and 403 would fire.
//! 4. **Half a ration is hunger, and bites less at once.** With 2.5 t at Q
//!    the first night feeds 500: the fed share is 0.5, below 0.6, and the
//!    count is 1. The share from the meal is 0.75, the fighting share
//!    0.99 times 0.75, 0.7425, and 742 would fire. With 3 t the share is
//!    0.6, not below the mark, and the count stays at nothing: the share
//!    from the meal is 0.8 and 800 would fire. By the rule of 0205 they
//!    are 500 and 600.
//!
//! # Statements: the rise
//!
//! 5. **Five nights hungry, five nights to mend.** With no stock for five
//!    nights and then 5 t reaching Q on each of days 6 to 10, V is fed in
//!    full from the sixth night. After it the count is 4 and 960 would
//!    fire; after the seventh 970, the eighth 980, the ninth 990, and
//!    after the tenth the count is nothing and 1,000 would fire. By the
//!    rule of 0205, 1,000 would fire after the sixth.
//! 6. **A short night does not mend.** With no stock for five nights and
//!    then 3.5 t on day 6 and 5 t on day 7: after the sixth night the fed
//!    share is 0.7 and the count stays 5; the share from the meal is
//!    0.85, the fighting share 0.95 times 0.85, 0.8075, and 807 would
//!    fire. After the seventh the count is 4 and 960 would fire.
//! 7. **Past a hundred nights there is nothing to give back at first.**
//!    With V given a count of 120 at the start and 110 t at Q, V is fed in
//!    full every night. After the first night the count is 119 and no man
//!    would fire; after the twentieth it is 100 and none would; after the
//!    twenty-first it is 99, the condition is 0.01, and 10 would fire.
//!
//! # Statements: the function
//!
//! 8. **Fatigue is taken after.** 1,000 men with a fighting share of
//!    0.475 and a fatigue of 20: 1,000 times 0.475 times 0.8, 380 fire.
//! 9. **The cut is made once, at the end.** 333 men with a fighting share
//!    of 0.475 and no fatigue: 158.175, and 158 fire.
//! 10. **Work.** V is a fighting unit: with a fighting share of 0.475 the
//!     work it makes in a day is 1,000 times 0.475 times 1 times 2 over 2
//!     times 0.5, 237.5. A working unit of 400 men with a count of 5, fed
//!     in full, makes 400 times 0.95, 380. The staff's count of a day's
//!     work for each gives the same figure.
//!
//! # Statements: work, training and what a side is told
//!
//! 11. **A unit fed nothing still works, and is not told it was not
//!     fed.** With V a unit at work on Q's defences and no stock, on the
//!     first day V's work is 1,000 times 0.495 times 0.5, 247.5, and the
//!     briefing gives no reason it did nothing. By the rule of 0205 it
//!     makes none and is told "not fed".
//! 12. **Men who are mending do not train.** With V set to train, hungry
//!     for five nights and fed as in statement 5: V does not train on
//!     days 1 to 9. On each of days 6 to 9 it was fed in full and its
//!     count was above nothing, and the reason A is given is "mending".
//!     It trains on day 10. By the rule of 0205 it trains on day 6.
//! 13. **The fight reads the same share.** V is given a count of 5 at the
//!     start and no stock. Side B has a unit E of 1,000 men in Q's hex,
//!     set to attack, with no base there, so E is fed nothing. After the
//!     first night V's count is 6 and E's is 1. In the first fight V
//!     fires 1,000 times 0.94 times 0.5, 470 men, and E fires 1,000 times
//!     0.99 times 0.5, 495, each with no fatigue. The record of V's fire
//!     gives a fed share of 0, 6 nights of hunger, a condition of 0.94
//!     and a fighting share of 0.47, and A is shown all four. B is shown
//!     none of them for V, and A none of them for E.
//! 14. **A is told its unit's condition, and only where it is below
//!     one.** In statement 5 V's line gives a condition of 0.95 after the
//!     fifth night and 0.96 after the sixth, and gives none after the
//!     tenth.
//!
//! # Statements: where it runs
//!
//! 15. **With the three figures of 0205 and without the two,** every
//!     statement's "by the rule of 0205" holds, and V's line gives no
//!     condition.
//! 16. **With no hunger figures at all,** V with no stock fires none after
//!     the first night, loses 50 men that night, and is whole in its
//!     share the first night it is fed, as today.
//! 17. **The loader.** A ground file with one of the two figures and not
//!     the other is refused. One with the two and without the three of
//!     0205 is refused. One with either below 0 or above 1 is refused. 0
//!     and 1 are taken.
//! 18. **The files.** `data/reference/ground_hunger.toml` gives
//!     `hunger_weakness_per_night` 0.01, `unfed_fighting_share` 0.9 and
//!     `hungry_below_share` 0.6, and no other figure of it differs from
//!     the live `ground.toml`'s. Its frozen counterpart gives the two new
//!     figures with the same values. Scenario 119's own files are as they
//!     were, and its tests pass unchanged. (Corrected by the architect on
//!     Nov 21, 2026: the table's figure was 0.5 as first written, and was
//!     moved to 0.9 on research into a day without food. This scenario's
//!     own file keeps 0.5, and no other statement moves.)
//!
//! # Properties
//!
//! - **No game on file moves.** Every scenario and recorded game on file
//!   but the table's copies gives the events and the daily hashes it gave
//!   before.
//! - **No new state.** The state's layout version is what it was, and a
//!   saved game of scenario 119 from before the change loads and replays.
//! - **Nothing is drawn.** The same game played twice gives the same
//!   events.
//! - **Each side sees its own.** B's view gives none of A's counts,
//!   conditions or fighting shares, in any event or answer.
//! - **One function.** The fight, the work, the side's count of firing
//!   men, the staff's count of work and the unit's line give the same
//!   fighting share for the same unit on the same morning, in statements
//!   5 and 13.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::condition::{unit_fighting_share, FightingShare};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, LandUnitId};
use sim_core::side_events::SideEventKind;
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_core::Staffs;
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, staffs};

mod common;

const Q: Hex = Hex::new(10, 0);

fn path() -> PathBuf {
    common::data_dir()
        .join("scenarios/hungry-men-are-slow-to-mend/122_hungry_men_are_slow_to_mend.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// Adds `what` at the mark `# MARK: name`; the mark stays.
fn at(text: &str, name: &str, what: &str) -> String {
    let mark = format!("# MARK: {name}\n");
    changed(text, &mark, &format!("{what}{mark}"))
}

fn fx(text: &str) -> Fixed {
    text.parse::<Fixed>().expect("a decimal figure")
}

fn unit(raw: u32) -> LandUnitId {
    LandUnitId::new(raw)
}

const V: u32 = 1;
const E: u32 = 2;

// ---------------------------------------------------------------- Variants

fn with_stock(text: &str, tons: &str) -> String {
    changed(text, "supply_t = 0\n", &format!("supply_t = \"{tons}\"\n"))
}

/// V given a count of hunger at the start.
fn with_count(text: &str, nights: u32) -> String {
    at(text, "v", &format!("hunger_nights = {nights}\n"))
}

fn with_men(text: &str, men: u32) -> String {
    changed(text, "men = 1000\n", &format!("men = {men}\n"))
}

fn with_days(text: &str, days: u32) -> String {
    changed(text, "days = 22\n", &format!("days = {days}\n"))
}

/// A supply line to Q.
fn with_line(text: &str, tons: &str, first: u32, last: u32) -> String {
    at(
        text,
        "lines",
        &format!(
            "[[supply_line]]\nbase = 1\nsupply_t = \"{tons}\"\nfirst_day = {first}\nlast_day = {last}\n\n"
        ),
    )
}

/// V given an activity on day 1: `train`, or `work` at a job.
fn with_activity(text: &str, unit: u32, activity: &str, job: Option<&str>) -> String {
    let job = job.map_or(String::new(), |j| format!("job = \"{j}\"\n"));
    at(
        text,
        "orders",
        &format!(
            "[[order]]\nday = 1\nside = \"allied\"\nland_unit = {unit}\nkind = \"set_activity\"\nactivity = \"{activity}\"\n{job}\n"
        ),
    )
}

/// A second Allied land unit at Q.
fn with_allied_unit(text: &str, id: u32, men: u32, extra: &str) -> String {
    at(
        text,
        "units",
        &format!(
            "[[land_unit]]\nid = {id}\nname = \"Placeholder unit {id}\"\nside = \"allied\"\nmen = {men}\nq = 10\nr = 0\n{extra}\n"
        ),
    )
}

/// Side B's unit E of `men` in Q's hex, set to attack, with no base of B there.
fn with_e(text: &str, men: u32) -> String {
    at(
        text,
        "units",
        &format!(
            "[[land_unit]]\nid = {E}\nname = \"Placeholder E\"\nside = \"japanese\"\nmen = {men}\nq = 10\nr = 0\nposture = \"attack\"\n\n"
        ),
    )
}

const NEW_TWO: [(&str, &str); 2] = [
    ("hunger_weakness_per_night = \"0.01\"\n", ""),
    ("unfed_fighting_share = \"0.5\"\n", ""),
];

/// The rule of 0205: the scenario's ground figures without the two new ones.
fn rule_of_0205() -> LoadOptions {
    common::altered_reference("rule_0205", |root| {
        common::edit_file_by(root, "ground_placeholder_scenario_122.toml", &NEW_TWO);
    })
}

/// No hunger figures at all: today's rule.
fn no_hunger_figures() -> LoadOptions {
    common::altered_reference("no_hunger", |root| {
        common::edit_file_by(
            root,
            "ground_placeholder_scenario_122.toml",
            &[
                NEW_TWO[0],
                NEW_TWO[1],
                ("rations_landed_nights = 3\n", ""),
                ("hungry_below_share = \"0.6\"\n", ""),
                ("hunger_nights_before_loss = 14\n", ""),
            ],
        );
    })
}

// ---------------------------------------------------------------- Playing

/// What a unit gives on a morning, from the side's own view and from the
/// truth.
#[derive(Clone, Copy, Debug)]
struct Seen {
    /// `SideView::firing_men`: "would fire".
    firing: u32,
    /// `SideView::fighting_share`.
    share: FightingShare,
    /// `unit_fighting_share` on the truth.
    truth: FightingShare,
    men: u32,
}

struct Played {
    events: Vec<Event>,
    ends: Vec<TruthState>,
    /// The readings at the start of each day's morning pulse (after the
    /// night's feeding), for every land unit as its own side sees it.
    mornings: BTreeMap<(u32, u32), Seen>,
    scenario: Scenario,
}

fn load_with(text: &str, options: &LoadOptions) -> Scenario {
    common::from_text_with(text, &path(), options).expect("the scenario loads")
}

fn play(text: &str) -> Played {
    play_with(text, &common::frozen())
}

fn play_with(text: &str, options: &LoadOptions) -> Played {
    play_scenario(load_with(text, options), false)
}

fn play_scenario(mut scenario: Scenario, with_staff: bool) -> Played {
    let mut ends = vec![scenario.world.umpire().clone()];
    let mut mornings = BTreeMap::new();
    let held = staffs();
    loop {
        let staff = if with_staff {
            as_staffs(&held)
        } else {
            Staffs::none()
        };
        let summary = scenario
            .next_day_watched(staff, &mut |world, moment| {
                if let sim_core::choices::Moment::PulseBegins {
                    day,
                    pulse: Pulse::Morning,
                } = moment
                {
                    let truth = world.umpire();
                    let Some(figures) = world.ref_data().ground.as_ref() else {
                        return;
                    };
                    for (id, land) in &truth.land_units {
                        let side = land.side;
                        let view = world.side_view(side);
                        let (Some(firing), Some(share)) =
                            (view.firing_men(*id), view.fighting_share(*id))
                        else {
                            continue;
                        };
                        mornings.insert(
                            (day, id.raw()),
                            Seen {
                                firing,
                                share,
                                truth: unit_fighting_share(land, figures),
                                men: land.men_ashore,
                            },
                        );
                    }
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        ends.push(scenario.world.umpire().clone());
    }
    Played {
        events: scenario.world.events().iter().cloned().collect(),
        ends,
        mornings,
        scenario,
    }
}

impl Played {
    fn end(&self, day: usize) -> &TruthState {
        &self.ends[day]
    }

    fn count(&self, day: usize, u: u32) -> u32 {
        self.end(day).land_units[&unit(u)].hunger_nights
    }

    fn men(&self, day: usize, u: u32) -> u32 {
        self.end(day).land_units[&unit(u)].men_ashore
    }

    /// The reading of unit `u` on the morning of `day`.
    fn morning(&self, day: u32, u: u32) -> Seen {
        self.mornings[&(day, u)]
    }

    /// The `MenFed` of Q as (day, men, fed, unfed).
    fn fed(&self) -> Vec<(u32, u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenFed {
                    hex,
                    men,
                    fed,
                    unfed,
                    ..
                } if *hex == Q => Some((e.day, *men, *fed, *unfed)),
                _ => None,
            })
            .collect()
    }

    /// The `MenLostUnfed` of unit `u` as (day, lost, ashore).
    fn lost(&self, u: u32) -> Vec<(u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenLostUnfed {
                    unit: who,
                    lost,
                    ashore,
                    ..
                } if *who == unit(u) => Some((e.day, *lost, *ashore)),
                _ => None,
            })
            .collect()
    }

    /// The days on which unit `u` had a `TrainingDay`.
    fn trained(&self, u: u32) -> Vec<u32> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TrainingDay { unit: who, .. } if *who == unit(u) => Some(e.day),
                _ => None,
            })
            .collect()
    }

    /// The `GroundFire` of unit `u` as (day, event).
    fn fire(&self, u: u32) -> Vec<&Event> {
        self.events
            .iter()
            .filter(
                |e| matches!(&e.kind, EventKind::GroundFire { unit: who, .. } if *who == unit(u)),
            )
            .collect()
    }

    /// The `UnitWorked` of unit `u`.
    fn worked(&self, u: u32) -> Vec<&Event> {
        self.events
            .iter()
            .filter(
                |e| matches!(&e.kind, EventKind::UnitWorked { unit: who, .. } if *who == unit(u)),
            )
            .collect()
    }
}

fn factor(event: &Event, name: &str) -> Fixed {
    event
        .breakdown
        .as_ref()
        .expect("a breakdown")
        .factors
        .iter()
        .find(|f| f.name == name)
        .unwrap_or_else(|| panic!("a factor {name} in {event:?}"))
        .value
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

/// V's reading on the morning of `day` is (count, condition, meal share,
/// fighting share, would fire), the three shares in decimals.
#[track_caller]
fn assert_morning(
    played: &Played,
    day: u32,
    count: u32,
    condition: &str,
    meal: &str,
    share: &str,
    firing: u32,
) {
    let seen = played.morning(day, V);
    let mending = seen.share.mending.expect("the figures of 0208 are given");
    assert_eq!(mending.hunger_nights, count, "count, morning {day}");
    assert_eq!(mending.condition, fx(condition), "condition, morning {day}");
    assert_eq!(mending.meal_share, fx(meal), "meal share, morning {day}");
    assert_eq!(seen.share.share, fx(share), "fighting share, morning {day}");
    assert_eq!(seen.firing, firing, "would fire, morning {day}");
    // The truth's count is the same.
    assert_eq!(
        played.ends[day as usize].land_units[&unit(V)].hunger_nights,
        count
    );
}

// ---------------------------------------------------------------- Sessions

/// A session of both seats over the scenario's world, `days` days played,
/// with Allied `orders` (the issue_orders list) given on day 1: the briefing
/// then is the one of day `days + 1`, of what happened in day `days`. The
/// scenario's own `[[order]]` entries are not used by a session.
fn session_after(text: &str, options: &LoadOptions, days: u32) -> Session {
    session_with(text, options, days, &[])
}

fn session_with(text: &str, options: &LoadOptions, days: u32, orders: &[Value]) -> Session {
    let scenario = load_with(text, options);
    let name = scenario.name.clone();
    let mut session = Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Player,
    );
    if !orders.is_empty() {
        let answer = session
            .call(Side::Allied, "issue_orders", &json!({ "orders": orders }))
            .expect("issue_orders answers");
        assert_eq!(answer["refused_in_call"], 0, "{answer}");
    }
    for _ in 0..days {
        for side in [Side::Allied, Side::Japanese] {
            session
                .call(side, "end_day", &json!({}))
                .expect("end_day answers");
        }
    }
    session
}

/// V set to `activity` (and `job`) by an order of the session.
fn set_activity_json(activity: &str, job: Option<&str>) -> Value {
    let mut order = json!({ "kind": "set_activity", "activity": activity });
    if let Some(job) = job {
        order["job"] = json!(job);
    }
    json!({ "unit": "land_unit:1", "order": order })
}

fn briefing(session: &mut Session, side: Side) -> Value {
    session
        .call(side, "get_briefing", &json!({ "text": false }))
        .expect("the briefing")
}

/// The line of a land unit in the briefing.
fn line_of(briefing: &Value, handle: &str) -> Option<Value> {
    briefing["land_units"]["units"]
        .as_array()
        .expect("units")
        .iter()
        .find(|l| l["land_unit"]["handle"] == handle)
        .cloned()
}

/// The condition a unit's line gives, if it gives one.
fn condition_of(line: &Value) -> Option<Fixed> {
    line.get("condition").map(|c| {
        let text = c.as_str().map_or_else(|| c.to_string(), str::to_string);
        fx(&text)
    })
}

/// The reasons the briefing gives for a unit that did nothing yesterday.
fn no_activity(briefing: &Value, handle: &str) -> Vec<String> {
    briefing["land_units"]
        .get("no_activity_yesterday")
        .and_then(Value::as_array)
        .map(|lines| {
            lines
                .iter()
                .filter(|l| l["land_unit"]["handle"] == handle)
                .map(|l| l["reason"].as_str().expect("a reason").to_string())
                .collect()
        })
        .unwrap_or_default()
}

// ---------------------------------------------------------------- The setting

#[test]
fn the_setting_is_what_the_comment_says() {
    let played = play(&with_days(&text(), 2));
    let figures = played.scenario.world.ref_data().ground.as_ref().unwrap();
    let hunger = figures.hunger.expect("the figures of 0205");
    assert_eq!(hunger.rations_landed_nights, n(3));
    assert_eq!(hunger.hungry_below_share, fx("0.6"));
    assert_eq!(hunger.hunger_nights_before_loss, 14);
    let mending = hunger.mending.expect("the figures of 0208");
    assert_eq!(mending.hunger_weakness_per_night, fx("0.01"));
    assert_eq!(mending.unfed_fighting_share, fx("0.5"));
    assert_eq!(figures.daily_supply_t_per_man, fx("0.005"));
    let start = played.end(0);
    assert_eq!(start.land_units[&unit(V)].men_ashore, 1000);
    assert_eq!(start.land_units[&unit(V)].hunger_nights, 0);
    assert_eq!(start.bases[&BaseId::new(1)].supply_t, Fixed::ZERO);
    assert_eq!(start.bases[&BaseId::new(1)].hex, Q);
    assert_eq!(start.land_units.len(), 1);
}

// ---------------------------------------------------------------- Statements: the fall

/// 1. A night on nothing halves a unit and no more.
#[test]
fn statement_1_a_night_on_nothing_halves_a_unit_and_no_more() {
    let t = with_days(&text(), 2);
    let played = play(&t);
    assert_eq!(played.fed()[0], (1, 1000, 0, 1000));
    assert_eq!(played.morning(1, V).share.fed_share, Fixed::ZERO);
    assert_morning(&played, 1, 1, "0.99", "0.5", "0.495", 495);
    // By the rule of 0205 none would.
    let old = play_with(&t, &rule_of_0205());
    assert!(old.morning(1, V).share.mending.is_none());
    assert_eq!(old.morning(1, V).firing, 0);
}

/// 2. Each night takes a hundredth more.
#[test]
fn statement_2_each_night_takes_a_hundredth_more() {
    let played = play(&with_days(&text(), 15));
    assert_morning(&played, 5, 5, "0.95", "0.5", "0.475", 475);
    assert_morning(&played, 14, 14, "0.86", "0.5", "0.43", 430);
    // The count is one more each night.
    for day in 1..=14u32 {
        assert_eq!(
            played.morning(day, V).share.mending.unwrap().hunger_nights,
            day
        );
    }
}

/// 3. Men still die as before.
#[test]
fn statement_3_men_still_die_as_before() {
    let played = play(&with_days(&text(), 16));
    assert_eq!(played.lost(V)[0], (15, 50, 950));
    assert_eq!(played.men(15, V), 950);
    assert_morning(&played, 15, 15, "0.85", "0.5", "0.425", 403);
    // 950 times 0.85 times 0.5 is 403.75.
    assert_eq!(n(950) * played.morning(15, V).share.share, fx("403.75"));
    // Nobody was lost before the fifteenth night.
    assert_eq!(played.lost(V).iter().filter(|l| l.0 < 15).count(), 0);
}

/// 4. Half a ration is hunger, and bites less at once.
#[test]
fn statement_4_half_a_ration_is_hunger_and_bites_less_at_once() {
    let half = with_days(&with_stock(&text(), "2.5"), 2);
    let played = play(&half);
    assert_eq!(played.fed()[0], (1, 1000, 500, 500));
    assert_eq!(played.morning(1, V).share.fed_share, fx("0.5"));
    assert_morning(&played, 1, 1, "0.99", "0.75", "0.7425", 742);
    let old = play_with(&half, &rule_of_0205());
    assert_eq!(old.morning(1, V).firing, 500);

    let three = with_days(&with_stock(&text(), "3"), 2);
    let played = play(&three);
    assert_eq!(played.fed()[0], (1, 1000, 600, 400));
    assert_eq!(played.morning(1, V).share.fed_share, fx("0.6"));
    assert_morning(&played, 1, 0, "1", "0.8", "0.8", 800);
    let old = play_with(&three, &rule_of_0205());
    assert_eq!(old.morning(1, V).firing, 600);
}

// ---------------------------------------------------------------- Statements: the rise

/// The game of statements 5, 12 and 14: no stock for five nights, then 5 t
/// reaching Q on each of days 6 to 10.
fn five_hungry_five_fed() -> String {
    with_line(&with_days(&text(), 12), "5", 6, 10)
}

/// 5. Five nights hungry, five nights to mend.
#[test]
fn statement_5_five_nights_hungry_five_nights_to_mend() {
    let t = five_hungry_five_fed();
    let played = play(&t);
    for day in 1..=5usize {
        assert_eq!(played.count(day, V), day as u32);
    }
    for (day, count, condition, firing) in [
        (6u32, 4u32, "0.96", 960u32),
        (7, 3, "0.97", 970),
        (8, 2, "0.98", 980),
        (9, 1, "0.99", 990),
        (10, 0, "1", 1000),
    ] {
        assert_eq!(
            played.fed()[day as usize - 1],
            (day, 1000, 1000, 0),
            "night {day}"
        );
        assert_morning(&played, day, count, condition, "1", condition, firing);
    }
    assert!(played.lost(V).is_empty());
    // By the rule of 0205, 1,000 would fire after the sixth.
    let old = play_with(&t, &rule_of_0205());
    assert_eq!(old.morning(6, V).firing, 1000);
}

/// 6. A short night does not mend.
#[test]
fn statement_6_a_short_night_does_not_mend() {
    let t = with_line(&with_line(&with_days(&text(), 8), "3.5", 6, 6), "5", 7, 7);
    let played = play(&t);
    assert_eq!(played.fed()[5], (6, 1000, 700, 300));
    assert_morning(&played, 6, 5, "0.95", "0.85", "0.8075", 807);
    assert_eq!(played.fed()[6], (7, 1000, 1000, 0));
    assert_morning(&played, 7, 4, "0.96", "1", "0.96", 960);
}

/// 7. Past a hundred nights there is nothing to give back at first.
#[test]
fn statement_7_past_a_hundred_nights_there_is_nothing_to_give_back_at_first() {
    let t = with_stock(&with_count(&text(), 120), "110");
    let played = play(&t);
    for day in 1..=21u32 {
        assert_eq!(
            played.fed()[day as usize - 1].2,
            1000,
            "fed in full, night {day}"
        );
    }
    assert_morning(&played, 1, 119, "0", "1", "0", 0);
    assert_morning(&played, 20, 100, "0", "1", "0", 0);
    assert_morning(&played, 21, 99, "0.01", "1", "0.01", 10);
    // Fed in full: nobody is lost to hunger.
    assert!(played.lost(V).is_empty());
}

// ---------------------------------------------------------------- Statements: the function

/// 8. Fatigue is taken after.
#[test]
fn statement_8_fatigue_is_taken_after() {
    // V's count of 4 and no stock: a fighting share of 0.475 on the first
    // morning; a fatigue of 20.
    let t = with_days(&with_count(&with_fatigue(&text(), 20), 4), 2);
    let played = play(&t);
    assert_eq!(played.end(0).land_units[&unit(V)].fatigue, n(20));
    let seen = played.morning(1, V);
    assert_eq!(seen.share.share, fx("0.475"));
    assert_eq!(seen.men, 1000);
    assert_eq!(played.scenario.world.umpire().land_units.len(), 1);
    assert_eq!(seen.firing, 380);
}

fn with_fatigue(text: &str, fatigue: u32) -> String {
    at(text, "v", &format!("fatigue = {fatigue}\n"))
}

/// 9. The cut is made once, at the end.
#[test]
fn statement_9_the_cut_is_made_once_at_the_end() {
    let t = with_days(&with_count(&with_men(&text(), 333), 4), 2);
    let played = play(&t);
    let seen = played.morning(1, V);
    assert_eq!(seen.share.share, fx("0.475"));
    assert_eq!(seen.men, 333);
    assert_eq!(n(333) * seen.share.share, fx("158.175"));
    assert_eq!(seen.firing, 158);
}

/// The work of V on day 1 (the event), as a fighting unit at the defences.
fn work_game(count: u32, men: u32) -> Played {
    let t = with_activity(
        &with_days(&with_men(&with_count(&text(), count), men), 3),
        V,
        "work",
        Some("defences"),
    );
    play(&t)
}

/// 10. Work. V is a fighting unit: with a fighting share of 0.475 the work
/// it makes in a day is 1,000 times 0.475 times 1 times 2 over 2 times 0.5,
/// 237.5. A working unit of 400 men with a count of 5, fed in full, makes
/// 400 times 0.95, 380.
#[test]
fn statement_10_work() {
    // V: count 4 at the start and no stock, so 5 and a share of 0.475 on the
    // morning of day 1, when it works.
    let played = work_game(4, 1000);
    let first = played.worked(V)[0];
    assert_eq!(first.day, 1);
    let EventKind::UnitWorked { work, .. } = &first.kind else {
        unreachable!()
    };
    assert_eq!(*work, fx("237.5"));
    assert_eq!(factor(first, "fighting_share"), fx("0.475"));
    assert_eq!(factor(first, "fed_share"), Fixed::ZERO);
    assert_eq!(factor(first, "hunger_nights"), n(5));
    assert_eq!(factor(first, "condition"), fx("0.95"));
    // The function the staff's count reads gives the same figure.
    let figures = played.scenario.world.ref_data().ground.as_ref().unwrap();
    let v = &played.end(0).land_units[&unit(V)];
    assert!(!v.working);
    let share = played.morning(1, V).truth.share;
    assert_eq!(
        sim_core::works::unit_work(v.men_ashore, share, v.skills.works, v.working, figures),
        fx("237.5")
    );

    // A working unit of 400 men, count 5 after a night fed in full (6 at the
    // start), 380; stock enough for V's 5 t and its 2 t.
    let t = with_activity(
        &with_days(
            &with_stock(
                &with_allied_unit(
                    &text(),
                    3,
                    400,
                    "working = true\nhunger_nights = 6\nskills = { works = 2 }\n",
                ),
                "20",
            ),
            3,
        ),
        3,
        "work",
        Some("defences"),
    );
    let played = play(&t);
    let first = played.worked(3)[0];
    let EventKind::UnitWorked { work, .. } = &first.kind else {
        unreachable!()
    };
    assert_eq!(*work, n(380));
    assert_eq!(factor(first, "fighting_share"), fx("0.95"));
    assert_eq!(factor(first, "fed_share"), Fixed::ONE);
    let figures = played.scenario.world.ref_data().ground.as_ref().unwrap();
    let w = &played.end(0).land_units[&unit(3)];
    assert!(w.working);
    assert_eq!(
        sim_core::works::unit_work(
            w.men_ashore,
            played.morning(1, 3).truth.share,
            w.skills.works,
            w.working,
            figures
        ),
        n(380)
    );
}

/// The reasons and orders of the staff on `day`, as one text.
fn staff_text(scenario: &Scenario, day: u32) -> String {
    let handed = scenario.world.staff_handed_in();
    let _ = day;
    format!("{:?}", handed.get(&Side::Allied))
}

/// 10, the staff's count. The staff keeps Q's defences with V, a fighting
/// unit. V's count is 4 at the start, so on the staff's day 2 (it asks at
/// the start of the day, from the state the night of day 1 left) V's count is
/// 5, its fed share 0 and its fighting share 0.475: the reason the staff
/// gives says 237.5 a day, which is `unit_work` of V's men, its
/// fighting share, its skill and its standing, as the work reads them.
#[test]
fn statement_10_the_staffs_count_of_a_days_work_gives_the_same_figure() {
    let commands = "[[command]]\nday = 2\nside = \"allied\"\nkind = \"set_intent\"\nintent = 1\nintent_kind = \"build_up\"\nbase = 1\nwants = [\"defences\"]\ndefences = 100\n\n[[command]]\nday = 2\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nland_units = [1]\n\n";
    let t = with_days(&with_count(&at(&text(), "commands", commands), 4), 4);
    let mut scenario = load_with(&t, &common::frozen());
    let held = staffs();
    let mut said = Vec::new();
    let mut ends = vec![scenario.world.umpire().clone()];
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        said.push((day, staff_text(&scenario, day)));
        ends.push(scenario.world.umpire().clone());
    }
    let day2 = &said.iter().find(|(d, _)| *d == 2).expect("day 2").1;
    assert!(
        day2.contains("237.5 a day by the units at it"),
        "the staff's count: {day2}"
    );
    // The same figure from `unit_work` on what the staff saw: V as the night
    // of day 1 left it.
    let figures = scenario.world.ref_data().ground.as_ref().unwrap();
    let work_of = |v: &sim_core::state::LandUnit| {
        sim_core::works::unit_work(
            v.men_ashore,
            unit_fighting_share(v, figures).share,
            v.skills.works,
            v.working,
            figures,
        )
    };
    let seen = &ends[1].land_units[&unit(V)];
    assert_eq!(seen.hunger_nights, 5);
    assert_eq!(work_of(seen), fx("237.5"));
    // V works on day 2 with the night of day 2 behind it, count 6: the work
    // it makes is the same function of that morning's state (a day on, the
    // staff's figure is a night old, as decision 0208, 19 says).
    let worked: Vec<_> = scenario
        .world
        .events()
        .iter()
        .filter(|e| matches!(&e.kind, EventKind::UnitWorked { unit: u, .. } if *u == unit(V)))
        .cloned()
        .collect();
    assert_eq!(worked[0].day, 2);
    let EventKind::UnitWorked { work, .. } = &worked[0].kind else {
        unreachable!()
    };
    assert_eq!(*work, work_of(&ends[2].land_units[&unit(V)]));
    assert_eq!(*work, fx("235"));
}

// ---------------------------------------------------------------- Statements: work, training, what a side is told

/// The text of a game without the scenario's own `[[order]]` entries, which a
/// session does not use (its orders are given through the session).
fn text_of_game(game: &str) -> String {
    match game.find("[[order]]") {
        Some(i) => game[..i].to_string() + "# MARK: orders\n",
        None => game.to_string(),
    }
}

/// 11. A unit fed nothing still works, and is not told it was not fed.
#[test]
fn statement_11_a_unit_fed_nothing_still_works_and_is_not_told_it_was_not_fed() {
    let t = with_activity(&with_days(&text(), 3), V, "work", Some("defences"));
    let played = play(&t);
    let first = played.worked(V)[0];
    assert_eq!(first.day, 1);
    let EventKind::UnitWorked { work, .. } = &first.kind else {
        unreachable!()
    };
    assert_eq!(*work, fx("247.5"));
    let order = [set_activity_json("work", Some("defences"))];
    let mut session = session_with(&text_of_game(&t), &common::frozen(), 1, &order);
    let b = briefing(&mut session, Side::Allied);
    assert_eq!(line_of(&b, "land_unit:1").unwrap()["activity"], "work");
    assert!(
        no_activity(&b, "land_unit:1").is_empty(),
        "no reason it did nothing: {}",
        b["land_units"]
    );
    // By the rule of 0205 it makes none and is told "not fed".
    let old = play_with(&t, &rule_of_0205());
    assert!(old.worked(V).is_empty());
    let mut session = session_with(&text_of_game(&t), &rule_of_0205(), 1, &order);
    let b = briefing(&mut session, Side::Allied);
    assert_eq!(line_of(&b, "land_unit:1").unwrap()["activity"], "work");
    assert_eq!(no_activity(&b, "land_unit:1"), vec!["not_fed".to_string()]);
}

/// 12. Men who are mending do not train.
#[test]
fn statement_12_men_who_are_mending_do_not_train() {
    // V is raw (experience 0), so that it has something to train toward.
    let plain = at(&five_hungry_five_fed(), "v", "experience = 0\n");
    let t = with_activity(&plain, V, "train", None);
    let order = [set_activity_json("train", None)];
    let played = play(&t);
    let days = played.trained(V);
    assert_eq!(days.first(), Some(&10), "trains from day 10: {days:?}");
    assert!(days.iter().all(|d| *d >= 10));
    // The supply stops after day 10, and the night of day 11 is unfed.
    assert_eq!(days, vec![10]);
    for day in 6..=9usize {
        let v = &played.end(day).land_units[&unit(V)];
        assert_eq!(v.fed_share, Fixed::ONE, "fed in full on day {day}");
        assert!(v.hunger_nights > 0, "count above nothing on day {day}");
    }
    // The reason A is given, day by day (the briefing of the next morning).
    for day in 1..=9u32 {
        let mut session = session_with(&plain, &common::frozen(), day, &order);
        let b = briefing(&mut session, Side::Allied);
        let reasons = no_activity(&b, "land_unit:1");
        let expected = if day <= 5 { "not_fed" } else { "mending" };
        assert_eq!(reasons, vec![expected.to_string()], "day {day}");
    }
    let mut session = session_with(&plain, &common::frozen(), 10, &order);
    let b = briefing(&mut session, Side::Allied);
    assert!(no_activity(&b, "land_unit:1").is_empty());
    // By the rule of 0205 it trains on day 6.
    let old = play_with(&t, &rule_of_0205());
    assert_eq!(old.trained(V).first(), Some(&6), "{:?}", old.trained(V));
}

/// The sides' views of the game of statement 13.
fn game_13() -> (String, Played) {
    let t = with_days(&with_e(&with_count(&text(), 5), 1000), 3);
    let played = play(&t);
    (t, played)
}

/// 13. The fight reads the same share.
#[test]
fn statement_13_the_fight_reads_the_same_share() {
    let (_, played) = game_13();
    assert_eq!(played.count(1, V), 6);
    assert_eq!(played.count(1, E), 1);
    assert_eq!(played.end(1).land_units[&unit(E)].fed_share, Fixed::ZERO);
    let first = |u: u32| -> &Event { played.fire(u).into_iter().next().expect("fired") };
    let v = first(V);
    let e = first(E);
    assert_eq!(v.day, 1);
    assert_eq!(e.day, 1);
    let EventKind::GroundFire { firing_men: fv, .. } = &v.kind else {
        unreachable!()
    };
    let EventKind::GroundFire { firing_men: fe, .. } = &e.kind else {
        unreachable!()
    };
    assert_eq!((*fv, *fe), (470, 495));
    // Each with no fatigue.
    assert_eq!(factor(v, "fatigue_factor"), Fixed::ONE);
    assert_eq!(factor(e, "fatigue_factor"), Fixed::ONE);
    // The record of V's fire.
    assert_eq!(factor(v, "fed_share"), Fixed::ZERO);
    assert_eq!(factor(v, "hunger_nights"), n(6));
    assert_eq!(factor(v, "condition"), fx("0.94"));
    assert_eq!(factor(v, "fighting_share"), fx("0.47"));
    assert_eq!(factor(e, "hunger_nights"), n(1));
    assert_eq!(factor(e, "condition"), fx("0.99"));
    assert_eq!(factor(e, "fighting_share"), fx("0.495"));
}

/// 13, who is shown what.
#[test]
fn statement_13_a_is_shown_all_four_b_none_of_them_for_v_and_a_none_for_e() {
    let (_, played) = game_13();
    let world = &played.scenario.world;
    let names = ["fed_share", "hunger_nights", "condition", "fighting_share"];
    let fires = |side: Side| -> Vec<(LandUnitId, Vec<(&'static str, Fixed)>)> {
        world
            .side_view(side)
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                SideEventKind::GroundFire { unit, .. } => Some((
                    *unit,
                    e.breakdown
                        .as_ref()
                        .map(|b| b.factors.iter().map(|f| (f.name, f.value)).collect())
                        .unwrap_or_default(),
                )),
                _ => None,
            })
            .collect()
    };
    let a = fires(Side::Allied);
    assert!(!a.is_empty());
    assert!(
        a.iter().all(|(u, _)| *u == unit(V)),
        "A is shown only V's fire"
    );
    let first = &a[0].1;
    let value = |name: &str| first.iter().find(|f| f.0 == name).map(|f| f.1);
    assert_eq!(value("fed_share"), Some(Fixed::ZERO));
    assert_eq!(value("hunger_nights"), Some(n(6)));
    assert_eq!(value("condition"), Some(fx("0.94")));
    assert_eq!(value("fighting_share"), Some(fx("0.47")));
    // B is shown none of them for V: it is shown no fire of V's at all.
    let b = fires(Side::Japanese);
    assert!(!b.is_empty());
    assert!(
        b.iter().all(|(u, _)| *u == unit(E)),
        "B is shown only E's fire"
    );
    // On the first day (the fight's first fire), B's own fire is shown with
    // E's figures, never V's.
    for (_, factors) in &b[..1] {
        let get = |name: &str| factors.iter().find(|f| f.0 == name).map(|f| f.1);
        assert_eq!(get("condition"), Some(fx("0.99")));
        assert_eq!(get("fighting_share"), Some(fx("0.495")));
        assert!(names.iter().all(|n| get(n).is_some()));
    }
    // And in the answers of the view.
    let a_view = world.side_view(Side::Allied);
    let b_view = world.side_view(Side::Japanese);
    assert!(a_view.fighting_share(unit(V)).is_some());
    assert!(a_view.fighting_share(unit(E)).is_none());
    assert!(a_view.firing_men(unit(E)).is_none());
    assert!(b_view.fighting_share(unit(V)).is_none());
    assert!(b_view.firing_men(unit(V)).is_none());
    assert!(b_view.fighting_share(unit(E)).is_some());
    // The line of A's unit is not given to B, nor E's to A.
    let mut session = session_after(&game_13().0, &common::frozen(), 1);
    let a_brief = briefing(&mut session, Side::Allied);
    let b_brief = briefing(&mut session, Side::Japanese);
    // (Each side numbers its own units: E is B's land_unit:1.) A's briefing
    // has one unit, V, with V's figures; B's has one, E, with E's.
    let units = |b: &Value| b["land_units"]["units"].as_array().unwrap().clone();
    let (a_units, b_units) = (units(&a_brief), units(&b_brief));
    assert_eq!((a_units.len(), b_units.len()), (1, 1));
    assert_eq!(a_units[0]["land_unit"]["name"], "Placeholder V");
    assert_eq!(b_units[0]["land_unit"]["name"], "Placeholder E");
    assert_eq!(condition_of(&a_units[0]), Some(fx("0.94")));
    assert_eq!(condition_of(&b_units[0]), Some(fx("0.99")));
    assert_eq!(a_units[0]["hunger_nights"], 6);
    assert_eq!(b_units[0]["hunger_nights"], 1);
    assert!(!b_brief["land_units"].to_string().contains("Placeholder V"));
    assert!(!a_brief["land_units"].to_string().contains("Placeholder E"));
}

/// 14. A is told its unit's condition, and only where it is below one.
#[test]
fn statement_14_a_is_told_its_units_condition_only_where_it_is_below_one() {
    let t = five_hungry_five_fed();
    for (day, expected) in [(5u32, Some("0.95")), (6, Some("0.96")), (10, None)] {
        let mut session = session_after(&t, &common::frozen(), day);
        let b = briefing(&mut session, Side::Allied);
        let line = line_of(&b, "land_unit:1").expect("V's line");
        assert_eq!(
            condition_of(&line),
            expected.map(fx),
            "after night {day}: {line}"
        );
        if expected.is_none() {
            assert!(line.get("condition").is_none(), "left out when 1: {line}");
        }
    }
    // The same through list_forces.
    let mut session = session_after(&t, &common::frozen(), 5);
    let answer = session
        .call(Side::Allied, "list_forces", &json!({}))
        .expect("list_forces");
    let line = answer["land_units"]
        .as_array()
        .unwrap()
        .iter()
        .find(|l| l["land_unit"]["handle"] == "land_unit:1")
        .expect("V's line");
    assert_eq!(condition_of(line), Some(fx("0.95")));
}

// ---------------------------------------------------------------- Statements: where it runs

/// 15. With the three figures of 0205 and without the two, every "by the
/// rule of 0205" holds (those are asserted in statements 1, 4, 5, 11 and
/// 12), and V's line gives no condition.
#[test]
fn statement_15_with_the_three_of_0205_and_without_the_two() {
    let t = five_hungry_five_fed();
    let played = play_with(&t, &rule_of_0205());
    let figures = played.scenario.world.ref_data().ground.as_ref().unwrap();
    let hunger = figures.hunger.expect("the figures of 0205");
    assert!(hunger.mending.is_none());
    // The fight and the view read the fed share.
    for day in 1..=10u32 {
        let seen = played.morning(day, V);
        assert!(seen.share.mending.is_none());
        assert_eq!(seen.share.share, seen.share.fed_share);
        assert_eq!(seen.firing, if day <= 5 { 0 } else { 1000 });
        // The count is kept as in 0205.
        assert_eq!(
            played.count(day as usize, V),
            if day <= 5 { day } else { 10 - day }
        );
    }
    for day in [1, 5, 6, 10] {
        let mut session = session_after(&t, &rule_of_0205(), day);
        let b = briefing(&mut session, Side::Allied);
        let line = line_of(&b, "land_unit:1").expect("V's line");
        assert!(condition_of(&line).is_none(), "day {day}: {line}");
        assert!(line.get("condition").is_none());
    }
}

/// 16. With no hunger figures at all.
#[test]
fn statement_16_with_no_hunger_figures_at_all() {
    let t = with_line(&with_days(&text(), 4), "5", 3, 3);
    let played = play_with(&t, &no_hunger_figures());
    assert!(played
        .scenario
        .world
        .ref_data()
        .ground
        .as_ref()
        .unwrap()
        .hunger
        .is_none());
    // None fire after the first night, and 50 men are lost that night.
    assert_eq!(played.morning(1, V).firing, 0);
    assert!(played.morning(1, V).share.mending.is_none());
    assert_eq!(played.lost(V)[0], (1, 50, 950));
    // The first night it is fed (night 3), whole in its share: all fire.
    assert_eq!(played.fed()[2].2, played.fed()[2].1);
    let seen = played.morning(3, V);
    assert_eq!(seen.share.share, Fixed::ONE);
    assert_eq!(seen.firing, seen.men);
    assert_eq!(seen.men, played.men(3, V));
    // No condition in V's line.
    let mut session = session_after(&t, &no_hunger_figures(), 2);
    let b = briefing(&mut session, Side::Allied);
    assert!(line_of(&b, "land_unit:1")
        .unwrap()
        .get("condition")
        .is_none());
}

fn ground_text() -> String {
    std::fs::read_to_string(
        common::data_dir().join("reference/ground_placeholder_scenario_122.toml"),
    )
    .expect("the figures file reads")
}

/// Loads a ground file made of `text` under a name of its own.
fn load_ground(
    tag: &str,
    text: &str,
) -> Result<sim_core::refdata::GroundFigures, sim_data::DataError> {
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("scenario_122_loader");
    std::fs::create_dir_all(&dir).expect("a folder");
    let file = dir.join(format!("{tag}.toml"));
    std::fs::write(&file, text).expect("the file writes");
    sim_data::load_ground_figures(&file)
}

fn edited(edits: &[(&str, &str)]) -> String {
    edits.iter().fold(ground_text(), |text, (a, b)| {
        common::replace_once(&text, a, b)
    })
}

/// 17. The loader.
#[test]
fn statement_17_the_loader() {
    let one = "hunger_weakness_per_night = \"0.01\"\n";
    let other = "unfed_fighting_share = \"0.5\"\n";
    // One of the two and not the other is refused.
    assert!(load_ground("no_weakness", &edited(&[(one, "")])).is_err());
    assert!(load_ground("no_unfed", &edited(&[(other, "")])).is_err());
    // The two without the three of 0205 are refused.
    for (i, three) in [
        "rations_landed_nights = 3\n",
        "hungry_below_share = \"0.6\"\n",
        "hunger_nights_before_loss = 14\n",
    ]
    .iter()
    .enumerate()
    {
        assert!(
            load_ground(&format!("no_three_{i}"), &edited(&[(three, "")])).is_err(),
            "{three}"
        );
    }
    let without_the_three = edited(&[
        ("rations_landed_nights = 3\n", ""),
        ("hungry_below_share = \"0.6\"\n", ""),
        ("hunger_nights_before_loss = 14\n", ""),
    ]);
    assert!(load_ground("two_alone", &without_the_three).is_err());
    // Either below 0 or above 1 is refused.
    for (name, value) in [
        ("hunger_weakness_per_night", one),
        ("unfed_fighting_share", other),
    ] {
        for (tag, bad) in [("below", "\"-0.01\""), ("above", "\"1.01\"")] {
            let line = format!("{name} = {bad}\n");
            assert!(
                load_ground(&format!("{name}_{tag}"), &edited(&[(value, &line)])).is_err(),
                "{line}"
            );
        }
        // 0 and 1 are taken.
        for (tag, ok, expected) in [("zero", "\"0\"", 0), ("one", "\"1\"", 1)] {
            let line = format!("{name} = {ok}\n");
            let figures = load_ground(&format!("{name}_{tag}"), &edited(&[(value, &line)]))
                .unwrap_or_else(|e| panic!("{line}: {}", e.message));
            let mending = figures.hunger.unwrap().mending.unwrap();
            let got = if name == "hunger_weakness_per_night" {
                mending.hunger_weakness_per_night
            } else {
                mending.unfed_fighting_share
            };
            assert_eq!(got, n(expected), "{line}");
        }
    }
    // The file as it stands loads.
    assert!(load_ground("as_it_stands", &ground_text()).is_ok());
}

fn toml_of(relative: &str) -> toml::Table {
    let file = common::data_dir().join(relative);
    std::fs::read_to_string(&file)
        .unwrap_or_else(|e| panic!("{}: {e}", file.display()))
        .parse::<toml::Table>()
        .expect("the file is TOML")
}

const THREE: [&str; 3] = [
    "rations_landed_nights",
    "hungry_below_share",
    "hunger_nights_before_loss",
];
const TWO: [&str; 2] = ["hunger_weakness_per_night", "unfed_fighting_share"];

/// The three figures of decision 0209, which `ground_hunger.toml` may add
/// to the live file too (0209, 2).
const FIGHT_SUPPLY: [&str; 3] = [
    "fight_supply_t_per_man",
    "unsupplied_fire_share",
    "fight_days_landed",
];

/// 18. The files.
#[test]
fn statement_18_the_files() {
    let hunger = toml_of("reference/ground_hunger.toml");
    let live = toml_of("reference/ground.toml");
    assert_eq!(hunger["hunger_weakness_per_night"].as_str(), Some("0.01"));
    assert_eq!(hunger["unfed_fighting_share"].as_str(), Some("0.9"));
    assert_eq!(hunger["hungry_below_share"].as_str(), Some("0.6"));
    // No other figure of it differs from the live ground.toml's, and the
    // only keys it adds are the five.
    for (key, value) in &live {
        assert_eq!(hunger.get(key), Some(value), "{key} is the live file's");
    }
    let added: Vec<&str> = hunger
        .keys()
        .filter(|k| !live.contains_key(*k))
        .map(String::as_str)
        .collect();
    // And the three of decision 0209, by the architect's leave (0209, 2).
    let mut expected: Vec<&str> = THREE
        .iter()
        .chain(TWO.iter())
        .chain(FIGHT_SUPPLY.iter())
        .copied()
        .collect();
    expected.sort_unstable();
    assert_eq!(added, expected);
    // The live ground.toml gives none of the five.
    for key in THREE.iter().chain(TWO.iter()) {
        assert!(!live.contains_key(*key), "{key} in ground.toml");
    }
    // Its frozen counterpart gives the two new figures with the same values.
    let frozen = toml_of("reference/step1/ground_hunger.toml");
    for key in TWO {
        assert_eq!(frozen.get(key), hunger.get(key), "{key} in the frozen copy");
    }
    // Scenario 119's own files are as they were: the three, not the two.
    for file in [
        "reference/ground_placeholder_scenario_119.toml",
        "reference/step1/ground_placeholder_scenario_119.toml",
    ] {
        let t = toml_of(file);
        assert_eq!(t["rations_landed_nights"].as_integer(), Some(3), "{file}");
        assert_eq!(t["hungry_below_share"].as_str(), Some("0.5"), "{file}");
        assert_eq!(
            t["hunger_nights_before_loss"].as_integer(),
            Some(14),
            "{file}"
        );
        for key in TWO {
            assert!(!t.contains_key(key), "{key} in {file}");
        }
    }
    // Scenario 122's own two files agree.
    let own = toml_of("reference/ground_placeholder_scenario_122.toml");
    let own_frozen = toml_of("reference/step1/ground_placeholder_scenario_122.toml");
    assert_eq!(own, own_frozen);
}

// ---------------------------------------------------------------- Properties

/// Properties: nothing is drawn. The same game played twice gives the same
/// events; a game with another seed and nothing fighting gives the same
/// events too (the rule draws nothing); and the fight's first fire, which
/// comes before any die is rolled for it, gives the same firing men.
#[test]
fn property_nothing_is_drawn() {
    // Nothing fights: V alone, hungry then fed.
    let t = with_line(&with_days(&with_count(&text(), 14), 8), "5", 4, 6);
    let first = play(&t);
    let second = play(&t);
    assert_eq!(first.events, second.events);
    assert!(first.events.len() > 20);
    let other = play_scenario(
        load_with(&t, &common::frozen())
            .with_seed(7)
            .expect("a seed can be set"),
        false,
    );
    let key = |p: &Played| -> Vec<(u32, Option<Pulse>, String, Vec<sim_core::events::Cause>)> {
        p.events
            .iter()
            .map(|e| (e.day, e.pulse, format!("{:?}", e.kind), e.causes.clone()))
            .collect()
    };
    assert_eq!(key(&first), key(&other));
    // With a fight: the same twice, and the first fire (before any hit) is
    // the same under another seed.
    let t = with_days(&with_e(&with_count(&text(), 5), 1000), 4);
    let (a, b) = (play(&t), play(&t));
    assert_eq!(a.events, b.events);
    let seeded = play_scenario(
        load_with(&t, &common::frozen())
            .with_seed(7)
            .expect("a seed"),
        false,
    );
    let firing = |p: &Played, day: u32| -> Vec<(u32, u32)> {
        p.events
            .iter()
            .filter(|e| e.day == day)
            .filter_map(|e| match &e.kind {
                EventKind::GroundFire {
                    unit, firing_men, ..
                } => Some((unit.raw(), *firing_men)),
                _ => None,
            })
            .collect()
    };
    assert_eq!(firing(&a, 1), vec![(1, 470), (2, 495)]);
    assert_eq!(firing(&a, 1), firing(&seeded, 1));
}

/// Properties: each side sees its own. B's view gives none of A's counts,
/// conditions or fighting shares, in any event or answer; here with B's unit
/// E present too (statement 13's game).
#[test]
fn property_each_side_sees_its_own() {
    let (t, played) = game_13();
    let world = &played.scenario.world;
    let enemy = world.side_view(Side::Japanese).events();
    for e in &enemy {
        if let Some(b) = &e.breakdown {
            // Every factor B sees of a unit's fire is its own unit's: V's
            // figures (count 6, condition 0.94, share 0.47) appear nowhere.
            // E's count on day d is d (V's is 5 more): so each figure shown
            // is E's own.
            let day = i64::from(e.day);
            for f in &b.factors {
                if f.name == "hunger_nights" {
                    assert_eq!(f.value, n(day), "{e:?}");
                }
                if f.name == "condition" {
                    assert_eq!(f.value, n(1) - fx("0.01") * day, "{e:?}");
                }
            }
        }
        let shown = format!("{:?}", e.kind);
        assert!(!shown.contains("MenLostUnfed"), "{shown}");
        assert!(
            !shown.contains("HungerNightsChanged") || shown.contains("Whole"),
            "{shown}"
        );
    }
    // B's answers: none of A's units, and none of A's figures.
    let mut session = session_after(&t, &common::frozen(), 2);
    let b = briefing(&mut session, Side::Japanese);
    let units = b["land_units"]["units"].to_string();
    assert!(!units.contains("Placeholder V"), "{units}");
    let answer = session
        .call(Side::Japanese, "list_forces", &json!({}))
        .expect("list_forces");
    assert!(!answer["land_units"].to_string().contains("Placeholder V"));
    // The one unit B is told of is its own E, with E's count (2), not V's (7).
    let lines = b["land_units"]["units"].as_array().unwrap();
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0]["hunger_nights"], 2);
    let view = world.side_view(Side::Japanese);
    assert!(view.fighting_share(unit(V)).is_none());
    assert!(view.firing_men(unit(V)).is_none());
}

/// Properties: one function. The fight, the work, the side's count of
/// firing men, the unit's line and the truth give the same fighting share
/// for the same unit on the same morning (statement 5's game and
/// statement 13's).
#[test]
fn property_one_function_in_statement_5() {
    let t = five_hungry_five_fed();
    let fighter = play(&t);
    let worker = play(&with_activity(&t, V, "work", Some("defences")));
    for day in 6..=10u32 {
        let seen = fighter.morning(day, V);
        assert_eq!(
            seen.share, seen.truth,
            "the view and the truth, morning {day}"
        );
        let share = seen.share.share;
        // Would fire is the men times the share, cut once.
        assert_eq!((n(i64::from(seen.men)) * share).trunc() as u32, seen.firing);
        // The line.
        let mut session = session_after(&t, &common::frozen(), day);
        let b = briefing(&mut session, Side::Allied);
        let line = line_of(&b, "land_unit:1").unwrap();
        let condition = seen.share.mending.unwrap().condition;
        assert_eq!(
            condition_of(&line),
            (condition != Fixed::ONE).then_some(condition),
            "the line, morning {day}"
        );
        assert_eq!(
            line["firing_men_now"].as_u64(),
            Some(u64::from(seen.firing))
        );
        // The work: the same unit worked, on that day, by the same share.
        let work_seen = worker.morning(day, V);
        assert_eq!(work_seen.share, seen.share, "morning {day}");
        let made = worker
            .worked(V)
            .into_iter()
            .find(|e| e.day == day)
            .unwrap_or_else(|| panic!("V worked on day {day}"));
        assert_eq!(factor(made, "fighting_share"), share, "work, day {day}");
        assert_eq!(
            factor(made, "condition"),
            seen.share.mending.unwrap().condition
        );
        let EventKind::UnitWorked { work, .. } = &made.kind else {
            unreachable!()
        };
        // 1,000 men, times the share, times 1, times 2 over 2, times 0.5.
        assert_eq!(*work, n(1000) * share * fx("0.5"), "work, day {day}");
    }
}

#[test]
fn property_one_function_in_statement_13() {
    let (t, played) = game_13();
    for (u, count_start) in [(V, 5u32), (E, 0)] {
        let seen = played.morning(1, u);
        assert_eq!(seen.share, seen.truth);
        let fire = played.fire(u)[0];
        assert_eq!(factor(fire, "fighting_share"), seen.share.share);
        assert_eq!(
            factor(fire, "condition"),
            seen.share.mending.unwrap().condition
        );
        assert_eq!(factor(fire, "hunger_nights"), n(i64::from(count_start + 1)));
        let EventKind::GroundFire { firing_men, .. } = &fire.kind else {
            unreachable!()
        };
        assert_eq!(*firing_men, seen.firing, "the fight and the view, unit {u}");
    }
    // The unit's line.
    let mut session = session_after(&t, &common::frozen(), 1);
    let b = briefing(&mut session, Side::Allied);
    let line = line_of(&b, "land_unit:1").unwrap();
    assert_eq!(condition_of(&line), Some(fx("0.94")));
    // The line is read at the start of day 2, after the day's fight and its
    // fatigue: the men, times the fighting share, times the rest of the
    // fatigue, cut once, from the truth at the end of day 1.
    let figures = played.scenario.world.ref_data().ground.as_ref().unwrap();
    let v = &played.end(1).land_units[&unit(V)];
    let share = unit_fighting_share(v, figures).share;
    assert_eq!(share, fx("0.47"));
    let firing = (n(i64::from(v.men_ashore)) * share)
        .mul_div(n(100) - v.fatigue, n(100))
        .trunc();
    assert_eq!(line["firing_men_now"].as_i64(), Some(firing));
}

/// Properties: no new state. The layout version of the state, and the
/// rules label, are what they were, and a recorded game saved before the
/// change loads and replays to its hashes.
#[test]
fn property_no_new_state() {
    // v53 when this was built; decision 0210 raised it to v54 since, for a
    // claim's types in the air, decision 0214 to v55 with the rules label 27,
    // for a force's leave to pursue, and decisions 0211 and 0212 to v56 and
    // 28, for a ship's fire among her aircraft, and decision 0218 the
    // rules label to 29 for the submarine, a boat hashed only where there
    // is one, and its third stage to 30 for the patrol order and a hold on
    // a boat; and decision 0222 to v57 and 31, for a forces limit's
    // `none_of` and a limit that binds the kinds it names; and decision 0223
    // to v58 and 32, for a side's submarine doctrine and the pattern of a
    // patrol, and decision 0228 to v59, for whether a side's staff chooses its
    // sources of supply; none touches this rule.
    assert_eq!(sim_core::hash::STATE_VERSION, b"flagplot-state-v59");
    assert_eq!(sim_core::world::RULES_VERSION, 32);
    let root = common::data_dir().join("..");
    let file = root.join("crates/sim-cli/tests/recorded_games/eastern_solomons_haiku/save.json");
    let saved = sim_cli::save::read(file.to_str().expect("a path")).expect("the save reads");
    let mut scenario = Scenario::load(&root.join(&saved.scenario_file))
        .expect("the scenario loads")
        .with_seed(saved.seed)
        .expect("seeded");
    scenario.replace_script(saved.script.clone());
    scenario.replace_script_inside_day(saved.script_inside_day.clone());
    scenario.replace_commands(saved.commands.clone());
    assert!(!saved.hashes.is_empty());
    for (day, hash) in &saved.hashes {
        for (side, decisions) in saved.staff.get(day).into_iter().flatten() {
            scenario.world.submit_staff(*side, decisions.clone());
        }
        let summary = scenario
            .next_day_watched(Staffs::none(), &mut |_, _| {})
            .expect("the day resolves")
            .expect("the day exists");
        assert_eq!(hash, &format!("{:016x}", summary.state_hash), "day {day}");
    }
}
