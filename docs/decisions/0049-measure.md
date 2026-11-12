# 0049: Measuring

Status: confirmed by the architect, within the plan for the agent interface the designer approved on Nov 11, 2026.

## The gap

The spec asks for geometry as tools: range, path, travel time and air coverage, because a model reasons poorly over raw hex coordinates. So does a person. The rules for each already exist inside the engine, and a player has no way to ask them a question before giving an order. Decision 0040 added that whatever measures an air distance must use the game's own function.

## Decisions

1. **`measure` answers four questions, each with the function the rules themselves use.**
   - **Air distance** between two places, in miles (decision 0040).
   - **Sea route** between two places: the hexes of the shortest path a force would take, its length in miles, and whether there is one (decision 0017).
   - **Passage** of one of the side's own task forces to a place: the sea route, the hours it takes at the force's ordered speed or at a speed asked for, the day and pulse it would arrive if it left at the start of the coming day, the fuel each ship would burn (decision 0006), and whether each ship has it.
   - **Air reach** of one of the side's own air groups: its type's radius, and for each place asked about the air distance and whether it is within the radius. For a fighter the same radius is its escort radius (decision 0030).
2. **A place is a hex, or something the side knows the place of:** one of its own units, a base, or a contact, which stands for the hex the side believes it is in. An enemy unit named by the log's number is not a place: the answer is "not known", the same as for a number that does not exist.
3. **Every figure is the engine's.** `measure` calls the functions movement, fuel, pathfinding and air range call. It does not have its own arithmetic for any of them. A test compares its answers with what then happens when the order is given and the day is run.
4. **A passage is a forecast, and says what it assumes:** that the force leaves at the start of the coming day, keeps the speed, meets nobody, and loses no ship. Where the force is between two hexes the forecast starts from where the force really is, by the turn-back rule (decision 0005).
5. **It measures the map and the side's own units, and nothing of the enemy.** It gives no chance of sighting, no odds of a fight and no advice.
6. **It reads a side's view and writes nothing,** like the other queries, and answers in JSON with a schema.

## Why

- **One function** is the lesson of the straight-line change: if the tool measured one way and the rules another, the tool would tell a player that an aircraft can reach what the rules say it cannot.
- **Passage, not only distance.** The question a commander asks is "can this force be there by the night of the 24th, and get home". Distance alone leaves the division, the speed rule and the fuel rule to the asker, which is where a model slips.
- **Contacts as places** lets a player ask "how far is that sighting from my carriers" without knowing a coordinate. It uses the believed position, so it tells nothing new.

## What it costs

- A passage forecast is wrong whenever the force is delayed, fights or changes speed. It says so, but a forecast that reads as a promise will be taken as one.
- It does not answer "what can reach me": a side does not know the enemy's aircraft or where they are based.

## Deferred

- Search coverage: what an arc covers and the chance of sighting within it.
- Areas: everything within reach of a base, drawn or listed.
- Round trips and "furthest point this force can reach and return from".

## As built

- **A passage is forecast by running the movement step's own code on a copy,** pulse by pulse. Nothing is worked out twice. Movement, the strike decision and the forecast now share the functions for a force's speed, the fuel a ship burns, the route and the radius test.
- **A fifth assumption is stated:** the force has the fuel aboard now and takes none.
- **An order the engine would refuse is reported as refused,** with the reason: already there, no route, no ship afloat. A force whose slowest ship cannot move is reported as unable to move.
- **Where a ship would run dry** the forecast names the ship and the miles, and stops there as the force would.

Checked against what happened: nine passages in six scenarios, 73 figures (hexes entered, miles, arrival day, pulse and time, fuel for each ship), all equal; six passages begun between two hexes; and every strike launch of Eastern Solomons and of the Rabaul scenario over twenty seeds agreeing with the reach given beforehand.

## Rule card

No rule of the game is added. Reads a side's view and the map. Writes nothing.
