# 0235, as built: An event's kind and fields are declared once

## Amended on review (Nov 27, 2026, pull request 745)

An independent reading of the record as first written said "must change". What changed, and why:

1. **Two tests read the enum from the source** (ruling 13, P3, P4, pull request 1). `event_kind_names` in `side_events.rs`'s tests and `event_kinds` in `tests/scenario_07_determinism.rs` look for the text `pub enum EventKind {`, which the macro removes, and the record as written forbade the mend. Both now read `EventKind::KINDS`, on the architect's word. The second is under `tests/`, so pull request 1 waits for items 2 and 3 of the programme. The trial of the macro had built the library only, and the record now says so.
2. **The gain was overstated.** 145 of the 179 kinds are fired in the 127 exports; 34 are not, and under ruling 2 keep their arms. The summary, the index and `docs/open-work.md` say about 110 to 120 arms and some 1,400 lines, where they said 140 and 1,700. No proof is added for the kinds not fired.
3. **Ruling 2 gains a third condition.** Identical bytes do not show a branch of an arm that no event took. An arm goes only if it has no such branch beyond the type's one reading, or every side of each is taken in the "before" exports, shown by a script.
4. **P3 names `crates/sim-core/src/lib.rs`,** for the `mod` line of a new file.
5. **Pull request 4 names its three scenarios.**

## Built: PR 1 (the declaration and the word)

Class 2. What was done: `event_kinds!` in `crates/sim-core/src/events.rs` wraps the enum; each variant's first line is `Name = "word" {`, the word being the snake case of its name (179 lines edited by script, the layout kept). The macro makes the enum, `EventKind::tag()` and `EventKind::KINDS` (a `KindInfo` of name, word and `by_hand`, every mark false); no `fields` yet. `describe` in `sim-cli`'s `export/events.rs` returns `Data` alone and its 179 literals are gone (by a script that unwraps each arm's `("word", data)`); `export/events.rs` and `export/sides.rs` take the word from `tag()`. A new test in `events.rs` holds each word equal to the snake case of its variant's name, all different, 179 of them (ruling 1). Ruling 13's two edits: the bodies of `event_kind_names` (`side_events.rs` tests) and `event_kinds` (`tests/scenario_07_determinism.rs`, with `EventKind` added to its `use`) read `EventKind::KINDS`. No new file, so no `mod` line. Two `let x = match ..; x` tails left by the unwrapping were folded to the match (clippy).

Diff: 6 files with this one, 1,233 lines added and 1,542 removed (most of both is `describe`, which rustfmt reflowed).

- **P1, exports.** 127 scenarios at their own seeds, plus `04_savo_island`, `05_eastern_solomons` and `24_shells_and_estimates` at `--seed 2` and `--seed 3`: 133 files, built from the merge base before and from the branch after. `sha256sum` of every file sorted, the two lists compared: `diff` printed nothing. Hash of the sorted list, both: `4c682911b86252b60e9372ede6566b7a2f178dcd52499e9ff26e6591f90c02f7`.
- **Visibility still decided.** A dummy variant `DummyProbe = "dummy_probe" { side: Side }` added locally gave E0004 at `events.rs` (`entities()`, `Display`) and at `side_events.rs` lines 851, 1814 and 2071 (`seen` and the kind tables); removed.
- **P7.** `cargo fmt --all -- --check`: clean. `cargo clippy --workspace --all-targets -- -D warnings`: clean. `cargo test --workspace --profile checked`: 5,869 passed, 0 failed, 23 ignored (the ignored are as before), including the new test and both edited tests. `sim-cli run data/scenarios/01_transit.toml --events` runs as before.
- **Not shown here:** P5's before-and-after test list and P6 (CI's `web` job); the pull request's CI does those.
