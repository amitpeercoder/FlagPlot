# 0077: Which radar sets count for sighting ships at night

Status: decided by the architect on the designer's delegation, Nov 13, 2026, as a rule of the derivation. Built on Nov 13, 2026 (see "As built"). It is the first step of the Savo Island tuning plan and is not a tuning round: no figure moves except `radar`.

Summary: Which radar sets count for sighting ships at night: a derived class has the radar figure only for a surface-search set whose display has a plan position indicator; an air-search set, a set of type not stated or one with no display row does not count and is named. At Savo Island no ship has such a set; at Cape Esperance Helena and Boise do. A rule of the derivation; no figure moves except radar. Built (Nov 13, 2026); class rows are now pinned by id in the list file

Standing: Decided by the architect on the designer's delegation

## The gap

A class has one figure for radar, `radar`, 0 or 1. The simulation reads it for one thing: a force with a ship that has it adds `radar_sighting_bonus` to its chance of sighting an enemy force in a night fight (`docs/rules/04`, "Sighting"; `crates/sim-core/src/surface.rs`). Nothing else reads it: not air search, not gunnery, not the briefing.

The derivation gives a class the figure if a ship has an air-search set or a surface-search set (decision 0019). At Savo Island that gives it to Canberra, Chicago, Quincy and Astoria, so both Allied groups sight with the bonus. In the record every Allied ship with a search set failed to detect the Japanese column.

The first tuning round was to lower the bonus. The rows say the fault is not the size of the bonus but which sets are given it:

| Ship, at Savo Island | Set | Rows in `data/oob/fits.csv` |
| --- | --- | --- |
| Chicago | CXAM, an air and surface warning set, "believed not in continuous operation during the action" | 372 (high, preferred), 61 (low) |
| Astoria | SC, an air-search set | 63 (medium, preferred) |
| Quincy | A set of type not stated, which picked up an aircraft | 347 (medium, preferred) |
| Canberra | Type 290, air warning; and Type 271, surface warning, which "gave no warning or range at Savo", "masked by nearby land; no telephone from the set to the bridge; hand-rotated" | 98, 352; 103, 355, 390 |

And of the sets themselves, in `data/oob/sensor_facts.csv`: the CXAM's display is an "A" scope (row 28, medium, preferred), the SC's an "A" scope (row 63), the SG's "PPI and A scopes" (row 71). The research note (`docs/research/oob/radar.md`, "Real limits", from a study of American night combat in 1942) says: "A-scope showed one bearing at a time. The PPI gave a picture". Under what a rule would need, it lists first: "Set type, band and display (A-scope versus PPI) per ship, not a single 'has radar' flag".

At Cape Esperance, two months later, the sets that found the enemy were the SGs of Helena and Boise (`fits.csv` 656 and 652, high, preferred: Helena's "at 2325 recorded an unmistakable surface vessel"; Boise's "searching continuously through 360"). The flagship San Francisco "lacked SG equipment" (643) and had an SC (646).

So lowering the bonus for every ship would take it from the SG as well, in the battle the surface figures are checked on. One figure that two battles want at different values means the rule is wrong, and here the rule is the derivation's.

## Decisions

1. **A class's `radar` figure is 1 only if a ship has a surface-search set whose display has a plan position indicator.** The tool reads the ship's or class's `surface_search_radar` fit as it does today, takes the set it names, and reads that set's `display` row in `sensor_facts.csv`. The figure is 1 if the fit's count is above 0 and the display's value contains "PPI".
2. **An air-search set does not count,** whatever its display. The `air_search_radar` fit is no longer read for this figure.
3. **A surface-search set with no readable display row, or of a type the source does not state, does not count.** The figure is 0, and the tool's output names the ship and the set as **not counted: no row gives its display**. It is also a gap for the research session. It is not a refusal: stopping a battle for a set nobody can name would hold back every other figure.
4. **Readable means what it means for every other fact** (decisions 0020 and 0023): a `preferred` row of medium or high confidence, valid on the list's date, or a row named by an exception in `rules.toml`. Rows that disagree stop the derivation, as for any fact.
5. **A sourced "none" is read as now.** A fit of the stand-in for no radar of any kind, at count 0, gives 0. The list's working assumption that unknown radar means none (decision 0019, 5) is unchanged, and the output goes on naming the ships it was used for.
6. **Nothing in the simulation changes,** and `radar_sighting_bonus` is not moved. The frozen step 1 figures and the placeholder classes are untouched.

## The hand derivation

Each line is worked by the rule above and is the expected value of the tool's test.

