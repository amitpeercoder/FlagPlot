# 0101: The data of the August landings

Status: settled by the architect on Nov 14, 2026. It settles the data that decision 0093 left to "when the scenario is written", for the full slice lengthened to 39 days, 7 August to 14 September 1942. Data is the architect's to settle (CLAUDE.md, "Who decides what"); every choice has its reason and the rows it rests on. Nothing is built. Three points change what decision 0093 said and are marked **Changes 0093**.

Rows are cited by the `id` column of their table: LUF is `land_unit_facts.csv`, TL `troop_lift_facts.csv`, EO `engagement_outcomes.csv`, PF `place_facts.csv`, PP `place_positions.csv`. Every row cited is unconfirmed unless a derivation already reads it. The rows were gathered by a team and about sixty were checked by its lead; each is read again when the scenario file is written, and a row that does not say what this record says goes back to the architect.

## Where places fall

1. **Kokumbona and the Matanikau fall in the airfield's hex.** By their position rows (PP 60, 58, 59) Kokumbona, the Matanikau and Point Cruz lie in (2,-4), the hex of Lunga. Only Tassafaronga (PP 51) and Cape Esperance (PP 32) fall in (2,-5). **Changes 0093, item 4,** which put Kokumbona in (2,-5) by its name.
2. **A force the record lands at Kokumbona is put in (2,-5),** for the reason decision 0093 gave for Taivu: men in a hex are one body, and a force that lay 17 nm from the field for a week without a fight cannot be put among the men who held it. The land links file says so beside the link.
3. **Taivu Point and Tasimboko stay in (3,-4),** as decision 0093 ruled, though their positions (PP 52, 53) fall in Tulagi's hex.
4. **Kamimbo has no position row.** It is put in (2,-5) with Cape Esperance, which it lies beside. This is recorded as a gap, and the row is asked for.
5. **Land links on the real map are two:** (2,-4) to (3,-4) and (2,-4) to (2,-5). Tulagi's hex has none. No hex of the area has a terrain, so every march is through jungle (decision 0098, "As built").

## The first day

6. **The slice begins before the landing.** Day 1 is 7 August. Lunga and Tulagi are Japanese bases. The American bases there are formed by the landings.
7. **Forces at sea on the night of 6 August start at sea,** where the record puts them: the transports and their escorts off Guadalcanal and Tulagi, the carriers to the south. Every other force starts at the base it sailed from and is ordered out on the day the record gives. **Changes 0093, item 3,** which had every force start at a base: no base is a day's steaming from the beaches.
8. **The squadrons of Henderson Field arrive, and do not start there.** They come as arrivals on day 14, 20 August (PF 53; `air_unit_facts` 36 and 108 to 135). An arrival due at a base that is not its side's, or has no airfield, is not delivered and not kept, as an arrival to a group away at the yard is not. So the field must be taken and finished by then, which is what happened.

## The airfield

9. **Lunga has no airfield on day 1, and 13,763 of work done on one.** The strip was 2,600 ft graded and rolled on 7 August (PF 60) of the 3,778 ft it had when finished (PF 62): 20,000 x 2,600 / 3,778, cut to a whole number. The share rests on rows. The total, `airfield_work`, is a placeholder, so the figure is marked a placeholder too.
10. **What this gives, before any tuning.** The 1st Pioneer Battalion, about 660 men, makes 660 a day: the 6,237 left take ten days of work. With the field taken on day 2 it is built on day 11 or 12, 17 or 18 August. The record's rows say 17, 18 or 19 August (PF 199, 69, 200). Nothing was fitted to get this, and it is reported as a measure (below).
11. **Tulagi's base has no airfield** and no work done. Rabaul and Espiritu Santo have theirs.

## Land units

Every unit has experience 2, every skill 2 and works 2: no real unit has a value until the rule that reads the research statements is written (decisions 0097, 0098 and 0099). Fighting units and working units are as decision 0099 has them.

### American

