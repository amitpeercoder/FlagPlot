# 0234, as built: A scenario built on another

## Built: step A (pull request 750)

`crates/sim-data/src/scenario.rs` became the folder `crates/sim-data/src/scenario/`, split by table as the record's "The split of `scenario.rs`" says, and its unit tests moved to `scenario/tests/`, one file for each table and a `mod.rs` for what they share.

- **Same tests.** The names of the tests of `cargo test -p sim-data -- --list`, read by their last part and sorted, are the same 304 before and after.
- **Token for token.** The tokens of the old file and of the new files together, with white space and commas taken out, differ only in `use`, `mod`, visibility and `super::` paths.
- **One brace.** `cargo fmt` dropped the braces of one closure whose indent changed; ruled harmless.
- **A misplaced doc comment kept.** The doc comment above `available_day` in `mod.rs` is the comment of `build_objectives`, which sat in the old file above the wrong item. The move is pure, so it is kept as it was; it is not mended here.
- No golden file, pin, baseline or recorded game moved.

## Built: step B

The loader reads `built_on`; nothing in `data/` is converted (that is steps C and D).

**Files.**

- `crates/sim-data/src/scenario/built_on.rs` (new): `Chain` reads the files from the one asked for, down through each `built_on`, through the same `Source` as every other file; `lay` puts one table over another by items 4 to 7 (`BY_ID` and `APPENDED` name the lists once); `named_for_the_first_folder` writes each file name a lower file states for the folder of the file asked for, the plain folder names followed by `..` struck out (item 8; `FILE_KEYS` is the list, to which a new file-naming key is added in the change that adds it); and the public `Scenario::resolved_text`, `Scenario::built_on_chain` and, for text that is not the file's own bytes, `resolved_text_of` and `built_on_chain_of`.
- `scenario/build.rs`: `parse_via` takes the old path (`toml::from_str` on the text, so a faulty file keeps its error line for line) for any text that has no top-level `built_on`; for one that has, it reads the chain, lays it and deserialises the table, and appends the paths of the files built on to `data_files`, nearest first, after every file listed before (item 11). `from_text` follows (item 13) since it calls the same function.
- `scenario/tests/built_on.rs` (new): L1 to L14 and L16 on invented files held in a `Pack` (the repository's own reference files are read from the disk), 15 tests.
- `crates/sim-cli/src/resolve.rs` and `main.rs`: `sim-cli resolve FILE`, the text on standard output and the chain on standard error.
- `crates/sim-cli/src/estimates.rs`: the classes come from the resolved scenario, and the mark's words are looked for in the headers of the chain joined, nearest first (decision 0207, rule 12); `chain_header` and `chain_header_of` give that text.
- `scripts/overlay-proof.sh`: the proof of steps C and D.
- `tests/scenarios_built_on_another.rs`: R1, R2 and L15 (the statements' text is unchanged); declared in `tests/all.rs` and struck from the exemption list of `every_test_file_is_declared_as_a_module`. `tests/common/mod.rs` gains `resolved_text(path)` and `headers(path)`. `tests/estimates_never_evidence.rs` also walks a file that states `built_on` and no `ship_classes`.
- Documents: the format is the rulebook's section "A scenario built on another" (`docs/rules/01-the-day-orders-and-what-a-side-knows.md`) and the record keeps one line naming it; the glossary's "Built on" and "Overlay"; `docs/engineering/layout.md`; `docs/engineering/data-rules.md` (rule 12's header); `docs/commands.md` (`resolve` and the proof script).

**Reading the statements.** L1 to L13, L14's first and second halves and L16 are tested in `sim-data`; L15 and R1 and R2 in `tests/`. R3 to R5 wait for steps C and D, as their statements say. `Scenario`'s existing public functions, `data_files` of a file with no `built_on`, and every version are as they were.

**Two small things the record did not say.**

1. The loader tells an overlay from a plain file by parsing the text as a table and looking for a top-level `built_on`; text that does not parse as a table, or has no such key, takes the old path, so its errors are the old ones. A mention of the word in a comment makes no overlay.
2. A chain error names the file asked for and, in its words, the file that could not be read or was read twice. A chain of more than 8 files is refused when the ninth is named.

**Checks.**

- `cargo fmt --all -- --check`: no output. `cargo clippy --workspace --all-targets -- -D warnings`: finished, no warning.
- `cargo test --profile checked --workspace`: every program `ok`, none failed (the `scenarios` program 3,093 passed and 20 ignored, `sim-data`'s library 305 passed, its integration program 15, `sim-cli`'s 431 and 167); no golden file, pin, baseline or recorded game remade. The loader's own 290 tests before this step pass with no expected text changed.
- `cargo test -p sim-data built_on`: the 15 tests of L1 to L14 and L16 pass; `cargo test -p scenario-tests --test scenarios -- scenarios_built_on_another:: every_test_file estimates_never`: 6 passed (R1, R2, L15, the guard and the two checks of decision 0207).
- `sim-cli resolve` of every one of the 263 files under `data/scenarios` (`tmp-` folders apart), read back as TOML, is the table of the file itself: 263 files, 0 differ.
- `sh scripts/overlay-proof.sh HEAD 1`, with `data/scenarios/midway/01d_midway.toml` temporarily written as an overlay of `base_midway.toml` stating its `name` alone (restored after; not committed): `data/scenarios/midway/01d_midway.toml resolve same run same`, `1 overlays, 0 differing`, exit 0. With the name changed one word: `resolve DIFFERS run DIFFERS`, `1 overlays, 1 differing`, `expected 2 overlays, found 1`, exit 1.
- The commands of "Before a commit": `run data/scenarios/01_transit.toml --events`, `hash data/scenarios/07_full_slice.toml` (day 39 `e09e0205abb9a981`, as pinned) and `oob-check data/oob` (no problems) were run.

## Built: step C

The 27 files of `coral-sea` (14: the two oiler bases and files a to d of 01, 02 and 03), `five-days` (9) and `midway` (4) are overlays, each at its own path and name, built on the file the record's table gives it: 38,829 lines before, 2,897 after. The three hand-written bases (`base_coral_sea`, `base_eastern_solomons`, `base_midway`) are untouched, so their headers still name the retired scripts. Nothing under `crates/` changed. The `table-*` families, `tests/golden` and `scripts/regenerate.sh` were not touched.

**Overlays.** A file a to c states `name` and its `[[command]]` rows verbatim from the flat file; a file d, `name`; the three bases, `name`, `ship_classes`, `class_sides`, the new task force and the new ships, each with the comment it had. The header of a file keeps what the generator's header said of the scenario less the sentences about the script, and names the file it is built on; a base keeps its own account and says what it states. No file says "Do not edit by hand".

**Proof,** `SIM_CLI=target/release/sim-cli sh scripts/overlay-proof.sh 21f1a806 27` (the commit the branch left `main` at): 27 lines, each `resolve same run same`, `27 overlays, 0 differing`, exit 0. `sim-cli hash` of `coral-sea/03a`, `five-days/01c` and `midway/01a`, before and after: day 5 `ba3aeefe6e1dc2ad`, `71283675c6410067`, `9c2c65c665e89b5e`, identical.

**Scripts deleted:** `scripts/coral-sea/` (3), `five-days/make_files.py`, `five-days/make_oiler_files.py`, `midway/make_files.py`. `scripts/README.md` and `docs/commands.md` say so.

**Statements.** R3 (the 27 rows copied into the test as data), R4 (file counts of the three table folders) and R5 are written in `tests/scenarios_built_on_another.rs`. R5 is `#[ignore]`d until step D, since the 31 files of the table's folders still say "Do not edit by hand"; run with `--ignored` it fails on exactly those, and the 8 files it names are the ones it finds besides. L15's set-up reads the resolved text of file 02a, which is an overlay.

**Tests changed, by reading** (no expected number or statement text touched):

- (a), a key, row or table: `text()` of scenarios 103, 105, 114 reads the resolved file; 104's `five_days_text`, its walk of every file for `for_play`, and 119's walk for `ground` likewise; 120's scan for `_oilers.csv`, the orders of the group and the points table (now read as a table); 121's orders; 114's copies of the table in `s21`.
- (b), a header: 103's "the file says X is left out", 105/114's `header_words`, 121's header words read `common::headers`.
- (c), "the same but for X": the byte comparisons of the four files (103, 105, 114) and 120's property of the oiler files are withdrawn, held by R3. Two statements that a copy is its source but for X (`boats_in_a_game` statement 7 and 119's copies) are read as tables, the source resolved, which is stronger.
- (d), a script's check: withdrawn for the five scripts deleted (103, 105, 114, 120 twice) and also for the table scripts `table-boats` (the four `boats_in_the_..._games` tests, and the field `script_files`) and `table-copies` (119, 121). Those two scripts copy the text of C's files and refuse an overlay ("its first key is not its name"), so their `--check` cannot pass once step C is built, whatever is done to them in D. `table-rear-bases`' check still passes and stays for D.
- (e), alter the resolved text: 104, 120, 121 alter the resolved text, with each literal in the resolved layout (keys in order, as `sim-cli resolve` writes them).

**Checks.** `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --profile checked --workspace`: every program ok, none failed (the `scenarios` program 3,088 passed and 21 ignored, R5 among them; `sim-cli`'s 431 and 167); no golden file, pin, baseline or recorded game remade. `table/check_screen.py` passes its 434 checks of the table's games and then stops where it passes the page to `node` as an argument ("Argument list too long", this machine); `web/check.mjs` needs `target/web/module`, not built here. Neither reads a converted file's text.
