# 0107: Step 3, stage 1: the command contract, and a staff that keeps a base supplied

Status: a plan by the architect, written on Nov 14, 2026, for the designer's approval. `docs/step3-spec.md` settles what the contract is; this record says how stage 1 builds it, what each part is in the state and in the tools, the three small rules it needs, and the cases it must pass. Needs review: the plan as a whole, and the three questions at the end. Nothing is built. The hand calculations it names are written by the architect after approval and before any code.

## What stage 1 is

The spec's row: the command contract, whole, and a staff for one area intent, keep a base supplied from a pool of shipping. It shows the loop from end to end: propose, explain, approve, run a quiet week, a shortage that stops the clock, a hold and its release, a save in the middle.

It has no fighting doctrine, no landing and no trial. Those are stages 2, 3 and 4. The enemy in stage 1's cases is the script, or nobody.

## Where each part lives

1. **What the commander says is state.** Intents, limits, holds, approved plans with their amendments, commitments, and interrupts set and acknowledged are part of each side's state in `sim-core`, in a module of their own. They are hashed and named beside the hash, saved, and replayed (the spec, "What is saved"). They are changed only by a **command**: a new kind of thing a seat submits, beside orders, taken in at the start of a day. A command is checked at intake and accepted or refused with a reason, as an order is.
2. **What the staff decides is code outside `sim-core`,** in a new crate, `sim-staff`. It depends on `sim-core` only. Every function in it takes a `SideView` and nothing else, as `sim-agent`'s do, and a test reads its source to check. It returns orders, each with its reason, and proposals.
3. **The view carries the side's own command state.** `SideView` gains the side's intents, limits, holds, plans and commitments, whole, and nothing of the enemy's. Whether a side has a staff, and what its intents are, is never told to the other side.
4. **The staff's orders are orders.** Before a day is resolved, the session asks the staff of each seat that has one for the day's orders and submits them with the staff as their origin. They are checked like any order. A replay runs the staff again from the saved state and gets the same orders; the saved game's rules label covers the doctrine, so a save made under another doctrine is refused (decision 0100).
5. **Proposals and reports are worked out, not stored.** They are a function of the state and are worked out again when asked for. An estimate a commander was shown is kept with the session, outside the state.

## The three rules stage 1 needs

The spec says a limit the orders cannot honour is not a limit, and names two orders. A third is forming convoys.

6. **A load of a stated weight.** A load order may name the tons of supply or of fuel to take. It takes no more than that, than the ship's room, and than the stock. With no weight it is as it is today.
7. **A reserve at a base, and refuelling that stops at it.** A base has a fuel reserve and a supply reserve, set by the reserve limit, 0 unless set. Refuelling in port takes a base's fuel down to its reserve and no further, in the order ships refuel today. The staff plans no load and no use that takes a stock below its reserve. An event can: a raid, the men's supply, aircraft upkeep. *Question 1 asks whether refuelling should also start to cost time here.*
8. **Forming and joining forces by order.** At a base, ships in port that are in no force may be formed into a task force; ships may be detached from a force that is stopped there into a new one; and two forces stopped at the same base may be joined. Joining uses the functions decision 0095 wrote for it and left uncalled. Each is refused at sea.

Each is a rule change with its lines in the rulebook, its rule card in the record that builds it, and a hand calculation.

## The contract in stage 1

9. **One kind of intent:** keep a base at so many days of supply. It names the base, the days, and optionally the same for fuel. The other kinds come with the stages that give them a doctrine.
10. **All six limits exist,** since every later stage leans on them, though stage 1's doctrine uses four:

   | Limit | In stage 1 |
   | --- | --- |
   | Forces | The ships, forces and bases the staff may use for an intent: the pool, and the base it draws stock from. Used |
   | Area | The hexes its ships may enter. Used: a route that would leave it is not planned |
   | Priority | The order of the side's intents. Used, between two supply intents |
   | Protected | A force that stays on its task. Used |
   | Reserve | Decision 7. Used |
   | Risk | Stored and shown. Stage 1's doctrine does not read it: nothing in it is risked. Stage 2 gives it numbers |

