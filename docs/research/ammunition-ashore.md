# Ammunition for men ashore

Read on 2026-11-21 for the architect, who is to plan a rule for ammunition for land units: today a unit cut off from supply fights on without limit. The note asks five things of the record, at the scale of the game (units of hundreds to thousands of men, days, tons at a base):

- A. what a force landed with and held;
- B. how fast a fight used it;
- C. what running short did;
- D. ammunition's share of supply by weight;
- E. captured ammunition, and the difference between the sides.

Each statement below is a row in `data/oob/land_unit_facts.csv`, ids 15082 to 15240, all `unconfirmed`. Their entities are `land_units.csv` 14934 to 14953 and their new sources are `sources.csv` 14930 to 14940. The gaps are the last 28 entries of `gaps.csv`.

How the quotations were checked:

- **150 rows** were compared by program with a fresh download of the page or file the row cites, after the researchers had checked them once against earlier downloads. All 150 were found word for word, after collapsing white space and, for web pages, stripping the markup. For the PDFs (McCarthy, Bullard, the two NIDS papers) the text was taken from the fresh PDF with pdftotext.
- **Nine rows** are tables of the 1941 FM 101-10 and the 1942 FM-E 101-10 (15082 to 15087, 15099, 15238, 15239). The scans' OCR scrambles the columns, so the lead read each figure by eye from the page image. The quotation in those rows is a transcription of one printed line, and the row's note says so.

**In short:**

- **What a unit landed with.** The record gives the American unit of fire by weapon and weight for 1941. A triangular-division infantry regiment's unit of fire of small arms, mortars and 37-mm weighed 72.4 tons. Twelve 105-mm howitzers' unit of fire weighed 68 tons.
- **The planning rate.** The Army planned on 1.5 units of fire for the first day of an attack or defence, 1.0 a day after, and 0.2 a day when inactive.
- **What the Marines had.** They landed at Guadalcanal with about 4 units of fire. They received no more ammunition for 42 days, through the Tenaru and the Ridge.
- **Men who carried their supply on their backs.** Japanese and Australian infantry carried about 100 to 330 rounds a rifle. One Australian company used all of it in a day.
- **What a fight used.** The record gives artillery expenditure for most of the named Guadalcanal fights. It gives no count of small-arms, mortar or grenade expenditure for any of them.
- **What running short did.** Units short of ammunition rationed fire and stopped answering it. Units out of it withdrew where they could.
- **What did not happen.** No row has a unit that ran out of ammunition attacking with the bayonet. In these cases surrender came from other causes.

## What was read

- **Doctrine and planning figures:**
  - FM 101-10, *Staff Officers' Field Manual: Organization, Technical, and Logistical Data*, editions of 15 June 1941 and 1 August 1945 (archive.org; for the 1941 tables, the page images). The HyperWar PDF of the 1941 edition is the same scan.
  - FM 100-10, *Administration*, 15 November 1943.
  - FM-E 101-10, *Enemy Forces* (1 October 1942), the Japanese part.
  - TM-E 30-480, *Handbook on Japanese Military Forces* (1944), chapter VIII (HyperWar).
- **Guadalcanal:**
  - Miller, *Guadalcanal: The First Offensive*, chapters II to XIV.
  - Hough and others, *Pearl Harbor to Guadalcanal*, Part VI.
  - Zimmerman, *The Guadalcanal Campaign*, chapters IV to VIII.
  - The 1st Marine Division's Final Report, phases I to V (archive.org OCR of the DTIC scans).
  - Japanese Monograph 45.
  - Saito's two NIDS papers (2013 and 2024).
- **Papua and Milne Bay:**
  - Milner, *Victory in Papua*.
  - McCarthy, *Kokoda to Wau*, chapters 4 to 10 (AWM chapter PDFs).
  - Bullard's translation of the Japanese official history, *Japanese Army Operations in the South Pacific Area*. No copy hosted by the AWM was found; the PDF read is a copy on another site, whose title pages name the AWM. The AWM's catalogue record is cited in the source row.
