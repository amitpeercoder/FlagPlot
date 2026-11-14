# 0042: The Rabaul against Guadalcanal scenario

Status: confirmed by the architect. The designer approved the plan for scenario 6 on Nov 11, 2026 and left data and rule details to the architect. Every figure here is either derived from `data/oob`, settled by the architect with its reason, or marked a placeholder.

Summary: The Rabaul against Guadalcanal scenario: who is there, what the aircraft can do, and what is a placeholder

Standing: Confirmed

Note of Nov 13, 2026: items 16 and 17 no longer hold for the live scenario. Decision 0082 sets the raid days from the units' forms by dated orders, removes the Japanese rest thresholds, and has the dive bombers search. Fatigue no longer stands in for weather in choosing the days. The standing patrol, the Allied threshold of 40 and the replacements of item 13 stay, marked invented. The scenario as this record built it is kept as `data/scenarios/step2-frozen/06_rabaul_guadalcanal.toml` for the hand calculation. The text below is as it was.

## What the scenario is

Twenty-eight days of raids on Henderson Field from Rabaul, 21 August to 17 September 1942. It closes scenario 6. It is the first scenario in which wear, repair, fatigue, rest, pilots, replacements, arrivals and base supply run together, over the real map and with real air groups.

It is not a reconstruction. The step asks for a run that works and a spread that is plausible; the bands are set from sources in step 2.

## Decisions

### Who is there

1. **Rabaul's bombers are one group of 41 G4M1.** The count is "Bettys 41" at Rabaul on 24 August (`air_unit_facts.csv` row 238, medium, preferred). They belonged to the 4th, Misawa and Kisarazu Air Groups; no source read splits the 41 among them on that day, so they are not split.
2. **Rabaul's fighters are the Tainan Air Group's 24 A6M2** (row for August, citing Hata) **and the 6th Air Group's 13 A6M2, which arrive on 31 August,** day 11 (Lundstrom as cited: 13 reached Rabaul).
3. **The 2nd Air Group's A6M3 Model 32s are left out.** They could not fly to Guadalcanal and back from Rabaul; sources say so, and their range rows are shorter than the Model 21's. They defended Rabaul and flew to New Guinea, and nothing attacks Rabaul here.
4. **Henderson Field starts with VMF-223's 19 F4F-4 and VMSB-232's 12 SBD-3,** which landed on 20 August, and receives the squadrons the sources date:

   | Day | Date | Arrival |
   | --- | ---- | ------- |
   | 4 | 24 August | 11 SBD-3, Enterprise's Flight 300 |
   | 10 | 30 August | 19 F4F-4 of VMF-224 and 12 SBD-3 of VMSB-231 |
   | 22 | 11 September | 24 F4F-4 of VF-5 |
   | 24 | 13 September | 18 F4F-4 from Hornet and Wasp, and 12 SBD-3 of VS-3 |

5. **VF-5 arrives on 11 September, not the 4th.** The Marine official history (Hough) gives the 11th; Wikipedia, citing Lundstrom, gives the 4th. The official history is the better source and the row it disagrees with was not read at first hand.
6. **The P-400s of the 67th Fighter Squadron are left out.** The official histories say they could not reach the height the raids came in at, and they were used against ground targets. The facts hold no range or protection for the type. The six TBF-1 of 13 September are left out too: six aircraft on the ground.
7. **Every arrival brings a pilot for each aircraft.** The sources count aircraft. Henderson Field had more pilots than aircraft in September (86 for 64), so this understates the Allies if anything.

### What the aircraft can do

8. **The A6M2's radius is 556 miles,** read for this scenario from the row that records the escort flown from Rabaul to Guadalcanal on 7 August 1942 (`aircraft_type_facts.csv` row 517). The source says "556 miles each way" without a unit. It is read as nautical: two official histories give 565 nautical miles for the route, and a statute reading would put the flight 80 miles short of its own target. The Eastern Solomons list still reads 261 miles, the radius from a carrier. The air distance between the two hexes is 556 miles (decision 0040), so the escort reaches with nothing to spare. That is what the sources say of it too: "nearly to their maximum range".
9. **The G4M1's radius is 611 miles,** a third of the 1,834 miles "with normal fuel and bomb loads" (row 532), the method decision 0029 uses where no radius is stated. Researchers looked twice for a stated radius and found none.
10. **The G4M1 has no pilot armour and no protected tanks** (rows 536 and 538). Four sources agree for 1942; none is a primary document.
11. **Henderson Field has 8 anti-aircraft shots:** two batteries of 90 mm guns by the end of August (Miller), at the four guns the one counted battery had (Zimmerman), counted as a ship's heavy guns are (decision 0029). The third battery of early September and the lighter guns are not counted: a base's guns do not change by date, and nothing gives the lighter guns' numbers.

### What is a placeholder

12. **Each base's supply and repairs a day:** 4,000 t and 3 a day at Henderson Field, 10,000 t and 4 a day at Rabaul. No convoy runs in the scenario. Henderson Field's fuel was short in fact (a reserve of four days in mid-September); that is for a later scenario with ships in it.
13. **Replacements at Rabaul: two bombers and two fighters a day, each with a pilot.** The sources give totals only: 60 fighters and 72 medium bombers "reinforced Rabaul" in a period that reads as the first half of September (Hollway, no references), and 85 aircraft delivered from 15 to 28 September. 132 over the 28 days is 4.7 a day; two and two is the nearest whole split below it. That replacement pilots came with the aircraft is an assumption.
14. **The combat, wear, fatigue and supply figures** are the placeholder files of stages 6a to 6d.

