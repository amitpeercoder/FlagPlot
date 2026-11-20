# 0185: A new game screen: the scenario, the seat and who plays the other side

Status: a plan by the architect, written on Nov 20, 2026, at the designer's asking. Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ) before any of it is built. It is work on the table and its bridge alone.

Summary: A game on the table is started today from a command line that names the scenario, the seat and a dozen options. The designer asked for a screen, shown when the game is launched, on which a person chooses the scenario, the side to take or to watch, and whether the other side is played by the engine or by an agent; and for the page to say how an agent is set up. The bridge is started with no scenario and shows that screen. It adds one kind of game the table cannot hold today: a person at one seat and an agent at the other

Standing: A plan, written by the architect on Nov 20, 2026, at the designer's asking, and amended on review. Not built

## Why

The designer, on Nov 20, 2026: "Let's also create a new game screen when the game is launched, where the user can choose the scenario, which side they'd like to take in it and whether the other side should be played by the engine or by an agent. We also need to explain how to set up an agent."

What is so today, read from the code:

- **The bridge holds one game named on its command line** (`table/serve.py SCENARIO --seat ...`), with the date of day 1, the islands' names, the directives and the map given as options. A game of the 39 days needs seven of them.
- **It has two ways of holding a game.** For a person at a seat it drives `sim-cli mcp` for that seat, and the other side is the engine's: its script, or its staff where the scenario gives it orders. For a watched game (decision 0182) it drives `sim-cli serve --watched`, gives each agent's seat a door with a key, and the page holds the clock and orders nothing.
- **A person at one seat with an agent at the other is neither.** A watched session has it in it: both seats played, each ready or waited for, and a step taken when both are ready. The table has no page for it.
- **An agent takes a seat through its door:** by HTTP, or by `table/seat_mcp.py`, which offers the door to a model as a tool server. The bridge starts no model and holds no key of one. What an agent is told on taking a seat, the game's opening words, is served by the door.
- **The page can begin a game again** from its first evening.
- **What a scenario is called, how long it is and what it is about** is in its file's `name`, its `days` and its comments. Nothing lists the scenarios meant for play.

## The rules

### The screen

1. **The bridge started with no scenario shows the new game screen** in place of a table. Started with a scenario, it does as it does today.
2. **It asks three things, in this order:**
   - **The scenario,** from a list of those meant for play, each with its name, its dates and length, and two or three lines on what it is and what each side is to do.
   - **The seat:** the Allied side, the Japanese side, or to watch.
   - **The other side:** the engine, or an agent. For a game watched, each side: the engine or an agent, and at least one an agent.
3. **A list of scenarios for play is a file of the table's,** `table/scenarios.json`: for each, the scenario's file, its name and lines, the date of day 1, the islands' names, each side's directives where it has them, and which seats may be taken. The screen reads nothing else of a scenario. The list begins with the 39 days and the one-day Eastern Solomons; the five-day scenario joins it when it is built (decision 0184).
4. **"Begin" starts the game** as the bridge would have from a command line, and the page becomes the table. A game is begun again, or another chosen, from the page: "New game" returns to the screen and ends the game in hand, after asking.

### The four games

5. **A person against the engine:** as today.
6. **A person against an agent** (new). Both seats are played. The person's page is the table of his seat, giving orders as it does today. His calls of time are held as an agent's are, and the bridge steps the clock itself whenever both seats are ready: there is no watcher. While the agent is waited for, the page says so, and he may read and may not order, as a seat that has ended its day may not. He is shown nothing of the agent's side and none of its calls.
7. **Watching an agent against the engine,** and **watching two agents:** as decision 0182 has them.

### An agent's seat

8. **Where a seat is an agent's, the screen shows how it is taken,** before the game begins and again from the table at any time:
   - the seat's door, to copy;
   - the one line that offers the door to a model as a tool server (`python3 table/seat_mcp.py --url DOOR`), and the same as the few lines of settings a Claude client takes for a tool server, to copy;
   - a brief to give the model, to copy: that it commands that side, the directive where the side has one, that it plays to the game's end by the tools it is given, that a call that moves time may be long and `wait` goes on waiting, and that it may read nothing but what the tools tell it.
9. **The game does not wait for the agent to begin.** The table opens; the agent's seat shows "not yet at its seat" until its first call, then ready or waited for as today.
10. **The keys are as decision 0182 has them:** one for each door, made at each start, guarding against mistakes and not against a program that sets out to cheat. In a game of a person against an agent the person's page has his own seat's key and no other.

### Said plainly on the screen

11. **What the engine is:** where the scenario gives the other side orders, its staff carries them out; where it gives none, that side's units do what they do by themselves. The screen says which, for the scenario chosen.
12. **No points and no balance are claimed.** The screen says what each side is to do, in the directive's words where there is one.

## What it leaves for later

- **A model started by the bridge.** It needs a key, a budget and a limit on a turn; the bridge still starts none.
- **Save and load,** and a list of games begun.
- **A seat at a command** (a task force or a base) chosen on the screen.
- **Choosing the seed.**

## Rule card

- **Reads:** the table's list of scenarios; what the bridge reads today.
- **Writes:** nothing in the game.
- **Reuses:** the bridge's two ways of holding a game; a watched session with both seats played; the doors and their keys; `seat_mcp.py`; the game's opening words.
- **New:** in `table/`, the screen, the list of scenarios, a person's seat in a watched session with the bridge stepping, and the words on taking an agent's seat. No Rust, no rule, no state.

