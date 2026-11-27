//! Scenario 114: Midway, 3 to 7 June 1942, a scenario for play.
//!
//! Proves decision 0198. What follows is the architect's, written before
//! any of the scenario is built: the forces, where they start, the
//! arithmetic of the transports, the orders a side the engine plays is
//! given, and the scoring. Each numbered statement is a test. If the
//! scenario or the game disagrees with a statement, the statement or the
//! scenario is wrong: a mismatch goes back to the architect, and nobody
//! else changes a number here.
//!
//! Rows are of `data/oob` unless said. "The list" is
//! `data/derivation/midway.toml` and the files it makes. "Rule N" is a
//! rule of decision 0198.
//!
//! # The clock and the days
//!
//! The game begins at 1800 on 2 June 1942. Day 1 is 3 June and day 5 is 7
//! June. Times are zone +12, Midway's clock; a Japanese row dates the
//! same day a day later, 21 hours on.
//!
//! 1. The scenario has `days = 5`, and its header says the game begins at
//!    1800 on 2 June 1942, that day 1 is 3 June, and that its times are
//!    zone +12.
//!
//! # The map and the bases
//!
//! The Midway map of decision 0199, `data/map/midway.csv`. Hexes are 40
//! nm. Hex (0,0) is at 28.5 N, 178.5 E.
//!
//! | Base | Number | Side | Hex | What it has |
//! | --- | --- | --- | --- | --- |
//! | Midway | 1 | Allied | (5,1) | Its garrison; its aircraft and their airfield; 4,000 t of supply, a placeholder (rule 94); no fuel for ships; no anti-aircraft figure and no port size (gaps) |
//! | Wake | 2 | Japanese | (-24,16) | 100 t of supply, a placeholder; no aircraft, no airfield, no men, no fuel (rule 81) |
//!
//! 2. Midway is a coast hex and Wake is, each by the map's mark of land.
//! 3. Midway has 4,000 t of supply and Wake 100 t, and the header marks
//!    both placeholders and says why Wake is in: a lift needs a base of
//!    its side, and the landing force in truth came from Saipan and Guam,
//!    off the map.
//! 4. No base has a port size, and no Japanese or Allied force has a home
//!    port: Pearl Harbor, Japan and the Marianas are off the map.
//!
//! # The Allied forces
//!
//! Every ship is of her class in the list (classes 5005 to 5021).
//!
//! **Task force 1, Task Force 16,** on the carrier mission, 17 ships:
//! Enterprise, Hornet; Pensacola, Northampton, New Orleans, Minneapolis,
//! Vincennes, Atlanta; Phelps, Worden, Monaghan, Aylwin, Balch,
//! Conyngham, Benham, Ellet, Maury.
//!
//! **Task force 2, Task Force 17,** on the carrier mission, 8 ships:
//! Yorktown; Astoria, Portland; Morris, Hughes, Anderson, Hammann,
//! Russell.
//!
//! Both start in hex (15,-6), the hex of 32 04 N, 172 45 W, where the
//! rows have the two forces meet on the afternoon of 2 June (engagement
//! positions 5737 to 5739), some two hours before the game begins. Left
//! out and said in the header: Gwin, who joined on 5 June (rule 85); the
//! fuelling group (Cimarron, Platte, Dewey, Monssen); the submarines; the
//! auxiliaries of the naval district.
//!
//! 5. Task force 1 has those 17 ships and task force 2 those 8, each of
//!    the class the list gives her, and both start at (15,-6), which is
//!    deep sea.
//!
//! The air groups, in this order, each of the type the list gives:
//!
//! | Group | Aboard or at | Type | Aircraft | Row |
//! | --- | --- | --- | --- | --- |
//! | VF-6 | Enterprise | F4F-4 | 27 | 12610 |
//! | VB-6 | Enterprise | SBD-3 (rule 90) | 18 | 12600 |
//! | VS-6 | Enterprise | SBD-3 | 18 | 12606 |
//! | VT-6 | Enterprise | TBD-1 | 14 | 5727 |
//! | VF-8 | Hornet | F4F-4 | 27 | 12641 |
//! | VB-8 and VS-8, one group | Hornet | SBD-3 | 35 | 12639 |
//! | VT-8 | Hornet | TBD-1 | 15 | 5739 |
//! | VF-3 | Yorktown | F4F-4 | 25 | 5705 |
//! | VB-3 | Yorktown | SBD-3 | 18 | 5702 |
//! | VS-5 | Yorktown | SBD-3 | 18 | 5711 |
//! | VT-3 | Yorktown | TBD-1 | 12 | 5708 |
//! | VMF-221 | Midway | F2A-3 | 21 | 5748 |
//! | VMF-221 | Midway | F4F-3 | 7 | 5749 |
//! | VMSB-241 | Midway | SBD-2 | 19 | 5750 |
//! | VMSB-241 | Midway | SB2U-3 | 17, of which 5 damaged (rule 92) | 5751 |
//! | The patrol squadrons | Midway | PBY-5 | 14 | 5753 |
//! | The patrol squadrons | Midway | PBY-5A | 16 | 5754 |
//! | The Army bombers | Midway | B-17E | 17 | 5755 |
//!
//! Left out and said in the header: the four B-26s and the six TBF-1s at
//! Midway (rules 60 and 78), and the six more B-17s that came on 4 June.
//!
//! 6. Enterprise has 27 + 18 + 18 + 14 = 77 aircraft, Hornet 27 + 35 + 15
//!    = 77, Yorktown 25 + 18 + 18 + 12 = 73, and Midway 21 + 7 + 19 + 17 +
//!    14 + 16 + 17 = 111: 338 Allied aircraft. No carrier has more than
//!    her class holds.
//! 7. Every group has the type the table gives and is ready in full at
//!    the start, but the SB2U-3 group, which has 12 ready and 5 damaged.
//!    The groups are numbered in the order of the table, 1 to 18.
//!
//! **Ashore:** land unit 1, Midway's garrison, 2,178 men (land unit facts
//! 6907; rule 63), at Midway, on defence.
//!
//! 8. Land unit 1 has 2,178 men ashore at (5,1).
//!
//! # The Japanese forces
//!
//! Every warship is of her class in the list (classes 5000 to 5003 and
//! 5022 to 5035). The transports are of a placeholder class of their own.
//!
//! **Task force 3, the First Carrier Striking Force,** on the carrier
//! mission, 21 ships: Akagi, Kaga, Soryu, Hiryu; Haruna, Kirishima; Tone,
//! Chikuma; Nagara; Kazagumo, Yugumo, Makigumo, Akigumo, Isokaze,
//! Urakaze, Hamakaze, Tanikaze, Nowaki, Arashi, Hagikaze, Maikaze. It
//! starts at (-2,-15), the hex of 37 01.5 N, 171 07 E, where its own
//! report has it at 1630 on 2 June (engagement positions 5700; rule 32).
//!
//! **Task force 4, the Midway Invasion Force,** on the transport mission,
//! 46 ships (rule 86):
//!
//! - the thirteen transports, "Placeholder transport 1" to "13" (rule
//!   88);
//! - their close screen, 11 ships: Jintsu; Kuroshio, Oyashio, Hatsukaze,
//!   Yukikaze, Amatsukaze, Tokitsukaze, Kasumi, Arare, Kagero, Shiranui;
//! - the Close Support Group, 8 ships: Kumano, Suzuya, Mogami, Mikuma;
//!   Arashio, Asashio, Oshio, Michishio;
//! - the Second Fleet's main body, 14 ships: Kongo, Hiei; Atago, Chokai,
//!   Haguro, Myoko; Yura; Yudachi, Murasame, Harusame, Samidare, Asagumo,
//!   Minegumo, Natsugumo.
//!
//! It starts at (-17,4) (rule 83, worked below), stopped, with no route.
//!
//! Left out and said in the header: Zuiho and Mikazuki (rule 87); the
//! Main Body with Yamato; the Aleutians forces; the seaplane tender
//! group; the minesweepers; the supply groups; Patrol Boats 1, 2 and 34;
//! the submarines; Wake's air units.
//!
//! 9. The two forces have 21 and 46 ships, 67 in all, each warship of the
//!    class the list gives her, and start at (-2,-15) and (-17,4), both
//!    deep sea. Every ship of the scenario, of either side, starts with
//!    her fuel full.
//! 10. The header names the thirteen ships the placeholder transports
//!     stand for, says the thirteenth is disputed, and says that the
//!     night surface threat of the Second Fleet's main body is not in the
//!     game.
//!
//! The air groups, numbered on from the Allied ones in this order,
//! carrier by carrier, fighters then bombers then attack planes, and
//! then the two cruisers:
//!
//! | Aboard | Fighters, A6M2 | Bombers, D3A1 | Attack planes, B5N2 | Other |
//! | --- | --- | --- | --- | --- |
//! | Akagi | 24 | 18 | 18 | |
//! | Kaga | 27 | 18 | 27 | |
//! | Soryu | 21 | 16 | 18 | |
//! | Hiryu | 21 | 18 | 18 | |
//! | Tone | | | | 3 Type 0 floatplanes, E13A1 |
//! | Chikuma | | | | 3 Type 0 floatplanes, E13A1 |
//!
//! Rules 73, 74 and 91. A carrier's 6th Air Group fighters are in her
//! fighter group.
//!
//! 11. The four carriers have 60, 72, 55 and 57 aircraft: fighters 24 +
//!     27 + 21 + 21 = 93, bombers 18 + 18 + 16 + 18 = 70, attack planes 18
//!     + 27 + 18 + 18 = 81, 244 in all. The two cruisers have 6: 250
//!     Japanese aircraft in 14 groups, numbered 19 to 32. No ship has
//!     more than her class holds.
//!
//! **The landing force,** aboard task force 4: land unit 2, the Ichiki
//! Detachment, 2,507 men, and land unit 3, the 2nd Combined Special
//! Naval Landing Force, 2,000 men (rule 62).
//!
//! 12. The two units are aboard the transports of task force 4 with
//!     2,507 and 2,000 men, 4,507 in all, and no man of them is ashore.
//!
//! # The transports
//!
//! **Their class** is a placeholder of its own, marked: 600 berths, no
//! guns, a cruising and a top speed of 8.8 kn, and its other figures
//! those of the Coral Sea's placeholder transport. Each carries 41 t of
//! supply, as the Coral Sea's do (rule 95). The speed stands on two
//! positions alone (rule 65): 700 miles from Midway at 0925 on 3 June
//! and 560 miles at 0120 on 4 June (engagement events 7699 and 7706) are
//! 140 nm in 15 hours 55 minutes, 8.8 kn to one place.
//!
//! **Their start** (rule 83). Back from 0925 on 3 June to 1800 on 2 June
//! is 15 hours 25 minutes, 135.7 nm at 8.8 kn: 835.7 nm from Midway on
//! bearing 261, laid off in the map's own flat frame. Midway is at 28.232
//! N, 177.370 W, which in that frame is 217.8 nm east and 16.1 nm south
//! of hex (0,0). On bearing 261 the start is 825.4 nm west and 130.7 nm
//! south of Midway: 607.6 nm west and 146.8 nm south of hex (0,0). A
//! hex's centre is 40 x (q + r/2) nm east and 34.641 x r nm south: r is 4
//! and q is -17, whose centre is 600 nm west and 138.6 nm south.
//!
//! 13. The thirteen transports are of that class, 600 berths each, 7,800
//!     in all for 4,507 men, each with 41 t of supply, and the file says
//!     beside the speed that it stands on the two positions alone.
//! 14. From (-17,4) to Midway's hex the sea way is 22 steps, charged to a
//!     ship as 827 nm, with no land in the way. At 8.8 kn that is 94
//!     hours less a minute: a force that sails at 1800 on 2 June and
//!     holds 8.8 kn reaches Midway's hex at about 1600 on 6 June, in the
//!     afternoon of day 4.
//! 15. The force's speed is the transports': no warship of it cruises
//!     slower than 8.8 kn.
//!
//! # Distances
//!
//! 16. By the map: from task force 3's start to Midway 23 steps, 817 nm
//!     by air; from the American carriers' start to Midway 10 steps, 356
//!     nm; from Wake to Midway 29 steps; from task force 3's start to
//!     (5,-3), four hexes north of Midway, 19 steps, charged as 666 nm.
//!
//! # What a side the engine plays is given
//!
//! All on day 1, with both leaves, to prepare and to launch.
//!
//! **The Japanese side:**
//!
//! - **Take Midway** (the place at (5,1)), its forces task forces 3 and 4
//!   and land units 2 and 3, with no need to know, and no risk set, so
//!   that it is balanced (rule 96). The carriers are in the take's forces
//!   and have no order of their own (rule 38).
//! - It is given no estimate of the men at Midway.
//!
//! **The Allied side:**
//!
//! - **Hold Midway,** its forces land unit 1, the two fighter groups of
//!   VMF-221 (groups 12 and 13) and the two groups of flying boats (16
//!   and 17) (rule 97).
//! - **Deny the waters within 4 hexes of (1,-1),** its forces the SBD-2
//!   group, the SB2U-3 group and the B-17E group (14, 15 and 18); what to
//!   strike first: carriers, then transports, battleships, cruisers,
//!   destroyers (rule 98).
//! - **Operate** task forces 1 and 2 from (9,-5), the hex of 31 30 N, 176
//!   30 W, the launching position chosen for dawn on 4 June, 202 miles
//!   north of Midway (engagement positions 5744 and 5745; rule 103).
//!   (Amended by the architect on Nov 22, 2026, on the designer's word
//!   after the first run of 1,000 games: as first written, task force 1
//!   from (14,-6), Point Luck, and task force 2 from (15,-6), where they
//!   waited on 2 and 3 June.)
//!
//! 17. Each order is accepted on day 1 as given, and none is refused.
//! 18. In the file in which the Japanese side is given its orders, by the
//!     morning of day 2 the staff has formed a plan under the take; task
//!     force 4 has sailed for Midway at 8.8 kn; and task force 3 is at,
//!     or under way to, a hex no more than 4 hexes from Midway. If the
//!     staff forms no plan, or gives task force 4 no order, the build
//!     stops and reports the staff's words: nothing is changed to make it
//!     sail.
//! 19. In the file in which the Allied side is given its orders, on day 1
//!     a fighter group patrols over Midway, both groups of flying boats
//!     search, no flying boat is sent to strike, and the hexes (5,-3),
//!     (-3,1) and (1,3) are within the waters denied.
//!
//! # The files
//!
//! Four, made by a script from one base, as the Coral Sea's are: in file
//! a both sides are given their orders; in b the Japanese side alone; in
//! c the Allied side alone; in d neither.
//!
//! 20. The four files differ from the base only in the orders, and the
//!     script's check passes on them.
//! 21. Each has the key that marks a scenario for play, and scenario
//!     104's statement of which files have it gains these four and the
//!     table's copies of b, c and d. (That statement is the architect's,
//!     and is restated with this one.)
//! 22. In files c and d, where the Japanese side is given nothing, task
//!     force 4 does not move and no man lands in five days.
//!
//! # The scoring
//!
//! The figures of the scenarios for play: a ship sunk, carrier 10,
//! battleship 8, cruiser 4, destroyer 1, transport 2; out of action over
//! the horizon, half of each; an aircraft destroyed 0.05; enemy men lost
//! 1 for each 100; and for the Japanese side men landed at Midway 1 for
//! each 100.
//!
//! The levels (rule 100): minor 5, major 15, decisive 30; the horizon 66
//! days, from 2 June to the landings on Guadalcanal on 7 August (28 days
//! of June, 31 of July, 7 of August).
//!
//! 23. The file's objectives are those figures, levels and horizon, and
//!     mark it for play.
//! 24. What is at stake, by those figures. The four fleet carriers sunk
//!     are 40 points. The thirteen transports sunk with every
//!     man aboard are 13 x 2 + 45.07 = 71.07. Every man of the landing
//!     force landed is 45.07 to the Japanese side. The three American
//!     carriers sunk are 30. Midway's garrison lost to the last man is
//!     21.78.
//! 25. The history, as the file's objectives state it for the end paper:
//!     four Japanese carriers and one cruiser sunk, one American carrier
//!     and one destroyer sunk, and no man landed.
//!
//! # What the game does not model, said in the header
//!
//! Rules 16, 17 and 42 to 45, and: the night surface threat (rule 86);
//! ships' experience, if the list could not derive it (rule 93); the
//! flying boats' night torpedo attack (rule 97); that no force's fuel is
//! weighed and the oilers are left out (rule 95); what Wake's being on
//! the map does to each side's search (rule 82); and every ship whose
//! class the list marks as having surface weapons not modelled, by
//! name, read from the list.
//!
//! 26. The header says each of these.
//!
//! # Properties
//!
//! - **Nothing is invented.** Every ship and aircraft type of the file is
//!   of a class or type the list derives, but the transports and their
//!   class and the two stocks, each marked a placeholder.
//! - **The list's check passes,** and no figure of the scenario is typed
//!   by hand where the list gives it.
//! - **The same game twice** gives the same events.
//! - **Each side sees its own.** The Japanese side's first briefing names
//!   no American carrier, and the Allied side's names no Japanese force.
//!
//! # One game before the thousand
//!
//! The build plays file a once, at seed 1, and reports it to the
//! architect before any run of many games is read (rule 41): day by day,
//! what each staff ordered and why, what was found and when, every
//! strike with its losses, whether and when the transports reached
//! Midway and what landed, the score by line, what the classes whose
//! surface weapons are not modelled took away (rules 68 and 79), which
//! way each side's search looked and what Wake did to it (rules 82 and
//! 102), whether any ship ran out of fuel (rule 95), what Midway's stock
//! stood at each evening (rule 94), and how what happened compares with
//! what rules 101 and 102 expect. The band
//! of decision 0187 is read on 1,000 games of file a only after that
//! report is read, and nothing is changed to enter it.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]

