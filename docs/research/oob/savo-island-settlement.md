# Savo Island: settlement sheet for the 20 ships

Status: **approved by Amit on 2026-11-10, in one pass, as recommended.** The rows are marked in the data with "Settled by Amit" and the item of this sheet. Two things the sheet had left open were approved later the same day: the 8th Year torpedo warhead, and treating main-gun ammunition for three Japanese ships as not limiting (see "What approval did"). The sheet below is kept as it was put to him, apart from this paragraph and the last section. It is for the architect to turn into a numbered decision record.

**Why it exists.** The derivation tool (`crates/oob-derive`, decision 0020) builds the ship classes a scenario loads from `preferred` fact rows that hold on 8 August 1942, and refuses anything else. Against this branch it reports 103 problems and derives no ship. Each group below is one decision that clears several of them.

**Who settles.** Decision 0015 says a person settles a dispute. An earlier draft of this work marked some rows `preferred` or `alternative` on the researcher's judgement. Those have been put back to `disputed` or `unconfirmed`, and each is on this sheet (group 9). Where a standing in the data is `preferred` today, it is so only by the format's own rule: one row at medium grade or better with nothing against it, or two sources that agree.

**What is already decided.** Decision 0019 records your choices on who was where, on standard displacement as the damage measure, and on treating unknown search radar as not fitted. This sheet does not reopen those. It asks for the figures.

**How to read a row.** `id` is the row's number in the file named at the head of the group. Grade is confidence and basis: `m/s` is medium and stated, `l/i` is low and inferred. "Sure" is how confident the recommendation is: **confident**, **fair**, or **coin-toss**. A coin-toss that barely moves the game figure is marked **bulk**: those can be approved together.

## Already done, and not for decision

These were corrections, not disputes. They are in the data.

- **Aoba and Furutaka displacement, as built.** Rows 585 and 579 are the figures before the rebuilds and now end in 1940 and 1939. The post-rebuild rows (16, 51) are then the only ones that hold. They still read `disputed` and need your mark (group 1).
- **Furutaka fuel, as built.** Row 55 (1,378, with coal) now ends in 1939 for the same reason.
- **Canberra's torpedo tubes.** The row the tool flagged (387) recorded that her triggers failed, not a fit. It is an outcome, already recorded as one, and the fit row is removed.
- **Cruise speeds that every source agrees on** became `preferred` by the format's own rule, not by a choice: 14 knots for the Takao, Aoba and Furutaka classes, 12 knots for the Bagley and Benham classes.

## Approve in bulk: the choice barely moves the figure

| # | What | Rows (value) | Recommend | Spread |
| --- | --- | --- | --- | --- |
| b1 | Tenryu class displacement | 591, 216 (3,948); 215 (3,886) | 3,948 | 1.6%. The same number, once converted as metric tons. See the warning under group 1 |
| b2 | New Orleans class displacement | 436, 437 (9,950); 620, 599 (10,136) | 9,950 | 1.9% |
| b3 | Northampton class displacement | 468 (9,050); 604 (9,006); 467 (9,300) | 9,050 | 3% |
| b4 | Bagley class displacement | 625, 315 (1,624); 609 (1,646); 629 (1,500, a nominal treaty figure) | 1,624 | 1.4%. Every row is low grade, so this needs a written exception, not just a mark |
| b5 | Benham class displacement | 612 (1,657); 333 (1,656); 332 (1,500, nominal) | 1,657 | One ton. Also needs an exception |
| b6 | Furutaka class speed after rebuild | 57, 584 (33); 694 (32.95, a trial) | 33 | 0.05 knot |
| b7 | New Orleans class speed | 601, 622, 445 (32.7); 717 (32.75); 444 (32.5); 715 (32.0) | 32.7 | 0.7 knot. Row 716 is a different kind of figure: see group 2 |
| b8 | 53 cm 6th Year torpedo warhead | 137 (200 kg, the US technical mission); 136 (203 kg, a website) | 200 | 1.5% |
| b9 | Aoba class fuel after rebuild | 683 (2,040); 684 (2,008) | 2,040 | The same 2,040, once converted as metric tons |
| b10 | Furutaka class fuel after rebuild | 692 (1,858); 693 (1,829) | 1,858 | As b9 |
| b11 | Benham class fuel | 338, 729 (484); 337 (528, the emergency maximum) | 484 | 9% |