## What it costs

- **A third way for the bridge to hold a game,** and a page that must hide the enemy's side wholly in it. The table has no test in CI: the bridge's part is checked by script, as the watched game's is, and the page by the architect in a browser.
- **A list of scenarios kept by hand.** A scenario renamed or moved must be changed there.
- **Documents:** `table/README.md`, `docs/commands.md`.

## Choices made for the designer

1. **To watch is a seat on the same screen,** not a separate way in.
2. **A person against an agent has no watcher's clock:** each plays at his own pace and the day is fought when both have finished, as two people would play.
3. **The screen explains an agent by three things to copy** (the door, the line that serves it to a model, and a brief), and starts no model itself.

## Amended on review

An independent reading of the plan, against the code, found the following. Each was checked before it was taken. This section replaces what is above it where they differ.

### What is so today, corrected

1. **The other side is one of three things,** not two: a fixed script, where the scenario's file gives that side orders to its units, which are played as written; a staff carrying out intents, where the file gives that side intents; or nothing, where it gives neither, and then its units do only what they do by themselves. And a scenario's intents are handed in for both sides whoever holds the seats, so a file decides what the person's own side begins with as well. A side given nothing is no opponent.

### The list of scenarios (replaces rule 3)

2. **The list names a file for each game it offers,** since the file differs with the seat and with who plays the other side. For each scenario: its name and lines, the date of day 1, the islands' names, each side's directives where it has them, and for each pairing offered (the person's seat, or watching, and engine or agent on each other seat) the scenario file that makes it a game: one in which the engine's side is given a script or intents, and a side played by a person or an agent is given none. A pairing with no such file is not offered. The 39 days has such files today (`stage-3-proof/07b_slice.toml` for a person on the Allied side against the engine, `07c_slice.toml` for the Japanese side, `staff/07_full_slice.toml` where neither side is the engine's); the list says which the one-day Eastern Solomons has.

### A person against an agent (rule 6, made buildable)

3. **Reading while waiting.** A seat that has said it is finished is refused every call until the clock has moved (decision 0182). So that the person may go on reading, the bridge answers his reads, while his call of time is held, as the watcher's read of his own side and of no other. He cannot preview, try a dry run or approve until the clock has moved.
4. **A call of time cannot be taken back.** Once the person has pressed Next pulse, One day or Carry on, he has finished, and if the agent takes minutes he cannot change his orders meanwhile. The page says so when he presses, in a line, and shows that the other commander is waited for. Taking a call back would need the session changed, and is left for later.
5. **The page's own clock is changed for this game:** a call of time is held and may answer that it waits, and the page goes on waiting for it; its answer, when it comes, is what the same call answers today, and is plotted as today. The page's list of tools is the seat's own.
6. **The door's opening words for this game** say that the clock moves when both commanders have finished, not that a watcher holds it.
7. **A long call holds the other seat's.** The game answers one call at a time: while one commander's dry run or a long step is worked, the other's call waits, and may be told that it waits. The words given to an agent say that any call may be so held, not only a call of time.

### Keeping the sides apart (replaces rule 10, and rule 8's "again from the table")

8. **In a game of a person against an agent the bridge has no watcher.** It refuses every request of a watcher's. The page is given its own seat's reads and calls; a standing that says of the other side only whether its commander is at his seat and whether he is waited for; and its own side's directives. Nothing of the other seat's calls, reasons or stops.
9. **The key is no guard against the person himself.** He sets the agent up, so he has handled its door. The door is shown on the screen where the agent is set up, and from the table only behind "The agent's seat", which says what it is; it is never drawn on the table itself. The record does not claim more than that.
10. **What is kept on disk** with `--keep` holds both seats' transcripts and the journal. That is the umpire's, and is for after the game.

### Setting up an agent (rule 8, made exact)

11. **What is given to copy:**
    - the line that adds the seat to a Claude Code client, with the whole path of the program: `claude mcp add flag-plot-seat -- python3 /…/table/seat_mcp.py --url DOOR`;
    - the same as an entry of tool-server settings, for a client that takes them as a file, with the note that such a client is started again to read it;
    - the door itself, for a program that speaks to it directly;
    - the brief: the side, its directive where it has one, to play to the game's end, and to read nothing but what the tools tell it. The game's own opening words reach the model through the door and are not repeated.
12. **What the person is told beside them:** that a model at a keyboard will stop of itself long before a campaign ends and must be told to go on, or be run so that it goes on; that a model with a shell could read the scenario's file, and asking it not to is the only guard; and that a call may wait up to about fifty seconds before it answers that it waits.
13. **An agent's door keeps its address for as long as the bridge runs,** so that a client is set up once. When a game ends and another begins, every call still held is answered that the game has ended and a new one begun; the agent's next call is in the new game, and its opening words are to be read again.

### The screen and the bridge (rules 1, 4 and 9)

14. **The bridge chooses how it holds a game when a game is begun,** not when it is started. Begun with a scenario on its command line it does as it does today.
15. **"New game" asks first, and then ends the game in hand whoever is at a seat.** Decision 0182's refusal to begin again while an agent has lately called is kept for "begin this game again" and does not bind "New game". What was kept of each game is kept under its own number.
16. **"Not yet at its seat"** is the door's own knowledge: no call has come through it in this game.

### Owed

17. Taking back a call of time; a model started by the bridge; a seat at a command.
