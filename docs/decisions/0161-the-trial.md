# 0161: The trial: the staff's war game of a plan, and the estimate a commander decides on. The plan for stage 4 of step 3

Status: a plan by the architect, written on Nov 19, 2026 and confirmed by the designer the same day with the seven answers the architect recommended. Nothing is built yet. It settles the contracts `docs/step3-spec.md` leaves to this record ("The trial") and takes in the two mends decision 0159 left for it.

Summary: The plan for stage 4 of step 3. A commander who has stated an intent can ask the staff to try the plan before it goes: the staff builds private worlds from what the side knows and believes, never from the live enemy, plays the plan in each with both sides on doctrine, and hands in an estimate: what the plan needs, when it could go, what it may cost, how often it came off, and what was assumed about what is not seen. The same course is tried at two or three dates, so the estimate says what waiting buys and what the enemy may do with the days. Nine steps, the first two the mends of decision 0159; a world built from a picture is the one new means in the engine; a new crate plays the trials. With the conditions of its proof and seven questions for the designer.

Standing: Confirmed by the designer, Nov 19, 2026, with his seven answers, each as recommended. Steps 1 to 8 are built (decisions 0162, 0163 and 0169; 0170; 0171, 0173 and 0174; 0175; 0176; 0177; 0178 and 0181, with 0204 after). Step 9, the proof, is not run: by the designer's two answers of Nov 24, 2026 it is run on the 39 days as they stand, before the ships of the record, and the times at the full size are accepted; it waits for the measurement of the men unfed and the record of the fallen-back men, and for its setup. Step 10, the enemy's approaches, has no record yet

## The target

Stated to be ambitious and reachable, as step 3's target is.

On the 39 days, from the Allied seat on the evening of 6 August, a commander states "control Guadalcanal", asks for a dry run, and inside the time the designer accepts reads an estimate that says, in a page: what the operation needs and has; that it could land on 7 August or wait for a named thing; in how many of the worlds tried the airfield was taken by a stated day and held to the end of the trial, and at what cost in men and ships between which bounds; what the enemy was assumed to hold and to want, with the cases where his carriers come and where they do not shown apart; and what waiting to a second date would buy. The same question asked twice gives the same page. Asking it changes nothing in the game. Two games that differ only in what the commander cannot see give the same page.

It is proved by a test that can fail: the estimate's spreads are checked against the engine it runs, over independent games, by a rule with numbers written before the run.

## What is so today, read from the code and the records

- **Doctrine is complete by itself.** A staff with no trial plays a side (stages 1 to 3 and decision 0159). The same staff plays either side from that side's view and intents.
- **A plan says what it waits for and when it expects to go** (decision 0159, the plan order): phases, what is waited for, men wanted and to hand, the day expected ashore. It says nothing of what the operation may cost or whether it is likely to come off.
- **A world cannot be built from a view** (decision 0100's experiment, `docs/research/step3-stage0-constructor-experiment.md` on the branch `s0-experiment`). The view gives the side's own units whole and the enemy as bases, contacts, claims and estimates. What could not be built without new means: a world with a day behind it; a claim or an estimate written into a new world's belief; the order book and log the side's units point into; the counters of units made in play; the enemy's objectives. Every world built before the carrier battle held no enemy fleet.
- **What a side believes before the first day** is being given to the 39 days now (decision 0160): estimates of men ashore, then an enemy base's works.
- **Speed.** A staffed day of the 39 days takes about 17 ms for both sides' decisions and resolves in 1 to 11 ms (stage 3's proof, Apple M3 Pro, release). A game of 39 days staff against staff takes about 0.8 s.
- **Two mends are owed** (decision 0159, "Closed on Nov 19, 2026"): the supply of men ashore is formed days after they land and a night's run cannot feed them; and a take lifts men to a place where its own count says they cannot be enough.

## The rules

### A. The two mends, before the trial

They come first because a trial plays the doctrine: a doctrine that starves the men it lands makes every trial of a landing say so. Each is a decision record of its own with its hand calculation, written by the architect when this plan is confirmed. Their shape:

1. **Fed with the lift.** The supply of men landed at a place not yet the side's is formed on the morning their lift is released, sized to the men the lift carries and the days the staff expects them to wait, and counted in the lift's shipping; where the runs the staff may send cannot feed them, the take says so before it sails.
2. **A take that does not go to starve.** Where the men a take can bring within its horizon are too few by its own count at its risk, and its own supply cannot feed them where they would wait, it does not lift: it hands in "cannot bring enough" and waits, as rule B of decision 0159 already does where a latest day is set. At press the commander has said to go, and it goes.

### B. A world built from a picture

3. **A picture** is plain data: everything a world needs, with nothing of the live world in it. It holds the day and pulse; one side's own units, bases, stocks, orders and command state as that side's view gives them; and for the other side, forces, bases, stocks, land units and command state as assumed. It holds what each side believes of the other. It is not state and is never saved.
4. **`sim-core` builds a world from a picture,** by one checked constructor, and refuses a picture that breaks an invariant. This is the one new means in the engine. It takes the day, belief with its ages, claims and estimates, the counters of made units, and orders in force, in place of the helpers the experiment used. A world so built has no history before its day: an event that would cite an earlier cause cites the picture.
5. **A trial's draws come from a stream of its own,** keyed on the trial's number, the world's number, the day, the pulse, the subsystem and the entity. It is a new seed domain, added at the end; the live game's stream is never read or advanced.

### C. The constructor: from a view to pictures

