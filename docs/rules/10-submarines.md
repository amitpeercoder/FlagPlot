# Submarines

A submarine is a boat on a station. A task force that spends hours in the hex she keeps may be attacked by her, and the force fired on may hunt her. Her own side knows her exactly. The enemy knows of her only what a salvo, a sighting or a belief it was told gives it: a report of a submarine near a hex, and waters where it suspects the enemy's boats work.

This chapter holds the boat as a unit of her own kind, her passage, the hours a force spends in her hex, the chance she attacks, her salvo, the hunt, what aircraft do to her, what each side is told, suspected waters, the station order and the objective. The staff that gives boats their stations, the patrol order, and the staff's regard for the waters a side suspects are chapter 8's (Patrolling waters with submarines). Every quantity is a fixed-point number, and "cut" below means cut toward zero at six decimals (chapter 1, Numbers and chance). Figures named as `submarines.toml: key` are read from `data/reference/submarines.toml`; every figure in it is a placeholder, each with the note of what it rests on.

A scenario that names no submarine files has no boat and no suspected waters, and plays as it did before the rule: no step of this chapter runs, no event of it is recorded, no draw of it is made and the state hash is what it was. [0218, rules 31 and 36, Games on file]

## Where this sits in a day

| When | What runs | Where |
| --- | --- | --- |
| Step 0 of the day | A boat held apart until today comes onto the map | A unit not yet entered, below |
| Every pulse, last in the movement phase | Each boat's passage, then each boat's attack | Passage; The attack |
| Morning and afternoon, in the air search phase | Each group that flies looks for boats in its arc, after its look for forces | Aircraft and the boat |
| Every pulse, at its end, before contacts age | Each side's reports of submarines age and are dropped | Reports of submarines |
| The close of a day, after the end-of-day phases and the afternoon's state reports, before the invariants | Suspected waters whose day is the next are told | Suspected waters |
| Dusk, with the other orders | A station order is taken in | The station order |

The boat's steps run at the end of the movement phase, while the movement step's own times are in hand, and nothing of those times is kept. By night that is before a shore bombardment and before loading and unloading, so a force hit then goes on with what it has. [0218, rule 37]

A side sees: nothing of the order of the steps. It sees what each rule below gives it.

## The boat

A submarine is a unit of her own kind, as an air group and a land unit are. She is not a ship, and a boat is in no task force. She has:

- a number (below 1,000,000, which is for units made in play), a name, a side and a class;
- a hex (the hex she has entered) and a station (the hex she keeps);
- a passage, or none: the hex she is bound for, whether it is her home base, the hex she has set out for and not reached, the miles steamed of that leg and its charge, whether she has arrived, and the order that sent her;
- the salvoes she has left;
- the last pulse she is held down, inclusive, or none;
- a condition: sound, damaged, sunk or home;
- the base her scenario names as her home;
- a leave day, an entry day, a day from which her side may use her and the order that gave her station, each or none.

Her class, in `submarine_classes`, gives her salvo (the torpedoes in one), the salvoes she carries, her passage speed on the surface and her torpedo. Her torpedo, in `submarine_torpedoes`, gives the damage of one hit. `submarines.toml: hit_chance` gives the chance one torpedo of that kind hits. She is on the map unless she is sunk, home or held apart until a later day. [0218, rules 1, 28 and 31]

**No rule made for a task force touches her.** She meets no surface fight, is sighted by no search of a force, is no contact, is struck by no raid, fuels nowhere, tires not at all, burns no fuel, takes no part in a fight ashore and is no command. The only searches that sight her are those of Aircraft and the boat, below; the only forces that attack her are those that hunt her after her salvo. A patrol's length is the scenario's. [0218, rules 2 and 32, What it does not do]

**She is on her station** when she is on the map, is on no passage and is in the hex of her station. She is **on passage** while she goes to a station or home. A passage that has ended is cleared at the start of the next pulse's step, so that she is on station from the pulse after she arrives. [0218, rule 43]

**She may be held apart and enter on a day, and may have a day to be used from, as any unit.** She does not use the entries of chapter 1, A unit not yet entered: no entry is made for her, so no entry's hash moves. Her own field, `enters_day` (the scenario's key `enters`), holds her apart: a boat held apart is in no hex for any rule, is not looked for, attacked or counted, and her passage does not run. On her day she comes onto the map first thing, after the entries of the units, in boat number order, and `SubmarineEntered` records it (no pulse, no cause). A boat is on the map for a day when she is not sunk or home and her entry day is not after it. An order to a boat held apart is refused before every other reason, as an order to a unit held apart is (chapter 1, A unit not yet entered); the day she may be used from is tried as any unit's is (chapter 1, A day from which a unit may be used), and before it she attacks nothing but still steams on a passage she was set on. [0218, rules 31 and 33; the architect's ruling on stage 2]

