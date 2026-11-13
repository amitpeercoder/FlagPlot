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

A side sees: it learns of a fight from the round in which its own force first sights the enemy or is first fired on, and not before. A fight a side never knows of is wholly absent from its view, including the events that begin and end it. The enemy learns the same way. [0047] The side is told the fight from that round, as its round 1, and is told a sinking or a break-off as the ending only if its own force made it, otherwise "contact lost" (chapter 1, A surface fight). [0085]

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
  - `radar_bonus` is `radar_sighting_bonus` if any ship afloat in the force has a class with `radar` true, else 0. Radar is a yes or no for a class. A derived class has it only if a ship of it has a surface-search set whose display has a plan position indicator, read from the facts; an air-search set does not give it, and a surface-search set with no row for its display does not either and is named as not counted. [0014, 0019, 0077]
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

A ship chooses a target for its guns and a target for its torpedoes at the start of the round, among the enemy ships afloat. Kinds are in the order of importance: carrier, battleship, cruiser, destroyer, transport, auxiliary. **Guns:** a ship fires at enemy ships of its own kind; if none is afloat, at those of the nearest more important kind; if none of those, at those of the most important kind afloat. Among them, in ship ID order, it takes the ship in place (p modulo their number), where p is the firing ship's place among the ships afloat of its own kind in its force, in ship ID order. **Torpedoes:** a ship fires at the enemy ships of the most important kind afloat, taking the ship in place (i modulo their number), where i is the firing ship's place among the ships afloat in its force, in ship ID order. Places are counted from 0: the first ship is in place 0, so with three targets the firer in place 4 takes the second. Every ship afloat counts for its place, armed or not, and an unarmed ship fires nothing. Burning or nearer ships do not draw fire. A ship fires its gun salvo first and then its torpedo salvo. [0014, 0078]

Known limit: there is no range. There is one night chance and one day chance, and no opening or closing of the range. Target choice is by kind, as above; choice by value, fire or distance is not built. [0014, 0078]

A side sees: its own ship's salvo, cut: the round, the firer, the weapon and the shots, without the target, the chance, the expected hits or the hits scored. The side whose ship is the target sees that it was fired on: the round, the target and the weapon, without the firing ship (decision 0065). The firer's side keeps the breakdown factors that are its own (guns, tubes, rate, minutes, the main battery, the share of its guns for being not ready, torpedo mount, the share of its torpedoes for being not ready, fire control and experience factors). The side fired on is given no breakdown, so the enemy never sees that a force fired part of its guns or its torpedoes. [0047, 0064, 0068, 0071]

### Surprise

A force is surprised in a round if it has not sighted the enemy after the sighting step, the enemy has, and at least one ship of the enemy has shots above zero to fire this round, from its guns or its torpedoes. A force that is surprised does not fire: only forces that have sighted fire. Fire at a surprised force is more accurate (see the hit chance). `ForceSurprised` records it, citing the enemy's sighting roll or else the beginning of the fight. [0014]

A round in which one force fires and the other, surprised, does not is that force's **free round**. The fired-on force has sighted from the next round (Sighting, above). So a fight between two forces that engage opens in one of four ways (an avoiding force may also slip away, above), decided by the first round in which either force sights: one force alone sights and has a free round; the other alone sights and has it; both sight first in the same round, in round 1 or later, and fire from that round on, which is called **both firing from the start** (the start of the fire, not of the fight); or neither ever sights and they pass unseen. [0014]

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

**Ready, and the shares of the guns and the torpedoes**

A force is ready for action, or not, once, as a fight begins, and the answer is kept for the whole of that fight: no sighting, no fire received, no alert gained and no round changes it. [0068]

