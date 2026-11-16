# How often search aircraft flew, how tired their crews were and how far bombers searched, 1942

Status: research note of 16 November 2026. It proposes nothing binding. Figures and rules are the architect's.

Asked by the designer: were pilots really so fatigued that a search unit could not fly after two or three days, and were bombers used for search, and to what extent? The rows are in `engagement_events.csv` ids 2266 to 2323 (a new engagement, `search-tempo-1942`, for the South Pacific rows; the Midway, Coral Sea, Santa Cruz and doctrine rows are under their own engagements). Five sources are new (`sources.csv` 1464 to 1468) and ten gaps are in `gaps.csv`. Every quotation was checked by program against the page text fetched on 16 November. Nothing here rests on Wikipedia. One row family rests on a popular book (the Bruning excerpt, low confidence, unconfirmed) and is marked.

Sites that refused: the US Naval Institute (403 to a plain fetch) and the Air Force Historical Support Office's PDF of "The AAF in the South Pacific to October 1942" (`dafhistory.af.mil`, 403). Neither was worked round. The HyperWar index lists Japanese Monograph 96 (Southeast Area Naval Air Operations, August to October 1942, a daily journal of the Japanese naval air units), but its page and PDF return 404; Monograph 98 is a 137 MB image scan with no text layer. Nothing from `references/` or any game database is in the rows.

## The short answers

**Were crews so fatigued that a search unit could not fly after two or three days?** The record read does not say so. What it shows:

- At Midway the station put up 22 PBYs on 30 May, 21 on 31 May, and 22 on each of 1, 2, 3 and 4 June, to 700 miles, every dawn: six days running, with 21 or 22 aircraft each day (rows 1182, 2292 to 2296). The one dip, to 21, follows two PBYs put out of action by Japanese patrol planes on 30 May. The search was cut to 10 aircraft and 250 miles on 5 June (row 2297) because the enemy was by then located; no source gives crew tiredness as the reason.
- In the South Pacific the patrol planes were "making daily searches" on 23 August with 39 PBYs and 30 B-17s on strength (row 2284), and a Navy squadron-group commander said in April 1943 that since 7 August they "have never missed a day of search" (row 2286). That is a daily search kept up for about 8.5 months by the force, with no count of aircraft or crews.
- B-17s searched from Espiritu Santo daily from 7 August. Their colonel rotated squadrons: "regularly relieved his units after 1 week in the forward area", to a rest base (rows 2272, 2273). So a unit's stated tempo was about 7 days of daily flying and then relief.
- What is said about tiredness is about the B-17s and is general: the missions' length "induced crew fatigue and strain" which hurt bombing accuracy (row 2278); the routine "wore down machines even more rapidly than men" (row 2275). No number of days, no hours limit and no remedy is stated. Of 11 B-17s lost from 31 July to 20 August, 10 were not lost in combat (row 2274).
- Crews: the only figure is for Oahu in December 1941, "scarcely more than one crew" per plane; sustained daily patrol would have needed the crews in three groups, which would have flown 20 aircraft a day of about 60 (rows 2322, 2323). Nothing for 1942.
- Henderson Field's dive-bomber squadron had "only three extra crews for the nine SBDs", so "some of the men would have to fly every day" (row 2314; low confidence). The pilot the excerpt names flew both searches on 28 August.

So the sources do not support "grounded after two or three days". They do show a cost, mostly on bombers, and they say the daily search was kept up by rotating squadrons and by flying a part of each unit on a given day. They also do not rule out tiredness: no source gives crew hours, and none was found that counts crews.

**How much of an attack force went on search?**