- **Other cases:**
  - Heinl, *The Defense of Wake*.
  - Morton, *The Fall of the Philippines*.
  - The 1st Airborne Division's war diary (Pegasus Archive transcription).
  - Cole, *The Ardennes*, chapters XVI, XVII, XIX and XXI.

**Could not get or did not read:**

- **US manuals:** a 1942 or 1943 edition of FM 101-10, or its changes; none was found on archive.org or by search. The 1945 edition's daily-requirement table could not be placed by column from the OCR, and its page image was not read.
- **Japanese records:** Frank's *Downfall*, which Wikipedia cites for the contents of a kaisenbun in rounds; no row rests on Wikipedia. Tactical and Technical Trends. JM-98 has no text layer; a researcher's own OCR of it found nothing new.
- **Arnhem:** the Pegasus Archive's 2nd Parachute Battalion and Frost pages (both return "not found"). The war diary's daily tables of rounds and tons, which the transcription flattens.
- **Not searched this pass:** Leighton and Coakley, though rows 1074, 1078 and 1089 already come from it.

## Already on file before this pass

These rows were found by a search of `data/oob` for "ammunition", "unit of fire" and "u/f". The new rows do not repeat them; where a second source says the same thing, it has its own row.

- **The Marines' load:**
  - 1st Marine Division: 60 days and 10 units of fire embarked, against a normal 90 days and 20 units (448 to 450, 452, 887, 915, 762).
  - Three units of fire per transport (410).
  - About 4 units of fire ashore when the transports left on 9 August (777, 464, 414, 903, 416). Shaw has "four days' supply of ammunition" instead (4054).
  - Ghormley's "only 5 units of fire" on 16 August (1102, 1103, place_facts 919).
  - Less than half of the 60 days and 10 units unloaded (465, 900).
- **Resupply on Guadalcanal:**
  - No ammunition received until the 7th Marines on 18 September, with about 10 units of fire (1154, 628, engagement_outcomes 1905, place_facts 920).
  - The Marines were ashore 38 days with no ammunition resupply (733).
  - The 164th Infantry brought 5 units of fire (357, 712).
  - The division asked for a level of 16 units of fire of 75-mm and 17 of 105-mm in October (1156).
  - Edson's Ridge: 1,992 rounds of 105-mm (engagement_outcomes 4114).
- **The Japanese on Guadalcanal and the Japanese Army generally:**
  - Each soldier carries 2 days of provisions and ammunition (1044); up to 20 days in the hills (1045).
  - A division leaves with one kaisenbun, 4 months and 3,000 tons of ammunition (1048).
  - Stock levels: ammunition for one campaign (1047).
  - 4.2 lb of ammunition a man a day for active operation, against 5.17 for the American (1038).
  - The October convoy landed only 10 to 20 percent of its ammunition (1050).
  - 18 percent of the ammunition carried from Rabaul reached the delivery points (1052).
  - 260 boxes of ammunition landed on 15 November (109, 193, troop_lift 828).
  - The Japanese lost all but 10 percent of their ammunition (180).
  - A stockpile planned for 60,000 men for 20 days (1178).
  - In December, offensive action was impossible: "ammunition stocks were too low" (14927).
- **Cases elsewhere:** the Marauders at Walawbum and Tingkrukawng (15066 to 15068), Blackpool (15075), and Urquhart's signal at Arnhem (15080).

## A. What a force landed with and held