- A force is ready if its readiness is battle (by order or from the start of the scenario), or if it was alert before the present pulse began. [0068, 0021]
- A force at normal readiness that was alerted in the present pulse is not ready: by a contact at the start of this pulse (chapter 3, Alert from belief), by being sighted or fired on in an earlier fight of this pulse, or by the alarm of a fight earlier in this pulse. A force alerted in an earlier pulse and still alert is ready. A force at normal readiness that is not alert is not ready. When a force was alerted is read from the event that alerted it (`alerted_by`, an event of a day and a pulse); no field of the state holds it. An alerted force with no such event is taken to have been alerted before. [0068]
- A force that is not ready fires every ship's guns at `unready_gun_share` of their shots in every round of that fight, and every ship's torpedoes at `unready_torpedo_share` of their shots in each round in which a side of them is fired, as the formulas above and below say. [0083] Being ready changes the shots and nothing else: the chance to hit, surprise, the alarm and breaking off do not read it, and sighting reads whether the force is alert (Sighting, above, and chapter 3), not whether it is ready. A force at normal readiness that is not alert is both not ready and not alert, so it fires at the shares and sights at `not_alert_sighting_factor`, each by its own rule. The force's readiness does not rise during the fight: a force that becomes alert in a fight is not ready in a later fight of the same pulse either, and is ready in a fight of a later pulse only if it is still alert then (chapter 3, Alert from belief, says when it stands down). [0068, 0071]
- Each figure is a placeholder (0.5 in the live file, 1 in the frozen step 1 file, at which the rule does nothing). The gun share's 0.5 is the half of the armament manned that Condition II means; the torpedo share's first value mirrors it and has no row of its own, and the one count the record states is Bagley's four of sixteen tubes. Each is one figure for both sides and every battle. [0068, 0071]
- The torpedo share does not carry how many tubes bear on a broadside, how many ships fire, the range or the chance to hit. [0071]

**Torpedo shots**

A ship fires her torpedoes in at most two salvos, one side's tubes and then the other's (decision 0083). The class has two figures: `torpedo_tubes`, every tube aboard (T), and `torpedo_tubes_one_side`, the tubes that bear on one side (B), from 0 to T and above 0 when T is. A derived class takes B from the rows (decision 0074); a class with centreline mounts, and every placeholder and frozen class, has B equal to T. [0074, 0083]

- The tubes of a salvo are B for the first, and T − B for the second, which is fired only if T is above B. A ship's `torpedoes_one_side_fired` flag says which is next: clear for the first, set for the second.
- Torpedo shots are `the salvo's tubes x torpedo_mount_factor x unready_torpedo_share`, in the first round in which the ship has torpedo shots to fire, and only if the ship's `torpedoes_ready` flag is set and its class has tubes. The factor is 1, `degraded_factor` or 0 as for the main battery.
- The first side is fired in the first round in which the ship has torpedo shots to fire. The second side is fired in the next round in which she has torpedo shots to fire, in the same fight or a later one. Its target is chosen at the start of its own round, as every target is. [0083]
- When a torpedo salvo with shots above zero is fired, its side is spent, however few tubes fired. After the first side of a class with T above B, `torpedoes_one_side_fired` is set and `torpedoes_ready` stays set. After any other salvo, `torpedoes_ready` and `torpedoes_one_side_fired` are both cleared. If the mounts are out, nothing is fired and nothing is used, and both flags stay as they are. A scenario ship with tubes starts with `torpedoes_ready` set and `torpedoes_one_side_fired` clear. [0014, 0016, 0083]
- Where B equals T there is one salvo of all the tubes, and nothing differs from the rule before decision 0083.

Known limit: there are no bearings, so the second side is fired at the first chance; in the record ships took five to fifteen minutes to turn it on. There are no reloads: each tube fires once in a game. [0083]

- `unready_torpedo_share` is `surface_combat.toml: unready_torpedo_share`, from 0.1 to 1 (the loader refuses a value outside it), if the firing ship's force was not ready when the fight began (see Ready, below), else 1. The same figure holds for every ship of the force and for the whole fight. It applies to each salvo, the first side's and the second's, at the readiness of the force in the fight that salvo is fired in. The tubes of a side that did not fire are not kept for a later round. At 1 the rule does nothing. It is a figure of its own, apart from `unready_gun_share`. [0071]

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

Each whole shot is one draw at the chance. A part of a shot is one more draw at the chance times the part. The hits are the draws that succeed. With no fractional part there is no extra draw. The number of draws depends only on the shots. Each draw is its own, so n whole shots at chance p all miss with chance (1 − p)^n, and score at least one hit with chance 1 − (1 − p)^n. [0033]

