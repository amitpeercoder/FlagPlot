# Research prompts: 1942 on one map, 1 May to 30 November 1942

Written on Nov 28, 2026 at the designer's request, for the scenario of `docs/design/one-map-1942.md`: one game from 1 May to 30 November 1942 on one map from Papua to Midway. They are for an outside model with access to the web (ChatGPT's latest). Nothing here is settled and nothing here is a plan of the game: the prompts gather sourced statements, and our readers check every statement against its source before a row is keyed into `data/oob`, as the passes of Nov 13 and Nov 14 did (`docs/research/oob/outside-research-nov-13.md`, `outside-research-nov-14.md`).

These stand beside `prompts.md`, which asks the same kinds of question of the whole war. Where one of its prompts already covers a need for these months, this file says so and does not ask twice.

## How to run them

The common block of `prompts.md` governs every run here: the four tables, the columns, the seventeen rules, where the answer is written and what may be read. It is not repeated.

1. **Start each run with one line**, naming one prompt and one part, for example:
   `Read the common block of prompts.md, then prompts-1942.md. Follow the common block and run prompt W01, part 2.`
   One part per run, and a fresh session for each.
2. **Answers go to `research-answers/`**, one file per run, named `<id>-part<n>-<short-subject>.md` (`W01-part2-japanese-directives.md`). The folder is ignored by git.
3. **About 80 statements a run.** A part that would need more is split again, and the prompt says where to start.
4. **Run by the web alone**, without a checkout: paste the common block of `prompts.md` (the fenced block under "The common block"), then the block below, then the prompt's own block. The answer is then one Markdown document in the chat, and the lines of the common block about files in the checkout do not apply.

## The block for this file (read after the common block, before every run)

```
THIS RUN IS FOR ONE SCENARIO

The period is 1 May to 30 November 1942. Give a fact from before 1 May
only where it says how things stood on 1 May, and one from after
30 November only where a prompt asks for it.

The map runs from about 30 S to 32 N and from about 142 E to 155 W. On
it: Papua and north-east New Guinea, north-east Australia down to
Brisbane, the Bismarcks, the Solomons, the New Hebrides, New Caledonia,
Fiji, Samoa, Tonga, the Gilberts, the Marshalls, the eastern Carolines
with Truk, Wake, Midway, Johnston, Palmyra, Canton and the Hawaiian
Islands. Off it: Japan, the Aleutians, the west coast of America,
Sydney, New Zealand, the Netherlands East Indies, the Philippines.
A force that comes from or goes to a place off the map matters to us
on the day it crosses the edge: give that day where a source gives it,
and never work it out yourself.

What the game keeps from history is what a commander could not change:
where forces stood, what was sent to him and when, what was taken from
him and when, what he was ordered to achieve and by what date, and
what he believed of the enemy. So in every prompt, where a source says
WHY a force was sent, held back, or taken away, or WHY a date was set
or changed, quote that sentence as a statement of its own.

On file already, to be skipped unless a prompt says otherwise: the
ships, air groups and events of the battles of the Coral Sea
(4 to 8 May) and Midway (3 to 6 June); Guadalcanal and Rabaul from
7 August to 30 November 1942, ships, air units, land units and the
runs to the island; the submarines of 1942 as classes.
```

## Order

| Tranche | Prompts | Why first |
| --- | --- | --- |
| 1 | **W01**, **W02** | The directives and their dates are the frame of the game, and nothing like them is on file |
| 2 | **W03**, **W04** parts 1 and 2, **W07** | The first morning, what came and went, and the places of a map that is not yet drawn. Stage 2 of the note (1 May to 10 June) needs these |
| 3 | **W08**, **W09** | Oilers and intelligence: both needed by stage 2, both partly on file |
| 4 | **W04** parts 3 and 4, **W05**, **W06**, **W10** | Papua ashore, the air war over it, and aircrew: stage 3 of the note |

## Prompt index

