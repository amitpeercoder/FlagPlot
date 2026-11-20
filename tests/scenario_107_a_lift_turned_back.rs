//! Scenario 107: a lift turned back. A staff that has sent men by sea to
//! take a place turns them back while enemy carriers threaten the way, and
//! sends them on again when the threat has passed.
//!
//! Proves decision 0191. The statements are the architect's, written
//! before the code. If the code disagrees, the code or the rule is wrong:
//! a mismatch goes back to the architect, and nobody else changes a
//! statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on the placeholder map, with
//! no real ship or unit. Side J has a base S, and a transport force L at
//! sea with a land unit aboard, three transports and a destroyer, 12
//! hexes from a coast hex P that side A holds. J is given "take the
//! place P", its forces L and the unit, with both leaves. A second J
//! force, K, with a carrier, is formed under the same intent. Side A has
//! a carrier force. Hexes are 40 nm; a fresh contact is one no more than
//! 3 pulses old; the danger's distance is 8 hexes. A lift is turned back
//! when it is under way with men aboard and P is no longer a hex it is to
//! act at on its route.
//!
//! In each statement the lift has been released and is under way to P.
//!
//! # Statements
//!
//! 1. **No danger, no change.** J holds no contact of a carrier. The lift
//!    sails on and lands, at every risk, as it does today.
//! 2. **Preserve the force turns back on the danger.** J holds a fresh
//!    contact reported as a carrier, 6 hexes from the lift's hex. The
//!    next morning the staff routes L to S at its cruise speed. Its men
//!    are aboard. The intent is in force and its plan released. The
//!    reason gives the contact, "carrier", its age in pulses and its
//!    distance, 6 hexes (240 nm), and the risk. L's order to unload is
//!    taken away. The notice
//!    `lift_turned_back` is handed in.
//! 3. **Balanced does not turn back on the sighting alone.** The same
//!    contact, at balanced, with no ship of L or carrier of K lost: L
//!    sails on. The staff's reason says the lift is threatened and has
//!    not been hurt.
//! 4. **Balanced turns back once hurt.** The same, and a transport of L
//!    has been sunk since the plan was released: L is routed to S. The
//!    reason names the ship lost.
//! 5. **A carrier of the side counts.** The same as statement 3, and K's
//!    carrier has been sunk since the release, with no ship of L
//!    touched: L is routed to S. With K's carrier damaged and afloat, L
//!    sails on.
//! 6. **A loss with no danger does not turn it back.** A transport of L
//!    is sunk and J holds no fresh contact of a carrier within 8 hexes of
//!    the lift or of its way to P:
//!    L sails on, at balanced and at preserve the force.
//! 7. **Press never turns back,** with the danger and the hurt of
//!    statement 4.
//! 8. **What counts as the danger.** None of these turns a lift back at
//!    any risk: a contact of a carrier 4 pulses old; a fresh contact of a
//!    carrier 9 hexes from the lift's hex and from every hex of its way
//!    to P; a fresh contact at 6 hexes reported as a battleship that is
//!    in truth a carrier; a fresh carrier contact 3 hexes from the way
//!    home from P and more than 8 from the way to it.
//! 9. **Along the way.** A fresh carrier contact 12 hexes from the
//!    lift's hex and 5 hexes from a hex of the sea route from the lift
//!    to P that is not one of the route's corners: the danger stands.
//! 10. **Going on again, after two clear mornings.** After statement 2,
//!     the contact is not reported again. On the next morning it is 4
//!     pulses old: no danger, and L is not yet sent on. On the morning
//!     after, with no contact of a carrier reported in the last 6
//!     pulses, the take routes L to P again from the hex it is in and
//!     gives it its order to unload. The reason says the danger has
//!     passed for two mornings.
//! 11. **Waiting at its source.** A lift that reaches S while the danger
//!     stands stops there, and no man goes ashore: it has no order to
//!     unload. On the second clear morning it sails for P.
//! 11a. **Preserve the force does not sail into danger.** A lift at S
//!     whose plan is ready, at preserve the force, on a morning of
//!     danger: it is not sent, and the reason says it is held for the
//!     danger. At balanced it sails.
//! 12. **No state.** Saved on the morning after statement 2 and loaded,
//!     the game goes on the same: the staff reads that L is under way
//!     with men aboard on a route that ends at S, and decides from that.
//! 13. **A held lift is left alone.** The commander holds L. With the
//!     danger and the hurt, the staff gives it no order and says it is
//!     held.
//! 14. **A lift that has begun to land is not turned back,** whatever is
//!     sighted.
//! 15. **Not a supply run.** A supply run of "keep a base supplied"
//!     under the same danger does what it does today.
//! 16. **The commander is told and may overrule.** With the standing
//!     interrupt for the notice on, a seat's day stops on the morning L
//!     is turned back. Setting press on the intent sends L on the next
//!     morning though the danger stands.
//! 17. **Truth and belief.** Every decision here is the same when the
//!     enemy's forces are moved in truth and J's contacts are left as
//!     they were.
//!
//! # Properties
//!
//! - With no contact of a carrier held by the side that lifts, no order
//!   of a staff differs from what it gave before this rule.
//! - A lift is never routed to its source by this rule at press.
//! - The men aboard a lift turned back are the men aboard before: none
//!   lands and none is lost by the turning, nor by a going on after it.
//! - Turned back at press implies turned back at balanced, which implies
//!   turned back at preserve the force.
//! - A contact made older or farther never makes a danger that the
//!   newer or nearer one did not.
//!
//! The tests are written by the build under these statements.

// The architect's doc comment above numbers a statement "11a."; clippy reads
// that as an unindented list item. The comment is not changed.
#![allow(clippy::doc_lazy_continuation)]

