# The class facts of the ships of the 39 days

Status: research report for the architect, read on 2026-11-19. Nothing here is a ruling. It is step 1 of decision 0165's plan: the facts a ship class is derived from, for the American transports, cargo ships and destroyer-transports of 7 August 1942 and the Japanese destroyers of the runs of 16 August to 14 September. No scenario, derivation list, reference file, rule or code was changed.

## What the derivation needs

The list was taken from `oob-derive` itself (`crates/oob-derive/src/derive.rs`, `derive_ship`) and checked by running it, with `--check`, on a scratch list of the 48 ships (not committed) before and after the rows were added. For each ship, on the list's date, it reads:

- the class's type, mapped to a kind (no mapping exists for `transport` or `cargo_ship`: a rule, not a fact);
- `endurance_speed_kn` (the cruise speed), `max_speed_kn`, `fuel_capacity_lt`, `endurance_nm`, `displacement_standard_lt` (toughness), `main_rounds_per_gun`, on the ship or its class, preferred or named by an exception;
- the `main_battery` and `torpedo_tubes` fits (a sourced count of 0 for none), and for the main gun `shell_weight_kg` and `rate_of_fire_practical_rpm` (or an achieved rate of its kind, decision 0070), for the torpedo `warhead_kg`;
- a radar fit, or a sourced none, unless the list assumes unknown means none;
- a troop figure where the list asks for one.

Three things the brief's list did not name are needed: **a fit that holds on the date** (a row dated "1942" by year only cannot be placed on 7 August, and many shipscribe and DANFS rows are dated so or are for the ship as reclassified in 1943); **the weapon identified by length or mark** (`us-5in-unspecified` has no shell or rate); and **a sourced torpedo fit of 0** for each transport (several are missing or dated after the landings). Cargo capacity is not read: the tool writes 0, and `cargo_capacity_lt` rows wait for decision 0165's step 2.

## What was added

| File | Rows | Ids |
| --- | --- | --- |
| `ship_class_facts.csv` | 80 | 2110-2129, 2250-2284, 2350, 2450-2476 (2464, 2466 and 2468 unused) |
| `ship_facts.csv` | 144 | 2700-2773, 2800-2856, 2900-2911, 3000 |
| `fits.csv` | 77 | 880-897, 960-984, 1020-1025, 1080-1103, 1116-1119 |
| `weapon_facts.csv` | 8 | 510-513, 530-533 |
| `ship_classes.csv` | 2 | 1063 `us-caldwell-class` (Manley), 1066 `us-libra-class` (proposed by one source) |
| `attributes.csv` | 8 | `fuel_capacity_note`, `endurance_note`, `armament_note` (class and ship, text, many); `endurance_nm` and `endurance_speed_kn` on a ship |
| `gaps.csv` | 71 | the last 71 entries |

Fits rows have no `as_stated` column: each one's quotation is appended to its note as `Quote: "..."`.

**Standings of rows on file changed** (standing only; a sentence saying why is appended to each note; no value was touched), so that `oob-check` passes: to `disputed` because a new row gives another value for overlapping dates, ship_class_facts 1418, 1422, 1432 and ship_facts 1125, 1126, 1127, 1162, 1164, 1196, 1229, 1230, 1231, 1232, 1234, 1383, 1384, 1394, 1395, 1399, 1400, 1401; to `preferred` because a second source now agrees (decision 0015, 9), fits 168, 836, ship_class_facts 1431 and ship_facts 1160, 1161, 1166, 1203, 1204, 1205, 1208, 1210. Rows of this round were set the same way.

## How it was read and checked

Four Sonnet researchers (the transports, the cargo ships with the 5-inch/51 and 3-inch/23, the destroyer-transports with the 4-inch/50 and 3-inch/50, the Japanese destroyers), each with its own id ranges; the lead merged and checked. Pages were read as HTML or text and saved as text: NavSource (navsource.net and Wayback copies of navsource.org), shipscribe.com's auxiliary pages, DANFS at HyperWar, the hazegray copy and history.navy.mil through Wayback, HyperWar's ship pages (Clancey), navweaps.com, navypedia.org, Nishida's rikukaigun.org pages, the OCR text of ONI 222-J on archive.org (read as plain text), the HyperWar Maritime Commission types page, naval-encyclopedia.com, destroyerhistory.org, two combinedfleet tabular records and Wikipedia (low, where nothing better was found). No PDF or image was downloaded; nothing came from `references/` or any game.

