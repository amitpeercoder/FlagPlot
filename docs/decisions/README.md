# Decisions

Design choices the spec left open, one short file each. A decision here can be overturned. When that happens, change the file and say what replaced it, so the history stays readable.

Files are numbered in the order the decisions were made. Add the next number for a new one.

Every record begins with its title (`# NNNN: Title`), its `Status:` line, a `Summary:` line and a `Standing:` line, each a paragraph of its own. The table below gives each record's number, its title and its `Standing:` line; it is made from those lines by `sh scripts/decisions-index.sh` and is never edited by hand: to change a row, change the record's title or `Standing:` line and run the command. The `Summary:` line is still required and checked, but the table does not show it. A conflict in the table is resolved by running the script, never by hand. CI fails when the committed table is not what the records give (decision 0108, item 9).

From record 0233 on a record holds the why, what was weighed, the rule card and the rulings; a rule is stated in the record only while it is a plan and in `docs/rules/` once built; and what was built, with the rulings on each pull request, is in `as-built/`, in a file of the record's name (decision 0233). Earlier records keep their form, and their new sections go in `as-built/` too.

A record that adds a rule to the simulation ends with a rule card: the state the rule reads and the state it writes. Records 0001 to 0010 predate this; 0001, 0005, 0006 and 0008 gained cards at the complexity review after scenario 5.

<!-- The table is generated from the records by scripts/decisions-index.sh: do not edit it by hand. -->

