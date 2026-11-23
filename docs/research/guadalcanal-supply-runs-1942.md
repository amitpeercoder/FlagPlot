# Supply runs to Guadalcanal, August and September 1942

Read on 2026-11-23 for the architect, after the measurement of the 39 days in decision 0224 found that the Japanese runs sail from Shortland, which the scenario gives no stock, and that Espiritu Santo's stock is one run. Two questions were put:

1. **The Japanese runs.** For each run a source describes: the ships, where they loaded and staged, what they carried, landed and lost. Then what was held at Shortland, Faisi and Buin, when it became a base and from where it was fed, and what the troops ashore received.
2. **Espiritu Santo and the Allied runs.** What was at Espiritu Santo and what arrived, each run to Guadalcanal and Tulagi with its tons, and how Henderson Field's gasoline and the Marines' rations stood by date.

The note settles nothing and proposes no rule. It builds on `docs/research/oob/runs-to-guadalcanal.md`, `docs/research/oob/ship-names-39-days.md`, `docs/research/oob/supply-levels-and-convoy-cycles.md` and `docs/research/rear-bases-and-supply-1942.md`, whose rows are cited below as held, and does not repeat them.

## How it was read

The lead downloaded every document to a folder outside the repository and turned it into text. Three Sonnet readers then read it and staged 413 statements, each with its exact words, the file and the line:

- **Japanese side:** the combinedfleet.com records of about 50 destroyers, transports, patrol boats, oilers and tenders; Morison vol. V; Saito (NIDS, 2013); Bullard's translation of Senshi Sosho; Miller, Zimmerman and Hough on the Japanese side.
- **Allied side:** Miller chapters 3 to 6, Zimmerman chapters 4 to 6, Hough part VI chapters 4 to 6, Dyer chapters 8 to 12, Carter chapters IV and V, *Building the Navy's Bases* chapter XXIV, and Morison vol. V.
- **The Graybook:** volumes 1 and 2, 1 August to about 20 September 1942, as machine-read text.

The lead then:

- **Checked every staged quotation by program** against its saved text, whitespace ignored. All 413 passed.
- **Removed every statement already on file.** A program compared each quotation with the `as_stated` of every row in `data/oob`, and the lead read the near-matches.
- **Extended each quotation that began mid-sentence to its whole sentence,** where that did not cross a page marker, and checked the result again. The row's note says so.
- **Read the figures the answers rest on in the saved texts.** These are the Truk, Palau, Rabaul and Shortland entries of the Sado Maru, Boston Maru, Amagiri and Yudachi records; Saito's rations; Hough on Tanaka loading supplies; Fomalhaut's report of 26 August; and the Graybook's entries on BUTTON.

One quotation crosses a page break and could not be checked by program. It is Hough's sentence on Tanaka loading supplies (`troop_lift_facts` 12837), whose text runs on over his map page; its note says so.

189 rows were filed. Every row is `unconfirmed`, `medium` and `stated`.

| File | Ids | Rows |
| --- | --- | --- |
| `troop_lift_facts.csv` | 12803 to 12890 | 88 |
| `place_facts.csv` | 14478 to 14560 | 83 |
| `land_unit_facts.csv` | 15241 to 15258 | 18 |
| `lift_entities.csv` | 6501 to 6505 (four Japanese runs and the Lakatoi) | 5 |
| `sources.csv` | 16223 (`morison-v5`) | 1 |
| `gaps.csv` | the last 8 entries | 8 |

`place_facts.csv` reuses the attributes of the rear-bases pass (`base_role_note`, `fed_from_note`, `stock_note`, `supply_interrupted_note`, `arrival_rate_note`, `port_note`, `fuelling_note`). The Graybook's statements about Japanese activity are what the Allies reported or believed. Each such row says so in its note.

**Could not get:** Frank's *Guadalcanal*; Tanaka's account in *The Japanese Navy in World War II*; Hara's *Japanese Destroyer Captain*; Senshi Sosho's Guadalcanal volumes; and Japanese Monograph 98 as text (it is a scan, read as images in an earlier pass). None is online in a form that could be read.

