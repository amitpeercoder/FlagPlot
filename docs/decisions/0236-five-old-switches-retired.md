# 0236: Five old switches retired, and the rest ruled

Status: ruled by the architect on Nov 27, 2026, on Amit's two words of that day. Shown the list of 35 switches (`docs/validation/old-switches.md`), he said: "Retire the five no-cost switches", which are numbers 16, 24, 25, 28 and 35 of the list. Asked about the others, he said, as the manager relayed it: "Don't wait on me. Let the architect make whatever decision." So the retirement of the five is his word, and the ruling on every other switch is the architect's on his delegation, not his approval of each. Nothing is built.

Summary: Five switches go, none of which any scenario on file takes the off side of: the four bombardment figures, the two base hit shares for men and the airfield, and the men lost to a hit become figures every file must give; the two figures of replacement men leave the code and are written, as placeholders, into the 34 ground files; and `search` and `surface_combat` stop being optional inside `sim-core`. No game moves: every day of every scenario hashes as it did. Of the other 30 switches, the key for the staff's choice of a source is to be retired by a record of its own, class 3; command levels and orders inside a day stay until the record of the interface's design rules them; and 27 are kept.

Standing: Ruled by the architect on Nov 27, 2026, on Amit's word for the five and his delegation for the rest; class 2; to be built in two pull requests, the first after decision 0234's step A is merged

## Why

Each stage came in "on only where a scenario says, and off plays as before". The list measured what is left of that: 35 switches, most of which are simply "this rule has no figures here", which the frozen step 1 figures need for as long as they are kept. Five have an off side that no scenario, proof or recorded game plays, so a reader holds a second game in mind that nobody runs, and for number 28 two placeholder figures sit in the code, where the data rules do not look.

## What was weighed

- **Leave 16, 24 and 25 optional and say so.** Not done: every one of the 6 surface combat files, 2 air combat files and 34 ground files gives them, and the loader, `sim-core` and the rulebook each carry a clause for a file that does not.
- **Keep the two figures of replacement men in the code and call the constants placeholders.** Not done: a figure in code is in no file's header, no frozen copy and no saved game's list of files, so it can be changed with nothing to show it.
- **Write the two figures into the live ground files only.** Not possible: a frozen file has every key its live file has (`crates/sim-data/tests/frozen_figures.rs`), and the hand calculations of the scenarios with replacement men read the frozen files.
- **Give `SearchFigures` and `SurfaceCombatFigures` a default so `RefData::default()` still builds.** Not done: an all-zero set is a figure set in code that some rule would one day run on. `RefData` stops deriving `Default` (ruling 12).
- **Leave number 35 for a quiet week,** as the list recommended. Overruled by Amit's word. It is the one of the five with real work in it (see "Where the list was wrong"), so it is a pull request of its own.
- **Retire number 4 here.** Not done: it moves eight games and restates three hand calculations, which is class 3, and this record is class 2.

## Where the list was wrong

Found while this record was written, and corrected in the list:

