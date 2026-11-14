# Step 3, stage 0: the constructor experiment

Decision 0100, point 14. Branch `s0-experiment`, not merged. What is kept is this report.

The code is `crates/sim-cli/tests/stage0_constructor.rs`. Run it with `cargo test -p sim-cli --test stage0_constructor -- --nocapture`. The constructor is `build_from_view`. It takes a `SideView`, the public map and reference data, a few switches and a seed. It never sees the live world. Six tests: the builds, a referee that compares the invented enemy with the truth (the constructor never sees that), leak probes, a declared-unseen-carrier case, a wrong-kind look and a made-id look.

One change to sim-core, on this branch only: three helpers in `world.rs` named `experiment_*` (set the day, write a claim into belief, count events). Nothing else in sim-core changed.

## What was built

Eight builds from the full slice (`07_full_slice.toml`), one from each side at four points: after day 2 (quiet), after day 3 (the carrier battle is the next day), after day 4 (the battle just fought) and after day 12 (late). Two extra builds on day 3 give the invented enemy a contact on each of our forces. Three more builds test a declared unseen force. In every build `World::new` accepted the world and two days resolved with no error and no panic. The invariants held.

That is the good news and it is the weak news. A world that resolves is not a world that is right (see 3 and 5).

Cost: all six tests (13 builds, about 40 resolved days and the live games they are compared with), in a debug build, run in under 2 seconds. A build is cheap. The cost is not in the build. No clock was read.

## 1. What the view gives a constructor, and where

- Own state, whole: `ships()`, `task_forces()`, `bases()`, `air_groups()`, `land_units()`, `replacements()`, `arrivals()`. All `pub` fields, so each can be cloned into a `TruthState`. Own ids, own fuel, damage, ammunition, aircraft, missions, routes and standing orders (those live in the state) come across exactly.
- Enemy bases: `enemy_bases()` gives id, name, hex, side (`PublicBase`).
- What the side holds: `contacts()` (hex, count, kind, age), `claims()` (hex, day, pulse, sunk, hits, kind, shot down, on ground), `ground_estimates()` (hex, men, day).
- The side's own mission, objectives, levels and horizon: `mission()`, `objectives()`, `levels()`, `horizon_days()`.
- History: `events()`. The visible `ReportReceived` and `ContactDropped` events give the history of contacts, including contacts since dropped. This is the only source for forces "known and lost".
- Public data: `ship_class()`, `aircraft_type()`, the figures methods. The whole `RefData` and `Map` are not on the view. The experiment took them from a freshly loaded scenario.

## 2. What had to be assumed, item by item

Every build prints its list. These are the kinds:

- **Enemy ship classes.** The class tables have no side column. Rule used: the lowest-id class of the contact's kind that the side does not itself use. It works in the slice because the sides share no class. It falls back to the side's own class when no other class of the kind exists (it did, for transports: only a placeholder cargo ship exists).
- **Composition.** A contact's kind is the most important kind in the group and its count is all the ships. A first version built "9 carriers" from a contact of 9 ships and kind carrier. The truth was 2 carriers, 1 battleship, 4 cruisers, 11 destroyers. Rule now: one ship of the kind, the rest destroyers (all of the kind for a convoy). Still a guess.
- **Per ship:** full fuel, full gun rounds, torpedoes ready, undamaged, no cargo, no troops, experience 2.
- **Aircraft.** A carrier gets a fighter group (a third of its capacity) and a bomber group (two thirds), ready, undamaged, pilots not counted, fatigue 0, mission Rest. Types: lowest-id type of the role that the side does not itself use. Land air groups at enemy bases: none built. The view says nothing of them.
- **Enemy bases:** supply 1000 t, fuel 500 t, no AA, no repairs, no defences, airfield yes. All invented. The view gives none of them.
- **Forces' mission and orders:** mission by kind, readiness Normal, no route, no home port, no orders. The enemy's standing orders are unknowable.
- **Enemy belief about us:** two settings. Nothing (default). Or an age-0 contact on each of our forces, merged by distance. The second is the worst case for the side. It is an assumption either way and must be shown.
- **Ids:** invented ones start at 900,000.
- **Seed:** the view has none. The caller must give one, keyed on permitted inputs.
- **Lost forces:** placed at the hex of their last report, not spread over the disc they could have reached.
- **Own contacts:** re-entered as starting contacts. Ages kept. Contact numbers are not the live numbers.
- **Own claims, own ground estimates:** re-entered. An estimate loses its day (it becomes day 0).
- **Event log:** the new log holds only day-0 belief events. The side's history of events cannot be replayed into a log (`SideEvent` is not an `Event`).