6. **A new crate, `sim-trial`,** depends on `sim-core` and `sim-staff`. It takes a `SideView`, the scenario's statement of what the side believed, and a trial's settings, and nothing else: never the world, the umpire's view or the full log. `docs/engineering/layout.md` gains its row.
7. **The side's own half is copied from its view.** The enemy's half is built by written rules, each named in the estimate where it mattered:
   - **A contact becomes a force** of its kind and count, with the make-up the rule gives (one ship of the kind reported and escorts for the rest, as the experiment settled), in the classes the scenario's reference data marks as that side's, fresh in fuel, ammunition and damage. A claim raises a contact's kind by a stated rule and is never subtracted from a count.
   - **An old contact is placed where it could have gone:** within the distance its kind steams in the time since the report, drawn from the trial's stream, one draw a world.
   - **Two reports that may be one force** are one force in some worlds and two in others, by a stated rule on kind, time and reach; the estimate says which worlds were which where the result turns on it.
   - **An enemy base** has the works and the men the side believes (decision 0160), and stocks by a stated assumption where nothing is believed.
   - **The unseen.** The scenario states, for each side, the enemy forces it believes exist and has not seen: a kind, a strength, where it was last known or that it is not known, and one of three standings: believed present, believed absent, not known. A force not known is in some worlds and not in others, and the estimate shows the two cases apart with no chance put on either. A force believed present is in every world, placed by reach from where it was last known.
8. **What the enemy wants inside a trial** is stated, not found out. The scenario gives, for each side, what it believes the enemy intends: intents in the staff's own vocabulary ("hold Lunga", "deny these waters"). With none stated the enemy holds what he holds. The estimate names the assumption. The enemy in a trial is the same staff on doctrine, reading the picture's enemy view; it is told nothing of the plan tried.
9. **No peek.** Two live worlds that differ only in what the side cannot see give the same pictures, byte for byte. The constructor is tested on the spec's hard cases: no contact; an old contact; two reports of one force; a claim of a sinking that is too high; a contact of the wrong kind; an enemy carrier never seen. Each gives a valid world, and across a trial the worlds hold what rule 7 says they must.

### D. A trial

10. **A trial plays one course in a number of worlds for a number of days,** both sides on doctrine alone. No trial is asked for inside a trial. It is given the side's command state as it is, with the draft tried standing as approved and leave given, in a private copy: the side's other plans go on and draw on the same troops, shipping and stocks.
11. **Its size is fixed work, not time.** Three sizes are offered, each a number of worlds and a horizon in days, set once from a measurement on the named machine with the staff and all, and recorded as placeholders: small, standard and full. The course chosen never depends on how fast the machine was. A trial can be abandoned; nothing of it remains.
12. **A trial changes nothing.** No state, belief, commitment, readiness or stock of the live game is read for writing; the live game's hash is the same after it. Its result is kept with the session, outside the state, as an estimate the commander was shown.

### E. The estimate

13. **An estimate is handed in with the plan order.** For the course tried it gives:
    - **what it needs and has:** men, lift, supply for the days, cover, and what must be known, from the plan order;
    - **when it could go:** the day, and what it waits for;
    - **how it went in the worlds tried,** in counts of worlds and never as a chance: in how many the end was met and by what day (earliest, middle, latest); in how many it was met and lost again; in how many it was not met by the horizon, which are reported and not dropped;
    - **what it cost,** as the lowest, the middle and the highest over the worlds: the side's men lost, by fire and unfed apart; ships sunk and out of action; aircraft; and what it believes it cost the enemy, marked as the trial's own and not a claim;
    - **what its other plans lost:** each plan the course displaced, with what it gave up;
    - **what was assumed:** the enemy's forces as built, the unseen and their cases, what the enemy was taken to want, and each rule of the constructor that changed a figure;
    - **where the cases part:** where the result differs between the worlds with a force not known and those without, the two are shown side by side.
14. **A report it foresees is not a report.** An estimate never shows a drawn enemy as known and reading one changes no belief. It may say that looking first would narrow it, from the difference between its cases.
15. **No points.** The estimate speaks in places, days, men, ships and aircraft. The objectives scale is the umpire's.

### F. The window: what if we wait

16. **The same course is tried at two or three dates,** chosen from when something changes and not at even steps: the first day it could go; and the next days on which a force comes available, a stock is landed, a repair is done, a unit's training or rehearsal is complete, or a tasked search is due to report. The days between are played, with preparation, use of supply and the enemy's moves.
17. **Each later date says what it buys and what it risks:** what the side has more of on that day, and what the enemy did with the same days in the worlds tried.

### G. Courses, and what a seat and the table are given

18. **A what-if varies one thing:** the date, the troops, the shipping, a reserve, the risk or a priority, as a draft beside the plan. Trying one changes no commitment; approving one applies that one, by the commands a seat already has.
19. **Where a limit binds the staff offers the comparison itself:** keep the convoy where it is, or borrow it and take the shortage, each with its estimate at the small size.
20. **The staff's own decisions stay doctrine's.** A staff does not launch, hold or choose by a trial in this stage, with one exception: before it asks leave to launch a take, it attaches a trial at the small size to the plan order, so the commander is asked with the estimate in hand. Doctrine weighed by trials is stage 5's question.
21. **A seat's tool `trial`** takes an intent of the commander's, a size and, where wanted, what to vary; it answers with the estimate, or with why it cannot be tried. The table's plan sheet gains "Dry run", shows the estimate under the plan order, brief first with the assumptions one step down, and the window as two or three columns.

### H. What is left out

22. Looking back at a live game ("what if we had started sooner"). The spec keeps it apart as a stated sensitivity; it is not in this stage's first build. The controlled comparison that proves preparation matters is in the proof.
23. A staff that reorganises its fighting forces or chooses between plans by trial (stage 5).
24. Unit types ashore and standing pictures of the enemy's ships and aircraft beyond contacts and the believed forces of rule 7 (the intelligence design).

## Rule card

| | |
| --- | --- |
| Reads | A side's view whole; the scenario's statement of what each side believes exists and intends; the reference data, with which side uses a class or type; the trial's settings |
| Writes | Nothing in the live game. Private worlds, dropped when the trial ends; an estimate kept with the session |
| Reuses | The engine's day, unchanged; both staffs on doctrine; the plan order and the need to know of decision 0159; the opening appreciation of decision 0160; the measurement of decision 0100 |
| New | In `sim-core`: a picture, one constructor of a world from it, and one seed domain. A crate, `sim-trial`. In a scenario: believed forces and believed intents, for each side, and which side uses a class. A seat's tool and its answer. Two rules of doctrine (the mends), each its own record |
| Draws | Placing an old contact and an unseen force; whether two reports are one force; which worlds hold a force not known. All from the trial's own stream |

