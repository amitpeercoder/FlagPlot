# 0132: Step 3, stage 2b: command levels, in five steps

Status: a plan by the architect, written on Nov 16, 2026, for the designer's approval, as the spec asks of every stage (`docs/step3-spec.md`, "Built in stages"). It plans the stage decision 0119 added, from the designer's direction in `docs/design/command-levels.md` and his answers in decisions 0119 and 0129. Nothing is built. Each step has its own decision record and hand calculation before any code.

Summary: A plan for step 3, stage 2b: each task force and each base becomes a command with its own picture of the enemy, changed only by what it sees and what is reported to it; reports travel up, down and between neighbours in radio reach, on time, a pulse late or lost; a superior knows his forces as last reported, and a force may keep radio silence; a commander has an initiative rating read by one rule; a scenario turns command levels on, and with them off every game plays as it does today; refusal and relief are left out

Standing: A plan for the designer's approval, Nov 16, 2026. Not confirmed. Not built

## What stage 2b is

Today a side has one picture. A sighting by one flying boat is known in the same pulse to every ship and group of its side, and a side knows each of its own units exactly. Stage 2b gives each command its own picture, so that a decision is made on what the command that owns it knew at the time.

It is the stage that makes the first pillar necessary and not merely convenient: a commander who learns of a contact after his captain has launched on it can only command by intent.

Stage 2 prepared it. Every doctrine since decision 0120 belongs to one level and reads only what that level has. Stage 2b changes what they read and not what they are.

## The one switch

1. **A scenario turns command levels on.** With them off, a side has one picture and knows its own forces exactly, as today, and every game plays as it does today: the same hash on every scenario built so far. This is a condition of the stage, checked on every step.
2. **Why a switch.** The four tuned battles and every hand calculation were made with one picture a side. They stay as they are. A battle is fitted again with command levels on only by a later decision, when the communications figures rest on research.
3. **The player's easier setting** (decision 0119, answer 5) is a different thing: with command levels on, the area commander's seat may be shown the whole side's picture. The staffs below play on their own pictures either way.

## The commands

4. **A command** is a thing that holds a picture and is given intents: the area command, each task force, and each base with its air groups and the land units at it. They are the addressees decision 0116 already has.
5. **The chain of command is data:** each command names its superior. The slice has two levels, the area and the forces under it. The data allows more (a service level, a theatre), and stage 2b builds and proves two.
6. **A unit is not a command.** A ship, an air group and a land unit act by the standing rules where they are, on the picture of the command they belong to. A carrier's groups read their task force's picture; a base's groups read the base's.
7. **A command has a commander,** from data: a name, and an initiative rating (step 4). No real commander's rating is presented as fact. Each is a placeholder with a note, as the data rules have it.

## What a command knows

8. **Its own sight,** in the pulse it saw it: its searches, its lookouts, its fights, the raids on it.
9. **What is reported to it,** when the report arrives.
10. **Its own units exactly;** a force a level down, as last reported (step 3).
11. **Nothing else.**

## Reports

12. **A report is a thing in the state:** who sent it, to whom, what it says, the pulse it was sent and the pulse it arrives. What it says is what a sighting says today (an estimated place, count and kind, each of which may be wrong), or a claim, or the sender's own state.
13. **Upward.** In each pulse a command sends its superior what it newly saw, what it claims and its own state.
14. **Sideways.** Every command of the side within the sender's radio reach hears the same report (decision 0119, answer 4).
15. **Downward.** An intent goes down with the contacts, enemy bases and friendly forces inside its area and dates (answer 3), and with word of them as the superior learns more. "Inside its area" is the area limit read up the tree.
16. **Time and loss.** A report arrives in the pulse it was sent, or one pulse later, or not at all, by a draw for each listener (answer 6). The three chances are figures. They are placeholders until research gives better, and the research is asked for now (below).
17. **Radio reach** is a figure of a ship's set, by the kind of ship, and of a base's. Placeholders, with the same research.
18. **A report is belief.** It passes on what the sender believed. A listener applies it to its picture through `BeliefState::apply_report`, the one way a picture changes, and cannot check it.

## Built in five steps

Each step is one decision record, one hand calculation, one pull request. Each leaves every earlier scenario playing as before.

| Step | What is built | Its hand calculation shows |
| --- | --- | --- |
| 1 | The chain of command as data, the switch, a picture for each command, and a view for a command as `SideView` is for a side. Every report arrives at once, so that with command levels on a game plays exactly as with them off | The same orders and the same hash, on and off, in two staffed scenarios |
| 2 | The enemy picture by command: own sight, reports up, sideways and down, on time, late or lost. Every rule that reads belief reads its own command's: the strike decision, the search arcs, each doctrine | Two task forces and a base: a sighting reaches one force a pulse before the other, the first strikes and the second does not, and each reason says what its command held |
| 3 | A superior's knowledge of his forces by report, and radio silence: a setting of a force, set by the staff from the risk of its task; a silent force sends nothing and hears everything | An area staff that routes a supply run on where it last heard its escort was; a silent carrier force its superior holds a day stale |
| 4 | Commanders as data, and initiative read by one rule (below) | A bold and a cautious commander given the same sighting of their own |
| 5 | The tools and the proof: `command_state` and `why` by command, the agent interface able to sit at a command and be given its picture, the viewer able to show one command's picture beside the truth; then the proof | The proof below |

