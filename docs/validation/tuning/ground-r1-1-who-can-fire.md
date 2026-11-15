# Ground round 1, step 1: who can fire. Two figures moved

**These figures have no check battle.** They are fitted on the two fights of the full slice, the Tenaru and the ridge, and nothing checks them (decision 0109, "The round"). Every figure moved here is recorded as fitted on two fights and not checked, and stays a placeholder.

The full slice, `data/scenarios/07_full_slice.toml`, 39 days, Nov 15, 2026. This is the first step of the round, in the order of decision 0109's table. The report before it is `docs/validation/full-slice-39-days-before-tuning.md`. Each trial is a batch of 1,000 runs, seeds 1 to 1,000, on a scratch copy of the live figures, and `sim-cli validate` against `data/validation/guadalcanal_landings.toml`; the batches are labelled `r1-s1-<value>`.

## What was wrong

The Ichiki echelon reached the field with 916 men and fired with 412. Its fatigue at the fire was 55: 40 for landing with no rehearsal, 20 for two days of march, less one day's rest of 5. The record has the echelon land unopposed at night at Taivu (land_unit_facts 1049), march west at once (994), and attack in strength over the Ilu on the night of 20 to 21 August (1020, 1024). The Kawaguchi parts fired with 2,147 of 3,090 men (fatigue 20 to 45).

## The figures

| File | Figure | Before | The direction a source supports |
| --- | --- | --- | --- |
| `data/reference/ground.toml` | `unprepared_landing_fatigue` | 40 | down: the landing was unopposed and at night (1049) |
| `data/reference/ground.toml` | `fatigue_per_march_day` | 10 | down: two days of march, then an attack in strength (994, 1020) |

**Why these and not others.** Fatigue cuts the men who fire and nothing else (rulebook chapter 7, "Firing men"). A unit that marches does not rest, so the landing fatigue is still on the echelon on its day of attack. The two figures are read by both sides alike: the American landing of 7 August takes the same landing fatigue, and any march takes the same march fatigue. No figure that only one side reads is moved.

## What was run

The firing men are worked by the rule from the figures (fatigue at the fire = landing less one rest of 5 plus two days of march, not below 0 after the rest); the measures are from the batches. Medians, with the 5th, 25th, 75th and 95th.

| Landing, march | Ichiki fatigue at the fire, firing men | Tenaru, American men lost (109 to 110), 5/25/50/75/95 | Tenaru, Japanese (800 to 900) | Ridge, American (134 to 253), reported | Ridge, Japanese (600 to 1,138), reported |
| --- | --- | --- | --- | --- | --- |
| 40, 10 (before) | 55, 412 | 5, 14, **23**, 41, 59: miss | 240, 300, **340**, 360, 420: miss | 49, 82, **114**, 135, 182 | 1,060, 1,140, **1,220**, 1,280, 1,380 |
| 30, 10 | 45, 503 | 5, 19, **29**, 46, 63: miss | the same | 62, 97, **117**, 148, 187 | the same |
| 20, 10 | 35, 595 | 5, 23, **37**, 50, 72: miss | the same | 66, 101, **130**, 153, 200 | the same |
| 10, 10 | 25, 687 | 9, 23, **41**, 58, 81: miss | the same | 66, 101, **132**, 153, 200 | the same |
| 0, 10 | 20, 732 | 9, 28, **41**, 58, 85: miss | the same | 66, 101, **132**, 153, 200 | the same |
| 40, 5 | 45, 503 | 5, 19, **29**, 46, 63: miss | the same | 66, 101, **130**, 153, 200 | the same |
| 20, 5 | 25, 687 | 9, 23, **41**, 58, 81: miss | the same | 82, 117, **148**, 182, 218 | the same |
| **10, 5 (kept)** | **15, 778** | 14, 28, **46**, 63, 86: miss | the same | 82, 117, **148**, 182, 218 | the same |
| 0, 5 | 10, 824 | 14, 32, **49**, 63, 90: miss | the same | 82, 117, **148**, 182, 218 | the same |

The Japanese losses do not move: the Marines' fire is set by the limit on engaged men, three times the attackers' men ashore, not by their firing men. The base-holding measures agree in every trial, and the airfield day (11) and the food at Lunga (68 days) do not move.

## The values kept, and why

`unprepared_landing_fatigue` **10**, `fatigue_per_march_day` **5**. The Ichiki echelon fires with 778 of 916 men (fatigue 15). Every Kawaguchi part and the Oka group fires at fatigue 10 (read from the logs: the landing's 10 is rested off before the march), 2,781 of 3,090 men at the ridge.

- **The measure moves the whole way down and never reaches the record.** The Tenaru's American losses rise from 23 to 46. No pair of values takes them inside the middle 90 percent, and no pair takes another measure out. So the step's own stop rules do not decide the value: the source does.
- **What the source sizes.** The rows say "in strength" and size nothing finer. Fatigue 15 puts 85 percent of the echelon into the fire.
- **Not 0 for the landing.** At 0 a landing with no rehearsal would cost nothing, against the figure's meaning: a hurried landing is a cost that wears off. 0 and 10 give the same fight within a few men.
- **The march at 5 and not lower.** At 0 a march would tire no one. The ridge also reads this figure, and the Kawaguchi men marched a week inland in the record against two days in the game (`march_days_per_hex`, outside the round).

## What it rests on

The direction: rows 1049, 994 and 1020 (and 1024 for the hour of the attack). The size: nothing. Both stay placeholders, fitted on two fights and not checked.

## What it left

The Tenaru's American losses still miss (46 against 109 to 110). Step 3 owns what a hit does. The ridge's American losses move into the middle half of their record (148 against 134 to 253), and the two fights agree on this step's direction.

The American landing of 7 August reads the landing figure too. The Americans fire on day 1 with fatigue 10, not 40. That does not change who holds Lunga: the limit cuts them to three times the garrison either way.
