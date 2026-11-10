# 0001: A game day runs 18:00 to 18:00 in the scenario's reference zone

Status: confirmed by the designer, Nov 10, 2026.

## The gap in the spec

The spec says a day resolves as a 12-hour night pulse, then a 6-hour morning pulse, then a 6-hour afternoon pulse. It does not say what clock time a day starts at, which clock that is, or how game days map to calendar dates.

## Decisions

1. **One simulation clock, anchored to UTC.** Inside the simulation, time is "elapsed hours since the scenario began". Day 1 starts at elapsed hour 0. Each scenario states the UTC instant of elapsed hour 0. Local times and dates are for display only.
2. **A game day starts at 18:00 in the scenario's reference zone.** For the South Pacific slice the reference zone is UTC+11, so a day starts at 07:00 UTC. Orders are given at dusk.

   | Pulse | Reference-zone time |
   | --- | --- |
   | Night | 18:00 to 06:00 |
   | Morning | 06:00 to 12:00 |
   | Afternoon | 12:00 to 18:00 |

3. **A game day takes the reference-zone date of its daylight pulses.** The night of 8 to 9 August plus daylight on the 9th is "9 August".
4. **A pulse is a time bucket. Darkness is a property of a hex during a pulse.** Rules that depend on light ask one function. They never read darkness off the pulse name. For the slice that function returns "is this the night pulse". On a wider map it becomes a calculation from longitude, latitude and date, and no rule changes.
5. **Pulses are simultaneous across the whole map.** There are no local-time pulses.

## Why

- Dusk orders let the player plan the night's surface action on the afternoon's sightings. That suits the Solomons, where the night belonged to the surface fleets. Tomorrow's air orders are 12 hours old at dawn, but the spec has local commanders make strike decisions inside the pulses, so the cost is small.
- WitP:AE also resolves night first, then day (manual section 3.0). This keeps the spec's pulse order.
- 18:00 UTC would be 05:00 in the Solomons, which is dawn. The clock has to be local to the theatre.
- The slice spans less than one hour of solar time and sits near 9°S, so one clock is accurate everywhere on it.

## What it means on the whole map

One clock is wrong far from its meridian. When it is 18:00 at Guadalcanal it is early afternoon in Calcutta and late evening, the previous date, at Pearl Harbor. Decision 4 exists so that this does not force a rewrite. Historical sources also disagree on the clock: the US Navy in the Solomons kept zone −11 time and the Japanese kept Tokyo time. Every historical time in scenario data is converted to the simulation clock.

## Deferred

- **The UTC anchor field in scenario files.** Added when a scenario first needs a real date (scenario 4, Savo Island).
- **The darkness function.** Added when night search arrives (scenario 3).
- **A scenario start offset,** for a scenario that opens at dawn. Added when a scenario needs it.
- **To step 5:** the real light calculation, whether a wide map needs four 6-hour pulses instead of 12, 6 and 6, and the reference zone for the full map.

## Alternatives rejected

- Start the day at 06:00 and resolve morning, afternoon, night. Orders are given at dawn. A night action is then ordered 12 hours before it happens, and it changes the spec's pulse order.
- Start at 00:00 with a split night. This needs four pulses and splits every night action across two days.
- Local-time pulses per zone. Two forces either side of a zone line would fight in different pulses.
