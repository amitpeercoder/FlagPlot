# 0157: The directive: what a commander is told from above, and where the points go

Status: a direction by the designer, recorded by the architect on Nov 18, 2026, with a first draft shown in the table. Looking at the table, the designer read the briefing a seat gets today, the full return written for a model, and said what a person in the chair should get in its place. Nothing in the engine or the seat's tools is changed.

Summary: The direction for what a human commander reads in place of the briefing: a directive from the commander above, which says what is going on (with what the commander could not know), the task and its purpose, what depends on it and what may be risked, and which is replaced by a new one when the war changes; the order of battle, what is coming and how the result is counted are kept elsewhere in the interface, and points are not shown to a person, though they are still counted to judge who is winning. Two directives for the 39 days of Guadalcanal are drafted and shown in the table

Standing: Direction given by the designer, Nov 18, 2026. Drafted and shown in the table as a prototype; nothing of it is in a scenario or the engine yet

## What is so today

`get_briefing` gives a seat one document: the heading, the mission and each objective with its points, the day's orders, contacts, action and losses, every force, air group, base and land unit, the arrivals and the warnings. It was written for a model, which reads all of it every day. On the first evening of the 39 days it runs to some five thousand words, most of them lists.

## The direction

In the designer's words: the game's briefing is "something that the commander gets from the level above them", which "could change from time to time" and should hold "data that the commander doesn't have and what's expected of the commander"; the "reinforcement table and oob can be found elsewhere in the interface", and "same goes for how points work", since "in a real war one doesn't get real points on sinking a ship (although we will collect points and use them to assess who's winning)".

So, for a person in the chair:

1. **A directive, from the commander above.** It has these parts, in this order: the situation, with what the commander above believes and the commander could not know; the task; why, which is what to go by when the plan stops fitting; what depends on the commander; what may be risked; and when the next will come.
2. **It changes when the war does.** A new directive arrives when a task is done, when the commander above learns something, or when the task can no longer be done. It replaces the last; the earlier ones are kept to be read.
3. **The lists are elsewhere.** The forces, the bases, the men and what is coming are read from the chart and from a ledger, not from the directive. The staff's full return stays one step away.
4. **Points are not shown to a person.** What matters, and how much, is said in words: in the task, in the purpose and above all in what may be risked. The points are still counted, for judging who is winning, for the proofs and for a model, and the end of a game gives its assessment in words first with the count behind it.
5. **The staff's morning report is a separate thing,** short and in a staff officer's voice: what changed, what needs the commander, what the staff means to do. It is not planned here.

## What follows from it, to be planned

- **The words must match the count.** A directive that says the field matters most while the points reward something else misleads the player. Until a scenario holds its directives, they are written by hand from its objectives and kept in step by hand.
- **Where a directive lives.** A scenario has one mission sentence and a list of objectives. Directives that arrive on a date or on an event, for each side, are a thing a scenario would hold and a seat's tool would give. That is to be planned with the onboarding scenarios.
- **What the commander above believes** is belief, not truth: it can be wrong or old. Where a directive states something of the enemy, a scenario's author writes what that commander would have believed. Later it can come from the reports that already travel down from a superior (decisions 0134 to 0138).
- **How the commander is doing,** in place of a score: a word from above from time to time. Not drafted yet.
- **No real document is quoted or imitated as the record.** The directives are the game's, in its own words, and say so.

## As drafted in the table

`table/directives/guadalcanal_allied.json` holds two directives for the Allied seat of the 39 days, from the Commander in Chief, Pacific Fleet, to the Commander, South Pacific: the first on the first evening ("Take Tulagi and Lunga"), the second when Lunga is the side's own ("Hold Lunga and get the field flying"). The bridge is given the file (`--directives`), and the page shows a directive when its condition first holds, read from the seat's own tools: at the start, or when a named base is the side's. A new one opens by itself and stays on the desk. The staff's full return is still there, under its own name. The page's address ending `#directives=all` shows every directive as if received, for reading the drafts.

What the two say matters most was written from the scenario's objectives: the field at Lunga, held and open (20 points of the count), above any one ship (a carrier is 10); the carriers risked only against carriers; the cargo landed and the enemy's men lost both counting. Their dates are the scenario's: the aircraft for Lunga are due on its day 14, 20 August.
