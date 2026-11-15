# 0116: Intents are ends, and they form a tree

Status: the designer's direction of Nov 15, 2026, written out by the architect, who proposed its shape and whose three refinements the designer agreed to the same day. It changes the command contract of `docs/step3-spec.md` ("Intent and limits", "The seat and the levels", "The plan's life"), as the spec says it may be changed: by a decision record that says what it changes and why. Nothing is built. Stage 2's plan (decision 0113) is rewritten on it.

Summary: An intent is an end, a state of the world a commander wants, and not an activity; intents form a tree in which the staff breaks an end into sub-intents until the leaves are orders; an intent is addressed to a level, which sets what it may use; a plan is the tree with its order in time; and one staff walks the tree, with the levels kept in the intents

Standing: Agreed by the designer, Nov 15, 2026. A change to the step 3 spec's command contract. Not built

## Why

Stage 2's first plan gave the commander seven kinds of intent: cover a place, strike what comes within reach, wear down a base, intercept, escort a run, keep a force fuelled, rest and repair. Most of them are things a staff does, with the commander's name put on them. A commander who must choose among them is choosing means, which is the work the first pillar gives to the staff.

The designer saw it first in reconnaissance: a commander who orders a place taken has already said that he needs to know what is on it, and should not have to order the looking. The same holds for the rest.

## What an intent is

1. **An intent is an end:** a state of the world a commander wants, on something on the map, that is met or not met. "The airfield stays open" is an end. "Fly patrols" is not.
2. **It has a measure the side can read from its own view,** so that the staff can say whether it is met, what it costs and what it displaces. A measure of what the enemy did is read from belief, and is judged as belief.
3. **It stands over time or has a date,** as the spec has it.
4. **A task** is a means the staff forms to meet an end: cover, search, strike, intercept, escort, a supply run, a reconnaissance. A task is itself an intent, lower in the tree, with an end of its own: this place is patrolled; these waters are searched.
5. **A policy** is how the staff behaves under every intent: the risk it runs, the reserves it keeps, how tired a unit may get, the fuel it keeps to get home. Policies are the limits of the contract and are not intents. Rest, repair and fuel are policy.

## The tree

6. **An intent may have sub-intents,** and they may have their own, until the leaves are orders. Orders are given only at the leaves.
7. **The staff breaks an end into sub-intents,** by doctrine, and hands them in as staff commands, each with its reason. A sub-intent names its parent. A commander may state a sub-intent himself, at any level, and the staff fills in what lies below it.
8. **Every intent says who made it,** the commander or the staff, and which level owns it.
9. **Limits pass down and only narrow.** A child has part of its parent's forces and area and no more risk than its parent allows. Siblings are ranked by priority, the higher never short so that a lower is met, and what is displaced is reported at each level, as the contract has it for intents today.
10. **Holds, overrides, approvals, leaves, interrupts and reasons work on any node,** as stage 1 built them for an intent and its plan. A commander's order to a unit is an override at a leaf.
11. **A reason names the chain above it.** The reason for an order gives the task it serves, the end that task serves, and so on to an intent the commander stated. This is what a player follows to see why a thing was done.

## Whom it is given to

12. **An intent is addressed to a level:** the area, a base, a task force, a land formation. With no forces limit set, it may use what the addressee commands: a task force's ships, a base's air groups, a formation's units. So a forces limit is rarely set by hand, and is still a hard limit when it is.
13. **The tree stops at what can act.** A task force is the smallest thing at sea that takes an order; inside it the rules decide at the point of contact, as chapter 1 of the rulebook lists. A ship is given no intent. An air group and a land unit are leaves of the same kind. On land the tree goes a level deeper when part of a unit can act apart from the rest, which is a rule for stage 3.
14. **Land formations need an order of battle with parents:** division, regiment, battalion. The game has units and no tree of them. It is data for stage 3.

## The plan

15. **A plan is the tree of an intent with its order in time.** Some branches run side by side and some in sequence; a sequence is the plan's phases, each with the condition that begins and ends it, as the spec has them. A plan's baseline is the stated intent and its limits; the sub-intents the staff forms under it are its to form and to change, inside the limits, as runs of a supply plan are.
16. **A proposal is the tree the staff would form,** shown before it is formed: the sub-intents, what each would use, what binds and what is displaced. Approving it approves the tree's top; the leaves are still the staff's.
17. **Collection is a branch.** What a plan needs to know is a sub-intent of it, formed by the staff, as `docs/design/intelligence.md` ("Reconnaissance as part of an operation") has it.

## One staff, and the levels kept

18. **One staff walks the tree.** A side has one picture of itself and of the enemy, and an order takes effect the next morning whoever gives it. Staffs at each level would decide what one staff decides. So the levels are recorded in the intents and not built as separate staffs.
19. **Friction in command is not in this decision.** Delay in passing an order down, a subordinate's narrower view, and his initiative would make the levels real. They are a question for the pillars and for a later step; the level on each intent is where they would attach. The designer took the question up the same day: `docs/design/command-levels.md` has his direction, that a staff at every level interprets and splits intent on its own picture and that levels differ in what they know and report, with what it would take to build.

## What it changes in the spec

- **"Intent and limits":** an intent is an end (decisions 1 to 5); it may have a parent, a maker and a level (6 to 8); limits pass down (9).
- **"The seat and the levels":** the three levels of the table, area, task and unit, become levels of the tree. The area intent is its root. "Area intent is in the first stage that has a staff, in a narrow form, and is widened stage by stage" stands: stage 2 builds two levels, an end and its tasks; stage 3 adds the operation and its phases.
- **"The plan's life":** a plan is the tree with its order (15 and 16). What the staff may change alone is the sub-intents it formed; what needs the commander is unchanged.
- **Stage 1 as built is a tree of one level:** a supply intent with its runs. Nothing built is undone.

## What it costs

- **State:** an intent gains a parent, a maker, a level and an addressee, in the hash and named beside it. Staff-made intents are staff commands, saved and replayed as the rest are.
- **The tools** show the tree, and `why` follows a reason up it.
- **Each rule that breaks an end into sub-intents is a doctrine,** with its hand calculation before code.
- **A diversion,** which the designer gave as an example of a sub-intent, means nothing until the enemy's staff acts on what it sees. It waits for that, in stage 3 or after.
