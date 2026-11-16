# 0136: What a superior knows of his forces, and radio silence: step 3 of stage 2b

Status: settled by the architect on Nov 16, 2026, inside the plan for step 3, stage 2b, that the designer confirmed the same day (decision 0132, step 3 of its five, with his answer that the staff or a scenario may set silence). It follows decisions 0134 and 0135. Built on Nov 16, 2026 ("As built" below), with points left to the architect.

Summary: Step 3 of command levels: at the end of each pulse a task force and a base report their own state to the area command, which holds each as last reported, with the pulse it is of; an area doctrine and the area commander's seat read that and not the truth; an order that cannot be carried out because the force is not as it was believed is refused with that cause; and a force may keep radio silence, set by a scenario, by the commander or by the staff at preserve the force, sending nothing until it fights or strikes

Standing: Settled by the architect, Nov 16, 2026, inside decision 0132. Built, points open ("As built", "Open")

## State reports

1. **At the end of each pulse a task force and a base report their own state to their superior:** for a task force, where it is, where it is going, and each of its ships and the air groups aboard as the side's view gives them; for a base, its stocks, its airfield, its air groups and the land units at it. The report is a copy of that state as of the end of that pulse.
2. **It arrives at the end of that pulse, at the end of the next, or never,** by one draw with the "up" chances of decision 0135 and a `rng::Draw` value of its own. A state report is sent up only: not sideways and not down.
3. **The area command holds, for each command under it, the latest state report that has arrived,** and the day and pulse it is of. An older report that arrives after a newer one changes nothing.
4. **At the start of the game** the area command holds every command under it exactly, as of the start.
5. **A command formed in the game** is held by the area command from its first state report, and until then as the staff or the commander formed it.
6. **A state report that arrives on time is no event.** One that is late or lost is: "state report late" or "state report lost", with its draw. A side sees its own.
7. **With no report figures, or with command levels off, a superior knows his forces exactly,** as now.

## Who reads what

8. **A command knows its own units exactly.** A task force's doctrine and a base's read their own ships, groups and men as they are.
9. **The area command knows a force a level down as last reported.** Every doctrine addressed to the area, and the area commander's seat, read a task force, its ships and groups, a base, its stocks, groups and men from the state the area command holds, through one view for a command, as decision 0134 has it for pictures.
10. **What the engine measures for the area staff starts from what it holds:** a passage, its hours and its fuel are measured from the place and the fuel last reported.
11. **A reason of an area doctrine that rests on a force's state says the pulse that state is of,** where it is not the pulse just ended.
12. **A task force or a base that reads a unit of another command** reads it as the side's view gives it, in this step. "As built" lists every such place. What a command knows of its neighbours' forces is left for later.

## An order that meets a different world

13. **The engine takes an order as it always has, against the truth.** An order from the area staff or the area commander that it refuses for a cause the state held did not show (the force is elsewhere, a ship is sunk or out of fuel, a group has no aircraft) is refused as now, and the refusal says what the area command held and the pulse it was of.
14. **Such a refusal is not a fault of the staff.** It is counted apart. Decision 0132's condition that no staff order is refused is read so: none but for this cause, and those are counted and reported.

## Radio silence

15. **A setting of a task force,** off unless set. A scenario may set it at the start; the commander may order it on or off; the staff sets it as 18 has it.
16. **A silent force sends nothing:** no state report, and no report of its own up or sideways. It hears everything sent to it.
17. **It breaks silence in a pulse in which it fights, is attacked or launches a strike.** In that pulse it sends that pulse's state report and every report of its own it has not sent that is not yet older than a contact may be, oldest first. In the next pulse it is silent again.
18. **The staff's doctrine.** A carrier force and a surface force that stands by night are set to radio silence at preserve the force, by the doctrine that owns the force, where report figures are loaded. At balanced and at press the staff leaves the setting as it finds it.
19. **What silence buys is not built.** Nothing listens to radio traffic yet (`docs/design/intelligence.md`). Until it does, silence costs a superior his knowledge and buys nothing, and the doctrine of 18 is a placeholder for the choice the intelligence layer will make real. The designer chose on Nov 16, 2026 that the staff may set it all the same.
20. **A limit.** The staff does not turn silence off when a force's risk rises from preserve the force. A commander does.

