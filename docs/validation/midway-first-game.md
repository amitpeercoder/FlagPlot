# Midway: one game before the thousand

Scenario 114, decision 0198, file a (`data/scenarios/midway/01a_midway.toml`): both sides given the orders of the hand calculation, staff against staff, on the live figures. Played once at seed 1 with `cargo run --release -p sim-cli -- run data/scenarios/midway/01a_midway.toml --events --seed 1`, on the branch `midway-build`, state hash layout `flagplot-state-v53`. The five daily hashes: `e25e607fbb678425`, `8c01064710f34956`, `31f82cb636b0d8bc`, `eaa7e178a5235b73`, `9b662804a517d799`.

This is the report the hand calculation asks for ("One game before the thousand", rule 41). It is not a test, and no run of 1,000 has been made. It changes no order, figure or point: what follows is for the architect to read before any batch. Event numbers are the game's (`#N`).

A day runs from 1800 the evening before its date: day 1 is 3 June, and its night pulse is the night of 2 to 3 June. So "D4 15:59" is 1559 on 6 June.

## In ten lines

1. **The Japanese side won a decisive victory, 49.41 to 7.6, a margin of 41.81** (decisive is 30).
2. **The landing happened in full:** the invasion force reached Midway's hex at 1559 on 6 June (D4) and put all 4,507 men ashore by the night of 6 to 7 June, 45.07 points, with no fight ashore.
3. **No carrier was sunk on either side.** Akagi was put out of action (flight deck out, 5 points), by Midway's land-based bombers, not by the American carriers.
4. **The American carriers never launched a strike.** They found the Japanese carriers on the morning of 4 June (D2) at 317 nm, closed on 5 June to 208 nm, just beyond their bombers' reach, lost the contact and went back to their station.
5. **The Japanese carriers never found the American carriers.** Their six floatplanes searched only toward Midway, 110 to 180 degrees, every day; the American carriers lay to the east.
6. **The Japanese carriers stood 160 nm north of Midway from the first evening** and raided the island every morning from 4 to 7 June; Midway's aircraft ready fell from 111 to 40.
7. **Midway's bombers struck the Japanese carriers twice on 4 June** (12 bomb hits on Akagi) and a transport once on 6 June; four more of their strikes found nothing where the contact had been.
8. The three things that most differ from the battle: **the landing** (historically turned back on 5 June, nobody landed); **no carrier battle at all** (historically four Japanese carriers and Yorktown sunk); **the hits on Akagi came from Midway's aircraft** (historically they hit nothing, and the carriers' dive bombers did the work).
9. No surface action could happen: no two hostile forces ever met at sea, so the classes whose surface weapons are not modelled took nothing away in this game.
10. No ship ran out of fuel; Midway's supply never came near to stopping an aircraft (2,172.59 t on the last evening).

## Day by day