use crate::common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde_json::{json, Value};
use sim_agent::SeatKind;
use sim_core::choices::Moment;
use sim_core::command::{CommandState, IntentKind, Maker, PlanState};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId, LandUnitId, ShipId, TaskForceId};
use sim_core::map::HexKind;
use sim_core::objectives::{score, Level, Objective, Outcome, Tally};
use sim_core::orders::{AirMission, Mission, SpeedSetting};
use sim_core::refdata::ShipKind;
use sim_core::state::{AirGroupLocation, Ship, ShipLocation, Side, TruthState};
use sim_core::time::Pulse;
use sim_core::World;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

const A: Side = Side::Allied;
const J: Side = Side::Japanese;
const LETTERS: [char; 4] = ['a', 'b', 'c', 'd'];

// ------------------------------------------------------------------ helpers

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn path(letter: char) -> PathBuf {
    common::data_dir().join(format!("scenarios/midway/01{letter}_midway.toml"))
}

/// The file resolved (decision 0234): a key, a row or a table is read from
/// the scenario it makes, whether the file holds it or the file it is built on.
fn text(letter: char) -> String {
    common::resolved_text(&path(letter))
}

fn reference(file: &str) -> String {
    std::fs::read_to_string(common::data_dir().join("reference").join(file))
        .unwrap_or_else(|e| panic!("{file} reads: {e}"))
}

/// Loaded on the live figures: the scenario runs on them.
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

/// The rows of a generated csv with the comment lines taken away.
fn rows(file: &str) -> Vec<Vec<String>> {
    reference(file)
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .skip(1)
        .map(|l| {
            // The class names are quoted and may hold commas; only the
            // columns before them are read here.
            l.split(',')
                .map(|c| c.trim_matches('"').to_string())
                .collect()
        })
        .collect()
}

/// The list's name (its slug) of a ship, from her scenario name: the last
/// word of the name is the last part of the slug ("USS Worden" is
/// "dd-352-worden"). None for a ship the list has not (Gwin is in the list
/// and not in the scenario; the transports are in neither).
fn slug_of(name: &str) -> Option<String> {
    let word = name.split_whitespace().last()?.to_lowercase();
    rows("ship_classes_midway.ships.csv")
        .iter()
        .map(|r| r[0].clone())
        .find(|slug| slug.rsplit('-').next() == Some(word.as_str()))
}