| Id | Subject | Parts | What the game would use it for |
| --- | --- | --- | --- |
| W01 | Directives, their dates, their postponements and the plans dropped | 2 (Allied, Japanese) | The directive a seat is given, its date, and what follows it met and not met |
| W02 | The timetable of each operation, from the order to the landing | 2 (Japanese, Allied) | The span a directive allows, and the chain of dates a plan is worked back through |
| W03 | The position on 1 May 1942 | 4 | The first morning |
| W04 | What entered and left the map, 1 May to 30 November | 4 | Forces that arrive by date, and the reason each was sent |
| W05 | Papua ashore, 21 July to 30 November | 3 | A land order of battle, the track, and how men were fed on it |
| W06 | The air war over Papua, the Bismarcks and north-east Australia | 2 (Allied, Japanese) | Air units by base and date outside Guadalcanal |
| W07 | Places and bases of the map | 3 (by region) | The map: positions, fields, ports, garrisons |
| W08 | Oilers, fuel at the main bases, and passages made | 2 (Allied, Japanese) | Fuel at sea over a wide map; speed made good |
| W09 | What each side read, knew and believed, by date | 2 (Allied, Japanese) | The first form of intelligence; the believed forces decision 0173 owes |
| W10 | Carrier aircrew through 1942 | 2 (American, Japanese) | Experience for air groups (decision 0064 waits on this) |

### Already asked in `prompts.md`, and not repeated here

- **Guadalcanal's remaining gaps** (the Cactus Air Force by squadron, the runs night by night, supply by month, Japanese air strengths from a better source): prompt CG, parts 3 and 5.
- **Command structure and area boundaries:** C01, items 1, 2 and 4. W01 here takes only the directives.
- **Ship classes not on file:** C04, once W03 and W04 show which ships are new to us.

---

## W01. Directives, their dates, their postponements and the plans dropped

**Why:** the game gives a seat a directive from above with a date (`docs/design/one-map-1942.md`, "Directives and their timelines"). The dates, and the plans that were dropped, are facts and need rows. The plans dropped are the branches the game takes when a directive is met or not met.

**Run in two parts:** (1) Allied; (2) Japanese. Split a part at 31 July if it exceeds the cap; May to July first.

```
Task: for SIDE (Allied / Japanese), every directive, order and plan
from above that set a task for forces on the map between 1 April and
30 November 1942. (From 1 April, since the orders for May were given
in April.)

For each, as separate statements with a quote each:
1. Who issued it, to whom, and on what date (directive_issued_on (new),
   issued_by (new), issued_to (new)).
2. The task it gave, in its own words (directive_task (new)), and the
   purpose it stated, where it stated one (directive_purpose (new)).
3. The date it set for the task: a landing day, a day to be ready by,
   a day not later than (target_date (new)). Say which kind it is.
4. The forces it named or allotted (forces_allotted_note (new)), and
   any force it promised on a condition or withheld.
5. Every change of the date: the new date, the date of the change, who
   asked for it, who decided, and the reason given
   (target_date_changed_to (new), postponement_reason_note (new)).
   Include requests for delay that were refused.
6. What it said was to follow: the next phase or task, and on what
   condition (follow_on_note (new)).
7. Plans made and dropped or put off: the plan, its intended dates and
   forces, the date it was dropped and the reason the source gives
   (plan_cancelled_on (new), cancellation_reason_note (new)).
8. Where the source says what the issuing command expected the enemy
   to do, that expectation, as an estimate (rule 5 of the common block).

Part 1, Allied. At least: the Joint Chiefs' directive of 2 July 1942
and its three tasks; the orders to CINCPAC and COMSOPAC for the
Guadalcanal and Tulagi landings and each change of their date; the
orders for the defence of Port Moresby, Milne Bay and Midway;
CINCPAC's operation plans for the Coral Sea and for Midway (check each
plan's number on the document, rule 17); MacArthur's orders for the
advance on Buna; what was ordered for the garrisons of Fiji, Samoa,
New Caledonia, Efate, Espiritu Santo and Tongatabu; and what was
planned and not done.
Part 2, Japanese. At least: the Imperial General Headquarters Navy and
Army orders and the Combined Fleet orders for the Port Moresby
operation, the Midway and Aleutians operations, the Nauru and Ocean
Island operation, the planned operations against Fiji, Samoa and New
Caledonia, the landing at Buna and the overland advance on Port
Moresby, the landing at Milne Bay, and the recapture of Guadalcanal.
Give each operation's name as the document prints it. For the plans
against Fiji, Samoa and New Caledonia give the date ordered, the date
planned, each postponement and the cancellation.

Start with: Morton, Strategy and Command: The First Two Years (US Army
Green Book, HyperWar); Hayes, The History of the Joint Chiefs of Staff
in World War II: The War Against Japan; the CINCPAC Command Summary
(Gray Book), by date; Milner, Victory in Papua (Green Book); Miller,
Guadalcanal: The First Offensive; Reports of General MacArthur, vol. I
and vol. II part I; Japanese Monographs on the navy's and army's
operations in the South Seas and Southeast Area (give each number and
title as printed); Senshi Sosho by volume and page, and its English
translation of the South Pacific Area army volume (Bullard, Japanese
Army Operations in the South Pacific Area, Australian War Memorial,
2007) by page; the USSBS Interrogations of Japanese Officials and The
Campaigns of the Pacific War.
```

