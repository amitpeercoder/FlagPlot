# Outside answers of 14 November 2026, checked against their sources

Status: research report for the architect, read on 2026-11-14. Nothing here is settled. Every new row is `unconfirmed`, except where `oob-check` requires another standing; those cases are listed under "Standing set by the checker".

On 14 November 2026 an outside research pass answered seven prompts: verdicts and time away, land units, construction and shore work, transports, marches, preparation, and submarines. Its answers are tables of statements. Each statement names a source, a page and a quoted sentence. A statement is a lead, not a source. The answer files stay outside the repository.

## How it was read

One reader (a Sonnet agent) was assigned to each answer file. For every statement, the reader:

- fetched the cited document with `curl` and saved it;
- converted it to text (HTML with the standard library, PDF with `pdftotext`; three image-only items by OCR);
- searched the text for the quoted words;
- read the passage around them;
- said whether the document says what the statement says.

Where the cited address refused (marines.mil, usmcu.edu, army.mil and history.navy.mil often do), the reader tried the same document elsewhere: the HyperWar copy or the Wayback Machine. A statement whose document could not be read anywhere was dropped.

Each reader could also propose statements found on the same saved pages that answer the prompt's questions. Those rows are marked "Found beside a lead". Rows from the statements themselves are marked "Lead from an outside research pass".

**The quote check.** The lead ran a second check by program on all 258 staged lines. Each quote was looked for in the reader's saved text, allowing only whitespace, typography and hyphens at a line break. 255 lines passed. The other three were the dropped statements, which have no text.

The lead also looked at two things by eye:

- the four quotes that the check passed only once soft hyphens at line breaks were allowed;
- the map label from Miller's Map XXI, which only an image holds.

**Image-only sources.** The ONI *Index of U.S. Fleet* (1944) has no text layer. It was read by OCR at 300 dpi, and the key lines were checked by eye against the page images.

**What the check does not show.** It does not show that a figure was filed against the right unit, ship or date. That filing is the readers'. Each note says what the passage around the quote says.

## Counts

A "statement" is one row of an answer's table. "Verified" means the source says what the statement says, including statements whose page, chapter or author needed correcting. "Corrected" counts those. "Already held" means the same source was already a row in the database.

| File | Statements | Verified | Corrected | Already held | Dropped | Rows added (statements and beside) |
| --- | --- | --- | --- | --- | --- | --- |
| 15 Verdicts and time away | 5 | 5 | 2 | 0 | 0 | 0 and 11 |
| 16 Land units | 13 | 13 | 1 | 2 | 0 | 32 |
| 17 Construction and shore work | 22 | 19 | 10 | 7 | 3, not reachable | 32 |
| 18 Transports | 13 | 13 | 3 | 0 | 0 | 37 |
| 19 Marches and positions | 16 | 16 | 4 | 5 | 0 | 27, and 10 positions |
| 20 Preparation | 17 | 17 | 5 | 2 | 0 | 32 |
| 21 Submarines | 17 | 17 | 3 | 0 | 0 | 13 |
| **All** | **103** | **100** | **28** | **16** | **3** | **194** |

**What was dropped.** All three dropped statements are from file 17. Two cite *Fortitudine* vol. 16 no. 4: the 500-man 1st Pioneer Battalion, and that it could not both unload and move cargo. The third cites a 2025 *Marine Corps History* article for "26 hours and 20 minutes" of actual unloading in the three days.

- Both hosts refused every reader, and the Wayback Machine had no copy that could be read.
- The 500 men is already held from Hough (`land_unit_facts` 500).
- The 26 h 20 min, and the 1942 report it is said to come from, are unproven, not proven false.

**What the corrections are.** No quote was invented. No source says the opposite of the statement that cites it. The corrections are of four kinds.

Wrong page or chapter:

- Miller's map of the final phase is Map XXI, not XX.
- The "veteran regiments" sentence is in Miller ch. VI, p. 137, not ch. V.
- Several Miller and Hough vol. V pages are one out. HyperWar's markers close the page they follow.
- The *Building the Navy's Bases* pages are 243 and 246.

Wrong author or year:

- *The Amphibians Came to Conquer* is by Dyer (Naval History Division), not "U.S. Marine Corps" or "U.S. Navy 1969".
- *From Makin to Bougainville* is Hoffman's, 1995, not 2001.
- *Victory and Occupation* is by Frank and Shaw.
- *First Offensive* is by Shaw.
- The ONI narrative's zone is minus 12, not minus 13: the scanned page says so, and the text layer misreads it.

A qualification dropped:

- The barbed-wire line is a photograph caption, and its ships "were forced to leave", not "carried away" the wire.
- The two officers of 17 July brought back photographs of Tulagi, not of Guadalcanal.
- The "130 men" of the destroyer-transports is the number four washbasins served, not a capacity.

