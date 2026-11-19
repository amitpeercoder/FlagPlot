# 0163: Fed with the lift: the second part of the first mend of stage 4

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 1 of its nine, "fed with the lift"). Step 1 is written in two records: decision 0162, the load of a supply run by night; and this one. It adds no rule to the engine: it changes where the staff finds its source of supply and where a lift goes when it has landed, and it gives one base of the 39 days a stock of fuel. Built on Nov 19, 2026 (As built, below).

Summary: Under "control an island" the staff named no source of supply until a transport happened to lie empty at one of its bases, and took the first base where one did, whatever that base held. The ships that had carried the men went back where they came from and fed nobody. Now the source is the base with the most supply, named on the first morning; a lift that has landed its men goes on to that base and joins the ships that feed the place; and Rabaul, which had no fuel, has some, so that a supply ship can make a third run

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161. Built on Nov 19, 2026

## Why

Decision 0162 says how the hunger of decision 0159's proof was traced. Three of its causes are taken here. Each is read from the code and from the trace of seed 1.

**The source waits for a ship, and takes the first base with one.** Decision 0152, item 10, chooses the source of supply "on the first morning there is a base that fits decision 0151, item 8, with a transport force it may take lies stopped there with nothing aboard". Until then the hold names no source, and three things follow. The supply of the men ashore is not formed (decision 0159, rule A, needs a source). The check of whether the place can be held (decision 0144, items 10 to 15) is not made, so a take sails with no word on how its men will be fed. And the base chosen is the first at which a force lies empty: the code takes it even where it holds no supply at all, and keeps it. On seed 1 the Japanese men landed on day 13 and the source was named on day 16, when the force that had carried them lay empty at Rabaul.

**The ships that carried the men feed nobody.** A lift as it stands is routed to the place and back to where it came from (decision 0152, item 6). On seed 1 eight Japanese forces carried men from Shortland, landed them, and went back to Shortland, which holds no supply and is not the source. They lay there for the rest of the game. The place was fed by one force of five ships.

**No Japanese base holds fuel.** The scenario gives Rabaul supply and no fuel. Refuelling is the engine's rule and needs a stock (chapter 2, Refuelling in port); the staff's reckoning of a run assumes the ships fuel on their return. The five ships made two runs and had not the fuel for a third: from day 29 the staff said so each morning and sent nothing.

Decision 0161 words this mend as the supply "formed on the morning their lift is released, sized to the men the lift carries and the days the staff expects them to wait, and counted in the lift's shipping". Read against the code, it is built so:

- **Formed with the lift.** The source is named before the lift sails, so the check of decision 0144 is made at the release and its words say whether the place can be held; and the supply intent is formed on the first morning the men have a base ashore, as decision 0159 has it, with no wait for a ship.
- **Sized to the men and the days.** That is decision 0144's check, items 11 to 14, which now runs.
- **Counted in the lift's shipping.** The lift's own ships become the ships of the supply. A lift's load of supply beside its men is not raised: by night a ship lands no more than the night's hours allow (decision 0145, item 7), and its men fill that.

One thing this does not gain is days. A force found empty at the source on a morning joins the pool by a command of that morning, and the supply is served on the limit as it stood: the first run is ordered the morning after, as before.

## The rules

1. **The source of supply is named without a ship.** This replaces the test of decision 0152, item 10. Where the commander names none, the source is the side's base in no named place's hex that the staff may take with the most supply above its reserve, the lower number on a tie; chosen on the first morning there is one, and kept, as before. A base with nothing above its reserve cannot be the source. No transport force need lie there.
2. **The pool is as before** (decision 0152, item 11): each morning the forces limit of each hold the staff formed is set to the source and the transport forces it may take that lie stopped there with nothing aboard. On the morning the source is named it may be the source alone.
3. **With no ship in the pool the supply intent is served and says so:** the run it would order, and that no ship of its forces limit can carry it. It is the supply doctrine's reason for a run it cannot send, unchanged.
4. **A lift that has landed goes on to the source of supply.** A lift of "take a place", the first or a later one, and a force a hold sends by decision 0152, item 8, is routed to the place and on to the source of the hold of that place, where a hold of the place with a source is in force and every ship of the force has the fuel for the passage to the place and on to that source, by the engine's measure at the speed it goes. Else it is routed back to where it came from, as before. Where the two are one base nothing changes. A source named on a morning is read by the take and the hold of that same morning.
5. **It joins the pool by item 2** once it lies there with nothing aboard. Nothing else is done to it: a force with men left aboard, or supply it did not land, does not join.
6. **The reason of the route** names the base it goes on to and why.

## The data

