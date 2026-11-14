# 0100: Step 3, stage 0: the ground the staff stands on

Status: a plan by the architect, written on Nov 14, 2026, as `docs/step3-spec.md` asks of every stage. The designer approved it and agreed decisions 4 and 8 on Nov 14, 2026. Built on Nov 14, 2026 ("As built" at the end). It adds no rule of the game and changes no result: every existing scenario gives the events and the hashes it gave before.

Summary: Step 3, stage 0: the choices a side makes inside a day become functions a side can ask, with the engine calling the same ones; the five choices made at the point of contact stay with the rules; a saved game records the rules it was played under and is refused if they differ; time is measured by a command outside the game; an experiment in building a trial's world. No rule changes and no result changes

Standing: Confirmed by the designer, Nov 14, 2026. Built Nov 14, 2026 (17 questions; saved game version 2 with the rules label 1; `sim-cli time`; the experiment on the unmerged branch `s0-experiment`)

## What stage 0 is

The spec gives stage 0 three things and one experiment:

- the choices the rules make for a unit inside a day, as functions a staff can ask;
- a saved game that says what rules it was played under;
- a measurement of what a day costs with a staff's workload on top;
- a small experiment in building a trial's private world, to find what is missing before stage 4 of step 3 leans on it.

## What was found

The code was read for every choice a rule makes on a unit's behalf during a day. There are about twenty.

- **One is already a function a side can ask:** which contact a strike group would go for, and whether it would launch (`SideView::strike_choice`, decision 0049).
- **About a dozen read only what the side knows** (its own units, its standing orders, its belief and the figures) but are written inside the code that carries them out, so nothing can ask them without running the day. They are listed in decision 1.
- **Five read the truth about the enemy.** Each is made at the point of contact, by men who can see what the staff cannot:

  | Choice | What it reads |
  | --- | --- |
  | Which enemy force a raid attacks when it arrives, and whether it finds anything | The enemy forces truly in the hex |
  | Which ship each strike group attacks | The true ships of that force |
  | How a patrol divides between the raids over it, and between escorts and bombers | The true size of each raid |
  | Whether a force that avoids slips away | Whether the enemy has sighted it |
  | Who fires at whom in a surface fight | The enemy ships truly afloat |

- **A side's view lacks some of the figures its own choices use:** the surface combat figures (how near a contact alerts a force, when a ship counts as crippled, the escorts a straggler takes), the fatigue figures and parts of the air figures. They are rules, not secrets, and are simply not handed over.
- **A saved game records the hashes of its scenario and data files and nothing of the rules.** A change of rules shows only as a hash that differs on some day of the replay. The batch file already records the version of the state's layout; the saved game does not.
- **Nothing in the repository measures time.** The only figure recorded is the soak's: 180 days in 0.27 seconds, release build. The lint forbids reading the clock anywhere in the workspace.

## Decisions

### Choices as functions

1. **Each choice that a side makes from what it knows becomes a function of that and nothing else,** and the engine calls the same function, so there is one answer and one place it is written. They are:

   | Choice | Reads |
   | --- | --- |
   | How many aircraft of a group can fly | The group, its base's supply, its ship's flight deck, its fatigue, the day it moved |
   | Whether a base strike launches this pulse | The group, its target, its reach, the base on the map |
   | How a fighter group splits into patrol and escort | The group and its patrol share |
   | Which raid the escorts go with, or whether they stay | The side's own raids of the pulse |
   | Where aircraft land when their deck is gone | The side's carriers in the hex and its bases in reach |
   | Whether a tired group stands down or returns | The group's fatigue and threshold |
   | Whether a force is alerted | Its position and readiness, and the side's contacts |
   | Whether a force is ready when a fight begins | Its readiness and when it was alerted |
   | Whether a force breaks off | Its own ships sunk and crippled, and its rules of engagement |
   | What a force does with its stragglers | Its ships' propulsion, their kinds, its rules of engagement |
   | Where a straggler or a retiring force goes | Its home port |
   | Whether a force turns back or goes on when its route changes | Its position on its leg, and the map |
   | Whether a force halts at a marked waypoint, and when it steams on | Its route, mission, cargo order and ships, and the bases on its map |
   | Whether a land unit goes back to defend | Its posture and fatigue |
   | What a land unit's day is | Decision 0099, 3b |

