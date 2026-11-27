# 0227: Boats in every game for play

Status: a plan and rulings by the architect, written on Nov 23, 2026. It carries on decision 0225, whose rule 6 and amended rule 13 named the research owed before the Coral Sea and the Solomons could have boats. That research is on file (`docs/research/submarine-rows-for-the-play-scenarios.md`, rows 17200 and up). The patterns are decision 0223's, stages 0 and A.

Summary: The table offers one game of each battle, and where the battle has boats that game has them (the designer, Nov 23, 2026). This says which boats each side has in the Coral Sea, the Eastern Solomons over five days and the 39 days of Guadalcanal, and which more Midway gains, each from a row; states each navy's pattern in every such game; derives three more classes and leaves six waiting; and rules on the rows in dispute. It adds no rule and changes no figure but the three classes' and the estimates that read them.

Standing: Planned on Nov 23, 2026; amended on review the same day; built and merged (pull request 717; see "As built").

## What decision 0225 settled, and stands

- **Boats go into copies in `data/scenarios/table-boats/`,** made by `scripts/table-boats/make_files.py` from the table's copies. No file a test, a proof or a baseline reads is changed.
- **No order is scripted.** A player orders his boats. A side the staff plays leaves its boats where they start, since the staff moves a boat only on a patrol order of the commander's (decision 0223 as amended).
- **No suspected waters,** for the reason of 0225's amended rule 5. The rows of belief are named in each game's notes.
- **The figures are played unset,** and the attack chance is watched and not tuned.
- **No objective counts a boat sunk.** The one-day Eastern Solomons, which has no staff, has no boats.

## The rules of placing

1. **Who goes in.** A boat goes into a game where a ship row ties her to a derived class and a row puts her at a place that can be laid on the map on a day of the game.
2. **Where.** One of three readings, each written on the boat as "PLACEHOLDER start" with its rows:
   - **A stated latitude and longitude:** the hex the point falls in. Where the position is her target's (the ship she fired at), it is read as hers and the note says so.
   - **A bearing and a distance from a place** with a preferred row in `place_positions.csv`: the point at that distance on that bearing, laid off flat in the map's frame, as decision 0225 laid off Midway's arcs. A compass point is its bearing, as the rows' notes read it.
   - **At or off a place, with no distance** ("arrives off Port Moresby", "reconnoitres Lunga Roads", "the Savo Island area"): the hex of the place, which is a base of the game or a place with a preferred row.
3. **Which position.** Her first position in the game's days that rule 2 can lay off. Where the rows give her two patrols in the days, the one that is longer by the rows' dates, since a boat that goes home does not return; the other is named in the notes.
4. **When she enters.** On the day of that position. On day 1 where a row has her in the same waters on or before the game's first day.
5. **When she leaves.** `leave_day` is the day a row has her leave her area; where the rows give only her arrival in port, that day, and the note says she is kept on her station longer than she was. With neither she is given no leave day, and the note names the gap.
6. **Her home** is a stand-in, named as one, unless the rows have her sail from it: Rabaul for every Japanese boat (the rows have the boats of the first weeks sail from Rabaul and return to it; the others' is Truk, off the map); Port Moresby for the Allied boat at the Coral Sea, Lunga in the five days and Espiritu Santo in the 39 days (the S-boats' is Brisbane, off the map).
7. **A boat that cannot be laid off is left out and named in the game's notes** with what is missing. Two place rows are owed and would place more (below): Cape St George and San Cristobal. When they are filed the architect works the hexes in a dated section here; the build does not wait for them.
8. **The classes are those derived as of 7 August 1942,** read for May and June as decision 0225 read them. No boat placed here is of a class whose rows change between May and August.

Three effects of rule 2's third reading are accepted, named in the notes and counted by the measure:

- A boat in the hex of an enemy air base is looked for in every pulse at the full count of the base's search (0225, ruling 3). So RO-33 and RO-34 at Port Moresby, I-123 and RO-34 at Lunga, and S-45 at Shortland.
- A force in the hex of its own base is not attacked. So those boats attack nothing that lies at the base, and S-37 and S-42 off Savo, whose hex in the 39 days is the hex of the Japanese stock west of the Lunga, attack no run that ends there.
- A boat with one start stands there for the game unless ordered. I-19 and I-17 start where they were sighted on passage and do not go on to their line.

## The boats, game by game

The number is the boat's in the game. Days are the game's. Every start is a PLACEHOLDER start.

### The Coral Sea (6 to 10 May 1942; day 1 is 6 May)

| No. | Boat | Class | Hex | Enters | Leaves | Rows |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | USS S-42 | 6010 | (-3, -12) | 1 | | `sh` 17206; off Cape St George on 5 May (`ev` 16103); `pos` 16005, Okinoshima's position of 11 May, 5-06 S 153-48 E |
| 2 | I-21 | 6003 | (0, 18) | 1 | | `sh` 17257, 17258; `pos` 17202, 20 miles west of Noumea on 7 May, from place row 16; off Noumea on 5 and 8 May (17201, 17204) |
| 3 | RO-33 | 6007 | (-17, -4) | 1 | 5 | `sh` 17277; `pos` 17205, off Port Moresby from 5 May, left on 10 May |
| 4 | RO-34 | 6007 | (-17, -4) | 1 | | `sh` 17279; `pos` 17206 |

- **S-42's position is of the day after the game's last.** The rows have her off Cape St George the day before the game and at that position the day after it, and nothing on file has her leave between. She is read as in those waters through the days. This is the one use of a position outside a game's days, and it is the architect's judgement.
- **Left out:** I-28 and I-29 (6003) and I-22 and I-24 (class not derived) on the line south-west of Guadalcanal, which `pos` 17200 gives as a bearing with no distance; I-27, off the map; S-44 and S-47 (6010), off Cape St George and off New Hanover, places with no row; S-38, S-39, S-41, S-43 and S-45, not in the map's waters in the days.

### The Eastern Solomons over five days (23 to 27 August 1942; day 1 is 23 August)

