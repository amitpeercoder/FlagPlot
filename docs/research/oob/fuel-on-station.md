# Fuel at sea and on station, August to November 1942

Status: a research report for the architect, read on 2026-11-15. Nothing here is settled and nothing here proposes a game figure. Rows: `ship_class_facts.csv` 1844 to 1872, `ship_facts.csv` 1824 to 1858, `place_facts.csv` 696 to 706, nine lines at the end of `gaps.csv`, and two new attributes in `attributes.csv`. `fuelling-in-port.md` (place_facts 241 to 295) is the earlier pass on the same subject; its rows are not repeated and are cited by id where a statement is already held.

## What was asked

1. What a force burned in a day in the South Pacific, at cruising and at high speed.
2. The oilers serving the South Pacific Force and the Japanese oilers at Truk, Rabaul and the Shortlands: names, dates, capacity, the round trip to reload.
3. Fuel on hand at Noumea, Espiritu Santo, Efate, Tongatabu, Tulagi, Truk and Rabaul, by date.
4. How many ships one oiler served at once and in a day.
5. Forces held, withdrawn, slowed or not sent because of fuel.

A list of 26 statements with addresses came with the task. Each was opened, and the answer for each is in section 1.

## How it was read

Every page was read through the web fetch tool. That tool returns the page text as an answer to a question, in excerpts of at most about 125 characters, and keeps no copy I could check by program. The quotations in the rows are the tool's quotations. I did not run the check by program that the earlier passes used (`fuelling-in-port.md`, "How it was read"). Where a row's figures can be tested another way, the row says so and the test is in section 3 (the two columns of each FTP 218 line agree by arithmetic; Zimmerman's days of fuel agree with the Dyer table). Confidence is `high` only where such a test passed, and `medium` elsewhere.

- **Zimmerman's *The Guadalcanal Campaign*:** the PDF at usmcu.edu returned HTTP 403 and was not opened. The HyperWar transcription (`USMC-M-Guadalcanal-3.html`, footnote 35, printed p. 50) was opened and is the page cited.
- **Bates, *The Battle of Savo Island*:** the PDF on wikimedia was over the tool's size limit and was not opened. The scan of DTIC ADA003037 on archive.org (OCR text, `battleofsavoisla00nava_djvu.txt`) was read for printed pages 92 to 94; the OCR misreads some characters.
- **Stille, *Japanese Combined Fleet 1942-43*** (dokumen.pub): the page returned a maintenance notice. Nothing was read from it.
- **NHHC pages** (H-gram 010-2, Sabine II): the first refused the connection; the second was not tried again. The DANFS Sabine entry linked from HyperWar was not on the page that opened.
- **Page numbers.** HyperWar puts the `--N--` marker at the end of each printed page. The tool counted some passages one page later than the leads did (Sabine's 22 August arrival, p. 32 against p. 31; Sabine's departure from San Pedro, p. 26 against p. 25). The pages in the rows are the tool's.

## 1. The 26 leads

"Same words" means the quotation in the lead appeared on the page, word for word. "Different wording" says what the page has.

| Lead | Statement | Status | Where it is held |
| --- | --- | --- | --- |
| Q30.1 | DD 421/459 class 242 bbl a day at 15.3 kn | Same words. Table I, printed p. 252, data period 1942-1943, mean displacement 2,400 tons | `ship_class_facts` 1864 |
| Q30.1 | The same class 1,515 bbl a day at 30.4 kn | Same words | 1865 |
| Q30.1 | The class underway average 296 bbl a day at 15.7 kn | Same words. Table II, printed p. 253 | 1866 |
| Q30.1 | Japanese destroyer 30 tons a day at economical speed (Stille) | Page would not open | none |
| Q30.1 | Nagato-class battleship 150 tons a day (Stille) | Page would not open | none |
| Q30.1 | Kinkaid on 8 August: destroyers fuel for about three days at 15 knots | Different wording. Dyer p. 388 gives the 1200 entry "Fuel situation this Force becoming critical" and the three days at 15 knots, heavy ships "little more"; the tool did not return the sentence whole. Bates p. 94 has "Fuel situation in this force becoming critical. It is estimated that the destroyers have fuel for about three days" | section 4 (no entity) |
| Q30.1 | Benham three days, Grayson two (Zimmerman) | Same words: "two 'short-legged' destroyers--Benham and Grayson--had fuel enough for three and two days respectively." Footnote 35, printed p. 50 | `ship_facts` 1824, 1825 |
| Q30.1 | Enterprise fuel for three more days (Bates) | Same words, printed p. 94 | 1830 |
| Q30.1 | Carriers 17 days (Zimmerman) | Same words, with the other classes (section 2) | 1826 to 1829 |
| Q30.2 | Kanawha capacity 75,000 bbl | Same words, Carter ch. IV p. 24 | already `place_facts` 257 |
| Q30.2 | Sabine left San Pedro on 2 August, two weeks to the Fijis | Different wording. The page says "the tanker Sabine left San Pedro for the South Pacific, but could not reach the Fijis before 2 weeks had elapsed", "The next day" after an order of 1 August. The date 2 August is an inference, not on the page | section 4 |
| Q30.2 | Sabine arrived from Suva on the 22nd, stayed 2 days | Same words | already 253 |
| Q30.2 | Sabine took 32,661 and 52,909 bbl on 30 and 31 August | Same words | already 254 |
| Q30.2 | Nippon Maru departed Truk to join the fleet, 22 to 28 August | Same words: "Departs Truk to join the fleet with destroyers MINEGUMO and URANAMI." Entry headed 22-28 August | `place_facts` 703 |
| Q30.2 | Toa Maru refuelled Aoba, Kinugasa, Furutaka at Shortland on 6 September | Same words | already 269 |
| Q30.3 | Bishopsdale empty on 1 August | Same words, with the page's typo "dispacthed" | already 255 |
| Q30.3 | 225,000 bbl planned to Noumea, and the same about 2 August | Same words | already 256 |
| Q30.3 | Dublon, six tanks, 49,000 tons (Stille) | Page would not open | none; gap |
| Q30.4 | Cimarron topped off Saratoga and eight destroyers on 3 August | Same words, Bates p. 93 | `ship_facts` 1831 |
| Q30.4 | Sabine fuelled 11 ships at Espiritu Santo before sailing on the 28th | Same words | already 253 |
| Q30.4 | Chartered tankers to be fitted to supply two ships at once | Same words, Carter ch. V p. 46 (lead said pp. 45 to 46) | section 4 (no entity) |
| Q30.5 | Kinkaid's diary: critical fuel and fighter losses led Fletcher to recommend withdrawal | Different wording and page. Dyer's text at the 2330 entry cites the fighter losses and "critical fuel situation" as reasons for the recommendation; the tool placed it at p. 390, the lead at p. 391 | section 4 |
| Q30.5 | Bates: low fuel does not appear to have been the reason | Same words, printed p. 94 | no row; in section 5 |
| Q30.5 | Ghormley: "informed me he had to withdraw for fuel, I approved" | Same words, Zimmerman footnote 35, quoting Ghormley's manuscript p. 93 | section 5 |
| Q30.6 | Ghormley: 25,000 bbl a day combatants, 3,000 auxiliaries, from 23 days of issues | Same words, Carter ch. IV p. 32 | already in the earlier note (4c) and `land_unit_facts` 1100 |
| Q30.6 | Nimitz planned about 225,000 bbl a month for the carrier task force | Same words, Carter p. 25 | section 4 |

## 2. What the record gives

### 2a. What a ship burned in a day (FTP 218, `ship_class_facts` 1844 to 1872, attribute `fuel_rate_note`)

FTP 218 prints each line as gallons an hour and barrels a day, side by side. For every line below the two agree (gallons an hour x 24 / 42 = barrels a day). Tons are barrels x 0.1516 (section 3). "Steady" is a steady run at the stated speed; "underway average" is the mean over the whole data period at all speeds; "not underway" is at anchor or in port.

| Class and period | Condition | Bbl a day | Tons a day | Rows |
| --- | --- | --- | --- | --- |
| Saratoga (CV 3), 1942-43, 47,700 t | steady 15.5 kn / 21.0 kn / 29.3 kn | 2,560 / 4,927 / 14,048 | 388 / 747 / 2,130 | 1844 to 1846 |
| | underway average 17.0 kn; not underway | 3,421; 253 | 519; 38 | 1847, 1848 |
| CA 32 class, 1941-43, 13,000 t | steady 15.2 / 20.0 / 30.0 kn | 705 / 1,250 / 4,937 | 107 / 190 / 748 | 1849 to 1851 |
| | underway average 15.4 kn; not underway | 805; 82 | 122; 12 | 1852, 1853 |
| CA 33 class, 1941-43, 14,000 t | steady 14.7 / 20.0 / 29.9 kn | 711 / 1,316 / 5,102 | 108 / 200 / 773 | 1854 to 1856 |
| | underway average 15.3 kn; not underway | 871; 88 | 132; 13 | 1857, 1858 |
| CA 27 class (Chester, Louisville), 1941-43, 13,100 t | steady 14.9 / 20.1 / 29.8 kn | 675 / 1,260 / 4,969 | 102 / 191 / 753 | 1859 to 1861 |
| | underway average 16.5 kn; not underway | 950; 84 | 144; 13 | 1862, 1863 |
| DD 421/459 class, 1942-43, 2,400 t | steady 15.3 / 30.4 kn | 242 / 1,515 | 37 / 230 | 1864, 1865 |
| | underway average 15.7 kn; not underway | 296; 38 | 45; 6 | 1866, 1867 |
| BB 55 class, **1944-45**, 45,700 t | steady 14.9 / 19.8 / 25.3 kn | 1,082 / 2,098 / 5,399 | 164 / 318 / 819 | 1868 to 1870 |
| | underway average 17.5 kn; not underway | 1,798; 281 | 273; 43 | 1871, 1872 |

FTP 218 has no 1942 rows for Enterprise, Hornet or Wasp: the CV 6 page says a study "will be issued when sufficient data are available", and the CV 9 class (Yorktown, Hornet, Wasp of 1943 on) has data for 1944-1945 at 36,600 tons. The Saratoga rows are the only carrier figures for 1942. The page lists Saratoga alone under its CV 3 heading; `ships.csv` puts her in `us-lexington-class`, so the rows are filed there. The DD 445 (Fletcher) class is 1944-45 data (306 bbl a day at 15.7 kn; 1,393 at 29.7 kn) and was not filed.

### 2b. What a force burned (the only force-level figure found)

| Statement | Source | Where |
| --- | --- | --- |
| 18 August 1942: Ghormley's combatant vessels used an average of 25,000 bbl a day and his auxiliaries 3,000; a study of 23 days of actual issues showed a shortage "soon after 14 September" | Carter ch. IV p. 32 | `place_facts` 255 area; `land_unit_facts` 1100; no new row |
| Early 1943: Tulagi PT boats burned 3,000 to 7,000 gal a day and aircraft about 1,000; by the end of 1943 PT boats about 5,000 and aircraft 5,000 to 10,000 | Carter ch. VI p. 50 | none (1943 and later; no entity) |

No source read gives the daily burn of a carrier task force, a cruiser force or a destroyer division as such.

### 2c. Fuel state of Task Force 61 on 7 to 10 August 1942

Dyer ch. 10 prints "NOON FUEL REPORT AS INDICATED FOR" TU 61.1.1, 61.1.2 and 61.1.3 with Received, Expended and On Hand for two dates per ship. The page prints **no unit**. Rows `ship_facts` 1834 to 1851 hold 18 ships' lines as printed (Saratoga and Wasp are not in the tables; Gwin, Laffey and some others are in the tables but are not in `ships.csv`). The page marks three on-hand figures with "#", noted "Low Ship in Group on 8 August": MacDonough (70,213), Grayson (39,520) and Sterett (59,497).

If the figures are US gallons, the days of fuel at the first printed day's expenditure are as below. That reading is mine; the page does not say it. It is supported by one check: Benham's 67,872 divided by 18,925 is 3.6 days and Grayson's 39,520 by 18,925 is 2.1 days, against Zimmerman's three and two.

| Ship | On hand, first date | Expended, first date | Days at that rate | Tons on hand (gal / 277) |
| --- | --- | --- | --- | --- |
| Enterprise (7 Aug) | 760,116 | 61,656 | 12.3 | 2,744 |
| North Carolina (8 Aug) | 877,570 | 68,966 | 12.7 | 3,168 |
| Portland | 395,328 | 60,948 | 6.5 | 1,427 |
| Atlanta | 199,556 | 26,297 | 7.6 | 720 |
| Balch (7 Aug) | 98,227 | 24,980 | 3.9 | 355 |
| Maury | 69,814 | 21,951 | 3.2 | 252 |
| Benham | 67,872 | 18,925 | 3.6 | 245 |
| Grayson | 39,520 | 18,925 | 2.1 | 143 |
| Farragut | 84,696 | 19,835 | 4.3 | 306 |
| MacDonough | 70,213 | 16,315 | 4.3 | 253 |
| Phelps | 96,382 | 19,193 | 5.0 | 348 |
| Worden | 72,850 | 18,011 | 4.0 | 263 |
| Aaron Ward (7 Aug) | 128,252 | 12,200 | 10.5 | 463 |
| Farenholt | 68,050 | 16,104 | 4.2 | 246 |
| Lang | 71,553 | 18,371 | 3.9 | 258 |
| Stack (7 Aug) | 78,676 | 15,088 | 5.2 | 284 |
| Sterett | 59,497 | 19,509 | 3.0 | 215 |
| Dale | 164,456 | 3,693 | 44.5 | 594 |

Dale's expenditure is far below every other destroyer's and gives an unusual figure; the page gives no reason. Her row is filed as printed.

Bates p. 94 gives the state as percentages of capacity on 8 August: destroyers of TG 61.1.1 about 75 per cent, those of TG 61.1.2 and 61.1.3 about 42 and 44 per cent, cruisers about 50 per cent or better. Zimmerman gives days at each ship's own 1 to 8 August average: carriers 17, North Carolina 18, cruisers about 11, destroyers about seven, Benham three, Grayson two (`ship_facts` 1824 to 1829). Fletcher's 9 September reply, quoted in Dyer p. 393, has CTF 16's destroyers, except Gwin, with fuel for two days at 15 knots on 7 August, and CTF 18's destroyers with fuel for 31 hours at 25 knots at noon on 8 August (section 4).

### 2d. Oilers of the South Pacific Force

The oilers have no entity in `ships.csv` and `ship_class_facts` has no class for them, so there are no rows for these figures. Capacities are statements of different kinds and do not agree (section 5).

| Oiler | Capacity | Source |
| --- | --- | --- |
| Kanawha (station oiler, Tongatabu) | 75,000 bbl (= 11,370 t) | Carter p. 24 (`place_facts` 257) |
| Platte, Cimarron | each 12,000 tons of fuel oil and 500,000 gal of aviation gasoline, loaded for the July rehearsal at Fiji | Zimmerman fn. 35 |
| Kaskaskia | 13,000 tons of oil, same occasion | Zimmerman fn. 35 |
| Cimarron (AO 22) | fuel oil 87,246 bbl (13,226 t); total 116,814 bbl; 19.0 kn; 3 main cargo pumps of 2,000 gal a minute | FTP 218 Appendix B, printed pp. 14 to 15 |
| Platte (AO 24), Sabine (AO 25), Kaskaskia (AO 27), Guadalupe (AO 32) | fuel oil 68,054 bbl each (10,317 t); totals 103,229, 103,269, 102,857, 100,571 bbl; 18.0 kn; 3 pumps of 2,000 gal a minute | same |
| Neosho (AO 48) | fuel oil 71,000 bbl; total 100,000; 16.5 kn | same |
| Neches (AO 47) | fuel oil 74,000 bbl; total 103,737; 16.5 kn | same |
| Tippecanoe (AO 21) | fuel oil 63,509 bbl; total 72,359; 10.5 kn; 2 pumps | same |

FTP 218 is a 1945 publication, and Appendix B does not say that its figures are 1942 loadings. Kanawha is not in Appendix B.

Dated movements:

| Date | Statement | Source |
| --- | --- | --- |
| 10 to 27 Jul | Mobilube at Tongatabu, pumped the rest of her cargo into Kanawha, left for San Pedro | Carter p. 24; `place_facts` 696 |
| 20 Jul | Kaskaskia scheduled to leave Pearl; Cimarron and Platte to be at Tongatabu | Carter pp. 24 to 25 |
| 21 Jul | Platte ordered to pump her remaining oil into Cimarron; the text says she took aboard 93,000 bbl and rejoined Task Force 11 (the tool's piece does not say who took the barrels) | Carter p. 25 |
| 1 Aug | Bishopsdale empty, sent to Brisbane to refill; then a mine put her out of service | Carter p. 25; 255, `place_facts` 697 |
| 1 Aug | E. J. Henry and Esso Little Rock, due at Noumea, already diverted, one to Efate and one to Suva | Carter p. 25; 698 |
| after 1 Aug | Sabine left San Pedro, two weeks at least to the Fijis | Carter p. 26 |
| 12 Aug | McCain: ships at Espiritu Santo need fuel oil and Diesel at once and 300,000 gal of aviation gasoline within 7 days; the nearest storage was at Suva | Carter p. 30; 699 |
| 14 Aug | Ghormley: Fletcher's carrier groups, after one week of normal cruising, "completely emptied the oilers Platte and Kaskaskia"; Cimarron held to keep the force fuelled | Carter p. 32 |
| 16 Aug | Nimitz ordered Guadalupe to sail with Task Force 17 | Carter p. 32 |
| 22 to 28 Aug | Sabine at Espiritu Santo (arrived the 22nd, stayed 2 days, went to sea to help fuel Task Force 61; back at Espiritu Santo, fuelled 11 ships, sailed the 28th for Noumea) | Carter p. 32; 253 |
| 30 to 31 Aug | Sabine at Noumea took 32,661 and 52,909 bbl from J. W. Van Dyke and Pacific Sun (4,951 and 8,021 t) | 254 |
| late Sep | A tanker sent to Espiritu Santo to fuel naval task groups | Carter p. 39; 700 |
| Oct | Request that all chartered tankers sent to the South Pacific be fitted with hose and facilities to supply two ships at once at maximum rate, to relieve "naval oilers, which had been going at a killing pace" | Carter p. 46 |

Japanese oilers (all from the combinedfleet tabular records; tons as printed):

| Oiler | Entries read | Rows |
| --- | --- | --- |
| Nippon Maru | 1 Aug 402 t to Kumano (0855 to 1220) and 410 t to Suzuya (1220 to 1500); 8 Aug went via Macassar and Tarakan, Borneo, to load oil, then joined the fleet north of the Solomons; 22 to 28 Aug left Truk to join the fleet; 7 to 8 Oct left Truk escorted, went to the north pass and returned; 9 Oct at Truk fuelled Kagero, Akashi, Isonami; 10 Oct she and Kenyo Maru left Truk; 12 Oct at sea fuelled destroyers and Isuzu; 16 to 18 Oct fuelled the fleet at sea, 420 t to Atago on the 18th | `ship_facts` 1852, 1853, 1856; `place_facts` 703 to 705 |
| Kyokuto Maru | 4 and 8 Aug bunkering destroyers; August went to Tarakan; 30 Aug at sea 480 t to Kumano and 700 t to Suzuya; 8 Sep at Truk fuelled Kumano; 25 Sep at Truk (`place_facts` 277); 11 Oct sortie from Truk "to provide refueling for Vice Admiral Nagumo's Striking Force"; 17 to 24 Oct "The Supply Force refuels the fleet at sea for eight consecutive days"; 18 Oct 2,153 t to Atago and Takao, 900 t of it to Atago; 19 Oct 840 t to Kongo; 21 Oct fuelled destroyers while steaming at 14 knots; 22 Oct Kongo, Haruna and 150 t to Atago | `ship_facts` 1854, 1855, 1857, 1858; `place_facts` 706 |
| Toa Maru | "served mainly as a station tanker at Shortland"; her only fuel entry from July to November 1942 is 6 September | `place_facts` 702, 269 |

Kongo (840 t on 19 October) and Haruna are not in `ships.csv`; their figures are in this table only. Capacity of the Japanese oilers is on none of the pages (Toa Maru is "10,052-ton merchant tanker" at laying down).

### 2e. Fuel on hand and tankage at the bases

No dated stock of fuel at any base was found. What the record holds is tankage and shipments:

| Place | Statement | Where |
| --- | --- | --- |
| Noumea | Plan of July: 225,000 bbl by chartered tankers, the same again about 2 Aug, and 200,000 bbl "ordered delivered every 15 days"; by late fall the Ducos farm held 370,000 bbl black oil and 30,000 bbl Diesel | 256, 258; Carter p. 26 |
| Tongatabu | "no facilities other than a little storage convenience established by ourselves" (Carter p. 26); Kanawha the station oiler | 257 |
| Espiritu Santo | no tank farm in 1942; fuelling by station tankers and incoming oilers until Nov 1943; reserve at 1943 "about five tanker-loads" | 260, 262 |
| Efate (Vila) | seven 1,000-bbl aviation gasoline tanks and others; no black-oil tank named; date not on the passage | `place_facts` 701 |
| Tulagi | in Carter's account ten 1,000-bbl tanks, 60,000 bbl Diesel, 280,000 bbl fuel-oil farm (a statement of the end of 1942 or later; row 259 holds a doubt about its date) | 259 |
| Truk, Rabaul | no figure | gap |

### 2f. How many ships an oiler served

| Statement | Date | Source |
| --- | --- | --- |
| Cimarron topped off Saratoga and eight destroyers | 3 Aug 1942 | Bates p. 93; `ship_facts` 1831 |
| Sabine fuelled 11 ships at Espiritu Santo between her arrival on the 26th and sailing on the 28th | 26 to 28 Aug | 253 |
| Omurosan Maru at Shortland: 11, 20, 19 and 4 vessels on 23, 24, 26 and 27 October | Oct | 273 to 276 |
| Platte fuelled the whole Enterprise group "dawn to dusk" | Jan 1942 | Wildenberg ch. 17 |
| "only four destroyers were filled during a ten-hour fueling period" in a heavy swell, Wake relief | Dec 1941 | Wildenberg ch. 17 |
| "Two large ships may be fueled simultaneously from one oiler under favorable conditions with some sacrifice of pumping rate" (Halsey) | 1945 | Wildenberg ch. 18 |

No source read says how many ships lay alongside one oiler together in August to November 1942. Carter p. 46 is a request for fittings, not an event.

## 3. Conversions

| Conversion | Value | Source |
| --- | --- | --- |
| 1 barrel | 42 US gallons | FTP 218 Appendix A, printed p. 13 |
| 1 ton of fuel oil | 277 gallons | same |
| Tons to barrels (long tons) | 6.595; barrels to tons 0.1516 (fuel oil); diesel 7.476 and 0.1338 | same; the page lists "Tons (long)" as 2,240 lb |
| Specific gravity assumed by those factors | 0.9708 fuel oil, 0.8566 diesel at 60/60 F | same |

This settles the earlier note's assumption: the factors are printed as long-ton factors, so tons of fuel oil converted at 277 gallons are long tons. The Japanese tons are not defined by their pages and are used as printed.

Arithmetic in this note:

- Barrels a day x 0.1516 = tons a day (Ghormley's 28,000 bbl a day = 4,245 t a day).
- Gallons x 24 / 42 per hour gives barrels a day; gallons / 277 gives tons (Dyer table; on the assumption that its unit is the US gallon).
- Japanese rate: 402 t from 0855 to 1220 is 3 h 25 min, 118 t an hour; 410 t from 1220 to 1500 is 2 h 40 min, 154 t an hour; 480 t from 0640 to 0830 is 1 h 50 min, 262 t an hour; 700 t from 1237 to 1514 is 2 h 37 min, 268 t an hour; 420 t from 0630 to 0738 is 1 h 8 min, 370 t an hour; 900 t from 1625 to 2010 is 3 h 45 min, 240 t an hour. The earlier note's table 4a holds the same entries for 1 and 30 August and 18 October at the same rates; the 18 October Nippon Maru entry (420 t) and the 22 October 150 t entry are new.
- Capacity: 75,000 bbl x 0.1516 = 11,370 t; 68,054 x 0.1516 = 10,317 t; 87,246 x 0.1516 = 13,226 t.

## 4. Statements with no row

No entity exists for a task force, a force's commander or an oiler, and the earlier note's section 4 uses the same rule.

| Statement | Source |
| --- | --- |
| 29 Jul 1942: Fletcher reported that his force would be "short over two million gallons of fuel oil" after the day's scheduled fuelling | Dyer ch. 10 p. 388 |
| 3 Aug: Fletcher, "If no tankers Efate for Task Force 62, top off situation may be serious" | Dyer p. 388 |
| Turner: "Enroute from Koro to the Solomons my big worry was OIL, OIL, OIL." The author calls the fuel problem "AFFAG" | Dyer p. 387, 388 |
| 8 Aug, 1200, Kinkaid (TU 61.1.2): "Fuel situation this Force becoming critical"; destroyers three days at 15 knots, heavy ships "little more" | Dyer p. 388; Bates p. 94 |
| 8 Aug, 1807, Fletcher to Ghormley: fighters down "from 99 to 78", "immediate withdrawal of my carriers", "Request tankers be sent forward immediately as fuel running low" | Dyer p. 383; Bates p. 92 gives "Request you send tankers immediately to rendezvous decided by you as fuel running low" and Zimmerman p. 50 "a shortage of fuel and of the loss of 21 of his 99 planes" |
| 8 Aug, 2330, Kinkaid's diary: enemy air attack, fighter losses and "critical fuel situation" led Fletcher to recommend withdrawal | Dyer p. 390 (the tool's page; the lead gave 391) |
| 7 Aug: CTF 16's destroyers, except Gwin, fuel for two days at 15 knots; noon 8 Aug: CTF 18's destroyers fuel for 31 hours at 25 knots | Fletcher's reply of 9 Sep 1942, Dyer p. 393 |
| TF 16 "did not actually fuel until 1700 on 10 August"; Fletcher withdrew to a point 500 miles south of the unloading area and refuelled on 10 August | Dyer p. 393; Carter p. 28 |
| Fletcher's 1963 list of fundamentals for the decision does not name fuel; his despatch "didn't say anything about needing to withdraw to fuel"; COMSOPAC's later report cited only the fuel shortage | Dyer pp. 391 and 392 |
| Ghormley, 2 Aug, to Fletcher, after Callaghan reported the Fiji conference: shows Fletcher "had planned to withdraw his task force, and therefore also the transports, before D-day plus three"; Ghormley, not protesting, "thereby gave tacit approval" | Zimmerman fn. 35 |
| 23 Aug 1942: ONI, "a combination of fueling needs and imperfect intelligence reports concerning the location of enemy carriers" deprived the force of the Wasp group; Carter p. 36, "if our logistic support had been good, the Wasp group would have been refueled within supporting distance" | `ship_facts` 1832, 1833 |
| Carter p. 25: "about 225,000 barrels a month for the carrier task force" (Nimitz's plan of 9 July); p. 32: "a study based on the actual issues of 23 days indicated that soon after 14 September there would be a fuel shortage" and the action that followed "prevented any further serious shortage" | Carter ch. IV |
| Oct 1942, Carter p. 46: "naval oilers, which had been going at a killing pace"; tankers to be fitted to supply two ships simultaneously | Carter ch. V |
| Jan 1942: the Wake raid was cancelled after Neches was torpedoed, "as the force could not proceed without fueling at sea and Cincpac had no other tanker to spare"; Nimitz then put at least one Cimarron-class oiler with each raid; Dec 1941: a Wake relief force was 425 miles from Wake and recalled, having filled only four destroyers in ten hours | Wildenberg ch. 17 |
| 1942 oiler assignments (Table 17): Platte and Sabine, Marshalls raids; Cimarron and Sabine, Tokyo raid; Kaskaskia, Tippecanoe and Neosho, Coral Sea; Platte, Cimarron and Guadalupe, Midway | Wildenberg ch. 17 |

## 5. Where sources disagree

- **Why the carriers withdrew on 8 August.** Fletcher's 1807 message and Kinkaid's diary give fuel (with fighter losses); Ghormley's later account says he approved because Fletcher said he had to withdraw for fuel (Zimmerman fn. 35). Bates (p. 94) concludes from the ships' logs that low fuel "does not appear" to have been the reason. Fletcher in 1963 says his despatch said nothing about withdrawing to fuel, and gives other reasons. Zimmerman's footnote adds that Ghormley's 2 August dispatch shows a withdrawal before D+3 was already planned. These are held as statements; none is preferred.
- **Days of fuel on 8 August.** Enterprise: three more days "of operations" (Bates, row 1830) against the carriers 17 days (Zimmerman, rows 1826 to 1828) and, by my arithmetic on the Dyer table in gallons, 12 days at her 7 August expenditure. Zimmerman's figures are at the 1 to 8 August average and Bates's basis is not stated; neither passage explains the other. Rows 1826 and 1830 are marked `disputed`. Cruisers: Zimmerman about 11 days; the table gives Portland 6.5 and Atlanta 7.6 days at the first day's expenditure. Destroyers: Zimmerman about seven days; the table gives 3.0 to 5.2 days for most.
- **Tanker capacity.** Zimmerman has Platte and Cimarron at 12,000 tons each and Kaskaskia 13,000; FTP 218 Appendix B has fuel-oil capacity for Cimarron 13,226 t, Platte 10,317 t and Kaskaskia 10,317 t. Zimmerman's figures are what the tankers loaded for the July rehearsal; Appendix B is a 1945 table of fuel-oil capacity. They are different quantities and are not reconciled.
- **Page numbers** of the same passage differ by one between the leads and the tool; the cause is probably the marker convention above.

## 6. Still a gap

`gaps.csv` has nine new lines. In short:

- Fuel on hand at any base, by date, for either side; the Dublon capacity (the page that carried it would not open).
- A daily burn for a whole task force; no 1942 Enterprise, Hornet or Wasp figure in FTP 218.
- A round trip for any one tanker; the capacity of any Japanese oiler.
- A dated event with ships alongside one oiler together.
- A Japanese carrier or battleship movement attributed to fuel in 1942 words. The oiler records show the Supply Force fuelling the fleet at sea for eight consecutive days in October and Nippon Maru's trip to Borneo to load oil, and no more.
- The unit of the Dyer noon fuel tables (the page has none).
- Pages that would not open: Zimmerman's PDF, Bates's PDF, Stille's page, the NHHC H-gram and Sabine pages.
