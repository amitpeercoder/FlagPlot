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

## Built: PR 3 (the other types, the documents)

Class 2. What was done: `crates/sim-core/src/field_value.rs` gained readings for the further types that have one everywhere, and `FieldValue` four variants (`Text`, `Str`, `Hexes`, `Ids`; it is no longer `Copy`). The words that were `match`es in `sim-cli`'s `export/names.rs` moved into `sim-core` as `side_word`, `weapon_word`, `track_word`, `end_reason_word`, `detach_reason_word`, `system_word`, `system_state_word`, `loading_loss_word`, `rehearsal_loss_word`, `roster_left_word`, and, whose only user was a deleted arm, `force_fatigue_word`, `land_fatigue_word` and `entry_wait_word` (ruling 4: the same bytes; `names.rs` calls them, and the six of its functions whose only users were deleted arms are removed). `describe` has a doc comment of its own, the misplaced text that sat above `cargo_named` ("The match names every variant") is gone from it, and the section comment for decisions 0095 and 0097 that went with its first arm in PR 2 is dropped: its arms (`LoadingLost`, `RehearsalLost`, `LeftRoster`) are deleted in this pull request, so nothing is left for it to head. The marks and the deletions were made by `apply.py` from the classification of `classify.py` (both in the scratchpad beside the PR 2 scripts); the declaration was not retyped, only the mark came off 48 lines.

- **Table of types and readings (ruling 3), added in this pull request.** The ids with a public number are 14, not six, and the PR 2 impl over `EntityId` already reads all of them (`ShipId`, `ShipClassId`, `AircraftTypeId`, `TaskForceId`, `AirGroupId`, `PilotId`, `BaseId`, `LandUnitId`, `SubmarineId`, `SubmarineClassId`, `SubmarineTorpedoId`, `IntentId`, `FormationId`, `InterruptId`); the as built of PR 2 said six, naming the ones then used. New: `ContactId`, `OrderId`, `EventId` (their number, read with the key, decision 0086); `PlanId` (its number); `Side` (`allied`, `japanese`); `Vec<T>` of the public ids (a list of numbers); `Vec<Hex>`; `EntityRef` (its `Display`); `Option<T>` of any type that has a reading (the value, or nothing: every field of an `Option` type in a kind that is still by hand was checked, and each writes the value or leaves the key out); thirteen enums read by their own `name()` (`AirFirer`, `AirTarget`, `CommandKind`, `ExperienceSource`, `FieldworksChange`, `FuelOrderEnd`, `GroundGrade`, `Job`, `MarchStop`, `PlanState`, `ShipKind`, `SortieKind`, `SubmarineLeftReason`); and thirteen read by a word written in `field_value.rs` (`Side`, the nine others of `names.rs` above, the two fatigue changes and `EntryWait`). Types left without a reading because they are written two ways or as an object: `Cargo` (left out for supply, `"none"` in one kind), `RejectReason`, `CommandRef`, `HoldTarget`, `SupplyUse`, `Vec<ShipSystem>`, `Issuer`, `AirGroupLocation`, `AsOf` and the objects of commands and claims.
- **P1, exports.** 133 files before (the merge base, built first) and after; the sorted `sha256sum` lists compared, `diff` printed nothing. Hash of the sorted list, both: `4c682911b86252b60e9372ede6566b7a2f178dcd52499e9ff26e6591f90c02f7`.
- **P2, kinds seen.** 145 of the 179 kinds appear in the "before" exports; all 48 that lost their mark are among them.
- **Ruling 2 (c), the script.** `classify.py` over the old arms (from `git show HEAD:`) and the "before" folder. An arm goes only if it is exactly one call for each field, in the order declared, under the field's own name, with the pattern binding every field and nothing else, each call the one reading of the field's type; the only branch an arm may hold is the reading of an `Option` field (`Some` writes the field, `None` writes nothing, or `id_when_some`), which is the type's one reading and which the script recognises by its exact shape. Printed:

