# 0117: The intent tree in the command state, and the end "hold a place"

Status: settled by the architect on Nov 15, 2026, inside decision 0116, which the designer agreed, and step 2 of the plan for stage 2 that he confirmed (decision 0113). It says how much of the tree step 2 builds, and writes the first end beside supply out with its doctrine and its hand calculation. Not built. It is built with decision 0115, in one step.

Summary: What step 2 of stage 2 builds of the intent tree: an intent gains a parent, a maker and an addressee, the staff forms a sub-intent by a command of its own, limits are read up the tree and may only narrow, a reason names the chain above it; and the end "hold a place", under which the staff forms cover from the air

Standing: Settled by the architect, Nov 15, 2026, inside decisions 0113 and 0116. Not built

## Why

Decision 0116 says what an intent is and that intents form a tree. It leaves to each step how much of the tree is built. Step 2 needs two levels: one end, "hold a place", and one task under it, cover from the air (decision 0115). This record builds that much and no more, so that every later end and task is added by the same means.

## The tree in the state

1. **An intent gains three things.** Its **parent**, another intent of the side or none. Its **maker**, the commander or the staff. Its **addressee**, the level it is given to: the area, a base, or later a task force or a land formation. The level that owns an intent is the kind of its addressee, so one field serves both.
2. **The addressee is set by the kind in this step,** and nobody chooses it. Keeping a base supplied is addressed to the area, as stage 1 built it: it uses ships from anywhere, named in a forces limit. Holding a place and covering a base are addressed to the base they name.
3. **The staff forms a sub-intent by a command of its own,** `form_intent`: the parent, the kind and the reason. It takes the number one above the highest the side has ever used. Its maker is the staff. Intake refuses it where the parent is not in force, where the parent's plan is not released, or where the doctrine's kind cannot stand under the parent's kind (rule 12 has the one pair of this step).
4. **A commander's `set_intent` makes a root,** as it does today. A commander states a task himself by stating it as a root, as scenario 47 does with cover. Stating an intent under a parent of his own choosing waits for the operation and its phases, in stage 3.
5. **A staff-made intent is the staff's to change and to withdraw.** A commander's `set_intent` or `withdraw_intent` on its number is refused. He shapes it by its limits (rule 7), by holds, and by its parent.
6. **Withdrawing an intent withdraws everything beneath it,** on the same day, each with its own event caused by the parent's.
7. **Limits are read up the tree.** An intent with no limit of a kind has its parent's, and so on to the root. A commander may set a limit on any intent, staff-made or his own. On an intent with a parent, a limit must narrow: a forces limit names only what the parent may use, and a risk level is no bolder than the parent's (preserve the force, then balanced, then press). Intake refuses one that widens, with a refusal of its own.
8. **With no forces limit anywhere above it, an intent may use what its addressee commands** (decision 0116, 12). For a base in this step that is the side's air groups based there. Ships at a base are added with the step that gives ships a task. An intent addressed to the area has nothing without a forces limit, as stage 1 has it.
9. **One plan for a tree.** A root's plan is drafted when it is set and is made ready and released by the leaves, as stage 1 built it. A sub-intent has no plan of its own: it belongs to its root's plan, and forming it is not an amendment (decision 0116, 15). The staff forms nothing and orders nothing under a root whose plan is not released.
10. **A reason names the chain above it.** What a reason serves gains the intents above the one it serves, nearest first, each with its number, its maker and its statement in words as it stood. The top of every chain is an intent the commander stated. The engine records the chain and never reads it.
11. **Siblings are ranked by priority, then by number.** Step 2 has one kind of sub-intent, so the rule is stated and first tested in step 3.

## The end: hold a place

12. **A kind of intent: hold a place.** It names a base of the side. It has no date. Under it the staff may form cover from the air (decision 0115), and nothing else in this step.
13. **Its doctrine.** Each morning, for each hold intent in force whose plan is released, in priority order:
    - where a cover intent for the same base is in force, whoever made it, the staff forms none, and its reason names the one that stands;
    - where the intent may use no air group at the base that is a fighter or can search, it forms none, and its reason says there is nothing at the base to cover it with;
    - otherwise it forms cover from the air under it.
    The cover doctrine then runs for the new sub-intent on the same morning.
14. **Its measure** (decision 0116, 2), shown in the command report and by `command_state`: the days it has been in force, the days of those on which the base was the side's and its airfield open, the air raids on it and how many of them a patrol of the side fought, and the base's days of supply today. It is met on a day the base is the side's and its airfield is open.
15. **Supply under it waits.** Decision 0113 puts supply runs, escorted, under "hold a place". Until step 5 a commander who wants a held base fed states "keep supplied" beside it, as stage 1 has it.

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| Forming a sub-intent | The side's intents and their plans; for hold a place, the air groups at the base and their types | A new intent with its parent, maker and addressee; its event, caused by the staff's command |
| Limits read up the tree | An intent's limits and those of the intents above it; an addressee's air groups | Nothing: it is how intake and the staff read a limit |
| A limit that must narrow | The parent's limit as read up the tree | The limit set, or a refusal |
| Withdrawing a tree | The intents beneath the one withdrawn | Each one's withdrawn day, with its event |

## What it leaves out

- **Taking a staff-made intent over,** and stating a sub-intent under a parent the commander names. Stage 3.
- **A task force or a land formation as an addressee.** Steps 4 and 5, and stage 3.
- **The staff withdrawing a sub-intent** because what it needs has gone. The cover doctrine's notice that it cannot meet the intent serves until a doctrine needs more.
- **Friction between the levels** (decision 0116, 19).

## What it costs

- **State:** the three fields of an intent, in the hash and named beside it; the state version rises to 28. The rules label does not rise: no game without the new intents plays differently. Every staffed scenario of stage 1 gives the same orders, its intents now roots made by the commander and addressed to the area.
- **Saved games:** a reason's chain and the new fields; the saved game version rises.
- **The staff:** the doctrine of rule 13 beside the cover doctrine; the doctrine's version rises.
- **The tools:** `set_intent` takes the new kinds; `set_limit` works on any intent and takes air groups; `command_state` and `propose` show the tree, each intent under its parent with its maker and addressee; `why` prints the chain; the agent interface version rises.
- **The rulebook's chapter 8** gains "The intent tree" and "Holding a place"; the glossary gains the tree's words: end, sub-intent, maker, addressee.

## The hand calculation

`tests/scenario_48_hold_a_place.rs`: scenario 47's base, groups and contact, with the commander stating one end, "hold base 1", and nothing else. The staff forms cover from the air under it on the first morning and gives the orders of scenario 47, each serving the sub-intent with the end above it. Its properties test the limits read up the tree, a limit that must narrow, withdrawal, and a plan not yet released.

## The proof of the step

As decision 0115 has it, with the Allied side of scenario 6 given one end, "hold Henderson Field", with no forces limit and both leaves, in place of a cover intent stated directly.