| # | Decision | Status |
| --- | --- | --- |
| 0001 | A game day runs 18:00 to 18:00 in the scenario's reference zone | Confirmed |
| 0002 | Fixed-point numbers | Settled |
| 0003 | Random numbers | Settled |
| 0004 | State hash | Settled |
| 0005 | Movement | Confirmed |
| 0006 | Fuel model | Confirmed |
| 0007 | Truth and belief in the type system | Settled |
| 0008 | Orders | Confirmed |
| 0009 | Scenario files and saved games | Settled |
| 0010 | Where scenario tests live | Settled |
| 0011 | Cargo handling at ports | Confirmed |
| 0012 | Air search, reports and contacts | Confirmed |
| 0013 | Random number keys for a pair of entities | Confirmed |
| 0014 | Surface combat | Confirmed |
| 0015 | Order-of-battle facts: format and rules | Confirmed; standing rules need review |
| 0016 | Damage to named systems, and stragglers | Confirmed |
| 0017 | The real map, and pathfinding | Confirmed |
| 0018 | A read-only replay viewer | Confirmed |
| 0019 | Savo Island, the designer's choices | Confirmed |
| 0020 | Game figures for real ships, derived from facts | Confirmed; constants are placeholders |
| 0021 | Readiness, alertness and alarm | Confirmed |
| 0022 | Meeting in time order, and breaking off | Confirmed |
| 0023 | The Savo Island ships: exceptions and assumptions | Confirmed |
| 0024 | The Savo Island scenario: what is in it and what is left out | Confirmed |
| 0024, notes | The Savo Island scenario's later changes | Ruled by the coordinating architect |
| 0025 | Air groups on carriers | Confirmed |
| 0026 | Air strikes on ships, and the flight deck | Confirmed |
| 0027 | Fighters, anti-aircraft fire and aircraft losses | Confirmed |
| 0028 | A strike that finds nothing drops the contact | Confirmed |
| 0029 | Figures for carriers, anti-aircraft guns and aircraft, derived from facts | Confirmed |
| 0030 | Escorts have a radius | Confirmed |
| 0031 | The Eastern Solomons scenario: what is in it and what is left out | Confirmed |
| 0032 | What the complexity review cut and merged | Confirmed |
| 0033 | Three changes to how hits are worked | Confirmed |
| 0034 | Raids on a base | Confirmed |
| 0035 | Wear, repair and operational losses | Confirmed; 2 needs review, after the air figures are tuned |
| 0036 | Fatigue and rest | Confirmed |
| 0037 | Fatigue at battle readiness | Confirmed |
| 0038 | Pilots and replacements | Confirmed |
| 0039 | Base supply consumption | Confirmed |
| 0040 | Air distance is a straight line | Confirmed |
| 0041 | Aircraft that arrive on a day | Confirmed |
| 0042 | The Rabaul against Guadalcanal scenario | Confirmed |
| 0043 | The determinism scenario, saved games and replay | Confirmed |
| 0044 | The soak scenario | Confirmed |
| 0045 | Refuelling in port | Confirmed |
| 0046 | Fuel as a cargo | Confirmed |
| 0047 | What a side may know of what happened | Confirmed |
| 0048 | The briefing and the queries | Confirmed |
| 0049 | Measuring | Confirmed |
| 0050 | Playing a side | Confirmed |
| 0051 | The MCP server and scenario 9 | Confirmed |
| 0052 | What step 1 leaves out | Confirmed |
| 0053 | The viewer measures, and shows a side's view | Settled |
| 0054 | Objectives and results | Confirmed |
| 0055 | After the architecture review | Confirmed (1 to 4 and the pillar wording); settled (5 to 9) |
| 0056 | Claims from the air | Confirmed |
| 0057 | The state hash at version 2, and results that say what they were run on | Settled |
| 0058 | Ships are charged straight-line miles | Confirmed |
| 0059 | Aircraft that cannot land where they took off | Confirmed |
| 0060 | A claim's breakdown is the umpire's | Confirmed |
| 0061 | A search that sighted is shown without its distance or its chance | Confirmed |
| 0062 | What `batch` records for each run | Settled |
| 0063 | Strike target priority as an order | Confirmed |
| 0064 | Experience | Confirmed |
| 0065 | A side's events name no enemy ship, task force or air group | Confirmed |
| 0066 | Land units, carried by sea and fed from a base | Confirmed by the designer, Nov 13, 2026 (7, 12 and 14 agreed); built Nov 13, 2026 (stage 1, scenario 21) |
| 0067 | Two objectives that could be gamed | Confirmed |
| 0068 | A force not at battle readiness fires with part of its guns | Confirmed |
| 0069 | Ship repair | Confirmed |
| 0070 | Rates of fire as achieved, not as designed | Settled by the architect, to be reviewed |
| 0071 | A force not ready fires part of its torpedoes | Decided by the architect on the designer's delegation |
| 0072 | The Cape Esperance scenario: what is in it and what is left out | Built and run once (Nov 13, 2026); settled by the architect, to be reviewed |
| 0073 | The Japanese search at the Eastern Solomons, from the record | Settled by the architect on the designer's delegation |
| 0074 | A class's torpedo tubes are the tubes that bear on one side | Decided by the architect on the designer's delegation |
| 0075 | An aircraft type has a search radius of its own | Settled by the architect on the designer's delegation |
| 0076 | How long surprise lasts: no rule | Decided by the architect on the designer's delegation: no rule |
| 0077 | Which radar sets count for sighting ships at night | Decided by the architect on the designer's delegation |
| 0078 | Which enemy ship a ship fires at | Decided by the architect on the designer's delegation |
| 0079 | Anti-aircraft fire and the attack | Decided by the architect on the designer's delegation |
| 0080 | Whom the patrol fires at | Decided by the architect on the designer's delegation |
| 0081 | The Eastern Solomons scenario's remaining orders, from the record | Decided by the architect on the designer's delegation |
| 0082 | Rabaul against Guadalcanal: the raid days and the dive bombers' searches from the record | Decided by the architect on the designer's delegation |
| 0083 | The other side's torpedo tubes | Decided by the architect on the designer's delegation |
| 0084 | What the agreements show, and what they do not | Decided by the architect on the designer's delegation |
| 0085 | What a side is told of a fight, and other leaks an outside review found | Decided by the architect on the designer's delegation |
| 0086 | The numbers a side is given | Decided by the architect on the designer's delegation |
| 0087 | The patrol's fire and the attack | Decided by the architect on the designer's delegation |
| 0088 | Ships out of action, counted by the time they are away | Confirmed by the designer, Nov 13, 2026; built Nov 13, 2026 |
| 0089 | After a second outside review of the tuning | Decided by the architect |
| 0090 | The fight ashore, and a base that changes side | Confirmed by the designer, Nov 13, 2026; rebasing (0091) came first. Built Nov 14, 2026 (stage 2, scenario 23) |
| 0091 | Rebasing an air group by order | Asked for by the designer, Nov 13, 2026; details by the architect. Built Nov 14, 2026 (scenario 22) |
| 0092 | Shells and bombs on a base and its men, and what a side knows of the enemy ashore | Confirmed by the designer, Nov 14, 2026, with shells weighed by the gun and a claim after a bombardment. Built Nov 14, 2026 (stage 3, scenario 24) |
| 0093 | The landings of August 1942 and the first attacks on the airfield, as a scenario | Confirmed by the designer, Nov 14, 2026, with the land units in the full slice and marching built first. Item 8 changed by 0101. Built Nov 14, 2026 (the full slice of 39 days and its measures); the soak (item 10) built Nov 15, 2026 (scenario 45, doctrine `supply-7`); the tuning round is not |
| 0094 | Marching overland | Asked for by the designer, Nov 14, 2026; details by the architect. Built Nov 14, 2026 (scenario 25) |
| 0095 | Preparing a landing | Confirmed by the designer, Nov 14, 2026; revised the same day with two owners. Built Nov 14, 2026 (scenario 27) |
| 0096 | A force that acts at a place and steams on | Called for by the step 3 spec, which the designer confirmed on Nov 14, 2026; details by the architect. Built (scenario 26) |
| 0097 | The quality of a land unit, and training | Confirmed by the designer, Nov 14, 2026. Built Nov 14, 2026 (scenario 28, with the activities ready and train of decision 0099) |
| 0098 | What a land unit is good at | Confirmed by the designer, Nov 14, 2026. Built Nov 14, 2026 (scenario 29) |
| 0099 | Units that work, and what they build | Confirmed by the designer, Nov 14, 2026. The one activity a day and the activities ready and train were built with 0097 (scenario 28). Built Nov 14, 2026 (scenario 30) |
| 0100 | Step 3, stage 0: the ground the staff stands on | Confirmed by the designer, Nov 14, 2026. Built Nov 14, 2026 (17 questions; saved game version 2 with the rules label 1; `sim-cli time`; the experiment on the unmerged branch `s0-experiment`) |
| 0101 | The data of the August landings | Settled by the architect, Nov 14, 2026. Changes three points of 0093. Amended from new rows, and by the architect's rulings during the build. Built Nov 14, 2026 (the full slice of 39 days; an empty air group waits at a base not its side's) |
| 0102 | What we take from the rules comparison | The order approved by the designer, Nov 14, 2026. Adds no rule |
| 0103 | An airfield damaged and mended | Confirmed by the designer, Nov 14, 2026, who chose that a badly damaged field takes no aircraft. Built Nov 14, 2026 (scenario 31) |
| 0104 | A beaten side falls back | Confirmed by the designer, Nov 14, 2026, who chose surrender. Built Nov 14, 2026 (scenario 32) |
| 0105 | Who is engaged in a fight ashore, and an attack that fails | Confirmed by the designer as revised, Nov 14, 2026. Built Nov 14, 2026 (scenario 33) |
| 0106 | Cover, men who dig in, and a place under bombardment | Confirmed by the designer as revised, Nov 14, 2026. Built Nov 14, 2026 (scenario 34, state version 22, rules label 5) |
| 0107 | Step 3, stage 1: the command contract, and a staff that keeps a base supplied | Confirmed by the designer, Nov 14, 2026, after an independent review. Step 1 built Nov 15, 2026: the engine half (command state, commands, the staff passed in, the saved game version 3, days of supply; scenario 35; state version 24) and the staff half (`sim-staff`, doctrine `supply-1`, the doctrine for keeping a base supplied, the tools of step 1, agent interface version 4; scenario 36). Step 2 built Nov 15, 2026: priority between intents, what is committed the same morning, commitments of stock as runs, the protected limit and displaced demand (doctrine `supply-2`; scenario 40; no state); scenario 40b's table corrected by the architect after the build. Step 3 built Nov 15, 2026: holds and release, the override, ending a run, runs set aside, cargo aboard carried, a cancelled plan's fallback, previews (doctrine `supply-3`; state version 25; rules label 10; agent interface version 6; scenario 41). Step 4 built Nov 15, 2026: interrupts, staff notices, `run`, the command report, resuming under the doctrine's version (doctrine `supply-4`; state version 26; rules label 10; agent interface version 7; scenarios 42 and 43). Stage 1 is built; the legibility test (ruling 39) did not meet its pass mark on its first run (20 of 24, but none of the four leave reasons), read and confirmed by the architect. The designer decided on Nov 15, 2026 to mend the reasons and run the four again: the reasons were mended (every reason states its intent in words; doctrine `supply-5`; saved game version 3 and agent interface version 7 unchanged) and the four leave reasons, run once more, passed 4 of 4, so with them in place of the first run's four the test comes to 24 of 24, resting on two runs; the architect's reading of the second run is to come |
| 0108 | A refactoring session before stage 1 | Settled by the architect, Nov 14, 2026, with the designer's approval to hold it. Built, Nov 15, 2026 |
| 0109 | The first tuning round of the ground figures: what is settled first, and the plan | Decision 1 confirmed by the designer, Nov 14, 2026; the rest settled by the architect. Decisions 1 to 4 built, with a new baseline, Nov 15, 2026. Round 1 run, Nov 15, 2026: four figures moved; steps 2 and 3 stopped, the two fights wanting different values; steps 4 and 5 moved nothing. The Tenaru's two measures accepted by the designer as a named miss, Nov 15, 2026, to be mended with step 3's stage 3 |
| 0110 | What a player commanding ashore must be told | Item 10 confirmed by the designer, Nov 14, 2026; the rest settled by the architect. Built, Nov 15, 2026 |
| 0111 | Three rules the staff needs: a load up to a weight, a reserve at a base, and forces formed and joined | Settled by the architect, Nov 14, 2026, inside decision 0107. Built Nov 14, 2026 (scenario 39, state version 23, rules label 6) |
| 0112 | Refuelling that takes time | Confirmed by the designer, Nov 15, 2026. Built Nov 15, 2026 (scenario 44, rules label 11, doctrine supply-6, state version unchanged) |
| 0113 | Step 3, stage 2: doctrine for the fighting tasks, and the enemy played by the staff | Confirmed by the designer, Nov 15, 2026, and confirmed again the same day after it was rewritten on decision 0116, with his answers to its seven questions as revised. Step 1 built Nov 15, 2026 (decision 0114; scenario 46; state version 27, rules label 12) |
| 0114 | Two settings of a strike group: how old a contact it launches on, and a share held back | Settled by the architect, Nov 15, 2026, inside decision 0113. Built Nov 15, 2026 |
| 0115 | Covering a base from the air: the first fighting doctrine | Settled by the architect, Nov 15, 2026, inside decision 0113. Built |
| 0116 | Intents are ends, and they form a tree | Agreed by the designer, Nov 15, 2026. A change to the step 3 spec's command contract. Not built |
| 0117 | The intent tree in the command state, and the end "hold a place" | Settled by the architect, Nov 15, 2026, inside decisions 0113 and 0116. Built |
| 0118 | A strike sized to its target | Settled by the architect, Nov 15, 2026, inside decision 0113. Built |
| 0119 | Command levels: the designer's answers, and a stage for them between stages 2 and 3 | Decided by the designer, Nov 15, 2026. A stage added to the step 3 spec. Not built |
| 0120 | Raids, and the end "neutralise a base" | Settled by the architect, Nov 15, 2026, inside decisions 0113 and 0119. Built |
| 0121 | An escort force: keeping station, and who is met first | Settled by the architect, Nov 15, 2026, inside decision 0113. Built |
| 0122 | Rearming: a ship in port takes ammunition through the hours of the day | Settled by the architect, Nov 15, 2026, inside decision 0113. Built |
| 0123 | The end "deny an area", and striking ships from a base | Settled by the architect, Nov 16, 2026, inside decisions 0113, 0118 and 0120. Built |
| 0124 | A surface force that stands by night: the task force as an addressee, the night stand, fuel as policy | Settled by the architect, Nov 16, 2026, inside decisions 0113 and 0120. Built |
| 0125 | A carrier force played from an end | Settled by the architect, Nov 16, 2026, inside decisions 0113, 0120, 0123 and 0124. Built |
| 0126 | Escorted and covered runs | Settled by the architect, Nov 16, 2026, inside decisions 0113, 0121 and 0124. Built |
| 0127 | Going back to rearm: ammunition as policy | Settled by the architect, Nov 16, 2026, inside decisions 0113, 0122 and 0124. Built |
| 0128 | The second proof of stage 2: its test plan, written before the run | Settled by the architect and confirmed by the designer, Nov 16, 2026, inside decision 0113. Run Nov 16, 2026: not passed as stated (three conditions fail; the choices they raise are the designer's). Run again Nov 16, 2026, at doctrine `supply-16`: not passed as stated (two conditions of the Eastern Solomons fail; the week and legibility pass). Run a third time Nov 16, 2026, at doctrine `supply-17`: not passed as stated (the same two conditions fail, the Allied strike on 767 seeds of 800; the week and legibility pass). Closed by the designer, Nov 16, 2026, with the two misses named |
| 0129 | A seat at every level: a level's staff may be a person, a model or the game's own staff | Decided by the designer, Nov 16, 2026. Adds to decision 0119. Not built |
| 0130 | A carrier force comes up at speed and closes the range | Settled by the architect, Nov 16, 2026, on the designer's choice after the second proof. Built |
| 0131 | A search that can be kept up: one sortie a day, its own fatigue figure, and bombers that take a turn | Decided by the designer, Nov 16, 2026, on the research. Built. Rule 6 amended the same day for a carrier force, on his choice after the proof's rerun |
| 0132 | Step 3, stage 2b: command levels, in five steps | Confirmed by the designer, Nov 16, 2026, with his four answers; on radio silence he differed from the architect: the staff or a scenario may set it. Built in five steps (decisions 0134 to 0138) and closed by the designer on Nov 17, 2026, with one named miss (decision 0138) |
| 0133 | Two sightings in one pulse: a contact reported this pulse is refreshed only from a hex away | Settled by the architect, Nov 16, 2026, on the designer's word. Built |
| 0134 | Commands and their pictures: step 1 of stage 2b | Settled by the architect, Nov 16, 2026, inside decision 0132. Built, one point open ("As built", the launch record) |
| 0135 | Reports between commands: step 2 of stage 2b | Settled by the architect, Nov 16, 2026, inside decision 0132. Built, three points open ("As built", "Open") |
| 0136 | What a superior knows of his forces, and radio silence: step 3 of stage 2b | Settled by the architect, Nov 16, 2026, inside decision 0132. Built, points open ("As built", "Open") |
| 0137 | Commanders, and initiative in one rule: step 4 of stage 2b | Settled by the architect, Nov 17, 2026, inside decision 0132. Built |
| 0138 | The tools and the proof of stage 2b: step 5, and the test plan written before the run | Settled by the architect, Nov 17, 2026, inside decision 0132, whose pass marks the designer confirmed on Nov 16, 2026. Tools built. Proof run Nov 17, 2026: passes as stated. Condition 4 by a stricter count made after the run: 174 of 200, which is the designer's to weigh. Closed by the designer, Nov 17, 2026, with that as a named miss |
| 0139 | Step 3, stage 3: taking and holding a place, in nine steps | Confirmed by the designer, Nov 17, 2026, with his six answers and the pass marks as written; amended by him the same day to nine steps, the staff assembling the force before the proof. Built but for "leave a place"; its proof run and the stage closed by the designer on Nov 18, 2026 with two named misses (decision 0153) |
| 0140 | A day from which a force may be used, formations, and the staff's setup of the 39 days: step 1 of stage 3 | Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its tables ruled by the architect on Nov 17, 2026 (As built) |
| 0141 | Units made ready, and a base built up: step 2 of stage 3 | Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings await the architect (As built) |
| 0142 | Taking a place: step 3 of stage 3 | Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings await the architect (As built) |
| 0143 | Ashore: the attack on the estimate, and a second lift: step 4 of stage 3, first part | Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings await the architect (As built) |
| 0144 | Holding a place with men, the supply that follows, and the check that it can be held: step 4 of stage 3, second part | Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings await the architect (As built) |
| 0145 | A run by night: step 5 of stage 3, first part | Settled by the architect, Nov 17, 2026, inside decision 0139. Built; amended after the build by item 2a, and its readings ruled (The architect's rulings) |
| 0146 | Bombardment from the sea: step 5 of stage 3, second part | Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings ruled (The architect's rulings) |
| 0147 | Reconnaissance for an operation: step 6 of stage 3, first part | Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings ruled (The architect's rulings) |
| 0148 | Cover for an operation by carriers: step 6 of stage 3, second part | Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings ruled (The architect's rulings) |
| 0149 | The force assembled by the staff: step 7 of stage 3 | Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings ruled (The architect's rulings) |
| 0150 | Leaving a place: step 8 of stage 3, first part | Settled by the architect, Nov 17, 2026, inside decision 0139. Not built; moved to after the proof of stage 3 |
| 0151 | Controlling an island: step 8 of stage 3, second part | Settled by the architect, Nov 17, 2026, inside decision 0139; amended after a review. Built; its readings ruled (The architect's rulings); item 8c, amended before the proof, built on Nov 18, 2026, its readings 12 to 16 and one finding for the architect's ruling (As built: item 8c) |
| 0152 | Forces with men aboard: what the 39 days ask of steps 7 and 8 | Settled by the architect, Nov 17, 2026, inside decision 0139. Built, with item 5b and a lost first lift mended on the loading check; its readings ruled (The architect's rulings) |
| 0153 | The proof of stage 3: the 39 days, staff against staff, from area intents alone | A plan by the architect, Nov 17, 2026. Run Nov 18, 2026: not passed as stated (the Japanese side never attacks the airfield ashore, and does worse from its intents than given nothing; every other condition passes, legibility 23 of 23). Closed by the designer on Nov 18, 2026 with both named misses |
| 0154 | The table, a prototype of the player's interface | Built as a prototype on Nov 18, 2026, on the designer's word. Not a stage of step 3; it has no test and no check in CI |
| 0155 | Stopping the clock between pulses: the direction, and what is to be planned | Direction given by the designer, Nov 18, 2026. Step 1 done in the table. The rule of step 2 is planned in decision 0156, which he confirmed the same day; it is built, and its proof is run and judged |
| 0156 | Orders inside a day: stopping the clock between pulses | Built in five steps, Nov 18, 2026. Its proof was run once and judged the same day: five of seven judged conditions hold and two miss. The designer had the first mended (the event of an order taken in now says what was ordered; six of six on a check); the second, speed at the tool on a near-empty day, is a named miss. Off for the 39 days |
| 0157 | The directive: what a commander is told from above, and where the points go | Direction given by the designer, Nov 18, 2026. Drafted and shown in the table as a prototype; nothing of it is in a scenario or the engine yet |
| 0158 | Plans a person can read, reconnaissance as part of a plan, and trying a plan: the direction | Direction given by the designer, Nov 18, 2026. The links and the brief-first sheet are drafted in the table; the rest waits for stage 4's plan |
| 0159 | A plan that knows what it waits for: the opening of step 3's stage 4 | Confirmed by the designer, Nov 18, 2026, with his six answers, each as recommended. Being built step by step; step 1, fed ashore, and step 2, a take that waits, are built; step 3, the limits to wait, is built; step 4, knowledge (rule D, items 12 and 13), is built; step 5, a force moved to look (rule D, item 14, as the architect ruled it on Nov 18, 2026), is built; step 6, the plan order (rule E), is built: all six steps are built. Its proof was run once on Nov 19, 2026: not passed as stated, one condition failing (the Japanese side's men landed at a place it has not taken are still lost to hunger, one in four against a mark of one in twenty) and every other holding; the designer closed it on Nov 19, 2026 with that as a named miss |
| 0160 | An opening appreciation: what a side believes of the enemy's places when the game begins | Confirmed by the designer, Nov 19, 2026. Step 1, the research and the data, is built (the Allied side of the 39 days holds two opening estimates, from Turner's Operation Plan A3-42 of 30 July 1942); step 2 follows stage 4's trial |
| 0161 | The trial: the staff's war game of a plan, and the estimate a commander decides on. The plan for stage 4 of step 3 | Confirmed by the designer, Nov 19, 2026, with his seven answers, each as recommended. Steps 1 to 8 are built (decisions 0162, 0163 and 0169; 0170; 0171, 0173 and 0174; 0175; 0176; 0177; 0178 and 0181, with 0204 after), and the two rules found on the way to the proof (decisions 0230 and 0231). Step 9, the proof, was run on Nov 28, 2026 (`docs/validation/stage-4-proof.md`) and did not pass as stated: conditions 3 and 13 miss on the Japanese side as expected, condition 11, legibility, misses at 2 of 6, and condition 10, the budget, is not yet read on its named machine; it is before the designer. By the designer's answers of Nov 24, 2026 it was run on the 39 days as they stood, with the two misses of conditions 3 and 13 expected and their cause named before the run. Step 10, the enemy's approaches, has no record yet |
| 0162 | A night's load spread over its ships: the first part of the first mend of stage 4 | Settled by the architect, Nov 19, 2026, inside decision 0161. Built on Nov 19, 2026 |
| 0163 | Fed with the lift: the second part of the first mend of stage 4 | Settled by the architect, Nov 19, 2026, inside decision 0161. Built on Nov 19, 2026 |
| 0164 | The plan order of a take the staff formed | Settled by the architect, Nov 19, 2026. Built on Nov 19, 2026 |
| 0165 | The 39 days with the ships of the record: a plan | Confirmed by the designer, Nov 19, 2026, with his six answers; answer 5 reversed by him on Nov 24, 2026: stage 4's proof is run first, on the 39 days as they stand, and steps 5 and 6 follow it. Step 1 is done; step 2 is decision 0166, whose part A (the Japanese destroyers of the runs) is built and whose part B (the American ships) is blocked for want of facts; of step 3 the Japanese classes are made and no scenario reads them; step 4 waits for part B |
| 0166 | Deriving a transport, and the rulings on the ships of the runs | Settled by the architect, Nov 19, 2026 |
| 0167 | What the attacked saw of a raid: its weapon and, where recognised, its aircraft. A plan | Confirmed by the designer, Nov 19, 2026, as recommended. Built on Nov 19, 2026 |
| 0168 | A raid recognised by the experience of the ship attacked: an amendment to decision 0167 | Confirmed by the designer, Nov 19, 2026, as recommended. Built on Nov 19, 2026 |
| 0169 | A take that does not go to starve: the second mend of stage 4 | Settled by the architect, Nov 19, 2026, inside decision 0161. Built on Nov 19, 2026 |
| 0170 | A world built from a picture: the one new means of stage 4 | Settled by the architect, Nov 19, 2026, inside decision 0161. Built on Nov 19, 2026 |
| 0171 | A side's own half of a picture: the first part of step 4 of stage 4 | Settled by the architect, Nov 19, 2026, inside decision 0161. Built on Nov 19, 2026 |
| 0172 | Four mends at the seat: a game file that knows its inputs, an escort's charge, a day that failed, and preview numbers | Settled by the architect, Nov 19, 2026. Built on Nov 19, 2026 |
| 0173 | What a side believed, and whose a class is: the second part of step 4 of stage 4 | Settled by the architect, Nov 19, 2026, inside decision 0161. Built on Nov 19, 2026 |
| 0174 | The enemy's half, by rule: the third part of step 4 of stage 4 | Settled by the architect, Nov 19, 2026, inside decision 0161. Built on Nov 19, 2026 |
| 0175 | A trial: one course played in a number of worlds | Settled by the architect, Nov 19, 2026, inside decision 0161, and amended on review the same day. Built on Nov 19, 2026. The sizes accepted by the designer the same day |
| 0176 | The estimate: what a trial says of a course, in counts of worlds | Settled by the architect, Nov 19, 2026, inside decision 0161, and amended on review. Built on Nov 19, 2026, and its readings ruled on. A case of a plan displaced by a take of another place is owed, with its hand calculation |
| 0177 | The window: the same course at two or three dates | Settled by the architect, Nov 19, 2026, inside decision 0161, and amended on review. Built on Nov 19, 2026, and its readings ruled on. A later date of a window is the order given later, not the landing prepared now and held; the times of a window at the full size wait for the designer |
| 0178 | A seat's trial: the tool, who runs it, and what the commander is shown | A plan, written by the architect on Nov 19, 2026, inside decision 0161, and amended on review. Built on Nov 20, 2026, and its readings ruled on the same day. The second trial for an intent in force was settled by decision 0181 |
| 0179 | The type of the aircraft claimed destroyed on the ground | A plan, written by the architect on Nov 19, 2026, and amended on review. Built on Nov 20, 2026, with decision 0180. Its readings were ruled on by the architect on Nov 20, 2026 |
| 0180 | Who made a claim, and an air group's tally of its claims | A plan, written by the architect on Nov 20, 2026, and amended on review. Built on Nov 20, 2026, with decision 0179. Its readings were ruled on by the architect on Nov 20, 2026 |
| 0181 | What-ifs and the window from a seat: a draft beside the plan, an order taken away, and the same order at later dates | A plan, written by the architect on Nov 20, 2026, inside decision 0161, and amended on review. Built on Nov 20, 2026, and its readings ruled on |
| 0182 | A game watched: agents at the seats, a watcher at the clock | A plan, written by the architect on Nov 20, 2026, at the designer's asking, and amended on review. Step 1, the session and `serve`, is built and ruled on; step 2, the table, is built, checked by script and not yet read in a browser |
| 0183 | What the enemy is scored for is shown to both sides | Built on Nov 20, 2026, to the plan as amended on review, and ruled on. A person at the table reads the briefing's text whole, the enemy's lines now among them: whether he should is put to the designer |
| 0184 | The Eastern Solomons over five days: a small scenario two commanders can both win. A plan for the designer | Confirmed by the designer and built on Nov 20, 2026. The first run of 1,000 games of staffs is no contest yet (the Allied side wins 846): a second version of the orders is the architect's to make, and whether landing the convoy is to score is put to the designer |
| 0185 | A new game screen: the scenario, the seat and who plays the other side | Built on Nov 20, 2026, to the plan as amended on review, looked at in a browser by the architect and ruled on |
| 0186 | Men put ashore score | Planned on Nov 20, 2026 and amended on review; built on Nov 20, 2026 ("As built"). |
| 0187 | A scenario's scoring is its own | In force from Nov 20, 2026 on the designer's ruling, amended on review; its one piece of code, the key that marks a scenario for play, is built with decision 0186. |
| 0188 | The Coral Sea, a scenario for play | Confirmed by the designer on Nov 20, 2026 and built. It has passed decision 0187's three guards: every figure has its reason before a run; the staffs' third run of 1,000 games is inside the band (574 and 426); and four games between agents do not go one way. It is not yet on the table's list. |
| 0189 | Difficulty | Planned on Nov 20, 2026 and amended on review; not yet built. |
| 0190 | What to strike first | Planned on Nov 20, 2026, amended on review and on a second review, and built on Nov 20, 2026 ("As built"); the readings marked there are the architect's to confirm, and scenario 105's part (rules 9, 24 and 37) is his to apply. |
| 0191 | A lift turned back | Planned on Nov 20, 2026, amended on review, amended in the build (an order to do no cargo work) and built on Nov 20, 2026 ("As built"); the readings marked there are the architect's to confirm. |
| 0192 | A station kept from the sea | Planned on Nov 20, 2026, amended on review and built on Nov 20, 2026 ("As built"); the readings marked there, and two statements of scenario 108, are the architect's to confirm. |
| 0193 | Men ashore with no more coming | Planned on Nov 20, 2026, amended on review and built on Nov 20, 2026 ("As built"); the readings marked there, and statement 9 of scenario 109, are the architect's to confirm. |
| 0194 | The last day ashore | Planned on Nov 20, 2026 and amended on review; built on Nov 20, 2026 ("As built", below), its readings with the architect. |
| 0195 | Forces that arrive | Planned on Nov 20, 2026 and amended on review; the designer's answer is taken; built on Nov 20, 2026 ("As built", below), its readings and four statements of scenario 111 with the architect. |
| 0196 | Stock from off the map | Planned on Nov 20, 2026 and amended on review; built on Nov 21, 2026 (scenario 112) and confirmed by the architect's rulings below. |
| 0197 | Men who replace the lost | Planned on Nov 20, 2026 and amended on review; built on Nov 21, 2026 (scenario 113) and confirmed by the architect's rulings below. |
| 0198 | Midway, a scenario for play | Confirmed by the designer on Nov 20, 2026; stage 0, the map, is decision 0199, built; stage A, the data, is gathered, ruled on (rules 50 to 80) and derived (`data/derivation/midway.toml`); stage B's rulings are rules 81 to 102 and its hand calculation is `tests/scenario_114_midway.rs`, written on Nov 22, 2026; stage B is built and its first game played ("Stage B: as built"), the band waiting on the architect's reading of that game. |
| 0199 | A second map | Planned and amended on review on Nov 20, 2026; the designer confirmed on Nov 21, 2026 that the present map's chart is drawn true, 2 in 100 narrower east to west; built on Nov 21, 2026 to the plan as amended, and waits for the architect's reading. |
| 0200 | A row of a later date, read by exception | Planned and amended on review on Nov 21, 2026; built the same day and confirmed by the architect's rulings below. |
| 0201 | Refuelling at sea | Planned on Nov 21, 2026, rewritten on the research and amended on two reviews. Stage A is built (Nov 21, 2026; As built, below); stages B, C and D follow. |
| 0202 | Carriers serve an order for an end | Planned on Nov 21, 2026, amended on two reviews and on the designer's word; built the same day (scenario 116) and confirmed by the architect's rulings below. The way a force takes is decision 0203, to come. |
| 0203 | The way a force takes | Planned on Nov 21, 2026; the designer's choice of limits and two reviews folded in; its hand calculation written the same day; stage 1 built the same day (scenario 117, As built below), waiting for the architect's reading. Stage 2 is not built. |
| 0204 | A dry run asked inside a day | Planned on Nov 21, 2026; its review folded in; its hand calculation written (scenario 118); rules 1 to 13 built on Nov 21, 2026 ("As built", below); stage 2 (rule 14) not built. |
| 0205 | Hunger takes time | Planned on Nov 21, 2026; its review folded in; its hand calculation written; built on Nov 21, 2026 (As built, below), for the architect's review. |
| 0206 | Real oilers | Planned on Nov 21, 2026; its review and the research's second pass folded in; its hand calculation is scenario 120; waits for its build. |
| 0207 | A figure estimated from like ships | Planned on Nov 21, 2026; its review folded in; built after decision 0206's oilers. |
| 0208 | Hungry men are slow to mend | Planned on Nov 21, 2026; its review folded in; its hand calculation written (`tests/scenario_122_hungry_men_are_slow_to_mend.rs`); built on Nov 21, 2026 ("As built"), for the architect's reading. |
| 0209 | A fight costs supply | Planned on Nov 21, 2026; its review folded in; its hand calculation written (`tests/scenario_123_a_fight_costs_supply.rs`); built on Nov 22, 2026 ("As built"), for the architect's reading. |
| 0210 | The type of the aircraft claimed shot down | Planned on Nov 22, 2026; its review folded in; its hand calculation written. Built on Nov 22, 2026 (As built, at the end), for the architect's reading of the core. |
| 0211 | A strike is spread over the ships of the kind it wants | Planned on Nov 22, 2026; reviewed and rewritten the same day; to be reviewed again and measured. |
| 0212 | Fire aboard a ship | Planned on Nov 22, 2026; reviewed and rewritten the same day; to be reviewed again and measured. |
| 0213 | A figure that differs by side and by date | Planned on Nov 22, 2026; to be reviewed. |
| 0214 | A force asks leave to go after what it has found | Planned on Nov 22, 2026; reviewed and rewritten the same day; confirmed by the designer on the points put to him. Built on Nov 22, 2026 (see "As built"), for the architect to read. |
| 0215 | The commander is told of what arrives | Planned on Nov 22, 2026; reviewed and amended the same day; built on Nov 22, 2026 (see "As built"), for the architect to read. |
| 0216 | Direct orders to a land unit at the table | Planned on Nov 22, 2026; reviewed and amended the same day; the three orders built on Nov 22, 2026 (see "As built"), rest not yet built. |
| 0217 | A deny that a carrier force serves is met, and a plan's cautions are said apart | Ruled on Nov 22, 2026; amended by the architect on the build's finding the same day; built on Nov 22, 2026 (see "As built"), for the architect to read. |
| 0218 | Submarines against warships | Ruled and amended on Nov 22, 2026; stage 1 (the data) built; stage 2 (the rule) built and merged (pull request 690); stage 3 (the staff, the agent and the table) built and merged (pull request 691). |
| 0219 | An order may be given at any stop, and acts when it can | Ruled on Nov 22, 2026; built on Nov 22, 2026 (see "As built"), for the architect to read. |
| 0220 | An order against an enemy force | Built on Nov 23, 2026 (see "As built" at the end); planned on Nov 22, 2026; amended on review; the scenario's statements written on Nov 23, 2026. |
| 0221 | The table is laid out round the stop | Planned on Nov 22, 2026; to be reviewed. |
| 0222 | An order's limits at the table, and a supply source the staff chooses | Planned on Nov 23, 2026; amended on review; confirmed by the designer; steps 1, 2 and 4 built; step 3, the staff's choice of a source, is planned again in decision 0228, which replaces rules 8 to 12 as amended. |
| 0223 | The staff places its boats its navy's way, and reads where the enemy's are | Planned on Nov 23, 2026; amended on review; stages 0 and A built; stage D1 built (see "As built: stage D1"). |
| 0224 | Rear bases, and supply from outside the map | Planned on Nov 23, 2026; amended on review; measured; the architect's rulings of Nov 23, 2026 on where the Japanese runs load and on Espiritu Santo's stock given, the fuel left open; nothing built. |
| 0225 | Boats in the games for play | Planned on Nov 23, 2026; amended on review and by the designer's answer; the Midway games built and merged (pull request 706). |
| 0226 | What is known of the enemy, on one paper | Planned on Nov 23, 2026; to be reviewed. |
| 0227 | Boats in every game for play | Planned on Nov 23, 2026; amended on review the same day; built and merged (pull request 717; see "As built"). |
| 0228 | The staff chooses a supply source | Planned on Nov 23, 2026; amended on review and by the architect's rulings on the build; built on Nov 23, 2026 (As built, below). |
| 0229 | A build of the game for the browser | Planned on Nov 24, 2026; amended on review the same day (What the review changed). Stage 1 is built (pull request 730); stage 2 is built (pull request 732), with the two tests the architect's rulings on it asked; stage 3 is built (pull request 733); stage 4 is built and merged (pull request 734), the three changes the architect's rulings on it asked and statement 16 by hand in three browsers to be confirmed against its "As built"; stages 5 to 7 are not built. Needs review: who playtests and whether the link is public, turning on the host, a new seed for each game, the marks for speed, what a playtester is told of the build, and sharing the packed data (Questions for the designer). The wording of two of `CLAUDE.md`'s "Always" rules was accepted by the designer on Nov 24, 2026 (question 7). |
| 0230 | Supply at press: the floor of a supply the staff forms is never less than a day | Planned by the architect on Nov 24, 2026; amended on review the same day. Its statements and hand calculation are written (scenario 136). Built and merged before stage 4's proof (pull request 728) |
| 0231 | Men ashore on an island's land are fed by the order for the island | Planned by the architect on Nov 24, 2026, inside decision 0161; amended on review the same day; both questions answered by the designer (the garrison's hunger is judged, and counted where a base of the side stands: decision 0161, condition 13). Its statements and hand calculation are written (scenario 135). Built and merged before stage 4's proof (pull request 726) |
| 0232 | A plan the commander writes | Confirmed. Planned by the architect on Nov 26, 2026; amended on review the same day; confirmed by the designer the same day, his three answers as recommended; step 1 built, read by the architect and merged on Nov 27, 2026, before stage 4's proof on the designer's word, the proof's set-up amended on the comparison; step 2's statements written, built, read by the architect and merged the same day, before the proof on the designer's word, the set-up amended again on its comparison; step 3 not built |
| 0233 | Ceremony by risk, and where things are written | Ruled by the architect on Nov 27, 2026, on Amit's delegation; amended on an independent review the same day; in force for records and pull requests from this one on; C to be built |
| 0234 | A scenario built on another | Ruled by the architect on Nov 27, 2026, on Amit's delegation; class 2, every step, the split of `scenario.rs` among them; to be built |
| 0235 | An event's kind and fields are declared once | Ruled by the architect on Nov 27, 2026, on Amit's delegation; amended on an independent review the same day; class 2; to be built in four pull requests, the first after items 2 and 3 of the programme |
| 0236 | Five old switches retired, and the rest ruled | Ruled by the architect on Nov 27, 2026, on Amit's word for the five and his delegation for the rest; amended on an independent reading the same day; class 2; to be built in two pull requests |
| 0237 | The first three rungs of the ladder | Confirmed by the designer on Nov 27, 2026, with the amendment on review and his answers to all eight questions; class 3; rung 1's file and hand calculation written, nothing built |
| 0238 | Switch 4 measured, and kept for now | Ruled by the architect on Nov 27, 2026, on Amit's delegation; amends decision 0236, ruling 16; switch 4 kept until the two conditions here are met; nothing to build |
| 0239 | A base formed by a march is a command | Ruled by the architect on Nov 27, 2026, on Amit's word; class 3; amended on an independent reading the same day; built in pull request 773 (`docs/decisions/as-built/0239-a-base-formed-by-a-march-is-a-command.md`); renumbered from 0237 on Nov 27, 2026, because the record of the ladder's first three rungs had taken that number first |
| 0240 | The order paper reads as the order | Approved by the designer on Nov 27, 2026; amends decision 0221, rule 8 and amended rule 12, decision 0222, rule 1, its as-built bullet "The order paper" and the paper's lines in `table/README.md`, decision 0232's as-built order of the parts (check P1), and decision 0237's as-built note on the floor's starting value; nothing built |
| 0241 | An invented game names its own things | Decided by the designer; amends the data rule "anything made up is named Placeholder" (`docs/engineering/data-rules.md`, `CLAUDE.md`); built for rung 1 |
| 0242 | A map from Papua to Midway | Planned by the architect and amended on review; approved by the designer on Nov 28, 2026; steps 1 to 4 of five are built |

<!-- End of the generated table. -->

"Needs review" means the choice affects how the game plays and the designer should confirm it. "Confirmed" means the designer has. "Settled" means it is an engineering choice with no effect on play. "Decided by the architect on the designer's delegation" means the coordinating architect decided it under the designer's delegation of Nov 13, 2026 (`docs/step2-spec.md`, "Decided by the designer", 10), which is its authority and not the designer's approval of the rule.
