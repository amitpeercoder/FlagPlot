# 0052: What step 1 leaves out

Status: confirmed. The designer agreed on Nov 12, 2026 to defer ground combat, land units and shore bombardment out of step 1. The smaller items below are settled by the architect.

Summary: What step 1 leaves out: ground combat, land units, shore bombardment and base orders deferred; pilots as a count and weapons as figures recorded as choices

Standing: Confirmed

## The gap

A check of the spec against the code, made as step 1 closes, found things the spec lists that were never built and that no decision record had put off. Step 1 was built scenario by scenario, each adding the subsystem its scenario tests, and none of the spec's test scenarios uses any of them. So they were neither built nor decided. This record decides them.

## Decisions

1. **Ground combat, land units and their orders are deferred** to the step whose scenario first needs them. The spec lists "simplified land units" and "ground combat" in scope, a `LandUnit` in the data model, ground combat as step 5 of the day, and land unit orders (posture, move, embark, disembark). None is built. The day keeps its empty ground combat phase so the order of resolution does not change when the rule arrives. A base's garrison waits with them.

   *Nov 14, 2026: built since: land units carried by sea (decision 0066), the fight ashore (decision 0090) and marching overland (decision 0094). Base orders and the ground support mission are still deferred.*

2. **Shore bombardment is deferred** in the same way. The night pulse keeps its empty phase, and `bombardment` stays a task force mission that changes nothing yet. Decision 0014 listed it as not built; this says when: with the scenario that needs it.

   *Nov 14, 2026: shore bombardment is now built, by decision 0092.*
3. **The ground support air mission is deferred** with ground combat. Escort is not a mission of its own: fighters escort a raid by the staff's choice (decisions 0027 and 0030).
4. **Base orders are deferred.** Reserve levels were deferred by decision 0050. Repair priority joins them: repairs are made lowest-numbered group first (decision 0035), and an order to change that waits for the staff, which is what would use it.
5. **Pilots are a count for each air group, not people** (decision 0038). Skills by mission, experience and status for each pilot wait for step 2, where pilot quality is tuned against the record. This was built so and is now recorded as a choice.
6. **Weapons are figures on a ship class and an aircraft type, not a table of their own.** The derivation reads weapons from the order-of-battle facts (decision 0020) and writes the figures a rule needs. A weapon table in the simulation's data would repeat them.
7. **Task force orders to form, disband and add ships, and air group orders to rebase, set target priority and set a range limit**, stay deferred as decision 0050 says. A search arc's range is the only range limit there is.

   *Nov 14, 2026: rebasing is now built, by decision 0091. Setting target priority is built by decision 0063 and the range limit by decision 0059, as the order `SetReach`.*
8. **Weather** stays out, as decisions 0005, 0012, 0017, 0036 and 0042 each say for their own rule.
9. **Events for one unit will be built before step 1 is called done.** The spec's definition of done says the event log answers "what happened to this entity". `explain` answers "why" for an event, and nothing yet lists the events that name one unit. It is small and it is promised, so it is built and not deferred.
10. **The spec's count of crates and scenarios is out of date** (three crates, eight scenarios). `CLAUDE.md` holds the layout as it is. The spec gets a note under "Scope" pointing here and is otherwise left as written, so it still shows what was first intended.

## Why

- **A rule with no scenario has no hand calculation, no property test and no legibility test.** Building ground combat now would add rules nothing checks, which is what the project's own process forbids.
- **The slice still needs them.** Guadalcanal is a land campaign fed by sea. This is a deferral with a place to land, not a cut.
- **Deciding it is better than leaving it.** An item in the spec that is silently absent reads as done.

## What it costs

- Step 1 cannot play the fight for Henderson Field. Bases do not change hands, and a transport's cargo is supply and fuel only.
- Scenarios that end in a landing or a bombardment (Savo Island's reason for being, the October bombardments) run only their sea and air parts.

## Rule card

No rule is added or changed.

## As built

Item 9 is built as `sim-cli events`, an umpire's command that reads the truth like `run`. `events SCENARIO_OR_SAVE --unit HANDLE [--day N] [--seed N]` runs the scenario (or replays the saved game) to its end and prints every event that names the unit, in the text `run --events` prints, with a count last. A handle is `ship:N`, `task_force:N`, `air_group:N`, `base:N` or `contact:N`; contact numbers belong to a side, so `allied:contact:N` or `japanese:contact:N` picks one. For a ship it also prints the events of the task force it was in at the time, each marked `via task_force:N`. `events SCENARIO_OR_SAVE --why EVENT` prints the event and its causes down to orders as an indented tree, with each event's breakdown, and an event reached twice is printed once and then referred to by number.

## Added on Nov 12, 2026: more that step 1 leaves out

A second check of the spec against the code found more that was listed and never built, for the same reason: no scenario of step 1 uses it. Settled by the architect, on the same ground the designer agreed for ground combat. The designer was told and may bring any of them back into step 1.

11. **Ship repair is deferred to early in step 2.** No rule lowers a ship's flotation, fire or system damage, in port or at sea (decisions 0014 and 0016 listed it as not built). It matters as soon as a campaign is long: without it fleets only wear down. It comes first in step 2 because the comparison with the record over months depends on it. *(Nov 13, 2026: decision 0069 plans it, and it is not part of step 2's close: it follows step 2, with land units and ground combat, before step 3, ruled by the coordinating architect on the designer's delegation.)* *(Nov 13, 2026, built: decision 0069.)*
12. **Damage to a base's port and airfield is deferred** (decisions 0011 and 0034 listed it as not built). A raid hits aircraft and supply only.
13. **Crew experience, air group morale, aircraft in reserve and a base's aviation support are deferred** to step 2 with pilot quality (item 5): they are qualities to tune against the record, and nothing reads them yet. Decision 0032 is the lesson: a quantity no rule reads does not stay.

   *Nov 14, 2026: ship experience is built, decision 0064, stage 1; air groups wait (decision 0064).*

14. **Terrain is not read by any rule.** The kinds exist; the generated map marks every land hex as jungle (decision 0017). It waits for ground combat (item 1).
15. **Port and airfield sizes are zero everywhere on the generated map.** Port size sets only how fast cargo is worked; no rule reads airfield size. Sizes for the real bases are data for step 2.
16. **"No belief data written except from a report" is kept by construction, not by a daily check:** `BeliefState::apply_report` is the only way in and is not visible outside the crate.
17. **A batch runs with breakdown recording off,** as the spec allows. That is now so.

