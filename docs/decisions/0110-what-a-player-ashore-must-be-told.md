# 0110: What a player commanding ashore must be told

Status: a plan by the architect, written on Nov 14, 2026, from the account of a model that commanded the Allied side for the first five days of the lengthened slice with the game's tools alone (`crates/sim-cli/tests/recorded_games/full_slice_five_days_sonnet/account.md`). It landed, took both bases and built the airfield, and it named what it could not work out. Items 1 to 9 change what a side is told of its own affairs and are the architect's, inside the agent interface the designer approved (decisions 0047 to 0051). Item 10 changes what a side believes: the designer confirmed it on Nov 14, 2026. Built on Nov 15, 2026 (scenario 38; rules label 9, agent interface version 5): see "As built". No rule of the fight, of supply or of work changes.

Summary: What a player commanding ashore must be told, from a model's five days in the Allied seat: why a unit made no work, the arrivals that are due and what they wait for, the rule of fatigue, whether a base counts as open today, what a base without fuel means for the ships there, when a force has nothing left to unload, a landing summed up for each force, and one hex's bases shown together; and a report that says when a side's men find no enemy in their hex

Standing: Item 10 confirmed by the designer, Nov 14, 2026; the rest settled by the architect. Built, Nov 15, 2026

## What the account showed

The model was given the tools' own descriptions and nothing else, and it read no file. It did the hard part without help: it worked out the landing from the order's description and it worked the first time. What it could not do was judge. For three days it attacked men who were marching away, wore its units out, and stopped its own engineers from working, because nothing told it that the enemy was leaving, that a contested hex stops work, or what fatigue does. It never learned when aircraft would come to the field it had built.

Each item below is something the side already has in its own view. None gives it anything of the enemy it could not have had.

## What a side is told of its own affairs

1. **Why a unit made no work.** When a unit set to work, or a working unit, makes none on a day, the briefing says why: its hex was contested, and whether by its own side's attack or the enemy's; it marched; it was not fed; its side holds no base in the hex; the job is done. A day's rehearsal or training not counted is explained the same way.
2. **Arrivals that are due.** The briefing lists the side's scheduled arrivals: the day, the group, the base, the aircraft and pilots. An air group that is empty and waiting says so on its line, with the day its arrival is due and what the arrival waits for: the base to be the side's, an airfield, the field's damage below the level. When an arrival waits past its day the briefing says which of the three holds it.
3. **The rule of fatigue, in the tools' text and on the unit's line.** The text that describes posture says what a day of attack, a failed day, a day of march and a day under fire add, what a day of rest takes off, the level above which a unit stops attacking, and that fatigue cuts the men who fire. A unit's line gives its fatigue, the men who would fire today, and what one more day of attack would make it.
4. **Whether a base counts as open today.** An objective of a base open at the end of each day rests on the side's own facts: supply at the base and an aircraft ready. The briefing says for each such objective whether today would count and, if not, which of the two is missing. It no longer says nothing shows it.
5. **What a base without fuel means.** In place of a bare warning that a base has no fuel and a force is stopped there, the briefing gives, for each force at such a base, the fuel it has, the fuel its passage home needs, and whether it can get there.
6. **When a force has nothing left to do.** A force with an unload order that has landed everything, or a load order that is complete, is named as free, with what it still carries.
7. **A landing summed up.** The day's landing is one line for each force: the men and tons landed that day, the men and tons still aboard, and when it will finish at the present rate. The lines for each ship in each pulse are given by `explain`, not in the briefing.
8. **One hex, shown together.** Where a side holds more than one base in a hex, as after a landing takes a base, the briefing and the list of bases show them as one place: the hex's stock in all, with each base under it and which one a landing formed. Nothing of the rules changes: they are still two bases.
9. **Losses name their cause.** A line of aircraft written off says to what: wear on a mission, a hit on the ground, a landing elsewhere.

## What a side believes

10. **A side's men report an empty hex.** Today a ground report is made where both sides have men ashore in a hex. So when the enemy's last man leaves or is lost, the reports stop, and the side goes on being shown its last estimate of men who are no longer there. The model's estimate of the enemy at Lunga stood at about 1,580 for days after the garrison had gone. The correction: on a day when a side has fighting men ashore in a hex and the enemy has no man there, the side is given a report that its men find none, and its estimate for that hex is cleared. It comes through the one way a belief changes, a report. It tells the side only what its own men, standing in the hex, could see. It says nothing of where the enemy went.
11. **An estimate carries its day.** Every estimate of the enemy ashore is shown with the day it was made, in the briefing and in the answers that give it, so that a player can see when he is looking at old news.

## What it does not do

- **It does not tell a side that the enemy is falling back.** Men marching out of a hex are still in it until they have gone, and the daily report of the hex still estimates them, as wrong as it ever was. A side learns they have gone by the report of item 10.
- **It does not tell a side what its fire did.** There is no claim of men killed ashore, as there is of ships sunk. The daily estimate is what a side has; whether a claim is wanted is a question for a later record.
- **It does not fold a landing's base into the base it takes.** That would be a rule; item 8 only shows them together.