Metadata the source does not print:

- The submarine casualty table prints no author or year.
- The Second War page names its author by first name only.

**What the answers called not found but their own sources hold.** As in the pass of 13 November, this is the weakest part of the answers:

- **Transports.** The ONI index lists Betelgeuse and Libra. Dyer calls Zeilin and Betelgeuse "radarless".
- **Preparation.** The warning chronology is in Zimmerman ch. I. So are the estimated against the actual garrison and the approach. Zimmerman also has the labour trouble at Wellington.
- **Submarines.** The ONI narrative the answer opened gives the repair chronology of Saratoga and North Carolina. It also gives the carrier forces' speeds and zigzag times.
- **Marches.** Miller and Hoffman give the guns abandoned by both Japanese columns.
- **Land units.** Miller gives the Ichiki Detachment's attachment for Midway.

## Where the rows went

| File | Ids | From | What |
| --- | --- | --- | --- |
| `sources.csv` | 1239 to 1249 | all | 11 new sources: the 11th Marines' brief history; the Americal Division's narrative; ANU Press, *Honiara* ch. 2; the Navy's official chronology for 1942; Frank and Shaw, *Victory and Occupation*; Hoffman, *From Makin to Bougainville*; the War Department's *Intelligence Bulletin* of May 1943; H-Grams 009 and 027; the ONI *Index of U.S. Fleet*, 1 December 1944; thesecondwar.pl (grade C) |
| `attributes.csv` | 4 lines | 18 | `gross_tonnage_grt`, `deadweight_lt`, `tonnage_note`, `radar_fit_note` (ship) |
| `places.csv` | 1045 to 1053 | 19 | tasimboko, koli-point, tenaru-river, ilu-river, bloody-ridge, matanikau-river, point-cruz, kokumbona, aola-bay |
| `engagement_outcomes.csv` | 4038 to 4048 | 15 | `verdict_note`: the H-Grams' words on Savo Island, the Eastern Solomons and Cape Esperance |
| `engagement_outcomes.csv` | 4049 to 4055 | 17 | Airfield work started (Japanese and US), beach clearing days, share and days of supply ashore, bombs landed by the CUB One convoy |
| `land_unit_facts.csv` | 813 to 844 | 16 | Prior combat, training, replacements, the Koro rehearsal, jungle training, heavy weapons landed and left, judgements |
| `land_unit_facts.csv` | 845 to 856 | 17 | Construction units' strength and landing, CUB One, the pioneers' unloading, where the perimeter was dug |
| `land_unit_facts.csv` | 857 to 883 | 19 | Positions, attack dates, the Maruyama trail's length, what columns carried, the raider movements |
| `land_unit_facts.csv` | 884 to 915 | 20 | Directive, warning, embarkation, sailing, rendezvous, rehearsal, loading, intelligence; the Japanese side |
| `place_facts.csv` | 189 to 201 | 17 | Henderson Field: Japanese and Seabee equipment, state by date, first aircraft and squadrons, fuel by hand |
| `place_positions.csv` | 52 to 61 | 19 | GeoNames positions of ten places |
| `ship_facts.csv` | 1373 to 1409 | 18 | Gross and deadweight tonnage and speed from the 1944 index; McCawley's "13,000-ton" and 17 knots (Dyer); "radarless" Zeilin and Betelgeuse (Dyer) |
| `ship_facts.csv` | 1410 to 1423 (no 1415) | 21 | Saratoga, North Carolina, O'Brien and Alhena out of action, repair places and return; Kako and O'Brien sunk |
| `gaps.csv` | last 8 lines | 16, 17, 19 | What was looked for and not found |

Id 1415 of `ship_facts.csv` is left unused. It was staged for H-Gram 009's account of Kako's loss, which conflicts in form with the existing `fate` row, and it is kept in this report only (see file 21).

**New attributes of `land_unit_facts.csv`.** These are added to the list in `land-forces.md`. `oob-check` still holds the file to the sourcing rule only.

| Group | Attributes |
| --- | --- |
| The unit before it went in | `prior_combat_note`, `training_note`, `replacements_men`, `amphibious_training_note`, `terrain_training_note`, `heavy_weapons_ashore_note`, `heavy_weapons_left_note`, `quality_judgement_note`, `strength_riflemen` (unused: no source gives it) |
| Marches | `position_on`, `march_distance_miles`, `march_days`, `attack_on`, `attack_planned_on`, `carried_note` |
| Preparing the landing | `directive_on`, `warning_on`, `embarked_on`, `sailed_on`, `rendezvous_on`, `rehearsal_from`, `rehearsal_to`, `loading_note`, `vehicles_left_share_percent`, `preparation_note`, `intelligence_note` |

New measures in `engagement_outcomes.csv`: `verdict_note`, `beach_clearing_days`, `ammunition_days_supply`, `bombs_landed`.

