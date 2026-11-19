# 0169: A take that does not go to starve: the second mend of stage 4

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 2 of its ten, "a take that does not go to starve"), which the designer confirmed. The hand calculation is written. To be built.

Summary: Decision 0159 made a take wait, or ask, when it could not bring the men it wanted. Its proof showed every Japanese lift of the 39 days sailing far short all the same: the rule is not worked for men already aboard, nor for a later lift, and the check that a place can be fed stops nothing at balanced. Now, at balanced, a lift that leaves the operation short with nothing in reach to make up the number goes only where its men can be fed; where they cannot, the take does not lift, hands in "cannot bring enough" and waits

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161. The hand calculation is written; the build is next

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
