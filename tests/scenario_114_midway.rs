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
//! - **Operate** task force 1 from (14,-6), Point Luck (place positions
//!   5113), and task force 2 from (15,-6) (rule 99).
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
