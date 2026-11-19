# 0176: The estimate: what a trial says of a course, in counts of worlds

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 6 of its ten, section E, rules 13 to 15), which the designer confirmed. It was read by a reviewer who had not seen it written, and amended (below), before any of it was built. The hand calculation is written. Built on Nov 19, 2026 (As built, below), on decision 0175's build.

Summary: A trial plays a course in a number of worlds. The estimate is what is read from them: in how many worlds the end was met, and by what day; in how many it was met and lost again; in how many it was not met; what it cost in men, ships and aircraft, as the lowest, the middle and the highest; what the trial's enemy lost; what was assumed; and where the worlds part by a thing the side did not know. It speaks in counts of worlds and never in chances, and in places, days, men, ships and aircraft and never in points. With it comes one thing the trial lacked: a way to say that the men at a place may be more than the side's estimate, and not known

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161, and amended on review. Built on Nov 19, 2026. Its case "a plan displaced" waits for the architect to state it, and its means for "believed men are set aside" for his ruling

## Why

Decision 0161, rule 13, lists what an estimate gives. Its case to be worked by hand is "a landing of 600 men against a place whose garrison is believed to be 200 or 600, not known which, four worlds: taken in the two with 200 by a stated day, not taken in the two with 600". A side's belief of the men at a place is one number (decision 0092), and the three standings of decision 0173 are for ships. Nothing built can say "200 or 600".

What is so today, read from the code:

- **A trial gives back, for each world,** its number, its assumptions, each day's hash and staff hand-ins, the commands of the course it refused, and the world as it stands at the end (decision 0175, amended, 10). `sim-trial` may not read a view of a world: its source test forbids the word.
- **The day a place changes side is an event both sides see** (`BaseChangedSide`), on the evening of the fight. The staff marks a take's plan complete and hands in "met" the morning after. Nothing records "met and lost again"; a base that changes side back is another such event. A plan is also marked complete when its intent is withdrawn, so a plan's state does not say that an end was met.
- **A side sees its own losses exactly,** as events: men lost in a fight, to fire, unfed, with a ship, by surrender; ships sunk or lost with a base; aircraft lost in the air, on the ground, ditched, worn out, with a ship or a base. The list the objectives count is in `objectives.rs`. A ship out of action is no state: it is worked at the end from the ship's days of repair (decision 0088).
- **The plan order** of decision 0159 gives, for an intent, the men wanted and to hand, what it waits for and the day expected ashore. It is worked from a side's view by the staff.
- **The house rule for a middle value** is the value at place ceil(n / 2), counting from 1, never an average of two (`docs/step2-spec.md`).

## The rules

### Men believed at a place and not known

1. **`[[believed_men]]`** in a scenario states men of the enemy's that a side believes may be ashore at a place beyond those it holds an estimate of. Its keys: `side`, the side that believes it; the place, `at_base` (a base on that side's map) or `q` and `r`; `men`, 1 or more; `standing`, `present`, `absent` or `unknown`; and `placeholder` with a `note`, as a believed force has. It is loaded, checked and kept as a believed force is (decision 0173, 8 and 9), and is no more state or belief than one.
2. **In a world of a trial** they are one more fighting unit of the enemy's in that hex, made as the unit of an estimate is (decision 0174, 6): in every world where present, in none where absent, and where unknown in the worlds its turn gives it (decision 0174, 14). The unknown entries take their turns after the believed forces of unknown standing, in the order of the scenario's list. The assumptions name the unit, its men and its standing.

### What an estimate is read from

3. **An estimate is of one intent of the side tried,** named by its number, in a trial's worlds as played. It may be given a second trial of the same worlds with an empty course, things as they stand, to compare with (rule 12).
4. **The end of an intent is a place being the side's.** For "take a place", its hex. For "control an island", every named place of the island. For "hold", its base. The end is met in a world on the first day at whose end the side holds a base at each of those places and the enemy none; it is read from the events of a base changing side that the side itself sees, and from who held the place when the world was built. An intent of another kind has no end an estimate can read: its estimate gives the costs and says so.
5. **Met and lost again:** the end was met and, on a later day of the horizon, a place of it changed side away from the side. Such a world is counted apart from the worlds where it was met and kept.
6. **`sim-trial` reads a world it played** through the views of its two sides, in the one part of the crate that makes estimates, and nowhere else. A world of a trial is the trial's own invention (decision 0175, 7): reading its enemy's view is not reading the enemy.

### What it says

