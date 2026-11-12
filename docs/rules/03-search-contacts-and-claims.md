# Search, contacts and claims

How a side comes to believe something of the enemy. A side never reads the enemy's truth. It is told things, and every telling is a report. A report may be wrong. Reports make and refresh contacts, and contacts are all a side holds of the enemy's forces. A claim is a different report: it says what a side's own people believe they did to the enemy.

Figures named here are in `data/reference/` and are placeholders. Quantities are fixed-point and every chance is rolled one draw at a time, as chapter 1, Numbers and chance, says. Where this chapter says "cut", it means cut toward zero at six decimals.

## Where this sits in a day

Chapter 1, The order in which a day resolves, holds the whole sequence. Before every pulse run alert from belief, then fatigue of forces, then the grounded-bases check. Then the pulse's phases run. The phases that touch belief, in order:

| Pulse | Phase | What it does to belief |
| --- | --- | --- |
| Morning, afternoon | Air search | Air groups search and make sighting reports |
| Morning, afternoon | Strike decisions | Read contacts; write nothing |
| Morning, afternoon | Air combat | Strikes make sighting reports, "nothing there" reports and claims |
| All three | Movement | Forces meet and fight in time order. A fight makes sighting reports and claims |
| All three | End of the pulse | Every side's contacts age or are dropped |

The night pulse has an empty search phase. The end-of-day intelligence step is empty: ageing and merging are done as above. [0001, 0012, 0022]

A side sees: it sees its own reports, contacts and claims and what becomes of them. The enemy sees none of it. [0047]

## Air search

### Who searches, and with what orders

An air group searches in a daylight pulse if all of these hold:

- its mission is search, and it holds a search arc;
- it has a position: it is at a base, or aboard a ship that is afloat (a group aboard a sunk ship is lost with it and has no position);
- it is not dark at its position (for now, dark means the night pulse, everywhere);
- the number of aircraft it can fly is above 0.

A group with a mission of search and no arc, or an arc and another mission, flies nothing and records nothing. [0012, 0025]

An order sets the mission and an order sets the arc, each on its own and each standing until replaced. A search arc order names three things:

- A from bearing and a to bearing, each a multiple of 10 from 0 to 350. An order with any other bearing is rejected.
- A range in nautical miles, above 0 and no more than the aircraft type's search range (`aircraft_types.csv: search_range_nm`). An order outside that is rejected.

[0012, 0008]

A side sees: its own orders and their acceptance or rejection. The enemy sees neither. [0047]

### The aircraft that fly

The aircraft a group sends is the number it can fly this pulse: its ready aircraft, limited by its pilots where they are tracked, by the flight deck of the ship it is aboard, by its fatigue, and by supply at its base. A stood-down group flies none. Chapter 5, What a group can fly in a pulse, holds that rule, and this chapter reads its result. A group that can fly none records nothing. A group aboard a ship flies from the ship's task force position at the start of the pulse. A group at a base flies from the base's hex. [0025, 0026, 0036, 0038, 0039]

A side sees: its own groups and their aircraft. The enemy sees nothing. [0047]

### The arc

An arc runs clockwise from the from bearing to the to bearing. If the two bearings are equal the arc is the whole circle, 360 degrees. Otherwise its width in degrees is (to minus from) mod 360. [0012]

Bearings run clockwise from north, in whole multiples of ten. North is the direction of decreasing r. Hexes are pointy-topped, so the six neighbours of a hex lie on bearings 030, 090, 150, 210, 270 and 330. The offset of a hex from the origin, in hex-centre spacings, is x = dq + dr/2 (east) and y = minus 0.866025 times dr (north). The direction of a bearing comes from a table of sines and cosines at ten-degree steps in `bearing.rs`, exact on the axes and on the six neighbour bearings. All the arithmetic is on integers. [0012]

A hex is inside the arc if it is the origin hex, or the arc is the whole circle, or its offset lies between the two bearing lines. The edges are inside. An arc wider than 180 degrees is handled. [0012]

A side sees: nothing separate. [0047]

### Range and distance

A target is looked for only if its hex is inside the arc and its air distance from the origin hex is no more than the arc's range. A distance equal to the range is inside. Air distance is the straight line between hex centres in whole miles (chapter 5, Air distance). [0012, 0040]

