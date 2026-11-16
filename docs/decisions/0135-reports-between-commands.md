# 0135: Reports between commands: step 2 of stage 2b

Status: settled by the architect on Nov 16, 2026, inside the plan for step 3, stage 2b, that the designer confirmed the same day (decision 0132, step 2 of its five). It follows decision 0134, which gave each command a picture and had every report reach every command at once. This record says who holds a report first, to whom it is passed, when it arrives and what a late one does to a picture. Nothing is built.

Summary: Step 2 of command levels: a report is held first by the command that saw the thing; it is passed up to the area command, sideways to every command of the side within the sender's radio reach, and down from the area command to the commands under it; each passing arrives in the same pulse, a pulse later, or never, by figures for each direction; a late report makes a contact as old as it is and never replaces newer word; and every passing is an event, so that a reason can say what its command held

Standing: Settled by the architect, Nov 16, 2026, inside decision 0132. Not built

## Whose a report is

1. **A report belongs first to the command that saw the thing,** and is applied to that command's picture in the pulse it is made, as now:

| The report | The command that holds it first |
| --- | --- |
| A sighting by an air group's search | The group's command |
| A sighting, a claim or "nothing there" from a strike | The command of the groups that flew it |
| A sighting or a claim from a fight at sea | Each task force of the side that fought |
| A raid on a base, a bombardment of it, and what its men count or find of the enemy ashore | The base |
| What a scenario starts a side with | Every command of the side, as decision 0134 has it |

2. **An aircraft's word to its own ship or base is not delayed.** A crew that did not report until it landed is a real case and is left to the research.

## Passing a report on

3. **Up.** In the pulse a command first holds a report of its own, it sends it to its superior.
4. **Sideways.** In the same pulse every other command of the side within the sender's radio reach is sent it: a task force by where it is, a base by where it is. The area command has no place: it is reached by 3 and never by this.
5. **Down.** In the pulse the area command first holds a report it was sent, it sends it to every command under it but the one that sent it. Decision 0119's "what bears on the intent" is every report, in this step, for a command whose intents have no area limit, and the reports inside the area limit, read up the tree, for one that has.
6. **Nobody passes on what it was only told.** A task force or a base sends its own reports only. With two levels that reaches everyone it can.
7. **Radio reach** is a figure in nautical miles for each kind of ship and for a base. A task force's reach is its ship's with the longest.

## When it arrives

8. **Each passing, to each listener, arrives in the pulse it was sent, in the next pulse, or never,** by one draw for that listener and that report. The chances are figures for each of the three directions: on time and a pulse late, the rest lost.
9. **The figures are placeholders,** marked so, in `data/reference/reports.toml`, until the research asked for (decision 0132, "What it costs") gives better. Live: on time 0.7, a pulse late 0.25, for each direction; reach 600 nm for a carrier, a battleship and a cruiser, 300 nm for another ship, 1,000 nm for a base.
10. **With no report figures,** as on the frozen step 1 figures, every report reaches every command at once, as decision 0134 has it. Scenario 61 stands.
11. **The draw** is keyed, as every draw is, on seed, day, pulse, the report's event and the listener, with a `rng::Draw` value of its own added at the end. A chance of 0 or 1 is still drawn, so that changing a figure changes no other draw.
12. **A report that arrives a pulse late** is applied at the start of that pulse, before anything of that pulse is seen, in the order of the reports' events.

## What a report does to a picture that gets it late

13. **A command hears a report once.** The same report sent to it by two roads is applied the first time it arrives and ignored after.
14. **A late sighting makes or refreshes a contact at the age the sighting has,** the pulses since it was made, and not at 0. It is matched as any sighting is (decisions 0012 and 0133).
15. **It never replaces newer word.** Where the contact it matches was last made or refreshed by a later sighting than this one, the report is heard and changes nothing.
16. **"Nothing there"** is carried by its hex and the pulse the strike arrived. In the picture of the command that flew the strike it drops the contact by its number, as now. In any other picture it drops the contact at that hex, if there is one whose last sighting is not later than that pulse.
17. **A claim and a count of the enemy ashore** are applied as they are, when they arrive.
18. **The check before a "hex is empty" report** reads the picture of the command whose men stand in the hex.

## The side's own picture

19. **The side's own picture still gets every report at once.** It is the picture of the easier setting (decision 0119, answer 5) and of a side with command levels off. With command levels on no decision reads it.

## Numbers and reasons

20. **Each picture numbers its own contacts.** A reason names a contact by the number it has in the picture of the command that decided, and says which command that is.
21. **A passing is an event,** "report sent": the report, the sender, and for each listener whether it arrives on time, a pulse late or never, with the draw's breakdown. The report's own event is its cause. A side sees its own.
22. **A decision's reason can say what its command held:** a staff reason that names a contact names the command whose picture it is, and `why` on a launch or an order follows the report's event to the "report sent" that brought it to that command and the pulse it arrived.

## What it leaves for later steps

- **A superior's knowledge of his own forces,** and silence: step 3.
- **Garbling on the way.** A report passed on says what the first report said.
- **A command that passes on what it heard,** which a third level would need.
- **Reports that bear on an intent chosen more finely** than by its area limit.

## Rule card

- **Reads:** the commands and their pictures, the chain, where a task force and a base are, a ship's kind, the report figures, the area limits of the intents addressed to a command.
- **Writes:** each command's picture; the reports in flight (a report, its listener, the pulse it arrives); for each command, the reports it has heard that are not yet older than a contact may be.
- **Reuses:** `BeliefState::apply_report`, the keyed draws, the events and their causes, decision 0134's one path for a report.

## What it costs

- **State:** reports in flight and what each command has heard, in the hash and named beside it; the state layout and saved game versions rise. The rules label does not: with command levels off no game plays differently, and no scenario built so far has them on with report figures.
- **A new kind of event,** which does not compile until `side_events.rs` says who sees it.
- **A new figure file** and its loader.
- **Speed,** measured with the step.

## The hand calculation

`tests/scenario_62_reports_between_commands.rs`.