Row prefixes in the tables: `T` is `troop_lift_facts`, `P` is `place_facts`, `L` is `land_unit_facts`, `S` is `ship_facts`, `Pa` is `passages`. Ids at or above 12803 (T), 14478 (P) and 15241 (L) are new; the rest were held.

## The Japanese runs

| Date landed | Ships | Loaded at, and staged | Carried | Landed, lost | Rows |
| --- | --- | --- | --- | --- | --- |
| 18 Aug (night) | Arashi, Hagikaze, Kagero, Hamakaze, Tanikaze, Urakaze | **Loaded at Truk.** Ichiki's men came Saipan to Truk in Boston and Daifuku Maru, arriving 12 Aug. Left Truk 0500 on 16 Aug. Hough has the ships loading supplies at Truk on 15 Aug and ordered "down to Rabaul"; Miller says "via Rabaul". | 916 or 917 men (900 and 1,000 on file); two infantry guns (Saito) | Landed at Taivu, 2030 on the 18th or 0100 on the 19th | T 12803 to 12810; held T 702, 703, 142, 43; Pa 430, 431 |
| 16 Aug | Oite | not stated in this pass | 200 or 500 men | landed | held T 135 to 137, 700 |
| 24 to 25 Aug | Uzuki | **From Rabaul** | food and ammunition | landed at Kokumbona | held T 723 |
| 25 Aug (turned back) | Boston Maru and Daifuku Maru (second echelon), Kinryu Maru (third echelon), patrol boats 1, 2, 34 and 35; Jintsu and destroyers escorting | **Loaded at Truk,** left 16 and 17 Aug. The SNLF came to Truk by Guam and Saipan. Retired to **Faisi, Shortlands,** on 25 to 26 Aug by order from Rabaul. | 1,100 (second echelon, combinedfleet) or 700 (Morison) men; about 800 or 1,000 SNLF | Kinryu Maru bombed and sunk, Mutsuki sunk; nothing landed. Morison: the troops were "reloaded in fast destroyers" for a later run. | T 12811 to 12817; held T 1 to 9, 736, 737; Pa 82, 432 |
| (convoy) | Sado Maru and Asakasan Maru, Sendai and Desdiv 20 | **Kawaguchi Detachment embarked at Palau** on 13 Aug. Left Truk 24 Aug. The rest disembarked **at Rabaul** on 27 Aug. | the brigade | the 600 transferred at sea (next row); the rest landed at Rabaul | T 12824 to 12832 |
| 28 Aug (turned back) | Yugiri, Asagiri, Amagiri, Shirakumo | **Transferred at sea** from Sado Maru on 26 Aug | 600 (Sado Maru record) or 550 (Amagiri) men | Bombed north of Guadalcanal: Asagiri sunk, Shirakumo towed to the Shortlands | T 12833, 12834; held T 143 to 145, 708, 709 |
| (transport) | Sado Maru; Fubuki, Shirayuki, Hatsuyuki escorting | **1st Bn 124th Infantry re-embarked at Rabaul** on 28 Aug. Arrived at **the Shortlands** on 29 Aug (Hough: "due from Truk"). | the battalion, most of it | (staged at the Shortlands) | T 12835, 12836 |
| 29 Aug (night) | Desdiv 24 and Desdiv 11 (5 destroyers; 4 in Zimmerman) | **Sortied from the Shortlands.** Tanaka "loaded supplies" on Desdiv 24 (Hough). | 450 men (Morison, Saito) plus 300 of Ichiki's rear echelon with four antitank guns (Saito); 750 or 900 on file | landed at Taivu | T 12837, 12838; held T 148, 149, 173, 174, 711 to 713; Pa 1 |
| 30 Aug (night) | Yudachi | **From the Shortlands,** 1000 on 30 Aug. The men were moved from Isokaze. | 130 men | landed at Taivu | held T 150 to 152, 710; Pa 394 |
| 31 Aug (night) | 8 destroyers (Desdiv 24, Kagero, Desdiv 11, Amagiri) | **From the Shortlands,** 0800 | 143 to 155 men a ship, about 1,200, with engineers, artillery and antitank units (Morison) | landed at Taivu or Tasimboko (Morison: "the west flank") | T 12839, 12840; held T 153 to 156 |
| 1 to 2 Sep | Sado Maru and Asakasan Maru with barges; Yudachi and Murakumo escorting | **Shortlands to Gizo and back** | barges for the Oka group | not stated | held T 716 |
| 2 to 6 Sep | barges of the Oka group; Kawakaze, Umikaze and Suzukaze escorted barges from Santa Isabel on 4 to 5 Sep | **Gizo and Santa Isabel** | about 1,000 men (Saito), 1,100 on file | a third reached Guadalcanal (Saito); landed at Kokumbona on 6 Sep (Miller) | T 12841, 12842; held T 171, 172, 721; L 916 to 918 |
| 2 Sep | Tsugaru and four destroyers, covered by Fubuki, Shirayuki and Amagiri | **Shortland** | 450 men, AA guns, artillery | landed | held T 717 to 719 |
| 4 to 5 Sep | Uranami, Shikinami, Ariake; Yudachi, Hatsuyuki, Murakumo | **From the Shortlands,** 0330 | about 1,000 men of Ichiki and **Aoba** "troops and materials"; the Aoba Detachment had arrived at Rabaul on 31 Aug (Bullard) | landed at Taivu | T 12843; L 15241; held T 161, 162 |
| 5 Sep | Fubuki, Shirayuki, Amagiri, Kagero, Yugure | (from the Shortlands) | 231 men "plus food and ammunition" | landed | held T 163 to 165 |
| 6 Sep | Yudachi and three others | **From the Shortlands,** 1105 | 370 men "and provisions" | landed off Lunga | held T 166, 167 |
| 11 Sep | Yudachi, Umikaze, Kawakaze | **From the Shortlands,** 1000 | 637 of the Aoba Detachment | landed at Kamimbo; unloading suspended | T 12844; held T 168, 169 |
| 17 Sep | Amagiri, Sazanami | not stated | "food and medical supplies" | to Kamimbo | T 12845 |
| 20 Sep | Yudachi, Ushio, Sazanami, Shikinami, each towing a barge | not stated; the munitions came from the tender Nisshin | munitions | to Kamimbo | T 12846, 12847 |

