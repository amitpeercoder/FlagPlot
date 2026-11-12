# 0059: Aircraft that cannot land where they took off

Status: confirmed. The designer approved the plan on Nov 12, 2026. The figures are placeholders set by the architect, to be set from the record in step 2. It is built in three stages; this record is written before the first and gains an "As built" section with each.

## The gap

Four things the rules get wrong are one thing.

- **A strike's radius is a wall.** A group flies at full strength at 190 miles and not at all at 200. In a recorded game two torpedo groups stayed on deck 10 miles short of a carrier. In the record strikes were launched at 150 to 210 miles with every type together, and Saratoga's torpedo bombers flew against Ryujo, sighted at 198 miles (`docs/research/oob/strike-ranges.md`).
- **Nothing ditches.** At the Eastern Solomons 7 of the 17 American carrier aircraft lost came down in the sea out of fuel. The model can reach the recorded totals only by shooting too many down.
- **Aircraft aboard and aircraft in the air are lost alike when a carrier sinks.** Ryujo's aircraft in the air came down beside her destroyers and four went down with her; the model loses 29 with the ship, and every pilot.
- **A carrier whose flight deck is out takes her aircraft back all the same.** Enterprise's dive bombers went ashore to Henderson Field. The scenarios script that as an arrival.

The record of what became of such aircraft and their crews is in `docs/research/oob/ditching-and-rescue.md`.

## Decisions

### Reach (stage 1)

1. **An air group has a reach, set by order: normal or extended.** Normal is the default and is the rule as it was: a target is in reach if its air distance is within the type's strike radius. It is the "range limit" order of the spec, deferred by decision 0050.
2. **Extended reach is the radius and a share more:** the radius times (1 + `air_combat.toml: extended_reach_share`), cut to a whole mile. A naval strike group on extended reach strikes a contact within that, a base strike group a base within that, and a fighter group on extended reach sends its escorts with a raid within that of its own radius (decision 0030).
3. **An aircraft that flew beyond its radius may run out of fuel on the way home.** After air combat, before wear, each strike or escort sortie whose one-way distance d was beyond the type's radius R rolls each aircraft that came back, on its own, at `ditch_chance_per_radius_beyond` times (d - R) over R, 1 at most. An aircraft that fails is ditched: it is gone from the group for good. A sortie within the radius rolls nothing, so nothing changes for a group on normal reach.
4. **Each type is measured against its own radius.** Fighters, with the shorter radius, are further beyond theirs on the same strike and run the greater risk. No rule singles them out.
5. **A ditched aircraft is not then rolled for wear.** It counts as flown for fatigue and supply, and as destroyed for the enemy's objectives (decision 0054), though no one claims it.

### The crew (stage 1)

6. **Whether the crew is saved depends on where the aircraft comes down.** Where pilots are counted, one draw for each ditched aircraft:

   | Where | Chance the crew is recovered |
   | --- | --- |
   | Out of fuel on the way home | `crew_recovered_short_of_home` |
   | No deck, beside ships of its own side (stage 2) | `crew_recovered_beside_own_force` |
   | No deck, with no ship of its own side in the hex (stage 2) | `crew_recovered_open_sea` |

   A crew not recovered is a pilot lost. An aircraft shot down over the enemy loses its pilot as now, and one shot down on patrol over its own hex keeps it (decision 0038).

### No deck (stage 2)

7. **Aircraft in the air when their own deck cannot take them look for another place, in this order:** another carrier of their side in the same hex, afloat, with her flight deck not out and room under her aircraft capacity; a base of their side within the type's strike radius of the hex; the sea. A deck cannot take them when the ship has sunk or her flight deck is out when they return.
8. **Aircraft that land elsewhere join the group of their type and side at that place,** by the mechanism that delivers arrivals (decision 0041), with their pilots. Where there is no such group, one is formed there with the next free number. The group they came from keeps what it has aboard.
9. **Aircraft that go into the sea for want of a deck are ditched** as in decision 3, with the crew's chance by decision 6.
10. **Aircraft aboard a ship that sinks are lost with her, with their pilots, as now.** Most of those men were really saved. A saved pilot with no aircraft has nowhere to go in the rules, since replacements arrive only at bases; it waits for a roster of pilots and is recorded here as a known error.

### The scripted arrival goes (stage 3)

11. **Flight 300 is produced by the rule.** The arrival that scripts Enterprise's dive bombers to Henderson Field is taken out of scenario 6 and the full slice where the rule now delivers them, and the test is that they get there.

## Placeholder figures

`extended_reach_share = "0.25"`, `ditch_chance_per_radius_beyond = "2"`, `crew_recovered_short_of_home = "0.7"`, `crew_recovered_beside_own_force = "0.95"`, `crew_recovered_open_sea = "0.5"`. The record behind the last three is 19 crews of 19 recovered beside their own force and 6 to 9 of 13 to 16 in open sea; the first two have examples and no rates.

## Why

- **One rule for four gaps.** An aircraft goes out and must come down somewhere. Reach, ditching, a lost deck and a closed deck are the same question asked at different distances.
- **The price is paid on the way home,** as it was, and it is paid in the step that already rolls an aircraft after its sortie.
- **A choice, not a cliff.** The commander decides whether a target is worth aircraft that may not get back.
- **The order is off unless given,** so no scenario changes until someone reaches.

## What it leaves out

- A landing planned at a field beyond the target, to strike further. Decision 7 gives the unplanned landing only.
- Daylight: a strike that comes home in the dark. It needs a clock inside the pulse.
- The carrier closing the range to shorten the flight home (step 3, with the staff).
- Aircraft pushed over the side to clear a full deck: those with no room go on to a field or the sea.
- Pilots aboard a ship that sinks (decision 10).

## What it costs

- A group's aircraft can come to be based somewhere else, and a group can be formed in the middle of a game. Both are new. A new group's number depends on how many groups exist, as a split-off task force's does (the leak decision 0047 records).
- One more field of state for an air group, so the hash version rises and every golden file's hashes are remade.
- When stage 2 lands, the Eastern Solomons and the full slice change, and their hand calculations with them.

## Rule card

| | |
| --- | --- |
| Reads | An air group's reach, its type's radius and the distance of its sortie; whether its ship is afloat and her flight deck's state; the carriers and bases of its side at and near its hex, and their room; the five figures |
| Writes | The group's ready aircraft and pilots; the aircraft and pilots of the group they join, or a new group; a ditching event |
