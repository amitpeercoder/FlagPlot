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
