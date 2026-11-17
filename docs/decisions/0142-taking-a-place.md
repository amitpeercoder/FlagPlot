# 0142: Taking a place: step 3 of stage 3

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3, that the designer confirmed the same day (decision 0139, step 3). It writes the end "take a place" and the doctrine that carries it as far as the landing, over rules already built (decisions 0066, 0090, 0095, 0098 and 0126) and the task "ready a landing" of decision 0141, with the hand calculation. It is built after decision 0141. Built on Nov 17, 2026: see "As built".

Summary: Step 3 of stage 3: the end "take a place", by which the staff chooses the units that land by what each is good at and by the ships that can carry them, forms "ready a landing" for them, asks leave to launch when it is ready, and on leave loads supply for the men, embarks them, sails, lands them on attack, and says when the place is the side's

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings await the architect (As built)

## Why

A commander can ask for a landing to be made ready (decision 0141). He cannot yet ask for a place. This step lets him: he names the place, the ships, the units and where they sail from, and the staff does the rest up to the day the place changes side. What the men do ashore when the enemy has men there is step 4; a run by night, bombardment, cover by carriers and reconnaissance are steps 5 and 6.

## The end

1. **Take a place** names a hex of land, by its hex or by a base on the side's map that is not the side's, which means that base's hex. It may have a date: the day by the end of which the place is to be the side's. It is an end, addressed to the area, and a root: a commander states it.
2. **Its forces limit** names the task forces, the land units and a source, a base of the side. A forces limit with no source is refused, as for keeping a base supplied.
3. **It is met** when the side holds a base in the hex and the enemy holds none there. Stated of a place that is the side's already, it is met at once and nothing is ordered.
4. **In this step the staff lands at the place itself.** For a place that is not a coast hex it gives no order and says that it has no way there: a march from a beach is step 4.

## The phases

5. **A plan's phase is read, not stored.** Prepare, while the plan is preparing. Move, from its release until the first man lands at the place. Land, from then until the intent is met. The tools show the phase. Follow up is step 4.

## Prepare

6. **The transport force** is the lowest-numbered task force of the forces limit on the transport mission, with a ship afloat of a class that carries men, that the side may use and the commander does not hold. It must lie stopped at the source, or have men of the limit's units aboard. One force lands an operation in this step.
7. **The units that may land** are the fighting units of the forces limit that the side may use, that the commander does not hold, and whose men are all ashore in the source's hex or all aboard the transport force. A working unit is not landed by this step.
8. **Their order** is by what each is good at: the higher assault skill first; then the higher experience; then the lower number.
9. **The ships.** A ship carries men of one unit. Units are taken in that order. Each takes the force's ships not yet taken, in number order, until their berths hold its men; a unit for which the ships left are too few is left out, and the next is tried. A unit aboard already keeps the ships it is in.
10. **How many men.** Where the side holds an estimate of the enemy's men at the place, the staff stops taking units once the men chosen are at least the estimate times the figure for its risk:

| The risk | Men for each man of the estimate |
| --- | --- |
| Preserve the force | 3 |
| Balanced | 2 |
| Press | 1 |

With no estimate it takes every unit the ships can carry, and says that it goes with no count of the enemy. Where the units the ships can carry are fewer than the men wanted, at balanced and at press it goes with what it has and says so; at preserve the force it forms nothing, asks no leave, and hands in the notice that it cannot meet the intent. The three figures are the staff's and are placeholders.
11. **The choice is made once,** on the first morning the plan is preparing, and kept as the forces limit of the task it forms. The commander may change that limit, as he may any.
12. **The task formed.** The staff forms "ready a landing" (decision 0141) under the intent, for the transport force at the place's hex, and sets its forces limit to the units chosen. Two things are new for this: `form_intent` for "ready a landing" under "take a place" is taken in while the plan is preparing or released; and the staff may give `set_limit` of forces for an intent it formed, checked as any limit under a parent is. The task is served that morning.
13. **Its date.** Where the intent has a date, the task's date is the morning of that day, less the days to embark and the days of the passage, plus one (below). The task's own rule then says on the first morning whether it will be late, by how many days and what binds.
14. **When it is ready.** As decision 0141 has it, by the risk. One case is added: a force that does not lie at a base of its side can gain no loading and its units no rehearsal, so the task is ready as it stands, and the reason says what it goes without.
15. **Leave.** On each morning the task is ready and the plan is preparing, the staff releases the plan where leave to launch is given. Where it is withheld it gives no order and hands in the notice that the plan waits for leave to launch. Before the task is ready it hands in no such notice: the day's reason gives the loading, each unit's rehearsal and the morning it expects to be ready.

