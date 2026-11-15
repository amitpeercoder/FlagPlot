# 0117: The intent tree in the command state, and the end "hold a place"

Status: settled by the architect on Nov 15, 2026, inside decision 0116, which the designer agreed, and step 2 of the plan for stage 2 that he confirmed (decision 0113). It says how much of the tree step 2 builds, and writes the first end beside supply out with its doctrine and its hand calculation. Built on Nov 15, 2026, with decision 0115, in one step ("As built" below).

Summary: What step 2 of stage 2 builds of the intent tree: an intent gains a parent, a maker and an addressee, the staff forms a sub-intent by a command of its own, limits are read up the tree and may only narrow, a reason names the chain above it; and the end "hold a place", under which the staff forms cover from the air

Standing: Settled by the architect, Nov 15, 2026, inside decisions 0113 and 0116. Built

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

## As built

Built on Nov 15, 2026, with decision 0115. What follows is what was built and every place the build read the record more closely or differed from it.

**State.** An intent has `parent`, `maker` (commander 1, staff 2) and `addressee` (the area 1, a base 2 with its number), hashed in that order after its kind; the kind "hold a place" is `IntentKind::Hold { base }`, code 3. Keep supplied is addressed to the area, hold and cover to their base, set by the kind (`IntentKind::addressee`); a commander's `set_intent` that changes a root's kind changes its addressee with it. A sub-intent's plan is its root's. The state version is 28, the rules label stays 12, the saved game version is 4, the doctrine version `supply-8`, the agent interface version 8. The invariants gain one: a sub-intent's parent exists, has a lower number and the same plan, a sub-intent is the staff's and a root the commander's, none is in force under a withdrawn parent, and the addressee is the kind's.

**Commands and events.** `form_intent { parent, kind }` is the staff's, added at the end; it takes `next_intent_number` (one above the highest the side has used) and is recorded by its `CommandTaken` and a new event `IntentFormed` (intent, parent) caused by it. It is refused where the parent is not in force ("is not in force"), its plan is not released ("the plan is ..., which the command does not apply to"), or the kind cannot stand under the parent's (new refusal `CannotStandUnder`). The commander's `set_intent` and `withdraw_intent` on a staff-made intent are refused with a new refusal, `MadeByStaff`. Withdrawing an intent records an `IntentWithdrawn` (intent, parent) for each intent in force beneath it, depth first, each caused by the withdrawal's `CommandTaken`; the plan ends only when the intent withdrawn is a root. Both new events are the side's own, whole, and the enemy's not at all.

**Limits up the tree** are `CommandState::forces_read_up`, `risk_read_up`, `priority_read_up` and `area_read_up`; the protected limit is not read up. A limit set on an intent with a parent must narrow, else the new refusal `WiderThanParent`: a forces limit names only ships in the parent's pool, task forces inside its limit, groups the parent may use and no source but the parent's (with no forces limit above, no ship, no force and no source); a risk no bolder than the parent's; and, reading decision 0116, 9, which says a child has part of its parent's area, an area inside the parent's. Priority and protected are not checked. A limit on a sub-intent amends its root's plan as any limit on an approved plan does.

**Holding a place** (rule 13) runs after the leaves and before the cover doctrine, in `crates/sim-staff/src/air.rs`. Readings: the hold doctrine hands in its reason every morning (the cover that stands, or nothing to cover with), serving the hold intent with an empty chain; the test of what is at the base counts every group the hold may use there, held or not, which the cover doctrine then leaves out; "a cover intent for the same base" is any in force on the side, in any tree.

**Reasons.** `Serves` gains `chain`, the intents above, nearest first, each with its number, maker and statement; the staff fills it from the command state as it stands after its own commands of the morning, so a sub-intent formed that morning is named. `why` prints it.

**The measure** (rule 14) is `sim_agent::hold_measure`, shown in `command_state` and the command report. It reads the side's own events: a day is met when the side's events have the base's `BaseAtDayEnd` that day and its airfield's damage at that day's end (from `AirfieldDamaged` and `AirfieldMended`) is below the ground figures' level. One limit of that reading: the side sees a base's end-of-day record only while the base is its own, so a hold whose base has been lost shows none of its earlier days met.

**What it was checked against.** Scenario 48 (`data/scenarios/48_hold_a_place.toml`, placeholder data) came out as the hand calculation has it in every number: the tree, the 17 orders by day (6, 1, 2, 1, 0, 0, 0, 0, 1, 3, 2, 1), each serving intent 2 and plan 1 with a chain of intent 1, the measure (12 days in force, 12 met, no raid), and every property. One reading of the calculation: "the reason of every morning without an order serves intent 2" is taken as the cover doctrine's reason; the hold doctrine's own daily reason serves intent 1 and is asserted apart. Stage 1 is unchanged: `sh scripts/compare-scenarios.sh --against origin/main` shows no scenario of stage 1 different with the hashes cut (the result is in the pull request).

**Left as the record leaves it.** A commander may re-set a root that has sub-intents to another kind; intake does not check the sub-intents against it. `command_state` gives an intent's place in the order served by the rank read up the tree, as the doctrine serves it.

