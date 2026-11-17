# 0137: Commanders, and initiative in one rule: step 4 of stage 2b

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 2b, that the designer confirmed on Nov 16, 2026 (decision 0132, step 4 of its five, and his answer that initiative is built as one rule, last, with the stage judged without it). It follows decisions 0134 to 0136. Built on Nov 17, 2026 ("As built" below).

Summary: Step 4 of command levels: a command may have a commander, from the scenario, with a name and an initiative rating of cautious, steady or bold, every rating a placeholder; the rating is read by one rule, the closing of a carrier force on a contact that is its own, which a cautious commander weighs at one level of risk more careful than his task's and a bold one at one level bolder; a contact he was told of, and everything else, is as before

Standing: Settled by the architect, Nov 17, 2026, inside decision 0132. Built

## Commanders

1. **A command may have a commander,** named in the scenario: a name, the command, and an initiative rating: cautious, steady or bold.
2. **Every rating is a placeholder.** The scenario gives each a note of what it rests on, and the loader refuses one without it. No rating of a real man is presented as fact (decision 0119, "What follows from them").
3. **A command with no commander named is steady.** So is every command while command levels are off: a commander is read only with them on.
4. **A commander goes with his command.** A force that ceases to be takes its commander with it. Changing a commander in the game, relief and refusal are not in this stage (decision 0132, "What it leaves out").

## A contact of his own

5. **A contact in a command's picture is its own** where the report that last made or refreshed it there was that command's: its own search, its own strike, its own fight. A contact last made or refreshed by a report sent to it, from above or from a neighbour, is one it was told of.
6. **A contact a scenario starts a side with** is one every command was told of. A scenario may say that one command saw it: it is then in that command's picture alone, as its own, and is sent to no one.

## The one rule

7. **The rating is read by the closing rule of a carrier force** (decision 0130, 2), and by nothing else. For a contact that is the force's own, the kinds the force closes on are read at:

| The commander | A contact of his own is weighed at |
| --- | --- |
| Cautious | One level of risk more careful than his task's: press as balanced, balanced as preserve the force, which closes on nothing |
| Steady | His task's risk |
| Bold | One level bolder: preserve the force as balanced, balanced as press |

8. **A contact he was told of is weighed at his task's risk,** whatever his rating.
9. **Nothing else moves with it.** The speed, the fuel that comes first, where he goes, his search, his patrol share, his strike settings and his break-off level are his task's risk's, as before. He stays inside his task's forces. Nothing here is a refusal.
10. **The reason says so.** Where a rating changed how a contact was weighed, the reason for closing or for not closing names the commander, his rating, that the contact is his own, and the level it was weighed at.

## Why only this

The plan had the rating also move whether a commander's aircraft strike. A group's strike settings are the group's, set once for every contact it may meet, and not for one contact; a rule that set them by whose contact it was would be a second way of setting them. And the plan had a cautious commander act "only inside his task's area". A carrier task rarely has an area limit, so the rule would rarely bite. The table above keeps the plan's intent, that a man's temper shows in how far he follows his own eyes, in the one place the doctrine already weighs a single contact.

It is the part of the stage the architect is least sure of, and the stage is judged without it (decision 0132).

## Rule card

- **Reads:** a command's commander and his rating; for each contact in a carrier force's picture, whether it is the force's own; the task's risk.
- **Writes:** nothing new but the commanders, from the scenario, and on each contact of a picture whether the last report was the command's own.
- **Reuses:** the closing rule of decision 0130 whole; the pictures of decision 0134; the reports of decision 0135, which already say who made each.

## What it costs

- **State:** the commanders, and one mark on a contact in a picture, in the hash and named. The state layout and saved game versions rise. The doctrine version rises. The rules label does not: no game without a commander plays differently.
- **Scenario data:** the commanders, and the key by which a starting contact is one command's own.

## The hand calculation

`tests/scenario_64_commanders_and_initiative.rs`.

## As built

Built on Nov 17, 2026. A commander is `Commander` with its `Initiative` (codes 1 cautious, 2 steady, 3 bold) in `crates/sim-core/src/commander.rs`, held in his command's `CommandPost` (`commander`), named by `World::name_commanders` before day 1. The mark of a contact of a command's own is the post's set `own` of contact numbers, kept by `CommandPost::apply` and `CommandPost::age` in `crates/sim-core/src/pictures.rs`, set by `World::apply_report` for the commands that hold a report first and cleared by `hear` in `crates/sim-core/src/passing.rs` for a command a report is sent to. A starting contact one command saw is applied with `Holders::Only` by `World::hold_starting_contacts_seen` (`crates/sim-core/src/start.rs`). A side reads both through `CommandView::commander` and `CommandView::is_own`. The rule is `weighed_at` and `closing_by` in `crates/sim-staff/src/carrier.rs`, read in `serve_carrier`'s weighing of the contacts; `closing` is `closing_by` with every contact at the task's risk. The scenario keys are `[[commander]]` (`side`, one of `task_force`, `base` or `area = true`, `name`, `initiative`, `note`) and `seen_by = { task_force = N }` or `{ base = N }` on a `[[contact]]`. The state version is `flagplot-state-v38`, the saved game version 13 and the doctrine version `supply-19`. The rules label stays 18 and the agent interface version 15: no game without a commander plays differently, and no answer of the agent reads a commander or the mark. Scenario 64 is `data/scenarios/64_commanders_and_initiative.toml` with `data/reference/reports_placeholder_scenario_64.toml` (scenario 63's figures, on time in every direction).

