# 0146: Bombardment from the sea: step 5 of stage 3, second part

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 5 of its nine). It writes the task "bombard a base", given to a task force, and the doctrine that serves it, over the rule of shore bombardment (decision 0092), the halt at a marked waypoint (decision 0096) and the doctrine of a force that stands by night (decisions 0124 and 0127), which it follows closely. It adds no rule to the engine. Nothing is built. It is built after decision 0145.

Summary: Step 5 of stage 3, second part: the task "bombard a base", given to a task force on the bombardment mission, by which the staff sends the force to the enemy base in the night, at the speed that brings it there in the dark, with the fuel and the rounds its risk asks for, and home at the morning; formed under "neutralise a base" and under "take a place", where it fires on the night of the day the landing is due

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Not built

## Why

The engine has had shore bombardment since decision 0092: a force on the bombardment mission that ends the night in the hex of an enemy base fires at it. Only a scripted order sends one. The record of the campaign has the airfield shelled from the sea night after night. This record gives the staff the task.

## The task

1. **Bombard a base** names a task force of the side and a base on the side's map that is not the side's: the target. It has no date. It is a task, given to the task force (decision 0124, item 6), and a commander may state it as a root.
2. **It is formed by the staff** under an end whose plan is released, for each task force on the bombardment mission with a ship afloat that the nearest forces limit on the end's chain names, in task force order, where no bombard task for that force is in force anywhere on the side:
   - under **neutralise a base,** at that base;
   - under **take a place,** at the enemy's lowest-numbered base on the side's map in the place's hex. With none there, none is formed.
3. **A force on the bombardment mission is given no stand task,** as now: the mission says which task a force takes.

## The force's morning

Each morning, for each bombard task in force whose plan is released and whose force is not held, in priority order, the staff decides at the force's level from the side's view. It is the stand doctrine's morning (decision 0124, items 9 to 15) with these differences.

4. **The target.** Where the target is no longer the enemy's, or is not on the side's map, the force is given no route; it is set to normal readiness where it is not; and the reason says so.
5. **Whether it sorties tonight.**
   - **As a root, or under neutralise:** at press and at balanced, every night. At preserve the force, every night on which the side holds no contact of ships within 15 hexes of the target that reports more ships than the force has afloat.
   - **Under take a place:** on the morning of the day the landing force is due at the place (decision 0142, item 18), and on no other. Before the landing force is routed that day is not known, and the force waits.
6. **The speed and the route.** As a stand: the force must lie stopped at its home port; it is routed to the target's hex and home, the target's hex marked to act at; cruising speed, then full, the first that brings it there within the night's 12 hours. Where neither does, there is no sortie and the notice that the intent cannot be met is handed in.
7. **Fuel and ammunition come first,** by the stand doctrine's rule 4 and decision 0127, at the task's risk. A force short of either does not sortie; its reason gives the day from which it may be used, or none and the notice.
8. **Readiness and rules of engagement** as a stand: battle readiness on a morning it sorties and normal on a morning it does not; the rules of engagement set once from the risk.
9. **The order of the orders** is the stand's: home port, rules of engagement, readiness, speed, route.
10. **It fires and comes home by the rules as they are.** The force halts in the target's hex, fires in the night's bombardment phase, and steams on at the start of the morning pulse.
11. **Reasons** give the target and its distance, the hours to it, the fuel aboard against the fuel needed, each ship's rounds against the level, and, at preserve the force, the contact that stopped it.

## What it leaves for later

- **What a bombardment did,** read by the staff: a side is told its shells and never its hits (decision 0092). A task that stops when the base is judged out of action waits on what a side can know of that.
- **A bombardment timed with a raid from the air or with a surface fight,** and a force that covers the bombarding force.
- **A force that rearms at another base** when its own holds no ammunition: moving a force's home is the commander's (decision 0127).
- **Bombardment on the nights before a landing,** which wants a choice of how many: stage 4.

## Rule card

- **Reads:** the side's own task forces and ships (place, fuel, rounds, mission, readiness, home port), its map of the enemy's bases, its contacts of ships, the forecast of a passage, the ammunition and fuel figures of decision 0113 and 0127.
- **Writes:** one kind of intent with its task force and target, in the command state; nothing else.
- **Reuses:** the stand doctrine whole but for its threat and its mark; `form_intent`; the home port, rules of engagement, readiness, speed and route orders; the engine's bombardment and halt.

## What it costs

- **State:** one kind of intent, in the hash and named. The state layout, saved game, agent interface and doctrine versions rise. The rules label does not.
- **The rulebook's chapter 8** gains "Bombarding a base", and "A task force as an addressee" gains the third task.

## The hand calculation

`tests/scenario_71_bombardment_from_the_sea.rs`.
