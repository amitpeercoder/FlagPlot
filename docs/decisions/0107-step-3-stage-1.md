# 0107: Step 3, stage 1: the command contract, and a staff that keeps a base supplied

Status: a plan by the architect, written on Nov 14, 2026, for the designer's approval, and revised the same day after an independent review that the designer asked for. The review found the direction right and six gaps that had to be closed before building: the order of a day, where a staffed day is run, how the staff writes the state it makes, what a saved game holds, what "days of supply" means, and how a supply plan can change without the commander. Each is settled below and marked *(review)*. `docs/step3-spec.md` settles what the contract is; this record says how stage 1 builds it. The designer answered its four questions on Nov 14, 2026, each as recommended, and approved the plan: stage 1 is built as laid out, after the two hand calculations are written, and anything they show still open comes back to him before any code. Confirmed. The two hand calculations are now written; they needed eight rulings, recorded below, and raised one question, on how much a run carries, which the designer answered after research: a supply intent carries a floor and an interval between shipments, 15 days to start. Nothing is built. Two hand calculations, for days of supply and for one convoy, are written by the architect before approval is asked, since they are what shows a rule still open.

Summary: Step 3, stage 1: the command contract as state changed by commands; the staff as code outside `sim-core`, passed in to the function that resolves a day and asked after the morning's refuelling; staff commands for the state the staff makes; a saved game keeps what was handed in and a check runs the staff again; four rules it needs (a load up to a weight, a reserve at a base, forming and joining forces, a scenario's commands); days of supply for a side in a hex; the doctrine for keeping a base supplied; the tools; built in four steps

Standing: Confirmed by the designer, Nov 14, 2026, after an independent review. Step 1's engine half built Nov 15, 2026 (command state, commands, the staff passed in, the saved game version 3, days of supply; scenario 35; state version 24); the staff, its doctrine and the tools are not built

## What stage 1 is

The spec's row: the command contract, whole, and a staff for one area intent, keep a base supplied from a pool of shipping. It shows the loop from end to end: propose, explain, approve, run a quiet week, a shortage that stops the clock, a hold and its release, a save in the middle.

It has no fighting doctrine, no landing and no trial. Those are stages 2, 3 and 4. The enemy in stage 1's cases is the script, or nobody.

## Where each part lives

1. **What the commander says is state.** Intents, limits, holds, the two leaves, approved plans with their amendments, commitments, and interrupts set and acknowledged are part of each side's state in `sim-core`, in a module of their own. They are hashed and named beside the hash. They change only by a **command**, a new kind of thing handed in beside orders, checked and accepted or refused with a reason as an order is.
2. **Every change to command state is an event,** with its causes, seen whole by its own side and not at all by the enemy, with its row in `side_events.rs`. *(review)*
3. **What the staff decides is code outside `sim-core`,** in a new crate, `sim-staff`. It depends on `sim-core` only. Every function in it takes a `SideView` and nothing else, and a test reads its source to check. It forbids floating point as `sim-core` does, because its decisions reach the hash. *(review)*
4. **The staff hands in three things:** orders, each with its reason; **staff commands,** for the state the staff itself makes; and decisions to give no order, each with its reason. A staff command may do only what the spec lets the staff do alone: commit and free ships and stock, add and time runs inside an approved plan (decision 22), mark a plan complete or failed, and, where the commander has given leave, move a draft to preparing or a plan to released. Intake refuses anything else from the staff. Staff commands are taken in, hashed and saved as the commander's are. *(review)*
5. **What is worked out and never stored:** proposals, previews, the command report, days of supply and the staff's projection of stock. Each is a function of the state and the view.
6. **The view carries the side's own command state,** whole, and nothing of the enemy's. Whether a side has a staff, and what its intents are, is never told to the other side.

## The order of a day *(review)*

The staff must see today's holds and reserves, and the stock after the morning's refuelling. So the day's sequence is fixed in `sim-core`, which is the only place that can hold it:

| Step | What happens |
| --- | --- |
| 1 | Commands handed in for the day are taken in: intents, limits, holds, leaves, interrupts, approvals |
| 2 | An order the commander has handed in for a unit that is in any intent's forces limit holds that unit, from now, as an override |
| 3 | Forces back from the yard are home. Ships in port refuel, each base down to its reserve and no further |
| 4 | The staff of each side that has one is asked, on the view as it now stands |
| 5 | Staff commands are taken in. Then all orders, the commander's and the staff's. A staff order to a held unit is refused, as a guard behind the staff's own check |
| 6 | The rest of the day, as it is |

