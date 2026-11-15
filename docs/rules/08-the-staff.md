# The staff

This chapter holds what a side's commander says as state, the commands that change it, the staff that a side may have, the reasons the staff gives, and days of supply. It is written by step 1 of stage 1 of step 3: the engine's half (command state, commands, the staff passed in, days of supply) and the staff's half (the crate `sim-staff`, the doctrine for keeping a base supplied, the reasons of staff commands, the staff's projection of days of supply, and the tools a seat commands with), step 2 adds priority between intents, commitments of stock and displaced demand, and step 3 adds holds, the override of a hold by the commander's own order, ending a run, the supply a free ship has aboard, the fallback of a cancelled plan, previews and the tools `hold`, `release` and `preview`, and step 4 adds interrupts and their acknowledgement, the staff's notices, a run of days that stops for an interrupt, the command report and the tools `set_interrupt`, `acknowledge`, `run` and `report`. Step 2 of stage 2 adds the intent tree (an intent has a parent, a maker and an addressee, and the staff forms sub-intents), the end "hold a place", the task "cover a base from the air" with the air doctrine that serves them, air groups in a forces limit, a hold on a whole air group, limits read up the tree and the measure of a hold intent. Every side of every game has the staff; a side with no intent in force has a staff that does nothing, so a game with no command is played exactly as it was before this chapter. [0107, 0111, 0115, 0116, 0117]

No rule here draws a random number. Every quantity is a fixed-point number (chapter 1, Numbers and chance).

## What is built and what is not

Built: command state (intents with their parent, maker and addressee, six limits, two leaves, plans with their states, baselines, amendments and runs, commitments of ships, holds, and interrupts); seventeen commands and who may give each; the staff passed in to the function that resolves a day, and what it hands in; the check of a staff's orders against the holds and against a forces limit; reasons; days of supply; the saved game of version 3, `replay --check-staff` and the doctrine version; a scenario's commands; the doctrine for keeping a base supplied, with its projection of days of supply and its proposals; priority between intents, what is committed the same morning, the protected limit, commitments of stock as runs and displaced demand (step 2); holds on a ship, a task force and the search arc of an air group, the override of a hold by the commander's own order (step 2 of the day), ending a run, the runs the staff sets aside for a held ship, the supply aboard a free ship counted toward a run's load, the fallback of a cancelled plan, and where a run lands in the projection (step 3); the tools of steps 1 and 3, with previews; and the interrupts of the commander, with the standing ones and their acknowledgement, tested each morning, the staff's notices, and the tools of step 4 (`set_interrupt`, `acknowledge`, `run` and `report`); and, from step 2 of stage 2, the intent tree, limits read up it, air groups in a forces limit, a hold on a whole air group, the end "hold a place" with its measure, the doctrine for covering a base from the air, and the chain of a reason. [0107, rulings 10 to 16, 17 to 30 and 31 to 37; 0115, 0116, 0117]

Not built, and so not rules: a hold on a base (it waits for a stage whose doctrine acts on a base); the withdrawal of a numbered interrupt and any condition but a base's days of supply; a notice the staff hands in on a day it weighs no new run (Notices, below); escort and the recall of a run; a second source; an intent for fuel; and a phase of a plan (below, Plans). Ships covering a place, bombers and a strike intent, a task force or a land formation as the addressee of an intent, a commander stating a sub-intent under a parent of his choosing, the staff withdrawing a sub-intent, and supply runs under "hold a place" are not built either (Not built, below). [0107, rulings 17, 30 and 35; 0115, 0117]

## Command state

Each side has one command state, held by the world beside its belief and hashed after the beliefs, Allied side first (chapter 1, The state hash). It holds the side's intents with their limits, its two leaves, its plans, its commitments, its holds and its interrupts. It changes only by a command, by the test of its interrupts each morning (Interrupts, below), or by the override of a hold, which an order of the commander's makes (Holds and the override, below). The enemy has none of it, and a side is never told whether the other has a staff. [0107 items 1 and 6]

A side sees: its own command state, whole, through its view. It sees none of the other side's. [0107 item 6]

### Intents

An intent is what the commander wants. Three kinds are built, and each names a base of the side. **Keep a base supplied** names a base, a floor in days of supply (at least 1) and, optionally, an interval in days (at least 1) between shipments. The floor is the days of supply the base's hex is never to fall below. The interval is the days of use each run is to bring above the floor. Where the commander gives none, the interval is not set and the staff proposes one. **Hold a place** and **cover a base from the air** name a base and nothing else (Holding a place and Covering a base from the air, below). An intent has no date: it stands until it is withdrawn. [0107 items 17 and 22, question 5; 0115; 0117]

The commander names the intent's number when he sets it. It is a whole number from 1. A withdrawn intent keeps its number in the state and the number is never used again. An intent records the day it was first set, the plan that serves it, the day it was withdrawn, if it was, and, since the tree (The intent tree, below), its parent, its maker and its addressee. [0107, 0117]

