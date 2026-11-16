# 0128: The second proof of stage 2: its test plan, written before the run

Status: settled by the architect on Nov 16, 2026, inside the plan for step 3, stage 2 (decision 0113, "What it must pass" and step 7 of its seven). It says what is run, what each side states, what passes and what is only reported, before any of it is run. Nothing here has been run. It is run when steps 5 and 6 are built. The designer read the pass marks on Nov 16, 2026, and confirmed that they stand as written.

Summary: The test plan of stage 2's second proof: the Eastern Solomons on its one day and then days 15 to 21 of the 39-day slice, each with both sides' intents stated and no unit order from anyone, over a thousand seeds; what each side states, the conditions that pass or fail with their numbers, the comparison against a side given no orders with its margin, and what is reported and not judged

Standing: Settled by the architect and confirmed by the designer, Nov 16, 2026, inside decision 0113. Not run

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

## The intents as stated

Written on Nov 16, 2026, by the team lead of step 7, before any judged run, and checked by the architect before one. **The architect's rulings of Nov 16, 2026, made on the loading check and before any judged run:** readings 1, 3, 4, 5, 8, 9 and 10 and the measures stand as read; reading 2 showed a defect in the forming rule, mended first in its own pull request (#375: a force has one stand task and one carrier task at most, whoever made it; doctrine `supply-14`); reading 6 is changed, so that a ship with a land unit aboard on the morning of day 15 is in no pool; reading 7 is changed, so that the Japanese source is Rabaul; and reading 11 is ruled on how a refused command of day 15 is counted. The section below is as it stands after those rulings, and the loading check was run again on it. What each side states, with the numbers of the files as they stand, and every place where the wording above did not settle a choice, with the reading taken. Nothing above is changed by it. The files are in `data/scenarios/second-proof/`, a folder that no loop over `data/scenarios/*.toml` walks (decision 0072); their data paths are one folder further up. Every intent is a root stated by the commander, both leaves are given the same day, and no risk is set, so each is at balanced risk.

### The Eastern Solomons, day 1

The six files are `05a` to `05e_eastern_solomons.toml` and the scenario itself, `data/scenarios/05_eastern_solomons.toml`, for (f). A side played from intents or given nothing has every one of its orders taken out; the other side keeps its own.

| File | Allied | Japanese |
| --- | --- | --- |
| (a) `05a` | intents | intents |
| (b) `05b` | intents | scripted |
| (c) `05c` | nothing | scripted |
| (d) `05d` | scripted | intents |
| (e) `05e` | scripted | nothing |
| (f) `05_eastern_solomons.toml` | scripted | scripted |

| Side | Intent | Command | Forces limit |
| --- | --- | --- | --- |
| Allied | 1 | hold base 1 (Henderson Field) | none: the base's groups, 9 (VMF-223) and 10 (VMSB-232) |
| Allied | 2 | deny the waters within 10 hexes of (2,-4), Henderson Field's hex | task forces 1 (Task Force 16) and 2 (Task Force 11) |
| Allied | 3 | operate the carrier force task force 1 from (7,-5) | none |
| Allied | 4 | operate the carrier force task force 2 from (7,-5) | none |
| Japanese | 1 | deny the waters within 10 hexes of (2,-4) | task forces 3 (the carrier striking force) and 4 (the Ryujo detachment) |
| Japanese | 2 | operate the carrier force task force 3 from (8,-11) | none |
| Japanese | 3 | operate the carrier force task force 4 from (7,-9) | none |

Readings:

1. **"The hex its scripted route ends at."** Task forces 3 and 4 each have one route order, to (8,-11) and to (7,-9). Task forces 1 and 2 have none and stay all day in (7,-5), where they start: that is the hex their scripted day ends at.
2. **The deny intent and the commander's own tasks.** The deny intent forms an operate task for each carrier force its forces limit names (decision 0125, 2), but not for a force that already has one in force, whoever made it (the architect's ruling of Nov 16, 2026, built in #375 before any judged run). So each of the four forces has one task, the commander's, at the hex above, and the deny's reason names it. *As first stated, before the ruling,* each force had two, and the Japanese pairs disagreed ((8,-11) and (7,-9) against the starting hexes (9,-19) and (7,-18)), every carrier's groups being ordered twice. In the loading check after the ruling, each carrier force has exactly one operate task and no staff order is given twice to a unit on a day.

