# The Coral Sea, 3 to 8 May 1942: order-of-battle facts

Read on 2026-11-20 for a future scenario of the battle. This pass gathered facts only: rows in `data/oob`, under the rules of `data/oob/README.md` and `docs/research/oob/database-design.md`. It derives no rating, writes no scenario and changes no map. Rows already on file for the battle (the strike launches and searches of decisions 0081 and after, the ditchings, the AA bulletin) were left as they were. The engagement `coral-sea` (id 1003) was already in `engagements.csv` with the dates 4 to 8 May and no zone. Both were left alone; the rows here run from 1 to 11 May.

## What was added

| File | Rows | What |
| --- | --- | --- |
| `ships.csv` | 43 (ids 1189 to 1231) | Lexington, Yorktown, the destroyers Monaghan, Aylwin, Perkins, Sims, Anderson, Hammann, Russell, Walke and Morris, the oilers Neosho and Tippecanoe, the tender Tangier; Shoho, Sazanami, Ushio, Akebono, Shigure, Shiratsuyu, Okinoshima, Kikuzuki, Yuzuki, Mochizuki, Asanagi and Tatsuta; the auxiliaries Toho Maru, Iro, Hoyo Maru, Kamikawa Maru, Hijirikawa Maru and Tama Maru; 11 transports |
| `ship_classes.csv`, `ship_class_facts.csv` | 3, 50 (2477 to 2526) | Zuiho class (Shoho), Cimarron class (Neosho), Okinoshima class |
| `aircraft_types.csv`, `aircraft_type_facts.csv` | 6, 80 (557 to 636) | TBD-1, F4F-3, SBD-2, A5M4, B5N1, G3M2 |
| `ship_facts.csv` | 227 (3321 to 3547) | Dates, fates and classes of the new ships, and their movements of 3 to 11 May |
| `air_units.csv`, `air_unit_facts.csv` | 10, 62 (451 to 512) | Squadrons of both US carriers, Shoho's air group, the Motoyama group, Tangier's search group; strengths and bases by date |
| `engagement_forces.csv` | 159 (376 to 534) | Who was there, by force and ship, on each side, from three sources |
| `engagement_positions.csv` | 52 (558 to 609) | Positions, reported positions and sinkings |
| `engagement_events.csv` | 53 (2469 to 2521) | Landings, strikes, attacks, orders and the turn-back |
| `engagement_outcomes.csv` | 65 (4209 to 4273) | Hits, losses of ships, aircraft and men, claims |
| `places.csv`, `place_positions.csv` | 6, 5 (95 to 99) | Deboyne, Misima, Rossel Island, Jomard Passage, Townsville, Point ZED |
| `gaps.csv` | 13 | What was looked for and not found |

The class, aircraft and ship-fact rows were written by two researchers under this pass. Every one of their 357 rows was checked by script against the page it cites: the row's quoted fragment, or failing that each figure in it, was found in the page text. The pass's own 291 engagement and air unit rows were checked the same way.

## Sources and how firm they are

