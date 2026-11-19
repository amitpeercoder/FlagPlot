# 0177: The window: the same course at two or three dates

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 7 of its ten, section F, rules 16 and 17), which the designer confirmed. It was read by a reviewer who had not seen it written, and amended (below), before any of it was built. The hand calculation is written. To be built when decision 0176's build has merged: it changes the same crate.

Summary: A commander who is shown what an operation would cost today wants to know what it would cost next week. The window tries the same course at two or three dates, in the same worlds, and sets the estimates side by side: what the side has more of at the later date, what the enemy did with the same days, and how it then went. The dates are taken from when something of the side's own changes, not at even steps. A later date is the same commands handed in on a later morning, with the days between played

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161, and amended on review. The build waits for decision 0176's

## Why

Decision 0161, rules 16 and 17. Its case to be worked by hand: "A take that could go on day 3 with one unit or on day 6 with two: two dates, and the second's estimate shows the men it has more and the days the enemy had."

What is so today, read from the code:

- **A course's commands each have a day of the world** (decision 0175, amended, 1), and the pictures and the draws of a trial do not depend on the course or on how many days are played (amended, 6 and 8). So the same worlds can be played with a course handed in on day 1 and on day 4.
- **A take chooses its units once,** on the morning its plan is made ready, from the units it may use that morning. A limit "not before a day" holds the release and does not move that choice. A course handed in on a later morning is chosen on that morning, with what is usable then.
- **With a count of the enemy the staff takes the men it wants and no more;** with none it takes every unit its ships can carry. So a later date sends more men where there is no count; or where the men wanted are more than are to hand on the earlier date and the earlier date goes all the same, at press or with nothing in reach. At balanced, where more men are in reach within the staff's horizon, the earlier date waits for them by itself (decision 0159, rule B) and the two dates send the same men.
- **A side's view gives the days its own things change:** the day a force, a ship, an air group or a land unit may first be used; the day aircraft fly in; the day a force is back from a yard; the day a supply run is due.
- **A trial's enemy, given no intent, orders nothing.** What he does with days is what the engine does for him: men on the defence dig in, by the ground figures (decision 0106), and eat.
- **An estimate is of one intent of the side tried,** with the plan order recorded at the end of the first day played (decision 0176, amended, 1 and 5).

## The rules

1. **A window is two or three estimates of one course,** each at its own date, in the same worlds: the same view, beliefs, figures, seed, trial's number and count of worlds.
2. **A date is a day of the world.** The course tried at a date is the course with each command's day moved on by the same number of days, so that its first command is handed in on the morning of the date. The days before it are played as things stand.
3. **The first date** is the first day of the trial: the course as it would go now.
4. **The later dates** are taken from the side's view when the window is asked: the days, after the first and within the trial's horizon, on which something the course's intent may use first becomes usable. They are: the day a task force, a ship, an air group or a land unit of the side may first be used; the day aircraft are due to fly in to a group; the day a force is due back from a yard; the day a supply run of the side's is due to arrive. Where the course gives its intent a forces limit, only what that limit names counts. The earliest two such days are the second and third dates; where there is one, the window has two dates; where there is none, it has one, and says that nothing of the side's is due to change.
5. **Each date is played for the days of its size after it:** a date `n` days after the first is played `n` days longer, so that each estimate has the same number of days from the morning its course was handed in.
6. **The plan order of a date** is the one recorded at the end of the day its course's first command was handed in (this replaces, for a date after the first, "the first day played" of decision 0176, amended, 1).
7. **What a later date buys** is said from the side's own things: each thing of rule 4 that came due between the first date and it, by name and day; and the plan order's men wanted and to hand, and its day expected ashore, beside the first date's.
8. **What the enemy did with the same days** is said from the worlds: for the places of the intent's end, at the end of the day before each date, the enemy's men ashore there and their cover, the higher of their fieldworks and the base's defences, each as the lowest, the middle and the highest over the worlds. It is the trial's own reckoning of an enemy it made (decision 0176, 9), and is marked so.
9. **How it went** is each date's estimate whole (decision 0176), and beside them, world for world, for each later date against the first: the worlds in which the end was met and kept at both, at the first alone, at the later alone, and at neither.
10. **A window says nothing of which date to choose.** It gives no chance and no advice (decision 0176, 15).
11. **A window is as much work as its dates.** Three dates at a size are three trials of that size and the days between. It is asked for by its size like a trial, and whoever asks is told the dates before the worlds are played.

## What it leaves for later

