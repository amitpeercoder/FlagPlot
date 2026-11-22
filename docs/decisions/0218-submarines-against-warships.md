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