7. **The staff is passed in, not called from outside.** `sim-core` defines what a staff is: something that, given a side's view, returns orders, staff commands and reasons. The function that resolves a day takes each side's staff, or none. `sim-staff` provides the staff. Every loop that resolves days passes it: the session, `run`, `batch`, the validation harness and the timing command. `sim-core` still depends on nothing, `sim-agent` still depends on `sim-core` only and is handed its staff by its caller, and a staffed day is the same sequence wherever it is run. *(review)*
8. **A day with no staff and no command is resolved exactly as today.** Every existing scenario, baseline and recorded game is unchanged.

## What a saved game holds *(review)*

9. **A saved game keeps what was handed in,** as it does today: the commander's commands and orders, and the staff's commands, orders and reasons, each with its origin. A replay hands them in again and does not run the staff. So a replay still works when the doctrine has since changed.
10. **A check runs the staff again and compares.** `replay --check-staff` asks the staff at each day and reports the first difference from what was saved. It is the test that the staff is a function of the view, and CI runs it on every staffed recorded game.
11. **The doctrine has its own version,** kept in `sim-staff` and recorded in the save beside the rules label (decision 0100, 10). It is checked when a game is resumed for more play, which is refused under another doctrine, and not when a game is replayed.
12. **A reason is kept, not worked out again.** It is handed in with its order or with the decision to give none, recorded as an event, and saved. `why` on last week's decision reads it. So a reason still resolves after its intent is replaced.

## The four rules stage 1 needs

Each gets a decision record of its own with a rule card and a hand calculation; the rulings here are what those records start from.

13. **A load up to a stated weight.** A load order may name a weight of supply or of fuel for the force: load until that much is aboard, shared among its ships in ship order. It is complete when the weight is aboard, or the room or the stock ends. It does not apply to men. With no weight it is as today.
14. **A reserve at a base.** A base has a supply reserve and a fuel reserve, 0 unless the reserve limit sets them.
    - Refuelling takes a base's fuel down to its reserve and no further, for every force, the commander's own included: it is a rule and not an order. Where a side has two bases in a hex, refuelling goes on to the next when the first is at its reserve.
    - A load ordered by the staff is capped when it is worked: it never takes a stock below its reserve, whatever was eaten in the night. So a draw the staff's own plan causes never goes below a reserve, as the spec requires.
    - What the men eat, aircraft upkeep, repairs, bombs and shells still take a stock below it. Those are events.
15. **Forming, joining and standing down forces, at a base.** Ships in port in no force may be formed into one. Two forces stopped at the same base may be joined. A force stopped at a base may be stood down, its ships then lying in port in no force. Each is refused at sea, for a held force, and for a force away at the yard. A formed force has no mission, cruise speed, the engagement settings a scenario's force has by default, and this base as its home port. In a join the receiving force keeps its orders and settings and takes the other's ships with what they carry; the other's orders end. The existing order that detaches ships is left as it is.
16. **A scenario may state a side's commands,** by day, as it states orders, so that a test can begin with an intent in force.

Refuelling that costs time is **not** in stage 1: question 1.

## The contract in stage 1

17. **One kind of intent:** keep a base at so many days of supply. It names the base and the days. It has no date: it stands until withdrawn. An intent for fuel waits for stage 2, since a base's fuel is used only by the forces sent to it and stage 1 sends none. *(review)*
18. **All six limits exist.** Stage 1's doctrine reads forces, area, priority, protected and reserve. Risk is stored and shown and read by nothing until stage 2.
19. **The forces limit of a supply intent names the pool and exactly one source base.** Several sources are for a later stage.
20. **Holds** on a setting, a unit and a parent force are tested apart, as the spec asks. Guarding a hold does not freeze what is above it: a held ship takes the force it is in out of the pool, and nothing else of the side. A held setting has nothing to bite on in stage 1's doctrine and is tested by a stub.
21. **Leave to prepare and leave to launch** are two settings of a seat, with the spec's defaults.

### A supply plan *(review)*

22. **A supply plan's baseline is its intent:** the base, the days of supply and the limits it was approved under. Its convoy runs are tasks inside its one phase. The staff may add a run, size it and time it alone, inside the limits: question 3 asks the designer to confirm this reading of what the staff may change alone. What needs the commander is a change to the days, the base or a limit, and cancelling.
23. **Its phase** begins when the plan is released and ends when the intent is withdrawn. **Released** covers every run of the plan: leave to launch is asked once. **Preparing** is forming and loading the first run, which does not sail.
24. **Its fallback** is come home: on cancelling, or when a run cannot go on, its ships return to the source.
25. **With no commander,** it follows the spec's order: slip the run, then accept fewer days than asked and say how many, then stand the plan down.
26. **A run** is carried out by these orders, which the staff gives on the days named: form the force if it is not formed; a load order of a stated weight at the source; and, on the first day the load is complete, a route to the destination and back, marked to act at the destination, with an unload order (decision 0096). Loading and sailing cannot be the same day, because a day's movement comes before its loading.
27. **Commitments** are of ships by day and of stock by weight and day. A ship is committed from the day its run is planned until it is back at the source and refuelled. A ton at the source is committed from the day its run is planned until it is loaded. Two plans never count the same ship or the same ton.