| What | Figure | Rows |
| --- | --- | --- |
| US Army unit of fire, small arms, 1941 | rounds a weapon: rifle 150, automatic rifle 750, light MG 2,000, heavy and .50 MG 3,000, SMG 200, pistol 20, 37-mm AT 120, 60-mm mortar 400, 81-mm 60 heavy or 240 light | 15082 |
| US Army unit of fire, artillery, 1941 | rounds a piece: 75-mm 300, 105-mm 225, 155-mm howitzer 150. Tons a piece: 3.45, 5.62 and 7.875. 12 pieces of 105-mm: 2,700 rounds, 68 tons | 15084 |
| One unit of fire for an infantry regiment, 1941 | 72.4 tons for small arms, mortars and 37-mm (2,099 rifles, 1,181 pistols and others) | 15083 |
| The same, 1945 | M1 rifle 278,700 rounds, 11.98 tons; 105-mm (cannon company) 1,200 rounds, 29.40 tons | 15092 |
| Prescribed load, 1941 | regiment 92 tons (about 2 units of fire for rifles, 328 rounds a rifle); triangular division 233 tons of small arms | 15099, 15085 |
| What a unit of fire is | set by the theatre commander (1941); "a balanced expenditure ... under conditions of normal action", not a day of supply (1943, 1945) | 15088, 15089, 15091 |
| 1st Marine Division, embarkation plan | 3 units of fire on each transport, 7 on the cargo ship | 15098 |
| 1st Marine Division ashore, August 1942 | 6,000,000 rounds of .30-caliber and 800 90-mm shells; 3 units of fire for the artillery battalions; ammunition first over the beach | 15101, 15100, 15102 |
| 7th Marines, 18 September | of about 10 units of fire landed, 5 to the regiment and the rest to the division reserve | 15122 |
| 25th Division, January 1943 | two units of fire for a 105-mm battalion weigh 135 tons, or 54 truckloads | 15130 |
| Japanese division, US estimate of 1942 | rounds a weapon carried in the division: rifle 330, heavy MG 15,475, light MG 2,970, 75-mm gun 580 (unit of fire 300), 105-mm 338, 75-mm mountain gun 216, 70-mm 240 | 15239 |
| Japanese soldier, US estimate of 1942 | a pack of 60 to 65 lb with rations and ammunition; 1 day's ration and 5 days' rice | 15240 |
| Japanese rifleman on Tulagi | 120 rounds in three belt cases | 15105 |
| Japanese "engagement of munitions" | about 2,000 field or mountain gun rounds, about 1,000 each of 15-cm howitzer and 10-cm cannon | 15155 |
| Japanese division engagement, 1945 plan | ground ammunition about 2,000 tons | 15156 |
| South Seas Detachment, Buna, 2 September | 150 rounds a battalion gun, 300 a rapid-fire gun, 180 a mountain or regimental gun; the mountain artillery limited to 200 rounds carried on men's backs | 15195, 15196 |
| Japanese 2d Division, October | fifteen 150-mm howitzers with "a limited amount of ammunition"; each man to carry one shell up the Maruyama trail; about 800 tons hand-carried | 15141, 15138, 15139, 15140 |
| Japanese landings, November | 38th Division: 4,200 men with rations and ammunition by 20 destroyers; 15 November: 4,000 men, 5 tons of supplies, 260 cases of ammunition; the convoy carried some 10,000 tons | 15143, 15144, 15145 |
| Australians, Kokoda Track | about 100 rounds and two grenades a man (Kokoda, 9 August); 100 to 150 rounds a rifleman; a battalion carried one 3-inch mortar with 15 or 24 bombs and a Vickers with 3,000 rounds | 15180, 15185, 15191 |
| Wake, Bataan, Corregidor | Wake: "enough for limited initial operations"; Bataan: about 15,000 tons on 8 December 1941 and 15,000 more shipped in; Corregidor: seacoast ammunition ample, little for land targets | 15159, 15164, 15172 |
| 1st Airborne, Arnhem; 101st Airborne, Bastogne | Arnhem: first-line scales plus extra in containers and gliders; Bastogne: pouches "not quite full" and a few truckloads of 105-mm | 15220, 15222, 15229 |

## B. How fast a fight used it

