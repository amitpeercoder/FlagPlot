//! Scenario 109: men ashore with no more coming. Men landed to take a
//! place, too few for what the risk asks and with nothing more to send,
//! attack, or wait and say so, by the risk.
//!
//! Proves decision 0193. The arithmetic is the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The sums
//!
//! fit men of a unit = men ashore x (100 - fatigue) / 100, cut to a whole
//! man. Men wanted = the estimate x 3, 2 or 1 by the risk while more is
//! coming; x 3, 1 or 0 with nothing more coming.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on the placeholder map, with
//! no real unit. Side J has taken a transport force to a coast hex P that
//! side A holds, under "take the place P", and landed two units: U, 3,000
//! men at fatigue 10, and V, 2,386 men at fatigue 0, both on defend.
//! Both can attack: each one's fatigue is at or below two thirds of the
//! reference file's attack fatigue limit, which the test reads and
//! states.
//! J's estimate of the enemy's men at P, made by its men in contact, is
//! 3,444. The transports lie empty. No unit of the take is left behind
//! at the source.
//!
//! Fit men: U, 3,000 x 90 / 100 = 2,700; V, 2,386. Together 5,086.
//!
//! # Statements
//!
//! 1. **Today, at balanced:** men wanted 3,444 x 2 = 6,888; 5,086 is
//!    fewer; the staff waits. (Before this rule it waited every
//!    morning.)
//! 2. **Nothing more is coming** on that morning, and the reason says
//!    what was looked for: no lift of the take with fighting men aboard,
//!    and no fighting unit of its forces limit off the place that can be
//!    used and carried by the end's last day.
//! 3. **Balanced, with nothing more coming:** men wanted 3,444 x 1 =
//!    3,444; 5,086 is at least that; U and V are set to attack. The
//!    reason gives the estimate and its day, 3,444 wanted, 2,700 and
//!    2,386, 5,086, and that nothing more is coming. (Restated by the
//!    architect on Nov 20, 2026, for decision 0194: the game is 7 days,
//!    so on the morning of day 6 two days are left; the attack of day 5
//!    failed, and the staff sets U and V back to defend that morning.
//!    Before it they stayed in to the end.)
//! 4. **Balanced, too few even so.** The estimate is 5,200: 5,086 is
//!    fewer. The staff waits and hands in the notice
//!    `cannot_bring_enough`, with
//!    the estimate, the 5,086, the 5,200 wanted and that nothing more is
//!    coming. It hands it in again the next morning.
//! 5. **Press, with nothing more coming:** U and V are set to attack with
//!    an estimate of 5,200, and with one of 20,000.
//! 6. **Preserve the force waits:** men wanted 3,444 x 3 = 10,332; it
//!    waits and hands in `cannot_bring_enough`.
//! 7. **While more is coming, as today.** A third unit, W, 2,000 men, in
//!    the take's forces limit, is ashore at the source and the
//!    transports lie there empty: a second lift is to be sent. At balanced the men wanted are 6,888 and the
//!    staff waits, without the notice.
//! 8. **A lift at sea is more coming.** W is aboard the transports under
//!    way to P: the men wanted are 6,888 and the staff waits.
//! 9. **A lift turned back is more coming** (decision 0191): W aboard,
//!    its route ending at the source: as statement 8.
//! 9a. **A unit before its day is coming.** W may be used from a day
//!     three days on, inside the end's last day: more is coming, the men
//!     wanted are 6,888 and the staff waits without the notice.
//! 9b. **A unit no ship can carry is not coming.** W is ashore at the
//!     source and every transport of the take is sunk: nothing more is
//!     coming, and statement 3 holds.
//! 9c. **Once sent in, they stay in.** After statement 3, the next
//!     morning's estimate is 9,000. U and V are on attack and the staff
//!     does not set them to defend; its reason gives the estimate and
//!     says they are committed.
//! 10. **A unit that cannot attack is not counted.** U, on defend, has a
//!     fatigue above the balanced mark. The fit men that can attack are V's 2,386,
//!     fewer than 3,444: the staff waits and hands in the notice.
//! 11. **No estimate, as today:** the units attack and the reason says it
//!     attacks with no count of the enemy.
//! 12. **The commander is told.** With the standing interrupt for the
//!     notice on, a seat's day stops on the morning of statement 4.
//!     Setting press on the intent has the units attack the next morning.
//! 13. **Truth and belief.** With the defenders doubled in truth and J's
//!     estimate left at 3,444, statement 3's orders are the same.
//!
//! # Properties
//!
//! - While more is coming, every order and reason of the judgement ashore
//!   is what it was before this rule.
//! - At preserve the force no unit is ever set to attack by this rule
//!   that would not have been before it.
//! - The men wanted with nothing more coming are never more than while
//!   more is coming.
//! - More fit men never turn an attack into a wait.
//! - A larger estimate never turns a wait into an attack.
//! - A lift added never lowers the men wanted.
//!
//! The tests are written by the build under these statements.

// The architect's doc comment above numbers statements "9a.", "9b." and
// "9c."; clippy reads them as list items. The comment is not changed.
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::doc_overindented_list_items)]