### Interrupts, runs and reports

28. **Interrupts** in stage 1: a base's days of supply falling below a number; a plan that can no longer be met; a ship of the side lost; a proposal or a leave waiting for the commander. They fire on the change.
29. **A run of days** resolves up to a number of days and stops between days for an interrupt, with every reason of the day listed together. Before it runs, the staff's orders for the first day are listed. One day of it is exactly `end_day`. It is offered to a game with one seat held by a player.
30. **The command report** keeps one identity for a matter that lasts: the plan, the run and the kind of matter. A convoy late for a week is one open matter with a history, and it is the same matter after a save and a resume.

## Days of supply *(review)*

31. **It is for a side in a hex, read when the staff is asked:** after the morning's refuelling and before the day's orders. An intent names a base and means its hex, since the men of a side are fed from every base it has there.
32. **It is the side's supply stock in the hex over its planned use for that day.** Planned use is:
    - the men's supply for its men ashore there, each unit at its weight;
    - the daily upkeep of its aircraft there, ready and damaged;
    - the repairs its bases there would pay for that night: for each, the smaller of its repairs a day and its damaged aircraft, times the supply of a repair;
    - the sorties of its groups there whose mission flies whatever is sighted, search and patrol, at each daylight pulse, with the aircraft able to fly.
33. **A strike mission counts nothing in stage 1.** Whether it flies depends on what is sighted, and a figure that moved with enemy reports would set off interrupts for no change in supply. Stage 2, which gives strikes a doctrine, decides how they are planned for.
34. **Standing down.** The staff of stage 1 gives no order to an air group, so it cannot raise the figure by resting one. From stage 2 a group the staff itself stood down is counted as flying. A group the commander rests is counted as resting: it is his order.
35. **The figure is today's; the staff plans on the days ahead.** Its projection takes each coming day's planned use, with what is on its way counted from the day it is due: aircraft arrivals and replacements the scenario has scheduled, and men aboard the side's own forces that are ordered to land there. The report gives the figure and the first day the projection falls below the intent.
36. **No planned use, no figure,** said in words. A hex that is not the side's has none.

The four cases the spec asks for are the architect's first hand calculation: a unit arriving, a surge of sorties, no consumption at all, and a base taken and lost. The surge has a scenario of its own, with aircraft and no wear, so that it has no dice.

## The doctrine: keep a base supplied

37. **Each day, for each supply intent in priority order,** the staff projects the hex's stock day by day, from planned use and the runs committed.
38. **One run of a plan is being loaded at a time.** A new run is planned on the first day when no run of the plan is loading and the projection, with the runs under way, falls below the intent's days on some day before a new run could land.
39. **A run's ships** are the pool's free ships in number order, taken until their room covers the load or none is left.
40. **A run's load** is what brings the hex to its days of supply on the day the run lands, plus the planned use of one round trip of those ships, so that the next run can be back in time. It is no more than the ships' room and the source's stock above its reserve and its commitments. It is fixed when the load order is given and not sized again.
41. **A run can be made** if the pool has a free ship, the source has stock to give, a sea route lies inside the area limit, and each ship has fuel for the round trip after refuelling at the source. The staff checks the route the engine would take and adds waypoints to keep it inside the area; if none does, it has no route.
42. **Times** are worked from the cargo rule's own rates, berth or lighter, at both ends, and the passage at cruise.
43. **A run under way is not recalled** for a higher intent in stage 1. The higher intent has the ships as they come free.
44. **When it cannot meet an intent** it says which binds: no ship free until a day, the source's stock, no route in the area, ships without the fuel, or the hex using more than the pool can carry. It gives the shortfall in days and goes on with what it can do, and says the day the source itself runs dry at the present rate.
45. **Escort, cover and what the enemy may do to a convoy are not in stage 1.** A ship lost is an interrupt and a new plan.

## The tools

