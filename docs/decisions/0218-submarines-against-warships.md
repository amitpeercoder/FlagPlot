# 0218: Submarines against warships

Status: a plan by the architect, written on Nov 22, 2026, on the designer's question "Is it time to plan submarines?" and his word to go ahead, and on research gathered that day (`docs/research/submarines-in-1942.md`, rows 16000 and up). It is the first of three stages. It is to be read by a reviewer who has not seen it written before anything is built, and its questions put to the designer.

Summary: The game has no submarines. In the battles on file they torpedoed Saratoga twice, sank Wasp, Yorktown and Kako, and hit North Carolina and O'Brien. This puts a submarine on the map as a boat on a station. A force that comes into her waters may be met; a boat that meets one fires at a ship of the kind she wants, the likelier to hit the slower and steadier the force and the thinner its screen; the escorts then hunt her, hold her down and may damage or sink her. A side knows its own boats and knows an enemy boat only by her torpedoes or a sighting. Scouting by submarines and patrol lines kept by the staff are the second stage, and the war on shipping the third.

Standing: Ruled and amended on Nov 22, 2026; stage 1 (the data) built; stage 2 (the rule) built on branch `submarine-rule`, to be reviewed by the architect; stage 3 not built.

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

## The designer's answers, Nov 22, 2026, and what they add

Asked one question at a time, on the plan as amended.

1. **The first stage, with aircraft against boats in it.** He chose to add aircraft now.
2. **Enemy boats: the waters where they are suspected are shown.** He chose that a side be told the waters where enemy boats are believed to be working, as intelligence told commanders then, over boats wholly unseen.

What follows is added to the plan as amended, and is to be read by a reviewer before it is built.

### Aircraft against boats

17. **A boat on her station is on the surface part of the time,** by night to charge and at times by day: the rows have boats caught on the surface at 0630 and after dawn near their stations (events 16024, 16025) as well as on passage. A boat on passage is on the surface.
18. **A search by aircraft may sight her.** In a daylight pulse a group on the search mission whose arc and range hold her hex sights her by the chance it has of sighting one ship there, times `submarine_surfaced_share` for a boat on her station, and whole for one on passage. Rule 2 is amended so far: no other rule of search touches her, and she is never a force in the search's lists.
19. **A boat sighted is reported and driven down.** Her enemy is given a report of one submarine, through the report and its error. She is held down for the rest of that pulse and the next.
20. **The aircraft that sight her attack her,** in the same pulse, by one draw: under `air_attack_damages_chance` she is damaged, and of those, by a second draw under `air_attack_sinks_share`, sunk. No strike is launched at a submarine and no group's mission is changed for one.
21. **A boat damaged goes home.** She leaves her station for the base her scenario names, attacks nothing, and is taken off the map there. Her side is told. This is state: a boat is sound, damaged or sunk.
22. **The hunt of rule 9 gains the same middle:** by one draw under `hunt_damages_chance` she is damaged, and of those under `hunt_sinks_share` sunk. `hunt_sinks_chance` is withdrawn. The rows have, of the boats that fired, one sunk and three damaged (I-168, Flying Fish, S-42).

### Suspected waters

23. **A scenario may give a side the waters where it believes enemy submarines work:** a hex, a distance, the words of the belief and the rows it rests on. It is what that side's command believed at the start, right or wrong, and is not made from where the boats are. The rows hold such beliefs: CINCPAC from 9 August that Japanese boats were concentrating against the Solomons (events 16298, 16303); the Japanese that one or two Allied boats were in the Bismarcks when there were four (event 16257); the Japanese staff's count of fourteen or sixteen American boats round Midway (events 16200 to 16202).
24. **The side is shown them:** in its briefing, in words, and on its chart as waters marked faintly, with no boat drawn.
25. **In play the belief is added to by what the side learns:** each report of a submarine (rules 14 and 19) is shown as a contact of its own sort for as long as a contact is held. The suspected waters themselves do not change in this stage.
26. **A side with no such entry is shown nothing** until a boat is reported.

### Figures added

| Figure | Rests on |
| --- | --- |
| `submarine_surfaced_share` | a judgement: boats charged by night and were seen at times by day |
| `air_attack_damages_chance`, `air_attack_sinks_share` | counted, on few cases: five boats caught by carrier aircraft in August 1942, two damaged, none sunk (events 16024 to 16029); one figure each for both sides |
| `hunt_damages_chance`, `hunt_sinks_share` | counted, on few cases: of the boats that fired at a warship or her charge, three damaged and one sunk; one figure each for both sides |

