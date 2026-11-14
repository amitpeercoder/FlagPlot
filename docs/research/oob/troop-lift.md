# Transports and troop capacity, August 1942

Status: research report of the troop-lift team for the architect, read on 2026-11-14, for decision 0093, 8. Nothing here is a ruling of the architect. Where the team chose between rows, the row's note says "Settled by the troop-lift lead, for the architect's review", and those choices are listed below.

## What was added

| File | Rows added | What |
| --- | --- | --- |
| `ships.csv` | 28 | The 13 transports (AP) and 6 cargo ships (AK) of Task Force 62 on 7 August 1942, and 9 Japanese destroyers of the troop runs (Oite, Arashi, Hamakaze, Urakaze, Yudachi, Amagiri, Asagiri, Ariake, Yugure). Hagikaze, Yugiri and Shirakumo, already there with no class, now have the class their new `ship_class` rows give |
| `ship_classes.csv` | 15 | Transport classes (type `transport`), cargo classes (type `cargo_ship`), and the Hatsuharu class |
| `ship_facts.csv`, `ship_class_facts.csv`, `fits.csv` | 239, 54, 93 | Class, speeds, displacements with their condition, cargo, fuel and endurance where found, guns, the absence of torpedo tubes, troop capacity, and the troop rows restated under the ships |
| `troop_lift_facts.csv`, `lift_entities.csv` | 35, 10 | The Japanese runs of 16 August to 11 September: men, ships, and which ships |
| `attributes.csv` | 16 | `troop_capacity_men`, `troops_carried_men`, `troop_run`, `displacement_light_lt`, `displacement_limiting_lt`, `cargo_capacity_lt`, `service_speed_kn`, `boats_carried_note`, `merchant_type`, and ship forms of the class attributes the ship rows use |
| `weapons.csv`, `sources.csv` | 7, 4 | US 3in/50, 3in/23, 4in/50, 5in/51 and three "calibre only" keys; DANFS as copied at hazegray.org, shipscribe.com, usmm.org's C2 list, the Naval History and Heritage Command's APD page |
| `gaps.csv` | 121 | Every fact looked for and not found |

## Sources, and how each quote was verified