7. **Rabaul holds fuel.** Base 2 of `data/scenarios/07_full_slice.toml` is given `fuel_t = 50000`, a placeholder: no row gives Rabaul's stock of fuel oil in August 1942, and `data/oob/gaps.csv` gains the gap. The figure is sized not to bind: a fast transport's round trip to Lunga burns about 139 t, so 50,000 t is three hundred and sixty such trips, and a hundred destroyers' tanks filled. The comment on the base says so. The staff's setup and the three arrangements of stage 3's proof are made again by their scripts. The frozen copy is not changed.
8. **Why it is right to change it before the proof.** A side whose main base has no fuel cannot run supply for 39 days by any rule; the proof of this stage would measure the scenario's gap and not the staff. It is recorded here, before any run, and no figure is set from a run.

## What it leaves for later

- **A base that feeds ships but is far.** The source is the base with the most supply, not the nearest. On the 39 days that is Rabaul for the Japanese side and Espiritu Santo for the Allied side, which is right. A nearer base with enough is not weighed.
- **Supply moved between the side's own bases,** so that a forward base becomes a source.
- **The staff reading a base's fuel** before it reckons a run's return. It assumes fuelling, as before.
- **The bombing of the stock on the beach,** which no rule here allows for.

## Rule card

- **Reads:** the side's bases, their supply and reserves; the forces limits of the holds; a force's ships, what is aboard and their fuel; the engine's measure of a passage.
- **Writes:** nothing of its own: the source in a forces limit, and a route.
- **Reuses:** the source and pool of decisions 0151 and 0152; the check of decision 0144; the supply formed by decision 0159; the route orders of a lift. No new state, event, draw or belief.

## What it costs

- **State:** none. The doctrine version rises.
- **Earlier scenarios.** A game with no "control an island" and no hold whose source differs from where its lifts came from plays as before. Scenario 77 keeps its orders: its lift's home is the source. The build lists each scenario whose orders or reasons change, and its new numbers go to the architect, not to the developer.
- **The 39 days change** on both sides: the source is named on day 1, the check of decision 0144 is made, and the lifts go on to the source. The scripted slice changes by Rabaul's fuel wherever a Japanese force lies there short of fuel. The recorded games on the slice are remade.
- **The rulebook:** chapter 8, "Controlling an island" and "Forces with men aboard", and the scenario's notes.

## The hand calculation

It is in `tests/scenario_86_fed_with_the_lift.rs`, written before the code. It is scenario 77 with the two spare forces taken away and the lift's home made a base with no supply. As the rule stood, the lift would land its men, go home, lie empty there, and that empty base would be named the source: the men would eat what they landed with and then go hungry. Now the source is the base with supply, named on day 1; the lift goes on to it, joins the pool on day 3, and on day 4 loads the run that lands on day 5.

## As built

Built on Nov 19, 2026, on the branch `fed-with-the-lift`.

