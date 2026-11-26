# Design note: levels

Status: a note for discussion, written by the architect on Nov 26, 2026 at the designer's asking, after he read decision 0232. Nothing here is built or planned, and it takes no decision number. It says what a level would be, how a plan would be judged, what skills a ladder of levels could teach, and what the first level would need. It ends with questions for the designer. Where it speaks of history it names what is on file and invents nothing.

The designer's words: "The game is about planning. The player will be taken through levels. Each level they will get to learn some new planning skills and practice previous skills. So scenarios will become more challenging (more units, longer scenarios). So the player will set a plan, maybe dry run it, amend, see how it plays, and so on. It could be that the player will only get command for some of the units and will need to fit a plan for a certain date."

## The idea in one line

A level is a small war with a mission given from above, in which the player writes as much of the plan as he chooses, tries it, and is then shown how that plan fares over many games beside the plan the staff would have made alone.

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

1. **The staff's own result is par, and a blank node earns par.** Kept. It is what keeps the cut on micromanagement: nothing is the player's alone to do, and doing nothing is a fair score. One consequence is hard and is said under Risks: a level is only worth playing where the staff's own plan is neither the best nor foolish.
2. **A plan is graded across seeds, and the game he watches is one of them.** Kept. One game is a draw from a spread; a good plan loses some games and a bad one wins some.
3. **The dry run stays fallible.** Kept, and it is so already: a trial is played in worlds built from the side's picture, never from the live enemy. It needs one thing more to matter, below: the truth must be able to differ from the picture in a way the player cannot learn by playing the level once.
4. **The scoring scale must be real.** Changed. The points of the record's scale are placeholders and stay the architect's to set from the record before stage 5; a level does not need them. A level is judged in the words the trial already uses: on how many games the mission was met by its date, and what it cost. No points are added up.
5. **Each level opens one more kind of node.** Changed. A skill of planning is not always a new kind of node. The first skills are taught with what a commander can already do: give an order, name forces, rank two orders, set the risk, say what must be known, tell a plan to wait, try it at two dates. Only the later rungs need a pair that decision 0232 leaves for later. Tying every rung to a new pair would make the ladder exactly as long as the doctrines that have been read for a written node, and the supply run, the escort and the search of the first guess are not nodes a commander writes at all: a supply order is a root today, an escort is the staff's, and "search waters" is refused to a commander.
6. **One level is built as the test before any ladder is designed.** Kept. It is planned below.
7. **The pillars would need the designer's amendment.** Kept in part. See "Against the pillars".

### What is compared