## 1. Displacement (`ship_class_facts.csv`, `displacement_standard_lt`)

**A warning that applies to the whole group.** The attribute is called standard displacement, but the rows are not all that. Where a source names the condition it is noted. Two Japanese figures are "normal" displacement, which is heavier than standard. From memory, and not from any row in the data, Tenryu's standard displacement is usually quoted near 3,230 tons and the Kamikaze class near 1,270: if so, the Tenryu figure below overstates her by about a fifth against the other ships. That is a lead to check, not a fact.

| Class | Rows | What each is | Recommend | Sure |
| --- | --- | --- | --- | --- |
| Takao (Chokai) | 575 (11,350, l/i); 200 (11,350, l/s); 199 (9,694, m/s) | 575, 200: condition not named by one, "standard" by Wikipedia. 199: as built, from the website that carries most Japanese class data; it reads like the design figure | 11,350 | Fair |
| Aoba | 16 (8,858, m/s) | After the 1938 to 1940 rebuild. The only row that holds | 8,858 | Confident it is the right row; the figure rests on one website |
| Furutaka | 51 (8,563, m/s) | After the 1936 to 1939 rebuild. The only row that holds | 8,563 | As Aoba |
| Yubari | 239, 632 (2,844, one source twice); 570 (3,387, l/i) | 2,844: "standard", as completed in 1923. 3,387: condition not named; probably normal | 2,844 | Fair. 19% apart |
| Kamikaze (Yunagi) | 73 (1,378, m/s); 596 (1,270, l/i) | 1,378: "normal". 1,270: condition not named | **1,270**, to keep to standard displacement | Coin-toss. 8% apart. The better-graded row is the wrong condition |
| Kent (Canberra) | 615, 263 (9,750); 634 (8,929); 264, 631 (10,000); 265 (10,300); 266 (10,400); 267 (10,570) | 9,750: two websites, one labelling it with a merchant-ship measure. 8,929: the Australian Navy page's 9,072 tonnes, which is the nominal 10,000 tons converted as short tons. The rest: single ships at other dates | 9,750 | Fair |

The bulk items b1 to b5 complete the group.

## 2. Maximum speed (`max_speed_kn`)

**The real question is design speed or measured speed.** Every Japanese and British figure in the data is a design speed. For the US destroyers there are also trial results, and the Navy's 1945 fuel tables give an *estimate* of speed at the weight the ships actually carried in war service, which is 2 to 4 knots below design. Using design speed for some ships and a lower measured figure for others would handicap the second group.

| Class | Rows | What each is | Recommend | Sure |
| --- | --- | --- | --- | --- |
| Aoba, after rebuild | 590 (33, m/s); 22 (32, m/s); 685 (33.43, l/s) | 33: design after rebuild. 32: the other Japanese class website. 33.43: Aoba's own trial, from Japanese Wikipedia | 33 | Fair |
| Bagley | 611, 323, 722 (38.5); 628 (38); 627, 324 (36.8); 630 (36.5); 723 (34.7) | 38.5 and 38: design. 36.8: Blue's trial. 36.5: one compiler. 34.7: the Navy's estimate at war-service weight | 38.5, the design speed, as for every other class | Fair on consistency. As a description of Bagley in 1942 the trial figure is nearer |
| Benham (Wilson) | 614, 730 (38.5); 342 (37.9); 341 (36.5) | 38.5: design. 37.9: a trial. 36.5: one website | 38.5 | As Bagley |
| New Orleans | see b7; 716 (30.9) | 30.9: the Navy's estimate at war-service weight, not a design or trial figure | 32.7 | Bulk |

**For the rule, not for this sheet:** if design speeds overstate what a loaded ship made in 1942, they do so for every ship. A single service-speed allowance in the rule would be fairer than mixing kinds of figure.

