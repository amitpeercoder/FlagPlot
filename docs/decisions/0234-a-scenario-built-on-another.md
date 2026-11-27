# 0234: A scenario built on another

Status: a plan by the architect, written on Nov 27, 2026, on Amit's delegation of that day (decision 0233's Status has his words). It is the record for items 4 and F of the programme in `docs/open-work.md`. Nothing is built.

Summary: A scenario file may state `built_on = "another.toml"` and then holds only what differs: `sim-data` reads the file below, lays this one over it and loads the result as it loads any scenario. The 58 scenario files that nine Python scripts in six folders make become such files, each shown to resolve to the table of the file it replaces and to play the same game day for day before that file goes; the generators are then deleted. The other 205 files stay as they are. Before it, `scenario.rs` is split by table as a pure move. A saved game still records the hash of the file it names, and now also the hash of each file that one is built on. No rule of the game changes.

Standing: Ruled by the architect on Nov 27, 2026, on Amit's delegation; class 2, every step, the split of `scenario.rs` among them; to be built

## Why

`data/scenarios` holds 263 files and 212,996 lines, of which 9,567 are distinct. More than half of the lines, 113,746, are in six folders whose files are one scenario told again with a name, a set of orders, an oiler, a ground figures file or some boats changed. They are kept alike by nine Python scripts in six folders under `scripts/`, 2,754 lines, which copy text and are run by the tests with `--check`. So a scenario has two sources, its text and its script; a reader of a 2,400 line file cannot see the twenty lines that make it itself; and a change to a base rewrites up to sixteen files. The loader that reads them, `crates/sim-data/src/scenario.rs`, is one file of 10,550 lines, 5,934 of them tests.

## What was weighed

- **Include: a file made of named pieces** (`include = ["ships.toml", "orders.toml"]`). Not chosen. A piece is not a scenario, so a new kind of file appears that loads only inside another, and decision 0072's question (is this file run?) has to be answered for it. Every family here is "that scenario, with this changed", which one file below answers.
- **The key named `base`,** as `docs/open-work.md` first had it. Not possible: `[[base]]` is the table of a scenario's bases, and a TOML file cannot hold both. The key is `built_on`.
- **A row replaced whole.** Not chosen. The table's 39 days change three keys of nine task forces; a row told again in full would keep its other keys as they were on the day it was copied, and drift from the file below without a word. A row states the keys it changes.
- **A way to take a row or a key away.** Not chosen: none of the 58 files needs it. A scenario with less than another is the file below, or a file of its own.
- **Hash the resolved scenario in a saved game, not the file.** Not chosen: there is no one text of a resolved scenario to hash, and the hash of the bytes of each file read is what a saved game already keeps for every data file.
- **Convert every near copy,** the proofs' files and scenarios 7 and 8 among them. Not done: see "Which files stay as they are".
- **Keep the generators, frozen.** Not done: a generator whose `--check` can no longer pass is a second, wrong account of how a file is made.

## The format as planned

The pull request that builds the loader (step B) moves this section to `docs/rules/01-the-day-orders-and-what-a-side-knows.md`, as a section "A scenario built on another" after "A game from a scenario", and leaves one line here naming it.