### Orders

15. **The Japanese fighters keep nothing over Rabaul** (a patrol share of 0) and send every fighter that can fly as escort. Nothing attacks Rabaul, and a patrol there would tire the escort for nothing.
16. **The staff rests the Japanese groups above a fatigue of 45 and the Allied fighters above 40.** With the placeholder figures the bombers fly on days 1, 2 and 3 and then every third day: eleven raids in the four weeks.
17. **The dive bombers at Henderson Field are held for ships.** There are none, so they stay on the ground, where bombs can find them.

## Not modelled, and it mattered

- **Weather.** Raids were turned back or not sent on many days. Here only fatigue and supply stop a raid.
- **Rabaul's other war.** The same groups flew to Milne Bay and Buna.
- **Warning.** Coastwatchers and radar gave Henderson Field time to climb. Here the patrol is always up.
- **Height.** The raids came in above 20,000 feet, which is why the P-400s are out and why the guns hit little.
- **Henderson Field's own strikes** on ships, and the ships that shelled it.
- **Ditchings on the way home.** Wear and write-offs stand in for them (decision 0035).

## What the runs show

Over seeds 1 to 1000. The figures are placeholders, so this is the shape of the four weeks and not a result to quote.

**The rhythm is right.** A raid flies on eleven mornings in every run: days 1, 2 and 3 and then every third day. The sources record about ten raids in the window, held down by weather and by Rabaul's other war; here fatigue alone does it.

| Raid day | 1 | 2 | 3 | 6 | 9 | 12 | 15 | 18 | 21 | 24 | 27 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Bombers launched | 41.0 | 27.6 | 19.1 | 23.3 | 24.0 | 24.0 | 24.0 | 23.0 | 23.0 | 23.0 | 22.7 |
| Escorts | 24.0 | 11.7 | 6.4 | 9.2 | 12.4 | 26.6 | 24.4 | 24.0 | 24.3 | 24.4 | 23.5 |
| Allied fighters on patrol | 19.0 | 13.4 | 9.6 | 6.0 | 3.8 | 16.8 | 10.0 | 6.6 | 3.7 | 39.8 | 27.0 |

After the first day a raid is about 23 bombers, which is the size the sources give for the raids of September (26, 27, 28). The escort collapses in the first week, to six on day 3, and recovers when the 6th Air Group arrives. The defence thins between arrivals and jumps with each one.

| A run, on average (lowest to highest) | Japanese | Allied |
| --- | --- | --- |
| Bombers shot down | 11.8 (3 to 26) | |
| Fighters shot down | 39.6 (22 to 61) | 33.7 (19 to 47) |
| Destroyed on the ground | 0 | 15.7 (4 to 35) |
| Written off after a sortie | 23.4 (8 to 43) | 6.5 (1 to 14) |
| Pilots lost | 51.4 (30 to 85) | 0 |

Bombs destroy 790 t of Henderson Field's 4,000 t a run, and it never runs out. Rabaul ends with 7,924 t of 10,000.

For comparison, one source gives 27 Marine fighters and 9 pilots lost from 21 August to 11 September against 31 Japanese aircraft destroyed in ten raids (Wikipedia, citing Lundstrom). The Allied fighter loss here is of that order. Four things are not:

- **No Allied pilot is ever lost.** A pilot shot down over his own field is always saved (decision 0038, 4). Nine were lost in three weeks.
- **The Japanese lose three fighters for every bomber.** The patrol fires at the escorts first, one for one (decision 0027). It reaches the bombers in 31 percent of raids, nearly all of them when fresh squadrons have just arrived and outnumber the escort: on day 24 it shoots down 4.6 bombers, on most days none. The Wildcats in fact dived through the escort for the bombers. (The architect expected the patrol almost never to reach the bombers. It does when it outnumbers the escort, and the arrivals see to that.)
- **VMF-223 is destroyed in every run,** by day 13 on average, and VMF-224 in 53. Losses to a patrol come off the lowest-numbered group first, and Henderson Field gets no aircraft but whole squadrons. In fact losses were spread and worn squadrons were kept up from the others.
- **Half as many Japanese aircraft are written off as are shot down.** Operational losses on this route were heavy, but the placeholder wear figures were not set from a source.

None of these is a fault in the code. Each is a rule or a placeholder doing what it says. They are the list for step 2, with Savo Island's.

## The legibility test

A cheap model was given a briefing as the Japanese air commander and the log of the first five days of the scenario's own seed, with every air subsystem running. It explained the two days without a raid from each group's fatigue. It split the shrinking of the third raid into its causes: for the bombers four fifths fatigue and one fifth damage, for the escort mostly damage. It predicted the sixth day exactly: four repairs, all to the fighters; one fighter and no pilot received; both groups returned at 40.72; 24 bombers and 10 escorts; 111.52 t for the raid. Asked what wears the force fastest it answered the distance, with the figures: 36 aircraft worn in three days against 7 shot down.

It noticed two things. A group that stays stood down for a second day records nothing that day, so the reader has only the rest events to go by; that is left, since an event every day for every resting group is noise. And the escort was starved of repairs because Rabaul repairs the lowest-numbered group first and the bombers took all four a night. That is the rule (decision 0035, 9) and it is worth a look in step 2: a base might better repair the group with the most damaged aircraft.

## Rule card

No rule is added. The scenario uses decisions 0027, 0030, 0033 to 0036 and 0038 to 0041.