11. **A plan** for a supply intent is a schedule of convoy runs: for each, the ships, the load by weight, the base it loads at, the route, the day it sails, the day it is due and the day the ships are free again. It has one phase. Its life is the spec's: draft, preparing, released, complete, failed, cancelled, with a baseline and numbered amendments.
12. **Leave to prepare and leave to launch** are two settings of a seat, with the spec's defaults. For a supply plan, preparing is forming the convoy and loading it; released, it sails.
13. **Commitments** are of ships by day and of stock by weight and day. A ship is free when it can be where its next run starts: the passage back, the unloading and the refuelling are inside the commitment. Two plans never count the same ton or the same ship.
14. **Holds** on a setting, a unit and a base, as the spec has them, with the three cases tested apart.
15. **Interrupts** a commander can set in stage 1: a base's days of supply, or of fuel, falling below a number; a plan that can no longer be met; a ship of the side lost; a proposal waiting for him. They fire on the change, as the spec has it.
16. **A run** resolves up to a number of days and stops early for an interrupt, with every reason of the day listed together.
17. **Reasons.** Every order the staff gives or withholds has the five parts of the spec. In stage 1 "worked" is always arithmetic that can be read: stock, use, passage and capacity.

## Days of supply

18. **Days of supply is a base's stock over its planned use for a day.** Planned use is what the base's side expects it to pay on a day like those ahead:
   - the men's supply for the men of its side ashore in its hex, at their weight (decisions 0066 and 0098);
   - the daily upkeep of the aircraft at it;
   - the supply of the sorties its groups fly on the missions they are set, counted as if every aircraft able to fly flew its mission each day it could.
19. **Men and aircraft that are on their way count from the day they are due,** by the plans the staff holds. So a base about to receive a regiment is short before the regiment lands.
20. **A base with no planned use has no figure,** and the tools say so in words. It is never shown as infinite or as zero.
21. **Standing aircraft down does not raise it.** Planned use counts the missions the commander's intents call for, not the ones flying today. Where the staff rests a group, the use it would have had is still counted.
22. **A base that changes side** has a figure from the day it is the side's, and none before; a base lost has none after.

The spec asks for four hand-worked cases: a unit arriving, a surge of sorties, no consumption at all, and a base taken and lost. They are the architect's first hand calculation for this stage.

## The doctrine: keep a base supplied

23. **Each day, for each supply intent in priority order,** the staff works out the base's stock day by day over the days ahead, from planned use and the runs already committed.
24. **It sends a run when one is needed and can be made.** A run is needed if, on the day a run sailing now would land, the stock would be below the intent's days of supply. It can be made if the pool has ships free, the source base has stock above its reserve, and a route exists inside the area limit.
25. **A run's size** is the smallest of: what brings the base to its days of supply on the day it lands, with the use of the days until the next run could come; the room of the ships free; and the source's stock above its reserve.
26. **Its ships** are the fewest that carry the load, the lowest-numbered first. **Its route** is the shortest sea route inside the area. **Its speed** is cruise. It unloads on arrival and comes back, by the order of decision 0096.
27. **An order stands until its reason changes.** A run under way is not re-planned on a day when the same run would be planned again.
28. **When it cannot meet an intent** it says which of the four things binds: no ship free until a day, the source's stock, no route in the area, or the base using faster than the pool can carry. It reports the shortfall in days of supply and goes on with what it can do. At the source, it reports the day the source itself runs dry at the present rate.
29. **Escort, cover and what the enemy may do to a convoy are not in stage 1.** A convoy lost to the enemy is a ship lost: an interrupt, and a new plan.

## The tools

30. **A seat with a staff has these tools,** beside the ones that exist. Each takes and returns JSON with a schema, and the agent interface's version rises:

   | Tool | What it does |
   | --- | --- |
   | `set_intent`, `withdraw_intent` | State or remove an intent |
   | `set_limit` | Set forces, area, priority, a protected commitment, a reserve or risk |
   | `hold`, `release` | Hold a setting, a unit or a base; release it |
   | `plans` | The side's plans, each with its state, baseline, amendments and what it is doing now |
   | `propose` | The staff's draft for an intent: the plan, the reasons, what binds, what it displaces |
   | `preview` | What an amendment or a changed limit would cost, tied to a revision. Changes nothing |
   | `approve` | Prepare a draft, release a plan that is preparing, apply a previewed amendment, cancel |
   | `set_interrupt`, `acknowledge` | Set a condition; acknowledge one that fired |
   | `run` | Resolve up to so many days, stopping for an interrupt |
   | `report` | The command report: what changed, what is open, each item leading to its reasons |
   | `why` | The reason for a staff decision, given or withheld |