2. **A side can ask each of them through its view,** as it can ask about a strike today. The answer is what the engine would do at this moment with what the side holds. It is not a forecast: it does not say what the enemy will do or what the dice will give.
3. **A side's view hands over the figures those choices use.** Figures are the rules of the game and are the same for both sides. Nothing of the enemy is added to the view.
4. **The five choices made at the point of contact stay with the rules.** They are not staff decisions: the pilot over the target and the captain in the fight see what is there, and the staff does not. A staff cannot ask them and does not need to. A trial does not ask them either: it plays them, in its own private world. *Agreed by the designer, Nov 14, 2026. The other course is to make them choices from belief, so that a raid attacks the kind of ship it was told of and not the ships that are there. That would change the results of every air and surface fight and undo the tuning of step 2. The architect recommends against it.*
5. **Which base feeds, refuels or repairs is not a choice.** It is a fixed rule of allocation (the lowest-numbered base of the side in the hex, in order) and stays where it is. A staff reads the rule; it does not ask it.

### The test of it

6. **Nothing changes.** After this stage every scenario gives, byte for byte, the events, the daily hashes and the baselines it gives before. The layout of the state does not change and the hash version is not raised.
7. **Ask, then run, then compare.** For every scenario in the repository and every day of it, each function is asked from each side's view before the day runs, and its answer is set beside what the engine then did. They agree, or the difference is one this record names: something that changed inside the day before the choice was made (a ship sunk that morning, a contact reported at noon). This is the stage's main test, and it is a test of every scenario at once.

### A saved game and its rules

8. **A saved game records what it was played under:** the version of the state's layout, as a batch file does, and a rules label, which this stage begins at 1 and which a pull request raises when it changes a rule in a way that changes what an existing game would do. A replay of a game whose record differs from the engine's is refused before it runs, with the two versions named. *Agreed by the designer, Nov 14, 2026. An older saved game is refused, with the reason, and not converted. A game in progress does not survive a change of rules, as decision 0055 already said. The other course is to carry old games forward, which would mean keeping every old rule alive.*
9. **The recorded games in the repository are remade once more** in the new format. From then on a rule change that would break them says so at once.
10. **Step 3 adds to the record as it goes:** the version of the doctrine, when there is one, and of the trial's settings.

### The measurement

11. **Time is measured by a command of `sim-cli`, outside the game.** The lint against reading the clock stays for the whole simulation; the one command that measures is exempted by name, and nothing it measures can reach a result.
12. **What it measures,** on a named machine, each as a spread over repeated runs:

    | Workload | What it stands for |
    | --- | --- |
    | One day of the full slice, early and late in the game | The cost a staff is added to |
    | That day with every function of decision 1 asked for every unit of both sides | A staff's daily decisions, at their dearest |
    | A copy of the world, early and late, with and without its event log | What a trial pays before it plays a day |
    | A hundred games of 30 days from one start | A trial of the size the spec has in mind |
    | The same on the soak at day 150 | A long game with its history behind it |

13. **Its report says what a trial could cost and what would have to change to make it cheaper,** and gives the designer the numbers to set the times the spec's "Speed" asks him to accept. The 39-day slice is measured when it exists; the 14-day slice and the soak serve until then.

### The constructor experiment

14. **A world is built from a side's view, three times,** from the slice on a quiet day, on the day of the carrier battle, and late in the game: once from each side. It is an experiment on a branch that is not merged. What is kept is its report: what a view gives, what had to be assumed, what could not be built at all, and what the view or the scenario must say for stage 4 to do it properly.

## What it leaves for later stages

- **A staff.** Nothing here decides anything new.
- **Opening both seats to agents through the game's own server.** One side is still a script there; the test server opens both.
- **Forecasting.** A function says what a unit would do now. What will happen is the trial's.

## What it costs

- **A refactor across the air, surface, movement, straggler, landing and ground code,** with no change in behaviour. It touches the files every ground rule in flight touches, so it is built when they have merged, not beside them.
- **New methods on a side's view, and the figures it hands over.** Each is checked by the leak tests: two worlds that differ only in what a side cannot see give the same answers.
- **The agent's tools gain the questions,** and the rulebook gains, for each choice, the sentence that says a side can ask it. Chapter 1 gains the line between what a commander decides and what is decided at the point of contact.
- **The saved-game format changes once,** and the recorded games are remade.

## Rule card

| | |
| --- | --- |
| Reads | What each choice reads today |
| Writes | Nothing new |

No rule is added, changed or removed. The rulebook changes to say which choices a side can ask, which is a fact about the interface, and the glossary gains "point of contact".