46. **A seat with a staff has these,** beside the tools that exist. Each has a schema, and the agent interface's version rises.

   | Tool | What it does |
   | --- | --- |
   | `command_state` | Reads what is in force: intents, limits, holds, leaves and interrupts |
   | `set_intent`, `withdraw_intent` | State or remove an intent |
   | `set_limit` | Set forces, area, priority, a protected commitment, a reserve or risk |
   | `set_leave` | Give or withhold leave to prepare, and leave to launch |
   | `hold`, `release` | Hold a setting, a unit or a force; release it |
   | `plans` | The side's plans: state, baseline, amendments, runs and what each is doing |
   | `propose` | The staff's draft for an intent: the plan, the reasons, what binds, what it displaces |
   | `preview` | What a changed intent or limit would cost, tied to a revision. Changes nothing |
   | `approve` | Prepare a draft, release a plan that is preparing, or apply what was previewed |
   | `cancel_plan` | Cancel a plan; its ships come home |
   | `staff_orders` | The orders the staff would give for the coming day, each with its reason |
   | `set_interrupt`, `acknowledge` | Set a condition; acknowledge one that fired |
   | `run` | Resolve up to so many days, stopping for an interrupt |
   | `report` | The command report: what changed, what is open, each item leading to its reasons |
   | `why` | The reason for a staff decision, by its number. A decision to give no order has a number too |

47. **Days of supply is in what exists already:** the list of bases and the briefing.
48. **Unit orders still work.** An order from the commander to a unit in an intent's forces limit is an override and holds the unit (step 2 of the day).
49. **A language model must be able to command through these from their descriptions alone.** That is tested in step 4 by a recorded game, as scenario 9 was.

## Built in four steps

| Step | What is built | What runs at the end |
| --- | --- | --- |
| 1 | The four rules. Command state, commands, staff commands, the day's order, the staff passed in, events, the save and the check. Days of supply. Commitments of ships. The doctrine for one intent. `command_state`, `set_intent`, `set_limit`, `propose`, `approve`, `staff_orders`, `why` | One convoy plan, proposed, explained, approved and carried out, saved and replayed |
| 2 | Priority, commitments of stock, a second intent, displaced demand reported | Two hexes and one pool: the higher intent is met, the lower's shortfall is stated with its cause, and a changed priority moves the ships as they come free |
| 3 | Holds and release, overrides, previews and amendments, the two leaves, cancelling | A hold, a changed intent, a week, a release. A plan brought ready to launch that has not sailed; and with leave given, the same plan run through unattended |
| 4 | Interrupts, `run`, the command report, resuming under the doctrine's version | A quiet week in one call; a shortage that stops the clock; a save in the middle, resumed, matching play that was not interrupted; a model commanding by the tools alone |

## What it must pass

Each case gets its numbers in a hand calculation or in the test plan of its step, written before the code.

- **Hand calculations:** days of supply (the four cases); one convoy from stock, use, rates and passage; one pool between two hexes; a load up to a weight with two forces loading from one base without both counting the same stock, and a refuelling stopped at a reserve with a second force refuelling the same morning.
- **The contract's cases of the spec** that stage 1 can reach: two bases and one convoy; a hold, a changed intent, a week, a release; a sunk supply ship; an intent that cannot be met; a plan cancelled with ships at sea; the save case. "A missed date" does not arise: a supply intent has no date.
- **Running time:** a quiet week in one call; a condition already true at the start; a shortage that lasts; two reasons on one day; an intent changed after a preview.
- **Properties:** the staff never orders what it does not own or what is held, directly or through a parent force; it never plans below a reserve or outside the area; every order given or withheld has a reason that resolves, also after its intent is replaced; the same inputs give the same orders; a staff with no intent gives no order; a staffed game's check passes.
- **No truth:** two worlds that differ only in what the side cannot see give the same orders, staff commands, proposals, reasons, previews, interrupts and reports.
- **Against no orders:** on a placeholder scenario with no enemy, no aircraft and so no dice, over 30 days: with the staff, from the day the first run lands, the hex's days of supply never fall below the intent's less the days one round trip takes; with no orders, it has run out by the day the hand calculation gives.
- **Legibility:** a cheap model, given the reason of each of a set of stage 1's decisions, says what the staff was trying to do and what would change its mind, at a pass mark written in step 4's test plan before the run.
- **Speed** is measured and reported, by the command of decision 0100: a day's staff decisions for one side and a quiet week in one call. It is not a test in stage 1: no number has been agreed to fail against.

## What it costs

