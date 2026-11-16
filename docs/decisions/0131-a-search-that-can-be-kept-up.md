# 0131: A search that can be kept up: one sortie a day, its own fatigue figure, and bombers that take a turn

Status: settled by the architect on Nov 16, 2026, on the designer's choice of the same day, after stage 2's second proof (decision 0128, "As run") and the research he asked for (`docs/research/oob/search-tempo.md`). It adds one setting of an air group and one figure, and changes three doctrines. It is built after decision 0130, and the second proof is run again when both are in. Built on Nov 16, 2026 ("As built" below).

Summary: A search group may be set to fly its search once a day, in the morning; a search tires a group by a figure of its own, sized so that a flying boat group can search to 700 miles every day as the record has it; and where a base's or a force's only searchers are its bombers, the least tired bomber group that can search flies the morning search while the rest hold the strike

Standing: Decided by the designer, Nov 16, 2026, on the research. Built

## Why

In the second proof the Japanese side missed search days in the week on 317 of 1,000 seeds: its only searchers, two floatplane groups of five, were above the fatigue level after a day and rested. Decision 0115 had said a group that searches at long range is exhausted in a day and left it to be weighed. The designer asked whether crews were really so tired, and whether bombers were used to search.

The record, as far as it was read:

- **Crews were not grounded by daily search.** Midway's flying boats flew 22, 21, 22, 22, 22 and 22 aircraft on six days running to 700 miles, one sortie a day (rows 1182, 2292 to 2296). A patrol plane commander said in April 1943 that since 7 August the patrol planes had never missed a day of search (2286). The cost a source states is for the heavy bombers: squadrons relieved after a week forward (2272, 2273). No source gives a limit of hours or a unit worn out by searching.
- **A long search is one sortie a day.** A search to 700 miles is some twelve hours in the air. The game flies it in the morning and again in the afternoon.
- **Bombers searched, heavily.** Lexington sent 18 dive bombers on the search of 8 May and 16 in the attack group (2305 to 2307). Enterprise sent 23 aircraft on the afternoon search of 24 August and held 25 (rows 157, 238). Where there were two carriers, one searched and the other held its strike (2303, 2308). At Midway the heavy bombers were kept off the search so as not to weaken the strike (2301).

Against that, the game's rule: a group that flew in a daylight pulse gains 3 and 2 for each 100 nm of its sortie, and every aircraft of a search group flies in both daylight pulses. A search to 700 nm gains 31 a pulse, and the group is close to grounded in a day and a half.

## The setting: search once a day

1. **A setting of a group: search once a day,** off until an order sets it (`set_search_once`, on or off). With it on, a group on the search mission flies its search in the morning pulse and not in the afternoon. In the afternoon it has not flown, and rests as a group that did not fly does.
2. With it off the group searches in both daylight pulses, as now.

## The figure: what a search costs

3. **A search tires a group by its own figure for distance:** `fatigue.toml: search_fatigue_per_100_nm`. The figure for the sortie itself is unchanged. With none given, the figure for every sortie serves, and the game is the game it was: the frozen step 1 figures give none.
4. **The live figure is 0.14.** A search to 700 nm flown once a day then gains 3 + 14 x 0.14 = 4.96 in the morning and loses 5 in the afternoon: the group can search every day, as Midway's did. It is sized from those rows and from nothing else, and is a placeholder in every other respect: it says a search crew tires far less for each mile than a strike crew, which the record supports only by what the units kept up.
5. **Strikes, raids, patrols and escorts keep the figure they have.**

## The doctrines

6. **A search is flown once a day** by every doctrine that sets a group to search (the cover, the search of waters, the carrier force), at preserve the force and at balanced. At press it is flown in both pulses.
7. **Bombers that take a turn, on a carrier force.** Decision 0125's rule 4 has the lowest numbered group that can both search and strike do the searching where the force has no group that can only search. It is changed to the least tired of those groups that can fly, then the lowest numbered, chosen each morning. The rest strike.
8. **Bombers that take a turn, at a base.** Where a base has a strike task or a raid task and its cover, or a search task, has no group that can only search, the task that has the bombers gives the search to the least tired of its bomber groups that can search and can fly, then the lowest numbered, each morning, where it has two or more such groups. That group flies the cover's or the search task's arc that day and does not strike or raid. With one bomber group, it holds the strike.
9. **A group that searched yesterday and strikes today** is given the missions in turn by orders, as the rest rule gives rest and work in turn.

## What is checked again

The live figure moves, so every live scenario in which a group searches with the fatigue figures loaded plays differently: the air campaign, the 39-day slice, the soak. Their recorded outputs are remade. The measures of the air campaign and of the slice are run again and set beside what they were, and the report says which measures moved and by how much. A measure that agreed and now misses is named, and is put to the architect before the change is merged. The hand-calculation tests on the frozen figures do not change.

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| Search once a day | The group's setting, its mission, the pulse | Whether the group flies its search in the pulse |
| What a search costs | The search figure, or the general one; the sortie's distance | The group's fatigue |