/// The class id the list gives a ship, by her scenario name.
fn list_class(name: &str) -> u32 {
    let slug = slug_of(name).unwrap_or_else(|| panic!("{name} is in the list"));
    rows("ship_classes_midway.ships.csv")
        .iter()
        .find(|r| r[0] == slug)
        .expect("the row")[1]
        .parse()
        .expect("a class id")
}

/// The experience the list's file gives a ship, if it gives one.
fn list_experience(name: &str) -> Option<u8> {
    let slug = slug_of(name)?;
    rows("ship_experience_midway.ships.csv")
        .iter()
        .find(|r| r[0] == slug)
        .map(|r| r[1].parse().expect("an experience"))
}

fn is_transport(s: &Ship) -> bool {
    s.name.starts_with("Placeholder transport")
}

/// The aircraft type of a number in the Midway list's file: its name.
fn type_name(w: &World, raw: u32) -> String {
    w.ref_data()
        .aircraft_types
        .values()
        .find(|t| t.id.raw() == raw)
        .unwrap_or_else(|| panic!("type {raw}"))
        .name
        .clone()
}

/// A game played to its end with the doctrine's staffs, as `run` and `batch`
/// play it.
struct Played {
    hashes: Vec<u64>,
    events: Vec<String>,
    after_day_1: BTreeMap<Side, CommandState>,
    /// The air groups' missions at the start of each pulse of days 1 and 2.
    missions: BTreeMap<(u32, u64), BTreeMap<u32, AirMission>>,
    /// At the start of the morning pulse of day 2: task forces 3 and 4's
    /// hex, route, speed setting, and the Japanese command state.
    day2: Option<Day2>,
    /// Task force 4's hex and the men ashore of land units 2 and 3 at the
    /// end of each day.
    convoy: Vec<(Hex, u32)>,
    end: World,
}

struct Day2 {
    at: BTreeMap<u32, Hex>,
    route: BTreeMap<u32, Vec<Hex>>,
    speed: BTreeMap<u32, SpeedSetting>,
    japanese: CommandState,
}

fn play(letter: char) -> Played {
    let mut scenario = load(letter);
    let held = staffs();
    let mut hashes = Vec::new();
    let mut after_day_1 = BTreeMap::new();
    let mut missions = BTreeMap::new();
    let mut day2 = None;
    let mut convoy = Vec::new();
    loop {
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                if let Moment::PulseBegins { day, pulse } = moment {
                    let t = world.umpire();
                    if day <= 2 {
                        missions.insert(
                            (day, pulse.code()),
                            t.air_groups
                                .iter()
                                .map(|(id, g)| (id.raw(), g.mission))
                                .collect::<BTreeMap<u32, AirMission>>(),
                        );
                    }
                    if day == 2 && pulse == Pulse::Morning {
                        day2 = Some(Day2 {
                            at: [3, 4]
                                .iter()
                                .map(|r| (*r, t.task_forces[&tf(*r)].position()))
                                .collect(),
                            route: [3, 4]
                                .iter()
                                .map(|r| (*r, t.task_forces[&tf(*r)].route.waypoints.clone()))
                                .collect(),
                            speed: [3, 4]
                                .iter()
                                .map(|r| (*r, t.task_forces[&tf(*r)].speed))
                                .collect(),
                            japanese: world.command_state(J).clone(),
                        });
                    }
                }
            })
            .expect("a day resolves");
        let Some(summary) = summary else { break };
        hashes.push(summary.state_hash);
        let t = scenario.world.umpire();
        convoy.push((
            t.task_forces[&tf(4)].position(),
            t.land_units[&unit(2)].men_ashore + t.land_units[&unit(3)].men_ashore,
        ));
        if summary.day == 1 {
            for side in Side::ALL {
                after_day_1.insert(side, scenario.world.command_state(side).clone());
            }
        }
    }
    Played {
        hashes,
        events: scenario
            .world
            .events()
            .iter()
            .map(|e| format!("{e:?}"))
            .collect(),
        after_day_1,
        missions,
        day2,
        convoy,
        end: scenario.world,
    }
}

/// One file played once, kept for the tests that read a game.
fn played(letter: char) -> &'static Played {
    static GAMES: [OnceLock<Played>; 4] = [
        OnceLock::new(),
        OnceLock::new(),
        OnceLock::new(),
        OnceLock::new(),
    ];
    let i = LETTERS.iter().position(|l| *l == letter).expect("a letter");
    GAMES[i].get_or_init(|| play(letter))
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

/// The handle a side's own tools give its unit of `name`, of the `kind`.
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

/// The air distance by the agent's measure, in nm, and the steps of the
/// hexes between (the hex distance, as the map counts it).
fn air_distance(s: &mut sim_agent::Session, side: Side, from: (i32, i32), to: (i32, i32)) -> u32 {
    let a = measure(
        s,
        side,
        json!({"question": "air_distance", "from": place(from.0, from.1), "to": place(to.0, to.1)}),
    );
    a["distance_nm"].as_u64().expect("miles") as u32
}

/// The sea route and its charged length between two hexes, by the game: the
/// charge in nm and the hexes of the way, the first being where it starts.
fn sea_route(
    s: &mut sim_agent::Session,
    side: Side,
    from: (i32, i32),
    to: (i32, i32),
) -> (u32, Vec<(i32, i32)>) {
    let a = measure(
        s,
        side,
        json!({"question": "sea_route", "from": place(from.0, from.1), "to": place(to.0, to.1)}),
    );
    assert_eq!(a["route_found"], json!(true), "{a}");
    let hexes = a["hexes"]
        .as_array()
        .expect("hexes")
        .iter()
        .map(|h| {
            (
                h["q"].as_i64().unwrap() as i32,
                h["r"].as_i64().unwrap() as i32,
            )
        })
        .collect();
    (a["distance_nm"].as_u64().expect("miles") as u32, hexes)
}

