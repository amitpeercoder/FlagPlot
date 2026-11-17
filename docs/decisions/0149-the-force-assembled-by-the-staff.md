# 0149: The force assembled by the staff: step 7 of stage 3

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 7 of its nine, added by the designer on Nov 17, 2026). It lets "take a place" be stated with no forces named, or with some: the staff chooses the rest. It adds no rule to the engine. Nothing is built. It is built after decision 0148.

Summary: Step 7 of stage 3: "take a place" may be stated with no forces named; the staff then works from its estimate of the enemy to the men wanted, the base they can sail from, the units by what each is good at, the transport force, and the bombardment and carrier forces it can give, keeps its choice as the forces limit of the task it forms, and says in numbers what it could not find

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Not built

## Why

Decision 0142 has the commander name the transport force, the units and the base they sail from. That is the staff's work. The designer set out the order a staff works in (decision 0139, "Amended by the designer"): the enemy, the men needed, the lift, the risk on the way, whether it can be kept up, and then the landing made ready. Steps 3 to 6 built each of those over forces the commander named. This step lets him name none.

## What the commander may leave out

1. **A "take a place" needs no forces limit.** Item 2 of decision 0142, that a forces limit with no source is refused, is withdrawn for this end.
2. **Each of three things is the commander's if he names it and the staff's if he does not:** the source; the land units; the task forces. What he names stands, and the staff chooses the rest inside it.
3. **What the staff chooses is kept** as the forces limit of the task "ready a landing" that it forms: the task forces, the land units and the source. Wherever decisions 0142 to 0148 read the forces limit of the "take a place" for one of the three, they read the commander's where he named it and this one where he did not. The commander may change it, as he may any limit.
4. **It is chosen once,** on the first morning the plan is preparing, and again on the first count of the enemy (decision 0147, item 16).

## What the staff may take

5. **A unit, a force or a base the staff may take** is the side's, one the side may use that day (decision 0140), not held by the commander, and not named in the forces limit of another intent in force that is higher in priority, nor taken by a task of one.

## The order it works in

6. **The enemy.** The side's estimate of the men at the place, with its grade and its day; and the men wanted, by the risk (decision 0142, item 10). With no estimate, as many as can be carried.
7. **The source.** A base can be the source if it is the side's, the staff may take it, a transport force it may take lies stopped there (decision 0142, item 6) and a fighting unit it may take has all its men ashore in its hex. For each such base, in base order, the staff works the lift it could make: the force of item 9 and the units of item 8 there. The source is the base whose lift gives the men wanted with the shortest passage to the place, by the engine's forecast for that force; where none gives them, the base whose lift gives the most men; then the lower number.
8. **The units,** from those at the source that it may take, in the order and by the ships of decision 0142, items 8 and 9, until the men wanted.
9. **The transport force,** of those stopped at the source that it may take: the lowest-numbered whose berths hold the men wanted; where none does, the one with the most berths, then the lowest numbered. One force lands an operation, as before.
10. **The risk on the way.** Every force on the bombardment mission and every force on the carrier mission that the staff may take, that lies stopped at a base of the side or, for a carrier force, anywhere, is taken, and is given its task by decisions 0146 and 0148. The escort is the run's (decision 0126), from the forces on the escort mission that the staff may take.
11. **Whether it can be kept up** is decision 0144's check, with the source chosen.

## What it could not find

12. **The reason of the choice gives each step in that order with its numbers,** and what is short: the men wanted against the men the lift gives; the berths wanted against the berths of the force; that it has no force to bombard, to cover or to escort.
13. **With no base that can be the source** the staff forms nothing, hands in the notice that it cannot meet the intent, once, and gives the reason each morning, with what is missing: a transport force stopped at a base of the side, or a fighting unit beside it.
14. **With fewer men than wanted** it does as decision 0142, item 10: at preserve the force it forms nothing and hands in the notice; at balanced and at press it goes with what it has and says so.
15. **A lack of a bombardment, carrier or escort force stops nothing** beyond what the rules of those forces already say. The reason names it.

## What it leaves for later

- **Forces brought together:** units and ships moved to one base, forces joined, and an operation that sails from two bases. The staff takes what already lies together.
- **Looking ahead to an operation not yet stated,** and keeping forces back for it: stage 4.
- **A source chosen for what it holds,** its supply, its airfield and its port, beyond the check of decision 0144.
- **Weather.**

## Rule card

- **Reads:** the side's own bases, task forces, ships and land units, with their places, missions, days and holds; every intent's forces limit and priority; the side's estimate; the forecast of a passage.
- **Writes:** nothing of its own: `form_intent` and `set_limit` on the task it formed.
- **Reuses:** every rule of decisions 0142 to 0148.

## What it costs

- **State:** none. The doctrine and agent interface versions rise. A "take a place" with a forces limit that names all three plays as before.
- **The rulebook's chapter 8,** "Taking a place", gains "With no forces named".

## The hand calculation

`tests/scenario_74_the_force_assembled.rs`.
