//! Scenario 131: the pattern of a patrol (decision 0223 as amended on review,
//! stages 0 and A).
//!
//! # The architect's statements
//!
//! Written by the architect on Nov 23, 2026, before any code. Nobody else
//! changes a statement or a hex here; a mismatch goes back to the architect.
//! The hexes are worked by hand below.
//!
//! ## The map and the games
//!
//! One placeholder map, `placeholder_patterns.toml`: deep sea for q from -10
//! to 10 and r from -8 to 8, with a coast hex under each base a game has and
//! nothing else unless a game says so. Hexes are axial (q, r); the distance
//! between two hexes is (|dq| + |dr| + |dq + dr|) / 2. Every boat is a
//! placeholder boat of scenario 128's placeholder class, on station at her
//! side's home base at the start, and "boat N" is the side's Nth boat in
//! number order. Each game is three days; the stations are read from the
//! staff's orders of the first morning the patrol is served and from
//! `preview`'s and `propose`'s `stations`.
//!
//! The line games (the Japanese side has the boats; its home base is at
//! (-9, 0); the patrol's centre is (0, 0)):
//!
//! - **L1:** pattern `lines`; one Allied base at (8, 0); waters within 3
//!   hexes; nine boats.
//! - **L2:** as L1 with waters within 6 hexes and fifteen boats.
//! - **L3:** as L1 with (-1, 1) a hex of land and six boats.
//! - **L4:** pattern `lines`; one Allied base, at (4, 0); waters within 3;
//!   three boats.
//! - **L5:** as L1 with a second Allied base at (0, -7) and three boats.
//! - **L6:** as L1 with no pattern given to the Japanese side, and three
//!   boats.
//!
//! The base games (the Allied side has the boats; its home base is at
//! (8, 0); the Japanese bases are base 1 at (-8, 0), base 2 at (-6, 3) and
//! base 3 at (-2, 0); the patrol's centre is (-6, 0), waters within 3):
//!
//! - **B1:** pattern `off_bases`; seven boats.
//! - **B2:** as B1 with (-3, 0) a hex of land and four boats.
//! - **B3:** pattern `off_bases`; the patrol's centre is (4, -4), waters
//!   within 2; two boats.
//!
//! ## Stage 0: a scenario says how a side's staff uses its boats
//!
//! 1. A scenario may give a side `submarine_pattern`, `off_bases` or
//!    `lines`, and `expects_boats_off_own_bases`, true or false. A side
//!    given neither has no pattern and expects nothing.
//! 2. Any other word for the pattern is refused when the scenario is
//!    loaded, naming the side and the word.
//! 3. A side's own picture holds its pattern and what it expects. The other
//!    side's picture, and every answer of every tool to the other side's
//!    seat, holds neither.
//! 4. Every scenario on file that gives no pattern plays as it did: the
//!    comparison of every run with the hashes cut shows no difference, and
//!    scenario 129's stations are what they were.
//!
//! ## Stage A: lines
//!
//! 5. **L1.** The approach is from the Allied base at (8, 0). The boats are
//!    stationed, boat 1 to boat 9, at (0, 0), (-1, 1), (0, -1), (-1, 2),
//!    (1, -2), (-2, 3), (1, -3), (-3, 0) and (2, 1).
//! 6. **L1.** The reason of each of the first seven station orders says the
//!    boat is on the first line across the approach from that base, naming
//!    the base; the reason of the last two says they are spread over the
//!    waters, the line being full.
//! 7. **L2.** The boats are stationed, boat 1 to boat 15, at (0, 0),
//!    (-1, 1), (0, -1), (-1, 2), (1, -2), (-2, 3), (1, -3), (-2, 4),
//!    (2, -4), (-3, 5), (2, -5), (-3, 6), (3, -6), (-5, 0) and (-6, 1). The
//!    reason of the last two says they are on the second line.
//! 8. **L3.** The line steps over the land: the boats are stationed, boat 1
//!    to boat 6, at (0, 0), (-1, 2), (0, -1), (-2, 3), (1, -2) and (1, -3).
//!    No boat is given (-1, 1).
//! 9. **L4.** The only enemy base lies within 5 hexes of the centre, so
//!    there is no approach and no line: the boats are stationed as decision
//!    0218 stations them, at (0, 0), (-3, 0) and (-3, 3), and the reason
//!    says there is no line and why.
//! 10. **L5.** The approach is from the base at (0, -7), the nearer of the
//!     two that lie more than 5 hexes from the centre: the boats are
//!     stationed at (0, 0), (1, -1) and (-1, 0).
//! 11. **L6.** With no pattern the boats are stationed as decision 0218
//!     stations them: at (0, 0), (-3, 0) and (-3, 3).
//! 12. **L1, the second morning.** No boat is given a station order: each
//!     keeps the station she was given, and the one decision to give no
//!     order names each as keeping it.
//!
//! ## Stage A: off bases
//!
//! 13. **B1.** The boats are stationed, boat 1 to boat 7, at (-7, 0),
//!     (-6, 2), (-3, 0), (-8, 1), (-7, 3), (-6, -3) and (-4, -2).
//! 14. **B1.** The reason of boats 1 and 4 says each is off base 1, of
//!     boats 2 and 5 off base 2, and of boat 3 off base 3, each naming the
//!     base; the reason of boats 6 and 7 says they are spread over the
//!     waters. Base 3 is given one boat and not two, and the staff's words
//!     say why: no second hex next to it lies in the waters.
//! 15. **B2.** Base 3 has no hex next to it that ships can enter and that
//!     lies in the waters, and is passed over: the boats are stationed at
//!     (-7, 0), (-6, 2), (-8, 1) and (-7, 3).
//! 16. **B3.** No enemy base lies in the waters or within a hex of them:
//!     the boats are stationed as decision 0218 stations them, at (4, -4)
//!     and (2, -4).
//! 17. In every game no boat is stationed on land, on a base's own hex, or
//!     outside the waters, and no two boats of one patrol share a station.
//!
//! ## The hexes, worked by hand
//!
//! **The line.** For an approach whose vector from the centre is (q, r),
//! the vector across it is (-(q + 2r), 2q + r), divided by the greatest
//! common divisor of its two parts. The line is drawn from the centre to
//! the centre plus that vector, and on from there by the same vector, and
//! the same with the vector's opposite: between two ends N hexes apart the
//! hexes are the N + 1 points at even steps, each rounded to the nearest
//! hex, the lowest hex (q, then r) where two are equally near. The first
//! hand is the vector as worked, the other its opposite. Stations are the
//! centre, then one hex farther along the first hand, then along the other,
//! by turns; when one hand has left the waters the other goes on.
//!
//! *L1, L2, L3, L6's geometry: approach (8, 0).* Across: (-(8), 16), which
//! is (-1, 2) eight times. From (0, 0) to (-1, 2) is 2 hexes: the middle
//! point is (-0.5, 1), equally near (-1, 1) and (0, 1), so (-1, 1); then
//! (-1, 2). On to (-2, 4): middle (-1.5, 3), so (-2, 3); then (-2, 4). On
//! to (-3, 6): middle (-2.5, 5), so (-3, 5); then (-3, 6). The other hand,
//! from (0, 0) to (1, -2): middle (0.5, -1), equally near (0, -1) and
//! (1, -1), so (0, -1); then (1, -2); then (1, -3) (middle (1.5, -3)),
//! (2, -4), (2, -5) (middle (2.5, -5)), (3, -6).
//!
//! Distances from the centre: (-1, 1) 1, (-1, 2) 2, (-2, 3) 3, (-2, 4) 4,
//! (-3, 5) 5, (-3, 6) 6, (-4, 7) 7; (0, -1) 1, (1, -2) 2, (1, -3) 3,
//! (2, -4) 4, (2, -5) 5, (3, -6) 6. So within 3 hexes the first line has 7
//! stations and within 6 it has 13.
//!
//! *L1's last two boats.* The second line would pass through (-5, 0), five
//! hexes from the centre away from the enemy, which is outside waters of 3
//! hexes: there is none. The two boats left are spread: each goes to the
//! hex of the waters farthest from the nearest station already given. No
//! hex of waters of 3 hexes is more than 3 from (0, 0), and four are 3 from
//! every station of the line: (-3, 0), (2, 1), (3, 0) and (3, -1). (For
//! (-3, 0): 3 from (0, 0), 3 from (-1, 1), 3 from (0, -1). For (2, 1): 3
//! from (0, 0), 3 from (-1, 1), 3 from (-1, 2). The other hexes at 3 from
//! the centre are within 2 of a station: (-3, 1) of (-1, 1), (-3, 2) of
//! (-1, 2), (3, -2) of (1, -2), (3, -3) of (1, -3), and so round.) The
//! lowest is (-3, 0). With (-3, 0) given, (2, 1), (3, 0) and (3, -1) are
//! still 3 from every station; the lowest is (2, 1).
//!
//! *L2's last two boats.* The second line passes through (-5, 0), which is
//! 5 from the centre and in waters of 6. From (-5, 0) to (-6, 2): middle
//! (-5.5, 1), equally near (-6, 1) and (-5, 1), so (-6, 1), which is 6 from
//! the centre. Boat 14 has (-5, 0) and boat 15 the next along the first
//! hand, (-6, 1).
//!
//! *L3.* (-1, 1) is land. The first hand's first hex ships can enter is
//! (-1, 2), then (-2, 3); (-2, 4) is outside. The other hand: (0, -1),
//! (1, -2), (1, -3). By turns: (0, 0), (-1, 2), (0, -1), (-2, 3), (1, -2),
//! and the first hand having left the waters, (1, -3).
//!
//! *L4 and L6, the spread of decision 0218.* The centre; then the hex of
//! the waters farthest from it, where all the hexes 3 away tie and the
//! lowest is (-3, 0); then the hex farthest from both: (-3, 3) is 3 from
//! each, and no hex with q below -3 is in the waters, and (-3, 1) and
//! (-3, 2) are 1 and 2 from (-3, 0).
//!
//! *L5: approach (0, -7).* The base at (0, -7) is 7 from the centre and
//! the base at (8, 0) is 8; both are more than 5, and the nearer is taken.
//! Across: (-(0 - 14), 0 - 7) = (14, -7), which is (2, -1) seven times.
//! From (0, 0) to (2, -1) is 2 hexes: the middle point is (1, -0.5),
//! equally near (1, -1) and (1, 0), so (1, -1). The other hand, to
//! (-2, 1): middle (-1, 0.5), equally near (-1, 0) and (-1, 1), so (-1, 0).
//!
//! **Off bases.** *B1.* Distances of the bases from the centre (-6, 0):
//! base 1 at (-8, 0) is 2, base 2 at (-6, 3) is 3, base 3 at (-2, 0) is 4,
//! which is one hex outside the waters. In that order:
//!
//! - Base 1's neighbours and their distances from the centre: (-7, 0) 1,
//!   (-7, -1) 2, (-8, 1) 2, (-8, -1) 3, (-9, 0) 3, (-9, 1) 3. Boat 1 has
//!   (-7, 0).
//! - Base 2's: (-6, 2) 2, (-5, 2) 3, (-7, 3) 3, (-5, 3) 4, (-6, 4) 4,
//!   (-7, 4) 4. Boat 2 has (-6, 2).
//! - Base 3's: (-3, 0) 3, (-3, 1) 4, (-2, -1) 4, (-1, 0) 5, (-2, 1) 5,
//!   (-1, -1) 5. Only (-3, 0) is in the waters. Boat 3 has it.
//! - The second round: base 1's nearest hexes not taken are (-7, -1) and
//!   (-8, 1), both 2 away; the lowest is (-8, 1), boat 4. Base 2's are
//!   (-5, 2) and (-7, 3), both 3; the lowest is (-7, 3), boat 5. Base 3
//!   has none.
//! - The spread, for boats 6 and 7: the hexes of the waters 3 from the
//!   nearest of the five stations are (-6, -3), (-5, -3), (-4, -3),
//!   (-3, -3) and (-4, -2), and none is farther. The lowest is (-6, -3).
//!   With it given, (-4, -2) and (-3, -3) are still 3 from every station
//!   and (-5, -3) and (-4, -3) are not; the lowest is (-4, -2).
//!
//! *B2.* With (-3, 0) land, base 3 has no hex. Boats 1 and 2 as in B1;
//! the second round gives boats 3 and 4 (-8, 1) and (-7, 3).
//!
//! *B3.* The Japanese bases are 11 hexes and more from (4, -4). The centre,
//! then the lowest of the hexes 2 away: those with q = 2 are (2, -4),
//! (2, -3) and (2, -2), and the lowest is (2, -4).

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::command::{StaffDecisions, SubmarineDoctrine, SubmarinePattern};
use sim_core::events::{Event, EventKind};
use sim_core::hex::Hex;
use sim_core::ids::IntentId;
use sim_core::orders::{Issuer, OrderKind, SubmarineOrder};
use sim_core::state::Side;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

