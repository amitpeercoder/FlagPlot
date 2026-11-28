//! Scenario 105: the Coral Sea, 6 to 10 May 1942, a scenario for play.
//!
//! Proves decision 0188. What follows is the architect's, written before
//! any of the scenario is built: the forces, where they start, the
//! arithmetic of the convoy, the orders a side the engine plays is given,
//! and the scoring. Each numbered statement is a test. If the scenario or
//! the game disagrees with a statement, the statement or the scenario is
//! wrong: a mismatch goes back to the architect, and nobody else changes a
//! number here.
//!
//! Rows are of `data/oob` unless said. "The list" is
//! `data/derivation/coral_sea.toml` and the files it makes.
//!
//! # The clock and the days
//!
//! The game begins at 1800 on 5 May 1942. Day 1 is 6 May and day 5 is 10
//! May. Times are zone -11, the clock of the analysis of 1947.
//!
//! 1. The scenario has `days = 5`, and its header says the game begins at
//!    1800 on 5 May 1942 and that day 1 is 6 May. (A scenario file has no
//!    key for a date.)
//!
//! # The map and the bases
//!
//! The present map, unchanged. Hexes are 40 nm.
//!
//! | Base | Number | Side | Hex | What it has |
//! | --- | --- | --- | --- | --- |
//! | Port Moresby | 1 | Allied | (-17,-4) | Its garrison; a stock of supply; an airfield with no aircraft in this version; no anti-aircraft figure (a gap); no port size (the map gives every hex none, and a scenario cannot set one) |
//! | Rabaul | 2 | Japanese | (-5,-13) | As the 39 days gives it: supply 10,000 t, fuel 50,000 t, 4 aircraft repaired a day |
//!
//! 2. Port Moresby is a coast hex, and Rabaul is.
//! 3. Port Moresby's stock is 450 t, a placeholder: 30 days for its 3,000
//!    men at the game's 0.005 t a man a day (3,000 x 0.005 = 15 t a day;
//!    15 x 30 = 450).
//!
//! # The Allied forces
//!
//! Both sources agree on them ship for ship (engagement_forces 464 to 490
//! and 491 to 516). Every ship is of her class in the list.
//!
//! **Task force 1, Task Force 17,** on the carrier mission, 16 ships:
//! Yorktown, Lexington; Minneapolis, New Orleans, Astoria, Chester,
//! Portland; Phelps, Dewey, Farragut, Aylwin, Monaghan; Morris, Anderson,
//! Hammann, Russell.
//!
//! **Task force 2, the support group,** on the surface combat mission, 5
//! ships: Australia, Chicago, Hobart, Perkins, Walke.
//!
//! Both start in hex (-3,5), the hex of 15 00 S, 160 00 E, where two
//! sources have the forces join on the morning of 5 May (engagement
//! positions 562, 566, 567), some ten hours before the game begins: a
//! placeholder start, marked one. Left out and said in the header: the fuelling
//! group (Neosho, Sims, Worden, Tippecanoe) and the search group (Tangier).
//!
//! 4. Task force 1 has those 16 ships and task force 2 those 5, each of
//!    the class the list gives her, and both start at (-3,5).
//! 5. The five ships of task force 2 have guns; no ship of task force 1
//!    has (the list models surface fire for the support group alone on
//!    the Allied side).
//!
//! ## Their aircraft
//!
//! Yorktown's are her own returns for 7 May, the nearest day there is, of
//! aircraft in the organisation (air_unit_facts 457, 454, 460, 451).
//! Lexington's are her own report's aircraft in operation on 7 May (606,
//! 609, 608, 607), which are floors: they leave out the air group
//! commander's two. They are read in place of the low row of ruling 18,
//! being her own record and of better standing.
//!
//! | Group | Number | Ship | Type | Ready |
//! | --- | --- | --- | --- | --- |
//! | VF-42 | 1 | Yorktown | F4F-3 | 17 |
//! | VS-5 | 2 | Yorktown | SBD-3 | 17 |
//! | VB-5 | 3 | Yorktown | SBD-3 | 18 |
//! | VT-5 | 4 | Yorktown | TBD-1 | 10 |
//! | VF-2 | 5 | Lexington | F4F-3 | 18 |
//! | VS-2 | 6 | Lexington | SBD-3 | 17 |
//! | VB-2 | 7 | Lexington | SBD-2 | 18 |
//! | VT-2 | 8 | Lexington | TBD-1 | 12 |
//!
//! Every ship is given the experience the list's file gives her; a ship
//! the file gives none takes the game's own value for a ship not
//! answered. Every ship starts with her fuel full, an assumption, as in
//! scenario 103. No task force has a home port.
//!
//! 6. Those eight groups, with those types and numbers. Yorktown has 17 +
//!    17 + 18 + 10 = 62 aircraft and Lexington 18 + 17 + 18 + 12 = 65:
//!    127 in all.
//!
//! ## Ashore
//!
//! No row gives Port Moresby's garrison in May. The nearest is about 3,000
//! men in January 1942. The scenario's garrison is one land unit of 3,000
//! men, a placeholder, named as one.
//!
//! 7. Land unit 1, "Placeholder strength: the garrison of Port Moresby",
//!    3,000 men ashore at (-17,-4), Allied.
//!
//! # The Japanese forces
//!
//! **Task force 3, the Striking Force,** on the carrier mission, 10
//! ships: Shokaku, Zuikaku; Myoko, Haguro; Akebono, Ushio, Ariake, Yugure,
//! Shigure, Shiratsuyu (both sources: 404 to 413, 440 to 449). It starts
//! in hex (4,-2). A row has it round the south-east tip of San Cristobal
//! at 1900 on 5 May (engagement_events 2480) and gives no position; (4,-2)
//! is the hex of that point on the map, and is marked a placeholder start.
//!
//! **Task force 4, the Main Force,** on the carrier mission, 6 ships:
//! Shoho; Aoba, Kinugasa, Kako, Furutaka; Sazanami (381 to 386, 451 to
//! 456). It starts in hex (-2,-8), the hex of Shortland, where a row has
//! the cruisers and Shoho fuel on 5 May (ship_facts 3496). The hour is not
//! given: a placeholder start.
//!
//! **Task force 5, the Port Moresby invasion force,** on the transport
//! mission, 16 ships: Yubari, as the list rates her; five "Placeholder
//! destroyers" for Oite, Asanagi, Mutsuki, Mochizuki and Yayoi, which both
//! sources give and no row can rate (387 to 392, 432 to 437); and ten
//! "Placeholder transports". Uzuki is in one source alone and is left out.
//!
//! 8. Task forces 3 and 4 have those ships of those classes and start at
//!    (4,-2) and (-2,-8). The five cruisers and destroyer of task force
//!    4 other than Shoho have guns; no ship of task force 3 has.
//! 9. Task force 5 has Yubari, five ships named "Placeholder destroyer 1"
//!    to "5" of the placeholder destroyer class, and ten named
//!    "Placeholder transport 1" to "10".
//!
//! ## Their aircraft
//!
//! The fleet carriers' are the one low row's for 7 May (479 to 484), read
//! by ruling 18. Shoho's are ruling 9's (611 to
//! 613). Rabaul's are the Japanese list of 4 May (493, 497, 498): the 4th
//! Air Group's 17 G4M1 as well as the Motoyama group's 24 G3M2, which
//! corrects decision 0188, amended 41, where the architect counted one
//! group; and 12 fighters of the Tainan Air Group at Rabaul. Its six at
//! Lae are left out with Lae. No row gives the Tainan group's type in
//! May: the row says "12 VF". It is read as the A6M2 from the group's
//! rows of August (air_unit_facts 85), a reading, said in the scenario.
//!
//! | Group | Number | Ship or base | Type | Ready |
//! | --- | --- | --- | --- | --- |
//! | Shokaku fighter unit | 9 | Shokaku | A6M2 | 18 |
//! | Shokaku bomber unit | 10 | Shokaku | D3A1 | 21 |
//! | Shokaku attack unit | 11 | Shokaku | B5N2 | 19 |
//! | Zuikaku fighter unit | 12 | Zuikaku | A6M2 | 20 |
//! | Zuikaku bomber unit | 13 | Zuikaku | D3A1 | 22 |
//! | Zuikaku attack unit | 14 | Zuikaku | B5N2 | 21 |
//! | Shoho fighters, A6M2 | 15 | Shoho | A6M2 | 6 |
//! | Shoho fighters, A5M4 | 16 | Shoho | A5M4 | 6 |
//! | Shoho attack unit | 17 | Shoho | B5N1 | 9 |
//! | 4th Air Group | 18 | Rabaul | G4M1 | 17 |
//! | Motoyama Air Group | 19 | Rabaul | G3M2 | 24 |
//! | Tainan Air Group | 20 | Rabaul | A6M2 | 12 |
//!
//! 10. Those twelve groups. Shokaku has 18 + 21 + 19 = 58, Zuikaku 20 +
//!     22 + 21 = 63, the two 121; Shoho 6 + 6 + 9 = 21; Rabaul 17 + 24 +
//!     12 = 53.
//!
//! # The convoy
//!
//! ## Its men
//!
//! No good source counts the men aboard. The units are named by the
//! analysis of 1947. The scenario's men are placeholders, named so:
//!
//! - land unit 2, "Placeholder strength: the South Seas Detachment",
//!   4,886 men, its strength at Guam in January 1942 (a medium row);
//! - land unit 3, "Placeholder strength: the 3rd Kure Special Naval
//!   Landing Force", 500 men (a low row).
//!
//! 4,886 + 500 = 5,386 men.
//!
//! ## Its transports
//!
//! The sources count five, six, nine, ten and eleven transports. A
//! placeholder transport berths 600 men, and a ship carries the men of
//! one unit only. The detachment's 4,886 need 4,886 / 600 = 8.14, so
//! nine ships; the landing force's 500 need a ship of their own. Ten
//! transports, which is also one source's count by name (engagement_forces
//! 422 to 431).
//!
//! The men aboard: transports 1 to 8 carry 600 of the detachment each (8
//! x 600 = 4,800); transport 9 carries its last 86 (4,886 - 4,800);
//! transport 10 carries the 500 of the landing force.
//!
//! 11. The three land units and their men; the detachment's men 600 in
//!     each of transports 1 to 8 and 86 in transport 9; the landing
//!     force's 500 in transport 10. No man is aboard Yubari or a
//!     destroyer. 5,386 men are aboard in all.
//!
//! ## Its supply
//!
//! The staff reckons 15 days for a lift's men. 5,386 x 0.005 = 26.93 t a
//! day; 15 days is 403.95 t. Each transport carries 41 t: 10 x 41 = 410
//! t, a placeholder, 15 days and a little.
//!
//! 12. Each transport has 41 t of supply aboard; 410 t in all.
//!
//! ## Its start and its speed
//!
//! It left Rabaul at 1600 on 4 May for Port Moresby at 6.5 kn
//! (engagement_events 2478; troop_lift_facts 759, 760). That hour is on
//! the Japanese clock, zone -9: it is 1800 on 4 May on the clock used
//! here. By 1800 on 5 May that is 24 hours: 24 x 6.5 = 156 nm. The
//! game's sea route from Rabaul to Port Moresby is 652 nm and rounds the
//! eastern tip of New Guinea; its fourth hex, (-8,-9), is 160 nm along
//! it. The convoy starts in (-8,-9), stopped, with no route, so that a
//! staff may take it as a lift as it stands.
//!
//! The placeholder transport of the other scenarios cruises at 10 kn.
//! This convoy's transports are of a placeholder class of their own,
//! "Placeholder transport, 6.5 knots": the same ship with a cruise speed
//! and a top speed of 6.5 kn, which a row calls the convoy's cruising
//! speed and another its most (troop_lift_facts 759).
//!
//! 13. Task force 5 starts at (-8,-9) with no route. Its transports
//!     cruise at 6.5 kn and can go no faster.
//! 14. The game measures 652 nm by sea from (-5,-13) to (-17,-4), and
//!     492 nm from (-8,-9) to (-17,-4).
//! 15. At 6.5 kn, 492 nm takes 492 / 6.5 = 75.69 hours, 75 hours 42
//!     minutes to the minute above. From 1800 on 5 May that is 2142 on 8
//!     May: in the night pulse of day 4. The game's measure of the
//!     passage for task force 5 gives that hour, to the minute it rounds
//!     to. (At 10 kn it would be 49.2 hours, 1912 on 7 May, a day sooner;
//!     the row's speed is why the class is its own.)
//!
//! When the convoy lands in a game is the staff's doing and is not worked
//! here: a staff may sail it later so as to arrive by night. It cannot
//! land sooner than statement 15's hour.
//!
//! # Distances that shape the game
//!
//! 16. From the Allied start (-3,5): 280 nm to the Striking Force's
//!     start (4,-2); 589 nm to (-12,-3), the nearest hex of the convoy's
//!     route; 803 nm to Port Moresby. Each by the game's measure. The
//!     convoy's route from (-8,-9) passes (-12,-4), (-12,-3), (-13,-2)
//!     and (-14,-2) in that order.
//! 17. Strike radii, from the list: SBD-3 200 nm, SBD-2 and TBD-1 180,
//!     D3A1 252, B5N2 175, B5N1 196, G4M1 611, G3M2 709.
//! 18. Rabaul to (-12,-3) is 40 x root(7 x 7 + 10 x 10 - 7 x 10) = 40 x
//!     root 79 = 355.5 nm: inside both land bombers' radius, and beyond
//!     the A6M2's 261, so the bombers go there without escort.
//!
//! # What a side the engine plays is given
//!
//! On day 1, as commands and orders of the scenario, with no risk set
//! (balanced), and leave to prepare and leave to launch given. Each is
//! what the side's mission implies.
//!
//! They were first written before any game. A reviewer then played one
//! trial game of them, and the architect saw it before writing these:
//! the Japanese side won it by 41.56 points on the landing. Two things
//! in the first version did nothing at all and are mended (the support
//! group's order, and nine transports that could not load). One more is
//! changed for a reason the mission gives and the trial showed: a station
//! from which every Allied bomber reaches the convoy's route. The
//! Japanese orders are as first written.
//!
//! The build then played one game of them, and the architect saw that too:
//! the Japanese side won it by 58.76. In it an order of his, to the six
//! Allied bomber groups to strike transports first, held those groups
//! from the staff, and they flew nothing for five days. A side the engine
//! plays cannot be told what to strike first: a commander's order to a
//! group takes it out of the staff's hands. The order is taken out, and
//! that a staff sent to stop a landing spends its strikes on the escort
//! is recorded as something the game cannot yet do (decision 0188). The
//! support group reached the convoy's route a day and a half late at its
//! cruising speed, and is now sent at 20 knots. After this the orders
//! are not changed again before the run of 1,000 games.
//!
//! **Japanese:**
//!
//! - intent 1, take the place (-17,-4), its forces task force 5 and land
//!   units 2 and 3. The Japanese side holds no estimate of the garrison
//!   when the game begins: no row gives what it believed, and with none
//!   the staff takes everything the ships carry and says it goes on no
//!   count. Once ashore the men count the garrison themselves, and the
//!   staff attacks only with twice what they count;
//! - intent 2, operate the carrier force task force 3 from (-4,1), the
//!   hex of the position a row gives it at 2400 on 6 May
//!   (engagement_positions 570);
//! - intent 3, operate the carrier force task force 4 from (-10,-2), the
//!   hex of Deboyne, where a row has Shoho on 6 May (ship_facts 3509);
//! - intent 4, deny the waters within 2 hexes of (-12,-3), where the
//!   convoy rounds the tip, its forces air groups 18, 19 and 20. Under it
//!   the staff forms a task to strike ships from Rabaul, whose bombers
//!   then fly at any ship contact in their reach, in those waters or not.
//!
//! **Allied:**
//!
//! - intent 1, hold Port Moresby (base 1), its forces land unit 1;
//! - intent 2, operate the carrier force task force 1 from (-12,1). No
//!   row gives a station that serves: where the carriers launched on 7
//!   May is some 300 nm from the route the game gives the convoy, which
//!   rounds the tip and does not use Jomard Passage. (-12,1) is the
//!   architect's choice from the mission, a placeholder: a deep sea hex
//!   from which three hexes of the convoy's route are inside the 180 nm
//!   of the SBD-2 and the TBD-1, the shortest radius aboard. (-13,-2) is
//!   40 x root(1 + 9 + 3) = 40 x root 13 = 144.2 nm; (-12,-3) is 40 x
//!   root(0 + 16 + 0) = 160 nm; (-14,-2) is 40 x root(4 + 9 + 6) = 40 x
//!   root 19 = 174.4 nm. Deboyne is 40 x root(4 + 9 - 6) = 40 x root 7 =
//!   105.8 nm;
//! - intent 3, stand at (-13,-2) with task force 2, on the convoy's
//!   route at the tip, as the support group was sent to bar the way. The
//!   force has no home port and is stood from where it is (decision
//!   0192);
//! - intent 4, deny the waters within 1 hex of (-13,-2), its forces task
//!   force 1, with the targets limit transport, carrier, battleship,
//!   cruiser, destroyer, auxiliary and the waters first (decision 0190).
//!   The seven hexes hold (-12,-3) and (-14,-2), where the convoy passes,
//!   and not the Main Force's station. The carriers keep their station by
//!   intent 2; the deny has their bombers strike what is in those waters
//!   before anything else, and the transports before the escort.
//!
//! These last two replace orders of the commander's own that the
//! architect first gave by hand (a route and a speed for task force 2,
//! and before that a list for each bomber group, which held the groups
//! from the staff). Decisions 0190 and 0192 let the intents say it.
//!
//! 19. File a's commands of day 1 are those, for each side, with both
//!     leaves, and no other, and it has no order of a commander's own.
//!     After day 1 each side's command state holds its intents with no
//!     risk set; no Allied air group or force is held; on the morning of
//!     day 2 task force 2 has a route to (-13,-2) or has arrived there,
//!     and each of the six Allied bomber groups has transports first on
//!     its list and the deny's seven hexes as its waters to strike first.
//! 20. From (-3,5), (-12,1) is 40 x root(9 x 9 + 4 x 4 + 9 x 4) = 40 x
//!     root 133 = 461.3 nm by air; the game's sea measure is stated by
//!     the test from the game and is not below that.
//!
//! # The files
//!
//! Four, as scenario 103's: a, both sides given their orders; b, the
//! Japanese side given its orders and the Allied none; c, the Allied
//! given and the Japanese none; d, neither. Made by a script from one
//! base, which is not played. With their names and commands taken away
//! the four are the same. The script is written so that files of the same
//! orders at another risk can be added to it later.
//!
//! 21. The four files load, differ only in name and commands, and the
//!     script says each is what it makes.
//!
//! # The objectives
//!
//! A scenario for play (decision 0187): `for_play = true`. Every figure
//! is one the game already has. The horizon is 30 days: the next battle,
//! Midway, was fought 30 days after the game begins, and a carrier out
//! for those days missed it, as two of these did. Each side's mission is
//! the sentence under "What each side is to do" of decision 0188 (rules
//! 11 and 12), and the history note says that Shoho was sunk and
//! Lexington lost, that Yorktown and Shokaku were damaged, and that the
//! convoy turned back on 7 May and landed no one.
//!
//! | Kind | Allied | Japanese |
//! | --- | --- | --- |
//! | Ships sunk: carrier 10, battleship 8, cruiser 4, destroyer 1, transport 2, auxiliary 1 | yes | yes |
//! | Ships out of action, over the horizon: carrier 5, battleship 4, cruiser 2, destroyer 0.5, transport 1, auxiliary 0.5 | yes | yes |
//! | Aircraft destroyed, 0.05 each | yes | yes |
//! | Enemy men lost, 1 for each 100 | yes | yes |
//! | Men landed at Port Moresby (base 1), 1 for each 100 | | yes |
//!
//! ## The levels, set here for this scenario's size
//!
//! The game turns on the convoy, so the levels are measured in it. One
//! transport's 600 men landed are 6 points. A third of the convoy's men
//! landed is 5,386 / 3 / 100 = 17.95 points; two thirds, 35.91.
//!
//! - minor 6: the worth of one full transport's men;
//! - major 18: a third of the convoy;
//! - decisive 36: two thirds of it.
//!
//! They are set before any game is played and are not moved to enter the
//! band (decision 0187, 15).
//!
//! 22. The objectives are that table, the levels 6, 18 and 36, the
//!     horizon 30, and the file says it is for play.
//! 23. Worked on made-up counts, through the game's own scoring: all
//!     5,386 men landed and nothing else is 53.86 points to the Japanese
//!     side, a margin of 53.86, a decisive victory. Three full
//!     transports sunk with their 1,800 men and nothing else is 3 x 2 +
//!     18 = 24 points to the Allied side, a major victory. One carrier sunk each side
//!     and nothing else is 10 to 10, a stalemate.
//!
//! # Rules switched on
//!
//! As scenario 103's base has them: fatigue, wear, supply and the orders
//! inside a day. Command levels are off.
//!
//! 24. The header's switches are those of scenario 103's base.
//!
//! # Properties
//!
//! - Each file loads and plays five days, and the same seed gives the
//!   same daily hashes twice.
//! - No real ship has a figure that is not the list's: every ship not
//!   named "Placeholder" is of a class the list derives.
//! - In files c and d, where the Japanese side is given no orders, the
//!   convoy does not move and no man lands.
//! - Nothing in a side's view names an enemy ship, group or unit by its
//!   truth before it is sighted.
//!
//! # One game before the thousand
//!
//! Not a test. When the files are made the build plays file a once on
//! its own seed and reports to the architect, before any run of 1,000 is
//! read: whether the convoy sailed and when; when and where it landed and
//! how many men; what the men did ashore and whether Port Moresby fell;
//! where each carrier force went and what it struck; what Rabaul's
//! aircraft and the support group did; and the result. If the
//! orders left out something the mission asks for, the architect mends
//! them then. What the reviewer's trial showed is expected again and is
//! reported, not mended, in this build: the men ashore count the garrison
//! and wait for twice its number, so Port Moresby is not attacked; the
//! Striking Force closes only on a contact it takes for a carrier and may
//! stay out of the fight for days; Shoho's nine torpedo planes are her
//! force's search and do not strike.
//!
//! The tests are written by the build under these statements.

