# 0111: Three rules the staff needs: a load up to a weight, a reserve at a base, and forces formed and joined

Status: settled by the architect on Nov 14, 2026, inside the plan for step 3, stage 1, that the designer confirmed (decision 0107, items 13 to 15, with his answers on reserves and on refuelling). It writes those three rules out with their rule cards. The fourth thing decision 0107 listed, a scenario stating a side's commands, comes with the commands themselves in the next part of stage 1. Not built.

Summary: Three rules the staff needs, built before it: a load order may name a weight to have aboard; a base has a supply reserve and a fuel reserve, which refuelling and a load ordered by the staff never go below; and ships at a base may be formed into a force, two forces there joined, and a force stood down into port

Standing: Settled by the architect, Nov 14, 2026, inside decision 0107. Not built

## Why these, and why first

The spec says a limit the orders cannot honour is not a limit. Today a load order fills a force from the stock and names no weight, a force in port refuels from whatever the base holds, and nothing forms a force from ships lying in port. A staff told to keep a reserve and to send convoys could do neither. These three rules are small, stand without the staff, and are proved by a hand calculation of their own, so they are built before the command state and the staff that will use them.

A scenario with no weight on a load, no reserve and none of the new orders gives exactly what it gave before.

## A load up to a weight

1. **A load order may name a weight.** For a load of supply or of fuel as cargo, the order may give the tons the force is to have aboard of that cargo. With no weight the order is as it is today: fill the room.
2. **It is the force's weight, not a ship's, and it is a level, not an addition.** The force loads until the cargo aboard all its ships is the weight. A force that already has that much aboard loads nothing. So no new state is kept: the order and what is aboard say what is left.
3. **The ships are filled in ship order.** Each takes what its room allows until the force's weight is aboard. The cargo rule's berths, rates and hours are as they are.
4. **The load is complete** when the weight is aboard, or no ship has room, or the stock it may draw on is gone (decision 7 says what that stock is). A force that acts and steams on (decision 0096) steams on then.
5. **It does not apply to men.** A load with a unit takes the unit, as now.

## A reserve at a base

6. **A base has a supply reserve and a fuel reserve,** each a number of tons, 0 unless set. A scenario may state them for a base. From the next part of stage 1 the reserve limit of decision 0107 sets them.
7. **Refuelling stops at the fuel reserve.** A force stopped at a base of its side refuels at the start of the day, as now, but takes the base's fuel down to its reserve and no further. It holds for every force, the commander's own included: refuelling is a rule and not an order. Where a side has more than one base at the hex, a force draws on them in base order, each down to its reserve.
8. **A load ordered by the staff never takes a stock below its reserve.** The cargo rule caps it as it is worked: at each moment such a load may take only what the base holds above its reserve. So whatever the night took from the stock, the staff's own load does not go below the reserve, as the spec requires. For such a load the stock is gone, in decision 4, when the base is at its reserve.
9. **A load ordered by the commander is not capped.** It may take a stock to nothing, as today (decision 0107, question 2). When such an order is taken in and the base holds less above its reserve than the order asks, the answer says so: the order is accepted, with the tons it may take below the reserve.
10. **Nothing else heeds a reserve.** The men's supply, aircraft upkeep, repairs, sorties, bombs and shells take a stock below it. A reserve is a line the planner keeps, not a second store.
11. **A reserve changes sides with its base** and is kept by the side that takes it, as a number of no meaning to it until it sets its own.

## Forces formed, joined and stood down

All three are orders to things at a base of the side, given for a day and carried out when the day's orders are taken in.

12. **Forming.** Ships of the side that lie in port at one base, in no force, may be formed into a task force. It takes the side's next force number. It is stopped at the base, with no mission, at cruise speed, with the engagement settings a force has when a scenario gives it none, and with that base as its home port. An air group aboard a ship goes with it.
13. **Joining.** A force stopped at a base may be joined to another force of the side stopped at the same base. The receiving force keeps its number, orders, mission, speed and settings, and takes the other's ships with everything aboard them: cargo, men, air groups, fuel, damage. The other force's orders end and it ceases to be; its number is not used again. Where either has a landing in preparation, the rule decision 0095 wrote for joining holds: the lower of the two loadings, and the rosters together.
14. **Standing down.** A force stopped at a base of its side may be stood down. Its ships then lie in port at that base in no force, with what they carry. Its orders end and it ceases to be. A ship in port in no force does not move, refuel, work cargo or take repair, as now.
15. **Each is refused** if the force or a ship is not the side's; at sea, or at a base that is not the side's; for a force away at the yard; for a force that is under way or has a route to follow; and, for a join, if the two are not at the same base. A ship named twice, or already in a force, is refused for forming.
16. **Detaching ships is as it is.** The order that exists is not changed.
17. **A side sees these whole for its own forces.** The enemy is told nothing: a force it holds as a contact is a contact still, and what the contact is of was never told.

## Rule cards

| Rule | Reads | Writes |
| --- | --- | --- |
| A load up to a weight | The load order's weight; the cargo aboard each ship of the force and its room; the base's stock | The cargo aboard each ship; the base's stock; the load's completion |
| A reserve | A base's reserves; the origin of a load order; the fuel a force needs | A base's stock and fuel; a ship's fuel and cargo; the answer to an order |
| Forming, joining, standing down | The ships in port at a base and their side; a force's status, place and orders; a landing in preparation | The forces and which ships are in them; a force's orders; a preparation |

## What it leaves out

- **Refuelling that takes time or a berth** (decision 0107, question 1): a decision of its own, before stage 3.
- **A weight on unloading.** A force unloads all it carries, as now.
- **A reserve of rounds in a force,** which the spec's reserve limit also names: it comes with the fighting doctrine of stage 2.
- **Joining at sea, and a ship changing force at sea.**

## What it costs

- **New state:** a base's two reserves, in the hash and named beside it. The state version rises. The rules label rises too: a load with a weight and the new orders are things a game played under the old rules could not hold.
- **The load order gains a field, and three orders are new,** in scenario files, the agent's orders and their schemas.
- **New events** for a force formed, joined and stood down, and for a refuelling or a load stopped at a reserve, each with its row in `side_events.rs`.
- **The rulebook:** chapter 2 (the base, cargo orders, refuelling, the task force) and chapter 1's tables of orders and refusals, and the glossary, in the same pull request.

## The hand calculation

`tests/scenario_39_load_reserve_and_forces.rs`: a refuelling stopped at a reserve with a second force refuelling the same morning; two forces loading from one base, one by the commander's order and one by the staff's, without both counting the same stock; a load shared between two small ships; and forces formed, joined and stood down.
