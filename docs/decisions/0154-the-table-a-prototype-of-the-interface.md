# 0154: The table, a prototype of the player's interface

Status: a record by the architect, written on Nov 18, 2026, of a prototype the designer asked for that day in the design conversation on the interface, which opened when stage 3 closed. It adds no rule and changes none. It is a prototype: it is not a stage of step 3, it has no pass mark, and what it settles is where it lives and what it may read.

Summary: A prototype of the player's interface, in `table/`: a chart table seen from the commander's chair, drawn in the browser, played against a real game through one seat's tools by a small bridge; what it may read (the seat's tools and the map file, never the umpire's files); what the designer has agreed about the interface so far; and the four gaps in the seat's tools that wiring it found, one of them mended at once

Standing: Built as a prototype on Nov 18, 2026, on the designer's word. Not a stage of step 3; it has no test and no check in CI

## Why it is recorded

The designer asked for the design of the interface to begin when stage 3 closed (Nov 17, 2026), and on Nov 18, 2026 asked for a table in three dimensions to try, and then for it to be wired to the game. A folder at the top of the repository and a program that reads a data file are choices the specs did not make, so they are written down.

## What the designer has agreed about the interface

From the design conversation of Nov 18, 2026. These are directions, not a spec.

1. One seat: the commander of a theatre, of one side.
2. A turn is "until the staff needs me": every stop is a question. The player watches the scene unfold on the map and may stop it to give or change an intent; a new intent takes time to become the staff's orders.
3. The staff's reason for a thing is one step away from the thing, and the figures a step beyond that.
4. The look is a real chart: tracks in pencil, and marks of the enemy that fade as the report ages. A block is the player's own and known exactly; a mark is something he was told.
5. Intent is stated on the chart and on a short order form, and the staff's preview is read before signing.
6. A table in three dimensions is tried, the commander seated at it; the flat chart may stay beside it.
7. A scenario lays its own sheet on the table: a small scenario shows only what it needs. New players begin on small ones, which grow to the war from 7 December 1941.
8. Ruled out: ordering ship by ship as the main way to play, and showing the truth of the enemy before the game ends.

## What is built

`table/serve.py` and `table/index.html` (`table/README.md` has how to run it and what it does).

1. **The bridge** starts `sim-cli mcp SCENARIO --seat SIDE` and passes the page's tool calls to it. It is given nothing the seat is not: the page's every word of the game is an answer of one of the seat's tools. It listens on the machine it runs on and nowhere else.
2. **What it may read.** The seat's tools, and the map file, for the coast. A map is known to both sides and no tool gives it. It reads no replay, save, batch, event log or scenario file: those are the umpire's (`docs/engineering/truth-and-belief.md`). The rule that only `sim-data` reads simulation data files is about the simulation; the bridge is outside it, as `map-tool` is, and reads the one file with a reader of its own.
3. **Where it lives.** `table/`, at the top of the repository beside `viewer/`. The viewer is the umpire's and shows the truth; the table plays the game and is shown only belief. They share no file, and the table never reads `viewer/replays`.
4. **What it is written in.** Python's standard library for the bridge and one page with no build step, as the viewer is. The page loads a drawing library and its fonts from the network. No crate and no dependency of the workspace is added.

## What wiring it found

1. **A seat could not state "control an island".** The argument check of `set_intent` left out `island`, so the tool refused the one argument that end takes; only a scenario file could state it, as stage 3's proof did. The tool's schema and description already offered it. Mended at once, with a test that states it through a seat (the pull request before this one). It is also why the model's game of the 39 days by intent alone, still owed, could not have been played through the tools as they stood.
2. **No tool lists a scenario's islands.** A seat can state control of island 1 and cannot ask what islands there are or what places each has. The bridge is told them on its command line. Open: a list of the islands in the seat's tools.
3. **No tool gives the map.** The bridge reads the map file. Open: whether the seat's tools should give the map's hexes.
4. **The seat's tools answer between days.** The game resolves a day in three pulses, and a side's events carry their pulse, but the tools hand a seat the day after it is over. The table shows a day as one move and stops only between days. The designer asked that the player see the scene unfold and may stop it at any time. Open, and a question of the rules as well as of the tools: whether a commander may give a command between pulses, and when the staff takes it up. Nothing is changed here. Decision 0155 has the direction the designer then gave, and the table now plots a resolved day pulse by pulse.

Also seen, and left: a preview is worked at the risk in force, so the order form says that a risk chosen on it goes with the order and is not in the preview.

## What it costs

Two files and a page of documents. It has no test: a change to the seat's tools can break it and nothing will say so. That is accepted for a prototype and is to be mended before it is anything more.

## Three ways of drawing the pieces, to compare

The designer asked on Nov 18, 2026 whether the pieces should be "kind of 3D NATO symbols". The architect's view: take the grammar and not the modern set. Shape says whose a piece is (so colour is not the only sign), an icon says what it is, and a dashed frame says it is only reported, which is what a contact is. The set itself is decades later than 1942, is dense for a player who does not know it, and is thin at sea, where this game mostly is.

The table now draws all three, switched by "Pieces", for him to choose by eye: the plain blocks; tiles in the standard's frames and colours with letter codes; and period tiles in ink on cream with the same shapes and a silhouette. In the two tile styles the enemy's bases, contacts and estimates ashore are pieces too, the reported ones dashed and fading with age. The tiles lean toward the chair so that they read from a seat. Nothing is chosen yet. The letter codes and the silhouettes are the prototype's own and stand for a task force's mission, not for a ship class.

**Chosen on Nov 18, 2026: the period tiles.** The designer compared the three on a game in progress and chose the period tiles. The blocks, the tiles in the standard's frames and the switch are taken out. The tiles now grow as the commander sits back and keep facing the chair when they move. Still rough: from the chair sat back a tile shows where a thing is more than what it is, and several pieces in one hex crowd each other.

## The plan sheet

The designer asked on Nov 18, 2026 whether a player can see what a plan includes and change it. The game gives a seat the plan's parts through `command_state` (the intent tree, each node's plan, measure and limits), `plans` (state, amendments, runs) and `staff_orders` (the day's decisions with what each rests on, what binds it and what would change it); the table showed only a slip with "approve". The table now has a sheet for each intent of the commander's that lays these out as one tree, with the instructions that exist: approve a plan, cancel it, set the risk, withdraw the intent. To keep a force out of a plan the commander orders the force himself, which holds it.

A plan is changed by instruction and not by editing it. That is the game's premise and is kept. What a commander cannot yet instruct, found while building the sheet, is listed in `docs/open-work.md` for the designer to choose from.