Every quotation (324 rows) was checked by script against the saved page text, whitespace and case normalised: none failed. The lead re-fetched six of the pages and found the sampled quotation on each (NavSource Betelgeuse, President Jackson and Heywood; DANFS Fomalhaut; Nishida's Hatsuharu page; navweaps 4-inch/50). The check does not show that a statement was filed against the right ship, date or kind of figure; the notes say where a filing rests on a reading.

What the lead changed in the researchers' rows: twelve inferred "no radar" fits (fits 1104 to 1115, not filed) that rested on Wikipedia's sentence that Hamakaze was the first Japanese destroyer with radar, late in 1942 (an inference, kept as ship_facts 3000 and the gaps); the Kagero, Kamikaze and Mutsuki endurances at 15 knots (ONI 222-J 5,700 nm; navypedia 4,000 and 4,000), filed as `endurance_note` (2463, 2467, 2465), not as `endurance_nm`, because an endurance at another speed does not contradict the class rows on file at 18 and 14 knots (filed as `endurance_nm` they made Kagero's and Kamikaze's endurance disputed and broke the Eastern Solomons and Savo Island lists); the gun-level "about 150 rounds" of the 12.7 cm/50 filed on the Kagero, Shiratsuyu and Hatsuharu classes (2450 to 2452) lowered to low, inferred and unconfirmed, since the page names no class and says the outfits of later ships are not available.

## Findings

**No standard displacement exists for the merchant hulls.** No source read gives one for any of the 19 transports and cargo ships. They give light, full load, limiting, gross, deadweight or a tonnage of no stated kind, all filed under their own names. Navypedia's "standard" for the cargo ships equals the "Navy light" of the Maritime Commission type in HyperWar and the light of shipscribe and NavSource: it is filed as printed (disputed, low) and is almost certainly a light displacement. Toughness for these ships needs a rule (decision 0165, step 2), not a fact.

**Endurance and fuel rest on navypedia alone.** For every American class the only endurance with a speed is navypedia's, at low; it gives the same 9,000 nm at 12 kn for the McCawley and Heywood classes and the same 4,449 t for McCawley, Harris and American Legion, which looks like a template. NavSource gives fuel in barrels for most ships (filed as `fuel_capacity_note`, not converted). FTP 218 has no transport or APD tables. The 1945 Ships' Data book (a PDF) would give fuel and radius and was not read.

**The guns of 7 August.** Shipscribe dates a single 5-inch/51 to 1941-42 or 1942 for the President Jackson class, American Legion, Hunter Liggett, Crescent City, the Arcturus class, Bellatrix, Libra and Fomalhaut, but by year only, so the derivation cannot place it on a day. NavSource photo captions put a 5-inch/51 aboard Barnett (18 December 1941) and aboard Neville, Hunter Liggett and American Legion until 1943 or 1944, with no start date. NavSource's specification blocks give 5-inch/38 for McCawley, Fuller and Elliott, but list later guns beside it. Alhena carried a 4-inch/50 from late June 1942 (shipscribe), against "1 5"" in DANFS. The 5-inch/51's shell is now on file (22.7 kg, navweaps); no source gives a practical rate for it, the 3-inch/23, the 4-inch/50 or the 3-inch/50 (designed rates filed).

**The destroyer-transports' guns stay disputed, and a new fact cuts across both readings.** NavSource gives three 3-inch/50 for all four (fits 1020 to 1023), undated and with 40 mm guns beside them, so probably later than 1942. DANFS (history.navy.mil) says the hits on Colhoun on 30 August 1942 "blew two 20mm. guns and one 4" gun off the ship" (ship_facts 2900): at least one 4-inch was aboard that month. No trial speed of a Wickes APD was found.

**The Japanese destroyers are nearly complete.** Kagero: the torpedo fit is filed (eight 61 cm Type 93 tubes, four sources, preferred); only rounds per gun (a gun-level figure, unconfirmed) and radar remain. Hatsuharu, which no list covered: displacement, speed, fuel, guns and a six-tube Type 90 fit are filed and preferred; its endurance is disputed (6,000 nm at 15 kn, navypedia; 4,000 at 14, Wikipedia). Shiratsuyu: torpedo fit filed; endurance disputed (6,000 at 15, navypedia; 4,000 at 18, Wikipedia). Kamikaze (Oite) and Mutsuki: no rounds per gun in any source read (navweaps prints "N/A" for the 12 cm/45). Mutsuki's guns are now two sources for the 12 cm/45 against Nishida's /50.

**Kamikaze class, a dispute that reaches Savo Island.** Navypedia says that in 1941-42 every ship of the class but Hayate lost its No. 4 gun and one twin torpedo mount; Wikipedia's Oite article says the same of Oite "early in the war". Filed as class fits of 3 guns and 4 tubes (1100 to 1103, disputed; the 4 is the researcher's subtraction). The class rows on file (4 guns, 6 tubes, fits 152 to 154 and 316 to 318) were left preferred: fits have no consistency check, and changing them would make Yunagi at Savo Island underivable. The architect should rule on it, knowing it bears on Yunagi.