31. **The command report keeps one identity for a matter that lasts.** A convoy that is late for a week is one open matter with a history, not seven.
32. **Unit orders still work.** A commander may give any order he could before. An order to a unit the staff is using is an override: the unit is held from then, by the spec's rule, until he releases it.

## Built in four steps

The spec's order. Each ends with something that runs and has its own pull request.

| Step | What is built | What runs at the end |
| --- | --- | --- |
| 1 | The three rules. Command state, commands, the view. Days of supply. The doctrine for one intent. `set_intent`, `set_limit`, `propose`, `approve`, `why` | One convoy plan, proposed, explained, approved and carried out, on a placeholder map |
| 2 | Priority, commitments of ships and of stock, a second intent, displaced demand reported | Two bases and one pool: the higher intent is met, the lower's shortfall is stated with its cause, and a changed priority moves the ships from the next day |
| 3 | Holds and release, overrides, amendments with their previews, the two leaves | A hold, a changed intent, a week, a release; a plan brought ready to launch that has not sailed |
| 4 | Interrupts, `run`, the command report, saving and resuming | A quiet week in one call; a shortage that stops the clock; a save in the middle, resumed, matching play that was not interrupted |

## What it must pass

Each case below gets its numbers in the hand calculation or the test plan of its step, written before the code.

- **Hand calculations:** days of supply (the four cases); one convoy schedule from stock, use, passage and unloading times; an allocation of one pool between two bases; a load of a stated weight and a refuelling stopped at a reserve, with a second force refuelling the same morning.
- **The contract's cases of the spec** that stage 1 can reach: two bases and one convoy; a hold, a changed intent, a week, a release; a sunk supply ship; an objective that cannot be met; a plan cancelled with ships at sea; the save case.
- **Properties:** the staff never orders what it does not own or what is held, directly or through another unit; it never plans below a reserve or outside the area; every order given or withheld has a reason that resolves; the same inputs give the same orders; a staff with no intent gives no order.
- **No truth:** two worlds that differ only in what the side cannot see give the same orders, proposals, reasons, previews, interrupts and reports.
- **Against no orders:** over 30 days on the placeholder supply scenario, with the same start, a side with the staff keeps its base at or above its days of supply, less one run's load, on every day after the first run lands, and a side given no orders has run out by a day the hand calculation gives. Both are exact: the scenario has no enemy and no dice.
- **Speed:** a day's staff decisions for one side, and a quiet week in one call, on the machine and by the command of decision 0100, inside times the designer accepts once they are measured.

## What it costs

- **A new crate,** `sim-staff`, and a line for it in `docs/engineering/layout.md`.
- **New state** for each side's command state, in the hash; the hash version and the rules label rise with each step that adds to it.
- **Three rule changes** and the lines of the rulebook for them; a new chapter of the rulebook, "The staff", which stage 1 begins.
- **Eleven tools** with their schemas, and the MCP text that tells a player how to command by intent.
- **A placeholder scenario or two** for supply, and the hand calculations.

## What it leaves for later stages

- Every intent but supply; escort and cover; the risk level's numbers; the enemy played by the staff (stage 2).
- Landing, marching, units made ready, bases built up (stage 3).
- The trial, and with it the finding of stage 0 that a world built from one side's view holds no enemy the side has not seen (stage 4).
- The day from which a force may be used, which the staff setup of the campaign needs (stage 2).

## Questions for the designer

1. **Refuelling and time.** Decision 0102 puts "refuelling that costs time and a berth" before stage 3. Stage 1 already changes the refuelling rule, for reserves. I recommend doing both at once: a ship refuels through the port's rate as cargo does, taking hours and a berth, and stops at the reserve. It is one change to the rule instead of two, and the supply staff then plans with the real cost from its first day. The other choice is reserves only now, and the time cost as its own change before stage 3.
2. **A reserve and the commander's own orders.** The spec makes a reserve hard for the staff's planning. I recommend that it also binds automatic refuelling, as decision 7 has it, and does not bind an order the commander gives himself: his load order may take stock below his own reserve, and the tools say that it does. The other choice is that a reserve binds every order until he lowers it.
3. **The whole plan.** Whether stage 1 may be built as laid out here, in its four steps.
