# The ships of the 39 days by name, 7 August to 14 September 1942

Status: research report for the architect, read on 2026-11-19. Nothing here is a ruling. It is for the ruling that will replace the placeholder ships of `data/scenarios/07_full_slice.toml` with real ones; no scenario, derivation list, reference file or code was changed.

## What was asked

In the full slice 93 of 133 ships are placeholders. The facts on file said what was carried and when, but not which ship carried it. Three questions, in order: (1) the Allied transports and cargo ships of 7 August and the later Allied arrivals inside the 39 days, by name, with their transport divisions, the units in each, their movements and losses; (2) the Japanese runs of 16 August to 14 September, ship by ship, with dates, places, loads and losses; (3) what each newly found ship could carry.

## What was added

| File | Rows | Ids | What |
| --- | --- | --- | --- |
| `ship_facts.csv` | 553 | 1900-2171, 2200-2389, 2480-2520, 2600-2649 | Formation, units embarked, movements, damage and fate of the ships of the landings and the runs; `troop_run` ties; capacities of the new ships |
| `troop_lift_facts.csv` | 82 | 600-609, 650-683, 700-737 | Run statements: ships in a run, men, tons, dates |
| `lift_entities.csv` | 15 | 1400, 1410-1419, 1420-1423 | New runs (below) |
| `ships.csv` | 15 | 1170-1180, 1185-1188 | Manley (APD-1), Stringham (APD-6), McFarland (AVD-14), William Ward Burrows (AP-6), Kopara (New Zealand), Tracy (DM-19), Gamble (DM-15), YP-239, YP-284, YP-326, YP-346; subchaser CH-24, Tsugaru, Sado Maru, Asaka Maru. No class is set: no class key fits a source's statement except where a `ship_class` row is filed |
| `ship_class_facts.csv` | 4 | 2100-2103 | Wickes-class destroyer-transport, from NavSource's APD pages |
| `passages.csv` | 3 | 430-432 | The Ichiki first echelon's departure and landing, the Tanaka convoy's return |
| `attributes.csv` | 4 | | New ship attributes, all text and many: `units_embarked_note`, `formation_note`, `movement_note`, `damage_note` |
| `gaps.csv` | 58 | | What was looked for and not found |

New runs: `us-run-1942-08-07-watchtower-landing`; `us-run-1942-08-16-manley-stringham`; `us-run-1942-08-20-burrows-mag23-echelon`; `us-run-1942-08-21-2-5-crossing`; `us-convoy-1942-08-28-burrows-kopara`; `us-run-1942-08-31-betelgeuse-6th-ncb`; `us-run-1942-08-31-yp-boats-with-helm`; `us-run-1942-08-31-raider-parachute-crossings`; `us-run-1942-09-03-fomalhaut-and-apds`; `us-run-1942-09-04-savo-raid`; `us-run-1942-09-14-bellatrix-fuller`; `jp-run-1942-08-24-uzuki-kokumbona`; `jp-run-1942-09-01-tsugaru-urakaze-tanikaze-shikinami-uranami`; `jp-run-1942-09-01-barge-convoy-gizo`; `jp-run-1942-09-04-santa-isabel-barges`.

## How it was read and checked

Four Sonnet researchers, one per topic and a fourth on capacities, each with its own id range; the lead merged their rows and checked them. Pages were read as HTML or text (HyperWar transcriptions of Zimmerman, Hough, Miller, Dyer, Shaw, the ONI Combat Narratives "The Landing in the Solomons" and "Miscellaneous Actions in the South Pacific", DANFS at HyperWar, the hazegray copy and NHHC pages through Wayback captures; the 1st Marine Division Final Report Phases I and III to V and Bates's Savo analysis as archive.org OCR text; combinedfleet.com tabular records, uboat.net, NavSource through Wayback, shipscribe, Nishida, navypedia, Japanese Monograph 45, MacArthur's Reports vol. II, the USSBS interrogation of Toyama). No PDF or image was downloaded and nothing was taken from `references/` or from any game.

