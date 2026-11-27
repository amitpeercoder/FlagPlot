# The old switches: a list for the designer

Written by the architect on Nov 27, 2026, for programme item "The old switches" of `docs/open-work.md`. It changes nothing: no code, data, test or rule. Nothing here is retired until Amit says so, switch by switch (his word of Nov 17, 2026). The frozen step 1 figures and their hand-calculation tests are not on the list and are treated throughout as fixed.

Where this page lives is ruled here: it is a report of a measurement, so it is under `docs/validation/`, which decision 0233's item 7 keeps out of housekeeping. Any change to it is the architect's.

## The short answer

There are 35 switches. Most are not old modes at all: their off side is the game that most scenarios, every recorded game and nearly every hand calculation still play. Retiring one of those buys a few lines and costs hundreds of pins, restated tests or a frozen file. Five are worth retiring, four of them at no cost to any game. Three more are worth retiring later. The other 27 should stay.

| # | Switch | On in | Recommendation | Reason in a line |
| --- | --- | --- | --- | --- |
| 1 | `command_levels` | 8 of 263 files | Retire later | Off is the game everything plays; on panics in scenario 29; closed proofs compare on with off |
| 2 | `reports` file (reports that travel) | 8 of 263 | Keep | The frozen figures have none on purpose; "at once" is a rule, not a leftover |
| 3 | `orders_inside_a_day` | 56 of 263 | Retire later | The engine does not read it; it is a permission at the seat, for the interface's design |
| 4 | `staff_chooses_supply_sources` | 20 of 263 | **Retire** | Decision 0228 kept the old choice only for closed proofs' records and says it goes |
| 5 | `submarine_pattern` | 27 of 33 files with boats | Keep | "No pattern" is a side whose staff sends no boat unasked; a real setting |
| 6 | `expects_boats_off_own_bases` | 22 true, 24 false | Keep | A doctrine value that differs by navy, not a mode |
| 7 | `air_combat` file | 163 of 263 | Keep | Giving it to all moves no game, but 100 files change, a frozen copy among them |
| 8 | `air_upkeep` file | 84 of 263 | Keep | On everywhere moves 67 games |
| 9 | `fatigue` file | 86 of 263 | Keep | On everywhere moves 82 games and 4 no longer load |
| 10 | `air_supply` file | 84 of 263 | Keep | On everywhere moves 38 games |
| 11 | `ground` file | 147 of 263 | Keep | On everywhere moves 13 games; a game with no land unit needs none |
| 12 | `class_sides` file | 81 of 263 | Keep | Data a trial needs, not a rule |
| 13 | `trial` file | 66 of 263 | Keep | "Not to be tried" is a statement about the scenario |
| 14 | The three submarine files | 33 of 263 | Keep | A frozen game has no boat, by decision 0218 |
| 15 | `merge_distance_same_pulse_hexes` | every live file | Keep | The frozen figures lack it on purpose |
| 16 | The four bombardment figures | every file, live and frozen | **Retire** | No file takes the off side |
| 17 | `[rearming]` and the port's ammunition rates | live yes, frozen no | Keep | The frozen figures lack them on purpose |
| 18 | `[fuelling_at_sea]` | live yes, frozen no | Keep | The same |
| 19 | The port's fuel rates | live yes, frozen no | Keep | The same |
| 20 | `[strike_aircraft_per_ship]` | live yes, frozen empty | Keep | A kind left out means "send everything", which the live file uses |
| 21 | `strike_is_spread` | live yes, frozen no | Keep | The frozen figures lack it on purpose |
| 22 | The three figures of fire among aircraft | live yes, frozen no | Keep | The same |
| 23 | `[[by_side]]` | live yes, frozen no | Keep | The same |
| 24 | The base hit shares for men and the airfield | every file | **Retire** | No file takes the off side |
| 25 | `men_lost_per_base_hit` | every file | **Retire** | No file takes the off side |
| 26 | The two ground report errors | every file but scenario 109's | Keep | Scenario 109's hand calculation rests on their absence |
| 27 | The two reconnaissance errors | 121 of 147 with ground | Keep | The frozen figures lack them on purpose |
| 28 | The two figures of replacement men | no file at all | **Retire** | The off side is two placeholder figures kept in code; they belong in the files |
| 29 | The three figures of hunger | 34 of 147 | Keep | Off is the game the 39 days and the proofs play |
| 30 | The two figures of mending | 33 of 147 | Keep | The same |
| 31 | The three figures of a fight's supply | 32 of 147 | Keep | The same |
| 32 | `search_fatigue_per_100_nm` | every live file | Keep | The frozen figures lack it on purpose |
| 33 | An air group's `pilots` | 35 of 155 files with air groups | Keep | Off is read by 120 scenarios and their tests |
| 34 | A scenario's own figures file | about 50 files of figures | Keep | Not a code path; it is what lets a file play the game its test proves |
| 35 | `search` and `surface_combat` optional inside `sim-core` | every loaded game | Retire later | Off only in unit tests; a refactoring for a quiet week |

