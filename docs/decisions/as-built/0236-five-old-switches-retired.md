# 0236, as built: Five old switches retired, and the rest ruled

## Amended on review (Nov 27, 2026, pull request 749)

An independent reading of the record as first written said step 1 was sound and step 2 must change. What changed, and why:

1. **Step 2 strikes seven sentences of the rulebook** (ruling 15, new), where the record said it changed no rulebook text. Each states what a game with no search or no surface combat figures does. Step 2's class is ruled again with the reasoning written out: class 2, because both are required keys of a scenario file, so the sentences describe a case no loadable game can reach.
2. **"No text changes" is withdrawn** (ruling 11). Seven setup errors and one refusal of the trial go with their checks; the reject reason of a strike contact age keeps its shape and its arm for `None`, which is in the log and the hash.
3. **Ruling 14 counted six statements and named seven; there are eight.** The eighth is the `bare` world of `estimate.rs`. The reading accepts all eight as vacuous by construction.
4. **`SideView::search_figures` and `surface_combat_figures` go plain** in step 2, with `bombardment_figures` (ruling 10).
5. **On the advice:** ruling 7 cites both limbs of the frozen README's exception; `RefData::default()` has two loader sites that are not tests, not three; step 1 waits on nothing and may be briefed beside decision 0234's step A; ruling 13 says its risk is unmeasured, makes the measurement step 2's first act and says what a developer does on a stop; and ruling 6 says its note is the code's comment reworded, not word for word.
6. The rulings on the other switches are numbered 16 to 20, where they were 15 to 19.

## The architect's ruling on step 1's stop: the export pins (Nov 27, 2026)

The developer stopped, rightly, on a pin the record did not name. `a_scenario_with_command_levels_off_exports_as_it_did` in `crates/sim-cli/tests/export_pictures.rs` pins the bytes of ten exports, and an export lists under `inputs` the hash of every data file its scenario names. For scenario 21 the hash beside `ground.toml` goes from `3b3353e3ccc0e0d7` to `edf695ac46e02951`, and that is the export's only difference. The record's proof missed this pin; it is the list's second fault over again (a file's hash kept somewhere the list did not look).

1. **It is the same kind as the three saves: the bytes of a data file, not a game.** No day hash, event, count or text of an export moves. Step 1 stays class 2. Decision 0233's item 11 is met: the pin is remade inside the class 2 pull request that changed the file it records, and shown to the architect.
2. **How it is remade, by the developer.** For each of the ten stems, `sim-cli export` is run on the commit the branch left `main` at and on the branch, and the two files compared. A stem whose scenario names no ground file must be byte for byte the same, and its pin stands. A stem that names one must differ only in the sixteen hex digits beside each ground file under `inputs`; its pin is then replaced with the value the test reports. Any other difference in any of the ten is a stop: the class has risen. One line is added to the test's doc comment in the form of those before it: "Remade again for decision 0236 (the two figures of replacement men written into the ground files): the exports of the scenarios that name a ground file change in that file's hash under `inputs` and nothing else: checked against the exports of" the base commit. The ten comparisons, stem by stem with `same` or the lines that differ, go in this file under step 1's "Built" section.
3. **The recorded games.** `sh scripts/regenerate.sh` is run. Its diff must be the `save.json` of `full_slice_five_days_sonnet`, `keep_two_bases_fed_sonnet` and `second_proof_week_sonnet`, each in the hash beside `ground.toml` only, and no transcript, baseline, golden file or line of `tests/golden/pinned_hashes.txt`. The two tests of `mcp.rs` then pass. Anything more is a stop.
4. **Any other pin of this kind** (the bytes of an export, a batch or a pack that lists its inputs) that fails is handled as in 2, with the same comparison shown here, and named. A pin of a day hash or of a game's events is not of this kind and is a stop.
5. **The record's proof is read with this added:** `crates/sim-cli/tests/export_pictures.rs` changes in the pins of the exports that name a ground file and in one line of a doc comment.

## Step 1: built (Nov 27, 2026)

Built as the record says, on the branch left from `main` at d4d1ff2d. Files: the three `...File` structs and the tests of ruling 9 in `crates/sim-data/src/reference.rs`; `refdata.rs`, `bombardment.rs` and `view.rs` of `sim-core`; the restated test of scenario 24; the 34 ground files; `export_pictures.rs` (below); the rulebook (chapters 4, 5 and 7), `data-rules.md`, the two READMEs and decision 0197's Status. Clauses struck beyond those named: none found in `docs/rules/` or the glossary; the "Missing gives" clauses of the four rows named and nothing else.

