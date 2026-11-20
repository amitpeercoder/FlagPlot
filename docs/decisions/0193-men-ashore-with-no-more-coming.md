# 0193: Men ashore with no more coming

Status: a plan by the architect, written on Nov 20, 2026, on the designer's word ("plan and build 1 and 4 next"). To be read by a reviewer who has not seen it written, and amended on that reading, before any of it is built.

Summary: Men landed to take a place attack only when they are as many as the risk asks for against the enemy they count: twice his number at balanced. When they are fewer, the staff waits for a second lift. When there is nothing more to send, it waits for ever. This gives the staff a judgement for that case: with no more coming, a balanced staff attacks if its fit men are at least the enemy's, a pressing staff attacks whatever the count, and a careful one waits and tells the commander.

Standing: Planned on Nov 20, 2026, amended on review and built on Nov 20, 2026 ("As built"); the readings marked there, and statement 9 of scenario 109, are the architect's to confirm.

## Why

Scenario 105 (decision 0188). 5,386 men landed at Port Moresby. They counted 3,444 defenders. At balanced the staff wanted 6,888, had 5,116 fit, and waited. No second lift existed. They would have waited as long as the game ran.

What is so today (chapter 8, Taking a place, Ashore and A second lift):

- **The men wanted** are the estimate of the enemy's men at the place times 3 at preserve the force, 2 at balanced, 1 at press.
- **The fit men** of a unit are its men ashore times (100 less its fatigue) over 100, cut to a whole man. A unit can attack if its fatigue is at or below the risk's mark.
- **Attack** where no estimate is held or the fit men of the units that can attack are at least the men wanted. **Otherwise wait.**
- **A second lift** is sent while units are left behind at the source and the transports lie there empty. "Where the men are short and none is left, the reason says so and the staff waits."

## The rules

1. **Nothing more is coming** to a place when, on the morning of the judgement: no fighting unit is left behind for a second lift; no transport force of this take has fighting men aboard, whether under way, turned back (decision 0191) or waiting; and no unit chosen is waiting for its men. It is worked from the side's own forces and plan, and the reason lists what was looked for.
2. **With nothing more coming, the men wanted fall,** by the risk read up the tree:

   | | Preserve the force | Balanced | Press |
   | --- | --- | --- | --- |
   | Men wanted while more is coming (today) | 3 times the estimate | 2 times | 1 times |
   | Men wanted with nothing more coming | 3 times: it waits | 1 times | None: it attacks |

3. **Balanced:** the units that can attack are set to attack where their fit men are at least the estimate. Where they are fewer, it waits.
4. **Press:** every unit that can attack is set to attack, whatever the estimate.
5. **Preserve the force:** it waits, as today.
6. **Where it waits with nothing more coming,** the staff hands in the notice `cannot_take`, each morning that holds, which the standing interrupts may stop the clock on. Its reason gives the estimate and its day, the fit men, the men wanted and that nothing more is coming. A commander may then set press, send more, or withdraw the intent.
7. **Everything else of the judgement is as today:** which units count, which can attack, the fatigue marks, the reason's figures, and what happens with no estimate held.
8. **The figures are the staff's and are placeholders,** as the 3, 2 and 1 are.

## Rule card

- **Reads:** the take's plan, its lifts and its units; the units ashore, their men and fatigue; the side's estimate of the enemy's men at the place; the risk on the chain.
- **Writes:** nothing but the staff's existing orders to a unit (attack, defend), their reasons and a notice.
- **Reuses:** the judgement ashore, the fit men, the men wanted and the units left behind; notices and standing interrupts.
- **New:** one test, "nothing more is coming", one row of a table, and one kind of notice. No state, no draw.

## What it costs

- **Games in which men ashore were waiting with nothing more coming change:** at balanced and at press they may now attack. The stage 3 proof's scenarios and the 39 days are checked, and any daily hash that moves is reported with the reason before golden files are remade. The rules label rises.
- **An attack at even numbers against men dug in may fail.** That is the commander's risk at balanced; at preserve the force nothing changes.
- **Rulebook:** chapter 8 (Taking a place, Ashore) and the glossary; the agent interface's notice kinds and its version.

## Scenario 105

9. **Nothing in its orders changes.** With this rule the Japanese men, 5,116 fit against 3,444 counted, attack at balanced on the first morning nothing more is coming. The land fight at Port Moresby then scores men lost on both sides. Whether the place falls inside the game is reported from the run.

## Amended on review

A reviewer read the plan against the take's code and ran the scenarios that have a take. Found right: the arithmetic, the fit men cut to a whole man, the men wanted, that only the side's own estimate is read, and that the table sits with the choice of units and with "not to starve".

