# The record for step 2: outcomes, claims, repair and base sizes

Status: research report for the architect, read on 2026-11-12. Nothing here is settled. Every figure is a row in `data/oob` with its source and locator. "Checked" means a second reading of the cited page against the row; everything else is as the researcher recorded it and is a lead until checked.

| Asked for | Where the rows are | New rows |
| --- | --- | --- |
| Outcomes and claims, Savo Island | `engagement_outcomes.csv` ids 745 to 800 | 56 |
| Outcomes and claims, Eastern Solomons | `engagement_outcomes.csv` ids 656 to 744 | 89 |
| Outcomes and claims, the air campaign; supply; Henderson Field's state | `engagement_outcomes.csv` ids 500 to 655 and 801 to 909 | 265 |
| Ship repair, by ship | `ship_facts.csv` ids 613 to 769 (attributes `out_of_action_from`, `repair_place`, `returned_to_service`) | 157 |
| Repair capacity and base sizes | `place_facts.csv`, a new file | 188 |
| Looked for and not found | `gaps.csv` | 95 |

Every claim row's note begins CLAIM and every loss taken from the loser's records begins ACTUAL.

## 1. Outcomes in the terms of decision 0054

### Savo Island, 8 to 9 August 1942

| | Allied | Japanese |
| --- | --- | --- |
| Ships sunk | 4 heavy cruisers: Canberra (scuttled), Vincennes, Quincy, Astoria | None in the action. Kako was sunk by a submarine on 10 August on the way home |
| Ships crippled | None by the 0054 test. Chicago lost part of her bow to a torpedo and was held to about 10 knots, but steamed and screened the transports next day. Ralph Talbot lost steering and took a 20 degree list, then recovered | None. Chokai had one turret out and a hit near her chart room; Aoba, Kinugasa, Tenryu and Yubari had slight damage and all could steam and fight |
| Lost afterwards, not in the action | Jarvis, damaged by aircraft on 8 August, sunk by aircraft on 9 August | |

Source: the 1950 Naval War College analysis, which uses both sides' records. Checked for the claims paragraph (p. 346). Rows 761 to 800. Killed and wounded totals by side were not found in an official source.

### Eastern Solomons, 24 to 25 August 1942

