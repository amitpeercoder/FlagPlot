# 0237: The first three rungs of the ladder

Status: planned by the architect on Nov 27, 2026, on the designer's four answers of that day, for an independent review and then his confirmation. Nothing is built. Needs review. It plans the first three levels of `docs/design/levels.md` as plain scenarios on the table's list; the note is amended with it ("The first three rungs", there).

Summary: The ladder begins with three small invented games on one invented sea, each a plain scenario on the table's list with a mission, judged by the game played and by nothing else: one ship keeping one garrison fed for 14 days, by the floor and the interval of one supply order; two garrisons and two unlike ships, by which ship is named to which order; and 600 men put on an empty place and fed after, over 21 days, by the risk of the landing and the supply order that can be given only once the place is his. No rule, state, event or draw is added. New: one placeholder map, three scenario files with their tests, and on the table a chart of a hand-written map, the mission on the first screen and a line at the end that says whether it was met.

Standing: Planned by the architect on Nov 27, 2026; needs review: waits for an independent review and the designer's confirmation; class 3; nothing built

## Why

The designer wants the ladder to start tiny and the 39 days to be its top rung. The first level `levels.md` planned is two landings on shared shipping, with an enemy, three situations, a mission held from above and a debrief over a thousand seeds: weeks of work before anyone learns whether a small game with one decision is worth playing. His answers of Nov 27, 2026 cut that:

1. **Rung 1 is by intent.** "He names the ship, the source, the floor and the interval; the staff sails her."
2. **Rung 1 may be a tutorial with one right answer,** with no enemy: a decision only by scarcity and time. The ship cannot carry two weeks in one go, and the garrison is about five days from hunger on day 1.
3. **Rungs 1 to 3 ship as plain scenarios on the table's list.** His words: "It can be plain. But it doesn't need to be judged against the staff's, only against the play itself."
4. **Where a rung has troops, they seize an empty place,** which is built, and do not reinforce a held base. The cut of rungs 2 and 3 is left to the architect.

So a rung of these three is a scenario and a mission, and the measure is whether the mission was met in the game he played. Everything `levels.md` lists as new for a level (the mission held, the check that a plan is his own, the history, the situations, the debrief) is not needed for them, and is not built for them.

## What was weighed

