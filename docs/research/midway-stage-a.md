# Midway, stage A: what is derived, what is blocked, and the rows found

Read on 2026-11-20 and 21 for decision 0198, rules 23 to 25 and 35 to 37 as amended and confirmed. Part 1 is the table rule 23 asks for: each ship class, aircraft type, air group, land unit and base of the forces the decision puts in the scenario, and whether it is (a) already derived (its class or type is in a derivation list today), (b) derivable now from rows in `data/oob`, perhaps with a ruling the list must make, or (c) blocked, with the rows that are missing. Part 2 is the research for the blocks and what it found. Nothing is derived here, no derivation list or scenario is written, and no dispute is settled: those are the architect's.

## How the table was made

A scratch derivation list outside the repository named every warship of the forces below and twenty aircraft types, dated 1942-06-03, and was run with `oob-derive --check` twice, before and after this pass's rows were merged: once with surface weapons modelled, once with every ship in `surface_weapons_not_modelled` and `radar_unknown_means_none`, as the Eastern Solomons and Coral Sea lists have their carriers and screens. The list had no `[class_ids]` table, which the tool also reports; that is the architect's to write.

The forces are those of `engagement_forces.csv` for `midway` that decision 0198 puts in: Task Forces 16 and 17 (Gwin, who joined Yorktown on 5 June, included); the First Carrier Striking Force with its Supporting Group and screen; the Transport Group's close screen (Jintsu and Destroyer Divisions 15, 16 and 18) and its transports; the Close Support Group (Cruiser Division 7 and Destroyer Division 8); the Second Fleet's main body (Atago, Chokai, Myoko, Haguro, Kongo, Hiei, Yura, Destroyer Divisions 2 and 9), and Zuiho with Mikazuki, whom the sources put either with that body or with the Aleutians Support Group (dispute 9 below). Left out by the decision or not named in it: the Main Body with Yamato, the Aleutians forces, the Seaplane Tender Group, the submarines, the minesweepers, the oilers of both sides (TG 16.6, the Mobile Force's supply group) and the auxiliaries at French Frigate Shoals and Pearl and Hermes Reef.

"A ruling" below means that rows are on file but none can be read without the list naming one: a row that is disputed, unconfirmed, dated another day, or dated so that the tool cannot tell whether it holds. The rows are named. "Blocked" means that no row exists for a fact the tool needs, or that the tool has no rule for the type.

What the runs say in general:

- **For the air figures** (the second run) the problems are few and named below. Most classes derive with none.
- **With surface weapons modelled,** nearly every American class lacks rounds per gun and has unconfirmed tubes, as at the Coral Sea; the existing lists meet that with `surface_weapons_not_modelled` and `main_gun_ammunition_not_limiting`. The Japanese surface ships that matter here (the Second Fleet's main body and the Close Support Group, the night threat of rule 35) are given by class below.
- **Search radar** is unknown for nearly every ship but the American carriers; the existing lists treat unknown as none.
- **Times.** Every row keeps its source's clock; none was converted (rule 11 asks the hand calculation to do that). The Nagumo report and Japanese Monograph 93 are Tokyo time and date the American 4 June as 5 June.

## Part 1: the table

### Ship classes

| Class | Ships | Before | After | What the list needs |
| --- | --- | --- | --- | --- |
| Yorktown (us-yorktown-class) | Enterprise, Yorktown | a (eastern_solomons, coral_sea) | a | nothing for the air figures |
| Yorktown, Hornet | Hornet | b: no class in `ships.csv` | b | `class` now filled from row 8805. Ruling: her 20 mm on the date, fits 6507 (24, dated "1942", cannot tell) and 6509 (30, disputed) |
| Pensacola (us-pensacola-class) | Pensacola | b: no class in `ships.csv` | b | `class` now filled (rows 12830, 12831). Rulings: aircraft capacity 2 or 4, rows 1012 to 1015 all disputed; light AA rows 678 and 679 dated "1942", which the tool cannot place on 3 June |
| Northampton | Northampton | a (coral_sea, savo) | a | nothing |
| New Orleans | New Orleans, Minneapolis, Vincennes, Astoria | a | a | surface only: main guns unconfirmed (246) |
| Atlanta | Atlanta | a (eastern_solomons) | a | surface only: no rounds |
| Portland | Portland | a | a | surface only: main guns unconfirmed (266), no rounds, no tubes row |
| Farragut, Porter, Mahan, Benham, Gridley, Sims, Gleaves | Worden, Monaghan, Aylwin; Phelps, Balch; Conyngham; Benham, Ellet; Maury; Hughes, Anderson, Hammann, Russell, Morris; Gwin | a | a | surface only: rounds missing, tubes unconfirmed, as in the Coral Sea list |
| Akagi (jp-akagi-class) | Akagi | b | b | Rulings on four disputes of conversions and sources: speed (7716 to 7718: 31.2 or 31.5), fuel (7725, 7726: 6,000 or 5,905, metric and long tons), endurance (7727, 7728: 8,200 or 10,000), standard displacement (7700 to 7702: 36,500 or 35,924) |
| Kaga | Kaga | b | b | Rulings: speed (7758 to 7760), endurance speed (7770, 7771: 16 or 15), fuel (7767, 7768), displacement (7742 to 7744) |
| Soryu | Soryu | b | b | Rulings: speed (7796 to 7798), fuel (7802, 7803), displacement (7784 to 7786). Surface only: no main battery, rounds or tubes row |
| Hiryu | Hiryu | b | b | Rulings: speed (7829 to 7831), fuel (7835, 7836), displacement (7817 to 7819). Surface as Soryu |
| Kongo | Haruna, Kirishima; Kongo, Hiei | a (eastern_solomons) | a | surface was blocked (no rounds, no tubes row); now clean: rounds 80 per gun (13105), no tubes after the rebuild (fits 13014, 13015, inferred from the armament lists) |
| Tone | Tone, Chikuma | a | a | surface: tubes and rounds now on file (fits 13003 to 13005, rounds 13102); blocked on the torpedo model, which no source read names (the fits use the new generic `jp-61cm-torpedo`, which has no warhead row) |
| Mogami | Kumano, Suzuya, Mikuma, Mogami | a | a | surface: as Tone (fits 13000 to 13002, rounds 13100) |
| Myoko | Myoko, Haguro | a (coral_sea) | a | surface: rounds now on file (13101); tubes are a dispute (number 10 below) |
| Takao | Atago, Chokai | a (savo) | a | nothing, either run |
| Nagara | Nagara, Yura | a | a | surface: tubes now on file (13009, 13010) but the model is generic; rounds per gun a gap (NavWeaps gives none for the 14 cm/50) |
| Sendai | Jintsu | b | b | Ruling: endurance now a dispute, 5,000 nm at 14 kn (168, 169, Wikipedia) or 6,000 at 15 (12800, 12801, Navypedia). Surface: rounds a gap |
| Kagero | twelve of the screens | a | a | nothing |
| Yugumo | Kazagumo, Makigumo, Yugumo | a | a | surface: tubes now on file (13011 to 13013); rounds "about 150" unconfirmed (13106) |
| Asashio | Arare, Kasumi, Arashio, Asashio, Michishio, Oshio, Asagumo, Minegumo, Natsugumo | b, with no problem for the air figures | b | surface: rounds "about 150" unconfirmed (13107) |
| Shiratsuyu | Harusame, Murasame, Samidare, Yudachi | a | a | nothing |
| Zuiho (jp-zuiho-class) | Zuiho | b: no class in `ships.csv` | a | the class is the Coral Sea list's Shoho (4020); `class` now filled (12832, 12833); no problem for the air figures |
| Mutsuki | Mikazuki | b: no class in `ships.csv` | b | `class` filled from row 9284. Ruling: endurance 3,600 nm at 14 kn rests on unconfirmed rows 108, 109. Surface: rounds a gap |
| none | Patrol Boats 1, 2 and 34 | c | c | no class on file: old destroyers converted, no class row or facts gathered |
| none | the 13 transports | c | c | no class; a marked placeholder by rule 7 and 33. Rows found for their tonnage and loads are in Part 2 |

Counted by class (28 classes, the patrol boats and transports apart): before, 21 derived, 7 derivable with rulings, 0 blocked for the air figures; after, 22, 6 and 0 (Zuiho's class is the Coral Sea's). For surface fire, the Japanese classes blocked before were Kongo, Tone, Mogami, Myoko, Nagara, Yugumo and Sendai on rows; after, Kongo is clean, Myoko and Yugumo need a ruling, and Tone, Mogami, Nagara and Sendai are still blocked (the torpedo model, and for Nagara and Sendai the rounds).

### Aircraft types

| Type | Flown by | Before | After | What is missing or needs a ruling |
| --- | --- | --- | --- | --- |
| F4F-4 | VF-3, VF-6, VF-8 | a | a | nothing |
| F4F-3 | VMF-221's seven | a (coral_sea) | a | nothing |
| SBD-3 | VB-3, VS-5, VS-6, VB-6 in part, VB-8, VS-8 | a | a | Ruling: no radius holds on 3 June (rows 549, 550 are of 4 and 5 June; 280 of August), as the Coral Sea list met by `combat_radius_row` |
| SBD-2 | VMSB-241's SBD-2s, part of VB-6 | a (coral_sea) | a | Ruling: radius 180 is dated 7 May (647) or 4 June (12203); the 500 lb bomb fits are dated 4 and 5 June (12203 to 12205) |
| TBD-1 | VT-3, VT-6, VT-8 | a (coral_sea) | a | Ruling: radius 180 of 7 May (648), or 175 and 155 of 4 June from Bates (12210, 12211) |
| TBF-1 | VT-8's detachment at Midway | a (eastern_solomons) | a | Ruling: radius rows unconfirmed or of other dates (288, 309 to 312, 551 to 555); the torpedo is dated 4 June (12206; "one torpedo", the mark inferred) or unconfirmed (52, 409) |
| F2A-3 | VMF-221's 21 | b | b | Rulings: range 839 a lone disputed row (5722); self-sealing tanks two grade C rows (5725, 12207) |
| SB2U-3 | VMSB-241's SB2U-3s | c: no armour or tank rows | b | Now on file: radius 486 nm with a 1,000 lb bomb (12200), armour (12206), tanks "no" (12205, inferred), the 500 lb bomb on 4 June (fits 12200, 12201). Ruling: the 500 lb bomb's weight rests on one unconfirmed row (weapon_facts 367) |
| B-17E | the Army's seventeen | b: armour and tanks unconfirmed | b, clean | Armour and tanks now preferred (12010, 12011, Air Force Magazine, B); it derives with no problem |
| B-26 | the four of the 18th and 69th Squadrons | c | c | The tool maps no simulation role to `medium_bomber` (aircraft_types.csv 1032): a rule, not a row. Radius 156 or 130 nm flown on 4 June (12016, 12017, disputed) or a disputed range (5765, 5775); armour and tanks only grade C (5756, 5757, 12014, 12015). They were the original B-26 (12018) |
| PBY-5A | VP-44 and part of Midway's 16 | c: no armour row | b | The Navy's characteristics sheet for the PBY-5A (12000 to 12004, fits 12000 to 12002, high) gives patrol range 2,211 nm, patrol radius 575 and bombing radius 610, armour yes, tanks partial; it is dated 1 September 1943 and is read for June 1942 only by exception. Its range disputes Wikipedia's 2,190 (215), and its tanks Wikipedia's "yes" (487). The 700-mile searches are rows 12007, 12008; the night torpedo attack about 434 nm out, 12009 |
| PBY-5 | VP-23 and VP-11, 14 in all | c: no armour row | b, weakly | Armour and tanks are one grade C page each (12005, 12006, dated "1942-06", which the tool cannot place); radius only a disputed range (314). No sheet for the flying boat was found: the architect may read the PBY-5A's for her |
| A6M2, D3A1, B5N2 | the four carriers | a | a | nothing |
| D4Y1-C | Soryu's two | c | c | The tool maps no simulation role to `reconnaissance` (6102). Rows now exist but are all low: range 849 (12212), no armour, no self-sealing tanks (12213 to 12216) |
| E13A1 | Tone's and Chikuma's | a | a | nothing |
| E8N2 | the battleships' and cruisers' Type 95s | c | c | No armour or tank row; range 485 or 484 (5819, 12217 to 12219: the same figure rounded two ways, now disputed) |
| A5M4, B5N1 | Zuiho's group, if Zuiho is in | a (coral_sea) | a | nothing; which types Zuiho carried is dispute 9 |

Counted by type (20): before, 12 derived, 2 derivable, 6 blocked; after, 12 derived, 5 derivable (F2A-3, SB2U-3, B-17E, PBY-5A, PBY-5), 3 blocked (B-26 and D4Y1-C on the tool's roles, E8N2 on rows). The flying boats of rule 36 are now derivable, on the architect's reading of the 1943 sheet.

### Air groups

| Group | What is on file | Standing |
| --- | --- | --- |
| Yorktown's VF-3, VB-3, VS-5, VT-3 | 25 F4F-4, 18 SBD-3, 18 SBD-3, 12 or 13 TBD-1 (5701 to 5718); VB-3 "SBD-3 and 3A" (its report, `midway1942-vb3-report`) | derivable; VT-3's 12 or 13 is a dispute, and the order of battle compilation's 27 VF-3 and 15 VT-3 (12628, 12634) dispute them |
| Enterprise's VF-6, VB-6, VS-6, VT-6 | 27 F4F-4 (12610, the group total read as the squadron), 18 SBD-2 and SBD-3 mixed (12600, 12601), 18 SBD-3 (12606, 12607), 14 TBD-1 (5727, 12618) | derivable; VB-6's split of models is not given (its losses were 6 SBD-2 and 5 SBD-3) |
| Hornet's VF-8, VB-8, VS-8, VT-8 | 27 F4F-4 (12641), 35 or 37 SBD for the two bomber squadrons (12639 against 5724), 15 TBD-1 (5739) | VB-8 and VS-8 not split on hand: 17 each flew the first strike (12642, 12643, lower bounds); the compilation's 18 and 16 (12624, 12622) are low |
| Midway: VMF-221 | 21 F2A-3 and 7 F4F-3 on hand, three sources agreeing (5748, 5749, 12646, 12647); 19 and 6 operational at 0615 on 4 June (12662, 12663, derived) | derivable |
| Midway: VMSB-241 | 19 SBD-2 and 17 SB2U-3 (Bates, 5750, 5751; the XO's report agrees, 12660) against 18 and 16 (the CO's report, 12648, 12649) | a dispute (number 6) |
| Midway: VT-8 detachment | 6 TBF-1 (5802, 5803) | derivable |
| Midway: patrol squadrons | 14 PBY-5 and 16 PBY-5A in all (Bates, 5753, 5754); by squadron VP-44 12 PBY-5A (from the XO's report), VP-23 8 PBY-5 and VP-11 4 PBY-5 (the NAS diary), against the compilation's VP-23 14, VP-24 3, VP-44 8, VP-51 3 (12672 to 12682) | by type, derivable; by squadron a dispute, and the reports of VP-23, VP-44 and Patrol Wing Two are a gap |
| Midway: Army bombers | 17 B-17 on 31 May (5755, 12686), arriving by squadron 6, 6, 2 and 1 (5808 to 5815), 6 more on 4 June (12690); 4 B-26 (5756, 5816, 5818) | by type, derivable; by squadron after arrival, a gap |
| The four Japanese carriers | Bates's total 234 = 90, 72, 72 (5864 to 5867) is firm; by carrier and type three readings (dispute 1); the Nagumo report's floors of planes operational in the morning's strike and stand-by (12400 to 12411, high) | a dispute |
| 6th Air Group's Zeros | dispute 2 | a dispute |
| Floatplanes of the battleships and cruisers | establishments from Japanese Wikipedia only (12466 to 12488, low): Tone and Chikuma 3 E13A1 and 2 E8N2 each, the battleships 3 Type 95 each, the heavy cruisers 3 or 4, the light cruisers 1 each; Chikuma had 4 operational at 1755 on 4 June (12412) | unconfirmed; the type split for the heavy cruisers and the Type 94 (E7K2) of the light cruisers have no rows |
| Zuiho's group | dispute 9 | a dispute |

### Land units and bases

| Unit or base | What is on file | Standing |
| --- | --- | --- |
| Japanese landing force | 2,500 (Heinl, 6934), 5,800 (Japanese Monograph 93, 6935), about 5,000 (Wikipedia, troop_lift 5802; combinedfleet's Zenyo Maru page, land 12803 and troop_lift 12800) | dispute 3, for the rule for disagreements (rule 47) |
| Its parts | the 2nd Combined SNLF 1,500, 2,000 or 2,800 (6948, 6947, 6946); the Ichiki Detachment 1,000, 2,507 or about 3,000 (6941, 12800, 6942 and 12801); the 11th and 12th construction units, no strength (650 of the 12th aboard Kano Maru, 12810) | disputes |
| Men in each transport | only Argentina Maru 816 of the Kure SNLF and Kano Maru 650 (ship facts 12802, 12809); which unit was in which ship for ten transports (units_embarked_note, ship facts 12800 to 12811) | a gap for the rest |
| The transports | gross tonnage of nine (ship facts 12812 to 12822; two disputed), Army or Navy status of seven (12823 to 12829); no speed for the convoy or any ship (the 8.8 kn of rule 30 rests on positions, not on a stated speed) | a placeholder class, as planned |
| Midway's garrison | 2,178 in CINCPAC's estimate (6907; the 6th Defense Battalion 1,899 and the raiders 279, place 6024, 6025); a popular history's 2,138 Marines, about 1,000 Navy, 374 Marine air and 120 Army on 25 May (land 12812 to 12815, place 12800, 12801, grade C) | the total ashore is a dispute and the June total of all services a gap |
| Midway (Sand and Eastern Islands) | runways, seaplane base, fuel, ammunition and port notes (place facts 6000 to 6031) | enough for a base; its stocks are a gap |
| Wake | name and position only (places 6104, positions 5107) | blocked: its airfield, aircraft and garrison were not searched (a gap row was added). Decision 0199 makes it the Japanese base on the map |

## Part 2: the research

Six researchers read the web under this lead, each writing staged rows and saving every page it cited outside the repository; the lead checked every quotation by script against the saved copy (294 rows: 289 found at once; four Bates rows found once the scan's line breaks were collapsed; one PBY-5A figure read from the sheet's image, as its note says) and merged the rows. Rows added: `air_unit_facts.csv` 190, `air_units.csv` 13, `aircraft_type_facts.csv` 39, `engagement_forces.csv` 2, `fits.csv` 26, `land_unit_facts.csv` 16, `place_facts.csv` 2, `ship_class_facts.csv` 10, `ship_facts.csv` 34, `sources.csv` 17, `troop_lift_facts.csv` 3, `weapons.csv` 1 (`jp-61cm-torpedo`, the generic 61 cm torpedo for a fit whose model no source names), `gaps.csv` 57. The `class` of Hornet, Pensacola, Zuiho and Mikazuki in `ships.csv` was filled from preferred `ship_class` rows.

1. **The PBY.** The find is the Navy's "Airplane Characteristics and Performance" sheet for the PBY-5A of 1 September 1943 (`buaer-acp-pby5a`, A; it supersedes one of 13 August 1942 that was not found): patrol range 2,545 statute miles at 117 mph, patrol radius 575 nm and bombing radius 610 nm at 20 per cent reserve, crew armour, and 603 of 1,478 gallons in protected tanks. The ONI narrative gives the 700-mile searches and the night attack about 500 miles out. No sheet for the PBY-5 flying boat was found; a grade C page says most PBY-5s had armour and partly protected tanks by mid-1942.
2. **The other American types.** SB2U-3: Vought's heritage pages (radius 560 miles with a 1,000 lb bomb, search range 2,640 miles with the centreline tank, the added wing tanks unprotected), the Naval Aviation Museum ("more armor protection"), and Bates for the 500 lb bomb of both Marine bomber types on 4 and 5 June. B-17E: Air Force Magazine for armour and self-sealing tanks. B-26: its distance flown and its model; armour and tanks only from grade C pages. F2A-3: only a grade C page for its tanks and armour. TBF-1: Bates's "each armed with one torpedo".
3. **The Japanese carriers' air groups.** No source better than those on file gives aircraft on hand by carrier and type: the Nagumo report's tables of organization are blank in its transcription, the ONI translation's scan and Parshall and Tully were not reached, and combinedfleet's carrier pages did not answer. The Nagumo report's own strike and stand-by tables give floors of planes operational. Japanese Monograph 93 and Wikipedia give the 6th Air Group's Zeros.
4. **The American squadrons.** The Enterprise, VB-3, VB-6, VS-6, MAG-22 and VMSB-241 reports on midway1942.com give the squadrons above; the MAG-22 executive officer's daily record of aircraft gives the Marine and patrol squadrons by type and day.
5. **The landing.** combinedfleet's transport pages (reached at `https://www.combinedfleet.com/<Name>_t.htm`) gave which unit each transport carried, two loads in men, tonnages and the Ichiki Detachment's 2,507. Its `Brazil_t.htm` is another ship; the Midway transport is `Brasil_t.htm`, entered on `brazil-maru`. Bates, the monograph, Heinl and the ONI narrative gave no new figure for the landing force.
6. **Japanese surface fits.** Tubes for the Mogami, Tone, Myoko, Nagara, Yugumo and Kongo classes from Navypedia, Nishida and Wikipedia; rounds per gun from NavWeaps's gun pages for the 20 cm guns (Mogami, Myoko, Tone, Akagi, Kaga), the 36 cm (Kongo) and the 12.7 cm/50 (Yugumo, Asashio, "about 150", low as rows 2450 to 2452 are).

**Rows on file whose standing changed,** as `oob-check` required once the new rows were in, with a sentence added to each note: `air_unit_facts.csv` 269, 270, 274, 275, 5701, 5734, 5745, 5746, 5752 (unconfirmed to preferred); `aircraft_type_facts.csv` 488, 489, 490 (the B-17E's armour and tanks, to preferred), 215 (the PBY-5A's range), 5747 (the SB2U-3's range) and 5819 (the E8N2's range) to disputed; `ship_class_facts.csv` 168 and 169 (the Sendai class's endurance) to disputed. No list's generated files change: every list on file was derived again and `data/reference` is as it was.

## Disputes for the architect

1. **The carriers' aircraft by carrier and type** (fighters, bombers, attack planes): Wikipedia's order of battle, Akagi 24/18/18, Kaga 27/20/27, Soryu 18/16/18 and two D4Y1-C, Hiryu 18/18/18 (5821 to 5857, 12434 to 12437, with the 6th Air Group's Zeros in the fighters); Navypedia, Akagi and Hiryu 21/21/21, Kaga 21/27/18 (on file); Japanese Wikipedia's establishments with spares, 21/21/21 for each but Kaga 21/21/30 (12438 to 12450). The Nagumo report's floors (12400 to 12411) fit all of them but Soryu's 16 bombers (12403 against 5843). Bates's 234 = 90/72/72 (5864 to 5867) is the only total of a grade A source.
2. **The 6th Air Group's Zeros:** six in each of the four carriers, 24, and twelve in Junyo (Monograph 93, 12414 to 12424); Akagi 6, Kaga 9, Soryu and Hiryu 3 each, 21 (Wikipedia, 12425 to 12432, 5860, 5862); 21 with no split (Japanese Wikipedia, 12433).
3. **The landing force:** 2,500 (land 6934), about 5,000 (troop_lift 5802 and 12800, land 12803) and 5,800 (land 6935); and its parts (Land units above).
4. **Midway's garrison:** 2,178 (6907) or 2,138 Marines (land 12812) and the other services' 1,494 (land 12813 to 12815, place 12800, 12801).
5. **The PBY-5A:** range 2,211 (12000) or 2,190 (215); self-sealing tanks partial (12004) or yes (487); and for the PBY-5, partial (486, 12006).
6. **VMSB-241:** 19 SBD-2 and 17 SB2U-3 (5750, 5751, 12660) or 18 and 16 (12648, 12649); flown 16 and 11 (5783, 5784) or 18 and 12 (12650, 12651).
7. **Midway's patrol squadrons** by squadron (12669, 12672 to 12682 and the diary rows).
8. **Hornet's dive bombers:** 35 (12639) or 37 (5724).
9. **Zuiho:** with the Second Fleet's main body (engagement_forces 5941, Monograph 93; 12400, 12401, Wikipedia) or the Aleutians Support Group (Bates, 5831); her aircraft 6 A5M4, 6 A6M2 and 12 B5N2 (12452 to 12454), 12 A6M2 and 12 B5N2 (12458, 12459), or 6, 6 and 9 (12460 to 12462).
10. **The Myoko class's tubes:** 16 in four quadruple mounts from 1941 (fits 13006 to 13008) or 8 (ship_class_facts 1313, 1314, 1351).
11. **Smaller:** the SB2U-3's flown distance 165 (12201) or 180 (12202) and its search range 2,294 (12204) or 2,130 (5747); the B-26's flown distance 156 (12016) or 130 (12017); the E8N2's range 485 or 484 (5819, 12217 to 12219, one figure rounded two ways); the Sendai class's endurance (168, 169 against 12800, 12801); the Ichiki Detachment's size (land 6941, 12800, 6942, 12801); Nankai Maru's and Kano Maru's tonnage (two rows each); Haruna's search plane, Type 95 (5873) or Type 0 (12413) in the same report.

## Gaps that remain, and where they were looked for

- **Japanese carrier aircraft on hand by carrier and type:** the Nagumo report's Part II tables (HyperWar, blank; the NHHC page returned 404), the ONI translation of 1947 (not found), Parshall and Tully (not in hand), combinedfleet's carrier pages (404 or timed out). Next: an archive.org scan of OPNAV P32-1002, or the book.
- **The torpedo model** of the Mogami, Tone and Nagara classes in June 1942, and **rounds per gun** of the 14 cm/50 (Nagara, Sendai; NavWeaps prints none) and of the Mutsuki class.
- **The E8N2's armour and tanks,** a scouting range for the D4Y1-C and E8N2, and a source above grade C for either.
- **The PBY-5's own characteristics sheet,** and a 1942 date for armour and protected tanks on either PBY (alternatewars.com did not connect; aahs-online has no PBY-5 sheet; the Air Force museum's fact sheets refused).
- **The B-26's range or radius and its armour** from a source above grade C (the AAF test memo of January 1942 was found but its scan was too poor to cite).
- **The F2A-3's range** from a Navy source and a second source on its tanks; the Navy's sheets for the F2A-3, SB2U-3, SBD-2 and TBF-1 (404 on aahs-online).
- **VB-8's and VS-8's aircraft on hand,** and the model of the Hornet and VS-5 dive bombers.
- **The reports of VP-23, VP-44 and Patrol Wing Two** ("not now available", NAS Midway's report says), and the B-17s by squadron after they arrived.
- **The men in eleven of the transports,** the convoy's speed, the Yokosuka and Kure SNLF units' strengths, and pages for Toa Maru No. 2, Kirishima Maru and Seicho Maru.
- **Midway's garrison of all services in June,** and its stocks.
- **Wake** (above).
- **Patrol Boats 1, 2 and 34:** no class.

## What the data says against decision 0198

- **Rule 37 names "two models of flying boat ... the B-26, two Marine types ... a Japanese floatplane and one more" as new.** The Marine types are three (F2A-3, SB2U-3 and the SBD-2, which the Coral Sea list already derives) besides the F4F-3, and the Japanese floatplanes two (the E8N2, and the D4Y1-C, a carrier reconnaissance type); the E13A1 is already derived.
- **The B-26 and the D4Y1-C cannot be rated by any row:** the tool has no simulation role for `medium_bomber` or `reconnaissance`. That is a rule of `oob-derive` (decision 0029, 9), not a gap, and goes to the architect.
- **Rule 37's "four destroyers" new to the game** is one class here: the Asashio class (nine ships), which no list derives but which needs no new row for the air figures. Mikazuki's Mutsuki class is new to the game too, on unconfirmed rows.
- **Rule 39's garrison "some 2,200"** is CINCPAC's estimate of the Marines and raiders only; with the Navy, Marine air and Army men of the one source that counts them all (grade C) it is about 3,600.
- **Rule 30's 8.8 kn** for the transports is not stated by any source read; it stands on the positions alone, as the rule says.
- **Zuiho,** named in no force of the decision, sailed with the Second Fleet's main body by two sources and with the Aleutians Support Group by Bates.
- **Every American surface ship is of a class the game has** (rule 37) holds once Hornet's and Pensacola's classes are filled, which this pass did; Pensacola's class is derived by no list today.

## The second pass

Read on 2026-11-21 for decision 0198, rules 68 and 70, and the derivation list `data/derivation/midway.toml`. Three researchers read the web; the lead checked every quotation by script against the saved pages (all found) and merged the rows. Rows added: `fits.csv` 14000 to 14007, `ship_class_facts.csv` 14000, `air_unit_facts.csv` 14100 to 14109 and 14200 to 14224 (not 14208, a sum of the researcher's, dropped), `air_units.csv` 14100, `place_facts.csv` 14100 to 14117, `land_units.csv` 14100 to 14101, `land_unit_facts.csv` 14100 to 14110, `sources.csv` 14020 to 14022 and 14040, and 16 entries of `gaps.csv`. A fit row that joined two dated statements into a range of its own (Type 93 for the Mogami class from March 1942 to October 1944) was not merged: the range is no source's. No row on file changed standing.

What was found:

- **The torpedoes.** Mogami fired Type 93s at Sunda Strait on 1 March 1942 (combinedfleet, and Japanese Wikipedia; fits 14000, 14001), and Suzuya carried them at Samar in October 1944 (14002). Japanese Wikipedia gives the Nagara class, Abukuma apart, the Type 90 at the outbreak of war (14004, ship class facts 14000), against the interrogation's 8th Year Type for Nagara in November 1942 (1359). The Naval Technical Mission's Table I gives the four triple mounts of Tone, Chikuma, Suzuya and Mikuma, a mount type and no torpedo (14005 to 14007). Japanese Wikipedia confirms Jintsu's quadruple mounts of 1940-41 (14003). Each row keeps its source's date, and none holds on 3 June 1942 with a model named, so the Mogami, Tone and Nagara classes stay blocked on the model.
- **Wake.** One runway in operation and a seaplane base before the battle (Bates); the Chitose Air Group there from 31 May, 13 fighters and 15 attack planes of it with 9 of the Misawa group on 6 June, 8 of the 1st Air Group arriving that evening and 17 fighters and 50 attack planes on 7 June (Bates), 51 and 18 on 8 June (Japanese Monograph 93). The 65th Guard Unit was there in June (the monograph); about 2,700 men in December 1942 and 3,100 early in 1943 (Spennemann); 1,990 repatriated in 1945 (JACAR).
- **The carriers' aircraft.** combinedfleet's records of 27 May (B): Akagi 18 fighters, 18 bombers and 18 attack planes with six of the 6th Air Group; Kaga 18, 18 and two spare, 27, with nine of the 6th; Soryu 18, 16 and 18, with three of the 6th and one or two D4Y1. Nothing for Hiryu. With the 6th Air Group's fighters counted in, Akagi's and Kaga's are the readings of rule 50; Soryu's fighters would be 21. The ONI translation of the Nagumo report (OPNAV P32-1002) has no scan on archive.org; NHHC's text of Part II lists only the units of 5 June.
- **Zuiho.** combinedfleet gives six A5M, six A6M and twelve B5N on 29 May (B), the reading English Wikipedia gives; the Naval Historical Society of Australia a total of 24; Pacific Wrecks 12 A6M2 and 11 D3A (C, the type apparently wrong).

What stays a gap:

- The torpedo model of the Mogami, Tone and Nagara classes on 3 June 1942.
- Rounds per gun of the 14 cm/50 (Nagara and Sendai classes) and the 12 cm/45 (Mutsuki class): NavWeaps prints none, nor do Japanese Wikipedia, combinedfleet or Navypedia; the Naval Technical Mission's O-47 reports were not found online.
- Wake's runway in June 1942 (lengths are only for later or undated states), its fuel and stores, its garrison in May to June 1942, the types of the aircraft based there, and any flying boats.
- Hiryu's aircraft by type before 4 June, a primary record by carrier, and Zuiho's in her unit's own report.

The list is in `data/derivation/midway.toml`. Its header says what is in it, what is left out and why; its exceptions are at the end of `data/derivation/rules.toml`.
