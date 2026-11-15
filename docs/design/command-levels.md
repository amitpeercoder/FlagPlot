# Design note: command levels, what each knows and what each reports

Status: a note written for discussion and since decided (decision 0119), written by the architect on Nov 15, 2026 at the designer's request, from his direction of that day. Nothing here is built. It says how a staff at every level of command could interpret the intent it is given and hand narrower intents down, each level acting on its own picture, and how reports pass between levels. It says what exists to build on, what is missing, and what the designer still has to decide. It follows decision 0116, which says intents form a tree and leaves the levels as a record on each intent.

## The designer's direction

Three things, in his words as the architect took them down:

- **A staff exists at every level of command.** A commander's intent goes to the level below, which interprets it and splits it among its own units, which do the same, down to the level that acts. The top general says "take Guadalcanal"; his staff splits it between the navy and the army; each of those splits it again; a rear admiral with a task force tells a carrier captain to find and destroy a carrier force believed on a bearing; and the captain decides how he does it, from his doctrine and his other orders.
- **Levels differ in what they know and in what they report.** That holds for the enemy and for a level's own forces: a commander knows his own forces as they have been reported to him.
- **Later, a player may choose a role.** He sees only what his level has, and carries out and splits the intents of the levels above him.

## The problem

Each side has one picture today. A contact seen by one flying boat is known in the same pulse to every ship and every group of its side, and a side knows each of its own units exactly: where it is, its fuel, its losses.

No commander of 1942 had that. The sighting of the Japanese cruiser force on 8 August reached the amphibious commander hours late, and the battle of Savo Island followed. Contact reports at the Eastern Solomons did not reach the carrier commander. A task force under radio silence was, to its own superior, where it was last heard from.

One picture also undoes the reason for the game's first pillar. A player who knows everything at once can run everything from the top, and the staff below him is a convenience. A player who learns of a contact after the captain has launched on it has to command by intent, because nothing else works.

## What exists to build on

- **The intent tree** (decisions 0116 and 0117): an intent has a parent, a maker and an addressee, the level it is given to. Limits pass down and narrow. A reason names the chain above it.
- **Belief changed only by reports** (`docs/engineering/truth-and-belief.md`): a side's picture changes only through `BeliefState::apply_report`. A report already holds an estimated position, count and kind, each of which can be wrong, and carries the event of its arrival.
- **A `SideView` for anything that plays:** nothing that plays the game reads the truth. The same wall would stand between levels.
- **Decisions made on the spot:** the choices a unit makes inside a day (to launch, to open fire, to break off) are made by standing rules where the unit is, and a side can ask them beforehand (decision 0100).
- **Claims:** what a side's own people believe they did, never corrected.
- **A side's events,** each with the rule that says who may see it (`side_events.rs`).

## The idea in one line

Every command has its own picture, changed only by what it sees and by what is reported to it, and every decision is made by the level that owns it, on that level's picture.

## The levels

The tree of command for the slice, as it would stand. The names are the game's, and the scenario's data says who is under whom.

| Level | Example | Commands | Decides |
| --- | --- | --- | --- |
| Theatre | Pacific Ocean Areas; Combined Fleet | Areas | What each area is given and asked for |
| Area | South Pacific; the command at Rabaul | Services and forces in the area | The ends of the campaign, and who gets what |
| Force | A task force; a base and its air; a division | Its ships, groups or units | How its task is done |
| Unit | A carrier and her group; a battalion | Itself | What it does at the point of contact |

A service level, navy beside army, sits between the area and the forces where a side has it. On the Japanese side it matters: the army and the navy on Guadalcanal held different pictures and reported up different chains.

## What a level knows

1. **Its own sight.** What its own searches, lookouts and fights showed, in the pulse they showed it.
2. **Its own forces, as it commands them directly:** exactly, for the units in its own hands. For anything a level down, as last reported.
3. **What was reported to it:** from below, the sightings, claims and state of its subordinates; from above, what its superior chose to pass down with an intent or after it; from beside, what a neighbour sent.
4. **Nothing else.** A level does not know what a neighbour saw until someone tells it.

So a picture has an age for everything in it, as a contact has today, and also for the level's own subordinate forces: a position as of the last report, losses as of the last report.

## What a level reports

5. **Upward:** what it saw, what it did, what it claims and the state it is in. A report takes time, may be garbled as a sighting may be, and may not arrive.
6. **Downward:** intents, with the part of the superior's picture that bears on them. "A carrier force is believed on this bearing" is knowledge handed down with a task.
7. **Sideways,** where the chain of command or a standing instruction provides for it: a base's search reports to the carriers it works with.
8. **Silence.** A force keeping radio silence reports nothing and may still receive. It is a setting of a force, set by doctrine and by risk, and it is a real choice: silence hides the force from the enemy's listening, which the intelligence note has as a layer still to come (`intelligence.md`), and it hides the force from its own superior.
9. **What is reported is belief.** A claim passes upward as it was made. A level that doubts it has no means to check it, as now.

## What a level decides

