# 0171: A side's own half of a picture: the first part of step 4 of stage 4

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 4 of its ten, section C, rules 6, 7 and 9), which the designer confirmed. Step 4 is written in three records: this one, a side's own half; the next, what a scenario says a side believed and which side uses a class; and the third, the enemy's half and the crate `sim-trial`. The hand calculation is written. Built on Nov 19, 2026 (As built, below).

Summary: A picture of a world is made of two halves. A side's own half is what its view gives it whole: its ships, forces, bases, air groups and men, its orders in force, its command state, what it believes of the enemy, and its counters. This record adds the half as plain data, the one call that copies it from a side's view, and the call that joins two halves into a picture. The copying is done in `sim-core`, where no field of a unit can be forgotten and the numbers a picture may not carry are taken out. Two worlds that differ only in what a side cannot see give it the same half, byte for byte

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161. Built on Nov 19, 2026

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

## As built

Built on Nov 19, 2026, on the branch `own-half-build`, from the head of the record's own branch. The tests are in `tests/scenario_92_a_sides_own_half.rs`, 17 of them. No number or statement of the hand calculation was changed.

**Every statement of the hand calculation holds but one, which is not the engine's to meet.** Case 2 says that day 4 of the full slice is the day of the carrier battle. It is not: in `07_full_slice.toml` the carrier forces meet on days 18 and 19, and days 4 and 5 hold no fight at sea or in the air (day 4: steaming, fuel, marches and men fed; day 5: unloading and refuelling). The statement as asserted, that everything that happens in the built world on days 4 and 5 is the live game's, holds, every event of both days compared by its day, pulse and kind. So that the round trip does go through the carrier battle, the build adds one test the hand calculation does not name: the full slice's two halves on every evening from day 1 to day 38, each joined and rebuilt, played to the end of day 39. On every evening the halves joined are the whole picture as case 2 says, and on every later day the built world has the live game's ships, task forces, bases, air groups and land units, field for field but for the numbers of the orders and events they cite, and every event the live game has, by day, pulse and kind: the carrier battle of days 18 and 19, the raids on bases, the ships sunk on days 19 to 21 and 25, the ground fights of days 37 and 39. No rule was found that reads a history the built world does not have.

**What decision 0170 owes.** The full slice round trip is the test of a world with days behind it that decision 0170's ruling 7 asks for. The four rules that read the log of earlier days are still not reached: the full slice has no refused order, no yard trip, no intent and so no interrupt or note. A test of each across a picture stays owed.

**What changed where.**

- `sim-core`, a new module `world_picture/half.rs`: the half, `PictureHalf`; `SideView::own_half`, which reads the view's public readers and nothing else (it takes no world), builds a state of the side's own units, and goes through `walk_held_ids` twice, once to gather the orders referred to and once to re-point each order to its place and each event to event 1, so a field added to a unit that holds a number is walked here as in the whole picture; `WorldPicture::from_halves`, which re-points the Japanese half's orders by the same walk.
- `world_picture.rs`: the copying of a belief is one function, `belief_of`, read by the umpire's whole picture and by the half alike.
- `view.rs` and `world.rs`: the view holds the side's next air group number, as it holds its next force number, and reads it with `SideView::next_air_group_id`. `view/known_view.rs`: a crate-private reader of whether the view's units are as a command holds them.
- The source tests: `crates/sim-agent/tests/leaks.rs` and `crates/sim-staff/tests/source.rs` each ban `own_half`, `from_halves` and `PictureHalf` (the staff's already bans `World`, and so `WorldPicture`).
- The rulebook: chapter 1 gains "A side's own half of a picture" after "A world built from a picture", and a row in "What a side may know" for the next numbers of its own units made in play. The glossary gains the half; `docs/engineering/layout.md` says the staff's source test bans it.

**The public surface added to `sim-core`.**

1. `world_picture::PictureHalf`, all fields public (item 1).
2. `SideView::own_half(&self) -> Result<PictureHalf, SimError>` (items 2 to 6).
3. `SideView::next_air_group_id(&self) -> AirGroupId` (item 6).
4. `WorldPicture::from_halves(one, other) -> Result<WorldPicture, SimError>` (item 7).

Nothing of the state, the hash, a draw or an event changed.

**Readings the record did not make.**

1. *The names.* The type is `PictureHalf`, the call on the view `own_half`, the join `WorldPicture::from_halves`, and the reader `next_air_group_id`, beside `next_task_force_id`.
2. *A half's units are kept by kind,* each in its own map as the state keeps them, and its replacement streams and arrivals as lists; it holds no hits on men, which an evening never holds.
3. *A view whose units are as a command holds them* (`SideView::known_to` for the area command while reports travel) is refused as a view cut to a seat is: its units are not the side's own state. The words are the same: "only a whole side has a half".
4. *The refusals' words.* Every refusal is `SimError::InvalidSetup`. A half while a day is open says "a picture is of an evening, and day N is open", as the whole picture does. The join's say "the halves cannot be joined:" and then "two halves of one side", "halves of different days" or "two units of one number", naming the side, the days or the unit.
5. *A fourth refusal of the join:* a half that refers to an order not in its own list. Joined, such a reference would name an order of the other half, which `World::from_picture` could not see; refusing it keeps each half's numbers its own.
6. *The order of replacements and arrivals.* The join puts both sides' replacement streams and arrivals in the one order the state keeps them in, the loader's (decision 0043, 4c), not the Allied half's and then the Japanese. The full slice's arrivals interleave the two sides by day, and the state's order is part of the hash.
7. *Not checked by the join, as item 7 does not ask:* that a unit's side is its half's, and the days an order was issued. `World::from_picture` checks what it checks.
8. *An order a unit refers to that is not the side's own* would make the half refuse ("refers to an order that is not the side's own"). No game reaches it: every order a unit holds was given by its side or for it by a rule.
9. *A seat-cut view's next air group number* is 0, as its next force number is.

## The architect's rulings

Made on Nov 19, 2026, on reading the build.

1. **The nine readings of "As built" stand as made.**
2. **The hand calculation was wrong in one sentence, and is corrected by the architect.** It said that day 4 of the full slice is the day of the carrier battle. That was true of the slice of one week; in the 39 days the carriers meet on days 18 and 19. The build found it and, so that the fights were covered, rebuilt the full slice on every evening from day 1 to day 38 and played each to day 39. That sweep is now a statement of the hand calculation.
3. **The sweep stands as the test decision 0170 owed of a world with days behind it** (its rulings, 7), for everything the full slice reaches: on no evening and no later day did a world built from two halves part from the live game. The four rules that read the log of earlier days are still not reached by it, and stay owed.
4. **A view of units as a command holds them is refused a half** (reading 3). Such a view is not the side's own state but what was last reported of it.
5. **The join puts replacements and arrivals in the loader's one order** (reading 6), since that order is part of the state; orders stay the Allied side's first and then the Japanese side's.

