# Submarines in the play scenarios

Read on 2026-11-23 for the architect, who is to place boats in the scenarios the table offers (decision 0218, "Games on file"). It lists the boats the fact rows put in or near each battle's map and days, what each side believed of the other's boats, and the gaps. It places no boat and decides nothing. Rows are cited as in `submarines-in-1942.md`: `ev` `engagement_events.csv`, `pos` `engagement_positions.csv`, `out` `engagement_outcomes.csv`, `fo` `engagement_forces.csv`, `sh` `ship_facts.csv`, `cl` `ship_classes.csv`, each with its id.

A boat can be put in a scenario only where her class is one of the classes derived in stage 1 (`data/reference/submarine_classes_1942.csv`, 6000 to 6011) and a row ties her to it. Of the boats below, a ship row with a class (`ships.csv` or `sh` 16000 to 16025) exists for: S-38 (6009), S-44, S-45, S-47 (6010), Nautilus (6011), Tambor, Trout (6001), Gato, Finback (6002), Sargo and Seawolf (6000), I-168, I-169, I-171, I-174, I-175 (6005), I-156, I-157, I-158, I-159, I-164, I-165, I-166 (6004) and I-121, I-122, I-123 (6008). Every other boat named below has no ship row: her class is a gap even where the class's own description lists her number (`cl` 16010 lists I-15, I-17, I-19, I-21, I-23, I-25 to I-45; `cl` 16015 lists RO-33 and RO-34; `cl` 16001 lists S-42 to S-47).

## The maps

- The South Pacific map (Coral Sea, Eastern Solomons, the 39 days): hexes from 1 S to 24 S and from 145 E to 172 E.
- The Midway map: from 18 N to 38 N and from 163 E to 166 W.

## The Coral Sea (the table's game: 6 to 10 May 1942)

**Japanese boats.** None has a ship row. `gaps.csv` holds two entries for the battle: the boats RO-33, RO-34, I-22, I-24, I-28 and I-29 left out of the order of battle (Bates, Appendix I), and their individual stations (beyond "patrol line SW of Guadalcanal by 5 May", off Nouméa, off Brisbane and the Port Moresby approaches) not found. One unidentified boat was sighted on the surface at 16-04 S 162-18 E on 2 May, 32 miles north of Task Forces 11 and 17 (ev 16749).

**American boats** (S-boats from Brisbane; none in `fo` for the battle; stations are named areas, with no latitude and longitude):

| Boat | Class | Where and when | What | Rows |
| --- | --- | --- | --- | --- |
| S-42 | gap (no ship row) | off Cape St George, 5 May; Okinoshima sunk 11 May at 5-06 S 153-48 E | missed a tender 5 May; 4 Mark 10, 2 or 3 hits, 11 May; hunted 6 hours | ev 16103, 16104 to 16107; pos 16005; out 16115, 16116, 16122 |
| S-44 | 6010 | Rabaul approaches, 8 to 12 May | 2 torpedoes missed 8 May; contacts 9 and 10 May; Shoei Maru sunk 12 May | ev 16108 to 16112 |
| S-47 | 6010 | Simpson Harbour 2 to 3 May; off New Hanover 8 May | missed an oiler; fired at a cargo ship 8 May | ev 16161 to 16163 |
| S-38 | 6009 | Jomard Passage, mid-May | no attack | ev 16113 |
| S-41, S-39, S-43, S-45 | S-45 6010; the others gaps | Shortland and Treasury from 9 May; Louisiades from 10 May; St George's Channel from 21 May; Bougainville to New Ireland from 12 May | none in the days | ev 16114, 16116, 16115, 16167 |

None of these attacked a warship of a task force in the scenario's days. The war on shipping is not built (decision 0218, the third stage).

**Beliefs.** Japanese: one or two Allied boats in the Bismarcks, when there were four (ev 16257, 1 May). Allied: boats probably on station for the Southwest Pacific offensive (ev 16534, 2 May); the surfaced boat of 2 May taken to have reported the force (ev 16750); CINCPAC's estimate of the MO forces with 12 submarines (ev 16849); enemy boats and a tender at Rabaul (ev 16513, 15 April).