use crate::common;

// ---------------------------------------------------------------------
// The games, each a file of `data/scenarios/the-pattern-of-a-patrol/`.
// ---------------------------------------------------------------------

#[derive(Clone, Copy)]
enum Game {
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    B1,
    B2,
    B3,
}

impl Game {
    fn file(self) -> &'static str {
        match self {
            Game::L1 => "131_l1_lines.toml",
            Game::L2 => "131_l2_lines_wider_waters.toml",
            Game::L3 => "131_l3_lines_over_land.toml",
            Game::L4 => "131_l4_no_approach.toml",
            Game::L5 => "131_l5_the_nearer_approach.toml",
            Game::L6 => "131_l6_no_pattern.toml",
            Game::B1 => "131_b1_off_bases.toml",
            Game::B2 => "131_b2_a_base_passed_over.toml",
            Game::B3 => "131_b3_no_base_near.toml",
        }
    }

    fn path(self) -> PathBuf {
        common::data_dir()
            .join("scenarios/the-pattern-of-a-patrol")
            .join(self.file())
    }

    fn text(self) -> String {
        std::fs::read_to_string(self.path()).expect("the game's file reads")
    }

    /// The side with the boats.
    fn side(self) -> Side {
        match self {
            Game::B1 | Game::B2 | Game::B3 => Side::Allied,
            _ => Side::Japanese,
        }
    }

    /// The patrol's order, as the commander's tools take it.
    fn patrol(self) -> Value {
        match self {
            Game::B1 | Game::B2 => json!({"intent_kind": "patrol", "hex": [-6, 0], "distance": 3}),
            Game::B3 => json!({"intent_kind": "patrol", "hex": [4, -4], "distance": 2}),
            Game::L2 => json!({"intent_kind": "patrol", "hex": [0, 0], "distance": 6}),
            _ => json!({"intent_kind": "patrol", "hex": [0, 0], "distance": 3}),
        }
    }

    fn scenario(self) -> Scenario {
        common::load(&self.path()).expect("the game loads")
    }

    fn play(self) -> Played {
        play(self.scenario(), self.side())
    }
}

