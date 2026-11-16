# 0133: Two sightings in one pulse: a contact reported this pulse is refreshed only from a hex away

Status: settled by the architect on Nov 16, 2026, on the designer's word of the same day to mend the fault that the close of stage 2 named (decision 0128, "Closed on Nov 16, 2026"). It changes the rule by which a sighting report refreshes a contact (decision 0012, item 13) and adds one figure. Built on Nov 16, 2026 ("As built" below).

Summary: A contact made or refreshed in this pulse is refreshed again in the same pulse only by a report within one hex of it, by a new figure; a report further off makes a contact of its own, so that two forces two hexes apart, sighted in one pulse, are held as two contacts and not as one at the place of the later report

Standing: Settled by the architect, Nov 16, 2026, on the designer's word. Built

## Why

A sighting report refreshes the nearest contact within `merge_distance_hexes`, 2, that is fresh, and replaces its place, count and kind. Nothing else is compared.

In the Eastern Solomons the Ryujo detachment and the carrier striking force lie two hexes apart, about 69 nm. On seed 3 of the second proof, read in full, a carrier's search reported the detachment at (7,-9), 5 ships, as a new contact, in reach of the carriers. In the same pulse Henderson Field's search reported the striking force at (8,-11), 13 ships. It refreshed the first contact. The side ended the day holding one contact, at the farther hex and out of reach, and never struck. Both reports named the true hex. The same was found on seeds 9, 26 and 139, between two carrier groups as well as between a carrier and the base.

A staff that plots two reports in the same hours, 70 miles apart, of 5 ships and of 13, plots two forces. The game plots one and throws the first away.

Between pulses the rule is right: a force moves, and a report two hexes from where it was last seen is most likely the same force. Within a pulse it has not moved. What separates two reports of one force in one pulse is the error of a report, and a report names its own hex or a neighbouring one (`search.toml`, the chance a report names a neighbouring hex).

## The rule

1. **A contact made or refreshed in this pulse** is refreshed again in the same pulse only by a report within `search.toml: merge_distance_same_pulse_hexes` of it. The live figure is 1, a placeholder: it is the furthest a report's error moves it.
2. **A contact not yet reported in this pulse** is refreshed as before, by a report within `merge_distance_hexes`.
3. **The rest is unchanged:** the freshness test, the nearest contact that qualifies, the lowest number on a tie, the report replacing what the contact held, and a new contact where none qualifies.
4. **Every report goes through it,** a strike's report among them, as before.
5. **With no figure the rule is as before.** The frozen step 1 figures have none, so every hand calculation on them stands.

## What it does not mend

- **Two reports of one force in one pulse, each a hex out in opposite directions,** make two contacts. The second is a ghost. It is refreshed by nothing and is dropped at 5 pulses, as any contact is. A staff of 1942 had ghosts too.
- **Two forces within a hex of each other** are still one contact.
- **A report is not compared by count or kind.** A report's count and kind are too often wrong to tell forces apart by them.

## Rule card

- **Reads:** a side's contacts (hex, age), the contacts made or refreshed this pulse (`refreshed_this_pulse`, which exists), the report's hex, two figures.
- **Writes:** a side's contacts, as before.
- **Reuses:** the one way a belief changes, `BeliefState::apply_report`. No new state: the set of contacts refreshed this pulse is already kept.

## What is checked again

Live games with two reports in one pulse play differently. Their recorded outputs are remade. The measures of the tuned battles that have search in them (Savo Island, the Eastern Solomons), of the air campaign and of the 39-day slice are run before and after and every measure whose standing changes is reported. A measure that agreed and now misses comes to the architect before anything is merged. Cape Esperance, the check, is not run.

## What it costs

- **A figure** in `search.toml` and its loader. The rules label rises: a live game plays differently. The state layout does not change.
- **Rulebook chapter 3,** "A sighting report makes or refreshes a contact", and the line that a later group's report in the same pulse can merge into a contact an earlier group made.

## The hand calculation

`tests/scenario_60_two_sightings_in_one_pulse.rs`.

## As built

Built on Nov 16, 2026, in one step. The rules label is 18; the state layout stays `flagplot-state-v34`, the saved game version 9, the doctrine and the agent interface as they were. The figure is `search.toml: merge_distance_same_pulse_hexes`, 1 in the live file, marked a placeholder; it is optional, the frozen step 1 file has none, and `crates/sim-data/tests/frozen_figures.rs` names it among the keys the frozen files go without. The rule is in `BeliefState::apply_sighting` (`crates/sim-core/src/belief.rs`): a contact in the set of contacts made or refreshed this pulse is taken within the new figure, any other within `merge_distance_hexes`; the rest is unchanged. Rulebook: chapter 3 (A sighting report makes or refreshes a contact; the line on a later group's report in the same pulse; a strike's report; starting contacts too near together); the glossary's "Fresh".