The ship dictionary (DANFS) at HyperWar and, where HyperWar has no page, the copy at hazegray.org read through the Wayback Machine; Clancey's ship pages at HyperWar (graded C); shipscribe.com; navypedia, naval-encyclopedia and uboat.net; English Wikipedia (always low); combinedfleet.com's tabular records of movement; and the histories already in `sources.csv` (Zimmerman, Hough, Miller, Dyer, Shaw, Japanese Monograph 45, MacArthur's Reports).

Five researchers downloaded every page they cite with `curl`, saved it, and checked each quote word for word against the saved text with a script. 442 rows were handed in; their own checks dropped 2 on a first pass (a quote pointed at the wrong page, a quote with wiki markup not on the page), both fixed and rechecked. The lead then ran a second, independent check over all 442 rows against the saved pages: 0 failed. To check that the saved pages are the real ones, 14 rows drawn at random were checked against freshly fetched pages: 13 matched; the 14th is a quote of Wikipedia's wikitext checked against the rendered page, where the markup differs. Wikipedia rows quote the wikitext.

What the check does not show: that a number was filed against the right ship, configuration and date. DANFS data lines carry no date and often describe a ship as reclassified in 1943; such rows are left undated or dated from the reclassification, and the note says which.

43 rows from Clancey's pages were filed by their researcher at medium confidence. `oob-check` holds a grade C source to low, so they were entered at low, value and quote unchanged.

## Where sources disagree

- **Classes.** Barnett: McCawley class (DANFS data line, Wikipedia, navypedia) or a Barnett class (DANFS copy at hazegray). George F. Elliott: Heywood class (DANFS) or her own (shipscribe). American Legion: her own class (DANFS) or Harris class (Clancey, Wikipedia). Libra: Arcturus (DANFS, Wikipedia) or Procyon (Clancey). Bellatrix: Bellatrix (DANFS), Procyon (Clancey) or Arcturus (Wikipedia): her `class` in `ships.csv` is left empty and is a gap. Each source has its own `ship_class` row; the disputed ones are `disputed`, and `ships.csv` holds the class two or more sources give.
- **Troop capacity.** Clancey gives 1,550 for the three President ships and 1,650 for American Legion, against the ship dictionary's 1,388, 1,350 and 1,644. Zeilin: 1,679 or 2,077 in one DANFS entry, both as an attack transport from November 1942. The destroyer-transports: 144 (navypedia) or 200 (Wikipedia).
- **Speeds.** Most transport and cargo classes have two or three figures from 14.5 to 18 knots, none labelled trial or service.
- **Guns.** The destroyer-transports after conversion: three 4-inch/50 and six .50 calibre (navypedia) or three 3-inch/50, a 40 mm and five 20 mm (Wikipedia). Several cargo ships: a 5-inch of unstated length (DANFS) or a 5-inch/38 (Clancey, a later fit).
- **The Japanese runs.** 18 August: 900 (Shaw, Hough's order), about 916 (combinedfleet) or about 1,000 (Miller) men. 29 August: four destroyers (Zimmerman) or "two destroyer transports" (Hough). 31 August: eight destroyers by name (combinedfleet), with Yudachi in Zimmerman's list where combinedfleet has Hatsuyuki. 5 September: 231 or 370 men.

## Settlements by the troop-lift lead, for the architect's review

1. **Barnett's load of 2 August:** 1,494 men preferred (the block's own columns and the group total both need it), the typed 1,404 alternative.
2. **Troop capacity of American Legion and the President ships:** the ship dictionary's data line preferred, Clancey's figures alternative: Clancey is grade C and his pages describe the ships as attack transports of 1943 or later.
3. **Ship classes in `ships.csv`,** as above: the class two or more sources give; Bellatrix none.
4. **The two runs the new list reads** (29 and 31 August), and their rows: in the list, with the reason.
5. **The radar of the four Fubuki-class ships** of the list is treated as not fitted, the working assumption of decision 0019, 5 that the Cape Esperance list already makes for Fubuki and Hatsuyuki.

## How the troop rows relate to ships and classes

The existing rows of `troop_lift_facts.csv` are about entities of `lift_entities.csv`, which are not ships. The smallest change that lets both tools use them without touching an old row was to **restate** the rows a ship's figure rests on under the ship, in `ship_facts.csv`, with the same source, locator and quote and a note naming the old row: the five stated capacities (rows 55, 78, 80, 81, 82) as `troop_capacity_men`, and the 22 loads of the station list of 2 August read from the page images (rows 111 to 132) as `troops_carried_men`. `oob-check` checks them as every ship fact. A run is not a ship, so run rows stay in `troop_lift_facts.csv`; a ship is tied to a run by a `troop_run` row of hers, and `oob-check` now knows the two troop files (entities, closed list of attributes, whole numbers, and that every `troop_run` names a run).

## The rule

`oob-derive` writes `troop_capacity_men` only for a list with `troop_figures = true`, so every existing class file is byte-identical. The rule, as the sources file gives it:

> troop_capacity_men of the ship, else of its class, as a source states it; else, where the list names a voyage, troops_carried_men of the ship on that voyage (a load on one voyage, not a capacity); else, where the list names a run for the ship, troops_carried of the run over the ships in the run as the list reads them from the named ships_in_run row, rounded down (a run total, a weaker kind of support than a stated capacity); else 0 for a warship (no troop row: carries none) and an error for a transport or cargo ship (decision 0093, 8)

Ships of one facts class whose figures differ get class rows of their own, named by the figure, as for every other figure (decision 0077): "Fubuki class, troop_capacity_men 225". A run is named in the list by its men row and its ships row, with the number of ships as the list reads it from the row (whose value may be words), and the reason; those rows are read whatever their standing, and the run's other rows of men are named in the sources file as not read. The run rule works by the ships the list names for it, not by ship type: every run read here names its ships.

## The derivation list

`data/derivation/guadalcanal_landings.toml`, date 1942-08-07, class ids 3009 and 3010. It derives only the four Fubuki-class destroyers, because a list writes nothing if one figure of one ship cannot be read:

| Class row | Ships | troop_capacity_men | Rests on |
| --- | --- | --- | --- |
| 3009 Fubuki class | Fubuki, Shirayuki | 225 | 900 men (troop_lift_facts 45, Zimmerman) over 4 ships (148, Zimmerman), run of 29 August; troop_run rows 1315, 1316 |
| 3010 Fubuki class | Hatsuyuki, Amagiri | 150 | 1,200 men (47, Zimmerman) over 8 ships (155, combinedfleet), run of 31 August; combinedfleet's 143 to 155 men a ship (153) agrees; troop_run rows 1327, 1330 |

## The troop figure each class would get, and what else stops it

These ships are not in the list. Their troop figure is what the rule gives from the rows now in the files; the last column is every other figure that cannot be read.

| Class (ships) | troop_capacity_men by the rule | What stops the class |
| --- | --- | --- |
| American Legion class (American Legion) | 1,644 stated (sf 1123) | kind; endurance with its speed, fuel: no rows; the 5-inch has no rate or shell; rounds per gun; standard displacement; radar |
| President Jackson class (Jackson, Hayes; Adams) | 1,388 stated (1127, 1126); Adams 1,350 stated (1125): two class rows | kind; max speed disputed (18, 17.9); endurance, fuel; the 5-inch; rounds per gun; standard displacement; radar |
| Heywood class (Neville; George F. Elliott; Heywood; Fuller) | Neville 1,150 stated (1124); Elliott 1,278 stated (1159); Heywood 1,158 and Fuller 1,167, loads of 2 August (1136, 1130): four class rows | kind; endurance, speed and fuel are lone navypedia rows; max speed disputed; no main battery fit holds on the date; rounds per gun; standard displacement; radar |
| McCawley class (McCawley; Barnett) | McCawley 1,428 and Barnett 1,494, loads (1133, 1149) | as Heywood |
| Harris class (Hunter Liggett; Zeilin) | Hunter Liggett 1,611, load (1135); Zeilin none: her stated rows are disputed and date from November 1942, and the list of 2 August gives no figure for her | as Heywood; the tube fit dates from November 1942 |
| Crescent City class (Crescent City) | 687, load (1139) | kind; endurance, fuel; main battery disputed; rounds per gun; standard displacement; radar |
| Arcturus class (Alchiba, Libra, Betelgeuse) | Alchiba 539, Libra 280 (1137, 1134); Betelgeuse none | kind; max speed, endurance, fuel; main battery; rounds per gun; standard displacement; radar |
| Alhena class (Alhena), Fomalhaut class (Fomalhaut) | 160, 227, loads (1140, 1138) | as Arcturus |
| Bellatrix (no class) | 255, load (1131) | her class |
| Wickes class, destroyer-transports (Colhoun, Gregory, Little, McKean) | 157, 155, 154, 149, loads (1145, 1146, 1144, 1147): the stated 144 and 200 are disputed and not read | max speed disputed (class 35, 35.3; ships 24, 25); main battery after conversion disputed; no shell weight for the 4-inch/50; rounds per gun and standard displacement are lone low rows; radar |
| Kagero class (Arashi, Hagikaze, Hamakaze, Urakaze, Tanikaze, Kagero) | 150 by the run of 18 August (900 over six) | no torpedo tube fit; no rounds per gun; radar |
| Shiratsuyu class (Kawakaze, Umikaze, Suzukaze, Yudachi) | 225 or 150 by the runs of 29 and 31 August; Yudachi 130 by her run of 30 August | no endurance with its speed; no tube fit; no rounds per gun; radar |
| Kamikaze class (Oite) | 200 by her run of 16 August | no rounds per gun; radar |

"Kind" means that no rule maps the types `transport` and `cargo_ship` to a simulation kind, and no rule derives a transport's cargo (the rule writes 0, "warships carry no cargo"). Both are rules the architect has not made; the derivation refuses rather than guess them.

## What was not found

- **Endurance and fuel** for any transport or cargo class from any source but navypedia (three classes, which look copied between pages). FTP 218 covers battleships to minesweepers only. No ONI or Bureau of Ships data book for transports was found online.
- **Standard displacement** of any transport or cargo ship: sources give light, full load, limiting or no condition.
- **Radar** of any transport, cargo ship or destroyer-transport on 7 August 1942. Gregory had a set on 5 September (DANFS).
- **Rounds per gun** of the transports' guns.
- **A stated troop capacity** of any destroyer of either navy from an official source.
- **DANFS pages** for Hunter Liggett, Colhoun, Fomalhaut and Betelgeuse at HyperWar.
- **Japanese runs:** no figure for the runs of 7 September and none for Hough's "two destroyer transports" of 29 August. Japanese Monograph 98 and the Strategic Bombing Survey were not searched for the runs.