fn hex(q: i32, r: i32) -> Hex {
    Hex::new(q, r)
}

fn of(rows: &[(i32, i32)]) -> Vec<(u32, Hex)> {
    rows.iter()
        .zip(1..)
        .map(|((q, r), boat)| (boat, hex(*q, *r)))
        .collect()
}

/// A game played to its end with the doctrine's staffs: what the side's
/// staff handed in each day, the log and the world.
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    events: Vec<Event>,
    scenario: Scenario,
}

fn play(mut scenario: Scenario, side: Side) -> Played {
    let held = staffs();
    let mut handed = BTreeMap::new();
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
    }
    let events = scenario.world.events().iter().cloned().collect();
    Played {
        handed,
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

    /// The staff's station orders as the engine took them, in order.
    fn taken(&self) -> Vec<(u32, Hex)> {
        let mut taken = Vec::new();
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
            assert_eq!(record.origin.intent, Some(IntentId::new(1)));
            let sim_core::ids::EntityRef::Submarine(id) = target else {
                panic!("a station order to {target:?}");
            };
            taken.push((id.raw(), *at));
        }
        taken
    }

    /// The working of the staff's order to boat `raw` on day 1.
    fn words(&self, raw: u32) -> String {
        self.handed[&1]
            .orders
            .iter()
            .find(|o| matches!(&o.kind, OrderKind::Submarine { target, .. } if target.raw() == raw))
            .map(|o| o.reason.worked.clone())
            .expect("an order for the boat")
    }

    fn no_order_text(&self, day: u32) -> Vec<String> {
        self.handed[&day]
            .no_orders
            .iter()
            .map(|r| r.worked.clone())
            .collect()
    }
}

