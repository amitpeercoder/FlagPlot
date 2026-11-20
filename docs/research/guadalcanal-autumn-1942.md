# Guadalcanal, 15 September to 30 November 1942: order-of-battle facts

Read on 2026-11-20 for a future scenario of about 115 days, 7 August to 30 November 1942, that carries on from the 39 days (`docs/design/larger-scenarios.md`, the first candidate). This pass gathered facts only: rows in `data/oob`, under the rules of `data/oob/README.md` and `docs/research/oob/database-design.md`. It derives no figure, edits no derivation list, reference file or scenario, and settles no dispute. No existing row was changed: every line of the pull request is an added line.

Nine researchers read the sources under a team lead, each writing to a folder of their own outside the repository. The lead merged the rows and checked them by script (see "How the rows were checked").

## The calendar in brief

Dates are as the sources print them. The Japanese ships' dates are from combinedfleet.com on the Japanese clock (Tokyo time, two hours behind the local clock of -11), so an event of the small hours can fall a day apart from the American date. The full calendar is in `ship_facts.csv` as `movement_note`, `formation_note`, `damage_note`, `sunk` and `repair_place` rows, ids 4000 to 7015.

### Allied heavy ships

| Date | Ship | What | Rows |
| --- | --- | --- | --- |
| 31 Aug, 21 Sep | Saratoga | Torpedoed by I-26 (31 Aug); reached Pearl Harbor with New Orleans 21 Sep | 4024, 4156 |
| 10 Sep | Enterprise | At Pearl Harbor for repair, escorted by San Juan | 4177 |
| 14 to 15 Sep | Washington | Reached Tongatabu 14 Sep; sailed 15 Sep to join Hornet's Task Force 17, then based on Noumea and Espiritu Santo | 4035, 4037 |
| 15 Sep | Wasp, North Carolina, O'Brien | I-19's spread: Wasp sunk (two or three hits), North Carolina and O'Brien hit; O'Brien broke up on 19 Oct on her way to repair | 4015, 4016, 4028, 7008 to 7011, events 3000 to 3017 |
| 16 to 17 Sep | Juneau, San Francisco | Wasp's survivors landed at Espiritu Santo; Juneau rejoined Hornet | 4064, 4065, 4121 |
| 21 Sep | Chester | Reached Noumea after overhaul | 4141 |
| 26 Sep, 2 Oct | Pensacola | Reached Noumea; sailed with Hornet 2 Oct | 4166, 4167 |
| 7 Oct | Task Force 64 (Washington, San Francisco, Salt Lake City and others) | Off or out of Espiritu Santo (the two pages disagree) | 4124, 4134 |
| 16 Oct | Enterprise, South Dakota | Left Pearl Harbor as Task Force 16 | 4048 |
| 20 Oct | Chester | Torpedoed by I-176 southeast of San Cristobal; out of the campaign | 4144, 4145, events 3018 on |
| 24 Oct | Hornet, Enterprise | Joined at 14 07 S 171 37 E as Task Force KING under Kinkaid | 4004, 4018 |
| 26 Oct | Hornet, Enterprise, South Dakota, San Juan | Santa Cruz: Hornet sunk (27 Oct in DANFS), the others damaged; Enterprise and South Dakota repaired by Vestal at Noumea from 31 Oct; San Juan to Sydney | 4007 to 4011, 4019, 4050, 4182, 4196 |
| 3 Nov | Honolulu | Sailed from San Francisco for Noumea | 4098 |
| 8 Nov | Colorado, Maryland | Based on Fiji from November | 4188, 4192 |
| 8 to 11 Nov | Turner's and Scott's groups, Callaghan's Task Group 67.4, Task Force 16 | Sailed from Noumea and Espiritu Santo for Guadalcanal; Enterprise sailed 11 Nov with repairs unfinished | 4068, 4116, 4128, 4021, 4054 |
| 13 Nov | Atlanta, Juneau | Juneau sunk by I-26 that morning; Atlanta scuttled off Lunga that evening | 4073, 4074; event 2811 |
| 15 Nov | South Dakota | 42 hits in the battleship action; sailed from Noumea for home 25 Nov | 4057, 4058 |
| 16 to 25 Nov | Enterprise, Washington, San Diego | At Noumea | 4023, 4046, 4081 |
| 24 to 25 Nov | Saratoga, North Carolina, Colorado, Maryland, San Juan | At Nandi, Fiji | 4025, 4026, 4031, 4189, 4193 |
| 25 to 29 Nov | Northampton, Pensacola, New Orleans, Honolulu, Minneapolis | At Espiritu Santo; sailed 2335 on 29 Nov for Tassafaronga | 4088, 4090, 4152, 4158, 4174 |
| 1 Dec | Northampton | Sank just after 0300 after Tassafaronga (the brief had 30 Nov) | 4093, 4094 |

