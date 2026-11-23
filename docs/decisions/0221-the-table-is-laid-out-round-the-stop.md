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

## Amended on review, Nov 22, 2026

Read by a reviewer who had not seen it written, and after the designer, looking at the drawing, missed the chart's controls: "show/hide plans, 2D/3D, zoom in/out". Where this differs from what is above, this holds.

### What the review found

- **Nothing can take back one signed order.** The tool that gives orders can only empty the whole queue; the tool for commands can only empty it while adding another; no tool lists the direct orders queued, so a list of what was signed would live in the page's memory and be lost at a reload. Decision 0216 as amended says in terms that an order signed is not taken back singly.
- **The first stage could not have shipped alone.** It took away the status line, where 48 places say their outcomes, the refusals, the prompts while a place is picked, and the second touch of New game and Restart, before their new homes were built; and "Give an order" before a force's card offered the orders on its waters.
- **The order paper had no place** among the stop paper, the cards and the reading papers, and an air group's search arc is drawn only while its card is open.
- **A column of 400 by 800 overflows:** a stop of the table's own can tell fifty lines; an interrupt's reasons open at nearly half the window's height; thirty reports opened in place push the orders off the screen. On the first evening the column is nearly empty and the one cue to give an order went with the button.
- **Stopping each evening by default** would make CARRY ON the same as the one-day control and take away the quiet week that passes in seconds.
- **The leave to launch is not a stop,** and with its stop switched off a plan would wait for ever unseen. A standing interrupt can be switched back on by the tool; only the page lacked the control. A switch given inside a day takes hold at 1800.
- **There is no CHART menu today:** the chart's seven controls are buttons, and two of them are the zoom.
- **Three listeners answer Escape at once.**

### The stop paper, restated

1. **Its head and foot do not scroll:** the clock above and the controls that run time below are always in sight. What lies between scrolls as one.
2. **It is 360 wide in a window under 1440 and 400 from there.** Under 1100 it is a drawer, opened by the clock, which stays at the top; while open it covers the chart, takes the focus, and Escape closes it.
3. **WHY THE CLOCK STOPPED says at most three lines for each kind of reason** and "and n more", which opens them as a paper for reading. An interrupt's reasons open as a paper too, not in the column.
4. **SINCE keeps the reports' own headings by pulse and their folding of a kind under one line,** shows the three latest lines, and opens all as a paper for reading. "Since" is since the clock last stopped in this sitting; after a reload it is the day's.
5. **An empty part says a fact and nothing more:** "No order in force." under YOUR ORDERS; FOR YOUR DECISION is left out when nothing waits.
6. **When the game is over** the head says so, WHY holds how it ended as a link to its paper, and the foot's controls are READ HOW IT ENDED, and under GAME, New game and Restart.
7. **A watched game has no FOR YOUR DECISION, YOUR ORDERS or SIGNED.** Its seat slips and the switch of sides are under the head. **Against an agent** the head has a third state, waiting for the other commander, in which nothing can be signed or taken back and the foot says so.
8. **A paper for reading wins the right side over a card:** the card is put by and comes back when the paper closes, as now.

### The chart's controls

9. **They are always in sight, at the foot of the chart on the right, and not in a menu:** what the chart shows (PLANS, PIECES, NAMES, HEXES, each lit when on, and cycling through what it cycles through today); the view (TABLE or FROM ABOVE); and the zoom, in and out. The wheel, the pinch and the drag work as now. Only STAFF and GAME are menus, at the top right. The agent's seat is under GAME.

### Running time

10. **Rule 13 is withdrawn.** CARRY ON runs to the next thing that is ticked to stop the clock, as now. The second control runs one step and is named for the game: NEXT PULSE where the game is played by pulses, NEXT DAY where by days. A tick "each evening" is in the fold, off unless the player ticks it.

### Giving an order, restated

11. **The order paper opens at the right, in the card's place,** with a way back to the card. It stays open while the chart is touched for a place, a course or an arc, and the chart draws what is being drafted while the draft is open.
12. **The parts are the same for every order; the controls between them are the kind's own:** a course's list of points, an arc's bearing, width and reach, a force's ships, the staff's reading and its try.
13. **NOT SENT says, line by line, what went and what did not.** An order refused leaves nothing of it queued: the hold the page sends before a unit's order is withdrawn where the order is refused.
14. **The commands a plan's paper and a card give** (approve, cancel, a risk, a pursuit setting, keep from the staff, return to the staff) go under SIGNED as an order does.

