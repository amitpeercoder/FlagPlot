# 0143: Ashore: the attack on the estimate, and a second lift: step 4 of stage 3, first part

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 4 of its nine, as amended). Step 4 is written in two records: this one, what the men of "take a place" do from the first man ashore until the place is taken; and the next, "hold a place" with men, the supply that follows and the check that the place can be held. It adds no rule to the engine. It is built after decision 0142. Built on Nov 17, 2026, with decision 0144: see "As built".

Summary: Step 4 of stage 3, first part: ashore under "take a place" the staff attacks when the men who can attack are enough against its estimate of the enemy for the risk, stands on the defence and rests when they are not, sends the units a first lift left behind, and attacks again when the men are rested and enough

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings await the architect (As built)

## Why

Decision 0142 lands the men on attack and then gives no order until the place is the side's. Against an empty airfield that is enough. Against a garrison it is not: the engine sends a tired unit back to the defence and nobody sends it forward again, nobody stops an attack that the first day shows to be too weak, and the units left at the source stay there. This record gives the staff those three judgements. They are judgements on the side's estimate of the enemy, which may be wrong, and the reasons say what it was.

## The judgement of each morning

It is made on each morning the plan of a "take a place" is released, the place is not taken and a unit chosen for it has men ashore in the place's hex. It replaces item 24 of decision 0142, and item 22 where they differ.

1. **The units of the operation** are the fighting units chosen (the forces limit of the task formed, decision 0142, item 11) and the units a later lift brought (item 9 below), that have men ashore in the place's hex, that the side may use and the commander does not hold.
2. **A unit can attack** if it is on attack already, or its fatigue is at or below the mark for the risk:

| The risk | The mark |
| --- | --- |
| Preserve the force | A third of `attack_fatigue_limit` |
| Balanced | Two thirds of it |
| Press | The limit itself |

Each is cut to a whole number. The two shares are the staff's and are placeholders.
3. **Its fit men** are its men ashore times 100 less its fatigue over 100, cut to a whole man: what the unit's own side knows of who would fire.
4. **The men wanted** are the side's estimate of the enemy's men in the hex times the figure for the risk of decision 0142, item 10: 3, 2 or 1.
5. **Attack.** Where no estimate is held, or the fit men of the units that can attack are at least the men wanted, each unit that can attack and is not on attack is set to attack, with no instruction for when it is spent: the rule's own default stands (it holds where it can be fed). With no estimate the reason says that it attacks with no count of the enemy.
6. **Wait.** Otherwise each unit of the operation that is on attack is set to defend, and no other order is given. The reason gives the estimate with its day, the men wanted, the fit men and how many are short, each unit's fatigue against the mark, and what is coming (item 9).
7. **A unit the rules sent back to the defence, tired,** is under the same judgement: it is sent forward again only when it can attack by item 2 and item 5 says attack.
8. **A unit that lands** is under the judgement from the first morning it has men ashore in the hex. Item 17 of decision 0142 stands for the first lift: its units are set to attack before they sail.

## A second lift

9. **The units left behind** are the fighting units of the intent's forces limit that were not chosen, and that the side may use, the commander does not hold, and whose men are all ashore in the source's hex.
10. **They are sent** while the place is not taken, on a morning the transport force lies stopped at the source with nothing aboard, where no estimate is held or the fit men of the units of the operation that can attack are fewer than the men wanted. The units are taken in the order and by the ships of decision 0142, items 8 and 9, until the men ashore and the men taken are at least the men wanted or no ship is left; with no estimate, every unit the ships can carry.
11. **The lift** is as the first, by items 16 to 19 of decision 0142: supply to a weight on the first morning (the daily charge of the men of this lift for 15 days), a unit a morning by a load order, then the route to the place and back and the unload order, with an escort as before. Two things differ. No unit of a second lift is set to attack before it sails: the judgement of each morning sets it. And the force goes as it stands, with no landing made ready: the reason says what loading it has.
12. **The units of a second lift are kept** as the forces limit of the task "ready a landing" was: the staff adds them to that limit by `set_limit` on the morning the lift is decided, so the choice is made once and the commander may change it.
13. **Nothing more is sent** once the place is taken, or when no unit is left behind. Where the men are short and nothing is left to send, the reason says so, and the staff waits: leaving the place is the commander's (step 8).