| Unit | Kind | Men | Lands | Rows and reason |
| --- | --- | --- | --- | --- |
| Combat Group A (5th Marines less 2/5) | fighting | 4,306 | Lunga, day 1 | LUF 646, the division's own report of men landed |
| Combat Group B (1st Marines) | fighting | 4,561 | Lunga, day 1 | LUF 652, the same report |
| Support Group, Guadalcanal | fighting | 1,373 | Lunga, day 1 | The rest of the 10,900 ashore on Guadalcanal (LUF 689, 690; EO 801) after the two groups and the pioneers. 10,900 is the middle of the three totals and has two sources |
| 1st Pioneer Battalion | working | 660 | Lunga, day 1 | LUF 498. The other rows say 596, 500 and 490 |
| 1st Raider Battalion | fighting | 828 | Tulagi, day 1 | LUF 518, 519 |
| 1st Parachute Battalion | fighting | 395 | Tulagi, day 1 | LUF 487 |
| 2d Battalion, 5th Marines | fighting | 1,085 | Tulagi, day 1 | LUF 531, 532 |
| 2d Marines and the rest of the Tulagi side | fighting | 3,767 | Tulagi, days 1 to 3 | The rest of the 6,075 ashore on the Tulagi side (LUF 778; EO 802) |
| CUB One detachment | working | 123 | Lunga, day 9 | LUF 660, 661; TL 86, 87: 5 officers and 118 men, 15 August |
| 6th Naval Construction Battalion | working | 392 | Lunga, day 26 | LUF 606: 5 officers and 387 men, 1 September. The rows of 200 and 357 are of parts |

The three battalions that crossed from Tulagi to Guadalcanal do so by order on the record's days: 2/5 on day 15 (LUF 534), the Raiders and the Parachutists on day 25 (LUF 798, 513).

### Japanese

| Unit | Kind | Men | Where and when | Rows and reason |
| --- | --- | --- | --- | --- |
| Naval guard troops, Lunga | fighting | 240 | Lunga, from day 1 | LUF 317, the Japanese-derived account. The other is 250 |
| 11th Construction Unit | working | 900 | Lunga, from day 1 | LUF 1 |
| 13th Construction Unit | working | 900 | Lunga, from day 1 | LUF 4. The two together are 1,800, between the totals of 1,600 and 1,700 (LUF 67, 65, 68) and well under 2,700 (LUF 66) |
| Tulagi garrison | fighting | 500 | Tulagi, from day 1 | LUF 227 |
| Construction unit, Tulagi | working | 200 | Tulagi, from day 1 | LUF 63. With the garrison, 700, against 780 by American count (LUF 226) |
| Yokosuka 5th Special Naval Landing Force | fighting | 200 | (2,-5), day 10 | TL 37, 38: the men one named ship carried on 16 August. The other rows say 150 and about 500 |
| Ichiki Detachment, first echelon | fighting | 916 | (3,-4), the night of day 13 | TL 141, the most exact of rows that run from 900 to 1,000. Six destroyers (TL 138 to 140) |
| Force landed at Tassafaronga on 29 August | fighting | 900 | (2,-5), day 23 | TL 45, 148, which the derivation of the Fubuki class already reads |
| Kawaguchi Detachment, main body | fighting | 3,090 | (3,-4), in five runs: 1,200 on day 25, 150 on day 27, 1,000 on day 29, 370 on day 30, 370 on day 31 | TL 153, 154, 159, 161, 164, 166. The five runs with rows add to 3,090, which is the row for the headquarters and the 124th Infantry (LUF 310). It is one unit, landed in parts |
| Aoba Detachment, part | fighting | 637 | (2,-5), day 36 | TL 168, at Kamimbo on 11 September |

### Left out, each a gap and not a guess

- **The Oka group,** which landed near Kokumbona and attacked from the west on 13 and 14 September: no row counts it.
- **The Ichiki second echelon and the men in Kinryu Maru,** which sailed and were turned back on 25 August. The slice's placeholder convoy of that day stays as it is, with no men.
- **The 1st Engineer Battalion:** no row gives its men. The airfield is built by the pioneers alone, which the measure of decision 10 will show.
- **About 1,390 men of the 2d Marines carried back to Espiritu Santo on 12 August** (LUF 549): they stay on Tulagi.
- **The raid on Tasimboko of 8 September, the patrols, and sickness.**

## Orders, where the record gives a day