**Savo Island** (the list's classes that have the figure today, and the Japanese)

| Class | Today | Surface-search fit read | Its display | Figure |
| --- | --- | --- | --- | --- |
| County, Kent subclass (Canberra) | 1 | Type 271, count 1 (`fits` 102, 103, 353, 355, 390) | no row | **0**, not counted: no row gives the Type 271's display |
| New Orleans, radar (Astoria, Quincy) | 1 | Astoria: none (her SC is an air-search set, `fits` 63). Quincy: row 348 is unconfirmed and low, so not read | none to read | **0** |
| Northampton (Chicago) | 1 | A set of type not stated, count 1 (`fits` 291, medium, preferred: "at 0226 Chicago's radar made contact with a ship at 7,000 yards") | no row: the type is not stated | **0**, not counted: no row gives the set's display |
| Bagley, radar (the pickets Blue and Ralph Talbot; not in scenario 4) | 1 | none: their SC is an air-search set (`fits` 82 to 85, 350, 373, 374) | none to read | **0** |
| Bagley, no radar; Benham; New Orleans, no radar | 0 | none; the list's assumption | | **0**, by the assumption as today |
| Every Japanese class | 0 | the stand-in for no radar, count 0 (`fits` 280 to 287, 357 to 366) | | **0** |

So every class of the Savo Island list has 0, and the two classes whose surface set has no display row are named.

**Cape Esperance** (ships whose sets are in rows read for this record; the list is derived by decision 0072 and is not changed here)

| Ship | Surface-search fit read | Its display | Figure |
| --- | --- | --- | --- |
| Helena | SG, count 1 (`fits` 656, high, preferred) | "PPI and A scopes" (`sensor_facts` 71) | **1** |
| Boise | SG, count 1 (`fits` 652, 653, high, preferred) | the same | **1** |
| San Francisco | SG, count 0 (`fits` 643, 644, high, preferred: "lacked SG equipment"). Her SC (646) is an air-search set | | **0** |
| Salt Lake City | SG, count 1, but the row is `disputed` (`fits` 649: "so, presumably"); row 683 is of 1943 | | not settled: the rows stop the derivation until the dispute is settled, by whoever derives that list |
| Farenholt | A set of type not stated, count 1 (`fits` 661, medium, preferred: "obtained radar contact bearing 345 T, distance 13,000 yards") | no row | **0**, not counted |
| Duncan | Rows 662 and 663 are `disputed` | | not settled |

**Made cases for the tool's test** (not real ships)

| Case | Figure |
| --- | --- |
| A surface-search fit of a set whose display is "PPI and A-scope", count 1 | 1 |
| The same set, count 0 | 0 |
| A surface-search fit of a set whose display is "A-scope", count 1 | 0, with no note: the display is read and has no plan position indicator |
| A surface-search fit of a set with no display row, count 1 | 0, named as not counted |
| An air-search fit of a set whose display is "A-scope and PPI", and no surface-search fit | 0 |
| The no-radar stand-in at count 0 | 0 |
| No radar fit at all, with the list's assumption | 0, named under the assumption as today |
| No radar fit at all, without the assumption | refused, as today |

## What it does at Savo Island

It is the same there as the bonus at 0, since only Allied ships have the figure. Over 1,000 seeds each, on main on Nov 13, 2026 with decisions 0070 and 0071 and the northern group's break-off level in:

| | Night fights the Japanese sight first / both in one round / the Allied side first | Runs in which every night fight is Japanese-first |
| --- | --- | --- |
| Today | 872 / 711 / 414 | 204 |
| With this rule | 1,140 / 542 / 316 | 335 |

Gun hits on Allied ships go from a median of 95 to 100 and on Japanese ships from 17 to 13 (the record is 10 to 17); the minutes of action from 40 to 35; ships sunk do not move.

**How the rule came to be written, said plainly.** It was written after a scratch run of the bonus at 0.1 and at 0 had been seen. It rests on the rows on the kind of set and its display, not on the run: the run shows only what the rule does. The measure of who sights first still agrees only at the edge with it, and that is left to show.

## What it does not do

Each is a gap, named so that no figure is asked to cover it.

- **Land behind the target.** Canberra's surface set was masked by land, Astoria's "swamped by land on most bearings", and at Tassafaronga the land mass hid contacts. A ship with an SG close under a coast keeps the whole bonus.
- **A set switched off, or kept silent for fear of being detected.**
- **Range.** The bonus is one number whatever the set's range and whenever the fight begins.
- **A set of an older kind that did see a ship.** Chicago's made a contact at 7,000 yards after the action, and Farenholt's at 13,000 yards at Cape Esperance. By this rule neither counts, because no row says what the set showed its operator.
- **Any use in gunnery.** Vincennes fired her first salvo on a radar range. Fire-control sets are not read and no figure reads them.
- **Passing the picture on.** Helena's contacts reached the admiral by voice after minutes. A force has the bonus if any ship has the set.

## Why

- **It is a fact of the ship.** Which set a ship carried and what it showed are in the rows. A tuned figure cannot tell an SC from an SG.
- **One rule for both battles.** At Savo Island no ship has such a set; at Cape Esperance two do. That is what the record has.
- **What is derived is checked before what is tuned** (step 2 spec, "Tuning", 6). The bonus multiplies nothing if the figure it reads is wrong.
- **A gap stays a gap.** A set nobody can name is not counted and is named, and no ship is given a bonus on a guess.

## What it costs

- `oob-derive`: the reading of a set's `display`, the rule, the note for a set not counted, and tests on the hand derivation.
- `rules.toml`: the rule's text, appended after `[search_radius]`.
- The Savo Island class file is generated again; scenario 4's run baseline and the Savo Island report are remade. That report is the one the tuning rounds start from.
- The Eastern Solomons class file has classes with the figure today from air-search sets. They become 0, the file's bytes change, and whatever pins that file's hash is remade (saved and recorded games, baselines). No surface weapon is modelled for those classes (decision 0029), so no fight of theirs changes; the build says whether any run does.
- `docs/rules/04` and the glossary say what a derived class's `radar` now means. No rule of the simulation changes, so there is no rule card.

## As built

Nov 13, 2026.

**The tool.** `oob-derive` reads `sensor_facts.csv`. A class's `radar` is 1 only if a read `surface_search_radar` fit has count above 0 and the set it names has a read `display` row containing "PPI". An air-search fit no longer gives the figure; it is read only to know that a radar fit is on record (so an air-search fit alone gives 0, not assumed and not refused), and its read errors do not stop the derivation. A surface-search set with no readable display row is not counted and is named in the class file's header ("Radar sets not counted, no row gives their display") and in the radar row of the sources file; the two sets so named are the Type 271 (Canberra's class, `fits.csv:102`) and the set of type not stated (Chicago's class, `fits.csv:291`), recorded in `gaps.csv`. The rule's text is in `rules.toml` and in the tool's output. No exception was needed.

**Class rows are pinned by id in the list file (added at the build, not in the decision above).** Merging two rows of a class that now have equal figures renumbered every later class id, which the live scenarios, the frozen figures and the hand-calculation tests all name by number. The tool had numbered rows by counting groups of equal figures, which was fragile. A list that generates class files now has a `[class_ids]` table giving each id and its ships; the tables of `savo_island.toml` and `eastern_solomons.toml` were written from the `.ships.csv` files on main, so every ship keeps its id. The tool makes one row for each pinned id, in id order, and never merges two. It refuses ships pinned to one id whose figures differ (naming the ships and the figure), a ship with no pinned id, a pinned ship not in the list, an id or a ship pinned twice, an id below `first_class_id`, and a list with ships and no table. Where two rows of one class have equal figures, each is named with its ships in brackets, so no row is called "radar" when its radar is 0.

**Class names that changed.** Savo Island: "Bagley class, no radar" is now "Bagley class (Bagley, Patterson, Helm, Jarvis)" (1007); "Bagley class, radar" is "Bagley class (Blue, Ralph Talbot)" (1008); "New Orleans class, no radar" is "New Orleans class (Vincennes)" (1010); "New Orleans class, radar" is "New Orleans class (Quincy, Astoria)" (1011). Eastern Solomons: "Shokaku class, no radar" is "Shokaku class (Zuikaku)" (2006); "Shokaku class, radar" is "Shokaku class (Shokaku)" (2007). The other names are as before.

**What changed in the generated files.** Radar went from 1 to 0 at Savo Island for County class (Kent subclass), Northampton, the New Orleans row of Quincy and Astoria and the Bagley row of Blue and Ralph Talbot, and in the Eastern Solomons class file for Shokaku, Lexington, North Carolina, Portland and Yorktown (Atlanta keeps it: it has an SG, a surface-search set with a PPI). The class ids and the number of rows (13 at Savo Island, 21 in the Eastern Solomons file) are as before. The files changed are the class, sources and ships files of both lists; no aircraft or experience file changed.

**Baselines remade.** `crates/sim-cli/tests/run_baseline/04_savo_island.events.txt` (two lines: the Allied groups' night sighting chance of the Japanese loses `radar_sighting_bonus`, 0.4375 to 0.3125 and 0.3325 to 0.2375; no event, hit or hash of the run changed on seed 1942), made with `sim-cli run data/scenarios/04_savo_island.toml --events`. The three recorded games' `save.json` files (the data-file hash of the Eastern Solomons class file), made by replaying each `game.jsonl` through `sim-cli mcp`; every transcript is byte for byte as before. No run baseline of scenarios 5, 7 or 8 changed, nor any event of any run of them: only the data-file hashes inside the saved games moved.
