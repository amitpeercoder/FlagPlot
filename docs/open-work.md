# Open work

A list kept by the architect of what is decided and not built, what is open, and what is next. Begun on Nov 12, 2026 as step 2's opening list. Each item that changes a rule becomes a plan and a decision record, with a hand calculation where it has a number, before it is built; decision 0233 says which changes need a record and what reviews each needs.

Trimmed on Nov 27, 2026 to what is open or next. What was closed, built or overtaken is in `docs/open-work-closed.md`, word for word; where a line moved there also named something still open, this page keeps that in short and names the decision. `CLAUDE.md`'s Status says in a few lines where the game stands, and the index in `docs/decisions/README.md` gives each record's standing.

## Next

1. **Stage 4's proof** (decision 0161, step 9): set up in `docs/validation/stage-4-proof-setup.md`, amended twice on Nov 27, 2026, for decision 0232's steps 1 and 2, and not run. Run once, by the architect. Two misses are expected on the Japanese side, conditions 3 and 13, their cause named before the run.
2. **A plan the commander writes** (decision 0232): steps 1 and 2 are built; step 3, the table, is next.
3. **The enemy's approaches** (decision 0161, step 10; below): its own record and plan first, for the designer's confirmation.
4. **Levels** (`docs/design/levels.md`): a note, nothing built; the designer's answers of Nov 26 and 27, 2026 are in it. Its first three rungs are planned as plain scenarios on the table's list (decision 0237, confirmed on Nov 27, 2026); rung 1 is built and on the table; rung 3 goes on before rung 2, on the designer's word of Nov 28, 2026, and its mission names day 14: its game is built (scenario 140) and its table is planned and not built (`docs/decisions/as-built/0237-the-first-three-rungs.md`); rung 2's figures wait for his account of rung 1.
5. **The clean-up in progress,** beside these and not ahead of the proof.

## The clean-up in progress

Approved in full by Amit on Nov 27, 2026, on the architect's review of the repository's upkeep, with the rulings on it delegated to the architect: "Let the architect rule where and when such needed. Have Sonnet agents do the development. Have the architect as a final reviewer. Do all that's needed (all items)." Decision 0233 records the delegation and rules A, B and C. It takes the place of the clean-up agreed on Nov 17, 2026, whose items 1 and 2 are done (the checks made quicker; the large files of `sim-core` split by phase) and whose items 3 and 4 are E and "The old switches" here.

The foundations stay as they are: the pure core, fixed point, keyed draws, events with causes, truth and belief, facts derived into ratings, decision records and hand calculations, the fixed order of phases. Each item is its own pull request, Sonnet developers build, and no two items edit the same files at once. The class is decision 0233's.

| Item | What | Class | Standing |
| --- | --- | --- | --- |
| 1 | The documents: `CLAUDE.md`'s Status rewritten, this page trimmed, rules chapter 8 split into a folder, the history taken out of `docs/commands.md`, the indexes trimmed | 1 for `docs/commands.md` and the indexes; 2 with no record for the split of chapter 8, which the architect reads to see that no sentence of a rule changed; `CLAUDE.md` and this page are the architect's | In flight: the architect's part with decision 0233, the rest by a developer |
| 2, D | One test binary for each crate, and no second run of the checks on `main` for a tree a pull request already passed | 2 with no record (decision 0233, item 14): the architect reads the pull request, shown the names of the tests run before and after | In flight |
| H | The scripts of closed proofs marked frozen: kept to say how a proof was run, not kept working | 1 | In flight, with item 1 |
| A, B | What a record holds and where a rule is stated; ceremony by class | Ruled | In force from decision 0233 |
| 3, C | The day hashes out of the 252 run baselines into `tests/golden/pinned_hashes.txt`; the state version raised once in a pull request | 2 | Ruled in decision 0233, C, which is its record. The day hashes are out of the baselines and in the `run_` lines of the pinned file (built: `docs/decisions/as-built/0233-ceremony-by-risk-and-where-things-are-written.md`, "Built: C"); the state version raised once in a pull request is a working rule, item 21 |
| 5 | Shared helpers for scenario tests in `tests/common`, taken from the tests' own code, and a test that fails when one is defined again. No builder for a struct: nothing calls for one | 2 with no record: the architect reads each pull request | Ruled, "Item 5 as ruled" below: two pull requests, the second when no other open pull request edits `tests/*.rs` |
| 4, F | A scenario built on another: `base = "..."` in a scenario file, read by `sim-data`, in place of the 63 generated scenario files and the Python that makes them; before it, `scenario.rs` split by table as a pure move | The split is 2 with no record, a pure move the architect reads. The overlay is 2 for a file nothing names by its bytes, and its plan says which files those are: a file that a closed proof, a frozen copy, a recorded game or a report names stays a file | The architect's plan next, a record of its own. Each overlay is shown to load to the same game as the file it replaces, day hash for day hash, before that file goes |
| E | The export carries the engine's own words and typed fields, so the viewers stop switching on event kinds; and an event's declaration narrowed to its mechanical parts (its place in the hash and the export), in place of the four or five places it is written today. Who may see an event stays a choice its author must state: a new event still fails to compile until it is decided. `sim-core` still depends on nothing | 2 as planned in decision 0235: no rule, state, event kind, text, export format or version moves | Planned in decision 0235, on measurement: only a kind's word and its fields are declared once, in `EventKind`, and the export reads them by type, every export byte for byte as it was (about 110 to 120 of the 179 arms of the export's `describe` go; the 34 kinds no scenario fires keep theirs); the text, the entities, who may see a kind, the briefing, the batch record and the viewer's switches stay. To be built in four pull requests, the first after items 2 and 3 are merged, since it edits a test under `tests/`. Typed fields for a seat's events, which is what would stop the table reading the engine's sentences, are class 3 and wait on Amit's word |
| G | The doctrines' common skeleton in `sim-staff`: first a survey by a developer, which changes nothing, of what the doctrines repeat | The survey is 1 if it is written under `docs/` and changes nothing else. Any change from it is 2 at least, with every recorded game and proof file untouched | The survey before stage 5; the architect reads it and plans from it |

