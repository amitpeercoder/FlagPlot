//! Scenario 110: the last day ashore. A staff with nothing more coming does
//! not send its men in on the last day of a game.
//!
//! Proves decision 0194. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The setting
//!
//! Scenario 109's, the placeholder scenario of decision 0193: two units
//! ashore on defend with 5,086 fit men that can attack, an estimate of
//! 3,444 enemy men, nothing more coming, at balanced. Its first morning
//! with nothing coming is the morning of day 4, with 3 days resolved. The
//! game has N days, which the test sets: the days left that morning are
//! N - 3. The staff begins no attack with fewer than 3 days left.
//!
//! # Statements
//!
//! 1. **A side reads the game's days.** With N = 6, each side's view
//!    gives 6 as the game's days, and a seat's view gives 6. The figure
//!    itself moves no hash: the same game with N = 6 and with N = 100
//!    has the same daily hashes on every day before either's last days.
//! 2. **Three days left: they go in.** N = 6 (6 - 3 = 3): on the morning
//!    of day 4 the units are set to attack, as scenario 109's statement
//!    3.
//! 3. **Two days left: they do not.** N = 5 (5 - 3 = 2): on the morning
//!    of day 4 they stay on defend. The reason gives the estimate, the
//!    5,086 fit men, the 3,444 wanted, that nothing more is coming, that
//!    2 days are left, and that the staff begins no attack so late. No
//!    notice is handed in.
//! 4. **Press too, and with the men enough.** N = 5 at press with an
//!    estimate of 20,000: they stay on defend. N = 5 at balanced with an
//!    estimate of 2,000, so that 5,086 is more than twice it: they stay
//!    on defend, with the same words. With no estimate held: the same.
//! 5. **Men already in.** N = 7: sent in on day 4 (4 left). On the
//!    morning of day 6 (2 left), their attack of day 5 having failed, the
//!    staff sets them to defend and says so. (The plan's other half, men
//!    whose attack did not fail stay in, has no game that reaches it in
//!    this setting: an attack that does not fail here takes the place
//!    the same day. It is tested on the judgement alone, in the staff's
//!    own tests.)
//! 6. **With three days left or more, as before:** N = 9, a unit still to
//!    come: the staff waits, 6,888 wanted, as scenario 109's statement 7.
//! 7. **A unit that comes after the game is not coming.** N = 6, a third
//!    unit in the take's forces limit that may be used from day 8: on
//!    the morning of day 4 nothing more is coming, and the units go in.
//!    With N = 9 the same unit is coming and the staff waits.
//! 8. **In a trial.** A dry run of the take asked on the evening before
//!    day 4 of the game of 5 days plays its staffs as statement 3: no
//!    world of it has the units sent in. A dry run that plays past day 5
//!    does not fail, and its staffs begin no attack after it.
//!
//! # Properties
//!
//! - On every morning with 3 days left or more, every order and reason of
//!   the judgement ashore is what decision 0193 gives.
//! - With fewer than 3 days left no unit on defend is set to attack.
//! - The days left are never below 0.
//!
//! The tests are written by the build under these statements.

// The architect's doc comment above numbers its statements and properties as
// lists; clippy reads some as lazy continuations. The comment is not changed.
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::doc_overindented_list_items)]

// ---------------------------------------------------------------- The harness
//
// Scenario 109's file, data/scenarios/men-ashore-no-more-coming/
// 109_men_ashore_with_no_more_coming.toml, on the frozen step 1 figures,
// with its `days` replaced by N (the test's variants are made from its
// text). Its morning M, the first morning of the judgement ashore with
// nothing more coming, is day 4: 3 days are resolved, so N - 3 days are
// left. The test reads the staff's orders and reasons handed in for J.

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{NoticeKind, Reason, StaffDecisions};
use sim_core::events::Event;
use sim_core::ids::{LandUnitId, TaskForceId};
use sim_core::orders::{LandUnitOrder, OrderKind, Posture};
use sim_core::state::{Side, TruthState};
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, staffs};
use sim_trial::{Asked, Believed};

mod common;

/// The morning of the first judgement ashore (3 days resolved).
const M: u32 = 4;

