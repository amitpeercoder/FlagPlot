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
//! group's order, and nine transports that could not load). Two more are
//! changed for a reason the mission gives and the trial showed: what the
//! Allied aircraft strike first, and a station from which every Allied
//! bomber reaches the convoy's route. Nothing else is changed in its
//! light, and the Japanese orders are as first written.
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
//! - an order to each of the six bomber groups, 2, 3, 4, 6, 7 and 8, to
//!   strike transports first, then carriers, then the other kinds in the
//!   game's own order. The mission is to stop the landing, and a strike
//!   that takes the most important kind first spends itself on the
//!   escort;
//! - an order to task force 2 to move to (-12,-3), on the convoy's route
//!   at the tip, as the support group was sent to bar the way. A staff
//!   stands a surface force only from a home port, and this force has
//!   none, so no intent would move it: the order is the commander's own.
//!
//! 19. File a's commands and orders of day 1 are those, for each side,
//!     with both leaves, and no other. After day 1 each side's command
//!     state holds its intents with no risk set; the six Allied bomber
//!     groups have transports first on their target lists on the morning
//!     of day 2 as on day 1 (the staff has not put them back); and task
//!     force 2 has a route to (-12,-3) or has arrived there.
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
//! aircraft and the support group did; whether the order of what to
//! strike first held through the staff's mornings; and the result. If the
//! orders left out something the mission asks for, the architect mends
//! them then. What the reviewer's trial showed is expected again and is
//! reported, not mended, in this build: the men ashore count the garrison
//! and wait for twice its number, so Port Moresby is not attacked; the
//! Striking Force closes only on a contact it takes for a carrier and may
//! stay out of the fight for days; Shoho's nine torpedo planes are her
//! force's search and do not strike.
//!
//! The tests are written by the build under these statements.