## What it costs

- The mends change the doctrine for every take: the label is raised and the files of games with a take are remade, twice.
- A world from a picture is new surface in `sim-core`, with its invariants and tests. No state is added to the live game; the hash of a live game does not change.
- The 39 days gains believed forces and believed intents for each side. Those of the Allied side before 7 August are historical beliefs and need rows, by the data rules; until then they are marked placeholder.
- Time: a trial at the standard size of the main operation is of the order of some tens of games of two weeks, each of the order of a third of a second today. The sizes are set by measurement in step 5, and the designer accepts the times.

## The arithmetic of its cases

Each step's test carries its hand calculation, written by the architect in the test's doc comment before the step is delegated. The cases the plan rests on:

- **A world from a picture, by hand.** Two forces and one base a side, built on day 5 with stated stocks and one contact a side: the world's first morning is day 5, the counters stand where the picture put them, and three days resolve as the same forces started fresh on day 1 with the same orders do, but for the day.
- **An old contact.** A cruiser force reported 2 days ago at a hex, 16 kn: it lies within 768 nm of the report in every world, at the report in none but by chance, and its place differs between worlds and is the same in the same world asked twice.
- **The unseen, three standings.** One carrier force believed present, last known at a base 6 days ago: in 8 of 8 worlds. The same not known: in 4 of 8, the cases shown apart. The same believed absent: in none.
- **Sampling, small enough to count.** A landing of 600 men against a place whose garrison is believed to be 200 or 600, not known which, four worlds: taken in the two with 200 by a stated day, not taken in the two with 600, by the land rules' own arithmetic worked by hand.
- **The window.** A take that could go on day 3 with one unit or on day 6 with two: two dates, and the second's estimate shows the men it has more and the days the enemy had.
- **Nothing changes.** The live game's daily hashes with a trial asked every morning are those of the game never asked.

## The conditions of its proof

Written before anything is built. None is a tuning mark; each can fail.

1. **The hand calculations** of the steps pass.
2. **Nothing changed where it is unused.** With no trial asked, every game plays as before step 3 of this plan; the mends change only games with a take, and what changes is stated.
3. **The mends hold where 0159 missed.** On the 39 days staff against staff, 1,000 seeds: of the men a side lands at a place it has not taken, the median lost to hunger is no more than one in twenty, each side by itself, by decision 0159's reading unchanged.
4. **No peek.** Pairs of live games that differ only in what a side cannot see give the same estimate, byte for byte, on every built case and on the 39 days at three days.
5. **It is the same twice, and changes nothing.** The same question gives the same estimate; the live game's hashes are those of the game never asked.
6. **Its sampling is right,** in the small cases worked by hand.
7. **Its spreads hold.** On built cases where the picture is right, over 200 independent games each: the outcome falls inside the estimate's lowest-to-highest for each cost in at least 9 games in 10, and the count of worlds in which the end was met is within a band, stated with the step's hand calculation, of how often it is met in the games. Where the middle half of the spread is wider than a stated share of the middle value, the estimate is counted as too wide to be of use, and the count is reported.
8. **A wrong picture moves it as expected.** Controlled cases: the garrison believed half the truth; a carrier force believed absent that is there; an enemy intent not as stated. In each the estimate is wrong in the direction worked by hand, and the estimate's own statement of assumptions names the thing that was wrong.
9. **Preparation matters, by a controlled comparison:** the same declared start and the same landing date, with only the day preparation began changed; the earlier start is ashore with more of its men fit in at least 9 worlds in 10.
10. **Within its budget.** A trial of "control Guadalcanal" on the evening of day 1, at each size, late in the campaign as well as early, inside the times the designer accepts, on the named machine.
11. **Legible.** A reader given an estimate alone says what the operation needs, when it could go, how often it came off, what it may cost and what was assumed: six of six.
12. **Reported, not judged:** on the 39 days, the estimate of day 1 beside what the 1,000 games then did.

## The steps

Each is one branch and one pull request, read by the architect before it merges.

1. **Fed with the lift** (A, 1): its record, scenario and test.
2. **A take that does not go to starve** (A, 2): its record, scenario and test.
3. **A world from a picture** (B): `sim-core`, with its invariants.
4. **The constructor** (C): `sim-trial`, the scenario's believed forces and intents, the hard cases, no peek.
5. **A trial** (D): the sizes, measured; abandoned; changes nothing.
6. **The estimate** (E): the data, the sampling cases.
7. **The window** (F).
8. **Courses, the tool and the table** (G).
9. **The proof,** run once.
10. **The enemy's approaches** (added on Nov 19, 2026, below): its own record and plan, confirmed by the designer before it is built. It comes after the proof and is not judged by it.

## Questions for the designer

1. **Counts of worlds, or chances?** "Taken by day 9 in 6 of 8 worlds", not "75%". The architect recommends counts: a chance claims more than eight or thirty-two games can know, and a staff officer would say "in most of the cases we tried".
2. **The unseen with nothing to support a chance: shown as two cases apart.** "If his carriers come: ... If they do not: ...". The architect recommends this over folding them into one spread.
3. **What the enemy wants is stated by the scenario** as what the side believes of him, and shown; with nothing stated he holds what he holds. The architect recommends this over a staff that guesses his intent.
4. **The staff attaches a small trial before it asks leave to launch a take,** and otherwise does not act on trials in this stage. The architect recommends it: the commander is asked with an estimate in hand, and doctrine stays readable.
5. **The mends first,** as steps 1 and 2. The architect recommends it: every trial of a landing is poorer until they are in.
6. **Looking back is left out** of this stage's first build. The architect recommends it.
7. **No points in an estimate.** Places, days, men, ships and aircraft only, as the directive from above has none. The architect recommends it.

