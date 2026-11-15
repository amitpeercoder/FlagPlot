# 0119: Command levels: the designer's answers, and a stage for them between stages 2 and 3

Status: the designer's decisions of Nov 15, 2026, on the seven questions of `docs/design/command-levels.md`, put to him one at a time by the architect with a recommendation on each. It changes `docs/step3-spec.md`: a stage is added between stages 2 and 3, as the spec says its stages may be changed, by a decision record that says what and why. Nothing is built.

Summary: The designer's seven answers on command levels: initiative is a rating of each commander, who may even refuse an order at a cost to himself; a commander knows any force a level down only as last reported; an intent goes down with the part of the picture that bears on it; neighbours within radio reach hear each other; the player sits at the area's picture by default, with an easier setting; reports travel on the three pulses; and it is built after stage 2 of step 3 and before stage 3

Standing: Decided by the designer, Nov 15, 2026. A stage added to the step 3 spec. Not built

## Why

The designer's direction of the same day (`docs/design/command-levels.md`): a staff at every level interprets and splits the intent it is given; levels differ in what they know and in what they report, of the enemy and of their own forces; and a player may later take a role at any level. The note left seven questions. These are his answers.

## The answers

1. **Initiative is a rating of each commander.** How far a level changes its own task when what it sees no longer fits its intent depends on the man who commands it. A commander may even refuse an order, and that may cost him his command and his career. The ratings are not to wait for research: they are given values that make sense, marked as placeholders, and amended when research gives better. The architect had recommended one rule for all: a level may change its task inside the end one level up.
2. **A commander knows any force a level down as of its last report:** its position, its losses, its fuel. A force that reports each pulse is known almost exactly; one that is silent, cut off or whose report was lost is known as it was last heard from. As recommended.
3. **An intent goes down with what bears on it:** the contacts, the enemy's bases and the friendly forces inside the area and time of the task, with word of them as the superior learns more. A subordinate knows nothing of the rest of the theatre. As recommended.
4. **Neighbours hear each other, always, within radio reach.** A command hears the reports of any other of its side within reach of its radio. The architect had recommended sharing by standing instruction, set with the intent.
5. **The player sits at the area's picture by default,** with a setting that gives one picture a side, for learning the game and for comparing. The staff below plays on its own level's picture either way. As recommended.
6. **Reports travel on the three pulses** of the day. A finer clock can replace it later if play shows the need. As recommended.
7. **It is built after stage 2 of step 3 and before stage 3.** As recommended.

## What follows from them

- **Commanders become things in the game.** A command has a commander, from data, with a rating for initiative and, by answer 1, a standing with his superior that a refusal spends. No real commander is given a rating that is presented as fact: each is a placeholder with a note of what it rests on, as the data rules have it for every figure not yet sourced, until research replaces it.
- **Refusal and relief need a rule of their own,** and it can wait: it is written with the initiative part of the stage and put to the designer first. He set its direction the same day, softening answer 1: **a departure with cause is not held against a commander.** A carrier captain who does not launch the full strike he was told to because he has word that his own ship is under attack has cause. A commander who refuses because he chooses to, or does something very different with nothing he knew to justify it, may be relieved. So the rule turns on what the commander knew when he departed from his intent, which is what a reason already records.
- **Radio reach is a figure** of a command: of a ship's set and of a base's. Answer 4 makes it the whole of who hears whom, so it carries much of the fog between neighbours, with silence (a silent force hears and does not send) and the delay and loss of a report.
- **"Bears on the intent" needs a test** the engine can make: by the area limit of the intent and its dates, read up the tree.
- **A report is on time or a pulse late,** by answer 6. The delay and loss figures are chances on that clock.

## What it changes in the spec

`docs/step3-spec.md`, "The build, in stages", gains a stage between 2 and 3:

| Stage | What is built | What it shows |
| --- | --- | --- |
| 2b | Command levels: a picture for each command, changed only by what it sees and what is reported to it; reports up, down and between neighbours in radio reach, with silence; a commander's knowledge of his subordinate forces by report; commanders with initiative, and refusal | A battle of the slice in which a sighting reaches one command a pulse before another, and the two decide differently on it, each for a reason it can state |

Stage 2's doctrines are written as belonging to one level and reading only what that level has, from its step 3 on, so that stage 2b changes what they read and not what they are. Stages 3 to 5 follow as they were, and stage 3's reconnaissance is built on pictures that are already by level.

The plan for stage 2b is written by the architect and confirmed by the designer before it is built, as every stage's is.

## What it costs

As the design note has it: a picture for each command in the state, the hash and the saved game; a report as a thing in the state; a view for a level; every rule that reads belief reading its own level's; the chain of command and the commanders as data; and some figures of step 2 fitted again.
