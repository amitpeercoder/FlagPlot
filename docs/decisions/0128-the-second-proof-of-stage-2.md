# 0128: The second proof of stage 2: its test plan, written before the run

Status: settled by the architect on Nov 16, 2026, inside the plan for step 3, stage 2 (decision 0113, "What it must pass" and step 7 of its seven). It says what is run, what each side states, what passes and what is only reported, before any of it is run. Nothing here has been run. It is run when steps 5 and 6 are built.

Summary: The test plan of stage 2's second proof: the Eastern Solomons on its one day and then days 15 to 21 of the 39-day slice, each with both sides' intents stated and no unit order from anyone, over a thousand seeds; what each side states, the conditions that pass or fail with their numbers, the comparison against a side given no orders with its margin, and what is reported and not judged

Standing: Settled by the architect, Nov 16, 2026, inside decision 0113. Not run

## Why it is written first

A proof whose pass mark is chosen after the run proves nothing. Decision 0113 asks for the margin "written in the test plan before the run". This is that plan. Nothing in it may be changed after a run to make the run pass; a condition that turns out to be badly put is reported as failed and restated for a second run, with both results kept, as the legibility test of stage 1 was.

## What the staff cannot do, and what the plan does about it

1. **It does not choose where a carrier force stands** (decision 0125). So each side's carrier forces are given the task "operate a carrier force from a hex" by the scenario, with the hex the scenario's own route for that force ends at. That is an intent a commander states, and no order: the staff routes the force and does the rest.
2. **It does not move a surface force's home** (decision 0124). A surface force stands where it can reach in a night from where it lies.
3. **A carrier does not raid a base** (decision 0125). The Ryujo detachment's raid on Henderson Field in the record has no end that asks for it. The detachment is played as a carrier force under "deny an area".
4. **Air groups that the scenarios split by wave** (a second wave, an escort group) are data of the script. They are played as the groups they are.

Each of these is a limit of stage 2 and is named in the report beside the result.

## First: the Eastern Solomons, 24 August 1942

Scenario 5 with every order taken out, over seeds 1 to 1,000, live figures.

**What each side states on day 1,** with both leaves and no risk set:

| Side | Intent | Addressed to | Forces limit |
| --- | --- | --- | --- |
| Allied | Hold Henderson Field | The base | none: its groups |
| Allied | Deny the waters within 10 hexes of Henderson Field | The area | Task Force 16, Task Force 11 |
| Allied | Operate each of those two forces from the hex its scripted route ends at | The force | |
| Japanese | Deny the waters within 10 hexes of Henderson Field | The area | The carrier striking force, the Ryujo detachment |
| Japanese | Operate each of those two forces from the hex its scripted route ends at | The force | |

**It passes if all of these hold:**

- every invariant holds and no staff order or command is refused, on every seed;
- on every seed each side flies a search and a patrol;
- on at least 800 of the 1,000 seeds each side launches at least one strike at ships. A carrier battle in which one side never finds the other is possible, and the record's own battle was close to it, so this is not asked of every seed;
- the same start and seed give the same game.

**Against no orders, judged.** Two more runs of 1,000 seeds: the Allied side from its intents against the Japanese side as scripted, and the same with the Allied side given nothing; then the same pair for the Japanese side. For each side it passes if the median of that side's margin on the scenario's objectives is higher from its intents than from nothing. Reported beside it: aircraft lost, ships sunk and ships out of action on each side.

**Against the script, reported and not judged:** the same measures with the scenario's own orders on both sides; and how many seeds the staff's first strike was launched in the same pulse as the script's.

## Second: a week of the slice, days 15 to 21

Scenario 7, the 39 days, run from day 1 as scripted through day 14. From day 15 every order dated day 15 or later is taken out and each side's intents are stated on day 15, and the game runs to the end of day 21. Seeds 1 to 1,000, live figures. Orders given before day 15 stand until the staff changes them, as a commander's would.

**What each side states on day 15,** with both leaves and no risk set. The forces are named from the scenario's file as it stands on day 15, and the list is written into this record, with the base and force numbers, before the run:

| Side | Intent | Forces limit |
| --- | --- | --- |
| Allied | Hold Henderson Field | Its groups; the surface combat forces within a night's steaming |
| Allied | Keep Henderson Field supplied, at the days of supply it holds on day 15 or 15, whichever is more | The cargo ships and transports not at sea under an order of before day 15, their source base, and every force on the escort mission |
| Allied | Deny the waters within 10 hexes of Henderson Field | Its carrier forces |
| Allied | Operate each carrier force from the hex it is in on day 15 | |
| Japanese | Neutralise Henderson Field | none: its bases |
| Japanese | Hold Rabaul | Its groups |
| Japanese | Keep its base on Guadalcanal supplied, if it has one on day 15, on the same rule | Its cargo ships and transports, their source, its escort forces |
| Japanese | Deny the waters within 10 hexes of Henderson Field | Its surface combat forces within a night's steaming, and its carrier forces |

**It passes if all of these hold:**

- every invariant holds and no staff order or command is refused, on every seed;
- on every seed each side flies a search and a patrol on at least 5 of the 7 days;
- on at least 800 of the 1,000 seeds each side launches at least one strike or raid in the week;
- on at least 800 of the 1,000 seeds each side that has a keep-supplied intent lands supply at its base in the week;
- the same start and seed give the same game.

**Against no orders, judged.** For each side in turn, that side from its intents against the other as scripted, beside that side given nothing from day 15. It passes for a side if, of these four measures over the week, at least three are no worse and at least one is better, on the medians: the days Henderson Field's airfield is open to its holder, the supply landed at the side's base, the side's aircraft lost, and the side's ships sunk or out of action. The objectives scale is reported beside them: the scenario's objectives are set for 39 days and are not a week's measure.

**Against the script, reported and not judged:** the same measures with the scenario's own orders on both sides for the week.

The history measures of step 2 are not judged on any of these games (decision 0113).

## Legibility

By ruling 39 of decision 0107, on a new set: 24 reasons taken by rule and not by eye, the first of each kind on the lowest days of the first seed of the week's run: 6 from the cover doctrine, 6 from the raid doctrine, 4 from the strike doctrine, 4 from the stand doctrine and 4 from the carrier doctrine. The same cheap model answers the same two questions on each. **It passes if at least 20 of the 24 pass both questions and no doctrine's reasons all fail.**

## A model's game, and speed

- **A model plays the Allied side of the week by intents alone** against the Japanese side played by the staff, once, and its account is kept, as in stage 1's step 4. It is not judged.
- **Speed:** `sim-cli time` on the week with both staffs at work, on the machine and with the workloads decision 0100 names. Reported, with the share of a day the staffs take.

## What is done with a failure

A condition that fails is reported with the seeds and the reasons the staff gave. Nothing is tuned to pass it. What it shows is put to the designer with the choice it raises: a doctrine to change, a limit of the stage to lift, or a miss to name.