## Confirmed on Nov 19, 2026

The designer confirmed the plan: "go with your recommendations."

1. Counts of worlds, not chances.
2. An unseen force with nothing to support a chance is shown as two cases apart.
3. What the enemy wants is stated by the scenario as what the side believes of him, and shown.
4. The staff attaches a small trial before it asks leave to launch a take, and otherwise does not act on trials in this stage.
5. The mends first.
6. Looking back is left out of this stage's first build.
7. No points in an estimate.

He also named the thing a person presses: a "dry run" or something of that sort, not "try it". The table's button is "Dry run"; the seat's tool keeps the name `trial`, and the records go on calling the thing a trial.

## Added on Nov 19, 2026: a tenth step, the enemy's approaches

The designer asked whether the staff works out where the enemy is likely to come from, so that it can plan a landing with that in mind and set a force, or part of one, to deny the way. It does not. Around a take it denies a circle of fixed size round the place (decision 0151), stations its carriers within their bombers' reach of it, searches, and neutralises the enemy bases it knows. Nothing in that is reasoned from the enemy's likely routes. The designer added the work to this stage.

What the step is to give, each point to be settled in its own decision record, which is a plan for the designer's confirmation before anything is built:

1. **An estimate of the approaches.** For a place the side means to take or hold: where the enemy could come from, read from the side's own picture and nothing else (the enemy bases it knows, the forces it believes he has and what it believes he intends, which are rules 7 and 8 of this record, and its contacts), the ways by sea from each to the place by the engine's own routes, and how soon a force from each could be there. It is part of the plan order, and is drawn on the table's chart with the plan.
2. **A force set across an approach.** Where the side has a surface force with no other task, or a force large enough to part with some of its ships, the staff may set it on an approach rather than leave it inside the circle; and it lays its searches across the approaches. Whether it detaches part of a force, and by what rule, is that record's question.
3. **The landing's days read against the enemy's.** The plan order says the earliest the enemy could arrive beside the day the men are to be ashore and the days the ships will lie off the beach.
4. **A dry run shows what the approach costs.** The courses a trial plays for the enemy (rule 8) are the same ones the estimate names, so a commander can see the plan with the covering force and without it.

Where it stands in the order: after step 9. The trial's pictures are what the estimate is made from, so it cannot come before step 4; and it changes what a staff does in a live game, which this record's proof must not have moving under it, so it is not built before the proof is run. It has a proof of its own, set in its record. It is doctrine, chosen by written rule; a staff that chooses between such dispositions by trial stays stage 5's (rule 23).

## Where the stage stands, Nov 24, 2026

The architect's reading, made from the records and the code on main, before the proof's setup is written. It changes no rule, figure, statement or condition. Where it fixes how a condition is read, it does so before any run of the proof is made, and says so.

### What is built

| Step | Records | Standing |
| --- | --- | --- |
| 1, fed with the lift | 0162, 0163 | Built, ruled |
| 2, a take that does not go to starve | 0169 | Built, ruled |
| 3, a world from a picture | 0170 | Built, ruled |
| 4, the constructor | 0171, 0173, 0174 | Built, ruled |
| 5, a trial | 0175 | Built, ruled; the three sizes accepted by the designer on the times of one trial |
| 6, the estimate | 0176 | Built, ruled |
| 7, the window | 0177 | Built, ruled |
| 8, courses, the tool and the table | 0178, 0181; 0204 after | Built, ruled; rule 14 of 0204 (the rest of today played inside a dry run) not built and not a condition |
| 9, the proof | none | Not run. No setup record, no report |
| 10, the enemy's approaches | none | Not planned. After the proof |

Every record of steps 1 to 8 has its "The architect's rulings", made on Nov 19 to 21, 2026. The lines of `docs/open-work.md` that say a record is "waiting on the architect's reading" were written with each build and not struck when the ruling was made; no reading of an "As built" is outstanding in this stage. The standing lines of decisions 0176, 0177 and 0178 said the same and are corrected today.

### What is owed, and whether the proof waits for it

| Owed | From | Does the proof wait for it |
| --- | --- | --- |
| Whether men ashore at a place the order does not name are the order's to feed; the cause of the Japanese hunger read on more seeds | 0224, the reading of Nov 23, 5 | Yes: condition 3. Its own record, next |
| Whether a place can be kept fed over weeks | 0169, rulings, 2 | No. It is what a trial of the place shows, and condition 12 reports it |
| A case of a plan displaced by a take of another place | 0176, rulings, 3 | No condition reads it. Owed as a scenario with its hand calculation (0176, "The architect's reading, Nov 24, 2026") |
| Words of an estimate for a commander | 0176, rulings, 5 | Settled for a seat by 0178 and 0181. Condition 11 is read on the seat's answer, not on the crate's `Display` |
| An air group that comes due makes no date of a window | 0177, rulings, 2 | No |
| The times of a window and of a trial beside another at the full size | 0177, rulings, 8; 0178, 8; 0181, 10 | Yes: condition 10 is "the times the designer accepts", and he has accepted one trial's times only. A question for the designer, below |
| A test of each of four rules that read the log of earlier days, across a picture | 0170, rulings, 7; 0171, 3 | No condition reads them. They stay owed |
| A dry run in the test that a seat's answers do not read the other seat's queue | 0182 | Not this proof's. It is owed before a game of two agents is called fair |
| An enemy's land air groups in a picture | 0174, "What it leaves for later" | No, by that record: "the first thing to mend after the trial is proved". Condition 12's report will show a landing tried with no bomber from Rabaul in it, and says so |
| A staff that reckons a force forward from its own orders when word of it is stale | 0136, open item 1 | No. It uses the stage's means and is no part of the estimate. Its own record, after the proof |
| The 39 days with the ships of the record, before the proof | 0165, answer 5 | A question for the designer, below |

### The conditions, one by one

