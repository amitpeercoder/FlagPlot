# The Coral Sea, stage A: the table and the research for its gaps

Read on 2026-11-20 for decision 0188, rules 23 and 43 as amended. Part 1 is the table rule 23 asks for: each ship and aircraft type of the forces of rules 4 and 5, whether its class is derived today, and what the rows lack for the figures `oob-derive` needs as of 6 May 1942. Part 2 is the research commissioned by rule 43 and what it found. Nothing is derived here, no derivation list or scenario is written, and no dispute is settled: those are the architect's.

## How the table was made

A scratch derivation list outside the repository named every warship of the forces below and every aircraft type, dated 1942-05-06, and was run with `oob-derive --check` twice: once with surface weapons modelled, once with `surface_weapons_not_modelled = "all"` and every ship in `radar_unknown_means_none`, as the Eastern Solomons list has them. Each line below is what the tool reported after this pass's rows were merged. The list had no `[class_ids]` table, which the tool also reports; that is the architect's to write. A "problem" is the tool's word for a figure it will not derive without a ruling or a new row.

The forces are those of `engagement_forces.csv` for `coral-sea`, both sources' lists, leaving out the Tulagi force, the Support Force and the Japanese supply group (Hoyo Maru, Iro). Three ships are in a source's list and not in rule 4: Tippecanoe, Worden and Tangier (fuelling and search groups); Toho Maru sailed with the Striking Force. Tippecanoe, Tangier and Toho Maru have no class on file; Worden is a Farragut and derives as Farragut does.

What the two runs say in general:

- **With surface weapons modelled,** almost every ship fails on something no Coral Sea row was asked for: rounds per gun (missing for most American classes, Myoko, Yubari and the old Japanese destroyers), torpedo tubes not preferred for the American destroyers and cruisers, main guns unconfirmed for the New Orleans and Portland classes, and no carrier has a `main_battery` fit. The existing lists meet these by `surface_weapons_not_modelled` or `main_gun_ammunition_not_limiting`. Whether this scenario models surface fire is the architect's choice, and it decides most of the table.
- **For the air figures alone** (the second run) the problems are few and named per ship below.
- **Search radar** is unknown for nearly every ship except the American carriers and Chicago; the existing lists treat unknown as none.
- **Anti-aircraft with no fit rows reads as 0 without a problem.** The tool allows an empty slot (`resolve_slot` with blanks allowed), so a class with no `heavy_aa` or `light_aa` rows derives an anti-aircraft figure of 0 and says nothing. Shoho was such a ship before this pass. The architect may want a list to name such ships.

## Part 1: the table

"Derived today" names the list and class id where the ship itself is derived. "Air-only problems" is what the second run reports; "Surface problems" is what the first adds.

### Allied ships

