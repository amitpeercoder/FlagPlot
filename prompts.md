# Research prompts: the Pacific War campaign, 7 December 1941 to 15 August 1945

Written on Nov 25, 2026 at the designer's request, to open research for the whole campaign, and revised the same day on the architect's review. They are for an outside model (ChatGPT / Codex). Nothing here is settled and nothing here is a plan of the game: they gather sourced statements, and our readers check every statement against its source before a row is keyed into `data/oob`, as the passes of Nov 13 and Nov 14 did (`docs/research/oob/outside-research-nov-13.md`, `outside-research-nov-14.md`).

The end date: the designer asked for "13 August 1945". Hostilities ended on 15 August 1945 (the Emperor's broadcast) and the surrender was signed on 2 September. The prompts run to 15 August; a scenario can end earlier. Change it in the common block if another date is meant.

## How to run them

ChatGPT (Codex) works in a checkout of this repository: it reads this file and writes each answer to disk.

1. **Start each run with one line**, naming one prompt and one part, for example:
   `Read prompts.md. Follow its common block and run prompt CG, part 1.`
   One part per run; a fresh session for each run is best, so one answer does not lean on another.
2. **Answers go to `research-answers/`** at the root of the checkout, one file per run, named `<id>-part<n>-<short-subject>.md` (`CG-part1-allied-ships.md`, `C03-part1-carriers-and-battleships.md`). The folder is ignored by git: the answers are leads, not sources, and stay out of the repository as before. Our readers key rows from them into `data/oob`.
3. **A run that stops short** writes what it has and says in Table 3 what it did not reach. A second run of the same part writes `<id>-part<n>b-...md` and does not overwrite the first.
4. **About 80 statements a run.** The clean answers of the earlier passes held 59 to 73 claims each. A part that would need more is split again (by year, by type, by region) and the prompt says where to start.
5. **Large public-domain tables are not transcribed by the outside model.** JANAC, the AAF Statistical Digest, Ships' Data and the USSBS appendices are US government works in tables. Where a prompt meets one, it asks only where the table is (document, page, URL, what its columns are); our readers key it directly.
6. **What the earlier passes taught** (Nov 13 and Nov 14): no quote was invented, but about one claim in ten was misattributed (right words, wrong book, chapter, ship, date or author); twice the model's own arithmetic was presented as the source's; some quotes were elided or reworded; its "not found" answers were the weakest part, and its searches were shallow. The common block is written against each of these.

## Order

| Tranche | Prompts | Why first |
| --- | --- | --- |
| 1 | **CG** (Guadalcanal, 15 September 1942 to 9 February 1943); **C12 part 1** for the South Pacific 1942 only | The next larger scenario (`docs/design/larger-scenarios.md`): forces that arrive by date and supply from off the map |
| 2 | C00, C01, C22 | The cheap skeleton: chronology, commands and directives, places |
| 3 | C02, C03, C04, C05, C06, C07, C08 | The forces, by tranche: carriers and battleships before destroyers |
| 4 | C13, C14 (1943 on), C16, C17, C18, C19 | Bases, submarines, land, engagements, losses, intelligence |
| Held | C09, C10, C11, C15, C20, C21 part 2 | See "Held for the designer" |

### Held for the designer

- **C09 to C11 and C15 (production, merchant shipping, materials, mining).** They follow the economy note (`docs/design/economy.md`, Nov 12), which places these in step 5. Decision 0102 (Nov 14) declined industry, resources, research and shipbuilding. Ask the designer before spending reader time on them.
- **C20 (special attack)** matters only once a scenario reaches October 1944.
- **C21 part 2 (seasonal climate):** weather is deferred until a measure misses (decision 0102). Part 1, the typhoons that struck forces, is cheap and can run with tranche 4.

### Open questions for the designer

Whether Burma, India and China are in the campaign or only its edge (C00 asks the record, it does not decide); whether the strategic bombing of Japan is in (C11 asks only for its effect on the flows); whether the Aleutians are a scenario or only a theatre that ties forces down; and whether the Soviet entry of 9 August 1945 is an edge of the campaign.

## Prompt index

| Id | Subject | Parts | What the game would use it for |
| --- | --- | --- | --- |
| CG | Guadalcanal, 15 September 1942 to 9 February 1943 | 5 | The next larger scenario: forces arriving by date, both sides |
| C00 | Chronology of operations and the scope of the theatre | 4 (by year) | The skeleton: which operations, where, when; the map's extent |
| C01 | Command, areas, directives and belief by date | 2 (Allied, Japanese) | Command levels, area boundaries, the directives from above (decision 0157) |
| C02 | Order of battle on 7 December 1941 | 4 | The opening position |
| C03 | Warships entering, leaving and returning to the theatre | by tranche | Forces that arrive by date |
| C04 | Ship classes not on file | 4 | Class facts for ships the derivation has never seen |
| C05 | Radar and anti-aircraft refits by date | 3 | Fits that change over the war; a fit needs a dated source |
| C06 | Aircraft types, their loads and service dates | 2 | Type facts and when each reached the theatre |
| C07 | Air units by date | by service and year | Where each air unit was, on what, at what strength |
| C08 | Training and experience by year, aircrew and ships' crews | 3 | Experience (decision 0064 waits on this) |
| C09 | Aircraft production and delivery to the theatre | 2 | Held |
| C10 | Japanese merchant shipping and its losses | 2 | Held |
| C11 | Oil, metals and general cargo: Japan's flows | 3 | Held |
| C12 | The Allied line of supply and the fleet train | 2 | Supply from off the map; replenishment at sea |
| C13 | Bases and their facilities by date | 5 (by region) | What each base could operate, repair and store, and when |
| C14 | The submarine war, 1943 to 1945, and anti-submarine war | 3 | Submarines (decision 0102 places them before a long campaign) |
| C15 | Mines and the mining campaign | 1 | Held |
| C16 | Land campaigns: forces and strengths by date | 7 (by campaign) | Ground orders of battle beyond Guadalcanal |
| C17 | Engagements to test against, 1941 to 1945 | 4 (by period) | History to compare a result with, never to script it |
| C18 | Losses: ships and aircraft by date and cause | 2 | Truth for validation; claims kept apart as belief |
| C19 | Intelligence: what each side read, knew and believed, by date | 2 | Standing sources, and the believed forces decision 0173 owes |
| C20 | Special attack (kamikaze) and late-war air defence | 1 | Held |
| C21 | Weather and seasons | 2 | Typhoons now; seasons held |
| C22 | Places and positions for a map of the whole Pacific | 3 (by region) | The map scheme decision 0017 says the whole Pacific needs |

---

## The common block (read before every run)

```
You are a research assistant gathering sourced historical statements for a
wargame of the Pacific War, 7 December 1941 to 15 August 1945. Your answer is
checked statement by statement by a reader who opens each source you cite and
searches it for the words you quote. A statement that cannot be found in the
source you name is thrown away, so precision matters more than coverage.
Aim for about 80 statements in this run. Stop there and say in Table 3
what you did not reach.

YOU ARE WORKING IN A CHECKOUT OF THE GAME'S REPOSITORY

- Run only the prompt and part you were given. The prompt's own fenced
  block is your task; the text around it is for the designer.
- Write your answer to research-answers/<id>-part<n>-<short-subject>.md
  at the root of the checkout (create the folder if it is missing). If
  that file exists, write <id>-part<n>b-... and do not overwrite it.
- Write nothing else. Do not edit, create or delete any other file, do
  not run git commit, git push or git add, and do not open a pull request.
- Never open, read or search the folder references/. It holds a
  commercial wargame's manuals, which this research must not use.
- You may read, to avoid repeating what is held and to use the right
  attribute names: data/oob/attributes.csv (the attribute names),
  data/oob/*.csv (what is already on file; a fact already there from the
  same source and page need not be given again: say "already held" in
  the note if you give it), data/oob/gaps.csv (what earlier passes looked
  for and did not find), docs/research/oob/source-policy.md and
  docs/research/oob/database-design.md (the rules behind this block).
  These files are not sources: never cite them, cite what they cite
  only after opening it yourself.
- Do not read docs/research/ answers or notes as sources either; they
  are our own summaries.
- At the top of the file, before the tables: the prompt id and part,
  the date, the model you are, and two lines on what you covered and
  what you did not reach.

WHAT TO DELIVER

One Markdown file with four tables, in this order.

Table 1. Statements. One row is one source's statement of one value.
Columns:
  n | entity | entity kind | attribute | value | unit | as printed |
  valid from | valid to | source | locator | text layer | opened on |
  quote | quote (original script) | source kind | note

- entity: the ship, class, aircraft type, unit, base, place, operation or
  person, named as the source names it, with hull number or unit
  designation where there is one (e.g. "Essex (CV-9)", "Zuikaku",
  "201st Kokutai", "VF-2", "Truk (Chuuk), Dublon anchorage").
- entity kind: ship | ship class | aircraft type | air unit | land unit |
  base | place | weapon | sensor | operation | flow | command | other.
- attribute: use the names given in the prompt. They are the names our
  database already uses; do not invent a synonym for one of them. Only
  where none fits, write a short snake_case name with the unit as a
  suffix and mark it "(new)".
- value: one value only. Convert to the unit named and keep the source's
  own figure in "as printed" (e.g. value 9144, as printed "30,000 ft").
  Never put a range or two values in one row: two values are two rows.
- valid from / valid to: dates in YYYY-MM-DD. If the source gives only a
  month or a year, write YYYY-MM or YYYY; never invent a day. "valid to"
  is the last day the source says the value held; we convert it. "True on
  this date, limits unknown" is both columns set to that date, with a
  note.
- source: author, title, edition or year, publisher, copied from the
  document's own title page or header, not from memory or a search
  result. Write "not printed" for anything it does not give. The source
  is the thing you actually opened. If a web page quotes a book you did
  not open, the source is the web page and the note says "citing <book,
  page>". Never cite a book's page you did not see.
- locator: page, table, chapter or section AND the full URL of the
  document you opened. For a scanned PDF give the printed page number and
  the PDF page number; for JACAR give the reference code and image
  number. HyperWar pages: give the chapter URL and the printed page the
  passage falls on (HyperWar's page markers close the page they follow;
  check you have the right one). Where the original host often refuses
  automated readers (history.navy.mil, history.army.mil, army.mil,
  marines.mil, usmcu.edu), also give a HyperWar, archive.org or Wayback
  Machine copy if one exists.
- text layer: "text" if the document can be searched as text, "image"
  if it is a scan without a text layer.
- opened on: the date you read it.
- quote: the sentence or table cell that carries the value, verbatim, at
  most 40 words, findable by text search. It must contain the entity's
  name (or the table's row label) and the value: never a bare number or
  a single word. No ellipsis; if words must be skipped, mark the gap
  [...]. For a table, quote the row label and the cell.
- quote (original script): for a Japanese source, the passage in
  Japanese as printed; the "quote" column then holds your translation,
  marked "(tr.)".
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
2. No arithmetic of your own in a value. If you sum, divide, subtract or
   convert anything beyond a unit conversion, put the result in the note,
   marked "my arithmetic", and leave the value to what the source prints.
   A date or a duration worked out from two other dates or times is your
   arithmetic.
3. One fact per row. A source that states five ships' dates gives five rows.
4. Claims are not losses. A side's claim of enemy aircraft shot down or
   ships sunk is a claim: give it attribute "claimed_..." and say whose
   claim it is. Only the loser's records, or a postwar study built on
   them (JANAC, Japanese records, USSBS), establish a loss.
5. Each side is the authority on itself. A US wartime figure for Japanese
   strength is an estimate (attribute "..._estimated_by_enemy" or note
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
11. Prefer, in this order:
    (a) primary records and US government works: action reports, war
    diaries, the CINCPAC Gray Book, ONI Combat Narratives, Naval War
    College analyses, the US Strategic Bombing Survey, JANAC, the
    Japanese Monographs, Reports of General MacArthur, the US Naval
    Technical Mission to Japan reports, the US Army "Green Books", the
    USMC historical monographs, Building the Navy's Bases in World War II,
    the Army Air Forces Statistical Digest; read on HyperWar
    (ibiblio.org/hyperwar), history.navy.mil, archive.org, DTIC, the
    Combined Arms Research Library, or a university library;
    (b) Japanese records and the Japanese official history: JACAR (with
    its reference code), Senshi Sosho (volume and page);
    (c) scholarly works and reference books, cited only by a page you
    actually read (Craven and Cate, The Army Air Forces in World War II,
    is copyrighted and belongs here);
    (d) careful web compilations (combinedfleet.com, navweaps, navsource,
    uboat.net, j-aircraft), taking one fact at a time, never a whole
    record or table. Use (d) where (a) to (c) cannot be opened, say so,
    and look for a second source of another class for each fact.
    Take no more than 30 statements in a run from any one site of (d).
12. Two copies are one source. Two sites repeating the same book do not
    corroborate each other; say in the note when you see that.
13. Time zones and dates: say which zone a time is in if the source
    says, and "zone not stated" if it does not. Japanese records keep
    Tokyo time, so their date for an event east of the date line can be
    a day later than the American date (4 June 1942 at Midway is 5 June
    in Japanese records). Give the date as printed and say which
    convention it follows.
14. Search deep, not wide. Open the full document and read around the
    passage. A search-engine snippet is not a reading.
15. Write in plain English. No copied paragraphs: one quote per row, at
    most 40 words.
16. If a question cannot be answered from sources you can open, say so in
    Table 3. An honest gap is worth more than a filled cell.
17. Where a prompt names a report by number or title, check the number
    and title printed on the document itself and give those. If the
    prompt's number is wrong, say so.

Our data holds the battles of Coral Sea, Midway, Savo Island, the Eastern
Solomons, Cape Esperance, Santa Cruz, the November 1942 night actions and
Tassafaronga, and forces in depth only for 7 August to mid-September 1942
at Guadalcanal and Rabaul. Where a prompt says "on file", skip it.
```

---

## CG. Guadalcanal, 15 September 1942 to 9 February 1943

**Why:** the next larger scenario is Guadalcanal to the end of November (`docs/design/larger-scenarios.md`, its step 5), and its first need is "the forces from 15 September to 30 November, by date of arrival". This runs to the Japanese evacuation so the scenario can be lengthened later. Run it first.

**Run in five parts:** (1) Allied ships and task forces; (2) Japanese ships and forces; (3) air units of both sides; (4) land units of both sides; (5) supply and reinforcement runs. Split each at 30 November if it exceeds the cap; 15 September to 30 November first.

```
Task: PART of the Guadalcanal campaign, 15 September 1942 to
9 February 1943, by date of arrival and departure.

(1) and (2) Ships and task forces: every warship, transport, cargo ship
    and oiler that entered or left the waters of the Solomons, the New
    Hebrides, New Caledonia or Rabaul in the period. For each: the date
    she arrived (attribute arrived_in_theatre (new)), the date she left
    (transferred_out_of_theatre (new)), the task force or unit she
    served in by date (formation_note), the dates she was out of action
    and returned (out_of_action_from, repair_place,
    returned_to_service), and her fate (sunk, fate). Use movement_note
    for a stated movement that fits none of these.
(3) Air units: for each US Navy, Marine and Army squadron and each
    Japanese kokutai and carrier air group that served at Guadalcanal,
    Espiritu Santo, Rabaul, Buin, Buka or the carriers in the period:
    equipped_with, based_at, aircraft_on_hand, aircraft_operational,
    aircraft_received (aircraft arriving on a date), pilots_on_hand, with
    the date of each. Japanese units were renumbered on 1 November 1942:
    give the old and new number as alias rows with the date.
(4) Land units: each unit of both sides landed or withdrawn in the
    period, at regiment, battalion and division level (for example the
    7th Marines, the 164th Infantry, the Americal Division, the 25th
    Division; the Japanese 2nd and 38th Divisions and the Kawaguchi and
    Oka detachments where after 15 September). Attributes: landed_at,
    landed_on, landed_men, strength_men, departed_on, embarked_on,
    sailed_on, carried_by, parent_stated, attached_to, commander,
    killed, sick_men_cumulative, evacuated_men, replacements_men,
    rations_days_on_hand, and the other side's estimate as
    strength_men_estimated_by_enemy.
(5) Supply runs: each Japanese run (destroyer, barge, submarine,
    transport convoy) and each Allied convoy to Guadalcanal: date,
    ships, men and tons landed, losses on the way. Attributes:
    troops_carried_men, supply_tons_landed, troop_run (the run's name).
Start with: Miller, Guadalcanal: The First Offensive (Green Book);
Zimmerman, The Guadalcanal Campaign (USMC monograph); Hough, Ludwig and
Shaw, Pearl Harbor to Guadalcanal (USMC history vol. I); the ONI Combat
Narratives for the period; the CINCPAC Gray Book; the Japanese
Monographs on the Southeast Area naval and army operations; the Senshi
Sosho volumes on the South Pacific army and Southeast Area navy
operations (volume numbers as printed) by page; combinedfleet.com tabular records for
Japanese ship dates, one date per row, each with a second source where
one can be opened.
```

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
   sources say of the forces they took. For August 1945, the Soviet
   entry on 9 August as an edge.
6. For this period, the edges of the war at sea: the farthest positions
   reached by each side's surface forces, carriers and submarines that a
   source states, with the place and date.

Order Table 1 by date. A statement is still one source's statement:
do not merge two sources into one row.
Start with: Cressman, The Official Chronology of the U.S. Navy in World
War II (NHHC, on HyperWar); Williams, Chronology 1941-1945 (US Army in
World War II); The Campaigns of the Pacific War (USSBS, 1946); Reports
of General MacArthur, vols. I and II (vol. II is the Japanese side); the
Japanese Monographs (list which numbers you read). Morison is
copyrighted: cite only pages you open.
```

## C01. Command, areas, directives and belief by date

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
Start with: the CINCPAC Command Summary (Gray Book); Morton, Strategy
and Command: The First Two Years (US Army Green Book, HyperWar); Hayes,
The History of the Joint Chiefs of Staff in World War II: The War Against
Japan; Reports of General MacArthur; the Japanese Monographs on navy and
army operations (give each number and title as printed).
```

## C02. Order of battle on 7 December 1941

**Why:** the opening position of the campaign. Our file has almost nothing before May 1942.

**Run in four parts:** (a) Japanese navy, ships; (b) Allied navies, ships; (c) both sides' air, by unit; (d) land garrisons and forces at the points of attack. Parts (a) and (c) will exceed the cap: split (a) into the striking force and the southern forces, and (c) into Japanese and Allied.

```
Task: PART (a / b / c / d) of the order of battle on 7 December 1941
(8 December west of the date line; say which date each source uses).

(a) Japanese navy: the organisation of the Combined Fleet and every
    other fleet: fleet, squadron (sentai), destroyer squadron and
    division, with every ship by name, where each was on 7 December and
    which operation it was assigned to (Hawaii, Malaya, Philippines,
    Guam, Wake, Hong Kong, the Indies, home waters). Attributes:
    formation_note (its unit), movement_note (where she was), and
    operation_assigned (new).
(b) Allied navies: the US Pacific Fleet (with ships at Pearl Harbor,
    at sea and on the West Coast), the US Asiatic Fleet, the Royal Navy's
    Force Z and China Station, the Royal Netherlands Navy in the Indies,
    the Royal Australian Navy and Royal New Zealand Navy ships in or
    able to reach the theatre. Same attributes.
(c) Air: the air units in the theatre on both sides: unit, parent,
    base, aircraft type, aircraft on hand and operational, pilots on hand
    where given. Japanese navy kokutai and carrier air groups, Japanese
    army air regiments (sentai) in the southern operations, US Navy and
    Marine squadrons, US Army Air Forces in Hawaii and the Philippines
    (FEAF), RAF and RAAF in Malaya, Dutch army and navy air in the Indies.
    Attributes: equipped_with, based_at, aircraft_on_hand,
    aircraft_operational, pilots_on_hand. Name the parent unit in the
    note (it is a column of our unit list, not a fact).
(d) Land: the garrisons at Pearl Harbor, the Philippines, Guam, Wake,
    Hong Kong, Malaya, Singapore, the Indies, Rabaul, and the Japanese
    landing forces for each first-wave operation. Attributes:
    strength_men, position_on, parent_stated, commander.
Start with: the Pearl Harbor attack hearings exhibits (HyperWar); Morton,
The Fall of the Philippines; the USSBS Campaigns of the Pacific War
appendix of Japanese forces; Reports of General MacArthur vol. II; the
Japanese Monographs on the opening and Southern Area operations (give
numbers as printed); Craven and Cate vol. I by page read; Kirby, The War
Against Japan vol. I (UK official history; cite pages only).
```

## C03. Warships entering, leaving and returning to the theatre

**Why:** "forces that arrive by date" is the first rule any larger scenario needs (`docs/design/larger-scenarios.md`). The date that matters is not commissioning but the date a ship reached the theatre, left it for repair, and came back.

**Run by tranche, one per run:** (1) US carriers (CV, CVL) and battleships; (2) Japanese carriers and battleships; (3) US heavy and light cruisers; (4) Japanese cruisers; (5) US escort carriers; (6) British, Dutch and Commonwealth ships; then destroyers, escorts and the fleet train, one year and one navy per run. The first two tranches are wanted first.

```
Task: for every ship of TRANCHE, her movements into and out of the
Pacific theatre from 7 December 1941 to 15 August 1945.

Attributes (one row each, dated):
  commissioned, arrived_in_theatre (new: first arrival at Pearl Harbor
  or a base west of it, or for Japanese ships first deployment out of
  home waters), out_of_action_from, repair_place, returned_to_service,
  last_refit_completed, transferred_out_of_theatre (new), sunk, fate,
  hull_designation. Use movement_note, formation_note or
  time_in_theatre_note for a stated movement that fits none of these.
For out_of_action_from, give the cause (the action or accident) in the
note.

Do not list every voyage. Only the dates above.
Skip dates inside 7 August to mid-September 1942 in the South Pacific
(on file).
Start with: DANFS (history.navy.mil, one entry per ship; cite the entry
URL, and the HyperWar or Wayback copy); NHHC War Damage Reports for
repair periods; a war diary or a yard's record over DANFS where you can
open one. For Japanese ships, combinedfleet.com's tabular records give
one date per row, never a whole record; for each date look for a second
source (Senshi Sosho, JACAR, Jentschura, Jung and Mickel by page read)
and give it as its own row. uboat.net Allied Warships for British, Dutch
and Commonwealth ships.
```

## C04. Ship classes not on file

**Why:** the derivation (`oob-derive`) can only rate a class it has rows for.

**On file already, skip:** Japanese: Akizuki, Aoba, Asashio, Chitose, Fubuki, Furutaka, Kagero, Kamikaze, Kongo, Mogami, Mutsuki, Myoko, Nagara, Nagato, Ryujo, Sendai, Shiratsuyu, Shokaku, Takao, Tenryu, Tone, Yubari, Yugumo, Hatsuharu, Zuiho, Akagi, Kaga, Soryu, Hiryu, Yamato, Hosho, Ise, Fuso, Kuma, Nisshin, and the submarine classes of 1942. US: Atlanta, Bagley, Benham, Clemson, Farragut, Gleaves, Gridley, Lexington, Mahan, New Orleans, North Carolina, Northampton, Pensacola, Porter, Portland, Sims, Wasp, Wickes, Yorktown, Brooklyn, St. Louis, Benson, the 1942 transports and oilers, and the Gato, Tambor, Sargo, Narwhal and S classes. British: County (Kent), Modified Leander.

**Run in four parts:** (1) the 1941 classes the opening needs: US Pennsylvania, Tennessee, Colorado, New Mexico, Nevada, Omaha, Ranger, Langley; British King George V, Renown, York; Dutch De Ruyter, Java, Tromp, Admiralen. (2) US classes of 1942 to 1945: Essex, Independence, Iowa, South Dakota, Alaska, Baltimore, Cleveland, Fletcher, Allen M. Sumner, Gearing, the destroyer escort classes, Casablanca, Bogue, Sangamon and Commencement Bay escort carriers, Balao and Tench submarines, and the lift: APA, AKA, LST, LSD, LCI. (3) Japanese classes new from 1942: Taiho, Unryu, Shinano, Hiyo (Junyo), Ryuho, the escort carriers Taiyo and Shinyo (the carrier 神鷹, not the boats of C20), Ise as converted in 1943, Agano, Oyodo, Shimakaze, Matsu, Tachibana, the kaibokan classes, the No. 1 and No. 101 transports, the I-400 and later submarines. (4) the British Pacific Fleet of 1945: Illustrious and Implacable classes, and the other British classes there.

```
Task: for each ship class of PART, the class facts below, each from a
source, one row per value.

Attributes: displacement_standard_lt, displacement_full_lt,
displacement_normal_lt, displacement_trial_lt (Japanese sources often
print trial or normal displacement: give it as printed under its own
name), length_overall_m, beam_m, max_speed_kn, shaft_power_shp,
endurance_nm, endurance_speed_kn, fuel_capacity_lt (or fuel_capacity_bbl
where the source gives barrels), belt_armour_max_mm, deck_armour_max_mm,
complement, aircraft_capacity, torpedo_tubes, torpedo_tubes_per_broadside,
torpedo_model, torpedoes_carried, torpedo_reloads, main_rounds_per_gun;
for transports troop_capacity_men and cargo_capacity_lt; for submarines
also surfaced_displacement_lt, submerged_displacement_lt,
submerged_max_speed_kn, diving_depth_m, torpedo_tubes_bow,
torpedo_tubes_stern. As-built armament as separate rows: each mount type
and number, with the date the source gives (armament_note if only a
list is given).
Speeds and endurance are nominal unless the source says trials or
service; say which.
Start with: the US Navy's Ships' Data, U.S. Naval Vessels (the edition
of 15 April 1945 and earlier editions; archive.org / NHHC); the US Naval
Technical Mission to Japan reports on Japanese ships (series S; give
each number and title as printed); combinedfleet.com class pages and
navweaps, one fact at a time, each with a second source where one can
be opened.
```

## C05. Radar and anti-aircraft refits by date

**Why:** what a ship carried changed through the war, and the derivation reads a fit only on its date. A class fit is not a ship's fit (common rule 7). Fits are kept in our database as: the ship or class, the equipment, how many, from which date.

**Run in three parts:** US radar (SC, SK, SG, SP, SM, Mk 4, Mk 12/22, Mk 8, Mk 13) and the 40 mm and 20 mm programme; Japanese radar (Type 21, 22, 13) and the 25 mm additions of 1943 and 1944; the proximity (VT) fuze and the Mk 37 and Mk 51 directors.

```
Task: for PART, when each fit reached ships in the Pacific.

1. For each equipment: date of first fit at sea; for each named ship
   where a source gives it, one row with the ship as entity,
   attribute "fit (new)", the value "<equipment> x <count>", and the
   date it was fitted or first seen fitted. Where the source names no
   count or set, use radar_fit_note or armament_note. Programmes ("all
   destroyers to receive X by ...") are quoted as programmes, with the
   class as entity.
2. For anti-aircraft: the number of each gun type on named ships on
   named dates, from a dated source (a photograph's caption counts only
   if the source dates it).
3. The proximity fuze: date first issued to the Pacific Fleet, first
   use in action, the share of 5-inch anti-aircraft rounds that were VT
   by period where a source gives it.
4. Japanese radar: date first fitted, which ships by date, and stated
   detection ranges (range_aircraft_nm, range_ship_nm), from the US
   Naval Technical Mission reports.
Start with: FTP 217, U.S. Radar: Operational Characteristics (1943), and
any later edition; the Bureau of Ordnance and Bureau of Ships records on
HyperWar; the US Naval Technical Mission to Japan reports on radar and
on ordnance (give each number and title as printed); combinedfleet.com
ship pages for dated Japanese refits, each with a second source where
one can be opened; Friedman's books and Baldwin, The Deadly Fuze, by
page read; NHHC pages on the VT fuze.
```

## C06. Aircraft types, their loads and service dates

**Why:** aircraft that arrive later in the war (F6F, F4U, TBF, SB2C, P-38, B-24 in the Pacific, B-29 later, A6M5, N1K, Ki-84, D4Y, B6N, P1Y) have no rows.

**Run in two parts:** Allied types; Japanese types. Split each into fighters and the rest if over the cap.

```
Task: for each aircraft type and variant of PART that served in the
Pacific from December 1941 to August 1945:

Attributes: max_speed_kn, max_speed_altitude_m, cruise_speed_kn,
range_normal_nm, range_max_nm, combat_radius_nm, endurance_h,
service_ceiling_m, engine, engine_power_hp, empty_weight_kg,
loaded_weight_kg, wingspan_m, max_bomb_load_kg, internal_fuel_l,
fuel_capacity_l, drop_tank_l, crew, pilot_armour, self_sealing_tanks,
carrier_capable, first_service (date the first unit received it),
first_in_action_pacific (new: date and place of first combat in the
theatre), name_native, alias (the Allied reporting name, with the date
it was given).
Guns and bombs as fits: for each type, each gun (calibre, number,
rounds per gun) and each bomb or torpedo load, one row per item, with
attribute "fit (new)" and the value "<weapon> x <count>".
Give the loading condition each performance figure was measured at.
For Japanese types give both the Japanese figures (from the Technical
Mission or Japanese records) and the US wartime estimate (TAIC), as
separate rows: the estimate is what the Allies believed.
Start with: Standard Aircraft Characteristics sheets (US Navy, Bureau of
Aeronautics) and the AAF characteristics summaries, on archive.org or
the Navy's sites; the TAIC manuals and Japanese Aircraft Performance and
Characteristics (TAIC; give the manual number and date printed on the
cover); USSBS Japanese Air Power; the Technical Mission's aircraft
reports.
```

## C07. Air units by date

**Why:** where each air unit was, on what aircraft and at what strength, is what the game's air groups are. Our file holds August and September 1942 at Rabaul and Guadalcanal only.

**Run by service and year, one per run:** US Navy carrier air groups by cruise; US Navy and Marine land-based squadrons; USAAF in the Pacific (Fifth, Seventh, Eleventh, Thirteenth, Twentieth Air Forces, Far East Air Forces); RAAF and RNZAF; the Fleet Air Arm in the Indian Ocean and the British Pacific Fleet; Japanese navy carrier air groups; Japanese navy land-based kokutai; Japanese army air units in New Guinea, the Philippines and the home islands.

```
Task: for each air unit of PART, in YEAR:

Attributes (dated rows): equipped_with (type), based_at (base or
carrier), aircraft_on_hand, aircraft_operational, aircraft_received
(aircraft arriving on a date), pilots_on_hand, formed, in_action_since,
disbanded (new), prior_combat_note, losses_before_scenario_note. Name
the higher unit in the note.
For carrier air groups give each cruise: carrier, dates embarked,
squadrons and their types and numbers.
For Japanese navy units two reorganisations are traps: the renumbering
of kokutai on 1 November 1942, and the hikotai system of 1944 that
separated flying squadrons from their kokutai. Give old and new names
as alias rows with the date, and say which a strength refers to.
A strength on a date is a row with that date. Do not interpolate.
Start with: Grossnick, United States Naval Aviation 1910-1995 (NHHC);
the Dictionary of American Naval Aviation Squadrons (it covers some
lineages only; say which volume); squadron war diaries and action
reports where you can open them; Maurer, Air Force Combat Units of
World War II and Combat Squadrons of the Air Force (USAF, public
domain); Craven and Cate by page read; for Japan, JACAR kodochosho (unit
action logs) with reference codes, USSBS Japanese Air Power,
j-aircraft.org unit histories, and Hata, Izawa and Shores by page read.
```

## C08. Training and experience by year, aircrew and ships' crews

**Why:** experience waits on this research (decision 0064). The pass of Nov 13 on pilot training was the weakest (six of eleven claims held); this prompt asks for narrower, countable statements.

**Run in three parts:** Japanese navy and army aircrew; US Navy, Marine Corps and Army aircrew; ships' crews of both navies after 1942 (gunnery, night action, damage control, lookout training).

```
Task: for PART, training and experience from 1941 to 1945.

Aircrew:
1. Pilots graduated per year (or per month where given), by service
   and by course: pilots_trained_per_year.
2. Length of the course and hours flown in it, by year:
   training_course_months, training_hours_in_course.
3. Hours a pilot had on joining a unit: flying_hours_on_joining; the
   average in a named unit on a named date: flying_hours_average_in_unit.
4. Pilot losses per month or per campaign, where the loser's records
   give them; claims kept apart (common rule 4).
5. Changes to training by date: courses shortened, fuel for training
   cut, night and instrument training added, carrier qualification
   (training_note, night_training_note, carrier_qualification_note,
   service_length_note), with the date and the stated reason.
6. Rotation: how long a US squadron stayed in combat before relief, and
   whether Japanese units were rotated, as statements.
Ships' crews (part 3):
1. Night action and gunnery practice by navy and period
   (gunnery_practice_note, night_training_note), damage control
   training (damage_control_note), lookout training
   (lookout_training_note), each dated.
2. Turnover of crews in named ships on named dates.
Start with: USSBS Japanese Air Power and the Interrogations of Japanese
Officials (give each interrogation number); the Japanese Monographs on
naval and army air training (give numbers and titles as printed); the
Annual Reports of the Secretary of the Navy; Grossnick; the AAF
Statistical Digest (tables of training: say where the table is, do not
transcribe it); for ships' crews, the Fleet Training publications and
gunnery bulletins of the US Navy on HyperWar, and the Technical Mission
and USSBS interrogations for Japan.
```

## C09. Aircraft production and delivery to the theatre (held)

**Why:** aircraft arrive at a rate by the day (decisions 0038, 0041). With an economy that rate is what industry delivered and shipping carried. Held: see "Held for the designer".

**Run in two parts:** Japan; the United States (and British carrier aircraft for the Pacific).

```
Task: for SIDE, monthly aircraft production and delivery, 1941 to 1945.

1. Where the monthly production tables are, by type or by class
   (fighter, bomber, reconnaissance, trainer): document, page, URL,
   columns. Do not transcribe them.
2. Deliveries to the Pacific theatre or to forward units per month,
   where a source states them in prose; where a table holds them, say
   where it is.
3. For Japan: the share of production lost in transit or in ferrying
   before reaching a unit, where a source states it.
4. Aircraft on hand in the theatre at named dates, where stated in prose.
Start with: USSBS The Japanese Aircraft Industry; USSBS Japanese Air
Power; Army Air Forces Statistical Digest, World War II; Grossnick
appendices for the Navy.
```

## C10. Japanese merchant shipping and its losses (held)

**Why:** economy answer 4: merchant shipping is a pool of tonnage with named convoys where a threat reaches. Held: see "Held for the designer".

**Run in two parts:** the pool and its use; the losses.

```
Task: PART for Japan, December 1941 to August 1945.

Part 1, the pool:
1. Merchant tonnage held at the start of each month, by category
   (army-requisitioned A, navy-requisitioned B, civil C), gross tons,
   and by kind (cargo, tanker, passenger-cargo): say where the tables
   are; quote prose statements.
2. New construction per month (tons and ships), tankers apart: the same.
3. Tonnage captured and salvaged.
4. Turn-round times of named routes (e.g. Singapore to Japan,
   Palembang or Balikpapan to Japan), from a source.
5. The convoy system: when convoys began on each route, their usual
   size and escort, by date.
Part 2, the losses:
1. Where JANAC's ship-by-ship list and its summaries by cause, month
   and area are (document, page, URL): do not transcribe them.
2. Prose statements on losses by cause and area from USSBS and others.
3. Tankers lost, by named ship, where a source other than JANAC gives
   the circumstances, up to the cap.
Start with: USSBS The War Against Japanese Transportation 1941-1945;
JANAC, Japanese Naval and Merchant Shipping Losses During World War II
by All Causes (1947); USSBS Japanese Merchant Shipbuilding; Parillo,
The Japanese Merchant Marine in World War II (by page read).
```

## C11. Oil, metals and general cargo: Japan's flows (held)

**Why:** economy answer 1: three materials. Held: see "Held for the designer".

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
Where a table holds a series, say where it is and quote only what the
text says of it. Units: oil in kilolitres or barrels as printed (the
"as printed" column holds the source's unit), metals in metric tons.
Never convert barrels to tons in the value.
Start with: USSBS Oil in Japan's War; USSBS The Effects of Strategic
Bombing on Japan's War Economy; USSBS The Japanese Wartime Standard of
Living and Utilization of Manpower; USSBS Coal and Metals in Japan's War
Economy; Cohen, Japan's Economy in War and Reconstruction (by page read).
```

## C12. The Allied line of supply and the fleet train

**Why:** supply from off the map is the second rule a long scenario needs. The US fleet's replenishment at sea in 1944 and 1945 is what let it stay at sea for weeks.

**Run in two parts:** (1) shipping to the theatre, run first for the South Pacific of 1942 only (Noumea, Espiritu Santo, Guadalcanal, Auckland, Brisbane), later periods after; (2) the fleet train.

```
Task: PART for the Allies, in PERIOD.

Part 1:
1. Cargo ships and tankers allocated to the area by month or quarter,
   numbers and tonnage, by the controlling authority (WSA, Army, Navy).
2. Fuel oil and aviation gasoline shipped to the area's bases per month.
3. Tonnage of cargo landed or handled at named bases per month, and the
   congestion at Noumea and elsewhere with dates. Attributes on the base:
   unloading_rate_tons_per_day, arrival_rate_note, fed_from_note,
   stock_note, ships_waiting_note, supply_interrupted_note.
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

**Run in five parts:** (1) Allied Central Pacific (Pearl Harbor, Midway, Johnston, and the Gilberts, Marshalls, Marianas and Iwo Jima after capture); (2) South Pacific (New Caledonia, the New Hebrides, Fiji, Samoa, the Solomons); (3) South West Pacific (Australia's north and east, New Guinea, the Admiralties, Morotai, Biak); (4) Japanese outer bases until captured or bypassed (Rabaul, Kavieng, Truk, Palau, the Marshalls and Carolines, Saipan, Tinian, the Indies, Singapore); (5) the Philippines, Okinawa, the home islands' naval bases and yards (Kure, Yokosuka, Sasebo, Maizuru) and the US West Coast yards. A base captured in the war goes in part 4 until it changed hands and in part 1, 3 or 5 after.

```
Task: for each base of PART, its facilities by date, 1941 to 1945.

Attributes (dated rows): runway_count, runway_length_m, runway_width_m,
runway_surface, airfield_opened, aircraft_capacity (stated parking or
operating capacity), aircraft_based, seaplane_base, revetments_note,
drydock (each dock as a row, with its lifting capacity or length in the
value as printed), repair_capability_note, repair_ship_present,
fuel_oil_storage_note, aviation_fuel_storage_note, ammunition_depot_note,
port_berths, port_note, unloading_rate_tons_per_day, garrison_strength,
construction_units, base_role_note, code_name, runway_state_note (for
damage and repair, with the date).
Give the date the facility opened, was damaged, was mended and was
captured or abandoned.
For Japanese bases give both the Japanese record and the Allied estimate
of the time (intelligence reports), as separate rows.
Start with: Building the Navy's Bases in World War II (2 vols., 1947,
HyperWar); the JICPOA studies of Japanese bases; USSBS The American
Campaign Against Wotje, Maloelap, Mille and Jaluit, and the USSBS
reports on Truk and Rabaul; Engineers of the Southwest Pacific (US Army);
the Japanese Monographs on base construction (numbers as printed).
```

## C14. The submarine war, 1943 to 1945, and anti-submarine war

**Why:** decision 0102 places submarines before any campaign past mid-September 1942. 1942 is on file (`docs/research/submarines-in-1942.md`: attacks on warships in the Coral Sea, Midway and Guadalcanal waters, patrol stations, the Mark 14 and Japanese torpedoes with their dated faults, anti-submarine practice to 1942).

**Run in three parts:** US submarines; Japanese submarines; anti-submarine war on both sides.

```
Task: PART, 1 January 1943 to 15 August 1945 (1942 is on file).

US submarines:
1. Boats on patrol per month, by base (Pearl Harbor, Fremantle,
   Brisbane, later Midway, Majuro, Guam).
2. Per patrol, where a source gives it in prose: boat, start and end
   dates, area, ships sunk, torpedoes fired. Where a table holds the
   patrols, say where it is.
3. The torpedo fixes of 1943 (magnetic exploder deactivated, contact
   exploder firing pin): the date of each, by command, quoted.
4. Wolf packs: first use and later practice by date.
Japanese submarines:
1. Boats in commission and on operations per month.
2. Their sinkings of warships by date, from the victims' records.
3. The use of submarines to supply garrisons: dates, boats, tonnage.
Anti-submarine war:
1. The Japanese Grand Escort Command (formed 15 November 1943): its
   forces by date, the escorts available per convoy by period.
2. Japanese anti-submarine radar and magnetic detection (MAD) at sea
   and in the air, by date.
3. US anti-submarine successes against Japanese boats: per boat, date,
   cause, from JANAC and Japanese records.
Start with: the Navy's Submarine Operations in World War II (Roscoe, by
page read) and the US submarine operational history on HyperWar; JANAC
(say where its tables are); USSBS interrogations; the Japanese
Monographs on submarine operations (numbers and titles as printed);
Boyd and Yoshida, The Japanese Submarine Force and World War II (by
page read); combinedfleet.com submarine records, one fact at a time.
```

## C15. Mines and the mining campaign (held)

**Why:** mining closed ports and routes, above all in 1945 (Operation Starvation). It acts on carriage (economy link 2). Held: see "Held for the designer".

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
(1946); Johnson and Katcher, Mines Against Japan (Naval Ordnance
Laboratory, 1973); JANAC for sinkings by mine.
```

## C16. Land campaigns: forces and strengths by date

**Why:** ground combat is built through stage 3, but only Guadalcanal has a land order of battle (and only to mid-September 1942; CG takes it on).

**Run in seven parts:** Malaya, Singapore and the Indies (1941-42); the Philippines 1941-42; New Guinea and Papua (1942-44); the Solomons after Guadalcanal (New Georgia, Bougainville, 1943-44); the Gilberts, Marshalls and Marianas; the Philippines 1944-45; Iwo Jima and Okinawa. Split a part by year if over the cap.

```
Task: for the CAMPAIGN of PART, the land forces of both sides.

Attributes (dated rows): strength_men, strength_men_planned,
landed_at, landed_on, landed_men, position_on, departed_on,
embarked_on, sailed_on, carried_by, parent_stated, task_org_parent,
attached_to, commander, killed, casualties_total, sick_men_cumulative,
evacuated_men, replacements_men, rations_days_on_hand,
heavy_weapons_ashore_note, attack_on, ceased_on, and the other side's
estimate as strength_men_estimated_by_enemy.
1. Each unit at division, regiment, brigade, battalion and garrison
   level, by date: where it was, how many men.
2. Its arrival and departure dates (by sea, from where).
3. The estimate the other side made of it at the time, as separate rows.
4. Each named action ashore: dates, units engaged, losses, outcome.
5. Sickness rates (malaria and others) by month where stated.
6. Replacements received, by unit and date.
Start with: the US Army in World War II volumes (Green Books) for the
campaign; the USMC historical monographs and the History of US Marine
Corps Operations in World War II (vols. I to V, HyperWar); Reports of
General MacArthur; the Australian official history (Australia in the
War of 1939-1945, army series, awm.gov.au) by page; the Japanese
Monographs for the campaign; Senshi Sosho by volume and page.
```

## C17. Engagements to test against, 1941 to 1945

**Why:** engagement rows are history to compare a result with. They are never scripted (data rules).

**Run one engagement per run** for the large ones (Pearl Harbor, Java Sea, the Indian Ocean raid, Philippine Sea, each of the four actions of Leyte Gulf, the Yamato sortie), and the smaller ones of a period together: Dec 1941 to Apr 1942 (also the Prince of Wales and Repulse, Sunda Strait, the carrier raids of early 1942, the Doolittle raid); 1943 (Bismarck Sea, Komandorski Islands, Kula Gulf, Kolombangara, Vella Gulf, Vella Lavella, Empress Augusta Bay, Cape St George, the Rabaul carrier raids, and the other night and air actions of the Solomons); 1944 (Truk raid, the Formosa air battle); 1945 (the carrier raids on Japan, Okinawa's air and sea actions).

```
Task: for ENGAGEMENT (or the engagements of PERIOD):

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
On file, skip: Coral Sea, Midway, Savo Island, Eastern Solomons, Cape
Esperance, Santa Cruz, the November 1942 night actions, Tassafaronga.
Start with: the action reports (on HyperWar and history.navy.mil); the
Naval War College analyses (Bates on Leyte Gulf); ONI Combat Narratives
for 1943; USSBS Campaigns of the Pacific War and its interrogations;
the Japanese Monographs and Senshi Sosho for the Japanese side;
combinedfleet.com battle pages, citing what they cite where you can
open it.
```

## C18. Losses: ships and aircraft by date and cause

**Why:** the truth to validate a campaign's attrition against, and the claims beside it as what each side believed.

**Run in two parts:** ships (warships of both sides); aircraft (by month and cause, both sides).

```
Task: PART, December 1941 to August 1945.

Ships:
1. Where the lists are: JANAC for Japanese warships, the US Navy's lists
   of vessels lost: document, page, URL, columns. Do not transcribe them.
2. Warships put out of action for more than 30 days: name, date, cause,
   days out (from the repair record), up to the cap, carriers,
   battleships and cruisers first.
3. The claim the other side made at the time for each carrier and
   battleship sunk or damaged, as a claim.
Aircraft:
1. Aircraft lost per month by side, service and cause (air combat,
   anti-aircraft fire, operational, on the ground), from each side's own
   records: where the tables are, and prose statements quoted.
2. Claims per month by each side for the other's aircraft, kept apart.
3. Ratio of claims to losses where a source computes one (that source's
   figure only).
Start with: JANAC; NHHC United States Naval Chronology and its list of
US naval vessels lost; USSBS Japanese Air Power and Campaigns of the
Pacific War (appendices on losses); the AAF Statistical Digest (losses
by theatre); Naval Aviation Combat Statistics, World War II (OPNAV,
1946).
```

## C19. Intelligence: what each side read, knew and believed, by date

**Why:** the intelligence design note's second layer is standing sources: what each side could read, see and hear, and when that changed. Decision 0173 also owes what each command believed of the other in the first week of August 1942, as rows.

**Run in two parts:** Allied (codebreaking, radio direction finding and traffic analysis, coastwatchers, photo reconnaissance, prisoner and document exploitation); Japanese (their codebreaking of Allied systems, radio intelligence, their reconnaissance and its gaps). Each part starts with item 6 for 1 to 9 August 1942.

```
Task: for SIDE, its sources of intelligence and its beliefs by date,
1941 to 1945.

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
6. What the side believed of the other's forces and intent on named
   dates, beginning with 1 to 9 August 1942 at Guadalcanal: the force
   it believed was there or coming (attribute believed_force (new)), and
   the intent it believed (believed_intent (new)), with who believed it.
Start with: the NSA Center for Cryptologic History publications released
to the public (Parker, A Priceless Advantage; Pearl Harbor Revisited) on
nsa.gov; SRH-series documents at the National Archives (cite the SRH
number); the CINCPAC Gray Book's estimates; Layton, And I Was There,
and Feldt, The Coast Watchers, by page read; USSBS interrogations for
Japanese intelligence; the Japanese Monographs on intelligence.
```

## C20. Special attack (kamikaze) and late-war air defence (held)

**Why:** from October 1944 the air war changed in kind. Held until a scenario reaches that date.

```
Task: Japanese special attack and the Allied defence against it,
October 1944 to August 1945.

1. Special attack sorties per month by service (navy, army), aircraft
   expended, and the number reaching targets, from Japanese records and
   USSBS.
2. Ships hit, sunk and damaged by special attack per month and per
   campaign (Philippines, Iwo Jima, Okinawa), from the US records, ship
   by ship where given (say where the tables are).
3. The rate of hits per sortie as the USSBS or a later study states it
   (that source's figure only).
4. Special attack boats (the navy's Shinyo 震洋 and the army's Maru-re),
   manned torpedoes (Kaiten) and Ohka: units, numbers, sorties, results,
   by date.
5. The Allied defence: radar picket stations at Okinawa and their
   losses, combat air patrol strength over the fleet by period, changes
   in carrier air group composition toward fighters (dates and numbers),
   the "big blue blanket" practice.
Start with: USSBS Japanese Air Power and the Interrogations; the US
Navy's summaries of anti-suicide action (COMINCH or CINCPAC; give the
title, issuer and date printed on the document, on HyperWar); the
Japanese Monographs on the Philippines and Okinawa air operations; NHHC
pages on the radar pickets; Rielly, Kamikazes, Corsairs, and Picket
Ships (by page read).
```

## C21. Weather and seasons

**Why:** weather is deferred until a measure misses (decision 0102), but typhoons damaged a fleet twice. Part 1 is cheap and runs with tranche 4; part 2 is held.

**Run in two parts:** (1) typhoons and storms that struck forces at sea or ashore; (2, held) seasonal conditions by region and month.

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
Start with: the courts of inquiry and CINCPAC reports on Typhoon Cobra
and Connie (NHHC); the US Navy Hydrographic Office Sailing Directions
and the climatological studies of the Pacific islands by the Weather
Bureau and the AAF Weather Service (1943-45, on archive.org or DTIC);
the Japanese Monographs where they discuss weather.
```

## C22. Places and positions for a map of the whole Pacific

**Why:** decision 0017's map is for the slice only. A map of the whole Pacific needs the bases, anchorages, straits and islands a campaign names, with positions.

**Run in three parts:** Japan, the Ryukyus, Formosa, China coast, the Philippines and the Indies; the Central and North Pacific (Hawaii, the Marshalls, Gilberts, Carolines, Marianas, Bonins, Aleutians, Kuriles); the South and South West Pacific (Australia, New Guinea, the Bismarcks, the Solomons, New Hebrides, New Caledonia, Fiji, Samoa). Bases and anchorages first, then straits, then islands.

```
Task: for PART, the places a campaign of 1941 to 1945 would name:
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
and west negative, and the source's own form in "as printed". The quote
for a gazetteer entry is the entry's name and its coordinates.
Start with: the GEOnet Names Server (NGA, US government) for positions;
the Hydrographic Office Sailing Directions of the period; Building the
Navy's Bases for wartime names; the JICPOA island studies.
```
