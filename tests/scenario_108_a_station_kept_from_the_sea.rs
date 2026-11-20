//! Scenario 108: a station kept from the sea. A surface force that begins
//! at sea with no home port is stood at its station by the staff from
//! where it is.
//!
//! Proves decision 0192. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on the placeholder map, with
//! no real ship. Side A has a surface force G of two cruisers and two
//! destroyers, at sea, stopped, with no home port, and one base B. G is
//! given "stand at" a station X. Hexes are 40 nm. G cruises at 15 kn and
//! has a full speed of 30 kn. The night is 12 hours.
//!
//! # Statements
//!
//! 1. **Today's answer is gone.** At balanced, with no contact held, on
//!    the first morning the staff does not say G is away from home. It
//!    routes G to X, and its reason says G has no home port and stands
//!    from where it is. G is at normal readiness. At press it is routed
//!    the same, at battle readiness.
//! 2. **Cruise where the night is enough.** X is 4 hexes from G: 160 nm,
//!    160 / 15 = 10.7 hours, within 12. G is ordered at cruise.
//! 3. **Full where it is not.** X is 8 hexes: 320 nm, 21.3 hours at
//!    cruise, 320 / 30 = 10.7 at full. G is ordered at full speed.
//! 4. **It goes though the night is not enough.** X is 12 hexes: 480 nm,
//!    32 hours at cruise and 16 at full. G is ordered at full speed and
//!    goes; a force with a home port in the same case does not sortie
//!    and hands in the notice that the intent cannot be met.
//! 5. **It arrives and stays.** G arrives at X and lies stopped there,
//!    with no mark and no leg home. It is given no route on the morning
//!    after or the next. On a morning it is under way to X it is given
//!    no route either.
//! 6. **The threat sets its readiness, not whether it goes.** At
//!    balanced, at X, with no contact of ships within 15 hexes of X, G is
//!    at normal readiness. On the morning a contact of ships is held 10
//!    hexes from X, G is set to battle readiness. At press it is at
//!    battle readiness every morning.
//! 6a. **Preserve the force keeps clear of what outnumbers it.** At
//!    preserve the force, with a contact of 9 ships 10 hexes from X
//!    against G's 4: G is not routed to X, and the reason names the
//!    contact and the numbers. With a contact of 3 ships it goes.
//! 7. **Fuel is weighed to a base that can fuel it.** B holds fuel above
//!    its reserve and has a rate, and is 5 hexes from X by sea. With X at
//!    4 hexes, the passage weighed is 160 nm to X and 200 nm from X to
//!    B. At press, where the margin is one, the reason gives both and
//!    the fuel aboard against the fuel needed; with one ship's fuel cut
//!    below that need, G does not go.
//! 7a. **No base that can fuel it.** With B's fuel stock at nothing, the
//!    fuel is not weighed, G goes, and the reason says no base of the
//!    side can fuel it. A nearer base, B2, that holds no fuel is passed
//!    over for B when B holds fuel.
//! 8. **Going in for fuel.** G is at X at balanced. A destroyer's fuel is
//!    below the measure of the passage from X to B at cruise times the
//!    margin of one and a fifth, and above the passage itself. That
//!    morning G is routed to B and B is set as its home port, and the
//!    reason names the destroyer. (At press the margin is one: a ship
//!    cut below the passage itself cannot make it, and runs dry short
//!    of B. That is what pressing costs, and is not mended here.)
//! 9. **Then it is a force with a home port.** Stopped at B and fuelled,
//!    G sorties to X by night and returns, as a stand's force does today.
//! 10. **A force with a home port is as today.** A second force, H, of
//!     the same ships, stopped at B with B its home port: every order
//!     and reason the staff gives H is what it gave before this rule.
//!     And H under way at sea toward B, B its home port, is answered
//!     "away from home", as today.
//! 10a. **Stopped at sea with a home port.** H stopped at sea on no
//!     route, B its home port: the staff routes H to B at cruise, and
//!     says why. (A home port taken by the enemy is cleared by the
//!     engine, so the force is then one with no home port, under
//!     statement 1; no game reaches a home port that is not the side's.)
//! 11. **Held.** With G held by the commander, the staff gives it no
//!     order and says it is held.
//! 12. **Under a deny.** A deny of the waters round X whose forces limit
//!     names G forms G's stand task at X, and statements 1 to 5 hold of
//!     it.
//!
//! # Properties
//!
//! - In every scenario with no stand task for a force at sea without a
//!   home port, no order of a staff differs from before this rule.
//! - G is never routed anywhere but X or the base its fuel is weighed to.
//! - A force that goes to its station at balanced goes at press.
//! - More fuel aboard never stops a force going to its station.
//!
//! The tests are written by the build under these statements.

// The architect's statement numbers (6a, 7a, 10a) are list items in the doc
// comment above, which is left as written.
#![allow(clippy::doc_lazy_continuation)]

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{NoticeKind, Reason, StaffDecisions, StaffOrder};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, ShipId, TaskForceId};
use sim_core::orders::{OrderKind, Readiness, SpeedSetting, TaskForceOrder};
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;
use sim_staff::stand::{margin, THREAT_HEXES};
use sim_staff::{as_staffs, staffs};

mod common;

/// Where G begins, B and its neighbours.
const START: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 5 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/station-from-the-sea/108_a_station_kept_from_the_sea.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