Artillery expenditure by the 11th Marines comes from the division's Final Report. The report calls its own figures "by no means precise" (15111).

| Action | Days | Ammunition used | Rows |
| --- | --- | --- | --- |
| First Matanikau, 18-19 August | 1 | 240 rounds of 75-mm, about 200 of 105-mm | 15112 |
| Tenaru (Ilu), 21 August | about 6 hours | 375 rounds of 75-mm, 30 of 105-mm (artillery only) | 15110 |
| Edson's Ridge, 13-14 September | 1 night | about 2,000 rounds of 105-mm; 878 of 75-mm on the flanks; grenades carried down by hand | 15113, 15114, 15115 |
| Matanikau, 20 and 27 September | 1 each | 418 rounds of 105-mm; 941 of 75-mm and 341 of 105-mm | 15157, 15158 |
| Matanikau, 7-9 October | 3 | 2,188 rounds of 75-mm, 1,063 of 105-mm; mortar ammunition ran low | 15120 |
| Matanikau, night of 23 October | 1 night | 6,164 rounds of 75-mm, 2,719 of 105-mm | 15123 |
| Henderson Field, 24-26 October | 2 nights | M Company, 7th Marines: 1,200 rounds of 81-mm in one night; 37-mm canister, no count | 15124, 15125 |
| Kokumbona, 10 and 23 November | 1 each | 4,570 rounds of 75-mm, 549 of 105-mm, 212 of 155-mm gun; 1,546, 655, 293 and 134 | 15128, 15129 |
| Galloping Horse, 22 January 1943 | 1 | division artillery 12.5 tons into one zone and 55.5 tons on Hill 87 | 15131 |
| Brigade Hill (Kokoda), 8 September | 1 | one Australian company used 1,200 grenades and 100 rounds a man, then the battalion reserve | 15186 |
| Buna and Sanananda | 1 each | a 700-round mortar preparation; 250 rounds of 25-pounder and 750 of 81-mm before an attack | 15210, 15211 |
| Quinauan and Anyasan, Bataan | weeks | one artillery battalion (8 guns), about 5,000 rounds | 15170 |
| Corregidor, 4 May 1942 (Japanese) | 1 | 16,000 shells of all calibres in 24 hours | 15177 |
| Wake, 11 December 1941 | 1 hour | one 5-inch battery, 120 rounds | 15161 |

**Planning rates:**

- **Units of fire a day of combat (FM 101-10, 1941).** For small arms:
  - attack or defence of a position: first day 1.5, later days 1.0;
  - meeting engagement: 1.0;
  - pursuit or retirement: 0.5;
  - inactive: 0.2.

  For 75-mm and 105-mm artillery: 2.0 on the first day of an attack or a defence, and 0.2 when inactive (15086). The manual says these rest on statistics "from World War sources".
- **Artillery rounds a gun an hour, for periods under 6 hours (1941).** 105-mm: 120 in a preparation, 100 in the first 2 hours of an attack and 60 after, 100 in defensive fires (15238).
- **Pounds a man a day (1945):** ground ammunition 5.14 lb in an amphibious theatre and 3.64 lb in a continental one, averaged over all forces (15093, 15094).
- **Japanese:**
  - Allied estimate for the Southwest Pacific: as low as 1 lb a day (15095), against the Japanese 4.2 lb already on file (1038).
  - Total maintenance 10 to over 30 lb a man a day (15096).
- **Kokoda:** air supply was planned at 10 lb a man a day of all classes, losses included (15184).

**Resupply and how it worked:**

- **Air drops.** Only 15 percent of the ammunition dropped by B-17 to the 35th Infantry in January 1943 was usable (15133).
- **Carrying.**
  - A pack-howitzer battery needed its whole strength of 193 men and 117 mules to move four guns and 200 rounds (15134).
  - Beyond Hill 35 everything was hand-carried (15132).
  - Japanese front-line units sent men to delivery points on the coast to collect ammunition (15146).