/// The header of the file and of every file it is built on, joined, as one
/// line of words.
fn header_words(letter: char) -> String {
    common::headers(&path(letter))
        .lines()
        .filter_map(|line| line.strip_prefix('#'))
        .map(|rest| rest.trim().to_string())
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

// ------------------------------------------------------------ The clock (1)

/// Statement 1: `days = 5`, and the header says the game begins at 1800 on 2
/// June 1942, that day 1 is 3 June, and that its times are zone +12.
#[test]
fn s01_five_days_from_1800_on_2_june_1942_with_day_1_the_3rd_and_zone_plus_12() {
    for letter in LETTERS {
        assert_eq!(load(letter).days, 5, "file {letter}");
        let t = text(letter);
        assert!(
            t.lines().any(|l| l == "days = 5"),
            "file {letter} says days = 5"
        );
        let header = header_words(letter);
        assert!(
            header.contains("The game begins at 1800 on 2 June 1942"),
            "file {letter}: {header}"
        );
        assert!(
            header.contains("day 1 is 3 June"),
            "file {letter}: {header}"
        );
        assert!(header.contains("zone +12"), "file {letter}: {header}");
    }
}

// ----------------------------------------------------- The map and the bases

/// Statement 2: Midway is a coast hex and Wake is, each by the map's mark of
/// land. (And the table's other cells: hexes, sides, the stocks, the
/// airfield, no anti-aircraft figure, no fuel.)
#[test]
fn s02_midway_and_wake_are_coast_hexes_where_the_table_puts_them() {
    for letter in LETTERS {
        let w = load(letter).world;
        let t = w.umpire();
        assert_eq!(t.bases.len(), 2, "file {letter}: two bases only");
        for (raw, at, side, name) in [(1, (5, 1), A, "Midway"), (2, (-24, 16), J, "Wake")] {
            let b = &t.bases[&base(raw)];
            assert_eq!(
                (b.name.as_str(), b.hex, b.side),
                (name, hex(at.0, at.1), side)
            );
            let data = w.map().get(b.hex).expect("the hex is on the map");
            assert_eq!(data.kind, HexKind::Coast, "{name} is a coast hex");
            assert_eq!(b.fuel_t, Fixed::ZERO, "{name} has no fuel for ships");
        }
        let midway = &t.bases[&base(1)];
        assert!(midway.airfield, "Midway has its airfield");
        assert_eq!(midway.anti_aircraft, 0, "no anti-aircraft figure: a gap");
        let wake = &t.bases[&base(2)];
        assert!(!wake.airfield, "Wake has no airfield");
        assert!(
            !t.air_groups
                .values()
                .any(|g| g.location == AirGroupLocation::Base(base(2))),
            "no aircraft at Wake"
        );
        assert!(
            !t.land_units.values().any(|u| u.hex == Some(hex(-24, 16))),
            "no men at Wake"
        );
    }
}

/// Statement 3: Midway has 4,000 t of supply and Wake 100 t, and the header
/// marks both placeholders and says why Wake is in: a lift needs a base of
/// its side, and the landing force in truth came from Saipan and Guam, off
/// the map.
#[test]
fn s03_midway_has_4000_t_and_wake_100_t_both_placeholders_and_the_header_says_why_wake_is_in() {
    for letter in LETTERS {
        let t = start(letter);
        assert_eq!(t.bases[&base(1)].supply_t, n(4000), "file {letter}");
        assert_eq!(t.bases[&base(2)].supply_t, n(100), "file {letter}");
        let header = header_words(letter);
        assert!(
            header.contains("100 t of supply, a PLACEHOLDER"),
            "file {letter}: {header}"
        );
        assert!(
            header.contains("Midway's stock and Wake's stock"),
            "file {letter}: both stocks are named among the placeholder data: {header}"
        );
        assert!(header.contains("WHY WAKE IS IN"), "file {letter}: {header}");
        assert!(
            header.contains("A lift needs a base of its side"),
            "file {letter}: {header}"
        );
        assert!(
            header.contains("Saipan and Guam, which are off the map"),
            "file {letter}: {header}"
        );
    }
}

/// Statement 4: No base has a port size, and no Japanese or Allied force has
/// a home port: Pearl Harbor, Japan and the Marianas are off the map.
#[test]
fn s04_no_base_has_a_port_size_and_no_force_a_home_port() {
    for letter in LETTERS {
        let w = load(letter).world;
        let t = w.umpire();
        for b in t.bases.values() {
            let data = w.map().get(b.hex).expect("on the map");
            assert_eq!(data.port_size, 0, "{} has no port size", b.name);
        }
        for f in t.task_forces.values() {
            assert_eq!(f.home_port, None, "{} has no home port", f.name);
        }
        let header = header_words(letter);
        assert!(
            header.contains("Pearl Harbor, Japan and the Marianas are off the map"),
            "file {letter}: {header}"
        );
    }
}

// ------------------------------------------------------ The Allied forces

/// Check each ship of `raw` ids: her name, her side, the class and the
/// experience the list gives her, and where she is.
fn ships_of_the_list(t: &TruthState, ids: std::ops::RangeInclusive<u32>, side: Side, force: u32) {
    for raw in ids {
        let s = &t.ships[&ship(raw)];
        assert_eq!(s.side, side, "ship {raw}");
        assert_eq!(s.location, ShipLocation::TaskForce(tf(force)), "{}", s.name);
        if is_transport(s) {
            assert_eq!(s.class.raw(), 19, "{}", s.name);
            continue;
        }
        assert_eq!(
            s.class.raw(),
            list_class(&s.name),
            "{}: the list's class",
            s.name
        );
        assert_eq!(
            s.experience,
            list_experience(&s.name).unwrap_or(Ship::DEFAULT_EXPERIENCE),
            "{}: the list's experience",
            s.name
        );
    }
}

/// Statement 5: Task force 1 has the 17 ships and task force 2 the 8, each
/// of the class the list gives her, and both start at (15,-6), which is deep
/// sea.
#[test]
fn s05_task_forces_1_and_2_have_their_17_and_8_ships_of_the_lists_classes_at_15_minus_6() {
    let names_1 = [
        "USS Enterprise",
        "USS Hornet",
        "USS Pensacola",
        "USS Northampton",
        "USS New Orleans",
        "USS Minneapolis",
        "USS Vincennes",
        "USS Atlanta",
        "USS Phelps",
        "USS Worden",
        "USS Monaghan",
        "USS Aylwin",
        "USS Balch",
        "USS Conyngham",
        "USS Benham",
        "USS Ellet",
        "USS Maury",
    ];
    let names_2 = [
        "USS Yorktown",
        "USS Astoria",
        "USS Portland",
        "USS Morris",
        "USS Hughes",
        "USS Anderson",
        "USS Hammann",
        "USS Russell",
    ];
    for letter in LETTERS {
        let w = load(letter).world;
        let t = w.umpire();
        for raw in [1, 2] {
            let f = &t.task_forces[&tf(raw)];
            assert_eq!(
                (f.side, f.mission),
                (A, Mission::Carrier),
                "task force {raw}"
            );
            assert_eq!(
                f.position(),
                hex(15, -6),
                "task force {raw} starts at (15,-6)"
            );
            assert_eq!(
                w.map().get(hex(15, -6)).expect("on the map").kind,
                HexKind::DeepSea
            );
        }
        let ships_of = |raw: u32| -> Vec<u32> {
            t.task_forces[&tf(raw)]
                .ships
                .iter()
                .map(|s| s.raw())
                .collect()
        };
        assert_eq!(ships_of(1), (1..=17).collect::<Vec<u32>>());
        assert_eq!(ships_of(2), (18..=25).collect::<Vec<u32>>());
        let named = |range: std::ops::RangeInclusive<u32>| -> Vec<&str> {
            range.map(|r| t.ships[&ship(r)].name.as_str()).collect()
        };
        assert_eq!(named(1..=17), names_1.to_vec());
        assert_eq!(named(18..=25), names_2.to_vec());
        ships_of_the_list(t, 1..=17, A, 1);
        ships_of_the_list(t, 18..=25, A, 2);
        // The classes are the list's own, 5005 to 5021.
        for raw in 1..=25 {
            assert!((5005..=5021).contains(&t.ships[&ship(raw)].class.raw()));
        }
    }
}

/// Statement 6: Enterprise has 27 + 18 + 18 + 14 = 77 aircraft, Hornet 27 +
/// 35 + 15 = 77, Yorktown 25 + 18 + 18 + 12 = 73, and Midway 21 + 7 + 19 +
/// 17 + 14 + 16 + 17 = 111: 338 Allied aircraft. No carrier has more than
/// her class holds.
#[test]
fn s06_the_allied_aircraft_are_77_77_73_and_111_338_in_all_and_no_carrier_over_her_class() {
    assert_eq!(27 + 18 + 18 + 14, 77);
    assert_eq!(27 + 35 + 15, 77);
    assert_eq!(25 + 18 + 18 + 12, 73);
    assert_eq!(21 + 7 + 19 + 17 + 14 + 16 + 17, 111);
    assert_eq!(77 + 77 + 73 + 111, 338);
    for letter in LETTERS {
        let w = load(letter).world;
        let t = w.umpire();
        let mut by_place: BTreeMap<String, u32> = BTreeMap::new();
        for g in t.air_groups.values().filter(|g| g.side == A) {
            let place = match g.location {
                AirGroupLocation::Ship(s) => t.ships[&s].name.clone(),
                AirGroupLocation::Base(b) => t.bases[&b].name.clone(),
                other => panic!("{} is {other:?}", g.name),
            };
            *by_place.entry(place).or_default() += g.ready + g.damaged;
        }
        assert_eq!(by_place["USS Enterprise"], 77, "file {letter}");
        assert_eq!(by_place["USS Hornet"], 77);
        assert_eq!(by_place["USS Yorktown"], 73);
        assert_eq!(by_place["Midway"], 111);
        assert_eq!(by_place.values().sum::<u32>(), 338);
        for (raw, held) in [(1, 77), (2, 77), (18, 73)] {
            let s = &t.ships[&ship(raw)];
            let cap = w.ref_data().ship_classes[&s.class].aircraft_capacity;
            assert!(
                held <= cap,
                "{} holds {held}, her class holds {cap}",
                s.name
            );
        }
    }
}

/// Statement 7: Every group has the type the table gives and is ready in full
/// at the start, but the SB2U-3 group, which has 12 ready and 5 damaged. The
/// groups are numbered in the order of the table, 1 to 18.
#[test]
fn s07_the_eighteen_allied_groups_are_of_the_tables_types_and_ready_but_for_five_sb2u_3() {
    // (group, name, aboard ship or at base, type, ready, damaged)
    let table: [(u32, &str, u32, bool, &str, u32, u32); 18] = [
        (1, "VF-6", 1, true, "F4F-4", 27, 0),
        (2, "VB-6", 1, true, "SBD-3", 18, 0),
        (3, "VS-6", 1, true, "SBD-3", 18, 0),
        (4, "VT-6", 1, true, "TBD-1", 14, 0),
        (5, "VF-8", 2, true, "F4F-4", 27, 0),
        (6, "VB-8 and VS-8", 2, true, "SBD-3", 35, 0),
        (7, "VT-8", 2, true, "TBD-1", 15, 0),
        (8, "VF-3", 18, true, "F4F-4", 25, 0),
        (9, "VB-3", 18, true, "SBD-3", 18, 0),
        (10, "VS-5", 18, true, "SBD-3", 18, 0),
        (11, "VT-3", 18, true, "TBD-1", 12, 0),
        (12, "VMF-221, F2A-3", 1, false, "F2A-3", 21, 0),
        (13, "VMF-221, F4F-3", 1, false, "F4F-3", 7, 0),
        (14, "VMSB-241, SBD-2", 1, false, "SBD-2", 19, 0),
        (15, "VMSB-241, SB2U-3", 1, false, "SB2U-3", 12, 5),
        (16, "Patrol squadrons, PBY-5", 1, false, "PBY-5 ", 14, 0),
        (17, "Patrol squadrons, PBY-5A", 1, false, "PBY-5A", 16, 0),
        (18, "Army bombers, B-17E", 1, false, "B-17E", 17, 0),
    ];
    for letter in LETTERS {
        let w = load(letter).world;
        let t = w.umpire();
        assert_eq!(
            t.air_groups.values().filter(|g| g.side == A).count(),
            18,
            "no other Allied group"
        );
        for (raw, name, at, aboard, kind, ready, damaged) in table {
            let g = &t.air_groups[&group(raw)];
            assert_eq!(g.name, name, "group {raw}");
            assert_eq!(g.side, A, "{name}");
            let want = if aboard {
                AirGroupLocation::Ship(ship(at))
            } else {
                AirGroupLocation::Base(base(at))
            };
            assert_eq!(g.location, want, "{name}");
            let full = type_name(&w, g.aircraft_type.raw());
            // "PBY-5 " with a space is the PBY-5 and not the PBY-5A.
            assert!(
                if kind == "PBY-5 " {
                    full.contains("PBY-5 ")
                } else {
                    full.contains(kind)
                },
                "{name} flies the {kind}, not {full}"
            );
            assert_eq!((g.ready, g.damaged), (ready, damaged), "{name}");
        }
    }
}

/// Statement 8: Land unit 1 has 2,178 men ashore at (5,1).
#[test]
fn s08_land_unit_1_has_2178_men_ashore_at_5_1() {
    for letter in LETTERS {
        let t = start(letter);
        let u = &t.land_units[&unit(1)];
        assert_eq!(
            (u.side, u.men_ashore, u.hex),
            (A, 2178, Some(hex(5, 1))),
            "file {letter}"
        );
    }
}

// ------------------------------------------------ The Japanese forces

/// Statement 9: The two forces have 21 and 46 ships, 67 in all, each warship
/// of the class the list gives her, and start at (-2,-15) and (-17,4), both
/// deep sea. Every ship of the scenario, of either side, starts with her
/// fuel full.
#[test]
fn s09_task_forces_3_and_4_have_21_and_46_ships_of_the_lists_classes_with_fuel_full() {
    for letter in LETTERS {
        let w = load(letter).world;
        let t = w.umpire();
        for (raw, at, count, mission) in [
            (3, (-2, -15), 21, Mission::Carrier),
            (4, (-17, 4), 46, Mission::Transport),
        ] {
            let f = &t.task_forces[&tf(raw)];
            assert_eq!((f.side, f.mission), (J, mission), "task force {raw}");
            assert_eq!(f.position(), hex(at.0, at.1), "task force {raw}");
            assert_eq!(f.ships.len(), count, "task force {raw}");
            assert_eq!(
                w.map().get(hex(at.0, at.1)).expect("on the map").kind,
                HexKind::DeepSea
            );
        }
        assert_eq!(21 + 46, 67);
        assert_eq!(t.ships.values().filter(|s| s.side == J).count(), 67);
        let ships_of = |raw: u32| -> Vec<u32> {
            t.task_forces[&tf(raw)]
                .ships
                .iter()
                .map(|s| s.raw())
                .collect()
        };
        assert_eq!(ships_of(3), (26..=46).collect::<Vec<u32>>());
        assert_eq!(ships_of(4), (47..=92).collect::<Vec<u32>>());
        ships_of_the_list(t, 26..=46, J, 3);
        ships_of_the_list(t, 47..=92, J, 4);
        // Every warship's class is the list's: 5000 to 5003 and 5022 to 5035.
        for s in t.ships.values().filter(|s| s.side == J && !is_transport(s)) {
            let c = s.class.raw();
            assert!(
                (5000..=5003).contains(&c) || (5022..=5035).contains(&c),
                "{} class {c}",
                s.name
            );
        }
        for s in t.ships.values() {
            let class = &w.ref_data().ship_classes[&s.class];
            assert_eq!(s.fuel_t, class.fuel_capacity_t, "{} starts full", s.name);
        }
    }
}

/// Statement 10: The header names the thirteen ships the placeholder
/// transports stand for, says the thirteenth is disputed, and says that the
/// night surface threat of the Second Fleet's main body is not in the game.
#[test]
fn s10_the_header_names_the_thirteen_transports_ships_and_the_dispute_and_the_night_threat() {
    for letter in LETTERS {
        let header = header_words(letter);
        for name in [
            "Nankai Maru",
            "Zenyo Maru",
            "Brazil Maru",
            "Argentina Maru",
            "Azuma Maru",
            "Kirishima Maru",
            "Kano Maru",
            "Toa Maru No. 2",
            "Akebono Maru",
            "Hokuriku Maru",
            "Goshu Maru",
            "Keiyo Maru",
            "Seicho Maru",
        ] {
            assert!(header.contains(name), "file {letter}: {name}");
        }
        assert!(
            header.contains("Which was the thirteenth is disputed"),
            "file {letter}: {header}"
        );
        assert!(
            header.contains("Kiyosumi Maru"),
            "file {letter}: the other candidate"
        );
        assert!(
            header.contains("night surface threat of the Second Fleet's main body"),
            "file {letter}"
        );
        assert!(
            header.contains("is not in the game"),
            "file {letter}: the threat is not in the game"
        );
    }
}

/// Statement 11: The four carriers have 60, 72, 55 and 57 aircraft: fighters
/// 24 + 27 + 21 + 21 = 93, bombers 18 + 18 + 16 + 18 = 70, attack planes 18
/// + 27 + 18 + 18 = 81, 244 in all. The two cruisers have 6: 250 Japanese
/// aircraft in 14 groups, numbered 19 to 32. No ship has more than her class
/// holds.
#[test]
fn s11_the_japanese_aircraft_are_60_72_55_57_and_6_250_in_14_groups_numbered_19_to_32() {
    assert_eq!(24 + 27 + 21 + 21, 93);
    assert_eq!(18 + 18 + 16 + 18, 70);
    assert_eq!(18 + 27 + 18 + 18, 81);
    assert_eq!(93 + 70 + 81, 244);
    assert_eq!(244 + 6, 250);
    // (group, name, carrier ship, type fragment, ready)
    let table: [(u32, &str, u32, &str, u32); 14] = [
        (19, "Akagi fighter unit", 26, "A6M2", 24),
        (20, "Akagi bomber unit", 26, "D3A1", 18),
        (21, "Akagi attack unit", 26, "B5N2", 18),
        (22, "Kaga fighter unit", 27, "A6M2", 27),
        (23, "Kaga bomber unit", 27, "D3A1", 18),
        (24, "Kaga attack unit", 27, "B5N2", 27),
        (25, "Soryu fighter unit", 28, "A6M2", 21),
        (26, "Soryu bomber unit", 28, "D3A1", 16),
        (27, "Soryu attack unit", 28, "B5N2", 18),
        (28, "Hiryu fighter unit", 29, "A6M2", 21),
        (29, "Hiryu bomber unit", 29, "D3A1", 18),
        (30, "Hiryu attack unit", 29, "B5N2", 18),
        (31, "Tone floatplanes", 32, "E13A1", 3),
        (32, "Chikuma floatplanes", 33, "E13A1", 3),
    ];
    for letter in LETTERS {
        let w = load(letter).world;
        let t = w.umpire();
        let mut aboard: BTreeMap<u32, u32> = BTreeMap::new();
        let mut kinds: BTreeMap<&str, u32> = BTreeMap::new();
        for (raw, name, carrier, kind, ready) in table {
            let g = &t.air_groups[&group(raw)];
            assert_eq!(g.name, name);
            assert_eq!(
                (g.side, g.location, g.ready, g.damaged),
                (J, AirGroupLocation::Ship(ship(carrier)), ready, 0),
                "{name}"
            );
            assert!(
                type_name(&w, g.aircraft_type.raw()).contains(kind),
                "{name} flies the {kind}"
            );
            *aboard.entry(carrier).or_default() += ready;
            if kind != "E13A1" {
                *kinds.entry(kind).or_default() += ready;
            }
        }
        assert_eq!(t.air_groups.values().filter(|g| g.side == J).count(), 14);
        assert_eq!(
            t.air_groups
                .keys()
                .filter(|id| id.raw() >= 19)
                .map(|id| id.raw())
                .collect::<Vec<u32>>(),
            (19..=32).collect::<Vec<u32>>()
        );
        assert_eq!(
            [26, 27, 28, 29].map(|r| aboard[&r]),
            [60, 72, 55, 57],
            "file {letter}"
        );
        assert_eq!(aboard[&32] + aboard[&33], 6);
        assert_eq!(kinds["A6M2"], 93);
        assert_eq!(kinds["D3A1"], 70);
        assert_eq!(kinds["B5N2"], 81);
        assert_eq!(aboard.values().sum::<u32>(), 250);
        for (raw, held) in aboard {
            let s = &t.ships[&ship(raw)];
            let cap = w.ref_data().ship_classes[&s.class].aircraft_capacity;
            assert!(
                held <= cap,
                "{} holds {held}, her class holds {cap}",
                s.name
            );
        }
    }
}

/// Statement 12: The two units are aboard the transports of task force 4 with
/// 2,507 and 2,000 men, 4,507 in all, and no man of them is ashore.
#[test]
fn s12_the_two_land_units_are_aboard_the_transports_with_2507_and_2000_men_none_ashore() {
    for letter in LETTERS {
        let t = start(letter);
        assert_eq!(t.land_units.len(), 3, "three land units");
        for (raw, men) in [(2, 2507), (3, 2000)] {
            let u = &t.land_units[&unit(raw)];
            assert_eq!(
                (u.side, u.men_ashore, u.hex),
                (J, 0, None),
                "unit {raw} is aboard"
            );
            let aboard: u32 = t
                .ships
                .values()
                .filter_map(|s| s.troops.map(|tr| (s, tr)))
                .filter(|(_, tr)| tr.unit.raw() == raw)
                .map(|(s, tr)| {
                    assert!(is_transport(s), "{} carries men", s.name);
                    assert_eq!(s.location, ShipLocation::TaskForce(tf(4)));
                    tr.men
                })
                .sum();
            assert_eq!(aboard, men, "land unit {raw}");
        }
        let all: u32 = t
            .ships
            .values()
            .filter_map(|s| s.troops)
            .map(|tr| tr.men)
            .sum();
        assert_eq!(all, 4507, "file {letter}");
        assert_eq!(2507 + 2000, 4507);
        assert!(
            t.ships
                .values()
                .filter(|s| !is_transport(s))
                .all(|s| s.troops.is_none()),
            "no warship carries a man"
        );
    }
}

// -------------------------------------------------------- The transports

/// Statement 13: The thirteen transports are of that class, 600 berths each,
/// 7,800 in all for 4,507 men, each with 41 t of supply, and the file says
/// beside the speed that it stands on the two positions alone.
#[test]
fn s13_thirteen_transports_of_class_19_600_berths_each_41_t_of_supply_8_8_kn() {
    for letter in LETTERS {
        let w = load(letter).world;
        let t = w.umpire();
        let class = &w.ref_data().ship_classes[&sim_core::ids::ShipClassId::new(19)];
        assert_eq!(class.name, "Placeholder transport, 8.8 knots");
        assert_eq!(
            (class.cruise_speed_kn, class.max_speed_kn),
            (fx("8.8"), fx("8.8"))
        );
        assert_eq!(class.troop_capacity_men, 600);
        assert_eq!(class.main_guns, 0, "no guns");
        let transports: Vec<&Ship> = t.ships.values().filter(|s| is_transport(s)).collect();
        assert_eq!(transports.len(), 13, "file {letter}");
        for (i, s) in transports.iter().enumerate() {
            assert_eq!(s.name, format!("Placeholder transport {}", i + 1));
            assert_eq!(s.class.raw(), 19);
            assert_eq!(s.supply_t, n(41), "{}", s.name);
        }
        let berths: u32 = transports
            .iter()
            .map(|s| w.ref_data().ship_classes[&s.class].troop_capacity_men)
            .sum();
        assert_eq!(berths, 7800);
        assert_eq!(13 * 600, 7800);
        assert!(berths > 4507);
        // 41 t each.
        let supply = transports.iter().fold(Fixed::ZERO, |a, s| a + s.supply_t);
        assert_eq!(supply, n(13 * 41));
        // The file says beside the speed that it stands on two positions
        // alone; so does the class's file.
        let header = header_words(letter);
        assert!(
            header.contains("8.8 kn, stands on two positions alone"),
            "file {letter}: {header}"
        );
        assert!(
            reference("ship_classes_placeholder_transport_8_8_knots.csv")
                .contains("stands on two positions alone"),
            "the class's file says it too"
        );
    }
}

/// Statement 14: From (-17,4) to Midway's hex the sea way is 22 steps,
/// charged to a ship as 827 nm, with no land in the way. At 8.8 kn that is
/// 94 hours less a minute: a force that sails at 1800 on 2 June and holds
/// 8.8 kn reaches Midway's hex at about 1600 on 6 June, in the afternoon of
/// day 4.
#[test]
fn s14_the_sea_way_to_midway_is_22_steps_827_nm_and_94_hours_less_a_minute() {
    let mut s = session('d', &[J]);
    let (miles, route) = sea_route(&mut s, J, (-17, 4), (5, 1));
    // The route lists the hexes after the start, as scenario 105's does.
    assert_eq!(route.last(), Some(&(5, 1)));
    assert_eq!(route.len(), 22, "22 steps: {route:?}");
    assert_eq!(miles, 827, "{route:?}");
    // By the map, no land in the way.
    let w = load('d').world;
    for (q, r) in &route {
        let kind = w.map().get(hex(*q, *r)).expect("on the map").kind;
        assert_ne!(kind, HexKind::Land, "({q},{r}) is land");
    }
    // At 8.8 kn: 827 / 8.8 = 93.977 hours, 93 hours 59 minutes (94 hours
    // less a minute), by the game's own measure of the passage.
    let force = handle(&mut s, J, "task_force", "Midway Invasion Force");
    let a = measure(
        &mut s,
        J,
        json!({"question": "passage", "task_force": force, "to": place(5, 1)}),
    );
    assert_eq!(a["speed"]["knots"], json!("8.8"), "{a}");
    assert_eq!(a["route_nm"], json!("827"), "{a}");
    assert_eq!(a["outcome"], json!("arrives"), "{a}");
    let hours: f64 = a["steaming_hours"]
        .as_str()
        .expect("hours")
        .parse()
        .expect("a decimal");
    assert!((hours - 827.0 / 8.8).abs() < 0.0001, "{hours}");
    assert_eq!((hours * 60.0).round() as i64, 94 * 60 - 1, "{a}");
    // About 1600 on 6 June: day 4, the afternoon pulse.
    assert_eq!(a["ends"]["day"], json!(4), "{a}");
    assert_eq!(a["ends"]["pulse"], json!("afternoon"), "{a}");
    assert_eq!(a["ends"]["clock"], json!("D4 15:59"), "{a}");
}

/// Statement 15: The force's speed is the transports': no warship of it
/// cruises slower than 8.8 kn.
#[test]
fn s15_the_forces_speed_is_the_transports_8_8_kn_and_no_warship_is_slower() {
    for letter in LETTERS {
        let w = load(letter).world;
        let t = w.umpire();
        let mut slowest = None;
        for raw in t.task_forces[&tf(4)].ships.iter() {
            let s = &t.ships[raw];
            let class = &w.ref_data().ship_classes[&s.class];
            assert!(
                class.cruise_speed_kn >= fx("8.8"),
                "{} cruises at {}",
                s.name,
                class.cruise_speed_kn
            );
            if is_transport(s) {
                assert_eq!(class.cruise_speed_kn, fx("8.8"));
            }
            slowest = Some(slowest.map_or(class.cruise_speed_kn, |m: Fixed| {
                m.min(class.cruise_speed_kn)
            }));
        }
        assert_eq!(
            slowest,
            Some(fx("8.8")),
            "the force cruises at the transports'"
        );
    }
}

// ---------------------------------------------------------------- Distances

/// Statement 16: By the map: from task force 3's start to Midway 23 steps,
/// 817 nm by air; from the American carriers' start to Midway 10 steps, 356
/// nm; from Wake to Midway 29 steps; from task force 3's start to (5,-3),
/// four hexes north of Midway, 19 steps, charged as 666 nm.
#[test]
fn s16_distances_by_the_map() {
    let mut a = session('d', &[A]);
    let mut j = session('d', &[J]);
    assert_eq!(hex(-2, -15).distance(hex(5, 1)), 23);
    assert_eq!(air_distance(&mut j, J, (-2, -15), (5, 1)), 817);
    assert_eq!(hex(-2, -15).air_distance_nm(hex(5, 1)), 817);
    assert_eq!(hex(15, -6).distance(hex(5, 1)), 10);
    assert_eq!(air_distance(&mut a, A, (15, -6), (5, 1)), 356);
    assert_eq!(hex(15, -6).air_distance_nm(hex(5, 1)), 356);
    assert_eq!(hex(-24, 16).distance(hex(5, 1)), 29);
    assert_eq!(
        hex(5, -3).distance(hex(5, 1)),
        4,
        "four hexes north of Midway"
    );
    assert_eq!(hex(-2, -15).distance(hex(5, -3)), 19);
    let (miles, route) = sea_route(&mut j, J, (-2, -15), (5, -3));
    assert_eq!(route.len(), 19, "{route:?}");
    assert_eq!(miles, 666, "{route:?}");
}

// ----------------------------------- What a side the engine plays is given

fn intent(state: &CommandState, raw: u32) -> &sim_core::command::Intent {
    &state.intents[&IntentId::new(raw)]
}

fn commander_intents(state: &CommandState) -> BTreeSet<u32> {
    state
        .intents
        .values()
        .filter(|i| i.maker == Maker::Commander)
        .map(|i| i.id.raw())
        .collect()
}

/// Statement 17: Each order is accepted on day 1 as given, and none is
/// refused.
#[test]
fn s17_each_order_is_accepted_on_day_1_as_given_and_none_is_refused() {
    let a = played('a');
    let refused: Vec<String> = a
        .end
        .events()
        .iter()
        .filter(|e| matches!(e.kind, EventKind::CommandRefused { .. }))
        .map(|e| format!("{:?}", e.kind))
        .collect();
    assert!(refused.is_empty(), "refused: {refused:?}");
    // Allied: 4 intents, 3 limits (the forces of 1 and 2, the targets of 2),
    // 2 leaves: 9. Japanese: 1 intent, 2 limits (forces, knowledge), 2
    // leaves: 5.
    let taken = |side: Side| {
        a.end
            .events()
            .iter()
            .filter(|e| e.day == 1)
            .filter(|e| match &e.kind {
                EventKind::CommandTaken {
                    side: s, issuer, ..
                } => *s == side && *issuer == sim_core::orders::Issuer::Player,
                _ => false,
            })
            .count()
    };
    assert_eq!(taken(A), 9, "Allied commands taken on day 1");
    assert_eq!(taken(J), 5, "Japanese commands taken on day 1");
    // The state holds each as given.
    let state = &a.after_day_1[&J];
    assert_eq!(commander_intents(state), BTreeSet::from([1]));
    assert_eq!(
        intent(state, 1).kind,
        IntentKind::TakePlace {
            hex: hex(5, 1),
            by_day: None
        }
    );
    let f = intent(state, 1)
        .limits
        .forces
        .as_ref()
        .expect("a forces limit");
    assert_eq!(f.task_forces, BTreeSet::from([tf(3), tf(4)]));
    assert_eq!(f.land_units, BTreeSet::from([unit(2), unit(3)]));
    assert!(f.air_groups.is_empty() && f.ships.is_empty() && f.source.is_none());
    assert_eq!(intent(state, 1).limits.risk, None, "no risk set: balanced");
    assert_eq!(intent(state, 1).set_day, 1);
    assert!(state.leaves.prepare && state.leaves.launch);

    let state = &a.after_day_1[&A];
    assert_eq!(commander_intents(state), BTreeSet::from([1, 2, 3, 4]));
    assert_eq!(intent(state, 1).kind, IntentKind::Hold { base: base(1) });
    let f = intent(state, 1)
        .limits
        .forces
        .as_ref()
        .expect("a forces limit");
    assert_eq!(f.land_units, BTreeSet::from([unit(1)]));
    assert_eq!(
        f.air_groups,
        BTreeSet::from([group(12), group(13), group(16), group(17)])
    );
    assert!(f.task_forces.is_empty() && f.ships.is_empty());
    assert_eq!(
        intent(state, 2).kind,
        IntentKind::Deny {
            hex: hex(1, -1),
            distance: 4
        }
    );
    let f = intent(state, 2)
        .limits
        .forces
        .as_ref()
        .expect("a forces limit");
    assert_eq!(
        f.air_groups,
        BTreeSet::from([group(14), group(15), group(18)])
    );
    let t = intent(state, 2)
        .limits
        .targets
        .as_ref()
        .expect("a targets limit");
    assert_eq!(
        t.kinds,
        vec![
            ShipKind::Carrier,
            ShipKind::Transport,
            ShipKind::Battleship,
            ShipKind::Cruiser,
            ShipKind::Destroyer
        ]
    );
    assert_eq!(
        intent(state, 3).kind,
        IntentKind::OperateCarriers {
            force: tf(1),
            hex: hex(9, -5)
        }
    );
    assert_eq!(
        intent(state, 4).kind,
        IntentKind::OperateCarriers {
            force: tf(2),
            hex: hex(9, -5)
        }
    );
    for raw in 1..=4 {
        assert_eq!(intent(state, raw).limits.risk, None, "balanced");
        assert_eq!(intent(state, raw).set_day, 1);
    }
    assert!(state.leaves.prepare && state.leaves.launch);
}

/// Statement 18: In the file in which the Japanese side is given its orders,
/// by the morning of day 2 the staff has formed a plan under the take; task
/// force 4 has sailed for Midway at 8.8 kn; and task force 3 is at, or under
/// way to, a hex no more than 4 hexes from Midway.
#[test]
fn s18_by_the_morning_of_day_2_a_plan_under_the_take_task_force_4_sailed_and_task_force_3_near() {
    for letter in ['a', 'b'] {
        let p = played(letter);
        let d = p.day2.as_ref().expect("the morning of day 2 is reached");
        let plans: Vec<_> = d
            .japanese
            .plans
            .values()
            .filter(|pl| pl.intent == IntentId::new(1))
            .collect();
        assert!(
            !plans.is_empty(),
            "file {letter}: the staff formed no plan under the take: {:?}",
            d.japanese.plans
        );
        assert!(
            plans
                .iter()
                .any(|pl| !matches!(pl.state, PlanState::Failed | PlanState::Cancelled)),
            "file {letter}: plans {:?}",
            plans.iter().map(|pl| pl.state).collect::<Vec<_>>()
        );
        // Task force 4 has sailed for Midway at 8.8 kn.
        // A lift's route is Midway, then home to Wake: it is bound for Midway
        // first.
        let route4 = &d.route[&4];
        assert_eq!(
            route4.first(),
            Some(&hex(5, 1)),
            "file {letter}: task force 4 is bound for Midway: at {:?}, route {route4:?}",
            d.at[&4]
        );
        assert!(
            d.at[&4] != hex(-17, 4),
            "file {letter}: task force 4 has left (-17,4)"
        );
        match d.speed[&4] {
            SpeedSetting::Cruise | SpeedSetting::Full => {}
            SpeedSetting::Knots(k) => assert!(k >= fx("8.8"), "{k}"),
        }
        // Task force 3 is at, or under way to, a hex within 4 of Midway.
        let near = |h: Hex| h.distance(hex(5, 1)) <= 4;
        assert!(
            near(d.at[&3]) || d.route[&3].last().is_some_and(|h| near(*h)),
            "file {letter}: task force 3 at {:?}, route {:?}",
            d.at[&3],
            d.route[&3]
        );
    }
}

/// Statement 19: In the file in which the Allied side is given its orders, on
/// day 1 a fighter group patrols over Midway, both groups of flying boats
/// search, no flying boat is sent to strike, and the hexes (5,-3), (-3,1) and
/// (1,3) are within the waters denied.
#[test]
fn s19_a_fighter_patrols_both_flying_boat_groups_search_none_strikes_and_the_waters_cover_three_hexes(
) {
    for letter in ['a', 'c'] {
        let p = played(letter);
        let day1: Vec<&BTreeMap<u32, AirMission>> =
            [Pulse::Night, Pulse::Morning, Pulse::Afternoon]
                .iter()
                .map(|pl| &p.missions[&(1, pl.code())])
                .collect();
        let ever = |raw: u32, mission: AirMission| day1.iter().any(|m| m[&raw] == mission);
        assert!(
            ever(12, AirMission::CombatAirPatrol) || ever(13, AirMission::CombatAirPatrol),
            "file {letter}: a fighter group patrols: {:?}",
            day1.iter().map(|m| (m[&12], m[&13])).collect::<Vec<_>>()
        );
        for raw in [16, 17] {
            assert!(
                ever(raw, AirMission::Search),
                "file {letter}: flying boat group {raw} searches: {:?}",
                day1.iter().map(|m| m[&raw]).collect::<Vec<_>>()
            );
            for mission in [AirMission::NavalStrike, AirMission::BaseStrike(base(2))] {
                assert!(
                    !ever(raw, mission),
                    "file {letter}: flying boat group {raw} is sent to strike"
                );
            }
            assert!(
                !day1.iter().any(|m| matches!(
                    m[&raw],
                    AirMission::NavalStrike | AirMission::BaseStrike(_)
                )),
                "file {letter}: no flying boat is sent to strike"
            );
        }
    }
    // The waters denied are the Allied Deny intent's: within 4 hexes of
    // (1,-1). (5,-3), (-3,1) and (1,3) are each 4 hexes from it.
    let state = &played('a').after_day_1[&A];
    let IntentKind::Deny {
        hex: centre,
        distance,
    } = intent(state, 2).kind
    else {
        panic!("intent 2 is not a deny");
    };
    assert_eq!((centre, distance), (hex(1, -1), 4));
    for (q, r) in [(5, -3), (-3, 1), (1, 3)] {
        assert!(
            hex(q, r).distance(centre) <= distance,
            "({q},{r}) is within {distance} of {centre}"
        );
    }
}

// -------------------------------------------------------------- The files

/// Statement 20: The four files differ from the base only in the orders (and
/// their name and header). That each states its name and commands and nothing
/// else is statement R3 of `scenarios_built_on_another`.
#[test]
fn s20_the_four_files_differ_from_the_base_only_in_name_header_and_orders() {
    for letter in LETTERS {
        assert_eq!(load(letter).days, 5, "file {letter}");
    }
    assert!(
        !text('d')
            .lines()
            .any(|line| line == "[[command]]" || line == "[[order]]"),
        "file d has no command or order"
    );
    for letter in ['a', 'b', 'c'] {
        assert!(
            text(letter).lines().any(|line| line == "[[command]]"),
            "{letter}"
        );
        assert!(
            !text(letter).lines().any(|line| line == "[[order]]"),
            "{letter}: no scripted order"
        );
    }
    let names: BTreeSet<String> = LETTERS.iter().map(|l| load(*l).name).collect();
    assert_eq!(names.len(), 4);
}

/// Statement 21: Each has the key that marks a scenario for play, and
/// scenario 104's statement of which files have it gains these four and the
/// table's copies of b, c and d.
#[test]
fn s21_each_file_has_the_key_and_so_have_the_tables_copies_of_b_c_and_d() {
    for letter in LETTERS {
        assert!(
            load(letter)
                .world
                .objectives()
                .expect("objectives")
                .for_play,
            "file {letter}"
        );
        assert!(text(letter).lines().any(|l| l == "for_play = true"));
    }
    for letter in ['b', 'c', 'd'] {
        let copy =
            common::data_dir().join(format!("scenarios/table-hunger/01{letter}_midway.toml"));
        let t = common::resolved_text(&copy);
        assert!(t.lines().any(|l| l == "for_play = true"), "copy {letter}");
        let loaded = common::load_live(&copy).expect("the table's copy loads");
        assert!(loaded.world.objectives().expect("objectives").for_play);
    }
    // Scenario 104's statement 23 lists them: its test, with the base
    // excluded and the copies named, is the architect's restatement.
    let s104 = std::fs::read_to_string(repo().join("tests/scenario_104_men_put_ashore.rs"))
        .expect("scenario 104 reads");
    assert!(s104.contains("scenarios/midway/01{l}_midway.toml"));
    assert!(s104.contains("scenarios/table-hunger/01{l}_midway.toml"));
    assert!(s104.contains("base_midway.toml"));
}

/// Statement 22: In files c and d, where the Japanese side is given nothing,
/// task force 4 does not move and no man lands in five days.
#[test]
fn s22_where_the_japanese_side_is_given_nothing_task_force_4_does_not_move_and_no_man_lands() {
    for letter in ['c', 'd'] {
        let p = played(letter);
        assert_eq!(p.convoy.len(), 5);
        for (day, (at, ashore)) in p.convoy.iter().enumerate() {
            assert_eq!(*at, hex(-17, 4), "file {letter} day {}", day + 1);
            assert_eq!(*ashore, 0, "file {letter} day {}: no man ashore", day + 1);
        }
        let t = p.end.umpire();
        assert!(t.task_forces[&tf(4)].route.waypoints.is_empty());
        assert_eq!(t.land_units[&unit(2)].men_ashore, 0);
        assert_eq!(t.land_units[&unit(3)].men_ashore, 0);
    }
}

// ---------------------------------------------------------- The scoring

/// Statement 23: The file's objectives are those figures, levels and horizon,
/// and mark it for play.
#[test]
fn s23_the_objectives_are_the_table_over_66_days_for_play() {
    for letter in LETTERS {
        let w = load(letter).world;
        let o = w.objectives().expect("objectives");
        assert!(o.for_play, "a scenario for play");
        assert_eq!(
            (o.levels.minor, o.levels.major, o.levels.decisive),
            (n(5), n(15), n(30))
        );
        assert_eq!(o.horizon_days, 66);
        assert_eq!(28 + 31 + 7, 66);
        assert!(!o.allied.mission.is_empty() && !o.japanese.mission.is_empty());
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
            ];
            if side == J {
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
                    Objective::MenLanded {
                        base: b,
                        points_per_100_men,
                    } => {
                        assert_eq!(side, J, "only the Japanese side lands men");
                        assert_eq!((*b, *points_per_100_men), (base(1), n(1)));
                    }
                    other => panic!("an objective the table has not: {}", other.name()),
                }
            }
        }
    }
}

