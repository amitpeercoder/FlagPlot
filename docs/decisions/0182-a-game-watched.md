# 0182: A game watched: agents at the seats, a watcher at the clock

Status: a plan by the architect, written on Nov 20, 2026, at the designer's asking. Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ) before any of it is built. It is built after decision 0181's build, which changes the same files.

Summary: The designer wants to watch a game on the table in which an agent commands a side, against the engine or against another agent. He gives no orders. He holds the clock: next pulse, carry on, stop, and he reads what a side's commander reads. This record gives a game a third party, the watcher, who holds the clock and can read and cannot order. A seat's own tools for moving time become its word that it is ready, and for how long. The table's bridge holds one such game, with a door for each agent

Standing: A plan, written by the architect on Nov 20, 2026, at the designer's asking, and amended on review. Step 1, the session and `serve`, is built and ruled on; step 2, the table, is built, checked by script and not yet read in a browser

## Why

The designer, on Nov 20, 2026: a game on the table of an agent against the engine, and of an agent against an agent. "The agent is giving the orders instead of the player, but the player can choose to Next pulse, to Carry on and to stop the clock so that the player can read." Later, perhaps, the player takes a seat himself, at a task force or a fleet.

What is so today, read from the code:

- **A session holds two seats,** each a player's or the script's (decision 0050). Every call names its seat. A day is resolved when every seat held by a player has called `end_day`; between its `end_day` and the resolution a seat can read and cannot order.
- **Whoever plays holds the clock.** `end_day` resolves a day. `run` resolves several and stops on an interrupt. `advance` resolves pulses and stops on what the side was told (decision 0156). `run` and `advance` are offered only where exactly one seat is a player's.
- **`sim-cli serve` holds one game with either or both seats played,** and speaks JSON lines: one request a line, one answer a line, each with the caller's `id`. `sim-cli mcp` serves one seat. Both open the game the same way, with the staffs and the trial.
- **The table's bridge** (`table/serve.py`) starts `sim-cli mcp` for one seat and passes the page's calls to it. The page is given that seat's tools and the map.
- **A seat's read tools give a side's view and nothing else** (`docs/engineering/truth-and-belief.md`). Some tools that look like reads leave a mark in the session: a preview is recorded, and a trial is kept and numbered.
- **Every call of a seat is in the session's transcript.** A saved game holds the orders and commands handed in, each with its pulse, and replays to the same hashes.
- **Nothing happens in real time** (decision 0050, 10). The same calls give the same game.

## The rules

### The watcher and the clock

1. **A game may be opened as watched.** It then has a third party beside the two seats: the watcher. The watcher holds the clock. He has no seat, gives no order and no command, and is no side.
2. **The clock moves a step at a time, and only at the watcher's word.** A step is one pulse where the scenario has orders inside a day on, and one day where it has not. The watcher's one act is `step`.
3. **A step is refused while a played seat is not ready** (rule 5), naming the seats waited for. So the watcher cannot run time past a commander who has not finished.
4. **Carry on and stop are the host's, not the game's.** "Carry on" is the host stepping whenever every seat is ready; "stop" is the host ceasing to. The session knows only steps. So a game watched slowly and one watched fast are the same game.

### A seat in a watched game

5. **A played seat is ready or not.** It is not ready at the start, and at every stop it asked for. While it is not ready it reads and orders as a seat does today. It becomes ready by the tools it already has for time, with the arguments they have:
   - `end_day`: ready until the next dusk;
   - `run`, where the seat has it: ready for that many days, or until a dusk at which an interrupt of its side fired;
   - `advance`, where the scenario has orders inside a day: ready for that many pulses, or until the end of a pulse in which its side was told one of the things the call names, or a dusk at which an interrupt fired.
   In a watched game `run` and `advance` are offered to every played seat, whatever the other seat is.
