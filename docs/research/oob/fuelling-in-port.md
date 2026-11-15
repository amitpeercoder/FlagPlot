# Fuelling in port, 1942

Status: a research report for the architect, read on 2026-11-15. Nothing here is settled and nothing here proposes a game figure. The rows are `place_facts.csv` ids 241 to 295, which `oob-check` does not read (it names no place file; `step2-record.md` says why). Every figure in the rates sections below is arithmetic on a quantity and a time that a source prints; no source prints a rate except the few in section 5.

## What was asked

Decision 0112 plans refuelling that takes time: a ship in port takes fuel through the hours of the day at the port's rate, at a berth or by lighter. It says "A fuelling rate by size of port has no row" and asks the record for the hours a carrier force, a cruiser force and a destroyer division took to fuel at Noumea, Espiritu Santo, Tulagi, Truk and Rabaul in 1942, from an oiler alongside and from a shore depot. This pass looked for:

1. dated fuellings with a quantity and a start and an end;
2. pumping rates of fleet oilers, fuel barges and shore lines, and what a ship of each kind could take in an hour;
3. how many ships could fuel at once, and the tankage, at the bases of both sides;
4. how long forces lay in port and how much they took;
5. any statement that fuelling delayed a sailing or was cut short.

## How it was read

Each page was fetched and read as text, and each quotation in a row was checked by program against the saved text of its page, allowing only a change of whitespace. All passed except the ones noted below.

