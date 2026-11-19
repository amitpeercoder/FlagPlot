# 0171: A side's own half of a picture: the first part of step 4 of stage 4

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 4 of its ten, section C, rules 6, 7 and 9), which the designer confirmed. Step 4 is written in three records: this one, a side's own half; the next, what a scenario says a side believed and which side uses a class; and the third, the enemy's half and the crate `sim-trial`. The hand calculation is written. To be built when decision 0170's build has merged.

Summary: A picture of a world is made of two halves. A side's own half is what its view gives it whole: its ships, forces, bases, air groups and men, its orders in force, its command state, what it believes of the enemy, and its counters. This record adds the half as plain data, the one call that copies it from a side's view, and the call that joins two halves into a picture. The copying is done in `sim-core`, where no field of a unit can be forgotten and the numbers a picture may not carry are taken out. Two worlds that differ only in what a side cannot see give it the same half, byte for byte

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161. The hand calculation is written; the build waits for decision 0170's

## Why

Decision 0161, rule 7: "The side's own half is copied from its view." Decision 0170 built the picture and its constructor and left the copying for this step, with one thing settled by its build (rulings, 4): a picture made outside `sim-core` cannot name the event a halted force or a force on a yard trip cites, so the copying is done inside it.

What a view gives, read from the code:

- **A side's own units whole:** its ships, task forces, bases, air groups, land units, formations, replacements and arrivals, each as the engine's own state holds it.
- **Its orders,** every one it has given, accepted or not, each with the day it was issued and who gave it; and for each a number of the side's own, counted from 1.
- **Its command state,** whole, and its belief: contacts, claims and estimates.
- **Its next force number.** Not its next air group number: the view has no reader for it.
- **Numbers it may hold and not read.** The units cite orders and events by the game's own numbers, which count both sides' orders and every event of the game. A side's units carry them; nothing a side is given lets it read one. Copied as they are into a picture they would tell how many orders the enemy has given.
- **A view cut to a seat** (decision 0137) gives a blank command state and no force number. Only the view of a whole side has a whole half.

## The rules

1. **A half is plain data,** a type of `sim-core` with public fields. It holds, for one side on one evening: the day; the side; its ships, task forces, bases, air groups, land units, formations, replacements and arrivals; its orders in force, as a list, each referred to by its place in the list from 1; its command state; its belief as a picture holds one (decision 0170, 1); and its two counters. It holds no number of a live game's order or event.
2. **A side's view gives its own half** by one call, on an evening. It is refused while a day is open, and at a view cut to a seat.
3. **The orders in force** are the orders the side's units and its command state refer to, and no other, in the order of the side's own numbers for them. Every reference is re-pointed to the order's place in the list.
4. **Every event a unit cites is the picture's.** The arrival of a force, what alerted it, its halt and its yard trip are set to the first event of a world built from a picture, as the umpire's whole picture sets them (decision 0170, 8 and 13).
5. **The belief** is the side's contacts in the order of the side's own numbers for them, its claims in the order held, and its estimates in the order of their hexes, each without the event that reported it.
6. **The counters** are the side's next number of a force made in play and of an air group made in play. The view gains a reader of the second; a side knows its own air groups, and the number says nothing of the enemy.
7. **Two halves make a picture.** One call joins the half of each side, of the same day, into a picture of a world: the units of both, the orders of the Allied half and then of the Japanese, each re-pointed to its new place, the belief, the command state and the counters of each. It refuses two halves of one side, halves of different days, and two units of one number.
8. **A half need not be copied.** Its fields are public so that the enemy's half can be made by rule (the third record of this step). A half made outside `sim-core` names its orders by their place, and can hold no halted force and no force on a yard trip: it has no event to cite.
9. **No peek.** A side's own half reads the side's view and nothing else. Two worlds that differ only in what the side cannot see and has not been told give it equal halves.

## What it leaves for later

- **The enemy's half,** the believed forces and intents, and which side uses a class: the next two records.
- **Objectives.** A side's view has its own mission and objectives; a picture's are the scenario's for both sides. A world built from two halves has none until a trial needs them, and a trial does not score (decision 0161, 15).
- **Command levels** in a half (decision 0170, 4).

## Rule card

- **Reads:** a side's view of a whole side, on an evening.
- **Writes:** nothing. It returns data.
- **Reuses:** the picture of decision 0170 and its walk over every number a unit holds; the side's own numbers for its orders and contacts.
- **New:** in `sim-core`, a half, the call on a view that gives one, the call that joins two, and a reader of a side's next air group number.

## What it costs

- **No live game changes:** nothing is added to the state, no draw moves, no hash.
- **The rulebook:** chapter 1's "How a world begins" gains the half and the joining; its passage on what a side knows gains the next air group number.
- **The agent's library** may not name the half or the call: its source test gains the words.

## The hand calculation

It is in `tests/scenario_92_a_sides_own_half.rs`, written before the code:

- **By hand:** the Allied half of scenario 1 on the evening of day 1, field by field.
- **Two halves are the whole:** the halves of both sides of a live game, joined, are the umpire's whole picture of it but for the order of the orders; and a world built from them plays on as the live game does. On scenario 86, and on the full slice from the evening of day 3, where both sides have given orders and fought.
- **No peek:** scenario 68's pair of worlds that differ only in what the Allied side cannot see give it equal halves.
- **What it refuses.**