## W02. The timetable of each operation, from the order to the landing

**Why:** a directive's date makes a commander plan backwards. The game needs the spans the record allowed (from the order to the sailing, from the sailing to the landing), so that a date counted from the day a directive is issued is one a staff could meet.

**Run in two parts:** (1) Japanese operations; (2) Allied operations.

```
Task: for each operation below, the chain of dated steps from the first
order to the end, each step a statement with a quote. Use the
attributes named, with the operation as the entity (entity kind
"operation") and the force or unit in the note.

Steps wanted, where a source gives the date:
- the decision or order to mount it (directive_issued_on (new));
- the date its plan or operation order was issued (plan_issued_on (new));
- each force's assembly: place and date (assembled_at (new),
  assembled_on (new));
- loading of troops and cargo: place, date begun, date ended
  (loading_began_on (new), loading_ended_on (new));
- any rehearsal: place and dates (rehearsal_note (new));
- each force's sailing: from where, on what date (sailed_on) and its
  planned speed of advance if stated (planned_speed_kn (new));
- fuelling on the way: place or position, date (fuelling_note);
- the day planned for the landing or attack (target_date (new)) and
  the day it happened (landed_on), as separate rows;
- the day the force was to be relieved, withdrawn or back in port, as
  planned and as it happened.
Where the source gives the length of a step in days in its own words
(for example the days allowed for loading), quote it as a statement.
Do not work out a duration yourself (rule 2).

Part 1, Japanese: the Port Moresby operation (the Tulagi force, the
Port Moresby transports, the covering force, the carrier force, from
Truk and Rabaul); the Midway operation (the carrier force, the main
body, the occupation force and its transports from Saipan and Guam,
the submarine line, the flying-boat reconnaissance of Pearl Harbor);
the Nauru and Ocean Island operation; the landing at Buna in July; the
landing at Milne Bay in August; the Ichiki and Kawaguchi movements to
Guadalcanal; the October convoy and the November convoy to
Guadalcanal.
Part 2, Allied: the carrier forces' movements to the Coral Sea and to
Midway (from the first warning to the sailing from Pearl Harbor,
including Yorktown's repair); the reinforcement of Midway in May; the
Guadalcanal and Tulagi landings (from the Joint Chiefs' directive to
the landing: the division's movement to New Zealand, the loading at
Wellington, the rehearsal at Koro, the sailing); the movement of
troops to Port Moresby and Milne Bay from June to September; the
reinforcement of Guadalcanal by the 7th Marines in September and by
the 164th Infantry in October; the advance on Buna in October and
November.

Start with: the sources of W01; the ONI Combat Narratives for the
Coral Sea, Midway and the Solomons; Bates, The Battle of the Coral Sea
and The Battle of Midway (Naval War College analyses); the action
reports and war diaries on HyperWar and history.navy.mil; Hough,
Ludwig and Shaw, Pearl Harbor to Guadalcanal; Zimmerman, The
Guadalcanal Campaign; McCarthy, South-West Pacific Area: First Year
(Australian official history, awm.gov.au) by page; combinedfleet.com
tabular records for Japanese sailing dates, one date per row, each
with a second source where one can be opened.
```

## W03. The position on 1 May 1942

**Why:** the first morning of the game. The battles' own forces are on file; what stood everywhere else on the map is not.

**Run in four parts:** (1) Allied ships; (2) Japanese ships; (3) air units of both sides; (4) land units and garrisons of both sides.

