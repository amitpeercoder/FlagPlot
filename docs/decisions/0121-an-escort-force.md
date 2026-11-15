# 0121: An escort force: keeping station, and who is met first

Status: settled by the architect on Nov 15, 2026, inside the plan for step 3, stage 2 (decision 0113, decision 14), on the designer's answer to its question 4: escorts sail as a force of their own, with a rule. It writes the rule out with its rule card and its hand calculation. Not built. It is built in step 5 of the stage, with the doctrine for escorted and covered runs, which gets its own record.

Summary: A task force on the escort mission may be given a charge, another force of its side: it keeps station on it without a route of its own, an enemy force that meets the charge must first fight every escort in station and reaches the charge only past an escort that is sunk or has broken off, and the escorts' guns fire with the charge's at aircraft that attack it

Standing: Settled by the architect, Nov 15, 2026, inside decision 0113. Not built

## Why

The escort mission of a task force exists and no rule reads it. Warships that sail in the same force as the cargo ships they guard fight as one force with them, so the cargo ships are in every fight; warships that sail as a second force in the same hex are met in task force number order, so an enemy may fight the cargo ships first.

The record has an escort as a body apart that goes to meet the enemy while its charge turns away (`docs/research/oob/escorts.md`): Tanaka sent his three escorting destroyers into the fight on 14 November and turned the transports north; on 12 November the American convoy withdrew and the warships stayed. The one plan found with stations in it, Task Force 67's of 27 November 1942, puts cruisers about 2,000 yards from a convoy and destroyers 2,000 to 3,000 yards beyond them, closing to about 1,800 yards under air attack. Those are distances inside one hex of 40 miles, so station is kept by being in the charge's hex.

No rule of the time for the number of escorts was found, for either side. This record does not give one: how many escorts a run is given is the staff's judgement of the threat, in step 5's doctrine.

## The rule

1. **A charge.** A force order, `escort`, on a task force on the escort mission names another task force of its side as its charge, or clears it. A force has at most one charge. A charge may have several escorts. An escort cannot be a charge, and a force cannot escort itself.
2. **In station.** An escort is in station when it is in its charge's hex and both have a ship afloat.
3. **Keeping station.** An escort with a charge has no route of its own, and a route order to it is refused while it has one. At the start of each pulse's movement:
   - in station, it takes its charge's route and speed for the pulse, and moves as a force ordered so would, at no more than its own ships can make;
   - out of station, it steams for its charge's hex by the shortest sea route at its cruising speed.
   It burns its own fuel by chapter 2's rule. Where its charge stops to work cargo, it stops with it.
4. **Who is met first.** Where an enemy force meets a charge by chapter 4's rule, and the charge has escorts in station, the enemy's meetings with those escorts are held first, in task force number order, and the meeting with the charge is held after them only if every one of those escorts was, when its fight ended, without a ship afloat or the one that broke off. An escort that the enemy did not sight and that did not sight the enemy has not been passed: the charge is not met.
5. **A meeting screened is not held later in the pulse.** A charge not met for rule 4 is not met by that enemy force again in the pulse while both stay in the hex.
6. **Against aircraft.** When a raid attacks a charge, the anti-aircraft shots of its escorts in station are added to the charge's, before and after the attack, by the share of chapter 5. The raid's target is chosen as it is today, and bombs fall only on the force chosen.
7. **When the charge is gone.** An escort whose charge has no ship afloat, or has been stood down or joined into another force, has no charge from that moment and stops where it is.
8. **What a side sees.** The order and its event are the side's own. A meeting not held is recorded as an event of the umpire's and of the charge's side, caused by the escort order and the escort's fight; the enemy is told nothing of a force it did not meet.

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| Keeping station | The escort's charge, both positions, the charge's route and speed, the escort's ships' speeds | The escort's route and speed for the pulse |
| Who is met first | The meetings of chapter 4, the charges and escorts in the hex, how each escort's fight ended | Which meetings are held, and their order |
| Against aircraft | The raid's target, its escorts in station and their guns | The shots of the guns that fire at the raid |

## What it leaves out

- **What the charge does when its escort fights.** It keeps to its route and its work. Turning away or ceasing to unload is the staff's order of the next morning, and a standing instruction for it waits for a need.
- **Stations inside the hex.** Day and night stations, pickets and the distance of the screen are below the grain of the map.
- **An escort that goes out to meet an enemy in a neighbouring hex.** Forces in neighbouring hexes do not fight (chapter 4's known limit).
- **Submarines,** which the game does not have.
- **A covering force.** It is a surface force the staff places and keeps at readiness, by rules that exist. Step 5's doctrine says where and when.
- **Escort of a force by aircraft.** A patrol over a force at sea is a carrier's or a base's, by chapter 5.

## What it costs

- **State:** a task force's charge, in the hash and named beside it; the state version rises. The rules label rises.
- **An order** and its event, added at the end; a new event for a meeting not held, with its entry in `side_events.rs`.
- **The rulebook:** chapter 2 gains "Keeping station"; chapter 4, "Meeting", gains the order of meetings with an escort; chapter 5 gains the escorts' guns; chapter 1's table of missions says what the escort mission now does. The glossary gains charge, escort force and in station.

## The hand calculation

`tests/scenario_51_an_escort_force.rs`: a cargo force and its escort over a passage of 200 nm, with the hexes of both at the end of each pulse; and, for the meeting rule, cases built so that the escort holds or is passed, tested over seeds for which meetings are held and in what order.
