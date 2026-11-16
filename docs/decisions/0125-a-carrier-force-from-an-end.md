# 0125: A carrier force played from an end

Status: settled by the architect on Nov 16, 2026, inside the plan for step 3, stage 2 (decision 0113, doctrines 4, 5, 8 and 10, and step 4 of its seven), written by level. It is the third part of step 4, after decisions 0123 and 0124, and the last doctrine the second proof needs. It adds no rule of the engine. Built on Nov 16, 2026 ("As built" below).

Summary: The task "operate a carrier force from a hex", addressed to a task force with a carrier in it: at the force's level the staff keeps it at its hex, flies its patrol by the risk level, searches toward what the side knows and the waters it is to deny, sets its bombers' standing orders as a base's are set, rests its groups, and takes it home when its fuel says so

Standing: Settled by the architect, Nov 16, 2026, inside decisions 0113, 0120, 0123 and 0124. Built

## Why

The second proof of the stage is a carrier battle played from ends by both sides. A base's air has its doctrines (decisions 0115, 0120 and 0123). A carrier's groups have none, and a carrier force has the same three needs at once: to defend itself, to find the enemy and to strike him, with one deck of aircraft and a fuel tank that sets how long it can stay.

## The task

1. **A kind of intent: operate a carrier force.** It names a task force of the side and a hex. It is addressed to the force (decision 0124, 6). It may use the force's ships and the air groups aboard them. It may stand under "deny an area" and under "hold a place", and a commander may state it himself as a root.
2. **Forming it, at the level above.** Under a deny or a hold intent whose forces limit names task forces, a task is formed for each named force that is on the carrier mission and has a carrier afloat with an air group aboard, in task force number order, where none for that force is in force under it. *Amended by the architect on Nov 16, 2026:* where no carrier task for that force is in force at all, whoever made it and wherever it stands; the staff's reason names the one that stands. The hex it names is the hex the force is in that morning: the staff does not choose where a carrier force stands. A commander who wants it elsewhere states the task himself, or moves the force.

## The doctrine, at the force's level

Each morning, for a task whose force is not held, over the air groups aboard its ships that are not held, in group number order.

3. **Fighters fly the patrol,** with the patrol share the risk gives (three quarters, a half, a quarter), raised by a quarter and not above 1 on a morning when the force was attacked from the air on any of the three days before. The rest are escorts.
4. **Who searches.** A group that can search and cannot attack a ship searches. Where the force has none, the lowest numbered group that can do both searches and the others strike. A search is flown at the type's range over the arc of decision 0115's rule 9, taken from the force's hex, with these threats: the enemy's bases on the side's map, the contacts it holds, and, under a deny intent, the hexes of its waters.
5. **The rest strike ships,** by decision 0123's rule 7 and its table, with strike sizing on by its rule 9: a carrier under this task has more to hit than one force.
6. **Rest.** A group rests for a day when its fatigue in the morning is above the level its risk gives (30, 45, 60). The last fighter group that can fly is never rested, and neither is the last group that searches.
7. **Its place.** Where the force is not in the task's hex and not on its way home, it is routed there at cruising speed. In the hex it is given no route.
8. **Fuel.** The fuel the force needs is the passage from where it is to its home port, by the engine's own measure, times the margin its risk gives (decision 0124's table), for the ship of the force that has the least against its need. On a morning a ship has less, the force is routed home, and the reason gives the figures. At home it fuels, by decision 0112, and on the first morning every ship has what the passage to the task's hex and home again needs, with the margin, it is routed back. Its home port is the base it was last stopped at, or the one an order set.
9. **Readiness and rules of engagement:** battle readiness while it is at its hex; the stance avoid, since a carrier force does not seek a gun action; the break-off level of the risk table.
10. **When it cannot.** On a morning the force has no carrier afloat, or no air group able to fly, the staff hands in that it cannot meet the intent.
11. **Reasons** carry each group's fatigue against the level, the share and what raised it, the threats and the arc, the fuel aboard against the fuel needed, and the level.

## What it leaves out

- **Where a carrier force should be.** The staff keeps it where it was put. Choosing a station from the enemy's reach and the task is the trial's and the operation's, in stages 3 and 4.
- **Keeping scarce ships out of the enemy's air reach,** which decision 0113's risk table has. It needs a belief about that reach, and waits for stage 2b.
- **Closing the range** on a contact the bombers cannot reach, and opening it after a strike.
- **Fuelling at sea.**
- **A raid on a base from a carrier:** "neutralise a base" tasks bases only (decision 0120), until a need.

## What it costs

