# 0014: Surface combat

Status: confirmed by the designer, Nov 10, 2026, as stage 4a of the scenario 4 plan. Every figure is a placeholder. Decision 0016 replaces the system damage track described here with named systems.

## The gap in the spec

The spec lists "surface engagements where opposing forces meet" in each pulse and asks scenario 4 to prove "surface combat, surprise, night detection". It gives no rule for when a fight starts, who sees whom, how fire is resolved, what damage does, or when a fight ends.

The spec's scenario 4 is Savo Island. It needs a combat rule, the real map and real ships. Those are several subsystems, so scenario 4 is built in stages, each its own pull request:

| Stage | Adds |
| --- | --- |
| 4a | The combat rule on placeholder ships (this record) |
| 4a2 | Damage to named systems, and what a force does with a cripple |
| 4b | The real map and pathfinding |
| 4c | Savo Island with real ships |

## Decisions

### Meeting

1. **Two task forces of opposing sides fight when they are in the same hex at the surface engagement step of a pulse.** That step comes after movement. Where more than two forces share a hex, each opposing pair is resolved in turn, in task force ID order. A force with no ships afloat does not fight.
2. **A fight runs in rounds of five minutes, six at most in one pulse.** Everything in a round happens at once: fire is worked out on the state at the start of the round, and damage is applied at its end. A ship sunk in a round still fires in that round.

### Who sees whom

3. **Each round, a force that has not yet sighted the enemy rolls to sight it.** The chance is a base figure for night or day, plus a bonus if any ship afloat in the force has radar, multiplied by a factor if the force is not alert. It is capped at 1. Whether it is night comes from the darkness function (decision 0001).
4. **A force that is fired on has sighted the enemy from the next round,** with no roll. Gun flashes give the firer away.
5. **A force is surprised in any round in which it is fired on before it has sighted the enemy.** It cannot fire in that round, and fire against it is more accurate.
6. *(Replaced by decision 0021: readiness is set by order, and a force at normal readiness is alert only while alerted.)* **A task force is alert or not alert.** It is a stored flag, alert unless the scenario says otherwise. A force becomes alert when it sights an enemy force or is fired on. What makes a force lose its alertness, or start without it, is deferred: see below.
7. **If neither force sights the other in six rounds, they pass unseen.** An event records it.

### Fire

8. **Every ship afloat in a force that has sighted the enemy fires each round.** Ships are matched in order: the first ship of the force fires at the first enemy ship afloat, the second at the second, and so on, starting again at the first when the enemy has fewer ships. Order is by ship ID.
9. **Guns and torpedoes use one mechanism: shots, a chance to hit, damage per hit.**
   - Gun shots in a round = guns x rounds per gun per minute x 5, reduced by the firing ship's system damage: x (100 - system damage) / 100. Shots are capped by the ammunition aboard and use it up.
   - Torpedo shots = tubes, once only, in the first round the ship fires.
   - Chance to hit = a base figure for the weapon, by night or day, x a factor if the target is surprised, x a factor if the target is lit by fire (any fire damage, at night only).
   - Expected hits = shots x chance. The whole part hits. The fraction is rolled once: hits = whole part, plus one with a chance equal to the fraction.
10. **Each hit damages one of three tracks, each running 0 to 100:**
    - Damage in points = 100 x the weapon's damage / the target class's toughness.
    - A gun hit lands on flotation, system or fire, by set chances.
    - A torpedo hit always lands on flotation.
    - A track stops at 100.
11. **What the tracks do:** a ship sinks when flotation damage reaches 100. System damage reduces its gun shots. Any fire damage lights the ship at night. Nothing else yet.
12. **A sunk ship leaves its task force.** It is in no task force and no port, and takes no further part in anything.

### Ending

13. **Each force has rules of engagement: engage or avoid, and a break-off level.** They are set by order and persist.
    - A force breaks off at the end of a round when the share of its ships that are sunk or crippled (flotation damage of 50 or more), out of the ships it began the fight with, reaches its break-off level.
    - A force set to avoid, which has sighted an enemy that has not sighted it, slips away: the fight ends at once with no fire. Otherwise an avoiding force fights when it can and breaks off at the end of the first round in which it was fired on.
14. **A fight ends** when either force breaks off or slips away, when one force has no ship afloat, when they pass unseen, or after six rounds. Both forces stay where they are. Breaking off does not move a force.

### What each side learns

