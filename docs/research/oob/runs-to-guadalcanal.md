# Runs to Guadalcanal and Tulagi, 7 August to 30 November 1942

Status: research report for the architect, read on 2026-11-15. Nothing here is a ruling. Rows are `unconfirmed` unless the row says otherwise; where sources disagree each has its own row.

## What was asked

Every supply and reinforcement run to Guadalcanal and Tulagi by either side from 7 August to 30 November 1942: the ships, the close escort, any covering force, the cargo or men landed, how long the ships lay off the beach and what became of the run; and the rule each side worked by for escort, cover and timing.

## What was added, and how it was read

| File | Rows | What |
| --- | --- | --- |
| `troop_lift_facts.csv` | 51 (ids 390 to 440) | Escorts, cover, times, outcomes and conduct of runs |
| `lift_entities.csv` | 9 (ids 1282 to 1290) | Nine runs the earlier rows had no key for |
| `land_unit_facts.csv` | 2 (ids 1385, 1386) | Turner's refusal of civilian-manned ships; the break in the flow of supplies when Fletcher withdrew |
| `passages.csv` | 7 (ids 387 to 393) | Sailing legs with a stated hour at one end |
| `gaps.csv` | 14 | What was looked for and not found, and three pages that would not open |
| `sources.csv`, `ship_facts.csv`, others | none | Every source cited is already in `sources.csv` |

Every quotation was checked by a program against a copy of the page downloaded on 2026-11-15, with the page number taken from the printed-page markers in the text (a page is the one after the marker, as in the earlier rows). Pages came from the HyperWar transcriptions of Miller, Hough, Zimmerman, Dyer, the Strategic Bombing Survey and DANFS, from combinedfleet.com's tabular records, uboat.net and Pacific Wrecks. Five candidate rows turned out to be already held word for word and were not added again: Hough's order of 12 August (land_unit_facts 1173), Miller's 3,200 tons for the 164th Infantry (359), the Survey's 900 men a night (troop_lift_facts 26), "the nocturnal Tokyo Express" (22) and the 228th Infantry landings (land_unit_facts 15).

Three pages would not open (`gaps.csv`): Dyer as a PDF on marines.mil (HTTP 403), General Miyazaki's report as a PDF on upload.wikimedia.org (larger than the fetch tool takes), and three history.navy.mil pages (certificate, connection). Nothing is taken from them. Dyer's passages were read in the HyperWar transcription instead.

## What the record gives: the runs

Rows: `T` is `troop_lift_facts`, `L` is `land_unit_facts`, `P` is `passages`. Rows below 390 were held before this pass. A blank is a gap.

### American runs

