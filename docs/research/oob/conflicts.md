# Order-of-battle conflicts between sources

Every place the first research pass found sources disagreeing, or a source disagreeing with itself. Nothing here has been settled. In the data, the rows concerned are marked `disputed`, or `low` confidence with a note. Read on 2026-11-10.

## The ones that matter most for scenarios 4 to 6

- **Which Allied ships were where at Savo Island.** The 1943 Navy narrative and the 1950 Naval War College analysis give different groupings for Australia, Jarvis, Hobart and four destroyers. Mostly this is organisation on paper against presence on the night, but it decides the scenario's starting positions.
- **Which radar each ship carried.** Only seven Allied ships at Savo have a sourced set, and Canberra's is disputed (Type 271 or Type 241). No source read gives a radar fit for Vincennes, Quincy, Australia, Hobart or six of the destroyers.
- **Japanese shipboard radar in August 1942.** One Japanese ship-movement record puts an air search set on Shokaku from June 1942 and has it detecting aircraft on 24 August. A recent article says there is no firm evidence of Japanese shipborne radar before late 1943. The same reference site contradicts itself on Ise, Hyuga and Kirishima.
- **Who was at Eastern Solomons.** The Navy narrative and Wikipedia (citing Morison) differ on the Saratoga group's destroyers and on whether Australia, Hobart and San Juan were present. On the Japanese side, three destroyers Wikipedia lists have movement records placing them elsewhere, and the seaplane carrier Chitose is missing from Wikipedia's list.
- **Carrier aircraft numbers on 24 August.** The Navy narrative gives 54 fighters for both US carriers together; per-squadron figures come only from Wikipedia and total 56. Zuikaku's dive bombers are 27 or 18 depending on source.
- **Type 93 torpedo settings.** Three sources give three sets of speed and range figures. They agree on the warhead.
- **Zero performance.** Eleven different maximum speeds for the Model 21 are in circulation.
- **Chokai's anti-aircraft guns in August 1942.** Three sources, three answers.
- **Japanese air strength at Rabaul.** Unit arrival dates and numbers differ between sources for the 2nd, 6th and Kisarazu air groups, and almost all of it rests on Wikipedia.

## Savo Island order of battle

