# 0016: Damage to named systems, and stragglers

Status: confirmed by the designer, Nov 10, 2026, as stage 4a2 of scenario 4. Every figure is a placeholder. It replaces the system damage track of decision 0014.

## The gap

Decision 0014 gave a ship three damage tracks. Its "system" track was one lump: a hit could not knock out fire control and leave the guns working, or stop the engines and leave everything else. Nothing slowed a damaged ship, and a force had no way to shed one.

## Decisions

### Systems

1. **A ship has named systems, each working, degraded or out.** The system damage track is gone. Flotation and fire stay as tracks.
2. **Which systems a ship has follows from its class:** propulsion always; main battery and fire control if it has main guns; torpedo mounts if it has tubes. A new system is added by naming it and giving it one effect. A flight deck arrives with carrier operations in scenario 5.
3. **Each system has one effect:**

   | System | Degraded | Out |
   | --- | --- | --- |
   | Propulsion | Half speed | Dead in the water |
   | Fire control | Gun hit chance x 0.5 | Gun hit chance x 0.25 |
   | Main battery | Half the guns fire | No guns fire |
   | Torpedo mounts | Half the tubes fire | No tubes fire |

4. **Every hit still damages a track,** as in decision 0014: a gun hit lands on flotation or fire by set chances, a torpedo hit on flotation.
5. **Each hit may also knock one system down one level.** Each weapon has a chance of a system hit and a table of chances for which system. A hit on a system the ship does not have, or one that is already out, changes nothing. An event records it all the same.
6. **Fire uses the state at the start of the round:**
   - Gun shots = guns x the main battery factor x rounds per gun per minute x 5.
   - Gun chance to hit is also multiplied by the firer's fire control factor.
   - Torpedo shots = tubes x the torpedo mount factor, in the first round the ship fires. The torpedoes are then gone, however few tubes fired. If the mounts are out, nothing is fired and nothing is used.

### Speed

7. **A ship's speed is its class speed at the task force's setting, times its propulsion factor:** 1, 0.5 or 0.
8. **Task force speed is still the speed of its slowest ship afloat** (decision 0005). So one damaged ship slows the whole force, and a ship dead in the water stops it. A stopped force stays under way and burns nothing.
9. **Fuel burn is unchanged.** A ship at half speed is below its cruise speed and burns its cruise rate per mile (decision 0006).

### Stragglers

10. **A straggler is a ship whose propulsion is degraded or out.** "Crippled" keeps its meaning from decision 0014: flotation damage of 50 or more. The two are separate.
11. **A task force has a standing instruction for stragglers,** set with its rules of engagement: keep, detach, leave or retire. It also has a share to keep, from 0 to 1. The default is keep.
12. **The instruction is applied at the start of every movement step and at the end of every surface engagement step,** to each force that has a straggler and an instruction other than keep.
    - **Keep:** nothing happens.
    - **Leave:** every straggler goes into one new task force that stays where it is. The parent carries on.
    - **Detach:** ships dead in the water are left, as above. Stragglers that can steam go into a new task force with escorts and head for the home port. The number of escorts is the sum, over those stragglers, of a figure for each one's kind. Escorts are taken from ships with working propulsion and main guns, the least important kind first, the lowest ID on a tie. From least to most important the kinds are auxiliary, transport, destroyer, cruiser, battleship, carrier: the reverse of the order decision 0012 uses for the most important kind in a report. A ship with degraded propulsion is a straggler, so it is never an escort and does not count toward the share to keep. If giving the escorts would leave the parent with less than its share to keep of the ships with working propulsion it had, or with none, the force retires instead.
    - **Retire:** ships dead in the water are left. The rest of the force, stragglers included, heads for the home port together.