## Rule card

- **Reads:** each command's own units; the report figures; the silence setting; whether a force fought, was attacked or launched in the pulse.
- **Writes:** the state the area command holds of each command under it, with its pulse; the state reports in flight; a force's silence setting; the reports a silent force has not sent.
- **Reuses:** decision 0135's chances, draws and late delivery; decision 0134's one view for a command; the engine's intake and its refusals.

## What it costs

- **State:** what the area command holds of each command, the state reports in flight, the silence setting and a silent force's unsent reports, in the hash and named. The state layout and saved game versions rise. The agent interface version rises: the area seat's lists and briefing give a force as last reported, with the pulse.
- **An order** `set_radio_silence` and its intake. Two kinds of event.
- **Every area doctrine** reads through the command's view. The supply doctrine of stage 1 is the largest.
- **Speed,** measured with the step.

## The hand calculation

`tests/scenario_63_a_superiors_knowledge.rs`.

## As built

Built on Nov 16, 2026. The held state, the state reports and the check of an order against what is held are `crates/sim-core/src/held.rs` (`AsOf`, `HeldState`, `state_of`, `held_truth`, `make_state_reports`, `hold_reported`, `validate_order`); one view for a command is `crates/sim-core/src/view/known_view.rs` (`SideView::known_to`, `KnownView`, `held_as_of`, `pulse_just_ended`, `radio_silence`); silence is `World::pass_report`, `break_silence` and `send_own_report` in `crates/sim-core/src/passing.rs`; the staff's silence order is `silence_order` in `crates/sim-staff/src/stand.rs`. The draw is `Draw::StateReport` (code 11). The events are `StateReportLate` and `StateReportLost`, seen whole by their own side and not at all by the other. The order is the task force order `SetRadioSilence { on }` (`set_radio_silence` in the agent interface); the scenario key is `radio_silence = true` on a `[[task_force]]`; the refusal is `RejectReason::NotAsHeld`. The state version is `flagplot-state-v37`, the saved game version 12, the doctrine version `supply-18` and the agent interface version 15. The rules label stays 18: with command levels off or no report figures no game plays differently, and scenario 62, the one built scenario with report figures, has its "up" on time 1 and no silence and plays as it did. Scenario 63 is `data/scenarios/63_a_superiors_knowledge.toml` with `data/reference/reports_placeholder_scenario_63.toml`.

### Readings taken

1. **When a state report is made.** At the end of the night and of the morning, after the forces that ceased to be are ended; the afternoon's at the end of the day, after the end-of-day phases (a fight ashore, the count of bases), as of the afternoon. The staff is asked next on the state as the day ended, so with every report on time the area command holds what the staff read before this step. Late reports are applied at the next report moment, before that moment's reports; an older one never replaces a newer one, and one of equal moment replaces.
2. **What a report holds.** A task force: its record, the ships in it or at a yard to rejoin it, and the air groups whose command it is. A base: its record (stocks, airfield), the air groups and the land units whose command it is. Anything of the side that no held state names is read as it is: a ship in port in no force, men aboard ship, a group lost, an arrival not yet reported.
3. **The view.** `known_to(CommandRef::Area)` builds the truth with each held state written over it, oldest first, and a view on it; every `SideView` method that reads the side's own units then reads the held state, the engine's forecasts and questions included, with no second copy of a rule (item 10). For any other command, or with command levels off or no report figures, it is the side's view itself. The pulse's scratch (strikes in flight, the bases grounded, deck losses) and the log are not overlaid: they are read by the engine's questions to a unit, which are the unit's command's.
4. **The start, a force formed, a base taken.** Holding begins when command levels are turned on with report figures loaded. A force or base formed in the game is held at once, as of the pulse in hand, else the afternoon just ended. A base that changes side is dropped at once by the side that lost it, with its reports in flight, and held by the side that took it.
5. **A force that ceases to be** makes one last report at that pulse's report moment (its record with no ship), drawn as any; lost, the area command holds it as it was. A silent force makes it only if it broke silence in that pulse.
6. **The receiver.** Every state report goes to the area command, by the "up" chances; a named superior other than the area command is given none in this step (two levels are built).
7. **Breaking silence** is called where a force is in a surface engagement (when it begins), fires a bombardment, is reached by an air attack, or a group of its command launches a naval strike or a raid. The first call in a pulse sends what the force kept, younger than a contact may be, oldest first, as made; its state report of that pulse is made.
8. **Silence is a setting, not an order that holds a force** from the staff (decision 0107, ruling 23). Turning it off drops what the force kept; nothing is sent. With command levels off or no report figures it does nothing, though the order is taken.
9. **"From the area"** (item 13) is the commander's order, or the staff's under an intent addressed to the area; an order under an intent addressed to a base or a force, or a scripted one, is refused as before. The check on what is held runs only on a refusal, on a copy of the world with its truth replaced by what is held.
10. **The interrupts' reading** (decision 0111) of a base's days of supply is made on the area's view: the commander and the area staff read it.
11. **Reasons (item 11)** add "(as of the morning of day 1)" after a force or base where its held moment is not the pulse just ended; with every report on time they never appear.

