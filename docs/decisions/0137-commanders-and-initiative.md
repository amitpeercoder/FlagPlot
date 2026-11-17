# 0137: Commanders, and initiative in one rule: step 4 of stage 2b

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 2b, that the designer confirmed on Nov 16, 2026 (decision 0132, step 4 of its five, and his answer that initiative is built as one rule, last, with the stage judged without it). It follows decisions 0134 to 0136. Nothing is built.

Summary: Step 4 of command levels: a command may have a commander, from the scenario, with a name and an initiative rating of cautious, steady or bold, every rating a placeholder; the rating is read by one rule, the closing of a carrier force on a contact that is its own, which a cautious commander weighs at one level of risk more careful than his task's and a bold one at one level bolder; a contact he was told of, and everything else, is as before

Standing: Settled by the architect, Nov 17, 2026, inside decision 0132. Not built

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
