# 0161: The trial: the staff's war game of a plan, and the estimate a commander decides on. The plan for stage 4 of step 3

Status: a plan by the architect, written on Nov 19, 2026 and confirmed by the designer the same day with the seven answers the architect recommended. Nothing is built yet. It settles the contracts `docs/step3-spec.md` leaves to this record ("The trial") and takes in the two mends decision 0159 left for it.

Summary: The plan for stage 4 of step 3. A commander who has stated an intent can ask the staff to try the plan before it goes: the staff builds private worlds from what the side knows and believes, never from the live enemy, plays the plan in each with both sides on doctrine, and hands in an estimate: what the plan needs, when it could go, what it may cost, how often it came off, and what was assumed about what is not seen. The same course is tried at two or three dates, so the estimate says what waiting buys and what the enemy may do with the days. Nine steps, the first two the mends of decision 0159; a world built from a picture is the one new means in the engine; a new crate plays the trials. With the conditions of its proof and seven questions for the designer.

Standing: Confirmed by the designer, Nov 19, 2026, with his seven answers, each as recommended. Step 1 is built (decisions 0162 and 0163); step 2 is next. A tenth step, the enemy's approaches, was added by the designer the same day

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

