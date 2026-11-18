# 0159: A plan that knows what it waits for: the opening of step 3's stage 4

Status: a plan by the architect, written on Nov 18, 2026, for the designer's approval before anything of it is built. It plans what comes before the trial in stage 4 of step 3: the two mends that stage 3's proof left (decision 0153), waiting on the schedule, reconnaissance as a stated part of a plan, and a plan handed in as an order (decision 0158). Nothing is built. The trial and its estimate, stage 4 proper, get their own plan after this one.

Summary: A plan for the opening of stage 4: men ashore at a place not yet taken are fed by supply runs to the base their landing formed; a take that cannot bring enough waits for the day the schedule brings it, and asks the commander where it never will, in place of going with what it has; a commander may tell a plan to wait (not before a day, until a force may be used, until another intent is met) and may say how much must be known before a landing, apart from the risk word; where nothing in reach can look, the staff may move a carrier force to look and says what that costs; and the staff hands in each plan as an order, with its phases, what each waits for and what it needs, so that a tool and the table can show it. With the arithmetic of its cases, the conditions of its proof, six steps and six questions for the designer

Standing: Confirmed by the designer, Nov 18, 2026, with his six answers, each as recommended. Being built step by step; step 1, fed ashore, is built

## What is so today, read from the code

1. **Men ashore are fed only from a base of their own side in their hex** (`ground.rs`, `feed_men_ashore`; decisions 0066 and 0090). A landing where the side has no base forms one, "Landing at (q,r)", and the supply the lift carried is landed into it. So the engine already feeds men at a place not yet taken. The staff does not: the island doctrine forms "hold" of the enemy's base at the place, the hold waits "for it to be the side's, and forms nothing and orders nothing" (`air.rs`, decision 0144, 1 to 3), and so no "keep supplied" is ever formed for the landing's own base. In stage 3's proof the Japanese side landed a median of 6,076 men at Lunga, fed them some 348 t in all, and lost a median of 1,543 to hunger.
2. **A take chooses its men against a count** (`take.rs`, decision 0142, 8 to 10): the estimate of the enemy at the place times 3 at preserve, 2 at balanced, 1 at press (placeholders). Where the ships cannot carry that many: at preserve it forms nothing; at balanced and press "the staff goes with what it has". Ashore it then attacks only if it has the men, and otherwise waits, for good if nothing more can be sent. In the proof the Allied staff wanted two for each of a median 10,012 counted, never had them, and attacked the airfield on 0 of 1,000 seeds against a mark of 600.
3. **A plan has a latest day and no earliest** (`by_day`). The staff leaves out a unit before the day it may be used and names the day (`available.rs`, decision 0140), and plans nothing against that day. The supply projection counts aircraft due and men on passage; nothing counts a ship or a land unit that may be used later.
4. **The staff looks before a landing** (`look.rs`, decision 0147, 13 to 20): at preserve it needs a count seen from the air or by men in contact no more than 3 days old, at balanced such a count of any day, at press none; one air group in reach is sent; the units are chosen again on what it finds; at preserve the landing is not released without it. All of this is in reasons. The commander's only say is the risk word, and no force is moved in order to look.
5. **A plan is handed in as a tree of intents with reasons.** A take has three phases that are read and not stored (prepare, move, land). What a plan needs, what it waits for and when it expects to go are in the words of reasons and nowhere as data.

## The rules

**A. Men ashore at a place not yet the side's are fed by runs.**

1. Where a hold names a base that is not the side's, and the side has a base of its own in that hex with fighting or working men ashore there, the staff forms "keep supplied" for the side's own base in the hex, under the hold, as it does for a base that is the side's (decision 0144, 13). The floor and the interval are those the hold would use.
2. When the place becomes the side's, the "keep supplied" stays on the base it names. Men in a hex draw on every base of their side there, so nothing is lost by it; a second "keep supplied" is not formed for the hex while the first is in force.
3. The reason of a hold that waits says which base the men are fed through.

**B. A take that cannot bring enough waits, or asks.**

4. The staff works the day the take could first lift the men it wants: the first day on which the units and ships it may then use, at the source, reach the number wanted. It reads the days on which units and forces may first be used; it reads nothing of the enemy.
5. Where that day is today, the take is as it is now.
6. Where that day is later and no later than the take's last possible day (from `by_day`, where one is set, or else within a horizon of 7 days, a placeholder of the staff's): at preserve and at balanced the staff waits for it, forms the task for that day, and says so with the day and what arrives. At press it goes with what it has, as now.
7. Where no such day is in reach: at preserve as now (it forms nothing and says it cannot meet the intent). At balanced it no longer goes with what it has: it hands in one notice, "cannot bring enough", with the men wanted, the men it can bring and the day, and waits for the commander, who may set the risk to press (go with what there is), set a limit (see C and D), or leave it waiting. At press as now.
8. Landing at a beach away from the enemy and marching is not in this plan (question 5).

**C. A commander may tell a plan to wait.**

