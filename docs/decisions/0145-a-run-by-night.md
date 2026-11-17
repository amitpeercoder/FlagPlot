# 0145: A run by night: step 5 of stage 3, first part

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 5 of its nine). Step 5 is written in two records: this one, a run timed to the dark; and the next, bombardment from the sea. It adds no rule to the engine: it times a run that the staff already orders. It is built after decision 0144. Built on Nov 17, 2026, with decision 0146: see "As built".

Summary: Step 5 of stage 3, first part: where an enemy base is near the place a run goes to, the staff times the run to arrive in the night, at cruising speed or at full, carries no more in each ship than the night's hours can land, and the force is gone at the morning; where no speed brings it there in the dark it goes by day and says so, or at preserve the force does not go

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings, and the earlier tests and games it changes, await the architect (As built)

## Why

A force that unloads by day off a place the enemy can reach from the air is found and struck there. The record's answer was to come in the dark and be gone by light. The rules allow it already: a force routed at the start of a day sails into the night pulse, a force that has landed its cargo steams on at the start of the next pulse, and a speed can be ordered. No staff uses them so. This record lifts the limit of stage 2 that a run is not timed to the dark (`docs/open-work.md`).

## When a run goes by night

1. **The runs it covers** are a supply run of "keep a base supplied" (decision 0107) and a lift of "take a place" (decisions 0142 and 0143).
2. **A run goes by night** at preserve the force and at balanced, where an enemy base on the side's map lies within 15 hexes of the place it goes to and not in that place's own hex. At press it goes as before. The 15 hexes are the placeholder of the stand doctrine (decision 0124). The risk is the intent's, read up the tree.
3. **A base in the place's own hex does not count,** because a landing at an enemy base is the enemy's base itself: what covers that landing is step 6.

## The timing

4. **The hour it would arrive** is worked by the engine's own forecast of the passage from the source, sailing at the first hour of a day, at cruising speed and then at full speed. A passage arrives in the night when its hours, less the whole days in them, are fewer than 12.
5. **The speed ordered** is the first of the two that arrives in the night with room for the load (item 7) and for which every ship has the fuel. Above cruising speed a ship burns its cruising rate times the square of the speed over its cruising speed (chapter 2, Fuel burn), so the fuel each ship needs is the engine's measure of the route out and home at the speed tried, in place of the run's measure at cruise. A speed a ship has not the fuel for is passed over, and the reason gives the two figures. Where it differs from the force's speed the staff orders it, on the morning the force sails, before the route. It is left as it is afterwards.
6. **The night's hours** are 12 less the hour of the night at which the force arrives.

## The night's limit

7. **No ship carries more than it can land in the night's hours:** the place's rate for a ship times those hours, the men by their weight and the supply together, at the rate the force's loading gives it where men are landed.
8. **A supply load is cut to it.** A load to a weight fills ships in ship order, so the load is cut until the first ship filled carries no more than its limit, and each ship after it likewise.
9. **A unit goes only in ships that can each land their part of it in the hours.** A unit that cannot is left for a later run, and the reason says so.
10. **The days of a run are reckoned as before,** at cruising speed.

## Where no speed arrives in the dark

11. **At balanced** the run goes by day at cruising speed, as before, and the reason says that neither speed brings it there in the night, with the two hours of arrival.
12. **At preserve the force** it does not sail. The staff gives no order for it and hands in the notice that it cannot meet the intent, on each morning it would have sailed.

## Gone by light

13. **Nothing is added.** A force that has landed its load in the night steams on at the start of the morning pulse by the rule as it is (decision 0096). It is at sea on its way home by day, as a force that stood for the night is.

## Reasons

14. **A reason for a run by night** gives the enemy base and its distance, the two hours of arrival, the speed chosen, the night's hours, each ship's load against its limit, and what was cut or left.

## What it leaves for later

- **Sailing at an hour inside a day,** so that a longer passage can end in the night: orders inside a day are not built.
- **A base known to hold aircraft,** and the reach of those aircraft, in place of 15 hexes from any base: it waits on what a side knows of an enemy field.
- **A load put ashore and left on the beach,** and a force that leaves with cargo aboard at the morning.
- **The first landing of an operation covered from the air:** step 6.