fn path() -> PathBuf {
    common::data_dir()
        .join("scenarios/men-ashore-no-more-coming/109_men_ashore_with_no_more_coming.toml")
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

fn load_with(text: &str, options: &LoadOptions) -> Scenario {
    common::from_text_with(text, &path(), options).expect("the scenario loads")
}

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    #[allow(dead_code)]
    events: Vec<Event>,
}

fn play(text: &str) -> Played {
    play_with(text, &common::frozen())
}

fn play_with(text: &str, options: &LoadOptions) -> Played {
    play_capped(text, options, u32::MAX)
}

/// The game played for at most `cap` days.
fn play_capped(text: &str, options: &LoadOptions, cap: u32) -> Played {
    let held = staffs();
    let mut scenario = load_with(text, options);
    let mut handed = BTreeMap::new();
    let mut truth = vec![scenario.world.umpire().clone()];
    loop {
        let day = scenario.world.day() + 1;
        if day > cap {
            break;
        }
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        handed.insert(
            day,
            scenario
                .world
                .staff_handed_in()
                .get(&Side::Japanese)
                .cloned()
                .unwrap_or_default(),
        );
        truth.push(scenario.world.umpire().clone());
    }
    Played {
        handed,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
    }
}

fn text_of(reason: &Reason) -> String {
    format!(
        "{} | {} | {}",
        reason.worked,
        reason.binding.join("; "),
        reason
            .evidence
            .iter()
            .map(|e| e.what.clone())
            .collect::<Vec<_>>()
            .join("; ")
    )
}

fn assert_says(all: &str, parts: &[&str]) {
    for part in parts {
        assert!(all.contains(part), "the reason says {part:?}: {all}");
    }
}

fn assert_silent(all: &str, parts: &[&str]) {
    for part in parts {
        assert!(
            !all.contains(part),
            "the reason does not say {part:?}: {all}"
        );
    }
}

