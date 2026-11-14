# 0101: The data of the August landings

Status: settled by the architect on Nov 14, 2026. It settles the data that decision 0093 left to "when the scenario is written", for the full slice lengthened to 39 days, 7 August to 14 September 1942. Data is the architect's to settle (CLAUDE.md, "Who decides what"); every choice has its reason and the rows it rests on. Nothing is built. Three points change what decision 0093 said and are marked **Changes 0093**. Amended the same day from the rows of `docs/research/oob/outside-research-nov-14-slice.md`: see "Amended from new rows" at the end.

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
- **The construction units stay at Lunga.** They do not hold the base and do not fire, but they stay in the hex, unfed, and take a share of the fire. In the record they left on the first day. *(Nov 14, 2026: no longer so. Since decision 0104 they fall back to (2,-5) with the garrison on day 1, as in the record, and reach it on day 4 (amendment 7). There they are fed from the stock of (2,-5), sized for them since amendment 8. Fed, they would have finished an airfield at the placeholder stock by their default job; since amendment 9 a scripted order sets them to work at the port, and nothing is built there (`docs/validation/full-slice-39-days-after-stock-fix.md`).)*
- **The west is quiet,** for want of the Oka group. *(Nov 14, 2026: no longer so. The Oka group is in (amendment 2) and attacks the field from the west on day 39.)*
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

## Amended from new rows

On Nov 14, 2026 a second research pass was read against its sources and keyed (`docs/research/oob/outside-research-nov-14-slice.md`). Where it fills a gap or bears on a choice above, this section rules, and it stands over the text above.

1. **Positions hold.** A second source for Kokumbona, Tassafaronga, Taivu Point and Tasimboko moves none of them to another hex (PP 86 to 89, 66, 72). Kamimbo now has a position and falls in (2,-5) (PP 73, 62), as item 4 put it. Gavutu and Tanambogo fall in Tulagi's hex and Kukum in the airfield's (PP 74 to 76).
2. **The Oka group is in, with 333 men.** The one source that counts it from the Japanese side has some 1,000 men setting out in barges and one third of them left under Oka's command on the island (LUF 916 to 918): 1,000 / 3, cut to a whole man. Lower-grade rows say about 1,000 landed and about 650 attacked (LUF 929 to 931, 925); they are kept as the dispute and the measures take the range. It lands in (2,-5) on day 32, 7 September, the day by which a row has it west of the perimeter (LUF 931), from placeholder fast transports, since barges have no class. It is ordered to march on day 37 so that it arrives on day 39 and attacks on 14 September, the day two sources give (LUF 1007, 949). Item 15 and the line "the west is quiet" no longer hold.
3. **The landing of 29 August stays as it is,** 900 men at Tassafaronga (TL 45, 148), which a derivation reads. Another source has 450 men and 300 men landed at Taivu that day (LUF 939 to 941; TL 173, 174). Nothing read shows whether they are the same landing told differently. They are not added: the main body's 3,090 is the row for its strength (LUF 310), whichever run brought which men.
4. **The 1st Engineer Battalion and the day Kawaguchi set out are still gaps.** No document read gives the battalion's men or the day the main body left Taivu. Items 10 and 14 stand.
5. **Japanese stocks are now sized from rows,** turned into tons by the game's charge for a man, and are no longer plain placeholders:
   - In (3,-4): two weeks of food for the Kawaguchi main body (LUF 975), 3,090 x 0.005 x 14 = 216.3 t; and seven days for the Ichiki echelon, the two days on the man, three in the unit and two at division that a Japanese officer gave as the practice (LUF 1044), 916 x 0.005 x 7 = 32.06 t. Together 248 t, cut to a whole ton.
   - In (2,-5): seven days by the same practice for the 2,070 men landed there (200, 900, 637 and 333): 72 t, cut to a whole ton.
   - Row 1044 states a practice and not what these units were issued, and the note on each stock says so. Tulagi's 35 t is still a placeholder: no row gives a quantity there.
