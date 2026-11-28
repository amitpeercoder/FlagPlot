//! Scenario 129: submarines, the patrol order and the staff. Where a
//! side's boats are put by an order, what holds a boat from the staff,
//! and how the staff keeps a standing force out of submarine waters.
//!
//! Proves the third stage of decision 0218: rules 11 and 12, 34, 54 and
//! 55 of the section "Amended on a second review", and the rulings on
//! rows 61, 62 and 68 of the differences. The data is placeholder data.
//! It stands on scenario 128 for everything a boat does once she has a
//! station.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything stated is
//! asserted exactly. If the code disagrees, the code or the rule is
//! wrong: do not change a statement here to make a test pass. A mismatch
//! goes back to the architect.
//!
//! ## The setting
//!
//! An open sea of hexes q from 0 to 12 and r from -6 to 6, all navigable
//! but where a game says. The Japanese side has a base at (12,0) and the
//! placeholder boats of scenario 128, numbered from 1, all at (12,0) at
//! the start, sound, with both salvoes. The Japanese side has a commander
//! and a staff.
//!
//! Distance is the count of hexes. Hexes are ordered by q, then by r: the
//! lowest hex is the one that comes first so.
//!
//! ## Part A: the stations of a patrol order
//!
//! The order: patrol the waters within 2 hexes of (5,0) with submarines.
//! The waters are 19 hexes. The stations are given so: the first is the
//! centre; each next is the navigable hex of the waters farthest from the
//! nearest station already given, the lowest hex where they tie.
//!
//! Worked, each station with its distance from the nearest before it:
//!
//! | Station | Hex | Distance |
//! | --- | --- | --- |
//! | 1 | (5,0) | the centre |
//! | 2 | (3,0) | 2 |
//! | 3 | (3,2) | 2 |
//! | 4 | (5,-2) | 2 |
//! | 5 | (5,2) | 2 |
//! | 6 | (7,-2) | 2 |
//! | 7 | (7,0) | 2 |
//!
//! After the seventh every hex of the waters left is 1 from a station.
//! Stations go to boats in the boats' number order.
//!
//! 1. Game A1, seven boats and the order with no boat named: at the first
//!    dusk the staff gives boat n the station of row n. Each order is a
//!    station order of the engine, caused by the patrol order, and the
//!    staff's reason for each says the boat and her station.
//! 2. Game A2, three boats: stations 1 to 3 of the table.
//! 3. Game A3, twenty boats: the nineteenth station is the last hex of
//!    the waters, and boat 20 shares station 1, (5,0).
//! 4. Game A4, as A1 with (5,0) land: the first station is the navigable
//!    hex of the waters nearest the centre, the lowest where they tie:
//!    (4,0). No station is on land.
//! 5. Game A5, as A1 with the order naming boats 2, 5 and 6: those three
//!    take stations 1 to 3 in their number order and no other boat is
//!    ordered.
//! 6. A boat already on the station the order would give her is given no
//!    order again: on the second dusk of A1 the staff hands in none for
//!    the boats, and its reason says each keeps her station.
//! 7. A boat that is damaged, going home, home or sunk is passed over and
//!    her station goes to no one: in A1 with boat 3 damaged at the start,
//!    boats 1, 2, 4, 5, 6 and 7 take stations 1, 2, 4, 5, 6 and 7 as
//!    before. Stations do not move up.
//! 8. The preview of the order says each boat's station before it is
//!    given, and gives the same stations.
//!
//! ## Part B: whose hands a boat is in
//!
//! 9. A direct order to a boat to take a station holds her from the
//!    staff, as a direct order holds a force. Game B1, A1 with the
//!    commander ordering boat 2 to (9,3) at the first dusk: boat 2 goes to
//!    (9,3); the staff orders boats 1, 3, 4, 5, 6 and 7 to stations 1, 3,
//!    4, 5, 6 and 7 and says in its reason that boat 2 is the commander's.
//! 10. A hold may name a boat. Held with no order she keeps what she was
//!    doing and the staff gives her none. Released, she is the staff's at
//!    the next dusk and takes the station her place in the order gives.
//! 11. An order's forces limit may name boats: only they serve it (A5).
//!    A boat named by the forces limit of another order is not used by the
//!    patrol order.
//! 12. A side with no commander is given its patrol order by its scenario
//!    and its staff serves it the same way.
//!
//! ## Part C: the staff and submarine waters
//!
//! The Allied side has a commander and a staff, a carrier force free to
//! be stationed, and an order to deny the waters within 1 hex of (6,0).
//! With nothing else to weigh, the station the staff's rule chooses for
//! the carrier force is hex S; the test finds S by running game C1 and
//! reads the staff's reason for it. The statements are about what changes.
//!
//! 13. Game C1, no submarine belief: the carrier force is stationed at S,
//!    and no reason speaks of submarines.
//! 14. Game C2, C1 with suspected waters told on day 1 whose circle holds
//!    S and not every hex the rule could choose: the staff stations the
//!    force at the hex its rule ranks next that is outside the waters,
//!    and its reason says S was passed over for submarine waters.
//! 15. Game C3, C1 with a report of a submarine held at a neighbour of S:
//!    the same, S being within one hex of the report.
//! 16. Game C4, C1 with suspected waters that hold every hex the rule
//!    could choose: the staff stations the force at S and its reason says
//!    no place outside submarine waters serves.
//! 17. In C2 and C3 the force's route is as the route rule gives it; a
//!    route is not bent for submarine waters.
//! 18. The same regard is had where the staff chooses the hex at which a
//!    force waits (the covering hex of a take), proved on one game as
//!    statements 14 and 16 are. Where a force stands by night the hex is the
//!    order's own and the staff chooses none.
//! 19. A side's staff reads only what the side believes: with an enemy
//!    boat lying at S that the side holds no report of and no suspected
//!    waters, the staff stations the force at S (C1 with the boat added).
//!
//! ## Part D: what a scenario may not give
//!
//! 20. A scenario that gives a boat no salvo is refused when it is read,
//!    with her number in the reason.
//! 21. A scenario whose boat is to go home to a base that no sea route
//!    reaches from her station is refused when it is read.
//!
//! ## Part E: nothing else moves
//!
//! 22. A game with no boat, no report of a submarine and no suspected
//!    waters gives the orders, the reasons and the hash it gave: the
//!    staffed scenarios on file are compared before and after.
// The architect's doc comment above carries lines clippy reads as a lazy
// continuation of a list; the comment is never edited, so the lint is allowed.
#![allow(clippy::doc_lazy_continuation)]

// Part C (statements 13 to 19) and Part E (statement 22) are built by
// another developer. The tests below prove statements 1 to 12, 20 and 21.
// The stations beyond the architect's table (A3's twenty boats, A4's seven)
// are worked by hand from the same rule, written out where they are used.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use sim_core::command::{
    Addressee, CautionKind, CommandKind, CommandState, IntentKind, PatrolStation, Proposal, Reason,
    Risk, Staff, StaffDecisions,
};
use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{IntentId, SubmarineId, TaskForceId};
use sim_core::orders::{Issuer, OrderKind, SubmarineOrder, TaskForceOrder};
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;
use sim_staff::station::{margin_of, order_by_risk, reach_for, Candidate};
use sim_staff::{as_staffs, staffs, SupplyStaff};

