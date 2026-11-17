# 0147: Reconnaissance for an operation: step 6 of stage 3, first part

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 6 of its nine, and its items 4 to 6, which the designer confirmed as the smallest useful part of `docs/design/intelligence.md`, "Reconnaissance as part of an operation"). Step 6 is written in two records: this one; and the next, cover for an operation by carriers. This one adds a rule to the engine. It is built after decision 0146. Built on Nov 17, 2026, with decision 0148: see "As built".

Summary: Step 6 of stage 3, first part: a count of the enemy ashore has a grade (told, seen from the air, or men in contact); an air group on the new reconnaissance mission flies to a place once a day and reports the men there, with a wider error than men in contact have; and "take a place" sends a group to look, chooses its units again on the first count, and is not launched until it holds the count its risk asks for, or says what it goes without

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Built, with two numbers of the hand calculation that the build does not give and one earlier test that changes, for the architect (As built)

## Why

A side learns the enemy's men in a hex only when its own men are there (decision 0092). A staff that plans a landing plans it blind: decision 0142 takes every unit the ships can carry and says it goes with no count. The designer's direction is that a commander who orders a place taken has said he needs to know what is on it.

## The grade of a count

1. **An estimate of the enemy's men in a hex has a grade,** kept with it and shown wherever the estimate is shown:

| Grade | Code | It comes from |
| --- | --- | --- |
| Told | 1 | A scenario's opening estimate: what the side was told before the game |
| Seen from the air | 2 | A reconnaissance flight (below) |
| Men in contact | 3 | The daily ground report of the side's own men in the hex (decision 0092) |

2. **A new report replaces the estimate,** whatever its grade, as now. The one case in which a worse report could replace a better one on the same ground is closed by item 6.
3. **It is state:** in the hash and named, in the saved game, and in a side's view of its own estimates. The enemy never sees it.

## The reconnaissance mission

4. **The mission.** An air group may be given the mission `reconnaissance` with a hex of the map, by the mission order. It is refused for a group whose type cannot search, as a search mission is.
5. **When it flies.** In the search phase of the morning pulse, once a day, where the group has an aircraft able to fly, a base or a deck to fly from by the rules of a search, and the hex within its type's search range. One aircraft flies. It costs the supply of one search sortie of twice the distance, and counts for the group's fatigue as one search sortie does.
6. **It does not fly** to a hex where its side has men of a fighting unit ashore: they see more than it would, and their report is the better one.
7. **What it reports.** The count is the true count of the enemy's men ashore in the hex, times an error, cut to a whole man. The error is drawn as a whole number of millionths between `ground.toml: reconnaissance_error_low` and `reconnaissance_error_high`, both included. The live values are 0.25 and 2, placeholders: wider than the 0.5 and 1.5 of men in contact. With no enemy man there the count is 0. The flight is made only if both figures are given: the frozen step 1 copy gives neither, so nothing built before this changes.
8. **The draw** is its own stream, keyed on the day, the morning pulse, the intelligence subsystem, the new draw `Reconnaissance`, added at the end, and the air group.
9. **What it does.** The report goes through `BeliefState::apply_report`, as every belief does: it makes or replaces the side's estimate for the hex, at the grade seen from the air, dated that day. It puts on the side's map every enemy base in the hex that is not on it, as a ground report does.
10. **The events.** `ReconnaissanceFlown`: the group, the hex, the distance and the supply spent, citing the mission order. `GroundReportReceived`, as now, with the grade, citing the flight; its breakdown (`true_men`, the two figures, the error) is the umpire's. `BaseFound` as now.
11. **Who sees it.** The side that flew sees both events whole but for the breakdown. The enemy sees nothing: being seen, and what the enemy makes of it, is left out.
12. **Nothing meets it.** The flight is not intercepted and no gun fires at it in this step.

## The plan's need to know

For a "take a place" whose plan is preparing.

13. **The count the risk asks for:**

| The risk | It may launch on |
| --- | --- |
| Preserve the force | An estimate of the place seen from the air or by men in contact, dated no more than 3 days before the morning |
| Balanced | An estimate seen from the air or by men in contact, of any day |
| Press | Anything, or nothing |

The 3 days are a placeholder.
14. **Who is sent.** On each morning the need is not met, and the side has no fighting man ashore in the place's hex, the staff sets one air group to reconnaissance of the place, where it is not set so already: the lowest-numbered group the intent's forces limit names, read up the tree, that can search, that the side may use and the commander does not hold, that no higher task has taken, and whose range reaches the place from where it is based. At press none is sent.
15. **When the need is met** the group is set to rest, by one order, and is the staff's to use again.
16. **The choice made again.** On the first morning the staff holds a count seen from the air or by men in contact, while the plan is preparing and no unit is aboard for it, the units are chosen again by decision 0142, items 8 to 10, on that count. Where the choice differs, the forces limit of the task "ready a landing" is set to it, and that task then orders the landing again with the new roster, at the cost the rules give a change (decision 0095).
17. **An estimate of 0** asks for one unit: the first in the order.
18. **Not launched without it.** The task is ready, for the leave and the release of decision 0142, item 15, only when the need is met as well.
19. **Where nothing can look,** because no group fits item 14: at preserve the force the plan is not released, and on the first morning the landing is otherwise ready the staff hands in the notice that it cannot meet the intent, with what it lacks, and gives the reason each morning after; at balanced it goes, and the reason of the release says that it goes on no count, or on what it was told.
20. **Reasons** give the estimate with its grade and its day, the count the risk asks for, the group sent with its range and the distance, and what the new count changed.