// ---------------------------------------------------------------- The harness
//
// How the scenario is set up (data/scenarios/lift-turned-back/
// 107_a_lift_turned_back.toml, on data/map/placeholder_a_lift_turned_back.toml):
//
// - Side J is the Japanese side, side A the Allied side. J's base 1 is S at
//   (0,0); A's base 2 is P at (14,0), a coast hex with no man in it.
// - L is task force 1, stopped at sea at (2,0): transports 1 to 3 (600 men
//   each; unit 1's 1,500 men are aboard) and destroyer 4. At 10 knots a day
//   is 6 hexes: it is at (8,0) at the morning of day 2 and at P on day 3.
// - K is task force 2, carrier 5, stopped at (-2,-3). A's carrier force is
//   task force 3, carrier 6, at (2,14); A's cruiser force is task force 4,
//   cruisers 7 and 8, at (20,10). Neither is told anything unless a variant
//   gives it an order.
// - J's commands on day 1: take the place P, forces L and unit 1 with S as
//   the source, both leaves given, at balanced. A staff of J that sees no
//   danger releases the plan on day 1 and routes L to P and home again,
//   acting at P, with the order to unload.
// - A contact is held from the start: a contact entered with age 0 is 3
//   pulses old at the staff's morning of day 2, the first morning the lift
//   is under way. A contact is dropped at 5 pulses.
//
// "The morning" of the statements is the staff's step at the start of a day.
// The lift is first under way on the morning of day 2, which is the morning
// M of the statements (with a contact of age 0 at the start). A contact a
// staff holds can be seen at 1 pulse old on M only by a sighting of the
// morning of day 1: the variants of statements 10, 11, 12 and 16 are made
// with such a sighting (seed 12: the Allied carrier force lies at (7,0), the
// lift meets it there in the morning pulse of day 1, and J's report names a
// carrier), so that the ages of statement 10 are the ones it gives.

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{
    InterruptRef, NoticeKind, PlanState, Reason, StaffDecisions, StaffOrder, Staffs, StandingKind,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{ShipId, TaskForceId};
use sim_core::orders::{OrderKind, SpeedSetting, TaskForceOrder};
use sim_core::state::{Side, TaskForceStatus, TruthState};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const S: Hex = Hex { q: 0, r: 0 };
const P: Hex = Hex { q: 14, r: 0 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/lift-turned-back/107_a_lift_turned_back.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &common::frozen()).expect("the scenario loads")
}

fn load_with(text: &str, options: &sim_data::LoadOptions) -> Scenario {
    common::from_text_with(text, &path(), options).expect("the scenario loads")
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// Adds `what` at the mark `# MARK: name` of the scenario's text; the mark
/// stays, so that more can be added.
fn at(text: &str, name: &str, what: &str) -> String {
    let mark = format!("# MARK: {name}\n");
    changed(text, &mark, &format!("{what}{mark}"))
}

fn contact(q: i32, r: i32, kind: &str, age: u32) -> String {
    format!(
        "[[contact]]\nside = \"japanese\"\nq = {q}\nr = {r}\nship_count = 1\nkind = \"{kind}\"\nage_pulses = {age}\n\n"
    )
}

fn risk_on(day: u32, level: &str) -> String {
    format!(
        "[[command]]\nday = {day}\nside = \"japanese\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{level}\"\n\n"
    )
}

fn route_of(day: u32, force: u32, side: &str, q: i32, r: i32) -> String {
    format!(
        "[[order]]\nday = {day}\nside = \"{side}\"\ntask_force = {force}\nkind = \"set_route\"\nwaypoints = [[{q}, {r}]]\n\n"
    )
}

/// The Allied cruiser force goes to (5,0) on day 1 and so meets the lift
/// there in the night pulse, and then to (5,8) on day 2, out of the way.
/// The first transport is hurt to 95 flotation and the destroyer is left
/// out, so that the cruisers' fire, which falls on the most important kind
/// afloat, falls on the transports and sinks the first.
fn a_transport_sunk(t: &str) -> String {
    let t = changed(
        t,
        "[[ship]]\nid = 4\nname = \"Placeholder destroyer 4\"\nclass = 2\nside = \"japanese\"\nfuel_t = 500\ntask_force = 1\n\n",
        "",
    );
    let t = changed(
        &t,
        "name = \"Placeholder troop transport 1\"\nclass = 10\nside = \"japanese\"\nfuel_t = 1000\n",
        "name = \"Placeholder troop transport 1\"\nclass = 10\nside = \"japanese\"\nfuel_t = 1000\nflotation_damage = 95\n",
    );
    let t = changed(
        &t,
        "q = 20\nr = 10\nmission = \"surface_combat\"",
        "q = 7\nr = 4\nmission = \"surface_combat\"",
    );
    let t = at(&t, "orders", &route_of(1, 4, "allied", 5, 0));
    at(&t, "orders", &route_of(2, 4, "allied", 5, 8))
}

/// The Allied cruiser force goes to K at (-2,-3) on day 1 and sinks its
/// carrier by the afternoon; no ship of L is touched.
fn k_carrier_sunk(t: &str) -> String {
    let t = changed(
        t,
        "q = 20\nr = 10\nmission = \"surface_combat\"",
        "q = 1\nr = -3\nmission = \"surface_combat\"",
    );
    at(&t, "orders", &route_of(1, 4, "allied", -2, -3))
}

/// K's carrier damaged and afloat.
fn hold_ship_on(day: u32, ship: u32) -> String {
    format!("[[command]]\nday = {day}\nside = \"japanese\"\nkind = \"hold\"\nship = {ship}\n\n")
}

/// The lift lies stopped in port at S, with its men aboard.
fn at_port(t: &str) -> String {
    changed(
        t,
        "q = 2\nr = 0\nmission = \"transport\"",
        "at_base = 1\nmission = \"transport\"",
    )
}

/// The lift lies 6 hexes from P, at (8,0): it is at P at the end of day 1.
fn near_p(t: &str) -> String {
    changed(
        t,
        "q = 2\nr = 0\nmission = \"transport\"",
        "q = 8\nr = 0\nmission = \"transport\"",
    )
}

fn k_carrier_damaged(t: &str) -> String {
    changed(
        t,
        "name = \"Placeholder carrier 5\"\nclass = 4\nside = \"japanese\"\nfuel_t = 3000\n",
        "name = \"Placeholder carrier 5\"\nclass = 4\nside = \"japanese\"\nfuel_t = 3000\nflotation_damage = 60\n",
    )
}

/// The sighting of statements 10, 11, 12 and 16: the Allied carrier force
/// steams north from (7,7) along the q = 7 line on day 1, at 16 knots, and
/// is in hex (7,0) at the end of the morning pulse, where the lift is. They
/// meet; the carrier force goes on north and is far away by the next
/// morning. With seed 12, J's report of the meeting names a carrier, one,
/// at (7,0). It is made in the morning pulse, so it is 1 pulse old at the
/// morning of day 2, 4 at the morning of day 3 and is dropped at 5: the
/// ages of statement 10.
fn sighted(t: &str) -> String {
    let t = changed(t, "seed = 1942\n", "seed = 12\n");
    let t = changed(
        &t,
        "q = 2\nr = 14\nmission = \"carrier\"",
        "q = 7\nr = 7\nmission = \"carrier\"",
    );
    at(&t, "orders", &route_of(1, 3, "allied", 7, -4))
}

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    allied: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    hashes: Vec<u64>,
}

fn play(text: &str) -> Played {
    play_from(load(text), BTreeMap::new())
}