### Item 5 as ruled

Ruled by the architect on Nov 27, 2026, on a count of `tests/*.rs` (189 files, 227,000 lines). It takes the place of the Nov 17 wording ("the tests that exist are not rewritten") in one respect only, the second pull request below.

**What the count found.** The repetition is small helpers, not struct literals. Commit e536d81, cited as 63 files for one field, touched 14 files under `tests/` and four literals in three of them; the rest were schemas and recorded games. So no builder with defaults is written for any struct of `sim-core`: none is asked for until a field added to a struct changes more than ten test files, and then the architect is called. The helpers counted below are 552 definitions in 139 files, about 2,000 lines, under one line in a hundred: the gain is that a new test stops copying them, not the size of `tests/`.

**1. What goes into `tests/common/values.rs`** (a new module, `pub mod values;` in `mod.rs`), each the form most files have today:

```rust
pub fn fx(text: &str) -> Fixed            // text.parse().expect("a decimal")
pub fn n(whole: i64) -> Fixed             // Fixed::from_int
pub fn hundredths(value: i64) -> Fixed    // Fixed::from_ratio(value, 100)
pub fn millionths(value: i64) -> Fixed    // Fixed::from_raw
pub fn hex(q: i32, r: i32) -> Hex         // stage_4.rs's `hex` becomes `pub use` of this one
pub fn ship(raw: u32) -> ShipId           // and force -> TaskForceId, group -> AirGroupId,
                                          // base -> BaseId, unit -> LandUnitId, intent -> IntentId
pub fn within_four_sigma(k: i64, n: i64, p: i64) -> bool   // the integer form, as scenario 3 has it
pub fn text_of(reason: &Reason) -> String                  // "worked | binding | evidence", as scenario 107
pub fn assert_says(all: &str, parts: &[&str])              // as scenario 107
pub fn at(text: &str, name: &str, what: &str) -> String    // `what` put before the line "# MARK: name"
pub fn command_table(day: u32, body: &str) -> String       // an Allied [[command]], as scenario 125
```

`changed` is `common::replace_once`, which exists: a file imports it under that name. `within_four_sigma` is the tolerance thirteen tests judge by; its body is moved, never altered. And `tests/common/played.rs`, for new tests only, taken from scenario 107's `play_from` and `rebuilt`:

```rust
pub struct Played {
    pub scenario: Scenario,                                       // as it stands at the end: events, day hashes and command state are read from it
    pub handed: BTreeMap<Side, BTreeMap<u32, StaffDecisions>>,    // what each side's staff handed in, by day
    pub truth: Vec<TruthState>,                                   // [0] the start, [d] after day d
}
pub fn play(scenario: Scenario) -> Played                         // both sides' staffs, `sim_staff::staffs()`
pub fn rebuilt(scenario: Scenario, handed: &BTreeMap<Side, BTreeMap<u32, StaffDecisions>>, upto: u32) -> Scenario
```

It has one test of its own, on `67_taking_a_place.toml` loaded with `common::load`, holding no number: two plays give the same day hashes and the same hand-ins, and `rebuilt` to the last day gives the play's day hashes.

**What stays in each file,** in old tests and new: `path()`, `text()`, `load()` and `figures()` (each names its own scenario and figures and is already one line onto `common`); `play`, `play_from`, `Played`, `Run`, `given`, `cmd`, `variant` and the like (86 forms of 112 structs; which side, which fields and which orders a test reads are part of what it states); and a helper of a common name with another meaning (`fx(value: i64)`, `base() -> Scenario`).

**2. What is migrated.** In the second pull request, by a script and not by hand: a local definition of one of the names above is deleted, and the name imported (`use crate::common::values::{..};`, or `use crate::common::replace_once as changed;`), only where its parameter types and return type are the common one's and its body is the common one's token for token, parameter names and white space aside. Four differences count as the same: `Hex::new(q, r)` for `Hex { q, r }`; `text.parse::<Fixed>()` and another `expect` message in `fx`; `changed(..)` for `common::replace_once(..)` in `at`; a type written with its path. Any other form stays where it is. No call is renamed, so no line inside a test changes. A `///` comment directly above a deleted helper goes with it, and the pull request lists each such comment for the architect to read. A `use` line may lose a name that is no longer used. Nothing else in any file changes: no `//!` comment, no statement, no expected number, no test's name.

**3. The proof of no loss,** shown in each pull request: (a) `cargo test -p scenario-tests -- --list`, sorted, before and after: the first adds only the tests named here, the second adds only the guard; (b) for the second, every removed line is a deleted helper, its `///` comment, a blank line beside it or a `use` line, and every added line is a `use` line, shown by the script's own report of each file (definitions deleted, with the form each matched); (c) `tests/golden/`, `data/` and every crate are untouched, and the three checks pass with `FLAGPILOT_UPDATE_GOLDEN` unset.

**4. Two pull requests.** The first adds `values.rs`, `played.rs` with its test, and the sentence for `docs/engineering/roles-and-tests.md` ("A scenario test takes `fx`, `n`, `hex`, the makers of ids and the other helpers of `tests/common/values.rs` from there and does not define them again; a new test that plays both staffs uses `common::played`"); it edits no test file that exists. The second is the migration and the guard, opened when no other open pull request edits `tests/*.rs`, and brought up to date by running the script again, never by merging by hand.

**5. The guard,** a test in `tests/all.rs` beside `every_test_file_is_declared_as_a_module`: it reads `tests/common/values.rs` for each `pub fn`'s name, parameter types and return type, and fails naming any file of `tests/*.rs` that defines a `fn` of the same name and types. It reads Rust files only.

**The other packages** (`crates/*/tests`, 56,000 lines) are not part of this: they cannot use `tests/common`, `sim-agent` has its own `tests/support`, and what else repeats is ten definitions of `binary` and eight of `temp_dir` in `sim-cli`.