/// A session over the game's file as `sim-cli` builds one, both seats
/// played, the scenario's own commands taken out: the commander gives his
/// own.
fn session(game: Game) -> Session {
    let mut scenario = game.scenario();
    scenario.replace_commands(BTreeMap::new());
    let name = scenario.name.clone();
    Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
    .with_staffs(staffs())
}

fn ask(s: &mut Session, side: Side, tool: &str, args: Value) -> Value {
    s.call(side, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} {args}: {}", e.message))
}

/// The stations of an answer's `stations`, as (boat, hex).
fn answer_stations(stations: &Value) -> Vec<(u32, Hex)> {
    stations
        .as_array()
        .expect("stations")
        .iter()
        .map(|s| {
            let boat = s["submarine"]
                .as_str()
                .and_then(|h| h.strip_prefix("submarine:"))
                .and_then(|n| n.parse().ok())
                .expect("a boat's handle");
            let q = i32::try_from(s["hex"]["q"].as_i64().expect("q")).unwrap();
            let r = i32::try_from(s["hex"]["r"].as_i64().expect("r")).unwrap();
            (boat, hex(q, r))
        })
        .collect()
}

/// Stations as (boat, hex), in the order given.
type Stations = Vec<(u32, Hex)>;

/// The stations `propose` gives for the patrol once ordered, and those
/// `preview` gives for the order before it is given.
fn proposed_and_previewed(game: Game) -> (Stations, Stations) {
    let side = game.side();
    let mut s = session(game);
    let mut command = game.patrol();
    command["command"] = json!("set_intent");
    let preview = ask(&mut s, side, "preview", command);
    assert_eq!(preview["valid"], true, "{preview}");
    let previewed = answer_stations(&preview["with_change"]["stations"]);
    ask(&mut s, side, "set_intent", game.patrol());
    let proposal = ask(&mut s, side, "propose", json!({"intent": 1}));
    (answer_stations(&proposal["stations"]), previewed)
}