## What it leaves out

- **A search flown by part of a group.** The record has whole units out daily, and with the figure above a whole group can be.
- **Relief of a squadron** after a week forward, which the record has for the heavy bombers.
- **A search that takes more than a day's light,** and night search.
- **Searchers that attack what they find.**

## What it costs

- **State:** one setting of a group, in the hash and named beside it; the state version rises. The rules label rises.
- **An order** added at the end, and one figure.
- **The rulebook's chapter 5** gains the setting and the figure; chapter 8 gains the doctrines' changes; the glossary gains search once a day.

## The hand calculation

`tests/scenario_59_a_search_kept_up.rs`: one base with a flying boat group searching to 600 nm for 12 days under a cover: every morning's fatigue and every order, once a day and in both pulses, with the search figure and without.

## As built

Built on Nov 16, 2026, in one step. The state layout is `flagplot-state-v34` (an air group's `search_once` and `search_once_order`, after its `strike_sizing_order`), the rules label 17, the saved game version 9, the doctrine `supply-16` and the agent interface version 14. Rulebook: chapter 1 (the order and the saved game), chapter 3 (who searches), chapter 5 (Air group orders, Fatigue gained and lost) and chapter 8 (Covering a base from the air, Searching waters, Operating a carrier force, the doctrine version); the glossary gains "Search once a day", "Search figure" and "Take a turn at the search".

**Where it is.** The rule is `World::searches_in` (`crates/sim-core/src/airstrike.rs`), read by the search step (`search.rs`) and by what flies in the air combat step; the figure is `FatigueFigures::gain_for` (`refdata.rs`), read by the fatigue step (`upkeep.rs`). The doctrines: `crates/sim-staff/src/search.rs` (the pure parts `once_a_day`, `least_tired` and `base_turn`; `work_turns`, which works out once each morning, before the cover is served, which bomber each base's cover or search task borrows), `air.rs` (the cover), `strike.rs` and `raid.rs` (the lender gives the borrowed group no order) and `carrier.rs` (`roles` and `decide`).

Readings the build took:

- **The setting in the afternoon.** With search once a day on, the group is not counted as flying a search in the afternoon by the search step or by the air combat step, so it has no sortie and rests by the rest figure. The search it flies cites the setting's order beside the mission and arc orders. A side's forecast of its own base's supply (`view/supply.rs`) counts one search sortie a day for such a group, to match the engine.
- **"The scenario's own file"** (scenario 59's figure of 0.15) is read as a fatigue file of its own, `data/reference/fatigue_placeholder_search_kept_up.toml`, the frozen figures with the search figure, with its frozen copy in `data/reference/step1`, as scenario 52 has its own surface combat file. The frozen `fatigue.toml` has no search figure, by rule 3, and `crates/sim-data/tests/frozen_figures.rs` exempts the key.
- **The breakdown** of a fatigue gain names `search_fatigue_per_100_nm` where it served and `air_fatigue_per_100_nm` otherwise.
- **The order of the orders** for a group that searches: the mission, then the setting, then the arc, as the hand calculation has it. The setting is ordered only where the group searches that morning and its setting is not what the risk wants (on at preserve the force and balanced, off at press); a group that rests is given no setting.
- **"A group that can only search"** is a group that is not a fighter, whose type has a search range and carries no weapon (the carrier doctrine's test of rule 4 of decision 0125, and the strike doctrine's bomber the reverse).
- **At a base (rule 8).** The borrowing is worked out for each cover and each search task in force with a released plan, in priority order, at a base none has yet been worked for. The lender is the first raid or strike-ships task in force, by priority, that has two or more bomber groups at the base that can search, can fly and are not held; whose bomber a group is, is read as the cover reads it (a raid's first, then a strike task's). The lent group is the cover's or the search task's that day whatever its forces limit says; it searches by the cover's rules and so rests above the level as any search group does. The raid or strike task gives it no order and its reason says it searches today for the borrower.
- **On a carrier (rule 7).** Where none of the groups that can do both can fly, the lowest numbered of them is the searcher, as before. The reason names each such group's fatigue and the aircraft it can fly.
- **The architect's ruling of Nov 16, 2026, on a carrier force that cannot move for want of fuel** (decision 0130, "As built", the 50 t case): where a ship of the force has no fuel aboard, or the engine's forecast refuses the passage to the task's hex as out of fuel, the staff gives the force no route and no speed order and does not weigh closing, hands in the cannot-meet notice with a reason naming the ship out of fuel and the hex where the force lies, and gives the air groups their orders as on any morning. Readiness and rules of engagement are given as on any morning. The stand doctrine of decision 0124 has no such hole: it routes a force only on a sortie whose fuel it has measured, and a ship that runs dry is short (decision 0130's ruling); a refused forecast gives no sortie. Scenario 58 gains the test `not_the_fuel_with_50_t_she_lies_dry_and_is_given_no_route_from_that_morning`: with 50 t no staff order is refused on days 1 to 3, and the notice is handed in from the first morning she lies dry, day 2, with no route or speed order.

**What it was checked against.** Scenario 59 (`data/scenarios/59_a_search_kept_up.toml`, on scenario 47's map and the frozen figures with its own fatigue file) came out as the hand calculation has it: three orders on day 1 (search, search once on, the arc 050 to 120 at 600 nm) and none after; on every morning fatigue 0, 6 aircraft in the morning and none in the afternoon, 4.8 gained and 4.8 lost, never a rest. So did the properties: at press (no setting order, mornings 0, 9.6, 19.2, 28.8, 38.4, 48, 57.6, 67.2, 57.2, 66.8, 56.8, 66.4, rests on days 8, 10 and 12, seven orders); without the search figure (mornings 0, 22, 44, 66, 56, 46, 36, 58, 48, 38, 60, 50, searches on days 1, 2, 3, 7 and 10); the rule alone; a strike of 1200 nm at 27 either way; the least tired bomber on a carrier and at a base (in views built for them, through the pure functions); the refused order. None is ignored.

**Earlier tests that failed, and the architect's rulings on them (Nov 16, 2026).** The build found eleven tests of scenario 47, sixteen of 48, one of 53, five of 55 and one of 58 failing on the search-once order the doctrine now gives at balanced, and the live soak's scripted acknowledgement refused; each went back to the architect marked ignored with what it got. He restated the hand calculations of scenarios 47, 53, 55 and 58 to what the build found (scenario 47: group 3's mornings 0, 22, 44, 66, 56, 46, 36, 58, 48, 38, 60, 50, searching on days 1, 2, 3, 7 and 10 in the morning only, orders 7, 0, 1, 1, 0, 0, 1, 1, 1, 3, 2, 1, eighteen, ending at 40; scenario 53: eleven orders on day 1, group 4 given three; scenario 55: nine on day 1 and group 2 searching in the two morning pulses only; scenario 58: eleven on day 1) and ruled that scenario 48 follows scenario 47's table. The tests were brought back to those numbers and pass, with the functions whose names carried the old numbers renamed. Two of scenario 48's variants did not follow plainly from the restated table and went back to him; he restated both to what the build found, and they pass: `a_limit_on_a_sub_intent_may_narrow` (group 3 is at 22 on day 2, not above preserve's 30, so day 2's orders are the two shares of 0.75 alone, without rest to group 3) and `a_parents_risk_set_after_its_childs_binds_the_child` (group 3 is on the search mission at 44 on day 3, above preserve's 30, so day 3 has three orders: the two shares and rest to group 3). Scenario 45's acknowledgement is data fitted to the seed-1942 game, not a hand-calculated number: by the ruling it moved from day 76 to day 78. The interrupt first fires on day 77, at or after day 76, in the staff's morning after the day's commands are taken, so an acknowledgement on day 77 is still refused ("has not fired") and day 78 is the first on which it is taken; the scenario file says why.

**Live scenarios.** The run baselines changed beyond their hash lines in scenarios 6, 6c, 7, 8, 45, 47, 48, 53, 55 and 58; every other differs only in its hashes. Scenario 5 does not change. The recorded games were remade with `regame.py`: `full_slice_five_days_sonnet` and `second_proof_week_sonnet` differ in fatigue and, in the second, in the staff's search-once orders and their reasons; the others only in their versions. No recorded game's result changed.

**The measures,** run on main and on this branch, 1000 runs, seeds 1 to 1000. The air campaign (scenario 6, `rabaul_guadalcanal.toml`): 7 agree, 1 at the edge, 2 miss on main; 7 agree, 2 at the edge, 1 miss here. One measure changed standing: `sbds_ready_on_day_21` (record 22) went from a miss (5th to 95th 27 to 33, median 30, no run holding it) to agreeing at the edge (21 to 30, median 25, 64 runs). `vmsb_232_ready_on_day_5` (record 9) moved from 8 to 12, median 10, to 4 to 10, median 8, and still agrees. No measure that agreed now misses. The 39-day slice (scenario 7, `guadalcanal_landings.toml`): 4 agree and 2 miss on both, and only the reported ridge measures moved (Japanese men lost at the ridge, 5th percentile 1060 to 1040). Cape Esperance's check was not run, nor the second proof.