6. **The call is answered when its time has passed.** The seat's call is held, and the seat can make no other call meanwhile. When the clock has stepped to where the call's stop falls, or the game is over, the call is answered as the same tool answers today: what was resolved, where the clock stands, and why it stopped. To an agent a watched game is the game it knows, played slower.
7. **Each seat's stop is its own.** After each step the session reads, for each ready seat, whether its stop has come, from that side's own events and interrupts, as `advance` and `run` read them today. A seat whose stop has come is not ready again; the other stays ready. The next step waits for it alone.
8. **What a seat may do at a stop** is what it may do there today: at dusk, everything; at 0600 and 1200, the seven kinds of order and the commands that are queued for dusk (decision 0156, rules 3 and 6).
9. **Orders are taken in as they are today:** at the start of the pulse that follows, the two sides' in the order intake has for them. Which seat spoke first changes nothing.

### What the watcher reads

10. **The watcher reads as a seat's commander reads.** He names a side and one of its seat's read tools, and is answered as that seat would be: the briefing, forces, bases, contacts, an entity, a measure, an explanation, events, the command state, plans, orders, the report, and the estimates the seat has been shown. He is given no tool that changes the game or leaves a mark in the session: no order, no command, no preview, no trial, no acknowledgement, and none of the tools of time.
11. **He may look from either side,** one at a time. He is not playing, so seeing both breaks no rule of truth and belief; nothing he reads reaches a seat. He is given nothing a seat is not: no truth of the enemy, which stays with the replay viewer.
12. **His reading leaves no trace:** it is in no seat's transcript, changes no state and no hash, and does not change what a seat's preview or trial is numbered.
13. **He is told where the game stands:** the day and the moment; for each played seat whether it is ready and until when, or that it is being waited for; and whether the game is over.
14. **He is shown what each agent did:** a seat's calls since the last stop, each with its tool, its arguments and whether it was refused, from the transcript. A call's `reason`, where the tool takes one, is the agent's own words for why.

### The host and the table

15. **`sim-cli serve` gains `--watched`,** and two kinds of request beside a seat's call: the watcher's `step`, and the watcher's read, which names the side watched. A held call's answer is written when it comes due, with its `id`; an answer line may so follow a later request's. It also answers, for a seat, the definitions of its tools and the words an agent is given on taking the seat, as `sim-cli mcp` gives them.
16. **The table's bridge holds one watched game:** `table/serve.py --watch`, with `--seat` given for each side an agent plays. It starts `sim-cli serve --watched` and offers:
    - to the page: the watcher's reads, the standing of rule 13, and the clock: next, carry on, stop;
    - to each agent: a door for its seat alone, by which it lists its tools and makes its calls. A call of time is held open until it is answered.
17. **An agent is whatever makes the calls.** A small program beside the bridge, `table/seat_mcp.py`, offers one seat's door as a Model Context Protocol server, so that a model with tools can take the seat as it takes `sim-cli mcp` today. The bridge starts no model and holds no key.
18. **The page in a watched game** shows one side at a time as its commander sees it, with a switch between the sides where both are agents'. Nothing on it gives an order: the order sheet, the approve and cancel buttons and the leave slips' actions are not offered. It shows who is being waited for, and each agent's orders and commands of the day as they are given. Its clock is three controls: next pulse, carry on, stop.
19. **A game against the engine** is the same with one agent: the other seat is the script's or the staff's, and is always ready.

### Kept

20. **On leaving, the host keeps** the saved game and each seat's transcript, as `serve` does today. The saved game replays without a watcher to the same hashes: who held the clock is no part of it.

## What it leaves for later

- **The watcher taking a seat.** A person at a task force or a fleet while an agent holds the area command is two seats on one side. A session has one seat a side today (decision 0138). It needs its own record: who may order what, and what each is told.
- **A model started by the host,** with a key, a budget and a limit on a turn's length.
- **An agent's turn inside a pulse shown as it happens.** The page shows a seat's calls when they are made; it does not show a model's thinking.
- **Two people at two tables.** The doors would serve, but nothing here is made for it.
- **Save and load in the middle of a watched game.**

## Rule card

- **Reads:** what the seats' tools read today.
- **Writes:** nothing new in the game. In the session: whether it is watched, and each seat's readiness with the stop it asked for.
- **Reuses:** the session with two seats; `end_day`, `run` and `advance` with their arguments, their stops and their answers; a day in parts (decision 0156); the transcript; the saved game; `serve`.
- **New:** in `sim-agent`, a watched session: the step, readiness, the held answer, and the watcher's read. In `sim-cli`, `serve --watched`. In `table/`, the bridge's watch mode, the seat's door and the page's watching. Nothing in `sim-core`.
- **Draws:** none.