## How it was measured

- **The files.** All 263 scenario files under `data/scenarios`, the five frozen copies and the proofs' files among them, read with a script for the keys each states and the figures files each names.
- **The games.** `sim-cli run` on every file as it stands, then again on a copy of `data/` with one switch turned on in every file. A game is "the same" if the output is byte for byte the same; "hashes only" if it differs only in the day hashes (the events counted each day and the closing state are as they were); "moved" otherwise. This is one seed a file, on the live figures. It does not measure other seeds, what an agent is answered, or the staffed proofs' 1,000 seeds.
- **The code.** Counts of the places that handle the absent side, by search. Lines are rough: a place is two to five lines.
- **What reads the off side.** 199 of the test files under `tests/` and the crates read the frozen figures (`data/reference/step1`), so any figure the frozen files lack is read as absent by nearly every hand calculation. The five recorded games name scenarios 5, 7, 43 and the second proof's week: all are on the off side of switches 1 to 6.

| Turned on in every file | Same | Hashes only | Moved | Would not run |
| --- | --- | --- | --- | --- |
| `command_levels` | 8 | 253 | 1 | 1 |
| `command_levels` with `reports.toml` | 7 | 0 | 255 | 1 |
| `orders_inside_a_day` | 263 | 0 | 0 | 0 |
| `staff_chooses_supply_sources`, both sides | 19 | 236 | 8 | 0 |
| `expects_boats_off_own_bases`, both sides | 0 | 263 | 0 | 0 |
| `air_combat` | 263 | 0 | 0 | 0 |
| `air_upkeep` | 196 | 0 | 67 | 0 |
| `fatigue` | 177 | 0 | 82 | 4 |
| `air_supply` | 223 | 2 | 38 | 0 |
| `ground` | 250 | 0 | 13 | 0 |

**A defect found on the way, for the manager.** With `command_levels = true` added to `data/scenarios/29_skills.toml`, the game panics on day 1 at `crates/sim-core/src/pictures.rs:231`, "base:8 is not a command of the Allied side", called from the fight ashore. The file has bases 1 to 7. No scenario on file has command levels on with a base that is not there at the start, so nothing on file meets it. It is not mended here.

## The modes a scenario states

All are keys in `crates/sim-data/src/scenario.rs` (`ScenarioFile` and `SideDoctrineSpec`).

**1. `command_levels`** (decision 0134).
- *Read:* `World::turn_command_levels_on` sets `World::chain`; off is `chain` empty. About 25 places in `sim-core` read `chain`, and 27 across `sim-core`, `sim-staff`, `sim-agent` and `sim-cli` ask `command_levels()` or `reports_travel()`.
- *On and off:* true in scenarios 61 to 64 and four files of stage 2b's proof; false in one file of that proof; absent in 254.
- *What reads the off side:* every game for play (the Coral Sea, Midway, the five days, the tables), the 39 days, every proof but stage 2b's, all five recorded games, the frozen copies, and every hand calculation but those of scenarios 61 to 64. Stage 2b's proof has an off file, `05b_eastern_solomons_off.toml`, as the arm it is compared against.
- *Code only the off side reaches:* little, about 80 to 120 lines: the "own belief" arm of each reader and a few refusals at the seat.
- *What retiring it costs (every game with commands, reports at once):* one raise of the state version and 253 scenarios' pins remade, with no baseline's game moved once item C is built; scenario 29's panic mended first; one file of a closed proof moved, its control; every recorded transcript regenerated, since a seat's answers name its commands; rules chapter 1 and chapter 8's words on the switch.
- *Recommendation:* **retire later.** It is the one switch whose two sides are two models of the game, and the design (`docs/design/command-levels.md`) is written for the on side. But today the off side is the game of record. Take it up when the interface settles whether every game has commands.

**2. The `reports` file** (decision 0135). Named by 8 files; with command levels on and no file, every report reaches every command at once.
- *Read:* `RefData::reports`, `World::reports_travel`; about 12 places.
- *What reads the off side:* scenario 61, and by design every game on the frozen figures, which have no report figures (`LIVE_ONLY` in `crates/sim-data/tests/frozen_figures.rs`).
- *Cost of retiring:* 255 games move if every game is given the live figures.
- *Recommendation:* **keep.**