6. **An arrival waits.** Item 8 is changed by decision 0103, item 5: the squadrons due on day 14 are delivered on the first night Lunga is American, has its airfield and is below the level of damage that refuses aircraft. The measure "the squadrons of 20 August are delivered" becomes "the day the squadrons of 20 August are delivered", reported against day 14.
7. **Falling back and surrender** (decision 0104) change what the slice shows: the garrison of Lunga marches west and the garrison of Tulagi surrenders. The line "the construction units stay at Lunga" no longer holds.
8. **The stock in (2,-5) counts the men who fall back there.** Amendment 5 sized it for the 2,070 men landed in that hex. The garrison of Lunga and its two construction units, 2,040 men, fall back to it on the first days (decision 0104) and were not counted, so they starved on a stock sized for others. By the same practice of seven days (LUF 1044): 2,040 x 0.005 x 7 = 71.4 t, and with the 72.45 t of amendment 5, 143.85 t: 143 t, cut to a whole ton. No row gives a Japanese stock west of the field, so it is still a placeholder resting on a stated practice. The men of the attacks who fall back there after the Tenaru and the ridge are not provided for: the record has them short of food.
9. **The construction units that fall back do not build.** With no order, a working unit at a base with no airfield builds one (decision 0099, 4), so the two construction units of Lunga, once fed in (2,-5), finished an airfield at the placeholder stock there, and aircraft with no deck landed on it. No row has them build anything after 7 August. The script gives each of them, on day 1, the order to work at the port, which stands when they fall back and does nothing where no ship unloads. It is a scripted order with no row behind it, marked so in the scenario, and it is there only to stop the default.

## Rulings by the architect during the build

Made on Nov 14, 2026, while the 39-day slice was built, on questions the team lead put. They stand over the text above where they differ.

- **A. The squadrons' groups wait at Lunga.** The rule change of item 4 of the build: a group with no aircraft and no pilots may be stationed at a base that is not its side's or has no airfield, and does nothing there; a base that changes side loses the groups of the beaten side only; such a group takes no rebase until it has aircraft. Its arrival waits by the rule of decision 0103, 5, already built.
- **B. The Kawaguchi main body is five units,** "Kawaguchi main body, part 1" to "part 5", of 1,200, 150, 1,000, 370 and 370 men, one for each run: a unit can start aboard one force only. Each is ordered to march on day 35. The measures sum them.
- **C. Ichiki marches on day 14.** It lands in the night pulse of day 13, and an order is taken at the start of the day, before the night pulse, so the first march order it can take is on day 14. It arrives and attacks on day 16, 22 August. **Changes item 13,** which had it march on the day it landed and arrive on day 15. The day of the attack is reported against the record's day 15.
- **D. The run of 15 August** has a new placeholder class, "Placeholder fast transport with cargo" (class 12): class 11 with 30 t of cargo, the 30 t resting on troop lift 70. It is used for that run and nowhere else unless a row gives a ship cargo. Class 11 is not changed.
- **E. The Japanese runs sail from Shortland,** a Japanese base at the hex PP 24 gives, (-2,-8), with no airfield, no stock and no guns. Ichiki's destroyers came from Truk, which is off the map: they start at Rabaul. Each run is timed to land in the night pulse of its landing day.
- **F. The Rabaul raids of 9 to 13 September and the arrivals of scenario 6's days 22 and 24** are restored at their shifted days.
- **G. The Eastern Solomons forces keep their start hexes** and sit until day 18. Where the carriers were on 6 August, "to the south" (item 7), is a gap.
- **H. Transports only.** The escorts and the covering cruisers are left out and listed as a gap. There is no night action off Savo Island on 8 to 9 August; the transports sail on day 3 by order.
- **A night landing is numbered a day later than its date.** A destroyer run landed after dark on the date its row gives (the one clock time read, LUF 1049, is about 9 p.m.), and the evening of a date is the night pulse of the next game day. Ichiki was numbered so. The others move one day later than the tables above: the Yokosuka 5th Special Naval Landing Force to day 11, Tassafaronga to day 24, the Kawaguchi parts to days 26, 28, 30, 31 and 32, and the Aoba Detachment's part to day 37. The Oka group stays on day 32: its row has it west of the perimeter by 7 September and it came by barge, with no night stated. The march orders do not move. The American landings were by day and do not move.
- **The cargo ship of day 26** carries the 392 men and no supply. No row gives her load, so none is invented; "with a placeholder load" in item 21 is struck.

On the rows:

1. **The Tulagi garrison is LUF 63,** one source for the place: the fighting unit "Tulagi garrison", 240 men (about 40 naval air men and a guard unit of about 200), and the working unit "Construction unit, Tulagi", 200 men; 440 in all, the Japanese-derived account, as the Lunga garrison follows the Japanese-derived account. 780 by American count (LUF 226) is the other end of the range the measures take. LUF 227 is a part only (Gavutu and Tanambogo) and is not the garrison. Tulagi's placeholder stock becomes ten days for 440 men, 22 t.
2. **The sailing on 9 August** rests on the note of LUF 414; LUF 773 is cited for the hours of unloading only.
3. **The main body's size is LUF 310;** the runs with rows give the days and sizes of its parts. TL 161 names the men of 4 September as of the Ichiki and Aoba Detachments, TL 164 is disputed (231 against 370) and TL 166 may repeat it, so the parts are a schedule for landing 3,090 men and not a statement of which unit came on which night.
4. **The ten days of Japanese food** of the count of about 15 August are read from the note of LUF 417, not its value.

## As built

Built on Nov 14, 2026, in `data/scenarios/07_full_slice.toml` (39 days), with the land links file `data/map/south_pacific_land_links.csv` and the measures `data/validation/guadalcanal_landings.toml`; the baseline is `docs/validation/full-slice-39-days-baseline.md`. Every figure of this record and its rulings is used as given. Choices the record did not make:

- **The arrival rule (item 8 and amendment 6).** The waiting of an arrival at a base not its side's or with no airfield was already built with decision 0103. What the slice lacked was a place for the groups to wait: ruling A is built as the loader and the invariants allowing a group with no aircraft and no pilots at such a base, a base that changes side losing only the beaten side's groups, and a new refusal, `AirGroupWaitsForBase` ("the air group has no aircraft and waits for its base"), added at the end, for a rebase of such a group. Mission orders to it are taken as standing orders and fly nothing, as for any group with no aircraft; "takes no mission" is read as that. The rulebook's chapters 1, 5 and 7 and the glossary say so. No other scenario gives other events.
- **A loader check.** The loader refused a base strike order on a base of the group's own side by the base's side at the start. Lunga is Japanese on day 1 and the Rabaul raids on it begin on day 19, so the loader now holds only a day-1 order to the start's side; a later order is checked at intake, which already refuses a base that is not an enemy's (`NotAnEnemyBase`). No scenario's result changes.
- **Bases.** Lunga is base 1 (it keeps the number of Henderson Field, and the objectives and orders that name base 1 still name it), Tulagi base 4, the Japanese stocks at Taivu and west of the Lunga bases 5 and 6 (off the enemy's map, no airfield, named "Placeholder"), Shortland base 7. Lunga keeps the slice's placeholder guns (8), repairs (3) and fuel (400 t). The American bases are formed by the landings (numbered from 8 in play), and Lunga and Tulagi become American when the Japanese are beaten there.
- **Where the American transports start.** Three hexes west of each beach by sea, 120 nm, which their cruising 10 knots steams in 12 hours from 18:00: (-1,-4) for Guadalcanal, (0,-5) for Tulagi. A start hex is worked out from a sailing time and is not a position from a row. They reach Lunga at 10:00 and Tulagi at 06:00 on day 1.
- **Ships.** 21 placeholder troop transports for Guadalcanal (8, 8, 3 and 2 for the four units) and 12 for the Tulagi side (2, 1, 2 and 7), the 60 days of food shared as 15 of 156 t and 6 of 155 t, and 10 of 152 t and 2 of 151 t. Four ships of class 12 with 30 t each for the run of 15 August. Two placeholder cargo ships of 1,000 t each for the run of 21 August. Six placeholder fast transports (task force 24) carry 2/5 and then the Raiders across, and two more (task force 25) the Parachutists, because a ship carries one unit and a load order names one. One placeholder troop transport brings the Seabees, with no supply. On the Japanese side Fubuki and Shirayuki go with three placeholder fast transports on 29 August, Hatsuyuki and Amagiri with five on 31 August; every other run is in placeholder fast transports, enough for its men (1, 5, 1, 5, 2, 2, 4 and 2 ships).
- **Sailing days and speeds.** Each American force is sent from Espiritu Santo on the day that brings it to its beach on the record's day at its cruising speed: task force 22 on day 8, task force 23 on day 13, task force 24 on day 11 (to lie at Tulagi), task force 25 on day 21, task force 26 on day 24. Each Japanese run is sent on the day before its landing, with its route to the landing hex and back home, a mark on the landing hex (decision 0096) and an unload order, at the speed that brings it in the night pulse of its landing day: 12 knots from Shortland to Taivu (arriving 20:00), 9.5 knots from Shortland to (2,-5) (19:35), 22 knots from Rabaul to Taivu for Ichiki (20:46). The speeds were worked out by running the scenario and reading the arrivals; they are placeholders of timing and rest on no row of speed.
- **Orders.** The American units start aboard on attack (the scenario key) and are set to defend on the day after the record's capture: day 3 at Lunga, day 4 at Tulagi. The transports are sent to Espiritu Santo on day 3. The crossings load the day before and land on the record's day. The Japanese march orders are given with an attack order: Ichiki on day 14, the five Kawaguchi parts on day 35, the Oka group on day 37.
- **Arrivals and raids.** VMF-223 (19 aircraft and pilots) and VMSB-232 (12) arrive on day 14 at base 1, the strengths of scenario 6; group 10's six damaged aircraft, a placeholder for coverage in the fourteen-day slice, are gone, since an arrival brings ready aircraft. The arrivals of scenario 6's days 22 and 24 are restored as days 36 and 38, and its raid orders of days 16 to 25 as days 30, 31, 34 and 39 (ruling F). Before day 15 the Rabaul bombers have no order and rest.
- **Objectives.** `base_held` for Lunga (10 points) and Tulagi (5) and `men_lost` (1 point for each 100 enemy men) for both sides, beside what the slice had. Every point value is a placeholder set for this build.
- **The measures** are those of decision 0093 and the three of this record, with two readings the architect is asked to confirm: the measure "the airfield is American at the end of 21 August" is taken at the end of day 16, the game's day of the attack (ruling C), since on day 15 a run would hold it before the attack came; and the men lost at the Tenaru are taken in the airfield's hex on days 16 to 19, at the ridge on days 37 to 39. A measure of the Japanese men lost on Tulagi (440 to 780, LUF 63 and 226; reported) is added for the range of the architect's ruling on rows, 1. The validation harness gained the takes `base_holder`, `men_lost` (by side, days and, if wanted, hex), `airfield_built_day`, `arrival_day` and `days_of_food`, and the batch record the counts behind them (`data/validation/README.md`; `docs/commands.md`); the batch file stays version 5.
- **What the baseline shows.** Of 6 judged measures 4 agree and 2 miss, the Japanese and American losses at the Tenaru; the airfield is built on day 11, the squadrons arrive on day 14, and every Japanese man who reaches the field is lost (`docs/validation/full-slice-39-days-baseline.md`, with what looks wrong and its causes).
- **Files remade.** The slice's run baselines; `tests/scenario_07_determinism.rs` now runs 39 days and permutes the land units' tables as well; the saved-game test's counts of data files (17) and days (39); the test of `play` against the held-open server over the 39 days; the recorded game `full_slice_five_days_sonnet`, regenerated by `regame.py`, whose requests were written for the fourteen-day slice (its README says what that means). The frozen copy of the slice is not touched; the new class file has a frozen copy, which no frozen scenario names. The property of scenario 26 that no other scenario halts a force now leaves out a scenario that marks a route (`act_at`), as the slice now does.

## As built (amendments 8 and 9)

Built on Nov 14, 2026: the placeholder stock west of the Lunga (base 6) holds 143 t in `data/scenarios/07_full_slice.toml`, its comment says what it rests on, a comment where the Japanese bases of Lunga and Tulagi are stated says their defences are 0 because no row sizes the works of the garrisons of 7 August (decision 0106), and two scripted orders of day 1 set the 11th and 13th Construction Units (land units 12 and 13) to work at the port (amendment 9). The engine takes both orders at Lunga on day 1 and they stand when the units fall back: the units work at the port of base 6 from day 5 and its airfield gets no work. The construction unit on Tulagi surrenders on day 1 and has no order. Nothing else in the file changed; no rule and no figure of `data/reference` changed. The run is `docs/validation/full-slice-39-days-after-stock-fix.md`: no measure moves; the stock runs short on day 20 where it ran short on day 13, and the west loses fewer men unfed (figures in the report). Built with amendment 8 alone, the construction units in the west finished an airfield at the stock on day 17 to 19 in every run; amendment 9 stops that, and no run of the report's batch builds one there. Files remade: the slice's run baselines (`crates/sim-cli/tests/run_baseline/07_full_slice.txt` and `.events.txt`) and the saved game of the recorded game `full_slice_five_days_sonnet`, by `regame.py` (its daily hashes; its transcript did not change). The determinism test pins no hash of the live file and did not change. The frozen copy of the slice is not touched.
