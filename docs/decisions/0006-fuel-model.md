# 0006: Fuel model

Status: confirmed by the designer, Nov 10, 2026, as the placeholder model for step 1. Decision 3 was added in that review and is not built yet. *(Nov 14, 2026: built: `speed_multiplier` in `crates/sim-core/src/movement.rs`.)*

## The gap in the spec

The spec lists speed, endurance and fuel capacity for a ship class and asks scenario 1 to check fuel burn. It gives no formula.

## Decisions

1. A class has a cruise speed, a fuel capacity and an endurance: the distance a full load lasts at cruise speed.
2. Burn per nautical mile at cruise speed is capacity divided by endurance.
3. **At speeds above cruise, burn per mile is multiplied by (speed / cruise speed) squared. At cruise speed or below, the multiplier is 1.** No ship burns less per mile than it does at cruise.
4. A ship that is not moving burns nothing.
5. If any ship in a task force would run dry, the whole task force stops at the point that ship's fuel runs out, and is marked out of fuel.
6. Each ship's burn uses its own class figures, even when the task force steams at another ship's speed.

## Why the square

The power needed to drive a hull rises roughly with the cube of speed. Fuel per hour follows power, and fuel per mile is fuel per hour divided by speed, which leaves the square. It is a common first approximation and nothing more.

## Why the floor at 1

Without it, a destroyer escorting a 10-knot convoy would burn 39% of its cruise rate per mile. Real ships have a most economical speed, and burn per mile rises again below it. A floor of 1 is still generous to slow steaming, but it removes the reward for it. The floor is built in scenario 2's branch, the first scenario with a slow convoy and faster escorts. It does not change scenario 1, which steams at cruise speed.

## What replaces this, and when

- **Step 2: sourced fuel curves.** The square law is replaced per class by two or three sourced points, each a range at a stated speed, with interpolation between them.
- **Before scenario 8: burn at anchor and stopped at sea.** Ships burn fuel keeping steam up. This belongs to the end-of-day logistics phase and needs a new figure per class. It is left out of scenario 2 so that scenario adds one logistics rule, not two.
- **With the briefing work: a fuel warning.** An event that says a task force cannot reach its destination on the fuel aboard, raised at intake and each day. It feeds the briefing and the interrupt rules, and should make decision 5 rare.
- **Later: detaching a dry ship and refuelling at sea.** Both need orders that do not exist yet. Until then decision 5 is the backstop.

## Rule card

Added at the complexity review after scenario 5. The rule runs inside movement (decisions 0005 and 0022).

| | |
| --- | --- |
| Reads | A ship's fuel; its class's fuel capacity, endurance and cruise speed; the force's speed and the distance steamed |
| Writes | The ship's fuel; the force's status, when a ship runs dry |
| Events | A fuel event for each ship, caused by the steaming event; out of fuel |
| Randomness | None |