- **Judge a rung by men lost unfed,** as the architect first proposed. Not done: on the live figures of hunger a man is lost only on a unit's fifteenth hungry night (`ground_hunger.toml: hunger_nights_before_loss` 14), so in 14 days nobody can be lost and the measure could not fail. The measure is a night unfed, which the seat is told every morning. A ground file with a shorter count, made for the ladder, was not chosen: it would teach a tempo the later rungs do not have.
- **Merge rungs 2 and 3.** Not done. Each rung adds one decision to the one before: how much and how often; which ship to which mouth; a landing and then its supply. Merged, a newcomer meets the forces limit, the risk, a landing and a supply order that must wait for it in one game, and a failure does not say which he got wrong.
- **Rung 2 with troops.** Not done: answer 4 puts the troops where a place is seized, and that is a rung of its own. Rung 2 moves no man.
- **Rung 3 as "control the island".** Not done: the staff then forms the take, the hold and the supply itself (scenario 76), and nothing is left for him to decide. A bare "take a place" leaves the feeding to the commander, which is the rung's point.
- **Rung 3 in 14 days, on figures that ready a landing in a day or two** (scenario 76's `preparation_per_day` 100). Not done, for the reason the hunger count was not shortened. On the live figures a landing at balanced is ready on day 21 and lands on day 22; at press it lands on day 12. So rung 3 is 21 days, and the risk is part of its decision (question 3).
- **A map in the generated form** (`.csv`, with a latitude and a longitude for each hex), which the table draws today. Not done: an invented sea would be drawn at a real place on the earth, with its degrees along the edge. The map is hand-written, as every placeholder map is, and the table learns to draw one.
- **Offer the dry run on these rungs.** Not done (question 2): of the trial's five costs none shows a night unfed short of the fifteenth, so it would report nothing lost of a plan that fails the mission.
- **Points for the mission.** Not done: none of the ten kinds of objective counts a side's own men fed, and a new kind is a rule. The files state no objectives; the verdict is read from what the seat is told.
- **Build all three at once.** Not done: rung 1 is built and played by the designer first (`levels.md`, suggestion 6, kept).

## Rule card

No rule of the game is added, changed or removed, and nothing is added to the rulebook or the glossary.

| | |
| --- | --- |
| State read | What the rules as built read: the bases' stocks of supply and fuel, the ships, the land units' men, fed share and nights of hunger, the command state (intents, limits, leaves, plans, runs, commitments) |
| State written | None new. No field, so the hash's layout and version stand |
| Events, and who sees them | None new |
| Draws | None. Rungs 1 and 2 have no enemy and no die. Rung 3's place has no man on it, so no fight is rolled |
| Versions | None moved: the doctrine, the hash and the save stand |
| Rules the rungs rest on | Chapter 8, The doctrine: keep a base supplied (the projection, a new run, its load, its days, its fuel, when it cannot); chapter 2, cargo, fuel burn and refuelling in port; chapter 7, The men's supply (Feeding, Nights of hunger), a landing and the rations it lands, a base that changes side; chapter 8, Taking a place (readying a landing, the risk) |
| Games moved | None on file. Three scenarios are added, with their pins |

## The sea

One placeholder map serves the three rungs, so that rung 1's garrison is rung 2's near one and the player learns one small sea.

`data/map/placeholder_ladder.toml`, hand-written, marked `placeholder = true`: open sea with four coast hexes. As tried: the source at (0,0), port size 1; the near place at (5,0), 200 nm from it; the far place at (0,14), 560 nm; and the empty place at (3,3), 240 nm; the last three with port size 0, worked by lighter. The geography is invented and has no place on the earth. Every base, ship and unit is named "Placeholder".

The table cannot draw it today: the bridge reads a map's hexes from the generated `.csv` form, with degrees for each hex, and the page places everything by degrees (`table/serve.py`, `read_map`; `table/index.html`, `world`). See "On the table".

## The three rungs

Each figure below is as tried by the architect on Nov 27, 2026, in runs of the engine on `origin/main` with the files the rungs would name ("What the trial runs showed"). A trial run is not a hand calculation: the figures are fixed by the hand calculation in each test's doc comment, written before any code, and may move there. Each file names the live figures, with `data/reference/ground_hunger.toml` for the ground as the table's other games do, names no trial figures and states no objectives.

### Rung 1: one ship

- **The forces.** The source holds 500 t of supply and 50 t of fuel. The near place holds 25 t and 1,000 men, who eat 5 t a day: five days. One placeholder fast transport with cargo (class 12: 30 t of cargo, 16 kn) lies in port at the source with 60 t of fuel. A round trip burns 50 t, so there is fuel for two. No enemy. 14 days.
- **The mission, as he reads it.** "Keep the garrison fed. Every man is to be fed every night to the end of day 14. It holds five days of food. Your one ship carries six days of it at a time, and the fuel on hand is enough for two round trips."
- **The decision he owns.** One order to keep the place supplied: the ship and the source named, the floor in days and the interval in days. Leave to prepare and to launch are his to give, as in every game.
- **What the staff does.** The doctrine as built: it projects the stock, sizes each run from the floor and the interval, loads, sails, unloads, comes home and refuels the ship, and says each morning what the next run waits for. Where a run cannot go for want of fuel it says so and the clock stops on the notice.
- **Why it is a decision.** A run's load is the stock under the floor on the day it arrives plus the interval's days of use, cut to the ship. A small floor with a short interval sends her part-empty, and two part-empty runs do not feed 14 nights. A floor of nothing sends no run at all. So there is a band of right answers and not one (question 4): a floor of 1 or 2 days with an interval of 5 or 6, or a floor of 10 days with an interval of 1 or 6, feeds every night; 2 and 4 misses the last night; 1 and 1 misses six.
- **Judged.** Mission met where no night's return shows a man unfed at the place, nights 1 to 14.

### Rung 2: two mouths, two ships

- **The forces.** Rung 1's near garrison as it was, and a second at the far place: 400 men, 2 t a day, with 6 t, three days. Two ships at the source: a placeholder cargo ship (class 3: 3,000 t, 10 kn), which is ship 1, and the fast transport of rung 1 (30 t, 16 kn), ship 2. The source has supply and fuel to spare. No enemy. 14 days.
- **The mission.** "Keep both garrisons fed, every man every night to the end of day 14. The near one holds five days of food and the far one three. The freighter carries everything and is slow; the fast transport is quick and small."
- **The decision he owns.** Which ship is named to which order. Two supply orders, each with its forces limit; the floor and the interval are practice of rung 1.
- **What the staff does.** As rung 1, for two orders. Given both ships for both orders it takes them in number order, so the freighter goes where the first order served sends her. Tried with a floor of 3 days and an interval of 14 on both orders.
- **Why it is a decision.** Only the fast transport reaches the far place before its fourth night, and she carries enough for it (30 t is 15 days there). The freighter reaches the near place in time with all it needs. The other way round, or with both ships named to both orders, the far garrison goes unfed on the night of day 4: the freighter unloads there hours after that night's meal.
- **Judged.** As rung 1, at both places.

### Rung 3: men, then food

- **The forces.** At the source: 600 men (3 t a day), a transport force of three placeholder fast transports that carry men and no cargo (class 11, 200 men each), and one fast transport with cargo (class 12, 30 t: ten days for 600 men). The empty place is a base of the enemy's with no man, gun, aircraft or stock on it, on the side's map. The enemy has nothing else. 21 days.
- **The mission.** "Seize the place, reported empty, and hold it with its men fed: it is to be ours, and no man ashore there is to go a night unfed, to the end of day 21. Men land with three nights of rations."
- **The decision he owns.** The landing and what follows it: an order to take the place, with its force and its unit named and its risk set; and, once the place is his, the supply order for it, which the game refuses while the base is the enemy's.
- **What the staff does.** It readies the landing, loads the men, sails, lands them and takes the place; the landing puts three nights of rations ashore (chapter 7). It forms no supply for a bare "take a place". Given the supply order it runs it as in rung 1.
- **Why it is a decision.** At balanced the landing is ready on day 21 and the men land on day 22, after the mission's last day. At press they land on the morning of day 12 and the place is his that day. The night's meal is the first thing of a day, so the rations feed the nights of days 13 to 15, and a run must land on day 15. A supply order given on day 13 or 14 does that; one given on day 15 lands on the morning of day 16, after that night's meal, and the men go unfed that night.
- **Judged.** Mission met where the place is the side's at the end of day 21 and no night's return shows a man of his unfed there.

## How a rung is judged

By the game played, once it is over, from what the seat was told and nothing else:

- **A night unfed** is a row of the morning briefing's return of the men fed (chapter 7, Feeding: a side sees `MenFed` for its own men, whole): the hex, the men, the fed and the unfed. The table keeps each morning's briefing already, for the casualty return. A night counts against the mission where the unfed of a named hex is above 0.
- **A place held** is the seat's own list of its bases at the end.
- **No umpire's data.** No replay, save, batch, event log or result from the truth is read. The measure is exact all the same, because a side knows its own men and bases exactly.
- **What he is shown.** "Mission met", or "Mission not met" with the nights and the men: "1,000 men at Placeholder base T went unfed on the night of day 14." Nothing is scored and nothing is compared with the staff's plan.
- **Nothing gates anything.** The three rungs and every other game on the list are open from the start. The check that a plan is his own, the history and the replay on a new situation of `levels.md` begin with the first rung that has an enemy (question 5).
- **He steers.** The clock stops where the staff says it cannot meet the order, and he may change the order in the game. That is play, and the verdict is of the whole game.

## On the table

- **The list.** Three entries at the head of `table/scenarios.json`, which is listed simplest first: "Rung 1: One ship" and so on, each marked a placeholder in its lines, with the Allied side for a person and the other side the engine's, which has nothing and does nothing. The entry's lines and its side's mission are the challenge: the new game screen shows them as it shows every scenario's. No pairing for an agent is offered until the designer has played rung 1.
- **The mission in the game.** A directive for the seat (`table/directives/`, decision 0157), in the game's own words, the mission above as its second part.
- **No date.** An invented game has no calendar. The entry gives no start, and the page counts days. If the page cannot do without one, the build stops and asks.
- **The chart.** The bridge reads a hand-written `.toml` map, and gives each hex a place to be drawn from its q and r; the page draws it with no degrees at the edge and names a place by its base's name or its hex. The web build takes its charts through the same function.
- **The verdict.** Each rung's entry names its measure, as data: the hexes whose nights count, the last day, and for rung 3 the base to be held. At the end of the game the page reads the briefings it has kept and the seat's bases, and shows the line above where the result is shown.

## What is built, what is new, and its class

Built and used as it is: the supply order with its floor, interval, forces and source; leaves; the doctrine and its reasons; refuelling in port; the men's supply and its return; "take a place" with its risk; rations landed; the staff's notices and the clock's stops; the table's list, new game screen, directives and briefings.

| New | Class | Why that class |
| --- | --- | --- |
| This record and the amendment of `levels.md` | 3 | The plan of three scenarios for play; the designer's to confirm |
| `data/map/placeholder_ladder.toml` | 3 | Data, with the first scenario that names it |
| `data/scenarios/ladder/138_one_ship.toml`, `139_two_mouths.toml`, `140_men_then_food.toml`, each with its test and its pins | 3 | A new scenario for play. The numbers are the next free today and are checked again when each is built |
| The table draws a hand-written map | 2 | What the table gives, with the rules as they are. The architect is shown the chart of the ladder's map, and that the charts of the two generated maps are byte for byte what they were |
| The rungs on the list, their directives, and the verdict | 2 | The same. The measure is this record's and not the build's: the architect writes the verdict's statements, the words and the cases, in an `as-built/` section before code, and is shown the screen check for a game met and a game not met on each rung |

The folder `data/scenarios/ladder/` keeps the files out of the loops that walk `data/scenarios/*.toml` (decision 0072); each is named by its own test and by the table's list.

## The tests, and what each hand calculation must work

The statements and the hand calculation are in each test's doc comment, written by the architect before code is delegated, and nowhere else (decision 0233, item 3). **Ruled:** each test runs the rung's own file on the figures it names, the live ones, because what it proves is the game the player is given; the frozen step 1 figures have no figures of hunger, and a test on them would prove another game. So a later tuning of a figure these files read (the hunger, port, fuel or preparation figures, the three placeholder classes) moves these tests, and comes back to the architect for them (decision 0233, item 11). The independent review is asked to weigh this against the rule that hand calculations run on the frozen figures, which was written for the battles of step 2.

- **`tests/scenario_138_one_ship.rs`.** For the reference answer, a floor of 2 days and an interval of 6: each morning's reading and projection; the day each run is ordered, its load, the days of loading, passage and unloading; the hour it unloads; the fuel aboard and at the source after each run; the stock and the men fed each night. Then the same for three answers that fail, each for a different reason: a floor of nothing (no run), 1 and 1 (part-empty runs, the fuel gone), 2 and 4 (the last night). And the two edges of the band, 2 and 5, and 10 and 1.
- **`tests/scenario_139_two_mouths.rs`.** For each of three plans (the right ships, the ships exchanged, both ships named to both orders): which ship the staff takes for which order and why, each run's days, the hour each unloads, and each place's stock and men fed each night.
- **`tests/scenario_140_men_then_food.rs`.** At press and at balanced: the loading and rehearsal each day, the day the landing is ready, the day it sails and lands, the rations landed, the day the base changes side. Then, at press, the supply order given on days 13, 14 and 15: the run's days and load, and the men fed each night to day 21. And the refusal of a supply order given before the place is his.

## What the trial runs showed

Run by the architect on Nov 27, 2026, with `sim-cli run` on `origin/main` at `761b9e3`, on files in a scratch folder that are not committed. They show the shapes hold; they prove no number.

| Rung | Tried | Nights unfed |
| --- | --- | --- |
| 1 | Floor and interval 2 and 6; 1 and 6; 2 and 5; 1 and 5; 10 and 1; 10 and 6 | None |
| 1 | 2 and 4 | 1 (day 14) |
| 1 | 3 and 3 | 2 (days 13, 14) |
| 1 | 2 and 2; 5 and 1 | 3 |
| 1 | 1 and 1 | 6 |
| 1 | 0 and 6 | 9 (no run is sent) |
| 2 | Freighter to the near place, fast transport to the far | None |
| 2 | The ships exchanged; both named to both orders | 1, at the far place (day 4) |
| 3 | Press; the supply order on day 13, or 14 | None; the men land on day 12 |
| 3 | Press; the supply order on day 15 | 1 (day 16) |
| 3 | Balanced | The men land on day 22, after the last day |

In no run was a man lost: with the live figures of hunger none can be in 21 days.

## The order of pull requests

After decision 0232's step 3, which goes on as planned and is what rung 4 will need:

1. **This record and the amendment of `levels.md`,** then its independent review, the amendment, and the designer's word.
2. **Rung 1's game** (class 3): the map, `138_one_ship.toml`, its test with the hand calculation, its pins.
3. **Rung 1 on the table** (class 2): the chart of a hand-written map, the entry, the directive, the verdict. It touches `table/` and may be built beside 2.
4. **The designer plays rung 1.** His account decides whether 5 and 6 go on as planned.
5. **Rung 2** (class 3): the file, its test, its entry and directive.
6. **Rung 3** (class 3): the same.

## What it leaves out

- **The mission held from above, the check that a plan is his own, the history, the situations and the debrief** of `levels.md`. They wait for the first rung with an enemy.
- **Rung 4,** two landings with an enemy, on decision 0232's screen. A record of its own.
- **A dry run that can show a night unfed,** and a kind of objective for a side's own men fed. Each would be a rule.
- **An agent at a rung's seat,** and a rung played from the other side.
- **A calendar, a history note and points** for an invented game.
- **The order of `levels.md`'s table of skills** beyond its first rows: it is settled when rung 4 is planned.

## Questions for the designer

1. **The measure is a night unfed, not a man lost.** On the live figures nobody dies of hunger inside two weeks, so "no man lost" could not fail. The architect recommends "every man fed every night". The other course is a game of 30 days or more, in which men are lost from the fifteenth hungry night.
2. **No dry run on rungs 1 to 3.** It would say nothing was lost of a plan that fails the mission. The architect recommends none: what he reads before the clock runs is the staff's own reading of his order, and the first dry run is rung 4's. The other course is a record that adds nights unfed to what a trial reports, built first.
3. **Rung 3 is 21 days, and its landing must go at press.** At balanced the live figures ready a landing in 20 days. So rung 3 asks two things of him, the risk and then the food. The architect recommends that: the risk costs nothing on an empty place, and saying so is the lesson. The other course starts the men aboard their ships, ready as they stand: 14 days, no risk to set, and the food alone.
4. **Rung 1 has a band of right answers, not one.** Any order that sends the ship nearly full twice feeds the garrison. The architect recommends leaving it so: the lesson is the reason, and a single pair of numbers would be a password.
5. **Nothing gates the three rungs.** They are plain scenarios, open from the start beside the battles, and "not trying blocks the next" begins at rung 4. The architect recommends that, as what answer 3 means.
