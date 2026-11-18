# 0158: Plans a person can read, reconnaissance as part of a plan, and trying a plan: the direction

Status: a direction by the designer, recorded by the architect on Nov 18, 2026, from the design conversation on the table. Part of it is drafted in the table; the rest is to be planned with stage 4 of step 3. Nothing in the engine, the staff or the seat's tools is changed by this record.

Summary: The direction for what a plan is to a person: it reads as an order, a short brief first and the detail beneath, and a day on which the staff gave no order is not part of the brief; what must be found out before a decision is a stated part of the plan, with who looks, what it takes in time and forces and what risk it buys down; a plan can be tried on the latest intelligence before it is approved; and wherever a thing is named the name leads to what is known of it. The links and a brief-first plan sheet are in the table; reconnaissance stated in the plan, the commander's say over it and the trial are to be planned with stage 4

Standing: Direction given by the designer, Nov 18, 2026. The links and the brief-first sheet are drafted in the table; the rest waits for stage 4's plan

## What the designer said

"I want to create a game that simulates pseudo-war-making. The plans should be understandable, with a proper brief and the ability to drill down for further details." "There's (probably) no reason to show 'no order today' in the higher level plan." On reconnaissance: if he states an intent and the staff needs to look first, "should it have it part of the plan, including what forces need to move in order to get such and such level of recon, to reduce risk (pay in time and resources)?" On trying it: "the ability to simulate a plan (according to the latest intelligence)". And on names: each fight, strike and raid in the report, a tile's intelligence, and "whenever a unit is mentioned" could carry "a URI of sorts that will enable the player to focus on what we know about a specific unit".

## What is so today

- **A plan is a tree of intents with reasons.** `command_state`, `plans` and `staff_orders` give it; the staff's reasons are written for a reader who wants every figure, and a day with no order is a reason like any other.
- **The staff does look before a landing** (decision 0147, items 13 to 20). For a "take a place" whose plan is preparing, the risk word sets a need to know: at preserve a count of the enemy seen from the air or by men in contact, a few days old at most; at balanced such a count of any day; at press none. One air group is sent to look, the units are chosen again on what it finds, and at preserve the landing is not released without it. This is in the staff's reasons and nowhere shown as part of the plan; the commander's only say over it is the risk word; and only an air group already in reach looks. No force is moved in order to look.
- **Trying a plan is stage 4 of step 3,** the trial and the estimate, as `docs/step3-spec.md` has them: a private world built from what the side may know, never the live enemy, played at two or three dates, with its assumptions shown. It is not built.

## The direction

1. **A plan reads as an order.** At the top: the mission and its purpose; how the staff means to do it, in phases, each ended by a condition and not by a date; the few decisions that are the commander's, each with what must be known first; what it needs and what it puts at risk. Beneath each line, one step down: the tasks, the forces, the day's decisions and their reasons, the figures. This follows the common form of a military order (situation, mission, execution, what it needs, who commands), taken as a lead from a short survey and not from a source read in full.
2. **The brief leaves out what did not happen.** A task for which the staff gave no order today is "standing", and why is in the detail.
3. **What must be found out is part of the plan.** For each decision that waits on knowledge the plan says what must be known, where, who will look and when the answer is expected, what looking costs (days, sorties, a force moved into reach and exposed there), and what is risked by going without it. The commander can ask for more or for less than the staff proposes, apart from the risk word. A force may be moved in order to look.
4. **A plan can be tried before it is approved,** on the side's latest picture: the trial of stage 4, shown with the plan as its estimate, with what was assumed of what is not seen.
5. **A name is a link.** Wherever a force, a base, a ship, an air group, a land unit or a contact is named, in a report, a plan, a card or a signal, the name leads to what is known of it, and a thing can be pointed at by an address.

## As drafted in the table

- **Links.** Every such name in the evening report, the plan sheet and the cards is a link: it opens that thing's card and turns the chair toward it. The page's address ending `#see=task_force:1` does the same. Ships, air groups and land units have cards of their own, read from `get_entity` and the lists.
- **The report tells what was fought.** A fight at sea, a strike (once, by its target, with the groups and aircraft sent and how many attacked), an attack on the side's ships, a raid and a bombardment are each told in a line, with their names as links.
- **The plan sheet is brief-first.** One line a task with a word for where it stands (waits for you, cannot be met, orders today, preparing, standing, done, ended) and a count of each at the top; the staff's reasons are one step down under each line. It is not yet the order of part 1: there are no phases, no decisions with what must be known, and no estimate, because the tools do not give them.

## To be planned, with stage 4

Stage 4's opening plan takes these with the two mends and with waiting on the schedule (`docs/open-work.md`): they are one subject, a plan that knows what it waits for. In order: reconnaissance stated in the plan and the commander's say over it; a force moved to look; phases with their conditions in what the staff hands in, so that a seat's tool can give a plan as an order; then the trial and the estimate, which is stage 4 proper.