use crate::common;
use crate::common::values::hex;

const COMMANDS_MARKER: &str = "# The Japanese commander's commands";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/129_submarines_the_staff_and_the_patrol.toml")
}

fn file_text() -> String {
    fs::read_to_string(path()).expect("the scenario file reads")
}

fn boat(raw: u32) -> SubmarineId {
    SubmarineId::new(raw)
}

/// The stations of the table of Part A: station n is the n-th.
const TABLE: [(i32, i32); 7] = [(5, 0), (3, 0), (3, 2), (5, -2), (5, 2), (7, -2), (7, 0)];

// ---------------------------------------------------------------------
// The games. Each is the scenario's head (the map, the figures, the base)
// with the boats and the commander's commands of the game after it.
// ---------------------------------------------------------------------

#[derive(Clone)]
struct Game {
    boats: u32,
    /// Lines added to a boat's entry, by boat number.
    boat_lines: BTreeMap<u32, String>,
    /// The commander's commands, replacing the file's.
    commands: String,
    /// Whole tables added after the commands.
    extra: String,
    days: u32,
    /// The map file, if not the scenario's own.
    map: Option<String>,
    side: &'static str,
}

fn intent_commands(intent: u32, hex: (i32, i32), distance: u32, side: &str) -> String {
    format!(
        "[[command]]\nday = 1\nside = \"{side}\"\nkind = \"set_intent\"\nintent = {intent}\nintent_kind = \"patrol\"\nhex = [{}, {}]\ndistance = {distance}\n\n",
        hex.0, hex.1
    )
}

fn leave_commands(side: &str) -> String {
    format!(
        "[[command]]\nday = 1\nside = \"{side}\"\nkind = \"set_leave\"\nleave = \"prepare\"\ngiven = true\n\n\
         [[command]]\nday = 1\nside = \"{side}\"\nkind = \"set_leave\"\nleave = \"launch\"\ngiven = true\n"
    )
}

fn game(boats: u32) -> Game {
    Game {
        boats,
        boat_lines: BTreeMap::new(),
        commands: format!(
            "{}{}",
            intent_commands(1, (5, 0), 2, "japanese"),
            leave_commands("japanese")
        ),
        extra: String::new(),
        days: 3,
        map: None,
        side: "japanese",
    }
}

impl Game {
    fn boat_line(mut self, boat: u32, lines: &str) -> Game {
        self.boat_lines.insert(boat, format!("{lines}\n"));
        self
    }

    fn with_extra(mut self, text: &str) -> Game {
        self.extra.push_str(text);
        self
    }

    fn text(&self) -> String {
        let file = file_text();
        let at = file.find("\n[[submarine]]").expect("the boats begin");
        let mut text = file[..at].to_string();
        assert!(file.contains(COMMANDS_MARKER));
        text = common::replace_once(&text, "days = 3", &format!("days = {}", self.days));
        if let Some(map) = &self.map {
            text = common::replace_once(
                &text,
                "map = \"../map/placeholder_patrol.toml\"",
                &format!("map = \"{map}\""),
            );
        }
        for id in 1..=self.boats {
            text.push_str(&format!(
                "\n[[submarine]]\nid = {id}\nname = \"Placeholder submarine {id}\"\nside = \"japanese\"\nclass = 6900\nq = 12\nr = 0\nhome = 1\n"
            ));
            if let Some(lines) = self.boat_lines.get(&id) {
                text.push_str(lines);
            }
        }
        text.push_str(&format!("\n{COMMANDS_MARKER}\n"));
        text.push_str(&self.commands);
        text.push_str(&self.extra);
        if self.side == "allied" {
            // The side with its patrol order given by its scenario and no
            // other command: every table of the Japanese side is the
            // Allied side's.
            text = text
                .replace("japanese", "allied")
                .replace("Japanese", "Allied");
        }
        text
    }

    fn scenario(&self) -> Scenario {
        common::from_text(&self.text(), &path()).expect("the game loads")
    }

    fn play(&self) -> Played {
        play(self.scenario(), self.side_enum())
    }

    fn side_enum(&self) -> Side {
        if self.side == "allied" {
            Side::Allied
        } else {
            Side::Japanese
        }
    }
}

/// A game played to its end with the doctrine's staffs: what the side's
/// staff handed in each day, the truth at the end of each day (index 0 is
/// the start), the log and the world.
struct Played {
    side: Side,
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    scenario: Scenario,
}

fn play(mut scenario: Scenario, side: Side) -> Played {
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
                .get(&side)
                .cloned()
                .unwrap_or_default(),
        );
        truth.push(scenario.world.umpire().clone());
    }
    let events = scenario.world.events().iter().cloned().collect();
    Played {
        side,
        handed,
        truth,
        events,
        scenario,
    }
}

impl Played {
    /// The station orders the staff handed in on `day`, as (boat, station)
    /// in the order handed in.
    fn stations(&self, day: u32) -> Vec<(u32, Hex)> {
        self.handed[&day]
            .orders
            .iter()
            .map(|order| match &order.kind {
                OrderKind::Submarine {
                    target,
                    order: SubmarineOrder::Station(at),
                } => (target.raw(), *at),
                other => panic!("the patrol doctrine gave another order: {other:?}"),
            })
            .collect()
    }

    fn command_state(&self) -> &CommandState {
        self.scenario.world.command_state(self.side)
    }

    fn nothing_refused(&self) {
        let refused: Vec<_> = self
            .events
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
                )
            })
            .collect();
        assert!(refused.is_empty(), "something was refused: {refused:?}");
    }

    /// Every station order of the staff, accepted, as the engine takes it:
    /// from the staff, serving intent `intent`, and recorded as the boat's
    /// station by `SubmarineOrderedToStation`, which cites it.
    fn assert_station_orders_taken(&self, intent: u32, expected: &[(u32, Hex)]) {
        let mut taken: Vec<(u32, Hex)> = Vec::new();
        for event in &self.events {
            let EventKind::OrderAccepted { order, target } = &event.kind else {
                continue;
            };
            let record = self.scenario.world.order(*order).expect("the order");
            let OrderKind::Submarine {
                order: SubmarineOrder::Station(at),
                ..
            } = &record.kind
            else {
                continue;
            };
            if record.origin.issuer != Issuer::Staff {
                continue;
            }
            assert_eq!(record.origin.intent, Some(IntentId::new(intent)));
            let sim_core::ids::EntityRef::Submarine(id) = target else {
                panic!("a station order to {target:?}");
            };
            let told = self
                .events
                .iter()
                .filter(|e| {
                    matches!(
                        &e.kind,
                        EventKind::SubmarineOrderedToStation { submarine, station }
                            if submarine == id && station == at
                    ) && e.causes.contains(&Cause::Order(*order))
                })
                .count();
            assert_eq!(told, 1, "{id}'s station is recorded once, citing the order");
            taken.push((id.raw(), *at));
        }
        assert_eq!(taken, expected);
    }

    /// The reason of the staff's order to boat `raw` on `day`.
    fn reason_of_order(&self, day: u32, raw: u32) -> &sim_core::command::Reason {
        self.handed[&day]
            .orders
            .iter()
            .find(|o| matches!(&o.kind, OrderKind::Submarine { target, .. } if target.raw() == raw))
            .map(|o| &o.reason)
            .expect("an order for the boat")
    }

    /// The reasons for giving no order on `day`.
    fn no_order_text(&self, day: u32) -> Vec<String> {
        self.handed[&day]
            .no_orders
            .iter()
            .map(|r| r.worked.clone())
            .collect()
    }

    /// Where boat `raw` is at the end of `day`, and her station.
    fn boat_at_end_of(&self, day: u32, raw: u32) -> (Hex, Hex) {
        let boat = &self.truth[day as usize].submarines[&boat(raw)];
        (boat.hex, boat.station)
    }
}