| Force and date | Search | Held or struck | Share on search | Rows |
| --- | --- | --- | --- | --- |
| B-17 group, 31 July to 6 August | 22 search sorties | 56 bombing sorties | 28 percent of sorties | 2267 |
| B-17s on Espiritu Santo, mid-August | 4 a day | 6 held for the strike, of 14 on the field | 29 percent of aircraft on the field; 4 not accounted for | 2271 |
| B-17 group, Harmon's basis, 22 October | 8 aircraft a day, 11 hours each | the rest of the flying | 78 percent of the group's effort | 2276 |
| B-17s, end of November | 6 a day (4 and 2) | strike missions "curtailed" | not stated | 2280 |
| Lexington, 8 May (Coral Sea) | 18 SBDs, 360 degrees to 200 and 100 miles | 16 SBDs in the attack group (or 24 in the other narrative) | 18 of 34 SBD sorties, 53 percent; 31 percent of the 59 aircraft launched in the two groups | 2305 to 2307 |
| Enterprise, 24 August (Eastern Solomons) | 20 SBDs at 0630; 23 aircraft at 1315 | 25 held (11 SBD, 7 TBF, 7 F4F), Saratoga's group struck | afternoon: 23 of 48, 48 percent; morning: 20 SBDs | 144, 157, 238 |
| Midway, 3 June | Yorktown's planes searched | Enterprise's and Hornet's air groups held as an attack force | one carrier of three | 2303 |
| Santa Cruz, 25 October | Enterprise did "all" searches and patrols | Hornet kept a strike group on the alert | one deck of two | 2308 |
| Henderson Field, 28 August | 7 SBDs in the afternoon search (and a morning search) | 9 SBDs held | 7 of 16, 44 percent (low confidence) | 2310, 2311 |
| Midway, daily | PBYs (22 of 24 recommended) | B-17s kept out of the search | none of the bombers; "reduction in the striking force would be involved" | 2287, 2288, 2301 |

How a bomber's search differs from a carrier's: at Midway the bombers were kept out of the search to keep the striking force whole, and the PBYs, which cannot stand against fighters, did it (row 2301, 2296). In the South Pacific the bombers did it because "no other aircraft on hand could press home an effective search in the face of air opposition", at a cost the air commanders called "a serious diversion from the available striking power" (rows 2282, 2283). Harmon's ceiling was a quarter of heavy-bomber effort (row 2277).

## What the rows show, by question

### Patrol squadrons and flying boats

