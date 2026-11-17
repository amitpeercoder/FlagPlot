# 0148: Cover for an operation by carriers: step 6 of stage 3, second part

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 6 of its nine). It lets "take a place" form the task "operate a carrier force" (decision 0125) at a hex the staff chooses, and lets that force's bombers strike the enemy's base at the place. It lifts two limits of stage 2: the staff chooses where a carrier force stands, and a carrier strikes a base (`docs/open-work.md`). It adds no rule to the engine. It is built after decision 0147. Built on Nov 17, 2026, with decision 0147: see "As built".

Summary: Step 6 of stage 3, second part: under "take a place" the staff gives each carrier force its forces limit names the task of operating from the first hex on its way from which its bombers reach the place; there it searches and flies its patrol as before, strikes ships first, and on a morning with no ship to strike sends its bombers at the enemy's base at the place by the raid's rules, until the place is taken, when the force is sent home

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Built, with three points of the hand calculation the build does not give, for the architect (As built)

## Why

A landing goes in under what the carriers can do for it: they find and strike the ships that come for the transports, and they strike the place before the men land. Stage 2 gave a carrier force its task, but the staff forms it only where the force already lies, and its bombers strike ships and nothing else.

## The task formed

1. **Under a "take a place" whose plan is released,** the staff forms "operate a carrier force" for each task force on the carrier mission, with a carrier afloat that has an air group aboard, that the nearest forces limit on the intent's chain names, in task force order, where no carrier task for that force is in force anywhere on the side. It is the forming of decision 0125, item 2, with a third parent.
2. **The hex** is the covering hex: the first hex along the shortest sea route from the hex the force is in that morning to the place's hex that is within the reach of the place, the reach being the least strike radius, at normal reach, among the bomber groups aboard (decision 0130, items 12 and 13, with the place for the contact). With no bomber group aboard, or the force already within the reach, the hex the force is in. With no sea route, none is formed and the reason says so.
3. **The force is then the task's,** by the carrier doctrine as it is: it is routed to its hex at the speed of that doctrine, flies its patrol, searches, rests its groups, weighs its fuel and closes on a contact. The enemy's bases on the side's map are among the threats of its search already, and the base at the place is one.

## The strike on the place

4. **Ships first.** A bomber group that the carrier doctrine sets to strike ships is left to it on a morning the side holds a fresh contact of ships within the group's reach of the force's hex (decision 0130, item 12: no more than 3 pulses old).
5. **Else the base.** On a morning with none, while the task stands under a "take a place" that is not met, the enemy holds a base on the side's map in the place's hex, and that hex is within the group's reach of the hex the force is in, the group is set to the base strike mission at that base, the lowest-numbered there, by the raid's rules 1 to 4 (decision 0120): which bombers go, whether the raid goes at the task's risk, what the last raid cost, and the orders. A fighter group aboard is the force's cover and is given no order by this: its escorts are what its patrol share leaves.
6. **Back to ships.** On a morning a fresh contact is in reach again, the group is set back to the naval strike mission. When the base changes side the engine clears the mission (chapter 7, A base that changes side) and the staff gives no order for it.
7. **A patrol over the beach is not flown.** A fighter's patrol covers its own force. Cover of the transports and of the men ashore is by what the carrier strikes and finds.

## When the place is taken

8. **On the first morning the intent is met** the force is routed to its home port at cruising speed, where it has one, and the task is given no order after: its groups are left as they are. With no home port it is given no order, and the reason says the commander must move it.

## Reasons

9. **The reason of the forming** gives the place, the force's hex, the reach and the group that sets it, the hex chosen and its distance from the force and from the place. **The reason of a base strike** gives that no fresh contact of ships is in reach, the base and its distance against the reach, and the raid's figures at the risk.

## What it leaves for later

- **Choosing the day it sails** so that it is on station no earlier than it must be: stage 4, with the choice of the day.
- **A patrol over another force or over a beach,** which the air rules do not have.
- **A covering hex chosen against the enemy's bases and searches,** not only by the bombers' reach.
- **Carriers under "hold a place" striking a base,** and a strike on a base timed with a bombardment.

## Rule card

- **Reads:** the side's own carrier forces and air groups, the map and its sea routes, the side's map of the enemy's bases, its contacts of ships, the aircraft types' reach, the raid's table.
- **Writes:** nothing of its own: `form_intent`, and mission and route orders.
- **Reuses:** the carrier doctrine whole (0125, 0130, 0131); the raid's rules 1 to 4 and the base strike mission (0120); the closing rule's reach and first hex (0130).

## What it costs

- **State:** none. The doctrine version rises. A game with no "take a place" that names a carrier force plays as before.
- **The rulebook's chapter 8:** "A task force as an addressee" gains the third parent, and "Operating a carrier force" gains "Under take a place".

## The hand calculation

`tests/scenario_73_carrier_cover.rs`.

## As built