/// Statement 17, for one game: no station on land, on a base's hex, or
/// outside the waters, and no two boats of the patrol share one.
fn sound(game: Game, played: &Played) {
    let world = &played.scenario.world;
    let given = played.stations(1);
    let (centre, distance) = match game {
        Game::B1 | Game::B2 => (hex(-6, 0), 3),
        Game::B3 => (hex(4, -4), 2),
        Game::L2 => (hex(0, 0), 6),
        _ => (hex(0, 0), 3),
    };
    let bases: Vec<Hex> = world.umpire().bases.values().map(|b| b.hex).collect();
    let mut seen = std::collections::BTreeSet::new();
    for (boat, at) in &given {
        assert!(world.map().is_navigable(*at), "boat {boat} on land at {at}");
        assert!(!bases.contains(at), "boat {boat} on a base's hex at {at}");
        assert!(
            at.distance(centre) <= distance,
            "boat {boat} outside at {at}"
        );
        assert!(seen.insert(*at), "two boats share {at}");
    }
}

/// A game's stations, checked whole: those handed in on the first morning,
/// as the engine took them, as `propose` and `preview` give them, and sound.
fn stationed(game: Game, expected: &[(i32, i32)]) -> Played {
    let played = game.play();
    played.nothing_refused();
    let expected = of(expected);
    assert_eq!(played.stations(1), expected, "the first morning's orders");
    assert_eq!(played.taken(), expected, "the orders the engine took");
    let (proposed, previewed) = proposed_and_previewed(game);
    assert_eq!(proposed, expected, "propose's stations");
    assert_eq!(previewed, expected, "preview's stations");
    sound(game, &played);
    played
}

// ---------------------------------------------------------------------
// Stage 0.
// ---------------------------------------------------------------------

