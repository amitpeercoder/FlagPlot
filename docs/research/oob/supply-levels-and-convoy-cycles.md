# Levels of supply and convoy cycles, South Pacific 1942

Status: a research report for the architect, read on 2026-11-14. Nothing here is settled. Every new row is `unconfirmed`; `oob-check` asked for no other standing.

The staff of step 3 keeps a base at a floor of so many days of supply (decision 0107, decisions 17 and 37 to 40). The rulings from its hand calculations size a run to bring the base to the floor plus one round trip's use, so in `tests/scenario_36_one_convoy.rs` a 3,000-ton ship carries 40 t every four days. Question 5 of the decision asks for a doctrine figure: the days of use a run carries above the floor. This pass looked for how supply levels were defined, ordered and run in the South Pacific in 1942, so that the figure and the floor can rest on the record.

## How it was read

Five readers each took one head of the work:

- the Army's doctrine manuals and the War Department's overseas levels;
- the Army's technical-service histories of the South Pacific;
- the Navy's logistics histories;
- Guadalcanal in practice;
- the Japanese side.

The method is the one in `outside-research-nov-14-slice.md`. For every statement, the reader fetched the document, converted it to text, found the words and read the passage around them. Staging was from the document's own words only. Each reader also checked the list of rows already held, and did not restage those.

**The quote check.** Every staged `as_stated` was checked by program against the reader's saved text, allowing only whitespace, typography and soft hyphens. All 206 passed. Two qualifications apply:

- The scans of the 1st Marine Division's final report and of Japanese Monograph 98 have poor machine-read text. Their quotes are as the text shows them, misreadings included (for example "Oast and Dog" for C and D rations).
- Rows whose figure rests on a doubtful digit say so and are graded down.

**What the lead read himself.** The points the two numbers rest on were read in the saved texts, not taken from the readers' reports:

- Leighton and Coakley, pp. 334 to 336: the levels, the operating level, the 90 and 180 illustration, the emergency rations.
- Their Appendix F-1, pp. 734 to 735, read cell by cell against the column heads, with footnotes a, c and h.
- Stauffer, p. 135: the Quartermaster levels and the two-part reserve.
- FM 100-10 of 1943, chapter 1: the definitions of the levels.
- Carter, ch. 4, pp. 32 to 33: Ghormley's signals of 16 and 18 August.
- Dyer, ch. 11, pp. 419 to 420: the days on which ships were unloading.
- The 1st Marine Division's final report, Phase V: the commander's recommendations on supply.
- Japanese Monograph 98, scan pp. 56 to 58: the drum runs.

**What only a reader checked.** Everything else. That includes:

- Japanese Monograph 98's 23 days of rations, which the reader checked on the page image;
- the Phase IV ship calendar;
- the Ballantine passages;
- the Bykofsky and Larson tables.

**What the check does not show.** It shows the words are in the document. It does not show that a figure was filed against the right base or date. That filing is the readers' work, looked over by the lead.

## Counts