**Standing set by the checker.** `oob-check` requires a value to be `preferred` when one row at medium or high confidence supports it. That applies to 41 new rows of `ship_facts.csv`. It also forbids two chosen rows with different values for one ship.

For six ships, the 1944 index gives a speed different from a row already chosen:

| Ship | Row already chosen | Index row | Speeds |
| --- | --- | --- | --- |
| Fuller | 1163 | 1397 | 16 against 18.2 |
| Heywood | 1165 | 1396 | 16.8 against 18.2 |
| Neville | 1167 | 1398 | 16 against 18.2 |
| Barnett | 1173 | 1402 | 15 against 16 |
| Fomalhaut | 1220 | 1404 | 15 against 14 |
| Betelgeuse | 1224 | 1407 | 15 against 15.5 |

- Both rows of each pair are now `disputed`, and the old row's note says why.
- No value was changed.
- The index's speeds for Bellatrix, Alchiba and Alhena (1403, 1405, 1406) are `disputed` beside the disputed rows already there.

The index does not say what kind of speed it prints.

## 15: verdicts and time away

The five statements are campaign dates. There is no entity for the campaign, so they are here and not in rows.

- **Miller, Map XXI:** "FINAL PHASE 26 January - 9 February 1943" (title block), and "Japanese evacuation nights 1/2, 4/5 and 7/8 Feb", read from the image.
- **Morton, *Strategy and Command*, printed p. 367:**
  - "at the end of December" Imperial General Headquarters decided to withdraw from Guadalcanal and Buna (no day);
  - the decision reached Imamura on 4 January;
  - Patch announced the end on Guadalcanal on 9 February.

**Verdicts.** The readers' own search produced 11 verdict rows, from H-Grams 010 and 011-1 (Cox) only:

- **Eastern Solomons:** "a victory for the U.S. Navy by a very narrow margin"; "temporary"; the Japanese push "fizzled". Cox reports that Nimitz and King saw "a lackluster performance".
- **Cape Esperance:** "Victory at Cape Esperance (Sort of)"; the first US commander to engage a major Japanese surface force and win; a morale boost; the reinforcement landed "unmolested"; "the Americans took away some bad lessons".
- **Savo Island:** "debacle" and "thrashing", in passing in the Cape Esperance essay.

**Not found:**

- verdicts from Morison, the Combat Narratives, Bates, Frank, Lundstrom, Dull or the Japanese official history;
- any statement by a commander weighing a damaged ship against a sunk one;
- departure and return dates beyond those already held. Ralph Talbot's are held as rows 757 to 759, and Boise's as 667.

Patterson, Tenryu, Yubari and Hatsuyuki still have no repair dates.

## 16: land units

What the rows say:

- **The 1st Marine Division and its units:**
  - Colonel Cates estimated that 90 percent of the 1st Marines had enlisted after Pearl Harbor (Shaw).
  - The division had combat-veteran officers and NCOs throughout (Shaw).
  - Hough says Vandegrift gave his "best-trained units" to the Tulagi side: the Raiders, the Parachutists and the 2d Battalion, 5th Marines.
  - Turner's memorandum: "neither the troops, ships, nor aircraft ... are adequately trained in amphibious warfare" (Dyer; undated in the passage).
- **The rehearsal at Koro:**
  - Hough gives 28 to 30 July; Vandegrift's report in Dyer gives 28 to 31 July.
  - Coral made the beaches "impractical for actual landing operations".
  - "Only a little better than one-third" of the Marines meant to rehearse had debarkation or shore training.
  - Craft had to be conserved.
  - Vandegrift called it "unsatisfactory" (May 1943), "a complete bust" (1948), and "probably was not that bad" (1964).
- **Heavy weapons:**
  - A 155 mm howitzer battalion and all the 2½-ton trucks were not loaded (Shaw).
  - 75 percent of the heavier rolling stock was left with the rear echelon (Hough).
  - Less than half the organic motor transport went (Hough).
  - The 2d and 3d Battalions, 11th Marines were on Guadalcanal (Shaw).
  - The engineers loaded "what little dirt-moving equipment they owned" (Hough).
- **The 164th Infantry and the Americal Division:**
  - "troops entering combat for the first time" (Americal history, not naming the 164th);
  - organisation and jungle training began after arrival in New Caledonia.
- **The Japanese units:**
  - Kawaguchi Force: sailed from China, landed in Borneo 16 December 1941, at Cebu in March and Mindanao in April 1942. It received 1,000 replacements in the Palaus in June (Miller).
  - Ichiki Detachment: attached for the Midway landing, then sailed for Guam (Miller). Shaw calls the 28th Infantry "crack".
  - The 17th Army: "veteran regiments" that "had seldom operated as one unit", and that had "never fought against a foe who possessed superior numbers, equipment, or strong defensive positions" (Miller ch. VI).