/// With the search figures' drop age of a contact at 9 pulses (the live
/// and frozen figures drop one at 5).
fn long_memory() -> sim_data::LoadOptions {
    common::altered_reference("lift_turned_back", |dir| {
        common::edit_file(dir, "search.toml", |text| {
            common::replace_once(
                &text,
                "contact_drop_age_pulses = 5",
                "contact_drop_age_pulses = 9",
            )
        });
    })
}

fn play_from(mut scenario: Scenario, mut handed: BTreeMap<u32, StaffDecisions>) -> Played {
    let held = staffs();
    let mut allied = BTreeMap::new();
    let mut truth = vec![scenario.world.umpire().clone()];
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        let of = |side: Side| {
            scenario
                .world
                .staff_handed_in()
                .get(&side)
                .cloned()
                .unwrap_or_default()
        };
        handed.insert(day, of(Side::Japanese));
        allied.insert(day, of(Side::Allied));
        truth.push(scenario.world.umpire().clone());
    }
    Played {
        handed,
        allied,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        hashes: scenario.world.day_hashes().to_vec(),
    }
}

/// An order of J's staff, plainly.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Landing(u32, Hex, Vec<u32>),
    Route(u32, Vec<Hex>, Vec<Hex>),
    Speed(u32, SpeedSetting),
    Unload(u32),
    NoCargoWork(u32),
    Other(String),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce {
            target,
            order: TaskForceOrder::PrepareLanding { hex, roster },
        } => Given::Landing(target.raw(), *hex, roster.iter().map(|u| u.raw()).collect()),
        OrderKind::TaskForce {
            target,
            order: TaskForceOrder::SetRoute {
                waypoints, act_at, ..
            },
        } => Given::Route(
            target.raw(),
            waypoints.clone(),
            act_at.iter().copied().collect(),
        ),
        OrderKind::TaskForce {
            target,
            order: TaskForceOrder::SetSpeed(speed),
        } => Given::Speed(target.raw(), *speed),
        OrderKind::TaskForce {
            target,
            order: TaskForceOrder::Unload,
        } => Given::Unload(target.raw()),
        OrderKind::TaskForce {
            target,
            order: TaskForceOrder::NoCargoWork,
        } => Given::NoCargoWork(target.raw()),
        other => Given::Other(format!("{other:?}")),
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

impl Played {
    fn days(&self) -> u32 {
        self.handed.len() as u32
    }

    fn orders(&self, day: u32) -> Vec<Given> {
        self.handed[&day].orders.iter().map(given).collect()
    }

    fn all_orders(&self) -> Vec<(u32, Given)> {
        (1..=self.days())
            .flat_map(|d| self.orders(d).into_iter().map(move |g| (d, g)))
            .collect()
    }

    fn notices(&self, day: u32) -> Vec<NoticeKind> {
        self.handed[&day].notices.iter().map(|nt| nt.kind).collect()
    }

    fn turned_back_on(&self) -> Vec<u32> {
        (1..=self.days())
            .filter(|d| self.notices(*d).contains(&NoticeKind::LiftTurnedBack))
            .collect()
    }

    /// Every reason of J's staff on `day`, of orders and decisions to give
    /// no order, in the words the staff wrote.
    fn reasons(&self, day: u32) -> Vec<String> {
        let d = &self.handed[&day];
        d.orders
            .iter()
            .map(|o| text_of(&o.reason))
            .chain(d.no_orders.iter().map(text_of))
            .collect()
    }

    fn all_reasons(&self, day: u32) -> String {
        self.reasons(day).join("\n")
    }

    /// The decisions to give no order on `day` that name the lift's danger.
    fn lift_reasons(&self, day: u32) -> Vec<String> {
        self.handed[&day]
            .no_orders
            .iter()
            .map(text_of)
            .filter(|t| t.contains("task_force:1"))
            .collect()
    }

    /// The state of truth at the morning of `day`: at the end of the day
    /// before.
    fn morning(&self, day: u32) -> &TruthState {
        &self.truth[day as usize - 1]
    }

    fn lift(&self, day: u32) -> &sim_core::state::TaskForce {
        &self.morning(day).task_forces[&TaskForceId::new(1)]
    }

    /// The men aboard the lift's ships at the morning of `day`.
    fn aboard(&self, day: u32) -> u32 {
        self.morning(day)
            .ships
            .values()
            .filter(|s| {
                matches!(
                    s.location,
                    sim_core::state::ShipLocation::TaskForce(f) if f == TaskForceId::new(1)
                )
            })
            .filter_map(|s| s.troops)
            .map(|t| t.men)
            .sum()
    }

    fn ashore(&self, day: u32) -> u32 {
        self.morning(day).land_units[&sim_core::ids::LandUnitId::new(1)].men_ashore
    }

    fn sunk(&self) -> Vec<(u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::ShipSunk {
                    ship, task_force, ..
                } => Some((e.day, ship.raw(), task_force.raw())),
                _ => None,
            })
            .collect()
    }

    fn interrupt_days(&self) -> Vec<u32> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::InterruptFired {
                    side: Side::Japanese,
                    interrupt:
                        InterruptRef::Standing {
                            kind: StandingKind::LiftTurnedBack,
                            ..
                        },
                    ..
                } => Some(e.day),
                _ => None,
            })
            .collect()
    }

    fn plan_released_day(&self) -> Option<u32> {
        self.events.iter().find_map(|e| match &e.kind {
            EventKind::PlanStateChanged {
                side: Side::Japanese,
                to: PlanState::Released,
                ..
            } => Some(e.day),
            _ => None,
        })
    }
}

/// A game with the contact `(q, r, kind, age at the start)` held by J,
/// preserve the force set on day `preserve_from` if given.
fn game(contact_of: Option<(i32, i32, &str, u32)>, risk: &[(u32, &str)]) -> String {
    let mut t = text();
    if let Some((q, r, kind, age)) = contact_of {
        t = at(&t, "contacts", &contact(q, r, kind, age));
    }
    for (day, level) in risk {
        t = at(&t, "risk", &risk_on(*day, level));
    }
    t
}

/// The danger of statement 2: a contact of a carrier 6 hexes from the
/// lift's hex at the morning of day 2, 3 pulses old then.
const DANGER: (i32, i32, &str, u32) = (8, 6, "carrier", 0);

fn static_game(risk: &[(u32, &str)]) -> Played {
    play(&game(Some(DANGER), risk))
}

/// The danger with a transport of L sunk on day 1 (statement 4).
fn hurt_game(risk: &[(u32, &str)]) -> Played {
    let mut t = a_transport_sunk(&game(Some(DANGER), &[]));
    for (day, level) in risk {
        t = at(&t, "risk", &risk_on(*day, level));
    }
    play(&t)
}

/// The danger with K's carrier sunk on day 1 (statement 5).
fn k_game(risk: &[(u32, &str)]) -> Played {
    let mut t = k_carrier_sunk(&game(Some(DANGER), &[]));
    for (day, level) in risk {
        t = at(&t, "risk", &risk_on(*day, level));
    }
    play(&t)
}