- **A new crate,** `sim-staff`, and the lines for it and for the staff passed in, in `docs/engineering/layout.md`.
- **New state** for each side's command state, in the hash. The state version rises when the layout does. The rules label rises only where an existing game's result could change, which a day with no command never does.
- **Four rule changes** with their records, and a new chapter of the rulebook, "The staff", which stage 1 begins.
- **The tools** with their schemas, and the text that tells a player how to command by intent.
- **Placeholder scenarios** for supply, and the hand calculations.

## What it leaves for later stages

- Every intent but supply; fuel; strikes in planned use; escort and cover; recalling a run; several sources; the risk level's numbers; the enemy played by the staff (stage 2).
- Landing, marching, units made ready, bases built up (stage 3).
- The trial, and with it the finding of stage 0 that a world built from one side's view holds no enemy the side has not seen (stage 4).

## Rulings from the hand calculations

Made on Nov 14, 2026, while writing the two hand calculations the designer asked for before any code (`tests/scenario_35_days_of_supply.rs` and `tests/scenario_36_one_convoy.rs`). Each closes a place the decisions above left open, and stands over them where they differ.

1. **Men due today count today** (decisions 32 and 35). A day's planned use counts the men aboard the side's own forces that are ordered to land in the hex and are due that day, or were due earlier and have not landed. So the figure falls on the day a regiment is due, a day before the base first pays for it, and the figure and the projection agree.
2. **A strike mission counts nothing, a patrol counts its two daylight pulses, a search counts its two daylight pulses at the distance it is ordered to fly** (decision 32). They are costed by the rule that charges sorties.
3. **An order given today shows tomorrow.** The figure is read before the day's orders are taken in, so a group set to patrol today raises the planned use from the next day's reading.
4. **When a run is needed** (decision 38). A run ordered on a day is needed if, without it, the figure read on the day after it would arrive is below the intent's. The staff does not send a run earlier than that.
5. **A run's load** (decision 40) is the floor's stock, less the stock projected for the reading on the day the run arrives and not below 0, plus the planned use of one interval between shipments (question 5, as answered). The first draft of this ruling had one round trip of the ship in place of the interval.
6. **A round trip is counted in whole days:** the passage both ways at cruise, rounded up to whole days, and a day to load and a day to unload. It no longer sizes a load. The staff uses it to say when a ship is free and whether the pool can keep the interval.
7. **Forming a force and its load order are given on the same day.** A side numbers its own forces, so the staff knows the number the new force will have.
8. **A ship is free when it is back at the source and has refuelled there,** which is the morning after it arrives.
9. **The hour a force steams on does not move a run** (Nov 15, 2026, after the build). A force that acts at a place steams on at the start of the first pulse after its work is done (decision 0096), not the hour the work ends; the table of scenario 36 first had it leave on the hour, and was corrected. The doctrine counts a round trip in whole days and a ship's free day from its arrival, so the hour a force steams on does not move a run.

## A question the hand calculation raised, and its answer

5. **How much a run carries.** By the first draft of ruling 5 a run carried only one round trip's use above the floor: in the worked case, a ship of 3,000 t carrying 40 t every four days. The designer asked that it be researched before it was settled. The research is `docs/research/oob/supply-levels-and-convoy-cycles.md`. It found that the doctrine of the period kept two levels at an overseas base, a minimum held in reserve and an operating level above it, and that the operating level was the cushion between regular shipments: what a shipment brought was the use of one interval between shipments (`land_unit_facts` 1057, 1065), not of one voyage. The intervals the rows state run from 15 days, for fuel oil (1099), to a month. No row sizes a run to Guadalcanal itself.

   **Answered by the designer on Nov 14, 2026, as the architect recommended:**
   - **A supply intent carries two numbers:** the floor, the days of supply the hex is never to fall below, and the interval between shipments, the days of use each run brings above the floor. Decision 17 is read so. The commander may set either; the staff proposes the interval where he has not.
   - **The interval's first value is 15 days.** It rests on one row and on a reading of which kind of base a forward base is. It is a starting value. Stage 1 has no enemy, so nothing in it can show whether 15 or 30 is the better: fewer and fuller sailings against more stock lying where it can be bombed or taken. It is looked at again in stage 2, when a convoy can be lost, escort is scarce and the risk level has its numbers.
   - The hand calculation of scenario 36 is reworked for it: runs of 360 t and 150 t, fifteen days apart, the stock between 30 and 45 days.

## Questions for the designer

Answered on Nov 14, 2026, each as the architect recommended.

