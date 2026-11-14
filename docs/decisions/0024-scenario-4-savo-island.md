# 0024, notes: The Savo Island scenario's later changes

Status: the notes of the scenario's later changes, ruled on Nov 13, 2026 (each note says by whom). They belong to decision 0024, `docs/decisions/0024-savo-island-scenario.md`, which says what is in the scenario and what is left out and is not rewritten; where a note differs from it, the note is later. The scenario now has four forces (Yunagi is a force of her own) and break-off levels read from the record, not the one column and the half-ships level that 0024 describes.

Summary: The later changes to scenario 4, in `0024-scenario-4-savo-island.md`: the break-off levels read from the record, and Yunagi as a force of her own, so four forces; Nov 13, 2026

Standing: Ruled by the coordinating architect

## Note of Nov 13, 2026: the break-off levels, read from the record

Ruled by the coordinating architect on the designer's delegation, before the first tuning round. A scenario's orders are set from fact rows before its battle's first round and are then left alone (`docs/step2-spec.md`, "Tuning", 8). The tuning rounds' records name these levels among the things not moved.

**The reading, which holds for every battle** (the same words are in decision 0072 for Cape Esperance). A break-off level is the commander's order. A row stating an order to hold or to fight on, or failing that the force's recorded conduct with no part of it leaving by choice at any loss, sets the level at 1. A row stating that the force withdrew by choice at a stated loss sets it at that share. A force with no such row keeps 0.5, the rule's default, and is marked invented.

**The three forces.** Each is read apart, on rows about that force.

| Force | Level | Rows |
| --- | --- | --- |
| Northern group | 1 | It fought where it stood and lost three of its five ships: `engagement_outcomes` 762, 763 and 764 (Vincennes, Quincy and Astoria sunk; Naval War College analysis, high, preferred). No row has any part of it leave by choice |
| Southern group | 0.5, invented | Neither test of the reading is met. **No order to hold.** The full text of Crutchley's special instructions was not found. What is quoted says how the groups act, not that they stand: "Either or both groups may be brought against the enemy depending on the size and composition of his force" and the Vincennes group "shall act independently of the Australia group but shall conform generally to the movements of the Australia group" (`engagement_outcomes` 3697, 3698; the Australian official history, medium, unconfirmed); and the 1950 analysis finds no provision "for night action in the event of a surprise raid by enemy surface ships detected only after they had gotten within gun range" (3529, low, unconfirmed). **Conduct.** The group was "dissolved into independent ships by 0150" (`engagement_events` 81). Chicago, its one cruiser left able to fight, "stood on to the westward to investigate this action and to enter it if occasion permitted" (`engagement_outcomes` 3639, her report as the analysis gives it; low, unconfirmed, and the scan is poor), made no enemy report (3711), and at 02:20 answered "Were but not now" (`engagement_events` 114). A cruiser that steams away from the fight on her captain's chosen course is not "no part of it leaving by choice", and her stated purpose is not a withdrawal at a stated loss. Patterson "did not cease fire till about 0200, when the Japanese cruisers turned north" (`engagement_events` 742, unconfirmed). So the default stands, and these rows are why |
| Eighth Fleet | 0.5, invented | Mikawa ordered the withdrawal at 02:20 with no ship sunk or crippled (`engagement_events` 110). That is a time and an order, not a share of loss, and a level cannot say it |

The southern group was read twice: once on the rows of Nov 12, 2026, and once more when the research on orders and conduct arrived (rows 3515 to 3858 of `engagement_outcomes`, `docs/research/oob/orders-and-conduct.md`). It stays at 0.5 both times. No level moves after the first tuning round.

**How the ruling was made.** It followed a scratch run, and it rests on the rows and not on the run: the 0.5 was never a row, and "never left by choice" is the record's statement of what the northern group did. It was not chosen for the length of the action, which it makes worse. The table is the effect of the northern group's level alone, over 1,000 seeds each, on the figures of main on Nov 13, 2026 with decisions 0070 and 0071 in. Each cell is the 5th to the 95th percentile, then the median; shells are the main battery's, in hundreds.

