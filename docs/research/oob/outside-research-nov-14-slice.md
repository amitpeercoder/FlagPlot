# Outside answers of 14 November 2026, second set, checked against their sources

Status: research report for the architect, read on 2026-11-14. Nothing here is settled. Every new row is `unconfirmed`; `oob-check` asked for no other standing.

Later on 14 November 2026 an outside research pass answered five more prompts: the Japanese forces west of the airfield, positions of places, the engineers and the airfield, the Kawaguchi Detachment by day, and Japanese rations. As with the first set (`outside-research-nov-14.md`), each answer is a table of statements with a source, a page and a quoted sentence. A statement is a lead, not a source. The answer files stay outside the repository.

## How it was read

The method is the first set's. One reader (a Sonnet agent) took each answer file. For every statement the reader fetched the cited document with `curl`, converted it to text (`pdftotext`, the standard library for HTML, OCR for scans), searched for the quoted words, read the passage around them, and said whether the document says what the statement says. Where a host refused, the reader tried the HyperWar copy or the Wayback Machine copy of the same document. Rows were staged only from the document's own words; the readers could also stage statements found on the same pages ("Found beside a lead").

**The quote check.** Every staged `as_stated` was checked by program against the reader's saved text, allowing only whitespace, typography and soft hyphens at a line break. All 178 staged rows passed (two needed the soft-hyphen allowance). The 22 rows from the gazetteer, which has no text layer, were checked against the lead's transcription of the page images (below).

**What the lead read himself.** For the points the architect asked about first, the lead went to the documents, not only the readers' reports:

- **The 1974 gazetteer.** The reader could not open it: the cia.gov address serves an HTML page to scripts. The lead fetched the Wayback Machine copy of the same address (217 pages, image only), rendered every page at 300 dpi, ran OCR, found each entry, and read each line used here on the page image. The 33 lines are transcribed with their PDF pages in the lead's working file; the OCR alone misreads many digits, so no figure rests on it.
- **Zimmerman, printed p. 77.** The lead read the page image of the Wayback copy of the usmcu.edu PDF: it prints "about 2200 men" beyond Kokumbona.
- **The two NIDS papers by Saito.** The lead read the passages on the Oka group, the landing of 29 August, the Kawaguchi movement and the hours (2013 paper pp. 82-83; 2024 paper pp. 114-117) in the saved text.

**What only a reader checked.** Everything else, including the translated Japanese order of 17 August (Captured Document No. 17), which a reader read from the scan's image and transcribed; the McCarthy, Hoffman, Hough, Miller and Zimmerman passages on the attacks of 12 to 14 September; the Donnelly and McMillan scans; and the handbook TM-E 30-480.

**What the check does not show.** It shows the words are in the document. It does not show that a figure was filed against the right unit or date: that filing is the readers', looked over by the lead. Each note says what the passage around the quote says.

## Counts

"Verified" means the document says what the statement says; "corrected" means it does, but a page, author, title or qualification in the statement was wrong. "Already held" means the same source was already a row.

| File | Statements | Verified | Corrected | Already held | Not found | Not reachable | Rows added (statements and beside) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 22 Western forces | 22 | 11 | 3 | 8 | 0 | 0 | 22 and 22 |
| 23 Positions | 25 | 17 | 3 | 1 | 1 | 3 | 18 and 14 |
| 24 Engineers | 27 | 19 | 1 | 7 | 0 | 0 | 20 and 12 |
| 25 Kawaguchi timeline | 17 | 7 | 0 | 10 | 0 | 0 | 8 and 51 |
| 26 Japanese rations | 9 | 6 | 1 | 2 | 0 | 0 | 10 and 23 |
| **All** | **100** | **60** | **8** | **28** | **1** | **3** | **78 and 122** |

Of the 68 statements verified or corrected, 67 are keyed. The one not keyed is a distance, not a position: *Fortitudine* (Fall 1992, p. 12) puts Red Beach "five miles east of Lunga Point".

**Not keyed, and why:**

- **Not reachable (3):** three Mapcarta pages (Tanambogo, Koilotumuria, Tassafaronga Point) answered 403 to every reader. They are modern map pages and the gazetteer gives all three places.
- **Not found (1):** the GeoNames search listing cited for Maravovo opens but has no Maravovo on it. The gazetteer gives Maravovo.