## What it keeps

14. **Holds and leaves** as in decision 0142, items 25 and 26. A unit the commander has given a posture is held by that order, as any order of his holds a unit, and is not counted among the units of the operation.
15. **Reasons** carry the numbers in the order of decision 0139's amendment: the estimate and its day, the men wanted, each unit with its men, its fatigue and its fit men, what is short, what is coming and when.

## What it leaves for later

- **A march from a beach to a place inland,** and with it the choice of the beach: stage 4. Decision 0142 named it for this step; it is moved because a beach is a choice between courses, which is stage 4's, and one fixed rule here would be undone there. A place that is not a coast hex is still refused.
- **Where a unit falls back to, and when the staff pulls men out:** the rules of a beaten side and of a spent attacker stand as they are; pulling out is "leave a place", step 8.
- **Holding the place once taken, feeding the men and the check that they can be fed:** the second part of this step.
- **Bombardment and cover before an attack:** steps 5 and 6.

## Rule card

- **Reads:** the side's own land units (men ashore, fatigue, posture, holds), its estimate of the enemy's men in the place's hex with its day, the ground figure `attack_fatigue_limit`, the transport force and the source, the intent's limits.
- **Writes:** nothing of its own. Posture, load, route and unload orders, and `set_limit` on the task the staff formed.
- **Reuses:** every rule of decision 0142; the posture order; the estimate of decision 0092.

## What it costs

- **State:** none. The doctrine version rises; no other version does. A game with no "take a place" plays as before, and scenario 67 gives the orders it gave: no enemy man is ashore in it, so the judgement says attack and every unit is on attack already.
- **The rulebook's chapter 8,** "Taking a place", gains "Ashore".

## The hand calculation

`tests/scenario_68_ashore.rs`.

## As built

Built on Nov 17, 2026, on the branch `stage3-step4`, with decision 0144 and the rulings on decision 0142's readings 4 and 8. Nothing is tuned and no figure changed. Every number of the hand calculation that an event, the state or a reason carries is asserted exactly, and no test is ignored: no mismatch was found. Scenario 67 gives the orders, events and numbers it gave, and every scenario built before this one gives the events it gave (the run baselines did not change).

### What was built

