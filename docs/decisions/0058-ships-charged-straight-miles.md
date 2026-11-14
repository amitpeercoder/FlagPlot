# 0058: Ships are charged straight-line miles

Status: confirmed. The designer approved the plan on Nov 12, 2026. It is the "D" of the choice he made when air distance was settled (decision 0040): straight-line air distance then, ships later.

Summary: Ships are charged straight-line miles: a step costs the miles it brings the force closer to its waypoint; 40 a hex only where land forces a detour

Standing: Confirmed

## The gap

A ship moves from hex centre to hex centre and is charged 40 miles for each step, in distance, time and fuel. Off the three axes of the grid a hex path is longer than the straight course: 11 in 100 on average on open water, 16 at most. On the routes the scenarios use it is 15 to 24 in 100 (Rabaul to Lunga is charged 640 miles for 556). Fuel endurance in the sources is in real miles, so ships are slower and shorter-legged than their own figures say. Step 2 compares ship movements with the record and cannot while this stands.

## Decisions

1. **A step is charged the miles it brings the force closer to its waypoint in a straight line.** At a hex centre H, steaming for waypoint W, with next hex N on the sea path: the step from H to N is charged `air(H, W) - air(N, W)` miles, where `air` is `Hex::air_distance_nm` (decision 0040), the one measure of straight distance in the game.
2. **Only where the way is clear.** That charge applies when the sea path from H to W has as many steps as there are hexes between them with no land (`Hex::distance`). When the sea path is longer, land forces a detour that is real, and the step is charged 40 miles as before. Once the force is past the land the rest of the way is clear and is charged by decision 1.
3. **So a clear passage costs its straight length.** The charges of the steps from H to W add up to exactly `air(H, W)`, whatever path the steps take. A step is never charged less than 1 mile.
4. **A route with waypoints is straight between each pair.** The waypoint a step is measured against is the one the force is steaming for.
5. **Time and fuel follow the miles charged,** by the rules that exist: hours are miles over speed, fuel is miles times the ship's burn a mile.
6. **Everything else about movement stands.** A force is in the hex whose centre is nearest. It enters the next hex when half the step's charge is steamed (20 miles of 40 before). Forces move in order of clock time and fight where they meet (decision 0022). The path is the map's shortest sea path (decision 0017).
7. **A force remembers what its step in progress is charged.** `TaskForce` gains `leg_nm`, set when a step begins and 40 for a force at a centre. A new route or a turn-back in the middle of a step keeps the step's charge: turned back, the force has `leg_nm` less its progress to go. It is state and is hashed.
8. **One function gives the charge** and everything that states a sea distance uses it: movement, the passage forecast, `measure`'s sea route, the briefing's range warning, and the viewer's measuring tool (checked against the engine by its fixture, decision 0053).
9. **The grid's axes are unchanged.** Along an axis a step brings the force 40 miles closer, so a route along one is charged as before.

## Why

- **It adds up to the one measure the game already has.** Aircraft and ships now agree on how far apart two hexes are.
- **A charge for each step can be explained and checked by hand.** "This step brought the force 34 miles closer." Scaling every step by one factor for the route gives numbers nobody can follow, and needs the route's length known before it is steamed.
- **Nothing else moves.** Hexes, meetings and paths are as they were; one number changes where it is used.

## What it costs

- **The step charges are uneven,** and they wobble by a mile because air distance is in whole miles. From (0,0) for (8,3) the steps cost 38, 39, 37, 37, 35, 34, 30, 24, 40, 40, 40, which add to 394.
- **The path is still an L and not a line.** A force bound north-east goes east and then north. It is charged the straight course but is drawn, sighted and met along the L. Making the path itself straighter is a separate matter.
- **An island on the straight line with a path of equal hex length beside it** is not a detour by decision 2, and the force is charged as if it sailed through. A few miles.
- **Every scenario with a ship off the axes gets new numbers,** scenario 1 among them, and every hand calculation that states a ship's miles, hours or fuel is redone.
- **The recorded game of the full slice** no longer replays to its transcript: its convoy arrives at another hour.

## Rule card

| | |
| --- | --- |
| Reads | The force's hex, its next hex and its waypoint; the map's sea distances |
| Writes | The force's `leg_nm`; through the rules that exist, its progress, its ships' fuel and the hours it steams |

## As built

Built Nov 12, 2026, stage 2 of the plan. What the code does, and the small choices the decision left open.

- **The functions.** `Map::step_charge_nm(field, from, next, waypoint)` is the charge of one step (decision 8): clear when `field[from]` equals `from.distance(waypoint)`, then `max(1, air(from, waypoint) - air(next, waypoint))`, else 40. `Map::sea_charged_nm_in(field, from, waypoint)` sums the steps of the sea path from `from` (its steps are walked, not assumed to be `air`), and `Map::sea_charged_nm(from, to)` is the same with the field worked out. `SideView::sea_charged_nm` and `sea_charged_nm_in` pass them on.
- **Callers.** Movement (`Mover::steam`, `distance_to_next_change`) and so the passage forecast, which builds the same mover; `TaskForce::reroute_centre` (the way ahead against the way back); `measure` sea route (`distance_nm`, with `steps` still the hex count); the briefing's cannot-reach-base warning (nearest base is now the nearest by charged miles); the map tool's `path` command; the viewer's `stepChargeNm` and `measureHexes`, checked against the engine by `viewer/measure_fixture.json` (version 2, which now lists each step's charge).
- **Hex-count thresholds that stay in hexes:** the belief merge distance, the surface-combat contact match, the bearing code's five-hex test, a search report's one-hex error and the leg's adjacency invariant. None of them is a ship's distance in miles.
- **State.** `TaskForce.leg_nm`, 40 at a centre, set when a step begins, copied to a force split off mid-leg, kept by a turn-back (progress becomes `leg_nm` less progress). Three invariants: progress is at least 0 and under `leg_nm`; `leg_nm` is 1 to 40; `leg_nm` is 40 at a centre. It is hashed after `progress_nm`, and `hash::STATE_VERSION` is `flagplot-state-v3` (decision 0057, 3). Saved games from before fail at day 1.
- **Events.** `TaskForceSteamed` still reports charged miles. Its breakdown gains one `step_charge_nm` factor for each step begun in the segment, in order; recording changes nothing. A step begun in one segment and finished in the next is listed in the segment that began it.
- **Replay.** The export's task force gains `leg_nm` next to `progress_nm`. The viewer rebuilds positions from the steaming events, so it needed nothing else, and the replay version stays 2 (an addition, decision 6 of 0057).
- **Choices made here.** The charge is computed from the hex the force is leaving, so a force that is clear stays clear for the rest of the path. The minimum of 1 mile is applied in the one function. Where the way is clear the sum of the charges is `air` exactly, and a test checks it on at least 300 pairs on the real map; where it is not, the sum is more than `air` and at most 40 a step, which also held for every detour pair tried. Unit tests that held the old rule as their oracle (`a_reroute_never_gains_distance`, `a_force_that_must_go_round_land_does_so_and_arrives`) now compute their oracle from the charge function.
- **Not rebuilt here.** The other scenarios' hand calculations. Their tests that stated the old miles fail and are listed with their numbers for the architect; the recorded games are not regenerated.