1. **Refuelling and time:** reserves only now. Refuelling that costs time is its own decision, after step 1 and before stage 3, as decision 0102 orders it.
2. **A reserve and the commander's own orders:** a reserve binds the staff and automatic refuelling. It does not bind a load order the commander gives himself; the tools tell him when his order goes below it, and what it takes from stock the staff had committed is reported as displaced.
3. **What the staff may change alone in a supply plan:** adding, sizing and timing runs inside the approved limits is the staff's alone. The baseline is the intent and its limits. A change to the days, the base or a limit, and cancelling, come to the commander.
4. **The whole plan:** approved, to be built after the two hand calculations. If writing them shows a rule still open, it comes back to the designer before any code.

## As built (step 1, the engine)

Written by the team lead on Nov 15, 2026, before the code, as the design the build follows; corrected where the build differed. It is the first of step 1's two pull requests: everything in `sim-core`, `sim-data` and `sim-cli` a staff needs to exist, with no staff and no doctrine. The second builds `sim-staff`, the doctrine and the tools on it.

**Command state** is `sim_core::command::CommandState`, one for each side, held by the world beside the beliefs (`World::command`), hashed after them and named beside the hash. It holds the intents by number, in force or withdrawn (a withdrawn number is never used again); the two leaves (default: prepare given, launch withheld); the plans by number; each ship's commitments; and the side's next plan number. The one intent kind is `KeepSupplied { base, floor_days, interval_days }`, the interval `None` where the commander has not set it. An intent carries its limits as they stand: forces (ships, task forces, exactly one source base), area (a set of hexes), priority (a rank, 1 first; unset comes after every ranked intent, then by number), protected (ships), risk (preserve, balanced, press; stored, read by nothing). The reserve limit names a base, not an intent, and writes the base's two reserves of decision 0111: one store, not two. Holds and interrupts have their place marked in the type and build nothing.

**A plan** is drafted when an intent's number is first set, one plan for each intent, numbered in the side's own series. Its state is one of the six. On its first approval its baseline is taken: the intent and its limits on that day. A change the commander makes afterwards to the intent or to one of its limits, while the plan is approved, is a numbered amendment of it. Withdrawing the intent ends the plan: complete if it was released, else cancelled. Setting again an intent whose plan has ended drafts a new plan. A run is a task inside the one phase: its number in the plan, its ships, its load in tons of supply, its source (the intent's) and four days, loading, sailing, due at the destination, and free (back at the source and refuelled). A commitment is of one ship to one plan (and run, if named) from one day to another, inclusive; a ship holds at most one on any day.

**Commands** are `CommandKind`, handed in beside orders. The commander's (`World::submit_commands`, issuer the player) are taken in at step 1 of the day: set and withdraw an intent, set a limit, set a leave, approve (prepare a draft, release a plan that is preparing), cancel a plan. The staff's come in its hand-in and are taken in at step 5: commit and free ships, add and time a run, mark a plan complete or failed, and approve, which for the staff needs the leave for that move. Each kind says who may give it, and intake refuses any other issuer. Each command is checked against the side's own units and command state, and recorded as `CommandTaken` (with the whole command) or `CommandRefused` (with the command and a reason, one reason for a unit that is not the side's and one that does not exist). What a command sets off is a follow-up event citing it: `PlanDrafted`, `PlanStateChanged`, `PlanAmended`. Each is seen whole by its own side and not at all by the other (`side_events.rs`). There is no command number: the event is the command's record, and the save keeps the commands handed in.

**The staff passed in.** `sim_core::command::Staff` is one method, `decide(&self, view) -> StaffDecisions`, taking `&self` so that a staff asked twice on one view answers the same. `StaffDecisions` is orders (each with the intent it serves and its reason), staff commands, and decisions to give no order (each a reason). `Staffs` holds a staff for each side or none. `World::resolve_day` resolves a day with none; `resolve_day_staffed(staffs)` and `resolve_day_watched(staffs, watch)` with them. `Scenario::next_day` has the same three. `World::submit_staff` hands in a side's decisions without a staff being asked: what a replay gives back, taken in at step 4 as if a staff had given it. After each day `World::staff_handed_in` gives what each side's staff handed in, for a runner to save; it is not state.

**The order of a day** is in `resolve_day_watched`: (1) the commander's commands; (2) the place of the override, built in a later step; (3) forces back from the yard, then refuelling down to the reserves; (4) each staff asked on its view as it then stands; (5) the staff's commands, then all orders, forming orders first and then by unit, a staff's order after the commander's to the same unit; (6) the rest, as before. With no command and no staff, steps 1, 4 and 5's first part do nothing, and the day is the day it was.