An intent in force carries its limits as they stand. Setting an intent that is in force again replaces its kind and base (and a supply intent's floor and interval) together, gives it the addressee of its new kind, and leaves its limits, its parent and its maker as they are. [0107, 0117]

### The six limits

A limit says what the staff may use and where it may go. Five belong to an intent and one to a base. A limit that was never set on an intent is read up the tree: the intent has its parent's, and so on to the root (The intent tree, below). A limit set nowhere above it does not bind, with one exception that follows from what it is: an intent with no forces limit anywhere above it may use what its addressee commands, which for an intent given to the area is nothing, so it gives the staff no ship and no order for it; for one given to a base it is the side's air groups based there. With no area limit the sea is open. [0107; the architect's ruling, Nov 15, 2026; 0117 item 8]

| Limit | What it holds | What reads it |
| --- | --- | --- |
| Forces | The pool: a set of ships, a set of task forces, a set of air groups, and a source base. A supply intent's forces limit must name exactly one source (a `set_limit` of forces without one is refused as "No source"); the forces limit of another kind of intent need not | The check of a staff's order and of a staff command that names ships (below); the air doctrine, for the groups it may order |
| Area | A set of hexes the intent's forces may enter. It is not empty, and every hex is on the map. An area not set on it or above it does not bind: the sea is open | The doctrine's route |
| Priority | A rank, 1 first, or none. An intent with none, on it and above it, comes after every ranked intent, then by number | The order in which the doctrine serves intents (the rank read up the tree, for the air doctrine) |
| Protected | A set of ships the staff may not take from the intent | The doctrine: no other intent's plan takes a protected ship, free or not, whatever the ranks (The doctrine, below) |
| Risk | Preserve, balanced or press. Balanced where none is set on the intent or above it | The cover doctrine: the share of the patrol and the fatigue at which a group rests (Covering a base from the air, below) |
| Reserve | For a base, a supply reserve and a fuel reserve, in tons, neither below 0 | Refuelling, and a load the staff orders (chapter 2, Refuelling in port and Cargo orders) |

[0107 items 18 to 20, 0111, 0115 items 2 and 4, 0117 items 7 and 8]

A ship is in the pool of a forces limit if the limit lists it, or if it lies in a task force the limit lists, afloat in it or away from it at the yard. A task force is inside the limit if the limit lists it, or if it has at least one ship and every ship of it is a ship the limit lists. An air group is one an intent **may use** if it is one of the side's own and either the intent's forces limit, its own or the nearest above it, lists it, or no forces limit is set on it or above it and the group is at the base the intent is addressed to (an intent given to the area may use no group without a forces limit; a group aboard ship or in no base is not at a base). [0107, 0115 item 2, 0117 item 8]

The reserve limit names a base and no intent. It writes the base's own two reserves (chapter 2, Bases on hexes): one store, not a second. A scenario's base entry gives the same two figures (`supply_reserve_t` and `fuel_reserve_t`). It is taken in at the first step of the day, so it binds that morning's refuelling. [0111, 0107]

### The two leaves

A side's commander holds two leaves for his staff. Leave to prepare lets the staff itself move a draft plan to preparing. Leave to launch lets it move a plan that is preparing to released. The defaults are the spec's for a player: prepare given, launch withheld. Only the commander sets a leave, and he may set either on any day. [0107 item 21]

### Plans

A plan is what the staff does to serve one intent and the intents beneath it: one plan for a tree. It is drafted when a root intent's number is first set, and numbered in the side's own series, 1, 2, 3 and on, so a side's plan numbers depend on its own history only. A sub-intent has no plan of its own: it belongs to its root's, and forming it drafts nothing and is no amendment (The intent tree, below). A plan has one of six states:

| State | Means |
| --- | --- |
| Draft | Drafted, not yet approved |
| Preparing | Approved for preparation. It has a baseline |
| Released | Approved to run: it may be carried out |
| Complete | Ended: its intent was withdrawn while it was released, or the staff marked it complete |
| Failed | Ended: the staff marked it failed |
| Cancelled | Ended: the commander cancelled it, or its intent was withdrawn before it was released |

Preparing and released are together **approved**. Complete, failed and cancelled are **ended**: nothing moves an ended plan again. The moves are these, and no other:

- Draft to preparing, and preparing to released, by an `approve` command (below).
- A plan that is not ended to cancelled, by a `cancel_plan` command.
- An approved plan to complete or to failed, by a `mark_plan` command, which only the staff gives.
- A plan that is not ended, when its root intent is withdrawn: a released plan becomes complete and a draft or preparing one becomes cancelled. [0107 items 22 to 24]

Setting again an intent that is in force whose plan has ended drafts a new plan for it, with the next number, and the intent serves from then on. A plan that has ended with its intent still in force does not withdraw the intent. [0107]

Limit: decision 0107, item 23, says a plan's phase begins when it is released and ends when the intent is withdrawn. The code has no phase state. A plan's runs are tasks held in the plan, and its state is the one of the six above (differences.md row 42). [0107 item 23]

### The baseline and amendments

A plan's **baseline** is taken the first time it is approved: the day, the intent's kind and the intent's limits as they stand that day. It never changes. A plan that is not yet approved has none.

An **amendment** is a numbered change to a plan that is approved. While a plan is approved, a `set_intent` for its intent, and a `set_limit` of a forces, area, priority, protected or risk limit of its intent (or of any intent beneath it, which shares its plan), apply to the intent as they stand and are also kept in the plan as its next amendment, numbered 1 and up, with the day and the change. A reserve is no amendment, because it names a base and no intent. Withdrawing the intent is none either: it ends the plan. A change made while the plan is a draft, or after it has ended, changes the intent and is no amendment: a draft's change enters its baseline when it is approved. [0107 items 22 and 25]

What the staff may change alone in a plan is its runs and commitments, inside the limits. A change to the days, the base or a limit, and cancelling, are the commander's. [0107, question 3]

### Runs

A run is one convoy of a supply plan: a task inside the plan's one phase. It has a number in its plan (1 and up), its ships, its load in tons of supply (above 0), its source, which is the source base of its intent's forces limit on the day the run is added, and four days:

| Day | Means |
| --- | --- |
| Load | The day loading begins |
| Sail | The day the force sails |
| Arrive | The day it is due at the destination |
| Free | The day its ships are free: the morning after the day they have finished fuelling at the source on their return (by the hours of the fuelling, below, Its days) |

The days are in this order or equal. A run added has its load day no earlier than the day the command is taken in, and a run timed again has its free day no earlier than that day. A run does nothing by itself. The orders that carry it out, form the force, load it to a weight, send it and unload it, are the staff's to give on the days it names. [0107 items 26 and 27, rulings 7 and 8]

A run also records **the day it ended**, none while it has not: the day an `end_run` command was taken in (below). An ended run is ended for good: nothing takes it up again. It holds no ship, is pending no load and is counted by no reading, and so is a run of a plan that has ended: the staff reads both as finished. What is aboard its ships stays aboard, and ending a run frees no commitment: that is a command of its own. [0107, rulings 20 and 29]

### Commitments

A commitment gives one ship to one plan, and to one of its runs if named, from one day to another, both days included. A ship holds at most one commitment on any day: a commitment that overlaps another of the same ship, whatever plan it is for, is refused. The staff commits ships and frees them. Freeing a ship ends every commitment it has to the plan. No rule of the day reads a commitment: it is the staff's record, and the check of the next commitment reads it. A commitment to a plan that has ended holds no ship from the staff's reading (The doctrine, below).

A **commitment of stock** is no record of its own: it is the run. A ton at a run's source is committed to its plan from the day the run is added until it is aboard: the run's load less the supply aboard its ships while it loads there. The stock at a source that a new run may draw on is the stock, less the reserve, less those tons for the runs of every plan of the side. [0107 item 27, ruling 12]

## Commands

A command is a change to the side's command state, handed in beside orders. It is checked against the side's own units and its command state, and it is accepted or refused with a reason, as an order is. A refused command changes nothing but the log. [0107 items 1 and 2]

### Who may give each command

| Command | Given by | What it does |
| --- | --- | --- |
| `set_intent` | Commander | Sets an intent (a root), or changes one in force that the commander made |
| `withdraw_intent` | Commander | Withdraws an intent in force that the commander made, everything beneath it, and ends its plan |
| `set_limit` | Commander | Sets one limit, on any intent |
| `set_leave` | Commander | Gives or withholds leave to prepare, or leave to launch |
| `approve` | Commander, or the staff with leave | Moves a draft to preparing, or a preparing plan to released |
| `cancel_plan` | Commander | Cancels a plan that has not ended |
| `hold` | Commander | Holds a ship, a task force, a whole air group or one setting of an air group (its search arc) from the staff |
| `release` | Commander | Lifts a hold, whatever made it |
| `set_interrupt` | Commander | Sets a numbered interrupt, or sets it again, or switches a standing kind off or on (Interrupts, below) |
| `acknowledge` | Commander | Marks the last firing of an interrupt seen |
| `commit_ships` | Staff | Commits ships of the intent's pool to an approved plan from one day to another |
| `free_ships` | Staff | Ends the commitments of ships to a plan |
| `add_run` | Staff | Adds a run to an approved plan, with the plan's next run number |
| `time_run` | Staff | Gives a run new days |
| `mark_plan` | Staff | Marks an approved plan complete or failed |
| `end_run` | Staff | Ends a run of an approved plan for good (Holds and the override, below; The doctrine) |
| `form_intent` | Staff | Forms a sub-intent under an intent in force whose plan is released (The intent tree, below) |

A command from the other issuer is refused as not yours. A staff may give `approve` only where the commander has given it the leave for the move: leave to prepare for a draft, leave to launch for a plan that is preparing. The commander's `approve` needs no leave. [0107 items 4 and 21]

The commander's commands are handed in with the orders for a day and taken in at its first step, the Allied side's first, each side's in the order handed in, so a hold or a release given for a day binds from that morning, before the staff is asked. The staff's are taken in after the staff is asked, in the order handed in, before any order. [0107, The order of a day; rulings 17 and 23]

### Every reason a command is refused

The reasons for one command are tried in the order written. The first that applies is the one recorded. A unit or a base that is not the side's, and one that does not exist, are refused alike, so a refusal never shows that an enemy unit exists. A ship is the side's own only if it is not sunk.

| Command | Reason | When |
| --- | --- | --- |
| Any | Not yours | The issuer may not give this kind of command. Tried first |
| `set_intent` | Intent number not usable | The number is 0, or it is the number of an intent that was withdrawn |
| `set_intent` | Made by the staff | The number is that of an intent in force that the staff formed |
| `set_intent` | Not your base | The base is not one of the side's |
| `set_intent` | No days | The floor is 0, or an interval is given and is 0 |
| `withdraw_intent` | No such intent | No intent of that number is in force |
| `withdraw_intent` | Made by the staff | The intent was formed by the staff |
| `form_intent` | No such intent | The parent is not in force. Tried after the issuer |
| `form_intent` | Wrong plan state | The parent's plan is not released |
| `form_intent` | Cannot stand under | The kind cannot stand under the parent's kind: the one pair built is cover from the air under hold a place for the same base |
| `form_intent` | Not your base | The base is not one of the side's |
| `set_limit` | No such intent | The limit is of an intent (all but the reserve) and no intent of that number is in force. Tried first |
| `set_limit` forces | No source | The intent is a supply intent and the limit names no source |
| `set_limit` forces | Not your ship | A listed ship is not the side's, in ship order |
| `set_limit` forces | Not your force | A listed task force is not the side's, in force order |
| `set_limit` forces | Not your group | A listed air group is not the side's, in group order |
| `set_limit` forces | Not your base | The source is not one of the side's |
| `set_limit` area | Bad area | The area has no hex, or a hex off the map |
| `set_limit` protected | Not your ship | A listed ship is not the side's |
| `set_limit` reserve | Not your base | The base is not one of the side's |
| `set_limit` reserve | Bad weight | The supply reserve, then the fuel reserve, is below 0 |
| `set_limit` forces, area, risk | Wider than its parent | The intent has a parent and the limit does not narrow (The intent tree, Limits read up the tree). Tried after every other reason of the limit |
| `approve` | No such plan | No plan of that number |
| `approve` | Wrong plan state | The plan is neither a draft nor preparing |
| `approve` | No leave | The staff gave it, and the leave the move needs is withheld |
| `cancel_plan` | No such plan | No plan of that number |
| `cancel_plan` | Wrong plan state | The plan has ended |
| `commit_ships` | No such plan | No plan of that number |
| `commit_ships` | Wrong plan state | The plan is not approved |
| `commit_ships` | No forces limit, no source | The plan's intent has no forces limit of its own; or it has one that names no source |
| `commit_ships` | Bad days | The first day is before the day of the command, or after the last |
| `commit_ships` | No such run | A run is named and the plan has no run of that number |
| `hold` | Not your ship | A ship is named and it is not the side's, or is sunk, or does not exist |
| `hold` | Not your force | A task force is named and it is not the side's |
| `hold` | Not your group | An air group is named, whole or by a setting, and it is not the side's. Each of these three is tried for the one target the command names |
| `hold` | Already held | The target has a hold already, made by a command or by an order. A whole air group and its search arc are two targets, and a group may have both |
| `release` | Not held | The target has no hold. An enemy unit and one that does not exist are refused so too |
| `set_interrupt` | Interrupt number not usable | A numbered interrupt is set and its number is 0 |
| `set_interrupt` | Not your base | A numbered interrupt is set and its base is not one of the side's |
| `set_interrupt` | No days | A numbered interrupt is set and its number of days is 0. Setting a standing kind has no other reason: it is refused only as not yours |
| `acknowledge` | No such interrupt | A numbered interrupt is named and none of that number was set |
| `acknowledge` | No such plan | A standing interrupt for a plan is named and the side has no plan of that number |
| `acknowledge` | Not your ship | A standing interrupt for a ship is named and the ship is not the side's, whether it exists or not |
| `acknowledge` | Not fired | The interrupt has never fired: for a standing one, the side holds no record of it |
| `acknowledge` | Already acknowledged | The last firing is acknowledged already |
| `commit_ships` | Not your ship, not in the pool, held, already committed | For each ship in ship order, in that order: it is not the side's; it is not in the pool of the forces limit; it is held (itself, its force, or any ship of its force); it has a commitment that overlaps the days asked for |
| `free_ships` | No such plan | No plan of that number. A plan of any state is found |
| `free_ships` | Not committed | A ship has no commitment to the plan |
| `add_run` | No such plan, wrong plan state, no forces limit | As for `commit_ships` |
| `add_run` | No ships | The run names no ship |
| `add_run` | Not your ship, not in the pool, held | For each ship in ship order, in that order, as for `commit_ships` (a ship that is held is refused before the load and the days are looked at) |
| `add_run` | Bad weight | The load is not above 0 |
| `add_run` | Bad days | The days are not in order, or the load day is before the day of the command |
| `time_run` | No such plan, wrong plan state | As for `commit_ships` |
| `time_run` | No such run | The plan has no run of that number |
| `time_run` | Bad days | The days are not in order, or the free day is before the day of the command |
| `mark_plan` | No such plan, wrong plan state | As for `commit_ships` |
| `end_run` | No such plan, wrong plan state | As for `commit_ships`: the plan is not approved |
| `end_run` | No such run | The plan has no run of that number |
| `end_run` | Run ended | The run has ended already |

`set_leave` is refused only when the staff gives it. [0107 items 1, 4, 22 and 27, rulings 17, 20, 31 and 33]

The same intent number set twice. Two `set_intent` commands of one day that name the same new number are taken in in the order handed in: the first sets the intent and drafts its plan, and the second changes the intent that now exists. The plan is a draft, so the second is no amendment and drafts nothing. The intent is as the second left it. [0107]

### What a command does and records

A command that passes is applied, then `CommandTaken` is recorded with the whole command and its issuer. It has no cause: it is the root record of the change. A command that fails records `CommandRefused` with the command, its issuer and the reason, and has no cause. What an accepted command sets off is recorded after `CommandTaken` as an event that cites it:

| Event | Recorded when |
| --- | --- |
| `PlanDrafted` | An intent's number is first set, or an intent in force whose plan has ended is set again. Names the plan and the intent |
| `PlanStateChanged` | A plan changes state, by `approve`, `cancel_plan`, `mark_plan` or the withdrawal of its intent. Names the plan and both states |
| `PlanAmended` | A change is kept as an amendment. Names the plan and the amendment's number |
| `IntentFormed` | A `form_intent` is taken in. Names the side, the new intent's number and its parent |
| `IntentWithdrawn` | A `withdraw_intent` is taken in: one for each intent beneath the one withdrawn, in the order the tree gives (The intent tree, below). Names the side, the intent and the one it stood under |

A `set_leave`, a `hold`, a `release`, a `set_interrupt`, an `acknowledge`, a `commit_ships`, a `free_ships`, an `add_run`, a `time_run` and an `end_run` set off no other event: the command's own record, with the whole command, is the record of the change. There is no command number. Withdrawing an intent whose plan has already ended, and setting an intent that is a draft again, set off nothing. [0107]

One more event is recorded for a hold, and it is not a command's:

| Event | Recorded when |
| --- | --- |
| `HeldByOrder` | The override (below) holds a unit by an order of the commander's. Names the side, what is held and the order. It cites the order and the event that accepted it, and is recorded after them, at the step of the override, with no pulse |

A side sees: its own `CommandTaken`, `CommandRefused`, `PlanDrafted`, `PlanStateChanged`, `PlanAmended`, `HeldByOrder`, `IntentFormed` and `IntentWithdrawn`, whole, and the enemy sees none of them. Each of these kinds is decided in `side_events.rs`. The two events of step 5, `StaffNotice` and `InterruptFired`, are the same (Interrupts and Notices, below). [0107, ruling 23; 0117]

## The intent tree

Intents form a tree. An intent is an end, a state of the world the commander wants of one of the side's bases; the staff breaks an end into sub-intents, and orders are given at the leaves. The tree is state: it is hashed, changed only by commands, and rebuilt by a replay. The engine records and checks it and never reads a reason's words about it. [0116, 0117]

**What an intent gains.** Each intent has three more fields beside its kind and base:

| Field | Holds |
| --- | --- |
| Parent | The intent it stands under, or none for a **root** |
| Maker | The commander (by `set_intent`) or the staff (by `form_intent`) |
| Addressee | The level the intent is given to. It is set by the kind and chosen by nobody: keeping a base supplied is given to the area; holding a place and covering a base are given to the base they name. A task force and a land formation are not built as addressees |

The level that owns an intent is the kind of its addressee. [0117 items 1 and 2]

**Roots and sub-intents.** A commander's `set_intent` makes a root, whatever its kind: a commander states a task such as cover himself by stating it as a root. The staff's `form_intent` makes a sub-intent: it names the parent and the kind, and the new intent takes the number one above the highest the side has ever used (a withdrawn number is never reused). Its maker is the staff, its addressee is its kind's, its plan is its parent's (so its root's: one plan for a tree), it has no limits of its own, and it is set on the day the command is taken in. Intake refuses a `form_intent` as in Every reason a command is refused: the parent must be in force, its plan released, and the kind able to stand under the parent's kind. The one pair built is cover from the air under hold a place for the same base; a commander cannot name a parent. `form_intent` records `IntentFormed`, citing the command's `CommandTaken`; it drafts no plan and is no amendment. [0117 items 3, 4 and 9]

**A staff-made intent is the staff's.** `set_intent` and `withdraw_intent` on the number of an intent the staff formed are refused as "Made by the staff". The commander shapes it by its limits (below), by holds, and by withdrawing the intent it stands under. [0117 item 5]

**Withdrawing a tree.** A `withdraw_intent` withdraws the intent and every intent in force beneath it, depth first, each branch by number, all with the day of the command. Each intent beneath records an `IntentWithdrawn` event naming the intent it stood under, citing the command's `CommandTaken` (the intent named in the command has no event of its own: the command is its record). A withdrawn intent's plan ends as for any intent (Plans, above) when it is the root. [0117 item 6]

**Limits read up the tree.** The commander may set a limit on any intent in force, a staff-made one too (`set_limit` names the intent). An intent with no limit of a kind has its parent's, and so on to the root: forces, risk, priority and area are each read from the intent itself or the nearest above it. The risk read where none is set anywhere is balanced. With no forces limit on it or above it, an intent may use what its addressee commands (The six limits, above). A limit set on an intent that is in force while its plan is approved is an amendment of the plan, whichever intent of the tree it is on. [0117 items 7 and 8]

**A limit must narrow.** On an intent that has a parent, `set_limit` of forces, risk or area is refused as "Wider than its parent" unless it narrows what the parent may use, read up the tree from the parent:

- **Forces:** each ship is in the parent's pool, each task force is inside the parent's limit, each air group is one the parent may use, and a source is named only where it is the parent's source. Where the parent has no forces limit on it or above it, the limit may name air groups the parent may use and no ship, task force or source.
- **Risk:** no bolder than the parent's, from preserve the force (the most careful) through balanced to press.
- **Area:** a subset of the parent's area. Where the parent has none, any area.

A priority, a protected set and a reserve are never refused for this. A limit on a root is not narrowed against anything. This reason is tried after every other reason of the limit. [0117 item 7. The rule for an area is in the code and decision 0117 does not state it]

**A reason names the chain above it.** A reason's serves part gains, for the intent it serves, the intents above it, nearest first: each with its number, its maker and its statement in words as it stood that morning. The top of every chain is an intent the commander stated. A reason for a root has none. The words follow the statement, for example "intent 2: cover base:1 from the air; plan 1; under intent 1 (the commander's): hold base:1". A game saved before the chain reads each reason with none. The engine records the chain and never reads it. [0117 item 10]

**Order among intents.** The staff serves intents of a kind in priority order: a rank, 1 first, read up the tree, then the intents with none, each by number. The supply doctrine serves keep-supplied intents (all roots) as it always has; the air doctrine serves hold and cover intents (The doctrine, covering a base from the air, below). [0107 ruling 10; 0117 item 11]

**The tools** show the tree: `command_state` lists each intent under its parent with its maker, addressee, children and depth, and its limits as set and as read up the tree with the intent each is read from; `propose` shows the tree the staff would form; `why` and `staff_orders` give the chain (The tools, below). [0117]

A side sees: its own intents, whole, with `IntentFormed` and `IntentWithdrawn`, and the acceptance or refusal of each command. The enemy sees none of it. [0117]

## Holds and the override

A **hold** keeps one thing from the staff: the commander says "this is mine, not yours". It is a record in the side's command state, hashed with it, and it has no date of its own: it stands until a `release`. [0107, rulings 17 and 18]

**What can be held.** A ship (the side's own and afloat), a task force of the side, an air group of the side as a whole, and one setting of an air group, which today is its search arc (a group's search arc and the mission that sets it flying). A group held as a whole is given no order of any kind by the staff; a group whose search arc is held is given no order for its arc or its mission. A group and its search arc are two targets and each may have a hold. A hold on a base waits for a stage whose doctrine acts on a base, and a land unit is never held. A target has one hold: a second `hold` of a target already held is refused, whoever made the first. A hold records the day it was made and what made it: a `hold` command, or an order of the commander's (the override, below). A `release` lifts the hold on a target whatever made it, and a later order of the commander's to the unit holds it again (below). A hold moves nothing and stops nothing: a held unit keeps its standing orders and goes on by them. [0107, rulings 17 and 19]

**What a hold takes from the staff.** One rule says which ships are held, and every rule below uses it: a ship is held if it is held itself, if the task force it is in is held, or if any ship of that force is held. A held ship is not free to the staff, no ship of a held force is free, and no ship of a force that has a held ship in it is free: an order to that force would move the held ship. A held setting keeps the staff from changing it, and a held air group is one the staff gives no order to: the air doctrine leaves a group out of its work altogether if the group or its search arc is held (Covering a base from the air, below). Nothing else of the side is touched. The staff gives no order to what is held. [0107, ruling 18]

**The guard behind it.** The engine refuses what the staff would do to a held thing, so the doctrine's own care is not the only check:

- A staff command that names ships, `commit_ships` and `add_run`, is refused as held for a ship that is held (Every reason a command is refused).
- A staff's order is refused **held** ("the unit is held by its commander", with the target) if it is for a task force that is held or has a held ship in it; if it is an order to form a task force that names a held ship, or a ship in a held force; if it is any order for an air group that is held as a whole; or if it is a search-arc order or an air mission order for an air group whose search arc is held (a patrol-share order for such a group is not refused as held). It is tried before the check of the forces limit (The forces limit on a staff's orders, below), so a held unit is refused as held whether or not it lies in the limit. A land unit and any other base order are never refused as held.
- The guard binds orders handed in by a staff. It does not bind the commander's own orders, and it does not bind an order a scenario's script gives with the issuer `staff`, which is a scenario's statement (below).

A refused order is numbered, kept and recorded as refused, as any is. [0107, rulings 18 and 30]

**The override.** The commander's own order to a unit the staff works with holds that unit, from that morning. It is step 2 of the day (chapter 1, Before the pulses). In the code it runs after the returns from the yard and before the staff is asked. For each side with at least one intent in force:

1. The side's pending orders are walked in the order intake sorts them: orders to form a task force first, then the rest by the unit they are for. An order of the issuer staff is skipped, so a scenario's scripted staff order never overrides.
2. An order **overrides** if it is for a unit inside the forces limit of an intent of the side in force: a task force the limit lists, or one with at least one ship whose every ship the limit lists; or an order at any base to form a task force that names at least one ship of the pool of such a limit (a ship the limit lists or one in a task force it lists). Or it is an order of any kind for an air group that some intent of the side in force may use (The six limits, above: named in the forces limit read up the tree, or, with none, at the base the intent is addressed to; the intent's plan is not looked at). An order for a land unit, a base order other than forming, a group no intent may use, or a unit in no forces limit does not override: it stays pending, in the order handed in, for step 5, as it was.
3. An order that overrides is taken in at once: numbered then, ahead of every other order of the day, and checked as any order is. If it is accepted, it holds what it was for, a task force, an air group as a whole, or each ship of the pool that a forming order names, and a `HeldByOrder` event is recorded for each, citing the order and its acceptance. An order that is refused holds nothing. A target that is held already gets no second hold, and the order is taken in all the same.

A side with no intent in force is not touched, and the day is as it was. `World::at_staff_step` takes the override too, so the view a staff is asked on shows the holds of the morning. A hold made by an order is lifted by a `release` like any other, and a later order of the commander's to the unit holds it again at the next step 2. [0107, rulings 23 and 30]

A side sees: its own holds, whole: each with its target, the day, whether a command or an order made it, and the order's number, in the agent's `command_state` (`holds`); its own `HeldByOrder`; the acceptance or refusal of each hold and release command and of each order, with the reason, including a staff order refused as held. The enemy sees none of it. A hold on an enemy unit, or on one that does not exist, is refused as not the side's, so a refusal never shows that an enemy unit exists. [0107, rulings 17 and 23]

## Interrupts

An interrupt is something the commander wants to be told of: when it fires, a run of days stops (The tools, `run`) and the command report lists it. It changes nothing else: a firing moves no unit, gives no order and changes no stock. It is command state: the interrupts set, whether each is armed, the day each last fired and whether that firing is acknowledged. It is hashed, saved through the commands that made it and rebuilt by a replay. [0107, rulings 31 and 32]

**The kinds.** One is set by the commander, and three stand without being set. [0107, ruling 33]

| Kind | Holds on a morning when | Armed apart for |
| --- | --- | --- |
| Set: a base's days of supply below a number | The days of supply of the hex of the base (Days of supply, below), read when the staff is asked that morning, has a figure and the figure is below the number of days. 35 is not below 35. It does not hold where the hex has no figure (no supply is planned to be used there), or where the base is no longer the side's | Each numbered interrupt |
| Standing `cannot_meet` | The staff handed in a cannot-meet notice for the plan that morning (Notices, below) | Each plan |
| Standing `waits_for_leave` | The staff handed in a notice for the plan that it waits for a leave, to prepare or to launch | Each plan |
| Standing `ship_lost` | A ship of the side was lost the day before: a ship sunk in a fight (`ShipSunk`) or lost in port with its base (`ShipLostWithBase`) on the day before, by the side the ship belongs to. Tested only for a side with at least one intent in force, so a game with no command gets no event of it. On day 1 there is no day before | Each ship |

A set interrupt names a base of the side and a number of days, at least 1. It has its own number, a whole number from 1, that the commander gives; a number is not withdrawn and no command withdraws one. A standing kind is on until the commander switches it off, and on again when he switches it on. A kind that is off is not tested; the records of its subjects stay as they were. The kinds in the code are `SupplyBelow` for the set one and `cannot_meet`, `waits_for_leave` and `ship_lost` for the standing ones. [0107, rulings 31 and 33]

**Armed, fired, acknowledged.** A set interrupt is armed when it is set, never fired and not acknowledged. A standing interrupt for a subject that has no record is armed and never fired, and gets a record the first morning it fires. On a morning when the condition holds and the interrupt is armed, it fires: it is no longer armed, it last fired today and it is not acknowledged. On a morning when the condition holds and it is not armed, nothing happens. On a morning when the condition does not hold, it is armed again, which for a standing kind is every record of the kind whose subject does not hold. So an interrupt already true when it is set fires that morning, once, and then fires again only after a morning on which it did not hold. Acknowledging marks the last firing seen and arms nothing; the next firing is not acknowledged. [0107, ruling 32]

**When they are tested.** Each morning, at step 5: after the staff's commands are taken in and its notices recorded, and before any order is taken in. The figure a set interrupt is tested on is the one read at step 4, on the view the staff was asked on (after the commands and the override, before the staff's commands), so a staff's order or command of that morning changes no reading it is tested on. The sides are tested in side order, Allied first; for a side the set interrupts by number, then the standing kinds in the order above, and for a kind its subjects in plan, then ship number order. A side with a set interrupt and no staff is tested all the same. [0107, ruling 31]

**What a firing records.** An `InterruptFired` event, with no pulse, naming the side and the interrupt. A set one carries its condition and the figure it was tested on; a standing one carries neither. It cites the cause of the firing:

| Interrupt | Cites |
| --- | --- |
| A set one | The latest `CommandTaken` of the command that set that number. A set interrupt of a scenario cites its scripted command |
| A standing one for a plan | The `StaffNotice` events of the morning for the plan that make it hold |
| `ship_lost` | The event that sank the ship, or lost it with its base |

When breakdowns are on, a set interrupt's firing records the stock in tons, the planned use in tons, the days of supply and the number it fell below. Recording it changes nothing. A firing draws no random number. [0107, rulings 31 and 33]

**The commands.**

- `set_interrupt` with a number, a base and a number of days sets that interrupt, or sets it again: the condition is replaced, the day set is today and its status is as a new one's, armed and never fired. With a standing kind and `on` it switches the kind off or on, from that morning.
- `acknowledge` names a set interrupt by its number, or a standing one by its kind and the plan or the ship. It marks the interrupt's last firing seen.

Both are the commander's and are taken in at the first step of the day, so a set interrupt is tested that same morning and an acknowledgement made that day stands for the firing of an earlier day. The refusals are in the table above (Every reason a command is refused). An acknowledgement of a ship that is not the side's is refused as not the side's, whether the ship exists or not, so a refusal never shows that an enemy unit exists. [0107, rulings 31 to 33]

A side sees: its own set interrupts, standing records and the kinds it has switched off, with whether each is armed, when it last fired and whether that is acknowledged, in the agent's `command_state` (`interrupts`) and `report`; its own `InterruptFired`, whole; and the acceptance or refusal of each command. The enemy sees none of it. [0107, rulings 31 and 37]

## The staff

### What a staff is, and when it is asked

A staff is code outside `sim-core`. `sim-core` defines what it is: given a side's view, it decides the day and hands in a `StaffDecisions`:

- **Orders**, each with the intent it serves and its reason.
- **Staff commands**: the commands of the staff's column above, each with its reason.
- **Decisions to give no order**, each a reason.
- **Notices**: structured statements beside these, each resting on one of the reasons above (Notices, below).

A staff is a function of the view: it is asked with a shared reference, so that a staff asked twice on the same view gives the same answer. That is what `replay --check-staff` tests. The function that resolves a day takes each side's staff or none. A side with none is not asked, and a day with no staff and no command is resolved as it was. [0107 items 4, 7 and 8]

A staff is asked once a day, at its place in the day's order (chapter 1, Before the pulses): after the commander's commands are taken in, after forces back from the yard are home, and after the override has taken in the commander's orders to units in a forces limit and held them, and before any other order is taken in. It gets the side's view as it then stands, so it sees the reserves, the leaves and the holds set that day, the orders that held a unit that morning, and the fuel ships and bases held at the end of the day before: nothing is refuelled before the staff is asked (chapter 2, Refuelling in port). It never gets the truth (chapter 1, Truth and belief). [0107, The order of a day]

What a replay hands in, in place of asking, is taken in at the same step as if a staff had given it. If a side has a live staff and something handed in for it, both are taken in, what was handed in first.

The staff's commands are taken in first, each side's in the order handed in, and after each side's commands its decisions to give no order are recorded, and then its notices (Notices, below). When every side's are done, the interrupts are tested (Interrupts, above). Then every order of the day that the override has not already taken in is taken in: the commander's and the staff's together (chapter 1, How orders are numbered and sorted). A staff's order comes after the commander's to the same unit, so the staff's replaces the commander's in a standing slot. A staff's order is marked with the issuer staff and the intent it serves (chapter 1, Who can issue), and is refused as held if it is for something the commander holds (Holds and the override, above). An air group's order serves the cover intent, and not the hold intent above it. A cargo order the staff gives heeds the reserve of a base (chapter 2, Cargo orders). [0107, 0111]

Every runner gives both sides the staff of `sim-staff` (`sim_staff::staffs()`): the doctrine for keeping a base supplied and, after it, the air doctrine for holding a place and covering a base from the air (below). A staff whose side has no intent in force and no commitment to free hands in nothing: nothing is taken in and nothing is saved for that day. [0107, 0115, 0117]

### The forces limit on a staff's orders

A staff's order that is not refused as held (Holds and the override, above) is refused as **outside the forces limit** ("the order is for a unit outside the forces limit of the intent it serves") unless the intent the order serves is in force and:

- for an order to an air group, the group is one the intent may use (The six limits, above): named in its forces limit, its own or the nearest above it, or, with no forces limit on it or above it, at the base it is addressed to; or
- for any other order, the intent has a forces limit, its own or the nearest above it, and the order is for a task force that the limit lists, or that has at least one ship and every ship of it is a ship the limit lists, or it is the limit's source base's order to form a task force from ships that are in the pool.

An order for a land unit or any other base order is inside no forces limit, so a staff's order for one is refused. The check does not look at the plan's state or at the area. A refused order is numbered and kept, and recorded as refused with this reason, and nothing else about it is checked. An order that passes is checked as any order is (chapter 1, Every reason an order is refused). [0107, As built]

The check binds the orders a staff hands in. It does not bind an order that a scenario's script gives with the issuer `staff`: that order is a scenario's statement, set down by the scenario's author as a starting condition, and not a decision of a staff made on a view. Scenario 39 is such a scenario. Such an order is checked as any scripted order is. [0107, As built]

A staff command that names ships is checked against the pool and the holds too (`commit_ships`, `add_run`; above). [0107]

A side sees: its own acceptance or refusal of the order whole, with the reason. [0107]

### Reasons

Every order a staff gives and every decision it makes to give none has a **reason**. A reason has five parts:

| Part | Holds |
| --- | --- |
| Serves | The intent, and the plan and the amendment the decision was made under, with the intent's statement: the intent in words as the commander set it and as it stood that day ("keep base:2 (Placeholder base T) at 30 days of supply, a run every 15 days"; "no interval set (the staff takes 15 days)" where he set none; "withdrawn on day N" where it was). The amendment is none for the baseline. Then, for an intent with a parent, the chain: the intents above it, nearest first, each with its number, its maker and its statement as it stood (The intent tree, above) |
| Evidence | Each thing the side held that mattered, in words, with its day |
| Binding | The limit, stock, distance or commitment that shaped the decision, in words |
| Worked | The calculation, in words |
| Would change | What, if different, would give a different decision |

A reason is handed in with its order, with its staff command, or with the decision to give no order, and is recorded as a `StaffReason` event. For an order, the event cites the order and the event that accepted or refused it, and is recorded after them. For a staff command it has no order, holds the command as handed in, and cites the command's `CommandTaken` or `CommandRefused`, and is recorded after it and its follow-ups. For a decision to give no order it has no order and no cause. [0107; differences.md row 43] **A reason is kept, not worked out again**: it stays in the log whatever the intent has since become, so it still resolves after its intent is replaced or withdrawn. Its number is the number of its event: a reason has no number of its own. [0107 items 4 and 12]

The staff writes the intent's statement into every reason it hands in, from the side's command state that morning, so the words travel with the reason: every place a reason is shown (the tools `why`, `staff_orders`, `propose`, `preview` and `report`, the replay export and viewer, `run --events` and `sim-cli events`) gives the same words, and a reason made before its intent was changed or withdrawn still says what the intent was then. Where its worked part speaks of a plan with no base in it (a leave given or withheld, a plan ended, a run ended or set aside, a commitment made again), it names the plan with what it is for: "plan:1, the plan to keep base:2 (Placeholder base T) at 30 days of supply,". The statement of a hold intent reads "hold base:1 (name)" and of a cover intent "cover base:1 (name) from the air", with ", withdrawn on day N" where it was. The statement of every intent above the one served is in the reason's chain, with its maker (The intent tree, above). A game saved before the statement was handed in reads its reasons with none. In the log's own line a reason reads "staff's reason for order N", "staff's reason for the command to ..." or "staff's reason for giving no order", then the serves part in brackets, then the worked part; a count of one there and in a command's own line is "1 ship", not "1 ships". [0107, "As built (step 4)", the mend]

The engine stores what the staff writes in a reason and checks none of it. Evidence names a side's own ships freely and the enemy only as a contact, a claim or an estimate (chapter 3, Belief): that is the staff's to keep, and a staff is given nothing else to name. [0107, truth and belief]

A side sees: its own `StaffReason` whole. The enemy sees none. [0107]

### Notices

A staff hands in notices beside its orders, commands and reasons: that it cannot meet a plan's intent, that a plan waits for a leave, that it has ended a run. A notice is structured, so that a standing interrupt can read it without reading words. It has a kind, the plan, the run where the kind is about one (a run ended; else none) and the reason it rests on, which is one of the same hand-in's staff commands or decisions to give no order. [0107, ruling 34]

| Kind | Means | The doctrine hands it in |
| --- | --- | --- |
| `cannot_meet` | The staff cannot meet the plan's intent | On a morning it weighs a new run for the plan and either can have none, or the run it can have leaves the reading it checks below the floor (The doctrine, Cannot meet). It rests on the day's reason for giving no order about the next run, or, for a cut run, on the reason of the `add_run`. A load cut that still leaves the reading at or above the floor is no notice. For a cover intent, on a morning no fighter group it may order can fly (Covering a base from the air, When it cannot); it rests on that morning's reason for giving no order |
| `waits_for_leave_to_prepare` | The plan is a draft and waits for the commander's leave to prepare | On a morning the plan is a draft and leave to prepare is withheld, for a supply plan and for the plan of a hold or cover root. It rests on the reason that says so |
| `waits_for_leave_to_launch` | A run that is loaded waits in a plan that is preparing, for the leave to launch or the commander's approval | On a morning a loaded run waits for that; for the plan of a hold or cover root, on a morning the plan is preparing and leave to launch is withheld. It rests on the day's reason for giving no order |
| `run_ended` | The staff ended a run | On a morning it sets a run aside for a held ship (The doctrine, Each morning, before any intent). It names the run and rests on the reason of the `end_run` |

[0107, ruling 35]

The engine records each notice as a `StaffNotice` event, naming the side, the kind, the plan and the run, with no pulse. It is recorded after the staff's commands and the decisions to give no order of the side, in the order handed in, and cites the `StaffReason` it rests on, so a notice leads to its reason and the reason to the evidence. A notice that rests on nothing the hand-in holds is recorded with no cause. The engine tests the standing interrupts on the notices and reads nothing of a reason's words. A notice changes no state. [0107, ruling 34]

A side sees: its own `StaffNotice`, whole. The enemy sees none. [0107]

## Days of supply

Days of supply is a side's supply stock in a hex over its planned use of supply there for a day. It is worked by `SideView::days_of_supply(hex)` from the side's view only: its own bases, land units, forces, ships and air groups, the map and the figures. It is nothing of the enemy's, changes nothing and draws no random number. [0107 items 31 to 36]

**Where.** It is for the side in a hex, not for a base. A hex where the side holds no base has none. Every base of the side in a hex carries the same figure, because the men of a side are fed from every base it has there. An intent that names a base means its hex. [0107 item 31]

**Stock.** The supply of all the side's bases in the hex, in tons. [0107 item 32]

**Planned use.** The tons the side is planned to spend there on the coming day, in five parts. A part whose figures the scenario does not load is 0, as the rule that charges it charges nothing:

| Part | What | Where it comes from |
| --- | --- | --- |
| Men ashore | Each of the side's land units with men ashore in the hex, at its daily charge for its men | Chapter 7, The men's supply |
| Men due | The same charge for men aboard the side's own forces that are ordered to unload in the hex and are due on or before the day, or were due earlier and have not landed | Chapter 7, Landing men. A force is due on the day its passage to the hex ends, forecast from the view as the measuring tool forecasts it. A force already stopped in the hex counts when its unload can work there and men are still aboard |
| Aircraft upkeep | The daily supply for each ready and each damaged aircraft of the groups at the side's bases in the hex | Chapter 5, Daily supply; `air_supply.toml: daily_supply_t_per_aircraft` |
| Repairs | For each base, the smaller of its repairs a day (its figure, and the repairs the work done on aircraft the day before gives it) and the damaged aircraft at it, times the supply of a repair. Counted only where the air upkeep figures are loaded | Chapter 5, Repair; `air_supply.toml: repair_supply_t` |
| Sorties | The sorties of the groups at its bases there that are on a mission that flies whatever is sighted, at each of the day's two daylight pulses: a search group with an arc set, at the cost of a sortie of twice its arc's range, for the aircraft able to fly; a group on combat air patrol, at the cost of a sortie of no distance, for the fighters that fly its patrol share of the aircraft able to fly | Chapter 5, Supply for sorties and What a group can fly |

**A strike mission counts nothing.** Whether a strike group flies depends on what is sighted, and a figure that moved with enemy reports would move for no change in supply. A group at rest counts nothing, and a group aboard a ship is counted at no base. Each part is worked by the engine's own rule for it and by no second formula. [0107 items 32 to 34, rulings 1 and 2]

**The figure.** The stock over the planned use, a fixed-point quotient cut toward zero at six decimals. **No planned use, no figure**: where the planned use is 0 there is no figure, and the answer says so in words ("No supply is planned to be used in this hex today, so there is no days-of-supply figure"). The stock is still given. [0107 item 36]

**When it is read.** At the step where a staff is asked: at the start of a day, after the commands, the returns and the override and before the day's orders are taken in. Read between days it gives the same figure, since nothing between the end of one day and that step changes a stock or a use. An order given today shows in the figure of tomorrow: a group set to patrol today raises the planned use read the next day. [0107 item 31, ruling 3]

**The projection** of the figure over the coming days, and the first day it falls below an intent's floor, are the doctrine's (below, The doctrine). The engine gives it what it counts by its own rule: `SideView::men_due_by_day(hex)`, each force due in the hex with its due day and its men's charge, the rule the men-due part above uses. [0107 items 34 and 35]

A side sees: its own figure, with its stock, its planned use and the five parts, in the agent's `list_bases` and `get_entity` (as `days_of_supply`) and in the briefing's entry for each of its bases (as `planned_days_of_supply`). The briefing's entry for a base also keeps its older `days_of_supply`, which is a different figure: the stock over yesterday's spending, cut to one decimal (glossary, Days of supply). The enemy sees none. [0107 item 47]

## The doctrine: keep a base supplied

The staff (`sim_staff::SupplyStaff`) decides from its side's view alone, at the step of the day where a staff is asked. It holds nothing of its own: what it remembers is the side's command state (its intents, plans, runs and commitments) and what the view shows of its ships and air groups. Asked twice on the same view it gives the same answer. It draws no random number and uses no floating point. Its version is `supply-8`. It runs two doctrines in this order: this one, for the keep-supplied intents, which serves no other kind; then the air doctrine (Holding a place and Covering a base from the air, below), for the hold and cover intents. It hands in notices (Notices, above) at the places named below. [0107 items 3 and 37 to 45, rulings 4 to 9, 17 to 30, 34 and 35]

"Today" below is the day being resolved. A ship, a force and a base are named in a reason as the engine writes them (`ship:1`, `task_force:1000001`, `base:2`); the agent's tools show them in the side's own numbers.

### The projection

For the hex of the intent's base the staff projects a reading for each day from today. Today's reading is the view's (Days of supply, above). The reading of the next day is that day's reading, less its planned use, plus what lands that day, and never below 0. The planned use of a coming day is today's, with the men due counted from the day each force is due (`men_due_by_day`). What lands: for each run of an approved plan of the side that has not finished, its load while it has not sailed, else the supply aboard its ships, on the day after its arrival day, or today if that day is past, and only at the hex where the run will land it (below). A run ended, set aside this morning or of a plan that has ended has finished and lands nothing. A reading's figure is its stock over its use; with no use it has none.

**Where a run lands.** A run still loading lands at the base its intent names as it stands now, so a changed base moves it. A run that has sailed lands at the first hex of its force's route still ahead that is marked to act at, and nowhere once that has been passed; where its route has no mark at all, or its ships are in no force, it lands at the intent's base. So a run that sailed for a base the intent no longer names is counted at the hex it will reach, and not at the new base. [0107, rulings 19 and 28] The report of it is today's figure and the first day whose figure is below the floor. [0107 item 35, ruling 1]

### Each morning, before any intent

The staff first **sets aside** each run of an approved plan that has not sailed and has a held ship (the ships of a run in a force, held through the force, are held; Holds and the override, above). A run has not sailed while its ships are all at the source, stopped; a run that has sailed is under way and is not set aside. For each run set aside the staff hands in an `end_run` and, where its ships hold a commitment to the plan, a `free_ships` for them, each with a reason that says which hold ("task_force:1000001 is held by the commander's hold of day 8", or "... by the commander's order of day 2") and what is aboard ("the 100 t aboard stay aboard", or "nothing is aboard"). These are the first staff commands of the morning. For each run set aside it also hands in a notice, run ended, naming the plan and the run and resting on the reason of the `end_run`. For everything else the staff reads that morning, the run is finished the same morning and the ships' commitment to the plan is gone, so they are free to the next run of whichever intent needs one, in the held ship's place, and the projection no longer counts the run's landing. The run is ended for good: a release does not take it up again, and the staff sizes a new run. A run of a plan that has ended is set aside by nothing, since it holds nothing. [0107, rulings 20 and 21]

A run that has sailed goes on by the orders it has when one of its ships is held: it lands its load and comes home, and the projection counts what it will land. The staff gives it no further order, since a held force is not free to it. [0107, ruling 19]

### Each day, for each intent

The keep-supplied intents in force are served in priority order: ranked ones by rank, 1 first, then those with none, each at equal rank by the lower number. A supply intent has no date, so the spec's tie-break by date does not arise. **What the staff has given an intent served earlier the same morning is committed for every intent served after it:** the ships of a run it has just added (until the run's free day) and the tons that run has still to load (all of its load). So two plans never count the same ship or the same ton on the day both are planned. A free ship goes to the first intent, in this order, that needs a run that day; the staff keeps no free ship idle for a higher intent that will need one later, and recalls no run under way. A changed priority is a new allocation from the next morning the staff is asked, and moves nothing that is committed. [0107 items 37 and 43, rulings 10, 11, 13 and 15]

