# Rabaul round 1: bomb hits on aircraft on the ground, 0.3 to 0.06

**This figure has no check battle.** It is tuned on Rabaul against Guadalcanal, and no other battle can read it. An agreement here is a fit, not a check. Ruled by the architect on Nov 13, 2026 (decision 0082, M5; `data/validation/README.md`).

Rabaul against Guadalcanal, Nov 13, 2026. This is the first of the slow figures, in the order of the plan the architect approved. **Written before any run of the round;** the sections from "What was run" on were added after it. **Ruled by the coordinating architect, Nov 13, 2026: 0.06.** The proposed move goes to the coordinating architect with its evidence before it becomes live.

## The figure

| File | Figure | Now |
| --- | --- | --- |
| `data/reference/air_combat.toml` | `[base_hit] aircraft` | 0.3 (placeholder, decision 0034) |

It is the chance that a bomb hit on a base lands on an aircraft on the ground. One draw is made for each hit. A hit on aircraft destroys one, damaged aircraft first, lowest group first (decision 0034, 7 and 8).

Nothing else moves. `bomb_hit_chance` (0.2) is an attack figure that every raid on ships reads. `[base_hit] supply` has no row and no measure.

## The rows that size it

**Aircraft destroyed on the ground at Henderson Field:**
- 2 Marine fighters in August (`engagement_outcomes` 1266).
- None of any type in September (1272, 1283, 1299).
- One dive bomber burned on 2 September, by a dispatch the statistics do not show (1330).

The measure `us_aircraft_destroyed_on_the_ground` takes 2 to 3 from these.

**Bomber sorties that reached the island:** 211 to 213 over the eleven raid days, from the units' forms. These are the rows of `bomber_sorties_that_reach_guadalcanal`.

So the record is about 2.5 aircraft destroyed for about 212 bomber sorties, about 0.012 a sortie.

In the rules each bomber drops one bomb at `bomb_hit_chance`, 0.2, and each hit lands on aircraft at this figure. That gives about 212 × 0.2 = 42 hits a run on the field, at the runs' own count of sorties (214 at the start of the round). The record's 2.5 is reached at about 2.5 / 42 = 0.06.

That is the size the rows point to. It rests on the rules' one bomb for each bomber and their bomb chance. Both are assumptions here: no row gives the bombs that hit Henderson Field.

## What is read beside it

The ground losses fall on the lowest-numbered groups with aircraft on the ground (decision 0034): VMF-223, then VMSB-232. Those are 7.0 and 5.6 a run before the round.

Since decision 0082 the dive bombers fly searches. So the round reads beside it the measures of aircraft ready:
- `vmsb_232_ready_on_day_5` (9);
- `sbds_ready_on_day_21` (22);
- `vmf_223_ready_on_day_5` (11) and `_day_10` (4 to 5);
- `f4fs_ready_on_day_21` (11).

The round says whether the ground losses were standing in for the losses the record gives to operations (rows 1281, 1282, 1295, 1298). If the dive bombers' counts rise above their records when fewer are destroyed on the ground, the ground losses were doing the wear's work. That gap is the wear round's, round 3 of this plan, and is not mended here.

The Japanese claims of aircraft on the ground (reported, not judged) move with the real count. They are reported.

## What stops the round

The round stops, and the figure stays at 0.3 with the reason written here, if either of these holds:
1. **No value works.** No value brings the record of 2 to 3 into the middle half of the runs.
2. **A ready measure breaks.** The value that does so turns a measure of aircraft ready from agreeing into a miss. Then the round names the operational losses as the cause and goes to the architect before moving.

## What was run

1,000 runs of Rabaul against Guadalcanal at each value, on scratch copies of the figures (5th to 95th, median, runs holding the record):

| `[base_hit] aircraft` | Destroyed on the ground, 2 to 3 | F4Fs ready on day 21, 11 | VMF-223 ready on day 10, 4 to 5 | SBDs ready on day 21, 22 | VMSB-232 ready on day 5, 9 | Japanese claims on the ground per hundred (reported) |
| --- | --- | --- | --- | --- | --- | --- |
| 0.3 (before) | 7 to 19, 13: miss | 6: miss (40) | 1: at the edge | 19 to 31, 26: at the edge (58) | 10: agrees | 260 |
| 0.1 | 1 to 8, 4: agrees (315) | 8: agrees (88) | 4: agrees | 26 to 33, 30: miss (1) | 10: agrees | 600 |
| 0.06 | 0 to 5, 2: agrees (476) | 9: agrees (89) | 5: agrees | 27 to 33, 30: miss (0) | 10: agrees | 900 |
| 0.04 | 0 to 4, 2: agrees (406) | 9: agrees (93) | 6: agrees | 27 to 33, 30: miss (0) | 10: agrees | 1,200 |

## The stop rule, and why the move was made

The record's second stop rule tripped. At the size the rows give, SBDs ready on day 21 turns from agreeing at the edge into a miss: 30 against 22.

The figure moves all the same, on the architect's ruling:

- **The old value agreed for the wrong reason.** It agreed on the dive bombers ready only because bombs on the field stood in for their operational losses. In September the record has those at 4 operational and 4 on other flights for the Marine bombers (`engagement_outcomes` 1281, 1282), and 1 to guns and 4 on other flights for the Navy's (1295, 1298).
- **A figure is not held at a wrong size to cover another figure's miss.**

**Handed to the wear round, not mended here:** SBDs ready on day 21 is a named miss, handed to the wear round (Rabaul round 3) with those rows. The dive bombers' searches under decision 0082 do not wear them as the record's operations did.

**Noted, not mended:** the Japanese claims of aircraft on the ground, 900 for each hundred destroyed (reported, not judged). Its divisor is now near the record's 2 to 3, and the bomber crews' claims stay a share of their bombs that missed (`bomb_false_claim_chance`, not moved). The record's 2,066 to 4,600 is a ratio over two or three aircraft.

## The move

| File | Figure | From | To |
| --- | --- | --- | --- |
| `data/reference/air_combat.toml` | `[base_hit] aircraft` | 0.3 | 0.06 |

**Its kind:** an attack-on-a-field figure, tuned on Rabaul with no check battle. An agreement here is a fit, not a check.

## The report

`docs/validation/rabaul-guadalcanal-air-rabaul-r1.md`, judged 10: agree 7, at the edge 1, miss 2. It was agree 5, at the edge 2, miss 3 after round 4.

- **Agree:** destroyed on the ground (2), F4Fs on day 21 (9), VMF-223 on day 10 (5), VMF-223 on day 5, VMSB-232 on day 5, American fighters lost (25), American claims (312).
- **At the edge:** Japanese fighters lost (13).
- **Miss:** Japanese bombers lost (29, the named miss of C3) and SBDs on day 21 (30, the wear round's).

With fewer fighters destroyed on the ground, more fly. So the fighting moves a little: Japanese bombers lost go from 26 to 29, and American fighters lost from 23 to 25.

The Eastern Solomons report does not change verdict, measure by measure: its one raid on a field is six bombers.
