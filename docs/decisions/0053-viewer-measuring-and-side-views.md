# 0053: The viewer measures, and shows a side's view

Status: settled, an engineering choice for the umpire's tool of decision 0018. No rule of the game changes.

## The gap

Decision 0018 left two things for later: a view of one side with the truth hidden, which "waits for briefings", and, by decision 0040, a measuring tool that must use the engine's one function for air distance. Briefings (0048), measuring (0049) and what a side may know of what happened (0047) now exist.

## Decisions

### Measuring

1. **The viewer has a Measure mode.** Two clicks pick two hexes. It gives the air distance in whole miles, the sea route in miles and hexes or "no sea route", and draws both.
2. **The page repeats the engine's two functions with whole numbers:** `Hex::air_distance_nm` (decision 0040) and `Map::sea_path` with its closed sides and the hexes ships may enter (decision 0017). A sea route's miles are its hexes times 40, as `measure` counts them (decision 0049).
3. **The page cannot drift from the engine.** A test writes `viewer/measure_fixture.json` from the engine's own functions (491 air cases, 136 sea cases on `data/map/south_pacific.csv`) and fails if the committed file differs; `node viewer/check.js` runs the page's functions on the file and fails on any difference.

### A side's view

4. **The viewer has a View selector: Umpire, Allied, Japanese.** Umpire is as before. A side's page shows that side's own units exactly, the enemy only as the contacts that side holds, the enemy's bases as public places (id, name, hex and side, decision 0048), and the events the engine gave that side.
5. **The export writes each side's events as `SideView::events()` gives them,** in a new top-level `side_views` (replay version 2). Nothing in the export or the page decides who may see what. Each event has the side's own number `n` (1, 2, 3, decision 0048, 14), its place `log_id` in the full log, `cut` when the engine cut it down, the engine's own wording as `text` (the `Display` of the side's event kind, the wording `explain` uses), its causes in the side's numbers, and whether it has causes the side cannot see.
6. **The page for a side is built from its events and its own units alone.** The end state comes out of them and is checked against the side's own end state, so the events a side is given are enough to draw it. The state hash, which comes from the truth, is not shown.
7. **A version 1 file opens in the Umpire view only.**

## Costs and caveats

- A side's steps are the ones in which it has events, so the step list does not say when the enemy did something.
- The detail panel shows the end-of-run damage, fuel and ammunition of the side's own ships, as the Umpire view does. It is the side's own ships and not the enemy's, and it is later than the step on screen.
- A side's events at day 0 use the engine's side wording for a starting contact, not the longer line the full log gives it.
- Orders keep their numbers from the full list: a side's own orders are shown, but their numbers have gaps where the other side's orders were.

## Rule card

No rule. The page reads the replay file and writes nothing. The export reads `SideView::events()` and writes nothing to the world.