| Day | Japanese staff | First Carrier Striking Force (task force 3) | Midway Invasion Force (task force 4) | Midway | Task Forces 16 and 17 |
| --- | --- | --- | --- | --- | --- |
| 1, 3 June | Under the take forms "ready a landing at (5,1) by task_force:4", its source Wake (`#57`, `#60`), and "operate the carrier force task_force:3 from (5,-3)" (`#66`). Holds no estimate, so "the staff takes as many as can be carried" (`#59`). The landing is ready at once: the force "does not lie at a base of its side, so it can gain no loading ... the landing is ready as it stands" (`#69`); released (`#65`). | Routed to (5,-3) at full speed, "cruise taking 44.4 hours, more than the night's 12" (`#165`); 28.3 kn, (-2,-15) to (5,-11) in the night, (5,-7), arrives at (5,-3) at 1732 (`#490`). Floatplanes search 100 to 170 toward Midway, the only threat they know (`#223`). Not sighted. | Sails at once: "827 nm out at 8.8 kn = 93.977272 hours, so it arrives at (5,1) at hour 93.977272; 1005 nm back to the source base:2" (`#171`). (-17,4) to (-14,4), (-13,4), (-12,4). | The flying boats search 200 to 000 to 898 and 737 nm (`#149`: threats Wake and the waters to deny). The PBY-5s sight task force 4 at 708 nm, reported as "45 ships, cruiser" (`#329`, `#330`); the PBY-5s miss task force 3 at 480 nm (chance 0.21). Bombers set to naval strike under the deny (`#133`); nothing in reach. | Task Force 16 moves to Point Luck (14,-6), there at 2040 on 2 June (`#254`); Task Force 17 stays at (15,-6). "Does not close: the side holds no contact of ships 3 pulses old or less" (`#52`). Dive bombers search 200 to 270, toward Wake: "threats: base:2 (Wake)" (`#113`). Nothing found. |
| 2, 4 June | "task_force:4 is routed to (5,1) and its orders stand" (`#552`). Bombers of the four carriers set to strike Midway: "the side holds no contact of ships 3 pulses old or less" (`#577`). | At (5,-3). Morning: 151 bombers with 43 escorts raid Midway (`#683` to `#722`). Struck twice by Midway's bombers (below): Akagi's deck degraded in the morning and out in the afternoon (`#733`, `#909`). One of Akagi's fighters lands on Kaga (`#915`). | (-12,4) to (-9,4), (-8,4), (-6,4). | Morning: VS-6's report of the carriers at (5,-3) (`#668`) is in hand, and Midway's three bomber groups strike at 160 nm: 19 SBD-2, 17 SB2U-3, 17 B-17E with 13 fighters (`#680` to `#692`). Afternoon: 8, 13 and 11 with 8 fighters (`#887` to `#891`). Raided in the morning: 24 hits, 300 t of supply, 63 men, airfield damage 10. | At the morning's start no carrier contact is held ("contact 1, a cruiser", `#548`), so neither force closes. VS-6 sights the carriers at 317 nm in the morning, "21 ships, carrier" (`#667`), and again in the afternoon (`#881`); VS-5 at 356 nm (`#670`). Beyond every bomber's reach; no launch. |
| 3, 5 June | Orders stand. | At (5,-3). Morning raid on Midway, 92 bombers (`#1176` to `#1200`). | (-6,4) to (-4,4), (-2,4), (-1,4). | A PBY-5A reports the carriers at (4,-3), a hex off (`#1170`). 7 SB2U-3 and 9 B-17E strike there at 183 nm and find nothing; contact 2 is dropped (`#1193` to `#1205`). Raided: airfield damage 22. | Both close: "contact 2, a carrier, 0 pulse(s) old, is 288 nm from (14,-6), beyond the 175 nm normal reach of its bombers ... at the balanced risk it closes on a carrier", routed to (11,-6) at cruise (`#1028`, `#1030`), aimed at the carriers' reported (6,-4). There at 0200 and 0440. VB-6 sights the carriers at 208 nm (`#1161`): beyond the SBD-3's 200 nm and the TBD-1's 175. No launch. Afternoon search misses. |
| 4, 6 June | Orders stand. | At (5,-3). Morning raid, 75 bombers (`#1574` to `#1601`). Re-sighted in the afternoon by VS-6, reported as "21 ships, battleship" (`#1743`). | (-1,4) to (3,3) in the night. Sighted by a PBY-5A at 80 nm (`#1567`) and struck (below): transport 1 hit. (4,2), then halts at (5,1) at 1559 to work cargo (`#1850`). Its landing forms a Japanese base, "Landing at (5,1)" (`#1851`), and 2,141 men land that afternoon (`#1852` to `#1872`). | Morning: 3 SBD-2, 6 SB2U-3, 7 B-17E at task force 4, 80 nm: 5 hits on transport 1 (`#1593` to `#1616`). Afternoon: 1, 5 and 6 at task force 4's reported (2,3), 106 nm: found nothing (`#1755` to `#1758`). | "Holds nothing in reach and nothing to close on, so it is routed to (14,-6)" and (15,-6) (`#1441`, `#1443`). Back at their stations by 0440. |
| 5, 7 June | Orders stand. | At (5,-3). Morning raid, 61 bombers (`#1984` to `#2004`). | The other 2,366 men land in the night (`#1943` to `#1956`); unloading done at 1222, 522.295 t of supply ashore (`#2155`). | Morning: 1, 5 and 4 at the carriers' reported (6,-3), 144 nm: nothing there (`#1996` to `#2007`). Afternoon: 1, 3 and 3 at (5,0), 40 nm: nothing there (`#2105` to `#2108`). | "Does not close: at the balanced risk it closes on a carrier, and no contact ... is of that kind (contact 4, a battleship)" (`#1896`, `#1898`): the carriers were reported as a battleship force the evening before. VB-6 sees task force 4 at Midway at 327 nm (`#1970`). |