### Japanese heavy ships

| Date | Ships | What | Rows |
| --- | --- | --- | --- |
| 10 to 11 Sep | Second and Third Fleets (Shokaku, Zuikaku, Zuiho, Kongo, Haruna, Hiei, Kirishima, cruisers) | Sortied from Truk north of the Solomons; back at Truk 23 Sep with no attack made | 5301, 5309, 5353, 5374, 5388, 5302 |
| 25 Sep to 8 Oct | Zuiho | To Yokosuka and back | 5318 to 5321 |
| 29 Sep, 2 Oct | Haguro, Jintsu | Left Truk for Sasebo and Kure | 5470, 5565 |
| 4 to 9 Oct | Junyo, Hiyo | Carrier Division 2 from Saeki to Truk | 5327, 5328, 5337, 5338 |
| 11 Oct | Both fleets and Carrier Division 2 | Sortied from Truk for the October offensive | 5303, 5311, 5322, 5330, 5340 |
| 11 to 17 Oct | Aoba | Damaged at Cape Esperance (the battle is on file); Truk 15 Oct, left for Kure 17 Oct | 5502 to 5507 |
| 11 to 14 Oct | Chitose, Nisshin | Carried troops and guns from Shortland to Tassafaronga; Chitose left Truk for Sasebo 10 Nov | 5628, 5629, 5632 |
| 13 to 14 Oct | Kongo, Haruna, Isuzu and destroyers | Bombarded Henderson Field (435 and 483 rounds by combinedfleet; 918 by Miller) | 5382, 5394; outcomes 4400 on |
| 21 to 26 Oct | Hiyo | Generator-room fire 21 Oct; back to Truk 26 Oct, repaired there; left for Kure 5 Dec | 5341 to 5349 |
| 25 Oct | Yura | Crippled by aircraft and scuttled by Harusame and Yudachi | event 3041 |
| 26 Oct | Shokaku, Zuiho, Chikuma | Damaged at Santa Cruz | 5305 on; outcomes 4400 on |
| 28 to 30 Oct | Both fleets | Back at Truk | 5305, 5313, 5334, 5384 |
| 2 and 4 Nov | Shokaku, Zuiho, Chikuma, Kumano; Zuikaku, Myoko | Left Truk for Japan | 5306, 5314, 5324 |
| 9 to 10 Nov | Hiei, Kirishima; Kondo with Junyo, Kongo, Haruna | Left Truk 9 Nov; Abe's force left Shortland 10 Nov | 5335, 5363, 5364, 5372, 5373, 5385, 5396 |
| 13 Nov | Hiei | Sunk after the night action | 5365; outcomes 4700 on |
| 14 Nov | Kinugasa | Sunk by aircraft after Mikawa's bombardment | event 2821 |
| 15 Nov | Kirishima | Sunk in the battleship action | outcomes 4700 on |
| 18 Nov | Kongo, Haruna, Junyo | Back at Truk | 5336, 5387, 5398 |
| 22 to 27 Nov | Takao | Left Truk for Kure for battle-damage repair | 5432 to 5434 |

Yamato and Mutsu stayed at Truk throughout. Nagato was not found in any page read. Fifty-four Japanese destroyers have dated rows from combinedfleet.com (5900 to 6491), and the table by division is in the researcher's notes, summarised under "What was added".