A side sees: nothing separate. [0047]

### Who can be sighted

A target is a task force of the other side with at least one ship, at its position at the start of the pulse. Search runs before movement, so it sees each force where the last pulse left it. A group never searches for its own side. The group sees no individual ships: the targets are task forces. [0012, 0025]

A side sees: nothing about a target it did not sight. A search that found nothing is not shown to anyone. [0047]

### The chance to sight

For one group and one target in range and arc:

1. `numerator` = (57.295779 times the aircraft flown) times the type's sweep width in nautical miles (`aircraft_types.csv: sweep_width_nm`). The first product, a fixed-point number times a whole number, is exact. The second, that result times the sweep width, is a fixed-point product, cut to six decimals. So a sweep width with decimals is multiplied in once, after the aircraft.
2. `denominator` = the air distance in miles times the arc width in degrees. It is exact.
3. `chance` = `numerator` divided by `denominator`, cut to six decimals, and no more than 1.

57.295779 is degrees per radian. If the distance is 0 (the target is in the origin hex) the chance is 1. The chance is the swept miles of sea over the miles of arc at the target's distance. More aircraft, a narrower arc and a nearer target each raise it. [0012, 0040]

The event's breakdown, when recorded, lists degrees per radian, aircraft, sweep width, arc width and distance. [0012]

A side sees: the chance only in a search that sighted, and only on the searching side. [0047]

### What is rolled, and the order

Air groups are taken in group number order. For each, the targets in range are taken in task force number order. For each target one stream is opened, in the search subsystem, keyed on the day, the pulse, the air group, and the task force as the second entity. This is the pair key of decision 0013: no other target and no other group changes this draw. The first draw from the stream is the sighting. If it sights, the rest of the report is drawn from the same stream, as set out below. [0012, 0013, 0055]

Events, in order: `SearchFlown` once for each group that flies (it records the group, its origin, the arc, the range and the aircraft, and cites the mission order and the arc order); then, for each target in range, `SearchAttempt` (it records the distance, the chance and whether it sighted, and cites `SearchFlown`); then, if sighted, `ReportReceived` (cites `SearchAttempt`). The flight is also noted as a sortie of twice the arc's range for the wear rule, which chapter 5, What counts as a sortie, holds. [0012, 0035]

Because each report goes into belief at once, a later group's report in the same pulse can merge into a contact an earlier group made. Each group looks at its own targets only through its own draws, so this does not change anyone's chance. [0012]

A side sees: its own `SearchFlown` whole. A `SearchAttempt` that sighted, whole, to the searching side only. A `SearchAttempt` that did not sight, to nobody. The sighted force is not told, and the enemy sees no part of the search. [0047]

Not built: a group sending only part of itself on search while the rest does something else; night search; radar, weather and coastwatchers; report delay; reports that never arrive. [0012, 0052]

## Sighting reports

### What a report holds

The kinds of ship, from most to least important, are carrier, battleship, cruiser, destroyer, transport, auxiliary. This is the importance of a kind. Every rule that takes "the most important" kind of a set, or walks the kinds in order, uses it (the kind a report is made of, the kind a strike picks, the ship a raid attacks, the escorts a straggler takes from the least important up).

A sighting report holds an estimated hex, an estimated ship count and an estimated kind. It carries the event that records its arrival. It holds no identity of the force seen, so a side cannot tell two sightings are of one force. It holds no confidence (decision 0032 withdrew it). Air search, a surface fight and a strike that attacked all build their reports by the same code, so a report means the same whoever made it. [0012, 0014, 0026, 0032]

### How each part is drawn

Each part can be wrong, and each is drawn independently, in this fixed order from the report's stream:

