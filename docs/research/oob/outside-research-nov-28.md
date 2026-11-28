# Outside answers of 28 November 2026 (1942 on one map), checked and keyed

Status: research report for the architect, keyed on 2026-11-28. Nothing here is settled. Every new row is `unconfirmed`, except where `oob-check` requires another standing; those are listed under "Standing set by the checker".

An outside model answered the 26 runs of `prompts-1942.md` (W01 to W10): directives, timetables, the position on 1 May, what came and went, Papua, the air war, places, oilers, intelligence and aircrew, 1 May to 30 November 1942. Each answer is a table of statements. A statement is a lead, not a source. The answer files, the readers' checks and the saved documents stay outside the repository.

## How it was read

- **The readers.** Eight readers (R1 to R8) opened the cited documents, saved them with their text, and gave a verdict on every statement (Table A of each check). Each reader also gave statements found on the same pages (Table B, "found beside a lead") and the documents opened (Table C).
- **The keying.** Six workers (Sonnet agents) each took a group of answer files. Each staged rows from the documents' own words, never from the answer's or the reader's paraphrase. A row was staged only from a statement marked `verified` or `verified, corrected`, or from Table B. A corrected statement carries the correction.
- **The merge.** The lead merged the six stagings, removed rows staged twice by two groups, assigned every id and wrote the rows. Every quotation was checked by program against the reader's saved text, allowing only whitespace, typography and a hyphen at a line break; `...` marks an elision. Every passage a note rests on was checked the same way.
- **Each note** begins with where the row came from: "Lead from an outside research pass (W.. part .., statement ..)" or "Found beside a lead (W.. part ..)". It ends with the sentence that the quotation was checked by program.

**The quote check.** A program of the lead's re-searched every new row's `as_stated` in its saved text after the merge. 435 rows were checked and 435 passed.

**One misreading found by the lead.** A check of where each quotation starts found one row that matched inside a longer date. The AWM Papuan timeline heads its entry "25/26 August 1942". A worker had read "26 August 1942". That row was dropped. The timeline's entry is keyed once, with its time empty and the heading in the note (`engagement_events` 18141).

**What the check does not show.** It does not show that a value was filed against the right unit, ship or date. That filing is the workers'. Each note says what the passage around the quote says.

## Counts

A "statement" is one row of an answer's Table 1. "Verified" counts every statement the reader marked `verified` or `verified, corrected`; "corrected" counts the second kind. "Already held" means the same source's same statement is already a row in `data/oob`; it was not keyed again. A row that rests on two answer files is counted under each, so the last two columns add to 450, not 435.

| File | Statements | Verified | Corrected | Not keyed, and why | Rows from statements | Rows from beside |
| --- | --- | --- | --- | --- | --- | --- |
| W01 part 1, Allied directives | 23 | 23 | 3 | 0 | 13 | 12 |
| W01 part 2, Japanese directives | 27 | 27 | 6 | 0 | 17 | 6 |
| W02 part 1, Japanese timetables | 24 | 24 | 7 | 0 | 26 | 13 |
| W02 part 2, Allied timetables | 12 | 12 | 3 | 0 | 14 | 11 |
| W03 part 1, Allied ships | 5 | 5 | 3 | 0 | 4 | 13 |
| W03 part 2, Japanese ships | 10 | 10 | 2 | 0 | 10 | 13 |
| W03 part 3, air units | 4 | 4 | 0 | 0 (two type rows held back, below) | 2 | 7 |
| W03 part 4, land garrisons | 13 | 13 | 5 | 0 | 15 | 13 |
| W04 part 1, Allied ships | 10 | 10 | 1 | 2, already held | 11 | 6 |
| W04 part 2, Japanese ships | 3 | 3 | 0 | 1, already held | 2 | 7 |
| W04 part 3, air units | 9 | 5 | 0 | 4, not a permitted source | 6 | 10 |
| W04 part 4, land movements | 20 | 20 | 2 | 0 | 19 | 15 |
| W05 part 1, Japanese in Papua | 23 | 23 | 5 | 0 | 19 | 9 |
| W05 part 2, Allied in Papua | 6 | 6 | 1 | 0 | 6 | 7 |
| W05 part 3, ground supply | 5 | 5 | 0 | 0 | 5 | 8 |
| W06 part 1, Allied air war | 7 | 7 | 1 | 0 | 8 | 11 |
| W06 part 2, Japanese air | 3 | 0 | 0 | 3, not a permitted source | 0 | 0 |
| W07 part 1, central Pacific | 16 | 16 | 2 | 0 | 15 | 12 |
| W07 part 2, South Pacific bases | 5 | 5 | 1 | 0 | 5 | 10 |
| W07 part 3, South-West Pacific bases | 6 | 6 | 0 | 0 | 7 | 8 |
| W08 part 1, Allied fuel | 2 | 1 | 0 | 1 already held; 1 not a permitted source | 0 | 0 |
| W08 part 2, Japanese oilers | 12 | 12 | 1 | 6, already held | 7 | 4 |
| W09 part 1, Allied intelligence | 7 | 7 | 2 | 0 | 9 | 6 |
| W09 part 2, Japanese intelligence | 13 | 13 | 1 | 0 | 12 | 9 |
| W10 part 1, American aircrew | 4 | 4 | 1 | 3, already held | 1 | 3 |
| W10 part 2, Japanese carrier aircrew | 9 | 1 | 0 | 8, not a permitted source | 1 | 3 |
| **All** | **278** | **262** | **47** | **16 not permitted; 13 already held** | **234** | **216** |

