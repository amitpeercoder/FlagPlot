# 0136: What a superior knows of his forces, and radio silence: step 3 of stage 2b

Status: settled by the architect on Nov 16, 2026, inside the plan for step 3, stage 2b, that the designer confirmed the same day (decision 0132, step 3 of its five, with his answer that the staff or a scenario may set silence). It follows decisions 0134 and 0135. Nothing is built.

Summary: Step 3 of command levels: at the end of each pulse a task force and a base report their own state to the area command, which holds each as last reported, with the pulse it is of; an area doctrine and the area commander's seat read that and not the truth; an order that cannot be carried out because the force is not as it was believed is refused with that cause; and a force may keep radio silence, set by a scenario, by the commander or by the staff at preserve the force, sending nothing until it fights or strikes

Standing: Settled by the architect, Nov 16, 2026, inside decision 0132. Not built

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