### Reinforcement of Guadalcanal

| Date | Side | Run | Men and tons | Rows |
| --- | --- | --- | --- | --- |
| 18 Sep | US | 7th Marines | on file | on file |
| by 30 Sep | JP | Rest of the 4th and all the 29th Infantry (2d Division) by destroyer and barge | 29th Infantry entire | land_unit_facts 1930 to 1932 |
| 11 to 12 Oct | JP | Chitose and Nisshin with six destroyers | 728 men (Zimmerman) or 280 on the tenders (combinedfleet); four 150 mm howitzers | lift 1503; troop_lift 800 to 807 |
| 13 Oct | US | 164th Infantry | on file; 3,200 and 1,000 ships' tons added | troop_lift 1102, 1103 |
| 13 to 15 Oct | JP | Six-transport convoy (Sado, Azumasan, Kyushu, Nankai, Sakito, Sasako) | about 4,500 embarked; 3,000 to 4,000 landed; three ships beached and lost | troop_lift 808 to 816; ship_facts 6605 to 6625 |
| 2 Nov | US | Alchiba and Fuller, two 155 mm batteries | batteries, stores | lift 1500 |
| 2 to 3 Nov | JP | Koli Point | about 1,500 of the 230th Infantry | troop_lift 817 |
| 4 Nov | US | 8th Marines and 1st Battalion 10th Marines; Aola force (1st Battalion 147th, two Raider companies, 500 Seabees) | | lift 1501; troop_lift 840 to 846 |
| 7 to 11 Nov | JP | 38th Division's advance by destroyer | 1,300 and 600 | troop_lift 818 to 822 |
| 11 to 12 Nov | US | Scott's and Turner's groups (182d Infantry) | about 6,000 men; share of cargo landed per ship | troop_lift 849 to 851; ship_facts 6663 to 6676 |
| 13 to 15 Nov | JP | Eleven transports (Tanaka) | about 10,000 men and 10,000 tons embarked; 2,000 or 4,000 landed; 5 tons landed | troop_lift 823 to 836, 1100; ship_facts 6626 to 6654; events 3156 to 3171 |
| 29 Nov | US | Four transports to Aola Bay, 3d Battalion 147th | | lift 1502 |

## What was added, by file

| File | Rows | Ids | What |
| --- | --- | --- | --- |
| `ships.csv` | 76 | 1240 to 1248, 1280 to 1298, 1340 to 1349, 1370 to 1386, 1420 to 1435, 1480 to 1484 | Hornet, Washington, Juneau, San Diego, Honolulu, Northampton, Colorado, Maryland, Vestal; 19 destroyers and Montgomery; Junyo, Hiyo, Kongo, Haruna, Yamato, Isuzu, Sendai, Chiyoda, Taiyo, Unyo; 17 Japanese destroyers; 16 Japanese transports; three Japanese oilers named at Santa Cruz, Clark and Seminole |
| `ship_facts.csv` | 1,431 | 4000 to 7015 | Movements, formations, damage, repair, losses, and dates of the new ships; the transports' runs and fates |
| `engagement_forces.csv` | 289 | 600 to 679, 900 to 1085, 1300 to 1322 | Santa Cruz both sides (80), the bombardments of 13 to 16 October, the two night actions of November and Tassafaronga by formation and column (186), and Task Forces 17 and 18 of 15 September (23) |
| `engagement_outcomes.csv` | 257 | 4400 to 4499, 4700 to 4827, 5100 to 5128 | Ships sunk and damaged, hits, killed and wounded, aircraft lost and claimed, the bombardments' rounds and their effect |
| `engagement_events.csv` | 140 | 2600 to 2646, 2800 to 2827, 3000 to 3042, 3150 to 3171 | Attacks, torpedo hits, sinkings, the convoys' air attacks |
| `engagement_positions.csv` | 61 | 650 to 663, 750 to 781, 900 to 908, 980 to 985 | Track points, reported positions, sinking positions |
| `engagements.csv` | 1 | 1015 | `guadalcanal-autumn-1942`, a holder for what belongs to no named battle (zone -11) |
| `air_units.csv`, `air_unit_facts.csv` | 23, 127 | 1100 to 1122; 700 to 826 | Marine, Navy and Army squadrons at Henderson Field and Espiritu Santo, the P-38 and B-26 squadrons, the Santa Cruz air groups, Junyo's and Hiyo's groups; Cactus totals by date; the renumbering of the Japanese groups as aliases |
| `aircraft_types.csv` | 1 | 1040 | P-38 |
| `land_units.csv`, `land_unit_facts.csv` | 15, 63 | 1200 to 1214; 1900 to 1961, 2300 | Japanese 16th, 29th and 229th Infantry and artillery; US 245th and 246th Field Artillery, 3d Battalion 147th; landings, commanders, strengths; Japanese strength on the island |
| `lift_entities.csv`, `troop_lift_facts.csv` | 4, 56 | 1500 to 1503; 800 to 851, 1100 to 1103 | Four new runs; men, ships and tons of the runs above |
| `sources.csv` | 2 | 1620, 1780 | `destroyerhistory-ships` (B), `wdr-wasp-sep42` (A) |
| `gaps.csv` | 134 | | What was looked for and not found |

