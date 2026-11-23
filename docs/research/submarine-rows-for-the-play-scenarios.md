# Submarine rows for the play scenarios

Read on 2026-11-23 for the architect, for decision 0225 (rule 6 and rule 13 of its amendment): the ship rows, class facts, stations by date, reliefs and beliefs that putting boats into the Coral Sea, Midway and Guadalcanal games needs. It places no boat and decides nothing. Rows are cited as in `submarines-in-the-play-scenarios.md`: `sh` `ship_facts.csv`, `cl` `ship_class_facts.csv`, `pos` `engagement_positions.csv`, `ev` `engagement_events.csv`, each with its id. Every new row has an id of 17200 or more.

## What was added

- **`ships.csv`**: 29 new boats (17200 to 17244, with gaps where the boat already had an identity row). Sixteen American boats of Midway already had identity rows (6318 to 6337) with no class; they keep them, as Nautilus and Tambor do, and gain fact rows only.
- **`ship_classes.csv`** 17200 to 17207, eight new classes of the type `submarine`: `jp-i-16-group` (the Technical Mission's block e: I-16, I-18, I-20, I-22, I-24, I-46 to I-48), `jp-i-7-group` (block b: I-7 and I-8), `jp-i-1-j1-group` (I-1 to I-4) and `jp-i-6-j2-group` (I-6), the split of the I-1 block the architect ruled, `us-dolphin-class`, `us-cachalot-class`, `us-porpoise-class` (DANFS's name) and `us-perch-class` (FM 30-50's).
- **`ship_facts.csv`** 17200 to 17292: a `ship_class` row and, where read, a `commissioned` row for each boat.
- **`ship_class_facts.csv`** 17200 to 17292: speeds, range with its speed, tubes, torpedoes, torpedo model and reloads (the existing attribute `torpedo_reloads`).
- **`engagement_positions.csv`** 17200 to 17262 and **`engagement_events.csv`** 17200 to 17248: stations and positions by date, arrivals and departures, and three Japanese beliefs of Allied boats at Midway.
- **`gaps.csv`**: the last 9 entries.

The sources: the US Naval Technical Mission to Japan's S-01-1 and S-01-6 (read off the page images; the scans' OCR is garbled), ONI 222-J (June 1945, read off the page images), FM 30-50 (1941), DANFS (the hazegray.org copy and the NHHC pages, both through the Wayback Machine), Nishida's class pages, combinedfleet.com's records of movement, Bates's Naval War College analyses of the Coral Sea and Midway, Japanese Monograph 93, and Wikipedia as a last resort. Each row's note names the page and line.

## The derivation list must change before these rows can be read

`cargo run -p oob-derive -- data/derivation/submarines_1942.toml --check` and the test `the_submarine_list_derives_exactly_the_files_in_data_reference` now fail, and are left failing on purpose, since the brief forbids editing a derivation list. Three things cause it, each the architect's to settle:

1. **The eight new classes** are neither derived nor left out. Each needs a `[[submarine]]` or `[[left_out]]` entry.
2. **The estimates by ratio** of the S-boats' and Narwhal's passage speed read every submarine class with both a `max_speed_kn` and an `endurance_speed_kn` row as a like class (decision 0207, rule 6). The new classes have such rows, some undated, unconfirmed or dated 1945, so the like classes cannot be read on 7 August 1942. The list has to name exceptions for them, or the rule has to choose its like classes.
3. **The I-15 class's torpedo** is now filed (`cl` 17236, Type 89, and 17237, Type 95 "also reported", ONI 222-J's I-15 sheet, the line the list marked as owed). By rule 29 a filed row drives out the list's `torpedo_from` entry for the class.

## Class facts, by class