Every quotation (812 rows) was checked by script against the saved page text, whitespace normalised: none failed. The lead re-fetched four pages and found four sampled quotes on them. What the check does not show is that a statement was filed against the right ship and date; OCR quotes (the Final Report, Bates) keep their misreadings, and the notes say so.

The researchers filed most single-source rows `unconfirmed`. `oob-check` holds a single row at medium confidence to `preferred`, and the lead set 252 such rows to `preferred` by that rule alone (mostly combinedfleet.com and DANFS movement notes); rows whose sources disagree are `disputed`. Two combinedfleet pages are one source, and the rows say so where it matters.

## Topic 1: the Allied ships

**7 August.** All 23 ships of Task Force 62's transport groups are named, and each is placed in a division by two sources that group them differently: the ONI Combat Narrative's task organization (X-Ray: Transdivs Afirm, Baker, Cast, Dog; Yoke: Easy and Twelve; Zimmerman and Hough give the same lists) and Dyer's standing divisions (Two, Eight, Ten, Twelve; Alchiba not in his list). Units embarked are found for every ship but Fomalhaut, from the Combat Narrative's "command afloat" lines, the Final Report's Annex K station list and Annex D (OCR), DANFS and Zimmerman. Dyer gives the 7 August anchoring times from the ships' logs for eight of them. The 9 August departures are by group only in Bates, from poor OCR, and disagree with Miller's ten at 1500 and the rest at 1830. George F. Elliott's end is disputed (sunk 8 August by DANFS; still afloat the next evening by the Combat Narrative; beached and destroyed by her sisters by Zimmerman and Hough).

**The scenario's grouping is not the record's.** The 2d Marines (less the parts that landed on the Tulagi side) were in President Adams, President Hayes and Crescent City of Group X-Ray, and Zeilin, which the Combat Narrative puts in Yoke, carried the 3d Defense Battalion. The scenario's 21 and 12 ships of 600 men are a size, not a list; the record has 15 and 8.

