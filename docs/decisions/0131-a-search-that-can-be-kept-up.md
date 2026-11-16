# 0131: A search that can be kept up: one sortie a day, its own fatigue figure, and bombers that take a turn

Status: settled by the architect on Nov 16, 2026, on the designer's choice of the same day, after stage 2's second proof (decision 0128, "As run") and the research he asked for (`docs/research/oob/search-tempo.md`). It adds one setting of an air group and one figure, and changes three doctrines. Not built. It is built after decision 0130, and the second proof is run again when both are in.

Summary: A search group may be set to fly its search once a day, in the morning; a search tires a group by a figure of its own, sized so that a flying boat group can search to 700 miles every day as the record has it; and where a base's or a force's only searchers are its bombers, the least tired bomber group that can search flies the morning search while the rest hold the strike

Standing: Decided by the designer, Nov 16, 2026, on the research. Not built

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