Two draws are added to the three, each with its own value at the end: the aircraft's attack and its sinking share are one stream of the boat and the group; the hunt's two are one stream of the boat and the force. A boat's state gains whether she is damaged.

### What it still does not do

- **Aircraft do not hunt a boat from a report,** patrol against submarines by order, or keep a boat down by flying over a force. A force's own patrol does not move a boat's chance to attack it. The rows have Japanese captains fearing aircraft most and give no size.
- **A boat does not report what she sees or move on a report.** The second stage.
- **Suspected waters are not worked out in play** from attacks and sightings; they are the scenario's.

## Amended on a second review, Nov 22, 2026

Read by a second reviewer who had not seen it written, on rules 1 to 26. Where this differs from what is above, this holds. The architect checked the review's findings on the rows and the search rule before ruling.

### What the review found

- **The rows do not hold what rule 1 needs for the boats this stage is about.** No American class has its torpedoes carried; three Japanese groups have none; no row ties a class to a torpedo; which Japanese boats carried which torpedo in 1942 is a listed gap; a passage speed is on file for the Japanese groups alone.
- **A unit of a new kind touches every place a land unit does,** some sixty files, and the plan counted few of them.
- **The hours a force spends in a hex exist only inside the movement step,** and the plan named neither where the attack is worked nor how a part of an hour is counted.
- **A submarine contact kept among the contacts of ships would be read by 46 places,** each of which would have to leave it out, and a report of "nothing there" would delete one.
- **Rule 13 gave the boat's side a report of the force,** which is the boat reporting what she sees: the second stage.
- **A search that starts in a boat's hex sights at a chance of one,** so a boat under a searching carrier would be found at the surfaced share every daylight pulse: a patrol over the force, which the plan said it did not make.
- **The history was miscounted in four places.** I-11 was damaged by a destroyer of the force she fired on, so four boats that fired were damaged, not three. All five boats caught by carrier aircraft were caught by day, three of them on passage. CINCPAC's order of kinds (event 16224) has no destroyer, and the Japanese order was "supply ships or capital ships" (event 16281). The rows of beliefs round Midway are of other waters, and CINCPAC's belief of the Solomons is dated 9 August, after the 39 days begin.
- **The staff would have done nothing about submarines,** so that keeping carriers out of marked waters would be a task only the player can do.

### The data, before anything else is built

27. **Research is commissioned for what the rows lack:** each class's torpedoes carried and the tubes that bear forward, its passage speed on the surface, and the torpedo each boat carried in 1942. What is found is added as rows.
28. **The derivation is by written rules, in the submarines' rules file:**
    - a salvo is the class's tubes that bear forward;
    - the salvoes carried are its torpedoes carried over its salvo, rounded down;
    - the passage speed is the class's speed of endurance on the surface;
    - the damage of a hit is by the torpedo's warhead, by the figure the ships' torpedoes use.
29. **A figure still missing after the research is estimated from like boats by the written rule of decision 0207, and marked estimated.** A boat whose torpedo no row states takes the torpedo stated for the boats of her group, or of the group nearest in date, and is marked. A boat is left out of a scenario only where no like boat gives the figure.
30. **`submarine_hit_chance` stays a figure of the weapon,** counted from the salvoes whose weapon is stated or is estimated by rule 29, the count saying which.

### The boat as a unit

31. **Her reference is a new kind, its code after the last.** The boats are a field of the state, hashed and named, and hashed only where a scenario has one, as the units held apart are: a game with no boat keeps its hash.
32. **She is not a kind of ship.** Her classes are a table of their own; the kinds of ship and every table keyed on them are as they are. The objectives gain one entry, the points for a submarine sunk. A boat sent home damaged scores nothing in this stage.
33. **She may be held apart and enter on a day, and may have a day to be used from,** as any unit.
34. **An order's forces limit may name boats, and a hold may hold one.**
35. **With command levels on she is under her side's highest command,** which knows her exactly and holds her reports first. That a boat's own word came late or not at all is the second stage's.
36. **Every step that walks each unit gains her case,** and the build lists them before it writes any: the invariants, the pictures and the world built from a picture, the briefing, the side's view and a seat's, the agent's tools, the export and the viewer, a saved game, the events' reader, and the table's cards and chart. A dry run builds the side's own boats and no enemy boat.

### Meeting and attacking, restated