**3. `orders_inside_a_day`** (decision 0156).
- *Read:* by the loader, which refuses an order with a pulse where the key is off, and by `sim-agent`'s session in two places (whether `advance` is offered, and whether a step of time is a pulse or a day). `sim-core` does not read it.
- *On and off:* true in 56 files (scenario 78, the proof's three, and the games for play); absent in 207. The designer's standing on decision 0156 is "off for the 39 days", for the speed of the tool.
- *What reads the off side:* nothing in the engine: all 263 games are byte for byte the same with it on. At the seat, every recorded game and legibility game was played without `advance`.
- *Code only the off side reaches:* about 40 lines, with the loader's test.
- *Cost of retiring:* no hash; the seat's tool list changes in 207 scenarios, so the agent's golden answers and possibly the recorded transcripts are remade; and it overturns "off for the 39 days".
- *Recommendation:* **retire later**, with the design of the interface, where the question of who may stop the clock belongs.

**4. `staff_chooses_supply_sources`** (decision 0228, A).
- *Read:* `CommandState::staff_chooses_sources`; three places in `sim-core` (`command.rs`, `command/intake.rs`) and seven in `sim-staff` (`holding.rs`, `island.rs`, `doctrine.rs`).
- *On and off:* true for a side in 20 files (the tables and decision 0228's own scenario); absent in 243.
- *What reads the off side:* every scenario with a hold or an island before decision 0228, stage 3's and stage 4's proofs, and the recorded games. Decision 0228 says why two rules are kept: "a closed proof's agent-played record cannot be said again in a new language", and that the old choice goes, and the key with it, when those records are marked as of an older engine.
- *Code only the off side reaches:* about 200 to 250 lines: a hold that forms no supply and asks for a source, the island's older choice by most supply above the reserve, and the older check of a forces limit.
- *Cost of retiring:* 8 games move on their file's seed (scenarios 69 and 79, `130_two_holds`, the four files of the five days, one of the table of hunger); three hand calculations restated by the architect (69, 79, 130) and decision 0228's statements 19 to 21 and 25 withdrawn; 236 scenarios' pins remade under one raise of the state version; the rules label raised; chapter 8 loses "what a side without the key gets"; 20 scenario files lose a line. The closed proofs are not rerun; their recorded games must be shown to replay, or be marked as of an older engine, before the key goes.
- *Recommendation:* **retire.** It is the only switch that keeps a rule the designer has already replaced ("staff chooses and user can change it").

**5. `submarine_pattern`** and **6. `expects_boats_off_own_bases`** (decision 0223, stage 0).
- *Read:* `SubmarineDoctrine` in `sim-core/src/command.rs`; the estimate of boats in `submarine_estimate.rs`; `sim-agent/src/submarines.rs`.
- *On and off:* a pattern in 27 files; the second key true for one side and false for the other in most of them.
- *Cost of retiring:* there is no one value to retire to: the two navies differ.
- *Recommendation:* **keep** both. They are doctrine, not modes.

## A figures file a scenario names or leaves out

Keys of `ScenarioFile`, each an `Option` in `sim-core/src/refdata.rs`. Without the file the rule does nothing.

| # | Key | Decision | Named by | Places that handle absence | If given to every file |
| --- | --- | --- | --- | --- | --- |
| 7 | `air_combat` | step 1 | 163 | about 22 | No game moves, no hash |
| 8 | `air_upkeep` | 0035 | 84 | about 6 | 67 games move |
| 9 | `fatigue` | 0036 | 86 | about 19 | 82 games move, 4 do not load |
| 10 | `air_supply` | 0039 | 84 | about 7 | 38 games move |
| 11 | `ground` | 0066 | 147 | about 40 | 13 games move |
| 12 | `class_sides` | 0173 | 81 | 2 | Not measured: each file needs its own |
| 13 | `trial` | 0174 | 66 | none in `sim-core` | Not measured |
| 14 | `submarine_classes`, `submarine_torpedoes`, `submarines` | 0218 | 33 | about 28 | Nothing without boats in the file |

- *What reads the off side:* for 8 to 11, between 116 and 179 scenarios and their hand calculations, the frozen copy of Savo Island among them. For 14, every frozen game.
- *Recommendation:* **keep** all eight. Number 7 is the near miss: making `air_combat` required moves nothing and would take about 50 lines out of `sim-core`, but 100 scenario files gain a line, one of them a frozen copy that may not change, and every saved game that names one of them is remade. Not worth it.

## A figure given or left out inside a file

Read in `crates/sim-data/src/reference.rs`; each is an `Option` or an empty table in `sim-core/src/refdata.rs`. `FROZEN_WITHOUT` in `crates/sim-data/tests/frozen_figures.rs` lists the ones the frozen figures leave out on purpose, so that step 1's results stand. For those, the off side is what nearly every hand calculation reads, and they stay.

| # | Figure | Decision | Live files | Frozen files | Places | Recommendation |
| --- | --- | --- | --- | --- | --- | --- |
| 15 | `search.toml`: `merge_distance_same_pulse_hexes` | 0133 | given | left out on purpose | 5 | Keep |
| 16 | `surface_combat`: the four `bombardment_` figures | 0092 | all 3 | all 3 | 4, and the loader's "all four or none" | **Retire**: make them required |
| 17 | `surface_combat`: `[rearming]`; port sizes: the two ammunition rates | 0122 | given | left out on purpose | 3 and 4 | Keep |
| 18 | `surface_combat`: `[fuelling_at_sea]` | 0201 | given in 1 of 3 | left out on purpose | 5 | Keep |
| 19 | Port sizes: the two fuel rates | 0112 | given | empty on purpose | about 8 | Keep |
| 20 | `air_combat`: `[strike_aircraft_per_ship]` | 0118 | 4 kinds of 6 | the table, empty | 1 | Keep |
| 21 | `air_combat`: `strike_is_spread` | 0211 | true | left out on purpose | 3 | Keep |
| 22 | `air_combat`: `aircraft_fire_chance`, `fire_spreads_chance`, `aircraft_fire_levels` | 0212 | given | left out on purpose | 10 | Keep |
| 23 | `air_combat`: `[[by_side]]` | 0213 | given | left out on purpose | in the loader | Keep |
| 24 | `air_combat`: the base hit shares `men` and `airfield` | 0092, 0103 | given | given | 2 | **Retire**: make them required |
| 25 | Ground: `men_lost_per_base_hit` | 0092 | all 17 | all 17 | 2 | **Retire**: make it required |
| 26 | Ground: `report_error_low` and `_high` | 0092 | 16 of 17 | 16 of 17 | 8 | Keep: scenario 109's file leaves them out so that no ground report is made, and its hand calculation rests on that |
| 27 | Ground: the two reconnaissance errors | 0147 | 4 of 17 | 2 of 17, on purpose | 7 | Keep |
| 28 | Ground: `replacement_experience` and `replacement_share` | 0197 | none | none | 2 | **Retire** the absent side: see below |
| 29 | Ground: the three figures of hunger that takes time | 0205 | 4 of 17 | 4 of 17 | 13 | Keep |
| 30 | Ground: the two figures of men slow to mend | 0208 | 3 of 17 | 3 of 17 | 10 | Keep |
| 31 | Ground: the three figures of a fight that costs supply | 0209 | 2 of 17 | 2 of 17 | 7 | Keep |
| 32 | Fatigue: `search_fatigue_per_100_nm` | 0131 | both | 1 of 2, on purpose | 5 | Keep |

- **16, 24 and 25.** Every figures file, live and frozen, already gives these. Only the loader's own tests of the absent case read the off side. Making them required moves no game, no hash and no data file. It removes about 100 lines of the loader and `sim-core` and two or three loader tests, whose removal is the architect's word (decision 0233, item 12). The rulebook's "without them no force bombards" and the like lose a clause.
- **28.** No file gives these two figures, so every game takes 2 and a half from the code. A placeholder figure in code is against the data rules in spirit. Retiring the absent side means writing the two lines into all 17 ground files and their 17 frozen copies (the frozen README allows a new figure with its first value) and making them required. No game moves; every saved game that names a ground file is remade in its file hashes only.
- **29 to 31.** `ground.toml`, which 86 scenarios name, has none of these; `ground_hunger.toml`, which the tables name, has all. Off is the game the 39 days and the proofs of stages 3 and 4 were judged on. Whether the 39 days should take hunger is a question of tuning for the designer, not of a switch.

## Others

**33. An air group's `pilots`** (decision 0038). Given for the groups of 35 of the 155 files with air groups; three of the frozen copies give it. Without it a group's pilots are not tracked. About 39 places handle the absence. Off is read by 120 scenarios and their tests. **Keep.**

**34. A scenario's own figures file.** About 50 files under `data/reference` are one scenario's or one family's figures (`ground_placeholder_scenario_68.toml` and the like, and `ground_hunger.toml`), each with a frozen copy where the frozen figures have that kind. They came with the architect's ruling under decisions 0143 and 0144 that a scenario file, run by itself, plays the game its test proves. No code path belongs to them: the loader reads whatever file is named. Retiring them would put tests back to altering figures in a copy, which that ruling ended. **Keep.**

**35. `search` and `surface_combat` optional inside `sim-core`.** The loader always supplies both; `RefData` holds them as optional "only in unit tests", and about 60 places handle the absence. No scenario, proof or recorded game reads the off side. Retiring it is a refactoring with every hash as it is, but 56 places, most of them in tests, build the reference data by hand and many would need both added. **Retire later**, as a class 2 change when someone is in those files anyway. It needs no ruling on the game, and is listed so that the count is whole.

**Not counted.** These are optional but are data or a tool's setting, not a mode of the game: `[objectives]` and `land_links`; the optional columns of the class files (aircraft capacity, anti-aircraft, men carried, the oiler's mark, the marks of estimates), which are 0 where left out; a unit's own values that default (damaged aircraft, fatigue, experience); `SimConfig::record_breakdowns`; a trial's `supplied_in_full`; and `LoadOptions::reference_root`, which is how the frozen figures are read and is not on the table.

## What it would save

If every "retire" above is done:

- **16, 24, 25 and 28:** about 150 lines of the loader and `sim-core` and three loader tests. No hash, no scenario, no hand calculation and no recorded game moves; 34 ground files gain two lines and saved games are remade in their file hashes.
- **4:** about 200 to 250 lines of `sim-staff` and `sim-core`, one rule in place of two in chapter 8, and one key out of the scenario format. Eight games move, three hand calculations are restated, the state version and the rules label are raised once, and 236 scenarios' pins are remade.

Together about 350 to 400 lines out of some 230,000 in the five crates. The three "retire later" would add perhaps 250 more. The honest reading: the switches cost little code. What they cost is that a reader must hold two games in mind, and for all but numbers 1 and 4 the second game is simply "that rule has no figures here", which the frozen figures need for as long as they are kept.

## What is needed from Amit

A yes or a no for each. The architect's recommendation is in brackets.

1. `command_levels`: retire the off side later, when the interface settles whether every game has commands? [yes, later; not now]
2. The `reports` file: keep? [keep]
3. `orders_inside_a_day`: retire later, with the interface's design, which would end "off for the 39 days"? [yes, later; not now]
4. `staff_chooses_supply_sources`: retire now, so that the staff's choice of decision 0228 is the one rule, with three hand calculations restated and eight games moved? [yes]
5. and 6. The two keys of a side's boats: keep? [keep]
7. to 14. The eight figures files a scenario may leave out: keep all? [keep]
15., 17. to 23., 26., 27. and 29. to 32. The figures the frozen files or a hand calculation leave out: keep all? [keep]
16., 24. and 25. The bombardment figures, the base hit shares and the men lost to a hit: make them required, with no game moved? [yes]
28. The two figures of replacement men: write them into every ground file and make them required? [yes]
33. An air group's `pilots`: keep? [keep]
34. A scenario's own figures file: keep? [keep]
35. `search` and `surface_combat` optional in `sim-core`: leave to the architect as a refactoring? [yes]

And one question beside the list: whether the delegation of Nov 27, 2026 covers the retirements at no cost to any game (16, 24, 25, 28 and 35), or whether each still waits on his word.

## What was ruled (Nov 27, 2026)

Added after the list was read. Amit: "Retire the five no-cost switches" (16, 24, 25, 28 and 35), and of the rest, "Don't wait on me. Let the architect make whatever decision." Decision 0236 retires the five in two pull requests, rules number 4 to be retired by a class 3 record of its own, leaves numbers 1 and 3 to the record of the interface's design, keeps the other 27, and rules that the panic of scenario 29 is mended before command levels are turned on anywhere new.

**Three corrections to the list above,** found while that record was written. The text above is left as Amit read it.

- **16, 24 and 25: "only the loader's own tests of the absent case read the off side"** is wrong. `p_a_scenario_with_no_bombardment_no_men_and_no_estimate_is_unchanged` in `tests/scenario_24_shells_and_estimates.rs` takes these figures out of the frozen files and runs scenarios 6a and 4a. Decision 0236, ruling 8, restates it.
- **28: "saved games are remade in their file hashes"** should have named them: the `save.json` of three recorded games (`full_slice_five_days_sonnet`, `keep_two_bases_fed_sonnet`, `second_proof_week_sonnet`) changes in the hash of `ground.toml` and in nothing else.
- **35 was called no cost** in the question to Amit. No game moves, but about 50 places build `RefData::default()`, two of them loaders in `sim-data`, and eight statements of `sim-core`'s unit tests about a world without the figures are withdrawn, with seven rulebook sentences, seven setup errors and one refusal of the trial. It is a pull request of its own.