10. **"Nothing more is coming" is replaced.** As written it missed men who come later: in the 39 days a take waits from day 11 to day 36 with no unit left behind while nine forces may be used from later days, and the rule would have told the commander, falsely, that nothing was coming. More is coming where, counted as "not to starve" counts what is in reach: a lift as it stands of this take has fighting men aboard; or a fighting unit of the take's forces limit that is not ashore at the place can be used and carried by the end's last day (its date, or today and seven days where it has none). A unit before its day counts from its day. A unit left behind that no ship of the take can ever carry does not count: the lift's force is lost, or was sent on to a hold's source, or "not to starve" forbids the lift.
11. **No new notice.** The notice `cannot_bring_enough` already means that the take cannot bring the men it wants on any day in reach. It is handed in for this case too, on each morning it holds, and its standing interrupt serves. The rule adds no kind and no state.
12. **Once sent in, they stay in.** The estimate of men in contact is drawn again each day and swings widely. With nothing more coming, a unit that is on attack is not set back to defend by the staff because that morning's estimate is larger; the engine's own rule sends a spent unit back, and it is judged again when it can attack. So the test of rule 3 decides when they go in and not, morning by morning, whether they stay.
13. **Today's rule also counts a unit on attack as able to attack,** whatever its fatigue; the plan left that out. A lift made at a base lands its units on attack; a lift taken as it stands lands them on defend.
14. **The worked example is corrected by the architect:** U and V are ashore on defend; the attack fatigue limit is the reference file's and the test states it; W is in the take's forces limit; statement 9 follows decision 0191; statement 4's and 6's notice is `cannot_bring_enough`; statements are added for rule 10 (a unit before its day is coming; a unit no ship can carry is not) and rule 12, with three properties: more fit men never turn an attack into a wait; a larger estimate never turns a wait into an attack; a lift added never lowers the men wanted.
15. **What moves:** of the scenarios with a take, the stage 3 proof's first file on days 37 to 39, where both takes wait with nothing coming and their fit men are below the estimate: new notices and no attack. The Coral Sea on day 5.
16. **For scenario 105 it is one day's fight.** The first judgement ashore is on the last day in every one of forty games; in 29 of them this rule sends the men in. Port Moresby will not fall inside the game.

## As built

Built on Nov 20, 2026 on the branch `men-ashore-no-more-coming`, to rules 1 to 8 as amended on review (10 to 16). Scenario 105 is not changed.

**What was built.**

- `crates/sim-staff/src/take/coming.rs`, new: the test "more is coming" (`more_coming`), the men wanted with nothing more coming (`men_per_man_with_nothing_coming`: 3, 1 and 0, placeholders), and the judgement as a pure function (`verdict`: attack, wait or committed), with unit tests of the worked example and of the six properties.
- `crates/sim-staff/src/take.rs`, the judgement ashore (`ashore`): where an estimate is held and the fit men that can attack are fewer than the men wanted, the test is worked; with nothing more coming the reason adds what was looked for and the men wanted, the units attack, wait or stay in, and where it waits the staff hands in `cannot_bring_enough`. Nothing else of the morning changes: the second lift, the later lifts and "not to starve" still read the men wanted while more is coming.
- `crates/sim-staff/src/lib.rs`: the doctrine version `supply-46`. `crates/sim-core/src/world.rs`: the rules label 24. No state, no event, no draw, no order and no notice kind is added; the state layout, the saved game's version and the agent interface are as they were.
- Scenario 109: `data/scenarios/men-ashore-no-more-coming/109_men_ashore_with_no_more_coming.toml`, on scenario 107's placeholder map, in a folder of its own (decision 0072), with a ground figures file of its own (`data/reference/ground_placeholder_scenario_109.toml` and its frozen copy in `data/reference/step1`: the frozen figures with `rest_per_day` 0, `unfed_loss_share_per_day` 0, `preparation_per_day` 100 and no report error, so the opening estimate stands). The tests are in `tests/scenario_109_men_ashore_with_no_more_coming.rs`, under the architect's statements.
- The rulebook (chapter 8, Taking a place, Ashore, items 4 to 10, A second lift, the notice's row and the standing interrupt's row, the versions), the glossary ("men wanted" and "more is coming", new; "cannot bring enough"), `docs/rules/differences.md` rows 50 to 52, `docs/commands.md`, `docs/open-work.md`, and the recorded games' saves (the rules label and the doctrine version; no transcript changed).