9. Three limits on an intent, each read up the tree as the other limits are: **not before** a day; **wait for** a named task force or land unit to be usable at the source; **after** another of the commander's intents is met (its measure says met, or its plan is complete).
10. While a limit of these is unmet, the staff prepares what it may (readying, loading, looking) and does not release. The plan's state stays preparing, its reason names what it waits for, and leave to launch is not asked until then.
11. A limit that can never be met (a force that is sunk, an intent withdrawn) is said once as a notice, and the plan waits.

**D. What must be known is the commander's to set, and is part of the plan.**

12. A limit **knowledge** on an intent, apart from the risk word: none; a count (seen from the air or by men in contact, of any day); a recent count (no more than so many days old, 3 unless the commander gives a number). Unset, the risk word sets it as today.
13. The need to know is handed in as data with the plan: what must be known and where, the grade and age asked, what is held now with its grade and day, who is looking and since when, and what the count held could be in truth, from the grade's range.
14. **A force moved in order to look.** Where the need is unmet and no air group the intent may use reaches the place from where it is, the staff may move one carrier force the intent may use to the nearest sea hex from which one of its groups that can search reaches the place, and look from there. It is formed as the task "operate the carrier force from a hex" that exists, with the hex chosen for reach, and it says the days of passage and that the force is then within so many miles of the place. At preserve the staff proposes it and waits for leave; at balanced it does it; at press nothing is needed. The force comes back to where it was when the need is met.

**E. A plan is handed in as an order.**

15. For each intent of the commander's the staff hands in an order made of what it already works out, as data: the mission (the intent in words); the phases, in order, each with the condition that ends it and whether it is done, under way or to come; what each phase waits for (leave, a day, a force, another intent, knowledge, strength), with the day expected where one can be worked; what it needs and has (men wanted and to hand, ships, supply); and the tasks under each phase, which are the staff's intents as today, with their reasons beneath.
16. The phases are read from the state, as a take's three are today, and not stored. A day on which a task had no order is not part of the order; it stays in the task's reasons.
17. A tool gives the order, and the table shows it. It adds nothing a side does not already have.

## Rule card

| | Reads | Writes |
| --- | --- | --- |
| A | the side's own bases in a hold's hex, its men ashore there | a "keep supplied" intent under the hold (command state, as today); command intake takes one under a hold for a base of the side in the held base's hex |
| B | units and forces with the day each may be used, berths, the estimate, `by_day`, the risk | the take's task day; one new notice, "cannot bring enough" |
| C | the three limits, the day, a unit's usable day, another intent's measure and plan | three new limits on an intent (command state: the hash version is raised) |
| D | the knowledge limit, the estimate with grade and day, groups and their reach, carrier forces | one new limit; the "operate carriers" task that exists, with a hex chosen for reach |
| E | the command state and the staff's own working | nothing in the state: the order is worked when asked |

No rule of a day's resolution changes. One rule of command intake does (rule A: "keep supplied" may stand under "hold a place" for a base of the side in the held base's hex). No random draw is added. The staff reads nothing of the enemy but the side's own estimates and contacts.

## What it costs

- The doctrine changes for every game with a hold or a take, so its label is raised and the generated files of those games are remade. A game with neither plays as before.
- Four limits and one notice are added to the command state: the state layout, the saved game and the agent interface are each raised once.
- Stage 3's proof is not run again to turn its two misses into passes. They stand as named. This plan has its own proof.

## The arithmetic of its cases

Each step's test carries its full hand calculation, written by the architect in the test's doc comment before the step is delegated. These are the figures the plan itself rests on, on the live figures.

- **Hunger, as today.** 4,000 men ashore at a landing's own base, the lift's 15 days landed with them. The charge is 0.005 t a man a day: 20 t a day, 300 t landed. Fifteen nights are paid in full and the stock is then nothing. On the first unpaid night all 4,000 are unfed and one in twenty of them is lost: 200, leaving 3,800. On the second, 190, leaving 3,610. On the third, 180.5 cut to 180, leaving 3,430. With rule A a run is called as for a held base, at a floor of 5 days, which is 100 t for 4,000 men; the days of the run are worked in the step's test from the planner's rule as it stands.
- **A take that waits.** The enemy at the place is counted at 3,000, seen from the air. At balanced the staff wants 2 for 1: 6,000. Three units of 1,500 may be used today, day 8: 4,500. A fourth of 1,500 may be used from day 12. Today the staff goes with 4,500. With rule 6 it waits for day 12 and lifts 6,000. With `by_day` set so that the last day to lift is day 10, no day in reach brings 6,000: at balanced it hands in "cannot bring enough: 6,000 wanted, 4,500 to hand, a fourth unit from day 12" and waits. At press it wants 3,000 and goes today with two units.
- **What a count is worth.** A count seen from the air is the truth times something between 0.25 and 2, so a count of 3,000 means between 1,500 and 12,000 men. A count by men in contact is the truth times 0.5 to 1.5, so 3,000 means between 2,000 and 6,000. The need handed in with the plan says which of these the count held is.

## The conditions of its proof

Written before anything is built. None is a tuning mark; each can fail. Stage 3's marks are not among them.

