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

**Replay version 4** adds a top-level `steps`: the state at the end of every step, for every day and every phase whether or not the phase has events. The steps are the ones the page makes of the events (`start`, then for each day `orders`, `night`, `morning`, `afternoon`, `end`), and the export records each at its moment of the day (`crates/sim-cli/src/export/steps.rs` says which). An entry is `{ day, phase, changed, removed }`. The first entry (day 0, `start`) carries every record of the start state in `changed`; each later one carries only the records whose JSON differs from the same record in the entry before (same kind and id; a contact by its id, a ground estimate by its hex, for each side), and in `removed` the ids of records that are gone. The records are those of `final` (task forces, ships, air groups, bases, land units, each side's contacts and ground estimates), made by the same code, so the steps applied in order end exactly at `final`; claims are in `final` only. Kinds with nothing to say are left out, so a step that changed nothing is just its day and phase. The page takes each step's state from `steps` and works none of it out again. It still draws a step's own events from the events (the moves, fights, searches and the air layer, and the hexes a force steamed through), and takes from the events what a side's records cannot give: the holder of a base the side lost, a ship the side saw sunk, an enemy base a report found, and the base a base strike is on. A side's page cuts every entry to its own task forces, ships, air groups, bases and land units and its own side's contacts and estimates, as it cuts `initial` and `final`; each of its steps shows the state at the end of the umpire's step its last event is in. A step with no entry in `steps` is an error the page reports. A version 1, 2 or 3 file has no `steps` and is rebuilt from its events as it always was (where this page says the page rebuilds from the events, it is about those files).

**Command state** (decision 0107, stage 1 of step 3) is written beside the records, only for a side whose command state is not empty (nothing was ever set), so a replay of a run with no command is byte for byte what it was and the version stays 4. `final.commands` is `{ allied?, japanese? }`, each side's state being `{ intents, leaves, plans, commitments, next_plan }`: an intent is `{ id, kind: "keep_supplied", base, floor_days, interval_days?, set_day, plan, withdrawn_day?, limits }` (`limits` is `{ forces?: { ships, task_forces, source }, area?: [[q, r]...], priority?, protected?, risk }`, each limit left out when not set); `leaves` is `{ prepare, launch }`; a plan is `{ id, intent, state, baseline?, amendments?, runs? }` (state is draft, preparing, released, complete, failed or cancelled; `baseline` is `{ day, kind, ..., limits }` taken at the first approval; an amendment is `{ number, day, change }` with `change` holding an `intent` or a `limit`; a run is `{ number, ships, load_t, source, days: { load, sail, arrive, free } }`); a commitment is `{ ship, plan, run?, from_day, to_day }`. A weight is a JSON number from its millionths, as everywhere. Every entry of `steps` may carry the same `commands` (`{ allied?, japanese? }`): a side's whole command state at the end of that step, written in the first step where the state is not empty and again in each step where it differs from the one before, so the last entry written equals `final.commands`. The page does not draw it yet and ignores the key; `check.js` does not look at it.

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

It syntax-checks both script blocks of `index.html`, runs the page's own pure functions on each replay (geometry, steps, rebuild against the final state, causes, labels); for a version 4 file it also checks that the entries of `steps`, applied from the first, end exactly at `final`'s records, that every step the page shows has its state, that a side's page holds nothing of the enemy's, and that the state taken from `steps` is the state the rebuild from the events makes, step by step (the one exception is the march a falling back gives a unit, which the rebuild does not make), the measuring functions against the engine's fixture, and on some synthetic edge cases and bad files. For the air layer it checks, in the Umpire model and both side models, that every launch has a launch hex that gives the engine's distance, that no strike or search is beyond its type's reach, that every raid's events are in its step and in one raid only, and that a side's page holds no true hex and no miss; a hand-built replay has one strike that found nothing, with its raid, tally and true hex asserted exactly. `node viewer/check.js --against-rebuild viewer/replays/*.json` also compares, for each version 4 file, the model taken from `steps` with the one the rebuild from the events makes, step by step, while versions 1 to 3's rebuild still knows every event. It exits non-zero if anything fails. No npm packages.

## Shells, bombs and the estimate of the enemy ashore (decision 0092)

The event list shows a shore bombardment (`bombardment_fire`, `shore_bombardment`, a claim with `from_bombardment`), men lost to fire (`men_lost_to_fire`), a ground report (`ground_report_received`) and a base a report found (`base_found`) as any event, in the engine's words; the map does not draw a bombardment. A base's side sees `shore_bombardment` without its `task_force`, and the firing side sees it without its hits. In a version 1 to 3 file the viewer rebuilds each unit's men from `men_lost_to_fire`, and each side's estimates from `ground_report_received`, and `check.js` compares them with `final.ground_estimates` (left out of a replay where no side holds an estimate); a raid's `hits_on_men` is written only when a hit landed on men.

## Cover and fieldworks (decision 0106)

The final state's land units carry `fieldworks`, the fieldworks a unit has dug, 0 to the ground figure `fieldworks_limit`, left out when 0 so that a replay made where nobody digs is as it was. In a version 1 to 3 file the viewer rebuilds each unit's fieldworks from the `fieldworks_changed` events (`dug`, `worn` or `lost`, with the fieldworks afterwards), and for a unit made by division from its parent's at `unit_divided`; `check.js` compares the rebuilt value with the final state's, as it does the men, the posture and the fatigue. A fatigue change `strained` is rebuilt like any other change of fatigue. `defences_worn` (a base's defences worn by fire) is shown in the event list in the engine's words and is not rebuilt. A side's page has the events of its own units and bases only: the enemy's fieldworks and defences are not in it.
