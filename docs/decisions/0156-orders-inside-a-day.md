# 0156: Orders inside a day: stopping the clock between pulses

Status: a plan by the architect, written on Nov 18, 2026, for the designer's approval before anything of it is built. It plans step 2 of decision 0155: a commander who may stop the clock between the pulses of a day and act there. Nothing is built. It differs from decision 0155's wording in one thing, stated first below, which is the designer's to accept or refuse.

Summary: A plan for commanding inside a day: where a scenario says so, a seat may stop the clock between pulses and give seven kinds of order to its own units, taken in at the start of the next pulse and holding the unit from the staff as an order at dusk does; commands to the staff are still taken in at dusk; time is advanced a pulse at a time by one new tool that stops on what the side was told, and a second gives a seat its events together; the engine resolves a day in parts with the same result; no state is added and a game with no order inside a day is the game it was. With the hand calculation, the conditions of its proof, five steps and five questions for the designer

Standing: Built in five steps, Nov 18, 2026. Its proof was run once and judged the same day: five of seven judged conditions hold and two miss (legibility of what an order changed, and speed at the tool on a near-empty day), which wait on the designer; off for the 39 days

## What differs from decision 0155, and why

Decision 0155 named the short list as commands to the staff: to hold a thing back, to withhold or give a leave, to change the risk, to withdraw. The code says those cannot act inside a day:

- The staff is asked once, before the night (`resolve.rs`, step 4). A leave, a risk, a hold or a withdrawal given at 0600 is read by nobody until the next dusk.
- A hold stops the staff from ordering a unit. It does not stop the unit: "A hold does not stop the game's rules" (`docs/step3-spec.md`). A group on a strike mission flies whether it is held or not, because the launch reads the group's mission and its command's picture and never the holds.
- What acts inside a day is a unit's standing orders: its route, speed, readiness and rules of engagement, its air mission and search arc, its posture ashore.

So to act inside a day a commander must speak to the unit, or the staff must be asked again. Decision 0155 left out asking the staff again every pulse, and this plan keeps to that. What is planned is therefore **orders to units**, of a short list of kinds, with the rule the game already has for a commander who orders a unit the staff is using: the order is taken in and the unit is held from the staff (decision 0107, ruling 23, the override).

This is not ordering ship by ship as the way to play. The staff still plans and orders everything from intent. This is the commander reaching in: "turn the transports back", "stand that group down", "go to battle readiness". It is what the record's commanders did inside a day, and the interface gives it as a few verbs on a force.

## The rules

1. **Where it is on.** A scenario says so with a key, off by default. With it off nothing of this plan can happen and every game, tool list and file is as it was. The small battles a new player begins with have it on; the campaign need not.

2. **The stops.** A day has three pulses. With the key on, a seat may stop the clock at two more moments than today: at 0600, between the night and the morning, and at 1200, between the morning and the afternoon. Dusk, 1800, is the stop it has now.

3. **What may be ordered at 0600 and 1200.** Seven kinds of order, each a standing setting that the pulses read and none that makes or unmakes a force:

   | Unit | Order | What it changes inside the day |
   | --- | --- | --- |
   | Task force | `set_route` | Where it steams from the next pulse |
   | Task force | `set_speed` | How fast |
   | Task force | `set_readiness` | Its readiness for a fight |
   | Task force | `set_rules_of_engagement` | When it breaks off |
   | Air group | `set_air_mission` | What it flies; `rest` stands it down |
   | Air group | `set_search_arc` | Where it searches |
   | Land unit | `set_posture` | Whether it attacks in the day's fight ashore |

   Every other kind is refused there with one reason, "given at dusk": forming, joining, detaching and standing down a force, loading and unloading, a yard, a home port, a rebase, an escort, the preparation of a landing, a march, an activity, and the finer air settings. They may come later, each for a reason; the list starts short.

4. **When it takes effect.** An order given at 0600 or 1200 is taken in at the start of the pulse that follows, before anything else of that pulse, and is checked then against the state as it stands, as every order is checked when it is taken in. Its events carry that pulse. This is the rule dusk already has: an order given at 1800 acts in the night that begins. No new delay is added (question 2).

5. **The unit is held.** If the unit lies in the forces limit of an intent in force, the order holds it from the staff, with a `HeldByOrder` event, exactly as the override does at dusk. The staff's orders to it are refused, with its reason saying so, until the commander releases it. A unit no intent uses is not held, as today.

6. **Commands to the staff.** At 0600 and 1200 a seat may still state an intent, set a limit or a leave, approve, hold, release, withdraw and set an interrupt. Each is queued and is taken in at the next dusk, and its answer says so. The staff is not asked again inside the day. So a new intent given at 0600 becomes the staff's orders at 1800: this is the time the staff takes, and it is the game's own clock, not a new figure.

