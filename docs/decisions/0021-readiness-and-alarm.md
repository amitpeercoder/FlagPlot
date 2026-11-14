# 0021: Readiness, alertness and alarm

Status: confirmed by the designer, Nov 10, 2026, as stage 4c2a of scenario 4. Every figure is a placeholder. It replaces decision 6 of decision 0014.

## The gap

Decision 0014 made a task force alert or not by a stored flag that the scenario set, and deferred how a force comes to be alert. A force that is not alert sights the enemy half as readily, which is most of what surprise is. Savo Island turned on it: the Allied groups were not expecting an attack, and the second group was surprised minutes after the first, a few miles away.

## Decisions

### Readiness

1. **A task force is at normal or battle readiness,** set by order. The default is normal. A force at battle readiness is alert.
2. **A force at normal readiness is alert only while it is alerted.** Being alerted lasts for the pulse. It is worked out afresh at the start of each pulse.

   *Nov 14, 2026: the flag a fight or an alarm sets is not cleared when the pulse ends. It is cleared at the start of a later pulse by alert from belief, which records `ForceStoodDown`; "lasts for the pulse" was loose (chapter 3 of the rules; `docs/rules/differences.md`, row 1).*

### What alerts a force

3. **Belief, at the start of each pulse.** A force is alerted if its side holds a contact within a set number of hexes of its position that is no older than a set number of pulses. The straight-line hex count is used. An event records a force becoming alerted this way and cites the report behind the contact. Another records it standing down when no such contact remains. A force that was alerted in the pulse before and still has such a contact stays alerted, and no event is recorded, because nothing changed.

   *Nov 14, 2026: a force alerted by a fight stays alerted for as long as the contact the fight's report made stays fresh and within the alert distance of the force (`docs/rules/differences.md`, row 1).*

4. **Events, during a pulse.** A force at normal readiness that sights an enemy force, or is fired on, is alerted for the rest of the pulse, as in decision 0014. For a force at battle readiness nothing changes: it is alert by its readiness.
5. **Alarm.** When a fight in which at least one salvo was fired ends, every task force in that hex other than the two that fought, with a ship afloat, that is not alert, has a set chance to be alerted for the rest of the pulse. An event records every such roll, with its chance and whether it succeeded, and cites the end of the fight.
6. **Belief does not alert a force in the middle of a pulse.** A side's belief is shared the moment a report arrives. If alertness followed it at once, the first report of a fight would alert every force of that side in the hex. So within a pulse the news of a fight reaches other forces only by the alarm roll, which can fail. From the next pulse, decision 3 applies and they know.

### Speed

7. **A speed order may name a speed in knots.** The force steams at that speed, or at the best speed its slowest ship can make as it is, if that is lower. A ship's best speed is its top speed times its propulsion factor: a 32 kn cruiser with degraded propulsion makes 16 kn, whether it is ordered to 24 or to 30. Fuel burn follows decision 0006.

### Starting contacts

8. **A scenario may start a side with contacts already held.** Each is entered as a report and then aged by the number of pulses given, by the same rules as any other, so belief is still written only from reports. An event before day 1 records each.

### Random numbers

9. **The alarm roll** comes from a stream keyed on the force alone, one draw for each fight it hears in a pulse, in the order the fights end.

## Why

- **Readiness by order** is what a commander decides: Mikawa's column was at battle stations because it was going in to attack.
- **Alert from belief** ties search to combat. A sighting that reaches a force in time makes it harder to surprise. One that arrives late, or names a place far away, does not. At Savo the only reports of the Japanese column put it some 300 nm away, hours earlier.
- **A roll for the alarm** leaves room for what happened: gunfire and flares seen from the northern cruisers were taken for something else.

## Legibility test

Run on Nov 10, 2026. A Haiku model was given this record, the combat figures and the event logs of two runs of the three-forces scenario. It answered all twelve questions correctly: the distance and fuel at 24 knots, why the two forces sight at different chances, what the alarm roll did in each run, why a fresh contact in the same hex did not alert a force in the same pulse, why a force at battle readiness has no "became alert" line, and predictions for a contact at the limits of distance and age and for a speed order above a damaged ship's best.

It found three things unclear, each now changed:

- The final state printed "battle readiness, not alerted", which reads as a contradiction. The command-line output now says whether a force is alert, and why.
- It could not tell why a force already alerted got no line at the next pulse. Decision 3 now says a force that stays alerted records nothing.
- "Every other task force" in decision 5 did not say the two that fought are excluded. It does now.

## Known gap

**Battle readiness costs nothing yet.** A player would always choose it. In reality a crew cannot stay at its stations for days: the Allied ships at Savo were at a relaxed condition because their men were exhausted. That cost is crew fatigue, which arrives with air operations in scenario 6. Until then readiness is set by scripted scenarios, and nothing that plays the game should be given the order. *(Nov 14, 2026: built by decision 0037: an alert force tires each pulse)*

## Deferred

- Fatigue, as above. *(Nov 14, 2026: built by decision 0037)*
- Alarm reaching neighbouring hexes.
- Reports that arrive late or not at all. Scenario 5.
- A force reacting to being alerted by changing course or speed. Scenario 5.

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| Alert from belief (start of each pulse) | The side's contacts; task force position and readiness; combat figures | Task force alerted flag |
| Alarm (after a fight) | Task forces in the hex, their readiness and alerted flag; combat figures | Task force alerted flag |
| Sighting (decision 0014, amended) | Task force readiness and alerted flag, in place of the alert flag | Task force alerted flag |
| Speed in knots (in movement) | The speed order; ship class top speed; propulsion state | Nothing new |
| Readiness and speed order intake | Ownership | Task force readiness; speed setting |

Search writes the contacts that alertness reads. This is the first rule in which what a side believes changes what happens to it.