| No. | Boat | Class | Hex | Enters | Leaves | Rows |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | USS S-42 | 6010 | (2, -5) | 4 | | `sh` 17206; `ev` 17242, on station between Savo and Cape Esperance from 26 August; place rows 10 and 32 |
| 2 | I-19 | 6003 | (7, -8) | 1 | | `sh` 17254; `pos` 17239, 07-30 S 162-15 E at 0725 on 23 August |
| 3 | I-17 | 6003 | (9, -9) | 1 | | `sh` 17252; `pos` 17240, 06-48 S 163-20 E at 0745 on 23 August |
| 4 | I-33 | 6003 | (8, -5) | 2 | | `sh` 17268; `pos` 17241, 09-21 S 163-35 E at 1105 on 24 August |
| 5 | RO-34 | 6007 | (2, -4) | 1 | | `sh` 17279; `pos` 17261, Lunga Roads at 1827 on 23 August |

- **Left out:** I-121 (6008) and I-31 (6003), whose positions are bearings from San Cristobal, a place with no row; I-26 and I-15 (6003), I-174 and I-175 (6005), whose areas in these days have no distance; I-9 and I-11, with no position in the days; I-123, with no dated position in the days; S-37, whose row gives her patrol's dates from Brisbane and not her days off Savo.

### Guadalcanal, the 39 days (7 August to 14 September 1942; day 1 is 7 August, 23 August is day 17, 1 September is day 26)

| No. | Boat | Class | Hex | Enters | Leaves | Rows |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | USS S-38 | 6009 | (-5, -12) | 1 | | off Cape St George on 7 August (`ev` 16119), eight miles south of it on 8 August (`ev` 16122); `pos` 16006, Meiyo Maru's position, 4-52 S 152-43 E |
| 2 | USS S-44 | 6010 | (-3, -17) | 4 | | `pos` 16008, Kako's position of 10 August, 02-28 S 152-11 E (`pos` 16004, 2-15 S 152-15 E, is in the same hex) |
| 3 | USS S-37 | 6009 | (2, -5) | 27 | 38 | `sh` 17200; `ev` 16150, north of Savo on 2 September; her patrol ended on 13 September (`ev` 17239); place row 10 |
| 4 | USS S-42 | 6010 | (2, -5) | 20 | | as in the five days |
| 5 | USS S-45 | 6010 | (-2, -8) | 25 | | `ev` 16165, on station in the Shortland Island area by the end of August, read as 31 August; the base's hex |
| 6 | I-122 | 6008 | (3, -5) | 7 | | `pos` 17226, 1.6 miles south-west of Tulagi at 1730 on 13 August |
| 7 | I-123 | 6008 | (2, -4) | 6 | | `pos` 17229, north of Lunga Point on 12 August |
| 8 | RO-34 | 6007 | (2, -4) | 17 | 31 | her second patrol: `pos` 17261; back at Rabaul on 6 September (`ev` 17202) |
| 9 | I-19 | 6003 | (7, -8) | 17 | | as in the five days |
| 10 | I-17 | 6003 | (9, -9) | 17 | | as in the five days |
| 11 | I-33 | 6003 | (8, -5) | 18 | | as in the five days |
| 12 | I-26 | 6003 | (8, -3) | 25 | | `sh` 17259; `pos` 16000, Saratoga's position when she fired, 10-34 S 164-18 E on 31 August; her area by `pos` 17247 |
| 13 | I-11 | 6014 | (4, -2) | 25 | 36 | `sh` 17275, 17276; `pos` 17248, 146 miles south-east of Tulagi on 31 August, from place row 4; at Truk on 11 September (`ev` 17211) |

- **RO-34's first patrol** (off Tulagi on 10 August, back at Rabaul on 16 August) is left out by rule 3.
- **I-122, S-38, S-44, S-42 and S-45 have no leave day:** no row gives one.
- **Left out:** I-121 and I-31, as in the five days; I-175, whose first patrol ended the day the rows first place her (12 August) and whose second has one position, three days before the end; I-15, I-174, I-9, I-21 and I-24, with no position; S-41, S-46, S-47, S-43, Amberjack and the boats off Truk, with none on the map; S-39, aground on her way.

### Midway (3 to 7 June 1942): ten more boats

Boats 1 to 5 stand as decision 0225 has them. The research gave every boat on the arcs a class row. By 0225's amended rule 2 (the arc read clockwise from its lower bearing, the boats at even steps in the row's order):

| No. | Boat | Class | Place in her row | Hex | Rows |
| --- | --- | --- | --- | --- | --- |
| 6 | USS Grayling | 6001 | third of six, 288 degrees, 150 miles | (2, -1) | `sh` 17226; `pos` 16010 |
| 7 | USS Grouper | 6002 | fifth of six, 336 degrees | (6, -4) | `sh` 17234; `pos` 16010 |
| 8 | USS Gudgeon | 6001 | sixth of six, 360 degrees | (7, -4) | `sh` 17230; `pos` 16010 |
| 9 | USS Grenadier | 6001 | third of three, 310 degrees, 200 miles | (3, -3) | `sh` 17228; `pos` 16009 |
| 10 | USS Flying Fish | 6002 | first of two, 310 degrees, 50 miles | (5, -1) | `sh` 17240; `pos` 16011 |
| 11 | USS Narwhal | 6011 | first of three on the line | (18, -3) | `sh` 17246; `pos` 16013 |
| 12 | USS Trigger | 6002 | third of three on the line | (16, -2) | `sh` 17242; `pos` 16013 |
| 13 | USS Dolphin | 6012 | first of three, 250 degrees, 200 miles | (0, 2) | `sh` 17212; `pos` 16009 |
| 14 | USS Cachalot | 6013 | second of two, 350 degrees, 50 miles | (6, -1) | `sh` 17214; `pos` 16011 |
| 15 | USS Cuttlefish | 6013 | 27 N 170 E | (-13, 3) | `sh` 17216; `pos` 16021 (30 May), 17213 (0020 on 4 June) |

- **The support patrol** (`pos` 16013) is a line of bearing 045 to 225 degrees through the point 075 degrees, 425 miles from Midway, the boats 30 miles apart. The row does not say which boat is at which end. The reading: the boats in the row's order from the 045 end, so Narwhal 30 miles from the centre on 045, Plunger at the centre and Trigger 30 miles from it on 225. Plunger's class is not derived, and her place is left empty.
- **All ten start on day 1,** on stations ordered on 2 June. Their home is Midway, a stand-in.
- **Grouper is in the game.** Decision 0225 left her out for want of a ship row. Her attack of the rows, on a destroyer alone, still cannot happen.
- **Left out:** Plunger; Pike, Tarpon, Growler and Finback, off the map; the Japanese cordons, off the map.

## Each navy's pattern

