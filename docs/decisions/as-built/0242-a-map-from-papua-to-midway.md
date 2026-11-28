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