### The week, days 15 to 21

The six files are `07a` to `07f_week.toml`, in the same six arrangements. Each is scenario 7 with every order dated day 15 or later of each side played from intents or given nothing taken out, the game ending on day 21, and the arrivals and orders dated after day 21 taken out, since the loader refuses them outside the game's days and they would never be reached. Days 1 to 14 are played as scripted in every file: on seed 1 every file has scenario 7's hashes for days 1 to 14, run to day 15 for the loading check. `07f_week.toml` has scenario 7's events for days 1 to 21 line for line on seed 1; its day hashes differ from scenario 7's from day 1, because the hash covers the list of arrivals and four of them are taken out.

The forces were read from the scripted game run through day 14 on seed 1 (the one seed the plan allows), and the days of supply from the staff's own reading on the morning of day 15 in the loading check. On that seed on the morning of day 15: Lunga (base 1, Henderson Field) is Allied with its airfield; Espiritu Santo (base 3) Allied; the Japanese hold Rabaul (2), Shortland (7), and on Guadalcanal the stocks at Taivu (5) and west of the Lunga (6); task forces 1 and 2 lie in (7,-5) and 3 and 4 in (9,-19) and (7,-18), where they started; the night forces lie in the Coral Sea.

| Side | Intent | Command | Forces limit |
| --- | --- | --- | --- |
| Allied | 1 | hold base 1 | none: the base's groups, 10, 11, 13, 14, 15, 16 and 17 |
| Allied | 2 | keep base 1 supplied at 24 days, no interval set | ships 37, 38, 52 to 84 and 85 to 88 (39 ships); task forces 8 and 9; source base 3 |
| Allied | 3 | deny the waters within 10 hexes of (2,-4) | task forces 1 and 2 |
| Allied | 4 | operate the carrier force task force 1 from (7,-5) | none |
| Allied | 5 | operate the carrier force task force 2 from (7,-5) | none |
| Japanese | 1 | neutralise base 1 | none: its bases |
| Japanese | 2 | hold base 2 (Rabaul) | none: the base's groups |
| Japanese | 3 | keep base 5 (Taivu) supplied at 15 days, no interval set | ships 100 to 105 (6 ships); source base 2 (Rabaul) |
| Japanese | 4 | keep base 6 (west of the Lunga) supplied at 15 days, no interval set | the same as intent 3 |
| Japanese | 5 | deny the waters within 10 hexes of (2,-4) | task forces 3 and 4 |

Readings:

3. **"Within a night's steaming"** is read by the stand doctrine's own test of a night (decision 0124, 12): the force can reach the place within the night's 12 hours at full speed. No surface combat force of either side can: the six night forces lie 14 to 18 hexes (560 to 720 nm) from Henderson Field, against 384 nm in 12 hours at their 32 knots. So the Allied hold names no force and keeps no forces limit, which is "its groups" (the Eastern Solomons table's "none: its groups"), and the Japanese deny names its carrier forces alone.
4. **"The days of supply it holds on day 15 or 15, whichever is more."** The staff's reading on the morning of day 15 is 24.056603 days at Henderson Field (1472.625 t over 61.215 t a day), 6 days at Taivu (27.48 t over 4.58 t) and 0 west of the Lunga (0 t over 8.905 t). A floor is a whole number of days; 24.056603 is cut to 24, the most whole days the base holds. So 24, 15 and 15. The plan names no interval, so none is set and the staff takes 15 days.
5. **"Cargo ships and transports"** is read as the ships whose class carries cargo (a cargo capacity above 0) in forces on the transport mission. Read by the class's kind, transport, the Japanese side would have none: every Japanese run is of fast transports whose kind is destroyer, as are the Allied fast transports of task force 22. Left out by the reading: ships 91 to 98 (task forces 24 and 25, which carry no cargo); ship 39, the destroyer of task force 5, and the Fubuki-class destroyers 106, 107, 111 and 112, which carry none (each sails with its force all the same, since a run brings its whole force).
6. **"Not at sea under an order of before day 15"** leaves out task force 23, cargo ships 89 and 90, which is under way to Lunga under its order of day 13. **A ship with a land unit aboard on the morning of day 15 is left out of every pool too** (the architect's ruling of Nov 16, 2026, which replaces the first reading, which kept them in): the staff of stage 2 does not land troops (decision 0113), and a supply run must not land them by accident. That leaves out ship 99 (task force 26, the 6th Naval Construction Battalion aboard) and every ship of the Japanese task forces 32 to 39 (the Kawaguchi and other units aboard, as the scenario starts them). Their men stay aboard for the week: a limit of the stage, named in the report.
7. **"Their source base."** The Allied ships all lie at Espiritu Santo, base 3. **The Japanese source is Rabaul, base 2, the one Japanese base that holds supply to load: ruled by the architect on Nov 16, 2026, after the loading check showed that Shortland holds no supply, and before any judged run.** The first reading took Shortland, where most of the ships lie; on the morning of day 15 the staff then read that neither Japanese intent could be met ("the source holds 0 t"). The forces limit names every Japanese ship that carries cargo and is neither at sea under an earlier order nor carrying a land unit, wherever it lies: ships 100 (task force 30, at Shortland) and 101 to 105 (task force 31, at Rabaul). What the staff does with a ship that does not lie at the source is its own doing and is reported. In the loading check after the ruling, the staff orders a run for Taivu on day 15 from task force 31; west of the Lunga it hands in that it cannot meet, every pool ship being committed until day 21.
8. **"Its base on Guadalcanal, if it has one on day 15."** The Japanese have two, Taivu and west of the Lunga, each off the enemy's map. Each is its base on Guadalcanal, so each is given a keep-supplied intent, in base number order and with no rank set, sharing one pool.
9. **"Every force on the escort mission."** Allied: task forces 8 (the bystander) and 9 (the dry destroyer), neither of which lies at the source, so neither is a free escort unless it comes there. Japanese: none is on the escort mission.
10. **"Operate each carrier force from the hex it is in on day 15":** (7,-5) for both. As in the Eastern Solomons, the deny intent forms its own operate task for each force at the same hex, so each Allied carrier force has two, with the same hex. The Japanese table has no operate row: the Japanese deny forms one for task forces 3 and 4 at (9,-19) and (7,-18).
11. **The state read on one seed.** The intents are stated once in each file for every seed. On a seed on which a base named here was no longer the side's on day 15, the command naming it would be refused there. Days 1 to 14 were not run on any other seed in this preparation. **Ruled by the architect on Nov 16, 2026:** such a refusal of a commander's command of day 15 is counted apart from the staff's refusals and reported with its seeds; the seed stays in the run, and it does not fail the condition that no staff order or command is refused.

### How the conditions are read

These are the measures as the scripts read them, written before the run. Each is read from the umpire's export of each seed (its events and its states after each pulse) and from the batch file; they were tested on arrangement (f) alone, and agree with the batch file there in every strike, raid, aircraft lost and ship sunk on 50 seeds of each part.

- **Invariants:** every seed's game runs to its end; a broken invariant stops it with an error.
- **Refused:** an `order_rejected` event for an order the staff gave, and any `command_refused` event. A scripted order rejected (the day-19 order of the wrong side's force, kept in scenario 7 for coverage, decision 0043) is counted apart and not judged.
- **A search** is a `search_flown` event of a group of the side. **A patrol** is flown in a daylight pulse by a group of the side on combat air patrol whose patrol share of its ready aircraft is at least one aircraft, with a pilot for it where pilots are counted and not above its rest threshold, at a base of its side or aboard a ship afloat with a flight deck: read from the states, because the Eastern Solomons loads no fatigue figures and a patrol there leaves no event. In the week, where it does, the patrol sorties' fatigue events are used, and on (f) they give the same days as the states on all 50 seeds and both sides. A day of the week counts when the side flies both a search and a patrol on it.
- **A strike at ships** is a `strike_launched`, **a raid** a `base_strike_launched`, by a group of the side (a group formed by aircraft landing elsewhere is its first group's side). The first strike's pulse is compared with (f)'s on the same seed.
- **Supply landed at the side's base** is the supply in `cargo_unloaded` on days 15 to 21 at base 1 for the Allied side and at bases 5 and 6 for the Japanese.
- **Henderson Field's airfield open to its holder** is its end-of-day record marked open, on days 15 to 21, counted for the side holding it that day. For a side, the measure is the days open while it held the base less the days open while the enemy held it, so more is better for each side.
- **Aircraft lost** are every loss the batch counts (shot down, destroyed on the ground, ready or damaged, lost with a ship, written off by wear or a rebase, ditched for fuel or for want of a deck), in the part's days: day 1 of the Eastern Solomons, days 15 to 21 of the week.
- **Ships sunk** are `ship_sunk` events in those days. **Ships out of action** are decision 0088's: ships not sunk that have days of repair at the end (any flotation damage, or a system degraded or out) or are at the yard, counted for ships hit in those days, so that damage of before day 15 is not counted in the week.
- **The objectives margin** is the batch's margin for each seed, the Allied one, and its negative for the Japanese.
- **Medians** are over the 1,000 seeds; a median "no worse" is equal or better, "better" strictly better.
- **The same start and seed** is checked by running each batch twice and comparing the files, and the exports of a sample of seeds twice.

## Restated before the legibility test was run

*Dated Nov 16, 2026: the architect's ruling, made before any reason was put to a reader.* The set as first written cannot be filled from the week: no surface force is within a night's steaming (reading 3 above), so no stand task is formed, and both deny intents name only carrier forces, so no base is given a strike task. Neither doctrine gives a reason in the week on any seed. Renaming the carrier task's reasons would not test the strike doctrine, so the set keeps its 24 reasons, its five doctrines and its pass mark, and two doctrines' reasons come from their own scenarios: cover 6, raid 6 and carrier 4 from the week's arrangement (a), seed 1, both sides; strike 4 from scenario 53 and stand 4 from scenario 54, each its file as it stands (the main case, its own seed, the figures it names). **The strike and stand reasons are from placeholder scenarios, because the week gives none.**

The rule, written before the pick (`crates/sim-cli/tests/recorded_games/legibility_second_proof/pick.py`): a reason's doctrine is the kind of the intent its serves line names. Within a doctrine the reasons are taken by day and then event number, alternately an order's reason and a day-with-no-order reason (an order first), the other sort where the one due has none left, skipping a reason whose worked text one already taken has. A reason of a staff command (a plan made ready or released under a leave) is neither and is not taken, as in stage 1's pick; this was added to the rule before any reader ran, when the first pick took two such reasons as carrier reasons. Each reader is given the reason's five parts and nothing else, with stage 1's two questions: what was the staff trying to do, and what would change its decision. The key (`key.md` in the same folder) was written from the reasons alone before any reader ran: question 1 passes if the answer names the place or force and the aim, question 2 if it gives a condition the reason's own "would change" or "binding" part states. The pass mark is unchanged: at least 20 of the 24 pass both questions, and no doctrine's reasons all fail.

| # | Doctrine | From | Side | Sort | Day | Event |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | cover | the week, arrangement (a), seed 1 | allied | order | 15 | 1507 |
| 2 | cover | the week, arrangement (a), seed 1 | japanese | no order | 15 | 1490 |
| 3 | cover | the week, arrangement (a), seed 1 | allied | order | 15 | 1509 |
| 4 | cover | the week, arrangement (a), seed 1 | allied | no order | 16 | 1851 |
| 5 | cover | the week, arrangement (a), seed 1 | allied | order | 15 | 1511 |
| 6 | cover | the week, arrangement (a), seed 1 | japanese | no order | 16 | 1861 |
| 7 | raid | the week, arrangement (a), seed 1 | japanese | order | 15 | 1585 |
| 8 | raid | the week, arrangement (a), seed 1 | japanese | no order | 15 | 1491 |
| 9 | raid | the week, arrangement (a), seed 1 | japanese | order | 18 | 2545 |
| 10 | raid | the week, arrangement (a), seed 1 | japanese | no order | 16 | 1862 |
| 11 | raid | the week, arrangement (a), seed 1 | japanese | order | 20 | 2932 |
| 12 | raid | the week, arrangement (a), seed 1 | japanese | no order | 17 | 2205 |
| 13 | carrier | the week, arrangement (a), seed 1 | allied | order | 15 | 1497 |
| 14 | carrier | the week, arrangement (a), seed 1 | allied | no order | 15 | 1441 |
| 15 | carrier | the week, arrangement (a), seed 1 | allied | order | 15 | 1499 |
| 16 | carrier | the week, arrangement (a), seed 1 | allied | no order | 15 | 1442 |
| 17 | strike | scenario 53 | allied | order | 1 | 27 |
| 18 | strike | scenario 53 | allied | no order | 1 | 24 |
| 19 | strike | scenario 53 | allied | order | 1 | 29 |
| 20 | strike | scenario 53 | allied | no order | 2 | 77 |
| 21 | stand | scenario 54 | allied | order | 1 | 17 |
| 22 | stand | scenario 54 | allied | no order | 3 | 63 |
| 23 | stand | scenario 54 | allied | order | 1 | 19 |
| 24 | stand | scenario 54 | allied | order | 1 | 21 |

**The model's game, restated with it.** A player seat is given no scripted orders, so the Allied seat's days 1 to 14 are pre-played with the script's own Allied orders through the tools (`issue_orders` and `end_day`, by the side's own numbering), on seed 1 of `data/scenarios/second-proof/07m_model_week.toml` (the week with the Japanese intents of day 15 and no Allied command), and the model is handed the game at the start of day 15 only after the day hashes 1 to 14 were checked equal to the scripted run's. The model is told the tools' own descriptions and one instruction, to command by intents alone and give no order to a unit.

## As run

Run on Nov 16, 2026, release build, live figures, seeds 1 to 1,000, doctrine `supply-14`, after the rulings above and #375. The report with every number is `docs/validation/stage-2-second-proof.md`. Nothing was tuned, and no pass mark, intent or measure was changed after a run.

- **The Eastern Solomons does not pass.**
  - These hold on every seed: every invariant; no staff order or command refused; a search and a patrol by each side; the same start giving the same game (batch files and exports identical when run twice).
  - At least one strike at ships on 800 seeds: the Allied side launched one on 0 seeds and the Japanese on 152.
  - Against no orders, the Allied side passes (median margin 1.05 from intents against 0.20 from nothing). The Japanese side does not (0.00 against 0.00).
  - Every staff first strike that the script also flew went in the script's pulse (129 of 129 Japanese seeds in (a), 447 of 447 Allied in (b)).
  - The reasons: the carrier doctrine routes a force to its hex at cruise, so the Japanese carriers come in later than the script's; the American carriers do not close the range on a contact beyond their reach.
- **The week does not pass, on one condition.**
  - These hold: every invariant; no refusal (no commander's command of day 15 refused on any seed either); a strike or raid on 997 Allied and 1,000 Japanese seeds; supply landed on 1,000 seeds for each side; the same start giving the same game.
  - A search and a patrol on 5 of the 7 days on every seed: the Allied side does so on all 1,000, the Japanese on 683.
  - On the other 317 seeds the Japanese floatplane groups, the side's only searchers, rest above the fatigue level of the balanced risk on two or more days.
  - Against no orders, both sides pass, each with three measures no worse and one better:
    - Allied: supply at Henderson Field 3,220.8 t against 2,000 t; aircraft lost worse, 24 against 1.
    - Japanese: supply landed 119.08 t against 0; aircraft lost worse, 29 against 0.
- **Legibility passes**, 24 of 24, every doctrine 4 or 6 of its own. The strike and stand reasons are from scenarios 53 and 54, as ruled.
- **The model's game** was played: Allied by intents alone from day 15, after a pre-play of days 1 to 14 checked hash for hash. It ended in a decisive victory, margin 56.58. It is kept, not judged.
- **Speed:**
  - A day of the staffed week is 9 to 13 ms.
  - The staffs' decisions take up to 4.08 ms (Allied, day 15) and 7.42 ms (Japanese, day 17).

Put to the designer with the choices it raises: how a carrier force from an end is to reach its strike (its speed to its hex, closing the range); and whether a force's searching should not rest all its searchers, or should draw on its bombers when they do.
