# 0005: Movement

Status: confirmed by the designer, Nov 10, 2026. Decisions 1 and 5 were changed in that review.

## The gap in the spec

The spec says a task force stores its hex plus progress toward the next one. It does not say when the task force changes hex, how it picks its path, or what happens when it is re-routed mid-hex.

## Decisions

1. **A task force is in the hex whose centre is nearest.** It stores the hex it left, the hex it is steaming toward, and the miles covered between the two centres. It changes hex at the midpoint: at 20 nm or more it is in the hex ahead.
2. **Task force speed is the speed of its slowest ship** at the current speed setting. "Cruise" uses each class's cruise speed. "Full" uses each class's top speed.
3. **Between waypoints, the task force takes greedy steps:** each step goes to the neighbour closest to the waypoint. Ties break in a fixed direction order. This always gives a shortest path over open water.
4. **There is no pathfinding until the real map arrives.** A route whose greedy path crosses a hex ships cannot enter is rejected at intake, and whoever issued it must add waypoints. A deterministic shortest-path search is added with the real map, at scenario 4 at the latest. Waypoints stay as the order format. The search fills in the legs between them.
5. **A task force re-routed between two hex centres goes on to the hex ahead or turns back to the hex it left, whichever makes the shorter trip to the first waypoint of the new route.** A tie goes ahead. Turning back costs the miles already steamed from the hex it left. The new route then starts from the hex centre it reaches. A re-route never gains distance.
   - Between centres, a route to the hex ahead or to the hex just left is a real route, not an empty one. "Stop at the next hex" and "come back" are both accepted.
   - Turning back happens when the order is taken in. It records no event of its own: the order-accepted event is the record, and the next movement event cites the order.
   - A force turned back at exactly 20 nm changes position, from the hex ahead to the hex it left, without steaming. It is on the boundary, and the tie rule in decision 1 puts it in the hex it now steams toward.
6. **A task force that reaches its last waypoint stops** and is marked as arrived. It stays a task force. It does not dock or disband.

## What the review changed

- **Decision 1** was "the task force enters the next hex after covering the full 40 nm". A force 39 nm along was reported, searched for and fought one hex behind where it was.
- **Decision 5** was "a re-routed task force keeps its progress, whatever the new direction". That let a force gain up to 40 nm by re-routing. The review first replaced it with "finishes the step it is on". That was changed the same day, before it was built, because a force 1 nm out of port and ordered back would have steamed 39 nm on and 40 nm back. Turning back fixes that.
- **Decision 4** gained its end date. It was open-ended.

WitP:AE turns speed into whole hexes per 12-hour phase (manual section 6.2.3), and has the player set routes by hand through waypoints (6.2.7). Flag Plot keeps the finer position, and will not ask the player or the staff to plot around islands.

## Built

Decisions 1 and 5 were built in their own pull request after scenario 2. They changed the movement state, the state hash and the hexes scenarios 1 and 2 report. Fuel figures and arrival times did not change. Pathfinding (decision 4) is still to come.

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| Movement (movement phase) | Task force status, route, speed setting, hex left, hex ahead, progress; ship fuel; ship class speeds and fuel figures; map | Task force hex left, hex ahead, progress, route index, status, arrival hour; ship fuel |
| Route order intake | Task force hex left, hex ahead, progress; map | Task force route, status, hex left, hex ahead, progress (when it turns back) |
| Position (read by other rules) | Task force hex left, hex ahead, progress | Nothing |

Every other rule that asks where a task force is reads its position, never the stored hexes.

## Not modelled yet

Weather, formation speed loss, damage slowing a ship, zigzagging, and any difference between day and night steaming.
