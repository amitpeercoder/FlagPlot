# Real oilers of 1942

Status: a research report for the architect, read on 2026-11-21, for decision 0201 (refuelling at sea), stage B (rule 28). Facts and their strength only: nothing is derived or settled here. The rows of `docs/research/fuelling-at-sea.md` (ship_facts 12834 to 12923) are not repeated.

## What was filed

| File | Ids |
| --- | --- |
| `ships.csv` | 14300 to 14304: Sabine (AO-25), Kaskaskia (AO-27), Toa Maru, Shinkoku Maru, Shiriya. Tippecanoe's `class` filled as `us-patoka-class` |
| `ship_classes.csv` | 14300 `us-patoka-class` |
| `ship_facts.csv` | 14300 to 14505 |
| `ship_class_facts.csv` | 14300 to 14310; row 2512 (Wikipedia's 7,470 light) changed from unconfirmed to disputed, with a sentence appended |
| `engagement_forces.csv` | 14300 to 14313 |
| `engagement_positions.csv` | 14300 to 14306 |
| `attributes.csv` | three new text attributes: `cargo_oil_note` (class and ship) and `cargo_oil_load_note` (ship) |
| `gaps.csv` | the last 12 entries |

The new attributes keep cargo apart from a ship's own fuel. `cargo_oil_note` is what a tanker could carry for others, as printed, with its unit and what it includes; `cargo_oil_load_note` is a load on a date (taken aboard, emptied, a voyage cargo). Neither converts anything. Rule 28's "attribute for cargo oil read from rows" is still the architect's to make: these hold the statements in the meantime.

## How it was read

- Two researchers read the US and Japanese web sources and proposed rows with quotations; I read Ships' Data 1945, FTP 218, Zimmerman and the shipscribe class pages myself. Every quotation in a row, or in a row's note, was checked by program as a contiguous run of the saved page's text (tags stripped, whitespace collapsed); 14 of the Japanese proposals failed on dropped apostrophes and were rewritten from the page or left out. Pages are kept outside the repository.
- Ships' Data 1945 (volume III, the copy read for `ships-data-1945.md`) was read from the page images, oiler table pp. 158 to 180, with each oiler's figures checked again at 200 to 300 dpi. Its text layer was used only to find the lines.
- The Graybook rows quote its uncorrected OCR ("Plat-ce", "GUADULUPE"), as their notes say.

## The classes

**Cimarron class** (Cimarron, Neosho, Platte, Sabine, Kaskaskia, Guadalupe; shipscribe's AO-22 class). Each figure below is a 1945 statement unless said otherwise.

| Fact | Rows say |
| --- | --- |
| Light displacement | 7,256 (shipscribe; DANFS for Kaskaskia) against 7,470 (Wikipedia). Disputed |
| Limiting or full load | 25,440 limiting (shipscribe; Ships' Data for four ships, 25,425 for Cimarron); DANFS gives Sabine 25,440 "(f.)"; Wikipedia 24,830 full load. Trial displacement 23,235 |
| Speed | trial 18.3 (Ships' Data, every ship), FTP 218 19.0 Cimarron and 18.0 the others, 18 (DANFS), 20 (Wikipedia Neosho) |
| Power | 13,500 shp (Ships' Data, shipscribe) against 30,400 (Wikipedia Neosho, uboat.net) |
| Her own bunkers | fuel oil 14,850 bbl Cimarron, 15,000 bbl Platte, Sabine, Kaskaskia, Guadalupe (Ships' Data, definition 14: 95 per cent of the tanks) |
| Endurance at a speed | **none found.** The only figure with a speed is the General Board's requirement of 1935 (10,000 miles at 16.5 knots), filed as a requirement; Wikipedia's 12,100 nmi has no speed |
| Cargo | see below |

Cargo, each a separate row, none reconciled:

| Source | Date | Cimarron | Platte | Sabine | Kaskaskia | Guadalupe | What it is |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Zimmerman fn. 35 | July 1942 | 12,000 t fuel oil + 500,000 gal avgas | same | - | 13,000 t oil | - | loads at the Fiji rehearsal |
| Carter ch. IV | July 1942 | - | took 93,000 bbl at Noumea | 85,570 bbl taken 30 to 31 Aug | - | - | loads |
| Wildenberg Table 19 | Nov 1943 | 103,233 bbl | 101,638 | 104,938 | - | 104,938 | "cargo oil", kind not stated |
| Ships' Data | Apr 1945 | 122,400 bbl oil, 805,000 gal gasoline | 123,700; 788,000 | same | same | same | total cargo tank capacity, oil including diesel and lube |
| Ships' Data | Apr 1945 | 16,500 t | 16,500 | 16,500 | 16,500 | 16,500 | cargo deadweight at limiting draft (total 18,200) |
| FTP 218 App. B | Sept 1945 | fuel oil 87,246, diesel 9,968, gasoline 19,600 bbl (116,814) | 68,054 fuel oil (103,229) | 68,054 (103,269) | 68,054 (102,857) | 68,054 (100,571) | tanks by product as allocated in design; deadweight 18,250 t |

So the decision's figures: 12,000 tons is Zimmerman's July load; 10,317 and 13,226 tons are FTP 218's fuel-oil columns at 0.1516 t a barrel (the reader's arithmetic, not a row); "about 15,900" matches Wildenberg's 104,938 bbl at the same factor. The Graybook's "105,000 bbls each" of about May 1942, cited in `fuelling-at-sea.md`, was not found again in the OCR text searched this time, so no row rests on it. FTP 218 also gives three main cargo pumps of 2,000 gallons a minute for the class.

**Patoka class** (Tippecanoe; shipscribe's AO-9 class): 5,375 light, 16,800 full, 10.5 knots, 2,900 hp (Tippecanoe a Curtis turbine, 2,600 shp in Ships' Data), 11,145 tons of oil as built (shipscribe). Tippecanoe in 1945: trial 11.0 knots, trial 15,590 t, limiting 17,820 t, bunkers 4,780 bbl, cargo 79,400 bbl and no gasoline, deadweight 12,400 t (cargo 11,100); FTP 218: 10.5 knots, 63,509 bbl fuel oil, 72,359 total, two pumps. No endurance found.

**Japanese oilers** (combinedfleet records and class pages; none gives displacement, deadweight, bunkers or range):

| Oiler | Tonnage | Speed | Cargo | Strength |
| --- | --- | --- | --- | --- |
| Toa Maru | 10,051 GRT | 19.5 | 93,000 bbl | stated, own class page |
| Toho Maru | 9,997-ton at laying down | 19.5 | 93,000 bbl | inferred; her class is disputed (Toa and Tatekawa class pages both claim her) |
| Kyokuto Maru | 10,051-ton | 19.5 | 93,000 bbl | inferred, Toa class |
| Nippon Maru | - | 19.5 | 103,000 bbl | cargo stated by name on the Tatekawa page; speed inferred |
| Toei, Kokuyo, Kenyo, Nichiei, Genyo Maru | 10,018 to 10,027-ton | 19.5 (Kokuyo: 19 empty, her own page) | 93,000 bbl | inferred, Itsukushima class; the page's tons conversion disagrees with its own factor |
| Shinkoku Maru | 10,020 GRT | 19.5 | 93,300 bbl | stated, own class page |
| Hoyo Maru | 8,691 GRT | 16.1 | 95,000 bbl | stated |
| San Clemente Maru | 7,385 then 7,354 GRT | 15 | 87,960 bbl | stated, San Ramon class |

Prewar loads at Panama (Toho 10,677 t fuel oil, 1938; Kenyo 11,964 t crude, Toei 12,048 t crude and 12,307 t diesel, Toa 12,032 t crude, 1940) and Toho's 8,200 t received from Nisshin Maru at Shortland on 12 to 13 May 1942 are filed as loads. Iro, Tsurumi, Sata and Shiriya: no characteristics read.

## Which oiler was where

**Coral Sea, 1 to 8 May.** US: Neosho met the carriers on 1 May (Wildenberg) and fuelled Task Force 17 on 2 to 3 May and the combined force on 5 to 6 May (ONI; Wildenberg puts the combined fuelling on 4 May); detached with Russell on 3 May to carry the new rendezvous, and with Sims at 1725 on 6 May for Point Rye (odd days Rye, even days Corn, Bates). Tippecanoe joined with Chicago and Perkins at 16-00 S, 161-45 E on 1 May, was to be emptied and sent back to Efate (ONI), yet ONI's task organisation of 6 May lists her with Worden in the fuelling group; DANFS has her at Efate during the battle. Kaskaskia is in Wildenberg's table for the battle but on the Graybook's plan of 26 April she was to reach Noumea on 23 May: filed `unconfirmed`. Japan: Toho Maru with Takagi's striking force (already on file); after the battle she took 8,200 t from Nisshin Maru at Shortland and left on 13 May with Shigure. Iro and Hoyo Maru as already on file.

**Midway** (cheaply): Graybook of 8 June, Cimarron and Guadalupe to Pearl with Blue and Ralph Talbot after fuelling Task Force 16. The Japanese supply groups are already on file.

**Guadalcanal landing, July to 10 August.** Cimarron and Platte came from Pearl with Task Force 11 (Carter; Graybook 9 July). On 21 July Platte pumped her oil into Cimarron, refilled 93,000 bbl at Noumea and rejoined; about 29 July Task Force 16, the Anzac Squadron and part of the amphibious force emptied Cimarron, who went to Noumea to refill. Kaskaskia was to leave Pearl 20 July and was ordered to Koro on 28 July (Carter); DANFS instead has her at Kodiak on 3 July, contradicted by Carter, Dyer and ONI. Task Force 62 fuelled from Kanawha and Platte at Koro on 31 July. ONI lists Platte, Cimarron, Kaskaskia, Sabine and Kanawha as Task Force 61's oilers; Sabine was still on passage from San Pedro (left about 2 August), so her row there is `unconfirmed`.

**After the landing to the Eastern Solomons.** 14 August: Platte and Kaskaskia "completely emptied" after a week (Ghormley, in Carter); Platte back at Noumea refilling (DANFS); Cimarron held to keep the force fuelled. Kaskaskia fuelled Enterprise on 18 August (Wildenberg caption). Guadalupe sailed with Task Force 17 (Hornet) on 16 August (Carter). Platte and Cimarron left Roses at daylight on 23 August (Graybook) and met the retiring Task Force 61 at 0800 on 25 August at 13-04 S, 164-03.5 E (ONI). Sabine reached Espiritu Santo on 22 August, went to sea with Task Force 61 for two days, returned on the 26th, fuelled 11 ships and sailed for Noumea on the 28th (Carter); DANFS's "in company with Saratoga" is filed `low`. 24 August: Guadalupe to go with a destroyer to White Poppy to refill; 31 August (GCT): Cimarron and Guadalupe to a new rendezvous for Task Force 17 at 13-15 S, 168-35 E about 0800 on 2 September.

Japan: Toho and Toei Maru, in the Guadalcanal Transport Replenishment Group No. 1 from 14 August, fuelled Zuikaku together on 21 August; Toho fuelled Suzuya on 22 August (02-36N 157-40E) and 26 August, Toei fuelled Kumano on 22 August (1,205 t) and through the night of 25 to 26 August (1,307 t); both took Ryujo's survivors. Kyokuto and Nippon Maru loaded at Tarakan, reached Davao 18 August and joined the fleet north of the Solomons on 21 August escorted by Shikinami and Uranami (Ryujo's record; Kyokuto's agrees, Nippon's own has her leaving Truk in an entry of 22 to 28 August); Kyokuto was "in the fleet supply train" on 24 August. Shinkoku and Nichiei Maru, of the Advance Unit's group, were torpedoed by Gudgeon on 17 August at 07-40N 151-05E and withdrew. Kokuyo Maru, reassigned 14 August, left Sasebo only on 1 September. Kenyo Maru joined Cruiser Squadron 5 at sea on 30 August. Iro fuelled Cruiser Division 6 in Mowe Passage on 11 August and went to Jaluit; San Clemente Maru joined the Eighth Fleet at Rabaul on 1 September; Toa Maru served Shortland from 1 September; Shiriya loaded 8,000 t and reached Truk on 8 September.

## Where sources disagree (not reconciled)

- Cimarron class light displacement (7,256 or 7,470), speed (18, 18.3, 19.0, 20), power (13,500 or 30,400) and every cargo figure above.
- Neosho's combined fuelling: 4 May (Wildenberg) or 5 and 6 May (ONI, Bates).
- Tippecanoe on 6 May: in the fuelling group (ONI organisation) or at Efate (DANFS, ONI chapter 1).
- Kaskaskia: at the Coral Sea (Wildenberg) or not (Graybook); in the South Pacific from July (Carter, Dyer, ONI) or at Kodiak (DANFS).
- Sabine at the Eastern Solomons: with Task Force 61 for two days (Carter) or "in company with Saratoga" (DANFS).
- Nippon Maru joining the fleet: 21 August (Ryujo's record) or from Truk in 22 to 28 August (her own).
- Toho Maru's class: Toa class or Tatekawa class.
- San Clemente Maru's gross tonnage: 7,385, 7,354 or 7,309 (class).

## Gaps

In `gaps.csv` (the last 12): endurance at a speed for the Cimarron and Patoka classes; any cargo capacity stated for 1942; Neosho's own figures; DANFS for Cimarron, Guadalupe and Tippecanoe (404 or refused); escorts by name and quantities at the August rendezvous; positions other than 25 August and 2 September; displacement, deadweight, bunkers and range of every Japanese oiler; characteristics of Iro, Tsurumi, Sata, Shiriya; which oilers served Kondo's and Abe's forces on 20 to 25 August; Toho Maru's class; and whether `shikoku-maru` (6387, from Japanese Monograph 93) is the same ship as the new `shinkoku-maru` (14303), left for the architect.

Could not be read: navsource (refused), the NHHC DANFS pages, the HyperWar DANFS for AO-21, 22 and 32, several combinedfleet pages (Hiei, Haguro, and some guessed addresses of the carriers' and cruisers' records). Kanawha and the chartered tankers have no entity and no rows beyond the notes.
