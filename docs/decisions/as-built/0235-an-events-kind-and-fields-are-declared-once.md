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

## Built: PR 2 (the reading by type, for the commonest types)

Class 2. What was done: `crates/sim-core/src/field_value.rs` (new, with its `mod` line in `lib.rs`) holds `FieldValue` (`Int`, `Num`, `Bool`, `Hex`), the trait `ReadField` and `Fields`. `event_kinds!` gained an optional `[by_hand]` after a kind's word, `KindInfo.by_hand` now follows it, and the macro makes `EventKind::fields(&self, &UmpireKey) -> Option<Fields>`: `None` for a kind marked, otherwise each field in the order declared, named by its own name and read through `ReadField` (a kind with a field of a type without a reading and no mark does not compile). In `sim-cli`'s `export/events.rs`, `describe` is the hand arms and then `_ => from_fields(kind.fields(key)...)`, which panics with the kind's word if a marked kind has no arm; `HAND_ARMS` (test-only) lists the words of the hand arms and the test `hand_arms_are_the_marked_kinds` holds it equal to the marked kinds of `EventKind::KINDS` (ruling 6). `Display`, `entities()`, `seen`, the kind tables, `describe_side` and `names.rs` are not touched. The marks and the deletions were made by scripts (`mark.py`, `delete_arms.py`, kept in the scratchpad beside `classify.py`); the enum was not retyped, only the 179 first lines changed.

- **Table of types and readings (ruling 3).** `Fixed` as a quantity (`Num`); `u32` and `u8` as a whole number (`Int`); `bool` as a flag, always written (the kinds that leave a false flag out stay by hand); `Hex` as `[q, r]`; every id of the six with a public number (`TaskForceId`, `BaseId`, `AirGroupId`, `LandUnitId`, `ShipId`, `SubmarineId`) as its number, by one impl over `EntityId`, so the three opaque ids are not read. Nothing else has a reading in this pull request.
- **P1, exports.** 133 files before (merge base, same tree as the branch before any edit) and after; the sorted `sha256sum` lists compared, `diff` printed nothing. Hash of the sorted list, both: `4c682911b86252b60e9372ede6566b7a2f178dcd52499e9ff26e6591f90c02f7`.
- **P2, kinds seen.** 145 of the 179 kinds appear in the "before" exports; all 61 that lost their mark are among them.
- **Ruling 2 (c), the script** (`classify.py`, run over the old arms from `git show HEAD:` and the "before" folder). An arm is deleted only if it is exactly the chain `.id/.int/.num/.hex/.flag("name", *name)` over every field, in the order declared, with the pattern binding every field and nothing else: so it has no `match`, `if`, `is_empty`, `*_when_*` or other call, hence no untaken branch, and each field's type is one of the eleven. It printed:

```
fired kinds over the before exports: 145 of 179
  61  DELETED: fired, plain chain of the eleven types, no branch
   2  KEPT: fired, a branch or a conditional key (_when_)
   2  KEPT: fired, a branch or a conditional key (if , Some()
   2  KEPT: fired, a branch or a conditional key (if , else)
   1  KEPT: fired, a branch or a conditional key (if , match , is_empty, Some()
   1  KEPT: fired, a branch or a conditional key (if , match , is_empty, Some(, else)
   5  KEPT: fired, a branch or a conditional key (match )
   7  KEPT: fired, a branch or a conditional key (match , Some()
   7  KEPT: fired, other helper or object (cargo_named, nested objects, commands)
  57  KEPT: fired, type without a reading in this pull request
  21  KEPT: no export fires it (not plain)
  13  KEPT: no export fires it (plain)
 deleted arms with a branch keyword: []
```
- **Arms: 61 deleted, 118 kept.** Kept: 57 hold a field of a type with no reading yet (mostly `Side`, 41 kinds; PR 3); 24 fired kinds have a branch, a conditional key or an object of their own and no branch was shown taken both ways, so none is argued away; 34 are fired in no export (13 of them plain and 21 not), which keep their arms by ruling 2. No arm was deleted on the ground that every side of its branches was taken: none with a branch was deleted.
- **P3.** Changed: `crates/sim-core/src/events.rs`, `crates/sim-core/src/field_value.rs` (new), `crates/sim-core/src/lib.rs` (one line), `crates/sim-cli/src/export/events.rs`, this file. **P4:** no line of `entities()` or either `Display` differs (the diff of `events.rs` is the macros, the imports and the 179 first lines).
- **P7.** `cargo fmt --all -- --check`: clean. `cargo clippy --workspace --all-targets -- -D warnings`: clean. `cargo test --workspace --profile checked`: 5,868 passed, 0 failed, 24 ignored, including the new test. (The main it was built on has moved since PR 1's count of 5,869 and 23; the ignored are the long ones the tests mark.) `UmpireKey` is still reachable only through `fields`'s parameter, which `sim-agent` and `sim-staff` never call.
- **Not shown here:** P5's before-and-after list of test names and P6 (CI's `web` job); the pull request's CI does those.

## Built: PR 4 (the agent's name of a kind)

Class 2. `SideEventKind::tag()` is added in `crates/sim-core/src/side_events.rs` as one new `impl` (one exhaustive `match`: a whole event's word is its `EventKind`'s `tag()`, each cut form's is the snake case of its variant's name). `sim-agent`'s `kind_name` in `common.rs` is now `kind.tag().to_string()` and no longer formats the event. `seen` and `project` are not touched. The test `crates/sim-agent/tests/kind_names.rs` keeps the old `Debug`-based way for the test alone and holds the two equal over every event each side is given in `04_savo_island` (seed 1), `05_eastern_solomons` (seed 1) and `128_submarines_against_warships` (seeds 1 to 60: the live file is one night with a boat that fires by chance, so several seeds are played for her salvo and the hunt; with the file's one seed it gives 1 event). The cut forms met: the surface fight's (engagement begun and ended, sighting roll, force surprised, salvo, salvo against), the air's (air attack, air attack against, air fight, anti-aircraft fire, base attacked, base attacked against, search attempt) and the boats' (own submarine fired, own submarine hunted, salvo from submarine, submarine hunted by); the test asserts three of the boats' are met. The agent's golden files and the recorded games pass unchanged.
