# 0026: Air strikes on ships, and the flight deck

Status: confirmed by the designer, Nov 11, 2026, with the plan for scenario 5 (stage 5b). The plan settled that strikes are decided by a simulated local commander on belief, resolve in the pulse they are launched in, use the hit mechanism of decision 0014, and that a damaged flight deck limits flying. The details below that level are the architect's, and decisions 4, 7 and 9 are marked for the complexity review.

## The gap

The spec's daylight pulse has three air steps: search, strike decisions by local commanders, and air combat. Only search existed. This decision adds the second and the simplest form of the third: a strike that nobody opposes. Fighters and anti-aircraft fire are stage 5c.

## Decisions

### Who strikes

1. **An air group has a third mission: naval strike.** It is set by order, like search. A group flies one mission.
2. **An aircraft type says what it attacks with:** nothing, bombs or a torpedo, the damage one hit does, and how far it can strike. A type that attacks with nothing cannot be given the strike mission.

### The decision

3. **In the strike decision step of a daylight pulse, each group on naval strike decides for itself.** It comes after search, so a sighting made that morning can be struck that morning. A group decides only if it has aircraft, has a position, it is not dark there, and it can launch at least one aircraft (decision 10).
4. **It strikes a contact its side holds that is no more than one pulse old and within its strike range** of where the group is. Among several it takes the one whose reported kind is the most important (carriers first, in the order of decision 0012), then the nearest, then the lowest contact number. With none, it does nothing, and nothing is recorded.
5. **It sends every aircraft it can launch.** Holding some back is stage 5c.
6. **The decision is made on belief.** The group flies to the hex the contact names. Nothing in the decision reads the truth.

### The strike

7. **In the air combat step of the same pulse the strike reaches that hex and attacks an enemy task force that is really there.** If several are, it takes the one with the most important ship, then the lowest number. If none is there, it finds nothing and comes home: an event says so. It does not search the neighbouring hexes.
8. **Both sides' strikes are launched before either lands.** Every decision of the pulse is made on the state at the start of the strike decision step, and every attack is worked on the state at the start of the air combat step. Damage is applied afterwards, in order of air group number. A group aboard a ship still attacks in a pulse in which its ship is sunk.
9. **All its aircraft attack one ship: the most important ship afloat in the force,** the lowest number on a tie. Each aircraft is one shot. Expected hits are shots times the chance to hit for its weapon, and hits are the whole part plus one more with a chance equal to the fraction, as in decision 0014.
10. **A hit does damage exactly as a hit in a surface fight does** (decisions 0014 and 0016): 100 x the weapon's damage / the target's toughness points; a bomb lands on flotation or fire by the gun's split, and a torpedo on flotation; each hit may knock a system down one level. Bombs have their own table of which system. A ship that reaches 100 flotation sinks.
11. **Nobody is lost on a strike yet.** Every aircraft comes home. Losses arrive with fighters and anti-aircraft fire in stage 5c.
12. **A strike that attacks sends a sighting report of the force it attacked,** by the error model of decision 0012. A strike that finds nothing sends none.

### The flight deck

13. **A ship has a fifth system, the flight deck,** working, degraded or out like the others (decision 0016). It matters only on a ship whose class carries aircraft.
14. **The flight deck limits how many aircraft each group aboard can fly:** all of its own aircraft, half of them (the degraded factor, rounded down), or none. It is worked group by group, not for the ship as a whole, and applies to search and to strikes alike.
15. **A bomb's system hit can land on the flight deck.** On a ship that carries no aircraft that hit changes nothing.

### Random numbers

16. **A strike's rolls come from one stream keyed on the air group and the task force attacked,** in the air combat subsystem, drawn in this order: the hit fraction, one draw per hit for its track and, if it is a system hit, its system, then the report.

### How it is recorded

17. **In the air combat step every strike's finding is recorded first,** in order of air group number: `StrikeFoundNothing` or `AirAttack`. Then, in the same order, each attack's damage is applied and its report sent. Damage events from the air carry no round number.
18. **Strikes in flight are working data of the pulse,** held from the decision step to the air combat step and gone before the day's hash is taken. They are not state.

## Why

- **One pulse old** stops a group flying at a sighting from yesterday, and still lets an afternoon strike follow a morning sighting. It is a placeholder figure in the air combat file.
- **Only the named hex** is the simple rule. It makes report errors matter, which is the point of flying on belief, and it is harsher than what happened: real strikes searched around the reported position. The complexity review should weigh a search of the neighbouring hexes against that.
- **One ship** is what dive bombers did to a carrier. It wastes aircraft on a ship already sinking and never touches the escorts. Spreading a strike is deferred.
- **The same damage path as gunfire** is the rule-complexity rule at work: one mechanism, and a ship does not care what hit it.

## Deferred

- Fighters, escorts, anti-aircraft fire and aircraft losses: stage 5c.
- A share of aircraft held back, target priorities and range limits as orders.
- Searching round a contact's hex. Telling the side that a contact was not there.
- Strikes on bases and ground targets.
- Ordnance stocks, and how many aircraft a deck can launch in a pulse.
- A force attacked from the air becoming alert.
- A strike by night.

## Rule card

| | |
| --- | --- |
| Reads | The group's mission, aircraft and position; its ship's flight deck; its side's contacts (hex, kind, age); the aircraft type's weapon, damage and range; the air combat figures; for the attack only, the truth: enemy task forces in the target hex and their ships |
| Writes | The target ship's damage tracks and systems; a ship sunk; the attacking side's belief, through a sighting report |
| Events | `StrikeLaunched`, `StrikeFoundNothing`, `AirAttack`, then the damage, system and sinking events of decision 0014, then `ReportReceived` |
| Randomness | One stream per air group and task force attacked |