- **HyperWar transcriptions** (Carter, Dyer, Building the Navy's Bases, Wildenberg, the South Atlantic administrative history, FTP 218): fetched as HTML. Page references are the transcription's `--N--` markers; where a passage lies between two markers the locator says so.
- **The Salt Lake City deck logs** (ussslcca25.com): fetched as HTML. The site gives selected entries by four-hour watch, not images of the logs.
- **The Aaron Ward war diary** (scottdavis61.com): fetched as HTML. It covers November 1942 only.
- **The tabular records of movement** on combinedfleet.com for the oilers Toa Maru, Iro, Hoyo Maru, Kyokuto Maru, Kokuyo Maru, Omurosan Maru, Nippon Maru, Toei Maru, Genyo Maru, Kamoi and Tsurumi: fetched as HTML and read for every fuelling in 1942 that gives a time or a quantity.
- **Two scans.** The Naval War College analysis of the Coral Sea (1947) was read through the scan's text layer, which misreads letters (c as o, for example); the words quoted are corrected and the rows say so. The 1941 Pacific Fleet annual report (the Base Force passage, printed p. 1270) has no text layer and was read by OCR; its digits and some lines are garbled and the rows say which.

The check shows that the words are on the page. It does not show that a figure sits against the right place or date; that filing is mine, and each row's note says what it rests on.

## Where the rows went

| File | Ids | What |
| --- | --- | --- |
| `place_facts.csv` | 241 to 295 | 55 rows: dated fuellings in port (`fuelling_event`), notes on supply, delay and practice (`fuelling_note`), tankage (`fuel_oil_storage_note`), pump and barge rates (`fuel_pump_rate_note`) |
| `sources.csv` | 1271 to 1275 | Salt Lake City deck logs; Aaron Ward war diary; the South Atlantic administrative history; the Pacific Fleet annual report of 1941; Wildenberg |
| `places.csv` | 1067 | `recife`, for the one comparison outside the theatre |
| `gaps.csv` | last 9 lines | What was looked for and not found |

The four new attributes are not in `attributes.csv`. The checker refuses an attribute for an entity type it does not know (`unknown entity type 'place'`; tried on 2026-11-15), as `step2-record.md` records for the earlier place attributes. A schema entry for places is the architect's to add.

A dated fuelling of a ship goes in `place_facts.csv` under the place where it happened, because the receiving ship is often not in `ships.csv` (Haruna, Shoho, Tsugaru and Isuzu are not) and `ship_facts.csv` takes only listed ships. A fuelling at sea, or in a place the page does not name, has no entity to hang on and has no row; it is in section 4 with its source and locator.

Existing rows used instead of new ones: Noumea's Ducos tank farm of 370,000 barrels with a 1943 date (`place_facts` 91), the tank farm cancelled in June 1942 (92), Noumea's one large-vessel dock (96).

## Conversions

Fuel is measured in barrels, gallons and tons in these sources, and the rates need one unit.

| Conversion | Value | Source |
| --- | --- | --- |
| 1 barrel | 42 US gallons | FTP 218, Table V, "Conversion factors", and Dyer: 12,000 barrels = 504,000 gallons |
| 1 ton of fuel oil | 277 gallons | FTP 218, Table V, same list (source `usn-fuel-consumption`, https://www.ibiblio.org/hyperwar/USN/ref/Fuel/Fuel-CA.html) |
| 1 barrel | 42 / 277 = 0.1516 ton; 6.595 barrels to the ton | arithmetic on the two lines above |

Assumptions, not stated by a source:

- **Which ton.** FTP 218 does not say long or metric on the page read. The earlier rows of `ship_class_facts.csv` read its tons as long tons (the Pensacola class: 586,132 gal = 2,116 tons at 277), and so does this note.
- **Japanese tons.** The combinedfleet pages print "t" and "tons" with no kind. They are used as printed. A long ton is 1.6 per cent more than a metric ton, which does not change a rate at the precision here.
- **Gallons of the Navy log are US gallons.** The logs do not say.
- Carter's own ratio, 219,830 tons "or approximately 1,300,000 barrels" for the petroleum products carried afloat in the first half of 1943 (BBBO-6), gives 5.9 barrels to the ton. It covers gasoline and diesel as well as black oil and is not used.

## 1. The rows

Every row is one source's statement; the full wording, the locator and the note are in `place_facts.csv`. `as_stated` holds the source's own words. Standing is `preferred` for a single medium or high row, `unconfirmed` where the date or the figure is doubtful (259), and `disputed` for the pair 287 and 288. Confidence is in the last column.

| Row | Place | Attribute | Date | Statement | Source | Conf. |
| --- | --- | --- | --- | --- | --- | --- |
| 241 | noumea | fuelling_event | 1942-05-19 | USS Salt Lake City took 241,469 gal of fuel oil from HMS Bishop Dale alongside in Great Road Harbor: moored to her at 1040, fuelling begun in the 0800-1200 watch, completed in the 1200-1600 watch, under way 1558 | slc-decklogs-1942 | medium |
| 242 | noumea | fuelling_event | 1942-09-04 | USS Salt Lake City took 10,162 bbl of fuel oil from SS Western Sun alongside at anchor (Berth 41, Dumbea Bay): tanker alongside and fuelling begun in the 0400-0800 watch, completed 1110 | slc-decklogs-1942 | medium |
| 243 | noumea | fuelling_event | 1942-09-18 | USS Salt Lake City took 368,928 gal of fuel oil from SS Comet alongside at anchor (Berth A-2, Great Roads): tanker alongside and fuelling begun in the 1600-1800 watch; the quantity is entered in that watch; no end time logged | slc-decklogs-1942 | medium |
| 244 | espiritu-santo | fuelling_event | 1942-09-16 | USS Salt Lake City delivered 923 bbl of fuel oil to USS Lansdowne at anchor in Segond Channel (Berth B-3): destroyer alongside in the 1600-1800 watch, pumping completed in the 1800-2000 watch | slc-decklogs-1942 | medium |
| 245 | espiritu-santo | fuelling_event | 1942-09-16 | USS Salt Lake City delivered 792 bbl of fuel oil to USS Lardner at anchor in Segond Channel: destroyer came alongside when Lansdowne cast off in the 1800-2000 watch, pumping completed in the 2000-2400 watch | slc-decklogs-1942 | medium |
| 246 | espiritu-santo | fuelling_event | 1942-10-05 | USS Salt Lake City took 2,699.5 bbl of fuel oil from tanker SS Gulfbird, port side to the tanker, between 0835 and noon, then anchored in Berth A-6 | slc-decklogs-1942 | medium |
| 247 | pago-pago | fuelling_event | 1942-10-24 | USS Salt Lake City received 5,658.8 bbl of fuel oil from the fuelling dock at Pago Pago, Tutuila, with USS Farenholt alongside: fuelling begun in the 1600-1800 watch, the quantity entered in the 2000-2400 watch | slc-decklogs-1942 | medium |
| 248 | pearl-harbor | fuelling_event | 1942-04-29 | USS Salt Lake City took 452,802 gal of fuel oil from the dock at Pearl Harbor: begun in the 1200-1600 watch, completed in the 1600-2000 watch | slc-decklogs-1942 | medium |
| 249 | espiritu-santo | fuelling_event | 1942-11-07 | USS Aaron Ward fuelled alongside the oiler Lackawanna at Button (Espiritu Santo): moored and commenced fuelling 0907, completed and under way 1135 | aaron-ward-diary-nov-1942 | high |
| 250 | espiritu-santo | fuelling_event | 1942-11-21 | USS Aaron Ward fuelled alongside the oiler Sabine in Segond Channel: entered the channel 0600, alongside 0635, completed 0845 | aaron-ward-diary-nov-1942 | high |
| 251 | espiritu-santo | fuelling_event | 1942-11-28 | USS Aaron Ward fuelled alongside the oiler Sabine: moored to her port side 1029, completed fuelling and cleared the tanker's side 1100 | aaron-ward-diary-nov-1942 | high |
| 252 | tongatabu | fuelling_event | 1942-07-24 | USS Enterprise fuelled in Tongatabu Harbor, about 12,000 bbl (504,000 gal) of fuel oil and 61,900 gal of aviation gasoline; no time or supplier stated | dyer-actc | high |
| 253 | espiritu-santo | fuelling_event | 1942-08-26 to 1942-08-28 | USS Sabine started back to Espiritu Santo on 26 August 1942, fuelled 11 ships there and sailed for Noumea on the 28th; no hours or quantities | carter-bbbo | medium |
| 254 | noumea | fuelling_event | 1942-08-30 to 1942-08-31 | USS Sabine filled at Noumea from the chartered tankers J. W. Van Dyke and Pacific Sun, 32,661 and 52,909 bbl on 30 and 31 August | carter-bbbo | medium |
| 255 | noumea | fuelling_note | 1942-08-01 | Ghormley to Nimitz, 1 August 1942: the tanker Bishopsdale at Noumea was empty and was being sent to Brisbane to refill; he asked for a loaded tanker of 50 to 100 thousand bbl for New Caledonia | carter-bbbo | high |
| 256 | noumea | fuelling_note | 1942-07 to 1942-08 | Plan of July 1942: 225,000 bbl of fuel oil to be brought to Noumea by chartered tankers, and the same amount about 2 August | carter-bbbo | high |
| 257 | tongatabu | fuelling_note | 1942-07 | Station oiler at Tongatabu: the Navy tanker Kanawha, capacity 75,000 bbl; the fleet oilers Cimarron and Platte to be there late in July to supply the amphibious force | carter-bbbo | high |
| 258 | noumea | fuel_oil_storage_note | 1942-11 | By late fall 1942 a fuel-oil depot on Ducos Peninsula, Noumea, held 370,000 bbl of black oil and 30,000 bbl of diesel, with a pier at which vessels could be unloaded and supplied | carter-bbbo | medium |
| 259 | tulagi | fuel_oil_storage_note | 1942-11 | Tulagi area, by late fall 1942 in Carter's account: ten 1,000-bbl tanks, 12,000 bbl of aviation gasoline, a 60,000-bbl diesel storage and a 280,000-bbl fuel-oil farm | carter-bbbo | low |
| 260 | espiritu-santo | fuelling_note | 1942-08 to 1943-10 | Fuelling of the fleet at Espiritu Santo before the fuel unit of November 1943 was done by station tankers and incoming oilers | carter-bbbo | medium |
| 261 | espiritu-santo | fuel_pump_rate_note | 1943-07 | Tank farms and fifty 10,000-bbl tanks at Espiritu connected by a pipeline system and pumps capable of handling 350 gal a minute | carter-bbbo | medium |
| 262 | espiritu-santo | fuel_oil_storage_note | 1943 | Aore Island: fifty 10,000-bbl bolted tanks, 25,000 ft of 12-inch spiral pipe and 50 pump units, built by the 57th Battalion | building-bases-v2 | medium |
| 263 | noumea | fuel_oil_storage_note | 1942-07 to 1942-08 | July and August 1942: the only project then scheduled for Noumea was a tank farm for 30,000 bbl of fuel oil and 20,000 bbl of diesel oil | building-bases-v2 | medium |
| 264 | shortland-islands | fuelling_event | 1942-05-06 | Shoho arrived at 0100 on 6 May 1942 and tried to fuel from an oil supply in a stone recess; fuelling began at 0630 and was discontinued after one hour; the Covering Force sailed at 0830 | bates-coral-sea-1947 | medium |
| 265 | shortland-islands | fuelling_event | 1942-05-05 to 1942-05-06 | The Covering Force's cruiser division (CruDiv 6) fuelled at Shortland from an oil supply there and completed fuelling at 0830 on 6 May 1942; no start hour | bates-coral-sea-1947 | medium |
| 266 | shortland-islands | fuelling_event | 1942-05-09 | First section of CruDiv 6 put into Shortland harbor at 0721 on 9 May 1942 and began fuelling from Iro and Mimuroyama Maru; at about 1200 it had completed fuelling and sailed | bates-coral-sea-1947 | medium |
| 267 | tulagi | fuelling_event | 1942-08-07 | On 7 August 1942 the transport President Jackson, landing troops at Tulagi, rigged ship for fuelling the destroyer-transports of Transport Division 12 at 0759 and fuelled McKean and Little between 1004 and 1239 | dyer-actc | high |
| 268 | tulagi | fuelling_note | 1942-08-07 | On 7 August 1942 the third attempt to fuel the destroyer-transport Colhoun alongside President Jackson succeeded at 1750: the first attempt at 1304 and the next were prevented by enemy plane radar contacts | dyer-actc | high |
| 269 | shortland-islands | fuelling_event | 1942-09-06 | Toa Maru, at the Shortland anchorage, refuelled the heavy cruisers Aoba, Kinugasa and Furutaka; no hours or quantities | combinedfleet | medium |
| 270 | shortland-islands | fuelling_event | 1942-12-03 | Toa Maru, at Shortland, refuelled the heavy cruisers Suzuya and Maya between 1000 and 1318 (3 h 18 min for both); no quantities | combinedfleet | medium |
| 271 | shortland-islands | fuelling_event | 1942-12-26 | Toa Maru, at Shortland, refuelled the destroyer Inazuma; no hours or quantity | combinedfleet | medium |
| 272 | shortland-islands | fuelling_event | 1942-05-05 | Iro refuelled CruDiv 6's Kinugasa, Aoba, Furutaka and Kako and the carrier Shoho at the Shortland anchorage | combinedfleet | medium |
| 273 | shortland-islands | fuelling_event | 1942-10-23 | Omurosan Maru at Shortland, 23 October 1942: 11 vessels refuelled, 24 t to 255 t each (the cruisers Kinugasa 153 t and Chokai 240 t, patrol boat PB-2 255 t), 1,120 t in all by the entry's figures | combinedfleet | medium |
| 274 | shortland-islands | fuelling_event | 1942-10-24 | Omurosan Maru at Shortland, 24 October 1942: 20 vessels refuelled, 14 to 116 tons each (light cruisers Yura 65 t and Tatsuta 116 t, destroyers 14 to 62 t) | combinedfleet | medium |
| 275 | shortland-islands | fuelling_event | 1942-10-26 | Omurosan Maru at Shortland, 26 October 1942: 19 vessels refuelled, the light cruiser Sendai 560 t, destroyers 59 to 364 t and smaller craft 7 to 60 t, 4,071 t in all by the entry's figures | combinedfleet | medium |
| 276 | shortland-islands | fuelling_event | 1942-10-27 | Omurosan Maru at Shortland, 27 October 1942: 4 vessels refuelled, the seaplane carrier Chitose 719 t, the destroyers Shigure 396 t and Ariake 160 t and the light cruiser Tatsuta 52 t, 1,327 t in all; she left Shortland later that day | combinedfleet | medium |
| 277 | truk | fuelling_event | 1942-09-25 | Kyokuto Maru at Truk: heavy cruiser Kumano 1,126 t between 0540 and 1818; heavy cruiser Suzuya 1,360 t between 1115 and 1630 | combinedfleet | medium |
| 278 | truk | fuelling_event | 1942-09-07 | Kokuyo Maru began refuelling the heavy cruiser Suzuya at Truk at 0830; no end or quantity | combinedfleet | medium |
| 279 | truk | fuelling_event | 1942-11-01 | Kokuyo Maru at Truk: fuel to the heavy cruiser Suzuya between 0938 and 1215 and to the heavy cruiser Kumano from 1240 to 1402; no quantities | combinedfleet | medium |
| 280 | truk | fuelling_event | 1942-12-08 | Kyokuto Maru at Truk refuelled the light cruisers Oi (1,070 t) and Kitakami (962 t); no hours | combinedfleet | medium |
| 281 | truk | fuelling_event | 1942-11-20 | Omurosan Maru at Truk refuelled the heavy cruiser Takao (1,105 t) and the battleships Kongo (3,750 t) and Haruna (1,565 t); no hours | combinedfleet | medium |
| 282 | rabaul | fuelling_event | 1942-05-11 | Iro at Rabaul: the minelayer Tsugaru lay alongside to starboard from 1040 to 1344 and was supplied with 401 t of heavy oil | combinedfleet | medium |
| 283 | rabaul | fuelling_event | 1942-03-15 | Iro refuelled the light cruiser Yubari at Rabaul from 0530 until 0910 and left Rabaul later that day; no quantity | combinedfleet | medium |
| 284 | rabaul | fuelling_event | 1942-12-05 | Tsurumi at the Rabaul supply anchorage transferred 570 t of bunker fuel to the heavy cruiser Suzuya on her port side between 1027 and 1310 | combinedfleet | medium |
| 285 | rabaul | fuelling_event | 1942-12-03 | Tsurumi at the Rabaul supply anchorage supplied the light cruisers Kitakami and Oi, alongside her together, with 450 t each from 1300; no end | combinedfleet | medium |
| 286 | rabaul | fuelling_event | 1942-11-23 | Tsurumi at the Rabaul supply anchorage on 23 November 1942: destroyer Suzukaze and PB-37 from about 0445 to 0840, 12 vessels of the 10th Squadron from 1100, a destroyer at 1500 | combinedfleet | medium |
| 287 | rabaul | fuelling_event | 1942-11-27 | Tsurumi at the Rabaul supply anchorage took 3,100 t of heavy oil for drums from the auxiliary oiler Kokuyo Maru between 0930 and 1030 (this page) | combinedfleet | medium |
| 288 | rabaul | fuelling_event | 1942-11-27 | Kokuyo Maru at the supply anchorage: Tsurumi came alongside 0930, lay alongside 1030, and at 1530 Kokuyo Maru transferred 3,100 t of heavy oil for drums to her | combinedfleet | medium |
| 289 | recife | fuel_pump_rate_note | 1942-06 to 1942-09 | The Caloric Oil Company's pumps at Recife, summer 1942: estimated capacity 150 tons an hour | usn-admin-hist-south-atlantic | medium |
| 290 | recife | fuel_pump_rate_note | 1942-06 to 1942-09 | The same pumps: 125 tons an hour was considered good in actual practice | usn-admin-hist-south-atlantic | medium |
| 291 | recife | fuelling_note | 1942-06 to 1942-09 | By the summer of 1942 destroyers were often brought into Recife for a three-hour quick turnaround: fuelled, supplied and ready for sea again within three hours | usn-admin-hist-south-atlantic | medium |
| 292 | pearl-harbor | fuel_pump_rate_note | 1940-07 to 1941-06 | Fuel-oil barges at Pearl Harbor, year to 30 June 1941: each barge 3,500 bbl capacity; 28,000 bbl of fuel oil could be delivered in an eight-hour day by the barges kept working (four, with one in upkeep) | cincpac-annual-report-1941 | medium |
| 293 | pearl-harbor | fuel_pump_rate_note | 1940-07 to 1941-06 | Pearl Harbor, year to 30 June 1941: the five fuel-oil barges met the Fleet's needs but often worked 12 to 16 hours a day; a rapid fuelling of the Fleet would need many cruisers and all destroyers to fuel from Navy Yard piers | cincpac-annual-report-1941 | medium |
| 294 | pearl-harbor | fuel_pump_rate_note | 1940-07 to 1941-06 | Pearl Harbor, year to 30 June 1941: when practicable the barges fuelled from oilers at anchor, because oilers could fill the barges in a shorter time than the shore activity | cincpac-annual-report-1941 | medium |
| 295 | truk | fuelling_event | 1942-02-26 | Hoyo Maru arrived at Truk and refuelled the aircraft carrier Shoho with 1,500 t of heavy oil; no hours | combinedfleet | medium |

## 2. Fuellings in port with a quantity, a time, or both

Tons are the row's quantity at 277 gallons to the ton and 42 to the barrel (section on conversions). "By watch" means the log gives the four-hour watch in which a thing happened and no hour, so the duration is bounded and not known. A rate is the quantity divided by the interval; a range comes from the two ends of an interval the log leaves open.

| Rows | Place and date | From, to | Quantity | Interval | Tons an hour |
| --- | --- | --- | --- | --- | --- |
| 242 | Noumea (Dumbea Bay, at anchor), 4 Sep | SS Western Sun, Salt Lake City | 10,162 bbl = 1,541 t | start in the 0400-0800 watch, end 1110: 3 h 10 min to 7 h 10 min | 215 to 487 |
| 241 | Noumea (Great Road), 19 May | HMS Bishop Dale, Salt Lake City | 241,469 gal = 872 t | alongside 1040, under way 1558: at most 5 h 18 min | at least 164 |
| 243 | Noumea (Great Roads, at anchor), 18 Sep | SS Comet, Salt Lake City | 368,928 gal = 1,332 t | none logged | none |
| 246 | Espiritu Santo (Segond Channel), 5 Oct | SS Gulfbird, Salt Lake City | 2,699.5 bbl = 409 t | under way 0835, anchored again before 1200: under 3 h 25 min | at least 120 |
| 244 | Espiritu Santo, 16 Sep | Salt Lake City, Lansdowne | 923 bbl = 140 t | by watch: under 4 h | no useful bound |
| 245 | Espiritu Santo, 16 Sep | Salt Lake City, Lardner | 792 bbl = 120 t | by watch: under 6 h | no useful bound |
| 254 | Noumea, 30 and 31 Aug | J. W. Van Dyke and Pacific Sun, Sabine | 32,661 and 52,909 bbl = 4,952 and 8,022 t | each within one day | at least 334 (for the 8,022 t) |
| 247 | Pago Pago (fuelling dock), 24 Oct | dock, Salt Lake City | 5,658.8 bbl = 858 t | by watch: 2 h to 8 h | 107 to 429 |
| 248 | Pearl Harbor (dock), 29 Apr | dock, Salt Lake City | 452,802 gal = 1,635 t | by watch: at most 8 h | at least 204 |
| 249 | Espiritu Santo, 7 Nov | Lackawanna, Aaron Ward | none | 0907 to 1135: 2 h 28 min | none (not over 203 if she took no more than the 500 t the page gives as her capacity) |
| 250 | Espiritu Santo (Segond Channel), 21 Nov | Sabine, Aaron Ward | none | 0635 to 0845: 2 h 10 min | none (not over 231 on the same condition) |
| 251 | Espiritu Santo, 28 Nov | Sabine, Aaron Ward | none | 1029 to 1100: 31 min | none |
| 267 | Tulagi (transport area), 7 Aug | President Jackson, the APDs McKean and Little | none | 1004 to 1239: 2 h 35 min for the two | none |
| 282 | Rabaul, 11 May | Iro, minelayer Tsugaru | 401 t | 1040 to 1344: 3 h 4 min | 131 |
| 277 | Truk, 25 Sep | Kyokuto Maru, heavy cruiser Kumano | 1,126 t | 0540 to 1818: 12 h 38 min | 89 |
| 277 | Truk, 25 Sep | Kyokuto Maru, heavy cruiser Suzuya | 1,360 t | 1115 to 1630: 5 h 15 min | 259 |
| 284 | Rabaul, 5 Dec | Tsurumi, Suzuya | 570 t | 1027 to 1310: 2 h 43 min | 210 |
| 287, 288 | Rabaul, 27 Nov | Kokuyo Maru, Tsurumi (oiler to oiler, for drums) | 3,100 t | the two pages disagree: 0930 to 1030 (1 h), 1030 to 1530 (5 h) or 0930 to 1530 (6 h) | 3,100, 620 or 517 |
| 270 | Shortland, 3 Dec | Toa Maru, Suzuya and Maya | none | 1000 to 1318: 3 h 18 min for the two | none |
| 266 | Shortland, 9 May | Iro and Mimuroyama Maru, first section of CruDiv 6 | none | 0721 to about 1200: 4 h 39 min | none |
| 264 | Shortland, 6 May | an oil supply in a stone recess, Shoho | none | 0630 for one hour, then discontinued | none |
| 279 | Truk, 1 Nov | Kokuyo Maru, Suzuya then Kumano | none | 0938 to 1215 (2 h 37 min); 1240 to 1402 (1 h 22 min) | none |
| 283 | Rabaul, 15 Mar | Iro, light cruiser Yubari | none | 0530 to 0910: 3 h 40 min | none |
| 286 | Rabaul, 23 Nov | Tsurumi, destroyer Suzukaze and PB-37; then 12 vessels of the 10th Squadron; then a destroyer | none | 0445 to 0840 (3 h 55 min); from 1100; 1500 | none |
| 285 | Rabaul, 3 Dec | Tsurumi, Kitakami and Oi together | 450 t each | from 1300, no end | none |

The Aaron Ward ceilings assume the page's "Fuel (oil) tons 450 - 500", which sits in the ship's specification table, is her capacity, and that she took no more than a full tank; the diary gives no quantity, and she may have taken much less. The 31 minutes of 28 November, a week after the 2 h 10 min of 21 November, was probably a top-off; the diary does not say.

## 3. What one ship took

Quantities in tons, from the rows or from section 4. The kind of ship is the source's.

| Kind | Quantity taken in one fuelling | Rows |
| --- | --- | --- |
| Destroyer | 120 and 140 t (Lardner, Lansdowne, from a cruiser); Japanese destroyers 59 to 396 t on 26 and 27 Oct at Shortland, 14 to 62 t on 24 Oct; 139 t (Farenholt, 38,585 gal, from Salt Lake City at sea, 28 Oct) | 244, 245, 275, 276, 274 |
| Light cruiser | Sendai 560 t; Oi 1,070 t; Kitakami 962 t; Tatsuta 52 to 116 t | 275, 276, 274, 280 |
| Heavy cruiser | Salt Lake City 872 t, 883 t (at sea), 1,332 t, 1,541 t, 1,635 t; Kumano 1,126 t (Truk), 1,380 t (at sea); Suzuya 1,360 t; Atago 900 t; Takao 1,105 t | 241 to 243, 248, 277, 281 |
| Battleship | Kongo 3,750 t; Haruna 1,565 t, and 1,645 t on 18 to 19 Oct | 281, section 4 |
| Carrier | Enterprise 12,000 bbl = 1,819 t at Tongatabu on 24 Jul; 20,000 bbl = 3,032 t from Kaskaskia at sea on 10 Aug; Shoho 1,500 t at Truk on 26 Feb | 252, section 4, 295 |
| Fleet oiler loading | Sabine 4,952 t and 8,022 t from chartered tankers on 30 and 31 Aug; Tsurumi 3,100 t from Kokuyo Maru on 27 Nov | 254, 287, 288 |

## 4. Statements with no row

These are verified statements for which no place or ship entity exists: at sea, a place the page does not name, or a force. Each has its source and locator here, and no row.

### 4a. Japanese oilers, from the combinedfleet records (tons as printed; "place not stated" means the entry names none)

The rate is the quantity over the interval of the entry. Every interval comes from the quoted sentence.

| Date | From, to | Quantity | Interval | Hours | Tons an hour | Page, entry |
| --- | --- | --- | --- | --- | --- | --- |
| 1942-01-20 | Iro to 4 heavy cruisers (CruDiv 6), at sea 01-00N 147-45E | 1,423 t | 0700-1500 | 8.00 h | 178 | https://www.combinedfleet.com/Iro_t.htm, 20 January 1942 |
| 1942-01-24 | Iro to the 4 heavy cruisers, place not stated | 1,746 t | 0600-1300 | 7.00 h | 249 | https://www.combinedfleet.com/Iro_t.htm, 24 January 1942 |
| 1942-01-30 | Iro to the 4 heavy cruisers, place not stated | 2,000 t | 0830-1430 | 6.00 h | 333 | https://www.combinedfleet.com/Iro_t.htm, 30 January 1942 |
| 1942-03-05 | Kamoi to minelayer Tsugaru, place not stated | 250 t | 0916-1159 | 2.72 h | 92 | https://www.combinedfleet.com/Kamoi_t.htm, 5 March 1942 |
| 1942-07-17 | Iro to minelayer Tsugaru, place not stated | 284 t | 1552-1807 | 2.25 h | 126 | https://www.combinedfleet.com/Iro_t.htm, 17 July 1942 |
| 1942-08-01 | Nippon Maru to heavy cruiser Kumano, place not stated | 402 t | 0855-1220 | 3.42 h | 118 | https://www.combinedfleet.com/Nippon_t.htm, 1 August 1942 |
| 1942-08-01 | Nippon Maru to heavy cruiser Suzuya | 410 t | 1220-1500 | 2.67 h | 154 | https://www.combinedfleet.com/Nippon_t.htm, 1 August 1942 |
| 1942-08-01 | Nippon Maru to destroyers Oyashio (153 t) and Hayashio (140 t) | 293 t | 1540-1650 | 1.17 h | 251 | https://www.combinedfleet.com/Nippon_t.htm, 1 August 1942 |
| 1942-08-02 | Nippon Maru to destroyer Harusame | 104 t | 1400-1535 | 1.58 h | 66 | https://www.combinedfleet.com/Nippon_t.htm, 2 August 1942 |
| 1942-08-03 | Nippon Maru to destroyer Samidare | 176 t | 1100-1220 | 1.33 h | 132 | https://www.combinedfleet.com/Nippon_t.htm, 3 August 1942 |
| 1942-08-04 | Kyokuto Maru to destroyer Yudachi (and bunkering Murasame) | 104 t | 1045-1300 | 2.25 h | 46 | https://www.combinedfleet.com/Kyokuto_t.htm, 4 August 1942 |
| 1942-08-30 | Kyokuto Maru to heavy cruiser Kumano, at sea | 480 t | 0640-0830 | 1.83 h | 262 | https://www.combinedfleet.com/Kyokuto_t.htm, 30 August 1942 |
| 1942-08-30 | Kyokuto Maru to heavy cruiser Suzuya, at sea | 700 t | 1237-1514 | 2.62 h | 268 | https://www.combinedfleet.com/Kyokuto_t.htm, 30 August 1942 |
| 1942-09-16 | Kokuyo Maru to heavy cruiser Kumano, at sea 02-40N 163-48E | 1,380 t | 1100-1530 | 4.50 h | 307 | https://www.combinedfleet.com/Kokuyo_t.htm, 16 September 1942 |
| 1942-09-16 | Kokuyo Maru to heavy cruiser Suzuya, at sea | 230 t | 1745-1851 | 1.10 h | 209 | https://www.combinedfleet.com/Kokuyo_t.htm, 16 September 1942 |
| 1942-10-18 | Kyokuto Maru to heavy cruiser Atago, place not stated | 900 t | 1625-2010 | 3.75 h | 240 | https://www.combinedfleet.com/Kyokuto_t.htm, 18 October 1942 |
| 1942-10-18 | Kyokuto Maru to heavy cruisers Atago and Takao and destroyers Takanami and Makinami (2,153 t in all) | 2,153 t | 1530-2340 | 8.17 h | 264 | https://www.combinedfleet.com/Kyokuto_t.htm, 18 October 1942 |
| 1942-10-18 | Toei Maru to battleship Haruna (start read as the 2000 of the entry) | 1,645 t | 2000-0020 | 4.33 h | 380 | https://www.combinedfleet.com/Toei_t.htm, 18 October 1942 (completed 0020 on 19 October, same page) |
| 1942-10-25 | Genyo Maru to heavy cruiser Atago, at sea | 75 t | 2318-2331 | 0.22 h | 346 | https://www.combinedfleet.com/Genyo_t.htm, 25 October 1942 |
| 1942-10-28 | Genyo Maru to heavy cruiser Atago on her starboard side, at sea 01-19S 161-50E | 68 t | 0406-0700 | 2.90 h | 23 | https://www.combinedfleet.com/Genyo_t.htm, 28 October 1942 |

Notes on the table: the 18 October entry of Kyokuto Maru gives 2,153 t to the heavy cruisers Atago and Takao over 1530 to 2340 and then names the destroyers Takanami and Makinami with no quantity, so the 264 an hour is for the two cruisers, taken together, and the destroyers' time is inside the interval. Toei Maru's entry gives "At 2000 transfers 1645 tons ... to battleship HARUNA" and the same page says on 19 October "At 0020 completes refuelling HARUNA"; 2000 is read as the start, which the sentence does not quite say. Genyo Maru's two entries are small quantities, 75 and 68 t, and say little about a rate. The January entries (Iro to the four cruisers of CruDiv 6) are not in the period of the campaign but are the only records of four cruisers fuelled in one operation with times.

### 4b. Salt Lake City, at sea (deck log, source `slc-decklogs-1942`, by watch)

| Date | From | Quantity | Interval by watch | Tons an hour |
| --- | --- | --- | --- | --- |
| 17 Apr | Cimarron (log `deckl10a.htm`: "Completed fueling, receiving 5,585.5 barrels fuel oil.") | 847 t | alongside begun after the 0800-1200 watch's maneuvering, done in the 1200-1600 watch: at most 8 h | at least 106 |
| 29 Jul | Lang fuelled by Salt Lake City, log `deckl13a.htm` ("having delivered 1,359 barrels fuel oil") | 206 t | within the 0800-1200 watch | at least 52 |
| 29 Jul | Platte ("received 220,240.4 gals. of fuel oil"; "approximately filled to 99% capacity") | 795 t | begun in the 0800-1200 watch, done in the 1200-1600 watch: at most 8 h | at least 99 |
| 11 Aug | Kaskaskia, log `decklg14.htm` ("Cast off from tanker having received 10,250 barrels fuel oil.") | 1,554 t | maneuvering to go alongside in the 1200-1600 watch, cast off in the 1800-2000 watch: 2 h to 8 h | 194 to 777 |
| 19 Aug | Kaskaskia ("Completed fueling ship having received 244,608 gallons.") | 883 t | maneuvering to go alongside and completion both in the 0800-1200 watch | at least 221 |
| 11 Sep | Guadalupe, log `decklg15.htm` ("Received 87,738 gals fuel, 1,942 gals. diesel fuel.") | 317 t | alongside and cast off in the 0800-1200 watch | at least 79 |
| 28 Oct | Salt Lake City to Farenholt, log `decklg16.htm` ("USS FARENHOLT came alongside and received 38,585 gallons of fuel.") | 139 t | no hours | none |

The "at least" figures use the longest interval the watches allow. They are lower bounds and no more.

### 4c. Dyer, Carter and Bates

| Statement | Source and place |
| --- | --- |
| Enterprise took 4,000 bbl (168,000 gal) at sea on 29 July, and on 10 August 20,000 bbl (840,000 gal) of fuel oil and 120,000 gal of aviation gasoline from Kaskaskia. | Dyer ch. 10, text after the --387-- marker |
| Gwin was fuelled by North Carolina on 7 August: "The fueling was conducted during darkness". | Dyer ch. 10, same |
| TF 16 "did not actually fuel until 1700 on 10 August". | Dyer ch. 10, after the --392-- marker |
| Ghormley, 14 August: Fletcher's carrier groups after one week of normal cruising "had completely emptied the oilers Platte and Kaskaskia". 18 August: from 23 days' issues, combatant vessels used 25,000 bbl a day and auxiliaries 3,000, 28,000 bbl a day in all (4,245 t a day). | Carter ch. IV, text after the --31-- marker |
| 21 July: Platte pumped her remaining oil into Cimarron, went to Noumea and refilled; she took aboard 93,000 bbl there. | Carter ch. IV, text after the --24-- marker |
| CTF 17 (Coral Sea, May 1942) fuelled destroyers "from whatever supply was available whenever they could receive as much as 500 barrels of fuel". | Bates p. 28, scan text, letters corrected |
| Bates: fuelling destroyers from larger ships was "much quicker" than from "the limited number of oilers"; at sea in a forward area was "normally, not quite so fast as fueling from an oiler at anchor or from a dock". | Bates pp. 29 and 50, scan text, letters corrected |
| TF 11 began fuelling from Tippecanoe about 0800 on 1 May (one passage) or early on 2 May (another); fuelling of TF 11 was completed at 1310 on 3 May, a day before the 4 May that CTF 11 had estimated. | Bates p. 28 to 29, scan text |
| Enterprise took on oil from Platte through the night "for five and a half hours", side by side at 8 to 12 knots, on the approach to the Tokyo raid, April 1942; no quantity. | Wildenberg ch. 17 (GSBO-17), p. 173 to 174 |

## 5. Shore systems, barges and pumps

Four figures for 1941 to 1943 are printed as rates or as a day's delivery; the older ones, at sea, follow the table.

| Row | Place | Statement | In tons an hour |
| --- | --- | --- | --- |
| 289 | Recife, summer 1942 | Caloric Oil Company's pumps, "estimated capacity was 150 tons per hour" | 150 (989 bbl an hour) |
| 290 | Recife, summer 1942 | same pumps, "125 was considered good in actual practice" | 125 (824 bbl an hour) |
| 291 | Recife, summer 1942 | destroyers brought in for a three-hour "quick turnaround": fuelled, supplied and ready for sea within three hours | none |
| 292 | Pearl Harbor, to 30 Jun 1941 | barges of 3,500 bbl each; 28,000 bbl could be delivered in an eight-hour day, four barges working and one in upkeep | 531 for the barges taken together (3,500 bbl an hour) |
| 293 | Pearl Harbor, to 30 Jun 1941 | five barges often worked 12 to 16 hours a day; a rapid fuelling of the Fleet would need many cruisers and all destroyers to fuel from Navy Yard piers | none |
| 294 | Pearl Harbor, to 30 Jun 1941 | barges fill from oilers at anchor in a shorter time than from the shore | none |
| 261 | Espiritu Santo, from July 1943 | a pipeline system and pumps "capable of handling 350 gallons a minute" | 76 (21,000 gal an hour) |

The Pearl Harbor figure of 28,000 barrels is for the barges that served the Fleet; it is a day's deliveries across four barges, not a pump rating. If the four worked side by side the figure per barge is 133 t an hour; the report does not say they did, and its own next sentence says the small barges spend a large part of their time going and coming from the fuel docks (the digits of that share are garbled in the scan).

**At sea and before the war** (Wildenberg ch. 3 and 17, no row; context only, not 1942 and not in port):

| Date | Statement | In tons an hour |
| --- | --- | --- |
| 11 Jan 1925 | Cuyama to the cruiser Omaha alongside, riding abeam: 221,878 gal from 1150 to 1615 with 4-inch hoses; "the transfer rate was more than 55,000 gallons per hour" | about 199 |
| Dec 1924 onward | Kanawha's over-the-stern gear: "at best only about 16,000 gallons per hour" | 58 |
| Feb 1931 | Salinas to three destroyers: 106,000 gal in "just over five hours" | about 77 for the three together |
| 1932 and 1933 | Brazos "as many as seven 4-inch fuel oil hoses" to fuel two destroyers at once | none |
| 22 Dec 1941 | Neches, destroyers of the Wake relief: "only four destroyers were filled during a ten-hour fueling period", in a cross-swell with seven hoses ruptured | none |

## 6. Rates in tons an hour that the rows support

Nothing below is a recommendation. Each figure is the arithmetic of section 2 or 4, and each rests on one event unless it says otherwise.

**One ship alongside a Japanese oiler, in harbour or at a place the entry does not name, quantity and hours both printed (tons as printed).** 13 fuellings, one ship each except the two destroyers fuelled together on 1 August: 46, 66, 89, 92, 118, 126, 131, 132, 154, 210, 240, 251 and 259 tons an hour; the median is 131. Those at a named place are 131 (Rabaul, 401 t to a minelayer), 89 and 259 (Truk, two cruisers served at once by one oiler), and 210 (Rabaul, a heavy cruiser). The other nine are from entries that name no place (rows of section 4a). The two largest single deliveries to a cruiser in port, 1,126 t and 1,360 t (277), took 89 and 259 tons an hour. The 89 is over 12 h 38 min with the oiler serving two ships; the entry does not say pumping was continuous.

**A heavy cruiser alongside a merchant tanker at anchorage, Noumea.** 215 to 487 (242), from an interval left open at one end. The same log gives lower bounds of 164 (241, a British tanker, May) and, at Espiritu Santo, 120 (246, a merchant tanker, October).

**A Navy oiler or shore dock to a cruiser.** Lower bounds only: at least 204 at the Pearl Harbor dock (248), 107 to 429 at the Pago Pago dock (247), and at sea at least 221 (Kaskaskia, 19 August).

**Oiler loading from a chartered tanker.** At least 334 (254), from a calendar day.

**Disputed.** 3,100 t between oilers at Rabaul: 3,100, 620 or 517 an hour by which interval is right (287, 288). The first is far above every other figure.

**At sea, Japanese oilers (section 4a).** 209 and 307 (Kokuyo Maru, 16 September), 262 and 268 (Kyokuto Maru, 30 August), 240 (Atago, 18 October, place not stated), and about 380 for a battleship (Haruna, if 2000 is the start). The January fuellings of four cruisers: 178, 249 and 333 over 6 to 8 hours.

**Shore lines and barges.** 125 (practice) and 150 (estimate) at Recife in 1942 (289, 290); 531 for the barges at Pearl Harbor taken together in 1941 (292); 76 for the Espiritu Santo pumps from 1943 (261).

**Destroyer times with no quantity.** 31 minutes, 2 h 10 min and 2 h 28 min for Aaron Ward (249 to 251), one ship from one diary. Two destroyer-transports in 2 h 35 min under air threat (267).

## 7. Hours by kind of ship

| Kind | Hours stated, with the row | Note |
| --- | --- | --- |
| Destroyer | 0 h 31 min (251), 2 h 10 min (250), 2 h 28 min (249); 1 h 20 min for 176 t and 1 h 35 min for 104 t (section 4a, Nippon Maru); 2 h 15 min for 104 t (Kyokuto Maru); destroyer and a patrol boat 3 h 55 min (286) | the last from Tsurumi's page; times are alongside intervals |
| Destroyer-transport | 2 h 35 min for two (267) | 7 August 1942 |
| Light cruiser | 3 h 40 min (283, Yubari) | 15 March 1942 |
| Heavy cruiser | 3 h 18 min for two together (270); 2 h 37 min and 1 h 22 min (279); 2 h 43 min for 570 t (284); 3 h 25 min for 402 t and 2 h 40 min for 410 t (Nippon Maru); 5 h 15 min for 1,360 t and 12 h 38 min for 1,126 t (277) | the range follows the quantity and the number of ships served |
| Four heavy cruisers | 6 h, 7 h and 8 h (January); 4 h 39 min for the first section on 9 May (266) | |
| Battleship | about 4 h 20 min for 1,645 t (Haruna, 18 to 19 October, see the notes to 4a) | the only battleship interval |
| Carrier | none in port | Enterprise's 12,000 bbl at Tongatabu (252) has no hours; Shoho 1 h at Shortland before the fuelling was stopped (264) |

## 8. How many at once, and tankage

**Berths and oilers alongside.** No source read gives a count for Noumea, Espiritu Santo, Tulagi, Truk, Rabaul or Shortland. What the logs show:

- At Truk, Kyokuto Maru served two heavy cruisers between 1115 and 1630 on 25 September (277).
- At Rabaul, Tsurumi had two light cruisers alongside her on 3 December, one on each side (285), and on 23 November served 14 vessels in a day (286).
- At Shortland, Omurosan Maru served 11 vessels on 23 October, 20 on 24 October, 19 on 26 October (4,071 t by the entry's figures) and 4 on 27 October (273 to 276). The page gives no hours.
- At Espiritu Santo, Salt Lake City served two destroyers one after the other on 16 September (244, 245), and Sabine fuelled 11 ships there between her return (she started back on 26 August) and her sailing on the 28th (253).
- At Noumea, one large-vessel dock existed (place_facts 96); the tankers named in rows 241, 242 and 243 came alongside the cruiser at anchor.

**Tankage.**

| Place | Statement | Row |
| --- | --- | --- |
| Noumea | July and August 1942: the only project then scheduled was a tank farm for 30,000 bbl of fuel oil and 20,000 bbl of diesel | 263 |
| Noumea | by late fall 1942: 370,000 bbl of black oil and 30,000 bbl of diesel at Ducos, with a pier at which vessels could be unloaded and supplied (a 1943 date is on 91) | 258 |
| Tulagi area | ten 1,000-bbl tanks, 12,000 bbl of aviation gasoline, a 60,000-bbl diesel storage and a 280,000-bbl fuel-oil farm, in Carter's account of late 1942 (unconfirmed) | 259 |
| Espiritu Santo | fifty 10,000-bbl tanks and 50 pump units on Aore; fleet fuelling "by means of station tankers and incoming oilers" before November 1943 | 262, 260 |
| Tongatabu | the Navy tanker Kanawha, 75,000 bbl, as station oiler; Cimarron and Platte to be there late in July | 257 |
| Noumea | 225,000 bbl planned by chartered tankers for July and again about 2 August; on 1 August the Bishopsdale was empty | 256, 255 |

Carter says of the oiler Platte, sent to Noumea on 21 July to refill from the chartered tankers there, "She cleaned out the tankers there"; on 1 August the Bishopsdale was empty, and the 225,000 barrels due in the chartered tankers E. J. Henry and Esso Little Rock had been diverted, one to Efate and one to Suva (rows 255, 256).

## 9. Delay and cut short

- **Shoho, 6 May 1942, Shortland (264).** She tried to fuel from an oil supply in a stone recess, her crew being green, began at 0630 and stopped after one hour; the Covering Force sailed at 0830; she was told to complete from Hoyo Maru (Bates, p. 50: "directed to completed fueling from the HOYO-MARU"). Bates cites her war diary.
- **Colhoun, Tulagi, 7 August 1942 (268).** Fuelling alongside President Jackson was prevented twice by enemy plane contacts and completed at 1750 on the third attempt, 4 h 46 min after the first preparation at 1304.
- **Iro, 8 May 1942.** "At 1100 begins refueling cruisers AOBA and KAKO with heavy oil but soon after it stops." No reason is given.
- **Recife (291).** A three-hour turnaround was required; "The Caloric pumps did not meet the situation."
- **Bates.** At sea in a forward area the fuel came slower than from an oiler at anchor or a dock, and fuelling destroyers from big ships was quicker than waiting for oilers (section 4c).
- **Carter.** Ghormley's cables of 14 and 18 August (section 4c) are about stock, not hours.

No statement was found that a port fuelling held a berth or kept a force in port beyond a sailing time.

## 10. Where the sources disagree

- **27 November 1942, Kokuyo Maru to Tsurumi, 3,100 t (rows 287, 288).** Tsurumi's page: alongside 0930, "From 0930-1030, KOKUYO MARU transfers 3100t heavy oil for drums". Kokuyo Maru's page: Tsurumi starts alongside 0930, lies alongside 1030, "At 1530, KOKUYO MARU transfers 3100t". The quantity and the 0930 agree. The pages are one site's, so the disagreement is inside one compilation, not between two.
- **8 and 9 May 1942.** Iro's page has the refuelling of Aoba and Kako at 1100 on 8 May, "soon after it stops", and a general entry for 9 May. Bates has the first section of CruDiv 6 start at 0721 on 9 May (266). Two readings of one episode are possible; the pages do not settle it.
- **3 December 1942, light cruiser Oi.** Kokuyo Maru's page gives her 400 t; Tsurumi's page gives her 450 t from Tsurumi at 1300. They may be two fuellings. Kyokuto Maru's page gives her 1,070 t on 8 December (280).
- **Where Carter and Building the Navy's Bases differ.** Carter's date for the Noumea and Tulagi tankage is "late fall of 1942" in a paragraph that goes on to July 1943; the Bureau of Yards and Docks gives no month for the Ducos farm. Row 258 and row 259 carry the doubt.

## 11. What is still missing

- A pumping rate, a hose count or a berth count for any United States fleet oiler or any South Pacific port in 1942. The only rates found are Recife's, Pearl Harbor's barges and the Espiritu Santo pumps of 1943.
- A fuelling of a United States carrier, battleship or cruiser force in a South Pacific port with both a quantity and the hours. Enterprise's 12,000 bbl has no hours; Salt Lake City's cruiser quantities have hours only by watch, except one finish.
- Any Japanese fuelling from a shore depot. Every Japanese fuelling in the record is from an oiler.
- A carrier fuelled at Truk or Rabaul in the autumn with a quantity.
- The 1938 Pearl Harbor delivery rate. The PDF named for it opened and is a scan of 71 pages of a Fleet annual report; OCR found nothing on fuel delivery, and the figure is not verified.
- The 1942 Pacific Fleet instructions for fuelling at sea, and Cimarron-class pump rates. See section 12.

The gaps are entered in `gaps.csv` (the last nine lines).

## 12. Pages that would not open

| Page | What happened |
| --- | --- |
| researcheratlarge.com, "1942RefuelingAtSea" (Pacific Fleet instructions for fuelling at sea) | certificate expired |
| history.navy.mil, "Chester Nimitz and the Development of Fueling at Sea" | certificate could not be verified |
| pwencycl.kgbudge.com, Cimarron class | certificate did not match the host |
| ussslcca25.com `deckl14a.htm`, `deckl16a.htm` to `deckl21a.htm`, `deckl11a.htm`, `deckl8a.htm` | no such page (HTTP 404); the August, October and November logs are on `decklg14.htm`, `decklg16.htm` and `decklg17.htm` |
| archives.gov, 1938 annual report scan | opened; a 71-page scan, OCR found nothing on fuel delivery |

No figure was taken from a search result.