| Class (boats in these battles) | Built? | What is now on file | Still missing |
| --- | --- | --- | --- |
| `jp-i-15-group` (I-15, I-17, I-19, I-21, I-26 to I-29, I-31, I-33) | 6003 | Torpedo model (`cl` 17236, 17237); reloads 11 (17238, S-01-6) | Nothing for derivation |
| `jp-i-16-group`, new (I-22, I-24) | no | S-01-1 block e: 2,180 t, 24 / 8.5 knots, 8 bow tubes, 20 torpedoes (12 for reload), 14,000 miles at 16 knots, Type 95 (`cl` 17211 to 17219); S-01-6 reloads 12 (17221); Wikipedia's Type 95 (17220) | Nothing for derivation |
| `jp-i-9-group` (I-9, I-11) | no (left out for want of a speed) | S-01-1 block c "I-9 to I-12": 2,443 t, 24 / 8.8 knots, 16,000 miles at 16 knots (`cl` 17200 to 17205); ONI 222-J: designed 17.0, 16,000 miles at 16.0 (17206 to 17209); S-01-6 reloads 12 (17210). The endurance speed of 16 knots agrees in both and is preferred; the top speed is disputed (24 against 17) | Settling the top speed if the derivation needs it |
| `jp-i-7-group`, new (I-7; I-8) | no | S-01-1 block b: 1,995 t, 23 / 8 knots, 6 bow tubes, 20,000 miles at 10 knots (`cl` 17222 to 17228); S-01-6: no stern tubes, 12 reloads (17229, 17230); Wikipedia's Type 89 (17231) | Torpedoes carried as a total from a source (6 tubes plus 12 reloads is 18; Wikipedia 20) |
| `jp-i-5-group` (ONI's: I-5, I-6, I-8) | no | ONI 222-J: designed 17.0 knots, 12,700 miles at 14.0, 8.0 submerged (`cl` 17232 to 17235) | Whether the class exists: the Technical Mission puts I-5 and I-6 in its I-1 to I-6 block and I-7, I-8 in their own; ONI does not list I-7 |
| `jp-i-1-j1-group`, new (I-1 to I-4) and `jp-i-6-j2-group`, new (I-6) | no | Nishida's speeds (18/8; 20/7.5) and Wikipedia's tubes, torpedoes, range and model, restated for each (`cl` 17245 to 17260), all low and unconfirmed. The Technical Mission's S-01-6 row for the whole block (4 bow, 2 stern, 14 reloads, `cl` 17241) agrees with I-6's Wikipedia page and not with J1's (6 bow) | A primary source for either type |
| `jp-i-168-group` (I-168, I-174, I-175) | 6005 | S-01-6: 4 bow, 2 stern, 6 reloads (`cl` 17239). Tubes plus reloads is 12, against the 14 of Wikipedia's row 16222 that the list reads by exception | Torpedoes carried as a total and the model from a primary source (gap) |
| `jp-ro-33-group` (RO-33, RO-34) | 6007 | S-01-6: 4 bow, no stern, 6 reloads (`cl` 17240, 17244). 4 plus 6 is 10, as Wikipedia's rows 16226 and 16227 | The model from a primary source (gap) |
| `jp-i-176-group`, `jp-i-121-group`, `jp-i-1-group` | 6006, 6008, no | S-01-6 reloads (`cl` 17242, 17243, 17241); I-121's 4 reloads plus 4 tubes is 8, agreeing with S-01-1's 8 against ONI's 20 | |
| `us-dolphin-class`, new (Dolphin) | no | FM 30-50: 17 knots, 6 tubes (`cl` 17261, 17262); Wikipedia: 8 submerged, 4,900 miles at 10, 4 bow, 2 stern, 18 torpedoes (17263 to 17268) | Torpedo model; a source better than Wikipedia for the rest |
| `us-cachalot-class`, new (Cachalot, Cuttlefish) | no | FM 30-50: 17 knots, 6 tubes (17269, 17270); Wikipedia: 4 bow, 2 stern, 16 torpedoes, range at 10 knots disputed (6,000 on the boat pages, 11,000 on the class page) (17271 to 17277) | As Dolphin |
| `us-porpoise-class`, new (Pike, Tarpon; by DANFS also Pickerel, Plunger) | no | FM 30-50's Pike class: 20 knots, 6 tubes (17278, 17279); Wikipedia's Pike: 6,000 miles at 10, 4 bow, 2 stern, 16 torpedoes (17280 to 17284) | As Dolphin |
| `us-perch-class`, new (Plunger, Pickerel by FM 30-50) | no | FM 30-50: 6 tubes (17285); Wikipedia's Plunger: 19.25 / 8.75 knots, 11,000 miles at 10, 4 bow, 2 stern, 16 torpedoes (17286 to 17292) | As Dolphin; which class Plunger and Pickerel belong to (`sh` 17222, 17244 and 17224, 17245 are disputed) |

Two findings against rows on file: the Technical Mission's block f reads "I-15, I-17, I-19, I-23, I-25 to I-45" on the page image, so it does not include I-21, which the description of `ship_classes.csv` 16010 lists (I-21's class rests here on Nishida and ONI 222-J, `sh` 17257, 17258); and ONI 222-J's I-5 sheet lists I-5, I-6 and I-8, not "I-5 to I-8" as `ship_classes.csv` 16008 says.

