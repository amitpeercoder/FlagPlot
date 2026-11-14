# 0018: A read-only replay viewer

Status: confirmed by the designer, Nov 10, 2026.

## The gap

Step 1 is headless by design, and the pillars say no real interface is built before the kill gate. But the only way to look at a map or a run was to read text: a grid of characters for the map, and an event log for a run. On Nov 10, 2026 the architect misjudged the generated map twice from that grid. A picture would have shown both problems at once.

## Decisions

1. **A viewer is built now, as a debugging tool.** It draws the map and replays a finished run on it. It is not the game's interface and settles nothing about it.
2. **It is read-only.** It takes no orders and cannot change a run. It makes no design commitment the kill gate could be said to have skipped.
3. **It is the umpire's view.** It can show the truth, and each side's contacts beside it. It is for the designer and the developers, like the command-line runner. Nothing that plays the game may use it.
4. **The runner exports a run as one replay file:** the map, the starting forces, every event with its causes, and the final state. The simulation core is not changed for it, and gains no dependency.
5. **The viewer is one page with no build step and no outside dependencies.** It opens a replay file and works offline.
6. **What it shows:** the hex map with land, coast, sea and closed sides; places by name; forces moving pulse by pulse; contacts appearing, ageing and dropping; the events of each pulse; and, for any event, the chain of orders and events that caused it.

## Why

- **Being able to see is worth it now.** Stage 4c is a fight among five groups in one or two hexes, with real geography.
- **Everything it needs is already recorded.** Every state change is an event with its causes. The viewer is a second reader of that log.
- **Read-only keeps it honest.** The questions the kill gate asks are about commanding. This cannot command.

## Deferred

- State as it was at each pulse. The replay has the starting state, the events and the final state. The viewer rebuilds positions and contacts from events. Ship damage is shown as events and as the final state.
- One side's view with the truth hidden, as a player would see it. That needs events filtered by side, which waits for briefings. *(Nov 14, 2026: built: the viewer shows a side's view, decision 0053)*
- Anything interactive: orders, plans, the clock.

## Rule card

No rule. Nothing in the simulation reads or writes anything for this.