| Ship | Class row | Derived today | Air-only problems | Surface problems |
| --- | --- | --- | --- | --- |
| Lexington | us-lexington-class (1032) | no | heavy AA: class row 235 (8 x 5-inch/38, navweaps, valid "1942") cannot be placed on 6 May; it is the refit Lexington never received (fits 1200 to 1202, Bennett). Her 5-inch/25 are in the class's `secondary_battery` slot (row 239), which the anti-aircraft rule does not read. A ship `heavy_aa` row for her 12 x 5-inch/25 on the date is a gap | 8-inch guns: ship rows now say 0 (1200 preferred, 1201, 1202), but 1202 is dated "1942" and cannot be placed; no rounds per gun; no torpedo-tube row (she had none: a gap) |
| Yorktown | us-yorktown-class (1043) | no | none (class rows 277 to 279 and 511 to 513, ship rows 1215 to 1222) | no `main_battery` fit (a carrier); no rounds; no tubes row |
| Chester | us-northampton-class (1036), tied in this pass | no (Chicago, same class, savo 1012) | 20 mm: ship row 1304 (13, Navypedia, valid "1942") cannot be placed on 6 May | tubes disputed (class rows 257, 258, 327, 328 and ship row 1309) |
| New Orleans, Minneapolis | us-new-orleans-class | eastern_solomons 2016 | none | main guns unconfirmed (row 246) |
| Astoria | us-new-orleans-class | savo 1011 | 20 mm class row 532 dated "1942" cannot be placed | main guns unconfirmed |
| Portland | us-portland-class | eastern_solomons 2019 | none | main guns unconfirmed (266); no rounds; no tubes row |
| Chicago | us-northampton-class | savo 1012 | none | tubes disputed |
| Farragut, Worden | us-farragut-class | eastern_solomons 2012 | none | no rounds; tubes unconfirmed (226) |
| Dewey, Monaghan, Aylwin | us-farragut-class | no (class is, in eastern_solomons 2012) | none | as Farragut |
| Phelps | us-porter-class | eastern_solomons 2018 | none | no rounds; tubes unconfirmed (263) |
| Perkins | us-mahan-class (1033) | no | speed, endurance and its speed, fuel and standard displacement all disputed (rows 416 to 426, 858 to 861, 945 to 948); 5-inch/38 row 547 (to "1942") and 20 mm row 549 (from "1942") cannot be placed | no rounds; tubes unconfirmed (243) |
| Sims, Anderson, Hammann, Russell, Walke, Morris | us-sims-class (1040) | no | speed, endurance and its speed, fuel, displacement disputed (522 to 532, 871 to 874, 958 to 962); heavy and light AA disputed (568, 570) | no rounds; tubes unconfirmed (272, 273) |
| Australia | uk-county-kent-class | no (Canberra, same class, savo 1006) | 2-pounder row 200 disputed | none |
| Hobart | uk-leander-modified-class (1024) | no | none | no rounds; tubes unconfirmed (209) |
| Neosho | us-cimarron-class (1068) | no | the class type `oiler` has no simulation kind; no endurance, its speed or fuel in long tons (fuel is a note in barrels, 2526); speed disputed (2516 to 2519) | no guns, rounds or tubes rows |

### Japanese ships

| Ship | Class row | Derived today | Air-only problems | Surface problems |
| --- | --- | --- | --- | --- |
| Shokaku, Zuikaku | jp-shokaku-class | eastern_solomons 2007, 2006 | none | no `main_battery` fit (a carrier); no rounds; no tubes |
| Myoko, Haguro | jp-myoko-class (1011) | no | endurance disputed: 7,500 nm at 14 kn (Navypedia, 2654, 2655) against 8,000 nm at 14 kn (Wikipedia, 2656, 2657) | no rounds; no tubes row |
| Akebono, Ushio (Striking Force), Sazanami (Main Force) | jp-fubuki-class (1004) | no | 25 mm row 732 disputed | none |
| Ariake, Yugure | jp-hatsuharu-class | guadalcanal_landings 3015, 3014 | none | none |
| Shigure, Shiratsuyu | jp-shiratsuyu-class (1016) | no | none | none |
| Shoho | jp-zuiho-class (1067) | no | standard displacement disputed (2477 to 2479, metric or long tons); fuel disputed (2491, 2492). Anti-aircraft now has rows (fits 1250 to 1258, below) and reports no problem | no `main_battery`; no rounds; no tubes |
| Aoba, Kinugasa | jp-aoba-class | savo 1000, cape_esperance 3000 | none | none |
| Kako, Furutaka | jp-furutaka-class | savo 1001 (Furutaka also cape_esperance 3001) | none | none |
| Yubari | jp-yubari-class | savo 1005 | none | no rounds (savo reads her ammunition as not limiting) |
| Asanagi, Oite | jp-kamikaze-class (1007) | no | none | no rounds per gun (item 6 below: still a gap) |
| Mutsuki, Mochizuki, Uzuki, Yayoi | jp-mutsuki-class (1010) | no | endurance 3,600 nm and its speed 14 kn rest on unconfirmed rows (108, 109) | no rounds per gun |
| Tsugaru | jp-tsugaru-class (1070), new in this pass | no (not in the scratch list) | standard displacement disputed (2659 to 2661); no fuel row | no tubes row; mines are a note (2671, 2672) |

### Aircraft types (as of 1942-05-06)

