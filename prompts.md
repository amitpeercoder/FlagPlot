# Research prompts: the Pacific War campaign, 7 December 1941 to 15 August 1945

Written on Nov 25, 2026 at the designer's request, to open research for the whole campaign. They are for an outside model (ChatGPT / Codex). Nothing here is settled and nothing here is a plan of the game: they gather sourced statements, and our readers check every statement against its source before a row is keyed into `data/oob`, as the passes of Nov 13 and Nov 14 did (`docs/research/oob/outside-research-nov-13.md`, `outside-research-nov-14.md`).

The end date: the user asked for "13 August 1945". Hostilities ended on 15 August 1945 (the Emperor's broadcast) and the surrender was signed on 2 September. The prompts run to 15 August; a scenario can end earlier. Change it in the common block if another date is meant.

## How to run them

1. **Paste the common block below, then one prompt.** Each prompt is written to stand alone with the common block above it. Run one prompt per conversation; the large ones say how to split them into parts, one part per run.
2. **Save each answer as its own file**, named by the prompt and part (`C05-part2-1943.md`). Keep the answers outside the repository, as before; our readers key rows from them.
3. **Order.** C00 and C01 first: they give the skeleton the others hang on. Then the forces (C02 to C08), then logistics and economy (C09 to C13), then the war on shipping and the battles (C14 to C17), then the rest. Within a group, any order.
4. **What the earlier passes taught** (Nov 13 and Nov 14): no quote was invented, but about one claim in ten was misattributed (right words, wrong book, chapter, ship, date or author); twice the model's own arithmetic was presented as the source's; its "not found" answers were the weakest part, and its searches were shallow. The common block is written against each of these.

## Prompt index

| Id | Subject | Parts | What the game would use it for |
| --- | --- | --- | --- |
| C00 | Chronology of operations and the scope of the theatre | 4 (by year) | The skeleton: which operations, where, when; the map's extent |
| C01 | Command, areas and directives by date | 2 (Allied, Japanese) | Command levels, area boundaries, the directives from above (decision 0157) |
| C02 | Order of battle on 7 December 1941 | 4 | The opening position |
| C03 | Warships entering, leaving and returning to the theatre | 8 (by navy and type) | Forces that arrive by date: the first need of any larger scenario |
| C04 | Ship classes new to the record, 1942 to 1945 | 3 | Class facts for ships the derivation has never seen |
| C05 | Radar and anti-aircraft refits by date | 3 | Fits that change over the war; a fit needs a dated source |
| C06 | Aircraft types and their service dates | 2 | Type facts and when each reached the theatre |
| C07 | Air units by date | 6 (by service and year) | Where each air unit was, on what, at what strength |
| C08 | Aircrew training and experience by year | 2 | Experience for air groups (decision 0064 waits on this) |
| C09 | Aircraft production and delivery to the theatre | 2 | The rate at which aircraft arrive |
| C10 | Japanese merchant shipping and its losses | 2 | The tonnage pool and the war on it (economy answer 4) |
| C11 | Oil, metals and general cargo: Japan's flows | 3 | The three materials (economy answer 1) |
| C12 | The Allied line of supply and the fleet train | 2 | Supply from off the map; replenishment at sea |
| C13 | Bases and their facilities by date | 6 (by region) | What each base could operate, repair and store, and when |
| C14 | The submarine war, both sides, and anti-submarine war | 3 | Submarines (decision 0102 places them before a long campaign) |
| C15 | Mines and the mining campaign | 1 | Denial of routes and ports |
| C16 | Land campaigns: forces and strengths by date | 7 (by campaign) | Ground orders of battle beyond Guadalcanal |
| C17 | Engagements to test against, 1941 to 1945 | 5 (by period) | History to compare a result with, never to script it |
| C18 | Losses: ships and aircraft by date and cause | 2 | Truth for validation; claims kept apart as belief |
| C19 | Intelligence: what each side read and knew, by date | 2 | The intelligence design note's standing sources |
| C20 | Special attack (kamikaze) and late-war air defence | 1 | The rule that changes the air war from October 1944 |
| C21 | Weather and seasons | 2 | Typhoons, monsoon, visibility by region and month |
| C22 | Places and positions for a map of the whole Pacific | 3 (by region) | The map scheme decision 0017 says the whole Pacific needs |

Open questions for the designer before some of these run: whether Burma, India and China are in the campaign or only its edge (C00 asks the record, it does not decide); whether the strategic bombing of Japan is in (C11 asks only for its effect on the flows); and whether the Aleutians are a scenario or only a theatre that ties forces down.

---

## The common block (paste above every prompt)

```
You are a research assistant gathering sourced historical statements for a
wargame of the Pacific War, 7 December 1941 to 15 August 1945. Your answer is
checked statement by statement by a reader who opens each source you cite and
searches it for the words you quote. A statement that cannot be found in the
source you name is thrown away, so precision matters more than coverage.

WHAT TO DELIVER

One Markdown file with four tables, in this order.

Table 1. Statements. One row is one source's statement of one value.
Columns:
  n | entity | entity kind | attribute | value | unit | as printed |
  valid from | valid to | source | locator | opened on | quote |
  source kind | note

- entity: the ship, class, aircraft type, unit, base, place, operation or
  person, named as the source names it, with hull number or unit
  designation where there is one (e.g. "Essex (CV-9)", "Zuikaku",
  "201st Kokutai", "VF-2", "Truk (Chuuk), Dublon anchorage").
- entity kind: ship | ship class | aircraft type | air unit | land unit |
  base | place | weapon | sensor | operation | flow | command | other.
- attribute: use the names given in the prompt where it gives them;
  otherwise a short snake_case name with the unit as a suffix
  (max_speed_kn, aircraft_on_hand, oil_imports_kl_per_month). Mark a new
  name with "(new)".
- value: one value only. Convert to the unit named and keep the source's
  own figure in "as printed" (e.g. value 9144, as printed "30,000 ft").
  Never put a range or two values in one row: two values are two rows.
- valid from / valid to: dates in YYYY-MM-DD. If the source gives only a
  month or a year, write YYYY-MM or YYYY; never invent a day. "True on
  this date, limits unknown" is both columns set to that date, with a note.
- source: author, title, edition or year, publisher. The thing you
  actually opened. If a web page quotes a book you did not open, the
  source is the web page and the note says "citing <book, page>". Never
  cite a book's page you did not see.
- locator: page, table, chapter or section AND the full URL of the
  document you opened. For a scanned PDF give the printed page number and
  the PDF page number. HyperWar pages: give the chapter URL and the
  printed page the passage falls on (HyperWar's page markers close the
  page they follow; check you have the right one).
- opened on: the date you read it.
- quote: the sentence or table cell that carries the value, verbatim, at
  most 40 words. It must be findable in the document by text search. For
  a table, quote the row label and the cell.
- source kind: primary (action report, war diary, contemporary manual or
  return) | official history | postwar study (USSBS, JANAC, Japanese
  Monographs, Technical Mission) | scholarly | reference book |
  web compilation | tertiary.
- note: anything the reader needs: a conversion, which of several figures
  in the passage you took, a time zone, the fact that the date is the
  report's date and not the event's.

Table 2. Disagreements. Where two sources give different values for the
same thing, list both statement numbers and say in one line how they
differ. Do not choose between them. Both stay in Table 1.

Table 3. Looked for and not found. Each row: what was sought, for which
entity and period, every source you searched (with URL), and whether you
searched the full text or only a summary or index. "Not found" is a claim
and is checked like any other, so only write it after reading the source
where it would be. If you ran out of time or the source would not open,
say that instead: "not reached" is not "not found".

Table 4. Sources. One row per document opened: full citation, URL, kind,
whether it is a US government work, and in one line its known weakness
(written from memory, wartime estimate of the enemy, a translation, a
compilation without notes).

RULES

1. Quote, do not paraphrase. Every value comes from a quoted passage.
2. No arithmetic of your own in a value. If you sum, divide or convert
   anything beyond a unit conversion, put the result in the note, marked
   "my arithmetic", and leave the value to what the source prints.
3. One fact per row. A source that states five ships' dates gives five rows.
4. Claims are not losses. A side's claim of enemy aircraft shot down or
   ships sunk is a claim: give it attribute "claimed_..." and say whose
   claim it is. Only the loser's records, or a postwar study built on
   them (JANAC, Japanese records, USSBS), establish a loss.
5. Each side is the authority on itself. A US wartime figure for Japanese
   strength is an estimate (attribute "estimated_..." or note
   "US estimate"), and the same the other way round. Keep estimates:
   they matter to the game, as what a side believed.
6. Nominal is not achieved. A manual's speed, range or rate of fire is
   the stated figure; say "nominal" in the note. Figures achieved in
   service are separate rows.
7. A class fit is not a ship's fit on a date. That a class received radar
   does not show a named ship had it on a named day. Give a ship-level
   date only where the source gives it for that ship.
8. Never use any wargame or its data: no War in the Pacific (Admiral's
   Edition), no Matrix/Slitherine forums, no mods, no scenario files, no
   board-game tables, no ratings from any game. If a page's figures come
   from a game, leave it.
9. Wikipedia and similar summaries are leads only. Follow the reference
   they cite, open it, and cite that. If the cited source cannot be
   opened, you may give the Wikipedia statement with source kind
   "tertiary", the article URL and revision date, and the note "citing
   <reference>". Never copy a table from it.
10. Forums, model-kit sites, YouTube, AI-generated pages and quiz sites
    are never sources.
11. Prefer, in this order: (a) primary records of the side concerned and
    US government works (action reports, war diaries, CINCPAC Gray Book,
    ONI Combat Narratives, Naval War College analyses, the US Strategic
    Bombing Survey, JANAC, the Japanese Monographs, the US Naval Technical
    Mission to Japan reports, the US Army "Green Books", the USMC
    historical monographs, Building the Navy's Bases in World War II,
    Craven and Cate, Army Air Forces Statistical Digest), read on
    HyperWar (ibiblio.org/hyperwar), history.navy.mil, archive.org, DTIC,
    the Combined Arms Research Library, or a university library; (b) for
    Japanese facts, Japanese records and studies built on them: JACAR
    (with its reference code), Senshi Sosho (volume and page),
    combinedfleet.com's tabular records (one date at a time, never a
    whole record copied); (c) scholarly works and reference books, cited
    only by a page you actually read; (d) careful web compilations
    (navweaps, navsource, uboat.net, j-aircraft). Use (d) where (a) to (c)
    cannot be opened, and say so.
12. Two copies are one source. Two sites repeating the same book do not
    corroborate each other; say in the note when you see that.
13. Time zones: say which zone a time is in if the source says, and say
    "zone not stated" if it does not. Japanese records keep Tokyo time.
14. Search deep, not wide. Open the full document and read around the
    passage. A search-engine snippet is not a reading.
15. Write in plain English. No copied paragraphs: one quote per row, at
    most 40 words.
16. If a question cannot be answered from sources you can open, say so in
    Table 3. An honest gap is worth more than a filled cell.

Our data already holds facts for May to November 1942 in the South
Pacific (Coral Sea, Midway, Savo Island, the Eastern Solomons, Cape
Esperance, Santa Cruz, the November night actions, Tassafaronga, the
Guadalcanal air campaign of August and September 1942). Where a prompt
says "not needed", skip it.
```

---

## C00. Chronology of operations and the scope of the theatre

**Why:** every later prompt hangs on a dated skeleton. A campaign game needs to know which operations happened, in which area, on which dates, with which forces in outline, and where the theatre's edges lay.

**Run in four parts:** Dec 1941 to Dec 1942; 1943; 1944; Jan to Aug 1945.

```
Task: build a dated chronology of the operations of the Pacific War in the
period of this part (PERIOD: ____ ), from sources.

1. Every named operation, landing, battle, raid, evacuation and major
   reinforcement run, at sea, in the air and ashore, in the Pacific Ocean
   Areas, the South West Pacific Area, the Netherlands East Indies,
   Malaya, the Philippines, the Aleutians, the Indian Ocean east of
   Ceylon, and the waters around Japan. Attributes: operation_name (both
   sides' code names where the record gives them, as separate rows),
   date_from, date_to, area, objective_place, side_attacking.
2. For each, the principal forces in outline as the source states them:
   which fleets, task forces, divisions, air forces. Not ship lists here.
3. Where a source says what the operation was meant to achieve, the
   stated objective, as a statement with a quote.
4. Where a planned operation was cancelled or postponed (for example
   Japanese plans against Fiji, New Caledonia and Samoa; Allied plans
   bypassed such as Rabaul's capture), the plan, its dates and the date
   and reason it was dropped.
5. Burma, India and China: list only operations that used ships or
   naval air, or that a source says drew forces away from the Pacific.
   Do not decide whether they belong to the theatre; give what the
   sources say of the forces they took.
6. For this period, the edges of the war at sea: the farthest positions
   reached by each side's surface forces, carriers and submarines that a
   source states, with the place and date.

Order Table 1 by date. A statement is still one source's statement:
do not merge two sources into one row.
Start with: Morison's volumes are copyrighted, use only pages you open;
The Campaigns of the Pacific War (USSBS, 1946); the Japanese Monographs
index; Cressman, The Official Chronology of the U.S. Navy in World War II
(NHHC, on HyperWar); the US Army in WWII chronology volume (Williams,
Chronology 1941-1945).
```

## C01. Command, areas and directives by date

**Why:** command levels are built (decision 0132); a campaign needs the real commands, their boundaries and the directives each received, which are what decision 0157 gives a player in place of a briefing.

**Run in two parts:** Allied; Japanese.

```
Task: for SIDE (Allied / Japanese), the command structure in the Pacific
from 7 December 1941 to 15 August 1945.

1. Each command at theatre, area, fleet and force level, with its
   commander, date of taking command and date of leaving it. Allied:
   ABDA, the Pacific Ocean Areas and its North, Central and South
   Pacific Areas, the South West Pacific Area, Pacific Fleet and its
   numbered fleets and type commands, Allied Naval and Air Forces SWPA,
   the British Eastern and Pacific Fleets. Japanese: Imperial General
   Headquarters, the Combined Fleet and each numbered fleet, the area
   fleets (Southwest, Southeast, Central Pacific), the base air forces
   (11th Air Fleet and its successors), the army's area armies and
   armies in the theatre.
2. Area boundaries by date: the lines given in the directive that drew
   them (for example the Joint Chiefs' directive of 30 March 1942 for
   SWPA and POA, and each change after), as quoted coordinates.
3. The directives received from above: the date, who issued it, to whom,
   and the task it gave, quoted. Allied: Joint Chiefs and CINCPAC
   directives for each phase. Japanese: Imperial General Headquarters
   Navy Orders (Dai-kairei) and Army Orders for each phase, the Absolute
   National Defence Sphere of September 1943, the Sho-Go and Ketsu-Go
   plans. Where only a summary is available, quote the summary and say so.
4. For each change of commander at fleet level and above, the reason the
   source gives, if it gives one.
Start with: the CINCPAC Command Summary (Gray Book) and Morton, Strategy
and Command: The First Two Years (US Army Green Book, HyperWar); Hayes,
The History of the Joint Chiefs of Staff in World War II: The War Against
Japan; the Japanese Monographs on navy and army operations (list which
monograph numbers you read).
```

## C02. Order of battle on 7 December 1941

**Why:** the opening position of the campaign. Our file has almost nothing before May 1942.

**Run in four parts:** (a) Japanese navy, ships; (b) Allied navies, ships; (c) both sides' air, by unit; (d) land garrisons and forces at the points of attack.

```
Task: PART (a / b / c / d) of the order of battle on 7 December 1941
(8 December west of the date line; say which date each source uses).

(a) Japanese navy: the organisation of the Combined Fleet and every
    other fleet: fleet, squadron (sentai), destroyer squadron and
    division, with every ship by name, where each was on 7 December and
    which operation it was assigned to (Hawaii, Malaya, Philippines,
    Guam, Wake, Hong Kong, the Indies, home waters). Attributes:
    assigned_to (unit), location_on_date, operation_assigned.
(b) Allied navies: the US Pacific Fleet (with ships at Pearl Harbor,
    at sea and on the West Coast), the US Asiatic Fleet, the Royal Navy's
    Force Z and China Station, the Royal Netherlands Navy in the Indies,
    the Royal Australian Navy and Royal New Zealand Navy ships in or
    able to reach the theatre. Same attributes.
(c) Air: every air unit in the theatre on both sides: unit, parent,
    base, aircraft type, aircraft on hand and operational, pilots on hand
    where given. Japanese navy kokutai and carrier air groups, Japanese
    army air regiments (sentai) in the southern operations, US Navy and
    Marine squadrons, US Army Air Forces in Hawaii and the Philippines
    (FEAF), RAF and RAAF in Malaya, Dutch army and navy air in the Indies.
    Use attributes equipped_with, based_at, aircraft_on_hand,
    aircraft_operational, pilots_on_hand.
(d) Land: the garrisons at Pearl Harbor, the Philippines, Guam, Wake,
    Hong Kong, Malaya, Singapore, the Indies, Rabaul, and the Japanese
    landing forces for each first-wave operation: unit, parent, strength
    in men by date, location.
Start with: the Pearl Harbor attack hearings exhibits (HyperWar), Morton,
The Fall of the Philippines; Craven and Cate vol. I; the USSBS
Campaigns of the Pacific War appendix of Japanese forces; Japanese
Monographs nos. 1 to 4 and the Southern Area operations monographs;
Kirby, The War Against Japan vol. I (UK official history; check its
rights and cite pages only).
```

## C03. Warships entering, leaving and returning to the theatre

**Why:** "forces that arrive by date" is the first rule any larger scenario needs (`docs/design/larger-scenarios.md`). The date that matters is not commissioning but the date a ship reached the theatre, left it for repair, and came back.

**Run in eight parts:** US carriers and battleships; US cruisers; US destroyers (by year); US destroyer escorts and patrol craft; US auxiliaries of the fleet train (oilers, ammunition ships, repair ships, tenders); Japanese carriers, battleships and cruisers; Japanese destroyers and escorts (kaibokan); British, Dutch and Commonwealth ships. Split a part further by year if it does not fit.

```
Task: for every ship of PART, her movements into and out of the Pacific
theatre from 7 December 1941 to 15 August 1945.

Attributes (one row each, dated):
  commissioned, arrived_in_theatre (first arrival at Pearl Harbor, or at
  a base west of it, or for Japanese ships first deployment out of home
  waters), out_of_action_from, repair_place, returned_to_service,
  last_refit_completed, transferred_out_of_theatre, sunk, fate.
For out_of_action_from, give the cause (the action or accident) in the
note.

Do not list every voyage. Only the dates above.
Not needed: ships sunk before 1 May 1942 that are already well known are
still needed; ships of the South Pacific May to November 1942 are only
needed for dates outside that window.
Start with: DANFS (history.navy.mil, one entry per ship; cite the entry
URL); combinedfleet.com tabular records of movement (one date per row,
never the whole record); NHHC War Damage Reports for repair periods;
uboat.net Allied Warships for British, Dutch and Commonwealth ships.
Prefer a war diary or a yard's record for a repair period over DANFS
where you can open one.
```

## C04. Ship classes new to the record, 1942 to 1945

**Why:** the derivation (`oob-derive`) can only rate a class it has rows for. Most 1943 to 1945 classes have none.

**Run in three parts:** US (Essex, Independence, Iowa, South Dakota and North Carolina where missing, Baltimore, Cleveland, Atlanta, Fletcher, Sumner, Gearing, the destroyer escorts, Casablanca, Bogue and Sangamon escort carriers, Gato, Balao and Tench submarines); Japanese (Taiho, Unryu, Shinano, the converted carriers Hiyo, Junyo, Chitose, Chiyoda, Zuiho, Ryuho, Shinyo, Akizuki, Shimakaze, Yugumo, Matsu, Tachibana, the kaibokan classes, the I-400 and later submarines); British Pacific Fleet (Illustrious and Implacable classes, King George V).

```
Task: for each ship class of PART, the class facts below, each from a
source, one row per value.

Attributes: displacement_standard_lt, displacement_full_lt,
length_overall_m, beam_m, max_speed_kn, shaft_power_shp, endurance_nm,
endurance_speed_kn, fuel_capacity_lt (or fuel_capacity_bbl where the
source gives barrels), belt_armour_max_mm, deck_armour_max_mm,
complement, aircraft_capacity, torpedo_tubes, torpedoes_carried,
torpedo_reloads, main_rounds_per_gun; for submarines also
surfaced_displacement_lt, submerged_displacement_lt,
submerged_max_speed_kn, diving_depth_m, torpedo_tubes_bow,
torpedo_tubes_stern. As-built armament as separate rows: each mount type
and number, with the date the source gives (armament_note if a list).
Speeds and endurance are nominal unless the source says trials or
service; say which.
Start with: the US Navy's Ships' Data, U.S. Naval Vessels (1 April 1945
and earlier editions; archive.org / NHHC), the US Naval Technical
Mission to Japan reports on Japanese ships (series S), navweaps class
pages for guns; combinedfleet class pages.
```

## C05. Radar and anti-aircraft refits by date

**Why:** what a ship carried changed through the war, and the derivation reads a fit only on its date. A class fit is not a ship's fit (common rule 7).

**Run in three parts:** US radar (SC, SK, SG, SP, SM, Mk 4, Mk 12/22, Mk 8, Mk 13) and the 40 mm and 20 mm programme; Japanese radar (Type 21, 22, 13) and the 25 mm additions of 1943 and 1944; the proximity (VT) fuze and the Mk 37 and Mk 51 directors.

```
Task: for PART, when each fit reached ships in the Pacific.

1. For each equipment: date of first fit at sea, date fitted to each
   named ship where a source gives it (attribute fitted_on with the ship
   as entity and the equipment in the value), and the dates of any
   programme ("all destroyers to receive X by ...") quoted as the
   programme.
2. For anti-aircraft: the number of each gun type on named ships on
   named dates, from a dated source (a photograph's caption with date
   counts only if the source dates it).
3. The proximity fuze: date first issued to the Pacific Fleet, first
   use in action, the share of 5-inch anti-aircraft rounds that were VT
   by period where a source gives it.
4. Japanese radar: date first fitted, which ships by date, and stated
   detection ranges, from the US Naval Technical Mission reports.
Start with: FTP 217 (1943) and FTP 220; the BuOrd and BuShips records on
HyperWar; Friedman's books only by pages read; the US Naval Technical
Mission to Japan reports E-01 and E-02 (radar) and O-series (ordnance);
combinedfleet.com ship pages for dated Japanese refits; Baldwin, The
Deadly Fuze, by page read; NHHC pages on the VT fuze.
```

## C06. Aircraft types and their service dates

**Why:** aircraft that arrive later in the war (F6F, F4U, TBF, SB2C, P-38, B-24 in the Pacific, B-29 later, A6M5, N1K, Ki-84, D4Y, B6N, P1Y) have no rows.

**Run in two parts:** Allied types; Japanese types.

```
Task: for each aircraft type and variant of PART that served in the
Pacific from December 1941 to August 1945:

Attributes: max_speed_kn, max_speed_altitude_m, cruise_speed_kn,
range_normal_nm, range_max_nm, combat_radius_nm, service_ceiling_m,
engine, engine_power_hp, empty_weight_kg, loaded_weight_kg,
max_bomb_load_kg, internal_fuel_l, drop_tank_l, crew, pilot_armour,
self_sealing_tanks, carrier_capable, first_service (date the first unit
received it), first_in_action_pacific (new: date and place of first
combat in the theatre).
Give the loading condition each performance figure was measured at.
For Japanese types give both the Japanese figures (from the Technical
Mission or Japanese records) and the US wartime estimate (TAIC), as
separate rows: the estimate is what the Allies believed.
Start with: Standard Aircraft Characteristics sheets (US Navy, Bureau of
Aeronautics) and the AAF characteristics summaries, on archive.org or
the Navy's sites; TAIC Manuals 1 and 2 and the Japanese Aircraft
Performance and Characteristics (TAIC, 1945); USSBS Japanese Air Power;
the Technical Mission's aircraft reports.
```

## C07. Air units by date

**Why:** where each air unit was, on what aircraft and at what strength, is what the game's air groups are. Our file holds August and September 1942 at Rabaul and Guadalcanal only.

**Run in six parts:** US Navy carrier air groups by cruise; US Navy and Marine land-based squadrons; USAAF in the Pacific (Fifth, Seventh, Thirteenth, Twentieth Air Forces, Far East Air Forces); Japanese navy carrier air groups; Japanese navy land-based kokutai; Japanese army air units in New Guinea, the Philippines and the home islands. Split each further by year.

```
Task: for each air unit of PART, from formation or 7 December 1941 to
15 August 1945:

Attributes (dated rows): equipped_with (type), based_at (base or
carrier), aircraft_on_hand, aircraft_operational, pilots_on_hand,
parent (the higher unit), formed, disbanded (new), in_action_since.
For carrier air groups give each cruise: carrier, dates embarked,
squadrons and their types and numbers.
For Japanese units, the kokutai's renumbering on 1 November 1942 is a
trap: give the old and new number as alias rows with the date.
A strength on a date is a row with that date. Do not interpolate.
Start with: Grossnick, United States Naval Aviation 1910-1995 (NHHC) and
the Dictionary of American Naval Aviation Squadrons; the squadron war
diaries and action reports where you can open them; Maurer, Air Force
Combat Units of World War II and Combat Squadrons of the Air Force
(USAF, public domain); Craven and Cate; for Japan, Hata, Izawa and
Shores (by page read), j-aircraft.org unit histories, JACAR kodochosho
(unit action logs) with reference codes, USSBS Japanese Air Power.
```

## C08. Aircrew training and experience by year

**Why:** experience for air groups waits on this research (decision 0064). The pass of Nov 13 on pilot training was the weakest (six of eleven claims held); this prompt asks for narrower, countable statements.

**Run in two parts:** Japanese navy and army; US Navy, Marine Corps and Army.

```
Task: for SIDE, aircrew training and experience from 1941 to 1945.

1. Pilots graduated per year (or per month where given), by service
   and by course. Attribute pilots_graduated_per_year (new).
2. Flying hours in training before joining a unit, by year: attribute
   pilot_flying_hours, with valid_from and valid_to for the year.
3. Flying hours or experience of pilots in named units on named dates
   (veteran_share_percent, pilot_flying_hours, pilot_experience_note).
4. Pilot losses per month or per campaign, where the loser's records
   give them; claims kept apart (common rule 4).
5. Changes to training by date: courses shortened, fuel for training cut,
   night and instrument training added, carrier qualification, with the
   date and the stated reason.
6. Rotation: how long a US squadron stayed in combat before relief, and
   whether Japanese units were rotated, as statements.
Start with: USSBS Japanese Air Power and the interrogations of Japanese
officials (name each interrogation number); Japanese Monograph no. 157
(naval air training) and others on training; the US Navy's training
statistics in the Annual Report of the Secretary of the Navy and
Grossnick; the AAF Statistical Digest (tables of training).
```

## C09. Aircraft production and delivery to the theatre

**Why:** aircraft arrive at a rate by the day (decisions 0038, 0041). With an economy that rate is what industry delivered and shipping carried.

**Run in two parts:** Japan; the United States (and British carrier aircraft for the Pacific).

```
Task: for SIDE, monthly aircraft production and delivery, 1941 to 1945.

1. Production per month by type (or by class: fighter, bomber,
   reconnaissance, trainer, where only that is given), with the source's
   own totals as separate rows.
2. Deliveries to the Pacific theatre or to forward units per month, where
   given (the AAF Statistical Digest gives aircraft on hand overseas by
   theatre; the Navy gives aircraft on hand by fleet).
3. For Japan: the share of production lost in transit or in ferrying
   before reaching a unit, where a source states it.
4. Aircraft on hand in the theatre at the end of each month, if stated.
Start with: USSBS The Japanese Aircraft Industry; USSBS Japanese Air
Power; Army Air Forces Statistical Digest, World War II (1945); US Navy,
Naval Aviation in World War II statistics (Grossnick appendices); the
Navy's aircraft on hand reports where open.
```

## C10. Japanese merchant shipping and its losses

**Why:** economy answer 4: merchant shipping is a pool of tonnage with named convoys where a threat reaches. The game needs the pool's size, its use and its losses by cause.

**Run in two parts:** the pool and its use (tonnage, allocation between army, navy and civil, new construction); the losses (by month, cause, area, ship).

```
Task: PART for Japan, December 1941 to August 1945.

Part 1, the pool:
1. Merchant tonnage held at the start of each month, by category
   (army-requisitioned A, navy-requisitioned B, civil C), gross tons,
   and by kind (cargo, tanker, passenger-cargo).
2. New construction per month (tons and ships), tankers apart.
3. Tonnage captured and salvaged.
4. Turn-round times of named routes (e.g. Singapore to Japan,
   Palembang or Balikpapan to Japan), from a source.
5. The convoy system: when convoys began on each route, their usual
   size and escort, by date.
Part 2, the losses:
1. Ships sunk per month by cause: submarine, carrier aircraft, land
   aircraft, mine, surface ship, marine casualty; tonnage and number.
2. By area where given.
3. For named ships: one row per ship from JANAC (name, tonnage, date,
   position, cause, who sank her). For this part give no more than 300
   ships per run, beginning with tankers.
Start with: USSBS The War Against Japanese Transportation 1941-1945;
JANAC, Japanese Naval and Merchant Shipping Losses During World War II
by All Causes (1947); USSBS Japanese Merchant Shipbuilding; Parillo,
The Japanese Merchant Marine in World War II (by page read).
```

## C11. Oil, metals and general cargo: Japan's flows

**Why:** economy answer 1: three materials. Japan's economy is the one the blockade cuts, and it is built first (answer 3).

**Run in three parts:** oil; metals (iron ore, coal, bauxite, the rest); food, rubber and the rest (general cargo), with the effect of the bombing of Japan on each flow.

```
Task: for MATERIAL, Japan's sources, carriage and stocks, 1941 to 1945.

1. Production at each source per month or quarter: for oil, the
   refineries and fields of Palembang, Balikpapan, Tarakan, Miri and
   Seria, Burma, and home production; for metals, each named mine or
   region; for food, rice imports by origin.
2. Imports into the home islands per month by origin.
3. Stocks in the home islands at the end of each period, by kind
   (crude, heavy oil, aviation gasoline; for metals, by ore).
4. Consumption by user: navy, army, civil, per month or year, where
   given; navy fuel oil consumption by fleet where given.
5. Damage to the sources by Allied attack: date of each attack on a
   named source and its stated effect on output.
6. For the third part only: the effect of strategic bombing and the
   mining campaign on each flow, as the USSBS states it, with dates.
Units: oil in kilolitres or barrels as printed (as printed column
holds the source's unit), metals in metric tons. Never convert barrels
to tons in the value.
Start with: USSBS Oil in Japan's War; USSBS The Effects of Strategic
Bombing on Japan's War Economy; USSBS Japanese Wartime Standard of
Living; USSBS Coal and Metals in Japan's War Economy; Cohen, Japan's
Economy in War and Reconstruction (by page read).
```

## C12. The Allied line of supply and the fleet train

**Why:** supply from off the map is the second rule a long scenario needs. The US fleet's replenishment at sea in 1944 and 1945 is what let it stay at sea for weeks.

**Run in two parts:** shipping to the theatre (cargo ships and tankers allocated to the Pacific, fuel shipped, tonnage landed at the main bases by month); the fleet train (service squadrons, oilers, ammunition ships, replenishment at sea doctrine and practice by date).

```
Task: PART for the Allies, 1941 to 1945.

Part 1:
1. Cargo ships and tankers allocated to the Pacific by month or quarter,
   numbers and tonnage, by the controlling authority (WSA, Army, Navy).
2. Fuel oil and aviation gasoline shipped from the West Coast and
   elsewhere to the Pacific bases per month.
3. Tonnage of cargo landed or handled at named bases per month
   (Noumea, Espiritu Santo, Guadalcanal, Brisbane, Milne Bay, Manus,
   Saipan, Guam, Ulithi, Leyte, Okinawa), and the congestion at
   Noumea and elsewhere with dates.
4. Turn-round times of the main routes.
Part 2:
1. Each Service Squadron (2, 4, 6, 8, 10 and others): when formed, where
   based, its ships by number and type by date.
2. Replenishment at sea: the date and place of the first and of each
   notable fuelling, ammunition and stores transfer; the rate (tons per
   hour, barrels per hour) achieved where stated.
3. The logistics retirement area practice of the fast carriers in
   1944-45, with dates and the oilers and escort carriers involved.
Start with: Carter, Beans, Bullets, and Black Oil (NHHC, 1953, on
HyperWar); Leighton and Coakley, Global Logistics and Strategy (US Army
Green Book); Building the Navy's Bases in World War II; Ballantine,
U.S. Naval Logistics in the Second World War (by page read).
```

## C13. Bases and their facilities by date

**Why:** what a base could operate, repair and store is what decides where fleets and air forces could be. Our file has seven bases.

**Run in six parts:** Central Pacific (Pearl Harbor, Midway, Johnston, the Gilberts, Marshalls, Marianas, Iwo Jima, Truk); South Pacific (New Caledonia, the New Hebrides, Fiji, Samoa, the Solomons); South West Pacific (Australia's north and east, New Guinea, the Admiralties, Morotai, Biak); Japanese outer bases (Rabaul, Kavieng, Truk, Palau, the Marshalls and Carolines, Saipan, Tinian, the Indies, Singapore); the Philippines and Okinawa; the home islands' naval bases and yards (Kure, Yokosuka, Sasebo, Maizuru) and the US West Coast and Pearl Harbor yards.

```
Task: for each base of PART, its facilities by date, 1941 to 1945.

Attributes (dated rows): runways (number), runway_length_m,
aircraft_capacity (stated parking or operating capacity), drydock
(each dock with its lifting capacity in tons or its length),
repair_capability_note, fuel_storage_bbl (or tons as printed),
aviation_gasoline_storage_bbl, anchorage_capacity_note,
wharf_berths, cargo_handling_tons_per_day, hospital_beds,
garrison_men. Give the date the facility opened, was damaged, was
mended and was captured or abandoned.
For Japanese bases give both the Japanese record and the Allied estimate
of the time (intelligence reports), as separate rows.
Start with: Building the Navy's Bases in World War II (2 vols., 1947,
HyperWar); the Joint Intelligence Center Pacific Ocean Areas (JICPOA)
studies of Japanese bases; USSBS The American Campaign Against Wotje,
Maloelap, Mille and Jaluit, and the Truk and Rabaul reports; the US
Army Corps of Engineers in the SWPA (Engineers of the Southwest
Pacific); Japanese Monographs on base construction.
```

## C14. The submarine war, both sides, and anti-submarine war

**Why:** decision 0102 places submarines before any campaign past mid-September 1942. The US submarine campaign is half of how the Allies won; the Japanese one is how carriers were lost.

**Run in three parts:** US submarines (boats on station by month, patrols, sinkings, the torpedo problems and when each was fixed); Japanese submarines (deployments, sinkings, the shift to supply runs); anti-submarine war on both sides (escorts, aircraft, radar, sonar, the Japanese escort command and kaibokan).

```
Task: PART, December 1941 to August 1945.

US submarines:
1. Boats on patrol per month, by base (Pearl Harbor, Fremantle,
   Brisbane, later Midway, Majuro, Guam).
2. Per patrol, where a source gives it: boat, start and end dates,
   area, ships sunk (JANAC), torpedoes fired.
3. The Mk 14 and Mk 6 exploder problems: the date of each finding and of
   each fix (depth, magnetic exploder deactivation, contact pin), quoted.
4. Wolf packs: first use and later practice by date.
Japanese submarines:
1. Boats in commission and on operations per month.
2. Their sinkings of warships and merchant ships, by date, from the
   victims' records.
3. The use of submarines to supply garrisons: dates, boats, tonnage.
Anti-submarine war:
1. The Japanese Grand Escort Command (formed November 1943): its forces
   by date, the escorts available per convoy by period.
2. Japanese anti-submarine radar and magnetic detection (MAD) at sea
   and in the air, by date.
3. US anti-submarine successes against Japanese boats: per boat, date,
   cause, from JANAC and Japanese records.
Start with: US Submarine Operations in World War II (Roscoe, by page
read) and the Navy's own Submarine Operational History (on HyperWar);
JANAC; USSBS interrogations; Japanese Monographs nos. 102 to 104 and the
submarine monographs; Boyd and Yoshida, The Japanese Submarine Force and
World War II (by page read); combinedfleet.com submarine records.
```

## C15. Mines and the mining campaign

**Why:** mining closed ports and routes, above all in 1945 (Operation Starvation). It acts on carriage (economy link 2).

```
Task: mines in the Pacific War, 1941 to 1945, both sides.

1. Each mining operation: date, area, by whom (aircraft, submarine,
   surface ship), number of mines laid, mine types.
2. Ships sunk or damaged by mines, by month and area, by side, with
   sources.
3. For Operation Starvation (March to August 1945): mines laid per
   month, ports and straits closed and for how long, the effect on
   tonnage moved through Shimonoseki and the Inland Sea, as the USSBS
   states it.
4. Japanese defensive minefields: location, date laid, ships lost on
   them.
5. Minesweeping: the forces and the time to clear a field, by case.
Start with: USSBS The Offensive Mine Laying Campaign Against Japan
(1946); Johnson and Katcher, Mines Against Japan (Naval Ordnance Lab,
1973, government work); JANAC for sinkings by mine.
```

## C16. Land campaigns: forces and strengths by date

**Why:** ground combat is built through stage 3, but only Guadalcanal has a land order of battle.

**Run in seven parts:** Malaya, Singapore and the Indies (1941-42); the Philippines 1941-42; New Guinea and Papua (1942-44); the Solomons after Guadalcanal (New Georgia, Bougainville, 1943-44); the Gilberts, Marshalls and Marianas; the Philippines 1944-45; Iwo Jima and Okinawa.

```
Task: for the CAMPAIGN of PART, the land forces of both sides.

Attributes (dated rows, as in our land_unit_facts): strength_men,
landed_at (place), landed_on, location, parent, losses_killed,
losses_wounded, losses_missing, losses_sick, captured, evacuated,
rations_days_held, artillery_pieces, tanks.
1. Each unit at division, regiment, brigade, battalion and garrison
   level, by date: where it was, how many men.
2. Its arrival and departure dates (by sea, from where).
3. The estimate the other side made of it at the time, as separate
   rows (the belief rows decision 0173 needs).
4. Each named action ashore: dates, units engaged, losses, outcome.
5. Sickness rates (malaria and others) by month where stated.
Start with: the US Army in World War II volumes (Green Books) for the
campaign; the USMC historical monographs and the History of US Marine
Corps Operations in World War II (vols. I to V, HyperWar); the
Australian official history (Australia in the War of 1939-1945,
army series, awm.gov.au) by page; the Japanese Monographs for the
campaign; Senshi Sosho by volume and page.
```

## C17. Engagements to test against, 1941 to 1945

**Why:** engagement rows are history to compare a result with. They are never scripted (data rules).

**Run in five parts:** Dec 1941 to Apr 1942 (Pearl Harbor, the Prince of Wales and Repulse, Java Sea, Sunda Strait, the Indian Ocean raid, the carrier raids of early 1942, the Doolittle raid); 1943 (Bismarck Sea, Kula Gulf, Kolombangara, Vella Gulf, Vella Lavella, Empress Augusta Bay, Cape St George, the Rabaul carrier raids, Komandorski Islands); 1944 (Truk raid, Philippine Sea, the Formosa air battle, Leyte Gulf with its four actions); 1945 (Okinawa, the Yamato sortie, the carrier raids on Japan); and the night and air actions of the Solomons in 1943 not listed above.

```
Task: for each engagement of PART:

1. engagement: name, date_from, date_to, time zone of the times given.
2. forces: each side's ships and air units present, by formation, with
   the source.
3. events: the timeline, one row per stated event with its time
   (sighting, launch, opening fire, torpedo fired, hit, sinking,
   break-off).
4. outcomes: hits by weapon on each ship, ships sunk and damaged,
   aircraft lost by cause, men killed, rounds and torpedoes expended.
   Each side's own losses from its own records; the other side's claims
   as claims.
5. positions: track points, sinking positions (lat, lon as printed).
6. What each commander knew and believed at the moments of decision,
   as stated in the record (for the belief model).
Not needed: Coral Sea, Midway, Savo Island, Eastern Solomons, Cape
Esperance, Santa Cruz, the November 1942 night actions, Tassafaronga.
Start with: the action reports (on HyperWar and history.navy.mil); the
Naval War College analyses (Leyte Gulf by Bates, Midway and Coral Sea
for method); ONI Combat Narratives for 1943; USSBS Campaigns of the
Pacific War and its interrogations; combinedfleet.com battle pages for
the Japanese side, citing what they cite where you can open it.
```

## C18. Losses: ships and aircraft by date and cause

**Why:** the truth to validate a campaign's attrition against, and the claims beside it as what each side believed.

**Run in two parts:** ships (warships of both sides; merchant ships are C10); aircraft (by month and cause, both sides).

```
Task: PART, December 1941 to August 1945.

Ships:
1. Every warship of both sides sunk: name, date, position, cause, who
   sank her (from the loser's record), and as separate rows the claim
   the other side made at the time.
2. Warships put out of action for more than 30 days: name, date, cause,
   days out (from the repair record).
Aircraft:
1. Aircraft lost per month by side, service and cause: in air combat,
   to anti-aircraft fire, operational (accident, weather, fuel), on the
   ground. From each side's own records.
2. Claims per month by each side for the other's aircraft, kept apart.
3. Ratio of claims to losses where a source computes one (that source's
   figure only).
Start with: JANAC; NHHC, United States Naval Chronology and US Naval
Vessels Lost; USSBS Japanese Air Power and Campaigns of the Pacific War
(appendices on losses); the AAF Statistical Digest (losses by theatre);
the Navy's Naval Aviation Combat Statistics, World War II (OPNAV, 1946).
```

## C19. Intelligence: what each side read and knew, by date

**Why:** the intelligence design note's second layer is standing sources: what each side could read, see and hear, and when that changed.

**Run in two parts:** Allied (codebreaking, radio direction finding and traffic analysis, coastwatchers, photo reconnaissance, prisoner and document exploitation); Japanese (their codebreaking of Allied systems, radio intelligence, their reconnaissance and its gaps).

```
Task: for SIDE, its sources of intelligence by date, 1941 to 1945.

1. Each enemy cipher or code system: the dates it was in force, the dates
   it was being read and how much (a share or a description quoted), and
   the dates of each change that cut reading off.
2. Each named case where intelligence was in hand before an operation
   and what was known (date known, what, by whom); and each case where
   it failed, with the cause stated.
3. Coastwatcher and observer networks: stations, dates, areas covered.
4. Photo reconnaissance and long-range search capacity by period
   (aircraft and their radius; how often a base was photographed).
5. Each side's estimate of the other's strength at named dates (carriers,
   aircraft, divisions), beside the truth if a source compares them.
Start with: the NSA/CCH histories released to the public (e.g. Parker,
A Priceless Advantage; Pearl Harbor Revisited) on nsa.gov; SRH-series
documents at the National Archives (cite the SRH number); Layton, And I
Was There (by page read); Feldt, The Coast Watchers; USSBS
interrogations for Japanese intelligence; Japanese Monographs on
intelligence.
```

## C20. Special attack (kamikaze) and late-war air defence

**Why:** from October 1944 the air war changed in kind. A campaign to 1945 cannot play the last year without it.

```
Task: Japanese special attack and the Allied defence against it,
October 1944 to August 1945.

1. Special attack sorties per month by service (navy, army), aircraft
   expended, and the number reaching targets, from Japanese records and
   USSBS.
2. Ships hit, sunk and damaged by special attack per month and per
   campaign (Philippines, Iwo Jima, Okinawa), from the US records, ship
   by ship where given.
3. The rate of hits per sortie as the USSBS or a later study states it
   (that source's figure only).
4. Special attack boats (Shinyo, Maru-re), manned torpedoes (Kaiten) and
   Ohka: units, numbers, sorties, results, by date.
5. The Allied defence: radar picket stations at Okinawa and their
   losses, combat air patrol strength over the fleet by period, changes
   in carrier air group composition toward fighters (dates and numbers),
   the "big blue blanket" practice.
Start with: USSBS Japanese Air Power and the Interrogations; the
CINCPAC/CINCPOA reports on anti-suicide action (on HyperWar); Japanese
Monographs on the Philippines and Okinawa air operations; NHHC pages
on the radar pickets; Rielly, Kamikazes, Corsairs, and Picket Ships
(by page read).
```

## C21. Weather and seasons

**Why:** weather is deferred until a measure misses (decision 0102), but over four years it is not small: typhoons damaged a fleet twice, and the monsoon shaped operations in the Indies and the Philippines.

**Run in two parts:** typhoons and storms that struck forces at sea or ashore; seasonal conditions by region and month.

```
Task: PART, 1941 to 1945.

Typhoons and storms:
1. Each one that affected an operation: dates, track points (lat, lon,
   date, time as printed), forces caught, losses (ships sunk, damaged,
   aircraft lost, men lost), the warning available at the time.
   Including Typhoon Cobra (December 1944) and Typhoon Connie (June
   1945), and any that struck Japanese forces.
Seasonal conditions:
1. For each region (the Solomons and Bismarcks, New Guinea, the
   Marshalls and Carolines, the Marianas, the Philippines, Formosa and
   the Ryukyus, the Indies, the waters around Japan, the Aleutians), by
   month: share of days with poor visibility or low cloud, prevailing
   wind and sea state, typhoon or cyclone frequency, rainy season dates,
   as a contemporary source states them.
2. Daylight: not needed (it is computed).
Start with: the Navy court of inquiry and CINCPAC reports on Typhoon
Cobra and Connie (NHHC); the US Navy Hydrographic Office Sailing
Directions and the Weather Bureau / AAF Weather Service climatological
studies of the Pacific islands (1943-45, on archive.org or DTIC);
Japanese Monographs where they discuss weather.
```

## C22. Places and positions for a map of the whole Pacific

**Why:** decision 0017's map is for the slice only. A map of the whole Pacific needs every base, anchorage, strait and island a campaign names, with positions.

**Run in three parts:** Japan, the Ryukyus, Formosa, China coast, the Philippines and the Indies; the Central and North Pacific (Hawaii, the Marshalls, Gilberts, Carolines, Marianas, Bonins, Aleutians, Kuriles); the South and South West Pacific (Australia, New Guinea, the Bismarcks, the Solomons, New Hebrides, New Caledonia, Fiji, Samoa).

```
Task: for PART, every place a campaign of 1941 to 1945 would name:
bases, airfields, anchorages, ports, straits and passages, and the
islands they lie on.

1. For each: the name used in 1941-45 (both sides' names as aliases),
   today's name, and its position (lat, lon) from a gazetteer or chart,
   with the feature the position refers to (airfield, harbour,
   island centre).
2. For straits and passages: the narrowest width and the controlling
   depth where a sailing direction states it.
3. For anchorages: the depth and the number of large ships it held,
   where stated.
Positions are data, not text: give lat and lon in decimal degrees, south
and west negative, and the source's own form in "as printed".
Start with: the GEOnet Names Server (NGA, US government) for positions;
the Hydrographic Office Sailing Directions of the period; Building the
Navy's Bases for wartime names; JICPOA island studies.
```