fn hex(q: i32, r: i32) -> Hex {
    Hex { q, r }
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

/// Adds `what` at the mark `# MARK: name` of the scenario's text; the mark
/// stays, so that more can be added.
fn at(text: &str, name: &str, what: &str) -> String {
    let mark = format!("# MARK: {name}\n");
    changed(text, &mark, &format!("{what}{mark}"))
}

/// The station X of the file, (4,0), moved to `q` hexes east of G.
fn station_at(text: &str, q: i32) -> String {
    changed(text, "hex = [4, 0]", &format!("hex = [{q}, 0]"))
}

fn command(day: u32, body: &str) -> String {
    format!("[[command]]\nday = {day}\nside = \"allied\"\n{body}\n\n")
}

fn risk_of(intent: u32, level: &str) -> String {
    command(
        1,
        &format!("kind = \"set_limit\"\nlimit = \"risk\"\nintent = {intent}\nrisk = \"{level}\""),
    )
}

/// A contact of `ships` Allied-held ships at `(q, 0)`.
fn contact(q: i32, ships: u32) -> String {
    format!(
        "[[contact]]\nside = \"allied\"\nq = {q}\nr = 0\nship_count = {ships}\nkind = \"cruiser\"\nage_pulses = 0\n\n"
    )
}

/// Task force 2, H: two placeholder cruisers and two placeholder destroyers
/// (ships 5 to 8) stopped at `place` (`at_base = N` or `q = ..` and `r = ..`),
/// with the given home port line, and its own intent, 2, to stand at `x`.
fn force_h(place: &str, home_port: &str, x: i32) -> String {
    let mut out = format!(
        "[[task_force]]\nid = 2\nname = \"Placeholder force H\"\nside = \"allied\"\n{place}\nmission = \"surface_combat\"\nstance = \"avoid\"\n{home_port}\n\n"
    );
    for (id, class, fuel) in [(5, 15, 2000), (6, 15, 2000), (7, 16, 500), (8, 16, 500)] {
        out.push_str(&format!(
            "[[ship]]\nid = {id}\nname = \"Placeholder ship {id}\"\nclass = {class}\nside = \"allied\"\nfuel_t = {fuel}\ntask_force = 2\n\n"
        ));
    }
    out.push_str(&command(
        1,
        &format!("kind = \"set_intent\"\nintent = 2\nintent_kind = \"stand_at\"\ntask_force = 2\nhex = [{x}, 0]"),
    ));
    out
}

/// A game played with the doctrine's staffs: what the Allied staff handed in
/// each day, and the truth at the end of each day (index 0 is the start).
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
}

fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &common::frozen()).expect("the scenario loads")
}

fn play(text: &str) -> Played {
    let mut scenario = load(text);
    let held = staffs();
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
                .get(&Side::Allied)
                .cloned()
                .unwrap_or_default(),
        );
        truth.push(scenario.world.umpire().clone());
    }
    Played { handed, truth }
}

fn route(waypoints: &[(i32, i32)]) -> TaskForceOrder {
    TaskForceOrder::SetRoute {
        waypoints: waypoints.iter().map(|(q, r)| hex(*q, *r)).collect(),
        act_at: Default::default(),
        stand_at: Default::default(),
    }
}

/// A route with the last-but-one waypoint marked to stand at, as a sortie.
fn sortie(x: (i32, i32), home: (i32, i32)) -> TaskForceOrder {
    TaskForceOrder::SetRoute {
        waypoints: vec![hex(x.0, x.1), hex(home.0, home.1)],
        act_at: Default::default(),
        stand_at: [hex(x.0, x.1)].into_iter().collect(),
    }
}

const FULL: TaskForceOrder = TaskForceOrder::SetSpeed(SpeedSetting::Full);
const BATTLE: TaskForceOrder = TaskForceOrder::SetReadiness(Readiness::Battle);

fn target_of(o: &StaffOrder) -> (TaskForceId, TaskForceOrder) {
    match &o.kind {
        OrderKind::TaskForce { target, order } => (*target, order.clone()),
        other => panic!("the doctrine gave a non-task-force order: {other:?}"),
    }
}

impl Played {
    fn decisions(&self, day: u32) -> &StaffDecisions {
        &self.handed[&day]
    }

    /// The orders a day gave `f`, in the order they were handed in, but for
    /// the rules of engagement and radio silence, which every stand sets as
    /// it does today and which these statements do not touch.
    fn orders_of(&self, day: u32, f: u32) -> Vec<TaskForceOrder> {
        self.decisions(day)
            .orders
            .iter()
            .map(target_of)
            .filter(|(t, _)| *t == force(f))
            .map(|(_, o)| o)
            .filter(|o| {
                !matches!(
                    o,
                    TaskForceOrder::SetRulesOfEngagement { .. }
                        | TaskForceOrder::SetRadioSilence { .. }
                )
            })
            .collect()
    }

    fn orders(&self, day: u32) -> Vec<TaskForceOrder> {
        self.orders_of(day, 1)
    }

    /// The route a day gave `f`, if any.
    fn route_of(&self, day: u32, f: u32) -> Option<TaskForceOrder> {
        self.orders_of(day, f)
            .into_iter()
            .find(|o| matches!(o, TaskForceOrder::SetRoute { .. }))
    }