For each intent:

1. **What stops it.** With no forces limit, no order and a reason. A destination or source base that is not the side's: an approved plan is marked failed (`mark_plan`), and no order. A plan that has ended: no order, except the fallback of a cancelled plan (Cancelling, below). A pool with no ship afloat: an approved plan is marked failed, and no order. A pool whose every ship afloat is protected for another intent in force (that intent's protected limit): no order, and the reason says that no ship of the pool may be taken, naming each ship and the intent it is protected for; the plan is not marked failed. The staff never marks a plan complete: a supply intent ends when it is withdrawn. [ruling 16]
2. **The leaves.** A draft is approved to preparing when leave to prepare is given; else no order, and the reason says the plan waits for the commander; a notice, waits for leave to prepare, rests on that reason. A plan that is preparing is released the same day when leave to launch is given. A plan that is preparing forms and loads its first run and does not sail it.
3. **The runs under way.** A run whose ships are all at the source, stopped, is loading, unless its sail day is past and nothing is aboard, when it has finished; a run with a ship away from the source is under way; a run whose ships a later run has taken, or whose ships are all sunk, has finished; so has a run that has ended and a run of a plan that has ended. A run loading whose load is complete (the supply aboard is its load, or the source has nothing more above its reserve, or its ships are full), after its load day, sails if the plan is released: a route to the destination and back to the source, marked to act at the destination, and an unload order. If its sail day has passed, it is timed again first (`time_run`) and its commitment renewed (`free_ships`, `commit_ships`). A run loaded in a plan that is preparing waits for leave to launch.
4. **A new run.** When no run of the plan is loading, a run is needed if, without it, the reading of the day after a run ordered today would arrive is below the floor's stock: the floor in days times the planned use of that day. Then:
   - **Its ships** are the pool's free ships in number order, taken until their room covers the load. A ship is free when no unfinished run of any plan holds it, no run added earlier the same morning holds it, no commitment of a plan that has not ended covers today, no other intent in force protects it, the commander holds neither it nor its force nor any ship of its force, and it is stopped at the source, in port in no force or in a force there with no route. A held ship gives no day on which it will be free: it is not counted in the day the staff expects the next ship. A ship in a force brings its whole force. A ship's room, for taking more, is its class's cargo less the supply aboard it. The supply aboard the free ships counts toward the load (below), so ships are taken in number order until what is aboard them and the room they have to take more cover the load, or the source's tons above its reserve if those are less.
   - **Its load** is the floor's stock on the day it arrives, less the reading of that day, and not below 0, plus the planned use of the interval's days from the day it arrives. The interval is the intent's, or 15 days where the commander set none. It is cut to the ships' room, and to the source's stock above its reserve and above what runs still loading there have yet to take, those added earlier the same morning included. It is fixed when it is ordered. **Supply already aboard** (a free ship that was set aside, or one that came back with its cargo): the run carries the larger of that load and what is aboard, and takes only the difference. The tons to take are the load less what is aboard, not below 0, cut to the source's tons above its reserve and to the ships' room; the run's load is what is aboard plus those tons. Nothing is put back ashore. A load order is given for the difference only, to a weight that is what is aboard plus those tons, and none is given when there is nothing to take: the supply aboard is the load, and the reason says so ("the run carries what is aboard and no load order is needed") and, where the sum's load is less than what is aboard, says that too. [0107, ruling 22]
   - **Its days**, in whole days: loading takes the hours of the load at the source's berth rate (the lighter rate for a ship with no berth), rounded up to days and at least 1; the passage each way takes the hours of the charged sea distance at the class's cruise speed, rounded up to days; unloading as loading, at the destination's rate. It loads today, sails after its loading days and arrives on its sail day plus the outward passage less one. Its ships are free on the morning after the day they have finished fuelling at the source (decision 0112): the hours are counted from the first hour of the sail day, the passage out and the unloading in whole days, the passage back at cruise, then the fuelling, of what each ship then lacks (its capacity less the fuel it holds now and less the fuel the round trip burns, as in Its fuel), at the source's fuel berth rate where the source has a berth and its fuel lighter rate where it has none, the longest ship's hours. The free day is the day that hour falls in, plus one, and never earlier than the sail day plus the passages and the unloading in whole days. Where the source gives no fuelling rate (no limit) the fuelling takes no hours and the ships are free on the day after they are back at the source, as before. The hour a force steams on from the destination does not move these days. [rulings 6, 8 and 9]
   - **Its route.** The engine's sea path out and back. Where an area limit is set and the path leaves it, waypoints are added along the shortest sea path inside the area until the engine's path between them stays inside; if there is no such path, there is no route. With no area limit the sea is open.
   - **Its fuel.** Each ship needs, in the fuel it holds when the staff is asked (the fuel as it stood at the end of the day before: nothing is refuelled before the staff is asked), the fuel of the round trip at cruise.
   - **On the day,** the staff hands in `add_run` and `commit_ships` (from today to the day before the ships are free), and orders: a forming order at the source where its ships lie in port in no force (the new force's number is the view's `next_task_force_id`, in the order intake forms forces), a join where they lie in more than one force, and, where there is anything to take, a load to a weight at the source, the weight being what is aboard and the tons to take. The `add_run` records the run's whole load. [ruling 7, ruling 22]
