# 0183: What the enemy is scored for is shown to both sides

Status: a plan by the architect, written on Nov 20, 2026, at the designer's asking. It changes decision 0054, item 6. Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ) before any of it is built.

Summary: A side has been shown its own mission and objectives and never the enemy's. Seven games of the 39 days played by agents, at the architect's asking on Nov 20, 2026, ended the same way: the Japanese commander bombed the airfield and chased warships, and never went for the transports, because nothing told him the Allied side is scored for every ton it lands. The scoring is a rule of the game, as the levels of victory are, and both players of a wargame read its victory conditions. A side is now shown what the enemy is scored for: the kinds of thing, the places named and what each is worth. It is not shown the enemy's mission, his progress or anything he knows

Standing: A plan, written by the architect on Nov 20, 2026, at the designer's asking, and amended on review. Not built

## Why

The designer, on Nov 20, 2026, after four games between agents of different strength: "Show the Japanese commander what the Allies are scored for."

What the games showed (they are no part of the repository: seven games of two agents, kept by the architect's session; the figures are the umpire's results as each game's record gave them). On the 39 days the Allied side's largest objective is supply and fuel landed at Lunga: 25 to 37 of its 58 to 90 points in the last four games. The Japanese briefing lists the Japanese objectives alone. Both of the stronger Japanese commanders said afterwards that the objective was not in their briefing and that it decided the game; neither struck the transports while they unloaded.

What is so today, read from the code and the records:

- **Decision 0054, item 6:** "A side sees its own mission and objectives from day 1, in the briefing, and never the enemy's." `SideView::objectives` gives the side's own, and its comment says "Never the enemy's".
- **The levels and the horizon are already given to both sides** as the scenario's configuration: `SideView::levels` and `SideView::horizon_days`, each with a comment that it is common to both sides and says nothing of the enemy's state.
- **An objective is configuration, not state.** It is a kind (ships sunk, ships crippled or out of action, aircraft destroyed, supply landed at a base, a base open or shut by days, a base held at the end, men lost) with its points and, for some, a base. Nothing in it changes during a game and no rule of the simulation reads it.
- **A side's progress** on its own objectives is counted from what it can see (decision 0054, item 7). The enemy's progress is the enemy's.
- **A mission is a sentence the scenario writes for one side,** and may say what that side believes of the other ("The American carriers are believed to be south or east of Guadalcanal").
- **A seat at a command** is given no objectives: they are the area command's.
- **The table shows a person no points** (decision 0157): a directive from the commander above says what is wanted. The briefing an agent reads lists the objectives with their points.

## The rules

1. **What each side is scored for is a rule of the game and both sides may read it.** This replaces decision 0054, item 6, in part: a side is still shown its own mission and objectives, and is now also shown the enemy's objectives.
2. **What is shown of the enemy's objectives:** each one's kind, the base it names where it names one, and what it is worth (its points, its points a ton, its points for each kind of ship), in the order the scenario gives them. For an objective judged over a horizon, the horizon.
3. **What is not shown:** the enemy's mission; his progress on any objective, known or true; whether a base of his is open now; and anything else of his state. The umpire's result at the end is as it was.
4. **A base an enemy's objective names** is named as the seat names any base: by the side's own number for a base of its own, by the map's for a base on its map. Every base an objective names is one the scenario has, and bases are on both sides' maps; if one is not on the side's map it is named by its place alone.
5. **Where it is shown:** in the briefing's objectives part, after the side's own objectives, under words that say whose they are ("The enemy is scored for:"), in the briefing as text and as data, from day 1 and on every day. Not at a seat at a command, which is given no objectives.
6. **A scenario with no objectives** shows none, as today. A scenario whose enemy has no objective says so in a line.
7. **Nothing else reads it.** No staff, no trial and no rule of the simulation reads the enemy's objectives. A commander reads them and decides.

## What it leaves for later

- **The table.** A person is shown no points. Whether a directive from above should say what the enemy is after, in words, is the designer's and the directives' authors' to settle; the directives are not changed here.
- **A staff that plans against what the enemy is scored for** (a doctrine that hunts transports because supply landed is worth most to the enemy). It would be a rule of doctrine with its own record.
- **A side's belief of what the enemy wants,** as a scenario states it for the trial (decision 0161, rule 8), is a different thing and is not changed.

## Rule card

- **Reads:** the scenario's objectives of the other side.
- **Writes:** nothing.
- **Reuses:** the briefing's objectives part and the way an objective is written for a seat.
- **New:** in `sim-core`, one reading on a side's view that gives the other side's objectives as configuration, beside the levels and the horizon. In `sim-agent`, the enemy's objectives in the briefing. No state, no event, no draw.

## What it costs

- **Truth and belief.** `docs/engineering/truth-and-belief.md` says never to add a method to a side's view that returns enemy truth. This is not truth of the enemy's state: it is the scenario's scoring, fixed before play, of the same kind as the levels and the horizon. The page gains that line, so that the next reader does not take it for a precedent for anything of the enemy's state.
- **No game changes:** no state, no hash, no saved game. The staffs do as they did.
- **Every briefing of a scenario with objectives gains lines.** The agent interface's version rises; the briefing's schema gains the part; the recorded transcripts are remade and differ in their briefings.
- **Documents:** the rulebook's chapter 6 and chapter 1 (what a side knows); the glossary; the agent's schemas and their README; the tool server's opening words, which say today that a scenario gives "your side" a mission and objectives.

## The test

There is no arithmetic. `tests/scenario_102_the_enemys_objectives_shown.rs` states what must hold, on existing scenarios, and is written before the code.

## Choices made for the designer

1. **Both sides are shown the other's,** not the Japanese side alone. A rule of the game is the same for both.
2. **The points are shown,** as a side's own are. Without them a commander cannot tell that supply landed outweighs a carrier.
3. **The enemy's mission is not shown.** It is his orders, and may say what he believes of you.

## Amended on review

An independent reading of the plan and its test, against the code, confirmed that an objective is configuration and that naming a base in one can reveal nothing, and found the following. Each was checked before it was taken. This section replaces what is above it where they differ, and the test's statements are amended with it.

1. **An enemy's objective has a line of its own, not a side's own line.** A side's own objective, as a seat is given it, carries the reader's state: what it knows of its progress, and whether a base of its own is open now. The enemy's is written by a separate, smaller form with no room for either: the kind; the base; for a base open or shut by days, which of the two it is; the points, the points a ton, or the points for each kind of ship; and, for ships out of action, the horizon. The briefing's schema gains a definition for it.
2. **"Open" or "shut" is part of the objective** (rules 2 and 3). The Japanese objective on Lunga is that it be shut; without the word it would read as the reverse.
3. **A base is named by the map's number and name** (replaces rule 4). There is no number of a side's own for a base. Every base a scenario states is on both sides' maps from the start, and the loader refuses an objective that names a base the scenario does not have; only a base a landing forms is off a map, and no objective can name one. So an enemy's objective can name no base a side does not already have on its map, and the case of a base named by its place alone cannot arise.
4. **The reading on a side's view gives the objectives alone,** never the table that holds the mission with them. Like the levels and the horizon, it gives nothing at a seat at a command.
5. **The enemy's objectives are written in words of their own** (rule 5). A side's own are written from its side ("enemy ships sunk", "tons you unload at"); the enemy's are written from the reader's side too: "your ships sunk", "tons the enemy unloads at Lunga", "Lunga is the enemy's at the end", "days Lunga is shut". They come after the side's own objectives and the line of the levels.
6. **In the data** the part is left out where the scenario has no objectives or the seat is at a command, so those briefings are what they were; and where the side has objectives and the enemy none, it is there and empty, and the text says so in a line.
7. **It discloses what the enemy is after, and that is the point.** An objective that he hold Lunga, or keep it shut, says in substance what his mission says. The mission's own words stay his, since they may also say what he believes of the reader. This is no leak of his state.
8. **The table's situation sheet shows the briefing's text whole,** and so shows a side's own points today, against decision 0157's "a person is shown no points"; it will show the enemy's as well. This record does not change the sheet. Whether a person at the table should read either is put to the designer.
9. **Also touched,** beyond what "What it costs" lists: the description of the tool `get_briefing`; the golden briefing of the agent's tests; the paragraph of the recorded games' README that says what they were remade after. The agent's source test bans nothing here and is not changed; it gains one case, that two worlds differing only in the enemy's mission give the same briefing.
10. **The test, corrected:** the scenario with command levels on and objectives is `data/scenarios/stage-2b-proof/05a_eastern_solomons.toml`; a side "with no objectives" keeps its table and its mission and has no objective entries, since the loader requires both sides; on the full slice the Japanese side's objectives name Lunga alone; and Lunga changes side in the scripted run only where the session hands in the script's orders for both sides.