1. **The hand calculations** are on main (scenarios 85, 86, 90 to 97, 100 and 118). The setup lists them by test.
2. **Nothing changed where it is unused.** Held by each build's baselines. The setup names the files compared.
3. **The mends hold where 0159 missed.** Not judged here. A measurement of Nov 23 on 100 seeds, with the table's ground figures, has the Japanese side losing 1,011 men a seed to hunger (0224, the reading, 4); that file is not the proof's and the figure is a lead. It is the reason the fallen-back men are planned before the proof. The condition's reading is decision 0159's, unchanged, and its mark is not moved.
4. **No peek.** Tests exist on the built cases. The 39 days at three days is to be made by the setup.
5. **The same twice, and nothing changed.** Tests exist (0175, 0178).
6. **Sampling.** Scenario 95.
7. **Spreads hold.** No built case has its band yet. The setup states the cases, the 200 games each, the band for the count of worlds met and the share that makes an estimate too wide, each with its hand calculation, before a run.
8. **A wrong picture.** The three cases are to be stated in the setup with the direction worked by hand.
9. **Preparation matters.** Read today, before any run: it is two trials of one number, so the same worlds, and not a window (0177, "The architect's reading, Nov 24, 2026"). "Fit" is the glossary's fit men, of the units the take landed, on the evening of the landing day, from the side's own view of each world. "At least 9 worlds in 10" at the standard size is at least 15 of 16. Whether the built staff can hold the landing day the same between an early and a late start needs the code read; it is commissioned below.
10. **Within its budget.** One trial is measured and accepted. The window and the comparison are measured and not accepted.
11. **Legible.** Six of six, on the seat's answer.
12. **Reported.** As written.

### The 39 days and decision 0165

Decision 0165 was to give the 39 days the ships of the record before this proof, and the designer confirmed that order (its answer 5). It has not happened, and part of it cannot. The standing is written in that record today ("Where it stands, Nov 24, 2026"). In short: the American ships cannot be derived and stay placeholders by answer 2; the Japanese destroyers are derived and no scenario reads them, because a rule for a class's troop figure across its runs, a change to the tool and rulings on five disputed runs are all unwritten.

The architect's recommendation, which reverses the designer's answer 5 and so is his to give: run the proof on the 39 days as they stand, on a file named in the setup and not changed until the report is written; give the Japanese runs their destroyers after it; and then read conditions 3 and 12 again on the new file and report them beside the first, with no second judgement. The reason is that condition 3 is read against decision 0159's proof, and a change of the ships that carry the men and the supply, made in the same stretch as a change to who is fed, would leave nobody able to say which of the two moved the hunger.

### To commission

1. **For condition 9,** a developer reads `sim-staff`'s take and reports, with the lines: on what a take with leave to launch sails (the day its men are to hand, or a level of loading and rehearsal); whether a latest day or "not before" in the order holds the day ashore the same when preparation began later; and what a unit under an order with "not before day D" does in the days before D (whether its force is in port with the order to prepare, so that 0095 counts the days). No change. The architect then writes the case and its hand calculation.
2. **For the setup,** a developer reports which staffed files of the 39 days name a trial's figures and a file of sides (`data/scenarios/staff/07_full_slice.toml`, `data/scenarios/stage-3-proof/07a_slice.toml`), and lists every change to those files and to the doctrine labels since decision 0159's proof of Nov 19, 2026, by commit and decision.
3. **For condition 3,** the measurement decision 0224's reading asks for is commissioned with the record of the fallen-back men, not here.

### Questions for the designer

1. **The proof before the ships of the record, or after?** Answer 5 of decision 0165 says after. The architect now recommends before, for the reason above; the American ships stay placeholders in either order.
2. **The times at the full size.** A window of three dates takes about 67 s on the 39 days and 804 MB; a trial beside another 27 to 36 s; one trial 16 to 23 s by the load on the machine. The standard size is 12.8 s for a window and about 5 s compared. Are these accepted for condition 10, or is the full size to be made cheaper first (worlds without breakdowns, worlds side by side: owed from 0175)? The architect recommends accepting them for the proof and taking the cheaper worlds after it: they move no result.

## Answered by the designer on Nov 24, 2026

Both questions of "Where the stage stands", each as the architect recommended.

1. **The proof is run before the ships of the record.** It is run on the 39 days as they stand, on files named in the setup and not changed until the report is written. This reverses answer 5 of decision 0165, and that record says so. After the proof the Japanese runs are given their destroyers (decision 0165, steps 5 and 6), and conditions 3 and 12 are then read again on the new file and reported beside the first reading, with no second judgement.
2. **The times at the full size are accepted for condition 10:** a window of three dates at about 67 s and 804 MB, and a trial beside another at 27 to 36 s, on the named machine, with one trial's times as accepted on Nov 19, 2026. Worlds that record no breakdowns and worlds played side by side stay owed from decision 0175 and are taken after the proof.

## Condition 9, the case (Nov 24, 2026)

Written before any run. The code was read for it (a developer's report, checked by the architect in `crates/sim-staff/src/ready.rs` and the rulebook's chapter 7). The statements and the full hand calculation go into the test file's doc comment, written by the architect before the case is built; what follows fixes the case and its arithmetic.

**What the code does.** A landing is ready when its force's loading and each unit's rehearsal are at the level its risk asks: 100 at preserve and at balanced, 50 at press (`thresholds`). A take with leave to launch sails on the first morning it is ready. So at balanced a landing prepared late is not less ready on the same day: it is as ready, later, and the staff hands in "late" once. An order with "not before day D" keeps the lift in port, and the orders to prepare and to rehearse stand through the wait, so loading and rehearsal go on rising to 100.

**So the condition is put at press, with a day.** The same order, "take the place, at press, not before day D", with leave to prepare and leave to launch, is handed in on day 1 in one course and on day P in the other, D being the first morning the later course is ready. Both lifts sail on the morning of day D and are ashore on the same day. Only the day preparation began differs, which is what the condition asks. The two courses are two trials of one number, at the standard size, 16 worlds for 14 days.

