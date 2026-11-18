# 0152: Forces with men aboard: what the 39 days ask of steps 7 and 8

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139), after reading the staff's setup of the 39 days against decisions 0142 to 0151. In that setup every man of both sides who is not on Guadalcanal or Tulagi on 7 August is aboard a transport force, at sea or in port, and each force may be used from its own day (decision 0140). Decisions 0149 and 0151 choose among units ashore beside ships, and decision 0143 sends after only what was left ashore. This record widens the three so that the staff can use what the campaign gives it. It adds no rule to the engine. Nothing is built. It is built after decision 0151, in its own pull request; decision 0150 waits until after the proof (decision 0151, "Amended after the review"). Built on Nov 18, 2026: see "As built".

Summary: A transport force with men aboard is a lift as it stands: "take a place" takes one as its first lift, sends each later one while the place is not taken and men are still wanted, and lands the working men aboard with the rest; a hold sends to its place a force with men aboard that no take needs; and the supply under a hold formed by "control an island" uses whatever transport forces lie empty at its source that morning

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings for ruling

## Why

The landings of 7 August begin with the men at sea. Every Japanese reinforcement of the 39 days is a run with its men aboard, usable from the day the record first has it. Without this record the staff of either side would find no base with a unit ashore beside a transport force, form nothing, and say it cannot meet the intent. That is a fault of the rules as written and not a finding about the staff.

## A lift as it stands

1. **A lift as it stands** is a transport force the staff may take (decision 0149, item 5) that has men aboard. Its units are the units aboard it, whole. Its source, for its way back and for the check of decision 0144, is its home port, or with none the nearest base of the side by the engine's sea path, the lower number on a tie. It is taken only when it is free: it has no route left to follow and is not landing, so a force under way on an order keeps it until it stops; and it needs no making ready wherever it lies, in port as at sea: a landing whose roster units have every man aboard the force and none ashore is ready as it stands. It takes supply at its source only where the staff may take that base (decision 0149, item 5); else it goes with what is aboard, as item 4 has it (added by the architect on Nov 18, 2026, on the build's finding).
2. **For "take a place" with no forces named** (decision 0149), the lifts as they stand with a fighting unit aboard come before any base: the first lift is the lowest-numbered such force, and the base rule of that decision's item 7 is used only where there is none. The men counted against the men wanted are its fighting men.
3. **Everyone aboard lands.** The unload order lands every man aboard, so working units aboard the lift land with it. They are not units of the operation for the judgement of decision 0143, and the reason names them.
4. **The supply that lands with the men** (decisions 0142, item 16, and 0144, item 13) is reckoned for every man aboard the lift. A force that does not lie at its source can take nothing more: it goes with what is aboard, and the reason gives what it is short.

## Later lifts

5. **Decision 0143's second lift is widened.** While the place is not taken, the first lift is no longer on its way there (it has landed its men, or is lost), and no estimate is held or the fit men who can attack are fewer than the men wanted, the staff sends, in this order: each lift as it stands with a fighting unit aboard that no "take a place" higher in priority has taken, in force order, on the first morning the side may use it; then the units left ashore at the source, as that decision has it.
6. **A later lift as it stands** is given a route to the place and back to its source, marked to act at the place, and an unload order, on that morning: it has nothing to load. Its units are added to the forces limit of the task "ready a landing", as decision 0143, item 12, has it. They are not set to attack before they sail: the judgement of each morning sets them.
7. **One force a morning to a place** is not asked: each lift is its own force and goes when it can.

5a. **Why the first lift must be in first** (amended on Nov 17, 2026, after the review of step 8). As first written, a side with no estimate sent every lift it had to its first place on the first morning, and a second island asked for the same day had nothing left to land: in the staff's setup of the 39 days the transports for Tulagi would have gone to Lunga. With the wait, the takes are served in priority order on the first morning, each with one lift, and a later lift goes where men are still wanted once the first is ashore.

## Men for a place already held

8. **A lift as it stands that no "take a place" in force can use,** because none is in force for a place not yet taken or its men are all working men, is sent by "hold a place": on a morning a hold's plan is released and its base is the side's, each such force, in force order, that no hold higher in priority has taken, is given a route to the base's hex and back to its source, marked to act there, and an unload order. Under "control an island" the holds are in the island's order, so the first place held has them.
9. **A run by night** (decision 0145) covers these runs and the later lifts, as it covers a lift.

## The supply under an island's holds