1. **The hand calculations** of the steps pass.
2. **Nothing changed where it is unused.** Every generated file of a scenario with no hold and no take is as it was.
3. **Men landed are fed.** The 39 days, staff against staff, as stage 3's proof arranged them, on 1,000 seeds: of the men a side lands at a place it has not taken, the median lost to hunger is no more than one in twenty of those landed. (Stage 3 saw about one in four.)
4. **No take goes short when waiting would have brought enough.** On the same seeds, at balanced: the number of lifts that sail with fewer men than wanted while a unit that would have made up the number could be used within the horizon. The mark is none.
5. **A plan that waits says what for.** Built cases, one of each: not before a day, wait for a force, after an intent, knowledge unmet, strength unmet. In each the order names the thing waited for and the day expected where there is one, and the plan is not released a pulse before the condition is met nor a day after.
6. **Legible.** A reader given the order alone says, for six plans, what its phases are, which is under way, what it waits for and when it expects to go: six of six.
7. **A seat is told nothing more.** The order is made from the side's view; the test that reads the agent's source still passes.
8. **Reported, not judged:** on the same seeds, whether the airfield is attacked and taken, and the margin, beside stage 3's figures.

## The steps

Each is one branch and one pull request, read by the architect before it merges.

1. **Fed ashore** (A). The doctrine and its scenario.
2. **A take that waits** (B), with the notice.
3. **The limits to wait** (C): the state, the tools, the saved game.
4. **Knowledge** (D, 12 and 13): the limit and the need as data.
5. **A force moved to look** (D, 14).
6. **The order** (E): the tool, the table's plan sheet, the rulebook's chapter 8 and the glossary through every step. Then the proof, run once.

## Questions for the designer

1. **At balanced, a take that can never bring enough asks you, and no longer goes with what it has.** Recommended: yes. Going short is what "press" is for.
2. **The staff waits by itself when the schedule will bring enough,** within the latest day you gave or else 7 days. Recommended: yes at preserve and balanced, not at press.
3. **How much must be known is yours to set, apart from the risk word.** Recommended: yes, as three levels with an age in days.
4. **A carrier force may be moved in order to look,** at balanced without asking and at preserve with your leave. Recommended: yes, in this opening; it is the cost of knowing made visible.
5. **Landing at a beach away from the enemy and marching.** Recommended: not now. It waits for marching between hexes on the 39 days, which stage 4 of ground combat owns.
6. **Order of work.** Recommended: these six steps, then the trial as its own plan. The trial needs a plan that says what it waits for and needs, which is what these steps give it.

## Confirmed on Nov 18, 2026

The designer approved the plan and answered the six questions "go with your recommendations":

1. At balanced, a take that can never bring enough asks the commander and no longer goes with what it has.
2. The staff waits by itself when the schedule will bring enough, within the latest day given or else 7 days, at preserve and at balanced.
3. How much must be known is the commander's to set, apart from the risk word, in three levels with an age in days.
4. A carrier force may be moved in order to look: at balanced without asking, at preserve with leave.
5. Landing at a beach away from the enemy and marching is not in this plan.
6. The six steps first, then the trial as its own plan.

## As built: step 1

Step 1, fed ashore (rule A, items 1 to 3), is built, with scenario 79 and its test (`tests/scenario_79_fed_before_it_is_taken.rs`). Every number of the hand calculation is asserted and passes: intent 2, keep base 3 supplied at a floor of 5 days, formed under the hold on day 1; no run on the morning of day 2 (42 t read, 30 t on day 4, not below the floor of 30 t) and on day 3 a run of 90 t (36 t read, 24 t on day 5), landed at B as 40 t and 50 t on day 4; three orders to task force 1 and none to a land unit; base 3 at 54 t, base 1 at 910 t, base 2 at 72 t, 968 t of fuel in each ship, and every man fed. Its five properties pass.

The plan was wrong on one point: it said no rule of the engine changes for rule A. Command intake let "keep supplied" stand under "hold a place" only for the held base (decision 0144, 7), and so refused the one the staff forms for the side's own base in the held base's hex, every morning. The designer was told and left the ruling to the architect on Nov 18, 2026, who ruled that intake take it: "keep supplied" may stand under "hold a place" for the held base or for a base of the side in the held base's hex; one in another hex, or of the other side, is refused as before ("cannot stand under"). No state, refusal or random draw is added, the hash is unchanged and the rules label is not raised: no saved game holds such a command, because a commander states roots and the staff's commands are worked again under the doctrine version.

In the doctrine (`crates/sim-staff`): a hold of a base not the side's reads the base's hex from the side's own map of the bases it does not hold, never from the truth. Where the side has a base of its own there and men of any unit ashore, it forms "keep supplied" for the lowest-numbered such base by the same rule as for a base that is the side's, and its reason says through which base the men are fed. Men who serve a "take a place" not yet met are counted, on the architect's ruling: rule A feeds any men of the side ashore in the hex. A hold with no man in the hex, or no base of its own there, plays as before. The doctrine is `supply-35`.

Whether the staff works the supply sum on the morning it forms the intent was left unasserted by the hand calculation. It does not: the intent is formed by a command taken in that morning, and the supply doctrine serves it from day 2.

