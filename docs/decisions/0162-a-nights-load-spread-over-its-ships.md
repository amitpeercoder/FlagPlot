# 0162: A night's load spread over its ships: the first part of the first mend of stage 4

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 1 of its nine, "fed with the lift"). Step 1 is written in two records: this one, the load of a supply run by night; and the next, the supply formed with the lift. It adds one field to an order of the engine. Built on Nov 19, 2026 (As built, below).

Summary: A supply run timed to the dark carried no more than its first ship could land in the night, and the ships after it sailed empty: on the 39 days a run of five ships asked for 110.3 t carried 25 t. A load to a weight may now say the most any one ship is to hold, and the staff's run by night says it, so each ship carries what it can land in the night's hours: the same five ships carry the 110.3 t

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161. Built on Nov 19, 2026

## Why

Decision 0159's proof missed one condition: of the men the Japanese side landed at Lunga's hex the median lost to hunger was a quarter, against a mark of one in twenty. One seed was traced, event by event (seed 1 of arrangement (a), from the proof's own export). What follows is read from that trace and from the code.

Decision 0145, item 8, cuts a night run's load "until the first ship filled carries no more than its limit, and each ship after it likewise". A load to a weight fills ships in ship order (chapter 2, A load to a weight), so the only way one weight can keep the first ship inside its limit is to be no more than that limit. `night::weight_within` does that: it stops at the first ship whose room passes its limit. On seed 1 the run of day 17 had five ships, each with 30 t of room and a limit of 25 t (20 t an hour by lighter, 1.25 hours of the night left on arrival). The load asked was 110.3 t. It was cut to 25 t, all in the first ship; four ships made the passage empty, and all five were committed for six days. The second run did the same. Two runs landed 50 t, where the same ships inside the same limit could have landed 191 t (110.3 t and the 80.7 t the second asked).

The rule was right that no ship should lie off an enemy airfield at dawn with cargo still aboard. It was wrong about what that costs: the limit is each ship's, and the cut was made on the whole force.

This is not the whole of the hunger. The trace names four more causes, and each is taken where it belongs:

- The supply of the men landed is formed only when a transport force lies empty at a base of the side, which on seed 1 was day 16, three days after the men landed. The next record takes it.
- The Japanese supply ships have no fuel left after two runs, and no Japanese base in the 39 days holds fuel. The next record takes it, as data.
- Allied aircraft destroyed 117.5 t of the stock on the beach, a third of all that was landed. That is the game. No rule here allows for it; a trial (decision 0161) will show it.
- The lifts sail with the 7 or 14 days of supply the scenario put aboard and load nothing. Decision 0161's second mend takes what a take does when it cannot feed its men.

## The rule in the engine

1. **A load to a weight may name the most a ship is to hold.** The load order with a weight (`load` with `weight_t`, of supply or of fuel as cargo) may carry `each_t`, tons. Ships are filled in ship order as before; each takes the least of what its room allows, what is left of the weight, and `each_t` less what it has aboard of that cargo, not below 0. Like the weight it is a level and not an addition: a ship that holds `each_t` or more of the cargo takes none and gives none up.
2. **The weight may not be reached,** where the ships' levels together are below it. The force then loads what the levels allow and the order is done, as a load to a weight is done when the ships' room is full.
3. **It is refused** with no weight, with a unit, or at 0 or below, where an order is checked: by the engine, when a scenario is read, and by the agent's decoder. With no `each_t` a load plays as before.
4. **Nothing else of loading changes:** the berths, the rates, the hours and the events are those of a load to a weight. No event is added. The order is state, so the state's layout version, the saved game's version, the agent interface version and the rules label rise.

## The rule in the staff

5. **A supply run by night spreads its load.** This replaces decision 0145, item 8, for a supply run. The level is the least of the ships' limits of item 7 (the place's rate for a ship times the night's hours). The most the run may carry is the sum, over its ships in ship order, of the least of a ship's room and the level less its supply aboard, not below 0. Where the run's weight is above that it is cut to it, and the cut counts as a cut of the run, as before (0145, The timing, item 7). The staff's load order names the weight and the level. The level is named on every supply run that goes by night, cut or not, so that a load inside the limit in all is also inside it in each ship.
6. **The least of the limits, not each ship's own.** One order carries one level. Where the place's rate differs between the ships of a run, the ships with the higher rate carry less than they could land. In the data as it stands the rate by lighter is one figure for a ship, so nothing is lost; the reason names the level and how it was reached.
7. **The speed is chosen on the load as spread.** Item 5 of decision 0145 takes the first speed that arrives in the night with room for the load. "Room for the load" is read on the load as this record spreads it: each ship's part at that speed's hours.
8. **The ships are chosen as before,** by their room, before the night is reckoned. A run cut by the night's limit does not take more ships from the pool to carry the rest. That is left: see below.
9. **A lift is not changed.** The supply a lift loads beside its men (decision 0145, items 7 and 9, in `take.rs`) is cut as it was. Its ships' limits differ by the men each holds, which one level does not fit, and the lifts of the 39 days load nothing.
10. **The reason** of the load gives the level, the ships and what each carries, in place of the words of the cut to the first ship.

## What it leaves for later

- **More ships for a load the night cuts.** The staff could take further free ships until the levels cover the load. It needs the night's hours before the ships are chosen, which the run does not have today.
- **A level for each ship,** where rates differ or men are aboard.

## Rule card

- **Reads:** the load order's weight and level, each ship's room and what it has aboard of the cargo. In the staff: the limits of decision 0145, item 7.
- **Writes:** what a load writes: the ships' cargo and the base's stock. In the staff: the weight and the level of a load order.
- **Reuses:** the load to a weight of decision 0111, whole; the night's limit of decision 0145. No new event, draw or belief.

## What it costs

- **State:** one field of an order. The four versions of item 4 rise, the doctrine version rises, and the generated files are made again.
- **Earlier scenarios.** A game with no supply run by night, or none the night's limit cut, gives the events it gave; its hashes change with the layout version alone. Scenario 70's property "The night's limit" changes: with the lighter rate at 4 t an hour each ship's limit is 21 1/3 t, and the run now carries 21 1/3 t in each of its two ships where it carried 21 1/3 t in the first alone. The three arrangements of the 39 days and the recorded games on them change wherever a run was cut. The build lists each, and its new numbers go to the architect, not to the developer.
- **The rulebook:** chapter 1's table of orders and chapter 2's "A load to a weight" gain the level; chapter 8's "A run by night" has the cut restated; the glossary gains the word.

## The hand calculation

It is in `tests/scenario_85_a_nights_load_spread.rs`, written before the code. Three fast transports of 50 t each feed a garrison 160 nm away under an enemy base. They have not the fuel for full speed, so they go at cruise and arrive with two hours of the night left: 40 t a ship. The load asked is 150 t. As the rule stood it would be cut to 40 t, in the first ship. Now each ship loads 40 t and the run lands 120 t in the dark.

A second case, of the order alone, is in the same file: ships with 10 t, nothing and 45 t aboard, a weight of 100 t and a level of 40 t, end with 40 t, 15 t and 45 t.

## As built

Built on Nov 19, 2026, on the branch `night-load-spread`.

- **The engine.** `TaskForceOrder::LoadTo` takes `each_t: Option<Fixed>`, and the cargo order of the state keeps it (`CargoOrder::each_t`), hashed after `heeds_reserve`. In the cargo rule each ship's load is cut by its level less what it holds of the cargo (`World::level_left`), and a load is complete when no ship can take more by its room and the level (item 2). A level of 0 or below is refused at intake with the new refusal `LevelNotAboveZero`, added at the end of the enum. The loaded event's breakdown gains the factor `each_t` only where a level is named, so a breakdown with none is as it was.
- **The loader and the decoder.** The scenario key `each_t` of a `load` order, and the agent's `"each_t"`, a decimal string, are refused with no weight, with a unit, and at 0 or below. The agent writes `each_t` only where there is one, so a load with none is written as before. The order schema, the issue_orders description and the agent's README name it.
- **The staff.** `night_load` in `doctrine.rs` reckons the level at each speed's night (the least of the ships' limits), chooses the speed on the load as spread (`night::choose_by`), cuts the load to what the levels allow (`night::most_within`), and names the level on the load order of every supply run that goes by night. The load order's reason gives the level, how it was reached and what each ship takes and holds; the night's words say no ship is to hold more than the level. `night::weight_within` is kept for the lift alone (item 9), which names no level.
- **Versions.** State layout `flagplot-state-v48`, saved game 23, agent interface 27, rules label 20, doctrine `supply-40`.
- **Scenario 85** (`data/scenarios/85_a_nights_load_spread.toml`, and `85b_a_load_with_a_level.toml` for the order alone) and class 13 in `data/reference/ship_classes_placeholder_fast_transport_50t.csv`, with its frozen copy in `data/reference/step1`. Every number of the hand calculation is asserted, and every one agrees. Scenario 70's property "The night's limit" is asserted as restated, and agrees.