**Radar.** No source read states, in words, that any of these ships had or had not radar on 7 August 1942, except Zeilin's NavSource photograph "without radar" early in 1942 and the Betelgeuse row on file for 30 July to 3 August. For the Shiratsuyu and Hatsuharu classes Wikipedia says radar was not fitted until late in the war (fits 1116 to 1119, low, unconfirmed). The radar of every ship stays to the list's working assumption.

**Classes of the ships with none.** Manley is Caldwell class by DANFS (new key `us-caldwell-class`); McFarland is Clemson class by two DANFS copies; Bellatrix is Arcturus class by NavSource, Procyon class by shipscribe and navypedia, and Bellatrix class by DANFS (all disputed); Libra is Arcturus (NavSource), Procyon (navypedia) or Libra class (shipscribe, new key `us-libra-class`), all disputed. `ships.csv` was not changed. Stringham's DANFS could not be read.

**Every Japanese ship of the runs** has the class `ships.csv` gives her. Classes no derivation list covers: Kagero, Shiratsuyu, Kamikaze (but Yunagi at Savo Island), Hatsuharu and Mutsuki; Murakumo, Uranami, Shikinami and Asagiri are Fubuki class, derivable now with the radar assumption.

## Sources wanted and not read

Ships' Data, U.S. Naval Vessels, 15 April 1945, volume III (shipscribe.com/pubs/SDB1945v3.pdf; PDF); the ONI Combat Narrative "Miscellaneous Actions in the South Pacific" (the HTML page timed out; the PDF not read); the transports' and APDs' action reports and Turner's report for 7 to 9 August; Friedman's US Amphibious Ships and Craft and US Destroyers; Lenton; USNTMJ O-01-1 (torpedo allocation by class; PDF); Lengerer on the Hatsuharu class (Warship 2007); USNI Proceedings, January 1952, "Japanese Destroyers in World War II" (refused); combinedfleet tabular records of Ariake, Yugure, Oite, Uzuki and the Kagero class (Wayback rate-limited them); Lacroix and Wells; Jentschura; DANFS for Bellatrix, Libra, Betelgeuse and Stringham (did not load).

## The table for the architect

Ship rows are in `ship_facts.csv`, class rows in `ship_class_facts.csv` (C), fits in `fits.csv` (F), weapons in `weapon_facts.csv` (W). "On file" means a row from before this round. "Not found" means the record read is silent; each has a gap row.

### American transports

| Class (ships) | Standard displacement | Max speed | Endurance with speed | Fuel | Main gun on 7 Aug 1942 | Rounds per gun | Radar | Troops; cargo |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| McCawley (McCawley, Barnett) | Not found. Light: Barnett 9,750 (2702) against 9,432 (C 1390) | McCawley 15 (2726), Barnett 15 (2727); class 17, 15, 15, 14.5 on file | 9,000 at 12, navypedia only (C 1386, 1387) | 4,449, navypedia only (C 1388); Barnett 9,570 bbl diesel (2761) | Barnett 5"/51 dated 18 Dec 1941 (F 880) and to 19 Feb 1943 (F 881); McCawley 5"/38 in a later block (F 886) against 5"/51 (F 759, 1942 by year) | Not found | Not found | Barnett 1,491 (2739); 1,700 DWT (2751) |
| Heywood (Heywood, Elliott, Fuller, Neville) | Not found. Light: 8,000 (2704, 2706), 7,475 Neville (2708), 7,630 Elliott (2710) against 8,789 (C 1399) | 16.8, 10.5, 16, 16 (2728-2731); class 16 and 16.8 on file | 9,000 at 12, navypedia only (C 1396, 1397) | 1,523, navypedia only (C 1398); Heywood in barrels (2762) | Neville 5"/51 to 17 Apr 1943 (F 882); Fuller and Elliott 5"/38 (F 887, 888) against 5"/51 (F 762, 765) | Not found | Not found | 1,278, 1,278, 878, 1,150 (2740-2743); cargo 2,000, 2,900, 2,400 (2752-2754) |
| Harris (Zeilin, Hunter Liggett) | Not found. Light 13,529 (2700, 2712) | Zeilin 18 (2725), Liggett 15 (2732) | 13,700 at 15, navypedia only (C 1411, 1412) | 4,449, navypedia only (C 1413); Liggett 25,720 bbl (2763) | Liggett 5"/51 to 29 Mar 1944 (F 883) and 1941-42 by year (F 895); Zeilin not found | Not found | Zeilin "without radar" early 1942 (2771); else not found | 1,679, 1,524 (2738, 2744); 2,000, 1,900 (2750, 2755) |
| American Legion | Not found. Light 13,529 (2714) | 17.5 (2733; on file C 1414) | 13,700 at 15, navypedia (2116, 2117) | 4,449, navypedia (2118); 25,720 bbl (2764) | 5"/51 to 6 Apr 1944 (F 884), 1941-42 by year (F 894); DANFS "5"" (F 777) | Not found | Not found | 1,644 (2745); 2,500 (2756) |
| President Jackson (Jackson, Adams, Hayes) | Not found. Light 9,500 (2716-2720), 10,210 (2110) | 18.4 (2112, 2734-2736) against 18, 17.9, 17 on file | 10,700 at 15, navypedia (2119, 2120) | 1,340, navypedia (2121); 10,950 bbl (2765-2767) | 5"/51 1941-42 by year (F 889, 891-893); DANFS "5"" (F 781, 784, 787) | Not found | Not found | 1,382 (2125, 2746-2748) against 1,388, 1,350 on file; 6,800 DWT (2757-2759) |
| Crescent City | Not found. Light 8,409 (2722), 8,889 (2113) | 17 (2737), 16.5 (2115) | 15,500 at 15, navypedia (2122, 2123) | 1,751, navypedia (2124); 12,925 bbl (2768) | 5"/51 (F 885, 896) against 6" (F 790) and 5"/38 (F 810) | Not found | Not found | 1,160 (2127, 2749); 2,300 (2128) |