// The tests are written by the build, to the statements above. Each names
// the statement it proves. Everything here runs on the live figures: no
// number asserted is one the step 1 freeze holds.

use crate::common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::common::values::{base, fx, group, hex, n, ship, unit};
use serde_json::{json, Value};
use sim_agent::SeatKind;
use sim_core::choices::Moment;
use sim_core::command::{CommandState, IntentKind, Maker};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{IntentId, ShipClassId, TaskForceId};
use sim_core::map::HexKind;
use sim_core::objectives::{score, Level, Objective, Outcome, Tally};
use sim_core::orders::{Issuer, Mission};
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

fn path(letter: char) -> PathBuf {
    common::data_dir().join(format!("scenarios/coral-sea/01{letter}_coral_sea.toml"))
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

fn tf(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
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

/// The class id the list gives each real ship, by the ship's name in the
/// list (`ship_classes_coral_sea.ships.csv`).
fn list_class(slug: &str) -> u32 {
    rows("ship_classes_coral_sea.ships.csv")
        .iter()
        .find(|r| r[0] == slug)
        .unwrap_or_else(|| panic!("{slug} is in the list"))[1]
        .parse()
        .expect("a class id")
}

/// The experience the list's file gives a ship, if it gives one.
fn list_experience(slug: &str) -> Option<u8> {
    rows("ship_experience_coral_sea.ships.csv")
        .iter()
        .find(|r| r[0] == slug)
        .map(|r| r[1].parse().expect("an experience"))
}

/// The real ships: (number, the scenario's name, the list's name, task force).
const REAL: [(u32, &str, &str, u32); 38] = [
    (1, "USS Yorktown", "cv-5-yorktown", 1),
    (2, "USS Lexington", "cv-2-lexington", 1),
    (3, "USS Minneapolis", "ca-36-minneapolis", 1),
    (4, "USS New Orleans", "ca-32-new-orleans", 1),
    (5, "USS Astoria", "ca-34-astoria", 1),
    (6, "USS Chester", "ca-27-chester", 1),
    (7, "USS Portland", "ca-33-portland", 1),
    (8, "USS Phelps", "dd-360-phelps", 1),
    (9, "USS Dewey", "dd-349-dewey", 1),
    (10, "USS Farragut", "dd-348-farragut", 1),
    (11, "USS Aylwin", "dd-355-aylwin", 1),
    (12, "USS Monaghan", "dd-354-monaghan", 1),
    (13, "USS Morris", "dd-417-morris", 1),
    (14, "USS Anderson", "dd-411-anderson", 1),
    (15, "USS Hammann", "dd-412-hammann", 1),
    (16, "USS Russell", "dd-414-russell", 1),
    (17, "HMAS Australia", "hmas-australia", 2),
    (18, "USS Chicago", "ca-29-chicago", 2),
    (19, "HMAS Hobart", "hmas-hobart", 2),
    (20, "USS Perkins", "dd-377-perkins", 2),
    (21, "USS Walke", "dd-416-walke", 2),
    (22, "Shokaku", "shokaku", 3),
    (23, "Zuikaku", "zuikaku", 3),
    (24, "Myoko", "myoko", 3),
    (25, "Haguro", "haguro", 3),
    (26, "Akebono", "akebono", 3),
    (27, "Ushio", "ushio", 3),
    (28, "Ariake", "ariake", 3),
    (29, "Yugure", "yugure", 3),
    (30, "Shigure", "shigure", 3),
    (31, "Shiratsuyu", "shiratsuyu", 3),
    (32, "Shoho", "shoho", 4),
    (33, "Aoba", "aoba", 4),
    (34, "Kinugasa", "kinugasa", 4),
    (35, "Kako", "kako", 4),
    (36, "Furutaka", "furutaka", 4),
    (37, "Sazanami", "sazanami", 4),
    (38, "Yubari", "yubari", 5),
];

/// The aircraft type of a name in the Coral Sea list's file: the id of the
/// row whose name holds `fragment`.
fn type_id(w: &World, fragment: &str) -> u32 {
    let found: Vec<u32> = w
        .ref_data()
        .aircraft_types
        .values()
        .filter(|t| t.name.contains(fragment))
        .map(|t| t.id.raw())
        .collect();
    assert_eq!(found.len(), 1, "one type is named {fragment}: {found:?}");
    found[0]
}

/// A game played to its end with the doctrine's staffs, as `run` and `batch`
/// play it.
struct Played {
    hashes: Vec<u64>,
    orders: Vec<sim_core::orders::Order>,
    after_day_1: BTreeMap<Side, CommandState>,
    /// At the start of the morning pulse of days 1 and 2: each air group's
    /// target list, and task force 2's position and route.
    mornings: BTreeMap<u32, Morning>,
    /// Task force 5's hex and the men ashore of land units 2 and 3 at the
    /// end of each day.
    convoy: Vec<(Hex, u32)>,
    end: World,
}

const TARGETS: [ShipKind; 6] = [
    ShipKind::Transport,
    ShipKind::Carrier,
    ShipKind::Battleship,
    ShipKind::Cruiser,
    ShipKind::Destroyer,
    ShipKind::Auxiliary,
];

struct Morning {
    lists: BTreeMap<u32, Vec<ShipKind>>,
    waters: BTreeMap<u32, BTreeSet<Hex>>,
    tf2_at: Hex,
    tf2_route: Vec<Hex>,
}

fn play(letter: char) -> Played {
    let mut scenario = load(letter);
    let held = staffs();
    let mut hashes = Vec::new();
    let mut after_day_1 = BTreeMap::new();
    let mut mornings = BTreeMap::new();
    let mut convoy = Vec::new();
    loop {
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                if let Moment::PulseBegins {
                    day,
                    pulse: Pulse::Morning,
                } = moment
                {
                    let t = world.umpire();
                    mornings.insert(
                        day,
                        Morning {
                            lists: t
                                .air_groups
                                .iter()
                                .map(|(id, g)| (id.raw(), g.target_priority.clone()))
                                .collect(),
                            waters: t
                                .air_groups
                                .iter()
                                .map(|(id, g)| {
                                    (id.raw(), g.strike_first_waters.iter().copied().collect())
                                })
                                .collect(),
                            tf2_at: t.task_forces[&tf(2)].position(),
                            tf2_route: t.task_forces[&tf(2)].route.waypoints.clone(),
                        },
                    );
                }
            })
            .expect("a day resolves");
        let Some(summary) = summary else { break };
        hashes.push(summary.state_hash);
        let t = scenario.world.umpire();
        convoy.push((
            t.task_forces[&tf(5)].position(),
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
        orders: scenario.world.orders().cloned().collect(),
        after_day_1,
        mornings,
        convoy,
        end: scenario.world,
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

fn air_distance(s: &mut sim_agent::Session, from: (i32, i32), to: (i32, i32)) -> u32 {
    let a = measure(
        s,
        A,
        json!({"question": "air_distance", "from": place(from.0, from.1), "to": place(to.0, to.1)}),
    );
    a["distance_nm"].as_u64().expect("miles") as u32
}

/// The sea route and its charged length between two hexes, by the game.
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

/// The air reach of a side's group to each place: (radius, miles, within).
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

/// Statement 1: `days = 5`, and the header says the game begins at 1800 on 5 May 1942
/// and that day 1 is 6 May.
#[test]
fn s01_five_days_from_1800_on_5_may_1942_with_day_1_the_6th() {
    for letter in LETTERS {
        assert_eq!(load(letter).days, 5, "file {letter}");
        let t = text(letter);
        assert!(
            t.lines().any(|l| l == "days = 5"),
            "file {letter} says days = 5"
        );
        let header = header_words(letter);
        assert!(
            header.contains("The game begins at 1800 on 5 May 1942"),
            "file {letter}: {header}"
        );
        assert!(header.contains("day 1 is 6 May"), "file {letter}: {header}");
    }
}

// ----------------------------------------------------- The map and the bases

/// Statement 2: Port Moresby is a coast hex, and Rabaul is. (And the table's other
/// cells: hexes, sides, the airfield, no port size, no anti-aircraft figure.)
#[test]
fn s02_port_moresby_and_rabaul_are_coast_hexes_where_the_table_puts_them() {
    for letter in LETTERS {
        let w = load(letter).world;
        let t = w.umpire();
        assert_eq!(t.bases.len(), 2, "file {letter}: two bases only");
        for (raw, at, side, name) in [
            (1, (-17, -4), A, "Port Moresby"),
            (2, (-5, -13), J, "Rabaul"),
        ] {
            let b = &t.bases[&base(raw)];
            assert_eq!(
                (b.name.as_str(), b.hex, b.side),
                (name, hex(at.0, at.1), side)
            );
            let data = w.map().get(b.hex).expect("the hex is on the map");
            assert_eq!(data.kind, HexKind::Coast, "{name} is a coast hex");
            assert_eq!(data.port_size, 0, "{name}: the map gives no port size");
        }
        let moresby = &t.bases[&base(1)];
        assert!(moresby.airfield, "Port Moresby has an airfield");
        assert_eq!(moresby.anti_aircraft, 0, "no anti-aircraft figure: a gap");
        assert!(
            !t.air_groups
                .values()
                .any(|g| g.location == AirGroupLocation::Base(base(1))),
            "no aircraft at Port Moresby in this version"
        );
    }
}

/// Statement 3: Port Moresby's stock is 450 t: 3,000 x 0.005 = 15 t a day; 15 x 30 =
/// Statement 450: Rabaul is as the 39 days gives it: supply 10,000 t, fuel 50,000 t, 4
/// aircraft repaired a day.
#[test]
fn s03_port_moresbys_stock_is_450_t_and_rabaul_is_the_39_days() {
    let w = load('a').world;
    let daily = w
        .ref_data()
        .ground
        .expect("ground figures")
        .daily_supply_t_per_man;
    assert_eq!(daily, fx("0.005"));
    assert_eq!(n(3000) * daily, n(15));
    assert_eq!(n(3000) * daily * n(30), n(450));
    let t = w.umpire();
    assert_eq!(t.bases[&base(1)].supply_t, n(450));
    let rabaul = &t.bases[&base(2)];
    assert_eq!(
        (
            rabaul.supply_t,
            rabaul.fuel_t,
            rabaul.aircraft_repairs_per_day
        ),
        (n(10_000), n(50_000), 4)
    );
    let slice = common::load_live(&common::data_dir().join("scenarios/07_full_slice.toml"))
        .expect("the 39 days load");
    let theirs = &slice.world.umpire().bases[&base(2)];
    assert_eq!(
        (
            rabaul.hex,
            rabaul.supply_t,
            rabaul.fuel_t,
            rabaul.aircraft_repairs_per_day
        ),
        (
            theirs.hex,
            theirs.supply_t,
            theirs.fuel_t,
            theirs.aircraft_repairs_per_day
        ),
        "the 39 days' Rabaul"
    );
}

// ------------------------------------------------------ The Allied forces

/// Statement 4: Task force 1 has those 16 ships and task force 2 those 5, each of the
/// class the list gives her, and both start at (-3,5).
#[test]
fn s04_task_forces_1_and_2_have_their_21_ships_of_the_lists_classes_at_minus_3_5() {
    for letter in LETTERS {
        let t = start(letter);
        for (raw, mission) in [(1, Mission::Carrier), (2, Mission::SurfaceCombat)] {
            let f = &t.task_forces[&tf(raw)];
            assert_eq!((f.side, f.mission), (A, mission), "task force {raw}");
            assert_eq!(
                f.position(),
                hex(-3, 5),
                "task force {raw} starts at (-3,5)"
            );
        }
        let ships_of = |raw: u32| -> Vec<u32> {
            t.task_forces[&tf(raw)]
                .ships
                .iter()
                .map(|s| s.raw())
                .collect()
        };
        assert_eq!(ships_of(1), (1..=16).collect::<Vec<u32>>());
        assert_eq!(ships_of(2), (17..=21).collect::<Vec<u32>>());
        for (raw, name, slug, force) in REAL.iter().filter(|r| r.3 <= 2) {
            let s = &t.ships[&ship(*raw)];
            assert_eq!((s.name.as_str(), s.side), (*name, A), "ship {raw}");
            assert_eq!(s.class.raw(), list_class(slug), "{name}: the list's class");
            assert_eq!(s.location, ShipLocation::TaskForce(tf(*force)), "{name}");
        }
    }
}

/// Statement 5: The five ships of task force 2 have guns; no ship of task force 1
/// has.
#[test]
fn s05_the_five_ships_of_task_force_2_have_guns_and_none_of_task_force_1() {
    let w = load('a').world;
    let t = w.umpire();
    let guns = |raw: u32| w.ref_data().ship_classes[&t.ships[&ship(raw)].class].main_guns;
    for raw in 17..=21 {
        assert!(guns(raw) > 0, "{} has guns", t.ships[&ship(raw)].name);
    }
    for raw in 1..=16 {
        assert_eq!(guns(raw), 0, "{} has none", t.ships[&ship(raw)].name);
    }
}

/// Statement 6: Those eight groups, with those types and numbers. Yorktown has 17 + 17
/// + 18 + 10 = 62 aircraft and Lexington 18 + 17 + 18 + 12 = 65: 127.
#[test]
fn s06_the_eight_allied_groups_hold_62_and_65_aircraft_127_in_all() {
    let w = load('a').world;
    let t = w.umpire();
    let table = [
        (1, "VF-42", 1, "F4F-3", 17),
        (2, "VS-5", 1, "SBD-3", 17),
        (3, "VB-5", 1, "SBD-3", 18),
        (4, "VT-5", 1, "TBD-1", 10),
        (5, "VF-2", 2, "F4F-3", 18),
        (6, "VS-2", 2, "SBD-3", 17),
        (7, "VB-2", 2, "SBD-2", 18),
        (8, "VT-2", 2, "TBD-1", 12),
    ];
    let mut aboard: BTreeMap<u32, u32> = BTreeMap::new();
    for (raw, name, carrier, kind, ready) in table {
        let g = &t.air_groups[&group(raw)];
        assert_eq!(g.name, name);
        assert_eq!(g.side, A, "{name}");
        assert_eq!(g.location, AirGroupLocation::Ship(ship(carrier)), "{name}");
        assert_eq!(
            g.aircraft_type.raw(),
            type_id(&w, kind),
            "{name} flies the {kind}"
        );
        assert_eq!(g.ready, ready, "{name}");
        assert_eq!(g.damaged, 0, "{name}");
        *aboard.entry(carrier).or_default() += g.ready;
    }
    assert_eq!(
        t.air_groups.values().filter(|g| g.side == A).count(),
        8,
        "no other Allied group"
    );
    assert_eq!(aboard[&1], 17 + 17 + 18 + 10);
    assert_eq!(aboard[&1], 62);
    assert_eq!(aboard[&2], 18 + 17 + 18 + 12);
    assert_eq!(aboard[&2], 65);
    assert_eq!(aboard[&1] + aboard[&2], 127);
}

/// Every ship is given the experience the list's file gives her; a ship the
/// file gives none takes the game's own value. Every ship starts with her
/// fuel full. No task force has a home port.
#[test]
fn experience_from_the_list_fuel_full_and_no_home_port() {
    for letter in LETTERS {
        let t = start(letter);
        let w = load(letter).world;
        for (raw, name, slug, _) in REAL {
            let s = &t.ships[&ship(raw)];
            let want = list_experience(slug).unwrap_or(Ship::DEFAULT_EXPERIENCE);
            assert_eq!(s.experience, want, "{name}");
        }
        for s in t.ships.values() {
            let class = &w.ref_data().ship_classes[&s.class];
            assert_eq!(s.fuel_t, class.fuel_capacity_t, "{} starts full", s.name);
        }
        for f in t.task_forces.values() {
            assert_eq!(f.home_port, None, "{} has no home port", f.name);
        }
    }
}

// ---------------------------------------------------- Ashore (7)

/// Statement 7: Land unit 1, "Placeholder strength: the garrison of Port Moresby",
/// 3,000 men ashore at (-17,-4), Allied.
#[test]
fn s07_land_unit_1_is_the_garrison_of_3000_men_at_port_moresby() {
    for letter in LETTERS {
        let t = start(letter);
        let u = &t.land_units[&unit(1)];
        assert_eq!(u.name, "Placeholder strength: the garrison of Port Moresby");
        assert_eq!(
            (u.side, u.men_ashore, u.hex),
            (A, 3000, Some(hex(-17, -4))),
            "file {letter}"
        );
    }
}

// ------------------------------------------------ The Japanese forces

/// Statement 8: Task forces 3 and 4 have those ships of those classes and start at
/// (4,-2) and (-2,-8). The five cruisers and destroyer of task force 4
/// other than Shoho have guns; no ship of task force 3 has.
#[test]
fn s08_task_forces_3_and_4_have_their_ships_start_where_stated_and_task_force_3_has_no_guns() {
    for letter in LETTERS {
        let w = load(letter).world;
        let t = w.umpire();
        for (raw, at, count) in [(3, (4, -2), 10), (4, (-2, -8), 6)] {
            let f = &t.task_forces[&tf(raw)];
            assert_eq!(
                (f.side, f.mission),
                (J, Mission::Carrier),
                "task force {raw}"
            );
            assert_eq!(f.position(), hex(at.0, at.1), "task force {raw}");
            assert_eq!(f.ships.len(), count, "task force {raw}");
        }
        let ships_of = |raw: u32| -> Vec<u32> {
            t.task_forces[&tf(raw)]
                .ships
                .iter()
                .map(|s| s.raw())
                .collect()
        };
        assert_eq!(ships_of(3), (22..=31).collect::<Vec<u32>>());
        assert_eq!(ships_of(4), (32..=37).collect::<Vec<u32>>());
        for (raw, name, slug, force) in REAL.iter().filter(|r| r.3 == 3 || r.3 == 4) {
            let s = &t.ships[&ship(*raw)];
            assert_eq!((s.name.as_str(), s.side), (*name, J), "ship {raw}");
            assert_eq!(s.class.raw(), list_class(slug), "{name}: the list's class");
            assert_eq!(s.location, ShipLocation::TaskForce(tf(*force)), "{name}");
        }
        let guns = |raw: u32| w.ref_data().ship_classes[&t.ships[&ship(raw)].class].main_guns;
        for raw in 22..=31 {
            assert_eq!(guns(raw), 0, "{} has none", t.ships[&ship(raw)].name);
        }
        assert_eq!(guns(32), 0, "Shoho has none");
        let armed: Vec<&str> = (33..=37)
            .filter(|raw| guns(*raw) > 0)
            .map(|raw| t.ships[&ship(raw)].name.as_str())
            .collect();
        assert_eq!(
            armed,
            vec!["Aoba", "Kinugasa", "Kako", "Furutaka", "Sazanami"],
            "the four cruisers and the destroyer have guns"
        );
    }
}

/// Statement 9: Task force 5 has Yubari, five ships named "Placeholder destroyer 1" to
/// "5" of the placeholder destroyer class, and ten named "Placeholder
/// transport 1" to "10".
#[test]
fn s09_task_force_5_has_yubari_five_placeholder_destroyers_and_ten_placeholder_transports() {
    for letter in LETTERS {
        let w = load(letter).world;
        let t = w.umpire();
        let f = &t.task_forces[&tf(5)];
        assert_eq!((f.side, f.mission), (J, Mission::Transport));
        assert_eq!(
            f.ships.iter().map(|s| s.raw()).collect::<Vec<u32>>(),
            (38..=53).collect::<Vec<u32>>()
        );
        let yubari = &t.ships[&ship(38)];
        assert_eq!(
            (yubari.name.as_str(), yubari.class.raw()),
            ("Yubari", list_class("yubari"))
        );
        for i in 1..=5u32 {
            let s = &t.ships[&ship(38 + i)];
            assert_eq!(s.name, format!("Placeholder destroyer {i}"));
            assert_eq!(s.class.raw(), 2);
            assert_eq!(
                w.ref_data().ship_classes[&s.class].name,
                "Placeholder destroyer"
            );
            assert_eq!(s.side, J);
        }
        for i in 1..=10u32 {
            let s = &t.ships[&ship(43 + i)];
            assert_eq!(s.name, format!("Placeholder transport {i}"));
            assert_eq!(s.class.raw(), 14);
            assert_eq!(s.side, J);
        }
        assert_eq!(t.ships.len(), 53, "no other ship");
    }
}

/// Statement 10: Those twelve groups. Shokaku has 18 + 21 + 19 = 58, Zuikaku 20 + 22 +
/// 21 = 63, the two 121; Shoho 6 + 6 + 9 = 21; Rabaul 17 + 24 + 12 = 53.
#[test]
fn s10_the_twelve_japanese_groups_hold_58_63_21_and_53_aircraft() {
    let w = load('a').world;
    let t = w.umpire();
    let ship_at = |raw: u32| AirGroupLocation::Ship(ship(raw));
    let rabaul = AirGroupLocation::Base(base(2));
    let table = [
        (9, "Shokaku fighter unit", ship_at(22), "A6M2", 18),
        (10, "Shokaku bomber unit", ship_at(22), "D3A1", 21),
        (11, "Shokaku attack unit", ship_at(22), "B5N2", 19),
        (12, "Zuikaku fighter unit", ship_at(23), "A6M2", 20),
        (13, "Zuikaku bomber unit", ship_at(23), "D3A1", 22),
        (14, "Zuikaku attack unit", ship_at(23), "B5N2", 21),
        (15, "Shoho fighters, A6M2", ship_at(32), "A6M2", 6),
        (16, "Shoho fighters, A5M4", ship_at(32), "A5M4", 6),
        (17, "Shoho attack unit", ship_at(32), "B5N1", 9),
        (18, "4th Air Group", rabaul, "G4M1", 17),
        (19, "Motoyama Air Group", rabaul, "G3M2", 24),
        (20, "Tainan Air Group", rabaul, "A6M2", 12),
    ];
    let mut totals: BTreeMap<String, u32> = BTreeMap::new();
    for (raw, name, location, kind, ready) in table {
        let g = &t.air_groups[&group(raw)];
        assert_eq!(g.name, name);
        assert_eq!(
            (g.side, g.location, g.ready, g.damaged),
            (J, location, ready, 0),
            "{name}"
        );
        assert_eq!(
            g.aircraft_type.raw(),
            type_id(&w, kind),
            "{name} flies the {kind}"
        );
        let place = match location {
            AirGroupLocation::Ship(s) => t.ships[&s].name.clone(),
            _ => "Rabaul".to_string(),
        };
        *totals.entry(place).or_default() += ready;
    }
    assert_eq!(t.air_groups.values().filter(|g| g.side == J).count(), 12);
    assert_eq!(totals["Shokaku"], 18 + 21 + 19);
    assert_eq!(totals["Shokaku"], 58);
    assert_eq!(totals["Zuikaku"], 20 + 22 + 21);
    assert_eq!(totals["Zuikaku"], 63);
    assert_eq!(totals["Shokaku"] + totals["Zuikaku"], 121);
    assert_eq!(totals["Shoho"], 6 + 6 + 9);
    assert_eq!(totals["Shoho"], 21);
    assert_eq!(totals["Rabaul"], 17 + 24 + 12);
    assert_eq!(totals["Rabaul"], 53);
}

// --------------------------------------------------------- The convoy

/// Statement 11: The three land units and their men; the detachment's men 600 in each
/// of transports 1 to 8 and 86 in transport 9; the landing force's 500 in
/// transport 10. No man is aboard Yubari or a destroyer. 5,386 men are
/// aboard in all.
#[test]
fn s11_the_three_land_units_and_the_5386_men_aboard_the_transports() {
    for letter in LETTERS {
        let t = start(letter);
        assert_eq!(t.land_units.len(), 3, "three land units");
        assert_eq!(
            t.land_units[&unit(2)].name,
            "Placeholder strength: the South Seas Detachment"
        );
        assert_eq!(
            t.land_units[&unit(3)].name,
            "Placeholder strength: the 3rd Kure Special Naval Landing Force"
        );
        for raw in [2, 3] {
            let u = &t.land_units[&unit(raw)];
            assert_eq!(
                (u.side, u.men_ashore, u.hex),
                (J, 0, None),
                "unit {raw} is aboard"
            );
        }
        // Transports 44 to 53 are transports 1 to 10.
        let aboard = |raw: u32| t.ships[&ship(raw)].troops.map(|tr| (tr.unit.raw(), tr.men));
        for i in 1..=8u32 {
            assert_eq!(aboard(43 + i), Some((2, 600)), "transport {i}");
        }
        assert_eq!(aboard(52), Some((2, 86)), "transport 9");
        assert_eq!(aboard(53), Some((3, 500)), "transport 10");
        for raw in 38..=43 {
            assert_eq!(
                aboard(raw),
                None,
                "{} carries no man",
                t.ships[&ship(raw)].name
            );
        }
        assert_eq!(
            t.ships
                .values()
                .filter_map(|s| s.troops)
                .map(|tr| tr.men)
                .sum::<u32>(),
            5386
        );
        assert_eq!(8 * 600 + 86, 4886);
        assert_eq!(4886 + 500, 5386);
        let per_unit = |u: u32| -> u32 {
            t.ships
                .values()
                .filter_map(|s| s.troops)
                .filter(|tr| tr.unit.raw() == u)
                .map(|tr| tr.men)
                .sum()
        };
        assert_eq!((per_unit(2), per_unit(3)), (4886, 500));
    }
}

/// Statement 12: Each transport has 41 t of supply aboard; 410 t in all.
#[test]
fn s12_each_transport_has_41_t_of_supply_aboard_410_t_in_all() {
    let w = load('a').world;
    let t = w.umpire();
    for raw in 44..=53 {
        assert_eq!(
            t.ships[&ship(raw)].supply_t,
            n(41),
            "transport {}",
            raw - 43
        );
    }
    for raw in 1..=43 {
        assert_eq!(
            t.ships[&ship(raw)].supply_t,
            Fixed::ZERO,
            "ship {raw} carries none"
        );
    }
    let aboard = (44..=53)
        .map(|r| t.ships[&ship(r)].supply_t)
        .fold(Fixed::ZERO, |a, b| a + b);
    assert_eq!(aboard, n(410));
    // The staff reckons 15 days for a lift's men: 5,386 x 0.005 x 15.
    let daily = w
        .ref_data()
        .ground
        .expect("ground figures")
        .daily_supply_t_per_man;
    assert_eq!(n(5386) * daily, fx("26.93"));
    assert_eq!(n(5386) * daily * n(15), fx("403.95"));
    assert!(n(410) > n(5386) * daily * n(15));
}

/// Statement 13: Task force 5 starts at (-8,-9) with no route. Its transports cruise
/// at 6.5 kn and can go no faster.
#[test]
fn s13_task_force_5_starts_at_minus_8_minus_9_with_no_route_and_6_5_knot_transports() {
    for letter in LETTERS {
        let w = load(letter).world;
        let t = w.umpire();
        let f = &t.task_forces[&tf(5)];
        assert_eq!(f.position(), hex(-8, -9), "file {letter}");
        assert!(f.route.waypoints.is_empty(), "file {letter}: no route");
        assert_eq!(f.toward_hex, None);
        let class = &w.ref_data().ship_classes[&ShipClassId::new(14)];
        assert_eq!(class.name, "Placeholder transport, 6.5 knots");
        assert_eq!(
            (class.cruise_speed_kn, class.max_speed_kn),
            (fx("6.5"), fx("6.5"))
        );
        assert_eq!(class.troop_capacity_men, 600);
        for raw in 44..=53 {
            assert_eq!(t.ships[&ship(raw)].class.raw(), 14);
        }
    }
}

/// Statement 14: The game measures 652 nm by sea from (-5,-13) to (-17,-4), and 492
/// nm from (-8,-9) to (-17,-4).
#[test]
fn s14_the_games_sea_measure_is_652_and_492_nm() {
    let mut s = session('d', &[J]);
    let (whole, _) = sea_route(&mut s, J, (-5, -13), (-17, -4));
    assert_eq!(whole, 652);
    let (rest, route) = sea_route(&mut s, J, (-8, -9), (-17, -4));
    assert_eq!(rest, 492);
    // Its fourth hex is 160 nm along the route from Rabaul: the convoy's
    // start is the fourth hex of the 652 nm route.
    let (_, from_rabaul) = sea_route(&mut s, J, (-5, -13), (-17, -4));
    assert_eq!(from_rabaul.get(3), Some(&(-8, -9)));
    assert_eq!(route.last(), Some(&(-17, -4)));
}

/// Statement 15: At 6.5 kn, 492 nm takes 492 / 6.5 = 75.69 hours, 75 hours 42 minutes
/// to the minute. From 1800 on 5 May that is 2142 on 8 May: the night pulse
/// of day 4. The game's measure of the passage for task force 5 gives that
/// hour, to the minute it rounds to.
#[test]
fn s15_the_passage_of_task_force_5_takes_75_hours_42_minutes_and_ends_at_2142_on_8_may() {
    let mut s = session('d', &[J]);
    let convoy = handle(&mut s, J, "task_force", "Port Moresby invasion force");
    let a = measure(
        &mut s,
        J,
        json!({"question": "passage", "task_force": convoy, "to": place(-17, -4)}),
    );
    assert_eq!(a["speed"]["knots"], json!("6.5"), "{a}");
    assert_eq!(a["route_nm"], json!("492"), "{a}");
    assert_eq!(a["outcome"], json!("arrives"), "{a}");
    let hours: f64 = a["steaming_hours"]
        .as_str()
        .expect("hours")
        .parse()
        .expect("a decimal");
    // 492 / 6.5 = 75.69 hours.
    assert!((hours - 492.0 / 6.5).abs() < 0.0001, "{hours}");
    assert_eq!((hours * 60.0).round() as i64, 75 * 60 + 42, "{a}");
    // 2142 on 8 May is in the night pulse of day 4, whose night begins at
    // 1800 on 8 May: the game's own moment of arrival.
    assert_eq!(a["ends"]["day"], json!(4), "{a}");
    assert_eq!(a["ends"]["pulse"], json!("night"), "{a}");
    assert_eq!(a["ends"]["clock"], json!("D4 21:42"), "{a}");
    assert_eq!(a["ends"]["elapsed_hours"], json!("75.692307"), "{a}");
}

// --------------------------------------------- Distances that shape the game

/// Statement 16: From the Allied start (-3,5): 280 nm to the Striking Force's start
/// (4,-2); 589 nm to (-12,-3), the nearest hex of the convoy's route; 803 nm
/// to Port Moresby. Each by the game's measure. The convoy's route from
/// (-8,-9) passes (-12,-4), (-12,-3), (-13,-2) and (-14,-2) in that order.
#[test]
fn s16_distances_from_the_allied_start_and_the_convoys_route() {
    let mut s = session('d', &[A]);
    for (to, miles) in [((4, -2), 280), ((-12, -3), 589), ((-17, -4), 803)] {
        assert_eq!(air_distance(&mut s, (-3, 5), to), miles, "to {to:?}");
        // By the map's own arithmetic, to the mile.
        let (dq, dr) = (to.0 + 3, to.1 - 5);
        let worked = (40.0 * f64::from(dq * dq + dr * dr + dq * dr).sqrt()).round() as u32;
        assert_eq!(worked, miles, "to {to:?} worked");
    }
    let mut j = session('d', &[J]);
    let (_, route) = sea_route(&mut j, J, (-8, -9), (-17, -4));
    let at = |h: (i32, i32)| {
        route
            .iter()
            .position(|x| *x == h)
            .unwrap_or_else(|| panic!("{h:?} is on the convoy's route: {route:?}"))
    };
    let order = [(-12, -4), (-12, -3), (-13, -2), (-14, -2)].map(at);
    assert!(
        order.windows(2).all(|w| w[0] < w[1]),
        "in that order: {order:?}"
    );
    // (-12,-3) is the nearest hex of the route to the Allied start.
    let nearest = route
        .iter()
        .min_by_key(|h| air_distance(&mut s, (-3, 5), **h))
        .expect("a route");
    assert_eq!(*nearest, (-12, -3));
    assert_eq!(air_distance(&mut s, (-3, 5), *nearest), 589);
}

/// Statement 17: Strike radii, from the list: SBD-3 200 nm, SBD-2 and TBD-1 180, D3A1
/// 252, B5N2 175, B5N1 196, G4M1 611, G3M2 709.
#[test]
fn s17_the_strike_radii_are_the_lists() {
    let w = load('a').world;
    for (kind, radius) in [
        ("SBD-3", 200),
        ("SBD-2", 180),
        ("TBD-1", 180),
        ("D3A1", 252),
        ("B5N2", 175),
        ("B5N1", 196),
        ("G4M1", 611),
        ("G3M2", 709),
    ] {
        let t =
            &w.ref_data().aircraft_types[&sim_core::ids::AircraftTypeId::new(type_id(&w, kind))];
        assert_eq!(t.strike_range_nm, n(radius), "{kind}");
    }
    // And the A6M2's 261, which statement 18 uses.
    let a6 = &w.ref_data().aircraft_types[&sim_core::ids::AircraftTypeId::new(type_id(&w, "A6M2"))];
    assert_eq!(a6.strike_range_nm, n(261));
}

/// Statement 18: Rabaul to (-12,-3) is 40 x root(7 x 7 + 10 x 10 - 7 x 10) = 40 x root
/// 79 = 355.5 nm: inside both land bombers' radius, and beyond the A6M2's
/// 261, so the bombers go there without escort.
#[test]
fn s18_rabaul_to_minus_12_minus_3_is_355_nm_inside_the_bombers_and_beyond_the_fighters() {
    // 7 x 7 + 10 x 10 - 7 x 10 = 79; 40 x root 79 = 355.5.
    let worked = 40.0 * f64::from(7 * 7 + 10 * 10 - 7 * 10).sqrt();
    assert!((worked - 355.5).abs() < 0.05, "{worked}");
    let mut s = session('d', &[J]);
    let (r4, l4) = reach(&mut s, J, "4th Air Group", &[(-12, -3)]);
    let (r3, l3) = reach(&mut s, J, "Motoyama Air Group", &[(-12, -3)]);
    let (rf, lf) = reach(&mut s, J, "Tainan Air Group", &[(-12, -3)]);
    // The game's mile is the worked figure rounded to a whole mile.
    assert_eq!(l4, vec![(worked.round() as u32, true)]);
    assert_eq!(l3, vec![(worked.round() as u32, true)]);
    assert_eq!((r4, r3, rf), (n(611), n(709), n(261)));
    assert_eq!(
        lf,
        vec![(worked.round() as u32, false)],
        "beyond the A6M2's 261"
    );
    assert!(n(355) < r4 && n(355) < r3 && n(355) > rf);
}

// ------------------------------------ What a side the engine plays is given

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

/// Statement 19: File a's commands and orders of day 1 are those, for each side, with
/// both leaves, and no other. After day 1 each side's command state holds
/// its intents with no risk set; no Allied air group is held; and on the
/// morning of day 2 task force 2 has a route to (-12,-3) or has arrived
/// there.
#[test]
fn s19_the_orders_of_file_a_on_day_1_and_what_stands_on_the_morning_of_day_2() {
    let a = played('a');
    // The Japanese commands.
    let state = &a.after_day_1[&J];
    assert_eq!(commander_intents(state), BTreeSet::from([1, 2, 3, 4]));
    assert_eq!(
        intent(state, 1).kind,
        IntentKind::TakePlace {
            hex: hex(-17, -4),
            by_day: None
        }
    );
    let f = intent(state, 1)
        .limits
        .forces
        .as_ref()
        .expect("a forces limit");
    assert_eq!(f.task_forces, BTreeSet::from([tf(5)]));
    assert_eq!(f.land_units, BTreeSet::from([unit(2), unit(3)]));
    assert!(f.air_groups.is_empty() && f.ships.is_empty() && f.source.is_none());
    assert_eq!(
        intent(state, 2).kind,
        IntentKind::OperateCarriers {
            force: tf(3),
            hex: hex(-4, 1)
        }
    );
    assert_eq!(
        intent(state, 3).kind,
        IntentKind::OperateCarriers {
            force: tf(4),
            hex: hex(-10, -2)
        }
    );
    assert!(intent(state, 2).limits.forces.is_none() && intent(state, 3).limits.forces.is_none());
    assert_eq!(
        intent(state, 4).kind,
        IntentKind::Deny {
            hex: hex(-12, -3),
            distance: 2
        }
    );
    let f = intent(state, 4)
        .limits
        .forces
        .as_ref()
        .expect("a forces limit");
    assert_eq!(
        f.air_groups,
        BTreeSet::from([group(18), group(19), group(20)])
    );
    assert!(f.task_forces.is_empty() && f.land_units.is_empty() && f.ships.is_empty());
    for raw in 1..=4 {
        assert_eq!(
            intent(state, raw).limits.risk,
            None,
            "no risk set: balanced"
        );
        assert_eq!(intent(state, raw).set_day, 1);
    }
    assert!(state.leaves.prepare && state.leaves.launch);

    // The Allied commands.
    let state = &a.after_day_1[&A];
    assert_eq!(commander_intents(state), BTreeSet::from([1, 2, 3, 4]));
    assert_eq!(intent(state, 1).kind, IntentKind::Hold { base: base(1) });
    let f = intent(state, 1)
        .limits
        .forces
        .as_ref()
        .expect("a forces limit");
    assert_eq!(f.land_units, BTreeSet::from([unit(1)]));
    assert!(f.task_forces.is_empty() && f.air_groups.is_empty() && f.ships.is_empty());
    assert_eq!(
        intent(state, 2).kind,
        IntentKind::OperateCarriers {
            force: tf(1),
            hex: hex(-12, 1)
        }
    );
    assert!(intent(state, 2).limits.forces.is_none());
    assert_eq!(
        intent(state, 3).kind,
        IntentKind::StandAt {
            force: tf(2),
            hex: hex(-13, -2)
        }
    );
    assert_eq!(
        intent(state, 4).kind,
        IntentKind::Deny {
            hex: hex(-13, -2),
            distance: 1
        }
    );
    let f = intent(state, 4)
        .limits
        .forces
        .as_ref()
        .expect("a forces limit");
    assert_eq!(f.task_forces, BTreeSet::from([tf(1)]));
    assert!(f.air_groups.is_empty() && f.land_units.is_empty() && f.ships.is_empty());
    let t = intent(state, 4)
        .limits
        .targets
        .as_ref()
        .expect("a targets limit");
    assert_eq!(t.kinds, TARGETS.to_vec());
    assert!(t.waters_first);
    for raw in 1..=4 {
        assert_eq!(intent(state, raw).limits.risk, None, "balanced");
        assert_eq!(intent(state, raw).set_day, 1);
    }
    assert!(state.leaves.prepare && state.leaves.launch);

    // No order of a commander's own, and nothing held from the staff.
    assert!(
        a.orders.iter().all(|o| o.origin.issuer != Issuer::Player),
        "no order of a commander's own"
    );
    assert!(state.holds.is_empty(), "held: {:?}", state.holds);

    let m = &a.mornings[&2];
    assert!(
        m.tf2_at == hex(-13, -2) || m.tf2_route.last() == Some(&hex(-13, -2)),
        "task force 2 at {:?} with route {:?}",
        m.tf2_at,
        m.tf2_route
    );
    let waters: BTreeSet<Hex> = [
        (-13, -2),
        (-12, -2),
        (-14, -2),
        (-13, -1),
        (-13, -3),
        (-12, -3),
        (-14, -1),
    ]
    .into_iter()
    .map(|(q, r)| hex(q, r))
    .collect();
    for raw in [2, 3, 4, 6, 7, 8] {
        assert_eq!(m.lists[&raw], TARGETS.to_vec(), "group {raw}'s list");
        assert_eq!(m.waters[&raw], waters, "group {raw}'s waters");
    }
}

/// Statement 20: From (-3,5), (-12,1) is 40 x root(9 x 9 + 4 x 4 + 9 x 4) = 40 x root
/// 133 = 461.3 nm by air; the game's sea measure is stated by the test from
/// the game and is not below that.
#[test]
fn s20_minus_12_1_is_461_nm_by_air_and_the_games_sea_measure_is_not_below_that() {
    // 9 x 9 + 4 x 4 + 9 x 4 = 133; 40 x root 133 = 461.3.
    let worked = 40.0 * f64::from(9 * 9 + 4 * 4 + 9 * 4).sqrt();
    assert!((worked - 461.3).abs() < 0.05, "{worked}");
    let mut s = session('d', &[A]);
    assert_eq!(air_distance(&mut s, (-3, 5), (-12, 1)), 461);
    let (sea, _) = sea_route(&mut s, A, (-3, 5), (-12, 1));
    assert!(sea >= 461, "the sea measure is {sea}");
    // The game gives 461 nm by sea: open water the whole way.
    assert_eq!(sea, 461, "the sea measure");
    // The station's reach, by the game's air measure: three hexes of the
    // convoy's route are inside 180 nm, and Deboyne is 105.8.
    for (to, miles) in [
        ((-13, -2), 144),
        ((-12, -3), 160),
        ((-14, -2), 174),
        ((-10, -2), 106),
    ] {
        assert_eq!(air_distance(&mut s, (-12, 1), to), miles, "to {to:?}");
    }
}

// -------------------------------------------------------------- The files

/// Statement 21: The four files load and differ only in name and commands.
/// That each states its name and commands and nothing else is statement R3 of
/// `scenarios_built_on_another`.
#[test]
fn s21_the_four_files_load_and_differ_only_in_name_and_orders() {
    for letter in LETTERS {
        let s = load(letter);
        assert_eq!(s.days, 5, "file {letter}");
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
    }
    let names: BTreeSet<String> = LETTERS.iter().map(|l| load(*l).name).collect();
    assert_eq!(names.len(), 4);
}

// ---------------------------------------------------------- The objectives

/// Statement 22: The objectives are that table, the levels 6, 18 and 36, the horizon
/// 30, and the file says it is for play. The history note says that Shoho
/// was sunk and Lexington lost, that Yorktown and Shokaku were damaged, and
/// that the convoy turned back on 7 May and landed no one.
#[test]
fn s22_the_objectives_are_the_table_over_30_days_for_play() {
    for letter in LETTERS {
        let w = load(letter).world;
        let o = w.objectives().expect("objectives");
        assert!(o.for_play, "a scenario for play");
        assert!(text(letter).lines().any(|l| l == "for_play = true"));
        assert_eq!(
            (o.levels.minor, o.levels.major, o.levels.decisive),
            (n(6), n(18), n(36))
        );
        assert_eq!(o.horizon_days, 30);
        let history = o.history.as_deref().expect("a history note");
        for words in [
            "Shoho was sunk",
            "Lexington was lost",
            "Yorktown",
            "Shokaku were damaged",
            "turned back on 7 May",
            "landed no one",
        ] {
            assert!(history.contains(words), "{words}: {history}");
        }
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

/// Statement 23: Worked on made-up counts, through the game's own scoring: all 5,386
/// men landed and nothing else is 53.86 points to the Japanese side, a
/// margin of 53.86, a decisive victory. Three full transports sunk with
/// their 1,800 men and nothing else is 3 x 2 + 18 = 24 points to the Allied
/// side, a major victory. One carrier sunk each side and nothing else is 10
/// to 10, a stalemate.
#[test]
fn s23_made_up_counts_scored_by_the_games_own_scoring() {
    let w = load('a').world;
    let o = w.objectives().expect("objectives");
    let sunk = |kind: ShipKind, count: u32| BTreeMap::from([(kind, count)]);

    // All 5,386 men landed and nothing else.
    let mut japanese = Tally::default();
    japanese.men_landed.insert(base(1), 5386);
    let r = score(o, &BTreeMap::from([(J, japanese), (A, Tally::default())]));
    assert_eq!(r.japanese.total, fx("53.86"));
    assert_eq!(r.allied.total, Fixed::ZERO);
    assert_eq!(r.japanese.margin, fx("53.86"));
    assert_eq!(r.japanese.outcome, Outcome::Victory(Level::Decisive));
    assert_eq!(r.allied.outcome, Outcome::Defeat(Level::Decisive));

    // Three full transports sunk with their 1,800 men and nothing else.
    let allied = Tally {
        sunk: sunk(ShipKind::Transport, 3),
        men_lost: 1800,
        ..Tally::default()
    };
    let r = score(o, &BTreeMap::from([(A, allied), (J, Tally::default())]));
    assert_eq!(r.allied.total, n(3 * 2 + 18));
    assert_eq!(r.allied.total, n(24));
    assert_eq!(r.japanese.total, Fixed::ZERO);
    assert_eq!(r.allied.margin, n(24));
    assert_eq!(r.allied.outcome, Outcome::Victory(Level::Major));

    // One carrier sunk each side and nothing else.
    let each = || Tally {
        sunk: sunk(ShipKind::Carrier, 1),
        ..Tally::default()
    };
    let r = score(o, &BTreeMap::from([(A, each()), (J, each())]));
    assert_eq!((r.allied.total, r.japanese.total), (n(10), n(10)));
    assert_eq!(r.allied.margin, Fixed::ZERO);
    assert_eq!(r.allied.outcome, Outcome::Stalemate);
    assert_eq!(r.japanese.outcome, Outcome::Stalemate);
}

// ----------------------------------------------------------- Rules switched on

/// Statement 24: The header's switches are those of scenario 103's base: fatigue,
/// wear, supply and the orders inside a day, the live figures files. Command
/// levels are off.
#[test]
fn s24_the_switches_are_those_of_scenario_103s_base() {
    let ours =
        std::fs::read_to_string(common::data_dir().join("scenarios/coral-sea/base_coral_sea.toml"))
            .expect("the base reads");
    let theirs = std::fs::read_to_string(
        common::data_dir().join("scenarios/five-days/base_eastern_solomons.toml"),
    )
    .expect("scenario 103's base reads");
    let line = |text: &str, key: &str| -> Option<String> {
        text.lines()
            .find(|l| l.starts_with(&format!("{key} = ")))
            .map(str::to_string)
    };
    for key in [
        "air_upkeep",
        "fatigue",
        "air_supply",
        "ground",
        "orders_inside_a_day",
        "search",
        "surface_combat",
        "air_combat",
        "trial",
    ] {
        assert!(line(&ours, key).is_some(), "{key} is set");
        assert_eq!(line(&ours, key), line(&theirs, key), "{key}");
    }
    // Command levels are off: nothing of them in any file.
    for letter in LETTERS {
        let t = text(letter);
        assert!(
            !t.lines().any(|l| {
                !l.starts_with('#') && (l.contains("command_levels") || l.contains("[[superior"))
            }),
            "file {letter} has command levels"
        );
    }
    // And the rules in play are scenario 103's.
    let a = load('a').world;
    let b = common::load_live(&path_103())
        .expect("scenario 103 loads")
        .world;
    assert_eq!(
        a.side_view(A).rules_in_play(),
        b.side_view(A).rules_in_play()
    );
    let rules = a.side_view(A).rules_in_play();
    assert!(rules.wear && rules.fatigue && rules.supply, "{rules:?}");
}

fn path_103() -> PathBuf {
    common::data_dir().join("scenarios/five-days/01a_eastern_solomons.toml")
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

/// No real ship has a figure that is not the list's: every ship not named
/// "Placeholder" is of a class the list derives, the class the list gives
/// her.
#[test]
fn every_real_ship_is_of_a_class_the_list_derives() {
    let derived: BTreeSet<u32> = rows("ship_classes_coral_sea.csv")
        .iter()
        .map(|r| r[0].parse().expect("a class id"))
        .collect();
    for letter in LETTERS {
        let w = load(letter).world;
        let mut real = 0;
        for s in w.umpire().ships.values() {
            if s.name.starts_with("Placeholder") {
                continue;
            }
            real += 1;
            assert!(
                derived.contains(&s.class.raw()),
                "{} class {}",
                s.name,
                s.class.raw()
            );
            let slug = REAL
                .iter()
                .find(|r| r.1 == s.name)
                .unwrap_or_else(|| panic!("{} is a ship of the list", s.name))
                .2;
            assert_eq!(s.class.raw(), list_class(slug), "{}", s.name);
            // The class is the list's own row, not another file's.
            assert!((4000..=4024).contains(&s.class.raw()), "{}", s.name);
        }
        assert_eq!(real, 38, "file {letter}: 38 real ships");
        // The aircraft types of every group are the list's too.
        let types: BTreeSet<u32> = rows("aircraft_types_coral_sea.csv")
            .iter()
            .map(|r| r[0].parse().expect("a type id"))
            .collect();
        for g in w.umpire().air_groups.values() {
            assert!(types.contains(&g.aircraft_type.raw()), "{}", g.name);
        }
    }
}

/// In files c and d, where the Japanese side is given no orders, the convoy
/// does not move and no man lands.
#[test]
fn where_the_japanese_side_is_given_no_orders_the_convoy_does_not_move_and_no_man_lands() {
    for letter in ['c', 'd'] {
        let p = played(letter);
        assert_eq!(p.convoy.len(), 5);
        for (day, (at, ashore)) in p.convoy.iter().enumerate() {
            assert_eq!(*at, hex(-8, -9), "file {letter} day {}", day + 1);
            assert_eq!(*ashore, 0, "file {letter} day {}: no man ashore", day + 1);
        }
        assert!(
            !p.orders.iter().any(|o| o.side == J),
            "file {letter}: no Japanese order of any kind"
        );
        let t = p.end.umpire();
        let aboard: u32 = t
            .ships
            .values()
            .filter_map(|s| s.troops)
            .map(|tr| tr.men)
            .sum();
        // No man has gone ashore; the men aboard are all there are but for
        // any lost with a transport sunk where the convoy lies (the Allied
        // carriers, given orders in file c, may strike it there).
        assert!(aboard <= 5386, "file {letter}: {aboard} aboard");
        if letter == 'd' {
            assert_eq!(aboard, 5386, "file d: every man still aboard");
        }
        assert!(t.task_forces[&tf(5)].route.waypoints.is_empty());
    }
}

/// Nothing in a side's view names an enemy ship, group or unit by its truth
/// before it is sighted.
#[test]
fn nothing_in_a_sides_view_names_an_enemy_before_it_is_sighted() {
    for letter in LETTERS {
        let w = load(letter).world;
        let truth = w.umpire();
        for side in Side::ALL {
            let view = w.side_view(side);
            // The view's own units are the side's.
            assert!(view.ships().all(|s| truth.ships[&s.id].side == side));
            assert!(view.task_forces().all(|f| f.side == side));
            assert!(view.air_groups().all(|g| g.side == side));
            assert!(view.land_units().all(|u| u.side == side));
            assert_eq!(
                view.contacts().count(),
                0,
                "nothing is sighted at the start"
            );
            // The enemy's bases are a place and a side and no more.
            assert!(view.enemy_bases().all(|b| b.side != side));
            // Everything the side's tools say, with the enemy's names looked
            // for in it.
            let mut s = session(letter, &[side]);
            let mut said = String::new();
            for tool in ["get_briefing", "list_forces", "list_contacts", "list_bases"] {
                said.push_str(
                    &s.call(side, tool, &json!({}))
                        .expect("a tool answers")
                        .to_string(),
                );
            }
            // The text is the side's own picture: it names the side's own.
            let own = if side == A { "Yorktown" } else { "Shokaku" };
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