### Taking back

15. **Two tools of the session are new,** and nothing in the engine: one lists what is queued for a seat, its direct orders and its commands, each with its place; one withdraws a single queued entry by its place. They are the session's, which holds the queues (decision 0219, rule 7). The agent's interface rises.
16. **Withdrawing an entry withdraws what rests on it:** the hold sent before a unit's order; the limits that name an order to the staff queued with them. Where another queued entry names what this one would make (an order to a force not yet formed) the tool refuses and names it.
17. **SIGNED is read from that tool,** so it is right after a reload. TAKE BACK is offered for each entry until the clock runs; nothing is offered after.
18. **Decision 0216's sentence that an order signed is not taken back singly is withdrawn by this.** A land unit's orders are taken back as any other.

### Stops, restated

19. **The fold holds the table's ticks** (a contact, an attack, a loss, a strike, a landing, reinforcements as decision 0215 has them, each evening) **and the staff's standing interrupts,** each a tick that can be cleared and set. A tick of the staff's changed inside a day shows "from 1800".
20. **The table's ticks are kept in the browser for the game,** keyed as its directives are, and cleared by New game and Restart. A page that cannot keep them plays as now.
21. **The leaves are not in the fold.** The leave to launch is a line of YOUR ORDERS ("Plans launch: when ready" or "on my word"), with its switch; a plan that waits for leave is under FOR YOUR DECISION whether or not its stop is ticked.

### Words, restated

22. **A unit the player holds is "Yours: you keep it out of the staff's plans",** as decision 0216 has it; an order signed for the evening "waits for 1800"; a signed entry is "taken back"; an order to the staff is "withdrawn"; a draft is "torn up".
23. **What a player must know before signing is said as a fact of that order** and never as a rule.

### Keys and reading aloud

24. **One Escape, the topmost thing first:** a place being picked, a draft (asked where something is chosen), a paper, a card, a menu, the fold.
25. **At a stop the focus goes to the first thing that waits,** or to WHY, and is never taken from a draft. A paper that closes gives the focus back.
26. **SINCE says a count while the clock runs,** not each line; the clock's state is said once at each change; a card is not read out whole at each open.
27. **An order on a hex can be begun from the keyboard** through any link to a place. A course's points stay the pointer's.

### How it is built, restated

Five stages, each its own pull request, the table playable after each; none begins before the table batch (decisions 0215 to 0219) is merged. The file is not split.

0. **One way to say an outcome:** every place that writes the status line calls one function with where it belongs, and the line stays for now.
1. **The stop paper, the clock and the chart's controls:** the column with its head, WHY, FOR YOUR DECISION, SINCE and foot; cards and the order forms at the right; the chart's controls at its foot; the tray, the reports panel and the bar go, the outcomes' line moving to the column's foot; the widths. "Give an order" stays in the foot and the Orders menu stays.
2. **The two tools, YOUR ORDERS and SIGNED,** with the leave's line; the Orders menu and the plan chip go; outcomes go to where they belong and the line goes.
3. **One order paper;** a force's card gains the orders on its waters; "Give an order" goes.
4. **The fold of stops, the words, the captions, the keys.**

The screen's checks keep their literal checks beside the code they check, list every call the page makes at load, and gain checks of each part of the stop paper.

## As built: stage 0

Every place that wrote the status line now calls `say(kind, words)` in `table/index.html`, and nothing else writes it. The line looks and reads as before. The kind says where the outcome belongs, for the stages that move it. The kinds are:

- `refused`: an order or command refused or not sent.
- `signed`: what signing or an answer gave.
- `draft`: a draft begun or torn up.
- `stopped`: why the clock stopped, or that the game is over.
- `running`: the clock running and what it is doing.
- `picking`: a prompt while a place is picked on the chart.
- `confirm`: the second touch that New game and Restart ask for.
- `hint`: the first look's hint.
- `failed`: a paper that could not be read.

The status line carries the kind as `data-kind`. `table/check_screen.py` checks that the old writer is gone and that every call names a known kind.

## As built: stage 1

The stop paper, the clock and the chart's controls are built in `table/index.html`, as the amended section's rules 1 to 9 say.