## Midway (the table's game: 3 to 7 June 1942)

**American boats** (`fo` 8538 to 8556, Bates's list again at 8583 to 8607, ONI's at 8633 to 8657; stations ordered 2 June, ev 16168, 16170 to 16175):

| Group and station | Boats | Rows |
| --- | --- | --- |
| Outer arc, 250 to 310 degrees, 200 miles from Midway | Dolphin, Gato (6002), Grenadier | pos 16009 |
| Middle arc, 150 miles | Tambor (6001), Trout (6001), Grayling, Nautilus (6011), Grouper, Gudgeon | pos 16010 |
| Inner arc, 310 to 350 degrees, 50 miles | Flying Fish, Cachalot | pos 16011 |
| About 700 miles west (27 N 170 E on 30 May) | Cuttlefish | pos 16012, 16021; ev 16179 |
| Support patrol, a line centred 075 degrees, 425 miles from Midway | Narwhal, Plunger, Trigger | pos 16013 |
| North of Oahu, about 300 miles from Oahu (off the map) | Pike, Tarpon, Growler, Finback (6002) | pos 16014 |

Boats with no class row (gaps): Dolphin, Grenadier, Grayling, Grouper, Gudgeon, Flying Fish, Cachalot, Cuttlefish, Narwhal, Plunger, Trigger and the six recalled on 5 June (ev 16176). Moves in the battle: closing the enemy, 4 June 0710 (ev 16181); a 100-mile circle by dawn 5 June (ev 16182). Attacks: Nautilus fired 5 torpedoes in three attacks on 4 June, one at Soryu with a dud claimed (ev 16063 to 16086; out 9390, 16107 to 16109); Grouper fired 2 at a destroyer (ev 16090). Tambor reported Cruiser Division 7 on 5 June (ev 16093, 7676). Nautilus's position at 0710 on 4 June is in dispute between pos 5775 and pos 16002.

**Japanese boats** (15, `fo` 5911 to 5925; ev 16198):

| Group and station | Boats | Rows | On the map? |
| --- | --- | --- | --- |
| Subron 3, the KO line, 20 N to 23-30 N at 166-20 W, from 1 June | I-169, I-171, I-174, I-175 (6005) | ev 16194; pos 16017, 16018 | no: 166-20 W is east of the map's edge |
| Subron 5, the OTSU line, 28-20 N 162-20 W to 26-00 N 165-00 W | I-156, I-157, I-158, I-159, I-164, I-165, I-166 (6004) | ev 16195; pos 16019, 16020 | no |
| Subdiv 13, French Frigate Shoals | I-121, I-122, I-123 (6008) | ev 16203 to 16205 | no |
| I-168, reconnoitring Midway | I-168 (6005) | ev 16202, 7670 to 7673; Yorktown and Hammann 6 June, ev 16052, 16053, 8036 to 8043; out 9387 to 9389, 16099 to 16101; pos 5774 | yes |

Point M of the cancelled flying-boat operation (I-171, 19-00 N 174-26 W, 31 May, pos 16015) is on the map; I-175 was 80 miles from Pearl Harbor (pos 16016). I-164 was sunk on 17 May (the note on ev 16195): Bates's list is wrong by one boat, and whether I-162 took her place is a gap. The times of I-168's attack differ between rows (13:36, 15:36, 05:01, 07:01, by zone), and `gaps.csv` notes that only some sources name I-168 as the boat that fired.

**Beliefs.** Japanese: 14 American boats sighted by I-168 off Midway, 1 to 2 June (ev 16202); the Combined Fleet staff's count of 14 at least, at Truk, north of Wake and off Japan (ev 16200, 16201: other waters). Allied: one boat west of Midway and three in northern waters by direction finding, 29 May (ev 16842); one 200 miles southwest of Midway, 16 April (ev 16518); a number believed in the Oahu to Midway area, 8 June, after the scenario (ev 16550). Decision 0218's second review withdrew the Midway rows as examples for being of other waters; that holds of 16200 and 16201, while 16202 is of Midway itself.

