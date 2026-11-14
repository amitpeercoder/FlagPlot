# Land forces on Guadalcanal and Tulagi, 7 August to 31 October 1942

Status: research report for the architect, read on 2026-11-12. Stage 0 of the plan in `docs/design/ground-combat.md`. Nothing here is settled: every row is `unconfirmed`, and where sources disagree each has its own row.

## What is in the files

| File | Rows | What it holds |
| --- | --- | --- |
| `land_units.csv` | 96 | The formations, echelons and task groupings the sources name, and six "forces at a place" entries for everything a side had ashore |
| `land_unit_facts.csv` | 812 | Strength in men by date, where and when landed, what the other side believed, rations, sickness, commanders |
| `lift_entities.csv` | 55 | The ships, ship types, transport groups and runs the lift rows are about |
| `troop_lift_facts.csv` | 110 | Stated troop capacity, men carried on a named run, the ships that made it |
| `engagement_outcomes.csv` | ids 1912 to 2275 | Men committed and lost in each action, and whether an attack held ground |
| `gaps.csv` | 14 new | What was looked for and not found |
| `sources.csv` | 5 new | Phases I, II and III of the division's final report; Japanese Monograph 45; Reports of General MacArthur, volume II |

## How it was read, and how far to trust it

Six groups of sources were read by researchers, one group each: Hough's Marine Corps history, Zimmerman's monograph, Miller's Army history, Shaw's short history with the 1st Marine Division's final report (Phases I, II, III and V), Dyer's history of the amphibious force with the ship dictionary, and the Japanese-derived accounts (Japanese Monograph 45, MacArthur's Reports volume II, the Strategic Bombing Survey).

**Every quote was checked by a program against a downloaded copy of the source.** The first readers used a web tool that returned paraphrases, and only about half of their quotes for Hough and Miller were the source's words. Those two, Zimmerman, Dyer and the Japanese-derived accounts were therefore read a second time from local text, and all 1,294 rows that went forward carry a quote found word for word in the source file. A row whose quote could not be found was dropped.

**What that check does not cover.** It shows the words are in the source. It does not show that the number was filed against the right unit or date. That filing was done by clerks from each researcher's own label for the row, and I looked through the main units by eye and corrected what I saw. Each note ends with the label the researcher used, so a misfiled row can be seen for what it is. Expect some: the same regiment appears under many names, and a figure for part of a force is easily taken for the whole.

**Parts and wholes.** A strength row was tagged as the whole body, a part of it, officers or enlisted only, a total over a period, or a planned figure, and the attribute says which (`strength_men`, `strength_men_part`, `strength_officers`, `strength_enlisted`, `strength_men_cumulative`, `strength_men_planned`). Officers and enlisted are never added here. Only the plain attribute should be read as a unit's strength.

**Whose figure.** A figure for a Japanese unit from an American history is the Americans' figure unless its note names a Japanese record. Each note on a Japanese unit says which kind of source it is. An estimate of the enemy is filed on the unit that was estimated, as `strength_men_estimated_by_enemy`, with the side that made it in the note.

**The division report is a scan.** Its tables are often garbled in the text layer. Those rows are graded low and say so. The page images were not read.

**Three rows I entered myself** from the ship dictionary pages, with five troop capacities and two destroyer-transport loads; I read each on the page.

## What the sources say

### 1. The Japanese on 7 August

The two Japanese-derived accounts disagree with each other. Monograph 45: a naval guard unit of about 240 men and a construction unit of about 2,700 on Guadalcanal; about 40 naval air personnel, a guard unit of about 200 and a construction unit of about 200 on Tulagi and Gavutu. MacArthur's Reports: 250 naval garrison troops and two construction units of about 1,600 men on Guadalcanal.

American counts after the landing: about 2,230 on Guadalcanal and 780 in the Tulagi group (Miller, Dyer); 2,571 on Guadalcanal out of 3,457 in all (Shaw); 2,900 present (Zimmerman's appendix); 430 sailors and 1,700 laborers (Miller); 1,850 on Guadalcanal from the 17th Army's record as Miller cites it. The division's intelligence annex, from prisoners: about 600 armed men and two labor units of about 900 each.

### 2. What the Americans expected

Ghormley's staff: 3,100 (Miller says both 3,400 and 3,100, on different pages). Turner's staff: 7,125, of which 1,850 in the Tulagi group and 5,275 on Guadalcanal. The division on 20 July: 8,400. The division's intelligence annex: 5,010, which it calls "a heavy over-statement". All are rows, dated where the source dates them.

### 3. What the Japanese believed

Few numbers. Early August: 1,000 Americans ashore (Miller), or a reconnaissance in force of perhaps 2,000 (Shaw); Imperial General Headquarters took it for "nothing more than a reconnaissance in force" (MacArthur's Reports). On 19 September the 17th Army believed 7,500 Americans held Lunga Point (Miller). For October Zimmerman gives "slightly in excess of 10,000". No Japanese-derived source read gives a number of men for the Tenaru or the ridge.

### 4. The landing force and its reinforcement

- Embarked: 19,546 or 19,105 by the division's records, 18,722 by the amphibious force; "almost 19,000" and "about 19,500" in the histories.
- Ashore by 9 August: 10,900 on Guadalcanal (also "about 10,000" and "nearly 11,000") and 6,075 on the Tulagi side. About 1,390 of the 2d Marines were carried away in the transports.
- The 7th Marines, 18 September: 4,262 (Hough, Zimmerman), 4,180 (Miller), or 215 officers and 3,450 enlisted (the division's personnel annex, which dates it 19 September).
- The 164th Infantry, 13 October: 2,852, 2,850 or 2,837.
- On Guadalcanal: 19,251 at the end of September and 23,088 after 13 October (Miller).

### 5. The Japanese forces sent

- **Ichiki Detachment:** a 2,000-man force; the first echelon of about 900 or 1,000 landed at Taivu Point on the night of 18 August from six destroyers; a little over 100 survived the Tenaru. The rest (850 in Zimmerman, about 300 in one passage of Hough) came later.
- **Yokosuka 5th Special Naval Landing Force:** about 500 landed near Kokumbona on 16, 17 or 18 August, by source.
- **Kawaguchi Detachment (35th Brigade):** over 6,000 in all (Hough, Miller); about 5,200 put ashore between 30 August and 7 September (MacArthur's Reports); 3,090 for brigade headquarters and the 124th Infantry in Zimmerman's table.
- **October:** about 20,000 Japanese ashore by mid October (22,000 and 25,000 to 28,000 in a footnote of Miller); 5,600 in the inland attack, about 2,900 in the coast force, 1,200 in Oka's column.
- Monograph 45: about 6,200 men took part to the end of September, with about 1,200 lost.

Men landed echelon by echelon are not given by any source read.

### 6. The attacks

| Action | American losses | Japanese losses | Ground |
| --- | --- | --- | --- |
| Tulagi, Gavutu, Tanambogo, 7 to 9 August | 144 dead and 194 wounded (Miller, Shaw); the division report differs | The garrison nearly destroyed | Taken and held |
| The Tenaru, 21 August | 34 or 35 killed, 75 wounded | Nearly 800 killed, 15 prisoners; "casualties numbered almost 800; only 130 survived" | None taken |
| The ridge, 12 to 14 September | 31 killed, 103 wounded, 9 missing (Miller, Zimmerman); 59, 194 and 10 (Shaw) | About 600 dead on the ridge; the Japanese reported 633 killed and 505 wounded | None held |
| The Matanikau, 23 to 27 September | 60 killed, 100 wounded; 42 and 129 in the task force diary | Not given | American attack failed |
| The Matanikau, 7 to 9 October | 65 killed, 125 wounded | Nearly 700 of the 4th Infantry, from a Japanese diary (Hough, Miller); 690 killed (the division report); more than 700 (Shaw) | A raid; not held |
| The October attack, 23 to 26 October | About 300 killed and wounded | 2,200 by the division's count, with about 1,200 more on the Matanikau; about 3,500 in Hough and Shaw; 350 killed, 500 wounded, 200 missing in a captured diary | None held |

Men committed are mostly words ("two battalions", "about battalion strength") and are kept as text rows. The Japanese counts of their own losses are the ridge report and one diary; everything else is an American count or estimate.

The older rows 1742 to 1748 (Phase IV of the division report) use the plain measures `killed` and `wounded`. The new rows say whose men they count (`us_killed`, `jp_killed`, and so on), and each note begins with whether it is the side's own figure or the other side's count.

### 7. Men carried by sea

- **Stated capacity:** American Legion 1,644; President Jackson and President Hayes 1,388; President Adams 1,350; Neville 1,150. The division's station lists of 2 August show American Legion carrying 1,777 and Neville 1,199, so ships sailed above the stated figure.
- **Destroyer-transports:** no capacity stated. Loads: Little 154 (partly legible), Manley 200, McKean 185 (in 1943); about 30 tons of cargo each.
- **Japanese destroyers:** run totals only. Six destroyers with 900 to 1,000 men on 18 August; 900 at Tassafaronga on 29 August; 1,200 from seven destroyers on 31 August; 200 from one destroyer on 16 August; "as many as 900 troops ... in a single night" (the Strategic Bombing Survey). The Lunga defenders reckoned 150 men a destroyer, which is their estimate and is filed as such.
- **Japanese transports:** Kinryu Maru with 800 or about 1,000 men; the convoy of 19 to 25 August with 700 Army and 800 naval troops, or about 1,500, which did not land.
- **The evacuation,** as the end state: 4,940, 3,902 and 1,730 men on three nights in February 1943, by 18, 17 and an unstated number of destroyers.

### 8. Supply

- **American:** 60 days of supplies embarked; 37 days of food ashore on 9 August (Hough), or 17 (Shaw); the inventory of about 15 August gave 17 days of field rations, 3 of emergency rations and 10 of captured Japanese food; two meals a day from 12 or 15 August; 120 tons of rations by three destroyer-transports on 21 August, enough for about three and a half days (the online text garbles the fraction, so that row is graded low); 1,012 tons of rations on 18 September, after which full rations returned; 70 days' rations with the 164th Infantry.
- **American sick:** malaria from the third week of August; 1,960 in hospital in October; the campaign totals are rows marked cumulative.
- **Japanese:** words for the period (half rations of rice on the march, a few days' rations carried into each attack). The figures the sources give are for November and December and are in the rows marked as outside the window.

## Where the sources disagree most

1. The Japanese garrison on 7 August: 1,600 or 2,700 construction men in the two Japanese-derived accounts, and four different American totals.
2. The Tulagi garrison: 780, about 500 on Gavutu and Tanambogo alone, 1,000 or 1,500 by Marine estimates, and Monograph 45's three small figures (about 40, 200 and 200).
3. The 7th Marines as landed: 4,262, 4,180 or 3,665.
4. The ridge: 31 or 59 Marines killed.
5. Rations on 9 August: 37 days or 17.
6. Tulagi's garrison of Marines at the end of September: the scan of the division report reads 8,260 and Miller, citing it, prints 3,260.
7. Monograph 45 dates the Ichiki landing 18 August in one place and 13 August in another.

## What was not found

In `gaps.csv`, with where each would be found. The ones that matter most to the plan:

- **No Japanese source for the garrison's units,** for men landed echelon by echelon, or for Japanese rations and sickness inside the period. Japanese Monograph 34 (the 17th Army) is what Miller cites and is not online; the official history's site was down.
- **No stated troop capacity for a destroyer or a destroyer-transport,** either navy, and none for a barge. A derived capacity for those classes will have to rest on run totals.
- **Per-ship loads on 7 August** are in the division report's tables but legible for five ships only. The page images would give the rest.
- **Strength of each Marine unit after the landing,** and men committed by the Americans in each action.

## What `oob-check` does and does not check

- It holds the three new fact files to the sourcing rule, because they have a `standing` column: a source, a locator, a date read, a known confidence, basis and standing, and a note on every row.
- It does not know `land_units.csv` or `lift_entities.csv`, so it does not check that a fact row's entity exists, that an attribute is in a vocabulary, or that two chosen rows agree. The attributes are not in `attributes.csv`.
- The attack rows in `engagement_outcomes.csv` are checked as that file always is. Their measures are new names.

Attributes used in `land_unit_facts.csv`: `strength_men` and its tagged forms; `strength_men_estimated_by_enemy`; `landed_men`, `landed_at`, `landed_on`, `carried_by`; `departed_on`, `ceased_on`; `commander`, `parent_stated`; `rations_days_on_hand`, `ration_scale_note`, `supply_tons_landed`, `supply_note`; `sick_men`, `evacuated_men` and their tagged forms; `estimate_note` and `note` for statements with no number. Added on 2026-11-14 (`outside-research-nov-14.md`): `prior_combat_note`, `training_note`, `replacements_men`, `amphibious_training_note`, `terrain_training_note`, `heavy_weapons_ashore_note`, `heavy_weapons_left_note`, `quality_judgement_note`, `strength_riflemen`; `position_on`, `march_distance_miles`, `march_days`, `attack_on`, `attack_planned_on`, `carried_note`; `directive_on`, `warning_on`, `embarked_on`, `sailed_on`, `rendezvous_on`, `rehearsal_from`, `rehearsal_to`, `loading_note`, `vehicles_left_share_percent`, `preparation_note`, `intelligence_note`.

Attributes used in `troop_lift_facts.csv`: `troop_capacity_men`, `troops_carried`, `ships_in_run`, `supply_tons_carried`, `troops_lost`, `crew_complement`, `troops_per_ship_estimated_by_enemy`, `lift_note`.

## For the architect

- The transports are not in `ships.csv` or `ship_classes.csv`, so their capacities are in a file of their own. Moving them to a ship class attribute needs the classes added first.
- A measure for "each attack the record names failed" can be read from the `jp_attack_held_ground` rows: every one is 0.
- A measure for how fast an unfed force wasted has nothing inside the period to rest on from these sources; the figures begin in November.

## The transports' loads, read from the page images (12 November 2026, second pass)

The division report's station list of 2 August 1942 (Phase I, Annex K) gives the Marine and Navy personnel of the landing force aboard each ship. The scan's text layer garbles it; the page images are clear, and I read them myself. Rows 111 to 134 of `troop_lift_facts.csv`.

| Ship | Type | Men aboard, 2 August | Stated troop capacity |
| --- | --- | --- | --- |
| American Legion | AP | 1,777 | 1,644 |
| Hunter Liggett | AP | 1,611 | not found |
| Barnett | AP | 1,404 as typed; the block adds to 1,494 | not found |
| McCawley | AP | 1,428 | not found |
| President Hayes | AP | 1,346 | 1,388 |
| President Adams | AP | 1,334 | 1,350 |
| George F. Elliott | AP | 1,328 | not found |
| President Jackson | AP | 1,295 | 1,388 |
| Neville | AP | 1,199 | 1,150 |
| Fuller | AP | 1,167 | not found |
| Heywood | AP | 1,158 | not found |
| Crescent City | AP | 687 | not found |
| Alchiba | AK | 539 | |
| Libra | AK | 280 | |
| Bellatrix | AK | 255 | |
| Fomalhaut | AK | 227 | |
| Alhena | AK | 160 | |
| Colhoun, Gregory, Little, McKean | APD | 157, 155, 154, 149 | none stated |

- **A destroyer-transport carried about 150 men:** the four carried one Raider company each with its attachments. The fifth Raider group, 213 men, had no destroyer-transport and was split between Neville and Heywood, outside those ships' figures.
- **Ships sailed over their stated capacity:** American Legion by 133 and Neville by 49, on the ship dictionary's figures; the three President ships sailed under theirs.
- **The grand total is 18,134,** without the 3d Defense Battalion in Zeilin and Betelgeuse, for which the list says "No figures available".
- **A correction.** An earlier row (71) from the garbled text gave Fomalhaut 539. That is Alchiba's total; Fomalhaut's is 227. The old row is marked `disputed` and says so.
- **Barnett.** The typed total is 1,404, but the block's own columns add to 1,494 and the group total needs 1,494. Both are rows; the typed figure is graded low.

These are loads on one voyage, not capacities. With the five stated capacities they are what a derived troop capacity for a transport class can rest on.