Every game with boats states both sides' doctrine, by decision 0223's stage 0, Midway's copies among them:

```
[side.allied]
submarine_pattern = "off_bases"
expects_boats_off_own_bases = true

[side.japanese]
submarine_pattern = "lines"
expects_boats_off_own_bases = false
```

- **Allied, `off_bases`:** American boats of 1942 worked off enemy bases, straits and focal points and not on lines: Rainbow Five's seven stations off Japan (16701), COMINCH's "focal or other favorable points" of 2 March (16486), CINCPAC's focal points of August (16583), and the stations off Rabaul, Kavieng, the Shortlands and Truk in the rows of each battle here.
- **Japanese, `lines`:** boats were put on the expected track of an American force in one or two lines across it (16773, 16770), two lines 200 miles apart with boats 30 miles apart in the Solomons (16348, 16772), and the picket lines of August and September in the rows above.
- **The expectation by habit** is the American staff's (Rainbow Five, 16710 and 16711). The record of 1942 gives the Japanese none. No rule reads it until decision 0223's stage D1.
- **It moves no boat.** The pattern is read only when a commander orders a patrol. A game played with no patrol order plays as it would without the two tables, but for its hash and its briefing.
- "Allied" is the American pattern for every Allied boat; every Allied boat here is American.

## The classes

Three are derived now, each because a boat placed above is of it. The ids are new and pinned in `[submarine_class_ids]`:

| Id | Class | Salvo | Salvoes | Passage speed | Torpedo |
| --- | --- | --- | --- | --- | --- |
| 6012 | `us-dolphin-class` | 4 (17266) | 18 / 4 = 4 (17268) | 10 (17265) | Mark 14, taken from the fleet type's row 16184, as Narwhal's is |
| 6013 | `us-cachalot-class` | 4 (17274) | 16 / 4 = 4 (17276) | 10 (17273) | the same |
| 6014 | `jp-i-9-group` | 6 (16188) | 20 / 6 = 3 (16214) | 16 (17204) | taken by rule 29, below |

- **Dolphin's and Cachalot's rows of tubes, torpedoes and range are Wikipedia's,** each a lone weak row with no row against it, read by an exception and marked, as the Sargo and Tambor classes' are. FM 30-50's six tubes in all (17262, 17270) agree with four and two. Their top speed, 17 knots, is FM 30-50's (17261, 17269).
- **The I-9 group's torpedo.** Its two rows (16215, Type 89; 16216, Type 95 "also reported") are ONI's of 1945 and are not read for 1942 (0225, the ruling on the derivation, 2). No row names a torpedo fired by a boat of the group in 1942. By rule 29 it takes the torpedo stated for the boats of the group nearest in date by the year built. The builder reads the year of the Technical Mission's block c from its heading. The groups whose boats have a stated torpedo are the I-15 block (1940, Type 95: `ev` 3002, 16433, 16434) and the I-121 block (1927, Type 89: `ev` 16429). I expect a year after 1933 and so the Type 95. If the heading gives no year, the builder stops and asks.
- **The estimates by ratio move.** Three more classes have both a top speed and a speed of endurance, so the S-boats' and Narwhal's estimated passage speeds are worked over eleven like classes and not eight: about 8.75 and 10.50 knots against 8.686399 and 10.423678. That is decision 0207's rule working as written. The tool's figures stand; these two are not expected numbers. If any test holds the old figures, the builder stops and asks.

Six wait, each still `[[left_out]]`, with these reasons in place of "waits on the architect's ruling":