fn table(n: usize) -> Vec<(u32, Hex)> {
    (1..=n)
        .map(|i| {
            (
                u32::try_from(i).expect("a boat"),
                hex(TABLE[i - 1].0, TABLE[i - 1].1),
            )
        })
        .collect()
}

fn of(rows: &[(u32, (i32, i32))]) -> Vec<(u32, Hex)> {
    rows.iter().map(|(b, (q, r))| (*b, hex(*q, *r))).collect()
}

/// A reason of a station order says the boat by name and number, her place,
/// her station as "(q,r)", and is decided at the level of the area.
fn assert_order_reason(played: &Played, day: u32, raw: u32, place: u32, at: Hex) {
    let reason = played.reason_of_order(day, raw);
    let words = &reason.worked;
    assert!(
        words.contains(&format!("submarine:{raw} (Placeholder submarine {raw})")),
        "{words}"
    );
    assert!(words.contains(&format!("in place {place} ")), "{words}");
    assert!(words.contains(&format!("her station is {at}")), "{words}");
    assert_eq!(reason.serves.intent, Some(IntentId::new(1)));
    assert_eq!(reason.serves.level, Some(Addressee::Area));
}

// ---------------------------------------------------------------------
// Part A: the stations of a patrol order.
// ---------------------------------------------------------------------

#[test]
fn statement_01_game_a1_boat_n_is_given_the_station_of_row_n() {
    let played = game(7).play();
    played.nothing_refused();
    assert_eq!(played.stations(1), table(7));
    played.assert_station_orders_taken(1, &table(7));
    for (n, (boat, at)) in table(7).iter().enumerate() {
        assert_order_reason(&played, 1, *boat, u32::try_from(n + 1).unwrap(), *at);
    }
    // The distances of the table are in the reasons: the centre, then each
    // station 2 hexes from the nearest before it.
    assert!(played
        .reason_of_order(1, 1)
        .worked
        .contains("the centre of the waters"));
    for n in 2..=7 {
        assert!(
            played.reason_of_order(1, n).worked.contains(&format!(
                "station {n} of the waters, 2 hexes from the nearest before it"
            )),
            "{}",
            played.reason_of_order(1, n).worked
        );
    }
    // Every boat is at her station by the end, none sunk or going home.
    for (boat, at) in table(7) {
        assert_eq!(
            played.truth[1].submarines[&SubmarineId::new(boat)].station,
            at
        );
    }
    // The staff gave the orders once: nothing is handed in again on day 3.
    assert!(played.stations(3).is_empty());
}

#[test]
fn statement_02_game_a2_three_boats_take_stations_1_to_3() {
    let played = game(3).play();
    played.nothing_refused();
    assert_eq!(played.stations(1), table(3));
    played.assert_station_orders_taken(1, &table(3));
}

#[test]
fn statement_03_game_a3_twenty_boats_the_nineteenth_station_is_the_last_hex_and_boat_20_shares_the_first(
) {
    // Worked by hand from the rule of the table: after the seven stations
    // of the table every hex of the waters left is 1 from a station, so the
    // next is the lowest hex of those that tie at 1 (the lowest hex left),
    // then the lowest of those left, and so on; the waters are 19 hexes, so
    // boat 20 begins again at station 1.
    let rows = [
        (1, (5, 0)),
        (2, (3, 0)),
        (3, (3, 2)),
        (4, (5, -2)),
        (5, (5, 2)),
        (6, (7, -2)),
        (7, (7, 0)),
        (8, (3, 1)),
        (9, (4, -1)),
        (10, (4, 0)),
        (11, (4, 1)),
        (12, (4, 2)),
        (13, (5, -1)),
        (14, (5, 1)),
        (15, (6, -2)),
        (16, (6, -1)),
        (17, (6, 0)),
        (18, (6, 1)),
        (19, (7, -1)),
        (20, (5, 0)),
    ];
    let played = game(20).play();
    played.nothing_refused();
    assert_eq!(played.stations(1), of(&rows));
    played.assert_station_orders_taken(1, &of(&rows));
    // The nineteen stations are the nineteen hexes of the waters, each once.
    let mut first_nineteen: Vec<Hex> = played.stations(1).iter().take(19).map(|s| s.1).collect();
    first_nineteen.sort();
    first_nineteen.dedup();
    assert_eq!(first_nineteen.len(), 19);
    assert_eq!(*first_nineteen.last().unwrap(), hex(7, 0));
    assert_eq!(played.stations(1)[19].1, hex(5, 0));
}

/// A copy of the scenario's map with `extra` appended, written once to the
/// build directory; the scenario's own map file is not touched.
fn map_with(name: &str, extra: &str) -> String {
    let original = fs::read_to_string(common::data_dir().join("map/placeholder_patrol.toml"))
        .expect("read the map");
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    fs::write(&path, format!("{original}\n{extra}")).expect("write the map");
    path.to_string_lossy().into_owned()
}

#[test]
fn statement_04_game_a4_the_centre_land_the_first_station_is_4_0_and_none_is_on_land() {
    let map = map_with(
        "scenario_129_land_at_the_centre.toml",
        "[[hex]]\nq = 5\nr = 0\nkind = \"land\"\n",
    );
    let mut a4 = game(7);
    a4.map = Some(map);
    let played = a4.play();
    played.nothing_refused();
    let given = played.stations(1);
    assert_eq!(given[0], (1, hex(4, 0)));
    // The rest by the rule, worked by hand with (5,0) left out: each next is
    // the hex of the 18 farthest from the nearest given, the lowest where
    // they tie.
    assert_eq!(
        given,
        of(&[
            (1, (4, 0)),
            (2, (5, 2)),
            (3, (7, -2)),
            (4, (3, 2)),
            (5, (5, -2)),
            (6, (6, 0)),
            (7, (3, 0)),
        ])
    );
    for (_, at) in &given {
        assert_ne!(*at, hex(5, 0), "no station is on land");
    }
    played.assert_station_orders_taken(1, &given);
}