5. **When it cannot.** It says which binds: no ship free until a day, the source's stock, no route in the area, a ship without the fuel, or the use more than the pool can carry. It gives the shortfall in days at the reading it checked: the floor less the figure of the day after the run would arrive, with what the run sends counted as landed, and not below 0. The source binds when there is nothing aboard and nothing to take. It gives the day the source falls to its reserve at the present rate (its stock above its reserve over the planned use of the destination and its own). With less stock than the load it sends what it can and says how many days it brings: it slips a run before it accepts fewer days, and accepts fewer days before it stands a plan down. [0107 items 25 and 44, ruling 14]

   **Displaced demand.** An intent that needs a run and has no ship free, while a ship of its pool is held by another plan, or whose load is cut by the ships' room while one is, or by the source's stock while tons there are still to be taken by another plan's run, is displaced. Its reason, and the reason of a run it sends cut, names its shortfall and each commitment that holds a ship of its pool or the tons at its source, its own plan's among them: for ships, the ships, the plan, its run and the days from the commitment's first to its last (`ship:1 committed to run 1 of plan:1 (intent:1) from day 1 to day 4`); for tons, the tons, the source, the plan and its run, from the run's load day to the day before it sails. It is said whichever of the two intents ranks higher: the earlier commitment stands. Where every commitment that holds them is the intent's own plan's, nothing is displaced and none is named. [ruling 14]

   **What the hold took.** Where the choice of ships for a run, made again with no hold, would take a ship the real choice did not because it is held, the staff says so: the reason of the run it gives, or of the day it gives none for want of a ship, says "Without the hold, ship:1 would have been sent.", names the hold in its evidence and in what bound it, and gives as what would change it that the target is released, when it would then be free. [0107, ruling 21]
   **Cannot meet.** The staff hands in a cannot-meet notice for the plan on a morning it weighs a new run (no run of the plan is loading and the intent needs one) and: (a) it can have none, because something binds (step 5 above); the notice rests on the day's reason for giving no order (step 6), and a day on which a loaded run is also dispatched gives that reason too, with its working beginning "No new run:", so that the notice has a reason about the new run and not about the dispatch; or (b) it orders a run whose load is cut and the reading it checks, with what the run sends counted as landed, is still below the floor (the shortfall above is more than 0 days): the notice rests on the reason of the `add_run`. A load cut that leaves the reading at or above the floor gives none. The staff weighs no new run, and so hands in no cannot-meet notice, for an intent with no forces limit, a base that is not the side's, a pool with no ship afloat, a pool whose every ship afloat is protected for another intent, a plan that has ended, or a day on which a run of the plan is still loading or waits to sail. [0107, ruling 35]

