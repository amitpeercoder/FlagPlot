# 0230: Supply at press: the floor of a supply the staff forms is never less than a day

Status: a plan by the architect, written on Nov 24, 2026, from a lead found while writing scenario 135 (decision 0229) and confirmed by a developer's reading the same day. It is to be read by a reviewer who has not seen it written before anything is built. It has no question for the designer.

Summary: A hold at press forms its supply with a floor of 0 days, and the supply doctrine sends a run only when a reading is below the floor's stock, which nothing can be below. So at press no run is ever sent, and the men of a place taken at press starve when what they landed with is gone. This makes the floor of a supply the staff forms the margin of its risk and never less than one day: 10, 5 and 1. One day is the least floor at which the doctrine's own timing never leaves a night short. One figure of doctrine; nothing in the engine.

Standing: Planned by the architect on Nov 24, 2026; to be reviewed, then built after decision 0229 and before the setup of stage 4's proof. Its statements and hand calculation are written (scenario 136)

## The fault

Decision 0144 (items 7 and 10) gave the check that a place can be held a margin by the hold's risk, 10 days at preserve the force, 5 at balanced and 0 at press, and made the same figure the floor of the "keep supplied" formed under the hold (`margin_days`, `crates/sim-staff/src/holding.rs`, read by its two places that form a supply). The supply doctrine orders a run when, without it, the reading of the day after it would arrive is below the floor's stock, the floor in days times the day's use. A reading is never below 0 t. With a floor of 0 days the test can never be met.

So at press: the take lands with its men the supply for the days until a run can follow and no margin, which is what press should mean; the hold forms "keep supplied, floor 0 days"; and the run never follows. Each morning the staff's reason says "not below the floor, so no run is needed".

**Seen in play.** Scenario 76 with the island at press, on main: 12 t land with 600 men on days 4 and 5; the place is out of food at the end of day 8; all 600 are unfed on the night of day 9 and 157 are lost by day 14. At balanced the same game sends 45 t on day 9 and loses nobody.

**Where it has not been seen.** No scenario on file has a hold at press: the scenarios at press have no hold. No arrangement of the 39 days sets a risk, so both sides there are at balanced. It would be met at the table, by a commander who orders a place taken or held at press.

A commander's own "keep supplied" cannot have a floor of 0 days: intake refuses it. The only floor of 0 in the game was the one the staff gave itself.

## The rule

1. **The floor of a "keep supplied" the staff forms is the margin of its risk in days, and never less than 1 day:** 10 at preserve the force, 5 at balanced, 1 at press. It holds for every supply the staff forms: under a hold of a base that is the side's, under a hold that waits for its base (decision 0159, A), and under an island for a base beside a place (decision 0229).
2. **The margin of the check that a place can be held is not changed:** 10, 5 and 0. At press the men land with the supply for the days until a run can follow and nothing over.
3. **A floor a commander sets is his.** Nothing of a commander's own supply intent changes.
4. **Nothing in the supply doctrine changes.** The test for a run, its load and its days are as they are. At press the load of a run is, as at any floor, what brings the stock to the floor on the day it arrives and the use of the interval after.
5. **The staff's words follow the figure:** "with a floor of 1 day (the least floor, at press)" where they now say "0 days (the margin at press)".

**Why one day and not another figure.** The doctrine checks the reading of the day after a run would arrive, and a run lands by day, after that day's night charge. A run is ordered on the first morning that reading is below the floor's stock; on the morning before it was not, so on the morning it is ordered the reading is no lower than the floor's stock less a day's use. For the night before the landing to be paid in full the reading must be no lower than nothing. That holds exactly when the floor is at least one day. One day is therefore the least whole floor at which, with a ship free, no night is short, and it is the figure for the risk that cuts it finest. It follows from the doctrine and is not tuned on anything.

**Why not a new test for a floor of 0** ("a run is needed when the reading checked has nothing left", the mend the ruling in decision 0229 first thought likely). Worked through, it leaves the night before the landing short whenever the stock is not a whole number of days of use, since the run is then ordered a day too late; and it is a second test in the doctrine beside the first. A figure does it with none.

## Rule card

| | |
| --- | --- |
| Reads | The risk read up the tree from a hold or an island, as today |
| Writes | The floor in the `form_intent` of a "keep supplied" the staff forms |
| Reuses | The supply doctrine whole; the margin by risk of decision 0144 |
| New | Nothing. One figure differs at press between the margin of the check and the floor |
| Draws | None |

## Determinism, and truth and belief

No draw, no state, no event, no change to the hash's layout. Nothing in `sim-core` changes: intake still takes a floor of 0 days under a hold (decision 0144) and under an island (decision 0229), and no staff now sends one. Those two allowances are left as they are, noted in `docs/rules/differences.md` as allowed and unused, and taken out in a clean-up, which would raise the rules label and is not worth doing alone. The rule reads the side's own command state and nothing of the enemy.

## What moves

- **The doctrine's label,** in every file that holds it, and the five recorded games pinned to it.
- **No game on file by what the staff does,** since none has a hold at press; the build confirms it by the baselines and lists any that does move in "As built".
- **Scenario 135's property "At press"** (decision 0229) says the supply under the island is formed with a floor of 0 days. With this rule it is 1 day. The statement is restated by the architect in that test's doc comment, dated, in this build's pull request: "intent 3 is formed with a floor of 1 day". Nobody else changes it.
- **The island's measure at press** counts a place on a day its days of supply are at least the floor of its supply, which was 0 days and is 1. No scenario on file reads it at press; scenario 136 does not assert it.
- **The rulebook:** chapter 8, "Holding a place" (The supply that follows; Men fed before it is taken), "Taking a place" where it gives the margins, and decision 0229's section when it is in; the glossary's entry for the margin of a risk, which gains the floor beside it.

## The versions

Decision 0220 takes rules label 33, doctrine `supply-60`, interface 55 and state v60. Decision 0229 takes rules label 34 and doctrine `supply-61`. This takes doctrine `supply-62`. The rules label, the state's layout and the interface do not move for it. The builder merges main first and takes the next label after whatever is then there.

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

One branch and one pull request, after the review of this plan and after decision 0229's build is merged.

1. The floor in the staff's forming of a supply, at each place it forms one, and the words of its reasons; the doctrine's label.
2. Scenario 136's tests filled; scenario 135's property as restated by the architect.
3. The baselines and the recorded games remade for the label; any game that moves listed.
4. The rulebook, the glossary and the row in `docs/rules/differences.md`, in the same pull request.