| Date | Ships | Close escort | Cover | Cargo or men landed | Time off the beach, and what became of it | Rows |
| --- | --- | --- | --- | --- | --- | --- |
| 7 to 9 Aug | 9 transports and 6 cargo ships (Guadalcanal), 3 transports, 4 APDs and 1 cargo ship (Tulagi) | Crutchley's force | Three carrier groups | Almost 17,000 men; less than half of 60 days' supplies and 10 units of fire | Unloading stopped for the night of 7 to 8 Aug (supplies piled up on the beach); ten transports left at 1500 on 9 Aug, the rest at 1830; Fomalhaut left with about 15 percent of her cargo out | T 105, 106, 396, 397, 395 |
| 15 Aug | 4 APDs of Transport Division 12 (not named) | none stated | Aircraft of Task Force 63 planned | 120 tons (gasoline, oil, bombs, spares), 120 aviation ground men; 30 tons the most an APD could take | Arrived after dark; hours not stated | T 86 to 89, 390, 391; L 1173 |
| 21 Aug | Colhoun, Gregory, Little (rations), McFarland (aviation material) | none stated | | 120 tons of rations | not stated | T 60 to 62; L 1123, 1124 |
| 22 to 23 Aug | Alhena, Fomalhaut | 3 destroyers (Blue, Helm, Henley by uboat.net; "destroyers" by Miller) | Smith's fighters from Henderson Field | "some supplies and weapons", by day; with the APDs and McFarland "over 2,000 tons", 200 of them rations | One destroyer (Blue) torpedoed on the 22nd, scuttled the 23rd; hours not stated | T 60, 392, 393, 394; L 1125 |
| 29 Aug | 7 destroyer-transports and destroyers; Burrows and Kopara | none stated | | Colhoun carried 17 tons when sunk on 30 Aug | Burrows and Kopara to unload by day at Guadalcanal, at night at Tulagi; Burrows aground 31 Aug with 60 percent unloaded | T 398, 399, 400; L 1126, 1135, 1136 |
| 30 Aug | Alhena (Zimmerman's date) | | Aircraft | MAG-23 forward echelon and supplies | Disputed date (see below) | T 401 |
| 31 Aug to 2 Sep | Betelgeuse; three YPs; Fomalhaut | | | | | L 1127 to 1129 |
| 7 to 8 Sep | Fuller, Bellatrix | destroyers (not named) | | | Back at Tulagi on 9 Sep, unloaded 1630 to 1900 after leaving for a night action | T 402; L 1130, 1159 |
| 14 to 18 Sep | 6 transports (7th Marines) | 3 cruisers (Leander named), destroyers, minesweepers | Hornet and Wasp group southeast; Covering Force of carriers | 4,180 men (Miller), 4,262 (Hough, Zimmerman), 3,665 (division report); 8,825 tons of rations by Miller | Sailed 0515 on the 14th; retired 1500 on the 15th for 24 hours; at Kukum 0550 to 1800 on the 18th, planned for at most 12 hours; left at 1800 with part of the men, tanks, vehicles and rations aboard | T 63, 64, 403 to 408; L 612 to 614, 621, 624, 629, 630; P 387 |
| 24 to 29 Sep | Betelgeuse; Alhena | one destroyer each | | | Betelgeuse 0628 to 1830 on the 24th, out each night, empty at 1500 on the 26th; Alhena in on the 26th, out at 2045, empty at 1200 on the 29th | T 409; L 1160 to 1164 |

### American runs after September

| Date | Ships | Close escort | Cover | Men and cargo landed | Time off the beach, and what became of it | Rows |
| --- | --- | --- | --- | --- | --- | --- |
| 13 Oct | McCawley, Zeilin | 3 destroyers, 3 minelayers | 4 cruisers, 5 destroyers (Scott), left flank | 2,852 men of the 164th Infantry, 210 air wing, 85 casuals; over 3,200 ships' tons | 0547 to before nightfall; two air raids; left completely unloaded | T 65 to 68, 410 to 412; L 358, 359; P 388 |
| 2 Nov | Alchiba, Fuller | | | stores, ammunition, two 155 mm batteries | | L 1169 |
| 4 Nov | 3 transports, 2 APDs (Aola Force) | | | | | T 92 |
| 11 Nov | Zeilin, Libra, Betelgeuse (Scott's group) | not stated; Zeilin left with 1 destroyer | Callaghan's warships; South Pacific aircraft | | Off Guadalcanal 0530; two raids; left 1800; Zeilin returned to Espiritu Santo | T 433, 434; L 1167; P 390 |
| 12 Nov | McCawley, President Jackson, President Adams, Crescent City, Betelgeuse, Libra | destroyers | Callaghan's and Scott's warships | About 6,000 men; shares of cargo landed: 100, 100, 80, 50, 40, 20 percent | Anchored 0530; stopped two hours; withdrew at 1815 for fear of the enemy battleships | T 69, 435, 440; L 1165 to 1168; P 389 |

### Japanese runs

| Date | Ships | Close escort | Cover | Men and cargo landed | Time off the beach, and what became of it | Rows |
| --- | --- | --- | --- | --- | --- | --- |
| 16 Aug | 1 destroyer (Oite) | none | none; Dyer: "unsupported daylight landing" | 200 troops and supplies | | T 135, 136 |
| 18 Aug | 6 destroyers (Ichiki's first echelon) | | | 900 (also 916, 1,000) | Landed at Taivu Point | T 39 to 44, 138 to 142, 179 |
| 19 to 25 Aug | Kinryu Maru and 4 old destroyer transports (4 transports by the Survey) | Jintsu and 3 destroyers (4 by the Survey and Toyama) | 2 carrier groups; Chitose; 25th Air Flotilla search | About 1,500 men planned | Bombed 25 Aug; the landing "cancelled" at 1200 by order, "retired to the Shortland Islands" | T 2 to 6, 438, 439 |
| 29 Aug | 4 destroyers (Fubuki, Shirayuki, Kawakaze, Umikaze) | | | 900 at Tassafaronga | | T 45, 148, 149 |
| 30 to 31 Aug | Yudachi; 7 or 8 destroyers | | | 130; 1,200 at Tasimboko | | T 47, 150, 151, 153 to 156 |
| 28 Aug | Desdiv 20 (4 destroyers), then 3 destroyers | | | 550 and 350 planned | Aborted; reason in the rows | T 143 to 147 |
| 2 to 11 Sep | 2 to 6 destroyers a night; 61 barges | | | 150, 1,000, 231 or 370, 370, 637; 1,100 by barge | | T 159 to 172 |
| 21 and 24 Sep | Kagero; Kawakaze, Umikaze | | | | Strafed or bombed while landing; two damaged | T 427 |
| 11 to 12 Oct | 1 transport and 2 destroyers | | | | Left ignored at 0230 | T 421, 422 |
| 12 Oct | Convoy from Shortlands | | | | Recalled part way, back the same day; no reason given | T 420 |
| 13 to 14 Oct | 6 transports, 8 destroyers (combinedfleet.com); 5 transports, 11 warships (Miller) | Akizuki and 7 destroyers under Takama | 11th Air Fleet and R-Area floatplane fighters; carrier fighters on the 14th | About 4,500 men planned; 3,000 to 4,000 landed and 80 percent of cargo; tank fuel and ammunition lost | Arrived at midnight; beached 3 ships; two left 1742 on the 14th | T 10 to 12, 182, 413 to 419; P 391, 392 |
| 28 Oct to 8 Nov | Tokyo Express | | | Elements of the 228th Infantry | | L 15 |
| 13 to 15 Nov | 11 transports | 11 destroyers (Miller 12) | 11th Air Fleet and R-Area; "23-ship convoy" sighted 0849 on the 14th | About 2,000 troops by dawn (combinedfleet.com); about 4,000 and 5 of 10,000 tons (Miller) | Seven sunk at sea, four beached by order at Tassafaronga on the 15th | T 13 to 19, 428 to 432; P 393 |
| 29 to 30 Nov | 8 destroyers | Tanaka in Naganami | none (flares only) | None landed that night | Returned to Buin; landed 3, 7 and 10 Dec | T 50, 183, 436, 437 |

## Escort and cover, set beside the transports

- **American, August.** The runs of 15, 21 and 29 August and the first runs generally name no escort. The only escort named is that of Alhena and Fomalhaut on 21 to 22 August: Blue, Helm and Henley (uboat.net, row 393). Cover is stated as aircraft: Task Force 63 for the APDs (Miller), Smith's fighters from Henderson Field for Alhena on the 22nd (Zimmerman).
- **American, September to November.** From 14 September each run has an escort and cover stated: three cruisers, destroyers and minesweepers for six transports on 14 September; three destroyers and three minelayers for two transports on 9 October, with four cruisers and five destroyers on the flank; a covering force of Callaghan's and Scott's warships for the transports of 11 and 12 November. Betelgeuse and Alhena at Lunga each had one destroyer.
- **Japanese, August.** Destroyers carried the men without further escort (6 on 18 August, 1 on 16 August, 4 on 29 August). The convoy of 19 to 25 August had a light cruiser and three or four destroyers, and two carrier groups.
- **Japanese, October and November.** Convoys had a destroyer squadron escort (8 destroyers with 6 transports on 13 October; 11 with 11 on 13 November) and shore-based air cover. The runs of 29 to 30 November had no transports and no cover.

## The rules stated at the time

These are statements found, not rules the sources call rules. None gives a number of escorts or says when a covering force was required.

| When | Whose | Statement (as quoted in the rows) | Row |
| --- | --- | --- | --- |
| 12 Aug | Ghormley's plan, Miller | the destroyer-transports "were to leave Espiritu Santo in time to reach Guadalcanal in the late afternoon, and to depart from there early in the morning"; "Aircraft of Task Force 63 were to cover them" | T 390 |
| 12 Aug | Ghormley's order, Hough | "to reach Sealark Channel late in the day, unload under cover of darkness, and depart early the following day" | L 1173 |
| 13 Aug | Zimmerman | each APD could take 30 tons | T 391 |
| 29 Aug | 1st Marine Division operations journal | Burrows and Kopara to unload by day at Guadalcanal, by night at Ringbolt | L 1135 |
| Aug | Turner, by Compton | "no civilian manned ships should go up at that time" | L 1385 |
| 15 Sep | Turner's decision (Dyer, staff log) | withdraw temporarily "in hopes of more favorable opportunity for reinforcement" | T 403 |
| 15 to 18 Sep | Turner's plan (Miller) | "no more than twelve hours unloading off Kukum", to "clear Sealark Channel before nightfall to avoid enemy warships" | T 408 |
| 24 Sep | Turner | vessels carrying supplies to Lunga Point "were to unload only during daylight"; if not unloaded "retire eastward out of the channel at night and return to the Point at dawn" | L 1160, 1161 |
| 12 Nov | Turner | to avoid destruction by the enemy battleships "he decided to withdraw all the cargo ships and transports" | T 435 |
| Japanese, Aug to Oct | Survey, p. 115 | destroyer transports left the Shortland area and went through the inside passage "so as to reach the landing area at night"; landings were "generally accompanied by bombardment of the airfield" | T 423 |
| Japanese, Sep to Oct | Hough | "gunfire ships had to come late and leave early to avoid the U.S. planes in daylight" | T 425 |
| Japanese, Oct | Miller | the Express "could not carry heavy equipment or large amounts of supplies", which is why transports were used on 14 to 15 October | T 22 |
| Japanese, Aug | Toyama | "We did not have air support for a landing" | T 3 |

## Where the sources disagree

- **The 22 to 23 August escorts.** Miller says "destroyers" and one torpedoed and sunk; uboat.net names Blue, Helm and Henley, with Blue torpedoed by what it says was thought to be a motor torpedo boat. A search summary of an NHHC H-Gram attributed it to Kawakaze; that page would not open and nothing is taken from it.
- **Alhena on 30 August.** Zimmerman puts Alhena with the MAG-23 echelon on 30 August (row 401) and on 22 August; the division's calendar has no Alhena on 30 August (L 1124 to 1130).
- **Fuller and Bellatrix.** 8 September (Zimmerman, row 402) or 7 September (calendar, L 1130).
- **The 7th Marines.** 4,180, 4,262 or 3,665 men (L 612 to 614).
- **The October convoy.** Six transports, eight escorts, 13 to 14 October (combinedfleet.com); five transports, eleven warships, 15 October (Miller, whose footnote cites a source saying six ships); departure 14 October and arrival 15 October (Pacific Wrecks, row 419, alternative).
- **The November convoy.** Eleven destroyers (combinedfleet.com) or twelve (Miller); about 2,000 troops by dawn on 15 November (combinedfleet.com) or about 4,000 and five of 10,000 tons (Miller). The pages count 23 ships against 22.
- **The 15 August cargo.** Dyer's "120 tons of aviation gasoline, lubricating oil, bombs, spare parts" and 120 men; Zimmerman's 30 tons a ship (row 391) makes 120 tons for four.

## What is still a gap

- The names of the four APDs of 15 August and of the three more of 21 August; the escorts of the runs of 15, 21 and 29 August and of Burrows and Kopara; the destroyers escorting Betelgeuse and Alhena on 24 to 29 September.
- Hours off the beach for every run before 14 September, and a total for 12 November.
- A rule of the Amphibious Force in its own words for the number of escorts or the need for a covering force; a Japanese order in Japanese words for escort, cover or clearing by a stated hour. The orders themselves (Turner's war diary and operation plans, the 8th Fleet's and 17th Army's orders) were not read.
- The reason for the recall on 12 October and the ships in it.
- A night-by-night Japanese record for 1 October to 12 November.
- General Miyazaki's "no more than an average of 20 percent", which would not open.