10. **The source,** where the commander names none, is chosen on the first morning there is a base that fits decision 0151, item 8, with "a transport force it may take lies stopped there with nothing aboard" for that item's test, and kept.
11. **The ships.** Each morning the staff sets the forces limit of each hold it formed to the source and the transport forces it may take that lie stopped at the source with nothing aboard, where that differs from the limit the hold has. So a convoy that may be used from a later day joins the pool on that day. The pool only grows: a force once named stays while it has a ship afloat, so that a ship serving a run, loaded or under way, stays inside the limit its orders are checked against; and a force a hold sends by item 8, or a later lift a take sends by item 6, is added by the staff's `set_limit` to the limit of the hold or of "ready a landing", since intake checks each order against the forces limits on its chain (added by the architect on Nov 18, 2026, on the build's finding).

## What it leaves for later

- **Men moved between places already held,** as the crossings from Tulagi to Guadalcanal were.
- **Which place a lift goes to,** chosen by what each place needs: the first take or hold in priority has it.
- **Where on an island a lift lands:** at the place itself, as before. The choice of a beach is stage 4's.

## Rule card

- **Reads:** the side's own transport forces with the men and supply aboard, their days and home ports; the intents in force and their priority.
- **Writes:** nothing of its own: `set_limit` on intents the staff formed, and route and unload orders.
- **Reuses:** decisions 0142 to 0151 whole.

## What it costs

- **State:** none. The doctrine version rises with decision 0151's.
- **The rulebook's chapter 8:** "Taking a place", "Holding a place" and "Controlling an island" each gain a paragraph.

## The hand calculation

`tests/scenario_77_forces_with_men_aboard.rs`.

## As built

