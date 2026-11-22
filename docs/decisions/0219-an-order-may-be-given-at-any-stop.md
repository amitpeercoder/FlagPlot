# 0219: An order may be given at any stop, and acts when it can

Status: a ruling by the architect on the designer's word at the table, Nov 22, 2026. A direct order he signed at 1200 was refused as one "given at dusk", and he said: "I think a command can be given at any time, but may be digested at certain times. A commander doesn't wait for end of day to issue orders (or to any other time)."

Summary: At the stops inside a day, 0600 and 1200, a seat may give only seven kinds of order, and any other is refused with the words that it is given at dusk. Commands to the staff given there are already taken and held for the next dusk. This does the same for an order to a unit: an order that cannot act inside the day is taken, kept, and handed in at the next dusk as if given then, and its answer says when it will act. Nothing changes in when any order acts.

Standing: Ruled on Nov 22, 2026; built on Nov 22, 2026 (see "As built"), for the architect to read.

## Why

- **What is so** (decision 0156, rules 3 and 6; chapter 1, "Orders inside a day"): seven kinds of order act inside a day (a force's route, speed, readiness and rules of engagement; an air group's mission and search arc; a land unit's posture). Any other handed in at 0600 or 1200 is refused. A command to the staff given at those stops "is queued and is taken in at the next dusk, and its answer says so."
- **The designer's word:** a commander gives an order when he thinks of it; when it takes hold is the game's affair.

## The rules

1. **At a stop inside a day a seat may give any order it may give at dusk.**
2. **An order of the seven kinds acts as decision 0156 has it,** at the start of the pulse it was given for.
3. **Any other order is kept for the next dusk** and handed in there with the seat's orders of that dusk, in the order given, as if given then. It is checked at once against the state as it stands, and refused at once where it would be refused whatever the hour (a unit not the side's, a place off the map); it is checked again when it is handed in, as every order of dusk is, and may be refused then for what has changed.
4. **Its answer says when it acts:** that it is kept and will be given at 1800 for the coming day.
5. **A kept order may be cleared** before that dusk, as the orders queued for a dusk are.
6. **A kept order does not hold its unit from the staff until it is handed in.** The override holds a unit when an order is taken in (decision 0141, item 14), and that is at dusk, before the staff is asked.
7. **The engine is not changed:** what it takes in at 0600 and 1200 is still the seven kinds, and what waits is kept by the seat's session and is not in the state or its hash. A scenario's script and a staff give their orders as they do.

## What a side learns by it

Nothing new of the enemy. A seat is told of its own kept orders, with the hour they will act.

## Rule card

Reads: the seat's orders given at a stop inside a day and their kinds. Writes: the seat's orders queued for the next dusk. Draws: none.

## Games on file

None moves. A recorded game in which a seat was refused such an order inside a day would differ; the build says whether any is on file.

## Statements for the tests

Written by the architect; a test that disagrees comes back to him.

1. In a scenario with orders inside a day, at 1200 a seat gives a task force a mission order (not one of the seven). The answer is not a refusal: it says the order is kept and acts at 1800 for the coming day. Nothing changes in the afternoon pulse: the force's mission at the day's end is what it was.
2. At the next dusk the order is handed in with that dusk's orders and accepted, and the force has the mission for the coming day. The game from there is the game in which the same order was given at that dusk and not before.
3. At 1200 the seat gives a route to the same force (one of the seven): it acts at the start of the afternoon, as before this ruling.
4. An order kept at 0600 and one kept at 1200 are handed in at dusk in the order given.
5. An order kept at 1200 for a unit that is lost in the afternoon is refused at dusk, for the reason an order to a lost unit is refused.
6. An order for a unit of the other side given at 1200 is refused at once.
7. Clearing the orders queued for dusk at 1200 removes the kept order, and nothing is handed in for it.
8. In a scenario without orders inside a day nothing changes.

## What it costs

- **The tool's answer and its queue; the words of chapter 1,** where "Any other order ... is refused" becomes rules 3 to 5 for a seat, the engine's own refusal standing for an order that reaches it; the table's forms, which offer every order at every stop and say when each acts.
- **The agent's interface rises.**

## As built, Nov 22, 2026

- **The check of the record** against the code found its "Why" and rules true. One line of chapter 1 said commands to the staff are not given inside a day; the code queues them for the next dusk, as the record says, and the chapter now says so.
- **The tool** (`sim-agent/src/session.rs`, `issue_orders` and `judge`): at 0600 or 1200 an order of the seven kinds is judged against the pulse's queue and queued for the next pulse, as before; any other is judged by the engine's `check_order_queued` against the seat's queue for dusk and this call's kept orders, and where it passes is put in that queue, which the day's dusk hands in with the seat's other orders, in the order given. Its result is `queued` with `kept: true` and `acts` ("kept: it is given at 1800 for the coming day, day N"); the answer gains `kept_in_call` and `kept`, and its summary says how many were kept for 1800. `clear` at a stop inside a day clears both queues. The saved game records a kept order as an order of that dusk, so a replay is the game in which it was given then. A watched game saved inside a day keeps neither queue, as it kept none before.
- **The words** of the tool's instructions for a game with orders inside a day (`sim-cli/src/mcp.rs`) say that any order may be given and when each acts. The engine's refusal "given at dusk" stays, for an order that reaches it, as a scenario's script's may.
- **Tests:** `crates/sim-agent/tests/orders_kept_for_dusk.rs`, statements 1 to 8 on scenario 78 with its own order inside a day removed, and `01_transit` for statement 8. Statement 2 compares a game in which the order was kept at 1200 with one in which it was given at that dusk: the daily hashes, the hand-in and the events from that dusk on are the same. **Statement 5 is proved on a stand-in:** no scenario with orders inside a day loses a whole task force in an afternoon, so the test keeps a stand-down at 0600 and a mission order at 1200 for the same force, and the second is refused at dusk because the force no longer exists. That an order to a force sunk in the afternoon is refused at dusk, for the reason an order to a lost unit is refused, is not shown on a sinking. The test `the_refusals_inside_a_day` of `advance.rs`, which held that such an order is refused, now holds that it is kept.
- **Games on file:** no recorded game has an order refused inside a day as given at dusk, so none moves for this.
- **The table:** every form offers every order at every stop; an order not of the seven kinds says "Acts at 1800, for the coming day." in the form before signing, in the status line and among what has been signed. The page keeps that list itself, and a reload forgets it, though the session still holds the orders: no tool lists a seat's queued orders.
- **Versions:** the agent interface 45, once for the batch with decisions 0215 and 0217.
