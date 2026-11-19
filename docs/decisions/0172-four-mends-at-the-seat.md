# 0172: Four mends at the seat: a game file that knows its inputs, an escort's charge, a day that failed, and preview numbers

Status: settled by the architect on Nov 19, 2026. Four faults found on a review of the code that plays a seat (`sim-agent`'s session and `sim-cli`'s game file), each checked against the code by the architect before it was written here. No rule of the game changes. Built on Nov 19, 2026 (As built, below).

Summary: Four things at the seat were wrong or unguarded. A game file kept by `play` is replayed from its scenario on every call and does not notice that the scenario or a file it reads has changed since, so a game can come back as another game. An escort's charge is named by the side's own number everywhere but at the area's seat, where the engine's number is read. A day that fails its invariants leaves the session open on a world half advanced. And preview numbers are counted for the session, so one side's previews show in the numbers the other side is given. Each is mended, with its test

Standing: Settled by the architect, Nov 19, 2026. Built on Nov 19, 2026

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

## As built

Built on Nov 19, 2026, on the branch `seat-mends`. Every statement of "What the tests are to show" holds: the first in `crates/sim-cli/tests/game_file_pin.rs` (and, for the recorded games, `every_recorded_game_replays_to_its_transcript_and_save` in `crates/sim-cli/tests/mcp.rs`, unchanged), the other three in `crates/sim-agent/tests/seat_mends.rs`. Scenario 51's own tests are unchanged and pass. No state, draw, hash or rule changed, and no generated file: the run baselines, the agent goldens, the recorded games and the pinned hashes are as they were.

The readings the build made that the record did not:

1. **The pin's form.** The header of a new game file gains `inputs`: `seed` (the seed the game is played on: the one given, or the scenario's), `scenario_hash`, `data_files` (each `file` and `hash`, in the order the scenario names them) and `rules_version`. The hashes are made by `save.rs`'s own function, and the files are the ones a saved game records (`Remembered::files`). The state layout's version is not pinned: the record names four things, and a change of layout alone does not make another game.
2. **When it is checked.** Before every new call and before every export of a pinned game: a game whose inputs have changed is not exported either, since its replay would be of another game. The doctrine's version is checked first, as before, so a game file of before doctrines is still refused in those words. The rules version, the scenario file and each pinned data file are compared before the scenario is loaded, by reading the files named in the header, so a scenario changed past loading is still named as changed; after the game is opened the seed and the list of data files are compared too, which can only differ if the files did. The refusal names the scenario file, the data file or the rules version, says the game is not replayed on other inputs, and writes nothing.
3. **The caller's word** is `--unpinned`. A game file with no `inputs` is exported as before; any call on it is refused, naming `--unpinned`; with it the call is made and the file stays unpinned: it is not pinned to today's inputs, which the calls before were perhaps not played on. `--unpinned` is refused on a pinned game file and on a new one. "A reading call" in the tests is an export (`--export-transcript`), since every call `play` answers is added to the file.
4. **The recorded games** are game files with no `inputs` (a test says so). They are replayed by their test through `mcp`, not `play`, and that test is unchanged. They also have no `doctrine_version`, so `play` refused to resume or export them before this record, and still does.
5. **An escort's charge** is translated by `own_force` at every seat, the area's and `area:easier` included, exactly as a join's force is; a number that names none of the side's forces becomes the number no force has, which the engine refuses as not the side's unit, shown as "not known". The tests' scripted player (`tests/support/play.rs`) gives a scripted escort's charge in the side's numbers, as it gives a join's force.
6. **A day that failed.** The stop is recorded for any error of the engine while a day is resolved (the day opened, an order handed in inside it, a pulse, the close), not only a broken invariant: each leaves the world in a state no seat should read. The words now name the day: "the day could not be resolved: the game has stopped inside day N on an internal error; the operator has the details"; the line to standard error names the day too. Every later call of any tool is given the same words, before anything is read, and is still entered in the transcript, as every refused call is. `Session::stopped_inside_day` says it to the runner. What the operator may take out: `play --export-save` is refused, saying the game stopped inside day N; the transcript is written as ever, each answer saying it; the saved game `serve` and `mcp` write at exit carries `stopped_inside_day` (an added key, read by nothing: the saved game's version is unchanged, and `replay` runs the days resolved before the stop, as it runs every saved day, and not the day that stopped). The test's means is the one `crates/sim-agent/tests/leaks.rs` already uses, scenario 12c with an arrival put in the world's starting state that overfills a carrier inside day 1: nothing was added to the library for it.
7. **Preview numbers.** Each side's previews are counted from 1; approval finds the preview by its number and its side, as it did. The words of the tool (`(1, 2, 3 among the previews you were shown)`), the schema's description of `preview` and the agent's schema README say so.

**The agent interface's version** stays 28. Its rule (`crates/sim-agent/src/answer.rs`) is that a field removed or changed in meaning raises it and a field added does not. No field is added or removed; the number's meaning to the seat that reads it, the number by which it approves a preview, counted from 1 over the previews it was shown, is what every session with one seat was given and is given now. Only in a session with a seat on each side did the old number differ, and there it carried the other side's count, which the seat was never to have. The build read that as a leak mended, not a meaning changed; raising the version would change only the `version` of every recorded transcript. The architect may rule otherwise.

## The architect's rulings

Made on Nov 19, 2026, on reading the build.

1. **The seven readings of "As built" stand as made.**
2. **The agent interface version is not raised.** A preview's number means what it meant to every seat that could know what it meant: the number to approve by, counted over the previews that seat was shown. What it carried besides, in a session of two seats, was a count of the other side's, which no seat was told it carried and none should have been given.
3. **Any error of the engine while a day resolves stops the session's game** (reading 6), not a broken invariant alone. The session cannot tell a world it may go on with from one it may not, and it is not for a seat to try.
4. **An export is checked against the pin as a call is** (reading 2): to export a game from changed inputs is to export another game.
5. **The state's layout version is not in the pin** (reading 1). The rules label and the files are; a change of layout with neither is a change that plays the same game.