fn sighted_game(risk: &[(u32, &str)]) -> Played {
    let mut t = sighted(&text());
    for (day, level) in risk {
        t = at(&t, "risk", &risk_on(*day, level));
    }
    play(&t)
}

/// The orders of the doctrine for a lift as it goes, day 1.
fn first_day() -> Vec<Given> {
    vec![
        Given::Landing(1, P, vec![1]),
        Given::Route(1, vec![P, S], vec![P]),
        Given::Unload(1),
    ]
}

fn turn_orders() -> Vec<Given> {
    vec![Given::Route(1, vec![S], vec![]), Given::NoCargoWork(1)]
}

fn go_on_orders() -> Vec<Given> {
    vec![Given::Route(1, vec![P, S], vec![P]), Given::Unload(1)]
}

/// Whether the lift was turned back on any day.
fn turned(g: &Played) -> bool {
    !g.turned_back_on().is_empty()
}

/// No order of the take is an order that turns the lift back.
fn never_turned_by_order(g: &Played) {
    for (day, o) in g.all_orders() {
        assert!(
            !matches!(&o, Given::NoCargoWork(_))
                && !matches!(&o, Given::Route(1, w, a) if w == &vec![S] && a.is_empty()),
            "day {day}: the lift is turned back: {o:?}"
        );
    }
}

fn first_orders_after_day_1(g: &Played) -> Vec<(u32, Given)> {
    g.all_orders()
        .into_iter()
        .filter(|(d, o)| *d > 1 && !matches!(o, Given::Other(_)))
        .collect()
}

// ------------------------------------------------------------- Statement 1

#[test]
fn statement_1_no_danger_no_change() {
    let mut seen: Vec<Vec<(u32, Given)>> = Vec::new();
    for risk in ["press", "balanced", "preserve"] {
        let g = play(&game(None, &[(1, risk)]));
        assert_eq!(g.orders(1), first_day(), "{risk}");
        // Nothing after day 1 changes the lift's orders: no turning back,
        // no notice, no order but the landing of men by the land unit.
        assert!(
            first_orders_after_day_1(&g).is_empty(),
            "{risk}: {:?}",
            first_orders_after_day_1(&g)
        );
        assert!(!turned(&g), "{risk}");
        never_turned_by_order(&g);
        // It lands: the men are ashore at P on the morning of day 4 and
        // none is left aboard.
        assert_eq!(g.ashore(4), 1500, "{risk}");
        assert_eq!(g.aboard(4), 0, "{risk}");
        assert!(g.lift_reasons(2).iter().all(|r| !r.contains("threatened")));
        seen.push(g.all_orders());
    }
    // At every risk the orders are the same.
    assert_eq!(seen[0], seen[1]);
    assert_eq!(seen[1], seen[2]);
}

// ------------------------------------------------------------- Statement 2

#[test]
fn statement_2_preserve_turns_back_on_the_danger() {
    let g = static_game(&[(2, "preserve")]);
    // On the morning of day 2 the lift is under way with men aboard and the
    // plan is released.
    assert_eq!(g.lift(2).status, TaskForceStatus::Underway);
    assert_eq!(g.lift(2).position(), Hex::new(8, 0));
    assert_eq!(g.aboard(2), 1500);
    assert_eq!(g.plan_released_day(), Some(1));
    // It is routed to S, at its cruise speed: the staff's orders are the
    // route to S (no place to act at) and the order to do no cargo work.
    assert_eq!(g.orders(2), turn_orders());
    assert_eq!(g.lift(2).speed, SpeedSetting::Cruise);
    // The reason gives the contact, "carrier", its age and distance, and
    // the risk.
    let all = g.all_reasons(2);
    assert_says(
        &all,
        &[
            "contact 1",
            "reported as a carrier",
            "3 pulses old",
            "6 hexes (240 nm)",
            "At preserve risk",
        ],
    );
    // The intent stays in force and its plan released.
    assert!(g.events.iter().all(|e| !matches!(
        &e.kind,
        EventKind::PlanStateChanged {
            to: PlanState::Complete | PlanState::Cancelled | PlanState::Failed,
            ..
        } if e.day <= 3
    )));
    // The order to unload is taken away; the men are still aboard.
    assert!(g.lift(2).cargo_order.is_some());
    assert!(g.lift(3).cargo_order.is_none());
    assert_eq!(g.aboard(3), 1500);
    assert_eq!(g.lift(3).route.waypoints, vec![S]);
    // The notice is handed in, once, on that morning.
    assert_eq!(g.turned_back_on(), vec![2]);
    assert_eq!(g.handed[&2].notices.len(), 1);
    assert_eq!(
        g.handed[&2].notices[0].plan,
        sim_core::command::PlanId::new(1)
    );
}

// ------------------------------------------------------------- Statement 3

#[test]
fn statement_3_balanced_does_not_turn_back_on_the_sighting_alone() {
    let g = static_game(&[]);
    assert!(g.sunk().is_empty());
    assert_eq!(g.orders(2), vec![]);
    assert!(!turned(&g));
    never_turned_by_order(&g);
    let all = g.lift_reasons(2).join("\n");
    assert_says(
        &all,
        &[
            "is threatened",
            "balanced risk a lift turns back on the danger only once it has been hurt",
            "no ship of the lift, of its escort or carrier of the side has been sunk",
        ],
    );
    // It sails on and lands.
    assert_eq!(g.ashore(4), 1500);
    assert_eq!(g.aboard(4), 0);
}

// ------------------------------------------------------------- Statement 4

#[test]
fn statement_4_balanced_turns_back_once_hurt() {
    let g = hurt_game(&[]);
    // A transport of L was sunk on day 1, after the release.
    assert_eq!(g.sunk(), vec![(1, 1, 1)]);
    assert_eq!(g.plan_released_day(), Some(1));
    let released = g
        .events
        .iter()
        .position(|e| {
            matches!(
                &e.kind,
                EventKind::PlanStateChanged {
                    to: PlanState::Released,
                    ..
                }
            )
        })
        .expect("released");
    let sunk = g
        .events
        .iter()
        .position(|e| matches!(&e.kind, EventKind::ShipSunk { .. }))
        .expect("sunk");
    assert!(released < sunk);
    assert_eq!(g.orders(2), turn_orders());
    assert_eq!(g.turned_back_on(), vec![2]);
    // The reason names the ship lost.
    let all = g.all_reasons(2);
    assert_says(
        &all,
        &[
            "At balanced risk a lift turns back on the danger once it has been hurt",
            "ship:1 (Placeholder troop transport 1)",
            "sunk on day 1",
            "has been sunk",
        ],
    );
}

