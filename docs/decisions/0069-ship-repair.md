# 0069: Ship repair

Status: confirmed. The designer approved the plan on Nov 13, 2026, with the answers below. Nothing is built. The hand calculation is written before any code is delegated.

## The gap

No rule ever lowers a ship's damage. A system once out stays out, in port or at sea, for the rest of the game (decision 0052, item 11). In one battle that does not matter. Over a campaign it does: fleets only wear down, a damaged ship has nowhere to go, and a commander has no reason to pull one out of the line.

The record is in `docs/research/oob/step2-record.md`, section 3, from `ship_facts.csv` rows 613 to 769 and `place_facts.csv`. It shows two kinds of repair, by where it was done.

- **Forward, at a base with a repair ship.** Divers, patches and ship's force. South Dakota and Enterprise were back 16 days after the bombs of 26 October with no yard. No forward base in the slice could dock a cruiser or do turret work, and the Japanese had the same split between Rabaul, Truk and the home yards.
- **In a yard, far away.** Enterprise after 24 August was away 53 days, Saratoga 71, Shokaku 125, Boise 160, Chester about 330.

The spec does not ask for this before tuning (`docs/step2-spec.md`, "Rule work that comes before tuning"): it matters to a campaign, not to one battle's losses.

## Decisions

### Repair at a forward base

1. **A base may have a ship repair figure:** points of damage it mends each day, 0 unless the scenario gives one. It stands for a repair ship or tender there. With no such base in a scenario, nothing changes.
2. **Repair is the staff's, as refuelling in port is.** At the start of each day, each force stopped at a base of its own side is repaired from that base's figure, forces in number order and ships in number order within a force. Nobody orders it. It uses the test refuelling already uses for "stopped at a base" (decision 0045).
3. **A forward base mends light damage and no more.** A point of repair takes off a point of fire damage, or a point of flotation damage if the ship's flotation damage is below a figure, `forward_repair_max_flotation`. A system that is degraded is put back to working at a cost in points. Flotation damage at or above the figure, and a system that is out, need a yard.

### Repair in a yard

4. **The yards are off the map.** A scenario may mark a base of a side as a port for the yard, with the days of the passage there and back.
5. **A force stopped at such a base can be ordered to the yard.** The whole force goes. It leaves the map, and returns to the same base on a stated day with every ship fully repaired, refuelled and rearmed. To send only the damaged ships, detach them first, with the order that exists.
6. **The days away are worked out, not drawn,** by one function, for each ship: the days of the passage, plus a figure for each point of flotation damage, plus a figure for each system degraded and a larger one for each system out. The force is away for the days of its slowest ship to mend. There is no random draw.
7. **The player is told the day of return before he sends her.** `measure` answers it, and the order's acceptance states it. The briefing lists forces away and their day of return.
8. **A force that would return after the scenario ends does not return.** Its ships are afloat and out of the game.

### What a side knows

9. A side sees its own repairs, departures and returns whole. The enemy sees nothing of them: a force that leaves for a yard is a contact that ages and is dropped, as any force that steams out of sight.

## A small worked case

Placeholder figures, for the example only: a base that mends 4 points a day; `forward_repair_max_flotation` 30; a degraded system costs 10 points; a yard passage of 20 days there and back, 1 day for each point of flotation damage, 10 for a system degraded and 30 for a system out.

| Ship | Damage | At the forward base | If sent to the yard |
| --- | --- | --- | --- |
| A | Fire 12, flotation 20, fire control degraded | 42 points: ready on the 11th day | 20 + 20 + 10 = 50 days |
| B | Flotation 60, flight deck out | Fire only; the rest needs a yard | 20 + 60 + 30 = 110 days |

A is of the order of South Dakota's 16 days in October. B is near Shokaku's 125. The figures themselves are set later from the nineteen ships in the research.

## Why

- **Two places, two speeds.** That is the pattern of the record, and it gives the commander a real choice: keep a damaged ship fighting, patch her forward for a fortnight, or lose her for months and get her back whole.
- **The yard is a day on a calendar, not a place.** The yards were thousands of miles off the map. Steaming there would add map, fuel and escort rules for a passage nobody can attack.
- **Worked out, not drawn.** The staff planner of step 3, and the player, must be able to count on a ship's return. The record's spread is wide, but it follows the damage, which the function reads.
- **It reuses what exists:** the "stopped at a base" test of refuelling, a daily allowance for a base as for aircraft repair (decision 0035), the detach order, and the arrival of units on a day (decision 0041).
- **Off unless a scenario asks.** No base has a repair figure and none is a port for the yard until a scenario says so, so no scenario changes.

## What it leaves out

- **A ship that must be patched before she can make the passage.** New Orleans needed eleven days at Tulagi to be seaworthy. Any ship that can reach the port may be sent.
- **Towing, scuttling and damage control at sea** (decision 0016).
- **Repair that costs the base supply,** as aircraft repair does. It can be added by the same figure when supply for ships exists.
- **A ship sent alone from a force.** Forming and merging forces by order is not built (decision 0052, item 7), so a force goes and returns as it is.
- **Attacks on a repair ship,** and a repair ship that moves between bases. The figure belongs to the base.
- **Refits that improve a ship.** She returns as her class has her.

## What it does for "out of the campaign"

The opening list has a second item: points on the objectives scale for a ship that must leave for repair. It should not read whether the enemy sent her to a yard, since a side could then deny the points by keeping a wreck in the line. It can read the function of decision 6: a ship is out of the campaign if the yard would keep her longer than some number of days. That is its own decision record. This one only provides the function.

## What it costs

- A new field of state for a base (its repair figure and its yard passage) and for a task force (away, and its day of return). The hash version rises and the golden files' hashes are remade.
- One new order, two new kinds of event for the yard and one for a repair, each with its line in `side_events.rs`.
- A handful of placeholder figures in `surface_combat.toml`, to be set from the research.
- The rulebook's "No repair" section is rewritten, with chapter 1 (the order), chapter 2 (the daily step) and the glossary.

## Rule card

| | |
| --- | --- |
| Reads | A base's repair figure and yard passage; whether a force is stopped at a base of its side; each ship's fire damage, flotation damage and systems; the repair figures; the day and the scenario's last day |
| Writes | A ship's fire damage, flotation damage and systems; a force's away state and day of return; for a returning force, its ships' fuel and ammunition |

## When to build it

Not before the three battles are tuned. Ship repair changes no battle's result, and tuning is what the step 2 spec asks for first.

## Answered by the designer

Approved on Nov 13, 2026, each as recommended.

1. **The yard is a day of return, off the map.** No passage is steamed. Decisions 4 and 5.
2. **The days away are worked out, with no random draw.** Decision 6.
3. **Forward repair is the staff's, with no order.** Decision 2.
4. **A whole force goes to the yard and returns together,** until forces can be formed by order. Decision 5.
5. **It is built after the three battles are tuned.**