/// Statement 24: What is at stake, by those figures, through the game's own
/// scoring. The four fleet carriers sunk are 40 points. The thirteen
/// transports sunk with every man aboard are 13 x 2 + 45.07 = 71.07. Every
/// man of the landing force landed is 45.07 to the Japanese side. The three
/// American carriers sunk are 30. Midway's garrison lost to the last man is
/// 21.78.
#[test]
fn s24_what_is_at_stake_by_the_games_own_scoring() {
    let w = load('a').world;
    let o = w.objectives().expect("objectives");
    let play = |a: Tally, j: Tally| score(o, &BTreeMap::from([(A, a), (J, j)]));
    let sunk = |kind: ShipKind, count: u32| BTreeMap::from([(kind, count)]);

    // The four fleet carriers sunk.
    let r = play(
        Tally {
            sunk: sunk(ShipKind::Carrier, 4),
            ..Tally::default()
        },
        Tally::default(),
    );
    assert_eq!(r.allied.total, n(40));
    assert_eq!(r.japanese.total, Fixed::ZERO);

    // The thirteen transports sunk with every man aboard: 13 x 2 + 45.07.
    let r = play(
        Tally {
            sunk: sunk(ShipKind::Transport, 13),
            men_lost: 4507,
            ..Tally::default()
        },
        Tally::default(),
    );
    assert_eq!(r.allied.total, fx("71.07"));
    assert_eq!(n(13 * 2) + fx("45.07"), fx("71.07"));

    // Every man of the landing force landed.
    let mut landed = Tally::default();
    landed.men_landed.insert(base(1), 4507);
    let r = play(Tally::default(), landed);
    assert_eq!(r.japanese.total, fx("45.07"));
    assert_eq!(r.allied.total, Fixed::ZERO);
    assert_eq!(r.japanese.outcome, Outcome::Victory(Level::Decisive));

    // The three American carriers sunk.
    let r = play(
        Tally::default(),
        Tally {
            sunk: sunk(ShipKind::Carrier, 3),
            ..Tally::default()
        },
    );
    assert_eq!(r.japanese.total, n(30));

    // Midway's garrison lost to the last man.
    let r = play(
        Tally::default(),
        Tally {
            men_lost: 2178,
            ..Tally::default()
        },
    );
    assert_eq!(r.japanese.total, fx("21.78"));
}