#[test]
fn statement_05_game_a5_boats_2_5_and_6_take_stations_1_to_3_and_no_other_boat_is_ordered() {
    let a5 = game(7).with_extra(
        "\n[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nsubmarines = [2, 5, 6]\n",
    );
    let played = a5.play();
    played.nothing_refused();
    let expected = of(&[(2, (5, 0)), (5, (3, 0)), (6, (3, 2))]);
    assert_eq!(played.stations(1), expected);
    played.assert_station_orders_taken(1, &expected);
    // The other boats stay where they were, at the base.
    for raw in [1, 3, 4, 7] {
        let (at, station) = played.boat_at_end_of(1, raw);
        assert_eq!((at, station), (hex(12, 0), hex(12, 0)), "boat {raw}");
    }
}

#[test]
fn statement_06_a_boat_on_her_station_is_given_no_order_again() {
    let played = game(7).play();
    assert_eq!(played.stations(1), table(7));
    // The second dusk: the staff hands in no order for a boat.
    assert!(played.stations(2).is_empty());
    let words = played.no_order_text(2);
    assert_eq!(words.len(), 1, "one decision to give no order: {words:?}");
    for (boat, at) in table(7) {
        assert!(
            words[0].contains(&format!(
                "submarine:{boat} (Placeholder submarine {boat}) keeps her station at {at}"
            )),
            "{}",
            words[0]
        );
    }
    assert!(words[0].contains("goes to no one"), "{}", words[0]);
    // Nor on the third.
    assert!(played.stations(3).is_empty());
}

#[test]
fn statement_07_a_damaged_boat_is_passed_over_and_her_station_goes_to_no_one() {
    let played = game(7).boat_line(3, "condition = \"damaged\"").play();
    played.nothing_refused();
    let expected = of(&[
        (1, (5, 0)),
        (2, (3, 0)),
        (4, (5, -2)),
        (5, (5, 2)),
        (6, (7, -2)),
        (7, (7, 0)),
    ]);
    assert_eq!(played.stations(1), expected);
    played.assert_station_orders_taken(1, &expected);
    // The reason says she was passed over and why.
    let words = played.no_order_text(1);
    assert_eq!(words.len(), 1, "{words:?}");
    assert!(
        words[0].contains("submarine:3 (Placeholder submarine 3) is passed over, damaged"),
        "{}",
        words[0]
    );
    assert!(words[0].contains("goes to no one"), "{}", words[0]);
    // Boat 3 is not on a station of the waters.
    assert_ne!(played.boat_at_end_of(1, 3).1, hex(3, 2));
}

#[test]
fn statement_08_the_preview_says_each_boats_station_and_gives_the_same_stations() {
    // The preview is asked of a plan that waits for the commander's leave
    // to launch: the plan is ready (preparing) and nothing is ordered.
    let waiting = game(7);
    let mut waiting = waiting;
    waiting.commands = format!(
        "{}[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_leave\"\nleave = \"prepare\"\ngiven = true\n",
        intent_commands(1, (5, 0), 2, "japanese")
    );
    let mut scenario = waiting.scenario();
    let held = staffs();
    scenario
        .next_day_watched(as_staffs(&held), &mut |_, _| {})
        .expect("day 1 resolves");
    assert!(scenario
        .world
        .staff_handed_in()
        .get(&Side::Japanese)
        .is_none_or(|d| d.orders.is_empty()));
    let view = scenario.world.side_view(Side::Japanese);
    let proposal: Proposal = SupplyStaff
        .propose(view, IntentId::new(1))
        .expect("a proposal for the patrol order");
    // The same stations as the game that gives them.
    let given = game(7).play();
    let given_stations = given.stations(1);
    let previewed: Vec<(u32, Hex)> = proposal
        .decisions
        .orders
        .iter()
        .map(|order| match &order.kind {
            OrderKind::Submarine {
                target,
                order: SubmarineOrder::Station(at),
            } => (target.raw(), *at),
            other => panic!("{other:?}"),
        })
        .collect();
    assert_eq!(previewed, given_stations);
    assert_eq!(previewed, table(7));
    // And as data: each boat, her station and her place.
    let data: Vec<PatrolStation> = proposal.stations.clone();
    assert_eq!(data.len(), 7);
    for (n, (boat, at)) in table(7).iter().enumerate() {
        assert_eq!(
            data[n],
            PatrolStation {
                boat: SubmarineId::new(*boat),
                hex: *at,
                place: u32::try_from(n + 1).unwrap(),
            }
        );
    }
    // The preview's words name each boat and her station before it is given.
    for (boat, at) in table(7) {
        assert!(
            proposal
                .reason
                .worked
                .contains(&format!("submarine:{boat}")),
            "{}",
            proposal.reason.worked
        );
        assert!(
            proposal.reason.worked.contains(&format!("{at}")),
            "{}",
            proposal.reason.worked
        );
    }
}

// ---------------------------------------------------------------------
// Part B: whose hands a boat is in.
// ---------------------------------------------------------------------

#[test]
fn statement_09_a_direct_order_holds_a_boat_from_the_staff() {
    let b1 = game(7).with_extra(
        "\n[[order]]\nday = 1\nside = \"japanese\"\nsubmarine = 2\nkind = \"station\"\nhex = [9, 3]\n",
    );
    let played = b1.play();
    played.nothing_refused();
    // The staff orders boats 1, 3, 4, 5, 6 and 7 to stations 1, 3, 4, 5, 6
    // and 7.
    let expected = of(&[
        (1, (5, 0)),
        (3, (3, 2)),
        (4, (5, -2)),
        (5, (5, 2)),
        (6, (7, -2)),
        (7, (7, 0)),
    ]);
    assert_eq!(played.stations(1), expected);
    // Boat 2 goes to (9,3), by the commander's order.
    assert_eq!(played.boat_at_end_of(1, 2).1, hex(9, 3));
    let record = played
        .scenario
        .world
        .orders()
        .find(|o| matches!(&o.kind, OrderKind::Submarine { target, .. } if target.raw() == 2))
        .expect("boat 2's order");
    assert_eq!(record.origin.issuer, Issuer::Player);
    // The reason says boat 2 is the commander's.
    let words = played.no_order_text(1);
    assert_eq!(words.len(), 1, "{words:?}");
    assert!(
        words[0].contains(
            "submarine:2 (Placeholder submarine 2) is passed over, held by the commander"
        ),
        "{}",
        words[0]
    );
    // And the staff never orders her afterwards.
    for day in 2..=3 {
        assert!(played.stations(day).iter().all(|(b, _)| *b != 2));
    }
}

#[test]
fn statement_10_a_hold_on_a_boat_keeps_her_from_the_staff_and_released_she_is_the_staffs_again() {
    let held = game(7).with_extra(
        "\n[[command]]\nday = 1\nside = \"japanese\"\nkind = \"hold\"\nsubmarine = 2\n\n\
         [[command]]\nday = 2\nside = \"japanese\"\nkind = \"release\"\nsubmarine = 2\n",
    );
    let played = held.play();
    played.nothing_refused();
    // Held with no order she keeps what she was doing, at the base, and the
    // staff gives her none.
    let first = of(&[
        (1, (5, 0)),
        (3, (3, 2)),
        (4, (5, -2)),
        (5, (5, 2)),
        (6, (7, -2)),
        (7, (7, 0)),
    ]);
    assert_eq!(played.stations(1), first);
    assert_eq!(
        played.boat_at_end_of(1, 2),
        (hex(12, 0), hex(12, 0)),
        "she keeps what she was doing"
    );
    let words = played.no_order_text(1);
    assert!(
        words[0].contains(
            "submarine:2 (Placeholder submarine 2) is passed over, held by the commander"
        ),
        "{}",
        words[0]
    );
    // Released, she is the staff's at the next dusk and takes the station
    // her place gives: station 2.
    assert_eq!(played.stations(2), of(&[(2, (3, 0))]));
    played.nothing_refused();
}

