# 0061: A search that sighted is shown without its distance or its chance

Status: decisions 1 to 4 are confirmed. The designer approved the change on Nov 12, 2026. It amends the table of kinds in decision 0047, and follows decision 0060. The question at the end needs review.

## The gap

A sighting report can name the wrong hex: the position error moves it to a neighbouring hex (decision 0012). The side is meant to hold only the report.

But a side saw its own `SearchAttempt` whole when it sighted (decision 0047). That event carries `distance_nm`, the straight distance from the searcher to where the force really was, and `chance`, which is worked from that distance. Its breakdown listed the distance again. The player was shown "air_group:7 searched for task_force:4 at 200 nm, chance 0.477464: sighted".

- The searcher's own position is known to its side, so the true distance draws a ring the force is on. A report that names a hex off that ring is shown to be wrong, and the ring narrows which neighbour is right.
- Hiding the distance alone does nothing. The chance is `57.295779 x aircraft x sweep / (arc x distance)`, and the side knows every term but the distance.

Row 40 of `docs/rules/differences.md` recorded it when decision 0060 was made. It named only the breakdown's factor; the event's own fields give the same away.

## Decisions

1. **A search that sighted is a cut event for the searching side.** It keeps the air group and the task force. It loses the distance, the chance and the flag `sighted`, which is always true for an event the side is given.
2. **Its breakdown keeps the side's own factors:** `aircraft`, `sweep_width_nm` and `arc_deg`. It loses `distance_nm`, and `degrees_per_radian`, which is a constant of the rule. It has no result, as no cut event has: the result is the chance.
3. **A search that did not sight is shown to nobody,** as before.
4. **The log keeps the whole event** for the umpire's tools, as before.

## Why

- It is the same reasoning as decision 0060. An explanation a side is owed stops at what the side could know. Its aircraft flew this arc in this number and found something; where exactly is what the report says, right or wrong.
- A cut event is the mechanism decision 0047 already has for "the side's own act, without the part that is the enemy's truth". Nothing new is built.

## What it costs

- A player can no longer read from the game how likely a search was to find what it found. The rule and the figures are in the rulebook and the data files; the side knows its own aircraft and arc, and can work the chance for any distance it supposes.
- The legibility test of decision 0051 read this breakdown too ("aircraft 15, sweep 20 nm, arc 180, distance 200"). The first three are still shown.

## Rule card

| | |
| --- | --- |
| Reads | The log's `SearchAttempt` event and the side asking |
| Writes | Nothing. No state, no event, no random draw, no change to the hash |
| Where | `side_events.rs`: `SideEventKind::SearchAttempt`, `seen`, `own_factors` |

## A question this does not settle (needs review)

The cut event still names the enemy task force by its number in the log, as decision 0047, item 8, has every event name an enemy unit. So do the cut air fight, air attack and anti-aircraft events.

For a search this sits badly with decision 0012, which says a report holds no identity of the force seen, "so a side cannot tell two sightings are of one force". A side that reads the event beside the report can: two sightings that name `task_force:4` are of one force, on any day of the game. The briefing does not show it, but `explain` on a report does.

Removing the number is a change to item 8 of decision 0047 and touches every event that names an enemy unit, so it is not made here. It is the designer's to decide.

## As built

- `SideEventKind::SearchAttempt { air_group, task_force }` is the cut form. The agent names it `search_attempt` as before, with the text "air_group:7 searched and sighted task_force:4". The briefing does nothing with it: a search that sighted reaches the briefing as the contact its report made.
- `sim-cli export` writes each side's view of it with the two fields.
- Tests: the row of the table of kinds in `side_events.rs` is now a cut row; `a_search_that_sighted_keeps_its_own_factors_and_loses_the_distance_and_the_chance` beside it; `tests/scenario_09a_side_events.rs` asserts on whole scenarios that a side's search event and its breakdown hold no distance, no chance and no result.
- The Haiku game's transcript was regenerated: one `explain` answer changed. No save and no hash changed.
- Row 40 of `docs/rules/differences.md` is removed.
