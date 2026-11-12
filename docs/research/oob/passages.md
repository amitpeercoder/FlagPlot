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