#[test]
fn statement_11_only_the_boats_a_forces_limit_names_serve_and_a_boat_another_order_names_is_not_used(
) {
    // The first half is game A5 (statement 5): only boats 2, 5 and 6 serve.
    let a5 = game(7).with_extra(
        "\n[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nsubmarines = [2, 5, 6]\n",
    );
    let played = a5.play();
    let ordered: Vec<u32> = played.stations(1).iter().map(|s| s.0).collect();
    assert_eq!(ordered, vec![2, 5, 6]);
    // The second: patrol 1 names no boat and patrol 2, of the waters within 1
    // hex of (5,4), names boat 2. Patrol 1 does not use boat 2: she is
    // passed over and her station, 2, goes to no one.
    let mut two = game(7);
    two.commands
        .push_str(&format!("\n{}", intent_commands(2, (5, 4), 1, "japanese")));
    two = two.with_extra(
        "\n[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\nsubmarines = [2]\n",
    );
    let played = two.play();
    played.nothing_refused();
    let for_one: Vec<(u32, Hex)> = played.handed[&1]
        .orders
        .iter()
        .filter(|o| o.intent == IntentId::new(1))
        .map(|o| match &o.kind {
            OrderKind::Submarine {
                target,
                order: SubmarineOrder::Station(at),
            } => (target.raw(), *at),
            other => panic!("{other:?}"),
        })
        .collect();
    assert_eq!(
        for_one,
        of(&[
            (1, (5, 0)),
            (3, (3, 2)),
            (4, (5, -2)),
            (5, (5, 2)),
            (6, (7, -2)),
            (7, (7, 0)),
        ])
    );
    let for_two: Vec<(u32, Hex)> = played.handed[&1]
        .orders
        .iter()
        .filter(|o| o.intent == IntentId::new(2))
        .map(|o| match &o.kind {
            OrderKind::Submarine {
                target,
                order: SubmarineOrder::Station(at),
            } => (target.raw(), *at),
            other => panic!("{other:?}"),
        })
        .collect();
    assert_eq!(for_two, of(&[(2, (5, 4))]));
    let words = played.no_order_text(1).join(" ");
    assert!(
        words.contains("submarine:2 (Placeholder submarine 2) is passed over, used by intent:2"),
        "{words}"
    );
}

#[test]
fn statement_12_a_side_with_its_order_from_its_scenario_and_no_other_command_is_served_the_same_way(
) {
    // The Allied side has no commander at the table: its scenario gives it
    // the patrol order and both leaves, and its staff serves it as the
    // Japanese side's is served.
    let mut allied = game(7);
    allied.side = "allied";
    let played = allied.play();
    played.nothing_refused();
    assert_eq!(played.stations(1), table(7));
    played.assert_station_orders_taken(1, &table(7));
    let state = played.command_state();
    assert_eq!(state.intents.len(), 1);
    assert_eq!(
        state.intents[&IntentId::new(1)].maker,
        sim_core::command::Maker::Commander
    );
    // The other side has no intent and was handed nothing.
    assert!(played
        .scenario
        .world
        .command_state(Side::Japanese)
        .intents
        .is_empty());
}

// ---------------------------------------------------------------------
// Part D: what a scenario may not give.
// ---------------------------------------------------------------------

fn refusal(game: &Game) -> String {
    match common::from_text(&game.text(), &path()) {
        Ok(_) => panic!("the game loads"),
        Err(error) => error.to_string(),
    }
}

#[test]
fn statement_20_a_boat_with_no_salvo_is_refused_with_her_number() {
    let message = refusal(&game(3).boat_line(2, "salvoes = 0"));
    assert!(message.contains("submarine 2 has salvoes = 0"), "{message}");
}

#[test]
fn statement_21_a_home_base_no_sea_route_reaches_is_refused() {
    // Base 2 lies on a coast hex whose six neighbours are land, so no sea
    // route reaches it from the boat's station.
    let ring = "[[hex]]\nq = 0\nr = 6\nkind = \"coast\"\nport_size = 3\nanchorage = true\n\n\
        [[hex]]\nq = 0\nr = 5\nkind = \"land\"\n\n[[hex]]\nq = 1\nr = 5\nkind = \"land\"\n\n\
        [[hex]]\nq = 1\nr = 6\nkind = \"land\"\n";
    let mut shut_in = game(2);
    shut_in.map = Some(map_with("scenario_129_shut_in.toml", ring));
    shut_in.extra = "\n[[base]]\nid = 2\nname = \"Placeholder shut-in base\"\nside = \"japanese\"\nq = 0\nr = 6\nairfield = false\n"
        .to_string();
    let text = shut_in.text().replacen("home = 1\n", "home = 2\n", 1);
    let message = match common::from_text(&text, &path()) {
        Ok(_) => panic!("the game loads"),
        Err(error) => error.to_string(),
    };
    assert!(
        message.contains("no sea route reaches submarine:1's home base base:2"),
        "{message}"
    );
}

// ---------------------------------------------------------------------
// Part C: the staff and submarine waters (statements 13 to 19).
//
// The setting is the architect's: the Allied side has a commander and a
// staff, a carrier force C free to be stationed, and an order to deny the
// waters within 1 hex of (6,0). The setting here, all placeholder data on
// the placeholder patrol map with one Allied coast hex added at (12,4):
//
// - Allied base H (base 2) at (12,4), C's home port; the Japanese base
//   (base 1) at (12,0) as in the other games, which the Allied side's map
//   does not hold (the scenario gives it no knowledge of it), so no circle
//   of danger reaches any hex and, by decision 0203's rule 6, the way a
//   force takes is the shortest sea route.
// - C, task force 1, stopped at sea at (11,0): one carrier and two
//   destroyers (classes 17 and 18 of scenario 116's placeholder classes:
//   15 kn at cruise, 30 at full speed, 1,000 t of fuel, 4,000 nm of
//   endurance, full), on the carrier mission, with a bomber group of a
//   normal strike radius of 210 nm, a fighter group and a scouting group
//   aboard. 210 nm less 40 nm for the waters' 1 hex is 170 nm: the
//   candidates are the hexes within 170 nm of (6,0).
// - The commander gives "deny the waters within 1 hex of (6,0)" as intent
//   1, with no forces limit, and both leaves, on day 2 (so that what a
//   game holds on day 1 is held when the station is chosen on the
//   morning of day 2). Nothing else is given.
// ---------------------------------------------------------------------

