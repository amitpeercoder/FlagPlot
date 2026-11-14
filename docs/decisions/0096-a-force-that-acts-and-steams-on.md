# 0096: A force that acts at a place and steams on

Status: confirmed as to the need by the step 3 spec, which the designer confirmed on Nov 14, 2026 ("What it builds on": needed before stage 2). The rule's details are the architect's, of the same date. Not built. It has no figure. The hand calculation is in `tests/scenario_26_act_and_steam_on.rs`.

## The gap

Orders are taken once, at the start of a day (decision 0055). A force that arrives by night to bombard a base (decision 0092) or to unload has no way to leave before the next day's orders. It lies in the enemy's hex, or off the beach, through the two daylight pulses that follow. No commander would order that, and a staff that did would be read as stupid. A bombardment by a force that is passing was left out of decision 0092 for want of this rule.

## Decisions

1. **A route may mark waypoints to act at.** A route order names its waypoints as now, and may name which of them the force acts at. The last waypoint needs no mark: a force stops there anyway.
2. **A force halts at a marked waypoint if it has work there, and only then.** Work is one of two things:
   - **A bombardment:** its mission is bombardment, a ship of it can fire, and the hex holds an enemy base on its side's map.
   - **Cargo:** it has a cargo order that can work in that hex by the rules as they are: a base of its side there, or men to land on a coast hex, or men of the order's unit ashore there to load.
   With no work it passes the waypoint as any other.
3. **Halted, it is stopped at the centre of the hex,** as a force that has arrived is. It works cargo for the hours of the pulse after it halted, and it bombards in the night's shore bombardment phase. It can be met and fought there as any stopped force can.
4. **It steams on when its work is done.** At the start of each pulse's movement a halted force resumes its route if:
   - for a bombardment, it has bombarded since it halted, or no ship of it can fire any longer;
   - for cargo, its loading or unloading is complete.
   A force with both does both. It then moves in that pulse with the pulse's full hours.
5. **A force that halts by day to bombard waits for the night.** The rule does not move the bombardment out of the night pulse. Arriving by night is a matter of timing, which is the commander's or the staff's.
6. **A new route order ends a halt.** So does a force that is out of fuel, by the rule as it is.
7. **A side sees its own forces halt and steam on,** each with the reason. The enemy sees neither.

## A small worked case

A cruiser force on the bombardment mission lies 4 hexes, 160 nm, from an enemy base. On day 1 it is ordered to the base, marked to act there, and back, at 16 knots.

| Pulse | What happens |
| --- | --- |
| Day 1, night | 10 hours' steaming. It reaches the base's hex and halts. In the shore bombardment phase it fires |
| Day 1, morning | It has bombarded: it steams on. 6 hours, 96 nm toward home |
| Day 1, afternoon | The last 64 nm in 4 hours. It is home at hour 22, having spent no daylight pulse in the base's hex |

Without the mark it would steam through the hex and fire nothing. With the base as its last waypoint it would lie there all of day 1.

## Why

- **The daily cycle of command stays.** Nothing is ordered inside a day. The force carries out what it was told at the start.
- **The mark is explicit.** A convoy whose route passes its own base on the way elsewhere does not stop to unload there unless told to.
- **Work, then go.** One rule serves a raid and a supply run, and the staff of step 3 needs both.

## What it leaves out

- **Acting for a set time,** such as unloading for one night only and leaving with cargo aboard. A force stays until its cargo work is complete.
- **A change of cargo order along a route,** such as loading at one waypoint and unloading at the next. A cargo order is one standing order.
- **Waiting for an hour of the day,** or timing an arrival by rule.
- **Halting to refuel or to repair.**

## What it costs

- **New state:** a route's marked waypoints, and whether a force is halted and for what. They go into the hash and are named beside it; the hash version is raised and the golden files remade (decision 0057).
- **Decision 0092, 1,** now holds for a halted force as for an arrived one. Its "What it leaves out" loses the passing force.
- **The route order gains an optional part,** new events with their rows in `side_events.rs`, and the agent interface: the mark on a route, a force's halt in the briefing and queries.
- **The rulebook:** chapter 2 (orders that move a force, arriving, working cargo), chapter 4 (shore bombardment) and chapter 1 (the order and the events), with the glossary, in the same pull request as the code.

## Rule card

| | |
| --- | --- |
| Reads | A force's route and its marks, position, mission, cargo order, ships and their rounds; the bases in the hex, their side and whether they are on the map; the land units of the order; whether the force bombarded or completed cargo since it halted |
| Writes | A force's status, route index and halt; the events |
