# Rabaul round 1: bomb hits on aircraft on the ground

**This figure has no check battle.** It is tuned on Rabaul against Guadalcanal, and no other battle can read it. An agreement here is a fit, not a check. Ruled by the architect on Nov 13, 2026 (decision 0082, M5; `data/validation/README.md`).

Rabaul against Guadalcanal, Nov 13, 2026. This is the first of the slow figures, in the order of the plan the architect approved. **Written before any run of the round.** The proposed move goes to the coordinating architect with its evidence before it becomes live.

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
