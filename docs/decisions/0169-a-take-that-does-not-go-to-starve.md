# 0169: A take that does not go to starve: the second mend of stage 4

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 2 of its ten, "a take that does not go to starve"), which the designer confirmed. The hand calculation is written. Built on Nov 19, 2026 (As built, below).

Summary: Decision 0159 made a take wait, or ask, when it could not bring the men it wanted. Its proof showed every Japanese lift of the 39 days sailing far short all the same: the rule is not worked for men already aboard, nor for a later lift, and the check that a place can be fed stops nothing at balanced. Now, at balanced, a lift that leaves the operation short with nothing in reach to make up the number goes only where its men can be fed; where they cannot, the take does not lift, hands in "cannot bring enough" and waits

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161. Built on Nov 19, 2026

## Why

Decision 0159's proof judged that no lift sails short when waiting would have brought enough, and passed it: 0 of 12,000. Beside it the report says that all 10,000 Japanese lifts sail far short with nothing in reach to make up the number, which the condition did not judge. The men they land are the men the proof then counts as lost to hunger.

What lets them go, read from the code (`take.rs`) and tried on scenario 86 with an estimate of 400 men added:

- **Men already aboard are let through.** The wait of decision 0159, rule B, is not worked where any unit chosen is aboard (`wait_for_men`: "A lift as it stands, or men already aboard, are as they were"). Every Japanese fighting unit of the 39 days starts aboard its own transport force, so the rule is never worked for that side. In the trial the staff wants 800 men, has 600 aboard, and says "at balanced the staff goes with what it has".
- **A later lift and a second lift are sent with no count of this kind.** A later lift as it stands is held back only while the first is on its way or once the fit men ashore are the number wanted (decision 0152, 5).
- **The check that the place can be held stops nothing at balanced.** Decision 0144's check (items 10 to 17) is worked for a lift as it stands too, and at preserve a shortfall stops the take. At balanced the staff writes the shortfall down and adds "the staff goes all the same".
- **What is aboard is not weighed against the days.** In the trial the lift has 30 t aboard against the 45 t the check sizes; it lies away from its source, loads nothing, and goes "short by 15 t". Nothing asks whether what is aboard lasts until the first run arrives.

The risk is not the cause (the Japanese side is at balanced, the default) and neither is a latest day (none is set).

Decision 0161 states the mend: "Where the men a take can bring within its horizon are too few by its own count at its risk, and its own supply cannot feed them where they would wait, it does not lift." The two conditions stand together. A side may still build up ashore by lifts, each too few by itself, as both sides did on Guadalcanal: what it may not do is put men ashore that it can see will go hungry before anything reaches them.

## The rule

1. **Where it is worked.** At balanced, on a morning the staff would form or send a lift of men under a "take a place": the first lift where any unit chosen is aboard (a lift as it stands among them), a later lift as it stands, and a second lift. A first lift whose men are all ashore at its source is decision 0159's, rule B, unchanged.
2. **Short.** The lift is short where the fit men of the operation ashore at the place that can attack, and the fighting men the lift carries, are together fewer than the men wanted: the side's estimate of the enemy's men at the place times the figure of the risk (decision 0142, 8). With no estimate held there is no count, nothing is short, and the lift goes as before.
3. **Nothing in reach.** The men in reach are those of item 2 and the fighting men aboard every other transport force the take may send as a lift as it stands, counted where the force may first be used on or before the take's last day: its latest day where one is set, else today and the 7 days after it (decision 0159's horizon). Where the men in reach are the number wanted or more, the lift goes as before: the wait ashore has an end the staff can name.
4. **Fed.** Where the lift is short and the men in reach are too few, the staff asks whether the men the lift lands can be fed. They can where both hold:
   - **The place can be held.** The take's place has a hold with a source, and the check of decision 0144 (items 10 to 15), worked for this lift, finds nothing short: the source holds the supply that lands with the men and the first run, and the ships have the room.
   - **What is aboard lasts until supply follows.** The supply the lift will have aboard when it sails, what is aboard and what it will load at its source, is at least the daily need of the men chosen times the days until supply follows: the check's own two figures, without its margin.

   With no hold of the place, or a hold with no source, nothing follows the men and they cannot be fed.
5. **Not fed: the take does not lift.** It forms no landing for that lift and gives its force no order. It hands in the notice "cannot bring enough" once, as decision 0159, item 7, does, and waits for the commander. Its reason gives the men wanted, the men the lift gives, the men in reach and the day reckoned to, and what of item 4 failed with its figures. On each later morning it reads all of it again, and goes on the first on which the lift is not short, enough men are in reach, or the men can be fed.
6. **Fed: the lift goes,** as before, and its reason says it goes short and why it may: the men can be fed where they wait.
7. **At press the commander has said to go, and it goes.** At preserve the take is as it was: it forms nothing where it is short and lands no men it cannot keep fed.
8. **What the commander may do** is what decision 0159 gave him: set the risk to press, set a limit, or leave it waiting. He may also mend what failed: put supply at the source, or name a source.
9. **The men not lifted stay where they are,** aboard their ships or ashore at their base. Nothing else is ordered of them by this rule.