- **Dates from things the view does not give a day for:** a unit's training or rehearsal complete, and a tasked search due to report (decision 0161, 16). The state holds their levels and no day; a staff's projection of the morning a landing is ready is the staff's own. They are for the step that gives a seat the tool, if a day can be had for them honestly.
- **An enemy who uses the days.** Given no intent he digs and eats. With a believed intent his staff acts, as far as his made forces allow; the enemy's approaches (decision 0161, step 10) are where he is given something to do with time.
- **The same date prepared early and prepared late** (the spec's controlled comparison): decision 0161's proof, condition 9.

## Rule card

- **Reads:** a side's view, for the dates; what a trial reads; the worlds a trial played, through their two sides' views, as an estimate does.
- **Writes:** nothing in any game.
- **Reuses:** the trial of decision 0175 with its commands on their days; the estimate of decision 0176.
- **New:** in `sim-trial`, the choosing of dates and the window.

## What it costs

- **No live game changes:** no state, no draw, no hash.
- **The rulebook's chapter on the trial** and the glossary gain the window and its date.

## The hand calculation

It is in `tests/scenario_96_the_window.rs`, written before the code: decision 0161's case. A take that can go now with one unit of 600 men or on day 4 with two, against 200 men who dig in while the side waits. Two dates. At the first the place falls on day 4; at the second on day 8, with 600 more men and the enemy's cover at 30 on its eve where it was 0; and it costs the same 100 men at both. A second case, added on review, has worlds that differ: where 200 more men may be there, the first date fails in the worlds that have them and the second takes the place in all four.

## Amended on review, Nov 19, 2026

The plan was read by a reviewer who had not seen it written, against decision 0161, the spec, decisions 0175 and 0176 as amended, and the code. What follows is the architect's and replaces the rules above where they differ. The hand calculation is amended with it.

1. **A later date is "decide later", not "prepare now and go later".** Nothing of the course is done before its date: the units it would take lie idle in the days between, and a landing decided on day 4 is ashore later than one prepared from day 1 and held. Decision 0161, 16, asks for the days between to be played "with preparation". The staff cannot yet prepare a landing and add to it a unit that comes later: a take chooses its units once. So a window overstates what waiting costs in days, and each later date says so in its own words. Preparing early for a later date is left with the proof's comparison of preparation (condition 9) and the what-ifs of the next step.
2. **The shift** (replaces rule 2). A date's shift is the date less the first date. Each command of the course is handed in that many days later than at the first date. A day written inside a command, a latest day or a "not before", is kept as written. What the side has already handed in for the morning, which whoever asks puts at the head of a course (decision 0175, amended, 3), is given apart as standing commands and is not shifted.
3. **Where the dates come from** (replaces rule 4): the days, after the first and no later than the last day of the first date's play, on which something the course may use first becomes usable, each on the morning the staff can first use it:
   - a task force, a ship, a land unit or an air group of the side: the day it may first be used;
   - a task force in a yard: the day it is back.

   Things due on one day make one date, and it names them all. What counts as "what the course may use" is what the take itself would count: where the course gives its intent a forces limit, the units and forces that limit names, and of a kind it does not name, every one of the side's; with no limit, every fighting unit and transport force of the side. The limit is read from the course's own commands. Aircraft due to fly in and supply runs due are left for later: they are usable only from the morning after their day, and which of them a course may use is not settled.
4. **Dates may be given.** Whoever asks may name the dates in place of those of item 3, up to three. A what-if that varies the date will.
5. **The days before a date are played once.** In each world the days from the trial's first to the eve of the last date are played as things stand, once, and each date's play goes on from the world as it stood on its eve. The result is the one separate trials would give, world for world and hash for hash: a world's draws are keyed on its own day, and nothing of the course is in them. So a window of three dates is three trials' days and the days between, not three trials each begun again.
6. **Its cost.** At the full size, three dates 3 and 5 days apart are 64 x (21 x 3 + 8) = 4,544 days of play against a trial's 1,344: about 54 s on the measured times of decision 0175 where a trial takes 16 s; at standard about 10 s against 2.9 s; at small about 3 s against 0.83 s. The designer accepted the times of a trial and is told these.
7. **What the play records for a window,** beyond decisions 0175 and 0176, for the side tried in each world: at the end of the day a course's first command is taken in, the side's plan orders and the units and forces of each landing it has formed; and from the world as it stands on each date's eve, read from its enemy's side, the men ashore at each place of the intent's end and their cover. The eve of the first date is the picture's own evening.
8. **Men to hand at a date** (replaces rule 7's reading) are the men of the units the take chose on the day its course was taken in, read from the landing it formed. A plan order gives men wanted and to hand only where the side holds a count of the enemy; where it does, both are given too.
9. **What the enemy did** (rule 8, made plain): for each place of the intent's end, one line: his men ashore there on the eve, and the cover of the unit of his there with the least cover, the higher of its fieldworks and, where he has a base there, the base's defences; each as lowest, middle and highest over that date's worlds.
10. **World for world** (rule 9, made plain): a world that could not be played at either date is counted apart and is in none of the four counts. Met and lost again counts as not met and kept. The two dates end on different days, each at the end of its own play. The comparison is one function of two estimates' records of their worlds, for any two courses in the same worlds.
11. **A window keeps for each date** its day, the day its course was taken in, what came due on it by name and kind, the men its take chose, its day expected ashore where a plan order gives one, and its estimate.

