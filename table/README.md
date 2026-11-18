# The table

A prototype of the player's interface: a chart table seen from the commander's chair, drawn in the browser and played against a real game. It is a prototype, kept to find out what the interface should be; it is not a stage of step 3 and nothing depends on it. Decision 0154 has what it is and what it found.

It plays the game. It is not the umpire's: it is given one seat's tools and nothing else. It never reads a replay, a save, a batch file or the event log.

## Run it

From the top of the repository:

```
cargo build --release -p sim-cli
python3 table/serve.py data/scenarios/stage-3-proof/07b_slice.toml --seat allied --start 1942-08-07 --island "1=Guadalcanal@1" --island "2=Tulagi@4"
```

Then open http://127.0.0.1:8742/. That scenario is the 39 days of the staff's setup with the Japanese side's intents stated and the Allied side given nothing, so the Allied seat states its own.

`--start` is the date of day 1, for the clock. `--island` names an island the scenario has (its number, a name for the order form, and the number of a base on it), because no tool lists a scenario's islands yet. `--map` is the map file to draw (the default is the slice's). `--port` is the port. The page loads its drawing library and its fonts from the network.

## What is in it

- `serve.py`: the bridge. It starts `sim-cli mcp SCENARIO --seat SIDE`, passes the page's tool calls to it one at a time, and serves the page and the map's hexes. Python's standard library only. It listens on this machine alone.
- `index.html`: the page. One file, no build step.

## What it does

- Draws the chart from the map's hexes, the side's forces as blocks, its bases, its men ashore, its contacts and its estimates of the enemy ashore. A contact and an estimate fade with age.
- "State an intent": touch the chart at a base or at open water, choose the end and the risk on the order form, and read the staff's own preview before signing. Signing approves that preview; a risk other than balanced is set after it.
- "One day" resolves a day. "Carry on" resolves days until something stops the clock (an interrupt fires, the game ends) or the player stops it. After each day the pieces move to where they now are.
- The signals tray shows what stopped the clock, each with the staff's reasons ("Why") and a way to switch that standing interrupt off; the plans that wait for leave; the standing leave to launch; what is signed and not yet taken up; and the briefing.

## What it does not do

- It shows a day as one move. The game resolves a day in three pulses and the seat's tools answer only between days, so the table cannot show a pulse or stop inside a day.
- It gives no order to a ship, a force, an air group or a unit, sets no forces limit, and withdraws no intent.
- It has no check in CI and no test.