**What the corrections are.** No quote was invented and no document says the opposite of the statement that cites it.

- Saito's Oka passage is on p. 83 of the 2013 paper, not p. 82 (the 450 and 300 of 29 August are on p. 82).
- The "SeamLines" newsletter is in fact *River Cities News* (Marine Corps League Detachment 1090), November 2018, a club newsletter whose text reads like Wikipedia's.
- Miller's 500 Yokosuka men are on p. 95, not 96 (already held).
- The Taivu Point light is at 160°20'.9 E, which is 160.34833, not 160.34850.
- The Metapona "mouth" is a sampling point of a 2016 survey, on its Table 2 (p. 10), with no datum.
- *The Navy's Air War* is by the Navy's Aviation History Unit; the author the statement named is not on the page.
- The interrogation of Lt. Col. Iwakoshi is USSBS Interrogation No. 500 inside a 90-page bundle, IPS Doc. No. 1803, whose cover names Hoshino. It is cited as that bundle.
- *Fortitudine* is Vol. XXII No. 2, Fall 1992, p. 12.

**What the answers called not found but their sources hold.** The gazetteer the answer opened holds positions it did not give: Red Beach (a variant of Tenaru Beach), the Metapona River's mouth (a variant of Matepono River), the populated place Nggurambusu (Gurabusu), Lungga Point, Point Cruz and Aola Bay. The answer also missed the gazetteer's own cross-references that tie Kamimbo, Gavutu, Kokumbona and Tassafaronga to its standard names.

## Where the rows went

| File | Ids | What |
| --- | --- | --- |
| `sources.csv` | 1250 to 1266 | 17 new sources: the BGN gazetteer of 1974; Saito's two NIDS papers (2013, 2024); Captured Document No. 17 (AWM); an Avonmore sample page; *River Cities News* (C); Battlefield Travels (C); Lloyd's Maritime Atlas, 23rd edition; Australian Notices to Mariners 2003, edition 17; IndexMundi's reprint of NIMA names (C); the 2016 Gold Ridge baseline survey; McMillan, *The Old Breed*; Donnelly, *A Brief History of U.S. Marine Engineers*; Shindo (NIDS 2019); Nafziger's order of battle (C); Inui's diary translation (C); IPS Doc. No. 1803 |
| `places.csv` | 1054 to 1066 | kamimbo, gavutu, tanambogo, kukum, halavo, haleta, tetere, metapona-river, maravovo, visale, koilotumaria, gurabusu, red-beach |
| `land_units.csv` | 1096, 1097 | The Takahashi detachment of the Yokosuka 5th SNLF; the 3d Battalion, 4th Infantry |
| `lift_entities.csv` | 1075 to 1077 | The Oka barge convoy from 2 September; the two NIDS destroyer runs to Taivu of 29 August |
| `place_positions.csv` | 62 to 94 | 33 positions: 22 from the gazetteer, 6 from Lloyd's, 1 light from the Notices to Mariners, 2 from IndexMundi, 1 survey point, 1 from Wikipedia |
| `land_unit_facts.csv` | 916 to 1054 | 139 rows: the Oka group and the west, 29 August, the engineers, the Kawaguchi Detachment by day, the Ichiki first echelon's hours, rations and supply |
| `troop_lift_facts.csv` | 170 to 182 | 13 rows: the barge convoy, the two Taivu runs, 110 men a destroyer, the destroyers' supply runs |
| `place_facts.csv` | 202 to 216 | 15 rows on Henderson Field: forecasts, state by date, captured equipment |
| `gaps.csv` | last 10 lines | What was looked for and not found |

No new attribute was needed. Two values were reshaped to fit the existing ones: a date with an hour keeps the date in `value` and the hour in the note, and two marches the diary gives in kilometres are notes, not `march_distance_miles` (no conversion was keyed).

## The six points the architect asked about

### 1. The Oka group's men, by date and place