Draws come from the firing ship's fire stream, keyed on the firing ship and the enemy force. It is opened when the ship first fires and kept for the whole fight. For each salvo the draws come in this order: the shots, then for each hit its track (guns only) and then its system. A ship's gun salvo is drawn before its torpedo salvo. [0014, 0033]

`Salvo` records the round, the firer, the target, the weapon, the shots, the chance, the expected hits and the hits. It cites the beginning of the fight and carries a breakdown of the shot count factors (guns or the salvo's tubes, B or T − B for a torpedo salvo (0083), rate, minutes, the system factor) and the hit chance factors (base, surprised, lit, for guns fire control, and last the experience factor, named only in a dark fight when the weapon's strength is above 0, so on the frozen figures no breakdown changes). A gun salvo's breakdown names `unready_gun_share` straight after `main_battery_factor`, and a torpedo salvo's names `unready_torpedo_share` straight after `torpedo_mount_factor`; each only when the firing force was not ready and its figure is below 1. A ready force's salvo never names either, and a gun salvo never names the torpedo share nor a torpedo salvo the gun share. [0014, 0033, 0064, 0068, 0071]

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
- Flotation damage of `surface_combat.toml: crippled_flotation` or more makes a ship heavily damaged. A heavily damaged ship counts toward its force's break-off level. It has no other effect. (The figure, the break-off event's text and its breakdown keep the older word, crippled; in this book and the glossary a crippled ship is the objectives' one, with propulsion or flight deck out, chapter 6.) [0014]
- Fire damage above 0 lights the ship at night: it is easier to hit (the lit factor). Fire damage of `claimed_sunk_fire` or more makes the enemy claim it as sunk (chapter 3, Claims). It has no other effect. [0014]
- Fire does not grow, spread or burn out, and is not fought. Fire damage changes by hits, and falls only by repair (below). [0014, 0069]
- A ship starts a scenario with no flotation or fire damage unless its `[[ship]]` entry gives `flotation_damage` (from 0 to below 100: a ship sunk is not given this way) or `fire_damage` (from 0 to 100), each a whole number or a decimal string; the loader refuses a value outside the range and a float. [0069]

A side sees: `ShipDamaged`, `SystemHit` and `ShipSunk` whole, for its own ships. It does not see the damage it does. [0047]

### Applying damage

Damage lands at the end of the round, one salvo at a time in the order they were fired. For each salvo, `ShipDamaged` is recorded for the flotation track if it took hits, then for the fire track, then `SystemHit` for each system hit in hit order. Each event cites the salvo. [0014, 0016]

- Points added to a track are `damage per hit x hits on that track`. [0014]
- A ship sunk earlier in the same round still takes the salvo's damage and events, and sinks only once. [0014]
- A track hit that takes flotation to 100 from below sinks the ship. [0014]

### Sinking

A ship that sinks has its location set to sunk and leaves its task force. It is in no task force and no port and takes no further part in anything. `ShipSunk` records it and cites the damage event. Every air group aboard is lost with it, with all its aircraft and pilots (chapter 5, A group lost with its ship), and so are all the men aboard (chapter 7, Lost at sea). [0014, 0025, 0066]

### Named systems

A ship has named systems, each working, degraded or out. A ship starts the scenario with every system working unless its `[[ship]]` entry sets one (`propulsion`, `fire_control`, `main_battery`, `torpedo_mounts` or `flight_deck`); the loader refuses a damaged state for a system its class does not have. Which it has follows from its class: propulsion always; main battery and fire control if the class has main guns; torpedo mounts if it has tubes; the flight deck if it carries aircraft. A system the class does not have stays working. [0016, 0026]

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
| Break-off level | The share of ships sunk or heavily damaged, from 0 to 1, at which the force breaks off | 0.5 |
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
- **Stragglers.** A straggler is a ship whose propulsion is degraded or out. It is not the same as a heavily damaged ship. The standing instruction for stragglers is applied after each fight to the two forces that fought, lower number first, and at the start of every movement step. Chapter 2, Stragglers, gives the rule. [0016, 0022]

## Repair

A ship's damage is mended in two ways, and nowhere else. Aircraft repair is a separate rule for aircraft and is not this (chapter 5). Both ways read `surface_combat.toml` and a base's keys; a scenario with no base that has a repair figure or a yard passage has no ship repair. Neither uses a random number. [0014, 0016, 0052, 0069]

