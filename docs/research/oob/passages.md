# Recorded ship passages, July to October 1942

Status: research report for the session that owns the step 2 spec, read on 2026-11-12. Nothing here is settled. The rows are in a new file, `data/oob/passages.csv` (76 rows), with 24 new gaps and six places added to `places.csv` (`taivu-point`, `suva`, `koro-island`, `wellington`, `san-diego`, `pago-pago`). Five quotations and one page of a Japanese action report were checked a second time; the rest are as the researchers recorded them.

## The answer first

**No passage found has all four facts over 250 nm at one held speed.** One row in 76 is marked steady, and it is short. Almost every speed a source states is an order, a burst, a ceiling or a forecast, and where the same source gives positions and clock times the mean run is well below the stated speed.

What comes nearest, in the order I would use them:

| Row | Run | From, to | Times | Stated speed | Length | Why it is or is not usable |
| --- | --- | --- | --- | --- | --- | --- |
| 11 | Japanese 6th Cruiser Division, 11 October | The Shortlands to off Savo Island | 1200 to 2133, their clock | 24 knots, then 30 from 1600 | About 265 nm between the places in the data. The report also gives a fix on the way: at sunset, 1626, bearing 310 degrees and 200 nm from Guadalcanal | **Best check.** Two speeds, both timed, from the force's own report. 4 hours at 24 and 5.55 at 30 give 262 nm (arithmetic). Departure read twice |
| 32 | US transports, noon 1 August to noon 2 August | 19°03'S 179°09'E to 18°58'S 174°00'E | 24 hours | "Average speed ... 11.5 knots" | 292 nm between the fixes (arithmetic) | **Usable as an average,** not a held speed: both ends are noon positions and the source calls 11.5 an average. The fixes give 12.2. Zone not stated. Checked |
| 34 | The same force, noon 2 August to noon 5 August | 18°58'S 174°00'E to 16°34'S 159°00'E | 72 hours | 12.31 knots average | About 870 nm | An average over three days, with zigzagging and a fuelling stop; joins two sources |
| 58 | US Task Force 64, 11 October | 11°30'S 161°45'E to 9°43'S 159°26'E | 1600 to 2145 | 29 knots | About 175 nm | Under 250 nm, and it slowed to recover aircraft. The positions and times give about 30 knots. Sources disagree (29, 28, 20) |
| 47 | Saratoga after her torpedo, 31 August | 10°25'S 162°26'E to 12°43'S 167°13'E | 0748 to 0700 next day | 6, then 10 under tow, then 12 to 14 | 313 nm (arithmetic) | Not at one speed; partly towed. Mean 12.9 knots. Good as a damaged-ship example |
| 70 | McFarland, 14 to 16 October | Espiritu Santo to Lunga Point | 1505 to 1320 two days on | 17 knots ordered | 557 nm (arithmetic) | Speed and course changed; mean 12.0 knots |
| 71 | Two minesweepers towing PT boats, 23 to 25 October | Espiritu Santo to Tulagi | 1452 to 0515 two days on | About 18 knots | 564 nm (arithmetic) | Towing; the start hour is when they joined. Mean 14.7 knots |
| 55 | Transports with the 7th Marines, 14 to 18 September | Espiritu Santo to Lunga Point | 0515 to 0625 four days on | None | 557 nm (arithmetic) | Both ends timed; no speed; the convoy waited and turned back once. Mean 5.8 knots |

Distances marked "arithmetic" are straight lines between positions, mine or the researcher's, not statements by a source.

## What each kind of run gave

