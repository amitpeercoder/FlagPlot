# 0130: A carrier force comes up at speed and closes the range

Status: settled by the architect on Nov 16, 2026, on the designer's choice of the same day after stage 2's second proof did not pass (decision 0128, "As run"; `docs/validation/stage-2-second-proof.md`): the carrier doctrine is given what it left out, and the proof is run again with its pass marks unchanged. It adds two rules to decision 0125's doctrine, and no rule of the engine. Not built.

Summary: Two additions to the doctrine of a carrier force played from an end: it steams to its hex at full speed where cruising speed would not bring it there in the night, and on a morning it holds a fresh contact its bombers want and cannot reach, it steams to the nearest hex from which they can, by its risk level, and goes back to its hex when there is nothing to close on

Standing: Settled by the architect, Nov 16, 2026, on the designer's choice after the second proof. Not built

## Why

The second proof played the Eastern Solomons from ends a thousand times. The Allied carriers never struck, and the Japanese carriers struck on 152 seeds, against the 800 the plan asked of each. The report gives the cause from the staff's own reasons. A carrier force is sent to its hex at cruising speed, so the Japanese carriers came up late or not at all. And a carrier force does not move toward an enemy it holds beyond its bombers' reach: the American carriers sat with their bombers on the strike mission and nothing in reach. Decision 0125 left both out and said so.

A carrier that will not come up and will not close does not fight. Both are the judgement of the man who commands the force, which is where this record puts them.

What this record cannot mend: orders are taken once a day, so a force closes overnight on what it knew in the morning. On a contact first made in the morning it does not move until the next day. A battle of one day is decided by where the forces were sent, not by closing.

## The two rules, at the force's level

1. **Speed.** Wherever decision 0125's doctrine routes the force (to its hex, to a hex it closes to, or home), the speed is cruising speed where that brings it there within the 12 hours of the coming night, and otherwise full speed. At preserve the force it is always cruising speed. The speed is set by order only where it differs from what the force has. The fuel the force needs (decision 0125, 8) is measured at the speed chosen.
2. **A contact to close on.** On a morning the force is not held, not on its way home for fuel and has a bomber group that can fly: a contact of ships the side holds, no more than 3 pulses old, of a kind its risk closes on, that is beyond the normal reach of the force's bombers from where the force is, and within a night at full speed and that reach of it. Where there are several, the kind first in the order of importance, then the nearest, then the lowest number.

| | Preserve the force | Balanced | Press |
| --- | --- | --- | --- |
| It closes on | Nothing | A carrier | Any kind |

3. **Where it goes.** The first hex, along the shortest sea route from the force to the contact's hex, that is within the normal reach of the force's bombers of that hex. The reach is the least strike radius among the bomber groups aboard that can fly. It does not go nearer than its bombers need.
4. **Fuel comes first.** It does not close where the passage there and from there to its home port, times its risk's margin, is more than it has. The reason gives the figures.
5. **It stays while there is something in reach.** A force that has left its task's hex to close stays where it is on a morning it holds a contact no more than 3 pulses old, of a kind its risk closes on, within its bombers' normal reach. On a morning it holds none, in reach or to close on, it goes back to its task's hex, as decision 0125's rule 7 has it, and is given no route when it is there.
6. **Its search arc** is taken from the hex the force is in when the staff decides, as before.
7. **Readiness.** Battle readiness on a day it closes, as at its hex.
8. **Reasons** give the contact, its kind and age, its distance against the reach, the hex chosen and the hours to it, or why the force does not close: nothing fresh, nothing of a kind it closes on, already in reach, too far, or not the fuel.

## What it leaves out

- **Opening the range** after a strike, or keeping outside the enemy's reach while inside one's own. The staff holds no belief about the enemy's reach until stage 2b.
- **Closing inside a day.** It needs orders inside a day.
- **A surface force that closes.** The stand task has a station; a hunt is another task.
- **Where the hex of the task should be.** Still the commander's, or the scenario's.

## What it costs

- **No new state.** The doctrine's version rises.
- **The rulebook's chapter 8,** "Operating a carrier force", gains the two rules.

## The hand calculation

`tests/scenario_58_a_carrier_force_closes.rs`: one carrier at her hex and a carrier reported 480 nm away, over 3 days at press: she closes at full speed in the night, strikes in the morning, and goes back.

## The proof

The second proof is run again when this and decision 0131 are built, by decision 0128's test plan with its pass marks and its stated intents unchanged. Both runs are kept.