**A staff's order** is refused `OutsideForcesLimit` unless its intent is in force with a forces limit and its unit is inside it: a task force listed, or one whose every ship is in the pool; a base order to form a force at the intent's source from ships in the pool. Air groups and land units are in no forces limit in step 1. The check binds orders a staff hands in, not orders a scenario's script gives with the issuer `staff` (scenario 39): such an order is a scenario's statement, not a staff's decision, and is not checked against a forces limit (the architect's ruling, Nov 15, 2026). After each staff order its reason is recorded, `StaffReason`, citing the order and its acceptance or refusal; a decision to give no order is a `StaffReason` with no order. A reason has the spec's five parts (serves: intent, plan and amendment; evidence, each with its day; binding; worked; would change). Its number is the side's number of that event, so `why` on a reason is the event it is, and it resolves after its intent is replaced.

**The saved game** (version 3) keeps, beside the orders handed in, the commander's commands handed in by day and each side's staff hand-in by day, whole: orders with intent and reason, commands, decisions to give no order. A replay submits them again and does not run a staff. `replay --check-staff` runs the doctrine's staff at each day through `CheckedStaff`, which asks the real staff, keeps the first difference from the saved hand-in and hands in the saved one, so the replay is the saved game whatever the staff now does; it reports the first difference or that there is none. The save records `doctrine_version` beside the rules label; it is checked when a game is resumed (`play --game`), and not on a replay. With no doctrine yet the version and the staff a runner passes come from one place in `sim-cli`, `src/doctrine.rs` (version `"none"`, no staff for either side), which the second team replaces with `sim-staff`'s.

**A scenario's commands** are `[[command]]` tables with `day`, `side` and `kind`, taken in as the commander's on their day. Two `set_intent` of one day naming the same new number: the first sets it, the second changes it, in the order handed in; a number withdrawn is refused `IntentNumberNotUsable`.

**Days of supply** is `SideView::days_of_supply(hex)`, `None` where the side holds no base in the hex; it gives the stock of the side's bases there, the planned use in its five parts (men ashore, men due, aircraft upkeep, repairs, sorties) and the figure, `None` where the use is 0. A force's men are due on the day its passage to the hex, forecast from the view as the measuring tool forecasts it, ends; they count from that day until they are ashore. It is in the agent's list of bases and the briefing.

**Decision 0111's open point.** `World::check_order_queued(side, order, queued)` is the check a seat's order meets when handed in beside the other orders it has queued for the day. An order to a task force that does not exist yet, where a forming order is queued, is checked on a copy of the world with the queued forming orders taken in in intake's order, so that the force has its number and its ships; any other order is checked as `check_order` checks it. The session uses it for a player. A staff's orders meet no check before intake, and intake takes forming orders first.

**Versions.** The state layout rises to 24. The rules label is not raised by this build: a day with no command is the day it was (it is 7 after decision 0109, merged alongside). The saved game is version 3.

## As built (step 1, the staff)

Written by the team lead on Nov 15, 2026, before the code, as the design the build follows; corrected where the build differed. It is the second of step 1's two pull requests: the crate `sim-staff`, the doctrine for keeping a base supplied, the reasons of staff commands, the projection of days of supply and the tools of step 1.

**The crate.** `sim-staff` depends on `sim-core` only, forbids floating point as `sim-core` does, and every function takes a `SideView` (with plain numbers beside it) and nothing else; a test reads its source, as `sim-agent`'s does. It holds `DOCTRINE_VERSION` (`"supply-1"`), `SupplyStaff` (the `Staff`), `staffs()` (a `SupplyStaff` for each side) and `as_staffs`. `sim-cli/src/doctrine.rs` is removed and every runner takes its staffs from `sim_staff::staffs()`. Both sides always have a staff: a staff whose side has no intent in force and no commitment to free hands in nothing, so every existing game is the game it was, and its save differs only in the doctrine version.

**What `sim-core` gains** (no state, no hash, no rules label): a staff command is handed in with its reason (`StaffCommand { kind, reason }`; `StaffDecisions::commands` holds them), and intake records the reason as a `StaffReason` with no order, citing the command's `CommandTaken` or `CommandRefused` (differences row 43). `Staff` gains `propose(&self, view, intent) -> Option<Proposal>`, none by default: a proposal is worked out and stored nowhere (decision 5); its types (`Proposal`, `ProposedRun`, `SupplyProjection`, `ProjectedReading`, `Shortfall`) are in `command.rs`. `World::at_staff_step()` is a copy of the world with steps 1 to 3 of the coming day taken (the commands handed in, the forces back from the yard, refuelling): the view a staff will be asked on, for `staff_orders`, `propose` and the check of a command a seat hands in. `SideView` gains `next_task_force_id()` (ruling 7) and `men_due_by_day(hex)` (each force due in the hex with its due day and its men's charge, by the rule `days_of_supply` already uses), so the projection counts what is on its way by the engine's own rule.