Two rows were written and dropped at merge as repeats of the same page's statement (ids 7001 and 7007, Meredith's and Juneau's fate); their ids are left unused. One row, 4660 (Smith's damage at Santa Cruz), was filed as `damage_note` because row 688 on file already gives her date out of action, and a second row for that date would have needed the standing of 688 changed.

New ships with no class key, because no class of theirs is on file or no page read states it: Hornet (DANFS says "cl. Hornet"; the Yorktown class is on file), Colorado, Maryland, Vestal, Fletcher, Nicholas, O'Bannon, Montgomery, Akatsuki, Ikazuchi, Inazuma, Hibiki, Naganami, Makinami, Takanami, Mikazuki, Satsuki, Fumizuki, Minazuki, Junyo, Hiyo, Yamato, Taiyo, Unyo, Chiyoda, Clark and the transports. Classes wanted for a later pass: Fletcher, South Dakota (her class is empty on file too), Colorado, Akatsuki, Hiyo, Yamato, Taiyo. Clark's hull number (DD-361) is in her key but was not read in any page, so her `hull_number` is empty. Honolulu's class (Brooklyn) was set by a researcher from general knowledge, and Washington's from a DANFS page that names the class.

## Sources and how firm they are

Of the 2,424 fact and engagement rows: 547 cite sources graded A, 1,817 B and 60 C; 434 are high confidence, 1,906 medium and 84 low; 2,266 stand preferred, 83 disputed, 73 unconfirmed and 2 alternative (the high ends of two stated ranges of Cactus strength, rows 704 and 713).