**After the landings.** Named: Manley and Stringham (16 August); McFarland, Colhoun, Gregory, Little, McKean, Stringham and Manley (21 August, uboat.net; Miller names Colhoun, Gregory, Little and McFarland); Alhena and Fomalhaut with Blue, Helm and Henley (21 to 23 August); William Ward Burrows and Kopara with Gamble, Tracy, Colhoun, Gregory and Little (26 to 31 August); Betelgeuse with Henley and the 6th Naval Construction Battalion (31 August to 2 September); Helm with YP-239, 284, 326 (and YP-346 by DANFS Burrows); Little and Gregory to Savo (4 September, both sunk on the 5th); Manley and McKean to Tasimboko (7 to 8 September); Fuller and Bellatrix (7 to 8 and 14 September). Not named: the four APDs of 15 August (Zimmerman: "the four destroyer transports of Transdiv 12"; the division's members are named elsewhere, and no source joins the two), the APDs of the 2/5 crossing of 21 August, the APDs of the raiders' crossing and the YPs of the parachutists' crossing, and the six transports of the 7th Marines.

## Topic 2: the Japanese runs

Every run but the Oka group's barges now has named ships, but most of the names rest on combinedfleet.com alone. An independent second source names: Oite (Dyer); the six destroyers of 18 August (Zimmerman, who misprints two names); Fubuki, Shirayuki, Kawakaze and Umikaze on 29 August (Zimmerman); Amagiri, Kagero and Suzukaze on 31 August (Zimmerman), with "all the above craft" read as the four of the 29th (inferred, low); Yudachi, Hatsuyuki and Murakumo in the Slot on the night of 4 to 5 September (DANFS Gregory). The Tanaka convoy of 19 to 25 August, which had no ships on file, is now named ship by ship from combinedfleet, Zimmerman, Hough and the Toyama interrogation, and disagrees on who carried the Ichiki second echelon (the four patrol boats by Zimmerman; Boston Maru and Daifuku Maru with the patrol boats as escort by combinedfleet).

Two findings bear on rows already on file:

- **6 September.** The rows already on file (troop_lift 166, ship_facts 1345 to 1348) have Uranami, Shikinami, Yudachi and Ariake landing 370 men. The archive copies of the same four combinedfleet pages, and the live Ariake page, say an attack mission that shelled the airfield instead (2374 to 2377). The live pages and their archive copies differ; this one needs a reading by the architect.
- **29 August.** Four ships and 900 men (Zimmerman), two destroyer transports (Hough), five ships and 750 men (the Tsugaru page), and the 11th and 24th Destroyer Divisions with about 450 men (the JACAR report on file, passages 1), against Saito's 450 and 300. Not reconciled.

New runs found: Uzuki's run to Kokumbona of 24 to 25 August; a troop and artillery run of 1 to 2 September (Urakaze, Tanikaze, Shikinami, Uranami, covered by Fubuki, Shirayuki and Amagiri, with two patrol boats); a barge convoy (Sado Maru, Asaka Maru) to Gizo escorted by Yudachi and Murakumo; and barges from Santa Isabel escorted by Kawakaze, Umikaze and Suzukaze on 4 to 5 September, which no page links to the Oka group's 61 barges.

**Asaka Maru.** The ship of the Gizo barge convoy is named "Asaka Maru" on the Tsugaru page, and is filed under that name. combinedfleet's own Asaka Maru is an auxiliary cruiser at Yokosuka on those dates, and the patrol boats' pages name an Asakasan Maru. The key holds the source's name; which ship it was is open.

## Topic 3: what the ships could carry

Found (ship_facts 2600 to 2649 unless stated): Manley 147 troops (NavSource, 3 officers and 144 men summed); Stringham 147; the Wickes-class APD template of NavSource (147 troops, 1,154 light, 1,600 full, 35.2 knots; ship_class_facts 2100 to 2103, the troop figure disputed against the 144 and 200 on file); William Ward Burrows 1,138 troops (two sources), 1,800 deadweight, 13 knots service; Kopara 679 gross, 950 deadweight, 12 knots; YP-346 259 gross; patrol boats No. 1 and 2 up to 250 troops, No. 34 and 35 up to 150 (combinedfleet, after the 1941 reconstruction); Kinryu Maru 9,310 tons of unstated kind and 800 or about 1,000 troops; Boston Maru and Daifuku Maru tonnage of unstated kind; Tsugaru 4,000 tons standard and a 519-man troop mission of 7 August. Gamble, Tracy and McFarland have speeds and displacements only.

Not found, as gaps: troop or cargo figures for McFarland, Tracy, Gamble, CH-24, Sado Maru, the YPs other than one tonnage; fuel and endurance of any of them in a form the attributes take (NavSource gives fuel in barrels and the Clemson design radius; navypedia gives Tsugaru 9,000 nm at 10 knots: quoted in the gap rows for the architect, as there is no ship attribute for endurance); any per-destroyer load for August and September beyond what was on file (drum counts begin in November).

## Disputes for the architect

1. President Hayes and President Adams: which battalions of the 2d Marines (Zimmerman 2d and 3d; Hough 1st and 3d), rows 1988, 1989, 1996 to 1998.
2. George F. Elliott's date and manner of sinking, rows 1932 to 1935, 1940.
3. The 9 August departure groups: Miller (troop_lift 396) against Bates (605, 606, ship_facts 2480 onwards).
4. Libra and Zeilin's anchorage on 7 August (DANFS against the task organization; notes only).
5. The MAG-23 forward echelon: Burrows 20 August (Hough), Alhena 30 August (Zimmerman), Burrows 29 August (the division's calendar, DANFS Burrows).
6. Burrows and Kopara's arrival hour on 29 August; Betelgeuse's dates and her Seabees (200, 357, or 5 officers and 387 men); Fomalhaut on 22 or 23 August; McFarland's whereabouts on 26 to 27 August.
7. The raiders' Savo patrol, 2 or 4 September; the raiders' and parachutists' crossing, 31 August or 28 August to 3 September; Bellatrix or YPs for 3/2 Marines on 14 September; Colhoun's dead.
8. The Yokosuka 5th SNLF's landing: 16, 17 or 18 August; Tassafaronga, Kokumbona or Lunga Point; 200 or 500 men.
9. The Ichiki first echelon: 900, 916, 917 or about 1,000 men; landing 2030 on 18 August or 0100 on 19 August.
10. The Tanaka convoy: who carried the Ichiki second echelon; Kinryu Maru's troops (800 or 1,000) and sailing (16 or 17 August); start at Truk or Rabaul; return to Shortland or Rabaul.
11. 29 August and 6 September, above; the second section on 4 to 5 September; 5 September's 231 or 370 men.
12. Speeds of Gamble, Tracy and Burrows; the APD troop figure (144, 147, 200).

## What is still a gap

The four APDs of 15 August by name; the ships of the 2/5 crossing and of the raiders' and parachutists' crossings; the escorts of Fuller and Bellatrix; the six transports of the 7th Marines; cargo tons by ship for every run (the Final Report's Phase IV Appendix 3, the table of ship arrivals and cargo, is cited in the report but is not in the text read); Fomalhaut's units on 7 August; the share of cargo landed by ship; a second source outside combinedfleet for the Japanese runs of 27 to 28 August, 30 August, 1 to 2, 2, 5, 7 and 11 September and for Hagikaze's bombing; the Oka group's ships. Not read: Turner's and the transports' action reports, the Gray Book (PDF), Japanese Monograph 98, the Saito papers (PDF), JACAR and Senshi Sosho, the New Zealand official history for Kopara, history.navy.mil directly.

## The table for the ruling

Ship rows are in `ship_facts.csv` unless marked T (`troop_lift_facts.csv`). "Not found" means the record read does not say. Where sources disagree both are given.

### Allied forces

| Task force (scenario) | Placeholders | Ships the rows support, with row ids | Units carried, with row ids | Not found |
| --- | --- | --- | --- | --- |
| 20, Placeholder transports, Guadalcanal | 21 troop transports (ids 52-72) | Group X-Ray (ONI) by Transdiv: Afirm: Fuller 1945, American Legion 1968, Bellatrix 2005; Baker: McCawley 1908, Barnett 1914, George F. Elliott 1937, Libra 2029; Cast: Hunter Liggett 1958, Alchiba 2012, Fomalhaut 2010, Betelgeuse 2025; Dog: President Adams 1982, President Hayes 1992, Crescent City 2000, Alhena 2018. Dyer's divisions: 1906, 1913, 1936, 1944, 1957, 1967, 1981, 1991, 1999, 2004, 2009, 2017, 2024, 2028. Run ties 2480-2502 | McCawley 1911, 1912; Barnett 1916, 1917; Elliott 1941-1943; Fuller 1947, 1948; Liggett 1962-1965; American Legion 1972-1974 (Combat Group A command); Adams 1985-1990 and Hayes 1994-1998 (2d Marines battalions, disputed); Crescent City 2002, 2003; Bellatrix 2007, 2008; Alchiba 2015, 2016; Alhena 2022, 2023; Betelgeuse 2027; Libra 2033-2035; men aboard on 2 Aug: 1128-1149 (on file); 2d Marines left aboard: T 607, 608 | Fomalhaut's units; share of cargo by ship; Elliott's end disputed (1932-1935, 1940) |
| 21, Placeholder transports, Tulagi | 12 troop transports (ids 73-84) | Group Yoke (ONI), Transdiv Easy: Neville 1950, Zeilin 1901, Heywood 1919, President Jackson 1976; Transdiv Twelve: Colhoun 2037, Gregory 2041, Little 2044, McKean 2048 (Dyer 1949, 1900, 1918, 1975, 2036, 2040, 2043, 2047) | Neville 1953-1956; Zeilin 1903-1905 (3d Defense Battalion; DANFS puts her off Guadalcanal); Heywood 1923-1928 (Parachute Bn, Special Weapons Bn); President Jackson 1978-1980 (1st Bn 2d Marines); APDs: Colhoun 2038, 2039, Gregory 2042, Little 2045, 2046, McKean 2049 (1st Raider Bn) | 3d Defense Battalion men by ship; APD clock times |
| 22, run of 15 August | 4 fast transports with cargo (85-88) | Not found by name. "The four destroyer transports of Transdiv 12" (T 650); the division is Colhoun, Little, Gregory, McKean (2050, 2053-2056) | CUB-1, 5 officers and 118 men, 120 t (T 86-89 on file; T 651) | which four ships on the day |
| 23, cargo ships, run of 21 August | 2 cargo ships (89, 90) | Alhena 2084, 2090; Fomalhaut 2083, 2092 (22 or 23 Aug), escorted by Blue 2085, Helm 2086, Henley 2087; with the APDs McFarland 2062, Colhoun 2063, Gregory 2064, Little 2065, McKean 2066, Stringham 2067, Manley 2068 (uboat.net); Miller names Colhoun, Gregory, Little, McFarland (T 656) | Alhena and Fomalhaut cargo 2088, 2089; rations 40 t each in three APDs 2077-2079; aviation gasoline 2069; over 2,000 t with 200 t rations (T 60 on file) | hours off the beach; tons by ship |
| 24, fast transports, the Tulagi crossings | 6 fast transports (91-96) | 2/5 on 21 Aug: not found by name ("the destroyer transports of TransDiv 12", T 653; "on APDs", T 654). Raiders: Kopara 2147 and unnamed APDs (journal, T 669-671); 31 Aug by Hough and Zimmerman (T 668) | 2/5, men not stated (T 654, 655); 1st Raider Bn, no count | the APDs by name for both crossings |
| 25, fast transports, the Parachutists' crossing | 2 fast transports (97, 98) | Not found by name: "Parachute Bn by YPs", 3 Sep (journal, T 671); 31 Aug (T 668) | 1st Parachute Bn, no count | which YPs or APDs |
| 26, transport, the Seabees of 1 September | 1 troop transport (99) | Betelgeuse 2134, escort Henley 2135; arrived 31 Aug 1700 (2136) or unloading 30 Aug to 2 Sep (2137) | 200 (T 664, 2138), 357 (T 58 on file) or 5 officers and 387 men (T 683, ship not named) | a second ship; departure; tons |
| 5, Placeholder supply convoy; 6 to 13, night forces A to F, bystander, dry destroyer | 3 + 12 | Invented for coverage of the rules (the scenario's header); no historical counterpart, not researched | | |

Other Allied arrivals inside the window that the scenario does not hold (for the architect's choice): Manley and Stringham 16 Aug (2058-2061); Burrows with the MAG-23 echelon (disputed, 2123, 2124, T 657); Burrows and Kopara with Gamble, Tracy, Colhoun, Gregory, Little, 26 to 31 Aug (2105-2133, T 659-661); Helm and the YPs (2140-2146); Fomalhaut and three APDs 3 Sep (2148, T 673); the Savo raid, Little and Gregory, 4 Sep (2149-2154); Tasimboko, Manley and McKean, 7 to 8 Sep (2156-2161); Fuller and Bellatrix 7 to 8 and 14 Sep (2162-2170).

### Japanese forces

| Task force (scenario) | Placeholders | Ships the rows support, with row ids and who names them | Men and supply, with row ids | Not found |
| --- | --- | --- | --- | --- |
| 30, run of 16 August, Yokosuka 5th SNLF | 1 (100) | Oite: combinedfleet 1301, Dyer 2202 | 200 (T 136, Dyer) or 500 (T 137, Miller); landing 16, 17 or 18 Aug (T 700, 701) | departure; supply |
| 31, run of 18 August, Ichiki Detachment | 5 (101-105) | Arashi, Hagikaze, Hamakaze, Tanikaze, Urakaze, Kagero: combinedfleet 1302-1307, 2203-2214; Zimmerman 2215-2220 (two names misprinted, 2219, 2220 inferred) | 900 (T 43, 44, 179), 916 (T 141), 917 (T 703), about 1,000 (T 142), 900 or 1,000 (T 705); left Truk 0500 16 Aug; landed 2030 18 Aug (2223) or 0100 19 Aug (T 707) | supply; Hagikaze's bombing (2224, 2225) has one source |
| 32, run of 29 August (with Fubuki, Shirayuki) | 3 (108-110) | Kawakaze, Umikaze: Zimmerman 1317, 1318; combinedfleet 1319, 1320. Hatsuyuki: combinedfleet 1321, 2326, Tsugaru page (T 711) | 900 (T 45), 750 (T 712), 450 and 300 (T 173, 174), about 450 (passages 1) | which is right; supply |
| 33, run of 31 August (with Hatsuyuki, Amagiri) | 5 (113-117) | Umikaze, Kawakaze, Suzukaze, Kagero, Fubuki, Shirayuki: combinedfleet 1323-1329; Zimmerman names Kagero, Suzukaze 2331, 2332 and "all the above craft" 2333-2336 (inferred); Yudachi by Zimmerman 1331 | 1,200 (T 47, 153), 143 to 155 a ship (T 154) | supply |
| 34, run of 2 September | 1 (118) | Yugure, Kagero: combinedfleet 1332, 1333 only | about 150 and supplies (T 159) | a second source |
| 35, run of 4 September | 5 (119-123) | Yudachi, Hatsuyuki, Murakumo (1334-1336, 2359, 2361, 2362; DANFS Gregory 2366, 2368, 2370); Uranami, Shikinami, Ariake (1337-1339, 2360, 2363, 2364); which section was which disputed (2359, 2360) | about 1,000 (T 161) | supply |
| 36, run of 5 September | 2 (124, 125) | Fubuki, Shirayuki, Amagiri, Kagero, Yugure: combinedfleet 1340-1344 only | 231 (T 163) or 370 (T 164) | a second source |
| 37, run of 6 September | 2 (126, 127) | Uranami, Shikinami, Yudachi, Ariake: landed 370 (1345-1348, T 166) or attack mission only (2374-2377); Zimmerman: Kawaguchi's headquarters landed from destroyers 6 Sep (T 722) | 370 or none | whether troops were landed |
| 38, run of 11 September, Aoba Detachment | 4 (128-131) | Yudachi, Umikaze, Kawakaze: combinedfleet 1354-1356 only; Uranami and Murakumo also have a troop run that day, no place (2378, 2379) | 637 (T 168) | a second source |
| 39, barges, the Oka group | 2 (132, 133) | Not found. Barges from Santa Isabel escorted by Kawakaze, Umikaze, Suzukaze, 4 to 5 Sep (2371-2373, T 721), not linked by any source to the Oka group's 61 barges (T 171, 172, Wikipedia) | 1,100 men (T 171, Wikipedia) | the ships, landing place and date |

Runs the scenario does not hold: the Tanaka convoy of 19 to 25 August (Jintsu, Kinryu Maru, Boston Maru, Daifuku Maru, patrol boats 1, 2, 34 and 35, Mutsuki, Yayoi, CH-24 and others; 2232-2305, 2385-2388, T 724-737), turned back, Kinryu Maru and Mutsuki sunk; Uzuki's run of 24 to 25 August (2382, 2383, T 723); the aborted runs of 27 to 28 August (Desdiv 20, Asagiri sunk; the Isokaze group; 2307-2323, T 708-710); Yudachi on 30 August (1322, 2329, T 150); the run of 1 to 2 September (2341-2355, T 717-719); the Gizo barge convoy (2337-2340, T 716); the runs of 7 September (1349-1353).
