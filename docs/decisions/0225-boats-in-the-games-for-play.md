# 0225: Boats in the games for play

Status: a plan by the architect, written on Nov 23, 2026. The designer said on that day that boats need not be held back from the games he plays. The rows and gaps are in `docs/research/submarines-in-the-play-scenarios.md`; the rules are decision 0218's.

Summary: The record places few boats well enough to put on a map. This puts in those it does, in games of their own at the table so that no file a test, a proof or a copy reads is touched, beginning with Midway, where the stations are on file. It names the data that must be found before the Solomons and the Coral Sea can have theirs, and it measures each game before and after.

Standing: Planned on Nov 23, 2026; amended on review and by the designer's answer; the Midway games built on branch `boats-midway`, to be reviewed by the architect.

## What the rows allow

- **A boat can be placed only where** a ship row ties her to a class that is built (class ids 6000 to 6011), a row gives where she was on a day of the game, and her class has a speed.
- **Midway.** American: Nautilus, Tambor, Trout (the arc 240 to 360 degrees, 150 miles from Midway) and Gato (the arc 250 to 310 degrees, 200 miles), by Bates's disposition of 2 June (positions 16009 and 16010). Eleven more American boats are named on the arcs and have no ship row. Japanese: I-168 alone lies on the map; the two cordons lie east of its edge.
- **The Coral Sea.** American S-44 and S-47 off Rabaul and New Ireland, by named waters with no position. No Japanese boat can be placed: six are named in the gaps with no station.
- **Guadalcanal.** No row gives a station for any boat of either side. The boats that hit Saratoga and Wasp have no ship rows.
- **Each side's belief** has rows for Midway (the Allied side: one boat west of Midway, 29 May) and for the Allied side at Guadalcanal; the Japanese side at Guadalcanal has none.

## The rules

1. **Boats go into games of their own.** A folder `data/scenarios/table-boats/` holds a copy of each table game that gains boats, made by a script from the game it copies, with a name no other scenario has. The table offers it beside the game without boats. No file that a test, a proof, a baseline or a table copy reads is changed; the tests that list every file for play gain the new names and nothing else.
2. **Midway is first.** Its copies gain:
   - **Nautilus, Tambor, Trout and Gato,** each at the hex nearest her place on her arc. The row gives the arc and the boats on it, in an order, and no bearing for each. The rule: the boats a row names are spread evenly along its arc in the order the row names them, the first at one end and the last at the other, and each boat's place is the point at the arc's distance on her bearing. All the boats the row names are counted in the spread, those with no ship row among them, so that the four stand where the row would have them and the others' places are left empty. Each place is marked as estimated from the row by this rule.
   - **I-168** at the hex of her first position the rows give in the game's days.
   - **Each boat's home** is her side's base on the map nearest her, as a stand-in named as such: the true homes lie off the map.
   - **Each side's suspected waters** where a row dated on or before the game's first day gives them, by the rule of decision 0218 (25 and 47): told on the day of the row, or on day 1 if earlier.
   - **No order:** each boat keeps her station. A side the staff plays alone is given no patrol order here.
3. **The eleven American boats with no ship row are not placed,** and are named in the game's notes as left out. Their ship rows are research owed (rule 6).
4. **Each game is measured before and after** on seeds 1 to 100, staff against staff: ships sunk and damaged by cause and by side, carriers lost, the result, and for each boat the attacks made, the hits, and whether she was hunted, damaged or sunk. The figures are reported beside what the rows say those boats did. Nothing is changed for the result.
5. **The attack chance is watched, not tuned.** At 0.02 an hour a force that lies a whole day in a boat's hex is attacked in 38 runs of 100, where the rows give about 1 to 5 attacks in 100 boat-days. The measure counts force-hours in boats' hexes and attacks made, so that the figure can be judged against the rows on every game and by one rule, later, and not on this one.
6. **Before the Solomons and the Coral Sea have boats, three things are found:**
   - ship rows for the boats named in the rows that have none (the I-15 class boats, I-11, the RO boats, S-37, S-41, S-42, S-43, and the eleven at Midway);
   - a station by position and date for each boat, where any source gives one; a boat known only by "off Rabaul" is given a patrol order of those waters and no station of her own, once the staff's pattern for a patrol is built (decision 0223, stage A);
   - reliefs by date over the 39 days, since a boat that goes home does not return.

## What it does not do