## Questions for the designer

Answered on Nov 14, 2026.

1. **Decision 4:** the five choices made at the point of contact stay with the rules and read what is truly there. Agreed.
2. **Decision 8:** a saved game from before a change of rules is refused, with the reason, and is not converted. Agreed.

## As built

Built on Nov 14, 2026. Where the plan left a choice open, the smallest thing consistent with it was taken; each is listed here.

### No result changed

Every scenario under `data/scenarios` and `data/scenarios/step2-frozen` (58 files) was run before and after with `run --events`, `hash`, `hash --seed 7` and `export`, and the five battles with `batch --runs 40`: all 237 files are byte for byte the same. Every run baseline, golden file, pinned hash and recorded transcript is unchanged. The state layout is unchanged (`flagplot-state-v18`), and the hash version was not raised. The only files whose format changed are the saved games, as decision 8 says.

### The choices as functions (decisions 1 to 5)

- **The questions are types.** `sim_core::choices` holds `Question` and `Answer`, one variant for each area (air, surface, movement, ground), each wrapping that area's own enum in `choices/air.rs`, `surface.rs`, `movement.rs` and `ground.rs`. `SideView::ask(&Question) -> Option<Answer>` answers one; `SideView::questions()` lists every question the side could ask now, one for each of its units each choice applies to. The answering code is in `view/*_choices.rs`, child modules of the view, which read only the side's own units, its belief, the public bases (`enemy_bases`) and the figures.
- **Seventeen questions,** the fifteen choices of decision 1 with the halt and its steaming on asked apart, and the strike choice of decision 0049 and 0063 brought in beside them (`Strike`): `Strike`, `AircraftAbleToFly`, `BaseStrike`, `FighterSplit`, `Escort`, `Landing`, `Rest`, `Alert`, `Ready`, `BreakOff`, `Stragglers`, `HeadHome`, `Reroute`, `Halt`, `SteamOn`, `BackToDefend`, `UnitDay`. Each is one pure function (for example `airstrike::able_to_fly`, `landing::place_landings`, `surface::alert_choice`, `stragglers::straggler_plan`, `TaskForce::reroute_centre`, `movement::halt_index`, `ground_combat::unit_day`), which the engine calls where it made the choice and acts on exactly as before.
- **A question carries what exists only inside its step.** A break-off is asked with the ships the fight began with, whether the force was fired on and whether it is already avoiding; going back to defend with the fatigue to test; a land unit's day with whether it marched, whether its hex is contested and the force it is rostered with; a landing with the groups in the air and their aircraft; a base strike and readiness with the pulse. The side states them. `questions()` fills them from the side's present state: a force's present ships as those a fight began with, not fired on; a unit's present fatigue; marched if a march is under way; contested only where one of the side's own fighting units in the hex is on attack and the side holds an estimate of enemy men there (a side cannot know the hex is contested); readiness for the coming night; a landing from the side's own sorties, or its ready aircraft, for its groups on a ship whose deck is gone.
- **The view gains the pulse's scratch and the figures.** `SideView` holds, by reference, the bases grounded at the pulse's start, the strikes in flight, the sorties and the decks lost, each list holding both sides and read only for the side's own; and it hands over `surface_combat_figures`, `air_combat_figures`, `air_upkeep_figures` and `fatigue_figures` (decision 3).
- **Two code paths were restructured with no change of result.** Coming home to no deck now gathers each side's groups, places them by `place_landings` (the room aboard a carrier is her aircraft at the step's start plus what earlier groups put aboard, which is what the old code read as it went), and then lands them in the same order with the same draws. The straggler pass works out a plan first (`straggler_plan`) and carries it out in the old order; whether the parent force is left empty is still read from the state, as before.
- **The work tests of a halt read through one struct.** `HaltGround` holds references to the reference data, the map and the ships, bases and land units; the rules index it only by the force's own ships, its cargo order's unit and the bases of its side. A bombardment's target is an enemy base in the hex that is on the side's map, which is what `enemy_bases()` gives, so the engine and the view agree. The type does not enforce the side check; the leak tests do.
- **Two differences the code allows and no scenario shows.** An enemy base off the side's map (formed by a landing) is never a base strike's target in the view, while the engine's launch test reads only the target's side and hex; an order against such a base is refused, so it arises only if a base leaves the map after the order. A home port that is an off-map enemy base would be answered with no hex. Neither occurs in any scenario.
- **The agent asks them.** `measure` takes a sixth question, `{"question": "choices", "unit": HANDLE}`, answering every choice for one of the side's own air groups, task forces or land units in the side's own handles, with a note that it is not a forecast and that the five choices at the point of contact are not asked. It is an addition: the interface stays at version 3, as when the yard question was added. The tool's description gained its sentence.
- **The rulebook** gains, in chapter 1, the line between what a commander decides and what is decided at the point of contact, with the table of the choices a side can ask; each chapter gains the sentence that says a side can ask the choice, and, for the five, that it cannot. The glossary gains "point of contact", "choice a side can ask" and "rules label".

