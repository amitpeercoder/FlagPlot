# 0183: What the enemy is scored for is shown to both sides

Status: a plan by the architect, written on Nov 20, 2026, at the designer's asking. It changes decision 0054, item 6. Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ) before any of it is built.

Summary: A side has been shown its own mission and objectives and never the enemy's. Seven games of the 39 days played by agents, at the architect's asking on Nov 20, 2026, ended the same way: the Japanese commander bombed the airfield and chased warships, and never went for the transports, because nothing told him the Allied side is scored for every ton it lands. The scoring is a rule of the game, as the levels of victory are, and both players of a wargame read its victory conditions. A side is now shown what the enemy is scored for: the kinds of thing, the places named and what each is worth. It is not shown the enemy's mission, his progress or anything he knows

Standing: Built on Nov 20, 2026, to the plan as amended on review, and ruled on. A person at the table reads the briefing's text whole, the enemy's lines now among them: whether he should is put to the designer

## Why

The designer, on Nov 20, 2026, after four games between agents of different strength: "Show the Japanese commander what the Allies are scored for."

What the games showed (they are no part of the repository: seven games of two agents, kept by the architect's session; the figures are the umpire's results as each game's record gave them). On the 39 days the Allied side's largest objective is supply and fuel landed at Lunga: 25 to 37 of its 58 to 90 points in the last four games. The Japanese briefing lists the Japanese objectives alone. Both of the stronger Japanese commanders said afterwards that the objective was not in their briefing and that it decided the game; neither struck the transports while they unloaded.

What is so today, read from the code and the records:

- **Decision 0054, item 6:** "A side sees its own mission and objectives from day 1, in the briefing, and never the enemy's." `SideView::objectives` gives the side's own, and its comment says "Never the enemy's".
- **The levels and the horizon are already given to both sides** as the scenario's configuration: `SideView::levels` and `SideView::horizon_days`, each with a comment that it is common to both sides and says nothing of the enemy's state.
- **An objective is configuration, not state.** It is a kind (ships sunk, ships crippled or out of action, aircraft destroyed, supply landed at a base, a base open or shut by days, a base held at the end, men lost) with its points and, for some, a base. Nothing in it changes during a game and no rule of the simulation reads it.
- **A side's progress** on its own objectives is counted from what it can see (decision 0054, item 7). The enemy's progress is the enemy's.
- **A mission is a sentence the scenario writes for one side,** and may say what that side believes of the other ("The American carriers are believed to be south or east of Guadalcanal").
- **A seat at a command** is given no objectives: they are the area command's.
- **The table shows a person no points** (decision 0157): a directive from the commander above says what is wanted. The briefing an agent reads lists the objectives with their points.

## The rules

1. **What each side is scored for is a rule of the game and both sides may read it.** This replaces decision 0054, item 6, in part: a side is still shown its own mission and objectives, and is now also shown the enemy's objectives.
2. **What is shown of the enemy's objectives:** each one's kind, the base it names where it names one, and what it is worth (its points, its points a ton, its points for each kind of ship), in the order the scenario gives them. For an objective judged over a horizon, the horizon.
3. **What is not shown:** the enemy's mission; his progress on any objective, known or true; whether a base of his is open now; and anything else of his state. The umpire's result at the end is as it was.
4. **A base an enemy's objective names** is named as the seat names any base: by the side's own number for a base of its own, by the map's for a base on its map. Every base an objective names is one the scenario has, and bases are on both sides' maps; if one is not on the side's map it is named by its place alone.
5. **Where it is shown:** in the briefing's objectives part, after the side's own objectives, under words that say whose they are ("The enemy is scored for:"), in the briefing as text and as data, from day 1 and on every day. Not at a seat at a command, which is given no objectives.
6. **A scenario with no objectives** shows none, as today. A scenario whose enemy has no objective says so in a line.
7. **Nothing else reads it.** No staff, no trial and no rule of the simulation reads the enemy's objectives. A commander reads them and decides.

## What it leaves for later

- **The table.** A person is shown no points. Whether a directive from above should say what the enemy is after, in words, is the designer's and the directives' authors' to settle; the directives are not changed here.
- **A staff that plans against what the enemy is scored for** (a doctrine that hunts transports because supply landed is worth most to the enemy). It would be a rule of doctrine with its own record.
- **A side's belief of what the enemy wants,** as a scenario states it for the trial (decision 0161, rule 8), is a different thing and is not changed.

## Rule card

- **Reads:** the scenario's objectives of the other side.
- **Writes:** nothing.
- **Reuses:** the briefing's objectives part and the way an objective is written for a seat.
- **New:** in `sim-core`, one reading on a side's view that gives the other side's objectives as configuration, beside the levels and the horizon. In `sim-agent`, the enemy's objectives in the briefing. No state, no event, no draw.

## What it costs

- **Truth and belief.** `docs/engineering/truth-and-belief.md` says never to add a method to a side's view that returns enemy truth. This is not truth of the enemy's state: it is the scenario's scoring, fixed before play, of the same kind as the levels and the horizon. The page gains that line, so that the next reader does not take it for a precedent for anything of the enemy's state.
- **No game changes:** no state, no hash, no saved game. The staffs do as they did.
- **Every briefing of a scenario with objectives gains lines.** The agent interface's version rises; the briefing's schema gains the part; the recorded transcripts are remade and differ in their briefings.
- **Documents:** the rulebook's chapter 6 and chapter 1 (what a side knows); the glossary; the agent's schemas and their README; the tool server's opening words, which say today that a scenario gives "your side" a mission and objectives.

## The test

There is no arithmetic. `tests/scenario_102_the_enemys_objectives_shown.rs` states what must hold, on existing scenarios, and is written before the code.

## Choices made for the designer

1. **Both sides are shown the other's,** not the Japanese side alone. A rule of the game is the same for both.
2. **The points are shown,** as a side's own are. Without them a commander cannot tell that supply landed outweighs a carrier.
3. **The enemy's mission is not shown.** It is his orders, and may say what he believes of you.

## Amended on review

An independent reading of the plan and its test, against the code, confirmed that an objective is configuration and that naming a base in one can reveal nothing, and found the following. Each was checked before it was taken. This section replaces what is above it where they differ, and the test's statements are amended with it.

1. **An enemy's objective has a line of its own, not a side's own line.** A side's own objective, as a seat is given it, carries the reader's state: what it knows of its progress, and whether a base of its own is open now. The enemy's is written by a separate, smaller form with no room for either: the kind; the base; for a base open or shut by days, which of the two it is; the points, the points a ton, or the points for each kind of ship; and, for ships out of action, the horizon. The briefing's schema gains a definition for it.
2. **"Open" or "shut" is part of the objective** (rules 2 and 3). The Japanese objective on Lunga is that it be shut; without the word it would read as the reverse.
3. **A base is named by the map's number and name** (replaces rule 4). There is no number of a side's own for a base. Every base a scenario states is on both sides' maps from the start, and the loader refuses an objective that names a base the scenario does not have; only a base a landing forms is off a map, and no objective can name one. So an enemy's objective can name no base a side does not already have on its map, and the case of a base named by its place alone cannot arise.
4. **The reading on a side's view gives the objectives alone,** never the table that holds the mission with them. Like the levels and the horizon, it gives nothing at a seat at a command.
5. **The enemy's objectives are written in words of their own** (rule 5). A side's own are written from its side ("enemy ships sunk", "tons you unload at"); the enemy's are written from the reader's side too: "your ships sunk", "tons the enemy unloads at Lunga", "Lunga is the enemy's at the end", "days Lunga is shut". They come after the side's own objectives and the line of the levels.
6. **In the data** the part is left out where the scenario has no objectives or the seat is at a command, so those briefings are what they were; and where the side has objectives and the enemy none, it is there and empty, and the text says so in a line.
7. **It discloses what the enemy is after, and that is the point.** An objective that he hold Lunga, or keep it shut, says in substance what his mission says. The mission's own words stay his, since they may also say what he believes of the reader. This is no leak of his state.
8. **The table's situation sheet shows the briefing's text whole,** and so shows a side's own points today, against decision 0157's "a person is shown no points"; it will show the enemy's as well. This record does not change the sheet. Whether a person at the table should read either is put to the designer.
9. **Also touched,** beyond what "What it costs" lists: the description of the tool `get_briefing`; the golden briefing of the agent's tests; the paragraph of the recorded games' README that says what they were remade after. The agent's source test bans nothing here and is not changed; it gains one case, that two worlds differing only in the enemy's mission give the same briefing.
10. **The test, corrected:** the scenario with command levels on and objectives is `data/scenarios/stage-2b-proof/05a_eastern_solomons.toml`; a side "with no objectives" keeps its table and its mission and has no objective entries, since the loader requires both sides; on the full slice the Japanese side's objectives name Lunga and Tulagi (corrected on the build's report: this said Lunga alone); and Lunga changes side in the scripted run only where the session hands in the script's orders for both sides.

