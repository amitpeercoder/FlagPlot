# 0134: Commands and their pictures: step 1 of stage 2b

Status: settled by the architect on Nov 16, 2026, inside the plan for step 3, stage 2b, that the designer confirmed the same day (decision 0132, step 1 of its five). It writes out the chain of command, the switch, a picture for each command and who reads which, with the hand calculation. Built on Nov 16, 2026 ("As built" below), with one point left to the architect: no record names the command whose picture a strike read.

Summary: Step 1 of command levels: a scenario may turn command levels on; each side then has an area command, and each of its task forces and bases is a command under it, each with a picture of its own; in this step every report reaches every command of the side at once, so each picture is the side's picture and a game plays as it does with command levels off; and every rule that reads what a side believes is changed to read the picture of the command that owns the decision

Standing: Settled by the architect, Nov 16, 2026, inside decision 0132. Built, one point open ("As built", the launch record)

## Why this step is first

Stage 2b changes who knows what. Before any report is made late or lost, the plumbing has to be right: every command has a picture, and every decision reads the picture of the command that owns it. This step builds that and nothing else. Because every report still reaches every command at once, a game with command levels on must play exactly as the same game with them off. That is a test that can fail, and it is the whole proof of the step.

## The switch

1. **A scenario turns command levels on** by one key. With it absent or off, nothing of this record exists in the game: no command, no picture but the side's.
2. **No scenario built before this step has it on.** Each plays as it did.

## Commands

3. **An area command for each side,** made when the game starts.
4. **Each task force and each base of a side is a command** under its side's area command. A scenario may name another superior for a command; the slice does not, and two levels are what is built and proved (decision 0132, 5).
5. **A task force formed during the game** is a command under the area command from the pulse it is formed. Its picture starts as a copy of the picture of the command that formed it: the force it was detached from, or the area command where the staff or the commander formed it from ships in port.
6. **A task force that is no more** (merged, sunk, dissolved) stops being a command and its picture is gone.
7. **A base that changes side** stops being a command of the side that lost it. It becomes a command of the side that took it, with a picture copied from that side's area command.
8. **A ship, an air group and a land unit are not commands.** Each belongs to one: a ship to its task force; an air group to the task force of the ship it is aboard, or to its base; a land unit to the base it is at, and to the area command where it is at no base of its side.

## Pictures

9. **A command's picture** holds what a side's belief holds today: contacts, claims and the estimates of the enemy's men ashore. It is changed only by a report applied to it, through `BeliefState::apply_report`, and by ageing.
10. **In this step every report is applied to every command's picture of the side,** in the pulse and in the order in which it is applied to the side's own. So every picture holds what the side's holds, contact for contact and number for number. Step 2 changes this and nothing else in this record.
11. **The side's own picture stays.** It is what a side with command levels off plays on, and with them on it is the picture of decision 0119's easier setting.
12. **Contacts a scenario starts a side with** are in every command's picture of that side at the start.

## Who reads which picture

13. **A decision reads the picture of the command that owns it:**

| The decision | The command |
| --- | --- |
| Whether an air group strikes, and at what | The group's command |
| Whether a task force is alerted by a fresh contact nearby, and everything a force decides at the point of contact | The task force |
| A doctrine of the staff | The command its intent is addressed to: the area, a base or a task force |
| The estimate of the enemy ashore that a land fight reads | The base where the fight is, or the area command |
| What a player or an agent in the area commander's seat is shown | The area command |

14. **Nothing reads another command's picture,** and nothing that plays reads the truth, as now.
15. **A view for a command,** as `SideView` is for a side: a command's contacts, claims and estimates. A side's own units are still known to it exactly through the side's view; step 3 changes that.
16. **With command levels off, "the command's picture" is the side's.** One path through the code, so that off is unchanged by construction and not by a second copy of each rule.

## What does not change in this step

- **No report is late or lost,** and no command is silent.
- **A side still knows its own forces exactly.**
- **No new event.** A report applied to a picture is already an event, `ReportReceived`, and in this step it is the same report for every command.
- **No commander.** Step 4.
- **The tools** show a command's picture only through the view; the agent interface and the viewer are step 5.

## Amendment to the plan