#[test]
fn statement_01_a_scenario_gives_a_side_its_pattern_and_what_it_expects() {
    let lines = Game::L1.scenario();
    assert_eq!(
        lines.world.side_view(Side::Japanese).submarine_doctrine(),
        SubmarineDoctrine {
            pattern: Some(SubmarinePattern::Lines),
            expects_boats_off_own_bases: false,
        }
    );
    let bases = Game::B1.scenario();
    assert_eq!(
        bases.world.side_view(Side::Allied).submarine_doctrine(),
        SubmarineDoctrine {
            pattern: Some(SubmarinePattern::OffBases),
            expects_boats_off_own_bases: true,
        }
    );
    // A side given neither has no pattern and expects nothing.
    let none = Game::L6.scenario();
    for side in [Side::Allied, Side::Japanese] {
        assert_eq!(
            none.world.side_view(side).submarine_doctrine(),
            SubmarineDoctrine::default()
        );
        assert_eq!(SubmarineDoctrine::default().pattern, None);
        assert!(!SubmarineDoctrine::default().expects_boats_off_own_bases);
    }
    // Either may be given alone.
    let only_expects =
        common::replace_once(&Game::L1.text(), "submarine_pattern = \"lines\"\n", "");
    let only_expects = common::replace_once(
        &only_expects,
        "expects_boats_off_own_bases = false",
        "expects_boats_off_own_bases = true",
    );
    let scenario = common::from_text(&only_expects, &Game::L1.path()).expect("it loads");
    assert_eq!(
        scenario
            .world
            .side_view(Side::Japanese)
            .submarine_doctrine(),
        SubmarineDoctrine {
            pattern: None,
            expects_boats_off_own_bases: true,
        }
    );
    let only_pattern =
        common::replace_once(&Game::B1.text(), "expects_boats_off_own_bases = true\n", "");
    let scenario = common::from_text(&only_pattern, &Game::B1.path()).expect("it loads");
    assert_eq!(
        scenario.world.side_view(Side::Allied).submarine_doctrine(),
        SubmarineDoctrine {
            pattern: Some(SubmarinePattern::OffBases),
            expects_boats_off_own_bases: false,
        }
    );
}

#[test]
fn statement_02_any_other_word_for_the_pattern_is_refused_naming_the_side_and_the_word() {
    for (game, side, word) in [
        (Game::L1, "japanese", "lines"),
        (Game::B1, "allied", "off_bases"),
    ] {
        let text = common::replace_once(
            &game.text(),
            &format!("submarine_pattern = \"{word}\""),
            "submarine_pattern = \"in_rows\"",
        );
        let Err(refusal) = common::from_text(&text, &game.path()) else {
            panic!("the word in_rows was taken for the {side} side");
        };
        let message = refusal.to_string();
        assert!(message.to_lowercase().contains(side), "{message}");
        assert!(message.contains("in_rows"), "{message}");
    }
}

#[test]
fn statement_03_a_sides_own_picture_holds_its_pattern_and_no_answer_to_the_other_seat_does() {
    let mut s = session(Game::L1);
    // The side's own: its briefing says its pattern and what it expects.
    let own = ask(&mut s, Side::Japanese, "get_briefing", json!({}));
    let doctrine = &own["submarines"]["doctrine"];
    assert_eq!(doctrine["submarine_pattern"], "lines", "{own}");
    assert_eq!(doctrine["expects_boats_off_own_bases"], false, "{own}");
    assert!(doctrine["words"].as_str().is_some_and(|w| !w.is_empty()));
    // The other side's picture holds neither of the Japanese side's.
    let world = Game::L1.scenario().world;
    assert_eq!(
        world.side_view(Side::Allied).submarine_doctrine(),
        SubmarineDoctrine::default()
    );
    // And no answer of any tool the Allied seat has says either.
    for tool in s.tools(Side::Allied) {
        let answer = match s.call(Side::Allied, tool, &json!({})) {
            Ok(answer) => answer.to_string(),
            Err(e) => e.message,
        };
        for word in [
            "submarine_pattern",
            "expects_boats_off_own_bases",
            "doctrine",
        ] {
            assert!(
                !answer.contains(word),
                "the Allied seat's {tool} says {word}: {answer}"
            );
        }
    }
}

