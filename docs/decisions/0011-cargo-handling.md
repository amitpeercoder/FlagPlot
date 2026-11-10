# 0011: Cargo handling at ports

Status: confirmed by the designer, Nov 10, 2026, as part of the scenario 2 plan. The port figures are placeholders.

## The gap in the spec

The spec asks scenario 2 to prove "loading, port capacity limits, base supply levels". It lists a port size of 0 to 10, a cargo capacity per ship class and a supply stock per base. It gives no rule for how fast cargo moves, what a port's capacity is, or what orders start it.

## Decisions

1. **One kind of cargo for now: supply, in tons.** A ship carries supply up to its class's cargo capacity. A base holds a supply stock.
2. **A port has berths and a handling rate per berth, both set by its size.** A ship on a berth moves cargo at the berth rate. A ship without a berth works by lighter, at the lighter rate, at the same time. A size 0 port has no berths, so every ship there uses lighters.
3. **There is no separate daily port limit.** A port's capacity is what its berths and lighters can move in the hours available. Nothing stops a task force from using a port because of its size.
4. **Berths are given out at the start of each loading phase,** to the ships that still have cargo to move, by task force ID and then ship ID. An assignment holds for that pulse. A berth freed part-way through a pulse is used from the next pulse.
5. **Cargo moves in the loading phase at the end of each pulse,** for the hours the task force was stopped at the port during that pulse. A task force that arrives eight hours into the night pulse works four hours of it. A task force that is under way moves no cargo.
6. **Two orders: load and unload.** Both are standing orders of one kind, "cargo", so one replaces the other.
   - **Load** fills every ship of the task force to capacity from the base's stock. It is rejected at intake unless the task force is stopped in a hex with a base of its own side. If the stock runs short, ships are filled in ship ID order and the load stops when the stock is gone.
   - **Unload** moves all supply aboard into the base's stock. It can be given at any time. It takes effect once the task force is stopped in a hex with a base of its own side.
7. **Unloading is never automatic.** A task force with a transport mission that arrives without an unload order does nothing.
8. **"Base supply levels" means the stock.** Nothing consumes supply yet.
9. **Scenario 2 includes loading at the origin.** The spec's row says "a convoy unloads" and lists loading under what it proves.

10. **Only a task force that is idle or has arrived works cargo.** One that is under way does not, and one that ran out of fuel does not, even if it counts as being in a base hex.
11. **Where a hex holds more than one base of a side,** cargo goes through the one with the lowest ID. Berths belong to a base, not to a hex.
12. **Within one loading phase, ships are worked in task force ID then ship ID order, and each change to the stock applies at once.** A task force that unloads can therefore supply one with a higher ID that loads at the same base in the same pulse.
13. **Finishing does not cancel the order.** A completed load or unload stays the standing cargo order until it is replaced.

## Why

- **One mechanism, not three.** WitP:AE limits cargo handling with a rate per ship per phase, a separate limit per port per day, and a gate on docking by tonnage (manual sections 6.3.1.1 and 6.3.3.2). A task force over the gate cannot dock and falls to a slow rate, so the player splits convoys by hand to fit. Berths give the same behaviour, a small port chokes a big convoy, from one rule with no cliff and nothing to split.
- **Hours at the port, not a points budget.** Movement already knows the hour a task force arrives. Using it keeps a knot of speed worth something at the far end.
- **An order for unloading** keeps every ton traceable to the order that moved it. The staff will issue it. The player never has to.

## Deferred

- Fuel, oil, troops and aircraft as cargo, and combat or amphibious loading.
- A load order for a stated tonnage.
- A single order that loads, sails and unloads.
- Port damage, support units, and limits on the size of ship a port can take.
- Supply consumption, spoilage and automatic movement of supply between bases. Consumption arrives by scenario 6.
- Handling interrupted by the enemy.
- Real handling rates. The port table is invented. Sourced figures come in step 2.

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| Cargo handling (loading phase) | Task force status, hex, arrival hour, cargo order; ship supply aboard; ship class cargo capacity; base stock; port size; port table | Ship supply aboard; base stock |
| Load order intake | Task force status and hex; base side and hex | Task force cargo order |
| Unload order intake | Nothing beyond ownership | Task force cargo order |

Movement writes the task force status and arrival hour that cargo handling reads. That is the only interaction with another rule.
