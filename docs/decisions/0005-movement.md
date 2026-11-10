# 0005: Movement

Status: needs review.

## The gap in the spec

The spec says a task force stores its hex plus progress toward the next one. It does not say when the task force changes hex, how it picks its path, or what happens when it is re-routed mid-hex.

## Decisions

1. **A task force enters the next hex after covering the full 40 nm** between hex centres. Until then it counts as being in the hex it left.
2. **Task force speed is the speed of its slowest ship** at the current speed setting. "Cruise" uses each class's cruise speed. "Full" uses each class's top speed.
3. **Between waypoints, the task force takes greedy steps:** each step goes to the neighbour closest to the waypoint. Ties break in a fixed direction order. This always gives a shortest path over open water.
4. **There is no pathfinding.** A route whose greedy path crosses a hex ships cannot enter is rejected at intake. The player, or later the staff, must add waypoints to go around land.
5. **A task force re-routed mid-hex keeps its progress.** The miles already steamed count toward the first hex of the new route, whatever its direction.
6. **A task force that reaches its last waypoint stops** and is marked as arrived. It stays a task force. It does not dock or disband.

## What needs review

- **Decision 1** affects detection and combat later. A task force 39 nm into a 40 nm step is still "in" the hex behind it. Switching hex at the midpoint would be more accurate. It does not matter until search exists (scenario 3).
- **Decision 4** will not survive the real map. The Solomons are full of land. Pathfinding is needed before scenario 2 if that scenario uses real geography.
- **Decision 5** lets a task force gain up to 40 nm in a new direction by re-routing. It is a small exploit. Fixing it properly means storing which hex the progress is toward.

## Not modelled yet

Weather, formation speed loss, damage slowing a ship, zigzagging, and any difference between day and night steaming.
