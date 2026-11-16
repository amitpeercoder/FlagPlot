# 0121: An escort force: keeping station, and who is met first

Status: settled by the architect on Nov 15, 2026, inside the plan for step 3, stage 2 (decision 0113, decision 14), on the designer's answer to its question 4: escorts sail as a force of their own, with a rule. It writes the rule out with its rule card and its hand calculation. Built on Nov 16, 2026, in step 5 of the stage, with the doctrine for escorted and covered runs, decision 0126 ("As built" below).

Summary: A task force on the escort mission may be given a charge, another force of its side: it keeps station on it without a route of its own, an enemy force that meets the charge must first fight every escort in station and reaches the charge only past an escort that is sunk or has broken off, and the escorts' guns fire with the charge's at aircraft that attack it

Standing: Settled by the architect, Nov 15, 2026, inside decision 0113. Built

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

## As built

Built on Nov 16, 2026, with decision 0126, in step 5 of stage 2. What follows is what was built and every place the build read the record more closely or differed from it.

**Versions.** The state version is 32 (a task force's `charge`, its force and the escort order, after its `landing_at`). The rules label is 15. The saved game is version 8 (the order `escort`), the doctrine `supply-12` (decision 0126), the agent interface version 12 (the order `escort` with `charge`, and a force's `charge` in its detail).

**The order and its refusals** (`TaskForceOrder::Escort(Option<TaskForceId>)`, intake in `resolve.rs`), checked in this order: the escort mission (`NotOnEscortMission`); the force no force's charge (`IsACharge`, naming the escort); for a charge named, not itself (`CannotEscortItself`), the side's (`NotYourUnit`), afloat (the refusal of a force that has ceased to be, or `NoShipAfloat`), with no charge of its own (`ChargeIsAnEscort`). A route order to a force with a charge is refused `KeepsStationOn`, naming the charge. A speed order is taken and is replaced by the next pulse's station. An escort order that clears the charge stops the force where it is and records nothing but the order and any turning back. Scenario files: `kind = "escort"`, `charge = N`, no `charge` to clear.

**Keeping station** (`crates/sim-core/src/escort.rs`) runs at the start of each pulse's movement, after the halts steamed on and the straggler pass. An escort in station takes its charge's waypoints still to reach, cut at the waypoint where the charge will halt in the pulse (the engine's own halt function), with no marks, at `Knots` of the speed the charge's slowest ship makes at its setting; where the charge will not steam (stopped, halted, out of fuel, no speed) the escort stops where it is. Out of station it is routed to the charge's hex at cruise. An escort out of fuel takes no station. The route is set as a route order's acceptance sets it (turning back between centres where that is shorter); the escort's route and speed orders become the escort order. A new event, `StationKept`, records the station where it changes the escort's route, speed or status, citing the escort order and, in station, the charge's route order: the record gave no event for it, and the traceability rule needs one. On a tie in time of two changes of position, a force that is no force's charge comes before one that is, so an escort that moves with its charge enters a hex first; with no charge in a game the order is as it was.

**Who is met first** (`movement.rs`, `World::meet`, `screen_passed`). An escort counts as passed when its fight with that enemy ended with it sunk to the last ship, or as the force that broke off or slipped away (slipping away is read as leaving); an escort whose fight passed unseen, or that is afloat in the hex with no fight held, is not passed. A reading the record did not state: where the escort sank the enemy, the meeting with the charge is one not held, and `MeetingNotHeld` is recorded (the hand calculation has it on every seed the escort holds). The pair is then marked as met for the pulse.

**When the charge is gone** (rule 7): a charge sunk in a fight leaves its escorts with no charge at the end of that fight (`ChargeLost` citing the escort order and the fight's end, the escort's open steaming recorded first); a charge stood down or joined, at the intake of that order (`ChargeLost` among the order's events); a charge sunk any other way (by air), at the start of the next movement, citing the escort order only. An escort itself joined or stood down loses its charge with it. Stopping where it is: at a centre no route, and idle if it was steaming; between centres routed to the hex it is in.

**The guns** (`airstrike.rs`): the shots of the ships of every escort in station on the force a raid attacks are added to that force's, from the same stream, keyed on the charge's lowest ship as before, and split by the same share.

**What a side sees.** `StationKept` and `ChargeLost` are the escort's side's; `MeetingNotHeld` the charge's side's. The agent's briefing does not fold them; they are in the side's events and `explain`.

**What it was checked against.** Scenario 51 (`data/scenarios/51_an_escort_force.toml`, on the frozen figures, and the variants its test builds) came out as the hand calculation has it in every number: part 1's hexes at the end of each pulse, the arrival at hour 20, the fuel of a force ordered the same route, closing from (0,2) in 5 hours, the slow escort, the refusals; part 2 on 200 seeds (the escort holds on all 200; passed and met after it on 193; unseen on 7, not passed); part 3's guns (6 and 6 shots of two destroyers given 6 each at a share of 0.5). Readings of the test: for part 2 the Allied forces start at (4,0) so that they reach T together in the night of day 2; part 3's raid is given a target list of transports, since by default it would take the escort; the slow escort is a class altered to make 10 knots.

**Compared with earlier scenarios.** `sh scripts/compare-scenarios.sh --against origin/main`: 168 runs, all the same with the hashes cut. Every earlier scenario test passes untouched.