// ------------------------------------------------------------- Statement 5

#[test]
fn statement_5_a_carrier_of_the_side_counts() {
    let g = k_game(&[]);
    // K's carrier is sunk on day 1; no ship of L is touched.
    assert_eq!(g.sunk(), vec![(1, 5, 2)]);
    assert!(g
        .morning(2)
        .ships
        .values()
        .filter(|s| matches!(
            s.location,
            sim_core::state::ShipLocation::TaskForce(f) if f == TaskForceId::new(1)
        ))
        .all(|s| s.flotation_damage == Fixed::ZERO));
    assert_eq!(g.aboard(2), 1500);
    assert_eq!(g.orders(2), turn_orders());
    assert_says(
        &g.all_reasons(2),
        &[
            "ship:5 (Placeholder carrier 5)",
            "a carrier of the side, of task_force:2",
        ],
    );
    // With K's carrier damaged and afloat, L sails on.
    let damaged = play(&at(
        &k_carrier_damaged(&text()),
        "contacts",
        &contact(8, 6, "carrier", 0),
    ));
    assert!(damaged.sunk().is_empty());
    assert_eq!(
        damaged.morning(2).ships[&ShipId::new(5)].flotation_damage,
        Fixed::from_int(60)
    );
    assert_eq!(damaged.orders(2), vec![]);
    assert!(!turned(&damaged));
    assert_eq!(damaged.ashore(4), 1500);
}

// ------------------------------------------------------------- Statement 6

#[test]
fn statement_6_a_loss_with_no_danger_does_not_turn_it_back() {
    for risk in ["balanced", "preserve"] {
        // No contact at all.
        let none = play(&a_transport_sunk(&game(None, &[(1, risk)])));
        assert_eq!(none.sunk(), vec![(1, 1, 1)], "{risk}");
        assert!(!turned(&none), "{risk}");
        never_turned_by_order(&none);
        assert_eq!(none.ashore(4), 900, "{risk}");
        // A fresh contact of a carrier more than 8 hexes from the lift and
        // from every hex of its way, and more than 15 from P, where a
        // threat to a run would be read.
        let far = play(&a_transport_sunk(&game(
            Some((-2, 12, "carrier", 0)),
            &[(1, risk)],
        )));
        assert_eq!(far.sunk(), vec![(1, 1, 1)], "{risk}");
        assert!(!turned(&far), "{risk}");
        never_turned_by_order(&far);
        assert_eq!(far.ashore(4), 900, "{risk}");
    }
}

// ------------------------------------------------------------- Statement 7

#[test]
fn statement_7_press_never_turns_back() {
    let games = [
        ("danger", static_game(&[(1, "press")])),
        ("hurt", hurt_game(&[(1, "press")])),
        ("carrier", k_game(&[(1, "press")])),
    ];
    for (name, g) in games {
        assert!(!turned(&g), "{name}");
        never_turned_by_order(&g);
        // The staff says so on the morning of day 2.
        assert_says(
            &g.lift_reasons(2).join("\n"),
            &["press risk a lift is never turned back"],
        );
        assert!(g.ashore(4) > 0, "{name}: it lands");
        assert_eq!(g.aboard(4), 0, "{name}");
    }
}

// ------------------------------------------------------------- Statement 8

/// Each of the four contacts that make no danger, at every risk: preserve
/// the force, balanced with a transport lost, and press.
#[test]
fn statement_8_what_counts_as_the_danger() {
    // (q, r, kind, age at the start, a note)
    let contacts: [(i32, i32, &str, u32); 4] = [
        // A carrier 4 pulses old on the morning of day 2.
        (8, 6, "carrier", 1),
        // 9 hexes from the lift's hex and from every hex of its way.
        (8, 9, "carrier", 0),
        // A battleship, in truth a carrier, at 6 hexes.
        (8, 6, "battleship", 0),
        // 3 hexes from the way home from P (the source is moved to (0,12),
        // and P to it runs through (7,6)), more than 8 from the way out.
        (7, 9, "carrier", 0),
    ];
    for (i, c) in contacts.into_iter().enumerate() {
        let source_far = |t: String| {
            if i == 3 {
                changed(
                    &t,
                    "q = 0\nr = 0\nsupply_t = 1000\nfuel_t = 2000\n",
                    "q = 0\nr = 12\nsupply_t = 1000\nfuel_t = 2000\n",
                )
            } else {
                t
            }
        };
        for risk in ["preserve", "balanced", "press"] {
            // Preserve is set on day 2, on the morning the lift is under way
            // (set from day 1 it would hold the lift in the sea for the
            // fresh contact it holds then, which is statement 9's). Balanced
            // is read with a transport lost on day 1, the hurt that turns it
            // back on a danger.
            let t = match risk {
                "preserve" => game(Some(c), &[(2, "preserve")]),
                "press" => game(Some(c), &[(1, "press")]),
                _ => a_transport_sunk(&game(Some(c), &[])),
            };
            let g = play(&source_far(t));
            assert!(!turned(&g), "contact {i}, {risk}");
            never_turned_by_order(&g);
            assert!(g.ashore(4) > 0 || g.ashore(5) > 0, "contact {i}, {risk}");
        }
    }
    // The 3 hexes of the fourth contact from the way home, by a straight
    // line of hexes from P to the source at (0,12), is a premise of the
    // statement, not the engine's: the line passes through (7,6), which is
    // 3 hexes from (7,9).
    assert_eq!(Hex::new(7, 6).distance(Hex::new(7, 9)), 3);
    assert!(Hex::new(7, 9).distance(Hex::new(8, 0)) > 8);
}

// ------------------------------------------------------------- Statement 9

#[test]
fn statement_9_along_the_way() {
    // A fresh carrier contact 12 hexes from the lift's hex at (2,0) and 5
    // hexes from hexes of its way that are not a corner of its route: (9,5).
    let c = Some((9, 5, "carrier", 0));
    assert_eq!(Hex::new(9, 5).distance(Hex::new(2, 0)), 12);
    assert_eq!(Hex::new(9, 5).distance(Hex::new(9, 0)), 5);
    // On day 1, at preserve the force, with the lift still stopped in the
    // sea: it is held for the danger; at balanced it sails.
    let held = play(&game(c, &[(1, "preserve")]));
    assert_eq!(held.orders(1), vec![Given::Landing(1, P, vec![1])]);
    assert_says(
        &held.all_reasons(1),
        &[
            "held for the danger",
            "5 hexes (200 nm) from the lift's way to the place",
        ],
    );
    let sails = play(&game(c, &[]));
    assert_eq!(sails.orders(1), first_day());
    // At the morning of day 2, with the lift at (8,0), the danger stands.
    let g = play(&game(c, &[(2, "preserve")]));
    assert_eq!(g.orders(2), turn_orders());
    assert_says(&g.all_reasons(2), &["5 hexes (200 nm) from the lift's way"]);
}

