//! Scenario 116: carriers serve an order for an end. A commander's order
//! to deny an area or to neutralise a base uses a free carrier force, and
//! the staff chooses where it works from by the risk, the enemy and the
//! fuel.
//!
//! Proves decision 0202. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on a placeholder map of open
//! sea, with no real ship. Hexes are 40 nm and are named (q,r); the
//! distance between two hexes is in hex steps, and on open sea the way by
//! sea is as many steps.
//!
//! Side A has a carrier force C: one carrier and two destroyers, on the
//! carrier mission, stopped at sea at (10,0). Aboard are a bomber group
//! whose normal strike radius is 210 nm (five hexes are in reach, 200 nm;
//! six are not, 240 nm), a fighter group and a group that searches. C
//! cruises at 15 kn and has a full speed of 30 kn. Each ship's class has a
//! fuel capacity of 1,000 t and an endurance of 4,000 nm, so it burns
//! 0.25 t a mile at cruise and four times that at full speed. A's home
//! port for C, where one is given, is base H at (10,2). A has no aircraft
//! at any base.
//!
//! Side B has what each statement gives it. The fuel margins are 1.5 at
//! preserve, 1.2 at balanced and 1.0 at press. A circle of danger is 15
//! hexes round an enemy base on A's map and 8 hexes round a fresh contact
//! reported as a carrier; a hex at exactly that distance is inside.
//!
//! In each statement A's commander has given leave to launch, unless it
//! says otherwise, and the order has no forces limit.
//!
//! # Statements: deny
//!
//! The order is "deny the waters within 2 hexes of (0,0)".
//!
//! 1. **The candidates.** 210 nm less 40 nm for each of the 2 hexes is
//!    130 nm: three hexes are in reach (120 nm), four are not. The
//!    candidates are the hexes within 3 of (0,0).
//! 2. **With no enemy known, the nearest.** B has no base on A's map and
//!    A holds no contact. At preserve, at balanced and at press the staff
//!    forms a task to operate C from (3,0), the candidate with the least
//!    passage, 7 hexes. Its reason gives the station, 3 hexes east of the
//!    centre of the waters, says no circle of danger reaches it, and
//!    gives the passage.
//! 3. **The speed is the carrier doctrine's.** 7 hexes are 280 nm: 18.7
//!    hours at cruise, more than 12, and 9.3 at full speed. At balanced
//!    and at press C is ordered there at full speed; at preserve at
//!    cruise.
//! 4. **Already in reach, it stays.** With C at (2,0), the candidate with
//!    the least passage is the hex it is in. The task names (2,0) and C
//!    is given no route.
//! 5. **Waters wider than the bombers reach.** For "within 6 hexes of
//!    (0,0)", 210 less 240 is less than one hex. The candidates are the
//!    hexes within 210 nm of (0,0) in a straight line: those within 5
//!    hexes, and the six at the middle of each side of the ring 6 out,
//!    such as (3,3), which is 208 nm away; 97 in all. C is sent to (5,0),
//!    5 hexes of passage. (Corrected by the architect on Nov 21, 2026:
//!    as first written it gave only the hexes within 5.)
//! 6. **A carrier contact casts a circle.** A holds a fresh contact
//!    reported as a carrier at (8,-6). (3,0) is 6 hexes from it and so
//!    inside its circle, by 2. Of the candidates, (0,3) is 9 hexes from
//!    the contact, outside by 1, and 10 hexes of passage from C; no
//!    candidate outside the circle has less.
//!    - At preserve and at balanced C is sent to (0,3). The reason says
//!      it lies outside the circle of that contact, by number and age,
//!      and that a nearer station, (3,0), lies inside it.
//!    - At press C is sent to (3,0), and the reason says it stands inside
//!      the contact's circle.
//! 7. **What casts no circle.** Each of these, in place of statement 6's
//!    contact, leaves the station at (3,0) at every risk: the same
//!    contact reported as a cruiser; the same contact grown too old to
//!    be fresh; a fresh carrier contact at (1,0), inside the waters the
//!    order denies.
//! 8. **An enemy base, and no way clear of it.** B has a base E at
//!    (-10,0) on A's map and A holds no contact. Every candidate is
//!    within 13 hexes of E and so inside its circle.
//!    - At preserve no task is formed for C. The reason says E's circle
//!      holds every candidate. With no base of A's that reaches, the
//!      staff hands in that it cannot meet the order, and names both the
//!      bases and the carrier force.
//!    - At balanced C is sent to (3,0): 13 hexes from E, inside by 2, the
//!      least deep of any candidate, and of the seven candidates equally
//!      deep the one with the least passage. (Corrected by the architect
//!      on Nov 21, 2026: as first written, four.) The reason says it stands
//!      in danger of E.
//!    - At press C is sent to (3,0), the least passage.
//! 9. **The waters and the targets are read.** At its station under the
//!    deny, the hexes of the waters are in C's search arc; and with the
//!    order's targets limit set to transports first, C's bombers are
//!    given transports first.
//! 10. **Its strikes count.** A strike by C's bombers on a ship in the
//!     waters is counted in the deny's measure.
//!
//! # Statements: neutralise
//!
//! B has a base E at (0,0) on A's map. The order is "neutralise E".
//!
//! 11. **The candidates** are the 97 hexes within 210 nm of (0,0), as in
//!     statement 5. Every one is inside E's own circle.
//! 12. **At preserve the carriers do not go.** No task is formed for C,
//!     and the reason says a raid on E is flown from inside E's circle.
//! 13. **At balanced, the edge of the reach.** The six candidates 6 hexes
//!     from E are the least deep, inside by 9. Of those, (3,3) and (6,-3)
//!     have the least passage from C, 7 hexes each, and (3,3) is first in
//!     hex order. C is sent to (3,3), and the reason says it stands in
//!     danger of E. At press it is sent to (5,0), the least passage, 5
//!     hexes. (Corrected by the architect on Nov 21, 2026: as first
//!     written it sent C to (5,0) at balanced too, having left out the
//!     six hexes of statement 5.)
//! 14. **The bombers raid the base.** At press, at (5,0), with no fresh contact of
//!     ships within 210 nm, C's bombers are set to strike E. On a morning
//!     A holds a fresh contact of ships 4 hexes from C, they are set to
//!     strike ships and not E.
//! 15. **The pair is allowed.** The task to operate carriers stands under
//!     the neutralise; today the pair is refused.
//!
//! # Statements: fuel
//!
//! The order is statement 2's deny, the station (3,0), and H at (10,2) is
//! C's home port. The engine charges a clear way by the straight line
//! (decision 0058): from (3,0) to H is 327 nm, and from (10,0) to (3,0)
//! is 280 nm. (Corrected by the architect on Nov 21, 2026: as first
//! written the way home was counted as 9 hexes of 40 nm, 360 nm, and the
//! figures of statements 16 to 18 followed from that.)
//!
//! 16. **What the passage takes.** Out at full speed, 280 nm at 1 t a
//!     mile, 280 t; home at cruise, 327 nm at 0.25 t a mile, 81.75 t:
//!     361.75 t. At press, with every ship holding 400 t, C is sent, and
//!     the reason gives 361.75 t needed against the least aboard.
//! 17. **The margin.** At balanced the need is 361.75 times 1.2, 434.1 t.
//!     With a destroyer at 400 t no candidate is in fuel: the cheapest
//!     other, (2,1), is 302 nm out and 342 nm home, 302 t and 85.5 t,
//!     387.5 t, and 465 t with the margin. No task is formed for C, and the reason says no
//!     candidate is in fuel and names the destroyer. With that destroyer
//!     at 450 t C is sent to (3,0).
//! 18. **Preserve sails at cruise.** At preserve the passage out is at
//!     cruise: 70 t out and 81.75 t home, 151.75 t, and 227.6 t with the
//!     margin of 1.5. With every ship at 250 t C is sent.
//! 19. **In port, it is weighed full.** With C stopped at H, H its home
//!     port, H holding fuel and able to fuel it, and its ships at 100 t:
//!     the task is formed, the station chosen as if the ships were full,
//!     and C fuels before it sails, as the carrier doctrine has it.
//! 20. **No home port.** With no home port C is sent as in statement 2 at
//!     every risk, and the reason says its fuel could not be measured
//!     against a passage home.
//!
//! # Statements: which force, and the commander's word
//!
//! 21. **Not free.** In each of these the staff forms no task for C under
//!     the deny, and C is given no order by it: C has an order of the
//!     commander's own to operate from (10,0); another order's forces
//!     limit names C; C is held.
//! 22. **A hold lifted.** With C held when the deny is given, and the
//!     hold lifted on day 3, the task is formed on the morning of day 4
//!     and the station chosen then, from where C is that morning.
//! 23. **A force the commander named keeps its hex.** With the deny's
//!     forces limit naming C, the task names (10,0), the hex C is in,
//!     and no station is chosen: decision 0125, as today.
//! 24. **One force, two orders.** A gives the deny of statement 2 and the
//!     neutralise of statement 11 at balanced. With the neutralise at the
//!     higher priority C serves the neutralise, and the deny's reason
//!     says the one carrier force is taken. With the two at one priority,
//!     the order with the lower number has it.
//! 25. **Leave withheld.** With leave to launch withheld, no task is
//!     formed and C is given no order. The preview of the order names C
//!     and (3,0). On the morning after the commander approves the plan,
//!     the task is formed and C is routed.
//! 26. **A direct order to a tasked force holds it.** With the task
//!     formed and C under way, the commander routes C himself: the staff
//!     gives C no route the next morning, by the override, as today.
//!
//! # Statements: the search on the way
//!
//! 27. **The way ahead is searched.** On the morning C is ordered from
//!     (10,0) to (3,0), its search arc takes in the hexes of its way,
//!     (9,0) to (3,0), with the waters of the deny; the reason for the
//!     arc names the way ahead to the station. On the morning after, at
//!     (3,0), the arc has the waters and no way.
//!
//! # Properties
//!
//! - **No game on file moves.** Every scenario and recorded game on file
//!   gives the events and the daily hashes it gave before.
//! - **Nothing is drawn.** The choice of a station draws no random number:
//!   the same view gives the same station.
//! - **The staff reads its own side.** Nothing in a reason for a station
//!   names what an enemy base holds.