## As built

Built on Nov 20, 2026, on the branch `enemy-objectives-build`, to the record as amended on review. No statement of `tests/scenario_102_the_enemys_objectives_shown.rs` was changed. Every statement holds as written but one clause of statement 1, below.

**What was built.**

- **`sim-core`** (`src/view.rs`): one reading, `SideView::enemy_objectives`, which gives the other side's objectives alone, a slice of `Objective`, never the table that holds his mission. `None` where the scenario has no objectives and at a seat at a command; an empty slice where the other side has none. Its comment says it is the scenario's scoring, fixed before play, of the kind of `levels` and `horizon_days`, and no precedent for anything of the enemy's state. No other method was added. No state, event, draw or hash changed.
- **`sim-agent`** (`src/objectives.rs`, `src/text.rs`, `src/answer.rs`): the briefing's objectives part gains `enemy_objectives`, a list of `EnemyObjectiveLine` (kind, base, closed, ship points, the horizon for ships out of action, points), a form with no field for progress or for a base open now. The text gains, after the line of the levels, "The enemy is scored for:" and a line under it for each objective in words from the reader's side, or one line that the enemy has none. The agent interface is version 32.
- **`sim-cli`** (`src/mcp.rs`): the tool server's opening words and the description of `get_briefing` say what the enemy is scored for is shown, and his mission and progress are not.
- **Tests:** `tests/scenario_102_the_enemys_objectives_shown.rs`, 15 tests, one or more to each statement, one of them ignored (below); and in `crates/sim-agent/tests/leaks.rs` one case added, that two worlds differing only in the enemy's mission give each side the same briefing, as data and as text, byte for byte, on Eastern Solomons. Nothing the source test bans was changed.
- **Generated:** the version is 32 in every schema; `briefing.schema.json` gains `enemy_objectives` and the definition `enemy_objective`; the agent's golden briefing `05_eastern_solomons_allied_after_day1.txt` gains the four lines; the five transcripts `regame.py` remakes differ only in the version and the new part and lines of their briefings. No run baseline, `save.json`, pinned hash, batch fixture or other golden changed.
- **Documents:** rules chapters 1 and 6, the glossary, `docs/engineering/truth-and-belief.md`, the schemas' README, the recorded games' README, `docs/commands.md`, `docs/open-work.md`, and a note at decision 0054, item 6, as other records note a later one that changes them.