**Readings made in the build** (the architect's to confirm):

1. **When it is worked.** Only on a morning an estimate is held and the fit men that can attack are fewer than the men wanted while more is coming. On any other morning the judgement and its reason are as before.
2. **The lifts of the take** are its first lift and the forces the forces limit of its "ready a landing" names. One with fighting men aboard a ship afloat is more coming, whatever its route: under way, turned back or waiting.
3. **Later lifts** are counted as "not to starve" counts them: a lift as it stands the end may send, with fighting men aboard, that may first be used by the end's last day. A lift the commander holds is left out of them, as before, and so is not coming.
4. **A unit or force before its day counts from its day.** With a date, it is coming where its day is no later than the date. With no date it is always in time: the staff's seven days are counted from its day. Read with today and seven days, the stage 3 proof would have moved on days 13 to 15, where the next force to come may be used from day 23; the review said it must not move before day 36 (`docs/rules/differences.md` row 50).
5. **A unit of the forces limit** is one chosen or one of the end's limit as it binds. It is coming where it is a fighting unit with men ashore, not at the place, with no man aboard, not held, ashore at the source, in time, and carried. **Carried:** the first lift has a ship afloat and is bound for the source (the last hex of its route, or where it lies). Where it lies stopped at the source, its ships must berth the units by the choice of a second lift, and at balanced "not to starve" must not forbid that lift. A unit ashore elsewhere than the source is not coming: no rule brings it.
6. **Once sent in, they stay in** holds, at balanced and at press, for a unit on attack that the staff sent in under this rule, and no notice is handed in then. The staff keeps no state: it reads its own reasons in the side's log. A unit was sent in under this rule where the staff's last order to it that the log holds is an attack whose reason is bound "sent in with nothing more coming". Any other unit on attack, a unit landed on attack by a lift made at a base among them, is set to defend as before. A first build kept every unit on attack in, and scenarios 68, 95 and 97 showed a unit landed on attack and beaten back left in to attack again, which the record does not ask for (`docs/rules/differences.md` row 51).
7. **The notice** is handed in on each morning it waits with nothing more coming, once a morning for the plan. Its standing interrupt fires once for the plan, on the first such morning, as the table of standing interrupts has it (`docs/rules/differences.md` row 52). At preserve the force, where units on attack are set to defend, the notice rests on a reason of its own beside the orders.

**The tests.** Every statement and property of scenario 109 has a test: 22 pass and 1 is ignored. Nine unit tests are added in `sim-staff`.

- **Statement 9, ignored:** a lift turned back cannot be built while men of the take are ashore. Decision 0191 turns back only the first lift before any of its men are ashore, and a scenario's contact is too old by the morning of the judgement. A force the scenario routes to its source is held by the commander's order and so not counted. The code counts a lift of the take with fighting men aboard whatever its route (reading 2), so the statement holds as built, but it is not proved in play.
- **The morning M** is day 4: the lift sails at press on day 1 (at balanced "not to starve" holds it, with no hold of the place) and the risk is balanced from day 2.
- **Statement 1** ("today") is proved by the reason's 6,888 wanted and that, with more coming, the staff waits each morning with no notice. No switch is in the code.
- **Statement 7:** on M the transports are on their way home, and the reason says the unit left behind is to be sent when they lie at the source; on day 6 they do, and it is sent. The staff waits with no notice on both mornings.
- **Statement 9b** is judged on day 5: the transports are sunk on day 4 after the landing, so on M more is coming and on day 5 nothing is.
- **Statement 9c** is exact: an Allied landing brings the true count to 9,000 and a report error of one gives that estimate the next morning. U loses a man a day unfed once ashore, so the fit men that day are 5,085.
- **Statement 12:** the interrupt stops the day once, on day 4; the notice is handed in on days 4 to 7.

**What it moved.** Every scenario under `data/scenarios` (the step 2 frozen copies left out) was run with the rule and without it on the same build (a switch not committed), and the output of `run --events` compared. Three moved:

- `data/scenarios/stage-3-proof/07a_slice.toml`, the Japanese side. Day 36, the take of (3,-5): its last lift as it stands, task force 38, was sent that morning by the take of (2,-4), higher in priority, so nothing more is coming to (3,-5). Days 37 to 39, both takes: nothing more is coming, and the fit men (4,403 to 4,435 at (2,-4) against estimates of 8,787 to 12,022; 1,200 at (3,-5) against 3,614 to 8,496) are fewer than the estimate. Each morning a new reason and the notice `cannot_bring_enough`; no attack. Days 11 to 35 do not move. The daily hashes of days 36 to 39 change.
- `data/scenarios/coral-sea/01b_coral_sea.toml`, day 5, the last: the Japanese men at (-17,-4), 5,116 fit against 3,444 counted, are set to attack at balanced. The other three Coral Sea files do not reach a judgement ashore on their seed (in 01a the lift is turned back).
- Scenario 109, its own.

No run baseline, golden file or pinned hash changed; the recorded games' saves changed in their rules label and doctrine version alone.

**Existing tests that changed.** Two tests of scenario 68, the architect's hand calculation, fail under the rule as it is meant and are ignored with the reason, for the architect to restate; no expected number was changed:

- `nothing_left_to_send` (`tests/scenario_68_ashore.rs`, the assertion of all the orders): with unit 3 held nothing more is coming, so on the morning of day 8 units 1 and 2, 600 fit men, are at least the estimate of 600 and are set to attack at balanced. Two orders follow the ten it expects: `(8, Attack(1))` and `(8, Attack(2))`.
- `a_hold_is_honoured` (the assertion of all the notices): with unit 2 held from day 7, nothing more is coming and the men are too few, so `cannot_bring_enough` is handed in on each morning from day 8 to day 14, not on day 8 alone (rule 11).

**Not done.** Statement 9 in play. Scenario 105's own tests and its forty games were not re-read for rule 16 beyond the full test run.

