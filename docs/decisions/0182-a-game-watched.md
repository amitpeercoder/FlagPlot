# 0182: A game watched: agents at the seats, a watcher at the clock

Status: a plan by the architect, written on Nov 20, 2026, at the designer's asking. It is to be read by a reviewer who has not seen it written, and amended, before any of it is built. It is built after decision 0181's build, which changes the same files.

Summary: The designer wants to watch a game on the table in which an agent commands a side, against the engine or against another agent. He gives no orders. He holds the clock: next pulse, carry on, stop, and he reads what a side's commander reads. This record gives a game a third party, the watcher, who holds the clock and can read and cannot order. A seat's own tools for moving time become its word that it is ready, and for how long. The table's bridge holds one such game, with a door for each agent

Standing: A plan, written by the architect on Nov 20, 2026. Not yet reviewed and not built

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