impl Played {
    /// Every reason of J's staff on `day`, of orders and of decisions to
    /// give no order, in the words the staff wrote.
    fn reasons(&self, day: u32) -> String {
        let d = &self.handed[&day];
        d.orders
            .iter()
            .map(|o| text_of(&o.reason))
            .chain(d.no_orders.iter().map(text_of))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// The postures J's staff orders on `day`, by unit.
    fn postures(&self, day: u32) -> Vec<(u32, Posture)> {
        self.handed[&day]
            .orders
            .iter()
            .filter_map(|o| match &o.kind {
                OrderKind::LandUnit {
                    target,
                    order: LandUnitOrder::SetPosture { posture, .. },
                } => Some((target.raw(), *posture)),
                _ => None,
            })
            .collect()
    }

    /// Every posture the staff orders, with its day.
    fn all_postures(&self) -> Vec<(u32, u32, Posture)> {
        self.handed
            .keys()
            .flat_map(|d| self.postures(*d).into_iter().map(move |(u, p)| (*d, u, p)))
            .collect()
    }

    fn notices(&self, day: u32) -> Vec<NoticeKind> {
        self.handed[&day].notices.iter().map(|n| n.kind).collect()
    }

    fn cannot_bring_enough(&self, day: u32) -> usize {
        self.notices(day)
            .iter()
            .filter(|k| **k == NoticeKind::CannotBringEnough)
            .count()
    }

    /// The state of truth at the end of `day`.
    fn after(&self, day: u32) -> &TruthState {
        &self.truth[day as usize]
    }

    /// A land unit's posture at the end of `day`.
    fn posture_after(&self, day: u32, unit: u32) -> Posture {
        self.after(day).land_units[&LandUnitId::new(unit)].posture
    }
}

// ------------------------------------------------------------------ Variants

/// J's estimate of the enemy at P.
fn estimate(t: &str, men: u32) -> String {
    changed(
        t,
        "men = 3444\n\n# MARK: estimate",
        &format!("men = {men}\n\n# MARK: estimate"),
    )
}

/// J holds no estimate of the enemy at P.
fn no_estimate(t: &str) -> String {
    changed(
        t,
        "[[estimate]]\nside = \"japanese\"\nq = 14\nr = 0\nmen = 3444\n\n",
        "",
    )
}

/// The risk from the second day.
fn risk(t: &str, level: &str) -> String {
    changed(t, "risk = \"balanced\"", &format!("risk = \"{level}\""))
}

fn w_at_source(t: &str) -> String {
    let t = at(
        t,
        "units",
        "[[land_unit]]\nid = 3\nname = \"Placeholder W\"\nside = \"japanese\"\nmen = 2000\nq = 0\nr = 0\n\n",
    );
    changed(&t, "land_units = [1, 2]", "land_units = [1, 2, 3]")
}

/// The judgement ashore of `day`: the reason of the staff that carries it.
fn judgement(g: &Played, day: u32) -> String {
    g.reasons(day)
        .lines()
        .find(|l| l.contains("the units that can attack have"))
        .unwrap_or_else(|| panic!("a judgement ashore on day {day}: {}", g.reasons(day)))
        .to_string()
}

/// The whole number that follows `marker` in `text`.
fn number_after(text: &str, marker: &str) -> u32 {
    let at = text
        .find(marker)
        .unwrap_or_else(|| panic!("{marker:?} in {text}"))
        + marker.len();
    text[at..]
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .expect("a number")
}

/// The men wanted while more is coming, as the judgement words them first.
fn wanted_while_more_is_coming(judgement: &str) -> u32 {
    number_after(judgement, "for each, ")
}

/// The men wanted with nothing more coming, where the judgement says so:
/// 0 for "none".
fn wanted_with_nothing_coming(judgement: &str) -> Option<u32> {
    let marker = "with nothing more coming, the staff wants ";
    let at = judgement.find(marker)? + marker.len();
    let rest = &judgement[at..];
    if rest.starts_with("none") {
        Some(0)
    } else {
        Some(number_after(rest, "for each, "))
    }
}

/// The men wanted that the staff worked to, whichever way it counted.
fn wanted(judgement: &str) -> u32 {
    wanted_with_nothing_coming(judgement).unwrap_or_else(|| wanted_while_more_is_coming(judgement))
}

// ------------------------------------------------------------------ Variants of this file

/// The game of N days. A game of one day has no day 2, so the command of
/// day 2 (the risk, balanced) is taken out of it; the risk of its only day
/// is press, as the file gives it for day 1.
fn days(t: &str, n: u32) -> String {
    let mut t = t.to_string();
    if n < 2 {
        let start = t
            .find("[[command]]\nday = 2\n")
            .expect("the command of day 2");
        let end = start + t[start..].find("\n\n").expect("its end") + 2;
        t.replace_range(start..end, "");
    }
    changed(&t, "days = 7", &format!("days = {n}"))
}

/// The game with command levels on, so that a seat has a view.
fn with_levels(t: &str) -> String {
    changed(
        t,
        "placeholder = true\n",
        "placeholder = true\ncommand_levels = true\n",
    )
}

/// W, 2,000 men, ashore at the source, may be used from day `from`; the
/// take is given no last day of its own.
fn w_from(t: &str, from: u32) -> String {
    changed(
        &w_at_source(t),
        "men = 2000\nq = 0\nr = 0\n",
        &format!("men = 2000\nq = 0\nr = 0\navailable_from = {from}\n"),
    )
}

/// The daily hashes of the game of the text, for at most `cap` days.
fn daily_hashes(text: &str, cap: u32) -> Vec<u64> {
    let held = staffs();
    let mut scenario = load_with(text, &common::frozen());
    while scenario.world.day() < cap {
        if scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves")
            .is_none()
        {
            break;
        }
    }
    scenario.world.day_hashes().to_vec()
}

fn hex16(hashes: &[u64]) -> Vec<String> {
    hashes.iter().map(|h| format!("{h:016x}")).collect()
}

/// Whether the reasons of `day` say the days left (either wording).
fn says_days_left(g: &Played, day: u32) -> bool {
    let r = g.reasons(day);
    r.contains("days of the game are left") || r.contains("1 day of the game is left")
}

/// Every figure the reasons of `day` give as the days left.
fn days_left_said(g: &Played, day: u32) -> Vec<String> {
    let mut found = Vec::new();
    let r = g.reasons(day);
    for marker in [" days of the game are left", " day of the game is left"] {
        let mut from = 0;
        while let Some(at) = r[from..].find(marker) {
            let end = from + at;
            let word: String = r[..end]
                .chars()
                .rev()
                .take_while(|c| c.is_ascii_digit() || *c == '-')
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            found.push(word);
            from = end + marker.len();
        }
    }
    found
}

fn left(n: u32, day: u32) -> u32 {
    n.saturating_sub(day - 1)
}

// ------------------------------------------------------------------ Statement 1

#[test]
fn statement_1_a_side_reads_the_games_days() {
    use sim_core::pictures::CommandRef;
    // Each side's view gives 6.
    let scenario = load_with(&days(&text(), 6), &common::frozen());
    for side in [Side::Allied, Side::Japanese] {
        assert_eq!(scenario.world.side_view(side).game_days(), Some(6));
    }
    // A seat's view gives 6. The test uses scenario 109's file with
    // `command_levels = true` and the seat of task force 1 (J's lift), as
    // the view's own `at_seat` gives it; and the held view (what the area
    // command is known to hold) gives it too.
    let levels = load_with(&with_levels(&days(&text(), 6)), &common::frozen());
    let view = levels.world.side_view(Side::Japanese);
    let seat = view
        .at_seat(CommandRef::TaskForce(TaskForceId::new(1)))
        .expect("a seat at task force 1");
    assert_eq!(seat.game_days(), Some(6));
    let held = view.known_to(CommandRef::Area);
    assert_eq!(held.view().game_days(), Some(6));
    // Another length is read as it is.
    let nine = load_with(&days(&text(), 9), &common::frozen());
    assert_eq!(nine.world.side_view(Side::Japanese).game_days(), Some(9));
}

/// The daily hashes of scenario 109's file for days 1 to 4, on the frozen
/// step 1 figures: what the game gave before this rule (the same for every
/// length of the game, which the hash does not hold), and with N = 6 what it
/// gives now. They are the values `sim-cli hash` printed before this rule,
/// remade for the state's layout alone at version 52 (decision 0201) and at
/// version 53 (decision 0205), at version 54 (decision 0210) and at version
/// 55 (decision 0212), none of which moved an event of the game.
const PRE_RULE: [&str; 6] = [
    "860da2b364b95e28",
    "ed9ce323c38f5333",
    "872edb8c608e9b46",
    "2781664c089403e7",
    "f3a329cc377461dc",
    "58aed02695b13047",
];

#[test]
fn statement_1_daily_hashes_before_the_last_days_are_those_before_the_rule() {
    let six = daily_hashes(&days(&text(), 6), 6);
    assert_eq!(hex16(&six[..4]), PRE_RULE[..4].to_vec());
}

#[test]
fn statement_1_the_games_length_is_not_in_the_hash() {
    // N = 6 and N = 100: the same hashes on days 1 to 4, before any morning
    // in the last days (the rule 12 point). And N = 7 against N = 100 on
    // days 1 to 5.
    let six = daily_hashes(&days(&text(), 6), 4);
    let hundred = daily_hashes(&days(&text(), 100), 7);
    assert_eq!(six[..4], hundred[..4]);
    let seven = daily_hashes(&days(&text(), 7), 5);
    assert_eq!(seven[..5], hundred[..5]);
    // And what a game of many days gives is what the game gave before this
    // rule, day for day.
    assert_eq!(hex16(&hundred[..6]), PRE_RULE.to_vec());
    assert_eq!(format!("{:016x}", hundred[6]), "8bf7552afec94e3f");
}

// ------------------------------------------------------------------ Statement 2

const BOTH_ATTACK: [(u32, Posture); 2] = [(1, Posture::Attack), (2, Posture::Attack)];

#[test]
fn statement_2_three_days_left_they_go_in() {
    // N = 6: 6 - 3 = 3 days left on the morning of day 4; as scenario 109's
    // statement 3.
    let g = play(&days(&text(), 6));
    assert_eq!(g.postures(M), BOTH_ATTACK.to_vec());
    let j = judgement(&g, M);
    assert_says(
        &j,
        &[
            "at balanced, with nothing more coming, the staff wants 1 man for each, 3444 men",
            "the units that can attack have 5086 fit men, at least the 3444 wanted",
            "nothing more is coming",
        ],
    );
    assert_silent(&j, &["of the game are left", "of the game is left"]);
    for order in &g.handed[&M].orders {
        let r = text_of(&order.reason);
        assert_says(
            &r,
            &["Nothing more is coming and the men that can attack are enough for it"],
        );
        assert_silent(&r, &["days of the game"]);
    }
    assert_eq!(g.cannot_bring_enough(M), 0);
    assert_eq!(g.posture_after(M, 1), Posture::Attack);
    assert_eq!(g.posture_after(M, 2), Posture::Attack);
}

// ------------------------------------------------------------------ Statement 3

#[test]
fn statement_3_two_days_left_they_do_not() {
    // N = 5: 5 - 3 = 2 days left on the morning of day 4.
    let g = play(&days(&text(), 5));
    assert_eq!(g.postures(M), vec![]);
    let j = judgement(&g, M);
    assert_says(
        &j,
        &[
            "the side's estimate is 3444 enemy men at (14,0)",
            "the units that can attack have 5086 fit men, at least the 3444 wanted",
            "nothing more is coming",
            "2 days of the game are left, fewer than 3",
            "the staff begins no attack ashore, because an attack begun so late only costs men",
        ],
    );
    assert_eq!(g.cannot_bring_enough(M), 0);
    assert!(g.handed[&M].notices.is_empty());
    assert_eq!(g.posture_after(M, 1), Posture::Defend);
    assert_eq!(g.posture_after(M, 2), Posture::Defend);
}

// ------------------------------------------------------------------ Statement 4

fn assert_waits_for_the_days(g: &Played, what: &str) {
    assert_eq!(g.postures(M), vec![], "{what}");
    assert_eq!(g.cannot_bring_enough(M), 0, "{what}");
    assert!(g.handed[&M].notices.is_empty(), "{what}");
    assert_says(
        &judgement(g, M),
        &[
            "2 days of the game are left, fewer than 3",
            "the staff begins no attack ashore, because an attack begun so late only costs men",
        ],
    );
    assert_eq!(g.posture_after(M, 1), Posture::Defend, "{what}");
    assert_eq!(g.posture_after(M, 2), Posture::Defend, "{what}");
}

#[test]
fn statement_4_press_too() {
    let g = play(&estimate(&risk(&days(&text(), 5), "press"), 20_000));
    assert_waits_for_the_days(&g, "press, estimate 20000");
    assert_says(
        &judgement(&g, M),
        &[
            "the side's estimate is 20000 enemy men at (14,0)",
            "at press, with nothing more coming, the staff wants none",
        ],
    );
}

#[test]
fn statement_4_and_with_the_men_enough() {
    // 5,086 is more than twice 2,000.
    let g = play(&estimate(&days(&text(), 5), 2_000));
    assert_waits_for_the_days(&g, "balanced, estimate 2000");
    assert_says(
        &judgement(&g, M),
        &["the side's estimate is 2000 enemy men at (14,0)"],
    );
}

#[test]
fn statement_4_with_no_estimate_held() {
    let g = play(&no_estimate(&days(&text(), 5)));
    assert_waits_for_the_days(&g, "no estimate");
    assert_says(
        &g.reasons(M),
        &["the side holds no estimate of the enemy's men at (14,0)"],
    );
}

// ------------------------------------------------------------------ Statement 5

#[test]
fn statement_5_men_already_in_are_set_to_defend_when_their_attack_failed() {
    // N = 7: sent in on day 4 (4 left); nothing on day 5 (3 left).
    let g = play(&days(&text(), 7));
    assert_eq!(g.postures(M), BOTH_ATTACK.to_vec());
    assert_eq!(g.postures(5), vec![]);
    assert_silent(&g.reasons(5), &["of the game are left"]);
    // The morning of day 6 (2 left), the attack of day 5 having failed: set
    // to defend, and the reason says so.
    assert_eq!(
        g.postures(6),
        vec![(1, Posture::Defend), (2, Posture::Defend)]
    );
    let r = g.reasons(6);
    assert_says(
        &r,
        &[
            "2 days of the game are left, fewer than 3",
            "the side's attack at (14,0) failed on day 5",
            "The side's attack failed yesterday and the game is in its last days: an attack so late only costs men, so the staff sets land_unit:1 (Placeholder U) to defend, where it rests",
            "so the staff sets land_unit:2 (Placeholder V) to defend, where it rests",
        ],
    );
    assert_eq!(g.posture_after(6, 1), Posture::Defend);
    assert_eq!(g.posture_after(6, 2), Posture::Defend);
    // The morning of day 7 (1 left): they are on defend, none is set to
    // attack.
    assert_eq!(g.postures(7), vec![]);
    assert_says(&g.reasons(7), &["1 day of the game is left, fewer than 3"]);
}

// ------------------------------------------------------------------ Statement 6

#[test]
fn statement_6_with_three_days_left_or_more_as_before() {
    // N = 9, W (2,000 men) ashore at the source: more is coming, the men
    // wanted are 6,888 and the staff waits, as scenario 109's statement 7,
    // on every morning with 3 or more days left (days 4 to 7, 6 to 3 left).
    let g = play(&days(&w_at_source(&text()), 9));
    for day in M..=7 {
        let j = judgement(&g, day);
        assert_eq!(wanted(&j), 6_888, "day {day}");
        assert_silent(&j, &["nothing more is coming", "of the game are left"]);
        assert_eq!(g.postures(day), vec![], "day {day}");
        assert_eq!(g.cannot_bring_enough(day), 0, "day {day}");
    }
    assert_says(
        &judgement(&g, M),
        &[
            "left behind at the source: land_unit:3 (Placeholder W) (2000 men)",
            "short by 1802",
        ],
    );
}

// ------------------------------------------------------------------ Statement 7

#[test]
fn statement_7_a_unit_that_comes_after_the_game_is_not_coming() {
    // N = 6, W may be used from day 8, after the game's last day, 6: on the
    // morning of day 4 nothing more is coming and the units go in.
    let g = play(&days(&w_from(&text(), 8), 6));
    assert_eq!(g.postures(M), BOTH_ATTACK.to_vec());
    let j = judgement(&g, M);
    assert_says(
        &j,
        &[
            "nothing more is coming",
            "land_unit:3 (Placeholder W) may be used from day 8, after the game's last day, day 6",
            "at least the 3444 wanted",
        ],
    );
    assert_eq!(wanted(&j), 3_444);
    assert_eq!(g.cannot_bring_enough(M), 0);
}

#[test]
fn statement_7_with_nine_days_the_same_unit_is_coming() {
    // N = 9: day 8 is inside the game: more is coming, the staff waits (the
    // men wanted are 6,888) on every morning with 3 or more days left.
    let g = play(&days(&w_from(&text(), 8), 9));
    for day in M..=7 {
        let j = judgement(&g, day);
        assert_eq!(wanted(&j), 6_888, "day {day}");
        assert_silent(&j, &["nothing more is coming"]);
        assert_eq!(g.postures(day), vec![], "day {day}");
        assert_eq!(g.cannot_bring_enough(day), 0, "day {day}");
    }
    assert_says(
        &judgement(&g, M),
        &["land_unit:3 (Placeholder W) may be used from day 8"],
    );
    assert_silent(&judgement(&g, M), &["after the game's last day"]);
}

// ------------------------------------------------------------------ Statement 8

// ------------------------------------------------------------------ Statement 9

/// The scenario with the files a trial needs named: the file of sides of
/// scenario 94 (every class and type to both sides) and the trial's
/// figures, `trial.toml`, both from the reference folder and each with its
/// frozen copy.
fn with_trial_files(t: &str) -> String {
    changed(
        t,
        "ground = \"../../reference/ground_placeholder_scenario_109.toml\"\n",
        "ground = \"../../reference/ground_placeholder_scenario_109.toml\"\nclass_sides = \"../../reference/class_sides_placeholder_scenario_94.csv\"\ntrial = \"../../reference/trial.toml\"\n",
    )
}

/// The game of `n` days played for `resolved` days, with the trial's files.
fn live_game(n: u32, resolved: u32) -> Scenario {
    let held = staffs();
    let mut scenario = load_with(&with_trial_files(&days(&text(), n)), &common::frozen());
    for _ in 0..resolved {
        scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves")
            .expect("a day to resolve");
    }
    scenario
}

fn believed_of(scenario: &Scenario) -> Believed {
    Believed {
        forces: scenario.believed_forces(Side::Japanese).cloned().collect(),
        intents: scenario.believed_intents(Side::Japanese).cloned().collect(),
        men: scenario.believed_men(Side::Japanese).cloned().collect(),
    }
}

/// A trial of J from the evening of day `resolved` of the game of `n` days:
/// 2 worlds, `horizon` days, an empty course (the take as it stands).
fn trial_of(n: u32, resolved: u32, horizon: u32) -> sim_trial::Played {
    let scenario = live_game(n, resolved);
    let figures = scenario.trial_figures().cloned().expect("trial figures");
    sim_trial::play(
        &scenario.world.side_view(Side::Japanese),
        &believed_of(&scenario),
        &figures,
        scenario.world.ref_data(),
        scenario.world.map(),
        Asked {
            trial: 1,
            worlds: 2,
            days: horizon,
            course: vec![],
        },
    )
    .expect("the trial begins")
}

/// The postures J's staff orders in a played day of a trial.
fn trial_postures(day: &sim_trial::PlayedDay) -> Vec<(u32, Posture)> {
    day.staff[&Side::Japanese]
        .orders
        .iter()
        .filter_map(|o| match &o.kind {
            OrderKind::LandUnit {
                target,
                order: LandUnitOrder::SetPosture { posture, .. },
            } => Some((target.raw(), *posture)),
            _ => None,
        })
        .collect()
}

fn trial_reasons(day: &sim_trial::PlayedDay) -> String {
    let d = &day.staff[&Side::Japanese];
    d.orders
        .iter()
        .map(|o| text_of(&o.reason))
        .chain(d.no_orders.iter().map(text_of))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn statement_9_the_pictures_and_worlds_of_a_trial_carry_the_games_days() {
    let scenario = live_game(5, 3);
    assert_eq!(scenario.world.day(), 3);
    let figures = scenario.trial_figures().cloned().expect("trial figures");
    let made = sim_trial::pictures(
        &scenario.world.side_view(Side::Japanese),
        &believed_of(&scenario),
        &figures,
        sim_trial::Trial {
            seed: figures.seed,
            trial: 1,
            worlds: 2,
        },
    )
    .expect("the pictures are made");
    assert_eq!(made.len(), 2);
    for picture in &made {
        assert_eq!(picture.picture.game_days, Some(5));
    }
    let played = trial_of(5, 3, 1);
    assert!(played.failed().is_empty());
    for w in &played.worlds {
        assert_eq!(w.world.as_ref().expect("a world").game_days(), Some(5));
    }
}

#[test]
fn statement_9_a_trial_plays_its_staffs_as_statement_3() {
    // Asked on the evening of day 3 of the game of 5 days (the morning of day
    // 4 is the next, with 2 days left), for 1 day and for 2: no world has U
    // or V sent in.
    for horizon in [1, 2] {
        let played = trial_of(5, 3, horizon);
        assert!(played.failed().is_empty());
        assert_eq!(played.worlds.len(), 2);
        for w in &played.worlds {
            assert_eq!(w.days.len() as u32, horizon);
            for d in &w.days {
                assert_eq!(
                    trial_postures(d),
                    vec![],
                    "world {} day {}",
                    w.number,
                    d.day
                );
                assert!(d.staff[&Side::Japanese].notices.is_empty());
            }
            assert_says(
                &trial_reasons(&w.days[0]),
                &[
                    "the units that can attack have 5086 fit men, at least the 3444 wanted",
                    "nothing more is coming",
                    "2 days of the game are left, fewer than 3",
                    "the staff begins no attack ashore, because an attack begun so late only costs men",
                ],
            );
            assert_eq!(w.days[0].day, 4);
        }
    }
}

#[test]
fn statement_9_a_trial_that_plays_past_the_games_end_does_not_fail_and_begins_no_attack_after_it() {
    // From day 3, 4 days: to day 7, 2 days past the game's last, day 5.
    let played = trial_of(5, 3, 4);
    assert!(played.failed().is_empty());
    for w in &played.worlds {
        assert_eq!(
            w.days.iter().map(|d| d.day).collect::<Vec<_>>(),
            vec![4, 5, 6, 7]
        );
        for d in &w.days {
            assert_eq!(
                trial_postures(d),
                vec![],
                "world {} day {}",
                w.number,
                d.day
            );
        }
        // After the game's last day the days left are 0, never less.
        for d in &w.days[2..] {
            let r = trial_reasons(d);
            assert_says(&r, &["0 days of the game are left, fewer than 3"]);
            assert_says(&r, &["the staff begins no attack ashore"]);
            assert_silent(&r, &["-1 day", "-2 day"]);
        }
        assert_says(
            &trial_reasons(&w.days[1]),
            &["1 day of the game is left, fewer than 3"],
        );
    }
}

#[test]
fn statement_9_control_with_three_days_left_a_trial_sends_them_in() {
    // The game of 6 days, asked from the same evening: 3 days left on the
    // morning of day 4, so the staffs of the worlds send U and V in.
    let played = trial_of(6, 3, 1);
    assert!(played.failed().is_empty());
    for w in &played.worlds {
        assert_eq!(
            trial_postures(&w.days[0]),
            BOTH_ATTACK.to_vec(),
            "world {}",
            w.number
        );
    }
}

// --------------------------------------------------------------- Properties

/// Scenario 109's file made into its variants: the three risks with the
/// estimates 2,000, 3,444, 5,200 and 20,000, and the three risks with no
/// estimate held, each named.
fn variants() -> Vec<(String, String)> {
    let mut all = Vec::new();
    for level in ["preserve", "balanced", "press"] {
        for men in [2_000, 3_444, 5_200, 20_000] {
            all.push((
                format!("{level}, estimate {men}"),
                estimate(&risk(&text(), level), men),
            ));
        }
        all.push((
            format!("{level}, no estimate"),
            no_estimate(&risk(&text(), level)),
        ));
    }
    all
}

/// The games' lengths swept.
const LENGTHS: std::ops::RangeInclusive<u32> = 1..=9;

#[test]
fn property_with_three_days_left_or_more_every_order_and_reason_is_decision_0193s() {
    // The same variant at N = 40, where no morning before day 38 is in the
    // last days, is decision 0193's judgement: each morning with 3 days left
    // or more, day <= N - 2, hands in the same orders, reasons and notices.
    for (name, t) in variants() {
        let large = play_capped(&days(&t, 40), &common::frozen(), 7);
        for n in LENGTHS {
            let g = play(&days(&t, n));
            for day in 1..=n.saturating_sub(2).min(7) {
                assert!(left(n, day) >= 3);
                assert_eq!(
                    g.handed[&day], large.handed[&day],
                    "{name}: N={n} day {day}"
                );
                assert!(!says_days_left(&g, day), "{name}: N={n} day {day}");
            }
        }
    }
}

#[test]
fn property_with_fewer_than_three_days_left_no_unit_on_defend_is_set_to_attack() {
    for (name, t) in variants() {
        for n in LENGTHS {
            let g = play(&days(&t, n));
            for (day, unit, posture) in g.all_postures() {
                if posture == Posture::Attack {
                    assert!(
                        left(n, day) >= 3,
                        "{name}: N={n}: unit {unit} set to attack on day {day} with {} left",
                        left(n, day)
                    );
                }
            }
        }
    }
}

#[test]
fn property_the_days_left_are_never_below_0_and_are_the_days_the_games_has_left() {
    let mut mornings_said = 0;
    for (name, t) in variants() {
        for n in LENGTHS {
            let g = play(&days(&t, n));
            for day in 1..=n {
                for word in days_left_said(&g, day) {
                    let said: u32 = word.parse().unwrap_or_else(|_| {
                        panic!("{name}: N={n} day {day}: {word:?} is not a whole number")
                    });
                    assert_eq!(said, left(n, day), "{name}: N={n} day {day}");
                    assert!(said < 3, "{name}: N={n} day {day}");
                    mornings_said += 1;
                }
            }
        }
    }
    assert!(mornings_said > 0);
    // And past the game's end, in a trial: never below 0.
    let played = trial_of(5, 3, 4);
    for w in &played.worlds {
        for d in &w.days {
            let r = trial_reasons(d);
            assert!(!r.contains("-"), "world {} day {}: {r}", w.number, d.day);
            assert!(
                r.contains(&format!(
                    "{} days of the game are left",
                    5u32.saturating_sub(d.day - 1)
                )) || r.contains("1 day of the game is left")
            );
        }
    }
}