## The Coral Sea (1 to 11 May 1942)

**Japanese.** Ship rows now exist for all six of Bates's submarine force and for I-21 and I-27.

| Boat | Class row | Where, by date | Placeable? |
| --- | --- | --- | --- |
| I-21 | `jp-i-15-group` (`sh` 17257, 17258) | Off Noumea: 5 May, John Adams sunk at 23-11S 165-08E (`pos` 17201, the target's position); 0630 on 7 May, 20 miles W of Noumea (17202); 7 May, Chloe sunk at 22-59S 166-29E (17203); 8 May in the same area (17204); left on 14 May | Yes: class built, positions dated, on the map |
| I-28, I-29 | `jp-i-15-group` (`sh` 17263, 17265) | Patrol line SW of Guadalcanal by 5 May (`pos` 17200), bearing only | Class built; a station by bearing with no distance; a patrol order of those waters |
| I-22, I-24 | `jp-i-16-group` (`sh` 17269, 17271) | The same line (`pos` 17200) | When the class is derived, as I-28 |
| RO-33, RO-34 | `jp-ro-33-group` (`sh` 17277, 17279) | Off Port Moresby from 5 May (`pos` 17205, 17206); RO-33 left on 10 May | Class built; a named place, no bearing or distance |
| I-27 | `jp-i-15-group` (`sh` 17261) | Off Brisbane from 3 May (`pos` 17207) | Off the map |
| Unidentified | | Submarines seen or attacked by Army aircraft on 11 May at 14-52S 152-20E, 20-20S 155-38E and 19-43S 157-00E (`pos` 17209 to 17211, Bates) | No boat to tie them to |

**American.** S-37 has no Coral Sea role; S-39, S-41, S-42, S-43 now have ship rows (`sh` 17202, 17204, 17206, 17208), classes 6009 and 6010. No source read gives any S-boat a latitude and longitude in May 1942: only named waters, already on file (ev 16103 to 16116, 16161 to 16167). Each can be given a patrol order of those waters (decision 0225, rule 13).

## Midway (28 May to 7 June 1942)

**American.** Every boat on the arcs now has a class row: Grayling, Grenadier, Gudgeon (`us-tambor-class`, 6001), Grouper, Flying Fish, Trigger (`us-gato-class`, 6002), Narwhal (6011), and Dolphin, Cachalot, Cuttlefish, Plunger (new classes, not built). Pike, Tarpon, Growler of the Oahu patrol (off the map) are classed too.

New positions after the arcs of 2 June, all from Bates:

| Date | Boats | Where | Rows |
| --- | --- | --- | --- |
| 3 June, 2400 | Cuttlefish | 263 degrees, 690 miles from Midway | `pos` 17212 |
| 4 June, 0020 | Cuttlefish | 27N 170-02E (her report) | 17213 |
| Dawn 5 June | the twelve of TG 7.1 | the 100-mile circle, 10-degree sectors from 230 to 350 degrees (which boat had which is not given) | 17214 |
| 5 June, 0923 | Plunger, Trigger (Narwhal did not go) | points on the 100-mile circle north of Midway | 17215 |
| 6 June, 0000 | Narwhal, Plunger, Trigger, Dolphin, Gato, Grenadier, Nautilus, Cachalot; Gudgeon | the 12-mile circle | 17216, 17220 |
| 6 June, 0000 | Tambor, Trout, Grayling; Flying Fish; Grouper | searching 264 degrees, 125 miles; 310 degrees, 200 miles; the 5-mile circle | 17217, 17218, 17219 |
| 6 June, 1640 | Grayling | 27-40N 174-30E, bombed by B-17s | 17221 |
| 6 June, 2015 | Dolphin, Nautilus, Cachalot, Trigger, Plunger | the 25-mile circle, 200 to 350 degrees | 17222 |
| 7 June, 0000 | Trout (Tambor 30 miles north, Grayling 30 south) | an ordered track through 29N 168E | 17223 |
| 7 June, 1300 | Cuttlefish | 420 miles from Midway, bombed | 17224 |

Which boat stood where on each arc is still not given (gap): rule 2 of the amended decision spreads them. Grayling, Grenadier, Gudgeon, Grouper and Flying Fish can now be placed by that rule; Dolphin and Cachalot when their classes are derived.

**Japanese.** Nothing new is placeable: the cordons are east of the map. Japanese beliefs of Allied boats at Midway: the report of 2 June of at least 16 sighted since 29 May, six off the homeland, four north-east of Wake, four around Truk (`ev` 17246); two sighted 48 degrees, 540 miles from Wake on 31 May (17248); eight sighted near Wake on 8 June (17247). None is of the Midway map's waters except I-168's 14, already on file.

## Guadalcanal (7 August to 14 September 1942, and to 31 October for I-19 and I-26)

**Japanese.** Positions by date (`pos`), with the class:

| Boat | Class | Where and when | Rows |
| --- | --- | --- | --- |
| I-121 | 6008 | 8 Aug, 30 miles S of Cape St George; 15 to 17 Aug, Lunga anchorage; from 18 Aug, SE of San Cristobal; 22 Aug 1710, 50 miles SE of San Cristobal (attack); 27 Aug 0630, 175 miles SW of San Cristobal (bombed); 29 Aug 0430, 80 miles NE of San Cristobal; back at Rabaul 4 Sep | 17225, 17230, 17231, 17233, 17244, 17246; `ev` 17212 |
| I-122 | 6008 | 13 Aug 1730, 1.6 miles SW of Tulagi; Indispensable Reef 14 to 20 Sep | 17226; `ev` 17213, 17214 |
| I-123 | 6008 | 12 Aug, surfaced N of Lunga Point (sunk 29 Aug, on file) | 17229 |
| RO-34 | 6007 | 10 Aug off Tulagi; 12 Aug Taivu Point; 23 Aug 1827 Lunga Roads (attack); 28 Aug 0134 off Cape Esperance; Rabaul 16 to 21 Aug and from 6 Sep | 17227, 17228, 17261, 17262; `ev` 17200 to 17202 |
| RO-33 | 6007 | 25 Aug S of Samarai; sunk 29 Aug at 09-36S 147-06E | 17259, 17260 |
| I-15, I-17, I-19 | 6003 | 23 Aug, patrol line E of the Santa Cruz Islands; I-19 at 07-30S 162-15E (0725) and I-17 at 06-48S 163-20E (0745), both on 23 Aug; I-19 200 miles SE of Guadalcanal 26 Aug; I-15 E of San Cristobal 28 Aug; all three left the area 20 Sep, at Truk 25 Sep | 17234, 17239, 17240, 17243, 17245; `ev` 17219 to 17225 |
| I-26 | 6003 | 20 Aug, 250 miles SE of Guadalcanal; 23 Aug, picket line SE of the Solomons; 31 Aug, her area 140 miles E of San Cristobal (Saratoga at 10-34S 164-18E, on file); Truk 25 Sep; 11 Oct SW of Guadalcanal; Indispensable Reef 18 to 26 Oct | 17232, 17236, 17247, 17256, 17257; `ev` 17226 to 17228 |
| I-19 (to 31 Oct) | 6003 | 15 Sep, Wasp force heard at 12-18S 164-15E; Noumea area from 19 Oct | 17253, 17258 |
| I-33 | 6003 | Picket line S of San Cristobal; 24 Aug at 09-21S 163-35E; left 20 Sep | 17237, 17241; `ev` 17229, 17230 |
| I-31 | 6003 | 25 Aug 0543 (zone -9), 175 miles SE of San Cristobal; Truk 30 Aug to 6 Sep; back SE of Guadalcanal 14 Sep | 17242; `ev` 17208 to 17210 |
| I-21, I-24 | 6003; `jp-i-16-group` | Left Yokosuka 23 Aug and 30 Aug; with I-15, I-9, I-17, I-19, I-26 and I-33 between Ndeni and San Cristobal from 10 Sep | 17251; `ev` 17231, 17233 |
| I-174, I-175 | 6005 | I-174 to SW of Rennell from 23 Aug (to Truk 22 Sep); I-175 from Rabaul 22 Aug to the area SE of Guadalcanal, at Lunga Point 11 Sep (to Truk 21 Sep) | 17238, 17252; `ev` 17215 to 17218 |
| I-9 | `jp-i-9-group` | 23 Aug patrol line A off San Cristobal; Truk 30 Aug to 8 Sep; 23 Sep 200 miles SE of Guadalcanal; left 1 Oct | 17235, 17254; `ev` 17205 to 17207 |
| I-11 | `jp-i-9-group` | 31 Aug 0405, 146 miles SE of Tulagi (attack); 6 Sep 13-20S 162-40E (Hornet); 7 Sep 07-12S 163-14E; Truk 11 Sep | 17248 to 17250; `ev` 17211 |
| I-7, I-5, I-4, I-176, I-22 | new or disputed classes; I-176 6006 | Left Truk 17 to 19 Sep (I-176 18 Sep) for the lines SE and S of San Cristobal; I-5 20 miles E of Guadalcanal 25 Sep; I-22 left Yokosuka 11 Sep | 17255; `ev` 17234 to 17238 |

Placeable today, class built and a dated position on the map: I-121, I-122, I-123, RO-34, I-15, I-17, I-19, I-26, I-31, I-33, I-21 (from 10 Sep, a line only), I-174 and I-175 (areas by bearing), and RO-33 (off the map's west edge on 29 Aug; S of Samarai on 25 Aug). Most stations are a compass bearing and a distance from a named island, read as the bearing of the compass point (each note says so); a line's spacing and extent is never given (gap).

**American.** S-37, S-39, S-41, S-42, S-43 and S-46 now have ship rows (classes 6009, 6010) and dated patrols (`ev` 17239 to 17244), but no position: S-37 off Savo 17 Aug to 13 Sep; S-39 from 10 Aug, lost off Rossel Island 13 to 14 Aug; S-41 New Britain to New Ireland from 7 Aug; S-42 on station between Savo and Cape Esperance from about 26 Aug to September; S-43 from 14 Sep east of Kiriwina, then Buka; S-46 in the Solomons 20 Jul to 15 Aug. Amberjack (`us-gato-class`, `sh` 17248) left Pearl Harbor on 3 Sep for the waters between New Ireland and Bougainville (`ev` 17245). Greenling, Drum and Flying Fish (classed) patrolled off Truk, north of the map. Each can be given a patrol order of its named waters.

**Reliefs.** The arrivals and departures above, with those on file (ev 16265 to 16295), give the Japanese boats' presence by date: the first wave (I-121, I-122, I-123, RO-33, RO-34) from 8 to 16 Aug; SubRon 1 and I-31, I-33 from 23 Aug; I-9 and I-31 away 25 Aug to 8 and 14 Sep; I-11 away from 8 Sep; I-174 and I-175 to Truk 21 and 22 Sep; SubRon 1 and I-33 left 20 Sep; the second wave (I-1 to I-7, I-176) left Truk from 17 Sep.

**Beliefs.** No Japanese estimate of Allied boats in the Solomons with a place and a number was found for August to October (gap). Japanese Monograph 98 names only the Meiyo Maru sinking south of Cape St George on 8 August and the Makin raid by two submarines on 17 August.

## What is still missing

The new entries of `gaps.csv`: the S-boats' positions on patrol (the war patrol reports are not online); the Japanese boats' stations on the Coral Sea line and on the Solomons lines by latitude and longitude, with each line's spacing and extent; which boat held which sector at Midway; the I-168 and RO-33 classes' totals and model from a primary source; the torpedo model of the four new American classes and their other figures from a source better than Wikipedia; the completion dates of I-31, I-33, I-11 and I-4; I-19's firing position at Wasp; any Japanese estimate of Allied boats at Guadalcanal.

What could not be checked: S-01-6's table is read from a page image whose row labels overlap (the I-15 and I-16 rows); the reloads are the column the Technical Mission calls "Reload Torpedoes", and whether its figures held in 1942 is not stated. ONI 222-J's I-9 sheet gives a designed speed of 17.0 knots, the same figure as its I-5 sheet beside it, against the Technical Mission's 24; it is filed and disputed, not explained.