### Forward repair

At the start of each day, after the day's orders are taken in (chapter 1, Before the pulses), a base that mends points of damage a day spends them, with no order, on the forces of its side stopped in its hex.

- **The allowance.** Each base's allowance for the day is its `ship_repair_points_per_day`. It is shared by every force of its side stopped there, and what is not spent is lost at the end of the day. A base with 0 mends nothing.
- **Who is served.** A force is served if it is stopped (not under way and with no hex ahead, as for refuelling, chapter 2), is not away at the yard and has a ship afloat. A force ordered to sail that morning is under way when the repair runs and is not served. A force that was given an order that was refused, or one that does not move it, is stopped and is served. A ship lying in port (`ShipLocation::Port`, a ship that is in no force) is not repaired, and no scenario of the slice starts a damaged ship there. [0069]
- **In which order.** Forces in order of their number, ships within a force in order of theirs. Each ship takes from the lowest-numbered base of its side in the hex whose allowance left is above 0. If there is none, the rest of the force gets nothing.
- **How a ship's points are spent.** In this order, each step taking the smaller of the allowance left and what the ship needs:
  1. Fire damage, down toward 0.
  2. Flotation damage, down toward 0, only if the ship's flotation damage is below `forward_repair_max_flotation` at the start of this step. Flotation damage at or above the figure is not touched.
  3. Degraded systems, in the order of the named systems (propulsion, fire control, main battery, torpedo mounts, flight deck; the order of Where a hit lands), the first degraded one only at a time. A system needs `forward_repair_system_points` less the points already banked on the ship. If the allowance left covers that, the system works again, the bank returns to 0 and the next degraded system is taken. If not, all the allowance left goes into the bank and the ship is done.
- **The bank.** A ship's `system_repair_points` holds the points put into her first degraded system that a base has not finished, so a base that mends fewer points a day than a system needs still mends it in time. It is from 0 up to below `forward_repair_system_points`. The banked points belong to the first degraded system in order at the time they are spent: if that system is then hit to out, they go to the next degraded one. When step 3 finds no degraded system the bank is set to 0, spending nothing.
- **A system that is out is never mended forward.** Only the yard mends it. Forward repair never adds damage, and leaves fuel, ammunition and torpedoes as they are.
- **What is recorded.** A ship that takes points records one `ShipRepaired`, after her points are spent and before the next ship is served: the points spent on her this day, her fire and flotation damage after, the systems made to work, and her bank after. It cites the force's arrival event, if it has one. A ship that takes no points records nothing. The one exception is a ship whose bank was set to 0 at step 3: that is a change to her, so if it is the only change that day it is recorded as a `ShipRepaired` of 0 points. Its breakdown holds the allowance before and after, the fire, flotation and system points spent, and `forward_repair_max_flotation` when her flotation damage was too high to be mended. [0069]

### The yard

A force stopped at a base of its side that is a port for the yard (a base with a `yard_passage_days`) can be ordered to the yard (chapter 1, Task force orders). Its ships then leave the map and come back, whole, on a day worked out from their damage and told when the order is given. Two quantities are named and kept apart:

- **Days of repair** of one ship is the time she needs from her damage alone, with no passage: `yard_days_per_flotation_point` times her flotation damage, plus `yard_days_per_system_degraded` for each system degraded, plus `yard_days_per_system_out` for each system out, the three products added and the sum rounded up to a whole day (a part of a day is a day). A system her class lacks is working and counts for nothing. Fire damage adds nothing: the yard mends it with the rest. A ship with no damage has 0 days of repair.
- **Days away** of a force is the port's `yard_passage_days` plus the largest days of repair of its ships afloat. The passage is at least 1, so the days away are at least 1. No passage is steamed: the force is simply away.
- **Day of return** is the day the order is taken in plus the days away. A force sent on day D with 34 days away is away on days D to D + 33 and is back at the start of day D + 34.

One function works out days of repair, and the order, its acceptance, `measure` and the count of ships out of action (chapter 6, decision 0088) all read it. Where two bases of the side in the hex are ports for the yard, the lowest-numbered is the port. The days are worked from the state when the order is taken in, which is before that day's forward repair, so the days stated are the days the force leaves with.