Verdict for every transport class: **not every figure can be read.** Missing as facts: standard displacement (none exists: a rule), rounds per gun, radar, a second source for endurance and fuel, a main gun dated to the day, and the kind (a rule).

### American cargo ships

| Ship or class | Standard displacement | Max speed | Endurance with speed | Fuel | Main gun on 7 Aug 1942 | Rounds | Radar | Cargo; troops |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Arcturus class (Alchiba, Betelgeuse; Libra and Bellatrix disputed) | Not found; navypedia "standard" 7,293 is light (2253) | 16.5 class (2252, 2255), ships 16.5 (2801, 2802, 2851) | Not found; "16,200 miles" with no speed (2260) | 1,790, navypedia (2256); 8,950 bbl (2822, 2823) | 5"/51 1942-44 by year (F 960); Betelgeuse 5"/51 1942 (F 962) | Not found | Betelgeuse none 30 Jul-3 Aug on file; else not found | 3,705, 4,785 (2817, 2818) |
| Bellatrix | Not found | 16.5 (2803) | 22,600 at 12, navypedia (2843, 2844) | 1,790 (2845); 8,950 bbl (2824) | 5"/51 1942 by year (F 971) | Not found | Not found | 4,515 (2819, 2849); 54 troops (2850, from 1943) |
| Libra | Not found | 15.5 (2804), 16.5 (2839) | 18,180 at 12, navypedia (2846, 2847) | 2,130 (2848); 10,670 bbl (2825) | 5"/51 1942 by year (F 973, 975) | Not found | Not found | 150 troops, Jan 1942 plan (2840) |
| Fomalhaut | Not found; navypedia 7,480 (2266) | 14.8 (2805, 2264), 15 (2855), 16 navypedia (2268) | Not found | 1,235 navypedia (2269); 9,345 bbl (2826) | 5"/51 1942 by year (F 976, 978) | Not found | Not found | 4,500 (2820); 150 or 174 troops (2265, 2271) |
| Alhena | Not found; navypedia 7,101 (2275) | 16.6 (2806, 2853, 2274, 2277) | 11,000 at 12, navypedia (2279, 2280) | 2,360 navypedia (2278) against 2,012 on file; 11,810 bbl and 1,175 bbl (2827) | 4"/50 from late June 1942 (F 980) against DANFS 5" (F 825) | Not found | Not found | 4,885 (2821, 2281); 162 troops (2282) |

Verdict for every cargo ship: **not every figure can be read**, for the same reasons as the transports, and Bellatrix and Libra also need their class settled.

### Destroyer-transports

