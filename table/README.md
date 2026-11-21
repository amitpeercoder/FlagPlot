# The table

A prototype of the player's interface: a chart table seen from the commander's chair, drawn in the browser and played against a real game. It is a prototype, kept to find out what the interface should be; it is not a stage of step 3 and nothing depends on it. Decision 0154 has what it is and what it found.

It plays the game. It is not the umpire's: it is given one seat's tools and nothing else. It never reads a replay, a save, a batch file or the event log.

## Run it

From the top of the repository:

```
cargo build --release -p sim-cli
python3 table/serve.py
```

and open the address it prints, `http://127.0.0.1:8742/table/KEY/`. That is the new game screen (decision 0185, below): choose the scenario, the seat and who plays the other side, and begin.

A game can still be named on the command line, and the bridge then holds that game alone, as it did before the screen:

```
python3 table/serve.py data/scenarios/stage-3-proof/07b_slice.toml --seat allied --start 1942-08-07 --island "1=Guadalcanal@1" --island "2=Tulagi@4"
```

Then open http://127.0.0.1:8742/. That scenario is the 39 days of the staff's setup with the Japanese side's intents stated and the Allied side given nothing, so the Allied seat states its own.

`--start` is the date of day 1, for the clock. `--island` names an island the scenario has (its number, a name for the order form, and the number of a base on it), because no tool lists a scenario's islands yet. `--directives` is a file of directives from the commander above, as `table/directives/` has them (decision 0157); without it the table shows none. `--map` is the map file to draw (the default is the slice's); it is used for a game named on the command line, and at the new game screen before a game is begun. A game begun from the screen is drawn on the map its scenario names (see "The chart's coastline" and `scenarios.json`). `--port` is the port. The page loads its drawing library and its fonts from the network.

## What is in it

- `serve.py`: the bridge. Started with no scenario it serves the new game screen and holds the game begun from it. Started with one, it starts `sim-cli mcp SCENARIO --seat SIDE`, passes the page's tool calls to it one at a time, and serves the page and the map's hexes; with `--watch`, a watched game (below). Python's standard library only. It listens on this machine alone.
- `scenarios.json`: the scenarios the new game screen offers, and the file for each game offered.
- `seat_mcp.py`: one seat's door as a Model Context Protocol server.
- `check_watch.py`: the check of a watched game by script. `check_screen.py`: the check of the new game screen and its games by script.
- `index.html`: the page. One file, no build step.
- `directives/`: the directives a seat is given from the commander above, one file a seat of a scenario, written by hand in the game's own words. `guadalcanal_allied.json` is for the Allied seat of the 39 days: add `--directives table/directives/guadalcanal_allied.json` to the command above.

## What it does