| Source | Grade | Rows | Note |
| --- | --- | --- | --- |
| combinedfleet.com tabular records | B | 1,083 | The Japanese ships' movements, damage and runs; the Japanese clock. The site timed out for long spells; some pages were read through the Wayback Machine |
| DANFS (HyperWar mirror; the Navy's site through the Wayback Machine) | B; A | 236; 33 | US ships' movements. Short entries, often undated |
| ONI Combat Narratives: Guadalcanal, Santa Cruz, Tassafaronga, Miscellaneous Actions | A | 410 | Santa Cruz in zone -12 (its footnote 4); misnames many Japanese ships |
| USSBS, Campaigns of the Pacific War | B | 81 | The Japanese forces of November; its tables carry misprints and lists Helena sunk |
| Miller; Hough, Ludwig and Shaw; Zimmerman; Shaw; Craven and Cate vol. 4; Mersky | B; A | 202 | The runs, the Cactus Air Force, the bombardments |
| Lundgren's papers on the November battles | B | 86 | The Japanese groups and damage |
| Cressman's chronology | B | 45 | Dated losses |
| NavWeaps order of battle of Santa Cruz | B | 54 | The only Japanese order of battle for Santa Cruz reached; it cites Hammel and Horan, not read |
| Pacific Wrecks, destroyerhistory.org, Tanaka, the Enterprise report, Bulletin 4, war damage reports | A to B | 168 | |
| Wikipedia | C | 60 | Every row low; the Japanese air units and the Santa Cruz air totals rest on it |

## The main disputes, for the architect

1. **The clocks and the date of the battleship bombardment.** The ONI narrative of Santa Cruz keeps zone -12; `engagements.csv` sets no zone for `santa-cruz`, and these rows say their clock in the note. combinedfleet.com keeps Tokyo time. The bombardment by Kongo and Haruna is 13 October at 0137 (combinedfleet, row 2410 on file), shortly before midnight of 13 October (Miller) or 14 October at 0137 (Pacific Wrecks); the Hiei and Kirishima pages put it on 14 to 15 October. The October convoy's air attacks are 14 October (combinedfleet) or 15 October (Miller, Hough, Zimmerman, Pacific Wrecks).
2. **Strength at Henderson Field on the days that mattered.** 26 October: 30 operational (Hough) or 23 fighters, 16 dive bombers and 1 torpedo plane (ONI). 14 October: 42 of 90 (Hough), with 7 (Mersky) or 5 (Craven and Cate) dive bombers. 12 November: 41 F4F, 30 SBD-3, 19 TBF-1, 2 P-400 and P-38s (Craven and Cate) or 47 fighters, 23 light and 12 medium bombers (Wikipedia). Squadron dates disagree too: MAG-11 on 31 October, 1 November or 2 November; VMF-212 on 5 September or 16 October.
3. **The Japanese forces in November.** Whether Tokitsukaze, Hatsukaze, Shigure, Shiratsuyu and Yugure were in the night action of 13 November (USSBS yes, Lundgren no); Mikawa's group of 13 to 14 November (Chokai, Maya, Suzuya by Lundgren; Kinugasa, Isuzu, Suzuya, Maya, Tenryu and two destroyer divisions by USSBS); Tassafaronga's eighth destroyer (Makinami, by USSBS only) and how many were sunk (one, or two by an ONI footnote, four by its claim).
4. **The convoys' loads.** Chitose and Nisshin on 11 October: 728 men or 280. The October convoy: the 2d Division (Hough) or the 230th and part of the 16th Infantry (the others). The November convoy: about 2,000 men landed (combinedfleet, Pacific Wrecks) or about 4,000 (Miller); Shinanogawa Maru (combinedfleet) or Kumagawa Maru (one Pacific Wrecks list) as one of the eleven. The Japanese strength on the island at its peak: 30,000 (Miller) or 26,000 Army and 3,000 naval (the USAFISPA report quoted by Zimmerman).
5. **Losses.** Wasp's torpedo hits, two (her war damage report, DANFS) or three (ONI, Juneau's DANFS entry); Lansdowne's torpedoes to scuttle her, four or five. Hornet's fatal torpedo at 1523 (Wikipedia) or 1620 (ONI), and whether Makigumo and Akigumo sank her. Aircraft lost at Santa Cruz: US 74 (ONI) or 80 (NavWeaps) or 81 (Wikipedia); Japanese 99 (NavWeaps) or 100 (Zimmerman). Meredith sunk 15 or 16 October. Porter's torpedo from a submarine (her DANFS page) or from an American aircraft (destroyerhistory.org, Anderson's page). Juneau's sinking position, 161 03 E or 161 44 E, about 40 miles apart.

Smaller ones are in the rows' notes: where Hornet met Enterprise (northeast or northwest of the New Hebrides), South Dakota's collision with Mahan (27 or 30 October), New Orleans at Espiritu Santo (by 25 or on 27 November), Hiei scuttled or sunk by her own shock, Laffey's last minute, the hits on San Francisco (15 of major calibre in ONI, 45 of all calibres on file), and the renumbering dates of the Japanese air groups (1 October or 1 November).

Rows that disagree with each other are marked `disputed` where the file's rule allows. A new row whose value differs from a row on file that stands `preferred` would have needed that row changed; there were none left after the merge.

