# 0235: An event's kind and fields are declared once

Status: ruled by the architect on Nov 27, 2026, on Amit's delegation of that day (decision 0233). It is the plan for item E of the refactoring programme in `docs/open-work.md`, measured before it was planned. Class 2 as planned here: no rule, state, draw, event kind, text, export format or version moves. Amended on an independent review the same day (`as-built/`). To be built. One thing it leaves out is class 3 and is Amit's to ask for: typed fields in what a seat is told of its events ("For the designer").

Summary: An event kind's word in the export and its fields are declared with the kind, in `sim-core`'s `EventKind`, through one `macro_rules!` there, and the export reads a kind's fields by their types where it wrote each by hand: about 110 to 120 of the 179 arms of `sim-cli`'s `describe` go, some 1,400 lines (the arms of the 34 kinds no scenario fires stay until one does), and a new kind whose fields are of known types needs no line in `sim-cli`. Every export stays byte for byte what it was. Of item E as first written only this part is worth its cost: the event's text, its entities, who may see it, the briefing, the batch record and the viewer's switches stay as they are, and the record says why of each. No rule of the game changes.

Standing: Ruled by the architect on Nov 27, 2026, on Amit's delegation; amended on an independent review the same day; class 2; to be built in four pull requests, the first after items 2 and 3 of the programme

## Why

Item E said the export should carry the engine's own words and typed fields so that the viewers stop switching on kinds, and that an event should be declared in one place in place of four or five. Measured on `origin/main` at decision 0233, much of that is already so, some of it should not be done, and one part is real.

## What was measured

| Where an event is written | Lines | What it is |
| --- | --- | --- |
| `sim-core/src/events.rs`, the enum | 1,851 | The declaration: 179 kinds, 815 fields of 82 types; eleven types (`Fixed`, `u32`, `Hex`, `bool`, `u8` and six ids) are 685 of the fields |
| the same, `entities()` | 628 | Chosen by hand. For about 100 kinds it is the kind's id fields in order; for the rest the order differs, a list or a place is added, or a field is left out |
| the same, `Display` | 2,195 | English, all by hand |
| `sim-core/src/side_events.rs` | 3,291, and 3,757 of tests | All decisions: `seen` (who may see a kind, whole, cut or not at all), the two tables of hidden causes, the rounds of a fight. Nothing mechanical |
| `sim-cli/src/export/events.rs`, `describe` | 2,594 of 2,952 | 179 arms, each the kind's word and its fields as JSON. In 153 the keys are the field names in the order declared, and in about 143 (1,750 lines) every value is also what its type always gives. The other 36 or so (830 lines) do something of their own: a key left out when false or 0, a figure worked out, a place written as `base` or `ship`, an object for a command or a refusal |
| `sim-cli/src/export/sides.rs`, `describe_side` | 290 | The cut forms of a side's events, by hand |
| `sim-cli/src/batch_record.rs`, `count` | 885 | What the measures count. About 120 kinds are named only to say nothing is counted |
| `sim-agent/src/briefing.rs` | 5,126 | A side's day read into its briefing's lines: adding up, not wording an event |

