# 0199: A second map

Status: a plan by the architect, written on Nov 20, 2026. It is stage 0 of decision 0198, Midway, and it amends decision 0017, whose projection was "for the slice only". To be read by a reviewer who has not seen it written, and amended on that reading, before any of it is built.

Summary: The game has one map, of the south-west Pacific, and the tool that makes it, the table that draws it and several other tools have that map's place on the earth written into them. The engine has not: it counts in hexes. This makes a map's place on the earth a thing each map says of itself, makes a second map for Midway that crosses the date line, lets an atoll too small for the sampling be land enough to land on, and has the table, the viewer and the tools read the map a game is played on.

Standing: Planned on Nov 20, 2026; waits for its review.

## Why

Midway (decision 0198) is in another sea. Its review found what stands in the way, and what does not.

What is so today:

- **The map tool** has the centre of the present map (12 S, 160 E), its factor for longitude (0.978148, the cosine of 12 degrees), its extent and parts of its header as constants. `locate` and `water` use them whatever map they are given.
- **The engine reads nothing of where a map is.** Darkness is by the clock, a hex's latitude and longitude are stored and neither hashed nor read, and distances are counted in hexes. A scenario names its own map file.
- **Land is sampled** at points 2 nm apart; a hex is coast when at least one point is land. Five small islands of the present map (Tikopia, Anuta, Sikaiana, Ontong Java, Duff) sit in hexes read as deep sea. Midway's two islands are smaller than the square one point stands for.
- **A hex that is deep sea** takes no land unit, forms no base when men unload there, and is no place a staff will take.
- **The table** loads one chart, the present map's, when its bridge starts, for every scenario; draws it with one scale for latitude and longitude; letters every meridian east; and has its coast and names files, and the script that clips the coast, fixed to the present map.
- **Elsewhere:** the check of passages in `sim-cli` has the present centre and factor as constants; the viewer's labels use the cosine of 12 degrees; the tool server's opening words say "a hex map of the Solomon Islands".
- **Decision 0017** held the error of its flat projection to 5 in 100 across the slice.

## The rules

### A map says where it is

