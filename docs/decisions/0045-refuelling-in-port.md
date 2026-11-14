# 0045: Refuelling in port

Status: confirmed. The designer chose on Nov 11, 2026 to build refuelling before the agent interface. The details are the architect's.

## The gap

A ship's fuel only falls (decision 0006). The spec lists refuelling under logistics, and gives a base a fuel stock, and neither exists. The soak of scenario 8 showed what that means: a transport lasts about eight round trips between Espiritu Santo and Lunga Point, its escort runs dry on day 38, and a force out of fuel is finished for the rest of the game.

## Decisions

1. **A base has a stock of fuel,** in tons, beside its supply. A scenario gives it. It starts at nothing unless the scenario says otherwise.
2. **A ship refuels by being in port at the start of a day.** At the start of each day, before the day's orders are taken in, every task force that is stopped at the hex of a base its own side holds refuels there. Stopped means at the centre of the hex and not under way: it arrived, or never left, or ran out of fuel there.
3. **Each ship afloat takes what it lacks,** up to its class's fuel capacity, from the base's stock. Forces are served in order of their number and ships within a force in order of theirs. When the stock runs short the ship being served takes what is left and the rest get nothing.
4. **Nobody orders it.** The staff refuels a force in port as a matter of course. In War in the Pacific the player chose a refuel option for each force (manual section 6.2.13.1); here there is nothing to choose yet.
5. **It takes no time and no berth.** A force that starts the day in port leaves that day with what it took. Loading or unloading cargo does not stop it.
6. **A force that ran out of fuel in a base's hex is under way again** once any of its ships has fuel: its state goes back to stopped, and an order can move it.
7. **Fuel for aircraft is not this stock.** Sorties draw on a base's supply (decision 0039).
8. **A scenario that gives its bases no fuel has no refuelling,** and runs as before.

## Why

- **A day is the grain.** Fuelling a force took hours, and the game's shortest step that a player plans by is the day. A rate in tons an hour would be a figure with no source yet and a queue to explain; a stock that can run short is the part that makes a player think.
- **In port at the start of the day** is a fact the log already shows: the force arrived, with an event, the day before. Nothing new has to be tracked to know who refuels.
- **Lowest number first** is the rule every other shortage in the game uses.

## What it costs

- A force can put in, refuel and sail in the same day at no cost in time.
- A base is never short of pumps, only of fuel.
- Nothing carries fuel to a base. A convoy carries supply (decision 0011). Until fuel is a cargo, a forward base has what its scenario gives it.

## Deferred

- Fuel as a cargo, and tankers. *(Nov 14, 2026: fuel as a cargo is built by decision 0046)*
- Refuelling at sea.
- A rate by port size, port damage, and a reserve a base keeps back.
- Refuelling at a friendly base while only passing through.
- Rearming.

## As built

- **Stopped** is a force that is not under way and is exactly at a hex centre. A force out of fuel between two hexes is adrift and gets nothing, even if the hex it is counted in holds a base.
- **A force that was out of fuel is freed by the first ship that takes fuel,** and the refuelling event says so. Its old route is dropped: it waits for a new order.
- **Where two bases of a side share a hex,** a ship takes from the lowest-numbered one that has fuel.
- **Scenario 7 was given fuel at Espiritu Santo and Henderson Field** so that the full slice records the new event. Its convoy fills up on day 5; nothing else in its log changes but the numbering and the hashes.
- **The soak has no base fuel,** so its check that ships never gain fuel still stands. Whether it should refuel is open.

## Rule card

| | |
| --- | --- |
| Reads | Each task force's position, whether it is stopped, its side; each ship's fuel and its class's capacity; each base's side, hex and fuel |
| Writes | A ship's fuel; a base's fuel; the state of a force that was out of fuel |
| Events | Ship refuelled, with the tons, what the ship now has and what the base has left |
| Randomness | None |