**The statement that does not hold as written.** Statement 1 says "On FS the Japanese objectives name Lunga alone". In `data/scenarios/07_full_slice.toml` the Japanese objectives are ships sunk, ships crippled, aircraft destroyed, Lunga shut (`base_open`, base 1, `closed = true`), Lunga held (`base_held`, base 1), Tulagi held (`base_held`, base 4) and men lost: they name Lunga and Tulagi. (The frozen copy in `data/scenarios/step2-frozen/` names Lunga alone, as "Henderson Field", but it has no land units and Lunga does not change side in it, so it cannot serve statements 3 and 5.) The test of that clause as written is kept and ignored, with its reason; the rest of statement 1, that one of them is that Lunga be shut included, is tested and holds. Amended item 10 says the same of the file, and goes back with it.

**Readings the record did not make.**

1. *The reading's shape.* `Option<&[Objective]>`: `None` with no objectives or at a seat at a command, `Some` of an empty slice where the enemy has none, so the briefing can tell the two apart (amended 6).
2. *The data part* is a field of the objectives part, `enemy_objectives`, after `result`, left out (not `null`) where it is not given, so a briefing without it has the shape it had.
3. *The enemy's line* has the fields of an own line of the same names (`kind`, `base`, `closed`, `ship_points`, `horizon_days`, `points`), with `base` and `closed` `null` where they do not apply and `horizon_days` left out but for ships out of action, as an own line has them; so statement 2 compares field by field.
4. *The words.* A base is written as everywhere else in the text, `base:1 "Lunga"`, so the record's phrases read: "your ships sunk, points for each: ...", "your ships left afloat with propulsion or flight deck out", "your ships out of action, counted by the share of the N-day horizon ...", "your aircraft destroyed", "tons of supply and fuel the enemy unloads at base:1 "Lunga" and does not load back, points for each ton: ...", "days base:1 "Lunga" is shut at the end of the day ..." (or "is open"), "base:1 "Lunga" is the enemy's at the end of the game", "your men lost, by any cause". Each is a line under "- The enemy is scored for:". The line where the enemy has none is "- The enemy is scored for: nothing, this scenario gives the enemy no objectives".
5. *The version* rises to 32 although the schemas' rule says an added field does not raise it: the record says it rises.
6. *Statement 3's play* is a watched session opened as `serve --watched` opens one, with both seats held by the script, so the session hands in the script's orders for both sides; the watcher steps and reads each seat. Lunga changes side on day 1 in it, and there are landings and fights by day 5.
7. *Statement 4's "sentence"* is each part of a mission split at ". ".
8. *Statement 5's "the seat's own tools"* is `list_bases`: Lunga among the Japanese seat's own bases before day 1 and among its enemy bases after, with the same handle and name both times.
9. *Statement 7's "byte for byte but for the version"* is checked as the objectives part having its four fields and no fifth, and the text's OBJECTIVES part its one line, on scenario 1 on two evenings. The golden briefings of scenarios without objectives (6a, 21) did not change when remade.
10. *Statement 8's seat* is task force 1 of the scenario (Task Force 16); the area seat of the same game is shown the enemy's objectives.
11. *Statement 9's copy* takes every `[[objectives.japanese.objective]]` table out of Eastern Solomons; the Japanese seat, with none of its own, is shown the Allied side's.
12. *Statement 10's "what they were"* is the committed run baselines of `sim-cli run` (`crates/sim-cli/tests/run_baseline`): the session's daily hashes equal their hash lines, and the result's totals, margins and outcomes their result lines. Neither Eastern Solomons nor the full slice gives a commander's intent, so no staff gives an order in either and the records compared are empty. A further test on the scenario with command levels (`stage-2b-proof/05a_eastern_solomons.toml`), where the staffs act, plays it to the end read and unread and finds the same orders, hashes and result.