## What it costs

- **No game changes.** No state, no draw, no hash. A game not opened as watched is as it was, with the same tools listed.
- **The agent interface** is unchanged in its tools and answers. Whether its version rises is the build's to report: nothing a seat is given changes shape.
- **The page** gains a mode. The table has no test in CI; the session and `serve` are tested.
- **Documents:** the rulebook's chapter 1, on the day and what a side knows; the glossary ("watcher", "watched game", "ready"); `docs/commands.md`; `table/README.md`; `docs/engineering/truth-and-belief.md`, which gains the watcher beside the umpire's tools.

## The test

There is no arithmetic in this record: it changes who moves the clock and nothing that happens when it moves. Its test, `tests/scenario_101_a_game_watched.rs`, states what must hold, on scenario 78 (orders inside a day) with both seats played, and is written before the code.

## Choices made for the designer

Each is the architect's, inside what was asked, and can be turned.

1. **The watcher sees what a commander sees, from either side,** and not the truth. It is what the table already draws, it is what he would see from a seat later, and the truth has its own viewer.
2. **An agent may ask to be woken inside a day or only at dusk,** as a player may today. An agent that asks for every pulse is slower and costs more; that is its choice.
3. **The clock never moves past an agent that has not finished.** A slow agent is waited for, and the page says who.
4. **The bridge starts no model.** The first games are played by agents this session starts, each given a seat's door.

## Amended on review

An independent reading of the plan and its test, against the code, found the following. Each was checked before it was taken. This section replaces what is above it where they differ, and the test's statements are amended with it.

### What is so today, corrected

1. **A command carries no pulse** in a saved game; only an order given inside a day does. And a seat that has ended its day can still ask for a preview, which leaves a record: "can read and cannot order" is not wholly so today.

### The seat's tools of time, in two parts

2. **Each of `end_day`, `run` and `advance` resolves time inside itself today,** so none can be reused whole. In a watched game each is split: **the call** checks its arguments, sets the seat's stop and makes it ready; **the answer** is made when the stop has come. A call refused for its arguments is answered at once, and the seat is not ready.
3. **Readiness is a field of its own** beside a seat's "ended", which is reset at every dusk and cannot say "ready for three days". With it the session holds what the answer is made from, as the tools gather it today inside their loops: where the clock stood at the call, the count of the side's events before each pulse, the last dusk closed, and for `run` the staff's orders worked before the first day.
4. **What goes in when.** A seat ready across several dusks has its queued orders and commands handed in at the first dusk only, as `run` has it today. `run` asked at 0600 counts the rest of that day as its first day, as today.
5. **An interrupt stops the seat that set it and no other.** No stop needs both seats.
6. **If the engine stops on an error,** every held call is answered with the words of the stop, as every call is today.
7. **When two seats' stops come on one step** the Allied seat's answer is made first.

### A held seat

8. **This changes decision 0050, rule 9, in a watched game only.** There a seat that has ended its day may still read. In a watched game a seat that is ready makes no call until it is answered: an agent's call of time is one call that takes long. A call from a held seat is refused, saying it is ready and waits for the clock; it changes no state, and is in the transcript as every refused call is.
9. **A held call's entry in the transcript** is written when it is answered.
10. **"As the same tool answers today"** (rule 6) means: as it answers in a game with that seat alone played, the same calls made. With two seats played and no watcher, `end_day` answers at once with the day not resolved; that is not the measure.

### Intake, and what a seat's answers read

11. **Inside a day the script's orders are taken in before the played seats',** and of the played seats' the Allied first. So which played seat spoke first changes nothing (rule 9), but a side played by a seat is not the same game as that side scripted where the script orders inside a day. The test does not claim it is.
12. **A seat's previews of the coming day are worked on a copy of the game with both sides' queues handed in.** Whether any answer a seat is given can differ with what the other seat has queued is not known. The build tests it: each read a seat has at dusk, with the other seat's queue empty and full, must be the same byte for byte. If one is not, the build stops and the architect is told before any game of two agents is played: it would be a leak between seats that is there today.

### What the watcher is given