// Where the build found a statement of the first hand calculation did not
// hold, the architect corrected the statement (each correction is dated in
// the doc comment above) and the tests below are of the statements as they
// now stand.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::choices::Moment;
use sim_core::command::{
    CommandKind, IntentKind, NoticeKind, Reason, Risk, Staff, StaffDecisions, StaffOrder,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId, TaskForceId};
use sim_core::orders::{AirGroupOrder, AirMission, OrderKind, SpeedSetting, TaskForceOrder};
use sim_core::refdata::ShipKind;
use sim_core::state::{Side, TruthState};
use sim_data::{LoadOptions, Scenario};
use sim_staff::station::{margin_of, order_by_risk, reach_for, Candidate, Circle};
use sim_staff::{as_staffs, staffs, SupplyStaff};

// ---------------------------------------------------------------- The harness

fn path() -> PathBuf {
    common::data_dir().join("scenarios/carriers-serve-an-end/116_carriers_serve_an_end.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn hex(q: i32, r: i32) -> Hex {
    Hex { q, r }
}

fn c() -> TaskForceId {
    TaskForceId::new(1)
}

/// Adds `what` at the mark `# MARK: name`; the mark stays.
fn at(text: &str, name: &str, what: &str) -> String {
    let mark = format!("# MARK: {name}\n");
    common::replace_once(text, &mark, &format!("{what}{mark}"))
}

fn command(day: u32, body: &str) -> String {
    format!("[[command]]\nday = {day}\nside = \"allied\"\n{body}\n\n")
}

/// The risk set on `intent`.
fn risk(text: &str, intent: u32, level: &str) -> String {
    at(
        text,
        "commands",
        &command(
            1,
            &format!(
                "kind = \"set_limit\"\nlimit = \"risk\"\nintent = {intent}\nrisk = \"{level}\""
            ),
        ),
    )
}

/// The deny's waters within `distance` hexes of (0,0).
fn waters(text: &str, distance: u32) -> String {
    common::replace_once(
        text,
        "hex = [0, 0]\ndistance = 2",
        &format!("hex = [0, 0]\ndistance = {distance}"),
    )
}

/// C stopped at `(q, r)` instead of (10,0).
fn c_at(text: &str, q: i32, r: i32) -> String {
    common::replace_once(
        text,
        "q = 10\nr = 0\nmission = \"carrier\"",
        &format!("q = {q}\nr = {r}\nmission = \"carrier\""),
    )
}

/// C stopped at base H instead of at sea.
fn c_at_h(text: &str) -> String {
    common::replace_once(
        text,
        "q = 10\nr = 0\nmission = \"carrier\"",
        "at_base = 1\nmission = \"carrier\"",
    )
}

/// C with no home port.
fn no_home(text: &str) -> String {
    common::replace_once(
        text,
        "mission = \"carrier\"\nhome_port = 1\n",
        "mission = \"carrier\"\n",
    )
}

/// Ship `id`'s fuel, in tons, instead of 1,000.
fn fuel(text: &str, id: u32, tons: i64) -> String {
    let name = if id == 1 { "carrier" } else { "destroyer" };
    let class = if id == 1 { 17 } else { 18 };
    let before = format!(
        "name = \"Placeholder {name} {id}\"\nclass = {class}\nside = \"allied\"\nfuel_t = 1000"
    );
    let after = format!(
        "name = \"Placeholder {name} {id}\"\nclass = {class}\nside = \"allied\"\nfuel_t = {tons}"
    );
    common::replace_once(text, &before, &after)
}

/// Every ship of C at `tons`.
fn all_fuel(text: &str, tons: i64) -> String {
    let t = fuel(text, 1, tons);
    let t = fuel(&t, 2, tons);
    fuel(&t, 3, tons)
}

/// B's base E, base 2, at `(q, r)`.
fn base_e(text: &str, q: i32, r: i32) -> String {
    at(
        text,
        "bases",
        &format!(
            "[[base]]\nid = 2\nname = \"Placeholder base E\"\nside = \"japanese\"\nq = {q}\nr = {r}\nsupply_t = 0\n\n"
        ),
    )
}

/// A contact A holds at `(q, r)`, of one ship reported as `kind`, `age`
/// pulses old.
fn contact(text: &str, q: i32, r: i32, kind: &str, age: u32) -> String {
    at(
        text,
        "contacts",
        &format!(
            "[[contact]]\nside = \"allied\"\nq = {q}\nr = {r}\nship_count = 1\nkind = \"{kind}\"\nage_pulses = {age}\n\n"
        ),
    )
}

/// The order of the neutralise statements: E at (0,0) and "neutralise E" in
/// place of the deny, as intent 1.
fn neutralise_only() -> String {
    let t = base_e(&text(), 0, 0);
    common::replace_once(
        &t,
        "intent_kind = \"deny\"\nhex = [0, 0]\ndistance = 2",
        "intent_kind = \"neutralise\"\ntarget = 2",
    )
}

fn hold_c(text: &str, day: u32) -> String {
    at(
        text,
        "commands",
        &command(day, "kind = \"hold\"\ntask_force = 1"),
    )
}

fn release_c(text: &str, day: u32) -> String {
    at(
        text,
        "commands",
        &command(day, "kind = \"release\"\ntask_force = 1"),
    )
}

/// The figures: the frozen step 1 figures, with a strike contact age of 4
/// pulses where `strikes` (statement 10), so that a contact held is fresh
/// enough to launch at.
fn figures(strikes: bool) -> LoadOptions {
    if !strikes {
        return common::frozen();
    }
    common::altered_reference("s116", |dir| {
        common::edit_file(dir, "air_combat.toml", |text| {
            common::replace_once(
                &text,
                "strike_contact_max_age_pulses = 0",
                "strike_contact_max_age_pulses = 4",
            )
        });
    })
}

fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &common::frozen()).expect("the scenario loads")
}

