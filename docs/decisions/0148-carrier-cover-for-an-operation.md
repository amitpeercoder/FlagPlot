# 0148: Cover for an operation by carriers: step 6 of stage 3, second part

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 6 of its nine). It lets "take a place" form the task "operate a carrier force" (decision 0125) at a hex the staff chooses, and lets that force's bombers strike the enemy's base at the place. It lifts two limits of stage 2: the staff chooses where a carrier force stands, and a carrier strikes a base (`docs/open-work.md`). It adds no rule to the engine. Nothing is built. It is built after decision 0147.

Summary: Step 6 of stage 3, second part: under "take a place" the staff gives each carrier force its forces limit names the task of operating from the first hex on its way from which its bombers reach the place; there it searches and flies its patrol as before, strikes ships first, and on a morning with no ship to strike sends its bombers at the enemy's base at the place by the raid's rules, until the place is taken, when the force is sent home

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Not built

## Why

A landing goes in under what the carriers can do for it: they find and strike the ships that come for the transports, and they strike the place before the men land. Stage 2 gave a carrier force its task, but the staff forms it only where the force already lies, and its bombers strike ships and nothing else.

## The task formed

1. **Under a "take a place" whose plan is released,** the staff forms "operate a carrier force" for each task force on the carrier mission, with a carrier afloat that has an air group aboard, that the nearest forces limit on the intent's chain names, in task force order, where no carrier task for that force is in force anywhere on the side. It is the forming of decision 0125, item 2, with a third parent.
2. **The hex** is the covering hex: the first hex along the shortest sea route from the hex the force is in that morning to the place's hex that is within the reach of the place, the reach being the least strike radius, at normal reach, among the bomber groups aboard (decision 0130, items 12 and 13, with the place for the contact). With no bomber group aboard, or the force already within the reach, the hex the force is in. With no sea route, none is formed and the reason says so.
3. **The force is then the task's,** by the carrier doctrine as it is: it is routed to its hex at the speed of that doctrine, flies its patrol, searches, rests its groups, weighs its fuel and closes on a contact. The enemy's bases on the side's map are among the threats of its search already, and the base at the place is one.

## The strike on the place

4. **Ships first.** A bomber group that the carrier doctrine sets to strike ships is left to it on a morning the side holds a fresh contact of ships within the group's reach of the force's hex (decision 0130, item 12: no more than 3 pulses old).
5. **Else the base.** On a morning with none, while the task stands under a "take a place" that is not met, the enemy holds a base on the side's map in the place's hex, and that hex is within the group's reach of the hex the force is in, the group is set to the base strike mission at that base, the lowest-numbered there, by the raid's rules 1 to 4 (decision 0120): which bombers go, whether the raid goes at the task's risk, what the last raid cost, and the orders. A fighter group aboard is the force's cover and is given no order by this: its escorts are what its patrol share leaves.
6. **Back to ships.** On a morning a fresh contact is in reach again, the group is set back to the naval strike mission. When the base changes side the engine clears the mission (chapter 7, A base that changes side) and the staff gives no order for it.
7. **A patrol over the beach is not flown.** A fighter's patrol covers its own force. Cover of the transports and of the men ashore is by what the carrier strikes and finds.

## When the place is taken

8. **On the first morning the intent is met** the force is routed to its home port at cruising speed, where it has one, and the task is given no order after: its groups are left as they are. With no home port it is given no order, and the reason says the commander must move it.

## Reasons

9. **The reason of the forming** gives the place, the force's hex, the reach and the group that sets it, the hex chosen and its distance from the force and from the place. **The reason of a base strike** gives that no fresh contact of ships is in reach, the base and its distance against the reach, and the raid's figures at the risk.

## What it leaves for later

- **Choosing the day it sails** so that it is on station no earlier than it must be: stage 4, with the choice of the day.
- **A patrol over another force or over a beach,** which the air rules do not have.
- **A covering hex chosen against the enemy's bases and searches,** not only by the bombers' reach.
- **Carriers under "hold a place" striking a base,** and a strike on a base timed with a bombardment.

## Rule card

- **Reads:** the side's own carrier forces and air groups, the map and its sea routes, the side's map of the enemy's bases, its contacts of ships, the aircraft types' reach, the raid's table.
- **Writes:** nothing of its own: `form_intent`, and mission and route orders.
- **Reuses:** the carrier doctrine whole (0125, 0130, 0131); the raid's rules 1 to 4 and the base strike mission (0120); the closing rule's reach and first hex (0130).

## What it costs

- **State:** none. The doctrine version rises. A game with no "take a place" that names a carrier force plays as before.
- **The rulebook's chapter 8:** "A task force as an addressee" gains the third parent, and "Operating a carrier force" gains "Under take a place".

## The hand calculation

`tests/scenario_73_carrier_cover.rs`.