```
Task: where every force on the map stood on 1 May 1942, or on the
nearest date a source gives (say which date).

(1) and (2) Ships: every carrier, battleship, cruiser, seaplane tender
    and oiler, every destroyer division or squadron by its ships, and
    every submarine squadron or division by its boats, that was on the
    map or about to enter it. For each: the port or position on the
    date (movement_note), the task force, fleet or division she served
    in (formation_note), whether she was fit for sea or under repair
    and until when (out_of_action_from, repair_place,
    returned_to_service). For a ship off the map on 1 May that entered
    it before 10 June, where she was and when she sailed. Skip the
    ships of the Coral Sea forces, which are on file, except to give
    the port and date each sailed from.
(3) Air units: each US Navy, Marine and Army squadron, each Australian
    squadron, and each Japanese air group, carrier air group and
    seaplane unit, on the map: based_at, equipped_with,
    aircraft_on_hand, aircraft_operational, pilots_on_hand, each with
    its date. Hawaii, Midway, Johnston, Palmyra, Canton, Samoa, Fiji,
    Tongatabu, New Caledonia, Efate, north-east Australia, Port
    Moresby; Rabaul, Lae, Salamaua, Kavieng, Truk, the Marshalls, the
    Gilberts, Wake.
(4) Land units and garrisons: each place on the map held by either
    side, the units there at regiment, battalion and defence battalion
    or base force level, with strength_men, parent_stated, commander,
    and the date. For the Japanese, the special naval landing forces
    and base forces by number and the army's South Seas Detachment.
    Where the other side made an estimate of a garrison at the time,
    give it as strength_men_estimated_by_enemy.

Start with: the CINCPAC Command Summary (Gray Book) for late April and
early May 1942, which gives dispositions; the Administrative Order of
Battle and war diaries on HyperWar; Bates, The Battle of the Coral Sea
(appendices); Craven and Cate, vol. I and vol. IV, by page (copyrighted:
cite only pages you open); the Army Air Forces Historical Studies on
the Pacific (AFHRA, public domain); Gillison, Royal Australian Air
Force 1939-1942 (awm.gov.au) by page; the US Army Green Books (Morton;
Milner; the volumes on the garrisons); Hough, Ludwig and Shaw for the
Marine defence battalions; Japanese Monographs and Reports of General
MacArthur vol. II for Japanese dispositions; combinedfleet.com tabular
records, one date per row.
```

## W04. What entered and left the map, 1 May to 30 November 1942

**Why:** forces that arrive by date (decision 0195) are how the game keeps what each country sent. The reason a force was sent or taken away is wanted as much as the date: a release tied to a directive rests on it.

**Run in four parts:** (1) Allied warships and oilers; (2) Japanese warships and oilers; (3) air units of both sides; (4) land units and their shipping, both sides. Split each at 31 July if it exceeds the cap.

```
Task: PART of what came onto the map and what left it between 1 May
and 30 November 1942.

(1) and (2) Ships: every carrier, battleship, cruiser, seaplane tender
    and oiler, and destroyers by division, that came onto the map from
    off it (from Japan, the Indian Ocean, the Netherlands East Indies,
    the Aleutians, the Atlantic, the west coast, Australia's south and
    New Zealand) or left it for any of them. For each: the date and
    place she entered (arrived_in_theatre (new)), where she came from,
    the date and place she left (transferred_out_of_theatre (new)) and
    where she went, the force she joined (formation_note), dates out
    of action and back (out_of_action_from, repair_place,
    returned_to_service), ships newly commissioned that reached the
    map in the period (commissioned, arrived_in_theatre (new)), and
    her fate if lost (sunk, fate). And, as a statement of its own, the
    reason the source gives for the move (movement_reason_note (new)):
    for what operation she was sent, at whose request, or what took
    her away. Include what went to the Aleutians in May and June and
    when it came back; what the landings in North Africa took or
    withheld; and Japanese ships sent home for repair or refit.
    The ships at Guadalcanal from 7 August are on file: for those give
    only the day each first came onto the map and the day she left it.
(3) Air units: each squadron, group and Japanese air group that
    arrived on the map, left it, was disbanded or re-formed:
    based_at, equipped_with, aircraft_on_hand, aircraft_received
    (aircraft arriving on a date, by type and number), pilots_on_hand,
    with the dates, and the reason given for the move. Japanese air
    groups were renumbered on 1 November 1942: give the old and new
    number as alias rows with the date. Guadalcanal and Rabaul from
    7 August are on file; give what reached Rabaul and the South
    Pacific from off the map, by date, whether or not it went on to
    Guadalcanal.
(4) Land units and shipping: each division, regiment, brigade and
    separate battalion of either side that arrived on the map or was
    moved between its main bases (not to Guadalcanal, which is on
    file): landed_at, landed_on, landed_men, strength_men, sailed_on,
    embarked_on, carried_by, departed_on, parent_stated, commander.
    Among them: the US divisions and regiments sent to New Caledonia,
    Fiji, Samoa, the New Hebrides and Hawaii; the Australian brigades
    returned from the Middle East and sent to Papua; the Japanese
    17th Army's units as they reached Rabaul, Truk, Palau and the
    Shortlands. And the transports and cargo ships each side had for
    them, where a source counts them by date
    (transports_available (new)).

Start with: Cressman, The Official Chronology of the U.S. Navy in
World War II; DANFS entries, one ship at a time; the CINCPAC Gray
Book; Leighton and Coakley, Global Logistics and Strategy 1940-1943
(Green Book) for what was sent where and why; Morton; Milner; Miller;
Craven and Cate vols. I and IV by page; Reports of General MacArthur;
the Japanese Monographs; Bullard's translation of Senshi Sosho by
page; combinedfleet.com tabular records, one date per row and no more
than 30 statements in a run, each with a second source where one can
be opened.
```

