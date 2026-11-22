# 0220: An order against an enemy force

Status: a plan by the architect, written on Nov 22, 2026, on the designer's words at the table, looking at a contact's card: "From an enemy force card, should we add the option to destroy it? I'm not sure about that, but that seems natural. Maybe even the option to avoid it (in the future) or to delay it." The architect proposed destroy and shadow now, avoid later and no delay; the designer said: "Yes, please." It is to be read by a reviewer who has not seen it written before anything is built.

Summary: Every order a commander can give is about a place or a base. None is about an enemy force, so a player who wants a force sunk must turn that into waters himself, and the order goes stale when the enemy moves. This adds two orders that name a contact: destroy it, and shadow it. The staff turns each into the tasks it already has, round where the contact is believed to be, and moves them as the reports move. The order says plainly when the contact is lost, when another report is taken to be the same force, and when it is taken for another kind of ship.

Standing: Planned on Nov 22, 2026; to be reviewed.

## Why

What is so today (rules chapters 3 and 8; decisions 0012, 0123 to 0125, 0133, 0202 and 0214):

- **A contact is a number, a hex, a count of ships, a kind and an age.** A report within `merge_distance_hexes` of a fresh contact refreshes it, and it keeps its number; the report replaces its hex, count and kind. A contact not refreshed ages and is dropped at `contact_drop_age_pulses`. A strike that finds nothing drops it. A number is never used again: the same force seen later is a new contact.
- **An order to deny waters names a hex and a distance.** Under it the staff forms tasks to strike ships and to search waters from bases, a stand or carrier task for each task force its forces limit names, and, with no task force named, a carrier task for a free carrier force at a station the staff chooses.
- **A carrier force under an order closes on a carrier it has found** within the bounds of the order, and beyond them by the leave to pursue (decision 0214).
- **A contact's card at the table** offers to deny the waters round it and to operate a carrier force from its hex. Both are orders on the hex, and stay there when the contact moves.

## The rules

### The two orders

1. **Two root orders are new, each naming a contact the side holds:** destroy it, and shadow it. Each is given to the area, stands until it ends or is withdrawn, and has the limits and leaves any root order has.
2. **Each has waters:** the hexes within `contact_order_hexes` of the contact's estimated hex. They are read again each time the staff is asked, from the commander's own picture, so they move as the contact is refreshed.
3. **Destroy is served as an order to deny those waters is served,** by the same doctrine and the same tasks, with two differences:
   - the tasks are formed again where the waters have moved, the old ones ending as a task ends whose order changed;
   - a carrier force that serves it has leave to pursue that contact: the bounds of rule 10 of decision 0214 are lifted for it, and no leave is asked. A hard limit binds as there: an area limit, a hold, the forces limit.
4. **Shadow forms the tasks to search waters and no other.** No task to strike, no stand task and no carrier task is formed for it. Nothing stops a group from striking the contact under another order or its own mission.
5. **Where nothing can serve it the order says so, as a deny does:** no base with a bomber in reach, no carrier force free.

### When the contact is refreshed

6. **A report that refreshes the contact moves the order with it.** Nothing is recorded but the tasks formed again.
7. **Where a refresh changes the contact's kind from the kind it had when the order was given,** the staff hands in a notice: the contact is now taken for that kind. It is handed in once for each change. The order stands.

### When the contact is lost

8. **Where the side no longer holds the contact, the order is lost-contact from that dusk.** It keeps the last waters it had, and is served there by the tasks to search waters alone. The staff hands in a notice that the contact is lost and that it searches where it was last reported.
9. **A lost order takes a new contact.** At a dusk at which the side holds a contact that was made after the old one was dropped and whose estimated hex is within `lost_contact_reach_hexes` of the last waters' centre for each day since the loss, the order takes the nearest of them, the lowest number on a tie. Count and kind are not compared, as a report is not compared with a contact. The staff hands in a notice: contact M is taken to be the force of contact N, with its kind and where. The order is then as it was before the loss.
10. **A lost order ends at the dusk `lost_contact_days` days after the loss** with no contact taken. The staff hands in a notice that the order is ended, the force not found. An order that ends is as one withdrawn.
11. **The order never ends because the force is believed sunk.** A side does not know that. A force destroyed is no longer reported, and the order ends by rule 10.
12. **The commander may withdraw the order at any time,** and may refuse a contact taken by rule 9 by withdrawing it.

### Stops

13. **Each of the three notices (now taken for another kind, lost, ended) may stop the clock at the table,** by stops of the table's own in its panel, on unless the player unticks them. No interrupt of the engine is added.

## What a side learns by it

Nothing of the enemy that it did not hold. The waters, the kind and the taking of a new contact are read from the commander's own picture and from nothing else. With command levels on, the picture is the area command's, as in decision 0214.

## What is recorded

- **Two kinds of order, their codes added at the end.** An order's state gains the contact it names, the kind when given and last told, the last waters' centre, and the day it was lost. Each is hashed in its place and named; the layout rises.
- **Three notices, their codes at the end.** A side sees its own.
- **Figures, all placeholders and judgements:** `contact_order_hexes` 2, the distance the table offers today; `lost_contact_days` 2; `lost_contact_reach_hexes` 6 for each day, a force at 20 knots making some 12 hexes of 40 miles in a day and a half of that allowed.
- **The doctrine's version, the rules label and the agent's interface rise.** The tool that gives an order takes the two kinds and a contact's number; it refuses a contact the side does not hold.
- **The frozen copies do not gain the orders.**

## Rule card

Reads: the order's contact, in the commander's picture: its hex, kind and whether it is held; the side's contacts and the events that made them; the order's state; the figures; all that the deny doctrine reads. Writes: the order's state; sub-orders, as the deny doctrine writes them; notices. Draws: none.

## At the table

- **A contact's card offers DESTROY THIS FORCE and SHADOW THIS FORCE** above the orders on its waters.
- **The order's paper names the contact as the card does** ("the force reported at 35°26′N 178°30′E, 21 ships, taken for cruisers") and follows it: the chart draws the order's waters round the contact, and a lost order's waters dashed.
- **The notices are slips from the staff** in the words of rules 7 to 10.
- **An order given inside a day acts at 1800,** as decision 0219 has it.

## Games on file

None moves: no game on file gives either order. Recorded games are remade for the versions; the records of closed proofs are not altered.

## What it does not do

- **Avoid is not in it.** Keeping forces out of an enemy force's reach comes after the staff's regard for submarine waters (decision 0218, rule 55), whose means it shares.
- **Delay is not in it.** The staff cannot tell an order to delay from a weak attack.
- **It does not send a surface force after a contact by itself.** A surface force serves a destroy order as it serves a deny: where the forces limit names it, by a stand task in the waters.
- **It does not name an enemy ship.** A side holds contacts, not ships.
- **It does not judge whether the force is worth the order.** The commander does.

## What it costs

- **Two kinds of order served by the deny doctrine with moving waters; the lost-contact state; three notices; the table's two buttons, paper, chart and slips.**
- **Rulebook:** chapter 8 (a section of its own after Denying an area; the intent tree; the notices; the tools) and the glossary ("Destroy order", "Shadow order", "Lost contact").
- **A scenario of its own and its test statements,** written by the architect before any code is delegated: a contact that moves and is followed; one that changes kind; one lost and found again under a new number; one lost and not found; a new contact too far to be taken; a carrier that closes past the order's bounds with no asking; an area limit that binds it; shadow forming no strike.

## Questions for the designer

None before the review.
