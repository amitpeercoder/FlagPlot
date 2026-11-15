# How many aircraft were sent against what was reported, 1942

Status: research report for the architect, read on 2026-11-15, for decision 0118 (a strike sized to its target). Nothing here is settled. The rows are 58 events in `data/oob/engagement_events.csv`, ids 2208 to 2265; four new sources (ids 1362 to 1365 in `sources.csv`); one new engagement key in `engagements.csv` (id 1011, `pacific-air-doctrine`, a place to hold doctrine rows, not a battle); six gaps at the end of `gaps.csv`. No new attribute was needed.

How the pages were read: each was fetched whole with a plain request and its text compared with the quotation on the row (whitespace, footnote marks and the printed page numbers aside). The doctrine compilation is a scan with no text layer; it was read by OCR and the page image was looked at to confirm the figures. Printed page numbers are those of the page's own footer. "Miles" in the US sources are taken as nautical, as in `launch-decisions.md`; no range is entered on these rows.

## What the rows are

A `strike_launch` row says what was sent: its `value` is the total aircraft launched against the target, and its `target` begins with what the report held before launch ("as reported before launch: ..."). The composition by type is in the `as_stated` quotation and the `note`. Aircraft kept back and the reason are on `other` rows where the source states them, or in the note. Where a figure for aircraft held is a subtraction from an air group's organization, the note says it is the researcher's. Rows stand `unconfirmed`, except the five Lexington counts, which disagree with one another and stand `disputed`.

## The strikes

"Attack aircraft" is dive bombers plus torpedo planes (and the horizontal bombers of the Midway land force); fighters are separate. "Per ship" is attack aircraft over the ships reported in the contact before launch, as the source states them; where the report named only a part ("2 carriers and battleships") a count is not worked and the cell says so. Row ids are in the last column.

### American carriers and land-based groups

| Battle, date, unit | Reported before launch | Attack aircraft sent | Fighters sent | Held, stated | Attack aircraft per reported ship | Rows |
| --- | --- | --- | --- | --- | --- | --- |
| Coral Sea, 7 May, Lexington (captain) | 2 CV, 2 CA, 2 DD | 40 (28 VSB, 12 VTB) | 10 | 8 SBD for anti-torpedo patrol | 6.7 per ship (6 ships); 20 per carrier if only carriers counted | 2208 |
| Coral Sea, 7 May, Lexington (air operations officer) | 2 CV, 4 CA, destroyers not filled in | 42 (30 SBD, 12 TBD) | 10 | 8 F4F patrol, 8 SBD anti-torpedo patrol (row 2122) | 5.3 per ship for 8 named ships; 21 per carrier | 2209 |
| Coral Sea, 7 May, Yorktown | 2 CV | 34 (24 VB/VS, 10 VT) | 8 | not stated (organization: 11 of VB-5 and 9 fighters left, by subtraction) | 17 per carrier | 2210, 2211 |
| Coral Sea, 8 May, Yorktown | 2 CV, 4 CA, many DD | 33 (24 + 9 VT) | 6 | the whole group ordered ("entire group including torpedo planes") | 5.5 per ship for 6 named ships | 2212 to 2214 |
| Coral Sea, 8 May, Lexington (captain) | 2 CV, 4 CA, 3 DD | 36 (24 SBD, 12 VTB) | 10 | not stated | 4.0 per ship (9 ships) | 2215 |
| Coral Sea, 8 May, Lexington (air officer; air operations officer) | 2 CV, 4 CA | 27 (15 + 12) in the air officer's count; 28 in the air operations officer's | 9; 13 | 4 fighters launched as relief patrol | 4.5 to 4.7 per ship for 6 named ships | 2216, 2217 |
| Midway, 3 June, 9 B-17 (land) | 11 ships, "main body" | 9 (half bomb loads) | none stated | not stated | 0.8 per ship | 2219 |
| Midway, 4 June, Midway planes (land) | 2 carriers and the main body, 180 miles | 4 B-26 and 6 TBF, 27 Marine dive bombers, 15 B-17 | none stated | every aircraft able to fly was in the air | not worked (main body not counted) | 2220 to 2224 |
| Midway, 4 June, Enterprise | 2 carriers and battleships | 47 (33 VSB, 14 VT) | 10 | not stated | not worked; 23.5 per carrier | 2225 |
| Midway, 4 June, Hornet | enemy carriers (two) | 50 (35 VSB, 15 VT) | 10 | not stated | 25 per carrier | 2226 |
| Midway, 4 June, Yorktown | two carriers | 29 (12 VT, 17 VSB) | 6 | 17 VSB, for two further carriers hoped for | 14.5 per carrier | 2227, 2228 |
| Midway, 4 June, evening, Enterprise | 1 CV, 2 BB, 3 CA, 4 DD (10) | 24 VSB | none | not stated | 2.4 per ship; 24 per carrier | 2229 |
| Midway, 4 June, evening, Hornet | 2 or 3 burning CV, 1 or 2 BB, CAs, DDs | 16 VSB | none | not stated | not worked (counts of ships not given) | 2230 |
| Midway, 5 June, Enterprise | 1 burning CV, 2 BB, 3 CA, 4 DD (10) | 32 VSB | none | not stated | 3.2 per ship; 32 per carrier | 2231 |
| Midway, 5 June, Hornet | 2 BB, 4 CA, 1 CV, some DD | 26 VSB | none | not stated | under 3.7 per ship (7 ships and some DD) | 2232 |
| Midway, 6 June, Hornet | first heard as 1 CV; corrected to 1 BB, 1 CA, 3 DD | 26 VSB | 8 ("in case of previously undetected air opposition") | not stated | 26 per ship first reported; 5.2 per ship as corrected | 2233, 2234 |
| Midway, 6 June, Enterprise | 2 CA, 2 DD | 34 (31 VSB, 3 VT) | 12 | not stated | 8.5 per ship | 2235 |
| Eastern Solomons, 24 August, Saratoga | 1 CV, 2 CA, 1 DD | 38 (30 dive bombers, 8 torpedo planes) | none named | held for more definite word, then for a second search | 9.5 per ship; 38 per carrier | 2240 to 2242 |
| Eastern Solomons, 25 August 0245, Henderson | 4 ships, probably destroyers | 8 dive bombers | none stated | not stated | 2.0 per ship | 2243 |
| Santa Cruz, 25 October, Enterprise | 2 CV | 18 (12 VSB, 6 VTB) | 11 | not stated | 9 per carrier | 2248 |
| Santa Cruz, 26 October, Hornet, two waves | Shokaku and what was thought Zuikaku | 39 (21 + 18) | 15 (8 + 7) | not stated | 19.5 per carrier | 2252, 2253 |
| Santa Cruz, 26 October, Enterprise | 2 CV | 12 (3 VSB, 9 VTB with the commander) | 8 | all flyable planes sent | 6 per carrier | 2249 to 2251, 2254 |
| Guadalcanal, 14 November 1220, Henderson | 26 ships, including 6 seaplane tenders | 25 (18 scout bombers, 7 torpedo planes) | 12 | not stated | 1.0 per ship | 2257 |
| Guadalcanal, 14 November 1255, Henderson | the transports; 12 transports and 6 to 8 light cruisers on arrival | 25 (17 scout bombers, 8 torpedo bombers) | 8 | not stated | 1.3 to 1.4 per ship (18 to 20 ships, as seen on arrival) | 2258 |

