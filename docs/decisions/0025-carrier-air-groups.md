# 0025: Air groups on carriers

Status: confirmed by the designer, Nov 11, 2026, with the plan for scenario 5 (stage 5a). The details below the plan's level are the architect's.

Summary: Air groups aboard ships: aircraft capacity, position from the ship, search from a carrier, lost with the ship

Standing: Confirmed

## The gap

The spec says an air group is based at an airfield or on a carrier, and that every air group is at exactly one base or on one carrier. Until now an air group could only be at a base. Scenario 5 needs carriers.

## Decisions

1. **An air group is based at a base or aboard a ship.** Exactly one of the two, always.
2. **A ship class has an aircraft capacity,** a whole number of aircraft, 0 for a ship that carries none. The aircraft of all the groups aboard a ship may not exceed its class's capacity. A scenario that breaks this does not load, and it is checked as an invariant at the end of every day.
3. **A group aboard a ship is where the ship is.** Its position is the position of the ship's task force. It moves with the ship and needs no order to do so.
4. **Search from a ship uses the rule of decision 0012 unchanged.** The search is flown from the ship's position at the start of the pulse, because search comes before movement in a daylight pulse. Distance, arc and darkness are worked from that hex.
5. **A group aboard a ship that is sunk is lost with it.** All its aircraft are lost, and an event says so, citing the sinking. Landing on another carrier or ashore is deferred.

   *Nov 14, 2026: a group in the air when its ship is sunk or her deck is closed lands on another deck or a field, by decision 0059; a group aboard a ship that is sunk is still lost with it.*

6. **A group aboard a ship in a base's hex is still aboard the ship.** Moving a group between a ship and a base is an order that does not exist yet. Its position is then the base's hex.
7. **Any aircraft type may be based on any ship with the capacity.** Which types can fly from a deck is a fact about the type, and arrives with the real aircraft in stage 5e.
8. **The class file's aircraft capacity column is optional.** A class file without it holds no ship that carries aircraft. This keeps the generated Savo Island class file as it is until the derivation tool learns the figure, in stage 5e.

## Why

- **Position from the task force** means there is one answer to "where is this group", and it cannot disagree with the ship.
- **Lost with the ship** is the simple rule and the harsh one. It overstates losses when a carrier sinks slowly near a friendly deck. It is recorded here so that the complexity review sees it.

## Deferred

- Aircraft that are damaged or in reserve, pilots and fatigue: scenario 6.
- Moving a group between ships and bases. Emergency landings.
- A flight deck that damage can close: stage 5b, with the first strikes.
- How many aircraft a deck can launch in a pulse.

## Rule card

| | |
| --- | --- |
| Reads | The group's ship; the ship's task force position; the ship's class capacity; a ship sinking |
| Writes | The group's aircraft (to 0 when its ship sinks) |
| Events | `AirGroupLost`, caused by the `ShipSunk` event |
| Randomness | None |