## What it leaves for later

- **A first lift from a base that could be fed.** At balanced decision 0159 has it ask and not go, fed or not. Men at a base wait at no cost, and the commander is asked. Whether such a lift should also go where it can be fed is left until the trial shows what it costs.
- **Whether the runs can keep the men fed over weeks.** The check asks for the first run's stock at the source and the room for it. A night's load (decision 0162), the number of ships in the pool and what the enemy sinks on the way are not weighed. The trial of decision 0161 is where a course is played forward.
- **A lift cut to what can be fed:** landing fewer men than are aboard.

## Rule card

- **Reads:** the side's estimate at the place; the risk; the men of the operation ashore and fit; the units aboard the transport forces the take may use and the day each force may first be used; the hold of the place and its source; the check of decision 0144; the supply aboard the lift.
- **Writes:** nothing new. Where it stops a lift, the take forms no intent and gives no order, and hands in a notice that exists.
- **Reuses:** the count and the notice of decision 0159, rule B, with its horizon; the check that a place can be held of decision 0144; the lift as it stands of decision 0152. No new notice, interrupt, state, event or draw.

## What it costs

- **The doctrine's version rises.** No state, saved game or interface changes: the notice and the interrupt exist.
- **Earlier scenarios.** A game with no take, or whose takes hold no estimate, or whose lifts are not short, plays as before. Scenario 77's property "A later lift of fighting men" sends a lift of 300 men while 2,000 of the enemy are ashore: it is worked by this rule and may change. Its figures go to the architect, who restates it; the developer does not.
- **The 39 days** change wherever a Japanese lift is short and cannot be fed. Nothing there is judged by this record: the proof of decision 0161 reads it (condition 3).
- **The rulebook:** chapter 8's passages on a take that waits, on a lift as it stands and on later lifts; the glossary's "cannot bring enough".

## The hand calculation

It is in `tests/scenario_90_a_take_that_does_not_go_to_starve.rs`, written before the code. It is scenario 86 with an estimate of 400 of the enemy's men at the place and 10 t of supply aboard where 30 t was: 800 men wanted, 600 aboard, nothing else in reach, and 10 t against the 20 t the men eat before a run can reach them. The take does not lift. The cases beside it: 30 t aboard (it goes), the source too poor, no estimate, an estimate it is not short of, press, and a second force in reach and out of it.

## As built

Built on Nov 19, 2026, on the branch `take-not-to-starve-build`.

- **The rule** (`starve_check` in `crates/sim-staff/src/take.rs`). It returns nothing, and the lift goes as before, where the risk read up the take is not balanced, no estimate is held at the place, the fit men ashore and the lift's fighting men are the men wanted or more, or the men in reach are. Otherwise it gives the count and whether the men can be fed. Where they cannot, the take hands in a new arm of decision 0159's `Waiting`, `Starve`, through the same `hand_in_wait`: the notice `cannot_bring_enough` once for the plan (read by `noticed`, as decision 0159 does, and not a second time where two later lifts are stopped on one morning), else a reason to give no order. The plan order keeps it as decision 0159's wait with no day (`MenWait` with no day): the men wanted and the men the lift gives.
- **Item 4.** The check is decision 0144's `hold_check`, which now also gives its daily need and its days until supply follows; the supply aboard is the level the staff's `supply` sets for the lift (what is aboard, and what it loads where it lies at its source), and for a later lift as it stands what is aboard, as decision 0152 sends it.
- **Doctrine** `supply-42`. No state, event, notice, interrupt, draw, order, command, saved key, rules label or agent interface version changes.
- **Scenario 90** (`data/scenarios/90_a_take_that_does_not_go_to_starve.toml`, `tests/scenario_90_a_take_that_does_not_go_to_starve.rs`): every statement of the hand calculation is asserted and agrees; no expected number was changed.

Readings the record did not make, for the architect:

1. **Where the first lift is worked.** In Prepare (`prepare_place`), before the task "ready a landing" is formed, beside decision 0159's wait: on the morning the take would form its landing, and on each later morning while no landing is formed. It is not worked at the draft (`draft_take_waits`), because under an island the take is drafted, and made ready on leave to prepare, before the island forms the hold of the place and names its source that same morning: worked there the first morning would always find no hold. So a take that does not lift is made ready (preparing) on leave to prepare and stays preparing, where decision 0159's wait keeps a draft a draft. A landing already formed is not stopped at its release: the check of decision 0144 at balanced still says "the staff goes all the same" there.
2. **Where the later lifts are worked.** In `send_later_lifts`, for each lift as it stands that decision 0152 would send that morning, one at a time, before the forces limit is set. A lift that does not go is left out of the limit and given nothing; the others go. The forces a take sets aside for its later lifts before a lower take chooses (decision 0152, 5b) are set aside as before, whether or not the rule then sends them.
3. **Where the second lift is worked.** On each morning `ashore` would give the second lift an order: where its units are aboard or at the source with the force there, before each step of `move_step`; and where it takes units left behind, before they are added to the limit. Not worked once the force is on its way.
4. **The men in reach.** The forces the take may send as a later lift as it stands are read by decision 0152's `later_lift_plan` with the day the choice reads set to the take's last day, so a force or unit is counted where it may be used by then; one under way on an order, the lift of another take, sent by another intent that morning, out of the limit named, or with no base to come back to, is not counted, as it would not be sent. Its men are the fighting men aboard. The lifts sent the same morning count in each other's reach.
5. **Whose men the check feeds.** The check and the supply aboard are worked for this lift's force and every man aboard it (decision 0152, 4), and for a second lift for the units it takes and every man aboard; not for the men already ashore at the place. So a small later lift with a week's food aboard passes, however many are hungry ashore.
6. **The fighting men of the lift** are the units chosen for a first lift, the fighting units aboard a later lift as it stands, and the fighting units of a second lift; working units land and are fed, but are not counted against the men wanted.
7. **The reason.** Not fed: what the staff read before the count (the choice, or the judgement of the morning and the lift), then "The staff wants N men; the lift by F gives M fighting men: M, short by D. The men in reach by day L, the 7 days the staff looks ahead: R (...), fewer than the N wanted.", then the check in its own words and "What the lift has aboard when it sails: A against the daily need of X for the U days until supply follows, X x U = E, short by S: the men cannot be fed.", or what failed. Fed: the same count and feeding appended to the reason of the landing formed, to the head of a later lift's passage, or to a second lift's supply words, ending "So it goes short: its men can be fed where they wait."

What changed in the games already built:

- **Scenario 77,** the property "A later lift of fighting men": it passes unchanged. On day 4 the staff wants 3,098 men (an estimate of 1,549 of day 3); the lift by task force 2 gives 300 fighting men and the 600 fit men ashore make 900; nothing else is in reach by day 11; the check for task force 2's 300 men (1.5 t a day, 4 days until supply follows, 13.5 t + 22.5 t against base 1's 920 t, room 3,000 t) can hold, and its 20 t aboard is at least the 6 t they eat. So it goes, as before, and its reason says so. Days 1 to 3 hold no lift the rule works: on day 1 no estimate is held, and on days 2 and 3 no lift is formed or sent (task force 2 may be used from day 4).
- **Scenario 68, the property "A hold is honoured", changes, and is marked `#[ignore]` for the architect to restate; its expected values are not changed.** With unit 2 held from day 7, the staff wants 1,200 men (an estimate of 600); on day 8 the second lift by task force 1, lying empty at base 1, would carry unit 3's 600 fighting men, and the 300 fit men of unit 1 ashore make 900, short by 300; nothing else is in reach by day 15; no hold of B with a source is in force, so the men cannot be fed. The staff gives no order (the test wanted, on days 8, 9 and 10, the load of 45 t, unit 3 aboard, and the route and the unload), hands in `cannot_bring_enough` on day 8, and says the same each morning after. Its main case is not short (unit 2 is not held: 600 + 600 = 1,200) and passes.
- **Scenarios 06d1 and 06d2** leave scenario 90 out of the files whose hashes they hold to before their stage, as they do the other staffed scenarios.
- **Every other scenario test** passes unchanged; no run baseline changes but scenario 90's, which is new. The recorded games change in their doctrine version alone; no transcript, hash, pinned hash or batch fixture changes.
- **The three arrangements of stage 3's proof,** run once each on their own seed, 1942. Arrangements (b) and (c): the staff hands in what it handed in before. Arrangement (a): every daily hash is as before, and the Japanese staff's orders are the same; only the words change. Its first lift on day 10 (task force 30, 200 men, against 11,744 wanted) and its later lifts on days 12, 23, 27, 29, 30, 31 and 36 are each short, with too few in reach, and each is found fed, so each goes as before, with the count and the feeding in its reason. Each lift carries 0.035 t a man, 7 days of food, against the 6 days until supply follows from Rabaul, and Rabaul's stock passes the check for the lift's own men. Nothing in them is judged here.

