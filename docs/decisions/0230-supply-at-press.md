# 0230: Supply at press: the floor of a supply the staff forms is never less than a day

Status: a plan by the architect, written on Nov 24, 2026, from a lead found while writing scenario 135 (decision 0231) and confirmed by a developer's reading the same day. It is to be read by a reviewer who has not seen it written before anything is built. It has no question for the designer.

Summary: A hold at press forms its supply with a floor of 0 days, and the supply doctrine sends a run only when a reading is below the floor's stock, which nothing can be below. So at press no run is ever sent, and the men of a place taken at press starve when what they landed with is gone. This makes the floor of a supply the staff forms the margin of its risk and never less than one day: 10, 5 and 1. One day is the least floor at which the doctrine's own timing never leaves a night short. One figure of doctrine; nothing in the engine.

Standing: Planned by the architect on Nov 24, 2026; amended on review the same day. Its statements and hand calculation are written (scenario 136). Built and merged before stage 4's proof (pull request 728)

## The fault

Decision 0144 (items 7 and 10) gave the check that a place can be held a margin by the hold's risk, 10 days at preserve the force, 5 at balanced and 0 at press, and made the same figure the floor of the "keep supplied" formed under the hold (`margin_days`, `crates/sim-staff/src/holding.rs`, read by its two places that form a supply). The supply doctrine orders a run when, without it, the reading of the day after it would arrive is below the floor's stock, the floor in days times the day's use. A reading is never below 0 t. With a floor of 0 days the test can never be met.

So at press: the take lands with its men the supply for the days until a run can follow and no margin, which is what press should mean; the hold forms "keep supplied, floor 0 days"; and the run never follows. Each morning the staff's reason says "not below the floor, so no run is needed".

**Seen in play.** Scenario 76 with the island at press, on main: 12 t land with 600 men on days 4 and 5; the place is out of food at the end of day 8; all 600 are unfed on the night of day 9 and 157 are lost by day 14. At balanced the same game sends 45 t on day 9 and loses nobody.

**Where it has not been seen.** No scenario on file has a hold at press: the scenarios at press have no hold. No arrangement of the 39 days sets a risk, so both sides there are at balanced. It would be met at the table, by a commander who orders a place taken or held at press.

A commander's own "keep supplied" cannot have a floor of 0 days: intake refuses it. The only floor of 0 in the game was the one the staff gave itself.

## The rule

1. **The floor of a "keep supplied" the staff forms is the margin of its risk in days, and never less than 1 day:** 10 at preserve the force, 5 at balanced, 1 at press. It holds for every supply the staff forms: under a hold of a base that is the side's, under a hold that waits for its base (decision 0159, A), and under an island for a base beside a place (decision 0231).
2. **The margin of the check that a place can be held is not changed:** 10, 5 and 0. At press the men land with the supply for the days until a run can follow and nothing over.
3. **A floor a commander sets is his.** Nothing of a commander's own supply intent changes.
4. **Nothing in the supply doctrine changes.** The test for a run, its load and its days are as they are. At press the load of a run is, as at any floor, what brings the stock to the floor on the day it arrives and the use of the interval after.
5. **The staff's words follow the figure:** "with a floor of 1 day (the least floor, at press)" where they now say "0 days (the margin at press)".

**Why one day and not another figure.** The doctrine checks the reading of the day after a run would arrive, and a run lands by day, after that day's night charge. A run is ordered on the first morning that reading is below the floor's stock; on the morning before it was not, so on the morning it is ordered the reading is no lower than the floor's stock less a day's use. For the night before the landing to be paid in full the reading must be no lower than nothing. That holds exactly when the floor is at least one day. One day is therefore the least whole floor at which, with a ship free, no night is short, and it is the figure for the risk that cuts it finest. It follows from the doctrine and is not tuned on anything.

**Why not a new test for a floor of 0** ("a run is needed when the reading checked has nothing left", the mend the ruling in decision 0231 first thought likely). Worked through, it leaves the night before the landing short whenever the stock is not a whole number of days of use, since the run is then ordered a day too late; and it is a second test in the doctrine beside the first. A figure does it with none.

## Rule card

| | |
| --- | --- |
| Reads | The risk read up the tree from a hold or an island, as today |
| Writes | The floor in the `form_intent` of a "keep supplied" the staff forms |
| Reuses | The supply doctrine whole; the margin by risk of decision 0144 |
| New | Nothing. One figure differs at press between the margin of the check and the floor |
| Draws | None |

## Determinism, and truth and belief

No draw, no state, no event, no change to the hash's layout. Nothing in `sim-core` changes: intake still takes a floor of 0 days under a hold (decision 0144) and under an island (decision 0231), and no staff now sends one. Those two allowances are left as they are, noted in `docs/rules/differences.md` as allowed and unused, and taken out in a clean-up, which would raise the rules label and is not worth doing alone. The rule reads the side's own command state and nothing of the enemy.