/// Statement 25: The history, as the file's objectives state it for the end
/// paper: four Japanese carriers and one cruiser sunk, one American carrier
/// and one destroyer sunk, and no man landed.
#[test]
fn s25_the_history_is_four_carriers_and_a_cruiser_one_carrier_and_a_destroyer_no_man_landed() {
    for letter in LETTERS {
        let w = load(letter).world;
        let history = w
            .objectives()
            .expect("objectives")
            .history
            .clone()
            .expect("a history note");
        for words in [
            "Akagi, Kaga, Soryu and Hiryu",
            "cruiser Mikuma",
            "were sunk",
            "Yorktown",
            "destroyer Hammann",
            "were lost",
            "landed no one at Midway",
        ] {
            assert!(history.contains(words), "{words}: {history}");
        }
    }
}

// ------------------------------------- What the game does not model

/// Statement 26: The header says each of these: rules 16, 17 and 42 to 45,
/// the night surface threat, ships' experience if the list could not derive
/// it, the flying boats' night torpedo attack, that no force's fuel is
/// weighed and the oilers are left out, what Wake's being on the map does to
/// each side's search, and every ship whose class the list marks as having
/// surface weapons not modelled, by name, read from the list.
#[test]
fn s26_the_header_says_what_the_game_does_not_model() {
    // The ships of the list marked not modelled, by the names of the
    // scenario: the list's slugs (Gwin, not in the scenario, is among them).
    let list = std::fs::read_to_string(repo().join("data/derivation/midway.toml"))
        .expect("the list reads");
    let start = list
        .find("surface_weapons_not_modelled = [")
        .expect("the list's key");
    let block = &list[start..];
    let block = &block[..block.find(']').expect("the end of the list")];
    let slugs: Vec<&str> = block.split('"').skip(1).step_by(2).collect();
    assert!(slugs.len() > 50, "{} slugs", slugs.len());
    let t = start_truth();
    let not_modelled: Vec<String> = slugs
        .iter()
        .filter_map(|slug| {
            let word = slug.rsplit('-').next().expect("a slug");
            t.ships
                .values()
                .find(|s| {
                    s.name
                        .split_whitespace()
                        .last()
                        .is_some_and(|w| w.to_lowercase() == word)
                })
                .map(|s| s.name.clone())
        })
        .collect();
    // Every slug but Gwin's, Zuiho's and Mikazuki's (left out, and said so)
    // is a ship of the scenario.
    assert_eq!(not_modelled.len(), slugs.len() - 3, "{not_modelled:?}");
    for letter in LETTERS {
        let header = header_words(letter);
        for words in [
            // Rule 16.
            "A carrier caught with her aircraft armed and fuelled on deck",
            // Rule 17.
            "Submarines",
            // Rules 42 to 45.
            "Weather",
            "Daylight",
            "Reach",
            "their bombers are held to the raid on Midway",
            // Rule 86.
            "night surface threat of the Second Fleet's main body",
            // Rule 93.
            "Ships' experience is derived",
            "Worden",
            // Rule 97.
            "flying boats' night torpedo attack",
            // Rule 95.
            "no force's fuel is weighed by the staff",
            "the oilers are left out",
            // Rule 82.
            "hat Wake's being on the map does to each side's search",
            "SURFACE WEAPONS NOT MODELLED",
        ] {
            assert!(header.contains(words), "file {letter}: {words}");
        }
        // Each ship of the list's, by the name of the scenario, before the
        // sentence that says surface fire is modelled for the rest.
        let from = header
            .find("SURFACE WEAPONS NOT MODELLED")
            .expect("the list");
        let until = header[from..]
            .find("Surface fire is modelled for")
            .expect("the other list")
            + from;
        let said = &header[from..until];
        for name in &not_modelled {
            assert!(
                said.contains(name.as_str()),
                "file {letter}: {name} is named"
            );
        }
        // And no ship of the other list is named among them.
        let modelled_for = &header[until..];
        for name in [
            "Arashio",
            "Asashio",
            "Oshio",
            "Michishio",
            "Kongo",
            "Hiei",
            "Atago",
            "Chokai",
            "Haguro",
            "Myoko",
            "Yudachi",
            "Murasame",
            "Harusame",
            "Samidare",
            "Asagumo",
            "Minegumo",
            "Natsugumo",
        ] {
            assert!(modelled_for.contains(name), "{name}: modelled for");
            assert!(!not_modelled.iter().any(|n| n == name), "{name}");
        }
    }
}

