# 0110: What a player commanding ashore must be told

Status: a plan by the architect, written on Nov 14, 2026, from the account of a model that commanded the Allied side for the first five days of the lengthened slice with the game's tools alone (`crates/sim-cli/tests/recorded_games/full_slice_five_days_sonnet/account.md`). It landed, took both bases and built the airfield, and it named what it could not work out. Items 1 to 9 change what a side is told of its own affairs and are the architect's, inside the agent interface the designer approved (decisions 0047 to 0051). Item 10 changes what a side believes and needs the designer's confirmation. Nothing is built. No rule of the fight, of supply or of work changes.

Summary: What a player commanding ashore must be told, from a model's five days in the Allied seat: why a unit made no work, the arrivals that are due and what they wait for, the rule of fatigue, whether a base counts as open today, what a base without fuel means for the ships there, when a force has nothing left to unload, a landing summed up for each force, and one hex's bases shown together; and a report that says when a side's men find no enemy in their hex

Standing: Plan. Item 10 needs review. Not built

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

1. **Item 10:** when a side's men stand in a hex and find no enemy there, the side is told so and its old estimate is cleared. I recommend it: it is what the men on the ground would report, and without it a player fights an enemy who has left. The other choice is to leave the estimate as it is and only show its day, by item 11.