## 3. Endurance and cruise speed (`endurance_nm`, `endurance_speed_kn`)

The tool reads the two together. Each recommendation is a pair.

| Class | Rows | What each is | Recommend | Sure |
| --- | --- | --- | --- | --- |
| Takao (Chokai) | 671 (8,000, m/s); 673 (8,000, l/s); 669, 202 (8,500) | 8,000 at 14 knots: the only figure that names Chokai and Maya. 8,500: Takao and Atago | 8,000 at 14 | Confident |
| Aoba, after rebuild | 679 (8,000, m/s); 681 (8,223, l/s); 18 (6,000, l/s) | 8,000 and 8,223: after the rebuild, two sources. 6,000: the as-built figure, which the first source confirms, so row 18 should end in 1940 | 8,000 at 14 | Fair |
| Furutaka, after rebuild | 688 (7,900, m/s); 690 (7,000, l/s); 53 (6,000, l/s) | 7,900: after the rebuild. 7,000: state not given. 6,000: as built; row 53 should end in 1939 | 7,900 at 14 | Fair. The source misdates the rebuild, which weakens it |
| Yubari | 240, 699 (3,310 at 14); 697 (5,000 at 14); 695 (5,500 at 10) | 3,310: "actual", two rows that are probably one source. 5,000: the design figure. 5,500 at 10: one website, kind not stated | 3,310 at 14 | Fair. Every row for the actual figure is low grade, so it needs an exception |
| Kent (Canberra) | 702 (13,300 at 12, m/s); 704 (13,200 at 12, l/s); 268 (8,000 at 10); 706 (10,400, speed not given) | 13,300: the class. 13,200: Canberra herself, Wikipedia citing a book. 8,000 at 10: Wikipedia's class page | 13,300 at 12 | Fair. The gap to 8,000 is large and unexplained |
| New Orleans | 708 (7,620 at 15, h/s); 438 (5,580 at 20, m/s); 710 (10,000 at 15, m/s); 439 (5,280 at 20) | 7,620 and 5,580: the Navy's 1945 war-service fuel tables, one class at two speeds; both are true. 10,000: a website, design or trial not stated | 7,620 at 15 | Confident in the source. Which speed to call "cruise" is the choice |
| Bagley | 317 (6,940 at 12, m/s); 318 (6,940); 316, 718, 720 (6,500 at 12) | 6,940: the Navy's war-service tables. 6,500: three websites that probably share one design figure | 6,940 at 12 | Fair. Three sources against one, but the one is primary and measured in service |
| Benham (Wilson) | 334 (5,390 at 12, m/s); 335 (5,390); 724, 726 (6,500 at 12) | 5,390: the Navy's war-service tables. 6,500: two websites, one of which looks copied from its Bagley page | 5,390 at 12 | Fair |

**An inconsistency to know about.** These recommendations take the Navy's war-service tables for the US ships and design figures for the others, because the tables are the only primary source in the group. The Northampton class already has a preferred figure from the same tables, but at 20 knots, where the others are at 12 to 15. A figure for her at 15 knots is in that table and is not yet in the data.

## 4. Fuel (`fuel_capacity_lt`)

| Class | Rows | What each is | Recommend | Sure |
| --- | --- | --- | --- | --- |
| Yubari | 700 (916, m/s); 243, 701 (902); 242 (817, m/s) | 916 and 902: the same 916 metric tons of oil, by three sources. 817: as completed in 1923, when she also carried coal | 902 | Confident |
| New Orleans | 712 (1,900, h/s); 441 (2,256, m/s); 442 (2,195); 713 (1,861); 714 (3,269) | 1,900: "full load" in the Navy's tables. 2,256: the same tables' emergency maximum, every tank at 95%. 3,269: one unsourced website, far above the rest | 1,900 | Fair. It is the normal load; the tables are dated 1945 |

The bulk items b9 to b11 complete the group. The Kent class now has one supported row (3,400) and needs no decision.

## 5. Torpedo warheads (`weapon_facts.csv`, `warhead_kg`)

**The US Mark 15. This one matters:** it nearly doubles the torpedo damage of seven destroyers.