7. **How it went,** in counts of worlds: the worlds played; of them, those in which the end was met and kept, with the earliest, the middle and the latest day it was met; those in which it was met and lost again; and those in which it was not met by the horizon. A world that could not be played is counted and named, and is in none of the others.
8. **What it cost the side,** over the days of the horizon, each as the lowest, the middle and the highest over the worlds played: men lost to the enemy (in a fight, to fire, with a ship, by surrender); men lost unfed; ships sunk (and lost with a base); ships out of action at the end, a ship afloat whose days of repair are 1 or more; aircraft lost, by the list the objectives count. Each is the side's own count from its own events and its own ships.
9. **What the trial's enemy lost,** the same five, read from the other side of each world, and marked as the trial's own reckoning: it is not a claim, not a report, and changes no belief.
10. **The middle** is the value at place ceil(n / 2) of the worlds' values in order, counting from 1.
11. **What it needs and has, and when it could go:** the plan order of the intent, as the side's staff works it from the side's view of world 1 at the end of day 1: the men wanted and to hand, what it waits for, and the day expected ashore. The side's half is the same in every world, so world 1 stands for all.
12. **What its other plans lost.** Where a trial of things as they stand is given: for each other intent of the side's own that has an end (rule 4), the worlds in which it was met with the course and without. A plan met in fewer worlds with the course is named as displaced, with the two counts.
13. **What was assumed:** every assumption of the worlds' pictures, each once, with the number of worlds it was made in; and any leave the course gave that the side had not given.
14. **Where the worlds part.** For each force or body of men of unknown standing: the worlds with it and the worlds without, each with its counts of rule 7 and its middles of rule 8, side by side. It is shown where the two differ in any count or middle, and left out where they do not.
15. **No chance, no points, no advice.** An estimate gives no share of worlds as a chance, no score, and no word on what to do. It is data, with a rendering in words for a reader, and reading it changes nothing.

## What it leaves for later

- **Where an estimate is kept and who is shown it:** the session and the seat's tool, step 8. **Dates side by side:** step 7.
- **Whether the estimate is true:** the spreads and the wrong pictures of decision 0161's proof (conditions 7 and 8), and the reader who is given an estimate alone (condition 11).
- **The end of an intent that is not a place:** a base kept supplied, waters denied.

## Rule card

- **Reads:** a trial's worlds as played, through their two sides' views; the plan order of the side's staff; the pictures' assumptions.
- **Writes:** nothing in any game. It returns an estimate.
- **Reuses:** the trial of decision 0175; the believed forces' standings and turns of decisions 0173 and 0174; the plan order of decision 0159; the losses the objectives count.
- **New:** `[[believed_men]]`; in `sim-trial`, the estimate and its words.

## What it costs

- **No live game changes:** no state, no draw, no hash.
- **`sim-trial`'s source test** lets the estimate's own module take the views of a world the crate played.
- **The loader** gains one kind of entry; `docs/commands.md`, the rulebook's chapter on the trial and the glossary gain the estimate and its words.

## The hand calculation

It is in `tests/scenario_95_the_estimate.rs`, written before the code: decision 0161's own case. On scenario 68's figures, where every shot is whole and hits, a landing of 600 men at a place the side estimates at 200 and believes may hold 400 more: four worlds, the place taken on day 4 in the two with 200 and not taken in the two with 600, with what each cost counted man by man.

## Amended on review, Nov 19, 2026

The plan was read by a reviewer who had not seen it written, against decision 0161, decision 0175 as amended, the code and scenario 68. What follows is the architect's and replaces the rules above where they differ. The hand calculation is amended with it.

1. **What a trial must keep, beyond decision 0175's record.** A world's state at the end of a day is not kept, and an estimate needs two things of it. So the playing of a trial records, for the side tried, in each world:
   - **at the end of each day,** the hexes in which the side holds a base, and the hexes of the enemy's bases on its map;
   - **at the end of the first day played,** the side's plan orders, as its staff works them from its view.

   To record them the play takes the tried side's view of the world at the day's end, and reads nothing else of it. This is added to decision 0175's record of a world by this build.
