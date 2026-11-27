# Design note: levels

Status: a note for discussion, written by the architect on Nov 26, 2026 at the designer's asking, after he read decision 0232. But for its first three rungs, which decision 0237 plans, nothing here is built or planned, and the note takes no decision number. Amended on Nov 26 to the designer's answers, and reworked on Nov 27, 2026 to his change of how a level is measured ("The designer's answers", at the end, which says what replaced what): a plan is measured once, in the game he fights; scoring it against the staff is his choice, by the dry run before and a debrief after; failing a level does not block the next, not trying does; a level may be replayed on a new situation, and the last game counts. Amended again on Nov 27, 2026 to four further answers ("The first three rungs", at the end): the ladder begins with three plain scenarios on the table's list, judged by the game played alone, which decision 0237 plans; nothing else here is built or planned.

The designer's words: "The game is about planning. The player will be taken through levels. Each level they will get to learn some new planning skills and practice previous skills. So scenarios will become more challenging (more units, longer scenarios). So the player will set a plan, maybe dry run it, amend, see how it plays, and so on. It could be that the player will only get command for some of the units and will need to fit a plan for a certain date."

## The idea in one line

A level is a small war with a mission given from above, in which the player writes a plan, may try it in a dry run, and fights it once. The war is the measure: the mission is met by its date or it is not. If he chooses, he is then shown how his plan and the staff's own fare over many games, so that he can tell a bad plan from bad luck. He may fight the level again on a new situation with a changed plan.

It continues decision 0232 and does not replace it. Decision 0232 lets a commander write part of a plan and set it beside the staff's in a dry run. A level adds three things: a mission he did not choose, one game that counts, and a reason to come back for the next.

## What exists to build on

Checked against the rulebook and the records on Nov 26, 2026.

| Needed | Exists | Standing |
| --- | --- | --- |
| A scenario scored on a scale of its own, made as a contest | Decision 0187: `for_play = true` in `[objectives]`, under three guards (the reason before the number; read on a band; agents play it before it is called balanced) | Built. Five games for play are on the table's list: the Eastern Solomons in one day and in five, the Coral Sea, Midway, the 39 days |
| A mission in a superior's words | The table's directives (decision 0157; `table/directives/`): a situation and a mission with its purpose, written for the game | Built, as text the player reads. Nothing in the engine holds him to it |
| A plan, and what of it is his | Intents, limits, the tree, the maker of each node; decision 0232 for a node he writes | Built, but for 0232, which is planned |
| A plan read back before it goes | The plan order (decisions 0159 and 0164); the staff's reading in a preview | Built |
| A dry run, and a draft beside another | The trial and its what-ifs (decisions 0175, 0178, 0181, 0204); 0232's step 2 for a written part beside the staff's own | Built, but for 0232's step 2. Offered only at a seat that plays a whole side, in a game whose file names the trial's figures |
| A game played over many seeds | `sim-cli batch`, and the staff's batches every proof has used, 1,000 seeds at a time | Built, as the umpire's tool |
| Command of part of a side | Command levels and a seat at any command (decisions 0132 to 0138) | Built for a model at a seat. No trial and no `run` at such a seat; `docs/step3-spec.md` keeps a choice of seats out of step 3 |
| An opponent easier or harder | Decision 0189 | Planned, not built |
| The way in for a newcomer | `docs/design/onboarding.md` | A note |

**How this note stands to `onboarding.md`:** it contains that note's ladder and changes what the rungs are. Onboarding's rungs each add a part of the game (a day, a contact, a convoy, a campaign). A level's rung adds a decision of planning. If the designer takes this direction, the ladder table of `onboarding.md` gives way to the one below; the rest of that note stands beside this one and is not touched by it: teaching through the role, the staff explaining itself, the first five minutes, watching a newcomer.

**How it stands to `planning.md`:** that note says how a staff estimates a plan, and its answer 3 says planning is judged at the kill gate "in a small version": one operation, its requirements, one window, a trial and two courses. The first level below is very nearly that small version, with a judgement added.

## What a level is, as an object

A level is a scenario for play and five things beside it. The scenario file today carries the first of the five in part and none of the others.