**The old switches,** listed by the architect on Nov 27, 2026 in `docs/validation/old-switches.md` (35 of them) and ruled in decision 0236. Amit's word of Nov 27, 2026: "Retire the five no-cost switches", numbers 16, 24, 25, 28 and 35 of the list; and of the others, "Don't wait on me. Let the architect make whatever decision." So nothing on the list waits on his word any more. Where each stands:

- **Retired by decision 0236, class 2, to be built in two pull requests:** the four bombardment figures (16), the base hit shares for men and the airfield (24) and `men_lost_per_base_hit` (25) become required; the two figures of replacement men (28) leave the code and are written as placeholders into the 34 ground files; and `search` and `surface_combat` stop being optional inside `sim-core` (35), a pull request of its own. No game moves. Step 1 waits on nothing; step 2 on step 1 and 0234's step B, and strikes seven rulebook sentences on a case no game can reach.
- **To be retired by a record of its own, class 3, not written yet:** the key for the staff's choice of a source (4). It is not no-cost: eight games move, three hand calculations are restated and 236 pins are remade. The architect writes the plan after decision 0236's two steps are merged; his ruling stands in place of Amit's confirmation, by Amit's word.
- **Not retired, ruled again by the record of the interface's design:** command levels (1) and orders inside a day (3). The list had three "retire later"; the third, 35, is retired now.
- **Kept:** the other 27.
- **Open defect, to be mended first:** with `command_levels = true` added to `data/scenarios/29_skills.toml` the game panics on day 1 at `crates/sim-core/src/pictures.rs:231`, "base:8 is not a command of the Allied side": a base that comes to be during a game has no command. Nothing on file meets it as scripted; whether a seat's own orders can in one of the eight files with command levels on has not been ruled out. Decision 0236, ruling 18: command levels are turned on in no further scenario, and switch 1 is not retired, until it is mended; the architect is to be called for it as its own job, to find what decisions 0134 to 0138 say of a base formed in play and to class the mend. **Ruled on Nov 27, 2026, decision 0239** (class 3; built in pull request 773, so mended): a base formed by a march was never made a command; it becomes one as a base formed by a landing does. The stop is on day 3, not day 1, and one file with command levels on does form such a base as written, `stage-2b-proof/07a_week.toml` on day 16, without stopping; its game moves with the mend.
- **The list was wrong in three places,** corrected in it and named in decision 0236: a test of scenario 24 reads the off side of 16, 24 and 25; three recorded games' saves change in the hash of `ground.toml`; and 35 is no game moved but real work.

Kept for later, and not part of this: speed at the size of a whole campaign, measured (`sim-cli time`) and given a budget before anything is changed for it.

## Open from step 3