- **The source** (`island_source` in `crates/sim-staff/src/island.rs`). Where the commander names none: the side's bases in no named place's hex that the staff may take, in base order; a base with nothing above its reserve is passed over and its line says it cannot be the source; of the rest, the most supply above the reserve, then the lower number. The pool there (`pool_at`) is worked as before and may be empty; the reason gives it ("with no transport force stopped there with nothing aboard", or the forces). The hold's forces limit is then the source and no force, and its reason says "and no force". Kept as before. `island_pools` is unchanged.
- **The route** (`lift_end` in `crates/sim-staff/src/take.rs`, read by `move_step`, which serves the first lift, a second lift, a later lift as it stands and a hold's lift). Where `hold_of_place` finds a hold of a base in the place's hex with a source, and that source is not the lift's own, the route is worked out to the place and on to that source, and the force's fuel is measured by the engine's forecast at the speed it goes: the burn to the place, and the burn of the whole way, less it, on. Every ship must have the whole burn aboard and not run dry; for a ship the forecast runs dry, the whole burn is worked as the night's measure works it (`night::burn_of` over the forecast's miles and speed), so the reason gives the full burn on and not what it burns before it runs dry. Then the route goes on; its reason says "N nm on to base:X" and "It goes on to base:X, the source of supply of intent:H, the hold of (q,r), to join the ships that feed the place: every ship has the fuel, by the engine's measure at ..., ship:S has A t against B t to (q,r) and C t on to base:X, D t"; the route order's own line says "out to the place and on to the source of supply of the hold of the place". Else it goes back to its own source as before, and the reason adds "It does not go on to base:X ...: not every ship has the fuel, ..." with the same figures. The night's timing and the fuel check of the passage are worked on the route it takes.
- **Data.** Base 2, Rabaul, of `data/scenarios/07_full_slice.toml` has `fuel_t = 50000` (ruling 5), with the comment; `data/oob/gaps.csv` has the gap (Rabaul's stock of fuel oil, not searched). `data/scenarios/staff/07_full_slice.toml` is made again by `make_staff_setup.py` (the copy run first on main's inputs made main's file byte for byte), and the three arrangements by `make_arrangements.py`. The frozen copy is not changed.
- **Doctrine** `supply-41`. No state, event, draw, order or rules label changes.
- **Scenario 86** (`data/scenarios/86_fed_with_the_lift.toml`, `tests/scenario_86_fed_with_the_lift.rs`). See the build's report for each assertion that does not agree with the hand calculation; none was changed.

Readings the record did not make:

1. **The commander's source is his, whatever it holds.** The test of "nothing above its reserve" is applied only to a source the staff chooses; a source the commander names is taken on the first morning with no ship and no stock (the hand calculation's property "the commander's source", base 4 with nothing). It must still be a base the staff may take, as before.
2. **Which hold.** "The hold of that place" is read as `hold_of_place`: the first hold in force, in intent order, of a base in the place's hex with a source in the forces limit on its chain, the hold the check of decision 0144 reads. A hold the commander made counts as one the staff made.
3. **Every lift.** Item 4 is applied in `move_step`, so to a first lift made at a base, a second lift, a later lift as it stands and a force a hold sends alike. Where the lift's own source is the hold's, nothing changes.
4. **The speed it goes,** for the fuel test: by night, the speed the night's choice gives on the route on to the source (cruising speed where neither speed brings it in the night); by day, the force's own setting. The burns are the engine's forecast over the staff's waypoints, with no margin.
5. **What is left as it was.** The supply a lift takes is still sized and loaded at its own source, and its escort is still found at its own source. A take whose first lift has gone on to another base is given no second lift with that force: the second lift waits for the force at the take's own source, as before.
6. **With no ship in the pool** the supply intent's reason is the supply doctrine's own for an empty pool, "No ship of the pool of intent:N is afloat, so there is nothing to send", which comes before the doctrine reckons the stock or a run. It does not say what run it would order. Nothing in the doctrine was changed for item 3.


What changed in the games already built:

- **Scenario 76.** Its main game gives the orders it gave; the reasons of the source now say "with nothing aboard". Its property "no source" (the commander holds the one transport force) no longer has no source: base 1 is named on day 1 with no force, and the engine forms "keep base 3 supplied" under the hold of base 3 on day 1 (ruling 7); the island is controlled on 0 of 14 days.
- **Scenario 77.** The same orders; only the words of the source's reason change in its run baseline.
- **The scripted full slice** changes from day 1 by Rabaul's fuel: the Japanese transports of task force 31 now fuel there. The recorded games change in their doctrine version, and `full_slice_five_days_sonnet` in its hashes too; no transcript changes.
- **Scenarios 06d1 and 06d2** leave scenario 86 out of the files whose hashes they hold to before their stage, as they do the other staffed scenarios.

## The architect's rulings

1. **The landing's base in scenario 86 is base 5** (Nov 19, 2026). The hand calculation called it base 3; bases 1, 2 and 4 are in the scenario, so the base a landing forms takes the next number. The doc comment is restated.
2. **A supply intent with no ship in its pool gives the doctrine's reason and reckons nothing.** Item 3 said it gives "the run it would order". The doctrine tests the pool before it reckons the stock and the run, and its reason is that no ship of the pool is afloat. That stands: item 3 is read as that reason, unchanged, and the hand calculation's day 3 is restated.
3. **"Not the fuel to go on" is restated,** with the source the farther base, so that a ship can be short for the way on and have the fuel for the way home.
4. **Scenario 76's property "No source" is restated** as "No source of the landing": the source of supply is now named with no ship, which is this record's item 1.
5. **Rabaul's fuel is 50,000 t, not 5,000 t.** The first figure was sized against the supply ships' runs alone. The build's runs showed the lifts that come on to Rabaul by item 4 fill their tanks there too, and 5,000 t was gone by day 26: the figure bound, which item 7 said it was not to. It is a placeholder for a stock no row gives, set so that fuel at the side's main base is not what limits it; no run was judged, and no figure is set to an outcome.
6. **The readings of "As built" stand as made,** with one noted for later: a take whose first lift went on to the source gets no second lift with that force.
7. **Scenario 76's supply under the hold is formed on day 1, not day 4** (Nov 19, 2026). The restated property gave day 4; the figure was a misreading of the build's report, where the 4 is the number of the hold it stands under. The source is named on the morning of day 1 and read the same morning (item 1), so the hold forms its supply that day, as it does in the scenario's main case. The doc comment is corrected by the architect.
8. **The reason of a lift that has not the fuel to go on gives the whole need.** Where the forecast runs a ship dry, the reason gave the burn up to that point, which is what the ship has and not what it lacks. It now gives the burn of the whole way, by the rule the night's measure burns by. Words only: no order changes by it.