| Class | Why it waits |
| --- | --- |
| `jp-i-16-group` | Its rows are whole and the Technical Mission's, and it could be derived (8 bow tubes, 20 torpedoes, 16 knots). No boat of hers can be placed: I-22 and I-24 are on lines with no position. Derived when one can. |
| `jp-i-7-group` | No boat of hers is in a game's days (I-7 sailed on 17 September). Its torpedoes carried have no row as a total. |
| `jp-i-1-j1-group`, `jp-i-6-j2-group` | Every row is low and unconfirmed, and the Technical Mission's row for the block (17241) agrees with one type and not the other. Wants a primary source. No boat of theirs is in a game's days. |
| `us-porpoise-class`, `us-perch-class` | No boat of theirs can be placed (Pike and Tarpon are off the map; Plunger's place is known and her class is in dispute, ruled below for when it is derived). Their torpedo and most figures are Wikipedia's alone. |

`jp-i-5-group`, `jp-i-1-group`, `jp-ro-35-group` and `us-fleet-type-submarine` stay left out as they are.

## Rulings on the rows in dispute

No row is changed. Where a ruling lets the derivation read a row, the builder writes it as an exception in `data/derivation/rules.toml` with the reason given here, "Settled by the architect, Nov 23, 2026 (decision 0227)".

1. **The I-9 group's top speed: row 17201 (24 knots, the Technical Mission) is read, over 17206 (17 knots, ONI 222-J).** It is read only as a like class's row for the estimate by ratio. The Technical Mission's table is the survey made from Japanese records after the war and is what every Japanese block on the list is read from. ONI's 17.0 is the same figure as its I-5 sheet beside it and stands one knot above its own sustained speed of 16; the research filed it as disputed and not explained, and so it stays.
2. **The I-9 group's speed of endurance: 17204 (16 knots).** ONI's 17208 agrees.
3. **The I-9 group's torpedoes carried: 16214 (20) stays read,** as ruled on Nov 22. The Technical Mission's six tubes and 12 reloads (16188, 17210) make 18, and both give three salvoes.
4. **The submerged speeds in dispute (17202 against 17209)** are read by no derivation and are left as they stand.
5. **The I-15 class's torpedo stays the Type 95, taken from her boats' rows of 1942** (`ev` 3002, 16433, 16434). Rows 17236 and 17237 are ONI's of 1945, name both types and are not read for 1942. The list's note that the row was research owed is taken out: it is filed, and it does not settle 1942. Her reloads (17238: six tubes and 11 make 17) stand beside the 16 read (16051); both give two salvoes.
6. **The I-168 group's torpedoes carried: 16222 (14) stays read.** Row 17239 gives tubes and reloads that make 12, is dated 1945, and is not a total; whether it held in 1942 is not stated. Both give three salvoes, so no game turns on it. The gap stays open.
7. **The I-121 group's torpedoes carried: 16033 (8) is confirmed over 16060 (20).** Four tubes and four reloads (16032, 17243) make eight. The exception's "pending research" is taken out.
8. **The RO-33 group's 10 (16226) stands,** agreeing with four tubes and six reloads (17240).
9. **Pickerel and Plunger: rows 17244 and 17245 (`us-perch-class`, FM 30-50) are read, over 17222 and 17224 (`us-porpoise-class`, DANFS),** when the class is derived. FM 30-50 is the Navy's own list of October 1941 and parts the Pike class from the Perch class, with different speeds on file; DANFS's "Porpoise" names the wider group and does not deny the finer one; and the class rows filed under `us-perch-class` are from Plunger's own page.
10. **I-8 is of `jp-i-7-group` (17283) and I-5 of `jp-i-1-group` (17286), by the Technical Mission's blocks, over ONI's I-5 class (17285, 17288).** The list is keyed to the Technical Mission's blocks, and ONI's sheet of 1945 does not list I-7. So `jp-i-5-group` has no boat by the rows read and stays left out; whether it is a class at all stays open.
11. **I-4 is of `jp-i-1-j1-group` (17290) when that class is derived.** Row 17289 says the same of the block before it was split, and is not against it.
12. **I-21 is of class 6003 by rows 17257 and 17258** (Nishida and ONI). The Technical Mission's block does not list her, as the research found.
13. **Cachalot's range (17272, 6,000 miles, against 17277, 11,000)** is read by no derivation. Both are at 10 knots (17273), which is read. Left in dispute.
14. **S-42's arrival off Savo: `ev` 17242 (26 August) is read, over `ev` 16152 (28 August).** Both file the same sentence of DANFS; 17242's note gives the sailing of 21 August and the five days, and 16152's date has no stated basis.
15. **Kako's position (`pos` 62 and 16008 against 16004)** needs no ruling here: both are in hex (-3, -17).
16. **Nautilus's position at 0710 on 4 June (`pos` 5775 against 16002)** is read by nothing, as decision 0225 left it.
17. **`jp-i-1-group` stays in the pool of like classes.** Its two rows (16001, 16006) are the Technical Mission's for the block, and nothing filed since is against the pair: the new rows of the two types are low and unconfirmed. Its `like_class_reason` is rewritten to say so.

Two names in `ship_classes.csv` misstate their source, as the research found: 16010 lists I-21 in the Technical Mission's block, and 16008 says ONI's sheet covers I-5 to I-8 where it lists I-5, I-6 and I-8. A class's name is the file's own description and not a source's statement. Research mends the two where `data/oob/README.md` allows a description to be mended, and the generated class file's name follows.

## Research owed, which the build does not wait for

- **A place row for Cape St George and one for San Cristobal** (the island, with the feature the row measures to named). With them: I-121 (50 miles south-east of San Cristobal on 22 August, there from 18 August, at Rabaul on 4 September) and I-31 (175 miles south-east on 25 August, at Truk on 30 August) in the five days and the 39; S-44 off Cape St George at the Coral Sea.
- **A distance for the line south-west of Guadalcanal of May** and positions for the lines of August and September, which the research already lists as gaps.
- **The day each S-boat and I-122 left her area.**

## What a builder does

One branch and one pull request for each of the three stages.

**Stage 1, the classes.**

1. In `data/derivation/submarines_1942.toml`: make `[[submarine]]` entries for `us-dolphin-class`, `us-cachalot-class` and `jp-i-9-group`, each with its `torpedo_from` and the reason above; pin 6012, 6013 and 6014; rewrite the reasons of the six classes that wait and the `like_class_reason`; take out the I-15 entry's note of research owed and say why the rows of 1945 are not read. The header's sentence that the new classes wait on a ruling goes.
2. In `data/derivation/rules.toml`: an exception for each row a ruling reads that the preference rules would not, with its reason and `agreed = "2026-11-23"`; mend the reason of the I-121 group's exception (ruling 7).
3. Run `oob-derive` on the list, then with `--check`; run `oob-check` and `cargo test --workspace`. Report the three classes' rows as generated, the like classes and the mean, and the S-boats' and Narwhal's new speeds.

**Stage 2, the games.**

4. Extend `scripts/table-boats/make_files.py` to make, from the table's copies and each game's file a: Midway's four with 15 boats; the Coral Sea's four (from `coral-sea/03a` and `table-hunger/03b` to `03d`); the five days' four (from `five-days/02a` and `table-hunger/02b` to `02d`); and the 39 days' four (from `stage-3-proof/07a_slice.toml`, and `table-hunger/07b_slice.toml`, `07c_slice.toml` and `07_full_slice.toml`). Each copy gains what Midway's gained and the two `[side]` tables. File names end `_submarines.toml`. If a source already has a `[side]` table, stop and ask.
5. Each boat's comment gives her ship row, her class's row, the rows of her start and what was read from them, her distance from the hex's centre where a point was laid off, and her stand-in home. Each header names: the boats left out and why; the three accepted effects; the rows of belief left out; the estimated figures of every class in the game, citing decision 0207; and that the figures are played unset.
6. Tests: restate `tests/boats_in_the_midway_games.rs` to the statements below, and add one test file for each of the other three games with the same statements. Scenario 104's statement 23 gains every copy that carries `for_play`. Nothing else in any test changes.
7. The table: the pairings of the Coral Sea, the five days and the 39 days name the copies in `table-boats/`; each game's lines and each side's directive gain the sentence below; Midway's sentences are replaced. `table/check_screen.py` checks each game as it checks Midway. `table/README.md` and `docs/commands.md` say what changed.

**Stage 3, the measure.**

8. For each of the four games, file a's copy on seeds 1 to 100 against the same file with its boats, their three keys and its `[side]` tables taken out, by 0225's amended rule 8 and with what its measure reported. Added: for each boat in the hex of an enemy base, the pulses she was looked for, sighted and attacked; for S-37 and S-42, the force-hours of Japanese forces in their hex. Reported in this record under "As built". Nothing is changed for the result.

## The statements each game's test holds

Each copy, loaded:

1. It has the submarines of its table above, by name, side and number, and no other.
2. Each starts in her hex, on station there, in a hex ships can enter; each enters on her day and has her leave day, or none.
3. Each is of her class, with her class's torpedo and salvoes as the class file loaded gives them.
4. Each one's home is the base rule 6 names.
5. It has no suspected waters.
6. The Allied side's pattern is off bases and it expects boats off its own bases; the Japanese side's is lines and it does not.
7. Taken with its submarines, their three keys and its two `[side]` tables removed, it is the game it copies, line for line but for its header, and for file a its ground figures.
8. Played on seed 1 with no order to any boat, every boat on the map at the end that is not going home is in the hex she started in.

And the script, run with `--check`, says every file kept is the one it makes and the folder holds nothing else. No statement holds a number that rests on a figure.

## The sentences

The situation of each side's directive ends with these; the game's lines carry a shorter one naming both sides' boats.

- **Coral Sea, Allied:** "One of our submarines, S-42, is on patrol in St George's Channel, south-east of Cape St George, and is under your command."
- **Coral Sea, Japanese:** "Three of our submarines are at sea and under your command: I-21 off Noumea, and RO-33 and RO-34 off Port Moresby. RO-33 leaves her area on 10 May."
- **Five days, Allied:** "One of our submarines, S-42, is due on station off Savo Island on 26 August, under your command."
- **Five days, Japanese:** "Four of our submarines are under your command: RO-34 off Lunga, I-19 and I-17 north-east of Malaita on their way south, and I-33, who arrives east of Malaita on 24 August."
- **39 days, Allied:** "One of our submarines, S-38, is on patrol off Cape St George, under your command. Four more are due: S-44 north-east of New Ireland on 10 August, S-42 off Savo Island on 26 August, S-45 off Shortland on 31 August and S-37 off Savo Island on 2 September."
- **39 days, Japanese:** "No submarine of ours is on station yet. They are due, under your command: I-123 off Lunga on 12 August and I-122 off Tulagi on 13 August; RO-34 off Lunga and I-19 and I-17 north-east of Malaita on 23 August; I-33 east of Malaita on 24 August; and I-26 east of San Cristobal and I-11 off its south-eastern end on 31 August. RO-34 leaves on 6 September and I-11 on 11 September."
- **Midway, Allied:** "Fourteen of our submarines are on station and under your command: Cuttlefish 700 miles west of Midway; Dolphin, Gato and Grenadier on the arc 200 miles to the west and north-west; Tambor, Trout, Grayling, Nautilus, Grouper and Gudgeon on the arc 150 miles out, from the south-west round to north; Flying Fish and Cachalot 50 miles to the north-west and north; and Narwhal and Trigger 425 miles to the east-north-east."
- **Midway, Japanese:** as it stands.

## The hexes, worked by hand

**The South Pacific map.** east = (lon - 160) x 60 x 0.978148; north = (lat + 12) x 60. A hex (q, r) is centred at east 40 (q + r/2), south 34.641 r, and a point is in the hex whose centre is nearest.

| Boat | From | Point (east, north) | Nearest centre | Next nearest |
| --- | --- | --- | --- | --- |
| I-21 | Noumea, 22.25675 S 166.42019 E, is (376.79, -615.40); 20 miles on 270 | 356.79, -615.40 | (0, 18) at 360, -623.54: 8.7 nm | (0, 17): 31.4 |
| S-42, May | 5-06 S 153-48 E | -363.87, 414.00 | (-3, -12) at -360, 415.69: 4.2 nm | (-4, -12): 36.2 |
| S-38 | 4-52 S 152-43 E | -427.45, 428.00 | (-5, -12) at -440, 415.69: 17.6 nm | (-4, -13): 23.5 |
| S-44 | 02-28 S 152-11 E | -458.75, 572.00 | (-3, -17) at -460, 588.90: 16.9 nm | (-3, -16): 25.8 |
| S-37, S-42 | Savo, 9.1303 S 159.81619 E; Cape Esperance, 9.25 S 159.71667 E | -10.79, 172.18; -16.63, 165.00 | (2, -5) at -20, 173.21: 9.3 and 8.9 nm | (3, -5): 30.8 |
| I-19 | 07-30 S 162-15 E | 132.05, 270.00 | (7, -8) at 120, 277.13: 14.0 nm | (7, -7): 28.6 |
| I-17 | 06-48 S 163-20 E | 195.63, 312.00 | (9, -9) at 180, 311.77: 15.6 nm | (10, -9): 24.4 |
| I-33 | 09-21 S 163-35 E | 210.30, 159.00 | (8, -5) at 220, 173.21: 17.2 nm | (7, -4): 22.9 |
| I-26 | 10-34 S 164-18 E | 252.36, 86.00 | (8, -3) at 260, 103.92: 19.5 nm | (7, -2): 20.8 |
| I-11 | Tulagi Harbour, 9.09975 S 160.15356 E, is (9.01, 174.01); 146 miles on 135 is (103.24, -103.24) | 112.25, 70.78 | (4, -2) at 120, 69.28: 7.9 nm | (3, -2): 32.3 |

Port Moresby (-17, -4), Lunga (2, -4), Tulagi (3, -5) and Shortland (-2, -8) are the hexes of the games' bases. I-26's point is 1.3 miles nearer her hex than the next; the rule gives (8, -3).

**The Midway map,** as decision 0225 has it: the atoll at east 217.77, north -16.08.

| Boat | d sin b, d cos b | Point (east, north) | Nearest centre | Next nearest |
| --- | --- | --- | --- | --- |
| Grayling, 150 at 288 | -142.66, 46.35 | 75.11, 30.27 | (2, -1) at 60, 34.64: 15.7 nm | (3, -1): 25.3 |
| Grouper, 150 at 336 | -61.01, 137.03 | 156.76, 120.95 | (6, -4) at 160, 138.56: 17.9 nm | (5, -3): 23.9 |
| Gudgeon, 150 at 360 | 0, 150 | 217.77, 133.92 | (7, -4) at 200, 138.56: 18.4 nm | (8, -4): 22.7 |
| Dolphin, 200 at 250 | -187.94, -68.40 | 29.83, -84.48 | (0, 2) at 40, -69.28: 18.3 nm | (-1, 3): 21.8 |
| Grenadier, 200 at 310 | -153.21, 128.56 | 64.56, 112.48 | (3, -3) at 60, 103.92: 9.7 nm | (4, -4): 30.3 |
| Flying Fish, 50 at 310 | -38.30, 32.14 | 179.47, 16.06 | (5, -1) at 180, 34.64: 18.6 nm | (4, 0): 25.2 |
| Cachalot, 50 at 350 | -8.68, 49.24 | 209.09, 33.16 | (6, -1) at 220, 34.64: 11.0 nm | (5, -1): 29.1 |
| The line's centre, 425 at 075 | 410.52, 110.00 | 628.29, 93.92 | (17, -3): Plunger's, left empty | |
| Narwhal, 30 on 045 from it | 21.21, 21.21 | 649.50, 115.13 | (18, -3) at 660, 103.92: 15.4 nm | (18, -4): 25.3 |
| Trigger, 30 on 225 from it | -21.21, -21.21 | 607.08, 72.70 | (16, -2) at 600, 69.28: 7.9 nm | (17, -2): 33.1 |
| Cuttlefish, 27 N 170 E | | -448.20, -90.00 | (-13, 3) at -460, -103.92: 18.3 nm | (-12, 2): 22.3 |

Cuttlefish's two other rows (17212, 263 degrees and 690 miles at 2400 on 3 June; 17213, 27 N 170-02 E) fall in the same hex.

## What it does not do

- It adds no rule and no state. The rulebook does not change.
- It scripts no order, so no boat is relieved and none returns: a boat has one patrol.
- It does not give the staff a patrol of its own; that is decision 0223's, set aside there.
- It does not set the submarine figures from the rows. The measures of the four games are what that work will read.

## Before it is built

Reviewed by a fresh reader given this record, decision 0225, the research, the rows and the map files, who works the hexes again with the maps' own projections and checks each boat's rows and days. The architect amends; then stage 1.

## Amended on review, Nov 23, 2026

A fresh reader was given this record and the repository. The architect checked each finding against the files and accepts all ten. Where this section and the text above differ, this section holds.

1. **The I-9 group's torpedo is read from row 16221, by an exception, and the class has no `torpedo_from`.** Row 16221 (Type 95, Wikipedia's Type A page, undated, unconfirmed) was on file and the plan missed it; "no row names a torpedo" was untrue. An undated row holds on the list's date, so it drives out any `torpedo_from` and the tool would fail (`crates/oob-derive/src/submarine.rs`, the check of `torpedo_from`). The exception's reason: "Type 95, Wikipedia's Type A page, read as the A1 type from the order of the table's columns; a lone weak row among the rows that hold on the date (16215 and 16216 are ONI's of 1945, and 16216 agrees with it); read and marked; it gives way to a better row when one is filed", as row 16225 is read for the I-168 group. Class 6014's torpedo is read, not taken, and is not marked estimated.
2. **`crates/sim-data/src/reference.rs`, near line 4940: `data.classes.len()` is 15,** not 12. The architect changes that expected number. The torpedoes stay 5.
3. **Rule 29 is not used for the I-9 group,** by ruling 1 here, and what the plan said of it is withdrawn: the block's year is on file (`sh` 17273 and 17275, "I-9 to I-12. Built 1941"), the "after 1933" test was wrong, and the list of groups with a torpedo stated for their boats left out the I-7 block (`ev` 16430) and the I-1 block (`ev` 16050, 16432). Had the rule been used, the group nearest 1941 is the I-15 block of 1940 and the answer is the same Type 95. The builder does not read a heading and has nothing to stop for there.
4. **The first two accepted effects are restated: they hold by who holds the base and whether it flies a search, and that is the game's state, not the start's.**
   - A boat in the hex of a base the enemy holds is looked for at the full count of that base's search where the base flies one. At the start that is so of RO-33 and RO-34 at Port Moresby (Coral Sea) and of RO-34 at Lunga in the five days. In the 39 days Lunga and Tulagi start Japanese and with no airfield, and Shortland has none: I-123 and RO-34 at Lunga and I-122 at Tulagi lie at a base of their own side until it is taken, and S-45 at Shortland is looked for only by what search the base has.
   - A force in the hex of a base its own side holds is not attacked. So I-122, I-123 and RO-34 in the 39 days may attack an Allied force in their hex while the base is Japanese and not after it is Allied; RO-34 in the five days and the RO boats at Port Moresby may not from the start; S-45 attacks nothing lying at Shortland; S-37 and S-42 as stated.
   - The headers say this in these words, and the measure reports who held each such base on each day.