Readings the record did not make:

1. **The engine cannot be given a level with no weight or with a unit:** its order type has a weight and no unit. Those two are refused by the loader and the decoder; the engine refuses a level at 0 or below.
2. **A ship at its level takes no berth** for the load: it has no cargo to move, as a ship whose room is full has none.
3. **A load the commander orders below a reserve** (decision 0111, item 9): what it asks counts each ship's room only up to the level less what it holds, in the engine and in the agent's answer.
4. **A load held at the reserve** (decision 0111, items 4 and 8) reads "room aboard" as a ship that can still take more by its room and the level.
5. **"The most the run may carry" (item 5)** is read as the most it may take on top of the supply aboard, which is what the engine will load: the run's weight is cut to the supply aboard plus that. Where nothing is aboard the two readings are one.
6. **Room at a speed (item 7)** is read as: the whole load is taken when spread under that speed's level, and each ship's supply aboard plus its part is within its own limit at that speed's hours. A ship already holding more than its limit fails it, as before.
7. **Not changed: the run's times.** The load and unloading hours a run's reasons give, and the days they reckon, are still worked on the load filled in ship order as first asked, before the night's cut, as they were before this record. Scenario 85's reason says "ship:1 50 t at 150 t an hour = 0.333333 hours" for the load and "unloading 120 t at base:2 = 2.5 hours", where the ships take 40 t each and land them in 2 hours. No day moves in scenario 85; it is left for the architect.
8. **The lift's check of an order already given** (`take.rs`) compares the weight only, as before.