| Row | Source | What it says |
| --- | --- | --- |
| `land_unit_facts` 916 | Saito 2013, p. 83 | "some 1,000 men of the 2nd Battalion", under Oka, trying to reach Guadalcanal "on small ships and barges"; undated |
| 917, 918 | Saito 2013, p. 83 | By the time they reached a point near Malbobo, "some 50 kilometers west of Lunga Point", Oka had one-third of his original force left under his command (air attack and weather). No count, no date, and no deaths stated |
| `troop_lift_facts` 171, 172; `land_unit_facts` 922 | Wikipedia (low) | 1,100 men and heavy equipment in 61 barges left northern Santa Isabel on 2 September |
| 928, 923 | Wikipedia (low) | About 90 killed in the barges on 4 to 5 September; the article's own note gives 400 from another book |
| 929, 930 | Wikipedia (low) | Most of the remaining 1,000 landed near Kamimbo "over the next few days" |
| 931 | Wikipedia (low) | 1,000 west of the perimeter by 7 September |
| 944, 945 | Zimmerman, printed p. 77 | About 2,200 men beyond Kokumbona by 6 September: parts of the 124th "being assembled by Colonel Oka", naval landing force men and the two construction units |
| 925 | Wikipedia (low) | About 650 of Oka's men attacked on 14 September |
| 927 | Wikipedia (low) | 120 lost in Oka's force |

**A dispute to rule on.** `land_unit_facts` 86 holds "about 200 beyond Kokumbona" from the HyperWar transcription of Zimmerman. The printed page reads 2200, on the page image and in the PDF's own text layer. Row 944 holds the printed figure. Both rows stand; neither is changed.

**Where Oka landed** has four answers, each a row: Kokumbona (Hough and Miller, held as 321 and 322), near Kamimbo (Wikipedia 929; Battlefield Travels 953), near Malbobo (Saito 918), and the mouth of the Matanikau (McCarthy p. 154, row 950). No source read gives a count landed on a date.

**Oka's attack.** Saito 2013 quotes Kawaguchi's plan of 6 September: Oka to attack along the coast toward the west side of the airfield, with the detachment attacking at 1600 on 12 September (919). The hours of the attack:

- Miller (1007) and McCarthy (949): the afternoon of 14 September.
- Wikipedia (926): about 0400 on 14 September, two companies against the 3d Battalion, 5th Marines.
- Battlefield Travels (952): moving on the evening of 13 September.

Zimmerman (947) has the attack on the 3d Battalion, 5th Marines "repulsed" by artillery, with no hour and no Japanese unit named.

### 2. The landing of 29 August: Taivu or Tassafaronga

| Rows | Source | Place | Men |
| --- | --- | --- | --- |
| held `land_unit_facts` 107, `troop_lift_facts` 45 and 148 | Zimmerman pp. 76-77 | Tassafaronga | 900, four destroyers (Fubuki, Shirayuki, Kawakaze, Umikaze) |
| held `troop_lift_facts` 149 | Hough p. 300 | (not stated in the row) | Ichiki's rear echelon on two destroyer transports |
| new `land_unit_facts` 939, 940; `troop_lift_facts` 173 | Saito 2013 p. 82 | Taivu Point | 450 men, the main body of the 1st Battalion, 124th Infantry, by the 11th Destroyer Squadron, "during the night of August 29" |
| new `land_unit_facts` 941; `troop_lift_facts` 174 | Saito 2013 p. 82 | "also at Taivu" | 300 men of the Ichiki second echelon with four antitank guns, by the 24th Destroyer Squadron, after that landing |

Saito's two figures make 750, against Zimmerman's 900, and the two accounts disagree on the place. They are kept as separate runs (`jp-run-1942-08-29-desron11-taivu`, `jp-run-1942-08-29-desron24-taivu`) beside Zimmerman's `jp-run-1942-08-29-tassafaronga-destroyers`, because nothing read shows they are the same landing.

On the next night the sources differ again. The Yudachi record (held) puts 130 Ichiki men at Cape Taivu on 30 August. Zimmerman (held 872) puts the rest of the Ichiki unit ashore at Tassafaronga. The diary translation (977, grade C) has its unit land at 2130 on 30 August near Taivu Point.

### 3. Positions and their hexes

The map's projection is from the header of `data/map/south_pacific.csv`:

- east = (lon − 160) × 60 × 0.978148 nm;
- north = (lat + 12) × 60 nm;
- hex (q, r) is centred at east 40 (q + r/2), south 34.641016 r.

The nearest centre wins. As a check, the held GeoNames positions give the same hexes as the first set's table.

