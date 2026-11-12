# Surface combat and damage

This chapter covers a fight between two task forces of opposing sides: when it begins, how it runs round by round, how a hit damages a ship, how it ends, and what the damage does afterwards. It also owns the hit mechanism that gunfire, torpedoes and bombs share: the shots, the hit chance, where a hit lands, and what a hit does to a ship. Chapter 5 points here and states only what differs for an air attack.

Numbers are fixed-point, and every multiplication and division below cuts, as chapter 1, Numbers and chance, says. Figures named as `surface_combat.toml: key` are read from `data/reference/surface_combat.toml`. Every figure in that file is a placeholder (decision 0014).

A scenario that names no surface combat figures has no fights, and no ship in it can be damaged (decision 0014).

## When a fight begins

### Meeting

Two task forces of opposing sides meet when one's position becomes a hex where the other is, or when both begin a pulse in one hex and neither will move in it. A task force with no ship afloat does not meet. [0014, 0022]

- Fights are held inside the movement step of a pulse, in order of clock time. The surface engagement step of the pulse is an empty slot. [0022]
- At the start of the movement step, every pair of opposing forces in one hex fights if neither force is moving. A force is moving if it is under way and has a budget above zero (chapter 2, The budget of a pulse). A force that is moving is leaving and does not fight at the start of the pulse. The list of pairs is taken once, in task force ID order, lower ID first. [0022]
- During the step, each time a force's position changes, it meets every opposing force with a ship afloat in the new hex, in order of the other force's ID. The list is taken once, so forces made by a fight are not on it. When several meetings fall at the same moment, they are taken in order of the lower task force ID, then the higher. [0022]
- A meeting is held only if both forces still have a ship afloat and are still in one hex when its turn comes. A force that an earlier fight at the same moment sent home may have left the hex. [0022]
- A pair fights at most once in a pulse. [0022]
- A force's position is the nearest hex centre, so a force that is under way enters a hex before it reaches the centre (chapter 2, Legs and position). [0005, 0022]
- A fight takes no time. Its rounds, at most `max_rounds` of `round_minutes` minutes each, cost the forces no steaming. After a fight each force carries on along its route for what is left of the pulse, at the speed its ships can then make (chapter 2, After a fight). [0022]
- A game with no surface combat figures has no meetings. [0014, 0022]
- Forces that meet and neither sights the other in `max_rounds` rounds pass unseen (see the ending checks). [0014]

A side sees: it learns of a fight from the round in which its own force first sights the enemy or is first fired on, and not before. A fight a side never knows of is wholly absent from its view, including the events that begin and end it. The enemy learns the same way. [0047]

Known limit: forces in neighbouring hexes do not fight, and a force cannot pursue one that breaks off. A commander's choice to press on or retire after a fight is not built. [0022]

### Darkness

A fight is dark if the pulse is the night pulse and light otherwise. The hex is not read (chapter 1, Darkness). [0001, 0014]

### What a fight holds

A fight holds the two forces (lower ID first), the hex, whether it is dark, and the ships each force had afloat when it began (`began_with`, in ship ID order). It records `EngagementBegun`, which cites the steaming event that brought a force into the hex, if one did, and each force's rules of engagement order, or failing that its route order. The fight's state is gone when the fight ends. [0014, 0022]

A side sees: `EngagementBegun` once the side knows of the fight, with its own force and without the enemy's. [0047, 0065]

## Alertness and readiness

Whether a force is alert, how a fight alerts it, the alarm that sounds when a fight ends and the fatigue that alertness causes are chapter 3, Readiness, alertness and fatigue of a force. A fight reads alertness in one place, Sighting below, and sets it in one place, step 5 of each round. [0021]

## The fight

A fight is a run of rounds, at most `surface_combat.toml: max_rounds`. A round is `surface_combat.toml: round_minutes` minutes long, which sets only the number of shots. Each round runs these steps in order. Everything in a round is worked out from the state at its start, and damage is applied at its end. [0014]

1. Sighting.
2. Slipping away.
3. Fire.
4. Damage.
5. Alertness: a force that sighted or was fired on this round is alerted if it is not already alert (chapter 3, Alerted inside a pulse).
6. The ending checks.

### Sighting

Each round, each force that has not yet sighted the enemy is taken in turn, lower ID first. [0014]