**The ten comparisons** of the architect's ruling, `sim-cli export` on d4d1ff2d and on the branch, the two files compared as formatted JSON:

- 01_transit: same
- 04a_night_action: same
- 05_eastern_solomons: same
- 21_landing: only `"hash": "3b3353e3ccc0e0d7"` to `"edf695ac46e02951"`, beside `ground.toml` under `inputs`
- 23_fight_ashore: the same single line
- 31_airfield_damage: the same single line
- 36_one_convoy: the same single line
- 43_keep_two_bases_fed: the same single line
- 50_neutralise_a_base: same
- 58_a_carrier_force_closes: same

The pins of the five that name a ground file are replaced with the values the test computes (21: `0x9be4_b1b5_35ea_6860`, 23: `0x3987_d6d3_a4a4_ac2c`, 31: `0x8d74_08a1_756c_620a`, 36: `0x973c_ddfc_7158_e6e8`, 43: `0x4081_388c_af5c_8774`); the five others stand. One line is added to the test's doc comment. No other pin of this kind failed.

**Checks**

- `git diff --numstat` on `data/`: 34 files, each `14 0`.
- `sh scripts/regenerate.sh`: run baselines, agent goldens, measure fixture, pinned hashes, batch fixture and submarine tables, no file changed; recorded games, three files, `save.json` of `full_slice_five_days_sonnet`, `keep_two_bases_fed_sonnet` and `second_proof_week_sonnet`, each 1 line, differing in the hash beside `ground.toml` and nothing else (checked by splitting both files at quotes: one token differs, `3b3353e3ccc0e0d7` to `edf695ac46e02951`); no transcript.
- `sim-cli hash data/scenarios/07_full_slice.toml` before and after: identical, 39 days.
- `sh scripts/compare-scenarios.sh` against d4d1ff2d: "254 runs compared, 254 the same with the hashes cut, 0 differ".
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --profile checked`: see the last lines of this section.
- `frozen_figures.rs` passes with `FROZEN_WITHOUT` untouched (part of the test run).
- `web/check.mjs`: not run, the module `target/web/module/sim_web_bg.wasm` is not built here (`scripts/web/build.sh` needs the wasm target). `table/check_screen.py`: not passed here, it stops with "Argument list too long" starting `node` in this environment, before any check; neither reads a file this step changed but the ground files by hash, and the manager should run them.

Last lines: `cargo fmt --all -- --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --locked --profile checked --no-fail-fast`: 318 test result lines, 0 failed.

## Step 2: built (Nov 27, 2026)

Built as rulings 10 to 15 say, on the branch left from `main` at 21f1a806. `RefData::search` and `surface_combat` are plain; `RefData` no longer derives `Default`; `SideView`'s three readers are plain; the readers in `sim-core`, `sim-staff`, `sim-trial`, `sim-cli` and the loaders of `sim-data` take them plainly. `load_ship_classes` and `load_ship_classes_marked` give the map (`BTreeMap<ShipClassId, ShipClass>`); their callers (`sim-cli/src/estimates.rs`, `oob-derive`'s test, three root tests) take the map. The seven setup errors and the trial's refusal are removed; `max` of `StrikeContactAgeOutOfRange` stays an `Option<u32>`, always `Some`, with its `None` arm of text. The rulebook's seven sentences are struck. The unit tests of `sim-core` build their `RefData` through `RefData::for_tests()`; `crates/sim-agent/tests/warnings.rs` reads the two frozen files through `sim-data`.

**The measurement of ruling 13 (first act).** With the two types changed, the helper added and the crate built, `cargo test -p sim-core`: 1,202 unit tests, 1,200 pass, 2 fail. **No test failed because a rule now acts in a world that gained the figures.** Both failures state the absent side, and neither is among ruling 14's eight; they are left failing on the branch, unrulled, for the architect:

1. `resolve::tests::world_new_checks_the_surface_combat_figures_and_class_toughness`. It expected `World::new` to refuse a `RefData` with surface combat figures and no search figures ("fights report through the search figures"). It got `Ok`: the world builds, since `search` is always there. No event differs; the failure is at `World::new`, line 3376 of `resolve.rs`. This is the statement of the check ruling 11 removes ("there are surface combat figures but no search figures"). Its other statements pass.
2. `resolve::tests::world_new_rejects_a_damaged_system_without_surface_combat_figures`. It expected `World::new` to refuse a world with a damaged propulsion system and no surface combat figures. It got `Ok`, as the world now has the figures (line 4623). This is the eighth setup text, below. The whole test states the absent side.

**Texts and statements met in the build that the record does not list** (ruling 11, last bullet; ruling 14, last sentence). The code could not compile with them, so each was taken out or made plain as the smallest change, and each is for the architect to confirm:

- `world.rs`: the setup error "{ship} has {system} {state} but there are no surface combat figures" (an eighth setup error beside the seven) and its block.
- `sim-trial/src/enemy.rs`: a second refusal, "the view has no search figures", at the `merge` read (the record names the other, "and so no merging distance"). `enemy::check` lost its `view` argument.
- `sim-data/src/scenario/build.rs`: the loader's "0 or more" arm of the message for a strike contact age out of range. `build_objectives` still takes its `has_surface_combat` flag, passed as `true`, with its refusal ("needs the surface combat figures") and its test untouched.
- `sim-cli/src/validate/mod.rs` and `batch.rs`: `round_minutes` is passed as `Some`, and `crippled_runs` as `Some`, so `take::Context`'s `None` refusal and the batch file's shape stand, never reached.
- Statements withdrawn beyond the eight: in `airstrike/tests.rs`, `able_to_fly_cases`, the last statement ("with `None` figures a deck that is out gives 10"; the function's figures argument is now plain); in `view.rs`, the vacuous `assert!(view.search_figures().is_some())`. `stragglers.rs`' `a_world_without_surface_combat_figures_does_nothing` is statement 4 of ruling 14.

**The eight statements of ruling 14,** all withdrawn: (1) and (2) in `resolve.rs` (the `r.search = None` pair, in `world_new_rejects_bad_air_group_setups`); (3) the last block of `the_age_order_is_checked_against_the_drop_age_and_stands`; (4) `a_world_without_surface_combat_figures_does_nothing`; (5) the `bare` world of `waters_are_refused_off_the_map_on_day_0_or_after_day_1_begins` (renamed, "with_no_search_figures_" out); (6) the second world of `a_force_without_ships_is_not_alerted` (renamed); (7) the `search = None` lines of `a_starting_contact_needs_ships_and_a_game_not_begun` (renamed); (8) the `bare` world's second statement in `estimate.rs`.

**The rulebook.** The seven sentences of ruling 15, each tag gaining 0236. Stragglers found by search and struck the same way: chapter 1's "A scenario with air groups must also name search figures..." (now "Every scenario names its search figures and its surface combat figures"); chapter 5's "Without the surface combat figures this step does nothing"; chapter 7's "The report error figures need the search figures: a world with them and no search figures is refused..."; chapter 10's three ("With no surface combat figures no ship is dead in the water", "...no system is hit", "A scenario with a boat needs the search figures"). The glossary has none.

**Checks.**
- `cargo fmt --all -- --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean.
- `cargo test --workspace --locked --profile checked --no-fail-fast`: every target passes but the two tests above.
- `sim-cli hash data/scenarios/07_full_slice.toml`: day 39 `e09e0205abb9a981`, the hash of `run_baseline/07_full_slice.txt` on `main`; no file under `tests/golden`, `run_baseline` or `recorded_games` changed.

