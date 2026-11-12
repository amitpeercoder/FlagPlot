# Cape Esperance, 11 to 12 October 1942: what the sources give a scenario

Status: research report for the architect, read on 2026-11-12. Nothing here is settled. The rows are in `data/oob` under the engagement `cape-esperance` (zone minus 11): `engagement_forces.csv` ids 298 to 375, `engagement_events.csv` ids 490 to 717, `engagement_positions.csv` ids 493 to 541, `engagement_outcomes.csv` ids 1506 to 1685, with 198 class fact rows, 154 ship fact rows, 99 fit rows, four new classes, ten new ships, one new gun and 69 gaps.

## Sources, and how far to trust them

| Source | What it is | Read |
| --- | --- | --- |
| Office of Naval Intelligence combat narrative, 1943 | Wartime account from the US action reports | In full. The US side rests on it |
| Fleet bulletin "Battle Experience", No. 3, 1943 | The reports' chronologies and the fleet commander's comments | Machine-read text; a few garbled names |
| 6th Cruiser Division action report, 9 November 1942 (JACAR C08030045800) | The Japanese force's own report, handwritten | In full, once. The claims page a second time, by me, and it agreed |
| 4th Destroyer Squadron and 8th Fleet diaries (JACAR) | Kinugasa's report, signals, the loss of Natsugumo | The pages for the night |
| Japanese ship records in a modern English compilation | From Japanese records | For damage and dates |
| The US ships' own action reports, Boise's war damage report, Scott's report | | **Not found online** |

**Clocks.** The US sources use zone minus 11. The Japanese records use their own clock, two hours earlier (their 2146 is the US 2346), and no page states the zone. Times are entered as each source prints them. One batch had been converted to the US clock by the researcher; the printed times were restored before merging.

## Disputes first

| Question | What the sources say |
| --- | --- |
| **Who detected first, and at what range** | Helena's surface-search radar at 2325, bearing 315, 27,700 yards (the narrative and the bulletin). A compilation and the post-war survey put a first radar contact about an hour earlier at 18,000 yards; that looks like the other clock, not another event |
| **Who sighted first by eye** | The Japanese. Their report has ship shapes ahead at about 10 km at 2137 to 2143 their time (2337 to 2343), taken as possibly their own landing group, and the enemy confirmed at 7 km at 2140 by the signal log. US lookouts sighted at 2345 |
| **Time and range of opening fire** | Helena at 2346 in every US source, at 3,800 yards (narrative) or about 4,000 (bulletin). The Japanese report has fire received at 2146 their time, the same minute |
| **Scott's cease-fire** | 2346, 2347 or 2350 by three ships' logs; resumed 2351 |
| **The countermarch** | Ordered 2332; executed 2333, 2335 or 2336; "about 2340" in one history |
| **Hits on Aoba** | "Up to forty" by the compilation, "over 40" by a modern Navy summary. The Japanese report says only "badly damaged" |
| **Aoba's dead** | 79, or Goto and 80 others |
| **Kinugasa and Hatsuyuki** | Kinugasa: four hits, or slight damage, or none. Hatsuyuki: damaged by three pages of the compilation and by a Japanese signal (holed forward above the waterline); undamaged by her own page |
| **Furutaka's sinking** | 0228 US time in English sources. The Japanese report's text, chart and signal give 0028, 0048 and 2400 their time |
| **Fubuki's sinking** | About 2353 (US observation), 2158 their time by the Japanese report as read, 2153 by Japanese Wikipedia |
| **Who torpedoed Furutaka** | Duncan, or Buchanan, by source |
| **Who hit Farenholt and Duncan** | Farenholt: US fire, by the narrative and her own report as quoted; one source says both sides. Duncan: both sides |
| **Duncan's end** | Sank about noon on the 12th after a tow failed (narrative); sunk by McCalla (bulletin) |
| **US dead** | "About 175" (narrative, 1943); 163 (later sources) |
| **The reinforcement group's destroyers** | Six, with Yamagumo, or with Ayanami in her place |
| **The Japanese speed** | 30 knots by the Japanese report; 20 by US radar plots; 26 in one history |