// ---------------------------------------------------------------- The harness
//
// How the scenario is set up (data/scenarios/men-ashore-no-more-coming/
// 109_men_ashore_with_no_more_coming.toml, on scenario 107's placeholder map,
// data/map/placeholder_a_lift_turned_back.toml):
//
// - Side J is the Japanese side, side A the Allied side. J's base 1 is S at
//   (0,0); A's base 2 is P at (14,0), a coast hex with an Allied garrison of
//   3,444 men (land unit 10) on defend.
// - L is task force 1: nine placeholder troop transports (600 men each), at
//   sea at (2,0), with U (land unit 1, 3,000 men, fatigue 10) and V (land
//   unit 2, 2,386 men, fatigue 0) aboard and nothing else. J's estimate of
//   A's men at P is the opening one, 3,444 (day 0).
// - J's commands on day 1: take the place P, forces L and units 1 and 2 with
//   S as the source, no count of the enemy asked for, both leaves given, at
//   press (so that the lift is sent: a lift short of the men wanted at
//   balanced with none to follow it to feed it is not). From day 2 the risk
//   is balanced (or the level a variant gives).
// - L is at (8,0) on the morning of day 2, at P on the morning of day 3, and
//   U and V are ashore on defend, with L on its way home, on the morning of
//   day 4: that morning is M, the first morning of the judgement ashore.
//   Neither side attacks but by the staff's order.
// - The ground figures are scenario 109's file: the frozen step 1 copy with
//   rest_per_day 0 (a unit's fatigue stays what it is), unfed_loss_share 0,
//   preparation_per_day 100 and no ground report (no report error figure), so
//   the opening estimate stands for the whole game. The attack fatigue limit
//   is the reference file's 60; the mark of balanced is two thirds of it, 40;
//   U's 10 and V's 0 are at or below it. A test of statement 9c gives the
//   figures a ground report and a fight of its own.
// - A "morning" of a statement is the staff's step at the start of a day.

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{
    InterruptRef, NoticeBasis, NoticeKind, Reason, StaffDecisions, StandingKind,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::ids::{LandUnitId, TaskForceId};
use sim_core::orders::{LandUnitOrder, OrderKind, Posture};
use sim_core::state::{Side, TruthState};
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, staffs};

mod common;

/// The morning of the first judgement ashore.
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
    events: Vec<Event>,
}

fn play(text: &str) -> Played {
    play_with(text, &common::frozen())
}

fn play_with(text: &str, options: &LoadOptions) -> Played {
    let held = staffs();
    let mut scenario = load_with(text, options);
    let mut handed = BTreeMap::new();
    let mut truth = vec![scenario.world.umpire().clone()];
    loop {
        let day = scenario.world.day() + 1;
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

    /// The reasons the notices of `day` rest on.
    fn notice_reasons(&self, day: u32, kind: NoticeKind) -> Vec<String> {
        let d = &self.handed[&day];
        d.notices
            .iter()
            .filter(|n| n.kind == kind)
            .filter_map(|n| match n.rests_on {
                NoticeBasis::NoOrder(i) => d.no_orders.get(i).map(text_of),
                _ => None,
            })
            .collect()
    }

    /// The state of truth at the end of `day`.
    fn after(&self, day: u32) -> &TruthState {
        &self.truth[day as usize]
    }

    /// A land unit's posture at the end of `day`.
    fn posture_after(&self, day: u32, unit: u32) -> Posture {
        self.after(day).land_units[&LandUnitId::new(unit)].posture
    }

    fn men_ashore_after(&self, day: u32, unit: u32) -> u32 {
        self.after(day).land_units[&LandUnitId::new(unit)].men_ashore
    }

    fn interrupts_of(&self, kind: StandingKind) -> Vec<u32> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::InterruptFired {
                    side: Side::Japanese,
                    interrupt: InterruptRef::Standing { kind: k, .. },
                    ..
                } if *k == kind => Some(e.day),
                _ => None,
            })
            .collect()
    }

    /// Whether J's staff orders any unit to attack on `day`.
    fn attacks_ordered(&self, day: u32) -> bool {
        self.postures(day)
            .iter()
            .any(|(_, p)| *p == Posture::Attack)
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

/// The men of the Allied garrison in truth.
fn garrison(t: &str, men: u32) -> String {
    changed(
        t,
        "side = \"allied\"\nmen = 3444\n",
        &format!("side = \"allied\"\nmen = {men}\n"),
    )
}

/// U's fatigue.
fn u_fatigue(t: &str, fatigue: u32) -> String {
    changed(
        t,
        "men = 3000\ntask_force = 1\nfatigue = 10",
        &format!("men = 3000\ntask_force = 1\nfatigue = {fatigue}"),
    )
}

/// U's men.
fn u_men(t: &str, men: u32) -> String {
    changed(
        t,
        "men = 3000\ntask_force = 1\nfatigue = 10",
        &format!("men = {men}\ntask_force = 1\nfatigue = 10"),
    )
}

fn risk_on(day: u32, level: &str) -> String {
    format!(
        "[[command]]\nday = {day}\nside = \"japanese\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{level}\"\n\n"
    )
}

fn w_at_source(t: &str) -> String {
    let t = at(
        t,
        "units",
        "[[land_unit]]\nid = 3\nname = \"Placeholder W\"\nside = \"japanese\"\nmen = 2000\nq = 0\nr = 0\n\n",
    );
    changed(&t, "land_units = [1, 2]", "land_units = [1, 2, 3]")
}

/// A second lift, task force 5: four placeholder troop transports at sea at
/// (q, r) with W aboard, in the take's forces limit.
fn w_aboard(t: &str, q: i32, r: i32) -> String {
    let mut ships = String::new();
    for i in 11..=14 {
        ships.push_str(&format!(
            "[[ship]]\nid = {i}\nname = \"Placeholder troop transport {i}\"\nclass = 10\nside = \"japanese\"\nfuel_t = 1000\ntask_force = 5\n\n"
        ));
    }
    let t = at(
        t,
        "units",
        &format!(
            "[[task_force]]\nid = 5\nname = \"Placeholder second lift\"\nside = \"japanese\"\nq = {q}\nr = {r}\nmission = \"transport\"\n\n{ships}[[land_unit]]\nid = 3\nname = \"Placeholder W\"\nside = \"japanese\"\nmen = 2000\ntask_force = 5\n\n"
        ),
    );
    let t = changed(&t, "land_units = [1, 2]", "land_units = [1, 2, 3]");
    changed(&t, "task_forces = [1]", "task_forces = [1, 5]")
}

fn route_of(day: u32, side: &str, force: u32, q: i32, r: i32) -> String {
    format!(
        "[[order]]\nday = {day}\nside = \"{side}\"\ntask_force = {force}\nkind = \"set_route\"\nwaypoints = [[{q}, {r}]]\n\n"
    )
}

/// The Allied cruiser force of the scenario made of `n` cruisers, stopped at
/// (q, r), and sent to (to_q, to_r) on `day`.
fn cruisers_hunt(
    t: &str,
    n: u32,
    (q, r): (i32, i32),
    day: u32,
    (to_q, to_r): (i32, i32),
) -> String {
    let t = changed(
        t,
        "q = 20\nr = 10\nmission = \"surface_combat\"",
        &format!("q = {q}\nr = {r}\nmission = \"surface_combat\""),
    );
    let mut more = String::new();
    for i in 1..n {
        more.push_str(&format!(
            "[[ship]]\nid = {}\nname = \"Placeholder Allied cruiser {}\"\nclass = 1\nside = \"allied\"\nfuel_t = 2000\ntask_force = 4\n\n",
            20 + i,
            20 + i
        ));
    }
    let t = at(&t, "units", &more);
    at(&t, "orders", &route_of(day, "allied", 4, to_q, to_r))
}

/// Every transport of L damaged to 95 flotation.
fn damaged(t: &str) -> String {
    let mut t = t.to_string();
    for i in 1..=9 {
        t = changed(
            &t,
            &format!("name = \"Placeholder troop transport {i}\"\nclass = 10\nside = \"japanese\"\nfuel_t = 1000\n"),
            &format!("name = \"Placeholder troop transport {i}\"\nclass = 10\nside = \"japanese\"\nfuel_t = 1000\nflotation_damage = 95\n"),
        );
    }
    t
}

fn sunk_on(g: &Played) -> Vec<(u32, u32)> {
    g.events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::ShipSunk { ship, .. } => Some((e.day, ship.raw())),
            _ => None,
        })
        .collect()
}