// ------------------------------------------------------------ Statement 11a

#[test]
fn statement_11a_preserve_does_not_sail_into_danger() {
    let at_s = |risk: &str| {
        let t = at_port(&at(&text(), "contacts", &contact(6, 6, "carrier", 0)));
        play(&at(&t, "risk", &risk_on(1, risk)))
    };
    let held = at_s("preserve");
    assert_eq!(held.orders(1), vec![Given::Landing(1, P, vec![1])]);
    assert_says(
        &held.all_reasons(1),
        &["it is held for the danger", "At preserve risk"],
    );
    assert_eq!(held.lift(2).position(), S);
    assert_eq!(held.lift(2).status, TaskForceStatus::Idle);
    // It is held while the danger stands, and not turned back, for it has
    // not sailed.
    assert!(!turned(&held));
    // At balanced it sails.
    let sails = at_s("balanced");
    assert_eq!(sails.orders(1), first_day());
    assert_eq!(sails.lift(2).status, TaskForceStatus::Underway);
}

// ------------------------------------------------------------- Statement 13

#[test]
fn statement_13_a_held_lift_is_left_alone() {
    // With the danger and the hurt, at balanced.
    let t = at(
        &a_transport_sunk(&game(Some(DANGER), &[])),
        "risk",
        &hold_ship_on(2, 2),
    );
    let hurt = play(&t);
    assert_eq!(hurt.sunk(), vec![(1, 1, 1)]);
    assert_eq!(hurt.orders(2), vec![]);
    assert!(!turned(&hurt));
    never_turned_by_order(&hurt);
    assert_says(
        &hurt.all_reasons(2),
        &["held", "a held lift is given nothing"],
    );
    // With the danger at preserve the force.
    let t = at(
        &game(Some(DANGER), &[(2, "preserve")]),
        "risk",
        &hold_ship_on(2, 1),
    );
    let g = play(&t);
    assert_eq!(g.orders(2), vec![]);
    assert!(!turned(&g));
    assert_says(&g.all_reasons(2), &["held", "a held lift is given nothing"]);
}

// ------------------------------------------------------------- Statement 10

#[test]
fn statement_10_going_on_again_after_two_clear_mornings() {
    let g = sighted_game(&[(2, "preserve")]);
    // The sighting: J's report of the carrier is made in the morning
    // pulse of day 1, so it is 1 pulse old on the morning of day 2.
    assert_eq!(g.orders(2), turn_orders());
    assert_says(
        &g.all_reasons(2),
        &["reported as a carrier", "1 pulse old", "At preserve risk"],
    );
    assert_eq!(g.aboard(2), 1500);
    // The next morning the contact is 4 pulses old: no danger, and L is not
    // yet sent on. It is kept back, and no notice is handed in.
    assert_eq!(g.orders(3), vec![]);
    assert!(g.notices(3).is_empty());
    assert_says(
        &g.lift_reasons(3).join("\n"),
        &[
            "no contact that makes the danger is fresh",
            "4 pulses old",
            "after two clear mornings running",
            "keeps it back",
        ],
    );
    assert_eq!(g.lift(3).route.waypoints, vec![S]);
    assert_eq!(g.aboard(3), 1500);
    // On the morning after, no contact of a carrier has been reported in the
    // last 6 pulses: the take routes L to P again from the hex it is in, and
    // gives it its order to unload.
    assert_eq!(g.orders(4), go_on_orders());
    assert!(g.notices(4).is_empty());
    assert_eq!(g.turned_back_on(), vec![2]);
    // It goes on and lands.
    assert_eq!(g.ashore(7), 1500);
}

// ------------------------------------------------------------- Statement 11

#[test]
fn statement_11_waiting_at_its_source() {
    let g = sighted_game(&[(2, "preserve")]);
    // The lift reaches S on the night of day 3 while the danger stands (the
    // contact is 4 pulses old on the morning of day 3 and a contact is
    // dropped at 5): on the morning of day 4 it lies stopped at S.
    assert_eq!(g.lift(4).position(), S);
    assert_eq!(g.lift(4).status, TaskForceStatus::Arrived);
    // No man has gone ashore: it has no order to unload, and its men are
    // aboard.
    assert!(g.lift(4).cargo_order.is_none());
    assert_eq!(g.aboard(4), 1500);
    assert_eq!(g.ashore(4), 0);
    // On the second clear morning, day 4, it sails for P.
    assert_eq!(g.orders(4), go_on_orders());
    assert_eq!(g.lift(5).status, TaskForceStatus::Underway);
    assert!(g.lift(5).cargo_order.is_some());
}

// ------------------------------------------------------------ Statement 14

#[test]
fn statement_14_a_lift_that_has_reached_the_place_is_not_turned_back() {
    // The lift, 6 hexes from P, is at P at the end of day 1 with its men
    // aboard and the landing to begin; a fresh carrier contact 6 hexes from
    // P, at preserve the force.
    let t = at(
        &at(&near_p(&text()), "contacts", &contact(13, 6, "carrier", 0)),
        "risk",
        &risk_on(2, "preserve"),
    );
    let g = play(&t);
    assert_eq!(g.lift(2).position(), P);
    assert_eq!(g.lift(2).status, TaskForceStatus::Halted);
    assert_eq!(g.aboard(2), 1500);
    assert!(!turned(&g));
    never_turned_by_order(&g);
    assert_eq!(g.orders(2), vec![]);
    // It lands.
    assert_eq!(g.ashore(3), 1500);
}

// ------------------------------------------------------------ Statement 15

/// A supply run of "keep a base supplied": J's base 3 at (0,12) holds a
/// garrison and little stock, and J's staff is told to keep it at 30 days of
/// supply from S with two cargo ships. It is far from the lift's way.
fn with_a_supply_run(t: &str) -> String {
    let t = changed(
        t,
        "q = 0\nr = 0\nsupply_t = 1000\nfuel_t = 2000\n",
        "q = 0\nr = 0\nsupply_t = 5000\nfuel_t = 2000\n",
    );
    at(
        &t,
        "orders",
        "[[base]]\nid = 3\nname = \"Placeholder base T\"\nside = \"japanese\"\nq = 0\nr = 12\nsupply_t = 100\nfuel_t = 0\n\n\
[[land_unit]]\nid = 2\nname = \"Placeholder garrison\"\nside = \"japanese\"\nmen = 2000\nq = 0\nr = 12\nposture = \"defend\"\n\n\
[[ship]]\nid = 10\nname = \"Placeholder cargo ship 10\"\nclass = 3\nside = \"japanese\"\nfuel_t = 1000\nport = 1\n\n\
[[ship]]\nid = 11\nname = \"Placeholder cargo ship 11\"\nclass = 3\nside = \"japanese\"\nfuel_t = 1000\nport = 1\n\n\
[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_intent\"\nintent = 2\nbase = 3\nfloor_days = 30\ninterval_days = 15\n\n\
[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\nships = [10, 11]\ntask_forces = []\nsource = 1\n\n\
[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_limit\"\nlimit = \"reserve\"\nbase = 1\nsupply_t = 1000\nfuel_t = 0\n\n",
    )
}

