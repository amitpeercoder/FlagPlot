# 0126: Escorted and covered runs

Status: settled by the architect on Nov 16, 2026, inside the plan for step 3, stage 2 (decision 0113, doctrine 9, and step 5 of its seven). It is the doctrine that uses decision 0121's escort rule and decision 0124's stand task. It adds no rule of the engine. Built on Nov 16, 2026, in step 5, with decision 0121 ("As built" below).

Summary: Under "keep supplied", the staff gives a run an escort from the escort forces the commander named, when it believes the run is threatened and by the risk level, has the escort keep station on the run and frees it when the run is home; and it gives each surface combat force the commander named the task of standing at the run's destination by night

Standing: Settled by the architect, Nov 16, 2026, inside decisions 0113, 0121 and 0124. Built

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

## As built

Built on Nov 16, 2026, with decision 0121. The doctrine is `crates/sim-staff/src/escort.rs`, called from the supply doctrine's `dispatch` (`doctrine.rs`); the covering force is `Day::supply_covers` in `stand.rs`. The doctrine is `supply-12`. Readings the build took:

- **Rule 2 is read whole:** with no task force in the forces limit the threat is not read and nothing is said, so every scenario built before gives the orders and reasons it gave. The first draft of two properties of the hand calculation ("Preserve, a threat and no escort" and "Balanced, a threat and no escort") took force 1 out of the forces limit, which rule 2 answers; the architect ruled rule 2 stands and restated both on Nov 16, 2026, after the build, with force 1 named and held, and added "None named, the threat is not read", at any risk level. The tests follow the restatement; the two that were ignored are removed.
- **The threat** is decision 0124's at balanced risk: every contact of ships within 15 hexes of the destination counts, at every risk level, the nearest and then the lowest number named.
- **A threat and no escort free at preserve:** the loaded run is not sent; the day's reason says why ("The loaded run cannot sail: ..."), the cannot-meet notice rests on it, and no new run is weighed while it waits. It is sent on the first morning the table lets it (day 3 in the hand calculation). At preserve with no threat and none free, the run sails alone.
- **A free escort** needs a ship afloat. Its fuel is the engine's forecast of the run's waypoints out and back at the run's speed in knots, times the margin; where the forecast stops because a ship would run dry, that ship's need is stated whole, the passage at its class's rate.
- **The escort order** is handed in before the run's route order. **The clear:** on a morning of a released plan on which a named escort's charge is stopped at the source with a ship afloat, its charge is cleared; a held escort is left alone. In the hand calculation that is day 5, and the run's force has not ceased to be.
- **The covering force:** the air doctrine now also runs for a side whose keep-supplied intent names task forces; it takes the intent's plan as released on a morning the supply doctrine releases it (the leaves given), and forms by decision 0124's rule a stand task at the destination's hex for each named surface force (not carriers). Intake takes a stand task under "keep supplied" at the destination's hex only.

**What it was checked against.** Scenario 56 (`data/scenarios/56_an_escorted_run.toml`, on the frozen figures) came out as the hand calculation has it: one more order on day 2 (the escort), force 1 in the run's hex at the end of every pulse from day 2 to day 4, arriving at T at hour 44 and at S at hour 86, 50 t taken by each destroyer on day 4, the clear on day 5 and no other order to it, the second run alone on day 17 with its reason, every reading, load and stock as scenario 36's, base 1's fuel 100 t below scenario 36's (1860 t at the end), as the comparison force burns; and the properties press, preserve (75 t needed against 500 t), short of fuel (10 t against 60 t, with base 1's fuel set to 0 so that the ships are not filled before the run), held, covering force (routed at full speed to (5,0) and back on days 1 and 2, standing at hours 6.25 and 30.25, in port from day 3), none named and the two worlds. The table first put the ships' fuelling at hour 86 "in the afternoon of day 4"; hour 86 is in the morning pulse, where the engine fills them, and the architect restated the row on Nov 16, 2026, after the build. No test of scenario 56 is ignored.

**Rulings after the build (Nov 16, 2026).** Every reading above was accepted as built, rule 2 over the two first-draft properties among them.
