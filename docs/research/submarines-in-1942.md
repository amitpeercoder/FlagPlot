# Submarines in 1942

Read on 2026-11-22 for the architect, who is to write a rule for submarines. It asks of the record what submarines of both navies did in the waters of the game's battles in 1942 (the Coral Sea, Midway, and Guadalcanal from 7 August to mid-September, with Saratoga's torpedoing of January 1942 and a few later statements where a source dates them). It covers every submarine attack on a warship found, where the boats were and why, what their sighting reports did, what made an attack succeed or fail, the torpedoes, the anti-submarine practice of each navy, and the classes of the boats. It settles nothing and proposes no rule and no figure.

The reading was done by six Sonnet researchers, each on one heading, who staged rows with the quotation and the text file it came from. The lead then:

- checked every quotation by program against the saved text (whitespace and case ignored; each fragment joined by " ... " checked on its own): all 809 staged rows that carry a quotation passed;
- threw out 30 rows that repeated a statement already on file or staged twice by two researchers;
- read two figures off the page image where the OCR misread them (`ship_class_facts` 16013 and 16021, each note says so);
- renamed the proposed Japanese class keys to the ship-number blocks the sources print (the type codes KD6A, B1, J2 and KD7 appear in no row read; see 7);
- made the weapon and class values fit the attribute types (yards and pounds made metres and kilograms with the factor named in the note; text that is not a number moved to a note attribute).

How rows are cited: `ev` is `engagement_events.csv`, `out` is `engagement_outcomes.csv`, `pos` is `engagement_positions.csv`, `cl` is `ship_class_facts.csv`, `sh` is `ship_facts.csv`, `wp` is `weapon_facts.csv`, each with its id. The files share the id range 16000 upward, so the prefix matters. Rows already on file before this pass are cited as "on file".

## What the rows hold

| File | Ids | Rows | What |
| --- | --- | --- | --- |
| `engagement_events.csv` | 16000 to 16425 | 413 | Attacks, sightings, counter-attacks, patrol orders and stations, reports sent and received, and doctrine and judgements by navy and date (engagement `submarine-war-1942` for what lies outside a battle on file) |
| `engagement_outcomes.csv` | 16000 to 16141 | 129 | One ship in one attack: screen, speed and zigzag, damage, time out, depth charges dropped or received, torpedo failures, verdicts |
| `engagement_positions.csv` | 16000 to 16021 | 22 | Where attacks and sinkings were, and the patrol stations and lines of both navies at Midway |
| `weapon_facts.csv` | 16000 to 16062 | 63 | The Mark 14 and Mark 10 and their faults by date; the Type 89, 92, 95 and 96 submarine torpedoes; the Japanese Type 95 and Type 2 depth charges |
| `ship_class_facts.csv` | 16000 to 16126 | 127 | Surfaced and submerged speed, submerged endurance, diving depth, range, tubes, torpedoes carried, displacement and complement for 17 submarine classes or blocks |
| `ship_facts.csv` | 16000 to 16025 | 26 | The class of 26 boats as a source gives it |
| `ship_classes.csv`, `ships.csv`, `weapons.csv` | 16000 up | 17, 6, 8 | New submarine classes (type `submarine`), six US boats not on file, six torpedoes and two depth charges |
| `attributes.csv` | last 16 | 16 | `submerged_max_speed_kn`, `submerged_endurance_note`, `diving_depth_m`, `diving_depth_note`, `surfaced_displacement_lt`, `submerged_displacement_lt`, `torpedo_tubes_bow`, `torpedo_tubes_stern`, `complement_note` (classes); `exploder_note`, `depth_fault_note`, `reliability_note`, `torpedo_failure_note`, `depth_setting_note`, `service_note`, `performance_note` (weapons) |
| `sources.csv` | 16000 to 16013 | 14 | New sources |
| `engagements.csv` | 1017 | 1 | `submarine-war-1942`, a holding place like `carrier-practice-1941-1945` |
| `gaps.csv` | last 61 entries | 61 | What was looked for and not found |

Rows by heading (a row counted once, under the heading it mainly serves): attacks on warships 163 events, 124 outcomes and 9 positions (1); where the boats were and their sighting reports 134 events and 13 positions (2 and 3; the reports listed in 3 are drawn from both groups); what made an attack succeed or fail and anti-submarine practice 105 events, 5 outcomes and 5 depth-charge rows (4 and 6); torpedoes 58 weapon rows and 11 events (5); classes 127 class rows and 26 ship rows (7); gaps 61 (8).

By confidence: of the 780 fact and event rows, 229 are `high` (War Damage Reports, ONI narratives in clean text, the Graybook, CINCPAC, Rowland and Boyd, WDR 58, the USSBS interrogations, Cominch), 512 `medium` (combinedfleet.com, DANFS, Bates through OCR, the Technical Mission's reports and ONI 222-J through OCR) and 39 `low` (the Naval Submarine League and Australian articles, Wikipedia, a 2024 newspaper article, a library photograph record, Bates Savo's garbled OCR). Every event and outcome row stands `unconfirmed`; the class and weapon rows stand as `oob-check`'s rule of support sets them, and rows of this pass that disagree on one value are `disputed`.

### Where the format did not fit, and what was done

- **Doctrine and practice by navy and date** went to a new holding engagement, `submarine-war-1942` (id 1017), as events with kind `other` or `order`, `time` the date of the document or the date the statement is about, `actor` the navy or command. Undated judgements from post-war reports have an empty time and say so.
- **New outcome measures** were used without a vocabulary to check them (the column is free): `screen_note`, `zigzag_note`, `counterattack_note`, `depth_charges_dropped`, `depth_charges_received`, `torpedo_failure_note`, `speed_after_damage_kn`, `time_out_note`, `target_speed_kn` (none left after removing repeats), and three whole-war counts under a ship of `us-fleet-submarines`.
- **Japanese classes** are filed under the blocks of ship numbers the Technical Mission's S-01-1 prints (`jp-i-168-group` for I-168 to I-175, `jp-i-15-group`, `jp-i-176-group`, `jp-i-1-group`, `jp-i-121-group`, `jp-i-153-group`, `jp-ro-33-group`) and ONI 222-J's class sheets where they differ (`jp-i-5-group`, `jp-i-9-group`, `jp-ro-35-group`). The Japanese type codes are not in any row read.
- **`us-fleet-type-submarine`** is not a class: it holds the general figures of NAVPERS 16160 (1946) for the wartime fleet boat, and its name says so.

## What was read and what could not be got

Read, all as text:

