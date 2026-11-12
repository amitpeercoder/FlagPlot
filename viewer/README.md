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

Keys: Left and Right step, Space plays and pauses, Escape clears the selection.

**Measure.** The Measure button on the map turns on a mode in which two clicks pick two hexes. The readout gives the air distance (the engine's `Hex::air_distance_nm`, decision 0040) and the sea route in miles and hexes (the engine's `Map::sea_path`, decision 0017), or "no sea route"; the route is drawn on the map. Escape or the button leaves the mode. The page repeats both functions in whole numbers, and `check.js` runs them on `viewer/measure_fixture.json`, which the engine writes (`FLAGPILOT_UPDATE_GOLDEN=1 cargo test -p sim-cli measure_fixture` rewrites it), so the page cannot disagree with the engine.

## Check it

```
node viewer/check.js viewer/replays/*.json
```

It syntax-checks both script blocks of `index.html`, runs the page's own pure functions on each replay (geometry, steps, rebuild against the final state, causes, labels), the measuring functions against the engine's fixture, and on some synthetic edge cases and bad files. It exits non-zero if anything fails. No npm packages.