**What is hashed.** The boats, the lists of reports of submarines and of suspected waters a belief holds, and the waters waiting to be told are hashed last, only where the game has any of them: a boat, waters waiting, or a belief or command picture holding a report of a submarine, suspected waters or a claim marked a submarine's. A game with none writes nothing there and its hash is what it was, so the state layout is not raised for it, as for the supply lines and the units held apart. [0218, rule 31, As built: stage 2]

**Invariants.** At the end of every day each boat is checked: her class is in the data; her salvoes are at most her class carries; a boat sunk or home has no passage; a boat not sunk or home has a hex ships can enter, and is at her station if she is on no passage; her station is a hex ships can enter; a pulse she is held down is not before day 1; and her number is her key and below 1,000,000. A break is `SubmarineUnsound`, a bug in the rules. [0218, rule 36]

A side sees: its own boats exactly, those held apart (with the day they come), sunk or home among them, in its view, its briefing's submarines part, `list_forces` and `get_entity` (`submarine:N` by the side's own numbers). From a seat at a task force or a base the boats are not shown: only the side's highest command, the area command, knows her. It is shown no enemy boat, on any chart, and never one before it is told of her by a report. [0218, rules 13, 16 and 35]

## Passage and station

A boat goes by passage from where she is to a hex: her station, when the scenario or an order gives her one that is not her hex, or her home base's hex, when she goes home. She attacks nothing on a passage. [0218, rules 3 and 43]

**Her steaming.** In each pulse's step, in boat number order, first an arrival of an earlier pulse is cleared, then, on her leave day, she is sent home (below), then she steams. Her budget is her class's `passage_speed_kn` times the pulse's hours (12 at night, 6 by day), in miles. She goes by the sea route a force takes, over the same field of sea distances, the same step and the same charge (chapter 2, The path and What a step is charged): at a centre she sets out on the next step toward her destination, which is charged 1 to 40 miles, and takes the leg; she enters the next hex at half the leg, and is at its centre at the end. She steams until the budget is spent or she is at the centre of her destination's hex. Her leg is 0 miles at a centre and the step's charge while she is on a leg. She burns no fuel, and no state of hers lowers her speed. [0218, rule 43; chapter 2]

**Arriving.** She has arrived when she is at the centre of the hex she is bound for. A boat bound for her station has `arrived` set, `SubmarineOnStation` is recorded (citing her steaming), and the passage is cleared at the start of the next pulse's step: she is on station from the pulse after she arrives and makes no attack in the pulse she arrives. A boat going home is taken off the map at her base's hex: her condition becomes home, her passage is cleared and `SubmarineReachedHome` is recorded. A boat that is off the map takes part in no rule. [0218, rules 21 and 43]

**Her events.** `SubmarineSteamed` records the hex she was in and the hex she is in, once for a pulse in which she steamed. It cites the order that sent her, or, on her way home, the `SubmarineLeftForHome` that sent her. [0218, rule 43]

**Going home.** A boat leaves for her home base's hex, by a passage like any other, when:

- **she is damaged** (by the hunt or from the air, below): `SubmarineLeftForHome`, her reason that she was damaged, citing what damaged her;
- **she has no salvo left,** in the pulse she fires her last: the reason that she had no salvo left, citing the salvo. A boat a scenario gives no salvo to at the start does not exist: the scenario is refused when it is read (Scenario keys and figures), so every boat on the map has a salvo until she fires her last;
- **her leave day comes:** a scenario may give her `leave_day`. In the night pulse of that day, or in her first pulse on the map after it, a boat not already going home leaves, the reason that her leave day came, citing nothing.

A leg she is on is finished first, as a force finishes its leg when its route changes. A boat already going home is not sent again, and one event is recorded for her leaving. A boat going home takes no order (below), and is sighted whole (Aircraft and the boat). A damaged boat stays damaged: she is not repaired. [0218, rules 3, 21, 43 and 52]

A side sees: its own boat's `SubmarineEntered`, `SubmarineSteamed`, `SubmarineOnStation`, `SubmarineLeftForHome` and `SubmarineReachedHome`, whole. The enemy sees none of them. [0218, rules 13 and 16]

