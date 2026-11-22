# 0215: The commander is told of what arrives

Status: a plan by the architect, written on Nov 22, 2026, on the designer's question at the table: "When reinforcement units arrive, does the staff take charge of them and add them to existing plans, or should the game stop the clock or notify the player, so that the player can decide what to do with them?" The architect proposed that the staff go on taking them, that the commander be told, and that he may ask to be stopped; the designer said: "Sounds like a sound plan. Go for it." It is to be read by a reviewer who has not seen it written before anything is built.

Summary: Today a force, a land unit or aircraft that arrive are taken into the staff's plans from their day without a word to the commander. This keeps that, tells him in the day's report what arrived, where, and what the staff has done with it, and adds a stop he may switch on for the evening after something arrives. He overrules the staff with the tools he has.

Standing: Planned on Nov 22, 2026; to be reviewed.

## Why

What is so today (decisions 0041, 0193 and 0195; rules chapters 1 and 8):

- **A unit held apart comes onto the map on its day,** and the engine records it: a force entered, a unit entered, aircraft arrived. Its own side sees these events.
- **The staff counts it as coming before its day and uses it from its day.** An order whose forces are of the staff's choosing may use it at the next dusk; an order whose forces the commander named does not touch it. A reason says "U enters on day N" before, and nothing marks the day itself.
- **A unit on the map from the start but not to be used before a day** (`available_from`) is the same to the staff and records nothing on its day.
- **No notice of the staff's speaks of an arrival, and no interrupt stops on one.** The nearest thing is the stop for a ship lost, which fires at dusk on an event of the day before.
- **The table's report of a day** is made from the side's events. Whether it shows the three events of arriving, and in what words, is for the build to find and mend.

Why not have the staff wait for the commander: placing each arriving squadron by hand is what the staff is for, and the 39 days alone have more than a dozen arrivals. Why tell him: an arrival is when he may want to change the plan.

## The rules

1. **The staff takes what arrives as it does today.** Nothing of its doctrine changes.
2. **The day's report says what arrived.** For each force entered, unit entered and aircraft arrived of the side's own in the day, and for each unit whose day to be used came with no entry, one line: what it is, by name, and where.
3. **The line says what the staff has done with it,** read at the next dusk from what the staff hands in and from the orders in force, and from nothing else:
   - the order it now serves, where an order of the staff's or the commander's names it or the staff's hand-in gives it an order under one: "put to order N", with the order as the table words it;
   - that no order uses it, where none does;
   - that it is in the commander's hands, where he holds it or has given it a direct order.
   The table works this from the tools it has (the events of the day, the staff's orders, the command state). No new notice of the staff's is added.
4. **A standing interrupt is new: something of the side's arrived the day before.** It is tested at dusk as the stop for a ship lost is, on the three events and on a unit's day to be used having come. It is off unless the commander switches it on. Its subject is the side, and it fires once for a day however many arrived.
5. **The commander overrules the staff as now:** a direct order, a hold ("keep this force out of the staff's plans"), a forces limit on an order, or a new order.

## What is recorded and who knows

6. **One standing kind is new,** its code added at the end. The command state holds it as it holds the others; the layout rises only if a field is new. **No event and no notice is new.** A side sees its own arrivals, as today, and nothing of the enemy's.
7. **The agent's interface rises** for the interrupt's name in `set_interrupt` and the command state. The doctrine's version does not rise: the staff does nothing new.

## Rule card

Reads: the side's events of the day before (force entered, unit entered, aircraft arrived), each unit's day to be used, the standing interrupts. Writes: that the interrupt fired. Draws: none.

## Games on file

- **No game moves.** The interrupt is off by default and an interrupt that fires changes nothing a side that the engine plays does. A recorded game is remade only for the interface's version. The records of closed proofs are not altered.

## What it does not do

- **It does not have the staff wait for leave before using what arrives.**
- **It does not let the commander say beforehand where an arriving unit is to go,** but by an order that names it, which he may give before its day.
- **It does not replace losses or bring a unit up to strength.** Replacements are not in the game.
- **It does not tell him of the enemy's arrivals.**

## What it costs

- **One interrupt; the report's lines; nothing in the staff.**
- **Rulebook:** chapter 8 (the interrupts); chapter 1 where the stops are listed. The glossary's "Standing interrupt".
- **A scenario of its own and its hand calculation,** written by the architect before any code is delegated: the interrupt on and off, a force that enters, a unit whose day comes, aircraft that arrive, two arrivals in one day.

## Questions for the designer

None. He chose the stop to be one he switches on.
