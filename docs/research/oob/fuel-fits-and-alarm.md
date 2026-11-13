# Fuel capacity, three fits, and alarm to first fire at Savo Island

Research for the step 2 coordinator's second list of 13 November 2026: five facts wanted for the Cape Esperance derivation, and one sentence in the Savo Island notes that had no row behind it. This page says what the rows hold, what was found and what was not. It settles nothing.

The reading was done by Sonnet agents: three readers quoting from saved pages, and a clerk keying the records. Every quote was checked by program as an exact span of the saved source text (34 of 34, 16 of 16 and 196 of 196).

## What the rows hold

| File | Rows | What |
| --- | --- | --- |
| `ship_class_facts.csv` | 1354 to 1358 | Fuel capacity of four classes from FTP 218; one full-load displacement for the Benson class |
| `fits.csv` | 748 to 750, 756, 757 | Laffey's 5-inch guns, San Francisco's main battery |
| `ship_class_facts.csv` | 1359 to 1362 | The Fubuki class's torpedo model (`torpedo_model`) |
| `engagement_outcomes.csv` | 3859 to 3941 | Savo Island: general quarters, readiness, first fire and reasons fire was late, by ship |

Standing follows the checker's rule: `preferred` where a value is supported, `unconfirmed` where it is not, and `disputed` where rows give different values.

## Fuel capacity

FTP 218, War Service Fuel Consumption of U.S. Naval Surface Vessels, Table V, read as the HyperWar transcription. The table prints three totals for each class in gallons, barrels and tons, with 277 gallons of fuel oil to the ton. The row holds the "maximum capacity (95%)" figure in tons; the other two are in its note.

| Class (table) | "Full load" | Radius oil | Maximum (95%) | Row |
| --- | --- | --- | --- | --- |
| Brooklyn (CL 47 Boise, her own table) | 1,981 | 2,036 | 2,175 | 1354 |
| Pensacola (CA 24 and 25) | 2,116 | 2,140 | 3,088 | 1355 |
| St. Louis (CL 49 only) | 1,892 | 2,036 | 2,288 | 1356 |
| Benson (DD 421/459 class) | 466 | 505 | 505 | 1357 |

- The older rows for these classes (999, 1035, 1075) took the upper end of a range on Navypedia as the maximum. For the Pensacola class that upper end, 2116, is the figure FTP 218 prints as "full load", and for Boise it is within a ton of it (1982 against 1,981). The older and newer rows differ, so all six are now `disputed`. Which of the three totals the derivation should read is the architect's to settle.
- Helena has no entry: the St. Louis table lists CL 49 alone. Using her sister's figure for her is an inference.
- The Benson class table's ship list does not include DD 459, Laffey.

## Laffey's 5-inch guns

Still disputed, four against five.

| Guns | Source | Row |
| --- | --- | --- |
| 4 | Cominch bulletin's ship table for 13 November 1942; OCR, and the ship is matched to her line by order only | 748 |
| 4 | DANFS header; the same header prints a displacement and a tube count that look wrong | 749 |
| 4 | Wikipedia, "DD-459 onward" | 724 (older) |
| 5 | NavSource armament line | 713 (older) |
| 5 | A press caption at her launch on 30 October 1941, which speaks of the design | 750 |

Not rows: her battle report of 14 November says "5" guns number two, three and four out of commission" and "Stern end blown off up to number four turret", and names no gun number five. The Destroyer History Foundation says the repeat ships "were ordered with only four 5-inch/38s" without naming her in that sentence.

## Benson class displacement

No second source that says "standard" was found. DANFS prints "dp. 1,620" and the Destroyer History Foundation and FTP 218 call the class "1,620-ton", none saying which displacement. Navypedia gives tonnes. One full-load figure was entered: 2,103 tons for Benson herself from NavSource (row 1358), which differs from the two already on file (2,395 and 2,474).

## The Fubuki class's torpedo in 1942

Every source read says Type 90; none says Type 93 before August 1943.

