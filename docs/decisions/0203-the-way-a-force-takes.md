# 0203: The way a force takes

Status: a plan by the architect, written on Nov 21, 2026 on the designer's word that a force's placement and the path it takes should weigh the risk, the enemy known and suspected, and fuel. Decision 0202 settled the placement; this is the path. To be read by a reviewer who has not seen it written, and amended on that reading, before its hand calculation is written and before any of it is built.

Summary: Every ship takes the shortest way by sea between the points of its route, whatever lies beside it. The staff chooses where a force goes and when, and never how. This has the staff choose the way too, for a carrier force it has stationed: by its risk it keeps the way clear of the circles of danger where it can, within what the fuel and a limit on the detour allow, and gives the engine that way as the points of a route. Convoys and supply runs are a second stage, since they would move closed proofs.

Standing: Planned on Nov 21, 2026; waits for its review.

## Why

What is so today:

- **The engine finds the shortest way by sea between the waypoints of a route** and nothing else: "the path is not stored… a route is still its waypoints" (decision 0017). It does not go round anything.
- **A route can be given point by point,** by a commander at the table or by the staff, and the engine follows the points.
- **The staff gives a destination and a speed.** In every doctrine the way is the engine's. The one exception is a route kept inside an area limit: there the staff searches step by step inside the area and adds the waypoints that hold the engine to that way (decision 0107).
- **The staff can already say where danger lies:** a circle of 15 hexes round each enemy base on its map and of 8 hexes round a fresh contact reported as a carrier (decision 0202), from its own picture. It uses them to choose a carrier's station, to time a supply run's arrival by night (0145) and to turn a lift back (0191). It never uses them to bend a way.
- **A detour costs fuel and time** through the miles the engine charges, and the staff can forecast both for a route it means to give.
- **Nearly every staff route in the closed proofs ends inside an enemy base's circle.** A change to the hexes a force passes through moves its meetings, its sightings and the test that turns a lift back.

## The rules

### Whose way is chosen

1. **Stage 1 is for a carrier force going to a station the staff chose** under decision 0202: to the station when the task is formed, back to it after closing on a contact, and to and from its home port for fuel.
2. **Every other route is as today:** a carrier force with a station of the commander's, a stand, a supply run, a lift, a take, a bombardment. Those are stage 2.

### What is weighed

3. **The circles of danger are decision 0202's,** read from the same picture on the morning the route is given: enemy bases to 15 hexes, fresh contacts reported as carriers to 8. A hex is in danger when it lies inside any circle.
4. **A way is weighed by two counts:** its length in steps by sea, and the number of its hexes that are in danger. The hex the force starts in is not counted; the hex it ends in is.

### The choice, by risk

5. **Press: the shortest way,** as today. Nothing is chosen.
6. **Balanced: of the ways no more than a quarter longer than the shortest,** the one with the fewest hexes in danger; then the shortest of those.
7. **Preserve: of the ways no more than twice as long as the shortest,** the one with the fewest hexes in danger; then the shortest of those.
8. **Fuel bounds both.** A way is taken only if every ship has the fuel for it and for the passage from its end to where the force fuels, times the margin of the risk, as decision 0202 weighs a station. Where the way chosen by rule 6 or 7 is out of fuel, the bound on length is brought down a step at a time until a way is in fuel; the shortest way is the last tried.
9. **Ties** between ways equal in both counts go to the engine's own order of directions at the first step where they part, so the same picture gives the same way.
10. **A quarter and twice** are figures of the doctrine, counted in steps and rounded up.

### How the way is given

11. **The staff gives the way as the waypoints of the route:** the fewest points that hold the engine's shortest path to the way chosen, by the routine that keeps a route inside an area today. Nothing in the engine changes.
12. **The speed is the doctrine's rule applied to the way chosen:** its charged miles, not the shortest way's.
13. **The way is chosen when the route is given and kept.** It is not chosen again each morning while the force is under way on it; a route given afresh, as after closing on a contact, is chosen afresh.
14. **The search on the way** (decision 0202, 23) follows the way chosen.

### What the staff says

15. **The reason for a route gives the way:** how many steps it is and how many the shortest is; how many of its hexes are in danger and of what, against how many on the shortest; and, where fuel shortened it, by how much.
16. **Where the shortest way is taken though it lies in danger,** at balanced or preserve, the reason says why: no way within the bound is clearer, or none is in fuel.

## Stage 2

17. **The same choice for every route the staff gives,** by putting it in the routine that builds all of them: supply runs, lifts and takes, stands, bombardments, and a carrier force at a station of the commander's.
18. **It moves closed proofs.** Almost every such route in the second proof, the stage 2b proof, the stage 3 proof, the Coral Sea and the five days of the Eastern Solomons ends inside a base's circle, and a way bent on its approach changes what the force meets and sights. Stage 2 is planned after stage 1 is played, with a list of the proofs that move and the designer's word on running them again.
19. **Long passages with their fuelling** (decision 0201, stage D) are planned with stage 2: a way and the fuellings on it are one plan.

## Rule card

Reads: what decision 0202's station choice reads, and the map's steps. Writes: the waypoints and the speed of a route the staff gives. Draws no random number. Adds no state.

## Games on file

Stage 1 touches only forces the staff stationed under decision 0202, and no game on file has one. The build plays every scenario and recorded game before and after; none may move. The doctrine's version rises.

## What it does not do

- It does not change the engine's pathfinding or how a commander's own route is followed.
- It does not know what an enemy base holds or how far its aircraft reach, so every base's circle is the same.
- It does not time a passage to cross a circle by night; the supply run's rule for arriving by night stays as it is and is not extended.
- It does not re-route a force already under way when a new contact appears; turning a lift back (0191) is unchanged.

## What it costs

- **A search over hexes and steps each time such a route is given.** The map has 1,600 hexes and a way is some tens of steps. The build measures a day with `sim-cli time` before and after.
- **Longer passages and later arrivals** for a cautious commander, and more fuel.
- **Rulebook:** chapter 8 (the carrier doctrine's routes, the routine that builds a route), the glossary ("way"), and decision 0017's line that a route is its waypoints gains a pointer here.

## Questions for the designer

1. **How far a cautious force may go round.** The plan allows a quarter longer at balanced and twice as long at preserve. They are judgements; the architect proposes them and expects to change them after games.
2. **Stage 2 will move closed proofs.** The architect proposes to build stage 1, play it, and bring the list of proofs that would move before stage 2 is built.
