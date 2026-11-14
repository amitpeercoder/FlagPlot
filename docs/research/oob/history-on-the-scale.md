# History on the objectives scale: the record in the scale's terms

Status: a fact sheet for the architect's plan to set the objectives scale from the record (decisions 0054 and 0088; decision 0069's section "The yard function's figures from the record", on branch `ship-repair`, pull request 235). Gathered Nov 13, 2026. It proposes no points, levels, horizons or figures. Nothing here settles a dispute: a dispute is shown as a range with each row. A missing fact is named as a gap. No row was added and no data file changed. Nothing from `references/` was read. *(Nov 14, 2026: the branches named are merged: decision 0069 is built (scenario 20a) and decision 0088 is built (scenario 20b).)*

How to read the citations:

- `EO n` is `data/oob/engagement_outcomes.csv` row id n. `SF n` is `ship_facts.csv`. `EF n` is `engagement_forces.csv`. `LUF n` is `land_unit_facts.csv`. `TLF n` is `troop_lift_facts.csv`. `PF n` is `place_facts.csv`. A gaps row is `gaps.csv` line n, since that file has no ids.
- Each row's source, locator, confidence and standing are on the row. Where a row is weak (`low`, `unconfirmed`, `disputed`) the table says so.
- "Days away" is date damaged to the date the row gives for her return, counted as `step2-record.md` section 3 and decision 0069 count it. Where the row's own note gives the count, it is that note's arithmetic. Otherwise it is mine, and marked so.
- "In action again" is read from the order-of-battle rows only: whether she is in a later engagement of the database (`engagements.csv`: Eastern Solomons 23 to 25 Aug, Cape Esperance 11 to 12 Oct, Santa Cruz 26 Oct, Guadalcanal 12 to 13 and 14 to 15 Nov, Tassafaronga 30 Nov to 1 Dec 1942), or a row's note says she sortied. The database is not a full record of every sortie. A "no" means "no row", not "she did not sail".

## 1. Each ship the scenario holds

### Savo Island, 8 to 9 August 1942 (scenario 4, 17 ships)

The scenario holds 8 Japanese and 9 Allied ships. It does not hold Ralph Talbot, Jarvis, the other screen ships or the transports (decision 0024).

**Sunk**

| Ship | Fate | Rows |
| --- | --- | --- |
| HMAS Canberra | Sunk 9 Aug, about 0800. She was abandoned, then scuttled by Selfridge and Ellet. Hits: 24 shells (EO 19 to 22). Torpedoes 0, 1 or 2 (disputed: EO 23, 24, 25, 163, 3222, 3231) | EO 33, 35, 761, 765; SF 609, 424 |
| USS Vincennes | Sunk 9 Aug 0250. Hits: 57 known, 74 with possibles. Torpedoes 1 to 3 (disputed) | EO 58 to 68, 762, 3968 |
| USS Quincy | Sunk 9 Aug, 0235 to 0240 (disputed). Hits: 36 known, 54 with possibles. Torpedoes 1 to 3 (disputed) | EO 69 to 81, 763, 3967 |
| USS Astoria | Sunk 9 Aug, 1215 or 1216. Hits: 34 by Bates (63 with possibles) against 65 by BuShips (disputed). No torpedo. The loss was "not inevitable" from structure; fires caused it | EO 82 to 92, 764, 3969 |
| Kako | Not damaged in the night action (EO 132). Sunk 10 Aug by the submarine S-44 on the way home, outside the action | EO 133, 135, 778; SF 443 |

**Damaged**