**Grep.** No staff (`sim-staff`), trial (`sim-trial`) or rule of `sim-core` reads `enemy_objectives`, or any side's objectives through a view; the reading is called only by the briefing's objectives part in `sim-agent`. The table (`table/`) was not changed and does not read the field; its situation sheet shows the briefing's text whole and so will show the enemy's lines (amended 8, the designer's).

**Test counts.** `cargo test --workspace --release --no-fail-fast`: 4,439 passed, 0 failed, 7 ignored (one of them statement 1's clause above), over 243 test binaries, read from the log. The first run failed four tests that pin the agent interface's version (`holds_and_preview`, `interrupts`, `play` and `validator` in `crates/sim-agent/tests`); their pin was raised from 31 to 32, as each earlier build raised it, and nothing else in them changed. `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `sh scripts/regenerate.sh --check` and `sh scripts/decisions-index.sh --check` pass.

**Not verified by this build.** An agent reading the new lines through `mcp` end to end; the table's situation sheet, which is not changed and was not opened in a browser; whether the words help a commander decide, which only games will show.

## The architect's rulings

Read on Nov 20, 2026, with the build's core: the one new reading on a side's view, which gives the other side's objectives alone and nothing at a seat at a command, and its words that it is no precedent for anything of the enemy's state; the enemy's line, which has no field for progress; and that the agent's source test gained its one case and bans what it banned.

1. **One clause of the test's first statement was the architect's mistake,** and is corrected by him: on the full slice the Japanese side's objectives name Lunga and Tulagi, not Lunga alone. The test the build left ignored is taken out; the test beside it already shows what the file gives.
2. **The reading tells "no objectives" from "the enemy has none"** (reading 1). Stands: it is what the briefing's two cases need.
3. **The agent interface's version rises** (reading 5), though a field added would not by itself raise it. Stands: every briefing an agent reads has changed, and a recorded game should say so.
4. **A base reads `base:1 "Lunga"`** in the enemy's lines, as everywhere in a briefing. Stands.
5. **The statements on hashes and results are played through a watched session with both seats the script's,** and a further check on a scenario where the staffs act shows the same orders, hashes and result whether the briefings are read or not (readings 6 to 8). Stand.
6. **Not seen:** an agent deciding differently for it. Two games of agents on the 39 days are to be played to see whether a Japanese commander now goes for the transports.