## What it leaves for later

As decision 0139 has it: patrols and men put ashore to watch; how hard a reconnaissance is pushed; being seen and what the enemy makes of it; harassment; coastwatchers. And a flight that is met by fighters or guns, more than one look a day, and a look at ships in a harbour.

## Rule card

- **Reads:** the truth of the enemy's men ashore in one hex, for the report only; the air group, its base and its type; the two figures.
- **Writes:** the grade on an estimate; one mission; the side's estimate, through a report; the base's supply and the group's fatigue by the rules of a search sortie.
- **Reuses:** the ground report and `apply_report`; the search mission's checks, range, supply and fatigue; `BaseFound`; the mission order.

## What it costs

- **State:** the grade of an estimate and one mission, in the hash and named. One draw, added at the end. The state layout, saved game, agent interface and doctrine versions rise, and the rules label rises: a game with the two figures and the mission plays differently.
- **Two figures** in `ground.toml`, live only.
- **The rulebook:** chapter 3 (the grade, the flight's report), chapter 5 (the mission), chapter 8 ("Taking a place": the need to know), the glossary.

## The hand calculation

`tests/scenario_72_reconnaissance.rs`.

## As built

Built on Nov 17, 2026, on the branch `stage3-step6`, with decision 0148. Nothing is tuned and no figure of the frozen step 1 set changed; the two live figures are added to `data/reference/ground.toml` (0.25 and 2, placeholders). The test of scenario 72 (`tests/scenario_72_reconnaissance.rs`) asserts every number and property of the hand calculation it can: 26 tests pass, and two are ignored on a mismatch with the hand calculation (below). The main game gives the orders, commands and notices of the table: the flight of day 1 and no other, 150 men seen from the air, unit 1 chosen alone on day 2 on 450 men wanted, the notice and release on day 3, the load to 45 t, seven orders and five staff commands, none refused.

### Mismatches with the hand calculation

1. **Day 2: "the loading is cut to 50 by the change".** Decision 0095, item 10 (and chapter 7, Preparing a landing) costs loading only for a unit added to a roster: a unit dropped leaves at no cost. On day 2 unit 2 is dropped and nothing added, so the loading stays at 100. The landing is still not ready on day 2, since the staff orders the landing again that morning (a landing ordered that morning is not ready that morning, as the doctrine of decision 0141 already held), and is ready on day 3 as the table has it; nothing else of days 1 to 4 differs. Test `day_2_the_loading_is_cut_to_50_by_the_change_and_gains_to_100`, ignored.
2. **The property "The grades", day 1.** With an opening estimate of 500 at preserve the force the men wanted are 1,500, and the two transports carry 1,200. By decision 0142, item 10, at preserve the force a shortfall forms nothing, asks no leave and hands in the notice that it cannot meet the intent: so day 1 forms no landing (the group is sent all the same, so that a count can come, and flies on day 1), and the landing is formed on day 2 on the count of 150, with unit 1 alone, a day behind the main case. Test `the_grades_day_1_chooses_the_same_two_units_on_1500_men_wanted`, ignored; `the_grades` asserts the grade told, the group sent on day 1 and the estimate replaced by 150 seen from the air.

### What was built

- **The grade (`crates/sim-core/src/belief.rs`).** `GroundGrade` (told 1, seen from the air 2, men in contact 3) on `GroundEstimate` and `GroundReport`; a scenario's opening estimate is told, the ground report of decision 0092 men in contact. Hashed after the estimate's event and named; state layout `flagplot-state-v43`.
- **The mission and the flight.** `AirMission::Reconnaissance(Hex)`, code 6 (`crates/sim-core/src/orders.rs`), refused with the new reason `AircraftCannotSearch` for a type with no search range, and for a hex off the map (`resolve.rs`, the mission order). `World::reconnoitre` (`crates/sim-core/src/reconnaissance.rs`, new), called in the morning pulse's search phase after the searches (`resolve.rs`): items 5 to 12, its draw `Draw::Reconnaissance` (code 12, at the end), the events `ReconnaissanceFlown` and `GroundReportReceived` with its grade, and `BaseFound` (shared with the ground report, `estimate.rs`, `find_hidden_bases`). `side_events.rs`: the flight is the flying side's whole, the enemy's nothing; the report's breakdown stays the umpire's.
- **The two figures** in `refdata.rs` and the loader (`crates/sim-data/src/reference.rs`), both or neither; the frozen step 1 `ground.toml` has neither.
- **The staff (`crates/sim-staff/src/look.rs`, new, and `take.rs`).** `knowledge` (the count held and the need of the risk, item 13), `look_orders` (items 14 and 15), `choose_again` (items 16 and 17), `need_to_know` (items 18 to 20), called from `prepare_place` and the launch of "take a place". `ready.rs`: a landing ordered again this morning has its projected day no earlier than tomorrow, as its reading already was.
- **Plumbing:** the scenario file's `set_air_mission` takes a `hex` (`crates/sim-data`); the saved game (version 18, `crates/sim-cli/src/save.rs`); the replay export and `sim-cli run` show the grade; the agent's mission order, briefing (`reconnaissance_of`, and an estimate's grade as `came_from`) and schemas (interface version 22); the viewer's words and `viewer/check.js`; `docs/commands.md`.
- **Scenario 72** (`data/scenarios/72_reconnaissance.toml`) on its own ground figures (`data/reference/ground_placeholder_scenario_72.toml`, frozen beside the others in `data/reference/step1/`), so that run by itself it plays the game its test proves.
- **The rulebook:** chapter 1 (the hash, the saved game), chapter 3 (the grade, the flight's report), chapter 5 (the mission), chapter 8 (Taking a place: the need to know; the versions), the glossary. Rules label 19, doctrine `supply-29` (with decision 0148).

### Readings taken

1. **The figures:** one of the two without the other is an error of the loader, as for `report_error`; with neither, no flight is made.
2. **The mission order** is refused for a type whose search range is 0 (`AircraftCannotSearch`) and for a hex off the map (as a waypoint off the map is).
3. **The flight:** the range is the type's search range against the distance; the supply spent is the cost of one search sortie of twice the distance (0 with no air supply figures); it is noted as a search sortie of one aircraft, so that wear, fatigue and supply pay for it; flights are made in group order after the day's searches; "men of a fighting unit" are men ashore of a unit that is not working.
4. **Who is sent:** the groups the forces limit names are its `air_groups`, read up the tree, and not the groups aboard a task force it names; the group is the lowest-numbered of them that can search, may be used, is not held, is not taken that morning by a cover, raid or strike, and whose range reaches the place from its base. A group already on reconnaissance of the place counts as sent. At press the staff sends none and sets none to rest. When the need is met every such group of the limit on reconnaissance of the place is set to rest.
5. **The order** goes after the "ready a landing" task is served that morning, so a morning's landing order comes before the group's, as the table has it; the group is sent also where the landing could not be formed (a shortfall at preserve), so that a count can come.
6. **"The first morning a count is held"** is the morning after the count's day: each new count of grade 2 or 3 makes the choice again while the plan is preparing and no unit is aboard.
7. **At preserve, a count that asks for more men than the ships carry** leaves the limit as it is, and the launch is held by decision 0142, item 10.
8. **The side's own fighting men ashore with no count** are read as "nothing can look", with words of their own.
9. **The need is read after the check that the place can be held** (decision 0144): a landing "otherwise ready" is one that passes that check too, so where the check stops it at preserve its own notice is the one handed in.
10. **The release's reason** ends "It goes on no count of the enemy", "on what it was told" or "on the count held", with the estimate's grade and day.

### Earlier tests, baselines and games that change

No expected number changed.

- **`tests/scenario_67_taking_a_place.rs`, `an_escort_at_preserve_the_force`:** at preserve the force with no count and no air group in the forces limit, nothing can look (item 19): plan 1 is never released, the notice that intent 1 cannot be met is handed in on the morning the landing is otherwise ready, and the escort, route and unload of day 8 are not given. Ignored with that reason, for the architect.
- **Scenario 73's property "Preserve the force"** (decision 0148): the same cause, in its As built.
- **Run baselines** differ from main beyond versions and hashes only in words: a ground report's line and an estimate's line name the grade ("(men in contact)", "(told)") in about 28 baselines (07, 23 to 25, 28 to 30, 32 to 34, 37, 38, 45, 68, and the events of 67 and 69), and the release and waiting reasons of scenarios 67 and 69 add "It goes on no count" or "on what it was told". No order or event changed.
- **Fixtures:** the version pins of the saved game (17 to 18) and agent interface (21 to 22); `NOT_IN_THE_FULL_SLICE` gains `ReconnaissanceFlown`; the count of side event kinds (147 to 148) and of air group order kinds (`sim-agent/tests/play.rs`); event patterns in scenarios 24 and 38 gain `..` for the grade; the two tests that compare every baseline with a run with no staff leave out scenarios 72 and 73. The recorded games differ in their versions and day hashes, and the one holding an estimate (`full_slice_five_days_sonnet`) in the estimate's `came_from` (`crates/sim-cli/tests/recorded_games/README.md`).

### Files to read

- `crates/sim-core/src/reconnaissance.rs` (`reconnoitre`, line 46), called at `crates/sim-core/src/resolve.rs:310`; the mission order at `resolve.rs:1114`; `belief.rs:74` (`GroundGrade`).
- `crates/sim-staff/src/look.rs` (`knowledge` 155, `look_orders` 234, `choose_again` 351, `need_to_know` 439); its calls in `crates/sim-staff/src/take.rs` at lines 263, 1199, 1224 and 1343.
