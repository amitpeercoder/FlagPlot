# 0147: Reconnaissance for an operation: step 6 of stage 3, first part

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 6 of its nine, and its items 4 to 6, which the designer confirmed as the smallest useful part of `docs/design/intelligence.md`, "Reconnaissance as part of an operation"). Step 6 is written in two records: this one; and the next, cover for an operation by carriers. This one adds a rule to the engine. Nothing is built. It is built after decision 0146.

Summary: Step 6 of stage 3, first part: a count of the enemy ashore has a grade (told, seen from the air, or men in contact); an air group on the new reconnaissance mission flies to a place once a day and reports the men there, with a wider error than men in contact have; and "take a place" sends a group to look, chooses its units again on the first count, and is not launched until it holds the count its risk asks for, or says what it goes without

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Not built

## Why

A side learns the enemy's men in a hex only when its own men are there (decision 0092). A staff that plans a landing plans it blind: decision 0142 takes every unit the ships can carry and says it goes with no count. The designer's direction is that a commander who orders a place taken has said he needs to know what is on it.

## The grade of a count

1. **An estimate of the enemy's men in a hex has a grade,** kept with it and shown wherever the estimate is shown:

| Grade | Code | It comes from |
| --- | --- | --- |
| Told | 1 | A scenario's opening estimate: what the side was told before the game |
| Seen from the air | 2 | A reconnaissance flight (below) |
| Men in contact | 3 | The daily ground report of the side's own men in the hex (decision 0092) |

2. **A new report replaces the estimate,** whatever its grade, as now. The one case in which a worse report could replace a better one on the same ground is closed by item 6.
3. **It is state:** in the hash and named, in the saved game, and in a side's view of its own estimates. The enemy never sees it.

## The reconnaissance mission

4. **The mission.** An air group may be given the mission `reconnaissance` with a hex of the map, by the mission order. It is refused for a group whose type cannot search, as a search mission is.
5. **When it flies.** In the search phase of the morning pulse, once a day, where the group has an aircraft able to fly, a base or a deck to fly from by the rules of a search, and the hex within its type's search range. One aircraft flies. It costs the supply of one search sortie of twice the distance, and counts for the group's fatigue as one search sortie does.
6. **It does not fly** to a hex where its side has men of a fighting unit ashore: they see more than it would, and their report is the better one.
7. **What it reports.** The count is the true count of the enemy's men ashore in the hex, times an error, cut to a whole man. The error is drawn as a whole number of millionths between `ground.toml: reconnaissance_error_low` and `reconnaissance_error_high`, both included. The live values are 0.25 and 2, placeholders: wider than the 0.5 and 1.5 of men in contact. With no enemy man there the count is 0. The flight is made only if both figures are given: the frozen step 1 copy gives neither, so nothing built before this changes.
8. **The draw** is its own stream, keyed on the day, the morning pulse, the intelligence subsystem, the new draw `Reconnaissance`, added at the end, and the air group.
9. **What it does.** The report goes through `BeliefState::apply_report`, as every belief does: it makes or replaces the side's estimate for the hex, at the grade seen from the air, dated that day. It puts on the side's map every enemy base in the hex that is not on it, as a ground report does.
10. **The events.** `ReconnaissanceFlown`: the group, the hex, the distance and the supply spent, citing the mission order. `GroundReportReceived`, as now, with the grade, citing the flight; its breakdown (`true_men`, the two figures, the error) is the umpire's. `BaseFound` as now.
11. **Who sees it.** The side that flew sees both events whole but for the breakdown. The enemy sees nothing: being seen, and what the enemy makes of it, is left out.
12. **Nothing meets it.** The flight is not intercepted and no gun fires at it in this step.

## The plan's need to know

For a "take a place" whose plan is preparing.

13. **The count the risk asks for:**

| The risk | It may launch on |
| --- | --- |
| Preserve the force | An estimate of the place seen from the air or by men in contact, dated no more than 3 days before the morning |
| Balanced | An estimate seen from the air or by men in contact, of any day |
| Press | Anything, or nothing |

The 3 days are a placeholder.
14. **Who is sent.** On each morning the need is not met, and the side has no fighting man ashore in the place's hex, the staff sets one air group to reconnaissance of the place, where it is not set so already: the lowest-numbered group the intent's forces limit names, read up the tree, that can search, that the side may use and the commander does not hold, that no higher task has taken, and whose range reaches the place from where it is based. At press none is sent.
15. **When the need is met** the group is set to rest, by one order, and is the staff's to use again.
16. **The choice made again.** On the first morning the staff holds a count seen from the air or by men in contact, while the plan is preparing and no unit is aboard for it, the units are chosen again by decision 0142, items 8 to 10, on that count. Where the choice differs, the forces limit of the task "ready a landing" is set to it, and that task then orders the landing again with the new roster, at the cost the rules give a change (decision 0095).
17. **An estimate of 0** asks for one unit: the first in the order.
18. **Not launched without it.** The task is ready, for the leave and the release of decision 0142, item 15, only when the need is met as well.
19. **Where nothing can look,** because no group fits item 14: at preserve the force the plan is not released, and on the first morning the landing is otherwise ready the staff hands in the notice that it cannot meet the intent, with what it lacks, and gives the reason each morning after; at balanced it goes, and the reason of the release says that it goes on no count, or on what it was told.
20. **Reasons** give the estimate with its grade and its day, the count the risk asks for, the group sent with its range and the distance, and what the new count changed.

## What it leaves for later

As decision 0139 has it: patrols and men put ashore to watch; how hard a reconnaissance is pushed; being seen and what the enemy makes of it; harassment; coastwatchers. And a flight that is met by fighters or guns, more than one look a day, and a look at ships in a harbour.

## Rule card

- **Reads:** the truth of the enemy's men ashore in one hex, for the report only; the air group, its base and its type; the two figures.
- **Writes:** the grade on an estimate; one mission; the side's estimate, through a report; the base's supply and the group's fatigue by the rules of a search sortie.
- **Reuses:** the ground report and `apply_report`; the search mission's checks, range, supply and fatigue; `BaseFound`; the mission order.

## What it costs

- **State:** the grade of an estimate and one mission, in the hash and named. One draw, added at the end. The state layout, saved game, agent interface and doctrine versions rise, and the rules label rises: a game with the two figures and the mission plays differently.
- **Two figures** in `ground.toml`, live only.
- **The rulebook:** chapter 3 (the grade, the flight's report), chapter 5 (the mission), chapter 8 ("Taking a place": the need to know), the glossary.

## The hand calculation

`tests/scenario_72_reconnaissance.rs`.