## Initiative, in one rule

Decision 0119, answer 1, makes initiative a rating of each commander. The designer's direction of Nov 16, 2026 leaves refusal and relief out of this stage. So the rating is read by one rule, and it changes how a force commander acts on a sighting of his own that his superior does not yet hold:

| Rating | A sighting of his own, not yet handed down to him |
| --- | --- |
| Cautious | He acts on it only inside his task's area and at his task's risk |
| Steady | He acts on it as his doctrine has it, as today |
| Bold | He acts on it at one level of risk bolder than his task's, and never bolder than press |

"Acts on it" is the two things a force commander's doctrine already decides: whether his aircraft strike it and whether he closes on it. He reports what he did. He never leaves his task's forces limit, and nothing here is a refusal.

This is the part the architect is least sure of. It is built last of the rules so that the stage can be judged without it.

## What it must pass

Set before anything is run, as decision 0128's were. Each step's own scenario passes its hand calculation. Then:

1. **Off is unchanged.** Every scenario built before this stage gives the same hash with command levels off as it does on main today. Judged on every step.
2. **On, with every report at once, is the same game** as off (step 1). Judged.
3. **The battle the spec asks for.** The Eastern Solomons with both sides from their stated intents of decision 0128, command levels on, over 1,000 seeds: every invariant holds, no staff order is refused, and the same start gives the same game. Judged.
4. **A sighting that reaches one command before another.** On at least 200 of those 1,000 seeds, some sighting is held by one command of a side a pulse or more before another command of that side that later decides on it, and the two decide differently in that pulse (one launches or closes, the other does not). Judged. The mark is low because the battle is one day long.
5. **Each can state its reason.** For 12 such cases, picked by a rule written before the run, a reader given only the two reasons says correctly which command knew of the sighting and which did not, and why each did what it did. Pass: 10 of 12. Judged.
6. **The week,** days 15 to 21 of the slice from the stated intents, command levels on: the conditions of decision 0128 are run and reported beside its third run. Reported and not judged: late reports are meant to cost something, and how much is what the report is for.
7. **Savo Island,** scripted, command levels on: the pulse in which each Allied command first holds the sighting of the Japanese cruiser force. Reported and not judged.
8. **Speed.** A staffed day of the week with command levels on takes no more than three times what it takes with them off. Judged.

Nothing is tuned to pass. A failure is reported with its cause from the staff's own reasons, and goes to the designer.

## What it leaves out

- **Refusal and relief.** Left out by the designer on Nov 16, 2026: to be added if play shows the need, for example intents systematically ignored or orders not obeyed. The standing with his superior that decision 0119 gave a commander is not built.
- **More than two levels** played and proved. The data allows them.
- **Fitting the tuned battles again** with command levels on.
- **People or models in several seats of one side** (decision 0129): step 5 gives a seat its picture; handing intents down from a seat that is not the area's, and several players on a side, come with the interface, after the kill gate.
- **A finer clock** than the three pulses.
- **The enemy listening** to radio traffic, which is what silence is for. It is in `docs/design/intelligence.md`. Until it is built silence costs a superior his knowledge and buys nothing, so the staff sets it only where a scenario says, and step 3's record says so.

## What it costs

- **State:** a picture for each command (the slice has some tens a side), the reports in flight, each command's last report of its forces, the chain of command. All in the hash and named beside it. The state version and the saved game version rise at step 1.
- **Every rule that reads belief** is changed to read a command's picture. With the switch off that is the side's one picture, which is how "off is unchanged" is kept.
- **Events:** a report sent and a report arrived are events, each with its causes; `side_events.rs` says who may see each, and now also which command.
- **Speed and memory,** which condition 8 bounds.
- **Research:** a prompt on how long a contact report took to reach each level in 1942, how often one was lost or garbled, radio ranges, and the practice of radio silence. Its answers are leads, as always.

## Risks

- **It may not be fun** to be told late where your own carriers are. The kill gate judges it.
- **Legibility gets harder first:** a reason must say who knew what. Condition 5 is there for it.
- **The pulse is coarse:** a report is on time or a third of a day late.

## Questions for the designer

1. **The switch.** Command levels are on only where a scenario says, and every battle already tuned stays off until a later decision. The architect recommends it: nothing already proved moves.
2. **Initiative in one rule,** as the table above has it, built as step 4. The architect recommends building it, last, and judging the stage without it. The other course is to leave the rating out with refusal and relief.
3. **Silence with no enemy listening.** The architect recommends building the setting in step 3 and having the staff set it only where a scenario says, until the intelligence layer gives it a use.
4. **The pass marks,** above: 200 of 1,000 seeds for condition 4, 10 of 12 for condition 5, three times for speed.