Two general statements:

- **The switch to destroyers.** The 17th Army switched the Ichiki second echelon and the Kawaguchi Detachment to destroyers on 26 Aug (Bullard). Combined Fleet said further transport would be "mainly with destroyers" (Saito). T 12818, 12819, 12822.
- **What a destroyer carried.** Saito: the men landed from 31 Aug to 7 Sep, 5,600 in 50 round trips, came "with approximately two weeks' rations"; a destroyer's capacity was "110 men, 0.3 guns ... and rations for two weeks". T 12821; held T 170, 176; L 921.

## Shortland, Faisi and Buin

What the record states:

- **A base since March 1942.** It was occupied on 30 March 1942 as "a relay base for the Tulagi offensive". The anchorage could shelter a large force, and an inlet on Shortland Island was suited to seaplanes (Bullard; P 14478 to 14480).
- **Where the August convoy retired.** The Tanaka convoy retired there on 25 to 26 Aug (T 12813, 12815; held Pa 432). Cruiser Division 6 arrived on 22 Aug (P 14481). The 17th Destroyer Squadron went there straight from Milne Bay on 29 Aug (P 14489).
- **The seaplane force.** The R-Area Air Force was formed on 28 Aug, "based in Shortland harbor", with Rekata Bay as its forward base. Chitose's aircraft joined it there, and from 14 Sep its seaplanes scouted the Noumea to Guadalcanal route (P 14483 to 14485, 14488, 14490). Kunikawa Maru left Kure on 24 Aug to operate from the Shortlands area (P 14482).
- **Fuel.** Patrol boat 34 fuelled there on 23 Aug, probably from Akebono Maru (held S 2282). The oiler Toa Maru came from Truk, arrived on 6 Sep and fuelled three heavy cruisers; Toa Maru was "mainly" a station tanker there (held P 14328, 269, 702). The tanker Toho Maru was escorted in on 24 Sep (held ship rows).
- **Buin.** Kunikawa Maru left Truk on 8 Sep with a construction force and materials for the airfield at Buin (P 14487).
- **Attacks.** One B-17 attacked the anchorage on 31 Aug (P 14486). The Allies reported Faisi mined on 23 Sep and claimed hits on shipping there on 24 and 26 Sep, after the 39 days (P 14551 to 14553).
- **Where the runs started.** Every destroyer run from 29 Aug to 11 Sep whose start is stated left the Shortlands (table above).
- **What fed it.** The men came from Rabaul in Sado Maru (28 to 29 Aug, and again 13 to 14 Sep). The oiler and the construction force came from Truk. Barges went on to Gizo.
- **Stores.** Hough says supplies were loaded into the destroyers there on 29 Aug.