- It adds no rule and changes no figure.
- It puts no boat in the one-day games, which have no staff.
- It does not say the games with boats replace those without. The designer chooses at the table.

## Before it is built

Reviewed by a fresh reader; then the architect's statements for the Midway copies: each boat's hex worked from its row by rule 2, and what each game must hold on day 1.

## Amended on review, Nov 23, 2026

A fresh reader was given this record, the research, the rows, the Midway files and the tests, and worked the hexes with the map's own projection. The architect accepts the findings. Where this section and the rules above differ, this section holds.

### What the review found

- **Rule 4 measured a game rule 1 did not make:** staff against staff is file a, which the table does not play.
- **Rule 2 did not say which end of an arc is first, which way the arc runs, or where it is measured from.** Two readings put Tambor, Trout and Nautilus in different hexes.
- **I-168's one row in the positions file is of 6 June,** where she sank Yorktown. On the game's first days the rows have her at Midway itself.
- **The even spread is not an estimate in the sense of decision 0207,** which is of a ship's figures from like ships. It is a reading of a row, of the kind the Midway base marks "PLACEHOLDER start".
- **Decision 0218 says of its figures "each is to be set from the rows before a game is played with it".** Rule 5 plays them unset.
- **The rows of belief are sentences, not waters:** "one west of Midway" has no distance.
- **Adding a folder is more than new names:** two tests exempt folders by name, one lists every file for play, the table's check pins the list of games and their pairings, and the table takes the first pairing whose seats match.
- **The arcs name seven boats with no ship row, not eleven;** four more are named elsewhere.

### The rules, as amended