| Ship | Guns after conversion | Max speed | Standard displacement | Endurance, fuel | 4-inch/50 shell | Rounds | Radar | Troops |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Colhoun, Gregory, Little, McKean | 3 x 3"/50 NavSource (F 1020-1023), Clancey (F 1024) against 3 x 4"/50 navypedia (F 847, 854, 861, 868); a 4" aboard Colhoun on 30 Aug 1942 (2900) | Disputed: 35 and 35.3 class, 24 and 25 ships on file, 35 Clancey (2907); no trial figure found | 1,090 lone low row on file; not found | 2,380 bbl (C 2350); no endurance | 14.97 kg (W 530); 3"/50 5.9 kg (W 532); designed rates only (W 531, 533) | 300, lone low row on file (C 1443); not found for APDs | Gregory aboard 5 Sep 1942 (F 872); otherwise not found | 144 or 147 (on file); a load of 185 in 1943 (2906) |
| Manley | 3 x 3"/50 (F 1025); 4-inch replaced by 3-inch (2902) | 30 (2908) against 27.2 on file | Not found | Not found | as above | Not found | Not found | 147 on file |
| McFarland | Not researched beyond class (Clemson, 2910, 2911) | | | | | | | |

Verdict: **not every figure can be read.** Missing: a dated main battery, a settled speed, a standard displacement above a lone low row, endurance and fuel in long tons, rounds per gun, radar.

### Japanese destroyers

| Class (ships of the runs) | Displacement, speed, fuel | Endurance | Main battery | Torpedo fit | Rounds per gun | Radar | Verdict |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Kagero (Kagero, Arashi, Hagikaze, Hamakaze, Tanikaze, Urakaze; Isokaze) | On file, readable | 5,000 at 18 on file; 5,700 at 15 ONI 222-J as a note (2463) | On file | 8 x 61 cm Type 93 (F 1088-1091) | 150, gun-level, unconfirmed (2450) | Not found (Hamakaze first, late 1942: 3000) | Every figure readable but rounds per gun (an exception on 2450, or the ammunition assumption) and radar (the list's assumption) |
| Shiratsuyu (Kawakaze, Umikaze, Suzukaze, Yudachi) | On file, readable | Disputed: 6,000 at 15 navypedia (2453, 2454), 4,000 at 18 Wikipedia (2455, 2456); ONI note (2457) | On file; a single gun removed 1942-43, month not stated (2476) | 8 x 61 cm Type 93 (F 1093, 1094; 1092 dated from 7 Dec 1941 by the researcher's reading of "by the start of the Pacific War") | 150, gun-level, unconfirmed (2451) | None till late in the war, Wikipedia, low (F 1116, 1117) | Endurance needs a ruling; rounds and radar as Kagero |
| Hatsuharu (Ariake, Yugure) | 1,688 Nishida (2469) and 1,715 navypedia (2470); 33.3 kn (2471, 2472); fuel 492 Nishida (2473) and 500 navypedia (2474) | Disputed: 6,000 at 15 navypedia (2458, 2459), 4,000 at 14 Wikipedia (2460, 2461); ONI note (2462) | 5 x 12.7 cm/50 (F 1080-1082) | 6 x 61 cm Type 90 (F 1083-1085) against Type 93 (F 1086) and 8 tubes (F 1087) | 150, gun-level, unconfirmed (2452) | As Shiratsuyu (F 1118, 1119) | Endurance needs a ruling; rounds and radar as Kagero |
| Kamikaze (Oite) | On file, readable | 3,600 at 14 on file; 4,000 at 15 navypedia as a note (2467) | 4 on file against 3 from 1941-42 (F 1100, 1101) | 6 on file against 4 (F 1102, 1103) | Not found | Not found | Rounds per gun missing; the guns and tubes dispute (it reaches Yunagi) |
| Mutsuki (Uzuki; Mutsuki, Yayoi) | On file | 3,600 at 14 Wikipedia only (unconfirmed); 4,000 at 15 navypedia as a note (2465) | 4 x 12 cm/45 now preferred (F 1096, 1097) | 6 x 61 cm 8th Year (F 1098, 1099; on file F 168 now preferred) | Not found | Not found | Rounds per gun and a preferred endurance missing; Uzuki's conversion to a fast transport in 1942, month not stated |
| Fubuki (Murakumo, Uranami, Shikinami, Asagiri) | On file | On file | On file | On file | On file | Not found | Every figure readable, with the radar assumption the landings list already makes |

## What was not verified

That the NavSource specification blocks and photo captions describe the ships as they were in August 1942 (most are of the APA or AKA of 1943 on); whether NavSource's "lt" is light or long tons (taken as light displacement in long tons, as the rows say); which ton navypedia's "t" is; whether navypedia's identical endurance figures are templated; the date of the APDs' 3-inch fit; any combinedfleet tabular record beyond Hamakaze's and Yudachi's.