1. **Position.** One draw against `search.toml: position_error_chance`. If it strikes, a second draw chooses one of the six neighbours evenly, by an index into the neighbour order (+1,0), (+1,-1), (0,-1), (-1,0), (-1,+1), (0,+1). If that neighbour is on the map and is deep sea, shallow sea or coast, the report names it. Otherwise (land, or off the map) it names the true hex. A closed hex side between the two hexes is not read. If the first draw does not strike, the second is not drawn.
2. **Count.** One draw `v` below 1,000,000. If `v` is below `count_under_chance` (as millionths) the report says one ship fewer, but never fewer than 1. If it is below the under and over chances together (`count_over_chance`) it says one more. Otherwise it says the true count.
3. **Kind.** One draw `v` below 1,000,000. The true kind's row in `search.toml: [kind_confusion.<kind>]` is walked in order of importance (see What a report holds). A kind absent from the row counts 0. The report names the first kind whose running total of chances exceeds `v`. Each row adds to 1, so one is always named.

The "true" values: the hex is where the force was seen, the count is the number of ships in it, and the kind is the most important kind among them, by the order carrier first. [0012]

Order in each stream: for air search, the sighting draw, then these. For a strike, the hit draws, then these. For a surface fight, these only. [0012, 0026]

A side sees: its own reports whole, including the breakdown if recorded. The enemy sees none. [0047]

### Reports from a surface fight

When a fight ends, each of its two forces in turn, lower task force number first, sends a sighting report if it sighted the enemy in the fight, or was fired on in the fight's last round (it saw the gun flashes). The report is of the other force: at the fight's true hex, with the number of the enemy's ships still afloat (or, if none are afloat, the number it began the fight with), and the most important kind among those. It goes through the error model above, from a stream in the intelligence subsystem keyed on the day, the pulse, the reporting force and the enemy force. The `ReportReceived` event cites `EngagementEnded`. Each force's report is sent before its claim, and the first force's claim before the second force's report. A fight in which neither force sighted makes no report. [0014, 0022]

A force sights in a fight when it rolls a sighting in a round, or when it is fired on (from the round after). The sighting roll is in chapter 4, Sighting. Whether a force is alert, which that roll reads, is in this chapter, Readiness, alertness and fatigue of a force.

A side sees: its own report whole. It learns of the fight from the first round in which its force sights or is fired on. [0047]

### Reports from a strike

A strike that finds ships sends a report of the force it attacked. It is worked out when the attack is, from the strike's own stream (keyed in the air combat subsystem, on the day, the pulse, the striking group and the task force attacked), after the hit draws. The true hex is the hex the strike was sent to. The true count is the number of ships in the force attacked at the start of the air combat step: the report is drawn before the step's damage lands, so a ship sunk by the attack is still counted. The true kind is the most important kind in that force. After all the strikes' findings are recorded, each attack's damage lands and then its report goes in, in group number order. The `ReportReceived` event cites the `AirAttack` event. A strike on a base makes no sighting report. To change: differences.md row 7, the report is to read the force after the damage, in step 2. [0026, 0028]

**What the report does to the contact.** The report goes through the same rule as any sighting report (A sighting report makes or refreshes a contact). Its hex is the hex the strike was sent to, or a neighbour of it if the position error struck, which is the hex of the contact the strike was sent at or one next to it. So it normally refreshes that contact: the contact keeps its number, takes the report's hex, count and kind, and has its age set to 0 and marked, so the ageing at the end of the pulse skips it. If the nearest fresh contact within `merge_distance_hexes` of the reported hex is another, that one is refreshed instead. If none is, a new contact is made. A strike that found nothing works differently (A strike that finds nothing).

The consequence is that the contact is at age 0 when the next pulse's strike decision is made. A group that struck it in the morning, and that can still fly aircraft (chapter 5, What a group can fly in a pulse), can be launched at it again in the afternoon pulse, if the contact's hex is within the group's reach (the type's strike radius on normal reach). A strike made in the afternoon leaves the contact at age 1 after the night's ageing, so the group can strike it again the next morning only if `air_combat.toml: strike_contact_max_age_pulses` is at least 1. [0012, 0026, 0032]

A side sees: its own report whole. The enemy sees nothing of it. [0047]

## Belief

### What a side holds

Belief holds two things for a side: contacts and claims. Both come only from a report. The only way to write them is `BeliefState::apply_report`. The one other change is ageing, which takes no truth and no report. [0007, 0012, 0055]

A contact holds an ID, an estimated hex, an estimated ship count, an estimated kind, an age in pulses, and the event that recorded the report that last made or refreshed it. Contact numbers are allocated for the side in order, and a number is never used again, including after a contact is dropped. [0012, 0028, 0032]