2. **Met is the staff's own test** (replaces rule 4's reading). A place is the side's at the end of a day where the side holds a base in its hex and no base of the enemy's is there on its map: the test by which the staff marks a take met. It is read from the day's record of item 1, so that a place taken by a landing where the enemy had no base is met as one taken from him is. The end of "take a place" is its hex; of "control an island", every named place of the island; of "hold", its base being the side's. It is one function, so that the proof can apply it to a live game.
3. **Days are the world's own day numbers.** An end already met when the trial begins, such as a hold of a base the side holds, is met on the first day played.
4. **Kept, and lost again** (replaces rule 5). The end is met and kept where it is met at the end of the horizon; its day is the first day it was met. It is met and lost again where it was met at the end of some day and is not at the end of the horizon. A world that met its end on the last day counts as kept, and nothing tried it after: the estimate gives the latest day met so that this can be seen.
5. **The plan order is world 1's** (replaces rule 11), as recorded at the end of the first day: the men wanted and to hand, what it waits for or that it waits for nothing, the day expected ashore, what must be known, and its tasks. Where another world's plan order differs, the estimate says in how many it does. The lift and the weight of supply that decision 0161, 13, also names are in the staff's orders and not in a plan order; they are left for the step that shows an estimate to a seat.
6. **Believed men are set aside when the side knows better.** They are not made in a world where the side holds an estimate of that hex of a day after the scenario's start, a count it has had since, or where the place is the side's by item 2. The assumptions say so. A believed body of men has a `name`, as a believed force has, for the estimate to call it by.
7. **The loader refuses** believed men with no place, with a base that is not on the believing side's map, and at a hex that is sea, as well as the faults of rule 1.
8. **Where the worlds part** (replaces rule 14): by each force or body of men of unknown standing, and by each pair of reports that is one force in some worlds and two in others (decision 0161, 7). Each side of a parting has its count of worlds, its counts of rule 7, and for each of the five costs, the side's and the enemy's, the lowest, the middle and the highest. A parting is shown where its two sides differ in any of those.
9. **What was assumed is grouped by what it is of,** not by the numbers given to the things made: an enemy base, the men of an estimate, a believed force or body of men, a contact, a pair. Each is one line with the worlds it was made in and, for a thing of unknown standing, the worlds it was not; a force placed by reach has the count of worlds it was placed in and no list of hexes.
10. **An estimate keeps each world's own figures:** its number; the day its end was first met, or none; whether it was lost again; the five costs of each side; and what its picture assumed. The lowest, middle and highest are read from them, and so can a comparison of two courses, world for world.
11. **What its other plans lost** (replaces rule 12): for each other intent of the side's with an end, the worlds in which it was met and kept with the course and without, and the middle day met of each. It is named as displaced where it is met in fewer worlds or its middle day is later. Whoever asks for an estimate asks for the second trial; the seat's tool will.
12. **Ships out of action** are the side's ships afloat at the end of the horizon whose days of repair are 1 or more, a ship in a yard among them.
13. **No world played:** an estimate of a trial none of whose worlds could be played says so, with the count, and gives no day and no cost.
14. **The source test of `sim-trial`** lets two parts of the crate take a view of a world the crate built: the play, to record item 1; and the making of an estimate. Nowhere else.
15. **The words** are for a reader. The data is what is asserted: the test checks the estimate's fields exactly, and of its words only that they name the counts of worlds and hold no chance, per cent or points.

## As built