1. **Number 16, 24 and 25: "only the loader's own tests read the off side".** Not so. `p_a_scenario_with_no_bombardment_no_men_and_no_estimate_is_unchanged` in `tests/scenario_24_shells_and_estimates.rs` takes the figures out of the frozen files and runs scenarios 6a and 4a. Its statement is restated here (ruling 8), by the architect, before any code.
2. **Number 28: "no game moves; saved games are remade in their file hashes".** True, and the list should have named them: three recorded games (`full_slice_five_days_sonnet`, `keep_two_bases_fed_sonnet`, `second_proof_week_sonnet`) hold the hash of `data/reference/ground.toml` in `save.json`. Each `save.json` changes in that one hash and in nothing else; no transcript changes.
3. **Number 35 was not "no cost" in work.** No game moves, but `RefData::default()` is called in about 50 places, three of them not tests (`sim-data`'s loaders fill a default piece by piece), and six places in `sim-core`'s unit tests state what a world without the figures does. Those statements are withdrawn here (ruling 14).

## What is retired

Numbers are the list's. Rulings are numbered through the record.

**16. The four bombardment figures of a surface combat file.**

1. `bombardment_minutes`, `bombardment_hit_chance`, `bombardment_reference_gun_damage` and `bombardment_false_claim_chance` are required keys. A file that lacks one is refused by the loader's usual words for a missing key. The words "give all four or none" go.
2. `SurfaceCombatFigures::bombardment` is a `BombardmentFigures`, not an `Option`. `SideView::bombardment_figures` keeps its name and, in step 1, its type: it is still `None` where a hand-built world has no surface combat figures. In step 2 it gives the figures plainly.

**24. The base hit shares for men and for the airfield.**

3. `[base_hit] men` and `[base_hit] airfield` are required keys of an air combat file. `"0"` remains a value a file may state, and is what the frozen file states for the airfield. The checks of range and of the four shares adding to 1 at most stand.

**25. `men_lost_per_base_hit`.**

4. It is a required key of a ground file. 0 remains a value a file may state (a hit on men then removes nobody), and the test of that stands. `World::base_hit_men_share` keeps its test for 0.

**28. The two figures of replacement men.**

5. `replacement_experience` and `replacement_share` are required keys of a ground file. `GroundFigures::REPLACEMENT_EXPERIENCE` and `REPLACEMENT_SHARE` are deleted; the test fixture `GroundFigures::FIRST` states 2 and a half itself.
6. The two figures are written into the 17 ground files of `data/reference` and their 17 copies in `data/reference/step1`, with the values the code held, `2` and `"0.5"`, directly after the line `cover_wear_per_hit`, one empty line before, in exactly these words:

   ```
   # Men who replace the lost (decision 0197, 12 and 14). Both figures are
   # placeholders and rest on no source. Until decision 0236 no file named them
   # and the loader read these two values from the code.

   # The experience of replacement men, from 0 to 4: a unit above it loses a
   # level as they join, one at or below it is not changed. Placeholder: 2, the
   # standard.
   replacement_experience = 2

   # The share of a unit's establishment that the men joined since its
   # experience last changed must reach for it to lose a level, above 0 and not
   # above 1. Placeholder: a half.
   replacement_share = "0.5"
   ```

   The note is the code's own comment on each figure (`crates/sim-core/src/refdata.rs`), less its clause on a file that names none. Every ground file is already marked `placeholder = true` and headed "PLACEHOLDER DATA", so the two are marked as the data rules ask.
7. **The frozen files.** Writing a figure into a frozen file is the one edit its README allows: "a rule that adds a figure adds it here too, with the value that leaves step 1's results as they were". Decision 0197 added the figure and, by reading it from the code, never wrote it; this writes it, with that value. No other byte of a frozen file changes. `data/reference/step1/README.md` gains one entry, "Added for decision 0236", saying so. **The frozen scenarios** (`data/scenarios/step2-frozen`, five files) and the files of the proofs are scenario files: they name figures files and hold no figure, so none lacks anything and none is touched. Every one of them was read for this: none holds a figures file of its own.

**The test of scenario 24, restated.**

8. `p_a_scenario_with_no_bombardment_no_men_and_no_estimate_is_unchanged` now states: with the figures of decision 0092 at the values at which they do nothing, the logs, the truth and the hash are the same as with the frozen figures. Its altered copy of the frozen figures changes: in `air_combat.toml`, `men = "0.2"` to `men = "0"`; in `ground.toml`, `men_lost_per_base_hit = 10` to `men_lost_per_base_hit = 0`, and `report_error_low` and `report_error_high` are dropped as now (switch 26, kept). The four bombardment figures are left as they are: no force in scenario 6a or 4a has the bombardment mission, and with them required there is no file without them to compare. In the control, the two assertions on `bombardment` being none and some are removed; the assertions that `base_hit_men` is 0, `men_lost_per_base_hit` is 0 and `report_error` is none in the altered copy, and that `base_hit_men` is not 0 in the frozen one, stand. The scenarios, the seeds and the three equalities stand. The test's name and its comment are changed to say "at the values that do nothing" for "taken out". No expected number changes.

**The loader's tests of the absent side** (`crates/sim-data/src/reference.rs`), by the architect's word under decision 0233, item 12:

9. `the_bombardment_figures_load_all_four_or_none` becomes `each_bombardment_figure_is_required`: the shipped file loads, and a file without any one of the four is refused with that key named. `files_without_the_figures_of_decision_0092_load_as_before` becomes `the_figures_of_decision_0092_are_required_but_the_report_errors`: a file without `men`, without `airfield`, or without `men_lost_per_base_hit` is refused with the key named; a ground file without the two report errors loads with no report error; the rest of the test, from "The rest of the ground figures are untouched" on, stands as it is. `the_replacement_figures_default_to_the_standard_and_a_half_and_are_checked` becomes `the_replacement_figures_are_required_and_checked`: the shipped file gives 2 and a half; a file without either is refused with the key named; 3 and `"0.25"` are read; and the four refused values stand, each set by replacing the key's line where the test now adds a line at the end. The test that takes the whole `[base_hit]` table away stands.

**35. `search` and `surface_combat` inside `sim-core`.**

10. `RefData::search` is a `SearchFigures` and `RefData::surface_combat` a `SurfaceCombatFigures`. `World::new` checks both, always.
11. Every place that handled the absence takes the figures plainly. Where the absent branch returned early or refused an order, the branch goes and what followed it stands. `RejectReason::StrikeContactAgeOutOfRange` keeps its `max` as it is, an `Option`, since it is in the log and the export: it is now always given. No event, reject reason, state field or text changes.
12. `RefData` no longer derives `Default`. The loaders of `sim-data` that filled a default (`reference.rs`, the class and type files; `scenario/build.rs`) read the search and surface combat files first and build the `RefData` with them. A public function of `sim-data` that gave back a `RefData` holding only classes or only types gives back the map it read; its callers take the map. The public names of `sim-data` otherwise stand.
13. A unit test in `sim-core` that built its world on `RefData::default()` builds it with `SearchFigures::from_search_toml()` and `SurfaceCombatFigures::from_surface_combat_toml()`, the fixtures the crate already has, through one helper in the crate's tests. `crates/sim-agent/tests/warnings.rs` reads the two frozen files through `sim-data`. **If a test's expectation then fails, the developer stops:** the figures changed what that test's world does, and whether the expectation or the world is restated is the architect's.
14. **Six statements of a world without the figures are withdrawn,** since no such world can be made: in `resolve.rs`, "`r.search = None` is an error" and "no air groups and no search figures is fine"; in `airstrike/tests.rs`, "with no search figures only a negative age is refused"; in `stragglers.rs`, `a_world_without_surface_combat_figures_does_nothing`; in `submarine_tests.rs`, the `bare` world of the test of suspected waters; in `surface/tests.rs`, the second world of `a_force_without_ships_is_not_alerted_and_without_combat_figures_nothing_runs` and the `search = None` lines of `a_starting_contact_needs_ships_and_search_figures_and_a_game_not_begun`. A test left with other statements keeps them and loses the words about figures from its name. Any other test that states the absent side, found in the build, is listed for the architect before it is touched.

## Its class

Class 2, both steps, and this record is its record. The architect reads each pull request before it is merged.

- **Step 1** is "a loader change with a stated equality to check": the same figures reach the same engine from the same or a longer file. No state, draw, event, order or tool kind; nothing in who knows what; no scenario file; no figure's value; no version.
- **The rulebook changes, and that does not make it class 3.** The clauses struck (below) each say how a file without a figure is read. They are rules of the file format, not of the day: no rule that resolves a day is added, changed or removed, and no game that could be loaded before plays differently. What class 3 would add for a change of this kind is Amit's word before the build, and he has given it for these five by number. The rulebook is still changed in the same pull request as the loader, as for any rule.
- **Writing a figure into 34 files is not "a figure tuned" and not "data settled".** The value in force is the one in force today; where it is written changes. The data rules gain one sentence (below), which is the architect's to write.
- **Step 2** is "a refactoring of a crate's `src` with every hash as it is", with six statements of unit tests withdrawn on the architect's word here.
- **It rises to class 3 and the build stops** if a day hash, a pin, a golden file, a transcript, a web or table check or a validation measure moves, or if a frozen file changes in any byte but the lines of ruling 6.
- **An independent review of this record is asked for before step 1 is briefed.** Class 2 does not need one; the list this record rests on was wrong in three places, so one is worth its cost.

## The proof that nothing is lost

- **By construction.** `ref_data` is not in `hash::hash_world`. Step 1 gives the engine the same `BombardmentFigures`, shares, men lost and replacement figures as before for every file on disk, since every file gave the first six and the code gave the last two at the values now written. Step 2 changes the type that holds figures the loader always supplied.
- **Step 1, shown by:** `cargo test --workspace` with nothing regenerated but the three saves; `tests/golden/pinned_hashes.txt`, every run baseline, every golden file and every transcript byte for byte as on `main`; `sh scripts/regenerate.sh` leaves a diff of exactly three files, the `save.json` of the three recorded games named above, each differing only in the hash beside `ground.toml`; `scripts/compare-scenarios.sh` against the commit the branch left `main` at says every scenario plays the same; `web/check.mjs` and `table/check_screen.py` pass; `frozen_figures.rs` passes with `FROZEN_WITHOUT` untouched; and `git diff --stat` on `data/` shows 34 files, each with the same 14 lines added and none removed.
- **Step 2, shown by:** all of the above with no file under `tests/golden`, `run_baseline` or `recorded_games` changed at all; and the names of the tests run (`cargo test --workspace -- --list`, sorted) are the same before and after but for the tests ruling 14 names.
- **What does move, and is told to Amit in the report of what was merged:** a saved game a player made at the table of any of the 147 scenarios that name a ground file is refused afterwards, since a file it names has changed, as after any change to a figures file.

## The rulebook, the glossary and the data rules

In step 1's pull request:

- **`docs/rules/04-surface-combat-and-damage.md`, Shore bombardment, Figures:** "and are given all or none; with none, no force bombards. A partial set is refused when the file loads." becomes "and every surface combat file gives all four: one that lacks any is refused when it loads." The tag gains 0236.
- **`docs/rules/05-air-operations.md`:** the sentences "A file that gives no `[base_hit] men` gives 0." and "A file that gives no `[base_hit] airfield` gives 0." are struck, and one is put after the second: "Every air combat file gives both shares; a share of 0 lands nothing there. [0236]"
- **`docs/rules/07-land-units.md`, Men who replace the lost, The figures:** "A file that leaves them out is read with the values below, so no existing file changes; neither file in the repository names them." becomes "Every ground file gives them (decision 0236)." The table's heading "Where not named" becomes "In `ground.toml`". In the chapter's table of figures, "Missing gives 0" goes from the rows of `men_lost_per_base_hit`, `[base_hit] men` and `[base_hit] airfield`, and "Missing gives 2 and 0.5" from the row of the replacement figures.
- **Any other clause** of `docs/rules/` on a file without one of the eight figures, found by searching for each key and for "Missing gives", is struck the same way and named in the as-built file.
- **`docs/glossary.md`:** no entry states the absent side of any of the five; nothing changes. "Hit on the airfield" keeps "only where ... the ground figures are loaded", which is switch 11, kept.
- **`docs/engineering/data-rules.md`** gains, after "Do not invent historical figures": "**No figure's value is in the code.** A figure is written in a data file, with its note on what it rests on. The loader gives a figure no value a file does not state. A figure a file may leave out is one whose absence turns its rule off, and the rulebook says so where it states the rule (decision 0236)."
- **`data/reference/step1/README.md`** and **`crates/sim-cli/tests/recorded_games/README.md`** each gain their paragraph (ruling 7; "Regenerated after decision 0236": three saves, the hash of `ground.toml` only, no transcript).
- **Decision 0197** gains one line in its Status pointing to this record's as-built file; its rule 14 is superseded in its second sentence.

Step 2 changes no rule and no rulebook text. The doc comments of `RefData` lose "`None` only in unit tests".

## The other 30 switches, ruled

Amit's second word gives these to the architect. Each ruling is the architect's, not Amit's confirmation.

15. **Number 4, `staff_chooses_supply_sources`: retire, by a record of its own.** It is class 3: eight games move, three hand calculations are restated, the state version and the rules label rise, 236 pins are remade, and the agent-played records of a closed proof are marked as of an older engine (decision 0228, "Two rules are kept for the choice"). It takes the full path of class 3: a plan by the architect with the restated hand calculations before code, an independent review, the rulebook in the same pull request. Amit's confirmation of that plan is replaced by the architect's ruling, on his word above; the report of what was merged tells him the games that moved. The record is the architect's next job on the switches, written after this record's two steps are merged, since all three touch the loader of scenario keys.
16. **Number 1, `command_levels`: not retired; ruled again by the interface's record.** Off is the game every recorded game, every closed proof and 255 scenarios play, and the proofs of stage 2b compare on with off. The trigger: the record of the interface's design that settles whether every game is played from a seat at a command. That record rules this switch in its own text, as class 3.
17. **The panic of scenario 29 with command levels on** (`crates/sim-core/src/pictures.rs:231`, "base:8 is not a command of the Allied side": a base that comes to be during a game has no command) **is an open defect and is mended first.** Command levels are not turned on in any scenario that does not have them on today, and number 1 is not retired, until it is. It is not left for that day: the eight files with command levels on are played from a seat, and whether a seat's own orders can form a base in one of them has not been ruled out. The architect is to be called for it as its own job: to find what decisions 0134 to 0138 say of a base formed in play, and to class the mend (class 2 if a record already says it, class 3 if a rule is wanting).
18. **Number 3, `orders_inside_a_day`: not retired; ruled again by the same record.** The engine does not read it; it is what a scenario says of how it is played at the seat, on in 56 files and off for the 39 days. The trigger is the same record of the interface's design, which settles how a day is played at a seat.
19. **Numbers 2, 5 to 15, 17 to 23, 26, 27 and 29 to 34: kept,** 27 switches, for the reasons the list gives each. A kept switch is looked at again only when the frozen step 1 figures are, since most are what those figures need.

So of the list's three "retire later" (1, 3 and 35), one is retired here and two wait on the interface's record; and of its five "retire", four are retired here and number 4 by its own record.

## The order of pull requests

| Step | Class | What | Waits for |
| --- | --- | --- | --- |
| 1 | 2 | Switches 16, 24, 25 and 28: the loader, the 34 ground files, `sim-core`'s `bombardment`, the tests of rulings 8 and 9, the documents | Decision 0234's step A merged, and the review of this record |
| 2 | 2 | Switch 35: `RefData`, its readers in five crates, `sim-data`'s loaders, the unit tests of rulings 13 and 14 | Step 1, and decision 0234's step B, since both hold `crates/sim-data/src/scenario/` |

One developer at a time holds `crates/sim-data/src/scenario*`, as decision 0234 has it: its step A, its step B, then this record's step 2, then its step E. Step 1 here touches `crates/sim-data/src/reference.rs`, which decision 0234 does not split, and nothing under `scenario/`.

## The briefs' essentials

Both briefs: work in a worktree; read this record, decision 0233 and `data/reference/step1/README.md`; change no statement and no expected number but as rulings 8, 9 and 14 say; a day hash, pin, baseline, golden file or transcript that moves is a stop, not a remake; the three checks before a commit; "class 2, decision 0236" in the pull request's description; what was built goes in `docs/decisions/as-built/0236-five-old-switches-retired.md`.

- **Step 1.**
  - *Files:* `crates/sim-data/src/reference.rs` (the three `...File` structs, the three places that match on an absent key, the tests of ruling 9); `crates/sim-core/src/refdata.rs` (`SurfaceCombatFigures::bombardment`, its `problem`, the two constants, `GroundFigures::FIRST`, the fixture `from_surface_combat_toml`, the doc comments that say "a file that names none"); `crates/sim-core/src/bombardment.rs` and `view.rs` (the two readers of `bombardment`); `tests/scenario_24_shells_and_estimates.rs` (ruling 8, that one test and the two lines that read `.bombardment`); the 34 ground files; the documents named above.
  - *Steps:* write the lines of ruling 6 into the 34 files first, with a script kept out of the repository, and run the tests: everything passes with the keys still optional. Then make the eight keys required. Then the tests of rulings 8 and 9. Then the documents. Then `sh scripts/regenerate.sh`.
  - *Checks:* the proof of step 1 above, each line of it reported in the as-built file with its output.
  - *Must not change:* any scenario file; any figure's value; any figures file but the 34, and those only by the 14 lines; `FROZEN_WITHOUT`; any test but the four named; any error text but "give all four or none"; `SideView`'s public names; anything under `crates/sim-data/src/scenario*`.
- **Step 2.**
  - *Files:* `crates/sim-core/src/refdata.rs` and every file of `sim-core` that reads `ref_data.search` or `ref_data.surface_combat` (about 35, tests among them); `crates/sim-data/src/reference.rs` and `scenario/build.rs` with the tests under `scenario/tests/` that read the two; `crates/sim-agent/src/queries.rs`, `crates/sim-staff/src/carrier.rs`, `crates/sim-trial/src/enemy.rs`, `crates/sim-cli/src/batch.rs`; `crates/sim-agent/tests/warnings.rs`; `tests/scenario_16_all_together.rs`, `scenario_55_a_carrier_force_from_an_end.rs` and `scenario_59_a_search_kept_up.rs`, named as the package names its test files that day, for the one or two lines each that unwrap the figures.
  - *Steps:* change the two types and let the compiler list the places; `sim-core`'s code first, then its tests through the one helper, then the other crates. List for the architect, before removing it, any test statement of the absent side that ruling 14 does not name.
  - *Checks:* the proof of step 2 above; a search of the workspace finds no `search: None`, `surface_combat: None`, `.search.as_ref()` or `.surface_combat.as_ref()` on a `RefData`.
  - *Must not change:* any event, reject reason, state field or text; `hash.rs`; any test's expectation (ruling 13); any file under `data/`; any version.

## What it leaves out

- The figures files a scenario may leave out (`air_combat`, `ground` and the rest, switches 7 to 14) stay optional in `RefData`. Only the two the loader always supplied change.
- The other figures a ground, air combat or surface combat file may leave out (switches 17 to 23, 26, 27 and 29 to 32) stay as they are.
- No figure is researched or moved. The two figures of replacement men are as much placeholders in the files as they were in the code.

## Questions for the designer

None. Told to him in the report of what was merged: that players' saved games of scenarios with a ground file are refused after step 1; that number 4's retirement will move eight games under a record he does not confirm, by his word; and that the list he read was wrong in the three places named above.