## Who was there

**US Task Force 64 (Scott).** In column: destroyers Farenholt, Duncan, Laffey; cruisers San Francisco (flag), Boise, Salt Lake City, Helena; destroyers Buchanan, McCalla.

**Japanese bombardment group (Goto, 6th Cruiser Division).** Aoba (flag), Furutaka, Kinugasa in column, Fubuki to starboard of Aoba and Hatsuyuki to port, 700 m off.

**Japanese reinforcement group.** Seaplane tenders Nisshin and Chitose and six destroyers, unloading at Tassafaronga from about 2210. It passed Savo three hours ahead of Goto and took no part in the night action. Two of its destroyers, Murakumo and Natsugumo, were sunk by aircraft next day while helping survivors; **their rows are marked as the next day's and kept apart.**

New to the data: Helena, Duncan, Laffey, McCalla; Fubuki, Hatsuyuki, Murakumo, Shirayuki, Ayanami, Nisshin; the Brooklyn, St. Louis, Benson and Nisshin classes; the US 6-inch/47 gun (200 rounds a gun, 8 to 10 rounds a minute). The Pensacola class, which had no facts, now has them.

## Radar, on which the night turned

| Ship | Surface search | Air search | Fire control | In use that night |
| --- | --- | --- | --- | --- |
| San Francisco (flag) | **None**: "lacked SG equipment" | Fitted, ordered off for fear the Japanese could detect it | In use on a limited sector | The flagship had no radar picture of her own |
| Boise | Yes | | Yes | Yes; five ships at 2338 |
| Helena | Yes | Listed by Wikipedia only | Yes | Yes; first contact, 2325 |
| Salt Lake City | The narrative contradicts itself: its text says she had one, its table credits her contact to the air-search set | Yes, searching | Yes | Contact at 2323 or 2326 |
| McCalla | | | Yes | Used after the action |
| Farenholt | A contact at 13,000 yards; the set is not named | | | |
| Duncan, Laffey, Buchanan | Not found | | | |
| The Japanese ships | None, by Wikipedia only | | | |

The fleet commander's bulletin remarks on the point, in a sentence that begins "Had the flagship the radar facilities of the Helena and Boise" (row 644 in `fits.csv`).

## The night, in outline (US clock)

| Time | Event |
| --- | --- |
| 1347 | Search aircraft report two cruisers and six destroyers 210 miles from Guadalcanal. A second report at 1810, 110 miles |
| 1815 | The task force at full readiness, expecting contact about 2300 |
| 2200 | Cruiser floatplanes launched; Salt Lake City's burns on the water |
| 2250 | San Francisco's aircraft reports one large and two small vessels off the beach: the reinforcement group |
| 2325 | Helena's radar contact, 27,700 yards. It is not reported to the flagship for some minutes |
| 2332 to 2336 | Column reverses course to 230; the three van destroyers are left on the engaged side |
| 2338 | Boise's radar shows five ships. Her voice report, using the word "bogies", gave two bearings, and only the flagship received both |
| 2337 to 2343 | The Japanese sight ships ahead and take them for their own |
| about 2345 | Helena asks permission to open fire by a procedure signal; the flagship's "Roger" is taken as permission |
| 2346 | Helena opens fire at about 4,000 yards. The Japanese were about to fire; Aoba's bridge is hit by the first shells and Goto is mortally wounded |
| 2347 to 2351 | Scott orders cease fire, believing he is hitting his own destroyers; firing resumes |
| about 2353 | Fubuki sinks |
| 2353 | Boise uses a searchlight |
| 0010 to 0013 | Boise is hit forward; fire in her magazines; she leaves the line. One source has her first hits at 2354 |
| 0020 | The action ends |
| 0228 | Furutaka sinks |
| 12 October, about noon | Duncan sinks |

