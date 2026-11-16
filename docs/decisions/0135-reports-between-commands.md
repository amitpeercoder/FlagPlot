# 0135: Reports between commands: step 2 of stage 2b

Status: settled by the architect on Nov 16, 2026, inside the plan for step 3, stage 2b, that the designer confirmed the same day (decision 0132, step 2 of its five). It follows decision 0134, which gave each command a picture and had every report reach every command at once. This record says who holds a report first, to whom it is passed, when it arrives and what a late one does to a picture. Built on Nov 16, 2026 ("As built" below), with three points left to the architect.

Summary: Step 2 of command levels: a report is held first by the command that saw the thing; it is passed up to the area command, sideways to every command of the side within the sender's radio reach, and down from the area command to the commands under it; each passing arrives in the same pulse, a pulse later, or never, by figures for each direction; a late report makes a contact as old as it is and never replaces newer word; and every passing is an event, so that a reason can say what its command held

Standing: Settled by the architect, Nov 16, 2026, inside decision 0132. Built, three points open ("As built", "Open")

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
11. **The draw** is keyed, as every draw is, on seed, day, pulse, the report's event and the listener, with a `rng::Draw` value of its own added at the end. **A report sent down has a draw value of its own, apart from the one for up and sideways** (ruled on Nov 16, 2026, after the build: as first written a report that went to one command by two roads, sideways and down, had one number for both, so the second road was no second chance). A chance of 0 or 1 is still drawn, so that changing a figure changes no other draw.
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

## As built

Built on Nov 16, 2026. The passing is `crates/sim-core/src/passing.rs` (`pass_report`, `send`, `hear`, `deliver_late_reports`, `prune_heard`, `ReportInFlight`); who holds a report first is `Holders` and `World::apply_report` in `crates/sim-core/src/pictures.rs`, with `by_hex`; a late sighting's age, newer word and "nothing there" by hex are in `BeliefState::apply_report` (`crates/sim-core/src/belief.rs`). The figures are `data/reference/reports.toml` (`ReportFigures` in `refdata.rs`, loaded by `sim-data`), named by a scenario's key `reports`. The event is `ReportSent` (`events.rs`), seen whole by its own side and not at all by the other (`side_events.rs`). The draw is `Draw::ReportPassed` (code 9) in the intelligence subsystem, keyed on `EntityRef::Event` (kind code 8, new) and the listener. The state version is `flagplot-state-v36` and the saved game version 11; the rules label is unchanged. Scenario 62 is `data/scenarios/62_reports_between_commands.toml` with `data/reference/reports_placeholder_scenario_62.toml`.

### Where each report is made, and who holds it first

| Where | The report | Held first by |
| --- | --- | --- |
| `start.rs` | A scenario's starting contacts | Every command |
| `estimate.rs`, opening estimates | A scenario's estimates ashore | Every command |
| `search.rs` | An air search's sighting | The searching group's command |
| `airstrike.rs` | "Nothing there" | The command of the group that found nothing |
| `airstrike.rs` | A strike's sighting of the force it attacked | The attacking group's command |
| `airstrike.rs` | A raid's claim, attacking side | The commands of the raid's strike groups (escorts not counted) |
| `airstrike.rs` | A raid's claim, defending side | The commands of its units that fired (patrol groups by their command, guns by their task force), and the base for a raid on a base |
| `surface.rs` | A fight's sighting and claim | The task force whose report it is (each force's report is made for it) |
| `bombardment.rs` | The base side's sighting of the bombarding force | The base |
| `bombardment.rs` | The bombarding side's claim | The bombarding task force |
| `estimate.rs` | A daily ground count, "none found" | The base of the side in the hex, else the area command |

### Readings taken

