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

Written on Nov 16, 2026, by the team lead of step 7, before any judged run, and to be checked by the architect before one. What each side states, with the numbers of the files as they stand, and every place where the wording above did not settle a choice, with the reading taken. Nothing above is changed by it. The files are in `data/scenarios/second-proof/`, a folder that no loop over `data/scenarios/*.toml` walks (decision 0072); their data paths are one folder further up. Every intent is a root stated by the commander, both leaves are given the same day, and no risk is set, so each is at balanced risk.

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
2. **The deny intent forms carrier tasks of its own as well** (decision 0125, 2): for each carrier force its forces limit names, an operate task at the hex the force is in that morning, beside the commander's. So each of the four forces has two operate tasks. For the Allied forces both name (7,-5). For the Japanese they differ: the commander's (8,-11) and (7,-9), the staff's (9,-19) and (7,-18), the forces' starting hexes. In the loading check every order of both was taken: task force 3 is routed to (8,-11) by intent 2 and set to battle readiness "at its hex" by intent 4 (formed under intent 1), and each carrier's groups are given each order twice. The plan is stated as written and this is not worked round; it is put to the architect.

### The week, days 15 to 21

The six files are `07a` to `07f_week.toml`, in the same six arrangements. Each is scenario 7 with every order dated day 15 or later of each side played from intents or given nothing taken out, the game ending on day 21, and the arrivals and orders dated after day 21 taken out, since the loader refuses them outside the game's days and they would never be reached. Days 1 to 14 are played as scripted in every file: on seed 1 every file has scenario 7's hashes for days 1 to 14, run to day 15 for the loading check. `07f_week.toml` has scenario 7's events for days 1 to 21 line for line on seed 1; its day hashes differ from scenario 7's from day 1, because the hash covers the list of arrivals and four of them are taken out.

The forces were read from the scripted game run through day 14 on seed 1 (the one seed the plan allows), and the days of supply from the staff's own reading on the morning of day 15 in the loading check. On that seed on the morning of day 15: Lunga (base 1, Henderson Field) is Allied with its airfield; Espiritu Santo (base 3) Allied; the Japanese hold Rabaul (2), Shortland (7), and on Guadalcanal the stocks at Taivu (5) and west of the Lunga (6); task forces 1 and 2 lie in (7,-5) and 3 and 4 in (9,-19) and (7,-18), where they started; the night forces lie in the Coral Sea.

| Side | Intent | Command | Forces limit |
| --- | --- | --- | --- |
| Allied | 1 | hold base 1 | none: the base's groups, 10, 11, 13, 14, 15, 16 and 17 |
| Allied | 2 | keep base 1 supplied at 24 days, no interval set | ships 37, 38, 52 to 84, 85 to 88 and 99 (40 ships); task forces 8 and 9; source base 3 |
| Allied | 3 | deny the waters within 10 hexes of (2,-4) | task forces 1 and 2 |
| Allied | 4 | operate the carrier force task force 1 from (7,-5) | none |
| Allied | 5 | operate the carrier force task force 2 from (7,-5) | none |
| Japanese | 1 | neutralise base 1 | none: its bases |
| Japanese | 2 | hold base 2 (Rabaul) | none: the base's groups |
| Japanese | 3 | keep base 5 (Taivu) supplied at 15 days, no interval set | ships 100 to 105, 108 to 110 and 113 to 133 (30 ships); source base 7 |
| Japanese | 4 | keep base 6 (west of the Lunga) supplied at 15 days, no interval set | the same as intent 3 |
| Japanese | 5 | deny the waters within 10 hexes of (2,-4) | task forces 3 and 4 |

Readings:

3. **"Within a night's steaming"** is read by the stand doctrine's own test of a night (decision 0124, 12): the force can reach the place within the night's 12 hours at full speed. No surface combat force of either side can: the six night forces lie 14 to 18 hexes (560 to 720 nm) from Henderson Field, against 384 nm in 12 hours at their 32 knots. So the Allied hold names no force and keeps no forces limit, which is "its groups" (the Eastern Solomons table's "none: its groups"), and the Japanese deny names its carrier forces alone.
4. **"The days of supply it holds on day 15 or 15, whichever is more."** The staff's reading on the morning of day 15 is 24.056603 days at Henderson Field (1472.625 t over 61.215 t a day), 6 days at Taivu (27.48 t over 4.58 t) and 0 west of the Lunga (0 t over 8.905 t). A floor is a whole number of days; 24.056603 is cut to 24, the most whole days the base holds. So 24, 15 and 15. The plan names no interval, so none is set and the staff takes 15 days.
5. **"Cargo ships and transports"** is read as the ships whose class carries cargo (a cargo capacity above 0) in forces on the transport mission. Read by the class's kind, transport, the Japanese side would have none: every Japanese run is of fast transports whose kind is destroyer, as are the Allied fast transports of task force 22. Left out by the reading: ships 91 to 98 (task forces 24 and 25, which carry no cargo); ship 39, the destroyer of task force 5, and the Fubuki-class destroyers 106, 107, 111 and 112, which carry none (each sails with its force all the same, since a run brings its whole force).
6. **"Not at sea under an order of before day 15"** leaves out task force 23, cargo ships 89 and 90, which is under way to Lunga under its order of day 13. Every other Allied ship read in 5 lies at Espiritu Santo. Ships with men aboard are not left out, since the plan does not: ship 99 (task force 26) has the 6th Naval Construction Battalion aboard, and the Japanese ships of task forces 32 to 39 have the Kawaguchi and other units aboard, as the scenario starts them. A run takes its whole force and its unload lands what is aboard, so a run of these ships may land those men earlier than the script does. Put to the architect.
7. **"Their source base."** The Allied ships all lie at Espiritu Santo, base 3. The Japanese ships lie at two bases: 25 of the 30 at Shortland, base 7, from which the scenario's runs sail, and task force 31's five at Rabaul. A forces limit has one source, and a run's ships must lie at it, so the source is read as where the ships lie, Shortland, and task force 31 stays named, though it can be taken only once it lies at Shortland. **In the loading check** the staff reads, on the morning of day 15, that neither Japanese intent can be met: "the source holds 0 t, keeps 0 t in reserve", and it hands in the notice that it cannot meet them. Shortland holds no supply, and the supply already aboard the ships at Shortland (about 7 to 17 t a ship) is not counted by that test. The other reading, Rabaul (9,818 t), would give the intents task force 31 alone. The reading is not changed after what the loading check showed; it is put to the architect.
8. **"Its base on Guadalcanal, if it has one on day 15."** The Japanese have two, Taivu and west of the Lunga, each off the enemy's map. Each is its base on Guadalcanal, so each is given a keep-supplied intent, in base number order and with no rank set, sharing one pool.
9. **"Every force on the escort mission."** Allied: task forces 8 (the bystander) and 9 (the dry destroyer), neither of which lies at the source, so neither is a free escort unless it comes there. Japanese: none is on the escort mission.
10. **"Operate each carrier force from the hex it is in on day 15":** (7,-5) for both. As in the Eastern Solomons, the deny intent forms its own operate task for each force at the same hex, so each Allied carrier force has two, with the same hex. The Japanese table has no operate row: the Japanese deny forms one for task forces 3 and 4 at (9,-19) and (7,-18).
11. **The state read on one seed.** The intents are stated once in each file for every seed. On a seed on which a base named here was no longer the side's on day 15, the command naming it would be refused there. Days 1 to 14 were not run on any other seed in this preparation.

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