- **HMAS Australia, present or not.** ONI chapter 2 (https://ibiblio.org/hyperwar/USN/USN-CN-Savo/USN-CN-Savo-2.html) says she was the Southern Group flagship but was absent that night, having taken Crutchley to a conference aboard McCawley. Bates (https://www.ibiblio.org/hyperwar/USN/rep/Savo/NWC/NWC-Savo-Fwd.html) lists the "Australia Group" as Australia, Chicago, Canberra, Patterson, Bagley, which is the organisation, not a statement of presence. Both rows are marked disputed in `engagement_forces.csv`.
- **Jarvis.** ONI chapter 2 says she was torpedoed in the 8 Aug air attack and replaced by Wilson in the Northern Group. Bates chapter III (https://www.ibiblio.org/hyperwar/USN/rep/Savo/NWC/NWC-Savo-3.html) lists her in DesRon 4 under TG 62.6 with no mention of damage. DANFS (https://www.ibiblio.org/hyperwar/USN/ships/dafs/DD/dd393.html) says she left Tulagi at midnight, steamed west between Savo and Cape Esperance, and was sunk by aircraft about 1300 on 9 Aug.
- **Yunagi and Jarvis.** combinedfleet (http://www.combinedfleet.com/yunagi_t.htm) says Yunagi "probably engaged" Jarvis. That fits DANFS only if Jarvis was passing west that night; no other source read confirms the engagement.
- **Wilson, Dewey, Ellet, Hull placement.** ONI puts Wilson with the Northern Group, Dewey with the Guadalcanal screen, Ellet and Henley with the Tulagi screen, and mentions Hull only in the transport-sinking account. Bates chapter III puts Quincy, Vincennes, Astoria, Hull, Dewey, Ellet and Wilson in TG 62.3 Fire Support Group LOVE (and notes "4 DD" for seven names). Bates also puts Henley with Chicago and Canberra on 7 Aug and Selfridge, Jarvis, Mugford and Ralph Talbot on the XRAY anti-submarine screen. These are different dates and groupings more than hard contradictions, but Hull is marked disputed because ONI gives her no station.
- **Gregory (APD-3).** Bates lists Gregory in Transport Group YOKE with Colhoun, Little and McKean. ONI's Tulagi screen names only Colhoun, Little and McKean. Marked disputed on the Bates row.
- **Hobart's group.** Bates chapter III puts Hobart in TG 62.6 with Australia, Canberra and Chicago; ONI puts her in the eastern patrol with San Juan. Bates' narrative also calls the eastern group the San Juan Group (CTG 62.4), and Fire Support Group MIKE (TG 62.4) is San Juan, Monssen, Buchanan.
- **Group names.** ONI chapter titles use "Southern Group" and "Northern Group"; Bates uses "Australia Group", "Vincennes Group", "San Juan Group" and "Western Screening Group". The label "Eastern Group" is the research brief's, not a phrase confirmed in ONI.
- **Southard hull.** ONI lists Southard as DD-207; uboat.net and the Bates minesweeper group treat her as DMS-10. Same ship, hull number changed with reclassification.
- **Patterson laid down.** DANFS (dd392) says 23 July 1933; uboat.net says 23 July 1935 (and the other Bagley-class ships were laid down in 1935). Both rows disputed.
- **Gregory launched.** DANFS (apd3) says 27 January 1915; uboat.net says 27 January 1918, with commissioning 1 June 1918 in both.
- **Class of DD-386 to 393 and 398.** DANFS entries read say "Gridley" for Bagley, Blue, Helm, Ralph Talbot, Henley, Patterson, Jarvis, Ellet; uboat.net groups 386-393 as "Bagley" and Ellet and Wilson as "Benham". `ships.csv` follows the research brief's keys (us-bagley-class, us-benham-class). Needs a decision.
- **Japanese heavy cruiser classes.** combinedfleet's Junyokan page (https://www.combinedfleet.com/Junyokan.htm) lists Aoba, Kako, Kinugasa and Furutaka all as "Furutaka Class". The research brief has a separate `jp-aoba-class`. `ships.csv` puts Aoba and Kinugasa in `jp-aoba-class` and Furutaka and Kako in `jp-furutaka-class`, following the class pages used for the class data. Needs a decision.
- **Who scuttled Canberra.** uboat.net's Canberra page says torpedoes from USS Ellet; its Selfridge page says Selfridge scuttled her. Not resolved.
- **San Juan decommissioning.** DANFS says 9 Nov 1946; uboat.net says 8 Nov 1946. Stored only in the fate text.
- **Bates spelling and misprints.** "Tenryo", "Tubari", "Thever", "Zane Southard", "Tsukuhara"; read as Tenryu, Yubari, Trever, Zane and Southard, Tsukahara.
- **Hobart commissioning.** uboat.net gives 28 Sep 1938 as RAN commissioning date; she was first commissioned in the Royal Navy as Apollo on 13 Jan 1936. Both dates are recorded in a note, not a conflict.

## Eastern Solomons order of battle

- **TF 11 destroyers.** ONI lists Phelps, Farragut, Worden, MacDonough, Dale (Task Force FOX): https://www.ibiblio.org/hyperwar/USN/USN-CN-ESols/USN-CN-ESolomons-2.html . Wikipedia lists Phelps, Farragut, MacDonough, Worden, Dewey, Bagley, Patterson (citing Morison pp.86-87 and Dewey's war diary): https://en.wikipedia.org/wiki/Eastern_Solomons_order_of_battle . ONI's totals (about 18 destroyers across FOX, KING and the Wasp group: 5+6+7) fit its own list; Wikipedia's give 20.
- **Patterson.** Wikipedia puts her in TF 11. ONI's summary of events says Grayson, Patterson and Monssen sank a submarine; Grayson and Monssen are in KING per ONI (https://www.ibiblio.org/hyperwar/USN/USN-CN-ESols/USN-CN-ESolomons-7.html). Formation not settled.
- **HMAS Australia and Hobart.** Wikipedia lists both in TF 11. ONI's list and its count of heavy cruisers (5 across all three groups) and one light cruiser leave no room for them. Not checked against navy.gov.au (timed out).
- **Wasp group.** ONI: Wasp with two heavy cruisers and seven destroyers, none named but Wasp. Wikipedia adds Salt Lake City, San Francisco, San Juan and seven named destroyers. ONI's 'one light cruiser' total is Atlanta, so San Juan is in doubt.
- **Fighter numbers.** ONI says the two carriers had 54 F4F-4s combined (USN-CN-ESolomons-4.html). Wikipedia lists VF-5 27 and VF-6 29, total 56 (uncited).
- **Saratoga strike size.** ONI: 30 dive bombers and 8 torpedo planes launched at 1435, one of each turned back. Wikipedia (citing Lundstrom p.122): 29 SBD-3 and 7 TBF-1. These are launch counts, not on-hand counts.
- **Zuikaku dive bombers.** Wikipedia: 27 D3A. USSBS Okumiya interrogation (https://www.ibiblio.org/hyperwar/AAF/USSBS/IJO/IJO-16.html): 18 dive bombers normal complement (Shokaku 27). The USSBS figure is a normal complement, not a 24 Aug count.
- **Wikipedia's own totals.** Its summary line (69 fighters, 41 dive bombers, 57 torpedo bombers for Japan) does not match its listed units (78/54/45); not used.
- **Vanguard vs Main Body.** Wikipedia puts Hiei, Kirishima, Kumano, Suzuya, Chikuma, Nagara in Abe's Vanguard Group. combinedfleet's 24 Aug entries put them with Nagumo's Main Body 'cruising NE of Guadalcanal' (e.g. https://www.combinedfleet.com/Kirishima.htm). May be naming rather than disagreement.
- **Akizuki and Hatsukaze.** Wikipedia: Vanguard Group, Desron 10. combinedfleet (akizuk_t.htm, hatsuk_t.htm): escorted Nagumo's Striking Force.
- **Yamagumo, Yukikaze, Tanikaze.** Wikipedia places them at the battle. combinedfleet: Yamagumo with Desron 11 training (no 24 Aug entry); Yukikaze no 23-25 Aug entry (training 13-20 Aug); Tanikaze 'with Milne Bay invasion force' on 25 Aug.
- **Minegumo and Natsugumo.** combinedfleet's Chitose page lists them, with Asagumo, Kuroshio, Hayashio, Oyashio and Yura, in Takama's screen. Wikipedia omits both and lists Yamagumo instead.
- **Shikinami and Uranami.** Wikipedia: Nagumo's screen. combinedfleet: escorted a fleet supply group; Uranami reached Rabaul 29 Aug.
- **Uzuki.** Wikipedia: Tanaka's Escort Unit. combinedfleet's Uzuki page: a transport run Rabaul to Guadalcanal 24-25 Aug.
- **Yura.** combinedfleet's Yura page puts her with Kondo/Nagumo (Desron 4), while the Chokai page says her floatplanes bombed Lunga on 24-25 Aug with Chokai's.
- **Chitose.** Absent from Wikipedia's order of battle. combinedfleet places her in Kondo's Advance Force and says the 'Mutsu-class battleship' attacked by Saratoga SBDs (also ONI's 'Mutsu', ONI page 4 and summary) was actually Chitose.
- **Minneapolis keel date.** The DANFS HyperWar copy as fetched gives 27 June 1921, but launch is 1933; probably a typo. Recorded as stated, confidence low.
- **Ryujo aircraft.** combinedfleet (16 Aug) and Wikipedia (24 Aug) agree: 24 A6M2 and 9 B5N2.

## Land-based air units

- VMF-223 aircraft on 20 Aug 1942: 19 F4F-4 per Shaw (https://www.ibiblio.org/hyperwar/USMC/USMC-C-Guadalcanal/index.html), Miller ch V (https://www.ibiblio.org/hyperwar/USA/USA-P-Guadalcanal/USA-P-Guadalcanal-5.html, "31 aircraft" total) and Pacific Wrecks (https://pacificwrecks.com/unit/usmc/VMF-223.html); 18 per Wikipedia Cactus Air Force (https://en.wikipedia.org/wiki/Cactus_Air_Force). Wikipedia also gives the leader as Maj. Smith where Shaw says Capt.
- VMF-224 first mission 31 Aug: 17 F4F-4 took off (VMF-224 page) vs 18 (VMF-223 page), both https://pacificwrecks.com/unit/usmc/VMF-224.html and /VMF-223.html; 19 arrived 30 Aug.
- VS-3 / VT-8 arrival: 13 Sep per Wikipedia Cactus Air Force; Miller ch V says 6 TBF and 12 dive bombers arrived 12 Sep (units not named there).
- Japanese seaplanes destroyed at Tulagi 7 Aug: 15 per Wikipedia Yokohama Air Group (https://en.wikipedia.org/wiki/Yokohama_Air_Group) and Cactus Air Force (15 of 17); 18 per Miller ch III (https://www.ibiblio.org/hyperwar/USA/USA-P-Guadalcanal/USA-P-Guadalcanal-3.html).
- Japanese bombers on 7 Aug: Wikipedia 4th Air Group says 27 G4M sent; Miller ch III says 25 twin-engine bombers attacked; Wikipedia Cactus gives 32 medium bombers in the 5th Air Attack Force. Different measures, not reconciled. Wikipedia 4th Air Group note on Misawa: 9 + 18 G4M = 27 arriving 7-8 Aug, which matches Cactus article.
- Total Japanese fighters 7 Aug: Cactus article says 39; Tainan 24 (Wikipedia Tainan) + 15 A6M3 of 2nd Ku (Wikipedia 2nd Air Group) = 39, consistent (our arithmetic).
- Tainan Ku base: Lakunai (Pacific Wrecks Tainan page, https://pacificwrecks.com/unit/ijn/tainan/index.html) vs Vunakanau and Lakunai (https://pacificwrecks.com/airfield/png/vunakanau/index.html) vs Lae detachment only (https://pacificwrecks.com/airfield/png/lakunai/index.html) vs "Rabaul" (Wikipedia Tainan).
- 2nd Ku arrival: Rabaul 6 Aug with 16 D3A + 15 A6M3 (Wikipedia 2nd Air Group) vs delivered by Yawata Maru "between 6 and 22 Aug" (Pacific Wrecks Lakunai page).
- 6th Ku: end of Aug (Wikipedia 6th Air Group); 31 Aug 13 Zeros joined 2nd Ku (Cactus article citing Miller); Sept (Pacific Wrecks Lakunai). Ferry of 18 Zeros: 13 arrived (Lundstrom per Wikipedia) vs all 18 (Kofukuda, Hata per Wikipedia).
- Kisarazu Ku: arrived Kavieng 20 Aug (Cactus article) vs 19 G4M1 arrived Rabaul 22 Aug (Wikipedia Kisarazu Air Group).
- Ndeni PBY tenders and units: Pacific Wrecks (https://pacificwrecks.com/location/solomons_graciosa_bay.html) says VP-23 and VP-11 with USS Mackinac and Ballard; Wikipedia VPB-23 says Mackinac supported the Malaita detachment, nine planes 7-9 Aug, six planes to Nendo 10 Aug (tender not named); Wikipedia VPB-11 shows Curtiss (Noumea to Espiritu Santo 11 Aug) and McFarland (Saweni Beach HQ), no Ndeni.
- Cactus strength: 20 serviceable 25 Aug (Miller ch V); 64 flyable 3 Sep (Wikipedia Cactus citing Camp 2006); Wikipedia Guadalcanal campaign says 64 "stationed" by end of Aug; 71 on 20 Sep. Flyable vs stationed not distinguished.
- Rabaul total: Allied estimate 160 on 23 Aug (Miller ch IV) vs 117 counted 20 Sep (Wikipedia, citing Frank); different dates and counters.
- 11th BG award: Wikipedia lead says Navy PUC, body says Distinguished Unit Citation. Not an air-order fact, noted only.

## Allied ship classes

Sources: WP = en.wikipedia.org class/ship pages (URL pattern https://en.wikipedia.org/wiki/<Class>), UB = https://uboat.net/allies/warships/class.html?ID=<n> (IDs: New Orleans 119, Northampton 118, Portland 120, Farragut 90, Mahan 88, Porter 224, Bagley 262, Gridley 91, Sims 85, Benham 84, Benson/Gleaves 87, Lexington 127, Yorktown 236, Wasp 242, Kent 60, Modified Leander 68), HW = HyperWar fuel tables (https://www.ibiblio.org/hyperwar/USN/ref/Fuel/Fuel-CA.html, Fuel-DD.html, Fuel-BB.html), NW = navweaps.com.

- Displacement: UB labels every figure "BRT" and many differ from WP standard displacement: Farragut 1395 v 1365, Porter 1825 v 1850, Gridley 1860 v 1590, Sims 1720 v 1570, Benham 1500 v 1656, Gleaves/Benson 1620 v 1630, Lexington 40000 v 36000, Kent 9750 v 10300-10570 (WP's own pages give 10400, 10570 and 10300 for the Kent class and 10000 for Canberra), Northampton 9300 v 9050, Modified Leander 6830 v 7270 (class) v 7003 (Hobart). UB Bagley 2245 equals WP's full load figure, so stored as inferred full load. Portland: WP infobox 9950-9800, WP text design standard 10096.
- Fuel capacity: HW max capacity (95%) v WP: Mahan 544 v 523; Sims 489 v 444; Benham 528 v 484; New Orleans 2256 v 2195; Portland 3061 v 1500.
- Endurance: New Orleans HW 5580 nmi at 20 kn v WP 5280 nmi at 20 kn (WP also gives about 9000 nmi at 10 kn). North Carolina HW 16320 at 15 kn v WP 17450 at 15 kn. Gleaves HW 5250 at 12 kn v WP 6500 at 12 kn (UB also 6500 for Bagley, HW and WP 6940 for Bagley/Mahan class). Portland WP infobox 13000 nmi at 15 kn v WP text 10000 at 15 kn v HW 7210 at 20 kn (different speed, so only HW is stored). Hobart: WP class 5730 nmi at 13 kn v WP Hobart page 7000 nmi at 16 kn. HW tables are about 1944-45 logistics data, not design endurance.
- HW groups Mahan and Bagley under one DD 364 class heading. HW CV page covers Essex (CV 9) and Ranger, not the 1942 carriers; HW CL 55 class is Cleveland, not Atlanta.
- Speed: New Orleans WP 32.7 v UB 32.5; Portland WP 32.7 (text design 32) v UB 32.5; Farragut/Mahan WP 37 v UB 36; Bagley WP 36.8 trials v UB 38.5; Gridley WP 38.5 v UB 38.6; Benham WP 37.9 trials v UB 36.5; Gleaves WP 37.4 v UB 37; Lexington WP design 33.25 (34.5 trials) v UB 33.
- Shaft horsepower: Mahan WP 46000 trials v UB 42800; Bagley 47191 v 49000; Gridley 47000 v 50000; Benham 49250 v 42800; Wasp WP infobox 70000 v WP text and UB 75000.
- Length: Mahan WP 341 ft 3 in v UB 334 ft; Sims WP 348 ft 3 in v UB 347 ft; Modified Leander WP class 554.9 ft v Hobart page 171.4 m v UB 562 ft. UB length for carriers (Lexington 850 ft, Yorktown 810 ft) looks like a different basis from WP overall length (888 ft, 824 ft 9 in) and is not stored.
- Yorktown beam: WP infobox 109 ft 6 in (flight deck) v WP text 114 ft.
- Complement: New Orleans 708 v 751; Northampton 1100 v 621; Portland 807 peace / 952 war v 621; Lexington 2791 v 1899; Yorktown 2217 v 1889; Wasp 1800 peace / 2167 war v 1889; Kent class 685/710/784 (WP) v 679 (UB), Canberra 751 wartime, Australia 815.
- Aircraft: Portland WP infobox 3 v WP text and UB 4; Wasp WP up to 100 v UB 84.
- Secondary guns: Northampton navweaps 8 x 5in/25 (8 Mark 19 mounts) v WP and UB 4 x 5in/25. Northampton torpedo tubes: WP says removed by 1936, UB lists 6 with no date. Hobart 4in: WP (class and ship) 4 singles as completed v NW and UB 8 (4 twin Mark XIX/XVI).
- 8in/55 mark: navweaps names the Mark 9/11/13/14 family without a class mapping; WP says Mark 9 for Portland; fits use Mk 9 with basis inferred.
- Atlanta light AA: WP infobox 4 quad 1.1in and 6 single 20mm v WP text 3 quad then 4th by early 1942 and 8 single 20mm added from early 1942.
- Kent class 2-pdr: WP infobox 8 (2x4 quad) v WP design text singles initially. HMAS Canberra: infobox 4 multiple pom-poms (1942) v text 2 multiple pom-poms and five 20mm added. Torpedo tubes: Kent class page says removed from 1935; Canberra page says two quadruple sets fitted; Australia page says removed in 1942.
- North Carolina 1.1in: WP says the design called for four quads, later text says two added in place of searchlights and Washington kept six to mid-1943; stored as 16 barrels (design) with the six-mount detail in the note.

## Japanese ship classes

- Takao class standard displacement: Nishida 9850 t (https://rikukaigun.org/Nishida/e/stc0332.html) vs Wikipedia 11,350 long tons (https://en.wikipedia.org/wiki/Takao-class_cruiser). Full load likewise: 12781 t trial vs 15,250 long tons.
- Chokai heavy AA in Aug 1942: Wikipedia says it kept four 12 cm singles (https://en.wikipedia.org/wiki/Japanese_cruiser_Chōkai; infobox contradicts its text); Nishida says 8 x 12.7 cm after a 1942 refit (stc0332); combinedfleet chokai_t.htm records no change to 12 cm or 12.7 cm guns.
- Chokai light AA: combinedfleet (https://www.combinedfleet.com/chokai_t.htm) says 25 mm twins added 3 May 1942; Nishida lists 4 x 13.2 mm and no 25 mm.
- Furutaka and Kako guns: combinedfleet (furuta_t.htm, kako_t.htm) says 20.32 cm No.2; navweaps (https://navweaps.com/Weapons/WNJAP_79-50_3ns.php) says No.1 guns from Haguro and Ashigara rebored to 20.32 cm; navweaps' No.2 page lists the class under No.2 with the author's caveat.
- Tenryu class: Nishida (stc0324) 3 twin torpedo mounts, belt 50 mm; Wikipedia 2 triple mounts, belt 63 mm. Unit of 3948 (metric t vs long tons). 25 mm timing: combinedfleet Feb and Jun 1942 vs Wikipedia 1940 and Feb 1942.
- Yubari: belt 58 vs 38 mm; shaft power 57750 vs 57900; fuel 830 t vs 902 lt; length 138.9 vs 139.45 m (Nishida stc0328 vs Wikipedia).
- Sendai class: Nishida 5595 t is 'normal' and 7100 t full load; Wikipedia calls 5,595 long tons full load. Belt 76 vs 64 mm, length 163.03 vs 162.15 m.
- Kamikaze/Mutsuki: Nishida metric t vs Wikipedia long tons for same numbers; Mutsuki main gun 12 cm/50 (Nishida) vs 12 cm/45 (Wikipedia).
- Nagato class beam 32.96 m (Nishida, 1936) vs 34.6 m (Wikipedia, Mutsu); complement 1368 vs 1475.
- Hiei 12.7 cm AA: combinedfleet.com/ships/kongo says Hiei 4 (class 8); Nishida and Wikipedia say 8.
- Nagara endurance on Nishida stc0326 ('5000 nm at 36 kt') looks like a typo, omitted.

## Allied weapons and sensors

- **HMAS Canberra radar.** Madsen (https://navyhistory.au/radar-in-the-south-and-southwest-pacific-as-at-savo-island-in-august-1942/) and Gregory (https://ahoy.tk-jk.net/macslog/HMASHIPSCanberraandShrops.html) say Type 271 (Gregory: fitted about May 1942). Wikipedia (https://en.wikipedia.org/wiki/HMAS_Canberra_(D33)) says Type 241 surface search plus A290 air warning, fitted Feb-May 1942. Madsen says only that she ran acceptance trials on a Type 290 before Guadalcanal. Recorded as disputed rows.
- **SK radar in 1942.** Navweaps (https://www.navweaps.com/Weapons/WNUS_Radar_WWII.php) says SK became operational in 1944 (150 cm, 250 kW, "Range 162 nm"). FTP 217 (https://ibiblio.org/hyperwar/USN/ref/Radar/Radar-1.html) gives 100 miles on medium bombers at 1,000 ft. The Wikipedia Enterprise infobox lists SK without a date. No SK fits recorded for Aug 1942.
- **Fire-control radar ranges.** Navweaps: Mk 3 tracking range 40,000 yd; Mk 4 40,000 yd on bombers and 30,000 yd on large ships. FTP 217: Mk 3 reliable 28,000 yd on battleships; Mk 4 35,000 yd on a PBY and 25,000 yd on battleships. Both recorded as disputed.
- **SG ship range.** FTP 217: 15 miles on large ships. Navweaps: 15.7 nm. Minor, recorded as disputed.
- **SC range.** Bates Ch.V (https://www.ibiblio.org/hyperwar/USN/rep/Savo/NWC/NWC-Savo-5.html): reliable range 4-10 miles at Savo. Pacificeagles: original SC about 10,000 yd (under 6 miles) on ships. FTP 217 gives 30 miles on medium bombers. These are different targets and are not all in the sensor rows.
- **CXAM dating.** The Wikipedia CXAM article (https://en.wikipedia.org/wiki/CXAM_radar) is inconsistent: its lead says all 14 CXAM-1 sets were delivered in 1940, its variants section says delivery began late 1941. Its ship range for large ships (14 miles) differs from FTP 217 (25,000 yd).
- **8in/55 marks.** The research brief expected Mk 12/14/15 on the Northampton, Portland and New Orleans classes. Navweaps (https://navweaps.com/Weapons/WNUS_8-55_mk12-15.php) says Mark 12 went only to the Tuscaloosa (CA-37) sub-class, Wichita and Baltimore, and Mark 15 to later ships (Minneapolis only in 1945). The Mark 14 is described on the Mark 9 page (https://www.navweaps.com/Weapons/WNUS_8-55_mk9.php) as the usual WWII regun. Which mark each cruiser carried in Aug 1942 is not established here.
- **5in/25 muzzle velocity.** Navweaps page header 2,155 fps (657 mps); its range table uses 2,110 fps (643 mps). Value 657 recorded, difference noted.
- **Ralph Talbot fire-control radar.** Madsen says an FD radar was destroyed; ONI ch.5 says "the radars". Recorded as low confidence.
- **Bates text.** HyperWar's OCR of Bates ch.V reads the picket radar as "8 C"; read as SC, consistent with Madsen, ONI ch.3 (footnote naming Astoria's SC) and pacificeagles.
- **Cruiser radars in November.** For Tassafaronga (30 Nov 1942), an NHHC H-gram and the ONI narrative disagree on which cruisers carried SG versus CXAM or SC-1 (from search snippets only; pages not read). Not used for the August fits.

## Japanese weapons and sensors

- Type 93 torpedo speed and range settings. NavWeaps (https://navweaps.com/Weapons/WTJAP_WWII.php) gives 48-50 kn at 20,000 m, 40-42 kn at 32,000 m, 36-38 kn at 40,000 m. USNTMJ report O-01-1 p.32 (https://fischer-tropsch.org/primary_documents/gvt_reports/USNAVY/USNTMJ%20Reports/USNTMJ-200D-0022-0469%20Report%200-01-1.pdf) gives 49 / 40 / 36 kn at the same three ranges. Wikipedia (https://en.wikipedia.org/wiki/Type_93_torpedo) gives 48-50 kn at 22,000 m, 37-39 kn at 33,000 m, 34-36 kn at 40,400 m, and says its own body text gives 61-65 km/h for the long setting while the infobox gives 63-67. All rows are marked disputed; NavWeaps midpoints are our inference.
- Type 93 warhead: NavWeaps, USNTMJ O-01-1 and Wikipedia all say 490 kg for Model 1. No page read was found giving 400 kg. Model 3 (larger warhead, about 780 kg per a search summary) was not verified on a fetched page.
- Type 93 in service: NavWeaps says 1935, and says Mod 3 did not enter service. USNTMJ says Model 1 Mod 1 was designed 1935 and Mod 2 produced 1936-1944; the Model 1 experimental torpedo dates from 1933.
- Type 90: NavWeaps 42 kn at 10,000 m, USNTMJ 43 kn. NavWeaps in service 1933; USNTMJ developed 1930, discontinued end of 1942.
- Type 8 (8th Year): warhead 346 kg (NavWeaps No. 2) vs 345 kg (USNTMJ); low setting 27 kn (NavWeaps) vs 28 kn (USNTMJ).
- Type 6 (6th Year): warhead 203 kg (NavWeaps) vs 200 kg (USNTMJ); high speed 35 kn vs 36 kn at 7,000 m.
- 12 cm/45 3rd Year Type muzzle velocity: NavWeaps table 825 m/s; a USNTMJ report quoted on the same page gives 780 m/s, which the page author thinks is an error (http://www.navweaps.com/Weapons/WNJAP_47-45_11ns.php). The report itself was not seen.
- 20 cm/50 No. 1: NavWeaps bore length 9.144 m is inconsistent with its own 381.5 in; overall length 10.000 m (50 cal) used.
- Shokaku Type 21 date: combinedfleet TROM (https://www.combinedfleet.com/shokaku.htm) says during the Kure repairs ending 27 Jun 1942. Kojinsha No. 6 (https://combinedfleet.com/kojinshavolume6.pdf) says during repairs after Midway but also has a caption saying a Type 21 was fitted at Yokosuka in February (on a photo dated 27 May 1943, probably the second set), and says it is unclear whether her August 1942 refit was complete.
- Ise Type 21 date: Ise TROM (https://www.combinedfleet.com/Ise.htm) 25 May 1942; combinedfleet radar page (http://www.combinedfleet.com/radar.htm) April 1942. Both say first or experimental.
- Hyuga Type 22 date: Hyuga TROM (https://www.combinedfleet.com/Hyuga.html) 20-28 May 1942; combinedfleet radar page October 1942. The radar page also lists Type 22 on destroyers from March 1942 while giving its operational date as September 1944.
- Type 21 in-service date: radar page "operational August 1943" vs fits from April-June 1942 on the same site.
- Kirishima: TROM footnote says several sources suggest a Type 21 fitted early August 1942 but it was scheduled to receive one and departed without it (https://www.combinedfleet.com/Kirishima.htm). The two statements disagree.
- Savo Island: Matsuyama (USSBS NAV-61, https://ibiblio.org/hyperwar/AAF/USSBS/IJO/IJO-61.html) says "We did not have radar". No fetched source says otherwise.
- Radar page count: the page notes one source says one Type 21 was built but also says it was installed widely; the page estimates 30-40 sets.

## Aircraft and air weapons

- **A6M2 Model 21 max speed.** At least eleven values are in circulation: 316 to 345 mph. j-aircraft Zero Facts 331.5 mph at 14,930 ft (https://j-aircraft.com/research/zerofacts.htm); Wikipedia 533 km/h at 4,550 m (https://en.wikipedia.org/wiki/Mitsubishi_A6M_Zero); ibiblio phha 340 mph at 19,700 ft (https://www.ibiblio.org/phha/ZERO.html); Japanese flight manual 275 kn at 4,400 m, BuAer memo 335 mph at 16,000 ft, TAIC 328 mph (all quoted in https://j-aircraft.com/research/rdunn/zeroperformance/zero_performance.htm). Each is a disputed row.
- **A6M2 engine.** j-aircraft and Wikipedia: Sakae 12, 940 hp. ibiblio phha: Sakae 21, 950 hp.
- **A6M2 weights.** j-aircraft: 3,704 lb empty, 5,313 lb loaded, 6,164 lb maximum. Wikipedia calls 6,164 lb (2,796 kg) the loaded weight. ibiblio phha: 3,920 / 5,750 / 6,330 lb.
- **A6M2 span.** Wikipedia 12 m, Pacificwrecks 12.1 m, ibiblio phha 36 ft 2 in (folded-tip span).
- **A6M3 Model 32.** j-aircraft lists two entries: 338 mph, 3,984 lb empty, 5,609 lb loaded vs 336 mph, 4,107 lb, 5,906 lb.
- **Zero cannon fit.** j-aircraft gives 60 rpg on A6M2, 100 rpg on A6M3. Pacificwrecks says 100-round magazines reached A6M2 only in Sept 1943 (https://pacificwrecks.com/aircraft/a6m/tech.html). Pacificwrecks gun page assigns Type 99 Model 2 to "A6M3 and later" (https://pacificwrecks.com/gun/20mm-type99/tech.html); Wikipedia says the initial A6M3 used Mark 1 Model 3 and the Mark 2 came with the A6M3a Model 22 (https://en.wikipedia.org/wiki/Type_99_cannon).
- **G4M1 engine and crew.** Wikipedia Kasei 11 (MK4A), crew 7; Pacificwrecks Kasei 21 (MK4P), crew 7-10 (https://pacificwrecks.com/aircraft/g4m/tech.html).
- **D3A1 data.** Wikipedia's table is for the D3A2 Model 22 (Kinsei 54), not the D3A1. ibiblio phha gives D3A1 with Kinsei 54, 281 mph at 20,300 ft; j-aircraft D3A1 gives Kinsei 43/44, 237 mph. Both disputed.
- **B5N2 torpedo and range.** Wikipedia: 800 kg Type 91 torpedo, range 978 km. Pacificwrecks: "Type 95" 800 kg torpedo, range 1,237 miles.
- **H6K defensive guns and range.** Wikipedia: Type 92 in nose, spine and two waists plus a 20 mm tail; range 4,797 km. Pacificwrecks: two 7.7 mm and a hand-held 20 mm tail; range 2,230 miles; Wikipedia itself gives both January 1934 and January 1938 for service entry.
- **F1M2 and E13A1 Pacificwrecks speeds.** Pacificwrecks states 200 km/h (F1M) and 203 km/h (E13A), which look like cruise speeds or mislabelled units against Wikipedia 370 km/h and 376 km/h. Pacificwrecks E13A range "1,128 miles" equals Wikipedia's 1,128 nmi (2,089 km).
- **F4F-4 speed.** Navy text via HyperWar: 320 mph at 19,800 ft (https://ibiblio.org/hyperwar/OnlineLibrary/photos/ac-usn22/f-types/f4f.htm); Wikipedia 318 mph at 19,400 ft.
- **Allied reporting names timing.** Wikipedia A6M page: the code system was introduced in 1942; Wikipedia D3A page: "Val" applied in mid-to-late 1943.
- **P-400 cannon designation.** Wikipedia's HS.404 page lists the P-39 Airacobra under the M1 and the P-400 under the AN/M2 (https://en.wikipedia.org/wiki/Hispano-Suiza_HS.404); the key used is us-20mm-hispano-m1.
- **Mk 13 and Type 91 models.** NavWeaps lists Mk 13 Mod 0 (884 kg, 30 kn, 5,700 yd) and Mod 10 (1,005 kg, 33.5 kn, 4,000 yd); which was standard in Aug 1942 is not stated. Type 91: NavWeaps prints Mod 2 as 1,841 lb but 935 kg (1,841 lb is 835 kg); Wikipedia gives 838 kg for Rev.2 and 848 kg for Rev.3 against NavWeaps 849 kg Mod 3. Wikipedia does not use Model/Kai labels.
- **Type 92 7.7 mm gun.** NavWeaps' Type 92 is a ship/army tripod Lewis derivative, not labelled aircraft; the aircraft flexible gun of the same name is assumed identical.
- **37 mm M4 rate of fire.** NavWeaps 125-150 rpm cyclic; Wikipedia (seen as a search snippet only) 150 rpm.
- **TBF-1 fixed guns.** Wikipedia: single .30 in the nose on early models, or two .50 in the wings; which applied in Aug 1942 not stated.