## What moves

- **The doctrine's label,** in every file that holds it, and the five recorded games pinned to it.
- **No game on file by what the staff does,** since none has a hold at press; the build confirms it by the baselines and lists any that does move in "As built".
- **Scenario 135's property "At press"** (decision 0231) says the supply under the island is formed with a floor of 0 days. With this rule it is 1 day. The statement is restated by the architect in that test's doc comment, dated, in this build's pull request: "intent 3 is formed with a floor of 1 day". Nobody else changes it.
- **The island's measure at press** counts a place on a day its days of supply are at least the floor of its supply, which was 0 days and is 1. No scenario on file reads it at press; scenario 136 does not assert it.
- **The rulebook:** chapter 8, "Holding a place" (The supply that follows; Men fed before it is taken), "Taking a place" where it gives the margins, and decision 0231's section when it is in; the glossary's entry for the margin of a risk, which gains the floor beside it.

## The versions

Decision 0220 takes rules label 33, doctrine `supply-60`, interface 55 and state v60. Decision 0231 takes rules label 34 and doctrine `supply-61`. This takes doctrine `supply-62`. The rules label, the state's layout and the interface do not move for it. The builder merges main first and takes the next label after whatever is then there.

## The case, the scenario and its test

`tests/scenario_136_supply_at_press.rs`, with `data/scenarios/supply-at-press/136_supply_at_press.toml`, which is scenario 76 with the island at press, in a folder of its own so that no loop reads it before it is built (decision 0072). Nine tests, each empty and marked "statement written, not built". In short:

- The supplies are formed with a floor of 1 day. The landing is scenario 76's with 12 t of supply in place of 27 t (3 t a day for the 4 days until supply follows, and no margin).
- The supply of the place is first served on day 6: the stock is 9 t and the reading checked 3 t, not below 3 t. On day 7 the stock is 6 t and the reading 0 t: a run of 0 + 15 x 3 = 45 t is ordered, loads that day, and lands 40 t and 5 t on day 8, the day the last 3 t were eaten in the night.
- No man is unfed on any night. The place holds 27 t at the end and the source 934 t. Ten orders.
- Properties: no night short with the unit at 500, 550, 650 and 700 men; nothing moves at balanced, and the floor is 10 days at preserve; a commander's own floor stands; the same twice.

The run and every stock were worked by hand and then checked on the engine as it stands, by a copy in which the commander states the supply of the place himself at a floor of 1 day, which the doctrine serves as it would the staff's. They agreed.

## How it is judged

1. Scenario 136's test passes as written.
2. Scenario 76 and every scenario with a hold play as before, by the baselines.
3. Scenario 135's restated property passes.

It is no condition of stage 4's proof and changes nothing the proof reads.

## What it leaves out

- **What press should buy.** Here it buys nothing but a smaller cushion: less landed with the men and a later, finer run. Whether a side at press should also run by day, or load before its men are ashore, is not asked.
- **The two allowances of intake** for a floor of 0 days, left unused.
- **A supply at press with no ship free on the day** goes short, as at any risk, and says so by `cannot_meet`.

## Building steps

One branch and one pull request, after the review of this plan and after decision 0231's build is merged.

1. The floor in the staff's forming of a supply, at each place it forms one, and the words of its reasons; the doctrine's label.
2. Scenario 136's tests filled; scenario 135's property as restated by the architect.
3. The baselines and the recorded games remade for the label; any game that moves listed.
4. The rulebook, the glossary and the row in `docs/rules/differences.md`, in the same pull request.

## Amended on review, Nov 24, 2026

The plan was read by a reviewer who had not seen it written, against the code. The reviewer found the rule sound as one figure in `sim-staff` with nothing in `sim-core`, did the hand calculation again and had the same numbers, and found the forming sites to be the two in `holding.rs` and the check's margin in `take.rs` alone. Five things were wrong or unsaid. What follows is the architect's, each checked in the file named, and replaces the text above where they differ.

1. **"No scenario on file has a hold at press" was wrong.** Two have:
   - **Scenario 69,** the property "Press" (`tests/scenario_69_taken_and_held.rs`): both intents at press, and its statement says the supply is "formed with a floor of 0 days", which its test asserts on day 6. The statement is restated by the architect, to be put in that doc comment with this build and its assertion made to match: "**Press.** With both intents at press: the margin is 0, the load of day 2 is 6 x 4 = 24 t, and intent 4 is formed with a floor of 1 day. (Restated by the architect on Nov 24, 2026, for decision 0230: the first writing gave a floor of 0 days.)" The load of 24 t does not move: it is the check's margin, which stays 0.
   - **Scenario 90,** the property "At press it goes" (`tests/scenario_90_a_take_that_does_not_go_to_starve.rs`): an island at press, played for 8 days. Its statement is of day 1 and stands, but the game moves after it: the hold's supply is formed at 1 day and a run may be sent. The build says what moved.

   "What moves" above is corrected so: no baseline or recorded game is expected to move, since these are variants made in their tests; the build confirms it.
