# 0186: Men put ashore score

Status: a plan by the architect, written on Nov 20, 2026, at the designer's asking ("Yes, plan the men ashore objective"). Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ) before any of it is built. It amends decision 0184, rule 13 ("No 'men landed' kind is needed").

Summary: The game scores the supply a side lands at a base and has no objective for the men it lands there. A convoy whose whole purpose is to put 1,500 men ashore scores for its 90 t of supply alone. This adds one kind of objective, men landed at a named base, counted as supply landed is counted and worth points for each hundred men as men lost are. Scenario 103 gives it to the Japanese side at the worth of a man lost, a placeholder for the designer to see.

Standing: Planned on Nov 20, 2026 and amended on review; not yet built.

## Why

Scenario 103, the Eastern Solomons over five days (decision 0184). In two runs of 1,000 games of staff against staff the Japanese convoy landed its 1,500 men at Taivu in 1,000 and in 997 games, and that scored 0.45 points each time, the points of its 90 t of supply. The Japanese side won 26 and then 5 games. Decision 0184's rulings, 5, and "The second version of the orders" put it to the designer whether men put ashore should score. He said to plan it.

What is so today, read from the code:

- **Supply landed** (`cargo_landed`, decision 0067) names a base. It counts the tons in the `CargoUnloaded` events of the side's own ships at the base, less the tons in their `CargoLoaded` events there, never below 0. A side knows its own count exactly.
- **Men landed** make an event, `MenLanded` (decision 0066): the unit, the ship, the hex, the men. Men taken off the shore make `MenEmbarked`, with the same parts. Each is seen whole by the side whose men they are.
- **Men lost** (`men_lost`, decision 0090, 26) scores for each 100 enemy men lost, a part of 100 scoring its part.
- **Objectives are configuration.** They are not in the state hash, no rule of the simulation reads them and no staff reads them.

## The rules

1. **A new kind of objective, men landed** (`men_landed`). It names a base and gives points for each 100 men. It is added at the end of the kinds.
2. **Its count** is the men in the `MenLanded` events of the side's own ships in the hex of the named base, less the men in the `MenEmbarked` events of the side's own ships in that hex, never below 0. Men put ashore and taken off again were not landed. A side's own ship is one whose side is the side's, as supply landed reads it.
3. **The hex is the base's, whoever holds the base.** A base does not move. Men landed in the hex of a base the enemy holds count, and so do men landed before or after the base changes side.
4. **Men who land and are then lost still count as landed.** The enemy is scored for them under men lost if he has that objective. Landing a man and losing him are two things.
5. **Men who were ashore in the hex at the start and embark there** take the count below what was landed; it stops at 0.
6. **Its points** are the points for each 100 men times the men over 100, a part of 100 scoring its part: the sum men lost uses. The scenario's key is `points_per_100_men`, as it is for men lost.
7. **What a side knows of its own progress** is exact: both events are its own and it sees them whole.
8. **It is shown** as every objective is: in the side's briefing, in the enemy's objectives shown to the other side (decision 0183), in the result at the end, in a trial's points (decision 0181) and on the table's end screen. The words: "Objective: men you land from ships at BASE and do not take off again, points for each hundred men: N", and for the other side "men the enemy lands from ships at BASE and does not take off again, points for each hundred men: N".
9. **Nothing else changes.** No staff acts on it; a commander reads it and gives his orders.

## Rule card

- **Reads:** the `MenLanded` and `MenEmbarked` events, the side of each event's ship, the hex of the named base.
- **Writes:** nothing in the state. One more count in the tally of objectives.
- **Reuses:** the tally and the net count of supply landed; the sum of points for each 100 men of men lost; the briefing's objective line with its base and its points.
- **New:** one kind of objective and one count. No state, no event, no draw, no hash change.

## Scenario 103

10. **The Japanese side gains** men landed at Taivu (base 5), 1 point for each 100 men, a placeholder. The Allied side gains nothing. The base of the scenario is changed and its four files made again.
11. **Why 1.** The game's one scale gives 1 point for each 100 enemy men lost. A man put ashore is given the worth of a man lost: what a side gains by landing him, the enemy regains by destroying him. It is not fitted to a run.
12. **What that does, said before it is built, from the second run's counts** (nothing replayed: each game's Japanese points are raised by its men landed over 100, since no game changes). The 1,500 men are worth 15 points, more than a carrier sunk. Of the 1,000 games the Allied side would win 483, the Japanese 249, with 268 stalemates, where it was 963, 5 and 32; the middle Allied margin 2.51, where it was 17.22. At half a point for each 100 men it would be 797, 59 and 144. The architect has seen these before choosing, and says so: the choice rests on rule 11 and the designer may set another figure.
13. **The game on the scenario's seed is the same game:** every daily hash of file a on seed 1942 is as it was. Its result gains the objective.

## What it costs

- **A rule added:** the rulebook's chapter 6 gains the kind in both its tables, and the glossary gains "men landed".
- **Every reader of an objective's kind learns one more:** the scenario loader, the briefing's schema and its README, the briefing's text for both sides, the table's end screen, the viewer if it names kinds. The agent interface's version rises.
- **Briefings of scenario 103 gain a line;** no other scenario's change.
- **No recorded game changes.** Scenario 103's two run reports stand as the record of the scale without the objective.

## What is built

