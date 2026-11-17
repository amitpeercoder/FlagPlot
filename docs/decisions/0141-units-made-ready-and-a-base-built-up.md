# 0141: Units made ready, and a base built up: step 2 of stage 3

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3, that the designer confirmed the same day (decision 0139, step 2 of its eight). It writes two tasks and their doctrines over rules already built (decisions 0095, 0097, 0098, 0099 and 0103), with the hand calculation. Nothing is built. It is built after decision 0140.

Summary: Step 2 of stage 3: two tasks a commander may state and that "take a place" will form: "ready a landing", by which the staff gives a force its landing order and roster and sets each unit to train or to rehearse by its experience, its risk and the date, and says when the landing is ready; and "build up a base", by which it sets the working units, and any fighting unit it is given, to the jobs wanted, in order, and says how many days each will take

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Not built

## Why

The rules by which a landing is prepared are built: a force gains loading for each day in port with its roster, a unit gains rehearsal for each day it stands ready on that roster, a raw unit gains experience by days of training, and working units build a field, a port and defences. Each is set by an order a scenario gives by hand. The spec asks that nobody give a `set_activity` order by hand ("The operation", step 2). These two doctrines give them.

They are the first phase of "take a place" (decision 0139, "An operation is a tree with phases"). They are written first, as tasks a commander may state himself, as cover from the air was before "hold a place".

## Ready a landing

1. **A task:** it names a task force, the coast hex of the landing, and, in its forces limit, the land units that are to land. It is addressed to the task force. It may have a date: the morning by which it is to be ready.
2. **The force's order.** The staff gives the force `prepare_landing` for the hex with the task's units as its roster, where the force's preparation differs.
3. **Each unit, each morning,** is set to train or to stand ready, which on a roster is to rehearse:

| The risk | A unit below its service's standard of experience |
| --- | --- |
| Preserve the force | Trains until it is at the standard, then rehearses, whatever the date |
| Balanced | Trains until it is at the standard, or until the days left before the date are no more than the days its rehearsal still needs; then rehearses. With no date it trains to the standard first |
| Press | Does not train. It rehearses from the first day |

A unit at or above the standard rehearses. A unit that cannot train (it has men aboard, or it is at no base of its side) rehearses. The days a rehearsal needs are what is left to 100 over the day's gain for that unit, rounded up.
4. **A working unit is given nothing by this task.**
5. **When it is ready.** The task is ready on the first morning that:

| The risk | The force's loading | Each unit's rehearsal | Each unit's experience |
| --- | --- | --- | --- |
| Preserve the force | 100 | 100 | At the standard |
| Balanced | 100 | 100 | Any |
| Press | 50 | 50 | Any |

The staff hands in a notice that it is ready, once. Step 3 of this stage asks leave to launch on it.
6. **A date it cannot make.** On the first morning the staff can see that the task will not be ready by its date, it hands in a notice with the days it will be late and what binds: the loading, a unit's rehearsal or a unit's training.
7. **Reasons** carry the numbers: each unit's experience against the standard, its days of training counted against the days a level takes, its rehearsal and the days it still needs, the force's loading, and the days to the date.

## Build up a base

8. **A task:** it names a base and what is wanted there, in order: an airfield; its damage mended; defences to a stated number of points; the port worked; aircraft repair. It is addressed to the base. With no forces limit it may use the working units at the base. A forces limit may add fighting units, which the staff then puts to work; it puts no fighting unit to work that it was not given.
9. **Each morning** every unit the task may use, that is at the base and not held, is given the job of the first want not yet met, where its job differs. All work the same job.
10. **A want is met** when the base has an airfield; when its airfield has no damage; when its defences are at the number or above. The port and aircraft repair are never met: they are worked while they are the first want left.
11. **With every want met** the staff gives no order, says so, and hands in a notice that the task is met, once. A want that stops being met, a field damaged again, is worked again.
12. **Reasons** give the work still to do on the want, the work the units make in a day, and the days that comes to.

## What both keep

13. **A unit serves one task.** Where two tasks may use a unit, the higher in priority has it, and the other's reason says so.
14. **A hold is honoured,** and a commander's order to a unit a task may use is an override that holds it, as for an air group (decision 0115, 3).
15. **Nothing is ordered without leave to prepare.**
16. **A unit before the day it may be used from** is left out (decision 0140).

## What it leaves for later steps

- **Choosing which units land,** by what each is good at: step 3.
- **Shipping gathered for the landing:** step 3.
- **What a base needs for an operation,** worked out by the staff and not stated: step 3 forms this task with its wants.
- **Training when no landing is planned.**

## Rule card

- **Reads:** a unit's experience, days of training, rehearsal, activity, job, where its men are; a force's preparation; a base's airfield, damage and defences; the ground figures; the risk and the date of the task.
- **Writes:** two kinds of intent and what a "build up" task wants, in the command state; nothing else.
- **Reuses:** the orders `prepare_landing` and `set_activity`, the day's activity, and every rule of decisions 0095 to 0099 and 0103 as they are. A forces limit gains land units.

## What it costs

- **State:** the two kinds of intent, the wants of a build-up, land units in a forces limit, the date of a task, in the hash and named. The state layout and saved game versions rise; the doctrine version rises. The rules label does not.
- **The staff gives orders to land units for the first time.**

## The hand calculation

`tests/scenario_66_units_made_ready.rs`.