### Every reader changed

| Reader | Where | Reads |
| --- | --- | --- |
| The supply doctrine (keeping a base supplied), its fleet, planner, readings, projection and escort | `sim-staff`, `doctrine.rs` | The area's view |
| A proposal for an area intent | `sim-staff`, `proposal.rs` | The area's view |
| The neutralise doctrine's candidates | `sim-staff`, `neutralise.rs` | The area's view |
| The deny doctrine's candidates | `sim-staff`, `deny.rs` | The area's view |
| The forming of a force's stand or carrier task, from deny, hold and the run's cover | `sim-staff`, `stand.rs`, `form_force_tasks`; `air.rs`, `supply_covers` | The area's view |
| The interrupts' reading | `sim-core`, `command/interrupt.rs` | The area's view |
| The briefing, `list_forces`, `list_bases`, `get_entity`, `measure` | `sim-agent`, `session.rs` | The area's view |

Unchanged on purpose: the doctrines of a base (hold, cover, raid, strike, search) and of a force (stand, carrier) read the side's view; the agent's `list_contacts`, `explain`, the check of an issued order and the staff tools (`staffing.rs`).

### Where a force's or a base's doctrine reads a unit of another command (item 12)

All read the side's view, in this step: `air.rs`, `may_use` and `at_base` (groups named by a forces limit, at another base or aboard ship) and the shared tallies of fighters and bombers taken across bases and tasks; `search.rs`, a bomber group borrowed for a search; `strike.rs` and `raid.rs`, groups and bases, and the target base's hex; `stand.rs`, the home port and its size; `carrier.rs`, the home base and its hex; `escort.rs`, the run's forces and ships.

### Open, for the architect

