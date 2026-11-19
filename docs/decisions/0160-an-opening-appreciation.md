# 0160: An opening appreciation: what a side believes of the enemy's places when the game begins

Status: a plan by the architect, written on Nov 19, 2026 and confirmed by the designer the same day with the three answers the architect recommended. Nothing is built yet. It follows the designer's remark on the table that an enemy base touched on the first day shows nothing but its name, while the directive from above speaks of what intelligence reports there.

Summary: A side begins a game holding a blank picture of every enemy place: its name, its position and who holds it, and nothing else until a report comes in during play. The plan gives a side an opening appreciation, the picture its intelligence handed it before the first day, in two parts. First, the 39 days is given opening estimates of the enemy's men ashore, by the rule the engine already has, from the record's own pre-landing estimates, which were wrong. Second, a side is told what a chart and an intelligence summary would tell it of an enemy base, whether it has an airfield and an anchorage and of what size, as last known and dated, by one new kind of belief. The enemy's types of unit are left for the intelligence design.

Standing: Confirmed by the designer, Nov 19, 2026. Step 1, the research and the data, is built (the Allied side of the 39 days holds two opening estimates, from Turner's Operation Plan A3-42 of 30 July 1942); step 2 follows stage 4's trial

## What is so today, read from the code

- **An enemy base is four facts.** `PublicBase` in `sim-core`'s view holds the number, the name, the hex and the side that holds it. A seat's `list_bases` and `get_entity` say so in as many words: "number, name, place and holder only".
- **An estimate of men ashore can be given at the start.** A scenario's `[[estimate]]` (decision 0092, item 16) gives a side a count of the enemy's men in a hex, held from day 1 with the grade "told". The staff reads it: it sizes a landing on it, and it does not count as knowing at balanced or at preserve the force (decision 0147, item 13), so a group is still sent to look. Scenarios 24, 38, 68, 74 and 80 use it. The 39 days gives none.
- **A side's own base has an airfield size and a port size,** which change in play: an airfield is built up (decision 0099) and damaged and mended (decision 0103).
- **The directive from above** (decision 0157) already says "Intelligence reports he is building an airfield at Lunga". The staff's own picture does not hold it.

## The rules

**A. The opening estimates of the 39 days.** No new rule: data.

1. The full slice and the setups made from it give each side an `[[estimate]]` for each enemy place its intelligence had a figure for before 7 August 1942. A place with no figure in the record gets none.
2. Each figure is the record's own estimate, not the truth, taken from a fact row in `data/oob` that cites what was read, by the rules of `docs/engineering/data-rules.md`. Where sources give different estimates each has its row and the architect settles which the scenario uses, with the reason on the row. Until the rows are in, the scenario's figures are marked placeholder.
3. An estimate "told" has no date in the engine but the first day. It is shown to a person as "told before the operation".

**B. What a side believes of an enemy base's works.** One new kind of belief.

4. A side holds, for each enemy base it has been told of or has seen, a belief of its works: the airfield's size and the port's size, each with the day it was learned and where it came from (told, seen from the air, or by men in contact). Either may be unknown.
5. A scenario gives the opening beliefs, as it gives the opening estimates: `[[works_estimate]]` with the side, the base, the sizes believed and nothing else. It may be wrong. A base not named is not known beyond its four facts.
6. A reconnaissance flight over the place (decision 0092's flight, which today reports men) also reports the works as they are, through `BeliefState::apply_report`, and so do men in contact in the hex. No new flight, mission or draw.
7. A base that changes side: the side that takes it knows its works exactly, as it knows any base of its own; the side that lost it keeps what it knew on that day as its belief, dated that day.
8. The staff does not read the belief in this plan. It is for the commander's picture. Whether the staff should weigh an enemy airfield it believes is there is a question for the doctrines that strike and neutralise, and is not opened here.
9. A seat is told it in `list_bases` and `get_entity` for an enemy base: `works` with the two sizes, the grade and the day, left out where nothing is held. The table's card of an enemy base shows it under the estimate of men.

**C. What is left out.**

10. The type of an enemy unit ashore, his aircraft on a field and his ships in an anchorage as standing beliefs. Ships and aircraft are held as contacts today; a standing picture of them, and unit types, belong to the intelligence design (`docs/design/`), which is not planned yet.

## Rule card

| | |
| --- | --- |
| Reads | The scenario's opening beliefs; the truth of a base's airfield and port sizes at the pulse a flight or men in contact report it; the report rule of decision 0092 |
| Writes | A side's belief state: one record of works for each enemy base, written only by `BeliefState::apply_report` |
| Reuses | The opening estimate of decision 0092, item 16; the reconnaissance flight and the report by men in contact of decision 0092; the grades told, seen from the air and men in contact |
| New | One belief record and its report; one scenario table; one field in two of a seat's answers; one event a side sees of its own report, decided in `side_events.rs` |
| Draws | None |

## What it costs

- The state gains a field in each side's beliefs: the hash version and the saved game are raised once and the golden files remade (decision 0057).
- The 39 days with opening estimates plays differently from today: the staff sizes its landings on a figure where it had none. Its generated files change, and so would the figures of any proof run after it. No proof already judged is run again.
- A research task for the pre-landing estimates of Guadalcanal, Tulagi and the places the Japanese side looked at, written as fact rows.

## The arithmetic of its cases

The step's test carries its hand calculation, written by the architect before it is delegated. The cases it will cover:

- A side told at the start that an enemy base has an airfield of size 2, where the truth is size 1: its card says 2, told, until a flight on day 3 reports 1, seen from the air, of day 3.
- A base with no opening belief: nothing is held until a flight reports it.
- A base taken on day 5 and lost on day 9, its airfield built from 1 to 2 on day 7: the side that lost it on day 5 believes 1, of day 5, until it sees otherwise; the side that lost it on day 9 believes 2, of day 9.
- The enemy is told nothing of what the side believes.

## The steps

1. **The research and the data** (A): the fact rows, and the estimates in the slice and its setups, with the slice's files remade.
2. **The belief of works** (B): the state, the report, the scenario table, the seat's answers, the table's card, the rulebook's chapters 3 and 7 and the glossary, with one scenario and its test.

## Questions for the designer

1. **Should the opening estimates be the record's, wrong as they were, or a fair count?** The architect recommends the record's: the commander of 1942 planned on a figure that was off, and finding that out is part of the game. The cost is that the staff, sizing a landing on it, may bring too many or too few until it has looked.
2. **Should a side know an enemy base's works at all without being told or looking?** The architect recommends no: told at the start where the record knew it, otherwise unknown until seen.
3. **Should step 1 come before stage 4's trial?** The architect recommends yes for the data, since the trial is played on what the side believes and is poorer with a blank picture; step 2 can follow the trial.

## Confirmed on Nov 19, 2026

The designer confirmed the plan as recommended:

1. The opening estimates are the record's own, wrong as they were.
2. A side knows an enemy base's works only where it was told at the start or has seen them.
3. Step 1, the data, comes before stage 4's trial; step 2, the belief of works, follows the trial.

## As built: step 1

The research is in `data/oob/land_unit_facts.csv` (ids 1700 to 1723, with the estimates already on file, 158 to 175, 199 to 238, 1584, 1585, 1587 and 1588) and the last four entries of `gaps.csv`. No row was changed.

**The architect's ruling on the rows (Nov 19, 2026).** The Allied side of the 39 days begins with two opening estimates of the Japanese men ashore, the figures of Admiral Turner's Operation Plan A3-42 of 30 July 1942, the last dated estimate before the landing and the one the landing was planned on:

- at Lunga's hex, (2,-4), base 1: 5,275 men, from rows 162 (Hough) and 163 (Zimmerman), both of the plan of 30 July;
- at Tulagi's hex, (3,-5), base 4: 1,850 men, the figure for Tulagi and Gavutu-Tanambogo together, which the slice holds as one place, from rows 232 (Hough, 30 July) and 230 (Zimmerman); row 233 (Miller, 30 July) gives the same.

The earlier estimates on file are not read: the division's of 20 July with Operation Order 7-42 (rows 161, 7,000 on Guadalcanal, and 231, 1,400 in the Tulagi area), CINCPAC's of 6 July (row 1700, Tulagi 1,400, and no place on Guadalcanal), the division's of 11 and 27 July (rows 1584 and 1585, 5,000 on Guadalcanal), and the totals for both places (rows 199 to 213 and 1701). The reason: the plan of 30 July is the last dated estimate before the landing, and the one the landing was planned on, so it is what the commander of 7 August believed. The 5,010 of the division's Phase II report (rows 171 to 175) gives no date, and which document it comes from is not stated (the gap of `gaps.csv` on the estimate documents themselves). Both figures were high, as the sources say: the record's own estimate, wrong as it was (the designer's answer 1).

The Japanese side gets no opening estimate: no Allied man was ashore before 7 August, and no Japanese figure exists (`gaps.csv`, "a Japanese estimate of Allied men ashore on Guadalcanal or Tulagi dated before 7 August 1942"). No other place has an estimate in the record, so none is given. The rows stay `unconfirmed`, as every row a scenario reads by a comment does; the ruling is recorded here and in the scenario beside the figures, as the slice records where its other figures come from (its land units cite their rows the same way). No derivation list or rules file holds the slice's land forces.

**The files.** `data/scenarios/07_full_slice.toml` gains two `[[estimate]]` tables after its land units, with a comment naming the rows, the ruling and the rows not read. `data/scenarios/staff/07_full_slice.toml` was made again by `make_staff_setup.py` (decision 0140's "As built"), which first made main's file again byte for byte; the only change is the same two tables and their comment. The three arrangements of stage 3's proof, `data/scenarios/stage-3-proof/07a_slice.toml`, `07b_slice.toml` and `07c_slice.toml`, were made again by `scripts/stage-3-proof/make_arrangements.py` from the new staff's setup, and carry the same two tables. The proofs of decisions 0153 and 0159 were run on the arrangement files as they were before this change (as made at commit af304144 and unchanged on main to bf48f924); the files now carry the opening estimates. Not changed: the frozen copy `data/scenarios/step2-frozen/07_full_slice.toml` (frozen figures go with frozen scenarios), and the files of the closed proofs of decisions 0128 and 0138 (`data/scenarios/second-proof/07*_week.toml`, `data/scenarios/stage-2b-proof/07a_week.toml`), which were made once by hand from the slice for a proof already judged and have no script that follows it.

**What plays differently.** Nothing of the engine changed: no state field, version or draw. On seed 1 the scripted slice plays as before, its events the same but for two "ground report ... (told)" events on day 0, a men-in-contact report at Lunga on day 1 that now says it replaces the estimate held, and one more event on day 1, the Allied men at Tulagi finding no enemy ashore and clearing the estimate of 1,850; every day's hash differs, as the belief is hashed. In stage 3's arrangements (a) and (c), where the Allied staff serves intents, the staff now sizes each landing on the figure: at Lunga it wants 10,550 men (2 for each of 5,275, at balanced) and the lift carries 10,240 fighting men, so it goes short by 310 with what it has; at Tulagi it wants 3,700 and the lift carries 6,075, all of which land, as a lift as it stands lands everyone aboard. It chooses the same forces as before, sends no group to look, and no plan waits: at balanced a *told* estimate does not count as knowing, as before, and the plan is released on day 1 as before. Every later event of seed 1 is the same.
