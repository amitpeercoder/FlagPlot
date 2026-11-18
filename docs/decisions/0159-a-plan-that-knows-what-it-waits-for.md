# 0159: A plan that knows what it waits for: the opening of step 3's stage 4

Status: a plan by the architect, written on Nov 18, 2026, for the designer's approval before anything of it is built. It plans what comes before the trial in stage 4 of step 3: the two mends that stage 3's proof left (decision 0153), waiting on the schedule, reconnaissance as a stated part of a plan, and a plan handed in as an order (decision 0158). Nothing is built. The trial and its estimate, stage 4 proper, get their own plan after this one.

Summary: A plan for the opening of stage 4: men ashore at a place not yet taken are fed by supply runs to the base their landing formed; a take that cannot bring enough waits for the day the schedule brings it, and asks the commander where it never will, in place of going with what it has; a commander may tell a plan to wait (not before a day, until a force may be used, until another intent is met) and may say how much must be known before a landing, apart from the risk word; where nothing in reach can look, the staff may move a carrier force to look and says what that costs; and the staff hands in each plan as an order, with its phases, what each waits for and what it needs, so that a tool and the table can show it. With the arithmetic of its cases, the conditions of its proof, six steps and six questions for the designer

Standing: Confirmed by the designer, Nov 18, 2026, with his six answers, each as recommended. Being built step by step; step 1, fed ashore, and step 2, a take that waits, are built; step 3, the limits to wait, is built with one property of its hand calculation (a wait that can never end) disagreeing by a day, which is with the architect

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

A supply intent is first served on the morning after it is formed: it is formed by a command taken in that morning, after the supply doctrine has served the intents then in force. The hand calculation first left this unasserted; the architect restated it, and scenario 79's test asserts that nothing is served on day 1.

Scenario 77 plays differently under the rule, as it should: its men are ashore on day 1 beside an airfield not yet theirs. The architect restated its hand calculation on Nov 18, 2026, and its test asserts the new figures. Four things changed: the supply intent is formed on day 2, a day sooner, and for base 3, the landing's base, not for base 2 (none is formed for base 2 on day 3, since one is in force for the hex); the run is called on day 3, not day 4; it is 80 t, not 85 t; and task force 3 carries it, not task force 1. The orders are still ten and the measure's counts are as they were. The team lead's check of two islands was restated with it: a keep-supplied intent's base is a base of the side in the hex of its island's place, under the hold of that place. No other scenario's test changed but two that run every scenario with no staff, which leave scenario 79 out as they leave out 77.

## As built: step 2

Step 2, a take that waits (rule B, items 4 to 8), is built, with scenario 80 and its test (`tests/scenario_80_a_take_that_waits.rs`). Every number of the hand calculation is asserted and passes: on days 1 to 3 the take forms nothing and gives no order, its reason saying 1,200 men wanted, 600 to hand, unit 2 from day 4 and that it waits for day 4, with no notice; on day 4 plan 1 is made ready, intent 2 formed with units 1 and 2 and the landing ordered; eight orders in all; the landing on day 8, base 2 the side's at its end, the plan complete on day 9; base 1 at 880 t at the end of day 5 and 871 t at the end, base 3 at 4 t, base 2 at 62 t, 968 t of fuel in each ship, 600 men in each unit. Its eight properties pass.

In the doctrine (`crates/sim-staff`, `take.rs`): where the first lift's choice is short of the men wanted, none of its men is aboard and the risk is preserve the force or balanced, the staff makes the choice again for each day after today, reading which units and forces may be used on that day (`available::not_before_on`, through the staff's day of choice, `Day::as_of`), and nothing more of the enemy. A day is in reach up to today plus `WAIT_HORIZON_DAYS`, 7, a placeholder, with no latest day; with one, where a landing formed that day would be ready by the date its task would be given, worked by the projection the notice `late` uses (`ready.rs`, now shared as `days_to_ready`). On the first such day it waits; with none, at balanced it hands in `cannot_bring_enough` once and forms nothing, and at preserve the force it is as before. The doctrine is `supply-36`.

Three readings the plan did not settle, made by the team lead and accepted by the architect on Nov 18, 2026: the plan of a take that waits stays a draft, not made ready, while it waits, since the hand calculation has the plan made ready on day 4 ; the notice carries its figures in the reason it rests on, as `late` does, not as data of its own; and a choice that finds no transport force or no unit at all today is as before, with no wait worked, since the rule speaks of a lift short of men.

The notice `cannot_bring_enough` (code 8) and the standing interrupt of that name (code 4) are added at the end. The state's layout is `flagplot-state-v45`; the saved game's version (20), the rules label (19) and the agent interface (25) are unchanged, since no key changes and a value added does not raise the interface.

