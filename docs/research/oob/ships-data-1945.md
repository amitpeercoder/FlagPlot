# Ships' Data 1945: the transports of the landings

Status: research report for the architect, read on 2026-11-19. Nothing here is a ruling. It reads the one source decision 0166 asked for ("What still cannot be derived"): the Navy's own data book of 1945, for the transports, cargo ships and destroyer-transports of 7 August 1942. No scenario, derivation list, reference file, rule or code was changed.

## The book and how it was read

Navy Department, Bureau of Ships, *Ships' Data, U. S. Naval Vessels*, volume III, *Auxiliary, District Craft and Unclassified Vessels* (NAVSHIPS 250-012), dated 15 April 1945, printed 1946. A Google Books scan of 777 pages, hosted at https://shipscribe.com/pubs/SDB1945v3.pdf. It was downloaded once with the designer's leave and is not committed, nor are its text or page images. Source key `ships-data-1945-v3` (sources.csv 1492), graded A, kind primary.

Each type has five tables on facing pages, one line per ship, keyed by a line number: (1) name, former designations and names, length, beam, trial speed, trial displacement; (2) limiting displacement, tons per inch, limiting draft, accommodations for the ship's company, troop capacity (APA and APD only), flag accommodations, largest boom; (3) engines, shaft horsepower, boilers; (4) gears, generators and fuel capacities; (5) the battery, cargo capacity by volume, deadweight capacity; and a sixth of builders and dates. Ships appear under their 1945 designations (Crescent City as APA21, "ex-AP40").

