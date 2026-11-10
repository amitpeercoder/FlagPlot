# 0005: Movement

Status: confirmed by the designer, Nov 10, 2026. Decisions 1 and 5 were changed in that review and the new rules are not built yet. See "Not built yet".

## The gap in the spec

The spec says a task force stores its hex plus progress toward the next one. It does not say when the task force changes hex, how it picks its path, or what happens when it is re-routed mid-hex.

## Decisions

1. **A task force is in the hex whose centre is nearest.** It stores the hex it left, the hex it is steaming toward, and the miles covered between the two centres. It changes hex at the midpoint, 20 nm.
2. **Task force speed is the speed of its slowest ship** at the current speed setting. "Cruise" uses each class's cruise speed. "Full" uses each class's top speed.
3. **Between waypoints, the task force takes greedy steps:** each step goes to the neighbour closest to the waypoint. Ties break in a fixed direction order. This always gives a shortest path over open water.
4. **There is no pathfinding until the real map arrives.** A route whose greedy path crosses a hex ships cannot enter is rejected at intake, and whoever issued it must add waypoints. A deterministic shortest-path search is added with the real map, at scenario 4 at the latest. Waypoints stay as the order format. The search fills in the legs between them.
5. **A re-routed task force finishes the step it is on.** The new route starts from the hex it is steaming toward. A re-route never gains distance. It can waste up to 40 nm when the new route doubles back.
6. **A task force that reaches its last waypoint stops** and is marked as arrived. It stays a task force. It does not dock or disband.

## What the review changed

- **Decision 1** was "the task force enters the next hex after covering the full 40 nm". A force 39 nm along was reported, searched for and fought one hex behind where it was.
- **Decision 5** was "a re-routed task force keeps its progress, whatever the new direction". That let a force gain up to 40 nm by re-routing.
- **Decision 4** gained its end date. It was open-ended.

WitP:AE turns speed into whole hexes per 12-hour phase (manual section 6.2.3), and has the player set routes by hand through waypoints (6.2.7). Flag Plot keeps the finer position, and will not ask the player or the staff to plot around islands.

## Not built yet

The code still follows the old decisions 1 and 5. The new rules change the movement state, the state hash and the hex positions scenario 1 expects. Its fuel figures and arrival time do not change.

They are built as one small pull request after scenario 2 and before scenario 3, with scenario 1's hand calculation updated in the same change. Nothing needs them until search exists.

## Not modelled yet

Weather, formation speed loss, damage slowing a ship, zigzagging, and any difference between day and night steaming.