## The architect's rulings on step 2 as built (Nov 27, 2026)

The measurement of ruling 13 is taken as made: no test failed because a rule now acts in a world that gained the figures. Step 2 stays class 2.

1. **`world_new_checks_the_surface_combat_figures_and_class_toughness`: one statement withdrawn, the rest stand.** The statement "surface combat figures without search figures are refused" (the comment "Fights report through the search figures, so they need them" and the `build` that follows it, expecting an error) is vacuous by construction: a `RefData` cannot be without search figures. It is removed. The comment "No figures at all is fine for a world with no fights" is no longer true of the world built and becomes "The fixture's figures are fine". Every other statement of the test stands untouched.
2. **`world_new_rejects_a_damaged_system_without_surface_combat_figures`: restated.** Its first statement, the refusal, is withdrawn as vacuous; the setup error it read, "{ship} has {system} {state} but there are no surface combat figures", is the eighth setup error and goes with the seven of ruling 11. Its second statement stands and is the whole test: a world whose ship has a degraded propulsion system is accepted by `World::new`. The test is renamed `world_new_accepts_a_damaged_system`.
3. **The two statements withdrawn beyond ruling 14 are accepted,** both vacuous by construction: the last statement of `able_to_fly_cases` (the function's figures are no longer an `Option`), and `assert!(view.search_figures().is_some())`. With rulings 1 and 2 here, twelve statements of the absent side are withdrawn in all: ruling 14's eight and these four.
4. **The second refusal of the trial,** "the view has no search figures", goes with the first, and `enemy::check` losing its `view` argument is accepted: rule 3 of chapter 9 has lost the clause already. Two refusals of the trial go, not one.
5. **The loader's "0 or more" arm** (`sim-data/src/scenario/build.rs`) is accepted as removed: the loader reads the search figures before any order, so the arm was never reached from a file. `sim-core`'s arm for `max: None` stays, as ruling 11 has it.
6. **`build_objectives` loses its flag.** A parameter that is always `true` is the switch this record retires, kept under another name. The parameter `has_surface_combat` goes, with the refusal "needs the surface combat figures" in `scenario/objectives.rs` and the sentence about it in `scenario/mod.rs`'s doc comment. The statement of that refusal in the loader's tests is withdrawn as vacuous (the thirteenth); a test left with other statements keeps them.
7. **`take::Context`'s `round_minutes` and the batch file's `crippled_runs` passed as `Some` are accepted as they are.** The second is the shape of a file the reports read, which this record does not move. The first is the validation tool's own type, not `sim-core`'s switch. Neither is a rule of the game.
8. **The rulebook's stragglers are within ruling 15's last sentence** and are accepted: chapter 1's sentence reworded to "Every scenario names its search figures and its surface combat figures", which is the sentence ruling 15 asked for; chapter 5's "Without the surface combat figures this step does nothing"; chapter 7's sentence on the report error figures needing the search figures; and chapter 10's three. Each states a game no file can make. Thirteen sentences are struck in all, not seven.
9. **The record is read with these counts:** eight setup errors and two refusals of the trial (ruling 11), thirteen test statements (ruling 14), thirteen rulebook sentences (ruling 15). The record's own text is left as the plan; this section is what was ruled on the build.
10. **Before the pull request is opened as ready:** rulings 1, 2 and 6 made; `cargo test --workspace` passes whole; `scripts/compare-scenarios.sh` finishes and says every scenario plays the same; and the list of test names before and after is in this file, differing only by the tests renamed or removed under rulings 14 here and above.

## Step 2: the rulings made, and the test names (Nov 27, 2026)

Rulings 1, 2 and 6 of the architect's section are made: the statement of surface combat figures without search figures and its comment are gone, and "No figures at all is fine..." reads "The fixture's figures are fine"; `world_new_rejects_a_damaged_system_without_surface_combat_figures` is `world_new_accepts_a_damaged_system` with its refusal dropped; `build_objectives` has no `has_surface_combat` parameter, no refusal, and its doc sentence and the test statement of that refusal are gone (the `ships_crippled` and `ships_out_of_action` check is collapsed into one `if`, as clippy asks).

**Test names, before (`origin/main`, 5,893 `#[test]` functions) and after (5,892),** compared as the sorted names of every `#[test]` function in the tree; the only differences:

- removed: `a_world_without_surface_combat_figures_does_nothing`
- renamed: `a_force_without_ships_is_not_alerted_and_without_combat_figures_nothing_runs` to `a_force_without_ships_is_not_alerted`; `a_starting_contact_needs_ships_and_search_figures_and_a_game_not_begun` to `a_starting_contact_needs_ships_and_a_game_not_begun`; `waters_are_refused_off_the_map_on_day_0_with_no_search_figures_or_after_day_1_begins` to `waters_are_refused_off_the_map_on_day_0_or_after_day_1_begins`; `world_new_rejects_a_damaged_system_without_surface_combat_figures` to `world_new_accepts_a_damaged_system`.

- `sh scripts/compare-scenarios.sh` against 21f1a806 (step 2, after the rulings): 254 runs compared, 254 the same with the hashes cut, 0 differ.