## What was found, when, and by whom

| When | Who found | What | How reported |
| --- | --- | --- | --- |
| D1 morning (3 June) | PBY-5s of Midway, at 708 nm | Task force 4 at (-14,4) | "45 ships, cruiser" (`#330`), contact 1, dropped on D3 after 5 pulses (`#1155`) |
| D2 morning (4 June) | VS-6 from Enterprise, at 317 nm; VS-5 from Yorktown, at 356 nm | Task force 3 at (5,-3) | "21 ships, carrier", then "22 ships, transport" (`#668`, `#671`), contact 2 |
| D2 afternoon | VS-6, 317 nm | Task force 3 | "21 ships, carrier" (`#882`) |
| D3 morning (5 June) | VB-6, 208 nm; a PBY-5A, 160 nm | Task force 3 | "20 ships, carrier" at (5,-3); then "21 ships, cruiser" at (4,-3), a hex off (`#1162`, `#1170`) |
| D4 morning (6 June) | A PBY-5A, 80 nm | Task force 4 at (3,3) | "45 ships, battleship" (`#1568`), contact 3 |
| D4 afternoon | VS-6, 317 nm | Task force 3 at (5,-3) | "21 ships, battleship" (`#1743`), contact 4 |
| D5 morning (7 June) | VB-6, 327 nm; the PBY-5s at 0 nm | Task force 4 at Midway | "46 ships, battleship" (`#1971`), contact 5 |

The Japanese side found nothing at sea in five days: its six floatplanes searched 100 to 170 and then 110 to 180 degrees every daylight pulse, toward Midway, "threats: base:1 (Midway)" (`#223`). It ended with no contact. It knew of Midway's garrison only by the men in contact after the landing (2,707 on D4, 2,949 on D5).

## Every strike, with its losses

Aircraft lost in the air or on the ground, from the events; the patrol's and the guns' shots are in the events cited.

| When | Strike | Aircraft | Target | Hits | Lost |
| --- | --- | --- | --- | --- | --- |
| D2 morning | Midway's bombers | 19 SBD-2, 17 SB2U-3, 17 B-17E; 10 F2A-3 and 3 F4F-3 as escorts | Akagi, 160 nm | 4, 3 and 1 bomb hits: flotation 18.1, fire 13.5, flight deck degraded (`#712` to `#738`) | 6 SBD-2, 1 F2A-3; Akagi's patrol lost 5 A6M2 |
| D2 morning | The four carriers' bombers | 18 + 18 + 18 + 27 + 16 + 18 + 18 + 18 = 151, with 43 A6M2 | Midway | 24 hits: 300 t of supply, 63 men, airfield damage 10, 1 aircraft (`#715` to `#757`) | Midway's patrol lost 4 F2A-3 to the escorts, 1 on the ground; no Japanese bomber lost |
| D2 afternoon | Midway's bombers | 8 SBD-2, 13 SB2U-3, 11 B-17E; 5 F2A-3 and 3 F4F-3 | Akagi | 1, 2 and 1 hits: flotation 29.0, fire 16.1, flight deck out (`#898` to `#911`) | 8 SBD-2, 6 SB2U-3, 1 F2A-3; 2 A6M2 |
| D3 morning | Midway's bombers | 7 SB2U-3, 9 B-17E; 3 fighters | Contact 2 at (4,-3), 183 nm | Found nothing (`#1193`) | none |
| D3 morning | The carriers' bombers | 92 (Kaga, Soryu and Hiryu: Akagi's deck is out) | Midway | 17 hits: 250 t, 16 men, airfield damage 22 | 1 F2A-3; 1 D3A1 |
| D4 morning | Midway's bombers | 3 SBD-2, 6 SB2U-3, 7 B-17E; 3 fighters | Transport 1, 80 nm | 0, 3 and 2 hits: flotation 40.3, fire 27.7 (`#1593` to `#1616`) | 2 SBD-2 |
| D4 morning | The carriers' bombers | 75 | Midway | 16 hits: 450 t, 7 men, airfield damage 26, 1 aircraft | 3 F2A-3 and 1 F4F-3, 1 F2A-3 on the ground; 2 D3A1 |
| D4 afternoon | Midway's bombers | 1 SBD-2, 5 SB2U-3, 6 B-17E | Contact 3 at (2,3), 106 nm | Found nothing (`#1755`) | none |
| D5 morning | Midway's bombers | 1 SBD-2, 5 SB2U-3, 4 B-17E | Contact 4 at (6,-3), 144 nm | Found nothing (`#1996`) | none |
| D5 morning | The carriers' bombers | 61 | Midway | 20 hits: 100 t, 28 men, airfield damage 38, 3 aircraft | 1 F4F-3, 3 F2A-3 on the ground; 1 D3A1 |
| D5 afternoon | Midway's bombers | 1 SBD-2, 3 SB2U-3, 3 B-17E | Contact 5 at (5,0), 40 nm | Found nothing (`#2105`) | none |

