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

## Amended on review, Nov 22, 2026

Read by a reviewer who had not seen it written. What follows is the plan as it now stands and replaces the rules, the figures and the table's part above.

### What the review found

- **A force on the move rarely keeps its contact's number from one day to the next.** A report refreshes a contact only within `merge_distance_hexes`, 2, of one no older than `fresh_contact_max_age_pulses`, 2; a hex is 40 miles, and a force at 15 to 20 knots makes 9 to 12 hexes in a day. A bases' search at balanced risk flies once a day. So each day's sighting is most often a new contact, while the old one is held, stale, for a day and a half more. The plan as written would have served waters round the stale one, refused the new one for having been made before the old was dropped, and ended "not found" with the force on the chart.
- **The staff never withdraws a deny's tasks and re-forms none.** The tasks to strike ships and to search waters name a base and read the waters each dusk, so they need no forming again; a stand task stands at the centre and would chase a point the force left in the night; a free carrier force's station is chosen once and kept.
- **Decision 0214's leave is a force's, not a contact's,** and lifting the bound for an order is its setting "go", which exists.
- **At balanced risk a carrier force closes on a carrier and on nothing else,** so an order to destroy a force taken for cruisers would have moved no carrier. A base under a deny's task strikes what its target list reaches, with no preference for the force named.
- **A contact's kind is wrong on some three reports in ten and is replaced by each,** so a notice on every change would have stopped the clock about once a day for each order.
- **With command levels on, a command's picture numbers its own contacts and records no event of making or dropping one.** A world made for a dry run numbers its contacts anew.
- **Shadow as written was a deny with the bombers left out.**

### The order

1. **One root order is new: destroy a force.** It names a contact of the commander's picture and a distance in whole hexes, the commander's choice as a deny's is. It is given to the area and stands until it ends or is withdrawn. It may be given as search only, which is the shadow of the first plan.
2. **It is a deny whose centre is read from the force it follows, each time the staff is asked.** All that serves a deny serves it, with these differences:
   - its waters are to be struck first, by the mechanism of decision 0190, for the bases and the carrier forces that serve it: a base's bombers take a contact in them before another, and a carrier force closes on a contact in them of any kind, at balanced risk and at press;
   - its leave to pursue is "go" unless the commander sets another;
   - no stand task is formed under it. A surface force is not sent after a force on the move;
   - a free carrier force's station is chosen again at a dusk at which the waters' centre lies more than `station_rechosen_hexes` from where it was chosen;
   - a task at a base that has not reached the waters at two dusks running is withdrawn by the staff, and formed again when the base reaches them.
3. **Search only** forms the tasks to search waters and no other, sets no waters to be struck first and gives no leave. Its searches fly in both daylight pulses at any risk.
4. **Where nothing can serve it the order says so,** as a deny does, by the notice a deny uses.

### Following

5. **The order keeps, for the force it follows: the contact's number in the commander's picture, the hex and the day and pulse of its last report, and whether it was taken for a carrier.** These are kept when the order is given and at each change of rule 7.
6. **While the followed contact is fresh, the order follows it:** the centre is its estimated hex.
7. **At a dusk at which it is not fresh, the order takes another contact** of the commander's picture, where one qualifies:
   - its last report is later than the followed contact's;
   - its hex is within the distance a force at `followed_force_knots` steams in the hours since the followed contact's last report, counted in hexes from that report's hex;
   - no other order to destroy follows it.
   Of those that qualify it takes one taken for a carrier where the followed was, and one not where it was not, before another; then the nearest; then the lowest number. Orders are worked by priority, then by number.
8. **A taking is recorded and told on the order's paper,** and is no stop of the clock, but where the contact taken is of the other sort (a carrier for a force that was not, or the other way): then the staff's notice says so and may stop the clock.
9. **Where none qualifies the order is lost-contact:** its centre stays the last report's hex and it is served by the tasks to search waters alone. The staff hands in a notice, once.
10. **It ends at the dusk `lost_contact_days` days after the last report** with nothing taken, by a notice that the force was not found. It never ends because the force is believed sunk.
11. **A contact's kind changing is not a notice.** The paper shows what the force is taken for.

### What the order cannot do, said plainly

- **The staff aims at where the force was last reported.** It is asked at dusk; by morning a force has gone six hexes or more. The waters to strike first may not hold it then, and the groups strike what they reach. A wider distance holds more of where it may be.
- **By night nothing serves it.** A force that comes and goes in the dark is not met by this order.
- **It adds most against a force that is found each day:** the searches turn to it, a free carrier force is stationed for it and closes on it of any kind, and the bombers take it first.

### Pictures, seats and dry runs

12. **The number is the commander's picture's.** With command levels on that is the area command's; a seat at a task force names a contact of its own picture, matched to the area's by hex and kind as decision 0214 does. A contact is later than another by its last report, which a picture holds; nothing is read from events.
13. **Given inside a day, the order keeps what rule 5 keeps at once,** and acts at 1800 (decision 0219): if the contact is no longer fresh then, rule 7 is worked from what was kept.
14. **In a dry run the followed contact is found again by its place in the order of the picture's contacts,** which a made world keeps.

### What is recorded

- **One kind of order, its code at the end,** with the distance and whether it is search only. What rule 5 keeps is state of the side's command, by order, not part of the order's kind, so that following is no amendment of a plan. It is written by the engine when it records the staff's notices of a taking and of a loss. All is hashed and named; the layout rises.
- **Three notices, their codes at the end:** follows another contact; contact lost; ended, not found. A refusal is new: the contact is not held.
- **Figures, placeholders and judgements, in the staff's figures:** `followed_force_knots` 20; `lost_contact_days` 2; `station_rechosen_hexes` 3.
- **The stops of the table are keys of the tool that runs time,** so that an agent has them: a taking of the other sort, lost, ended.
- **The doctrine's version, the rules label and the agent's interface rise.** The frozen copies do not gain the order.

### Rule card

Reads: the commander's picture's contacts, each one's hex, kind, age and last report; the order's kept state; the figures; all that the deny doctrine, the targets and the carrier doctrine read, with the order's waters taken from the followed force. Writes: the kept state, by the engine on a notice; sub-orders formed and withdrawn; notices. Draws: none.

### At the table

- **A contact's card offers DESTROY THIS FORCE,** with the distance, 2 or 4 hexes round, and SHADOW IT for search only.
- **The order's paper says what it follows now,** by the report: where, how many, taken for what, how old; and each taking in the order it happened.
- **The chart draws the waters round the followed force,** dashed where the contact is lost.

### Before it is built

A count is made on the games for play, of sightings of one force that refresh a contact against those that make a new one, day by day. It is a check of the review's first finding and of `followed_force_knots`, and tunes nothing.

### The scenario's statements

A force at 20 knots seen on two mornings, the second a new contact while the first is held, and the order follows it. Two forces in reach, a carrier and not, and the sort decides. Two orders and one new contact. A contact dropped by a strike that found nothing. A kind that changes and stops nothing. A carrier force that closes on a force taken for cruisers at balanced. A base that falls out of reach and its task withdrawn. A free carrier force's station chosen again. Lost, then ended. Search only forming no strike. Command levels on with the area's picture late, and a seat at a task force. A dry run of a side with the order. The frozen figures untouched.
