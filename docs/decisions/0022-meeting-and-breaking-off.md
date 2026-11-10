# 0022: Meeting in time order, and breaking off

Status: confirmed by the designer, Nov 10, 2026, as stage 4c2b of scenario 4. It replaces decisions 1 and 14 of decision 0014.

## The gap

Decision 0014 checked for fights once in a pulse, after all movement, between forces that ended the pulse in the same hex. A force steaming through a hex never met what was in it, and a force could not arrive, fight and leave within one night. Breaking off ended a fight and changed nothing else: a force that had just broken off could attack the next enemy in the same hex.

The spec lists movement and surface engagements as two steps of a pulse. This decision merges them.

## Decisions

### Meeting

1. **Within a pulse, forces move in order of clock time.** Each change of a force's position is taken in the order it happens, the lower task force ID first when two happen together.
2. **Two opposing forces meet when one's position becomes a hex where the other is.** They fight there and then, by decision 0014. A force's position is the nearest hex centre (decision 0005), so a force enters a hex 20 nm before its centre.
3. **Forces that begin a pulse in the same hex meet at its start,** if neither will move in that pulse. A force that is under way and able to move is leaving, and does not fight at the start of the pulse what it shares a hex with.
4. **A pair fights once in a pulse at most.**
5. **After a fight each force carries on along its route** for what is left of the pulse, at the speed its ships can then make. The distance it has left is the distance it had left before the fight, scaled by its new speed over its old. A force that could not move before the fight and can after it, because the ship that held it sank or was left behind, has all of what is left of the pulse: after a fight at the start of a pulse, the whole pulse. If its speed changed, that distance is also limited by how far the fuel now aboard takes it at the new speed, as at the start of a pulse (decision 0006). Stragglers are dealt with straight after the fight, by the force's instruction (decision 0016), so a force sent home starts for home at once.
6. **A fight takes no time.** Up to six rounds of five minutes cost a force no steaming. This is a simplification.

   A second one: a force's steaming is accounted at its own changes of position. When a force under way is met part-way between two of them, its steaming since the last one, under 40 nm, is treated as still to come, and so is scaled by its speed after the fight. Where the force met is stopped, or changes position at the same moment, as in every scenario so far, nothing is lost. One visible result: a force that is met in the hex of its last waypoint after reaching the centre has its last 20 nm and its arrival recorded after the fight, with the arrival's own, earlier, hour, and if the fight leaves it unable to move it is not recorded as arrived.
7. **Several meetings at the same moment** are taken in order of the lower task force ID, then the higher.
8. **The surface engagement step of a pulse is now empty.** It is kept as a slot.

### Breaking off

9. **A force that breaks off, or slips away, avoids action for the rest of the pulse.** It takes the avoid stance of decision 0014 for any other force it meets: it slips away if it sights an enemy that has not sighted it, and otherwise fights back and breaks off at the end of the first round in which it is fired on. At the start of the next pulse it takes up its ordered stance again. This is not stored: within a pulse it matters only inside the movement step, and the events that record a force breaking off or slipping away are the record of it.
10. **It keeps to its route.** A force with a route out of the hex leaves. A force with no route stays.

## Why

- **Time order is one mechanism in place of two steps,** and it is what makes a night's work possible: steam in, fight, and be gone by dawn. At Savo the Japanese column was in action for under an hour and well up the Slot by first light.
- **Meeting on entering a hex** means a force cannot slip past an enemy by going fast.
- **Leaving forces do not fight again at the next pulse,** because otherwise every night action would be followed by a daylight one against an enemy who is steaming away.
- **Avoiding after breaking off** is what the words mean. Decision 0014 left it open.

## What changed in earlier scenarios

- In scenarios 4a and 4c2a the attacker used to fight where it ended the night pulse. It now fights on entering the hex, 20 nm short of the centre. The same rolls are made, so who sights whom and every count over the 1,000 seeds are unchanged. What changed is the attacker's steaming after the fight in that same pulse, which now follows the damage it took.
- Scenario 4c2a had a force ordered to 40 kn that passed through an enemy's hex untouched. It now fights there.
- Scenarios without opposing forces in one hex are unchanged.

## Deferred

- A commander's choice to press on or retire after a fight. A route in and out does this for a scripted scenario. A rule belongs with the staff and rules of engagement, in scenario 5.
- A fight taking time, and a force pursuing one that breaks off.
- Forces in neighbouring hexes fighting at long range.
- Closing a met force's steaming at the exact moment of the meeting, which would remove the second approximation of decision 6. It makes distances that are not whole numbers; it waits until a scenario needs it.
- Clock times in the log are cut to the minute, not rounded, so 10:40 can print as 10:39.

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| Movement in time order (replaces movement, decision 0005, and meeting, decision 0014) | Everything movement read; every task force's position, side and ships | Everything movement wrote |
| Meeting | Task force position, side, ships, whether it is moving; pairs that have fought this pulse | Starts a fight (decision 0014) |
| Breaking off | The fight's outcome | Nothing stored. The force's stance in later fights of the same movement step |

Movement and combat now run inside one step. Combat changes ship damage, which movement reads within the same pulse.
