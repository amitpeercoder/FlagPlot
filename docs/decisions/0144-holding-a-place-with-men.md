# 0144: Holding a place with men, the supply that follows, and the check that it can be held: step 4 of stage 3, second part

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 4 of its nine, and item 2 of the designer's amendment). It widens "hold a place" (decision 0117) to the men ashore and their supply, lets it be stated of a place the side does not hold yet, and gives "take a place" (decision 0142) the check that the place can be held before leave to launch. It is built after decision 0143. Built on Nov 17, 2026, with decision 0143: see "As built".

Summary: Step 4 of stage 3, second part: "hold a place" may be stated of a base the side means to take and waits for it; once the base is the side's the staff puts the men there on the defence and forms "keep supplied" under the hold from the source its limit names; and a "take a place" with such a hold beside it works out before it asks leave whether the men can be fed, and lands the supply that lasts until the first run that follows, with a margin for the risk

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings await the architect (As built)

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
17. **Where it cannot,** the reason says what is short and by how much. At preserve the force (the risk of the "take a place") the staff does not ask leave and does not release: it hands in the notice that it cannot meet the intent, once, and gives the reason each morning the check fails (decision 0142, the ruling on reading 8). At balanced and at press it goes, and says so.

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

## As built

Built on Nov 17, 2026, on the branch `stage3-step4`, with decision 0143. Nothing is tuned and no figure changed. Every number of the hand calculation that an event, the state or a reason carries is asserted exactly, and no test is ignored: no mismatch was found. Scenarios 67 and 68 give the orders they gave, and every scenario file built before this one gives the events it gave (no run baseline changed). One recorded game changed in more than its versions (Earlier tests touched, below).

### What was built