- **The column** is fixed at the left: 360 wide under 1440 and 400 from there. Its head holds the name, the seat, the date and hour, and a state line: the day of how many, and THE CLOCK IS STOPPED, THE CLOCK RUNS, WAITING FOR THE … COMMANDER, or THE GAME IS OVER. Below the head come, in order:
  - the seat slips and the switch of sides of a watched game, and the other commander's slip in a game against an agent;
  - WHY THE CLOCK STOPPED;
  - FOR YOUR DECISION · n;
  - SINCE.

  The foot holds the outcomes' line, Give an order, the step, and Carry on. The step is "Next pulse", or "Next day" where the game is played by days; it was "One day". The head and the foot do not scroll.
- **WHY** has one slip for each kind of reason: what the table's ticks were told, with at most three lines; each fired interrupt, with WHY and DO NOT STOP ME FOR THIS; reinforcements; the staff asking leave; a plan that waits for leave; and the game's end, with a link to how it ended. "And n more" opens the rest as a paper for reading, and so does an interrupt's WHY. WHY is empty, and so not shown, when the stop is the player's own. It is not kept across a reload.
- **FOR YOUR DECISION** holds a force asking leave, a plan that waits, and a directive unread. It is left out when nothing waits, and in a watched game.
- **SINCE** is kept as data. It is cleared when the clock starts from a stop, and its heading says the count of the reports: "SINCE 1800, 3 JUN · 38 REPORTS". It shows the three latest reports under their pulse's heading, and "and n more" opens all of them as a paper. A line that folds the rest of a kind ("and 5 more reports of bombing of a base") is counted for what it folds and is shown only on the paper. After a reload SINCE holds the day's reports, read once with `events`. On the first evening it is left out.
- **Cards and the order forms** (`#card`, `#order`, `#force`, `#course`) open at the right. A paper for reading is centred over the chart's part of the window, and wins over a card as before.
- **The chart's controls** sit at the foot of the chart on the right: PLANS, PIECES · ALL/OURS/ENEMY/OFF, NAMES · POINTER/ALL/OFF, HEXES, TABLE | FROM ABOVE, and + and −. Each does what it did. At the top right are the Orders menu (until stage 2), STAFF and GAME. The in-tray of directives became "Directives · n" in STAFF, and the agent's seat is under GAME.
- **The chart turns to the middle of what is not covered.** The camera's view is offset, by `setViewOffset`, to the middle of the window less the column and less a card or form at the right, and the offset eases as panels open and close.
- **Under 1100 wide** the column is a drawer. Its head stays at the top left and opens it, Escape closes it, and the chart is then centred in the whole window.
- **At the game's end** the foot has READ HOW IT ENDED, and New game and Restart stay under GAME. **Against an agent**, while the other commander is waited for, Give an order is disabled and the foot's line says so.
- **Gone:** the signals tray, the reports panel `#plot`, the bar at the foot and `--bar-h`, and the in-tray button.
- `table/check_screen.py` checks the column's parts and their order, the foot's controls, which parts do not scroll, the widths and the drawer, that the old panels are gone, the chart's controls and the menus, and that cards open at the right. `PAGE_LOAD_CALLS` gains the one call made at load for SINCE (`events`).

In the same pull request, the plan's paper was mended on the designer's word:

- A plan's APPROVE and CANCEL are given once, at the topmost task on that plan.
- What the staff wants to know before a landing is said once, on the highest task that says it.
- The held count names where it came from ("as it was told", "seen from the air", "from men in contact"). The page had read a field the tool does not send, which left an empty part between two commas.
- A task's name says a place by its base's name, or by its position with the hex in brackets, as a card does, linked.

## The architect's rulings on stages 0 and 1, Nov 22, 2026

Read on the team's report, its screenshots and the page's checks before merging.