- **Open from the close of stage 2** (decision 0128): a comparison for one-day battles that does not reward staying away; and one reason of the search of waters is 52 KB because it lists every hex, and should name the waters.
- **Open, found on pull request 765's reading and not mended there:** a preview of an unchanged `set_intent` of an order lists other intents (for example 4 to 8) in `other_plans_give_up`, with identical counts on both sides. Older behaviour.
- **Open from stage 2b's step 3 (decision 0136):** (1) a staff that reckons a force forward from its own orders when word of it is stale, which stage 4's trial machinery makes possible: without it an area staff with no word of a supply run never sails it (scenario 36 with every state report lost); (2) a base reporting the ships in its port, which the area command reads as the truth today; (3) what a command knows of its neighbours' forces; (4) the afternoon's state report is made before the first steps of the next day change a force.
- **Open from the close of stage 2b:** (1) the chances that a report is late or lost, and radio reach, are placeholders; the research has come in and is being checked into rows ("What the architect settles from the research", below); (2) the initiative rule is built and had nothing to act on in a battle of one day; it wants a longer game or orders inside a day to be judged; (3) a seat's order holds its unit against the staff for the rest of the game and the seat cannot release it; a seat is not told when an order of its own is refused unless it asks; (4) what silence buys waits for the enemy listening (`docs/design/intelligence.md`); (5) a seat that is not the area's cannot state intents, and several players on one side, which come with the interface.
- **Open from stage 3 and from stage 4's building** (decisions 0153, 0159 and 0161):
  1. *The enemy's approaches* (decision 0161, the tenth step, added by the designer on Nov 19, 2026): the staff estimates where the enemy could come from against a place it means to take or hold, sets a force or part of one across an approach, and reads the landing's days against his. After the trial's proof; its own record and plan first, for the designer's confirmation.
  2. *Three costs of the supply staff's own way of working, owed after stage 4's proof* (decision 0161, "The reading of three seeds", Nov 24, 2026; the designer's answer of that day). Each is the rule as written and none is a fault; they were read on three seeds of the proof's arrangement and are not mended before it. (i) A force back at the source is given to the pool the morning it is found and its run is ordered the morning after, since the supply doctrine is asked before the island's: two mornings between a ship in port and its orders. (ii) A finished run's ships stay committed to the day reckoned when the run was ordered, two to three days after they are back. (iii) A force goes whole to one base, so a load of 15 days for one base takes the only force while another base of the same island has none; with it, that a commander cannot rank one place of an island above the land beside it (decision 0231, amended 2). Each wants its own record and hand calculation. The proof's conditions 3 and 13 are read again after any of them is built and reported beside the proof's reading, with no second judgement.
  3. *Seen in the proof and not mended:* no carrier force has a home port in the staff's setup, so its fuel is never weighed; the bombardment force stops for want of rounds on 816 seeds, Shortland holding no ammunition; the cargo ships of 21 August are never sent, not being in the forces limit the hold set on day 1; a second lift under a limit the staff assembled finds no unit.
  4. *Owed:* a model's game of the 39 days by intent alone, reported and not judged; it also replaces `full_slice_five_days_sonnet`, whose requests were written for the fourteen-day slice.
- **Open from decision 0232's step 3, the table** (`docs/decisions/as-built/0232-a-plan-the-commander-writes.md`), each found on a reading and none mended:
  1. *SIGNED says "another order" for an `After` on a landing not yet in force.* A landing written under an order (pull request C) may wait for another landing of his signed the same evening; SIGNED's line for it is said by `afterWords` in `table/index.html`, which finds only an intent in `command_state` and so says "after another order" where the paper he signed said "The landing at Lunga". A named gap: the words should name the landing from `queued`, as the paper does.
  2. *`approve`'s note is untrue of an order to seize an island.* The note "intent:N has no forces limit: the staff gives no order for it until you set one (set_limit forces)." is given for any order to the area with no forces limit on its chain (`crates/sim-agent/src/staffing.rs`, in the notes of the command state, and again as "no forces limit yet" for an order queued). The second line of step 3's review found that the staff does plan such an order to seize and hold an island. A lead, not checked against the doctrine: whoever takes it up reads which kinds the staff plans with no forces limit, and the note is then narrowed to the kinds it is true of. A change to a tool's words, so class 2 at the least, and the architect's to classify.
  3. *A `dry_run` entry that names no size the page knows offers no size.* An entry of `table/scenarios.json` whose `dry_run.sizes` holds none of small, standard and full makes `sizesOffered` give none, and the dry run's box draws no button. No entry is so written. It should offer the three, as an entry with no `dry_run` does.
- **The interface.** The table has grown far past the prototype, in records built and planned (decisions 0154 to 0157, 0182, 0185, 0204, 0214 to 0217, 0219, 0221, 0222, 0226 and 0229). The open points written here on Nov 18 and 19, 2026 (the tools a seat lacks, the directive, the staff's report, the cards and the ledger, and the six instructions a commander could not yet give) are in `docs/open-work-closed.md` as written and have not been read again against those records: the architect reads them again when the interface is next planned, and what is still open comes back here.
- **Limits of stage 2 as built,** for the stages after it to lift: the staff does not choose where a carrier force the commander named stands (it does for a take, an island and, since decision 0202, a free carrier force under his deny or neutralise) or move a force's home; a force that stands by night steams home by day; no rule holds a strike to an area; one escort force goes with a run or none; a run is not timed to the dark; a carrier raids a base only under a take, an island or, since decision 0202, a neutralise; a single fighter group that both patrols and escorts is never rested.

## Owed from records that are built

Each line is what a built record still owes. "Readings" are the readings a build made and listed for the architect to confirm, in the record's "As built" and in the rows of `docs/rules/differences.md` named.

- **Open from decision 0166 (the ships of the runs):** two sources say the Kamikaze class lost a gun and a pair of torpedo tubes in 1941 or 1942 (fits 1100 to 1103, disputed). Yunagi at Savo Island is derived with four guns and six tubes. Left as it is while Savo Island is closed; to be ruled by the next round that opens it.
- **Owed from decision 0168, a gap in traceability:** the draw of whether a raid was recognised (decisions 0167 and 0168) records no breakdown of its chance, though a ship's chance is now worked from its experience. It needs a home of its own, since an attack's breakdown is that of its hits. To be closed with the next change to the events of an air attack.
- **Owed from decision 0169:** (a) on a morning a lift is stopped, the judgement ashore still says the unit left behind "is sent now" (rulings, 4); (b) at preserve a take that cannot meet its end hands in "cannot meet" every morning and not once; (c) the rule weighs only the men a lift lands, and on one seed of the 39 days it stops no Japanese lift: whether a place can be kept fed over weeks is for the trial (rulings, 2).
- **Costs that grow with a campaign's length, not mended** (noted with decision 0172): a run checks for a fired interrupt by projecting the side's whole history of events each day (about 9 ms a day at day 150 of a soak, measured once); a preview copies the world twice for each staging; a session keeps every call it is sent, asked for a transcript or not. To be taken before campaigns longer than the 39 days. A game file kept by `play` plays every `trial` call in it again when it is replayed (decision 0178, rule 20): about 1 to 2 s a call at the small size on the 39 days, 23 to 36 s at the full.
- **Owed from decision 0170:** a test, across a picture, of each rule that reads the log of earlier days (the words of a refusal to a force that is gone, an interrupt's count of the day's losses, yard trips in the score, a note not written twice): rulings, 7. With step 4 of decision 0161. The test of a world with days behind it is made by decision 0171's build: the full slice, rebuilt from its two halves on every evening and played to day 39 through the carrier battle, plays as the live game does. None of the four rules is reached in it, so a test of each stays owed.
- **Owed from decision 0173:** the check of a replacement's and an arrival's aircraft type against the file of sides; and the script that makes `data/scenarios/staff/07_full_slice.toml`, which is not in the repository (rulings, 3 and 4).
- **Owed from decision 0175** (the designer accepted the three sizes on Nov 19, 2026, on the measured times): if the times come to ask for it, worlds of a trial that record no breakdowns, and worlds played side by side.
- **Owed from decision 0176:** a case of a plan displaced by a take of another place; and words for an estimate that a commander can read, where they now print the engine's own names (rulings, 3 and 5). The assertion of "a plan displaced" as the architect stated it is in scenario 95's test file, added with decision 0177's build.
- **Owed from decision 0177:** an air group that comes due makes no date of a window, since no take counts one (rulings, 2); and a window at the full size takes 67 s on the 39 days, which goes to the designer with step 8 (rulings, 8).
- **Owed from decision 0182** (the architect's rulings on step 1, 1): a dry run is to be tried in the test that a seat's answers do not read the other seat's queue, on a scenario with a trial, before a game of two agents is called fair.
- **Decision 0169, also:** scenario 68's property "A hold is honoured" is marked ignored, for the architect to restate.
- **Decision 0173, also:** the 39 days' entries of what each side believed are placeholders; what each command believed of the other in the first week of August 1942 is owed as rows in `data/oob`.
- **Decision 0174:** no land air group is built at an enemy base in the enemy's half of a trial.
- **Decision 0176, also:** the lift and the weight of supply in what an estimate says is needed; the end of an intent that is not a place.
- **Decision 0177, also:** a window overstates what waiting costs in days, since nothing of a course is prepared before its date; dates from aircraft due, supply runs due, training and searches; an enemy who uses the days.
- **Decisions 0178 and 0181:** the trials the staff asks for itself (the one a leave to launch is asked with, and the comparison where a limit binds); stopping a trial from a seat; and, for the designer, a full-size trial with its second trial at 36 s and a full-size window at 67 s on the 39 days.
- **Decision 0182, also:** the watcher taking a seat at a command while an agent holds the area command; the watching page says "your order" and "your leave" where it means the commander watched; the watcher cannot look from the engine's side of a game against the engine.
- **Decision 0183:** statement 1's "the Japanese objectives name Lunga alone" does not hold on the full slice (they also name Tulagi), which is the architect's to restate. For the designer: the table's situation sheet shows the briefing whole, a side's points included, against decision 0157.
- **Decision 0185:** not read in a browser; taking back a call of time, a model started by the bridge, a seat at a command.
- **Decisions 0184, 0186 and 0187 (scenario 103):** the four games between agents before it is called balanced; the band read again when its file or a doctrine it uses changes; the table of objectives and the words "the game's one scale" in its hand calculation, the architect's to change.
- **Decision 0188 (the Coral Sea):** a run with decisions 0193 and 0194. Found by the agents and not taken up: no fuel warning for a force with no home port; the carrier search's aim; a carrier force that will not leave a contact in reach; a waiting plan that does not say it can no longer arrive; whether a convoy fought in its landing hex can land.
- **Decision 0190:** scenario 105's application of it, the architect's (rules 9, 24 and 37, its hand calculation amended first); the 1,000 games of scenario 105 in four cells with decision 0191; readings, row 49.
- **Decision 0191:** readings (row 46); later lifts and a hold's lifts are not turned back.
- **Decision 0192:** readings (rows 47 and 48; scenario 108's statements 1, 8 and 10a); scenario 105's rules 10 and 19.
- **Decision 0193:** readings (rows 50 to 52); statement 9 of scenario 109 is not tested in play.
- **Decision 0194:** readings (rows 53 to 55); scenario 109's statement 3 to be restated; scenario 110's statements 1, 5 and 8 could not be tested as written.
- **Decision 0195:** readings; nothing leaves the map; the 39 days' late forces still lie on the map from day 1, by the designer's answer.
- **Decision 0196:** readings, and statement 8a (its "day 5" cannot be given; the code gives day 7); cargo that comes in ships the enemy can sink is not built; every line is a placeholder.
- **Decision 0197:** the architect's reading of the build; with the designer, whether replacements cost a veteran unit a level; the staff does not count the men a stream will add.
- **Decision 0198 (Midway):** the architect's reading of the first game (`docs/validation/midway-first-game.md`), then the band of 1,000 games, stage C.
- **Decision 0202:** four statements of its hand calculation do not hold as written (its "As built").
- **Decision 0203:** a day on which a route of least danger is given costs near eight times what it did; stage 2, the same choice for every route the staff gives, moves closed proofs and is not planned.
- **Decisions 0179, 0180 and 0210:** the table's claim line is to name the types, who claims and the tally; one statement of scenario 124 does not hold as written and waits on the architect.
- **From the designer's queue of Nov 20, 2026:** forces that leave the map, to plan. Waiting, by his word: a place's port, and carriers reported as other kinds.
- **Planned, decision 0189 (difficulty):** written on Nov 20, 2026 at the designer's asking and amended on an independent review; not built. Its first step lists scenario 103 on the table and measures easier, standard and harder against the engine's staff by five runs of 1,000 games; a setting is listed only where the runs show a difference.

## Planned and not built

From the Standing lines of the records on Nov 27, 2026; the index has each in full. A Standing line that says "for the architect to read" of a build since merged is itself owed a correction.

- Decision 0201, refuelling at sea: stages B to D (below, "Kept so they are not lost"). Decision 0206, the American oiler as a real class, waits for its build, and decision 0207's estimate follows it.
- Decision 0204, a dry run inside a day: stage 2 (rule 14).
- Decisions 0211 (each raid its own ship) and 0212 (fire aboard a carrier): to be reviewed again and measured. Decision 0213 (a figure by side and date): to be reviewed.
- Decision 0216, a land unit's orders at the table: the three orders are built, the rest is not.
- Decisions 0221 (the screen round the stop) and 0226 (the paper THE ENEMY): planned, to be reviewed.
- Decision 0224, how each base's stock is replaced: ruled and measured, nothing built; the fuel is left open.
- Decision 0229, the build for the browser: stages 5 to 7, and its questions for the designer (who playtests, whether the link is public, the host, a new seed for each game, the marks for speed).
- Decision 0223, each navy's way of placing boats: the stages after D1.

## Open from the ground rules and the staff's first stages

The section "Next, before step 3" is in `docs/open-work-closed.md` whole: everything it listed is built. What it left open:

- **The objectives scale set from the record:** the points and levels of decision 0054 (`docs/research/oob/history-on-the-scale.md`), frozen before the games of step 3's stage 5 are played.
- **The ground figures are all placeholders.** The first tuning round (decision 0109, "As built: round 1") moved four; the Tenaru's two measures are a named miss, and of the three rules named to mend it (units held back are out of the fight, Kawaguchi's force re-cut into battalions, a defender's counter-attack) the architect is to say which stage 3 built. The figures of decisions 0103 to 0106 rest on no source and have rows to tune them on.
- **Real land units:** experience, the four skills, the mark of a working unit and the works skill are 2 or unset for every unit, and wait on research.
- **The gaps decision 0101 lists** in the 39 days: the escorts and covering cruisers, the carriers on 6 August, the 1st Engineer Battalion, the day Kawaguchi left Taivu, Japanese rations.
- **Stage 1's loose ends** (decision 0107, as that section gives them in its item 10): a cannot-meet notice is handed in only when the staff weighs a new run; a set interrupt cannot be withdrawn; a preview is of `set_intent` and `set_limit` only; a hold on a base.
- From decision 0112 (refuelling that takes time): the staff counts a ship's fuelling hours at the berth rate for every ship of a run, where the engine gives a berth only to as many ships as the port has; a run of more ships than berths is free later than the staff counts. To be made exact with the first doctrine that sends several ships to fuel at once (stage 2 of step 3).

## A limit the tool takes where nothing reads it

From decision 0222 (the designer's answer 2, and "As built: the engine", rule 7): each row is a limit that `set_limit` takes on a kind of order where no doctrine reads it. The table does not offer it there; each is to be settled one way or the other: a doctrine comes to read it, or the tool comes to refuse it.

| Limit | Orders on which nothing reads it |
| --- | --- |
| Forces: ships and task forces | Cover a base, strike ships, search waters, raid, patrol, build up a base |
| Forces: air groups | Keep supplied, stand at, patrol; operate a carrier force (the groups aboard go with their force: a limit naming one of them leaves every one ordered, seen on Midway at the table, decision 0222 step 4) |
| Forces: land units | Every kind but hold, ready a landing, build up a base, take a place and control an island |
| Forces: submarines | Every kind but patrol |
| Forces: source | Every kind but keep supplied, hold, take a place and control an island |
| Area | Cover, strike ships, search waters, stand at, patrol, raid, build up a base, ready a landing as a root; a carrier's station (only its closing reads it); a deny, a hold or a neutralise only through a carrier task or a bombardment beneath it. A keep supplied reads its own area and not one above it |
| Protected | Every kind but keep supplied |
| Targets | Keep supplied, cover, stand at, patrol, ready a landing, build up a base |
| Risk | Patrol |

From decision 0222, ruling H: a preview does not show the approvals that prepare and release a plan.

From decision 0222, step 4: a preview takes one order with its limits or one limit: several ranks cannot be read as one change.

## Not planned

- `docs/rules-comparison.md` (the designer's, Nov 14, 2026, checked against the code) sets Flag Plot's rules against another game's manual and is the list of gaps to triage. Decision 0102 rules on each gap (build, defer or decline); the rows it marks "To be written" are decision records still to come.
- Submarines against warships are planned in three stages (decision 0218): the data (stage 1), the rule (stage 2) and the staff's part (stage 3) are built. Stage 3 is the patrol order with its stations, a hold on a boat, boats in a forces limit, the staff's regard for submarine waters (rules chapter 8, Patrolling waters with submarines), the loader's refusal of a boat with no salvo and of a home base the sea cannot reach, the agent's tools (interface 48) and the table's part. Open from it:
  1. **Boats are in no scenario for play.** Scenarios 128 and 129 hold the only boats; putting boats into the scenarios the game is played on (the Solomons games, the 39 days) is not done, and what it does to a game is to be measured before and after, as a rule that changes every game is.
  2. **Stages two and three of the submarine design** (`docs/design/`): a boat that reports what she sees or moves on a report, the war on shipping, and what a boat's approach and conditions add, are not built (chapter 10, What was cut).

## Decisions still to make

| Item | What it is | Why now | Where it came from |
| --- | --- | --- | --- |
| A friendly field beyond the target | A strike that cannot get back to its carrier may land at a base of its own side within reach of the target, and fly from there or return later. Noted, not to be built with the first version of the reach rule | It eased long launches in the record: landing ashore was used twice at Eastern Solomons. It needs air groups that can be away from their carrier, which the game does not have (rebasing, decision 0091, is built, but a group still cannot be away from its carrier and come back to it) | The strike-range research; added at the designer's request on Nov 12, 2026 |
| Daylight as a limit on a launch | A strike launched late must be recovered in the dark or not launched. Noted, not to be built with the first version | Dusk limited a launch as much as fuel did, and the longest reaches at Midway ended in a night recovery. Today a strike flies in the morning or afternoon pulse with no regard to the hour within it | The same |
| Experience for air groups | One number for an air group, read by a fighter's and a bomber's chance to hit and the chance of running out of fuel, diluted by replacements (decision 0064, stage 2). **It waits:** worked by hand from the rows, the facts are too thin to rest the values on, and no air group carries a value until research closes the gaps listed | The air figures are tuned meanwhile with every air group alike in experience, a named gap | Decision 0064, "Nov 13, 2026: stage 2 waits"; `docs/research/oob/air-group-experience-proposal.md` |

## Open, named by the work of Nov 13, 2026

Where an item says when it is done, that is the coordinating architect's ruling of Nov 13, 2026, on the designer's delegation.

- One fight for three forces, so that fights can overlap: a plan for the designer at the close, not built in step 2 (decision 0083, "What it carries and what it does not"). Chicago's fate and the northern group's torpedo hits at Savo Island rest on it.
- Ships of a force that do not fire at all: at Savo Island Helm and Wilson fired no torpedo, Wilson's order to attack not received and Helm's target friendly (`engagement_outcomes` 3217 to 3219), where in the rules every ship of a force that has sighted fires. `allied_torpedoes_fired` is a named miss resting on it; it is participation, not the torpedo share (architect's ruling, Nov 13, 2026; `docs/validation/tuning/savo-island-closed.md`).
- Torpedo reloads: each tube fires once in a game, where Kako fired ten torpedoes from eight tubes at Savo Island and the Takao class had a gear for quick reloading (decision 0083, "What it carries and what it does not").
- A raiding force that fights and passes on: no rule has a force retire when it has done what it came for, so `minutes_of_action` misses at Savo Island (decision 0024, the note on break-off levels). Decision 0096 (built) lets a force act at a marked waypoint and steam on: it can bombard by night at a marked waypoint and go on, and a convoy can unload and come home. It does not make a force fight and pass on of its own accord, time an arrival, act for a set time, or change cargo orders along a route.
- Secondary guns: a cruiser's secondary battery has no counterpart in the rules (`docs/step2-spec.md`, "Where the model stands today").
- Identification under fire, and a force seen and stalked before the first shot (decision 0076, "The gaps, named").
- Radar fire control, and land behind a target masking a set (decisions 0072 and 0077). No rule of radar fire control before Cape Esperance is run (0072, "Built", 1). It has been run: `gun_hits_on_aoba` misses, and radar fire control is named among its likely causes, as conjecture, beside the short night fight and the rule of targets (`docs/validation/baseline-reading.md`, "Cape Esperance: the check").
- A search flown in one pulse only (decision 0073, "What the rules cannot express").
- One kind for every cruiser, so Tenryu and Yubari fire their guns at cruisers (decision 0078, "What it cannot give").
- The briefing's guns line after the attack (decision 0079): the briefing lists the guns' fire after the attack as a guns line of its own, not marked as after the attack. A wording item.
- The fuel round leaves out the ditchings of Ryujo's raid on Henderson Field (decision 0084, finding 13): her B5N2s fly 200 miles on a derived radius of 175 whose rows state no load, so their ditchings are not evidence for `ditch_chance_per_radius_beyond`. A radius with a bomb load is asked of research (decision 0081, C5, option 2).
- Aoba's and Furutaka's hits at Cape Esperance: measured ship by ship since Nov 13, 2026, before the check run (decision 0084, finding 14 and its note); the total of 130 to 134 is retired. Aoba is judged at 40, a weak record; Furutaka is reported only, her rows being lower bounds; Kinugasa is judged at 0 to 4. A row that bounds Aoba or Furutaka and arrives before the check is run changes the record; none changes it after. The check has been run (Nov 13, 2026), so the records stand.
- Named by the Cape Esperance check (`docs/validation/baseline-reading.md`), for step 3 and not tuned on: fire on a friend and a ship acting apart (Duncan), behind `us_destroyers_sunk` and the Japanese claims; Task Force 64 fighting Japanese stragglers by day in 771 runs of 1,000, since it is ordered to stay in (2,-5), which no measure counts; and its five destroyers firing every tube, where two attacked, the participation named at Savo Island.

## What the architect settles from the research

- Which figure to prefer where sources disagree: the Japanese aircraft lost at Eastern Solomons (about 40, 62 or 75), and each day's losses over Guadalcanal.
- In the claims round: with no ship burning, a surface claim of ships sunk equals the truth, against what the briefing promises (a claim "can be well above" it). Decision 0085, 9.
- Base sizes for the real map (decision 0052, item 15).
- The points and levels of decision 0054, and history's result on that scale for scenarios 4, 5 and 6 (`docs/research/oob/history-on-the-scale.md`).
- The objectives scale stays decision 0054's placeholders at the close of step 2, and the comments in scenarios 4, 5 and 6 that say it is "to be set from the record in step 2" stand until it is set: a scenario's bytes are named by the reports and the frozen copies. Step 2 reports the scale and does not judge it (ruled by the architect, Nov 13, 2026).
- **Stage 3's placeholders against the record (Nov 17, 2026).** Research on five subjects has come in and is being checked against its sources into fact rows (`docs/research/oob/stage-3-leads-checked.md`, when its pull request lands). No figure moves until the rows are in, and none moves before stage 3's proof is run: the proof is run on the figures its hand calculations were written on. What the architect then settles, each from rows:
  1. *Reports* (stage 2b, decision 0136): the chance that a report is late or lost and how late, from sighting and receipt times in the record; radio reach between ships.
  2. *The estimate of the enemy ashore* (decision 0147): how far an estimate made before a landing stood from the later count, on each side, which sizes `reconnaissance_error_low` and `reconnaissance_error_high` and the error of an estimate that is only told; and how old an estimate a side acted on, against the three days the "preserve" setting asks.
  3. *Bombardment from the sea* (decision 0146, and the proof's placeholder force, decision 0153): what ships shelled the airfield by night in the weeks of the slice and from where, which sizes the placeholder force at Shortland; and that in the record ships on a troop run also shelled, which the game's one mission for a force does not allow (a named gap, not for stage 3). No figure for the rounds a ship keeps back for a fight at sea was found: the threshold of decision 0146 stays a placeholder with no row behind it.
  4. *Sizing and supplying a landing* (decisions 0142, 0144 and 0149): the men wanted for each enemy man believed ashore (no doctrine read gives a number, so the three settings stay placeholders unless the landings of 1942 give one); the days of supply a landing takes with it, against the 15 the staff sends; the men a destroyer and a transport carried, against the placeholder classes; and the days a landing needed to unload, against the days its cover stayed.
  5. *Carrier cover* (decision 0148): how far from the place the carriers stood, against the rule that takes the first hex within the bombers' reach; and what they flew. The record's cover kept fighters standing over the transports through the day, which decision 0148 leaves out ("no patrol over a beach"): this is the gap of stage 3 most likely to show in the proof, and is the first thing to take up after it.
- **Guadalcanal to 30 November (Nov 20, 2026).** The order-of-battle facts for 15 September to 30 November 1942 are in (`docs/research/guadalcanal-autumn-1942.md`), for the 115-day scenario of `docs/design/larger-scenarios.md`: the heavy ships' arrivals and departures on both sides, the Japanese destroyers by division, the runs to Guadalcanal, the forces and losses of Santa Cruz, the October bombardments, the November actions and Tassafaronga, air strengths by date and the submarine losses. Missing: Japanese-clock times and wreck positions for the November battles, killed and wounded by ship, the Cactus Air Force by squadron, every Japanese air strength from a better source than Wikipedia, the Tokyo Express night by night, and any supply by month. The architect settles its disputes (the clocks and the date of the battleship bombardment, Cactus strength on the days that mattered, the Japanese ships in the November actions, the convoys' loads, the losses at Santa Cruz and of Wasp) and the classes the new ships want before a scenario reads the rows.
- Port rows that no derivation reads: `place_facts.csv` has Noumea's one large berth and 5,000 tons a day in December 1942 (rows 96, 116) and no wharves at Henderson Field or Milne Bay (58, 85). `port_sizes.csv` stays placeholders, read by no battle of the slice.

## Known gaps between the model and the record

Where step 2 ends, battle by battle, with every named miss and agreement at the edge and its cause: `docs/validation/step2-close.md`. The reports are in `docs/validation/`.

## Kept so they are not lost

- Speed made good. A force in the game holds its ordered speed on a straight course. In the record, where a source gives positions and times as well as a speed, a convoy or a ship in transit made good a half to three quarters of its stated speed: it zigzagged, waited, slowed for landfall or towed. Warships on a combat run are the exception (the 6th Cruiser Division from the Shortlands to off Savo Island on 11 October 1942 took what its speeds give for the distance). Noted by the designer on Nov 12, 2026 and not for step 2 (`docs/step2-spec.md`, "Decided by the designer", item 8): taken up when convoys and supply are compared with the record. The simplest answer may be a cruising speed for routine movement below a ship's stated speed, not a new mechanism. The facts are in `data/oob/passages.csv` and `docs/research/oob/passages.md`.
- Before step 3: the staff closing the range on a contact it cannot reach; objectives that name a flow (`docs/design/economy.md`); what the first tuning round of ground combat's stage 4 could not reach (decision 0109, "As built: round 1"), and timing a trial of a plan by hand (`docs/design/planning.md`).
- Refuelling at sea, decision 0201: stage A, the rule on placeholder classes, is built (scenario 115; state version 52, rules label 26, doctrine `supply-51`, agent interface 39). Open: stage B, real oilers (a ship type the derivation maps to an auxiliary, cargo oil read from rows, the mark, and the Cimarron class's missing rows), after which the Coral Sea and Midway are looked at again for their oilers; stage C, warships fuelling their own destroyers; stage D, the staff planning fuel, a decision of its own to be written now that stage A is built.
- At the start of stage 3 of step 3: reconnaissance as part of an operation, by the designer's direction of Nov 15, 2026 (`docs/design/intelligence.md`, "Reconnaissance as part of an operation"): implied by an intent to take a place and planned by the staff as a time of collection; knowledge in grades by how close the source gets; and a reconnaissance mission that can be pushed from covert to in force, with more learned and more risked. Coastwatchers as standing sources wait with it (layer 2).
- For step 3, named by the American losses at the Eastern Solomons (`docs/validation/tuning/r3-us-losses-named-misses.md`, ruled Nov 13, 2026): bombers' return fire at fighters (deferred since decision 0027); a search that meets enemy fighters; an aircraft written off for combat damage; a carrier's deck closed for a time, with aircraft aloft running out of fuel while it is.
- For step 3, named by Rabaul round 2 (`docs/validation/tuning/rabaul-r2-fatigue.md`, ruled Nov 13, 2026): a strike of a given size, so that a staff can send part of a group as the units at Rabaul sent about 18 bombers a raid whatever their rest; in the rules a group sends every aircraft it can fly.
- For step 3, named by the closing legibility test (`docs/validation/closing-tests.md`, finding L6, ruled Nov 13, 2026): a side shown an enemy strike on its ships with its causes hidden is told nothing of why it came; the briefing could say that an enemy strike always rests on a contact the enemy held, without saying which.
- For step 3, the same test, L7: a task force's mission is shown in the briefing; the bombardment mission is read by a rule (decision 0092), and the item holds for the other missions, which no rule reads: show one only when a rule reads it, or say there that it is a label.
- For step 3, named by Rabaul round 3 (`docs/validation/tuning/rabaul-r3-wear.md`, ruled Nov 13, 2026): small airfields that wear aircraft faster (left unbuilt by decision 0035), for the American losses on take-off, landing and ferrying that do not go with distance; and a wear that does not rise in a straight line with distance, since the dive bombers' searches and Rabaul's raids want the distance term opposite ways.
- When the staff's doctrine is next changed (decision 0221, the architect's rulings on the plan's paper, Nov 22, 2026): `propose`'s arcs gain the threats each arc was set for as a field, and each sub-intent gains its fact for ordering nothing. The table's plan paper reads both from the staff's words until then.

## Open from decision 0228 (the staff chooses a source of supply)

Built on Nov 23, 2026 (scenario 132; see the decision's "As built"). Each row is open:

- **The staff does not choose again** when the kept source runs dry: it says so and names the other bases that could be one (rule 17), and the commander names a source.
- **The staff brings no ship to a source.** A source with no transport force waits for one to lie there.
- **The need of a place not yet the side's is 0,** so before a landing every base that can spare anything covers it; the take's own reckoning of what its men will eat is not read.
- **Distance is the fourth key only.** Look at the order again when a rear base stands behind a rear base (Nouméa behind Espiritu Santo, decision 0224).
- **The danger of the way** and **fuel** are not weighed in the choice.
- **Two rules are kept for the choice** (the island's old one for a side without the key), because a closed proof's agent-played record cannot be said again in the new language. Decision 0238 measured the retirement and kept the key: it goes when the slowness below is mended and there is a way to keep `second_proof_week_sonnet` replaying; the cost of that day (29 tests, 34 games) is in that record.
- **A defect: the staff's choice of a source is slow in a trial.** With every side on the one rule a trial of the Allied side on `stage-3-proof/07a_slice.toml` (`sim-cli time`, section h, 16 worlds for 14 days) takes 44.2 s against 5.7 s, and 1.9 s against 0.9 s for 8 worlds and 7 days (decision 0238, Nov 27, 2026). Not traced; the lead is that the bases are weighed, each with a sea path, every morning and not only when a source is chosen. The 20 files with the key on run this code today; whether a trial at the table pays it is not measured. To do: measure a trial on a `table-boats` file with the key and without, trace it, and have the architect class the mend.
- **A run still loading when a commander renames the source** of its supply is read by the supply doctrine as it reads an island's renamed source; it has not been traced.
- **Rule 17's list of other bases** weighs a base before its day and not the sources of orders higher in priority.
- **An order to keep a base supplied given from a base's card** at the table is sent with no source unless the player sets one; only a hold and an island get the staff's choice.