15. **A force that sighted the enemy, or was fired on, sends a sighting report** when the fight ends. It reports the enemy ships still afloat. It goes through the same error model and into the same contacts as an air search report (decision 0012).
16. **A force that fired sends a battle claim:** enemy ships believed sunk. A side counts as sunk every enemy ship that sank, and every enemy ship left with fire damage of 30 or more. Claims go into belief. Real damage goes into truth. The two can differ.

### Random numbers

17. **Sighting rolls** come from a stream keyed on the force and the enemy force, one draw a round until it sights. **Fire** comes from a stream keyed on the firing ship and the enemy force, kept for the whole fight, drawn in this order each round: the gun fraction, one draw per gun hit for its track, then the torpedo fraction in the round torpedoes fire.

## Why

- **WitP:AE gives no mechanism to learn from.** Its surface combat section is a paragraph. It says forces in one hex fight, that radar raises a detection level per ship inside the fight, and that burning ships stand out at night (manual sections 6.4.2 and 10.3). How range, targets, guns and torpedoes work is not stated.
- **One mechanism for guns and torpedoes** keeps a new weapon a matter of data.
- **Surprise is not a separate rule.** It falls out of who sights first. A force that sights first fires on a force that cannot answer and is easier to hit.
- **Expected hits with one roll for the fraction** can be worked by hand for any salvo, and still varies. The spread in a fight comes mostly from who sights first and where hits land.
- **Three damage tracks, each with exactly one effect.** A track with no effect would be a rule nobody could explain.

## Legibility test

Run on Nov 10, 2026. A Haiku model was given this record, the combat figures and the event logs of two runs, and nothing else. It answered fifteen questions correctly: why a surprised force did not fire, where each hit chance came from, why a damaged ship fired fewer shots, why one force broke off and the other did not, why a side claimed a ship that had not sunk, and three predictions for changed inputs.

It found two things hard:

- The report error model is in decision 0012, which it was not given. It could say what was wrong in a report but not which rule produced each error.
- The log gives each ship's damage as it changes, but not a summary at the start of each round. Whether a target was lit has to be worked out by reading back. A per-round state line belongs with briefings.

## Deferred

- **How a force comes to be alert or not.** The intended rule ties it to belief: a force is alert when its side holds a fresh contact nearby. That is what would explain Savo, where sighting reports arrived late and were misread. It arrives with stage 4c.
- **Several forces in one hex fought in sequence,** and the choice to press on or retire after a fight. Stage 4c.
- **Range.** There is one night range and one day range, inside the base figures. Opening and closing the range, and long-range torpedoes, come later.
- **Target choice.** Ships are matched in order. Burning or nearer ships do not draw fire yet.
- **Stage 4a2, agreed with the designer on Nov 10, 2026.** The single system track is replaced by named systems, each working, degraded or out, and each with one effect: propulsion (half speed, then dead in the water), fire control (hit chance and night sighting), main battery (guns out) and torpedo mounts (tubes out). Flotation and fire stay as tracks. Where a hit lands depends on the weapon, from a data table. A ship with propulsion damage slows its task force through the existing slowest-ship rule. A standing instruction, set with the rules of engagement, says what a force does with a cripple: keep it, detach it with an escort to its home port, or leave it. That needs the detach and home port orders the spec lists. It gets its own decision record.
- **Towing and scuttling** a ship that is dead in the water, and a force moving away when it breaks off.
- **Fire and flooding that grow or are fought,** repair, and ammunition resupply.
- **Armour, shell types and torpedo reliability.**
- **Shore bombardment,** ships at anchor, and transports as targets.
- **Real figures.** Hit chances and damage come from sources in step 2.

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| Meeting (surface engagement step) | Task force position, side, ships | Nothing |
| Sighting | Task force alert flag; ship class radar; darkness; combat figures | Task force alert flag |
| Fire | Ship class guns, tubes, damage, toughness; ship ammunition, torpedoes ready, system and fire damage; whether the target is surprised; darkness; combat figures | Ship flotation, system and fire damage; ship ammunition and torpedoes ready |
| Sinking | Ship flotation damage | Ship location; task force ships |
| Breaking off | Rules of engagement; ship flotation damage | Nothing |
| Reports and claims | The fight's outcome; search figures | The side's contacts and claims |
| Rules of engagement order intake | Ownership | Task force rules of engagement |

Movement writes the position that meeting reads. Cargo handling and search both read task force ships, which sinking now changes. Air search reads darkness, as sighting does.