1. **`built_on`.** A scenario file may state `built_on = "path"`, one path, relative to its own folder. The file named is **the file it is built on**; the file that states the key is **an overlay**. A file with no `built_on` is read exactly as before this record, error for error.
2. **A chain.** The file built on may itself be an overlay. A chain that comes back to a file it has read is refused, and so is one of more than 8 files. A file states one `built_on`, never a list.
3. **What is read.** The loader reads the lowest file's table, lays each file above over it in turn, and gives the result to the one loader of scenarios, with the folder of the file that was asked for. Unknown keys, missing keys, floats and every other check are made on the result, as for a file written out in full.
4. **A plain key** (anything that is not a list of tables: `name`, `days`, `ground`, `ship_classes` whether one name or a list, the `[objectives]` table, the `[side]` table) stated by the overlay takes the place of the whole value below. `[side.allied]` in an overlay therefore drops a `[side.japanese]` below unless the overlay states it too.
5. **A list of units by id** (`[[base]]`, `[[ship]]`, `[[task_force]]`, `[[air_group]]`, `[[land_unit]]`, `[[submarine]]`, `[[formation]]`). A row of the overlay whose `id` the list below has changes that row: each key the row states takes the place of that key's whole value, and the row's other keys and its place in the list stand. A row with a new `id` is put after the rows below, in the overlay's order. A row with no `id`, or two rows of one `id` in one file, are refused.
6. **Any other list of tables** (`[[order]]`, `[[command]]`, `[[contact]]`, `[[estimate]]`, `[[replacement]]`, `[[arrival]]`, `[[supply_line]]`, `[[men_replacement]]`, `[[suspected_waters]]`, the three `[[believed_...]]` lists, `[[commander]]`, `[[island]]`): the overlay's rows are put after the rows below, in its order.
7. **Nothing is taken away.** An overlay cannot remove a row, remove a key of a row, or change the order of what is below.
8. **A file a scenario names** (`map`, `land_links`, `ship_classes`, `port_sizes`, `aircraft_types`, `search`, `surface_combat`, `air_combat`, `air_upkeep`, `fatigue`, `air_supply`, `ground`, `reports`, `class_sides`, `trial`, `submarine_classes`, `submarine_torpedoes`, `submarines`) is read relative to the folder of the file that states it. Where the overlay is in another folder than the file it is built on, a name it takes from below is given the path from the one folder to the other before it, and each plain folder name followed by `..` is then struck out: `../coral-sea/` before `../../map/x.csv` is `../../map/x.csv`. A name the file asked for states itself is used as written. A key that names a file, added to the scenario format later, is added to this list in the same change.
9. **`built_on` is not a reference file.** It is read where it is named, through the same `Source` as every other file, and `LoadOptions::reference_root` does not move it.
10. **The file built on is a whole scenario.** It loads and runs alone. No file is "only a base": whether a loop over `data/scenarios/*.toml` walks a file is decided, as before, by the folder it is in (decision 0072), for an overlay and for the file it is built on alike. An overlay directly in `data/scenarios` is a scenario the loops run; none is made by this record.
11. **What a saved game records.** `Scenario::data_files` ends with the files the scenario is built on, nearest first, after every file listed today. A saved game, a game file, a batch file and an export therefore record the hash of the bytes of the file named, as now, and the hash of the bytes of each file below it, and a saved game is refused when any of them has changed. A pack holds them too, with nothing added to `pack_of`. No version moves: a file with no `built_on` records what it recorded.
12. **The resolved text.** `sim_data::Scenario::resolved_text(source, path)` gives the resolved scenario as TOML: it holds no `built_on`, and every file name in it is written for the folder of the file asked for (item 8), so the text put in that folder, or handed to `from_text` with that path, loads as the overlay does. `sim_data::Scenario::built_on_chain(source, path)` gives the files read, the one asked for first. **`sim-cli resolve FILE`** only prints the first on standard output and the second on standard error. Neither builds a world or reads a file but the chain. It is the umpire's tool, for reading an overlay whole and for the proof below.
13. **`from_text`.** Text that states `built_on`, handed to `Scenario::from_text` or `from_text_with`, is laid over the file it names, read relative to the path given, as a file at that path would be. Text with none is read as before.
14. **Where an overlay may not be.** No file in `data/scenarios/step2-frozen`, no file of a proof (`orders-inside-proof`, `second-proof`, `stage-2b-proof`, `stage-3-proof`, `stage-4-proof`, `staff`) and no scenario a recorded game names is an overlay. Any of them may be built on: that reads it and changes nothing in it.

## Rule card

No rule of the game is added, changed or removed. No state, draw, event, order or tool kind; nothing in truth and belief; no state, save, rules or interface version moves.

- **Reads:** scenario files, in `sim-data` alone, through `Source`. `sim-core` reads no file and is not touched.
- **Writes:** nothing in the state. `Scenario::data_files` is longer for an overlay.
- **New:** the key `built_on`; `sim-cli resolve`; the glossary's "Built on" and "Overlay".

## Its class

- **Every step is class 2,** and this record is its record. The architect reads each pull request before it is merged.
- **Step A, the split of `scenario.rs`,** changes lines under a crate's `src`, which decision 0233's item 7 leaves off the list of class 1: it is "a refactoring of a crate's `src` with every hash as it is". Step E is the same.
- **Steps B, C and D** are "a loader or tool change with a stated equality to check". A converted file is the same scenario as the file it replaces, by equality 1 below, so no scenario of record or for play is changed. A file that fails equality 1 would be a changed scenario: for that file the class rises to 3 and the build stops. The statements are in the doc comment of `tests/scenarios_built_on_another.rs`, written before any code. No game of a closed proof and no validation measure moves; if one does, the class has risen and the build stops (0233, 10).