- **Intake (`crates/sim-core/src/command/intake.rs`, `command.rs`).** A hold may name a base of the other side that is on the map (`check_intent_kind`). "Keep supplied" may stand under "hold a place" for the same base (`IntentKind::may_stand_under`), and formed there its floor may be 0. A third change, which the hand calculation needs and the rule card does not name: a hold with no land unit named in any forces limit on its chain may order the fighting units with men ashore in its base's hex (`may_use_land_unit`), as a build-up may its working units; without it the posture orders of day 6 are refused as outside the forces limit (reading 4).
- **The hold doctrine (`crates/sim-staff/src/air.rs`, `holds`, and the new `crates/sim-staff/src/holding.rs`).** A hold of a base not the side's waits with one reason (items 1 to 3). Otherwise, before the forces' tasks and the cover: `Day::hold_men` (items 4 and 5) and, on a morning the hold has men there, `Day::hold_supply` (items 7 and 9). Item 6 needed no code: the tasks of decisions 0124 and 0125 are formed for the surface combat and carrier missions only.
- **The supply doctrine** (`doctrine.rs`, `setup`) reads a supply intent's forces limit up the tree where it has none of its own, and its projection (`projection.rs`, `run_landings`) counts a plan's runs as its supply intent's: the plan's own intent, or the "keep supplied" formed in the plan under a hold (reading 8).
- **The check (`crates/sim-staff/src/take.rs`, `hold_of_place` and `hold_check`)**, used for the supply of the first lift (`supply`, by its days) and in `launch_place` before leave is asked, with the notice once at preserve the force (the ruling on decision 0142's reading 8).
- **Scenario 69** (`data/scenarios/69_taken_and_held.toml`, on scenario 27's map, with its own ground figures in `data/reference/ground_placeholder_scenario_69.toml`: decision 0143, As built, "A scenario file plays its game") and its 16 tests (`tests/scenario_69_taken_and_held.rs`).
- **The tools.** `form_intent` of `keep_supplied` is written and read with `floor_days` and `interval_days` (`crates/sim-agent/src/commands.rs`); the tool descriptions and the schemas say a hold may name an enemy base on the map; the agent interface is version 20.
- **Versions.** Doctrine `supply-24`; agent interface 20 (and the schemas' constants). The state layout (`flagplot-state-v41`), the saved game (16) and the rules label (18) do not rise.

### Readings taken

1. **"A base on the side's map that is not the side's"** is a base of the other side with `on_map`, as for the target of a neutralise.
2. **Waiting** gives one reason a morning, before anything else under the hold, and no notice.
3. **The men of the hold** are fighting units only (a working unit is not set to defend), at the base's hex, read through every forces limit on the chain where any names a land unit, else all. "The commander does not hold" counts a hold by command and by order; a held unit on attack gets a reason naming the hold. A unit "serves a take a place not yet met" where it is named in the forces limit of a "take a place" whose plan has not ended and whose place is not the side's, or of a task under it; "met" is read as the condition (the place the side's), not the plan's state, so on the morning the hold first runs after the capture (before "take a place" marks its plan complete) the men are already the hold's, as the hand calculation has it.
4. **The third intake change** (What was built): needed for the posture orders; it also makes a commander's own order to such a unit a hold by order (Holds and the override), as for any unit an intent may use.
5. **The supply follows the men.** Items 7 and 9 act only on a morning the hold has men at the base (a fighting unit of item 4, on attack or not, not held). A hold with no man there forms nothing and says nothing of supply: so it plays as it did, as the properties ask. A hold with men and no source gives the reason of item 9 each morning it has men there.
6. **"No keep supplied for the base's hex in force"** is any "keep supplied", by anyone, whose base lies in that hex.
7. **Floor 0.** At press the floor is 0, which intake refuses for a commander's "keep supplied"; it is accepted for one formed under a hold.
8. **The formed intent's limits.** Its forces limit is read up the tree (the hold's) by the supply doctrine, and intake reads it so already for the staff's orders and the plan's commands. Its area is its own (none), not read up. The projection had read a plan's runs as its root intent's, which under a hold is not a supply intent: the run of day 9 was then not projected and the doctrine handed in `cannot_meet` on day 10. It now reads them as the plan's supply intent's.
9. **The hold of the check** is the lowest-numbered hold in force of a base, the side's or not, in the place's hex, whose nearest forces limit names a source of the side; its risk gives the margin, and the end's risk decides whether the staff goes.
10. **The fullest ship** is, over the units chosen, the most men of one unit in one ship of the force, or, for a unit ashore, the least of its men and the most berths of a ship of the force. Its men weigh `handling_t_per_man` each.
11. **The place's rate for a ship** is its port size's berth rate where the port has a berth, else its lighter rate; the landing factor of a force's loading and work on the port are not counted. With no rate, one day.
12. **The passages** are from the place to the hold's source and back, inside the end's area, at the cruise speed of the transport force's slowest ship, each rounded up to whole days, one at least. With no route the check is not made and the supply is 15 days.
13. **The room** of the transport force is its ships' cargo capacity less any fuel carried as cargo (supply aboard counts as room for supply). The stock is the source's above its reserve that morning, with no load still to be taken counted against it.
14. **The days of the check set the supply** in every reading of the first lift's supply, the day-1 words of the choice included; a later lift (decision 0143) takes 15 days.
15. **The words.** Where it can be held, the check's words follow the release's reason, or the waiting reason; where it cannot at balanced or press, "the staff goes all the same". At preserve the reason carries the check and the notice is handed in once.
16. **Order within a morning.** The hold doctrine runs before "take a place", so on day 6 the hold's `form_intent` comes before the `mark_plan` of the end, and its posture orders before anything of the end's.

### Earlier tests touched

No expected number changed. The agent interface's pins move from 19 to 20 (`crates/sim-agent/tests`, the schemas). The run baselines are unchanged; the recorded games are remade by `scripts/regenerate.sh`: each differs in its doctrine and interface versions, and `second_proof_week_sonnet` also gains, from day 15, one reason a morning under the Allied hold of base 4 (Tulagi), where the side has men and the hold names no source, that the commander may name one (reading 5). No order or command of that game changed; its day hashes of days 15 to 21 changed because the extra reasons are events and move the numbers of later events, which the state cites. The architect may rule whether a hold with men and no source should say this.

### Files to read

- `crates/sim-staff/src/holding.rs` and `air.rs` (`holds`).
- `crates/sim-staff/src/take.rs`: `hold_of_place`, `hold_check`, `supply` and the leave in `launch_place`.
- `crates/sim-staff/src/doctrine.rs` (`setup`) and `projection.rs` (`run_landings`).
- `crates/sim-core/src/command/intake.rs` (`check_intent_kind`, `may_use_land_unit`, the `form_intent` floor) and `command.rs` (`may_stand_under`).
- `tests/scenario_69_taken_and_held.rs`.