### The test of it (decisions 6 and 7)

- **A trace of choices.** A world can keep a trace (`World::trace_choices`, `take_choice_trace`): for each choice the engine makes, the moment of the day, the side, the question and the answer. It is not state: not hashed, not saved, read by no rule, off unless a test turns it on; it is kept in a mutex so that a world can still be shared between threads for reading. `World::resolve_day_watched` resolves a day and stops at each moment (the day begins, the orders, each pulse, each step, each step after the last pulse), handing the world, read only, to the caller; `resolve_day` is it with nothing watching.
- **Ask, then run, then compare** is `tests/choices_asked_and_run.rs`. For every scenario and every day, the day is run once with the trace on, and again from the same start asking each traced question from its side's view before the day and at the moment before the step that made the choice. The two runs give the same hash. Asked at that moment, every answer is the engine's, except six kinds of choice made in the middle of a step after the step changed what they read, which the test names and which are the only kinds allowed to differ: a break-off after the fight's rounds hit the force; the straggler pass run again after a fight; a force the straggler pass detached in the step, sent home, routed and moved in the same step (head home, reroute, halt), which did not exist before it; and a land unit's day after its men were lost in the fight ashore earlier in the step. Asked before the day, an answer that differs is one of those, or one that agrees at the moment of its step, so something changed inside the day before the choice. Over all the scenarios the engine made about 145,000 choices: 21,877 of aircraft able to fly, 41,267 alerts, 56,889 straggler passes, 7,484 strike choices, among the rest. 73 differed inside their step (36 break-offs, 25 straggler passes, 3 halts, 3 head homes, 4 reroutes, 2 days of land units); about 14,200 agreed at the moment of their step and not before the day.
- **Leak tests.** Each area's tests build two worlds that differ only in what the side cannot see (the enemy's ships, groups, damage, positions, bases' stores, land units, strikes in flight) and assert that every question of `questions()` gives the same answers; the agent's `leaks.rs` adds every own unit's `choices` answer to what it compares.

### A saved game and its rules (decisions 8 to 10)