// The words of statement 10: "The reason says the danger has passed for two
// mornings." The doctrine adds them where a contact of a carrier is still
// held near the way, older than 6 pulses. With the figures as they are a
// contact is dropped at 5 pulses, so none is ever held at that age and the
// words are never said; with a drop age of 9 they are.
#[test]
fn statement_10_the_reason_says_the_danger_has_passed_with_a_long_memory() {
    // At balanced, with K's carrier sunk on day 1 (the hurt), for at
    // preserve the old rule of a threatened run, which reads a contact of any
    // age, holds the lift while one is held.
    let t = sighted(&k_carrier_sunk(&text()));
    let g = play_from(load_with(&t, &long_memory()), BTreeMap::new());
    assert_eq!(g.orders(2), turn_orders());
    assert_eq!(g.orders(3), vec![]);
    assert_eq!(g.orders(4), go_on_orders());
    assert_says(
        &g.all_reasons(4),
        &["The danger has passed for two mornings"],
    );
}

#[test]
fn statement_10_the_reason_says_the_danger_has_passed() {
    let g = play(&sighted(&k_carrier_sunk(&text())));
    assert_eq!(g.orders(4), go_on_orders());
    assert_says(
        &g.all_reasons(4),
        &["The danger has passed for two mornings"],
    );
}

// ------------------------------------------------------------ Statement 12

/// A game rebuilt from what was handed in to day `upto`, as a saved game
/// replays.
fn rebuilt(text: &str, through: &Played, upto: u32) -> Scenario {
    let mut scenario = load(text);
    for day in 1..=upto {
        scenario
            .world
            .submit_staff(Side::Japanese, through.handed[&day].clone());
        scenario
            .world
            .submit_staff(Side::Allied, through.allied[&day].clone());
        scenario
            .next_day_watched(Staffs::none(), &mut |_, _| {})
            .expect("a day resolves")
            .expect("a day to resolve");
    }
    scenario
}

#[test]
fn statement_12_no_state() {
    let mut t = sighted(&text());
    t = at(&t, "risk", &risk_on(2, "preserve"));
    let through = play(&t);
    // Saved on the morning after statement 2 (the morning of day 3, after
    // day 2 was resolved) and loaded: the game goes on the same.
    let at_2 = rebuilt(&t, &through, 2);
    assert_eq!(at_2.world.day_hashes(), &through.hashes[..2]);
    let first: BTreeMap<u32, StaffDecisions> =
        (1..=2).map(|d| (d, through.handed[&d].clone())).collect();
    let resumed = play_from(at_2, first);
    assert_eq!(resumed.hashes, through.hashes);
    assert_eq!(resumed.handed, through.handed);
    // The staff reads, on the morning of day 3, that L is under way with
    // men aboard on a route that ends at S, and decides from that: it keeps
    // it back with no order, as the game without a save did.
    assert_eq!(through.lift(3).status, TaskForceStatus::Underway);
    assert_eq!(through.aboard(3), 1500);
    assert_eq!(through.lift(3).route.waypoints, vec![S]);
    assert_eq!(resumed.handed[&3], through.handed[&3]);
    assert_eq!(resumed.orders(3), vec![]);
    assert_says(&resumed.lift_reasons(3).join("\n"), &["keeps it back"]);
    // And a save on the morning of day 4 goes on the same, too.
    let at_3 = rebuilt(&t, &through, 3);
    let first: BTreeMap<u32, StaffDecisions> =
        (1..=3).map(|d| (d, through.handed[&d].clone())).collect();
    let resumed = play_from(at_3, first);
    assert_eq!(resumed.hashes, through.hashes);
    assert_eq!(resumed.handed, through.handed);
    assert_eq!(resumed.orders(4), go_on_orders());
}

// ------------------------------------------------------------ Statement 16

#[test]
fn statement_16_the_commander_is_told_and_may_overrule() {
    // The standing interrupt for the notice is on by default: it fires on
    // the morning L is turned back, and the seat's day stops there.
    let g = sighted_game(&[(2, "preserve")]);
    assert_eq!(g.turned_back_on(), vec![2]);
    assert_eq!(g.interrupt_days(), vec![2]);
    // The commander sets press on the intent the next morning: L is sent on
    // that morning though a contact of a carrier, 4 pulses old, is held and
    // the lift would be kept back at preserve.
    let over = sighted_game(&[(2, "preserve"), (3, "press")]);
    assert_eq!(over.orders(2), turn_orders());
    assert_eq!(over.orders(3), go_on_orders());
    assert_eq!(over.lift(3).position(), Hex::new(2, 0));
    assert_eq!(over.aboard(3), 1500);
    // Without it the lift is kept back that morning.
    assert_eq!(g.orders(3), vec![]);
    // With the standing interrupt switched off by the commander the notice
    // is still handed in and nothing fires.
    let off = play(&at(
        &at(&sighted(&text()), "risk", &risk_on(2, "preserve")),
        "risk",
        "[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_interrupt\"\nstanding = \"lift_turned_back\"\non = false\n\n",
    ));
    assert_eq!(off.turned_back_on(), vec![2]);
    assert!(off.interrupt_days().is_empty());
}

// ------------------------------------------------------------ Statement 17

#[test]
fn statement_17_truth_and_belief() {
    // The game of statement 2. The Allied forces are moved in truth, to
    // places where they meet nothing; J's contacts are left as they were.
    let base_text = game(Some(DANGER), &[(2, "preserve")]);
    let base = play(&base_text);
    let moved = [
        // The carrier force onto the contact's place, and then far away.
        changed(
            &base_text,
            "q = 2\nr = 14\nmission = \"carrier\"",
            "q = 8\nr = 6\nmission = \"carrier\"",
        ),
        changed(
            &base_text,
            "q = 2\nr = 14\nmission = \"carrier\"",
            "q = 18\nr = 14\nmission = \"carrier\"",
        ),
        // The cruiser force far from everything else.
        changed(
            &base_text,
            "q = 20\nr = 10\nmission = \"surface_combat\"",
            "q = -3\nr = 10\nmission = \"surface_combat\"",
        ),
    ];
    for (i, t) in moved.iter().enumerate() {
        let g = play(t);
        // The truth is not the same: the state's hash differs from the
        // first day.
        assert_ne!(g.hashes, base.hashes, "moved {i}");
        // Every decision of J's staff is the same, with its reasons.
        assert_eq!(g.handed, base.handed, "moved {i}");
    }
}