5. **I-171 at Midway is left out and named.** Row `pos` 16015 puts her at point M, 19 N 174-26 W, on 31 May, which is hex (1, 16) of the Midway map (20.2 nm from its centre, 20.3 from that of (1, 17)); her ship row is 16002 and her class 6005. The position is of three days before the game, for an operation that was cancelled, and the rows have her on the KO line at 166-20 W from 1 June (`ev` 16194), off the map. No row has her on the map in the game's days, so rule 1 leaves her out. Decision 0225's "I-168 alone lies on the map" is true of the game's days and not of the rows; the header of Midway's copies names I-171 so.
6. **Cuttlefish's start is read from row 17212** (263 degrees, 690 miles from Midway at 2400 on 3 June), her first position in the game's days: the point is east -467.09, north -100.17, 8.0 nm from the centre of (-13, 3). Rows 16021 (30 May) and 17213 (0020 on 4 June) fall in the same hex and are named beside it.
7. **S-42 at the Coral Sea is not in St George's Channel.** Her hex is about 65 miles east-south-east of the hex where the rows put "eight miles south of Cape St George" (`pos` 16006 with `ev` 16122). She is placed as ruled, at her one stated position, and the judgement that she was in those waters through the days stands as a judgement. The sentence becomes: "One of our submarines, S-42, is on patrol south-east of Cape St George, off the south coast of New Ireland, and is under your command."
8. **Leave days in the 39 days, said in full.** I-19, I-17, I-33 and I-26 have none because the rows have them leave on 20 September, after the game. I-123 has none: the rows have her sunk on 29 August, day 23 (`pos` 16001, `ev` 16054), which is an outcome and is not scripted; the header says she is kept as long as she lives. I-122, S-38, S-44, S-42 and S-45 have none because no row gives one, a gap. In the five days no boat has one.
9. **What was left to the builder is settled:**
   - **The rows of belief each header names as left out.** Coral Sea: Japanese 16257; Allied 16513, 16534, 16750 and 16849. Five days: Allied 16298, 16584, 16608, 16589 and 16590; Japanese none on file, a gap. The 39 days: those five and 16303, 16610 and 16613; Japanese none. Midway: as its header has them, and 17246 to 17248, of other waters.
   - **The two `[side]` tables** open the section the script puts at the end of each copy, before the first `[[submarine]]`.
   - **The copy of `07_full_slice.toml`** is `07_full_slice_submarines.toml`; the others are `07a_slice_submarines.toml`, `07b_` and `07c_`.
   - **Exceptions for the Technical Mission's rows of the I-9 group** dated only to 1942: 16188 (6 bow tubes), 17204 (16 knots) and 17201 (24 knots, as a like class's row), each with the words the other blocks' exceptions use ("gives the block as built (built 1941) and dates nothing within 1942; judged to hold on 7 August 1942").
   - **`rules.toml`, the exception for row 16214:** "The class stays out until its speed is found" becomes "Its speed is on file (rows 17204 and 17208) and the class is derived (decision 0227)".
   - **The count at the end of `submarines_1942.toml`:** I-11 at Hornet moves from "not counted" to "Type 95, class row" (16221, by exception): 4 fired, 0 hits. The Type 95's totals become 16 fired, 3 to 4 hits. It sets no figure, as before.