## 3. What could not be built, and why

Through the public API alone, a world with the side's belief cannot be built at all. Four things needed the branch helpers or are impossible:

1. **A game with a day behind it cannot take starting contacts** (`hold_starting_contacts` fails once the day is above 0). The belief has to go in at day 0, and `World` has no way to set the day afterwards. The experiment added `experiment_set_day`.
2. **A claim cannot be written into a belief.** `World::beliefs` is crate-private and `BeliefState::new` is too. The experiment added `experiment_hold_claim`. A ground estimate has the same problem as a claim: only the opening-estimate route exists, and it drops the day.
3. **Made-in-play ids clash.** `MadeIds` counters start fresh at 1,000,001 (Allied) and 2,000,001 (Japanese) and cannot be set. The slice's own state already holds Allied air groups 1,000,001 and 1,000,002 from day 4, and Japanese forces 2,000,001 and 2,000,002 from day 8. The first group or force made in a built world takes a number an existing unit holds. The build accepted this without complaint. Two units with one id would share a random stream, which the determinism rules forbid. A real constructor must be able to set the counters.
4. **The order book cannot be rebuilt.** `OrderId` has no public maker and `World::orders` is crate-private. Own state keeps `OrderId`s and `EventId`s that point at the live log (`mission_order`, `route_order`, `alerted_by`, `arrival_event`, `halt.event`, `yard_trip.departure`, and more). In the slice 5 forces cite an event by day 12. They were copied as they were. Resolving two days did not crash on them. It is luck that nothing looked them up. The constructor should clear or re-point them.

Also not built:

- **The scenario's objectives for the world.** `Objectives` is public and could be made from `mission()`, `objectives()`, `levels()` and `horizon_days()`, but the enemy's `SideObjectives` are not in the view. Not done. A trial can then score the side's own points only, with no margin.
- **`known_progress()` in the built world.** It is worked from the event log, which is empty. A built world forgets what the side knew of its progress.
- **Enemy land units, enemy replacements and arrivals, enemy bases made by landing.** Nothing says they exist. (The slice has no land units. The code builds one unit per ground estimate, but it was not exercised.)
- **A scenario's "starting contacts".** `Scenario` keeps them in a private field with no accessor. The slice has none, so nothing was lost here. A trial needs them.

## 4. What the view or the scenario must say for stage 4

- **The scenario must state, per side, the enemy forces the side believes exist and has not seen** (place or area, ships, top kind). After day 3 the Allied view holds no contact at all, and the Japanese carriers strike on day 4. Every pre-battle build had no enemy ships. Those worlds are valid and prove nothing, which is exactly the spec's warning. "Not seen" and "believed absent" need two separate inputs.
- **A contact needs a class guess.** Better: a contact carries a composition guess (kind counts), or the class tables carry a side column and a "typical force" for a kind. Today the constructor guesses a side from "classes the side does not use", which would fail the day two sides share a class.
- **Class and aircraft tables need a side column.**
- **A contact's count and kind are weak.** At day 4 the Allied contact was "3 cruisers" and the truth was 17 ships with 2 carriers. The claim's kind (`claimed_kind` carrier at that hex) was the only sign of the carriers. The constructor now raises a contact's kind from a nearby raid claim. That should be a stated rule, not a quiet one.
- **The view lacks a way to say how old a contact's hex is in distance.** `age_pulses` is there. The reach per pulse and a spread rule are not. "Placed by how far they could have gone" is a disc, and after 8 days it covers 81 hexes, most of the map. One world holds one place. Placing needs a distribution, so a trial draws from it, and the draw needs a stream of its own.
- **`World` needs a constructor from parts:** a day, a belief (contacts with ages, claims, estimates with days), made-id counters, an event log or a stated "history starts here", all by a public, checked call. Not four helpers.
- **Enemy standing orders.** A built enemy has none and does nothing (see 5). Stage 4's doctrine must play it. The experiment cannot say what it should hold.
- **The seed** must be derived from permitted inputs and passed in.