| Rows | Value | What it is |
| --- | --- | --- |
| 304 (m/s) | 224 kg | The first model, "Mod 0", 494 lb of TNT, on a weapons reference site |
| 407, 428, 429 (m/s) | 374 kg | The Navy's 1978 torpedo history. Its data sheet names this warhead as a later model (Mod 3) with a different explosive. Its text says only that the Mark 15 design grew "from 500 to 825 pounds" |

What dates it: the Bureau of Ordnance's own history says the more powerful explosive was first used in torpedoes in the autumn of 1942. Nothing read says outright what destroyers carried in August.

**Recommend 224 kg. Sure: fair.** The evidence points one way but is indirect. If you want it firmer, this is the single figure most worth a trip to a library.

The 53 cm 6th Year Type warhead is bulk item b8. **Canberra's torpedo has no warhead figure at all:** the mark her class carried is itself disputed between the Mark V and the Mark VII, and only the Mark VII has a figure, 336 kg. She fired none that night. **Recommend treating her as carrying the Mark VII, 336 kg, by written exception. Sure: coin-toss, and it cannot affect the result unless the scenario lets her fire.** The rows are in group 9, r2 and r7.

## 6. Yubari's torpedo (`fits.csv`, `torpedo_tubes`)

| Rows | Torpedo | Basis |
| --- | --- | --- |
| 378 (l/i) | 61 cm 8th Year Type | The US technical mission's table of torpedo mounts lists her with the 8th Year twin mount |
| 379 (l/s) | 61 cm 8th Year Type | One unsourced website |
| 312, 313 (l/i) | 61 cm 8th Year Type | Class rows, by inference from what was standard when she was built |
| 132 (l/s) | 61 cm Type 93 | An assumption made in the first pass because the calibre matched. No source |

**Recommend the 8th Year Type. Sure: fair.** The mount is sourced; the torpedo in it is inferred. Row 132 has nothing behind it and should become `alternative`. The count of four is not in question. This choice sets her warhead at 345 kg instead of 490.

## 7. Chicago

**Main guns (`fits.csv` row 254).** Nine 8-inch guns in three triple turrets is not in doubt. The row is `unconfirmed` because the source lists four marks of the same gun family for the class without saying which Chicago had. The data has one set of figures for the family. **Recommend accepting row 254 by written exception. Sure: confident on what matters to the rule.**

**Rounds per gun (`ship_class_facts.csv`).** All four rows are `disputed`.

| Row | Value | What it is |
| --- | --- | --- |
| 657 | 167 | 1,500 rounds a ship "early in the war", which the source says of the Northampton class by name |
| 658 | 144 | "Just over 1,300" a ship after refits that began after July 1942. No source read says Chicago had been refitted by 8 August |
| 656 | 150 | The stowage per gun quoted for the gun, not an outfit |
| 659 | 100 | The peacetime outfit, to January 1942 |

Row 657 carried an end date of July 1942 that no source supports for Chicago. It has been removed, which is a correction. **Recommend 167. Sure: fair.**

## 8. Single weak rows and missing figures