In all, 39 Allied aircraft were lost to the enemy and 11 Japanese. Wear wrote off 25 more Allied aircraft and 21 Japanese, nearly all on search sorties (the flying boats' 1,796 nm sorties wear 0.41 of an aircraft each: `#343`). The objective "aircraft destroyed" counts both: 64 Allied and 32 Japanese. The American carriers' dive bomber groups that searched ended with more aircraft damaged than ready (VB-3 6 ready and 10 damaged, VS-5 5 and 11), with no combat.

## The invasion force and what landed

It sailed on day 1 at hour 0 and held 8.8 kn the whole way: 105.6 nm each night and 52.8 nm each daylight pulse. It reached Midway's hex at 1559 on 6 June (D4), the hour of statement 14. It landed 2,141 men that afternoon and the other 2,366 in the night of 6 to 7 June: all 4,507 (the Ichiki Detachment's 2,507 and the 2nd Combined Special Naval Landing Force's 2,000), and 522.295 t of the 533 t aboard. It was never turned back: the lift is turned back when it is hurt and a carrier is near, and the Japanese side never held a contact of an American carrier. Transport 1 was hit five times on the morning of D4 and landed her men all the same.

Ashore, neither side attacked: with fewer than three days left the staff begins no attack. Both sides end with men ashore in the one hex: the garrison 2,064, the landing force 4,507. Their own estimates of each other at the end: Allied 6,204 Japanese ashore, Japanese 2,949 Allied.

## The score by line

| Side | Line | Count | Points |
| --- | --- | --- | --- |
| Allied | Ships sunk | 0 | 0 |
| Allied | Ships out of action over 66 days | Akagi (88 days), transport 1 (81 days) | 5 + 1 = 6 |
| Allied | Aircraft destroyed | 32 | 1.6 |
| Allied | Enemy men lost | 0 | 0 |
| Allied | **Total** | | **7.6** |
| Japanese | Ships sunk | 0 | 0 |
| Japanese | Ships out of action | 0 | 0 |
| Japanese | Aircraft destroyed | 64 | 3.2 |
| Japanese | Enemy men lost | 114 (2,178 to 2,064, all to bombs) | 1.14 |
| Japanese | Men landed at Midway | 4,507 | 45.07 |
| Japanese | **Total** | | **49.41** |

The margin is 41.81 to the Japanese side: a decisive victory (minor 5, major 15, decisive 30). The landing alone is 45.07; without it the game is a stalemate either way (Allied 7.6 against Japanese 4.34).

## What the classes with surface weapons not modelled took away

Nothing, in this game. No two hostile forces were ever in the same hex at sea: the American carriers came no nearer than 208 nm to any Japanese force, and task force 4 met no Allied ship on its way or at Midway, where there was none. So neither the screens' guns nor the Close Support Group's (Kumano, Suzuya, Mogami and Mikuma, whose torpedo is not on file, and Yura) were ever called for. No force bombarded Midway: no order asked it.

## Which way each side's search looked, and what Wake did to it

- **The American carriers' dive bombers** searched 200 to 270, then 200 to 280 or 290 once they held the carriers' contact, and 190 to 290 at the end: "threats: base:2 (Wake) at (-24,16); the narrowest arc that holds them is 230 to 240, widened by 30 degrees at each end" (`#113`). Wake set the arc. It lies across the Japanese carriers' station from (14,-6), and so the arc found them: on 4 June at 317 nm, and four times after. Rule 102's reasoning held.
- **Midway's flying boats** searched 200 to 000 (to 020 on some days), the waters denied and Wake (`#149`). They found task force 4 on the first morning at 708 nm and again at 80 nm on 6 June, and task force 3 once, a hex off, on 5 June; they never found it on 3 June (chance 0.21 and 0.24 at 480 nm).
- **The Japanese floatplanes** searched 100 to 170 and then 110 to 180, toward Midway alone (`#223`). They never looked east or north-east, where the American carriers lay. Wake, being Japanese, did nothing to their search.

## Fuel

No ship ran out of fuel. The lowest at the end were Nagara at 60 per cent and the Kagero- and Yugumo-class destroyers of task force 3 at 67 per cent, after the run at 28.3 kn on the first day; the American destroyers ended near 95 per cent.

## Midway's stock each evening

| Evening | Supply | Aircraft ready |
| --- | --- | --- |
| Start | 4,000 t | 106 (5 SB2U-3 damaged) |
| D1, 3 June | 3,813.59 t | 94 |
| D2, 4 June | 3,260.885 t | 57 |
| D3, 5 June | 2,895.06 t | 53 |
| D4, 6 June | 2,345.52 t | 43 |
| D5, 7 June | 2,172.59 t | 40 |

1,100 t of the 1,827.41 t spent were destroyed by bombs; the rest went on sorties, upkeep and the garrison. The stock never came near to stopping an aircraft (rule 94).

## Against what rules 101 and 102 expected

- **Rule 101, the landing likely and the game turning on the carriers.** The first half held to the hour: the force reached Midway at 1559 on 6 June, the men landed with no fight and scored 45.07 in full. The second half did not: the game did not turn on the carriers, because there was no carrier battle. The landing decided it.
- **Rule 102, the Japanese carriers found first.** Held: they were found on the morning of 4 June and the American carriers were never found. They went at full speed to the hex 160 nm north of Midway and were there by the first evening, as said. Their floatplanes looked toward Midway, and the American carriers' search looked toward Wake and across them, as said.
- **What neither rule said.** Found first did not mean struck first. The American carriers decide whether to close only at a day's start, on the contacts then held: on 4 June the contact came in the morning, a pulse too late; on 5 June they closed to 208 nm of where the carriers were, eight miles beyond the SBD-3's 200 nm, aiming at a reported position a hex off; on 6 and 7 June the carriers were held as a battleship force and the staff closes at balanced only on a carrier. Meanwhile the Japanese carriers, serving the take, spent every morning on Midway (rule 45) and never searched for ships.

## What the architect may want to look at (not mended)

These are reported, not changed; nothing was moved to make the game come out otherwise.

1. The American carriers' staff closes only at a day's start and only on a contact held as a carrier at the balanced risk; with search reports that call the carrier force a transport, cruiser or battleship force half the time, it closed once in five days.
2. Closing aims at the contact's reported hex and stops at the first hex within the 175 nm "normal reach of its bombers" (the TBD-1's), so a report one hex off left the force 208 nm from the carriers, beyond the SBD-3's 200 nm.
3. The Japanese floatplanes' arc is set by the threats the side knows, Midway alone, so the Japanese side cannot find a carrier force it does not already suspect. This is rule 34 as the game has it: the Japanese side knew of no American carrier.
4. Wear on long search sorties costs as many aircraft as the enemy did (25 Allied and 21 Japanese written off), and those count as aircraft destroyed for the other side.