| Type | Flown by | Derived today | Problems |
| --- | --- | --- | --- |
| A6M2, D3A1, B5N2 | Shokaku, Zuikaku; A6M2 also Shoho | eastern_solomons | none |
| G4M1 | 4th Air Group, Rabaul | rabaul_guadalcanal | none |
| SBD-3 | VS-2, Yorktown's VB-5 and VS-5 (unconfirmed rows 465, 466) | eastern_solomons | no radius on the date: the radius rows 280 to 287 are dated August 1942, 549 and 550 June; the range row 271 is disputed |
| F4F-3 | VF-2, VF-42 | no | range disputed (588, 589); pilot armour and self-sealing tanks rest on unconfirmed rows (593, 594) |
| SBD-2 | VB-2 | no | no range or radius, no bomb-load fit, no armour row |
| TBD-1 | VT-2, VT-5 | no | range unconfirmed (571); no torpedo-load fit; no armour row |
| A5M4 | Shoho | no | range unconfirmed (616); no armour or tanks rows |
| B5N1 | Shoho | no | no range, no torpedo-load fit; armour and tanks unconfirmed (618, 619) |
| G3M2 | Motoyama Air Group, Rabaul | no | no range, no bomb-load fit; armour and tanks unconfirmed (634, 635) |

That Shokaku and Zuikaku flew the B5N2 in May rests on undated low rows (69, 193) and on the navweaps order of battle of 7 December 1941 (600 to 605, this pass). No source read names the model for May 1942.

### Experience (decision 0064, as of 6 May 1942)

The four questions read `commissioned`, night practice in the three months before 6 May (from 6 February), and, for Japanese cruisers and destroyers, the navy row.