1. **The mission from above, with its date.** Today a directive is words on the table, and the player states his own order. In a level the order is there when he sits down: a root intent the scenario states for his side on day 1, with its date and its limits, which he may not withdraw, replace or widen. He writes beneath it and narrows inside it. **New:** an intent marked as given from above, which the engine holds him to. It is state, and so a decision record's to plan.
2. **What is his.** In the first levels, everything of his side in the file. Later, a part: the rest belongs to a neighbour or a superior, and is on the chart and in the plan but not his to order, hold or limit. **New, and the largest piece:** see "Part of a side", below.
3. **What he may write, and the part that is his to write.** Which kinds of node are open under the mission (decision 0232's table of pairs), and which kinds of command are the level's taught part, of which a plan of his own has at least one ("What counts as trying"). The engine has one table for every game; a level would narrow it. **New, small:** a scenario's list of the pairs it offers and of its taught part.
4. **The staff's own plan, to set his beside.** The same mission and limits with nothing written, played over many seeds, for the debrief if he asks for it. It gates nothing. **New:** the tool that makes it.
5. **What it teaches.** One line, for the level's screen and for whoever builds the next level: the decision this level is about. It is words and no rule reads it.

A level's file is a scenario for play in every other respect and passes decision 0187's guards as one. It is never a proof's or a validation measure's scenario, and its results are never laid beside the record of a battle.

## How a plan is judged

### The manager's seven suggestions, each judged

1. **The staff's own result is par, and a blank node earns par.** Kept as a comparison he may ask for, and no longer a mark that gates anything (the change of Nov 27). The staff can still do every task of a level; what it cannot do for him is count as his try.
2. **A plan is graded across seeds, and the game he watches is one of them.** Kept. One game is a draw from a spread; a good plan loses some games and a bad one wins some.
3. **The dry run stays fallible.** Kept, and it is so already: a trial is played in worlds built from the side's picture, never from the live enemy. It needs one thing more to matter, below: the truth must be able to differ from the picture in a way the player cannot learn by playing the level once.
4. **The scoring scale must be real.** Changed. The points of the record's scale are placeholders and stay the architect's to set from the record before stage 5; a level does not need them. A level is judged in the words the trial already uses: on how many games the mission was met by its date, and what it cost. No points are added up.
5. **Each level opens one more kind of node.** Changed. A skill of planning is not always a new kind of node. The first skills are taught with what a commander can already do: give an order, name forces, rank two orders, set the risk, say what must be known, tell a plan to wait, try it at two dates. Only the later rungs need a pair that decision 0232 leaves for later. Tying every rung to a new pair would make the ladder exactly as long as the doctrines that have been read for a written node, and the supply run, the escort and the search of the first guess are not nodes a commander writes at all: a supply order is a root today, an escort is the staff's, and "search waters" is refused to a commander.
6. **One level is built as the test before any ladder is designed.** Kept. It is planned below.
7. **The pillars would need the designer's amendment.** Kept in part. See "Against the pillars".

### Measured once, in the war

The designer's change of Nov 27, 2026: "the player may choose to score their plan against the staff, but like irl, their plan is measured once in the war itself. Once. So, yes, they may be lucky or not. The goal is to learn and improve, feeling more and more accomplished. Failure will not block them from proceeding to the next step. Not trying will. If they choose to continue after failing - fair enough."

It replaces the two-part score, the pass by beating par, the game's part that could fail a level and the count of tries (the answers of Nov 26 numbered 2, 3, 5, 7 and 11 at the end). What a level now is:

- **He signs a plan and fights one game of it,** steering as a commander does: he answers what stops the clock and may amend. The level draws the situation (a variant of the truth) and the seed after he signs.
- **Success is the mission met by its date in that game. Failure is the mission not met.** That is all the two words mean. For an island: every place the side's at the end of the named day. He is shown that, what it cost him (the five costs a trial gives), and the account of the game, plan against what happened.
- **Neither gates anything.** A failed level opens the next as a met one does. What opens the next level is having tried this one (below).
- **Luck is in it and is meant to be.** One game cannot tell a good plan that met bad dice from a poor plan that met good ones. The game does not pretend otherwise; the debrief is where he can find out.

### Scoring against the staff: his choice, before and after

"The player may choose to score their plan against the staff." There are two times he can, and they are different instruments.

- **Before the game: the dry run.** The trial with `compare` (decisions 0178 and 0181), and for a written part decision 0232's step 2: his draft and the staff's own plan in the same worlds with the same costs. It is played in worlds made from his side's picture and never reads the truth, so it can be wrong, and it is his to ask or not. Nothing new is built for it.
- **After the game: the debrief.** His signed plan and the staff's own plan for the same mission, each played cold over many seeds from the true start of the situation he fought, and set side by side: on how many games each met the mission by the date, what each cost, and where his one game fell among the games of his own plan. This reads the truth, so it is the umpire's and is offered only when his game is over. It is where the many seeds earn their place.
- **The debrief says what the numbers can bear.** On paired games, let *a* be the games his plan meets the mission and the staff's does not, and *b* the other way round. Where *a* − *b* is clear of noise (its square at least 4 × (*a* + *b*)), it says his plan was the better, or the worse; otherwise it says the two cannot be told apart on a thousand games. It is a reading, in words, and gates nothing.
- **It is of the situation he fought, and no other.** The level's other situations are not played for him until he has fought them: a count over all of them would tell him what the enemy may hold next time.
- **"Cold"** means a plan played with nobody at the seat: leave to prepare and to launch given, as a trial gives its course the leaves it lacks; every other setting as signed; nothing answered, so that where the staff waits for the commander it waits to the end. His own game is not cold, which is why it may fall well above or below the middle of his plan's games; the debrief shows that and makes no score of it.

Checked against the source, and still what the debrief rests on: a draw is keyed on the seed, the day, the pulse, the subsystem and the entity (`rng.rs`, `RngKey::stream`), and the staff draws nothing. Two plans played on one seed share their dice where the same unit does the same thing on the same day and pulse, and part where the plans part. Pairing by seed takes out the luck the two share. That is sound over a thousand pairs and says nothing sure of one.

### What counts as trying

Ruled by the designer, Nov 27, 2026: "Signing a plan of your own." Leaving it all to the staff does not count. He chose it knowing it makes writing mandatory inside a level.

As a check a program can make:

- **Each level names its taught part:** the kinds of command the level is about. For a level whose skill is a written node (rung 7 and after), a sub-intent of an open pair under the mission, with the commander as its maker. For a level whose skill is a limit or a wait (rungs 2 to 5), the kinds of limit it teaches: the forces or the source, a rank, the risk or the knowledge asked, a wait. For rung 1, the leave he gives. For rung 6, any of these, since its skill is trying and amending, which a program cannot see in a plan.
- **A plan is his own where, among the commands he signs before the first day, at least one is of the level's taught part,** on the mission or beneath it. The paper will not sign without one and says which part is his to write.
- **He has tried the level when he has signed such a plan and fought its game to the end.** That opens the next level.
- **What the check does not ask:** that his choice differs from the staff's, or that it is any good. A player who sets the one limit to what the staff would have chosen has tried. The check holds the door; the war does the measuring.

### The record, and playing again

Ruled by the designer, Nov 27, 2026: "Replay on a new situation, player can amend their plan, record keep history, and the last game played counts. That encourages replayability, learning, exploration and curiosity."

- **The ladder keeps a history for each level:** every game fought, in order, each with the plan as signed, the situation and seed it drew, whether the mission was met, what it cost, and whether a debrief was asked. **The last game is the level's standing;** the earlier ones are there to be read.
- **A new situation is another variant of the truth:** the enemy's start changed inside what the side's opening picture allows, so that the briefing and the estimates he reads are the same and what is behind them is not. Each is a file of its own, made with the level.
- **A replay draws a situation he has not fought,** with a fresh seed, and he signs again: the same plan or an amended one.
- **How many.** A level has at least three, so that the first game and two replays are each new. Settled by the architect as the least that makes "amend and try again" mean something; a level may have more.
- **When he has fought them all,** a replay draws among them again with a fresh seed, and the history marks that game as on a situation already seen. It still counts as the last game: nothing is gated on it, and a record that says what it is cannot mislead.
- **"Once" is once for a situation:** a game is never played over, and a situation is not met again until every other has been.

### What the trial can and cannot do for this

- **It can** be asked as often as he likes, of a draft with a written part beside the staff's own plan (0232, step 2), at three sizes; it shows what the plan needs, when it could go and how often it came off in the picture's worlds.
- **It optimises against the picture.** The same question at the same hour is the same worlds, so a player can tune a plan until the dry run likes it. That is a plan fitted to what he believes. Whether that is a good plan is the level's lesson, and it is only a lesson if the truth can differ.
- **It cannot be the debrief.** It never reads the truth, and the debrief must.
- **It is not offered at a seat of a command,** only at a seat that plays a whole side. A level in which part of the side is not his cannot use it until that is planned.
- **A window of dates is refused for a written part** in 0232's step 2. "What if I land two days later" is asked by a "not before" on the node, compared.

### What cannot be done with what exists

- **Nothing holds a player to a mission.** He can withdraw the order.
- **Nothing checks that a plan is his own,** or keeps a level's history.
- **Nothing makes a debrief.** A batch is run by hand from a file. Nothing takes the commands a player signed, plays them cold beside the staff's plan and reports the two.
- **The debrief reads the truth,** so it is the umpire's and can be shown only when his game is over, as a game's result is today.
- **No game on file has variants of the truth.** The lettered files of a game for play differ in who plays each side.

## The skills, as a ladder

Each rung is a decision a commander makes. "Built" means a player can make it at the table today. The scenarios named are candidates on file, not a plan; a file of record is never used as a level, a copy for play is made as decision 0187 has it.

**Before this table come the first three rungs** of decision 0237 ("The first three rungs", at the end): one supply order, two supply orders with their ships, a landing and its supply. They teach rows 1 and 2 below and the risk of row 4, at a smaller size than those rows' candidates, and the level planned under "The first level" is the fourth rung. The rows are renumbered when that rung is planned, not here.

| Rung | The decision | What it needs open | Standing | A scenario that could carry it |
| --- | --- | --- | --- | --- |
| 1 | Give an order and read what the staff will do with it; give or withhold leave | A root order, the plan order, leaves | Built | The Eastern Solomons in one day, for play; or a placeholder of one convoy (scenario 36) |
| 2 | What to spend: which forces, from which base, what is kept back | Forces, source, reserve | Built (decision 0222) | Placeholder: two bases and one stock (scenarios 40b, 43) |
| 3 | Which of two orders comes first: plans in force side by side on the same ships and aircraft ("parallel plans", first sense) | Rank; what each order displaces | Built | The same placeholders; the Eastern Solomons in five days, for play, where a convoy and a carrier battle compete |
| 4 | How much to risk, and how much must be known first | Risk, knowledge | Built | Placeholder (scenarios 72, 82); the Coral Sea in five days, for play: when to turn a landing back |
| 5 | When: hold a plan to a day, to a force, to another plan, so that one plan follows another ("consecutive plans"); try it at two dates | Not before, wait for, after; the window | Built | Placeholder (scenarios 80, 81, 96) |
| 6 | Try, compare and amend before committing | The trial with `compare` | Built | Any of the above whose file names the trial's figures |
| 7 | Two landings on shared shipping: which first, with what, by a date | A take written under an order for an island | Decision 0232, steps 1 and 2: planned | Placeholder: scenario 137 grown (the first level, below). In history: the landings at Lunga and Tulagi of 7 August 1942, the first days of the 39 days, whose facts are on file |
| 8 | Where the carriers stand, and until when | A carrier task written under his own order; a day on which a task ends | New work: 0232 leaves the pair for later, and no limit ends a task on a day | The Eastern Solomons in five days; Midway. Facts on file |
| 9 | A surface force placed to meet the enemy by night | A stand task written under his own order | New work | Savo Island and Cape Esperance have facts and scenarios of record; a copy for play of either does not exist |
| 10 | Holding a place and keeping it fed under attack | A hold written under an island; its source | New work, and the hardest pair (0232, What it leaves out) | The 39 days |
| 11 | A campaign: several orders that draw on the same ships for weeks | Everything above | The orders are built as roots | The 39 days |
| 12 | An alternative held ready: a second plan written beforehand with the condition that switches to it ("parallel plans", second sense) | A branch with a trigger | New work, with a decision record of its own (below) | None. A carrier battle is its natural ground: the Eastern Solomons in five days, Midway |
| 13 | A plan for part of a side, fitted to a superior's and a neighbour's | A mission held from above; forces not his; a trial at such a seat | New work | Any battle with command levels on (the files of stage 2b's proof have them; whether a file for play does was not checked) |

**Rungs with no scenario:** 8 to 10 have none made for play that isolates the decision; 12 and 13 have none at all. **Rungs with no facts:** none of the thirteen needs a battle that is not researched. A ladder that goes on to "more units, longer" past the 39 days does: the battles of October and November 1942 have order-of-battle facts and no scenario, and nothing outside 1942 is on file (`docs/design/larger-scenarios.md`). An invented level is marked `placeholder` and named "Placeholder", as every made-up thing is.

Rungs 1 to 6 need no new rule of the game. They need levels made, the mission held, the check that a plan is his own, and the debrief.

## Plans that follow, plans side by side, and a plan held ready

The designer, Nov 26, 2026: "I think that users may also write consecutive and parallel plans." Consecutive was read back to him as one plan following another in time. Of parallel he chose both senses offered: several plans in force at once drawing on the same ships and aircraft, and alternatives held ready with the condition that triggers each.

| | What exists | What is new | Where on the ladder |
| --- | --- | --- | --- |
| **Consecutive:** one plan after another | "After", "not before" and "wait for", on an order to take a place or control an island, and by decision 0232 on a landing written under an order. A plan order shows what each waits for | The waits on any other kind of order (a supply, a denial, a carrier's task wait for nothing today); a day on which a task ends | Rung 5 with what exists; rung 7 for two landings; rung 8 needs the day a task ends |
| **Parallel, in force at once** | Several orders at once, ranked; the higher never short for the lower; what each displaces is reported; a dry run gives what the other plans lost | Nothing for the first levels. Ranks read as one batch in a preview are open work (decision 0222, step 4) | Rung 3, then rung 11 at the size of a campaign |
| **Parallel, held ready:** plan B and its trigger | Nothing. No command switches a plan on a condition. The nearest things: an interrupt, which is a condition tested each morning that stops the clock and does nothing else; a plan that is preparing and waits for leave to launch; the three waits, which release a plan on a day, a force or another plan's end | All of it | Rung 12 |

**The branch wants a decision record of its own.** This note does not plan its rule. What that record must settle:

- **What a trigger may read.** The side's own picture and nothing else: its contacts and their age and kind, its estimates, its own forces, stocks and losses, the day, and what its own plans have done. Never the truth: "if the carriers are north of the island" can only mean "if my picture holds a contact of carriers there, no older than so many pulses". A trigger is therefore wrong as often as a report is, and that is part of what the rung teaches.
- **When it is tested, and whether it fires once.** The interrupts' rule is to hand: each morning, on a change and not on a state. Inside a day is another question.
- **What switching does.** Which plan is withdrawn, held or left, and which is released; what becomes of ships at sea and of what was made ready, by the owners' rules the spec already has; whether plan B is prepared while it waits, and what that preparation takes from plan A.
- **Whether it reuses a mechanism.** A trigger as a fourth wait ("not before this is so"), with plan B a plan that is preparing, is one candidate; a new command is another. The first is the smaller.
- **How it is played cold, and in a dry run.** A branch is part of the signed plan, so a cold game follows it. That is its worth for a level: it is how a plan answers the battle without its author. A trial must play it too, in worlds made from the picture.
- **How the staff says it,** in the plan order and its reasons, and what the enemy's staff may do with the same means.
- **State:** a trigger is the commander's and is hashed and saved.

## Part of a side

"The player will only get command for some of the units." There are two ways, and they are not the same game.

- **A seat at a command** (decisions 0132 to 0138): he sits at a task force or a base, sees that command's picture, and is given intents by the level above. It exists for a model. He has no staff planning for an area, no trial and no `run`. It is a different altitude, the tactical one, and it is not where planning lives.
- **The area's seat with a neighbour:** he plans for the area, and some forces on the chart are a neighbour's or the superior's, promised to him for a time ("the carriers cover you until the 9th") or not at all. This is the commander's real constraint and it is a planning problem. Nothing builds it: the staff plans for every force of the side.

The second is what the designer's words describe. It is new work of the size of a stage and is not in the first level.

## The first level

Planned far enough that a decision record could follow. Every figure below is a shape, not a number: the numbers are the record's, with their reasons.

**Name:** Placeholder level 1: two landings.

**What it teaches:** when shipping cannot carry both landings at full strength, which goes first and with what.

**The forces.** Scenario 137's, which decision 0232 builds and proves by hand, grown in four ways:

- the enemy holds both places with men, and the side holds an estimate of each, seen from the air, so that each count could be a quarter to twice the truth by the rule as it is;
- the transports carry fewer men at once than both landings want at the mission's risk;
- one more transport force and one more unit may be used from a later day, which is before the mission's date by too little to wait for both landings;
- the truth has three situations or more (the enemy's men divided differently between the places, each inside what the estimates allow), each a file of its own. The player is not told which he has drawn.

**The mission and its date.** Given from above and held: control the island, both places, by the end of a named day, at balanced. The date is set so that the staff's own plan meets it on some games and not on most.

**What he commands.** Everything of his side in the file. Nothing is a neighbour's.

**What he may write, and must.** The two landings under the order, each with its forces, its "not before", "wait for" and "after", what must be known, and a risk no bolder than the mission's. The taught part is a landing written under the order: he writes at least one, and the staff plans whatever he leaves.

**What it is measured by.** His one game: both places the side's by the end of the named day, or not. The staff's own plan over 1,000 seeds of the situation he fought is what the debrief sets his beside, if he asks.

**Where a plan of his can differ from the staff's.** The staff forms both takes on the first morning in the island's order; the first chooses its men against its estimate and the second takes what is left (chapter 8, Controlling an island; Not built: "the order the places are taken in"). It does not weigh one place against the other, does not put the small place first to free its ships, and does not wait for the later force unless a take is short. Those are the three things a player can do. Each has a cost: the small place first delays the large one; everything on the large one leaves the small one for a second trip; waiting spends the date; looking first spends days and a group. The staff's plan may be the best of them on a given situation. That no longer closes anything: he is measured by the war, not against the staff.

**"A plan worth writing" against "filling a form".** Beating the staff is no longer a pass, so the conditions change. Checked by the architect with agents before the designer sits down, and written into the level's record before any run is read:

1. **The plan matters:** over the seeds of a situation, at least two plausible plans differ from each other in how often the mission is met by more than noise. If every plan does alike, the game he fights measures the dice and nothing else.
2. **The mission is neither given nor hopeless:** in each situation a sensible plan meets it on a fair share of games, well short of all. A level won nine times in ten whatever he writes teaches nothing, and one lost nine in ten sends him away.
3. **No plan wins on everything:** the plan that meets the date most often does not also lose the fewest men. If one does, the level is a puzzle with one answer.
4. **The situations differ in what they reward:** the plan that does best in one is not the best in every other. Otherwise a replay is the same level with new dice, and "a new situation" is a name.
5. **The dry run does not give the answer away:** in at least one situation the plan the dry run likes best is not the plan the truth likes best.
6. **The designer's own account,** which is the test that counts: playing blind, did he want the debrief, did it tell him something his one game had not, and did he want to fight the level again with a changed plan.

If 1 to 5 cannot be made to hold on a level this small, the direction is in doubt and it is cheap to have found out.

**What must be built for it, beyond decision 0232's steps 1 to 3.**

| Piece | What | Size |
| --- | --- | --- |
| The mission held | An intent given from above that its side's commander cannot withdraw, replace or widen: one mark on an intent, set by a scenario, checked in intake. State, so the hash's layout rises | Small. A decision record with a rule card |
| The debrief | A tool of the umpire's that takes the commands a player signed before day 1, plays them cold and the unwritten mission over the seeds of the situation he fought, and reports the two side by side with his own game placed among his plan's. In `sim-cli`, beside `batch`; the session is handed it as it is handed the trial's runner, and may show it only when his game is over. The rules of "cold" are written once, in its record | Medium. No rule of the game, and nothing waits on it to open a level |
| The plan of his own, and the history | The level's taught part named in its file; the check at signing; the record of each game with its plan, situation, seed and result, the last counting | Small to medium. The record is the session's or the table's, not the game's state |
| The level's file and record | Scenario 137 grown, in three situations or more, with its trial figures and its mission's measure | Medium. The hand calculation covers the days before the fights; the fights have dice and are read by batch under decision 0187's guards |
| The level on the table | The mission shown as given; the part that is his to write; the plan signed; at the end the mission met or not, the costs and the account; the debrief if he asks; the history; play again | Medium |
| The situation drawn | The level draws the situation and the seed of his game after he signs; a replay draws a situation not yet fought | Small |
| The check that a plan is worth writing | Agents write plans and are played over the seeds; conditions 1 to 5 | A few runs, and the architect's reading |

**What fell out of the build on Nov 27:** the floor and the test of beating par as a pass; the game's part, with its band, its cap and the cold game of his own seed as a score; a scale of the level's own on which one game is read (the mission met and the costs are enough again); the count of tries; and the check that a level can be passed, since none can be failed shut. **What stays:** the mission held from above, the rules of "cold", the many seeds (now the debrief's), the situations, and the test of a difference clear of noise, as a sentence in the debrief.

Not needed for it: forces that are not his, a new kind of node, difficulty, any scale of points.

## Against the pillars, the spec and the open work

This note amends none of them. The designer has since asked for the pillars and the onboarding note to be amended now, and the architect's drafts are in those files, marked for his review. What moves:

- **The five pillars stand.** A level is command by intent with a staff that explains itself. Pillar 1 is narrowed in practice: a ladder teaches one altitude at a time and leaves "anything can be grabbed" to the last rungs.
- **The concept shifts.** `docs/pillars.md` opens with "the player commands by intent and a staff executes". "The game is about planning" puts the weight on the plan before the first day, where the pillars put it on interrupts and running time (pillar 3). Both can be true of one game, and the note above keeps both: the plan is judged cold, the game is fought warm. If the designer means the plan to be the core, that is a line for "Since these pillars were written", in his words.
- **The cut on mandatory micromanagement.** The designer has ruled that trying a level means signing a plan of his own, knowing it makes writing mandatory inside a level. The cut's words still hold in one sense and not in another, and the proposed block in `docs/pillars.md` says both: the staff can do every task of a level, and does every one he does not write; but inside a level the taught part is his to write, and the staff's writing it for him does not count as his try.
- **The first slice and the build sequence** name the area commander's seat on the South Pacific as the first playable version, and step 5 as more theatres. A ladder puts small invented wars before the slice and makes the 39 days its top rung. Step 5's order would change.
- **The kill gate.** The designer has ruled it is judged on levels. Its proposed questions are reworded again in `docs/pillars.md`, since beating the staff is no longer what a level asks: the first is now whether writing a plan and seeing it fought is worth doing again.
- **`docs/step3-spec.md`.** The target (the 39 days from one statement, both sides) and stage 5 (a model by intent against the same model ordering units) are untouched: a level is not a proof's scenario. Point 6 of the target, "The choices", asks for built situations with two real courses; a level is one, and could serve as one of them if its record says so before it is run. The spec keeps a choice of seats out of step 3, which is why "part of a side" is not in the first level.
- **`docs/open-work.md`.** The objectives scale set from the record stays open and stays where it is, before stage 5; levels do not wait on it. Decision 0189, difficulty, is planned and would sit under a ladder. Nothing is closed by this note.

## What it does not settle, and the risks

- **A puzzle with one answer.** If one plan is best on every count, the level is solved once and is then a form. Conditions 3 and 4 are the guard, and they may not be satisfiable on a small level.
- **A level known after one play.** The seeds move the dice, not the enemy, and a debrief shows the truth of the situation fought. A replay is therefore on another situation, by the designer's ruling; once every situation has been fought the level is known, and the history says so.
- **The staff is not finished.** Every mend to a doctrine moves what the debrief shows for the staff's plan, and may undo conditions 1 to 5 of a level. They are checked again with each doctrine.
- **A staff that plans well.** Where the staff's plan is as good as a careful player's, the debrief says the two cannot be told apart. Since nothing is gated on beating it, that is a fair thing for a level to say, and no longer in tension with a staff the player trusts.
- **A ladder longer than the kinds that can be opened.** Rungs 8 to 10, 12 and 13 each wait on doctrine read for a written node, or on new rules. The ladder that can be built soon is rungs 1 to 7.
- **Small scenarios, each with a hand calculation.** Every scenario's test has one, written by the architect before code. Twelve levels are twelve of them, and each for-play file also has its runs and its games between agents. The first level is cheap because scenario 137 is already owed.
- **The verdict is one count.** "Met by the date on more games" will not fit a level about supply or attrition. Each level names its own measure, with its reason first, as decision 0187 asks; a ladder needs a few kinds of measure, not one.
- **A plan played cold flatters plans that need no author.** The debrief plays both plans cold, so a plan that leans on its commander's hand looks worse there than it fought. His own game placed among them is the only answer, and the debrief should say in words that it was steered and they were not.
- **One game teaches little by itself.** A player who never asks for the debrief learns from luck. It is his choice by the designer's ruling; the table should make the debrief one touch away and not a chore.
- **A try can be nominal.** One limit set to what the staff would have chosen opens the next level. The designer chose not to gate on merit, and this is the price.
- **The last game counts, so a record can be polished** by replaying until the dice are kind. Nothing is gated on it and the history shows every game, so it harms nobody.
- **Fun is not shown by any of this.** The five conditions say a level has a decision in it. Only the designer playing says whether deciding it is a pleasure.
- **Not settled here:** whether a level may be played from the enemy's side; what a level tells a player who is below par (the staff's reasons are the obvious teacher, and they are written for the umpire today); whether an agent's ladder is the same ladder.

## The designer's answers, Nov 26, 2026

Given one by one, in two rounds. Quoted text is his own words.

To the four questions this note first asked:

1. **Invented or historical first:** invented first. The first level is the placeholder above.
2. **How a level is graded:** both the plan and the game. "Both. Because I don't believe that planners who only planned but who's plans where never tried in battle, are remembered. So the player will get scoring on their plan and an addition for the game." Of three shapes for the addition he chose: the plan's score across the seeds is the base; the game adds what his steering gained over his own plan played cold on the same seed; steering worse than that subtracts ("The game can also cost you"). The architect had recommended the plan alone.
3. **May a level be passed at par:** no. "Must beat par to pass." He chose it knowing it makes writing a plan mandatory inside a level. The architect had recommended yes.
4. **The pillars and the onboarding note:** "Amend now." The architect's drafts are in `docs/pillars.md` and `docs/design/onboarding.md`, each marked as proposed and awaiting his review.

To the five that followed:

5. **Can the game's part change the pass?** "The subtraction can fail it but users may try again with the same plan(s). I think that users may also write consecutive and parallel plans." The architect had recommended that the pass be the plan's alone.
6. **What he meant by parallel plans:** both of the senses offered, plans in force at once on the same ships and aircraft, and alternatives held ready, written beforehand with the condition that triggers each. He was told the second is new work.
7. **Which attempt counts on the ladder:** "First signed plan is kept." His first signed plan is the one recorded; every later play, with the same plan or an amended one, is scored and shown; each play draws a fresh seed and one of the level's variants.
8. **Is there play outside the ladder:** "I think the ladder is the game, but we may decide later that free play is unlocked."
9. **The first playable version:** "The first levels." The slice becomes the top rung, reached later, and the long order-of-battle work for it waits.
10. **What the kill gate is judged on:** "Judged on levels." The wording of the gate's questions is his to review; the architect's proposal is in `docs/pillars.md`.

To the one question that followed:

11. **Are tries of the game unlimited:** yes, the architect's recommendation, offered to him as: "Unlimited, count kept: try as often as you like. The ladder keeps your first game's result and the number of games it took, so a pass on the first try reads differently from a pass on the sixth."

## The change of Nov 27, 2026, and what it replaces

His words: "Here's a thout I'm having, and it's a change: the player may choose to score their plan against the staff, but like irl, their plan is measured once in the war itself. Once. So, yes, they may be lucky or not. The goal is to learn and improve, feeling more and more accomplished. Failure will not block them from proceeding to the next step. Not trying will. If they choose to continue after failing - fair enough."

Two points it left open, each answered the same day:

12. **What counts as trying a level:** "Signing a plan of your own." He must write at least the part the level teaches and fight it; leaving it all to the staff does not count.
13. **May a level be played again:** "Replay on a new situation, player can amend their plan, record keep history, and the last game played counts. That encourages replayability, learning, exploration and curiosity."

The earlier answers stand in the record above and are replaced as follows:

| Earlier answer (Nov 26) | Replaced by |
| --- | --- |
| 2. The plan's score across the seeds is the base, and the game adds to it or subtracts | The plan is measured once, in the game he fights. Scoring against the staff is his choice: the dry run before, the debrief after |
| 3. "Must beat par to pass" | Failure does not block the next level; not trying does (12) |
| 5. The game's subtraction can fail a level, with another try allowed | Nothing fails a level shut. A game is fought once; a replay is on a new situation (13) |
| 7. "First signed plan is kept" | The history is kept and the last game played counts (13) |
| 11. Tries unlimited, the count kept | Overtaken: there are no tries of one game. The history shows every game |

Answers 1, 4, 6, 8, 9 and 10 stand: invented first; the pillars and onboarding amended now; consecutive and parallel plans; the ladder is the game; the first playable version is the first levels; the kill gate is judged on levels.

**Overtaken with them,** two readings the architect had flagged as his own: that an addition cannot pass a plan that did not beat par, and that a later plan may pass a level. There is no pass by score for either to be about. So is his settling that the cold result of the player's own seed is kept from him: no such result is a score now, and the debrief as a whole is shown only after the game.

Judged by the architect, of what the manager said to the designer and he did not answer: each is right and is in the sections above. This replaces what the manager listed. The debrief is where the many seeds earn their place. A comparison before the game must be the fallible one, on the side's picture, and the exact one comes only after.

Settled by the architect: the debrief is of the situation he fought and no other; a level has at least three situations; when all are fought a replay draws among them and the history says so; what a level's taught part is for each kind of rung ("What counts as trying").

## Questions for the designer

None is open.

## The first three rungs, as plain scenarios (Nov 27, 2026)

The designer was given the architect's view that the ladder should start tiny, with the 39 days as its top rung, and answered four questions the same day. Decision 0237 is the plan; this section says what the answers change in this note.

14. **Rung 1 is by intent.** He names the ship, the source, the floor and the interval, and the staff sails her.
15. **Rung 1 may be a tutorial with one right answer,** with no enemy: a decision only by scarcity and time.
16. **Rungs 1 to 3 ship as plain scenarios on the table's list.** "It can be plain. But it doesn't need to be judged against the staff's, only against the play itself."
17. **Troops seize an empty place,** which is built, and do not reinforce a held base. The cut of rungs 2 and 3 was left to the architect.

What a rung of these three is: a scenario for play, a mission in a directive, and a line at the end that says whether the mission was met in the game he played, read from what his seat was told. Of the five things "What a level is, as an object" sets beside a scenario, they have the first as words and the fifth; the mission is not held by the engine, everything of his side is his, and no staff's plan is set beside his.

| In this note | For rungs 1 to 3 |
| --- | --- |
| The mission given from above and held by the engine | Words on the table. He can withdraw his own order, and the verdict is of the game whatever he did |
| Scoring against the staff: the dry run before, the debrief after | Neither. No dry run is offered (decision 0237, question 2) and no debrief is made |
| What counts as trying, and "not trying blocks the next" | Nothing is checked and nothing is gated: the three are open from the start, beside the battles |
| The history, the situations, a replay on a new situation, the last game counts | None. One situation each, with no enemy to vary; he may play a rung again as he may any scenario, and nothing is kept |
| "Measured once, in the war" | Stands, and is the whole of it |
| The check that a plan is worth writing, conditions 1 to 5 | Not asked of a tutorial. Rung 1 has a band of right answers by answer 15; the conditions begin with the first rung that has an enemy |
| "The first level": two landings | The fourth rung, on decision 0232's screen, with a record of its own |

**The three, in a line each** (decision 0237 has the forces, the missions and what was tried):

1. **One ship.** A forward garrison five days from hunger, a source with men of its own on a stock 5 t above what both need, one ship that carries six days of the forward garrison's food, fuel for two round trips, 14 days. His decision: the floor and the interval of one supply order; too little starves one place and too much the other.
2. **Two mouths, two ships.** A near garrison and a far one, a slow freighter and a fast small transport, 14 days. His decision: which ship is named to which order.
3. **Men, then food.** 600 men to put on an empty place and keep fed, 21 days. His decision: the risk that sets the day of the landing, and how the place is fed: a hold given with the landing, or a supply order when the place is his.

**Why three and not two.** Each adds one decision to the one before. Rung 2 moves no man, since answer 17 puts troops where a place is seized; rung 3 is that seizure, and practises rungs 1 and 2 after it.

**The measure is a night unfed, not a man lost.** On the live figures of hunger a man is lost only on his unit's fifteenth hungry night, so nobody can be lost in these games; a night unfed is what the seat is told each morning. "The verdict is one count" under "What it does not settle" foresaw it: a level about supply names its own measure.

**Everything above this section stands for the rungs that follow,** from the fourth: the mission held, a plan of his own, the history and the situations, the debrief, and the conditions.
