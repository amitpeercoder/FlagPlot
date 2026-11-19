# 0160: An opening appreciation: what a side believes of the enemy's places when the game begins

Status: a plan by the architect, written on Nov 19, 2026 and confirmed by the designer the same day with the three answers the architect recommended. Nothing is built yet. It follows the designer's remark on the table that an enemy base touched on the first day shows nothing but its name, while the directive from above speaks of what intelligence reports there.

Summary: A side begins a game holding a blank picture of every enemy place: its name, its position and who holds it, and nothing else until a report comes in during play. The plan gives a side an opening appreciation, the picture its intelligence handed it before the first day, in two parts. First, the 39 days is given opening estimates of the enemy's men ashore, by the rule the engine already has, from the record's own pre-landing estimates, which were wrong. Second, a side is told what a chart and an intelligence summary would tell it of an enemy base, whether it has an airfield and an anchorage and of what size, as last known and dated, by one new kind of belief. The enemy's types of unit are left for the intelligence design.

Standing: Confirmed by the designer, Nov 19, 2026. Step 1, the research and the data, is next; step 2 follows stage 4's trial

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
