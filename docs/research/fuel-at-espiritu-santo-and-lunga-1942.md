# Fuel at Espiritu Santo and at Lunga, 1942

Read on 2026-11-24 for the architect, for decision 0224 (rear bases and supply from outside the map), where Espiritu Santo's 600 t of fuel is gone on day 6 of the 39 days and Lunga's 200 t on day 4, and "no row gives a fuel stock or an oiler's load there" (ruling 3). Nothing here is settled and no game figure is proposed: the architect decides which rows a figure reads.

## Result in one paragraph

The record does not give what the architect wanted. For 7 August to 14 September 1942 no source read gives (a) a stock of fuel oil, diesel or avgas at Espiritu Santo, (b) the load of any oiler or tanker discharged at Espiritu Santo, Efate or Lunga/Tulagi, or (c) the avgas on hand at Henderson Field on any day between 10 August (400 drums) and 18 September (3,823 drums landed). What was found are dated stocks and loads **after** the window (October and November 1942), the Graybook's primary dispatches behind the July fuel plan, a few dates and quantities of deliveries to Noumea, Suva and Tongatabu, and the tank capacity built later at Espiritu Santo, Tulagi and Guadalcanal. They are filed as rows so that the architect can see how far they reach and where they stop. The gaps are written in `data/oob/gaps.csv` (the last five lines).

## What was read

All sources were fetched on 2026-11-24 and saved outside the repository; each quotation in a row was checked by program as a contiguous run of the saved text (whitespace collapsed; curly quotes made straight). Nothing from `references/` or the War in the Pacific database was used.

- Graybook (Nimitz, Command Summary) volumes 1 and 2, whole text, searched for tanker, oiler, avgas, gasoline, drums, barrels and every oiler and tanker name. Printed page numbers are read from the typescript; where a page has no number the locator says "inferred".
- Morison, vol. V (archive.org machine text), searched for fuel terms.
- Hough, Part VI chapters 3 to 7; Miller chapters III to VI; Zimmerman chapters 3 to 5; Craven and Cate vol. IV chapters 1 to 3; Carter chapters IV to VI; Building the Navy's Bases chapter XXIV; Dyer chapters 11 and 12 (HyperWar).
- DANFS (through the web archive) for Platte, Guadalupe, Sabine, Cimarron, Alhena, Fomalhaut, Bellatrix, Betelgeuse and McFarland. Kaskaskia and Kanawha were not reachable. DANFS gave no quantity of fuel discharged at any of the places and no row was made from it.

## What is filed

27 rows, `place_facts.csv` 14561 to 14587; five gap lines in `gaps.csv`. No new source key was needed. Every row is `unconfirmed`.

### In the window (7 August to 14 September 1942)

| Row | Place | Date | What it says |
| --- | --- | --- | --- |
| 14561 | Noumea | 9 Jul | The Graybook dispatch behind Carter's account (held rows 256, 14446): 225,000 bbl of fuel oil in chartered tankers due 22 July and the same on 2 August; about 225,000 bbl a month per carrier task force; diesel, avgas and stores as requested. A plan, in barrels |
| 14562, 14563 | Noumea, Suva | 4 and 5 Aug | Esso Little Rock three days overdue at Noumea; then arrived safely at Suva, diverted. Carter (held row 698) says the same tanker pair was diverted to Efate and Suva. No load stated |
| 14564 | Henderson Field | 12 Aug | The CINCPAC staff's record of the four APDs with avgas for the 15th: no quantity (Dyer, held, gives 120 t in all for the run) |
| 14565 | Noumea | 29 Aug | A carrier task force ordered to White Poppy for fuel and provisions on 2 September; low confidence (OCR) |
| 14566 | Tongatabu | 28 Aug | Enterprise's group asks to take about 23,000 bbl from the tanker H. S. Rheem due 31 August: a request, outside the three places |

### After the window, filed for the size of the figures