6. **A day it gives the intent no order** has one reason: what each run is doing, and the reading the next run waits for, with the day it is expected to be ordered, and the hold's words where a hold is what kept a ship from a run. A loaded run that waits in a plan that is preparing says that what would change it is leave to launch, or the commander's approval of the plan; a notice, waits for leave to launch, rests on that reason. A day the staff dispatches a loaded run and a new run is needed and can be had by none, the reason is given as well, and its working begins "No new run:" (Cannot meet, above).

First, before any intent is served, a plan that has ended whose ships are committed from today on has them freed (`free_ships`), so that an intent served that morning may commit them: the staff reads them as free, and the engine refuses a commitment that meets one still standing. (Until doctrine `supply-7` this was done last, after the intents; the soak ashore found the refusal it made, decision 0093, item 10.)

Every order, staff command and decision to give no order has a reason of five parts (above, Reasons). Its "worked" part is the arithmetic: the stocks, uses, days, loads and hours.

A side sees: its own staff's decisions, reasons and commands whole; the enemy sees none (above).

### Leaves

The two leaves are as they were: leave to prepare is given and leave to launch withheld unless the commander sets them. A plan that is preparing forms and loads its first run and sails nothing. It is released by the commander's approval, or by the staff the same morning where leave to launch is given. A run that waits is not sized again when it sails: its load was fixed when it was ordered, and it is timed again, and its ships' commitment renewed, on the day it sails. [0107, ruling 24]