**Japanese destroyer runs to Guadalcanal (the request's first item).** The reinforcement group's own report for late August gives departures and arrivals (the Shortlands at 1000, Taivu Point at 2330 on 29 August, the arrival hour doubtful) and **no speed for any run.** Its daily log is times only. The destroyer squadrons' diaries for September and October, where the later runs would be, were not found. The ship records in the English compilation give dates without hours.

**US convoys (the second item).** Ends are well dated; speeds are not given. The 164th Infantry's convoy left Noumea on the morning of 9 October and was off Lunga at 0547 on the 13th: about 855 nm in a straight line, 8.4 to 9.5 knots as a mean. The transports' withdrawal after Savo Island: Lunga on 9 August to Noumea on 13 August, planned at 13.5 knots, a mean of about 9.

**Ships leaving for repair (the third item).** Speeds are stated; the legs they belong to are not timed at both ends.
- North Carolina, Tongatabu to Pearl Harbor: "a sustained speed of 18 knots and a maximum of 24" (checked). Arrival known, departure not found.
- Chicago after Savo Island: 12 knots by a dispatch, 10 by the 1950 analysis.
- Chester after her torpedo: 6 knots forecast.
- Enterprise on 24 to 25 August: 248 nm in 20 hours by two positions, a mean of 12.4 knots, with the air attack and her steering failure on the way.

**Forces with a stated speed between dated positions (the fourth item).** The Navy narrative says the carriers ran at 23 knots on 24 to 25 October; its own positions give 12.9 and then 11.5. A stated fleet speed in these sources is the speed through the water while zigzagging and working aircraft, not the speed made good.

**Long approaches.** Three task forces to the Fiji rendezvous in July: 3,100 miles in 10 days (12.9 knots) and 5,500 miles in 24 days (9.5 knots), dates only.

## Disputes

- **Task Force 64's run-in speed on 11 October:** 29, 28 or 20 knots by source; one intermediate fix in the narrative implies an impossible 37.
- **The patrol boats' arrival at Taivu Point on 30 August:** 2130 in the report's narrative, 2330 in its log.
- **Goto's departure from the Shortlands:** 1200 by his division's report; 1000 and 1400 in rows already in the data from Wikipedia.
- **Saratoga:** left Tongatabu on 10 or 12 September.
- **Koro Island to the turning point south of the Russell Islands:** "1,000 miles" in one history gives 8.7 knots; the narrative's noon positions give about 12.

## What this means for the rule's check

These are my readings; the check is the spec session's.

- **Speed made good ran at about half to three quarters of the speed a source states** for carrier forces and convoys, wherever positions let it be measured. A rule that charges a force its ordered speed in a straight line would overstate how far it gets.
- **A fast surface force on a short dash made what it claimed:** the Japanese cruisers on 11 October and the US cruisers the same evening both check to within a few miles.
- **The record will not supply a 250 nm run at one speed** from the sources reachable here. The nearest substitutes are the average-speed convoy legs (rows 32 and 34) and the two-speed dash (row 11).

## The file

`data/oob/passages.csv`, one row per passage per source. `oob-check` does not read it yet.

| Column | May be blank | Holds |
| --- | --- | --- |
| `id` | No | Whole number, stable, never reused |
| `subject` | No | A short name for the run |
| `ships` | No | Ship keys from `ships.csv` where the ship is in the data, else a plain name or formation; joined with `\|` |
| `from_place`, `to_place` | Yes | A key of `places.csv`. Blank when the source gives a position at sea or no place |
| `from_lat`, `from_lon`, `to_lat`, `to_lon` | Yes | Decimal degrees, south and west negative, only where the source gives them |
| `depart_time`, `arrive_time` | Yes | `YYYY-MM-DDTHH:MM` as printed; a date alone where the source gives no hour (flag `date-only`) |
| `zone` | Yes | The clock the source states. Blank means it states none; the note says what is presumed |
| `speed_kn` | Yes | The speed the source states |
| `speed_note` | Yes | Its words |
| `distance_nm_stated` | Yes | Only a distance the source states |
| `steady` | No | `yes`, `no` or `unknown` |
| `flags` | Yes | Why a run is not clean, joined with `\|`: `speed-changed`, `zigzag`, `waited`, `refuelled`, `towed`, `air-attack`, `date-only`, `time-inferred`, `from-not-located`, `to-not-located` and others in plain words |
| `as_stated` and the source columns | No | As in every fact file |
| `note` | Yes | |

Each end has a place, or a latitude and longitude, or the flag saying it is not located. Eleven rows join statements from two sources; they name one as the source, say so in the note, and are marked inferred.

## What stayed a gap

- A speed for any Japanese destroyer run; the 2nd and 3rd Destroyer Squadron diaries; two further volumes of the reinforcement group's report, found and not read.
- Hours for the Japanese long runs (the Shortlands to Truk, Truk to Kure).
- A US deck log with daily positions: the one transcription found has watch entries only.
- Departure times for the supply ships' runs; Boise's passage home; Enterprise's sailing date from Tongatabu.
- Passages by Australian and New Zealand ships were not looked for.

## Second pass, 2026-11-12: the Japanese destroyer squadron diaries

The archive's catalogue search was made to work, and it found the 2nd Destroyer Squadron's diaries for September and October 1942 (twelve volumes) and five more volumes of the reinforcement group's August report. **The 3rd Destroyer Squadron's diaries for August to October, which ran most of the September runs, are not in the catalogue:** its entries stop at May 1942. Rows added: `passages.csv` ids 77 to 129, with 18 new gaps. One daily page was read a second time by me and agreed; the researchers re-read every noon position at higher resolution, and most speeds once.

### The answer is unchanged

**Still no run with a start, an end, both times and one held speed.** No row in this pass is marked steady. Every speed found in the reinforcement group's volumes and the 4th Destroyer Squadron's October diary is an order or a plan (a convoy planned at 14 knots on course 120, for example), never a record of a completed run. The 4th Destroyer Squadron's own action reports for the runs of 11 to 16 October are cited in its diary and are not in the volumes found.

### What is new: a daily track from a force's own diary

The 2nd Destroyer Squadron's diary has a noon position for every day at sea. That gives noon-to-noon legs for two sorties from Truk, each beside the speeds the same pages state. The distances and the speed made good are my arithmetic on the positions; they are not statements by the source.

**September sortie with the Advance Force, 9 to 23 September** (flag in a destroyer; the squadron's cruiser was under repair at Truk):

| Noon to noon | Distance between the positions | Made good | Speeds stated on the pages | What the force was doing |
| --- | --- | --- | --- | --- |
| 10 to 11 Sep | 359 nm | 15.0 kn | 16 kn from 0545, 14 kn from 1700 | Running south, zigzagging |
| 11 to 12 Sep | 150 nm | 6.2 kn | 16 kn, then 12 kn | |
| 12 to 13 Sep | 424 nm | 17.7 kn | 20 kn from 0400 | Running south |
| 13 to 14 Sep | 74 nm | 3.1 kn | 28 kn, 24 kn under air attack, 20 kn | Off the Solomons, manoeuvring |
| 14 to 15 Sep | 361 nm | 15.0 kn | 12 kn refuelling, 14 kn | Withdrawing north |
| 15 to 16 Sep | 261 nm | 10.9 kn | 14 kn | |
| 16 to 17 Sep | 91 nm | 3.8 kn | 14 kn | Waiting |
| 19 to 20 Sep | 80 nm | 3.3 kn | 16 kn | Waiting; the operation was cancelled at 1038 on the 20th |
| 20 to 21 Sep | 367 nm | 15.3 kn | 16 kn | Returning to Truk on course 305 |
| 21 to 22 Sep | 375 nm | 15.6 kn | 16 kn, 18 kn from 1630 | Returning to Truk |

**October sortie, 11 to 30 October** (the bombardment of the airfield on the night of 13 to 14 October and the carrier battle of the 26th):

| Noon to noon | Distance | Made good | Speeds stated | What the force was doing |
| --- | --- | --- | --- | --- |
| 11 to 12 Oct | 395 nm | 16.5 kn | 20 kn from 0500, 16 kn about noon; 30 kn ordered at 0230, stopped at 1140; 14 kn from 1230 | Running south-east from Truk, zigzagging; refuelling from 1305 on the 12th. The 12 October page read twice |
| 12 to 13 Oct | 424 nm | 17.6 kn | 16 kn at 1800, 18 kn at 2300, 26 kn from 0330, 28 kn from about 0900 | Running south; the bombardment group left the main body at 0330 |
| 13 to 14 Oct | 74 nm | 3.1 kn | 26, 18 and 21 kn on the run in; 26 and then 29 kn retiring | To Guadalcanal and back in the 24 hours: a round trip, so the figure means nothing as a speed |
| 14 to 16 Oct | 87 and 157 nm | 3.6 and 6.5 kn | Read once | Refuelling; a second bombardment on the 15th |
| 16 to 26 Oct | 38 to 243 nm a day | 1.6 to 10.1 kn | 14 to 16 kn, read once | Cruising north of the Solomons, refuelling, waiting |
| 27 to 28 Oct | 381 nm | 15.9 kn | 18 kn on course 320 from 1200; 14 kn while refuelling from 2140 to 0855; 18 kn zigzagging from 0900. Read a second time by me | Returning north; refuelling overnight. The three timed speeds give about 386 nm (arithmetic), against 381 between the positions |
| 28 to 29 Oct | 399 nm | 16.6 kn | 20 kn, read once | Returning to Truk |

**What the daily track shows.**
- **On a passage, a force made good about 15 to 18 knots while the pages state 16 to 20:** six such days in September and October give 359 to 424 nm a day. Speed made good was 80 to 95 per cent of the stated speed, with zigzagging on. That is higher than the half to three quarters found for US carrier forces and convoys in the first pass.
- **In the operating area the daily run collapses** to 40 to 240 nm, at stated speeds of 14 to 16 knots, because the force was refuelling, waiting or doubling back. A day's stated speed says little about the distance between two noons there.
- **The bombardment run is fully timed** by the squadron's action report: the group left the main body at 0330 on 13 October at 26 knots, went to 28 at about 0900, was between Cape Esperance and Savo Island at 2237, opened fire at 2337, ceased at 0100 and retired at 26 and then 29 knots. It has no stated start position, and it changed speed, so it is a check and not a single-speed run.

**Usable as the first pass's rows 32 and 34 were: as averages between noon positions.** The six passage days (10 to 11, 12 to 13, 20 to 21 and 21 to 22 September; 11 to 12 and 12 to 13 October) each cover 359 to 424 nm between two positions from the force's own diary. Several lie west of 172°E, inside the map.

### Corrections to the first pass

- The circled numbers on the September track chart are courses, not speeds. No speed is printed on the chart.
- A position quoted in the first pass's notes for 15 September (11°37'S) was a misreading of 1°37'S; the page's own header gives 2°1'S 164°56'E. Both are recorded.

### Doubts

- Two positions are flagged as implausible and left out or marked: 18 September (the latitude as written does not fit the track) and 30 October (the position conflicts with the ship passing Truk's channel that afternoon).
- The two researchers read 11 October differently in one respect: one has the force leaving Truk's north channel at 0500 at 20 knots (the diary), the other has it sailing at 0130 with 20 knots from 0500 (the action report). Both are recorded.
- Speeds for 14 to 30 October were read once at low resolution and are graded low. The noon positions are the firm part.

### Not read

About 480 pages of telegram logs in the reinforcement group's and the 4th Destroyer Squadron's volumes; the 2nd Destroyer Squadron's signal-log volumes; the 4th Destroyer Squadron's signal pages in the volumes read in the third pass below.

## Third pass (12 November 2026): the runs from the Shortlands to Guadalcanal

**What was read.** Three volumes of the 4th Destroyer Squadron's action reports for 25 September to 9 November 1942 (JACAR C08030113100, C08030113300, C08030113400), found through the archive's catalogue, and the rest of the 2nd Destroyer Squadron's October action report (C08030098300, pages 39 to 72). Rows 130 to 170. All new rows stand `unconfirmed`. Each row's note says whether I read the page myself after the researcher or not.

### The runs, as the logs give them

Times are Tokyo time as printed. Tassafaronga is not a key of `places.csv`, so the Guadalcanal end of each run is named in the row's note and flagged as not located.

| Run | Left | Arrived | Speeds stated | Row |
| --- | --- | --- | --- | --- |
| High-speed convoy, Rabaul group, from north of Buka | 13 Oct 0945 | point A, 14 Oct 0400 | 14 kn | 130 |
| High-speed convoy, the Shortlands group | the Shortlands, 13 Oct 1700 | point A, 14 Oct 0400 | none | 131 |
| High-speed convoy, six transports, from point A | 14 Oct 0400 | Tassafaronga, 14 Oct 2200 | 14 kn, then 13 kn from 1245 | 132 |
| Kito Maru and Sado Maru with the escort, withdrawing | off Guadalcanal, 15 Oct 1545 | the Shortlands, 16 Oct 0830 | 16 kn | 134 |
| Reinforcement run, cruisers and destroyers | the Shortlands, 17 Oct 0215 | Tassafaronga, 17 Oct 2030 | 18 kn from 0350, 26 kn from 0540, 28 kn from 1245 | 135 |
| The same force returning | Tassafaronga, 17 Oct 2240 | the Shortlands, 18 Oct 0930 | 28 kn | 137 |
| Second Attack Unit (Akizuki, Yura) | the Shortlands, 24 Oct 1530 | position at 1800 | 18 kn, 19 kn from 1545 | 139 |

**What they show.**
- **Warships ran the Shortlands to Tassafaronga in about 18 hours out and under 11 hours back.** Outbound the force went north of the direct line and timed its arrival for dark; the return was one stated speed, 28 knots on course 320, by the central route. The return (row 137) is the nearest thing found to a single-speed run between two named places, but the log states the speed once, at the start, and its length is not stated.
- **The transports made 13 to 14 knots on the way in and 16 knots coming out.** The convoy took 18 hours from point A to the anchorage, under air attack twice.
- **Row 136 gives two timed positions six hours apart on the run in** (1200 and 1800 on 17 October, printed in the page heading), with a change from 26 to 28 knots at 1245 between them.

### The 2nd Destroyer Squadron, 15 to 30 October

Thirty rows (141 to 170) give the timed speeds of the force the squadron screened: 30 and 28 knots retiring from the bombardment of 15 October, 12 to 14 knots refuelling, 14 to 18 knots cruising and waiting north of the Solomons, 24 knots and more on 26 October. None has a start or end position, so they are speeds over timed stretches, to be read beside the noon-to-noon rows 88 to 102.

The date cells of this action report give the same daily positions as the war diary read in the second pass, for every day from 16 to 29 October. They are not entered a second time. For 30 October the researcher read 10°49.7'N where the diary row has 10°45.7'N; that position was already marked as doubtful.

### Doubts

- **The base is written as code letters** on the 4th Destroyer Squadron's pages (R×E as I read it, R×B as one researcher read it). The page headings name the Shortlands for the same days, and the rows say so.
- **Printed positions I could not read** are left out, not guessed: the noon positions of the convoy on 13, 14 and 15 October, a position written small beside point A, and the positions of 25 and 26 October in the Second Attack Unit's log.
- **One reading was wrong and is not used:** a researcher read the air attack of 1605 on 14 October as about 3 aircraft; the page says about 30. Attack sizes are not recorded in these rows.
- **Row 144's course** was read as 027 by the researcher and as 270 by me. The speed, 14 knots, is clear.
- **The 2nd Destroyer Squadron rows were read once by a researcher,** except rows 144 and 158. Rows the researcher graded low, and items with no legible time, were left out.
- **No track chart is attached to the convoy volume,** though its contents page lists one. The charts in the other two volumes (C08030113300 page 26, C08030113400 page 60) print times and courses but no speeds and no grid.

### Not read

The signal pages of the three volumes (about 100 pages), and volumes 1, 2 and 4 of the same file (barge transport, and the lessons of the convoy escort).

