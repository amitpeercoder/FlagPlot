# 0242, as built: A map from Papua to Midway

## Step 1: the second kind of projection (Nov 28, 2026)

What step 1 makes. It adds the second kind of frame, "about a centre", to the map tool and to `sim-data`'s reading of a header, and proves the two present maps and their exports are unchanged. No new map, no frame file for the wide map and no file of located positions are made. No rule of the simulation changes and nothing in `sim-core`.

### What changed, by crate

- **`map-tool`, `frame.rs`.** A `Frame` holds a `Kind` (`OneFactor(Degrees)` or `AboutCentre(Degrees)`) in place of its factor. A frame file takes `kind = "about a centre"` with `scale`; a file with no `kind`, or `kind = "one factor"`, takes `factor`, so `data/map/frames/midway.toml` reads as it did. A scale in a frame of one factor, a factor in a frame about a centre, neither, an unknown kind and an unknown key are all refused. `projection_lines` for one factor is as it was; for a map about a centre it writes the four-line header of the hand calculation's part 4 with the frame's own numbers (the extent line is shared). `Frame::of_map` reads either kind from `sim-data`. A scale must be above 0 and not above 1, as a factor must.
- **`map-tool`, `projection.rs`.** `Projection` gains `about_centre: bool`; its `factor` is the scale when that is set. `to_nm`, `from_nm`, `centre_of`, `hex_of` and `distance_to_centre` work for both kinds, the formulas of parts 1 and 2 with the literal 3437.746771. The angle from the centre is taken as `atan2(sin c, cos c)`, with `sin c` from the same terms that give the bearing: the same angle as the hand calculation's, and exact near the centre. The longitude that comes out of the inverse is in the frame's run.
- **`map-tool`, `generate.rs`.** `extent_hexes` takes its box from the four edges of the extent, each sampled at every tenth of a degree and at its end, with a hex to spare. `keep_near_extent` pads by one degree of latitude and of longitude for a map about a centre, and as before for one factor.
- **`map-tool`, `main.rs`.** `locate` falls back to the south-west Pacific frame for a map with no projection lines, or a map that is not there (the load then says so, as before). A map that has projection lines and fails to read stops with an error naming the map.
- **`sim-data`, `map.rs`.** `MapFrame` holds `projection: MapProjection`, an enum of `OneFactor { factor }` and `AboutCentre { scale }`; the old `factor` field is gone, so every reader must match. `parse_map_frame` tells the kinds apart by the first line, reads both forms, and refuses a header with some lines of either kind and not all, with lines of both kinds, or with only the extent line. A header with none has no frame. The lines of a map about a centre must be exactly as the plan gives them, with the miles in a radian equal to 3437.746771. `sim-data` works out no position and no hex. `MapProjection` is exported from `lib.rs`.
- **`sim-cli`, `export/map.rs`.** A map of one factor writes `centre_lat`, `centre_lon`, `factor`, `west`, `east` as before, byte for byte. A map about a centre writes `kind` (`"about a centre"`) and `scale` in place of `factor`.
- **`sim-cli`, `validate/passages.rs`.** `Frame::of` returns a result. For a map of one factor it is unchanged (whole numbers). For a map about a centre the check stops with an error saying its positions are read from the map's file of located positions (decision 0242, rule 15), which is not built yet. No floating point added.
- **`map-tool/tests/committed_map.rs`.** The check that the factor is the cosine of the centre's latitude reads the frame's kind and applies to a map of one factor.

### The proof that the two present maps are unchanged (rule 11)

Made in `/private/tmp/claude-501/map-about-a-centre-proof`, outside the worktree: a copy of the two committed inputs (`data/map/south_pacific_anchors.csv`, `data/map/frames/midway.toml`) at the same relative paths, and `references/natural-earth/` pointing at the two shapefile folders, so that the command line each header records is run as written and so as to be written the same. The changed tool, built `--release` from this branch, was run from that folder:

```
map-tool generate --land references/natural-earth/ne_10m_land/ne_10m_land.shp --minor-islands references/natural-earth/ne_10m_minor_islands/ne_10m_minor_islands.shp --out data/map/south_pacific.csv --anchors data/map/south_pacific_anchors.csv
map-tool generate --land references/natural-earth/ne_10m_land/ne_10m_land.shp --minor-islands references/natural-earth/ne_10m_minor_islands/ne_10m_minor_islands.shp --frame data/map/frames/midway.toml --out data/map/midway.csv
```

`cmp` of each against the committed file: identical (SHA-1 of `south_pacific.csv` 487d8bba319436de644ff06ba17131917b73d87c, of `midway.csv` d65722346bc1f1da64eb73c34c8acc7017c9f01c). No committed map was written over.

