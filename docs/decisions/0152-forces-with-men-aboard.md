# 0152: Forces with men aboard: what the 39 days ask of steps 7 and 8

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139), after reading the staff's setup of the 39 days against decisions 0142 to 0151. In that setup every man of both sides who is not on Guadalcanal or Tulagi on 7 August is aboard a transport force, at sea or in port, and each force may be used from its own day (decision 0140). Decisions 0149 and 0151 choose among units ashore beside ships, and decision 0143 sends after only what was left ashore. This record widens the three so that the staff can use what the campaign gives it. It adds no rule to the engine. Nothing is built. It is built after decision 0151, in its own pull request; decision 0150 waits until after the proof (decision 0151, "Amended after the review"). Built on Nov 18, 2026: see "As built".

Summary: A transport force with men aboard is a lift as it stands: "take a place" takes one as its first lift, sends each later one while the place is not taken and men are still wanted, and lands the working men aboard with the rest; a hold sends to its place a force with men aboard that no take needs; and the supply under a hold formed by "control an island" uses whatever transport forces lie empty at its source that morning

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Built, with item 5b and a lost first lift mended on the loading check; its readings ruled (The architect's rulings)

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

5b. **A higher take's later lift comes before a lower take's first** (added by the architect on Nov 18, 2026, on the loading check of the proof; "Amended on the loading check", below). On a morning, the takes are served in priority order for every lift as it stands: a take higher in priority that may send a later lift by item 5 is given each free lift it may take before any take lower in priority chooses its first lift from what is left. A lower take forms nothing that morning from a lift a higher take sends.

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

## The architect's rulings

Made on Nov 18, 2026, on reading the build.

1. **Readings 1 to 15 are accepted as built.** Five were ruled during the build, on the team's findings: a lift as it stands is ready wherever it lies; it is free, with no route left; it draws supply only at a base the staff may use; the pool of supply ships only grows; and a force a hold or a take sends is added to that intent's limit.
2. **The landing of days 4 and 5 in scenario 77 was the architect's error and is restated;** the code stands. A working unit ashore works on the port by default (decision 0099), so the men of day 4 land at 28 t an hour and the supply of day 5 at 34 t an hour.
3. **The measure of an island counted a day on which the enemy still held the airfield,** because a base formed by the landing stood in the hex. Mended in this pull request: a place that names a base is the side's when that base is, on each day by the base's own day-end event. Decision 0151's item 10, the rulebook and the glossary say so.
4. **The measure still keeps no record,** so it reads every past day at today's use and can count a day and later not count it (scenario 77: day 2 counts when read on days 2 and 3 and not from day 4, when more men are ashore). Scenario 77's hand calculation is restated to say exactly this: 5 of 8 days, with the reading of each day. A measure that keeps its days is state, and is built with the interface's work.
5. **Known and left:** a commander's own hold with no forces limit sends no force with men aboard (reading 12), and says so each morning, which adds 21 lines of reasons to one recorded game.
6. **Item 5b and the lost first lift, on reading their build (Nov 18, 2026).** Readings 1 to 6 of "As built: item 5b" are accepted. A force with no ship afloat is lost for the wait of item 5 and for the lifts set aside, and is left as it was where the staff judges the men already ashore.

## Amended on the loading check

On Nov 18, 2026 the loading check of the proof (decision 0153: arrangement (a), seed 1) showed the Japanese side, which asked for Guadalcanal first and Tulagi second, landing its first lift at Lunga and every lift after it at Tulagi. Two faults of this record as built lay behind it, and both are mended in one pull request before the judged run:

1. **A first lift with no ship afloat was still read as on its way.** On seed 1 the first lift landed its men at Lunga and its one ship was then sunk. The engine keeps a force whose ships are all sunk, halted with its route as it stood, and the staff read that as a lift still on its way: by item 5 the first island's take then waited for it every morning, and the second island's take chose each new force. This was the cause on seed 1. A first lift with no ship afloat is lost, as reading 7 meant.
2. **The order of the morning's work.** The architect first took this for the cause, and it was not the cause on seed 1, but it is a fault: the tree of a lower island was formed before a higher take weighed its later lifts, so a lower take could choose as its first lift a force the higher take was about to send. Item 5b states the order, and scenario 77's property "The higher take first" fails on the code as it was.

## As built: item 5b

Built on Nov 18, 2026, on the branch `stage3-lift-order`. No state, engine or event change: the doctrine version rises to `supply-34` and nothing else (no state layout, saved game, agent interface or rules label). Nothing is tuned and no figure changed. `tests/scenario_77_forces_with_men_aboard.rs` gains the test `the_higher_take_first`, asserting the property "The higher take first" and no more; it fails on the code before the change (the take of base 4 forms a landing of task force 2 at C on day 4) and passes after. Every other test of scenario 77, "Two islands" among them, passes unchanged, and scenarios 67 to 76 hand in, day by day, the orders and commands they handed in before (`scenarios_67_to_76_give_the_orders_they_gave`). No expected number changed and nothing is ignored.

### Readings taken

1. **What a higher take sends that morning** is read for it right after its own preparing (so after any choice made again), before any take lower in priority prepares: its plan released, its landing formed, and its first lift lost, or with a unit of the operation ashore at the place, the first lift not on its way and the fit men that can attack fewer than the men wanted or no estimate held, or been to the place with none of its men ashore there; with no man of the operation ashore, a take whose men wanted are none sends nothing. Each force its later-lift survey (`later_lift_plan`, the one its launch uses) would send is set aside for it. These are the gates its launch applies.
2. **Set aside** means the force is not a lower take's first lift as it stands, and its reason names it: "task_force:N is a later lift of intent:M, higher in priority, this morning"; nor is it taken as a transport at a base by the base rule of decision 0149, item 7.
3. **A take whose plan is not yet released** when it is read (one released by leave to launch later that morning) sets nothing aside: its first lift has still to sail, and it sends no later lift that morning.
4. **Between two higher takes** the first in priority to set a force aside is named with it; which of them sends it is decided at the launch, as before.
5. **A side with at most one take in force** gets the orders and reasons `supply-33` gave, where no first lift has lost every ship (reading 6).
6. **A first lift with no ship afloat is lost** (the architect's ruling of Nov 18, 2026, on the team's finding on seed 1 of the proof: reading 7 as it was meant). The engine keeps a force whose every ship is sunk, halted where it was with its route as it stood, so a first lift sunk at the place after its landing was read as still on its way, and its take waited for it every morning and sent no later lift. `on_its_way` now reads a force with no ship in it as not on its way. That is the one place the test is made: the ashore judgement's wait, the later lifts sent alone, and `later_lifts_due` all read it; the test that the first lift is gone from the view (`launch_place`) is left as it is, so the morning's judgement of the men ashore still runs. `tests/scenario_77_forces_with_men_aboard.rs` gains a test of the team's own, not part of the hand calculation and marked so (`a_first_lift_with_no_ship_afloat_is_no_longer_on_its_way`): task force 1 lands its men and is sunk at B on day 2 by a placeholder Japanese cruiser force while it lands its supply, and task force 2 is sent as a later lift on day 4. It fails without the change.

### Earlier tests, baselines and games that change

None in more than versions. Remade by `scripts/regenerate.sh`: no run baseline, agent golden, pinned hash or batch fixture changed; of the recorded games each `save.json` changes its doctrine version only, and no transcript changed.

### Files to read

- `crates/sim-staff/src/take.rs`: `takes_prepare` and `later_lifts_due` (new); `on_its_way` (reading 6); `units_ashore` is taken out of `ashore` unchanged, so that both read the units ashore one way.
- `crates/sim-staff/src/assemble.rs` (`first_lift_as_it_stands` and the base rule's transports), `air.rs` (`Day::lifts_due`).