**Several statements, one row.** An answer often split one sentence into several statements (who issued an order, to whom, the date, the task). Those became one `order` row, and the note's origin names every statement it covers. Army Order No. 626 (W01 part 2, statements 1 to 4, and W02 part 1, statement 6) and Navy Order No. 18 (W01 part 2, statements 5 to 8) are examples.

**Rows staged twice.** Two groups read the same page for nine statements: Crowl and Love on Nauru, Ocean Island and Makin (W02 part 1 and W07 part 1); Milner on the Yokoyama Force (W04 part 4 and W05 part 1); JM-93's cancellation of K Operation (W02 part 1 and W09 part 2); Willoughby's estimate (W05 part 1 and W09 part 1); and 75 Squadron's move to Gurney Field (W06 part 1 and W07 part 3). Each is one row naming both origins. Two rows that restated the 164th Infantry's departure, already keyed as events, were dropped.

## Where the rows went

| File | Ids | What |
| --- | --- | --- |
| `engagement_events.csv` | 18000 to 18201 | Orders, plan dates, postponements, cancellations, estimates and the steps of the timetables |
| `land_unit_facts.csv` | 18000 to 18095 | Garrisons by place (Morton's Table 4), defence battalions, Papua by unit, rations and evacuations on the track |
| `ship_facts.csv` | 18000 to 18063 | Movements in and out of the map, formations, the Japanese oilers' fuellings |
| `place_facts.csv` | 18000 to 18039 | Fields and runways of Hawaii, Palmyra, Fiji, Efate, Espiritu Santo, Milne Bay, Dobodura, Kokoda, Pongani and the Gilberts |
| `air_unit_facts.csv` | 18000 to 18027 | Bases and strength by date, the RAAF squadrons, Air Group 10, the Japanese carrier air groups after Midway |
| `passages.csv` | 18000 to 18002 | Long Island, San Diego to Pearl Harbor; Nashville, Hawaii to Dutch Harbor; Yamato, Kure to Truk |
| `training_facts.csv` | 18000, 18001 | Takata on the Japanese carrier pilots after Midway |
| `engagements.csv` | 1018 to 1034 | 17 operations (below) |
| `sources.csv` | 16224 to 16248 | 25 new sources (below) |
| `land_units.csv` | 14954 to 14991 | 38 new land units |
| `places.csv` | 6117 to 6138 | 22 new places |
| `air_units.csv` | 14101 to 14106 | 6 new air units |
| `ships.csv` | 17245 to 17248 | Nashville, I-162, Long Island, Neches |
| `gaps.csv` | last 18 lines | 16 statements not keyed, and the two aircraft marks not printed |

## The ruling on operations, and the new event kinds

By the architect's ruling for this pass, an operation, a directive, a plan and its timetable are not entities. The operation is a row of `engagements.csv`. What was ordered, planned, changed, cancelled, estimated and done is a row of `engagement_events.csv`. The ruling and the columns of each kind are now in `database-design.md`, under "Operations, directives and plans".

| Kind | Rows | New |
| --- | --- | --- |
| `order` | 36 | no |
| `other` | 32 | no |
| `estimate` | 28 | yes |
| `plan_date` | 23 | yes |
| `sailed` | 19 | yes |
| `landing` | 16 | no |
| `arrived` | 15 | yes |
| `postponement` | 9 | yes |
| `cancellation` | 7 | yes |
| `assembly` | 5 | yes |
| `report` | 4 | no |
| `loading`, `rehearsal` | 1 each | yes |
| `strike_attack` 2, `strike_launch`, `search_launch`, `sighting`, `readiness` 1 each | 6 | no |

`fuelling` was added to the list and not used: the oilers' fuellings went to `ship_facts` `fuelling_note`, as the rows already on file do.

**The new engagements.** Each event's note gives its document's clock. No document read states a zone for a whole operation, so every new engagement's zone is empty. A comment line in `engagements.csv` says so.

| Id | Key | What |
| --- | --- | --- |
| 1018 | `mo-operation` | The Japanese Port Moresby operation, with Tulagi |
| 1019 | `mi-operation` | The Japanese Midway operation (JM-93's "AF (MIDWAY) Operation") |
| 1020 | `ao-operation` | The Japanese Aleutians operation (JM-93's "AO (ALEUTIANS)") |
| 1021 | `fs-operation` | The Japanese plan against New Caledonia, Fiji and Samoa |
| 1022 | `ry-operation` | The Japanese occupation of Nauru and Ocean Island |
| 1023 | `ri-operation` | The Japanese landing at Buna and the overland advance on Port Moresby |
| 1024 | `re-operation` | The Japanese landing at Milne Bay |
| 1025 | `ka-operation` | The Japanese recapture of Guadalcanal (one row so far, an Allied estimate) |
| 1026 | `watchtower` | Task One, the Guadalcanal and Tulagi operation as directed and prepared, and its reinforcement |
| 1027, 1028 | `jcs-task-two`, `jcs-task-three` | Tasks Two and Three of the Joint Directive of 2 July 1942 |
| 1029 | `providence` | The Allied plan to occupy Buna |
| 1030 | `papua-campaign-1942` | Allied orders and steps in Papua outside PROVIDENCE |
| 1031 | `south-pacific-garrisons-1942` | Allied orders for the island garrisons |
| 1032 | `comint-1942` | Signals intelligence and security (the call-sign change of 1 October) |
| 1033 | `gilberts-reinforcement-1942` | The Japanese reinforcement of the Gilberts after the Makin raid |
| 1034 | `papua-air-war-1942` | Air balance and raids over Papua outside the other operations |

The keys `ry-`, `ri-`, `re-`, `ka-` and `fs-operation` are the brief's. No document read prints those code letters, so the names do not claim them. Each engagement's `date_from` and `date_to` are the first and last dates of its keyed events.

## Attributes added

- **`position_note`** (`place_facts.csv`; text, no unit): where a source places an island by distance and direction in its own words. Two rows use it, for Johnston and Palmyra. No existing attribute holds a position without coordinates, and `place_positions.csv` takes printed latitude and longitude only. It is added to the list in `step2-record.md`, which is the vocabulary `place_facts.csv` names.
- No other attribute was added. Every "(new)" name of the prompts was mapped to an existing attribute or to an event kind. `land_unit_facts` also uses `casualties_total` and `evacuated_men_cumulative`, which older rows already use but `land-forces.md` does not list.

## New entities

- **Ships:**
  - `cl-43-nashville` and `cve-1-long-island`, with the class left empty;
  - `ao-47-neches`, with the class left empty (DANFS names the Mattaponi class, which is not on file);
  - `i-162` (I-62 until 20 May 1942).
- **Air units:** `98th-bs` and `42d-bs` (parent `11th-bg`), `27th-air-depot-group`, `8th-service-group`, `76-sqn-raaf`, `40th-fs` (parent `35th-fg`).
- **Land units:**
  - Nine "forces at a place" for Morton's Table 4: `us-army-forces-` with hawaii, canton, new-caledonia, efate, fijis, christmas-island, bora-bora, australia and tongatabu.
  - `nz-forces-viti-levu`.
  - Marine defence units: the 2d, 7th and 8th Defense Battalions, the 2d and 3d Marine Brigades, and the Samoan Area Defense Force.
  - The New Caledonia Task Force and the 43d Division.
  - Australian: the 7th and 18th Brigades, the 55th, 2/25th and 2/31st Battalions, MAROUBRA Force and New Guinea Force.
  - Japanese: the Yokoyama Force, the 1st and 2d Battalions of the 144th Infantry, the 9th Company of its 3d Battalion, the 15th Independent Engineer Regiment, the 41st Infantry (the Yazawa Detachment), its 3d Battalion, the 3d Battalion of the 229th Infantry, the Eighteenth Army, the "Stanley detachment" (as Williams prints it), and the Japanese forces on the Kokoda trail.
  - American: the 126th and 128th Infantry.
- **Places:**
  - Hawaii and the central Pacific: hilo, palmyra, johnston-island, dutch-harbor.
  - The Gilberts: gilbert-islands, makin, tarawa, apamama, nauru, ocean-island.
  - Fiji: viti-levu, nandi, nandi-airfield, narewa-airfield, nausori-airfield, lauthala-bay.
  - New Caledonia: new-caledonia, plaines-des-gaiacs.
  - Papua: gurney-field, dobodura, kokoda, pongani.
  - `nandi` is the place a squadron "landed at" and `nandi-airfield` the field described with its runways. They are two rows because the two documents name two things.

## New sources

- **Official histories (B unless noted):**
  - `morton-strategy-command`, `matloff-snell-strategic-planning-1941-42`, `williams-army-chronology`, `crowl-love-gilberts-marshalls`, `macarthur-reports-v1`, `conn-guarding-united-states`;
  - `cmh-papua-campaign-booklet` (CMH Pub 72-7), `papuan-campaign-cmh-1945`;
  - `ghq-fec-ijn-in-wwii-1952`, `japanese-monograph-88`;
  - `building-bases-v2-ch26` (A);
  - `nhhc-photo-80-g-10121`, a Navy photograph's caption page.
- **Scholarly and reference (B):** `vego-fleet-versus-fleet-pacific`, `williams-kokoda-thesis-2008`, `james-awm-track-2009`, `maurer-air-force-combat-units`.
- **The AWM's Wartime magazine (web compilation, B):** `awm-wartime-59-james-clowes`, graded as `awm-wartime-93`, which is on file.
- **Web summary pages (tertiary, C, every row low):** `awm-unit-75-squadron`, `awm-unit-76-squadron-raaf`, `awm-remembering-1942-milne-bay`, `awm-papuan-campaign-timeline`, `dva-anzac-portal-kokoda-conditions`, `dva-anzac-portal-milne-bay`, `raaf-history-75-sqn-arrival` and `nhhc-midway-forces-assemble`. Each is its own source, never the book behind it.
- Where a saved page does not print an author, publisher or year, the source's note says the value is the standard citation.
- Two keys staged for the same page were made one: three keys for the AWM's 75 Squadron page and two for its Milne Bay article.
- A chapter key for Craven and Cate vol. IV was mapped to the existing `craven-cate-v4`, with the chapter in the locator.

## What was not keyed

**Not a permitted source (16, all in `gaps.csv`):**

- **W04 part 3, statements 5 to 8:** Miyazaki's interrogation (USSBS 446, Nav. No. 97), read only as an image-only Wikimedia upload. It was the source for the redesignation of the Fourth, Yokohama and Tinian air groups in November and the 25th Air Flotilla's return to Japan. An official copy is wanted.
- **W06 part 2, statements 1 to 3:** the Bullard translation, read only as a Scribd upload. It was the source for the 25th Air Flotilla's 23 Zeros and 7 flying boats on 15 May, and the "2nd Air Corps"' 15 Zeros on 6 August. `sources.csv` already holds `bullard-jpn-army-ops-south-pacific` from another copy, so pages 122 and 125 could be read there.
- **W08 part 1, statement 2:** Cimarron topping off Saratoga and eight destroyers on 3 August. The upload was not permitted. `bates-savo-1950` (DTIC) is on file and could be read for it.
- **W10 part 2, statements 1 to 8:** the Genda interrogation (USSBS serial 479), read only as a Wikimedia upload with no text layer. It was the source for the state of training of Carrier Divisions 1 and 2 in August and October 1942, their flying hours, and what became of the First Air Fleet's pilots. An official copy is wanted.

**Already held (13).** The same source's same statement is a row on file:

- W04 part 1, statements 8 and 9: `ship_facts` 4141 and 4143;
- W04 part 2, statement 1: `ship_facts` 5702;
- W08 part 1, statement 1: `place_facts` 258;
- W08 part 2, statements 1 to 4, 7 and 11: `ship_facts` 14474, 14478, 14483, 14495, 14600 and 14502;
- W10 part 1, statements 1 to 3: `air_unit_facts` 774 to 776.

Several Table B rows were also on file (`place_facts` 258, 259, 701, 14586 and 14587; `ship_facts` 674, 676, 4144, 5675, 5686, 14476 and 14500; `air_unit_facts` 768 to 773).

**Held back by the lead (2, in `gaps.csv`).** A worker filed the B-17s of the 435th Squadron as `b-17e` and the Kittyhawks of 75 Squadron as `p-40e`, "the nearest type on file". The pages print no mark. That is an identity the document does not give, so neither row was keyed. The P-40E of 76 Squadron is keyed, because the AWM page prints "Kittyhawk P-40E".

**Not a row, from the readers' notes:**

- the Army chronology's "Operation Plan No. 1-42" of 16 July;
- JM-45's Army-Navy agreement "published" on 28 July;
- the Chronology's landing "near Gona";
- Operation Order 2-42's 22,000 Army troops at Noumea;
- Cressman's Nashville grounding and I-21;
- Melson's Cockrell;
- Milner's Hawkins estimate of 6 November.

These were seen on the saved pages but were in no table. They are leads for the next pass.

## What the corrections were

No quote was invented. No source says the opposite of the statement that cites it. The corrections were of these kinds.

**Pages one out.** HyperWar's markers close the page they follow, so many pages were one out. Examples:

- Milner ch. III p. 41, not 42; ch. V pp. 66 and 67, not 68 and 69;
- JM-45 p. 85, not 74; p. 72, not 72 to 73;
- JM-93 pp. 7 and 8, not 6 and 7;
- the Army chronology p. 46, not 45;
- Milner ch. X p. 146, not 145;
- Dyer p. 447; Vego printed p. 47 (PDF p. 72).

**Entity or author:**

- The 28 May sailing is Task Force 16 (Enterprise and Hornet), not Yorktown (W02 part 2, statement 3).
- The Amphibians Came to Conquer is by George C. Dyer (statement 11, `dyer-actc`).
- The Air Group 10 row is "the displaced Navy crews" (W10 part 1, statement 4).
- Nagumo's report is Part III, not Part V (W09 part 2, statement 8).

**A quotation that is a table, not a sentence (W03 part 4).** The five garrison statements quote a sentence that is not on Morton's page. The figures are cells of Table 4, "Army strength in Pacific, April 1942", p. 224. Each row's `as_stated` is the table row as printed, for example "Hawaii 62,700 8,900 7,300 --- 100,000 15,000". The value is the Present Ground figure, and the note says the row is a table cell. The four Table B rows of the same table (Christmas, Bora Bora, Australia, Tongatabu) are keyed the same way.

**Dates that are the outside model's own** (the date stays out of the row, and the note marks it "not printed"):

- W01 part 1, statement 19: 12 June, printed as "the following day";
- W04 part 3, statement 4: 25 July;
- W04 part 4, statement 7: 20 July, "four days later";
- W05 part 2, statement 5: 16 to 30 November;
- W07 part 1, statement 4: 27 August, "two days earlier";
- W07 part 1, statement 16: the airfield's start is not printed, so the row is dated at the detachment's arrival on 17 July;
- W07 part 3, statement 6: the withdrawal from Kokoda "four days later", keyed as a second row with an empty time;
- W10 part 1, statement 4: 26 October is not the landing date, so the row runs from 1942-10.
- W09 part 2, statement 4: Nagumo's estimate is undated. Its time is empty, and the note says the date is open.

**Readers' slips the workers found:**

- Toei Maru's fuelling of 17 October ended at 2350, not 2335.
- Mersky p. 15 does print 26 October, though as the day the battle began, not the day the crews landed.
- The Santa Cruz page is the ONI Combat Narrative (`oni-cn-santa-cruz`), not a book by the authors the reader named.

## Disagreements left for the architect

Both rows are keyed and neither is settled.

1. **The Buna convoy of July.** Morton has the convoy leaving Rabaul on 20 July and arriving at 1900 on the 21st, with a plan for 3,300 men (`engagement_events` 18072, 18073, 18071). The Army chronology has it sailing on 19 July with about 1,800 men (18074). Milner gives the Yokoyama Force as about 1,800 (`land_unit_facts`, the Yokoyama Force row of 1942-07).
2. **The order of 18 July to land at Buna.** JM-45 says the 17th Army gave it (18042). Morton says only that the detachment "was directed", after an Imperial General Headquarters decision (18069).
3. **The 164th Infantry leaving New Caledonia.** Morton gives 8 October (18106). The Americal history gives 9 October (18102), and so does the Army chronology (`land_unit_facts` 18045). Already on file: Miller, loaded on 8 October and sailed the next morning (`land_unit_facts` 342); Hough, embarked on 8 October (354); 9 October (844).
4. **The 8th Defense Battalion at Wallis.** Melson gives 27 May (`land_unit_facts` 18017). Cressman gives 1 May (18018). Hough gives "at the end of the month" (18019), with the month read as May from the sentence before. All three are low.
5. **US Army in Fiji.** Morton's Table 4 gives 10 Present Ground in April (18004), and its Suva row is the same. Craven and Cate give 600 men in all by May, including airdrome personnel (18010). The two figures count different things on different dates.
6. **The 18th Brigade at Milne Bay.** The AWM timeline gives 18 August (18035). The Army chronology gives 21 August (18036), and Milner has the brigade's move completed "by 21 August" (18074).
7. **The Japanese landing at Milne Bay:**
   - Milner: about 2200, 25 August (`engagement_events` 18139);
   - the AWM article: late on the night of 25 August (18140);
   - the AWM timeline: "25/26 August" (18141);
   - the Army chronology: the convoy came after nightfall on 26 August (18142);
   - the Anzac Portal: 26 August (18162);
   - the AWM 75 Squadron page: the early hours of 25 August (18163).
8. **The 98th Squadron in New Caledonia.** Wings at War gives 21 July at Plaines des Gaiacs (`air_unit_facts` 18006). The Army chronology gives 22 July (18010).
9. **75 Squadron at Milne Bay.** The AWM unit page gives 21 July (`air_unit_facts` 18014). On file, McCarthy has both squadrons arriving on 24 July (`place_facts` 83 and 87). The AWM 76 Squadron page has its move "in August".
10. **Efate's airfield.** Dyer has it in use since 28 May (`place_facts` 18021). On file, the first planes landed on 8 May (`place_facts` 9).
11. **Espiritu Santo's first strip.** Dyer has bombers operating from it from 30 July (`place_facts` 18022). On file, the first fighter squadron landed on 28 July and B-17s the next day (`place_facts` 20).
12. **The attack of 2 May on a Japanese submarine.** combinedfleet's I-24 page names I-24 (`ship_facts` 18020). Cressman, not keyed, names I-21.
13. **Platte at Noumea.** Operation Order 2-42 expects her on 13 May (`ship_facts` 18009). CINCPAC on 26 April gave 12 May (`ship_facts` 14387). Both are plans made on different dates.
14. **Long Island in July.** One DANFS entry has her leaving San Diego on 8 July for Pearl Harbor on 17 July (`passages` 18000). The same entry has her leaving the formation on 17 July and returning to the west coast (`ship_facts` 18034). Both rows are low.
15. **Air Group 10 at Henderson Field.** Row 18025 dates the stay from 1942-10. `air_unit_facts` 739, from the same Mersky passage, dates it from 1942-10-26 and is marked inferred. No row on file was changed.
16. **Takata.** He gives "30% killed, 40% injured" (`air_unit_facts` 18026) and "30% to Japan, 40% to the Third Fleet" (`training_facts` 18001). The same two numbers stand for two different things. Both rows are low.
17. **Buna in November, three Allied estimates.** Willoughby, about 4,000 on 10 November (`land_unit_facts` 18067). Vasey, 1,500 to 2,000 on 14 November (`engagement_events` 18178). Harding's G-2, "not more than a battalion", undated (18169).

## Judgements made that the brief did not settle

These are the workers' and the lead's.

1. **The Joint Directive of 2 July** is three `order` rows, one each in `watchtower`, `jcs-task-two` and `jcs-task-three`. Its purpose and forces paragraphs are in the Task One note. Its rows are `high`, because they are the directive's own text as Morton prints it.
2. **Navy Order No. 18** covers Midway and the Aleutians. It is filed once, in `mi-operation`.
3. **A request that is not an order,** a decision to proceed, and Nimitz's comment are `other`.
4. **The time of an event:**
   - a range of days ("10-12 August", "21-22 July") takes the month or the first day, with the days in `as_stated`;
   - a relative date ("four days later", "the following day") leaves the time empty.
5. **Estimates.** An estimate goes in the engagement of the operation estimated. Japanese beliefs of the American carriers go in `midway`, and the Tulagi report of 7 August in `watchtower-landing`.
6. **Willoughby's estimate** is a land fact, `strength_men_estimated_by_enemy`, because the attribute exists. Vasey's and Harding's are `estimate` events.
7. **Land units' positions** use `position_on` (a place and date), and arrivals at a port use `landed_at`, as rows on file do. The vocabulary has no `based_at` for a land unit.
8. **Conflicting dates.** In a group of conflicting dates, workers graded every side `low`, including rows that a row on file agrees with.
9. **DANFS rows are `medium`.** The rows already on file from it are mostly `high`.
10. **Feet are converted to metres** for `runway_length_m` and `runway_width_m`, as rows on file are, with the printed feet in `as_stated`.
11. **One row from one sentence.** The 13 November fuelling of three ships by Shinkoku Maru is one row, because the record is one entry.
12. **Takata's statements.** His navy-wide statements went to `training_facts` (`ijn-carrier-aviators`, `training_note`). His statements about the carriers' air groups went to `air_unit_facts` `losses_before_scenario_note` on `mobile-force-carrier-air`.
13. **The 41st Infantry** and "the Yazawa Detachment" are one entity, because Milner equates them in one sentence.
14. **The 27th Air Depot Group** "arrived early in December from Brisbane". The paragraph is about Port Moresby, so the row is `based_at` `port-moresby` with `basis` `inferred` and a note saying so (`air_unit_facts` of 1942-12).
15. **Two-column PDFs.** Where a PDF's text layer interleaves two columns (Maurer; the Japanese loss table of `ghq-fec-ijn-in-wwii-1952`), the row was checked against the reader's one-page extract of the same document.
16. **Nashville's class** is left empty, although DANFS prints "cl. Brooklyn". No `ship_class` fact row was keyed for it.
17. **Rows that duplicate a row on file** were not keyed again. A second row of one source's one statement adds nothing.

## Standing set by the checker

`oob-check` requires a value to be `preferred` when one row at medium or high confidence supports it, in the files it holds to the rule. That set 81 new rows to `preferred`: 62 of `ship_facts.csv` and 19 of `air_unit_facts.csv`. The other new rows of those two files stand `unconfirmed`, and so does every new row of the files the rule does not reach.

No new row's value clashes with a chosen row on file, so no row on file was changed.

## What the checks said

- `cargo run -p oob-check -- data/oob`: no problems.
- `oob-derive --check` for `savo_island`, `rabaul_guadalcanal`, `coral_sea` and `midway`: each passed. The output is identical to the output before the new rows, so no derived file changes. Every other list in `data/derivation` passed too.
- `cargo test -p oob-check` passed. `cargo test --workspace` was run in full, and every test passed except two:
  - `scenario_40_one_pool_two_hexes::a_game_saved_in_the_middle_replays_and_the_staff_checks`;
  - `scenario_42_interrupts_and_runs::d_every_game_of_42_saves_and_the_staff_checks`.
- Those two fail together on a clean `main` too, without these rows. Each passes when run alone. They interfere when run at the same time, and neither reads `data/oob`.
- The program re-search of every new row's quotation: 435 of 435 passed.
- The diff under `data/oob` is added lines only: 565 rows and one comment line in `engagements.csv`.

## What was not done

- **No disagreement is settled,** and no row on file was changed, including `air_unit_facts` 739.
- **Not keyed:** the answers' Table 3 ("not found") rows, and the statements seen beside a lead but in no table (listed above).
- **No `place_positions` row:** no document in this pass prints a latitude and longitude.
- **No transport, class or derivation row:** nothing here feeds a derivation list.
- **The 16 statements of non-permitted uploads** wait for official copies.