- **A force back from the yard.** Steps 1 to 3 of a day (a force home from the yard; the commander's orders taken by the override) change the truth after the afternoon's report and before the staff is asked. With every report on time the area command then holds such a force as it was at the day's end. No game tested here has one. The options: make the afternoon's report after step 3, or leave it.
- **NotAsHeld counted apart.** `RejectReason::is_not_as_held` tells them apart; no report of the agent interface or of `sim-cli` counts refused staff orders by cause today, so none is counted yet. Where it goes is the command report (`sim-agent`, `staffing.rs`).

### A supply run on stale word (for the architect, not asserted)

Scenario 36 (one convoy, no enemy) with `command_levels = true` and report figures whose "up" is never (made for the run and not kept), release build. The area command holds base 2 at 100 t and every force as of the start, or as formed, for all 18 days. Day by day:

- **Day 1:** as with command levels off: the plan approved and released, run 1 of 360 t added, ship 1 committed to it from day 1 to 4, formed into task force 1000001 at base 1 and ordered to load. It loads 360 t in the night.
- **Day 2:** no order: "Reading today at base:2 (as of the start): 100 t ... Run 1 is loading at base:1 (as of the start): 0 t of 360 t aboard. One run loads at a time, so no new run is considered." With command levels off, run 1 is sent on day 2 with a route and an unload order.
- **Day 3:** ship 1's run 1 no longer counts as loading, and the reading is still 100 t (as of the start), so run 2 of 360 t is added on ship 2, formed into task force 1000002 and ordered to load. It loads 360 t in the night.
- **Days 4 to 18:** the same, turn about: on each even day "Run N is loading at base:1 (as of ...): 0 t of 360 t aboard", no order; on each odd day a new run of 360 t on the other ship, with a load order that does nothing, for the ship has 360 t aboard. Nine runs in all; no ship ever sails, for the area command never hears that a load is complete.
- **The outcome:** base 2 runs out of supply in the night of day 10; its 2,000 men go unfed from day 11 and are 1,330 on day 18 (with command levels off it never runs out and no man is lost). Both ships lie at base 1 with 360 t aboard.
- **Refusals:** none. Every order is taken: a load order to a ship already loaded is accepted against the truth. 210 "state report lost" events, none late.

The pool's free ships carry no "(as of ...)": they lie in port in no force, which no state report names, so the area command reads them as they are (reading 2).

### Measured (for the architect, not judged)

`sim-cli time --slice ... --early-day 15 --late-day 18 --repeats 5`, release, on the week of the slice from intents, and on the same file with `command_levels = true` and the live report figures (`data/reference/reports.toml`; made for the measure and not kept): six staffed days from day 15, 65.1 and 70.2 ms (medians of 5); three from day 18, 26.6 and 29.3 ms; seven from the start, 8.01 and 10.0 ms; one day resolved, 12.5 and 12.9 ms (day 16), 8.22 and 8.87 ms (day 19); the longest staff day 3.71 and 4.09 ms (Allied, day 15), 7.22 and 7.18 ms (Japanese, day 17); a staff day with nothing to do, 1 and 31 microseconds (the copy of the truth that the area's view is built on).

The week of the slice from intents (`second-proof/07a_week.toml`), command levels on with the live report figures, seed 1: no order and no command is refused, for the cause of item 13 or any other (none with command levels off either); the staff gives 130 orders against 127 with command levels off; 705 state report events, late or lost.

### Tests

Scenario 63 passes its hand calculation with nothing ignored (21 tests): the game equal to 58's, the table of what the area command holds of task force 1 and of base 1 after every pulse, ship 1's fuel and group 3 as of the morning of day 1, "nothing there" sent when she launched, no late or lost report, and every property (not silent, a pulse late, never, the staff's silence at preserve the force, the commander's order, the seat, an order that meets a different world, every chance certain and no silence is step 2 on scenario 63, the Eastern Solomons from intents on seeds 1 to 10 and the week of the slice from intents on seed 1, off unchanged, the same start, the Japanese side seeing nothing). A held report sent on breaking silence and an order refused because the force was not as held are tested on views built for them in `crates/sim-core/src/held_tests.rs` (18 tests). `crates/sim-staff/tests/radio_silence_and_held.rs` tests the staff's silence order and an area doctrine reading a force where it is held; `crates/sim-agent/tests/area_seat.rs` the seat's `as_of` and the order. Every earlier scenario's run baseline is unchanged but for its hash lines; the recorded games changed only in their versions.

### The architect's rulings on the open points (Nov 17, 2026)

- **The afternoon's report and the first steps of the day:** left as built. A force back from a yard, or a commander's order taken by the override, after the afternoon's report is held as it was at the day's end. No game built has one; it is in `docs/open-work.md`.
- **Counting the refusals of item 14** is built with the tools of step 5, where the proof needs it.
- **A ship in port in no force** is read as the truth, in this step. A base reporting the ships in its port is a rule for later, in `docs/open-work.md`.
- **The supply run on stale word is the finding of this step.** With every state report lost, the area staff of scenario 36 never learns that its run has loaded, never sails it, forms a new run on the other ship every second day, and the base it was to feed starves. The staff does what the rule says: it reads what it was last told. A staff of 1942 with no word from a convoy would have reckoned that its orders were carried out. That rule, a staff that reckons a force forward from its own orders when word of it is stale, is not in this stage. It needs the machinery of stage 4, which plays a plan forward, and it is in `docs/open-work.md` as the first thing command levels ask of that stage. On the live placeholder figures the cost is small: the week of the slice from intents gives 130 staff orders against 127 with command levels off, and none refused. Every state report lost is a case built to be extreme.