| Source | Grade | Rows | Note |
| --- | --- | --- | --- |
| Naval War College, *The Battle of the Coral Sea ... Strategical and Tactical Analysis* (1947), `bates-coral-sea-1947` | A | 183 | Read whole in its machine-read text on archive.org. It works from both sides' records (CruDiv 6's combat report, the 25th Air Flotilla's war diary, Shoho's action report, the USSBS Truk interrogations) and keeps zone -11. The OCR misreads digits and mangles tables; rows that rest on a doubtful reading are `low` and say so. Page numbers are inferred from the page markers in the text and may be one out where a marker was lost |
| ONI Combat Narrative, *The Battle of the Coral Sea* (1943), `oni-cn-coral-sea` | A (US), C (Japanese) | 111 | HyperWar transcription, all chapters and the appendix. Zone -10 until 1700 on 7 May, then -11. Its damage to the enemy is wartime claim |
| combinedfleet.com tabular records (Shoho, Zuikaku, Shokaku and the ships' pages) | B | 158 | Japanese clock; not tied to the US clocks |
| Wikipedia | C | 130 | `low` and `unconfirmed` everywhere; mostly class and aircraft figures, plus Lundstrom's strengths by carrier as the article quotes them |
| DANFS (HyperWar mirror and the Navy's site) | B | 84 | US ships' dates and fates |
| Yorktown and Lexington action reports | A | 13 | Squadron strengths and types |
| Navypedia, Nishida, uboat.net, NHHC aircraft pages, Pacific War Online Encyclopedia, Wikidata | B to C | 74 | Class and aircraft figures, place positions |

## Disputes a scenario builder has to settle

1. **The clocks.** Bates keeps zone -11 and his times for the attack on Shoho run 1107 to 1131. ONI says its times before 1700 on 7 May are zone -10, yet gives 1145 and 1147 for the same attack. combinedfleet gives 0855 to 0935 on Japanese time. The Neosho attack is 1131 (Bates) or 1201 (ONI); the rendezvous of 5 May 0816 (Bates) or 0846 (ONI). Times are entered as printed and every row names its source's clock.
2. **The Japanese strike of 8 May.** 69 aircraft (18 fighters, 33 dive bombers, 18 torpedo planes: combinedfleet, and Wikipedia); 90 (Bates's text: 70 attack planes and 20 fighters); or 20 fighters, 29 dive bombers and 41 torpedo planes (Bates's own table). combinedfleet's two records give 0730 and 0622 for the launch; Bates 0915 on zone -11.
3. **Japanese aircraft lost.** Bates's summary gives 105 in all, 45 carrier planes on 8 May; his own analysis gives 43 for 8 May; the Japanese report he quotes says 15 in combat. The Allied claims for 8 May were 73. Zuikaku's operational aircraft afterwards: 9 at 1500 on 8 May, 15 on the morning of 9 May, 13 at noon on 9 May (all Bates), 36 on 8 May (Wikipedia), 45 at the end (combinedfleet).
4. **The Port Moresby transports.** Bates's appendix names nine ships, five of which can be matched to combinedfleet's ten (Goyo, Shoka, Akibayama/Akibasan, Mogamikatca/Mogamigawa, and an Azumayama Maru that Bates also lists at Tulagi); combinedfleet adds Chowa and five Army ships (Matsue, Taifuku, Mito, China, Hibi); Wikipedia says eleven. Both sets are kept as forces rows marked `disputed`.
5. **Shoho's air group and her end.** 12 fighters (A6M2 and A5M4) and 6 B5N1 (combinedfleet, 24 April); about 21, 12 fighters and 9 to 12 torpedo planes (Bates); 18 (Wikipedia citing Lundstrom). Hits: 12 bombs and 7 torpedoes (Bates) or 13 and 7 (combinedfleet). Survivors: 100 (Bates, Sazanami's report) or 131 and 72 wounded (combinedfleet).

Others, smaller: the 0800 position of TF 17 on 8 May (one minute apart), the 8 May contact (11 51 S 156 04 E in ONI, 12 00 S 156 12 E plotted from Point ZED in Bates), the B-17 report of 7 May (152 26 E or 152 36 E), Neosho's sinking (1522 at 15 35 S or 1452 at 15 38 S; scuttled by Henley in both narratives, by Helm in Bates's chronology), Okinoshima's sinking (11 May in Bates, 12 May in combinedfleet and Wikipedia), Shokaku's casualties (108 killed and 40 wounded, or 223 killed or wounded), the flying-boat group's name (Yokosuka in Bates's text, Yokohama in his appendix), and whether the Striking Force flew off its 18 Zeros for Rabaul (Bates) or was stopped by weather (combinedfleet). The class and aircraft disputes (Zuiho's displacement in metric or long tons; Cimarron's speed and power; the TBD-1's weights and power) are in the rows' notes.

## Gaps

- **Positions of the Japanese forces on 3 to 6 May.** Only what Bates's text states: the Striking Force at 2400 on 6 and 7 May and 0600 on 7 May, CruDiv 6's sections and the Support Force at 2400 on 7 May, the Covering Force and Striking Force relative to Tulagi on 3 May. His track diagrams do not survive the OCR. The Salomon and Tomioka track chart he cites is the source to find.
- **Sinking positions of Shoho and Sims.** Shoho's only position is Wikipedia's; Sims has none.
- **Lexington's squadron strengths.** Yorktown's are in her reports for 7 and 8 May; Lexington's only as Wikipedia quotes Lundstrom. The same holds for each Japanese carrier by type.
- **The troops in the invasion convoys.** The units are named (South Seas Detachment, 144th Infantry, the 3rd Kure SNLF, the 10th Construction Unit) but no source read gives their numbers except Wikipedia (about 5,000 and 500; 400 at Tulagi), which is noted in `gaps.csv` and not entered. The planned landing date likewise.
- **Allied land-based air.** The Army groups are named by Bates; their strengths are in a table the OCR destroyed.
- **The seaplanes at Deboyne and Tulagi**, and the position of Jomard Passage (Wikidata refused further requests that day).
- Submarines are left out, as throughout.

## What to look for next, and where

- The page images of the Bates analysis (archive.org holds the scan) for the diagrams, the Allied air table and the doubtful digits.
- USSBS, *The Campaigns of the Pacific War*, the Coral Sea chapter and its appendices (HyperWar), and the USSBS interrogations Nav. 10 and others the analysis cites, for Japanese strengths and times.
- The Japanese Monographs on the South Seas operations, and the Senshi Sosho volume on the Southeast Area naval operations, for the convoys, the troops and the landing date.
- CTF 17's action report (HyperWar `rep/Coral/TF17.html`, already a source here) read in full, and the Lexington air group's enclosures for squadron strengths; NavSource, which refused automated reading.
- Lundstrom's *The First Team* and Frank's *Guadalcanal* are the books that would settle most of the disputes above; not in hand.

Not reached: NavSource (refuses scripts), history.navy.mil directly (read through the Wayback Machine where used), GeoNames (now a script application), Wikidata after its rate limit.

## The map

The present map (`data/map/south_pacific.csv`, header) covers every hex whose centre lies from 1 S to 24 S and 145 E to 172 E, on hexes of 40 nautical miles centred at 12 S 160 E. That holds Port Moresby (9 28 S 147 08 E), the Louisiades with Deboyne, Misima, Rossel and Jomard Passage (about 10 to 11 30 S, 152 to 154 E), Rabaul (4 13 S 152 10 E), Lae, Tulagi, Shortland, Townsville (19 16 S 146 49 E), Noumea and every position of the battle in these rows (the farthest west is Crace's group at about 149 59 E, the farthest south Neosho near 16 58 S). Truk (7 N) and Tongatabu are outside it, as they are for the slice. Port Moresby and Townsville lie near the western edge, three to five hexes inside it. The map was not changed.
