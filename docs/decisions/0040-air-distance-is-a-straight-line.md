# 0040: Air distance is a straight line

Status: confirmed by the designer on Nov 11, 2026: straight-line distance for aircraft now, ships later. His reason is recorded under "Why". *(Nov 14, 2026: ships were settled by decision 0058: a ship is charged straight-line miles where the way is clear.)*

## The gap

Every rule that asks how far an aircraft flies counts hexes and multiplies by 40 miles (decision 0017, 15). That is exact along the three axes of the hex grid: east and west, and the two diagonals 60 degrees either side. Between the axes the count is the length of a staircase from one hex to the other, and at the worst bearing, 30 degrees off an axis, it is 15.5 percent longer than the line. Due north and south is such a bearing.

Rabaul to Henderson Field runs south-east, almost exactly between two axes:

| Measure | Distance |
| --- | --- |
| The real distance (Miller, *CARTWHEEL*; Shaw and Kane) | 565 miles |
| A straight line between the two hex centres, (-5,-13) and (2,-4) | 556 miles |
| The hex count, 16 hexes | 640 miles |

No sourced range for the A6M2 or the G4M1 reaches 640 miles. With real figures the raids scenario 6 is about could not be flown. The fault is in the measure, not in the aircraft.

It was hidden until now. The placeholder scenarios lie along an axis, Savo Island is fought in one hex, and the ranges at the Eastern Solomons had slack.

## Decisions

1. **The air distance between two hexes is the straight line between their centres,** in whole nautical miles: 40 times the square root of (dq x dq + dr x dr + dq x dr), rounded to the nearest mile, where dq and dr are the differences of the two hexes' coordinates. Along an axis it is the hex count times 40, as before.
2. **Every rule that asks how far an aircraft flies uses it:** whether a contact or a base is within a strike's radius, whether a raid is within an escort's radius, whether a target is within a search arc's range and the distance in the chance of sighting it, and the distance of a sortie for wear, fatigue and supply.
3. **Nothing else changes.** A ship still moves hex by hex and each step is 40 miles (see "Deferred"). The distances that are thresholds in hexes stay in hexes: how near a contact must be to alert a force, and how near two reports must be to be one contact.
4. **There is one function for it,** and anything that shows or measures an air distance uses that function: the events, a briefing, the replay viewer, and any measuring tool the game is given later. A tool that measured another way would tell the player an aircraft can reach what the rules say it cannot.
5. **It replaces decision 0017, 15,** which kept straight-line hex counts for search and other rules.

## Why

- **An aircraft flies the line.** Decision 0017 said as much, "an aircraft does not go round an island", and then measured the line as a staircase.
- **The designer's reason:** a game that sets out to simulate something should not be limited by its own design. Hexes are how the game says where things are and who can engage whom. They should not bend how far an aircraft can fly.
- **The other ways out put a figure on a real aircraft that no source states.** Giving the Zero 640 "map miles" lets it reach a target that really is 640 miles away along an axis. Scaling every radius by a tenth is wrong in both directions. The data rules forbid both.
- **Whole miles.** A distance of 222.710575 miles in an event helps nobody. A range is known to the nearest mile at best.

## What it costs

- Air distances are no longer multiples of 40, and a reader can no longer count hexes to check one. The event gives the distance.
- `sim-core` gains an integer square root.
- The Eastern Solomons hand calculation changes where the geometry is off an axis: the Japanese search finds the American carriers a little more often (0.762752 of runs with a contact, from 0.749133), their strike flies 223 miles and not 240, and more of the hexes a report can name are within reach of the TBF-1s and the D3A1s. Who can reach the true position of the enemy does not change for any group.
- The placeholder scenarios do not change: everything in them lies along an axis.

## Deferred

- **Ships.** A force that steams between two axes zigzags and makes up to 15.5 percent less ground than its speed says. Noumea to Espiritu Santo is 405 miles and 520 by hex (decision 0017). That needs its own decision, because it reopens how a force moves and where it is (decisions 0005 and 0017). It is to be settled before step 2 compares ship movements with the record. *(Nov 14, 2026: settled by decision 0058.)*
- The projection is true east and west at 12 degrees south and up to 5 percent off at the edges of the slice, and a place is taken to be at the centre of its hex, up to 23 miles from where it is. Both are small beside the staircase and are left.

## What the runs show

Eastern Solomons over seeds 1 to 1000, before and after. Nothing else changed: no other scenario's golden file moved.

| | Hex count | Straight line |
| --- | --- | --- |
| Runs in which the Japanese attack | 518 | 520 |
| Runs in which the Allies attack | 383 | 383 |
| Runs in which Ryujo is sunk | 265 | 265 |
| Runs in which Enterprise ends crippled | 9 | 9 |
| Bomb hits on Enterprise a run | 2.95 | 2.97 |
| Aircraft lost a run, Allied and Japanese | 1.88 and 22.55 | 1.89 and 22.61 |

The map tool's `path` now prints the air distance beside the sea path: Rabaul's hex to Henderson Field's is 556 miles by air.

## Rule card

| | |
| --- | --- |
| Reads | The coordinates of two hexes |
| Writes | Nothing |
| Events | None new. Events that give an air distance give this one |
| Randomness | None |
