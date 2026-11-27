//! Scenario 103: the Eastern Solomons over five days, 23 to 27 August
//! 1942. Three Japanese carriers against two American and an airfield,
//! with a Japanese convoy that must reach Guadalcanal. It has no scripted
//! order: each side is played by a commander or a staff.
//!
//! Proves decision 0184 as amended and confirmed. The scenario comes as
//! four files made by one script from one base, in
//! `data/scenarios/five-days/`, which is kept out of the loops that walk
//! `data/scenarios/*.toml` (decision 0072):
//!
//! - `01a_eastern_solomons.toml`: both sides given the orders below;
//! - `01b_eastern_solomons.toml`: the Japanese side given its orders, the
//!   Allied side none (a person or an agent takes the Allied seat);
//! - `01c_eastern_solomons.toml`: the Allied side given its orders, the
//!   Japanese side none;
//! - `01d_eastern_solomons.toml`: neither side given orders.
//!
//! They differ in their `[[command]]` tables and their names and in
//! nothing else. It runs on the live figures.
//!
//! # Hand calculation
//!
//! Written by the architect before the scenario was built. Everything in
//! this section is asserted exactly. If the code or the file disagrees,
//! one of them or the rule is wrong: do not change a number here to make
//! a test pass. A mismatch goes back to the architect.
//!
//! What the staffs then do with it, day by day, is not worked here: it
//! is reported by the build for the architect to state (see "Reported").
//!
//! ## The map's arithmetic
//!
//! From the header of `data/map/south_pacific.csv`: east in nm is
//! (longitude - 160) x 60 x 0.978148; north in nm is (latitude + 12) x
//! 60, south latitude negative. The centre of hex (q, r) is 40 x (q +
//! r/2) east and 34.641016 x r south. So r = -north / 34.641016 and q =
//! east / 40 - r / 2, rounded to the nearest hex. The distance in the air
//! between two hexes is 40 x the square root of (dq x dq + dr x dr + dq x
//! dr), rounded to a whole mile.
//!
//! ## Where each force starts, and why
//!
//! The game begins at 1800 on 22 August; day 1 is 23 August. No row
//! places any force on the map on the 22nd. Each force starts where a row
//! first places it on the 23rd, or where scenario 5 has it; every start
//! is a placeholder for the evening before, and is marked so in the file
//! with its row.
//!
//! | Force | Hex | From |
//! | --- | --- | --- |
//! | Task forces 1 and 2, the American carriers | (6,-3) | position row 262: Task Force 61 at 10 S, 163 E at 1040 on the 23rd |
//! | Task force 3, the carrier striking force | (9,-18) | position row 413: 1.8 S, 160.2 E at 0600 on the 23rd, read off a chart, of low confidence |
//! | Task force 4, the Ryujo detachment | (7,-18) | scenario 5's own start for it |
//! | Task force 5, the convoy | (6,-12) | position row 291: 250 miles north of Guadalcanal at 1040 on the 23rd, taken as due north of Lunga Point (9.42806 S, 160.05472 E). Rows 294 and 352 say 200 and 350 miles |
//!
//! The arithmetic:
//!
//! - **10 S, 163 E:** east 3 x 60 x 0.978148 = 176.07; north 2 x 60 =
//!   120; r = -3.46, q = 4.40 + 1.73 = 6.13: hex (6,-3).
//! - **1.8 S, 160.2 E:** east 0.2 x 60 x 0.978148 = 11.74; north 10.2 x
//!   60 = 612; r = -17.67, q = 0.29 + 8.83 = 9.13: hex (9,-18).
//! - **250 nm north of Lunga Point:** latitude -9.42806 + 250/60 =
//!   -5.2614; east 0.05472 x 60 x 0.978148 = 3.21; north 6.7386 x 60 =
//!   404.32; r = -11.67, q = 0.08 + 5.84 = 5.92: hex (6,-12).
//!
//! The places are the 39 days': Lunga, base 1, at (2,-4); Taivu at
//! (3,-4); Tulagi at (3,-5); Rabaul at (-5,-13).
//!
//! ## Distances, in the air
//!
//! | From | To | dq, dr | Miles |
//! | --- | --- | --- | --- |
//! | The convoy, (6,-12) | Taivu, (3,-4) | -3, 8 | 40 x root 49 = 280 |
//! | Lunga, (2,-4) | the convoy's start | 4, -8 | 40 x root 48 = 277 |
//! | The American carriers, (6,-3) | Lunga | -4, -1 | 40 x root 21 = 183 |
//! | The American carriers | Taivu | -3, -1 | 40 x root 13 = 144 |
//! | The American carriers | the convoy's start | 0, -9 | 360 |
//! | The American carriers | the carrier striking force, (9,-18) | 3, -15 | 40 x root 189 = 550 |
//! | The carrier striking force | the convoy's start | -3, 6 | 40 x root 27 = 208 |
//! | Rabaul, (-5,-13) | Lunga | 7, 9 | 40 x root 193 = 556 |
//!
//! ## What can reach what on the first evening
//!
//! With the live aircraft figures (search and strike radius in miles:
//! SBD-3 397 and 200; TBF-1 420 and 190; F4F-4 105 and 105; G4M1 611 and
//! 611; A6M2 from a base 556 and 556; from a carrier 261 and 261; D3A1
//! 252 and 252; B5N2 358 and 175):
//!
//! - **Lunga's dive bombers** can search to the convoy's start (277 is
//!   within 397) and cannot strike it there (277 is beyond 200).
//! - **The American carriers** can search to the convoy's start (360 is
//!   within 397 and 420) and can strike a ship off Lunga or Taivu (183
//!   and 144 are within 200 and 190). They can neither search nor strike
//!   to the carrier striking force (550).
//! - **Rabaul's bombers and fighters** can reach Lunga (556 is within
//!   611, and is the fighters' radius exactly).
//! - **Neither side's carriers can strike the other's** on the first
//!   evening.
//!
//! ## The convoy
//!
//! Task force 5, on the transport mission. Its men are the reading of
//! troop lift rows 8 and 9: 700 army and 800 naval, 1,500 in all (row 7
//! agrees on the total; the ships' own rows give about 1,100 and about
//! 800, which the file notes and does not use).
//!
//! - **Three placeholder troop transports** (the 39 days' class 10: 10 kn
//!   cruising, 12 kn at full speed, 600 men), named "Placeholder
//!   transport" 1 to 3. No real transport is given a rating.
//! - **Four destroyers the rows name with the convoy and the game rates:**
//!   Kagero, Kawakaze, Umikaze and Suzukaze. Jintsu, the four patrol
//!   boats, Isokaze, Mutsuki and Yayoi are left out, and the file says
//!   so.
//! - **Three land units aboard,** fighting units: 350 and 350 men of the
//!   Ichiki Detachment's second echelon on transports 1 and 2, and 800 of
//!   the Yokosuka 5th Special Naval Landing Force: 600 on transport 3,
//!   which has 600 berths, and the rest on the destroyers in the force's
//!   order, 152 on Kagero and 48 on Kawakaze. (Corrected by the architect
//!   on Nov 20, 2026, on the build's report: this put all 800 on
//!   transport 3, which cannot carry them.)
//! - **Supply aboard:** 30 t on each transport, 90 t, a placeholder (no
//!   row gives a figure): 1,500 men x 0.005 t x 12 days.
//!
//! The force's speed is its slowest ship's: 10 kn cruising, 12 at full
//! speed. The passage to Taivu is 280 miles: 28 hours at 10 kn, 23 hours
//! 20 minutes at 12.
//!
//! **Landing where there is no port.** A hex has port size 0: lighters at
//! 20 t an hour, and for men landing unprepared 0.13 of that, 2.6 t an
//! hour for each ship. A man is 0.02 t, so a ship lands 130 men an hour.
//! Transports 1 and 2 land their 350 men in 350 / 130 hours, 2 hours 42
//! minutes; transport 3 its 600 in 4 hours 37 minutes.
//!
//! ## The rest of the forces
//!
//! - **Task forces 1 to 4** are scenario 5's, ship for ship: 10, 8, 14
//!   and 4 ships. Their air groups are scenario 5's, group for group,
//!   each at the strength scenario 5 gives it.
//! - **Lunga,** base 1, Allied, with its airfield. Its squadrons are at
//!   the strength of the rows of 20 August that the 39 days uses: 19
//!   fighters of VMF-223 and 12 dive bombers of VMSB-232.
//! - **The Marines on Guadalcanal** are the 39 days' four land units
//!   there, 4,306, 4,561, 1,373 and 660 men, 10,900 in all, ashore at
//!   Lunga on the defence.
//! - **Lunga's supply** is 1,000 t, a placeholder: no row gives the stock
//!   on the 22nd. The men alone eat 10,900 x 0.005 = 54.5 t a day, 272.5
//!   t in five days, so the stock does not run out in the game by their
//!   eating.
//! - **Taivu** is a Japanese stock with no supply and one land unit of
//!   100 men, from land unit row 270 ("little more than 100", a floor),
//!   on the defence.
//! - **Rabaul:** the land bombers and the fighters the 39 days has there,
//!   at its strengths, and its supply.
//! - **Tulagi** is Allied, with the 39 days' units there, and is given
//!   no part.
//!
//! ## What a side the engine plays is given
//!
//! On day 1, as commands of the scenario:
//!
//! - **Japanese:** hold Taivu, its forces the convoy and its three land
//!   units, with Rabaul (base 2) the source the convoy returns to;
//!   neutralise Lunga, its forces the bombers and the fighters at Rabaul
//!   (groups 27 and 28); operate
//!   task force 3 from (8,-11) and task force 4 from (7,-9), each with
//!   its own air groups; leave to prepare and leave to launch given.
//! - **Allied:** hold Lunga, its forces the four land units and the two
//!   squadrons; operate task force 1 from (6,-3) and task force 2 from
//!   (7,-3), each with its own air groups; deny the waters within 2
//!   hexes of (3,-7), north of Guadalcanal, with no forces limit; leave to
//!   prepare and leave to launch given.
//!
//! No risk is set: every order is at balanced.
//!
//! These are the second version of the orders (decision 0184, "The second
//! version of the orders"). The first left out the source, the fighters
//! and the deny; the first run's report keeps what it gave.
//!
//! ## The objectives
//!
//! A scale of the scenario's own: it is a scenario for play (decision
//! 0187), and its file says so. Every figure but the last is the one
//! scenarios 5 and 7 give the kind; the last, men landed, is the figure of
//! men lost used again (decision 0186). Levels: minor 3, major 8, decisive
//! 16. A horizon of 30 days.
//!
//! | Kind | Allied | Japanese |
//! | --- | --- | --- |
//! | Ships sunk: carrier 10, battleship 8, cruiser 4, destroyer 1, transport 2, auxiliary 1 | yes | yes |
//! | Ships out of action, over the horizon: carrier 5, battleship 4, cruiser 2, destroyer 0.5, transport 1, auxiliary 0.5 | yes | yes |
//! | Aircraft destroyed, 0.05 each | yes | yes |
//! | Enemy men lost, 1 for each 100 | yes | yes |
//! | Lunga open, 10 points over the days | open | shut |
//! | Supply landed at Taivu, 0.005 a ton | | yes |
//! | Men landed at Taivu, 1 for each 100, a placeholder | | yes |
//!
//! # Properties
//!
//! - **Each file loads and plays five days,** and the same seed gives
//!   the same daily hashes twice.
//! - **The four files are one scenario:** with their `[[command]]`
//!   tables and names taken away they are the same, byte for byte; and
//!   the script that makes them, asked to check, says they are what it
//!   makes.
//! - **The forces are scenario 5's:** every ship of task forces 1 to 4
//!   and every air group of theirs has the class, the type and the
//!   strength scenario 5 gives it.
//! - **Every start is where the calculation puts it,** and every
//!   distance and reach above is what the game's own measure gives.
//! - **Nothing waits for a later day:** no unit has a day from which it
//!   may be used.
//! - **A side given no orders does nothing of its own:** in file d, with
//!   both seats the engine's, no order is given by either staff in five
//!   days.
//! - **A side given its orders acts on the first morning:** in file a,
//!   on day 1 the Japanese staff gives orders to the convoy and to the
//!   bombers at Rabaul, and each side's staff to its carrier forces.
//! - **It can be played from a seat:** file b with the Allied seat
//!   played and file c with the Japanese seat played each open, list
//!   the tools of command, and answer `trial`.
//! - **Scenario 5 is untouched:** its file, its frozen copy and its
//!   variants are what they were.
//!
//! # Reported, for the architect to state
//!
//! Not asserted here; given in the build's report and in
//! `docs/validation/`:
//!
//! - In file a on the scenario's seed, day by day: when the convoy
//!   sails, when and where it lands and how many men and tons, whether it
//!   is found and struck and by whom; what the bombers at Rabaul do; where
//!   each carrier force goes and whether the carriers find and strike one
//!   another; Lunga's supply and whether it is open each day; and the
//!   result.
//! - **The first run:** 1,000 games of file a, staff against staff: how
//!   often each side wins and at which level, and the spread of each
//!   count behind the points (carriers sunk and out of action each side,
//!   transports sunk, men and tons landed at Taivu, men lost each side,
//!   aircraft lost each side, days Lunga was open).