## The proof that nothing is lost

- **Step B (the loader).** Every file on `main` has no `built_on`, so: every golden file, pin, run baseline, recorded game, saved game fixture and the web check pass untouched, and the loader's own 5,934 lines of tests pass with no expected text changed.
- **Steps C and D (each file converted), three equalities, made by `scripts/overlay-proof.sh REV`** with `REV` the commit the branch left `main` at, using the branch's own `sim-cli` on both trees:
  1. `sim-cli resolve` of the overlay is byte for byte `sim-cli resolve` of the file as it was at `REV`. The loader is one function of the resolved table and the folder, so this alone gives the same world, script, commands and beliefs, read from the same reference files. The overlay's list of data files is that list and then the files it is built on (item 11).
  2. `sim-cli run` of the two prints the same text: the same hash and count of events for every day.
  3. The script lists each file with `same` or `DIFFERS` for each of the two, counts them, and exits 1 on any difference or if the count is not the step's. Its output goes into the as-built file.
- **Held by tests that are not touched:** the pinned hashes of `tests/scenario_130/part_f.rs`, the boats tests' games, `pack_from_source`, `door`, the web check and `table/check_screen.py`.
- **If equality 1 fails for a file** (a generator put a row somewhere the format cannot), the developer stops. That file is the architect's to rule: it stays as it is, or the file below is reordered in a pull request of its own.

## Which files convert: 58

All 55 files a live generator makes and marks "Do not edit by hand", and the three bases a generator makes. Paths and names do not change.

| Folder | Files | Built on | States only |
| --- | --- | --- | --- |
| `coral-sea` | `base_coral_sea_oilers` | `base_coral_sea` | `name`, `ship_classes`, `class_sides`, task force 6, ships 54 and 55 |
| `coral-sea` | `base_coral_sea_both_oilers` | `base_coral_sea_oilers` | `name`, `ship_classes`, `class_sides`, task force 7, ship 56 |
| `coral-sea` | `01a` to `01d`, `02a` to `02d`, `03a` to `03d` (12) | the base of its number | `name`, `[[command]]`; file d, `name` alone |
| `five-days` | `base_eastern_solomons_oilers` | `base_eastern_solomons` | `name`, `ship_classes`, `class_sides`, task force 6, ships 44 and 45 |
| `five-days` | `01a` to `01d`, `02a` to `02d` (8) | the base of its number | `name`, `[[command]]`; file d, `name` alone |
| `midway` | `01a` to `01d` (4) | `base_midway` | `name`, `[[command]]`; file d, `name` alone |
| `table-hunger` | 12 | the file `scripts/table-copies/make_copies.py` lists as its source | `ground` |
| `table-boats` | 16 | the file `scripts/table-boats/make_files.py` copies | `submarine_classes`, `submarine_torpedoes`, `submarines`, `[side]`, `[[submarine]]`; `ground` too in the four built on a file of the live ground figures |
| `table-rear-bases` | 3 | the file of the same name in `table-boats` | `name`; `trial`; base 3's `supply_t`; for task forces 30 and 32 to 39, `at_base`, `home_port`, `available_from`; `[[island]]` rows: island 3, and in `07_full_slice_submarines.toml` islands 1 and 2 before it (amended on Nov 27, 2026 for decision 0232's step 3, which adds them before step D: `as-built/0232-a-plan-the-commander-writes.md`, item 5a) |

An overlay's header says what it is built on and what it states, and keeps what its generator's header said of the scenario, less the sentences about the script. Notes the generators wrote beside a changed line go beside that line in the overlay.

## Which files stay as they are: 205

By rule, with the rule that keeps each:

- **Named by their bytes, 44.** The frozen copies (`step2-frozen`, 5). The files of a proof, held by its script's `--check` or its report (`orders-inside-proof` 4, `second-proof` 12, `stage-2b-proof` 5, `stage-3-proof` 3, `staff` 1, `stage-4-proof` 6). The scenarios a recorded game or a fixture's saved game names by hash and are not in those folders (`05_eastern_solomons`, `06_rabaul_guadalcanal`, `07_full_slice`, `43_keep_two_bases_fed`, both files of `a-limit-binds-what-it-names`, `table-submarines/129_the_patrol`) and the reports' batches name by hash (`04_savo_island`, and scenarios 5, 6 and 7 again). Eight of these 44 carry "Do not edit by hand" from a frozen proof's script and keep it.
- **Hand-written and whole, 161.** The other files directly in `data/scenarios` and the other single-scenario folders, and the three bases written by hand (`base_coral_sea`, `base_eastern_solomons`, `base_midway`). Nothing makes them from another file. A family written by hand as near copies (`a-force-followed`, `the-pattern-of-a-patrol`) may be rewritten as overlays by whoever next changes it, under this record's proof; it is not part of this programme.
- **Scenarios 7 and 8.** They share 2,612 lines and stay two files. Scenario 7 is named by recorded games; scenario 8 is the fourteen-day slice carried on from 21 August, and since scenario 7 was lengthened to 39 days it differs from it by rows removed and reordered, which the format does not do.