2. **The reason for one day holds under two conditions, and they are now stated.** The argument takes the morning before the run was ordered to have been weighed with the same days of loading and passage. It fails where there was no morning before, on the first day a supply is served, and where the days moved between the two mornings, as when a larger load takes a day more to load. In either case a night may be short at press. This is accepted and not ruled out: it is the same at every floor, where a supply first served late or a run that slips eats into the floor, and a floor of one day is the one with nothing to eat into, which is what the commander asked for when he set press. So the claim is: with a ship free, the supply served the morning before, and the run's days the same on both mornings, no night is short. The record's "never leaves a night short" in the summary is read with that.
3. **The property "Never a night short" is restated,** in scenario 136's doc comment, before anything is built. Changing the unit's men could not run its case: the men land with four days of their own need and one night is paid before the supply is first served, so the stock is three days at any size. The case is now made by giving the enemy's base 3 t, of which half is taken with it: the stock is 7.5 t, two and a half days, on the morning the run is ordered, and no night is short. It was checked on the engine as the main case was.
4. **One function gives the floor.** `margin_days` stays as it is and is read by the check alone. A second function beside it gives the floor of a supply the staff forms, the margin and not less than 1, and every site that forms a supply calls it: the two of `holding.rs`, and the site decision 0231 adds for a base beside a place. Decision 0231's build is under way and calls `margin_days` there, as the holds do today; it is not asked to change. This build, made on main with 0231 merged, changes all three.
5. **Scenario 135 at press** (decision 0231) moves in two intents, not one: the supply of the base beside the place and the supply under the hold of the place. Its property is restated by the architect, to be put in that doc comment with this build: "**At press.** With intent 1's risk set to press on day 1: intents 3 and 5 are formed, each with a floor of 1 day, and neither is refused. Only the forming is asserted. (Restated by the architect on Nov 24, 2026, for decision 0230: the first writing gave intent 3 a floor of 0 days.)" Intake's allowance of a floor of 0 under an island stays tested in `sim-core`'s own tests, where a command is handed in directly.
6. **The reason's words are stated and asserted** (rule 5): where a supply is formed at press the reason says "with a floor of 1 day (the least floor, at press)"; at balanced and at preserve the words are as they are, "(the margin at balanced)" and "(the margin at preserve the force)". Scenario 136's first test asserts the words at press.
7. **Where the doctrine's label is held:** the five recorded games under `crates/sim-cli/tests/recorded_games/`, `crates/sim-cli/tests/export_pictures.rs`, `docs/commands.md`, and the rulebook's chapters 1 and 8. Each is brought to the new label. `sim-agent`'s reading of an island (`crates/sim-agent/src/island.rs`), which gives the floor of a place's supply in the measure, reads the intent and needs no change; `sim-trial` reads no floor.
8. **What the sums rest on** is written into scenario 136's doc comment: the man's ration, the loss of the unfed, a day to load and a day's passage, the night's charge before the day's landing, a reading never below 0 t, the lower-numbered base paying first, and 15 days of use where no interval is set.


## As built, Nov 24, 2026

Built in `crates/sim-staff/src/holding.rs`: `staff_floor_days` (the margin, never less than 1) and `staff_floor_note` (the words), called from the two sites of `holding.rs` and from `beside.rs`, the site of decision 0231 (the renumbered decision on men beside a place; the review's "decision 0229"). `margin_days` is read by the check alone, in `take.rs`. Nothing in `sim-core`. Doctrine `supply-62`; the rules label (34), the state layout (v60), the saved game (28) and the interface (55) did not move. The reason at press says "with a floor of 1 day (the least floor, at press)"; at balanced and at preserve the words are as they were ("(the margin at balanced)", "(the margin at preserve)"). Scenario 136's nine tests are filled to the statements as written and pass; the case of a stock that is no whole number of days was checked on the engine. Scenario 69's "Press" (floor 1, `keep_kind(1)`) and scenario 135's "At press" (intents 3 and 5, each at 1 day) are restated as the review gave them and pass. Scenario 90's day-1 statement stands.

What moved: only the doctrine label in the five recorded games' saves (each save differs in `doctrine_version` alone; no transcript changed) and the tests' pins that name it. No run baseline, agent golden, pinned hash or batch fixture changed, so no game on file moved by what the staff does. The rulebook (chapter 8), the glossary, `docs/commands.md` and the recorded games' README say so. The two allowances of intake for a floor of 0 days stay, unused (noted in chapter 8).