7. **What a seat may read there.** What it may read at dusk, on the state as it stands: its forces, bases and contacts, an entity, a measure, an explanation, its command state, plans and report. The briefing stays the document of dusk and is not made at 0600 or 1200; the seat reads its events of the day so far instead (rule 9).

8. **Advancing time.** One new tool, offered only where the key is on and one seat is a player's, as `run` is: `advance`. It resolves up to a number of pulses and stops at the end of the first pulse in which the side was told any of the things the call names, or at a dusk where an interrupt fired as `run` stops, or when the game is over. What it can stop on is read from the side's own events of the pulse, as `run` reads the firings of a day, so it adds no state:

   | Stop on | The side was told, in that pulse |
   | --- | --- |
   | `contact` | A contact it did not hold |
   | `attack` | One of its forces or bases was attacked, from the air or the sea |
   | `loss` | One of its ships was sunk or put out of action |
   | `strike` | One of its own strikes or raids was launched or came back |
   | `landing` | Men of its own landed, or a fight ashore |

   Crossing a dusk, `advance` hands in the orders and commands queued for the day, as `end_day` does. `run` and `end_day` stay as they are and resolve whole days.

9. **A seat's events together.** A second new tool, offered in every game: `events`, the side's own events of a day, or of one pulse of it, each with its number, pulse, kind, words and the units it names. It gives nothing `explain` does not already give one at a time.

10. **The other side.** A scripted side and a staff give their orders at dusk and nothing inside a day. Their units still decide inside the pulses what they decide today. A scenario's script may give an order a pulse, for a test and for a replay. The proof reports what a seat gains from commanding inside a day (condition 7); the staff is not given the same until that is seen.

## What is built in the engine

- **A day in parts.** Today one function resolves a whole day. It becomes three that it calls in turn: open the day (the commands, the staff, the interrupts and the orders of dusk), resolve the next pulse, and close the day (the end-of-day phases, the invariants, the hash). The sequence, the events and the hash are the same. This step is a move and changes no rule.
- **Orders handed in while a day is open** wait, unhashed as pending orders are, and are taken in at the start of the next pulse in the order intake sorts them. The engine already takes one kind of order inside a day, the local commander's order to a straggler, with its pulse on the events; this is the same path.
- **No state is added.** The hash is taken at the end of a day, when no day is open and nothing waits. A saved game gains the pulse of an order given inside a day, left out for an order of dusk, and its version rises; the replay hands such an order in at its pulse.

## Rule card