- **New ships (commissioned after 6 May 1941):** Shokaku (1941-08-08) and Zuikaku (1941-09-25), both preferred; Shoho (1941-11-30, unconfirmed, and her date is her conversion's).
- **No `commissioned` row:** Ariake, Yugure, Mutsuki, Oite, Uzuki, Yayoi, Tsugaru. Unconfirmed only: Farragut, Akebono, Ushio, Sazanami, Shigure, Shiratsuyu, Asanagi, Mochizuki, Shoho. Chester's is new in this pass (3702, DANFS).
- **Night practice inside the window:** Aoba, Kinugasa, Kako and Furutaka have preferred medium rows dated 1942-04-20 (1094 to 1097). Australia and Chicago have rows dated "1942-05" (1020, 1021), which straddle the day. No other ship has one.

## Part 2: the research

Five researchers read the web under this lead; every row they wrote was checked before it was merged (see "How the rows were checked"). Rows added: `fits.csv` 38, `ship_facts.csv` 47, `ship_class_facts.csv` 28, `ship_classes.csv` 1, `air_units.csv` 14, `air_unit_facts.csv` 56, `aircraft_types.csv` 6, `land_units.csv` 19, `land_unit_facts.csv` 61, `place_facts.csv` 12, `places.csv` 5, `lift_entities.csv` 1, `troop_lift_facts.csv` 16, `engagement_events.csv` 3, `sources.csv` 13, `gaps.csv` 53; and the `class` column of two ships was filled.

### 1. Lexington's guns and anti-aircraft, May 1942

- **8-inch guns removed at Pearl Harbor on 30 March 1942** (Bennett, Coast Defense Journal 2008, B; fit 1200, count 0). Navypedia says "4/1942" (1201) and navweaps "1942" (1202), the same value. Nothing replaced them: the planned 5-inch/38 twin mounts were never fitted.
- **1.1-inch:** Navypedia gives five quads from 1941 (1203) and seven quads from 4/1942 (1204); her own gunnery report names a mount No. 8 (now ship fact 3602, a note). How many on 6 May is open.
- **20 mm:** Navypedia, 32 from 4/1942 (1206). Her report and the Bureau of Ships damage report confirm 20 mm aboard on 8 May without a total (notes 3603, 3604).
- **5-inch/25 and .50 calibre:** both named in action on 8 May, no counts (notes). No ship-dated count of her 5-inch/25 was found: a gap, and it matters (Part 1).
- **Radar:** CXAM from 1940 (Navypedia, 1212); a search set in use on 8 May, unnamed (note); "2x Mk 4" fire-control in 4/1942 (1214, low, inferred).
- **Torpedo tubes:** no source says "none". Gap.
- Other: full load 41,000 tons on 8 May (Bureau of Ships, 3600); Navypedia's endurance 10,500 nm at 15 kn (2600, 2601) and fuel 3,600 t (2602) against the rows on file.

### 2. Shoho

- **Heavy AA: 8 x 12.7 cm/40 in four twin mounts.** Navypedia (Shoho's own line), combinedfleet, Nishida and Wikipedia agree (fits 1250, 1252, 1255, 1257).
- **Light AA: 8 or 12 x 25 mm.** combinedfleet: four twin mounts at completion, "probably upgraded" to four triple mounts at Yokosuka in April 1942 (1253, 1254); Navypedia's Shoho line: four triples (1251); Nishida and Wikipedia: eight (1256, 1258). For 6 May the two sources that speak of the date both say 12; the rows stand disputed for the architect.
- Aircraft capacity 30 (2650, 2651). Endurance 7,800 nm at 18 kn from Wikipedia (2652, 2653) agrees with the rows on file.
- **Gaps:** radar, machine guns and torpedo tubes (no source says none).

### 3. Yorktown in May 1942

Her own Coral Sea report gives, true on 8 May (high): 8 x 5-inch/38 (1215), four 1.1-inch quads (1216), 24 x 20 mm (1217), 18 x .50 calibre (1218) and CXAM (1219). Navypedia's "mid-1942" line (24 x 20 mm in place of 24 x .50 calibre, two Mk 4 sets) disagrees with her own count of .50 calibre guns on 8 May and stands disputed (1221, 1222; 1223 low). The Bureau of Ships report on her loss at Midway (June) is entered only for what it says of June (3601 and a note).

### 4. The Myoko class's endurance

7,500 nm at 14 kn (Navypedia's Nachi page, medium; 2654, 2655) and 8,000 nm at 14 kn (Wikipedia, low; 2656, 2657), whose text also says weight growth cut the design figure to 7,000 (2658, a note). Disputed. Nishida and combinedfleet give none.

### 5. Chester and Tsugaru

- **Chester:** her class is the Northampton class (DANFS 3700, high; Navypedia 3701), and `ships.csv` now says so. Commissioned 24 June 1930 (3702). Navypedia's Chester lines: 8 x 5-inch/25 (1300, inferred from the class line), four 1.1-inch quads from mid-1941 (1301), 13 x 20 mm from "1942" (1304), eight .50 calibre to 1942 (1305), CXAM from 1940 (1306), tubes gone in 1931 (1309, disputed with the class rows); and a 1942 line that removes two quads, read literally (in the note of 1301, not entered). Her own report confirms 20 mm and 1.1-inch fire and a radar in use on 8 May (3704, 3705).
- **Tsugaru:** a new class `jp-tsugaru-class` (id 1070, single ship, after Nishida, 3650), and `ships.csv` now says so. Standard displacement 4,000 t (Navypedia, Wikipedia) or 3,937 (Nishida, converted), disputed; trial 4,262; 20 kn; 9,000 nm at 10 kn; 4 x 12.7 cm/40 in two twin mounts; 4 x 25 mm (Wikipedia: 13 mm, disputed); 600 mines (Wikipedia's infobox: 500); one aircraft. Fuel and tubes are gaps.

### 6. Rounds per gun, Kamikaze and Mutsuki classes

**Still a gap.** Navweaps' page for the 12 cm/45 3rd Year Type prints the stowage as "N/A"; Navypedia, Nishida and Wikipedia give nothing. The 10th Year Type page gives 200 to 250 rounds, for a different gun, and was not entered. The US Naval Technical Mission to Japan's ordnance reports were not read; they are the next place to look.

### 7. Port Moresby in May 1942

- **Garrison:** no figure for May was found. The latest is "3,000 men" in January 1942 after the 30th Brigade's 39th and 53rd Battalions landed on 3 January (Milner, B; 1752, and `place_facts` 951); before that 1,088 on 8 December 1941 (McCarthy, A; 1750) and about 1,200 in March 1941 (1751). The 49th Battalion had been there since March 1941. Support: the 13th Field Regiment, the 23rd Heavy Anti-Aircraft Battery (four 3.7-inch at Tuaguba Hill and three mobile 3-inch stations, March), the Paga battery of two 6-inch guns, the Papuan Infantry Battalion (30 officers and 280 men on 6 June, McCarthy; about 300 on 24 June, Milner). Commander: Major-General Morris. The 14th Brigade embarked at Townsville on 15 May, after the battle. About 5 to 7 May the command judged the force too small to hold the coast and prepared a line seven miles inland (Gillison, 1775). Ten new land units (1150 to 1159).
- **Port:** nothing found. Gap.
- **Airfields:** three fields (Seven Mile, Ten Mile and Three Mile, Gillison, low); Seven Mile in March with practically no hard standing; small fields with too little dispersal, often attacked (Bates); one of three bases whose runways took B-17s and B-26s fully loaded (Gillison). No runway length for May. Kila Kila (Three Mile) built 1933 (Pacific Wrecks).

### 8. The Port Moresby invasion convoy

- **Units:** Bates's appendix (A) names the South Seas Detachment under Horii (the 144th Infantry, a company of the 55th Cavalry, the 1st Battalion of the 55th Mountain Artillery, companies of the 55th Engineers, the 55th Transport Regiment and the 47th Field Anti-Aircraft Battalion), the greater part of the 3rd Kure SNLF and the 10th Construction Unit. Nine new land units (1180 to 1188).
- **Men:** no count aboard from an A or B source. The nearest is 4,886 officers and men for the Detachment on leaving Guam in January 1942 (MacArthur's Reports, B; 1803). Wikipedia's 5,000 and 500 are entered low and unconfirmed (1804, 1830, troop_lift 765). Gap.
- **Supply:** nothing found. Gap.
- **Transports:** the sources disagree and every row stands disputed: Bates names nine and lists "6 Army transports" with the Army units; combinedfleet's pages give ten or eleven by name ("five Navy and six Army transports" on Tsugaru's page); Milner says five; the USSBS interrogation Nav. 8 says five; MacArthur's Reports, six Army transports with the naval troops in six naval auxiliaries. Convoy speed 6.5 kn (MacArthur's Reports). Five names in combinedfleet's lists (Asakasan Maru and four others) have no ship entity; they are in `gaps.csv`.
- **Dates:** X-day dawn on 10 May (Horii's orders of 29 April in Milner; MacArthur's Reports; Wikipedia). Embarkation began 2 May; sailed from Rabaul 4 May at 1600 (combinedfleet); began withdrawing at 0700 on 7 May; dissolved at Rabaul on 9 May. The landing places disagree (Milner: a beach 7.5 miles north-west of Port Moresby; MacArthur's Reports: south-east of Pari Mission); both disputed.

Decision 0188 rule 39 struck the planned date for want of a row; there are now three that agree.

### 9. Squadron strengths and the torpedo bomber model

- **Lexington:** no table of aircraft on hand was found in her report, the CTF 17 report or its enclosures. Rows 606 to 610 give what she launched on 7 May (VF-2 18, VT-2 12, VB-2 18, VS-2 17), lower bounds, medium.
- **Shoho:** Navypedia's air group table gives, for May 1942, 6 A5M4, 6 A6M2 and 9 B5N (611 to 613, written by this lead from the page, disputed with 485 to 488).
- **Shokaku and Zuikaku:** nothing better than Wikipedia's Lundstrom figures on file. Bates's table of planes available on the morning of 8 May, read from the page image of printed page 81 by a researcher, gives Task Force 17 121 (32 VF, 33 VS, 35 VB, 21 VT) and the Striking Force 122 (40 VF, 39 VB, 43 VT); his text gives 144 for the same force. These are force totals for 8 May and are recorded only in `gaps.csv`; they were not checked against the image by the lead.
- **Torpedo bomber model:** no source names it for May 1942; navweaps' Pearl Harbor order of battle has B5N2 for both carriers on 7 December 1941 (600, 603). Gap.

### 10. Land-based air (lower priority)

- **Allied:** Bates's Table 1 of Allied shore and tender aircraft as of 1 May (printed p. 22a), read from the page image by a researcher and checked against the image by the lead: Port Moresby 50 P-39 (8th Fighter Group, two squadrons); Charters Towers 19 B-25, 19 A-24, 14 A-20 (3rd Bombardment Group); Townsville area 12 B-25 and 80 B-26 (22nd), 50 P-39 (8th), 4 F-4; Cloncurry 48 B-17 (19th); Darwin 90 P-40; Sydney 100 P-39; Tulagi 6 RAAF PBY-5, withdrawn 2 May; 498 in all; the table's note says no attempt was made to tell operational aircraft from those under maintenance. Gillison and Craven and Cate put 19 A-24 of the 8th Squadron at Port Moresby on 7 and 8 May (disputed with the table's Charters Towers). Craven and Cate: the 19th Group averaged no more than 17 B-17s ready in May and June. No. 75 Squadron RAAF's figures are of March; no source puts it at Port Moresby on 6 May. Fourteen new air units and six aircraft types (identity only).
- **Japanese:** a new air unit for the Support Force's seaplanes at Deboyne (12, MacArthur's Reports; Pete or Dave, Bates); Bates's text puts "at least 12" flying boats at Lae on 4 May, against his own table's 3, 3 and 6 at Rabaul, Shortland and Tulagi (rows 501 to 505), disputed.

## Disputes for the architect

New in this pass, beyond those the first pass listed: Lexington's 1.1-inch mounts (5, 7 or 8) and the date of her 20 mm; Yorktown's .50 calibre (her own 18 on 8 May against Navypedia's removal of all 24 in "mid-1942"); Shoho's 25 mm (8 or 12); the Myoko class's endurance; Tsugaru's displacement, light AA and mines; Chester's 1.1-inch (four quads, or two after a 1942 removal); the Lexington class's endurance and fuel and the Yorktown class's displacement and fuel (Navypedia against rows on file; 2600 to 2604, entered as alternative or disputed only because the format allows nothing else beside a preferred row, not as a ruling); the number and names of the transports; the landing place; the A-24s' base; the Tulagi Catalinas (6 or 3); the B-17s (48 on hand or about 17 ready); the flying boats on 4 May; the Papuan Infantry Battalion's strength.

## What is still a gap, and where to look

- Lexington's 5-inch/25 count and any heavy AA on the date, and a "no tubes" row: her Bureau of Ships plans, Friedman's US Aircraft Carriers, NavSource (refuses scripts).
- Rounds per gun for the 12 cm/45 3rd Year Type, and for Yubari, Myoko and the American classes: the US Naval Technical Mission to Japan's ordnance reports; the Bureau of Ordnance's ammunition allowances.
- Shoho's radar and machine guns; the B5N model in May: the Japanese Monographs and Senshi Sosho; Lundstrom.
- The convoy's men and supply and what each transport carried: Japanese Monographs 33 and 37 (not online as text) and 98 (a 137 MB image scan, not read); Senshi Sosho.
- Port Moresby's garrison in May, its port and its runways: McCarthy and Gill's later chapters, the Allied Land Forces' returns; the RAAF's airfield records.
- Squadron strengths by carrier: Lundstrom's The First Team; the page images of Bates's tables.

## How the rows were checked

The lead checked every new row against a fresh fetch of the page it cites, held in memory, by script: 259 fact and fit rows in all. 223 had their quoted fragment found in the page text (the AWM PDFs through their text layer). 29 had every figure found but not the fragment (the armament lines of Navypedia, Nishida, combinedfleet and Wikipedia, whose fit rows carry no fragment, and Bates's Table 1); the lead read those lines on the page, and read Bates's Table 1 on the page image itself. The four rows from Chester's report and one from Yorktown's Midway damage report rest on scans with no text layer; the lead ran OCR on them and found each quoted fragment. One Bates row matched once the page's double spaces were collapsed. The three rows the lead wrote itself (611 to 613) were read on the page. Not checked: the gap rows' notes, and the Bates page-81 totals (in `gaps.csv` only).

The lead changed rows before merging: twelve fit rows that named a gun or set and gave no count became `armament_note` or `radar_fit_note` ship facts or were dropped as duplicates (a ship fit row with no count would replace the class's slot on that day); researchers' `alternative` standings were set to `preferred` or `disputed` where nothing on file differed, and to `preferred` where `oob-check` found the value supported; three air rows whose value was not an aircraft type key were given the nearest type as an inferred, unconfirmed reading, and two rows in attributes air units do not have were filed as `equipped_with` and `based_at`; a row whose variant came from another source than the one cited was graded down; three ids that two researchers both took were renumbered before commit; a second copy of one source was dropped. Each such change is said in the row's note.

Two researchers saved fetched pages and PDFs to their working folders outside the repository, against their brief, to search and OCR them; nothing of that is committed, and the folders were removed after the check.

Not reached in this pass: NavSource (refuses scripts), Wikipedia and a navweaps forum for one researcher (refused), Japanese Monographs 33 and 37 (not online as text), Monograph 98 (image scan), Gill's volume, Lundstrom.

## What the list now derives (Nov 20, 2026)

`data/derivation/coral_sea.toml`, built to the architect's rulings (decision 0188, "Stage A as built" has every reading), derives as of 6 May 1942:

- **38 ships in 24 classes (4000 to 4023):** the two American carriers, the attack group, Fitch's four destroyers, the support group (Australia, Chicago, Hobart, Perkins, Walke), the Striking Force, the Main Force with Shoho, and Yubari. Surface fire is modelled for the support group, the Main Force's cruisers and Yubari only.
- **8 aircraft types (4100 to 4107):** F4F-3, SBD-3, A6M2, D3A1, B5N2, B5N1, G4M1, G3M2.
- **Experience** for 32 of the ships.

The research of this pass (ruling 17 and what the build turned up) added: the SBD-2's 1,000 lb bomb and the TBD-1's torpedo on 7 May from Lexington's report; a second source for the TBD-1's 378 nm with a torpedo (historyofwar); the B5N1's range on internal fuel and its torpedo (Pacific Eagles, low); the G3M2's range with a normal load and its loads from ONI 232 S of 1943 (a wartime estimate, for the Type 96 Mk 4, read as the G3M2); Lexington's twelve 5-inch/25 and Yorktown's 1.1-inch from Navypedia's armament lines; Chicago's 5-inch/25, 1.1-inch and 20 mm from Navypedia's Northampton page; the torpedo marks of the Leander, Mahan and Sims classes from NavWeaps with Navypedia's tube counts (the Sims class had eight from summer 1940); and Shoho's completion on 30 November 1941 from combinedfleet. Rows: `fits.csv` 1310 to 1323, `aircraft_type_facts.csv` 637 to 639, `ship_facts.csv` 3777, `sources.csv` 1539 to 1541.

**Still left out:** Neosho, Sims, Worden, Tsugaru, Tippecanoe, Tangier and Toho Maru; the Kamikaze and Mutsuki classes (placeholder destroyers); and three types, for want of a fact the rules read: the **SBD-2** (pilot armour; range with a load), the **TBD-1** (pilot armour) and the **A5M4** (armour and self-sealing tanks). So VB-2, VT-2, VT-5 and Shoho's six A5M4 have no type. Pacific Eagles gives the A5M4 746 miles on internal fuel against pwencycl's 500 (row 616); it was not entered, since a disputing row would change 616's standing. Also open: what the G3M2s and G4M1s carried against the support group on 7 May; a commissioning row of a second source for Farragut, Akebono, Ushio, Sazanami, Shigure and Shiratsuyu, without which their experience is not derived. `gaps.csv` has each, with where it was looked for. The Bureau of Aeronautics' characteristics sheets on the American Aviation Historical Society's site, which would settle the American types, refused the fetch.