Built on Nov 19, 2026, on the branch `the-estimate-build`, from the head of `a-trial-build` (decision 0175's build, pull request 536), with main merged in, to the record and hand calculation as amended on review. No number or statement of the hand calculation was changed. Every statement of the main case holds as written, at the first run; the engine parts from the hand calculation in one place, the means of one property (reading 1), and one property is built and reported, not asserted, as the hand calculation asks (reading 2).

**What was built.**

- **`sim-core`:** `believed::BelievedMen { side, name, hex, base, men, standing, placeholder, note }` with `enemy()`, plain data beside the believed forces. Nothing else: no state, hash, event or draw.
- **`sim-data`:** `[[believed_men]]` (`side`, `name`, `men`, `standing`, `at_base` or `q` and `r`, `placeholder`, `note`; no `day`), checked and kept as a believed force is (`Scenario::believed_men(side)`), kept by `with_seed`. Refused: no man; an unknown standing; no place; both a base and a hex; one of `q` and `r` alone; a base not of the scenario, of the believing side's own, or not on its map (`on_map` false); a hex off the map, or of sea (a hex that is not land or coast); a placeholder with no note; an unknown key.
- **`sim-trial`:**
  - `Believed` gains `men`. The enemy's half makes them after the believed forces (decision 0174's rules 14 to 17) and before rules 18 and 19; `check` refuses men that are not the tried side's. Four new assumptions, of decision 0176, rule 2 (`Assumption::decision()` is 176 for them, `rule()` 2): `BelievedMenMade { unit, place, name, hex, men, standing }`, `BelievedMenRuledOut { place, name, hex, men }`, `BelievedMenNotInWorld { place, name, hex, men }` and `BelievedMenSetAside { place, name, hex, men, standing, why }`, with `SetAside::{CountSince { day }, PlaceIsTheSides}`.
  - **What a trial now keeps** (amended 1), added to decision 0175's record: `PlayedDay` gains `bases: BasesAt { own, enemy }`, the hexes in which the side tried holds a base and the hexes of the bases on its map not its own (`view.enemy_bases()` whose side is not the side's), taken from the side tried's view at the end of each day played; `PlayedWorld` gains `plan_orders: Option<Vec<PlanOrder>>`, the side's staff's `plan_orders` of its view at the end of the first day played (`None` where no day was); `Played` gains `side: Option<Side>`, the side tried (`None` only for a trial given no picture). `BasesAt::of(view)` is public, for the met test on a live game.
  - **The estimate** (`src/estimate.rs`): `estimate(played, intent, as_they_stand: Option<&Played>) -> Estimate`; `met(end, bases) -> bool`, the one function of amended 2; `end_of(view, intent) -> Option<End>`; and the types `Estimate { side, intent, kind, end, asked, last_day, went, could_not, side_costs, enemy_costs, plan_order, plan_order_world, plan_order_differs_in, assumed, leaves_given, uneven, partings, other_plans, worlds }`, `End { places }`, `Went { worlds, met_kept, met_days, lost_again, not_met }`, `Spread { lowest, middle, highest }` with `Spread::of`, `Costs` (the five spreads), `Losses` (the five counts), `WorldFigures { number, met_day, lost_again, side, enemy, assumptions }`, `Assumed { of, made_in, not_in, notes }` with `AssumedOf` and `AssumedNote`, `Parting { by, with, without }` with `PartedBy` and `PartSide { went, side, enemy }`, `OtherPlan { intent, kind, with_course, without, displaced }` with `Kept { worlds, middle_day }` and `Displaced::{InFewerWorlds, Later}`. `Estimate`'s `Display` is its words.
  - **The source test** (amended 14): `side_view` and `command_state(` stay banned in every file but two. In `play.rs`, `.side_view(` may stand on one line, which records the day's end. In `estimate.rs`, `.side_view(` and `.command_state()` may stand on any line: the views of the two sides of a world played, and the side's own command state through its view. Each allowance is cut from its line before the other bans are checked, so the line is held to them all; a second line in `play.rs` fails the test. A world named in either file is one the crate built, since no function takes one.
- **Data:** `data/scenarios/95_the_estimate.toml` (scenario 68 with the changes of the inputs); `data/reference/trial_placeholder_scenario_95.toml` (200 t of supply, 0 t of fuel, seed 1942, decision 0175's sizes) and `data/reference/class_sides_placeholder_scenario_95.csv` (scenario 94's, every class and type to both sides), each with a frozen copy; scenario 95 names scenario 68's ground figures file, unchanged. All placeholders.
- **Tests:** `tests/scenario_95_the_estimate.rs`, 22 tests; `crates/sim-trial/src/estimate_tests.rs`, 5 tests inside the crate (met and lost again, a place met by a landing, a plan displaced by counting, the met test, the middle); the source test as above. Scenario 93's and 94's tests and `sim-cli time` hand in the believed men with the rest.
- **Generated:** `crates/sim-cli/tests/run_baseline/95_the_estimate.txt` and `.events.txt`, new. `sh scripts/regenerate.sh` changed no other file.

**Readings the record did not make.**