- **US reports:** ONI Combat Narrative "Miscellaneous Actions in the South Pacific" (its chapter on the submarine encounters of 31 August to 15 September 1942; the address on file had moved, and the working web.archive.org copy is cited); War Damage Reports No. 19 (Saratoga, January 1942), No. 28 (O'Brien) and No. 61 (North Carolina) through web.archive.org copies of the NHHC pages, and the Wasp report on file; the BuShips Summary of War Damage of 17 October 1941 to 7 December 1942; Yorktown's Midway report; the Nimitz Graybook, volumes 1 and 2; Bates on Midway, the Coral Sea and Savo Island; CINCPAC's Midway report and Operation Plan 29-42; Cominch Battle Experience Bulletins 1, 3 and 4; WDR 58, the BuShips submarine war damage report of 1949 (sections II and III and Appendix I); OEG Report 51 (Antisubmarine Warfare in World War II, 1946); JANAC (1947); NAVPERS 16160; DANFS entries for about 30 ships.
- **US torpedoes:** Rowland and Boyd, U.S. Navy Bureau of Ordnance in World War II (1953), chapter 6; Jolie, NUSC TD 5436; two Naval Submarine League articles and a Naval Historical Society of Australia paper (all low: articles with no notes saved).
- **The Nautilus report:** only the narrative of her first war patrol as the Naval Submarine League printed it in 1992 (key `nsl-nautilus-wpr1`), not the report itself.
- **Japanese side:** the Technical Mission's S-01-1 (characteristics of submarines), S-17 (Japanese submarine operations), S-24 (anti-submarine warfare) and O-01-1 (torpedoes), all OCR of scans; USSBS interrogations Nav 72 (Captain Miwa), 74 (Kamide), 63 (Okamoto) and 56; Japanese Monograph 93 (Midway); USSBS Campaigns of the Pacific War; ONI 222-J (1945); combinedfleet.com's records of movement for about 50 Japanese boats and for Kako and Okinoshima.

Could not get:

- **Japanese Monographs 102, 105, 108, 110 and 111** (submarine operations, December 1941 to 1943). HyperWar's index lists them with the links removed; archive.org has none. So the Japanese orders of August and September 1942 rest on combinedfleet.com and the Technical Mission.
- **American war patrol reports** for S-38, S-44, Flying Fish, Grouper and the others, and the Nautilus report itself; the USSBS interrogations of the commanders of I-19, I-26, I-176 and I-21 (HyperWar has removed the links to Nav 107 and 108).
- **Saratoga's own report** of 31 August and any War Damage Report for that hit; the destroyers' reports for 31 August, 6 September and 6 June; HMAS Arunta's report on RO-33 (navy.gov.au refused); Gamble's report on I-123.
- **Any US screening doctrine of 1942 as a publication** (USF-10A, Current Tactical Orders), Cominch's Bulletin No. 2, and any order after the Saratoga and Wasp torpedoings telling carriers to keep speed or zigzag.
- **The ComSubPac Submarine Operational History.** H-Gram 009 was fetched but its two columns would not give a word-for-word quotation.

## 1. Every submarine attack on a warship, as events

### Japanese boats against American ships

| Date | Boat and target | What the target was doing | Fired, hit | Counter-attack and boat | Rows |
| --- | --- | --- | --- | --- | --- |
| 11 Jan | I-6 against Saratoga, south-west of Oahu | Not found (gap) | 3 Type 89 from 4,700 yards, 1 hit (combinedfleet); 6 killed; speed after 17, 18 or 16 knots by source | Destroyers began seven minutes after the hit and did not find her; she went to 330 feet | ev 16049 to 16051; out 16089 to 16095 |
| 6 Jun | I-168 against Yorktown and Hammann, Midway | Yorktown dead in the water under tow with Hammann alongside; five destroyers screening at 2,000 yards (Yorktown's report) | 4 from 1,900 yards (combinedfleet); on file 8035 to 8043 | About 40 charges; forward torpedo room and motor room flooded, batteries damaged; surfaced at 1640 and escaped | ev 16052, 16053; out 16096 to 16101 |
| 22 Aug | I-121 against Task Force 16 | Not stated | 1, passed between Enterprise and a cruiser (combinedfleet) | None recorded | ev 16023 |
| 31 Aug | I-26 against Saratoga, south-east of the Solomons | 13 knots; course 140 steady after the dawn launch from 0655, zigzag again from 0726, steady on 180 at 0741; destroyers on an arc 50 to 230 relative at 3,500 yards; TF 17 six miles off; area of 150 by 60 miles kept for days | 6 from 3,830 yards (combinedfleet), 1 hit; MacDonough saw the periscope close aboard and dropped two charges unset; one torpedo broached | Two destroyers dropped charges, Monssen stayed ten hours; I-21 heard 37 charges; the boat escaped. ONI says the Japanese never claimed the hit; the Technical Mission's list of claims has it | ev 16000 to 16011; out 16000 to 16031; pos 16000; ev 16359 |
| 6 Sep | I-11 against Hornet | Turned into the wind at 15 knots to launch | 4 as a snap shot (combinedfleet); none hit: an Avenger's depth bomb set one off, one passed North Carolina at 300 to 400 yards | No sound contact by the force; Russell attacked later with 6 charges; 80 percent of I-11's batteries wrecked, she could not dive and went to Truk | ev 16012 to 16022; out 16032 to 16040; ev 16306 |
| 15 Sep | I-19 against Wasp, North Carolina and O'Brien | Wasp turning back to base course at 16 knots after flight operations at 13; screen out of station, a 90-degree arc open; TF 17 seven miles off; North Carolina turning at revolutions for 19 knots | 6 (on file 3002); Wasp 2 hits (BuShips) or 3 (ONI), North Carolina 1, O'Brien 1 | About 30 charges (on file 3017); I-19 escaped; I-15 near and fired nothing; ONI: "two, probably not more than three" boats | ev 16034 to 16048; out 16047 to 16088; on file 3001 to 3017 |

Boats sighted and attacked before they could fire, or sunk, in the same waters:

- Aircraft from the carriers caught boats on the surface on 12, 23, 24 and 27 August: I-175, I-19, I-17, I-33 and I-121 (ev 16024 to 16029; USSBS's three screening boats attacked, ev 16264). None was sunk; I-175 and I-121 were damaged.
- I-9 against Grayson on 25 August: five attacks and 46 charges; ONI and DANFS say sunk, combinedfleet says she survived with a leak and returned to Truk (ev 16032; out 16041 to 16046).
- I-123 sunk by the destroyer-minelayer Gamble on 29 August off Guadalcanal, after several attacks from 0844 to 1147 (ev 16054; out 16102, 16103; pos 16001).
- RO-33 sunk by HMAS Arunta on 29 August off Port Moresby, after she had torpedoed the merchant Malaita (ev 16055; out 16104). RO-34 hunted for 18 miles on 23 August and escaped (ev 16056). RO-61, in the Aleutians, hit the tender Casco on 30 August and was depth charged (ev 16057; out 16105).
- I-11 attacked by Catalinas on 7 and 8 September (ev 16021, 16022). Task Force 17 attacked three boats on 6 to 7 September, "2 promising 1 negative" (ev 16306).

Later attacks named only by a Cominch lesson: Chester on 20 October (the torpedo probably fired from over 6,000 yards; the screen on a 3,000-yard circle) and Porter on 26 October, hit while stopped (ev 16398 to 16402).

### American boats against Japanese ships

| Date | Boat and target | Target doing | Fired, hit | Counter-attack | Rows |
| --- | --- | --- | --- | --- | --- |
| 2 to 10 May | S-47, S-44 and other S-boats off Rabaul and New Ireland | Escorted shipping | S-47 hunted three hours by two destroyers and two minesweepers; S-44 fired 2 and missed on 8 May, was outrun on 9 May and driven off on 10 May; S-38, S-39, S-41, S-43 and S-45 made no attack | | ev 16109 to 16116, 16161 to 16163, 16167 |
| 11 May | S-42 against the minelayer Okinoshima | In rain, at night | 4 Mark 10, 3 hits; sank | Destroyers from about 0520 to 1130, contact lost at 1430; water leaking into her control room | ev 16103 to 16107; out 16115, 16116, 16122, 16123; pos 16005; on file 2521 |
| 12 May | S-44 against the salvage ship Shoei Maru | Escorted | 4, 2 hits; sank | Escort dropped charges | ev 16108, 16112 |
| 4 Jun | Nautilus against a battleship, Arashi and a carrier | Mobile Force manoeuvring under air attack; the battleship estimated at 25 knots, 4,500 yards; the carrier stopped and burning | At 0825 two tubes, one did not fire, one missed; at 0910 one at Arashi from 2,500 yards, missed; at 1359 to 1405 three at the carrier, a fourth would not fire; she claimed three hits; Japanese survivors are reported to have said two missed and one hit was a dud | 42 charges in the day, from 0810 to the evening; Nagara and Arashi named by Bates | ev 16058 to 16087; out 16106 to 16110, 16112, 16120, 16121; pos 16002, 16003, 16007; on file 8018, 8044 to 8047 |
| 4 to 6 Jun | Grouper, Gudgeon, Cuttlefish, Tambor, Trout, Grayling at Midway | | Grouper fired 2 at a destroyer on sound and missed; the rest made no attack (could not close, held down by aircraft, outrun); Grayling was bombed by B-17s | Grouper over 170 charges and bombs; Tambor's periscopes damaged by two depth bombs | ev 16089 to 16102; out 16113, 16114 |
| 7 to 8 Aug | S-38 against Mikawa's force and Meiyo Maru | Cruisers at speed passed too close to fire; a transport with one destroyer | 2 at the transport, 2 hits; sank | Charges from 2330 | ev 16118 to 16124; out 16119; pos 16006; on file 4, 6 |
| 10 Aug | S-44 against Kako | Four heavy cruisers in column at 16 knots, no destroyer screen (combinedfleet), a floatplane overhead | 4 Mark 10 from 700 yards (or 900); 3 hits (combinedfleet, Bates) or all four exploded (DANFS); sank in minutes; 68 killed | Charges from 0812 without success | ev 16125 to 16132, 16160, 16414; out 16117, 16118, 16124; pos 16004, 16008; on file 133, 135, 778 |
| 28 Aug | Flying Fish against a battleship off Truk | Two destroyers and air cover | 4, two hits heard by sound (DANFS); then a salvo at a destroyer, missed | 36 charges; damaged, back to Pearl | ev 16137 to 16140; out 16125; ev 16135 |
| 2 to 4 Sep | Flying Fish against patrol vessels | | Torpedoes hit and did not explode; then sank one with 2 | 5 hours with two destroyers | ev 16141, 16142 |
| Aug to Sep | Trout, Amberjack, S-37, S-41, S-45, S-46, S-47, Greenling, Drum off Truk, Rabaul and the Solomons | | Trout 5 at a light carrier on 28 September, two explosions heard; Amberjack 4 at a transport and 6 at a cruiser, all missed; S-37 damaged a destroyer; S-45 lost depth control and could not fire; Greenling and Drum could not reach a firing position | Trout 45 charges and 10 more; S-44 hindered by air and surface patrols | ev 16136, 16143 to 16159, 16164 to 16166 |

JANAC credits American submarines, in these waters and months, with two Japanese warships sunk: Okinoshima and Kako (out 16115 to 16118), and Soryu jointly with carrier aircraft (out 16120, 16121).

## 2. Where the boats were and why

- **Midway, American.** CINCPAC planned on 20 May to put every submarine in the Hawaiian area on a scouting line west of Midway, first six and then six more (ev 16212 to 16214); five were on station from 26 May, eight more were ready from 24 to 30 May (ev 16210, 16211, 16225). Task Force 7 had 19 boats: twelve in the Midway Patrol on arcs at 200, 150 and 50 miles (three, six and two boats, Cuttlefish 700 miles west), three in the Support Patrol and four in the Oahu Patrol (ev 16168 to 16175; pos 16009 to 16014, 16021). All were on station by 3 June (ev 16175, 16179). CINCPAC's report says thirteen on the 200 and 150 mile circles (ev 16222). The plan's priority of targets was carriers, battleships, transports, cruisers, auxiliaries (ev 16224). On 4 June at 0710 nine boats were sent toward the enemy, and at 1525 all twelve to the 100-mile circle; at 0609 on 5 June they were drawn in to five miles round Midway on Tambor's report; the Support and Oahu patrols stayed put until released on 6 June (ev 16181 to 16193, 16223). Six boats returning from patrol were ordered to intercept the retreating Japanese (ev 16176, 16219).
- **Midway, Japanese.** Fifteen boats of Submarine Squadrons 3 and 5 and Division 13 were sent (ev 16198, 16230). Squadron 3 was to be on the KO line from 1 June, Squadron 5 on the OTSU line (ev 16194, 16195, 16227, 16229; pos 16015 to 16020); I-169 was on station on 31 May (ev 16228). Japanese Monograph 93 says the date was "N-1 day" and "it appeared that the enemy forces had already passed" (ev 16199); Bates says Task Force 16 crossed the OTSU line on 29 to 30 May and Task Force 17 on 31 May to 1 June (ev 16197). I-164 of Squadron 5 had been sunk by Triton on 17 May (ev 16236). The K operation needed boats at French Frigate Shoals; I-123 found American ships there and it was cancelled at 2123 on 31 May (ev 16203 to 16205, 16233). On 4 June the cordon was ordered west and on 5 June to line III; it swept with twelve boats and found nothing (ev 16206 to 16209, 16231). I-168 reconnoitred Midway from 2 June (ev 16202, 16232). USSBS says the fleet "advanced behind a submarine scouting line" (ev 16226), against Bates and JM 93.
- **The Coral Sea.** Six Japanese boats and two tenders (RO-33, RO-34, I-22, I-24, I-28, I-29) were ordered to destroy striking forces entering the Coral Sea, to destroy shipping and to reconnoitre (ev 16247, 16248). Four left Truk on 30 April for a line south-west of Guadalcanal by 5 May; RO-33 and RO-34 left Rabaul on 1 May to blockade Port Moresby; I-21 was off Noumea and I-27 off Brisbane (ev 16238 to 16244). On 7 May Inoue moved the boats bound for Townsville north of it; they were recalled to Truk about 9 May (ev 16254, 16256). The Japanese believed one or two Allied boats were in the Bismarcks; there were four (ev 16257). The six boats' position on 7 May is "not known exactly" to Bates (ev 16249). Four American boats were thought able to attack Shokaku on her way home (ev 16259); the concentration off Truk "should have been able to do more" (ev 16260, 16261).
- **The Solomons, August to September, Japanese.** I-121 and I-122 left Rabaul on 7 August to reconnoitre Guadalcanal and Tulagi (ev 16291); I-123 went to the Indispensable Strait (ev 16292). Squadron 1's I-9, I-15, I-17, I-19 and I-26 left Yokosuka on 15 August and were ordered on 20 August east of San Cristobal; Division 2 took a line east of the Santa Cruz Islands on 23 August, ordered to attack only supply ships or capital ships (ev 16265 to 16268, 16281 to 16283); I-11, I-31, I-174 and I-175 followed (ev 16285, 16287, 16290). On 26 August eight boats were ordered south and east of San Cristobal (ev 16271); on 10 September eight patrolled between Ndeni and San Cristobal (ev 16273); on 13 September a flying boat's report brought six or eight boats into a line (ev 16274, 16275); on 15 to 16 September the First Picket Unit (I-31, I-7, I-17, I-176) was formed (ev 16280, 16289); on 20 September seven boats left (ev 16276). USSBS names Forces A and B of six boats each (ev 16262, 16263). The Technical Mission says "SubRon 3 (10 subs)" was massed near the Solomons (ev 16358) and describes 16 boats on two lines 200 miles apart, the boats 30 miles apart (ev 16348).
- **The Solomons, American.** Task Force 42 at Brisbane had 11 S-boats on 6 July (ev 16297). In mid-August five fleet boats were off Truk and six at Kiska, two moved to Kavieng, leaving five off Japan (ev 16300, 16304). Ghormley asked for fleet boats at Guadalcanal on 14 August and was told none were available (ev 16301); his suggestion of 31 August to use submarines against a screening force north-east of the Solomons was answered that the few were already placed (ev 16302). CINCPAC believed Japanese boats were concentrating against the Solomons from 9 August (ev 16298, 16303).

## 3. Sighting reports by submarines

- **Tambor at Midway**: contact at 0215 on 5 June; Midway receipted at 0306; CTF 7 at Pearl had it at 0400 as "many unidentified ships"; it drew the patrol in to five miles at 0609; the amplifying report naming two Mogami-class cruisers was receipted at 0617 (ev 16093, 16096, 16186 to 16189; on file 7675). Its effect on Spruance is not in these rows (gap).
- **Grenadier** was the only boat to receive the 0925 contact report of 3 June, and about twelve hours late (ev 16177). **Cuttlefish** learned of the enemy only at 2200 on 3 June and reported a tanker at 0020 on 4 June (ev 16092, 16178). The B-17s were sent at a submarine's report (on file 7668, 7682).
- **I-168** reported from Midway on 2 June; the Japanese staff counted 14, or at least 16, American submarines (ev 16200 to 16202, 16232). **I-123's** report from French Frigate Shoals stopped the K operation (ev 16203, 16204).
- **The Coral Sea:** a boat attacked by Yorktown's scouts on 2 May made no report by Bates (ev 16250 to 16252), though combinedfleet says I-21 reported (ev 16245); I-29's report of an aircraft at 1840 on 9 May (ev 16255).
- **S-38** on 7 August: her report of Mikawa's force was received by Turner's commanders at 0738 on 8 August (on file 4 to 6); she could not attack, her port motor having failed (ev 16118 to 16121). **S-44** saw the cruisers sortie on 7 August (ev 16125).
- **Japanese boats in the Solomons** reported Enterprise's force on 25, 27 and 28 August (I-15, I-17, I-26, I-31; ev 16030, 16031, 16278, 16279, 16284, 16288) and Hornet's on 6 September by sound (ev 16286). Nothing came of them but chases ordered and not made (ev 16270, 16272).
- **What the Technical Mission and Miwa say of Japanese reporting:** in a Solomons episode the boat that made contact did not report until it was well clear of the line (ev 16345); at the Wasp attack two boats made contact and neither reported to the group commander (ev 16349); reports went to headquarters and not to the carriers (ev 16370). CINCPAC believed the Wasp's attackers reported by radio (out 16084).

## 4. What made an attack succeed or fail

| Statement | Navy and date | Rows | Strength |
| --- | --- | --- | --- |
| In all three attacks of 31 August to 15 September the ships made 13 to 15 knots, on or soon after a turn into the wind or back to base course, so zigzag had stopped; destroyers regaining station at high speed had sound gear that did not work; Wasp's screen left a 90-degree arc open; it was never clear how many boats attacked | USN (ONI), 1942 | out 16014 to 16018, 16047 to 16050, 16065 | high |
| Saratoga's force spent days in an area 150 by 60 miles; Wasp crossed her own track 12 times in three and a half days | USN (ONI) | out 16012, 16016 | high |
| The attack on Yorktown succeeded because a carrier dead in the water is hard to screen | USN, Yorktown's captain | out 16097 | high |
| Task forces that stay several days in one area at low speed and are seen are in danger from submarines; the Japanese coordinate air and submarine scouting | Cominch, on the Coral Sea | ev 16392, 16393 | high |
| A carrier needs 10 destroyers, preferably 12, to be well protected against submarines; a screen drawn in against aircraft must still be thought of against submarines; zigzag plan 8 used too often; a screen on 3,000 yards where 5,000 to 6,000 might be better; a ship stopped in submarine waters is in danger | Hornet's captain and Cominch, October 1942 | ev 16396 to 16403 | high |
| Early 1942: Task Force 17's anti-submarine defence of 4 destroyers and air patrols; a cruiser at 15 knots with a suitable zigzag "reasonably safe" | CINCPAC staff | ev 16410, 16411 | medium |
| "Avoid remaining in same locality to prevent concentration of enemy submarines" (to Task Force 17, 4 September); "plan to avoid a repetition" (late 1942) | COMSOPAC; CINCPAC | ev 16305; ev 16409 | high; medium (date inferred) |
| Trailing submerged a ship of even moderate speed is futile; a cautious commander accomplishes little; only two submarines of all those at Midway attacked; the difficulty of attacking well-screened formations | Cominch, 1942 | ev 16389 to 16391, 16395 | high |
| American skippers with the same torpedoes ranged from over 60 percent hits to under 10; 40 percent of attacks were at periscope depth between 1900 and 0600 (undated, of 1942 to 1943) | BuOrd | wp 16036, 16037 | high |
| Japanese: boats saved for the American fleet; captains' "caution"; a boat forbidden to attack anything smaller than a cruiser or merchant under 5,000 tons; ambush on the expected path the commonest employment; no concerted group attack, the prime weakness; stations set before sailing | Technical Mission (1946) | ev 16340 to 16351 | medium |
| Japanese: boats were auxiliaries to the fleet; the change to other work came after Midway; supply runs to Guadalcanal because the American fleet offered no battle and Japanese strength was too small; American air scouting "very severe" | Miwa, 1945 | ev 16366 to 16374 | high |
| Japanese evasion: 40 metres early in the war was deep enough against aircraft; under attack by ships, near test depth at least speed; a "period of grace" of 24 hours after a landing | Technical Mission | ev 16352 to 16354 | medium |
| Japanese firing data: 1,500 metres and a 3-degree spread (I-58, 1945); 1,000 metres (RO-115, 1944) | Technical Mission | ev 16364, 16365 | medium, low; not 1942 |

The rows give firing ranges for the 1942 attacks themselves: I-6 4,700 yards, I-26 3,830, I-168 1,900, I-19 985 (on file 3002), S-44 700, Nautilus 2,500 and 4,500, Trout 1,500 (ev 16050, 16010, 16053, 16127, 16130, 16062, 16066, 16145). No row states an attack doctrine of 1942 for either navy (gaps).

## 5. Torpedoes

- **Mark 14 depth.** Rowland and Boyd: complaints of deep running, prematures and duds from the start; net firings ordered in the summer of 1942; the services told on 1 August 1942 of a 10-foot error, allowed for in settings until cured (wp 16015 to 16020, 16022). The Naval Submarine League and Australian papers (low) add the Newport tests of February to March 1942 (4 feet), Blandy's letter of 30 March, the Albany net tests of 20 to 21 June (11 feet, or 11 to 15) and 18 July (Saury, all five deep), and the cure by moving the pressure port in the second half of 1943 (wp 16043 to 16046; ev 16421 to 16425).
- **Mark 6 exploder.** Early in 1942 BuOrd advised shots set 10 feet beneath the keel; in August 1942 the first sign that the exploder caused prematures, up to 50 feet off; submariners thought about 10 percent of shots premature, BuOrd not over 2 percent; arming at 450 yards; a redesign ordered before the end of 1942; CINCPAC ordered the magnetic feature off on 24 July 1943 (Rowland and Boyd) or 24 June (NSL); the South-West Pacific kept it into 1944 (wp 16021, 16023 to 16033; ev 16416 to 16418). Contact-exploder duds were cured after Tinosa's 15 torpedoes of 24 July 1943 and the tests at Pearl Harbor, with a lighter firing pin (wp 16034, 16038 to 16042; ev 16419, 16420). ComSubPac in mid-1943: 9 contact-exploder failures in more than 1,800 torpedoes fired, over 600 hits (wp 16035).
- **Mark 10 (S-boats).** 3,500 yards at 36 knots, a Mark 3 contact exploder, no magnetic feature; ran 4 feet deep, the fleet told in January 1942 and setting shallower (wp 16005 to 16014). S-42 and S-44 hit with them in May and August (ev 16107, 16130).
- **Japanese.** Type 89 (air, 21-inch): developed 1929, in service to the end of 1942, used because later types were short (wp 16050 to 16054). Type 95 (oxygen): production from 1938; trouble with the first air vessel led to about 300 Type 96 (38 percent oxygen) in 1942 to 1943 (wp 16055 to 16060, 16062). Type 92 electric from 1942 (wp 16061). I-6 fired Type 89 at Saratoga (ev 16050); ONI 222-J gives Type 89 for the I-121, I-153 and I-5 classes (cl 16060, 16064, 16070). BuShips judged the North Carolina warhead close to the Type 89's, and called the torpedoes that hit Saratoga twice and North Carolina "deep-running" (out 16072, 16027, 16080, 16091).
- **Failures seen in the attacks:** Nautilus's tubes that would not fire and her dud (out 16108 to 16110, 16112); Flying Fish's torpedoes that hit and did not explode on 2 September (ev 16141); an I-26 torpedo that broached (ev 16006); a Japanese torpedo set off by a depth bomb (ev 16014).

## 6. Anti-submarine practice

- **Japanese, by the Americans' reckoning:** charges set too shallow, "especially during the first two years of the war"; a lack of persistence, quick to believe a boat sunk; no way to know a boat's depth; listening gear fairly good at 2,000 metres or more when slow, echo ranging like the American QC of about 1937; fleet destroyers carried about 30 charges (WDR 58: ev 16307 to 16313). The Technical Mission's S-24: patterns to cover all depths; a maximum setting of 120 metres; convoy destroyers carried 90 and dropped 36 on a sure contact; no bathythermographs; training "practically non-existent" after shakedown; the Grand Escort Fleet only from November 1943 (ev 16327 to 16337). Type 95 charge settings early 98 and 197 feet, later 292; the Type 2 of 1942 to 480 feet (wp 16000 to 16004).
- **The "May incident"** of June 1943 rests only on Wikipedia here (ev 16412, 16413, low): no primary row ties any change in Japanese settings to it.
- **American boats damaged in 1942:** nine depth-charge or bomb cases from WDR 58's appendix, with charges counted (ev 16317 to 16326); over the war 110 damage cases, 58 by depth charge, 52 boats lost (out 16126 to 16128, whole war). In these waters in 1942 the rows show American boats held down for hours (S-42, Nautilus, Grouper, Flying Fish, Trout) and none lost.
- **American, against Japanese boats, in these waters in 1942:** of the boats that fired at a warship, I-6, I-26, I-19 and I-168 escaped, I-168 damaged; I-11 escaped crippled; I-123 and RO-33 were sunk, neither while attacking a warship; I-9 escaped though claimed (section 1). The OEG counts 11 Japanese boats sunk from January to September 1942 against 76 ships sunk, and 18 from October 1942 to June 1943 against 19 (ev 16384 to 16387).
- **What the Japanese feared:** aircraft most, and the coordination of destroyer and aircraft; damaged boats returned "very frequently"; losses heaviest at Guadalcanal, Leyte and the Aleutians (Miwa, ev 16375 to 16380). Japanese anti-submarine aircraft were of little account early in the war (ev 16381 to 16383).
- **American screening doctrine of 1942** was not found as a publication (gap); what the rows give is practice: destroyers at 3,500 yards on an arc (Saratoga), 4,000 yards in a circle with cruisers at 2,500 (Wasp), 2,000 yards round Yorktown, and the lessons of section 4.

## 7. The boats' classes

| Class or block | Surface / submerged speed (knots) | Submerged endurance | Range | Depth | Tubes, torpedoes | Rows |
| --- | --- | --- | --- | --- | --- | --- |
| US S-boat, S-1 group (S-38) | 14.5 / 11 | gap | gap | gap | 4 bow, 0 stern (inferred); 12 Mark 10 | cl 16071 to 16076; 16127, 16128, 16131 to 16134, 16180, 16181 |
| US S-boat, S-42 group (S-42 to S-47) | 14.5 / 11 | gap | gap | gap | 4 bow, 0 stern (inferred); 12 Mark 10 (and wp 16014) | cl 16077 to 16083; 16129, 16130, 16135 to 16138, 16182, 16183 |
| US Narwhal (Nautilus) | 17.4 / 8 | gap | 18,000 or 15,000, no speed | gap | 4 bow, 2 stern, 4 topside; 24; torpedo gap | cl 16084 to 16089; 16139 to 16150 |
| US Sargo | 20 / 8.75 | gap | 11,000 at 10 (Wikipedia); 15,000, no speed | gap | 4 bow, 4 stern; 24 | cl 16106 to 16113; 16151 to 16158 |
| US Tambor | 20 / 8.75 | 48 hours at 2 knots (Wikipedia) | 11,000 miles at 10 (Wikipedia); 15,000, no speed | 250 to 300 feet test (Wikipedia) | 6 bow, 4 stern; 24 | cl 16090 to 16097, 16121 to 16123; 16159 to 16163, 16166 to 16169 |
| US Gato | 20 or 20.25 / 8.75 | 48 hours at 2 knots (Wikipedia) | 11,000 at 10 (Wikipedia, NavSource) | 300 feet test (Wikipedia) | 6 bow, 4 stern; 24 | cl 16098 to 16105, 16124 to 16126; 16164, 16165, 16170 to 16177 |
| US fleet type, general (NAVPERS 16160) | about 21 / about 10 | | over 12,000 | | 6 bow, 4 stern; Mark 14 (Mark 10 in reserve) | cl 16114 to 16120; 16178, 16179, 16184 to 16186 |
| I-1 to I-6 (I-6) | 17 / 8 | | 20,000 at 10 | 75 m | 4 bow, 2 stern; 20 or 17 (Wikipedia, disputed); ONI's I-5 class 4 bow, 2 stern (or 6 bow, disputed), 20, Type 89 with Type 95 reported | cl 16000 to 16006, 16070; 16187, 16189 to 16191, 16196 to 16212 |
| I-15 block (I-15, I-17, I-19, I-26 ...) | 22 / gap | 32 hours at 3 knots | 14,000 at 16 | 100 m; 327 feet (ONI) | 6 bow, 16 | cl 16015 to 16022, 16048 to 16052 |
| I-168 to I-175 | 23 / 8 | 22 hours at 3 knots | 14,000 at 10 | 75 m | 4 bow, 2 stern; 14 and Type 89 (Wikipedia only) | cl 16007 to 16014; 16193, 16194, 16222 to 16225 |
| I-176 to I-185 | 23 / 8 | 10 hours at 5 knots | 8,000 at 16 | gap | 6 bow, 12 | cl 16023 to 16030, 16053 to 16056 |
| I-153 to I-167 | gap | gap | 10,000 at 10 | 60 m | 6 bow, 2 stern (ONI), 16 | cl 16036 to 16039, 16061 to 16064 |
| I-121 to I-124 | gap | gap | 8,000 at 12 | gap | 4 bow; 8 or 20 | cl 16031 to 16035, 16057 to 16060 |
| RO-33, RO-34 | 18.9 / 8.4 | 25.7 hours at 3.5 knots | 3,000 at 16 | 75 m | 4 bow; 10 (Wikipedia only) | cl 16040 to 16047; 16195, 16226 to 16229 |
| I-9 to I-11 (I-9 class) | | | | | 6 bow; 20 (ONI) or 18 (Wikipedia), disputed; Type 89, Type 95 reported | cl 16069; 16188, 16192, 16213 to 16221 |

The Japanese figures are the Technical Mission's "as built" table (1946) read through OCR; the depth is a "submergence depth" the report does not call test or operating. The I-9 class carried a floatplane (cl 16069). Twenty-six boats are tied to a class by a row (sh 16000 to 16025).

## 8. Gaps

The last 61 entries of `gaps.csv`. The ones that matter most to a rule:

- the Japanese Monographs on submarine operations (102, 105, 108, 110, 111) and any Japanese record of the August and September orders other than combinedfleet.com;
- the American war patrol reports of 1942 (Nautilus, S-38, S-44, Flying Fish, Grouper, Trout), and any 1942 count of duds, prematures and misses per patrol;
- the reports of Saratoga and her destroyers for 31 August, of Hornet's force for 6 September, of the destroyers round Yorktown on 6 June, of Gamble and of Arunta;
- the arrival of each boat of the Midway cordon (only I-169's is stated);
- the stations of the American boats off Truk, Rabaul, Kavieng and Bougainville, and of the Japanese boats in the Coral Sea beyond the line south-west of Guadalcanal;
- any statement of attack doctrine of 1942 for either navy (ranges, spreads, target speeds accepted), and the American screening doctrine of 1942 as a publication;
- an official source for the submerged endurance and test depth of the American classes (the torpedoes carried by each are now on file, section 9);
- which Japanese boats carried the Type 95 and which the Type 89 in 1942, beyond the firings and loads of section 9;
- a rate of counter-attack, damage or loss per attack for either navy in 1942;
- when Japanese depth-charge settings changed, from a primary source.

## What a rule could and could not rest on

This section says what the rows give and do not give. It proposes no rule and no figure.

**Where the rows give a direction and no size.**

- *Meeting a force.* The rows give the boats on station and the dates (19 American boats round Midway from 3 June; 15 Japanese boats meant for 1 June and late; six to sixteen Japanese boats east of the Solomons from 20 August to 20 September), and they give the encounters that followed (section 1 and 3). Anyone may count encounters against boat-days from them, but no source states such a rate, and the counts are a handful of cases.
- *A force's conduct.* Every attack that hit in 1942 struck a ship that was slow, steady or stopped: Yorktown dead in the water, Saratoga at 13 knots on a steady leg, Wasp at 13 to 16 knots turning after flight operations, Kako in column without a screen, a carrier stopped and burning at Midway. ONI, Cominch and COMSOPAC said so at the time. The rows support a rule in which speed, zigzag, keeping one area and flight operations matter; they give no figure for how much.
- *The screen.* The rows give screens of 2,000 to 4,000 yards that the boats got inside or under, Cominch's wish for 10 to 12 destroyers, and screens disordered by flight operations and rescue. They give no figure for what a destroyer more or less did to the chance.
- *Counter-attack.* Of the attacking boats, American and Japanese, in these waters in 1942, the rows show none sunk while attacking a warship (I-168 damaged, I-11 crippled; Nautilus, S-42, Flying Fish and Grouper held down for hours). The two Japanese boats sunk (I-123, RO-33) were found by escorts going about other business. That is a direction; the cases are too few for a share.

**Where the two navies differed by doctrine and date.**

- *Japanese:* boats as scouts and ambushers on the expected path of the American fleet, in lines set before sailing, without concerted group attack, with reports going to headquarters and some not made; capital ships and carriers the target; the change to supply work after Midway and at Guadalcanal from late 1942 (sections 2 to 4). Japanese anti-submarine practice: shallow settings, little persistence, no depth estimate, weak echo ranging, few charges in fleet destroyers, a reorganisation only in November 1943 (section 6).
- *American:* boats held round Midway as a defensive screen and then drawn in; few fleet boats in the South Pacific in August and September (five off Truk; S-boats from Brisbane, old and slow); captains judged too cautious in 1942; the Mark 14 running about 10 feet deep until August 1942 was found and then allowed for, with exploder faults not cured until the second half of 1943. The Mark 10 of the S-boats ran 4 feet deep, was allowed for from January 1942, and hit.
- *Torpedoes by date:* for the Mark 14 the rows date the depth fault (found June to August 1942) and the exploder faults (prematures from August 1942, the magnetic feature off from June or July 1943, the contact pin mended from September 1943). For the Japanese, the Type 89 served to the end of 1942 beside the Type 95; the rows say nothing dated of their failures beyond the Type 95's first air vessel.

**Where there is nothing.**

- No rate of hits per torpedo for either navy in 1942 from any source; Rowland and Boyd's spread of 10 to 60 percent by captain is undated, and ComSubPac's 1,800 torpedoes and 600 hits are of mid-1943.
- No statement of either navy's attack doctrine of 1942 (range, spread, speeds accepted).
- No American screening doctrine of 1942 as written.
- No official American figure for submerged endurance or test depth.
- No primary statement of when Japanese depth-charge settings changed.

## Where sources disagree

Each has its own rows.

- **Wasp's hits:** two (BuShips, out 16063) against three with a fourth ahead (ONI, ev 16036; on file 871 to 872).
- **O'Brien's hit:** near the stem on the port side (WDR 28), the stem (ONI), the starboard bow (BuShips Summary, out 16079), the port quarter (combinedfleet, on file 3009).
- **North Carolina:** frame 46 (WDR 61, ONI) against 55 (Summary, out 16080); five missing (WDR), five or six killed (ONI, DANFS), none killed and three unaccounted for (CTF 17's first report, out 16082); speed after 18, 24 or 25 knots (out 16055, 16085, 16066, 16067).
- **Saratoga, January:** speed after 17, 18 or 16 knots; 350 or 500 miles from Oahu; DANFS names the boat "I-16".
- **Saratoga, 31 August:** the destroyer sent to the radar contact was Farenholt (ONI) or Farragut (combinedfleet) (ev 16001, 16008); ONI says the Japanese never claimed the hit (out 16011), but the Technical Mission's list of claims has it (ev 16359); ONI and CINCPAC thought Monssen probably sank the boat (out 16009, 16026), which survived.
- **6 September:** the Technical Mission's claim list credits I-12 with two hits on a carrier (ev 16360); combinedfleet credits I-11, with no hit.
- **I-9 on 25 August:** sunk (ONI, DANFS) against survived (combinedfleet) (out 16044 to 16046).
- **Nautilus at Midway:** the carrier was Soryu (Nautilus, Bates, JANAC) or Kaga (Territory Stories, low); two or three hits (Bates) against two misses and a dud (Japanese survivors as printed by the NSL); her 0825 target a battleship of the Ise class (Nautilus) or Kirishima (combinedfleet); JANAC prints Soryu's position as 179-17 E, Nautilus 179-17 W (pos 16007, 16003).
- **Kako:** fired at 0706 (combinedfleet, Tokyo time), 0806 (DANFS) or 0908 (Bates); 700 or 900 yards; three hits or four.
- **Okinoshima:** 11 May (Bates, DANFS) against 12 May (combinedfleet, Japanese date).
- **Meiyo Maru:** "Meigo Maru", 5,628 tons (DANFS) against Meiyo Maru, 5,268 tons (JANAC).
- **Tambor's contact:** 0215 on 5 June (Bates) against 0216 on 6 June (DANFS).
- **Flying Fish, 28 August:** two hits heard (DANFS); no other row.
- **The Midway cordon:** USSBS has the fleet advancing behind it; Bates and JM 93 have it late. Bates counts Squadron 5 as seven boats, but I-164 had been sunk; Squadron 3 four or five boats. CINCPAC counts 13 American boats on the two outer circles, Bates nine.
- **The line of 13 September:** six boats (I-26 and I-15 pages) or eight (I-21 and I-24 pages).
- **CINCPAC's order to turn off the magnetic feature:** 24 July 1943 (Rowland and Boyd) against 24 June (NSL); the South-West Pacific's: spring 1944, December 1943 or March 1944.
- **The drop test of 1943:** 90 feet (NSL) against 900 (Rowland and Boyd).
- **Depth charges in a Japanese destroyer:** about 30 in fleet destroyers (WDR 58) against 90 in convoy destroyers (S-24).
- **Class figures:** I-15 block 2,212 tons (Technical Mission) against 2,100 (ONI); I-176 1,630 against 1,609; I-121 torpedoes 8 against 20; Tambor's complement 79 against 59 and her displacement 1,476 against 1,475 (two DANFS entries); Sargo's complement 55 against 64; S-47 800 tons against S-44's 850.

## 9. Second pass: the classes' torpedoes (2026-11-22)

Read later the same day for decision 0218's figures: torpedoes carried, bow and stern tubes, the speed at which a range is quoted, which torpedo each class or boat carried in 1942, and the Type 96's warhead. Two Sonnet readers staged statements with their quotations; the lead checked every quotation by program against the saved text (all passed), read two figures off page images (S-01-1's I-1 and I-9 blocks, O-01-1's Type 96 charge), and wrote the rows. Rows: `ship_class_facts.csv` 16127 to 16229, `weapon_facts.csv` 16063 to 16065, `engagement_events.csv` 16426 to 16435, `sources.csv` 16014 (FM 30-50, the Army's identification manual of US Navy ships, October 1941) and 16015 (the Fleet Type Submarine series pamphlet "21-Inch Submerged Torpedo Tubes"), and six entries at the end of `gaps.csv`. No existing row was changed.

**Closed.**

- *Torpedoes carried, US:* S-boats 12 (FM 30-50, HyperWar, NavSource; cl 16127, 16129, 16132, 16134, 16136, 16138); Narwhal, Sargo, Tambor and Gato 24 (HyperWar and NavSource; cl 16141, 16144, 16148, 16153, 16155, 16161, 16166, 16172, 16175). No source separates reloads from the torpedoes in the tubes.
- *Torpedoes carried, Japanese:* only from Wikipedia, except ONI's I-9 class. I-168 group 14 (cl 16222, unconfirmed); I-1 group 20 for the J1 boats against 17 for I-6 (cl 16202, 16203, disputed: two types in one block); I-9 group 20 (ONI) against 18 (Wikipedia) (cl 16214, 16218, 16220, disputed); I-5 group 20 (cl 16212, agreeing with 16070); RO-33 group 10 (cl 16226, 16227, unconfirmed).
- *Bow and stern tubes:* S-boats 4 bow (HyperWar, unconfirmed) and 0 stern (inferred from 4 bow and FM 30-50's 4 in all; cl 16131, 16133, 16135, 16137); Narwhal 4 and 2 (cl 16139, 16140, 16142, 16143, 16146, 16147), with 4 topside tubes NavSource dates to Nautilus's refit of July 1941 to April 1942 (noted, not filed: no torpedo named for a fit); Sargo 4 and 4 (cl 16151, 16152, HyperWar only, unconfirmed); Tambor and Gato 6 and 4 (the tubes pamphlet: every boat SS-198 and higher but SS-204 and 205; HyperWar, NavSource; cl 16159, 16160, 16162 to 16165, 16170, 16171, 16173, 16174); the fleet type 6 and 4 (NAVPERS 16160, cl 16178, 16179). Japanese: I-1 group 2 stern (S-01-1, read off the page image, cl 16187; Wikipedia agrees); I-9 to I-12 six forward (S-01-1, cl 16188; Wikipedia agrees); ONI's I-5 class 4 bow and 2 stern against Wikipedia's 6 all bow (cl 16206, 16207, 16210). Nishida's totals (cl 16189 to 16195).
- *Range and its speed, US:* 11,000 miles at 10 knots for Gato (NavSource and Wikipedia, cl 16176, 16177), for Tambor and Sargo from Wikipedia only (cl 16157, 16158, 16169). Other statements give a range with no speed: Narwhal 18,000 (HyperWar) or 15,000 (a press caption of about 1928 to 1930), Sargo 15,000, Tambor 15,000 (a 1940 caption) (cl 16145, 16149, 16154, 16167).
- *Which torpedo, US:* the Mark 10 for the S-boats (Rowland and Boyd: built for the R and S boats that could not fire the longer Mark 14; NavWeaps; cl 16180 to 16183); the Mark 14 for the fleet type in general (NavWeaps, and the tubes "designed primarily for the Mark 14"), with the Mark 10 as a reserve weapon (cl 16184 to 16186). No source names the torpedo by class (Narwhal, Sargo, Tambor, Gato) or by date.
- *Which torpedo, Japanese:* class statements: ONI gives Type 89 for its I-5 and I-9 classes, each with "Type 95 also reported" (cl 16208, 16209, 16215, 16216); Wikipedia gives Type 89 for the I-168 group and I-6, Type 95 for the J1, A1 and RO-33 types (cl 16204, 16205, 16221, 16225, 16229, unconfirmed). Firings and loads named by combinedfleet (ev 16426 to 16435): I-17 fired the old 6th Year Type in December 1941; I-26 embarked Type 95 at Kwajalein on 11 January 1942 and fired a Type 89 at Coast Trader on 7 June; I-123 fired four Type 89 off Darwin on 20 January; I-7 and I-3 fired Type 89 in April; I-4 fired two Type 96 on 6 April; I-21 fired Type 95 on 5 and 7 May (both of the second pair premature). With I-19's six Type 95 at Wasp (on file 3002) and I-6's Type 89 at Saratoga (ev 16050), the record shows both types in use by the same squadrons in 1942, and I-26 firing Type 89 after embarking Type 95.
- *The Type 96 warhead:* 405 kg (891 lb) in the Technical Mission's O-01-1 against 550 kg (1,213 lb) of Type 97 explosive in NavWeaps (wp 16063 to 16065, disputed). The Technical Mission's 405 kg is the same as its Type 95 Model 1 charge (wp 16056), from which the Type 96 was made.

**Still gaps** (the six new entries of `gaps.csv`):

- the range and its speed for the S-boats; the endurance speed of Narwhal; a range with speed for any US class from an official source;
- reloads apart from loaded tubes, for every class;
- the torpedo by class and date for Narwhal, Sargo, Tambor and Gato;
- a primary statement that the S-1 and S-42 groups had no stern tube;
- torpedoes carried for the I-168 and RO-33 groups from any source but Wikipedia (ONI has no sheet for either);
- the torpedo type for I-15, I-17, I-168, I-11, RO-33, I-9, I-31, I-174, I-175, I-121, I-122, I-24, I-176 and RO-34 in 1942: their records of movement name none (I-168's four at Yorktown and Hammann are not typed).

**Where the format did not fit.** `fits.csv` ties a weapon to a carrier with a count of tubes or mounts; no source here gives a count tied to a torpedo type, so the class statements went to `torpedo_model` in `ship_class_facts.csv` (a type with no count, as that attribute is defined) and the boat statements to engagement events in `submarine-war-1942`. There is no attribute for a boat's torpedo type outside a firing.

**Rows this pass left for the architect.** Wikipedia's Type J1 page (6 bow tubes) disagrees with row 16004 (the Technical Mission, four forward); it is filed `alternative` (cl 16198) so as not to change 16004, and is not settled by anyone. NavSource's 11,000 miles for Gato was not filed as `endurance_nm`, because a second agreeing source would require 16125 (Wikipedia) to change from `unconfirmed` to `preferred`; the speed is filed (cl 16176).
