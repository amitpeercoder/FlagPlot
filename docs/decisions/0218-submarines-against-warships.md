# 0218: Submarines against warships

Status: a plan by the architect, written on Nov 22, 2026, on the designer's question "Is it time to plan submarines?" and his word to go ahead, and on research gathered that day (`docs/research/submarines-in-1942.md`, rows 16000 and up). It is the first of three stages. It is to be read by a reviewer who has not seen it written before anything is built, and its questions put to the designer.

Summary: The game has no submarines. In the battles on file they torpedoed Saratoga twice, sank Wasp, Yorktown and Kako, and hit North Carolina and O'Brien. This puts a submarine on the map as a boat on a station. A force that comes into her waters may be met; a boat that meets one fires at a ship of the kind she wants, the likelier to hit the slower and steadier the force and the thinner its screen; the escorts then hunt her, hold her down and may damage or sink her. A side knows its own boats and knows an enemy boat only by her torpedoes or a sighting. Scouting by submarines and patrol lines kept by the staff are the second stage, and the war on shipping the third.

Standing: Planned on Nov 22, 2026; to be reviewed.

## Why

What the rows hold (the research report's sections 1 to 7):

- **The attacks.** Six Japanese attacks on American warships from January to September 1942 hit in four (Saratoga twice, Yorktown with Hammann, Wasp with North Carolina and O'Brien) and missed in two (Task Force 16 on 22 August, Hornet on 6 September). American boats sank Okinoshima and Kako and a transport, and most of their attacks failed or were never made: at Midway two boats of nineteen attacked.
- **Every hit was on a ship slow, steady or stopped.** Yorktown dead in the water; Saratoga at 13 knots on a steady leg after a launch; Wasp at 13 to 16 knots turning after flight operations with an arc of her screen open; Kako in column at 16 knots with no destroyer. ONI, Cominch and COMSOPAC said at the time that keeping one area for days at low speed was what exposed a force (events 16392, 16393, 16305; outcomes 16012, 16016).
- **The boats were on stations set beforehand.** Nineteen American boats on arcs round Midway; fifteen Japanese boats meant for two lines that the American carriers had already crossed; six to sixteen Japanese boats on lines east and south of the Solomons from 20 August to 20 September.
- **An attacking boat was hunted and seldom sunk.** Of the boats that fired at a warship in these waters in 1942 none was sunk in the attack: one was damaged, one crippled, the American boats held down for hours. Two Japanese boats were sunk by escorts on other work, and aircraft caught five on the surface and sank none.
- **The navies differed by doctrine and date:** Japanese boats kept for the enemy's capital ships; American fleet boats few in the South Pacific; the American Mark 14 torpedo running about ten feet deep until August 1942 and its exploder faulty until 1943, the older Mark 10 of the S-boats allowed for from January 1942; Japanese depth charges set shallow and hunts short.
- **What they do not hold:** a rate of meeting for a boat on station, a rate of hits for a torpedo, or what a destroyer more or less did. The rows give boats, days and encounters to count, and directions with no sizes.

What the game has that this can stand on: ships with classes and kinds; a task force at a hex with orders; torpedo hits worked through the damage rules of chapter 4; search, contacts and claims (chapter 3); a force's speed, readiness and what its groups fly; figures by side (decision 0213).

## The rules

### A boat

1. **A submarine is a ship of the kind submarine,** of a class with her surface and submerged speed, her tubes and her torpedoes, derived from the fact rows as any class is. She is alone in a task force of her own with the mission **submarine patrol**. No other ship joins it, and it joins no other force.
2. **She keeps a station:** a hex, with the kinds of ship she is to attack in their order. A scenario gives her one; her side may give her another by an order, to which she goes at her surface cruising speed by night and stays.
3. **She is at her station submerged by day and on the surface by night.** By day a search does not sight her. By night, and in any pulse in which she is on passage, a search may sight her as it sights a ship, at a chance cut by `surfaced_submarine_sighting_share`.
4. **She burns fuel and carries torpedoes,** and goes home when either is at the level her side sets, as a force does for fuel today. A salvo spends its torpedoes.

### Meeting a force

5. **In each pulse, for each boat on her station and each enemy task force in her hex at any time in the pulse,** one draw, under the chance of meeting: `submarine_meets_chance`, times a factor for each of these that is so:
   - **the force's speed:** at or below `slow_force_kn` the factor `met_when_slow`; stopped, or with a ship dead in the water, `met_when_stopped`;
   - **flying:** a ship of the force launched or landed aircraft in the pulse, `met_when_flying`;
   - **keeping one area:** the force has been within `kept_area_hexes` of this hex for `kept_area_days` days or more, `met_when_kept`.
   A force passing through at speed and not flying has the plain chance alone.
6. **A boat held down** (rule 11) meets nothing.
7. **A boat meets at most one force in a pulse:** the first in the order of the kinds she is to attack, by the best ship of each force, then the lowest force number.

### The attack

8. **A boat that meets a force fires one salvo at one ship:** the ship of the kind first in her order, and of several the one her side's attackers would choose (decision 0211's order, a ship that looks lost last). She fires `submarine_salvo` torpedoes or what she has left.
9. **Each torpedo hits by one chance:** her side's `submarine_torpedo_hit_chance`, times the same factors of rule 5 that made the meeting likelier, times a factor for the screen: `screened_share` for each destroyer or other escort with the force beyond the first `screen_free` of them, to no less than `screened_least`. A hit is a torpedo hit of her weapon, worked through chapter 4 as a ship's torpedo is. A torpedo that misses her ship may hit another ship of the force, by `stray_torpedo_chance`, drawn once for each miss, on the next ship of the force by number.
10. **A dud is a miss.** A side whose torpedoes failed has that in its chance to hit, by side (decision 0213), and nothing else models it.

### The hunt

11. **A force that has escorts hunts a boat that fired on it,** and one that met it and did not fire for want of torpedoes. One draw: under her enemy's `hunt_finds_chance` she is found. Found, she is held down for `held_down_pulses` pulses, and a second draw under `hunt_damages_chance` damages her (flotation damage, by `hunt_damage`), and at 100 she is sunk. Not found, she is held down for the rest of the pulse.
12. **A force with no escort hunts nothing.**
13. **Aircraft that sight a boat on the surface** (rule 3) attack her as they attack a ship where a group has the mission and the reach, and whether or not they do she is held down for the next pulse.

### What a side knows

14. **A side knows its own boats exactly,** and of an attack what the boat saw: the kind of ship fired at, the torpedoes fired and the explosions heard, which is a claim and not the truth (chapter 3).
15. **A side knows an enemy boat only as it finds her:** a ship torpedoed with no attacker in sight is told "torpedoed by a submarine" and holds a contact of a submarine at the hex; a hunt that finds her and a search that sights her make or refresh the contact. Nothing else shows an enemy boat, on any chart.
16. **A hunt's claim** is the hunters' belief: a boat held down may be claimed sunk, as ONI and CINCPAC believed I-26 and I-9 sunk.

## Figures

All placeholders. Each is to be set from the rows before a game is played with it, by a written count, and none is moved to enter a band.

| Figure | Set how |
| --- | --- |
| `submarine_meets_chance` | counted: the encounters on file (attacks and reports of a force met) over the boat-days on station in the same waters and dates, for the Solomons from 20 August to 20 September and for Midway, the count written in the figure's note with the rows it used |
| `met_when_slow`, `met_when_stopped`, `met_when_flying`, `met_when_kept`, `slow_force_kn`, `kept_area_hexes`, `kept_area_days` | judgements on a direction the rows give with no size; the speeds and the days from the attacks' own rows (13 to 16 knots; an area kept three days and more) |
| `submarine_torpedo_hit_chance`, by side | counted: torpedoes fired and hits in the attacks on file, by navy, with the Mark 10 boats and the Mark 14 boats said apart in the note. By side because the navies' torpedoes differed by rows |
| `submarine_salvo` | from the attacks: four to six |
| `screened_share`, `screen_free`, `screened_least` | judgements: the rows give screens the boats got inside and Cominch's wish for ten to twelve destroyers |
| `stray_torpedo_chance` | one case: I-19's salvo at Wasp hit two other ships |
| `hunt_finds_chance`, `hunt_damages_chance`, `hunt_damage`, `held_down_pulses`, by side | counted from the boats that fired and what became of them; by side because Japanese hunts were shorter and shallower by rows |
| `surfaced_submarine_sighting_share` | a judgement; five boats caught on the surface by carrier aircraft in August |

## What is recorded

New events, each with its causes and who sees it: a force met (the boat's side); a salvo fired, with each torpedo's chance and draw (the boat's side whole; the enemy the hits alone); a hunt, with its draws (each side its own part); a boat held down (her side). A boat sunk is a ship sunk. New draws, each with its own value at the end: the meeting, the torpedoes, the stray, the hunt's two. A boat's station, her torpedoes left and the pulses she is held down are state, hashed and named.

## Rule card

Reads: each boat's station, torpedoes, held-down count and class; each enemy force in her hex in the pulse with its speed, its ships dead in the water, whether it flew, where it has been, its escorts; the figures. Writes: torpedo hits, through chapter 4; the boat's torpedoes, held-down count and damage; contacts and claims. Draws: five new values.

## Games on file

A scenario with no submarine plays as it does. The scenarios for play and the 39 days gain their boats in a later change of their own, each with the rows of where the boats were, and are measured before and after then.

## What it does not do

- **It does not have a boat report what she sees,** or a staff place boats on a line. Those are the second stage. A boat here is placed by the scenario or by a direct order.
- **It does not attack merchant shipping on a supply line** or count tonnage. That is the third stage, with the economy.
- **It does not let boats attack together.** No navy did in 1942.
- **It does not model a boat's approach,** her depth, the water, sonar conditions or her captain. The chances carry them.
- **It does not let a force zigzag by order.** A force's speed and whether it flies and lingers are what it is judged by.
- **It does not give dates to the torpedo figures.** Every battle on file is in 1942; the cure of the Mark 14 waits for the campaign, as decision 0213 has it.
- **It does not carry aircraft in a boat,** lay mines, or land men or supply from one.

## What it costs

- **A kind of ship, a mission, a step in each pulse, some fifteen figures, five draws, four events, and the table's cards and chart for a boat and for a submarine contact.**
- **Rulebook:** a new section of chapter 4 or a chapter of its own; chapters 1 (the order of a pulse, who sees what, the draws), 2 (a boat's passage and fuel) and 3 (sighting a boat, the contact and the claims); the glossary.
- **Derivation:** the submarine classes and their torpedoes from the rows gathered, by written rules, with what is a gap left a gap.
- **A scenario of its own and its hand calculation,** written by the architect before any code is delegated.

## Questions for the designer

1. **Three stages, this one first:** boats on stations against warships; then boats that report and patrol lines the staff keeps; then the war on shipping.
2. **How a player commands his boats in this stage:** each boat by a direct order to a station, as a force is ordered. A patrol order that the staff carries out, with the boats spread over waters, comes with the second stage.
3. **An enemy boat is never shown until she is found:** no marker, no hint, until a torpedo or a sighting makes a contact.

## Amended on review, Nov 22, 2026

Read by a reviewer who had not seen it written. What follows is the plan as it now stands and replaces the rules, the figures and the questions above.

### What the review found

- **History said too strongly.** Saratoga in January was hit at 15 knots on passage, neither lingering nor flying; Hornet's force met every condition the plan named and was missed; CINCPAC's staff held a ship at 15 knots with a zigzag "reasonably safe" (event 16411). RO-33 was sunk by the escort of the ship she had just torpedoed, the one such case on file. Flying Fish and S-42 were damaged as well as I-168. Two of the five boats caught by aircraft were on the surface by day. The Japanese hunts on file in 1942 were not short: S-42 was hunted for nine hours. North Carolina and O'Brien were in another task force than Wasp, five to seven miles off.
- **"Keeping one area" was largely boats brought to a force that had been reported** (COMSOPAC: "to prevent concentration of enemy submarines"), which is the second stage's mechanism.
- **The conduct factors were used twice,** in meeting and in hitting, squaring an effect the rows give once; and "flying" would have been true of every carrier force in every daylight pulse.
- **The figures could not be counted as the table said.** Encounters over boat-days give 0.8 to 5 in 100 a boat-day on a handful of cases, and that is not the chance the rule needs, which is for a force in the boat's hex: the stations are gaps. A side's torpedo figure counted from results is what decision 0213 forbids, and the two American torpedoes differ tenfold on one side.
- **A boat as a task force of one would have been caught by every rule made for a task force:** a surface fight in her hex, a strike sent at her contact, the staff's reading of every contact as a threat to its runs and stations, alerts, the trial building her as a surface ship, fuelling, command levels.
- **Truth handed over:** an event telling the boat's side the force she met, each torpedo's chance made of the enemy's conduct, a contact at the true hex for the side torpedoed.
- **The pillars:** boats placed only by the scenario or a direct order are a task only the player can do, and the other side's staff could place none.

### A boat

1. **A submarine is a unit of her own kind,** as an air group and a land unit are: not a ship in a task force. She has a number, a name, a side, a class, a hex, a station, her salvoes left, and the pulses she is held down. Her class gives her surface speed, her torpedo and the salvoes she carries, derived from the fact rows; what the rows do not hold is a gap, and a boat whose class lacks a figure the rule needs is not put in a scenario.
2. **No rule made for a task force touches her:** she meets no surface fight, is sighted by no search and struck by no aircraft in this stage, fuels nowhere, tires not at all, and is no command.
3. **She keeps a station, a hex.** Ordered to another she goes there at her class's surface speed, in a straight passage by sea, and attacks nothing on the way. A scenario may give her a day by which she must leave for home; she then goes to the base it names and is taken off the map.

### Meeting and attacking

4. **In each pulse, after the forces have moved, for each boat on her station and not held down, and each enemy task force that was in her hex in the pulse:** one draw, under `submarine_attacks_per_hour` for each hour the force spent in the hex in that pulse, the chance for the pulse being one less the chance that every hour fails. The hours are the movement's own. A force that lies in the hex all a night has twelve hours of it; one that crosses at 30 knots, an hour and a third.
5. **A ship of the force dead in the water** multiplies the chance of each hour by `attacks_ship_dead_in_water`. No other conduct of the force moves it in this stage.
6. **Of several forces she attacks one:** the force whose best ship ranks first by her side's order of kinds (carrier, battleship, transport, cruiser, auxiliary, destroyer, as CINCPAC's plan of Midway had it, event 16224), then the lowest number.
7. **She fires one salvo at one ship** of that force: the ship first by that order, a ship dead in the water before another of its kind, then the lowest number. A salvo is her class's, and is one of the salvoes she carries.
8. **Each torpedo hits by her torpedo's `submarine_hit_chance`,** a figure of the weapon and not of the side, and a hit is worked through chapter 4 as a ship's torpedo hit is. Nothing of the force's conduct or its screen moves it in this stage. A dud is a miss, and is in the weapon's figure.

### The hunt

9. **A force with a warship that is not the ship hit hunts the boat that fired on it.** She is held down for `held_down_pulses` pulses, in which she attacks nothing. One draw: under `hunt_sinks_chance`, one figure for both sides, she is sunk.
10. **A force with no such ship hunts nothing,** and the boat is free in the next pulse.

### Where the boats are put

11. **An order is new: patrol waters with submarines.** The commander names waters, a hex and a distance, and may name the boats; the staff gives each boat it may use a station in those waters, the stations spread by a written order of the waters' hexes that puts them as far apart as they go, and sends a boat that has come back out again. A side with no commander is given its order by the scenario, as its other orders are.
12. **A direct order to a boat** gives her a station and holds her from the staff, as a direct order holds a force.

### What a side knows

13. **A side knows its own boats exactly.** Of an attack it is told that the boat fired, at a ship of the kind she took it for, through the confusion of kinds a sighting has, and what she claims: its claim is made as a ship's torpedo claim is made (chapter 3). It is given the report of the force as a sighting gives one, through the report and its error, and nothing of the truth.
14. **The side attacked** is told of each hit as it is told of any, with "by a submarine" where no attacker is in sight, and is given a report of one submarine at the hex, through the report and its error. A contact of a submarine is of its own sort: it does not merge with a contact of ships, alerts no force, is no threat in the staff's readings, is no target of a strike or a sortie, and is not built in a dry run.
15. **A hunt tells the hunters only that they hunted.** A boat sunk is told to her own side when she does not come back: at the pulse, in this stage.
16. **Nothing else shows an enemy boat.**

### Figures

All placeholders, and judgements unless it says counted.

| Figure | Rests on |
| --- | --- |
| `submarine_attacks_per_hour` | a judgement. No source gives a rate, and the stations it would be counted on are gaps. Checked, not set, against the count the rows allow: four attacks, or nine or ten with the reports of forces met, on some 190 to 500 boat-days east of the Solomons from 20 August to 20 September |
| `attacks_ship_dead_in_water` | one case and a captain's word: Yorktown under tow (outcome 16097) |
| `submarine_hit_chance`, a figure of each torpedo | counted for each weapon from the attacks on file, the count and its choices written in the weapon's rules file: the aimed ship only; a torpedo that left no tube not counted; a disputed count as a range. The Mark 10 from the S-boats' attacks, the Mark 14 from the fleet boats', the Japanese torpedoes from the six salvoes |
| a class's salvo and salvoes carried | from the class rows, a gap where they are silent |
| `held_down_pulses` | from the hunts on file, three to ten hours and more |
| `hunt_sinks_chance` | one boat sunk by the escort of the ship she hit, of some fifteen that fired |

The points for a submarine sunk, in the objectives' tables, are a placeholder like every point value.

### What is recorded

Events, each with its causes and its entry for who sees it: a boat ordered to a station; a salvo, with each torpedo's draw (her side: that she fired, the kind believed and her claim; the other side: the hits, as hits are told); a boat held down (her side); a boat sunk (her side). Three new draws, each with its own value at the end: the attack, each torpedo, the hunt. The boats are new state, hashed and named; a pulse's hours in a hex are worked in the pulse and not kept.

### Rule card

Reads: each boat's hex, station, salvoes, held-down count, class and torpedo; each enemy force's hours in her hex in the pulse, its ships and their kinds, which are dead in the water; the figures; for the staff, the order's waters and the boats it may use. Writes: torpedo hits through chapter 4; the boat's salvoes and held-down count; that she is sunk; her station and passage; reports. Draws: three new values.

### What it does not do, added

- **Nothing of a force's speed, zigzag, flying or screen moves a chance,** but the hours it spends in a boat's hex and a ship dead in the water. The rows give the direction of each and the size of none, and the first of them is carried by the hours.
- **No aircraft finds or attacks a boat,** and no search sights one. Aircraft held boats down and damaged two in August 1942; that waits for the second stage with the boats' own reports.
- **No boat moves on a report of the enemy.** The Japanese lines brought to Saratoga's and Wasp's waters are the second stage.
- **A boat burns no fuel.** A patrol's length is the scenario's.
- **A hunt does not damage a boat short of sinking her.**
- **A torpedo that misses hits no other ship.**

### Games on file

A scenario with no boat and no patrol order plays as it does, byte for byte. Boats are put into the scenarios for play in a later change, from the rows of where they were, and those scenarios are measured before and after then.

### Questions for the designer

1. **The first stage as amended:** boats as units of their own on stations, a patrol order the staff carries out for either side, one chance of attack that grows with the hours a force spends in a boat's waters, and the hunt. Aircraft against boats, boats that report and move on reports, and the war on shipping come after.
2. **An enemy boat is never shown until she attacks,** and then only as a report of a submarine near where the ship was hit.
