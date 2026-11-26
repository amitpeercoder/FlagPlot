# Design note: levels

Status: a note for discussion, written by the architect on Nov 26, 2026 at the designer's asking, after he read decision 0232. Nothing here is built or planned, and it takes no decision number. Amended the same day to the designer's answers, given in two rounds ("The designer's answers", at the end): the score has two parts, the plan's and the game's; a level is passed only by a plan that beats par, and the game he fights can fail it; the ladder is the game; and plans may follow one another, stand side by side, or be held ready against a condition. It says what a level would be, how a plan would be judged, what skills a ladder of levels could teach, and what the first level would need. It ends with questions for the designer. Where it speaks of history it names what is on file and invents nothing.

The designer's words: "The game is about planning. The player will be taken through levels. Each level they will get to learn some new planning skills and practice previous skills. So scenarios will become more challenging (more units, longer scenarios). So the player will set a plan, maybe dry run it, amend, see how it plays, and so on. It could be that the player will only get command for some of the units and will need to fit a plan for a certain date."

## The idea in one line

A level is a small war with a mission given from above, in which the player writes a plan, tries it, fights one game of it, and is then scored: on how that plan fares over many games beside the plan the staff would have made alone, and on what his own hand added to it or cost it in the game he fought.

It continues decision 0232 and does not replace it. Decision 0232 lets a commander write part of a plan and set it beside the staff's in a dry run. A level adds three things: a mission he did not choose, a judgement after the fact on the truth, and a reason to come back for the next.

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
3. **What he may write.** Which kinds of node are open under the mission (decision 0232's table of pairs), named by the level so that a rung can open one more. The engine has one table for every game; a level would narrow it. **New, small:** a scenario's list of the pairs it offers.
4. **The mark to beat.** The staff's own result for the same mission and limits with nothing written, over the level's seeds, kept with the level and with the versions of rules, doctrine and figures it was worked under. **New:** the record of it, and the tool that makes it and judges a plan beside it.
5. **What it teaches.** One line, for the level's screen and for whoever builds the next level: the decision this level is about. It is words and no rule reads it.

A level's file is a scenario for play in every other respect and passes decision 0187's guards as one. It is never a proof's or a validation measure's scenario, and its results are never laid beside the record of a battle.

## How a plan is judged

### The manager's seven suggestions, each judged

1. **The staff's own result is par, and a blank node earns par.** Kept as the mark; changed by the designer's answer 3 in what it earns: par is no longer a pass. A blank still earns par, and par still means the staff can do every task of the level. One consequence is hard and is said under Risks: a level is only worth playing where the staff's own plan is neither the best nor foolish.
2. **A plan is graded across seeds, and the game he watches is one of them.** Kept. One game is a draw from a spread; a good plan loses some games and a bad one wins some.
3. **The dry run stays fallible.** Kept, and it is so already: a trial is played in worlds built from the side's picture, never from the live enemy. It needs one thing more to matter, below: the truth must be able to differ from the picture in a way the player cannot learn by playing the level once.
4. **The scoring scale must be real.** Changed. The points of the record's scale are placeholders and stay the architect's to set from the record before stage 5; a level does not need them. A level is judged in the words the trial already uses: on how many games the mission was met by its date, and what it cost. No points are added up.
5. **Each level opens one more kind of node.** Changed. A skill of planning is not always a new kind of node. The first skills are taught with what a commander can already do: give an order, name forces, rank two orders, set the risk, say what must be known, tell a plan to wait, try it at two dates. Only the later rungs need a pair that decision 0232 leaves for later. Tying every rung to a new pair would make the ladder exactly as long as the doctrines that have been read for a written node, and the supply run, the escort and the search of the first guess are not nodes a commander writes at all: a supply order is a root today, an escort is the staff's, and "search waters" is refused to a commander.
6. **One level is built as the test before any ladder is designed.** Kept. It is planned below.
7. **The pillars would need the designer's amendment.** Kept in part. See "Against the pillars".

### The score has two parts

The designer's ruling of Nov 26, 2026: "Both. Because I don't believe that planners who only planned but who's plans where never tried in battle, are remembered. So the player will get scoring on their plan and an addition for the game." The plan's score across the seeds is the base. The game adds what his steering gained over his own plan played cold on the same seed, and steering worse than his own plan played cold subtracts: "The game can also cost you."

### The plan's part

- **The plan is his commands before the first day runs:** the nodes he wrote, their limits, the settings he made. It is the same thing a trial calls a course.
- **Cold** means that plan played with nobody at the seat, by rules written once for every level:
  - leave to prepare and leave to launch are given, as a trial gives its course the leaves it lacks, so that a plan is judged as approved and not on a leave he forgot;
  - every other setting is as he signed it: the leave to pursue on each order, the interrupts, the holds;
  - nothing is answered. Where the staff waits for the commander ("cannot bring enough", a wait that can never be met, a force asking leave to pursue where he set "ask") it waits to the end of the game. An asking never lapses into a yes (decision 0214). That is the verdict on a plan that needed its author, and it is why steering can add.
- **It is played by both staffs** from the level's true start, on each of the level's seeds and variants; and so is the mission with nothing written, which is par. The two are set seed against seed.
- **What is read from a game:** whether the mission was met by its date, and what the side lost (the five costs a trial gives).

### Beating par, as a number

"Must beat par to pass" needs a win told from a tie and from noise. On N paired games, let *a* be the games in which his plan meets the mission and the staff's does not, and *b* the games the other way round. The games in which both meet it or neither does say nothing of the difference.

- **He beats par where both hold:** *a* − *b* is at least a floor (a placeholder: 20 games in 1,000, so that a difference too small to matter is not a pass), and (*a* − *b*)² is at least 4 × (*a* + *b*), which is the difference standing two standard errors clear of nothing, in whole numbers.
- **A tie is not a win.** The same plan as the staff's gives *a* = *b* = 0. A plan that differs and does no better gives a difference inside the noise. Both are at par, and at par the level is not passed.
- **The plan's score** is *a* − *b*: the games in a thousand by which he beat the staff. It can be below nothing.
- **The floor and the two are figures of the judge, one set for every level,** each with its reason written before a level is read with it; the count of seeds is 1,000 as the proofs use, to be measured for time on the first level, in the browser's build too.
- **Where the mission is met on nearly every game by both, or by neither, nothing can be told** and the level is badly made: its date or its forces are changed, not the test.

Checked against the source: the two games of a pair share their dice only in part. A draw is keyed on the seed, the day, the pulse, the subsystem and the entity (`rng.rs`, `RngKey::stream`), and the staff draws nothing. So where both plans have the same unit do the same thing on the same day and pulse, the die is the same; where the plans part, a unit that acts on another day or against another unit draws from another key. Pairing takes out the luck the two games share and leaves the rest. That is enough for a thousand pairs. It is not enough for one, which matters next.

### The game's part

- **He fights one game:** one variant and one seed, drawn by the level and not chosen, after the plan is signed. The plan as signed is frozen then as the cold plan; what he amends in the game is steering and changes the game, not the plan's score.
- **The same seed is played cold** by the judge, among its thousand. His steered game and that cold game are the pair.
- **What his hand gained or cost** is read on those two games. One game cannot be read by "met on how many": it is met or it is not. So the game's part needs a measure of a single game, and this is where a scale comes back: the level's own, of a scenario for play (decision 0187), with the reason for each figure written first. The mission met by its date is by far the largest thing on it; the costs are the rest.
- **It adds where his game is better than his cold game by more than a band, subtracts where it is worse by more than the band, and is nothing inside it.**
- **It is partly luck, and the note cannot make it otherwise.** The moment he steers, his units act on other days and draw other dice than the cold game's (above). A commander who gives his leaves on the first morning and then touches nothing gets the cold game exactly and a game's part of nothing, which is right. A commander who steers gets the effect of his steering and a share of fresh luck, mixed, and one pair cannot part them. Battle is like that, and the designer's reason for wanting the game scored is that it is. The designer was told so and ruled that the game can fail a level all the same, with the game open to another try (below).
- **He does not see the cold result of his seed before he plays, nor any result of the judge.** Settled by the architect: every one of them is read from the truth, and shown before the game is over it would tell him where the enemy is and how the dice fall. They are the umpire's, shown at the end, as a result is today.

### What passes a level

Ruled by the designer in two answers. The plan must beat par. And, of the game's part: "The subtraction can fail it but users may try again with the same plan(s)." He was told before he answered that one game's comparison is his steering mixed with fresh luck.

Read together with "first signed plan is kept", this gives the following. Where the note chooses between readings it says so.

- **A level is passed where both hold:** a plan he signed beats par, by the test above; and a game he fought with that plan was not worse than that plan played cold on the same seed by more than the band. A game that is worse fails that attempt. A game that is better adds to the score. (The architect's reading of "the subtraction can fail it": the game's part fails or does not, and beyond that moves the score. An addition does not pass a plan that did not beat par, which he was not asked and did not say.)
- **A retry of the game** is the same signed plan fought again: the level draws a fresh seed and one of its variants each time, and that game is set beside the cold game of its own seed. Nothing of an earlier game carries over.
- **A new or amended plan** may be signed on a later play. It is judged against par as the first was, scored and shown.
- **What the ladder records:** the score of the first plan he signed, kept and never written over; whether the level is passed, and if so on which play, by which plan, and after how many games fought. Every play's scores are shown to him at its end.
- **A level may be passed by a later plan.** The architect's reading, since otherwise a first plan that did not beat par would close the ladder for good. The first plan's score stays on the record beside the pass.

**Do unlimited tries empty the game's part?** Nearly, as a gate. By luck alone something like half of steered games come out no worse than their cold game, so a few tries pass almost any plan that beats par, and the gate then costs time and proves little. It is not empty as a score: the first game fought is one game, and so is the count of games it took. So the record carries both, and the gate is left as he ruled it. Whether tries should be limited is Question 1.

### What the trial can and cannot do for this

- **It can** be asked as often as he likes, of a draft with a written part beside the staff's own plan (0232, step 2), at three sizes; it shows what the plan needs, when it could go and how often it came off in the picture's worlds.
- **It optimises against the picture.** The same question at the same hour is the same worlds, so a player can tune a plan until the dry run likes it. That is a plan fitted to what he believes. Whether that is a good plan is the level's lesson, and it is only a lesson if the truth can differ.
- **It cannot be the judge.** It never reads the truth, and the judge must.
- **It is not offered at a seat of a command,** only at a seat that plays a whole side. A level in which part of the side is not his cannot use it until that is planned.
- **A window of dates is refused for a written part** in 0232's step 2. "What if I land two days later" is asked by a "not before" on the node, compared.

### What cannot be done with what exists

- **Nothing holds a player to a mission.** He can withdraw the order.
- **Nothing judges.** A batch is run by hand from a file. Nothing takes the commands a player signed, plays them beside par and reports the two.
- **The judged games read the truth,** so their result is the umpire's and can be shown only when the level is over, as a game's result is today. Shown before, it would tell him where the enemy is.
- **A level played twice is a level known.** Seeds change the dice and not the enemy's start. See Risks.

## The skills, as a ladder

Each rung is a decision a commander makes. "Built" means a player can make it at the table today. The scenarios named are candidates on file, not a plan; a file of record is never used as a level, a copy for play is made as decision 0187 has it.

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

Rungs 1 to 6 need no new rule of the game. They need levels made, the mission held, and the judge.

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
- the truth has two or three variants (the enemy's men divided differently between the places, each inside what the estimates allow), each a file of its own. No game on file has variants of the truth: the lettered files of a game for play differ in who plays each side. The player is not told which he has. The level is judged over all of them.

**The mission and its date.** Given from above and held: control the island, both places, by the end of a named day, at balanced. The date is set so that the staff's own plan meets it on some games and not on most.

**What he commands.** Everything of his side in the file. Nothing is a neighbour's.

**What he may write.** The two landings under the order, each with its forces, its "not before", "wait for" and "after", what must be known, and a risk no bolder than the mission's. He may write neither, one or both.

**Its mark.** The staff alone, 1,000 seeds over the variants: on how many games both places are the side's by the date, and the middle of what it lost. He passes by beating it, by the test of "Beating par, as a number"; at par he has not passed. Recorded with the versions it was worked under, and worked again when the doctrine changes.

**Why the staff's plan is beatable and not foolish.** The staff forms both takes on the first morning in the island's order; the first chooses its men against its estimate and the second takes what is left (chapter 8, Controlling an island; Not built: "the order the places are taken in"). It does not weigh one place against the other, does not put the small place first to free its ships, and does not wait for the later force unless a take is short. Those are the three things a player can do. Each has a cost: the small place first delays the large one; everything on the large one leaves the small one for a second trip; waiting spends the date; looking first spends days and a group.

**"Beating par was a struggle" against "it was filling a form".** Checked by the architect with agents before the designer sits down, as decision 0187's third guard has it, and written into the level's record before any run is read:

1. **The level can be passed:** at least two written plans that differ from each other beat par by the test, found by agents and kept with the level's record. Since a level must be beaten, a level no plan beats is a wall, and this is checked again whenever the doctrine changes, since a mend to the staff raises par.
2. **Par is not the worst:** at least one plausible written plan is below par by as much. If every written plan beats the staff, the level is a form with one box.
3. **No plan wins on everything:** the plan that meets the date most often does not also lose the fewest men. If one does, the level is a puzzle with one answer, and it is rebuilt.
4. **The dry run does not give the answer away:** in at least one variant the plan the dry run likes best is not the plan the truth likes best.
5. **The designer's own account,** which is the test that counts: playing blind, did he try more than one plan, did the dry run change his mind, and can he say what he gave up.

If 1 to 4 cannot be made to hold on a level this small, the direction is in doubt and it is cheap to have found out.

**What must be built for it, beyond decision 0232's steps 1 to 3.**

| Piece | What | Size |
| --- | --- | --- |
| The mission held | An intent given from above that its side's commander cannot withdraw, replace or widen: one mark on an intent, set by a scenario, checked in intake. State, so the hash's layout rises | Small. A decision record with a rule card |
| The judge | A tool of the umpire's that takes the commands a player signed before day 1, plays them cold and the unwritten mission over the level's variants and seeds, reports the two seed for seed with *a*, *b* and the test, and sets his steered game beside the cold game of its seed on the level's scale. The rules of "cold" are written once, in its record. In `sim-cli`, beside `batch`; the session is handed it as it is handed the trial's runner, and may show it only when the level is over | Medium. No rule of the game |
| The level's file and record | Scenario 137 grown, with its variants, its trial figures, its mission's measure, its own scale for one game with the reasons, the band of the game's part, its par and the plans that beat it | Medium. The hand calculation covers the days before the fights; the fights have dice and are read by batch under decision 0187's guards |
| The level on the table | The mission shown as given; the plan signed and frozen before the game; at the end the plan's score, pass or not, the spread with his own game marked in it, and what his hand added or cost | Medium |
| The seed drawn | The level draws the variant and the seed of his game after he signs, and a second play draws another | Small |
| The check that it is a struggle | Agents write plans and are judged; conditions 1 to 4 | A few runs, and the architect's reading |

Not needed for it: forces that are not his, a new kind of node, difficulty. Needed after all, by the designer's answer 2: a scale of the level's own on which one game is read.

## Against the pillars, the spec and the open work

This note amends none of them. The designer has since asked for the pillars and the onboarding note to be amended now, and the architect's drafts are in those files, marked for his review. What moves:

- **The five pillars stand.** A level is command by intent with a staff that explains itself. Pillar 1 is narrowed in practice: a ladder teaches one altitude at a time and leaves "anything can be grabbed" to the last rungs.
- **The concept shifts.** `docs/pillars.md` opens with "the player commands by intent and a staff executes". "The game is about planning" puts the weight on the plan before the first day, where the pillars put it on interrupts and running time (pillar 3). Both can be true of one game, and the note above keeps both: the plan is judged cold, the game is fought warm. If the designer means the plan to be the core, that is a line for "Since these pillars were written", in his words.
- **The cut on mandatory micromanagement.** The designer has ruled that a level must be beaten, knowing that it makes writing a plan mandatory inside a level. The cut's own words still hold: no task exists that only the player can do, since the staff plans every level and its plan is par. What changes is that inside a level doing only what the staff does is not enough. The proposed wording is in `docs/pillars.md`, marked for his review.
- **The first slice and the build sequence** name the area commander's seat on the South Pacific as the first playable version, and step 5 as more theatres. A ladder puts small invented wars before the slice and makes the 39 days its top rung. Step 5's order would change.
- **The kill gate.** Its first question is "Is commanding by intent fun, or does it feel like watching a screensaver?", which is the designer's complaint of Nov 25. The first level is a direct test of it, and it fits what `planning.md` already gives the gate. The gate's questions need no change to be asked of a level.
- **`docs/step3-spec.md`.** The target (the 39 days from one statement, both sides) and stage 5 (a model by intent against the same model ordering units) are untouched: a level is not a proof's scenario. Point 6 of the target, "The choices", asks for built situations with two real courses; a level is one, and could serve as one of them if its record says so before it is run. The spec keeps a choice of seats out of step 3, which is why "part of a side" is not in the first level.
- **`docs/open-work.md`.** The objectives scale set from the record stays open and stays where it is, before stage 5; levels do not wait on it. Decision 0189, difficulty, is planned and would sit under a ladder. Nothing is closed by this note.

## What it does not settle, and the risks

- **A puzzle with one answer.** If one plan is best on every count, the level is solved once and is then a form. Conditions 3 and 4 are the guard, and they may not be satisfiable on a small level.
- **A level known after one play.** The seeds move the dice, not the enemy. Variants of the truth help for a few plays. A player who replays a level until he passes it is fitting a plan to the truth by trial, which is the opposite of the lesson. The honest answer may be that a level is judged on the first plan signed, and later plays are practice.
- **Par depends on the staff, and the staff is not finished.** Every mend to a doctrine moves every level's par, and a level whose whole struggle is one weakness of the staff disappears when that weakness is mended. A level's record must say which reading of the staff its struggle rests on, so that the mend is seen to end the level.
- **A staff that plans well leaves nothing to beat.** The game's premise is a competent staff. The better it gets, the less a player's plan can add, and the levels reward finding where it is still weak. That is in tension with pillar 2, a staff the player trusts. Where the staff is as good as a careful player, the right verdict is "at par", and a ladder must be able to live with that.
- **A ladder longer than the kinds that can be opened.** Rungs 8 to 10, 12 and 13 each wait on doctrine read for a written node, or on new rules. The ladder that can be built soon is rungs 1 to 7.
- **Small scenarios, each with a hand calculation.** Every scenario's test has one, written by the architect before code. Twelve levels are twelve of them, and each for-play file also has its runs and its games between agents. The first level is cheap because scenario 137 is already owed.
- **The verdict is one count.** "Met by the date on more games" will not fit a level about supply or attrition. Each level names its own measure, with its reason first, as decision 0187 asks; a ladder needs a few kinds of measure, not one.
- **A plan played cold flatters plans that need no author.** A plan with a clever wait and a commander watching it may be the better plan and be marked down on its plan's part. The game's part is the designer's answer to that.
- **A level that must be beaten can become a wall.** Every mend to the staff raises par. A level's passing plans are checked again with each doctrine, and a level nothing beats is withdrawn or rebuilt.
- **The game's part rewards meddling as much as judgement.** A player who steers gets fresh dice, and by luck alone about half of such games come out ahead. A cap and a band are the only guards.
- **Fun is not shown by any of this.** The five conditions say a level has a decision in it. Only the designer playing says whether deciding it is a pleasure.
- **Not settled here:** how levels are unlocked; whether a level may be played from the enemy's side; what a level tells a player who is below par (the staff's reasons are the obvious teacher, and they are written for the umpire today); whether an agent's ladder is the same ladder.

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

Settled by the architect, of what he did not rule on: he does not see the cold result of his seed, or any result of the judge, before his game is over. Read by the architect and said where it is read ("What passes a level"): the game's part fails an attempt or does not, and an addition does not pass a plan that did not beat par; a level may be passed by a later plan, the first plan's score staying on the record.

## Questions for the designer

1. **Are tries of the game unlimited?** As ruled, a game that goes worse than the plan played cold fails the attempt and he may try again. With no limit, luck passes nearly any plan that beat par within a few tries. The architect recommends leaving them unlimited and making the record carry the weight: the first game fought with a plan, and the number of games it took to pass, are kept on the ladder beside the first plan's score. A limit would make one unlucky game close a level, which is harsher than anything he has said.