fn start_truth() -> TruthState {
    start('d')
}

// ----------------------------------------------------------------- Properties

/// Nothing is invented: every ship and aircraft type of the file is of a
/// class or type the list derives, but the transports and their class and the
/// two stocks, each marked a placeholder.
#[test]
fn property_nothing_is_invented() {
    let derived: BTreeSet<u32> = rows("ship_classes_midway.csv")
        .iter()
        .map(|r| r[0].parse().expect("a class id"))
        .collect();
    let types: BTreeSet<u32> = rows("aircraft_types_midway.csv")
        .iter()
        .map(|r| r[0].parse().expect("a type id"))
        .collect();
    for letter in LETTERS {
        let w = load(letter).world;
        let mut real = 0;
        let mut placeholders = 0;
        for s in w.umpire().ships.values() {
            if is_transport(s) {
                placeholders += 1;
                assert_eq!(s.class.raw(), 19, "{}", s.name);
                continue;
            }
            real += 1;
            assert!(
                derived.contains(&s.class.raw()),
                "{} class {}",
                s.name,
                s.class.raw()
            );
            assert_eq!(s.class.raw(), list_class(&s.name), "{}", s.name);
        }
        assert_eq!((real, placeholders), (92 - 13, 13), "file {letter}");
        for g in w.umpire().air_groups.values() {
            assert!(types.contains(&g.aircraft_type.raw()), "{}", g.name);
        }
        // The placeholders are marked: the class's file, the header's
        // placeholders and the two stocks.
        let class_file = reference("ship_classes_placeholder_transport_8_8_knots.csv");
        assert!(class_file.contains("PLACEHOLDER DATA"));
        let header = header_words(letter);
        assert!(header.contains("PLACEHOLDER DATA"), "file {letter}");
        assert!(header.contains("Midway's stock and Wake's stock"));
        assert!(
            text(letter).lines().any(|l| l == "placeholder = true"),
            "file {letter} is marked a placeholder"
        );
    }
}