## Rule card

- **Reads:** the side's map of the enemy's bases, the forecast of a passage at two speeds and the fuel each burns, the port figures, the ships' room and the men aboard, the risk.
- **Writes:** nothing of its own: a speed order, and the weight of a load order.
- **Reuses:** the supply doctrine's run and the lifts of decisions 0142 and 0143; the speed order; the halt and the steaming on of decision 0096.

## What it costs

- **State:** none. The doctrine version rises. A game in which no run goes to a place within 15 hexes of an enemy base in another hex plays as before: scenarios 67, 68 and 69 do.
- **Earlier scenarios with a supply intent may change,** where an enemy base on the side's map lies within 15 hexes of the base supplied. The build lists each, and its new numbers go to the architect, not to the developer.
- **The rulebook's chapter 8** gains "A run by night" under the supply doctrine.

## The hand calculation

`tests/scenario_70_a_run_by_night.rs`.

## As built

Built on Nov 17, 2026, on the branch `stage3-step5`, with decision 0146. Nothing is tuned and no figure changed. Every number of the hand calculation that an event, the state or a reason carries is asserted exactly in `tests/scenario_70_a_run_by_night.rs` (15 tests), and no test of it is ignored: no mismatch was found. The times written in hours and minutes are the engine's own fixed-point values: the passage of 240 nm at 36 kn is 240 / 36 = 6.666666 hours, so the force halts at C at hour 126.666666 and 270.666666, has landed by 128.166666 and 272.166666, steams on at 132 and 276 and is home at 138.666666 and 282.666666; the night's hours are 12 - 6.666666 = 5.333334, and a ship's limit 20 x 5.333334 = 106.66668 t (4 x 5.333334 = 21.333336 t in the property "The night's limit").

### What was built

