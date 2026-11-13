# Rabaul round 2: the fatigue of a long sortie, against raid sizes by day. Nothing moved

**This figure has no check battle.** It is tuned on Rabaul against Guadalcanal and no other battle can read it. An agreement here is a fit, not a check (`docs/step2-spec.md`, "Tuning": the figures that take weeks to show).

Rabaul against Guadalcanal, Nov 13, 2026. This is the second of the slow figures, in the order of the plan the architect approved. **Written before any run of the round;** the sections from "What was run" on were added after it. **Ruled by the coordinating architect, Nov 13, 2026: no move.** The proposed move goes to the coordinating architect with its evidence before it becomes live.

## The figure

| File | Figure | Now |
| --- | --- | --- |
| `data/reference/fatigue.toml` | `air_fatigue_per_100_nm` | 2 (placeholder, decision 0036) |

**What it does.** A group that flies gains `air_fatigue_per_sortie` (3) plus this figure times its longest sortie over 100 nm, once a pulse. A group that does not fly loses `air_rest_per_pulse` (5) a daylight pulse. A group flies (100 − fatigue) / 100 of its ready aircraft (decision 0036).

**Its effect at Rabaul.** A raid from Rabaul is a sortie of 1,112 nm out and back, so it adds 3 + 2 × 11.12 = 25.2 fatigue, and the afternoon after rests 5. On consecutive raid days each raid flies about a quarter fewer aircraft than the one before.

**Why this figure and not the others.**
- The distance term is what the long raid reads. The other two fatigue figures are read by every group that flies at all, the patrols over Henderson Field among them, where nothing about raid sizes speaks to them.
- One figure is moved, and the other two stay placeholders.
- The scenarios that read the fatigue file are 6, 6c, 6c2, 7 and 8. Only scenario 6 has a measure. 6c and 6c2 run on the frozen figures and are not moved by it.

## The rows that give the target

**The bombers in each raid that reached Henderson Field, from the units' forms.** These are the rows of `bomber_sorties_that_reach_guadalcanal`: 1115, 1044, 982, 1047, 987, 1059, 1005, 1065, 1008, 1075, 1011, 1080, 1015, 1085, 1019, 1089, 1023, 1094, 1028, 1100, 1033, 1108, 1037.

| Day of the scenario | 5 | 6 | 9 | 10 | 13 | 16 | 20 | 21 | 22 | 23 | 24 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Date | 25 Aug | 26 Aug | 29 Aug | 30 Aug | 2 Sep | 5 Sep | 9 Sep | 10 Sep | 11 Sep | 12 Sep | 13 Sep |
| Forms | 23 | 16 | 18 | 18 or 20 | 18 | 18 | 18 | 22 | 24 | 20 | 16 |
| Runs before the round (median) | 40 | 26 | 31 | 19 | 21 | 22 | 28 | 18 | 8 | 1 (in 394 runs) | 1 (in 4) |

The forms are the firmer source: one form a unit a mission, read twice. They show that the bomber units held 16 to 24 a raid through five raids on consecutive days, 9 to 13 September. The runs fly 28, 18, 8, then almost nothing.

The round's target is the run of consecutive days, days 20 to 24.

## Named and not tuned to

- **Rows 3976 and 3977.** These are one officer's recollection: 80% of the aircraft available the day after an all-out attack, 50% on the third. He was the senior staff officer of the 25th Air Flotilla, interrogated after the war, and spoke of 1942 in general, tying it to supply. They are named, not tuned to. The forms' September run does not fall that way.
- **The first raid's 41 against 23.** The group holds all three units' bombers and launches every one it can fly. How many each unit sent is a scenario order, which is not changed after tuning began (tuning rule 8). It is named.
- **Wear and repair.** They also take bombers out of the raids (1,112 nm at the placeholder wear figures). They are the next round's, and are not moved here.

## What judges the round

No judged measure reads raid sizes. `bomber_sorties_that_reach_guadalcanal` (211 to 213) and `raid_days_that_reach_guadalcanal` (11) are reported.

The round reads the table above against the forms, and reports every judged measure of the battle before and after. A move that brings the consecutive days nearer the forms changes how many bombers and escorts meet the patrol. So the losses and the aircraft ready move with it, and they are reported, not chased.

## What stops the round

The figure stays at 2, with the reason written here, if either of these holds:

1. **No value fits days 20 to 24.** No value at or above 0 brings the medians of days 20 to 24 within the forms' range of 16 to 24 without taking the early raids (days 5 to 16, with rest days between them) further from the forms than they are now.
2. **The fit needs zero.** The fit needs a value of 0 or below, which would mean a long sortie tires a group no more than a short one, against the figure's meaning. The round then names fatigue as not the cause, and goes back to the architect.

## What was run

1,000 runs at each value on scratch copies of the figures. The table gives the median bombers attacking on each raid day, then two of the judged measures.

| `air_fatigue_per_100_nm` | d5 | d6 | d9 | d10 | d13 | d16 | d20 | d21 | d22 | d23 | d24 | Bomber sorties, 211 to 213 (reported) | Japanese bombers lost, 12 | American fighters lost, 7 to 23 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| The forms | 23 | 16 | 18 | 18 or 20 | 18 | 18 | 18 | 22 | 24 | 20 | 16 | 211 to 213 | 12 | 7 to 23 |
| 2 (now) | 40 | 26 | 31 | 19 | 21 | 22 | 28 | 18 | 8 | 1 (in 394 runs) | 1 (in 4) | 212 | 29: miss | 25: agrees |
| 1.75 | 40 | 27 | 33 | 21 | 25 | 28 | 34 | 24 | 15 | 8 | 2 | about 255 | 34: miss | 27: at the edge |
| 1.5 | 40 | 28 | 35 | 24 | 29 | 32 | 41 | 29 | 20 | 13 | 6 | about 300 | 39: miss | 29: at the edge |
| 1.25 | 40 | 29 | 37 | 26 | 33 | 38 | 41 | 29 | 21 | 15 | 9 | about 320 | 40: miss | 29: at the edge |
| 1 | 40 | 29 | 39 | 28 | 36 | 37 | 41 | 30 | 23 | 17 | 11 | 329 | 40: miss | 29: at the edge |

## The ruling

No move. `air_fatigue_per_100_nm` stays 2. The first stop rule tripped: every value that lifts days 22 to 24 toward the forms takes the early raids, already a rested group of 41 against 16 to 23, further above them.

**Fatigue is not the cause of the raid-size gap.** The forms show the units sending about 18 a raid whether they had rested two days or none. In the rules a group sends every aircraft it can fly.

The gap is named in two parts:
- The units' sortie counts, which are a scenario order that is not changed after tuning began (tuning rule 8).
- A missing rule: a strike of a given size. It goes on the open list.

**Said plainly:** total bomber sorties agree with the forms at 2 (212 against 211 to 213) only because the five consecutive raid days collapse, not because the raids are the right size.

**Fatigue's three figures close as placeholders:** `air_fatigue_per_sortie` (3), `air_fatigue_per_100_nm` (2) and `air_rest_per_pulse` (5). No row sizes them. The forms' raid sizes cannot, for the reason above. Rows 3976 and 3977 are one officer's recollection and are named only.