| | US | Japanese |
| --- | --- | --- |
| Ships sunk | None | Ryujo (carrier), Mutsuki (destroyer), Kinryu Maru (transport) |
| Ships crippled | Enterprise: three bombs. Her deck was ready again at 1816 and the first plane landed at 1819, about an hour after the attack; steering failed at 1851 for 38 minutes. By the 0054 test (flight deck out at the end) she is not crippled, yet she was out of the war for 53 days. Checked | Chitose (near misses: port engine room flooded, towed or steamed to Truk; sources differ on the list and the route). Jintsu (one bomb forward, to Truk and Kure) |
| Aircraft lost | 20 by the 1943 Navy narrative and the post-war summary; other sources 6, 9, 16 or 25, by what they count | About 40 carrier aircraft from Japanese-record compilations (24 in the first wave, 5 in the second, 7 of Ryujo's strike, 4 lost with her). Wikipedia, from books not read: 75 of all kinds. The Japanese official history as Japanese Wikipedia quotes it: 30 fighters, 23 dive bombers, 6 attack planes, 3 seaplanes, which is 62. **Unresolved** |
| Convoy | | No troops landed on 25 August. The convoy turned back; the men reached Guadalcanal by destroyer from 29 August. Troops embarked: four different figures |

Rows 656 to 744. Not found: aircraft lost on Enterprise's deck, PBY losses, troops lost in Kinryu Maru, cargo tonnage.

**A point for the scoring rule.** Enterprise is the case 0054 does not catch: not crippled at the end of the day by the rule's test, and away for seven weeks.

### The air campaign, 21 August to 17 September 1942

- **Raid days found:** 21, 24, 25, 26, 29, 30 August; 2, 5 (disputed), 9, 10, 11, 12, 13, 14 September. The Japanese monograph says weather stopped the raids on 22 to 23 and 28 August and 1 September, turned one back on 4 September, and that the raids of 6 to 7 September went to Port Moresby. None on 15 to 17 September.
- **Ships sunk in the period, outside the two battles:** US: Blue (22 August), Colhoun (30 August, bombers), Gregory and Little (5 September, destroyers), Wasp (15 September, submarine); O'Brien torpedoed 15 September and lost on passage in October. Japanese: Asagiri (28 August, aircraft).
- **Ships crippled:** Saratoga (31 August, one torpedo, propulsion out, towed), North Carolina (15 September, one torpedo, still made 25 knots). Japanese: Yugiri and Shirakumo (28 August).
- **Supply landed, US:** 15 August, 400 drums of aviation fuel, 282 bombs and 123 men by four destroyer-transports. About 21 August, 120 tons of rations. 30 August, 17 tons. 1 September, 392 Seabees (one source: 200). 18 September, 4,262 men (one source: 4,180), 1,012 tons of rations, 3,823 drums of fuel, 147 vehicles. Checked for the 15 August, 21 August and 18 September figures. On 7 to 9 August under half of 60 days' supplies had been landed.
- **Aircraft delivered to Henderson Field:** 31 on 20 August, 5 on 22 August, 11 on 24 August, 9 on 27 August, 31 on 30 August (one history dates these 21 August), 24 on 11 September, 18 on 13 September.
- **Reinforcement landed, Japanese:** about 900 to 1,000 on 18 August; none on 25 August; 900 on 29 August; 1,200 on 31 August; about 1,000 of 1,100 by barge after 2 September. On the island by 6 to 7 September: 3,190 by one history, "over 6,000" by two others. **No source gives tons for any Japanese run.**
- **Days Henderson Field could not fly:** one dated closure, about 18 August, after 17 bomb hits on the runway. No dated rain-out was found, though two histories say fighters could not operate on muddy days. The Seabee history's claim that the field "was never out of operation for more than four hours" is recorded as a claim (row 892).
- **Aircraft destroyed on the ground:** almost nothing found: three dive bombers burned on 2 September, one P-400 on 11 September. A gap.

## 2. Claims against actual losses

| Engagement | Claimant | Claimed | Actually lost | Ratio |
| --- | --- | --- | --- | --- |
| Savo Island | Japanese cruiser force | 14 ships sunk (2 British and 5 US heavy cruisers, 1 light cruiser, 6 destroyers), 2 damaged | 4 sunk, 3 damaged | 3.5 on ships sunk |
| Savo Island, the air attack of 8 August | Japanese land-based air | 14 ships sunk (2 heavy cruisers, 1 large cruiser, 2 destroyers, 9 transports), 3 damaged | 1 destroyer (Jarvis) and 1 transport (George F. Elliott) damaged, by the 1950 analysis | far above any ratio |
| Savo Island | Allied | No ships claimed sunk; "several hits" | Slight damage to five ships | none |
| Eastern Solomons | US fighters | 46 aircraft (26 and 17 by the two squadrons, plus 3 scouts) | | |
| Eastern Solomons | US ships' guns | About 30 by ship, which the Navy narrative says overlap | | |
| Eastern Solomons | US, all, at the scene | About 70 | 24 of the 37 in the wave that attacked | 2.9 |
| Eastern Solomons | US, ships | 13 sunk or damaged in the narrative's headline; its own list adds to 8 sunk and 5 damaged | 3 sunk, 2 to 3 damaged | 2.7 on ships sunk, by the list |
| Eastern Solomons | Japanese carrier aircrews | Two carriers hit, "heavily damaging one ship and lightly damaging the others". Checked | One carrier hit | 2 |
| Air campaign | US fighters and guns, on the four days with a loss figure from a named book | 43 | 19 | 2.3 |
| Air campaign | The same, adding three days from the unsourced chronology | 82 | 28 | 2.9 |
| Air campaign | Japanese, 30 August only | 12 | 4 shot down, 6 more written off | 3 or 1.2 |

The ratios are arithmetic on the rows, not statements by any source.

**What this supports for decision 0055, and how firmly.**
- US aircraft claims run at about two and a half to three times losses, in the carrier battle and over Guadalcanal alike. Medium confidence: the Japanese loss figures all pass through modern compilations or Wikipedia.
- Japanese claims of ships by night surface action: three and a half times. One battle, well sourced.
- Japanese claims of ships by air attack: enormous on 8 August, double on 24 August. Two cases.
- Japanese claims of aircraft: one day. **Not enough to set a chance.** No Japanese claim of aircraft at Eastern Solomons was found at all.
- Aircraft claimed destroyed on the ground: nothing found for either side.

## 3. Ship repair

**Forward bases, as the sources describe them** (`place_facts.csv`, attributes `repair_ship_present`, `drydock`, `repair_capability_note`, `repair_example`):

| Base | What it could do in the period |
| --- | --- |
| Tongatabu | Repair ship Vestal from 29 August: 963 jobs on 58 ships in 60 days. Divers and patches; no dock. Checked |
| Noumea | Tenders only until Vestal came on 31 October; ships were "patched" to reach a yard, with "no facilities for turret work". A small floating dock for destroyers by late autumn. Checked |
| Espiritu Santo | A repair ship from mid November 1942; cruisers had "practically no repair facilities". Checked |
| Tulagi | Ship's force and Seabees only; "inadequate repair facilities". Eleven days made New Orleans seaworthy. Checked |
| Sydney | The nearest docks that could take a damaged cruiser |
| Truk | Repair ship Akashi from 23 August 1942; the Japanese reckoned she could do 40 per cent of the fleet's repairs. Checked |
| Rabaul | A repair ship from 2 October 1942; emergency repairs, long ones at Truk |

**Time away**, date damaged to date back (the arithmetic is mine):

| Ship | Damage | Forward repair | Yard | Days away |
| --- | --- | --- | --- | --- |
| South Dakota | Bomb, 26 October | Noumea | none | 16 |
| Enterprise | Bombs, 26 October | Noumea | none | 16 |
| Chitose | Near misses, 24 August | Truk | Yokosuka | 25 |
| San Juan | Bomb, 26 October | | Sydney | 29 |
| South Dakota | Grounding, 6 September | Tongatabu | Pearl Harbor | 36 |
| Enterprise | Bombs, 24 August | Tongatabu | Pearl Harbor | 53 |
| North Carolina | Torpedo, 15 September | Tongatabu | Pearl Harbor | 63 |
| Saratoga | Torpedo, 31 August | Tongatabu | Pearl Harbor | 71 |
| Ralph Talbot | Shells, 9 August | | Mare Island | 94 |
| Zuiho | Bombs, 26 October | Truk | Sasebo | 89 |
| Chikuma | Bombs, 26 October | Truk | Kure | 124 |
| Shokaku | Bombs, 26 October | Truk | Yokosuka | 125 |
| Aoba | Shells, 11 October | Shortland, Truk | Kure | 127 |
| Jintsu | Bomb, 25 August | Truk | Kure | 137 |
| Yugiri | Bombs, 28 August | Truk | Kure | 140 |
| Farenholt | Shells, 11 October | Noumea | Pearl Harbor | 143 |
| Chicago | Torpedo, 9 August | Noumea, Sydney | San Francisco | about 150 |
| Boise | Shells, 11 October | Noumea | Philadelphia | 160 |
| Chester | Torpedo, 20 October | Espiritu Santo, Sydney | Norfolk | about 330 |

**The pattern for a repair rule.** Three bands. Damage a repair ship can patch: two to four weeks, done forward. Torpedo or heavy bomb damage: patched forward for one to three weeks, then a yard, two to five months in all. A cruiser with her bow or a turret gone: six months to a year. No forward base in the slice could dock a cruiser; the Japanese had the same split between Rabaul, Truk and the home yards.

Not found: a dock at Truk, Akashi's count of ships, Rabaul's shore facilities, and exact dates for several returns (month only).

## 4. Base sizes, August and September 1942

| Base | Runways | Aircraft | Port |
| --- | --- | --- | --- |
| Henderson Field | One. 2,600 ft finished by the Japanese, 1,178 ft added; 3,800 by 150 ft when the Seabees took over on 2 September. Earth and gravel, mud after rain; steel mat from 25 September. A second, grass strip of 4,600 by 300 ft from early to mid September. Checked | Planned for 36 fighters and 9 scout bombers. 31 on 20 August; about 64 at the end of August; 50 to 70 from mid September | None. Everything over the beach by boat. 400 drums of fuel on capture (or 800 to 900), no tanks until late October |
| Rabaul, Lakunai | One, 4,700 ft, sand and volcanic ash, later coral | Not found for the period | Simpson Harbour: anchorage for at least 20 large ships |
| Rabaul, Vunakanau | One, 5,100 ft grass with a 4,050 ft concrete centre strip | Not found for the period. The Japanese monograph has the whole base air force at 19 fighters, 29 bombers and 4 flying boats on 26 August | |
| Rabaul, others | Rapopo, Tobera and Keravat did not exist yet (1943) | | |
| Buka | Operational October 1942 by one history; the monograph has 30 carrier fighters sent there in late August | | |
| Espiritu Santo | One, 6,000 ft coral, opened 28 July 1942. A fighter strip about October | 14 B-17s | No docks; fuel by drum |
| Noumea | Tontouta, 5,000 ft (one source says two runways); Magenta unfinished | Not found | Two quays. 5,000 tons a day in early December 1942 and still behind arrivals. Checked |
| Tulagi | None. A seaplane base | | Anchorage |
| Milne Bay | No. 1 strip with steel mat from mid July; No. 3 begun 7 August | Two RAAF fighter squadrons from late July | No wharf; ships unloaded onto pontoons |

**Weak or missing:** Rabaul's figures come from a history describing late 1943 and from a website; the post-war US survey of Rabaul could not be read. Aircraft capacity is sourced for Henderson Field only. Unloading rates: one figure, for Noumea in December. Runway lengths at Milne Bay and Buka in the period: not found.

`place_facts.csv` is new and `oob-check` does not read it. Its attributes: `runway_count`, `runway_length_m`, `runway_width_m`, `runway_surface`, `runway_state_note`, `airfield_opened`, `aircraft_capacity`, `aircraft_based`, `seaplane_base`, `revetments_note`, `aviation_fuel_storage_note`, `port_note`, `port_berths`, `unloading_rate_tons_per_day`, `ships_waiting_note`, `garrison_strength`, `construction_units`, `repair_ship_present`, `drydock`, `repair_capability_note`, `repair_example`. They are not in `attributes.csv`, because the checker rejects an attribute for an entity type it does not know. A schema entry for places is the architect's to add.

## How firm the new rows are

- **Firm:** Savo Island outcomes and claims (the 1950 analysis); Enterprise on 24 August (her own report); US supply runs and aircraft deliveries (three official histories that mostly agree); repair capacity (the Navy's logistics history and ship histories).
- **Middling:** Japanese ship damage and repair dates (one modern compilation from Japanese records); US claims per day over Guadalcanal (official histories, which round).
- **Weak:** Japanese aircraft actually lost, per day and in total. Of the 265 air campaign rows, 49 come from an unsourced hobby chronology and 44 from Wikipedia; they are graded low, marked, and kept only because nothing better gives a per-day Japanese figure. The daily Japanese air journals (monographs 112 and 120) are not online. Lundstrom's two books would replace all of these rows.
- **A slip of mine while merging,** caught and reversed before commit: a tidy-up briefly rewrote existing ship names. `ships.csv` differs from main only by ten added ships.

## Still to do

The Japanese side of Eastern Solomons from Japanese records (search orders, tracks) stands as the next item, and with it the Japanese loss totals above. *(Nov 14, 2026: done: `japanese-records-2026-11-12.md` and `japanese-air-records.md`.)*
