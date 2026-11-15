# The staff

This chapter holds what a side's commander says as state, the commands that change it, the staff that a side may have, the reasons the staff gives, and days of supply. It is written by step 1 of stage 1 of step 3: the engine's half (command state, commands, the staff passed in, days of supply) and the staff's half (the crate `sim-staff`, the doctrine for keeping a base supplied, the reasons of staff commands, the staff's projection of days of supply, and the tools a seat commands with). Every side of every game has the supply staff; a side with no intent in force has a staff that does nothing, so a game with no command is played exactly as it was before this chapter. [0107, 0111]

No rule here draws a random number. Every quantity is a fixed-point number (chapter 1, Numbers and chance).

## What is built and what is not

Built: command state (intents, six limits, two leaves, plans with their states, baselines, amendments and runs, and commitments of ships); eleven commands and who may give each; the staff passed in to the function that resolves a day, and what it hands in; the check of a staff's orders against a forces limit; reasons; days of supply; the saved game of version 3, `replay --check-staff` and the doctrine version; a scenario's commands; the doctrine for keeping a base supplied, with its projection of days of supply and its proposals; and the tools of step 1. [0107]

Not built, and so not rules: holds on a setting, a unit or a force, and the override of a held unit by the commander's own order; interrupts and their acknowledgement; a run of days that stops for an interrupt; the command report; commitments of stock beyond the loads still to be taken (below, The doctrine); previews; escort, cover and the recall of a run; a second source; an intent for fuel; the fallback of a cancelled plan beyond a route that already ends at the source; and a phase of a plan (below, Plans). Their places are marked in the code and in the day's order below. [0107]

## Command state

Each side has one command state, held by the world beside its belief and hashed after the beliefs, Allied side first (chapter 1, The state hash). It changes only by a command. The enemy has none of it, and a side is never told whether the other has a staff. [0107 items 1 and 6]

A side sees: its own command state, whole, through its view. It sees none of the other side's. [0107 item 6]

### Intents

An intent is what the commander wants. The one kind built is to keep a supply base supplied: it names a base, a floor in days of supply (at least 1) and, optionally, an interval in days (at least 1) between shipments. The floor is the days of supply the base's hex is never to fall below. The interval is the days of use each run is to bring above the floor. Where the commander gives none, the interval is not set and the staff proposes one. An intent has no date: it stands until it is withdrawn. [0107 items 17 and 22, question 5]

The commander names the intent's number when he sets it. It is a whole number from 1. A withdrawn intent keeps its number in the state and the number is never used again. An intent records the day it was first set, the plan that serves it and the day it was withdrawn, if it was. [0107]

An intent in force carries its limits as they stand. Setting an intent that is in force again replaces its kind, base, floor and interval together and leaves its limits as they are. [0107]

### The six limits