Not built: fuel for a boat, her fatigue, her repair, a harbour, mines, or the carriage of aircraft, men or supply. [0218, What it does not do]

## The hours a force spends in her hex

The attack on a force reads the hours the force spent in the boat's hex in the pulse. They are worked in the movement step, kept nowhere and counted only where a boat is on the map: with none, nothing is counted and nothing else of the step changes. [0218, rules 37 and 38]

A force's hours in a hex are, by what the force did in the pulse:

| The force | Its hours in a hex |
| --- | --- |
| Did not move | The whole pulse, in the hex it is in |
| Crossed the hex | From when it entered the hex to when it left it |
| Stopped, or was left dead, in the hex | From when it entered to the end of the pulse |
| Was made in the pulse, by a detach, where it was made | From when it was made |

These are worked so:

1. The force's position changes at half a leg: a force is in a hex from half a leg before its centre to half a leg after it, one hex's steaming of 40 miles where the way is clear. [0218, rule 38; chapter 2, Legs and position]
2. Each segment of the force's steaming (a run at one speed between two changes) counts the miles it steamed in each hex, up to half the leg in the hex the force is in and after it in the hex ahead. The hours of a segment in a hex are those miles over the segment's speed: one division for each hex of each segment, cut, and never the difference of two times. A force that crosses a hex in one segment at 30 knots has 40 over 30, 1.333333 hours in it. The hours of the segments in one hex are added.
3. A force that did not steam the pulse out has the rest of the pulse in the hex it ended in: the pulse's hours less the hours it steamed (or less the hours before it was made). A force that steamed the pulse out has no remainder. A force stopped for want of fuel has the rest of the pulse where it stopped.
4. A force that enters a hex at the last moment of the pulse has no hours in it. A force with no ship afloat has none anywhere. A force that passes by a hex without entering it has none in it.

A force's hours in a hex that holds a base of its own side on the map do not count (below, The attack). [0218, rules 38 and 41]

A side sees: nothing. The hours are not recorded as an event; the chance cites them (below).

## The attack

In each pulse, after the passage of every boat, each boat in number order is tried in turn. A boat attacks only if all of these hold, and otherwise makes no draw:

- she is sound, on station and has at least one salvo left;
- her side may use her (a boat with an `available_from` later than the day makes no attack);
- she is not held down: her held-down pulse is before this pulse.

**Which forces.** For each task force of the other side that spent hours in her hex, in force number order, a chance is made if the force has a ship of an attackable kind: a carrier, a battleship, a transport, an auxiliary or a cruiser. A force of destroyers alone is not attacked and has no chance. A force whose hours are in a hex that holds a base on the map of the force's own side has none either: no harbour is in the game, and the rule is a judgement the war on shipping will rule on again. [0218, rules 4, 40 and 41]

**The chance for the pulse.** The hourly chance is `submarines.toml: submarine_attacks_per_hour` times `submarines.toml: attacks_ship_dead_in_water` if any ship of the force is dead in the water, else times 1, cut and no more than 1. A ship is **dead in the water** when its propulsion gives it no speed: the speed factor of the propulsion's state is 0 (chapter 4, Named systems), which is when it is out. With no surface combat figures no ship is dead in the water. The hours of the force in her hex are split into `whole` hours and a `part` of an hour. The miss for the pulse is worked so:

1. Start with 1.
2. For each whole hour multiply by one less the hourly chance, each product cut.
3. Multiply by one less the product of the hourly chance and the part, that product cut.

The chance for the pulse is one less the miss. At the figures of scenario 128, 0.02 an hour: 12 hours (a night stopped in her hex) give a miss of 0.784713 and a chance of 0.215287; 6 hours, 0.114159; a crossing at 30 knots, 1.333333 hours, a miss of 0.98 times 0.993334, 0.973467, and a chance of 0.026533; with a carrier dead in the water the hourly chance is 0.1, and 6 hours give a chance of 0.468559. No other conduct of the force moves it: not its speed, its zigzag, its flying, its screen or an escort. The hours carry the speed. [0218, rules 4, 5 and 39]

`SubmarineAttackChance` records, for each such force in number order, the boat, the force, the hex, the hours, the chance and the hourly chance, citing the boat's station order if she has one. Its breakdown, when recorded, lists `attacks_per_hour`, `attacks_ship_dead_in_water` (as used: 1 or the figure), `hours`, `whole_hours`, `part_hour` and `miss`. Then one draw is made for the pair, on its own stream (`Draw::SubmarineAttack`), under the chance. Every force that has a chance has its own draw, whether or not another force's draw succeeds. [0218, rules 4 and 39]

