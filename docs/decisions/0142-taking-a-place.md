# 0142: Taking a place: step 3 of stage 3

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3, that the designer confirmed the same day (decision 0139, step 3 of its eight). It writes the end "take a place" and the doctrine that carries it as far as the landing, over rules already built (decisions 0066, 0090, 0095, 0098 and 0126) and the task "ready a landing" of decision 0141, with the hand calculation. Nothing is built. It is built after decision 0141.

Summary: Step 3 of stage 3: the end "take a place", by which the staff chooses the units that land by what each is good at and by the ships that can carry them, forms "ready a landing" for them, asks leave to launch when it is ready, and on leave loads supply for the men, embarks them, sails, lands them on attack, and says when the place is the side's

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Not built

## Why

A commander can ask for a landing to be made ready (decision 0141). He cannot yet ask for a place. This step lets him: he names the place, the ships, the units and where they sail from, and the staff does the rest up to the day the place changes side. What the men do ashore when the enemy has men there is step 4; a run by night, bombardment, cover by carriers and reconnaissance are steps 5 and 6.

## The end

1. **Take a place** names a hex of land, by its hex or by a base on the side's map that is not the side's, which means that base's hex. It may have a date: the day by the end of which the place is to be the side's. It is an end, addressed to the area, and a root: a commander states it.
2. **Its forces limit** names the task forces, the land units and a source, a base of the side. A forces limit with no source is refused, as for keeping a base supplied.
3. **It is met** when the side holds a base in the hex and the enemy holds none there. Stated of a place that is the side's already, it is met at once and nothing is ordered.
4. **In this step the staff lands at the place itself.** For a place that is not a coast hex it gives no order and says that it has no way there: a march from a beach is step 4.

## The phases

5. **A plan's phase is read, not stored.** Prepare, while the plan is preparing. Move, from its release until the first man lands at the place. Land, from then until the intent is met. The tools show the phase. Follow up is step 4.

## Prepare

6. **The transport force** is the lowest-numbered task force of the forces limit on the transport mission, with a ship afloat of a class that carries men, that the side may use and the commander does not hold. It must lie stopped at the source, or have men of the limit's units aboard. One force lands an operation in this step.
7. **The units that may land** are the fighting units of the forces limit that the side may use, that the commander does not hold, and whose men are all ashore in the source's hex or all aboard the transport force. A working unit is not landed by this step.
8. **Their order** is by what each is good at: the higher assault skill first; then the higher experience; then the lower number.
9. **The ships.** A ship carries men of one unit. Units are taken in that order. Each takes the force's ships not yet taken, in number order, until their berths hold its men; a unit for which the ships left are too few is left out, and the next is tried. A unit aboard already keeps the ships it is in.
10. **How many men.** Where the side holds an estimate of the enemy's men at the place, the staff stops taking units once the men chosen are at least the estimate times the figure for its risk:

| The risk | Men for each man of the estimate |
| --- | --- |
| Preserve the force | 3 |
| Balanced | 2 |
| Press | 1 |

With no estimate it takes every unit the ships can carry, and says that it goes with no count of the enemy. Where the units the ships can carry are fewer than the men wanted, at balanced and at press it goes with what it has and says so; at preserve the force it forms nothing, asks no leave, and hands in the notice that it cannot meet the intent. The three figures are the staff's and are placeholders.
11. **The choice is made once,** on the first morning the plan is preparing, and kept as the forces limit of the task it forms. The commander may change that limit, as he may any.
12. **The task formed.** The staff forms "ready a landing" (decision 0141) under the intent, for the transport force at the place's hex, and sets its forces limit to the units chosen. Two things are new for this: `form_intent` for "ready a landing" under "take a place" is taken in while the plan is preparing or released; and the staff may give `set_limit` of forces for an intent it formed, checked as any limit under a parent is. The task is served that morning.
13. **Its date.** Where the intent has a date, the task's date is the morning of that day, less the days to embark and the days of the passage, plus one (below). The task's own rule then says on the first morning whether it will be late, by how many days and what binds.
14. **When it is ready.** As decision 0141 has it, by the risk. One case is added: a force that does not lie at a base of its side can gain no loading and its units no rehearsal, so the task is ready as it stands, and the reason says what it goes without.
15. **Leave.** On each morning the task is ready and the plan is preparing, the staff releases the plan where leave to launch is given. Where it is withheld it gives no order and hands in the notice that the plan waits for leave to launch. Before the task is ready it hands in no such notice: the day's reason gives the loading, each unit's rehearsal and the morning it expects to be ready.