The eight "Do not edit by hand" files of frozen proofs counted in the 63 (`orders-inside-proof` 4, `stage-3-proof` 3, `staff` 1) are among the 44. So 63 is 55 that convert and 8 that stay.

## The generators

Nine scripts in six folders, 2,754 lines: `scripts/coral-sea/` (three), `scripts/five-days/make_files.py` and `make_oiler_files.py`, `scripts/midway/`, `scripts/table-boats/`, `scripts/table-copies/` and `scripts/table-rear-bases/`. They are deleted, each in the pull request that converts the last file it makes, and not before. `scripts/five-days/read_first_run.py` stays, frozen, and so do the two `make_arrangements.py` of the proofs. `scripts/README.md` loses their line under "Live" and gains one line saying they were retired by this record and are in git before that commit. Records before 0233 that cite them are not edited.

Their `--check` said a file was its source with stated changes. In its place: the loader, by construction; statements R1 to R5 of `tests/scenarios_built_on_another.rs`; and each test that ran a script with `--check` loses that call.

**Statements about the text of a converted file** in the tests of scenarios 103, 104, 105, 106, 114, 119, 120, 121 and 123, the four `boats_in_...` tests, `tests/common/boats_in_a_game.rs`, `table_rear_bases_files.rs` and `estimates_never_evidence.rs` are read as follows, and a statement that fits none of the five goes back to the architect:

- (a) **Of a key, a row or a table** (the file has `days = 5`; it has no `[[command]]`; no row has `available_from`): read from the resolved scenario, through one helper in `tests/common`.
- (b) **Of a header** (the header says the game begins at 1800 on 5 May; it names the class that is estimated): read from the headers of the file and of every file it is built on, joined, nearest first, through one helper.
- (c) **That two files are the same but for X,** byte for byte or line for line: withdrawn as a statement about bytes, and held as "the overlay states X and nothing else" (R3).
- (d) **That a header names its script, or that the script's check passes:** withdrawn.
- (e) **A test that reads the file's text, alters it and loads it with `from_text`** (scenarios 104, 106, 120, 121 and 123): it alters the resolved text (item 12), through the same helper as (a), and loads that with the file's own path. What it alters and what it expects stand.

`crates/sim-cli/src/estimates.rs` reads a scenario's text itself and asks for `ship_classes`, which an overlay need not state. In step B it reads the class files of the resolved scenario through `sim-data`, and looks for the mark's words in the headers of the chain, joined as in (b): decision 0207's rule 12 is then "in its header or the header of a file it is built on" (statement L15).

No expected number is touched by any of the five.

## The split of `scenario.rs` (step A)

`crates/sim-data/src/scenario.rs` becomes the folder `crates/sim-data/src/scenario/`: `mod.rs` (`Scenario`, `LoadOptions`, the ways in, the accessors, the days), `file.rs` (`FileList`, `ScenarioFile`), `build.rs` (`parse_via`, whole), and one file for each table's types and the helpers that serve only them (`bases`, `ships` with task forces, `air` with replacements and arrivals, `land` with formations and skills, `submarines`, `orders` with the small enums of an order, `beliefs` with contacts, estimates and the believed lists, `objectives`, `commanders` with sides and doctrine, `islands`, `supply` with lines and men). The 5,934 lines of tests stay unit tests, since they use private items, in `scenario/tests/`, one file for each table and a `mod.rs` for what they share.

Every item moves whole. Nothing inside an item changes but what the move forces: `use` lines, `mod` lines, `pub(super)` or `pub(crate)` on an item now reached from another file, `super::` paths, and what `cargo fmt` does to a line whose indent changed. `parse_via` is not cut in this step. The public names of `sim_data` do not change.

## The order of pull requests

