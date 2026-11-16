# 0134: Commands and their pictures: step 1 of stage 2b

Status: settled by the architect on Nov 16, 2026, inside the plan for step 3, stage 2b, that the designer confirmed the same day (decision 0132, step 1 of its five). It writes out the chain of command, the switch, a picture for each command and who reads which, with the hand calculation. Nothing is built.

Summary: Step 1 of command levels: a scenario may turn command levels on; each side then has an area command, and each of its task forces and bases is a command under it, each with a picture of its own; in this step every report reaches every command of the side at once, so each picture is the side's picture and a game plays as it does with command levels off; and every rule that reads what a side believes is changed to read the picture of the command that owns the decision

Standing: Settled by the architect, Nov 16, 2026, inside decision 0132. Not built

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