- A force that was fired on in the previous round has sighted the enemy now, with no roll and no draw. It counts as newly sighted this round. [0014]
- Any other force that has not sighted rolls. The chance is:

  `chance = min(1, (base + radar_bonus) x alert_factor x experience_factor) x fatigue_factor`

  - `base` is `surface_combat.toml: sighting_chance_night` if the fight is dark, else `sighting_chance_day`.
  - `radar_bonus` is `radar_sighting_bonus` if any ship afloat in the force has a class with `radar` true, else 0. Radar is a yes or no for a class. [0014, 0019]
  - `alert_factor` is 1 if the force is alert, else `not_alert_sighting_factor`.
  - `experience_factor` is 1 if the fight is light or `surface_combat.toml: experience_sighting_strength` is 0. In a dark fight with the strength above 0 it is `1 + strength x (mean - 2)`, where the mean is the sum of the experience of the ships in the force's list at the moment of the roll, over their count, in the fixed arithmetic (three ships of 3, 3 and 2 have a mean of 2.666666). It is inside the cap, so it cannot lift a chance past 1. [0064]
  - `fatigue_factor` is (100 less the force's fatigue) over 100, when the scenario names fatigue figures, else 1. It is applied after the cap at 1. The fatigue is its value after the start-of-pulse change (chapter 3, Fatigue of a force). [0037]
- The roll is one draw from a stream keyed on this force and the enemy force, one draw a round until it sights. If it sights, the force has sighted from now on. [0014]

A force that has sighted does not roll again. Both forces can sight in the same round. `SightingRoll` records each roll, with its chance, the fatigue factor and the result, and cites the beginning of the fight.

The breakdown of a roll names `experience_factor` after `alert_factor` (and before `fatigue_factor`) only in a dark fight with the strength above 0, as `fatigue_factor` is named only when fatigue is in play. [0064]

A side sees: its own sighting roll, only if it sighted, without the force sighted. A roll that did not sight is hidden from both sides. The breakdown of the roll is shown whole, so the side sees its own force's experience factor and never the enemy's. [0047, 0064, 0065]

### Radar of unknown fit

Where no source says a ship carried radar, the scenario treats the class as not fitted. This is a working assumption named in the scenario's ship list and is not a fact. [0019]

### Slipping away

After sighting, a force slips away if its effective stance is avoid, it has sighted the enemy, and the enemy has not. The forces are tested lower ID first and the first that qualifies slips. [0014]

- The fight ends at once for the round. No fire is worked out and no damage is done. [0014]
- `ForceSlippedAway` records it. It cites the force's successful sighting roll of this round, or else the beginning of the fight, and the force's rules of engagement order. [0014]
- The slipping force avoids action for the rest of the pulse. [0022]
- Forces that sighted this round still become alert. [0014]
- The fight ends with reason slipped away.

The effective stance of a force is avoid if it broke off or slipped away earlier in this pulse, whatever its orders say, and otherwise its ordered stance. The ordered stance is engage or avoid, set by order, default engage. [0014, 0022]

If both forces have sighted, neither slips, whatever their stances. [0014]

A side sees: `ForceSlippedAway` for the force that slipped, without the force it slipped away from. The other side does not see it. [0047, 0065]

### Who fires

Every ship afloat at the start of the round, of every force that has sighted, fires. A ship that is sunk in the round still fires in it. [0014]

Ships are matched in ship ID order. The ship in place i of a force's afloat list takes the enemy ship in place (i modulo the number of enemy ships afloat). Every ship afloat counts for its place, armed or not, and an unarmed ship fires nothing. The target is chosen at the start of the round. Burning or nearer ships do not draw fire. A ship fires its gun salvo first and then its torpedo salvo, both at the same target. [0014]

Known limit: there is no range. There is one night chance and one day chance, and no opening or closing of the range. Target choice by value, fire or distance is not built. [0014]

A side sees: its own ship's salvo, cut: the round, the firer, the weapon and the shots, without the target, the chance, the expected hits or the hits scored. The side whose ship is the target sees that it was fired on: the round, the target and the weapon, without the firing ship (decision 0065). The firer's side keeps the breakdown factors that are its own (guns, tubes, rate, minutes, the main battery, the share of its guns for being not ready, torpedo mount, fire control and experience factors). The side fired on is given no breakdown, so the enemy never sees that a force fired part of its guns. [0047, 0064, 0068]

### Surprise

A force is surprised in a round if it has not sighted the enemy after the sighting step, the enemy has, and at least one ship of the enemy has shots above zero to fire this round, from its guns or its torpedoes. A force that is surprised does not fire: only forces that have sighted fire. Fire at a surprised force is more accurate (see the hit chance). `ForceSurprised` records it, citing the enemy's sighting roll or else the beginning of the fight. [0014]

A side sees: `ForceSurprised` for the force that was surprised, without the force that surprised it. The enemy does not see it. [0047, 0065]

### Shots

Guns and torpedoes use one mechanism: shots, a chance to hit, damage per hit. Air attacks use it too (chapter 5, The attack on a ship). [0014, 0016, 0026]

**Gun shots in a round**

`full = gun_rounds_per_min x (main_guns x round_minutes)`, then `shots = min(full x main_battery_factor x unready_gun_share, ammunition aboard)`. The share comes before the ammunition cap, as the main battery factor does.

- `gun_rounds_per_min`, `main_guns` and `gun_rounds_per_gun` are keys of the ship class (`ship_classes*.csv`). `round_minutes` is `surface_combat.toml: round_minutes`.
- The main battery factor is 1 if the main battery is working, `degraded_factor` if degraded, 0 if out. [0016]
- `unready_gun_share` is `surface_combat.toml: unready_gun_share`, from 0.1 to 1 (the loader refuses a value outside it), if the firing ship's force was not ready when the fight began (see Ready, below), else 1. The same figure holds for every ship of the force and for every round of the fight. At 1 the rule does nothing. [0068]
- Ammunition is the ship's `gun_rounds`. A scenario ship starts with `main_guns x gun_rounds_per_gun` unless the scenario gives another figure. Each salvo takes its shots from it. [0014]
- For some ships the derivation writes 9,999 rounds a gun as a marker for ammunition that is not limiting, where no source gives an outfit. [0023]

**Ready, and the share of the guns**

A force is ready for action, or not, once, as a fight begins, and the answer is kept for the whole of that fight: no sighting, no fire received, no alert gained and no round changes it. [0068]

- A force is ready if its readiness is battle (by order or from the start of the scenario), or if it was alert before the present pulse began. [0068, 0021]
- A force at normal readiness that was alerted in the present pulse is not ready: by a contact at the start of this pulse (chapter 3, Alert from belief), by being sighted or fired on in an earlier fight of this pulse, or by the alarm of a fight earlier in this pulse. A force alerted in an earlier pulse and still alert is ready. A force at normal readiness that is not alert is not ready. When a force was alerted is read from the event that alerted it (`alerted_by`, an event of a day and a pulse); no field of the state holds it. An alerted force with no such event is taken to have been alerted before. [0068]
- A force that is not ready fires every ship's guns at `unready_gun_share` of their shots in every round of that fight, as the formula above says. Torpedoes, the chance to hit, sighting, surprise, the alarm and breaking off are unchanged. The force's readiness does not rise during the fight: a force that becomes alert in a fight is not ready in a later fight of the same pulse either, and is ready in a fight of a later pulse only if it is still alert then (chapter 3, Alert from belief, says when it stands down). [0068]
- The figure is a placeholder (0.5 in the live file, the half of the armament manned that Condition II means; 1 in the frozen step 1 file). It is one figure for both sides and every battle. [0068]

**Torpedo shots**

Torpedo shots are `torpedo_tubes x torpedo_mount_factor`, in the first round in which the ship has torpedo shots to fire, and only if the ship's `torpedoes_ready` flag is set and its class has tubes. The factor is 1, `degraded_factor` or 0 as for the main battery. A ship's torpedoes are fired once. When a torpedo salvo with shots above zero is fired, `torpedoes_ready` is cleared, however few tubes fired. If the mounts are out, nothing is fired and nothing is used, and the flag stays set. A scenario ship with tubes starts with the flag set. [0014, 0016]

A shot count can have a fractional part (for example 67.5 shots). A salvo with no shots is not fired and records no event.

### The hit chance

`chance = base x surprised_factor x lit_factor x fire_control_factor x experience_factor`, multiplied in that order.

- `base` for a gun is `surface_combat.toml: gun_hit_chance_night` or `gun_hit_chance_day`. For a torpedo it is `torpedo_hit_chance_night` or `torpedo_hit_chance_day`. Night means the fight is dark. [0014]
- `surprised_factor` is `surprised_target_factor` if the target's force is surprised this round, else 1. [0014]
- `lit_factor` is `lit_target_factor` if the fight is dark and the target has any fire damage above zero, else 1. The target's fire damage is as it stood at the start of the round. [0014]
- `fire_control_factor` is the firer's fire control factor for a gun, else 1 for a torpedo. It is 1 if fire control is working, `degraded_factor` if degraded, and `fire_control_out_factor` if out. [0016]
- `experience_factor` is 1 if the fight is light, or if the strength for the weapon is 0 (`surface_combat.toml: experience_gun_strength` for a gun, `experience_torpedo_strength` for a torpedo). In a dark fight with the strength above 0 it is `1 + strength x (experience - 2)` with the firing ship's own experience (a whole number from 0 to 4, 2 unless the scenario's `[[ship]]` gives `experience`; the loader refuses a value outside 0 to 4; a field of the ship set when the scenario loads and written by no rule). It is multiplied in last, so that a factor of 1 changes nothing to the last digit. With a strength of 0.5 a crew of experience 0 has a factor of 0 and its shots hit nothing. Experience is not read by day. [0064]

