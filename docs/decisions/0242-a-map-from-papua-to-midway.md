# 0242: A map from Papua to Midway

Status: a plan by the architect, written on Nov 28, 2026 at the designer's asking ("Start the map plan"), for the first stage of `docs/design/one-map-1942.md`. Read on Nov 28 by a reviewer who had not seen it written, against the code, and rewritten to that reading ("Amended on review", at the end, says what changed). Approved by the designer on Nov 28, 2026 ("Approved", with the three questions at its end before him: the second kind of projection, the extent, and one clock for the whole map). It is built in the order it gives; what is built is recorded in `docs/decisions/as-built/`.

Summary: The game has two maps, each of one sea, and each flat with one factor for longitude. A game of 1942 from Papua to Midway needs one map of both seas and what lies between, about 10,400 hexes. Measured, the present way of flattening would put the carrier battle off Midway 8 to 10 in 100 out east to west and the Solomons 5 to 7. This plans a third map with a second kind of projection, about the map's own centre, which holds every named place within 4 in 100; says what every reader of a map must learn for it; and leaves the two present maps, and every game on them, as they are.

Standing: Planned by the architect and amended on review; approved by the designer on Nov 28, 2026; steps 1 to 4 of five are built

## Why

The designer, on Nov 27, 2026: "I actually want Midway on one wider map. I want all these to be on the map (Papua, Midway, and the other islands related to this scenario)." The design note (`docs/design/one-map-1942.md`) says why one map and not a series of battles: distance becomes a decision, and ships and aircrew persist.

What is so today:

