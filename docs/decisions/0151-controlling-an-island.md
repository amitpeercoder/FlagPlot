# 0151: Controlling an island: step 8 of stage 3, second part

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 8 of its nine as amended, and the designer's first answer: control of an island means its named places held and their garrisons supplied above their reserve). It writes the area intent stated alone. It adds no rule to the engine (but see "As built": the staff's withdrawal, plans of their own and the reading of limits under an island, each added by the architect on the build's finding). Built on Nov 18, 2026: see "As built". Amended by the architect on Nov 17, 2026, after a review of step 8 that the designer accepted ("Amended after the review", below): it is built before decision 0150, which waits until after the proof.

Summary: Step 8 of stage 3, second part: the end "control an island", stated alone, from which the staff draws the tree: "take a place" and "hold a place" for each named place that is not the side's, "hold a place" for each that is, the supply under each hold from a source it chooses, a denial of the island's waters with the carrier forces under it, and the neutralising of each enemy base near it, the base at a place still to be taken among them; with its measure, the days on which every named place was the side's and held supply for its floor of days

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139; amended after a review. Built; its readings ruled (The architect's rulings)

## Why

The spec's play is one statement: control this island (`docs/step3-spec.md`, "The play it must support"). Every end it implies is built by now: take, hold, keep supplied, deny, neutralise. This record lets a commander state the one and have the staff state the rest, as decision 0116 meant a tree to work. He may still state any of them himself.

## An island

1. **An island is data.** A scenario names it: `[[island]]`, with a number, a name and its places in order, each a base or a coast hex. It is on both sides' maps, as the map is. The loader refuses an island with no place, a place off the map, and a number used twice.
2. **It is not state:** nothing of it changes in a game, and it is not in the hash.

## The end

3. **Control an island** names an island. It may have a date: each "take a place" the staff forms under it is formed with that date, and the rule of a take's date is decision 0142's. It is an end, addressed to the area, and a root. Its plan is drafted, prepared and released by the leaves, as a hold's is, and stands until the intent is withdrawn.
4. **It needs no forces limit.** With one, the staff's choices are inside it, kind by kind, as decision 0149 has it. Under "control an island" a forces limit that names no air group does not bind the air groups: a hold's cover, and every doctrine under the island that uses the groups at a base, uses the groups based there as with no limit (decisions 0117, 8; 0120, 7; 0123, 3); a limit that names air groups narrows them as before (added by the architect on Nov 17, 2026, when the build found that the staff's limit on a hold would have kept its cover from the fighters at its base).
5. **Inside one tree nothing is kept from a sibling.** The rule of what the staff may take (decision 0149, item 5) keeps a unit, a force or a base from an intent of another root that is higher in priority. Under one root the intents share: a unit or a force serves the first intent, in the order of item 6, that takes it that morning, and a ship serves the run or the lift that has it until it is back.

## The tree

Each morning the plan is released, the staff forms what is missing, by `form_intent` under the intent, in this order. An intent of the kind and place already in force anywhere on the side, whoever made it, counts, and none is formed beside it.

6. **In the order of the island's places:**
   - **Take a place,** for each named place where the side holds no base, and no "take a place" of it is in force with a plan that has not ended.
   - **Hold a place,** for each named place with a base on the side's map: of the side's base there, or, where the side has none, of the enemy's lowest-numbered base there, which waits until it is taken (decision 0144). A named place that is a bare hex has its hold formed on the first morning the side has a base there.
   - **Deny the waters** within 3 hexes of the island's first place, once. The 3 hexes are a placeholder.
   - **Operate a carrier force,** under the denial, for each task force on the carrier mission that the staff may take, with a carrier afloat that has an air group aboard, in task force order, where no carrier task for that force is in force anywhere on the side. Its hex is the covering hex of decision 0148, item 2, reckoned to the island's first place.
   - **Neutralise a base,** for each enemy base on the side's map within 15 hexes of a named place, in base order: a base in a named place's hex, which a take of this tree is to take, as well as a base off the island. The 15 hexes are the placeholder of decision 0124.
7. **Their limits.** The staff sets no risk: each reads the island intent's. It sets a forces limit only on the holds, by item 8. The takes choose their forces by decision 0149; the denial and the neutralising use what their doctrines find (decisions 0120 and 0123) and say so where they find nothing.
8. **The source of supply** for the holds is the commander's, where his forces limit names one. Else it is the side's base in no named place's hex, that the staff may take, at which a transport force it may take lies stopped, with the most supply above its reserve; then the lower number. The staff sets each hold's forces limit to that source and the transport forces stopped there. With no such base the holds form no supply and say so.
8a. **The carriers are the island's, not a take's.** Under "control an island" a take takes no carrier force (decision 0149, item 10, is set aside for carriers), and a force is not sent home when a take is met (decision 0148, item 8, is set aside). The force stays on its task under the denial, by the carrier doctrine as it is: it searches, strikes ships, weighs its fuel and its rounds and goes back for them. The strike on the place (decision 0148, items 4 to 6) applies to the task while a take of the tree has its plan released and is not met and its place is in the bombers' reach: the first such take in the island's order.
8c. **The bombardment forces are the island's too** (added by the architect on Nov 18, 2026, before the loading check of the proof; "Amended before the proof", below). Under "control an island" a take takes no force on the bombardment mission (decision 0149, item 10, is set aside for them as for carriers). Each morning, for each task force on the bombardment mission with a ship afloat that the staff may take and that lies stopped at a base of the side, in task force order, where no bombard task for that force is in force anywhere on the side, the staff forms "bombard a base" under its neutralising of the enemy's base at a named place: the first such neutralising in the island's order. A force that may be used only from a later day is so taken on its first morning. The task is then decision 0146's as it is: whether it sorties, its speed, its fuel and its rounds. A base off the island is not bombarded by this. When the base is taken the neutralising is withdrawn (item 8b) and the bombard task with it, and the force is given the readiness order of decision 0146, reading 2.
8b. **A neutralising that has done its work.** On the first morning a base the staff is neutralising under the island is the side's, the staff withdraws the intent it formed for it. The staff may withdraw an intent it formed; one the commander made it may not (added by the architect on Nov 17, 2026, when the build found no staff command withdrew anything).
9. **What follows is the built doctrines':** each take is prepared, launched on leave and landed; each hold puts its men on the defence, covers its base and forms its supply; the check that a place can be held is made before a take is launched (decision 0144). Each take the staff forms has a plan of its own, drafted at its forming, made ready that morning on leave to prepare, released and marked complete by decision 0142's doctrine as a root take's is, and ended with the island if the island is withdrawn (added by the architect on Nov 17, 2026, on the build's finding: the island's plan is released on its first morning, and a take that shared it would have had no prepare). So has each "keep supplied" the staff forms under a hold of the island, since the island's holds share one plan and a plan's runs are its one supply intent's; intake reads its forces limit and its source up the tree, the hold's (added by the architect on Nov 18, 2026, on the build's finding).

## Its measure

10. **Controlled on a day:** at the end of the day every named place is the side's (a place that names a base, when that base is the side's; mended on Nov 18, 2026, decision 0152's ruling 3), and in each the side's supply is above the reserves of its bases there and its days of supply in the hex are at least the floor of the supply intent under the place's hold (5 days unless the commander set another). Worked from the side's own view and events, stored nowhere, as a hold's measure is (decision 0117, item 14).
11. **It gives** the days in force, the days controlled, and for each place whether it is the side's and its days of supply today. It is shown in `command_state` and in `report`, in the words "controlled on N of M days in force".
12. **The intent is never marked complete.** It stands, and what it formed stands, until the commander withdraws it.

## What it leaves for later

- **An island with a place inland,** which waits on the march from a beach (stage 4).
- **The order the places are taken in,** chosen by the staff: they are taken in the island's order, each as its forces come free.
- **Garrisons sized and moved between places,** and defences built under a hold.
- **Leaving formed by the staff** when a place cannot be held.
- **Where a carrier force stands under an island,** chosen against the enemy and not only by the bombers' reach of the first place; and carriers of one island answering for another.
- **A carrier force short of rounds.** The carrier doctrine weighs its fuel and goes home for it and back; it weighs no ammunition, so a carrier force under an island is not sent anywhere to rearm (decision 0127 is the surface forces'; the build's finding, accepted by the architect on Nov 17, 2026).

## Rule card

- **Reads:** the island's places; the side's own bases, forces and units; its map of the enemy's bases; every intent in force and its plan.
- **Writes:** one kind of intent with its island, in the command state; the staff's withdrawal of an intent it formed; nothing else.
- **Reuses:** `form_intent`, `set_limit` and the withdrawing of an intent; the ends of decisions 0117, 0120, 0123, 0125, 0142, 0144, 0148 and 0149 whole.

## What it costs

- **State:** one kind of intent, in the hash and named. The state layout, saved game, agent interface and doctrine versions rise. The rules label does not.
- **Data:** the islands of each scenario that states the intent. The staff's setup of the 39 days names Guadalcanal's places from the rows the slice already rests on.
- **The rulebook's chapter 8** gains "Controlling an island".

## The hand calculation

`tests/scenario_76_controlling_an_island.rs`.

## Amended after the review

On Nov 17, 2026 the architect reviewed step 8 before it was built, at the designer's asking, and the designer accepted the review. Four things in this record would have shown in the proof as failures of the staff that were faults of the rule as written:

1. **The enemy's base at a place still to be taken was not neutralised.** A side asked to control an island whose airfield the enemy holds would have raided every enemy base near the island but that one, since a take has carrier strikes and bombardment and no raid from the land. Item 6 now neutralises it too, until it is taken (item 8b).
2. **A carrier force was sent home when the landing was done and given nothing after.** Items 6 and 8a keep the carrier forces under the denial for as long as the island is asked for.
3. **The measure counted any supply at all,** since no base of the slice keeps a reserve. Item 10 asks for the floor of days.
4. **An island could not be asked for by a date,** which decision 0139's eighth condition names. Item 3 passes the date to the takes.

The fifth fault was decision 0152's and is mended there (its item 5). "Leave a place" (decision 0150) is moved to after the proof: neither side states it in the 39 days, and the staff leaving of itself is stage 4's.

## As built

Built on Nov 18, 2026, on the branch `stage3-step8a`. Nothing is tuned and no figure changed. Scenario 76 (`data/scenarios/76_controlling_an_island.toml`, on scenario 27's map, its own ground figures `data/reference/ground_placeholder_scenario_76.toml` with `preparation_per_day` 100, frozen beside the others) plays the main case run by itself. `tests/scenario_76_controlling_an_island.rs` has 29 tests and all pass: the hand calculation in its parts, each property in its own, and one test of the architect's ruling on air groups under an island (not part of the hand calculation). The main game gives the hand calculation whole: the tree of the first morning (intents 2 to 6 in order, then "keep base 3 supplied" and "ready a landing"), the take's plan of its own preparing on day 1, released on day 2 and complete on day 5, intent 6 withdrawn on day 5, "keep base 2 supplied" formed on day 5, ten orders, none refused, the hours, base 4 formed and base 2 the side's at the end of day 4, 919, 7, 35 and 79 t at the end, and the island controlled on 10 of 14 days.

### Mismatch with the hand calculation

1. **By a date.** The hand calculation had the task "ready a landing" with day 10; by decision 0142, item 13, it is day 10 less the 2 days to embark and the day of passage, plus 1: day 8, which the build gives. The architect restated the property on Nov 18, 2026, and the test asserts day 8.

### Found by the build, and ruled by the architect

1. **The staff could not withdraw an intent.** `withdraw_intent` was the commander's alone. It may now be given by the staff on an intent the staff made, and is refused "Not yours" on one the commander made; the commander's on a staff-made one is still refused "Made by staff". The apply is unchanged. Used only for item 8b. The rules label does not rise: no saved game holds such a command (as decision 0149, reading 15).
2. **A take under an island needs a plan of its own** (item 9). The island's plan is released on its first morning by the leaves, so a take that shared it would never prepare.
3. **So does a "keep supplied" under an island's hold** (item 9). Both holds share the island's plan; the supply doctrine reads a plan's runs as its one supply intent's, so the run for base 2 was read by the doctrine of base 3, and the transports were routed to C as well; and intake checked a run against the island's forces limit, which it has none of. `CommandState::forms_own_plan` is the one rule for both cases, read by intake, the invariant and the staff. With two islands stated by one side, each supply intent's runs are on its own plan and intake reads each run against its own plan's owner, so nothing of one island's supply is read as the other's (checked by reading the code; no scenario states two islands).
4. **Air groups under an island** (item 4): a limit that names no air group does not bind them.
5. **A carrier force weighs fuel and not rounds** (item 8a): added to what this record leaves for later.

### What was built

- **The state and the engine** (`crates/sim-core`). `IntentKind::ControlIsland { island, by_day }`, code 15, `control_island`, addressed to the area, a root; take, hold, deny and neutralise may stand under it (`command.rs`, `may_stand_under`). In the hash after a bombardment's fields, island then date, named in `name_every_field`; layout `flagplot-state-v44`. The island is map data (`map.rs`: `Island`, `IslandPlace` with its hex, `Map::island`; `SideView::island`), not hashed. Intake (`command/intake.rs`): the refusal `NoSuchIsland`, at the end; the staff's `withdraw_intent`; a plan drafted at the forming where `forms_own_plan` holds, ended with the island; the narrowing check of decision 0149 under an island; a run's forces limit and source read up the tree for a supply intent that owns a plan with no limit of its own. `forces_on_chain` fills an island's limit from the nearest beneath it; `groups_on_chain` under an island leaves out a limit with no air group. The invariant reads a sub-intent's plan as that of the nearest owner on its chain (`plan_owner`).
- **The data** (`crates/sim-data`): `[[island]]` with `number`, `name` and `places` (`{ base = N }` or `{ hex = [q, r] }`), refused with no place, a number twice, a base not in the scenario, a place off the map or not coast, or a place with both keys or neither; and `control_island` with `island` and `by_day` in `[[command]]`.
- **The staff** (`crates/sim-staff/src/island.rs`, new): `Day::islands`, run after the leaves and before the hold doctrine (`air.rs`, `Day::run`). The leaves serve every intent that owns a plan (`leave_for`). `cover.rs`: `covering_hex` taken out of the forming under a take, unchanged in its words; `under_island` and `island_strike_take`; the island's carrier tasks served after the landing's morning (`take_carriers`). `carrier.rs`: the strike on the place for a task under an island's denial. `assemble.rs`: no carrier force for a take under an island; the take chooses inside the island's limit and names its whole choice. `holding.rs`: a supply intent's own plan. Doctrine `supply-31`.
- **The measure** (`crates/sim-agent/src/island.rs`): `island_measure`, in `command_state` and the report.
- **The tools**: `set_intent` takes `control_island` with `island` and `by_day`; `command_state` gives `island` on every intent; the report gains `islands`. Agent interface 24; saved game 19.
- **The rulebook**: chapter 8, "Controlling an island" and every section it touches; chapter 1's hash paragraph; the glossary; `docs/commands.md`.

### Readings taken

1. **A place's hex.** A base place means its base's hex, written by the loader, so both sides read the same hex whoever holds the base. A bare place must be a coast hex.
2. **"The kind and place already in force"** is a take of the same hex with a plan not ended; a hold of any base in the place's hex; a denial centred on the first place, whatever its distance; a neutralising of the same base; a carrier task of the same force. Where it stands in another tree the reason names it, its number and whose it is; what the island's own tree formed stands without a word.
3. **The hold's base** is the side's lowest-numbered base at the place, else the enemy's lowest-numbered there on the side's map; a bare hex with no base gives a reason each morning and waits.
4. **The source** is worked each morning the island forms a hold, and the hold's limit set that morning, once: the source and the transport forces (transport mission, a ship afloat) stopped there that the staff may take. The limit is not set again (decision 0152, item 11, will set it each morning). A base "the staff may take" is decision 0149's, item 5, served before the island in another tree; the commander's source, where his limit names one, is the only base weighed. "Supply above its reserve" is the stock less the reserve, the most first, then the lower number.
5. **The carrier forces under the denial** are formed only under a denial of the island's own tree; a denial of those waters in another tree is named and no carrier task is formed under it. A force the staff may not take is named with the reason. The forming's words are given below.
6. **The strike on the place** is for the first take of the tree in the island's order whose plan is released, whose place is not the side's, and which lies within the longest normal strike radius of the bomber groups aboard from the force's hex; each group then weighs its own reach by decision 0148's rules.
7. **The take's own plan** is made ready the morning it is formed where leave to prepare is given, else it waits with the notice; on later mornings the leaves serve it as a root's.
8. **Under an island's limit** a take with no limit of its own chooses inside the island's limit, kind by kind, and its staff limit names the whole choice: the transport force and the other forces, the units, and the source, so that the take reads it as it binds.
9. **The measure** applies today's daily use in a hex to every past day: the side's events give a base's stock at each day's end but not its use. With no use in a hex, a place counts where its stock is above its reserves. The floor is that of the lowest-numbered "keep supplied" in force for the hex under a hold of the island; with none, 5 days (a placeholder). A place the side has lost shows none of its earlier days, as a hold's measure.
10. **The neutralising withdrawn** (item 8b) is one the staff formed under the island whose target is among the side's bases that morning; it is withdrawn before anything is formed, with everything beneath it.
11. **The island's order of work** is: the withdrawals, then for each place its take and its hold, then the denial and its carrier tasks, then the neutralising, in base order.

The forming reason of a carrier task under the denial, as worked: "{deny} denies the waters of island {n} ({name}), its plan is released and no carrier task of {force} stands: {force as held} is on the carrier mission with {c} carrier(s) afloat and {g} air group(s) aboard, and lies at {hex}. For the island's first place at {place}: {the covering hex's words of decision 0148}. The staff forms {intent}, to operate the carrier force {force} from {hex}, under {deny}, for {force} to carry out."

### Earlier tests, baselines and games that change

No expected number changed and no earlier test is ignored. Every scenario that states "take a place" directly (66 to 74) plays as before. Run baselines: every one changed in its hash lines only; scenario 76's two are new. Pinned hashes and the batch fixture are remade. Recorded games: each `save.json` differs only in its versions and day hashes; each transcript in the answers' `version` (23 to 24), and two (`keep_two_bases_fed_sonnet`, `second_proof_week_sonnet`) in the new keys `island` (null) and `islands` (empty). Version pins move in the agent's and the saved games' tests; `crates/sim-cli/tests/export_pictures.rs`'s export hashes are remade at layout v44 (the state version, the day hashes and the new `island` key of an intent's kind); the two tests that compare every baseline with a run with no staff leave out scenario 76.

### Files to read

- `crates/sim-staff/src/island.rs` (the doctrine whole: `islands`, `island_tree`, `withdraw_done`, `island_take`, `island_hold`, `hold_limit`, `island_source`, `island_deny`, `island_carriers`, `island_neutralise`).
- `crates/sim-staff/src/cover.rs` (`under_island`, `island_strike_take`, `covering_hex`), `carrier.rs` (the call of the strike), `assemble.rs` (item 8a), `holding.rs`, `air.rs` (`leaves`, `leave_for`, `Day::run`).
- `crates/sim-core/src/command.rs` (`forms_own_plan`, `owns_plan`, `plan_owner`, `island_on_chain`, `forces_on_chain`, `groups_on_chain`), `command/intake.rs` (the staff's withdraw, `forces_of`, the forming), `invariants.rs`.
- `crates/sim-agent/src/island.rs`; `tests/scenario_76_controlling_an_island.rs`.

## The architect's rulings

Made on Nov 18, 2026, on reading the build.

1. **Readings 1 to 11 are accepted as built.**
2. **Four rules were added during the build, each on a finding of the team and each ruled before it was built:** the staff may withdraw an intent it formed (item 8b); a take formed under an island has a plan of its own, and so has the supply formed under an island's hold (item 9); and a forces limit that names no air group does not bind air groups under an island (item 4). Without the last a held airfield would have flown no patrol once the staff had set its hold's supply ships.
3. **One number of scenario 76's hand calculation was the architect's error and is restated:** with the island asked for by day 10 the task "ready a landing" has day 8.
4. **A carrier force weighs its fuel and not its rounds** (the team's check before building). It is brought back to its hex once fuelled. Rounds for a carrier's aircraft are left for later.
5. **Known and left, on reading 9:** the measure is worked each time from the side's own view of today, so a place that is lost shows none of its earlier days, and today's use is applied to past days. It is reported and not judged in the proof; a measure that keeps its days is for the interface's work.
6. **Not yet played:** two islands stated by one side. The code keeps each supply intent's runs on its own plan; scenario 77's property "Two islands" (decision 0152) is the first game of it.

## Amended before the proof

On Nov 18, 2026, the three setup files of the proof were smoke-tested on main before decision 0152 was built (`docs/validation/stage-3-proof-setup.md`). The run was not a loading check and nothing was judged. It showed one fault of this record: a force on the bombardment mission was taken only by a take, on the one morning the take assembled its force (decision 0149, item 10), and never under an island's neutralising, since decision 0146 forms a bombard task only for a force a forces limit names and an island has none. In the staff's setup the Japanese bombardment force may be used from day 18, days after the first lift is assembled, so it would have lain at Shortland for the whole game. The record of the campaign has the airfield shelled from the sea through these weeks, and the proof's fourth condition counts it. Item 8c makes the bombardment forces the island's, as item 8a made the carriers. It is built in its own pull request, with a property of scenario 76, before the loading check.