10. **Statement 8 and the time of the tests.** For Midway, the Coral Sea and the five days it runs on each of the four copies, as Midway's statement 7 does today. For the 39 days it runs once, on the copy of file a, in a test marked `#[ignore]` with the reason that it plays 39 days; the builder runs it in release and reports its time, and the measure of stage 3 reports the same statement over its 100 seeds. Statements 1 to 7 of the 39 days load the copies and play nothing, and run in the suite.

## As built, Nov 23, 2026

Built on branch `boats-placed` by a developer for the architect, to the plan and its amendments, the statements and the hexes as written. Stage 1 is one commit and stage 2 another; stage 3 is this section. Nothing in `crates/` was changed but the expected class count in `crates/sim-data/src/reference.rs`.

### Stage 1, the classes

- `data/derivation/submarines_1942.toml`: `[[submarine]]` entries for `us-dolphin-class` and `us-cachalot-class` (each with `torpedo_from` the fleet type's row 16184, as Narwhal's) and `jp-i-9-group` (no `torpedo_from`; its torpedo is read by exception); ids 6012, 6013 and 6014 pinned; the six classes that wait have the reasons of this record in place of "waits on the architect's ruling"; `jp-i-1-group`'s `like_class_reason` says what ruling 17 says; the I-15 entry's note of research owed is replaced by why rows 17236 and 17237 are not read; the header's sentence that the new classes wait on a ruling is replaced; the three estimates say eleven like classes; the count at the end moves I-11 at Hornet to "Type 95, class row" (4 fired, 0 hits) and the Type 95's totals to 16 fired, 3 to 4 hits.
- `data/derivation/rules.toml`: exceptions for the I-9 group's rows 16188 (6 bow tubes), 17204 (16 knots), 17201 (24 knots, as a like class's row, over ONI's 17), 16221 (the Type 95, as row 16225 is read for the I-168 group) and, for the Dolphin and Cachalot classes, rows 17266, 17265 and 17268 and 17274, 17273 and 17276 (each a lone weak row of Wikipedia's, marked); the reason of row 16214's exception ends "Its speed is on file (rows 17204 and 17208) and the class is derived (decision 0227)"; the reason of row 16033's no longer says "pending research" and gives the four tubes and four reloads that make eight (ruling 7). Each new exception has `agreed = "2026-11-23"`.
- `data/reference/submarine_classes_1942.csv` and its sources companion regenerated by `oob-derive`; the torpedo file is unchanged. `crates/sim-data/src/reference.rs`: the expected class count is 15, as the architect ruled; the torpedoes stay 5.
- The three classes as generated (salvo, salvoes carried, passage speed in knots, torpedo): 6012, 4, 4, 10, 6100 (estimated, the Mark 14); 6013, 4, 4, 10, 6100 (estimated); 6014, 6, 3, 16, 6103 (read; not estimated). The like classes are eleven (jp-i-1-group, jp-i-15-group, jp-i-168-group, jp-i-176-group, jp-i-9-group, jp-ro-33-group, us-cachalot-class, us-dolphin-class, us-gato-class, us-sargo-class, us-tambor-class), the mean of their ratios 0.603239. The S-boats' passage speed is 8.746965 and Narwhal's 10.496358 (against 8.686399 and 10.423678), as the record says. No test held the old figures.
- `oob-derive --check` passes for `savo_island`, `rabaul_guadalcanal`, `coral_sea`, `midway` and `submarines_1942`; `oob-check` and the tests of `sim-data`, `sim-core`, `oob-derive` and `oob-check` pass.