When the order is taken in, the ships leave the force (their location becomes away at the yard, `ShipLocation::Yard`) and the force stays at its base's hex, idle, with no ship, route, cargo order, leg, alert or the event that alerted it. The force keeps its mission, speed and its order, readiness and its order, rules of engagement and their order, home port and fatigue while it is away. `ForceLeftForYard` is recorded as the order's follow-up, with each ship's days of repair, the passage, the days away and the day of return. The acceptance states the day: an answer to a queued order carries the day of return, or says the force does not return if that day is after the last day of the game.

**While away,** a force is on no hex of the map as far as any rule goes: it is not moved, fought, sighted, searched for, struck or alerted, takes no fatigue, is not refuelled or repaired forward, and takes no order (chapter 1, Every reason an order is refused). Its ships are afloat, with the damage they left with. Chapter 2 says what the enemy knows of it.

**When it returns,** at the start of its day of return, before refuelling, each ship of the force in number order is made whole:

| Field | Becomes |
| --- | --- |
| Flotation damage, fire damage, her bank | 0 |
| Each named system | Working |
| Fuel | Her class's `fuel_capacity_t` |
| Gun rounds | Her class's `main_guns` times `gun_rounds_per_gun`, the load a scenario gives a ship with no `gun_rounds` key |
| Torpedoes ready | Yes if her class has torpedo tubes |
| Side fired (`torpedoes_one_side_fired`) | No |
| Cargo aboard, experience | Unchanged |

The force is idle at its base's hex, with no route, cargo order, alert or fatigue. Its mission, speed, readiness, rules of engagement and home port are as they were when it left. Its arrival event is the `ForceReturnedFromYard` that is then recorded, citing the departure, so the day's refuelling and forward repair cite it. The force's orders are taken as for any force.

A force whose day of return is after the last day of the game never returns: its ships end the game afloat at the yard, with the damage they left with. The engine does not hold the last day; the session that plays the game does, and tells it in the briefing and in `measure`. A whole force goes and returns together, until forces can be formed by order. [0069]

### The figures

Five figures of `surface_combat.toml`, required like every key there, each a placeholder. The same values stand in the frozen step 1 copy, which no step 1 scenario can reach.

| Figure | Means | The loader refuses |
| --- | --- | --- |
| `forward_repair_max_flotation` | Forward repair mends flotation damage only below this many points | Below 0 or above 100 |
| `forward_repair_system_points` | The points a forward base must put into one degraded system to make it work | 0 or below |
| `yard_days_per_flotation_point` | Days of repair for each point of flotation damage | Below 0 |
| `yard_days_per_system_degraded` | Days of repair for each system degraded | Below 0 |
| `yard_days_per_system_out` | Days of repair for each system out | Below 0, or below `yard_days_per_system_degraded` |

A base's points a day and a port's passage are not figures of this file: they are the scenario's, on the base's entry (chapter 2, Bases on hexes). [0069]

### Events

| Event | When | Who sees it |
| --- | --- | --- |
| `ShipRepaired` | A ship is mended forward (above) | Own ship |
| `ForceLeftForYard` | A yard order is applied | Own force |
| `ForceReturnedFromYard` | A force returns | Own force |

The enemy is shown none of the three (chapter 1, What a side sees of an event). [0069]

Known limit: flooding that slows a ship, magazine explosions and damage to radar are not built. Shore bombardment and ships at anchor as targets are not built. Repair of a ship lying in port and of port damage is not built, nor is sending part of a force to the yard. Real hit chances, damage and repair figures come from sources in step 2. [0014, 0016, 0052, 0069]

## Random draws

| Draw | Stream keyed on | Order |
| --- | --- | --- |
| Sighting | The force and the enemy force | One a round until it sights |
| Fire | The firing ship and the enemy force | Per salvo: the shots, then for each hit the track (guns) and the system. Gun salvo before torpedo salvo |
| The second side's torpedoes (`Draw::TorpedoOtherSide`) | The firing ship and the enemy force | One stream for that one salvo: the shots, then for each hit its system. After the ship's gun salvo of the round [0083] |
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