10. **A doctrine belongs to one level and reads only what that level has:** the intent it was given, the forces it commands, its picture, its doctrine and its other intents.
11. **A level hands down intents, not settings,** until the level that acts. Only that level turns an intent into orders. The area staff does not set a fighter group's patrol share; it tells the base to be held, and the base decides.
12. **A reason names the level that decided,** and what that level knew when it did.
13. **Initiative.** A level that sees what its superior cannot acts on it inside its intent and its limits, and reports what it did. This is what a carrier captain did when a contact appeared. What a level may do outside its intent, when what it sees makes the intent wrong, is the hardest rule here and is left open below.

The size of a strike (decision 0118) is the first case. The admiral gives the carrier a task; the captain decides how many aircraft go, from the task, from his doctrine and from what else he must keep aircraft for. Nothing above him sets it.

## The player and his seat

- **The area commander's seat,** the slice's own, shows the area level's picture: late, second-hand and aged. His own task forces are drawn where they were last reported.
- **Dropping down,** which the first pillar promises, is taking a lower seat: the player then sees that level's picture, fresher about what is near it and blind to the rest, and he carries out an intent from above.
- **A role chosen for a whole game** is the same thing held: a task force commander under a staff-played area commander, receiving intents and splitting them.
- **The umpire's view is unchanged:** the replay viewer and the validation harness show the truth.

## What is missing

- **A picture for each command** in place of one for each side: in the state, the hash and the saved game. Tens of pictures a side, most of them small.
- **A report between levels** as a thing in the state: what it says, who sent it, to whom, when it was sent and when it arrives.
- **A communications rule:** the delay of a report by who sends it and how, the chance it is lost or garbled, what silence does, and what a base's radio can reach. It needs research and figures of its own.
- **A view for a level,** as `SideView` is for a side, and the rule for each kind of event of which levels may see it.
- **Every rule that reads belief** reads its own level's picture: the strike decision, the search arcs, the staff's doctrines, the estimate of the enemy ashore.
- **The chain of command as data:** who is under whom, on which days.
- **The tuned battles.** Step 2's measures were fitted with one picture a side. Reports that arrive late change who strikes and when, and some figures will have to be fitted again. The battles' own orders are scripted, which limits the damage: a scripted order does not wait for a report.

## Rules it must keep

- **Determinism:** a report's delay and loss are draws from keyed streams, as a sighting's errors are.
- **Traceability:** a report is an event with causes; a decision's reason cites the reports its level held.
- **Truth and belief:** a picture changes only by a report applied to it. No level reads another level's picture, and none reads the truth.
- **One set of rules for both sides.** The Japanese chain of command differs in data, not in code.
- **Legibility:** a player must be able to see why his picture and a subordinate's differed when a thing was decided. If he cannot, the fog reads as a bug.

## Risks

- **The clock is coarse.** A day has three pulses, so a report is on time or a third of a day late. Savo's delay fits; a delay of an hour does not exist. A finer clock for reports alone is possible and costs more than it looks.
- **Cost in speed and memory.** Tens of pictures, each aged every pulse, where there were two. Stage 4 of step 3 plays many games forward; each would carry them all.
- **It may not be fun.** Not knowing where your own carriers are is true to the record and may feel like being robbed. The gate that judges whether commanding by intent is fun judges this too.
- **Legibility gets harder** before it gets better: a reason now has to say who knew what.
- **The first pillar is touched.** "Anything can be grabbed" meets "you see only what your level sees". Grabbing a force means sitting where its commander sits, and seeing less of everything else while you do.

## Order of arrival

1. **Now, in stage 2 of step 3:** every doctrine is written as belonging to one level and reading only what that level has (10 and 11 above). One program still does the work and one picture still serves a side, so nothing plays differently. It is what makes the rest possible without rewriting the doctrines.
2. **A stage of its own after stage 2,** in three parts, each with a decision record and a hand calculation first:
   - a unit acts on what it saw this pulse, and the rest of its side learns of it only when it is reported: the enemy picture by level;
   - a level's knowledge of its own subordinate forces by report, with silence;
   - initiative: what a level may do when what it sees and what it was told disagree.
3. **A seat at any level,** with the staff playing the levels above and below: after the kill gate, with the interface.

## Decided by the designer, Nov 15, 2026

- A staff interprets and splits intent at every level, down to the level that acts.
- Levels differ in what they know and in what they report.
- A level's knowledge of its own forces also depends on reports.
- A player may later choose a role and see only what its level has.

## The seven questions, answered

Put to the designer one at a time on Nov 15, 2026. His answers are decision 0119.

1. **Initiative:** a rating of each commander, with placeholder values until research; a commander may even refuse an order, at a cost to himself.
2. **How stale your own forces may be:** any force a level down, as of its last report.
3. **What goes down with an intent:** what bears on it.
4. **Neighbours:** always hear each other within radio reach.
5. **The player's default seat:** the area's picture, with a setting for one picture a side.
6. **The clock:** the three pulses.
7. **Where it comes:** after stage 2 of step 3 and before stage 3, as stage 2b.