**Which force.** Of the forces whose draw succeeded she attacks one: the force whose best ship ranks first by the order of kinds, carrier, battleship, transport, auxiliary, cruiser (one order for both sides), then the lowest force number. A destroyer has no place in the order. If no draw succeeded she attacks nobody. [0218, rules 6 and 40]

**Which ship.** She aims at one ship of that force: the first by the same order of kinds, a ship dead in the water before another of its kind, then the lowest ship number. [0218, rules 7 and 40]

A side sees: neither side is told the chance or the draw: `SubmarineAttackChance` names the enemy force. [0218, rule 37]

### The salvo

She fires one salvo at the one ship, and her salvoes left fall by one before the torpedoes are drawn. The salvo is her class's `salvo` torpedoes, all at the aimed ship. [0218, rule 7]

The taken-for kind is drawn first, from her claim's stream (below): the kind her side takes the ship for, read on the confusion row of the ship's true kind in `search.toml: kind_confusion`, as a report's kind is (chapter 3, How each part is drawn). [0218, rule 45]

**Each torpedo** hits by one chance: `submarines.toml: hit_chance` for her class's torpedo, a figure of the weapon and not of the side or the target. The torpedoes' draws come from one stream of the boat and the ship aimed at (`Draw::SubmarineTorpedo`), the n-th number being the torpedo in the n-th place of the salvo. A dud is a miss and is in the figure. A torpedo that misses hits no other ship. [0218, rules 8, 30 and 42; What it does not do]

**A hit** is worked through chapter 4 as a ship's torpedo hit is: its damage points are 100 times the torpedo's damage (`submarine_torpedoes` `damage`) over the target class's `toughness`, in one product and one truncating division; every hit lands on the flotation track. Each hit makes one system draw, taken from the same stream after the torpedoes' draws, in hit order: the system chance is the points times `surface_combat.toml: system_hit_chance_per_point`, no more than 1, and the bands are those of `surface_combat.toml: torpedo_system_hit` (chapter 4, Where a hit lands). With no surface combat figures no system is hit. The hits are applied at once, as chapter 4, Applying damage says, each event citing the salvo: `ShipDamaged`, `SystemHit` and, if the ship goes down, `ShipSunk`. The ship's task force loses the sunk ship. [0218, rule 8]

`SubmarineSalvo` records the boat, the force, the ship, the hex, the torpedoes, the hits and the kind taken for, citing the chance's event. Its breakdown, when recorded, lists `hit_chance`, then one `torpedo_hit` for each torpedo, 1 or 0, and its result is the hits. [0218, rule 42]

Boats are tried in number order and each salvo's damage lands at once, so a later boat meets a force as an earlier one left it.