### Japanese strikes

| Battle, unit | Reported before launch | Attack aircraft sent | Fighters sent | Held | Attack aircraft per reported ship | Rows |
| --- | --- | --- | --- | --- | --- | --- |
| Coral Sea, 7 May dusk, Carrier Division 5 | "the reported American carriers" | 27 (12 dive bombers, 15 torpedo planes) | none (Zeros unsuited to night) | not stated | not worked | 2218 |
| Midway, Hiryu, 0758 Tokyo time | 1 carrier, 5 cruisers, 5 destroyers | 18 bombers | 6 | not stated | 1.6 per ship; 18 per carrier | 2238 |
| Midway, Hiryu, second | a carrier of the Enterprise class and one of the Hornet class | 10 torpedo planes | 6 | not stated | 5 per carrier | 2239 |
| Midway, first strike on the island | the island | 72 (36 bombers, 36 torpedo planes) | 36 | Organization No. 4: 12 fighters and 81 attack planes held ready against ships | not applicable (base) | 2236, 2237 |
| Eastern Solomons, 24 August, Carrier Division 1, first strike | the carriers of Task Force 61 | 27 dive bombers | 10 | torpedo planes held for follow-up | 13.5 per carrier for two | 2245, 2247 |
| Eastern Solomons, second strike | the same | 27 dive bombers | 9 | the same | not worked | 2246 |
| Santa Cruz, 26 October, Carrier Division 1, first strike | 1 Saratoga-class CV and 15 other ships | 41 (20 torpedo planes, 21 dive bombers) | 12 | second wave kept ready | 2.6 per ship; 41 per carrier | 2255 |
| Santa Cruz, second wave | the same | 20 dive bombers (Shokaku's), and Zuikaku's 16 torpedo planes later | 5 (and 4) | not stated | not worked | 2256 |

### Doctrine

The 1945 Air Intelligence compilation of Japanese doctrine, for a worked example against "3 carriers, 18 destroyers, and an air cover of 40 planes": 9 dive bombers per carrier to force it out of action (27), 3 per screening destroyer (54), 18 torpedo planes per carrier sunk (54), a fighter unit of about 40; 7 torpedo hits to sink a carrier, 3 bomb hits to put one out of action, losses of one third expected (rows 2260 to 2265). That is 27 attack aircraft per carrier, and 9 more for every six destroyers if the destroyers are counted at 3 each. **Date limit:** the compilation is of 10 February 1945, built on prisoner reports and captured documents, and the translated passages carry no date; it does not show a 1942 rule. Every doctrine row says so.

## What the sources disagree on

- **Lexington, 7 May:** the captain gives 28 VSB, 12 VTB and 10 VF at about 0925 on a report of two carriers, two cruisers and two destroyers; the air operations officer gives 3 + 16 + 11 SBD, 12 TBD and 10 F4F at about 0953 on a report of two carriers, four cruisers and a blank for destroyers; ONI (row 404) gives 10 fighters, 28 VSB and 12 VTB, 50 in all. The difference of two in the SBD count is the group command section. Rows 2208 and 2209, `disputed`.
- **Lexington, 8 May:** 46 (captain), 41 (air operations officer), 36 plus 4 relief fighters (air officer), 43 (ONI, row 407). Rows 2215 to 2217.
- **Yorktown, 7 May:** the group's own report (row 2210) and ONI (row 405) agree on 42 in total; Lexington's fighter director records "8 VF, 34 VSB and VTB" (not entered).
- **Reports before launch differ from the truth, and from each other.** Yorktown's scout reported two carriers on 7 May and the error (two destroyers) was found after launch; Hornet's scout on 6 June was heard as "1 CV" and had seen a battleship, a cruiser and three destroyers. Which report fixed the size cannot be told from the sources (row 2234).
- **Midway, Enterprise:** the ship's report (row 2225) and ONI (row 412) agree at 57 aircraft; ONI's times run about two hours earlier than the ships' (see `launch-decisions.md`).

## What the data shows about size

These are readings of the rows, not rulings.

- Every US strike of the carrier battles sent the whole of an attack group, or all that could fly, against what was reported; none was sized by ship. The Midway land force sent every aircraft able to leave the ground (row 2224). Fitch's order at Coral Sea on 8 May says "launch entire group including torpedo planes" (row 2214). The largest hold is Yorktown's 17 scout bombers at Midway, kept back for carriers not yet found (row 2228); Lexington kept 8 anti-torpedo SBD at Coral Sea (row 2208).
- Against a reported carrier, the carrier strikes sent about 14 to 25 attack aircraft per reported carrier at Midway (rows 2225 to 2227), 17 at Coral Sea (row 2210), 38 for Saratoga on a single carrier (row 2240), and 6 to 20 at Santa Cruz (rows 2248 to 2254). Japanese: 18 per carrier (Hiryu), 41 on a single reported carrier at Santa Cruz.
- A reported group of many escorts did not raise the number in a visible way: 2 CV with 4 CA sent 33 to 42; 1 CV with 9 other ships sent 24 to 32 (rows 2229, 2231); 1 CV with 15 other ships drew 41 from the Japanese. Strikes against one or two destroyers or cruisers are 8 (row 2243) to 34 (row 2235).
- The land-based groups at Guadalcanal sent about one attack aircraft per reported ship (rows 2257, 2258) and the B-17s at Midway under one per ship (row 2219).

## What is missing

See the new gaps in `gaps.csv`. In short: the aircraft ready aboard at each launch (so "held" is almost always by subtraction or silence); any 1942 standing allocation per ship kind; an after-action judgement naming a strike too small or too large for its target; a prelaunch division between two reported targets; and Japanese records of strikes beyond a website and a translation.

Not entered: the Japanese strike of 15 October 1942 on a destroyer and a tug (8 fighters, 21 dive bombers and 9 torpedo planes, the combinedfleet Zuikaku record), for lack of an engagement to hold it; it is the Japanese strike best matched to a small target.

## Sites and sources

- Scribd (a transcription of the Enterprise report on Santa Cruz): returned a page with no document text to a plain request. It was not worked round; the same report was read from the cv6.org copy through an Internet Archive page (source 1363).
- NHHC pages: not opened in this pass.
- Read: HyperWar (Lexington, Yorktown, Enterprise and Hornet reports; ONI chapters for Midway, Eastern Solomons and Guadalcanal), midway1942.com, govinfo, a Wikimedia Commons scan, combinedfleet, and an Internet Archive page.
- ONI Midway, on HyperWar: the Hornet launch is on printed page 20 and the Yorktown hold on printed page 21.