1. *The case of "believed men are set aside" parts from the engine.* The hand calculation says the build makes it "from the live game played to the evening of day 5 with scenario 68's course, where the day's report gives a count". On scenario 95's live game, with the course (unit 1 alone) handed in on day 1 and both staffs, the landing goes as in the worlds, and on day 4 unit 1's 600 men attack the live garrison of 1,200: 6 shots against 12, the weight of fire below the defence's, the attack fails (`GroundAttackFailed`, weight 6 against 12) and unit 1 loses all 600 men (`MenLostInFight`, `LandUnitDestroyed`). No man of the side is at B on any evening after, so no ground report is made, and on the evening of day 5 the side's estimate of B is still the opening one, 200 of day 0. The men are not set aside there. The test makes the case instead from world 2 of the main trial at the end of its play, a side's view in which the daily report of the men in contact gives 300 men at B of day 8: pictures made from it make the 400 in no world and say `SetAside::CountSince { day: 8 }`. The statement is unchanged; the means waits for the architect's ruling.
2. *"A plan displaced"* is built and run, and not asserted (`a_plan_displaced_is_played_and_reported`, run with `--nocapture` to read it). The figures are in the build's report to the architect.
3. *The plan order "waits for nothing"* is read as no thing waited for that is not met: world 1's plan order at the end of day 1 has an empty `waits_for`. It also has a need to know (a count asked, by the risk, not held: the opening estimate is told), which the estimate keeps with the plan order.
4. *The intent's kind and end* are read from the side's own command state in the first world played, through its view, at the end of play; an intent the side holds in no world played has `kind: None` and no end.
5. *A hold's end* is its base's hex, by the test of amended 2 (a base of the side's there and none of the enemy's), not the base's own side: a base of the side's own formed in the hex counts, and so does another base of the enemy's there against it.
6. *Ships out of action* count a ship in a yard always, and a ship in a force or a port where her days of repair (`repair::days_of_repair`) are 1 or more; with no surface figures, none in a force or port. Ships sunk count each ship once over `ShipSunk` and `ShipLostWithBase`. Every cost is counted from events the side sees whole (`SideEventKind::Whole`), which are its own; aircraft by the events the objectives count, with their counts.
7. *Believed men set aside take no turn* among the things of unknown standing, as believed absent ones take none: setting aside is the same in every world, so a turn would only pair worlds that do not differ. Absent is read before set aside. The estimate of an opening count (day 0) does not set them aside.
8. *What was assumed* groups lines by `AssumedOf`; a line's `made_in` and `not_in` are world numbers, and its notes carry counts of worlds (placed by reach, put at an enemy base, a contact taken for it, not made, a kind raised, ships or a group left out, ruled out, set aside). A pair's line is made in the worlds it is one force and not in those it is two; the force made for a pair is the pair's line, not a contact's. The leaves given are a field of their own (`leaves_given`) and a line of the words.
9. *Partings* are taken in the order the things first appear in the worlds' assumptions, and given only where each side has a world played and the two differ in the counts of met and kept, lost again and not met, the days met, or a spread of either side's costs. The counts are compared as counts, not as shares of their sides.
10. *Other plans* are the commander's intents (`Maker::Commander`) with an end, other than the one estimated, as the side holds them at the end of the first world played of the trial of things as they stand; each is met and kept, with the course and without, over each trial's worlds played. Displaced in fewer worlds is named before later days. Without a second trial `other_plans` is `None`; with one and no other plan, an empty list.
11. *The plan order of world 1* is that of the first world with a record of plan orders; `plan_order_differs_in` names every world whose plan order of the intent is not equal to it.
12. *The words* name each count as "n of m worlds" and the worlds by number, and render the plan order's tasks and the enemy's intents by the engine's own names where no words exist. They hold no per cent sign and neither "chance" nor "points".

**Documents.** Chapter 9 gains rule 15 of the enemy's half (believed men) and "The estimate" (rules 1 to 12), and its opening, what is built, the source's rules and what a trial gives back say so; chapter 1 names `[[believed_men]]`; the glossary gains "estimate (of a trial)", "believed men", "met and kept", "lost again" and "parting (of a trial's worlds)", and amends "estimate (of the enemy ashore)" and "standing"; `docs/commands.md` the new entry; `docs/engineering/layout.md` the crate's row; the frozen figures' README the two new copies. `docs/engineering/data-rules.md` lists nothing of a side's beliefs and is unchanged.

## The architect's rulings

Made on Nov 19, 2026, on reading the build.

1. **The readings of "As built" stand as made.**
2. **The hand calculation's means for "believed men are set aside" was wrong, and is corrected by the architect.** It had the live game played on with the course until a report gave a count. The live garrison is 1,200: the landing is destroyed on its first evening and no report is made. The case is made from a view that does hold a later count, the side's view of a world of the main trial at its end. The rule and its statement are unchanged.
3. **"A plan displaced" in a scenario is stated from the build's run,** in the test file: two takes of one place that share a force do not displace one another by anything an estimate reads, since the place is taken on the same day by whichever unit lands. The rule's own counting is tested inside the crate. A case of two places is owed with the next step.
4. **"It waits for nothing" stands beside a need to know that is not met.** A count told at the start is not a count seen, and at balanced the plan order asks for one; with nothing that can look, the take goes without. The estimate gives both, as the plan order does.
5. **The words name tasks and kinds as the engine prints them.** That is for a reader who is a programmer. Words for a commander are the next step's, which shows an estimate to a seat.

