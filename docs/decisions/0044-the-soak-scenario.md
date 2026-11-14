# 0044: The soak scenario

Status: confirmed by the architect, on the designer's standing word of Nov 11, 2026.

## The gap

The spec's scenario 8 is "Soak: 180 days with scripted orders on both sides", proving no crashes and that all invariants hold. The longest run so far is 28 days. Nothing has shown that the engine survives a campaign's length: stocks that run out, groups that are destroyed and never rebuilt, ships out of fuel, counters that only grow.

## Decisions

1. **Scenario 8 is the full slice of scenario 7 carried on for 180 days,** from 21 August 1942. It is a machine test and not history: nothing in it is compared with the record, and its forces do what keeps every subsystem busy, not what the commanders did.
2. **Every subsystem stays in use to the end.** The orders are scripted so that in each 30 days of the run there is at least one air raid on a base, one air search, one strike on ships or attempt at one, one surface fight, one convoy unloading, one repair, one arrival or replacement, and one group stood down and returned. A test counts them window by window. A soak in which everything is dead by day 20 proves nothing about the other 160.
3. **What must hold, every day of every run:** the engine does not panic; every invariant it checks at the close of a day holds; no stock, count or fatigue leaves its range; aircraft, pilots and supply are conserved as the scenario 6 tests state it; and the day's hash is the same on a second run.
4. **It runs for the scenario's own seed and a handful of others in the ordinary test run,** and for more in a test that is run by hand. The number in the ordinary run is set by time: the whole test run must stay practical.
5. **Things running out is expected and is not a failure:** a base out of supply, a ship out of fuel, a group with no aircraft or no pilots, a force with every ship sunk. Each must leave the run going and the invariants whole. The scenario is arranged so that at least one of each happens.
6. **The event log's size and the run's time are measured and recorded,** so that a later change that makes a long game ten times slower or larger is seen.

## Why

- **Carrying on the full slice** means the soak starts from a scenario already known to record every kind of event, and adds only length.
- **Window by window** because the failure a soak exists to find is the slow one: a quantity that drifts, an entity left in a state no rule expected.

## What it costs

- The scripted orders of 180 days are long and say nothing about the war.
- The ordinary test run gets slower by the time of a few 180-day runs.

## What the soak found

**It runs.** 180 days, the scenario's own seed and 48 others in the ordinary test, 400 more by hand: no panic, no invariant broken, no count out of range, aircraft, pilots and supply conserved, and the same hashes twice. Every 30 days has every activity of decision 2. Rabaul is out of supply on day 166; ships are out of fuel from day 2; groups run out of aircraft and of pilots; forces lose every ship.

**It does not grow worse than its length.** A run records 18,659 events, 107 a day in the first 30 days and 90 in the last, and takes 2.2 seconds in a debug build and 0.27 in a release build. Scenario 7's fourteen days scaled to 180 would be 19,183 events. The replay file is 9.2 MB.

**One fault, fixed.**

7. **A force that turned back recorded nothing.** A route order to a force between two hexes can turn it round (decision 0005). Its two hexes and its progress changed with no event, against the rule that every state change records one. It showed because a force that turns back at exactly the middle of a leg is at once in the other hex: a force is in the hex ahead from the midpoint on. The replay viewer, which works out positions from events, had three such forces one hex off. **A turn-back now records an event** with the force, the hex it comes from, the hex it heads for, its progress and its position, citing the order. The midpoint rule is unchanged: changing which hex a force is in at the midpoint would move the meeting times of scenario 4a.
8. **An order to a force with no ship afloat, or to an air group lost with its ship, is rejected,** with a reason. Before, it was accepted and applied to nothing; the fault above was first seen in a force whose one ship had been sunk two days before it was ordered to move.
9. **These cost two scenarios their event numbers:** Savo Island, where forces turn back in 47 runs of 50, and the soak itself. In both, with the new events taken out and numbers stripped, the events and end states are as they were. Scenario 7 was given one order that turns a force back, so that the full slice still records every kind of event.

**Three things found and left, for the designer to know.**

- **Ships cannot refuel.** Fuel only falls. A transport lasts about eight round trips between Espiritu Santo and Lunga Point and its escort runs dry on day 38. The spec lists refuelling under logistics; it has not been built, and no campaign longer than a few weeks can be played without it. *(Nov 14, 2026: built by decision 0045)*
- **Two forces left in one hex fight every pulse.** A force with no home port stays where it fought (decision 0016, 14). One pair met 303 times in the run with the orders that part them taken out, and 6 times with them in. What a force does after a fight is a staff rule that is still deferred.
- **A stranded force can be ordered again** and records that it is out of fuel again. Only its route changes.

## Rule card

No rule of the game is added.