| Step | Class | What | Waits for |
| --- | --- | --- | --- |
| A | 2 | `scenario.rs` split by table, a pure move | nothing |
| B | 2 | The loader reads `built_on`; `data_files`; `sim-cli resolve`; `scripts/overlay-proof.sh`; the statements' tests on invented files; the rulebook section, the glossary, `layout.md`, `data-rules.md`, `commands.md` | A |
| C | 2 | `coral-sea`, `five-days` and `midway` converted (27 files); their six scripts deleted | B |
| D | 2 | `table-hunger`, `table-boats` and `table-rear-bases` converted (31 files, each chain reaching C's files); their three scripts deleted | C |
| E | 2 | `parse_via` cut into one function for each table, in the order it has now; no error text, hash or baseline moves | B; beside C and D, since it touches `sim-data` alone and they touch none of it |

One developer at a time holds `crates/sim-data/src/scenario*`: A, then B, then E. C and D are one after the other because the tests of scenarios 104 and 105 read both families.

## The briefs' essentials

Every brief: work in a worktree; read this record and the doc comment of `tests/scenarios_built_on_another.rs`; change no statement and no expected number; a golden file, pin, baseline or recorded game that moves is a stop, not a remake; the three checks before a commit; the class and this record's number in the pull request's description; what was built goes in `docs/decisions/as-built/0234-a-scenario-built-on-another.md`.

- **A.** Files: `crates/sim-data/src/scenario.rs` to `scenario/` as above, and nothing else. Steps: move the tests out first, then the types, building after each. Checks: the names of the tests of `cargo test -p sim-data -- --list`, read by their last part and sorted, are the same list before and after; a throwaway count shows the tokens of the old file and of the new files together, with white space and commas taken out, differ only in `use`, `mod`, visibility and `super::`; `cargo test --workspace` with nothing regenerated. Must not change: any line inside a function or a test, any public name, any error text.
- **B.** Files: `crates/sim-data/src/scenario/` (a new `built_on.rs` for reading the chain and laying one table over another, and the few lines of `parse_via`'s caller that hand it the result), `crates/sim-cli` (the `resolve` command), `scripts/overlay-proof.sh`, `tests/scenarios_built_on_another.rs`, named as the package names its test files that day, `crates/sim-cli/src/estimates.rs`, the helpers `common::resolved_text(path)` and `common::headers(path)` in `tests/common`, the loader's tests of statements L1 to L16 on invented files held in a `Pack`, and the documents the table above names. Steps: `Scenario::resolved_text` and `Scenario::built_on_chain` are public functions of `sim-data` and `sim-cli resolve` only prints them; a file with no `built_on` takes the old path, `toml::from_str` on its text, so its errors keep their lines; an overlay is laid as tables and deserialised from the result. Checks: the proof of step B above; `sim-cli resolve` of a file with no `built_on` parses back to that file's table. Must not change: `Scenario`'s existing public functions, `data_files` for a file with no `built_on`, any version.
- **C and D.** Files: the scenario files of the step, the scripts deleted, `scripts/README.md`, `docs/commands.md`, `table/scenarios.json`'s note (its paths stand), and the tests named above, changed only by readings (a) to (e). Steps: write each overlay from its generator's own account of what it changes; run `scripts/overlay-proof.sh`; only then delete the scripts and the `--check` calls. Checks: the proof's count is 27 for C and 31 for D, every line `same`; R1 to R5 pass; `web/check.mjs` and `table/check_screen.py` pass. Must not change: a path, a name of a scenario, any file outside the step's folders under `data/`, a file of the 205.
- **E.** Files: `crates/sim-data/src/scenario/build.rs` and new files beside it. Checks: every test of the workspace with nothing regenerated. Must not change: the order in which tables are built and checked, so that a file with two faults reports the same one.

## What it leaves out

- A player's saved game of one of the 58 files, made before its conversion, is refused afterwards, since the file it names has changed: as after any change to a scenario file. No recorded game is of these files.
- The orders of a pairing are still told three times in the Coral Sea (files 01, 02 and 03 of a letter) and twice in the five days, since a file is built on one file.
- The proofs' files, the frozen copies and scenarios 7 and 8 are not reduced. Most of the lines that remain are theirs.
- The hand-written families of near copies are not converted here.
- Nothing is decided on generating a proof's arrangements; a future proof may write its arrangements as overlays from the start.

## Questions for the designer

None. The saved games of players at the table, first line of "What it leaves out", is told to him in the report of what was merged.