A side sees: its own contacts whole, by its own numbers. The enemy's forces reach it only as contacts. [0007, 0047]

### A sighting report makes or refreshes a contact

When a sighting report arrives:

1. Take the side's contacts that are within `search.toml: merge_distance_hexes` of the reported hex and no older than `search.toml: fresh_contact_max_age_pulses`. The distance is a count of hexes (`Hex::distance`), not an air distance.
2. If there is one, the nearest is refreshed, and the lowest contact number wins a tie. Its hex, count and kind become the report's. Its age goes to 0. Its last report event becomes this report's event. No average is taken: the report replaces what the contact held.
3. If there is none, the report makes a new contact with the next number, at age 0.

The test reads only what the side believes. A refreshed or created contact is marked, so the ageing step skips it once. [0012, 0032, 0040]

A side sees: `ReportReceived` whole, saying the contact and whether it was created. [0047]

### Ageing and dropping

At the end of every pulse, night included, for each side in order, each contact that was not made or refreshed in that pulse gains one pulse of age. A contact whose age reaches `search.toml: contact_drop_age_pulses` is dropped. The marks are then cleared. Each aged or dropped contact is an event (`ContactAged` or `ContactDropped`, with the age) that cites the report that last refreshed it, and they come in contact number order.

Read as pulses: a contact reported in pulse P is skipped by the ageing at the end of P, so its age is 1 at the end of the next pulse, and it is dropped at the end of the pulse that brings its age to `contact_drop_age_pulses`. [0012, 0032]

There are three ages that matter, and each is its own test:

| Test | Limit | Read by |
| --- | --- | --- |
| Fresh | `search.toml: fresh_contact_max_age_pulses` | A new report merging into it; a force being alerted by it |
| Can be struck | `air_combat.toml: strike_contact_max_age_pulses` | The strike decision (chapter 5, Naval strike: the contact) |
| Forgotten | `search.toml: contact_drop_age_pulses` | Ageing |

[0032, 0026]

Not built: a contact's course and speed, and moving a contact along its estimated course as it ages. [0012]

A side sees: `ContactAged` and `ContactDropped` whole, its own. [0047]

### Starting contacts

A scenario may start a side with contacts held. Each entry names a side, a hex, a ship count (at least 1), a kind and an age in pulses. Each is entered as a sighting report with exactly the stated hex, count and kind (no error is drawn), and then aged by the number of pulses given, by the same ageing rule, so belief is still written only from reports. The entries are entered from the oldest down, and the contacts are aged once after each age step. Events are recorded on day 0, with no pulse. Each entry's `ReportReceived` has no cause, and its ageing events cite the report.

The scenario fails to load if an entry has no ships, if the game has already begun or contacts were already given, if an entry's report would refresh another contact rather than make a new one (two entries too near together), or if an entry would be dropped by ageing before day 1. [0021]

A side sees: those events as its own. [0047]

### A strike that finds nothing

If a raid's target hex holds no enemy task force at the start of the air combat step, each of its strike groups that was sent at a contact records `StrikeFoundNothing` (it cites the launch). Each then sends a report: nothing at this hex. The side drops the contact the strike was sent at only if it still holds that contact and the contact still names that hex. Then `ContactNotThere` records it, citing `StrikeFoundNothing`. If the contact is gone, or a later report moved it, nothing changes and no `ContactNotThere` is recorded. No random number is used. The dropped number is not used again, so sighting the force later makes a new contact. A strike group with no bomber left after the fights finds nothing and reports nothing, so its contact stays. [0026, 0027, 0028]

Not built: a strike searching the hexes around the one it was sent to, and a search that covers a hex and sees nothing dropping a contact there. [0028]

A side sees: `StrikeFoundNothing` and `ContactNotThere` as its own. The enemy sees neither. [0047]

## Readiness, alertness and fatigue of a force

This section owns the whole rule for who is alert. Chapter 2 and chapter 4 point here.

### Readiness by order

A task force is at normal or battle readiness. It is normal by default, and the order that sets it stands until another replaces it. A readiness order is accepted for a force of the issuing side and changes nothing else, not even whether the force is alerted. [0021]

