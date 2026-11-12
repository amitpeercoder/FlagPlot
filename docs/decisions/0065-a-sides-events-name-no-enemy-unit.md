# 0065: A side's events name no enemy ship, task force or air group

Status: confirmed. The designer answered the question of decision 0061 on Nov 12, 2026: a side's events stop naming an enemy task force by its number. The same day the designer confirmed carrying the rule to enemy ships and air groups (decision 2) and leaving a fight without labels (decision 6), which the architect had proposed. It replaces item 8 of decision 0047 and amends its table of kinds.

## The gap

A side knows the enemy only as contacts and claims. Neither holds an identity: decision 0012 says a report holds no identity of the force seen, "so a side cannot tell two sightings are of one force".

But decision 0047, item 8, had every event a side may see name the enemy unit in it by its number in the log. Decision 0047 listed this as a cost accepted for step 1 and deferred the remedy.

- A search that sighted named the force: "air_group:7 searched and sighted task_force:4". Two sightings that name `task_force:4` are of one force, on any day of the game.
- An air attack named the ship and its force: "air_group:2 bombs at ship:33". A strike on Tuesday and a strike on Thursday that name `ship:33` hit the same ship, whatever the reports said.
- A raid named the raiding group: "the raid of air_group:12". The group that raided today is the group that raided yesterday, and a side learned how many groups the enemy flies.
- A fight at sea named both forces and every ship that fired or was fired at.
- The numbers are the stable numbers of the data files. A player who has read a scenario file knows which ship `ship:33` is.

## Decisions

1. **No event a side may see names an enemy task force.** The side's form of the event has no field for it.
2. **Nor an enemy ship or an enemy air group.** The reasons are the same, and a rule that hid the force while an air attack still named its ship would hide nothing.
3. **Where the log names two forces of a fight, the side's event holds its own.** `EngagementBegun` and `EngagementEnded` name `a` and `b`. A side's form has one field, `task_force`, for whichever is its own. This is the one place a cut event has a field the log's kind does not; its value is the log's.
4. **An event that loses only a name keeps everything else.** Six kinds were whole and are now cut of the enemy's name and nothing more: engagement begun and ended, a sighting roll that sighted, force surprised, force slipped away, an alarm that was heard, and a raid on the side's own base. Each keeps every other field and its whole breakdown with its result. A sighting roll and an alarm also lose the flag that says they succeeded, which is always true for an event the side is given.
5. **Two things are still named.**
   - An enemy base. A base is on the map for both sides, and a side orders a strike on it by its number.
   - The target a side wrote in its own order, in the event that accepts or rejects that order. It is the side's own words. A rejected order still says only "not your unit" for an enemy unit and for a number that does not exist (decision 0050).
6. **Nothing stands in for the name.** There is no label such as "enemy ship A" for the length of a fight. A side sees that its ship fired and with how many shots, and that its ship was fired on and with what.
7. **The log keeps every name** for the umpire's tools, as before.

## What each cut event now holds

| Event | For whom | Holds | Lost here |
| --- | --- | --- | --- |
| Salvo | The firer | Round, firer, weapon, shots | The target |
| Salvo | The target | Round, target, weapon | The firer |
| Air attack | The attacker | Air group, weapon, shots | The ship and its force |
| Air attack | The ship attacked | Task force, ship, shots | The air group |
| Air fight | The firer | Firer, what fired and at what, shots | The group fired at |
| Anti-aircraft fire | The force or base | Force or base, shots | The raid's group |
| Base attacked | The attacker | Air group, base, shots | Nothing |
| Base attacked | The base's side | Base, shots, chance, expected, hits and where | The air group |
| Search attempt | The searcher | Air group | The force |
| Engagement begun | A side that knows of the fight | Its own force, hex, hours | The enemy's force |
| Engagement ended | The same | Round, its own force, hex, reason | The enemy's force |
| Sighting roll | The force that sighted | Round, force, chance, fatigue factor | The force sighted |
| Force surprised, slipped away | The force | Round, force | The other force |
| Alarm roll | The force alarmed | Force, chance | The two forces that fought |

## Why

- The enemy is meant to reach a side as contacts and claims and nothing else. A number that is the same on every day of the game is an identity, and identity is the thing fog is made of: whether the carrier sighted this morning is the one bombed yesterday is the question a commander has to answer from reports.
- It is the mechanism decision 0047 already has. A cut event has no field for what the side may not see. No state, no event and no draw is added.
- Decision 0047 deferred "naming enemy units in fights by contact". That needs a rule for which contact a ship in a fight belongs to, and a fight can begin with no contact held. Removing the name needs no rule. Naming by contact can still be added later, as a field that is the side's belief.

## What it costs

- **Within a fight a side cannot tell one enemy ship from another.** It no longer sees that two of its ships fired at the same target, or that one enemy ship did all the firing. If play shows the need, the remedy is a label that lasts one fight and is made for the side ("enemy ship 1"), which is a new rule and is not made here.
- **Attacks from the air on one ship are counted together.** The briefing listed an attack by each enemy group. It now gives one line for a ship: 27 aircraft in 2 attacks.
- **An air attack in the briefing names no target.** The strike's launch names the contact it was flown at, and the report that follows is listed under the attack.
- **The agent interface is version 2.** Fields left the briefing, and the version rule of the schemas says a field removed raises the version.

## Rule card

| | |
| --- | --- |
| Reads | The log's events and which side each unit belongs to |
| Writes | Nothing. No state, no event, no random draw, no change to the hash |
| Where | `side_events.rs`: `SideEventKind`, `seen`, `side_breakdown` |

## As built

- `SideEventKind` has seven new cut variants (`EngagementBegun`, `EngagementEnded`, `SightingRoll`, `ForceSurprised`, `ForceSlippedAway`, `AlarmRoll`, `BaseAttackedAgainst`) and the nine it had lose their enemy fields. `only_a_name_is_cut` gives the seven their whole breakdown.
- The briefing: a fight has `force`; `fired`, `air_fire`, `guns` and `raids_on_bases` have no enemy field; an attack on a ship has `by`, `"ship"` or `"aircraft"`; an air attack flown has no target. The text says "at a raid", "at the enemy's bombers", "from the air".
- `sim-cli export` writes each side's events with the fields they now have. The viewer shows a side's events as a list and reads no enemy field from them.
- The MCP server's instructions say that no event names an enemy ship, task force or air group.
- Tests: `no_event_a_side_is_given_names_an_enemy_ship_force_or_air_group` in `side_events.rs` runs every row of the table of kinds both ways round. `tests/scenario_09a_side_events.rs` asserts on whole scenarios, for every event a side is given, that no enemy unit is in any field or in the text.
- The three recorded games were regenerated. Every answer carries version 2, and the briefings and `explain` answers that named an enemy unit no longer do. No save and no hash changed.
