# 0150: Leaving a place: step 8 of stage 3, first part

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 8 of its nine as amended). Step 8 is written in two records: this one, the end "leave a place"; and the next, "control an island". It adds no rule to the engine. Nothing is built. Moved by the designer on Nov 17, 2026, on the architect's review of step 8, to after the proof of stage 3: neither side states it in the 39 days, and it is built before stage 4, which has the staff leave a place of itself.

Summary: Step 8 of stage 3, first part: the end "leave a place", by which the staff flies the air groups out to the base named, sends the transport force for the men, takes the working units off first and the fighting units after, a unit a morning, and brings them back, in as many trips as the berths need, until nothing of the side's is left there that could be taken off

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Not built; moved to after the proof of stage 3

## Why

A commander must be able to give a place up before it is taken from him. The rules say what happens if he does not: a beaten side with nowhere to fall back to surrenders, and "a side that means to take its men off by sea must do it before it is beaten" (chapter 7). Every rule the taking off needs is built: the load order with a unit, the rebase order, the route. No staff uses them so.

## The end

1. **Leave a place** names a base of the side: the place is its hex. It may have a date, the day by the end of which nothing is to be left. It is an end, addressed to the area, and a root.
2. **Its forces limit** names the task forces that may take the men off and a base of the side, the source, which here is where everything goes. A forces limit with no source is refused. It may name land units and air groups: then only those are taken off.
3. **It is met** when no land unit of those it covers has a man ashore in the hex and no air group of those it covers is based at a base of the side there. A unit or a group the commander holds is not covered: the staff plans without it and says so each morning.
4. **The base stays the side's** until the enemy takes it. Its supply, its fuel and its works are left, and the reason says what is left.

## The leaves

5. **Leave to prepare** lets the plan be made ready: nothing is ordered by it. **Nothing flies out or sails without the plan released,** by leave to launch or the commander's approval. While it waits the staff hands in the notice that the plan waits for leave to launch, each morning.

## The air groups

6. **On the morning the plan is released,** and on each morning after while one is still there, each air group covered that is based in the hex is given a rebase order to the source, in group order. A group the order would be refused for is given none, and the reason says why: the source has no airfield, or lies beyond the group's transfer range, or the group has already moved that day. It is left, and it does not stop the intent being met where it cannot be flown out: the reason of the day the plan is marked complete names it.

## The men

7. **The transport force** is the lowest-numbered task force of the forces limit on the transport mission, with a ship afloat that carries men, that the side may use and the commander does not hold.
8. **The order they go in:** working units first, then fighting units, each in number order. The men who can fight are the last ashore.
9. **To the place.** On a morning the force lies stopped at a base of the side that is not in the place's hex, with no man aboard, and a unit covered has men ashore at the place, it is given a route to the place and a load order for the first unit in the order.
10. **At the place.** On each morning the force lies stopped there: where the next unit in the order has men ashore and the berths not yet filled hold its men, a load order for it; a unit the free berths do not hold waits for the next trip, and no later unit is taken before it.
11. **Back.** On the first morning the force lies stopped at the place with men aboard and no unit to load by item 10, it is given a route to the source and an unload order.
12. **Again,** by item 9, while a unit covered has men ashore at the place.
13. **A unit larger than all the force's berths** is taken in parts: the load order takes what the berths hold, by the rule of a unit divided (chapter 7), and the rest follows.
14. **A run by night** (decision 0145) covers these trips as it covers a lift.

## Met

15. **On the first morning the intent is met** the staff marks the plan complete and hands in the notice that it is met. The force that carries the last men is given its route back that same morning, before the plan is marked.
16. **By its date.** Where the intent has a date, the staff says on the first morning the plan is released on what day the last men will be off, from the trips the berths need and the passage, and where that is after the date hands in the notice that it cannot meet the intent, with the days it is late.

## What it leaves for later

- **Supply and fuel taken off or destroyed,** and works destroyed.
- **A rear guard,** and a taking off under attack timed against the enemy.
- **Men taken off by a force other than a transport force,** and a march out overland to another place.
- **Leaving formed by the staff of itself,** when it judges a place cannot be held: stage 4.
- **To settle before it is built:** item 6 flies every air group out on the first morning, the fighters among them, so the transports load for days with no cover of their own. The review of Nov 17, 2026 asks whether the fighters should go last.

## Rule card

- **Reads:** the side's own land units, air groups, task forces and bases; the map; the aircraft types' transfer range; the intent's limits, date and leaves.
- **Writes:** one kind of intent with its base and date, in the command state; nothing else.
- **Reuses:** the rebase order (0091); the load order with a unit, the route and the unload order (0066); `mark_plan` and the notices; the run by night (0145).

## What it costs

- **State:** one kind of intent, in the hash and named. The state layout, saved game, agent interface and doctrine versions rise. The rules label does not.
- **The rulebook's chapter 8** gains "Leaving a place".

## The hand calculation

`tests/scenario_75_leaving_a_place.rs`.