**The projection** (decision 35). Today is the view's day plus one. The reading of today is the view's stock; the reading of day d + 1 is the reading of day d, less the planned use of day d, plus what lands on day d, and not below 0. The planned use of a coming day is today's, with the men due counted from their due day and the aircraft of a scheduled arrival from its day. What lands: for each run of an approved plan of the side to that hex not yet finished, its load while it has not sailed, else the supply aboard its ships; it lands on the run's arrival day plus one, or today if that is past. The figure of a reading is its stock over its use. The report gives today's figure and the first day the projection falls below the floor.

**The doctrine,** each day, for each supply intent in force, in priority order (ranked first, then by number):
1. No forces limit, a source or a destination not the side's, a plan ended: no order, a reason. A plan whose pool has no ship afloat is marked failed.
2. A draft with leave to prepare is approved; a plan preparing with leave to launch is released, the same day. Without the leave, a no-order reason says what waits.
3. Each run not finished is carried on: on the first day its load is complete (the supply aboard is its load, or the load order has ended), and the plan is released, a route to the destination and back, acting at the destination, and an unload order; if its sail day has passed, `time_run` and its commitment renewed first. A run finished is one whose ships are back at the source, stopped and empty, after its sail day.
4. If no run of the plan is loading, a run is needed when, without a new one, the reading of the day after a run ordered today would arrive is below the floor's stock (ruling 4). Its ships are the pool's free ships in number order (a ship in a force brings the whole force) until their room covers the load; a ship is free when no unfinished run holds it, no commitment covers today, and it is stopped at the source. Its load (ruling 5): the floor's stock on the arrival day, less the reading of that day, not below 0, plus the planned use of the interval's days from the arrival day; the interval is the intent's, or 15 days where the commander set none. It is cut to the ships' room and to the source's stock above its reserve and above the loads still to be taken by runs loading there. Its days (ruling 6): loading `ceil(hours / 24)` days, at least 1, at the source's berth rate, or its lighter rate with no berth; the passage each way `ceil(hours at cruise / 24)` days; unloading as loading, at the destination's rate. Load day today; sail day after the loading days; arrival the sail day plus the passage less one; free the day after the ship is back (ruling 8). Its route: the engine's sea path out and back; where an area limit is set and the path leaves it, waypoints are added along the shortest path inside the area until the engine's path between them stays in; none, no route. Every ship needs fuel for the round trip at cruise. On the day: `add_run`, `commit_ships` from today to the day before it is free; a forming order where its ships lie in port in no force (the new force's number from the view), a join where they lie in more than one force; and `load_to` at the source, the level being what is aboard plus the load.
5. When it cannot, it says which binds (no ship free until a day, the source's stock, no route in the area, a ship without the fuel, the use more than the pool can carry), the shortfall in days at the reading it checked, and the day the source falls to its reserve at the present rate (its stock above the reserve over the destination's and its own planned use). With less stock than the load it sends what it can and says how many days it brings.
6. A day it gives an intent no order has one reason: what the runs are doing, and the reading the next run waits for and the day it is expected to be ordered.
7. An ended plan's ships committed from today on are freed (`free_ships`).

Every order, command and no-order reason has the spec's five parts; "worked" is the arithmetic in tons, days and hours. Units are written as the engine writes them (`ship:1`, `task_force:1000001`, `base:2`), so a tool shows them in the side's own numbers.

**A proposal** for an intent is what the staff would do on the coming day as if the plan were released, and the runs the same rules give over the next 30 days on the projection, with today's projection and what binds. It displaces nothing in step 1 (one intent).

**The tools,** in `sim-agent`, for a seat whose side has a staff, and through the MCP server and `play`: `command_state` (intents, limits, reserves, leaves, plans and their runs and commitments, the commands queued, and each intent's figure and first day below), `plans`, `set_intent` (offering the side's next free number), `withdraw_intent`, `set_limit`, `set_leave`, `propose`, `approve`, `cancel_plan`, `staff_orders` (the staff's decisions for the coming day, asked on `World::at_staff_step()`), `why` (a `StaffReason` by its event number, whole, with the order or command it is for). A command is queued for the coming day like an order, and checked at once by taking it into `at_staff_step()`; its answer is queued or refused with the engine's reason. Ships and forces are written in the side's numbers. The interface's version rises to 4.

**Not built in step 1:** holds and overrides, interrupts, `run`, `report`, `preview`, cancelling's fallback beyond a route that already ends at the source, a second intent's competition for ships and stock, escort and cover.