## Move

16. **The supply.** On the morning the plan is released the force is ordered to load supply at the source to a weight: the daily charge of the men chosen for 15 days, the interval of decision 0107 where none is set. The 15 days stand until step 4 builds the check that the place can be held, which sets the supply by the days until the first run that follows can arrive (decision 0139, "Amended by the designer", 2). It is cut to the source's stock above its reserve and to the ships' room, and what is aboard already counts toward it. With nothing to take no order is given and the reason says so.
17. **The men.** On each morning after, one unit whose men are ashore at the source is ordered aboard, in number order, by a load order with the unit and no cargo. On the same morning it is set to attack. A unit aboard already needs neither.
18. **The days.** The days to embark are one for the supply, where there is any to take, and one for each unit ashore. The passage is its hours at the cruise speed of the slowest class, rounded up to days. A force released on the morning of day R sails on day R plus the days to embark, and is due on that day plus the days of the passage less one.
19. **The passage.** On the first morning that every unit chosen is aboard and the supply is aboard or has nothing more to take, the force is given a route to the place and back to the source, marked to act at the place, and an unload order. The route, the area limit and the fuel each ship needs for the round trip are as for a run (decision 0107). An escort is given as for a run (decision 0126), with the place as the destination. A ship without the fuel, or no route, stops it, and the reason says which.
20. **The ready task under it** is given nothing from the morning the force is routed: the landing spends what it made ready.

## Land

21. **The landing** is the unload order's, by the rules as they are: the men, then the supply, at the rate the loading gives; the first man forms a base where the side has none.
22. **A unit that could not be set to attack before it sailed,** because it was aboard already, is set to attack on the first morning it has men ashore at the place.
23. **Taken.** On the first morning the intent is met the staff marks the plan complete and hands in the notice that the intent is met, once. Nothing else is ordered: what the men do next is step 4.
24. **Not taken.** While the enemy still holds the place the staff gives no order in this step and says how many men it has ashore and what it holds of the enemy's.

## What both keep

25. **Nothing is ordered without leave to prepare,** and nothing sails or embarks without the plan released.
26. **A hold is honoured.** A held transport force or a held unit is left out, and the reason names the hold.
27. **Reasons** carry the numbers, in the order a staff works in (decision 0139, "Amended by the designer", 3): the estimate and the men wanted; each unit in its order with its assault skill, its experience and its men; the berths; each unit left out and why, and what is short; the supply in tons; the days.
28. **A proposal** shows the tree before it is formed: the task, the units, the ships, the supply and the days.

## What it leaves for later steps

- **The fight ashore,** the march from a beach to a place inland, and what follows the capture: step 4.
- **A landing by more than one force,** and a working unit landed with the fighting ones.
- **Cover, bombardment and a run timed to the dark:** steps 5 and 6.
- **A landing not ready until it holds a count of the enemy:** step 6.
- **The force assembled by the staff,** where the commander names none: step 7, added by the designer on Nov 17, 2026. In this step a forces limit is required.
- **Choosing the day and the beach:** stage 4.
- **Building up the base it sails from:** formed with cover in step 6, where a field is what cover needs.

## Rule card

- **Reads:** the side's own task forces, ships, land units and bases; its estimate of the enemy's men at the place; the map; the ground figures; the intent's limits, its date and the two leaves.
- **Writes:** one kind of intent with its place and date, in the command state; nothing else.
- **Reuses:** `form_intent`, `set_limit`, `approve`, `mark_plan` and the notices; the task of decision 0141; the load, route, unload and posture orders; the run's route, fuel and escort rules. No rule of the engine changes but the two of item 12.

## What it costs

- **State:** one kind of intent, in the hash and named. The state layout, saved game and doctrine versions rise; the agent interface version rises with the tools. The rules label does not: no game without the intent plays differently.
- **The rulebook's chapter 8** gains "Taking a place".

## The hand calculation

`tests/scenario_67_taking_a_place.rs`.