## W05. Papua ashore, 21 July to 30 November 1942

**Why:** the land campaign for Port Moresby is on the map and nothing of it is on file. The fight ashore is built (ground combat, stages 1 to 3, with marching and hunger); it has never been given a track over a mountain range.

**Run in three parts:** (1) the Japanese; (2) the Australians and Americans; (3) the ground, the tracks and how men were supplied on them.

```
Task: PART of the land campaign in Papua from the Japanese landing near
Buna on 21 July 1942 to 30 November 1942: the Kokoda Track, Milne Bay,
and the opening of the fight for Buna, Gona and Sanananda.

(1) and (2) Forces. Each unit at division, brigade, regiment and
    battalion level, with dated rows: strength_men, landed_at,
    landed_on, landed_men, position_on (a named place and a date),
    departed_on, embarked_on, sailed_on, carried_by, parent_stated,
    attached_to, commander, killed, casualties_total,
    sick_men_cumulative, evacuated_men, replacements_men,
    rations_days_on_hand, heavy_weapons_ashore_note, attack_on,
    ceased_on, and the other side's estimate as
    strength_men_estimated_by_enemy. For each named action (Kokoda,
    Isurava, Brigade Hill, Ioribaiwa, Milne Bay, Eora Creek,
    Oivi-Gorari, the first attacks at Buna and Gona): its dates, the
    units engaged, the strength each side had there, the losses, and
    how it ended. The order to the Japanese to fall back from
    Ioribaiwa: its date, who gave it, and the reason stated.
(3) The ground and supply. As statements with quotes:
    - the track from Buna to Port Moresby by stage: the named places
      in order, the distance between them and the marching time a
      source gives for each stage (march_time_note (new),
      track_distance_km (new)), with the date and the unit that made
      the march;
    - what a carrier or a soldier carried and for how many days
      (carrier_load_kg (new), rations_carried_days (new)), how many
      carriers each side had by date (carriers_on_hand (new));
    - supply dropped by air to the Australians: tons by date and
      place where stated (air_supply_tons_delivered (new)), and what
      share was recovered;
    - what reached the Japanese forward troops and what they were
      short of, by date, in the source's words;
    - the airfields built at Milne Bay, Wanigela, Pongani and Dobodura
      and at Kokoda: dates begun and usable (airfield_usable_from (new));
    - sickness by month: malaria and dysentery, men evacuated sick,
      for units of both sides (sick_men_cumulative, evacuated_men).

Start with: McCarthy, South-West Pacific Area: First Year: Kokoda to
Wau (Australian official history, awm.gov.au), by page; Milner,
Victory in Papua (Green Book, HyperWar); Reports of General MacArthur
vols. I and II; Bullard, Japanese Army Operations in the South Pacific
Area (the translation of Senshi Sosho, Australian War Memorial), by
page; the Japanese Monographs on the 18th Army and the South Seas
Detachment (numbers as printed); Gillison for the air supply; the
Australian War Memorial's unit war diaries (AWM52), one statement per
diary page, with the item number; Walker, The Island Campaigns
(Australian medical history) for sickness.
```

## W06. The air war over Papua, the Bismarcks and north-east Australia

**Why:** from May to November the air units at Port Moresby, Lae and Rabaul fought almost daily, and outside Guadalcanal none of it is on file. A side's air strength by base and date is what a staff plans a landing against.

**Run in two parts:** (1) Allied; (2) Japanese. Split at 31 July if over the cap.