- **The export already carries the engine's words.** `EventOut.text` is the event's `Display` (three kinds of the staff's are reworded for the viewer, and a test says which), with `kind`, `entities`, `causes`, the breakdown and `data`. The viewer prints `text` and words no event itself.
- **The viewer's switches are not wording.** `viewer/index.html` has 110 `case` labels over 72 kinds in five switches. They rebuild the state from the events for replay files of versions 1 to 3, and draw trails, fights and raids; since version 4 the export carries the state at each step, and CI's `viewer/check.js` checks those steps against that same rebuild. They read `data`'s typed fields, which is what typed fields are for.
- **The table does not read the export.** It plays, and reads a seat's `events` tool: `event_kind`, `text` and `entities`, with no fields. It tests `event_kind` 57 times over 31 kinds and reads the engine's sentences by regular expression in about 20 places, because it is given no fields. That is a side's matter, not the export's.
- **A kind's word is written twice.** `describe` writes it as a literal, 179 times; `sim-agent`'s `kind_name` makes it from the variant's `Debug` output, formatting the whole event to read its first word. All 179 literals are the snake case of the variant's name.
- **The umpire's text and a side's text have one source already.** A side's text is the `Display` of its `SideEventKind`: the whole event only where `seen` ruled it whole, and otherwise a cut variant that has no field for what the side may not know; `sim-agent` then writes the side's own numbers into it.
- **Nothing committed holds an export.** `viewer/replays/` is git-ignored; CI exports every scenario and runs the viewer's check; `crates/sim-cli/tests/export.rs` compares an export with a run. The hash does not read events (`hash.rs` names none), so no golden file, pin, baseline, saved game or recorded game depends on anything here.
- **The macro was tried.** With the declaration below, `sim-core` copied out of the tree builds with all 179 kinds as they are, 78 of them read by type from the eleven commonest types alone, in the time it took before (19 s against 18 s, one run each). The trial built the library only, not its tests: two tests read the enum from the source and would have failed (ruling 13).
- **Not every kind is fired.** The 127 scenarios exported at their own seeds fire 145 of the 179 kinds; 34 are in no export (the submarines' kinds, a march, a rest, a force that enters and others), and eight are fired exactly once.

## What was weighed

- **Everything in one declaration: word, entities, fields and a class of visibility.** Not done. See the next three.
- **Who may see a kind, declared as a class on the enum.** Refused. `seen` is already the one place, exhaustive, with no wildcard, and a new kind does not compile until it is placed there. A class written beside the variant would turn some sixty one-line decisions into a default, which is what `CLAUDE.md` forbids ("decide it, do not default it"); it would be class 3, as anything in truth and belief is; and it would save nothing, since the cut kinds and the kind tables are decisions whatever is done.
- **`entities()` made from the field types.** Not done: four kinds in ten differ from their id fields in order or in what is named, so it is a choice, and it is already in the file of the declaration.
- **Text shared further between the umpire and a side.** Nothing to do, and one line drawn: see ruling 9.
- **Wildcards in the batch record and the briefing,** so a new kind need not be named there. Not done. Each is the question "does a measure count this?" and "is the side told this in its briefing?", and a kind that fails to compile there is that question asked.
- **The viewer's switches taken out.** Not this record's: they are the reading of old replay files and one half of CI's check of the steps. Whether versions 1 to 3 are still read is decision 0018's matter.
- **A procedural macro, or `serde` in `sim-core`.** Refused: either is a dependency, and `sim-core` depends on nothing.
- **Leave it all alone.** Weighed in earnest. After this a new kind is still written in ten places where it was eleven, and the ten are English or decisions. What is gained is some 1,400 lines of copying gone, a kind's word written once, and an export that cannot name a field other than as the engine does. What is paid is one macro of some forty lines and `rustfmt` no longer formatting the enum's body. Worth it for the fields and the word; not for more.

## The declaration

In `crates/sim-core/src/events.rs` the enum is written inside one macro of that file, with nothing but each variant's first line changed:

```
event_kinds! {
    /// A task force stopped because a ship ran dry.
    TaskForceOutOfFuel = "task_force_out_of_fuel" {
        task_force: TaskForceId,
        hex: Hex,
    },
    OrderRejected = "order_rejected" [by_hand] {
        order: OrderId,
        target: EntityRef,
        reason: RejectReason,
    },
}
```

From it the macro makes:

1. **The enum,** with its derives, doc comments, variants, fields and their order exactly as now.
2. **`EventKind::tag(&self) -> &'static str`,** the kind's word.
3. **`EventKind::KINDS`,** a constant list of every kind in the order declared: the variant's name, its word, and whether it is `[by_hand]`.
4. **`EventKind::fields(&self, key: &UmpireKey) -> Option<Vec<(&'static str, FieldValue)>>`.** For a kind not marked: each field in the order declared, named by its own name, its value read by its type; a field whose reading is "nothing" is left out. For a kind marked `[by_hand]`: `None`.

`FieldValue` is plain data in `sim-core` (a whole number, a `Fixed`, a flag, a fixed word, words, a hex, hexes, a list of ids): what `sim-cli`'s `Field` holds, less its objects. A type is read through one small trait in `sim-core`, `ReadField`, `fn read(&self, key: &UmpireKey) -> Option<FieldValue>`, implemented once for each type that has one reading everywhere it is used. A kind with a field whose type has no reading does not compile unless it is marked `[by_hand]`.

## The rulings

1. **The word is written once.** `describe` returns the data alone; the export of the log and of a side's events take the word from `tag()`. A test in `sim-core` holds that the words are all different and that each is the snake case of its variant's name, so a word cannot drift from its kind.
2. **A kind is read by type only on proof.** A variant loses `[by_hand]` and its arm in `describe` only if (a) at least one export of the proof below holds an event of the kind, (b) every export is byte for byte what it was, and (c) the arm has no branch the exports did not take: it holds no `match`, no `if`, no `is_empty` and no call that writes a key only sometimes (`flag_when_true`, `int_when_nonzero`, `id_when_some`) beyond the one reading of each field's type, or else each side of every such branch is taken in the "before" exports. Identical bytes show the keys, their order and the values that were written; they do not show a branch no event took, and a kind fired once shows one. (c) is shown by a script over the old arms, given in the description with what it printed. A kind no export fires keeps its mark and its arm, so the 34 kinds no scenario fires keep theirs until a scenario fires them; no proof is made for them here. So does a kind whose JSON is not exactly its fields in order by their types: nothing in the export is changed to fit the macro.
3. **A type has a reading only if it is the same wherever the type is exported today.** `bool` reads as a flag; the kinds that leave a false flag out stay by hand. `Option<T>` reads as `T` or nothing only if every kind that holds one writes it so. The developer's table of types and readings is in the pull request's description and the architect reads it.
4. **A word for a value is written once.** Where a type's word is in `sim-cli`'s `export/names.rs` today (a side, a weapon, a track and the like) and the type is read by type, the word moves into `sim-core` as the type's reading and the function in `names.rs` calls it. The words are the same bytes.
5. **`fields` is the umpire's.** It asks for the `UmpireKey`, which only a `World` gives, because an order's, an event's and a contact's number are read with it (decision 0086). Nothing that plays can call it. It gives nothing the variant's public fields do not.
6. **The export of a kind marked `[by_hand]` is `sim-cli`'s arm, and a mark with no arm is caught by a test and by CI, not by the compiler.** `describe` is the hand arms and then `_ =>` the reading by type. Beside it is the list of the words its hand arms cover; a test holds that list equal to the marked kinds of `EventKind::KINDS`; and an event that reaches `_` with no fields panics with its word, which CI's export of every scenario meets the first time a scenario fires the kind. This is weaker than the compiler and is accepted for the export, which is the umpire's and decides nothing. It is not accepted for who may see an event, which this record does not touch.
7. **`seen`, the kind tables, `entities()` and `Display` are not edited.** No line of them changes in any pull request of this record; the one edit among `side_events.rs`'s tests is ruling 13's. `describe_side` stays by hand.
8. **`rustfmt` does not format the body of the macro's call.** The enum's body keeps the layout it has, four spaces a variant and eight a field, so the pull request's difference in `events.rs` is the macro, the 179 first lines and nothing else. The reviewer reads the layout of a new variant; that is the cost named above.
9. **Text for a side stays the `Display` of what the side was given.** Nothing here hands `sim-agent` anything new. The export's rewording of the staff's three kinds stays in `sim-cli` and is never used for a side. No text for a side is ever built from `fields`, from the log's `EventKind` before `seen` has judged it, or outside `sim-core`'s `Display` and the mapping of numbers `sim-agent` already does. `SideEventKind` gets no `fields` in this record.
10. **`sim-agent` takes a kind's name from the engine.** `SideEventKind::tag()` in `side_events.rs`: a whole event's is its `EventKind`'s, a cut form's is written in one exhaustive `match`, each the word `kind_name` gives today (the snake case of the cut variant's name). `kind_name` calls it and no longer formats the event. It is a name the side already has; `seen` and `project` are not touched.
11. **Versions.** `hash::STATE_VERSION` is not raised: no state and nothing hashed is touched. The replay's version (6), the agent interface's (56) and the doctrine's stay. `sim-core/Cargo.toml` is not edited.
12. **Stop, and back to the architect,** if any export differs and the cause is not a kind wrongly unmarked; if a word, a key, a text, a schema or a golden file would have to change; if `seen` or a kind table would have to be edited; or if a wrong variant inside the macro gives an error a developer cannot read. A class can rise and never falls (decision 0233, item 10).

13. **Two tests that read the enum from the source read `EventKind::KINDS` instead, on the architect's word** (decision 0233, item 12). `event_kind_names` in the tests of `crates/sim-core/src/side_events.rs` (which holds that there are 179 kinds and that each has a row) and `event_kinds` in `tests/scenario_07_determinism.rs` both look for the text `pub enum EventKind {` in `events.rs`, which the macro takes away. Each function's body becomes the variants' names from `EventKind::KINDS`; nothing else of either test changes, not its assertions, its count of 179 or its doc comment beyond "read from the source". This is no loosening: the list is made by the compiler from the declaration and cannot miss a variant, where the reading of the source could. Both edits are in pull request 1, which for the second, a file under `tests/`, waits until items 2 and 3 of the programme (one test binary a crate; the pins) are merged.

## Rule card

No rule of the game is added, changed or removed. No state is read or written, no draw made, no event kind added, no event's text, causes, entities or visibility changed, no version moved. `sim-core` gains three public things of `EventKind` (`tag`, `KINDS`, `fields`), one of `SideEventKind` (`tag`), `FieldValue` and `ReadField`, and still depends on nothing.

## Proof of no loss

What must be byte for byte the same before and after each pull request, and how it is shown. "Before" is the merge base, built in a second worktree.

- **P1, the exports.** `sim-cli export` of every `data/scenarios/*.toml` at its own seed, as CI's loop does it, and of `04_savo_island`, `05_eastern_solomons` and `24_shells_and_estimates` at `--seed 2` and `--seed 3`, into one folder before and one after. `sha256sum` of every file, the two lists compared: no line differs. This holds each event's `kind`, `text`, `entities`, `data`, the breakdowns and both sides' `side_views`. The description gives the number of files and the hash of the sorted list.
- **P2, which kinds the proof saw.** The distinct `events[].kind` over the "before" folder, counted and listed (`node` or `jq`); for pull requests 2 and 3, every kind that lost its mark is on that list (ruling 2).
- **P3, the paths.** `git diff --stat` against the merge base names only `crates/sim-core/src/events.rs`, one new file of `sim-core` for `FieldValue` and the readings if the developer wants one, with its `mod` line in `crates/sim-core/src/lib.rs`, `crates/sim-core/src/side_events.rs` (pull request 1: the body of `event_kind_names` alone; pull request 4: the new function alone), `tests/scenario_07_determinism.rs` (pull request 1: the body of `event_kinds` alone), `crates/sim-cli/src/export/{events,sides,names,data}.rs`, `crates/sim-agent/src/common.rs`, and the two documents of pull request 3. Nothing else under `tests/`, nothing under any crate's `tests/`, `data/`, `viewer/`, `table/`, `web/`, `crates/sim-agent/schemas/` or `.github/`.
- **P4, the text and the choices.** In `events.rs` no line of `entities()` or of either `Display` differs; in `side_events.rs` no line but the added function and the body of `event_kind_names`; in `tests/scenario_07_determinism.rs` no line but the body of `event_kinds`.
- **P5, the tests.** `cargo test --workspace -- --list`, sorted: the same names before and after but for the tests this record adds; none removed or ignored. The whole suite passes, with the agent's golden files, the recorded games, the pins and the 252 baselines untouched (P3) and so compared as they were.
- **P6, the other build.** CI's `web` job passes: the module for the browser answers as the native program does.
- **P7, the three checks** of "Before a commit", and `cargo run -p sim-cli -- run data/scenarios/01_transit.toml --events` printing what the baseline holds.

## The pull requests

One branch each, in this order. Pull request 1 is begun only when items 2 and 3 of the programme are merged, since it edits a file under `tests/`. 1, 2 and 3 edit the same files and follow one another; 4 may be built beside 2 and 3 once 1 is merged. None edits `crates/sim-cli/tests/export.rs` or `tests/common`. The architect reads each before it is merged.

1. **The declaration and the word.** The macro, the enum inside it with every variant's word, `tag()` and `KINDS` (with every kind's mark false and no `fields` yet), the test of ruling 1 and the two edits of ruling 13; `describe` returns the data alone and the 179 literals go; `export/events.rs` and `export/sides.rs` take the word from `tag()`. Shown: P1, P3 to P7, and `cargo test --workspace` built and passed, not the library alone.
2. **The reading by type, for the commonest types.** `FieldValue`, `ReadField` for `Fixed`, `u32`, `u8`, `bool`, `Hex` and the ids with a public number; `fields()`; every variant marked `[by_hand]` but those whose fields are all of these types and that pass ruling 2, whose arms are deleted; the `_` arm, the list and the test of ruling 6. Shown: P1 to P7, the table of ruling 3 and the script of ruling 2 (c).
3. **The other types.** A reading for each further type that has one everywhere (a side and the other words, the opaque ids, lists of ids, hexes, `EntityRef`, and `Option` where ruling 3 allows); the words moved as ruling 4 says; more marks and arms removed under ruling 2. With it the documents: a section "Adding a kind of event" in `docs/engineering/traceability.md` listing the places as they then are (the declaration and its word; `entities()`; `Display`; `seen` and the kind tables; a hand arm only if marked; the batch record; the briefing), and a line in `docs/engineering/layout.md`. Shown: P1 to P7, the table, the script of ruling 2 (c), and the number of arms and lines left in `describe`.
4. **The agent's name of a kind.** Ruling 10. A test in `sim-agent` keeps the old way of making the name, for the test alone, and holds the two equal over every event each side is given in `04_savo_island` (the cut forms of a surface fight), `05_eastern_solomons` (those of the air) and `128_submarines_against_warships` (those of the boats). Shown: P3 to P7; the agent's golden files and the recorded games unchanged.

## For each developer's brief

- Work in a worktree on the pull request's own branch. Read this record, `docs/engineering/determinism.md`, `truth-and-belief.md` and `layout.md` first.
- Make the "before" exports first, from the merge base, and keep them; nothing is edited until they exist.
- The change is mechanical and its measure is P1. If an export differs, put the mark back on the kind; do not change a key, a word or a reading to make it pass, and do not edit a test but as ruling 13 says.
- Do not touch `Display`, `entities()`, `seen`, a kind table, a schema, a golden file, a baseline, a pin or the viewer. Do not add a dependency. No float, no hash collection.
- Edit the enum's first lines by a script and say so in the description, with the script; do not retype the enum. Keep its layout.
- The description carries the class (2), this record's number, and everything "Shown" for the pull request.
- Anything unclear, or anything of ruling 12: stop and ask the manager, who asks the architect.

## What it leaves out

- **Typed fields for a side.** See below.
- **The viewer's reading of replay files of versions 1 to 3,** and the two copies of its rebuild.
- **`describe_side`,** the hand arms that remain in `describe`, and the objects of commands, refusals and claims.
- **`batch_record.rs` and `briefing.rs`,** which read events and do not word them.

## For the designer

Nothing in this record needs Amit's word: it is class 2, and he learns of it from the report of what was merged.

One thing it found is his to ask for. The table reads the engine's sentences by regular expression in about twenty places, and tests a kind's word 57 times, because a seat's `events` and `explain` give an event's kind, text and entities and no fields. Giving a seat its events' fields would end that. It is class 3: it changes what a seat is given, every id in a field would have to be the side's own number and never the umpire's (decisions 0085 and 0086), the schemas and the agent interface's version would move and the recorded games with them. It is not planned here and nothing here depends on it. If he wants it, it is a record of its own with his word before it is built, and the reading by type made here is what it would be built on.