**Every figure filed was read from the page image, not from the text layer.** Each page used was rendered at 150 dpi (three at 400 or 600 dpi where a figure was faint) and the ship's line read across all the tables. 221 rows were filed; every one was taken from the image. The text layer was used only to find pages. It was wrong or silent on at least 14 of the figures filed: a footnote mark run into seven designed speeds ("117.5", "115.0", "1 17.0", "1 16.0" twice, "1 15.0", "1 18.0" for 17.5, 15.0, 17.0, 16.0, 15.0, 16.0, 18.0); Alhena's cargo space ("410 , 1975, 765" for 410,197 and 5,765); President Adams's deadweight ("0, 900" for 6,900); Barnett's 9,570 barrels placed under fuel oil when the page prints it under diesel oil; Heywood's limiting displacement and officers' berths, missing from her line; Bellatrix's 3-inch guns run into the 40 mm column. Elsewhere it split a ship's figures onto a line apart from her name (Fomalhaut's limiting table), which is why the image, not the text, decides which ship a figure belongs to.

Things on the pages worth knowing:
- Neville's line is printed 193 on the dimensions page and 183 on the facing pages; it is 183.
- In this copy a pen line is drawn through the limiting displacements of lines 1 to 4 of the AKA table (Alchiba is line 3); the figure is legible and filed as printed, with a note.
- Heywood's 16.8 knots carries a mark that the scan shows only as a dot. The page's footnotes are "Designed speed" and "Maximum speed full load as reported by ship"; no other line on the page uses the second, so it is probably that one. Filed at medium, with the reading in the note.
- On the APA pages for Heywood and Hunter Liggett the berth column is headed "Troop accommodations", but its footnote says "Ship's company", as on every other APA page: a misprinted header.

## Answers for the architect

**(a) Standard displacement.** The book defines it (definition 5, printed p. IX, PDF p. 14) in the words of the London Treaty of 1936: the ship complete, fully manned, engined and equipped ready for sea, with armament, ammunition, stores and fresh water for the crew, "but without fuel or reserve feed water on board", for the ship as designed. **But the tables for these types have no standard displacement column.** For the AP, APA, AK, AKA and APD tables the only displacements are the *trial displacement* (definition 23: "the average displacement of the ship during the standardization trials"; printed only where trials were run) and the *limiting displacement* (the displacement at the limiting draft, which definition 7 calls the draft set "as a guide to operating personnel against overloading"). Neither is light or standard: the trial displacement of a C2 cargo hull is 13,910 tons against a limiting 14,225, so it is near loaded. The words "light displacement" appear nowhere in the volume. Definition 6 defines full load, but these tables give none.

**(b) Radius or endurance.** None for any of these ships, and none in the tables of any auxiliary type. A search of the whole text for radius, endurance, cruising and miles finds one: a remark on a district patrol craft, YP87, "Cruising radius, 6,750 miles" (printed p. 562). The machinery tables give shaft horsepower, boilers, steam conditions, gears, generators and fuel capacities, but no speed and range.

**(c) Fuel in tons, or a weight per barrel.** Neither. Fuel is given only as fuel oil and diesel oil in barrels and gasoline in gallons. Definitions 14 and 15 (printed p. X, PDF p. 15): fuel oil capacity "is given as 95 % of the total capacity of all tanks on the vessel which carry fuel oil", including tanks marked "fuel oil or ballast"; diesel oil the same. No page states the weight of a barrel or a gallon of fuel oil. The oilers' and fuel barges' tables give cargo in barrels beside deadweight in tons, but state no factor between them, and dividing one by the other would be an inference, not a statement. So decision 0166's rule 8 still has no factor to convert a barrel to long tons.

## What was filed

| File | Rows | Ids |
| --- | --- | --- |
| `ship_facts.csv` | 221 | 3100 to 3320 |
| `sources.csv` | 1 | 1492 `ships-data-1945-v3` |
| `attributes.csv` | 9 | all on `ship`: `hull_designation`, `displacement_limiting_lt`, `trial_displacement_lt`, `deadweight_at_limiting_lt`, `cargo_capacity_cuft`, `cargo_refrigerated_cuft`, `troop_capacity_officers`, `troop_capacity_enlisted`, `accommodations_note` |
| `gaps.csv` | 64 | the last 64 entries: three per ship present (light, standard and full-load displacement; endurance; fuel in tons) and one per ship absent |

The new attributes were needed because none on file fits: `displacement_limiting_lt` and the trial displacement existed only for a class (the class trial displacement is the Japanese two-thirds condition, a different measure); the book's deadweight is at the Navy's limiting draft, not the merchant load line that `deadweight_lt` holds, and filing it there would have made false disputes with the merchant figures on file; the book prints troops as officers and enlisted men in two columns and no total, so the total is not filed (`troop_capacity_men` is left alone).

**Dates.** Every row is dated 1945-04-15 to 1945-04-15 ("true on this date, limits unknown"), except the seven speeds the book footnotes as designed, which are figures of the hull as designed and are left undated. Trial speeds are dated to the book, since the trial date is not given and definition 20 says they may come from another ship of the class. Nothing else is dated to 1942 on this book's word. The cargo, troops, fuel and battery are those of the APA, AKA or APD of 1945; the AP or AK of August 1942 may differ.

**Standings of rows on file changed** (standing only; a sentence saying why is appended to each note), as `oob-check` requires: to `disputed` because a new row gives another value over the same date: ship_facts 2817 (Alchiba cargo 3,705 against the book's 4,705), 2757, 2758, 2759 (the President Jackson class's cargo 6,800 DWT against the book's 3,500, 3,600 and 2,500; 6,800 is the book's *total* deadweight for President Jackson, so NavSource's figure may be the total), 2607 (Stringham 35.2 knots, undated, against the APD's trial 27.7 in 1945; the two may describe the destroyer and the APD and need not contradict each other); to `preferred` because the book now agrees at high confidence: ship_facts 1168 (Neville cargo 1,800, Wikipedia). New rows of this round were set the same way: every new speed row is `disputed` (each ship's speed was already disputed) except Zeilin's 18, which agrees with the rows on file; the new cargo rows of Alchiba, Betelgeuse, Fuller and the three Presidents are `disputed`.

**NavSource copied this book.** NavSource's fuel figures in barrels, its cargo tons and its "full load" for the AKAs equal this book's fuel, cargo deadweight and limiting displacement, line for line. Its agreement is therefore not a second, independent source, and its "full load" for the cargo ships is the limiting displacement.

## The ships

"Not in the book" means the tables have no such column for the type, or the line is blank. Speeds: D = designed (footnote), T = trial. Row ids in brackets. All figures as of 15 April 1945 unless marked designed.

### Transports (APA in 1945)

| Ship (line) | Designation | Max speed | Trial displ. | Limiting displ. | Fuel (Bbls, 95 %) | Cargo, tons (deadweight at limiting) | Cargo space, cu ft | Troops (off./enl.) | Ship's company (off./enl.) | Battery, 1945 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| American Legion (4) | APA17, ex-AP35 (3100) | 17.5 D (3101) | not in the book | 21,900 (3102) | oil 25,720, diesel 375 (3106) | 2,500 of 8,400 total (3111, 3110) | 120,000; refrigerated none (3108, 3109) | 107 / 1,537 (3104, 3105) | 43 / 639 (3103) | 4 3"/50; 2 twin 40 mm, ultimate (3107) |
| Barnett (12) | APA5, ex-AP11 (3112) | 15.0 D (3113) | not in the book | 14,080 (3114) | diesel 9,570; fuel oil blank (3118) | 1,700 of 4,600 (3123, 3122) | 130,000; none (3120, 3121) | 88 / 1,207 (3116, 3117) | 44 / 447 (3115) | 4 3"/50; 2 twin (3119) |
| Crescent City (68) | APA21, ex-AP40 (3124) | 17.0 D (3125) | not in the book | 14,247 (3126) | oil 12,925, diesel 450 (3130) | 2,300 of 5,600 (3135, 3134) | 140,000; none (3132, 3133) | 58 / 1,012 (3128, 3129) | 36 / 498 (3127) | 4 3"/50; 2 twin, 2 single (3131) |
| Fuller (98) | APA7, ex-AP14 (3136) | 16.0 D (3137) | not in the book | 14,450 (3138) | oil 11,475, diesel 300 (3142) | 2,400 of 6,100 (3147, disputed; 3146) | 130,000; none (3144, 3145) | 60 / 898 (3140, 3141) | 43 / 337 (3139) | 4 3"/50; 2 twin (3143) |
| Heywood (126) | APA6, ex-AP12 (3148) | 16.8, mark unread (3149) | not in the book | 14,450 (3150) | oil 11,475, diesel 300 (3154) | 2,000 of 6,100 (3159, 3158) | 145,000; none (3156, 3157) | 75 / 1,203 (3152, 3153) | 46 / 504 (3151) | 4 3"/50; 4 twin (3155) |
| Hunter Liggett (153) | APA14, ex-AP27 (3160) | 15.0 D (3161) | not in the book | 21,900 (3162) | oil 25,720, diesel 375 (3167) | 1,900 of 7,700 (3172, 3171) | 140,000; none (3169, 3170) | 107 / 1,417 (3165, 3166) | 52 / 673; flag 12 / 33 (3163, 3164) | 4 3"/50; 2 twin, ultimate (3168) |
| Neville (183) | APA9, ex-AP16 (3173) | 16.0 D (3174) | not in the book | 14,450 (3175) | oil 11,475, diesel 300 (3179) | 1,800 of 5,800 (3184, 3183) | 145,000; none (3181, 3182) | 64 / 1,249 (3177, 3178) | 50 / 524 (3176) | 4 3"/50; 2 twin (3180) |
| President Adams (201) | APA19, ex-AP38 (3185) | 18.4 T (3186) | 11,760 (3187) | 16,000 (3188) | oil 10,950, diesel 300 (3192) | 3,600 of 6,900 (3197, disputed; 3196) | 185,000; none (3194, 3195) | 76 / 1,258 (3190, 3191) | 36 / 477 (3189) | 4 3"/50; 2 twin, 2 single (3193) |
| President Hayes (202) | APA20, ex-AP39 (3198) | 18.4 T (3199) | 11,760 (3200) | 16,000 (3201) | oil 10,950, diesel 300 (3205) | 2,500 of 5,800 (3210, disputed; 3209) | 185,000; none (3207, 3208) | 76 / 1,312 (3203, 3204) | 36 / 477 (3202) | 4 3"/50; 2 twin, 2 single (3206) |
| President Jackson (203) | APA18, ex-AP37 (3211) | 18.4 T (3212) | 11,760 (3213) | 16,000 (3214) | oil 10,950, diesel 300 (3219) | 3,500 of 6,800 (3224, disputed; 3223) | 185,000; none (3221, 3222) | 70 / 1,312 (3217, 3218) | 35 / 477; flag 8 officers (3215, 3216) | 4 3"/50; 2 twin, 2 single (3220) |
| Zeilin (235) | APA3, ex-AP9 (3225) | 18.0 D (3226) | not in the book | 21,900 (3227) | oil 29,675, diesel 375 (3232) | 2,000 of 8,600 (3237, 3236) | 190,000; none (3234, 3235) | 121 / 1,558 (3230, 3231) | 29 / 638; flag 43 / 108, fitted as flagship (3228, 3229) | 4 3"/50; 1 twin, 1 quad, ultimate (3233) |
| McCawley | listed only as "APA4 McCAWLEY (sunk)" | not in the book | | | | | | | | |
| George F. Elliott (AP13) | not in the book; the George F. Elliott of the AP tables is AP105, ex-Delbrasil, a later ship | not in the book | | | | | | | | |

### Cargo ships (AKA in 1945; Fomalhaut AK)

| Ship (line) | Designation | Max speed | Trial displ. | Limiting displ. | Fuel (Bbls, 95 %) | Cargo, tons | Cargo space, cu ft (refrig.) | Ship's company | Battery, 1945 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Alchiba (3) | AKA6, ex-AK23 (3238) | 16.5 T (3239) | 13,910 (3240) | 14,225, struck through in this copy (3241) | diesel 8,950 (3243) | 4,705 of 6,925 (3247, disputed; 3246) | 274,000; refrig. blank (3245) | 31 / 325 (3242) | 1 5"/38; 4 twin (3244) |
| Alhena (7) | AKA9, ex-AK26 (3248) | 16.6 T (3249) | 13,910 (3250) | 15,080 (3251) | oil 11,810, diesel 1,175 (3253) | 4,885 of 7,930 (3258, 3257) | 410,197 (5,765) (3255, 3256) | 75 / 371 (3252) | 1 5"/38; 4 twin (3254) |
| Bellatrix (17) | AKA3, ex-AK20 (3259) | 16.5 T (3260) | 13,910 (3261) | 14,225 (3262) | diesel 8,950 (3264) | 4,515 of 6,735 (3269, 3268) | 262,096 (36,180) (3266, 3267) | 43 / 326 (3263) | 1 5"/38, 4 3"/50; no 40 mm (3265) |
| Betelgeuse (18) | AKA11, ex-AK28 (3270) | 16.5 T (3271) | 13,910 (3272) | 14,225 (3273) | diesel 8,950 (3275) | 4,785 of 7,005 (3280, disputed; 3279) | 266,488 (28,300) (3277, 3278) | 34 / 524 (3274) | 1 5"/38; 4 twin (3276) |
| Libra (35) | AKA12, ex-AK53 (3281) | 16.5 T (3282) | 13,910 (3283) | 14,225 (3284) | oil 10,670 (3286) | 4,605 of 7,280 (3291, 3290) | 364,038 (23,800) (3288, 3289) | 49 / 445 (3285) | 1 5"/38; 4 twin (3287) |
| Fomalhaut (61) | AK22, ex-AKA5 (3292) | 14.8 T (3293) | 14,350 (3294) | 11,100 (3295) | diesel 9,345 (3297) | 4,500 of 6,400 (3302, 3301) | 253,622 (12,398) (3299, 3300) | 23 / 260 (3296) | 1 5"/38, 4 3"/50; 2 single 40 mm (3298) |

The AK and AKA tables have no troop column. Fomalhaut's limiting displacement (11,100) is printed below her trial displacement (14,350); both are filed as printed.

### Destroyer-transports (APD)

| Ship (line) | Designation | Max speed | Trial displ. | Limiting displ. | Fuel (Bbls, 95 %) | Troops (off./enl.) | Ship's company | Battery, 1945 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Manley (75) | APD1, ex-AG28, DD74 (3303) | 27.2 T (3304) | 1,600 (3305) | 1,600 (3306) | oil 2,380, diesel 55 (3310) | 3 / 144 (3308, 3309) | 8 / 98 (3307) | 3 3"/50; 2 single 40 mm, ultimate (3311) |
| Stringham (100) | APD6, ex-DD83 (3312) | 27.7 T (3313) | 1,660, faint (3314) | 1,600 (3315) | oil 2,380, diesel 55 (3319) | 3 / 144 (3317, 3318) | 8 / 98 (3316) | 3 3"/50, ultimate; 2 single (3320) |
| Colhoun, Gregory, Little, McKean | not in the book: listed "sunk" (Gregory, Little, McKean) or absent (Colhoun); the index's ships of these names are later destroyers in volume I | | | | | | | |
| McFarland | not in this volume: listed "AVD14 McFARLAND (now DD237)"; as a destroyer she is in volume I, not read | | | | | | | |

The APD tables give no cargo. Their troop capacity (3 officers, 144 enlisted) agrees with the 144 or 147 already on file for the class.

## Verdict against decision 0166

Decision 0166's rules ask, for a transport or cargo ship: the light displacement (rule 2), a stated cargo capacity by weight (rule 3), the maximum speed (rule 7), and fuel in tons with an endurance and its speed (rule 8).

- **Light displacement: not in the book, for any ship.** Its trial and limiting displacements are loaded conditions and do not stand in for it. Rule 2 still reads the light displacements on file (NavSource, shipscribe and navypedia rows of the last round).
- **Cargo by weight: in the book for all 17 transports and cargo ships present**, as cargo deadweight at limiting displacement, from a primary source, but as of 1945. For twelve it equals a row already on file (Zeilin, Barnett, Heywood, Hunter Liggett, American Legion, Neville, Bellatrix, Alhena, Fomalhaut, Betelgeuse, Fuller, and Crescent City's class row 2128); for Libra it is the first cargo figure on file (4,605); for Alchiba (4,705 against 3,705) and the three Presidents (2,500 to 3,600 against 6,800) it disagrees with NavSource and the groups are now disputed. Whether a 1945 APA's cargo can be read for the AP of 7 August 1942 is a ruling the book cannot make.
- **Maximum speed: better for the eleven transports**, whose designed speeds the book gives undated (17.5, 15.0, 17.0, 16.0, 15.0, 16.0, 18.0); the cargo ships, the Presidents and the APDs have only a trial speed of 1945 (possibly a sister's). Every ship's speed group but Zeilin's remains disputed, so rule 7's written exception is still needed for each.
- **Fuel: still blocked.** The book gives fuel only in barrels at 95 % of tank capacity, and no factor to tons (answer c). It confirms the barrel figures on file but adds nothing the derivation can read.
- **Endurance with speed: not in the book, for any ship** (answer b).

So for every ship of the landings the book improves cargo and speed but **not every figure can be read**: fuel in long tons and endurance with its speed are still missing for all of them, and light displacement still rests on the compilations. Part B of decision 0166 stays blocked on fuel and endurance. What would unblock it: a source that states the Navy's weight of a barrel of fuel oil (or gives these ships' fuel in tons), and any source with a radius or endurance at a stated speed for these hulls (the ships' own data books of 1942 or 1943, the Maritime Commission's design data for the C2 and C3 hulls, or the transports' war diaries).

## What was not verified

That any figure of 1945 held on 7 August 1942 (the conversions to APA and AKA came in 1943, and the book does not say which figures they changed); whether the book's "tons" are long tons (the Navy's practice, not stated in the book); which ship of each class ran the trials behind a trial speed or displacement; the reference mark on Heywood's speed; Alchiba's struck-through limiting displacement, which is legible but marked by a reader; whether NavSource copied the 1945 edition or the 1943 one.