- US Naval Technical Mission to Japan, report O-01-1, on the night action of 14 November 1942: "The Japanese destroyers were fitted with Type 90 torpedoes, except SAMIDARE which had Type 93." The destroyers named are Samidare, Shirayuki and Hatsuyuki (class row 1359, entered for the class by inference; low confidence because the OCR dropped a line, confirmed by the reader from the page image).
- NavWeaps lists the Type 90 for "Heavy Cruisers and Fubuki destroyer classes" (1360). Japanese Wikipedia has the change from the Type 8 at about 1935 (1361). English Wikipedia has the class still loaded with the Type 90 from the start of the war to mid 1943 (1362).
- Leads, not rows: Japanese Wikipedia says some ships' tubes were altered for the Type 93 from August 1943. The same US report's summary gives the Type 90 as in service 1931 to 1942 without naming a class.
- These are statements of type only, with no tube count, so they are rows of the class attribute `torpedo_model` in `ship_class_facts.csv` (1359 to 1362). They were first entered as fits 751 to 755 with an empty count, which a derivation cannot read; those five ids are retired and will not be reused. Nothing names Murakumo.

## San Francisco's main battery

Nine 8-inch guns in DANFS (756; the mark is not printed, so the item is inferred) and "9-8"/55" in the Cominch bulletin's ship table for 13 November 1942 (757). No source read states three triple turrets for her by name.

## Salt Lake City's rounds a gun

Not found. NavWeaps says it has no figures for the Pensacola class. The sources read on her 1943 action say that an armour-piercing allowance for each mount existed and that the after mounts ran out; none gives a number. Row 1016 stays as it was.

## Alarm to first fire at Savo Island

The notes page for Savo Island said "two to five minutes from alarm to first salvo". No source read prints that, for any ship or for the group, and the sentence is corrected there.

What the sources do print (from outcomes 3859 to 3941 and the older events for each ship: Astoria 90, 94, 96; Vincennes 97, 1063 to 1065; Canberra 59, 79; Quincy 1048, 1066). Times are as printed; Bates's text is OCR and some of its times are garbled, as the rows' notes say:

| Ship | Alarm | First fire | Interval printed |
| --- | --- | --- | --- |
| Canberra | general alarm, no time; Condition I "immediately" | main battery as she turned; port 4-inch "may have" fired | none |
| Chicago | readiness "not reported" (ONI) | star shell about 0147 | none |
| Patterson | general quarters at once on sighting | guns with the torpedo order, star shell first | none |
| Bagley | general quarters at once | torpedoes about 0149; no gun fire described | none |
| Vincennes | Condition I about 0146 (Bates) | 8-inch about 0155, or about 0152 3/4 in another passage | first searchlight 1 minute after general quarters, or 5 by another report (to the searchlight, not a salvo) |
| Quincy | about 2 minutes after the others (ONI); first of the cruisers (Bates) | about 0159 | eight minutes since general quarters with no shot; first salvo delayed "about twelve minutes" (outcomes 3059, 3028); "possibly ten minutes" to set Condition I (3924) |
| Astoria | rung by a quartermaster on his own initiative; sounded properly only after the cease-fire | about 0152 to 0153, six or nine guns | "fully five minutes" after the other cruisers' alarm with no action on her bridge |
| Helm, Wilson | Condition I about 0145 to 0150 (Bates presumes for Helm) | about 0150 | none |
| Ralph Talbot | about 0148 | about 0216 to 0219; held fire thinking the first ship friendly | none |

Bates's general figure is that setting Condition I from Condition II took about five minutes by day and a minute or so more at night (outcomes 3917, 3925). That is time to man stations, not time to a first salvo.

Of 196 records read, 83 became rows; 45 were already in the database from the same source and 68 were dropped as repeats or as not stating what the reader took from them. Three rows first keyed as alarm-to-fire intervals run from Condition II to Condition I and are entered as `condition_one_setting_time_min`.

## Not found

In `gaps.csv`: Salt Lake City's rounds a gun; a fuel figure for Helena herself; Laffey's gun count from a dated source that names her without doubt; a standard displacement for the Benson class from a second source; Murakumo's torpedo; and an alarm-to-salvo interval for any ship but Quincy.