## What it costs

- **The briefing, the lists and the tools' text** in `sim-agent`, with their schemas and the golden briefings remade; the agent interface's version rises.
- **For item 10,** a new report and its event with its row in `side_events.rs`, a line in the rulebook's chapter 7 ("What a side knows of the enemy ashore"), and a hand calculation of one case: men in a hex, the enemy leaving, the estimate cleared on the day he is gone and not before.
- **The recorded games are regenerated,** and the game of the lengthened slice is worth playing once more afterwards, to see what a player still cannot work out.

## Rule card, for item 10

| | |
| --- | --- |
| Reads | Each side's fighting men ashore in a hex; the other side's men ashore there; the side's estimate for the hex |
| Writes | A report to the side, and through it the side's estimate for the hex; the event |

## Question for the designer

Answered on Nov 14, 2026.

1. **Item 10:** when a side's men stand in a hex and find no enemy there, the side is told so and its old estimate is cleared. Agreed.

## As built

Built on Nov 15, 2026, in one pull request. Scenario 38 (`data/scenarios/38_an_empty_hex.toml`, `tests/scenario_38_an_empty_hex.rs`) proves item 10 on the frozen figures; every number of its hand calculation holds.

**Item 10.** A new report, `Report::NoneFound`, goes in through `BeliefState::apply_report` and removes the side's estimate for the hex; its event is `GroundFoundEmpty` (side, hex, and the men and day of the estimate it cleared), seen whole by its own side and not at all by the other. It is tried at the end of each hex's part of the ground combat phase, where today's reports are made, for a side that has men ashore where the enemy has none; it is made only if the side holds an estimate there and has men of a fighting unit there. It draws nothing. Its causes are the event of the estimate it clears and the day's fight in the hex, if there was one: an enemy's march out or surrender is not cited, since the side may not see it. There is no new state (estimates were already hashed), so the state version stays 24. The rules label is 9: a game saved under 8 where a side holds an estimate of a hex its men stand in after the enemy has gone plays differently.

Read more closely than the record:

- **Scenario 24's hand calculation stated the old rule:** that each side keeps its estimate once the enemy's last man is gone. Three of its tests failed on the build. The architect restated it on Nov 15, 2026: on the day unit 3 loses its last man the Allied side's men find the hex empty and its estimate is cleared; the Japanese side, with no man there, keeps its own. No other number of that calculation moved.
- **The scenarios whose events change:** 7 (the full slice: the estimate at Lunga is cleared on day 4, 1,580 of day 3, and again on day 19, 686 of day 18), 32, 33 and 37 (each a garrison that falls back and leaves the hex), and 38 (new). No other scenario changes.

**Items 1 to 9 and 11** change the briefing, `list_bases` and the tools' text; the agent interface's version is 5 (a field of the fuel warning changed shape). The view gains two answers of the side's own units: the men a unit would fire with now, and its fatigue after one more day of attack. Where the build read the record more closely:

- **Item 1:** a finished job gives no line, because the work rule records a day's work on a built airfield or full defences: no unit makes "no work" for that reason. The side's own attack is named when an own unit in the hex fired on attack; otherwise the enemy's. Rehearsal and training not counted have their own reasons (men aboard, at the ceiling, rehearsal full, the force not with the unit all day).
- **Item 2:** every arrival not yet delivered is listed, with whether it could be delivered now and, if not, which of the three conditions holds it. Arrivals of a group lost with its base, or on a ship sunk or at the yard, are not listed.
- **Item 3:** the scenario's fatigue figures are given once in the land units part and the rule in the `issue_orders` text; "one more day of attack" adds only the attack day, and the text says a failed attack, a march and strain add more.
- **Item 4:** the line says what today would give if the day ended now, and the day ends the side has seen at the base while it was its own.
- **Item 5:** one line for each force stopped in a hex where none of the side's bases has fuel, with its least fuelled ship's fuel against the passage home that `measure` forecasts. A hex where one base has fuel and another none no longer warns.
- **Item 6:** a force is named free on the day its unloading or loading completes, and on later days while it stands at a base with an unload order and nothing aboard.
- **Item 7:** the line's "finishes in about N days" is the longer of the men's and the tons' rate, rounded up, and is left out when part of what remains was not landed yesterday. The ship lines stay in the data, not in the text.
- **Item 8:** the hex's days of supply are printed once, under the hex.
- **Item 11:** every answer already carried an estimate's day; an opening estimate now reads "the opening estimate, day 0", and the text no longer says an estimate is never corrected but by a report.

The catch-all arms over event kinds that `docs/decisions/0108` named were already made exhaustive with decision 0107 ("As built"); a new kind did not compile until each place decided it, which this build confirmed.
