# 0176: The estimate: what a trial says of a course, in counts of worlds

Status: a plan by the architect, written on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 6 of its ten, section E, rules 13 to 15), which the designer confirmed. To be read by a reviewer who has not seen it written, and amended, before it is built.

Summary: A trial plays a course in a number of worlds. The estimate is what is read from them: in how many worlds the end was met, and by what day; in how many it was met and lost again; in how many it was not met; what it cost in men, ships and aircraft, as the lowest, the middle and the highest; what the trial's enemy lost; what was assumed; and where the worlds part by a thing the side did not know. It speaks in counts of worlds and never in chances, and in places, days, men, ships and aircraft and never in points. With it comes one thing the trial lacked: a way to say that the men at a place may be more than the side's estimate, and not known

Standing: A plan, written by the architect on Nov 19, 2026, inside decision 0161. Under review

## Why

Decision 0161, rule 13, lists what an estimate gives. Its case to be worked by hand is "a landing of 600 men against a place whose garrison is believed to be 200 or 600, not known which, four worlds: taken in the two with 200 by a stated day, not taken in the two with 600". A side's belief of the men at a place is one number (decision 0092), and the three standings of decision 0173 are for ships. Nothing built can say "200 or 600".

What is so today, read from the code:

- **A trial gives back, for each world,** its number, its assumptions, each day's hash and staff hand-ins, the commands of the course it refused, and the world as it stands at the end (decision 0175, amended, 10). `sim-trial` may not read a view of a world: its source test forbids the word.
- **The day a place changes side is an event both sides see** (`BaseChangedSide`), on the evening of the fight. The staff marks a take's plan complete and hands in "met" the morning after. Nothing records "met and lost again"; a base that changes side back is another such event. A plan is also marked complete when its intent is withdrawn, so a plan's state does not say that an end was met.
- **A side sees its own losses exactly,** as events: men lost in a fight, to fire, unfed, with a ship, by surrender; ships sunk or lost with a base; aircraft lost in the air, on the ground, ditched, worn out, with a ship or a base. The list the objectives count is in `objectives.rs`. A ship out of action is no state: it is worked at the end from the ship's days of repair (decision 0088).
- **The plan order** of decision 0159 gives, for an intent, the men wanted and to hand, what it waits for and the day expected ashore. It is worked from a side's view by the staff.
- **The house rule for a middle value** is the value at place ceil(n / 2), counting from 1, never an average of two (`docs/step2-spec.md`).

## The rules

### Men believed at a place and not known

1. **`[[believed_men]]`** in a scenario states men of the enemy's that a side believes may be ashore at a place beyond those it holds an estimate of. Its keys: `side`, the side that believes it; the place, `at_base` (a base on that side's map) or `q` and `r`; `men`, 1 or more; `standing`, `present`, `absent` or `unknown`; and `placeholder` with a `note`, as a believed force has. It is loaded, checked and kept as a believed force is (decision 0173, 8 and 9), and is no more state or belief than one.
2. **In a world of a trial** they are one more fighting unit of the enemy's in that hex, made as the unit of an estimate is (decision 0174, 6): in every world where present, in none where absent, and where unknown in the worlds its turn gives it (decision 0174, 14). The unknown entries take their turns after the believed forces of unknown standing, in the order of the scenario's list. The assumptions name the unit, its men and its standing.

### What an estimate is read from

3. **An estimate is of one intent of the side tried,** named by its number, in a trial's worlds as played. It may be given a second trial of the same worlds with an empty course, things as they stand, to compare with (rule 12).
4. **The end of an intent is a place being the side's.** For "take a place", its hex. For "control an island", every named place of the island. For "hold", its base. The end is met in a world on the first day at whose end the side holds a base at each of those places and the enemy none; it is read from the events of a base changing side that the side itself sees, and from who held the place when the world was built. An intent of another kind has no end an estimate can read: its estimate gives the costs and says so.
5. **Met and lost again:** the end was met and, on a later day of the horizon, a place of it changed side away from the side. Such a world is counted apart from the worlds where it was met and kept.
6. **`sim-trial` reads a world it played** through the views of its two sides, in the one part of the crate that makes estimates, and nowhere else. A world of a trial is the trial's own invention (decision 0175, 7): reading its enemy's view is not reading the enemy.

### What it says

7. **How it went,** in counts of worlds: the worlds played; of them, those in which the end was met and kept, with the earliest, the middle and the latest day it was met; those in which it was met and lost again; and those in which it was not met by the horizon. A world that could not be played is counted and named, and is in none of the others.
8. **What it cost the side,** over the days of the horizon, each as the lowest, the middle and the highest over the worlds played: men lost to the enemy (in a fight, to fire, with a ship, by surrender); men lost unfed; ships sunk (and lost with a base); ships out of action at the end, a ship afloat whose days of repair are 1 or more; aircraft lost, by the list the objectives count. Each is the side's own count from its own events and its own ships.
9. **What the trial's enemy lost,** the same five, read from the other side of each world, and marked as the trial's own reckoning: it is not a claim, not a report, and changes no belief.
10. **The middle** is the value at place ceil(n / 2) of the worlds' values in order, counting from 1.
11. **What it needs and has, and when it could go:** the plan order of the intent, as the side's staff works it from the side's view of world 1 at the end of day 1: the men wanted and to hand, what it waits for, and the day expected ashore. The side's half is the same in every world, so world 1 stands for all.
12. **What its other plans lost.** Where a trial of things as they stand is given: for each other intent of the side's own that has an end (rule 4), the worlds in which it was met with the course and without. A plan met in fewer worlds with the course is named as displaced, with the two counts.
13. **What was assumed:** every assumption of the worlds' pictures, each once, with the number of worlds it was made in; and any leave the course gave that the side had not given.
14. **Where the worlds part.** For each force or body of men of unknown standing: the worlds with it and the worlds without, each with its counts of rule 7 and its middles of rule 8, side by side. It is shown where the two differ in any count or middle, and left out where they do not.
15. **No chance, no points, no advice.** An estimate gives no share of worlds as a chance, no score, and no word on what to do. It is data, with a rendering in words for a reader, and reading it changes nothing.

## What it leaves for later

- **Where an estimate is kept and who is shown it:** the session and the seat's tool, step 8. **Dates side by side:** step 7.
- **Whether the estimate is true:** the spreads and the wrong pictures of decision 0161's proof (conditions 7 and 8), and the reader who is given an estimate alone (condition 11).
- **The end of an intent that is not a place:** a base kept supplied, waters denied.

## Rule card

- **Reads:** a trial's worlds as played, through their two sides' views; the plan order of the side's staff; the pictures' assumptions.
- **Writes:** nothing in any game. It returns an estimate.
- **Reuses:** the trial of decision 0175; the believed forces' standings and turns of decisions 0173 and 0174; the plan order of decision 0159; the losses the objectives count.
- **New:** `[[believed_men]]`; in `sim-trial`, the estimate and its words.

## What it costs

- **No live game changes:** no state, no draw, no hash.
- **`sim-trial`'s source test** lets the estimate's own module take the views of a world the crate played.
- **The loader** gains one kind of entry; `docs/commands.md`, the rulebook's chapter on the trial and the glossary gain the estimate and its words.

## The hand calculation

It is in `tests/scenario_95_the_estimate.rs`, written before the code: decision 0161's own case. On scenario 68's figures, where every shot is whole and hits, a landing of 600 men at a place the side estimates at 200 and believes may hold 400 more: four worlds, the place taken on day 4 in the two with 200 and not taken in the two with 600, with what each cost counted man by man.