- **WHY gains "A plan waits for your leave." and "The staff asks your leave."** where that is why the clock stopped. Right: an empty WHY at a stop is worse than a line that FOR YOUR DECISION repeats.
- **WHY is not kept across a reload.** It is kept with the game in stage 4, with the table's ticks.
- **SINCE is left out on the first evening,** and the column is then nearly empty until stage 2 gives it YOUR ORDERS and "No order in force."
- **Cards and forms stop above the chart's controls,** and the chart's turning does not allow for a paper for reading. Both stand.
- **The step control reads NEXT DAY** where the game is played by days. It read "One day"; the plan's word is kept.
- **The plan paper's tally still counts a waiting plan once for each task under it** ("2 wait for you" for one plan). It is to count plans, and is mended in stage 2 with the plan's line under YOUR ORDERS.
- **Not tried by the team and owed before stage 4 closes:** a force asking leave to pursue in the column; an interrupt's reasons opened as a paper; the keyboard alone.

## As built: stage 2

**The two tools** are the session's, in `crates/sim-agent/src/queued.rs` and `session.rs`; nothing in `sim-core` changes. Both are tools of every seat, listed after `end_day`, and neither is the watcher's.

- `queued` lists what the seat has queued: its orders for the next pulse, its orders for dusk and its commands, in that order. Each entry carries its place (`queue` and `number`), the unit and the order or the tool and the command in the side's numbers, its words, when it acts (`acts`, `acts_words`), `goes_with` and `rests_on`. The answer carries an `as_of` token that changes whenever the queues change or the clock moves.
- `take_back {queue, number, as_of}` withdraws one entry and what goes with it: a hold goes with the last queued order to its unit, and the commands that name a new intent go with the `set_intent` that makes it. The tool takes them out, judges what is left again by the engine's own checks, and is refused, changing nothing, where an entry that passes now would fail without them; the refusal names that entry and its place. A stale `as_of`, a place with nothing in it and a game that is over are refused too.
- The agent interface is 46. Fifteen tests are in `crates/sim-agent/tests/queued_take_back.rs`. Chapter 8's list of tools, chapter 1, `docs/commands.md` and the glossary say the tools. The recorded transcripts are remade, changing in their version alone; the records of the closed proofs are not touched.
- While the other commander is waited for, `queued` is refused like any call of a seat that is ready, so SIGNED is not read then; nothing can be signed then either. Ruled acceptable.

**The table** (`table/index.html`):

- **A force's card offers the orders on its waters**, below its own controls. "Give an order" and its cue are gone. Nothing was reachable only through it: a touch on open water opens the hex's card with its orders.
- **YOUR ORDERS** lists each order in force as a link to its plan, with a note where its plan waits. The plan chip's close is now TAKE OFF on that plan's line, while the plan is on the chart. The line "Plans launch: when ready" or "on my word" carries its switch. "No order in force." stands where there is none. The Orders menu and the plan chip are gone; their papers are under STAFF or in the column.
- **SIGNED is read from `queued`** at load and at every refresh, so it is the same after a reload.
  - Each entry is a line in the table's words, the unit by name and linked, with "Acts at 0600", "Acts at 1200", "Acts at 1800" or "Waits for 1800".
  - TAKE BACK is offered while the clock is stopped and the seat may sign. A refused one says NOT TAKEN BACK and why, beside the entry.
  - An entry that rests on another, or goes with it, is not a line of its own: a hold goes with its order, and a limit with the order to the staff it was signed with.
  - The commands of a plan's paper and of a card are listed as orders are.
- **Each outcome is said where it belongs,** through `say(kind, words, home)`, and the outcomes' line is gone:
  - `refused` is NOT SENT in what was signed.
  - `signed` is the new entry under SIGNED, marked JUST SIGNED.
  - `stopped` is WHY.
  - `picking` and `hint` are a prompt above the chart's controls.
  - `confirm` is in the GAME menu.
  - `running` is the head's line.
  - `failed` is NOT READ on the paper that failed.
  - `draft` is said to a reader of the screen only.
- **The plan paper's tally counts plans**, not tasks.
- **The drawer, under 1100 wide,** is open at load and at every stop. The player may close it, and it stays so until the next stop; it closes when the clock starts. Its head shows a chevron pointing down when closed and up when open, labelled for a reader, and when closed the count of what waits ("1 FOR YOUR DECISION"). With the drawer closed, the two controls that run time hang under the head as a strip: the same elements, moved by CSS.

**Mends on the designer's word, in the same pull request:**