Built on Nov 18, 2026, on the branch `stage3-step8b`. No state changes: the doctrine version rises to `supply-32` and nothing else (no state layout, saved game, agent interface or rules label). Nothing is tuned and no figure changed. Scenario 77 (`data/scenarios/77_forces_with_men_aboard.toml`, on scenario 27's map, with its own ground figures file `data/reference/ground_placeholder_scenario_77.toml`, the frozen step 1 ground figures with no figure of its own, frozen beside the others so that the file run by itself plays its hand calculation) plays the main case run by itself. `tests/scenario_77_forces_with_men_aboard.rs` has 24 tests: 22 pass, 2 are ignored with their reasons (Mismatches, below). The main game gives the hand calculation's tree, its ten orders on the days the table has them, none refused and no command refused, the hours, base 3 formed on day 1 and base 2 the side's at the end of day 2, and 15, 79 and 915 t at the end. Every property passes. Scenarios 67 to 74 and 76 hand in, day by day, exactly the orders and commands they handed in before (the test `scenarios_67_to_76_give_the_orders_they_gave`, against digests recorded on main at 4dec6155 before the change).

### Mismatches with the hand calculation

1. **The men and supply landed at B on days 4 and 5** (`the_men_and_supply_landed`, ignored). Written: ship 3 lands 160 men in the morning of day 4 and 140 in the afternoon, and ship 1 lands 40 t and 45 t on day 5, at the size 0 port's 20 t an hour. Built: 224 men and 76 men, and the 85 t faster. Unit 2, a working unit, works by default (decision 0099) at the port of base 2 from day 2, 400 a day, and the engine raises the unloading rate by 1 + work / `work_to_double_unloading`. The day-1 landing, the hours, the stocks at every day's end and the measure's days of supply agree.
2. **The measure** (`the_island_is_controlled_on_6_of_8_days`, ignored). Written: controlled on days 2 and 4 to 8, 6 of 8. Built: 5 of 8, days 4 to 8. The measure applies today's daily use to every past day (decision 0151, reading 9, known and left by its ruling 5): at the end of day 8 day 2's 25 t is read at 6.5 t a day, 3.8 days. Read on its own day each day agrees: 5 days at the end of day 2, 4 at the end of day 3.

### Readings taken

1. **A lift as it stands** is a force on the transport mission with men aboard a ship afloat that the staff may take (decision 0149, item 5: usable that day, not held, not named by an intent of another tree higher in priority).
2. **Free** (the architect's ruling): no route left to follow, not landing, not between hexes and not at the yard. A force that is not free is named with "is under way on its orders, which stand" or "is landing at".
3. **Ready as it stands** (the architect's ruling): the "ready a landing" whose roster units each have every man aboard its force and none ashore is ready on the morning it is formed wherever the force lies; the landing order with the roster is still given where the force has none, as for a force at sea.
4. **Its source** is its home port where that is a base of the side; else the base of the side with the fewest miles by `route_between` from where the force lies, the lower number on a tie. The supply is drawn from it only where the commander or his island named it, or the staff may take it (the architect's ruling); else the reason says the staff may not draw on it and what the lift is short.
5. **The first lift** is the lowest-numbered lift as it stands among the forces the take may take (inside what the commander or the island named) that is free, is not the lift of another take whose plan has not ended, was not sent by another intent that morning, and has aboard a fighting unit the staff may take. Its units are those fighting units, whole: the men wanted do not cut them. The source line of the reason lists every force with men aboard in force order and why each is or is not taken.
6. **Everyone aboard is fed:** the supply and the check that the place can be held are worked for the units chosen and every other unit with men aboard the force. The working units are named in the reason: "Also aboard task_force:N: ...; everyone aboard lands, and it is (they are) not of the operation."
7. **The first lift is no longer on its way** when it is not landing at the place and the place is not ahead on its route, or the force is lost. While the first force is on its way, with its first lift or a second lift of decision 0143, the later lifts as they stand wait, and the reason says so.
8. **The gate of item 5.** With no unit of the operation ashore at the place (the first lift has been there and none of its men is ashore, or it is lost), the fit men are none. The units left behind of decision 0143 are chosen as that decision has it, counting the fit men ashore and not the men of the later lifts.
9. **A later lift** has its force and its fighting units added to the forces limit of "ready a landing" by one `set_limit` that morning (the force too, the architect's ruling), and is given a route from where it lies to the place and on to its source, marked to act there, and the unload order, with nothing to load. Its units are given no posture order before they sail.
10. **A unit aboard another force** is not a unit of the first force's second lift of decision 0143.
11. **"No take can use it"** (item 8): a lift with a fighting unit aboard is the takes' whenever a take of any priority is in force with a plan not ended for a place not yet the side's; a lift with working units alone is a hold's.
12. **A hold sends** after its men and its supply and before its cover. A hold the commander made sends only a force inside the forces limits on its chain, and none with no limit; a hold the staff made has the force added to its limit by `set_limit` (the architect's ruling). Where the side has a force with men aboard and the hold sends none, one reason gives each force and why. At preserve, where neither speed brings a hold's lift there in the night, it does not sail and no notice is handed in.
13. **The source kept** (item 10): the source of a hold the staff formed under the island, while it is a base of the side, is the source of every later morning; "nothing aboard" is no supply, no cargo fuel and no man in any ship of the force.
14. **The pool** (item 11, the architect's ruling): each morning each hold the staff formed under the island is given the source and the forces its limit named before at that source that still have a ship afloat, with the transport forces the staff may take stopped there with nothing aboard; the `set_limit` is handed in only where that differs.
15. **The words.** Each morning a take or a hold weighs the forces with men aboard, its reason names each with what is aboard and why it is or is not sent, so a force before its day is named each morning with "task_force:N may be used from day D", and a held force with its hold.

The reason of a force with men aboard sent by a hold, as worked (its route order; the unload order's ends "This is the unload order the route acts on: the landing."): "{hold} holds {base}, which is the side's: {force} is a force with men aboard ({units aboard}) and no take is in force for a place not yet the side's, so the hold sends it to {base} as it stands, with {t} of supply aboard and nothing to load; everyone aboard lands. It sails today (day {d}, hour {h}): {nm} nm out at {kn} kn = {hours} hours, so it arrives at {hex} at hour {h2}; {nm2} nm back to the source {source}. Fuel: {each ship}.{escort words} This is the route order: out to the place and back to the source, acting at the place." Where a take is in force whose men are all of working units, "no take is in force for a place not yet the side's" reads "a take is in force, but its men are all of working units".

### Earlier tests, baselines and games that change

No expected number of an earlier test changed and no earlier test is ignored. `tests/scenario_65_a_day_from_which_a_force_may_be_used.rs`, whose check that no earlier scenario gives a unit a day walks every scenario file, leaves out scenario 77, which gives task force 2 its day. The two tests that compare every baseline with a run with no staff (`tests/scenario_06d1_pilots_and_replacements.rs`, `tests/scenario_06d2_base_supply.rs`) leave out scenario 77, as they leave out 76. Remade by `scripts/regenerate.sh`: no run baseline but scenario 77's two, which are new, changed (the side with no force with men aboard and no island hands in what it did); the agent goldens, the pinned hashes and the batch fixture are unchanged. Of the recorded games, each `save.json` changes its doctrine version, and `second_proof_week_sonnet` also gains 21 reasons, one each morning for each of three commanders' holds (Lunga and Tulagi, Allied; Rabaul, Japanese) that name the forces with men aboard as outside the hold's forces limit and send none, with its day hashes and, in its transcript, the event numbers and the `staff_orders` answers that list them. No order or command of any game changed.

### Files to read

- `crates/sim-staff/src/lifts.rs` (new: a lift as it stands, free, its source, the later-lift and hold surveys, `hold_lifts`).
- `crates/sim-staff/src/assemble.rs` (`first_lift_as_it_stands`), `take.rs` (`move_step` with its place, source and `LiftKind`; `supply` with the draw; `later_lifts_alone`, `send_later_lifts`, `on_its_way`; the later lifts in `ashore`), `ready.rs` (ready as it stands), `island.rs` (`island_source` kept, `pool_at`, `island_pools`), `air.rs` (the hold's call).