**Readiness.** The US force had been at full readiness since dusk and expected a fight. The Japanese report says the staff judged there was "almost no chance" of meeting a large surface force, and that fire came as Aoba was about to open on what she had just identified.

**Position of the contact.** Savo Island bearing 280 degrees, 15 nm, course 135, at 2150 Japanese time, by Kinugasa's report and the 8th Fleet's diary. Fubuki's sinking position is the only latitude and longitude found (9°06'S 159°38'E, from the compilation). Furutaka sank 22 nm from Savo on bearing 310.

## Outcomes, in the terms of decision 0054

| | US | Japanese |
| --- | --- | --- |
| Sunk in the night action | Duncan (destroyer) | Furutaka (heavy cruiser), Fubuki (destroyer) |
| Sunk next day by aircraft, kept apart | | Murakumo, Natsugumo (destroyers) |
| Crippled by the rule's test | None. Boise kept 20 knots with her three forward turrets out and a magazine fire; Salt Lake City kept 22 to 25 knots with a boiler room and steering damaged; Farenholt listed with her tubes out | None. Aoba had her bridge wrecked, two turrets out and four boilers off line, and steamed home |
| Damaged, lightly | | Kinugasa, Hatsuyuki (both disputed) |
| Days away, date damaged to date back (arithmetic on sourced dates) | Boise 160, Salt Lake City 151, Farenholt 143 | Aoba 127 |

Rounds fired, where found: Salt Lake City 296 8-inch and 175 5-inch in 29 minutes; McCalla 390 5-inch; from Japanese Wikipedia only, Aoba 7, Furutaka 20 or 36, Kinugasa 80 main-battery rounds and 16 torpedoes. US firing lasted 34 minutes.

## Claims against actual

| Claimant | Claimed sunk | Actually sunk that night | Source of the claim |
| --- | --- | --- | --- |
| US ships' action reports | "About 15" | 2 | Navy narrative, 1943. Read twice |
| The captains' conference afterwards | 3 heavy cruisers, a possible light cruiser, 4 destroyers | 2 | The same |
| Nimitz's revision | 2 heavy cruisers, a light cruiser, an auxiliary, 5 destroyers; Aoba badly damaged | 2 (and 2 next day by aircraft) | The same |
| Japanese 6th Cruiser Division | A heavy cruiser, a light cruiser and a destroyer sunk outright; a light cruiser badly damaged | 1 destroyer | Its own report. Read twice |
| Kinugasa alone | A heavy cruiser by torpedo and a destroyer sunk; a heavy cruiser badly damaged | | The same |

US night surface claims ran four to seven times the losses; the Japanese claimed three for one. At Savo Island the Japanese claimed 14 for 4. **Night surface claims on both sides ran at three to seven times actual, well above the three to one of air fighting.** The ratios are arithmetic on the rows.

## What stayed a gap

- The US ships' own action reports, so rounds fired by ship (only two ships have them), hits by ship, and the destroyers' radar sets.
- Positions in latitude and longitude for either force; column intervals.
- How far the reinforcement group was from the fight.
- Rounds and torpedoes fired by the Japanese ships: the report refers to ship reports that are not in the file.
- Pensacola-class 8-inch rounds per gun; light anti-aircraft guns by ship in October 1942; Akizuki's torpedo model.
- The identification signal attributed to Aoba is in no primary page read: the report says the flagship's signal log was lost in the battle.

## For a scenario

- **The radar asymmetry is inside the US force:** two cruisers had a surface-search set and the flagship did not, and the contact took minutes to reach the admiral.
- **Detection and recognition were different things.** Each side saw the other before fire opened. The Japanese took what they saw for friends; the US admiral took his own van destroyers for the enemy and ceased fire.
- **Readiness decided the first minute.** One side at action stations, the other expecting to shell an airfield.
- **A formation change at the wrong moment** put three US destroyers between the lines; two were hit by their own side.
- **Nobody was crippled by the rule's test, and four ships were away four to five months.** The same point as Enterprise at Eastern Solomons.