- **The stops speak as the staff.** "A contact is reported" became "We have a contact", "You are attacked" "We are attacked", "Your strike is away" "Our strike is away", and "The staff asks your leave." "We ask your leave." Where the slip's events say which it is, the heading for a loss or for men ashore names it ("We have lost USS Quincy", "Our men are fighting at Lunga"); the general heading is kept where several kinds are in one slip. The game's end stays in the umpire's voice.
- **Aircraft are named short in those lines.** A type is said as the cards say it: the maker left off, and a Japanese type's long name cut after its designation ("G4M1 aircraft").
- **A contact card's HOW WE KNOW** gives the date and pulse on a line of their own and the words beneath, from the events' fields: units by name and linked, no handle, the place as latitude and longitude with the hex in brackets, the kind in the plural, in the staff's voice ("VS-5 sighted 14 ships, taken for carriers, at …").
- **The card's trail** never holds one thing twice running, and BACK never leads to the card it is on. That holds when the card is opened again after a form has closed it. A link in the stop paper starts a new trail. Stepping through a stack does not grow the trail.

`table/check_screen.py` checks each part: YOUR ORDERS and its empty fact, the launch line, SIGNED from `queued` with TAKE BACK, the outcomes' homes, that the removed parts are gone, the drawer, the chevron and the strip, the trail, and that no card text shows a handle. `PAGE_LOAD_CALLS` gains `queued`.

Left for later stages: an air group's mission and its arc are two entries under SIGNED; SINCE still names an aircraft type in full and gives a position without its hex; the prompt while a place is picked covers the zoom-in control in a narrow window.

## The architect's rulings on stage 2, Nov 22, 2026

Read on the session's code for `queued` and `take_back`, the team's report and its screenshots before merging.

- **A take back is tried and judged again:** the entry and what goes with it are taken out, what is left is judged as it was when queued, and where an entry that passed now fails the queues are put back and the refusal names it. That is the right way: no second set of rules says what rests on what.
- **The two tools are not a watcher's,** and a seat whose call of time is held cannot read what it signed until the other commander has played. Accepted.
- **The drawer closes when the clock starts,** the strip with STOP THE CLOCK staying in sight. Accepted.
- **A slip's heading says what its events say,** and the general heading stays only for a slip of several kinds. The headings for a loss and for a landing were proved on sample events and not seen in play; they are owed a look when a game gives one.
- **Not tried, and owed before stage 4 closes:** a refusal by the engine shown as NOT SENT (the forms offer only what it takes; force one in the check); an air attack's row under HOW WE KNOW.
- **For stage 3:** an air group's mission and its arc are one entry under SIGNED, not two. **For stage 4:** SINCE says an aircraft type short and a position with its hex, as the slips now do; in a narrow window the prompt while a place is picked does not cover a control.
- **One test failed once under the full parallel run and passes alone** (scenario 120, statement 6, which runs the derivation command inside itself). It is not of this change; it is a row of open work to stop it running a build inside a test run.

## As built: the plan's paper as one tree

On the designer's word, after looking at the paper of a plan that waits ("could portray a hierarchy of orders"), the plan's paper says its plan once, as one tree. It was built in its own pull request, between stages 2 and 3.

- **The order is at the root, with its tasks under it** as the staff would form them, each under its parent, with rules down the left. A plan that waits is built from `propose`'s `tree.sub_intents`; a plan in force from `command_state`'s intents under the root. Tasks carry no order number; the root keeps the player's.
- **Under each task are the units it would order,** by name and linked, each in one line of what it is ordered. They come from `would_hand_in` for a plan that waits and from `staff_orders` for a plan in force. A task that orders nothing says the one fact that matters.
- **Each task has one fold, WHY**, holding short lists of facts: WEIGHED, CHOSE, HOLDS IT BACK and WOULD CHANGE WITH, read from its reason's fields. A search arc is said as a bearing range and drawn on the chart while the fold is open or the task is under the pointer. Cautions stay in the warning ink under their task.
- **APPROVE and CANCEL are given once a plan,** on its line at the top. The full working is the last fold, closed, and nothing is at the foot: the caption on how a plan is changed is gone.
- **The paper says things in the table's words,** without the engine's phrases:
  - no "hex(es)" and no "nearest hex";
  - fatigue as the cards say it;
  - shares said one way;
  - positions with the hex in brackets;
  - "ours" for "the side's".

Two facts are read from the staff's words, because no field gives them:

- **The threats a search arc was set for** are the `base:N` and `contact:N` handles in the arc's working, found and linked. A list of threats on `propose`'s `arcs` would end this, but it would change the staff's reason and the doctrine's label.
- **The fact of a task that orders nothing** is the sentence of `tree.no_orders` that names it first.

`table/check_screen.py` checks:

- one APPROVE a plan, at the top;
- the tree and its rules;
- the WHY fold;
- the last fold;
- no caption at the foot;
- the paper's words, by running a real proposal of the 39 days through the page's own wording.

## The architect's rulings on the plan's paper, Nov 22, 2026

- **The threats of a search arc, and the one fact of a task that orders nothing, are read from the staff's words for now.** This is a row of open work. When the staff's doctrine is next changed, `propose`'s arcs gain a list of threats as a field, and each sub-intent gains its fact for ordering nothing. The staff is not changed for it now.
- **In a WHY list a place said once is not said in full again in the same list.** After its first mention with latitude, longitude and hex, later mentions in that task's WHY say "there", or the place's short name where that reads plainly. This is done in stage 4, with the words.

## As built: stage 3

One order paper, `#paper`, gives every order. It replaces the operation order (`#order`), the order to a force (`#force`) with its course (`#course`), and the air group's and land unit's forms inside their cards. It opens at the right in the card's place. When it was opened from a card it carries "← BACK TO" that card. Every direct-order button and every "orders you can give" opens it.

- **Its parts, in order:**
  - **TO**, "The staff" or the unit by name, linked.
  - **THE ORDER**, the kind's own controls, every one kept. A course's points are in the paper, with ADD ON THE CHART and DONE ON THE CHART.
  - **IN WHOSE HANDS**, for a unit: "The staff's now. Yours once signed: you keep it out of the staff's plans."
  - **ACTS**: "Acts at once, at 1800" (or 0600 or 1200), or "Acts at 1800, for the coming day". A draft that mixes the two says which part acts when.
  - **The staff's reading**, open, with DRY RUN where the game has a trial.
  - **SIGN AND SEND and TEAR UP.**
- **The paper stays open while the chart is touched**, and a touch that picks nothing opens no card. The chart draws what is drafted while the paper is open: the course, the search arc, the march's places and the deny circle. Before, an air group's arc was drawn only while its card was open.
- **Escape or BACK on a draft:**
  - With nothing chosen, it tears the draft up.
  - With something chosen, the paper asks "Tear up this order?" with TEAR UP and KEEP WRITING, and a second Escape keeps writing.
  - A link to another card asks the same.