**The gazetteer** (BGN, May 1974) gives whole minutes, "for finding purposes only", and states no datum. By its foreword, points are read at the extremity, streams at the mouth, populated places at the map symbol and other features near the centre. Its own cross-references name the 1942 spellings: "KAMIMBO BAY SEE KOMIMBO BAY", "GAVUTU ISLAND SEE GHAVUTU ISLAND", "KOKUMBONA SEE KAKAMBONA", "TASSAFARONGA POINT SEE TASIVARONGO POINT", "RED BEACH SEE TENARU BEACH" and "LUNGA POINT SEE LUNGGA POINT". It has no entry spelled Tasimboko or Gurabusu.

| Place | Row | Source, feature | Position | Hex | To centre | Next nearest |
| --- | --- | --- | --- | --- | --- | --- |
| Kamimbo | 73 | BGN, Komimbo Bay | -9.25000, 159.66667 | **(2,-5)** | 8.2 nm | (2,-4) at 32.9 nm |
| Kamimbo | 62 | Wikipedia coordinate (low) | -9.25889, 159.67167 | (2,-5) | 8.8 nm | (2,-4) at 32.3 nm |
| Gavutu | 74 | BGN, Ghavutu Island | -9.11667, 160.18333 | **(3,-5)** | 9.2 nm | (2,-5) at 30.8 nm |
| Gavutu | 68 | Lloyd's | -9.11667, 160.20000 | (3,-5) | 8.3 nm | |
| Tanambogo | 75 | BGN, Tanambogho Island | -9.10000, 160.18333 | **(3,-5)** | 9.3 nm | (2,-5) at 30.8 nm |
| Kukum | 76 | BGN, Kukum (PPLX) | -9.43333, 159.98333 | **(2,-4)** | 15.5 nm | (2,-5) at 27.0 nm |
| Kukum | 67 | Lloyd's, "Kokum Beach" | -9.41667, 160.01667 | (2,-4) | 16.5 nm | (3,-5) at 26.3 nm |
| Kokumbona | 86 | BGN, Kakambona | -9.41667, 159.91667 | (2,-4) | 17.1 nm | (2,-5) at 23.7 nm |
| Kokumbona | 66 | Lloyd's, Kokumbona Beach | -9.41667, 159.91667 | (2,-4) | 17.1 nm | (2,-5) at 23.7 nm |
| Tassafaronga | 87 | BGN, Tasivarongo Point | -9.36667, 159.86667 | (2,-5) | 19.5 nm | (2,-4) at 21.0 nm |
| Taivu Point | 88 | BGN, the point | -9.40000, 160.35000 | (3,-5) | 17.2 nm | (3,-4) at 26.1 nm |
| Taivu Point | 72 | Notices to Mariners, the light (2003) | -9.40500, 160.34833 | (3,-5) | 17.5 nm | (3,-4) at 26.0 nm |
| Tasimboko | 89 | BGN, Tadhimboko (area) | -9.43333, 160.30000 | (3,-5) | 19.4 nm | (2,-4) at 23.4 nm |
| Halavo | 77, 64 | BGN; IndexMundi | -9.10000, 160.20000 | (3,-5) | 8.3 nm | (2,-5) at 31.7 nm |
| Haleta (Florida) | 78, 63 | BGN; IndexMundi | -9.08333, 160.11667 | (3,-5) | 13.3 nm | (2,-5) at 26.9 nm |
| Tetere | 79 | BGN, area | -9.43333, 160.21667 | (2,-4) | 20.0 nm | **(3,-5) at 20.5 nm** |
| Tetere | 69 | Lloyd's | -9.41667, 160.23333 | **(3,-5)** | 19.3 nm | (2,-4) at 21.4 nm |
| Metapona mouth | 80 | BGN (variant of Matepono River) | -9.40000, 160.20000 | (3,-5) | 19.1 nm | (2,-4) at 21.0 nm |
| Metapona diversion mouth | 65 | 2016 survey point | -9.42342, 160.20842 | (2,-4) | 20.2 nm | (3,-5) at 20.2 nm |
| Maravovo | 81, 82 | BGN, village and mission | -9.28333, 159.63333 and 159.61667 | (2,-5) | 10.3 and 10.5 nm | (1,-4) at about 30 nm |
| Visale | 83 | BGN, mission | -9.25000, 159.70000 | (2,-5) | 8.5 nm | (2,-4) at 31.8 nm |
| Koilotumaria | 84 | BGN, Koilotumuria | -9.46667, 160.43333 | (3,-4) | 19.8 nm | (3,-5) at 21.9 nm |
| Gurabusu | 85 | BGN, Nggurambusu | -9.48333, 160.46667 | (3,-4) | 17.7 nm | (3,-5) at 23.4 nm |
| Red Beach | 90 | BGN, Tenaru Beach | -9.43333, 160.10000 | (2,-4) | 16.5 nm | (3,-5) at 23.8 nm |
| Tenaru River mouth | 91 | BGN | -9.43333, 160.10000 | (2,-4) | 16.5 nm | |
| Lunga Point | 92 | BGN, Lungga Point | -9.40000, 160.03333 | (2,-4) | 17.5 nm | (3,-5) at 24.9 nm |
| Point Cruz | 93 | BGN | -9.41667, 159.95000 | (2,-4) | 16.7 nm | (2,-5) at 25.0 nm |
| Aola Bay | 94 | BGN | -9.51667, 160.48333 | (3,-4) | 15.6 nm | (3,-5) at 25.6 nm |
| Koli Point | 70 | Lloyd's | -9.40000, 160.15000 | (2,-4) | 19.5 nm | (3,-5) at 20.5 nm |
| Tulagi | 71 | Lloyd's | -9.08333, 160.16667 | (3,-5) | 10.4 nm | |