## The gaps, and where to look next

The 134 entries of `gaps.csv` added by this pass name what was looked for. The main ones:

- **Japanese clock rows for the battles.** combinedfleet.com would not answer for most of the session, so the battles carry few Japanese-side times, and there are no wreck or sinking positions for Hiei, Kirishima, Kinugasa or Takanami. The site again, or the Wayback Machine at a quieter hour.
- **Killed and wounded by ship** for Hornet, Smith, Porter, South Dakota at Santa Cruz, Preston and Gwin, the four cruisers at Tassafaronga, and every Japanese ship. The war damage reports on HyperWar and the ships' action reports.
- **The Cactus Air Force by squadron and type on dated days**, the Navy squadrons' departures after 15 September, and the PBYs. Sherrod's history (not reached), the squadron war diaries, the MAG-23 and MAG-14 reports.
- **The Japanese air units by unit after September.** Every Japanese air row rests on Wikipedia. The Japanese Monographs on the Southeast Area air operations and the USSBS strength graphs (images, not read). The date Kahili opened.
- **Night by night, the Tokyo Express of 20 September to 12 November**, and the men and tons of each transport. Japanese Monograph 98 was downloaded (137 MB, no text layer) and not read; OCR of its pages is the next step.
- **The Hornet's screen at Santa Cruz as one list**, Hornet's own action report, CINCPAC's report on the November battles as a document.
- **The names** of the transports of the 8th Marines, two of the three Aola transports and the four of 29 November.
- **Supply by month.** No source read gives tons reaching Guadalcanal, Espiritu Santo, Noumea, Rabaul or the Shortlands by month for these months; the rows on file (`docs/research/oob/supply-levels-and-convoy-cycles.md`) are still all there is.
- **Allied cruisers of Australia and New Zealand, oilers and tugs**, and several destroyers (Sterett, Stack, Wilson, Lang, Gridley, McCalla, Buchanan, Helm, Tracy, Preble, Breese, Southard) whose pages give nothing for these months.

Not reached: combinedfleet.com for long spells; the Wayback Machine after bursts of requests; Sherrod on the Internet Archive (lending copy, refused); NavSource and history.navy.mil directly (not tried; read through the Wayback Machine where used).

## The map

The present map covers every hex whose centre lies from 1 S to 24 S and 145 E to 172 E. Every latitude and longitude in these rows is inside it: from 7 00 S to 14 07 S and from 147 30 E to 171 50 E. The eastern edge is close: Hornet and Enterprise met at 14 07 S 171 37 E on 24 October, about 20 miles inside it, and their approach from the east began beyond it. Places named in the rows and outside the map, as they were for the 39 days: Truk (about 7 N), where the Japanese fleets lay between sorties; Japan; Palau, Davao and Manila; Pearl Harbor; Tongatabu, Fiji (Nandi), Funafuti and Tarawa. Fiji and Tongatabu, where Saratoga, the older battleships, Washington and the repair ship lay in these months, are beyond the eastern and southern edges; a scenario that brings ships in from them would enter them at the map's edge.

## How the rows were checked

- Every page or document a row cites was fetched by script into a folder of its own and turned into plain text. Each researcher listed, for every row, the text file read and a verbatim fragment.
- The lead's script then checked all 2,426 fact and engagement rows of the staging files: the fragment was found in the text for every row, and the row's own `as_stated` was found in the text for all 2,137 rows that have one (with case, quotes, dashes and line breaks made alike). For every web page it also checked that the page's address matches the row's locator; two Wikipedia pages that redirect to a new title were the only differences. 92 rows rest on PDFs (war damage reports, Lundgren), whose text has no address line; their folders were checked by hand.
- A sample of rows from each file was read by the lead against its quotation for whether the value says what the quotation says. Not every row was read this way: the script proves the words are in the source, not that the value was read from them rightly.
- `oob-check` reports no problems and `cargo test -p oob-check` passes. The six derivation lists' checks (`oob-derive --check`) pass, so no generated file changes.
