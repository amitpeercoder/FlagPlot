# 0197: Men who replace the lost

Status: a plan by the architect, written on Nov 20, 2026, on the designer's word (his queue of that day: "replacements for land units"). Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ) before any of it is built.

Summary: A land unit's men only ever fall. Nothing in play adds a man. Over 39 days that is bearable; over a campaign of months every unit wastes away. This lets a scenario give a base a stream of replacement men, so many a day, who join the units ashore there that are under strength and not in a fight, and who bring the unit's experience down toward a recruit's.

Standing: Planned on Nov 20, 2026 and amended on review; not yet built. The designer's answer is taken.

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

## Confirmed on Nov 20, 2026

The designer: "Go with your recommendations." **Replacements cost a unit above the standard level one level** once they are half its establishment (rule 12).