- **State:** one kind of intent. The state version rises; the rules label does not.
- **The staff:** one doctrine at the force's level, reusing the cover, strike and search pieces, and the forming rule in the deny and hold doctrines.
- **The rulebook's chapter 8** gains "Operating a carrier force"; the glossary gains nothing new.

## The hand calculation

`tests/scenario_55_a_carrier_force_from_an_end.rs`: one carrier with a fighter group, a scout group and a dive bomber group, three contacts in reach and no enemy unit, over 2 days at press: every order, the patrol, the arc of each day and every launch.

## As built

Built on Nov 16, 2026, with decision 0124, in one step (the versions are that record's). The doctrine is `crates/sim-staff/src/carrier.rs`; the decision for the groups is a pure function (`carrier::decide`, with `roles` and `patrol_share`), which the properties "in a view built for it" test. Readings the build took:

- **The kind** is `operate_carriers`. Its forming is decision 0124's rule 8 (`form_force_tasks`): a force on the carrier mission with a carrier afloat and an air group aboard one of its carriers; the hex is the force's position that morning.
- **Attacked from the air** is read from the side's own events: an air attack on a ship of the force (`AirAttackAgainst`) on any of the three days before.
- **Rest.** The last fighter group that can fly and the last searching group that can fly are never rested, by the cover's rule (most tired first, then by number). A striking group rests above the level with no exception, by decision 0123's table.
- **Who searches** uses the engine's own tests: a group can search when it is not a fighter and its type has a search range, and can attack a ship when its type carries a weapon (the strike doctrine's bomber).
- **The force.** At its hex: battle readiness, no route. Not at its hex, not on its way home and not at home: routed to the hex at cruise (a speed order first where it is not at cruise), and given no readiness order. On its way home (under way or halted with its home port's hex its last waypoint): no route. At home, not at its hex: routed back when every ship has the fuel for the passage to the hex and home at cruise with the margin, else it stays and fuels. Elsewhere, with a ship short of the passage home (at the speed it has) with the margin: routed home, at the speed it has, with no readiness order.
- **The home port** is the force's own; else, stopped at a base of the side, that base. No order sets it ("or the one an order set" is read as the commander's order). With none, the fuel is not weighed and the reason says so.
- **When it cannot** (no carrier afloat, or no aircraft able to fly in any group aboard), the notice is handed in and no order is given that morning.
- **The order of the orders:** readiness, rules of engagement, speed, route, then each group in number order, its mission first.

**What it was checked against.** Scenario 55 (`data/scenarios/55_a_carrier_force_from_an_end.toml`, on the frozen figures with scenario 53's table) came out as the hand calculation has it: the tree, the ten orders of day 1 and the one of day 2, the levels (the sizing order's at `air_group:3`), the arcs 060 to 300 and 110 to 180 at 300 nm, the two strikes of 10 at contacts 2 and 3, the patrol of 3, no fuel burnt. So did the properties of balanced, no scout, a commander's hex, a raid raising the patrol, nothing to fly, a surface force and a carrier force named together (force 2 is given its stand orders: home port, rules of engagement, battle readiness, full speed to (0,5) and the route; 15 orders on day 1), the held force and group, the same start and two worlds.

**One mismatch, ruled.** "Fuel takes her home" first gave the need as 66.666666 x 1.2 = 80 t. The code multiplies the engine's burn, 66.666666 t, by the margin 1.2 in fixed point and keeps 79.999999 t; its reason reads "79 t aboard against 79.999999 t needed", and the routing was as the calculation had it (79 t goes home, 81 t stays). **Restated by the architect after the build (Nov 16, 2026):** the 80 was a rounding slip; the code stands and the need is not rounded. The property now reads 79 against 79.999999, and its test asserts that, the routing home at 79 t and the stay at 81 t; none is ignored.

**One task a force (Nov 16, 2026).** The architect ruled, on what the loading check of the second proof showed (decision 0128, "The intents as stated", reading 2), that rule 2 forms no carrier task for a force that already has one in force, whoever made it and wherever it stands: "under it" let the commander's "operate task force 3 from (8,-11)" and the staff's own task at the force's hex stand together, the one routing the force and the other setting it to battle readiness "at its hex", and every group of the force was ordered twice. The forming rule is decision 0124's and is mended there (`form_force_tasks`); the doctrine is `supply-14`. Scenario 55 gains the property "One task a force": with the commander's "operate task force 1 from (0,0)" as intent 2 beside the deny end, no task is formed under intent 1, the reason names intent 2, and day 1 has 8 orders, each given once and serving intent 2 at balanced risk. It passes.