/// The patrol map with its sea made longer to the west, so that the
/// Japanese base can lie further from every hex of the waters than a circle
/// of danger reaches (15 hexes), and `extra` appended; written once to the
/// build directory.
fn wide_map_with(name: &str, extra: &str) -> String {
    // Written once: the games run on parallel threads and read it.
    static MAP: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    MAP.get_or_init(|| {
        let original = fs::read_to_string(common::data_dir().join("map/placeholder_patrol.toml"))
            .expect("read the map");
        let wide = common::replace_once(&original, "q = [0, 12]", "q = [-24, 12]");
        let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
        fs::write(&path, format!("{wide}\n{extra}")).expect("write the map");
        path.to_string_lossy().into_owned()
    })
    .clone()
}

/// One game of Part C: lines added to the head, and the Japanese boat's
/// entry if there is one.
#[derive(Clone, Default)]
struct Sea {
    /// Whole tables added after the standard ones.
    extra: String,
    /// The seed of the game.
    seed: Option<u64>,
    /// The hex the Japanese boat starts at, if there is one.
    boat: Option<(i32, i32)>,
    /// The hex of a stopped Allied cruiser the boat may fire on, if any.
    decoy: Option<(i32, i32)>,
    /// The force's hex, (12,-2) by default.
    force_at: Option<(i32, i32)>,
    /// The Japanese base's hex, (-20,0) by default.
    enemy_base: Option<(i32, i32)>,
    /// The commander's commands of day 2 in place of the deny's and the
    /// two leaves.
    orders: Option<String>,
}

const DENY_AND_LEAVES: &str = "[[command]]\nday = 2\nside = \"allied\"\nkind = \"set_intent\"\nintent = 1\nintent_kind = \"deny\"\nhex = [6, 0]\ndistance = 1\n\n\
    [[command]]\nday = 2\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"prepare\"\ngiven = true\n\n\
    [[command]]\nday = 2\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"launch\"\ngiven = true\n";

impl Sea {
    fn text(&self) -> String {
        let map = wide_map_with(
            "scenario_129_part_c_map.toml",
            "[[hex]]\nq = 12\nr = 4\nkind = \"coast\"\nport_size = 3\nanchorage = true\n\n\
             [[hex]]\nq = -20\nr = 0\nkind = \"coast\"\nport_size = 3\nanchorage = true\n\n\
             [[hex]]\nq = 0\nr = 0\nkind = \"coast\"\nport_size = 3\nanchorage = true\n",
        );
        let (fq, fr) = self.force_at.unwrap_or((12, -2));
        let (bq, br) = self.enemy_base.unwrap_or((-20, 0));
        let seed = self.seed.unwrap_or(1942);
        let mut text = format!(
            "name = \"Placeholder scenario 129, part C\"\nplaceholder = true\nseed = {seed}\ndays = 3\nmap = \"{map}\"\n\
             ship_classes = [\"../reference/ship_classes.csv\", \"../reference/ship_classes_placeholder_scenario_116.csv\"]\n\
             port_sizes = \"../reference/port_sizes.csv\"\n\
             aircraft_types = [\"../reference/aircraft_types.csv\", \"../reference/aircraft_types_placeholder_scenario_116.csv\"]\n\
             search = \"../reference/search.toml\"\nsurface_combat = \"../reference/surface_combat.toml\"\nair_combat = \"../reference/air_combat.toml\"\n\
             submarine_classes = \"../reference/submarine_classes_placeholder_scenario_128.csv\"\n\
             submarine_torpedoes = \"../reference/submarine_torpedoes_placeholder_scenario_128.csv\"\n\
             submarines = \"../reference/submarines_placeholder_scenario_128.toml\"\n\n\
             [[base]]\nid = 1\nname = \"Placeholder Japanese base\"\nside = \"japanese\"\nq = {bq}\nr = {br}\nairfield = false\n\n\
             [[base]]\nid = 2\nname = \"Placeholder base H\"\nside = \"allied\"\nq = 12\nr = 4\nsupply_t = 1000\nfuel_t = 5000\n\n\
             [[task_force]]\nid = 1\nname = \"Placeholder carrier force C\"\nside = \"allied\"\nq = {fq}\nr = {fr}\nmission = \"carrier\"\nhome_port = 2\n\n\
             [[ship]]\nid = 1\nname = \"Placeholder carrier 1\"\nclass = 17\nside = \"allied\"\nfuel_t = 1000\ntask_force = 1\n\n\
             [[ship]]\nid = 2\nname = \"Placeholder destroyer 2\"\nclass = 18\nside = \"allied\"\nfuel_t = 1000\ntask_force = 1\n\n\
             [[ship]]\nid = 3\nname = \"Placeholder destroyer 3\"\nclass = 18\nside = \"allied\"\nfuel_t = 1000\ntask_force = 1\n\n\
             [[air_group]]\nid = 1\nname = \"Placeholder dive bombers 1\"\nside = \"allied\"\nship = 1\naircraft_type = 5\nready = 12\n\n\
             [[air_group]]\nid = 2\nname = \"Placeholder fighters 2\"\nside = \"allied\"\nship = 1\naircraft_type = 4\nready = 12\n\n\
             [[air_group]]\nid = 3\nname = \"Placeholder carrier scouts 3\"\nside = \"allied\"\nship = 1\naircraft_type = 2\nready = 6\n\n"
        );
        if let Some((q, r)) = self.decoy {
            text.push_str(&format!(
                "[[task_force]]\nid = 2\nname = \"Placeholder decoy force\"\nside = \"allied\"\nq = {q}\nr = {r}\nmission = \"surface_combat\"\n\n\
                 [[ship]]\nid = 4\nname = \"Placeholder cruiser 4\"\nclass = 1\nside = \"allied\"\nfuel_t = 2000\ntask_force = 2\n\n"
            ));
        }
        if let Some((q, r)) = self.boat {
            text.push_str(&format!(
                "[[submarine]]\nid = 1\nname = \"Placeholder boat 1\"\nside = \"japanese\"\nclass = 6900\nq = {q}\nr = {r}\nhome = 1\n\n"
            ));
        }
        text.push_str(&self.extra);
        text.push_str(self.orders.as_deref().unwrap_or(DENY_AND_LEAVES));
        text
    }

    fn scenario(&self) -> Scenario {
        common::from_text(&self.text(), &path()).expect("the game loads")
    }

    fn play(&self) -> Played {
        play(self.scenario(), Side::Allied)
    }
}

/// The suspected waters the Allied side is told on day 1.
fn waters_told(q: i32, r: i32, distance: u32) -> String {
    format!(
        "[[suspected_waters]]\nside = \"allied\"\nq = {q}\nr = {r}\ndistance = {distance}\nday = 1\n\
         words = \"Placeholder belief: enemy boats work these waters\"\n\
         rows = \"placeholder, no row\"\nreading = \"Placeholder reading, the architect's of scenario 129\"\n\n"
    )
}

/// The seed whose game, with a boat at (10,-3) and a cruiser stopped there
/// for her to fire on, has her fire on the first night and the Allied side
/// told of one report at her hex, (10,-3), and of no other on day 1 (found
/// by trying seeds from 1; the first is 2).
const REPORT_SEED: u64 = 2;