## As built

Built on Nov 17, 2026, on the branch `stage3-step3`. Nothing is tuned and no figure changed. Every number of the hand calculation that an event or the state carries is asserted exactly, and no test is ignored: no mismatch was found. The flagged question, which base takes the supply landed after base 2 changes side, comes out as the hand calculation has it: the engine puts it into the side's lowest-numbered base in the hex (base 2). Two hours of the table are not asserted because no event carries them: the hour each unit's embarkation is complete (169 and 193); the unit is aboard, whole, the next morning, which is asserted. Every scenario built before this one gives the events it gave: the run baselines changed in their hash lines only, and the recorded games in their versions and day hashes.

### What was built

- **The state (`crates/sim-core/src/command.rs`).** `IntentKind::TakePlace { hex, by_day }` (code 13, `take_place`, addressed to the area, a possible root); `IntentKind::place`; "ready a landing" may stand under it where the landing's hex is the place; `PlacePhase` (prepare, move, land) and `SideView::place_phase` (`crates/sim-core/src/view.rs`), the phase read from the plan and the side's men; `CommandRefusal::NotLand`, at the end. In the hash after a build-up (`hash.rs`), named, layout `flagplot-state-v41`.
- **Intake (`command/intake.rs`), the two engine changes of item 12.** `form_intent` of a "ready a landing" under a "take a place" is taken in while the plan is preparing as well as released. The staff may hand in `set_limit` of forces (`CommandKind::allowed_from`), and intake refuses it "Not yours" on an intent the staff did not form or for any other kind of limit; it is otherwise checked as any limit under a parent. A take-a-place forces limit with no source is refused "No source". The place must be a hex of land (coast or inland) on the map.
- **The doctrine (`crates/sim-staff/src/take.rs`).** `Day::takes_prepare` (met, item 4, the transport force, the units, the ships, the men wanted, forming the task and its limit, its date) runs in the air doctrine's morning before `Day::readies`, and `Day::takes_launch` (leave to launch, the supply, the men, the passage, the land phase) after it (`air.rs`, `Day::run`). The ready task records its readiness each morning (`Day::landings`, `crate::take::LandingReading`), which the end reads. The leaves no longer release a take-a-place plan (`air.rs`, `leaves`). Item 14 is in `ready.rs` (`serve_landing`), and item 20's skip in `Day::readies`. The escort rule of decision 0126 now takes a `RunContext` (`escort.rs`), made from a supply run's planner or from the place: no change for a run.
- **Scenario 67** (`data/scenarios/67_taking_a_place.toml`, on scenario 27's map) and its 28 tests (`tests/scenario_67_taking_a_place.rs`): the hand calculation in nine, each property in its own, plus a sim-core test of the intake (`command/tests.rs`).
- **Versions.** State layout `flagplot-state-v41`; saved game 16; agent interface 19 (and the schemas' constants); doctrine `supply-22`. The rules label stays 18.
- **The tools.** `set_intent` takes `take_place` with `hex` or `base` (an enemy base on the side's map, read as its hex) and `by_day`; `command_state` gives the place as `hex`, `by_day` and a `phase` for every intent (null but for a take-a-place); the viewer draws the kind at its hex (`viewer/index.html`, `viewer/check.js`).

### Readings taken

Where the decision leaves a detail open; the architect may rule on each.

1. **The place is stored as a hex.** A base names its hex at the tool (`sim-agent`), read from the side's public bases and refused there if it is not a base on the side's map that is not the side's. A scenario file names the hex.
2. **"A hex of land"** is a coast or land hex on the map; any other, and a hex off the map, is refused with the new reason "Not land".
3. **The staff's `set_limit`** is of forces only, on an intent with maker staff; otherwise "Not yours", tried after "No such intent".
4. **Met first.** Each morning the plan is approved, the place being the side's is tested before anything else; then `mark_plan` complete, and the notice `met` (the kind of decision 0141, reused: its event's words stay "every want of plan:N is met") once, resting on the `mark_plan`'s reason. Stated of a place already the side's, that is day 1, after the leaves approve the plan to preparing.
5. **The phase "land"** begins when any man of the side is ashore at the place, not only a man of the units chosen.
6. **The ships a unit takes** are counted, not named: which ship its men go into is the load order's. So the reason says "land_unit:3 takes 1 ship", and in the hand calculation unit 2, loaded first, goes into ship 1.
7. **The stop of item 10** is tested before each unit after the first, so at least one unit is taken whatever the estimate; a unit after the stop is "not needed". The men chosen count a unit's men ashore and aboard.
8. **`cannot_meet` at preserve the force** is handed in once for the plan (read from the side's events, as decision 0141's notices); later mornings give a reason with no notice.
9. **The supply's charge** is each chosen unit's men, ashore and aboard, times `daily_supply_t_per_man` times its weight factor, for 15 days. The source's stock above its reserve counts only where the force lies stopped at the source; the level is the least of that charge, what is aboard plus that stock, and what is aboard plus the ships' room.
10. **One step a morning, in order.** On the morning of release: the supply where there is any to take, else the first unit; each later morning the next unit; then the passage. A standing load not yet done is waited for with a reason. This gives item 18's arithmetic: with no supply to take, the first unit goes aboard on the release morning.
11. **The posture order** comes before the load order on the same morning. On a morning of the land phase any chosen unit ashore at the place not on attack is set to attack (item 22), and with none the reason of item 24 is given.
12. **The route** is from where the force lies (so a force at sea is routed from there) to the place and on to the source, each leg as `route_between` gives it inside the area read up the tree. The fuel is the run's rule, each ship's class rate over the miles out and back, with no margin. The escort is decision 0126's rule unchanged, with the place as the destination and the end's forces limit; its charge is not cleared after the landing (item 23: nothing else is ordered).
13. **"Routed"** is read from the force: its route acts at the place, or it is landing there. From the morning after the route is given the ready task under the end is skipped; on the morning itself the ready task is served before the route is given, and gives nothing new.
14. **Item 14's base** is a force stopped at a hex centre where the side has a base. The ready task still gives the force `prepare_landing` where its preparation differs (decision 0141, item 2), at sea too; it is ready as it stands with a roster that is not empty.
15. **The task's date** is not set before day 1; the passage is at least one day.
16. **Leave to launch** is asked each morning the landing is ready and the plan preparing (the notice each morning, as a hold's). The reason of days 1 to 4 is the end's own, giving the ready task's loading, rehearsals and projected morning of that morning.
17. **A unit held after the choice** is not dropped: the end waits with a reason naming the hold when it is the next to embark.
18. **A proposal** shows the forming and its words (units, ships, supply, days) and goes no further: no leave, no move.
19. **The view.** The doctrine reads the side's own view, as decision 0141's does, not the area command's held picture.

### Earlier tests touched

No expected number changed. Version pins move in the agent's and the saved games' tests (19 and 16); `crates/sim-cli/tests/export_pictures.rs`'s export hashes and the golden and recorded files are remade by `scripts/regenerate.sh` at layout v41.

### Files to read

- `crates/sim-staff/src/take.rs`: the doctrine whole (`takes_prepare` and `prepare_place`, `choose`, `supply`, `route_for`, `days`, `takes_launch` and `launch_place`).
- `crates/sim-staff/src/ready.rs`: item 14 in `serve_landing` and item 20 in `readies`; `crates/sim-staff/src/air.rs`: the leaves and `Day::run`; `crates/sim-staff/src/escort.rs`: `RunContext`.
- `crates/sim-core/src/command.rs` and `command/intake.rs`: the kind, the phase, the two intake changes.
- `tests/scenario_67_taking_a_place.rs`.


### The architect's rulings, Nov 17, 2026

The nineteen readings are accepted as built. Four are worth their reason or leave something to do.

- **Reading 4, the notice's words.** The notice that the intent is met reuses decision 0141's kind, whose event says "every want of plan:N is met". The kind is right and the words are a build-up's. They are mended with step 4's build: the words are the intent's, "intent:N is met".
- **Reading 6, ships counted and not named.** Which ship a unit's men go into is the load order's. The hand calculations of the later steps name the ship a unit goes into only as the load order gives it.
- **Reading 8, the notice that it cannot meet the intent, once.** A notice is news, handed in on the morning a thing first holds and again if it holds anew after it stopped. The later steps of this stage follow this, where their records say "each morning": the reason is given each morning and the notice once.
- **Reading 12, the fuel with no margin.** It is the run's rule, and a landing force is a run.