### Cancelling

`cancel_plan` ends the plan (Plans, above): its commitments are freed from that day, and the intent stays in force with no plan until the commander sets it again or withdraws it. The staff reads every run of the cancelled plan as finished and gives the plan no more runs. A force of the plan that is still on its way out is given a way home, **once**: for each run of the plan, not ended, that has sailed (its ships are away from the source) and whose ships afloat are all in one force, if the force's remaining route still holds the intent's destination and the force is not stopped there, and the commander does not hold it, the staff gives it a route to the source, by the engine's sea path inside the area (the source's hex alone where there is none), with no mark to act at, and a reason that says the plan is cancelled and the force is sent home. It gives no new cargo order: the cargo stays aboard and the force's standing unload order stands, so the cargo is put back ashore at the source. A force that has begun to unload at the destination, a force still loading at the source and a force whose remaining route no longer holds the destination are given nothing, and one that has been sent home is not sent again, since its new route does not hold the destination. Where it gives such an order it gives no other decision for the intent that day. [0107, rulings 24 and 29]

### A proposal

What the staff would do for one intent, worked out and stored nowhere: what it would hand in on the coming day as if the plan were released, the runs the same rules give over the next 30 days on the projection, the projection itself, what binds if it cannot meet the intent, and a reason. It is worked in the day's priority order: the intents before it are served first as the staff would serve them that morning, so what they would commit is committed for it; then it is served; then the intents after it, with what it would commit. So it says both what it is displaced by (the commitments of the plans served before it that hold its ships or its tons) and what it displaces (each intent after it that it would displace, with that intent's shortfall and the commitments of this plan that cause it). It begins as the day does, with the runs set aside for a held ship (above) ended and their ships freed, so it shows the morning the staff will have, and the load of each of its runs counts the supply aboard its ships. The runs of its 30 days count as free only the ships free today: a ship held, under way or given to a higher intent that morning is not offered to its later runs. [0107 items 5 and 46, rulings 14 and 22]

## Holding a place

The end "hold a place" and the doctrine that serves it. The staff runs it after the supply doctrine, on the same morning, at the step where a staff is asked (The staff, above). It hands in nothing for a side with no hold or cover intent in force, so every game without one is as it was. It draws no random number. [0117 items 12 to 15]

**The end.** A hold intent names a base of the side. It has no date: it stands until withdrawn. It is given to the base it names (The intent tree, above). It is a root: a commander states it by `set_intent` with the kind `hold`, and nothing may be formed above it. Under it the staff may form cover from the air (Covering a base from the air, below), and nothing else in this step. Its limits are read as any intent's are: with no forces limit anywhere it may use the side's air groups based at the base, and it needs no source. [0117 items 2, 8 and 12]

**Its plan and the leaves.** A hold intent's plan, like that of a cover intent stated as a root, is drafted when the intent is first set and is made ready and released by the same leaves as a supply plan, by the staff's `approve` command. For each such root in priority order (the rank read up the tree, ranked first, then by number):

- A plan that has ended: one reason for giving no order ("the staff forms nothing and gives no order under" the intent), and no notice.
- A draft: with leave to prepare the staff approves it, to preparing, with a reason; without it, a reason that the plan waits for the commander and a `waits_for_leave_to_prepare` notice resting on it.
- A plan that is preparing (one the staff has just prepared this morning included): with leave to launch the staff releases it the same morning, with a reason; without it, a reason and a `waits_for_leave_to_launch` notice.

The staff forms nothing and orders nothing under a root whose plan is not released, and the doctrine below reads a plan it has just released as released the same morning. [0117 item 9; 0107 items 21 and 22]

**Each morning, for each hold intent in force whose plan is released,** in priority order, the staff does the first of these that applies, and gives the reason:

1. **A cover stands.** Where a cover intent for the same base is in force, whoever made it and wherever it stands in the tree, the staff forms none. It hands in a decision to give no order whose reason names the cover that stands and who made it. This is the reason of every morning after the first on which the staff has formed cover.
2. **Nothing at the base to cover it with.** Where the intent may use no air group at the base that is a fighter group or a group that can search (Covering a base from the air, below; a group that is held counts here), the staff forms none. Its reason says there is nothing at the base to cover it with, and gives what would change that: a fighter group or a group that can search at the base, or in the forces limit.
3. **Otherwise** the staff forms cover from the air under it: it hands in a `form_intent` staff command (parent the hold intent, kind cover, the same base) with a reason that says the base is held, its plan is released and no cover stands. The command passes intake as any staff command does (The intent tree, above). The cover doctrine runs for the new sub-intent on the same morning, on the state as the formation will leave it, so its orders are handed in the same morning and name the new intent.

[0117 item 13]

**Its measure.** A hold intent is met on a day the base is the side's at the end of the day and its airfield is open. The measure of the intent is worked by the agent interface from the side's own view and nothing else, stored nowhere, so the same view gives the same measure after a save and a resume (`sim_agent::hold_measure`). It gives:

| Part | What |
| --- | --- |
| Days in force | The days from the day the intent was set to the last day resolved, both counted; or, for an intent withdrawn, to the day before it was withdrawn. 0 where the intent was set after the last day resolved |
| Days met | Of those days, the days on which the side's own events show the base held at the end of the day (`BaseAtDayEnd`) and its airfield open. A base has its airfield open on a day if it has an airfield now (and, where the side's events show it built, that it was built by then) and the damage to the field at the end of that day is below `ground.toml: airfield_landing_refused_at` (the last `AirfieldDamaged` or `AirfieldMended` event of the day or before it; where the side's events show none, the base's damage as it stands). A side's events of a base are its own only while the base is its own, so a base that was lost shows none of the days it was held before |
| Air raids | The daylight pulses in the days in force in which the base was bombed, from the bombs the side's own events show falling on it |
| Raids fought | Of those, the pulses in which a patrol of the side's, flown by a group now based at the base, fired |
| Days of supply | The base's days of supply today (Days of supply, above), or none where the base is not the side's or the hex has no figure |

It is shown in `command_state` (as `measure` on the hold intent) and in `report` (as `holds`), with the words "held on N of M days in force (the base the side's at the end of the day with its airfield open); N air raids, N fought by a patrol of the side's; N days of supply at the base today". It changes nothing and no rule reads it. [0117 items 14, 0116 item 2]

**Supply under it waits.** Supply runs under "hold a place" are not built: a commander who wants a held base fed states "keep supplied" beside it. [0117 item 15]

**A proposal.** `propose` for a hold or a cover intent runs the air doctrine for the tree of its root as if its plan were released, and keeps what the doctrine hands in for that tree: the sub-intents it would form, each air group it would order with its order, the arc with its threats, the decisions to give no order, and one reason. It has no projection and no runs. [0117 item 14]

A side sees: its own `StaffReason`, `StaffNotice`, `IntentFormed`, the `CommandTaken` of each staff command and the measure, whole. The enemy sees none of it. [0117]

## Covering a base from the air

The task under the end "hold a place", and an intent a commander may state himself, as a root. The doctrine below is the whole of it. [0115]

**The intent.** A cover intent names a base of the side. It has no date. It is given to the base. The staff forms it under a hold intent (Holding a place, above), and a commander may state it himself by `set_intent` with the kind `cover`, when it is a root with a plan of its own, made ready and released by the leaves. Where it is the staff's, it belongs to its root's plan. [0115 item 1; 0117 items 4, 9 and 12]

**The groups it serves.** Each morning, for each cover intent in force whose plan is released, in priority order (the rank read up the tree, ranked first, then by number), the staff serves the air groups that:

- the intent may use (The six limits, above): named in its forces limit read up the tree, or, with none, the side's groups at its base;
- are at the intent's base (a group named in a forces limit that is elsewhere is given no order);
- are not held, neither as a whole nor by their search arc (Holds and the override, above); and
- are not taken by a cover intent served earlier the same morning: a group serves one intent, and where two may use it the higher in priority has it.

