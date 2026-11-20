# 0197: Men who replace the lost

Status: a plan by the architect, written on Nov 20, 2026, on the designer's word (his queue of that day: "replacements for land units"). To be read by a reviewer who has not seen it written, and amended on that reading, before any of it is built.

Summary: A land unit's men only ever fall. Nothing in play adds a man. Over 39 days that is bearable; over a campaign of months every unit wastes away. This lets a scenario give a base a stream of replacement men, so many a day, who join the units ashore there that are under strength and not in a fight, and who bring the unit's experience down toward a recruit's.

Standing: Planned on Nov 20, 2026; waits for its review.

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