/// A game played with the doctrine's staffs: what the Allied staff handed in
/// each day, and the truth at the end of each day (index 0 is the start).
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    scenario: Scenario,
}

fn play_with(text: &str, options: &LoadOptions, days: u32) -> Played {
    let mut scenario = common::from_text_with(text, &path(), options).expect("the scenario loads");
    let held = staffs();
    let mut handed = BTreeMap::new();
    let mut truth = vec![scenario.world.umpire().clone()];
    loop {
        let day = scenario.world.day() + 1;
        if day > days {
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
                .get(&Side::Allied)
                .cloned()
                .unwrap_or_default(),
        );
        truth.push(scenario.world.umpire().clone());
    }
    Played {
        handed,
        truth,
        scenario,
    }
}

fn play(text: &str, days: u32) -> Played {
    play_with(text, &common::frozen(), days)
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

impl Played {
    fn decisions(&self, day: u32) -> &StaffDecisions {
        &self.handed[&day]
    }

    /// The carrier tasks the staff formed on `day`: (parent, force, hex).
    fn formed(&self, day: u32) -> Vec<(IntentId, TaskForceId, Hex)> {
        self.decisions(day)
            .commands
            .iter()
            .filter_map(|c| match c.kind {
                CommandKind::FormIntent {
                    parent,
                    kind: IntentKind::OperateCarriers { force, hex },
                } => Some((parent, force, hex)),
                _ => None,
            })
            .collect()
    }

    /// The station of the one carrier task formed on `day`, if one.
    fn station(&self, day: u32) -> Option<Hex> {
        let formed = self.formed(day);
        assert!(formed.len() <= 1, "one task at most: {formed:?}");
        formed.first().map(|(_, _, h)| *h)
    }

    /// The reason of the carrier task formed on `day`.
    fn station_reason(&self, day: u32) -> String {
        self.decisions(day)
            .commands
            .iter()
            .find(|c| {
                matches!(
                    c.kind,
                    CommandKind::FormIntent {
                        kind: IntentKind::OperateCarriers { .. },
                        ..
                    }
                )
            })
            .map(|c| words(&c.reason))
            .unwrap_or_default()
    }

    /// The orders `day` gave the task force `f`.
    fn force_orders(&self, day: u32, f: TaskForceId) -> Vec<TaskForceOrder> {
        self.decisions(day)
            .orders
            .iter()
            .filter_map(|o| match &o.kind {
                OrderKind::TaskForce { target, order } if *target == f => Some(order.clone()),
                _ => None,
            })
            .collect()
    }

    fn route_of(&self, day: u32) -> Option<Vec<Hex>> {
        self.force_orders(day, c())
            .into_iter()
            .find_map(|o| match o {
                TaskForceOrder::SetRoute { waypoints, .. } => Some(waypoints),
                _ => None,
            })
    }

    fn group_orders(&self, day: u32, g: u32) -> Vec<AirGroupOrder> {
        self.decisions(day)
            .orders
            .iter()
            .filter_map(|o: &StaffOrder| match &o.kind {
                OrderKind::AirGroup { target, order } if *target == AirGroupId::new(g) => {
                    Some(order.clone())
                }
                _ => None,
            })
            .collect()
    }

    /// Every reason of a day as one text.
    fn said(&self, day: u32) -> String {
        let d = self.decisions(day);
        d.orders
            .iter()
            .map(|o| &o.reason)
            .chain(d.no_orders.iter())
            .chain(d.commands.iter().map(|c| &c.reason))
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

    /// The reason the "cannot meet" notice of `day` rests on.
    fn cannot_meet_reason(&self, day: u32) -> String {
        let d = self.decisions(day);
        d.notices
            .iter()
            .filter(|n| n.kind == NoticeKind::CannotMeet)
            .filter_map(|n| match n.rests_on {
                sim_core::command::NoticeBasis::NoOrder(i) => d.no_orders.get(i).map(words),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn position(&self, day: usize) -> Hex {
        self.truth[day].task_forces[&c()].position()
    }
}

/// Each risk in turn.
const RISKS: [&str; 3] = ["preserve", "balanced", "press"];

fn at_risk(text: &str, level: &str) -> String {
    risk(text, 1, level)
}

// ---------------------------------------------------------------- The setting

#[test]
fn the_file_is_the_setting_the_doc_comment_describes() {
    let s = load(&text());
    let t = s.world.umpire();
    let force = &t.task_forces[&c()];
    assert_eq!(force.position(), hex(10, 0));
    assert_eq!(force.home_port, Some(BaseId::new(1)));
    assert_eq!(force.ships.len(), 3);
    assert_eq!(t.bases[&BaseId::new(1)].hex, hex(10, 2));
    // No aircraft at any base of A's.
    assert!(t
        .air_groups
        .values()
        .all(|g| !matches!(g.location, sim_core::state::AirGroupLocation::Base(_))));
    let view = s.world.side_view(Side::Allied);
    let bomber = view
        .aircraft_type(t.air_groups[&AirGroupId::new(1)].aircraft_type)
        .expect("the bomber's type");
    assert_eq!(bomber.strike_range_nm, Fixed::from_int(210));
    for ship in force.ships.iter() {
        let class = view.ship_class(t.ships[ship].class).expect("a class");
        assert_eq!(class.cruise_speed_kn, Fixed::from_int(15));
        assert_eq!(class.max_speed_kn, Fixed::from_int(30));
        assert_eq!(class.fuel_capacity_t, Fixed::from_int(1000));
        assert_eq!(class.endurance_nm, Fixed::from_int(4000));
    }
}

// ---------------------------------------------------------- Statements: deny

/// Statement 1: 210 less 80 is 130 nm; the hexes within 3 of (0,0), 37 of
/// them, and no hex 4 away.
#[test]
fn statement_1_the_candidates() {
    assert_eq!(
        reach_for(Fixed::from_int(210), Some(2)),
        Fixed::from_int(130)
    );
    let played = play(&text(), 1);
    assert!(
        played
            .station_reason(1)
            .contains("The candidates, 37 of them,"),
        "{}",
        played.station_reason(1)
    );
    // The 37 by the engine's own test are exactly the hexes within 3.
    let within: Vec<Hex> = (-4..=4)
        .flat_map(|q| (-4..=4).map(move |r| hex(q, r)))
        .filter(|h| h.air_distance_within(hex(0, 0), Fixed::from_int(130)))
        .collect();
    assert_eq!(within.len(), 37);
    assert!(within.iter().all(|h| h.distance(hex(0, 0)) <= 3));
}

/// Statement 2.
#[test]
fn statement_2_with_no_enemy_known_the_nearest() {
    for level in RISKS {
        let played = play(&at_risk(&text(), level), 1);
        assert_eq!(
            played.formed(1),
            vec![(IntentId::new(1), c(), hex(3, 0))],
            "{level}"
        );
        let said = played.station_reason(1);
        assert!(
            said.contains(
                "The station is (3,0), 120 nm (3 hexes) east of the centre of the waters"
            ),
            "{level}: {said}"
        );
        assert!(
            said.contains("no circle of danger reaches it"),
            "{level}: {said}"
        );
        assert!(said.contains("7 hexes of passage"), "{level}: {said}");
    }
}

/// Statement 3.
#[test]
fn statement_3_the_speed_is_the_carrier_doctrines() {
    for level in ["balanced", "press"] {
        let played = play(&at_risk(&text(), level), 1);
        let orders = played.force_orders(1, c());
        assert!(
            orders.contains(&TaskForceOrder::SetSpeed(SpeedSetting::Full)),
            "{level}: {orders:?}"
        );
        assert_eq!(played.route_of(1), Some(vec![hex(3, 0)]), "{level}");
        assert!(played
            .said(1)
            .contains("cruise taking 18.666666 hours, more than the night's 12"));
    }
    let played = play(&at_risk(&text(), "preserve"), 1);
    let orders = played.force_orders(1, c());
    assert!(
        !orders.contains(&TaskForceOrder::SetSpeed(SpeedSetting::Full)),
        "{orders:?}"
    );
    assert_eq!(played.route_of(1), Some(vec![hex(3, 0)]));
    assert_eq!(
        played.truth[1].task_forces[&c()].speed,
        SpeedSetting::Cruise
    );
}

/// Statement 4.
#[test]
fn statement_4_already_in_reach_it_stays() {
    let played = play(&c_at(&text(), 2, 0), 1);
    assert_eq!(played.station(1), Some(hex(2, 0)));
    assert_eq!(played.route_of(1), None);
}

/// Statement 5, as it holds: C is sent to (5,0), 5 hexes of passage.
#[test]
fn statement_5_waters_wider_than_the_bombers_reach() {
    assert_eq!(
        reach_for(Fixed::from_int(210), Some(6)),
        Fixed::from_int(210)
    );
    let played = play(&waters(&text(), 6), 1);
    assert_eq!(played.station(1), Some(hex(5, 0)));
    assert!(played.station_reason(1).contains("5 hexes of passage"));
}

/// Statement 5's "the candidates are the hexes within 5 of (0,0)" does not
/// hold: the engine measures a strike's reach in a straight line (decision
/// 0040), and the six hexes 6 steps from (0,0) at the middle of each side of
/// that ring, such as (3,3), are 208 nm away, within 210.
/// What holds beside it: the 97 candidates, the 91 within 5 and six at 6.
#[test]
fn statement_5_as_it_holds_the_candidates_by_air_distance() {
    assert_eq!(hex(0, 0).air_distance_nm(hex(3, 3)), 208);
    let played = play(&waters(&text(), 6), 1);
    assert!(
        played
            .station_reason(1)
            .contains("The candidates, 97 of them,"),
        "{}",
        played.station_reason(1)
    );
}

/// Statement 6.
#[test]
fn statement_6_a_carrier_contact_casts_a_circle() {
    let t = contact(&text(), 8, -6, "carrier", 0);
    assert_eq!(hex(3, 0).distance(hex(8, -6)), 6);
    assert_eq!(hex(0, 3).distance(hex(8, -6)), 9);
    assert_eq!(hex(0, 3).distance(hex(10, 0)), 10);
    for level in ["preserve", "balanced"] {
        let played = play(&at_risk(&t, level), 1);
        assert_eq!(played.station(1), Some(hex(0, 3)), "{level}");
        let said = played.station_reason(1);
        assert!(said.contains("outside the circle of 8 hexes round contact 1, reported as a carrier, 0 pulse(s) old, at (8,-6), 9 hexes from it, by 1"), "{level}: {said}");
        assert!(said.contains("a nearer station, (3,0), 7 hexes of passage, lies inside the circle round contact 1"), "{level}: {said}");
    }
    let played = play(&at_risk(&t, "press"), 1);
    assert_eq!(played.station(1), Some(hex(3, 0)));
    let said = played.station_reason(1);
    assert!(
        said.contains("it stands in danger: it lies inside the circle of 8 hexes round contact 1"),
        "{said}"
    );
}

/// Statement 7.
#[test]
fn statement_7_what_casts_no_circle() {
    let variants = [
        contact(&text(), 8, -6, "cruiser", 0),
        contact(&text(), 8, -6, "carrier", 4),
        contact(&text(), 1, 0, "carrier", 0),
    ];
    for (i, t) in variants.iter().enumerate() {
        for level in RISKS {
            let played = play(&at_risk(t, level), 1);
            assert_eq!(played.station(1), Some(hex(3, 0)), "variant {i} at {level}");
        }
    }
}

/// Statement 8.
#[test]
fn statement_8_an_enemy_base_and_no_way_clear_of_it() {
    let t = base_e(&text(), -10, 0);
    let preserve = play(&at_risk(&t, "preserve"), 1);
    assert_eq!(preserve.station(1), None);
    let said = preserve.said(1);
    assert!(
        said.contains(
            "the circle of 15 hexes round base:2 (Placeholder base E) at (-10,0) hold every one"
        ),
        "{said}"
    );
    assert!(preserve.cannot_meet(1));
    let cannot = preserve.cannot_meet_reason(1);
    assert!(cannot.contains("base:1 (Placeholder base H)"), "{cannot}");
    assert!(
        cannot.contains("no carrier force can serve it: task_force:1"),
        "{cannot}"
    );

    let balanced = play(&at_risk(&t, "balanced"), 1);
    assert_eq!(balanced.station(1), Some(hex(3, 0)));
    let said = balanced.station_reason(1);
    assert!(said.contains("it stands in danger: it lies inside the circle of 15 hexes round base:2 (Placeholder base E) at (-10,0), 13 hexes from it, by 2"), "{said}");
    assert!(!balanced.cannot_meet(1));

    let press = play(&at_risk(&t, "press"), 1);
    assert_eq!(press.station(1), Some(hex(3, 0)));
}

/// The candidates of statement 8 at balanced, by the pure choice.
fn statement_8_candidates() -> Vec<Candidate> {
    let circles = [Circle {
        centre: hex(-10, 0),
        hexes: 15,
        words: String::new(),
    }];
    (-3..=3)
        .flat_map(|q| (-3..=3).map(move |r| hex(q, r)))
        .filter(|h| h.distance(hex(0, 0)) <= 3)
        .map(|h| Candidate {
            hex: h,
            passage: h.distance(hex(10, 0)),
            margin: margin_of(h, &circles),
        })
        .collect()
}

/// Statement 8's "of the four candidates equally deep" does not hold: seven
/// candidates are 13 hexes from E.
/// What holds beside it: seven equally deep, of which (3,0) has the least
/// passage.
#[test]
fn statement_8_as_it_holds_seven_equally_deep() {
    let all = statement_8_candidates();
    let deepest: Vec<Candidate> = all
        .iter()
        .copied()
        .filter(|c| c.margin == Some(-2))
        .collect();
    assert_eq!(deepest.len(), 7);
    assert!(all.iter().all(|c| c.margin.is_some_and(|m| m <= -2)));
    assert_eq!(order_by_risk(Risk::Balanced, &all)[0].hex, hex(3, 0));
}

/// Statement 9.
#[test]
fn statement_9_the_waters_and_the_targets_are_read() {
    let t = at(
        &text(),
        "commands",
        &command(
            1,
            "kind = \"set_limit\"\nlimit = \"targets\"\nintent = 1\nkinds = [\"transport\", \"carrier\", \"battleship\", \"cruiser\", \"destroyer\", \"auxiliary\"]\nwaters_first = false",
        ),
    );
    let played = play(&t, 2);
    // At its station on day 2, the waters in the arc.
    assert_eq!(played.position(1), hex(3, 0));
    let said = played.said(2);
    assert!(said.contains("the waters to deny at (0,0)"), "{said}");
    let list = played
        .group_orders(1, 1)
        .into_iter()
        .find_map(|o| match o {
            AirGroupOrder::SetTargetPriority { kinds } => Some(kinds),
            _ => None,
        })
        .expect("the bombers' list is set");
    assert_eq!(list.first(), Some(&ShipKind::Transport));
}

/// Statement 10: a Japanese transport in the waters at (1,0), held fresh;
/// C already at (2,0) in reach. A strike by its bombers is counted.
#[test]
fn statement_10_its_strikes_count() {
    let t = c_at(&text(), 2, 0);
    let t = at(
        &t,
        "forces",
        "[[task_force]]\nid = 2\nname = \"Placeholder transport force T\"\nside = \"japanese\"\nq = 1\nr = 0\nmission = \"transport\"\n\n[[ship]]\nid = 20\nname = \"Placeholder transport 20\"\nclass = 3\nside = \"japanese\"\nfuel_t = 1000\ntask_force = 2\n\n",
    );
    let t = contact(&t, 1, 0, "transport", 0);
    let played = play_with(&t, &figures(true), 1);
    let launched = played
        .scenario
        .world
        .events()
        .iter()
        .filter(|e| matches!(&e.kind, sim_core::events::EventKind::StrikeLaunched { air_group, .. } if *air_group == AirGroupId::new(1)))
        .count();
    assert!(launched > 0, "C's bombers launched");
    let measure = sim_agent::deny_measure(
        &played.scenario.world.side_view(Side::Allied),
        IntentId::new(1),
    )
    .expect("the deny's measure");
    assert_eq!(measure.strikes_launched as usize, launched);
}

// ----------------------------------------------------- Statements: neutralise

/// The candidates of the neutralise, by the engine's own test.
fn neutralise_candidates() -> Vec<Hex> {
    (-7..=7)
        .flat_map(|q| (-7..=7).map(move |r| hex(q, r)))
        .filter(|h| h.air_distance_within(hex(0, 0), Fixed::from_int(210)))
        .collect()
}

/// Statement 11, as it holds: 97 candidates, every one inside E's circle.
#[test]
fn statement_11_as_it_holds_every_candidate_is_inside_es_circle() {
    let all = neutralise_candidates();
    assert_eq!(all.len(), 97);
    assert!(all.iter().all(|h| h.distance(hex(0, 0)) <= 15));
}

/// Statement 12.
#[test]
fn statement_12_at_preserve_the_carriers_do_not_go() {
    let played = play(&at_risk(&neutralise_only(), "preserve"), 1);
    assert_eq!(played.station(1), None);
    let said = played.said(1);
    assert!(
        said.contains(
            "a raid on base:2 (Placeholder base E) at (0,0) is flown from inside its own circle"
        ),
        "{said}"
    );
}

/// Statement 13 at balanced does not hold: the least deep candidates are
/// the six of the ring of 6 within 210 nm, inside by 9, and of those (3,3)
/// and (6,-3) have the least passage, 7; (3,3) comes first in hex order.
/// Statement 13 as it holds: at balanced (3,3), inside by 9, in danger of E;
/// at press (5,0).
#[test]
fn statement_13_as_it_holds() {
    let balanced = play(&at_risk(&neutralise_only(), "balanced"), 1);
    assert_eq!(balanced.station(1), Some(hex(3, 3)));
    let said = balanced.station_reason(1);
    assert!(said.contains("it stands in danger: it lies inside the circle of 15 hexes round base:2 (Placeholder base E) at (0,0), 6 hexes from it, by 9"), "{said}");
    let press = play(&at_risk(&neutralise_only(), "press"), 1);
    assert_eq!(press.station(1), Some(hex(5, 0)));
}

/// Statement 14, at press with C at (5,0): no contact, the bombers strike E;
/// a fresh contact of ships 4 hexes from C, they strike ships.
#[test]
fn statement_14_the_bombers_raid_the_base() {
    let t = c_at(&at_risk(&neutralise_only(), "press"), 5, 0);
    let played = play(&t, 1);
    assert_eq!(played.station(1), Some(hex(5, 0)));
    assert!(played
        .group_orders(1, 1)
        .contains(&AirGroupOrder::SetAirMission(AirMission::BaseStrike(
            BaseId::new(2)
        ))));

    let with_ships = contact(&t, 9, 0, "cruiser", 0);
    let played = play(&with_ships, 1);
    let orders = played.group_orders(1, 1);
    assert!(
        !orders
            .iter()
            .any(|o| matches!(o, AirGroupOrder::SetAirMission(AirMission::BaseStrike(_)))),
        "{orders:?}"
    );
    assert!(
        orders.contains(&AirGroupOrder::SetAirMission(AirMission::NavalStrike)),
        "{orders:?}"
    );
}

/// Statement 15.
#[test]
fn statement_15_the_pair_is_allowed() {
    let task = IntentKind::OperateCarriers {
        force: c(),
        hex: hex(5, 0),
    };
    assert!(task.may_stand_under(IntentKind::Neutralise {
        target: BaseId::new(2)
    }));
    let played = play(&at_risk(&neutralise_only(), "press"), 1);
    assert_eq!(played.formed(1), vec![(IntentId::new(1), c(), hex(5, 0))]);
    // Taken in by the engine: the task is in force under the neutralise.
    let state = played
        .scenario
        .world
        .side_view(Side::Allied)
        .command_state()
        .clone();
    assert!(state
        .intents_in_force()
        .any(|i| i.parent == Some(IntentId::new(1))
            && matches!(i.kind, IntentKind::OperateCarriers { .. })));
}

// ----------------------------------------------------------- Statements: fuel

/// Statement 16 as written: the passage home is 360 nm, 90 t, and 370 t in
/// all. The engine charges a step of a clear way as the straight line
/// (decision 0058): from (3,0) to (10,2) is 327 nm, 81.75 t, and the need at
/// press is 361.75 t.
/// Statement 16 as it holds: at press with 400 t aboard each, C is sent, and
/// the reason gives 280 t out, 81.75 t home and 361.75 t against the least
/// aboard.
#[test]
fn statement_16_as_it_holds() {
    let played = play(&all_fuel(&at_risk(&text(), "press"), 400), 1);
    assert_eq!(played.station(1), Some(hex(3, 0)));
    let said = played.station_reason(1);
    assert!(said.contains("280 nm at full speed"), "{said}");
    assert!(said.contains("327 nm at cruise"), "{said}");
    assert!(
        said.contains("361.75 t (280 t out and 81.75 t home, times 1)"),
        "{said}"
    );
    assert!(
        said.contains("the least aboard is ship:1's 400 t"),
        "{said}"
    );
}

/// Statement 17 as written: 444 t and 492 t.
/// Statement 17 as it holds: with a destroyer at 400 t no candidate is in
/// fuel at balanced and the reason names it; at 450 t C is sent to (3,0).
#[test]
fn statement_17_as_it_holds() {
    let played = play(&fuel(&text(), 2, 400), 1);
    assert_eq!(played.station(1), None);
    let said = played.said(1);
    assert!(said.contains("is in fuel at the balanced risk"), "{said}");
    assert!(
        said.contains("ship:2, 400 t aboard against 434.1 t needed"),
        "{said}"
    );
    let played = play(&fuel(&text(), 2, 450), 1);
    assert_eq!(played.station(1), Some(hex(3, 0)));
}

/// Statement 18: at preserve the passage out is at cruise; 250 t each is
/// enough (the need is (70 + 81.75) x 1.5 = 227.625 t by the engine's
/// charge of the way home).
#[test]
fn statement_18_preserve_sails_at_cruise() {
    let played = play(&all_fuel(&at_risk(&text(), "preserve"), 250), 1);
    assert_eq!(played.station(1), Some(hex(3, 0)));
    let said = played.station_reason(1);
    assert!(said.contains("280 nm at cruise"), "{said}");
    assert!(said.contains("70 t out"), "{said}");
}

/// Statement 19.
#[test]
fn statement_19_in_port_it_is_weighed_full() {
    let t = all_fuel(&c_at_h(&text()), 100);
    let played = play(&t, 1);
    assert!(played.station(1).is_some(), "{}", played.said(1));
    let said = played.station_reason(1);
    assert!(said.contains("its ships are weighed full"), "{said}");
    // The carrier doctrine fuels it before it sails.
    assert!(played.said(1).contains("fuels there"), "{}", played.said(1));
    assert_eq!(played.route_of(1), None);
}

/// Statement 20.
#[test]
fn statement_20_no_home_port() {
    for level in RISKS {
        let played = play(&no_home(&at_risk(&text(), level)), 1);
        assert_eq!(played.station(1), Some(hex(3, 0)), "{level}");
        assert!(played
            .station_reason(1)
            .contains("its fuel could not be measured against a passage home"));
    }
}

// ------------------------------------- Statements: which force, the commander

/// Statement 21.
#[test]
fn statement_21_not_free() {
    let own = at(
        &text(),
        "commands",
        &command(
            1,
            "kind = \"set_intent\"\nintent = 2\nintent_kind = \"operate_carriers\"\ntask_force = 1\nhex = [10, 0]",
        ),
    );
    let named = at(
        &at(
            &text(),
            "commands",
            &command(
                1,
                "kind = \"set_intent\"\nintent = 2\nintent_kind = \"deny\"\nhex = [10, 0]\ndistance = 0",
            ),
        ),
        "commands",
        &command(
            1,
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\ntask_forces = [1]",
        ),
    );
    let held = hold_c(&text(), 1);
    for (i, t) in [own, named, held].iter().enumerate() {
        let played = play(t, 1);
        assert!(
            !played
                .formed(1)
                .iter()
                .any(|(p, _, _)| *p == IntentId::new(1)),
            "variant {i}"
        );
        let state = played
            .scenario
            .world
            .side_view(Side::Allied)
            .command_state()
            .clone();
        let under_deny: Vec<_> = state.descendants_in_force(IntentId::new(1));
        assert!(
            played
                .decisions(1)
                .orders
                .iter()
                .all(|o| !under_deny.contains(&o.intent) && o.intent != IntentId::new(1)),
            "variant {i}"
        );
    }
}

/// Statement 22. The hold is lifted on day 3, as the commander does at the
/// end of that day's play: a release for day 4 in the file (a command of the
/// file's day N is taken before the staff's morning of day N).
#[test]
fn statement_22_a_hold_lifted() {
    let t = release_c(&hold_c(&text(), 1), 4);
    let played = play(&t, 4);
    for day in 1..=3 {
        assert_eq!(played.station(day), None, "day {day}");
    }
    assert_eq!(played.station(4), Some(hex(3, 0)));
    assert!(played.station_reason(4).contains("from (10,0)"));
}

/// Statement 23.
#[test]
fn statement_23_a_force_the_commander_named_keeps_its_hex() {
    let t = at(
        &text(),
        "commands",
        &command(
            1,
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\ntask_forces = [1]",
        ),
    );
    let played = play(&t, 1);
    assert_eq!(played.station(1), Some(hex(10, 0)));
    assert_eq!(played.route_of(1), None);
    assert!(!played.station_reason(1).contains("The station is"));
}

/// The deny of statement 2 and the neutralise of statement 11, at balanced.
fn deny_and_neutralise() -> String {
    let t = base_e(&text(), 0, 0);
    at(
        &t,
        "commands",
        &command(
            1,
            "kind = \"set_intent\"\nintent = 2\nintent_kind = \"neutralise\"\ntarget = 2",
        ),
    )
}

/// Statement 24.
#[test]
fn statement_24_one_force_two_orders() {
    let t = at(
        &deny_and_neutralise(),
        "commands",
        &command(
            1,
            "kind = \"set_limit\"\nlimit = \"priority\"\nintent = 2\nrank = 1",
        ),
    );
    let played = play(&t, 1);
    let formed = played.formed(1);
    assert_eq!(formed.len(), 1, "{formed:?}");
    assert_eq!(formed[0].0, IntentId::new(2));
    assert!(played
        .said(1)
        .contains("the one carrier force task_force:1 is taken by intent:2"));

    let played = play(&deny_and_neutralise(), 1);
    let formed = played.formed(1);
    assert_eq!(formed.len(), 1, "{formed:?}");
    assert_eq!(formed[0].0, IntentId::new(1));
}

/// Statement 25.
#[test]
fn statement_25_leave_withheld() {
    let withheld = common::replace_once(
        &text(),
        "leave = \"launch\"\ngiven = true",
        "leave = \"launch\"\ngiven = false",
    );
    // The preview of the order names C and (3,0).
    let mut scenario = load(&withheld);
    let held = staffs();
    let mut proposal = None;
    scenario
        .next_day_watched(as_staffs(&held), &mut |world, moment| {
            if moment == (Moment::Orders { day: 1 }) {
                proposal = SupplyStaff.propose(world.side_view(Side::Allied), IntentId::new(1));
            }
        })
        .expect("day 1");
    let proposal = proposal.expect("a proposal");
    assert!(
        proposal
            .reason
            .worked
            .contains("to operate the carrier force task_force:1 from (3,0)"),
        "{}",
        proposal.reason.worked
    );
    let played = play(&withheld, 2);
    assert_eq!(played.station(1), None);
    assert!(played.force_orders(1, c()).is_empty());
    // Approved on day 2: formed and routed that morning.
    let approved = at(
        &withheld,
        "commands",
        &command(2, "kind = \"approve\"\nplan = 1"),
    );
    let played = play(&approved, 2);
    assert_eq!(played.station(1), None);
    assert_eq!(played.station(2), Some(hex(3, 0)));
    assert_eq!(played.route_of(2), Some(vec![hex(3, 0)]));
}

/// Statement 26: the task formed and C under way on day 1 (with the way
/// slowed to cruise at preserve, so it is still under way on day 2), the
/// commander routes C himself on day 2: no route from the staff the next
/// morning.
#[test]
fn statement_26_a_direct_order_to_a_tasked_force_holds_it() {
    let t = at(
        &at_risk(&text(), "preserve"),
        "commands",
        "[[order]]\nday = 2\nside = \"allied\"\ntask_force = 1\nkind = \"set_route\"\nwaypoints = [[10, 0]]\n\n",
    );
    let played = play(&t, 3);
    assert_eq!(played.station(1), Some(hex(3, 0)));
    let state = played
        .scenario
        .world
        .side_view(Side::Allied)
        .command_state()
        .clone();
    assert!(state
        .holds
        .contains_key(&sim_core::command::HoldTarget::TaskForce(c())));
    assert_eq!(played.route_of(3), None);
}

// ----------------------------------------------- Statements: the search on the way

/// Statement 27.
#[test]
fn statement_27_the_way_ahead_is_searched() {
    let played = play(&text(), 2);
    let said = played.said(1);
    for q in 3..=9 {
        assert!(
            said.contains(&format!("the way ahead to the station (3,0) at ({q},0)")),
            "{q}: {said}"
        );
    }
    assert!(said.contains("the waters to deny at (0,0)"));
    assert_eq!(played.position(1), hex(3, 0));
    let said = played.said(2);
    assert!(!said.contains("the way ahead"), "{said}");
    assert!(said.contains("the waters to deny at (0,0)"));
}

// ---------------------------------------------------------------- Properties

/// The choice of a station draws no random number: the same view gives the
/// same station, whatever the seed.
#[test]
fn property_nothing_is_drawn() {
    let base = contact(&text(), 8, -6, "carrier", 0);
    let a = play(&base, 1);
    let b = play(&common::replace_once(&base, "seed = 1942", "seed = 7"), 1);
    assert_eq!(a.formed(1), b.formed(1));
    assert_eq!(a.station_reason(1), b.station_reason(1));
}

/// A reason for a station names no holding of an enemy base: no aircraft,
/// supply or fuel of E.
#[test]
fn property_the_staff_reads_its_own_side() {
    let t = base_e(&text(), -10, 0);
    for level in RISKS {
        let played = play(&at_risk(&t, level), 1);
        let said = played.said(1);
        for line in said.lines().filter(|l| l.contains("base:2")) {
            assert!(!line.contains("supply"), "{line}");
            assert!(!line.contains("aircraft"), "{line}");
            assert!(!line.contains("fuel_t"), "{line}");
        }
    }
}