## 5. The hard cases

- **No contact.** Arose: every pre-battle build. Built a world with no enemy ships at all. Valid, resolves, and proves nothing. The constructor cannot fix it alone: it needs the declared-unseen input. Tested: three cases after day 3 (none; six carriers at the enemy base; seventeen ships three hexes from the real fleet). In all three, two days resolved with no fight. Invented enemy forces with no orders sit still, and their air groups are on Rest. The enemy needs doctrine. The built world's own side lost no ships and had 169 ready aircraft after day 4, against 158 in the live game.
- **An old contact.** Arose: Allied view day 12, a contact aged 2 pulses. Placed at its estimated hex, age ignored. A truly old contact needs the disc.
- **Two reports of one force.** Arose, and the constructor handled it badly. Two dropped Allied contacts were judged "probably the same as a held contact" by a crude rule (same kind inside the reach disc), and dropped. That rule is nearly always true after a week. In the Japanese view on day 12 a day-7 cruiser and a day-9 carrier at the same hex (0,10) became two forces. They are probably one. Nothing in the view separates the cases. This needs a rule and a test of its own.
- **A claim of a sinking that is too high.** Did not arise: claimed sunk equals true sunk on days 4 and 12 for both sides (1 and 1, 2 and 2). The constructor does not use `claimed_sunk`. Applying it would shrink a contact by a number that can be wrong in only one direction (never below the truth), so it would under-count the enemy. Policy for stage 4 to decide.
- **A contact of the wrong kind.** Arose. The Japanese view on day 9 holds "1 carrier" at (0,10). The truth there is cruisers on both sides. The constructor built a carrier with two air groups from it, and carried the invented carrier into day 12 as a lost force. The Allied contact on day 4 ("cruiser", truth carriers) was the opposite error, and was repaired only because a claim named the kind. A contact's kind needs a stated error rate in the trial.
- **An enemy carrier never seen.** Arose (see "No contact"). It is the whole difference between the world after day 3 and the real day 4.

## 6. Leaks in `SideView`

Probed with the truth's enemy ids: every `ask` question variant (air, surface, movement, ground), mixed own and enemy ids, `strike_choice`, `forecast_passage`, `yard_plan`, `landing_rate_factor`, the ground methods, and the visible events (ids and ship names). No leak found.

Two things that look like leaks and are not:

- `ask(Air(Landing { groups }))` with an enemy group answers with an empty list. It answers the same for a group that does not exist, so it tells nothing.
- A Japanese event on day 12 names task force 5 in an order rejected as `NotYourUnit`. That is the side's own order, which is allowed (decision 0065).

Things to watch: several choices (`halt_ground`, `surface_choices`, `yard_plan_in`) are given the whole truth maps and rely on their own side checks. The probes show those checks hold today. The type does not enforce it.

## 7. Cost

See the top: a build is cheap. A whole build plus two days in a debug build is a small fraction of a second.

## What to do with this

1. Add the declared-unseen input to the scenario.
2. Give `World` one public, checked constructor from parts (day, belief, made-id counters, history marker).
3. Give a contact a composition and give the tables a side column.
4. Settle a rule for two reports of one force, and for a wrong kind.
5. Do not count a built world as proof of anything until its enemy has doctrine.
