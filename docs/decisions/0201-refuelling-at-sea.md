# 0201: Refuelling at sea

Status: a plan by the architect, written on Nov 21, 2026, for the designer's queue of rules (the second of nine). It takes up what decisions 0045, 0046 and 0112 deferred. To be read by a reviewer who has not seen it written, and amended on that reading, before its hand calculation is written and before any of it is built.

Summary: A ship takes fuel only in a port of her side. A force with no port in reach goes home or runs dry, and the oilers that kept the carriers at sea in 1942 are left out of every scenario because they could do nothing. This gives a ship that can fuel others at sea a mark on her class, lets ships stopped beside her take the fuel she carries as cargo, at a rate and a few at a time, and has the staff count such a ship as a place to fuel.

Standing: Planned on Nov 21, 2026; waits for its review.

## Why

What is so today:

- **Fuel is taken in port and nowhere else.** A force that is stopped at the centre of a hex holding a base of its side fuels with no order, in each pulse's loading work, each ship taking the least of what she lacks, what the base holds above its reserve, and the rate times the hours left after her cargo work (decisions 0045 and 0112). The rulebook says "there is no refuelling at sea (no oiler, no transfer between ships)".
- **A ship can carry fuel as cargo.** It is a separate figure from the fuel she steams on, loaded and landed at a base by the orders for cargo; "only landing it at a base makes it usable". Any ship with room for cargo can carry it.
- **There is no oiler.** A ship's kind is carrier, battleship, cruiser, destroyer, transport or auxiliary. The Coral Sea's Neosho and Midway's Cimarron and Platte are left out of their scenarios because "the game refuels only in port, so Neosho fuels nobody at sea" (decision 0188, 47).
- **Two forces of a side can lie in one hex,** and nothing passes between them at sea.
- **The staff sends a force to a base for fuel:** a force on a station with no home port is sent to the nearest base that can fuel it when a ship is short (decision 0192), and a carrier force goes home. A force with no home port and no base in reach has its fuel weighed against nothing; two agent commanders ran destroyers nearly dry at the Coral Sea with no warning.
- **The rows:** 36 notes of fuelling in `data/oob/ship_facts.csv`, among them an oiler that topped off a carrier and eight destroyers in a day (1831) and one that passed 480 tons to a cruiser at sea between 0640 and 0830 (1854). The oilers' cargo of oil is on file only from a publication of 1945. No row gives a pumping rate of 1942 or how many ships lay alongside at once: both are gaps.

## The rules

### The ship that gives

1. **A ship's class may say that she fuels others at sea.** It is a mark on the class, beside its room for cargo, not a new kind of ship: an oiler is an auxiliary with the mark.
2. **What she gives is the fuel she carries as cargo.** The fuel she steams on is her own and is never given. She loads cargo fuel at a base by the orders for cargo, as any ship does.
3. **A ship without the mark gives nothing at sea,** whatever she carries. A carrier or cruiser fuelling her destroyers is not in this rule.

### Fuelling

4. **It happens with no order, in the pulse's loading work, after fuelling in port.** A ship fuels at sea when:
   - her force is stopped at the centre of a hex, as for fuelling in port, and did not run dry in this pulse;
   - a ship with the mark, of her side, is in the same hex in a force that is also stopped at its centre, and holds cargo fuel;
   - no base of her side in the hex fuelled her this pulse. A port is used before an oiler.
   A ship of the oiler's own force fuels from her as any other does. The oiler does not fuel herself.
5. **It is a daylight task.** No ship fuels at sea in the night pulse.
6. **An oiler serves a few ships at a time.** Each oiler has a number of places alongside, a figure of the game. Ships are taken in a fixed order: a ship out of fuel first, then the ship with the smallest share of her capacity aboard, then by id. A ship not taken this pulse waits for the next.
7. **Each ship takes the least of** what she lacks, the cargo fuel the oiler still holds, and the rate at sea times the pulse's hours left after her own force's cargo work. The rate at sea is a figure of the game, for each place alongside.
8. **Where two oilers are in the hex,** the one with the lower id is used first, and a ship takes from one oiler in a pulse.
9. **A force out of fuel is set going again** by the first ship fuelled, as in port.
10. **Both forces are stopped while it is done.** That is the cost: a stopped force is where it is for the whole pulse, to be found and struck. Nothing else changes for them: air groups fly as from any stopped force. The slow steaming of a true fuelling is not modelled.

