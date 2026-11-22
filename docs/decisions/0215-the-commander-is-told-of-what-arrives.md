# 0215: The commander is told of what arrives

Status: a plan by the architect, written on Nov 22, 2026, on the designer's question at the table: "When reinforcement units arrive, does the staff take charge of them and add them to existing plans, or should the game stop the clock or notify the player, so that the player can decide what to do with them?" The architect proposed that the staff go on taking them, that the commander be told, and that he may ask to be stopped; the designer said: "Sounds like a sound plan. Go for it." It is to be read by a reviewer who has not seen it written before anything is built.

Summary: Today a force, a land unit or aircraft that arrive are taken into the staff's plans from their day without a word to the commander. This keeps that, tells him in the day's report what arrived, where, and what the staff has done with it, and adds a stop he may switch on for the evening after something arrives. He overrules the staff with the tools he has.

Standing: Planned on Nov 22, 2026; reviewed and amended the same day; built on Nov 22, 2026 (see "As built"), for the architect to read.

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

## Amended on review, Nov 22, 2026

Read by a reviewer who had not seen it written. What follows replaces rules 2 to 4 and 6 to 7 above and what rests on them.

What the review found:

- **A standing interrupt for the side would have missed most arrivals.** An interrupt fires while armed and is armed again only on a morning its condition does not hold; the 39 days have arrivals on eight days running.
- **It would have stopped a day or two late,** after the staff had used the unit: interrupts are tested in the morning after the staff is asked, and a seat is stopped after the day is played.
- **"Off unless switched on" has no place in the state:** every standing kind is on until switched off, and seeding it off would move every game's hash.
- **The table cannot work out what the staff did with a unit.** A reason for giving no order names no unit, and an order that uses a unit without ordering it that day (an air order with no forces limit, a later lift in a pool) would read as "no order uses it".
- **The table cannot see that a unit's day to be used has come:** the tools give that day only while it has not.
- **Today's report** says nothing of a force or unit that entered but its raw event at the day's end, and nothing in the evening's report of aircraft that arrived.

The plan as it now stands:

1. **The staff takes what arrives as it does today.** No rule of the engine changes, and nothing in the simulation core.
2. **The evening's report says what came today and what comes tomorrow,** made where the briefing is made and not at the table, so that an agent at a seat is told the same:
   - **Came today:** each task force and land unit that entered, each air group that received aircraft, and each unit whose day to be used was today, by name, with where it is. A unit that both entered and may be used today has one line. Day 1 has none: what is there at the start is not a reinforcement.
   - **Comes tomorrow:** each that enters, arrives or may first be used tomorrow, so that the commander may hold it or give it an order before the staff does. A hold may already name a unit held apart (decision 0195, rule 14).
3. **Each line of what came says only what is explicit:** that the staff has given it an order today, with the order it serves; that an order's forces limit names it; that it is held or under a direct order of the commander's; or, where none of these, that the staff has given it no order today. Aircraft that join a group say the group and its mission. It never says that no order uses a unit.
4. **The table's own stops gain one:** "when reinforcements arrive", in its panel of what stops the clock, off until the player ticks it. With it on, time run from the table stops at the evening whose report has a line of what came or what comes tomorrow. It is kept by the table as its other stops of that panel are, and is nothing in the engine.
5. **An agent that runs several days at a time has no such stop.** It reads the briefing's lines. An engine interrupt for it is not built.
6. **The day's plot shows an entry in words,** "Task Force 18 arrived at Espiritu Santo, 6 ships", at the start of its day and not at the end.

What is recorded: nothing new. The agent's interface rises, for the briefing's two lists. The state, the hash, the doctrine and the rules label are as they are, and no game on file moves; recorded games are remade for the interface's version, and the records of closed proofs are not altered.

Rule card: none; no rule of the simulation changes. Rulebook: none but chapter 1's account of the briefing, where it lists what a briefing holds.

Tests are the briefing's, in the agent's crate, on a scenario that has each kind of arrival: a force that enters, a unit that enters, aircraft that arrive, a unit whose day comes with no entry, two on one day, and none on day 1. There is no hand calculation: nothing is computed.

## As built, Nov 22, 2026

- **The briefing** (`sim-agent/src/arrivals_told.rs`) gains, in its `arrivals` part, `came_today` and `comes_tomorrow`, each left out when empty. A line has the unit's `kind`, the unit, `how` (one or more of `enter`, `aircraft` and `usable`), where it is (`place`, and `at`, a base or a ship), what it is (a force's `ships` by kind, its `air_groups` and `land_units` aboard, a unit's `men`, a group's `aircraft` and `aircraft_received`, its `mission` and `target`), and, in `came_today`, `staff`, exactly one of `ordered` (the intent and its words), `named_in_limit` (the intent and its words), `held`, `commander_ordered` and `no_order_today`. The text briefing says them as "Came today: ..." and "Comes tomorrow: ...".
- **What the staff did** is read from the side's own orders (`SideView::orders`, each with its issuer and the intent it serves) and the day's `OrderAccepted` events: the staff's order accepted for the unit in the day just played; then the commander's; then a hold of the whole unit in the command state; then an intent in force whose own forces limit names it; else no order that day. A limit inherited down a tree is not read, and a hold on one setting of a group is not counted as a hold.
- **One line a unit.** A unit that enters and may be used the same day has one line. What a force carries is in the force's line and has no line of its own, but for a group aboard that receives aircraft with no entry of its own. A unit that entered and is gone by the evening has no line.
- **Day 1 has none** in either list, and so the briefing before day 1 has no `comes_tomorrow` and the arrivals of day 1 are delivered but not told.
- **A unit held apart** may be used from the later of its entry day and its own day; `comes_tomorrow` reads the unit's own day from the entry, since the view gives the later one.
- **The table:** the evening report has two parts, "Reinforcements came today" and "Reinforcements due tomorrow", one line each by name ("12 aircraft joined VMF-223 at Henderson Field, on patrol"), with the staff's part in the page's words. The panel of what stops the clock is offered in every game now and gains "reinforcements arriving", off until ticked: with it ticked, time run from the table stops at the evening whose briefing has a line in either list. In a game played a day at a time the panel shows this stop alone. The day's plot shows an entry under the night of its day, in words ("Carrier force K arrived at ..., 3 ships"), not at the day's end.
- **Tests:** `crates/sim-agent/tests/told_of_arrivals.rs`, on scenario 72 made to have a force that enters and is first usable on one day, a land unit that enters, a land unit whose day comes with no entry, aircraft due on days 1, 3 and 5 (three arrivals on day 3), and on scenario 21 with a force entering with two land units aboard; the staff's parts, the text, the enemy's briefing, and every briefing against its schema. Not tested: a group aboard that receives aircraft with no entry of its own.
- **Versions:** the agent interface 45, once for the batch. No state, hash, doctrine or rules label changes.