| Row | Place | Date | What it says |
| --- | --- | --- | --- |
| 14567 | Lunga | 14 Oct | Shelling destroyed 5,000 gallons of gasoline (Graybook) |
| 14575 | Lunga | 13 Oct | The noon raid burned 5,000 gallons of aviation fuel (Hough). A different event with the same figure; it also answers in part the gap on gasoline burned in the 13-14 October attacks |
| 14568, 14576, 14569, 14583 | Lunga | 16 Oct | The McFarland's run: Graybook "delivered 32,5000 gallons" (read 32,500); Morison "40,000 gallons ... in tanks below and drums topside"; Graybook staff narrative: "about 20000 gals. reached the shore" after the bomb hit, with 350 drums in a barge exploded; Craven and Cate "several thousand additional gallons". Four statements, not reconciled |
| 14570, 14577 | Lunga | 16 Oct | The Vireo's barge: 2,000 **drums** (Graybook) against 2,000 **barrels** per barge (Morison). The units disagree |
| 14571 | Lunga | about 18 Oct | The Southard delivered almost 10,000 gallons (date inferred) |
| 14578 | Lunga | Oct | The submarine Amberjack fitted to carry "gooo" gallons (read 9,000, a guess) |
| 14572 | Guadalcanal area | 22 Oct | "Over 100,000 gals" of avgas: enough to keep Geiger's planes filled, not B-17s |
| 14573, 14574 | Lunga, Tulagi | 28 Oct | 45,000 gallons at Guadalcanal; 4,900 at Tulagi. The one dated stock at the places |
| 14579, 14582 | Lunga | 15 Oct | A C-47 trip from Santo carried twelve drums, "enough to keep twelve planes in the air for one hour": one drum a plane-hour, the reader's arithmetic |
| 14580 | Lunga | 30 Nov | "Only four days' supply of aviation gasoline was on hand" (Morison) |
| 14581 | Lunga | Nov-Dec | Plan to build storage for a million gallons (Morison); Carter's "1,300,000 gallons" is in 14587 |
| 14584 to 14586 | Espiritu Santo | late 1942 and 1943 | Eleven 1,000-bbl avgas tanks with a filling station and lines for tankers (after October 1942); by 1943 seventeen 1,000-bbl motor-gas, two 10,000-bbl diesel and twenty-three 1,000-bbl avgas tanks (Building Bases and Carter agree on the totals); the fifty 10,000-bbl fuel-oil tanks of Aore are held row 262 |
| 14587 | Tulagi, Guadalcanal | later than 1942, undated | Tulagi area: ten 1,000-bbl tanks plus 12,000 bbl of avgas, 60,000 bbl diesel, 280,000 bbl fuel oil; Guadalcanal 1,300,000 gallons of avgas storage (Carter) |

## Where sources disagree

- **The McFarland's 16 October run:** 32,500 gallons delivered, 40,000 carried, about 20,000 landed, "several thousand". See 14568, 14569, 14576, 14583.
- **Barrels or drums on the Vireo's tow:** 14570 against 14577.
- **The Esso Little Rock:** the Graybook of 4 August has her overdue at Noumea; Carter says her cargo had been diverted before the Bishopsdale's loss. Not reconciled.
- **Gasoline burned in October:** 5,000 gallons by day (Hough) and 5,000 gallons by night (Graybook) are likely two events; neither source mentions the other.

## Already held, and what it still says about the figures

The rows already in `place_facts.csv` bear on the 600 t and 200 t more than the new ones do, and the architect should read them with these:

- **Espiritu Santo, August 1942: no bulk storage** (row 21): drums dumped from cargo vessels, one B-17 mission 50 drums, 25,000 gallons poured on 6 August. Carter's "fuel storage equal to about five tanker-loads" (row 232) is dated November 1942 or later, by the fuel unit's start (the text's "November 193" is a typo); it is not a stock of August.
- **McCain's request of 12 August** (row 699): fuel oil and diesel at once and 300,000 gallons of avgas for tenders within 7 days. Row 924 (Craven and Cate) has the Navy moving 300,000 gallons of gasoline to Espiritu for the 11th Group by 1 August. These are the only quantities a source ties to Espiritu Santo in the window, one a request and one a transfer.
- **Sabine at Espiritu Santo, 22 to 28 August** (rows 14435, 253): fuelled 11 ships; no quantity. Her cargo at Noumea on 30 and 31 August was 85,570 bbl taken from two chartered tankers (rows 254, 14436): what she took aboard, not what she discharged.
- **Henderson Field, 10 August to 18 September:** 400 drums on 10 August (row 193) and again 800 to 900 drums and some 150,000 gallons of Japanese motor fuel (land_unit_facts 177, 1175); a "large dump" at Halavo on 16 August (14534); 3,000 drums of avgas in the Santa Ana, held back (14456); 3,823 drums on 18 September (14467); a four-day reserve in mid-September (gaps). So the figure for the window is the 400 drums at the beginning and the 3,823 at the end, with the Japanese drums and the Navy's runs between them.

## Gaps

Written in `gaps.csv`: an oiler or tanker's discharge at Espiritu Santo in the window (none found; the 12 August tanker is unnamed); any fuel stock at Espiritu Santo or Efate in tons or gallons; avgas at Henderson Field by day from 13 August to 17 September; the Cactus Air Force's burn in gallons a day (only per-use rates are stated).

Not filed: Fletcher's ships' fuel on 8 August (Morison, footnote 18, from ships' logs: Saratoga 1,149,000 gallons and so on; Zimmerman, from the same logs: every ship had at least four days'). Those are ships' fuel, not base stocks. The Platte's and Cimarron's loads of 12,000 t fuel oil and 500,000 gallons of avgas each (Zimmerman) are already on file in the oiler passes.

## For the architect

- The only window quantities at Espiritu Santo are a transfer of 300,000 gallons of avgas by 1 August and McCain's request of 12 August for the same quantity; the only window oiler is the Sabine, with no quantity. A fuel figure for either place that rests on rows needs a gap kept open under it.
- After the window the Lunga stocks are over 100,000 gallons in the area (22 October) and 45,000 at Guadalcanal (28 October), of the same size as one McFarland run (20,000 to 40,000 gallons). They are an order of magnitude, not a rate, and are of October; read back into August or September they would be a guess.
- Units: barrels, gallons and drums are all used; none is converted in any row. The held conversions are 42 gallons to the barrel and 277 gallons to the ton of fuel oil (FTP 218, `fuelling-in-port.md`), and a drum of 55 gallons (row 201).