Built on Nov 17, 2026, on the branch `stage3-step6`, with decision 0147. Nothing is tuned and no figure changed. The test of scenario 73 (`tests/scenario_73_carrier_cover.rs`) has 17 tests: 16 pass and one is ignored (below). The main game gives what the hand calculation states: the landing's days, the covering hex (4,0), 160 nm from A and 240 nm from B, the carrier's 8, 2 and 1 orders and the transports' 8, nineteen in all, six staff commands, group 3 launching at base 2 on days 3 to 6 and on no other, no aircraft lost and no order refused, and 2,946.666667 t aboard on the morning of day 3.

### Mismatches with the hand calculation

1. **The arc of day 2.** The hand calculation has 060 to 120; the build gives 050 to 120, by the arc rule as it is (`crates/sim-staff/src/air.rs`, `arc_for`, decision 0115, 9): the narrowest ten-degree arc that holds a threat on 090 is taken from the lowest start, 080 to 090, and widened by 30 degrees at each end. Scenario 55's arc follows the same rule. The architect ruled 050 to 120 right and restated the doc comment; the day-2 orders are asserted whole with it, and `the_arc_of_day_2_is_050_to_120` passes.
2. **The fuel the passage home needs.** 53.333333 x 1.2 is 63.9999996, which the engine's fixed point cuts to 63.999999 t; the hand calculation writes 64 t. The test asserts the engine's 63.999999.
3. **The property "Preserve the force"** cannot be played as written once decision 0147 is built: at preserve the force the plan holds no count of the enemy at B, and no air group of the forces limit can look (item 19 of 0147), so plan 1 is never released, no carrier task is formed, and there is no reason of day 3 giving 6 against 12. `preserve_the_force_group_3_holds_back_a_third_and_no_raid_goes` is ignored with that reason. Without decision 0147 it passed as written. A scout group at base 1 cannot meet the need here: the scout's search range is 300 nm, B is 400 nm from A, and the scenario's ground figures give no reconnaissance figures.

### What was built

- **Intake (`crates/sim-core/src/command.rs`, `may_stand_under`):** "operate a carrier force" may stand under "take a place". No state is added.
- **`crates/sim-staff/src/cover.rs` (new):** `take_carriers` forms the tasks under each released "take a place" and serves each carrier task under one, after the landing's morning and the bombard tasks (`air.rs`); `form_cover_tasks` (items 1, 2 and 9: the reach the least normal strike radius of the bomber groups aboard, the hex by the closing rule's `close_to` along the sea route to the place); `cover_strike` (items 4 to 6, by the raid's `decide` at the task's risk); `send_home` (item 8).
- **`crates/sim-staff/src/carrier.rs`:** `carrier_forces` leaves the tasks under "take a place" to `take_carriers`; `serve_carrier` calls `cover_strike` for a task under one, and the raid's orders replace the table's mission and held-share orders of the groups it sends. `raid.rs`: `last_raid_of` is the crate's.
- **Scenario 73** (`data/scenarios/73_carrier_cover.toml`, on its own placeholder map `data/map/placeholder_carrier_cover.toml` and ground figures `data/reference/ground_placeholder_scenario_73.toml`, frozen beside the others in `data/reference/step1/`).
- **The rulebook:** chapter 8, "A task force as an addressee" (the third parent), "Operating a carrier force", "Under take a place", and the staff's morning; the glossary ("covering hex"). Doctrine `supply-29` (with decision 0147).

### Readings taken

1. **The reach** of item 2 is the least normal strike radius over the bomber groups aboard that may be used, able to fly or not.
2. **A bomber group is weighed for the base** only where the carrier doctrine's table makes it a striker, no fresh contact of ships is within its own reach of the force's hex, and the base is within its own reach.
3. **Where the raid does not go** no group is set to the base strike and the table's orders stand; a group still on the base strike is set back to the naval strike by the table; the reason gives the raid's figures ("can send 6 between them, against the 12").
4. **The raid's orders replace** only the mission and the held share of the groups it sends; their sizing, contact age, target list and reach orders stay the table's.
5. **"The first morning the intent is met"** is the morning the staff marks the plan complete; a force already stopped at or routed to its home port, or held, is given no order.
6. **A carrier task the commander states** under "take a place" is served the same way.
7. **Only a "take a place" parent** chooses the covering hex; a carrier task under a hold or a deny is formed where the force lies, as before.
8. **The property "Ships first"** is played with a stationary Japanese force of two placeholder cruisers at (6,0) that the Allied search finds on day 3 and that sails away on day 3: a scenario can give a contact only at the start of a game. Group 3 keeps the naval strike on days 3 and 4 with no base strike order, and is set to the base strike on day 5, as the property has it.

### Earlier tests touched

No expected number changed. The two tests that compare every run baseline with a run that has no staff (`tests/scenario_06d1_pilots_and_replacements.rs`, `tests/scenario_06d2_base_supply.rs`) leave out scenario 73, as they leave out every scenario played by the staff. No earlier baseline, game or test changed for this decision.

### Files to read

- `crates/sim-staff/src/cover.rs` (`take_carriers` 62, `form_cover_tasks` 123, `cover_strike` 313, `send_home` 538); `crates/sim-staff/src/carrier.rs` from line 1288 (the reach of each group and the call of `cover_strike`); `crates/sim-staff/src/air.rs:520`.