**The arithmetic,** on the case's own placeholder figures, `preparation_per_day` 10 and `unprepared_landing_fatigue` 40, one fighting unit of 1,000 men at a source one day's embarking and passage from the place, P = 4 and D = 9:

- Begun on day 1: eight days in port with the order, days 1 to 8. Loading and rehearsal 8 x 10 = 80.
- Begun on day 4: five days, days 4 to 8. Loading and rehearsal 5 x 10 = 50, which is the level press asks, on the morning of day 9 and not before.
- Landing fatigue is `unprepared_landing_fatigue` times 100 less the rehearsal, over 100: 40 x 20 / 100 = 8 for the earlier, 40 x 50 / 100 = 20 for the later.
- Fit men, of 1,000 landed and before any fight: 1,000 x 92 / 100 = 920 for the earlier, 1,000 x 80 / 100 = 800 for the later. The higher loading also lands the earlier's men faster (`unprepared_landing_share`), which works the same way.

**What is asserted.** In each world, on the evening of the day the men are ashore, the fit men of the unit by the side's own view of that world, the course begun on day 1 against the course begun on day 4: more in at least 15 of the 16 worlds. A world in which either course's lift did not land that day is counted as not more, and reported. The day counts above (whether the day an order is taken in counts as a day in port) are checked against scenario 27's hand calculation when the doc comment is written; if a count is one out, P or D moves by a day and the condition does not.

**What it does not show.** That a landing at balanced gains by an earlier start: it gains a date, not fitness, and the window's "decide later" already shows a date. The report says so.

## The measurement of the men unfed, before the record of the fallen-back men (Nov 24, 2026)

Decision 0224's reading of Nov 23 (5) left a lead from one game: the Japanese men lost to hunger are those fallen back to (2,-5), a hex no order names. This says what is to be counted on more seeds before the record is written. It is the umpire's reading, from exports. It prints counts and never a verdict. It is not the proof's run and judges no condition; the proof's report will say that seeds 1 to 100 of its arrangement were read for a cause beforehand.

**The games.** Staff against staff, 39 days, seeds 1 to 100, one `sim-cli export` a seed, release build, from main as it stands (state v59, rules label 32, doctrine `supply-59`; the commit is written in the result). Two arrangements, reported apart:

- **P:** `data/scenarios/stage-3-proof/07a_slice.toml` as it is on main. It is the file condition 3 will be read on.
- **T:** the same file with the table's ground figures, as decision 0224's reading names it (1), so that the result can be set beside the 1,011 men a seed of Nov 23. If T's Japanese men lost unfed do not come to that figure's neighbourhood, say so before anything else.

**What is counted,** for each seed and each side, with `scripts/0159-proof/hunger.py`'s reading of "landed" and "lost" kept word for word (L, the men landed at a place not the side's; U, the men of every `men_lost_unfed` of those units from their first counted landing):

1. **A unit's place.** The hex of its latest counted `men_landed`.
2. **A unit's latest move** before a given event: the last, in event order, of its `unit_fell_back`, `spent_unit_fell_back` and `march_arrived` events, or none.
3. **A fallen-back man unfed** is a man of a `men_lost_unfed` event of a counted unit in a hex H, where the unit's latest move before that event is a `unit_fell_back` or a `spent_unit_fell_back` whose `to` is H. Their sum is F. F is split by which of the two events it was, and by whether H is the unit's place (it fell back to where it landed) or another hex.
4. **The rest of U,** split: lost at the unit's place with no move before it (A); lost in a hex reached by `march_arrived` (M); anything else (X, which should be nothing: each such event is listed).
5. **For each hex H that has any of F,** summed over the seeds and listed by seed: the men who fell back there (the events' `men`) and the days; the first and last day of a loss there; whether a base of the side stood in H at the first loss, of the start or formed (`base_formed`, `march_base_formed`); the tons of supply the side unloaded in H from the day of the first fall-back to the end (`cargo_unloaded`); and the last `men_fed` of each unit there before its first loss, with its day.
6. **Whether the staff looked to H.** For each such H and seed: whether, from the day of the first fall-back, any order of the side's staff names H (a route's end, an unload, a march to or from it), counted by kind. Zero is the lead's claim.

**What is printed,** for each arrangement and side: the medians over the seeds on which the side landed men of U/L, F/L, A/L and M/L, and of (U less F)/L, which is what the hunger would be were no fallen-back man lost; the count of seeds in which F is more than half of U; the sum of F by hex; and the list of item 6. The per-seed rows go to a file beside it.

**What the record will rest on.** If (U less F)/L is at or under one in twenty and F is most of U, the cause is the lead's and the record settles whose the fallen-back men are to feed. If A is large, the men starve where they landed and the cause is still the supply of the place, which is a different record. If neither, the result comes back to the architect before anything is planned.

## The men unfed, as measured (Nov 24, 2026)

Made as the section above says. Facts only: no condition is judged and no figure moved here.

**The games.** `sim-cli` built in release from commit `3b5a7b5b` (the tree of `origin/main` at `7b7ea9c0`, which differs from it by no file; state v59, doctrine `supply-59`). Seeds 1 to 100, 39 days, one export a seed, 200 exports:

```
CARGO_TARGET_DIR=/tmp/unfed/target cargo build --release -p sim-cli
sim-cli export data/scenarios/stage-3-proof/07a_slice.toml     --seed N --out /tmp/unfed/P/seedN.json     (P)
sim-cli export data/scenarios/tmp-inside/07a_slice_T.toml      --seed N --out /tmp/unfed/T/seedN.json     (T)
```

T is P with one line changed, `ground = "../../reference/ground_hunger.toml"` for `ground = "../../reference/ground.toml"` (the table's ground figures, 0224's reading (1)); `data/scenarios/tmp-inside/07a_slice_T.toml` is a temporary file and is not committed; no committed scenario was changed.