A side sees: her side sees `OwnSubmarineFired`, the cut of the salvo: that she fired, where, how many torpedoes, and the kind her people took the target for; not the force, the ship, the hits or the chance. The side fired on sees `SalvoFromSubmarine`: that its ship was fired on from a submarine, where; not the boat, the torpedoes or the hits. It is told of every salvo, hit or miss. The hits it took reach it as `ShipDamaged` and `SystemHit` and `ShipSunk`, whole, for its own ship, as any hit does, with no attacker named. [0218, rules 13, 14 and 46; the architect's ruling on stage 2]

### The hunt

A force with a warship, a carrier, a battleship, a cruiser or a destroyer, other than the ship aimed at, hunts the boat that fired on it, whether the salvo hit or missed. A force with no such ship hunts nothing, and the boat is free to attack in the next pulse. [0218, rules 9, 10 and 46]

A boat that is hunted is held down: she attacks nothing for the rest of the pulse and for the next `submarines.toml: held_down_pulses` pulses. The last pulse she is held down is that many pulses after the pulse of the hunt. One stream of the boat and the force (`Draw::SubmarineHunt`) gives two draws: the first, under `submarines.toml: hunt_damages_chance`, damages her; the second, drawn only if she is damaged, under `submarines.toml: hunt_sinks_share`, sinks her. [0218, rules 9, 22 and 53]

- **Sunk:** her condition is sunk, her passage is cleared and `SubmarineSunk` is recorded, citing the hunt. She is off the map at once. She is not sent home.
- **Damaged, afloat:** her condition is damaged and she leaves for home (Passage and station).
- **Untouched:** she is held down and stays on her station.

`SubmarineHunted` records the boat, the force, the hex, the last pulse she is held down, and whether she was damaged and whether she was sunk, citing the salvo. With a hunt or none, a boat with no salvo left that is not sunk goes home for it. [0218, rules 9, 21 and 22]

A side sees: her side sees `OwnSubmarineHunted`: where, until when she is held down, and whether she was damaged; not the force that hunted. A sinking is told by `SubmarineSunk`, whole to her side. The force's side sees `SubmarineHuntedBy`: that its force hunted a submarine and where; not the boat and not what came of it. It is told nothing of the result. [0218, rules 13 and 15]

## Aircraft and the boat

A group on the search mission that flies in a daylight pulse looks for boats in its arc after its look for forces (chapter 3, Air search). For each boat of the other side on the map, in boat number order, whose hex is in the group's arc and range, there is one chance and one draw (`Draw::SubmarineSighted`, a stream of the boat and the air group). A boat is looked for whatever she is doing and however long she is held down. No search by night looks for a boat. [0218, rules 18, 48 and 49]

**The chance.** The distance is the air distance from the group's origin to the boat's hex, taken as at least one hex (40 miles), so that a boat in the group's own hex is not sighted for certain. The chance is the chance of sighting one force at that distance (chapter 3, The chance to sight) times `submarines.toml: submarine_surfaced_share` for a boat on her station, and whole for a boat on passage, going home included. `submarine_surfaced_share` stands for both the time she is up and how much harder she is to see than a ship. `SubmarineSearchAttempt` records the group, the boat, the distance, the chance and whether it sighted, citing the flight; its breakdown lists the factors of the chance of a force and `surfaced_share`. [0218, rules 17, 18 and 48]

**Sighted,** she is reported to the group's side as one submarine at her hex through the report's error (below), `SubmarineReported` citing the attempt; she is held down for the rest of the pulse and the next (the later of that and what she was); and the aircraft attack her in the same pulse. One stream of the boat and the group (`Draw::SubmarineAirAttack`) gives two draws: the first, under `submarines.toml: air_attack_damages_chance`, damages her; the second, drawn only if she is damaged, under `submarines.toml: air_attack_sinks_share`, sinks her. `SubmarineAttackedFromAir` records the group, the boat, the hex, and whether she was damaged and sunk, citing the attempt. Sunk, she is off the map, as in the hunt; damaged, she leaves for home. No aircraft is lost, no bomb is used and no sortie is added: the attack is two draws and nothing else. Any group on the search mission may sight and attack, flying boats too. A boat that one group sank is off the map for the groups after it; one it damaged is on her way home, on passage, and may be sighted by a later group at the whole chance. [0218, rules 19, 20 and 50 to 52]

No strike is launched at a boat, no group's mission is changed for one, and no other rule of search touches her: she is never a force in the search's lists. [0218, rules 2 and 18]

A side sees: the group's side sees `AttackedSubmarineFromAir`, that its group attacked a submarine and where, and the report of the submarine. It is told nothing of the result. The boat's side sees `OwnSubmarineAttackedFromAir`, where and whether she was damaged; a sinking is told by `SubmarineSunk`. The search attempt is seen by neither side. [0218, rules 13, 15 and 19]

## What each side is told

**Her own side** is told of a salvo by its cut (above), and is given her claim. The claim is made as a raid's claim is made (chapter 3, The claim after a raid from the air): one claim for the salvo, straight after the hits have landed, marked `from_submarine`. It holds the hits really scored plus one false hit for each torpedo that missed, each at one draw under `submarines.toml: submarine_false_hit_chance`; the kind of ship her people took the target for; and the ships her hits sank. It holds no aircraft. The stream is the claim's own (`Draw::SubmarineClaim`, a stream of the boat and the ship aimed at): the kind first, one number read on the confusion row, then one draw for each torpedo that missed, in order. `submarine_false_hit_chance` stands where a raid's claim figure stands (`air_combat.toml: bomb_false_claim_chance`) and takes its value. A claim is made for every salvo, whether it hit or not, and is held first by her side's area command. `ClaimReceived` cites the salvo; its breakdown (`real_hits`, `torpedoes_missed`, `false_hit_chance`, `false_hits`) is the umpire's. The claim feeds her side's known progress on `ships_sunk` as any claim does (chapter 6). Her side is given no contact of the force and no report of ships. [0218, rules 13 and 45]

**The side fired on** is given a report of one submarine at her hex through the report's error: one stream of the boat and the ship aimed at (`Draw::SubmarineReportError`): a draw under `search.toml: position_error_chance` and, if it strikes, one of six directions, the report naming that neighbour if it is a hex ships can enter and the true hex if not. The count and the kind are not drawn: a report of a submarine holds only a hex, a day, a pulse and its age. It is held first by the command of the force fired on. `SubmarineReported` records it. [0218, rules 14 and 46]

### Reports of submarines

A side's belief holds reports of submarines in a list of its own, beside its contacts, claims and estimates of the enemy's men ashore (chapter 3, Belief). It is written only by `BeliefState::apply_report`, with the report kind `Submarine`, from a salvo or a sighting.

- **One for a hex, the newer.** Two reports at one hex are one: the newer (by the event that records its arrival) replaces the other. A report older than the one held is heard and changes nothing. [0218, rule 44]
- **Nothing there removes none.** A report that a hex is empty (chapter 3, Belief) removes a contact and never a report of a submarine.
- **Aged and dropped by its own figure.** At the end of every pulse, night included, before the contacts age, each report not made or replaced in the pulse gains one pulse of age, and one that reaches `submarines.toml: submarine_report_drop_age_pulses` is dropped, in hex order. This runs wherever the scenario has submarine data, boat or none. `SubmarineReportDropped` records each, citing the report that made it. [0218, rule 44]
- **In every command's picture.** With command levels on a report of a submarine is passed between commands as a sighting report is (chapter 1, Commands and their pictures), a late one arriving at the age it has, and each command's picture ages them as the side's belief does.
- **Read by nothing that reads contacts.** A report of a submarine is not a contact: it does not merge with a contact of ships, alerts no force, is no threat in the staff's reading, is no target of a strike or a sortie, and is not built in a dry run (chapter 9). Nothing in this stage reads it but the side's own briefing, tools and chart. [0218, rules 14 and 44]

A side sees: its own `SubmarineReported` and `SubmarineReportDropped` whole, and its reports in its briefing's submarines part, in the order of their hexes, each with its place, the day it was made and its age. The enemy sees nothing of them. [0218, rules 14, 44 and 46]

## Suspected waters

A scenario may give a side the waters where it believes enemy submarines work (`[[suspected_waters]]`): a hex and a distance round it, the words of the belief, the rows it rests on and the architect's reading of those rows into this hex and distance, marked as hers. It is what the side's command believed at the start, right or wrong, and is not made from where the boats are. [0218, rules 23 and 47]

**It is a belief told on a day, as the opening estimates are** (chapter 3, The ground report and the estimate; decision 0160). An entry of day 1 is held at setup, on day 0, before the first day. An entry of day D after it waits in the state, in the order the scenario gives, and is told at the close of day D - 1, after the day's end phases and the afternoon's state reports, so that day D's briefing and view hold it and day D - 1's do not. An entry whose day has passed and which was not yet told is told at the next close. The entry is taken in by `BeliefState::apply_report` (the report kind `SuspectedWaters`), held by every command at once with no travel, and `SuspectedWatersTold` is recorded in no pulse and citing nothing, on the day of the setup or of the close. It is hashed, and is in every picture. [0218, rules 23 and 47]

**In play the waters never change:** they are not aged, replaced, added to by a sighting or a salvo, or dropped. A side with no such entry is shown nothing until a boat is reported. A refused setup: an entry off the map, an entry of day 0, one given after the first day. [0218, rules 25 and 26]

A side sees: its own waters in its briefing's submarines part, in the order told, each with its place, its distance in hexes, its words and the day it was told, and on its chart as waters marked faintly with no boat drawn. The enemy sees nothing of them. [0218, rules 24 and 47]

## The station order

A boat takes one order, `station` (in a scenario, `[[order]]` with `kind = "station"`, `submarine = N` and `hex = [q, r]`): go to the hex and keep it. It is a standing order: the hex becomes her station. It is given at dusk with the other orders; it is not one of the seven kinds that may be given inside a day, so it is kept for the next dusk (chapter 1, Orders inside a day). [0218, rules 3 and 43]

An order is refused, in this order, with the reason of chapter 1, Every reason an order is refused: the boat is held apart until a later day (**not yet entered**), the day she may be used from has not come (**not yet available**), the boat does not exist or is not the issuer's (**not your unit**, one reason for both), she is sunk or home (**the submarine is off the map**), she is damaged or going home (**the submarine is going home**), the hex is off the map or a hex ships cannot enter or no sea route joins her hex to it (the reasons of a route's waypoint: **waypoint off the map**, **waypoint unreachable**). A staff's order for a boat is refused as **held** if the boat is held, and, after that, as outside the forces limit of the intent it serves unless the intent is a patrol in force that may use her: the staff's order for a boat is inside the forces limit of a patrol that may use her and of no other intent (chapter 8, Patrolling waters with submarines). A hold may name a boat (`hold` and `release` with `submarine`; a boat that is not the side's, does not exist or is sunk is refused as "Not your submarine"), and a commander's order to a boat that a patrol in force may use, or that any intent in force names in its forces limit, holds her from the staff from that morning, with a `HeldByOrder` event, as a direct order holds a force (chapter 8, Holds and the override). [0218, rules 3, 12, 34 and 43; the architect's ruling on stage 2]

An accepted order sets her station and the order that gave it, and `SubmarineOrderedToStation` records it, citing the order. She sets out by the passage a force would take, finishing a leg she is on first. A boat already at the hex and on no leg has no passage. The order of her passage is the order's, so her steaming cites it, and her attacks cite it as the order behind her station. [0218, rule 43]

A side sees: its own order's acceptance or refusal and `SubmarineOrderedToStation`, whole. The enemy sees nothing. [0218, rules 13 and 16]

## The objective

`submarines_sunk` is a kind of objective (chapter 6, The ten kinds): the enemy's boats whose condition is sunk, a number of points for each, whoever sank them. A boat sent home, damaged, for want of a salvo or on her leave day scores nothing. Scenario keys: `kind = "submarines_sunk"` and `points`. The side that hunted is told nothing of the result, so its known progress is not observable. The points are a placeholder, like every point value. [0218, rule 32]

## Draws

Every draw of the boat is in the subsystem `Submarine` (code 10) and carries a `Draw` value of its own, at the end of the list (chapter 1, Every draw is keyed). Each stream is of an entity and, where it says, a second:

| `Draw` | Value | Entity and second entity | What the stream gives, in order |
| --- | --- | --- | --- |
| `SubmarineAttack` | 22 | The boat and the task force | The attack: one number under the chance for the pulse |
| `SubmarineTorpedo` | 23 | The boat and the ship aimed at | One number for each torpedo of the salvo in its place, then one for each hit, the system |
| `SubmarineHunt` | 24 | The boat and the task force | The damage, then, if damaged, the sinking |
| `SubmarineSighted` | 25 | The boat and the air group | The sighting |
| `SubmarineAirAttack` | 26 | The boat and the air group | The damage, then, if damaged, the sinking |
| `SubmarineReportError` | 27 | The boat and the ship aimed at (a salvo), or the air group (a sighting) | The position error, then, if struck, one of six directions |
| `SubmarineClaim` | 28 | The boat and the ship aimed at | The kind taken for, then one false hit for each torpedo that missed |

All are keyed on the day and the pulse. Suspected waters draw nothing. [0218, Draws and records]

## Events

| Event | Cites | Seen by |
| --- | --- | --- |
| `SubmarineEntered` | Nothing | Her side, whole |
| `SubmarineOrderedToStation` | The order | Her side, whole |
| `SubmarineSteamed`, `SubmarineOnStation`, `SubmarineLeftForHome`, `SubmarineReachedHome` | Her order, her steaming, or what sent her | Her side, whole |
| `SubmarineAttackChance` | Her station order | Neither side |
| `SubmarineSalvo` | The chance | Her side cut (`OwnSubmarineFired`); the side fired on cut (`SalvoFromSubmarine`) |
| `SubmarineHunted` | The salvo | Her side cut (`OwnSubmarineHunted`); the force's side cut (`SubmarineHuntedBy`) |
| `SubmarineSearchAttempt` | The flight | Neither side |
| `SubmarineAttackedFromAir` | The attempt | Her side cut (`OwnSubmarineAttackedFromAir`); the group's side cut (`AttackedSubmarineFromAir`) |
| `SubmarineSunk` | The hunt or the air attack | Her side, whole |
| `SubmarineReported`, `SubmarineReportDropped` | The salvo or the attempt; the report | The side that holds the report, whole |
| `SuspectedWatersTold` | Nothing | The side told, whole |
| `ClaimReceived` (marked `from_submarine`) | The salvo | Her side, whole, without its breakdown |

Each is a kind of event placed in the table of chapter 1, What a side sees of an event. [0218, What is recorded]

## Scenario keys and figures

A scenario names all three of `submarine_classes`, `submarine_torpedoes` and `submarines`, or none. The first two are reference files, read by file name from `--reference` where one is given; `submarines` is read where the scenario names it and is not redirected, since the frozen step 1 figures give no boat. A scenario with a boat needs the search figures. The three files fit together: each class's torpedo is in the torpedo file with a hit chance in the figures. [0218, rules 28 and 30]

**`[[submarine]]`.** `id`, `name`, `side`, `class`, `q`, `r` (the hex she starts in) and `home` (the base she goes home to) are required. Optional: `station_q` and `station_r` (both or neither; without them her hex, and a station that is not her hex means she starts on passage to it); `leave_day` and `available_from` (from 1); `enters` (from 2 to the scenario's `days`); `salvoes` (from 1 to her class's, else her class's: a boat that is given 0 is refused when the scenario is read, with her number in the reason, since a boat with no salvo is sent home and the rule would never send her); `condition` (`sound`, else `damaged`, which starts her going home); and `held_down_pulses` (pulses she starts held down, counted from the night of day 1 and that pulse included: held down through day 1's N-th pulse, so 1 holds her through the first night and 4 through the second; 0 for none). Her hex and station must be hexes ships can enter, her home base must exist, its hex must be a hex ships can enter and a sea route must reach it from her station, and her class must be in the class file. A home base whose hex ships cannot enter, or that no sea route reaches from her station, is refused when the world is set up, which is before day 1 of every run, with her number and the base in the reason; the check is the world's and not the scenario file's. A boat given twice is refused. [0218, rules 1, 3, 33 and 43]

**`[[suspected_waters]]`.** `side`, `q`, `r`, `distance` (hexes round the centre), `words`, `rows` and `reading` are required, the last three not empty; `day` is the day the side is told, from 1 to the scenario's `days`, 1 by default. [0218, rules 23 and 47]

**An objective** of a side's `[objectives]` table: `kind = "submarines_sunk"` with `points`. **`[[order]]`.** `kind = "station"`, `submarine` and `hex`. [0218, rules 32 and 43]

**Figures** (all in `data/reference/submarines.toml`, each with a note of what it rests on; chances are 0 to 1, the report drop age at least 1):

| Key | What it is |
| --- | --- |
| `submarine_attacks_per_hour` | The chance of an attack in each hour a force is in the boat's hex |
| `attacks_ship_dead_in_water` | The multiple of it where a ship of the force is dead in the water |
| `held_down_pulses` | The pulses a boat is held down after a hunt, after the pulse of the hunt |
| `hunt_damages_chance`, `hunt_sinks_share` | The hunt's damage, and its sinking of what it damaged |
| `submarine_surfaced_share` | How much of a boat on her station a search finds |
| `air_attack_damages_chance`, `air_attack_sinks_share` | The air attack's damage, and its sinking of what it damaged |
| `submarine_false_hit_chance` | The chance a torpedo that missed is claimed a hit |
| `submarine_report_drop_age_pulses` | The age at which a report of a submarine is dropped |
| `[hit_chance]` | One for each torpedo, by its number: the chance one torpedo hits |

The class file's columns are `id`, `name`, `salvo`, `salvoes_carried`, `passage_speed_kn`, `torpedo_id` and `estimated` (read, and kept by nothing); the torpedo file's are `id`, `name` and `damage`. A class with a salvo or salvoes carried of 0 or a passage speed not above 0 is refused. The files are generated from the fact rows by `oob-derive` (`data/derivation/submarines_1942.toml`) and never edited by hand. [0218, stage 1, rules 28 to 30]

## What was cut

- **A boat that reports what she sees, or moves on a report.** A boat does not scout, move on a report of the enemy, patrol a line the staff keeps or attack together with others. The second stage. [0218, What it does not do]
- **The war on shipping.** A boat attacks no merchant ship on a supply line and no tonnage is counted. [0218, What it does not do]
- **A boat's approach, depth, the water, sonar conditions or captain,** a zigzag or a patrol over a force, or a date on a torpedo's figure: the chances carry them. [0218, What it does not do]
- **A stray torpedo and a hunt that does not hold the boat down.** A torpedo that misses hits no other ship, and a hunted boat is always held down. [0218, What it does not do, added]

## Not in this chapter

- How a ship sinks, how its systems are hit and how its damage is applied: chapter 4.
- Search, the chance to sight, the report's error, belief and claims: chapter 3.
- Movement, the sea route, legs and position: chapter 2.
- Orders, how they are checked and refused, entries and what a side sees of an event: chapter 1.
- Objectives, points and results: chapter 6.
- The pictures of a trial: chapter 9. A dry run builds the side's own boats and no enemy boat.
