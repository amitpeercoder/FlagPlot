# 0197: Men who replace the lost

Status: a plan by the architect, written on Nov 20, 2026, on the designer's word (his queue of that day: "replacements for land units"). Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ) before any of it is built.

Summary: A land unit's men only ever fall. Nothing in play adds a man. Over 39 days that is bearable; over a campaign of months every unit wastes away. This lets a scenario give a base a stream of replacement men, so many a day, who join the units ashore there that are under strength and not in a fight, and who bring the unit's experience down toward a recruit's.

Standing: Planned on Nov 20, 2026 and amended on review; built on Nov 21, 2026 (scenario 113), awaiting the architect's reading of the build.

## Why

`docs/design/larger-scenarios.md`, and decision 0102, which deferred "replacements for land units" until the tuning of the ground figures needed them.

What is so today:

- **"No arrivals or replacements of men by schedule. A scenario's units are those it starts with"** (chapter 7). Men ashore rise only when ships land them.
- **Aircraft have a stream** (`[[replacement]]`): so many a day to a base, to the groups there of a type, the lowest number first, never above a group's strength, and what is not used that day is lost.
- **A land unit has an experience** from 1 to 5 and skills (decisions 0097, 0098). "Nothing in play ... lowers its experience."
- **A unit has no strength on file** but the men it has: there is no figure for what it should have.

## The rules

1. **A unit's establishment** is the men it has when the game begins, ashore and aboard together, or the figure the scenario gives it by a new key, `establishment`, which may not be below its men. A unit made by division takes the men it is made with as its establishment, and the unit it left has its own lowered by as many. A unit that enters (decision 0195) has the men it enters with.
2. **A new table of a scenario, `[[men_replacement]]`:** a side, a base, men a day (a whole number above 0), and optionally a first day and a last day.
3. **Each night,** in the pulse where replacements of aircraft are received, from day 2, a stream whose days hold the day and whose base is its side's gives its men to the land units of its side that are ashore in the base's hex, that have men, that are below their establishment, and that are not in a fight: not on attack, and not attacked in the day before. They are served in number order, each up to its establishment, until the men are gone. Men not given that night are lost.
4. **They are fed like any men** from that night on, and they count in the unit's men for every rule.
5. **Experience falls toward a recruit's.** A unit's experience after replacements is its experience before times its men before, plus the recruit's experience times the men who joined, over its men after, as an exact fraction; the game's rules read it cut to a whole level as they do today. The recruit's experience is a figure of the ground file, a placeholder: 1.
6. **Skills and fatigue are the unit's,** unchanged.
7. **A unit destroyed is not rebuilt:** one with no men gets none.
8. **An event,** `MenJoined`, with the unit, the base, the men, the unit's men after and its experience before and after, seen whole by the unit's side alone.
9. **A scenario with no stream plays as before.**

## Rule card

- **Reads:** the streams; the day; the holder of a base; the units ashore in its hex, their men, establishment, posture and whether they were attacked.
- **Writes:** a unit's men and its experience.
- **Reuses:** the step and the serving order of aircraft replacements; a unit's experience and how the rules read it.
- **New:** one table of a scenario and its state; a unit's establishment; one figure of the ground file; one event. No draw.

## What it costs

- **State and hash:** the streams and each unit's establishment are state; a unit's experience becomes an exact figure where it was a whole level; the hash version rises and the golden files are remade.
- **A unit's experience as a fraction** is read by every rule that reads experience today; each is checked to give what it gave for a whole level.
- **Rulebook:** chapter 7 and the glossary; the scenario file's keys; the agent's briefing and a unit's line.
- **No figure is on file** for how many men reached either side as replacements: a scenario's streams are placeholders, marked.

## Questions for the designer

1. **Experience falls with replacements.** The architect's choice, since green men in a veteran unit is the cost of a long campaign. The simpler rule leaves experience as it is.

## Amended on review

A reviewer read the plan against the rules of a land unit and the code. He found the plan's handling of experience wrong in three ways, and it is replaced. Found right: that nothing adds men in play today; how aircraft streams are served; the sums of the worked example as first written; that the new men are fed on the night they join.

### Experience, replaced (rules 5 and 6, and the question)