- **A map is flat with one factor.** East in miles is the difference of longitude times 60 times one number, the cosine of one latitude; north is the difference of latitude times 60 (decision 0017, rule 2; decision 0199). It is true along one parallel and wrong everywhere else, more with distance from it.
- **Decision 0017 held that error to 5 in 100** on the south-west Pacific map and said "the whole Pacific needs its own scheme". Decision 0199 accepted 8 and 11 in 100 at the edges of the Midway map, since the battle was fought near its middle.
- **No rule reads where a map lies.** Distances are counted in hexes. A ship is charged straight-line miles between hex centres (decision 0058) and an aircraft flies the straight line (decision 0040). A bearing is read on the grid, north being the direction of decreasing r (chapter 3; `sim-core`'s `bearing.rs`). So an error of the map is an error of every range, passage and bearing on it, and of nothing else.
- **Floating point is allowed in the map tool and nowhere else that matters here.** `docs/engineering/layout.md` gives it to `map-tool` alone, and nothing but the `tests` crate depends on that crate. `sim-data` passes no value through floating point and is built into the browser's module. The check of passages turns a position into a hex in whole numbers and says "no float anywhere".
- **Many things read a map's frame,** each with the one-factor formula written into it. Rule 16 lists them.
- **A path over the sea is worked out afresh** as a field of distances over the whole map (`Map::sea_distances_to`), and never stored (decision 0017, rule 12).

## What was measured

By `scripts/wide-map/projection_error.py`, on Nov 28, 2026. Its positions are approximate, to a tenth of a degree, and are for this measurement only. For each scheme it reads a leg of 250 miles on the map in 24 directions from a place and gives the shortest and the longest reading against the true leg, in parts in 100.

Two schemes on the extent of rule 1. **A** is the present kind, with the factor 0.936672, the cosine of 20 30, which is the frame the trial map below was made with. **B** is the scheme of rule 3.

| Place | A, shortest and longest | B, shortest and longest |
| --- | --- | --- |
| The carrier battle off Midway (31 30 N, 179 W) | 0.0, +9.9 | -3.6, +0.4 |
| Midway | 0.0, +6.4 | -3.6, -0.4 |
| Pearl Harbor | -0.2, +1.0 | -3.6, +1.6 |
| Truk | -5.5, 0.0 | -3.6, -1.0 |
| Kwajalein | -5.2, 0.0 | -3.6, -3.3 |
| Tarawa | -6.3, 0.0 | -3.6, -3.6 |
| Rabaul | -6.1, 0.0 | -3.6, -0.8 |
| Port Moresby | -5.0, 0.0 | -3.6, +1.0 |
| Lunga | -5.1, 0.0 | -3.6, -1.7 |
| Noumea | 0.0, +1.5 | -3.6, +0.2 |
| Brisbane | 0.0, +5.7 | -3.5, +3.6 |
| The north edge, 38 N | 0.0, +18.8 | -3.6, +2.3 |
| The four corners | 0.0, +8.2 to +18.8 | -3.6, +7.1 to +7.6 |

Under A the error is all east and west; north and south is true. Under B a line toward or away from the centre reads 3.6 short everywhere, and a line across it reads from 3.6 short at the centre to 3.6 long at Brisbane.

**A at its best.** The single factor whose worst reading among these places is smallest is 0.920. It reads 8.0 long at the carrier battle off Midway and 4.6 at Midway, and 8.0 short at Tarawa, 7.8 at Rabaul, 7.2 at Truk and 6.8 at Lunga. No factor does better than 8.0 at worst; the table's is one trade among them, kinder to the south.

For comparison, the same measure on the maps the game has: Lunga -0.9 and Rabaul -1.9 on the south-west Pacific map, Noumea +5.8; the carrier battle off Midway +3.4 on the Midway map.

Long passages, straight line on the map against the true distance:

| Passage | True, nm | A | B |
| --- | --- | --- | --- |
| Pearl Harbor to Midway | 1,135 | +2.9 | -0.3 |
| Pearl Harbor to Noumea | 3,348 | -1.5 | -3.1 |
| Truk to Midway | 2,145 | -0.5 | -2.3 |
| Truk to Rabaul | 696 | 0.0 | -1.1 |
| Rabaul to Lunga | 560 | -3.8 | -1.8 |
| Truk to Kwajalein | 948 | -5.4 | -3.5 |

Two other projections about the centre were measured in a first pass, on a slightly smaller extent: one that keeps shapes (stereographic), 5.4 at worst among the named places, and one that keeps areas (Lambert's), 5.1. Neither beats B, and each costs what B costs.

**Where north is under B.** The grid's north is true north only on the meridian through the centre. Worked from rule 3, true north lies on the grid at these bearings:

| Place | True north, as a grid bearing |
| --- | --- |
| Midway | 357 (3 degrees west of grid north) |
| Pearl Harbor | 352 |
| Brisbane | 353 |
| Saipan | 006 |
| The north-west corner | 015 |

Under A, grid north is true north everywhere.

**A trial map of kind A** was made with the present tool on the extent of rule 1, in a scratch folder. Every figure in this paragraph is of that map, and the map of kind B will differ a little. It had 10,797 hexes (353 land, 367 coast, 10,077 deep sea), 90 with more than one body of water; it took 53 seconds to make and is 418 kB. The sampling found land at Midway, Canton, Kwajalein, Jaluit, Tarawa, Makin, Truk, Funafuti and Ocean Island, and none at Wake, Johnston, Palmyra, Wotje, Maloelap, Eniwetok or Nauru. Brisbane's hex was land, 98 in 100.

**The count under B,** worked from rule 3 without making a map, is 10,395 hexes.

**The field of distances** over the sea takes 0.65 ms on the south-west Pacific map and 4.3 ms on the trial map, in step with the count of hexes. A copy of the map takes a tenth of a millisecond.

## The rules

### The map

1. **Its extent:** every hex whose centre lies from 30 S to 38 N and from 142 E across the date line to 153 W. Hexes of 40 miles, as everywhere: every range in the rules is counted in them. About 10,400 hexes.
2. **What that takes in and leaves out.** In: Papua and north-east New Guinea, Australia from Horn Island to Brisbane, the Bismarcks, the Solomons, the New Hebrides, New Caledonia, Fiji, Samoa, Tonga, the Ellice and Gilbert Islands, Nauru and Ocean Island, the Marshalls, the Carolines from Truk east, Saipan and Guam, Wake, Midway, Johnston, Palmyra, Canton and the Hawaiian Islands. The north edge is 38 N, the Midway map's, so that a force bound for Midway from the north-west has the sea it used. Out: Japan, the Aleutians, the American coast, Sydney, New Zealand, Palau, the Philippines and the Indies. A force from any of them comes in at an edge (decision 0195).
3. **Its projection is about its own centre.** The earth is taken as a sphere of 60 miles to the degree. The centre is 4 N, 174 30 E. For a point, take its distance from the centre along the great circle, as an angle, and the bearing on which that great circle leaves the centre. Its distance from hex (0,0)'s centre on the map is that angle in radians times 3,437.746771 (the miles in a radian) times 0.964, on that bearing: east is the distance times the sine of the bearing and north the distance times the cosine. Hex (0,0) is centred at the centre. A point belongs to the hex whose centre is nearest, as on every map. Worked the other way, from the plane to a position, the longitude that comes out is brought into the frame's run, above 180 east of the date line, before it is compared with the extent.
4. **0.964 is chosen, not derived.** With no such number the map is true along lines through the centre and long across them, by 7.5 in 100 at Brisbane and 5.4 at Pearl Harbor. 0.964 splits the error: it is the number at which the worst reading among the named places is smallest. It and 3,437.746771 are literals of the frame, as the factor is of the other kind.
5. **Its error is recorded and accepted:** within 3.6 in 100 at every named place and for every leg measured, and up to 7.6 at the four corners, which are open sea. This is inside decision 0017's 5 in 100 wherever anything is. The build measures again from the rows of `data/oob/place_positions.csv` and writes what it finds in the record of what was built.
6. **A bearing on this map is a grid bearing.** It is so on every map; on this one it differs from the compass, by the table above. That is a known limit and is not mended: a search ordered from 300 to 030 at Pearl Harbor covers what a navigator would call 308 to 038. Everything that tells a person or an agent a bearing gives the grid's, which is the one the rules read.
7. **A place too small for the sampling is marked,** by decision 0199's land mark: a line of the frame file citing the place's row of `data/oob/place_positions.csv`. The first marks are the places a base is wanted at that the sampling misses. On the trial they were Wake, Johnston, Palmyra, Wotje, Maloelap, Eniwetok and Nauru; the build finds its own list on the map as made. A place with no row of position is not marked until research gives it one (`prompts-1942.md`, W07).
8. **Its anchorages begin as the present map's.** The anchors file gives positions with their sources, not hexes, so the south-west Pacific map's list is copied and added to in the same columns. The land links of that map are hexes and are not carried over; marches on this map are for the record that plans Papua.
9. **A port whose hex is land** (Brisbane, on the trial) is not settled here. The record that plans the first scenario on the map says which coast hex stands for it.

### A second kind of projection

10. **A frame is of one of two kinds:** "one factor" (decisions 0017 and 0199) and "about a centre" (rule 3). A frame file with no kind is of the first, so the Midway frame file does not change.
11. **The two present maps do not change by one byte.** A map file's bytes are hashed into every saved game. The lines the tool writes for a map of one factor stay exactly as they are (`frame.rs`'s `projection_lines`, guarded by its test that the default frame writes the present header lines); the build makes both maps again with the changed tool and compares, as decision 0199's rule 21 did.
12. **The header of a map about a centre** gives its projection in these lines, with the map's own numbers:

    ```
    # Projection: about a centre. Hex (0,0) is centred at 4 N, 174.5 E.
    #   distance_nm = angle from the centre in radians * 3437.746771 * 0.964, on the bearing from the centre
    #   east_nm = distance_nm * sin(bearing), north_nm = distance_nm * cos(bearing); a sphere of 60 nm to the degree.
    # Extent: every hex whose centre lies from 38 N to 30 S and from 142 E to 153 W.
    ```

    A reader tells the kinds apart by the first line: "Projection: about a centre." against "Projection: hex (0,0) is centred at". A header with some lines of either form and not all of them is refused, as a part of a frame is today. A header with neither has no frame.
13. **`sim-data` reads the lines into fixed-point values and no more:** the kind, the centre, the scale and the extent. It works out no position and no hex for a map about a centre, so it stays without floating point. A map's frame holds its projection as one of two cases, a factor or a scale, and every reader matches on the case, so that a reader which has not learned the second kind does not compile. The map tool reads a map's header through `sim-data` already, so the header's grammar stays in one place.
14. **Only the map tool turns a position into a hex on a map about a centre.** It is the one crate that may use floating point, and the formula is written there, once in Rust, both ways. Nothing depends on the map tool that does not today, and no crate is added.
15. **What a position's hex is, is recorded as data.** For a map about a centre the map tool writes a generated file beside it, `data/map/<name>_located.csv`: each position that something needs a hex for (a row of `data/oob/place_positions.csv`, or a position a passage gives), cited, with the hex it falls in. A test in the `tests` crate, which may use the map tool, checks that the file is what the tool gives. The check of passages reads a place's or a position's hex from that file for a map about a centre, and stops with an error naming any position not in it; for a map of one factor it works in whole numbers as it does today. So no game's outcome and no umpire's report rests on floating point: a scenario names hexes, and a position becomes a hex when a file is written, never while a game runs.

### The readers

16. **Every reader of a frame, and what each does.** The reviewer's list, checked against the code:

    | Reader | For a map about a centre |
    | --- | --- |
    | The map tool: `generate`, `locate`, `water`, its frame file and its reading of a header | Learns the second kind. `locate` today falls back to the south-west Pacific frame when a header fails to read; for a map that has projection lines and fails to read, it stops with an error |
    | The map tool's search for hexes (`extent_hexes`) | Takes its box today from two corners. On this map a meridian bows outward: the west edge reaches 1,882 miles from the centre near 2 S and only 1,559 at the north-west corner. The box is taken from the four edges sampled along their length, at every tenth of a degree, with a hex to spare |
    | The map tool's choice of coastline shapes (`keep_near_extent`) | Pads by the factor today. For a map about a centre the pad is one degree of latitude and of longitude |
    | The map tool's test of the committed maps | Checks that a factor is the cosine of the centre's latitude for a map of one factor only; for a map about a centre, that each centre is where rule 3 puts it |
    | `sim-data`: the frame | Rule 13 |
    | The check of passages | Rule 15 |
    | The export and the viewer | A map of one factor is exported byte for byte as today, since exports are pinned by hash; a map about a centre exports its kind and scale and no factor. The viewer lays hexes from (q, r) already. Its labels go through the projection, and it stops with an error for a frame it cannot read where today it falls back to the cosine of 12 degrees. `viewer/check.js` accepts either kind |
    | The table's bridge (`table/serve.py`) and the site script (`scripts/web/site.py`) | Read and pass on either kind. A map with projection lines that fail to read is an error, not a chart with no frame |
    | The table's page | Rule 17 |
    | The script that clips the coast | Clips to the extent with room to spare, as now, and writes the coast in the run |
    | `scripts/table-boats/make_files.py`, which lays placeholder starts off flat in a map's frame (decision 0227) | Works for a map of one factor only, and says so and stops for the other kind |
    | The two scenario tests that use the map tool's projection (scenarios 4b and 103) | Are on the south-west Pacific map and do not change |

17. **The table lays every map out in the plane.** Today it places everything by latitude and longitude (`world(lat, lon)`), takes the board's size and a hex's radius from the ranges of latitude and longitude, draws the lines of latitude and longitude straight, and draws search arcs and strike rings by laying a bearing and a distance off a position. On a map about a centre the hex centres are not a regular grid in latitude and longitude, so hexes drawn that way would overlap and gap. So: a hex is placed from its (q, r) in map miles, as the viewer does; arcs, rings and bearings are drawn and worked in the plane, on the grid, as the rules read them (rule 6); and the projection is used only for the coast, the names, the lines of latitude and longitude, which are curves on this map, and the position under the pointer. For a map of one factor this gives the chart it has now.
18. **The page and the scripts have the formula too,** since they are not Rust: the table's page, the viewer and the clipping script. The build writes one file of positions with the plane coordinates and the hex the map tool gives for each, and each of the three is checked against it by its own check.
19. **A hex's centre in the file is the authority for what a person or an agent is told.** The latitude and longitude columns are written by the map tool, west negative, as today. Nothing but the tools of rule 18 turns a hex back into a position by formula.

### What does not change

20. **No rule of the simulation, and nothing in `sim-core`.** No scenario exists on the new map when this is built, so no game plays differently and no hash moves. No saved game, recorded game, pinned hash or golden file moves while the two present map files and their exports stay byte for byte.
21. **One clock for the whole map.** A pulse is the same pulse in every hex and the night pulse is dark in every hex (chapter 1). Across this map the sun is more than four hours apart from one edge to the other, and the date line runs through it. That is recorded here as a known limit, as decision 0198's rule 43 was for Midway, and is not mended. Which calendar a scenario's dates are on, where Japanese and American records differ by a day, is for the record that plans the scenario.

### What is measured before anything is changed for speed

22. **A placeholder scenario on the new map,** named as one and kept out of the loops that walk `data/scenarios/*.toml` as decision 0072 says, with forces enough to stand for the size of the first game. Three things are timed on it: a plain run through `sim-cli time`; a day with both staffs at work; and a trial. The field of distances costs six and a half times what it does today for each call, and the callers that make one field a call, not one a waypoint, are where it will show: the staff's choice of a station and of a way, which make one for each candidate hex; a briefing, which makes one for each base; and a trial, which multiplies the staff's cost by its worlds and days. Nothing is cached or changed for speed in this record. If the measure asks for it, that is a record of its own, and decision 0017 already names a cache of path distances as deferred.
23. **The table is tried with the map's 10,400 hexes** on the designer's machine and the server's before a scenario is planned on the map. What it does if it is slow is not planned here.

## Rule card

No rule of the simulation is added or changed, and no state. New: a kind in a frame; a second form of projection lines in a map's header and its reading in `sim-data`; the projection about a centre in the map tool, the viewer, the table and the clipping script; a generated file of located positions and the check of passages' reading of it; a third generated map with its frame file, its anchors, its coast and its names; a placeholder scenario for timing.

The rulebook changes in chapter 2, "Where a map lies", which today defines a frame as "the factor for longitude (the cosine of the centre's latitude)": it is rewritten for two kinds of frame and three generated maps, with the file of located positions. Chapter 3 gains a sentence that a bearing is the grid's and where, on this map, that departs from the compass. `docs/engineering/layout.md` gains the generated file under what the map tool writes. The glossary gains nothing.

## What was weighed

- **One factor, as today (A).** It needs no reader changed: the trial map was made with the tool as it is. At its best it is 8 in 100 out at the carrier battle off Midway and 7 to 8 in the Solomons, at Rabaul and at Truk, where today's maps are 3.4 and 1 to 2. A carrier's strike reach of 240 miles would read about 20 miles long toward the east off Midway. It keeps north true, which B does not. Not taken: the two seas this map joins are the two it would be worst in.
- **A projection that keeps shapes, or one that keeps areas.** Measured; no better than B and the same cost.
- **The formula in a crate of its own, or in `sim-data`.** A new crate so that the check of passages could call the projection; or floating point in `sim-data`, which is built into the browser's module and says it has none. Not taken: recording a position's hex as generated data (rule 15) needs neither, and is what a scenario needs anyway.
- **Larger hexes.** Fewer of them, and every range, search and fight in the rules is counted in hexes of 40 miles. Not taken.
- **Two maps joined by a passage off the map.** The design note's "interlude". The designer chose one map.
- **A narrower map,** Midway to Guadalcanal without Truk, Saipan or Pearl Harbor. The forces for both battles sailed from those three. Not taken.

## What it costs

- **Many readers learn a second kind,** three of them outside Rust, and each copy of the formula is a place to be wrong. Rule 18's shared file of checked positions is what holds them together.
- **The table's layout is reworked,** not adjusted: hexes, arcs and rings move from latitude and longitude to the plane, for every map. It is the largest piece of the work.
- **Grid north is not the compass** on this map, by up to 8 degrees where anything is.
- **One more generated file** to keep in step with its map and the positions it cites.
- **Every later map of a wide sea** can use the second kind. A map of the whole Pacific would need its own measure and perhaps a third.

## In what order

Each is a pull request the architect reads. None changes a rule of the simulation; the rulebook's pages change with the first and the third.

1. The second kind in the map tool and `sim-data`, with the proof that the two present maps and their exports are unchanged. No new map.
2. The map: its frame file, anchors and marks, the map and its file of located positions made and committed, the measure of rule 5 from the rows.
3. The table laid out in the plane, proved on the two present maps before the new one is shown on it.
4. The other readers: the check of passages, the export and the viewer, the coast and names, and rule 18's shared file.
5. The placeholder scenario and the measures of rules 22 and 23.

## Questions for the designer

1. **The second kind of projection, at its cost, or the present kind at none?** The architect proposes the second: within 3.6 in 100 everywhere against 8 at best off Midway and in the Solomons. Its price is the work above and a grid whose north is up to 8 degrees off the compass at Pearl Harbor and Brisbane.
2. **The extent.** Saipan and Guam are on it; Palau and the Philippines are not; the north edge is 38 N; the east edge takes in the whole of Hawaii. Sydney and Auckland are off it.
3. **One clock for the whole map** (rule 21), as a known limit.

## Amended on review

A reviewer who had not seen the plan written read it against the code on Nov 28, 2026. The architect checked each finding the plan now rests on (the crates' dependencies and `layout.md`, `extent_hexes`, the best single factor, the count of hexes, the grid's north) and rewrote the text above; nothing of the first draft stands beside it. What changed:

- **Where the formula lives.** The draft had it "where the map tool and the check of passages can both call it". No such place exists without a new crate or floating point in `sim-data`. Rules 13 to 15 replace it: the map tool alone, and a generated file of located positions.
- **The search for hexes** took its box from two corners and would have cut about eight columns from the west and east edges with no error. Rule 16 has it taken from the edges.
- **The header's lines** are now given exactly, with how a reader tells the kinds apart and what a frame holds (rules 12 and 13).
- **The table** was said to "draw through the projection". It lays everything out by latitude and longitude, which cannot hold on this map. Rule 17 is new, and is the largest piece of the work.
- **The readers** were said to be six. Rule 16 lists them all, with two that fell back silently to the south-west Pacific frame or to the cosine of 12 degrees.
- **Bearings.** The draft said an error of the map is one of ranges and passages only. Grid north departs from true north on this map; rule 6 and its table are new.
- **The figures.** "The best single factor" was not the best: 0.920 is, at 8.0 in 100. The map of kind B has about 10,400 hexes, not 10,800. Without its scale B is 7.5 long at Brisbane and 5.4 at Pearl Harbor, not 7 at both.
- **The measures of rule 22** now name a day with the staffs and a trial, and the callers where the cost will show.

Not checked by the reviewer, and so resting on the architect's measurement alone: the trial map's counts and the islands its sampling found, and the two timings of the field of distances.