/// W is ashore at the source but may be used only from `from`, and the take
/// has the last day `last`.
fn w_before_its_day(t: &str, from: u32, last: u32) -> String {
    let t = changed(
        &w_at_source(t),
        "men = 2000\nq = 0\nr = 0\n",
        &format!("men = 2000\nq = 0\nr = 0\navailable_from = {from}\n"),
    );
    changed(
        &t,
        "intent_kind = \"take_place\"\nhex = [14, 0]\n",
        &format!("intent_kind = \"take_place\"\nhex = [14, 0]\nby_day = {last}\n"),
    )
}

/// The ground figures of statement 9c: scenario 109's with a ground report
/// of the true count (the report error is 1 at the least and at the most),
/// no man hit in a fight and no fatigue from a day's attack.
fn with_reports() -> LoadOptions {
    common::altered_reference("scenario_109_reports", |dir| {
        common::edit_file(dir, "ground_placeholder_scenario_109.toml", |text| {
            let text = common::replace_once(
                &text,
                "# SCENARIO 109: no report error figure is given",
                "report_error_low = \"1\"\nreport_error_high = \"1\"\n# SCENARIO 109: no report error figure is given",
            );
            let text = common::replace_once(&text, "hit_chance = \"0.3\"", "hit_chance = \"0\"");
            let text = common::replace_once(
                &text,
                "defender_hit_chance = \"0.6\"",
                "defender_hit_chance = \"0\"",
            );
            common::replace_once(
                &text,
                "fatigue_per_attack_day = 20",
                "fatigue_per_attack_day = 0",
            )
        });
    })
}