**The reader** is outside the repository: `/tmp/unfed/unfed.py` (it imports `scripts/0159-proof/hunger.py`'s `read_seed` unchanged for L, U and the counted units, and stage 3's `common.py`) and `/tmp/unfed/hextable.py` (the per-hex table). `python3 -I unfed.py DIR [--all] --rows OUT.jsonl`; per-seed rows are in `/tmp/unfed/rows{P,T}.jsonl` (the base reading) and `/tmp/unfed/rowsall{P,T}.jsonl` (the extension below), and the printed output in `/tmp/unfed/out*.txt`. Python's 3.9; `-I`. The scripts, rows and exports are in `/tmp`, which a reboot may clear; the two commands above make the exports again, and the games are deterministic.

**What each count is.** L and U are `hunger.py`'s (L: men of `men_landed` of a unit of the side in a hex whose base of the start was not the side's; U: the `lost` of every `men_lost_unfed` of those units from the pulse of their first counted landing). A unit's latest move before an event is the last of its `unit_fell_back`, `spent_unit_fell_back` and `march_arrived` events in event order. **F:** a `men_lost_unfed` of a counted unit in H whose latest move is a `unit_fell_back` or `spent_unit_fell_back` whose `to` is H. **A:** no move before it and H is the unit's place (the hex of its latest counted `men_landed`). **M:** the latest move is a `march_arrived` whose `to` is H. **X:** anything else. Each X is listed in the rows file.

### 1. The first fact, before anything else

**T's Japanese men lost unfed come to the neighbourhood of the Nov 23 figure, but on the base reading none of them is counted.** All units of the Japanese side lose a median 1,009 men a seed in T (979 to 1,035; 101,129 over 100 seeds; the Nov 23 record has 1,009 to 1,011). In P the median is 1,860 (1,553 to 3,017). On the base reading, U is **0** in all 100 seeds of T for both sides: every one of those men belongs to units 11, 12 and 13 (the Japanese combat groups ashore at (2,-4) at the start; each is a land unit with no `men_landed` event in the ten seeds checked for it, 1, 11 and so on to 91, in both arrangements), which `hunger.py`'s reading never counts as landed. Landed men on the Japanese side, L, are a median 6,038 a seed (T: 599,227 over 100 seeds; P: 600,501) and none of them is lost unfed in T.

### 2. The base reading (counted units only), medians over the 100 seeds

| | Side | Seeds with L | median L | median U | U/L | F/L | A/L | M/L | X/L | (U less F)/L | seeds F over half of U |
|---|---|---|---|---|---|---|---|---|---|---|---|
| P | Japanese | 100 | 6,038 | 300.5 | 0.0505 | 0.0000 | 0.0505 | 0.0000 | 0.0000 | 0.0505 | 0 (U above 0 in 100 seeds) |
| P | Allied | 100 | 16,975 | 468 | 0.0276 | 0.0000 | 0.0276 | 0.0000 | 0.0000 | 0.0276 | 0 (U above 0 in 59 seeds) |
| T | Japanese | 100 | 6,038 | 0 | 0.0000 | 0.0000 | 0.0000 | 0.0000 | 0.0000 | 0.0000 | 0 (U is 0 in every seed) |
| T | Allied | 100 | 16,975 | 0 | 0.0000 | 0.0000 | 0.0000 | 0.0000 | 0.0000 | 0.0000 | 0 (U is 0 in every seed) |

Sums over the seeds: P Japanese U 35,764, all A; P Allied U 96,136, all A. F is 0 and M is 0 on every row. The sum of F by hex is empty. The Japanese counted men lost in P are lost where they landed, with no move before.

### 3. The same split over every unit of the side (an extension, not the section's definition)

The section's definition counts none of the units that the loss is in, so the same F, A, M and X are also printed over the `lost` of **every** `men_lost_unfed` of every unit of the side, a unit with no counted landing having the hex of its starting record as its place (`--all`). L is as above. Japanese side:

| | median U (all units) | U/L | F/L | A/L | M/L | X/L | (U less F)/L | sums over 100 seeds |
|---|---|---|---|---|---|---|---|---|
| P | 1,860 | 0.3083 | 0.0000 | 0.0505 | 0.2093 | 0.0468 | 0.3083 | U 190,017: A 35,764, M 126,076, X 28,177, F 0 |
| T | 1,009 | 0.1681 | 0.0000 | 0.0000 | 0.1681 | 0.0000 | 0.1681 | U 101,129: M 101,129, A 0, X 0, F 0 |

Allied side, all units: P U 100,420 (A 100,420; the 4,284 beyond the counted units are lost at (2,-4) by units ashore at the start), T U 0.

**F is 0 in every seed of both arrangements, whichever way it is counted, and seeds with F over half of U are 0 of 100 in each.** The reason is in the events: every `unit_fell_back` in these games is followed by a `march_day` series and a `march_arrived` to the same hex, so the latest move before any loss is a `march_arrived`, and the loss falls in M by the definition as written. Every one of the 126,076 (P) and 101,129 (T) M men is of a unit whose move before the `march_arrived` is a `unit_fell_back` to the same hex (`M_after_fell_back_to_same_hex` in the rows). Read with those men counted as fallen back (G, a count made here beside the definition): median G/L is 0.2093 in P and 0.1681 in T; (U less G)/L is 0.0985 in P and 0.0000 in T; seeds with G over half of U are 98 of 100 in P and 100 of 100 in T. In P the X men are 28,177 over the seeds, of units 11, 12 and 13, each of whose latest move was a `unit_fell_back` to (2,-5) and which lose men at (2,-4) with no later move event.

### 4. The hexes where fallen-back men are lost (units 11, 12 and 13, by the G reading above)

One hex, **(2,-5)**, in every seed of both arrangements (Japanese side; no Allied loss). The per-hex items of the section:

