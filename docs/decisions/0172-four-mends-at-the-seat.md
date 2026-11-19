# 0172: Four mends at the seat: a game file that knows its inputs, an escort's charge, a day that failed, and preview numbers

Status: settled by the architect on Nov 19, 2026. Four faults found on a review of the code that plays a seat (`sim-agent`'s session and `sim-cli`'s game file), each checked against the code by the architect before it was written here. No rule of the game changes. To be built.

Summary: Four things at the seat were wrong or unguarded. A game file kept by `play` is replayed from its scenario on every call and does not notice that the scenario or a file it reads has changed since, so a game can come back as another game. An escort's charge is named by the side's own number everywhere but at the area's seat, where the engine's number is read. A day that fails its invariants leaves the session open on a world half advanced. And preview numbers are counted for the session, so one side's previews show in the numbers the other side is given. Each is mended, with its test

Standing: Settled by the architect, Nov 19, 2026. To be built

## What is wrong, read from the code

1. **A game file does not know its inputs** (`sim-cli/src/play.rs`). Its header holds the scenario's path, a seed where one was given, and the doctrine's version. Each call loads the scenario again and replays every call before it. If the scenario, its map or a figures file has changed, or the rules have with the doctrine's version unchanged, the replay is of another game: an order once accepted may be refused, a handle may name another unit, a game not over may be over. A saved game already carries what is needed to notice: the hash of the scenario and of each data file, and the rules label (`save.rs`).
2. **An escort's charge at the area's seat** (`sim-agent/src/session.rs`). A side names its forces by its own numbers (decision 0111). The force an order is for is translated at every seat, and so is the force a join names. The charge of an escort is translated only at the seat of a command (decision 0138, 5); at the area's seat its number is read as the engine's. Where a scenario's numbers are not 1, 2, 3 in order, the charge named is refused as not known, or is another force of the side.
3. **A day that failed** (`sim-agent/src/session.rs`). The engine checks its invariants when it closes a day, after the day's events. When they fail it stops before the day is counted. The session has by then emptied what was handed in and reset the seats; it answers that the game has stopped, and records nothing to make it so. A later call reads the half-advanced world, or tries the day again.
4. **Preview numbers are the session's** (`sim-agent/src/session.rs`). Previews are numbered 1, 2, 3 as they are shown, whoever asked. With a seat on each side, the number a side is given tells it how many previews the other has asked for. Approval is already refused across sides. What a seat is shown must not depend on what it cannot see (`docs/engineering/truth-and-belief.md`; decision 0107, "No truth").

## The mends

1. **A game file is pinned to its inputs.** When a game file is begun its header also records the seed the game is played on, the hash of the scenario file and of each data file it reads, and the rules label, by the means a saved game uses. Before a game file is replayed for another call, these are compared with what is on disk now. Where one differs the call is refused, with the name of what changed, and nothing is written. A game file with no such record (every one made before this) may be read and replayed, as the recorded games are by their tests, and is refused a new call unless the caller says in so many words that it is to be continued unpinned.
2. **An escort's charge is named by the side's number at every seat,** as a join's force is. A number that names none of the side's forces becomes the number no force has, so that an enemy's force and none are refused alike.
3. **A day that fails ends the session's game.** The session records that it has stopped. Every later call that would read or advance the game is answered with the same words and changes nothing. What the operator may still take out says that it is of a game that stopped inside a day.
4. **Previews are numbered for each side,** from 1. A preview is approved by its number among its own side's. The checks of scope and of a changed intent are as they were.

## What the tests are to show

Written by the architect; the build writes the tests to them.

1. **A game file.**
   - A game of scenario 1 begun and given one `end_day`, then called again with nothing changed: it resumes on day 1, not over, as before.
   - The same after the scenario's `days` is changed from 3 to 1 (in a copy made by the test): the next call is refused, names the scenario file as changed, and the game file is byte for byte what it was.
   - The same after a figures file the scenario reads is changed in a copy: refused, naming that file.
   - A game file with no record of its inputs: a reading call replays it; a call that would add to it is refused; with the caller's word it is continued.
   - Every recorded game of the repository replays as it did.
2. **An escort's charge.** A copy of scenario 51 with its two Allied forces numbered 10 and 20. At the area's seat the side lists them as its forces 1 and 2. An escort order to force 1 with force 2 as its charge is queued, and the order the engine records has task force 10 escorting task force 20. With a charge that names no force of the side, it is refused as not known. Scenario 51's own tests pass unchanged.
3. **A day that failed.** In a session whose world is made to break an invariant inside a day (by a means of the test, not of the library), `end_day` answers that the game has stopped. A second `end_day`, a briefing, an order and a preview are each answered the same, and the world's day and its event count are what they were after the failure.
4. **Preview numbers.** Two sessions of one scenario with a seat on each side, the same in everything the Allied seat does. In the second the Japanese seat asks for three previews first. The Allied seat's answers are the same in both, preview numbers and all, and it approves its preview 1 in both. In the second the Japanese seat's previews are 1, 2 and 3.

## What it costs

- **The agent interface** changes in one thing a two-seat session can see: preview numbers. A session with one seat, which is every recorded game, is given the numbers it was given. The build says whether the interface's version must rise by the rule that version follows, and why.
- **The game file's format** gains fields. A game file made before is still read.
- **No state, no draw, no hash, no rule.** No generated file should change; the build says so, or says what did and why.
- **Documents:** `docs/commands.md` for `play` and for the word that continues an unpinned game; the agent's README for preview numbers.

## What is left

The same review named three costs that grow with a campaign's length and are not mended here: the check of whether an interrupt fired reads the side's whole history of events each day of a run; a preview copies the world more than once; and a session keeps every call it is sent. None bites at 39 days. They are written in `docs/open-work.md`.
