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