They are taken in group number order. The doctrine gives an order only where what a group has differs from what the doctrine wants, so a morning on which nothing changed has no order for a group. Each order is a staff order for an air group, serves the cover intent, and passes intake as any staff order does (The forces limit on a staff's orders, above). [0115 items 2 and 3, 5 to 11]

A group is a **fighter group** if its aircraft type's role is fighter; a **search group** if it is not and its type has a search range above 0 (`search_range_nm`); otherwise it is neither. A group's **fatigue** and the number of its aircraft **able to fly** are the side's own view's: the engine's count of the aircraft that are ready, crewed and on a field and deck that let them fly, and the group's fatigue now (chapter 5, What a group can fly in a pulse).

**The risk level** is the cover intent's risk limit read up the tree, balanced where none is set. It gives two figures:

| Risk | Share of its aircraft a fighter group keeps over the base | Fatigue above which a group rests |
| --- | --- | --- |
| Preserve the force | 3/4 | 30 |
| Balanced | 1/2 | 45 |
| Press | 1/4 | 60 |

[0115 items 4, 5 and 8]

1. **Fighters fly the patrol.** A fighter group that does not rest (rule 4) is set to the combat air patrol mission, with the patrol share the risk gives (chapter 5, Missions and orders and Escorts and patrol), and the rest of its aircraft are its escorts. The staff orders the mission if it is not combat air patrol and the share if it is not the share wanted.
2. **The share is raised.** On a morning when the base was bombed on any of the three days before the day being resolved (as the side's own events show: the `BaseAttackedAgainst` events for the base), the share is raised by 1/4, and not above 1. The reason names the day.
3. **Groups that can search, search.** A search group that does not rest is set to the search mission and to a search arc from the arc of rule 6, at its type's search range. The staff orders the mission if it is not search and the arc if the group's arc (from, to and range) is not the one wanted.
4. **Rest.** A group rests for a day, by the rest mission, when its fatigue in the morning is above the level its risk gives (a fatigue equal to it does not rest). The order is given only where the group's mission is not rest already. Fighter groups are taken most tired first, then by number. A fighter group above the level rests if it has no aircraft able to fly, or if another fighter group that can fly is still not rested; **the last fighter group that can fly is never rested**, so the base is not left without a patrol while a fighter group can fly. A search group above the level always rests: it has no such exception. The engine's own rest threshold is not set by the staff. [0115 item 8]
5. **A group that is neither** is given no order, and the reason says it is left for a strike intent.
6. **The search arc.** It points at what the side knows. The threats are the hexes of the enemy's bases on the side's map and the estimated hexes of the contacts it holds. With none the arc is the whole circle (from and to both 0). Otherwise it is the narrowest arc, on a grid of ten degrees, that holds every threat as seen from the base's hex by the engine's own test of an arc (`bearing::in_arc`): widths of 10 to 350 degrees in steps of 10, and for a width every start from 0 to 350, the lowest start first, the first that holds every threat taken. It is ordered widened by 30 degrees at each end (from the start less 30, to the end plus 30, each modulo 360), and as the whole circle where the width and the 60 degrees come to 360 or more, or where no arc under 360 holds them. [0115 item 9]
7. **When it cannot.** On a morning when none of the fighter groups the doctrine serves for the intent has an aircraft able to fly (which includes the case that it serves none at all), the staff hands in a decision to give no order whose reason says it cannot meet the intent, with why, and a `cannot_meet` notice that rests on it. The standing interrupt `cannot_meet` of the commander fires on that notice (Interrupts, above). [0115 item 10]
8. **Reasons.** An order's reason says the group, its kind, its fatigue against the level of the risk, what the group has and what is wanted, and, for the share, the share and what raised it; for an arc, the threats and the arc they give, and the contacts the group's arc was set against that the side has since dropped. A decision to give no order is handed in on a morning on which a group of the intent got no order or the intent cannot be met: it lists each such group with its fatigue, mission and setting, the share, the level and the arc, and, where it cannot, the sentence that it cannot. [0115 item 11]

A cover intent's limits narrow as the tree says (The intent tree, above): a forces limit on the staff's cover intent names only groups its hold intent may use, and a risk limit is no bolder than its hold intent's.

**The cost of search.** The engine sends every aircraft of a search group that can fly in both daylight pulses, and a long search tires the group. The doctrine does not send part of a group. [0115, What it leaves for later steps]

A side sees: its own orders, their acceptance or refusal, reasons and notices, whole. The enemy sees none of it. [0115]

## The tools

A seat whose side has a staff commands through these tools beside the others, in `sim-agent` and through the MCP server and `play`. Every seat of a game those open has one; the staff does nothing for a side until an intent is set, and for a supply intent until it has a forces limit. A session made with no staff has none of them. The agent interface is version 8 (version 7 had the four tools of step 4, and `interrupts` in `command_state`; version 8 adds the intent tree, the kinds `cover` and `hold`, air groups in a forces limit, a hold on a whole air group, a reason's chain and the measure of a hold intent). [0107 items 46, rulings 25 and 36 to 37; 0117]

- `command_state`, `plans`: what stands, as the coming day will find it, with each intent's figure and first day below, and yesterday's staff decisions with their numbers for `why`. `command_state` lists every intent, each root by number with the intents beneath it after it, and gives each its kind and base, its `parent`, `maker`, `addressee`, `children` and `depth`, its `limits` as set on it and its `limits_read_up` (each of forces, risk, area and priority as the staff reads it, with the intent it is read from and, where none is set, what then holds), for a hold intent its `measure` (Holding a place, above), and notes for a supply intent given to the area with no forces limit and for each intent the staff formed. It gives each intent in force its place in the priority order (`served`, by each intent's own rank, not read up the tree) and what displaces it today (`displaced_by`), and the side's `holds`: each with its target, the day, whether a `hold` command or an order made it, and that order's number. It gives too the side's `interrupts`: each interrupt set with its base, its number of days, the day it was set, whether it is armed, the day it last fired, whether that is acknowledged and whether it needs acknowledging; each standing record that has fired, by its kind and its plan or ship; and the kinds switched off. Those fired and not acknowledged come first.
- `set_intent` (the side's next free number when none is given; `intent_kind` is `keep_supplied`, the default, `hold` or `cover`, and `floor_days` and `interval_days` belong to `keep_supplied` alone), `withdraw_intent`, `set_limit`, `set_leave`, `hold`, `release`, `approve`, `cancel_plan`, `set_interrupt` and `acknowledge`: each queues one command for the coming day, taken in at its first step. It is checked at once, by taking it in on a copy of the world after the commands already queued, and answered queued or refused with the engine's reason. `set_limit` forces takes `ships`, `task_forces`, `air_groups` and `source`, each optional, and `set_limit` works on any intent. `hold` and `release` name exactly one of a ship, a task force, or an air group, which is held as a whole with no setting and by its search arc alone with the setting `search_arc`, in the side's own numbers, and a number that is not the side's is refused as the engine refuses it. `set_interrupt` takes `interrupt` (the side's next free number, one past the highest ever set or queued, when none is given), `base` and `below_days`; or `standing` (`cannot_meet`, `waits_for_leave` or `ship_lost`) and `on`; a table that mixes the two is refused. `acknowledge` takes `interrupt`, or `standing` with one of `plan` and `ship`, in the side's own numbers.
- `preview`: what a changed intent or limit would do, for `set_intent` and `set_limit` only, worked on two copies of the world as the coming day will find it (the side's queued commands taken in, and the same with the changed command last). It stores nothing: no state, no event, no hash, nothing queued. It sets what the staff would do on the coming day for the intent as things stand beside what it would do with the change (its orders, commands and decisions to give no order, the next run and the runs over 30 days, the first day below the floor and what binds), and says which ships are held and so not free, what each run still in hand would do (a run that has sailed goes on to the base it was sent to and lands there; a run still loading is sent to the new base; a change that is not of the base leaves every run as ordered), what is lost that was made ready (nothing: stock aboard a ship stays aboard and is carried by its next run, and stock landed stays where it lies), and what other plans give up. A command the tool would refuse is answered as the refusal and is not numbered. [0107, rulings 25, 27 and 28]
- `approve` with a `preview` number in place of a plan queues exactly the command that preview showed, answered as any queued command is. A preview is **void** once the side has queued an order or a command, or emptied its queue, or a day has been resolved, since it was made; approving a void preview is refused and says why (what was queued after it, or that a day was resolved). It is the session's, not the world's. [0107, ruling 26]
- `propose`: the staff's proposal for an intent, with what it is displaced by and what it displaces (`displaced_by`, `displaced_intents`, and in words `displaces`). For a hold or a cover intent it also gives the `tree` the staff would form (Holding a place, A proposal): the sub-intents, each with its number as the side's next free numbers give it, parent, maker and reason; the air groups each intent would order with the order and its working; the search arcs with their threats; and the decisions to give no order. It is null for a supply intent.
- `staff_orders`: what the staff would hand in at the start of the coming day, each with its reason, asked on a copy of the world with the day's first three steps taken (`World::at_staff_step`, which takes the override too). Only the session holds that copy; the staff and the tools are given its side's view.
- `run`: resolves up to `days` days (a whole number, at least 1), each exactly as `end_day` does, with the orders and commands queued going in on the first day. It stops after a day on which any interrupt of the side fired, or at the end of the game, or at its number of days, whichever comes first. The clock stops at the end of the day of a firing, not before it: nothing is ordered inside a day and the commander acts the next morning. It gives the staff's orders for the first day, worked before that day is resolved as `staff_orders` works them; the days resolved; why it stopped (`interrupt`, `game_over` or `days`); and everything that fired on the last day, each with its day, its event number, whether it is acknowledged and the `why` numbers of the reasons its notices rest on, with the result when the game is over. It takes an optional `reason`, which must be text and changes nothing. It is offered only where exactly one seat of the game is held by a player; with none or more it is not among the tools, and a call is refused, saying to use `end_day`. [0107 item 29, ruling 36]
- `report`: the command report, with no argument. It is worked from the side's own events and stored nowhere, so it is the same after a save and a resume. Its matters: a **matter** is a plan, a run where there is one and a kind of notice (the two leaves are two kinds), as the staff handed it in. It is open on the day last resolved if its notice was handed in that day, and closed otherwise, from the day after its last notice; handed in again later it is the same matter, with a gap in its history. Each matter gives its plan and intent, run, kind, whether it is open, the day it was first handed in and the day it closed, and its history: one entry for each day a notice of it was handed in, with the `why` number of the reason that day's notice rests on, so that `why` reads each day's reason. Open matters come first, then closed ones, each by the day first handed in, then plan, run and kind. The report also gives the interrupts as `command_state` does, the firings of the day last resolved, `holds` (the measure of each hold intent in force, with its intent, its base and its words) and a summary in words, which ends with a sentence for each hold intent. [0107 item 30, ruling 37; 0117 item 14]
- `why`: one `StaffReason` by its event number, whole, with the order or command it is for, or as a decision to give no order; its `serves` is the statement in words and its `chain` lists the intents above the one served, nearest first, each as "under intent N (the commander's)" with its statement, the last the commander's own. `staff_orders` gives the same chain in each reason's `serves`.

A tool is not a rule: what it shows is worked from the side's view, and the agent's schemas say each answer's shape (`crates/sim-agent/schemas/`).

## What is saved

A game is saved as it has always been, as what was handed in, and from version 3 of the saved game it holds what step 1 of stage 1 adds (chapter 1, What a saved game holds); version 4, since step 2 of stage 2, adds the intent tree to the commands and the hand-ins (below): the commander's commands by day, and each side's staff hand-in by day, whole. A hand-in is the orders with their intent and reason, the staff's commands and the decisions to give no order with their reasons, and, since step 4, the notices (each its kind's name, its plan, its run or none, and the reason it rests on, as the number of a command or of a decision to give no order of the same hand-in). A hand-in with no notice is written with no `notices` key, as it always was, and a hand-in saved without the key reads as having none; the saved game is still version 3. Only the days and sides that handed in something are kept. The saved game also records the doctrine version beside the rules label. Version 4 writes in a command `set_intent` with `intent_kind` (`cover` or `hold`; left out for keeping a base supplied), the staff's `form_intent` (`parent`, `intent_kind`, `base`), a forces limit with `air_groups` and a `source` that may be left out, and a hold or release of a whole air group (an `air_group` and no `setting`), and in a reason's `serves` the `chain`. A saved game of version 3 or before is refused by the reader, not converted. It holds no command state: the state is rebuilt by taking the commands in again, the commander's `hold` and `release` among them, and the staff's `end_run` among its hand-in, and a commander's `set_interrupt` and `acknowledge` among the commands. It holds no interrupt's status: it is rebuilt by taking the commands in and testing each morning again, and a replay hands the saved notices in as the staff gave them. It holds no override either: the commander's orders are saved as handed in, and the override is taken again when a replay takes them in at step 2. [0107 items 9, 11 and 12]

Three versions name what a saved game was played under. The state's layout was `flagplot-state-v26` from step 4, `flagplot-state-v27` from decision 0114 (an air group's two strike settings, chapter 1, The state hash and saved games) and is `flagplot-state-v28` since decision 0117 (an intent's parent, maker and addressee after its kind, the kinds cover and hold, a forces limit's air groups and its source as optional, a risk limit that may be unset, and the hold of a whole air group) (a side's interrupts: those set, the standing records that have fired and the kinds switched off, written after its holds; `flagplot-state-v25` before it, since step 3, wrote a side's holds after its commitments and a run's day of ending after its free day). The rules label was 10 from step 3 and stayed 10 in step 4 (it is 12 since decision 0114, and the intent tree did not raise it: no game without a hold or cover intent plays differently): a commander's order to a unit inside a forces limit holds it and the staff's saved orders to it are refused (step 3), so a game saved under the old rules in which the commander ordered such a unit could play differently, and is refused by `replay` before it runs. An interrupt and a notice change nothing a unit does, no order, stock or chance, and so no game saved under label 10 plays out differently for them: a replay hands in the saved orders, commands and hand-in, and the interrupts are tested on what that game has. What they add to a game's log is events (a notice, a firing), in a game that hands in a notice or sets an interrupt. A game saved under layout v25 is refused by its layout version. The doctrine version was `supply-4` then, and is `supply-8` since the air doctrine (`supply-7` from the soak ashore, decision 0093, item 10; `supply-6` from decision 0112) (The doctrine version, below). The saved game is version 4. [0107, rulings 23, 30 and 31 to 35; 0115; 0117; chapter 1, The state hash and saved games]

**Replay.** A replay hands the saved orders, the saved commands and, before each day, the saved hand-in of each side into the engine (`World::submit_staff`), and asks no staff. So a replay of a staffed game works when the doctrine has since changed, and gives every daily hash that was saved. [0107 item 9]

**`replay --check-staff`.** It asks the doctrine's staff as well. On each saved day, each side that the doctrine gives a staff is asked on the view the engine then holds, and what it gives is compared part by part, orders, then commands, then decisions to give no order, then notices, with what was saved. The saved hand-in is what is handed in, whatever the staff gave, so the replay is the saved game and its hashes are still compared. The run stops at the first day and side where they differ and says what differs. A saved hand-in for a side the doctrine gives no staff is a difference too. A difference in the staff is reported before a hash that differs on the same day. If there is none, it says so, naming the doctrine now and the one the game was saved under. It is the test that a staff is a function of the view. [0107 item 10]

**The doctrine version.** The doctrine has a version of its own, `DOCTRINE_VERSION` in `sim-staff` (`"supply-8"` from step 2 of stage 2, decisions 0115 and 0117: the air doctrine after the supply doctrine, which serves keep-supplied intents alone, for hold and cover intents, and a side with none gets what `supply-7` gave; `"supply-7"` from the soak ashore, decision 0093 item 10: the ships of an ended plan are freed first in the morning, before any intent is served; `"supply-6"` from decision 0112: a ship is free on the morning after the day it has finished fuelling at the source, by the hours of the fuelling (Its days), and its fuel is read as it stood at the end of the day before; `"supply-5"`: each reason is handed in with its intent's statement in words; `"supply-4"` from step 4 of stage 1: the notices, cannot meet, waits for a leave and a run ended, and the reason a day gives that begins "No new run:"; `"supply-3"` from step 3 of stage 1: holds, runs set aside, the supply aboard a free ship, the fallback of a cancelled plan and where a run lands in the projection; `"supply-2"` from step 2, priority between intents and displaced demand; `"supply-1"` in step 1; `"none"` before a doctrine existed). A change to what a staff decides from a view changes it. A saved game records it. It is checked when a game kept by `play --game` is resumed for more play, which is refused under another doctrine, naming both the version the game was begun under and the one the program has, and not when a game is replayed: a replay is what was saved. A `play --game` file begun before the key existed is read as `"none"`. [0107 item 11]

Where the staff comes from. Every loop that resolves days gets its staffs from the one place, `sim_staff::staffs()`, and passes them in: `run`, `batch`, `export`, `hash`, `events`, `time`, `replay --check-staff` and a game held open for play. `sim-agent`'s session is handed them by its caller. [0107 item 7; docs/engineering/layout.md]

A side sees: none of this. A saved game belongs to the umpire (chapter 1, What a saved game holds).

## A scenario's commands

A scenario may state a side's commands by day, as it states orders, so that a test can begin with an intent in force. A `[[command]]` table has `day`, `side`, `kind` and the keys of its kind, and no other key. Only the commander's kinds may be written; the staff's are the staff's to give, `form_intent` among them: a scenario states a commander's end, as a root, and the staff forms what serves it. A scenario with no `[[command]]` plays as it did. [0107 item 16]

| `kind` | Keys |
| --- | --- |
| `set_intent` | `intent`, `base`, and `intent_kind`: `"keep_supplied"` (the default where the key is absent, so a scenario written before the kinds reads as it did), which also takes `floor_days` and `interval_days` (optional); `"cover"` (cover `base` from the air) or `"hold"` (hold `base`), which take neither |
| `withdraw_intent` | `intent` |
| `set_limit` | `limit`, and the keys of that limit, below |
| `set_leave` | `leave` (`"prepare"` or `"launch"`) and `given` |
| `approve`, `cancel_plan` | `plan` |
| `hold`, `release` | Exactly one of `ship`, `task_force` and `air_group`; with `air_group`, `setting` (`"search_arc"`) holds that setting alone and no `setting` holds the whole group; no other target takes a `setting` |
| `set_interrupt` | Either `interrupt`, `base` and `below_days` (a numbered interrupt set), or `standing` (`"cannot_meet"`, `"waits_for_leave"` or `"ship_lost"`) and `on` (`true` or `false`) |
| `acknowledge` | Either `interrupt`, or `standing` with exactly one of `plan` and `ship` |

The limits: `"forces"` takes `intent` and, each optional (a list not given is empty), `ships`, `task_forces` and `air_groups`, and `source` (a supply intent's forces limit must name one, another kind's need not); `"area"` takes `intent` and `hexes`, as `[[q, r], ...]`; `"priority"` takes `intent` and `rank`; `"protected"` takes `intent` and `ships`; `"reserve"` takes `base`, `supply_t` and `fuel_t`; `"risk"` takes `intent` and `risk`, `"preserve"`, `"balanced"` or `"press"`. A quantity is a whole number or a decimal string, never a float.

The loader refuses, when the scenario is read: a day that is 0 or after the scenario's last day; a key the kind does not take, or that is no key of any command; a missing key; a TOML float; a kind that is the staff's (`end_run` among them); an unknown limit, leave, risk or setting; a hold or release that names none or more than one target, or a `setting` with a ship or a task force; an `intent_kind` that is none of the three, or a `cover` or `hold` with a key only a supply intent takes; a `set_interrupt` or `acknowledge` that mixes the keys of its two forms; a standing kind that is none of the three; a `set_interrupt` with `standing` and no `on`; an `acknowledge` with `standing` and both or neither of `plan` and `ship`. Whether a base, ship, task force, intent, plan or interrupt exists, whether a number of days is 0 and whether an interrupt has fired are not the loader's to say: the engine checks it when the command is taken in, and refuses it as above. A scenario's commands are kept by day in the order written, kept when a seed is changed, and handed in before the day is resolved, as the commander's, so the engine takes them in at its first step. Two commands of one day for one side are taken in in the order written, and the Allied side's before the Japanese side's. [0107 item 16]

A scenario's `[[order]]` may give the issuer `staff` and an `intent`. Such an order is not checked against a forces limit or against the holds, and the override skips it (above, The forces limit on a staff's orders; Holds and the override). [0107, As built]

## The day's order

Chapter 1 owns the sequence. Where this chapter's rules run, in the numbering of decision 0107's table (which chapter 1 now numbers the same: the staff is asked at its step 4, and its commands, notices, interrupts and the orders intake are its step 5):

| Step | What happens | Where |
| --- | --- | --- |
| 1 | The commander's commands are taken in, `hold` and `release` among them | This chapter, Commands |
| 2 | The override: the commander's orders to units in an intent's forces limit (an air group an intent may use, among them) are taken in and hold them. In the code it runs after step 3 | This chapter, Holds and the override |
| 3 | Forces back from the yard are home. Ships in port are not refuelled here: they take fuel in the loading phase of each pulse, each base down to its reserve | Chapter 4, Repair; chapter 2, Refuelling in port |
| 4 | Each side's staff, if it has one, is asked on its view as it now stands. Then the days of supply of the base of each interrupt the commander set are read on that same view | This chapter, The staff; Interrupts |
| 5 | The staff's commands are taken in (the runs set aside first; the air doctrine's after the supply doctrine's, with `form_intent` among them), then its decisions to give no order are recorded, then its notices; then each side's interrupts are tested, on those readings and those notices; then all orders not already taken in, the commander's and the staff's | This chapter, The staff; Notices; Interrupts; chapter 1, Orders |
| 6 | The rest of the day, as it was | Chapters 2 to 7 |

[0107, The order of a day; rulings 23, 30 and 31]

## Not built

- **A hold of the commander on a base,** and holds on a setting other than a search arc: they wait for a stage whose doctrine acts on a base or changes a setting. (A hold intent is an intent, and is built.) [0107, rulings 17 and 30]
- **Interrupts of other kinds, and the withdrawal of a numbered one.** The one set kind is a base's days of supply below a number. A numbered interrupt cannot be withdrawn, only set again with another base and number. [0107, ruling 33]
- **A notice on every day the staff weighs no new run.** A cannot-meet notice is handed in only when the staff weighs a new run for the plan: none is handed in for an intent with no forces limit, a base that is not the side's, a pool with no ship afloat or one whose every ship afloat is protected for another intent, a plan that has ended, or a day a run of the plan is still loading or waits to sail. Those days give a reason, or a plan marked failed, and no notice. A preview carries no notice. A run-ended notice is handed in only for a run set aside for a hold: a run of a cancelled plan, which the staff reads as finished, has none. [0107, ruling 35]
- **A second source base** for an intent. [0107 item 19]
- **Escort, the recall of a run, an intent for fuel and a phase of a plan**: stage 2. [0107]
- **Ships covering a place, bombers and fighters sent as escort on purpose** (the strike intent), and a hex of sea as the subject of an intent. The cover doctrine orders air groups at a base and no ship. [0115, What it leaves for later steps]
- **Part of a search group flown,** and searching where nothing is known but something is feared (the area limit's to say). The arc points at what is known. [0115]
- **A task force or a land formation as the addressee of an intent,** a commander stating a sub-intent under a parent he names, a commander taking a staff-made intent over, the staff withdrawing a sub-intent because what it needs has gone (the cannot-meet notice serves), a ranking of siblings stated and tested beyond the one kind of sub-intent built, supply runs under "hold a place", and friction between the levels (a delay in passing an order down, a subordinate's narrower view). [0116 item 19; 0117, What it leaves out]

## Not in this chapter

- How a command's refusal and an order's refusal differ for a player: the agent's tools, not rules.
- How orders are numbered, sorted and refused, and who can issue them: chapter 1, Orders.
- Refuelling, loading to a weight, reserves and forming, joining and standing down forces: chapter 2.
- Belief, contacts and claims, which a staff reads through the view: chapter 3.
- The men's supply, landings and the land units a base feeds: chapter 7. Aircraft upkeep, repair and sorties: chapter 5.
- The saved game's other contents and the state hash: chapter 1, The state hash and saved games.
