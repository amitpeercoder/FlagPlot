# Rear bases and supply, 1942

Read on 2026-11-23 for the architect, who will write a rule for which bases are rear bases and what they get. The question was put per base and period, for the three operations the scenarios for play cover:

- the Coral Sea, May 1942;
- Midway, June 1942;
- Guadalcanal, 7 August to 14 September 1942.

For each base it asks four things:

1. Which bases were each side's main supply and fuel points, and from where each was fed.
2. Stocks on hand and rates of arrival, where a source states them.
3. Whether the enemy could and did interrupt the base's supply.
4. Port capacity where stated.

The note settles nothing. It proposes no rule and no figure beyond the last paragraph's note of what the facts seem to allow.

It follows three earlier passes and does not repeat them: `docs/research/oob/supply-levels-and-convoy-cycles.md` (levels of supply, Noumea's congestion, Guadalcanal in practice), `docs/research/oob/fuel-on-station.md` and `docs/research/oob/fuelling-in-port.md` (fuel in the South Pacific, the oilers, tankage). Rows of those passes are cited below as "on file" with their ids.

## How it was read

Four Sonnet readers each took a share:

- **A:** the Japanese bases (Truk, Rabaul, Shortland, Lae) and what fed them;
- **B:** Port Moresby and the Australian ports;
- **C:** Midway, Pearl Harbor, Wake and Kwajalein;
- **D:** the American South Pacific, from the West Coast to Guadalcanal.

Each reader saved every document as text outside the repository and staged each statement with its exact words, the saved file and the line. The lead then:

- **Checked every quotation by program** against its saved text (whitespace ignored). All 216 staged statements passed, and so did the lead's own seven.
- **Extended each quotation of under 140 characters to its whole sentence** as saved, where that did not cross a page marker. The row's note says so.
- **Read the points the main figures rest on in the saved texts:**
  - the Graybook's Midway gasoline entries of 23 May, 2 June and 3 June;
  - Heinl's footnote 31;
  - Carter's chapter IV on the July plan and the shortage;
  - Miller on the 18 September drums and Turner's 3,000-ton order;
  - Milner on Moresby's one ship at a time;
  - the JANAC rows.
- **Read Gill's chapters 1 to 3 of *Royal Australian Navy 1942-1945* himself**, because reader B could not reach them. The seven Gill rows are the lead's.
- **Chose the rows.** 162 of the readers' 216 statements became rows, with the lead's seven from Gill: 169 in all. The 54 left out either repeat a row on file, fall outside the periods without bearing on them, or are a reader's synthesis rather than a source's statement.

Every new row is `unconfirmed`: one source's statement, not yet read against a second.

**What the quote check does not show.** It does not show that a statement was filed against the right base. For the 18 JANAC rows the base is the reader's reading of JANAC's coordinates; JANAC prints positions only, and each row's note says so.

**OCR.** The two Graybook volumes are machine-read typescript. Their quotations keep the reading errors as the saved text has them, for example "satisf actor y" and "1\IIDiHAY" for MIDWAY.

**Code words in the Graybook.** Its code words are read as WHITE POPPY for Noumea, BUTTON for Espiritu Santo, ROSES for Efate, CACTUS for Guadalcanal and RINGBOLT for Tulagi. The code words on file are `place_facts` 14300 to 14308.

## What the rows hold

| File | Ids | Rows |
| --- | --- | --- |
| `place_facts.csv` | 14309 to 14477 | 169 |
| `places.csv` | 6111 to 6116 (`palau`, `japan`, `brisbane`, `cairns`, `kwajalein`, `san-francisco`) | 6 |
| `sources.csv` | 16022 (`ussbs-transportation-1947`) | 1 |
| `gaps.csv` | the last 20 entries | 20 |

`place_facts.csv` is not read by `oob-check`, and its attributes are free (`docs/research/oob/step2-record.md`). This pass reuses `port_note`, `fuelling_note`, `fuel_oil_storage_note`, `aviation_fuel_storage_note` and `fuel_pump_rate_note`. It adds five text attributes, each with its figure in words in `value` (`port_note`, already in use, is in the table for the key it is shown by):

| Attribute | Holds | Shown below as |
| --- | --- | --- |
| `stock_note` | A stock on hand: tons, gallons, barrels, days, or "no storage" | stock |
| `arrival_rate_note` | Ships, tons or barrels arriving, or planned to arrive, in a period | rate |
| `fed_from_note` | Where a base's supply or fuel came from, and by what ships or route | fed from |
| `supply_interrupted_note` | A ship sunk or damaged on a base's line or in its harbour, a raid on it, or a statement that the line was or was not cut | cut or hit |
| `base_role_note` | What a base was for, its distance, its command, or a judgement of it as a base | role |
| `port_note` | Berths, wharves, lighterage, cranes, unloading (an attribute already in use) | port |

### Sources read

**Japanese side**

- JANAC's merchant losses (`janac-1947`).
- The combinedfleet.com records of about 50 ships, oilers and transports (`combinedfleet`).
- Bullard's translation of Senshi Sosho (`bullard-jpn-army-ops-south-pacific`), whose pages are counted by the PDF's page breaks.
- The USSBS *Interrogations*, nos. 8, 10, 11, 13, 16, 33, 37, 45, 46, 49, 52, 53, 57, 60, 64, 66, 70, 72 and 74.
- The USSBS *Campaigns of the Pacific War*, chapters 1 to 16.
- The USSBS *The War Against Japanese Transportation* (new source).
- Miller's *Cartwheel*, chapters 1 to 5, which gave nothing.

**Southwest Pacific**

- McCarthy, *South-West Pacific Area: First Year*, chapters 1, 2, 4, 5 and 7.
- Gill, *Royal Australian Navy 1942-1945*, chapters 1 to 3.
- Milner, *Victory in Papua*, chapters 2 to 6.
- Leighton and Coakley.
- Craven and Cate, vol. I, chapters 12 and 13.
- Pacific Wrecks on the Macdhui.

**Central Pacific**

- The Graybook, vol. 1.
- Heinl, *Marines at Midway*.
- The NAS Midway war diaries.
- Bates, *Midway*.
- *Building the Navy's Bases*, ch. 22.
- Carter, chapters I to III.
- Spennemann, *To Hell and Back* (Wake).

**South Pacific**

- The Graybook, vol. 2.
- Carter, chapters IV to VI.
- *Building the Navy's Bases*, chapters XXIV and XXV.
- Leighton and Coakley; Bykofsky and Larson; Ballantine.
- Craven and Cate, vol. IV, chapters 1 to 3.
- Miller, *Guadalcanal*, chapters III to VII.

### Could not get

Each is a gap entry.

- **USSBS, *The Allied Campaign Against Rabaul* (1946).** The likeliest source for Rabaul's stocks and arrivals. The HyperWar file names were guessed and returned 404, its directory listing is forbidden, an archive.org search found nothing, the Library of Congress refused, and only a Google Books catalogue entry was found.
- **USSBS, *Oil in Japan's War*, *The Reduction of Truk* and *The Reduction of Wake Island*.** Not found for the same reasons.
- **Japanese Monograph 98.** The 136 MB scan timed out.
- **Masterson, *U.S. Army Transportation in the Southwest Pacific Area*.** Only a Scribd copy was found.
- **Dyer, *The Amphibians Came to Conquer*.** The reader's fetch of its index returned no links.
- **Morison, vol. V.** Not sought.

## Per base: what each source states

The rows are below, base by base, oldest first. "(low)" marks a row graded low. Under each side's heading, the rows already on file are named first.

### Already on file, by base

All ids below are `place_facts` unless another file is named.

**Japanese side**

- **Rabaul:**
  - Simpson Harbour has room for at least 20 ships of 10,000 to 15,000 tons (124).
  - Emergency repairs were done at Rabaul, long ones at Truk (125).
- **Truk:**
  - Akashi, the repair ship (171, 177, 178).
  - The oilers Nippon Maru and Kyokuto Maru fuelled there in Aug to Oct 1942 (`fuel-on-station.md`, section on the Japanese oilers; 703 to 706).
- **Shortland:**
  - Toa Maru was "mainly" a station tanker there (702).
  - No repair ship was found (157).
- **The drum runs and the 17th Army's plans:** `land_unit_facts` 1178 to 1185 and `troop_lift_facts` 183 to 188.
- **Wake:**
  - Seaplane facilities but no ship base (14101).
  - American-built tanks for 150,000 bbl of gasoline (14115).

**Southwest Pacific**

- **Port Moresby:** its airfield of 1933 (956).
- **Milne Bay:** ships of 4,000 to 5,000 tons unloaded onto pontoons (85).

**Central Pacific**

- **Midway:**
  - the gasoline explosion of 22 May (6011, 6012);
  - the 22 tanks of 2,500 gallons begun in 1940 (6010);
  - the fuel lines and tanks destroyed on 4 June (6013 to 6015);
  - the lagoon channel and anchorage (6020, 6021);
  - bombs stored in May (6008).
- **Pearl Harbor:** the fuel barges' rates of 1941 (292 to 294).

**South Pacific**

- **Noumea:**
  - ships waiting and discharged (115 to 116, 217 to 221, 224 to 230, 234 to 237);
  - the July 1942 fuel plan and the Bishopsdale (255, 256, 697, 698);
  - the Ducos tank farm of late 1942 (258) and the cancelled tank farm (263);
  - 200,000 bbl every 15 days and Ghormley's 28,000 bbl a day (`land_unit_facts` 1099 to 1101).
- **Espiritu Santo:**
  - drums in Segond Channel and no bulk storage (21);
  - no docks (24, 222);
  - about five tanker-loads of storage, November 1942 (232);
  - station tankers until 1943 (260);
  - McCain's request of 12 Aug (699);
  - a tanker sent in late September (700);
  - 300,000 gal moved in for the B-17s (924).
- **Efate:**
  - anchorages only (223);
  - small tanks (10, 701);
  - a chartered tanker diverted there (698).
- **Tongatabu:** the station oiler Kanawha (257, 696); the ammunition ship Rainier (915, 916).
- **Tulagi:** the late-1942 tank farm (259); the harbour (181).
- **Henderson Field and Lunga:**
  - fuel from drums by hand (54, 201);
  - 400 drums found on 15 Oct (238);
  - Harmon's 250,000 gallons (239);
  - 35,800 gallons for 20 B-17s (240);
  - the beaches (58, 231);
  - 5 units of fire and 3 days' rations on 16 Aug (919);
  - rations and supply by date (`land_unit_facts` 1102, 1123 to 1131, 1152 and others; `supply-levels-and-convoy-cycles.md`, section 2).

### Japanese side: Japan, Palau, Truk, Rabaul and the forward bases

#### `japan`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14341 | rate | 1941-12 to 1942-07 | Army shipping: 2,100,000 tons conscripted at the start of the war, planned to fall to about 1,000,000 tons by July 1942 | `bullard-jpn-army-ops-south-pacific`, page 78 of the PDF read, by its page breaks |
| 14340 | rate | 1942-06 to 1942-07 | Imperial Headquarters had provided about 100,000 tons of shipping for the FS Operation; on its cancellation 50,000 tons was judged right for the 17th Army | `bullard-jpn-army-ops-south-pacific`, page 81 of the PDF read, by its page breaks |
| 14333 | fed from | 1942-11 | Oiler KOKUYO MARU departed Kure for Rabaul 14 Nov 1942, arrived 27 Nov | `combinedfleet`, entry 14 November 1942 |

#### `palau`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14338 | role | 1942-07 to 1942-08 | 17th Army staff request (July 1942): 20,000 tons of shipping used for the sea-route campaign will return to Palau in mid-August | `bullard-jpn-army-ops-south-pacific`, page 111 of the PDF read, by its page breaks |
| 14339 | role | 1942-07 to 1942-08 | 17th Army staff asks for around 50,000 tons of shipping to return from Palau in early August 1942 for the 35th Infantry Brigade (answer: that will be dealt with) | `bullard-jpn-army-ops-south-pacific`, page 111 of the PDF read, by its page breaks |

#### `truk`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14334 | fed from | 1942-01 | Provisions ship IRAKO sailed Hashirajima for Truk and Saipan carrying 500 tons of vegetables and other food stores, 6 Jan 1942 | `combinedfleet`, entry 6 January 1942 |
| 14321 | cut or hit | 1942-03 | Tanker KAIJO MARU NO 2 (8,636 tons) sunk by a US submarine 1 Mar 1942 at 4-52N 151-20E, south of Truk on the Truk-Rabaul line | `janac-1947`, merchant losses, chronological list |
| 14322 | cut or hit | 1942-08 | Transport BRASIL MARU (12,752 tons) sunk by a US submarine 5 Aug 1942 at 9-50N 150-38E, south-east of Truk | `janac-1947`, merchant losses, chronological list |
| 14323 | cut or hit | 1942-08 | Cargo NANIWA MARU (4,353 tons) sunk by a US submarine 3 Aug 1942 at 7-37N 150-18E, near Truk; PALAO MARU (4,495 tons) the next 3 days at 9-04N | `janac-1947`, merchant losses, chronological list |
| 14324 | cut or hit | 1942-10 | Cargo SHUNKO MARU (6,781 tons) sunk by a US submarine 14 Oct 1942 at 5-35N 144-25E on the approach to Truk from the west | `janac-1947`, merchant losses, chronological list |
| 14332 | fuelling | 1942-10 | At Truk 17 Oct 1942 oiler KENYO MARU arrived empty; battleships MUTSU and YAMATO each off-loaded 4,500 tons of fuel to her to refuel fleet units for Guadalcanal | `combinedfleet`, entry 17 October 1942 |
| 14346 | fed from | 1942-12 to 1943-12 | Auxiliary carriers CHUYO, UNYO and TAIYO ferried aircraft from the Empire to Truk and Rabaul in 1942 and 1943 (CHUYO 50 to 60 planes a trip, about 30 unloaded at Truk on one trip in Dec 1942) | `ussbs-interrogations`, Interrogation summary |

#### `rabaul`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14345 | avgas | 1942-01 to 1943-12 | Rabaul interrogation: all aircraft at Rabaul were fuelled from 200-litre drums (low) | `ussbs-interrogations`, Interrogation of 27 Oct 1945 |
| 14349 | role | 1942-01 to 1942-12 | USSBS: throughout 1942 Rabaul was the headquarters for joint Army-Navy air operations against Allied positions in the Solomons and New Guinea | `ussbs-campaigns`, Chapter 8 |
| 14330 | fuelling | 1942-02 | Rabaul, 18 Feb 1942: auxiliary KIYOKAWA MARU received 300 tons of L1 oil from tanker HOYO MARU, and AA ammunition and aviation gasoline in tins from NANKAI MARU No. 2 | `combinedfleet`, entry 18 February 1942 |
| 14309 | cut or hit | 1942-04 | Transport KOMAKI MARU (8,525 tons) sunk by US Army aircraft on 18 Apr 1942 at 4-12S 152-10E (Rabaul harbour) | `janac-1947`, merchant losses, chronological list |
| 14331 | port | 1942-04 | Transport KOMAKI MARU arrived at Simpson Harbor alongside pier No. 3 on 17 Apr 1942 and began unloading bombs and ammunition | `combinedfleet`, entry 17 April 1942 |
| 14311 | cut or hit | 1942-05 | Converted salvage vessel SHOEI MARU (5,644 tons) sunk by a US submarine 12 May 1942 at 4-51S 152-54E, off Cape St George | `janac-1947`, merchant losses, chronological list |
| 14343 | fed from | 1942-06 to 1943-01 | Convoy escort commander Matsuyama: June 1942 to Jan 1943 convoys ran principally from Rabaul carrying Army transports to Buna, Wewak and Madang and Navy landing parties to Milne Bay | `ussbs-interrogations`, Interrogation of 31 Oct 1945 |
| 14310 | cut or hit | 1942-07 | Passenger-cargo TENZAN MARU (2,776 tons) sunk by a US submarine 8 Jul 1942 at 4-00S 151-50E, west of Rabaul | `janac-1947`, merchant losses, chronological list |
| 14312 | cut or hit | 1942-08 | Transport MEIYO MARU (5,628 tons) sunk by a US submarine 8 Aug 1942 at 4-52S 152-43E, the Rabaul approach on the day of the Savo recall | `janac-1947`, merchant losses, chronological list |
| 14335 | fed from | 1942-09 | Transport NAMINOUE MARU left Saeki for Rabaul 27 Sep 1942 in convoy "Oki Part 1" carrying 290 soldiers for the 17th Army, escorted by a destroyer and a patrol boat | `combinedfleet`, entry 27 September 1942 |
| 14336 | fed from | 1942-09 | Transport CHOKO MARU left Saeki for Rabaul 29 Sep 1942 in convoy Oki No. 4 with HAMBURG MARU and IKUSHIMA MARU | `combinedfleet`, entry 29 September 1942 |
| 14350 | cut or hit | 1942-09 to 1942-12 | USSBS Transportation: in the fall of 1942 about 30,000 tons of shipping were sunk by aircraft in the Southwest Pacific area, the Fifth Air Force efforts being mainly against Rabaul harbour (low) | `ussbs-transportation-1947`, pdftotext default mode, chapter IV, The Attack in 1942 |
| 14313 | cut or hit | 1942-10 | Aircraft ferry KATSURAGI MARU (8,033 tons) sunk by a US submarine 1 Oct 1942 at 5-51S 153-18E, off Cape St George | `janac-1947`, merchant losses, chronological list |
| 14314 | cut or hit | 1942-10 | Transport NAMINOUE MARU (4,731 tons) sunk by a US submarine 7 Oct 1942 at 3-51S 151-21E, on the Rabaul run | `janac-1947`, merchant losses, chronological list |
| 14315 | cut or hit | 1942-10 | Cargo SUMIYOSHI MARU (1,921 tons) sunk by a US submarine 14 Oct 1942 at 3-20S 150-03E (north-west of Rabaul) | `janac-1947`, merchant losses, chronological list |
| 14316 | cut or hit | 1942-10 | Passenger-cargo CHOKO MARU (6,783 tons) sunk by a US submarine 21 Oct 1942 at 3-30S 150-30E | `janac-1947`, merchant losses, chronological list |
| 14317 | cut or hit | 1942-10 | Converted net tender KOTOBUKI MARU NO 5 (720 tons) sunk by US Army aircraft 25 Oct 1942 at 4-13S 152-11E (Rabaul harbour) | `janac-1947`, merchant losses, chronological list |
| 14318 | cut or hit | 1942-12 | Transport ITALY MARU (5,859 tons) sunk by US Army aircraft 27 Dec 1942 at 4-15S 152-10E (Rabaul harbour) | `janac-1947`, merchant losses, chronological list |
| 14319 | cut or hit | 1942-12 | Cargo TSURUGISAN MARU (3,108 tons) sunk by US Army aircraft 27 Dec 1942 at 4-13S 152-11E (Rabaul harbour) | `janac-1947`, merchant losses, chronological list |
| 14320 | cut or hit | 1942-12 | Passenger-cargo TOMIURA MARU (3,821 tons) sunk by US Army aircraft 30 Dec 1942 at 4-15S 152-10E (Rabaul harbour) | `janac-1947`, merchant losses, chronological list |

#### `shortland-islands`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14328 | fuelling | 1942-09 | TOA MARU at Shortland refuelled heavy cruisers on 6 Sep 1942 (AOBA, KINUGASA, FURUTAKA), after leaving Truk on 1 Sep | `combinedfleet`, entry 6 September 1942 |
| 14347 | role | 1942-10 | USSBS: during the Oct 1942 build-up additional Japanese troops and supplies were being concentrated in the Rabaul-Shortland area, with Kiyokawa Maru seaplanes searching from Shortland | `ussbs-campaigns`, Chapter 7 |
| 14327 | role | 1942-12 | Oilers TOA MARU and FUJISAN MARU were at Shortland on 10 Dec 1942 and were attacked by 11 B-17s and six fighters; TOA MARU was damaged (repairs to Mar 1943) | `combinedfleet`, entry 10 December 1942 |
| 14329 | fuelling | 1942-12 | TOA MARU at Shortland refuelled heavy cruisers SUZUYA and MAYA on 3 Dec 1942 (1000 to 1318) | `combinedfleet`, entry 3 December 1942 |

#### `buin`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14348 | role | 1942-11 | Early November 1942: a convoy of 12 heavy transports was assembled in the Buin-Faisi area and loaded with about 10,000 replacement troops of the Hiroshima Division, 3,500 SNLF troops, heavy field artillery and other supplies | `ussbs-campaigns`, Chapter 7 |

#### `lae`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14325 | cut or hit | 1942-03 | At the 10 Mar 1942 carrier raid, cargo YOKOHAMA MARU (6,143 tons) sunk at 7-01S 147-07E (Lae-Salamaua) | `janac-1947`, merchant losses, chronological list |
| 14337 | role | 1942-03 | Army-Navy invasion force unloaded troops and cargo at Lae-Salamaua 8 Mar 1942 (KINRYU MARU) | `combinedfleet`, entry 8 March 1942 |
| 14326 | cut or hit | 1942-04 | Cargo TAIJUN MARU (1,274 tons) sunk by US Army aircraft 11 Apr 1942 at 6-49S 147-02E (Lae) | `janac-1947`, merchant losses, chronological list |
| 14344 | cut or hit | 1942-09 to 1943-01 | Matsuyama: after September 1942 transport use to eastern New Guinea was suspended because slow transports were exposed to air attack; submarine attacks south of New Britain were not an important factor | `ussbs-interrogations`, Interrogation of 31 Oct 1945 |
| 14342 | rate | 1942-12 | December 1942: 18th Army formed a coastal transport unit of ten barges manned by engineers (21st Independent Mixed Brigade) to carry stores from Rabaul toward the Lae sector, via the Dampier Strait | `bullard-jpn-army-ops-south-pacific`, page 209 of the PDF read, by its page breaks |

### Japanese side: Kwajalein and Wake

#### `kwajalein`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14410 | role | 1941-12 to 1942-12 | Kwajalein was the headquarters of the 6th Base Force and of the 6th (Submarine) Fleet; Roi in Kwajalein Atoll was the base of the 22nd Air Flotilla; the Pearl Harbor submarine screen came from the 6th submarine fleet base at Kwajalein. | `spennemann-to-hell-and-back`, Section 'The Japanese Attack Wake Atoll' and command table |
| 14400 | port | 1942-01 to 1944-01 | At Roi, Kwajalein Atoll, the Japanese operated a large pier facility with a hammerhead crane able to lift about 40 tons. | `spennemann-to-hell-and-back`, Section 'Supplies' |
| 14411 | cut or hit | 1942-02 | 1-2 February 1942: Task Forces 8 and 17 (Enterprise, Yorktown, with the fleet oilers Platte and Sabine) struck Wotje, Maloelap, Kwajalein, Roi, Jaluit, Makin, Taroa, Lae and Gugegive. | `carter-bbbo`, Carter ch. III |

#### `wake`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14399 | fed from | 1942-01 to 1944-01 | Bulk of the supplies for Wake arrived via Kwajalein Atoll, headquarters of the 6th Base Force; at Roi the Japanese ran a large pier facility; a hammerhead crane could lift about 40 tons; smaller boats carried goods from Kwajalein to Wake. | `spennemann-to-hell-and-back`, Section 'Supplies' (Wake as a Japanese base) |
| 14401 | fed from | 1942-01 to 1943-07 | Japanese garrisons on the Marshall atolls were supplied directly from Japan once every six months with major food supplies; small supplies at least monthly from Kwajalein by motorlaunch. (low) | `spennemann-to-hell-and-back`, Section 'Supplies' |
| 14402 | stock | 1942-01 to 1943-07 | All Japanese garrisons had sufficient food stores to last six months or more; but the Wake supply records were destroyed so no detailed account of what had been supplied to Wake is held. (low) | `spennemann-to-hell-and-back`, Section 'Supplies' |
| 14403 | role | 1942-01 to 1944-01 | There was a daily ferry service by transport plane of mail and personnel between Wake and Kwajalein. | `spennemann-to-hell-and-back`, Section 'Supplies' |
| 14409 | fed from | 1942-01 to 1943-12 | Wake's Japanese naval garrison unit was directly subordinate to the 6th Base Force on Kwajalein, which itself was subordinate to the 4th Fleet at Truk. (low) | `spennemann-to-hell-and-back`, Section 'The Japanese bases in the Marshalls' |
| 14404 | cut or hit | 1942-02 | Carrier raid of 24 February 1942 (Halsey, Enterprise): a dredge in the Wake lagoon was hit by naval gunfire from Northampton and Salt Lake City; the first carrier strike was largely for exercise and a show of force. | `spennemann-to-hell-and-back`, Section on the Japanese base |
| 14405 | cut or hit | 1942-02 to 1943-12 | After Wake fell on 23 Dec 1941, raids were intermittent and harassing: 'Wake was always hardest hit when American operations were proceeding in adjacent areas'; the mainstay of early raids were AAF B-24s from Midway. | `spennemann-to-hell-and-back`, Section 'U.S. Strategy regarding Wake' (USSBS quoted by Spennemann) |
| 14408 | role | 1942-08 to 1942-09 | Wake's Japanese fortification began in earnest in August 1942; night soil was shipped in from Japan; all but 98 civilian workers were moved to Japan on Tachibana Maru on 30 September 1942. | `spennemann-to-hell-and-back`, Section on prisoners and civilians |
| 14406 | cut or hit | 1943-01 | Late January 1943: U.S. planes saw two small cargo ships south of the southern channel entrance, a small boat in the harbour and two merchant vessels south of the channel entrance of Wake (merchant shipping called at Wake in 1943). | `spennemann-to-hell-and-back`, Section 'Supplies' |
| 14407 | cut or hit | 1944-01 to 1945-06 | US submarine offensive in the Marshall waters stepped up in August 1943; after January 1944 supplies could no longer be shipped to garrisons; the last supply vessel to reach Wake, Akagi Maru, arrived January 1944; thereafter submarines at monthly or six-week intervals. | `spennemann-to-hell-and-back`, Section 'Supplies' |

### Allied side, Southwest Pacific: the Australian ports and Port Moresby

#### `sydney`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14376 | cut or hit | 1942-06-04 to 1942-06-08 | Japanese submarine attacks on merchant ships off eastern Australia began in May 1942; on 4 June the Naval Board suspended merchant sailings between Adelaide and Brisbane (except Adelaide-Melbourne and Melbourne-Tasmania) pending convoys, which began on 8 June | `awm-gill-v2`, ch. 3, p. 77 |

#### `brisbane`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14359 | port | 1942 | Brisbane was 1,100 miles from New Guinea; Melbourne, Sydney and Brisbane could handle considerable traffic in large ocean-going ships. | `leighton-coakley-gls-1940-43`, Ch. XII, p.~391 |

#### `cairns`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14362 | role | 1942-01 | A single railroad wound around the Australian coast from Perth to Cairns, with rail lines of five different gauges needing frequent transshipment; manual transfer averaged about 24 hours at each transshipment point. | `leighton-coakley-gls-1940-43`, Ch. VII, p.168 |

#### `townsville`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14358 | role | 1942 | Townsville, 900 miles north of Brisbane with far less ample facilities, had to be developed as the principal transshipment point from Australia to New Guinea. | `leighton-coakley-gls-1940-43`, Ch. XII (Pacific Theaters logistics, "Turning Point in the Pacific"), p.~391 |
| 14361 | fed from | 1942-03 to 1942-04 | MacArthur concentrated on defence of Port Moresby; base development was focused on northeastern Australia, shortening the line from southeastern ports and making Townsville the focal point in the north. | `leighton-coakley-gls-1940-43`, Ch. VII (Australia), p.~173 |
| 14367 | cut or hit | 1942-05 | Coral Sea: Shokaku and Zuikaku had been detailed to press on to Townsville and destroy the assemblage of Allied shipping and aircraft there (plan, prevented). | `awm-mccarthy-v5-ch2`, Ch. 2, p.82 |
| 14370 | fed from | 1942-06 | On 6 June 1942 Macdhui joined a convoy with aviation fuel; at Townsville embarked 154 soldiers of 39th Battalion and left for Moresby, arriving 15 June. (low) | `pacificwrecks`, Wartime History |
| 14373 | fed from | 1942-06 to 1942-12 | Convoy designations of 1942 included TN, Townsville to New Guinea, and NT, New Guinea to Townsville; BV and VB ran Brisbane to Townsville and back | `awm-gill-v2`, ch. 3, footnote on p. 77 |

#### `port-moresby`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14360 | port | 1942 | In 1942 ocean-going vessels could not go along New Guinea north coast beyond Milne Bay; cargo was transshipped at Port Moresby or Milne Bay to trawlers, schooners, luggers and ketches largely procured in Australia. | `leighton-coakley-gls-1940-43`, Ch. XII (small ships section) |
| 14369 | fed from | 1942-01 to 1942-06 | Macdhui operated Sydney to northern Australia carrying supplies and personnel to Port Moresby; June 1942 trip with aviation fuel via Townsville convoy. (low) | `pacificwrecks`, Wartime History |
| 14372 | rate | 1942-01 to 1942-12 | During 1942, 41 convoys of 167 ships in all ran between the Australian mainland and New Guinea (Gill's footnote count of the 252 convoys of 1,672 ships of the year) | `awm-gill-v2`, ch. 3, footnote on p. 77 |
| 14365 | stock | 1942-02 | Australian chiefs of staff, late Feb 1942: garrison well provisioned (no figures). | `awm-mccarthy-v5-ch2`, Ch. 2, p.~65 |
| 14366 | cut or hit | 1942-03 | March 1942 brought seventeen more heavy air raids on Port Moresby. | `awm-mccarthy-v5-ch2`, Ch. 2, p.67 |
| 14357 | fed from | 1942-04 | Moresby supply line ran from Australia across the Gulf of Papua, exposed to enemy action; port facilities inadequate; Townsville-Cloncurry air concentration area was 700 miles away (April 1942). | `milner-victory-papua`, Ch. II, p.26 |
| 14374 | cut or hit | 1942-05-02 | 2 May 1942: Port Moresby had its 38th Japanese air raid (the same day as the air attacks on Tulagi before its occupation) | `awm-gill-v2`, ch. 2 "Japan's First Check - Coral Sea", p. 42 |
| 14371 | fed from | 1942-05-18 to 1942-05-31 | Convoy ZK.8 of four Dutch ships (Bantam, Bontekoe, Van Heemskerk, Van Heutsz) left Sydney on 18 May 1942 escorted by Tromp and Arunta, carrying 4,735 troops of the Australian 14th Brigade to reinforce Port Moresby, and arrived without incident at the end of the month | `awm-gill-v2`, ch. 3 "Australia's Coast Raided", p. 63 |
| 14375 | cut or hit | 1942-06-17 to 1942-06-18 | 17 and 18 June 1942: in Port Moresby's 61st raid (18 bombers, 9 fighters) the Australian ship Macdhui (4,561 tons) was hit and gutted amidships; in a raid of 18 bombers the next day she was hit again and burned | `awm-gill-v2`, ch. 3, p. 88 |
| 14351 | port | 1942-08 | Existing Port Moresby facilities (Aug 1942) allowed only one ship to be unloaded at a time, very slowly, with two or three more waiting in the roads exposed to enemy attack. | `milner-victory-papua`, Ch. VI, p.75 (Plans to make Port Moresby a large supply area) |
| 14352 | port | 1942-08 | Port Moresby harbour described as shallow and suitable only for light traffic (Aug 1942). | `milner-victory-papua`, Ch. VI, p.75 |
| 14353 | role | 1942-08 | Planned Tatana Island port in Fairfax Harbor would unload several ocean-going ships at once, joined to the mainland by a half-mile causeway; to begin when engineers and equipment became available (a plan, not capacity in place). | `milner-victory-papua`, Ch. VI, p.75 |
| 14354 | role | 1942-08 | US Advanced Base in New Guinea established 11 Aug 1942 at Port Moresby by USASOS to run the port and other New Guinea ports and supply American troops. | `milner-victory-papua`, Ch. VI, p.75 |
| 14355 | cut or hit | 1942-08 to 1942-10 | Shipping shortage in Aug 1942: emergency troop movements to Port Moresby and Milne Bay delayed movement of units and supplies to northern Queensland bases. | `milner-victory-papua`, Ch. VI |
| 14356 | fed from | 1942-08 | Seven transpacific ships were requisitioned to move 7th Australian Division to Milne Bay and Port Moresby (Aug 1942). | `milner-victory-papua`, Ch. VI |
| 14364 | fed from | 1942-08 | MacArthur to Marshall 2 Aug 1942: plans hampered by critical shortage of transportation, especially sea transport, and lack of naval convoy ships to protect supply routes. | `awm-mccarthy-v5-ch4`, Ch. 4, p.~118 |
| 14363 | port | 1942-09 | Rowell (20 Sept 1942): troops at Port Moresby of poor fighting quality were employed entirely on unloading ships. | `awm-mccarthy-v5-ch7`, Ch. 7, p.243 (letter quoted) |

#### `milne-bay`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14368 | cut or hit | 1942-09 | MV Anshun was shelled and sunk by Japanese naval gunfire at Gili Gili wharf while unloading on night of 6 Sept 1942 (later salvaged). Key milne-bay is made up. | `awm-mccarthy-v5`, Ch. 5, photo caption p.~178 |

### American side, Central Pacific: Pearl Harbor and Midway

#### `pearl-harbor`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14396 | role | 1941-12 to 1942-12 | Before the war required advanced bases Pearl Harbor was the storage and issue point for most of the stores, fuel and equipment used in the Pacific; advanced supplies had to be stock-piled in such quantities as to provide for unforeseen demands. | `building-bases-v2-ch22`, Ch. 22, Storage Facilities |
| 14398 | fed from | 1942-01 | CINCPAC (Jan 1942, to ABDA/Cominch): diversion of tankers to supply Australia and New Zealand will jeopardize supply of Pearl Harbor. | `nimitz-graybook-v1`, Graybook vol 1, 21 Jan 1942 entry (message ~182210 reply) |
| 14397 | fed from | 1942-04 | CINCPAC Graybook, 20 April 1942: Cominch wants tankers speeded up on the Hawaii-West Coast run; this is being done to some extent. | `nimitz-graybook-v1`, Graybook vol 1, entry April 20 |
| 14394 | fuel storage | 1942-09 to 1943-07 | The first underground vault was usable on 28 September 1942 and all tanks were ready to receive oil by 20 July 1943; work began December 1940. So at Midway (May-June 1942) the underground tank farm held no oil. | `building-bases-v2-ch22`, Ch. 22, Pearl Harbor fuel storage |
| 14395 | pump rate | 1942-09 to 1945-08 | Pearl Harbor fuel-oil pier: 54 by 1,300 feet, berthing four ships, issuing oil at 20,000 barrels per hour. | `building-bases-v2-ch22`, Ch. 22, Pearl Harbor fuel storage |
| 14393 | fuel storage | 1943-07 | Pearl Harbor underground fuel storage as completed: 5,400,000 barrels fuel oil and 600,000 barrels diesel oil, 6,000,000 barrels in twenty vaults. | `building-bases-v2-ch22`, Ch. 22, Pearl Harbor fuel storage |

#### `midway`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14391 | fed from | 1941-12 | Midway reinforcements by sea from Pearl: Batteries A and C, 4th Defense Battalion, with 'miscellaneous supplies of all types' left Pearl on 19 Dec 1941 in USS Wright and arrived Christmas Eve; USS Tangier arrived 26 Dec with aviation supplies and artillery matériel. | `heinl-marines-midway`, Ch. 3, December 1941 |
| 14378 | stock | 1942-05 | On 31 May 1942 daily aviation gasoline consumption of planes based on Eastern Island was 65,000 gallons, with the aircraft listed in the source (Army 4 B-26 and 17 B-17; Navy 16 PBY-5A and 6 TBF; Marines 19 SBD-2, 17 SB2U-3, 21 F2A-3, 7 F4F-3). | `heinl-marines-midway`, Footnote 31, ch. 3 |
| 14380 | stock | 1942-05 to 1942-06 | CINCPAC staff summary for 2 June 1942: almost 400,000 gallons accidentally destroyed the previous week; CINCPAC's ability to keep enough avgas at Midway caused concern; excessive use of gasoline at Midway. | `nimitz-graybook-v1`, Graybook vol 1, entry 'June 2.' |
| 14381 | stock | 1942-05 | CINCPAC staff entry for 23 May 1942: an error placing demolition charges destroyed considerable avgas at Midway; CINCPAC would send replacement gasoline immediately, in drums. | `nimitz-graybook-v1`, Graybook vol 1, entry 'May 23.' |
| 14385 | fuelling | 1942-05 | Ships were fuelled from Midway station tanks in May 1942: USS Nashville took 3,773 barrels of fuel oil and 1,800 gallons avgas on 8 May. | `midway1942-nas-midway-diary`, War diary entry 8 May 1942 |
| 14386 | fuelling | 1942-05 | USS St. Louis and USS Case, which arrived 25 May 1942, were fuelled to 95% capacity at Midway the same day (arrived 0800, left 1730). | `midway1942-nas-midway-diary`, War diary entry 25 May 1942 |
| 14387 | fed from | 1942-05 | USS Kitty Hawk, which arrived at Pearl on 17 May 1942, was reloaded there with 28 planes, Marine air groups, eight 3-inch AA guns with ammunition, left 23 May and reached Midway 26 May, delivering and also giving the station fuel oil, clearing on 29 May. | `carter-bbbo`, Carter ch. III |
| 14388 | port | 1942-05 | Few ships in May 1942 had the crane capacity to unload planes and heavy cargo at the dock at Midway. | `carter-bbbo`, Carter ch. III |
| 14389 | fed from | 1942-05 to 1942-06 | Distances: Pearl to Midway 1149 miles; Midway to Saipan 2300 miles; Midway to Dutch Harbor 1653. | `nimitz-graybook-v1`, Graybook, Operation Plan 29-42 estimate of the situation, para 2(d) |
| 14390 | port | 1942-05 to 1942-06 | CINCPAC estimate of the situation (late May 1942): ships up to the size of CAs can enter and berth at Midway, but must exercise extreme caution. | `nimitz-graybook-v1`, Graybook, Operation Plan 29-42, 'Character of the Theater' |
| 14392 | role | 1942-05 | Admiral Nimitz told the Midway commander to submit direct to CinCPac a detailed list of all supplies and equipment required; list submitted 7 May and, within less than a week, Marines and matériel were being embarked in the Hawaiian area. | `heinl-marines-midway`, Ch. 3, early May 1942 |
| 14379 | stock | 1942-06 | CINCPAC staff summary for 3 June 1942: Midway avgas on hand 543,000 gallons, enough for 6-7 days, further receipt depending on events (a 22 May explosion had destroyed almost 400,000 gallons). | `nimitz-graybook-v1`, Graybook vol 1, entry 'June 3. (Cont'd)' |
| 14382 | stock | 1942-06 | After the 4 June 1942 air attack the distribution line to one of the two fuel tanks was destroyed, isolating about 70,000 gallons of gasoline; only two gasoline trucks were available and most fuelling was from drums with hand pumps. | `bates-midway-1948`, Bates, operations at Midway (American), p. ~115-116 |
| 14383 | rate | 1942-06 | Between 3 and 4 June night, about 45,000 gallons of gasoline were hand-pumped from 55-gallon drums into aircraft at Midway. | `bates-midway-1948`, Bates, Operations at Midway, night 4-5 June |
| 14384 | rate | 1942-06 | On 5 June and the night of 5-6 June between 70,000 and 75,000 gallons of gasoline were poured or pumped from 55-gallon drums servicing aircraft at Midway. | `bates-midway-1948`, Bates, Operations at Midway, 5-6 June |

### American side, South Pacific: the West Coast to Guadalcanal

#### `san-francisco`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14426 | role | 1942-01 to 1943-06 | Supply planning pipeline factor: about fifteen days in transit time in the United States and forty-five days for movement to the theater; a separate allowance of about 2.5 percent was made in 1942 for ship sinkings. | `leighton-coakley-gls-1940-43`, ch. XIII? (supply planning), p. 301 |
| 14427 | cut or hit | 1942-01 to 1942-12 | For planning in 1942 'a separate allowance of about 2.5 percent' was made for ship sinkings over and above the distribution factors (all theaters). | `leighton-coakley-gls-1940-43`, ch. XIII?, p. 301 |
| 14428 | rate | 1942-02 to 1942-12 | Troop convoy schedule of 9 Feb 1942: sailings to Australia and the Pacific ferry islands were to be spaced at intervals of thirty days; sailings to Hawaii six days. Troop convoys, not the cargo flow of Aug-Oct 1942. | `leighton-coakley-gls-1940-43`, ch. VII, p. 208 |
| 14424 | role | 1942-07 to 1942-12 | 15 July 1942 Joint Logistical Plan (Somervell-Horne): the Navy was to provide all gasoline and oil including aviation fuel for South Pacific bases; each service to requisition remainder from its own mainland source (Army: San Francisco port; Navy: Service Force Subordinate Command and Twelfth Naval District). | `leighton-coakley-gls-1940-43`, ch. XVII, Joint Logistical Plan of 15 July 1942 |
| 14425 | fed from | 1942-07 to 1942-12 | Army requisitions for South Pacific bases went to the San Francisco port of embarkation; Navy requisitions to the Commander Service Force Subordinate Command Pacific Fleet and the Commandant Twelfth Naval District; shipment 'in shipload lots by each service'. | `leighton-coakley-gls-1940-43`, ch. XVII, Joint Logistical Plan of 15 July 1942 |
| 14446 | rate | 1942-07 to 1942-08 | Nimitz to Ghormley, 9 July 1942: chartered tankers bringing 450,000 bbl to Noumea would be followed by others with about 225,000 bbl a month for the carrier task force; other requirements (aviation gasoline, Diesel, stores) would be supplied as Ghormley requested. | `carter-bbbo`, ch. IV, p. 24-25 |
| 14447 | rate | 1942-08 | 1 Aug 1942: after the Bishopsdale loss, Nimitz ordered the 2 large fast tankers then at San Pedro to proceed to Noumea with black oil for Ghormley to divert, in addition to the 200,000 bbl ordered delivered every 15 days; the tanker Sabine left San Pedro on 2 Aug and could not reach the Fijis before 2 weeks had elapsed. | `carter-bbbo`, ch. IV, p. 26 |
| 14449 | role | 1942-08 | On 15 Aug Nimitz ordered the tanker Guadalupe to sail with Task Force 17 (Hornet) on 16 Aug to reinforce the South Pacific; Platte and Kaskaskia had been emptied by Fletcher's groups in one week of normal cruising (14 Aug). | `carter-bbbo`, ch. IV, p. 32 |
| 14476 | role | 1942-08 to 1942-12 | The Bureau of Supplies and Accounts routed shipments for Noumea and Auckland 'too heavily upon the port of San Francisco' so that Twelfth Naval District facilities were 'hard put to keep up'; lack of cargo information hurt shipping estimates. | `ballantine-us-naval-logistics`, ch. (Advance base support; shipments to Noumea and Auckland), p. 66-68 |
| 14429 | rate | 1942-09 to 1942-11 | September-November 1942: 36,000 troops and 310,000 measurement tons of cargo were shipped from the United States to the South Pacific (against 8,300 troops and 157,000 tons to SWPA, 25,000 troops and 330,000 tons to Hawaii). | `leighton-coakley-gls-1940-43`, ch. XVIII, p. 397-398 (shipments, Sep-Nov 1942) |

#### `auckland`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14445 | role | 1942-07 to 1942-08 | Auckland, the best of the poor South Pacific bases because New Zealand could furnish food and some repair facilities, was too far from the scene of operations; Tongatabu was a submarine-protected anchorage behind reefs well beyond range of Japanese land-based planes. | `carter-bbbo`, ch. IV, p. 26 |
| 14448 | rate | 1942-08 | Nimitz told Ghormley on 12 Aug 1942 that the Cabrillo had left San Francisco on 3 Aug with 40 guns and 200,000 rounds of 20-mm ammunition for Auckland and should arrive about 23 August; the Vestal and Kitty Hawk would sail 15 Aug with other ammunition. | `carter-bbbo`, ch. IV, p. 30 |

#### `suva`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14440 | fuel storage | 1942-06 to 1942-10 | Fiji tank farm as planned: 50,000 bbl fuel oil, 20,000 bbl diesel, 10,000 bbl gasoline, 500 bbl lubricating oil; the tank-farm material did not arrive until late October 1942. Nandi airfield held two underground tanks for 98,000 gal of aviation gasoline. | `building-bases-v2`, ch. XXIV, Fiji section |
| 14441 | port | 1942-06 to 1942-12 | Suva: three large vessels could be accommodated at a timber wharf; excellent anchorage in the inner harbor. | `building-bases-v2`, ch. XXIV, Fiji section |

#### `tongatabu`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14438 | fuel storage | 1942-05 to 1942-12 | Planned Tongatabu fuel depot: twenty-two 10,000-barrel tanks for fuel and diesel oil and twenty-six 500-barrel tanks for aviation gasoline (basic plan; the Seabee detachment arrived 9 May 1942). | `building-bases-v2`, ch. XXIV, Tongatabu section |
| 14439 | role | 1942-11 to 1943-02 | Tongatabu tank farm was dismantled and moved to Wallis Island when the Seabees were ordered to stop building in November 1942. | `building-bases-v2`, ch. XXIV, Tongatabu section |

#### `noumea`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14377 | cut or hit | 1942-05-05 to 1942-05-07 | 5 May 1942: the American John Adams (7,180 tons) was torpedoed off New Caledonia and blew up the next day; on 7 May the Greek Chloe (4,641 tons) was sunk by a submarine's gunfire off New Caledonia; Gill judges them targets of opportunity of a reconnaissance boat | `awm-gill-v2`, ch. 3, p. 61 |
| 14412 | role | 1942-06 to 1942-11 | Noumea was developed as the main South Pacific fleet base, taking over the functions first planned for Auckland; South Pacific headquarters opened there on 8 Nov 1942. | `building-bases-v2`, ch. XXIV, Noumea section |
| 14413 | role | 1942-06 to 1942-08 | Until early summer 1942 Auckland, not Noumea, was preferred as the site of the principal South Pacific base facilities; development of Noumea 'proceeded slowly at first' while effort went into Auckland, 1,000 miles south. | `building-bases-v2`, ch. XXIV, Noumea section |
| 14415 | port | 1942-06 to 1942-09 | A shortage of stevedore personnel held up the unloading of ships in the summer months of 1942; all available Navy personnel were used and arriving construction battalions did their own unloading. | `building-bases-v2`, ch. XXIV, Noumea section |
| 14444 | role | 1942-07 to 1942-08 | Carter's July 1942 assessment: of the South Pacific bases (Auckland, Fiji, Tongatabu, Noumea, Efate) none was more than a small airfield and a protected anchorage; Noumea 'seemed the most suitable' with a large anchorage 'quite well protected against submarine attack by islands and mine fields'. | `carter-bbbo`, ch. IV, p. 26 |
| 14414 | port | 1942-08 to 1942-12 | Le Grand Quai at Noumea: 1,400 ft long, 20 to 26 ft of water, 68,500 sq ft of transshipment shed, no crane; Nickel Dock took one large vessel (3 x 7-ton cranes, little storage). 'Servicing facilities were meager.' | `building-bases-v2`, ch. XXIV, Noumea section |
| 14416 | port | 1942-08 to 1942-12 | At Noumea the Army had one three-berth dock, the Grand Quay, and occasionally the berth at the Nickel Dock when commercial use allowed; cargo-handling gear was rudimentary and tugs and barges few. Espiritu Santo had a good harbor but no piers to berth ocean-going vessels. | `bykofsky-larson-tc-overseas`, ch. XVIII (The South and Central Pacific), p. 495 |
| 14418 | role | 1942-08 to 1942-12 | Noumea was 'a port with an excellent harbor but very limited docking facilities' that had to serve as the main forward base; Auckland was 1,500 miles from Guadalcanal and Noumea 900 miles. | `leighton-coakley-gls-1940-43`, ch. XVIII, p. 397 (Pacific distances) |
| 14421 | stock | 1942-08 | In August 1942 General Patch (New Caledonia) furnished on short notice over 20,000 tons of supplies for the Marines going to Guadalcanal and had to supply the new bases at Efate and Espiritu Santo; Harmon wrote 'Everyone robs Poppy' on 20 Aug 1942. | `leighton-coakley-gls-1940-43`, ch. XVIII, 'The General Depot at Noumea', p. 406-407 |
| 14436 | fuelling | 1942-08 | At Noumea on 30 and 31 Aug 1942 the Sabine took aboard 32,661 barrels from the chartered tanker J.W. Van Dyke and 52,909 barrels from the chartered Pacific Sun. | `carter-bbbo`, ch. IV, p. 31 |
| 14461 | fed from | 1942-08 | Advanced supply depots were established at Noumea and Espiritu Santo on 20 Aug 1942 by order of Admiral Turner. | `miller-guadalcanal`, ch. IV, Logistics paragraph |
| 14419 | port | 1942-09 | Noumea harbor had only a few berths, shared with the commercial exports of the French Nickel Company; neither service had appreciably expanded them before the 'avalanche of shipping' of August and September 1942. Army port troops: only a provisional port company of 73 men by mid-September. | `leighton-coakley-gls-1940-43`, ch. XVIII, p. 398 |
| 14420 | cut or hit | 1942-09 to 1942-12 | On 23 Sep 1942 General Patch reported 86 ships at anchor in Noumea; a War Department concern was that such a concentration 'presented an ideal target for the Japanese'. (The figure 86 is already on file; the target remark is new. No source says the Japanese attacked the harbor.) | `leighton-coakley-gls-1940-43`, ch. XVIII, p. 400 |
| 14422 | role | 1942-09 to 1942-12 | Even by autumn 1942 in the South Pacific there was no central reserve of supplies and no provision for prompt replacement of sudden heavy losses; requisitioning on the United States continued base by base. | `leighton-coakley-gls-1940-43`, ch. XVIII, p. 407 |
| 14457 | role | 1942-09 | 23 Sep 1942 COMSOPAC to VICEOPNAV: WHITE POPPY 'has of necessity become extremely active and congested port'; unloading, handling, storing and housing facilities 'totally inadequate or non existent'; requested 20 self-propelled barges each of 50, 100 and 250 ton. | `nimitz-graybook-v2`, Graybook vol. 2, COMSOPAC 23 1251, p. 926 |
| 14450 | fuelling | 1942-10 | Oct 1942: Rear Admiral Crosse (Base Force Subordinate Command, San Francisco) was asked to equip all chartered tankers sent to the South Pacific with enough hose to supply two ships at once at maximum rate; the naval oilers had been 'going at a killing pace'. Done on most commercial tankers. | `carter-bbbo`, ch. V, p. 46 |
| 14475 | role | 1942-10 | Dec 1942-Jan 1943: congestion at Noumea 'continued to increase through December and January until at one time as many as one hundred vessels were awaiting discharge'; Ghormley (17 Oct 1942) recommended a base of 20,000 men at Noumea. | `ballantine-us-naval-logistics`, ch. on advance bases (p. 112) |
| 14430 | role | 1942-11 | On 20 Nov 1942 Halsey asked the Army to take over all discharge and loading at Noumea; the Army's SOS Transportation Division moved to Noumea (late 1942) and by end of 1942 manifests were arriving more regularly. | `bykofsky-larson-tc-overseas`, ch. XVIII, p. 499 |
| 14458 | stock | 1942-11 | 20 Nov 1942 (CINCPAC): 'Five or six fast Fleet tankers have been kept available in Sopac area for task forces'; shore tanks were under construction at WHITE POPPY, ROSES (Efate) and BUTTON (Espiritu Santo) and would give the principal relief; the old tanker J.C. Donnell was being used as mobile black-oil storage in the area. | `nimitz-graybook-v2`, Graybook vol. 2, CINCPAC 20 0925 to Vice Opnav, p. 992 |

#### `efate`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14442 | port | 1942-03 to 1942-08 | At Efate Vila harbor had a few small piers in water six feet deep at low tide, and Havannah Harbor had no facilities before the US forces came. | `building-bases-v2`, ch. XXIV, Efate section |
| 14443 | role | 1942-07 to 1942-08 | Efate had two harbors: Vila, too small for more than one or two combatant ships, and Havannah, large enough but without protection against submarines (July 1942 assessment). | `carter-bbbo`, ch. IV, p. 26 |
| 14437 | stock | 1942-08 | 1 Aug 1942: apart from fuel oil in tankers, South Pacific shore stocks were small quantities of black oil for patrol craft and 'considerable tank and barreled gasoline at Tongatabu and Efate and a smaller amount in New Caledonia'. | `carter-bbbo`, ch. IV, p. 25-26 |

#### `espiritu-santo`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14433 | stock | 1942-07 to 1942-08 | The first Espiritu Santo field (Turtle Bay) opened 28 July 1942; aircraft were fueled from drums (first B-17 raid on Guadalcanal 30 July). | `building-bases-v2`, ch. XXIV, Espiritu Santo section |
| 14434 | role | 1942-07 to 1942-12 | Espiritu Santo as built: 630 miles southeast of Guadalcanal, 400 miles closer to Japanese positions than Efate; designed for heavy bombers, fighters, two carrier groups, accumulation of ammunition, provisions, stores. 7th Seabee Battalion arrived 11 Aug 1942. | `building-bases-v2`, ch. XXIV, Espiritu Santo section |
| 14477 | fed from | 1942-07 to 1943-03 | Cub No. 1 began its movement to Espiritu Santo shortly after 1 July 1942; Lion No. 1 was detained on the West Coast almost six months and moved to Espiritu Santo early in 1943. | `ballantine-us-naval-logistics`, ch. (advance base units), p. 111 |
| 14417 | port | 1942-08 to 1942-12 | Espiritu Santo had an excellent natural harbor but no piers capable of berthing ocean-going vessels; cargo had to be lightered to barge piers or the beach (this and virtually all other South Pacific islands). | `bykofsky-larson-tc-overseas`, ch. XVIII, p. 495 |
| 14435 | fuelling | 1942-08 | The tanker Sabine, diverted to Espiritu Santo, arrived from Suva on 22 Aug 1942, stayed 2 days, helped fuel TF 61, returned 26 Aug and fueled 11 ships at Espiritu before sailing for Noumea on the 28th. | `carter-bbbo`, ch. IV, p. 31 |
| 14432 | port | 1942-09 to 1943-03 | At Espiritu Santo vessels lay in Segond Channel over three months before they could be touched; no unloading cranes (ships' booms handled nothing beyond seven tons while some radar units exceeded twenty tons per package). Pallikula (Pallii Kula) Bay had cranes able to handle P-39/P-40/P-38 fighters with the dock 200 yards from the airfield. | `craven-cate-v4`, ch. 3 (base facilities) |
| 14452 | cut or hit | 1942-09 | CINCPAC summary: the Saratoga was torpedoed 30 Aug and went to Pearl; on the 16th (GCT) the Wasp was hit and sunk by submarine torpedoes and the North Carolina received one torpedo hit and went to Pearl. (Wasp group was covering the 7th Marines convoy from Espiritu Santo.) | `nimitz-graybook-v2`, Graybook vol. 2, narrative summary (Sept 1942), p. ~893 |
| 14423 | cut or hit | 1942-10 | The transport President Coolidge, carrying an infantry combat team and a harbor defense unit, struck an American mine field off Espiritu Santo and sank (caption dates the beached ship October 1942); all equipment lost, personnel losses slight. Friendly mines, not enemy action. | `leighton-coakley-gls-1940-43`, ch. XVIII, p. 399-400 |
| 14451 | role | 1942-10 to 1942-11 | Fitch relieved McCain on 21 Sep 1942 flying his flag in the Curtiss at Espiritu Santo; Halsey (19 Oct) agreed Santo was the best base and approved bringing in a Lion; the repair ship Rigel gave 'meager service of fuel, dry provisions, and occasionally some fresh and frozen provisions'. | `carter-bbbo`, ch. V, p. 40 |
| 14431 | port | 1942-11 to 1942-12 | Immobilization of shipping 'of a lesser magnitude' was developing at Espiritu Santo in late 1942; the first Army port battalion (390th) reached Espiritu only in Feb 1943. | `bykofsky-larson-tc-overseas`, ch. XVIII, p. 499 |
| 14459 | role | 1942-11 | 20 Nov 1942 CINCPAC: the Esso Richmond incident was 'regrettable but was primarily due to lack of any shore storage in forward area'; the old tanker J.C. Donnell was being used as mobile storage for black oil in the Sopac area. | `nimitz-graybook-v2`, Graybook vol. 2, p. 992 |

#### `tulagi`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14454 | cut or hit | 1942-09 | 10-11 Sep 1942: a hostile destroyer shelled Tulagi harbor twice (0015 and 0130 on the 10th local) and destroyed a previously damaged YP. | `nimitz-graybook-v2`, Graybook vol. 2, COMSOPAC dispatch 11 1320 |
| 14466 | stock | 1942-09 | 14 Sep 1942: Vandegrift ordered the cargo ship Bellatrix to unload her gasoline at Tulagi because the Lunga area was under attack. | `miller-guadalcanal`, ch. V |
| 14469 | port | 1942-09 to 1942-10 | Ships unloading in the greater security of Tulagi Harbor could unload continuously, whereas ships off Guadalcanal unloaded only during daylight (order of 29 Sep 1942). | `miller-guadalcanal`, ch. V |

#### `henderson-field`

| Row | Kind | Date | What the source states | Source, where |
| --- | --- | --- | --- | --- |
| 14460 | stock | 1942-08 | The 1st Marine Division expected its 60 days' supplies to prove ample and did not provide for immediate resupply; advanced supply depots were set up at Noumea and Espiritu Santo on 20 Aug 1942 by order of Admiral Turner, outside divisional control. | `miller-guadalcanal`, ch. IV, Logistics paragraph |
| 14462 | rate | 1942-08 | By 20 Aug 1942 six destroyer-transports had been assembled to run between Espiritu Santo and Guadalcanal; Task Force 62 was charged on 17 Aug with the line of communications. | `miller-guadalcanal`, ch. IV |
| 14463 | role | 1942-08 to 1942-10 | 20 Aug 1942 onward: Marine Air Group 25 C-47s flew daily Espiritu Santo to Guadalcanal, bringing in 3,000-pound loads; the Espiritu Santo-Henderson leg was 560 nautical miles. | `miller-guadalcanal`, ch. IV |
| 14464 | cut or hit | 1942-08 | 30 Aug 1942: the destroyer-transport Colhoun was sunk in the afternoon by eighteen enemy bombers while carrying only 17 tons of stores; Little and Gregory sunk night of 4-5 Sep by a superior enemy force. | `miller-guadalcanal`, ch. IV, 'Logistics' end paragraph |
| 14472 | cut or hit | 1942-08 to 1942-09 | Japanese submarines deployed southward in August and September 1942 to try to cut the American supply lines leading to Guadalcanal. | `miller-guadalcanal`, ch. VI, Air and Naval Preparations |
| 14453 | cut or hit | 1942-09 | 5 Sep 1942 (local): five Japanese destroyers shelled Guadalcanal and sank the destroyer-transports Little and Gregory; casualties believed heavy (12 officers and 226 men rescued per later message). | `nimitz-graybook-v2`, Graybook vol. 2, COMSOPAC dispatch 05 1156 |
| 14455 | cut or hit | 1942-09 | 30 Sep 1942 (local): the cargo ship (AK) Alhena was torpedoed while returning from Guadalcanal near 10-47 S, 161-16 E; shaft sprung; the tug Navajo went to tow her. Graybook summary: 'Our AKs are continually exposed to this submarine menace.' | `nimitz-graybook-v2`, Graybook vol. 2, COMSOPAC dispatch 30 Sep |
| 14456 | stock | 1942-09 | 9 Sep 1942: Guadalcanal needed the cargo ship Santa Ana quickly: aboard were 700 bundles of Marston mat, 1,200 rolls of Sommerfeld mat, 3,000 drums of avgas, tankage and a power shovel; Cub One estimated 4 days to unload working 24 hours; nine planes had crashed the day before and the field was 'practically inoperative'. (low) | `nimitz-graybook-v2`, Graybook vol. 2, COMAIRSOPAC dispatch 09 1215 |
| 14465 | cut or hit | 1942-09 | Night of 4-5 Sep 1942: a superior force of enemy warships sank the destroyer-transports Little and Gregory in Sealark Channel; the cargo ships Bellatrix and Fuller entering on 7 Sep were driven off by the threat of attack. | `miller-guadalcanal`, ch. V, 'Air Power and Supply' |
| 14467 | stock | 1942-09 | 18 Sep 1942: the McCawley, Bellatrix, MacFarland and Tracy put 3,823 drums of fuel ashore at Kukum-Lunga, with 8,825 tons of B, C, D rations still aboard; 930 tons of C and D rations and 82 tons of B rations were landed. | `miller-guadalcanal`, ch. V, landing of the 7th Marines |
| 14468 | rate | 1942-09 to 1942-10 | 29 Sep 1942: Turner ordered Amphibious Force ships to load no more than 3,000 tons for Guadalcanal or Tulagi, 'a quantity which could be unloaded in twenty-four hours'; off Guadalcanal ships unloaded only by daylight, at Tulagi continuously. | `miller-guadalcanal`, ch. V |
| 14470 | role | 1942-09 | End of September 1942: 19,251 men on Guadalcanal and 3,260 on Tulagi; full rations restored to all combat troops by about 18 Sep. | `miller-guadalcanal`, ch. V |
| 14471 | stock | 1942-10 | Mid-October 1942: Henderson aviation gasoline had fallen to a critically low level; on 14 Oct only enough remained for one strike; on 15 Oct C-47s began flying gasoline from Espiritu Santo, twelve drums each; the MacFarland ran in a load from Espiritu Santo and was damaged on 16 Oct. | `miller-guadalcanal`, ch. VI, Air and Naval preparations |
| 14473 | cut or hit | 1942-10 | 16 Oct 1942: the seaplane tender MacFarland carrying gasoline from Espiritu Santo was caught by Japanese planes in Sealark Channel and seriously damaged, then salvaged in an inlet on Florida Island. | `miller-guadalcanal`, ch. VI |
| 14474 | cut or hit | 1942-11 | 28 Nov 1942: the cargo ship Alchiba was torpedoed by a Japanese submarine at Guadalcanal; Miller calls it a late example of attacks continuing even after the danger from enemy warships lessened. | `miller-guadalcanal`, ch. VII |

## What the record supports

### Firmly

Each of these is a figure or a plain statement in a primary record or an official history, read in the saved text by the lead.

**Midway's aviation gasoline before the battle**

- 543,000 gallons were on hand on 3 June, "satisfactory for the next 6-7 days" (Graybook, 14379).
- Use was 65,000 gallons a day on 31 May (Heinl, 14378).
- The 22 May explosion destroyed "almost 400,000 gallons" (Graybook, 14380). That settles the reading of the war diary's garbled "375,00" (on file 6011) towards 375,000, not 37,500. Row 6011 is not changed; it is a matter for the architect.
- After 22 May, replacement gasoline was sent from Pearl "in drums" (14381).

**Midway was fed from Pearl Harbor by single ships, not by a line**

- Wright and Tangier came in December 1941.
- Kitty Hawk left Pearl on 23 May and arrived on 26 May. She gave the station fuel oil, but Carter states no quantity.
- Pearl to Midway is 1,149 miles (Graybook).
- Few ships could unload heavy cargo at Midway's dock (Carter).

**Pearl Harbor's underground fuel farm**

- The farm of 6,000,000 barrels was not usable until its first vault on 28 September 1942.
- Pearl's fuel in the Midway period was therefore in the surface tanks and the tankers, whose figure no source read gives.

**The South Pacific's fuel line as planned and as used**

- Chartered tankers were to bring 450,000 bbl to Noumea, and then about 225,000 bbl a month for the carrier force (Nimitz's plan of 9 July 1942, Carter).
- 200,000 bbl were to come every 15 days (on file).
- Use was 25,000 bbl a day by the combatants and 3,000 by the auxiliaries (on file).
- A study of 23 days' issues foresaw a shortage "soon after 14 September" (Carter).
- The two oilers with the carriers were emptied in one week of normal cruising (Carter, 14 August).
- Five or six fast fleet tankers were kept in the area for the task forces (Graybook, 20 November).

**Noumea's port, the bottleneck**

- **Berths:** one three-berth quay, the Grand Quay, with occasional use of the Nickel Dock (Bykofsky and Larson). *Building the Navy's Bases* describes the Grand Quai as 1,400 ft long with no crane.
- **Discharge:** about 24 ships a month could be discharged when properly spaced, and twice that number were sent (Craven and Cate, on file 115).
- **Labour:** an Army port company of 73 men in mid-September (Leighton and Coakley).
- **Holding:** ships were kept as long as 90 days.

**Espiritu Santo and Efate had anchorages and no piers**

- Cargo was lightered to barge piers or the beach (Bykofsky and Larson; Leighton and Coakley).
- At Espiritu ships waited in Segond Channel more than three months (Craven and Cate).
- Ships' booms there could handle no more than seven tons.

**The Espiritu Santo to Guadalcanal leg was cut and hit**

- Colhoun was sunk on 30 August by 18 bombers, carrying 17 t of stores.
- Little and Gregory were sunk by warships on the night of 4 to 5 September.
- Bellatrix and Fuller were driven off on 7 September.
- A YP was destroyed at Tulagi on 10 September.
- Alhena was torpedoed on 30 September. The Graybook says "Our AKs are continually exposed to this submarine menace".
- MacFarland was bombed carrying gasoline on 16 October.
- From 29 September, Turner limited a ship to 3,000 t, "a quantity which could be unloaded in twenty-four hours", to be unloaded by daylight only off Lunga, and at any hour at Tulagi.

**The West Coast to Noumea and Noumea to Espiritu Santo legs**

- No source read names a cargo ship or tanker sunk on these legs in July to October 1942, nor says the line was cut. This is silence, not a statement.
- The losses at sea in the area were warships: Saratoga on 30 August, and Wasp and North Carolina on 15 September.
- The two May 1942 sinkings off New Caledonia (John Adams and Chloe, Gill) were, in Gill's judgement, a reconnaissance boat's targets of opportunity.
- President Coolidge was lost on an American minefield off Espiritu Santo in October.
- In 1942 the Army's planning allowed about 2.5 percent for ship sinkings, in every theatre (Leighton and Coakley).

**The Japanese lines to Rabaul and Truk could be and were hit in 1942**

- **Submarines on the approaches to Rabaul and Cape St George:** JANAC credits them with Shoei Maru (12 May), Tenzan Maru (8 July), Meiyo Maru (8 August, carrying reinforcements for Guadalcanal), Katsuragi Maru (1 October), Naminoue Maru (7 October, an "Oki" convoy from Saeki), Sumiyoshi Maru (14 October) and Choko Maru (21 October).
- **Army aircraft in Rabaul harbour:** Komaki Maru (18 April, at pier No. 3 unloading bombs) and three ships on 27 and 30 December.
- **Submarines near Truk:** a tanker (1 March) and three cargo ships or transports (3 to 5 August).
- The USSBS puts ships sunk by aircraft in the area in the fall of 1942 at about 30,000 tons (low: the line is split in the extracted text).

**Port Moresby was fed across the Gulf of Papua from Townsville, and its port was small**

- **Feed:** Townsville was "the principal transshipment point from Australia to New Guinea" (Leighton and Coakley). The 1942 convoy series to New Guinea, TN and NT, ran from Townsville (Gill).
- **Port:** existing facilities "permitted only one ship to be unloaded at a time, and that very slowly", with two or three others waiting in the roads (Milner, August 1942).
- **Convoys:** in 1942, 41 convoys of 167 ships ran between the mainland and New Guinea (Gill, for the year).
- **Air raids:** the 38th raid fell on 2 May and the 61st on 17 June, when the Macdhui was hit; she was lost the next day.

**The east-coast route was stopped and then convoyed**

- After the submarine attacks of May and June 1942, the Naval Board suspended sailings between Adelaide and Brisbane on 4 June.
- Coastal convoys began on 8 June (Gill).

### Loosely

These rest on one secondary source, a website, a single entry, or a statement without a figure.

**The Japanese side**

- **Truk's feed:**
  - fed from Japan by provisions ships (Irako, 500 t of food, January 1942);
  - oilers from Kure (Kokuyo Maru to Rabaul, November 1942);
  - aircraft ferries from the Empire (combinedfleet.com; USSBS interrogations).
  - No rate.
- **Shortland's station oiler:** Toa Maru fuelled cruisers there on 6 September and 3 December, and was bombed with Fujisan Maru on 10 December (combinedfleet.com).
- **Rabaul's single transfers:**
  - Hoyo Maru gave 300 t of oil and Nankai Maru No. 2 gave aviation gasoline in tins, on 18 February 1942;
  - aircraft were fuelled from 200-litre drums (USSBS interrogation, low).
- **Army shipping** (Bullard's Senshi Sosho):
  - 2,100,000 tons conscripted at the start of the war, planned to fall to about 1,000,000 by July 1942;
  - 100,000 tons for the FS operation, of which 50,000 were judged right for the 17th Army;
  - shipping returning to Palau in August 1942.
  - These are shipping, not cargo.
- **Transports to eastern New Guinea:** suspended after September 1942 because slow transports were exposed to air attack. Submarine attacks south of New Britain "were not an important factor" (Matsuyama, USSBS interrogation 57).
- **Wake under the Japanese** (Spennemann; dated loosely or for the Marshalls in general):
  - supplied via Kwajalein, where Roi had a large pier and a 40-ton crane;
  - the atoll garrisons got major food supplies from Japan every six months and small lots monthly from Kwajalein;
  - garrisons held six months' food or more;
  - a daily transport plane to Kwajalein;
  - the American raids of 1942 were intermittent;
  - the last supply ship reached Wake in January 1944.

**The American side**

- **West Coast transit:**
  - Cabrillo left San Francisco on 3 August and was expected at Auckland "about 23 August";
  - Sabine could not reach the Fijis from San Pedro in under two weeks;
  - the planning factor was 45 days for movement to a theatre (Leighton and Coakley).
  - None states a transit time from the West Coast to Noumea.
- **Tons to the South Pacific:** 310,000 measurement tons and 36,000 troops from the United States in September to November 1942 (Leighton and Coakley). The total is for three months, with no monthly split.
- **Stores from New Caledonia:** Patch furnished over 20,000 tons of supplies to the Marines in August 1942 (Leighton and Coakley).
- **Fuel to Guadalcanal by air:**
  - C-47s brought 3,000-pound loads daily from 20 August;
  - from 15 October each C-47 carried twelve drums of gasoline (Miller).
- **Fuel to Guadalcanal by sea:** 3,823 drums of fuel came ashore on 18 September (Miller). The Santa Ana's 3,000 drums of 9 September are low: OCR, destination unsure.

### Gaps

Each is in `gaps.csv`.

- **Rabaul, Truk, Shortland, Lae and Kavieng:** no stock of stores or fuel in 1942 in tons or days; no tons or ships a month arriving; no unloading rate.
- **Port Moresby:**
  - no stock in tons or days (only "well provisioned", February 1942);
  - no tonnage or frequency of its convoys in April and May 1942;
  - no unloading rate.
- **Midway and Pearl Harbor:**
  - no figure for Midway's fuel oil, stores or ammunition, and no tonnage or tanker delivery in the May build-up;
  - no 1942 stock at Pearl Harbor;
  - no count of the Pacific Fleet's oilers in 1942.
- **Wake and Kwajalein:** no tonnage or frequency of Wake's supply, no 1942 ship lost on its run, and no Kwajalein stock.
- **Espiritu Santo and Efate:** no stock in August or September 1942, and no unloading rate in tons a day.
- **The West Coast:** no transit time to Noumea or Auckland, and no sailings a month to the South Pacific, for July to October 1942.
- **The documents most likely to fill these, not obtained:** the USSBS *Allied Campaign Against Rabaul*, *Oil in Japan's War*, *Reduction of Truk* and *Reduction of Wake Island*, Japanese Monograph 98, and Masterson.

## What the facts seem to allow

Two kinds of base appear in the rows.

**Bases fed at a distance by lines that no source shows cut in the period.**

- Noumea, Espiritu Santo and Efate from the West Coast, on the Army's planning factor of 45 days to a theatre, with the port, not the enemy, as the limit.
- Townsville from the southern Australian ports, stopped for four days in June and then convoyed.
- Pearl Harbor from the West Coast.
- Truk from Japan, hit by submarines only now and then.

**Forward bases supplied by named runs that the enemy found, struck and sank.**

- Guadalcanal and Tulagi from Espiritu Santo.
- Port Moresby across the Gulf of Papua, bombed 61 times by mid-June.
- Rabaul and the bases beyond it, through the submarines off Cape St George and the bombers over Simpson Harbour.
- Midway from Pearl Harbor, by single ships.
- Wake from Kwajalein.

The record states rates at only a few points:

- the planned fuel deliveries to the South Pacific (450,000 bbl, then about 225,000 bbl a month or 200,000 every 15 days);
- Noumea's 24 ships a month and its 5,000 t a day of December;
- Moresby's one ship at a time;
- Turner's 3,000 t a ship;
- the year's 167 ships to New Guinea.

It gives stocks at only one place, Midway's gasoline in June. For the Japanese side it gives no rate and no stock at all. So a rule could rest on the record for:

- which bases are fed from off the map and which by runs;
- the American South Pacific's fuel rate.

The stocks and the Japanese rates remain open, and any figure for them would be an estimate, with the gap kept open.