    /// Every reason of a day, the orders' and the decisions to give none,
    /// as one text: its calculation, evidence and bindings.
    fn said(&self, day: u32) -> String {
        let d = self.decisions(day);
        d.orders
            .iter()
            .map(|o| &o.reason)
            .chain(d.no_orders.iter())
            .map(words)
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// The reasons of the day that serve `intent` (the task of a force).
    fn said_of(&self, day: u32, intent: u32) -> String {
        let d = self.decisions(day);
        d.orders
            .iter()
            .map(|o| &o.reason)
            .chain(d.no_orders.iter())
            .filter(|r| r.serves.intent == Some(sim_core::ids::IntentId::new(intent)))
            .map(words)
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn cannot_meet(&self, day: u32) -> bool {
        self.decisions(day)
            .notices
            .iter()
            .any(|n| n.kind == NoticeKind::CannotMeet)
    }

    /// Where `f` is at the end of `day` (day 0 is the start).
    fn position(&self, day: usize, f: u32) -> Hex {
        self.truth[day].task_forces[&force(f)].position()
    }
}

/// A reason's text: its calculation, its evidence and its bindings.
fn words(reason: &Reason) -> String {
    let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
    format!(
        "{} | {} | {}",
        reason.worked,
        evidence.join(" | "),
        reason.binding.join(" | ")
    )
}

// ------------------------------------------------------------- The variations

/// G begins at `(q, r)` instead of (0,0).
fn g_at(text: &str, q: i32, r: i32) -> String {
    changed(
        text,
        "q = 0\nr = 0\nmission = \"surface_combat\"",
        &format!("q = {q}\nr = {r}\nmission = \"surface_combat\""),
    )
}

/// Destroyer 4's fuel, in tons, instead of 500.
fn ship4_fuel(text: &str, tons: i64) -> String {
    changed(
        text,
        "name = \"Placeholder destroyer 4\"\nclass = 16\nside = \"allied\"\nfuel_t = 500",
        &format!(
            "name = \"Placeholder destroyer 4\"\nclass = 16\nside = \"allied\"\nfuel_t = {tons}"
        ),
    )
}

/// Destroyer 3's fuel, in tons, instead of 500.
fn ship3_fuel(text: &str, tons: i64) -> String {
    changed(
        text,
        "name = \"Placeholder destroyer 3\"\nclass = 16\nside = \"allied\"\nfuel_t = 500",
        &format!(
            "name = \"Placeholder destroyer 3\"\nclass = 16\nside = \"allied\"\nfuel_t = {tons}"
        ),
    )
}

/// Base B's fuel, in tons, instead of 5,000.
fn b_fuel(text: &str, tons: i64) -> String {
    changed(
        text,
        "supply_t = 1000\nfuel_t = 5000",
        &format!("supply_t = 1000\nfuel_t = {tons}"),
    )
}

/// An Allied base `id` at `(q, r)` with `fuel` tons of fuel.
fn base_at(id: u32, q: i32, r: i32, fuel: i64) -> String {
    format!(
        "[[base]]\nid = {id}\nname = \"Placeholder base {id}\"\nside = \"allied\"\nq = {q}\nr = {r}\nsupply_t = 500\nfuel_t = {fuel}\n\n"
    )
}

/// The file with the station at `q` hexes east of G and the risk set.
fn at_station_and_risk(q: i32, level: &str) -> String {
    at(&station_at(&text(), q), "commands", &risk_of(1, level))
}

fn without_g_commands_hold() -> String {
    at(
        &text(),
        "commands",
        &command(1, "kind = \"hold\"\ntask_force = 1"),
    )
}

fn deny_of_x() -> String {
    let t = changed(
        &text(),
        "intent_kind = \"stand_at\"\ntask_force = 1\nhex = [4, 0]",
        "intent_kind = \"deny\"\nhex = [4, 0]\ndistance = 0",
    );
    at(
        &t,
        "commands",
        &command(
            1,
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\ntask_forces = [1]",
        ),
    )
}

fn readiness_after(played: &Played, day: usize, f: u32) -> Readiness {
    played.truth[day].task_forces[&force(f)].readiness
}

fn speed_after(played: &Played, day: usize, f: u32) -> SpeedSetting {
    played.truth[day].task_forces[&force(f)].speed
}

// ---------------------------------------------------------------- Statements

#[test]
fn the_file_is_the_setting_the_doc_comment_describes() {
    let played = play(&text());
    // G: four ships, stopped at sea at the start, no home port, cruise.
    let g = &played.truth[0].task_forces[&force(1)];
    assert_eq!(g.position(), START);
    assert_eq!(g.home_port, None);
    assert_eq!(g.speed, SpeedSetting::Cruise);
    assert_eq!(g.ships.len(), 4);
    assert_eq!(played.truth[0].bases[&base(1)].hex, B);
}

/// Statement 1. The reason, the route and the absence of "away from home".
/// (At press the code sets G to battle readiness every morning, as
/// statement 6 says, so the readiness half of statement 1's "The same at
/// press" is tested at statement 6.)
#[test]
fn statement_1_todays_answer_is_gone_g_is_routed_to_x_and_stands_from_where_it_is() {
    let balanced = play(&text());
    assert_eq!(balanced.orders(1), vec![route(&[(4, 0)])]);
    let said = balanced.said(1);
    assert!(said.contains("has no home port and stands from where it is"));
    assert!(!said.contains("away from"));
    assert!(!said.contains("gives it no order"));
    // At normal readiness: no order for readiness, and G is at normal.
    assert_eq!(readiness_after(&balanced, 1, 1), Readiness::Normal);

    let press = play(&at_station_and_risk(4, "press"));
    assert_eq!(press.route_of(1, 1), Some(route(&[(4, 0)])));
    let said = press.said(1);
    assert!(said.contains("has no home port and stands from where it is"));
    assert!(!said.contains("away from"));
}

/// Statement 2. 4 hexes: 4 x 40 = 160 nm; 160 / 15 = 10.666 hours at
/// cruise, within the night's 12.
#[test]
fn statement_2_cruise_where_the_night_is_enough() {
    let played = play(&text());
    assert_eq!(speed_after(&played, 0, 1), SpeedSetting::Cruise);
    // G is already at cruise, so no order for speed is needed; none says full.
    assert!(!played.orders(1).contains(&FULL));
    assert_eq!(played.orders(1), vec![route(&[(4, 0)])]);
    let said = played.said(1);
    assert!(said.contains("the station is 10.666666 hours away at cruise and 5.333333 at full"));
    assert!(said.contains("It is routed to the station (4,0) at cruise"));
    // G arrives in the night: at X at the end of day 1, at cruise.
    assert_eq!(played.position(1, 1), hex(4, 0));
    assert_eq!(speed_after(&played, 1, 1), SpeedSetting::Cruise);
    // It makes the 160 nm at 15 kn in 10.67 hours, within the night's 12.
    assert!(Fixed::from_ratio(160, 15) <= n(12));
}

/// Statement 3. 8 hexes: 8 x 40 = 320 nm; 320 / 15 = 21.33 hours at cruise,
/// 320 / 30 = 10.67 at full speed.
#[test]
fn statement_3_full_where_it_is_not() {
    let played = play(&station_at(&text(), 8));
    assert_eq!(played.orders(1), vec![FULL, route(&[(8, 0)])]);
    let said = played.said(1);
    assert!(said.contains("the station is 21.333333 hours away at cruise and 10.666666 at full"));
    assert!(said.contains("It is routed to the station (8,0) at full speed"));
    assert_eq!(speed_after(&played, 1, 1), SpeedSetting::Full);
    assert_eq!(played.position(1, 1), hex(8, 0));
}

/// Statement 4. 12 hexes: 480 nm; 480 / 15 = 32 hours at cruise, 480 / 30 =
/// 16 at full. G goes; H, with a home port 12 hexes from X, does not sortie.
#[test]
fn statement_4_it_goes_though_the_night_is_not_enough_and_a_force_with_a_home_port_does_not() {
    let played = play(&station_at(&text(), 12));
    assert_eq!(played.orders(1), vec![FULL, route(&[(12, 0)])]);
    let said = played.said(1);
    assert!(said.contains("the station is 32 hours away at cruise and 16 at full"));
    assert!(!played.cannot_meet(1), "G's intent is met: it goes");
    // 16 hours at full: it arrives within the day.
    assert_eq!(played.position(1, 1), hex(12, 0));

    // H, stopped at base 3 at (0,1), 12 hexes from X, with that home port,
    // at press: not even full speed brings it within the night.
    let t = station_at(&text(), 12);
    let t = at(&t, "bases", &base_at(3, 0, 1, 5000));
    let t = at(&t, "forces", &force_h("at_base = 3", "home_port = 3", 12));
    let t = at(&t, "commands", &risk_of(2, "press"));
    let played = play(&t);
    assert_eq!(played.route_of(1, 1), Some(route(&[(12, 0)])), "G goes");
    assert!(played.route_of(1, 2).is_none(), "H does not sortie");
    assert!(
        played.cannot_meet(1),
        "the notice that H's intent cannot be met"
    );
    assert!(played.said_of(1, 2).contains("not even full speed brings"));
}

/// Statement 5, at X 4 hexes away: G arrives, lies stopped with no mark and
/// no leg home, and is given no route on days 2, 3 and 4.
#[test]
fn statement_5_it_arrives_and_stays() {
    let played = play(&text());
    // The route has no mark: the last hex of a route is never marked.
    match played.route_of(1, 1) {
        Some(TaskForceOrder::SetRoute {
            waypoints,
            act_at,
            stand_at,
        }) => {
            assert_eq!(waypoints, vec![hex(4, 0)], "no leg home");
            assert!(act_at.is_empty() && stand_at.is_empty(), "no mark");
        }
        other => panic!("no route on day 1: {other:?}"),
    }
    assert!(!played.said(1).contains("mark"));
    for day in 2..=4u32 {
        assert!(played.route_of(day, 1).is_none(), "day {day}: no route");
        assert!(played.orders(day).is_empty(), "day {day}: no order");
        let said = played.said(day);
        assert!(said.contains("lies stopped at its station"), "day {day}");
    }
    for day in 1..=4usize {
        assert_eq!(played.position(day, 1), hex(4, 0), "day {day}: at X");
    }
}

/// Statement 5, the morning it is under way. X is 20 hexes away (800 nm; 800
/// / 30 = 26.7 hours at full speed, more than a day), B holds no fuel so
/// that the fuel is not weighed and the force goes.
#[test]
fn statement_5_on_a_morning_it_is_under_way_to_x_it_is_given_no_route() {
    let t = b_fuel(&station_at(&text(), 20), 0);
    let played = play(&t);
    assert_eq!(played.route_of(1, 1), Some(route(&[(20, 0)])));
    assert_ne!(
        played.position(1, 1),
        hex(20, 0),
        "under way at the end of day 1"
    );
    assert!(played.route_of(2, 1).is_none());
    assert!(played.orders(2).is_empty());
    assert!(played.said(2).contains("under way to its station"));
    // It arrives on day 2 (26.7 hours in all) and then lies stopped.
    assert_eq!(played.position(2, 1), hex(20, 0));
    assert!(played.route_of(3, 1).is_none());
    assert!(played.said(3).contains("lies stopped at its station"));
}

/// Statement 6. At balanced, with no contact of ships within 15 hexes of X,
/// G is at normal readiness; with a contact of ships held 10 hexes from X it
/// is set to battle; at press it is at battle readiness every morning.
#[test]
fn statement_6_the_threat_sets_its_readiness_not_whether_it_goes() {
    // At X, no contact: no order, G at normal readiness on every day.
    let at_x = g_at(&text(), 4, 0);
    let none = play(&at_x);
    for day in 1..=4usize {
        assert!(none.orders(day as u32).is_empty(), "day {day}");
        assert_eq!(readiness_after(&none, day, 1), Readiness::Normal);
    }
    // A contact 20 hexes from X is no threat: the same.
    let far = play(&at(&at_x, "contacts", &contact(24, 3)));
    assert!(far.orders(1).is_empty());
    assert_eq!(readiness_after(&far, 1, 1), Readiness::Normal);
    // A contact of ships 10 hexes from X (within the threat's 15): battle.
    const _: () = assert!(10 <= THREAT_HEXES);
    let near = play(&at(&at_x, "contacts", &contact(14, 3)));
    assert_eq!(
        near.orders(1),
        vec![BATTLE],
        "set to battle, given no route"
    );
    assert_eq!(readiness_after(&near, 1, 1), Readiness::Battle);
    assert!(near.said(1).contains("a threat"));
    // On the way, the same contact does not stop it going: it goes at once
    // and is set to battle.
    let going = play(&at(&text(), "contacts", &contact(14, 3)));
    assert_eq!(going.orders(1), vec![BATTLE, route(&[(4, 0)])]);

    // At press it is at battle readiness every morning.
    let press = play(&at_station_and_risk(4, "press"));
    for day in 1..=4usize {
        assert_eq!(readiness_after(&press, day, 1), Readiness::Battle);
    }
    assert_eq!(press.orders(1), vec![BATTLE, route(&[(4, 0)])]);
}

/// Statement 6a. At preserve the force, a contact of 9 ships 10 hexes from X
/// against G's 4: G is not routed, and the reason names the contact and the
/// numbers. With a contact of 3 ships it goes.
#[test]
fn statement_6a_preserve_the_force_keeps_clear_of_what_outnumbers_it() {
    let preserve = at_station_and_risk(4, "preserve");
    let nine = play(&at(&preserve, "contacts", &contact(14, 9)));
    assert!(nine.decisions(1).orders.is_empty(), "no order at all");
    let said = nine.said(1);
    assert!(said.contains("contact 1 of 9 ship(s) is 10 hexes from the station (4,0)"));
    assert!(said.contains("outnumbers the 4"));
    assert!(said.contains("does not go to its station"));
    assert_eq!(nine.position(1, 1), START, "G stays where it is");

    let three = play(&at(&preserve, "contacts", &contact(14, 3)));
    assert_eq!(three.route_of(1, 1), Some(route(&[(4, 0)])));
    assert_eq!(three.orders(1), vec![BATTLE, route(&[(4, 0)])]);
    // The same 9 ships at press or balanced do not hold it back.
    for level in ["balanced", "press"] {
        let t = at(&at_station_and_risk(4, level), "contacts", &contact(14, 9));
        assert_eq!(play(&t).route_of(1, 1), Some(route(&[(4, 0)])), "{level}");
    }
    // At X at preserve the force with the 9 ships near, it is given nothing.
    let at_x = at(&g_at(&preserve, 4, 0), "contacts", &contact(14, 9));
    let held = play(&at_x);
    assert!(held.decisions(1).orders.is_empty());
    assert!(held.said(1).contains("outnumbers the 4"));
}

/// Statement 7. X at 4 hexes, B 5 hexes from it by sea: 160 nm to X and 200
/// nm from X to B, 360 nm in all. A cruiser burns 2,000 t / 8,000 nm = 0.25 t
/// a nm at cruise, a destroyer 500 / 4,000 = 0.125. At press (margin one) the
/// need is 360 x 0.25 = 90 t for a cruiser and 360 x 0.125 = 45 t for a
/// destroyer.
#[test]
fn statement_7_fuel_is_weighed_to_a_base_that_can_fuel_it() {
    let press = at_station_and_risk(4, "press");
    let played = play(&press);
    assert_eq!(played.route_of(1, 1), Some(route(&[(4, 0)])));
    let said = played.said(1);
    assert!(said.contains("its fuel is weighed to base:1"));
    assert!(said.contains("200 nm from the station"));
    assert!(said.contains("the passage to the station and from there to base:1"));
    assert!(said.contains("times the 1 of the press risk"));
    assert!(said.contains("ship:1 has 2000 t aboard against 90 t needed"));
    assert!(said.contains("ship:2 has 2000 t aboard against 90 t needed"));
    assert!(said.contains("ship:3 has 500 t aboard against 45 t needed"));
    assert!(said.contains("ship:4 has 500 t aboard against 45 t needed"));
    // 160 nm to X at 15 kn is the hours the reason gives.
    assert!(said.contains("10.666666 hours away at cruise"));
    // At balanced the margin is 1.2: 90 x 1.2 = 108 and 45 x 1.2 = 54.
    assert_eq!(
        n(90) * margin(sim_core::command::Risk::Balanced),
        n(108),
        "the engine's balanced margin"
    );
    let balanced = play(&text()).said(1);
    assert!(balanced.contains("ship:1 has 2000 t aboard against 108 t needed"));
    assert!(balanced.contains("ship:3 has 500 t aboard against 54 t needed"));

    // One ship's fuel cut below its need (40 t against 45): G does not go.
    let short = play(&ship4_fuel(&press, 40));
    assert!(short.route_of(1, 1).is_none());
    assert_eq!(short.position(1, 1), START);
    let said = short.said(1);
    // The engine's measure for the dry ship is what it has aboard: it runs
    // dry (at 0.125 t a nm, 40 t last 320 nm) short of the 360 nm.
    assert!(said.contains(
        "ship:4 has 40 t aboard against more than 40 t needed, running dry short of the passage"
    ));
    assert!(said.contains("short of the fuel it needs: "));
    assert!(said.contains("does not go to its station"));
    // With exactly the need it goes: short is less than.
    let exact = play(&ship4_fuel(&press, 45));
    assert_eq!(exact.route_of(1, 1), Some(route(&[(4, 0)])));
}

/// Statement 7a. With B's fuel stock at nothing the fuel is not weighed and
/// G goes; a nearer base B2, with no fuel, is passed over for B.
#[test]
fn statement_7a_no_base_that_can_fuel_it() {
    let played = play(&b_fuel(&at_station_and_risk(4, "press"), 0));
    assert_eq!(played.route_of(1, 1), Some(route(&[(4, 0)])));
    let said = played.said(1);
    assert!(said.contains(
        "no base of the side holds fuel above its reserve and can fuel it, so its fuel is not weighed"
    ));
    assert!(!said.contains("its fuel is weighed to"));

    // B2, at (4,2), 2 hexes from X, holds no fuel: B is the base it is
    // weighed to, as without B2.
    let t = at(
        &at_station_and_risk(4, "press"),
        "bases",
        &base_at(2, 4, 2, 0),
    );
    let passed = play(&t);
    assert_eq!(passed.route_of(1, 1), Some(route(&[(4, 0)])));
    let said = passed.said(1);
    assert!(said.contains("its fuel is weighed to base:1"));
    assert!(!said.contains("weighed to base:2"));
    assert!(said.contains("200 nm from the station"));
    // Were B2 to hold fuel it would be the nearer, weighed to instead.
    let t = at(
        &at_station_and_risk(4, "press"),
        "bases",
        &base_at(2, 4, 2, 1000),
    );
    assert!(play(&t).said(1).contains("its fuel is weighed to base:2"));
}

/// Statement 8, as written: G is at X at press; destroyer 3's fuel is cut to
/// 20 t, below the passage from X to B at cruise: 200 nm x 0.125 t a nm x 1
/// = 25 t. It is routed to B, B is set as its home port, and the reason
/// names the destroyer. (G begins at X with the fuel already cut, so that
/// this is the first morning and not the next. At press the margin is one,
/// so a ship short of the passage cannot make it: 20 t last 160 nm, and the
/// engine's measure says "more than 20 t needed, running dry".)
#[test]
fn statement_8_going_in_for_fuel() {
    let t = ship3_fuel(&g_at(&at_station_and_risk(4, "press"), 4, 0), 20);
    let played = play(&t);
    assert_eq!(
        played.orders(1),
        vec![
            TaskForceOrder::SetHomePort(base(1)),
            BATTLE,
            route(&[(4, 5)]),
        ]
    );
    let said = played.said(1);
    assert!(said.contains(
        "short of the fuel it needs: ship:3 has 20 t aboard against more than 20 t needed, running dry short of the passage"
    ));
    assert!(said.contains("It goes in"));
    assert!(said.contains("routed to base:1"));
    // No mark on the route, and B is its home port from the first morning.
    assert_eq!(
        played.truth[1].task_forces[&force(1)].home_port,
        Some(base(1))
    );
    // With every ship at or above the 25 t of the passage it stays at X.
    let enough = play(&ship3_fuel(
        &g_at(&at_station_and_risk(4, "press"), 4, 0),
        25,
    ));
    assert!(enough.route_of(1, 1).is_none());
    assert_eq!(enough.truth[1].task_forces[&force(1)].home_port, None);
}

/// Statements 8 and 9 through to the sortie. At balanced the margin is 1.2:
/// the passage from X to B at cruise is 25 t x 1.2 = 30 t, and a destroyer
/// with 27 t is short of it yet can make the 25 t. G, at X, goes in, arrives
/// at B in the morning pulse (200 nm at 15 kn is 13.3 hours), fuels, and on
/// day 2, with the risk raised to press by the commander, is a force with a
/// home port: it sorties to X by night and returns, as a stand's force does
/// today, X marked to stand at.
#[test]
fn statements_8_and_9_going_in_then_a_force_with_a_home_port() {
    let t = ship3_fuel(&g_at(&text(), 4, 0), 27);
    let t = changed(&t, "days = 4", "days = 6");
    let t = at(
        &t,
        "commands",
        &command(
            2,
            "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"press\"",
        ),
    );
    let played = play(&t);
    // Day 1: in to B, B its home port, the reason names the destroyer.
    assert_eq!(
        played.orders(1),
        vec![TaskForceOrder::SetHomePort(base(1)), route(&[(4, 5)])]
    );
    let said = played.said(1);
    assert!(said.contains("short of the fuel it needs: ship:3 has 27 t aboard against 30 t needed"));
    assert_eq!(played.position(1, 1), B, "at B by the end of the day");
    assert_eq!(
        played.truth[1].task_forces[&force(1)].home_port,
        Some(base(1))
    );
    // It fuelled in port.
    assert!(played.truth[1].ships[&ship(3)].fuel_t > n(27));
    // Day 2: a force with a home port, stopped at it: the sortie as today,
    // 200 nm from B to X over the night's 12 hours at cruise, so at full
    // speed; X marked, B last.
    assert_eq!(played.orders(2), vec![BATTLE, FULL, sortie((4, 0), (4, 5))]);
    assert!(!played.said(2).contains("stands from where it is"));
    assert_eq!(played.position(2, 1), B, "home again by the end of day 2");
}

/// H, stopped at B with B its home port, at press, standing at X (4,0).
fn h_at_b() -> String {
    let t = at(
        &text(),
        "forces",
        &force_h("at_base = 1", "home_port = 1", 4),
    );
    at(&t, "commands", &risk_of(2, "press"))
}

/// The words the new branch adds to a reason: none of them is in any reason
/// of a force with a home port that is stopped at it.
const NEW_WORDS: [&str; 5] = [
    "stands from where it is",
    "is no longer its side's",
    "goes home at cruise",
    "routed home to",
    "its fuel is weighed to",
];

/// Statement 10. H, of the same ships, stopped at B with B its home port,
/// at press: the stand's sortie as it is today, 200 nm from B to X, 13.3
/// hours at cruise and so full speed (6.7 hours), the route X then B with X
/// marked to stand at, battle readiness; no reason of H's has a word of the
/// new branch.
#[test]
fn statement_10_a_force_with_a_home_port_is_as_today() {
    let played = play(&h_at_b());
    assert_eq!(
        played.orders_of(1, 2),
        vec![BATTLE, FULL, sortie((4, 0), (4, 5))]
    );
    // The rules of engagement come first, as for any stand; no home port
    // order, since B is already its home port.
    let raw: Vec<TaskForceOrder> = played
        .decisions(1)
        .orders
        .iter()
        .map(target_of)
        .filter(|(t, _)| *t == force(2))
        .map(|(_, o)| o)
        .collect();
    assert!(matches!(
        raw[0],
        TaskForceOrder::SetRulesOfEngagement { .. }
    ));
    assert_eq!(raw.len(), 4);
    let said = played.said_of(1, 2);
    assert!(!said.is_empty());
    for words in NEW_WORDS {
        assert!(!said.contains(words), "{words:?} in {said}");
    }
    assert!(said.contains("the station is 13.333333 hours away at cruise and 6.666666 at full"));
    assert!(said.contains("routed to the station (4,0) and home to base:1"));
    // And over the whole game: none of H's reasons has them.
    for day in 1..=4u32 {
        let said = played.said_of(day, 2);
        for words in NEW_WORDS {
            assert!(!said.contains(words), "day {day}: {words:?} in {said}");
        }
    }

    // H under way at sea with B its home port: "away from home", as today.
    // H begins stopped at sea at (-6,-6), 21 hexes from B (840 nm, 56 hours
    // at cruise): on day 1 the staff sends it home (statement 10a), and on
    // day 2 it is under way, away from its home port. (A commander's order
    // to H would hold it, and the staff would say so instead.)
    let t = at(
        &text(),
        "forces",
        &force_h("q = -6\nr = -6", "home_port = 1", 4),
    );
    let away = play(&t);
    assert_eq!(away.orders_of(1, 2), vec![route(&[(4, 5)])]);
    assert_ne!(away.position(1, 2), hex(-6, -6), "H is under way");
    assert!(away.route_of(2, 2).is_none());
    assert!(away.orders_of(2, 2).is_empty());
    let said = away.said_of(2, 2);
    assert!(said.contains("not stopped at its home port"), "{said}");
    for words in NEW_WORDS {
        assert!(!said.contains(words), "{words:?} in {said}");
    }
}

/// Statement 10a. H stopped at sea on no route, B its home port: the staff
/// routes H to B at cruise, and says why. (With B taken by the enemy: see
/// the ignored test below.)
#[test]
fn statement_10a_stopped_at_sea_with_a_home_port() {
    let t = at(
        &text(),
        "forces",
        &force_h("q = 0\nr = -3", "home_port = 1", 4),
    );
    let played = play(&t);
    // H is at cruise already: no order for speed; the route has no mark.
    assert_eq!(played.orders_of(1, 2), vec![route(&[(4, 5)])]);
    let said = played.said_of(1, 2);
    assert!(said.contains("a force stopped at sea goes home at cruise"));
    assert!(said.contains("routed home to base:1"));
    assert!(!said.contains("stands from where it is"));
    // G, with no home port, is unaffected: it goes to X as before.
    assert_eq!(played.orders_of(1, 1), vec![route(&[(4, 0)])]);
}

/// Statement 10a, with B taken by the enemy. No scenario can state it: the
/// loader refuses a home port that is not a base of the force's own side
/// ("is {side} but its home port is {side}"), and the engine clears the home
/// port of a force when its base changes side (`HomePortLost`, decision 0090,
/// 22), so the staff's branch for a home port that is no longer the side's
/// cannot be reached through a scenario. The body is what the statement says.
/// Statement 11. With G held by the commander, the staff gives it no order
/// and says it is held.
#[test]
fn statement_11_held() {
    let played = play(&without_g_commands_hold());
    for day in 1..=4u32 {
        assert!(played.decisions(day).orders.is_empty(), "day {day}");
        assert!(
            played.said(day).contains("is held by the commander"),
            "day {day}"
        );
        assert_eq!(played.position(day as usize, 1), START);
    }
}

/// Statement 12. A deny of the waters round X whose forces limit names G
/// forms G's stand task at X, and statements 1 to 5 hold of it.
#[test]
fn statement_12_under_a_deny() {
    let played = play(&deny_of_x());
    assert_eq!(played.orders(1), vec![route(&[(4, 0)])]);
    let said = played.said(1);
    assert!(said.contains("standing at (4,0) with task_force:1"));
    assert!(said.contains("has no home port and stands from where it is"));
    assert!(!said.contains("away from"));
    assert!(said.contains("10.666666 hours away at cruise"));
    for day in 2..=4u32 {
        assert!(played.route_of(day, 1).is_none(), "day {day}");
        assert!(played.said(day).contains("lies stopped at its station"));
    }
    for day in 1..=4usize {
        assert_eq!(played.position(day, 1), hex(4, 0));
    }
    // At 8 hexes, full speed (statement 3).
    let t = changed(&deny_of_x(), "hex = [4, 0]", "hex = [8, 0]");
    assert_eq!(play(&t).orders(1), vec![FULL, route(&[(8, 0)])]);
}

// ---------------------------------------------------------------- Properties

/// Property 1: in every scenario of data/scenarios with no stand task for a
/// force at sea without a home port, no order of a staff differs from before
/// this rule. The build compares every scenario with the rule on and off and
/// reports it; this walks the scenarios and finds that no staff gives the
/// new branch's reason anywhere.
#[test]
#[ignore = "run by hand: the build's on/off comparison of every scenario is reported in decision 0192's As built"]
fn property_no_scenario_of_the_loops_reaches_the_new_branch() {
    let dir = common::data_dir().join("scenarios");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("the scenarios folder reads")
        .map(|e| e.expect("an entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "toml"))
        .collect();
    files.sort();
    let mut reached = Vec::new();
    for file in files {
        let mut scenario = common::load(&file).expect("the scenario loads");
        let held = staffs();
        loop {
            let summary = scenario
                .next_day_watched(as_staffs(&held), &mut |_, _| {})
                .expect("a day resolves");
            if summary.is_none() {
                break;
            }
            for decisions in scenario.world.staff_handed_in().values() {
                let reasons = decisions
                    .orders
                    .iter()
                    .map(|o| &o.reason)
                    .chain(decisions.no_orders.iter());
                if reasons
                    .map(words)
                    .any(|w| w.contains("stands from where it is"))
                {
                    reached.push(file.display().to_string());
                }
            }
        }
    }
    reached.dedup();
    assert!(
        reached.is_empty(),
        "the new branch is reached in {reached:?}"
    );
}

/// Whether G, on day 1, is routed to X, at `(q, 0)`.
fn goes(text: &str, q: i32) -> bool {
    play(text).route_of(1, 1) == Some(route(&[(q, 0)]))
}

/// Property 2: G is never routed anywhere but X or the base its fuel is
/// weighed to (B), whatever the variant.
#[test]
fn property_g_is_never_routed_anywhere_but_x_or_the_base_its_fuel_is_weighed_to() {
    let mut variants: Vec<(i32, String)> = Vec::new();
    for q in [4, 8, 12] {
        for level in ["balanced", "press", "preserve"] {
            variants.push((q, at_station_and_risk(q, level)));
            variants.push((
                q,
                at(
                    &at_station_and_risk(q, level),
                    "contacts",
                    &contact(q + 10, 3),
                ),
            ));
            variants.push((
                q,
                at(
                    &g_at(&at_station_and_risk(q, level), q, 0),
                    "contacts",
                    &contact(q + 10, 9),
                ),
            ));
        }
        variants.push((q, b_fuel(&at_station_and_risk(q, "press"), 0)));
        variants.push((q, ship4_fuel(&at_station_and_risk(q, "press"), 40)));
        variants.push((
            q,
            ship3_fuel(&g_at(&at_station_and_risk(q, "press"), q, 0), 20),
        ));
    }
    for (q, text) in variants {
        let played = play(&text);
        let mut routes = 0;
        for day in 1..=4u32 {
            for order in played.orders_of(day, 1) {
                if let TaskForceOrder::SetRoute { waypoints, .. } = order {
                    routes += 1;
                    assert!(
                        waypoints == vec![hex(q, 0)] || waypoints == vec![hex(4, 5)],
                        "q = {q}, day {day}: routed to {waypoints:?}"
                    );
                }
            }
        }
        let _ = routes;
    }
}

/// Property 3: a force that goes to its station at balanced goes at press.
/// Across the station's distance and a ship's fuel, balanced gives it a
/// route to X only where press does.
#[test]
fn property_a_force_that_goes_at_balanced_goes_at_press() {
    let mut went = 0;
    for q in [4, 8, 12] {
        for fuel in [20, 40, 45, 60, 100, 200, 300, 380, 400, 456, 500] {
            let balanced = goes(&ship4_fuel(&at_station_and_risk(q, "balanced"), fuel), q);
            let press = goes(&ship4_fuel(&at_station_and_risk(q, "press"), fuel), q);
            if balanced {
                went += 1;
                assert!(
                    press,
                    "q = {q}, fuel {fuel}: goes at balanced, not at press"
                );
            }
        }
    }
    assert!(went > 3, "the property is tried on cases that go");
}

/// Property 4: more fuel aboard never stops a force going to its station:
/// raising a ship's fuel turns no go into a no-go. Run on the station 4 hexes
/// away, where cruise is the speed whatever the fuel.
#[test]
fn property_more_fuel_aboard_never_stops_a_force_going_at_cruise() {
    let mut turned = 0;
    for level in ["balanced", "press"] {
        let mut last = false;
        for fuel in [5, 20, 40, 45, 60, 100, 200, 300, 380, 400, 456, 500] {
            let now = goes(&ship4_fuel(&at_station_and_risk(4, level), fuel), 4);
            assert!(
                now || !last,
                "{level}: goes with less fuel, not with {fuel} t"
            );
            if now && !last {
                turned += 1;
            }
            last = now;
        }
        assert!(last, "{level}: goes with a full tank");
    }
    assert_eq!(turned, 2, "a no-go turns into a go once at each risk");
}

/// Property 4 across the station's distance. At 8 and 12 hexes cruise does
/// not reach X within the night, so the speed is full whatever the fuel
/// aboard, and the fuel is weighed at full speed: a ship turns from a no-go
/// into a go once, as its fuel rises past the need at full speed.
#[test]
fn property_more_fuel_aboard_never_stops_a_force_going() {
    for q in [4, 8, 12] {
        for level in ["balanced", "press"] {
            let mut last = false;
            for fuel in [5, 20, 40, 45, 60, 100, 150, 200, 300, 380, 400, 456, 500] {
                let now = goes(&ship4_fuel(&at_station_and_risk(q, level), fuel), q);
                assert!(
                    now || !last,
                    "q = {q}, {level}: goes with less fuel, not with {fuel} t"
                );
                last = now;
            }
            assert!(last, "q = {q}, {level}: goes with a full tank");
        }
    }
}