**Not found:** any stock of provisions, ammunition or fuel at Shortland, Faisi or Buin in tons or days, a depot, or a ship that brought cargo for one (gap). What the record shows is a staging anchorage where men brought from Rabaul were moved into destroyers, with an oiler and a seaplane force from late August. The provisions carried are stated only as each run's "two weeks' rations", "food and ammunition" or "provisions".

## The Allied runs

| Date | Ships | Loaded at | Carried | Landed, lost | Rows |
| --- | --- | --- | --- | --- | --- |
| 15 Aug (night) | 4 APDs of Transdiv 12 (Colhoun, Gregory, Little, McKean by Morison) | **Espiritu Santo**, on COMSOPAC's order of 12 Aug. Left by day on the 14th, at 18 knots. | 400 drums of avgas, 32 of lubricant, 282 bombs, belted ammunition, parts; 120 t in all (Dyer); 30 t a ship; about 120 men of CUB One. No rations. | all unloaded; left safely | T 12848 to 12858; held T 86 to 89, 390, 391, 573, 650, 651 |
| 16 Aug | Manley, Stringham | Espiritu Santo (DANFS) | bombs, ammunition, gasoline | — | held T 652 only; not in any source read here (gap) |
| 20 Aug | William Ward Burrows | the New Hebrides | MAG-23's forward echelon | men and some gear landed; aground at Tulagi, gear jettisoned | T 12872, 12873; held T 657 |
| 20 to 21 Aug | 3 APDs (Colhoun, Gregory, Little); McFarland | Espiritu Santo | 120 t of rations, about 3½ days (Morison 20 Aug; Miller 21 Aug); McFarland aviation material | landed | T 12859; held T 656, L 1123 |
| 21 to 23 Aug | Alhena, Fomalhaut; 6 APDs; Blue, Helm, Henley escorting | **Fomalhaut at Noumea** ("95 percent that taken at WHITE POPPY"); Alhena "from Espiritu Santo" (Zimmerman). On 16 Aug one AK was loading at Noumea and one at Espiritu Santo. | Group: over 2,000 t, 200 t of it rations (Dyer). Fomalhaut alone: about 1,500 t discharged, three quarters of her cargo. | Fomalhaut did not land her 20 mm and larger ammunition or wire. Unloading took over 24 hours. Blue torpedoed. Fomalhaut had been turned back on 20 Aug because only one large ship could be handled at a time. | T 12860 to 12871; P 14535; held T 60 to 62, 392 to 394 |
| 29 to 30 Aug | Burrows, Kopara; APDs; Gamble, Tracy | Efate (held row); Kopara's cargo of bombs and avgas from Morinda | ammunition, rations, aviation spirit, stores (held) | Colhoun sunk with 17 t aboard; Burrows aground twice, off on 3 Sep, left 4 Sep | T 12874 to 12880; held T 398 to 400, 659 to 662 |
| 31 Aug to 1 Sep | Betelgeuse; Henley escorting | not stated | 6th Naval Construction Battalion; a 5-inch battery | landed | held T 663 to 665, 683 |
| 3 Sep | Fomalhaut and 3 APDs | not stated | "more supplies" | landed; Little and Gregory sunk on 4 to 5 Sep | held T 673 |
| 7 to 8 Sep | Fuller, Bellatrix | not stated | not stated | stopped unloading and left before an expected attack | T 12881, 12882; held T 402 |
| 9 Sep | Santa Ana (civilian manned, held back) | — | 3,000 drums of avgas, mat | Turner would not send her | held P 14456, T 666 |
| 14 Sep | Bellatrix, Fuller | not stated | gasoline (unloaded at Tulagi) | landed | held T 681, P 14466 |
| 18 Sep | 6 transports of the 7th Marines; McFarland, Tracy, Bellatrix with gasoline | **Espiritu Santo,** sailed 0515 on 14 Sep (the 7th Marines came from Samoa) | 4,180 or 4,262 men; 3,823 drums of fuel; 1,012 t of rations landed (82 B, 930 C and D); 8,825 t of rations still aboard (as printed) | landed in about twelve hours | T 12883 to 12889; held T 403 to 408, P 14467 |
| undated (August) | Lakatoi | **Noumea** | 400 t of rations and ammunition | capsized three days out | T 12890 |