A side sees: its own orders. [0047]

### What alert is

A force is alert if it is at battle readiness or if its alerted flag is set. A force at battle readiness is alert whatever its flag says. A force at normal readiness is alert only while the flag is set. [0021]

The flag is stored with the event that set it (`alerted_by`). It is set in three places:

- by a fresh contact nearby, at the start of a pulse (Alert from belief);
- by the force's own sighting of the enemy, or by being fired on, in a fight (Alerted inside a pulse);
- by an alarm, after a fight nearby (The alarm).

It is cleared in one place: by Alert from belief, at the start of a pulse, when no fresh contact is near. A force made by a split takes its parent's flag and the event behind it (chapter 1, Task force orders). [0021]

### Alert from belief

At the start of every pulse, before any phase, each task force with a ship afloat looks at its own side's contacts. A contact counts if its age is no more than `search.toml: fresh_contact_max_age_pulses` and its estimated hex is within `surface_combat.toml: alert_contact_distance_hexes` of the force's position, counted as hexes (`Hex::distance`). Among those the nearest is the one cited, the lowest number on a tie. The rule reads belief only, never the truth of an enemy force. It does nothing in a game with no surface combat figures. [0021, 0032]

The rule reads the flag, not the readiness:

| Flag now | A contact counts | Result |
| --- | --- | --- |
| Not set | Yes | The flag is set. `ForceAlertedByContact` records it and cites the report behind the contact |
| Set | No | The flag is cleared. `ForceStoodDown` records it and cites the event that set the flag |
| Set | Yes | Nothing changes and nothing is recorded |
| Not set | No | Nothing changes and nothing is recorded |

So the rule gives the same results whatever set the flag. A force alerted by a fight or an alarm stays alerted at the start of the next pulse only if its side then holds a fresh contact within the distance. The report a fight sends makes or refreshes a contact at the fight's hex (or at a neighbour of it, if the position error struck), at age 0, which the ageing at the end of the pulse skips once. So a force that sighted or was fired on in a fight stays alerted for as long as that contact is fresh and the force is within the distance of it, and stands down at the first pulse start where no contact qualifies. A force alerted only by an alarm has sent no report, and stands down at the next pulse's start unless a contact of its side qualifies. [0021, 0032]

A force at battle readiness is handled like any other: the flag is set and cleared, and the two events are recorded, though the force is alert either way. To change: differences.md row 2, a contact is to neither alert nor stand down a force at battle readiness, and the events are not to be recorded for it. [0021]

A side sees: both events whole, its own force's. [0047]

### Alerted inside a pulse

A force at normal readiness that sights the enemy in a fight, or is fired on, is alerted from the end of that round. `ForceBecameAlert` records it. It cites the force's successful sighting roll of that round, else the first salvo fired at it that round, else the beginning of the fight. A force at battle readiness, or one already alerted, changes nothing and records nothing. Belief does not alert a force in the middle of a pulse: within a pulse a force hears of a fight only by the alarm. [0014, 0021]

A side sees: `ForceBecameAlert` whole, its own force's. [0047]

### The alarm

When a fight in which at least one salvo was fired ends, every other task force in the fight's hex, of either side, with a ship afloat, that is not alert at that moment, rolls once against `surface_combat.toml: alarm_chance`, in task force number order. The two forces that fought do not roll. Each roll is recorded by `AlarmRoll` (the chance, whether it was alarmed), citing `EngagementEnded`. On success the force's flag is set for the rest of the pulse, and the `AlarmRoll` is the event behind it. A failed roll changes nothing else.

The roll comes from a stream keyed on the force alone, in the surface combat subsystem. The stream is opened the first time the force rolls in the movement step and kept across every fight it hears in the step, one draw for each fight, in the order the fights end. The roll succeeds if the draw is below the chance. [0021, 0022, 0055]

Not built: the alarm reaching neighbouring hexes. [0021]

A side sees: an `AlarmRoll` its force heard, whole. A roll that failed is shown to nobody. [0047]

### Fatigue of a force