**The set of contacts reported this pulse.** Checked before building: `refreshed_this_pulse` gains every contact made or refreshed by a sighting, and every sighting goes through `apply_report`: search, a fight's report, a bombardment's and a strike's report at its target, and the contacts a scenario starts a side with. It is cleared by the ageing at the end of every pulse, night included, and a contact dropped by a "nothing there" report leaves it. Starting contacts of one age are entered as the reports of one pulse, so with the live figure two of the same age need only be more than a hex apart to load.

**The cases.** `BeliefState::apply_report` cannot be driven from the tests crate, so cases A to H, the distances and the property of the same reports in the same order are unit tests in `belief.rs`, each named `scenario_60_...`; the scenario file holds the battle property. Every outcome of A to H agrees with the hand calculation. Two readings did not:

- **Ages.** Case E says contact 2 "is 2 pulses old at the end of pulse 2"; the code gives 1, and cases C, D and F's "1 pulse old" in pulse 2 is 0 in the code. A contact is not aged at the end of the pulse that made it, as the hand calculation's own figures say. No outcome turns on it. The test of E's age is marked ignored, with what the code gives.
- **The battle.** On seed 3 of `second-proof/05a_eastern_solomons.toml`, live figures, the Allied side holds two contacts at the end of the morning of day 1, as the hand calculation says, but at (7,-8) and (8,-11), not (7,-9) and (8,-11). The first report made contact 1 at (7,-9) and the report at (8,-11) made contact 2. The carriers then struck contact 1 in the same pulse, sixteen, seven, fifteen and seven aircraft from air groups 3, 4, 7 and 8 at 160 nm, and the strike's four reports refreshed it, each within a hex; the last named (7,-8). The test as written is marked ignored, with what the code gives; a second test asserts what the build found (two contacts, one at (8,-11), made by the reports at (7,-9) and (8,-11)). Before this decision the same seed held one contact, contact 1 refreshed to (8,-11), and no Allied group struck that day; every strike launched was the Japanese side's. After it, the Allied carriers strike in the morning and again in the afternoon.

**An earlier unit test's setup.** `surface::tests::alert_choice_takes_the_nearest_fresh_contact_and_the_lowest_id_on_a_tie` sets the merge distance to 0 so that three reports of one pulse stay three contacts; on the live figures two of them, a hex apart, now merged by the new figure. Its setup sets the new figure to 0 as well; its expected answer is unchanged. Every hand-calculation test in `tests/` passes unchanged.

**Recorded outputs remade** (`scripts/regenerate.sh`): the run baselines of scenarios 5, 8 and 45 changed beyond their hash lines; the pinned hashes; the batch fixture; and the five recorded games, whose saves gain the rules label 18 and the new hash of `search.toml`, and of which only `second_proof_week_sonnet` played differently (from day 18; its result is unchanged). `replay --check-staff` passes on each.

**The measures,** 1,000 seeds each, in release, by the build of `e141c6a8` (before) and by this one (after), judged against the same files:

- **Savo Island** (`savo_island.toml`): the same report, 21 agree, 4 at the edge, 4 miss of 29.
- **The air campaign** (`06_rabaul_guadalcanal`, `rabaul_guadalcanal.toml`): the same report, 7 agree, 2 at the edge, 1 miss of 10.
- **The Eastern Solomons** (`eastern_solomons.toml`): 11 agree, 2 at the edge, 2 miss of 15 before; 12, 1, 2 after. One standing changed: `japanese_carriers_sunk` (record 1) went from agrees at the edge to agrees, its runs holding the record 249 to 272 of 1,000. Other values moved within their standing: `us_aircraft_attacking_ryujo` median 0 to 38, runs holding 488 to 541; `us_aircraft_left_to_attack_ryujo` median 0 to 37, 273 to 309; `bomb_hits_on_enterprise` median 3 to 2, 105 to 104; `japanese_carrier_aircraft_lost_all_causes` 189 to 194 runs; `japanese_carrier_aircraft_lost_to_enemy_action` 234 to 233; `japanese_aircraft_attacking_enterprise_force` 611 to 609; `japanese_aircraft_claimed_per_hundred_shot_down` 67 of 980 to 65 of 981 (left out 20 to 19). Reported only: `japanese_bombers_left_to_attack_enterprise_force` 539 to 538; `us_hits_claimed_per_hundred_scored` median 140 to 142, 251 of 488 to 267 of 541. The Allied margin moved from 5th -3.75, median 1.55, 95th 12.5 to -3.7, 2, 12.5. No measure that agreed now misses.
- **The 39-day slice** (`07_full_slice`, `guadalcanal_landings.toml`): 4 agree, 0 at the edge, 2 miss of 6, before and after. Only the reported `ridge_japanese_men_lost` moved, 219 to 218 runs holding the record. The Allied margin moved from 5th 77.765333, median 96.205333, 95th 107.485333 to 78.315333, 96.265333, 107.585333.

Cape Esperance's check was not run.

**The strike at ships in the second proof,** reported and not a pass mark: on `second-proof/05a_eastern_solomons.toml`, seeds 1 to 1,000, in release, the Allied side launches at least one strike at ships on 767 seeds before and 822 after. The Japanese side does on 979 before and after.