The chance is not capped when it is worked out. When it is rolled it is clamped between 0 and 1. The expected hits recorded are `shots x chance`. Radar does not enter the hit chance. It enters sighting only. [0014, 0016]

Known limit: experience is read by ships in a dark fight only, and not at all by aircraft (stage 2 of decision 0064 is not built). It does not change with time or action: it is set when the scenario loads. A force's mean is of the ships in its list now, so a sunk ship stops counting at once. [0064]

### Each shot is rolled

Each whole shot is one draw at the chance. A part of a shot is one more draw at the chance times the part. The hits are the draws that succeed. With no fractional part there is no extra draw. The number of draws depends only on the shots. [0033]

Draws come from the firing ship's fire stream, keyed on the firing ship and the enemy force. It is opened when the ship first fires and kept for the whole fight. For each salvo the draws come in this order: the shots, then for each hit its track (guns only) and then its system. A ship's gun salvo is drawn before its torpedo salvo. [0014, 0033]

`Salvo` records the round, the firer, the target, the weapon, the shots, the chance, the expected hits and the hits. It cites the beginning of the fight and carries a breakdown of the shot count factors (guns or tubes, rate, minutes, the system factor) and the hit chance factors (base, surprised, lit, for guns fire control, and last the experience factor, named only in a dark fight when the weapon's strength is above 0, so on the frozen figures no breakdown changes). A gun salvo's breakdown names `unready_gun_share` straight after `main_battery_factor`, and only when the firing force was not ready and the figure is below 1; a ready force's salvo and any torpedo salvo never name it. [0014, 0033, 0064, 0068]

### Where a hit lands

Each hit has damage points: `100 x weapon damage / toughness`, in one product and one truncating division. The weapon damage is the firing class's `gun_damage` or `torpedo_damage`. The toughness is the target class's `toughness`. [0014, 0026, 0032]

- A gun hit lands on flotation if its draw is below `surface_combat.toml: gun_hit_flotation`, else on fire. A torpedo hit always lands on flotation and makes no track draw. [0014, 0032]
- Each hit also makes one system draw, always, even if the chance below is 0. The system chance is `min(1, points x system_hit_chance_per_point)`. The systems take consecutive bands of the draw, in the code's order of the systems (`ShipSystem::ALL`): propulsion, fire control, main battery, torpedo mounts, flight deck. This order is the same for every table. Each band is as wide as the system chance times the system's share in the table of the weapon: `gun_system_hit` for a gun and `torpedo_system_hit` for a torpedo here, and `air_combat.toml: bomb_system_hit` for a bomb (chapter 5, The attack on a ship). A system absent from a table has share 0, so its band has no width. A draw beyond the last band hits no system. A system the ship does not have is still hit, and nothing changes. [0016, 0033]
- Neither table in `surface_combat.toml` names the flight deck. A surface hit cannot land on it. Bombs have their own table (chapter 5, The attack on a ship). [0026]

## Damage

One mechanism applies damage to a ship, shared by surface fire and air attack. [0026]

### Damage tracks

A ship has two damage tracks, flotation and fire, each from 0 to 100. A hit adds its points to its track, 100 at most. `ShipDamaged` records the hits on one track from one salvo, with the points and the value after. The third track of decision 0014, system damage, no longer exists: it was replaced by named systems. [0014, 0016]

- Flotation damage of 100 sinks the ship. Nothing else sinks it. [0014]
- Flotation damage of `surface_combat.toml: crippled_flotation` or more makes a ship crippled. A crippled ship counts toward its force's break-off level. It has no other effect. [0014]
- Fire damage above 0 lights the ship at night: it is easier to hit (the lit factor). Fire damage of `claimed_sunk_fire` or more makes the enemy claim it as sunk (chapter 3, Claims). It has no other effect. [0014]
- Fire does not grow, spread or burn out, and is not fought. Fire damage changes only by hits. [0014]

A side sees: `ShipDamaged`, `SystemHit` and `ShipSunk` whole, for its own ships. It does not see the damage it does. [0047]

### Applying damage

Damage lands at the end of the round, one salvo at a time in the order they were fired. For each salvo, `ShipDamaged` is recorded for the flotation track if it took hits, then for the fire track, then `SystemHit` for each system hit in hit order. Each event cites the salvo. [0014, 0016]

- Points added to a track are `damage per hit x hits on that track`. [0014]
- A ship sunk earlier in the same round still takes the salvo's damage and events, and sinks only once. [0014]
- A track hit that takes flotation to 100 from below sinks the ship. [0014]

### Sinking

A ship that sinks has its location set to sunk and leaves its task force. It is in no task force and no port and takes no further part in anything. `ShipSunk` records it and cites the damage event. Every air group aboard is lost with it, with all its aircraft and pilots (chapter 5, A group lost with its ship). [0014, 0025]

### Named systems

A ship has named systems, each working, degraded or out. Which it has follows from its class: propulsion always; main battery and fire control if the class has main guns; torpedo mounts if it has tubes; the flight deck if it carries aircraft. A system the class does not have stays working. [0016, 0026]

A system hit steps a system down one level: working to degraded, degraded to out. Out stays out. A hit on a system the ship does not have changes nothing, and `SystemHit` still records it with the state before and after. [0016]

| System | Degraded | Out |
| --- | --- | --- |
| Propulsion | Speed times `degraded_factor` | Speed 0, dead in the water |
| Fire control | Gun hit chance times `degraded_factor` | Gun hit chance times `fire_control_out_factor` |
| Main battery | Gun shots times `degraded_factor` | No gun shots |
| Torpedo mounts | Torpedo shots times `degraded_factor` | No torpedo shots |
| Flight deck | Aircraft that can fly times `degraded_factor` | No aircraft can fly |

The factor of a system is `surface_combat.toml: degraded_factor` when degraded, 1 when working, 0 when out, except fire control, which is `fire_control_out_factor` when out. [0016]

### The flight deck

The flight deck is the fifth system. It limits the aircraft the ship's air groups can fly in a pulse (chapter 5, What a group can fly in a pulse). Bombs can hit it, and surface fire cannot, because no surface table names it. [0026]

A side sees: `SystemHit` whole, for its own ships. [0047]

## Breaking off

### Rules of engagement

Each force has rules of engagement, set by one order and kept until another replaces them. [0014, 0016]

| Part | Meaning | Default |
| --- | --- | --- |
| Stance | Engage or avoid | Engage |
| Break-off level | The share of ships sunk or crippled, from 0 to 1, at which the force breaks off | 0.5 |
| Straggler instruction | Keep, leave, detach or retire (chapter 2, Stragglers) | Keep |
| Keep share | The share of its sound ships a force keeps when it detaches stragglers, from 0 to 1 | 0 |

A side sees: its own rules of engagement order accepted or rejected, whole. [0047]

### The ending checks

At the end of each round, after damage and alertness, the checks run in this order and the first that ends the fight ends it. [0014]

1. **A force with no ship afloat.** The fight ends with reason force sunk, citing the last ship sunk that round.
2. **Break-off.** Each force is tested, lower ID first. Both can break off in one round. A force breaks off if either of these holds:
   - **Over its level.** The number of ships it began the fight with that are now sunk, or afloat with flotation damage of `crippled_flotation` or more, is at least its break-off level times the number it began with. The test is made without dividing. A level of 0 is always met.
   - **Avoiding under fire.** Its effective stance is avoid and it was fired on in this round.

   `ForceBrokeOff` records it, with the share (lost over began) and a breakdown of the count lost, the count it began with and the level. It cites the damage events to the force this round, else the first salvo fired at it, else the beginning of the fight, and the force's rules of engagement order. A force that breaks off avoids action for the rest of the pulse. The fight ends with reason broke off. [0014, 0022]
3. **The last round.** In the last round, if neither force has sighted, `ForcesPassedUnseen` is recorded and the fight ends with reason passed unseen. Otherwise the fight ends with reason reached the last round.

The count is over the ships the force had when the fight began, so a ship sunk in an earlier fight of the pulse has already left the force and is not counted. A force that lost ships in an earlier fight may not break off from the next one at once, though it still avoids action. [0033]

If neither force sights the enemy in `max_rounds` rounds, they pass unseen. [0014]

### Avoiding for the pulse

A force that breaks off, or slips away, avoids action for the rest of the pulse. It takes the avoid stance in every fight it has: it slips away if it sights an enemy that has not sighted it, and otherwise fights back and breaks off at the end of the first round in which it is fired on. At the start of the next pulse it takes up its ordered stance again. This is not stored. The `ForceBrokeOff` and `ForceSlippedAway` events are the record. Breaking off does not move a force: it keeps to its route, and a force with no route stays. [0022]

A side sees: `ForceBrokeOff` whole and `ForceSlippedAway` without the enemy's force, for its own force. The enemy does not see them. A force that breaks off in a round in which it has neither sighted nor been fired on is not shown the break-off, because showing it would say an enemy was there. [0047]

## The end of the fight

When the fight ends, `EngagementEnded` records the round, the hex and the reason, and cites the ending causes. Then, lower force first, each force reports and claims. After them, if any salvo was fired, the alarm sounds. [0014, 0021]

A side sees: `EngagementEnded` once it knows of the fight, with its own force and without the enemy's. [0047, 0065]

### The report and the claim

After `EngagementEnded`, lower force first, each force sends its sighting report and then, if it fired, its claim. A force that sighted the enemy or was fired on in the last round reports; a force that fired claims. What each holds, how the report's errors are drawn and what a claim counts are chapter 3, Reports from a surface fight and The claim after a surface fight. The report makes or refreshes a contact through `apply_report`, and the claim is stored as a claim, so real damage goes into truth and what the side believes can differ from it. [0012, 0014, 0056]

A side sees: its own `ReportReceived` and `ClaimReceived`, whole. It never sees the hits it scored. [0047]

## What damage does afterwards

- **Speed.** A damaged propulsion slows the ship, and the force steams at its slowest ship's speed. A ship dead in the water stops the force, which stays under way and burns nothing. Chapter 2, Speed, gives the rule. [0005, 0016, 0021]
- **Guns and sighting.** Main battery, torpedo mounts and fire control change shots and hit chances as the table of named systems gives. Radar is not damaged and no sensor is. [0016]
- **Stragglers.** A straggler is a ship whose propulsion is degraded or out. It is not the same as a crippled ship. The standing instruction for stragglers is applied after each fight to the two forces that fought, lower number first, and at the start of every movement step. Chapter 2, Stragglers, gives the rule. [0016, 0022]

## No repair

No rule ever lowers a ship's flotation damage, fire damage or the state of a system, in port or at sea. A damaged ship stays damaged for the rest of the game. A system once out stays out. Aircraft repair is a separate rule for aircraft and is not this. [0014, 0016, 0052]

Known limit: ship repair is deferred to early in step 2. Flooding that slows a ship, magazine explosions and damage to radar are not built. Shore bombardment and ships at anchor as targets are not built. Real hit chances and damage come from sources in step 2. [0014, 0016, 0052]

## Random draws

| Draw | Stream keyed on | Order |
| --- | --- | --- |
| Sighting | The force and the enemy force | One a round until it sights |
| Fire | The firing ship and the enemy force | Per salvo: the shots, then for each hit the track (guns) and the system. Gun salvo before torpedo salvo |
| Alarm | The force alone | One for each fight it hears in a pulse, in the order the fights end |
| Report error | The force and the enemy force, subsystem intelligence | Chapter 3, How each part is drawn |

All randomness goes through `World::rng` or `World::rng_pair`. [0003, 0013, 0014, 0021]

## Not in this chapter

- Movement, routes, speed, stragglers and fuel burn: chapter 2.
- Search, reports, contacts, their error model, claims, and who is alert and why: chapter 3.
- Air strikes on ships, anti-aircraft fire, fighters, the bomb table and the flight deck's use: chapter 5.
- Air groups lost with a sunk ship, pilots, and aircraft repair: chapter 5.
- Orders, their intake and their checks, including the rules of engagement order: chapter 1, Orders.
- How a side is shown events and what is cut: chapter 1, What a side sees of an event.
- Objectives and the result a battle's losses feed: chapter 6.