### What is recorded and who knows

11. **An event for each ship fuelled,** naming the ship, the oiler and the tons, with a breakdown of what she lacked, what the oiler held, the hours and the rate. Its cause is the stop of each force. One more event, once, when ships wait because the places are full.
12. **Each side sees its own fuelling whole and nothing of the enemy's.**
13. **No new state.** The fuel given leaves the oiler's cargo and joins the ship's own; both figures exist and are hashed today. The mark and the two figures are of the reference data.

### The staff

14. **An oiler that stands is a place to fuel.** Wherever the staff today looks for "the nearest base that can fuel" a force (decision 0192's station from the sea), it also counts a force of its side that is stopped, holds an oiler with cargo fuel, and is not itself under way to somewhere else. It sends the force to the nearer of the two and stops it there; when every ship has the fuel its task asks, it goes back.
15. **A carrier force with no home port is weighed too.** It is sent to fuel, by rule 14, when a ship is short of the passage to the nearest place to fuel times the margin of its risk. With no such place its fuel is still weighed against nothing, and the staff says so in its report each day a ship is below a quarter of her capacity: the warning the agent games asked for.
16. **The staff places an oiler only where an order says.** An order to stand at a point (decision 0192) given to a force with an oiler puts her there; the staff does not choose a fuelling point by itself in this decision, and does not send her to reload.
17. **A player's orders are enough without the staff:** route two forces to one hex and stop them.

## Figures

18. **The oilers' cargo of oil** is read from the rows of 1945 by the exception of decision 0200, a row of a later date: the ships' tanks were what they were built with. Those rows are to be entered as fact rows first; today they are in a research note.
19. **The rate at sea and the places alongside are placeholders,** marked so, until rows of 1942 are found. They are sized from two rows: 480 tons in an hour and fifty minutes to one cruiser (1854), and a carrier and eight destroyers topped off by one oiler in a day (1831). The hand calculation states them; a second pass of research for a pumping rate and for the ships alongside is commissioned with the build.
20. **A quarter of capacity,** the staff's warning of rule 15, is a figure of the doctrine.

## Rule card

Reads: a class's mark, room for cargo and fuel capacity; a ship's fuel and cargo fuel; a force's state and place; the pulse; the two new figures. Writes: a ship's fuel, an oiler's cargo fuel, a force's state when it is set going. Draws no random number.

## What it does not do

- No fuelling under way, no speed limit while fuelling, no weather.
- No fuel from a warship to her escorts.
- No aviation fuel or other stores passed at sea.
- No doctrine that sends an oiler to reload or chooses where she waits.
- No oiler is added to a scenario of record by this decision. The Coral Sea and Midway are looked at again when it is built: with oilers, the Japanese search has the decoy it found in May 1942.

## What it costs

- **One more thing a stopped force does,** in the loading work that is already the longest step of a pulse to explain.
- **Two placeholder figures** where the sources of 1942 are thin.
- **Rulebook:** chapter 2 (fuelling, and the two sentences that say there is none at sea), chapter 8 (the staff's places to fuel and its warning), the glossary ("oiler", "fuelling at sea", "places alongside"), and the correction of chapter 2's sentence that a route order sets a force out of fuel going, which the code and the glossary deny.
- **Versions:** the rules label rises if any existing game plays differently; none should, since no class on file has the mark. The doctrine's version rises with rule 15, which can move a carrier force in an existing game: that is checked on the scenarios of record and said.

## Questions for the designer

1. **Stopped, not slow.** The plan has both forces stopped for the pulse they fuel in, which is simple and costs them a pulse of movement. The truer picture, steaming at ten knots on a steady course, needs forces to keep station on each other at sea, which is not built. The architect proposes stopped, for now.
2. **Daylight only.** Night fuelling was done but was rare in 1942. The architect proposes daylight only.

## Amended on review

A reviewer who had not seen the plan written read it against the code and the rows on Nov 21, 2026. Where this and the text above differ, this holds.

**No closed proof moves (replaces rule 15).**

21. **The warning is all that applies where no oiler stands.** A force with no home port, on a station or a carrier force, whose ship is below a quarter of her class's fuel capacity is named in the staff's report each day. Nothing else changes for it: it is not sent to a base. The carrier forces of the Eastern Solomons and of both proofs have no home port, and sending them in would move measures that are closed.
22. **A force is sent to fuel at sea only to an oiler that stands** (rule 14), and only when a ship is short of the passage to her times the margin of its risk. With no oiler on the map no existing game plays differently, and that is checked on the scenarios of record.

**How it is built into the pulse (replaces rules 4, 5, 7 and 8 where they differ).**

23. **Fuelling at sea is a pass of its own,** once in the loading work, after every force has done its cargo, its fuelling in port and its ammunition. The forces at sea that the loading work passes over today are taken into it.
24. **It asks whether the hex is dark,** as every rule of light does; it does not read the pulse's name. A force halted at a mark that steams on in daylight is not stopped by day and so never fuels; a force that has arrived or is idle is.
25. **An oiler that did cargo work in the pulse gives nothing in it.** She loads in one pulse and gives in another.
26. **A ship fuelled by a base in the pulse takes nothing from an oiler in it,** even where the base's rate left her short. That is on purpose: a port and an oiler in one hex are not added together.
27. **A ship with the mark takes nothing at sea.** Oilers do not fuel each other.
28. **The hours** are those of the pulse left after the later of the two forces' stops, less the receiving ship's own cargo hours.

**Places alongside (replaces rule 6's "places").**

29. **An oiler has a number of pump lines, and each serves ships in turn.** In a pulse each line has the hours of rule 28; a ship uses the hours her tons take at the rate, of one line, and the next ship takes what is left of it. Ships of every force in the hex are taken in one order, that of rule 6. With two oilers a ship goes to the oiler of lower id that has hours left, and takes from one oiler in a pulse.
30. **The figures are two, the same for every oiler,** kept with the fuelling rates of ports: the rate of a line at sea and the lines of an oiler. A scenario whose figures lack them has no fuelling at sea. They are placeholders, marked so. The rate is sized from the rows of fuel passed at sea with their hours, which run from 118 to 370 tons an hour and agree at about 265 for the two best timed (1854 and 1855). The lines are two, from the one statement on file, of 1945, that two large ships could be fuelled at once in good conditions; the rows of 1942 show one ship after another. Row 1831 has no hours and sizes nothing.

**A force adrift (adds to rule 9).**

31. **A force that ran dry between two hexes is not fuelled by this rule.** It is adrift and not at a hex's centre, as in port. It is the commonest way to run dry at sea, so the rule tops ships off before they are dry and does not rescue them. Said as a limit.

**The events (replaces rule 11).**

32. **A new kind of event for a ship fuelled at sea,** since the event of fuelling in port names a base. Its causes are the events that stopped the two forces, where there are any; a force idle since the game began has none. One event for each oiler in each pulse in which ships waited for her, naming them. Both are seen whole by their side and by no one else, and are placed in the list of who sees what, the briefing, the agent's tools, the export, the viewer and the table.

**The staff (replaces rules 14 and 16 where they differ).**

33. **An oiler stands when her force has arrived or is idle,** at sea, and she holds at least the fuel the force that needs her is short of. The staff reads her cargo fuel and her class's mark from its own side's view.
34. **No home port is set for a force sent to an oiler.** Each morning the staff works it afresh, keeping nothing: a force beside an oiler that still holds fuel stays while a ship of it is short of what its task asks; otherwise it goes back. If the oiler has gone when it arrives, the next morning's weighing sends it on or back.
35. **The staff places no oiler.** She is put where she waits by a route given to her force. An order for where oilers wait is a later decision.

**The oilers of the record (replaces rule 18).**

36. **This decision is built in two stages.** Stage A is the rule, on placeholder classes, with its scenario and hand calculation. Stage B makes real oilers: the derivation gains a ship type for an oiler, a class's room for cargo read from rows (it is written as nothing today), and the mark; and the rows are gathered that the Cimarron class lacks, her own bunkers and her endurance at a speed.
37. **Her cargo of oil is read from a statement of the time:** 12,000 tons each loaded in Platte and Cimarron in July 1942, to be entered as rows from the research note, beside the figures of 1945, which are loadings of single ships and differ between sisters. Decision 0200 is not needed for it.

**Also named for the build:** the mark is a column a class file may leave out, so no class file on file changes; the dropped route when a force is set going again is in the rule card; the agent's instructions say what an oiler is.