| Measure | The record | Northern group at 0.5 | Northern group at 1 |
| --- | --- | --- | --- |
| Minutes of action | 20 to 30 | 20 to 60, 35 | 30 to 60, 40 |
| Surface fights by day | 0 | 0 to 4, 0 | 0 to 4, 0 |
| Allied cruisers sunk | 4 | 0 to 1, 0 | 0 to 2, 1 |
| Southern group cruisers sunk | 1 | 0 to 1, 0 | 0 to 1, 0 |
| Northern group cruisers sunk | 3 | 0 to 0, 0 | 0 to 2, 0 |
| Allied destroyers sunk | 0 | 1 to 4, 3 | 1 to 4, 3 |
| Japanese ships sunk | 0 | 0 to 1, 0 | 0 to 1, 0 |
| Gun hits on Allied ships | 153 to 219 | 32 to 100, 74 | 32 to 137, 95 |
| Gun hits on Japanese ships | 10 to 17 | 2 to 51, 16 | 4 to 52, 17 |
| Torpedo hits on Allied ships | 4 to 9 | 1 to 7, 3 | 1 to 7, 3 |
| Torpedo hits on Japanese ships | 0 | 0 to 4, 1 | 0 to 4, 1 |
| Japanese shells | 10 to 11 | 8 to 18, 13 | 8 to 21, 17 |
| Allied shells | 3 | 1 to 11, 5 | 2 to 11, 5 |
| Southern group shells | 0 | 0 to 3, 0 | 0 to 3, 0 |
| Northern group shells | 3 | 1 to 8, 4 | 1 to 8, 4 |
| Tenryu and Yubari shells | 1 | 2 to 3, 2 | 2 to 4, 3 |

What got worse: the minutes of action (the median is five more from the record, and the shortest runs are now at its top), Japanese shells, and Tenryu and Yubari's. What got better: gun hits on Allied ships, and the northern group's cruisers can now be sunk, which they could not be in 19 runs of 20. Allied destroyers and the torpedo hits did not move. What the worse rows show is named and left: the Japanese ended each fight by passing on, and no rule has a force that has done what it came for retire. That would be a rule, planned for the designer, if the tuning rounds show it is needed.

**The hand calculation.** The scoring of Savo Island in `tests/scenario_10b_objectives.rs` was worked on the scenario as it stood. It now runs on a frozen copy, `data/scenarios/step2-frozen/04_savo_island.toml`, and no number of it changes ("Tests while figures move", 9). The scenario's own hand calculation (`tests/scenario_04_savo_island.rs`) reads no break-off level and runs on the live file as before.

## Note of Nov 13, 2026: Yunagi in a task force of her own

Ruled by the coordinating architect on the designer's delegation, as scenario data from rows, set with decision 0078 and before the reports tuning starts from were remade. It is not moved again.

**The rows.** Yunagi, the column's one destroyer, took no part in the cruiser action. She "engaged Jarvis, turned away from the cruiser action" (`engagement_outcomes` 158, medium, preferred). She "opened fire on the JARVIS" at 01:55 and broke off about 02:00 (`engagement_events` 1061 and 1062, the 1950 analysis, medium, unconfirmed). Her one searchlight was seen sweeping from about 15,000 yards at 02:10 (`engagement_events` 105 and 108). Jarvis, damaged the day before and steaming away alone, is not in this scenario.

**The scenario.** She is task force 4, starting with the column at (-1,-8) at 25 knots, with a route to (1,-5), the hex west of the hex of Savo Island, and back to (-1,-8). She meets neither Allied group and is home at 07:55. That she was with the column until about 01:55 is the rows'. The hex she turns in is the architect's choice: the last hex before the column's fight. Her ship id is 8 as before, and every measure that lists ships 1 to 8 reads as it did: the record's Japanese shells have none of hers ("Yunagi's main battery has no row"), and the record has her take no hit.

**How it came to be looked at.** After the prototypes of decision 0078 had been seen: under that rule a destroyer's guns go at destroyers, so with Yunagi in the column the four Allied destroyers all fired at her. It rests on the rows above and not on the prototypes.

**The frozen copy** (`data/scenarios/step2-frozen/04_savo_island.toml`) keeps her in the column, as it stood. The scenario's own hand calculation (`tests/scenario_04_savo_island.rs`) now runs on that copy, as its scoring test does, so neither moves with the live scenario.