13. **A force made this way** takes the side, mission, speed setting and home port of its parent. It is set to avoid combat, and to keep its own stragglers. A force that retires is set the same way.
14. **Heading for the home port is an order.** The simulation issues a set-route order for the force, marked as issued by the staff, taken in at once. Its acceptance cites the event that detached or retired the force, and movement cites the order as usual. A force with no home port gets no order and stays where it is. *Note, Nov 13, 2026:* the order is issued by the local commander, not the staff, as decision 0055 makes it (`docs/rules/differences.md` row 39).
15. **Two orders from the spec's list arrive:** set home port, and detach ships. A detached force starts at its parent's exact position, part-way along a leg if the parent is. An event names the new force and cites the order.
16. **A force with no ship afloat is never sent anywhere.** If every ship of a force is dead in the water, they are left behind and the empty parent gets no order.

## Why

- **Three states and one effect each** can be read off a ship's card and worked by hand. Percentages on four systems could not.
- **A cripple slowing its force needs no rule.** It follows from the slowest-ship rule that already existed.
- **The instruction is the local commander's.** The player cannot act inside a day, and a commander with a cripple did not wait for tomorrow's orders. WitP:AE splits a damaged ship off automatically at fixed speed thresholds, with no choice (manual section 6.2.3.1).
- **Staff orders keep the trace whole.** Every movement cites an order, including one the simulation issued.

## What the staff planner must weigh later

The designer named four things that should decide how a straggler is handled. Two are in this rule in simple form. Two are judgements for the staff planner in step 3, which will issue its own detach orders and show its reasoning:

| Factor | Here | Staff planner |
| --- | --- | --- |
| The straggler's value | Escorts by its kind | Its real worth in the campaign |
| Whether the force can go on | The share to keep | How much the mission matters |
| Whether the straggler can defend itself | Not used | Its weapons and what is left of them |
| The dangers on the way home | Not used | Contacts, air threat, submarines |

## Legibility test

Run on Nov 10, 2026. A Haiku model was given this record, the combat figures and the event log of the straggler scenario, and nothing else. Of thirteen questions it answered eleven correctly: why that escort, why the dead ship was left, the check that chose detaching over retiring, why the detached force made 8 knots, why half speed saves no fuel per mile, where the staff order came from, why nothing more was detached later, and predictions for a carrier with two escorts and for a ship with no tubes.

It got two wrong, and both were gaps in what it was given, not in the rule: it was not told that a day starts at 18:00 (decision 0001), so it misread the clock, and it was not given the rule that hits are the whole part of the expected number plus one roll (decision 0014).

What it found hard led to three changes:

- This record did not say which kinds are less important. Decision 12 now lists them.
- It did not say whether a ship with degraded propulsion counts toward the share to keep. Decision 12 now says it does not.
- The log did not show who issued the route order. The command-line output now marks an order issued by the staff.

Still open: the log does not print the share-to-keep check itself. It is in the event's breakdown, which the command-line output does not show. That belongs with briefings.

## Deferred

- Towing and scuttling a ship dead in the water.
- Repair *(built: decision 0069, Nov 13, 2026)* and damage control, and fire that spreads or is fought.
- Flooding that slows a ship. Damage to radar and other sensors.
- Magazine explosions and other single hits that destroy a ship.
- Forming and merging task forces by order.
- The flight deck, with launching and landing, in scenario 5. *(Nov 14, 2026: the flight deck is a fifth system by decision 0026)*
- Real figures.

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| System hits (in fire, decision 0014) | Weapon; ship class; combat figures | Ship system states |
| Fire (amended) | Ship main battery, fire control and torpedo mount states, besides what decision 0014 lists | As decision 0014 |
| Ship speed (in movement, decision 0005) | Ship propulsion state | Nothing new: task force speed is derived |
| Stragglers | Task force instruction, share to keep, home port, position; ship propulsion state, class kind and guns; combat figures | Task force ships; new task forces; orders |
| Home port and detach order intake | Ownership; ship membership | Task force home port; task force ships; new task forces |

Movement now reads ship damage, which combat writes. Stragglers changes task force membership, which movement, cargo handling, search and combat all read.