- **The plan is his commands before the first day runs:** the nodes he wrote, their limits, the leaves he gave. This is "the plan played cold". It is the same thing a trial calls a course.
- **It is played by both staffs, with nobody steering,** from the level's true start, on each of the level's seeds; and so is the same mission with nothing written. The two are compared seed for seed, since they share their dice wherever they do the same thing.
- **Two things are read from each game:** whether the mission was met by its date (for an island: every place the side's at the end of that day), and what the side lost (the five costs a trial already gives).
- **The verdict has three values and a band.** Above par: the mission is met on more games than the staff's plan by more than the band. Below: on fewer by more than the band. Otherwise at par. The band is set, with its reason, before the level is played by anyone, as decision 0187's is. The costs are shown beside the verdict and do not enter it: a plan that meets the date more often and loses more men is above par and is shown to have paid for it.
- **How many seeds.** The proofs use 1,000, and the band is sized to that count. A small level of a week should cost little at that size; it is to be measured on the first level before anything leans on it, in the browser's build as well as the desktop's.

### The player's hand

A plan played cold cannot answer an interrupt. Where the staff would stop and ask (leave to launch, "cannot bring enough", a wait that can never be met), a cold game does what the game does with nobody at the seat: with leave given it goes on, and where the staff waits for the commander it waits to the end. That is a fair verdict on a plan that needed its author.

The game the player himself plays is another thing: he steers it, answers its interrupts and amends. It is one game on one seed, and it is where the story is. It is not the grade, because what he did on day 4 answered that game's day 3 and means nothing on another seed. It is shown beside the spread of his cold plan: where his own game fell among the thousand.

So a level gives him two things at its end: the verdict on the plan he wrote, and the account of the game he fought. A level could be passed on the first alone.

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
| 3 | Which of two orders comes first | Rank | Built | The same placeholders; the Eastern Solomons in five days, for play, where a convoy and a carrier battle compete |
| 4 | How much to risk, and how much must be known first | Risk, knowledge | Built | Placeholder (scenarios 72, 82); the Coral Sea in five days, for play: when to turn a landing back |
| 5 | When: hold a plan to a day, to a force, to another plan; try it at two dates | Not before, wait for, after; the window | Built | Placeholder (scenarios 80, 81, 96) |
| 6 | Try, compare and amend before committing | The trial with `compare` | Built | Any of the above whose file names the trial's figures |
| 7 | Two landings on shared shipping: which first, with what, by a date | A take written under an order for an island | Decision 0232, steps 1 and 2: planned | Placeholder: scenario 137 grown (the first level, below). In history: the landings at Lunga and Tulagi of 7 August 1942, the first days of the 39 days, whose facts are on file |
| 8 | Where the carriers stand, and until when | A carrier task written under his own order; a day on which a task ends | New work: 0232 leaves the pair for later, and no limit ends a task on a day | The Eastern Solomons in five days; Midway. Facts on file |
| 9 | A surface force placed to meet the enemy by night | A stand task written under his own order | New work | Savo Island and Cape Esperance have facts and scenarios of record; a copy for play of either does not exist |
| 10 | Holding a place and keeping it fed under attack | A hold written under an island; its source | New work, and the hardest pair (0232, What it leaves out) | The 39 days |
| 11 | A campaign: several orders that draw on the same ships for weeks | Everything above | The orders are built as roots | The 39 days |
| 12 | A plan for part of a side, fitted to a superior's and a neighbour's | A mission held from above; forces not his; a trial at such a seat | New work | Any battle with command levels on (the files of stage 2b's proof have them; whether a file for play does was not checked) |

**Rungs with no scenario:** 8 to 10 have none made for play that isolates the decision; 12 has none at all. **Rungs with no facts:** none of the twelve needs a battle that is not researched. A ladder that goes on to "more units, longer" past the 39 days does: the battles of October and November 1942 have order-of-battle facts and no scenario, and nothing outside 1942 is on file (`docs/design/larger-scenarios.md`). An invented level is marked `placeholder` and named "Placeholder", as every made-up thing is.

Rungs 1 to 6 need no new rule of the game. They need levels made, the mission held, and the judge.

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

**Its mark.** The staff alone, 1,000 seeds over the variants: on how many games both places are the side's by the date, and the middle of what it lost. Recorded with the versions it was worked under, and worked again when the doctrine changes.

**Why the staff's plan is beatable and not foolish.** The staff forms both takes on the first morning in the island's order; the first chooses its men against its estimate and the second takes what is left (chapter 8, Controlling an island; Not built: "the order the places are taken in"). It does not weigh one place against the other, does not put the small place first to free its ships, and does not wait for the later force unless a take is short. Those are the three things a player can do. Each has a cost: the small place first delays the large one; everything on the large one leaves the small one for a second trip; waiting spends the date; looking first spends days and a group.

**"Beating par was a struggle" against "it was filling a form".** Checked by the architect with agents before the designer sits down, as decision 0187's third guard has it, and written into the level's record before any run is read:

1. **Par is not the best plan:** at least one written plan is above par by more than the band.
2. **Par is not the worst:** at least one plausible written plan is below par by more than the band. If every written plan beats the staff, the level is a form with one box.
3. **No plan wins on everything:** the plan that meets the date most often does not also lose the fewest men. If one does, the level is a puzzle with one answer, and it is rebuilt.
4. **The dry run does not give the answer away:** in at least one variant the plan the dry run likes best is not the plan the truth likes best.
5. **The designer's own account,** which is the test that counts: playing blind, did he try more than one plan, did the dry run change his mind, and can he say what he gave up.

If 1 to 4 cannot be made to hold on a level this small, the direction is in doubt and it is cheap to have found out.

**What must be built for it, beyond decision 0232's steps 1 to 3.**

| Piece | What | Size |
| --- | --- | --- |
| The mission held | An intent given from above that its side's commander cannot withdraw, replace or widen: one mark on an intent, set by a scenario, checked in intake. State, so the hash's layout rises | Small. A decision record with a rule card |
| The judge | A tool of the umpire's that takes the commands a player signed before day 1, plays them and the unwritten mission over the level's variants and seeds, and reports the two seed for seed. In `sim-cli`, beside `batch`; the session is handed it as it is handed the trial's runner, and may show it only when the level is over | Medium. No rule of the game |
| The level's file and record | Scenario 137 grown, with its variants, its trial figures, its measure, its band and its par | Medium. The hand calculation covers the days before the fights; the fights have dice and are read by batch under decision 0187's guards |
| The level on the table | The mission shown as given; the verdict and the spread at the end, with his own game marked in it | Medium |
| The check that it is a struggle | Agents write plans and are judged; conditions 1 to 4 | A few runs, and the architect's reading |

Not needed for it: forces that are not his, a new kind of node, difficulty, any change to scoring.

## Against the pillars, the spec and the open work

Nothing here amends any of them. What the designer would have to decide:

- **The five pillars stand.** A level is command by intent with a staff that explains itself. Pillar 1 is narrowed in practice: a ladder teaches one altitude at a time and leaves "anything can be grabbed" to the last rungs.
- **The concept shifts.** `docs/pillars.md` opens with "the player commands by intent and a staff executes". "The game is about planning" puts the weight on the plan before the first day, where the pillars put it on interrupts and running time (pillar 3). Both can be true of one game, and the note above keeps both: the plan is judged cold, the game is fought warm. If the designer means the plan to be the core, that is a line for "Since these pillars were written", in his words.
- **The cut on mandatory micromanagement** is kept only while a blank earns par and a level can be passed at par. A ladder in which a level must be beaten to reach the next would make writing a plan mandatory. That is his to rule (Question 3).
- **The first slice and the build sequence** name the area commander's seat on the South Pacific as the first playable version, and step 5 as more theatres. A ladder puts small invented wars before the slice and makes the 39 days its top rung. Step 5's order would change.
- **The kill gate.** Its first question is "Is commanding by intent fun, or does it feel like watching a screensaver?", which is the designer's complaint of Nov 25. The first level is a direct test of it, and it fits what `planning.md` already gives the gate. The gate's questions need no change to be asked of a level.
- **`docs/step3-spec.md`.** The target (the 39 days from one statement, both sides) and stage 5 (a model by intent against the same model ordering units) are untouched: a level is not a proof's scenario. Point 6 of the target, "The choices", asks for built situations with two real courses; a level is one, and could serve as one of them if its record says so before it is run. The spec keeps a choice of seats out of step 3, which is why "part of a side" is not in the first level.
- **`docs/open-work.md`.** The objectives scale set from the record stays open and stays where it is, before stage 5; levels do not wait on it. Decision 0189, difficulty, is planned and would sit under a ladder. Nothing is closed by this note.

## What it does not settle, and the risks

- **A puzzle with one answer.** If one plan is best on every count, the level is solved once and is then a form. Conditions 3 and 4 are the guard, and they may not be satisfiable on a small level.
- **A level known after one play.** The seeds move the dice, not the enemy. Variants of the truth help for a few plays. A player who replays a level until he passes it is fitting a plan to the truth by trial, which is the opposite of the lesson. The honest answer may be that a level is judged on the first plan signed, and later plays are practice.
- **Par depends on the staff, and the staff is not finished.** Every mend to a doctrine moves every level's par, and a level whose whole struggle is one weakness of the staff disappears when that weakness is mended. A level's record must say which reading of the staff its struggle rests on, so that the mend is seen to end the level.
- **A staff that plans well leaves nothing to beat.** The game's premise is a competent staff. The better it gets, the less a player's plan can add, and the levels reward finding where it is still weak. That is in tension with pillar 2, a staff the player trusts. Where the staff is as good as a careful player, the right verdict is "at par", and a ladder must be able to live with that.
- **A ladder longer than the kinds that can be opened.** Rungs 8 to 10 and 12 each wait on doctrine read for a written node, or on new rules. The ladder that can be built soon is rungs 1 to 7.
- **Small scenarios, each with a hand calculation.** Every scenario's test has one, written by the architect before code. Twelve levels are twelve of them, and each for-play file also has its runs and its games between agents. The first level is cheap because scenario 137 is already owed.
- **The verdict is one count.** "Met by the date on more games" will not fit a level about supply or attrition. Each level names its own measure, with its reason first, as decision 0187 asks; a ladder needs a few kinds of measure, not one.
- **A plan played cold flatters plans that need no author.** A plan with a clever wait and a commander watching it may be the better plan and be marked down. The spread with his own game in it is the only answer this note has.
- **Fun is not shown by any of this.** The five conditions say a level has a decision in it. Only the designer playing says whether deciding it is a pleasure.
- **Not settled here:** how levels are unlocked; whether a level may be played from the enemy's side; what a level tells a player who is below par (the staff's reasons are the obvious teacher, and they are written for the umpire today); whether an agent's ladder is the same ladder.

## Questions for the designer

1. **Is the first level invented or historical?** The architect recommends invented: scenario 137 grown, small enough to prove by hand and to rebuild if the struggle is not there. The landings of 7 August 1942 are the same decision with facts on file and 133 ships, and are the better second trial once the shape is judged.
2. **Is a plan graded cold, on what was signed before the first day, with the game he steers shown beside it and not graded?** The architect recommends yes. It is the only grade that means the same thing on every seed.
3. **May a level be passed at par?** The architect recommends yes: the mission met as often as the staff meets it is a pass, and above par is a distinction. Otherwise writing a plan becomes the one thing only the player can do, which the pillars cut.
4. **Does the ladder replace the ladder of `onboarding.md`, and does "the game is about planning" go into the pillars?** The architect recommends deciding both after the first level has been played, not before: it is one level's work to find out whether the direction holds.