fn reason_text(reason: &Reason) -> String {
    let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
    format!(
        "{} | {} | {}",
        reason.worked,
        evidence.join(" | "),
        reason.binding.join(" | ")
    )
}

impl Played {
    /// The carrier task formed on `day`: its station and its reason. At most
    /// one is formed.
    fn carrier_task(&self, day: u32) -> Option<(Hex, &Reason)> {
        let formed: Vec<(Hex, &Reason)> = self.handed[&day]
            .commands
            .iter()
            .filter_map(|c| match c.kind {
                CommandKind::FormIntent {
                    kind: IntentKind::OperateCarriers { force, hex },
                    ..
                } if force == TaskForceId::new(1) => Some((hex, &c.reason)),
                _ => None,
            })
            .collect();
        assert!(formed.len() <= 1, "one carrier task at most: {formed:?}");
        formed.first().copied()
    }

    /// The station of the carrier task formed on day 2 (the order is given
    /// on day 2), and on no other day.
    fn station(&self) -> Hex {
        for day in [1, 3] {
            assert!(
                self.carrier_task(day).is_none(),
                "no task formed on day {day}"
            );
        }
        self.carrier_task(2).expect("a task formed on day 2").0
    }

    fn station_reason(&self) -> String {
        reason_text(self.carrier_task(2).expect("a task formed on day 2").1)
    }