1. **The folder `data/scenarios/table-boats/` holds four Midway games:** a copy of each of the table's b, c and d and one of a, on the table's ground figures, made by a script from the table's copies (a from the game's own file a), with file names no other scenario has. The table offers b, c and d as a game of their own, "Midway, with submarines". File a is for the measure and is not offered.
2. **Where each boat starts** (replacing rule 2's first two items). An arc is read clockwise from its lower bearing, the first boat a row names at the lower bearing and the last at the higher, the boats between at even steps. Bearings are true from the atoll's own position, laid off flat in the map's frame at the arc's distance, and the boat starts in the hex that point falls in. Every boat the row names is counted. That gives, from row 16010 (240 to 360 degrees, 150 miles, six boats, steps of 24 degrees) and row 16009 (250 to 310 degrees, 200 miles, three boats, steps of 30):

   | Boat | Place in her row | Bearing | Hex |
   | --- | --- | --- | --- |
   | Tambor | first of six | 240 | (1, 3) |
   | Trout | second of six | 264 | (1, 1) |
   | Nautilus | fourth of six | 312 | (4, -2) |
   | Gato | second of three | 280 | (1, -1) |

   Each is marked "PLACEHOLDER start" with the row and this rule, as the game's other starts worked from rows are. Nautilus's stated position at 0710 on 4 June (row 16002: 30-00 N, 179-25 W, the hex next to hers) agrees with this reading of the order and is not used as her start: it is of the second day.
3. **I-168 starts in Midway's hex, (5, 1),** where rows 16202 and 7673 have her on 1, 2 and 4 June. There she attacks nothing by the rule (a force in the hex of its own base is not attacked). What the rows say she did, the attack of 6 June 159 miles away, takes an order to go there, which a player may give and the staff does not.
4. **Homes.** The American boats' home is Midway and I-168's is Wake, each a stand-in named as one. Two effects are accepted and named in the game's notes: a boat going home to a base that has changed side still leaves the map there; I-168's way home is five days on passage, sighted at the whole chance.
5. **No suspected waters in these games.** The rows (16842, 16518, 16202) give no distance, and waters would change where the staff stands its forces, so that the game with boats would differ from the game without by more than boats. They are named in the notes as left out.
6. **No order is scripted.** The rows that moved the boats (16181, to close the enemy on 4 June; 16182, the circle of 100 miles by dawn on 5 June) are named in the notes. A player orders his boats; a side the staff plays leaves its boats on their stations. The two sides are so unequal in every pairing, and the notes say so.
7. **The figures are played unset,** on the designer's word of Nov 23, 2026 that boats are not to be held back. Decision 0218's sentence stands amended to that extent, and the game's name at the table says it is a trial. The classes are those derived as of 7 August 1942; the five boats' classes did not change between June and August.
8. **The measure** (replacing rule 4): file a's copy on seeds 1 to 100 against the same file with the boats taken out, made for the run and not kept. Reported: ships sunk and damaged by cause and side, carriers lost, the result; for each boat the pulses in which a force lay under her chance and the force-hours in them (from `SubmarineAttackChance`, the only record of them), the salvoes, hits, hunts, sightings from the air, and whether she was damaged, sunk or went home; and the hex of each Japanese force at each day's end, so that an empty count can be told from a force that never came near. A force of destroyers alone is under no chance, so Grouper's attack of the rows could not happen here; she is not in the game.
9. **What the tests gain, and nothing else in them changes:** `table-boats/` joins the folders the two hunger tests exempt (scenario 119 near line 1040, scenario 123 near line 1527); scenario 104's statement 23 gains the three files for play; the table's check gains the new game and its pairings; a new test runs the new script with `--check` and holds the statements below. The header of each copy is written anew: it is made by the new script, is offered at the table, has submarines, and names the estimated figures of classes 6001, 6002 and 6011 as the data rules require.
10. **The directive** for each seat of the new game gains one sentence naming the boats the side has and where.
11. **No objective counts a boat sunk.**
12. **Decision 0223 as amended moves no boat without a patrol order of the commander's,** so these boats keep their stations when it is built.
13. **Rule 6 above gains:** S-39 and S-46; I-11's class; the speeds of the I-9 and I-5 groups; I-22, I-24, I-28 and I-29; the Japanese belief of Allied boats in August and September. A boat known by "off Rabaul" may be given a patrol order of those waters today; it does not wait for decision 0223.

### The statements the new test holds

Each copy, loaded:

1. It has five submarines: Tambor, Trout, Nautilus and Gato of the Allied side and I-168 of the Japanese, and no other.
2. They start at (1, 3), (1, 1), (4, -2), (1, -1) and (5, 1) in that order, each on station there, each in a hex ships can enter.
3. Tambor and Trout are of class 6001, Gato of 6002, Nautilus of 6011 and I-168 of 6005, each with her class's torpedo and salvoes.
4. The Allied boats' home is Midway and I-168's is Wake.
5. It has no suspected waters.
6. Taken with its submarines and their three keys removed, it is the game it copies, line for line but for its header and its ground figures as the copy it comes from has them.
7. Played on seed 1 with no order to any boat, every boat still afloat at the end is in the hex she started in.

### The hexes, worked by hand

The atoll (28.232 N, 177.370 W, which is 182.630 E) lies at east 217.77 nm, north -16.08 nm in the map's frame (east = (lon - 178.5) x 60 x 0.878817; north = (lat - 28.5) x 60). A point at distance d on bearing b is east + d sin b, north + d cos b. A hex (q, r) is centred at east 40 (q + r/2), south 34.641 r, and a point is in the hex whose centre is nearest.

| Boat | d sin b, d cos b | Point (east, north) | Nearest centre | Next nearest |
| --- | --- | --- | --- | --- |
| Tambor, 150 at 240 | -129.90, -75.00 | 87.87, -91.08 | (1, 3) at 100, -103.92: 17.6 nm | (1, 2): 23.2 nm |
| Trout, 150 at 264 | -149.18, -15.68 | 68.59, -31.76 | (1, 1) at 60, -34.64: 9.1 nm | |
| Nautilus, 150 at 312 | -111.47, 100.37 | 106.30, 84.29 | (4, -2) at 120, 69.28: 20.3 nm | (4, -3): 20.6 nm |
| Gato, 200 at 280 | -196.96, 34.73 | 20.81, 18.65 | (1, -1) at 20, 34.64: 16.0 nm | (1, 0): 26.8 nm |

Nautilus's point lies within a third of a mile of the line between two hexes; the rule gives (4, -2), and her stated position of 4 June is in the other.

## The designer's answer, Nov 23, 2026

The table offers one game of each battle, and where the battle has boats that game has them. Amended rule 1 changes so: there is no second game 'Midway, with submarines (trial)'. The table's existing Midway game plays the copies with submarines: its pairings for b, c and d name the files in `data/scenarios/table-boats/`, its name stays 'Midway', and its directives gain the sentence of rule 10. The copies without submarines stay on disk, since tests and the copies' own checks read them, and the table no longer offers them. Rule 7's 'the game's name at the table says it is a trial' is withdrawn; the figures being unset is said in the game's notes and in this record. The same holds for the Coral Sea and Guadalcanal when their boats go in.

## As built, Nov 23, 2026

Built on branch `boats-midway` by a team lead for the architect, to the amended rules 1 to 12, the statements and the hexes as written. Nothing in `crates/` was changed.

### File by file

- `scripts/table-boats/make_files.py` (new): makes the four copies, or with `--check` says whether each kept is the one it makes and whether the folder holds anything else. Each copy is the file it copies with its header written anew (rule 9), then the copied header as it stands under a line saying where it no longer holds (it says the submarines are left out); the three keys of the submarine files after the `surface_combat` line; a section SUBMARINES at the end, one `[[submarine]]` for each boat with her ship row, her class's row, the row her start is read from, the bearing and the distance from the hex's centre, and her stand-in home; and nothing else. File a's copy also names `ground_hunger.toml`, as `make_copies.py` changes the others.
- `data/scenarios/table-boats/01a_midway_submarines.toml` to `01d_midway_submarines.toml` (new, made by the script): from `midway/01a_midway.toml` and `table-hunger/01b_midway.toml` to `01d`. The boats: submarine 1 USS Tambor (1,3), 2 USS Trout (1,1), 3 USS Nautilus (4,-2), 4 USS Gato (1,-1), all Allied with Midway (base 1) as home; 5 I-168 (5,1), Japanese, Wake (base 2) as home; classes 6001, 6001, 6011, 6002, 6005; each on station in her hex, with her class's salvoes, sound, no order. The names are the rows' of `ships.csv` (6321, 6322, 6317, 6326, 6424). The header names: made by the script; offered at the table, or for a kept for the measure; that it has submarines and where; each start a PLACEHOLDER start with its row and the rule; the stand-in homes and their two effects; the seven boats on the arcs with no ship row left out; the rows of belief left out (16842, 16518, 16202) and why; the rows of movement not played (16181, 16182); the unequal sides; the estimated figures of classes 6001 (torpedo), 6002 (torpedo) and 6011 (passage speed and torpedo), citing decision 0207; the figures played unset (rule 7).
- `table/scenarios.json`, to the designer's answer: no new game. The entry `midway` keeps its name and its seven pairings, which now name `table-boats/01b_midway_submarines.toml`, `01c_` and `01d_` in place of `table-hunger/01b_midway.toml` to `01d`; its lines gain one saying that each side has its submarines on station and where, and its note says where the Midway files come from. `table/directives/midway_allied.json` and `midway_japanese.json`, edited in place: one sentence added at the end of the situation, "Four of our submarines are on station and under your command: Tambor, Trout and Nautilus on the arc 150 miles west and north-west of Midway, and Gato 200 miles west of it." and "The submarine I-168 is off Midway itself, where she has been reconnoitring the atoll, and is under your command.", and the note says so. The copies' header says the figures are played unset; nothing at the table calls the game a trial.
- `table/check_screen.py`: a check that the Midway entry, still named Midway, plays the three copies with submarines and that each side's directive ends its situation with the one sentence naming its boats; a check through the bridge that each seat of Midway has its own boats on station and none of the other's, and that a station order to one is queued. The list of games and their counts are as they were. `table/README.md`: Midway's boats and the check. `scripts/table-copies/make_copies.py`: a comment that Midway's copies are kept though the table plays the copies of them with submarines.
- Tests: `tests/boats_in_the_midway_games.rs` (new, registered in `tests/Cargo.toml`): the seven statements as the architect wrote them in its doc comment, one test each, on the live figures (no statement holds a number that rests on a figure; statement 3 compares a boat's salvoes and torpedo with the class file loaded), and the script run with `--check`. `tests/scenario_119_hunger_takes_time.rs` and `tests/scenario_123_a_fight_costs_supply.rs`: `table-boats/` joins the folders exempted. `tests/scenario_104_men_put_ashore.rs`: statement 23's list gains the copies. No other expectation changed.
- `docs/commands.md`: the script.

### Where the build differs from the plan

- **Statement 23 of scenario 104 gains four files, not three.** File a's copy carries `for_play = true` because file a does, and statement 6 forbids taking a line out; so the list names all four copies, with a comment saying why.
- **The copies keep the names of the files they copy** (`name = "Scenario 114, file b: ..."`), since statement 6 allows only the header and the ground line to differ. The table shows the game's own name from `scenarios.json`.
- **No test asserted which files the table's Midway plays.** Scenario 121's `property_the_table_s_copies` reads `scenarios.json` for the Coral Sea's copies alone, and scenario 114's statements 20 and 21 read the `table-hunger/` Midway files themselves and scenario 104's text, not `scenarios.json`; none was restated. `table/check_screen.py` is the one check that names the Midway files the table plays, and it is changed as above.
- **The table's check runs on six ports**, the one given and the three from it plus three; run with `--port 8796` it used 8796 and 8799 to 8801.

### Found at the table, not mended (outside this change)

- **In a game where both sides have boats, `queued` names a boat by the engine's number while every other answer names her by the side's own.** For the Japanese seat I-168 is `submarine:1` in `list_forces` and on her card, and a station order given her is listed by `queued` as `submarine:5`, so SIGNED reads "submarine 5: take station at ..." in place of "I-168", and the number tells the side that the enemy has four boats numbered before her. The order itself acted on I-168 at 1800. This is in `sim-agent`, which this change does not touch; it is the first game with boats on both sides.
- I-168's card gives her class as the class file's full name ("I-168 to I-175, fleet submarines built 1934 on (the Technical Mission's block); the Japanese type code is not stated by the rows").

### The measure (amended rule 8)

File a's copy on seeds 1 to 100, staff against staff, against the same file with its section of boats and its three keys taken out, made for the run outside the repository with its paths made whole. Each game was read from `sim-cli export --seed N` (the events with their causes, the state after every step, the final state) and the result from `sim-cli run --seed N`; the script and its output (`runs.jsonl`, one line a run) are kept with the build's notes, not in the repository. How each figure was read: a ship sunk is a `ship_sunk` event, its cause found by walking its causes up to the first `air_attack` (air) or `submarine_salvo` (submarine); a ship damaged is one with a `ship_damaged` event and afloat at the end, counted once for each cause that hit her; a carrier is a ship of kind carrier in `initial.ships`; the result is the Allied line of `run`; for each boat, a pulse under a chance and its force-hours are the `submarine_attack_chance` events (their day and pulse, and the hours their text gives), salvoes, torpedoes and hits the `submarine_salvo` events, hunts `submarine_hunted`, looks and sightings from the air `submarine_search_attempt`, attacks from the air `submarine_attacked_from_air`, and her end her condition in `final.submarines`; the closest an enemy force came is the least hex distance from her hex to any task force of the other side after any step; the Japanese forces' hexes are their places after each day's last step.

| | Without boats | With boats |
| --- | --- | --- |
| Ships sunk, Allied (air, submarine, other) | 54, 0, 0 | 54, 0, 0 |
| Ships sunk, Japanese | 276, 0, 0 | 276, 0, 0 |
| Ships damaged and afloat, Allied, by air | 49 | 49 |
| Ships damaged and afloat, Japanese, by air | 186 | 186 |
| Carriers lost, mean, Allied / Japanese | 0.53 / 2.74 | 0.53 / 2.74 |
| Allied outcome: decisive, major, minor victory; stalemate; minor, major, decisive defeat | 3, 11, 13; 21; 26, 12, 14 | the same |
| Allied margin, mean / median | -6.41 / -5.52 | the same |

Every run's result and ships sunk are the same with the boats and without (100 of 100).

| Boat | Pulses under a chance | Force-hours | Salvoes | Hunts | Looked for / sighted / attacked from the air | End: sound, damaged, sunk, home | Closest enemy force, runs at 0 / 1 / 2 / 3 or more hexes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| USS Tambor (1,3) | 0 | 0 | 0 | 0 | 0 / 0 / 0 | 100, 0, 0, 0 | 0 / 2 / 87 / 11 |
| USS Trout (1,1) | 0 | 0 | 0 | 0 | 0 / 0 / 0 | 100, 0, 0, 0 | 0 / 0 / 0 / 100 |
| USS Nautilus (4,-2) | 0 | 0 | 0 | 0 | 380 / 14 / 14 | 99, 0, 1, 0 | 0 / 100 / 0 / 0 |
| USS Gato (1,-1) | 0 | 0 | 0 | 0 | 0 / 0 / 0 | 100, 0, 0, 0 | 0 / 0 / 0 / 100 |
| I-168 (5,1) | 0 | 0 | 0 | 0 | 1,230 / 264 / 264 | 56, 20, 13, 11 | 0 / 0 / 0 / 100 |

Where the Japanese forces ended each day (the commonest hex, with the runs in it): the First Carrier Striking Force at (5,-3) on every day (96 runs on day 1, 100 after), one hex from Nautilus; the Midway Invasion Force at (-12,4), (-6,4), (-1,4), then Midway's hex (5,1) on days 4 and 5 (100, 97, 87, 73 and 79 runs), along a row south of the arcs. No Japanese force ended a day in an American boat's hex in any run, and no boat on her station was ever under the chance (no `submarine_attack_chance` event in any run), so the count of attacks is empty because no force came into a boat's hex, not because the chance failed.

Beside the rows: Nautilus fired five torpedoes in three attacks on 4 June, one at Soryu (engagement_events 16063 to 16086); Tambor reported Cruiser Division 7 on 5 June (16093); I-168 sank Yorktown and Hammann 159 miles east of Midway on 6 June (16052, 16053) and was tracked from Eastern Island and not fired on (7673). In the game the American boats made no attack, since the Japanese carriers stand at (5,-3) and the invasion force comes in along row 4; and I-168, at Midway, is looked for by Midway's flying boats in every morning's search, sighted at the share of a boat on station, and attacked: sunk in 13 runs and sent home damaged in 31. Nothing was changed for the result. The measure was run again after main was merged in (decision 0222's engine, pull request 702), and every table is the same.

## A ruling on the derivation, Nov 23, 2026

The research of Nov 23, 2026 (`docs/research/submarine-rows-for-the-play-scenarios.md`, pull request 705) filed eight new submarine classes with speed rows, and ONI 222-J's torpedo line for the I-15 class. As `oob-derive` stood, the new classes' rows would have entered the pools of like classes of the S-boats' and Narwhal's estimated passage speeds, and the I-15 class's rows, though dated 1945, would have driven out its `torpedo_from` entry. Research on file must not change what the game reads until the architect rules on it. So:

1. **A class named in a `[[left_out]]` entry of the list is in no like-class pool, unless its entry carries `like_class = true` with a reason** (`like_class_reason`). It is set on `jp-i-1-group` only: "in the pool of like classes as it has been since the list was first derived; whether it stays is the architect's to rule with the new rows (decision 0225, rule 13)". The loader refuses `like_class` without a reason, and an unknown key as it does today.
2. **In `check_torpedo_from`, only a `torpedo_model` row that holds on the list's date drives a `torpedo_from` entry out;** a row beginning after that date is not read, as no row after the list's date is. Rows 17236 and 17237 stay as filed.

The eight new classes are left out of `submarines_1942.toml` with the reason "On file, not derived: waits on the architect's ruling (decision 0225, rule 13)". The generated tables' rows are as before; the class file's header lists the eight among the classes left out.

### The architect's rulings on pull request 706 (Midway)

Read: the new test and its statements, the boats as the copies give them, and the measure.

1. **Built as amended and as the designer answered:** one Midway at the table, with its five boats. The statements are held as written. Scenario 104's statement 23 gains four files and not three, because file a's copy carries `for_play` as file a does; that is the statement working as written.
2. **The boats change nothing in the game the staffs play.** On 100 seeds every result and every ship sunk is the same with the boats and without, and no boat is ever under a chance to attack: no Japanese force ends a pulse in an American boat's hex, and I-168 lies where she may not attack. That is the honest state of this game: the boats are the player's to use, and until he orders one she watches. It is also what rule 5 was written to find out. The attack chance cannot be judged on this game.
3. **I-168 is found by Midway's flying boats every morning** and is sunk in 13 runs of 100 and sent home damaged in 31. The rows have her off Eastern Island for days, watched and not attacked. She is left where the rows put her and nothing is tuned. What it shows is owed to the figures: a boat on station in the hex of an enemy air base is looked for in every pulse at the full count of the base's search. Open work, with the attack chance, for when the submarine figures are set from the rows on every game together.
4. **A boat's number under SIGNED is the engine's and not the side's** (`queued` in `sim-agent`): the Japanese seat reads "submarine 5" for his only boat and can count four boats before her. This tells a side something of the enemy and is mended at once, in its own pull request, before anything else is built on this game.
5. **Not seen:** the agent pairings in a browser; the whole test suite after the last two merges of main, which CI runs.

## Carried on, Nov 23, 2026

Rule 6 and amended rule 13 are answered by the research of Nov 23, 2026 and by decision 0227, which places the boats of the Coral Sea, the five days and the 39 days, adds ten boats to Midway, states each navy's pattern in every game with boats, and rules on the classes and the rows in dispute. Statements 1, 2, 3 and 6 above are restated there for Midway's 15 boats.