#[test]
fn statement_04_a_scenario_with_no_pattern_plays_as_it_did_and_scenario_129s_stations_are_what_they_were(
) {
    // Scenario 129's game A1: the table of its Part A, as its own test
    // states it. The comparison of every run with the hashes cut is made by
    // `scripts/compare-scenarios.sh` and reported with the pull request.
    let path = common::data_dir().join("scenarios/129_submarines_the_staff_and_the_patrol.toml");
    let scenario = common::load(&path).expect("scenario 129 loads");
    for side in [Side::Allied, Side::Japanese] {
        assert_eq!(
            scenario.world.side_view(side).submarine_doctrine(),
            SubmarineDoctrine::default()
        );
    }
    let played = play(scenario, Side::Japanese);
    played.nothing_refused();
    assert_eq!(
        played.stations(1),
        vec![
            (1, hex(5, 0)),
            (2, hex(3, 0)),
            (3, hex(3, 2)),
            (4, hex(5, -2)),
            (5, hex(5, 2)),
            (6, hex(7, -2)),
            (7, hex(7, 0)),
        ]
    );
}

// ---------------------------------------------------------------------
// Stage A: lines.
// ---------------------------------------------------------------------

const L1: [(i32, i32); 9] = [
    (0, 0),
    (-1, 1),
    (0, -1),
    (-1, 2),
    (1, -2),
    (-2, 3),
    (1, -3),
    (-3, 0),
    (2, 1),
];

#[test]
fn statement_05_l1_the_boats_are_stationed_on_the_line_across_the_approach_from_8_0() {
    let played = stationed(Game::L1, &L1);
    assert!(
        played
            .words(1)
            .contains("The approach is from Placeholder Allied base 2 (base:2) at (8,0)"),
        "{}",
        played.words(1)
    );
}

#[test]
fn statement_06_l1_the_first_seven_reasons_say_the_first_line_from_the_base_the_last_two_the_spread(
) {
    let played = Game::L1.play();
    for boat in 1..=7 {
        let words = played.words(boat);
        assert!(
            words
                .contains("first line across the approach from Placeholder Allied base 2 (base:2)"),
            "boat {boat}: {words}"
        );
    }
    for boat in 8..=9 {
        let words = played.words(boat);
        assert!(
            words.contains("spread over the waters, the line being full"),
            "boat {boat}: {words}"
        );
        assert!(
            !words.contains("is on the first line"),
            "boat {boat}: {words}"
        );
    }
}

#[test]
fn statement_07_l2_fifteen_boats_two_lines_and_the_last_two_on_the_second() {
    let played = stationed(
        Game::L2,
        &[
            (0, 0),
            (-1, 1),
            (0, -1),
            (-1, 2),
            (1, -2),
            (-2, 3),
            (1, -3),
            (-2, 4),
            (2, -4),
            (-3, 5),
            (2, -5),
            (-3, 6),
            (3, -6),
            (-5, 0),
            (-6, 1),
        ],
    );
    for boat in 14..=15 {
        let words = played.words(boat);
        assert!(
            words.contains("second line across the approach"),
            "boat {boat}: {words}"
        );
    }
    for boat in 1..=13 {
        let words = played.words(boat);
        assert!(
            words.contains("first line across the approach"),
            "boat {boat}: {words}"
        );
    }
}

#[test]
fn statement_08_l3_the_line_steps_over_the_land() {
    let played = stationed(
        Game::L3,
        &[(0, 0), (-1, 2), (0, -1), (-2, 3), (1, -2), (1, -3)],
    );
    assert!(played.stations(1).iter().all(|(_, at)| *at != hex(-1, 1)));
}

#[test]
fn statement_09_l4_no_enemy_base_beyond_5_hexes_no_line_and_the_reason_says_why() {
    let played = stationed(Game::L4, &[(0, 0), (-3, 0), (-3, 3)]);
    for boat in 1..=3 {
        let words = played.words(boat);
        assert!(words.contains("There is no line"), "boat {boat}: {words}");
        assert!(
            words
                .contains("no enemy base on the side's map lies more than 5 hexes from the centre"),
            "boat {boat}: {words}"
        );
        assert!(
            words.contains("Placeholder Allied base 2 (base:2) at (4,0), is 4 hexes from it"),
            "boat {boat}: {words}"
        );
    }
}

#[test]
fn statement_10_l5_the_approach_is_from_the_nearer_base_beyond_5_hexes() {
    let played = stationed(Game::L5, &[(0, 0), (1, -1), (-1, 0)]);
    for boat in 1..=3 {
        let words = played.words(boat);
        assert!(
            words.contains(
                "first line across the approach from Placeholder Allied base 3 (base:3) at (0,-7)"
            ),
            "boat {boat}: {words}"
        );
    }
}