```
hand arms at the start of this pull request: 118
  48  DELETED: fired, plain chain of readable types, no branch
   7  KEPT: fired, arm has other calls
   3  KEPT: fired, call for after_attack differs from its reading
   3  KEPT: fired, call for base differs from its reading
   2  KEPT: fired, call for chance differs from its reading
   1  KEPT: fired, call for ditched differs from its reading
   1  KEPT: fired, call for hits_on_men differs from its reading
  13  KEPT: fired, not a plain chain
   1  KEPT: fired, pattern differs from the fields
   1  KEPT: fired, type CommandRef has no reading
   1  KEPT: fired, type HoldTarget has no reading
   1  KEPT: fired, type RejectReason has no reading
   1  KEPT: fired, type SupplyUse has no reading
   1  KEPT: fired, type Vec<ShipSystem> has no reading
   7  KEPT: no export fires it (not plain)
  27  KEPT: no export fires it (plain)
Option fields of the deleted arms (events of the kind / events that wrote the key):
  AirAttack              recognised     Option<AircraftTypeId>    435 /    329  both sides taken
  MenFed                 base           Option<BaseId>       2915 /   2771  both sides taken
  AirGroupRebased        pilots         Option<u32>             3 /      0  ONE SIDE ONLY
  MarchArrived           base           Option<BaseId>         58 /     24  both sides taken
  StationKept            knots          Option<Fixed>           3 /      3  ONE SIDE ONLY
  AircraftFireFought     task_force     Option<TaskForceId>     14 /     14  ONE SIDE ONLY
```

  Three of the 48 hold an `Option` branch that the exports took one way only (`AirGroupRebased`, `StationKept`, `AircraftFireFought`). They go because the branch is exactly the type's one reading and ruling 2 (c) allows that; if the architect reads (c) as asking for both sides of every branch, those three take their mark back and their arms with them, and the numbers below are 45 and 73. The script's output is kept as `ruling2c.out` in the scratchpad.
- **Arms: 48 more deleted, 70 left** (from 179 at the start of the record, 118 at the start of this pull request). Of the 70, 36 hold a field of a type with no reading or an arm of their own shape (an `if`, a `match` on more than a field, an object, a key whose name is not the field's, a flag left out when false), and 34 are fired in no export (27 of them plain) and keep their arms by ruling 2. `describe` is 1,138 lines, from 1,686 at the start of this pull request and 2,594 at the start of the record; `export/events.rs` is 1,612 lines.
- **Visibility still decided.** A dummy variant `DummyProbe = "dummy_probe" { side: Side }` added locally gave E0004 at `events.rs` (`entities()`, `Display`) and at `side_events.rs` (`seen`, `first_known_round`, `fight_round`, `count_from_known_round`, `whole_may_have_hidden_causes` and `project`); removed. The new section of `traceability.md` names those.
- **Documents.** The section "Adding a kind of event" in `docs/engineering/traceability.md` (the places as they now are, and the table of types) and a clause in the `sim-core` row of `docs/engineering/layout.md`.
- **P3.** Changed: `crates/sim-core/src/events.rs` (marks only), `crates/sim-core/src/field_value.rs`, `crates/sim-cli/src/export/events.rs`, `crates/sim-cli/src/export/names.rs`, the two documents and this file. **P4:** no line of `entities()` or either `Display` differs; `seen` and the kind tables are untouched.
- **P7.** `cargo fmt --all -- --check`: clean. `cargo clippy --workspace --all-targets -- -D warnings`: clean. `cargo test --workspace --profile checked`: 5,868 passed, 0 failed, 24 ignored (as PR 2; no test added), including `hand_arms_are_the_marked_kinds`. `UmpireKey` is still reachable only through `fields`'s parameter.
- **Not shown here:** P5's before-and-after list of test names and P6 (CI's `web` job); the pull request's CI does those. PR 4 (`kind_name`) is another developer's and is not here.

## Built: PR 4 (the agent's name of a kind)

Class 2. `SideEventKind::tag()` is added in `crates/sim-core/src/side_events.rs` as one new `impl` (one exhaustive `match`: a whole event's word is its `EventKind`'s `tag()`, each cut form's is the snake case of its variant's name). `sim-agent`'s `kind_name` in `common.rs` is now `kind.tag().to_string()` and no longer formats the event. `seen` and `project` are not touched. The test `crates/sim-agent/tests/kind_names.rs` keeps the old `Debug`-based way for the test alone and holds the two equal over every event each side is given in `04_savo_island` (seed 1), `05_eastern_solomons` (seed 1) and `128_submarines_against_warships` (seeds 1 to 60: the live file is one night with a boat that fires by chance, so several seeds are played for her salvo and the hunt; with the file's one seed it gives 1 event). The cut forms met: the surface fight's (engagement begun and ended, sighting roll, force surprised, salvo, salvo against), the air's (air attack, air attack against, air fight, anti-aircraft fire, base attacked, base attacked against, search attempt) and the boats' (own submarine fired, own submarine hunted, salvo from submarine, submarine hunted by); the test asserts three of the boats' are met. The agent's golden files and the recorded games pass unchanged.