```
Task: for SIDE, the air units based at or operating against Port
Moresby, Milne Bay, Lae, Salamaua, Buna, Gasmata, Rabaul, Kavieng,
Horn Island and the fields of north-east Australia, 1 May to
30 November 1942. Guadalcanal's own air war is on file and is not
wanted here.

For each unit, with the date of each row: based_at, equipped_with,
aircraft_on_hand, aircraft_operational, aircraft_received,
pilots_on_hand, formed, in_action_since, and each move of base.
Then, by date:
1. Each raid a source describes: the date, the target, the units, the
   number of aircraft sent by type, the losses of the side flying it
   from its own records, and what it claimed (rule 4: a claim is a
   claim; give it as "claimed_..." and say whose).
2. Aircraft destroyed on the ground, by date and place, from the
   loser's records.
3. The state of each field where a source gives it: runways usable,
   what types could use it, from what date (airfield_usable_from (new)).
4. Monthly or weekly strength returns where a source prints one: give
   where the table is (document, page, URL, its columns), and no more
   than ten rows from it (item 5 of "How to run them" in prompts.md).
5. Replacement aircraft and pilots reaching the units, by date.

Part 1, Allied: the Fifth Air Force (from September) and Allied Air
Forces before it; the US fighter and bomber groups by squadron; the
RAAF squadrons at Port Moresby and Milne Bay.
Part 2, Japanese: the 25th Air Flotilla and the units under it (the
Tainan, 4th and Yokohama air groups and those that relieved or joined
them), the detachments at Lae and Buna, and what the 11th Air Fleet
sent to Rabaul.

Start with: Craven and Cate vols. I and IV by page (copyrighted: only
pages you open); the Army Air Forces Historical Studies (AFHRA) on the
air war in the South West Pacific; Gillison, Royal Australian Air
Force 1939-1942 (awm.gov.au) by page; Kenney's reports as printed in
Reports of General MacArthur; the Japanese Monographs on Southeast
Area naval air operations; JACAR unit action reports (kodochosho) by
reference code and image number; Senshi Sosho by volume and page;
j-aircraft.com and pacificwrecks.com only as leads, one fact at a time.
```

## W07. Places and bases of the map

**Why:** the map of `docs/design/one-map-1942.md` is three times as wide and as tall as the present one, and the places of the Central Pacific and the islands between are not on file. Prompts C13 and C22 of `prompts.md` ask this of the whole ocean and the whole war; this asks it of this map in 1942, and should be run in their place for these regions.

**Run in three parts:** (1) the Central Pacific: the Hawaiian Islands, Midway, Johnston, Palmyra, Canton, Wake, the Marshalls, the Gilberts, Nauru and Ocean Island, the eastern Carolines with Truk, and Saipan and Guam where a source says a force sailed from them; (2) the South Pacific: Samoa, Fiji, Tonga, New Caledonia, the New Hebrides, the Santa Cruz Islands, the Ellice Islands; (3) the South West Pacific: north-east Australia from Brisbane to Horn Island, Papua, north-east New Guinea, New Britain, New Ireland, the Admiralties, Bougainville and the Shortlands.

```
Task: for the REGION of this part, every place a force was based at,
anchored at, landed at or flew from in 1942.

For each place, as statements with quotes:
1. Its position as a source prints it, in degrees and minutes
   (position_note (new); give the printed latitude and longitude in
   "as printed"). One source's position is one row; a second source's
   is a second row.
2. Who held it on 1 May 1942, and each change of holder with its date.
3. Airfields and seaplane bases: the date each was begun, first usable
   and usable by bombers; runway length and surface as stated; what
   types flew from it in 1942 (airfield_usable_from (new),
   runway_length_m (new), airfield_note (new)).
4. The port or anchorage: whether a fleet could anchor, depth and
   berths where stated, tons a day it could unload in 1942 where a
   source gives a figure (port_unloading_tons_per_day (new)), whether
   it had fuel storage and how much (fuel_storage_bbl (new)), repair
   ships, floating docks or yards and what they could take.
5. Its garrison in 1942 by date (strength_men, with the unit), and
   coast and anti-aircraft guns where stated.
6. For atolls and small islands: whether a landing over the reef was
   judged possible, in the source's words.
Where a source gives a table of distances between places, say where
the table is and give no more than ten of its rows.

Start with: Building the Navy's Bases in World War II, vol. II
(Bureau of Yards and Docks, 1947, HyperWar), by page; the US Army
Green Books on the garrisons and on the Corps of Engineers in the war
against Japan; Hough, Ludwig and Shaw for the Marine garrisons; the
Sailing Directions and Pilots of the period (US Hydrographic Office,
archive.org) for anchorages and positions; the Joint Army-Navy
Intelligence Studies (JANIS) and the wartime Allied Geographical
Section terrain studies where they can be opened; the US Naval
Technical Mission to Japan and USSBS reports on Truk, Rabaul and the
Marshalls (The Reduction of Truk; The Allied Campaign Against Rabaul;
The American Campaign Against Wotje, Maloelap, Mille and Jaluit);
Japanese Monographs on the base forces; the Australian official
histories for the bases of north-east Australia and Papua.
```