## Espiritu Santo

What the record states:

- **August 1942.** The base was begun on 8 July and in early August was a "small shadow" of what it became. In mid-August it was "in the earliest throes of being built and stocked" (Dyer; P 14491, 14494).
- **Role.** It was named the "present strong point" on 9 Aug (P 14507). The 2d Marines landed there on 12 Aug, and 1,800 of them reinforced it on 16 Aug (P 14508 to 14510). COMSOPAC called it the "staging point for supplies to CACTUS" on 26 Aug (P 14512). It staged aircraft to Guadalcanal (P 14513 to 14515).
- **Depots and units.** A branch of the division's base depot was ordered on 14 Aug, and Marine Advanced Supply Depots at Noumea and Espiritu Santo on 20 Aug (P 14492, 14495). It became a full supply depot only with LION One in February 1943 (P 14493). CUB One arrived on 11 Aug and landed gasoline trucks and drums (P 14499, 14505, 14506).
- **Fuel.** McCain asked on 12 Aug for fuel oil, diesel and 300,000 gallons of avgas within 7 days (held P 699). A tanker was then on its way, and the nearest diesel was at Suva (P 14496, 14497). Sabine was there from 22 to 28 Aug and fuelled 11 ships (held P 14435). Efate had no fuel for Task Force 62 on 2 Aug (P 14520, 14521).
- **Supply.** On 20 Aug it was "receiving vital supplies from NOUMEA via B-17s" (P 14511). On 16 Aug an AK was loading there with rations and ammunition (P 14535).

**Not found:** any stock at Espiritu Santo of stores, rations, avgas, fuel oil or ammunition, in tons, gallons or days, in August or September 1942 (gap). The Graybook's BUTTON entries and Dyer's chapter on CUB One give arrivals and roles, never quantities.

## Henderson Field and the men ashore

**Aviation gasoline:**

| Date | What the record states | Rows |
| --- | --- | --- |
| 10 Aug | 400 drums on hand | held 193 |
| 15 to 16 Aug | Fuel available at Kukum; a "large dump" of avgas at Halavo | P 14529, 14534 |
| 20 Aug | Fighters cannot be kept up "more than a few days" without aviation supplies | P 14530 |
| 2 Sep | Some avgas destroyed by bombing | P 14531 |
| 13 Sep | "OKay except for gas shortage" | P 14532 |
| 15 Sep | Need "acute" | P 14533 |
| 18 Sep | 3,823 drums landed | held P 14467 |