| Ship | Damage, in the sources' words | Days away | In action again within 30 / 60 / 90 days |
| --- | --- | --- | --- |
| USS Chicago | One torpedo in the bow that detonated, one dud amidships (EO 38, 39; a reading of two, EO 3201). One shell hit, probably 4.7-inch (EO 37). "Unable to maintain any greater speed than ten knots" on the morning of the 9th; "could still steam and fight" (EO 788). Repaired at Noumea, Sydney and San Francisco (EO 789) | Out from 9 Aug (SF 750). Noumea, Aug, temporary (SF 751). Sydney, about Sep (SF 752, low, unconfirmed). San Francisco from 13 Oct (SF 753). Back January 1943, month only (SF 754, 755, 756). **145 to 175 days** (mine, from the month); "about 150" in `step2-record.md` | No / no / no. No later engagement row in the slice. Sunk 30 Jan 1943 (SF 80) |
| Chokai | Shells: 3, 4 or 10 hits (disputed: EO 114, 115, 116, 117). "No. 1 turret out of action; partly restored, only starboard gun usable; chart room destroyed; 34 killed" (EO 793). Two 8-inch shells aft of the bridge, flag plot, 30 killed (EO 794). "Slightly damaged" (EO 779) | Out from 9 Aug (SF 761). Temporary repairs at Rabaul, 10 to 17 Aug (SF 762). Back 17 Aug, with No. 1 turret only partly restored (SF 763). **8 days.** Bates: all three damaged cruisers "returned to service by 17 Aug" (EO 779 note) | Yes / yes / yes, but only as distant cover. She left Shortland 23 Aug "as distant cover for reinforcement convoys" (EF 94; also EF 153, low). No row of her firing |
| Aoba | Shells: 1 hit (EO 125, disputed: Bates lists none). "Machine gun hits" on No. 2 torpedo mount (EO 795). "Fire and steaming unimpaired" (EO 796 note). "Slightly damaged" (EO 780) | Out from 9 Aug (SF 764). Kavieng, 10 to 17 Aug (SF 765). Back 17 Aug (SF 766, low, unconfirmed: the source gives only her departure). **8 days** | Yes / yes / yes. Distant cover at the Eastern Solomons (EF 93, 154). In action at Cape Esperance, 11 Oct, 63 days after |
| Kinugasa | Shells: 2 (EO 140, 141). "No. 1 engine room hit by 5-inch shell; port steering gear hit; one killed" (EO 797). An intermediate-battery hit starboard near the waterline (EO 798). "Remained able to steam and fight" (EO 797 note). "Slightly damaged" (EO 781) | Out from 9 Aug (SF 767). Kavieng, 10 to 17 Aug (SF 768). Back 17 Aug (SF 769, low, unconfirmed, inferred as for Aoba). **8 days** | Yes / yes / yes. Distant cover at the Eastern Solomons (EF 96, 156). In action at Cape Esperance, 63 days after |
| Tenryu | Shells: 1, splinters from Chicago (EO 151, low). "After deck hit by shell splinters; 23 killed; remained operational" (EO 800). "Slightly damaged" (EO 783) | **0 days found**: "no time out of action found" (EO 783 note). Back at Rabaul 10 Aug and escorted a convoy 17 Aug (EO 800 note). Her repair rows SF 745 and 746 are a later B-17 bomb at Rabaul, not Savo | No later engagement row. The convoy escort of 17 Aug is the only later duty in the rows |
| Yubari | Shells: 3 (EO 156, low). "Some scratches from the light fire of an enemy destroyer" (EO 784, 799). "Fully operational" (EO 799) | **0 days found** (EO 782 note) | No later engagement row. To Rabaul 10 Aug, Truk 20 Aug (EO 799 note) |
| USS Patterson | Shells: one 5.5-inch hit (EO 50). "Knocked out her Number 4 gun, killed 10 men ... damaged ... the Number 3 gun"; "stayed in service" (EO 792, 770) | **0 days found.** She reached Noumea 14 Aug "and went to sea at once" (SF 760). No `out_of_action_from` row, by the researcher's choice (SF 760 note) | Possibly within 30: listed in Task Force 11 at the Eastern Solomons (EF 10, low, disputed) |

**Undamaged**

- Furutaka: no hit; a steering casualty at 0145 (EO 147, 785, 786). In action at Cape Esperance, where she was sunk.
- Yunagi: no hit (EO 157, 787).
- USS Bagley: "suffered no damage" (EO 49, 773).
- USS Helm: no hits (EO 93, 774).
- USS Wilson: no hits (EO 96, 775).

