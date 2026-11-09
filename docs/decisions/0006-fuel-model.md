# 0006: Fuel model

Status: needs review. This is a placeholder model.

## The gap in the spec

The spec lists speed, endurance and fuel capacity for a ship class and asks scenario 1 to check fuel burn. It gives no formula.

## Decision

- A class has a cruise speed, a fuel capacity and an endurance: the distance a full load lasts at cruise speed.
- Burn per nautical mile at cruise speed is capacity divided by endurance.
- At other speeds, burn per mile is multiplied by (speed / cruise speed) squared.
- A ship that is not moving burns nothing.
- If any ship in a task force would run dry, the whole task force stops at the point that ship's fuel runs out, and is marked out of fuel.
- Each ship's burn uses its own class figures, even when the task force steams at another ship's speed.

## Why the square

The power needed to drive a hull rises roughly with the cube of speed. Fuel per hour follows power, and fuel per mile is fuel per hour divided by speed, which leaves the square. It is a common first approximation and nothing more.

## What needs review

- The square law is a guess. Real fuel curves come from sources in step 2, and may need a table per class instead of a formula.
- Ships burn fuel at anchor and when stopped at sea. That belongs to the logistics phase, which is not built yet.
- A task force slower than a ship's own cruise speed makes that ship burn less per mile under this formula. Real ships have a most economical speed, below which burn per mile rises again.
- Stopping the whole task force when one ship runs dry is crude. Detaching the dry ship, or refuelling at sea, are real options that need orders that do not exist yet.