## Move

16. **The supply.** On the morning the plan is released the force is ordered to load supply at the source to a weight: the daily charge of the men chosen for 15 days, the interval of decision 0107 where none is set. It is cut to the source's stock above its reserve and to the ships' room, and what is aboard already counts toward it. With nothing to take no order is given and the reason says so.
17. **The men.** On each morning after, one unit whose men are ashore at the source is ordered aboard, in number order, by a load order with the unit and no cargo. On the same morning it is set to attack. A unit aboard already needs neither.
18. **The days.** The days to embark are one for the supply, where there is any to take, and one for each unit ashore. The passage is its hours at the cruise speed of the slowest class, rounded up to days. A force released on the morning of day R sails on day R plus the days to embark, and is due on that day plus the days of the passage less one.
19. **The passage.** On the first morning that every unit chosen is aboard and the supply is aboard or has nothing more to take, the force is given a route to the place and back to the source, marked to act at the place, and an unload order. The route, the area limit and the fuel each ship needs for the round trip are as for a run (decision 0107). An escort is given as for a run (decision 0126), with the place as the destination. A ship without the fuel, or no route, stops it, and the reason says which.
20. **The ready task under it** is given nothing from the morning the force is routed: the landing spends what it made ready.

## Land

21. **The landing** is the unload order's, by the rules as they are: the men, then the supply, at the rate the loading gives; the first man forms a base where the side has none.
22. **A unit that could not be set to attack before it sailed,** because it was aboard already, is set to attack on the first morning it has men ashore at the place.
23. **Taken.** On the first morning the intent is met the staff marks the plan complete and hands in the notice that the intent is met, once. Nothing else is ordered: what the men do next is step 4.
24. **Not taken.** While the enemy still holds the place the staff gives no order in this step and says how many men it has ashore and what it holds of the enemy's.

## What both keep

25. **Nothing is ordered without leave to prepare,** and nothing sails or embarks without the plan released.
26. **A hold is honoured.** A held transport force or a held unit is left out, and the reason names the hold.
27. **Reasons** carry the numbers: each unit in its order with its assault skill, its experience and its men; the berths; each unit left out and why; the estimate and the men wanted; the supply in tons; the days.
28. **A proposal** shows the tree before it is formed: the task, the units, the ships, the supply and the days.

## What it leaves for later steps

- **The fight ashore,** the march from a beach to a place inland, and what follows the capture: step 4.
- **A landing by more than one force,** and a working unit landed with the fighting ones.
- **Cover, bombardment and a run timed to the dark:** steps 5 and 6.
- **A landing not ready until it holds a count of the enemy:** step 6.
- **Choosing the day and the beach:** stage 4.
- **Building up the base it sails from:** formed with cover in step 6, where a field is what cover needs.

## Rule card

- **Reads:** the side's own task forces, ships, land units and bases; its estimate of the enemy's men at the place; the map; the ground figures; the intent's limits, its date and the two leaves.
- **Writes:** one kind of intent with its place and date, in the command state; nothing else.
- **Reuses:** `form_intent`, `set_limit`, `approve`, `mark_plan` and the notices; the task of decision 0141; the load, route, unload and posture orders; the run's route, fuel and escort rules. No rule of the engine changes but the two of item 12.

## What it costs

- **State:** one kind of intent, in the hash and named. The state layout, saved game and doctrine versions rise; the agent interface version rises with the tools. The rules label does not: no game without the intent plays differently.
- **The rulebook's chapter 8** gains "Taking a place".

## The hand calculation

`tests/scenario_67_taking_a_place.rs`.