Outside the scenario, for completeness: Ralph Talbot was damaged and away 94 days (SF 757 to 759; decision 0069's table). Jarvis was damaged by aircraft on 8 Aug and sunk by aircraft on 9 Aug (`step2-record.md` section 1).

### Eastern Solomons, 24 August 1942 (scenario 5, 36 ships)

The scenario holds 18 US and 18 Japanese ships. It does not hold the reinforcement convoy and its escort (Jintsu, Mutsuki, Kinryu Maru and the others) or Chitose.

**Sunk**

| Ship | Fate | Rows |
| --- | --- | --- |
| Ryujo | Sunk 24 Aug, about 1755 or 1800 Tokyo time. Bomb hits: 4 (combinedfleet, EO 258), 4 claimed (USSBS, EO 259), 10 by a Japanese report (EO 260, 668), or near misses only with no bomb hit given (8th Sentai diary, EO 1176, 1177); all disputed. Torpedoes: 1 (EO 262, 263, 967, 1177). "Machinery and rudder unusable, list over 20 degrees to starboard" (EO 1176) | EO 266 to 274, 666, 667, 969, 1175; SF 527 |

**Damaged**

| Ship | Damage, in the sources' words | Days away | In action again within 30 / 60 / 90 days |
| --- | --- | --- | --- |
| USS Enterprise | Three bomb hits (EO 181 to 184). Near misses 1 to 5, disputed by what each source counts (EO 185 to 189). "No damage to the main propulsion plant"; Nos. 2 and 3 elevators out; Group III 5-inch guns out (EO 206). Deck "again ready to receive aircraft" at 1816 and first plane landed 1819, about 65 minutes after the attack began (EO 203, 205, 656, 657). Steering lost 1850 for 38 minutes; held to 10 knots meanwhile (EO 200 to 202, 204). Fires under control in 40 minutes, recurring until about 0100 (EO 207). BuShips: "Continued to operate aircraft at reduced efficiency; damage repaired in three weeks" (EO 210, disputed) | Out from 24 Aug (SF 613). Tongatabu 30 Aug, for fuel only (SF 614, 615). Pearl Harbor 10 Sep to 16 Oct (SF 616). Back 16 Oct (SF 617). **53 days.** Yard 36 days by DANFS against "three weeks" by BuShips (EO 209, 210, both disputed) | No / no / yes. Santa Cruz, 26 Oct, 63 days after (EO 1261 and others) |
| Shokaku | Two near misses, "one only ten meters away" (EO 281, 374; EO 280 is the US claim, low). "Slight damage to the starboard side", six killed (EO 283, 374) | **Gap.** No `out_of_action_from` or repair row for August 1942 | Yes, within 90: Santa Cruz, 26 Oct (EO 1448, 1449). No row between |
| Kumano | "Port after propeller damaged; three shafts; best speed 31 kn". Cause unknown, "not an enemy hit as written" (EO 1187, 1188) | **Gap.** No repair row | No later engagement row |

**Undamaged**

- US side: the ONI summary lists "Own ships damaged: 1 CV, Enterprise" (EO 664 note). Rows that name the ship: North Carolina, near misses only, "undamaged" (EO 245, 662, 663); Saratoga, "not attacked" (EO 230, 664); Grayson, strafed, "undamaged" (EO 660, 661). Portland, Atlanta, Balch, Benham, Ellet, Maury, Monssen, New Orleans, Minneapolis, Farragut, Macdonough, Worden, Dale and Phelps have no damage row and fall under the ONI summary.
- Japanese side: Tone, "no damage" (EO 1178); Kirishima, "neither ... damaged" (EO 1186). Zuikaku, Chikuma, Hiei, Suzuya, Nagara, Akigumo, Kazagumo, Makigumo, Yugumo, Akizuki, Hatsukaze, Amatsukaze and Tokitsukaze have no damage row. No row says in so many words that they were undamaged.

Outside the scenario, for completeness: Mutsuki and Kinryu Maru were sunk on 25 Aug. Jintsu was away 137 days and Chitose 25 (SF 618 to 627; decision 0069's table). Saratoga was torpedoed 31 Aug (away 71 days, SF 628 to 633) and North Carolina 15 Sep (away 63 days, SF 634 to 637). Both are ships of scenario 5, damaged inside the Rabaul window but not in this battle.

### Rabaul against Guadalcanal, 21 August to 17 September 1942 (scenario 6)

The scenario holds no ships. It has no `[[ship]]` or `[[force]]` table (its own comments at lines 169 to 170, 448 to 449 and 452 say so).

### Cape Esperance, 11 to 12 October 1942 (scenario 17, 14 ships)

The scenario holds 5 Japanese and 9 US ships. It leaves out the reinforcement group (the scenario file, line 15). It was read for its ship list and not run.

**Sunk**

| Ship | Fate | Rows |
| --- | --- | --- |
| Furutaka | Sunk 12 Oct, 0228 local. About 90 shell hits (EO 1600 to 1602, 4035). One torpedo (EO 1603, 1604, both disputed). "No.3 turret jammed; dead in water about 0040" (EO 1579). "Hit in torpedo tubes (oxygen leak, large fire); engine rooms flooded; all turrets out" (EO 1580, low) | EO 1506, 1544, 1559, 1661, 3854; SF 395, 396 |
| Fubuki | Sunk 11 Oct, about 2212 Japanese time. "Foundered from the shellfire", with no magazine explosion (EO 1546, 1581). Count of hits: gap (EO 1701 gives none) | EO 1507, 1561, 1562, 1662; SF 865, 866, 892 |
| USS Duncan | Sunk 12 Oct, about noon, after being abandoned about 0200. She sank of her damage (ONI) or was scuttled by McCalla (Cominch); disputed. Hits: "four or more" (EO 1597) to "at least 14" (EO 1596), from both sides | EO 1532 to 1535, 1563, 1575, 1576; SF 823 to 825 |

**Damaged**

| Ship | Damage, in the sources' words | Days away | In action again within 30 / 60 / 90 days |
| --- | --- | --- | --- |
| Aoba | Hits: "up to forty" or "over 40" (EO 1598, 1599, 4034, disputed; Japanese reports give no count, EO 1510, 1694). "Bridge wrecked; No.2 turret out, No.3 turret destroyed; four boilers off line" (EO 1577). "All turrets unusable; max speed about 32 kn after" (EO 1509). "Hull below waterline and engines intact" (EO 1578, low). Japanese record: 大破, heavy damage (EO 1508). USSBS: "Major" (EO 1548). Many hits were duds (EO 1695, low) | Out from 11 Oct (SF 720). Shortland 12 to 13 Oct (SF 721), Truk 15 to 17 Oct (SF 722), Kure 22 Oct to 15 Feb 1943 (SF 723). Back 15 Feb 1943 (SF 724; also SF 784, low; SF 785 gives 24 Feb, low, disputed). **127 days** (SF 724 note) | No / no / no |
| USS Boise | Hits: 6 known (EO 1587) or 8 (EO 1588, 3965); disputed. "Forward three turrets put out of action by magazine powder fire" (EO 1565). Carter: one turret wrecked by an 8-inch shell, the other two forward turrets "out of commission" (EO 1566). An 8-inch hit "into the 6" magazine"; flooding put the fire out (EO 1567). She made 20 knots once the fires were out (EO 1526). USSBS: "Major" (EO 1527) | Out from 11 Oct (SF 665; SF 770 gives 12 Oct, the hour she fell out of line). Espiritu Santo 13 to 15 Oct (SF 771). Noumea, patching only, "no facilities for turret work" (SF 666). Philadelphia, 19 Nov 1942 to 20 Mar 1943 (SF 667, 772). Back 20 Mar 1943 (SF 668; SF 773, low). **160 days** (SF 668 note) | No / no / no |
| USS Salt Lake City | Hits: 2 (EO 1590, 1592) or 3 (EO 1591, 3971); disputed. "No.1 boiler room out of action; steering failed on the bridge, steered aft; speed 22-25 kn; she kept station" (EO 1569). "Armor-piercing shell into No.1 fireroom, oil fire" (EO 1570). USSBS: "Minor" (EO 1528) | Out from 11 Oct (SF 659). Espiritu Santo 13 to 15 Oct (SF 774), Noumea 17 to 21 Oct (SF 775), Pearl Harbor 1 Nov 1942 to 11 Mar 1943 (SF 776). Carter's Sydney route (SF 661, 662) is disputed by DANFS (SF 663). Back 11 Mar 1943 (SF 777), or "March 1943" by month only (SF 664). **151 days** (SF 777 note), or **141 to 171** from the month | No / no / no |
| USS Farenholt | Hits: 3 (EO 1594) or 4 (EO 1593, 1595); disputed. Part of the fire was from her own side (EO 1724). Three hits on the port side at the waterline; "extensive internal structural damage"; torpedo tubes jammed; heavy list (EO 1571 to 1574). "Holed twice near the water line but ... she was seaworthy" (EO 1530). USSBS: "Major" (EO 1531) | Out from 11 Oct (SF 669). Espiritu Santo 13 to 15 Oct (SF 778), Noumea about 17 to 21 Oct (SF 779, low), Pearl Harbor (SF 780; arrival 1 Nov inferred, SF 781, low). Back 3 Mar 1943 (SF 782; SF 673, low). **143 days** (SF 782 note) | No / no / no |
| Kinugasa | **Disputed whether damaged at all.** Four hits by combinedfleet (EO 1605; EO 4036, low). "Minor" by USSBS and two Wikipedias (EO 1549 to 1551, 1583). The 6th Sentai report and her own signal: she took no fire; "Kinugasa: no damage" (EO 1686 to 1689, high, disputed). One wounded (EO 3857) | **2 days, or none.** At Shortland 12 Oct; sailed again 13 Oct with no repair period (SF 786) | Yes, within 30: she shelled Henderson Field 15 Oct (EO 1582 note; SF 786 note). Sunk 14 Nov (SF 463, 464) |
| Hatsuyuki | **Disputed.** "Hit forward above the waterline; unsafe above 24 kn or in heavy weather" (3rd Destroyer Squadron signal of 15 Oct, EO 1513, 1702, high). Her own tabular record mentions no damage (EO 1553, low). "Damaged", no detail (EO 1552, 1584) | **Gap.** No out-of-action row. "Returned" 13 Oct (SF 787, low, unconfirmed): she rescued 518 of Furutaka's men and escorted Aoba to Truk 13 to 15 Oct (EO 1553) | No later engagement row |

**Undamaged.** Each of these rests on its absence from a list of damaged ships, not on a source that says "undamaged".

- San Francisco (EO 1536, 1537).
- Helena (EO 1538).
- Laffey (EO 1540; ONI has "no official record" of her part).
- Buchanan (EO 1539, 1543).
- McCalla (EO 1541; "no serious damage", EO 1542, low).

Outside the scenario: Murakumo and Natsugumo were sunk by aircraft the next day (the measures file's comment, `cape_esperance.toml` line 18; EO 1662).

## 2. Aircraft lost for good

This part is taken from the measures files. It is not re-researched. The scale's `aircraft_destroyed` counts aircraft "lost for good, by any cause" (decision 0054). The measures' cause sets are as each measure states them. Every cited row was opened and matches its measure.

| Battle | Side | Measure (file, line) | Record | What it counts | Rows |
| --- | --- | --- | --- | --- | --- |
| Savo Island | both | **None.** No aircraft-loss measure in `savo_island.toml`. Floatplanes and the air attack of 8 Aug are not counted | gap | | |
| Eastern Solomons | Japanese | `japanese_carrier_aircraft_lost_all_causes` (`eastern_solomons.toml` 118), judged | **35 to 54** | All seven causes, including 4 lost with Ryujo, for the carriers' air groups to 18:00 on 24 Aug. Floatplanes not counted | 291, 1172, 289, 270, 293, 974 (974 low, disputed, read as the high end by exception) |
| | Japanese | `japanese_carrier_aircraft_lost_to_enemy_action` (106) | 29 to 32 | Shot down only | 291, 287, 288, 1165, 1166, 296, 1172, 271 |
| | Japanese | `ryujo_aircraft_lost_with_the_ship` (130), reported | 4 | Lost with the ship | 270 |
| | US | `us_aircraft_lost_all_causes` (154), judged | **20 to 21** | All seven causes; air groups 1 to 10, 25, 26 | 305, 306, 307, 700 (all disputed) |
| | US | `us_aircraft_lost_to_enemy_action` (142) | 13 to 14 | Shot down only | 305, 311, 700 |
| Rabaul against Guadalcanal | Japanese | `japanese_bombers_lost_to_enemy_action` (31) | 12 | G4Ms shot down | 1048, 1060, 1020, 1024, 1095, 1101, 1034, 1109 |
| | Japanese | `japanese_fighters_lost_to_enemy_action` (43) | 6 to 10 | Tainan and 6th Air Group fighters shot down. The high end adds 4 lost escorting a reconnaissance flight the scenario does not fly | 1202, 1216, 1140, 1231 |
| | Japanese | `japanese_bombers_forced_down` (55), reported | 5 to 6 | Wrecked in forced landings; a floor | 988, 1049, 1061, 1090, 1110, 1102 (low) |
| | US | `us_fighters_lost_to_enemy_aircraft` (69) | 7 to 23 | F4Fs. The low end is August only. The high end is all of September, a ceiling | 1263, 1269, 1285 |
| | US | `us_aircraft_destroyed_on_the_ground` (81) | 2 to 3 | F4Fs and SBDs | 1266, 1272, 1283, 1299, 1330 |
| Cape Esperance | both | **None.** No aircraft-loss measure in `cape_esperance.toml` | gap | | |

At Rabaul, no measure counts "lost for good by any cause" for either side. The pieces above are by cause and by type. Gaps for that battle:

- US losses all causes.
- US bombers lost.
- Japanese fighters lost to other causes.
- Japanese aircraft destroyed on the ground.

`step2-close.md` names F4F write-offs of "about 6.1" as reported, not judged. The file's header puts floatplanes, the 2nd Air Group, Shokaku's fighters at Buka and the Army's P-400s outside its scope.

## 3. Rabaul against Guadalcanal: cargo landed and Henderson Field open

### What the scenario's objectives use today

`base_open` for base 1, Henderson Field, worth 10 to each side: the Allies want it open, the Japanese want it closed (`closed = true`). Both sides also have `aircraft_destroyed` at 0.05. There is no `cargo_landed` objective. The scenario has no ships and no cargo. Its only arrivals are aircraft, and its supply is a starting stock (`supply_t = 4000` at Henderson Field, 10,000 at Rabaul).

### US cargo landed at Guadalcanal, 21 Aug to 17 Sep

| Date | What | Rows |
| --- | --- | --- |
| 21 Aug | 120 tons of rations, 3.5 days' supply, by Colhoun, Gregory and Little. Arrival at Tulagi or Lunga is unclear (EO 1893 note) | EO 822 (medium), 1893 (high); EO 904 (low), 1894 |
| 21 to 22 Aug | "Over 2,000 tons of logistic support including 200 tons of rations" (Alhena, Fomalhaut and others). Not reconciled with the 120 tons above | TLF 60 (medium, unconfirmed) |
| 22 Aug | Alhena "with supplies and a part of the 2d Marines", no quantity | TLF 54 (low) |
| 30 Aug | About 17 tons of stores, Colhoun; unloaded at Kukum, or still aboard when she was sunk (the sources disagree) | EO 823 (high), 824 |
| 29 to 31 Aug | Burrows, aground at Tulagi, 60 per cent unloaded. That is Tulagi, not Lunga | EO 1895 |
| 1 Sep | 392 Seabees (EO 825, 826) or 200 (EO 827), all disputed; two bulldozers | EO 825 to 827; TLF 58, 59 |
| 16 Sep | Aviation fuel drums moved from Tulagi to Lunga by YPs: 212 sent that morning, 108 loaded that evening, 161 to follow on the 17th | EO 1899 to 1902 (medium) |
| 18 Sep, a day after the window | 1,012 tons of rations, 3,823 drums of fuel, 147 vehicles, 4,180 to 4,262 men | EO 831, 832, 492, 833, 828 to 830, 1903 |

No row gives US tonnage for:

- 22 to 29 Aug.
- 31 Aug.
- 2 to 15 Sep.
- Alhena and Fomalhaut's runs.
- The run of 29 Aug.
- Bellatrix and Fuller on 7 and 14 Sep.
- Betelgeuse's cargo.

The gaps rows are lines 614, 652, 653 and 1019. No row gives a stock of fuel at Lunga by date: only a four-day reserve in mid-September (EO 493; gaps lines 560, 654, 1024).

Japanese cargo: **no row gives tons for any Japanese run** (gaps lines 612, 661). Troops landed are in EO 837 to 854 and TLF 7 to 9 and 45 to 53. They are all disputed or weak beyond the 900 of 29 Aug (EO 846) and the 1,200 of 31 Aug (EO 847).

### Henderson Field open or closed, by day

There is no daily series (gaps lines 589, 656, 663, 749, 752). Every row on the field's state:

| Day | Row | State |
| --- | --- | --- |
| 18 to 19 Aug, before the window | EO 891 (medium) | Closed: 17 bomb hits on the runway |
| 23 Aug | EO 1907 (low) | Night work on the field, with lights |
| 6 to 7 Sep | EO 1820 note | Rain |
| 8 Sep | EO 1820 (low, unconfirmed) | "Airfield in bad condition following two days rain". Limited, not closed (the row's note) |
| 9 Sep | EO 1341 (medium) | "Practically inoperative": short of Marston mat and gasoline, and aircraft crashed |
| 9 Sep | EO 1828 (high) | Fighter One "placed in commission". Also EO 905 (low) |
| 10 Sep | EO 1342 to 1344 (high) | 36 aircraft flyable (11 F4F, 22 SBD, 3 P-400), as reported for "yesterday" in a dispatch of the 11th |
| 20 Aug to 30 Sep | PF 70, 75 (high) | Mud restricted operations; fighters flew only when the surface was dry. No day named |
| Sep to Nov | PF 72 (high); EO 892 (low, unconfirmed) | The Seabees' claim that the field was "never out of operation for more than four hours" |

Aircraft-ready counts exist for 25, 29 and 30 Aug, 1, 3, 4, 9 and 10 Sep, and about 16 to 24 Sep (EO 455, 549, 3959, 568 to 570, 893, 894, 1315 to 1317, 1329, 606, 1342 to 1344, 1309). Each is low for its day, but none is zero for the whole field. No row gives the field's state on 17 Sep, the scenario's last day.

## 4. The scenarios' objectives today, and where the runs fall

### What the files say

The levels are the same in all four files: `minor = 3`, `major = 8`, `decisive = 16`. Each file's comment says: "The points and the levels are the placeholder scale of that decision, to be set from the record in step 2" (`04_savo_island.toml` lines 333 to 334). Where a file gives ship points, they are the same for both sides in all three ship scenarios:

```toml
kind = "ships_sunk"
points = { carrier = 10, battleship = 8, cruiser = 4, destroyer = 1, transport = 2, auxiliary = 1 }

kind = "ships_crippled"
points = { carrier = 5, battleship = 4, cruiser = 2, destroyer = "0.5", transport = 1, auxiliary = "0.5" }
```

| Scenario | Kinds, each side | `history`, quoted |
| --- | --- | --- |
| 4 Savo Island | `ships_sunk`, `ships_crippled` | "Four Allied heavy cruisers were sunk for no Japanese ship lost that night: a decisive Japanese victory, though the transports the screen guarded were left untouched. (The architect's reading; not yet scored from the order-of-battle facts.)" |
| 5 Eastern Solomons | `ships_sunk`, `ships_crippled`, `aircraft_destroyed` (`points = "0.05"`) | "Ryujo was sunk and Enterprise was damaged and withdrew for repair; Japanese aircraft losses were several times the American. Usually judged a limited American victory. (The architect's reading; not yet scored from the order-of-battle facts.)" |
| 6 Rabaul | `aircraft_destroyed` (0.05); `base_open`, base 1, 10 points (the Japanese with `closed = true`) | "Henderson Field never stopped flying in these four weeks, and Rabaul's raids cost it more than it could replace. (The architect's reading; not yet scored from the order-of-battle facts.)" |
| 17 Cape Esperance | `ships_sunk`, `ships_crippled` | "Two Japanese ships, the heavy cruiser Furutaka and the destroyer Fubuki, were sunk that night for one American destroyer, Duncan; Aoba was badly damaged and the bombardment of Henderson Field was not carried out. (From the order-of-battle facts; not yet scored from them.)" |

The missions are in each file's `[objectives.allied]` and `[objectives.japanese]` tables. None names a horizon, since `horizon_days` is not built (decision 0088, "As planned for the build", on branch `out-of-action-build-spec`). *(Nov 14, 2026: `horizon_days` is built, decision 0088, scenario 20b.)*

### The runs: 1,000 runs each, seeds 1 to 1000, seen from the Allied side

| Battle | Decisive victory | Major victory | Minor victory | Stalemate | Minor defeat | Major defeat | Decisive defeat | Allied margin, 5th / median / 95th | Report |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Savo Island | 0 | 3 | 11 | 71 | 143 | 364 | 408 | -21 / -13 / 0 | `savo-island-close.md` lines 116 to 128 |
| Eastern Solomons | 0 | 218 | 211 | 502 | 69 | 0 | 0 | -3.75 / 1.55 / 12.5 | `eastern-solomons-close.md` lines 76 to 88 |
| Rabaul | 0 | 1,000 | 0 | 0 | 0 | 0 | 0 | 10.25 / 11 / 11.75 | `rabaul-guadalcanal-air-close.md` lines 67 to 79 |
| Cape Esperance | 0 | 449 | 393 | 146 | 11 | 1 | 0 | 0 / 6 / 12 | `cape-esperance-check.md` lines 67 to 79 |

### History on the same placeholder scale, as arithmetic only

This is for orientation and is not a proposal. It counts only the ships the scenario holds and the day it covers. "Crippled" uses decision 0054's test (propulsion or flight deck out at the end), read against the sources' words above.

- **Savo Island.** The Japanese score 4 cruisers sunk x 4 = 16. By `step2-record.md` section 1 no Allied ship is crippled: Chicago steamed, and Patterson stayed in service. The Allies score 0 if Kako, sunk the next day outside the action, is not counted, or 4 if she is. The margin is **-16, a decisive Japanese victory**, or **-12, a major one**. In the closing runs a decisive Allied defeat is 408 of 1,000 and a major one 364. (`baseline-reading.md` line 31, "one run in 1,000 is", is about the first baseline, not the closing report.)
- **Eastern Solomons.** The Allies score Ryujo 10, plus 35 to 54 aircraft x 0.05 = 1.75 to 2.7. The Japanese score 20 to 21 aircraft x 0.05 = 1.0 to 1.05. Enterprise is not crippled by the test (`step2-record.md`). The margin is **+10.7 to +11.7, a major Allied victory**, which is 218 runs in 1,000. Mutsuki and Kinryu Maru are not in the scenario.
- **Rabaul.** The Allies score 10 for `base_open`, if the field counts as open on 17 Sep: no row gives that day, and the nearest is 36 flyable on 10 Sep with a few days' fuel in mid-September. They add 23 to 28 Japanese aircraft x 0.05 = 1.15 to 1.4: the bombers and fighters shot down and the bombers forced down, the only Japanese losses measured. The Japanese score 9 to 26 US aircraft x 0.05 = 0.45 to 1.3: the fighters shot down and the aircraft destroyed on the ground; US losses all causes are not measured. The margin is about **+9.85 to +10.95, a major Allied victory**, as in every run. It rests on the gaps named in part 2.
- **Cape Esperance.** The Allies score Furutaka 4 and Fubuki 1, which is 5. The Japanese score Duncan 1. No ship is crippled on my reading: Aoba made about 32 knots (EO 1509), Boise 20 (EO 1526), Salt Lake City 22 to 25 (EO 1569), and Farenholt steamed to Espiritu Santo under her own power (EO 1530, 1572). The margin is **+4, a minor Allied victory**, which is 393 runs in 1,000.

## 5. How historians judge each battle

Only what the repository holds. No new research was done.

| Battle | Judgement in the repository | Who says it |
| --- | --- | --- |
| Savo Island | "a decisive Japanese victory" | **The architect's reading** (scenario 4's `history`; repeated in `baseline-reading.md` line 31). No fact row states a verdict. Bates and Innis, *The Battle of Savo Island ... Strategical and Tactical Analysis* (Naval War College, 1950; `sources.csv` 1001), is cited for facts only |
| Eastern Solomons | "Usually judged a limited American victory" | **The architect's reading** (scenario 5's `history`; `docs/step2-spec.md` line 42). No historian is named and no row states a verdict |
| Rabaul against Guadalcanal | None | No source in the repository judges the four weeks. Scenario 6's `history` is a description by the architect, with no verdict |
| Cape Esperance | "the American victory of Cape Esperance" | SF 665, on the source `danfs` (Boise's entry), though the words sit in the row's `value` column and may be the researcher's phrasing. Two cited sources carry "victory" in their titles only: NHHC, Samuel J. Cox, *H-Gram 011-1: Guadalcanal: Victory at Cape Esperance* (2019; `sources.csv` 1126), and Lippman, "The First Victory" (Warfare History Network, grade C; `sources.csv` 1065, EO 1612). No row quotes either text for a verdict |

## 6. The operation and its length, for the horizon

| Date | What | Rows |
| --- | --- | --- |
| 7 Aug 1942 | The landings on Guadalcanal and Tulagi | LUF 475 (Hough, 1st Marines ashore from 0930), 476, 602, 369, 511, 512, 533, 488; `sources.csv` 1173, the Marine report's Phase II "the landings, 7 to 9 August 1942" |
| 1 to 2, 4 and 7 Feb 1943 | The evacuation's three runs: 4,940, 3,902 and 1,730 men. "In the final evacuation on 7 February" | TLF 32, 33, 34 (MacArthur reports, ch. VIII p. 196); LUF 101 (9,800 Army and 830 Navy, 1 to 7 Feb; Japanese Monograph 45, p. 123), 103 |
| 7 Aug 1942 to 9 Feb 1943 | The validity range of a "whole campaign" figure (Japanese losses close to 25,000). The end date is the researcher's range, not stated as the campaign's end | LUF 104 (Shaw, p. 52) |
| 16 Nov 1942 to 9 Feb 1943 | "The Express made about eleven trips to Guadalcanal" | TLF 49 (Miller, ch. IX fn. 51) |

From the landing to the final evacuation run is 184 days, 7 Aug 1942 to 7 Feb 1943 (my arithmetic). A row-backed end of 9 Feb 1943 rests only on LUF 104's range and TLF 49.

**Gap:** the Japanese decision to evacuate (about 31 Dec 1942 or 4 Jan 1943). No row gives it. No row names Operation Watchtower or Operation KE.

**Shorter periods a source names.** These are the 1st Marine Division's own phases (`sources.csv`):

| Phase | Dates | Days (mine) | Source |
| --- | --- | --- | --- |
| I | Before 7 Aug | | 1172 |
| II | 7 to 9 Aug | 3 | 1173 |
| III | 10 to 21 Aug | 12 | 1174 |
| IV | 20 Aug to 18 Sep | 30 | 1157 |
| V | 18 Sep to 5 Dec 1942 | 79 | 1158 |

The Rabaul window, 21 Aug to 17 Sep (28 days, `engagements.csv` 1002), is the project's, chosen by decision 0042. It nearly coincides with Phase IV. No source names a period such as "the struggle for Henderson Field" or "the first phase" in a row.

## 7. What cannot be put on the scale from the record

**Ships whose days away are unknown or only bounded:**

- Chicago (Savo): month only, 145 to 175 days.
- Aoba and Kinugasa after Savo: 8 days, inferred from the departure date (SF 766, 769, low, unconfirmed).
- Patterson (Savo): no out-of-action row. She "went to sea at once".
- Tenryu and Yubari (Savo): no time out found.
- Shokaku (Eastern Solomons): slight damage, no row.
- Kumano (Eastern Solomons): propeller damage not by the enemy, no row.
- Kinugasa (Cape Esperance): whether she was damaged at all is disputed; 2 days or none.
- Hatsuyuki (Cape Esperance): damage disputed. Her return is unconfirmed (SF 787).
- Salt Lake City (Cape Esperance): 151 days by one row, 141 to 171 by the month.
- Enterprise: the 53 days are firm. Her yard time is 36 days by DANFS against "three weeks" by BuShips (disputed).

**Damage the model's terms cannot be read from** (flotation points and five systems). No source gives flotation points. Decision 0069's section already lists Aoba, Boise, Farenholt and Ralph Talbot as unreadable.

- Aoba at Cape Esperance: "up to"/"over" 40 hits, many of them duds. In the model 40 hits would sink her several times over.
- Boise: 6 or 8 hits, disputed. "Forward three turrets out" is part of a main battery, and the model has the battery as one system.
- Farenholt: 3 or 4 hits, some from her own side. Decision 0069 lists her as having no derived class; scenario 17 now gives her class 3003 from `ship_classes_cape_esperance.csv`.
- Ryujo: 0 to 10 bomb hits, disputed. She sank, so this matters only for a hit measure.
- Chokai: a turret "partly restored", with one gun of two working, after 8 days.
- Hatsuyuki: a hole forward above the waterline, a speed and weather limit, and no hit count.
- Chicago: the one torpedo is readable only by decision 0069's assumption A2, since a ship torpedo has no figure for the model.
- Enterprise: "Group III 5-inch guns out" and two elevators out have no system in the model. The flight deck was out about 65 minutes and steering 38 minutes.

**Aircraft:**

- Savo Island and Cape Esperance have no aircraft-loss measure at all.
- At Rabaul no measure counts losses by every cause, for either side. US losses all causes, US bombers and Japanese aircraft on the ground are gaps.
- At the Eastern Solomons the Japanese high end of 54 rests on a low, disputed row (EO 974).

**The record's judgement that the scale has no kind for:**

- Savo Island: the transports the screen guarded were left untouched (decision 0054, "What it costs"). The Allied mission was to protect them, and they are not in the scenario.
- Eastern Solomons: the Japanese convoy turned back and landed no troops on 25 Aug (EO 842). The scenario holds no convoy, so `cargo_landed` cannot see it, and no source gives its tonnage.
- Cape Esperance: the bombardment of Henderson Field was not carried out (scenario 17's `history`). The scale has no kind for a mission prevented, and the reinforcement group, which did land, is left out of the scenario.
- Rabaul: Japanese troops landed on Guadalcanal in the window (900, 1,200 and others), but no row gives tons and there is no ground combat. The `base_open` objective reads one day only, the last, and no row gives the field's state on 17 Sep.
- Ships lost the day after or outside the scenario's ship list: Kako, Jarvis, Mutsuki, Kinryu Maru, Murakumo and Natsugumo. A day-long scenario cannot score them.
- Men killed and wounded are recorded in many rows but are no kind on the scale.
- Days away cannot be scored until `ships_out_of_action` is built (decision 0088). Enterprise's 53 days, Aoba's 127, Boise's 160, Salt Lake City's about 151 and Farenholt's 143 score nothing today. *(Nov 14, 2026: `ships_out_of_action` is built, decision 0088, scenario 20b.)*

### Rows worth adding

None of these sources has been read for this note. They are places to look, in sources the repository already cites:

- **A stated verdict on Savo Island.** Look in Bates and Innis (`bates-savo-1950`), whose conclusions chapters the rows cite for other facts.
- **A stated verdict on Cape Esperance.** Look in the text of the NHHC H-Gram 011 (`nhhc-hgram-011`) and in Boise's DANFS entry, to check whether SF 665's "American victory" is DANFS's words.
- **The date of the Japanese decision to evacuate.** Look in the MacArthur reports, ch. VIII "Evacuation of Guadalcanal" (`macarthur-reports-v2`, around p. 196, already the locator of TLF 29 to 34), and in Japanese Monograph 45, Part 2 (`japanese-monograph-45`, around p. 123).