Decision 0132's table says step 1 shows "the same orders and the same hash, on and off". The hash cannot be the same: with command levels on the state holds the commands and their pictures, and they are in the hash. What is the same is every event, every order and reason, and every field of the state but the commands. The hand calculation asserts that.

## Rule card

- **Reads:** the scenario's key; a side's task forces and bases; every report a side receives.
- **Writes:** the commands of a side and each one's picture.
- **Reuses:** `BeliefState` as the picture, `apply_report` as the one way it changes, the ageing step, the side's view.

## What it costs

- **State:** the switch, the commands and their pictures, in the hash and named beside it. The state layout version and the saved game version rise. The rules label does not: no game plays differently.
- **Every reader of a side's belief** in `sim-core` and `sim-staff` is changed to ask for a command's picture. "As built" lists each one and the command it now reads.
- **Speed:** each report is applied once for each command. Measured and reported with the step.

## The hand calculation

`tests/scenario_61_commands_and_their_pictures.rs`: scenario 58, the carrier force that closes, with command levels on.

## As built

Built on Nov 16, 2026. The core is `crates/sim-core/src/pictures.rs` (the commands, the one path a report and an ageing take, the hooks for a force formed, a force gone and a base that changes side or is formed) and `crates/sim-core/src/view/command_view.rs` (a command's view). The scenario key is `command_levels = true`; a named superior is `superior = { task_force = N }` or `superior = { base = N }` on a `[[task_force]]` or `[[base]]` entry (`crates/sim-data/src/scenario.rs`). The state version is `flagplot-state-v35` and the saved game version 10; the rules label is unchanged, since no game plays differently. Scenario 61 is `data/scenarios/61_commands_and_their_pictures.toml`.

### Every reader changed, and the command it now reads

| Reader | Where | The command |
| --- | --- | --- |
| The strike decision (`naval_strike_launch`) | `sim-core`, `airstrike.rs` | The group's command |
| `SideView::naval_strike` and `strike_choice`, the strike question | `sim-core`, `view.rs` | The group's command |
| Alert from belief, and the report its event cites | `sim-core`, `surface.rs` | The task force (the area command once it has ceased to be) |
| The alert question | `sim-core`, `view/surface_choices.rs` | The task force, likewise |
| Whether a fight ashore is certain (the ground question) | `sim-core`, `view/ground_choices.rs` | The base of the side in the fight's hex, else the area command |
| The threat to a run (escorting a run) | `sim-staff`, `escort.rs` | The command the intent is addressed to: the area for keeping a base supplied |
| The threat to a station | `sim-staff`, `stand.rs` | The task force the intent is addressed to |
| The contacts a carrier force weighs closing on, and its groups' arcs | `sim-staff`, `carrier.rs` | The task force the intent is addressed to |
| The threats a base's search arc covers, and the contacts lost since an arc was set | `sim-staff`, `air.rs` | The base the cover intent is addressed to |
| The briefing's contacts and estimates ashore, `list_contacts`, a contact named in `get_entity` and `measure` | `sim-agent`, `briefing.rs`, `queries.rs`, `measure.rs` | The area command |
| The briefing's word on a strike group that launched nothing | `sim-agent`, `briefing.rs` | The group's command, the picture its strike decision read |

A doctrine reaches its picture through one helper (`sim-staff/src/picture.rs`), the agent through two (`sim-agent/src/common.rs`), the engine and the view through `pictures::picture_of`; with command levels off each returns the side's own belief.

Not changed, on purpose: `SideView::contacts`, `claims`, `ground_estimates` and `contact_by_number` still give the side's own picture (item 11); and the check before a report that a hex is empty (`estimate.rs`, `report_empty`) reads the side's own belief, since it decides whether a report is made, not what a command does.

### Readings taken

1. **A task force is a command** while it has a ship, or a ship away at a yard to rejoin it (decision 0069): that is a force that has not ceased to be. At the start every such force of a side is one.
2. **A force that ceases to be** (joined, stood down, sunk) stops being a command at the end of the pulse in which it ceased, before the pictures age; until then its picture still takes every report. A rule that asks for the picture of a force that has ceased to be gets the area command's.
3. **A command whose superior is gone** is put under the area command.
4. **A base formed by a landing** (decision 0066, 11) is a command from the pulse it is formed, with a copy of the landing force's picture. Item 5 names task forces only; a base formed in play is read the same way.
5. **Which command a unit belongs to** where item 8 is silent: an air group aboard a ship in port in no force belongs to that base; aboard a ship at a yard, to the force she rejoins; a group lost or aboard a sunk ship, to the area command. A land unit belongs to the lowest-numbered base of its side in the hex its men ashore are in, and to the area command aboard ship or ashore away from one. A fight ashore reads the lowest-numbered base of the side in its hex.
6. **A doctrine whose addressee is no longer a command of the side** (a base lost, a force gone) reads the area command's picture.
7. **With command levels off**, a command's view exists for the area, each task force and each base of the side, and holds the side's belief; `SideView::commands` is empty, since no command exists.
8. **A report that a strike found nothing** names a contact by the number it has in the picture the strike was decided on, and is applied by that number to every picture. In this step every picture numbers its contacts as the side does, so it is right; step 2, where pictures differ, must carry the hex alone or the number in each picture.
9. **The agent's seat is the area command's** (item 13's last row). Whether it is shown the side's picture instead, the easier setting of item 11, is left to step 5, which builds the seats; in this step the two are the same.
10. **Copies are made with the pulse's refreshed set** of the picture they copy, so a copy made in a pulse ages at its end as its source does.

### Open: the launch record

The hand calculation asks that the strike of the morning of day 1 name task force 1 as the command whose picture was read, in its reason or the launch's record. Nothing was added: the strike decision is a rule of the engine with no staff reason, and any record of it (a field of `StrikeLaunched`, a factor of its breakdown, or an entry in the trace of choices) would differ between the game with command levels on and the same game off, which the property "on is off" forbids for every event. The staff's own orders to task force 1 and its groups already name it as the level that decided (decision 0125). The options, for the architect:

- **A field of `StrikeLaunched`,** the command whose picture was read, `None` with command levels off and left out of the event's text when `None`: off is unchanged, and "on is off" is restated to compare events but that field.
- **A breakdown factor,** present only with command levels on: the same restatement, for breakdowns.
- **No record:** the command is a function of the state at the launch (`SideView::command_of_group`), which the test asserts; a reason by command comes with step 2, where it first matters.

Scenario 61's `the_launch_record_names_task_force_1` is ignored with that message; `the_strike_of_day_1_is_launched_by_a_group_of_task_force_1` asserts the third.

### Speed

`sim-cli time --slice data/scenarios/second-proof/07a_week.toml --early-day 15 --late-day 18 --repeats 5`, release build, on the week of the slice from intents and on the same file with `command_levels = true` put first (made for the measure and not kept): six staffed days from day 15 took 67.7 ms with command levels off and 67.4 ms with them on (medians of 5), about 11.2 ms a day either way; three days from day 18, 27.5 ms and 27.7 ms; the longest day's staff decisions 3.84 and 3.83 ms (Allied, day 15), 7.35 and 7.33 ms (Japanese, day 17). The cost of applying each report to every picture is within the spread.

### Tests

Scenario 61 passes its hand calculation, with the one test above ignored: the commands at the start, the table of pictures at the start and the end of every pulse, the game equal to scenario 58's, the reasons naming contact 1, on is off, balanced, a commander's detach that forms nothing, nothing that plays given another command's picture, every earlier scenario file off, and the bigger games (the Eastern Solomons from intents and Savo Island on seeds 1 to 20, the week of the slice from intents on seeds 1 to 3: the same events and end truth, and every command's picture equal to its side's at every pulse). `crates/sim-core/src/pictures_tests.rs` tests the force formed (detached, and formed in port), the force gone (joined, sunk, at a yard), the base that changes side, the base formed by a landing, and off. Every earlier scenario's run baseline is unchanged but for its hash lines.

**The architect's ruling on the launch record (Nov 16, 2026).** No record names the command whose picture a strike read, in this step: the third of the options above. The command is what the side's view gives for the group, which a test asserts. The hand calculation asked for more than the step allows, and is restated. Step 2 writes who knew what into the record, where it first differs; it also changes the "nothing there" report, which is applied here by a number that every picture shares, and looks again at the check before a "hex is empty" report, which still reads the side's own belief.