10. **The scale was wrong.** A unit's experience is a whole level from 0 to 4, with 2 the standard, 0 raw, and 2 the most that training gives. Every real unit is at 2 (decision 0097). A recruit at 1 would have dropped every real unit a level with its first man.
11. **No fraction.** The engine's numbers are millionths and cut, so "an exact fraction" cannot be held, and cutting a fraction to a level is a cliff: one recruit in a thousand veterans would cost the unit a level. Experience stays a whole level, read by every rule as it is today.
12. **The rule:** a unit keeps a count of the men who have joined it since its level last changed. Replacements are trained men: their level is the standard, 2. A unit at 2 or below is not changed by them. A unit above 2 loses one level on the night the count reaches half its establishment, and the count begins again from nothing. It never falls below 2 by this rule. The count also begins again when the unit gains a level by fighting or training.
13. **Decision 0097, item 6, "men who join a unit do not dilute it", is superseded** by rule 12, by name.
14. **The two figures** (the replacements' level, 2, and the share of the establishment, a half) are figures of the ground file, placeholders. A ground file that leaves them out is read with those two, so that no existing file is changed.

### The rest

15. **In a fight, defined.** A unit keeps the last day on which it took part in a fight ashore, attacking or defending. It is "in a fight" on a night when it is on attack, or that day is the day before. Bombs and shells are not a fight. A unit on a march or falling back takes none; a working unit takes them.
16. **The order of a day, corrected.** The fight ashore is at the end of a day, after that day's night. So a unit attacked on day 1 takes none on the night of day 2, and a base taken at the end of day 2 gives none from the night of day 3. The worked example is restated by the architect.
17. **Below its establishment** counts the unit's men ashore and aboard together, so that a unit with men still in a ship is not filled ashore and then landed above its strength.
18. **Division.** A unit is divided only where its men aboard land in another hex than its men ashore; nothing orders it. The new unit's establishment is the men it is made with and the unit it left is lowered by as many, so the whole of any shortfall stays with the unit it left. The count of rule 12 stays with the unit it left; the new unit begins at nothing.
19. **An establishment given by a scenario is a figure of history** where the unit is a real one: it rests on a row, or is marked a placeholder. With no key a unit that began short has nothing to be made up to.
20. **Decision 0193 is untouched:** replacements reach units at a base of their own side, and the place of a take is not that.
21. **New state, all of it in the hash:** the streams; and for each unit its establishment, its count of men joined, and the last day it fought. No reader of experience changes its type.
22. **Built last of the four,** after decisions 0194, 0195 and 0196, since all change the same files; a unit that enters (decision 0195) has the men it enters with as its establishment.

### The question for the designer, restated

Replacements above the standard level cost a veteran unit a level once they are half its strength. The architect proposes it. The simpler rule leaves experience alone, as decision 0097 had it.

## As built

Built on Nov 21, 2026, on the branch `men-who-replace-the-lost`, on decision 0196's build, to the plan as amended on review. No figure changed. Every scenario under `data/scenarios` (164 files, the frozen copies and `tmp-inside` apart) was played before and after with `sim-cli run --events` and `sim-cli hash`: no event and no daily hash moved. No existing scenario is given a stream or an establishment.

### What was built

- **The state** (`crates/sim-core/src/state.rs`). `TruthState::men_replacement`, a `MenReplacement` of the `streams` (each a `MenStream`: side, base, `men_per_day`, `first_day`, `last_day`, kept in one order) and `units`, a map from each land unit to its `Strength` (`establishment`, `joined`, `last_fought`). It is filled only where the scenario has a stream or names an establishment, and then for every unit, those held apart until their day among them. It is hashed last, after the supply lines, and only where it is not empty, each field named in `hash::name_every_field`; the state's layout (`flagplot-state-v51`) is not raised and no golden file was remade, since a game without the rule hashes as before.
- **The night step** (`crates/sim-core/src/replacement_men.rs`, `resolve.rs`). `MenReplacement`, its own step right after the supply lines and before aircraft upkeep and the men's supply, with its moment for a watcher of the day. It draws no random number.
- **A fight, noted** (`ground_combat/fight.rs`): every unit in a day's fight (the units that share its losses, on either side) has its last day fought set. **Division** (`cargo.rs`): the new unit's establishment is its men, the parent's is lowered by as many, the parent keeps its count and day fought. **Experience raised** by training or fighting begins the count again (`ground_combat/activity.rs`).
- **The event** (`events.rs`, `side_events.rs`). `MenJoined { unit, side, base, men, men_after, experience_before, experience_after, joined }`, one for each unit that takes men, with no cause and a breakdown, seen whole by the unit's side and by nobody else.
- **The figures** (`refdata.rs`, `sim-data/src/reference.rs`). `replacement_experience` (2) and `replacement_share` (0.5), optional keys of the ground file, placeholders; neither file in the repository names them.
- **The invariants**: `MenStreamUnsound` and `StrengthUnsound` (a strength kept for a unit neither in the state nor held apart, or a unit with more men ashore and aboard than its establishment).
- **The view and the halves** (`view.rs`, `view/seat_view.rs`, `world_picture/half.rs`). `SideView::men_streams` (the side's own; from a seat at a base, that base's) and `SideView::strength` (its own units', held ones among them). A side's half holds its own streams and its own units' strengths; the halves join them.
- **The loader** (`crates/sim-data/src/scenario.rs`). `[[men_replacement]]` with `base`, `men`, `first_day`, `last_day` and `side`, and a land unit's `establishment`. Refused: an unknown base, a stream of no men, a first day below 1, a first or last day past the game's last, a last day before the first, and an establishment below the unit's men.
- **The agent** (interface version 38): the briefing's `arrivals.men_replacements` and `land_units.men_joined_yesterday`, a land unit's `establishment` where one is kept, and their lines in the text briefing. The schemas are updated.
- **The umpire's readers** (`sim-cli`: the export's `men_joined` and the batch record) and **the trial** (`sim-trial/src/enemy.rs`: the enemy's made half holds no stream and no strength).
- **Scenario 113** (`data/scenarios/men-who-replace-the-lost/113_men_who_replace_the_lost.toml`) and its tests (`tests/scenario_113_men_who_replace_the_lost.rs`, 22 pass; and `crates/sim-core/src/replacement_men_tests.rs`, `crates/sim-agent/tests/men_replacements.rs`).
- **Versions.** The rules label stays 25, the state's layout v51 and the saved game's version 23. The doctrine version stays `supply-49`: the staff is not changed. The agent interface is 38 (from 37); the recorded games' transcripts were remade for that label alone and are otherwise byte for byte what they were; no save changed.

### Readings taken

a. **Where the state is kept.** The establishment, the count and the day fought are kept in a table of their own in the state, not as fields of a land unit, so that a game without the rule holds and hashes nothing of it and no unit's hash moves. It is kept for every unit where the scenario has a stream or names any establishment, and for none otherwise.
b. **"In a fight"** is read as the units of the side in a day's fight in their hex, those that share the fight's losses (correction 1 of decision 0109), attacking or defending; a unit falling back that is out of the fight is not in it. A working unit in the hex is among them.
c. **"On a march or falling back takes none"** is read as a unit with any march standing, by order or by falling back.
d. **The count on a fall begins from nothing**: men over the share that night are not carried over (statement 3a's count of 100 on the night after the fall reads so). The event's `joined` is the count after the night, 0 on the night of a fall.
e. **A unit at or below the replacements' level** keeps counting the men who join it, though they never cost it a level; a rise by training or fighting begins the count again.
f. **The share is compared exactly**: the men joined against the establishment times the share, in fixed point, with no cut.
g. **A stream's first day** is 1 where the file gives none, as a supply line's is; nothing is given on day 1 whatever the stream says, as aircraft replacements begin on day 2.
h. **The step is in every night's order,** so a watcher of the day sees one more moment each night, even in a game with no stream. No output and no hash moved.
i. **The staff is not changed.** The men who join raise a base's daily charge, and the staff's days of supply read the men there today, so its forecast at such a base is low by the stream's men for the days ahead. That is a gap of the staff's, as a supply line's fuel is in decision 0196; the doctrine version is not raised.
j. **Statements 6 (a unit destroyed) and 6a (men aboard)** are proved in sim-core, in a world built by hand, since no scenario file can give a unit no men, or men both ashore and aboard at once. The scenario test of statement 5a finds the first seed on which day 1's shells kill men of unit 1; statement 9 is checked with all three units (1,250 men fed) and with unit 1 alone (500 men, 2.5 t).
k. **The briefing** lists the side's streams whose last day is not before today, a line for each unit that took men yesterday, and a unit's establishment only where the rule is kept, so no existing answer changes but for the version.

No statement of the hand calculation failed to hold; none is ignored.

### Not done

Replacements that arrive in ships and can be sunk, new units by stream, and units rebuilt from nothing are not built. No figure is researched: every stream is a placeholder, and no scenario of history is given one.
