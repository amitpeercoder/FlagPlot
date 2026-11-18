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

`--start` is the date of day 1, for the clock. `--island` names an island the scenario has (its number, a name for the order form, and the number of a base on it), because no tool lists a scenario's islands yet. `--directives` is a file of directives from the commander above, as `table/directives/` has them (decision 0157); without it the table shows none. `--map` is the map file to draw (the default is the slice's). `--port` is the port. The page loads its drawing library and its fonts from the network.

## What is in it

- `serve.py`: the bridge. It starts `sim-cli mcp SCENARIO --seat SIDE`, passes the page's tool calls to it one at a time, and serves the page and the map's hexes. Python's standard library only. It listens on this machine alone.
- `index.html`: the page. One file, no build step.
- `directives/`: the directives a seat is given from the commander above, one file a seat of a scenario, written by hand in the game's own words. `guadalcanal_allied.json` is for the Allied seat of the 39 days: add `--directives table/directives/guadalcanal_allied.json` to the command above.

## What it does

- A touch on the chart, when nothing is being drafted, shows what the side holds of the thing touched. Of the enemy: a contact with its age in pulses, or a place with the latest estimate of his men, where it came from and its age in days, and the words that it is a report and can be wrong. Of its own: a force (what it is doing, its ships and their fuel, the air groups and men aboard, whether the staff or the commander has it) or a base (its stock, days of supply, men ashore, air groups and the forces lying there). An own base leads to the forces at it.
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

## What it does not do

- It reads the clock from the words of a refused briefing when it is opened at 0600 or 1200, since no tool tells a seat the hour.
- Inside a day it orders only a force's speed, readiness and route and stands an air group down: not the rules of engagement, a search arc, another air mission or a land unit's posture, which the engine also takes.
- It gives no order to a ship, a force, an air group or a unit, sets no forces limit, and withdraws no intent.
- It has no check in CI and no test.
