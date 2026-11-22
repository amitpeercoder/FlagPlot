# 0221: The table is laid out round the stop

Status: a plan by the architect, written on Nov 22, 2026, on the designer's words at the table: "Maybe we don't need Give an order? It's done more naturally now. As for Next pulse and Carry on, maybe we could give them a different place and clear this area? ... should we look at an overhaul for the game screen after all we've gone through?" The architect had the screen audited, drew three layouts, and recommended the third; the designer chose it: "C, the stop paper." It is to be read by a reviewer who has not seen it written before anything is built.

Summary: The table grew a surface at a time. At a stop of the clock the reason is in one place, the decision in another, what happened in a third, and what the player signed nowhere. This lays the screen out round the stop: one paper down the left side holds, top to bottom, the clock, why it stopped, what waits for the player, what happened since the last stop, his orders and what he has signed, and the two controls that run time. Cards open on the right. One form gives every order. The bar at the foot, the signals tray, the reports panel and the button "Give an order" go.

Standing: Planned on Nov 22, 2026; to be reviewed.

## Why

What the audit of `table/index.html` found (the architect holds it; its line numbers are of main on that day):

- **A direct order has no home once signed.** A force's is said once in the status line; an air group's shows on that group's card until the clock moves; a force formed in port shows in a line that goes. Nothing lists what was signed, and nothing takes one back.
- **The status line is the only place many outcomes are said,** refusals among them, and 48 places write to it, the last winning.
- **The reports panel and the card are one box,** left 16, top 60; the panel has no close.
- **Why the clock stopped is in three places** (the status line, a slip in the tray, the reports) in three wordings; a plan that waits is in six.
- **Three forms give orders** and differ in their controls, their buttons, whether they say when the order acts and whose hands the unit is in.
- **What stops the clock is set in four places** with four lifetimes, one of them forgotten at each load, one that cannot be switched back on.
- **With a card and the tray open the panels cover two fifths to a half of the window,** and the chart turns to the window's middle, under them.
- **"Give an order" reaches one thing nothing else does:** an order on the waters where a force of the player's lies, since a force's card offers no orders on its waters.

## The layout

1. **The stop paper** is a column down the left side, the window's full height, 400 wide. It is always there in play. Top to bottom:
   - **The head:** the game's name, the seat, the date and hour, the day of how many, and whether the clock is stopped or running.
   - **WHY THE CLOCK STOPPED:** one or more lines, each a sentence with links, for each reason of this stop. Not shown where the stop is the player's own.
   - **FOR YOUR DECISION:** each thing that waits for him: a force that asks leave, a plan that waits, a directive unread. Each has its controls in place. A count is in the heading.
   - **SINCE the last stop:** the reports of the pulses played since, the three first and "and n more", which opens them all in the paper.
   - **YOUR ORDERS:** the orders in force, each a link to its plan; and SIGNED, each direct order and command signed and not yet acted on, with when it acts and TAKE BACK.
   - **The foot:** the two controls that run time, and a fold, WHAT STOPS THE CLOCK.
2. **While the clock runs** the head says so, the foot's main control is STOP THE CLOCK, and the reports come into SINCE as they are plotted.
3. **Cards open at the right,** one at a time, with their trail and their stepping as now. A paper for reading (a plan, the evening report, the war diary, the order of battle) opens over the chart's part of the window, not over the stop paper.
4. **Three menus stay at the top right:** STAFF, CHART and GAME. ORDERS as a menu goes: its papers are in the stop paper. The in-tray of directives is under FOR YOUR DECISION while one is unread and in STAFF after.
5. **The chart turns to the middle of what is not covered,** and a link to a place does the same.
6. **In a narrow window** (under 900 wide) the stop paper is a drawer that covers the chart and is opened by the clock, which stays at the top.

## One way to give an order

7. **"Give an order" goes.** An order begins from what is touched. A force's card gains the orders on its waters that a hex's card offers, and a touch on open water opens the hex's card as now.
8. **One order paper gives every order,** to the staff or to a unit, with the same parts in the same places:
   - **TO:** the staff, or the unit by name;
   - **the order:** its choices, by the controls the kind needs;
   - **IN WHOSE HANDS:** for a unit, whose it is now and whose it will be;
   - **ACTS:** when it acts, before signing ("Acts at once" or "Acts at 1800, for the coming day", decision 0219);
   - **the staff's reading,** for an order to the staff, shown open;
   - **SIGN AND SEND, and TEAR UP.** Esc tears up a draft that has nothing chosen and asks for one that has.
9. **Signing closes the paper and puts the order under SIGNED** in the stop paper, with when it acts. A refusal keeps the paper open and says NOT SENT with the reason in the paper, not in a line elsewhere.
10. **TAKE BACK** withdraws what was signed and has not acted: an order kept for the evening is cleared (decision 0219); one given this stop for the pulse to come is withdrawn if the tool can withdraw it, and otherwise the control is not shown for it and the paper says before signing that it cannot be taken back. Which the tool can do is the reviewer's to find.

## One place for what stops the clock

11. **The fold WHAT STOPS THE CLOCK holds every stop:** the table's own ticks, the staff's interrupts, the leave to launch plans, and the pursuit setting of each order by a link to it. Each can be switched off and on there. A slip's "do not stop me for this" stays, and what it switched off is found again in the fold.
12. **The table's own ticks are kept with the game** for as long as the page keeps anything for it, and not forgotten at each load.
13. **Run without stops other than those:** where the table runs by days, the clock stops each evening unless the player unticks "each evening".

## Words

14. **One word for each thing,** in every place: an order is withdrawn; a draft is torn up; a signed order is taken back; a unit is "kept from the staff" and "returned to the staff"; a plan "waits for your leave".
15. **Captions that explain a rule go,** as agreed on Nov 19, 2026: the notes under each end of an order, "before you sign", the foot of the plan's paper, and the like. What a player must know before signing is said as a fact of this order ("Leaves the staff's hands").
16. **Text of the engine does not reach the screen unworded:** the situation's text goes through the table's words as the other papers do.

## What goes

The bar at the foot and its status line; the signals tray; the reports panel; the Orders menu; "Give an order"; the plan chip, whose READ and close move to the plan's line under YOUR ORDERS; the dead diary button on the card.

## What it keeps

The chart and its pieces, marks and labels; the cards' content; the papers' content; the menus STAFF, CHART and GAME; the new-game sheet; every tool call the page makes; a watched game and a game against an agent, whose seat slips go under the head.

## How it is built

Four stages, each its own pull request, the table playable after each:

1. **The stop paper and the clock.** The column, its head, WHY, FOR YOUR DECISION, SINCE and the foot; the tray, the reports panel and the bar go; cards move to the right; the chart's turning; the narrow window.
2. **Your orders and what is signed.** YOUR ORDERS and SIGNED with TAKE BACK; the Orders menu and the plan chip go; outcomes and refusals said where they belong.
3. **One order paper.** The three forms become one; a force's card gains the orders on its waters; "Give an order" goes.
4. **Stops, words and what is left.** The fold of stops; the words; the captions; keyboard and focus (Esc on a draft, focus returned when a paper closes, a card not read out whole at each open).

`table/check_screen.py` is kept passing at each stage, its literal checks moved with the code they check, and gains checks of the stop paper's parts.

## What it does not do

- **No rule of the game changes,** and nothing in the engine, but a tool to withdraw a direct order if stage 2 finds none and the designer wants one.
- **It does not redraw the chart or the cards' content.**
- **It does not add the debrief, or the orders of decisions 0216 and 0220;** they are built into the new layout when they are built.

## Questions for the designer

None before the review. He chose the layout.
