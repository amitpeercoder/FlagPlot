# Replay viewer

A page that draws the map and replays a finished run on it, pulse by pulse, with the events of each step and the chain of orders and events behind any one of them.

**What it is.** An umpire's debugging tool for the designer and developers, like the command-line runner. It shows the truth, and optionally each side's contacts beside it.

**What it is not.** It is read-only: it takes no orders and cannot change a run. It is not the game's interface, and nothing that plays the game may use it. See [docs/decisions/0018-replay-viewer.md](../docs/decisions/0018-replay-viewer.md).

It is one file (`index.html`), no build step, no outside dependencies, and it works offline.

## Make a replay

From the repo root:

```
cargo run -p sim-cli -- export data/scenarios/04a_night_action.toml --out viewer/replays/04a_night_action.json
```

Add `--seed N` for another seed, and `--labels data/map/south_pacific_anchors.csv` for the real map with place names.

`viewer/replays/` is git-ignored.

## Open it

- Open `viewer/index.html` in a browser (straight from disk is fine), then pick a replay file with the button, or drop it on the page.
- Or serve the repo and name the replay in the address:

  ```
  python3 -m http.server 8765 --directory .
  ```

  then open `http://localhost:8765/viewer/?replay=replays/04a_night_action.json`. If the fetch fails the page says so and the file picker still works.

**View.** The View selector shows the Umpire's truth (as before), or one side's page: its own units exactly, the enemy only as the contacts that side holds, and only the events the engine let that side see (`SideView::events()`, decision 0047), numbered by the side. The export writes them in `side_views` (replay version 2, decision 0053); a version 1 file opens in the Umpire view only.

**Replay version 3** adds a top-level `aircraft_types` (id, name, `strike_radius_nm`, `extended_reach_nm`, `search_range_nm`) and an `aircraft_type_id` on every air group record, for the reach rings. A side's page carries the types of its own groups only. Version 1 and 2 files still open, without rings.

**Air.** The map shows air activity from the events the replay already records, nothing else. The Air box in the map controls shows or hides the whole layer (it is off for a replay with no air events).

- A search arc as before (`search_flown`). A small filled diamond, in the searching side's colour, where a search's report put a contact (`report_received` caused by a `search_attempt`). In the Umpire view only: a faint line from the diamond to where the force truly was, and a grey ring where a search missed the force it looked for (`search_attempt` with `sighted` false, at that force's position at that moment). A side's page has the diamonds only: the engine gives it neither the miss nor the true place.
- A raid is the strikes launched in one step from one place (a ship or a base) at one hex, as the rules group them. Its line runs from the hex the group launched from (the group's base, or its ship's force, at that moment) to the hex the strike was sent to (`strike_launched`, `base_strike_launched`), labelled with its aircraft and escorts (`escorts_sent`). Solid with an arrowhead: it attacked. Dashed ending in a hollow ring: `strike_found_nothing`; in the Umpire view a grey dashed line and "target truly here" show where the force its report was about really was, found by following the launch to its report to the sighting or attack behind it (or "not traceable"). Dotted: no bomber got through. A raid on a base ends in a small square.
- A tally box beside each target hex gathers the fight there in one step: raids, aircraft, escorts, the patrol that engaged (Umpire only), shots, anti-aircraft fire, aircraft lost by side and by cause, ditched, landed elsewhere, lost with a ship, hits, ships sunk, and claims (written "claimed", which is what the claimant believes). A side's page says only what its own events carry: its losses without their cause, its attack without its hits. Zoomed out the box is a badge with the raid count; hover for the box. Click it: the event list shows only the events of that air fight (with a "show all" bar; Escape also clears it), and the raid's first attack (else its found-nothing, else its first launch) is selected, so Why and Led to show the chain of causes.
- Reach on request: select a force or ship, or click a base on the map, and the detail panel lists its air groups (type, mission, reach as at this step). Click a group to draw rings round its position: its strike radius, its extended reach (solid when the group is on extended reach, faint when not) and its search range, labelled in miles. A replay with no aircraft figures (version 1 or 2) says so and draws no rings.

Keys: Left and Right step, Space plays and pauses, Escape clears the selection.

**Measure.** The Measure button on the map turns on a mode in which two clicks pick two hexes. The readout gives the air distance (the engine's `Hex::air_distance_nm`, decision 0040) and the sea route in miles and hexes (the engine's `Map::sea_path`, decision 0017), or "no sea route"; the route is drawn on the map. Escape or the button leaves the mode. The page repeats both functions in whole numbers, and `check.js` runs them on `viewer/measure_fixture.json`, which the engine writes (`FLAGPILOT_UPDATE_GOLDEN=1 cargo test -p sim-cli measure_fixture` rewrites it), so the page cannot disagree with the engine.

## Check it

```
node viewer/check.js viewer/replays/*.json
```

It syntax-checks both script blocks of `index.html`, runs the page's own pure functions on each replay (geometry, steps, rebuild against the final state, causes, labels), the measuring functions against the engine's fixture, and on some synthetic edge cases and bad files. For the air layer it checks, in the Umpire model and both side models, that every launch has a launch hex that gives the engine's distance, that no strike or search is beyond its type's reach, that every raid's events are in its step and in one raid only, and that a side's page holds no true hex and no miss; a hand-built replay has one strike that found nothing, with its raid, tally and true hex asserted exactly. It exits non-zero if anything fails. No npm packages.

## Shells, bombs and the estimate of the enemy ashore (decision 0092)

The event list shows a shore bombardment (`bombardment_fire`, `shore_bombardment`, a claim with `from_bombardment`), men lost to fire (`men_lost_to_fire`), a ground report (`ground_report_received`) and a base a report found (`base_found`) as any event, in the engine's words; the map does not draw a bombardment. A base's side sees `shore_bombardment` without its `task_force`, and the firing side sees it without its hits. The viewer rebuilds each unit's men from `men_lost_to_fire`, and each side's estimates from `ground_report_received`, and `check.js` compares them with `final.ground_estimates` (left out of a replay where no side holds an estimate); a raid's `hits_on_men` is written only when a hit landed on men.