- **Reads:** the side's orders handed in while a day is open; the state as it stands at the start of a pulse; the side's intents in force and their forces limits (for the hold); the side's own events of a pulse (for `advance`).
- **Writes:** what the seven orders write today (a force's route, speed, readiness and rules of engagement; a group's mission and arc; a unit's posture); the side's holds, by the override; the events `OrderAccepted`, `OrderRejected` and `HeldByOrder`, with a pulse where they had none. Nothing else.
- **Reuses:** the order check and intake; the override and its hold; the local commander's intake inside a day; `run`'s way of stopping on a side's events; the saved game and its replay.
- **Draws:** no random number.

## What it costs

- **Versions.** The state layout stays `flagplot-state-v44` and the rules label stays 19, since no existing game changes; if the build finds either must rise, it comes back to the architect first. The saved game rises to 20 and the agent interface to 25. The doctrine version stays.
- **The rulebook.** Chapter 1 ("A side gives its orders ... before a day and cannot interrupt or give orders inside it") is rewritten for a scenario with the key on, with the seven kinds, when each is taken in and the order of a pulse; chapter 8 says the override also acts inside a day. The glossary gains "an order inside a day".
- **Named and not built:** the time an order takes to reach a force when command levels are on. Today an order of dusk reaches every force at once, and this plan keeps that for an order of 0600 or 1200. A delay downward is a rule of its own, with figures from rows.

## The hand calculation

For `tests/scenario_78_orders_inside_a_day.rs`, whose doc comment takes this section word for word. Written by the architect on Nov 18, 2026, before any code. If the code disagrees, the code or the rule is wrong: no number here is changed to make a test pass.

**The setup** is scenario 1's, with the key on: one Allied task force, a cruiser (2,000 t of fuel, 0.25 t a mile at cruise) and a destroyer (500 t, 0.125 t a mile), ordered at dusk of day 1 from (0,0) to (8,3) at cruise, 16 kn. Scenario 1's hand calculation gives the path and the miles steamed when the force enters each hex: (1,0) at 19, (2,0) at 57.5, (3,0) at 95.5, (4,0) at 132.5, (5,0) at 168.5, (6,0) at 203, (7,0) at 235, (8,0) at 262, (8,1) at 294, (8,2) at 334, (8,3) at 374; the transit is 394 miles. A ship below its cruise speed burns its cruise rate a mile (chapter 2, Fuel burn), so fuel is miles times the rate at either speed.

**Case A: at 0600 of day 1 the commander orders `set_speed` of 8 kn.**

| Pulse | Hours | Speed | Miles | Total | Hexes entered | Position at its end |
| --- | --- | --- | --- | --- | --- | --- |
| Night | 12 | 16 kn | 192 | 192 | (1,0) to (5,0) | (5,0) |
| Morning | 6 | 8 kn | 48 | 240 | (6,0) at 203, (7,0) at 235 | (7,0) |
| Afternoon | 6 | 8 kn | 48 | 288 | (8,0) at 262 | (8,0) |

- The order is accepted in the morning pulse of day 1; no event of the night names it.
- End of day 1: 288 miles steamed, the force in (8,0). Fuel burned: cruiser 288 x 0.25 = 72 t, 1,928 t left; destroyer 288 x 0.125 = 36 t, 464 t left.
- Day 2: 394 - 288 = 106 miles remain, 13.25 hours at 8 kn. The night steams 96, to 384, entering (8,1), (8,2) and (8,3). The last 10 miles take 1.25 hours of the morning: the force arrives 37.25 hours after the start, at 07:15 on day 2. Fuel on day 2: cruiser 26.5 t, destroyer 13.25 t.
- Totals: cruiser 98.5 t, destroyer 49.25 t, as scenario 1's, since the miles are the same.

**Case B: the same order at 1200 of day 1.**

| Pulse | Hours | Speed | Miles | Total | Hexes entered | Position at its end |
| --- | --- | --- | --- | --- | --- | --- |
| Night | 12 | 16 kn | 192 | 192 | (1,0) to (5,0) | (5,0) |
| Morning | 6 | 16 kn | 96 | 288 | (6,0), (7,0), (8,0) | (8,0) |
| Afternoon | 6 | 8 kn | 48 | 336 | (8,1) at 294, (8,2) at 334 | (8,2) |

- The order is accepted in the afternoon pulse of day 1.
- End of day 1: 336 miles steamed, the force in (8,2). Fuel burned: cruiser 84 t, 1,916 t left; destroyer 42 t, 458 t left.
- Day 2: 58 miles remain, 7.25 hours at 8 kn. The force enters (8,3) and arrives 31.25 hours after the start, at 01:15 on day 2, in the night pulse. Fuel on day 2: cruiser 14.5 t, destroyer 7.25 t. Totals as above.

**Case C: the key on and no order inside a day.** Every daily hash is scenario 1's.

**Beside the numbers,** tests that state what must always hold:

- An order of a kind not among the seven, handed in at 0600, is refused as given at dusk and changes nothing but its event: the truth of each day is the truth without it, the arrival event's number set aside, and the daily hash is that of the same game with one order refused at dusk, since the hash counts orders and events (decision 0055).
- The later the order, the farther the force: the same order at dusk of day 1 (8 kn for 24 hours), at 0600 and at 1200 gives 192, 288 and 336 miles steamed on day 1.
- With a staff: in scenario 36, the one convoy, a `set_speed` at 0600 to the convoy's force, which is in the forces limit of the supply intent, records `HeldByOrder` in the morning pulse. At the next dusk the staff gives the held force no order: it sets it aside as chapter 8 has it, ends the run with a reason that names the hold, frees the ship and forms another run with another ship.
- A saved game of case A replays to the same daily hashes. The same file with the order's pulse changed to the afternoon replays to case B's.

## The conditions of its proof

Written before anything is built. None is a tuning mark; each can fail.

1. **Nothing changed where it is off.** Every run baseline, golden file, recorded game and pinned hash is as it was. A batch of 1,000 seeds of the Eastern Solomons made before and after is byte for byte the same.
2. **Nothing changed where it is on and unused.** A game advanced a pulse at a time with no order inside a day has the daily hashes of the same game run by days, on 1,000 seeds of the Eastern Solomons with the key set on a copy.
3. **The hand calculation** of scenario 78 passes, cases A, B and C, with the tests beside it.
4. **A saved game replays,** and refuses a scenario whose key was changed.
5. **A seat is told nothing more.** At 0600 and 1200 the tools answer from the side's view at that moment; the test that reads the agent's source still passes, and a seat's count of its events at 1200 is the count of what its view holds then.
6. **Legible.** A reader given one side's events of a day with an order inside it says, for each of six such orders, when it was given, in which pulse it acted and what it changed: six of six.
7. **Reported, not judged: what it is worth.** The Eastern Solomons, one day, played by a model in the Allied seat from intents alone, 30 games by days and 30 advancing by pulse: whether it strikes ships, and the margin. And the same with the staff alone, for comparison.
8. **Speed.** A day resolved in three calls costs no more than 1.1 times the day resolved in one.

## The steps

Each is one branch and one pull request, read by the architect before it merges.

1. **A day in parts.** The engine's day as open, pulse, close. No rule; every hash as it was.
2. **Orders inside a day.** The seven kinds, the intake at a pulse, the hold, the scenario's key and the pulse of a scripted order, the saved game, scenario 78 and its tests, the rulebook and the glossary.
3. **The tools.** `events` and `advance`, what each tool answers at 0600 and 1200, the schemas and the server.
4. **The table.** It stops between pulses, and a force takes the verbs.
5. **The proof,** run once, as written above.

## Questions for the designer

1. **Orders to units, not commands to the staff.** The finding at the top. Recommended: accept it. The other way is to ask the staff again at 0600 and 1200, which decision 0155 left out.
2. **When an order of 0600 or 1200 acts.** Recommended: in the pulse that follows, as an order of dusk does. The other choice is a pulse later, which makes the commander's reach slower than his staff's own units and needs a reason the record would have to supply.
3. **On only where a scenario says.** Recommended: yes, off by default, on for the small battles and off for the 39 days until its proof is read.
4. **The five things `advance` can stop on.** Recommended as listed. A player chooses among them; the table's default would be contact, attack and loss.
5. **When it is built.** Recommended: now, beside the planning of stage 4's two mends, since it touches the day loop and the tools and they touch the doctrine. Steps 1 to 3 are the engine's and the tools'; the proof follows.

## Confirmed on Nov 18, 2026

The designer approved the plan on Nov 18, 2026 and said to build it. His five answers, each as the architect recommended:

1. Orders to units, not commands to the staff: accepted.
2. An order of 0600 or 1200 acts in the pulse that follows.
3. On only where a scenario says, off by default; on for the small battles, off for the 39 days until the proof is read.
4. `advance` can stop on a contact, an attack, a loss, a strike and a landing, as listed.
5. It is built now, beside the planning of stage 4's two mends.

## As built: step 2, and two rulings

Steps 1 and 2 are built (pull requests 426 and 427): the day in parts, and orders inside a day as the rules above have them. Every number of the hand calculation is asserted and passes. Two lines of "Beside the numbers" were wrong as the architect first wrote them, and are corrected above; neither is a number of the hand calculation, and no rule changed for either.

1. **A refused order and the hash.** The architect wrote that an order of a kind not allowed, refused at 0600, leaves "the day it would have been". The daily hash counts orders and events (decision 0055), and the truth keeps an event's number for an arrival, so the hash differs by that count. What is true and is asserted: the refusal is the only event that names the order; the events with it taken out are the events without it; the truth each day is the truth without it, the arrival event's number set aside; and the hash is that of the same game with one order refused at dusk.
2. **What the staff does with a held convoy.** The architect wrote that the staff's order to the held force would be refused as held. The staff reads the hold and gives the force no order. It sets the force aside (chapter 8), ends the run with a reason naming the hold ("held by the commander's order of day 1; the 360 t aboard stay aboard"), frees the ship, and forms a second run with the other ship. So an order inside a day to a force on a staff's run takes that force out of the staff's hands until it is released, and the staff plans round it. This is the game's rule for a hold and is what a commander who reaches in should expect; the interface should say so before he signs.


## As built: step 3, the tools

Step 3 is built (branch `advance-and-events`): `events` and `advance`, orders at 0600 and 1200 through `issue_orders`, and every other tool there. The agent interface is version 25. No rule changed; the state layout, the rules label, the saved game's version and the doctrine stay. Two things were added to the engine on the architect's two rulings of Nov 18, 2026, both read-only and unhashed:

1. **`World::as_if_day_closed`**, a copy with the open day treated as closed: no day or pulse in hand, the day counter at the open day, and the pulse's scratch that is empty at every day boundary emptied (the orders waiting inside the day, the forces that broke silence, the commands that ceased to be, the strikes in flight, the sorties); the grounded bases, the damage of the fields and the decks lost are left as a closed day leaves them, as the last pulse left them. No end-of-day phase is run. The session uses it, and only in `session.rs`, for the check of a command and for `command_state`, `plans`, `staff_orders`, `propose` and `preview` at 0600 and 1200; each of those answers says so in a note `inside_day`.
2. **`SideView::day_in_hand` and `SideView::pulse_last_resolved`**, the clock, for the interface's words only (no rule of the engine and nothing of the staff reads them; `pulse_just_ended` is unchanged). A land unit's `fed_last_night` reads the night of the open day once it is resolved, and the area seat's `as_of` compares with the pulse last resolved; both are what they were with no day open.

What the build settled that the plan and the brief did not say:

- The session resolves every day in its parts, from dusk or from the open day (`Session::resolve`); end_day and run inside a day first resolve the rest of it.
- `events` names the kind of each event `event_kind`, as `explain`'s node does, and adds `begun` and `count`. With no day resolved or open its day is 1, not begun.
- `advance` reads what the side was told as its events numbered above the count it held before the pulse, so for the afternoon the events of the day's end are counted with it (a fight ashore, which comes at the end of the day, stops the call at that dusk). It stops for, in this order: the game over, an interrupt fired on the day just closed, what it was told, the pulses asked for. `told` is given for the last pulse; `fired` is of the last day closed in the call, empty when none closed. `clock.day` is the day whose pulse is next.
- `stop_on` "loss" is a ship sunk, a ship lost with its base (added by the architect's ruling on reading the step) or a `system_hit` taking propulsion or the flight deck to out from not out; a ship lost with its base (`ship_lost_with_base`) is not counted. "landing" counts every `ground_fight` the side is told of, which a side is told of only where it has men in the fight.
- A preview is void once a pulse is resolved after it ("a pulse was resolved after it").
- At 0600 and 1200 the copy a command is checked on takes the seat's orders queued inside the day as orders of its dusk, so that a hold they make shows in what the staff would do.
- `report`'s day is the open day at 0600 and 1200.
- `advance` is listed last, after the command tools; `play --tools`, which has no game, lists the 28 tools without it.

Left as it is, and named: `measure` works a passage or a yard trip from the start of the coming day's night, which at 0600 or 1200 has passed, and says so in the note; a forecast from a given pulse is the engine's to give (`docs/open-work.md`). At 0600 and 1200 `command_state`'s `forces_held` marks as current only a state held as of the afternoon of the open day, since it is worked on the copy. A briefing line's "moved yesterday" and a base's works of the day before read the last day closed.

## As built: step 4, the table

The table (`table/`, decision 0154) stops between pulses where the scenario offers `advance`: it resolves a pulse at a time and plots each as it is fought from `events`, the player chooses which of the five things stop the clock, and at 0600 and 1200 a force takes an order from the chart (its speed, its readiness or a place to steam for, and its air groups stood down), with the warning that the staff plans round an ordered force until it is released. Tried by hand on a copy of the 39 days with the key on: an intent signed at dusk, the clock stopped at 0600 of day 1, a transport force slowed to 8 knots, and the morning plotted to a stop at 1200. `table/serve.py` gains the list of the tools a seat is offered. In a scenario without the key the table works as it did, reading a day's events from `events` in place of `explain`.

One more gap in the seat's tools was found: none tells a seat the hour. A page opened at 0600 reads it from the words of a refused briefing. Open, in `docs/open-work.md`.

## As run: step 5, the proof

Run once on Nov 18, 2026 and judged by the architect; `docs/validation/orders-inside-a-day-proof.md` has it in full, and nothing was changed after the run.

- Conditions 1 to 5 hold: 1,000 seeds of the Eastern Solomons byte for byte the same before and after; by pulse and unused, 1,000 of 1,000 seeds agree from each seat; the hand calculation, 9 of 9 tests; a saved game replays and a changed key is refused; a seat's events at 0600 and 1200 are its view's.
- Condition 6 misses: 5 of 6 by the key, 4 of 6 by the condition's words. An `order_accepted` event does not say what was ordered, so a change of readiness or of an air group's mission cannot be read from the day's events.
- Condition 8 misses by the stricter reading: in the engine 0.946 and 0.996; at the tool 1.012 on the battle and 1.541 on ten near-empty days of the slice, about 46 microseconds a day.
- Condition 7, not judged: a model in the Allied seat struck ships in 19 of 30 games by days (median margin 3.3) and 13 of 30 by pulse (0.675). The 12 games in which it gave orders inside the day struck in 2: an order takes a unit from the staff.

The misses are the designer's to accept or have mended. The mend the architect recommends for the first: the event of an order taken in says what was ordered, to the side that gave it. The key stays off for the 39 days.