13. **His reads, by name** (rule 10): `get_briefing`, `list_forces`, `list_bases`, `list_contacts`, `get_entity`, `measure`, `explain`, `events`, `command_state`, `plans`, `staff_orders`, `propose`, `order`, `why`, `report` and `trials`. Not `preview` and not `trial`, which leave a record.
14. **Three more requests of his own** (rule 15): where the game stands (rule 13); what a seat did since its last stop (rule 14); and the step, whose answer says what was resolved: the day, the pulse, where the clock now stands, and which seats' stops came. What each side was told in the pulse he reads with `events`, as that side.

### Keeping the sides apart at the bridge

15. **Each door has its own key.** The bridge makes a key for each seat's door and one for the watcher when it starts, and prints them. A call without its door's key is refused. The page is served the watcher's key. This guards against a seat's agent calling the other seat's door or the watcher's by mistake. It is not proof against an agent that sets out to cheat on the same machine, and the record does not claim it.
16. **Whoever starts the agents gives each its own door and key and nothing else,** and passes nothing the watcher reads to any agent. Where this session starts them, each is a separate agent told only its seat.

### The host

17. **A call of time held long.** A tool call that waits while the watcher reads may outlast a client's patience. The door answers a held call after a set wait with "still waiting", and offers the seat one tool of the door's own, `wait`, which holds again and gives the answer when it has come. The session makes the answer once and keeps it until the door collects it.
18. **`serve` answers a seat's tool definitions and opening words,** which only `mcp` has today; they are moved to where both can use them. The bridge reads `serve`'s lines on a thread of its own and matches each answer to its call by `id`.
19. **Leaving in the middle of a day:** the saved game says it stopped inside a day, as decision 0172 has it. The bridge takes a folder to keep the saved game and the transcripts in.
20. **No restart in a watched game** while a seat's agent is connected.

### The page

21. **Its clock is drawn from the step's answer and from `events` read as the side watched,** not from the answer of `advance`, which it no longer calls.
22. **Carry on stops on what the watcher ticks,** read from the side watched: the same list a player has today (a contact, an attack, a loss, a strike, a landing). It is the page's own stopping and no rule of the game.
23. **Switching the side watched** changes the heading, the words for a base taken and the directives shown to that side's. A side with no directives file shows none.

### Two steps, two pull requests

24. **Step 1: the session and `serve`,** with both seats, and the test. **Step 2: the bridge, the doors, the seat's program and the page.** A game of one agent against the engine can be watched once both are in; so can a game of two.

### The test, corrected

25. Scenario 78 has no Japanese unit and nothing to stop on, and scenario 5 is one day. The statements that need two sides or a stop are made on `data/scenarios/orders-inside-proof/05_eastern_solomons_on.toml`; the one that needs days without orders inside a day on `data/scenarios/01_transit.toml`. The session is opened through `sim-cli`'s library on the live figures, as `serve` opens it; the function that opens a game moves there from the binary.

## As built: step 1

Built on Nov 20, 2026: the session and `serve --watched`, with both seats, and the test. Step 2 (the table's bridge, the doors, the seat's program and the page) is not built.

### What was built