### Stage 2, the games

- `scripts/table-boats/make_files.py` makes sixteen files, four for each of four games, from the sources the record names. The names are the source's name with `_submarines` before `.toml` (`03a_coral_sea_both_oilers_submarines.toml`, `02a_eastern_solomons_oilers_submarines.toml`, `07a_slice_submarines.toml`, `07b_`, `07c_` and `07_full_slice_submarines.toml`); the record gave only the 39 days' names, and this rule gives them. Each header names the boats left out and why, the accepted effects as restated in the amended section (by who holds each base and whether it flies a search), the rows of belief left out, the estimated figures of every class in the game, and that the figures are played unset; each section opens with the two `[side]` tables, then holds the boats, each with her ship row, her class's row, the rows of her start and what was read from them, her distance from the hex's centre where a point was laid off, and her stand-in home. A source that already had a `[side]` table would stop the script; none had.
- `data/scenarios/table-boats/`: the sixteen files. Midway's four now have 15 boats; the Coral Sea's 4, the five days' 5, the 39 days' 13, all as the tables of the record give them.
- Tests: `tests/boats_in_the_midway_games.rs` restated, and `tests/boats_in_the_coral_sea_games.rs`, `boats_in_the_five_days_games.rs` and `boats_in_the_thirty_nine_days_games.rs` added, each with the statements as the record writes them in its doc comment and one test for each, on the shared `tests/common/boats_in_a_game.rs`. All hold on the first run: statements 1 to 7 on each game's four copies, statement 8 on all four copies of Midway, the Coral Sea and the five days, and the script's `--check`. The 39 days' statement 8 is `#[ignore]` and was run once in release (`cargo test --release -p scenario-tests --test boats_in_the_thirty_nine_days_games -- --ignored`) on the copy of file a: it passed in 1.44 s (9.6 s with the build). `tests/scenario_104_men_put_ashore.rs`: statement 23's list gains the twelve copies that carry `for_play` (Midway, the Coral Sea, the five days; the 39 days' sources carry none).
- The table: `table/scenarios.json` names the copies in `table-boats/` for the Coral Sea, the five days and the 39 days, and the lines of each game name both sides' boats; the six directives of those games end the situation with the sentence of the record, and Midway's Allied one is replaced; `table/check_screen.py` checks each game's pairings, each side's last sentence and each seat's boats on the first day (those that enter later are listed as held apart); `table/README.md` and `docs/commands.md` say what changed. `python3 table/check_screen.py --port 8850 --sim-cli target/release/sim-cli`: all held (494 checks).

### Where the build differs from the plan

- **`tests/scenario_121_a_japanese_oiler_by_estimate.rs`, `property_the_table_s_copies`, is changed**, though stage 2's step 6 says nothing else in any test changes. It asserted that `table/scenarios.json` names `table-hunger/03{b,c,d}_coral_sea_both_oilers.toml`, which step 7 makes untrue; it now asserts that the table-hunger copies still exist and that the table names the copies with submarines made from them. No number or hand calculation moved. The architect to confirm.
- **Statement 8 reads `on_map` and `passage.home`:** a boat is "on the map and not going home" when she is neither sunk nor home, has entered, and has no passage that is a way home.
- **The statements 1 to 7 of the 39 days hold on all four copies**, loaded; only statement 8 is run on file a alone.
- **The measure's force-hours** of Japanese forces in the hex of S-37, S-42 and of every other boat are worked from the forces' places after each pulse (a force that stood in the hex through a pulse counts the pulse's hours, 12 at night and 6 otherwise; one that came or left in it, half), since the engine records hours only where a chance is drawn.
- The measure's script and its output (`runs.jsonl`) are kept with the build's notes, not in the repository.

### The measure (stage 3)

File a's copy of each game on seeds 1 to 100, staff against staff, against the same file with its section of boats, its three keys and its two `[side]` tables taken out (paths made whole, outside the repository), by 0225's amended rule 8, read as it read Midway: from `sim-cli export --seed N` and `sim-cli run --seed N` in release. A ship sunk is a `ship_sunk` event whose causes are walked to the first `air_attack` (air) or `submarine_salvo` (submarine), else other; a ship damaged and afloat is counted once for each cause that hit her; a pulse under a chance is a `submarine_attack_chance` event, its hours the event's; looked for, sighted and attacked are `submarine_search_attempt` events (sighted where the text says so) and `submarine_attacked_from_air` events; the closest enemy force is the least hex distance from her hex to any enemy task force after any step; who held each base is its side at the start and each `base_changed_side`. Midway's tables reproduce decision 0225's for its five boats, figure for figure.

Traceback (most recent call last):
  File "/private/tmp/claude-501/-Users-amitpeer--claude-spares-spare-WRvGWC/e3176bbe-6d30-4d1a-b4ac-0332356b458e/scratchpad/aggregate.py", line 63, in <module>
    print(table(g,BASES[g]))
  File "/private/tmp/claude-501/-Users-amitpeer--claude-spares-spare-WRvGWC/e3176bbe-6d30-4d1a-b4ac-0332356b458e/scratchpad/aggregate.py", line 12, in table
    runs=load(game)
  File "/private/tmp/claude-501/-Users-amitpeer--claude-spares-spare-WRvGWC/e3176bbe-6d30-4d1a-b4ac-0332356b458e/scratchpad/aggregate.py", line 8, in load
    for l in open(f'{OUT}/{game}.jsonl'):
FileNotFoundError: [Errno 2] No such file or directory: 'measure/midway.jsonl'

What the tables say, and nothing is changed for it:

- **Midway.** The ten new boats change no result: all 100 runs are the same with the boats and without. No boat is ever under a chance to attack. Grayling, Grouper, Gudgeon, Grenadier, Flying Fish and Cachalot are looked for by Midway's flying boats and attacked from the air (Flying Fish ends sunk in 4 runs, Cachalot in 3 and Grouper in 1, and home in 23, 12 and 11, Gudgeon home in 8, Grenadier in 3 and Grayling in 1), as Nautilus and I-168 were; Narwhal, Trigger, Dolphin and Cuttlefish are not looked for and no force comes near them.
- **The Coral Sea.** All 100 runs are the same with the boats and without. RO-33 and RO-34 lie in Port Moresby's hex, looked for at the base's full search (626 and 560 looks, 60 and 22 sighted and attacked from the air; RO-33 sunk in 6 runs and RO-34 in 1) and are under no chance; Port Moresby was Allied on every day in every run. S-42 and I-21 are never near a force (the closest is two hexes).
- **The five days.** I-33 is the one boat that fires: 5 salvoes, 30 torpedoes, 12 hits in 56 pulses under a chance (317 force-hours), in 9 runs with a force in her hex; one Allied ship is sunk by a submarine and three damaged. Two runs of 100 differ in result and ships sunk from the boats' absence. I-19, I-17 and RO-34 are looked for and attacked from the air (RO-34 at Lunga, 1,049 looks), I-19 and RO-34 end home in 26 and 33 runs; S-42 is looked for 651 times, sighted and attacked from the air 24, and is never nearer a force than one hex.
- **The 39 days.** I-19 fires: 16 salvoes, 96 torpedoes, 28 hits in 177 pulses under a chance (976 force-hours), in the 30 runs in which a force stood in her hex; S-37 and S-42, in the hex of the Japanese stock west of the Lunga, are each under a chance in 1 and 3 pulses and fire nothing (6.3 force-hours of Japanese forces in the hex a run). Nine Allied ships are damaged and afloat by a submarine and none is sunk by one; 4 runs of 100 differ in result and ships sunk (Allied margin mean 61.58 against 61.80). Lunga and Tulagi are Allied from day 1 in every run, so I-122, I-123 and RO-34, which enter on days 7, 6 and 17, lie at an Allied base throughout: they are looked for 5,232, 5,075 and 3,269 times, sighted and attacked from the air 329, 293 and 137 times, and end sunk in 17, 14 and 7 runs and home in 56, 57 and 93 (RO-34 has her leave day, 31; I-122 and I-123 have none, so theirs is a boat damaged and sent home). S-45 at Shortland, which has no airfield, is looked for 436 times. Japanese forces stand in Shortland's hex through most of the game (6,402 force-hours a run), so S-45 attacks nothing.
- The attack chance is watched and not tuned (rule 5 of decision 0225).