- **NOT SENT says, line by line, what went and what did not**, one line for each order of the call ("Speed: sent." "Course: NOT SENT — the reason"). On a second try what went is not sent again. An order to the staff signs in steps (the order, its risk, its forces), and a step that did not run says so.
- **A refused unit order leaves nothing queued.** Where nothing of the order went and the page had sent a hold first, the page reads `queued` and takes that hold back with `take_back`. Where part of it went, the hold stays.
- **An air group's mission and its arc, signed together, are one entry under SIGNED** (the ruling on stage 2). Its TAKE BACK takes back both.
- **Gone:** the three forms, their captions ("BEFORE YOU SIGN", "It is 1800. The order goes with tonight's orders.", the land unit's "takes it out of the staff's hands"), and the functions that served them.

`table/check_screen.py` checks the paper's parts in order, ACTS, IN WHOSE HANDS, the way back, Escape, the draft drawn while the paper is open, NOT SENT line by line, and the mission and arc as one entry. It also takes a hold back against the engine after a refused order.

Left for stage 4:
- ACTS says "Acts at once, at 1800" where SIGNED says "Acts at 1800": to be made one wording.
- Escape on an empty draft closes the card it came from too, rather than going back to it.
- The order to the staff's descriptions under each end are old wording.
- Once, SIGNED was not read again after a signing when the bridge reset the page's connections; the next refresh showed it. The page says nothing when a refresh fails.

## The architect's rulings on stage 3, Nov 22, 2026

- **The paper at the right, with its parts as built, is accepted.**
- **A refusal forced by a malformed order stands as the proof of NOT SENT** until the engine refuses something the paper offers.
- **Stage 4 takes in five more things:**
  - The prompt while a place is picked lies over neither the paper nor a control.
  - ACTS and SIGNED say one wording.
  - Escape on an empty draft goes back to its card.
  - The descriptions of each end are reworded as facts or dropped.
  - A refresh that fails after a signing is said, never left silently stale.

## As built: stage 4

**What stops the clock** (amended rules 19 to 21):

- **The fold WHAT STOPS THE CLOCK** is closed by default and sits in the stop paper's foot, above the controls that run time; it scrolls inside itself.
- **What it holds.**
  - The table's ticks: a contact, an attack on mine, a ship lost, my strike away, men ashore, reinforcements arriving and each evening. The first three are ticked at the start, and "each evening" is off.
  - Under THE STAFF STOPS IT WHEN, the six standing interrupts, read from `command_state` and set or cleared with `set_interrupt`. A tick changed inside a day shows "from 1800".
- **What went.** The old panel of what stops the clock and its button under GAME are gone, so a standing interrupt switched off by a slip's DO NOT STOP ME FOR THIS can be ticked back on here.
- **"Each evening".** CARRY ON stops at dusk when it is ticked, and WHY says "It is evening."
- **What is kept in the browser.**
  - The table's ticks are kept for the game, under the same key as the directives, and cleared by New game and Restart.
  - WHY THE CLOCK STOPPED is kept the same way, a force's asking included, and dropped once the clock moves.
  - A page that cannot keep them plays as before.

**Keys and reading aloud** (amended rules 24 to 27):

- **One listener answers Escape**, topmost first: a place being picked, a paper for reading, a draft, a card, a menu, the fold, the drawer. A paper for reading comes before a draft because it lies over one. An empty draft, or one torn up, goes back to the card it came from.
- **Focus.** At a stop the focus goes to the first thing that waits, or to WHY, and is never taken from a draft or a paper. A card, paper or draft that closes gives the focus back. Ctrl+Enter signs the order paper.
- **What is said aloud.** The clock's state is said once at each change. While the clock runs, SINCE says a count of reports. A card is not one live region, and its title is its label.
- **The keyboard.** An order on a hex is begun through any link to a place.

**The words** (amended rules 22 and 23, plan rules 15 and 16):

- **A unit the player holds** is "Yours: you keep it out of the staff's plans." everywhere. "Release" became "Return to the staff".
- **ACTS and SIGNED say one wording:**
  - "Acts at 1800, with tonight's orders" at dusk.
  - "Acts at once" inside a day, for an order that acts at the next pulse.
  - "Acts at 1800, for the coming day" for one that waits.
- **Captions that explain a rule are gone or said as facts:**
  - the notes under each end of an order to the staff;
  - the dry run's "Sign it on the order, or tear it up" and its foot;
  - the leave line's "Giving it frees every plan…";
  - the reading's "your risk goes with the order", which is now "This reading is worked at balanced risk.";
  - the evening report's "A claim is what they believe";
  - the fuel and course note, now "A force fuels at sea or steams a course, not both.".
- **The situation is a paper of its parts,** each line in the table's words: units by name, places linked with the hex, no decimals. An interrupt's slip words go through the same.
- **SINCE, the war diary and the evening report** name an aircraft type short, and give a position with its hex, linked.
- **In a task's WHY in the plan's paper,** a place said once in full is "there" after.
- **The prompt while a place is picked** sits at the top centre of the chart's free part, under the head and its strip in a narrow window, over neither the paper nor a control.
- **A read after signing or taking back that fails** says "Could not read what is signed." with READ AGAIN, where SIGNED is.

**Tried, owed since stage 1:**

- A force asking leave to pursue, in scenario 125: WHY, and FOR YOUR DECISION with ALLOW and REFUSE, both kept across a reload.
- An interrupt's reasons opened as a paper, in scenario 40a and the 39 days.
- The keyboard alone.

**Checks.** `table/check_screen.py` checks each part. It also forces a refusal through the page's own wording: a real refused `issue_orders` answer from the bridge is run through the page's NOT SENT functions.

**Left open.**

- The situation still shows the scoring's lines and the note on estimates. Against "no points during play" this is the designer's to rule.
- "for the coming day" under SIGNED was not seen live.
- The per-order pursuit setting is not linked from the fold. The amended rules leave it out; the staff's standing tick for a force asking leave is in it.

## The architect's rulings on stage 4, Nov 22, 2026

- **The pursuit setting of each order stays out of the fold,** as the amended rules have it.
- **No points are shown during play,** by the designer's standing rule. The situation paper does not show the scoring's lines, the levels of victory or any point value while the game is in play. It says the mission and each objective in words, without their worth, and keeps what is known of progress in words. The points belong to HOW IT ENDED alone. The note under the estimates stays only if it says what the commander knows, not how a rule works.

### As built

In its own pull request, "The situation shows no points during play", the page shows the situation through `situationLine`.

- **While the game is in play, the OBJECTIVES part is cut back:**
  - an objective's worth is cut ("Objective: Lunga is your own side's at the end of the game");
  - the line of levels is left out;
  - "worth N points" is cut from what is known of progress;
  - "The enemy is scored for:" becomes "What the enemy is after:", with each line's worth cut.
- **The note under the estimates is left out:** "These are estimates from reports, not counts: each can be wrong by up to half either way …" says how a rule works.
- **Once the game is over,** the objectives are shown whole.

The evening report, the directives and the plan's paper were read on Midway and the 39 days in play, and none says "points". `table/check_screen.py` puts the real situation of the 39 days through the page's own functions in node and checks two things: no line shown in play says "points" or the levels, and each objective and what the enemy is after are still said. It also checks that the page writes the word "points" only in HOW IT ENDED.

## The staff's reading in the order paper is the plan's tree, Nov 23, 2026

On the designer's words about an order's THE STAFF'S READING on the Coral Sea ("this text is still not very comprehensible … what should the player do with this?"), the architect ruled that the staff's reading in the order paper is the same tree as the plan's paper, built from the same structure and by the same functions, in a short form. The designer then added that a reason which says a unit is not free because of another order must link to that order, so that the player can withdraw it.

### As built

- **The engine.** `preview` had no tree: its `with_change` gave the staff's orders and decisions for the coming day only, and the tasks were told in one paragraph. It now carries the proposal's `tree`, `would_hand_in` and `reason`, as `propose` gives them for an intent in force. This is agent interface 49, and nothing in `sim-core` changes. `propose`'s schema now admits a `keep_supplied` task with its days, which a hold under a control of an island forms.
- **The reading.** It is drawn from that answer by the plan's paper's own `proposedTree` and `paperNode`:
  - **One line at the head** says what signing would do ("Task Force 17 would work these waters from 13°44′S 148°45′E (-18, 3)."; "The staff would take it up in these tasks:"). Where nothing would serve the order, it reads "Nothing can serve this order yet." in the warning ink.
  - **The tree** follows, each task with its units one line each and a closed WHY fold.
  - **What cannot be done** is said once, as a fact under the task it belongs to ("No air base can reach these waters: Port Moresby, in these waters, has no bomber or search aircraft the staff may use."), with WOULD CHANGE WITH ("a bomber or search group at a base within reach").
  - **What is left out:** counts of orders and commands, the numbers of tasks, and "It cannot yet". The draft is "this order".
  - **The staff's working in full** is the last fold, closed. DRY RUN stays.
- **The words, shared.** The shared wording drops the count of hexes on the map, says "hexes" for "hex(es)", "lies in" for "0 nm from the nearest hex of" and "N miles from" for the rest. The plan's paper, WHY on a slip, an interrupt's reasons and the dry run gain the same.
- **A unit that is not free.** Where a reason says a unit is not free because of another order or task, it says the unit once and names that order as the player knows it ("Task Force 17 is not free: it serves your order to operate Task Force 17 from …"). The order's words are a link to its paper, opened at the task, where WITHDRAW THIS ORDER is. WOULD CHANGE WITH gains "that order withdrawn". The order is found from the `intent:N` in the engine's words and the parent chain of `command_state`, not from the quoted words.
- **Risk.** The reading is still worked at balanced risk: `preview` takes one command, `set_intent` has no risk, and a risk limit needs an order in force. "This reading is worked at balanced risk." is said only where the risk chosen is not balanced. Working the reading at the chosen risk needs `preview` to take a risk with a `set_intent`, an engine change left open.
- **Left open:**
  - A hold made by an order is named without the order. Linking it would need the order's number in the hold's words (`hold_words` prints the maker without it).
  - The plan's paper's in-force "cannot be met" line still shows the staff's sentence.
  - Some diary and evening-report lines that do not pass through the shared wording were not traced.
