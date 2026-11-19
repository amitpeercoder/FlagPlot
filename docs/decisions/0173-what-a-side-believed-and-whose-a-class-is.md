# 0173: What a side believed, and whose a class is: the second part of step 4 of stage 4

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 4 of its ten, section C, rules 7 and 8), which the designer confirmed. The second of the three records of step 4: the data the enemy's half is built from. It adds two kinds of entry to a scenario and one reference file, and no rule of play. Built on Nov 19, 2026 (As built, below).

Summary: To build the enemy's half of a picture a constructor needs three things no view gives it and no rule should guess: which classes of ship and types of aircraft are the enemy's; what forces the side believed the enemy had and had not seen; and what the side believed the enemy meant to do. This record gives each a place in the data: a file that says whose each class and type is, and in a scenario the forces a side believes exist and the intents it believes the enemy holds. The 39 days are given theirs, as placeholders until the record of what each side believed is researched

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161. Built on Nov 19, 2026

## Why

The experiment of decision 0100 built an enemy from "the lowest class of the contact's kind that the side does not itself use", and noted that it works only while the sides share no class. It found that a view with no contact gives a world with no enemy ships, "valid, and proves nothing", and asked for the scenario to state the forces a side believes exist and has not seen, with "not seen" and "believed absent" as different inputs. And it found that an invented enemy with no orders sits still.

Decision 0161 took each up (rules 7 and 8): the classes "the scenario's reference data marks as that side's"; believed forces with three standings; and what the enemy wants, "stated, not found out", in the staff's own vocabulary.

What is so today, read from the code and the data:

- **No class or type has a side.** The class files and the type files have no such column, and nothing lists the classes of a side. A ship has its class and its side in the scenario. Placeholder classes are used by both sides.
- **A scenario states what a side holds at the start** in two ways: `[[contact]]`, a place, a count and a kind with an age; and `[[estimate]]`, a place and a count of men. Both become belief. Neither can say "a carrier force exists and we do not know where".
- **An intent is written in a scenario** as a `[[command]]` of kind `set_intent`, with the keys of its kind.
- **The generated class files are never edited by hand,** and a missing fact is a gap, not a guess (`docs/engineering/data-rules.md`).

## The rules

### Whose a class is

1. **A file of sides.** A scenario may name one file, `class_sides`, a CSV of the classes of ship and the types of aircraft its reference files give, each with the side that uses it: `allied`, `japanese` or `both`. Its columns are `table` (`ship_class` or `aircraft_type`), `id`, `side` and `note`. It is a file of its own and written by hand, so that no generated file changes.
2. **It is whole and it is true of the scenario.** Where a scenario names the file, the loader refuses the scenario if a class or type its reference files give has no row, if a row names a class or type they do not give, if one is given twice, and if a ship or an air group of the scenario is of a class or type the file gives to the other side alone.
3. **What a row rests on.** A class or type derived from the order of battle takes the side of the ships or groups its derivation list gives it, and its note names the list. A placeholder class takes `both` unless the note says why not. The side of a real class is a fact of the navy that built it and needs no figure.
4. **The reference data holds it,** for a view to read as it reads a class: whose a class is and whose a type is. A scenario that names no file has none, and plays as before.

### What a side believed exists