**Does a second source move a place to another hex?** No.

- **Kokumbona** stays in (2,-4). The gazetteer and Lloyd's give the same minute as the held GeoNames row 60. The gazetteer's "KOKUMBONA SEE KAKAMBONA" settles the identity that row 60 left open.
- **Tassafaronga** stays in (2,-5), within a minute of row 51 and still close to the border (21.0 nm to (2,-4)).
- **Taivu Point** stays in (3,-5), both for the point and for the light.
- **Tasimboko** stays in (3,-5).

GeoNames repeats the gazetteer's figures for Kokumbona, Taivu Point and Tasimboko exactly, and very likely descends from it. Those agreements are therefore not independent.

**Other points:**

- **Kamimbo** is in (2,-5), the Cape Esperance hex, well inside it.
- **Gavutu and Tanambogo** are in (3,-5), the Tulagi hex.
- **Kukum** is in (2,-4), the airfield's hex.
- **Tetere and the Metapona mouth** lie almost on the border between (2,-4) and (3,-5): the gazetteer's Tetere falls in (2,-4) by half a nautical mile, and Lloyd's in (3,-5).
- **Koilotumaria and Gurabusu**, east of Koli Point, are in (3,-4) with Aola Bay.

### 4. The 1st Engineer Battalion's men landed on Guadalcanal

No document read gives a count: not landed, embarked, authorised, or by company. Rows keyed:

- **Hough p. 261** (955, 956): the battalion "less detachments" on Guadalcanal when the transports left, and the 2d Platoon, Company A on Tulagi with the 2d Engineer Battalion.
- **Hough App. C** (954): Frazer commanded to 24 October. Held row 379 (Zimmerman) says to 25 October.
- **Donnelly** (962): the battalion "less Company B" was in the landing, and Company B joined on 18 September. McMillan has Company B with the Raiders before that.
- **Donnelly** (963): a table of organization for the engineer regiment, not the battalion.
- **Zimmerman** (957, 968, 969): no power shovels or dump trucks landed; Fomalhaut left with about 15 percent of her cargo discharged, carrying most of the earth-moving equipment; C Company wired in the 2d Battalion, 1st Marines on 21 to 22 August.
- **Zimmerman** (958): the pioneers' one bulldozer.
- **Work on the field:**
  - started 9 August (959);
  - 2,600 feet completed on 12 August (961, McMillan);
  - captured equipment with counts (place_facts 208, Hough: nine rollers, six working, two locomotives with hopper cars, and more);
  - the gap of 180 to 200 feet (209, Hough), beside Miller's 196 feet (held).