## W08. Oilers, fuel at the main bases, and passages made

**Why:** over this map a force without an oiler goes nowhere. Fuelling at sea is built (decision 0201) and real oilers are on file only for the Coral Sea and for five days of August. And the designer has noted that a convoy made good a half to three quarters of its stated speed (`docs/open-work.md`, "Speed made good"); a wide map makes that matter.

**Run in two parts:** (1) Allied; (2) Japanese.

```
Task: for SIDE, fuel and movement over the map, 1 May to
30 November 1942.

1. Fleet oilers: each oiler that served on the map in the period, with
   the date and place of each fuelling a source records, the force she
   fuelled, the oil she gave where stated (fuelling_note,
   cargo_oil_load_note), the force she was assigned to by date
   (formation_note), and her loss or damage (sunk, fate, damage_note).
   The oilers of the Coral Sea, and the American oilers of
   23 to 27 August, are on file.
2. How fuelling at sea was done in 1942, as the side's own documents
   say: the speed, the number of ships at a time, by day or by night,
   the weather it could be done in, how long a carrier or a destroyer
   took (fuelling_note, with the document's date). For the Japanese
   especially: which ships could fuel others, and how.
3. Fuel at the main bases by date, where a source gives a figure:
   Pearl Harbor, Noumea, Tongatabu, Suva, Samoa, Efate, Espiritu
   Santo, Brisbane, Townsville; Truk, Rabaul, Kwajalein, Saipan
   (fuel_on_hand_bbl (new), fuel_storage_bbl (new)), and any statement
   that a shortage of oil or of oilers held a force back, quoted.
4. Passages made. For each passage a source dates at both ends: the
   force, the port left and the date and hour, the port or position
   reached and the date and hour, and the speed of advance if the
   source states one (movement_note; planned_speed_kn (new)). Do not
   work out a speed or a distance yourself (rule 2). Wanted above all:
   carrier forces between Pearl Harbor and the Coral Sea, Tongatabu
   and Noumea; transports from the west coast, Hawaii, New Zealand and
   Australia to the South Pacific; the Japanese forces from the Inland
   Sea, Saipan, Guam and Truk to Midway, the Coral Sea and Rabaul.
5. What a force burned: any statement of oil used by a carrier force
   in an operation, or a day's steaming, in the source's figures
   (fuel_rate_note).

Start with: Carter, Beans, Bullets, and Black Oil (US Navy, 1953,
HyperWar); Wildenberg, Gray Steel and Black Oil (copyrighted: only
pages you open); the CINCPAC Gray Book for oiler movements and fuel
states; the war diaries of the oilers and of the Service Force where
they can be opened; DANFS entries for each oiler; Ballantine, U.S.
Naval Logistics in the Second World War; the US Naval Technical
Mission to Japan reports on fuelling at sea and on fuel; the USSBS
report Oil in Japan's War; the Japanese Monographs; combinedfleet.com
tabular records for the Japanese oilers, one date per row.
```

## W09. What each side read, knew and believed, by date

**Why:** Midway was an ambush because one side read the other's signals. On a wide map with a free enemy the game needs a first form of intelligence (`docs/design/one-map-1942.md`, "Intelligence"; `docs/design/intelligence.md`), and it must rest on what each command actually believed on each date, which decision 0173 already owes for the 39 days. Prompt C19 of `prompts.md` asks this of the whole war; this is its part for these months, in more depth.

**Run in two parts:** (1) Allied; (2) Japanese. Split at 31 July if over the cap.