5. **`[[believed_force]]`** in a scenario states one force of the enemy's that a side believes in and does not hold a contact of. Its keys:
   - `side`: the side that believes it;
   - `name`: words, for the estimate to name it by;
   - `kind`: the kind of its most important ship, as a contact has one;
   - `ships`: how many ships it believes it has, 1 or more;
   - `standing`: `present`, `absent` or `unknown`;
   - where it was last known: `at_base` (a base on the side's map), or `q` and `r`, with `day`, the day it was known there (0 for before the first day, the default); or neither, where the side does not know where;
   - `placeholder`: true where the belief is not from a source, with a `note`.
6. **The three standings mean** (decision 0161, 7): believed present, it is there; believed absent, it is not, and the entry is the record that the side ruled it out; unknown, the side does not know whether it is there. The loader gives no chance to any of them.
7. **`[[believed_intent]]`** states one intent a side believes the enemy holds. It has `side`, the side that believes it, and then the keys of a `set_intent` command for the enemy: `intent_kind` and what that kind takes. It is checked as that command would be checked for the enemy's side on the side's own map: an intent that names a base names one on the map of the side that believes it. A kind the staff alone may give is refused, as it is in a command.
8. **They are the scenario's, not the world's.** A believed force and a believed intent are not state, are not belief, are never hashed and change no game. The loader checks them and keeps them with the scenario, for the constructor to be handed with a side's view. No rule of the engine reads them, and a seat is not shown them by this record.
9. **The loader refuses** an entry with an unknown key, a side or a kind it does not know, no ship, a standing it does not know, a base that is not on the believing side's map or a hex off the map, a day after the scenario's last, both a base and a hex, and an intent the enemy could not be given.

### The 39 days

10. **The full slice and the three arrangements of the proof name a file of sides** for their classes and types.
11. **They are given what each side believed on the evening before 7 August,** as placeholders, each marked and noted as resting on no source yet:
    - the Allied side believes a cruiser force present, last known at Rabaul before the first day, of 6 ships;
    - the Allied side believes a carrier force of 8 ships, standing unknown, where not known;
    - the Allied side believes the Japanese side means to hold Lunga and to hold Tulagi;
    - the Japanese side believes a carrier force of 8 ships, standing unknown, where not known;
    - the Japanese side is given no belief of what the Allied side intends.
12. **The research is owed.** What each command believed of the other's forces and purpose in the first week of August 1942 is in the record, and belongs in `data/oob` as rows before these entries lose their mark. Until then no estimate that rests on them is anything but a placeholder's.

## What it leaves for later

- **How a believed force becomes ships,** where one of unknown place is put, and what the enemy does with a believed intent: the third record of this step.
- **Belief that changes.** A believed force is the opening belief. What a side learns in play is in its contacts, claims and estimates; nothing here strikes a force out when it is found.
- **A seat shown what its side believes exists.** The directive from above (decision 0157) is where a commander would be told; it is not built here.

## Rule card

- **Reads:** a scenario's file and its reference files.
- **Writes:** nothing in a game. The scenario as loaded holds the believed forces and intents; the reference data holds whose a class is.
- **Reuses:** the loading of a contact's kind and place, of a base on a side's map and of a `set_intent` command.
- **New:** one reference file kind; two kinds of scenario entry; in `sim-core`, the plain data they are loaded into.

## What it costs

- **No game changes.** No state, no draw, no hash. A scenario that names no file and states no belief loads as before.
- **`sim-data`** loads the file and the entries; `docs/commands.md` and the scenario notes say how they are written; `docs/engineering/data-rules.md` gains the file of sides and what a row rests on.
- **The saved game** records the hash of each data file a scenario reads: the file of sides is one of them, so a saved game of a scenario that gains one is of other inputs. The build remakes what must be remade and says what.

## What the tests are to show

Written by the architect; the build writes the tests to them.

1. **A file of sides, by hand.** A scenario of the placeholder classes with class 1 given to the Allied side and class 2 to both: it loads, and the reference data answers Allied for class 1 and both for class 2. Refused, each with words that name the fault: a class the files give with no row; a row for a class they do not give; a class given twice; a Japanese ship of class 1.
2. **A believed force.** One entry of each standing loads and is kept as written: the side, the name, the kind, the count, the standing, the place and its day. An entry at a base is kept with that base's hex. Refused: no ship; an unknown standing; an unknown kind; a base that is the believing side's own; a hex off the map; both a base and a hex; a day after the last; an unknown key.
3. **A believed intent.** "The Allied side believes the Japanese side holds base 2" loads and is kept as a hold of base 2 for the Japanese side. Refused: an intent naming a base that is not on the Allied side's map; a kind only the staff may give; an unknown key.
4. **The 39 days.** The full slice and each arrangement of the proof load with their file of sides: every class and type they give has a row, and no ship or group is of a class or type given to the other side alone. Each holds the five entries of item 11 as written, each marked placeholder.
5. **Nothing changes.** Every scenario of the repository plays as it did: no run baseline and no pinned daily hash moves. What changes is listed: the saved games and fixtures that record the hash of a scenario file or name its data files.

## As built

Built on Nov 19, 2026. The five statements of "What the tests are to show" hold: statements 1 to 3 in the loader's own tests in `crates/sim-data/src/scenario.rs` (`a_file_of_sides_says_whose_a_class_is_and_is_refused_where_it_is_not_whole_or_true`, `a_believed_force_is_kept_as_written_and_refused_where_it_cannot_be`, `a_believed_intent_is_checked_as_the_enemys_command_on_the_believing_sides_map`), statement 4 in `crates/sim-data/tests/the_39_days_beliefs_and_sides.rs` (each of the four files on the live and on the frozen figures), and statement 5 by `sh scripts/regenerate.sh` (no run baseline, agent golden, pinned hash or batch fixture moved; one saved game changed, below) and by `believed_entries_and_a_file_of_sides_change_nothing_in_the_world` (the world loaded with a file of sides and both kinds of entry has the same state hash and log as the world loaded without them).

**What was built.**

- `sim-core`: `refdata::ClassSide` (`Allied`, `Japanese`, `Both`, with `name` and `used_by(side)`), `refdata::ClassSides` (the two maps, by class and by type), `RefData::class_sides: Option<ClassSides>` with `RefData::ship_class_side` and `RefData::aircraft_type_side`; the two readers `SideView::ship_class_side` and `SideView::aircraft_type_side` beside `ship_class` and `aircraft_type` in `view.rs`; a module `believed` with `BelievedForce`, `BelievedIntent`, `Standing` and `LastKnown`, plain data; and `World::check_root_intent(side, kind)`, which a commander's `set_intent` now goes through and the loader calls for a believed intent. `RefData` is not hashed (`hash.rs` names it as the same for a whole game), so the new field changes no hash; nothing of the engine reads it.
- `sim-data`: the scenario keys `class_sides`, `[[believed_force]]` and `[[believed_intent]]`; `load_class_sides`; `Scenario::believed_forces(side)` and `Scenario::believed_intents(side)`, in the file's order, copied by `with_seed`.
- Data: `data/reference/class_sides_full_slice.csv`, 50 rows (39 classes, 11 types), and its frozen copy `data/reference/step1/class_sides_full_slice.csv`, 44 rows; the key and the five entries in `data/scenarios/07_full_slice.toml`, `data/scenarios/staff/07_full_slice.toml` and, made again by `scripts/stage-3-proof/make_arrangements.py`, `data/scenarios/stage-3-proof/07a_slice.toml`, `07b_slice.toml` and `07c_slice.toml`.

**Readings the record did not make,** for the architect:

1. **One file serves all four.** The full slice, the staff's setup and the three arrangements name the same class and type files, so one file of sides serves them.
2. **The file of sides is a reference file,** read by `reference()` and so redirected by `LoadOptions::reference_root` by its file name, as the class files are. It must be: the agent's tests load the full slice on the frozen step 1 figures, whose `ship_classes_guadalcanal_landings.csv` lacks classes 3011 to 3016, and a file whole for the live classes is refused there (rule 2, a row for a class they do not give). So `data/reference/step1` holds a copy with rows for exactly the frozen classes and types (no 3011 to 3016), noted in its README; `crates/sim-data/tests/frozen_figures.rs` also requires a frozen copy of every CSV in `data/reference`. The alternative, reading it where the scenario names it as the report figures are, would refuse every frozen load of the slice.
3. **What each row rests on.** The 10 placeholder classes (1 to 6, 8, 10, 11, 12) are `both`. The 29 derived classes take the nation in `data/oob/ships.csv` of every ship their list's `[class_ids]` gives them, all one nation for each class: 2000 to 2009 and 3009 to 3016 `jp`, Japanese; 2010 to 2020 `us`, Allied (lists `data/derivation/eastern_solomons.toml` and `guadalcanal_landings.toml`). The 11 types take the nation in `data/oob/aircraft_types.csv` of the type at their place in the list's `[[aircraft]]` order from its first id, checked against the generated names: 2000 to 2002 and 2100, 2101 `us`; 2003 to 2006, 2102, 2103 `jp` (lists `eastern_solomons.toml` and `rabaul_guadalcanal.toml`). Every class and type could be placed; none was left out. Classes 3011 to 3016 have rows though no ship of the four files is of them: the rule asks a row for every class the reference files give.
4. **Every row has a note,** and an empty note is refused; an unknown `table` or `side` word is refused.
5. **Rule 2's last check reads ships and air groups only,** as the record says. A replacement stream or an arrival names an aircraft type and a side and is not checked.
6. **A believed force at a base** names a base of the scenario that is not the believing side's own (statement 2) and is on its map: a base of the other side with `on_map` true. It is kept with that base's hex at the start.
7. **The day of a believed force** defaults to 0 and may be the scenario's last day; a day after it is refused, and so is a day with no place (a day is the day it was known at a place). One of `q` and `r` alone is refused.
8. **`placeholder` defaults to false;** a placeholder needs a note, and a note given may not be empty. The loader does not check that a placeholder's name starts "Placeholder"; the 39 days' forces are so named by hand.
9. **A believed intent requires `intent_kind`,** where a command defaults to `keep_supplied`, and takes no `intent` number, `day` or `kind`. Its keys are the command's keys of any `set_intent` kind (`base`, `target`, `hex`, `distance`, `floor_days`, `interval_days`, `task_force`, `wants`, `defences`, `by_day`, `island`) and `placeholder` and `note`; any other key is refused by name, and a key its kind does not take is refused in the command's own words ("does not take ..."), since the table is built into a `[[command]]` and parsed by the same code (`CommandSpec::build_kind`).
10. **The check of a believed intent** is, in order: every base the kind names (its own base, and a target) is a base of the believing side or one of the other side's with `on_map`; then `World::check_root_intent` for the enemy's side on the world as loaded, the same check a `set_intent` of a new number gets (a kind that may not be a root, which is `search_waters`; then the kind's own base, target, hexes, force and island). The parts of the command check that read the intent's number and the command state do not apply: there is no number and no command state is written. As first built, a believed intent naming a task force (`stand_at`, `operate_carriers`, `ready_landing`, `bombard`) was checked against the enemy's real forces.

    **The architect's ruling on reading 10 (Nov 19, 2026):** a believed intent may not name a force of the enemy's. A side cannot name an enemy force by its number, and checking the name against the enemy's real forces reads what the side cannot see. So a `[[believed_intent]]` whose kind takes a task force (`stand_at`, `operate_carriers`, `ready_landing`, `bombard`) is refused, by its kind alone, with words that say a side cannot name a force of the enemy's. The kinds it may hold name only a base, a hex or an island: `keep_supplied`, `cover`, `hold`, `strike_ships`, `deny`, `build_up`, `take_place`, `control_island`, `neutralise`, `raid`. As built: the bases a believed intent names are checked against the believing side's own view (its own bases, and the enemy bases it sees); then `World::check_root_intent` for the enemy's side reads, for these kinds, only the side of each base named, which the believing side's map shows; the `on_map` of the believing side's own base, where the enemy is believed to hold or strike it, which is a field of the side's own base, known to it whole; and the map (hexes, kinds of hex, islands). It reads no unit and no stock of the enemy's, so no remaining kind had to be refused. `a_believed_intent_names_no_enemy_force_and_reads_none` refuses each of the four kinds and loads the same ten believed intents (each kind left but `control_island`, which the test scenario has no island for) from two scenarios that differ only in one more enemy task force with a ship. (The coordinator's message called this reading 8, which was its number in my report; it is reading 10 here.)
11. **Nothing reads them.** The entries are on the loaded `Scenario`, not in the `World`; no view, seat, tool or rule is given them.
12. **The staff's setup was changed by hand** (accepted by the architect, Nov 19, 2026; the script that made it, `make_staff_setup.py`, is not in the repository). `make_staff_setup.py` is kept outside the repository (decision 0140, "As built") and was not to hand. The same change made to scenario 7 was made to `data/scenarios/staff/07_full_slice.toml`, the `class_sides` key with its path one folder deeper and the same block of five entries after its two opening estimates, which is what the script carries over from scenario 7 for every table it does not take out. Whether the script, run again, makes the file byte for byte is not checked; the arrangements were made from it by `make_arrangements.py`, whose `--check` passes. Not changed: the frozen copy `data/scenarios/step2-frozen/07_full_slice.toml` and the files of the closed proofs of decisions 0128 and 0138.

**What changed of what is generated.** `crates/sim-cli/tests/recorded_games/full_slice_five_days_sonnet/save.json`: the hash of `data/scenarios/07_full_slice.toml`, and one more data file listed with its hash, `class_sides_full_slice.csv`; its paragraph is in that folder's `README.md`. And one pinned count in a test, `crates/sim-cli/tests/saved_games.rs` (`the_saved_game_holds_orders_with_their_origin_and_the_files_it_names`): the full slice's saved game names 18 data files, not 17. Nothing else: no run baseline (the full slice's run, events and daily hashes are as they were), no agent golden, no pinned hash, no batch fixture.