| Reader | Staged | Keyed (rows) | Not keyed: no table | Not keyed: duplicate or held |
| --- | --- | --- | --- | --- |
| Army doctrine (FM 100-10, FM 101-10, TM 20-205, ASF report, Leighton and Coakley) | 54 | 30 | 23 | 1 |
| Army histories (Stauffer, Leighton and Coakley, Bykofsky and Larson, Craven and Cate) | 37 | 28 (29 rows) | 0 | 9 |
| Navy histories (Carter, Dyer, Ballantine, Building the Navy's Bases) | 35 | 28 | 7 | 0 |
| Guadalcanal in practice (final report IV and V, Miller, Hough, Dyer) | 65 | 60 | 0 | 5 |
| Japanese (Japanese Monograph 98) | 15 | 15 | 0 | 0 |
| **All** | **206** | **161 statements, 162 rows** | **30** | **15** |

All 206 statements were verified: every quote is in its document. One statement (the anchorages of Espiritu Santo and Efate) is keyed twice, once for each place.

**Not keyed, and why:**

- **Duplicates (13).** The Army readers read Leighton and Coakley from two archive.org scans and staged the same sentence seven times. Two readers staged the same five sentences of Dyer. One reader staged a partial reading of Appendix F-1, which the other read in full.
- **Already held (1).** Craven and Cate's 24 ships a month at Noumea is in `place_facts` 115.
- **Inferred (1).** The South Pacific's Quartermaster emergency reserve of 90 days is a subtraction, 150 less 60, that Stauffer does not state. It is in the note of row 1082, not a row.
- **No table (30).** Two kinds of statement fit no entity, and no table was invented for them. They are listed below under "Doctrine not keyed".
  - **Doctrine for every command (23):** FM 100-10 (1940 and 1943), FM 101-10 (1941), TM 20-205 (1944), the Army Service Forces' final report, and three general passages of Leighton and Coakley.
  - **Navy advanced base units (5) and Bora Bora's unloading (2):** neither a Lion nor Bora Bora is an entity.

The readers' counts by kind of statement differ from the table above. The table counts what the lead keyed.

## Where the rows went

| File | Ids | What |
| --- | --- | --- |
| `sources.csv` | 1267 to 1270 | Leighton and Coakley, *Global Logistics and Strategy 1940-1943* (1955); Stauffer, *The Quartermaster Corps: Operations in the War Against Japan* (1956); Bykofsky and Larson, *The Transportation Corps: Operations Overseas* (1957); Ballantine, *U.S. Naval Logistics in the Second World War* (1947) |
| `land_units.csv` | 1098, 1099 | `us-forces-south-pacific-area` (the Army and Navy bases and the forces afloat) and `us-forces-southwest-pacific-area` (Australia), both `forces-at-place` |
| `lift_entities.csv` | 1078 to 1081 | The Japanese destroyer drum runs of 7, 11 and 13 December 1942 and of 2, 10 and 14 January 1943 |
| `land_unit_facts.csv` | 1055 to 1185 | 131 rows: levels and policy for the South Pacific, Guadalcanal in practice, the division's recommendations, the Japanese plans |
| `place_facts.csv` | 217 to 240 | 24 rows: Noumea's congestion and discharge, Espiritu Santo and Efate, the Guadalcanal beaches, fuel at Henderson Field |
| `troop_lift_facts.csv` | 183 to 189 | 7 rows: the drum runs and the evacuation plan's destroyers |
| `gaps.csv` | last 10 lines | What was looked for and not found |

**No new attribute was used.** Levels, policies and intervals are `supply_note` rows on a land unit, each with its figure in words in `value`. Days of rations on hand are `rations_days_on_hand`. The ports are `port_note` and `ships_waiting_note` rows, and fuel at Henderson is `aviation_fuel_storage_note`. The drum runs are `ships_in_run` and `lift_note` rows, since drums are a count and not tons.

**Rules for every overseas base.** The War Department's 1942 rules for every overseas base are filed on `us-forces-south-pacific-area`, because they governed its bases. The note of each such row says the sentence does not name the South Pacific. The rows are 1056 to 1062, 1064 to 1066, 1074, 1078 to 1080 and 1089.

## 1. Levels of supply as doctrine

### The terms

These come from FM 100-10 of 15 November 1943, chapter 1 (not keyed; listed below).

- **Day of supply:** the estimated average use per day in campaign.
- **Minimum level:** a reserve, drawn on only in emergency.
- **Operating level:** the quantity needed to maintain a command, which "is determined by the frequency of shipments and the time required for supplies to reach their destination".
- **Maximum level:** the minimum plus the operating level, on hand and due in.
- **Reorder point:** the stock below which supplies are requisitioned on the next scheduled date.

The 1940 edition, with its changes to 1942, has the day of supply and automatic supply on a schedule. It does not define the levels. The definitions first appear in 1943, codifying the War Department's instructions of 1942.

### What the War Department ordered in 1942 (Leighton and Coakley)

- **What a level was.** Authorized levels were rough estimates, in days of supply, of what would carry a base through an interruption of its regular supply (1089). In May 1942 they were made minimum reserves, to be drawn on only in emergency (1056).
- **The operating level.** The July 1942 instructions added an operating level above the minimum, as a cushion between regular shipments (1057).
  - For Class I (rations), the maximum is the minimum plus "normal consumption until expected arrival of next supply shipment". The port commander sets it with the overseas commander (1065).
  - For Class II, the maximum is the minimum plus 90 days (1064, 1058).
  - For Classes I and III, how far stock may rise above the minimum is agreed between port and base. Class IV and ammunition have no operating level (1059).
- **Practice.** Maximums were seldom reached (1061). The book's illustration is a base with a 90-day minimum and a 180-day maximum: it holds its 90 days, and up to 90 more are in the pipeline back to the port (1060).
- **The South Pacific.** After the Japanese drive of spring 1942, the War Department raised the reserve levels of the South Pacific bases to 120 days (1055). Appendix F-1 gives the minimum levels from the War Department letters of 19 July 1942 and 10 July 1943 (1063):
  - 1942: Class I 120 days (footnote h: 90 for Australia and New Zealand), Classes II, III and IV 90 days each.
  - 1943: all four classes 90 days.
  - Emergency rations are included in the Class I level (footnote c).
- **Emergency rations.** In the spring of 1942, six months of concentrated emergency rations were authorized for the Pacific island bases other than Hawaii and Australia (1062).
- **Ammunition.** Ammunition levels were minimum levels in units of fire (1074).
- **The 1943 revision.** It capped operating levels at 90 days for Classes I to III (1080), with modest reductions (1079), and put ammunition into days of supply (1078).
- **Other 1942 levels:**
  - Bora Bora (Bobcat), early 1942: 60 days of maintenance, doubled for rations and gasoline (1073).
  - Plan X, December 1941: 60 days for Australia and the Philippines (1071).
  - Australia, February 1942: 90 days for ground forces and five months for air forces (1072).
- **What later planners wanted.** In November 1942 Harmon and Lutes wanted 30 days in the intermediate depots, as a strategic reserve above the 120-day theatre level. OPD refused (1093).

### Quartermaster stocks (Stauffer, p. 135)

The War Department split the reserve into an emergency (minimum) reserve and an operating reserve. Supplies on order or in the troops' hands were excluded.

| Area | Emergency reserve | Operating reserve | Maximum reserve | Rows |
| --- | --- | --- | --- | --- |
| Southwest Pacific | 90 days | 90 days | 180 days | 1081 |
| South Pacific | not stated | 60 days | 150 days | 1082 |

Stauffer says the South Pacific's levels were lower because it was nearer the West Coast. He also says the distinction between the two reserves blurred in practice (in the note of 1082).

Requisitioning added 90 days of "order and delivery" time in the South Pacific (1083). That is 1943 practice. Before the general depot of 1943, a requisition on the United States took three or four months (1084).

### Who set the levels

- **The Army:** the War Department set them by letter (1055, 1063). Within the 1942 rule, the operating levels for Classes I and III were agreed between the port and the overseas commander (1059, 1065).
- **The Navy:** under the Basic Logistical Plan of 8 March 1943, the area commander determines the levels to be maintained (1122).
- **The joint plan of 15 July 1942:** it shipped supplies in shipload lots, by each service, directly to the bases (1068, 1070). It split the classes between the services: the Navy for all petroleum, the Army for subsistence at Army bases (1069, 1098, 1107).

### The Marine Corps

The 1st Marine Division embarked 60 days of supply and 10 units of fire, against a normal 90 days and 20 units (held 448). Its final report recommends, for a future landing:

- embark not more than 30 days of supply, 10 units of fire and 50 days of rations (1144; held 453 is Hough's paraphrase of the same);
- land half at first (1145);
- land at first no more than can be unloaded in 10 to 12 hours, about five days of supply (1147);
- bring in the cargo ships singly or in pairs at regular intervals from D plus 2 (1146), and no more than two at a time (1149).

The report also says a supply plan must fix "tonnage and frequency of subsequent shipments", and none did for Guadalcanal (1143). Its D-4 recommended "levels to be maintained" in an Appendix 5 that is not in the copy read (1151).

## 2. How it ran at Guadalcanal, August to December 1942

**No resupply schedule at the start.** No naval operation order for WATCHTOWER provided scheduled or automatic resupply over the first 30 to 60 days (1117). Unloading stopped on D plus 1, against an expected D plus 4 (1140, 1142). Supplementary shipments were "most irregular" until October (1141).

**Days on hand.**

| Date | Reading | Rows |
| --- | --- | --- |
| 9 August | 37 days of food | held 414 |
| About 15 August | 17 days of B rations, 3 of C and 10 of captured Japanese rations: 30 in all | held 416, 417, 419, 710 |
| 16 August | 3 days of rations and 5 units of fire (Ghormley's report to King and Nimitz) | 1102, 1103 |
| 21 August | 120 tons of rations, given as about 3½ days | 1123; held 711 |
| September to December | aviation gasoline "generally" under 6 days | 1152 |
| 15 October | 400 drums of aviation gasoline, about two days of operations | 238 |
| November | motor gasoline "critically low" several times | 1153 |

The division's daily ration need rose from 35 to 50 tons as the garrison grew (held 467). The garrison went from 18,134 men to over 45,000 (1157).

**Arrivals.** The division's calendar (1124 to 1130):

| Date | Ships |
| --- | --- |
| 21 August | Colhoun, Gregory and Little |
| 22 August | Alhena and Fomalhaut |
| 29 August | Burrows and Kopara |
| 31 August | Betelgeuse |
| 1 September | three YPs |
| 2 September | Fomalhaut |
| 7 September | Bellatrix and Fuller |

The ships came at irregular intervals and were "rarely unloaded completely" (1131). Later runs:

- Betelgeuse, 24 to 26 September (1162);
- Alhena, 26 to 29 September (1163);
- Alchiba and Fuller, 2 November (1169);
- the transports of 11 and 12 November, unloaded 20 to 100 percent by ship (1165 to 1168).

**How often ships were unloading** (Dyer, 1112 to 1114):

- on 6 of the last 20 days of August;
- on 13 days of October;
- on all 31 days of December.

**How much came.**

- 5,259 long tons were shipped to Guadalcanal in November and 7,271 in December (1170).
- The 18 September landing put ashore about 28 days of rations by the division's report (1155, poor text), against 40 days in Turner's plan (held 624).

**Limits at the beach.**

- Unloading was by lighter to open beaches (1110).
- From 24 September, ships unloaded off Lunga Point by day only and retired east at night (1160, 1161, 1164).
- After 20 September, a cargo ship of 3,000 to 3,500 long tons could be emptied in 30 to 35 working hours (1148).
- Turner's blockade-run orders of 12 August were to arrive late, unload at night and leave early (1173).

**Ships.**

- Turner's cargo ships for the run fell to four (1109, Dyer, around September).
- Halsey reported four undamaged cargo ships in the whole South Pacific Force at the end of November (1171).
- Convoys to Guadalcanal were kept small because of the tactical situation, and backlogs built up in the rear ports (1095).

**What was asked for.** Turner wrote of reserves "very low" in November (1115). In December support ships landed the requirements daily (1116). Harmon asked for a standing supply of at least 250,000 gallons of aviation fuel at Henderson Field (239).

**The rear base.** Noumea was the bottleneck:

- 86 ships at anchor on 23 September (217).
- 37 ships with 88,000 tons to discharge on 20 November (218, 226).
- 91 vessels with 180,000 tons late in November (237).
- As many as 100 at one time (235).
- Ships were held as floating warehouses (220, 224, 227), some for as long as 90 days (227).

## 3. The Japanese

**Rows already held.** These say what a run carried. A destroyer was planned to carry 110 men and two weeks' rations (`troop_lift_facts` 170 and 178). The Kawaguchi movement carried two weeks' rations (`troop_lift_facts` 176, `land_unit_facts` 975). The Army's rule was 2 days' rations on the man, 3 in the unit and 2 more at division (1044). Expeditionary units left Japan with 30 days (1048).

**New rows, from Japanese Monograph 98:**

- **The plans.**
  - November 1942: ammunition and provisions for 60,000 men for 20 days, to be stored up for the 38th Division (1178).
  - The recapture was to move 30,000 men, 300 guns and 30,000 tons, which would need 150 transports or more than 800 destroyer loads (1179, 1180).
  - The directive of 16 November: guarantee "a minimum supply" to hold the status quo (1182).
  - The evacuation plan of January 1943: 23 days of rations to be sent ahead by 15 January (1185).
- **The drum runs.**
  - 30 November: eight destroyers, drums abandoned (183).
  - 7 December: 11 destroyers, not unloaded (184).
  - 11 December: nine destroyers, 1,200 drums delivered, not more than 200 recovered (185, 186).
  - A run JM-98 dates 13 December: 11 destroyers, about a third of 1,500 drums recovered (187).
  - Runs of 2, 10 and 14 January (188).
- **What it came to.** Supply could not meet "even minimum requirements" of the more than 30,000 men on the island (1184).

No document read states a planned interval between runs, a cargo per destroyer in tons, or a stock the 17th Army meant to keep in days, beyond the 20 and the 23 days of the plans.

## 4. Planning cycles and reorder intervals

The cycles the rows state:

| Interval | What | Rows |
| --- | --- | --- |
| Every 15 days | 200,000 barrels of fuel oil, a standing order for the South Pacific (August 1942) | 1099 |
| Monthly | Fuel shipments sized to a measured use of 28,000 barrels a day (Ghormley's request of 18 August) | 1100, 1101 |
| 30 days | Troop convoys to Australia and the Pacific ferry islands (9 February 1942) | 1067 |
| Monthly | The three echelons of the March 1942 base expeditions | 1119 |
| Not oftener than 30 days | Provisioning of the forces afloat (March 1943) | 1106 |
| Until the next expected shipment | The Class I operating level | 1065 |
| Frequency of shipments plus transit time | The operating level (FM 100-10, 1943) | not keyed |
| Not longer than the days carried by the division | Periodic Class I shipments to a division (FM 101-10, 1941) | not keyed |
| 3 to 4 months | A requisition on the United States | 1084 |

## Where the sources disagree

- **The South Pacific's level.** Leighton and Coakley's text has 120 days for the bases (1055). Their table gives 120 for Class I only and 90 for the other classes (1063). Stauffer gives Quartermaster stocks an operating reserve of 60 days and a maximum of 150 (1082). The two books may count different things: all classes as a minimum, against Quartermaster items as two reserves. Neither reconciles them.
- **Rations at Guadalcanal in mid-August.**
  - 3 days, Ghormley's report of 16 August in Carter (1102).
  - 17 days, Dyer, 15 August (held 710).
  - 30 days with captured rations, the division and Miller (held 416, 419).
  - 37 days, Hough, 9 August (held 414).
- **Noumea's discharge.**
  - Leighton and Coakley: 47,808, 138,085 and 213,982 short tons in November, December and January (219).
  - Bykofsky and Larson: 38,447, 141,362 and 217,449 short tons of cargo discharged, in a total traffic of 53,545, 154,654 and 239,660 (229).
  - Miller: 34,327 and 126,216 long tons in November and December (236).
- **Ships waiting at Noumea:** 86, 37, 91 or 100. The dates and the ships counted differ (217, 218, 237, 235).
- **Four cargo ships left:** around September (Dyer, 1109) or at the end of November (Halsey, in Miller, 1171).
- **The 18 September landing:** 28 days of rations (division report, poor text, 1155) or 40 days (Miller, held 624).
- **The drum run of "13 December":** JM-98 alone gives that date (187).

## Doctrine not keyed (no entity)

These are statements of doctrine for every command, or about the Navy's advanced base units. They are in the readers' saved texts and passed the quote check.

| Source | Where | What it says |
| --- | --- | --- |
| FM 100-10, 15 Nov 1943 | ch. 1, "Level of Supply" a to e, pp. 3-4 (https://archive.org/download/FM100-10-nsia/FM100-10_djvu.txt) | Day of supply; minimum level (a reserve drawn on only in emergency); operating level (set by the frequency of shipments and the transit time); maximum level (minimum plus operating); reorder point (requisition on the next scheduled date when stock falls below it) |
| FM 100-10, 15 Nov 1943 | par. 160, p. 62 | 2 days of supply stocked at army supply establishments |
| FM 100-10, 9 Dec 1940 with changes 1 to 3 | definitions; pars. 56, 62, 161 (https://archive.org/download/FM100-10FieldServiceRegulationsAdministration/) | Day of supply; automatic supply on a predetermined schedule; balanced stocks for a fixed period; depot levels set by the theatre commander; supply by prearranged schedules of shipments; 2 days at army level |
| FM 101-10, 15 Jun 1941 | pars. 83, 104, 107, pp. 163, 180, 181 (https://archive.org/details/FM101-10) | Periodic Class I shipments to a division at intervals no longer than the days of supply the division carries; the theatre commander fixes the day of supply; the pounds-per-man table's assumptions |
| TM 20-205, 18 Jan 1944 | entry "day of supply" (https://archive.org/details/TM20-205-nsia) | The definition of a day of supply |
| *Logistics in World War II* (ASF final report, 1948) | pp. 79-81 (https://archive.org/details/LogisticsInWorldWarII) | The first phase of a theatre supplied automatically, with reserves then built to maximum and minimum levels; automatic supply for rations and fuel at established bases; the monthly status report; up to three months from report to shipment |
| Leighton and Coakley | pp. 299, 326, 327 | The Army Supply Program's daily factors; the ports' monthly automatic-supply reports; Boston's radio report of days on hand by class |
| Carter, ch. 6, p. 50; Ballantine, p. 103; *Building the Navy's Bases*, vol. I, ch. V, pp. 120, 125, 126 | | What a Lion and a Cub were and their size; 4 Lions and 12 Cubs procured; the Bureau of Yards and Docks' procurement classes and its 12- to 18-month schedule (classes of material, not of supply) |
| *Building the Navy's Bases*, vol. II, ch. XXIV, pp. 198-199 | | Bora Bora: the first unloading took six weeks; 60,000 tons unloaded in 18 months |

## What is missing

Ten lines were added to `gaps.csv`:

- the operating level agreed for Classes I and III at a named South Pacific base;
- the War Department letters of 19 July 1942 and 10 July 1943 in their own words;
- the size and interval of a scheduled maintenance shipment to Noumea, Espiritu Santo or Guadalcanal;
- days on hand at Noumea, Espiritu Santo and Efate;
- the days of supply a Lion or Cub was outfitted for;
- days on hand at Guadalcanal after mid-August. The division's appendices that would give them are not in the copies read.
- tons landed by run, September to November;
- unloading rates in tons a day at Guadalcanal and Espiritu Santo;
- the Japanese cycle, cargo per run, daily need and stock. Japanese Monograph 34 could not be fetched.
- the South Pacific's ammunition row in Appendix F-1.

Not read in this pass:

- Morison, Frank and Lundstrom: copyrighted, and not in hand.
- Furer.
- The USAFISPA history.
- Japanese Monograph 34 and 99.
- The USSBS interrogations, for the Japanese supply plan.

## What this bears on

Decision 0107 needs two numbers: the floor, which is the days of supply a commander keeps at a base, and the doctrine figure, which is the days of use a run carries above it.

### The doctrine figure: days of use a run carries above the floor

**What the rows say.**

- The quantity kept above the minimum was the operating level. It was a cushion between regular shipments (1057), sized for Class I to consumption until the next expected shipment (1065). FM 100-10 of 1943 sets it by shipment frequency and transit time (not keyed).
- So in the doctrine of the period, what a shipment brings above the floor is the use of one interval between shipments. It is not one round trip of the ship.
- **The stated operating levels:**
  - 60 days for Quartermaster stocks in the South Pacific (1082).
  - 90 days for the Southwest Pacific (1081).
  - Up to 90 days, or three months of maintenance, for Class II (1058, 1064).
  - At most 90 days for Classes I to III after July 1943 (1080).
  - The 90 days in the pipeline of Leighton and Coakley's illustration (1060).
- **The stated intervals between shipments:**
  - 15 days for fuel oil (1099).
  - Monthly or 30 days for fuel requests, troop convoys, base echelons and fleet provisioning (1101, 1067, 1119, 1106).

**The range the rows support.** On the rows alone, the days above the floor range from 15, the fuel interval, to 90, the cap on operating levels. The Army's stated operating levels for the area are 60 to 90 days, and its shipping intervals are 15 to 30 days.

**What the rows do not support.**

- No row gives a planned cycle or load for Guadalcanal itself. No resupply schedule was set (1117, 1143), and runs were irregular (1131, 1141).
- What a run carried there was set by the beach and the enemy, not by a level:
  - a ship of 3,000 to 3,500 tons in 30 to 35 working hours (1148);
  - unloading by day only (1160);
  - ships often left part full (1131, 1166).
- The ammunition rows (units of fire) give no figure in days.
- The Japanese rows give what a run carried (two weeks' rations a destroyer, held 170 and 178) and the stocks they planned (20 days, 1178; 23 days, 1185). They give no interval.

**What is the architect's choice:**

- whether the game's convoy is the "regular shipment" of the doctrine, so that the figure is the use of one interval, or a forward-area run like those to Guadalcanal, which the rows do not size;
- whether to take the Army's figures for a rear base or the shorter intervals of fuel and of the fleet.

The 14 days proposed in question 5 sits just below the 15-day fuel interval. It is below every operating level the rows state.

### The floor: the days a commander keeps at a base

**What the rows say.**

- The War Department's minimum for the South Pacific bases was 120 days of rations and 90 of the other classes in 1942, and 90 for all classes in 1943 (1063, 1055).
- Other 1942 bases had 60 to 120 days (1071 to 1073).
- The Marine division, after Guadalcanal, recommended embarking 30 days of supply and 50 of rations for a landing (1144, held 453). It recommended landing only about five days at first (1147).
- The Japanese planned 20 days for the 38th Division (1178) and 23 days for the force awaiting evacuation (1185).

**What was held at Guadalcanal.** In practice the forward base held far less than any of these:

- 3 to 37 days of rations in August, by source (1102, held 414 to 419, 710);
- under 6 days of aviation gasoline (1152);
- two days of operations on 15 October (238).

**The range the rows support.** For a base in a rear area, 60 to 120 days. For a force landed in the forward area, 30 days of supply and 50 of rations embarked, by the division's recommendation. Scenario 36's 30 days falls in the second range.

**What is the architect's choice:**

- which of these a game base stands for;
- whether the floor should be the War Department's minimum, a reserve "drawn on only in emergency" (1056), or the much lower stock a contested base actually held.