| Item | P | T |
|---|---|---|
| Seeds, men lost at (2,-5) by these units over 100 seeds | 100, 126,076 | 100, 101,129 |
| Men who fell back there (the events' `men`, summed over the seeds), day of the fall-back | 198,370, day 1 in every seed | 198,370, day 1 in every seed |
| First day of a loss there (all seeds); last day | 13; 39 | 26; 39 |
| A base of the side in the hex at the first loss | a base of the start in 100 seeds (base 6, "Placeholder Japanese stock west of the Lunga", 71 t at the start, `on_map = false`); none formed | the same, 100 seeds |
| Tons the side unloaded in the hex from day 1 to the end | 0 in every seed | 0 in every seed |
| Last `men_fed` at the hex before the first loss (median over seeds; the event is of a hex and a side, not of a unit, so it is the hex's and not each unit's) | day 13; 1,698 men, 616 fed, 1,082 unfed; from base 6 in 100 seeds | day 26; 1,980 men, 0 fed, 1,980 unfed; from base 6 in 100 seeds |
| Staff orders (route end or via, acting at, prepare a landing, unload by a force lying there, or a command taken naming the hex) naming (2,-5) from day 1 | none in any seed | none in any seed |

The per-unit item (the last `men_fed` of each unit) cannot be read from the events as written, since a `men_fed` event names a hex, a side and a base and no unit; the hex's last is given above. Of an order that "names" a hex: an unload order has no hex in its text, so it was read against the position of the force it was given to at the start of its day; a march to or from the hex that is no order is not an order of the staff and none appears in the orders or the commands taken.

### 5. What was not read

The Allied side has no F, M or X in either arrangement. No order of either side's staff names (2,-5) from day 1 in any of the 200 games. Units 11, 12 and 13 begin ashore, so the base reading carries none of the Japanese men lost unfed in T and 35,764 of the 190,017 lost by every Japanese unit in P (19 per cent). The hunger of the counted landed men (U/L 0.0505 in P, 0 in T) and the hunger of the men ashore at the start (M, 0.2093 in P and 0.1681 in T) are two counts.

## The architect's reading of the men unfed (Nov 24, 2026)

Read: the section above, the scenario's file for base 6 and units 11 to 13, and the rulebook's "Controlling an island" and "Holding a place". The developer's counts are taken as measured; the reader is outside the repository and is brought into it by decision 0229. No condition is judged here.

1. **The lead of decision 0224 was two things taken for one.** "The Japanese men lost to hunger are the same 1,011 a seed" and decision 0159's named miss are different men. The miss of 0159 was of men landed. The 1,011 are the garrison that begins ashore at Lunga: the naval guard, 240 men, and the two construction units, 900 each. Beaten on day 1, they fall back along the land link to (2,-5), where their side's base 6 holds 71 t, eat it and are then fed by no one.
2. **The men landed are now fed, to about the mark.** On the proof's own arrangement the median lost unfed of the Japanese men landed is 0.0505 on these 100 seeds, against 0.2537 in decision 0159's proof; the Allied side's is 0.0276. The mark is one in twenty, on 1,000 seeds. 0.0505 on 100 seeds says the two mends did what they were for; it does not say on which side of the mark the proof will fall, and nothing is changed to move it. On the table's ground figures no landed man of either side is lost.
3. **The definition of F was the architect's error.** A unit that falls back is set to march, so its latest move is always the march's arrival, and F as written could only be nothing. The developer's count G, a fall-back followed by the arrival of its own march, is what was meant, and is the reading used here: G is over half of all the Japanese men lost unfed in 98 seeds of 100 on the proof's figures and in all 100 on the table's.
4. **(2,-5) is not "a hex no order names" for want of a base.** It holds a base of the side from the start. No order names it because the island is a list of places, Guadalcanal's has one, the airfield at Lunga, and the staff forms supply only for a hold of a named place or for the side's men in a named place's hex (decision 0159, A). The answer to the question decision 0224 left, whether such men are the order's to feed, is yes, and decision 0229 is the rule.
5. **Condition 3's reading does not change.** It is written of "the men a side lands at a place it has not taken", by decision 0159's reading, and a unit that begins ashore is not landed. Counting the garrison in it now would be a change to a condition after its figures were seen. The garrison's hunger is a second count, made by the same reader, and whether the proof judges it or reports it is a question for the designer in decision 0229.
6. **One thing not explained,** on the proof's figures alone: 28,177 men over the seeds are lost by units 11 to 13 at (2,-4) after they fell back to (2,-5), with no move event between. A developer is to say from one seed's events how the units come to be at (2,-4) again (an attack ordered across the link is the likely way). It changes nothing in decision 0229 unless the men are there by the staff's order, in which case it comes back to the architect.
7. **The order of work stands:** decision 0229 built, then the proof's setup. The 39 days change for the Japanese side by it; that is before the proof and is what it is for.

## Condition 13, added by the designer on Nov 24, 2026

Asked by decision 0229 whether the hunger of men who begin ashore is judged or reported, the designer answered: judged. It is stated here before decision 0229 is built and before any run of the proof.

13. **Men who begin ashore are fed.** On the 39 days staff against staff, 1,000 seeds: of the men of a side who begin ashore on an island that side is ordered to control, the median lost to hunger is no more than one in twenty, each side by itself.

Its reading, fixed with it: the men are those of land units with men ashore at the start in a hex of the island's land (decision 0229, rule 1) of an island their side states "control an island" of on day 1; lost to hunger is the `lost` of every `men_lost_unfed` of those units to the end of the game, wherever they are; the seed's value is that over the units' men at the start; the median is over the seeds, each side apart. Condition 3 is unchanged and is of men landed; a unit is in one count or the other and never both.

**A note before the build, and a question.** One seed read on Nov 24, 2026 (decision 0229, amended 9) explains the men lost "at (2,-4)" in the architect's reading above (6): they are the garrison on its three days' march after it is beaten, with no base where it walks, and no rule of a staff can feed them. On the proof's figures that is about 283 of 1,984 men a game, one in seven, so this condition as stated is expected to miss on the Japanese side. Whether it is read as stated, or of the men lost where a base of their side stands with the march's losses reported beside it, is the designer's, and is asked in decision 0229. Until he has said, the reading above is the one in force.