/// Allied task force 6, ten transports with land unit 11 (5,556 men) aboard,
/// 18 hexes from P at (-4,0): ordered to P on day 1, it is there at the start
/// of day 4 and unloads, so that the garrison is 9,000 men by the evening.
fn reinforced(t: &str) -> String {
    let mut ships = String::new();
    for i in 30..40 {
        ships.push_str(&format!(
            "[[ship]]\nid = {i}\nname = \"Placeholder Allied transport {i}\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000\ntask_force = 6\n\n"
        ));
    }
    let t = at(
        t,
        "units",
        &format!(
            "[[task_force]]\nid = 6\nname = \"Placeholder Allied reinforcement\"\nside = \"allied\"\nq = -4\nr = 0\nmission = \"transport\"\n\n{ships}[[land_unit]]\nid = 11\nname = \"Placeholder Allied reinforcement unit\"\nside = \"allied\"\nmen = 5556\ntask_force = 6\n\n"
        ),
    );
    let t = at(&t, "orders", &route_of(1, "allied", 6, 14, 0));
    at(
        &t,
        "orders",
        "[[order]]\nday = 1\nside = \"allied\"\ntask_force = 6\nkind = \"unload\"\n\n",
    )
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

/// The figure after `attack_fatigue_limit = ` in a ground figures file.
fn attack_fatigue_limit(file: &std::path::Path) -> u32 {
    let text = std::fs::read_to_string(file).expect("the figures file reads");
    number_after(&text, "\nattack_fatigue_limit = ")
}

fn both_attack(g: &Played, day: u32) -> bool {
    g.postures(day) == vec![(1, Posture::Attack), (2, Posture::Attack)]
}

// ------------------------------------------------------------------ The setting

#[test]
fn the_setting_is_what_the_sums_say() {
    // The attack fatigue limit is the reference file's, live and frozen: 60.
    // Two thirds of it, 40, is the mark of balanced; U's 10 and V's 0 are at
    // or below it, so both can attack.
    let live = attack_fatigue_limit(&common::data_dir().join("reference/ground.toml"));
    let frozen = attack_fatigue_limit(&common::frozen_root().join("ground.toml"));
    let own =
        attack_fatigue_limit(&common::frozen_root().join("ground_placeholder_scenario_109.toml"));
    assert_eq!((live, frozen, own), (60, 60, 60));
    let two_thirds = live * 2 / 3;
    assert_eq!(two_thirds, 40);
    let (u_fatigue, v_fatigue) = (10u32, 0u32);
    assert!(u_fatigue <= two_thirds && v_fatigue <= two_thirds);

    let g = play(&text());
    // On the morning M the men are ashore on defend and nothing has been
    // ordered to attack before it.
    assert_eq!(g.men_ashore_after(M - 1, 1), 3000);
    assert_eq!(g.men_ashore_after(M - 1, 2), 2386);
    assert_eq!(g.posture_after(M - 1, 1), Posture::Defend);
    assert_eq!(g.posture_after(M - 1, 2), Posture::Defend);
    assert_eq!(
        g.after(M - 1).land_units[&LandUnitId::new(1)].fatigue,
        Fixed::from_int(10)
    );
    assert_eq!(
        g.after(M - 1).land_units[&LandUnitId::new(2)].fatigue,
        Fixed::ZERO
    );
    // J's estimate of the enemy at P stands at 3,444, of day 0.
    let reason = judgement(&g, M);
    assert_says(
        &reason,
        &[
            "the side's estimate is 3444 enemy men at (14,0), of day 0",
            "land_unit:1 (Placeholder U): 3000 men ashore, on defend, fatigue 10 against the mark of 40, 2700 fit men, can attack",
            "land_unit:2 (Placeholder V): 2386 men ashore, on defend, fatigue 0 against the mark of 40, 2386 fit men, can attack",
            "the units that can attack have 5086 fit men",
        ],
    );
    // The transports lie empty and no unit is left behind at the source.
    assert_says(&reason, &["coming: no unit is left behind to bring"]);
    // The first morning of the judgement is M and no earlier one.
    for day in 1..M {
        assert!(!g.reasons(day).contains("the units that can attack have"));
    }
}

// ------------------------------------------------------------------ Statement 1

#[test]
fn statement_1_today_at_balanced_it_waits() {
    // The men wanted while more is coming: 3,444 x 2 = 6,888. 5,086 is
    // fewer, and short by 1,802.
    let g = play(&text());
    let j = judgement(&g, M);
    assert_eq!(wanted_while_more_is_coming(&j), 6_888);
    assert_says(
        &j,
        &[
            "at balanced the staff wants 2 men for each, 6888 men",
            "the units that can attack have 5086 fit men against the 6888 wanted, short by 1802",
        ],
    );
    // "Today" is the rule as it stands while more is coming: with a second
    // lift to come (W, 2,000 men, ashore at the source) the staff waits, on
    // every morning of the game and with no notice, as it did before the
    // rule.
    let more = play(&w_at_source(&text()));
    let j = judgement(&more, M);
    assert_eq!(wanted(&j), 6_888);
    assert_says(&j, &["short by 1802"]);
    for day in M..=7 {
        assert!(
            more.all_postures().iter().all(|(d, ..)| *d != day),
            "day {day}"
        );
        assert_eq!(more.cannot_bring_enough(day), 0, "day {day}");
    }
    for day in M..=7 {
        assert_eq!(more.posture_after(day, 1), Posture::Defend);
        assert_eq!(more.posture_after(day, 2), Posture::Defend);
    }
}

// ------------------------------------------------------------------ Statement 2

#[test]
fn statement_2_nothing_more_is_coming_and_the_reason_says_what_was_looked_for() {
    let g = play(&text());
    let j = judgement(&g, M);
    assert_says(
        &j,
        &[
            "nothing more is coming",
            "no lift of the take has fighting men aboard",
            "no lift as it stands with fighting men aboard can be sent",
            "no fighting unit of its forces limit off (14,0) can be used and carried",
            "with no date, a unit or force before its day counting from its day",
        ],
    );
    assert_silent(&j, &["more is coming:"]);
    // With the take's last day given, it says what it looked by.
    let dated = play(&changed(
        &text(),
        "intent_kind = \"take_place\"\nhex = [14, 0]\n",
        "intent_kind = \"take_place\"\nhex = [14, 0]\nby_day = 10\n",
    ));
    assert_says(
        &judgement(&dated, M),
        &["nothing more is coming (by day 10, the latest day)"],
    );
}

// ------------------------------------------------------------------ Statement 3

#[test]
fn statement_3_balanced_with_nothing_more_coming_the_units_attack() {
    let g = play(&text());
    assert_eq!(
        g.postures(M),
        vec![(1, Posture::Attack), (2, Posture::Attack)]
    );
    // Nothing is ordered before M. In the game's last days (decision 0194)
    // the attack of day 5 has failed and both are set back to defend.
    assert_eq!(
        g.all_postures(),
        vec![
            (M, 1, Posture::Attack),
            (M, 2, Posture::Attack),
            (6, 1, Posture::Defend),
            (6, 2, Posture::Defend)
        ]
    );
    let j = judgement(&g, M);
    assert_eq!(wanted(&j), 3_444);
    assert_says(
        &j,
        &[
            "the side's estimate is 3444 enemy men at (14,0), of day 0",
            "at balanced, with nothing more coming, the staff wants 1 man for each, 3444 men",
            "2700 fit men",
            "2386 fit men",
            "the units that can attack have 5086 fit men, at least the 3444 wanted",
            "nothing more is coming",
        ],
    );
    for order in &g.handed[&M].orders {
        let r = text_of(&order.reason);
        assert_says(
            &r,
            &["Nothing more is coming and the men that can attack are enough for it"],
        );
    }
    assert_eq!(g.cannot_bring_enough(M), 0);
    assert_eq!(g.posture_after(M, 1), Posture::Attack);
    assert_eq!(g.posture_after(M, 2), Posture::Attack);
    // No notice on any day.
    for day in 1..=7 {
        assert_eq!(g.cannot_bring_enough(day), 0, "day {day}");
    }
}

// ------------------------------------------------------------------ Statement 4

#[test]
fn statement_4_balanced_too_few_even_so_it_waits_and_says_so_each_morning() {
    let g = play(&estimate(&text(), 5_200));
    for day in M..=7 {
        assert_eq!(g.postures(day), vec![], "day {day}");
        assert_eq!(g.cannot_bring_enough(day), 1, "the notice on day {day}");
        assert_eq!(g.posture_after(day, 1), Posture::Defend);
        assert_eq!(g.posture_after(day, 2), Posture::Defend);
    }
    for day in 1..M {
        assert_eq!(g.cannot_bring_enough(day), 0, "day {day}");
    }
    let n = g.notice_reasons(M, NoticeKind::CannotBringEnough);
    assert_eq!(n.len(), 1);
    assert_says(
        &n[0],
        &[
            "the side's estimate is 5200 enemy men at (14,0)",
            "at balanced, with nothing more coming, the staff wants 1 man for each, 5200 men",
            "5086 fit men against the 5200 wanted, short by 114",
            "Nothing more is coming and the men that can attack are too few",
            "hands in the notice that it cannot bring enough",
        ],
    );
    // The next morning it hands the notice in again.
    let again = g.notice_reasons(M + 1, NoticeKind::CannotBringEnough);
    assert_eq!(again.len(), 1);
    // (U has lost a man unfed in the night: 2,999 men, 2,699 fit men.)
    assert_says(
        &again[0],
        &[
            "5200 men",
            "5085 fit men against the 5200 wanted, short by 115",
            "nothing more is coming",
        ],
    );
}

// ------------------------------------------------------------------ Statement 5

#[test]
fn statement_5_press_with_nothing_more_coming_the_units_attack_whatever_the_count() {
    for men in [5_200, 20_000] {
        let g = play(&estimate(&risk(&text(), "press"), men));
        assert_eq!(
            g.postures(M),
            vec![(1, Posture::Attack), (2, Posture::Attack)],
            "estimate {men}"
        );
        let j = judgement(&g, M);
        assert_eq!(wanted(&j), 0, "estimate {men}");
        assert_says(
            &j,
            &[
                &format!("the side's estimate is {men} enemy men at (14,0)"),
                "at press, with nothing more coming, the staff wants none",
                "every unit that can attack goes in, whatever the estimate",
            ],
        );
        assert_eq!(g.cannot_bring_enough(M), 0, "estimate {men}");
    }
}

// ------------------------------------------------------------------ Statement 6

#[test]
fn statement_6_preserve_the_force_waits_and_says_so() {
    let g = play(&risk(&text(), "preserve"));
    for day in M..=7 {
        assert_eq!(g.postures(day), vec![], "day {day}");
        assert_eq!(g.cannot_bring_enough(day), 1, "day {day}");
    }
    let j = judgement(&g, M);
    assert_eq!(wanted(&j), 10_332);
    assert_says(
        &j,
        &[
            "at preserve, with nothing more coming, the staff wants 3 men for each, 10332 men",
            "5086 fit men against the 10332 wanted, short by 5246",
            "nothing more is coming",
        ],
    );
    assert_eq!(3_444 * 3, 10_332);
    let n = g.notice_reasons(M, NoticeKind::CannotBringEnough);
    assert_eq!(n.len(), 1);
    assert_says(&n[0], &["hands in the notice that it cannot bring enough"]);
    assert_eq!(g.posture_after(7, 1), Posture::Defend);
}

// ------------------------------------------------------------------ Statement 7

#[test]
fn statement_7_while_more_is_coming_as_today_a_second_lift_to_be_sent() {
    // W, 2,000 men, in the take's forces limit, ashore at the source. On M
    // the transports are on their way home to the source; on the morning
    // they lie there empty (day 6) the staff sends W.
    let g = play(&w_at_source(&text()));
    for day in M..=7 {
        let j = judgement(&g, day);
        assert_eq!(wanted(&j), 6_888, "day {day}");
        assert_silent(&j, &["nothing more is coming"]);
        assert_eq!(g.postures(day), vec![], "day {day}");
        assert_eq!(g.cannot_bring_enough(day), 0, "day {day}");
    }
    assert_says(
        &judgement(&g, M),
        &[
            "left behind at the source: land_unit:3 (Placeholder W) (2000 men)",
            "to be sent when task_force:1 lies stopped at the source with nothing aboard",
            "short by 1802",
        ],
    );
    assert_says(
        &judgement(&g, 6),
        &["task_force:1 lies stopped at the source with nothing aboard, so land_unit:3 is sent now"],
    );
    // The transports are empty and at the source that morning.
    let l = &g.after(5).task_forces[&TaskForceId::new(1)];
    assert_eq!(l.position(), sim_core::hex::Hex::new(0, 0));
}

// ------------------------------------------------------------------ Statement 8

#[test]
fn statement_8_a_lift_at_sea_is_more_coming() {
    // W, 2,000 men, aboard four transports of a second lift, task force 5,
    // at sea and in the take's forces limit: the staff sends it on M and it
    // is under way after. The men wanted stay 6,888 and the staff waits,
    // with no notice.
    let g = play(&w_aboard(&text(), -4, -4));
    for day in M..=7 {
        let j = judgement(&g, day);
        assert_eq!(wanted(&j), 6_888, "day {day}");
        assert_silent(&j, &["nothing more is coming"]);
        assert_eq!(g.postures(day), vec![], "day {day}");
        assert_eq!(g.cannot_bring_enough(day), 0, "day {day}");
    }
    assert_says(
        &judgement(&g, M),
        &["task_force:5, with land_unit:3 (Placeholder W), 2000 men, a fighting unit aboard, is a later lift as it stands and is sent now"],
    );
    assert_says(
        &judgement(&g, M + 1),
        &["task_force:5 is under way on its orders, which stand"],
    );
    // W is aboard and afloat under way on the later mornings.
    assert_eq!(g.men_ashore_after(M, 3), 0);
    let f = &g.after(M).task_forces[&TaskForceId::new(5)];
    assert_ne!(f.position(), sim_core::hex::Hex::new(-4, -4));
}

// ------------------------------------------------------------------ Statement 9

#[test]
#[ignore = "a lift turned back cannot be built in this scenario: the staff turns a lift back only on a fresh carrier contact within 8 hexes of its way (decision 0191), and a contact is entered at the start of the game, so on M (day 4) it is 9 pulses old and not fresh; a scripted route to the source on a task force of the take makes the staff leave it out (\"held by the commander's order\") and so it is not counted; and W aboard L itself is landed with U and V by L's unload order. What the code gave with a scripted route: \"nothing more is coming\" and the attack on M, with the reason \"task_force:5 is left out: task_force:5 is held by the commander's order of day 1\""]
fn statement_9_a_lift_turned_back_is_more_coming() {
    panic!("not buildable here: see the reason of the ignore");
}

// ------------------------------------------------------------------ Statement 9a

#[test]
fn statement_9a_a_unit_before_its_day_is_coming() {
    // W may be used from day 7, three days on from M, inside the take's last
    // day, 10: more is coming, the men wanted are 6,888, the staff waits and
    // hands in no notice.
    let g = play(&w_before_its_day(&text(), 7, 10));
    for day in M..=6 {
        let j = judgement(&g, day);
        assert_eq!(wanted(&j), 6_888, "day {day}");
        assert_silent(&j, &["nothing more is coming"]);
        assert_eq!(g.postures(day), vec![], "day {day}");
        assert_eq!(g.cannot_bring_enough(day), 0, "day {day}");
    }
    assert_says(
        &judgement(&g, M),
        &["land_unit:3 (Placeholder W) may be used from day 7"],
    );
    // A unit whose day is after the last day of the take is not coming:
    // statement 3 holds.
    let late = play(&w_before_its_day(&text(), 12, 10));
    assert_eq!(
        late.postures(M),
        vec![(1, Posture::Attack), (2, Posture::Attack)]
    );
    assert_says(
        &judgement(&late, M),
        &[
            "nothing more is coming (by day 10, the latest day)",
            "land_unit:3 (Placeholder W) may be used from day 12, after the latest day",
        ],
    );
}

// ------------------------------------------------------------------ Statement 9b

#[test]
fn statement_9b_a_unit_no_ship_can_carry_is_not_coming() {
    // W is ashore at the source. Every transport of the take is sunk on day
    // 4 by an Allied cruiser force (the nine transports are damaged to 95
    // flotation and the cruisers are sent to their way home on that day),
    // after U and V are ashore. On M the transports are afloat and more is
    // coming: the staff waits. On the next morning, with no ship afloat,
    // nothing more is coming and statement 3 holds.
    let g = play(&w_at_source(&cruisers_hunt(
        &damaged(&text()),
        12,
        (6, 2),
        4,
        (6, 0),
    )));
    let sunk: Vec<u32> = sunk_on(&g).iter().map(|(_, ship)| *ship).collect();
    assert_eq!(sunk, (1..=9).collect::<Vec<u32>>());
    assert_eq!(g.after(M).task_forces[&TaskForceId::new(1)].ships.len(), 0);
    assert_eq!(g.postures(M), vec![]);
    assert_silent(&judgement(&g, M), &["nothing more is coming"]);
    assert_eq!(g.cannot_bring_enough(M), 0);
    assert_eq!(wanted(&judgement(&g, M)), 6_888);
    assert!(both_attack(&g, M + 1));
    let j = judgement(&g, M + 1);
    assert_eq!(wanted(&j), 3_444);
    assert_says(
        &j,
        &[
            "nothing more is coming",
            "land_unit:3 at the source: no ship of task_force:1, the take's lift, is afloat to carry them",
            "at least the 3444 wanted",
        ],
    );
}

// ------------------------------------------------------------------ Statement 9c

#[test]
fn statement_9c_once_sent_in_they_stay_in() {
    // The ground figures give a ground report of the true count each
    // evening, no man is hit and a day's attack tires no one. The Allied
    // garrison is 3,444 men and on day 4 an Allied lift lands 5,556 more: the
    // evening's report, which replaces J's estimate, is 9,000.
    let g = play_with(&reinforced(&text()), &with_reports());
    // Statement 3 on M: the estimate, from the report of day 3, is 3,444.
    assert_eq!(
        g.postures(M),
        vec![(1, Posture::Attack), (2, Posture::Attack)]
    );
    assert_eq!(wanted(&judgement(&g, M)), 3_444);
    assert_says(
        &judgement(&g, M),
        &["the side's estimate is 3444 enemy men at (14,0)"],
    );
    // The next morning the estimate is 9,000: the units are on attack, the
    // staff does not set them to defend and sets no other unit to attack.
    assert_eq!(
        g.after(M).land_units[&LandUnitId::new(10)].men_ashore,
        3_444
    );
    assert_eq!(
        g.after(M).land_units[&LandUnitId::new(11)].men_ashore,
        5_556
    );
    let next = judgement(&g, M + 1);
    assert_says(
        &next,
        &[
            "the side's estimate is 9000 enemy men at (14,0)",
            "at balanced, with nothing more coming, the staff wants 1 man for each, 9000 men",
            "against the 9000 wanted",
            "The units on attack are committed",
        ],
    );
    assert_eq!(wanted(&next), 9_000);
    for day in M + 1..=7 {
        assert_eq!(g.postures(day), vec![], "day {day}");
        assert_eq!(g.posture_after(day, 1), Posture::Attack, "day {day}");
        assert_eq!(g.posture_after(day, 2), Posture::Attack, "day {day}");
        assert_eq!(g.cannot_bring_enough(day), 0, "day {day}");
    }
}

// ------------------------------------------------------------------ Statement 10

#[test]
fn statement_10_a_unit_that_cannot_attack_is_not_counted() {
    // U's fatigue is 50: above the balanced mark of 40 (and below the limit
    // of 60). The fit men that can attack are V's 2,386, fewer than 3,444.
    let g = play(&u_fatigue(&text(), 50));
    assert_eq!(g.postures(M), vec![]);
    assert_eq!(g.cannot_bring_enough(M), 1);
    let j = judgement(&g, M);
    assert_says(
        &j,
        &[
            "land_unit:1 (Placeholder U): 3000 men ashore, on defend, fatigue 50 against the mark of 40, 1500 fit men, cannot attack",
            "the units that can attack have 2386 fit men against the 3444 wanted, short by 1058",
            "nothing more is coming",
        ],
    );
    assert_eq!(wanted(&j), 3_444);
    let n = g.notice_reasons(M, NoticeKind::CannotBringEnough);
    assert_eq!(n.len(), 1);
    assert_says(&n[0], &["hands in the notice that it cannot bring enough"]);
}

// ------------------------------------------------------------------ Statement 11

#[test]
fn statement_11_no_estimate_as_today() {
    let g = play(&no_estimate(&text()));
    assert_eq!(
        g.postures(M),
        vec![(1, Posture::Attack), (2, Posture::Attack)]
    );
    for order in &g.handed[&M].orders {
        let r = text_of(&order.reason);
        assert_says(
            &r,
            &[
                "the side holds no estimate of the enemy's men at (14,0)",
                "With no count of the enemy it attacks",
            ],
        );
        assert_silent(&r, &["nothing more is coming"]);
    }
    assert_eq!(g.cannot_bring_enough(M), 0);
}

// ------------------------------------------------------------------ Statement 12

#[test]
fn statement_12_the_commander_is_told_and_may_overrule() {
    // With the standing interrupt for the notice on (it is on by default),
    // a seat's day stops on the morning of statement 4 (the interrupt fires
    // once for the plan; the notice is handed in on each morning).
    let g = play(&estimate(&text(), 5_200));
    assert_eq!(g.interrupts_of(StandingKind::CannotBringEnough), vec![4]);
    for day in M..=7 {
        assert_eq!(g.cannot_bring_enough(day), 1, "day {day}");
    }
    // The commander sets press on the intent that morning's day (day 5):
    // the units attack the next morning.
    let over = play(&at(&estimate(&text(), 5_200), "risk", &risk_on(5, "press")));
    assert_eq!(over.interrupts_of(StandingKind::CannotBringEnough), vec![4]);
    assert_eq!(over.postures(M), vec![]);
    assert!(both_attack(&over, M + 1));
    assert_says(
        &judgement(&over, M + 1),
        &["at press, with nothing more coming, the staff wants none"],
    );
    // Without it the units wait on that morning too.
    assert_eq!(g.postures(M + 1), vec![]);
    // With the standing interrupt switched off by the commander the notice
    // is still handed in and nothing fires.
    let off = play(&at(
        &estimate(&text(), 5_200),
        "risk",
        "[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_interrupt\"\nstanding = \"cannot_bring_enough\"\non = false\n\n",
    ));
    assert_eq!(off.cannot_bring_enough(M), 1);
    assert!(off
        .interrupts_of(StandingKind::CannotBringEnough)
        .is_empty());
}

// ------------------------------------------------------------------ Statement 13

#[test]
fn statement_13_truth_and_belief() {
    // The defenders doubled in truth (6,888), J's estimate left at 3,444:
    // statement 3's orders are the same, with their reasons, on every day up
    // to and including M; the truth is not the same.
    let base = play(&text());
    let doubled = play(&garrison(&text(), 6_888));
    for day in 1..=M {
        assert_eq!(doubled.handed[&day], base.handed[&day], "day {day}");
    }
    assert_eq!(
        doubled.postures(M),
        vec![(1, Posture::Attack), (2, Posture::Attack)]
    );
    assert_eq!(
        base.after(M - 1).land_units[&LandUnitId::new(10)].men_ashore,
        3_444
    );
    assert_eq!(
        doubled.after(M - 1).land_units[&LandUnitId::new(10)].men_ashore,
        6_888
    );
}

// --------------------------------------------------------------- Properties

/// J's staff orders a unit to attack on M, in a game.
fn attacks(g: &Played) -> bool {
    g.attacks_ordered(M)
}

const RISKS: [(&str, u32); 3] = [("preserve", 3), ("balanced", 2), ("press", 1)];

const ESTIMATES: [u32; 9] = [
    1_000, 1_695, 1_696, 2_543, 2_544, 3_444, 5_086, 5_087, 9_000,
];

/// A game in which J's estimate on M is `men`: the garrison is `men` in
/// truth and the ground report of the evening before gives the true count
/// (the lift is planned on the opening estimate, 3,444, so that U and V are
/// both landed whatever the count).
fn reported(t: &str, level: &str, men: u32) -> Played {
    play_with(&garrison(&risk(t, level), men), &with_reports())
}

#[test]
fn property_while_more_is_coming_every_order_and_reason_is_what_it_was() {
    // With a second lift to come (W at the source) the staff attacks where
    // the fit men, 5,086, are at least the estimate times 3, 2 or 1 by the
    // risk, as before this rule, and otherwise waits: with no notice, and
    // with the reasons saying nothing of "nothing more is coming".
    for (level, per) in RISKS {
        for men in ESTIMATES {
            let g = reported(&w_at_source(&text()), level, men);
            let before = 5_086 >= men * per;
            assert_eq!(attacks(&g), before, "{level} {men}");
            assert_eq!(g.cannot_bring_enough(M), 0, "{level} {men}");
            let j = judgement(&g, M);
            assert_says(
                &j,
                &[&format!(
                    "the side's estimate is {men} enemy men at (14,0), of day 3"
                )],
            );
            assert_silent(&j, &["nothing more is coming"]);
            assert_eq!(wanted_while_more_is_coming(&j), men * per, "{level} {men}");
            assert_eq!(wanted(&j), men * per, "{level} {men}");
        }
    }
}

#[test]
fn property_at_preserve_the_force_no_unit_is_set_to_attack_that_was_not_before() {
    for men in ESTIMATES {
        let nothing = reported(&text(), "preserve", men);
        let more = reported(&w_at_source(&text()), "preserve", men);
        let before = 5_086 >= men * 3;
        assert_eq!(attacks(&nothing), before, "{men}");
        assert_eq!(attacks(&nothing), attacks(&more), "{men}");
        // Where it waits with nothing more coming it says so.
        if !before {
            assert_eq!(nothing.cannot_bring_enough(M), 1, "{men}");
        }
    }
}

#[test]
fn property_the_men_wanted_with_nothing_coming_are_never_more_than_while_more_is_coming() {
    for (level, per) in RISKS {
        for men in ESTIMATES {
            let g = reported(&text(), level, men);
            let j = judgement(&g, M);
            let while_more = wanted_while_more_is_coming(&j);
            assert_eq!(while_more, men * per, "{level} {men}");
            if let Some(nothing) = wanted_with_nothing_coming(&j) {
                assert!(
                    nothing <= while_more,
                    "{level} {men}: {nothing} {while_more}"
                );
            }
            assert!(wanted(&j) <= while_more, "{level} {men}");
        }
    }
}

#[test]
fn property_more_fit_men_never_turn_an_attack_into_a_wait() {
    // U's men from few to many (fit men are 90 of every 100 of them, V's
    // 2,386 are fit): once the units attack, no larger U makes them wait.
    for (level, men) in [("preserve", 1_200), ("balanced", 3_444), ("press", 3_444)] {
        let mut was_attack = false;
        for u in [500, 900, 1_200, 1_300, 1_400, 1_500, 2_000, 2_500, 3_000] {
            let g = reported(&u_men(&text(), u), level, men);
            let now = attacks(&g);
            if was_attack {
                assert!(now, "{level} {u}");
            }
            was_attack = now;
            // The reason's fit men are what the sum says.
            let fit = u * 90 / 100 + 2_386;
            assert_says(
                &judgement(&g, M),
                &[&format!("the units that can attack have {fit} fit men")],
            );
        }
        assert!(was_attack, "{level}: the largest U attacks");
    }
}

#[test]
fn property_a_larger_estimate_never_turns_a_wait_into_an_attack() {
    for (level, _) in RISKS {
        let mut was_wait = false;
        for men in ESTIMATES.into_iter().chain([12_000, 20_000]) {
            let now = attacks(&reported(&text(), level, men));
            if was_wait {
                assert!(!now, "{level} {men}");
            }
            was_wait = !now;
        }
    }
    // And the same while more is coming.
    for (level, _) in RISKS {
        let mut was_wait = false;
        for men in ESTIMATES {
            let now = attacks(&reported(&w_at_source(&text()), level, men));
            if was_wait {
                assert!(!now, "{level} {men}");
            }
            was_wait = !now;
        }
    }
}

#[test]
fn property_a_lift_added_never_lowers_the_men_wanted() {
    for (level, _) in RISKS {
        for men in ESTIMATES {
            let without = reported(&text(), level, men);
            let with_source = reported(&w_at_source(&text()), level, men);
            let with_sea = reported(&w_aboard(&text(), -4, -4), level, men);
            let base = wanted(&judgement(&without, M));
            assert!(wanted(&judgement(&with_source, M)) >= base, "{level} {men}");
            assert!(wanted(&judgement(&with_sea, M)) >= base, "{level} {men}");
        }
    }
}