1. **The report figures are not redirected** by `--reference` or `LoadOptions::reference_root`: a scenario names them where they are, as it does its map. The frozen step 1 figures have none (`data/reference/step1/README.md`, `LIVE_ONLY` in `crates/sim-data/tests/frozen_figures.rs`).
2. **With no report figures, or command levels off,** nothing of this record happens: no draw, no event, every report to every picture at once (item 10). Scenario 61 is unchanged.
3. **A report is known by an event:** a sighting by its `ReportReceived`, a claim, a ground count and "none found" by their own events, "nothing there" by the strike's `StrikeFoundNothing`. That event keys the draws, the heard sets and the order of late reports, and is the cause of each "report sent".
4. **The area command as a holder** (a ground count away from any base, a group whose place is lost) sends its own report down, as it would one it was sent.
5. **Two holders of one report** (a raid's defenders in two forces) each send it; the area command sends it down once, the first time it holds it, leaving out the command that sent it.
6. **The listeners of a passing** are its superior first, then the commands in reach in command order; every command within reach is sent it, even one that has heard it, and hears it once (item 13). A passing with no listener is not recorded.
7. **Radio reach** is measured between hex centres in nautical miles, by the engine's air distance; within means not more than the reach. A force's reach is its longest-reaching ship's, counting the ships in it.
8. **The area limit of item 5:** a command gets a report sent down if no intent in force is addressed to it, or one of them has no area read up the tree; else only one about a hex in the union of those areas. An intent addressed to an air group counts for the group's command. A report's hex is its sighting's, its claim's, its "nothing there"'s or its count's hex.
9. **The roll** is one number in 0 to 1, always taken, compared with the on-time chance and then with on time plus late. In the breakdown it is named `draw`, as a raw die is everywhere, so that a side is not given it (decision 0085); the factors are, for each listener in order, `on_time_chance`, `late_chance` and `draw`, and the result is the number of listeners it reaches on time or late.
10. **The draw is keyed on the report's event and the listener,** as item 11 says, so two passings of one report to one listener in one pulse (sideways from a force and down from the area) share one number, and with figures alike they arrive alike. See "Open".
11. **A report made at the end of the day** (a ground count, "none found") has no pulse: its draws and its event are keyed on no pulse; it is counted as made in the afternoon, and a late one arrives in the next night.
12. **Late reports** are applied right after the pulse begins and before the alert from belief, in the order of the reports' events and then of the listeners. One to a command that is no more is dropped with no event. The area command that first holds a late report sends it down in that pulse, with draws of that pulse.
13. **A late sighting's contact counts as refreshed in the pulse it arrives,** so it is not aged at the end of it: made in pulse 4 and arriving in pulse 5 it is 1 at the end of 5 and 2 at the end of 6, as in the maker's picture (in the maker's it is 0 during pulse 5, in the hearer's 1).
14. **Newer word** is told by event number: a sighting whose matched contact was last made or refreshed by a report of a later event changes nothing. In the side's own picture and with no figures it never happens, since reports are applied in the order they are made.
15. **"Nothing there" by hex** drops the lowest-numbered contact at that hex. Item 16's "whose last sighting is not later than that pulse" always holds when it is applied: on time, nothing later exists yet; late, it is applied at the start of the next pulse before anything of it is seen. The side's own picture, with report figures, hears it by hex too (its numbers are not the strike's command's).
16. **`ContactNotThere`** is recorded when the picture of the strike's command held the contact at the hex, and names the contact by that picture's number. With command levels off or no figures that picture is the side's, as before.
17. **What each command has heard** is kept, by the report's event and the pulse it was made, until it is as old as the contact drop age (`contact_drop_age_pulses`), and pruned at the end of each pulse after the ageing. A command formed by a copy has heard what the copied one had.
18. **The check before a "hex is empty" report** reads the picture of the base of the side in the hex, else the area command (`command_at_hex`): the command whose men stand there, as item 18 asks.
19. **A reason names the command** only when reports travel (command levels on and report figures loaded): "contact 1 held by task_force:2" (`contact_words`, `crates/sim-staff/src/picture.rs`). With no figures the words are as before, which scenario 61 asserts. The reason that a contact dropped since an arc was set names the command whose picture the doctrine reads now.
20. **`events --why`** prints, beneath any event that is a report's event, each "report sent" of that report with its listeners and arrivals (`crates/sim-cli/src/events.rs`). It shows every passing of the report, not only the one to the command that decided: a launch does not record its command (decision 0134's ruling).
21. **`ReportSent` in the briefing:** the agent's briefing does not list it; the export gives it a line (`report_sent`).

### Open, for the architect

- **One number for two roads.** Item 11 keys the draw on the report and the listener, so the two roads of item 13 share a number and a report lost sideways is usually lost down as well (with the live figures, the same roll compared with the same chances gives the same arrival). In the Eastern Solomons below, task force 2 lost one report by both roads on seed 1. If the second road is meant to be a second chance, the key needs the sender (or the direction).
- **"On is off but for report sent" and the reasons.** With every chance certain the events equal those of the game with command levels off but for the "report sent" events and the words " held by <command>" in the staff's reasons, which item 22 adds. Scenario 62's test removes that clause before comparing.
- **The launch record.** As in decision 0134, no event names the command whose picture a strike read; `why` shows every passing of the report.

### Tests

Scenario 62 passes its hand calculation with nothing ignored (20 tests): the twenty orders of day 1, the two "report sent" events of the morning with three and two listeners, the table of pictures, day 2's two orders and task force 2's reason, day 3's one, and every property, the bigger games included (the Eastern Solomons from intents on seeds 1 to 10, the week of the slice from intents on seed 1, every command's picture equal to its side's at every pulse end). "A late sighting is as old as it is" and "newer word stands" need a picture built in the engine's own tests: `passing_tests::a_late_sighting_is_one_pulse_old_then_one_and_then_two_as_the_makers_is` and `passing_tests::newer_word_stands` (`crates/sim-core/src/passing_tests.rs`, 29 tests of the rules). Every earlier scenario's events are unchanged; only hash lines moved. Three tests that walk every scenario file leave scenario 62 out as they leave out 61 (`scenario_61`'s file test, `scenario_06d1` and `scenario_06d2`'s hash tests).

### Measured (for the architect, not judged)

`sim-cli time --slice ... --early-day 15 --late-day 18 --repeats 5`, release, on the week of the slice from intents, the same with `command_levels = true`, and the same with the live report figures as well (files made for the measure and not kept): six staffed days from day 15, 67.4, 67.6 and 68.7 ms (medians of 5); three from day 18, 27.5, 27.6 and 28.1 ms; one day of the slice 13.1, 13.1 and 13.2 ms early, 8.66, 8.70 and 8.75 ms late; the longest staff day 4.24, 4.32 and 4.33 ms (Allied, day 15), 7.62, 7.60 and 7.65 ms (Japanese, day 17).

The Eastern Solomons from intents (`second-proof/05a_eastern_solomons.toml`), command levels on with the live report figures, seeds 1 to 200, release: the Allied side launches a strike at ships on 168 seeds and the Japanese on 194 (168 and 195 with command levels off). Of 21,106 passings to a listener, 14,846 arrived on time, 5,266 a pulse late and 994 never.