- **Runway forecasts of 8 August:** one week (206, Zimmerman) or about two weeks (207, Hough) for the last 1,178 feet.
- **Runway state by date:**
  - 2,800 feet operable on 12 August (210, *Navy's Air War*);
  - nearly 4,000 feet within six days (211);
  - completed 18 August (212, Donnelly, quoting Hough) or 19 August (213, Zimmerman). The held rows give 17 August (Miller) and 18 August (Hough).
- **The Japanese units:** Nafziger (grade C) gives the 11th and 13th Construction Units 1,350 and 1,221 men (964, 965), against about 900 each in held rows 1 and 4. Moore (ANU) names the commanders (967) and gives a runway of 1,151 m on 7 August (place_facts 216), which is the length after the American work in other rows.

### 5. Kawaguchi's departure from Taivu, and the hours of the attacks

**Departure of the main body.** No document read gives a day or hour, by column. What the rows give:

- **The trail:** begun about 2 September (Hough 1016, a second source for held row 858).
- **8 September:** the main body near Taivu Point (Miller, held 868), or already advancing inland (Hoffman 999).
- **12 September:** a bivouac on the upper Lunga and the move north to assault (Zimmerman 1011).
- **The coast column, in Inui's diary (grade C, low):**
  - two battalions (Kokusho to Tetere, Watanabe to the Mbalisuna) moved at 2000 on 1 September (980);
  - the diarist's antitank company, with the Kuma battalion, set out at dusk on 5 September (982) and at 1630 on 8 September (984);
  - the attack day was set for 12 September (985), then put back a day because the main body had not reached its start line (987);
  - his company began the retreat at 1400 on 14 September (989).

**Hours of the attacks.**

| Night | Hour | Source, row |
| --- | --- | --- |
| Plan of 6 September | 1600 on 12 September | Saito 2013, as Kawaguchi's plan (919) |
| 12 to 13 September | "night of 12 September", no hour | Japanese Monograph 45 (996) |
| 12 to 13 September | about 2100, ship shelling and a light probe | Hough (1017) |
| 12 to 13 September | 2200 flares, then 20 minutes of ship fire, then piecemeal attacks | Hoffman (1001) |
| 13 September | 1830 first blow; 2130 the main effort down the ridge | Zimmerman (1012, 1013) |
| 13 September | 2000 | Saito 2024, p. 117 (997); the 2013 paper says the same |
| 13 September | about 2100 flare, a two-battalion attack | Hough (1018) |
| 13 September | massing by 2100, assault at 2200 | Hoffman (1002) |
| 13 September | "shortly after nightfall"; artillery from 2100 | Miller (1004) |
| 13 to 14 September | about midnight, the flank attack on the 3d Battalion, 1st Marines | Miller (1006), Zimmerman (1014: the 1st Battalion, 124th Infantry), Hough (1019) |
| 14 September | afternoon, the west sector, probably Oka | Miller (1007), McCarthy (949) |

**The Ichiki first echelon:**

- **Landing:** "around 9:00 p.m." on 18 August at Taivu Point (Saito 2024, 1049). This is the only clock hour for the landing. The diary has a translator's note of 2300 on 16 August (1028), against every other source.
- **Setting out:** it "marched westward at once" (JM-45, 994).
- **Approach:** detected east of the Ilu in the night of 20 to 21 August (1020); Zimmerman's footnote has the first flare at 2030 on 20 August (1025).
- **Forming up:** on the point on the east bank before the attack (1023).
- **Assault:** 0310 (Hough 1022, Zimmerman 1024, beside Miller held 857) against 0130 (Shaw 1027). JM-45 has "the night of the 20th" (995).

### 6. Japanese rations, days issued, tons landed, and captured stocks

**A ration scale.** The only scale is the War Department's handbook TM-E 30-480 of 1944: a US description of the Japanese Army in general, not of Guadalcanal in 1942. It is filed on `jp-forces-guadalcanal` at low confidence and says so.

- **Scales by weight (1030 to 1033):**
  - special field ration 3 pounds (the handbook's own table gives 40 or 45 oz);
  - standard about 4⅛ pounds;
  - reserve class A 2¼ pounds and class B 1¾ pounds;
  - iron ration about half a pound a meal.
- **In active theatres (1034):** about 3½ pounds, often cut to a half or a third.
- **Rice (1035):** 23.3, 20.5 and 30.7 oz by scale, the column split being the reader's reading of a flattened table.
- **Against the American ration (1037):** about two-thirds.

**Days issued.**

- **Unit, rule and doctrine:**
  - 2 days on the man and 3 in the unit, with 2 more at division (Iwakoshi, USSBS Interrogation 500, 1044);
  - infantry in the hills sometimes 20 days, "especially true on Guadalcanal" (1045);
  - expeditionary units left Japan with 30 days, and advance troops mostly a week (Interrogation 501, 1048).
- **The Kawaguchi movement:** "two weeks' rations" were carried with about 5,600 Army and Navy men, 31 August to 7 September (Saito 2024: 975; `troop_lift_facts` 176).
- **One destroyer:** a planning figure of 110 men, 0.3 guns and two weeks' rations (`troop_lift_facts` 170, from the 2013 paper; 177 and 178, from the 2024 paper).
- **An unnamed unit, 12 September:** three days (held 867). The bulletin's survivors faced "a food shortage" (1039), the men were put on reduced rations (1040), and carriers took 2 or 3 days for a round trip (1041). None is dated beyond that.

**Tons landed.** None for August or September. Saito 2024 gives the following, all outside the window:

- about 240 tons on one transport of the October convoy, an estimate (`troop_lift_facts` 182);
- milled rice per man of 349 g in October falling to 203 g in December (1051);
- campaign averages of the supplies that reached the delivery points (1052).

**Captured stocks.** No quantity anywhere. Miller says the rice was in 100-pound bags in the dumps (1053). Hough says the stacks of captured rations grew in importance after 9 August (1054). The held rows already list what was found (184, 185) and the Marines' inventory, which counted ten days of Japanese rations (419). Nothing new for Tulagi.

## Conflicts between sources

- **Men beyond Kokumbona on 6 September:** about 2,200 (Zimmerman printed, 944) against about 200 (the same work as transcribed, held 86).
- **Where Oka landed:** Kokumbona, Kamimbo, Malbobo, or the mouth of the Matanikau.
- **Losses in the barges:** 90 or 400, both in Wikipedia's account.
- **When Oka attacked:** the evening of 13 September, 0400 on 14 September, or the afternoon of 14 September.
- **29 August:** Tassafaronga with 900 (Zimmerman) against Taivu with 450 and 300 (Saito).
- **The Yokosuka 5th:**
  - 500 at Kokumbona (Miller, held);
  - about 100 embarked on 15 August (Captured Document No. 17, row 932), which does not say they landed;
  - 113 landed on the second night (the Avonmore page, 936), with 197 Sasebo men the night before (937).
- **The ridge, 13 September:** 1830, 2000, about 2100 or 2200 for the opening, and JM-45 puts the attack on the 12th.
- **The Ichiki assault:** 0310 or 0130.
- **The Kawaguchi total:** about 5,600 Army and Navy (Saito 2024) against over 6,000 (Miller, Hough) and about 5,200 (MacArthur's Reports).
- **Men a destroyer:** 110 (Saito, a planning figure) against 143 to 155 on 31 August (combinedfleet, held 154) and 150 (a Marine estimate, held 27).
- **The airfield:**
  - ready on 17, 18 or 19 August;
  - the forecast for the last 1,178 feet was one week or two;
  - on 12 August 2,600 or 2,800 feet;
  - the gap was 196 feet or 180 to 200 feet.
- **The Japanese construction units:** 1,350 and 1,221 (Nafziger, C) against about 900 each.
- **Frazer's last day:** 24 or 25 October.
- **The handbook's special ration:** 3 pounds in its text and 40 or 45 oz in its table.

## What is still missing

Ten lines were added to `gaps.csv`:

- the 1st Engineer Battalion's men by company;
- each Kawaguchi column's departure from Taivu and the order that set it;
- the Oka group's men embarked, landed by date and place, and present at the attack;
- a 1942 Japanese ration scale in grams, days issued to each echelon, and ammunition per weapon;
- tons landed by run to 14 September, a destroyer's cargo limit and the 17th Army's daily need;
- quantities of the stocks captured at Lunga, Kukum, Tulagi and Gavutu;
- the Ichiki first echelon's hour of setting out, its halt and its arrival at the Ilu;
- the runway's usable length on 9, 15, 17 and 20 August;
- the Korean and Japanese men in each construction unit and their fate;
- the ends of Red Beach.

No gap line was closed. The first set's gap on Ichiki's march gains a landing hour, but its distance and rations are still unstated. Its gap on the Kawaguchi march gains no day count. Neither line is changed.