- `sim-core`: the kind, its count in the tally, its score and what a side knows of it.
- `sim-data`: the kind in a scenario file, with the checks men lost and supply landed have (the base exists; the points are not below 0).
- `sim-agent`: the kind in the briefing's data and text, for a side and for its enemy's objectives; the schemas.
- The table's end screen and anything else that names the kinds.
- Scenario 103's base, its files, and its test of the objectives.
- `tests/scenario_104_men_put_ashore.rs`, whose hand calculation the architect writes first.
- A third run of 1,000 games is not needed for the points (rule 12's sum is exact); the helper that reads a run is made to read the new objective, and the sum is checked against it on the kept seeds.

## Questions for the designer

1. **The figure:** 1 point for each 100 men in scenario 103, or another.
2. **"Lunga shut"** stays at 10 points for the Japanese side, got in 8 games of 1,000. This plan leaves it.
3. **The 39 days** could give each side men landed too. This plan does not: its points are a placeholder scale set from the record later (step 3, stage 5).

## Amended on review

A reviewer who had not seen the plan written read it against the code and ran the four files of scenario 103. Eleven findings; the architect's ruling on each is here. The arithmetic of the statements, the claims of "what is so today", the visibility of the events and "no hash change" were checked and found right.

### What was wrong

14. **A trial gives no points** (decision 0181, rule 7). Rule 8's "in a trial's points" is struck.
15. **The count could be raised by landing the same men twice:** land in the hex, march to another coast hex, embark there, land again. Rule 2 and rule 5 are replaced:
    - **The count is kept unit by unit.** For each land unit, a number that starts at 0. Reading the events in order: men of the unit landed in the base's hex by a ship of the side raise it by those men; men of the unit embarked by a ship of the side, in any hex, lower it by those men, and not below 0. The objective's count is the sum over the units.
    - So men landed, marched off and taken aboard elsewhere were not landed, and landing them again brings the count back to what it was and no higher. Men who embark before their unit has landed anyone (at the port they sail from, or from the hex itself where they began) lower nothing. Men of another unit who march into the hex and embark there lower nothing.
    - A unit divided aboard (decision 0066, 14) is a new unit from then on, with a number of its own starting at 0.
16. **"Counted as supply landed is counted" was loose.** Supply landed is keyed on the base its event names, which is the landing side's own base in the hex. Men landed is keyed on the hex of the base named and on the unit. The summary's and the rule card's words are to be read so.
17. **The Allied line does not name Taivu.** Taivu (base 5) is not on the map, so the Allied seat has no name for it and the line an Allied reader gets today for supply landed there says "base:5". Decision 0183, rule 4, says a base not on the side's map "is named by its place alone", and its amended rule 3 says the case cannot arise. It arises in scenario 103. The build makes the enemy's objective name such a base by its hex, "(3,-4)", for supply landed and men landed alike, and the rulebook's chapter 6 and `docs/rules/differences.md` say so. That a Japanese stock exists there is told to the Allied side by the scoring, which decision 0183 allows: where an objective is, is part of the victory conditions.

### What was missing

18. **This plan reverses two lines of decision 0184** and says so: its rule 13 (no such kind is needed) and its fixed list of kinds. Decision 0184 gains a line pointing here. That scenario 103 now has a kind the common scale has not is settled by decision 0187: it is a scenario for play, with a scale of its own.
19. **Places where a kind left out fails with no error,** each named for the build:
    - `Objective::name()` and `Objective::base()` in `sim-core` (the second ends in a catch-all: a kind left out of it is never checked for its base, by `World::set_objectives`, and carries no base in its lines);
    - the two matches on a kind's name in the briefing's text, which end in catch-alls that write any unknown kind as a base open: the build makes both name every kind, with no catch-all;
    - the table's end screen, which has a default;
    - the three lists of kinds in the briefing's schema and the description beside them; the schema's README;
    - the doc comment at the head of `sim-core`'s objectives;
    - the rulebook's chapter 6: the heading "The eight kinds", both tables, a line beside supply landed's on what is not landed, and the list in "What a side sees of the enemy's objectives";
    - the table of objectives in scenario 103's hand calculation, which the architect changes.
20. **Where the checks are.** That the base exists is checked in `sim-core` when the objectives are set; that the points are not below 0, in `sim-data`.
21. **Property tests** (`docs/engineering/rule-complexity.md`) are added to the statements: a further landing in the hex never lowers the count; an embarking never raises it; the points never fall with men landed and never rise with men embarked; nothing the other side's ships do changes it. And two statements: men landed who march away still count, and men of another unit who marched in and embark do not lower it.
22. **Rule 12's sum is exact only if no Japanese man was embarked in any of the 1,000 games.** The run's helper counted landings alone. The build makes the helper give the new count and checks it against the landings on the kept seeds; if any game differs, the figures of rule 12 are corrected from it by the architect.

### What the figure means, owned

23. **The worth of 1 for each 100 men is the placeholder of men lost used again, and not derived from anything.** Its reason holds in the margin only: men landed and then destroyed leave the margin where never sailing would. With it:
    - one convoy is worth about 30 points of margin between sunk on the way (the Allied side scores 15 for the men) and landed (the Japanese side scores 15), against levels of 3, 8 and 16;
    - 1,500 men who land and then sit unfed score more than a carrier sunk;
    - men who go on to take a base also score for the base and for the enemy's men lost.
    These are accepted for scenario 103 as a scenario for play, under decision 0187's guards, and are why the kind is not given to the 39 days by this plan.
24. **Showing what the figure does at 1 and at a half before choosing it** is the designer's to allow, and he has: decision 0187 records the scale as scenario 103's own.

### The questions, as they now stand

- The figure is set at 1 under decision 0187 and may be changed by the designer.
- "Lunga shut" stays at 10 points.
- The 39 days is not given the kind.