#[test]
fn statement_11_l6_with_no_pattern_the_boats_are_spread_as_decision_0218_spreads_them() {
    let played = stationed(Game::L6, &[(0, 0), (-3, 0), (-3, 3)]);
    // The words of decision 0218, unchanged.
    assert!(played.words(1).contains("the centre of the waters"));
    assert!(played
        .words(2)
        .contains("station 2 of the waters, 3 hexes from the nearest before it"));
    for boat in 1..=3 {
        let words = played.words(boat);
        assert!(!words.contains("line"), "boat {boat}: {words}");
        assert!(
            !words.contains("spread over the waters"),
            "boat {boat}: {words}"
        );
    }
}

#[test]
fn statement_12_l1_the_second_morning_no_boat_is_ordered_and_each_keeps_her_station() {
    let played = Game::L1.play();
    assert_eq!(played.stations(1), of(&L1));
    assert!(played.stations(2).is_empty());
    let words = played.no_order_text(2);
    assert_eq!(words.len(), 1, "one decision to give no order: {words:?}");
    for (boat, at) in of(&L1) {
        assert!(
            words[0].contains(&format!(
                "submarine:{boat} (Placeholder submarine {boat}) keeps her station at {at}"
            )),
            "{}",
            words[0]
        );
    }
    assert!(played.stations(3).is_empty());
}

// ---------------------------------------------------------------------
// Stage A: off bases.
// ---------------------------------------------------------------------

#[test]
fn statement_13_b1_the_boats_are_stationed_off_the_bases_then_spread() {
    stationed(
        Game::B1,
        &[
            (-7, 0),
            (-6, 2),
            (-3, 0),
            (-8, 1),
            (-7, 3),
            (-6, -3),
            (-4, -2),
        ],
    );
}

#[test]
fn statement_14_b1_each_reason_names_its_base_or_the_spread_and_base_3_has_one_station_and_why() {
    let played = Game::B1.play();
    for (boat, base) in [(1, 1), (4, 1), (2, 2), (5, 2), (3, 3)] {
        let words = played.words(boat);
        assert!(
            words.contains(&format!(
                "off Placeholder Japanese base {base} (base:{base})"
            )),
            "boat {boat}: {words}"
        );
    }
    for boat in [6, 7] {
        let words = played.words(boat);
        assert!(
            words.contains("spread over the waters"),
            "boat {boat}: {words}"
        );
    }
    // Base 3 is given one station and not two, and the words say why.
    let words = played.words(3);
    assert!(
        words.contains(
            "Placeholder Japanese base 3 (base:3) at (-2,0) is given 1 station(s) and not 2: \
             no second hex next to it that ships can enter lies in the waters"
        ),
        "{words}"
    );
}

#[test]
fn statement_15_b2_base_3_has_no_hex_in_the_waters_and_is_passed_over() {
    let played = stationed(Game::B2, &[(-7, 0), (-6, 2), (-8, 1), (-7, 3)]);
    let words = played.words(1);
    assert!(
        words.contains(
            "Placeholder Japanese base 3 (base:3) at (-2,0) is passed over: \
             no hex next to it that ships can enter lies in the waters"
        ),
        "{words}"
    );
}

#[test]
fn statement_16_b3_no_enemy_base_in_or_next_to_the_waters_the_boats_are_spread() {
    let played = stationed(Game::B3, &[(4, -4), (2, -4)]);
    for boat in 1..=2 {
        let words = played.words(boat);
        assert!(
            words.contains(
                "No enemy base on the side's map lies in the waters or within a hex of them"
            ),
            "boat {boat}: {words}"
        );
    }
}

#[test]
fn statement_17_in_every_game_no_station_is_on_land_on_a_base_or_outside_and_none_is_shared() {
    for game in [
        Game::L1,
        Game::L2,
        Game::L3,
        Game::L4,
        Game::L5,
        Game::L6,
        Game::B1,
        Game::B2,
        Game::B3,
    ] {
        let played = game.play();
        assert!(!played.stations(1).is_empty(), "{}", game.file());
        sound(game, &played);
    }
}