Open for step 4 (knowledge): looking at the place needs the plan preparing, so while a take waits for its men as a draft nothing looks at it, and the count it waits on cannot change by the side's own looking. Looking while waiting is for step 4 or after.

Scenario 74's property "short of men, at balanced" plays differently under the rule, as it should: with unit 2 left out and an estimate of 500, the staff wants 1,000 men, has 600, and no unit is coming, so at balanced it no longer goes short with unit 1: it forms nothing, hands in `cannot_bring_enough` on day 1 and waits. The architect restated the property on Nov 18, 2026, and its test asserts it. No other scenario's run changed beyond its hash lines.

## As built: step 3

Step 3, the limits to wait (rule C, items 9 to 11), is built, with scenario 81 and its test (`tests/scenario_81_told_to_wait.rs`). The main case passes in every number of the hand calculation: plan 1 made ready on day 1 with intent 2 and the landing ordered with units 1 and 2; the notice that intent 2 is ready on day 2; plan 1 preparing and not released on days 2 to 5, each morning's reason saying it is not to go before day 6, with no notice on days 3 to 5 and none that it waits for leave; released on day 6 with the load of 90 t; eight orders in all; the force at B at hour 208 and back at A at hour 244; 160 and 440 men of each unit landed on day 9, base 3 formed and base 2 the side's that day; the 80 t landed by hour 220; the plan complete and the notice that intent 1 is met on day 10; base 1 at 874 t at the end of day 6 and 865 t at the end, base 3 at 4 t, base 2 at 62 t, 968 t of fuel in each ship, 600 men in each unit. The properties no limit, the limit lifted, leave withheld, wait for a force, after another place (intent 3 released on day 2 with 45 t, at C at hour 96, its 600 men landed in the night of day 5 and its supply by hour 105.75, base 4 the side's on day 5, intent 1 released on day 6), refused at intake, the same start and a save on day 3 pass.

One property disagrees: **a wait that can never end.** The hand calculation has the notice that intent 1 cannot be met handed in on day 3. The game hands it in on day 2, once, and none after; plan 1 is preparing and not released on days 2 to 6, and from day 2 the reason names the limit after intent 3 and that intent 3 is withdrawn. The commander's withdrawal of day 2 is taken in at the first step of day 2, before the staff is asked (chapter 8, Commands), and day 2 is the first morning intent 1's landing is ready, so the staff reads the withdrawn intent that morning. The test asserts the hand calculation and fails; the number is the architect's to rule on.

The state (`crates/sim-core`, `command.rs`): an intent's limits gain, after its risk, `not_before` (a day), `wait_for` (`Awaited`, a task force or a land unit, codes 1 and 2) and `after` (an intent), each `None` when unset; `Limit` gains `NotBefore`, `WaitFor` and `After` at the end, each with `None` to clear it; `CommandState::waits_read_up` reads them up the tree, the latest "not before" and every "wait for" and "after" on the chain, each with the intent it is set on. They are hashed after the risk and, as an amendment records them, with the codes 7, 8 and 9; the state's layout is `flagplot-state-v46`. Intake (`command/intake.rs`) refuses the three on an intent that is not a take or an island, or that the staff formed, as `NoWaitLimitOn` (added at the end), a set and a clear alike; a "wait for" naming a force or unit not the side's as `NotYourForce` or `NotYourLandUnit`, as a forces limit naming it; an "after" naming no intent in force as `NoSuchIntent`, and one naming its own intent, an intent of another kind or one the staff formed as `NotAnEndToWaitFor` (added at the end). No narrowing check applies: each is set only on a root. The rules label stays 19: no game without such a limit plays differently.

The doctrine (`crates/sim-staff/src/told.rs`, read in `take.rs` after the need to know of decision 0147 and before leave to launch): unmet as the hand calculation states; a force the side no longer has is one not in its view or with no ship, a land unit one not in its view or with no man ashore or aboard a ship of the side. An "after" whose place is the side's is met whether or not its intent is still in force; one withdrawn with its place not the side's can never be met. A limit that can never be met is said with `cannot_meet` once for the plan (the notice already handed in for the plan for any reason counts), its reason naming the limit, and the reason is given each morning after. The doctrine is `supply-37`.

The saved game is version 21 (its commands may hold the three limits, `from_day`, `task_force` or `land_unit`, and `other_intent`); the agent interface stays 25, since fields and values are added and none changes. A scenario's `[[command]]` and the tools take the same keys; the export writes the limits only where set, so a game without them exports as it did. Readings the plan did not settle, made by the team lead: the keys `from_day` (as `day` is a `[[command]]`'s own key) and `other_intent`; a cycle of "after" limits between two takes is not refused, and both would wait; the viewer's Command tab does not draw the three limits.