| # | Needed for | What exists | Recommend | Sure |
| --- | --- | --- | --- | --- |
| e1 | Rate of fire, Japanese 20 cm gun (five cruisers) | Row 431: 2.4 rounds a minute, from a table in a US technical mission report. The table does not say whether it is a design or an achieved rate. The same reference site calls about 2 "realistic" for the older mounting of this gun | 2.4, by exception | Fair. On the night these ships averaged 1.2 to 1.6 a gun a minute over their whole firing time, which is a floor and is consistent |
| e2 | Rate of fire, Japanese 14 cm gun (Tenryu, Yubari) | Row 432: about 6, from the same table, for the twin mount | 6, by exception | Weak. Nothing else exists |
| e3 | Rate of fire, Japanese 12 cm gun (Yunagi) | Only a designed rate: row 44, 5.5 | 5.5, by exception, flagged as a designed rate | Coin-toss. Yunagi's guns played almost no part |
| e4 | Rate of fire, US 8-inch Mark 12 (Vincennes, Quincy) | No practical rate. Designed rate, row 349: 3 | 3, by exception | Fair |
| e5 | Rate of fire, US 8-inch Mark 9 (Astoria, Chicago) | Row 422: 5, the rate Houston kept up for a short time in another battle, "possibly" by an irregular loading method. Graded low and `unconfirmed`. Designed rate, row 358: 3 | 3, by exception, as for the Mark 12 | Fair. Using 5 would give two cruisers a rate two-thirds higher than their sisters on one ship's best quarter-hour |
| e6 | Rounds per gun, Tenryu, Yubari, Yunagi | **Nothing found.** The reference site says "not available" for both guns | No figure can be recommended. Tenryu fired 80 rounds that night and Yubari 96, far below any plausible outfit, so the scenario could treat their main-gun ammunition as not limiting, as a stated assumption | Your call |
| e7 | Search radar, Vincennes | A fire-control radar is sourced (it gave a range of 8,250 yards). No source names a search set or says she had none. Her sister Astoria had an SC search set, and Quincy's radar picked up an aircraft | **Unknown.** If the scenario must choose, "fitted" is the better guess from her sisters | For you to place on the "unknown means" list either way |
| e8 | Search radar, Jarvis | Nothing in any source read | **Unknown** | As e7. She took no part |

## 9. Choices first made in the data, now put back for you

Each of these was marked by the researcher and has been returned to `disputed` or `unconfirmed`.

| # | What | Rows | What each is | Recommend | Sure |
| --- | --- | --- | --- | --- | --- |
| r1 | Canberra's surface radar (`fits.csv`) | 390 (Type 271, h/s); 102, 353, 103, 355 (Type 271, low); 351, 101 (Type 241, low) | 390: the Australian official history says she "was using her Type 271 radar set for surface warning" that night. The other 271 rows: two enthusiast sources. 241: a website and Wikipedia | Type 271 | Confident. Under decision 0019 the "not fitted" assumption stops applying to her once this is approved, because the facts then hold a fit |
| r2 | Canberra's torpedo tubes (`fits.csv`) | 342, 343, 389 (Mark V, 8 tubes); 388 (Mark VII, 8); 104 (mark not named, 8) | Every row says eight tubes. The mark is the only difference, and every row is low grade and inferred from a statement about the class | Eight tubes. Mark VII for the warhead, as in group 5 | Confident on eight. Coin-toss on the mark |
| r3 | New Orleans class rounds per gun | 662 (156, m/i); 661 (150, m/s); 663 (125, from September 1942) | 156: "1,400 rounds a ship" for most nine-gun cruisers, divided by nine. 150: the same page's stowage per gun, which the first figure exceeds. 125: after later refits | 150 | Coin-toss between the first two, 4% apart. Bulk |
| r4 | Bagley and Benham rounds per gun | 665, 667 (300); 666, 668 (100, to 1940) | 300: a design outfit the page gives for a range of destroyer classes without naming these two. 100: the earlier peacetime outfit. The rows do not overlap in date, so nothing is disputed and 300 is `preferred` by rule | 300, knowing it is a design figure for a group of classes | Fair |
| r5 | Practical rate of fire, US 5-inch/38 (seven destroyers) | 424 (15, m/s); 423 (25, m/s) | Both from a British ship's gunnery trials with this gun: 15 with the normal ammunition supply, 25 from ready-use rounds for a short burst. Neither is a rate recorded for US destroyers. The designed rate in the data is 12 | 15 | Weak. It is the right kind of figure from the wrong navy |
| r6 | Practical rate of fire, British 8-inch (Canberra) | 426 (3.5, m/i); 427 (5, m/s) | 3.5: the midpoint of "3 to 4" sustained in action. 5: reached briefly in trials | 3.5 | Fair |
| r7 | British Mark VII torpedo warhead | 411 (335.7 kg); 430 (340 kg) | The same 740 lb, exact and rounded | 335.7 | Bulk |
| r8 | Torpedo type, Kinugasa (`fits.csv`) | 381 (Type 93, l/i) | Inferred: her class was given the quadruple mount that the US technical mission says fires the Type 93. No source names her torpedo. Her class row also says Type 93 | Type 93, by exception | Fair |
| r9 | Torpedo type, Tenryu and Yunagi (`fits.csv`) | 377, 375 (a 21-inch torpedo, type not stated, l/s); 376 (6th Year Type, l/i, Yunagi) | The 1950 analysis gives each six 21-inch tubes and names no torpedo. The technical mission's table lists the 6th Year mount for Yunagi's class. The class rows say 6th Year Type by inference | 6th Year Type, by exception | Fair. It cannot be the Type 93, which is 24-inch |

