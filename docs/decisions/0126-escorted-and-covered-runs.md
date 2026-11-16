# 0126: Escorted and covered runs

Status: settled by the architect on Nov 16, 2026, inside the plan for step 3, stage 2 (decision 0113, doctrine 9, and step 5 of its seven). It is the doctrine that uses decision 0121's escort rule and decision 0124's stand task. It adds no rule of the engine. Not built. It is built in step 5, with decision 0121.

Summary: Under "keep supplied", the staff gives a run an escort from the escort forces the commander named, when it believes the run is threatened and by the risk level, has the escort keep station on the run and frees it when the run is home; and it gives each surface combat force the commander named the task of standing at the run's destination by night

Standing: Settled by the architect, Nov 16, 2026, inside decisions 0113, 0121 and 0124. Not built

## Why

A supply run sails alone. The escort rule (decision 0121) lets warships guard it, and nothing yet decides when they should. The record gives counts of what sailed and no rule for the number (`docs/research/oob/escorts.md`), so the judgement is the staff's, from the threat it believes in and the risk it was given.

## What the commander names

1. **An escort is drawn from what the end may use.** The forces limit of a keep-supplied intent may name task forces, as it may today. A named force on the escort mission is an escort the staff may give to a run. A named force on the surface combat mission is a covering force.
2. **With none named, runs sail as they do today.**

## The doctrine, at the area's level, where "keep supplied" is

3. **The threat to a run** is read on the morning the run is routed: the side holds a contact of ships within 15 hexes of the run's destination. It is decision 0124's threat, taken at the destination.
4. **Whether the run is escorted:**

| | Preserve the force | Balanced | Press |
| --- | --- | --- | --- |
| A run is escorted | Always, where an escort is free | On a threat, where an escort is free | Never |
| A threat and no escort free | The run does not sail, and the staff hands in that it cannot meet the intent | The run sails alone, and its reason says so | The run sails |

5. **A free escort** is a named force on the escort mission that is stopped at the run's source, has no charge, is not held, and has in each ship the fuel for the passage to the destination and back, by the engine's measure at the run's speed, times the margin its risk gives (decision 0124's table). The lowest numbered free escort is taken. One escort force goes with one run.
6. **The orders.** On the morning the run is routed, the escort is given the run's force as its charge (decision 0121, 1). It then keeps station by that decision's rule and needs no route. On the first morning the run's force is back at the source, the escort's charge is cleared, and the escort is free again on the morning after it has fuelled.
7. **A covering force.** For each named force on the surface combat mission, a stand task (decision 0124) is formed under the keep-supplied intent with the destination's hex as its station. Its own doctrine decides the nights it sorties.
8. **Reasons** carry the contact and its distance from the destination, the escort taken or why none was, and the fuel figures.

## What it leaves out

- **How many escorts.** One named force goes or none does. Sizing an escort to the threat needs a way to split and join forces by doctrine, and a rule the record did not give.
- **Timing a run to the dark.** Decision 0113 has a run arrive at dusk and be clear by morning where the enemy holds the air. A cargo ship at a lighter port unloads for days, so it cannot be clear by morning, and a run by fast ships that can is a different run. It waits for the slice's data in step 7.
- **Escorts for a run under "hold a place".** Supply under "hold a place" was left to step 5 by decision 0117. It stays with "keep supplied": a commander who wants a held base fed states both.
- **Air cover of a run.**

## What it costs

- **No new state.** The doctrine's version rises.
- **The rulebook's chapter 8** gains "Escorting and covering a run".

## The hand calculation

`tests/scenario_56_an_escorted_run.rs`: scenario 36's base, convoy and intent, with an escort force named and a contact near the destination that ages out: the first run is escorted and the second is not.