12. **The Americans attack on day 1** at Lunga and at Tulagi, and stand on the defence once the base is theirs.
13. **The Ichiki echelon marches for the airfield on the day it lands** and attacks when it arrives. On the placeholder march of three days it arrives on day 15, 21 August, the day of the record.
14. **The Kawaguchi main body is ordered to march on day 35,** so that it arrives on day 37, 12 September. The attack's date has rows and the day it set out has none (gaps, lines 1264 and 1265). When the march figure is tuned this order moves with it, and the record of the round says so.
15. **The forces in (2,-5) stay there.** Without the Oka group nothing west of the field attacked in the period.

## Ships that carry men

16. **The transports are placeholders.** **Changes 0093, item 8.** The American transports, cargo ships and destroyer-transports cannot be derived: no rule gives a transport its kind or its cargo, and most lack the facts a class needs (`docs/research/oob/troop-lift.md`). Until a record settles that, the slice carries American men and supply in the placeholder troop transport (600 men) and the placeholder fast transport (200 men), each named "Placeholder", enough of them to carry the men of each landing.
17. **The four Fubuki-class destroyers keep their derived troop figure** (classes 3009 and 3010). Every other Japanese run uses the placeholder fast transport, because no Kagero-class or Shiratsuyu-class destroyer has a troop figure. The real destroyers of those runs are not in the slice as carriers until they have one.

## Supply

18. **The Americans embark 60 days of food** (LUF 409): 10,900 x 0.005 x 60 = 3,270 t for Guadalcanal and 6,075 x 0.005 x 60 = 1,822 t for the Tulagi side, shared among the transports of each.
19. **The transports sail on day 3,** 9 August, as they did (LUF 773), with whatever they have not landed.
20. **Lunga's Japanese stock is 1,090 t.** Half is kept by the side that takes the base: 545 t, ten days for the Americans there, which is the ten days of Japanese food in the count of 15 August (LUF 417, 418).
21. **The supply runs with rows are in:** 120 t in four fast transports on day 9 (TL 70, 88), 2,000 t on day 15 (LUF 748; TL 60), and the cargo ship that brought the construction battalion on day 26, with a placeholder load.
22. **Japanese stocks are placeholders.** No row gives a Japanese ration, stock or tonnage in the period, and a destroyer has no cargo figure. A force put ashore with nothing would have a fed share of 0 and fire nothing, which the record does not support: Kawaguchi's men had three days of food left on 12 September (LUF 867). So the scenario states a Japanese base, off the enemy's map, in each landing hex, with ten days of food for the men who land there: 200 t in (3,-4) and 87 t in (2,-5). Tulagi's stock is ten days for its 700 men, 35 t. Each is marked a placeholder.

## Measures added to those of decision 0093

| Measure | The record | Judged |
| --- | --- | --- |
| The day the airfield at Lunga is built | 17, 18 or 19 August (PF 199, 69, 200) | Reported |
| The squadrons of 20 August are delivered | They were (PF 53) | Yes |
| Days of food ashore at Lunga when the transports sail | 17 to 37 (LUF 415, 414) | Reported |

## What it will show that is wrong

Beside the list in decision 0093:

- **The transports may land everything.** On the placeholder rates thirteen ships can put 60 days of food ashore in two days, where the record's landed less than half. It is measured and not forced.
- **The construction units stay at Lunga.** They do not hold the base and do not fire, but they stay in the hex, unfed, and take a share of the fire. In the record they left on the first day.
- **The west is quiet,** for want of the Oka group.
- **The pioneers build the field alone,** and in the record they also unloaded the ships.

## What it asks for

- **Research rows:** the Oka group's men; a position for Kamimbo, Gavutu and Kukum; the 1st Engineer Battalion's men; the day Kawaguchi's main body left Taivu; Japanese rations.
- **A record on deriving transports:** a kind, a cargo rule and the facts they need.
- **The rule that reads the research statements into experience and skills,** before any real unit differs from another.

## What it costs to build

- The scenario file, the land links file for the real map, the measures and the takes of decision 0093.
- Every day number in the slice moves by 14.
- Files remade: the slice's run baselines, the test of scenario 7's hashes, the export, saved-game and agent tests that read the live slice, and the recorded game that plays it. The frozen copy is not touched.
- The arrival rule of decision 8 is a small change to a rule, with its line in the rulebook.
