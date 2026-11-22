# Carrier doctrine by navy and date, 1941 to 1945

Read on 2026-11-22 for the architect, who is to write three rules that apply to every battle (decisions 0211 and 0212, on the branch `carriers-plan`): how the groups of a carrier strike are divided among the ships they find; fire aboard a ship that can grow and destroy her; and how likely a bomb is to set fire among a carrier's own aircraft and fuel. It asks of the record what differed between the United States Navy and the Imperial Japanese Navy (and, where cheap, the Royal Navy), and when it changed. It settles nothing; every new row is `unconfirmed`.

The reading was done by six Sonnet researchers, each on one question, who staged rows with the quotation and the text file it came from. The lead then checked every quotation by program against the downloaded text (whitespace and case ignored; each fragment of a quotation joined by " ... " checked on its own), threw out duplicates of rows already on file, widened 24 quotations that were cut short to the source's whole sentence, and dropped the rows that rest on an unauthorised copy of a copyrighted book (below). All 311 rows that remain passed the check.

How rows are cited here: `ev` is `engagement_events.csv`, `out` is `engagement_outcomes.csv`, `tr` is `training_facts.csv`, each with its id. The three files share the id range 15300 upward, so the prefix matters.

## What the rows hold

| File | Ids | Rows | What |
| --- | --- | --- | --- |
| `engagement_events.csv` | 15300 to 15408 | 109 | Strike doctrine and its practice by battle (engagement `pacific-air-doctrine` for doctrine; the battle's key for practice); aircraft handling, damage-control orders and the Royal Navy's notes (the new engagement `carrier-practice-1941-1945`) |
| `engagement_outcomes.csv` | 15300 to 15462 | 163 | One ship in one battle: what her aircraft were doing when hit (`aircraft_aboard_note`), the fire (`fire_note`), the result (`damage_effect`), how it was fought (`conduct_note`), the source's judgement (`verdict_note`); ships of 1944 and 1945 under `carrier-practice-1941-1945` |
| `training_facts.csv` | 15300 to 15338 | 39 | A navy's damage control, training and equipment over a period (`usn-surface-fleet`, `ijn-surface-fleet`, `usn-cruiser-destroyer-crews`; `damage_control_note`, `training_note`) |
| `sources.csv` | 15300 to 15309 | 10 | New sources |
| `engagements.csv` | 1016 | 1 | `carrier-practice-1941-1945`, a holding place like `search-tempo-1942` |
| `gaps.csv` | last 49 entries | 49 | What was looked for and not found |

By confidence: of the 311 rows, 104 are `high` (action reports, War Damage Reports, CINCPAC, ONI, USSBS interrogations in clean text), 149 `medium` (OCR of scans, the Technical Mission's reports, scholarly articles) and 58 `low` (combinedfleet.com pages that cite no document, the 1945 ONI compilation's garbled OCR, and USF-74 known only second-hand).

### Where the format did not fit, and what was done

A navy's doctrine or practice at a date is not a fact about one ship or one engagement, and no file holds it cleanly.

- **Strike doctrine** went where the doctrine rows of `strike-sizes.md` already are: engagement `pacific-air-doctrine`, with `time` the date of the document.
- **Aircraft handling and damage-control practice, orders and letters, and the carriers of 1944 and 1945** went to a new holding engagement, `carrier-practice-1941-1945` (id 1016), because `pacific-air-doctrine` is named for the weight of air attack and no battle on file holds Taiho, Franklin or the Bureau of Ships' letters.
- **A navy's damage control over a period** went to `training_facts.csv`, which has `damage_control_note` and a date range, under `usn-surface-fleet` and `ijn-surface-fleet`. Those entities are named for the surface fleet; here they cover the carriers too, and each row's note says so where it matters.
- **Proposed, not done:** entities `usn-carrier-forces`, `ijn-carrier-forces`, `rn-carrier-forces` and attributes `strike_doctrine_note` and `aircraft_handling_note` in `training_facts.csv` (a change to `crates/oob-check/src/training.rs`), or a file of navy doctrine by date. If the architect wants either, the rows here can be moved with their ids.

## What was read and what could not be got

Read, all as text (HTML, a PDF's text layer, or OCR run locally on a scan):

- **US doctrine and reports:** the CINCPAC report on Midway of 28 June 1942; Cominch Battle Experience Bulletins 1, 3 and 4; the ONI Combat Narratives of the Coral Sea, Midway, the Eastern Solomons and Santa Cruz; the Naval War College analyses of the Coral Sea (1947) and Midway (1948) (archive.org OCR); USF 10B (1945); the Yorktown and Lexington action reports of the Coral Sea and Yorktown's of Midway (HyperWar); Enterprise's reports of 24 August and 26 October (cv6.org through web.archive.org).
- **War Damage Reports:** Lexington, Yorktown, Wasp, Hornet (No. 30), Franklin (No. 56), Randolph, the 1943 summary, No. 29 (the Savo cruisers), Boise (No. 24), San Francisco, Portland and Pensacola, and the Bureau of Ships' summary of bomb damage to British ships of 1942.
- **Japanese side:** Nagumo's Midway report (ONI translation); USSBS interrogations (among them Amagai, Kaga's air officer; Ohara, Soryu's executive officer; Aoki, Akagi's captain; Kawaguchi, Hiryu's air officer; Ozawa; Kurita; Watanabe; Fuchida); Japanese Monograph 97 (the Pearl Harbor orders, reconstructed after the war); the ONI compilation of Japanese bombing and torpedo doctrine of 10 February 1945 (pages not read before); the US Naval Technical Mission to Japan's S-84(N) *Japanese Damage Control* (February 1946), A-11 *Aircraft Arrangements and Handling Facilities* (January 1946), S-06-2 (Taiho, Musashi, Yamato, Shinano) and S-06-3 *Japanese Records of Major Warship Losses* (January 1946), all OCR of scans; Tully, "Calculating Scenarios in the Loss of CV Shokaku" (IJNH, 2002); Tully, Parshall and Wolff, "The Sinking of Shokaku" (1998, combinedfleet.com); combinedfleet.com's records of movement for the carriers and cruisers.
- **Secondary:** Wildenberg, "Midway: Sheer Luck or Better Doctrine?" (Naval War College Review, 2005); Cox, "McClusky at Midway" (a web essay quoting Rigby's biography of McClusky, which quotes USF-74 of 1941).

Could not get, or did not use:

- **The texts of USF-74 (1941), USF-74B (1944), USF-75, USF-77 and PAC-10 (1943).** Not on HyperWar (its index lists USF 74B and USF 77 without links), archive.org or DTIC as far as was searched. USF-74 of 1941 is known here only through Cox quoting Rigby, so those rows are `low`.
- **Herder, *US Fast Carrier Task Force Tactics 1943-45* (Osprey, 2020).** A researcher read it in an unauthorised online copy. The nine rows from it (USF-74B's rule against putting all bombers on one carrier, the task group target coordinator, Mitscher's instructions of May 1944, PAC-10's date) were dropped, because the source policy reads copyrighted books only in hand. Its points stand as leads in `gaps.csv`.
- **Parshall, Dickson and Tully, "Doctrine Matters" (NWC Review, 2001), and Parshall, "A Tale of Three Whoppers" (NWC Review, 2010).** The PDFs sit behind a bot check, which was not bypassed. They are the main published reading of the state of the Japanese hangars at Midway; their view reaches the rows only through Tully's combinedfleet.com pages, which cite no document.
- **Fuchida's own account** (USNI Proceedings, 1952, and his book): not obtainable; his USSBS interrogation says nothing of the flight decks or of dividing targets.
- **Japanese originals:** the Senshi Sosho, the carriers' detailed action reports (Shokaku's of 1944 is lost, Tully says; Taiho's committee report was burned, S-06-2 says), and any Japanese order on fuel or ordnance handling issued after Midway.
- **Hornet's own action report of 26 October, Enterprise's damage-control enclosures, the War Damage Reports of Princeton and Liscome Bay** (scans not read for want of time) **and of Bunker Hill** (not found).
- **CTF 58's reports of the Philippine Sea and Leyte, and Bates's Leyte volumes after 23 October.**

## A. How a strike was organised and its targets divided

| Navy | Date | Statement | Rows | Strength |
| --- | --- | --- | --- | --- |
| USN | March 1941 (USF-74, Revised) | Concentrate enough strength to sink or disable individual ships rather than disperse | ev 15300 | low, second-hand |
| USN | 1941 | All aircraft of a dive-bombing section on one target; "No. 1 target is the farthest away" | ev 15301 | low, second-hand |
| USN | 1941 | If higher authority has not, the group commander fixes the number of main targets, before take-off or before ordering the attack; each squadron "will normally divide planes equally among the main objectives assigned" | ev 15302 | low, second-hand |
| USN | 1941 | Group commanders "determine and direct the distribution of targets" to their squadrons, partly by what the groups ahead have attacked; silent on two targets equally far (Rigby) | ev 15303, 15304 | low, second-hand |
| USN | 1941 | Once the enemy is found all else is secondary to bombing and torpedo attack; destroying carriers controls the air | ev 15305 | medium, secondary |
| USN | 1942 practice | The whole deck-parked strike launched at once; at Midway differing types and ranges spoiled a coordinated attack | ev 15306, 15307 | medium, secondary |
| USN | Coral Sea, 7 May | A combined attack group of both carriers launched; both groups attacked the one carrier (Shoho), each thinking it attacked alone; Lexington's scouts, then her bombers and torpedo planes together | ev 15320, 15321; on file 2494 | high |
| USN | Coral Sea, 8 May | Yorktown's bombers circled to wait for the torpedo planes, then attacked together; both groups went at Shokaku and none at Zuikaku, which was in a squall | ev 15322; on file 2509 to 2511 | high |
| USN | Coral Sea lessons | Dive bombing to start just before and go on through the torpedo attack; the success of the Coral Sea owed much to it | ev 15310, 15311, 15323 | high (Cominch); medium (Bates) |
| USN | Midway, 4 June | The groups went independently, fighters not with the torpedo planes; Hornet's group turned south and found nothing; Enterprise's commander turned north, which CINCPAC calls decisive; the squadrons of two carriers fell on three carriers at about the same time, "Many hits were made on each carrier" and some pilots, thinking them destroyed, attacked other ships | ev 15314 to 15318 | high |
| USN | Midway lessons | The coordination of the Coral Sea "was missing" at Midway; concentrate attacks; small groups are ineffective; the Midway-based groups did not join | ev 15312, 15313, 15319, 15330 | high |
| USN | Eastern Solomons, 24 August | Saratoga's leader sent seven dive bombers and two torpedo planes at the cruiser and the rest at the carrier (on file 2241); Enterprise's torpedo planes could not join the bombers and found nothing | ev 15324 | high |
| USN | Santa Cruz, 26 October | Squadrons that found no carrier attacked a cruiser and a battleship on their own decision; Hornet's second-wave leader called his group commander, got no answer and attacked a cruiser; "complete coordination" asked for | ev 15325 to 15327; on file 2601 to 2606 | high |
| USN | 1948 (Bates, on Midway) | Attacking groups should be coordinated, not piecemeal; "a Strike Leader or Target Coordinator" should be named where groups of more than one carrier or base go at one objective, "if not provided by doctrine" | ev 15328, 15329 | medium |
| USN | July 1945 (USF 10B) | Doctrine for a carrier air group's attack is in the air type publications; the officer in tactical command orders the attack and names the objective; a deliberate attack is planned in detail beforehand | ev 15308, 15309 | high |
| IJN | 23 Nov 1941 (Operations Order No. 3, reconstructed) | One leader over groups by type; targets given by attack unit in the written plan, not over the target; first wave battleships then carriers, second wave carriers first; order of attack by type depends on surprise; more torpedo bombers against a fleet at sea | ev 15331 to 15336 | medium; reconstructed from memory (ev 15337) |
| IJN | 7 Dec 1941 | Watanabe: "The first aim was at aircraft carriers" (disagrees with the order); Zuikaku's new group kept to land targets | ev 15338, 15339 | medium; low |
| IJN | Coral Sea, 7 May | A mass strike in flight was not diverted; Takahashi chose on the spot to bomb the oiler and sent his torpedo planes home | ev 15340, 15341 | low |
| IJN | Coral Sea, 8 May | Shokaku's aircraft went for Lexington and Zuikaku's for Yorktown (Tully); Astoria's captain counted 14 torpedo planes and 12 dive bombers at Yorktown, 15 and 12 at Lexington, 1 and 2 at Astoria; torpedo planes first from two sides with the dive bombers at once, both carriers together | ev 15342 to 15345 | low (Tully); medium (ONI) |
| IJN | Midway, 4 June | Hiryu's division commander sent each strike at the one carrier reported; a shadower led the first by radio; the leader ordered the attack formation and then "Entire force, attack"; the Japanese believed the second strike had hit a second carrier | ev 15346 to 15348; out 15300 | medium to high |
| IJN | Eastern Solomons, 24 August | (US view) all the planes went for the nearer carrier group; the carrier the prime target, several dive bombers on North Carolina; no torpedo planes | ev 15349 to 15351 | medium |
| IJN | Santa Cruz, 26 October | Hornet attacked by dive bombers from one quarter and torpedo planes from the other almost together; one torpedo plane took Pensacola because she lay in its path; the torpedo planes split to both sides of Enterprise; when a squall hid Enterprise the attack spread over the formation, hitting South Dakota and San Juan | ev 15352 to 15356 | medium (ONI's reading) |
| IJN | 19 June 1944 | Ozawa: flight commanders were "only to attack the carriers"; the squadrons formed up separately and "every squadron was to take its individual target" | ev 15359, 15360 | high |
| IJN | Feb 1945 (ONI compilation of captured documents, undated within) | Carriers first; attacks "will not be over-concentrated against a single target", but assigned so that no carrier can launch or land; then transports, then escorts; dive bombers stop flight operations and torpedo or level bombers destroy; bombing first in a coordinated attack; the escort attacked so that the torpedo planes can sink the carrier; a single pilot attacks the first of several ships; on file: half to a third of the attack on the carrier (2264) | ev 15364 to 15371; on file 2260 to 2265 | low |

Escorts against carriers: the Japanese at the Coral Sea put about 3 of 56 counted aircraft on an escort (ev 15343); at Santa Cruz escorts were attacked when the carrier was hidden or in the way (ev 15353, 15355, 15356). The Americans attacked escorts when they found no carrier (ev 15318, 15325, 15326) or by the leader's choice (on file 2241). No row of either navy sets a share for the escort as doctrine before the 1945 compilation.

## B. Damage control and firefighting

| Navy | Date | Statement | Rows | Strength |
| --- | --- | --- | --- | --- |
| USN | May 1942 | Yorktown drained her gasoline system above the tanks on an alert before the attack of 8 May; found no way to test the CO2 pressure of the lines and made one; her damage control "in a high state of readiness" | ev 15383; tr 15306; out 15371 | high; the log's date uncertain |
| USN | 8 May 1942 | Lexington: lines above the armoured deck drained, the gas control room flooded and sealed with CO2; refuelling resumed about 1230; the explosion of 1247 was gasoline vapour, set off by electrical machinery; fire fighting then hampered by smoke, falling fire-main pressure (80 to 100 lb in the engine room, about 30 at the flight deck), no fog nozzles; the hangar water curtain worked until pressure failed | out 15362 to 15370, 15389 | high to medium |
| USN | 4 June 1942 | Yorktown: fuel lines drained and filled with CO2 at 20 lb, the tank spaces already full of CO2, the auxiliary tank dropped; three aircraft set alight in the hangar put out by sprinklers and water curtains; a rag-store fire beside the tanks did not reach them; after the torpedoes no power for damage control | out 15372 to 15376; out 15339 to 15346 (C) | high to medium |
| USN | 15 Sep 1942 | Wasp: hit while rearming and refuelling with much of the gasoline system live; hangar fire plugs at low pressure from an unisolated break in the forward loop; no fog nozzles; repair-party headquarters lost; BuShips: she would have survived the hull damage "had there been no fires" | out 15377 to 15382, 15388 | high to medium |
| USN | 26 Oct 1942 | Hornet: gasoline system "completely blanketed with CO2" before the attack; fires under control in an hour, out in two; upholstery and clothing fed them, the removal of flammables "not complete"; BuShips: damage control much improved since Yorktown | out 15383 to 15387 | high |
| USN | 1942 | Carriers' allowance six portable gasoline pumps; emergency lighting fitted; Lexington's engineer found the drainage pump poor and doubted a single fire main | tr 15304, 15305, 15308, 15309 | high to medium |
| USN | 1944 to 1945 | Franklin: hangar fire of 30 October 1944 confined by sprinklers and curtains and out in about two hours; the conflagration of 19 March 1945 the worst any US warship survived, the armoured hangar deck keeping it from the machinery; Bunker Hill's centre burned about four hours; Randolph's hangar aircraft had been defuelled | out 15390 to 15398 | high; Randolph low |
| IJN | as found in 1945 (S-84, Feb 1946) | Organisation, training and equipment "were inferior" to American and British, improved in the war, but "good results never were obtained"; yet: three schools, a chief damage control officer on each ship, emergency groups of 16 to 24, the whole crew drilled, firefighting taught on four old ships at Yokosuka, sectionable fire mains, portable pumps, foam and CO2 | tr 15326 to 15332 | medium; American, from six officers' interviews, no ship cited |
| IJN | Jan 1946 (A-11) | Closed hangars (open ones tried and disliked); divided by roller fire curtains sprayed from the fire main; a fixed foam system the main means against aircraft and gasoline fires; CO2 blanketing in the lower hangars only; ventilation by exhaust fans in the newer ships; gasoline tanks in two groups surrounded before the war by a CO2 space | ev 15395 to 15397, 15400, 15401, 15403 to 15406 | medium |
| IJN | 4 June 1942 | Akagi turned on her CO2 apparatus three minutes after the first induced explosions; her pumps failed (on file 9301); Kaga's fire fought under the air officer, the bridge party being dead | out 15427, 15428 | high |
| IJN | 1942, other carriers | Shokaku at Santa Cruz: hangars nearly empty, "no aviation fuel was active", fires out in just under five hours; Ryujo's fire was put out and flooding sank her | out 15425, 15329 | low |
| IJN | 1944 | Taiho: one torpedo cracked the forward gasoline tank; fumes filled both hangars; damage control opened every door and ran every fan, spreading them; no foam over the elevator pit; a vapour explosion hours later; the Navy's own committee found her crew "guilty of poor damage control practice" on three counts; the tanks "could scarcely have been located in a worse position". Shokaku: power failed at once, the hangar mains could not be reached, bucket brigades; her damage control "among the most experienced in Kido Butai". Hiyo: an induced explosion after an aircraft torpedo. American verdict on the four big ships: good in Musashi, mediocre in Yamato, poor in Taiho and Shinano; Musashi showed little fire though full of wood | out 15403 to 15424, 15429; tr 15333 to 15335 | medium (NavTechJap); low (combinedfleet) |
| RN | Jan and May 1941 | Illustrious's hangar fire curtains "torn to bits" by a bomb through her armoured deck; British notes: steel curtains to be replaced by fabric, secondary positions for hangar spraying; Furious's curtains destroyed | ev 15388 to 15391 | low to high; fragments of an interleaved table |

## C. Where aircraft were armed and fuelled, and what was aboard when hit

General practice:

- **US:** most aircraft parked on the flight deck, the hangar for repair (Wildenberg, ev 15379); Yorktown fuelled her fighters on deck at Midway (out 15339, 15343, 15346); a US officer in 1933 saw a carrier caught with aircraft on deck as the target that decides an action (ev 15381); ONI after the Coral Sea: the large quantity of gasoline is a menace to carriers (ev 15377). No row was found that states open hangars with roller curtains as a US practice against the Japanese closed hangar (a gap).
- **Japanese:** the hangar was the main stowage and aircraft were prepared there (Wildenberg, ev 15378); closed hangars (A-11, ev 15396); bombs and torpedoes "normally" loaded on the flight deck, "occasionally in the hangars", with separate bomb and torpedo rooms and a hoist aft (A-11, ev 15398, 15399).

By carrier:

| Carrier | When hit | Aircraft and fuel | Rows | Disagreement |
| --- | --- | --- | --- | --- |
| Akagi | 4 June | preparing the second attack; about 40 aboard, loaded with bombs inside the hangar (Aoki); armed and fuelled torpedo planes in the hangar, gasoline from ruptured lines (Tully); torpedoes stored in the hangars exploded | out 15301 to 15303, 15309, 15310, 15321, 15322, 15399; on file 9300, 9301 | two hits (Nagumo, on file 9300) against one effective hit (Tully, out 15323) |
| Kaga | 4 June | about 30 in the hangar loaded and fuelled, the rest on deck, six fighters aloft (Amagai); "ready for launching and were gassed and armed" (S-06-3); torpedo planes armed and fuelled in the hangars, with "unstored ordnance" (Tully, alone); the fire reached both fuel tanks | out 15304, 15306 to 15308, 15324, 15400 | Amagai puts some on deck; Tully none |
| Soryu | 4 June | the Midway returnees being refuelled and rearmed on the lower hangar deck, where the first bomb burst (Ohara); "aboard, armed and gassed" (S-06-3); bomb, torpedo and gasoline explosions by 0730 (on file 9312) | out 15311 to 15313, 15325, 15326, 15401 | none |
| Hiryu | 4 June, afternoon | about 20 aboard, the rest away (Kawaguchi); one aircraft about to take off (Nagumo); "a majority of her planes were not aboard" yet the fire spread and grew (S-06-3) | out 15305, 15314, 15315, 15402 | about 20 (Kawaguchi) against Nagumo's 15 operational |
| Shoho | 7 May | very few aboard (USSBS); launching her patrol fighters (Tully) | out 15316, 15317 | none |
| Shokaku | 8 May | strike away; the first bomb set the gasoline afire forward; could land but not launch; saved | out 15318, 15319 | none |
| Shokaku | 26 October | all away but about 16 fighters of the patrol (USSBS, on file 2185); hangars nearly empty, no fuel active (Tully); saved | out 15320, 15425, 15328 | hits: six (USSBS, out 15328) against three or four elsewhere on file |
| Zuiho | 26 October | strike already aloft; two bombs aft, flight operations stopped; no fire stated | out 15327; on file 1450 | none |
| Ryujo | 24 August | four aircraft went down with her; fire put out, flooding sank her; aircraft aboard when hit not stated | out 15329, 15330 | her captain claimed no bomb hits |
| Lexington | 8 May | tanks full, the voids round them filled with water; lines drained during the action; refuelling resumed about 1230; one aircraft in the forward hangar when the later fire began; 27 torpedoes on the hangar mezzanine; 35 aircraft on deck went down | out 15331 to 15337, 15368, 15369 | when the gas room was flooded (BuShips after inspection; the captain about 1200) |
| Yorktown | 8 May | serviceable aircraft rearmed and refuelled at once on return; small fires in the aviation store and hangar overhead, put out | out 15338, 15347 | none |
| Yorktown | 4 June | fuelling fighters on deck, scouts in the landing circle; lines drained and filled with CO2; aircraft in the hangar, some with torpedoes, set alight and put out by sprinklers | out 15339 to 15346, 15372 | none |
| Enterprise | 24 August | last aircraft launched three minutes before; gasoline mains aft ruptured; fire under control | out 15354 to 15356; on file 181 to 184 | none |
| Enterprise | 26 October | bombs being loaded on aircraft forward in the hangar (one reading of an explosion); aircraft in the forward deck park bounced clear | out 15357, 15360 | her report gives two readings of the hangar explosion |
| Hornet | 26 October | no planes on deck; gasoline system blanketed with CO2 | out 15358, 15359, 15383 | none |
| Wasp | 15 September | rearming and refuelling; 16 fuelled and armed aircraft forward in the hangar, four with a depth bomb; torpedo planes armed with CO2 in their tanks; the gasoline system partly live | out 15348 to 15353 | none |
| Saratoga | 31 August | torpedo; electrical explosions lost power; nothing on aircraft | out 15361 | a gap |

## D. Fires in other ships, 1942

| Ship | Hits | Lost or saved | Fire or flooding | The source's verdict | Rows |
| --- | --- | --- | --- | --- | --- |
| Astoria | 65 or more | lost about 10 hours after | fire, then a magazine blew a hole below the waterline | fires "the direct cause"; the structural damage alone survivable (BuShips) | out 15430 to 15432, 15435 |
| Quincy, Vincennes | many, and torpedoes | lost within the hour | both | loss inevitable; fires "a major contributing factor" | out 15433, 15434 |
| Savo, the three | | | paint, records, gear; no gasoline-tank fire | the fires lit the ships for the enemy | out 15436, 15437 |
| Canberra | about 24 | scuttled | fire gained headway, list 17 to 20 degrees | hopeless once the force had to leave | out 15438 to 15440 |
| Boise | 8 | saved | magazine fire, about 3,000 lb of powder | quick flooding of the source mattered more than venting | out 15441 to 15443 |
| Salt Lake City | 3 | saved | oil fire in a fireroom | none | out 15444 |
| Furutaka | about 90 | lost | fires, torpedoes alight, a torpedo hit | none | out 15445 (low) |
| San Francisco | about 45 | saved | 22 fires, all quickly controlled | intact fire mains and a working plant "probably the most effective" factor | out 15446, 15447 |
| Portland | a torpedo | saved | no fire | | out 15448 |
| Cushing, Monssen, Atlanta | many | lost | blew up from fires the next day; Atlanta set afire | none | out 15449 to 15451 |
| Laffey, Preston | | lost | Preston's fires set her torpedo warheads alight | none | out 15452, 15453 |
| Pensacola | a torpedo into fuel tanks | saved | oil fire over 100 feet high, about 12 hours | might well have forced abandonment; the first US cruiser to control a fire that size | out 15454, 15455; tr 15338 |
| Northampton | 2 torpedoes | lost | uncontrolled fires and flooding | none | out 15456 |
| Mikuma | 5 bombs | lost | fires amidships, her torpedoes exploded | none | out 15457, 15458 (low) |
| Mogami | 3 or more bombs | saved | fire in an empty torpedo room, out in an hour | none | out 15459 (low) |
| Jintsu | 1 bomb | saved | fire; forward magazines flooded | | out 15460 (low) |
| Chikuma | 3 bombs | saved | a bomb in the torpedo room after the torpedoes were jettisoned | | out 15461 (low) |
| Kinugasa | 1 bomb | lost | oil store afire about 3 hours, flooding, steering lost | | out 15462 (low) |

At the time: CINCPAC in October 1942 noted damaged ships putting fires out quickly and the removal of flammables (tr 15336); Washington had no serious fires on 14 to 15 November and gave the removal of paint and linoleum first among the reasons (tr 15337). The Japanese side rests on combinedfleet.com alone: no Japanese primary account of a cruiser's fire was found.

## E. Dated changes

| Navy | Date | Change | Rows |
| --- | --- | --- | --- |
| IJN | before the war, "early in the war" | CO2 spaces round the gasoline tanks; found inadequate early in the war and the top and side spaces filled with concrete (A-11, undated) | ev 15401, 15402 |
| USN | May 1942 or before | Yorktown drains her gasoline lines on an alert (Coral Sea); Lexington drains the lines above her armoured deck in action | ev 15383; out 15368 |
| USN | May to June 1942 | After Lexington: research on gasoline stowage; inert gas in the voids by the gasoline tanks (Saratoga); exhaust ventilation and no sparking machinery on suspicion of vapour; 15 asbestos suits, 60 breathing sets and a fog nozzle for each fire plug for every carrier; gas-tight electrical gear | tr 15300 to 15303, 15311 |
| USN | June 1942 | After Yorktown: a fixed fog-nozzle sprinkling system for all carriers; CINCPAC orders a salvage party in every ship | tr 15307, 15310 |
| USN | 4 June and 26 Oct 1942 | Lines drained and filled with CO2 before an attack (Yorktown, Hornet) | out 15340, 15372, 15383 |
| IJN | 26 Oct 1942 | Strikes launched carrier by carrier as each was ready, "bearing the lessons of Midway in mind"; the armed second wave brought up from the hangars; fuel carts rolled overboard (Tully) | ev 15357, 15358, 15375, 15376 (low) |
| USN | Oct to Nov 1942 | Removal of flammables credited for small fires | tr 15336, 15337 |
| USN | 22 Jan, 25 Feb 1943 | Bureau of Ships letters on firefighting sent to all ships (the 22 January letter known only by its file number) | ev 15384 |
| USN | 1943 | The Firefighting Manual; firefighting schools "well established" at the major yards and bases; gasoline-handling schools approved; inert gas in the voids round gasoline tanks; from the Essex class the mains blown empty with inert gas; diesel fire pumps and defuelling connections for all carriers | ev 15385; tr 15312 to 15314, 15316, 15317 |
| USN | March 1944 | Essex class: the port ventilation trunk sectionalised after smoke spread through it | ev 15387 |
| IJN | June to July 1944 | After Taiho: gasoline capacity "drastically reduced"; three feet of concrete round all gasoline tanks; supply fans and wind scoops for the hangars; an inquest on Taiho, Shokaku and Hiyo; suggestions to halve the fuel load | ev 15392 to 15394, 15407, 15408 |
| IJN | 19 June 1944 | Squadrons each take their own target; carriers only; much of a light-carrier strike fighters | ev 15359 to 15363 |
| USN | 1944 to 1945 | Most carriers defuel aircraft on return to the hangar; hangar fires controlled in one to four hours with every system working; foam for small fires only; fixed fog systems never controlled a major fire; schools and FTP-170-B "proved their value"; breathing-set allowance raised | tr 15318 to 15325 |
| USN | 1945 | Removal of excess fixed fog nozzles authorised (7 August); air attack doctrine in the air type publications (USF 10B, July) | ev 15386, 15308 |
| USN | 1948, looking back | A target coordinator recommended "if not provided by doctrine" | ev 15329 |

No row dates the US task group target coordinator, USF-74B's rule on spreading bombers over carriers, or a Japanese order on fuel and ordnance handling between June 1942 and June 1944.

## Where sources disagree

- **USF-74 of 1941 on dividing a strike** (all three second-hand, through Rigby): the farthest target first (ev 15301); divide planes equally among the main targets (ev 15302); the group commander directs, by what the groups ahead have attacked (ev 15303).
- **Pearl Harbor's first aim:** battleships, then carriers (the reconstructed order, ev 15331) against carriers first (Watanabe, ev 15338).
- **Akagi's hits:** two (Nagumo, on file 9300) against one effective and a near miss (Tully, out 15323).
- **Kaga's aircraft:** some on deck (Amagai, out 15306) against all in the hangars with ordnance unstowed (Tully, out 15324).
- **Hiryu's aircraft aboard:** about 20 (Kawaguchi, out 15314) against 15 operational (Nagumo, out 15305).
- **Where the Japanese armed their aircraft:** normally on the flight deck (A-11, ev 15398) against the Midway accounts of rearming in the hangars (out 15309, 15311, 15306; Wildenberg ev 15380). A-11 states the usual practice and the Midway rows a rearming under attack; they need not conflict, but no row says how often aircraft were armed below.
- **Concrete round the Japanese gasoline tanks:** early in the war (A-11, ev 15402) against after Taiho in 1944 (S-06-2, ev 15393; combinedfleet, ev 15408).
- **Taiho's explosion:** 1530 (S-06-2, out 15410) against 1432 (combinedfleet, out 15422); the torpedo to starboard or port.
- **Shokaku in 1944:** an enormous explosion (S-06-3) against Tully's doubt of it (out 15417).
- **Zuiho on 25 October 1944:** a large hangar fire (S-06-3, out 15406) against a small one (combinedfleet, out 15426).
- **Shokaku's hits at Santa Cruz:** six (USSBS, out 15328) against three or four elsewhere.
- **Hiryu's second strike:** the Japanese believed it hit a second carrier (out 15300); it hit Yorktown again.

## On the claim that Japanese damage control was simply poor

The rows do not bear it out as a plain statement.

- **The word "inferior" is American and post-war.** It comes from S-84 (February 1946), which says it rests on interviews with six officers of the Navy Technical Department and the Japanese outline of the organisation, cites no ship and calls itself a negative report (tr 15326). The 1948 journal's extract already on file (tr 29) is the same report.
- **The same report describes a real system**: schools, a damage control officer in every ship, the whole crew drilled, firefighting taught on old ships, foam, CO2 and portable pumps (tr 15327 to 15332). Damage control was not left to a specialist cadre alone (tr 15329), against the H-gram on file in `docs/research/oob/damage-control.md`.
- **Where it is judged by results, the results are mixed:** good in Musashi, poor in Taiho and Shinano (tr 15333); Shokaku's crew saved her twice and was "among the most experienced" (out 15418); Taiho's crew was condemned by her own navy's committee (out 15411).
- **Much of what lost Japanese carriers was design and situation:** the enclosed hangar that held vapour (out 15408, 15409, 15421), a tank placed where "could scarcely have been ... worse" (out 15413), power and mains lost to torpedoes (out 15415), and in 1942 aircraft armed and fuelled below when the bombs came (C). Japanese carriers bombed with their hangars nearly empty were saved (Shokaku twice, Zuiho).
- **There is no Japanese primary account** of damage control aboard any carrier of 1942 beyond Nagumo's few lines (out 15427, 15428; on file 9301). The case against Japanese crews in 1942 cannot be told apart, on these rows, from the case against Japanese aircraft handling and design.

## Gaps

The last 49 entries of `gaps.csv`. The ones that matter most to the three rules:

- the texts of USF-74, USF-74B, USF-75, USF-77 and PAC-10, and the date and origin of the target coordinator;
- a primary statement of how McClusky divided his squadrons between Kaga and Akagi;
- any Japanese statement of how Takahashi's strike of 8 May, Seki's of 24 August and the Santa Cruz strikes were divided, and why Hiryu's second strike took Yorktown for another carrier;
- "Doctrine Matters" and "A Tale of Three Whoppers" (the state of the Japanese hangars at Midway), and Fuchida's own account;
- any Japanese order on fuel or ordnance handling between Midway and the Philippine Sea;
- who ordered Yorktown's lines drained and filled with CO2, and whether it was a standing order; the date inert gas became required; the issue date of the fog nozzle; the opening dates of the firefighting schools;
- Ryujo's and Zuiho's aircraft and fuel when hit in 1942; Saratoga's when torpedoed;
- the Royal Navy's fuel stowage and hangar firefighting.

## The reader's arithmetic, not fact

This section is the lead's reading of the rows above for the architect. It is not a source's statement and is not to be filed as a row. Counts are of the cases on file and are small.

### Rule 1: how a strike's groups are divided among the ships they find

- **USN, 1941 to 1942.** The doctrine, known only second-hand, leans to concentration on individual ships, with the group commander distributing targets among his squadrons (ev 15300, 15302, 15303). In practice each carrier's group attacked as a unit and joined no other group. Where two groups found two carriers in 1942 (the Coral Sea on 8 May), both attacked one; where three squadrons of two carriers found three carriers (Midway), they took all three, by the leaders' choice and the order of arrival; where one group found a carrier and a cruiser (the Eastern Solomons), its leader sent nine aircraft at the cruiser and then called them onto the carrier. Groups that found no carrier attacked escorts (Santa Cruz). A rule that a group takes one ship and the groups of one pulse spread over the ships of the kind they want fits Midway and not the Coral Sea; the rows do not give a figure between the two.
- **IJN, 1941 to 1942.** The Japanese divided by plan or by carrier unit: at Pearl Harbor each attack unit had its target in the written order; at the Coral Sea on 8 May each carrier's aircraft took one American carrier, about evenly (26 and 27 aircraft), with about 1 in 20 on an escort. Where only one carrier was found (Midway, the Eastern Solomons) all went at it, with a few dive bombers on a battleship. At Santa Cruz the attack spread over the escorts when the carrier was hidden.
- **What the rows support as a difference in 1942:** the Japanese more ready to divide a strike evenly among the carriers found, by carrier unit (one case, the Coral Sea); the Americans, by group, apt to fall together on one (one case, the Coral Sea) and to spread only as leaders and arrival times happened to divide them (one case, Midway). That is two or three cases a side, enough to say which way a difference runs and not enough to set a proportion.
- **After 1942.** In 1944 the Japanese order was carriers only, each squadron its own target (ev 15360, high); the 1945 compilation says do not over-concentrate and aim to stop every carrier's flight deck, with a half to a third of a dive-bomber attack on the carrier and the rest on the screen (ev 15364, on file 2264). For the Americans the rows hold only Bates's recommendation of a target coordinator in 1948, "if not provided by doctrine", and USF 10B of 1945; the 1944 rule against putting all bombers on one carrier is a lead (Herder) and not a row. So a change toward spreading by date is supported for the Japanese (1944) by a row, and for the Americans only by leads.
- **Escorts:** no row supports a share of a strike sent at escorts while a carrier is seen, for either navy in 1942, beyond the Japanese 1 in 20 at the Coral Sea and the 1945 compilation's doctrine. Decision 0211's rule that a group does not go at escorts while a ship of the wanted kind is afloat is close to what the 1942 rows show.

### Rule 2: fire that grows and destroys a ship

- **It should reach ships other than carriers.** Of the US cruisers and destroyers of 1942 with major fires on file, Astoria was lost to fire by BuShips's verdict; Cushing, Monssen and Northampton were lost with fire named first; in Quincy and Vincennes fire contributed; San Francisco, Boise, Salt Lake City and Pensacola were saved. That is roughly four lost with fire as the stated cause and four saved, close to the even chance in decision 0212's table; the Japanese cases (Mikuma, Furutaka, Kinugasa lost; Mogami, Jintsu, Chikuma saved) run the same way but rest on combinedfleet.com.
- **What decided a fire in the American reports** was whether the fire mains and power survived (San Francisco against Quincy and Wasp), what fed the fire (paint, gear, ready ammunition, aircraft gasoline), and whether a magazine could be flooded (Boise). The rows give times: carriers' hangar fires one to four hours with every system working (1944 to 1945), Hornet's out in two hours, Astoria about 10 hours to her loss, Pensacola 12 hours to save.
- **A difference by navy in fighting a fire, 1942:** the rows do not support a separate figure for Japanese crews apart from the state of their ships. They do support that by mid-1942 American carriers drained their gasoline lines and filled them with CO2 before an attack (Yorktown in May and June, Hornet in October) and had hangar sprinklers and water curtains that worked, and that no Japanese row shows the lines purged before an attack.
- **A difference by date:** the American rows support fire being fought better from 1943 (inert gas, fog nozzles, schools, the manual, diesel pumps) and markedly by 1944 to 1945, when Franklin and Bunker Hill survived fires that burned for hours; they give no proportion. For the Japanese the rows give a change only in mid-1944 (less gasoline, concrete, fans), and no row says it helped.

### Rule 3: a bomb that sets fire among a carrier's aircraft and fuel

- **The state of the aircraft explains more than the navy.** Japanese carriers bombed with aircraft armed and fuelled aboard (Akagi, Kaga, Soryu): three of three lost to fire, after one to four hits. Japanese carriers bombed with their strike away (Shokaku twice, Zuiho, Ryujo): none lost to fire. American carriers bombed in 1942 (Yorktown twice, Enterprise twice, Hornet, and Lexington's bombs): none lost to fire set by a bomb: Yorktown and Hornet had purged their lines, and the fire of Enterprise's ruptured mains on 24 August was brought under control. The one American carrier hit with fuelled aircraft in the hangar and a live gasoline system, Wasp, was lost to fire, after torpedoes.
- **The exception is Hiryu,** bombed with most of her aircraft away and lost to fire all the same (out 15402, 15314). Decision 0212's share squared would give her a small chance; one case in four of the Japanese losses at Midway does not fit it.
- **A difference by navy that the rows support:** where aircraft are fuelled, the American practice of the time kept them on the flight deck and the lines purged before an attack, and the Japanese kept them in closed hangars, where vapour and induced explosions followed (Midway, Taiho). If the rule's chance is to differ by navy in 1942, the rows would support a higher chance for a Japanese carrier with aircraft aboard than for an American one with the same share, because of where they were and whether the lines were live. They give no figure for how much higher.
- **By date:** the American rows support a falling chance from 1943 (defuelling on return, inert gas, the mains blown empty from the Essex class); the Japanese rows support a change only from mid-1944, when gasoline was cut and tanks set in concrete.