// --------------------------------------------------------------- Properties

#[test]
fn property_no_carrier_contact_changes_nothing() {
    // With no contact of a carrier held by J, no order differs from the game
    // with no contact: a battleship near the lift's way, a carrier far from
    // it and from P, none at all.
    for risk in ["press", "balanced", "preserve"] {
        let none = play(&game(None, &[(1, risk)]));
        for c in [
            (8, 6, "battleship", 0),
            (-2, 12, "carrier", 0),
            (-2, 12, "battleship", 0),
        ] {
            let g = play(&game(Some(c), &[(1, risk)]));
            if c.2 == "carrier" || risk == "press" {
                // A contact that makes no danger and does not threaten P
                // leaves every order as it was.
                assert_eq!(g.all_orders(), none.all_orders(), "{risk} {c:?}");
            }
            assert!(!turned(&g), "{risk} {c:?}");
            never_turned_by_order(&g);
        }
    }
}

#[test]
fn property_a_lift_is_never_routed_to_its_source_by_this_rule_at_press() {
    let games = [
        static_game(&[(1, "press")]),
        hurt_game(&[(1, "press")]),
        k_game(&[(1, "press")]),
        sighted_game(&[(1, "press")]),
        sighted_game(&[(2, "press")]),
    ];
    for g in &games {
        assert!(!turned(g));
        never_turned_by_order(g);
    }
}

#[test]
fn property_the_men_aboard_are_the_men_aboard_before() {
    // The sighted game: turned back on day 2, kept on day 3, on its way
    // again on day 4. The men aboard are 1,500 on every morning until they
    // land; none lands and none is lost by the turning or by the going on.
    let g = sighted_game(&[(2, "preserve")]);
    for day in 1..=6 {
        assert_eq!(g.aboard(day), 1500, "day {day}");
        assert_eq!(g.ashore(day), 0, "day {day}");
    }
    assert_eq!(g.ashore(7), 1500);
    assert_eq!(g.aboard(7), 0);
    // The same of the game of statement 2, with the lift sent on at once.
    let s = static_game(&[(2, "preserve")]);
    for day in 1..=4 {
        assert_eq!(s.aboard(day), 1500, "day {day}");
        assert_eq!(s.ashore(day), 0, "day {day}");
    }
    // With a transport lost on day 1 the men aboard from the morning of
    // the turning are the 900 left, to the landing.
    let h = hurt_game(&[]);
    for day in 2..=5 {
        assert_eq!(h.aboard(day), 900, "day {day}");
        assert_eq!(h.ashore(day), 0, "day {day}");
    }
}

/// Whether the lift is turned back at `risk`, with or without the hurt.
fn turned_at(risk: &str, hurt: bool) -> bool {
    let g = if hurt {
        hurt_game(&[(2, risk)])
    } else {
        static_game(&[(2, risk)])
    };
    turned(&g)
}

#[test]
fn property_turned_back_at_press_implies_balanced_implies_preserve() {
    for hurt in [false, true] {
        let press = turned_at("press", hurt);
        let balanced = turned_at("balanced", hurt);
        let preserve = turned_at("preserve", hurt);
        assert!(!press, "press never turns back (hurt {hurt})");
        assert!(!press || balanced, "hurt {hurt}");
        assert!(!balanced || preserve, "hurt {hurt}");
        // And the table itself.
        assert_eq!(balanced, hurt);
        assert!(preserve);
    }
}

#[test]
fn property_an_older_or_farther_contact_never_makes_the_danger_a_newer_nearer_one_did_not() {
    // Contacts at (8, distance), at 4 to 12 hexes from the lift's way, with
    // age at the start 0 to 3: at preserve the force. The lift is turned back
    // for a contact only where it is fresh and within 8 hexes.
    let turn = |age: u32, d: i32| -> bool {
        let t = game(Some((8, d, "carrier", age)), &[(2, "preserve")]);
        let t = changed(&t, "days = 8\n", "days = 3\n");
        turned(&play(&t))
    };
    let mut table: BTreeMap<(u32, i32), bool> = BTreeMap::new();
    for age in 0..=3 {
        for d in 4..=12 {
            table.insert((age, d), turn(age, d));
        }
    }
    for ((age, d), turned) in &table {
        if let Some(older) = table.get(&(age + 1, *d)) {
            assert!(!older || *turned, "age {age} distance {d}");
        }
        if let Some(farther) = table.get(&(*age, d + 1)) {
            assert!(!farther || *turned, "age {age} distance {d}");
        }
    }
    // And the edges: fresh at 3 pulses (age 0 at the start) and 8 hexes
    // turns it back; 9 hexes, or a contact 1 pulse older, does not.
    assert!(table[&(0, 8)]);
    assert!(!table[&(0, 9)]);
    assert!(!table[&(1, 6)]);
}

/// J's supply run, with a contact of ships near its base T at (0,12), far
/// from the lift's way.
fn supply_game(kind: &str, risk: &str) -> Played {
    play(&with_a_supply_run(&game(
        Some((-3, 15, kind, 0)),
        &[(1, risk)],
    )))
}

#[test]
fn statement_15_not_a_supply_run() {
    for risk in ["preserve", "balanced", "press"] {
        let carrier = supply_game("carrier", risk);
        let battleship = supply_game("battleship", risk);
        // The run is sent: a force formed from a cargo ship, loaded, routed
        // to T and to unload there.
        let run = |g: &Played| -> Vec<(u32, Given)> {
            g.all_orders()
                .into_iter()
                .filter(|(_, o)| {
                    matches!(o, Given::Route(2_000_001, ..) | Given::Unload(2_000_001))
                })
                .collect()
        };
        assert_eq!(
            run(&carrier),
            vec![
                (
                    2,
                    Given::Route(2_000_001, vec![Hex::new(0, 12), S], vec![Hex::new(0, 12)])
                ),
                (2, Given::Unload(2_000_001)),
            ],
            "{risk}"
        );
        // Under the same danger it does what it does with the contact
        // reported as a battleship, the rule of the lift reading only a
        // carrier: every order of the staff is the same, and the run is
        // never turned back, its cargo work never taken away.
        assert_eq!(carrier.all_orders(), battleship.all_orders(), "{risk}");
        assert!(!turned(&carrier), "{risk}");
        never_turned_by_order(&carrier);
        for (_, o) in carrier.all_orders() {
            assert!(!matches!(o, Given::NoCargoWork(_)), "{risk}");
        }
    }
}