1. **Every map has a frame:** the latitude and longitude of the centre of hex (0,0), the factor for longitude (a literal, the cosine of the centre's latitude to six places), and its extent in latitude and longitude. The frame is written in the map file's header, as the present map's is today in words, in lines a tool can read.
2. **A frame is a small file of its own** beside the map, which the map tool reads to make the map and copies into the header: `data/map/<name>.frame.toml`. It also names the map's anchors file. The present map is given the frame it has always had.
3. **Longitude runs on without a break.** A frame that crosses the date line gives its longitudes above 180 (163 to 194 for 163 E to 166 W). The source's shapes, the anchors and any position given to a tool are brought into the frame's run before they are used. A longitude shown to a person or handed to an agent is always given east or west, 0 to 180.
4. **Every tool that needs a map's place on the earth reads that map's frame:** the map tool's `locate`, `water` and `render`; the check of passages; the viewer's labels; the table's chart. None keeps a constant of its own.
5. **The present map is made again by the changed tool and is the same, byte for byte.** That is checked once, by hand, by the build, since the map's source is not in the repository; the test that the committed map loads and has its counts stays.

### The Midway map

6. **Its frame:** centre 28 30 N, 178 30 E; factor 0.878817; extent 19 N to 38 N and 163 E to 166 W; hexes of 40 nm as everywhere. About 1,350 hexes, nearly all deep sea.
7. **Its error is larger than the slice's and is accepted.** A flat map with one factor for longitude is true along 28 30 N. East and west distances read 8 in 100 long at 19 N and 10 in 100 short at 38 N. At Midway (28 12 N) they are true to 3 in 1,000. Decision 0017's 5 in 100 was for the slice; for a map of open sea, on which the battle was fought between 26 N and 37 N, this is recorded as a known limit and not mended.
8. **Its places:** Midway, Kure, Pearl and Hermes Reef, Lisianski, French Frigate Shoals and Wake, each from a row of `data/oob/place_positions.csv`, in an anchors file of its own.

### A small island is land enough

9. **A named place makes its hex coast.** Where a map's anchors file marks a place as land, and the sampling finds no land point in the hex its position falls in, that hex is coast with a land percentage of 0, as a hex with one land point is today. Its body of water is the hex's whole sea. The header lists each hex made coast this way.
10. **The present map marks none,** so it does not change: Tikopia and the others stay as they are until a scenario needs them and a record says so.
11. **For Midway's map** the marks are Midway and Wake, the two places that hold a base. The others are named on the chart and stay sea.

### The table, the viewer and the tools

12. **The table's list names each scenario's map,** and the bridge serves the chart of the game in hand. A scenario with no map named is on the present one.
13. **The chart is drawn in the map's frame:** longitude scaled by the frame's factor, so that hexes are not stretched; meridians lettered east or west; the coast and names of that map, made by the clipping script with the frame given to it.
14. **The viewer** takes its factor for labels from the replay's own map.
15. **An agent** is told positions east or west, and the tool server's opening words name no sea: they say "a hex map" and the briefing says which.

### What does not change

16. **No rule of the simulation.** Darkness stays by the clock: at Midway in June the game's night is longer than the true one (decision 0198, 43). No scenario on the present map plays differently, and no hash moves.

## Rule card

No rule of the simulation is added or changed. New: a frame file for each map and its lines in the header; a mark in an anchors file; a map named for each scenario in the table's list; a second map, its anchors, its coast and its names.

## What it costs

- **The map tool is rewritten around a frame,** and its tests with it. The proof that nothing changed for the present map is a check made once by hand.
- **The table's chart is redrawn** for a factor it did not have. The present map's chart changes by 2 in 100 in its east and west scale, since it too is drawn today with no factor; that is a change a person can see and is said here.
- **The source of the land** is the public data the present map was made from, which is kept outside the repository; the build needs it on the machine it runs on, with the file hashes the present header records.
- **Rulebook and documents:** decision 0017 gains a line pointing here; `docs/commands.md` for the map tool; the viewer's and the table's notes; chapter 1 where it says what a map is.

## Questions for the designer

1. **The present map's chart will be drawn a little narrower,** 2 in 100, when it is drawn true. The architect proposes to do it, so that one way of drawing serves every map.

## Amended on review

A reviewer who had not seen the plan written read it against the code on Nov 20, 2026. What follows replaces the rules it names; where it and the text above differ, this holds.

**The present map's file does not change by one byte (replaces rules 1, 2 and 5).** A map file's bytes are hashed into every saved game, and five recorded games, a batch and the validation reports carry the present map's hash. So:

17. **The header's lines as they are today are the frame.** The lines "Projection: hex (0,0) is centred at …", "east_nm = …", "north_nm = …" and "Extent: every hex whose centre lies from … to … and from … to …" are written for every generated map in exactly the present form, with that map's numbers (degrees as whole numbers or decimals, "28.5 N, 178.5 E"). No line is added to the header for the frame.
18. **The header is the authority for every reader but the tool that makes the map.** `sim-data` reads those lines into fixed-point values and hands a map's frame to whoever asks: the check of passages, the export, the agent's places. The table's bridge reads the same lines of the map file, which it may read. A map with no such lines has no frame, and everything behaves for it as it does today.
19. **The rule is for generated maps.** The 45 placeholder maps are on no sea and have no frame.
20. **The map tool takes a frame file as an argument, and with none it makes the present map.** The present map's frame stays in the tool as its default, named as that, with its title and its date; its command line and so its header are unchanged. This is the one place a map's frame is in code. A frame file gives the title, the date, the centre, the factor, the extent and the land marks, every quantity as a string. It is kept in `data/map/frames/`, where nothing that walks `data/map/` for maps will take it for one, and only the map tool reads it.
21. **What proves it:** the build makes the present map again with the changed tool and compares it with the committed file, byte for byte, and says so in "As built". The test of the committed map is widened to every committed generated map: its rows are those of its header's extent, and each centre is where its header's projection puts it. For the Midway map, a test that the frame file and the header agree.

**Longitude in the file (replaces rule 3).**

22. **The map file's longitude column stays as it is documented: west negative, between -180 and 180.** A hex of the Midway map at 177 24 W reads -177.4. An agent is given that number, as today.
23. **Only the map tool, in making a map, and a drawing of a map run longitude on.** The tool shifts the source's shapes whole, ring by ring, into the frame's run before sampling. The table's chart, its position readout and names, the viewer's labels and the script that clips the coast add 360 to any longitude west of the frame's western edge, and letter what they show east or west.

**The frame of the Midway map (replaces rules 6 and 7).**

24. **South edge 18 N,** not 19 N, so that Wake (19 17 N) is not in the last row, where a small change to the frame would drop it. Extent 18 N to 38 N, 163 E to 166 W; about 1,400 hexes. Decision 0198's rule on the south edge is amended by this line.
25. **The error, the right way round.** East and west distances on the map are 0.878817 over the cosine of the latitude times the true ones: 8 in 100 short at 18 N, 11 in 100 long at 38 N, 3 in 1,000 short at Midway. Accepted as rule 7 says.
26. **Pearl Harbor is off the map,** and with it every Allied base but Midway; a force going east stops at 166 W. Wake's southern sides are the edge. Scenario 114's hand calculation says what that means for each force.

**Land marks (replaces rules 8 and 9's mechanism).**

27. **A land mark is a line of the frame file:** a name and a position, citing its row of `data/oob/place_positions.csv`. The anchors file and its seven columns are not changed, and the Midway map needs no anchors file: a hex with no land point has one body of water.
28. **A marked hex is coast with land percentage 0** and is listed in the header in a line of its own, "Marked land: …"; it is not counted among the hexes with a land point and 0 percent.
29. **Names that are only names** (Kure, Pearl and Hermes Reef, Lisianski, French Frigate Shoals) go in the table's names file and the export's labels, not in any file the map is made from.

**What else changes (added to rules 12 to 15 and "What it costs").**

30. **The export's map gains the frame** (centre, factor, western and eastern edge), and the viewer and its check read it there instead of the cosine of 12 degrees.
31. **Named for the build,** beyond what the plan listed: the map tool's search box and padding for the extent; `water`, which is given a map to read its frame from; the three tests that call the tool's projection; the check of passages' constants; the table bridge's default map and its chart loaded once at start; the clipping script's fixed box; the tool server's opening words.
32. **Decision 0185 is amended** where it has the table's list name a scenario without its map (rule 12 here).
