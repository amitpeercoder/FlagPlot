
## Note of Nov 13, 2026: the break-off levels, read from the record

Ruled by the coordinating architect on the designer's delegation, before the first tuning round. A scenario's orders are set from fact rows before its battle's first round and are then left alone (`docs/step2-spec.md`, "Tuning", 8). The tuning rounds' records name these levels among the things not moved.

**The reading, which holds for every battle** (the same words are in decision 0072 for Cape Esperance). A break-off level is the commander's order. A row stating an order to hold or to fight on, or failing that the force's recorded conduct with no part of it leaving by choice at any loss, sets the level at 1. A row stating that the force withdrew by choice at a stated loss sets it at that share. A force with no such row keeps 0.5, the rule's default, and is marked invented.

**The three forces.** Each is read apart, on rows about that force.

| Force | Level | Rows |
| --- | --- | --- |
| Northern group | 1 | It fought where it stood and lost three of its five ships: `engagement_outcomes` 762, 763 and 764 (Vincennes, Quincy and Astoria sunk; Naval War College analysis, high, preferred). No row has any part of it leave by choice |
| Southern group | 0.5, invented | No row gives a level. Crutchley's special instructions as keyed (`engagement_events` 20: "Each group acts independently if both are attacked") hold no order to hold a station. The group was "dissolved into independent ships by 0150" (row 81), and Chicago, its one cruiser left able to fight, answered at 02:20 "Were but not now" and at 02:45 was standing toward Lengo (row 114), with nothing on how she left the action. Patterson "did not cease fire till about 0200, when the Japanese cruisers turned north" (row 742, unconfirmed) |
| Eighth Fleet | 0.5, invented | Mikawa ordered the withdrawal at 02:20 with no ship sunk or crippled (`engagement_events` 110). That is a time and an order, not a share of loss, and a level cannot say it |

The research session has been asked for the text of paragraphs 3(b) and 6 of the special instructions and for Chicago's track from 01:50 to 02:20. Rows that arrive before the first tuning round are read once more by the same reading; none after it moves a level.

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