- **`crates/sim-staff/src/night.rs`** (new): the night's hours of a passage (items 4 and 6), the enemy base that sends a run by night (items 2 and 3), the two speeds tried by the engine's forecast with the fuel of the route out and home at each (item 5), the choice of speed and the weight a load to a weight may take within each ship's limit (items 7 and 8), and the words of item 14.
- **The supply run (`crates/sim-staff/src/doctrine.rs`).** On the morning a run is ordered, `night_load` (called in `serve` after the planner's evaluation) works the speed and cuts the load to the night's limit, the cut being one of the run's cuts; on the morning it sails, `night_dispatch` (in `dispatch`) orders the speed before the route where it differs from the force's, or, at preserve the force with no speed that arrives in the dark, does not send the run (the cannot-meet notice each morning, by the doctrine's own rule for a run held). The hand-in gained the step `Speed` (`handin.rs`); `RunPlan` the run's words by night (`planner.rs`).
- **The lift of "take a place" (`crates/sim-staff/src/take.rs`, `move_step` and `lift_night`).** The supply cut to the night's limit, the men aboard counted with it (items 7 and 8); a unit ashore taken aboard only where every ship it would fill could land its part in the night's hours, else left for a later run with a reason (item 9); at the passage the speed order, the fuel at that speed, and the cases of item 11 and 12, the notice once at preserve the force (the ruling on decision 0142's reading 8).
- **Scenario 70** (`data/scenarios/70_a_run_by_night.toml`, on scenario 27's map, on the figures it names, which are the frozen step 1 figures' namesakes; it needs none of its own) and its test.
- **The rulebook:** chapter 8, "A run by night" under the supply doctrine; chapter 2, Fuel burn, one line. Doctrine `supply-25` (then `supply-26` with decision 0146).

### Readings taken

1. **Where no speed has room for the whole load,** of the speeds that arrive in the night with the fuel the one with the longest night is taken, and the load is cut to its limit (items 5 and 8 read together).
2. **Ships in no one force** on the morning a run is ordered (loose ships, or a run whose ships lie in more than one force) cannot be forecast as a force: their passage is the route's miles over the slowest ship's speed and their fuel the burn rule of chapter 2, which is what the forecast gives on an open route. On the morning it sails the run is one force and the forecast is used.
3. **A ship the forecast runs dry** short of the route is given, as its need, the burn of the whole route at that speed by the burn rule, so that the reason gives it against what is aboard (the property "Not the fuel for full speed": 300 t against 303.75 t).
4. **The enemy base named** is the nearest that counts, the lowest number on a tie.
5. **At balanced with no speed in the dark** the run goes at cruising speed: where the force stands at another speed (full, from an earlier run by night), a speed order to cruise is given before the route.
6. **At preserve with no speed in the dark** a supply run is still ordered and loaded on its day (the hand calculation's "the run is loaded on day 5 and does not sail"); it is not sent on any morning after, and the notice rests on the day's reason, as for a run held for want of an escort.
7. **The cut counts as a cut of the run,** so where the reading the run checks stays below the floor the doctrine hands in that it cannot meet the intent in full, as for a cut by the ships' room.
8. **The supply run's route reason** still gives the passage at cruise and its arrival hour (the days are reckoned at cruise, item 10) before the words by night, which give the speed gone at and the night's hours.
9. **The lift:** the place's rate for a ship is multiplied by the factor of the force's loading for that place (`SideView::landing_rate_factor`) where the force lands men; a unit is left where no speed that arrives in the night with the fuel lets every ship it would fill land its part; where no speed arrives in the night at all no unit is left. The lift by night has no hand calculation of its own: no number of it is proven by a test.
10. **An escort of a run by night** is chosen as before, at the run's cruise timing; the escort keeps station at its charge's speed by the engine's rule.

### Earlier scenarios, tests and games that change

The rule changes what a staff does wherever a supply run goes to a base with an enemy base on the side's map within 15 hexes and not in its hex. None of the hand-calculation scenarios 1 to 69 changes its orders or events: every run baseline but one is unchanged beyond its hash lines (the state layout rises with decision 0146). What changes, all left for the architect, with no expected number touched:

- **Scenario 45, the soak ashore** (`crates/sim-cli/tests/run_baseline/45_soak_ashore.txt`, remade): its Allied runs to Lunga have Shortland within 15 hexes; they go by night, at full speed, and the night's limit of a ship working by lighter cuts their loads. The day's events differ from day 2 (230 events in place of 231). Two of its tests are ignored with the reason: `each_thing_that_must_happen_happens_and_leaves_the_run_going` (no run is ended by a hold any more) and `both_sides_still_have_aircraft_flying_in_the_last_30_days` (the Allied groups at Henderson Field no longer fly in the last 30 days). `crates/sim-cli/tests/batch.rs`, `refused_commands_are_counted_by_side_as_the_log_says_them`, is ignored too: the Allied side's refused commands in the soak are 1, not 2.
- **Four tests that add an enemy base the side "cannot see"** are ignored with the reason: the base they add is on the Allied map (a scenario's base is on the map unless it says not), within 15 hexes of the base supplied, so the run goes by night and the orders differ from the world without it. `tests/scenario_36_one_convoy.rs` and `tests/scenario_40_one_pool_two_hexes.rs` (`two_worlds_that_differ_only_in_what_the_allied_side_cannot_see_give_the_same_decisions`), `tests/scenario_42_interrupts_and_runs.rs` (`no_truth_two_worlds_the_side_cannot_tell_apart_give_the_same_runs_and_reports`) and `tests/scenario_56_an_escorted_run.rs` (`two_worlds_that_differ_only_in_what_the_allied_side_cannot_see_give_the_same_orders`).
- **The recorded game `second_proof_week_sonnet`** (remade by `scripts/regenerate.sh`): its Allied run of day 15 to Lunga, Shortland 8 hexes away, goes by night: 62.9 hours at cruise (by day) and 52.416666 at full speed, with 7.583334 hours of the night left; ship 19 lands by lighter at 20 t an hour, so the load of 1,282.05 t is cut to 151.66668 t and the staff hands in that it cannot meet the intent in full. The staff's answers from day 15 differ accordingly (`crates/sim-cli/tests/recorded_games/README.md`). The other recorded games differ only in their versions.

What this shows for the architect to weigh: the night's limit at a place worked by lighter cuts a large run to what one ship lands in a few hours, and a run of several days' passage is timed by the same rule.

