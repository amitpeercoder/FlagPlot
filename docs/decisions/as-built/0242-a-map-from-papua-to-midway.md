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