No quantity was found for any day between 20 August and 17 September (gap).

**The Marines' rations:**

| Date | What the record states | Rows |
| --- | --- | --- |
| From 12 Aug | Two meals a day, with captured rice; food for "but a month" (Morison) | L 15246 to 15248 |
| 16 Aug | 3 days' rations, as the Graybook prints the dispatch | L 15255 |
| 19 Aug | The food situation "not yet cleared up" | L 15256 |
| 20 Aug | "Met before anyone goes hungry" | L 15257 |
| 11 Sep | "Food situation CACTUS not good" | L 15258 |
| About 18 Sep | Full rations restored | held 14470 |

**The Japanese ashore:**

- Some of the garrison were "without food" on 12 Aug (L 15242, 15243).
- The landings of 31 Aug to 7 Sep brought two weeks' rations (T 12821).
- The main depot at Tasimboko, with rice, ammunition and medical supplies, was destroyed on 8 Sep, and "neither food nor equipment remained in a usable condition" (L 15244, 15245).
- The force went to the ridge on 12 to 14 Sep with "only a few days' rations" (held L 312).

## What the record supports

**Firmly** (several sources agree, including the ships' own records):

- **The August runs.** The Ichiki first echelon and the Tanaka convoy loaded at Truk.
- **The Kawaguchi Detachment.** It embarked at Palau, came through Truk and was landed at Rabaul, all but 600 men put into destroyers at sea. Its battalions came from Rabaul to the Shortlands in Sado Maru.
- **The destroyer runs.** Every run of 29 August to 11 September whose start is stated left the Shortlands. The Shortlands were the staging anchorage, fed with men from Rabaul and with an oiler and a construction force from Truk.
- **The Allied runs.** The runs to Guadalcanal loaded at Espiritu Santo or Noumea. Espiritu Santo was the staging point, without a stated stock and with no depot of size until 1943. The 15 August APD run carried 120 t, about 30 t a ship.

**Loosely** (one source, or sources that disagree):

- **Ichiki's first echelon.** Hough has it ordered "down to Rabaul" and Miller has it "via Rabaul"; the ships' records and Morison have it sail from Truk.
- **The 21 to 22 August group.** It landed over 2,000 t (Dyer alone), and Fomalhaut's share was about 1,500 t (her own report alone).
- **Rations on the destroyer runs.** Two weeks' rations is Saito's figure, in two papers.
- **Supplies at the Shortlands.** Hough alone says supplies were loaded into destroyers there.

**Gaps** (all in `gaps.csv`):

- any stock at Shortland, Faisi, Buin or Espiritu Santo;
- tons of provisions on any Japanese run;
- the Japanese ration state by day;
- Henderson Field's gasoline by quantity from 20 Aug to 17 Sep;
- the cargo tons of every Allied cargo ship but Fomalhaut;
- the Manley and Stringham run in any source but DANFS;
- the Japanese monographs and books that might fill these.

## What the facts seem to allow

The Japanese side has two kinds of place:

- **A loading port with stores and troops: Rabaul, and Truk before it.** Every unit of the 39 days whose embarkation is stated was brought to Rabaul or Truk by transport.
- **A forward anchorage: the Shortlands.** There men from Rabaul were moved into destroyers that sailed the same day or the next. It had an oiler and seaplanes from late August, and no stated store of provisions.

So the record supports runs that carry what was loaded at Rabaul or Truk and staged through the Shortlands. It does not support a Shortland stock of any size. The rations a run carried are given only as about two weeks for the men it landed.

On the Allied side, Espiritu Santo was a staging point. Some cargo came to Guadalcanal in ships loaded at Noumea, Fomalhaut among them, and Espiritu Santo itself was supplied from Noumea, partly by air. The record gives it no stock to set. What it gives are the loads of single runs: 120 t, 2,000 t, 1,500 t, and the 7th Marines' convoy.