Also tried, in the scratch folder only and not kept: the frame of rule 3 (kind about a centre, centre 4 N 174.5 E, scale 0.964, 38 N to 30 S, 142 E to 153 W) made a map of 10,395 hexes (344 land, 363 coast, 9,688 deep sea), the count of part 3, with the header of part 4. `locate` put Midway at hex (31,-41), as part 1 has it.

### Tests added

- `crates/map-tool/tests/about_a_centre.rs`, under the doc comment with no word of it changed: `part_1_a_position_to_the_plane_and_its_hex` (east and north to 0.001 nm, the angle to 0.00000001, the bearing to 0.0001 degree, the distances to 0.01 nm, the hex and the next nearest, for the eight rows); `part_2_a_hex_to_its_position` (the seven hexes, latitude and longitude to 0.00005 degree, the longitude in the run and in the file, each back into its own hex); `part_3_the_hexes_of_the_extent` (10,395 hexes in rows -60 to 58, the seven rows' first q, last q and count, every centre inside the extent, none within 0.0005 degree of an edge, the four edge extremes, 587 hexes missed by a box from two corners and hex (-46,3) among them, every centre back in its own hex); `part_4_the_headers_lines` (the four lines exactly, `sim-data` reading them back, the refusals); `part_5_the_two_present_maps_are_unchanged` (the lines the tool writes for each map are in its file, and `extent_hexes` is its rows in order).
- `crates/map-tool/src/frame.rs`: a frame about a centre writes its lines; the two kinds' keys do not mix. The test `the_default_frame_writes_the_present_header_lines` is as it was.
- `crates/sim-data/src/map.rs`: a header about a centre reads as part 4 says; the refusals (each line left out in turn, the extent alone, a line of each kind, both kinds whole, a wrong miles-in-a-radian, a scale out of range, a wrong centre, an extent reversed or not holding the centre); the two committed maps read as of one factor, with their numbers. Its two existing frame tests changed only in naming the factor through `MapProjection::OneFactor`.

### Settled here, not in the plan

- The corner-box figure in the hand calculation's part 3 is "east 1721.7"; the tool gives 1721.687, so that test compares it to 0.05 nm, not to the 0.001 of part 1. The hand calculation has no mismatch; the number is stated to a tenth.
- A scale is accepted above 0 and up to 1, as a factor is.
- `locate` on a `.csv` map that is not there falls back to the south-west Pacific frame, so that the load's own error is the one said, as before.
- `Projection` keeps one field for the factor or the scale, with a flag, so that the default frame's test and every reader of the factor stand; the kind is in `Frame`.
- Not reached without a wide map, and unchanged: `water.rs`, `render.rs` and the lattice in `generate.rs` use only `Projection`, so they work for both kinds (the lattice ran on the trial above). `water`'s report was not run on a map about a centre.

### Checks run

- `cargo fmt --all -- --check`: passes.
- `cargo clippy --workspace --all-targets -- -D warnings`: passes (two lints met on the way: `from_*`/`to_*` method names on `Projection`, now `plane_to_position_about_centre` and `position_to_plane_about_centre`; a complex type in a test).
- `cargo test --workspace`: passes, 5,909 tests, none failed, 24 ignored (all ignored before this change). `crates/sim-cli/tests/timing.rs` took about 20 minutes in the debug build on a busy machine; it does not touch a map's frame.
- `cargo run -p sim-cli -- export data/scenarios/04a_night_action.toml --out <scratch>/04a.json`: ran; the export pinned by hash is unchanged (the tests above hold it).
- `sh scripts/regenerate.sh --check`: "every generated file is what the engine gives".

## The architect's reading of step 1

Made on Nov 28, 2026 on reading the build.

1. **The hand calculation holds as written.** No number or word of the doc comment changed, and the five tests check every part of it.
2. **The angle from the centre is taken from its sine and cosine,** not from the cosine alone as the doc comment's formula has it. It is the same angle and is exact near the centre. Confirmed.
3. **One guard added by the architect.** The map tool's projection keeps a map's scale in the field that holds a factor, with a mark for which it is. Its one helper that means a factor only, the miles in a degree of longitude, now stops if it is asked of a map about a centre; its one caller already took the other way for such a map.
4. **The rulebook's page on where a map lies** (chapter 2) is rewritten for two kinds of frame, by the architect, in the same pull request.
5. **Left for the steps that follow, as planned:** the map and its file of located positions; the table, the viewer and the scripts, none of which has learned the second kind; the check of passages, which stops on a map about a centre.


## Step 2: the map and its file of located positions (Nov 28, 2026)

What step 2 makes. The map from Papua to Midway, its anchors file, a new map tool command that writes a map's file of located positions, and that file for this map. No rule of the simulation changes and nothing in `sim-core`, `sim-data`, the table, the viewer or the scripts. The two present maps are not touched.

### The command lines used

The map was made from a scratch folder outside the worktree, `/private/tmp/claude-501/wide-map-step-2/gen`, holding copies of `data/map/frames/papua_to_midway.toml` and `data/map/papua_to_midway_anchors.csv` at those relative paths and `references/natural-earth/ne_10m_land` and `ne_10m_minor_islands` as symbolic links to the two shapefile folders, so that the command the header records is run as written. The tool was built `--release` from this branch.

```
map-tool generate --land references/natural-earth/ne_10m_land/ne_10m_land.shp --minor-islands references/natural-earth/ne_10m_minor_islands/ne_10m_minor_islands.shp --frame data/map/frames/papua_to_midway.toml --out data/map/papua_to_midway.csv --anchors data/map/papua_to_midway_anchors.csv
```

The file was copied into the worktree. It took 24 seconds. The file of located positions was made from the repository root:

```
map-tool locate-rows --map data/map/papua_to_midway.csv --positions data/oob/place_positions.csv --out data/map/papua_to_midway_located.csv
```

### The map's counts

- 10,395 hexes in rows -60 to 58: 344 land, 365 coast, 9,686 deep sea. These are the numbers of the hand calculation.
- 84 hexes have more than one body of water: 48 with two, 17 with three, 13 with four, two each with five and six, one each with seven and eight.
- 77 closed sides between two hexes that are both not land, each pair counted once.
- 97 hexes are made coast by a land point at 0 percent. The two marks are not among them, so the map has 99 coast hexes of land percentage 0.

### The marks

Midway, row 5100, is hex (31,-41) and Wake, row 5107, is hex (2,-26). Each is coast, land percentage 0, one body of water, and named in the header's "Marked land" line. They are the two marks the architect's frame file names; none was added.

### The file of located positions

113 lines, one for each row of `data/oob/place_positions.csv`; every row is inside the extent. Every line is at the hex the hand calculation gives. By the map's kinds the 113 are in 107 coast hexes, 2 land hexes and 4 deep sea hexes.

- In land hexes: row 44, Lae (hex (-49,18), land 87 percent), and row 98, Townsville (hex (-59,40), land 95 percent). Rule 9 leaves these as they are.
- In deep sea hexes: row 99, Point ZED (a reference point at sea), and three Hawaiian islets, row 5110 French Frigate Shoals (hex (43,-34)), row 5112 Lisianski (hex (34,-37)) and row 5113 Point Luck (hex (39,-47)). The sampling found no land in these three hexes. They are not marked: the frame file, which the architect owns, marks two places. Rule 7 says the build finds its own list; this is the list of places whose rows sit in a deep sea hex, for the architect to rule on.

### The anchorages

The anchors file is the south-west Pacific map's seven anchorages in the same columns, with a new comment header; no anchorage was added. On this map: Rabaul (-39,14), Buka (-37,16), Tulagi and Lunga Point (both (-32,22)) and Noumea (-33,44) stand for the main body of water they were already in, with no change. Milne Bay, hex (-47,24), changed its hex's main body of water, from 65 points to 11. Espiritu Santo, hex (-27,33), was ignored: its body of water has 1 point, fewer than 5, and the main body stays at 249.

### File sizes

`data/map/papua_to_midway.csv` 403,983 bytes; `papua_to_midway_located.csv` 5,052 bytes; `papua_to_midway_anchors.csv` 1,829 bytes.

### What changed, by file

- `crates/map-tool/src/located.rs` (new) and `lib.rs`: `locate_rows` and `located_text`. A row is inside the extent when its latitude is from the south to the north edge and its longitude, brought into the frame's run, from the west to the east edge. Its hex is `Projection::hex_of`. The module's doc comment says the map tool may read `data/oob/place_positions.csv` for this and nothing else under `data/oob`. The `csv` reader the tool already uses reads the file, so a quoted comma works.
- `crates/map-tool/src/main.rs`: the command `locate-rows --map --positions --out`, in the usage text.
- `crates/map-tool/tests/papua_to_midway.rs`: tests below the hand calculation, which is not changed.
- `crates/map-tool/tests/committed_map.rs`: the check that the generated maps found include `papua_to_midway.csv` is widened to name it, as it names the other two.
- `docs/commands.md`: the command and the map.
- Data: `data/map/papua_to_midway.csv`, `papua_to_midway_anchors.csv` and `papua_to_midway_located.csv`.

### Tests added

In `crates/map-tool/tests/papua_to_midway.rs`: the header's lines, the title, the frame file's lines, the anchors line and the command; the 10,395 hexes and rows, in order and as `extent_hexes` gives them; the two marks (coast, 0 percent, one body of water, named in "Marked land", and 99 coast hexes of 0 percent against the header's 97); the counts by kind; the anchors file's rows equal to the present map's, and one header line for each; every one of the 113 located rows against the hex of the hand calculation, with the row's own text for latitude and longitude; the committed located file byte for byte what the command gives; and a row outside the extent has no line, the order is the ids' and a quoted comma in the positions file reads.

### Settled here, not in the brief

- The located file's header has no date, so the file is the same whenever it is made. It says it is generated and never edited by hand, and gives the command.
- Its header carries no line that begins "Projection:" or "Extent:", so that `sim-data` finds no frame in it and `committed_map.rs`'s walk takes it for no map.
- A row inside the extent whose nearest hex is not on the map is an error, not a skipped line. None is.
- A place with a comma or a quote in it cannot be written in the file and is an error. None is.
- The anchors file's comment header names this map and says it began as a copy; its rows are byte for byte the copy's.
- The test for the anchors file reads the present anchors file, so a change to that file would fail it, as it should while the two are to be the same list.

### What walks `data/map/` and takes the new files

Looked at and found to take none of the three new `.csv` files for something they are not: `crates/map-tool/tests/committed_map.rs` (the one walk of the folder; it takes each `.csv` whose header gives a frame, and the anchors and located files have none); `crates/sim-cli/tests/export.rs`, `crates/sim-staff/tests/holds.rs`, `crates/sim-cli/src/export/measure_fixture.rs`, `crates/sim-cli/src/validate/passages.rs` and the scenario tests under `tests/` (each names one map file); `table/serve.py` and `table/scenarios.json` (a scenario's map is named in the entry), `table/check_screen.py` (names two folders by path), `scripts/orders-inside-proof/run.sh` (copies the folder whole), `scripts/regenerate.sh`, `scripts/web/site.py` and `.github` (no walk of the folder). Every other `read_dir`, `glob`, `listdir` and `os.walk` in `crates`, `tests`, `table`, `scripts` and `viewer` is on another folder.

### Checks run

- `cargo fmt --all -- --check`: passes.
- `cargo clippy --workspace --all-targets -- -D warnings`: passes.
- `cargo test -p map-tool -p sim-data`: passes, with the eight new tests.
- `cargo test --workspace`: the scenario tests ran 3,112 passed and 5 failed. Run alone, each of the five passed: scenario 36's, scenario 40's and scenario 41's "saves and the staff checks" tests and scenario 42's two named. Every other test of the workspace passed (2,800, none failed) in a second run that left the scenario tests out.
- `cargo run -p sim-cli -- export data/scenarios/04a_night_action.toml`: ran.
- `sh scripts/regenerate.sh --check`: "every generated file is what the engine gives".
- `data/map/south_pacific.csv` and `data/map/midway.csv` are not changed.

## The architect's reading of step 2

Made on Nov 28, 2026 on reading the build.

1. **The hand calculation holds as written.** The map has the 10,395 hexes of step 1's part 3; the two marks are in the hexes worked; all 113 rows of the located file are in the hexes the architect worked apart from the tool, compared by him again line by line.
2. **The measure of rule 5, from the rows.** At each of the 30 places with a preferred row of position, a leg of 250 miles on the map in 24 directions reads within 3.6 in 100 of the true leg (`scripts/wide-map/projection_error.py`'s measure, on the rows' positions). The worst is 3.6 short, along a line through the centre, as the plan's rule 5 says. Midway reads 3.6 short to 0.4 short, Rabaul 3.6 to 0.8 short, Lunga Point 3.6 to 1.7 short, Port Moresby 3.6 short to 1.1 long, Saipan 3.6 short to 1.1 long. Pearl Harbor, Brisbane and the other places of the plan's table have no row of position yet.
3. **French Frigate Shoals and Lisianski are not marked.** No base is wanted at either, and they are names only on the Midway map too (decision 0199, rule 29). Point Luck and Point ZED are points at sea. A mark is added when a scenario bases something at a place and a row gives its position.
4. **Johnston, Palmyra, Canton, Wotje, Maloelap, Eniwetok and Nauru are not marked,** for want of a row of position. They wait on the research (`prompts-1942.md`, W07). Until then nothing can be based at them on this map.
5. **Espiritu Santo's anchorage does not steer its hex here.** On this map's lattice the water at the anchorage's position is a body of one point, under the five decision 0017 asks, so the hex stands for its largest body. It is left so and looked at again when a scenario puts a force there; Milne Bay's anchorage does steer its hex, as on the south-west Pacific map.
6. **Lae and Townsville fall in land hexes.** That is the plan's rule 9: the record that plans the first scenario on the map says which coast hex stands for each.
7. **The located file follows its rows.** A row of position added inside the extent means the file is made again in the same pull request; `docs/engineering/layout.md` says so. The research rows of Nov 28, 2026 added none.
8. **Five tests that save a game failed in the developer's full run and passed alone** (scenarios 36, 40, 41 and 42). They read no map of this change and are reported to fail together on `main` too. Not this change; reported to the designer as a fault to look at.



## Step 3: the table laid out in a map's plane (Nov 28, 2026)

What step 3 makes. The table's page lays its chart out in the map's plane, so that it can draw the map from Papua to Midway. The bridge reads both kinds of frame. The coast and the names of the new map, the shared file of checked positions, and a placeholder game are made. No rule of the simulation changes and nothing in `sim-core`, `sim-staff` or `sim-trial`. The two present maps, their coast files and their names files are not changed by one byte. The viewer, the check of passages and the export are not touched: they are step 4.

### What changed, by file

- **`table/index.html`, commit A.** Two read-only hooks and nothing else: `window.flagplotLayout()` gives `{TW, TD, K, HEXR, hexes}` and `window.flagplotChartHash()` gives an FNV-1a (32 bits, as hex) of the base chart canvas's `toDataURL()` with the board's width and depth and the hex radius to six decimals.
- **`table/index.html`, commit B.** One function, `plane(lat, lon)`, gives east and north in nautical miles from the centre of hex (0,0), the longitude in the frame's run: for a frame of one factor east is (lon - centre_lon) x 60 x factor and north is (lat - centre_lat) x 60; for a frame about a centre it is part 1 of `crates/map-tool/tests/about_a_centre.rs` with 3437.746771 and the frame's scale (the angle from its sine and cosine, as the map tool does it); with no frame a degree of latitude and a degree of longitude are both 60 miles. `world(lat, lon, y)` is that point centred on the board and scaled to it, so every caller of `world` is as it was. `worldOff(lat, lon, east, north)` is a point so many miles east and north of a position in the plane. `hexOfPlane(east, north)` is the hex whose centre is nearest a point of the plane. `layChart` takes the board's width, depth and hex radius from the planes of the hexes. The lines of latitude and longitude, the ticks along the border and the outside of the map are drawn as the brief says (below). The raycaster's own variable `plane` is now `tablePlane`, since the new function took the name.
- **`table/serve.py`.** `read_frame` and `read_frame_and_edges` read both forms of a header. A frame about a centre is served with `kind` "about a centre" and `scale`; a frame of one factor is served with the keys and values it has always had. A map with projection lines that cannot be read as either kind is a `ValueError` that names the map.
- **`scripts/web/site.py`: not changed.** It writes `load_chart`'s frame as it is, so the browser build carries the new kind. Run alone it makes the site as before (the frames of the two generated maps are what they were, and the hand-written map's is `null`), and the frame of the new map, passed through `load_chart` and written as the script writes it, carries `kind` and `scale`. The full build (`scripts/web/build.sh`) and `web/check.mjs` were not run: `wasm-bindgen-cli` is not installed here.
- **`table/chart/clip_coast.py`.** Its docstring says it reads either kind through the bridge's reader. Its code is not changed: it already read the frame through `read_frame_and_edges`. It made `table/chart/papua_to_midway_coast.json`, 631 rings and 19,315 points, with `--map data/map/papua_to_midway.csv` from the two Natural Earth shapefiles under `references/`, which are not committed. Run again for the Midway map it gives `midway_coast.json` byte for byte.
- **`table/chart/papua_to_midway_names.json`.** 28 names: the 23 of `south_pacific_names.json` and the 5 of `midway_names.json`, each at the position its file gives. None is left out and none is added. The seas are in it at the positions the south-west Pacific file gives for them, which rest on no row; they were in a names file already.
- **`crates/map-tool/src/fixture.rs` (new), `main.rs`, `lib.rs`, `located.rs`.** The command `plane-fixture --out FILE [--positions CSV]` writes `table/chart/plane_fixture.json`: one entry for each of the three generated maps, with the file's name, the frame as the bridge serves it and the points (222 in all: 97 for the south-west Pacific map, 12 for the Midway map, 113 for the map about a centre, the lines of its located file). A point is the row, the latitude and longitude as the row gives them, east and north in the plane to three decimals, and the hex. `located.rs` gains `rows_inside`, the reading of the positions file that `locate_rows` did in its body, so that the fixture reads the file one way; `locate_rows` gives what it gave and the located file is byte for byte the same (its test passes).
- **`crates/map-tool/tests/plane_fixture.rs`.** Tests under the architect's doc comment, which is not changed.
- **`table/check_screen.py`.** Two new functions, run first: `the_bridge_reads_both_kinds_of_frame` and `the_plane_agrees_with_the_shared_file`. The second cuts the page's own `plane` and `hexOfPlane` from its text and runs them in node over every point of the file, as the page's other functions are run.
- **`data/scenarios/wide-map/placeholder_the_wide_sea.toml`, `tests/scenario_the_wide_sea.rs`, `tests/all.rs`.** The placeholder game and its test.
- **`docs/commands.md`, `table/README.md`.** The command, and the page's plane, the second kind of frame, the fixture, the hooks and the wide sea.

### The hashes and the layouts, before and after

Made with the table's bridge on the same scenario each time, in the browser pane of the desktop app. Before is commit A (`efcc1af3`) and after is commit B. The games: scenario 5c (`data/scenarios/second-proof/05c_eastern_solomons.toml`) on the south-west Pacific map; `data/scenarios/table-boats/01b_midway_submarines.toml` with `--map data/map/midway.csv`; `data/scenarios/ladder/138_one_ship.toml` with `--map data/map/placeholder_ladder.toml` for the hand-written map. The base chart depends on the map alone, not on the game.

| Map | Layout, before and after | Hash as loaded, before | Hash as loaded, after | Hash with the canvas's blur off, before and after |
| --- | --- | --- | --- | --- |
| South-west Pacific, 1,600 hexes | TW 10, TD 8.591958229875925, K 0.3703701663430477, HEXR 0.1425638851581159: the same in all four | 144b37e7 | 144b37e7 | 3a302d2b, 3a302d2b |
| Midway, 1,418 hexes | TW 10, TD 7.459211539496315, HEXR 0.14081804433331546: the same; K 0.3658538656348586 before and 0.36585386563485867 after, one unit in the last place | f4eac523 | f4eac523 on some loads, not on others (below) | 39b6dc7d, 39b6dc7d |
| Hand-written (the ladder's), 209 hexes | TW 10, TD 8.294228585288568, K 0.7499999625000018: the same; HEXR 0.2886752644986169 before and 0.28867526449861697 after, one unit in the last place | 6c0f765c | 6c0f765c | e92e09db, e92e09db |

The two last digits that differ are the order in which the floating-point sums are made (the plane's and the old degrees'); every hash, which holds the sizes to six decimals, is unchanged.

**What the proof rests on, and how it was taken.** The hash of a picture made with the canvas's blur filter is not steady. The page paints the coast with `ctx.filter = 'blur(...)'`; the browser's blur does not give the same pixels twice. Commit A itself, loaded five times in an iframe on the Midway game, gave a0a24e18, 658fa114, f4eac523, d7ab6ea4 and f4eac523, and a repaint of the south-west Pacific chart gave c3d063f9 once in four. So a hash as loaded can be compared only where it is steady, and on the Midway map it is not. What was compared instead: the same page (a copy of commit A from `git archive` beside the changed page, each with a temporary hook that repaints and which was taken out before the commit) with the setter of `CanvasRenderingContext2D.prototype.filter` replaced by one that does nothing, repainted four times, the hash read each time; every one of the four gave one value on each page, and the two pages gave the same value for each of the three kinds (the last column). The same Midway chart, loaded at commit A and at commit B, was also compared tile by tile (100 by 100 pixels, 300 tiles): on the one load at each where the picture was steady, none of the 300 differed. The hash hook as committed does not repaint; it reads the picture as the page last drew it.

### What was found that worked in latitude and longitude, and what was done

- `world(lat, lon, y)`: now the plane (above). Every caller is as it was, and the hex centres, the names, the coast, the land drawn from the hexes, the forces' places and every route drawn through `world` are placed by it.
- `layChart`: the board's width, depth and hex radius from the planes of the hexes. For a map of one factor, the depth is padded as it always was by a hex's width in degrees of longitude taken as degrees of latitude: a hex's width over the factor, in miles. For a map with no frame the factor is 1 and for a map about a centre the pad is a hex's width, 40 miles.
- The lines of latitude and longitude and the ticks along the border: for a map about a centre, curves (below); for the others, straight as they were, each through `world`.
- The DENY ring and the PATROL ring (`world(h[3] + miles / 60, h[4])` before): `worldOff(h[3], h[4], miles, 0)`.
- `reachShape`, which draws a search arc, a strike's reach, a suspected waters' ring and a following ring (`lat + d cos b / 60`, `lon + d sin b / (60 FX)` before): `worldOff(lat, lon, d sin b, d cos b)`. A bearing is the grid's, north being up the page.
- `compassWord`, which says north-east and the like for a march (the bearing worked with the cosine of the latitude of the hex marched from, before): the bearing is worked in the plane, from the planes of the two hexes. For a hex's neighbour the word is the same as before.
- `hexMiles` (a great-circle distance between hexes (0,0) and (1,0), rounded to five, before): the plane distance between them, rounded to five. It is 40 on every map, as it was.
- `FX` and `lonMid` and `latMid` as page-wide variables are gone. A local `latMid` and `lonMid` remain in `layChart` for the straight lines of the maps that have them.
- Found and not changed: the pointer's position is read as a point on the table and taken to the nearest hex through `world`; no readout turns it into a latitude and longitude. So no inverse of the plane is in the page.

### The lines of latitude and longitude, the board's edge, and the hexes

On a map about a centre each line of latitude and longitude is sampled every half degree through the plane (parallels over the hexes' longitude range and 8 degrees more each way, meridians over the latitude range and 8 degrees more), kept where it is on the board, and drawn. A line is drawn every ten degrees on such a map when its hexes span more than 30 degrees of either, else every two; no ticks are drawn along the border. Each line is lettered where its last point on the board is: a meridian at the foot, a parallel at the east, set in the chart's ink with a pale halo so that it reads over the dark tone. The part of the board outside every hex of the map is painted in one flat tone, `#4b6371`, over everything drawn before it and under the names, the hex overlay and the border's frame. The map's own chart has 6 meridians (150 E to 160 W) and 6 parallels (20 S to 30 N), drawn as curves.

### Settled here, not in the brief

1. **A hex is placed through the plane from the latitude and longitude the map file gives it,** not from its (q, r) as the plan's rule 17 says. The map files give four decimals (0.0001 degree is under 0.006 nm), and the three present kinds must come out as they are today, which they do only if the hexes are placed from the same numbers. A neighbouring hex of the new map is 39.997 nm away on the table, not 40. If the architect would have a hex placed from (q, r) for a map about a centre, it is one line (`plane` is the only function that would change) and the present maps would be unaffected.
2. **The rule of a line every ten degrees on a map "more than 30 degrees across" is applied to a map about a centre only.** The Midway map is 31 degrees across, and its chart must not change.
3. **The served frame of one factor has no `kind` key,** so that it is exactly what it was. The brief says both that the frame carries `kind` and that a frame of one factor is exactly as it is. The page reads a frame with no `kind` as of one factor, as a frame file and the export do (the export of a map of one factor has none). A frame about a centre carries `kind` and `scale`.
4. **`K` in `flagplotLayout()` is board lengths to a degree of latitude,** as it was, so that the number is comparable; the plane's own scale is `KP`.
5. **`rows_inside` was made in `located.rs`** (a small refactor of the map tool's own reading, class 2) so that the fixture and the located file read the positions file one way.
6. **The tone outside the map, `#4b6371`, and the halo on the letters** are the developer's; the brief said only plain, darker and flat.
7. **The rule 18 file's note, and the order of the three maps** (south-west Pacific, Midway, map from Papua to Midway) are the developer's.
8. **The python that runs the bridge is 3.11 or later** (`tomllib`). This machine's `python3` is 3.9; the checks here were run with 3.12. CI's is the runner's.

### Where this departs from the plan's wording

- Rule 17 says a hex is placed "from its (q, r) in map miles, as the viewer does" (settled 1 above).
- The brief names nine hand-worked points in part 1 of `tests/about_a_centre.rs`; the doc comment of `tests/plane_fixture.rs` lists eight, each with a row of the positions file. The ninth in part 1's table is the centre itself, which is no row of the positions file, so the fixture has no point for it. The test checks the eight in the file and the centre by the projection (east 0, north 0, hex (0,0)).
- The game is **not on the new game screen's list**; see below.

### The game, and what stopped it

`data/scenarios/wide-map/placeholder_the_wide_sea.toml` is on `data/map/papua_to_midway.csv`, three days, no objectives, four bases (Placeholder base at Midway (31,-41) and at Lunga Point (-32,22), Allied; at Wake (2,-26) and at Rabaul (-39,14), Japanese) and one force of one placeholder cruiser and one placeholder destroyer (classes 1 and 2 of `data/reference/ship_classes.csv`) at each, with no order. It opens on the table from the command line:

```
python3 table/serve.py data/scenarios/wide-map/placeholder_the_wide_sea.toml --map data/map/papua_to_midway.csv
```

An entry for it was written for `table/scenarios.json` (the ladder's shape: `placeholder` true, no start, `"engine": "absent"`, one pairing of a person at the Allied seat) and **taken out again**. The architect's checks of the list (`L2` and `K2` in `check_screen.py`) hold that every entry after the first two has no `placeholder` key, has a start and has no pairing with the engine absent, and `L1` and `K1` hold the first two entries; an entry anywhere in the list fails one of them. They cannot be changed by the developer. The entry that fits the list's pairings is:

```
{
  "id": "the-wide-sea",
  "map": "data/map/papua_to_midway.csv",
  "name": "Placeholder: the wide sea",
  "dates": "An invented game",
  "days": 3,
  "placeholder": true,
  "lines": ["A game to look at the map from Papua to Midway with: four bases, a small force at each, three days and no enemy to meet.", "A placeholder game: everything in it is invented. Nothing is ordered on either side."],
  "sides": {"allied": "Look at the map: Midway and Lunga Point are your bases."},
  "islands": [],
  "pairings": [{"allied": "person", "japanese": "engine", "engine": "absent", "file": "data/scenarios/wide-map/placeholder_the_wide_sea.toml"}]
}
```

It is for the architect to say where it goes and which of the four checks he changes. The seat sees only its own side's units and bases as in any game; nothing of the enemy's truth is shown for this to work.

### Tests added

- `crates/map-tool/tests/plane_fixture.rs`, under the doc comment: the committed file is what the command gives; it has an entry for each of the three maps, in order, with a point for each line of the located file and for each row inside the extent of the others; the eight rows of part 1 as the doc comment prints them (east and north to 0.0005 nm, the hex exact) and the centre; Lunga Point on the south-west Pacific map (east 1.679, north 155.677, hex (2,-4)); and every point's hex is the nearest to its plane position.
- `tests/scenario_the_wide_sea.rs`: it loads on the map (10,395 hexes), is a placeholder of three days named "Placeholder: the wide sea"; each base and force is at the architect's hex, a coast hex of the map, a base named "Placeholder base at ..."; it runs its three days with no order from either side, ends, and every force lies where it began.
- `table/check_screen.py`: `the_bridge_reads_both_kinds_of_frame` and `the_plane_agrees_with_the_shared_file`: the page's plane agrees with all 222 points to 0.01 nm (worst 0.0005) and gives the same hex for each.

### Checks run

- `cargo fmt --all -- --check`: passes. `cargo clippy --workspace --all-targets -- -D warnings`: passes.
- `cargo test --workspace --no-fail-fast`: 5,927 passed and 4 failed. The four are scenario 36's, 40's and 41's "saves and the staff checks" tests and scenario 42's `d_a_play_game_of_another_doctrine_version_is_refused_and_names_it`, the tests that save a game and are reported to fail when the whole binary runs together. Each was run alone, with `--exact`, and passed. Every other test passed, among them every test of `map-tool`, `sim-data` and `sim-cli` (the `timing` test, which took about 25 minutes in the debug build on a busy machine, passed) and the three tests of the wide sea.
- `python3 table/check_screen.py`, with python 3.12 and `sim-cli` built in release, as CI runs it: "all held (1445 checks)", the two new functions among them.
- `sh scripts/regenerate.sh --check`: "every generated file is what the engine gives".
- `map-tool plane-fixture --out` run in release as well as in the debug build the test uses: the same bytes as the committed file.
- The web site script, `python3 -I scripts/web/site.py FOLDER`: makes the site; the charts of the three maps the list offers carry the frames they had. `scripts/web/build.sh` and `node web/check.mjs`: not run (`wasm-bindgen-cli` is not installed on this machine).
- The page, loaded from `python3 table/serve.py` on ports 8801 to 8804 (not 8742; each stopped): the south-west Pacific map, the Midway map, the hand-written ladder map and the wide sea. There is no headless browser in the repository's own checks, so the built-in browser pane of the desktop app was used: each page booted (no "The table could not reach the game" in its text), no resource failed except `/names` for the hand-written map, which has no names file and never had, and the layout of each was read (the wide sea: TW 10, TD 10.862208143188871, HEXR 0.060777836354910054, 10,395 hexes). The pane's console log was empty throughout; it may not have recorded errors from before the log was first read, so "no console error" is not shown as strongly as it would be by a headless run. The wide sea's chart was looked at: the curved meridians and parallels, the dark tone outside the map, the coast, the names and the three-dimensional table with the bases. Rings and search arcs on the wide map were not drawn by an order in a browser; `worldOff` was measured on it: a ring's edge 40 miles east or north of a hex is 40 miles in the plane, and `compassWord` gives north-west for the hex above-left.