- **The judgement and the second lift (`crates/sim-staff/src/take.rs`).** `Day::launch_place` hands a released plan with a unit chosen ashore at the place to `Day::ashore` (the judgement, items 1 to 8 and 15, and the decision to send a second lift, items 9 to 13); the steps of a lift are `Day::move_step`, shared by the first lift (decision 0142, items 16 to 19) and later ones (item 11); `Day::left_behind` and `Day::lift_picks` are items 9 and 10. The mark and the fit men are `attack_share` and `fit_men`.
- **The ready task** (`crates/sim-staff/src/ready.rs`, `readies`) is given nothing from the first morning a unit of its limit has men ashore at the place, as well as from the morning its force is routed (item 11: a later lift goes as the force stands).
- **The notice that the intent is met (the ruling on reading 4).** The event `StaffNotice` gains, at the end, `intent`: the intent of a `met` notice whose plan is a "take a place" (`crates/sim-core/src/command/intake.rs`), `None` for every other. Its words are "intent:N is met" (`events.rs`); the export writes the key `intent` (`crates/sim-cli/src/export/events.rs`) and the command report says the same words (`crates/sim-agent/src/reporting.rs`). Events are not in the state hash: no state version changes.
- **Scenario 68** (`data/scenarios/68_ashore.toml`, on scenario 27's map, with its own ground figures in `data/reference/ground_placeholder_scenario_68.toml`) and its 17 tests (`tests/scenario_68_ashore.rs`): the hand calculation in nine, each property in its own.
- **Versions.** The doctrine `supply-23` for this record (`supply-24` with decision 0144, which is what the branch carries). Nothing else rises for this record.

### Readings taken

Where the decision leaves a detail open; the architect may rule on each.

1. **The units of the operation** are those in the forces limit of the "ready a landing" under the end (the first choice and every later lift's units, added there by item 12), with men ashore in the place's hex. A unit held, by a hold or by an order of the commander's, or before its day, is named in the reason with why and not counted.
2. **The judgement begins** on the first morning a unit of that limit has men ashore at the place (item 8's "from the first morning it has men ashore"), not on the first man of the side ashore (decision 0142's reading 5 still gives the phase "land").
3. **The estimate's day** is the day of the side's estimate (the report that last set it), so "600, of day 5" on morning 6.
4. **"The men ashore and the men taken" of item 10** are read as the fit men of the units that can attack and the men of the units taken, the same sum item 10 tests to send the lift at all; in the hand calculation both are 600.
5. **The ships of a second lift** are the force's ships that carry men, in number order, each unit taking ships until their berths hold its men, as in decision 0142, item 9; the force has nothing aboard, so every ship is free. Which ship the men go into is the load order's (the ruling on reading 6).
6. **"Nothing aboard"** is no supply, no fuel carried as cargo and no men on any ship of the force; "stopped at the source" is the supply doctrine's test of a force at a hex (`fleet::force_stopped_at`).
7. **A lift under way.** The units of a later lift are those of the task's limit not ashore at the place. While any is, the staff carries the lift on one step a morning (supply for their men for 15 days, a unit a morning, then the passage) while the force lies stopped at the source, and gives a reason that the force is on its way while it does not. The supply is for the men of the lift alone.
8. **The morning a lift is decided** the `set_limit` and the lift's first step (the load of its supply) are handed in together, as the hand calculation has it; the planned state takes the new limit at once.
9. **One reason a morning for the men.** Each posture order carries the judgement's reason; a morning with no posture order has one reason that says why (all that can attack are on attack, or the men are too few and none is on attack). Posture orders are in unit number order.
10. **With no estimate** the judgement says attack and a second lift is still sent while the place is not taken (item 10's "where no estimate is held").
11. **The route of a later lift** names in its reason that the force goes as it stands, with no landing made ready.

### A scenario file plays its game (the architect's addition to the brief)

A scenario file must play, run by itself, the game its test proves. Scenarios 66 and 67 named the live `data/reference/ground.toml` and their tests swapped in altered figures, so run from the file (`sim-cli run` or `export`) scenario 67 never took its place. Now each of scenarios 66 to 69 names a ground figures file of its own, `data/reference/ground_placeholder_scenario_NN.toml`: the frozen step 1 `ground.toml` with the figures its hand calculation names (the header of each lists them and marks it placeholder), frozen beside the others in `data/reference/step1/` with the frozen header (its `README.md` says so). Each test reads the file as it stands, through the frozen figures (`common::frozen()`), and no longer alters the main case; no property of the four alters a figure again. No expected number changed. Run by itself, scenario 66 meets its build-up on day 5 and readies its landing on day 8, scenario 67's base 2 changes side on day 10, scenario 68's on day 11 and scenario 69's on day 5, each with the notice that the intent is met the next morning. The run baselines of 66 and 67 change with their figures (they now play the tested game); every loop that walks `data/reference` or `data/scenarios` passes (`crates/sim-data/tests/frozen_figures.rs`, the run baselines, the export of every scenario and `node viewer/check.js`).

### Earlier tests touched

No expected number changed. The agent interface's version pins move to 20 with decision 0144. Scenarios 68 and 69 give no `experience` line (each unit is at the default, 2), so `tests/scenario_04_experience_values.rs` is not touched. `tests/scenario_06d1_pilots_and_replacements.rs` and `tests/scenario_06d2_base_supply.rs` leave scenarios 68 and 69 out of the scenarios whose hashes are those of before their stage, as they left out 66 and 67. `tests/scenario_66_units_made_ready.rs` and `tests/scenario_67_taking_a_place.rs` read their scenario's own figures file in place of altering the live one (code only; their doc comments are untouched).

### Files to read

- `crates/sim-staff/src/take.rs`: `launch_place`, `ashore`, `left_behind`, `lift_picks`, `move_step`.
- `crates/sim-staff/src/ready.rs`: `readies`.
- `crates/sim-core/src/command/intake.rs` and `events.rs`: the notice's intent.
- `tests/scenario_68_ashore.rs`.