// The tests are written by the build, to the statements above. Each names the
// statement it proves. Everything here runs on the live figures: no number
// asserted is one the step 1 freeze holds (the doc comment, "It runs on the
// live figures").

use crate::common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::OnceLock;

use serde_json::{json, Value};
use sim_agent::SeatKind;
use sim_core::command::{CommandState, IntentKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{
    AirGroupId, BaseId, EntityRef, IntentId, LandUnitId, ShipClassId, ShipId, TaskForceId,
};
use sim_core::orders::{Issuer, Mission, Order, Posture};
use sim_core::state::{AirGroupLocation, ShipLocation, Side, TruthState};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

const A: Side = Side::Allied;
const J: Side = Side::Japanese;
const LETTERS: [char; 4] = ['a', 'b', 'c', 'd'];

// ------------------------------------------------------------------ helpers

fn path(letter: char) -> PathBuf {
    common::data_dir().join(format!(
        "scenarios/five-days/01{letter}_eastern_solomons.toml"
    ))
}

/// The file resolved (decision 0234): a key, a row or a table is read from
/// the scenario it makes, whether the file holds it or the file it is built on.
fn text(letter: char) -> String {
    common::resolved_text(&path(letter))
}

/// The header of the file and of every file it is built on, joined.
fn headers(letter: char) -> String {
    common::headers(&path(letter))
}

/// Loaded on the live figures: the scenario runs on them (the doc comment).
fn load(letter: char) -> Scenario {
    common::load_live(&path(letter)).unwrap_or_else(|e| panic!("file {letter} loads: {e}"))
}

fn start(letter: char) -> TruthState {
    load(letter).world.umpire().clone()
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn hex(q: i32, r: i32) -> Hex {
    Hex::new(q, r)
}

fn tf(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn group(raw: u32) -> AirGroupId {
    AirGroupId::new(raw)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

fn unit(raw: u32) -> LandUnitId {
    LandUnitId::new(raw)
}

/// A game played to its end with the doctrine's staffs, as `run` and `batch`
/// play it: the daily hashes, every order given, the events, the command
/// state of each side after day 1, and the world at the end.
struct Played {
    hashes: Vec<u64>,
    orders: Vec<Order>,
    after_day_1: BTreeMap<Side, CommandState>,
}

fn play(letter: char) -> Played {
    let mut scenario = load(letter);
    let held = staffs();
    let mut hashes = Vec::new();
    let mut after_day_1 = BTreeMap::new();
    while let Some(summary) = scenario
        .next_day_watched(as_staffs(&held), &mut |_, _| {})
        .expect("a day resolves")
    {
        hashes.push(summary.state_hash);
        if summary.day == 1 {
            for side in Side::ALL {
                after_day_1.insert(side, scenario.world.command_state(side).clone());
            }
        }
    }
    Played {
        hashes,
        orders: scenario.world.orders().cloned().collect(),
        after_day_1,
    }
}

/// Each file played once, kept for the tests that read a game.
fn played(letter: char) -> &'static Played {
    static GAMES: OnceLock<BTreeMap<char, Played>> = OnceLock::new();
    &GAMES.get_or_init(|| LETTERS.iter().map(|l| (*l, play(*l))).collect())[&letter]
}

/// A session opened on a file as `serve` opens one, with `players` held by
/// players and the other seat the script's.
fn session(letter: char, players: &[Side]) -> sim_agent::Session {
    let seats: BTreeMap<Side, SeatKind> = players.iter().map(|s| (*s, SeatKind::Player)).collect();
    sim_cli::game::open(
        path(letter).to_str().expect("a path in words"),
        None,
        seats,
        &BTreeMap::new(),
    )
    .unwrap_or_else(|e| panic!("file {letter} opens: {e}"))
    .0
}

/// Every object in `value` with a `handle` and a `name`, as (name, handle).
fn handles(value: &Value, out: &mut BTreeMap<String, String>) {
    match value {
        Value::Object(map) => {
            if let (Some(Value::String(h)), Some(Value::String(name))) =
                (map.get("handle"), map.get("name"))
            {
                out.entry(name.clone()).or_insert_with(|| h.clone());
            }
            map.values().for_each(|v| handles(v, out));
        }
        Value::Array(items) => items.iter().for_each(|v| handles(v, out)),
        _ => {}
    }
}

/// The handle a side's own tools give its unit of `name`, of the `kind`
/// (`task_force`, `air_group`, `base`, `land_unit`).
fn handle(s: &mut sim_agent::Session, side: Side, kind: &str, name: &str) -> String {
    let mut found = BTreeMap::new();
    for tool in ["get_briefing", "list_forces", "list_bases"] {
        let answer = s.call(side, tool, &json!({})).expect("a list");
        handles(&answer, &mut found);
    }
    let h = found
        .get(name)
        .unwrap_or_else(|| panic!("{name} is listed for the side"))
        .clone();
    assert!(h.starts_with(&format!("{kind}:")), "{name}: {h}");
    h
}

fn measure(s: &mut sim_agent::Session, side: Side, question: Value) -> Value {
    let a = s.call(side, "measure", &question).expect("measure answers");
    assert_eq!(a["kind"], "measure", "{question}: {a}");
    a
}

fn place(q: i32, r: i32) -> Value {
    json!({"q": q, "r": r})
}

fn air_distance(s: &mut sim_agent::Session, from: (i32, i32), to: (i32, i32)) -> u32 {
    let a = measure(
        s,
        A,
        json!({"question": "air_distance", "from": place(from.0, from.1), "to": place(to.0, to.1)}),
    );
    a["distance_nm"].as_u64().expect("miles") as u32
}

/// The air reach of a side's group to each place: (miles, within the
/// group's radius).
fn reach(
    s: &mut sim_agent::Session,
    side: Side,
    group_name: &str,
    places: &[(i32, i32)],
) -> (Fixed, Vec<(u32, bool)>) {
    let h = handle(s, side, "air_group", group_name);
    let to: Vec<Value> = places.iter().map(|(q, r)| place(*q, *r)).collect();
    let a = measure(
        s,
        side,
        json!({"question": "air_reach", "air_group": h, "to": to}),
    );
    let radius: Fixed = a["radius_nm"].as_str().expect("a radius").parse().unwrap();
    let lines = a["places"]
        .as_array()
        .expect("places")
        .iter()
        .map(|l| {
            (
                l["distance_nm"].as_u64().expect("miles") as u32,
                l["within_radius"].as_bool().expect("within"),
            )
        })
        .collect();
    (radius, lines)
}

/// The aircraft type's search and strike radius, from the figures loaded.
fn radii(world: &sim_core::World, type_raw: u32) -> (Fixed, Fixed) {
    let t = &world.ref_data().aircraft_types[&sim_core::ids::AircraftTypeId::new(type_raw)];
    (t.search_range_nm, t.strike_range_nm)
}

/// A stable digest of bytes, FNV-1a of 64 bits, for the files that must not
/// change.
fn digest(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

// ------------------------------------------------- The map's arithmetic

/// The map's arithmetic: each latitude and longitude of the table, put into
/// the map tool's own projection, lands in the hex the calculation gives, by
/// the numbers it works.
#[test]
fn the_map_arithmetic_puts_each_row_in_its_hex() {
    // The projection of the map the scenario is on, from the map's header
    // (decision 0199): the present map's.
    let projection = map_tool::frame::Frame::of_map(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../data/map/south_pacific.csv"),
    )
    .unwrap()
    .expect("the map has a frame")
    .projection();
    let to_nm = |lat: f64, lon: f64| projection.to_nm(lat, lon);
    let hex_of = |lat: f64, lon: f64| projection.hex_of(lat, lon);
    let near = |a: f64, b: f64| (a - b).abs() < 0.01;
    // 10 S, 163 E: east 176.07, north 120; r -3.46, q 6.13.
    let (e, nn) = to_nm(-10.0, 163.0);
    assert!(near(e, 176.07) && near(nn, 120.0), "{e} {nn}");
    let r = -nn / 34.641016;
    let q = e / 40.0 - r / 2.0;
    assert!(
        (r - -3.46).abs() < 0.005 && (q - 6.13).abs() < 0.005,
        "{q} {r}"
    );
    assert_eq!(hex_of(-10.0, 163.0), (6, -3));
    // 1.8 S, 160.2 E: east 11.74, north 612; r -17.67, q 9.13.
    let (e, nn) = to_nm(-1.8, 160.2);
    assert!(near(e, 11.74) && near(nn, 612.0), "{e} {nn}");
    let r = -nn / 34.641016;
    let q = e / 40.0 - r / 2.0;
    assert!(
        (r - -17.67).abs() < 0.005 && (q - 9.13).abs() < 0.005,
        "{q} {r}"
    );
    assert_eq!(hex_of(-1.8, 160.2), (9, -18));
    // 250 nm due north of Lunga Point, 9.42806 S, 160.05472 E: latitude
    // -5.2614; east 3.21, north 404.32; r -11.67, q 5.92.
    let lat: f64 = -9.42806 + 250.0 / 60.0;
    assert!((lat - -5.2614).abs() < 0.0001, "{lat}");
    let (e, nn) = to_nm(lat, 160.05472);
    assert!(near(e, 3.21) && near(nn, 404.32), "{e} {nn}");
    let r = -nn / 34.641016;
    let q = e / 40.0 - r / 2.0;
    assert!(
        (r - -11.67).abs() < 0.005 && (q - 5.92).abs() < 0.005,
        "{q} {r}"
    );
    assert_eq!(hex_of(lat, 160.05472), (6, -12));
}

/// Where each force starts, in every file, and the places, the 39 days'.
#[test]
fn every_start_is_where_the_calculation_puts_it() {
    for letter in LETTERS {
        let t = start(letter);
        for (raw, at) in [
            (1, (6, -3)),
            (2, (6, -3)),
            (3, (9, -18)),
            (4, (7, -18)),
            (5, (6, -12)),
        ] {
            assert_eq!(
                t.task_forces[&tf(raw)].position(),
                hex(at.0, at.1),
                "file {letter}, task force {raw}"
            );
        }
        for (raw, at, side) in [
            (1, (2, -4), A),
            (5, (3, -4), J),
            (4, (3, -5), A),
            (2, (-5, -13), J),
        ] {
            let b = &t.bases[&base(raw)];
            assert_eq!((b.hex, b.side), (hex(at.0, at.1), side), "base {raw}");
        }
        assert_eq!(t.bases.len(), 4, "Lunga, Rabaul, Tulagi and Taivu only");
    }
    // The places are the 39 days'.
    let slice = common::load_live(&common::data_dir().join("scenarios/07_full_slice.toml"))
        .expect("the 39 days load");
    for raw in [1, 2, 4, 5] {
        assert_eq!(
            slice.world.umpire().bases[&base(raw)].hex,
            start('a').bases[&base(raw)].hex,
            "base {raw}"
        );
    }
}

// ------------------------------------------------------ Distances, in the air

#[test]
fn the_distances_in_the_air_are_the_tables_by_the_games_own_measure() {
    let mut s = session('d', &[A]);
    for (from, to, dq_dr, miles) in [
        ((6, -12), (3, -4), (-3, 8), 280),
        ((2, -4), (6, -12), (4, -8), 277),
        ((6, -3), (2, -4), (-4, -1), 183),
        ((6, -3), (3, -4), (-3, -1), 144),
        ((6, -3), (6, -12), (0, -9), 360),
        ((6, -3), (9, -18), (3, -15), 550),
        ((9, -18), (6, -12), (-3, 6), 208),
        ((-5, -13), (2, -4), (7, 9), 556),
    ] {
        let (dq, dr): (i32, i32) = dq_dr;
        assert_eq!((to.0 - from.0, to.1 - from.1), (dq, dr), "{from:?} {to:?}");
        // 40 x the root of (dq dq + dr dr + dq dr), to a whole mile.
        let worked = (40.0 * f64::from(dq * dq + dr * dr + dq * dr).sqrt()).round() as u32;
        assert_eq!(worked, miles, "{from:?} to {to:?} worked");
        assert_eq!(air_distance(&mut s, from, to), miles, "{from:?} to {to:?}");
        assert_eq!(hex(from.0, from.1).air_distance_nm(hex(to.0, to.1)), miles);
    }
}

// ------------------------------------- What can reach what on the first evening

#[test]
fn the_live_aircraft_figures_are_the_calculations() {
    let w = load('d').world;
    for (raw, search, strike) in [
        (2001, 397, 200), // SBD-3
        (2101, 397, 200),
        (2002, 420, 190), // TBF-1
        (2000, 105, 105), // F4F-4
        (2100, 105, 105),
        (2103, 611, 611), // G4M1
        (2102, 556, 556), // A6M2 from a base
        (2003, 261, 261), // A6M2 from a carrier
        (2004, 252, 252), // D3A1
        (2005, 358, 175), // B5N2
    ] {
        assert_eq!(radii(&w, raw), (n(search), n(strike)), "type {raw}");
    }
}

#[test]
fn lungas_dive_bombers_can_search_to_the_convoy_and_not_strike_it() {
    let w = load('d').world;
    let mut s = session('d', &[A]);
    let (radius, lines) = reach(&mut s, A, "VMSB-232", &[(6, -12)]);
    assert_eq!(radius, n(200));
    assert_eq!(lines, vec![(277, false)], "beyond 200: no strike");
    let (search, _) = radii(&w, 2101);
    assert!(n(277) <= search, "277 is within 397");
}

#[test]
fn the_american_carriers_search_to_the_convoy_and_strike_off_lunga_and_taivu() {
    let w = load('d').world;
    let mut s = session('d', &[A]);
    // Enterprise's dive bombers (VB-6, SBD-3) and torpedo planes (VT-3,
    // TBF-1), from (6,-3).
    for (name, type_raw, radius) in [("VB-6", 2001, 200), ("VT-3", 2002, 190)] {
        let (r, lines) = reach(&mut s, A, name, &[(2, -4), (3, -4), (6, -12), (9, -18)]);
        assert_eq!(r, n(radius), "{name}");
        assert_eq!(
            lines,
            vec![(183, true), (144, true), (360, false), (550, false)],
            "{name}"
        );
        let (search, _) = radii(&w, type_raw);
        assert!(n(360) <= search, "{name} searches to the convoy's start");
        assert!(
            n(550) > search,
            "{name} cannot search to the striking force"
        );
    }
}

#[test]
fn rabauls_bombers_and_fighters_reach_lunga() {
    let mut s = session('d', &[J]);
    for (name, radius) in [
        (
            "Land attack bombers at Rabaul (4th, Misawa and Kisarazu Air Groups)",
            611,
        ),
        ("Tainan Air Group", 556),
    ] {
        let (r, lines) = reach(&mut s, J, name, &[(2, -4)]);
        assert_eq!(r, n(radius), "{name}");
        assert_eq!(lines, vec![(556, true)], "{name}: 556 is within {radius}");
    }
}

#[test]
fn neither_sides_carriers_can_strike_the_others_on_the_first_evening() {
    let mut allied = session('d', &[A]);
    for name in [
        "VF-6", "VB-6", "VS-5", "VT-3", "VF-5", "VB-3", "VS-3", "VT-8",
    ] {
        let (_, lines) = reach(&mut allied, A, name, &[(9, -18), (7, -18)]);
        assert!(lines.iter().all(|(_, within)| !within), "{name}: {lines:?}");
    }
    let mut japanese = session('d', &[J]);
    for name in [
        "Shokaku fighter unit",
        "Shokaku bomber unit",
        "Shokaku attack unit",
        "Zuikaku fighter unit",
        "Zuikaku bomber unit",
        "Zuikaku attack unit",
        "Ryujo fighter unit",
        "Ryujo attack unit",
    ] {
        let (_, lines) = reach(&mut japanese, J, name, &[(6, -3)]);
        assert!(lines.iter().all(|(_, within)| !within), "{name}: {lines:?}");
    }
}

// ---------------------------------------------------------------- The convoy

#[test]
fn the_convoy_is_task_force_5_on_the_transport_mission() {
    let t = start('a');
    let convoy = &t.task_forces[&tf(5)];
    assert_eq!((convoy.side, convoy.mission), (J, Mission::Transport));
    let ships: Vec<u32> = convoy.ships.iter().map(|s| s.raw()).collect();
    assert_eq!(ships, vec![37, 38, 39, 40, 41, 42, 43]);
}

#[test]
fn its_men_are_troop_lift_rows_8_and_9() {
    let t = start('a');
    let men: Vec<(u32, u32, bool)> = [27, 28, 29]
        .iter()
        .map(|raw| {
            let u = &t.land_units[&unit(*raw)];
            assert_eq!(u.side, J);
            assert_eq!(u.men_ashore, 0, "aboard");
            let aboard: u32 = t
                .ships
                .values()
                .filter_map(|s| s.troops.filter(|tr| tr.unit == unit(*raw)).map(|tr| tr.men))
                .sum();
            (*raw, aboard, u.working)
        })
        .collect();
    assert_eq!(
        men,
        vec![(27, 350, false), (28, 350, false), (29, 800, false)]
    );
    assert_eq!(350 + 350, 700, "army, row 8");
    assert_eq!(350 + 350 + 800, 1500, "row 7's total");
    // Every man aboard is in task force 5.
    for s in t.ships.values().filter(|s| s.troops.is_some()) {
        assert_eq!(s.location, ShipLocation::TaskForce(tf(5)), "{}", s.name);
    }
}

#[test]
fn three_placeholder_troop_transports_of_the_39_days_class_10() {
    let w = load('a').world;
    let class = &w.ref_data().ship_classes[&ShipClassId::new(10)];
    assert_eq!(class.name, "Placeholder troop transport");
    assert_eq!(
        (
            class.cruise_speed_kn,
            class.max_speed_kn,
            class.troop_capacity_men
        ),
        (n(10), n(12), 600)
    );
    for (raw, name) in [
        (37, "Placeholder transport 1"),
        (38, "Placeholder transport 2"),
        (39, "Placeholder transport 3"),
    ] {
        let s = &w.umpire().ships[&ship(raw)];
        assert_eq!((s.name.as_str(), s.class.raw(), s.side), (name, 10, J));
    }
    // The 39 days uses the same class for its troop transports.
    let slice = common::load_live(&common::data_dir().join("scenarios/07_full_slice.toml"))
        .expect("the 39 days load");
    assert_eq!(slice.world.umpire().ships[&ship(52)].class.raw(), 10);
}

#[test]
fn four_rated_destroyers_and_the_ships_left_out() {
    let w = load('a').world;
    for (raw, name, class, class_name) in [
        (40, "Kagero", 3011, "Kagero class"),
        (
            41,
            "Kawakaze",
            3012,
            "Shiratsuyu class, troop_capacity_men 150",
        ),
        (
            42,
            "Umikaze",
            3012,
            "Shiratsuyu class, troop_capacity_men 150",
        ),
        (
            43,
            "Suzukaze",
            3012,
            "Shiratsuyu class, troop_capacity_men 150",
        ),
    ] {
        let s = &w.umpire().ships[&ship(raw)];
        assert_eq!((s.name.as_str(), s.class.raw()), (name, class));
        assert_eq!(
            w.ref_data().ship_classes[&ShipClassId::new(class)].name,
            class_name
        );
        assert_eq!(s.location, ShipLocation::TaskForce(tf(5)));
    }
    // The classes are the ones the reference data gives these ships.
    let listed = std::fs::read_to_string(
        common::data_dir().join("reference/ship_classes_guadalcanal_landings.ships.csv"),
    )
    .expect("the list reads");
    for line in [
        "kagero,3011,",
        "kawakaze,3012,",
        "umikaze,3012,",
        "suzukaze,3012,",
    ] {
        assert!(listed.contains(line), "{line}");
    }
    // Left out, and the file says so.
    let names: BTreeSet<String> = w
        .umpire()
        .ships
        .values()
        .map(|s| s.name.to_lowercase())
        .collect();
    for left in [
        "jintsu", "isokaze", "mutsuki", "yayoi", "pb-1", "pb-2", "pb-34", "pb-35",
    ] {
        assert!(
            !names.iter().any(|n| n.contains(left)),
            "{left} is left out"
        );
        assert!(
            headers('a').to_lowercase().contains(left),
            "the file says {left} is left out"
        );
    }
}

#[test]
fn the_supply_aboard_is_30_t_a_transport() {
    let t = start('a');
    for raw in [37, 38, 39] {
        assert_eq!(t.ships[&ship(raw)].supply_t, n(30), "transport {raw}");
    }
    for raw in [40, 41, 42, 43] {
        assert_eq!(t.ships[&ship(raw)].supply_t, Fixed::ZERO, "destroyer {raw}");
    }
    // 1,500 men x 0.005 t x 12 days = 90 t.
    let daily = load('a')
        .world
        .ref_data()
        .ground
        .expect("ground figures")
        .daily_supply_t_per_man;
    assert_eq!(daily, fx("0.005"));
    assert_eq!(n(1500) * daily * n(12), n(90));
}

#[test]
fn the_force_makes_its_slowest_ships_speed_and_the_passage_is_280_miles() {
    let mut s = session('d', &[J]);
    let convoy = handle(&mut s, J, "task_force", "Reinforcement convoy");
    for (speed, knots, hours) in [("cruise", 10, "28"), ("full", 12, "23.333333")] {
        let a = measure(
            &mut s,
            J,
            json!({"question": "passage", "task_force": convoy, "to": place(3, -4), "speed": speed}),
        );
        assert_eq!(a["speed"]["knots"], json!(knots.to_string()), "{a}");
        assert_eq!(
            a["speed"]["slowest_ship"]["name"]
                .as_str()
                .map(|n| n.starts_with("Placeholder transport")),
            Some(true),
            "{a}"
        );
        // 280 miles: 28 hours at 10 kn, 23 hours 20 minutes at 12.
        assert_eq!(a["route_nm"], json!("280"), "{a}");
        assert_eq!(a["steaming_hours"], json!(hours), "{a}");
    }
}

/// Landing where there is no port: lighters at 20 t an hour, 0.13 of that
/// for men landing unprepared, 2.6 t an hour a ship; a man is 0.02 t, so
/// 130 men an hour.
#[test]
fn landing_where_there_is_no_port() {
    let w = load('a').world;
    assert_eq!(w.map().get(hex(3, -4)).expect("Taivu's hex").port_size, 0);
    let port = &w.ref_data().port_sizes[&0];
    assert_eq!(port.lighter_rate_t_per_h, n(20));
    let ground = w.ref_data().ground.expect("ground figures");
    assert_eq!(ground.unprepared_landing_share, fx("0.13"));
    assert_eq!(ground.handling_t_per_man, fx("0.02"));
    let rate = port.lighter_rate_t_per_h * ground.unprepared_landing_share;
    assert_eq!(rate, fx("2.6"));
    assert_eq!(rate / ground.handling_t_per_man, n(130), "men an hour");
    // 350 men in 2 hours 42 minutes; 800 in 6 hours 9 minutes, to the
    // nearest minute.
    let minutes = |men: i64| ((men as f64) / 130.0 * 60.0).round() as i64;
    assert_eq!(minutes(350), 2 * 60 + 42);
    assert_eq!(minutes(800), 6 * 60 + 9);
}

/// Transports 1 and 2 carry the two parts of the Ichiki Detachment's second
/// echelon, 350 men each.
#[test]
fn transports_1_and_2_carry_350_men_each() {
    let t = start('a');
    for (raw, u) in [(37, 27), (38, 28)] {
        let troops = t.ships[&ship(raw)].troops.expect("men aboard");
        assert_eq!((troops.unit, troops.men), (unit(u), 350), "transport {raw}");
    }
}

/// The 800 of the Yokosuka 5th Special Naval Landing Force are 600 on
/// transport 3, which has 600 berths, and the rest on the destroyers: 152
/// and 48 (the hand calculation as the architect corrected it).
#[test]
fn the_800_of_the_yokosuka_5th_are_600_on_transport_3_and_the_rest_on_destroyers() {
    let t = start('a');
    let on_3 = t.ships[&ship(39)].troops.expect("men aboard");
    assert_eq!((on_3.unit, on_3.men), (unit(29), 600));
    let mut men: Vec<u32> = t
        .ships
        .values()
        .filter_map(|s| s.troops)
        .filter(|troops| troops.unit == unit(29))
        .map(|troops| troops.men)
        .collect();
    men.sort_unstable();
    assert_eq!(men, vec![48, 152, 600]);
}

// --------------------------------------------------------- The rest of the forces

#[test]
fn task_forces_1_to_4_hold_10_8_14_and_4_ships() {
    let t = start('a');
    let counts: Vec<usize> = (1..=4)
        .map(|raw| t.task_forces[&tf(raw)].ships.len())
        .collect();
    assert_eq!(counts, vec![10, 8, 14, 4]);
}

#[test]
fn lunga_is_allied_with_its_airfield_and_the_squadrons_of_20_august() {
    let t = start('a');
    let lunga = &t.bases[&base(1)];
    assert_eq!((lunga.side, lunga.airfield), (A, true));
    for (raw, name, type_raw, ready) in [(9, "VMF-223", 2100, 19), (10, "VMSB-232", 2101, 12)] {
        let g = &t.air_groups[&group(raw)];
        assert_eq!(
            (g.name.as_str(), g.aircraft_type.raw(), g.ready, g.location),
            (name, type_raw, ready, AirGroupLocation::Base(base(1))),
        );
    }
    // The 39 days brings the same squadrons in on 20 August, its day 14, at
    // those strengths and types.
    let slice = common::load_live(&common::data_dir().join("scenarios/07_full_slice.toml"))
        .expect("the 39 days load");
    let st = slice.world.umpire();
    let arrived: BTreeMap<u32, (u32, u32)> = st
        .arrivals
        .iter()
        .filter(|a| a.day == 14)
        .map(|a| {
            (
                a.air_group.raw(),
                (a.aircraft, st.air_groups[&a.air_group].aircraft_type.raw()),
            )
        })
        .collect();
    assert_eq!(arrived.get(&10), Some(&(19, 2100)), "VMF-223");
    assert_eq!(arrived.get(&11), Some(&(12, 2101)), "VMSB-232");
}

#[test]
fn the_marines_on_guadalcanal_are_the_39_days_four_units_on_the_defence() {
    let t = start('a');
    let slice = common::load_live(&common::data_dir().join("scenarios/07_full_slice.toml"))
        .expect("the 39 days load");
    let mut total = 0;
    for (raw, men) in [(1, 4306), (2, 4561), (3, 1373), (4, 660)] {
        let u = &t.land_units[&unit(raw)];
        assert_eq!(
            (u.side, u.men_ashore, u.hex, u.posture),
            (A, men, Some(hex(2, -4)), Posture::Defend),
            "unit {raw}"
        );
        assert_eq!(u.name, slice.world.umpire().land_units[&unit(raw)].name);
        total += men;
    }
    assert_eq!(total, 10_900);
}

#[test]
fn lungas_supply_does_not_run_out_by_the_mens_eating() {
    let w = load('a').world;
    assert_eq!(w.umpire().bases[&base(1)].supply_t, n(1000));
    let daily = w
        .ref_data()
        .ground
        .expect("ground figures")
        .daily_supply_t_per_man;
    let a_day = n(10_900) * daily;
    assert_eq!(a_day, fx("54.5"));
    assert_eq!(a_day * n(5), fx("272.5"));
    assert!(a_day * n(5) < n(1000));
    // In file d, where nothing is done to it, the stock is above zero at the
    // end of every day.
    let mut d = load('d');
    while d.next_day().expect("a day resolves").is_some() {
        assert!(d.world.umpire().bases[&base(1)].supply_t > Fixed::ZERO);
    }
}

#[test]
fn taivu_is_a_japanese_stock_with_no_supply_and_100_men() {
    let t = start('a');
    let taivu = &t.bases[&base(5)];
    assert_eq!(
        (taivu.side, taivu.supply_t, taivu.airfield, taivu.on_map),
        (J, Fixed::ZERO, false, false)
    );
    let u = &t.land_units[&unit(26)];
    assert_eq!(
        (u.side, u.men_ashore, u.hex, u.posture, u.working),
        (J, 100, Some(hex(3, -4)), Posture::Defend, false)
    );
}

#[test]
fn rabaul_has_the_39_days_bombers_and_fighters_and_its_supply() {
    let t = start('a');
    let slice = common::load_live(&common::data_dir().join("scenarios/07_full_slice.toml"))
        .expect("the 39 days load");
    let st = slice.world.umpire();
    for (here, there) in [(27, 1), (28, 2)] {
        let (g, s) = (&t.air_groups[&group(here)], &st.air_groups[&group(there)]);
        assert_eq!(
            (&g.name, g.aircraft_type, g.ready, g.location),
            (&s.name, s.aircraft_type, s.ready, s.location),
            "group {here}"
        );
        assert_eq!(g.location, AirGroupLocation::Base(base(2)));
    }
    assert_eq!(t.bases[&base(2)].supply_t, st.bases[&base(2)].supply_t);
}

#[test]
fn tulagi_is_allied_with_the_39_days_units_and_has_no_part() {
    let t = start('a');
    let slice = common::load_live(&common::data_dir().join("scenarios/07_full_slice.toml"))
        .expect("the 39 days load");
    let tulagi = &t.bases[&base(4)];
    assert_eq!((tulagi.side, tulagi.airfield), (A, false));
    for raw in 5..=8 {
        let (u, s) = (
            &t.land_units[&unit(raw)],
            &slice.world.umpire().land_units[&unit(raw)],
        );
        assert_eq!(u.name, s.name);
        assert_eq!((u.side, u.hex), (A, Some(hex(3, -5))));
        assert_eq!(u.men_ashore, [828, 395, 1085, 3767][raw as usize - 5]);
    }
    // No order of either side names it or its men in file a.
    let a = played('a');
    assert!(!a.orders.iter().any(|o| matches!(
        o.kind.target(),
        EntityRef::Base(b) if b == base(4)
    ) || matches!(o.kind.target(), EntityRef::LandUnit(u) if (5..=8).contains(&u.raw()))));
}

// ---------------------------------------------- What a side the engine plays is given

fn intent(state: &CommandState, raw: u32) -> &sim_core::command::Intent {
    &state.intents[&IntentId::new(raw)]
}

#[test]
fn the_japanese_side_is_given_its_four_intents_and_both_leaves() {
    for letter in ['a', 'b'] {
        let state = &played(letter).after_day_1[&J];
        assert_eq!(intent(state, 1).kind, IntentKind::Hold { base: base(5) });
        let f = intent(state, 1)
            .limits
            .forces
            .as_ref()
            .expect("a forces limit");
        assert_eq!(f.task_forces, BTreeSet::from([tf(5)]));
        assert_eq!(f.land_units, BTreeSet::from([unit(27), unit(28), unit(29)]));
        assert!(f.air_groups.is_empty() && f.ships.is_empty());
        assert_eq!(f.source, Some(base(2)));
        assert_eq!(
            intent(state, 2).kind,
            IntentKind::Neutralise { target: base(1) }
        );
        let f = intent(state, 2)
            .limits
            .forces
            .as_ref()
            .expect("a forces limit");
        assert_eq!(f.air_groups, BTreeSet::from([group(27), group(28)]));
        assert!(f.task_forces.is_empty() && f.land_units.is_empty());
        assert_eq!(
            intent(state, 3).kind,
            IntentKind::OperateCarriers {
                force: tf(3),
                hex: hex(8, -11)
            }
        );
        assert_eq!(
            intent(state, 4).kind,
            IntentKind::OperateCarriers {
                force: tf(4),
                hex: hex(7, -9)
            }
        );
        for raw in 1..=4 {
            let i = intent(state, raw);
            assert_eq!(i.limits.risk, None, "no risk set: balanced");
            assert_eq!(i.set_day, 1);
        }
        assert!(
            intent(state, 3).limits.forces.is_none() && intent(state, 4).limits.forces.is_none()
        );
        assert!(state.leaves.prepare && state.leaves.launch);
    }
}

#[test]
fn the_allied_side_is_given_its_four_intents_and_both_leaves() {
    for letter in ['a', 'c'] {
        let state = &played(letter).after_day_1[&A];
        assert_eq!(intent(state, 1).kind, IntentKind::Hold { base: base(1) });
        let f = intent(state, 1)
            .limits
            .forces
            .as_ref()
            .expect("a forces limit");
        assert_eq!(
            f.land_units,
            BTreeSet::from([unit(1), unit(2), unit(3), unit(4)])
        );
        assert_eq!(f.air_groups, BTreeSet::from([group(9), group(10)]));
        assert!(f.task_forces.is_empty() && f.ships.is_empty() && f.source.is_none());
        assert_eq!(
            intent(state, 2).kind,
            IntentKind::OperateCarriers {
                force: tf(1),
                hex: hex(6, -3)
            }
        );
        assert_eq!(
            intent(state, 3).kind,
            IntentKind::OperateCarriers {
                force: tf(2),
                hex: hex(7, -3)
            }
        );
        assert_eq!(
            intent(state, 4).kind,
            IntentKind::Deny {
                hex: hex(3, -7),
                distance: 2
            }
        );
        assert!(intent(state, 4).limits.forces.is_none());
        for raw in 1..=4 {
            assert_eq!(intent(state, raw).limits.risk, None, "balanced");
            assert_eq!(intent(state, raw).set_day, 1);
        }
        assert!(state.leaves.prepare && state.leaves.launch);
    }
}

#[test]
fn a_side_not_given_orders_holds_no_intent() {
    for (letter, side) in [('b', A), ('c', J), ('d', A), ('d', J)] {
        let state = &played(letter).after_day_1[&side];
        assert!(
            !state
                .intents
                .values()
                .any(|i| i.maker == sim_core::command::Maker::Commander),
            "file {letter}, {side:?}"
        );
    }
}

// ---------------------------------------------------------------- The objectives

#[test]
fn the_objectives_are_the_scenarios_own_over_30_days() {
    use sim_core::objectives::Objective;
    let w = load('a').world;
    let o = w.objectives().expect("objectives");
    assert!(o.for_play, "a scenario for play");
    assert_eq!(
        (o.levels.minor, o.levels.major, o.levels.decisive),
        (n(3), n(8), n(16))
    );
    assert_eq!(o.horizon_days, 30);
    let sunk = [
        ("carrier", "10"),
        ("battleship", "8"),
        ("cruiser", "4"),
        ("destroyer", "1"),
        ("transport", "2"),
        ("auxiliary", "1"),
    ];
    let out = [
        ("carrier", "5"),
        ("battleship", "4"),
        ("cruiser", "2"),
        ("destroyer", "0.5"),
        ("transport", "1"),
        ("auxiliary", "0.5"),
    ];
    for side in Side::ALL {
        let list = &o.side(side).objectives;
        let named: Vec<&str> = list.iter().map(|x| x.name()).collect();
        let mut expected = vec![
            "ships_sunk",
            "ships_out_of_action",
            "aircraft_destroyed",
            "men_lost",
            "base_open",
        ];
        if side == J {
            expected.push("cargo_landed");
            expected.push("men_landed");
        }
        assert_eq!(named, expected, "{side:?}");
        for x in list {
            match x {
                Objective::ShipsSunk(p) | Objective::ShipsOutOfAction(p) => {
                    let table = if matches!(x, Objective::ShipsSunk(_)) {
                        &sunk
                    } else {
                        &out
                    };
                    let got: BTreeMap<String, Fixed> =
                        p.0.iter()
                            .map(|(k, v)| (k.name().to_string(), *v))
                            .collect();
                    let want: BTreeMap<String, Fixed> =
                        table.iter().map(|(k, v)| (k.to_string(), fx(v))).collect();
                    assert_eq!(got, want, "{side:?} {}", x.name());
                }
                Objective::AircraftDestroyed { points } => assert_eq!(*points, fx("0.05")),
                Objective::MenLost { points_per_100_men } => {
                    assert_eq!(*points_per_100_men, n(1))
                }
                Objective::BaseOpen {
                    base: b,
                    closed,
                    points,
                } => {
                    assert_eq!((*b, *closed, *points), (base(1), side == J, n(10)));
                }
                Objective::CargoLanded {
                    base: b,
                    points_per_t,
                } => {
                    assert_eq!((*b, *points_per_t), (base(5), fx("0.005")));
                }
                Objective::MenLanded {
                    base: b,
                    points_per_100_men,
                } => {
                    assert_eq!((*b, *points_per_100_men), (base(5), n(1)));
                }
                other => panic!("an objective the table has not: {}", other.name()),
            }
        }
    }
}

// ----------------------------------------------------------------- Properties

/// Each file loads and plays five days, and the same seed gives the same
/// daily hashes twice.
#[test]
fn each_file_plays_five_days_the_same_twice() {
    for letter in LETTERS {
        let first = &played(letter).hashes;
        assert_eq!(first.len(), 5, "file {letter} plays five days");
        assert_eq!(&play(letter).hashes, first, "file {letter} again");
    }
}

/// The four files are one scenario but for their names and command tables.
/// That they state nothing else is statement R3 of
/// `scenarios_built_on_another`, which holds it of each file's own text.
#[test]
fn the_four_files_are_one_scenario() {
    assert!(
        !text('d').lines().any(|line| line == "[[command]]"),
        "file d has no command"
    );
    for letter in ['a', 'b', 'c'] {
        assert!(
            text(letter).lines().any(|line| line == "[[command]]"),
            "{letter}"
        );
    }
    // The name of each is its own.
    let names: BTreeSet<String> = LETTERS.iter().map(|l| load(*l).name).collect();
    assert_eq!(names.len(), 4);
}

/// The forces are scenario 5's: every ship of task forces 1 to 4 and every
/// air group of theirs has the class, the type and the strength scenario 5
/// gives it.
#[test]
fn the_forces_are_scenario_5s() {
    let five = common::load_live(&common::data_dir().join("scenarios/05_eastern_solomons.toml"))
        .expect("scenario 5 loads");
    let f = five.world.umpire();
    for letter in LETTERS {
        let t = start(letter);
        for raw in 1..=4 {
            assert_eq!(t.task_forces[&tf(raw)].ships, f.task_forces[&tf(raw)].ships);
            assert_eq!(t.task_forces[&tf(raw)].name, f.task_forces[&tf(raw)].name);
        }
        for raw in 1..=36 {
            let (a, b) = (&t.ships[&ship(raw)], &f.ships[&ship(raw)]);
            assert_eq!(
                (&a.name, a.class, a.side, a.fuel_t, a.location),
                (&b.name, b.class, b.side, b.fuel_t, b.location),
                "ship {raw}"
            );
        }
        let mut theirs = 0;
        for (id, g) in &f.air_groups {
            if let AirGroupLocation::Ship(_) = g.location {
                let ours = &t.air_groups[id];
                assert_eq!(
                    (&ours.name, ours.aircraft_type, ours.ready, ours.location),
                    (&g.name, g.aircraft_type, g.ready, g.location),
                    "group {id:?}"
                );
                theirs += 1;
            }
        }
        assert_eq!(theirs, 24, "groups 1 to 8 and 11 to 26");
        // Nothing is added to the carrier forces.
        assert_eq!(
            t.air_groups
                .values()
                .filter(|g| matches!(g.location, AirGroupLocation::Ship(_)))
                .count(),
            24
        );
    }
}

/// Nothing waits for a later day: no unit has a day from which it may be
/// used.
#[test]
fn nothing_waits_for_a_later_day() {
    for letter in LETTERS {
        assert!(!text(letter).contains("available_from"), "file {letter}");
        let t = start(letter);
        let units: Vec<EntityRef> = t
            .ships
            .keys()
            .map(|id| EntityRef::Ship(*id))
            .chain(t.task_forces.keys().map(|id| EntityRef::TaskForce(*id)))
            .chain(t.air_groups.keys().map(|id| EntityRef::AirGroup(*id)))
            .chain(t.land_units.keys().map(|id| EntityRef::LandUnit(*id)))
            .collect();
        for u in units {
            assert_eq!(t.available_from(u), None, "file {letter}, {u:?}");
        }
    }
}

fn staff_orders(p: &Played, side: Side, day: u32) -> Vec<&Order> {
    p.orders
        .iter()
        .filter(|o| o.side == side && o.day_issued == day && o.origin.issuer == Issuer::Staff)
        .collect()
}

/// A side given no orders does nothing of its own: in file d no order is
/// given by either staff in five days.
#[test]
fn a_side_given_no_orders_does_nothing_of_its_own() {
    let d = played('d');
    assert!(
        !d.orders.iter().any(|o| o.origin.issuer == Issuer::Staff),
        "{:?}",
        d.orders
            .iter()
            .filter(|o| o.origin.issuer == Issuer::Staff)
            .collect::<Vec<_>>()
    );
    for side in Side::ALL {
        for day in 1..=5 {
            assert!(staff_orders(d, side, day).is_empty());
        }
    }
}

/// A side given its orders acts on the first morning: in file a, on day 1
/// the Japanese staff gives orders to the convoy and to the bombers at
/// Rabaul, and each side's staff to its carrier forces.
#[test]
fn a_side_given_its_orders_acts_on_the_first_morning() {
    let a = played('a');
    let targets = |side: Side| -> BTreeSet<EntityRef> {
        staff_orders(a, side, 1)
            .iter()
            .map(|o| o.kind.target())
            .collect()
    };
    let japanese = targets(J);
    for t in [
        EntityRef::TaskForce(tf(5)),
        EntityRef::AirGroup(group(27)),
        EntityRef::TaskForce(tf(3)),
        EntityRef::TaskForce(tf(4)),
    ] {
        assert!(
            japanese.contains(&t),
            "Japanese staff to {t:?}: {japanese:?}"
        );
    }
    let allied = targets(A);
    for t in [EntityRef::TaskForce(tf(1)), EntityRef::TaskForce(tf(2))] {
        assert!(allied.contains(&t), "Allied staff to {t:?}: {allied:?}");
    }
    // And in the files of one side, only that side's staff acts.
    assert!(!played('b')
        .orders
        .iter()
        .any(|o| o.side == A && o.origin.issuer == Issuer::Staff));
    assert!(!played('c')
        .orders
        .iter()
        .any(|o| o.side == J && o.origin.issuer == Issuer::Staff));
    assert!(!staff_orders(played('b'), J, 1).is_empty());
    assert!(!staff_orders(played('c'), A, 1).is_empty());
}

/// It can be played from a seat: file b with the Allied seat played and
/// file c with the Japanese seat played each open, list the tools of
/// command, and answer `trial`.
#[test]
fn it_can_be_played_from_a_seat() {
    for (letter, side, intent) in [
        (
            'b',
            A,
            json!({"intent_kind": "deny", "hex": [3, -4], "distance": 2}),
        ),
        (
            'c',
            J,
            json!({"intent_kind": "deny", "hex": [6, -3], "distance": 2}),
        ),
    ] {
        let mut s = session(letter, &[side]);
        let tools = s.tools(side);
        for tool in [
            "get_briefing",
            "issue_orders",
            "end_day",
            "set_intent",
            "set_limit",
            "set_leave",
            "command_state",
            "staff_orders",
            "trial",
            "trials",
        ] {
            assert!(tools.contains(&tool), "file {letter}: {tool} in {tools:?}");
        }
        let queued = s
            .call(side, "set_intent", &intent)
            .expect("set_intent answers");
        assert_eq!(queued["status"], "queued", "{queued}");
        let a = s
            .call(side, "trial", &json!({"intent": 1, "size": "small"}))
            .unwrap_or_else(|e| panic!("file {letter}: trial: {}", e.message));
        assert_eq!(a["kind"], "trial", "file {letter}: {a}");
    }
}

/// Scenario 5 is untouched: its file, its frozen copy and its variants are
/// what they were when this scenario was built (their digests pinned here).
/// The second proof's 05a and 05b and stage 2b's three were given `none_of`
/// on their denies' forces limits by the architect's ruling B of decision
/// 0222, which says the same order in the new language; their digests are
/// remade for it, and their games are what they were.
#[test]
fn scenario_5_is_untouched() {
    let pinned: [(&str, u64); 17] = [
        ("scenarios/05_eastern_solomons.toml", 0xa9443b066a393e0b),
        ("scenarios/05a_carrier_search.toml", 0xa40fa2e4d863e0af),
        ("scenarios/05b_strike.toml", 0x765f73dc37361e41),
        ("scenarios/05c_air_combat.toml", 0x10d40e4ec2bdcbb5),
        ("scenarios/05d_carrier_duel.toml", 0xe5565dc0a55c5747),
        (
            "scenarios/orders-inside-proof/05_eastern_solomons_on.toml",
            0x77c513715059390c,
        ),
        (
            "scenarios/orders-inside-proof/05c_allied_by_days.toml",
            0x6bb50bf358a60ce0,
        ),
        (
            "scenarios/orders-inside-proof/05c_allied_by_pulse.toml",
            0xd5dd3bbceb85ce34,
        ),
        (
            "scenarios/second-proof/05a_eastern_solomons.toml",
            0xd64d_53ab_74f6_bcc0,
        ),
        (
            "scenarios/second-proof/05b_eastern_solomons.toml",
            0xaebb_1fe1_89ce_7edc,
        ),
        (
            "scenarios/second-proof/05c_eastern_solomons.toml",
            0xffee21cdec4cd934,
        ),
        (
            "scenarios/second-proof/05d_eastern_solomons.toml",
            0xd75e56a0c1d93dbf,
        ),
        (
            "scenarios/second-proof/05e_eastern_solomons.toml",
            0xbbeb57e5cf77c5f3,
        ),
        (
            "scenarios/stage-2b-proof/05a_eastern_solomons.toml",
            0x7164_a06d_bfbc_9e12,
        ),
        (
            "scenarios/stage-2b-proof/05b_eastern_solomons_off.toml",
            0x935d_0b33_c813_1bd8,
        ),
        (
            "scenarios/stage-2b-proof/05c_eastern_solomons_bold.toml",
            0xe3ce_11dc_a5d6_b89a,
        ),
        (
            "scenarios/step2-frozen/05_eastern_solomons.toml",
            0x9228b5cf39c7917a,
        ),
    ];
    let mut now = Vec::new();
    for (name, _) in &pinned {
        let bytes = std::fs::read(common::data_dir().join(name)).expect("reads");
        now.push((name.to_string(), digest(&bytes)));
    }
    let want: Vec<(String, u64)> = pinned.iter().map(|(n, d)| (n.to_string(), *d)).collect();
    assert_eq!(now, want);
}