If the scenario has `fatigue.toml`, each task force has a fatigue from 0 to 100, starting at 0 unless the scenario says otherwise. At the start of every pulse, after Alert from belief, in force number order, each force with a ship afloat that is alert gains `fatigue.toml: force_fatigue_per_pulse_alert`, to 100 at most. Each force that is not alert loses `force_rest_per_pulse`, to 0 at least. Night counts as any other pulse. `TaskForceFatigue` is recorded only when the number changes, with the direction, the amount and the new value. It cites the readiness order for a force at battle readiness, the event behind the flag for a force alerted by something else, and for a force at rest its readiness order if it has one. A force made by a split takes its parent's fatigue. No random number is used. Without the file there is no fatigue and the fatigue factor of a sighting is 1. [0037]

Fatigue cuts one thing: a force's chance to sight in a fight (chapter 4, Sighting). It does not touch gunnery, speed, the alarm roll or sighting by aircraft. The fatigue used is its value after the start-of-pulse change. The staff does not stand a force down: readiness is the player's order. [0037]

A side sees: `TaskForceFatigue` whole, its own force's. [0047]

Not built: a rest threshold for forces, and fatigue from steaming or fighting. [0037]

## Claims

### What a claim is

A claim is a report to the claimant's belief of what its own people believe they did. It holds, each as a count that is 0 when the claim is not about it: ships believed sunk, hits believed scored on ships, the kind of ship believed attacked (or none), aircraft believed shot down, and aircraft believed destroyed on the ground. It holds the hex, the day and the pulse, whether it is from the air, and the event that records it. It holds no identity of an enemy unit. [0014, 0056]

A claim never ages. A claim is never corrected: if a ship claimed sunk is sighted again, the contact shows it, and the claim stands. Setting claims against later intelligence is left to a later step. [0012, 0056]

A side sees: its own claims whole. It never sees the enemy's. [0047, 0056]

### The claim after a surface fight

When a fight ends, each force that fired at least one salvo in it (guns or torpedoes) sends one claim, straight after its own sighting report, lower task force number first. The claim is of ships sunk, and of nothing else. It counts:

- every enemy ship that sank in this fight, and
- every ship still in the enemy force with fire damage of at least `surface_combat.toml: claimed_sunk_fire`, whether it burns from this fight or from an earlier one.

The two sets do not overlap, because a sunk ship has left its force. No random number is used. A surface claim names no kind, has no hits and no aircraft, and is not marked from the air. The `ClaimReceived` event cites `EngagementEnded`. A force that did not fire makes no claim. A surface claim can be above the truth only through burning ships: a ship burning at the claim level is claimed sunk, and a ship that sank is counted exactly. [0014, 0056]

A side sees: its own `ClaimReceived` whole. The enemy sees none. [0047]

### The claim after a raid from the air

One claim is made for each side that fired in a raid, once the whole air combat step has resolved (fighters, anti-aircraft fire, the attack, damage and reports). A raid that found nothing makes none, and neither does a raid whose escorts did not fire and whose bombers were all shot down before they attacked. A raid is the strikes launched in one pulse from one place at one hex, with their escorts (see the glossary). Claims are made in the order of the raids' first strike groups, the raiding side's claim before the defending side's.

A side fired if it has a firing event of its own in the raid:

- for the raiders, their escorts' fight, an attack on ships, or an attack on a base;
- for the defenders, their patrol's fight, the guns of the ships, or the guns of the base.

The claim holds a count of each of these, and each count is the real number plus false ones:

| Count | Real part | A false one is drawn for each | Chance |
| --- | --- | --- | --- |
| Aircraft shot down | Aircraft its fighters and guns really shot down in the raid, counted as aircraft removed from the groups, after the pool limits | Whole shot of fighters that missed | `air_combat.toml: fighter_false_claim_chance` |
| | | Whole shot of ships' or a base's guns that missed | `air_combat.toml: anti_aircraft_false_claim_chance` |
| Hits on ships (raiding side only, raid on ships) | Bombs and torpedoes that really hit | Whole bomb or torpedo that missed | `air_combat.toml: bomb_false_claim_chance` |
| Aircraft destroyed on the ground (raiding side only, raid on a base) | Aircraft the bombs really destroyed, ready and damaged together | Whole bomb that did not destroy one | `air_combat.toml: bomb_false_claim_chance` |

