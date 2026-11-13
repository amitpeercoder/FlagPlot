# 0047: What a side may know of what happened

Status: confirmed. The designer approved the plan for the agent interface on Nov 11, 2026, with this rule as its first choice. The table of event kinds is the architect's working out of it. Amended by decision 0065 (no enemy unit named) and decision 0085 (a fight's ending and start as the side knew them, the rounds counted from there, hidden causes flagged by kind).

## The gap

A side's belief is its own units, exactly, and the enemy as contacts (decision 0012). That is state. The spec also asks that a player can be told what happened and ask why, and the only record of what happened is the event log, which is the truth: it holds the enemy's orders, his rolls, his hits and his losses. Until now only the umpire read it. An agent or a person playing a side must be shown something, and it cannot be that.

## Decisions

### The rule

1. **A side sees everything that happens to its own units,** with its causes: orders taken and refused, movement, fuel, cargo, wear, repair, fatigue, rest, arrivals, replacements, supply spent, and every loss and hit it takes.
2. **A side sees every report, contact and claim made to it,** and what becomes of its contacts.
3. **Of a fight it took part in, a side sees what its own people did and what was done to them.** It sees that it fired, at what and with how many shots. It sees that its ships were fired on and by whom, and the hits its own ships, aircraft and bases took. It does not see the hits it scored, the enemy's rolls, or the enemy's damage and losses. What it learns of those is what the rules already give it: a claim or a report.
4. **A side does not see a search that found nothing, a force that looked and did not sight, or an alarm its force did not hear.** Each would say that something was there to be found. The search that sights, the look that sights and the alarm that is heard are the side's own and are shown whole.
5. **A side sees a surface fight from the round in which its force first sights the enemy or is fired on.** Before that it does not know there is a fight.
6. **Everything else is hidden:** the enemy's orders, movements, stocks, fatigue, repairs and decisions, and every event of a fight the side had no part in.

### How it is given

7. **A side's view gives the events it may see, each cut down to what it may know.** An event is whole, or has fields removed, or is absent. Nothing is added or altered. The cut event names its number in the full log, so two sides and the umpire can talk about the same event.
8. **An enemy unit in an event a side may see is named as the log names it.** A side that fought a ship knows which ship fired at it for the length of that fight. Nothing else of that ship is shown. Replaced by decision 0065: no event a side may see names an enemy ship, task force or air group.
9. **The causes of an event are cut the same way.** A cause the side may not see is dropped, and the event says that it has causes the side cannot see, so that "why" is answered honestly: this much is known, and there is more.
10. **Breakdowns follow the event.** A side sees the breakdown of an event it sees whole. A cut event carries only the factors that are the side's own. Amended by decision 0060: a claim is seen whole and its breakdown is not shown, because it says how much of the claim is real.
11. **It is the only door.** Anything that plays the game takes its events from the side's view. The full log stays with the umpire: the runner, the tests and the replay viewer.

### The kinds

| What a side sees | Kinds of event |
| --- | --- |
| Whole, if the unit is its own | Order accepted, order rejected (the side that gave it); task force steamed, turned back, arrived, out of fuel; fuel burned; ship refuelled; cargo loaded, unloaded, loading complete, unloading complete; search flown; report received, claim received; contact aged, dropped, not there; force alerted by contact, became alert, stood down; ships detached, force retired; force fatigue; aircraft worn, repaired, arrived; replacements received; air group lost, stood down, returned, fatigue; strike launched, base strike launched, strike found nothing; escorts sent; aircraft lost; pilots lost; aircraft destroyed on the ground, damaged aircraft destroyed on the ground; base supply destroyed, base supply spent; ship damaged, system hit, ship sunk; force broke off, slipped away, surprised (the force it happened to) |
| Whole to both sides, once each knows of the fight | Engagement begun, engagement ended |
| Whole to its own side if it succeeded; nothing if it did not | Sighting roll; search attempt (cut since decision 0061: without the distance and the chance); alarm roll |
| The firer: its own firing, without the chance, the expected hits or the hits scored | Salvo; air attack; air fight; anti-aircraft fire, a base's anti-aircraft fire |
| The target: that it was fired on. For a salvo the round, the firing ship, the target and the weapon. For an air attack the air group, the target and the aircraft attacking | Salvo; air attack |
| The attacker: that it attacked, without the hits. The defender: whole | Base attacked |
| Nothing | Forces passed unseen |

Since decision 0065 the table is narrower in one way throughout: no row shows a side an enemy ship, task force or air group. Engagement begun and ended, a sighting roll, force surprised, force slipped away, an alarm roll and the defender's base attacked are cut of the enemy's name and nothing else, and each firing kind loses the enemy unit it named. `docs/rules/01` has the table as it stands.

A side knows of a surface fight from the earliest of: its own force sighting, a salvo at its ships or its own salvo, a hit on its ships, its force being surprised, or its force slipping away. It all comes from the log; no state is kept for it.

## Why

- **The approved rule, and its alternative.** A stricter rule, the briefing and the side's own state and nothing of a fight, is safer and leaves "why did I lose that ship" without an answer. This one answers it from the loser's side of the fight and no further.
- **No hits scored.** The game already has a way for a side to learn what it did to the enemy: the claim after a surface fight and the report after a strike, both of which can be wrong (decisions 0014 and 0026). Showing the true hits beside them would make the claims pointless and the fog a formality.
- **A cut event, not a rewritten one.** If the side's version of an event were a different sentence written for it, two descriptions of one event would have to be kept true together. Removing fields cannot make a false statement.

## What it costs

- **Fighters get no word of what they shot down.** There is no claim for aircraft yet. A side sees its own losses in an air fight and nothing of the enemy's. An air claim is the remedy and is deferred.
- **A side knows the log's numbers for enemy ships it fought.** It could tell that the cruiser of Tuesday is the cruiser of Thursday. Contacts are meant to hide that. It is accepted for the length of step 1. Ended by decision 0065.
- **The log's numbers are not shown to a player.** Gaps in them would tell a side that something happened it cannot see, and a hidden event of the enemy's shifts every later number. So what a player is shown numbers a side's events 1, 2, 3 in the side's own order (decision 0048, 14). The log's number stays on the event for the umpire.

- **A force can break off from a fight its side never knew of.** A force over its break-off level breaks off at the end of a round whatever happened in it (decision 0014). If its ships were crippled before the meeting and it neither sights nor is fired on, its own side is not shown the break-off: showing it would say an enemy was there. The force's state still changes. The two rules pull against each other and decision 5 is given the last word.
- **The numbers of forces made during a game leak a count.** A detached force takes the next free number, and the enemy's forces use the same numbers. So a side's new force has a number that depends on how many forces the enemy has, and because random draws are keyed on a force's number, an enemy force that does nothing changes the luck of forces made later. A test shows it and is kept, switched off. The remedy is to number the forces each side makes from its own range. It changes the hashes of every scenario with a detachment, and is left for now.

## Deferred

- Claims for aircraft shot down and for damage done to a base.
- Naming enemy units in fights by contact, not by the log's number. (Decision 0065 removes the number; naming by contact is still not made.)
- What a side may learn later: a prisoner, a wreck, a decoded signal.

## Rule card

| | |
| --- | --- |
| Reads | The event log; which side each unit belongs to; for a surface fight, the round in which each force first sighted or was fired on |
| Writes | Nothing |
| Events | None |
| Randomness | None |