| What | Figure | Row |
| --- | --- | --- |
| Midway PBYs flying, 30 May to 5 June | 22, 21, 22, 22, 22, 22, then 10 | 1182, 2292 to 2297 |
| Midway plan of 23 May | about 23 aircraft a day on 180 degrees to 700 miles; total of 24 PBYs recommended | 2287, 2288, 1183 |
| Sortie shape at Midway | out at 100 knots, back at 111, off at dawn, home before dark; take-off "usually about 0415" | 1183, 2302 |
| VP-44 600-mile search, May | ordered 24, 27, 28, 29 May (11 or 12 aircraft); no sector listed 25 and 26 May | 1180, 1181, 2289, 2290, 2291 |
| VP-23 | daily 400-mile search of islands and reefs, 19 to 29 May | 2291 |
| South Pacific, 23 August | 39 PBYs and 30 B-17s, "making daily searches" | 2284 |
| Ndeni, 5 August | five patrol planes, with McFarland | 2285 |
| Days running | "never missed a day of search" from 7 August to April 1943 (a commander's statement) | 2286 |
| Oahu, January 1942 | 25 PBYs and 12 B-17s daily to 700 miles, 290 degrees; 109 long-range aircraft on hand (34 percent flying) | 2321 |
| Nimitz's letter of 7 January 1942 | about 50 planes a day for 800 miles, and "three times the number needed for one day" on hand | 2319, 2320 |
| Japanese seaplanes, Rabaul, 7 August | 2 Type 97 on a 700-mile sector, a second search at 0900 | 2316, 2317 |
| Japanese flying boats from Wake | every morning on 14-hour patrols (monograph); alternate days (US inference) | 1145, 2304 |

Hours per sortie: the 700-mile outbound leg at 100 knots is 7 hours, 6.3 back at 111 (plan arithmetic, row 1183), and the Japanese monograph's 14 hours for Wake (row 1145) agrees with it. The Rabaul seaplanes were at 700 miles 4 hours 10 minutes after launch, which is 168 knots (row 2317); that is faster than a flying boat of the type cruised, so the miles or the clock are doubtful (inference only).

### Bombers on search

The B-17 rows are above. In addition: the first seven days gave 22 search sorties to 56 bomb sorties (row 2267); two aircraft searched to 700 miles on D-day (row 2268); "day after day" 0300 take-offs and 1,600 miles flown (row 2270); at most nine search planes were up at one time (row 2279). At Midway the B-17 crews flew about 30 hours in the two days before the battle, including delivery from Hawaii (row 2299), and about 4 hours each morning airborne for safety, not search (row 2300).

### Carrier aircraft on search

Counts already on file (rows 125 to 158, 1165 to 1176, 1190 to 1195): Enterprise 8 SBDs on 23 August, 20 and 23 aircraft on the 24th; 16 SBDs on 26 October in pairs; Yorktown 10 SBDs at 1130 on 4 June; Enterprise 18 SBDs on 6 June. New: Lexington's 18 SBDs on 8 May with the attack group's make-up (rows 2305 to 2307); which carrier searched and which held a strike (rows 2303, 2308); the Marine morning search that kept Saratoga's group waiting on 24 August (row 2318).

For the Japanese carrier forces: the existing rows give 7 attack planes from Shokaku and 12 from Zuikaku on the morning of 24 August, with 4 vanguard floatplanes later (rows 366, 368 to 370); and at Midway 2 carrier attack planes (Akagi, Kaga) with 4 or more floatplanes of Tone and Chikuma on 4 June (rows 1133, 1146). The number of attack planes each carrier had, which a share needs, is not in any row (gap).

### Henderson Field, August to November

A daily afternoon search is not stated by any source I could open; the Air Force study that the leads name refused the fetch. What is in hand: "long single-plane searches" from the first week (row 1253, existing); occasional night searches by mixed squadrons by 30 August (row 2309); Mangrum's system, from a popular book, of two searches a day to 250 miles with the rest of the squadron kept armed on the strip (rows 2310 to 2314); and the Army history's statement that New Zealand Hudsons from 26 November "relieved" the Marine SBDs "from much of their search responsibility" (row 2281). So the SBDs carried the Henderson search from late August until late November.

### Doctrine in its own words

- Midway, the reason for a daily 700-mile search: each day's search must reach so far that an undiscovered force could not close within 200 miles before the next (rows 1159, 2302).
- Midway, the cost of using bombers: "reduction in the striking force would be involved" (row 2301).
- Army air commanders, South Pacific: "Air search properly should be carried out by patrol planes, or by shore-based reconnaissance aircraft and float planes" (row 2283); the PBY could not hold contact against fighters (row 2282).
- Nimitz, 7 January 1942: three aircraft on hand for each one flying a day (row 2320). This is a summary of the letter by the Committee, not the letter.

### Days with no search or a reduced one

Reduced: 5 June at Midway (10 aircraft, 250 miles; row 2297); 4 June radius cut to 425 miles (row 1162, with the station's own listing of 700, row 2296); VMSB-232's first days at Henderson, "only a few planes for a token search" (row 2315, low confidence); two aircraft only on D-day for the B-17s (row 2268). No dated day of no search, with a stated reason, was found. 25 and 26 May at Midway show no VP-44 600-mile sector in the diary (row 2291), and the diary gives no reason.

## What the sources disagree on

- **4 June at Midway:** 22 PBYs (the station's listing, row 2296; the Naval War College plan, row 1184) against 11 (the Marine monograph, row 2298). The radius is 700 miles in the listing and 425 in the ONI narrative (row 1162). Both left as they are. Eleven may be one group of the 22.
- **Wake flying boats:** every morning (Japanese monograph, row 1145) against alternate days (the Midway staff's inference from contact times, row 2304).
- **Lexington's attack group on 8 May:** 41 aircraft with 16 SBDs (row 2306) against 46 with 24 SBDs (row 2307), both in the ship's own report.
- **Harmon's 78 percent:** he called his basis "somewhat academic" (row 2276); the same history gives 29 percent of the aircraft on Espiritu Santo in mid-August (row 2271) and 28 percent of sorties in the first week (row 2267). The three are different measures, not a conflict.

## What is missing

See `gaps.csv`. Chiefly: any day-by-day series for a patrol squadron in the South Pacific; crews per aircraft in 1942; a stated hour limit; a dated case of crews worn out and what was done; Japanese flying-boat and floatplane search by day (the Japanese naval air monograph is not on the site); Henderson's daily SBD searches; the size of each carrier's air group on each search day; the Santa Cruz morning search of 25 October; and a dated day of no search with its reason.

## Against the game's figures

The rule (`docs/rules/05-air-operations.md`, "Fatigue gained and lost"; `data/reference/fatigue.toml`, placeholders): a group that flew gains 3 plus 2 for each 100 nm of its longest sortie in a daylight pulse, the sortie of a search being twice the arc's range; one that did not fly loses 5 in a daylight pulse; nothing changes at night; it flies its ready aircraft times (100 less fatigue) over 100; and every aircraft able to fly goes on each search in both daylight pulses. Arithmetic is mine, from those figures.

| Case | Game, flying both pulses each day | What the record shows |
| --- | --- | --- |
| PBY searching to 700 nm (Midway) | Sortie 1,400 nm: 3 + 28 = 31 a pulse. Fatigue 0, 31 after the morning, 62 after the afternoon. Day 2 flies 38 percent of ready aircraft in the morning and 7 percent in the afternoon. After about 1.5 days the group is nearly grounded. | One sortie a day of about 14 hours by 21 or 22 aircraft for six days (rows 2292 to 2296); not both pulses. If the group flew one pulse a day, it would gain 31 and lose 5 in the other pulse, 26 a day: 26, 52 and 78 after days 1 to 3, flying 74, 48 and 22 percent of ready aircraft on days 2, 3 and 4; the record has 22 of 22 on day 4. |
| Flying boat to 400 nm (the designer's example) | 800 nm sortie, 19 a pulse, 38 a day, 76 after two days; flying 100, 81, 62, 43 percent of ready aircraft through the four pulses of two days, then 24 and 5 percent on day 3. | No 1942 source for a 400 nm flying-boat search by day. The Oahu 1941 estimate is one crew in three, so one day in three. |
| Rest | 5 a daylight pulse, 10 a day: 76 takes about 8 days to come back; the commission's "about four days" would be 19 a day. I give the rule as written. | The B-17 rotation in the record is 1 week forward then relief to a rest base (row 2272). The length of the rest is not stated. |
| B-17 to 800 nm | Sortie 1,600 nm: 3 + 32 = 35 a pulse; grounded in about 1.5 days of two pulses. | 8 aircraft a day on 11-hour missions from August to October (rows 2270, 2276); a squadron's stint in the forward area 7 days (row 2272). |
| SBD to 200 to 250 nm | Sortie 400 to 500 nm: 11 to 13 a pulse. | Enterprise searched on both pulses of 23 and 24 August with different squadrons' aircraft; Henderson's SBDs flew two searches a day to 250 nm with a few spare crews (rows 2312 to 2314). |
| Share of aircraft | All able aircraft of the group go in each search, both pulses. | The record has a part of the unit on search on any one day: 4 of 14 B-17s (row 2271), 7 of 16 SBDs in the afternoon (row 2310), 22 of the PBYs at Midway (rows 2292 to 2296) of 24 planned (row 2288), and one carrier of three searching while the others held their groups (rows 2303, 2308). |
| Searches per day | Two (morning and afternoon pulses) for every aircraft. | One sortie a day for a PBY or B-17; two searches a day for a Henderson squadron and a carrier, by different aircraft (rows 2312, 2305, 157). |

What the arithmetic shows and the rows cannot settle: the game's gain per pulse for a long search is about a third of the whole scale, so a group that flew every pulse would be at the cap in 3 to 4 pulses, while the units in the record flew once a day for 6 days (Midway PBYs), for about a week before relief (B-17s) or for months of "never missed a day" (the force). The record shows no sign of such a decline, but it also shows rotation and part-force flying that the game's search rule does not have, and it has no crew or hour count to measure fatigue itself. No figure or rule is proposed here.