Every number of the hand calculation came out as written, and no test is ignored: the bold commander closes with ten orders on day 1, none on day 2, two on day 3, arriving at (6,0) at hour 8 and at (0,0) at hour 56, with no launch; the contact is in task force 1's picture alone, as its own, at the side's ages, and dropped at the end of the morning of day 2; steady gives eight, none and one; cautious on his own carrier eight, and no commander on the same carrier ten; a contact he was told of eight; bold at press scenario 58's twelve on day 1 and the orders and reasons of the file with no commander; cautious at preserve the force the orders, reasons and events of the file with no commander; bold at preserve the force on his own carrier the route to (6,0) with no speed order, at hour 15; command levels off eight, with the orders, reasons, events and hash of the file with no commander and no `seen_by`.

### Readings taken

1. **Where the mark is.** On the command's post, as a set of the picture's contact numbers, and not on the contact: the side's own belief, which every rule with command levels off reads, carries no mark, and its layout is unchanged.
2. **With no report figures** every report still reaches every command at once (decision 0134), and the commands that hold it first mark it their own while the rest were told of it, as item 5 reads. Nothing reads the mark there but a carrier force's commander.
3. **A report heard that changes nothing** (decision 0135, 15) leaves the mark as it was; a contact dropped, by ageing or by "nothing there", loses it.
4. **A command formed in the game** (a detach, a force formed in port, a base formed by a landing) takes the contacts of the picture it copies as ones it was told of, and has no commander. A base that changes side takes its commander with it, and the side that takes it has none for it.
5. **A commander with command levels off** is checked by the loader and not held in the state at all: there is no command to hold him, and item 3 has him read only with them on. With them on he is in the hash with his command.
6. **What the loader refuses,** with command levels on or off: a rating with no note or an empty one, a commander for a task force or base the scenario does not give that side, an unknown rating, two commanders for one command, and a `seen_by` naming a command the side does not have. With them on the world refuses too a commander or a `seen_by` for a force that is not a command at the start (one with no ship). An area command and a base may have a commander; nothing reads his rating.
7. **`seen_by` with command levels off** is read and changes nothing: the contact is the side's, as item 6 has it for one every command was told of.
8. **Preserve the force.** The closing rule's first test, that preserve the force closes on nothing, holds where every fresh contact is weighed at preserve the force; a bold commander's fresh contact of his own, weighed at balanced, is weighed on. A cautious commander at preserve the force and a bold one at press weigh as their task does, and no reason's words change.
9. **The first contact** (the architect's ruling of Nov 16, 2026, recorded in decision 0130) is taken from the fresh contacts of a kind closed on, each at its own level; the night at full speed is read for that one.
10. **The reason's words.** For each fresh contact whose level a rating changed, a sentence: "Placeholder Bold, commanding task_force:1, is bold: contact 1 held by task_force:1 is task_force:1's own, so he weighs it at the press risk, one level bolder than the task's balanced risk, and at the press risk the force closes on any kind". On a closing route it takes the place of "at the balanced risk it closes on a carrier" and the binding says "a bold commander weighs a contact of his own at the press risk, and at that risk the force closes on any kind ..."; on any other morning it follows the words on closing. Where no fresh contact is of a kind closed on and a rating changed one, the words are "no contact of ships 3 pulses old or less is of a kind it closes on at the risk it is weighed at", each contact with its level, since the old words ("at the balanced risk it closes on a carrier, and no contact ... is of that kind (contact 1, a carrier)") would contradict themselves. The kinds named in the words on staying and on a contact in reach are those of the contact's own level, which is the task's where no rating changed it.
11. **The rules label** is not raised, as the decision says: a game saved before cannot hold a commander, and one without a commander plays as before (every earlier scenario's run baseline is unchanged but for its hash lines).
12. **Earlier tests touched,** as each scenario with command levels on did before: scenario 64 is added to the lists of files that turn command levels on in the tests of scenarios 61 and 63 and to the files the pinned-hash tests of scenarios 6d1 and 6d2 leave out, its report figures to `LIVE_ONLY` in `crates/sim-data/tests/frozen_figures.rs`, and the saved game version the saved-game tests read is 13. No expected number of any hand calculation changed.