## Guadalcanal (7 August to 14 September 1942; the five days 23 to 27 August; the one day 24 August)

No boat of either side has a station row with a latitude and longitude in these days. `gaps.csv` says so of both: the American fleet boats off Truk, Rabaul, Kavieng and Bougainville and the S-boats; the Japanese lines east and south of the Solomons, of which a document gives two lines 200 miles apart with boats 30 miles apart and 16 boats, with no position. The positions the rows give are of sightings and attacks. No boat has a row in `passages.csv`.

**Japanese boats** (all of engagement `submarine-war-1942`, `eastern-solomons`, `rabaul-guadalcanal-air` or `guadalcanal-autumn-1942`):

| Boat | Class | Where and when | What | Rows |
| --- | --- | --- | --- | --- |
| I-121 | 6008 | from Rabaul 7 Aug; south-east of San Cristobal from 18 Aug | 1 torpedo at Task Force 16, 22 Aug, 50 miles south-east of San Cristobal, missed; bombed by Wasp's aircraft 27 Aug, could not dive | ev 16291, 16860, 16901, 16023, 16024 |
| I-122 | 6008 | from Rabaul 7 Aug | none stated | ev 16291, 16860 |
| I-123 | 6008 | off Indispensable Strait, then the Lunga anchorage | sunk by Gamble 29 Aug at 09-21 S 160-43 E, no attack made | ev 16292, 16054; pos 16001; out 16102, 16103 |
| I-174 | 6005 | from Rabaul about 23 Aug, south-west of Rennell; east of San Cristobal from 26 Aug; Ndeni to San Cristobal 10 Sep | none | ev 16290, 16271, 16273, 16276 |
| I-175 | 6005 | 170 miles south-west of Espiritu Santo 12 Aug | bombed and damaged, made for Rabaul; named in the orders of 26 Aug | ev 16025, 16271 |
| I-15, I-17, I-19, I-26, I-31, I-33, I-21 | gaps (no ship row; the I-15 block's description lists them) | lines east of Santa Cruz, Malaita and San Cristobal from 23 Aug (ev 16265, 16268, 16282, 16283, 16287); 8 boats east of San Cristobal 26 Aug (ev 16271); Ndeni to San Cristobal 10 Sep (ev 16273, 16274, 16275) | I-26: 6 torpedoes at Saratoga 31 Aug, 1 hit (ev 16005, 16010, 16011; out 16009 to 16011). I-19: Wasp, 15 Sep, after the 39 days (ev 3001 to 3009). Sightings and bombings 23 to 28 Aug: I-19 at 07-30 S 162-15 E (ev 16026), I-17 at 06-48 S 163-20 E (ev 16027), I-33 at 09-21 S 163-35 E (ev 16029), I-15 and I-17 in contact 25 and 27 Aug (ev 16030, 16031, 16278) | as stated |
| I-11 | gap (class not stated) | from Truk 20 Aug, Santa Cruz to the Solomons | 4 torpedoes at Hornet 6 Sep at 13-20 S 162-40 E, no hit; damaged by Catalinas | ev 16285, 16286, 16012, 16017; out 16034 to 16040 |
| I-9 | left out (the I-9 group has no speed) | line A off San Cristobal 23 Aug | depth-charged 25 Aug, to Truk 30 Aug; back by 10 Sep | ev 16266, 16032, 16273; out 16042 to 16046 |
| I-24, I-7, I-4, I-5, I-8, I-22, I-176 | gaps or left out (I-24 not in the I-15 block's list; I-1 and I-5 blocks left out; I-176 after the days) | Force A and Indispensable Strait; I-7 reported carrier aircraft 23 Aug | reports only | ev 16262, 16289, 381, 16293 to 16295 |
| RO-34 | gap (no ship row) | Lunga Roads 23 Aug | depth-charged, escaped | ev 16056 |
| RO-33 | gap (no ship row) | Gulf of Papua | torpedoed Malaita 29 Aug at 09-50 S 144-55 E (off the map), sunk by Arunta at 09-36 S 147-06 E | ev 16055; out 16104 |

On the five days and the one day: boats sighted or bombed on 23 Aug (ev 16026, 16027, 136, 123, 124), 24 Aug (ev 16029, 16028; two surfaced boats seen by Shokaku's scouts, side unknown, ev 367, 440) and 25 Aug (ev 16030, 16031, 16288, 16032, 16284).

**American boats:**

| Boat | Class | Where and when | What | Rows |
| --- | --- | --- | --- | --- |
| S-38 | 6009 | off Cape St George, 7 to 8 Aug | saw Mikawa's force, too close to fire; 2 torpedoes at Meiyo Maru 8 Aug, both hit | ev 16118 to 16124; pos 182, 50; out 16119 |
| S-44 | 6010 | Steffen Strait 7 Aug; off Simberi 10 Aug | 4 Mark 10 at Kako 10 Aug, 3 or 4 hits, sunk at 02-28 S 152-11 E (in dispute) | ev 16125 to 16127, 16130, 16160, 16414; pos 62, 63; out 133, 778, 16118 |
| S-45 | 6010 | Shortland end of Aug; Trobriands 12 Sep | no attack | ev 16165, 16166 |
| S-47 | 6010 | New Ireland from 2 Sep | damaged a warship 12 Sep | ev 16164 |
| S-37, S-41, S-42, S-43, S-39, S-46 | gaps (no ship row) | north of Savo 2 Sep; New Britain to New Ireland 7 Aug; Savo to Cape Esperance from about 28 Aug; Kiriwina and Buka; Rossel Island; the Solomons to 15 Aug | S-37 damaged a destroyer 2 Sep; S-41 fired 7 Aug; S-39 aground 13 to 14 Aug | ev 16150 to 16155 |
| Flying Fish, Trout, Greenling, Drum, Amberjack, Pickerel | gaps except Trout (6001) | off Truk and between Truk and New Ireland (north of the map); Kavieng | Flying Fish fired at a battleship off Truk 28 Aug | ev 16135 to 16144, 16156, 16157, 16147, 16320 |

**Beliefs.** Allied: Japanese boats concentrating against the Solomons, 9 Aug (ev 16298); hostile boats operating freely in the Cactus area, 13 Aug (ev 16584); a boat seen at 10-30 S 162-30 E on 20 Aug (ev 16608); boats in the Ringbolt to Cactus area, estimate 12, 26 Aug (ev 16589); boats known in the Tulagi unloading waters, 27 Aug (ev 16590); a large number on the line to Tulagi, 1 Sep (ev 16303); 9 near the Hornet group at 10 to 15 S 164 to 165 E, 7 Sep (ev 16610); at least 17 in the Solomons, 14 Sep (ev 16613). Japanese: no row of a Japanese estimate of Allied boats in these days; `gaps.csv` says no estimate document of 1942 was found.

**Counts.** No row gives boat-days on station. The rows give the boats ordered or present by date: 5 on 20 Aug (ev 16282), 8 on 26 Aug (ev 16271), 8 on 10 Sep (ev 16273), 6 or 8 on 13 Sep (ev 16274, 16275), 7 leaving on 20 Sep (ev 16276).

## Gaps that bar a boat from a scenario

- Ship rows (`ships.csv`, `ship_facts` class) for I-15, I-17, I-19, I-21, I-26, I-31, I-33, RO-33, RO-34, S-37, S-39, S-41, S-42, S-43, S-46 and the American fleet boats of Midway other than Nautilus, Tambor, Trout, Gato and Finback. Without them the 39 days, the five days and the one day have no Japanese boat on the lines east of the Solomons but I-121, I-122, I-123, I-174 and I-175, and the Coral Sea has no Japanese boat at all.
- The I-11's class; the I-9, I-5 and I-1 blocks' speeds (stage 1's research owed).
- A station position for any boat in the Solomons or the Coral Sea in the days of the games: only named areas, sightings and attacks.
- A Japanese belief of Allied boats from 7 August to 14 September.