37. **The attack is worked at the end of the movement step, in each pulse,** while the step's own times are in hand, and nothing of them is kept. By night that is before a bombardment and before loading and unloading: a force hit then goes on with what it has.
38. **A force's hours in a boat's hex in a pulse are:** all the pulse where it did not move; from when it entered to when it left where it crossed; from when it entered to the end where it stopped or was left dead there; from when it was made where it was detached in the hex. A force with no ship left has none.
39. **The chance for the pulse is one less the chance that every hour fails,** worked so: one less the hourly chance, multiplied by itself for each whole hour, and once more by one less the hourly chance times the part of an hour left over.
40. **The order of kinds is one for both sides: carrier, battleship, transport, auxiliary, cruiser.** A force with none of these is not attacked. It rests on CINCPAC's order and the Japanese one (events 16224 and 16281); neither names a destroyer.
41. **A force lying in a hex that holds a base of its own side is not attacked.** A judgement, marked: no harbour is in the game, and the war on shipping will rule on it again.
42. **Each torpedo's draw is of the boat and the ship aimed at, by the torpedo's place in the salvo.**
43. **She goes to a station by the sea route a force would take, at her passage speed,** and is on station from the pulse after she arrives. A boat with no salvo left goes home as a damaged one does, and scores nothing.

### What a side knows, restated

44. **Reports of submarines are a list of their own in a side's belief,** as the estimate of the enemy ashore is: a report of its own kind, taken in by the one way a belief changes, aged and dropped by its own figures, copied into each command's picture. Nothing that reads the contacts of ships reads it. Two reports in one hex are one, the newer; a report of nothing there removes none.
45. **Rule 13's report of the force is withdrawn.** Her side is told that she fired, the kind she took her target for, and her claim, which is made as a raid's claim is, its hits raised by a draw, and marked as a submarine's.
46. **The side fired on is told of every salvo, hit or miss,** as a ship's salvo against it is told, with no boat named, and is given the report of a submarine at her hex through the report's error. A salvo that misses is hunted as one that hits.
47. **Suspected waters are a belief told on a day,** as the opening estimates are (decision 0160): taken in as a report, recorded, hashed, and in every picture. An entry carries its day, so that CINCPAC's of 9 August is told on the third day of the 39. Its hex and distance are the architect's reading of the row's words and are marked so. The rows of beliefs round Midway are withdrawn as examples.

### Aircraft, restated

48. **The chance of sighting a boat takes its distance as at least one hex.** `submarine_surfaced_share` stands for both the time she is up and how much harder she is to see than a ship; it is a judgement.
49. **There is no search by night, and rule 17's words on the night are withdrawn.** Three of the five boats caught were on passage; nothing rests on boats caught near their stations.
50. **The attack is a draw and nothing else:** no bomb is used, no aircraft lost, no extra sortie flown.
51. **Any group on the search mission may sight and attack,** flying boats too. `air_attack_damages_chance` rests on the six attacks by carrier aircraft on five boats, two damaged, and the Catalinas' attacks that did none (events 16021, 16022, 16306); `air_attack_sinks_share` is a judgement, none having been sunk.
52. **A boat damaged goes home.** I-121 went on working damaged; the rule is simpler than the row and says so.
53. **`hunt_damages_chance` is restated on four boats damaged and one sunk** of some fifteen that fired; RO-33's target was a merchant ship and is counted.

### The staff and the player

