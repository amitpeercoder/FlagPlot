# 0007: Truth and belief in the type system

Status: settled.

## The gap in the spec

The spec says "the type system should make reading truth from a side's view impossible". It does not say how.

## Decision

- `TruthState` holds the real world. `World` owns it privately.
- `BeliefState` holds one side's contacts. Its fields are private. The only method that changes it is `apply_report`, which takes a `Report`.
- `World::side_view(side)` returns a `SideView`. It exposes that side's own ships, task forces and bases exactly, the side's contacts, the map and the reference data. It has no method that returns anything about the enemy except contacts.
- `World::umpire()` returns the truth. It is for tests, the command-line runner and hashing.

The rule: anything that plays the game is handed a `SideView` and never a `World`. That covers the staff planner, an agent and the UI.

## What this does and does not guarantee

- Code that holds only a `SideView` cannot read enemy truth. The compiler enforces it.
- Code that holds a `World` can call `umpire()`. Nothing but the rule above stops it. When the agent server is built, it should hold the `World` itself and pass only `SideView`s outward.
- "Belief is written only from reports" holds by construction, because `apply_report` is the only way in.

## Scaffold only

No rule generates reports yet, so belief states are empty. The detection and error model, contact ageing and contact merging arrive with scenario 3. `Contact` and `Report` hold a position, a ship count and a confidence. Course and composition by type are not there yet.