- Wherever a force, a base, a ship, an air group, a land unit or a contact is named, in the report, the plan sheet or a card, the name is a link: it opens that thing's card and turns the chair toward it. The page's address ending `#see=task_force:1` does the same (decision 0158).
- "Read the report" at dusk is the staff's evening report, one sheet in place of the long return: where things stand, what happened since the last evening, what is for the commander's decision or notice, and what comes this week. A part with nothing to say is left out. It is made from the briefing's figures, the command state and the schedule; the full return is one step from it.
- "Read the plan" on an intent of the commander's opens the plan sheet: the intent, its risk and leave, and under it the staff's own intents as a tree, each with its plan and state, how it is going, what the staff decided for the coming day and why, what binds that decision and what would change it. A plan that waits can be approved there and a live one cancelled; the risk can be set and the intent withdrawn. A plan is changed by instruction, never line by line. The sheet is brief-first: one line a task with a word for where it stands, and the staff's reasons one step down under each. Above the brief, from the tool `order` (decision 0159, rule E), the plan as the staff hands it in: its phases in order with the one under way marked, what it waits for with the day it is expected (in the enemy's colour while unmet), the men wanted and to hand, and the day it expects to be ashore; a refusal shows nothing. Under a take whose plan is preparing, one line says what must be known before it goes (decision 0159, 13), from the plan's `need_to_know`: what is asked and by what (the knowledge limit or the risk), what is held with its age and what it could be in truth, who is looking and since when, the carrier force moved to look and its hex where there is one (decision 0159, item 14), and whether it is met; it is in the enemy's colour while unmet. It sets no knowledge limit.
- When the game is over the page opens "How it ended", once, and the umpire's slip opens it again: who won and by what name (a minor, major or decisive victory, or a stalemate), then what each side did in two columns (enemy ships sunk and left out of action, aircraft destroyed, men lost, supply landed, days an airfield was open or shut, places held at the end), the scenario's note on the war itself where it has one, and the umpire's points one step down. In a watched game the result is read from the final briefing of the side watched. A game with no objectives says so.
- "Order a force" has a "Fuel" row (decision 0201), offered only where another force of the side has a ship whose class fuels others at sea (an oiler, as the force's ship lines say): tick it and choose the force to fuel from. It cannot be signed with a course, which ends the fuelling. The diary and the plot say, in the game's words, what each ship took, when fuelling was broken off and why an order ended.
- "Dry run" on the plan sheet, in a game that has a trial (decisions 0178 and 0181): the order is played through by the staff at the size chosen, plain, beside the side without the order, or at later dates the staff picks. While it runs the buttons are off and a line says how many runs of how many days are played and the seconds gone. The estimate opens as its own sheet, brief first: in how many runs the aim was met and held and by what day, what it may cost each side as lowest, middle and highest, and where the runs part; later dates are set in columns. What it needs, what was assumed of the enemy and the staff's own words are one step down. It says "runs" where the records say worlds. It changes nothing in the game. A draft order has its own DRY RUN beside SIGN AND SEND: the draft is played and stays unsigned. Under "What if" on the plan sheet a change of risk, or "not before" a day, is previewed and played beside the plan as it stands, run for run, and can then be given from the estimate with one press; nothing is given until then.
- The pieces are period tiles: cream, in ink, leaning toward the chair. A rectangle is the side's own and a diamond the enemy's; a silhouette says what it is (a carrier, a surface force, transports, an escort, the crossed box for men ashore, a pennant for a base); a dashed frame says it is only reported, and such a tile fades as its report ages. The tiles grow as the commander sits back.
- A touch on the chart, when nothing is being drafted, shows what the side holds of the thing touched. Of the enemy: a contact with its age in pulses, or a place with the latest estimate of his men, where it came from and its age in days, and the words that it is a report and can be wrong. Of its own: a force (what it is doing, its ships and their fuel, the air groups and men aboard, whether the staff or the commander has it) or a base (its stock, days of supply, men ashore, air groups and the forces lying there). A touch on a tile is a touch on that thing, whatever else lies in its hex; a base or a place leads to the forces and contacts lying there. A contact's card says how it is known, from the report that last gave it. Which directives were received and read is remembered across a reload of the page.
- "What is coming" in the signals tray is the schedule: the air groups due and the forces not yet available, by date. With the directive it takes the place of the lists in the staff's return.
- Shows the directive from the commander above when one is due: the first on the first evening, a later one when its condition holds (a named base is the side's own). A new one opens by itself and stays on the desk. It carries no points; the staff's full return, which does, is one step away under its own name. The page's address ending `#directives=all` shows every directive, for reading the drafts.
- Draws the chart from the map's hexes, the side's forces as blocks, its bases, its men ashore, its contacts and its estimates of the enemy ashore. A contact and an estimate fade with age.
- "State an intent": touch the chart at a base or at open water, choose the end and the risk on the order form, and read the staff's own preview before signing. Signing approves that preview; a risk other than balanced is set after it.
- "One day" resolves a day. "Carry on" resolves days until something stops the clock (an interrupt fires, the game ends) or the player stops it. A day that has been resolved is then plotted pulse by pulse, the night, the morning, the afternoon and the end of the day: the forces move as the side's own events say they steamed in that pulse, and a list beside the chart gives what the side was told in it, with the names of its units and places. The events come from the tool `events`.
- The clock shows the hour a commander commands at: a game day runs from 1800 to 1800, so the orders for day 1, whose daylight is the date given as `--start`, go in at 1800 the evening before.
- The signals tray shows what stopped the clock, each with the staff's reasons ("Why") and a way to switch that standing interrupt off; the plans that wait for leave; the standing leave to launch; what is signed and not yet taken up; and the briefing.

## Where a scenario lets a seat stop inside a day

A scenario with `orders_inside_a_day = true` (decision 0156) offers the tool `advance`, and the table then works a pulse at a time:

- "Next pulse" resolves one pulse and "Carry on" resolves pulses until the clock is stopped. Each pulse is plotted as it is fought, from the tool `events`.
- "The clock stops when I am told of" chooses what stops it: a contact, an attack, a ship lost, a strike away, men ashore. An interrupt of the staff's stops it at dusk as before.
- At 0600 and 1200 "Order a force" takes an order to one of the side's task forces (its speed, its readiness, or a place to steam for) and to the air groups in its hex (stand down). The order is taken in at the start of the next pulse. The form says, before it is signed, that the staff plans round a force the commander has ordered until it is released; a held force shows in the signals with "Release to the staff".
- An intent stated at 0600 or 1200 is taken up by the staff at dusk, and the tools' answers say so.

In a scenario without the key the table resolves a day whole and then plots it, as before. To try it on the 39 days, copy `data/scenarios/stage-3-proof/07b_slice.toml` into a folder beside it and add `orders_inside_a_day = true` under its `name`.

## A watched game

The table can hold a game in which agents command and the person at the table only watches and holds the clock (decision 0182). An agent is whatever makes the calls at its seat's door: the bridge starts no model and holds no key of any model.

```
cargo build --release -p sim-cli
python3 table/serve.py data/scenarios/orders-inside-proof/05_eastern_solomons_on.toml --watch --seat allied --seat japanese --keep /tmp/watched
```

`--seat` names each side an agent plays; a side not named is the script's, so `--seat allied` alone is an agent against the engine. The bridge starts `sim-cli serve SCENARIO --watched` and prints:

- the watcher's table, `http://127.0.0.1:8742/watch/KEY/`, to open in the browser;
- one door for each agent's seat, `http://127.0.0.1:8742/seat/KEY`.

Each key is made new at every start. Give each agent its own door and nothing else.

With `--keep DIR` the saved game and the transcripts are written when the bridge leaves (Ctrl-C, or a signal to end). It also writes `journal.jsonl` there as the game goes: every seat's call and every step, in the order they were sent. A game killed outright before it could save is played again from it, to the same game: `sim-cli serve SCENARIO --watched --seat allied=player --seat japanese=player --save save.json --transcript transcript.jsonl < journal.jsonl` (with the seats the game had).

**An agent at a door, by HTTP:** `GET DOOR/tools` lists the seat's tools and the door's own `wait`; `GET DOOR/instructions` gives the words an agent is given on taking the seat; `POST DOOR/call` with `{"tool": .., "arguments": {..}}` makes a call, answered `{"ok": the answer}` or `{"refused": the words}`. A call of `end_day`, `run` or `advance` is the seat's word that it is ready, and is answered when the watcher's clock reaches the stop it asked for. The door holds a call open up to 50 seconds (`--hold`), then answers `{"waiting": true, ..}`; the seat then calls `wait` (no arguments), as often as it answers so, until the answer comes. While its call is held a seat makes no other call.

**An agent by the Model Context Protocol:** `python3 table/seat_mcp.py --url DOOR` serves one door on standard input and output, as `sim-cli mcp` serves a seat, so a model with tools takes the seat as it would `sim-cli mcp`. For Claude Code, an `.mcp.json` entry whose command is `python3` with those arguments.

**The watcher's table** shows one side at a time as its commander sees it, with a switch between the two sides where both are agents'. Nothing on it gives an order. The seat's slip at the top of the signals says whether each agent is ready and until when, or that it is waited for, and lists the calls the agent watched has made since its last stop, with the words it gave as its reason. The clock is "Next pulse" (or "Next day", where the scenario has no orders inside a day), "Carry on" and "Stop". Carry on is the page's: it steps whenever every seat is ready, plots each step from what the side watched was told, and stops when that side is told of a thing ticked under the clock sign, when the game ends or when stopped. "Begin again" is refused while an agent has called its door in the last five minutes.

**Kept:** with `--keep DIR`, on leaving (Ctrl-C) the bridge closes the game, and `serve` writes the saved game, `DIR/save.json`, and the whole transcript, `DIR/transcript.jsonl`; the bridge then writes each seat's calls to `DIR/transcript_allied.jsonl` and `DIR/transcript_japanese.jsonl`. A game begun again is kept beside it as `save-2.json` and so on.

**What the keys are for.** Each door answers its own seat's calls and nothing else, and no door returns anything the watcher reads. The keys keep a program from calling a door not its own, or the watcher's, by mistake. They do not stop a program on this machine that sets out to cheat: it can read the bridge's printed lines, or the game's own files. Watch fair agents.

**The check.** `python3 table/check_watch.py` (ports 8760 to 8762, `--port` for others) plays two short watched games through the doors as scripted agents would, with the watcher stepping, checks `seat_mcp.py` over a door, then the player's table without `--watch`, and stops everything it started. It shortens the door's 50 seconds to 3. It needs `sim-cli` built in release.

## The new game screen

`python3 table/serve.py`, with no scenario (decision 0185), prints one address, `http://127.0.0.1:8742/table/KEY/`, with a key made new at every start. The screen and the table are both there. Options: `--port`, `--sim-cli`, `--map`, `--hold` (the doors' wait, 50 seconds), `--keep DIR` (each game is kept in `DIR/game-N` when it ends: the saved game, the transcript, the journal and, where seats were played through `serve`, a file of each seat's calls) and `--scenarios FILE` (the list, `table/scenarios.json` unless given). `--seat`, `--start`, `--island`, `--directives` and `--watch` name a game, and are refused without a scenario.

The screen asks three things: the scenario, from `table/scenarios.json`, with its dates and length and what each side is to do; the seat, Allied, Japanese or to watch; and who plays each other side, the engine or an agent. It says what the engine is for the choice made (a staff carrying out the orders the scenario gives that side, or forces following the scenario's orders as written), and for each seat an agent holds, how the agent takes it. "Begin" opens the game and the page becomes the table. On the table "New game", touched twice, ends the game in hand, whoever is at a seat, and returns to the screen. "Begin again" begins the same game anew, and is refused while an agent has called its door in the last five minutes.

**The list.** `table/scenarios.json` gives for each scenario its name, dates, length, lines, the date of day 1, the islands' names, each side's directives file where it has one, its map (`map`, a repository path: the chart drawn is that map's; an entry with none is on the slice's `data/map/south_pacific.csv`, and every entry names it today), and its pairings: who holds each side (`person`, `agent` or `engine`), what the engine is (`staff` or `script`) and the scenario file that makes it a game. A game is offered only where a file gives the engine's side orders or orders for its staff, and the sides played by a person or an agent nothing. Kept by hand: a scenario renamed or moved is changed there. It offers today:

| Scenario | Game | File |
| --- | --- | --- |
| The 39 days | Allied against the engine; watching an Allied agent against the engine | `stage-3-proof/07b_slice.toml` |
| The 39 days | Japanese against the engine; watching a Japanese agent against the engine | `stage-3-proof/07c_slice.toml` |
| The 39 days | A person against an agent, either side; watching two agents | `staff/07_full_slice.toml` |
| The Eastern Solomons | Allied against the engine; watching an Allied agent against the engine | `second-proof/05c_eastern_solomons.toml` |
| The Eastern Solomons | Japanese against the engine; watching a Japanese agent against the engine | `second-proof/05e_eastern_solomons.toml` |

The Eastern Solomons has no file in which neither side is given anything, so it offers no game of a person against an agent and none of two agents.

**How the bridge holds each game.** A person against the engine: `sim-cli mcp` for his seat, as without the screen. A game watched: `sim-cli serve --watched`, as `--watch` has it. A person against an agent: `sim-cli serve --watched` with both seats played, and no watcher. The bridge steps the clock itself whenever both seats are ready. The person's calls are his seat's own; a call of time (Next pulse, One day, Carry on) is held until the clock reaches its stop, and once pressed it cannot be taken back. While it is held he may read, and is answered as the watcher would be from his own side; he may not preview, try a dry run or order. Of the other side he is told only whether its commander is at his seat and whether he is waited for. Every request of a watcher's is refused in this game.

**An agent's seat.** Each side has a door, `http://127.0.0.1:8742/seat/KEY`, whose address holds for as long as the bridge runs, so that a client is set up once. A door answers only while its side is an agent's in the game in hand; it says "not yet at its seat" on the table until its first request in that game. When a game ends, every call still held at a door is answered that the game has ended and a new one begun. The screen, and on the table the button "The agent's seat", give for each agent's seat, to copy: the `claude mcp add flag-plot-SIDE -- python3 /…/table/seat_mcp.py --url DOOR` line; the same as an entry of tool-server settings, for a client that reads them from a file when it starts; the door itself; and a brief (the side, its directive where it has one, to play to the end, and to read nothing but what the tools tell it). Beside them the person is told that a model at a keyboard stops of itself long before a campaign ends, that a model with a shell could read the scenario's file and asking it not to is the only guard, and that a call may wait about fifty seconds before it answers that it waits. The door's opening words say that the clock moves when both commanders have finished (or, in a game watched, that a watcher holds it), and that any call may be held, not only a call of time.

**What the keys are for.** The table's key and each door's keep a program from calling what is not its own by mistake. They are no guard against a program on this machine that sets out to cheat, and none against the person himself: he sets the agent up, so he has handled its door. In a game of a person against an agent nothing of the agent's side, its calls or its reasons is sent to the page, and nothing of the person's to the agent's door; the door is shown on the table only behind "The agent's seat". What `--keep` writes holds both seats' calls and is the umpire's, for after the game.

**The check.** `python3 table/check_screen.py` (ports 8780 to 8785, `--port` for others) starts the bridge with no scenario, reads the list, begins every game it offers and plays a day of each, plays a person against an agent through several days with both the page's calls and the agent's door made by script, checks that the sides are kept apart, that a held call is answered when a new game begins and that the door serves the new game, runs `seat_mcp.py` against a door, and then runs `check_watch.py` for the bridge started with a scenario. It shortens the doors' wait to 3 seconds and stops everything it started. It needs `sim-cli` built in release.

## What it does not do

- It reads the clock from the words of a refused briefing when it is opened at 0600 or 1200, since no tool tells a seat the hour.
- Inside a day it orders only a force's speed, readiness and route and stands an air group down: not the rules of engagement, a search arc, another air mission or a land unit's posture, which the engine also takes.
- It gives no order to a ship, a force, an air group or a unit, sets no forces limit, and withdraws no intent.
- It has no check in CI and no test.

## The chart's coastline

The chart is drawn from the map the game is played on, in that map's own frame (decision 0199). The bridge serves four things of it: `/map` (the hexes), `/frame`, `/coast` and `/names`. For a game begun from the new game screen they are those of the map of its scenario's entry in `scenarios.json` (loaded when the game opens, once for each map); for a game named on the command line, those of `--map`.

**The frame.** A map generated by the map tool says where it is on the earth in its header: the lines "Projection: hex (0,0) is centred at ...", "east_nm = (lon - ...) * 60 * FACTOR" and "Extent: every hex whose centre lies from ... to ... and from ... to ...". The bridge reads those lines (`read_frame` in `serve.py`) and serves `{centre_lat, centre_lon, factor, west, east}` at `/frame`, every longitude in the map's run: a map that crosses the date line (the Midway map's extent runs 163 E to 166 W) gives longitudes above 180, 163 to 194. A map with no such lines, as the hand-written placeholder maps have none, has no frame (`/frame` answers `null`) and the page draws it as it always did. The page draws longitude scaled by the frame's factor, so a hex is not stretched, letters the meridians and the position readout east or west (a longitude in the run above 180 is shown as 360 less it, west), and brings the hexes' and the names' longitudes into the run before drawing. The slice's chart is drawn 2 in 100 narrower than before, as the plan says, since it too is drawn true.

**The coast and the names.** Each map has its own, found by the map file's name in `table/chart/`: `<name>_coast.json` and `<name>_names.json` for `data/map/<name>.csv`. They are for the eye only: the game plays on the hexes and nothing in the simulation, the staff or a seat's tools reads them. With no coast file the page shapes the land from the hexes themselves. A names file gives each name's position west negative, as `data/oob/place_positions.csv` has it, and the page brings it into the run; a coast file stores its longitudes in the run already, so the page does not wrap it.

**The slice's coast.** `table/chart/south_pacific_coast.json` is cut from Natural Earth's 1:10m land and minor islands (naturalearthdata.com, public domain), clipped to 142 to 175 degrees east and 26.5 degrees south to 2 degrees north and rounded to three decimals, by `table/chart/clip_coast.py` with no `--map`, which takes the two downloaded GeoJSON files and the output path.

**The Midway coast.** `table/chart/midway_coast.json` (38 rings, 263 points) was made from the same two files, as shapefiles, from the top of the repository:

```
python3 table/chart/clip_coast.py --map data/map/midway.csv references/natural-earth/ne_10m_land/ne_10m_land.shp,references/natural-earth/ne_10m_minor_islands/ne_10m_minor_islands.shp table/chart/midway_coast.json
```

With `--map` the script reads the frame from that map's header, clips to its extent padded by 3 degrees on every side, brings each ring lying wholly west of the frame's western edge into the run (360 added to every point of it), and writes a ring that both files hold once. It reads GeoJSON or `.shp` (outer rings only; the clockwise parts of a shapefile). Standard library only. `midway_names.json` lists the five places with a row of `data/oob/place_positions.csv`, each at that row's position; no sea is named. Natural Earth's files are not kept in the repository.