54. **The patrol order is a new root kind of order.** With no boats named it uses every boat of the side that is not held. Its stations are given so: the first is the waters' centre, or the navigable hex of the waters nearest it; each next is the navigable hex of the waters farthest from the nearest station already given, the lowest hex where they tie; boats beyond the hexes share stations from the first again. Its preview and reasons say each boat's station. The words on sending a boat out again are withdrawn: a boat that has gone home is off the map.
55. **The staff keeps a standing force out of submarine waters where another place serves.** A doctrine that chooses where a force is to stand or wait (the carriers' position, a station, a place to stand at) passes over a hex inside the side's suspected waters or within one hex of a report of a submarine it holds, and says so in its reason; where no other hex serves it takes the hex and says that. Routes are not changed. The doctrine's version rises.
56. **The table may stop the clock on a submarine's attack,** by a stop of its own as for reinforcements. No interrupt of the engine is added.
57. **What a player can do is then:** keep forces out of the waters by an order's area, go faster, not stand in them, keep searches flying, send his own boats, and leave the rest to the staff. Escorts, a zigzag and a patrol over a force move nothing in this stage; a destroyer counts in the hunt.

### Draws and records, in full

Each a new value at the end, in a subsystem of its own: the attack (boat and force); each torpedo (boat, ship and place in the salvo); the hunt's two (boat and force, one stream); the search's sighting of a boat (boat and group); the aircraft's attack and its share (boat and group, one stream); the error of a report of a submarine; the claim. With breakdowns on, each attack chance is recorded with its hours and factors.

### How it is built

Three pull requests, each its own stage:

1. **The data:** the research, the rows, the derivation rules and the classes derived.
2. **The rule:** the boat and her state, the hours and the attack, the hunt, the list of reports, aircraft, the station order, holds, suspected waters, with the scenario and its hand calculation, which uses placeholder boats.
3. **The staff, the agent and the table:** the patrol order and its stations, the staff's regard for submarine waters, the tools, the cards and the chart.

### A simpler way, considered and not taken

Waters kept by a side with a count of boats in them, and no boat as a unit, would cost far less. It is not taken: the table names every ship, and the second and third stages need a boat that moves, reports and is sunk.

### Rule card, restated

Reads: each boat's hex, station, passage, salvoes, state and held-down count, her class and torpedo; each enemy force's hours in her hex in the pulse, its ships, their kinds, which are dead in the water, and whether the hex holds a base of its side; each searching group's arc, range and aircraft; the figures; the side's suspected waters and reports of submarines, for the staff. Writes: torpedo hits through chapter 4; the boat's salvoes, state, held-down count, station and passage; reports of submarines and their ageing; claims. Draws: the seven above.

## As built: stage 1, the data

Built on Nov 22, 2026, on branch `submarine-data`, by a team lead for the architect, and amended the same day on the architect's rulings (next section). No rule of the simulation is built and nothing in `sim-core` or `sim-data` changes; no scenario loads the new tables.

**The list.** `data/derivation/submarines_1942.toml`, dated 7 August 1942. Each of the seventeen classes of the type submarine in `ship_classes.csv` is either derived (`[[submarine]]`) or left out with its reason (`[[left_out]]`), and the tool refuses a list that does neither for one. Ids pinned: classes 6000 to 6011, torpedoes 6100 to 6104 (`[submarine_class_ids]`, `[submarine_torpedo_ids]`).

**The written rules** (rule 28) are set out in `data/derivation/rules.toml` under "Decision 0218, stage 1" and worked in `crates/oob-derive/src/submarine.rs`: salvo = `torpedo_tubes_bow`; salvoes carried = `torpedoes_carried` / salvo, rounded down; passage speed = `endurance_speed_kn`; damage = `warhead_kg` x `torpedo_damage_per_warhead_kg`. Every fact is read as any row is (`reading::resolve`: preferred rows and exceptions, dates, `later = true`). Rule 29: a passage speed may be an `[[estimate]]` of decision 0207 with the class as subject (its refusals all hold); a class whose torpedo no `torpedo_model` row states, of any standing, takes the torpedo of a `torpedo_from` entry (`weapon`, `group` for the group nearest in date by the year built, `rows` cited, `by`, `date`, `reason`), refused where any such row is filed for the class. Either is named in the class file's column `estimated`.

**The files** (generated, never edited): `data/reference/submarine_classes_1942.csv` (`id,name,salvo,salvoes_carried,passage_speed_kn,torpedo_id,estimated`) and `submarine_torpedoes_1942.csv` (`id,name,damage`), each with a `.sources.csv`; their frozen copies in `data/reference/step1`. Checked by `crates/oob-derive/tests/submarines.rs` (the list derives exactly the files on disk, and the rules on small data sets) and by step 6 of `scripts/regenerate.sh`.

| Id | Class | Salvo | Salvoes | Passage kn | Torpedo |
| --- | --- | --- | --- | --- | --- |
| 6000 | Sargo | 4 | 6 | 10 | Mark 14, estimated |
| 6001 | Tambor | 6 | 4 | 10 | Mark 14, estimated |
| 6002 | Gato | 6 | 4 | 10 | Mark 14, estimated |
| 6003 | I-15 block | 6 | 2 | 16 | Type 95, estimated (her boats) |
| 6004 | I-153 block | 6 | 2 | 10 | Type 89, estimated (the I-121 block's boat) |
| 6005 | I-168 block | 4 | 3 | 10 | Type 89 (class row) |
| 6006 | I-176 block | 6 | 2 | 16 | Type 95, estimated (the I-15 block's boats) |
| 6007 | RO-33 group | 4 | 2 | 16 | Type 95 (class row) |
| 6008 | I-121 block | 4 | 2 | 12 | Type 89, estimated (her boat I-123) |
| 6009 | S-boats, S-1 group | 4 | 3 | 8.686399, estimated | Mark 10 (class rows) |
| 6010 | S-boats, S-42 group | 4 | 3 | 8.686399, estimated | Mark 10 (class rows) |
| 6011 | Narwhal | 4 | 6 | 10.423678, estimated | Mark 14, estimated |

The three passage speeds are by ratio from the class's top speed (14.5 and 17.4 knots) times 0.599062, the mean of endurance speed over top speed of the eight like classes with both rows (the I-1, I-15, I-168, I-176 and RO-33 blocks and the Sargo, Tambor and Gato classes), between 0.434782 (I-168) and 0.846560 (RO-33): 6.304339 to 12.27512 knots for the S-boats, 7.565206 to 14.730144 for Narwhal.

Torpedoes: Mark 14 92, Mark 10 90.72, Type 89 120, Type 95 162, Type 96 162 (warheads 230, 226.8, 300, 405 and 405 kg at 0.4 a kilogram, a placeholder).

**Left out:** the fleet type (not a class); the I-1 block (two types in one block, to be split when rows of its own exist); I-5 and I-9 (no speed); RO-35 (no speed, 1945 rows only); the Type 92 (no warhead).

**The count of torpedoes and hits** of rule 30 is in the list's closing notes and sets no figure.

## The architect's rulings on stage 1

Ruled on Nov 22, 2026, on pull request 684, and recorded as the repository records a settled dispute: an exception in `data/derivation/rules.toml` with the reason on it, no row on file changed. Every exception and entry that had said it was the lead's proposal now says it is settled by the architect, with the reason.

**Disputes.**

1. **I-9 class torpedoes carried: 20** (ONI 222-J, row 16214, read with `later = true`), over Wikipedia's 18 (16218, 16220): a class sheet of naval intelligence over a tertiary page; the salvoes do not move. The class stays out until its speed is found.
2. **I-5 class bow tubes: 4** (ONI 222-J, 16206, `later = true`), over Wikipedia's 6 (16210): two sources of the time agree, ONI and the Technical Mission's block for I-1 to I-6. Out until its speed is found.
3. **The I-1 block is to be split** into the J1 boats (I-1 to I-4) and I-6, as new class keys with new pinned ids, only if no new fact row is needed. Every row on file speaks of the block and stays the block's, so the new classes would have no rows of their own: the block is left out and the rows wanted are listed below.
4. **Type 96 warhead: 405 kg** (the Technical Mission, 16063), over NavWeaps's 550 (16064): the Technical Mission examined the weapon and derives it from the Type 95 Model 1; the other names another explosive and no document. The Type 96 is in the torpedo table (6104).
5. **The I-15 block's torpedo: Type 95,** estimated and marked, resting on I-19 at Wasp (3002, the one preferred, stated row for a boat of the block in the period) and I-21's firings (16433, 16434); I-26's Type 89 of 7 June (16435) stays on file as evidence against and is named in the entry's reason. ONI's torpedo line for the class is to be filed by research before stage 2's scenarios use it; the list says so.
6. **Gato top speed: 20 knots** (16105), over 20.25 (16100), the lower of two rows of one source, as decision 0207's first use read. With it the passage speeds of the S-1 group, the S-42 group and Narwhal are worked by decision 0207's estimate, marked; the S-boats carry the Mark 10 (stated by their class rows); Narwhal takes the fleet type's Mark 14, estimated, as Sargo does. The S-boats' bow tubes are read from their lone unconfirmed rows (16131, 16135) by exception. The estimate also needed the like classes' rows dated only to 1942 (the top speeds of the I-1, I-15, I-168, I-176 and RO-33 blocks and the I-1 block's endurance speed), read by the same kind of exception as the classes' own.
7. **I-121 block torpedoes carried: 8** (the Technical Mission, 16033), the row of the time over one of 1945 (16060), pending research. The class is derived (6008); its torpedo is the Type 89 of its boat I-123 (16429), estimated.

**Readings accepted.** The Mark 14 for the Sargo, Tambor and Gato classes from the fleet type's rows, marked estimated, the Mark 10 as a reserve weapon set aside. The group nearest in date is by the year the boats were built, and the rules file says so. The exceptions that read a lone weak row with no row against it, each marked, each giving way to a better row when one is filed. The reasons are worded so that the generated header no longer prints a doubled stop; the line format is unchanged.

**Research still owed, as rows wanted.**

- **The I-9 group:** its surface speed of endurance (`endurance_speed_kn`) or its top speed (`max_speed_kn`); the Technical Mission's S-01-1 block for I-9 to I-12 (the source of row 16188) may hold them.
- **The I-5 group:** `endurance_speed_kn` or `max_speed_kn`.
- **The RO-35 group:** `endurance_speed_kn` or `max_speed_kn`, and whether its ONI sheet covers RO-33 and RO-34 or boats of 1942 at all.
- **The I-15 group:** ONI 222-J's I-15 sheet's torpedo line, filed as a `torpedo_model` row (row 16051's as_stated reads "Typa89s"); once filed, the class's `torpedo_from` is driven out.
- **The I-168 and RO-33 groups:** `torpedoes_carried` from a primary source (today Wikipedia alone, 16222 and 16226).
- **The I-1 block's split:** for the J1 boats (I-1 to I-4) and for I-6, each: `torpedo_tubes_bow`, `torpedoes_carried`, `endurance_speed_kn` and `torpedo_model`, filed to the new class keys, and the boats' `class` in `ships.csv`.
- **The S-boats:** a range with its speed (`endurance_speed_kn`), which drives out their estimate; a primary statement of their bow and stern tubes.
- **Narwhal:** `endurance_speed_kn`, which drives out her estimate; her torpedo by class and date.
- **The I-121 block:** torpedoes carried from a third source, to settle 8 against 20.

## As built: stage 2, the rule

Built on Nov 22, 2026, on branch `submarine-rule`, by a team lead for the architect, in the phases the second review named: the boat as a unit with her state, hash, loader and views (phase 1, `8a74aeff`); the hours in a hex and the boat's passage (2a, `5e5d00b5`); the attack, the salvo and the hunt (2b, `943e4db4`); reports of submarines in belief and a boat's claim (3, `6e16beb3`); aircraft against boats (4, `67174ee5`); the station order, suspected waters, the objective and the briefing (5, `a7216d50`); and the architect's rulings on the way (`5d0b1468`, `56732c8e`). The hand calculation is scenario 128 (`tests/scenario_128_submarines_against_warships.rs`, the architect's text kept as written) and the unit tests in `crates/sim-core/src/submarine_tests.rs`. The staff, the patrol order, the table and the chart are not built: they are the third stage's. The rules as built are in `docs/rules/10-submarines.md`, with a short addition to chapters 1, 2, 3, 4 and 6, the glossary and `docs/rules/differences.md` (rows 60 to 68, row 63 settled in this change by mending the notes).

**Where it is.**

- `sim-core`: `submarine.rs` (the passage, the attack, the salvo, the hunt, a boat's claim and report, the air group's look for a boat and its attack, sending a boat home); `suspected_waters.rs`; `movement.rs` (the hours ledger, `HoursInHex`, local to the step and kept nowhere); `search.rs` (the call that looks for boats after a group's look for forces); `belief.rs` (the report kind `Submarine`, the report kind `SuspectedWaters`, their lists, `age_submarine_reports`, a claim's `from_submarine`); `state.rs` (`Submarine`, `SubmarineCondition`, `SubmarinePassage`, `WatersWaiting`, `TruthState::submarines` and `waters_waiting`); `resolve.rs` (the station order's check and effect, the ageing of reports at the end of a pulse, the waters told at the close of a day); `entering.rs` (`SubmarineEntered`); `available.rs`; `orders.rs` (`OrderKind::Submarine`, `SubmarineOrder::Station`, the two reasons `SubmarineOffTheMap` and `SubmarineGoingHome`); `events.rs` and `side_events.rs` (fifteen new kinds of event and six cut ones); `rng.rs` (the subsystem `Submarine` and seven `Draw` values); `objectives.rs` (`submarines_sunk`); `refdata.rs` (`SubmarineData`); `hash.rs`; `invariants.rs`; `view.rs`, `view/`, `pictures.rs`, `passing.rs`, `world_picture.rs`, `world_picture/half.rs`, `world.rs`.
- `sim-data`: `reference.rs` (the class, torpedo and figure loaders) and `scenario.rs` (`[[submarine]]`, `[[suspected_waters]]`, the three file keys, the objective and the station order).
- `sim-agent`: `submarines.rs` (the briefing's part), `briefing.rs`, `queries.rs`, `orders.rs`, `common.rs`, `text.rs` and the schemas: the briefing's `submarines` part, `list_forces`' `submarines`, the entity answer `submarine:N`, the order `station`, a claim's `from_submarine`.
- `sim-cli`: `events`, `export` and `main` carry the boats; `viewer/` draws them (`viewer/README.md`).
- Data: `data/reference/submarines.toml` (the figures, all placeholders); the placeholder files of scenario 128 (`data/reference/submarine_classes_placeholder_scenario_128.csv`, `submarine_torpedoes_placeholder_scenario_128.csv`, `submarines_placeholder_scenario_128.toml`, their frozen copies of the first two, `data/map/placeholder_submarines.toml`, `data/scenarios/128_submarines_against_warships.toml`). The live classes and torpedoes are stage 1's files.

**The architect's rulings on stage 2.**

1. **Two figures were added** to the record's list, both in `submarines.toml`. `submarine_false_hit_chance`, the chance a torpedo that missed is claimed a hit, stands where a raid's claim figure stands (`air_combat.toml: bomb_false_claim_chance`, the chance a bomb that missed is claimed a hit) and takes its value, 0.1: no row sizes it. `submarine_report_drop_age_pulses` is the age at which a report of a submarine is dropped, the "own figures" of rule 44, 6 pulses, the team lead's placeholder.
2. **The live hit chances,** by torpedo, each still a placeholder and set from the count in the stage 1 list's closing notes (rule 30): Mark 14 (6100) 0.15, from 0 to 7 hits of 26 fired, several only heard; Mark 10 (6101) 0.6, from 9 to 11 hits of 16; Type 89 (6102) 0.3, from 2 to 3 hits of 7 or 8; Type 95 (6103) 0.3, from 3 to 4 hits of 12; Type 96 (6104) 0.3, no firing on file, taken as the Type 95 it was made from, and marked so.
3. **Rule 41 is read** as: a force's hours in a hex that holds a base of its own side on the map do not count. A force that crosses such a hex is not attacked in it any more than one that lies there. A base held off a side's map does not protect.
4. **Suspected waters are told at the close of day D - 1** for an entry of day D, after the end phases and the afternoon's state reports, so that day D's briefing and view hold them, as the opening estimates are (decision 0160); an entry of day 1 is held at setup, on day 0. They are never changed in play.
5. **The warships of the hunt** (rule 9) are a carrier, a battleship, a cruiser and a destroyer, other than the ship aimed at, as the ship kinds of the code name them. A transport and an auxiliary are not warships.
6. **A boat is held apart by a field of her own** (`Submarine::enters_day`), not by the entries of decision 0195. The entries' list holds forces with their ships, land units and air groups at bases, and a unit that waits for its base; a boat has no base to wait for, and an entry for her would have changed the walk of the entries and the hash of every game that has one. With her own field the entries' mechanism and hash are untouched, and her entering is her own event, `SubmarineEntered`, taken in after the entries of the units, first thing on her day.
7. **The scenario key `held_down_pulses`** of a `[[submarine]]` means held down through day 1's N-th pulse, counted from the night, the night included. It is not the figure of the same name in `submarines.toml`, which counts pulses after a hunt.
8. **The ledger's remainder:** a force that did not steam the pulse out has the rest of the pulse in the hex it ended in, the pulse's hours less the hours it steamed (or less the hours before it was made). A force that steamed the pulse out, or that ran short of fuel and was left where it stopped, is counted as rule 38 says.
9. **A passage's leg is 0 at a centre.** A force's leg is 40 miles at a centre, as the invariants say; a boat's is 0, since she holds no leg until she sets out on the next step.
10. **A7's row is restated:** the force "passes by (4,0), (5,-1), (6,-1) and (6,0) and never enters (5,0)", a straight route through (4,0) and (6,0) entering (5,0).
11. **The cut of a salvo from a submarine** is that of a ship's salvo against a side: the side fired on is told that its ship was fired on from a submarine and where, and neither the torpedoes nor the hits; the hits reach it as hits do. This replaces the cut of phase 2b that kept the torpedoes.
12. **A2 and A8** are read as the hand calculation works them (6 hours stopped, a miss of 0.885841; a carrier dead in the water, an hourly chance of 0.1 and a miss of 0.531441), **statement 12's held-down period** (the rest of the pulse and two after) and **statement 13's bands** (the chance a boat is sunk 0.06, damaged and afloat 0.24, untouched 0.7) are accepted as built.
13. **A ship is dead in the water** when her propulsion gives her no speed, as the movement rules read it (the speed factor of the propulsion's state is 0). With no surface combat figures none is.

**The hash.** The boats, each side's belief's and each command's picture's reports of submarines and suspected waters, the places of the claims marked a submarine's and the waters still waiting are hashed last of all, only where the game has a boat, waters waiting or any of those held, as the units held apart and the supply lines were. A game with none writes nothing there and keeps its hash. `STATE_VERSION` is not raised (`flagplot-state-v56`); the golden file `tests/golden/pinned_hashes.txt` gains two lines only, the pinned hashes of scenarios 4a and 5b for the hand calculation's statement 27 (a game with no boat keeps its hash), and no golden file is remade. A boat's name, the issuer of an order and what a view shows are not hashed.

**The versions.** The rules label is 29 (28 before it), since a boat is state a game saved under the old rules could not hold. The agent interface version is 47 (46 before it): the briefing's `submarines` part, `list_forces`' `submarines`, the entity `submarine:N`, the order `station`, a claim's `from_submarine` and the objective `submarines_sunk`. The saved game's version is unchanged: a station order is saved as any order is, with the unit `submarine:N`.

**Where the build differs from the record.** Rows 60 to 68 of `docs/rules/differences.md`: the cut of a salvo (ruled), a boat with no salvo at the start (not ruled), holds and forces limits that name a boat (not ruled), the note of `held_down_pulses` in the data file (the code stands), the leave day (the code stands), rule 41 and A7 (ruled), the system draws of the torpedoes' stream (the code stands), a boat before her day (not ruled) and a home base the sea cannot reach (not ruled).

### Rule card, as built

- **Reads:** each boat's hex, station, passage, salvoes left, condition, the last pulse she is held down, `available_from`, `enters_day`, `leave_day`, home base and class, with her class's salvo, passage speed and torpedo and the torpedo's damage and hit chance; each task force's ships and their kinds and classes' toughness, which are dead in the water, its hours in each hex of the pulse (from the movement step), and, for the hunt, whether it holds a warship besides the ship aimed at; each base's hex, side and whether it is on the map; each searching group's origin, arc, range, aircraft and sweep width; `search.toml`'s position error and confusion rows; `surface_combat.toml`'s system chance and torpedo table; the figures of `submarines.toml`; each side's command chain, for who holds a report first; the scenario's suspected waters and the waters waiting.
- **Writes:** a boat's hex, passage, salvoes, held-down pulse and condition; the flotation, systems and sinking of the ship aimed at, through chapter 4; a claim (marked a submarine's) in her side's belief and the area command's picture; a report of a submarine in the side fired on's or the searching side's belief and in its command's picture, and the ageing and dropping of reports; suspected waters in a side's belief and every picture, and the list waiting; the events of Events in chapter 10; the objective's count at the end of the game, read from the final state.
- **Draws:** seven, each its own `Draw` value in the subsystem `Submarine` (code 10): `SubmarineAttack` 22 (boat and task force), `SubmarineTorpedo` 23 (boat and ship aimed at; the torpedoes, then the system draw of each hit), `SubmarineHunt` 24 (boat and task force; the damage, then the sinking), `SubmarineSighted` 25 (boat and air group), `SubmarineAirAttack` 26 (boat and air group; the damage, then the sinking), `SubmarineReportError` 27 (boat and ship aimed at, or air group; the position error, then the direction) and `SubmarineClaim` 28 (boat and ship aimed at; the kind taken for, then a false hit for each torpedo that missed). Suspected waters and a passage draw nothing.

## The architect's rulings on stage 2, Nov 22, 2026

Read on the attack's code, the side's view, the pinned hashes and the scenario's file before merging. The hand calculation is as the architect wrote it but for the row of game A7, which he restated: a force cannot pass (4,0) and (6,0) in a line and miss (5,0).

- **The side fired on is told of a salvo with no count of torpedoes and none of hits,** as a ship's salvo against it is told. It learns of hits by the damage to its ships.
- **The digests of events in the tests of scenarios 106, 126 and 127 are remade** for the field a claim gained, as they were for decision 0210. With the field taken out each is what it was.
- **A world with a boat must have the search figures,** and **suspected waters still to be told are in no side's half of a picture:** both accepted.
- **Row 61** (a boat given no salvo by her scenario is never sent home): the loader is to refuse a boat with no salvo, in stage 3. Open until then.
- **Row 62** (no forces limit names a boat, no hold holds one): stage 3, as planned.
- **Row 64** (she leaves in the night pulse of her leave day): accepted; the rulebook says it.
- **Row 65** (the hours in a hex with a base of the force's own side do not count): accepted, as ruled while it was built.
- **Row 66** (a hit's draw for a system comes from the torpedo's stream after the torpedoes): accepted; the stream is the boat's and the ship's and nothing else reads it.
- **Row 67** (a boat before her day to be used attacks nothing): accepted. A boat not yet to be used is not on patrol; a force before its day is still fired on because the enemy does the firing.
- **Row 68** (a home base no ship can reach is not checked): the loader is to refuse it, in stage 3. Open until then.
- **Not checked by the team and owed:** reports of submarines passing between commands where reports can be late or lost; the briefing's words read through; the viewer by eye.
