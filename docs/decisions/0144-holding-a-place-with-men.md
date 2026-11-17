# 0144: Holding a place with men, the supply that follows, and the check that it can be held: step 4 of stage 3, second part

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 4 of its nine, and item 2 of the designer's amendment). It widens "hold a place" (decision 0117) to the men ashore and their supply, lets it be stated of a place the side does not hold yet, and gives "take a place" (decision 0142) the check that the place can be held before leave to launch. Nothing is built. It is built after decision 0143.

Summary: Step 4 of stage 3, second part: "hold a place" may be stated of a base the side means to take and waits for it; once the base is the side's the staff puts the men there on the defence and forms "keep supplied" under the hold from the source its limit names; and a "take a place" with such a hold beside it works out before it asks leave whether the men can be fed, and lands the supply that lasts until the first run that follows, with a margin for the risk

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Not built

## Why

Decision 0142 ends when the place changes side. The men are then on attack with nobody to attack, with the supply they landed with and nothing coming. Decision 0117 left the supply of a held base to the commander ("Supply under it waits"). And the designer asked that the staff say, before it asks leave to launch, whether the men can be kept fed, and that the 15 days of supply a landing takes be reasoned and not borrowed (decision 0139, "Amended by the designer", 2).

Taking and holding stay two ends. A commander who wants a place taken and kept states both, and step 8's "control an island" will state both for him. A place taken with no hold stated is taken and left as decision 0142 leaves it.

## A hold stated of a place not yet the side's

1. **A hold may name a base on the side's map that is not the side's.** It is taken in, and its plan drafted, prepared and released by the leaves as any hold's.
2. **While the base is not the side's** the staff forms nothing and orders nothing under it, and its reason says that it waits for the base to be the side's. It hands in no notice and marks nothing failed.
3. **A base the side held and lost** is in the same case: the hold waits.

## With men

On each morning a hold's plan is released and its base is the side's, before the cover:

4. **The men of the hold** are the side's fighting units with men ashore in the base's hex that the side may use and the commander does not hold: those its forces limit names, read up the tree, or with no land unit named anywhere, all of them. A unit that serves a "take a place" not yet met is that operation's and is left to it.
5. **Each that is on attack is set to defend.** Nothing else is ordered of them: on the defence they rest and dig in by the rules as they are. Attacking an enemy who lands at a held place is not this step's.
6. **A force on the transport mission named in the hold's forces limit is given no task of its own** (decisions 0124 and 0125): it is the supply's.

## The supply that follows

7. **Where the hold's forces limit names a source,** and no "keep supplied" for the base's hex is in force, the staff forms one under the hold, for the base, by `form_intent`: its floor is the margin of item 10 in days, and it has no interval, so the staff uses 15. `form_intent` takes "keep supplied" under "hold a place". Its forces limit is the hold's, read up: the ships and task forces and the source.
8. **It is served by the supply doctrine as any is,** from the next morning. Nothing of that doctrine changes.
9. **With no source named** the staff forms none and says that the commander may name one or state "keep supplied" himself, as before.

## The check that it can be held

It is worked by "take a place" on each morning its landing is ready and its plan is preparing, before leave is asked or the plan released, where a hold of a base in the place's hex is in force with a source in its forces limit. With no such hold nothing follows the landing, the check is not made, and the supply is the 15 days of decision 0142, item 16.

10. **The margin** is the staff's, in days, by the risk of the hold: 10 at preserve the force, 5 at balanced, 0 at press. Placeholders.
11. **The daily need** is the daily charge of the men chosen.
12. **The days until supply follows** are, in whole days: the days to land the men (the hours of the men's weight in the fullest ship at the place's rate for a ship, rounded up, and one at least); the passage back; one day to load; the passage out.
13. **The supply that lands with the men** is the daily need times those days and the margin. It replaces the 15 days of decision 0142, item 16, and is cut as that item says.
14. **The first run** is taken to carry the daily need for the 15 days of item 7.
15. **It can be held** if the source's stock above its reserve is at least the supply that lands with the men and the first run together, and the transport force's room is at least each.
16. **Where it can,** the reason of the release, or of the day it waits for leave, gives the check: the need, the days, the margin, the supply, the source's stock and the first run.
17. **Where it cannot,** the reason says what is short and by how much. At preserve the force (the risk of the "take a place") the staff does not ask leave and does not release: it hands in the notice that it cannot meet the intent, each morning the check fails. At balanced and at press it goes, and says so.

## What it leaves for later

- **A hold that attacks** an enemy who lands at the place or is still in the hex, and men moved between places to hold them.
- **Defences built under a hold:** the commander states "build up a base"; "control an island" forms it in step 8.
- **A source chosen by the staff,** and the ships for the supply chosen from all the side has: step 7.
- **Ammunition apart from supply.** The game has one stock.

## Rule card

- **Reads:** the side's own bases, land units, task forces and stocks; the hold's and the take's limits and risk; the ground and port figures; the map.
- **Writes:** nothing of its own. Posture orders, `form_intent`, and through decision 0142 the weight of one load order.
- **Reuses:** the hold, its plan and its leaves (0117); the supply doctrine whole (0107); the posture order; the landing of 0142. Two rules of intake change: a hold may name a base that is not the side's, and "keep supplied" may be formed under a hold.

## What it costs

- **State:** none new. The doctrine and agent interface versions rise. The rules label does not: no game without a hold of a place not the side's, or of a place with men, plays differently. A game with a hold of a base where the side has men on attack would: none built has one.
- **The rulebook's chapter 8,** "Holding a place", loses "Supply under it waits" and gains these three parts.

## The hand calculation

`tests/scenario_69_taken_and_held.rs`.
