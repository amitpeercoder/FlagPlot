# 0133: Two sightings in one pulse: a contact reported this pulse is refreshed only from a hex away

Status: settled by the architect on Nov 16, 2026, on the designer's word of the same day to mend the fault that the close of stage 2 named (decision 0128, "Closed on Nov 16, 2026"). It changes the rule by which a sighting report refreshes a contact (decision 0012, item 13) and adds one figure. Nothing is built.

Summary: A contact made or refreshed in this pulse is refreshed again in the same pulse only by a report within one hex of it, by a new figure; a report further off makes a contact of its own, so that two forces two hexes apart, sighted in one pulse, are held as two contacts and not as one at the place of the later report

Standing: Settled by the architect, Nov 16, 2026, on the designer's word. Not built

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