A limit says what the staff may use and where it may go. Five belong to an intent and one to a base. A limit that was never set does not bind, with one exception that follows from what it is: an intent with no forces limit gives the staff no ship, so it gives no order for it. With no area limit the sea is open. [0107; the architect's ruling, Nov 15, 2026]

| Limit | What it holds | What reads it |
| --- | --- | --- |
| Forces | The pool: a set of ships, a set of task forces, and exactly one source base for a supply intent | The check of a staff's order and of a staff command that names ships (below) |
| Area | A set of hexes the intent's forces may enter. It is not empty, and every hex is on the map. An area not set does not bind: the sea is open | The doctrine's route |
| Priority | A rank, 1 first, or none. An intent with none comes after every ranked intent, then by number | The order in which the doctrine serves intents |
| Protected | A set of ships the staff may not take from the intent | Nothing yet: with one intent no ship is taken from another. Stage 1's step 2 |
| Risk | Preserve, balanced or press. Balanced is the default | Nothing. Stored and shown, and read by no rule until a later stage |
| Reserve | For a base, a supply reserve and a fuel reserve, in tons, neither below 0 | Refuelling, and a load the staff orders (chapter 2, Refuelling in port and Cargo orders) |

[0107 items 18 to 20, 0111]

A ship is in the pool of a forces limit if the limit lists it, or if it lies in a task force the limit lists, afloat in it or away from it at the yard. A task force is inside the limit if the limit lists it, or if it has at least one ship and every ship of it is a ship the limit lists. [0107]

The reserve limit names a base and no intent. It writes the base's own two reserves (chapter 2, Bases on hexes): one store, not a second. A scenario's base entry gives the same two figures (`supply_reserve_t` and `fuel_reserve_t`). It is taken in at the first step of the day, so it binds that morning's refuelling. [0111, 0107]

### The two leaves

A side's commander holds two leaves for his staff. Leave to prepare lets the staff itself move a draft plan to preparing. Leave to launch lets it move a plan that is preparing to released. The defaults are the spec's for a player: prepare given, launch withheld. Only the commander sets a leave, and he may set either on any day. [0107 item 21]

### Plans

A plan is what the staff does to serve one intent. It is drafted when its intent's number is first set, and numbered in the side's own series, 1, 2, 3 and on, so a side's plan numbers depend on its own history only. A plan has one of six states:

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
- A plan that is not ended, when its intent is withdrawn: a released plan becomes complete and a draft or preparing one becomes cancelled. [0107 items 22 to 24]

Setting again an intent that is in force whose plan has ended drafts a new plan for it, with the next number, and the intent serves from then on. A plan that has ended with its intent still in force does not withdraw the intent. [0107]

Limit: decision 0107, item 23, says a plan's phase begins when it is released and ends when the intent is withdrawn. The code has no phase state. A plan's runs are tasks held in the plan, and its state is the one of the six above (differences.md row 42). [0107 item 23]

### The baseline and amendments

A plan's **baseline** is taken the first time it is approved: the day, the intent's kind and the intent's limits as they stand that day. It never changes. A plan that is not yet approved has none.

An **amendment** is a numbered change to a plan that is approved. While a plan is approved, a `set_intent` for its intent, and a `set_limit` of a forces, area, priority, protected or risk limit of its intent, apply to the intent as they stand and are also kept in the plan as its next amendment, numbered 1 and up, with the day and the change. A reserve is no amendment, because it names a base and no intent. Withdrawing the intent is none either: it ends the plan. A change made while the plan is a draft, or after it has ended, changes the intent and is no amendment: a draft's change enters its baseline when it is approved. [0107 items 22 and 25]

What the staff may change alone in a plan is its runs and commitments, inside the limits. A change to the days, the base or a limit, and cancelling, are the commander's. [0107, question 3]

### Runs

A run is one convoy of a supply plan: a task inside the plan's one phase. It has a number in its plan (1 and up), its ships, its load in tons of supply (above 0), its source, which is the source base of its intent's forces limit on the day the run is added, and four days:

| Day | Means |
| --- | --- |
| Load | The day loading begins |
| Sail | The day the force sails |
| Arrive | The day it is due at the destination |
| Free | The day its ships are free: back at the source and refuelled |

The days are in this order or equal. A run added has its load day no earlier than the day the command is taken in, and a run timed again has its free day no earlier than that day. A run does nothing by itself. The orders that carry it out, form the force, load it to a weight, send it and unload it, are the staff's to give on the days it names. [0107 items 26 and 27, rulings 7 and 8]

### Commitments

A commitment gives one ship to one plan, and to one of its runs if named, from one day to another, both days included. A ship holds at most one commitment on any day: a commitment that overlaps another of the same ship, whatever plan it is for, is refused. The staff commits ships and frees them. Freeing a ship ends every commitment it has to the plan. No rule of the day reads a commitment: it is the staff's record, and the check of the next commitment reads it. Commitments of stock are not built. [0107 item 27]

## Commands

A command is a change to the side's command state, handed in beside orders. It is checked against the side's own units and its command state, and it is accepted or refused with a reason, as an order is. A refused command changes nothing but the log. [0107 items 1 and 2]

### Who may give each command

| Command | Given by | What it does |
| --- | --- | --- |
| `set_intent` | Commander | Sets an intent, or changes one in force |
| `withdraw_intent` | Commander | Withdraws an intent in force and ends its plan |
| `set_limit` | Commander | Sets one limit |
| `set_leave` | Commander | Gives or withholds leave to prepare, or leave to launch |
| `approve` | Commander, or the staff with leave | Moves a draft to preparing, or a preparing plan to released |
| `cancel_plan` | Commander | Cancels a plan that has not ended |
| `commit_ships` | Staff | Commits ships of the intent's pool to an approved plan from one day to another |
| `free_ships` | Staff | Ends the commitments of ships to a plan |
| `add_run` | Staff | Adds a run to an approved plan, with the plan's next run number |
| `time_run` | Staff | Gives a run new days |
| `mark_plan` | Staff | Marks an approved plan complete or failed |

A command from the other issuer is refused as not yours. A staff may give `approve` only where the commander has given it the leave for the move: leave to prepare for a draft, leave to launch for a plan that is preparing. The commander's `approve` needs no leave. [0107 items 4 and 21]

The commander's commands are handed in with the orders for a day and taken in at its first step, the Allied side's first, each side's in the order handed in. The staff's are taken in after the staff is asked, in the order handed in, before any order. [0107, The order of a day]

### Every reason a command is refused

The reasons for one command are tried in the order written. The first that applies is the one recorded. A unit or a base that is not the side's, and one that does not exist, are refused alike, so a refusal never shows that an enemy unit exists. A ship is the side's own only if it is not sunk.

| Command | Reason | When |
| --- | --- | --- |
| Any | Not yours | The issuer may not give this kind of command. Tried first |
| `set_intent` | Intent number not usable | The number is 0, or it is the number of an intent that was withdrawn |
| `set_intent` | Not your base | The base is not one of the side's |
| `set_intent` | No days | The floor is 0, or an interval is given and is 0 |
| `withdraw_intent` | No such intent | No intent of that number is in force |
| `set_limit` | No such intent | The limit is of an intent (all but the reserve) and no intent of that number is in force. Tried first |
| `set_limit` forces | Not your ship | A listed ship is not the side's, in ship order |
| `set_limit` forces | Not your force | A listed task force is not the side's, in force order |
| `set_limit` forces | Not your base | The source is not one of the side's |
| `set_limit` area | Bad area | The area has no hex, or a hex off the map |
| `set_limit` protected | Not your ship | A listed ship is not the side's |
| `set_limit` reserve | Not your base | The base is not one of the side's |
| `set_limit` reserve | Bad weight | The supply reserve, then the fuel reserve, is below 0 |
| `approve` | No such plan | No plan of that number |
| `approve` | Wrong plan state | The plan is neither a draft nor preparing |
| `approve` | No leave | The staff gave it, and the leave the move needs is withheld |
| `cancel_plan` | No such plan | No plan of that number |
| `cancel_plan` | Wrong plan state | The plan has ended |
| `commit_ships` | No such plan | No plan of that number |
| `commit_ships` | Wrong plan state | The plan is not approved |
| `commit_ships` | No forces limit | The plan's intent has no forces limit |
| `commit_ships` | Bad days | The first day is before the day of the command, or after the last |
| `commit_ships` | No such run | A run is named and the plan has no run of that number |
| `commit_ships` | Not your ship, not in the pool, already committed | For each ship in ship order, in that order: it is not the side's; it is not in the pool of the forces limit; it has a commitment that overlaps the days asked for |
| `free_ships` | No such plan | No plan of that number. A plan of any state is found |
| `free_ships` | Not committed | A ship has no commitment to the plan |
| `add_run` | No such plan, wrong plan state, no forces limit | As for `commit_ships` |
| `add_run` | No ships | The run names no ship |
| `add_run` | Not your ship, not in the pool | For each ship in ship order, in that order, as for `commit_ships` |
| `add_run` | Bad weight | The load is not above 0 |
| `add_run` | Bad days | The days are not in order, or the load day is before the day of the command |
| `time_run` | No such plan, wrong plan state | As for `commit_ships` |
| `time_run` | No such run | The plan has no run of that number |
| `time_run` | Bad days | The days are not in order, or the free day is before the day of the command |
| `mark_plan` | No such plan, wrong plan state | As for `commit_ships` |

`set_leave` is refused only when the staff gives it. [0107 items 1, 4, 22 and 27]

The same intent number set twice. Two `set_intent` commands of one day that name the same new number are taken in in the order handed in: the first sets the intent and drafts its plan, and the second changes the intent that now exists. The plan is a draft, so the second is no amendment and drafts nothing. The intent is as the second left it. [0107]

### What a command does and records

A command that passes is applied, then `CommandTaken` is recorded with the whole command and its issuer. It has no cause: it is the root record of the change. A command that fails records `CommandRefused` with the command, its issuer and the reason, and has no cause. What an accepted command sets off is recorded after `CommandTaken` as an event that cites it:

| Event | Recorded when |
| --- | --- |
| `PlanDrafted` | An intent's number is first set, or an intent in force whose plan has ended is set again. Names the plan and the intent |
| `PlanStateChanged` | A plan changes state, by `approve`, `cancel_plan`, `mark_plan` or the withdrawal of its intent. Names the plan and both states |
| `PlanAmended` | A change is kept as an amendment. Names the plan and the amendment's number |

A `set_leave`, a `commit_ships`, a `free_ships`, an `add_run` and a `time_run` set off no other event: the command's own record, with the whole command, is the record of the change. There is no command number. Withdrawing an intent whose plan has already ended, and setting an intent that is a draft again, set off nothing. [0107]

A side sees: its own `CommandTaken`, `CommandRefused`, `PlanDrafted`, `PlanStateChanged` and `PlanAmended`, whole, and the enemy sees none of them. Each of these kinds is decided in `side_events.rs`. [0107]

## The staff

### What a staff is, and when it is asked

A staff is code outside `sim-core`. `sim-core` defines what it is: given a side's view, it decides the day and hands in a `StaffDecisions`:

- **Orders**, each with the intent it serves and its reason.
- **Staff commands**: the commands of the staff's column above, each with its reason.
- **Decisions to give no order**, each a reason.

A staff is a function of the view: it is asked with a shared reference, so that a staff asked twice on the same view gives the same answer. That is what `replay --check-staff` tests. The function that resolves a day takes each side's staff or none. A side with none is not asked, and a day with no staff and no command is resolved as it was. [0107 items 4, 7 and 8]

A staff is asked once a day, at its place in the day's order (chapter 1, Before the pulses): after the commander's commands are taken in and after forces back from the yard are home and ships in port have refuelled, and before any order is taken in. It gets the side's view as it then stands, so it sees the reserves and the leaves set that day and the stock after the morning's refuelling. It never gets the truth (chapter 1, Truth and belief). [0107, The order of a day]

What a replay hands in, in place of asking, is taken in at the same step as if a staff had given it. If a side has a live staff and something handed in for it, both are taken in, what was handed in first.

The staff's commands are taken in first, each side's in the order handed in, and after each side's commands its decisions to give no order are recorded. Then every order of the day is taken in: the commander's and the staff's together (chapter 1, How orders are numbered and sorted). A staff's order comes after the commander's to the same unit, so the staff's replaces the commander's in a standing slot. A staff's order is marked with the issuer staff and the intent it serves (chapter 1, Who can issue). A cargo order the staff gives heeds the reserve of a base (chapter 2, Cargo orders). [0107, 0111]

Every runner gives both sides the supply staff (`sim_staff::staffs()`). A staff whose side has no intent in force and no commitment to free hands in nothing: nothing is taken in and nothing is saved for that day. [0107]

### The forces limit on a staff's orders

A staff's order is refused as **outside the forces limit** ("the order is for a unit outside the forces limit of the intent it serves") unless all of these hold:

- the intent the order serves is in force and has a forces limit; and
- the order is for a task force that the limit lists, or that has at least one ship and every ship of it is a ship the limit lists, or it is the intent's source base's order to form a task force from ships that are in the pool.

An order for an air group, a land unit or any other base order is inside no forces limit in step 1 of stage 1, so a staff's order for one is refused. The check does not look at the plan's state or at the area. A refused order is numbered and kept, and recorded as refused with this reason, and nothing else about it is checked. An order that passes is checked as any order is (chapter 1, Every reason an order is refused). [0107, As built]

The check binds the orders a staff hands in. It does not bind an order that a scenario's script gives with the issuer `staff`: that order is a scenario's statement, set down by the scenario's author as a starting condition, and not a decision of a staff made on a view. Scenario 39 is such a scenario. Such an order is checked as any scripted order is. [0107, As built]

A staff command that names ships is checked against the pool too (`commit_ships`, `add_run`; above). [0107]

A side sees: its own acceptance or refusal of the order whole, with the reason. [0107]

### Reasons

Every order a staff gives and every decision it makes to give none has a **reason**. A reason has five parts:

| Part | Holds |
| --- | --- |
| Serves | The intent, and the plan and the amendment the decision was made under. The amendment is none for the baseline |
| Evidence | Each thing the side held that mattered, in words, with its day |
| Binding | The limit, stock, distance or commitment that shaped the decision, in words |
| Worked | The calculation, in words |
| Would change | What, if different, would give a different decision |

A reason is handed in with its order, with its staff command, or with the decision to give no order, and is recorded as a `StaffReason` event. For an order, the event cites the order and the event that accepted or refused it, and is recorded after them. For a staff command it has no order and cites the command's `CommandTaken` or `CommandRefused`, and is recorded after it and its follow-ups. For a decision to give no order it has no order and no cause. [0107; differences.md row 43] **A reason is kept, not worked out again**: it stays in the log whatever the intent has since become, so it still resolves after its intent is replaced or withdrawn. Its number is the number of its event: a reason has no number of its own. [0107 items 4 and 12]

The engine stores what the staff writes in a reason and checks none of it. Evidence names a side's own ships freely and the enemy only as a contact, a claim or an estimate (chapter 3, Belief): that is the staff's to keep, and a staff is given nothing else to name. [0107, truth and belief]

A side sees: its own `StaffReason` whole. The enemy sees none. [0107]

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

**When it is read.** At the step where a staff is asked: at the start of a day, after ships in port have refuelled and before the day's orders are taken in. Read between days it gives the same figure, since nothing between the end of one day and that step changes a stock or a use. An order given today shows in the figure of tomorrow: a group set to patrol today raises the planned use read the next day. [0107 item 31, ruling 3]

**The projection** of the figure over the coming days, and the first day it falls below an intent's floor, are the doctrine's (below, The doctrine). The engine gives it what it counts by its own rule: `SideView::men_due_by_day(hex)`, each force due in the hex with its due day and its men's charge, the rule the men-due part above uses. [0107 items 34 and 35]

A side sees: its own figure, with its stock, its planned use and the five parts, in the agent's `list_bases` and `get_entity` (as `days_of_supply`) and in the briefing's entry for each of its bases (as `planned_days_of_supply`). The briefing's entry for a base also keeps its older `days_of_supply`, which is a different figure: the stock over yesterday's spending, cut to one decimal (glossary, Days of supply). The enemy sees none. [0107 item 47]

## The doctrine: keep a base supplied

The supply staff (`sim_staff::SupplyStaff`) decides from its side's view alone, at the step of the day where a staff is asked. It holds nothing of its own: what it remembers is the side's command state (its plans, runs and commitments) and what the view shows of its ships. Asked twice on the same view it gives the same answer. It draws no random number and uses no floating point. [0107 items 3 and 37 to 45, rulings 4 to 9]

"Today" below is the day being resolved. A ship, a force and a base are named in a reason as the engine writes them (`ship:1`, `task_force:1000001`, `base:2`); the agent's tools show them in the side's own numbers.

### The projection

For the hex of the intent's base the staff projects a reading for each day from today. Today's reading is the view's (Days of supply, above). The reading of the next day is that day's reading, less its planned use, plus what lands that day, and never below 0. The planned use of a coming day is today's, with the men due counted from the day each force is due (`men_due_by_day`). What lands: for each run of an approved plan of the side to that hex that has not finished, its load while it has not sailed, else the supply aboard its ships, on the day after its arrival day, or today if that day is past. A reading's figure is its stock over its use; with no use it has none. The report of it is today's figure and the first day whose figure is below the floor. [0107 item 35, ruling 1]

### Each day, for each intent

The intents in force are served in priority order: ranked ones by rank, then those with none, each by number. For each:

1. **What stops it.** With no forces limit, no order and a reason. A destination or source base that is not the side's: an approved plan is marked failed (`mark_plan`), and no order. A plan that has ended: no order. A pool with no ship afloat: an approved plan is marked failed, and no order. The staff never marks a plan complete: a supply intent ends when it is withdrawn.
2. **The leaves.** A draft is approved to preparing when leave to prepare is given; else no order, and the reason says the plan waits for the commander. A plan that is preparing is released the same day when leave to launch is given. A plan that is preparing forms and loads its first run and does not sail it.
3. **The runs under way.** A run whose ships are all at the source, stopped, is loading, unless its sail day is past and nothing is aboard, when it has finished; a run with a ship away from the source is under way; a run whose ships a later run has taken, or whose ships are all sunk, has finished. A run loading whose load is complete (the supply aboard is its load, or the source has nothing more above its reserve, or its ships are full), after its load day, sails if the plan is released: a route to the destination and back to the source, marked to act at the destination, and an unload order. If its sail day has passed, it is timed again first (`time_run`) and its commitment renewed (`free_ships`, `commit_ships`). A run loaded in a plan that is preparing waits for leave to launch.
4. **A new run.** When no run of the plan is loading, a run is needed if, without it, the reading of the day after a run ordered today would arrive is below the floor's stock: the floor in days times the planned use of that day. Then:
   - **Its ships** are the pool's free ships in number order, taken until their room covers the load. A ship is free when no unfinished run of any plan holds it, no commitment covers today, and it is stopped at the source, in port in no force or in a force there with no route. A ship in a force brings its whole force. A ship's room is its class's cargo less the supply aboard.
   - **Its load** is the floor's stock on the day it arrives, less the reading of that day, and not below 0, plus the planned use of the interval's days from the day it arrives. The interval is the intent's, or 15 days where the commander set none. It is cut to the ships' room, and to the source's stock above its reserve and above what runs still loading there have yet to take. It is fixed when it is ordered.
   - **Its days**, in whole days: loading takes the hours of the load at the source's berth rate (the lighter rate for a ship with no berth), rounded up to days and at least 1; the passage each way takes the hours of the charged sea distance at the class's cruise speed, rounded up to days; unloading as loading, at the destination's rate. It loads today, sails after its loading days, arrives on its sail day plus the outward passage less one, and its ships are free on the day after they are back at the source. The hour a force steams on from the destination does not move these days. [rulings 6, 8 and 9]
   - **Its route.** The engine's sea path out and back. Where an area limit is set and the path leaves it, waypoints are added along the shortest sea path inside the area until the engine's path between them stays inside; if there is no such path, there is no route. With no area limit the sea is open.
   - **Its fuel.** Each ship needs, in the fuel it holds after the morning's refuelling, the fuel of the round trip at cruise.
   - **On the day,** the staff hands in `add_run` and `commit_ships` (from today to the day before the ships are free), and orders: a forming order at the source where its ships lie in port in no force (the new force's number is the view's `next_task_force_id`, in the order intake forms forces), a join where they lie in more than one force, and a load to a weight at the source, the weight being what is aboard and the load. [ruling 7]
5. **When it cannot.** It says which binds: no ship free until a day, the source's stock, no route in the area, a ship without the fuel, or the use more than the pool can carry. It gives the shortfall in days at the reading it checked, and the day the source falls to its reserve at the present rate (its stock above its reserve over the planned use of the destination and its own). With less stock than the load it sends what it can and says how many days it brings: it slips a run before it accepts fewer days, and accepts fewer days before it stands a plan down. [0107 items 25 and 44]
6. **A day it gives the intent no order** has one reason: what each run is doing, and the reading the next run waits for, with the day it is expected to be ordered.

Last, a plan that has ended whose ships are committed from today on has them freed (`free_ships`).

Every order, staff command and decision to give no order has a reason of five parts (above, Reasons). Its "worked" part is the arithmetic: the stocks, uses, days, loads and hours.

A side sees: its own staff's decisions, reasons and commands whole; the enemy sees none (above).

### A proposal

What the staff would do for one intent, worked out and stored nowhere: what it would hand in on the coming day as if the plan were released, the runs the same rules give over the next 30 days on the projection, the projection itself, what binds if it cannot meet the intent, and a reason. In step 1 it displaces nothing: one intent. [0107 items 5 and 46]

## The tools

A seat whose side has a staff commands through these tools beside the others, in `sim-agent` and through the MCP server and `play`. Every seat of a game those open has one; the staff does nothing for a side until an intent with a forces limit is set. A session made with no staff has none of them. [0107 item 46]

- `command_state`, `plans`: what stands, as the coming day will find it, with each intent's figure and first day below, and yesterday's staff decisions with their numbers for `why`.
- `set_intent` (the side's next free number when none is given), `withdraw_intent`, `set_limit`, `set_leave`, `approve`, `cancel_plan`: each queues one command for the coming day, taken in at its first step. It is checked at once, by taking it in on a copy of the world after the commands already queued, and answered queued or refused with the engine's reason.
- `propose`: the staff's proposal for an intent.
- `staff_orders`: what the staff would hand in at the start of the coming day, each with its reason, asked on a copy of the world with the day's first three steps taken (`World::at_staff_step`). Only the session holds that copy; the staff and the tools are given its side's view.
- `why`: one `StaffReason` by its event number, whole, with the order or command it is for, or as a decision to give no order.

A tool is not a rule: what it shows is worked from the side's view, and the agent's schemas say each answer's shape (`crates/sim-agent/schemas/`).

## What is saved

A game is saved as it has always been, as what was handed in, and from version 3 of the saved game it holds what step 1 of stage 1 adds (chapter 1, What a saved game holds): the commander's commands by day, and each side's staff hand-in by day, whole. A hand-in is the orders with their intent and reason, the staff's commands and the decisions to give no order with their reasons. Only the days and sides that handed in something are kept. The saved game also records the doctrine version beside the rules label. It holds no command state: the state is rebuilt by taking the commands in again. [0107 items 9, 11 and 12]

**Replay.** A replay hands the saved orders, the saved commands and, before each day, the saved hand-in of each side into the engine (`World::submit_staff`), and asks no staff. So a replay of a staffed game works when the doctrine has since changed, and gives every daily hash that was saved. [0107 item 9]

**`replay --check-staff`.** It asks the doctrine's staff as well. On each saved day, each side that the doctrine gives a staff is asked on the view the engine then holds, and what it gives is compared part by part, orders then commands then decisions to give no order, with what was saved. The saved hand-in is what is handed in, whatever the staff gave, so the replay is the saved game and its hashes are still compared. The run stops at the first day and side where they differ and says what differs. A saved hand-in for a side the doctrine gives no staff is a difference too. A difference in the staff is reported before a hash that differs on the same day. If there is none, it says so, naming the doctrine now and the one the game was saved under. It is the test that a staff is a function of the view. [0107 item 10]

**The doctrine version.** The doctrine has a version of its own, `DOCTRINE_VERSION` in `sim-staff` (`"supply-1"`; it was `"none"` before a doctrine existed). A change to what a staff decides from a view changes it. A saved game records it. It is checked when a game kept by `play --game` is resumed for more play, which is refused under another doctrine, and not when a game is replayed: a replay is what was saved. A `play --game` file begun before the key existed is read as `"none"`. [0107 item 11]

Where the staff comes from. Every loop that resolves days gets its staffs from the one place, `sim_staff::staffs()`, and passes them in: `run`, `batch`, `export`, `hash`, `events`, `time`, `replay --check-staff` and a game held open for play. `sim-agent`'s session is handed them by its caller. [0107 item 7; docs/engineering/layout.md]

A side sees: none of this. A saved game belongs to the umpire (chapter 1, What a saved game holds).

## A scenario's commands

A scenario may state a side's commands by day, as it states orders, so that a test can begin with an intent in force. A `[[command]]` table has `day`, `side`, `kind` and the keys of its kind, and no other key. Only the commander's kinds may be written; the staff's are the staff's to give. A scenario with no `[[command]]` plays as it did. [0107 item 16]

| `kind` | Keys |
| --- | --- |
| `set_intent` | `intent`, `base`, `floor_days`, `interval_days` (optional) |
| `withdraw_intent` | `intent` |
| `set_limit` | `limit`, and the keys of that limit, below |
| `set_leave` | `leave` (`"prepare"` or `"launch"`) and `given` |
| `approve`, `cancel_plan` | `plan` |

The limits: `"forces"` takes `intent`, `ships`, `task_forces` and `source`; `"area"` takes `intent` and `hexes`, as `[[q, r], ...]`; `"priority"` takes `intent` and `rank`; `"protected"` takes `intent` and `ships`; `"reserve"` takes `base`, `supply_t` and `fuel_t`; `"risk"` takes `intent` and `risk`, `"preserve"`, `"balanced"` or `"press"`. A quantity is a whole number or a decimal string, never a float.

The loader refuses, when the scenario is read: a day that is 0 or after the scenario's last day; a key the kind does not take, or that is no key of any command; a missing key; a TOML float; a kind that is the staff's; an unknown limit, leave or risk. Whether a base, ship, task force, intent or plan exists is not the loader's to say: the engine checks it when the command is taken in, and refuses it as above. A scenario's commands are kept by day in the order written, kept when a seed is changed, and handed in before the day is resolved, as the commander's, so the engine takes them in at its first step. Two commands of one day for one side are taken in in the order written, and the Allied side's before the Japanese side's. [0107 item 16]

A scenario's `[[order]]` may give the issuer `staff` and an `intent`. Such an order is not checked against a forces limit (above, The forces limit on a staff's orders). [0107, As built]

## The day's order

Chapter 1 owns the sequence. Where this chapter's rules run, in the numbering of decision 0107's table (chapter 1 numbers the returns from the yard and the refuelling apart, and so counts the staff's steps as 5 and 6):

| Step | What happens | Where |
| --- | --- | --- |
| 1 | The commander's commands are taken in | This chapter, Commands |
| 2 | The override of a held unit by the commander's order: not built | This chapter, What is built and what is not |
| 3 | Forces back from the yard are home, then ships in port refuel, each base down to its reserve | Chapter 2, Refuelling in port |
| 4 | Each side's staff, if it has one, is asked on its view as it now stands | This chapter, The staff |
| 5 | The staff's commands are taken in; then all orders, the commander's and the staff's | This chapter, The staff; chapter 1, Orders |
| 6 | The rest of the day, as it was | Chapters 2 to 7 |

[0107, The order of a day]

## Not built

- **Holds** on a setting, a unit and a parent force, and the override: an order from the commander to a unit inside an intent's forces limit holds that unit. Step 2 of the day is its place. [0107 items 20 and 48]
- **Interrupts**, their setting and acknowledgement. [0107 item 28]
- **A run of days** that resolves up to a number of days and stops between days for an interrupt. One day of it is `end_day`. [0107 item 29]
- **The command report**, with one identity for a matter that lasts. [0107 item 30]
- **Commitments of stock**, and a second source base for an intent. [0107 items 19 and 27]
- **The fallback**: a cancelled plan's ships turned for home. `cancel_plan` marks the plan cancelled; the staff gives it no more orders and frees its ships' commitments from that day; a force at sea keeps its route, which already ends at the source. [0107 item 24]
- **Escort, cover, the recall of a run, a second source, an intent for fuel, a second intent competing for ships and stock, previews, holds, interrupts, `run` and the command report**: stage 1's steps 2 to 4 and stage 2. [0107]

## Not in this chapter

- How a command's refusal and an order's refusal differ for a player: the agent's tools, not rules.
- How orders are numbered, sorted and refused, and who can issue them: chapter 1, Orders.
- Refuelling, loading to a weight, reserves and forming, joining and standing down forces: chapter 2.
- Belief, contacts and claims, which a staff reads through the view: chapter 3.
- The men's supply, landings and the land units a base feeds: chapter 7. Aircraft upkeep, repair and sorties: chapter 5.
- The saved game's other contents and the state hash: chapter 1, The state hash and saved games.