What changed in the games already built:

- **Scenario 70.** Its main case goes by night at full speed, inside the limit, and now names the level (106 2/3 t) on its load order; it loads and lands as before, and only the reasons' words change in its run baseline. Its property "The night's limit" is asserted as the architect restated it: 21 1/3 t in each of its two ships, 42 2/3 t in all, where 21 1/3 t went in the first.
- **Every other run baseline, the recorded games, the pinned hashes and the batch fixture** change in their versions and hashes only. No recorded game has a supply run by night, and no transcript changes but in its `version`.
- **The three arrangements of stage 3's proof** (`data/scenarios/stage-3-proof/`), run once each on their own seed, 1942: no supply run by night is ordered in any of them; every run by night in them is a lift, which this record leaves as it was, so each plays as before. On seed 1 of arrangement (a), the seed the record traces, the Japanese supply run of day 17 now loads 110.3 t, 25 t in each of four ships and 10.3 t in the fifth, where it loaded 25 t in the first; the game then differs, and the run of day 23 loads 94.1 t. Nothing in them is judged here: the proof is run again with the next record.


## The architect's rulings

1. **The hours a run's reasons give are still worked on the load filled in ship order** (Nov 19, 2026). The load and unloading hours in a night run's reasons are reckoned as before this record, on the uncut load filled ship after ship: in scenario 85 they say 2.5 hours of unloading where the ships land their 40 t each in 2. This stands for now. The days of a run are reckoned at cruise as before (decision 0145, item 10) and no day moves by it; the hours given are never shorter than the hours taken, so nothing is promised that is not kept. The reasons should give the hours of the load as spread, and that is owed as a correction of words with the next change to the supply run.
2. **The seven readings of "As built" stand as made.**