    /// Every reason of every day, as one text.
    fn all_said(&self) -> String {
        self.handed
            .values()
            .flat_map(|d| {
                d.orders
                    .iter()
                    .map(|o| &o.reason)
                    .chain(d.no_orders.iter())
                    .chain(d.commands.iter().map(|c| &c.reason))
            })
            .map(reason_text)
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// The routes the staff gave task force 1 on `day`.
    fn routes(&self, day: u32) -> Vec<Vec<Hex>> {
        self.handed[&day]
            .orders
            .iter()
            .filter_map(|o| match &o.kind {
                OrderKind::TaskForce {
                    target,
                    order: TaskForceOrder::SetRoute { waypoints, .. },
                } if *target == TaskForceId::new(1) => Some(waypoints.clone()),
                _ => None,
            })
            .collect()
    }

    /// The reports of a submarine the Allied side was told of on `day`.
    fn reports_on(&self, day: u32) -> Vec<Hex> {
        self.events
            .iter()
            .filter(|e| e.day == day)
            .filter_map(|e| match &e.kind {
                EventKind::SubmarineReported { side, hex, .. } if *side == Side::Allied => {
                    Some(*hex)
                }
                _ => None,
            })
            .collect()
    }
}

/// The hexes the rule ranks, in its order, for C1: the candidates (the hexes
/// the force can reach by sea within the reach the waters leave, 210 nm less
/// 40 nm for the waters' 1 hex, of (6,0)), each with its passage by sea from
/// the force's hex and, there being no circle of danger, no margin, ordered
/// by the rule of the balanced risk (the least passage, then hex order). It
/// is worked on the Allied view of the game as it stands before day 1, by
/// the rule's own public functions.
fn rule_order(sea: &Sea) -> Vec<Hex> {
    let scenario = sea.scenario();
    let view = scenario.world.side_view(Side::Allied);
    let here = sea.force_at.map_or(hex(12, -2), |(q, r)| hex(q, r));
    let field = view.sea_distances_to(here);
    let within = reach_for(Fixed::from_int(210), Some(1));
    let centre = hex(6, 0);
    let candidates: Vec<Candidate> = field
        .iter()
        .filter(|(h, _)| h.air_distance_within(centre, within))
        .map(|(h, passage)| Candidate {
            hex: *h,
            passage: *passage,
            margin: margin_of(*h, &[]),
        })
        .collect();
    assert_eq!(
        candidates.len(),
        61,
        "the candidates: hexes within 4 of (6,0)"
    );
    order_by_risk(Risk::Balanced, &candidates)
        .into_iter()
        .map(|c| c.hex)
        .collect()
}

/// The rule's first hex outside the circle of `hexes` hexes round `centre`.
fn first_outside(order: &[Hex], centre: Hex, hexes: u32) -> Hex {
    *order
        .iter()
        .find(|h| h.distance(centre) > hexes)
        .expect("a hex outside")
}

#[test]
fn statement_13_game_c1_no_submarine_belief_the_station_is_s_and_no_reason_speaks_of_submarines() {
    let sea = Sea::default();
    // S, by the rule: of the 61 candidates the least passage from (12,-2) is
    // 2 hexes, held by (10,-2), (10,-1) and (10,0) alone, and the lowest hex
    // is (10,-2).
    let order = rule_order(&sea);
    assert_eq!(&order[..3], &[hex(10, -2), hex(10, -1), hex(10, 0)]);
    let played = sea.play();
    assert_eq!(played.station(), hex(10, -2));
    assert!(played.station_reason().contains("The station is (10,-2)"));
    let said = played.all_said().to_lowercase();
    assert!(!said.contains("submarine"), "{said}");
    assert!(!said.contains("passed over"), "{said}");
    for day in 1..=3 {
        for command in &played.handed[&day].commands {
            assert!(
                command.reason.cautions.is_empty(),
                "no caution: {command:?}"
            );
        }
    }
}

#[test]
fn statement_14_game_c2_suspected_waters_that_hold_s_the_force_is_stationed_at_the_next_hex_outside(
) {
    // Waters of 1 hex round (10,-3) hold (10,-2), S, one hex from the centre,
    // and not (10,-1), 2 hexes from it.
    let sea = Sea {
        extra: waters_told(10, -3, 1),
        ..Sea::default()
    };
    // The next the rule ranks that is outside the waters, worked from the
    // rule's own order on the same view.
    let order = rule_order(&sea);
    assert_eq!(order[0], hex(10, -2));
    let expected = first_outside(&order, hex(10, -3), 1);
    assert_eq!(expected, hex(10, -1));
    let played = sea.play();
    assert_eq!(played.station(), expected);
    let reason = played.station_reason();
    assert!(
        reason.contains(
            "(10,-2) passed over: inside the suspected submarine waters within 1 hex of (10,-3)"
        ),
        "{reason}"
    );
    assert!(
        !reason.contains("no place outside submarine waters serves"),
        "{reason}"
    );
    assert!(reason.contains("The station is (10,-1)"), "{reason}");
}

#[test]
fn statement_15_game_c3_a_report_of_a_submarine_next_to_s_the_force_is_stationed_at_the_next_hex_outside(
) {
    // A boat at (10,-3) fires on a cruiser stopped there in the first night:
    // by the engine's rule (decision 0218, rule 14) the side fired on is
    // told of one submarine at her hex, here undisplaced by the error. The
    // report is held from day 1; the order is given on day 2.
    let sea = Sea {
        seed: Some(REPORT_SEED),
        boat: Some((10, -3)),
        decoy: Some((10, -3)),
        ..Sea::default()
    };
    let played = sea.play();
    assert_eq!(played.reports_on(1), vec![hex(10, -3)]);
    // S is within one hex of the report; (10,-1) is two hexes from it.
    let expected = first_outside(&rule_order(&sea), hex(10, -3), 1);
    assert_eq!(expected, hex(10, -1));
    assert_eq!(played.station(), expected);
    let reason = played.station_reason();
    assert!(
        reason.contains(
            "(10,-2) passed over: within one hex of a report of a submarine at (10,-3), day 1"
        ),
        "{reason}"
    );
    assert!(
        !reason.contains("no place outside submarine waters serves"),
        "{reason}"
    );
}

#[test]
fn statement_16_game_c4_waters_that_hold_every_candidate_the_force_is_at_s_and_no_place_outside_serves(
) {
    // The waters of 4 hexes round (6,0) hold every hex within 170 nm of it.
    let sea = Sea {
        extra: waters_told(6, 0, 4),
        ..Sea::default()
    };
    let order = rule_order(&sea);
    assert!(order.iter().all(|h| h.distance(hex(6, 0)) <= 4));
    let played = sea.play();
    assert_eq!(played.station(), hex(10, -2));
    let reason = played.station_reason();
    assert!(
        reason.contains("no place outside submarine waters serves, so the station is (10,-2), inside the suspected submarine waters within 4 hexes of (6,0)"),
        "{reason}"
    );
    assert!(!reason.contains("passed over"), "{reason}");
    // Decision 0217's cautions: one of the new kind, of the station's hex.
    let (_, formed) = played.carrier_task(2).expect("a task");
    let cautions: Vec<_> = formed
        .cautions
        .iter()
        .map(|c| (c.kind, c.hex, c.of))
        .collect();
    assert_eq!(
        cautions,
        vec![(
            CautionKind::StationInSubmarineWaters,
            Some(hex(10, -2)),
            None
        )]
    );
    assert_eq!(CautionKind::StationInSubmarineWaters.code(), 3);
    assert_eq!(
        CautionKind::from_name("station_in_submarine_waters"),
        Some(CautionKind::StationInSubmarineWaters)
    );
}

#[test]
fn statement_17_the_route_is_as_the_route_rule_gives_it_not_bent_for_submarine_waters() {
    // No circle of danger: by decision 0203's rule 6 the way is the shortest
    // sea route, so the route is the one point of the station.
    let c1 = Sea::default().play();
    assert_eq!(c1.routes(2), vec![vec![hex(10, -2)]]);
    let c2 = Sea {
        extra: waters_told(10, -3, 1),
        ..Sea::default()
    }
    .play();
    assert_eq!(c2.routes(2), vec![vec![hex(10, -1)]]);
    let c3 = Sea {
        seed: Some(REPORT_SEED),
        boat: Some((10, -3)),
        decoy: Some((10, -3)),
        ..Sea::default()
    }
    .play();
    assert_eq!(c3.routes(2), vec![vec![hex(10, -1)]]);
    // And in the game of every candidate held, the station and the route are
    // those of C1.
    let c4 = Sea {
        extra: waters_told(6, 0, 4),
        ..Sea::default()
    }
    .play();
    assert_eq!(c4.routes(2), vec![vec![hex(10, -2)]]);
}

// Statement 18, "where a force waits": scenario 73's game (a carrier force
// named in the forces limit of a take of the place at (10,0), its covering
// hex (4,0) by that scenario's hand calculation, the next hex along the
// row (5,0)) with the Allied side told suspected waters on day 1.

fn take_path() -> PathBuf {
    common::data_dir().join("scenarios/73_carrier_cover.toml")
}

fn take_figures() -> sim_data::LoadOptions {
    common::altered_reference("s129take", |dir| {
        common::edit_file(dir, "air_combat.toml", |text| {
            format!("{text}\ncruiser = 12\ndestroyer = 6\ntransport = 6\nauxiliary = 4\n")
        })
    })
}

/// Scenario 73's game with `extra` added, played, with the Allied staff's
/// carrier task of the morning of day 2.
fn take_game(extra: &str) -> (Hex, String) {
    let text = format!(
        "{}\n{extra}",
        fs::read_to_string(take_path()).expect("scenario 73 reads")
    );
    let scenario =
        common::from_text_with(&text, &take_path(), &take_figures()).expect("the take game loads");
    let played = play(scenario, Side::Allied);
    let (hex, reason) = played.carrier_task_of_any_force(2);
    (hex, reason)
}

impl Played {
    /// The one carrier task formed on `day`, whichever force: station and
    /// reason.
    fn carrier_task_of_any_force(&self, day: u32) -> (Hex, String) {
        let formed: Vec<(Hex, String)> = self.handed[&day]
            .commands
            .iter()
            .filter_map(|c| match c.kind {
                CommandKind::FormIntent {
                    kind: IntentKind::OperateCarriers { hex, .. },
                    ..
                } => Some((hex, reason_text(&c.reason))),
                _ => None,
            })
            .collect();
        assert_eq!(formed.len(), 1, "one carrier task on day {day}: {formed:?}");
        formed.into_iter().next().expect("one")
    }
}

#[test]
fn statement_18_where_a_force_waits_the_covering_hex_is_the_next_along_the_route_outside_submarine_waters(
) {
    // Without waters: scenario 73's covering hex, (4,0), and no word of
    // submarines.
    let (hex_none, reason_none) = take_game("");
    assert_eq!(hex_none, hex(4, 0));
    assert!(
        !reason_none.to_lowercase().contains("submarine"),
        "{reason_none}"
    );
    // Waters of 1 hex round (3,0) hold (4,0) and not (5,0), the next hex of
    // the row the shortest sea route follows.
    let (hex_next, reason_next) = take_game(&waters_told(3, 0, 1));
    assert_eq!(hex_next, hex(5, 0));
    assert!(
        reason_next.contains(
            "(4,0) passed over: inside the suspected submarine waters within 1 hex of (3,0)"
        ),
        "{reason_next}"
    );
    // Waters of 3 hexes round (7,0) hold every hex of the route within the
    // reach, (4,0) to (10,0): the first serves, and the reason says no place
    // outside serves.
    let (hex_all, reason_all) = take_game(&waters_told(7, 0, 3));
    assert_eq!(hex_all, hex(4, 0));
    assert!(
        reason_all.contains("no place outside submarine waters serves, so it is (4,0)"),
        "{reason_all}"
    );
    assert!(!reason_all.contains("passed over"), "{reason_all}");
}

#[test]
fn statement_19_the_staff_reads_only_what_the_side_believes_a_boat_at_s_the_side_holds_no_report_of(
) {
    // C1 with a Japanese boat at S = (10,-2), and no cruiser for her to
    // fire on: the Allied side holds no report and no suspected waters on
    // day 1, so the staff stations the force at S.
    let played = Sea {
        boat: Some((10, -2)),
        ..Sea::default()
    }
    .play();
    assert!(played.reports_on(1).is_empty());
    assert_eq!(played.station(), hex(10, -2));
    let said = played.all_said().to_lowercase();
    assert!(!said.contains("submarine"), "{said}");
    assert!(!said.contains("passed over"), "{said}");
}