- **In `sim-agent`** (`session.rs`): `Session::watched`, and in a watched game `seat_call` (a call of time held, or an answer now), `step`, `watch` (the watcher's read), `watcher_tools`, `standing` and `seat_calls`. `WATCHER_TOOLS` (`record.rs`) is amended 13's list. Each of `end_day`, `run` and `advance` is split into its call (`run_wait`, `advance_wait`), what each pulse or day closed does to it (`run_after_day`, `advance_after_pulse`) and its answer (`end_day_answer`, `run_answer`, `advance_answer`). The game not watched calls the same parts in its own loop, so the two answer alike by construction. `run` and `advance` are offered to every played seat of a watched game. `Session::call` in a game not watched is as it was.
- **In `sim-cli`:** the function that opens a game (`game::open`, and `game::open_loaded` for a scenario already loaded) and the saved game (`save`) moved into the library. `serve --watched` and its requests (its doc comment and `docs/commands.md` give the shapes). `mcp::instructions_for` gives a seat's opening words to `mcp` and to `serve`.
- **The test:** `tests/scenario_101_a_game_watched.rs`, 22 tests over statements 1 to 19 and 21, and `crates/sim-cli/tests/serve_watched.rs`, 2 tests, for statement 20. Every statement holds as written but statement 5, read as reading 11 below says.
- **No game changes:** no state, no draw, no hash, no golden, saved game, baseline or recorded transcript. The agent interface's version is not raised: nothing a seat is given changes shape. `crates/sim-agent/tests/leaks.rs` is unchanged and passes.

### Statement 13, the tripwire

It holds. On ES at dusk, each read a seat has (the watcher's sixteen where the seat has them, with arguments, and two previews), asked with the other seat's queue empty and holding three orders and a leave (each seat's own intent queued in both games), is the same byte for byte, both ways. ES has no trial, so `trial` was not among them.

### Readings the record did not make, for the architect to rule on

1. `run`'s staff orders for its first day are worked at the call, not at the step that opens the day. With statement 13 holding, the other seat's later queue does not change them.
2. A held call's transcript entry is placed when it is answered (amended 9) and keeps the days resolved when it was made, so a watched seat's transcript is the plain game's (statement 1 checks this).
3. The watcher may read from a side whose seat is the script's, as its seat would be answered.
4. A seat's calls "since its last stop" are those after its last call of time answered; the call it holds now is not in the transcript yet and is shown beside them as `held`.
5. The step's answer is `{kind: "step", resolved: {day, pulses}, clock, stops, game_over, summary}`; `pulses` lists the three where a step is a day.
6. A step is refused, in this order: the game not watched, stopped on an error, over, a played seat not ready.
7. In a watched game `end_day` after the game is over is answered at once (`resolved` false), as today; `run` and `advance` are refused, as today.
8. `Session::call` on a watched session holds a call of time as `seat_call` does and answers it with an error that says so; `serve` uses `seat_call`.
9. The new requests of `serve` exist only with `--watched`. Without it each is answered as any request that is not a call was, by the old error. A seat's `tools` and `instructions` are refused for a seat the script holds.
10. Only a watched game left inside a day is saved with `stopped_inside_day` (amended 19). A game not watched, left at 0600, is saved as before (no key), so that it behaves exactly as it did; the same is arguably true of it, and is the architect's to rule.
11. **Statement 5.** On ES the Allied side is first told of a contact in the afternoon, the last pulse of the game. `advance` reads the end of the game before what a side was told, so the plain answer says `stopped: "game_over"`, with the contact under `told`. The test holds the rest as written: answered after the step of that pulse and not before, and byte for byte the plain game's answer. The test gives the seat the scenario's own Allied orders of day 1; other orders were not tried.
12. Statement 9's interrupt is the standing one of a plan waiting for leave: a cover of Noumea set on S78, whose plan waits for the commander's leave to launch from the dusk before day 1, so `advance` stops at the dusk after day 1.
13. Statement 10's seat at a command is on scenario 62 (`62_reports_between_commands.toml`, read and not changed): none of the three scenarios has command levels on.
14. Statement 17's engine error is made as `leaks.rs` makes one: ES with an arrival in its starting state that overfills the Enterprise inside day 1, the world built in the test and opened through `game::open_loaded`.
15. Statement 18's replay reads the saved game with `sim_cli::save::read` and replays it in the test as `replay` does; `serve_watched.rs` also runs `sim-cli replay` on a watched game's save.
16. Statement 21's recorded games are replayed in the test through a session opened by `game::open`, each call's answer against its kept transcript; the harness in `crates/sim-cli/tests/mcp.rs` runs as before.
17. The watcher's documents (`step`, `standing`, `seat_calls`) carry the interface's format and version and have no schema under `crates/sim-agent/schemas/`: no seat is given them.

### Not verified

- Anything of step 2: no bridge, door, key or page exists, and no agent has played a watched game.
- A seat at a task force or a base in a watched game beyond its list of tools.
- A watched game with a trial: none of the three scenarios names a trial's figures.

## The architect's rulings on step 1

Read on Nov 20, 2026, with the build's core: the watcher's read, which goes through a seat's own tools and only those of his list, and leaves no entry in a transcript; that `sim-core`, `sim-trial` and the agent's source test are untouched; and that the statements in `tests/scenario_101_a_game_watched.rs` are as the architect wrote them.

1. **A seat's answers do not read the other seat's queue** (statement 13). It held, both ways, for every read a seat has at dusk and for two previews. No leak between seats was found. A dry run was not among the reads tried, since the scenario used has none: it is to be tried on a scenario with a trial before a game of two agents is called fair. Owed.
2. **Statement 5 as tested stands.** On the scenario named, the first contact comes in the last pulse of the game, and `advance` says the game is over before it says what the side was told. The test holds what matters: the call is answered at that step and not before, with the plain game's answer byte for byte.
3. **Each tool of time is a call, a check and an answer,** and the game not watched uses the same three in its own loop. Stands: the two cannot drift apart.
4. **`run`'s staff orders are worked when the call is made.** Stands, as `run` does today.
5. **A held call's entry in the transcript** is placed when it is answered and keeps the days resolved when it was made. Stands.
6. **The watcher may read a side whose seat is the script's.** Stands: it is what watching an agent against the engine needs.
7. **The watcher's requests exist only in a watched game.** Stands.
8. **Only a watched game left inside a day is saved as stopped inside a day.** Stands.
9. **The scenarios the build chose** for a seat at a command, an interrupt and an engine's stop stand.
10. **The watcher's answers have no schema, and the agent interface's version is not raised.** Stands for step 1: nothing a seat is given changed shape. Step 2 is built against the shapes in "As built: step 1"; if the page needs them fixed, they are given schemas then.

## As built: step 2

Built on Nov 20, 2026: the table's bridge in watch mode, a door for each agent's seat, the seat's program and the page's watching (rules 16 to 18, amended 15 to 23). No Rust changed.

### What was built

- **`table/serve.py --watch`,** with `--seat` once for each side an agent plays, the other the script's. It starts `sim-cli serve SCENARIO --watched --seat SIDE=player ..`, reads its answer lines on a thread of its own and matches each to its request by `id`. At start it makes a key for each agent's door and one for the watcher and prints the watcher's page, `http://127.0.0.1:PORT/watch/KEY/`, and each door, `http://127.0.0.1:PORT/seat/KEY`. Without `--watch` it is the bridge it was, driving `sim-cli mcp`.
- **A door** answers `GET /tools` (the seat's tools from `serve`, and `wait`), `GET /instructions` (the seat's opening words from `serve` and a paragraph on the watched clock) and `POST /call` with `{tool, arguments}`, answered `{"ok": ..}` or `{"refused": .., "allowed": ..}`. A call not answered within `--hold` seconds, 50 unless given, is answered `{"waiting": true, "tool", "words"}`; the door keeps the call and its `wait` holds again and gives the answer once it has come (amended 17).
- **The watcher's address** serves the page and, under it, the config, the map, the watcher's tools from a side, a read (`POST /call` with `{tool, arguments, side}`, passed as the watcher's read), the standing, a seat's calls, the step and the restart.
- **`--keep DIR`** passes `--save` and `--transcript` to `serve`; on leaving (Ctrl-C or SIGTERM) the bridge closes `serve`'s input, `serve` writes the saved game and the transcript, and the bridge writes each seat's entries to a file of its own.
- **`table/seat_mcp.py --url DOOR`:** a Model Context Protocol server on standard input and output (`initialize`, `ping`, `tools/list`, `tools/call`) that passes each call to the door. The briefing is text unless asked for data, as `sim-cli mcp` has it. Standard library only.
- **The page,** when the bridge says the game is watched: the heading names the commander watched; a switch between the sides where both are agents'; the order form, the direct orders, the air group's order, forming a force, the release, the leave's switch, approve and cancel, the risk and the withdrawal, the dry run, the what-ifs, the "do not stop me" and "give it" buttons are not offered; the reads go to the watcher's reads from the side watched; a slip for each agent's seat, ready and until when or waited for, with the calls of the side watched since its last stop (tool, arguments in the table's words where it has them, refused or not, and the `reason`) and the call it holds; the clock is Next pulse (Next day where a step is a day), Carry on and Stop.
- **`table/check_watch.py`,** the check by script, below.

### Readings the record did not make, for the architect to rule on

1. **The watcher's key is in the page's address,** not written into the page: the page finds it in its own path and puts every request under it. The page is not served without the key (`/` says the game is watched), and neither are the config (it holds both sides' directives), the map or the reads.
2. **Carry on's loop is the page's.** It asks the standing; while a seat is waited for it says who and asks again every 1.5 s; when none is, it steps, plots the step from `events` read as the side watched, and stops on a thing ticked, the game's end or the watcher's Stop. The bridge passes steps and holds no loop. Closing the page stops it. Next pulse with a seat waited for says who and does not step.
3. **The page reads what the side was told from its events,** not from `advance`'s `told`, which it no longer calls (amended 21): by the event kinds `told_as` names, and, for two of them, by the event's text (a report of a new contact; propulsion or flight deck out).
4. **The door holds any call** not answered within its wait, not only calls of time: a long dry run would be held and collected by `wait` the same way.
5. **A call while a held call is unanswered is passed to the session,** which refuses it and enters it in the transcript (amended 8). A call after the held call's answer has come and before `wait` collects it is refused by the door, saying to call `wait`, and is in no transcript; so is `wait` with nothing held.
6. **The watcher looks only from a side an agent plays.** The session lets him read the script's side (ruling 6); the bridge refuses it, as rule 18 offers the switch only where both sides are agents'. Widening it is one line of the bridge.
7. **The watcher's tools from a side are taken from the refusal of a read naming no tool,** whose `allowed` lists them: `serve` has no request that lists them.
8. **The slip lists the calls of the side watched;** of the other agent it shows only whether it is ready. Its calls are seen by switching side.
9. **Begin again** in a watched game is refused while an agent has called its door in the last five minutes (amended 20). The keys are kept; the new game's files are kept beside the first, as `save-2.json` and so on.
10. **The transcript is one file from `serve`;** the bridge keeps it and writes one more file a seat (`transcript_allied.jsonl`, `transcript_japanese.jsonl`).
11. **Directives in a watched game** are given as `--directives allied=FILE` and `--directives japanese=FILE`; a bare file is the first agent's.
12. **Calls held when the bridge is left** are answered that the game's server has stopped.

### The check by script

`python3 table/check_watch.py`, on ports 8760 to 8762, with the door's wait shortened to 3 seconds. On `orders-inside-proof/05_eastern_solomons_on.toml` with both seats agents', and `--keep`: wrong keys refused at a door and at the watcher's address, the page not served without the key; each door's tools with `wait` and its words; reads and an order at each seat, with a `reason`; the watcher refused `issue_orders`, `preview` and `end_day`, and reading the Japanese briefing; a step refused naming both seats, then both ready; the watcher shown the Allied order with its reason and its held `advance`; the Allied `advance` answered after the night's step in its open call; the Japanese `end_day` answered "waiting" after 3 s, a read from the held seat refused, the next step waiting for the Allied seat alone; the morning and the afternoon stepped, the game over with both stops; the Japanese `wait` giving its `end_day` answer; `wait` with nothing held refused; restart refused; on leaving, the saved game and the three transcripts kept, each seat's holding its own calls, the watcher's reads in none; the saved game replayed by `sim-cli replay`. On scenario 78 with the Allied seat an agent's and the Japanese the script's: one door printed; the watcher refused the script's side; an order queued; `seat_mcp.py` answering `initialize`, `tools/list` (with `wait`), `tools/call` of `get_briefing` as text and a refusal as an error result; three pulses stepped and the `end_day` answered at dusk and not before; the day's events read as the Allied side. The bridge without `--watch` serving the page, its tools, `get_briefing` and `end_day` through `/call` as before. Every line held.

### Not verified

- **The page in a browser.** Its script parses, and nothing else of it was run. To be looked at: the watched heading and the side switch (and that switching changes the heading, the words for a base taken and the directives, and that a side with no directives file shows none); that no control that orders shows anywhere (the signals, the force, base and air group cards, the plan sheet, the order form, the leave's switch); the seat slips and the calls' words; Next pulse, Carry on and Stop, the plot of a pulse and of a day, the wait for a seat, and Carry on stopping on what is ticked; the directives kept apart for each side across a reload; Begin again.
- **A game played by models.** The doors were called by scripts only.
- **A watched game with a trial,** and the dry run among the reads of the tripwire (ruling 1, owed).
