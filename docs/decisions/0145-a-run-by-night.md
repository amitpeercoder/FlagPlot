# 0145: A run by night: step 5 of stage 3, first part

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 5 of its nine). Step 5 is written in two records: this one, a run timed to the dark; and the next, bombardment from the sea. It adds no rule to the engine: it times a run that the staff already orders. Nothing is built. It is built after decision 0144.

Summary: Step 5 of stage 3, first part: where an enemy base is near the place a run goes to, the staff times the run to arrive in the night, at cruising speed or at full, carries no more in each ship than the night's hours can land, and the force is gone at the morning; where no speed brings it there in the dark it goes by day and says so, or at preserve the force does not go

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Not built

## Why

A force that unloads by day off a place the enemy can reach from the air is found and struck there. The record's answer was to come in the dark and be gone by light. The rules allow it already: a force routed at the start of a day sails into the night pulse, a force that has landed its cargo steams on at the start of the next pulse, and a speed can be ordered. No staff uses them so. This record lifts the limit of stage 2 that a run is not timed to the dark (`docs/open-work.md`).

## When a run goes by night

1. **The runs it covers** are a supply run of "keep a base supplied" (decision 0107) and a lift of "take a place" (decisions 0142 and 0143).
2. **A run goes by night** at preserve the force and at balanced, where an enemy base on the side's map lies within 15 hexes of the place it goes to and not in that place's own hex. At press it goes as before. The 15 hexes are the placeholder of the stand doctrine (decision 0124). The risk is the intent's, read up the tree.
3. **A base in the place's own hex does not count,** because a landing at an enemy base is the enemy's base itself: what covers that landing is step 6.

## The timing

4. **The hour it would arrive** is worked by the engine's own forecast of the passage from the source, sailing at the first hour of a day, at cruising speed and then at full speed. A passage arrives in the night when its hours, less the whole days in them, are fewer than 12.
5. **The speed ordered** is the first of the two that arrives in the night with room for the load (item 7) and for which every ship has the fuel. Above cruising speed a ship burns its cruising rate times the square of the speed over its cruising speed (chapter 2, Fuel burn), so the fuel each ship needs is the engine's measure of the route out and home at the speed tried, in place of the run's measure at cruise. A speed a ship has not the fuel for is passed over, and the reason gives the two figures. Where it differs from the force's speed the staff orders it, on the morning the force sails, before the route. It is left as it is afterwards.
6. **The night's hours** are 12 less the hour of the night at which the force arrives.

## The night's limit

7. **No ship carries more than it can land in the night's hours:** the place's rate for a ship times those hours, the men by their weight and the supply together, at the rate the force's loading gives it where men are landed.
8. **A supply load is cut to it.** A load to a weight fills ships in ship order, so the load is cut until the first ship filled carries no more than its limit, and each ship after it likewise.
9. **A unit goes only in ships that can each land their part of it in the hours.** A unit that cannot is left for a later run, and the reason says so.
10. **The days of a run are reckoned as before,** at cruising speed.

## Where no speed arrives in the dark

11. **At balanced** the run goes by day at cruising speed, as before, and the reason says that neither speed brings it there in the night, with the two hours of arrival.
12. **At preserve the force** it does not sail. The staff gives no order for it and hands in the notice that it cannot meet the intent, on each morning it would have sailed.

## Gone by light

13. **Nothing is added.** A force that has landed its load in the night steams on at the start of the morning pulse by the rule as it is (decision 0096). It is at sea on its way home by day, as a force that stood for the night is.

## Reasons

14. **A reason for a run by night** gives the enemy base and its distance, the two hours of arrival, the speed chosen, the night's hours, each ship's load against its limit, and what was cut or left.

## What it leaves for later

- **Sailing at an hour inside a day,** so that a longer passage can end in the night: orders inside a day are not built.
- **A base known to hold aircraft,** and the reach of those aircraft, in place of 15 hexes from any base: it waits on what a side knows of an enemy field.
- **A load put ashore and left on the beach,** and a force that leaves with cargo aboard at the morning.
- **The first landing of an operation covered from the air:** step 6.

## Rule card

- **Reads:** the side's map of the enemy's bases, the forecast of a passage at two speeds and the fuel each burns, the port figures, the ships' room and the men aboard, the risk.
- **Writes:** nothing of its own: a speed order, and the weight of a load order.
- **Reuses:** the supply doctrine's run and the lifts of decisions 0142 and 0143; the speed order; the halt and the steaming on of decision 0096.

## What it costs

- **State:** none. The doctrine version rises. A game in which no run goes to a place within 15 hexes of an enemy base in another hex plays as before: scenarios 67, 68 and 69 do.
- **Earlier scenarios with a supply intent may change,** where an enemy base on the side's map lies within 15 hexes of the base supplied. The build lists each, and its new numbers go to the architect, not to the developer.
- **The rulebook's chapter 8** gains "A run by night" under the supply doctrine.

## The hand calculation

`tests/scenario_70_a_run_by_night.rs`.