- **The Japanese on Guadalcanal:** 492 troops and 2,269 Korean labourers landed (ANU *Honiara*; the units are grouped).

**Not found:**

- prior combat of the 1st, 5th, 7th and 11th Marines, the 2d Marines and the 3d Defense Battalion;
- riflemen against total strength, for any unit;
- guns landed against establishment;
- Japanese amphibious or night training;
- the 2d Battalion, 4th Infantry.

## 17: construction and shore work

Seven statements were already held: the 2,600 and 1,178 feet, the Seabees' 387 men and 5 officers, Number One Field in three days, "never out of operation for more than four hours", "less than one-half" of the supplies ashore, and the 120 tons of rations.

**The Japanese field.**

- *Building the Navy's Bases*: the Japanese occupied Guadalcanal in May 1942 and "immediately began the construction".
- The Second War site (grade C, low) gives grass clearing on 20 June and runway work "around 9 July". It gives the 11th and 13th units together as "some 2,500 men", a large share Korean forced labourers.
- What the Japanese left: *Building the Navy's Bases* gives eight small tandem rollers and 15 trucks. Hough, already held, gives nine rollers.

**The American field.**

- The 1st Engineer Battalion began on 9 August.
- It filled a 196-foot gap with 100,000 cubic feet of earth "with hand shovels, trucks, and captured dump cars" (Miller).
- The first aircraft was a PBY on 12 August.
- The field was reported ready on three dates: 17 August (Tulagi radio, Miller), 18 August (Hough, already held) and 19 August (Mersky).
- 19 F4F and 12 SBD arrived on 20 August.
- The Seabees landed on 1 September. A detail took over the field on 2 September. The division report, already held, says about mid-September.

**CUB One** (Mersky; the sidebar is by Jones and Aquilina):

- 120 men, landed on the evening of 15 August;
- 282 bombs landed;
- fuel poured by hand with no pumps;
- 12 days of servicing before the squadrons' ground crews came.

**Unloading.**

- Beach Red "was cleared in five days of hauling" (Miller).
- The division landed only 30 percent of its authorised trucks. The truck size is garbled in the text.
- The shore party was "woefully inadequate". More than half the supplies were still aboard when the ships left (Shaw).
- Ashore were 17 days' rations, counting captured food, and four days' ammunition (Shaw).
- Division pioneers, "later supplemented by hired native labor", unloaded ships until October (Miller).

**Defences.** The Marines dug in "between the Tenaru and the ridges west of Kukum" (Shaw). Ships carrying barbed wire and engineering tools left with them aboard (a Shaw caption). The perimeter was reorganised into ten sectors after 18 September.

**Disagreements:**

- when the Japanese began the field;
- the day the field was ready;
- rollers left;
- when the Seabees took over;
- CUB One's size: 120 (Mersky), 123 (Hough and Zimmerman, already held) or "100-plus" (Dyer, already held).

**Not found:**

- total tons landed 7 to 9 August;
- the shore party's size;
- the time to dig or wire the perimeter;
- matting laid a day;
- aircraft serviced a day;
- a destroyer run's tonnage and how it was landed.

## 18: transports

Every figure the answer gave is in the ONI *Index of U.S. Fleet* of 1 December 1944 or in Dyer.

**The index.**

- It is a later listing. Its rows carry no date and their notes say so.
- It calls its own data "approximate and, in some respects, incomplete".
- It marks tonnage "G." or "D.W." with no key. "G." is read as gross because the same index writes "gross" in full elsewhere. "D.W." is never spelled out.
- It prints no displacement for these ships.

**What it gives:**

- Zeilin 14,123 G.; Hunter Liggett 13,712 G.; American Legion 13,736 G.; Barnett 7,857 G.
- Heywood, Fuller and Neville 11,773 D.W. each; the three President ships 9,937 D.W. each; Crescent City 9,021 D.W.; Alhena 10,000 D.W.
- Bellatrix, Fomalhaut, Alchiba, Betelgeuse and Libra 6,500 G. each.
- Speeds of 14 to 18.5 knots for 15 of the ships.

McCawley, George F. Elliott and the four destroyer-transports are listed only as "(sunk)".

**Dyer.**

- McCawley is "a 13,000-ton, 17-knot diesel-engined merchant ship" (ch. VIII, printed p. 283). It does not say which ton or which speed.
- Zeilin and Betelgeuse are "radarless", in the passage on the days after Koro (p. 309). Dyer also says the two ships "took up the stern chase at 16 knots"; that is not keyed.

**Destroyer-transports.** Hoffman does not state how many men one carried.

## 19: marches and positions

Five statements were already held. What the rows add:

- **Ichiki:**
  - landed at Taivu Point about 18 August;
  - an advance party was met near Taivu Point on 19 August;
  - his men were east of the Ilu on the evening of 20 August;
  - the attack across the sandbar was about 0310 on 21 August.