/// The list's check passes, and no figure of the scenario is typed by hand
/// where the list gives it: every ship's class and experience, as the list
/// gives them, is the file's.
#[test]
fn property_the_lists_check_passes_and_the_files_figures_are_the_lists() {
    // `oob-derive data/derivation/midway.toml --check`, asked of the library
    // the program is built on, as the program asks it.
    let list = repo().join("data/derivation/midway.toml");
    let said = oob_derive::said(
        &list.display().to_string(),
        oob_derive::run(&list, &repo().join("data/oob"), true),
    );
    assert!(said.passed, "{}{}", said.out, said.err);
    for letter in LETTERS {
        let t = start(letter);
        for s in t.ships.values().filter(|s| !is_transport(s)) {
            assert_eq!(s.class.raw(), list_class(&s.name), "{}", s.name);
            if let Some(e) = list_experience(&s.name) {
                assert_eq!(s.experience, e, "{}", s.name);
            }
        }
    }
}

/// The same game twice gives the same events.
#[test]
fn property_the_same_game_twice_gives_the_same_events() {
    let first = played('a');
    assert_eq!(first.hashes.len(), 5, "file a plays five days");
    let again = play('a');
    assert_eq!(first.hashes, again.hashes);
    assert_eq!(first.events.len(), again.events.len());
    assert!(first.events == again.events, "the events differ");
    for letter in ['b', 'c', 'd'] {
        assert_eq!(
            played(letter).hashes.len(),
            5,
            "file {letter} plays five days"
        );
    }
}

/// Each side sees its own: the Japanese side's first briefing names no
/// American carrier, and the Allied side's names no Japanese force.
#[test]
fn property_each_side_sees_its_own_and_names_no_enemy_force_before_it_is_sighted() {
    for letter in LETTERS {
        let w = load(letter).world;
        let truth = w.umpire();
        for side in Side::ALL {
            let view = w.side_view(side);
            assert!(view.ships().all(|s| truth.ships[&s.id].side == side));
            assert!(view.task_forces().all(|f| f.side == side));
            assert!(view.air_groups().all(|g| g.side == side));
            assert!(view.land_units().all(|u| u.side == side));
            assert_eq!(
                view.contacts().count(),
                0,
                "nothing is sighted at the start"
            );
            assert!(view.enemy_bases().all(|b| b.side != side));
            let mut s = session(letter, &[side]);
            let mut said = String::new();
            for tool in ["get_briefing", "list_forces", "list_contacts", "list_bases"] {
                said.push_str(
                    &s.call(side, tool, &json!({}))
                        .expect("a tool answers")
                        .to_string(),
                );
            }
            let own = if side == A { "Yorktown" } else { "Akagi" };
            assert!(said.contains(own), "{side:?}'s tools name their own {own}");
            let enemy_names: Vec<&str> = truth
                .ships
                .values()
                .filter(|x| x.side != side)
                .map(|x| x.name.as_str())
                .chain(
                    truth
                        .air_groups
                        .values()
                        .filter(|g| g.side != side)
                        .map(|g| g.name.as_str()),
                )
                .chain(
                    truth
                        .land_units
                        .values()
                        .filter(|u| u.side != side)
                        .map(|u| u.name.as_str()),
                )
                .chain(
                    truth
                        .task_forces
                        .values()
                        .filter(|f| f.side != side)
                        .map(|f| f.name.as_str()),
                )
                .collect();
            assert!(enemy_names.len() > 30);
            for name in enemy_names {
                assert!(
                    !said.contains(name),
                    "file {letter}: {side:?}'s tools name {name:?} before it is sighted"
                );
            }
        }
    }
}