- **Air supply to the 32d Division.** It flew in 2,450 tons of all classes over its time in combat. Its best day was 178 tons (15205, 15206).
- **Bastogne.** On 23 December 241 aircraft dropped about 1,200 lb each, and "the bulk" of the air cargo was artillery ammunition (15233, 15234).
- **Arnhem.** Air resupply put 106 tons in the division's hands, 7.4 percent of what was dropped (15228).

## C. What running short did

| Case | Cut off or short since | What the source says it could then do | Rows |
| --- | --- | --- | --- |
| 1st Marine Division, Guadalcanal | no ammunition received 7 August to 18 September | held; shortages of 81-mm, 37-mm canister and signals after the Ridge; asked for 37-mm canister and grenades on 22 August, which first came on 18 September; shortages "plagued the Division from the battle of the Tenaru" until late November | 15116, 15117, 15127 |
| Company D, 1st Battalion, 7th Marines, 26 September | cut off | one 81-mm mortar and about 50 rounds | 15118 |
| Puller's battalion, 27 September | in a fight | fired mortars until the ammunition was exhausted, then withdrew by order | 15119 |
| Berry's line, Bataan, 24 January 1942 | supply route cut | "ammunition was short"; artillery support lost when Fowler's ammunition was exhausted; withdrew | 15166, 15167 |
| Bataan, Scouts at Quinauan | weeks | machine guns little used because ammunition came by hand; mortars with little or no ammunition | 15168, 15169 |
| Bataan's end, 8-9 April 1942 | three months | dumps of hundreds of thousands of rounds blown up: the force did not fall for want of ammunition | 15171, 15165 |
| Corregidor, April and May 1942 | months | short of the right kind of shell (about 1,000 personnel shells in March, 400 in April and May) | 15174, 15176, 15173 |
| Japanese 14th Army, April 1942 | none; by choice | kept artillery fire to a minimum to save ammunition for Corregidor | 15178 |
| Mindanao | blockaded | range firing banned | 15179 |
| 39th Battalion, Kokoda, 9-10 August 1942 | 1 day | food and ammunition "running low"; Symington would retire if nothing was dropped by 7 o'clock | 15181, 15182, 15183 |
| 21st Brigade, Brigade Hill and Nauro, 8-10 September | 2 days | company used all its ammunition and the battalion reserve in a day; two days later, short of food and ammunition, it broke into groups to get out | 15186, 15187, 15188, 15189 |
| 2/10th Battalion company, Milne Bay, 27-28 August | 1 night | about five rounds a man; fell back by stages, firing | 15192 |
| 3d Battalion, 128th Infantry, Buna, 19-20 November | at once (one day's supply carried) | .30-caliber ran dangerously low; an air drop called for | 15201, 15202, 15203 |
| Sanananda roadblock, December 1942 | about 12 days | "about to run out of ammunition" when a party broke through on 14 December | 15212 |
| Chagnon's 73 men, 4 January 1943 | short at the start | fought until all their ammunition was gone, then swam for it | 15213 |
| 105-mm howitzer at Buna | 400 rounds brought by air | silent for days after the first few hundred rounds | 15208, 15209 |
| 2d Parachute Battalion, Arnhem bridge, 19-20 September 1944 | 2 days | could not reply to fire "due to shortage of ammunition, which was now acute"; held six houses | 15223, 15224 |
| 1st Airborne Division, 21-25 September 1944 | 4 to 8 days | "running very low", then "increasingly short"; PIAT ammunition exhausted and over-run by tanks; withdrew across the river | 15225, 15226, 15227 |
| 101st Airborne, Bastogne, 22-26 December 1944 | from 21 December | ten rounds a tube; no more than five rounds a mission; one 155-mm battalion allowed a sixth of its starting rate | 15230, 15231, 15232 |
| St. Vith salient, 20 December 1944 | cut off from trains | seven rounds a 105-mm howitzer | 15235 |
| Japanese on Guadalcanal, October to January | from the landings | attacked with only machine guns besides hand-carried weapons (25 October); stocks "nearly exhausted" by mid-December; artillery seldom used and lacked shells to reach the airfield; the 38th Division "in short supply" | 15126, 15148, 15150, 15151, 15147 |
| Oka's group, Mount Austen, December | weeks | hand-carriers brought "a negligible quantity"; held a fixed position | 15149, 15150 |
| Kawaguchi Detachment, September | the Ridge | abandoned ammunition and the guns of two batteries | 15137 |
| Japanese, Papua beachhead, December 1942 | weeks | Tsukamoto's men short of all types of ammunition; Giruwa rationing mortar and rifle ammunition; Gona fell with a little ammunition left; Yasuda fought on after a submarine brought some | 15214 to 15218 |
| South Seas Detachment, retreat | October | discarded heavy weapons, then ammunition, then pistols | 15199 |
| Milne Bay landing force | from the first morning | almost all its ammunition, fuel and rations destroyed by air attack on the beach | 15193 |
| Kampfgruppe Peiper; 5th Parachute Division, December 1944 | days | asked for ammunition by air; rationed to seven rounds a howitzer a day | 15237, 15236 |

## D. Ammunition as a share of supply by weight

- **1941:** the day of supply for Classes I to IV is 24.0 lb a man a day. Ammunition is left out of it and counted in units of fire instead (15087).
- **1943 doctrine:** ammunition is "by far the greatest mass of supplies to be moved to combat troops" (15090).
- **1945, a mostly amphibious theatre:** Class V is 8.61 lb (ground 5.14, aircraft 3.47) of a total of 67.4 lb a man a day, and rations are 6.708 lb (15093).
- **1945, a continental theatre:** rations 7.169 lb, Class V 8.05 lb (ground 3.64) (15094).
- **Japanese, by an American estimate:**
  - Total maintenance 10 to over 30 lb a man a day (15096).
  - Ammunition as low as 1 lb (15095), or 4.2 lb by the Japanese estimate (1038).
  - "Very little supply movement except ammunition and rations" (15240).
- **Bastogne:** the bulk of the air cargo was artillery ammunition (15234).
- **Kokoda:** 10 lb a man a day of all classes, ammunition inside it, with no split (15184).
- **Guadalcanal:** no convoy or destroyer run on either side gives ammunition's tons against rations' tons (gaps).

## E. Capture, and the difference between the sides

**Capture:**

- **Tulagi.** The garrison's captured stores included 187,200 rounds of .25-caliber, 22,800 of .303, 880 hand grenades and 460 rifle grenades (15104). The garrison itself carried US M1903 rifles with ammunition captured at Guam (15106).
- **Kukum.** The Marines took over abandoned Japanese guns and used them to bolster their defences (15107). They fired 20 rounds from captured 3-inch guns at a cruiser on 19 August (15109). Hough calls the captured material "an important if unforeseen factor" (15108).
- **Isurava.** The Koiwai Battalion found a large Australian stockpile, ammunition included (15198).

**The two sides:**

- **US estimates of the Japanese division (1942):** 330 rounds a rifle carried within the division (15239), against the 1941 American prescribed load of 328 rounds a rifle in a regiment (15099).
- **Requirements:** the Japanese estimated 4.2 lb a man a day for active operation, against the American 5.17 (1038). The Allied estimate for the Southwest Pacific was as low as 1 lb (15095).
- **A Japanese view:** a Japanese evaluation admired "the abundance of ammunition" of the Americans (15154).
- **Delivery:** on average only 18 percent of the ammunition the Japanese carried from Rabaul reached the delivery points (1052).

## Where sources disagree

- **What the Marines held on 9 August.**
  - Miller and Hough say about four units of fire (777, 464).
  - Shaw says "four days' supply" (4054), which is not the same unit.
  - Ghormley says 5 units of fire on 16 August (1103).
- **When the 164th Infantry's 5 units of fire landed.** Miller gives 13 October (712). The division's report OCR reads "about November 5" (357).
- **Edson's Ridge.** Hough and Miller give 1,992 rounds of 105-mm (engagement_outcomes 4114). The division report gives "2000 rounds" (15113) and adds 878 of 75-mm (15114).
- **The October convoy.** Miller and Hough say 80 percent of the cargo was landed (troop_lift 418, 189). Saito, from a staff officer, says only 10 to 20 percent of the ammunition (1050).
- **The Japanese battle's worth of ammunition.**
  - Iwakoshi: one kaisenbun, 4 months and 3,000 tons for a division (1048).
  - JM-45, a 1945 plan: one engagement of a division, about 2,000 tons (15156).
  - Bullard's editor: an engagement of munitions in gun rounds only (15155).
  - FM-E 101-10: a unit of fire of 300 rounds for the 75-mm gun (15239).

  These may not be the same measure.
- **Japanese ammunition per man a day.** The Japanese estimate is 4.2 lb (1038). The Allied estimate for the Southwest Pacific is as low as 1 lb (15095). Both come from the same handbook.
- **Corregidor's 12-inch personnel shells.** Morton gives about 1,000 in chapter XXVII (15174) and 400 in chapter XXX (15176). This may be a stock drawn down between the two dates rather than a dispute.

## Gaps

These are the last 28 entries of `gaps.csv`.

- **No figure of small-arms, mortar or grenade expenditure** for any named Guadalcanal fight (the Tenaru, the Ridge, 23 to 26 October). The Phase IV appendix table of expenditure by engagement is not legible in the OCR.
- **No rounds a man or a gun** for the Ichiki and Kawaguchi forces, the 2d Division or the 38th Division as landed.
- **No Japanese rationing order** (rounds a gun a day) on Guadalcanal, and no order to fight with the bayonet for want of ammunition.
- **No ammunition share of tonnage** in any Guadalcanal convoy, for either side, or of the 32d Division's air supply.
- **No Marine Corps unit of fire** in rounds or tons for 1942: only counts of units.
- **No US planning figure of 1942** in tons a man or a division a day. The 1941 manual counts in units of fire; pounds a man appear only in 1945.
- **The kaisenbun in rounds** (Frank, *Downfall*, not read).
- **Bataan: no rationing rate.** Wake: no statement that ammunition ran out. Milne Bay: no Japanese tonnage.
- **Arnhem:** the daily tables. **Bastogne:** the tons of ammunition dropped.
- **Leighton and Coakley**, not searched this pass.

## The reader's arithmetic (not fact)

This section is the lead researcher's reading of the rows for the architect. It states no fact and settles nothing. Weights a round are worked from the 1941 table, packing included: 75-mm about 23 lb (3.45 tons for 300 rounds) and 105-mm about 50 lb (5.62 tons for 225 rounds). Where a strength in men is used, it is not in the rows and is marked.

**1. Days of fighting a unit carries when it lands**

- **The 1941 rates.** A unit with N units of fire can attack or defend for about N − 0.5 days at 1.5 and then 1.0 a day (15086), or sit for 5N days at 0.2.
- **The Marines.** About 4 units of fire is about 3 to 4 days of hard fighting by the manual, or 20 days quiet. In fact the Marines went 42 days on them, through the Tenaru and the Ridge, with shortages of particular kinds (81-mm, 37-mm canister, grenades) but not exhaustion. So the manual's rates, which rest on 1914-18 statistics, overstate what a defending force on Guadalcanal used. The average was nearer 0.1 unit of fire a day, with fights that used a fraction of a unit.
- **A force that carries its ammunition on its back.**
  - The Japanese man's 120 rounds on Tulagi and the Australian 100 to 150 rounds are under one American rifle unit of fire (150).
  - One Australian company emptied its 100 rounds a man and 1,200 grenades in a single day (15186).
  - The Japanese plan was two days carried on the man (1044).
  - Such a force holds about one day of heavy fighting on the man, and more only by what the division transport or carriers bring up. In the 1942 estimate of a Japanese division, the division transport roughly doubles the rifle's rounds (15239).
- **A rule might** carry a land unit's ammunition in days of fighting:
  - about 3 to 4 for a force landed with a 1942 American load ashore;
  - about 1 to 2 for a force that carries its own;
  - with a quiet day costing a small fraction (0.1 to 0.2) of a fighting day.

**2. Tons of ammunition per thousand men per day**

- **The weight of a unit of fire.** A 1941 infantry regiment's small-arms unit of fire is 72.4 tons for 2,099 rifles and 1,181 pistols (15083). That is about one weapon a man, so roughly 3,300 men; the strength is not in the rows. That gives about 22 tons per thousand men per unit of fire, before artillery. Twelve 105-mm howitzers add 68 tons per unit of fire (15084), and the 25th Division's 135 tons for two units confirms it (15130). With its artillery, an infantry division of the 1941 table is in the order of 500 tons per unit of fire, roughly 35 tons per thousand men. The division strength (about 15,000) is not in the rows.
- **By the manual's rates (15086):**
  - **Attack, and defence of a prepared position:** about 50 tons per thousand men on the first day and 35 after.
  - **Quiet:** about 7.
- **What was used on Guadalcanal was far less on most days and as much on a few nights:**
  - Edson's Ridge: about 60 tons of artillery ammunition in one night.
  - Matanikau, 23 October: about 140 tons in one night.
  - Galloping Horse: 68 tons in a day.
  - These are for divisions of roughly 10,000 to 20,000 men ashore. Over the 42 days without resupply the whole division averaged about 0.1 unit of fire a day, roughly 3 to 4 tons per thousand men.
- **Campaign averages:**
  - 1945, ground ammunition: 5.14 lb a man a day, about 2.3 tons per thousand (15093).
  - Japanese: 4.2 lb, about 1.9 tons, for active operation (1038), and as low as 1 lb, about 0.45 tons (15095).
- **A plausible scale for a rule:**
  - **Quiet:** about 0.5 to 3 tons per thousand men a day, the Japanese at the low end.
  - **Defence on a night of assault:** about 5 to 15 tons per thousand men, most of it artillery.
  - **Attack:** about 10 to 50, the top only with full American artillery.
  - Small-arms ammunition alone is under a tenth of these. The weight is in the guns and mortars.

**3. What a unit out of ammunition can still do**

- **Short, not out.**
  - Units rationed fire: rounds a gun a day, a ban on range firing, five rounds a mission (15179, 15231, 15235, 15215).
  - They stopped answering fire they could not afford (15223).
  - They held positions that were not pressed hard.
- **Out, or close to it.** Every unit in these rows withdrew where it could, or tried to get out (15119, 15167, 15187, 15192, 15213, 15226). One was over-run when its anti-tank ammunition was gone (15227).
- **Attack.**
  - The record has no unit that attacked with the bayonet because it had no ammunition.
  - The Japanese attacked on Guadalcanal with only their hand-carried weapons and machine guns (15126). Their artillery and ammunition had not come up, but they did not lack rifle ammunition altogether.
  - By December the Japanese judged attack impossible, "ammunition stocks were too low" (14927).
- **Surrender.** None of these cases was a surrender for want of ammunition. Bataan fell with its dumps full (15171). Wake fell to a landing. Arnhem withdrew.
- **A rule might** let a unit short of ammunition hold at reduced fire but not attack, and make a unit out of ammunition unable to hold a pressed position and withdraw if it can. The rows would not support a unit fighting on unchanged, nor one that surrenders at once.