- **Kawaguchi:**
  - his echelons arrived from 29 August to 11 September;
  - the trail from Tasimboko was begun about 2 September, "using hand tools";
  - the main body was near Taivu Point on 8 September, with Oka near Kokumbona "more than thirty miles west";
  - he was "less than five miles east of the perimeter" by 10 September (Zimmerman);
  - only one battalion was at its jump-off point on the evening of 12 September (Hoffman);
  - artillery and most supplies were left behind (Hoffman);
  - the retreat round Mount Austen to Kokumbona is in a row (19-A07);
  - Zimmerman's caption puts the brigade headquarters ashore near Tasimboko on 6 September. His text, already held, has 1,200 men at Tasimboko on 31 August.
- **Oka:** landed at Kokumbona on 6 September.
- **Maruyama:**
  - left Kokumbona on 16 October;
  - the trail is "about fifteen miles", unit not stated; a footnote says its exact trace is unknown;
  - the men were on half rations of raw rice, and each carried one shell;
  - the guns were abandoned on the way;
  - the attack was put off to 22, then 23, then 24 October.
- **American movements:**
  - The raiders embarked at Kukum at 1800 on 7 September and landed beyond Tasimboko at Taivu Point at about 0520 on the 8th.
  - Carlson's patrol: 150 miles in 30 days to 4 December. His men carried four days of canned rations. He was at Binu on 9 November. Three companies landed at Tasimboko on 10 November.
- **Rations:** an unnamed Japanese unit that attacked on 12 September carried "only three day's ration" (*Intelligence Bulletin*, low).

**Distances stated, in this report only, since they are not positions:**

- Taivu Point is "about twenty-two statute air miles east of Lunga Point" (Miller ch. IV, p. 93) and "about twenty-five statute miles east" (ch. V, p. 120). This is Miller against Miller.
- Tasimboko is "some eighteen miles east of Lunga Point" (Miller ch. V, p. 111).
- The ridge is "about 800 yards east of the Lunga River" (Miller) and "a short mile from the runway" (Zimmerman).
- Aola Bay is "about 40 miles east of the Lunga River" (Frank and Shaw, p. 710).

**Not found:**

- Ichiki's march distance, rations and ammunition;
- any day count or mileage for Kawaguchi's march to the ridge;
- Oka's march after landing;
- the end of Maruyama's march by column.

## 20: preparation

Two statements were already held. What the rows add:

- **The dates:**
  - warning orders to Ghormley on 24 June and to Vandegrift on 26 June (Zimmerman);
  - the Joint Chiefs' directive on 2 July;
  - Combat Group A embarked from 2 July;
  - the second echelon arrived at Aotea Quay on 11 July, and had to unload, sort and reload for combat by 22 July;
  - sailed 22 July;
  - the whole force assembled south-east of the Fijis on 26 July, less Zeilin and Betelgeuse;
  - rehearsal 28 to 31 July (Miller), or 28 to 30 July (Zimmerman);
  - D-Day was put back from 1 to 7 August on 16 July because of the late echelon and the weather (Miller).
- **Loading:**
  - The ships were not combat-loaded leaving the United States.
  - Aotea Quay berthed five ships at a time.
  - Working parties of 300 a ship worked three reliefs around the clock.
  - "labor difficulties with the highly unionized stevedores" left the Marines to do the work (Zimmerman).
  - It rained.
  - Vandegrift was disturbed at "the slowness of the New Zealand dock workers" (11th Marines history).
  - No source read states a strike.
- **What was taken:** 60 days' supplies, 10 units of fire and less than half the vehicles; 75 percent of the heavier rolling stock left behind (Frank and Shaw, p. 664); ammunition cut by half (Miller).
- **Time:** "there was no time for a deliberate planning phase" (Vandegrift, in Miller).
- **Intelligence:**
  - "a stab in the dark" (Zimmerman);
  - "no good maps of Guadalcanal ... never remedied" (Miller);
  - the B-17 flight of 17 July saw no airfield and brought back photographs of Tulagi only;
  - coastwatcher reports were sometimes delayed three days.
- **The Japanese side:**
  - Ichiki's unit was turned back to Truk, arrived on 12 August, and was ordered there to retake the area.
  - The Marines' strength was "badly underestimated by the Japanese" (Zimmerman, fn. 42).
  - The 17th Army, less a division, was taken off the Port Moresby plan.

**For comparison** (report only): New Georgia had "no overall dress rehearsal". Its large transports trained and rehearsed at Efate for ten days from 16 June 1943 (Dyer ch. 14).

**Not found:**

- the name "Operation Shoestring" and who first used it;
- ship-by-ship loading and its effect on unloading;
- order dates for Ichiki and Kawaguchi.

