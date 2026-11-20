# 0193: Men ashore with no more coming

Status: a plan by the architect, written on Nov 20, 2026, on the designer's word ("plan and build 1 and 4 next"). To be read by a reviewer who has not seen it written, and amended on that reading, before any of it is built.

Summary: Men landed to take a place attack only when they are as many as the risk asks for against the enemy they count: twice his number at balanced. When they are fewer, the staff waits for a second lift. When there is nothing more to send, it waits for ever. This gives the staff a judgement for that case: with no more coming, a balanced staff attacks if its fit men are at least the enemy's, a pressing staff attacks whatever the count, and a careful one waits and tells the commander.

Standing: Planned on Nov 20, 2026; waits for its review.

## Why

Scenario 105 (decision 0188). 5,386 men landed at Port Moresby. They counted 3,444 defenders. At balanced the staff wanted 6,888, had 5,116 fit, and waited. No second lift existed. They would have waited as long as the game ran.

What is so today (chapter 8, Taking a place, Ashore and A second lift):

- **The men wanted** are the estimate of the enemy's men at the place times 3 at preserve the force, 2 at balanced, 1 at press.
- **The fit men** of a unit are its men ashore times (100 less its fatigue) over 100, cut to a whole man. A unit can attack if its fatigue is at or below the risk's mark.
- **Attack** where no estimate is held or the fit men of the units that can attack are at least the men wanted. **Otherwise wait.**
- **A second lift** is sent while units are left behind at the source and the transports lie there empty. "Where the men are short and none is left, the reason says so and the staff waits."

## The rules

1. **Nothing more is coming** to a place when, on the morning of the judgement: no fighting unit is left behind for a second lift; no transport force of this take has fighting men aboard, whether under way, turned back (decision 0191) or waiting; and no unit chosen is waiting for its men. It is worked from the side's own forces and plan, and the reason lists what was looked for.
2. **With nothing more coming, the men wanted fall,** by the risk read up the tree:

   | | Preserve the force | Balanced | Press |
   | --- | --- | --- | --- |
   | Men wanted while more is coming (today) | 3 times the estimate | 2 times | 1 times |
   | Men wanted with nothing more coming | 3 times: it waits | 1 times | None: it attacks |

3. **Balanced:** the units that can attack are set to attack where their fit men are at least the estimate. Where they are fewer, it waits.
4. **Press:** every unit that can attack is set to attack, whatever the estimate.
5. **Preserve the force:** it waits, as today.
6. **Where it waits with nothing more coming,** the staff hands in the notice `cannot_take`, each morning that holds, which the standing interrupts may stop the clock on. Its reason gives the estimate and its day, the fit men, the men wanted and that nothing more is coming. A commander may then set press, send more, or withdraw the intent.
7. **Everything else of the judgement is as today:** which units count, which can attack, the fatigue marks, the reason's figures, and what happens with no estimate held.
8. **The figures are the staff's and are placeholders,** as the 3, 2 and 1 are.

## Rule card

- **Reads:** the take's plan, its lifts and its units; the units ashore, their men and fatigue; the side's estimate of the enemy's men at the place; the risk on the chain.
- **Writes:** nothing but the staff's existing orders to a unit (attack, defend), their reasons and a notice.
- **Reuses:** the judgement ashore, the fit men, the men wanted and the units left behind; notices and standing interrupts.
- **New:** one test, "nothing more is coming", one row of a table, and one kind of notice. No state, no draw.

## What it costs

- **Games in which men ashore were waiting with nothing more coming change:** at balanced and at press they may now attack. The stage 3 proof's scenarios and the 39 days are checked, and any daily hash that moves is reported with the reason before golden files are remade. The rules label rises.
- **An attack at even numbers against men dug in may fail.** That is the commander's risk at balanced; at preserve the force nothing changes.
- **Rulebook:** chapter 8 (Taking a place, Ashore) and the glossary; the agent interface's notice kinds and its version.

## Scenario 105

9. **Nothing in its orders changes.** With this rule the Japanese men, 5,116 fit against 3,444 counted, attack at balanced on the first morning nothing more is coming. The land fight at Port Moresby then scores men lost on both sides. Whether the place falls inside the game is reported from the run.

## Amended on review

A reviewer read the plan against the take's code and ran the scenarios that have a take. Found right: the arithmetic, the fit men cut to a whole man, the men wanted, that only the side's own estimate is read, and that the table sits with the choice of units and with "not to starve".

10. **"Nothing more is coming" is replaced.** As written it missed men who come later: in the 39 days a take waits from day 11 to day 36 with no unit left behind while nine forces may be used from later days, and the rule would have told the commander, falsely, that nothing was coming. More is coming where, counted as "not to starve" counts what is in reach: a lift as it stands of this take has fighting men aboard; or a fighting unit of the take's forces limit that is not ashore at the place can be used and carried by the end's last day (its date, or today and seven days where it has none). A unit before its day counts from its day. A unit left behind that no ship of the take can ever carry does not count: the lift's force is lost, or was sent on to a hold's source, or "not to starve" forbids the lift.
11. **No new notice.** The notice `cannot_bring_enough` already means that the take cannot bring the men it wants on any day in reach. It is handed in for this case too, on each morning it holds, and its standing interrupt serves. The rule adds no kind and no state.
12. **Once sent in, they stay in.** The estimate of men in contact is drawn again each day and swings widely. With nothing more coming, a unit that is on attack is not set back to defend by the staff because that morning's estimate is larger; the engine's own rule sends a spent unit back, and it is judged again when it can attack. So the test of rule 3 decides when they go in and not, morning by morning, whether they stay.
13. **Today's rule also counts a unit on attack as able to attack,** whatever its fatigue; the plan left that out. A lift made at a base lands its units on attack; a lift taken as it stands lands them on defend.
14. **The worked example is corrected by the architect:** U and V are ashore on defend; the attack fatigue limit is the reference file's and the test states it; W is in the take's forces limit; statement 9 follows decision 0191; statement 4's and 6's notice is `cannot_bring_enough`; statements are added for rule 10 (a unit before its day is coming; a unit no ship can carry is not) and rule 12, with three properties: more fit men never turn an attack into a wait; a larger estimate never turns a wait into an attack; a lift added never lowers the men wanted.
15. **What moves:** of the scenarios with a take, the stage 3 proof's first file on days 37 to 39, where both takes wait with nothing coming and their fit men are below the estimate: new notices and no attack. The Coral Sea on day 5.
16. **For scenario 105 it is one day's fight.** The first judgement ashore is on the last day in every one of forty games; in 29 of them this rule sends the men in. Port Moresby will not fall inside the game.