Torpedo reloads are not on this sheet because the tool does not read them. The data has eight for the Aoba and Furutaka classes as a low-grade inference, and Chokai's class disputed between 16 and 8.

## What approval did

Applied on 2026-11-10. Winning rows are `preferred` and the others `alternative`, each with a note naming this sheet's item. The data checker passes. The derivation tool went from 103 problems to 29.

**Rows chosen that cannot be `preferred` under the format's rule,** because each rests on a lone low-grade source or on weak sources only. They stay `unconfirmed`, say "chosen" in their note, and need a written exception in the tool's rules file (decision 0020, item 3):

| Ships | Fact | Row to read | Value | Sheet item |
| --- | --- | --- | --- | --- |
| Bagley, Patterson, Helm, Blue, Ralph Talbot, Jarvis | Displacement | `ship_class_facts.csv` 625 or 315 | 1,624 | b4 |
| Wilson | Displacement | 612 | 1,657 | b5 |
| Chicago | Displacement | 468 | 9,050 | b3 |
| Yunagi | Displacement | 596 | 1,270 | group 1 |
| Yubari | Endurance and cruise speed | 240 or 699; 241 or 698 | 3,310 at 14 | group 3 |
| Yubari | Fuel | 243 or 701 | 902 | group 4 |
| Chicago | Main guns | `fits.csv` 254 | Nine 8-inch | group 7 |
| Canberra | Torpedo tubes | `fits.csv` 388 | Eight, Mark VII | r2 |
| Five Japanese heavy cruisers | Practical rate of fire | `weapon_facts.csv` 431 | 2.4 | e1 |
| Tenryu, Yubari | Practical rate of fire | 432 | 6 | e2 |
| Kinugasa; Tenryu, Yunagi | Torpedo type | `fits.csv` 381; the class rows | Type 93; 6th Year Type | r8, r9 |

**Approved figures the tool cannot yet read at all,** because they are designed rates and it reads only the practical-rate attribute. The rules file would have to name the designed-rate row:

| Ships | Row | Value | Sheet item |
| --- | --- | --- | --- |
| Yunagi | `weapon_facts.csv` 44 | 5.5 | e3 |
| Vincennes, Quincy | 349 | 3 | e4 |
| Astoria, Chicago | 358 | 3 | e5 |

**Approved afterwards, on the same day:**

- **The 61 cm 8th Year torpedo warhead (Yubari): 345 kg,** the US technical mission's figure (row 156), over 346 kg from a website (row 155). Applied in the data.
- **Rounds per gun for Tenryu, Yubari and Yunagi (e6): their main-gun ammunition is treated as not limiting.** No figure was found. Tenryu fired 80 rounds that night and Yubari 96, far below any plausible outfit. This is a scenario assumption, not a fact, so nothing is added to the facts layer: there is still no `main_rounds_per_gun` row for the three classes, and `gaps.csv` still lists it. The scenario or the tool's rules file has to carry the assumption by name.

**Still unknown in the sources:**

- **Search radar for Vincennes and Jarvis (e7, e8).** Decision 0019's working assumption covers both.

**Three things this sheet cannot fix and the rule should know.**

- **The rates of fire are not one kind of figure.** Japanese 2.4 is from a table of unstated kind, British 3.5 is a sustained rate in action, US 8-inch 3 is a designed rate, and US 5-inch 15 is a British trial. They are the best available and they are not comparable.
- **Displacement is not one condition** (group 1), and most Japanese figures rest on one website.
- **Design speeds overstate 1942 service speeds** for every ship (group 2).
