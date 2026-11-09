# 0001: A game day runs 18:00 to 18:00

Status: needs review.

## The gap in the spec

The spec says a day resolves as a 12-hour night pulse, then a 6-hour morning pulse, then a 6-hour afternoon pulse. It does not say what clock time a day starts at.

## Decision

Because the night pulse comes first, a game day starts at 18:00 and ends at 18:00:

| Pulse | Clock time |
| --- | --- |
| Night | 18:00 to 06:00 |
| Morning | 06:00 to 12:00 |
| Afternoon | 12:00 to 18:00 |

Orders are therefore given at dusk. Inside the simulation, time is "elapsed hours since the scenario began". Day 1 starts at elapsed hour 0.

## What it means for play

- The player plans the night's surface actions and tomorrow's air operations in one sitting, at dusk. That suits the Solomons, where the night belonged to the surface fleets.
- A scenario that starts with a dawn event, such as a landing, starts mid-day or needs a scenario start offset. No offset exists yet.
- Mapping game days to calendar dates is not defined yet. "Day 1" has no date.

## Alternatives

- Start the day at 06:00 and resolve morning, afternoon, night. Orders are given at dawn. This reads more naturally but changes the spec's pulse order.
- Start at 00:00 with a split night. This needs four pulses.