- The rules label is `sim_core::world::RULES_VERSION`, 1. The saved game is version 2 and records `state_version` (the hash's layout string, as the batch file records it) and `rules_version`. Every reader of a saved game (`replay`, `events`, `export`, `play --export-save`) reads it through one function, which refuses before anything runs a version 1 save ("saved before saved games recorded the rules they were played under ... it is not converted, so play it again") and a save whose state layout or rules label differs, naming both. The batch file and the replay file for the viewer are unchanged: they record the state layout already, and decision 8 names the saved game.
- The three recorded games were remade with `regame.py`. Their transcripts are byte for byte the same; each `save.json` differs only in the version and the two new keys. No other saved game is kept in the repository.

### The measurement (decisions 11 to 13)

`sim-cli time --machine TEXT` (in `crates/sim-cli/src/timing.rs`) is the one place in the workspace that reads the clock, in one function exempted from the lint by name. On an Apple M3 Pro (11 cores, 36 GB, macOS 27.2), release build, medians of 20 runs:

| Workload | Time |
| --- | --- |
| One day of the full slice, day 2 from day 1 | 2.46 ms |
| One day of the full slice, day 13 from day 12 | 59 µs |
| Day 2 with every question of both sides asked at every one of the day's 35 moments (5,040 questions) | 5.19 ms |
| Day 13 the same (6,051 questions) | 442 µs |
| The questions of both sides asked once before the day (144 early, 171 late) | 6 µs, 11 µs |
| A copy of the slice's world at day 1 / day 12 (37 / 1,376 events) | 19 µs / 80 µs |
| A copy of the soak's world at day 150 (14,950 events) | 717 µs, of which the event log about 680 µs |
| 100 games of 30 days from the start of the soak | 3.2 s (32 ms a game) |
| 100 games of 30 days from the soak at day 150 | 3.7 s (37 ms a game) |

What it means. A staff's questions cost little beside a day: asked once, they are microseconds; asked at every moment of a day, they double the dearest day of the slice. A trial of a hundred games of thirty days costs between three and four seconds on this machine, and the history behind a late game adds about a sixth. What would make a trial cheaper is the copy: late in a long game it is almost all the event log, so a trial's world that starts with an empty log (as a world built from a side's view does) copies in tens of microseconds. A copy keeps its seed, so the hundred games are one game played a hundred times; a trial's own stream (stage 4) changes what the games are, not what they cost. Why the slice's second day costs forty times its thirteenth was not looked into. The 39-day slice is measured when it exists.

### The constructor experiment (decision 14)

Built on the branch `s0-experiment`, which is not merged (commit 76722b4; the code is `crates/sim-cli/tests/stage0_constructor.rs` there, and its full report `docs/research/step3-stage0-constructor-experiment.md`). A world was built from one side's view of the full slice, from each side, after day 2 (quiet), day 3 (the carrier battle the next day), day 4 (the battle just fought) and day 12 (late), and two days were resolved in each. What it found:

- **Every build was accepted and ran, and almost every build was wrong in the way that matters.** Before the battle the Allied view held no contact of the Japanese carriers, so every world built then had no enemy fleet at all, and resolved in peace the day the real carriers struck. A world with nothing unseen is valid and proves nothing, as the spec warns.
- **What the view gives:** the side's own units whole (ships, task forces, bases, air groups, land units, replacements, arrivals, with their standing orders), the enemy's bases (number, name, hex, side), contacts, claims, ground estimates, its own mission and objectives, its visible events, and the classes and types. A contact is a place, a count and one kind.
- **What had to be assumed, item by item:** the enemy's ship classes (the class tables have no side, so the rule taken was the lowest-numbered class of the kind the side does not use itself); a contact's make-up (a "9 x carrier" contact first gave nine carriers where the truth was 2 carriers, a battleship, 4 cruisers and 11 destroyers; the rule became one ship of the kind and destroyers for the rest); every enemy ship's fuel, ammunition, damage and experience; the aircraft aboard an enemy carrier (by its capacity, by invented shares); the enemy's bases' supply, fuel and aircraft; the enemy's orders and missions (none); the enemy's belief of us (none, or a fresh contact on every own force at the worst); the seed; and forces once reported and lost, placed at their last report rather than over all the places they could have reached.
- **What could not be built without new means in the engine:** a world with a day behind it (starting contacts are taken only on day 0); a claim or an estimate written into a new world's belief; the order book and the event log the side's own units point into (by day 12, five own forces cite events the new world does not have); the counters of units made in play, which restart and give a made unit the number of one that exists (the slice holds made units from day 4); the enemy's objectives, so no margin can be scored; the side's known progress, which is worked from the log; and enemy land units, replacements, arrivals and bases formed by landing. The experiment added three helpers to sim-core on its branch to go on.
- **What stage 4 needs from the view or the scenario:** the scenario must say, for each side, the enemy forces the side believes exist and has not seen, and "not seen" must be a different input from "believed absent"; a contact needs a guess of its make-up, and the class and type tables a side; a stated rule for raising a contact's kind from a claim (on day 4 the Allied contact read cruisers and only the raid's claim named the carriers); positions for the unseen drawn from where they could have gone, from the trial's own stream (after eight days that is most of the map, so one world holds one draw); one checked constructor that takes the day, the belief with its ages, claims and estimates, the counters of made units and a mark for history, in place of helpers; doctrine for the invented enemy, which otherwise does nothing (six carriers declared at its base did nothing in two days); and a seed from permitted inputs.
- **The hard cases of stage 4:** no contact and a carrier never seen arose (the same case: an empty sea); an old contact arose and was placed where it was reported; two reports of one force arose and the crude rule used (one kind inside the reach of an earlier one) made one force of two or two of one; a contact of the wrong kind arose both ways (a carrier reported where there were cruisers was built as a carrier with its aircraft; carriers reported as cruisers were found only through a claim); a claim of a sinking too high did not arise (claims equalled the truth on the days built), and the constructor does not subtract claims, since a claim is never below the truth and subtracting would leave too few.
- **No leak was found** in the view: every question and query asked with the enemy's numbers answered as for a unit that does not exist.
- **Cost:** a build is cheap (thirteen builds and about forty resolved days in under two seconds in a debug build); the cost of a trial is its games, not its world.