```
Task: for SIDE, what the command believed of the enemy, by date, from
1 April to 30 November 1942, and how it knew.

1. Before each enemy operation (the Port Moresby operation, Midway and
   the Aleutians, the Buna landing, Milne Bay, each attempt to retake
   Guadalcanal; and for the Japanese, the Guadalcanal landing, the
   reinforcement of Port Moresby and Milne Bay, the carrier raids):
   each estimate the command made of the enemy's objective, date and
   forces, with the date of the estimate, who made it and the source
   it rested on where stated (estimate_of_enemy_note (new), with the
   estimate's own date as valid from). One estimate, one row. Give the
   later estimates too, so that the change over the weeks can be seen.
2. How far ahead it knew: the date on which the command first named
   the objective, and the date it first named the day, in the source's
   words.
3. What it believed of the enemy's carriers and where they were, on
   each date a source gives: which it thought sunk, damaged, in home
   waters, at sea (believed_position_note (new),
   believed_sunk_note (new)). Among them: what the Japanese believed
   of the American carriers after the Coral Sea and before Midway, and
   what each side believed it had sunk in each carrier battle.
4. Its standing sources and when each was working: for the Allies,
   which Japanese codes were being read, how much, and each change of
   code or call signs that cut it off, with dates; traffic analysis
   and direction finding; coastwatchers by place; submarine and air
   reconnaissance of Truk and Rabaul. For the Japanese: radio
   intelligence, submarine scouting lines and their dates on station,
   flying-boat and submarine-borne reconnaissance of Pearl Harbor and
   the South Pacific bases, and what each found or failed to find.
5. Estimates of the enemy's garrisons and air strength at named places
   by date, as strength_men_estimated_by_enemy and
   aircraft_estimated_by_enemy (new), where W03 and W06 do not have
   them.
6. Each occasion a source says the command was wrong about the enemy,
   and what it then did, quoted.

Start with: the CINCPAC Command Summary (Gray Book), whose running
estimates are dated; the CINCPAC Fleet Intelligence summaries and the
Combat Intelligence Unit's records where NHHC or NSA has released
them; the NSA Center for Cryptologic History monographs (Parker, A
Priceless Advantage; and the others on the Pacific, nsa.gov, US
government works); Bates's Naval War College analyses, which set out
what each commander knew hour by hour; Reports of General MacArthur
and the Allied Translator and Interpreter Section's reports; Feldt,
The Coast Watchers (copyrighted: only pages you open); the USSBS
Interrogations of Japanese Officials; the Japanese Monographs; Senshi
Sosho by volume and page; Bullard's translation by page.
```

## W10. Carrier aircrew through 1942

**Why:** the theme of the scenario is that a deck or an air group lost in May is one a side lacks in October. Experience for air groups waits on research (decision 0064): worked by hand from the rows on file, the facts were too thin. Prompt C08 of `prompts.md` asks training of the whole war; this asks the carrier groups of 1942 in depth.

**Run in two parts:** (1) American; (2) Japanese.

```
Task: for SIDE, the aircrew of each carrier air group and squadron
that fought between 1 May and 30 November 1942, followed through the
period.

For each squadron (US: VF, VB, VS, VT by number and ship; Japanese:
each carrier's fighter, dive bomber and attack units), with a date on
every row:
1. Pilots on its roster before and after each battle (pilots_on_hand)
   and aircraft (aircraft_on_hand, aircraft_operational).
2. Aircrew lost in each battle from the side's own records: killed,
   missing, recovered (give each as its own row; use the names in
   data/oob/aircrew_fates.csv for the kinds if you can read it).
3. Replacements joined: how many, on what date, from where, and what
   a source says of their training (aircraft_received for aircraft;
   pilots_joined (new) for men; pilot_experience_note).
4. Flying hours: the hours a source gives for the squadron's pilots,
   or for new pilots joining it, with the date (pilot_flying_hours).
   The syllabus hours of each navy's training in 1941 and 1942 as the
   navy's own documents give them.
5. The share of the squadron's pilots who had fought before, where a
   source states it (veteran_share_percent, prior_combat_note). Do
   not work a share out from a roster yourself (rule 2); give the
   roster facts and leave the arithmetic to us.
6. Squadrons moved between ships or ashore, merged, or re-formed:
   the date and what the source says became of the men (for example
   Yorktown's and Lexington's squadrons after their ships were lost;
   the Japanese carrier groups re-formed after Midway; carrier groups
   sent ashore to Guadalcanal, Buin or Rabaul).
7. Any statement by a commander of the time on the quality of his
   aircrew or of replacements, quoted, with its date
   (pilot_experience_note).

Start with: the squadrons' and air groups' action reports and war
diaries on HyperWar and history.navy.mil; the ONI Combat Narratives;
Bates's analyses; the Bureau of Aeronautics' and the Naval Aviation
History Office's training histories (Aviation Training 1911-1939 and
its successors, US government works); the USSBS reports on Japanese
air power and its Interrogations of Japanese Officials; the Japanese
Monographs on naval air training; JACAR carrier and air group action
reports (kodochosho) by reference code and image number; Senshi Sosho
by volume and page. Lundstrom's The First Team volumes, Parshall and
Tully's Shattered Sword, and Hata, Izawa and Shores are copyrighted:
cite only pages you open, and never a page from memory.
```