A miss is counted in whole units, and each miss is one draw. For a firing event of fighters or guns, and for an attack on ships, the misses are the shots of the event, cut to a whole number, less its hits, never below zero. For a raid on a base they are the bombs dropped (the whole shots of the attack events) less the aircraft the bombs really destroyed, so a bomb that hit supply or nothing, or a hit on aircraft that found none left to destroy, counts as a miss. A torpedo counts as a bomb. There is no claim for supply destroyed. [0056]

Ships sunk, in a raid on ships, for the raiding side: the ships the raid's own attacks really sank (each ship once), plus the ships of the force attacked that are still afloat at the end of the step with fire damage of at least `claimed_sunk_fire`. The end of the step includes the fire from every raid of the pulse, so a burning ship is claimed by each raid on its force that pulse, and a ship sunk by another raid is not afloat and is not counted twice. [0056, 0014]

The kind of ship in the claim is the kind the contact was taken for when the strike was launched: the contact's estimated kind. A strike sent at a "carrier" claims hits on a carrier, whatever was there. If several groups of a raid were sent at contacts of different kinds, the claim takes the first group's. A claim from a raid on a base, and the defender's claim, name no kind. [0056]

The draws are from a stream in the intelligence subsystem with the claim draw tag, keyed on the day, the pulse, the claimant's first firing unit, and the raid's first strike group as the second entity. The first firing unit is the air group for fighters and attacks, the task force for ships' guns and the base for a base's guns. The draws come in this order: one for each missed whole shot in each fighter or gun firing event, in the order the events were recorded, at that event's chance; then, for the raiding side only, one for each missed whole bomb or torpedo on ships, or one for each whole bomb on a base that did not destroy an aircraft, at the bomb chance. The draws are made after the raid is resolved and are their own, so no fight draw moves and no battle result changes. [0056, 0055]

The `ClaimReceived` event cites every firing event of the claimant's own that the claim rests on, in the order recorded, and nothing else. Its breakdown, when recorded, lists the real and missed counts, the chances and the false counts. The breakdown is the umpire's: the side that made the claim is not shown it (chapter 1, Causes and breakdowns). [0060] Claims are marked from the air. A claim of nothing is made when a side fired and claims no aircraft and no hit. [0056]

A side sees: its own `ClaimReceived` whole, with the firing events it rests on and without its breakdown. The enemy sees none. [0047, 0056, 0060]

Not built: a claim of ships crippled, a claim of supply destroyed, and setting claims against later intelligence. The chances are the same for both sides and are placeholders for step 2. [0056]

### What reads claims

A side's progress toward its objectives reads its claims: its known count of enemy ships sunk, with their kinds where a claim has one, and of enemy aircraft destroyed, are sums of its claims, labelled as claimed. Ships crippled stay not observable. The umpire's result at the end reads the truth. Chapter 06 holds the rule. The briefing's "Claims yesterday" lists them. [0054, 0056]

A side sees: its own claim count, which can be above the truth. [0056]

## What was cut

- **Noticing.** A sighted force no longer rolls to notice the aircraft that sighted it. It returns with a rule for how a force reacts to being sighted. [0032]
- **Contact confidence.** A contact and a report no longer carry a confidence: a contact simply ages and is dropped at an age. It returns when a report can be more or less sure of what it saw. [0032]

## Not in this chapter

- How a strike chooses its contact, how a raid fights and what a hit does: chapter 5, The strike decision and The raid, step by step.
- The surface fight itself (meeting, rounds, the sighting chance, fire, damage, breaking off and slipping away): chapter 4.
- Objectives, points and results, and the umpire's end-of-game result: chapter 6.
- The air group's fatigue, pilots, supply, wear and the number of aircraft that can fly: chapter 5.
- Orders, how they are taken in and checked, and the issuer of each: chapter 1, Orders.
- The random number key and how a stream is drawn from it, and the state hash: chapter 1, Numbers and chance, and The state hash and saved games.
- Which events a side sees and how an event is cut: chapter 1, What a side sees of an event. The briefing and the queries are not rules.
- Movement, fuel and cargo: chapter 2.