## 21: submarines

There is no submarine rule. Rows went only to ships already in `ships.csv`; everything else is here.

**Rows.**

- **Saratoga:**
  - out of action from 31 August (combinedfleet, ONI);
  - temporary repairs at Tongatabu 6 to 12 September;
  - at Pearl Harbor from 21 September (ONI's "arriving 9 days later");
  - "Early in November, she was once more ready for sea" (ONI).
- **North Carolina:** out of action from 15 September (chronology, ONI); at Tongatabu from 19 September.
- **O'Brien:** hit on 15 September; sunk 19 October (chronology). The database had no `sunk` row for her.
- **Alhena:** torpedoed 29 September (I-4; one of two torpedoes exploded).
- **Kako:** sunk 10 August (chronology).

**Report only.**

- **Japanese patrols:**
  - The patrol line of 13 September: eight boats on the I-21 page, six directed on the I-26 page.
  - Seven boats left the area on 20 September.
  - I-2 and I-3 left Shortland on 27 September on a supply run, towing barges.
  - I-19's floatplane flew over Santa Cruz on 30 August (chronology) or over Graciosa Bay on 28 August (combinedfleet).
- **Saratoga:** I-26 fired six torpedoes at her; one hit at 0748, ONI zone minus 12.
- **15 September:**
  - I-19 fired six Type 95 torpedoes. Wasp was hit by two or three, North Carolina and O'Brien by the same spread.
  - The time is 1145 (combinedfleet), 1444 to 1452 (ONI) or about 1445 (H-Gram 027). The offset is exactly three hours, and no zone is printed by combinedfleet or the H-Gram.
  - Thirty depth charges were dropped on I-19, which escaped.
- **American submarines and losses:**
  - S-44 fired four torpedoes at Kako and three hit. 34 crew were lost (H-Gram 009) or 68 (combinedfleet).
  - S-38 hit Meiyo Maru twice on 8 August.
  - Wasp's SBDs damaged I-17 on 26 August.
  - Gamble sank I-123 on 28 August.
- **Speeds when hit:**
  - Saratoga's force was zigzagging north at 13 knots at dawn, steady on 140 at 13 knots from 0655, and zigzagging again from 0726.
  - On 15 September the base course was 280 at 16 knots, zigzagging from 0650. Wasp had slowed to 13 knots to launch.
  - ONI: "the ships were operating at comparatively slow speeds, ranging between 13 and 15 knots", and zigzagging had stopped for flight operations in every case.
- **Time out:** North Carolina "out of action for two months" (H-Gram 027). Carter, already held, gives 30 September to 17 November.
- **Disagreements:**
  - Saratoga's date: H-Gram 027 says 28 August; every other source says 31 August.
  - O'Brien's hit: port quarter (combinedfleet) or bow (ONI, DANFS).
  - Kako's position.
  - Meiyo Maru's position.
  - How many boats attacked Wasp's group: one (I-19) or "not more than three" (ONI).

## For the architect

### Transport gaps, class by class

None of the gaps in `troop-lift.md` is closed in the form the derivation reads.

| Class | Gap | What the new rows give |
| --- | --- | --- |
| Harris (Zeilin, Hunter Liggett) | Standard displacement | Still a gap: only gross tonnage (1944) |
| Harris (Zeilin, Hunter Liggett) | Radar | Zeilin: a stated absence ("radarless", Dyer, late July 1942), as `radar_fit_note`, not a fit row |
| Harris (Zeilin, Hunter Liggett) | Endurance with its speed, fuel, rounds per gun | Still open |
| Harris (Zeilin, Hunter Liggett) | Speed | New ship-level speeds from the index: Zeilin 18, Hunter Liggett 18.5 |
| American Legion | Standard displacement | Still a gap: only gross tonnage |
| American Legion | Speed | 18.5 |
| American Legion | Everything else listed | Open |
| President Jackson (Jackson, Adams, Hayes) | Standard displacement | Still a gap: only deadweight |
| President Jackson (Jackson, Adams, Hayes) | Speed | 16.5 for each, a third figure beside 18 and 17.9 |
| President Jackson (Jackson, Adams, Hayes) | Everything else listed | Open |
| Heywood (Heywood, Fuller, Neville) | Standard displacement | Still a gap: only deadweight |
| Heywood (Heywood, Fuller, Neville) | Speed | Now disputed for each ship (18.2 against 16 to 16.8) |
| Heywood (Heywood, Fuller, Neville) | Everything else listed | Open |
| Heywood: George F. Elliott | Everything | Nothing new |
| McCawley | Tonnage and speed | "13,000-ton", kind unstated, and 17 knots, kind unstated (Dyer) |
| McCawley | Everything else listed | Open |
| Barnett | Standard displacement | Still a gap: only gross tonnage |
| Barnett | Speed | Disputed (16 against 15) |
| Crescent City | Standard displacement | Still a gap: only deadweight |
| Crescent City | Speed | 18 |
| Crescent City | Everything else listed | Open |
| Arcturus, Alhena, Fomalhaut, Bellatrix | Standard displacement | Still a gap: only gross or deadweight |
| Arcturus, Alhena, Fomalhaut, Bellatrix | Speed | Speeds of 14 to 15.5, disputed where rows existed |
| Arcturus, Alhena, Fomalhaut, Bellatrix | Radar | Betelgeuse "radarless" |
| Arcturus, Alhena, Fomalhaut, Bellatrix | Bellatrix's class | Still a gap |
| Arcturus, Alhena, Fomalhaut, Bellatrix | Endurance, fuel, cargo capacity, boats carried | Still open |
| Wickes destroyer-transports | Everything | Nothing new. The index lists them only as sunk. No stated troop capacity |
| Japanese transports and patrol boats | Everything | Nothing new: the answer had none |

**What a derivation would need.**

- **Displacement.** Either a standard or full-load displacement, or a written rule taking tonnage from gross or deadweight. A gross figure is a volume and a deadweight figure is a lift, not a displacement. That rule is the architect's to write or refuse.
- **Radar.** A fit row of "no radar" for Zeilin and Betelgeuse on 7 August. Dyer's sentence would support it, and the format already has the stand-in sensor for "no radar of any kind". It was not keyed as a fit, because that is a filing choice for the derivation.
- **The rest.** Endurance with its speed, fuel, rounds per gun, and the two rules decision 0093, 8 lacks: a kind for the types `transport` and `cargo_ship`, and a cargo rule.
- **The check.** `oob-derive data/derivation/guadalcanal_landings.toml --check` passes unchanged. The list derives only the four Fubuki-class ships.

### Decision 0094: the two marches

The placeholders are `march_days_per_hex` = 3 and `march_supply_days` = 5.

**Ichiki.** He landed at Taivu Point (hex (3,-5)) about 18 August, was east of the Ilu (hex (2,-4)) on the evening of 20 August, and attacked at about 0310 on 21 August.

- That is about three days from landing to attack across one hex side (the lead's arithmetic from stated dates).
- No source gives the distance he marched.
- The straight-line distance is Miller's "about twenty-two statute air miles" from Taivu Point to Lunga Point, or "about twenty-five".

**Kawaguchi.** His echelons landed from 29 August to 11 September. The trail from Tasimboko (hex (3,-5)) was begun about 2 September.

| Date | Where his force was |
| --- | --- |
| 8 September | Main body near Taivu Point |
| 10 September | Less than five miles east of the perimeter |
| 12 September | Only one battalion at its jump-off point |
| Night of 12 to 14 September | The attack on the ridge (hex (2,-4)) |

- That is about ten days from the start of the trail to the attack, and about four or five from Taivu to the ridge for the main body (the lead's arithmetic).
- The march went through jungle inland, by a trail cut with hand tools. Artillery and most supplies were left behind.
- The retreat round Mount Austen to Kokumbona took "over a week" by Miller's account in row 19-A07, filed as `march_days` 7, low.

**Maruyama.** He set out on 16 October and attacked on 24 October, about 15 miles on Miller's figure, unit not stated. That is about eight days, mostly inside hex (2,-4) (Kokumbona to south of the airfield, both (2,-4)).

**Carlson.** 150 miles in 30 days, a patrol.

**Supply carried.**

- Kawaguchi: "only a few days' rations" (Miller, already held).
- An unnamed attacking unit of 12 September: three days (*Intelligence Bulletin*).
- Maruyama: half rations of raw rice, and one shell a man.
- Carlson: four days of canned rations.
- Ichiki: seven days, from Wikipedia only (already held, row 841).

### Decision 0095: preparation and the share unloaded

**Days of preparation, all the lead's arithmetic from stated dates:**

| Interval | Days |
| --- | --- |
| Vandegrift's warning (26 June) to the landing (7 August) | 42 |
| The directive (2 July) to the landing | 36 |
| Re-loading of the second echelon, 11 to 22 July | 11 |
| The rehearsal | 3 or 4 |

**What sources say the shortness cost:**

- D-Day was put back six days for the late echelon and the weather.
- "no time for a deliberate planning phase".
- The rehearsal gave "a little better than one-third" of the Marines meant to rehearse any debarkation or shore training.
- Vandegrift judged the rehearsal "dubious" against "priceless time" (1943).
- Turner: not "adequately trained in amphibious warfare".

**Share unloaded:**

- less than half of 60 days' supply (Miller; already held);
- "more than half of the division's supplies still needed ashore" left aboard (Shaw);
- Fomalhaut 15 percent (already held);
- ashore were 17 days' rations, counting captured food, and four days' ammunition;
- Beach Red took five days of hauling to clear.

**What is still missing.** The one figure that would size `unprepared_landing_share` directly, actual unloading hours (26 h 20 min), could not be read. No source gives tons landed.

### Decisions 0097 and 0098: what would distinguish the named units

These are statements, not ratings.

- **1st Marines:** 90 percent enlisted after Pearl Harbor (Cates).
- **The division:** veteran officers and NCOs throughout; amphibious training cut short at Koro.
- **Raiders, Parachutists and 2d Battalion, 5th Marines:** the "best-trained units" (Hough). Then "rested after their hard fighting" on 7 to 8 August.
- **164th Infantry:** first combat in October; jungle training only from New Caledonia, from March 1942.
- **Kawaguchi Force:** Borneo, Cebu and Mindanao, December 1941 to April 1942; 1,000 replacements in June.
- **Ichiki Detachment:** "crack" (Shaw); trained for the Midway landing.
- **17th Army:** "veteran regiments", seldom operated as one unit, never against superior numbers or strong positions.
- **Japanese construction units:** largely Korean labourers. 492 troops landed with 2,269 labourers (ANU).

**What no source gives:** riflemen against total strength, and Japanese amphibious, night or jungle training.

### Decision 0099: men and days to build the airfield, and unloading rates

**The Japanese field.**

- Begun in May (Navy base history), or runway work from about 9 July (grade C).
- Built by the 11th and 13th units: "some 2,500 men" (grade C). Rows already held give about 900 a battalion, and the ANU count of 2,269 labourers.
- 2,600 feet done by 7 August.

**The American field.**

- The 1st Engineer Battalion, from 9 August, with captured equipment.
- It added 1,178 feet and filled a 196-foot gap by moving 100,000 cubic feet of earth by hand and with captured dump cars.
- Ready on 17, 18 or 19 August: eight to ten days (the lead's arithmetic).
- CUB One's 120 men worked with them from 16 August.
- The first squadrons came on 20 August.

**The Seabees.** 392 men landed on 1 September and took over on 2 September (or mid-September). They built Number One Field, a 4,600-foot turf strip, in three days with Japanese equipment (already held).

**Unloading.**

- No rate in tons a day.
- The 500-man pioneer battalion (already held) and hired native labour unloaded until October.
- Beach Red took five days to clear.

**Defences.** Where the Marines dug is stated, but how long it took is not.

### Each place's hex

From the projection in `data/map/south_pacific.csv` (nearest hex centre). Positions are GeoNames, except Tassafaronga and Cape Esperance, which use the rows already held.

| Place | Position used | Hex | Distance to centre | Next nearest |
| --- | --- | --- | --- | --- |
| Taivu Point | -9.40000, 160.35000 (row 52) | **(3,-5)** | 17.2 nm | (3,-4) at 26.1 nm |
| Tasimboko (GeoNames area "Tadhimboko") | -9.43333, 160.30000 (53) | **(3,-5)** | 19.4 nm | (2,-4) at 23.4 nm |
| Koli Point | -9.40875, 160.17792 (54) | **(2,-4)** | 19.9 nm | (3,-5) at 20.1 nm, on the border |
| Tenaru River (GeoNames stream point, not the mouth) | -9.43073, 160.09602 (55) | (2,-4) | 16.6 nm | (3,-5) at 23.9 nm |
| Ilu River (the same) | -9.43803, 160.06990 (56) | (2,-4) | 15.7 nm | |
| Bloody Ridge | -9.45008, 160.04939 (57) | (2,-4) | 14.7 nm | |
| Matanikau (GeoNames "Mataniko River") | -9.43502, 159.96684 (58) | (2,-4) | 15.5 nm | |
| Point Cruz | -9.42615, 159.95899 (59) | (2,-4) | 16.0 nm | |
| Kokumbona (GeoNames "Kakambona"; identity not stated by the page) | -9.41667, 159.91667 (60) | (2,-4) | 17.1 nm | (2,-5) at 23.7 nm |
| Tassafaronga Point | -9.36806, 159.86412 (51) | (2,-5) | 19.5 nm | (2,-4) at 20.9 nm |
| Cape Esperance | -9.25, 159.71667 (32) | (2,-5) | 8.9 nm | |
| Aola Bay | -9.52618, 160.48769 (61) | **(3,-4)** | 15.1 nm | (3,-5) at 26.2 nm |

The check gives Lunga Point and Henderson Field (2,-4) and Tulagi (3,-5), as the map's anchors do.

- **Taivu Point** is in (3,-5), as the architect's guess had it.
- **Tasimboko** is in (3,-5) too.
- **Koli Point and Tassafaronga** lie almost on hex borders.
- **Aola Bay** is in (3,-4), not (3,-5).

So both Japanese landing places east of the perimeter are one hex from the airfield's hex.
