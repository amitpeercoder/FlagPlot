# Outside answers of 13 November 2026, checked against their sources

On 13 November 2026 the designer put eleven research prompts to an outside model (ChatGPT, through Codex). Its answers are tables of claims, each with a source, a page and a quoted sentence. An outside model's answer is a lead, not a source. A row was keyed here only where one of our readers opened the cited document, found the statement in it, and keyed the row from the document's own words. The answer files stay outside the repository.

The reading was done by Sonnet agents, one for each answer file. Each fetched the cited document as raw text (HTML, or PDF through pdftotext; two scanned USSBS interrogations were read by OCR and the key pages by eye), searched it for the quoted words, and read the passage around them. Every staged row's quote was checked by program as a span of the saved text: 177 of 177 passed. 176 were keyed; one was withdrawn, as the Q1.8 section says. The lead re-read the disputed cases (the Cape Esperance table quote, the misdated Saratoga strike, the Gunnery Bulletin figures) in the saved text. This page says what the rows hold, what the answers got wrong and what is still missing. It settles nothing.

## How far the answers can be trusted

A claim is one row of an answer's table. An "absence" claim says a source does not give something; it yields no row, but its cited quote was checked like any other. "Verified" counts claims whose source says what the answer says, including those already in the database from the same source. A claim that was "misattributed" is counted again in the last column whatever its other verdict: a right figure from a different book, chapter, ship or date, or a wrong author.

| File | Claims | Verified | Of which already held | Source says otherwise | Not reachable | Not a source | Invented or misattributed |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 01 Ship fits | 20 | 12 | 6 | 5 | 2 | 1 | 4 |
| 02 Savo Island | 12 | 12 | 2 | 0 | 0 | 0 | 0 |
| 03 Night gunnery | 19 | 19 | 3 | 0 | 0 | 0 | 1 |
| 04 Eastern Solomons | 30 | 27 | 14 | 2 | 1 | 0 | 3 |
| 05 Guadalcanal air | 22 | 22 | 7 | 0 | 0 | 0 | 2 |
| 06 Pilot training | 11 | 6 | 0 | 1 | 4 | 0 | 2 |
| 07 Cape Esperance | 22 | 19 | 12 | 1 | 2 | 0 | 3, and 2 that could not be traced |
| 08 Air effectiveness | 12 | 12 | 0 | 0 | 0 | 0 | 0 |
| 09 Air upkeep | 8 | 7 | 2 | 0 | 0 | 0 | 1 |
| 12 Night action duration | 73 | 73 | 51 | 0 | 0 | 0 | 0 (page numbers one out) |
| 13 Fighter defence | 59 | 59 | 14 | 0 | 0 | 0 | 0 (page numbers one out) |
| **All** | **288** | **268** | **111** | **9** | **9** | **1** | **16 (6%)** |

Files 12 and 13 came later than the first nine and are much cleaner. Each opened official narratives directly and quoted them closely, and every claim held. Their one weakness is page numbers, which are sometimes a page out. Of their 132 claims, 65 were already rows, so the new matter is mostly what the readers found next to the quotes.

What this says about the channel:

- **No quoted sentence was made up**, as far as the sources could be opened. Every quote that could be checked is in a real document, though some are elided or reworded.
- **About one claim in ten is misattributed:** the right words given to the wrong book, chapter, ship, date or author. Examples: a 1967 cruise book of the later Laffey (DD-724) cited for DD-459; a ShipCraft volume cited as Lacroix and Wells; Saratoga's strike of 23 August given as the strike of 24 August; Miller's Guadalcanal credited to "Samuel Milner"; Mersky's Time of the Aces credited to Sherrod.
- **Two figures were the model's own arithmetic, presented as the source's:** Helena's "24 minutes" of fire at Cape Esperance, and a "from 24 August" date for the SBD searches.
- **Two sentences could not be traced at all:** the Japanese turn at Cape Esperance, on a document host (device.report) that refuses every reader. They are unproven, not proven false.
- **Its "not found" answers are the weakest part.** In five places the source the model cited holds what it said was not there:
  - the Savo cruisers' hit counts;
  - Tone's and Fubuki's tubes in FM 30-58;
  - Enterprise's rounds by calibre;
  - the standard practice of sending the P-400s and SBDs off on warning;
  - Yorktown's rounds per gun at Coral Sea.
- **Its searches were shallow.** File 02 opened one document; the other four timed out. File 04 never opened Enterprise's own reports, which answer Q4.3.
- **What it adds is small.** Of the first nine files, 46 of the 136 verified claims were already in the database from the same source. The new rows below come mostly from passages our readers found next to the cited quote, not from the answers themselves.

Used this way, as a list of documents worth opening, the channel is useful. Used as a source, it would have entered about one wrong attribution in ten.

## What the rows hold

| File | Ids | Rows | What |
| --- | --- | --- | --- |
| `sources.csv` | 1223 to 1227, 1229 to 1231 | 8 | New sources: Bombing Squadron Six's Midway report; CINCPAC Gunnery Bulletin 2-42; FM 30-58; ShipCraft 5; Benham's Eastern Solomons report; the War Damage Reports for San Francisco, South Dakota and Boise |
| `engagement_events.csv` | 1241 to 1279 | 39 | Opening of fire at the Eastern Solomons; Henderson Field's scramble routine; Salt Lake City's firing at Cape Esperance; Savo and 15 November times |
| `engagement_outcomes.csv` | 3942 to 3971 | 30 | Rounds per gun at Coral Sea; Midway bomb hits; hits received by San Francisco, South Dakota, Boise and Salt Lake City; aircraft at Henderson Field |
| `ship_class_facts.csv` | 1375 to 1381 | 7 | Japanese torpedo tubes by class, from a wartime US Army recognition manual, and Myoko as designed |
| `fits.csv` | 758 | 1 | Benham's 16 tubes |
| `training_facts.csv` | 98 to 105 | 8 | Japanese carrier pilots' hours and who flew in August 1942, from two USSBS interrogations |
| `sources.csv`, `engagement_outcomes.csv` | source 1232; outcomes 3972 to 3978 | 8 | Air upkeep at Guadalcanal and Rabaul (file 09) |
| `sources.csv`, `engagement_events.csv`, `engagement_outcomes.csv`, `aircraft_type_facts.csv` | source 1233; events 1280 to 1297; outcomes 3979 to 4013; aircraft type facts 556 | 55 | Fighter defence (file 13) |
| `engagement_events.csv`, `engagement_outcomes.csv`, `engagement_positions.csv` | events 1298 to 1300; outcomes 4014 to 4033; positions 542 to 548 | 30 | Night action duration (file 12) |
| `gaps.csv` | last three lines | 3 | The patrol that made contact; torpedo reload times; Mikawa's own contemporary reason |
| `gaps.csv` | the line before them | 1 | Warning minutes and fighters in contact, by raid over Henderson Field |

Every row's note ends with the words "Lead from an outside model's answer (Nov 13, 2026), checked against the source", so the rows of this pass can be found.

## Savo Island (file 02, and file 03 for the cruisers)

**Q2.3, Q2.4, torpedoes by ship, side and target; torpedo hits and by whom.**

- **The answer adds nothing.** The outside model opened only War Damage Report 29. That report covers Astoria, Quincy and Vincennes, and has no Japanese launches and no attackers. Its two figures are already rows: Quincy 2 torpedo hits (outcomes 72) and Vincennes 1, possibly 2 (61).
- **One thing the answer missed:** the report guesses that Quincy's two hits were "probably from a submarine" (Astoria narrative, para 17). Not keyed: it is the report's guess.
- **What the database already holds** (Bates unless named; low, OCR):

| Ship | Torpedoes fired, with time, side and target | Rows |
| --- | --- | --- |
| Chokai | 0138, four port, at Canberra and Chicago; 0148, four starboard, at Vincennes. Her own report agrees: 2337 port, 2347 starboard, Tokyo time | events 55, 1097, 100, 1098, 1130, 1131 |
| Kako | Before 0146, three starboard, at Chicago; 0201, four port, at Astoria; 0205, two more. Her total is 6 in one row and 10 expended in two others | events 1100 to 1102; outcomes 130, 131, 3501 |
| Furutaka | About 0143, four port; 0146, one starboard, at Patterson; 0149, three starboard, at Canberra | events 58, 73, 1105 |
| Kinugasa | 0153, four port, at Canberra; another passage says two | events 101, 1103, 1104 |
| Aoba | Three starboard about 0145; one port at a garbled time | outcomes 124, 3181; event 1099 |
| Tenryu | 0202, six starboard, at Quincy | event 1106 |
| Yubari | Four, by Bates's inference only | outcome 155 |
| Yunagi | Nothing | — |

- **Reloads:** no row for any ship; the gaps are listed.
- **Hits by firer,** all Bates's attributions: Tenryu 2 on Quincy (outcome 3203); Aoba 1 on Quincy (3184); Chokai 2 or 3 on Vincennes (3178); Yubari 1 on Vincennes (3204); Kako 2 on Chicago (event 70). Canberra has 0, 1 or 2 hits, all disputed, with no firer named by any source.

**New Savo rows, from War Damage Report 29** (wdr-29). The model cited this report but did not read these passages:

- **Firing and readiness (events).** Enemy fire ceased at 0215 (1241). The three northern cruisers' 8-inch fire is given at 0155 (1260, 1262, 1264); for Vincennes the report says the battery was directed to fire then, not that it fired. Readiness at 0130 was "Readiness II", material condition Zed (1261, 1263, 1265).
- **Damage effect (outcomes).** For Vincennes, the worst gunfire damage came from the first few 8-inch salvos (3966). Quincy's and Vincennes' losses were inevitable (3967, 3968). Astoria's structural damage was not enough to sink her; uncontrolled fires did (3969).
- **When the enemy began hitting.** "In the case of all three ships the enemy commenced hitting on the fourth or fifth salvo" (outcome 3970).
- **Hit counts (Q3.1).** The answer said exact hit counts are not known. The same sentence of the report gives the known counts: Astoria at least 65, Quincy at least 36, Vincennes 57. These were already rows (82, 69, 58).

## Night gunnery elsewhere (file 03)

**San Francisco, 13 November.**

- About 45 separate hits, with the calibres the report gives in the note (outcome 3962, from the new source wdr-ca38-nov42). The report finds that most damage came from the 6-inch secondary battery (Q3.8).
- No torpedo hit her (3963).
- Her 0148 opening of fire was already held (events 833, 939).

**South Dakota, 15 November** (new source wdr-bb57-nov42).

- At least 26 hits, with the split in the note (outcome 3964).
- Her main battery opened at 0018, at about 15,700 yards, lost the target at 0024 and checked fire at 0033 (events 1257 to 1259). The 0018 differs by a minute from the 0017 already held from NavWeaps.

**Boise, 11 to 12 October.** 8 hits (outcome 3965, new source wdr-cl47-oct42). This agrees with the BuShips summary and differs from the Cominch bulletin's 6 (1587). The row is `disputed`.

**Q3.4, hits to sink a cruiser.** No source gives a number. The damage-effect rows above are what the reports say instead.

**Q3.7.** The quoted torpedo instruction is real. It is War Instructions 1944, para 12173, not "Night Action Instructions", and it is outside 1942. Not keyed.

## Eastern Solomons (file 04)

**Q4.3, guns against the dives.**

- **The model's answer.** It cited cv6.us, which puts the first fire at 1712.
- **What Enterprise's own reports say.** These are her action report (`cv6-action-reports`) and her gunnery officer's report (`cv6-gunnery-report-0824`), both already in the database.
  - One 20 mm gun, battery no. 4, "commenced firing immediately, although the range was about 4,000 yards" when the first dive bomber was seen coming down, at about 1709 (events 1242 and 1243). The two rows are not independent witnesses.
  - By the time the planes were within range, all guns in that sector were firing (1244).
  - The Task Force commander's comment: "The 5-inch fire was commenced only after that of ENTERPRISE small caliber weapons, and after the first attacking planes were in their dives" (1245). The directors never picked up the planes, which came from astern out of the sun.
  - The report puts the start of the dives at 1711.
- **Already held:** Enterprise's rounds by calibre (5-inch 97, 1.1-inch 1,500, 20 mm 12,280, .50 cal 880 or 885). The answer called these not found.

**Q4.5, the patrol.**

- No source gives how many fighters made contact.
- The action report says the fighters' attacks "had delayed effectiveness because of the interposition of enemy Zero fighters. As a result, many enemy dive bombers reached diving position before they could be intercepted" (outcome 3942).
- First contact, by one section at 1655, and the fighters airborne were already held (218, 219).
- cv6.us's "54 circling overhead" at 1632 is contradicted by the action report: 25 in the air at 1630. 54 is the two carriers' total strength (row 212). Not keyed.

**Q4.7, Saratoga's strike on Ryujo.**

- USSBS's "31 dive bombers and 6 torpedo planes" is the strike of 23 August that missed the convoy. It is already held correctly (138).
- combinedfleet's Ryujo page gives 30 SBD and 8 TBF launched at 1315 (event 1246, `disputed` against 188 and 190, which give 1435). It does not print the "21 SBD, 5 TBF, 1550" the answer gave it; those are on the Chikuma page and already held (196).
- The quote cited for a second Saratoga strike is about B-17 flights.

**Q4.9.** combinedfleet has Ryujo's strike met over Guadalcanal at 1215 by 10 F4F and later 2 P-400 (event 1247, `disputed` against 184 and 185).

**Q4.1, Q4.10.**

- The Okumiya transcript is a scan with no text layer and was not checked.
- The action report gives Enterprise's deck ready at 1816 and the first landing at 1819. Not keyed; a lead for the next pass.

## The air campaign over Guadalcanal (file 05)

**Q5.3, Q5.4, scrambles, warning, contact, target.**

- **Warning minutes and fighters in contact:** neither Miller nor Wings at War gives them, by any raid. This is a new gap row.
- **The scramble routine (Miller).** Miller quotes the 67th Fighter Squadron's historian on the noon raids. A red-and-white flag at the pagoda meant scramble: every plane that could fly took off two at a time, the Grummans climbed for height, and the P-400s and SBDs went off to work over Japanese ground (events 1249, 1250). The raid he describes is 18 to 24 bombers with 20 or more Zeros.
- **Standard practice (Miller).** When warned, the P-400s and SBDs took off before the raid and bombed or strafed Japanese positions (1248). The answer called this not found.
- **The warning system (Wings at War).** There was "no real warning system" until radar in September, and a black flag was raised when a raid threatened (1254). This uses the black flag for the alert, where Miller's source uses it for "take cover".
- **29 August (Wings at War).** The P-400s climbed to 14,000 feet; the 18 bombers held 17,000; the Grummans did the attacking, and 4 bombers and 4 escorts were claimed (1255, low).
- **30 August (Wings at War).** On a coastwatcher report of 22 planes, all aircraft came in at 1130 to be serviced (1256).
- **13 September (Miller).** Twenty F4Fs intercepted 14 miles north of the field. They went for bombers and fighters both. No doctrine of which to attack is stated anywhere (1252).
- **"Almost daily, at noon"** (Miller, 1251). This differs from row 281 (Wikipedia: no raids 1 to 8 September).

**Q5.6.** Wings at War: the SBDs flew long single-plane searches up to New Georgia and Santa Isabel (1253). It prints no start date; the answer's "from 24 August" is not in the source.

**Q5.7, aircraft at Henderson Field (Miller).**

- 11 SBDs of Flight 300 on 24 August (3957).
- Nine more P-400s on 27 August (3958).
- Eight F4Fs in commission on 29 August (3959).
- Five of the original F4Fs flyable on 30 August (3960, `disputed` with 569 and 1792, which say four).
- Twenty-nine more fighters and dive bombers by 30 August (3961, `disputed` with 571, 1795, 1796).

## Air attack and anti-aircraft fire (file 08)

**Q8.4, rounds per aircraft.**

- No source gives rounds for each aircraft shot down.
- **Yorktown, Coral Sea.** CINCPAC Gunnery Bulletin 2-42 (new source, primary) gives rounds per gun on the last day at Coral Sea: 5-inch 50, 1.1-inch 180, 20 mm 330, .50 cal 880 (outcomes 3947 to 3950). The bulletin does not date the day.
- **A destroyer division,** ships not named: 5-inch 65, 1.1-inch 500 (3951, 3952).
- **Half shot down by guns.** The gunnery officers' consensus that "about half the planes attacking were shot down by gun fire" is a single judgement, so it is kept low (3953).

**Q8.8, the share shot down before release against after.**

- No source gives a share. The leads:
  - Coral Sea: of 17 fighters in the air only 3 were vectored onto the raid before it developed (3954).
  - Coral Sea: Japanese dive bombers were "usually ... turned back by AA fire and dropped sooner" (3955).
  - Midway, Bombing Squadron Six: "No fighter opposition was encountered prior to pull-outs from the dives", and most planes were attacked after the pull-out (3956).

**Q8.1, Q8.2, Midway.** These are attackers' claims of hits, all low:

- Bombing Squadron Six's first division scored at least three hits (3943).
- Air Group Six as a whole: at least eight hits observed (3944).
- The torpedo attack is thought to have made one hit (3946).
- Fifteen VB-6 aircraft, each with one 1,000 lb bomb, is the squadron's own figure (3945, new source).

## Ship fits (file 01)

**Q1.10, Q1.11.** The answer file stops at Q1.9: the B5N2's radius and the stated strike radii of the SBD-3 and TBF-1 were not answered. Nothing was checked for them.

**Q1.1, Q1.2, Laffey.**

- **Q1.1.** The cited source is the 1967 cruise book of the later Laffey, DD-724. Its history page says DD-459 carried four 5-inch guns, but a book about another hull, written 25 years later, is not a source for her. Her four-gun rows are already held (fits 724, 748, 749).
- **Q1.2, fuel.** The FTP 218 figure of 466 tons is already held (1372). The table's ship list omits DD-459, as that row says.
- **Q1.2, displacement.** The ONI 222 citation is a scan with no text layer and was not checked.

**Q1.6, Q1.7.** Salt Lake City's rounds a gun and the radar of Salt Lake City and Duncan: the answer says not found, and the gaps were already recorded (842, 1129, 824, 849).

**Q1.9, tubes on one side by class.** No source read gives tubes on one side.

- **FM 30-58** (US Army, 29 December 1941) is a wartime recognition manual: a US estimate of the enemy, reliability C. It prints total tubes, which were keyed (ship_class_facts 1375 to 1380):

| Class | FM 30-58 | Standing |
| --- | --- | --- |
| Atago (Takao) | 8 | `disputed`, as the class's group already was: 16 against 8 |
| Mogami | 12 | `preferred`, agreeing with rows already held |
| Mutsuki | 6 | `preferred`, agreeing with rows already held |
| Kamikaze | 6 | `preferred`, agreeing with rows already held |
| Tone | 12 | `preferred`, agreeing with rows already held. The answer said not found |
| Fubuki | 9 | `preferred`, agreeing with rows already held. The manual lists Fubuki under "Shinonome class"; the answer said not found |

- **Myoko.** The cited PDF is ShipCraft 5 (Backer), not Lacroix and Wells. Its 12 tubes are the 1924 design (1381, `alternative`).
- **Nagara.** The quoted twin 533 mm tubes are the Kuma class's. The page gives Nagara eight 610 mm tubes, already held (1331).

**Q1.4.** Benham's Eastern Solomons report gives 16 tubes in four mounts (fits 758, new source, primary). Her General Information Book is a scan with no text layer and was not checked.

**Q1.8.** The Canberra radar passage in the RAN radar notes is by Lieutenant Commander Gregory, Canberra's officer of the watch, not by Stevens, who compiled the notes. It gives Type 271.

- **Not keyed.** It is Gregory's account again, and his rows (102, 353) already hold it; two copies are one source. Keyed, it would also have been read into the Savo Island derivation's provenance list, which changes that generated file for no new fact.
- The row was staged as fits 759 and removed before this pull request. Fits id 759 and source id 1228 are left unused.
- Madsen's sentence is reworded in the answer; it is already held (103, 355).

## Cape Esperance (file 07)

**Most of the answer was already held.** Twelve of its 22 claims were already rows, from the ONI narrative and the BuShips summary.

**Q7.2, the Japanese turn.**

- Both sentences ("all ships began an immediate retirement"; Aoba, Furutaka and Fubuki to starboard, Kinugasa and Hatsuyuki to port) are cited to a page on device.report. Every reader was refused there, including a browser and the web archive. Neither sentence was found anywhere else.
- The answer's labels for the document ("USSBS", "Japanese Navy after-action report") are unsupported.
- Not keyed. The Japanese record of the turn already held is events 509, 512 and 664.

**Q7.3, firing times and rounds.**

- No source gives rounds by ship (gaps 829, 830, 1105).
- The cited sentence for that absence is in chapter 6, not chapter 10.
- **Helena's "24 minutes".** The quote ("Main battery target … sinks") is real, elided from "Main battery target, Kako class cruiser, breaks in two and sinks". The 24 minutes is the model's own arithmetic from 2346 to 0010, and 0010 is not a cease-fire.
- **New rows from the narrative's chronological table** (`oni-cn-cape-esperance`, chapter 10):
  - Salt Lake City ceased, resumed and checked fire from 2347 to 0017 (events 1266 to 1277).
  - McCalla opened fire at 2353 (1278).
  - Buchanan's "main and torpedo batteries open fire on enemy cruiser" at 2353 (1279); the number of torpedoes is not stated.

**Q7.5, hits on Salt Lake City.**

- The BuShips summary gives Salt Lake City 3 gunfire hits (outcome 3971, `disputed` against 1590 and 1592, which say 2).
- The answer called two of them 8-inch. The summary gives an A.P. hit at frame 48, an H.E. hit at frame 29 and a short, with no calibre.

**Q7.6, Q7.7.** Radar ranges and Scott's instructions were already held.

**Q7.8.** The BuShips URL cited is missing a word and does not resolve. The statement is in the summary at the other URL and already held (1593).

## Pilot training (file 06)

The two USSBS interrogations on Wikimedia Commons are scans. They were read by OCR, and the key pages were read from the images. Each row says so. They are cited under `ussbs-interrogations`, with the Commons scan as the locator.

**Interrogation 444, Captain Mieno, staff of Carrier Division 5 in Zuikaku from September 1941 to July 1942.**

- **Pearl Harbor pilots' hours.** The best had 2,500 hours, the average about 800, the minimum 300 (training 98). This agrees with row 25, which may draw on the same interrogation.
- **Land-based pilots,** in his opinion, had as many as 1,500 hours (99).
- **1943 and after:** carrier pilots averaged about 600 hours, and 500 was then accepted as the minimum (100).
- **After Midway:** "during 1942, in the Solomons campaign, the quality of our pilots was maintained", though training in units was shortened (101).

**Interrogation 479, Captain Genda.**

- **After Midway,** about 200 of about 300 carrier pilots survived and went to Shokaku and Zuikaku (102).
- **For the August 1942 sortie,** the best pilots went to Shokaku, Zuikaku and Ryujo and the younger ones to Zuiho (103).
- **His "excellent pilots, most in the war from the start"** describes Carrier Division 1 at Rabaul in October 1942, after he joined; the answer gave it to August (104).
- **Hearsay.** He was not on the staff in August and does not know the plan of the 16 August sortie. These are memories of three years later.
- **Wrong page numbers.** The answer's page numbers are wrong: the complement change is on p. 6, the "best pilots" line on p. 7.
- **The air group's complement** changed from 18 to 27 fighters and from 27 to 18 torpedo bombers. This is verified, but `training_facts.csv` has no attribute for it. Not keyed.

**Mersky.** Many Japanese Navy pilots had been flying combat for perhaps a year or longer before August 1942 (105). The source is Mersky, Time of the Aces, not Sherrod as the answer says.

**Not found:**

- US Navy pilots' hours at the end of training or on joining a squadron in 1942. The answer has none (gaps 898, 1110).
- Whether the Japanese carrier air groups of August 1942 were the crews of December 1941. No source gives a share. What the interrogations give is above: Midway survivors went to Shokaku and Zuikaku, and the best pilots were put on Shokaku, Zuikaku and Ryujo.
- The Museum of Flight transcript (Carl, VMF-223) no longer resolves, either directly or in the web archive.

## Air upkeep (file 09)

This file feeds the slow figures of the Rabaul against Guadalcanal scenario. All its quotes were found. Two of its facts were already held: Mersky's figure (outcomes 568) and Miller's (495).

**New rows.**

- **Aircraft at Henderson Field, 26 October** (USSBS Campaigns, p. 119): 23 fighters, 16 dive bombers and one torpedo plane available for its defence (outcomes 3972 to 3974). The text says "available", not serviceable, and says the fuel shortage limited their flying.
- **Fuel by air** (same page): each transport flew in enough fuel "to maintain 12 fighters in the air for 1 hour" (3975).
- **Japanese availability at Rabaul in 1942** (Shaw and Kane, vol. II, p. 453): after an all-out attack, 80% of the aircraft were available the next day and 50% on the third (3976, 3977).
  - The passage rests on USSBS Interrogation 446, the 25th Air Flotilla's senior staff officer, from memory.
  - It puts the low availability of 1942 down to a lack of supply.
  - For 1943 it gives 50% at any time and 30% the day after an all-out operation; not keyed.
- **4,000 gallons of 100-octane petrol** found at the edge of Henderson Field (3978, low).
  - The cited document is misattributed. The PDF is Goralski and Freeburg, *Oil & War* (1987, reprinted 2021), a popular history that gives no source, not the US Navy history of energy security of 2010 the answer names. The row cites the real book, reliability C.

**Not keyed.**

- Japanese aircrews crashed stretching the range of the Val and the Kate (Shaw and Kane, p. 453). No count and no date.
- By October there was no fourth-echelon repair in the South Pacific, and major repair went to Australia. The 13th Air Depot Group reached Noumea on 22 November (Craven and Cate, vol. IV, p. 76). Qualitative.

**What each slow figure now has.**

| Figure | Sized by a row? |
| --- | --- |
| Wear | Partly. Japanese availability after an all-out day (3976, 3977) is one officer's recollection, and the source ties it to supply. No new US operational-loss rate: the older rows 482 and 487 stand |
| Repair time | None. Only the statement that major repair went to Australia |
| Fatigue | None |
| Pilots | None that sizes replacement |
| Supply | US fuel only: fighter-hours per transport load (3975), the 4,000 gallons (3978, low), and the 26 October counts held back by fuel. No figure for bombs, and nothing for Japanese supply beyond "lack of supply" |

## Fighter defence (file 13)

The file's sources were the ONI narratives of the Eastern Solomons and Santa Cruz, Zimmerman, Mersky, Miller, Tactical and Technical Trends, and the Navy's communiqués of August and September 1942. All eleven pages were read.

Its quotes are often one word ("1420", "88"), which proves little. Each was checked against the passage around it.

### Eastern Solomons

**Warning.**

- Radar had the raid at 88 miles at 1632, lost it for 17 minutes, and had it again at 44 miles at 1649. These were already rows: events 206 to 208, 213, 214.
- The fighters had 22 minutes to intercept (outcomes 3985).

**Fighters airborne.** 38 at 1649 and 53 at 1655. Already held: 215, 216, 218, 219.

**Fighters in contact.**

- No count for the whole patrol exists; this is a gap.
- The nearest is the ONI study of 38 VF-5 and VF-6 pilots (outcomes 3979 to 3984). Their counts do not sum to 38, so each is its own row:
  - 4 first engaged before the push-over;
  - 7 at the push-over or in the dive;
  - 7 in the retirement;
  - 8 never engaged;
  - 12 were engaged by fighters before they could reach the bombers.

**Heights.**

- Attackers at 12,000 feet by radar.
- Defenders at 10,000 to 15,000 feet.
- Both were already held.

**The narrative's judgement** (outcomes 3986, 3987).

- The patrol's mixed result is put down to radio discipline and the interposed Zeros.
- Dow ordered the fighters to concentrate on the bombers.

### Henderson Field

**The ground rules.**

- Daily four-plane patrols were flown at 0545 to 0830 and 1400 to 1830 (event 1295, Zimmerman). The wording is garbled and the note says so.
- The interception of 24 August was at 1420 (1294). This agrees with row 275.
- Japanese bombers customarily flew at 25,000 to 30,000 feet. The P-400's absolute ceiling was 14,000 (outcomes 3989, low; aircraft type facts 556, from Zimmerman's footnote).
- Japanese formations of 1942 bombed from 22,000 to 24,000 feet, in flights of 27, with Zeros weaving above, below and to port (outcomes 3990 to 3992, Tactical and Technical Trends no. 8, low).

**30 August (Mersky).**

- 8 F4F and 7 P-400 at 15,000 feet orbited for 40 minutes after the coastwatchers lost the bombers in cloud (event 1296). The height is disputed against Wings at War's 14,000 (outcomes 556).
- Zeros came out of the cloud, the bombers turned back, and 14 fighters were claimed (1297).

**The communiqués** (new source `navy-communiques-1942`, outcomes 3993 to 4013).

- The rows give raid sizes, claims and own losses for 26 and 29 August and for 2, 5, 9, 13 and 14 September. Enemy numbers and claims are low and unconfirmed.
- Communiqué 112's raid of "August 25 (Washington date)" of 16 two-engined bombers is keyed to the raid of 26 August, inferred. It is not the raid of 24 August.

### Santa Cruz

Events 1280 to 1293 and outcomes 3988, from ONI.

**Hornet group.**

- 8 CAP at 10,000 feet over the carrier at 0800.
- Outgoing strike planes reported two groups about 60 miles out at 0930, and 24 dive bombers at 60 miles on 280 at 0940.
- Hornet's radar had the raid at 20 to 30 miles at 1005. The narrative says "a few minutes before 1000".
- The patrol was seen attacking about 15 miles out.
- ONI judges that most enemy aircraft were "intercepted too late, often after they had completed their attack", because fighter direction was overloaded.

**Enterprise group.**

- 12 fighters on station, and 11 more launched at 0940.
- Radar contacts: 45 miles at 0957; 23 miles at 1100; South Dakota at 55 miles, tracked to 25; 45 miles at 1201, before the attack of about 1220.
- The radar plot reported the screen clear a minute or less before that attack.
- Attackers were at about 17,000 feet by radar, with nine fighters above.
- Vejtasa's flight was at 12,000 feet with bombers above it and was directed down to 7,000. It intercepted eleven torpedo planes at 13,000 feet.

**Not found:** how many of the whole patrol made contact, at the Eastern Solomons or at Santa Cruz (gap).

## Night action duration (file 12)

The file quotes Bates, the ONI narratives, the USSBS naval interrogations and Lundgren (NavWeaps) closely. 51 of its 73 claims were already rows. What is new:

**Why Mikawa withdrew from Savo Island** (outcomes 4014 to 4017, Bates chapter XVII, printed p. 220, OCR).

- **Bates's judgement:** "the real reason for the decision of Commander Cruiser Force to withdraw was his apprehension of Allied carrier-based aircraft".
- **His staff's argument:** re-forming the scattered force would take considerable delay, which would leave it within reach of Allied aircraft at dawn.
- **Mikawa's own account,** given in 1949, is that he wished to go back but let his staff prevail.
- **Bates's reconstruction:** 0330 as the earliest planned retirement (low, inferred).
- **Matsuyama** (USSBS interrogation, 18th Cruiser Division; outcomes 4020 to 4023): his orders were to pass round Savo, attack and go home. He did not know why the force retired.
- **Patterson** could no longer reach a torpedo position once Kinugasa turned north at 0205 (outcomes 4018).

All of this is a later judgement or postwar testimony. No contemporary Japanese statement of the reason was found (gap).

**The Japanese column's speed at Savo** (positions 542 to 545, Bates).

- 26 knots from the order for independent fire at 0136.
- Chokai at 26 knots on 069 at 0150. Row 209 reads 068.
- Furutaka at 26 knots on 000 at 0156.
- The eastern group retiring on 328 at 26 knots after 0216.
- The earlier rows had 26 knots up to 0140.
- Japanese fire ceased at 0216 (outcomes 4019).

**Torpedo reloads.** No source gives the time to reload a tube for any ship or class. Bates's times for Kako (0146, 0201, 0205) do not say which torpedoes were reloads. This stays a gap, now with the sources read.

**Tassafaronga, from the Japanese side** (USSBS interrogation of Toyama; outcomes 4024 to 4028, positions 546 to 548). None of this was held before.

- Orders: land supplies and a few troops.
- No gunfire unless necessary; a mass torpedo attack.
- Takanami fired about 70 rounds; the calibre is not stated.
- The force retired to the north-west for fear of interception.
- Speeds: 15, 12 and 24 knots.

**15 November** (outcomes 4029 to 4031, event 1298).

- Gatch took South Dakota out as "more of a liability".
- At 0133 Lee reversed course "to avoid a possible torpedo trap".
- COMSOPAC's order: engage "unless profitably engaged" otherwise.
- Kondo's instruction suspended the bombardment until the American surface force was destroyed (Lundgren).

**Also held now:**

- the "last episode of the action proper" on 13 November (outcomes 4033);
- Helena's order to assemble at 0226 (event 1299);
- Wright's instruction at Tassafaronga to close and sink cripples once action is broken off (outcomes 4032);
- the time Tisdale received command (event 1300).

**For the architect.** Bates's text and footnote call Mikawa's order "8th Fleet Signal Order No. 25". Positions row 193 calls it No. 26. The page image was not read.

## Cape Esperance hits (file 14, added after the merge of the first eleven)

File 14 asked for shell hits on Aoba, Furutaka, Kinugasa and the destroyers, for shells fired by ship, and for hits on the American ships by firer.

All 16 of its claims were checked. Seven were verified and nine were already held. Nothing was invented, and nothing in the file was misattributed.

**No source gives a Japanese damage survey's or a crew's count of hits on Aoba or Furutaka.**

- The 6th Cruiser Division's own report gives no count (rows 1510, 1694).
- Every number below is a historian's figure, and none names the counter.

**Aoba, hits received.**

- combinedfleet: "up to forty 6-inch and 8-inch shells" (1598). An upper bound.
- H-Gram 011: "over 40 hits" (1599). A lower bound.
- Gill, vol. II ch. 8: "40 hits on Aoba entailed her return to Japan for major repairs" (outcomes 4034, new, disputed). A flat 40 with no bound stated.
- Japanese Wikipedia gives no number. It says many of the hits were duds.

**Furutaka, hits received.**

- combinedfleet: "About 90" (1600).
- H-Gram 011: "over 90" (1601).
- Warfare History Network: "hit over 90 times" (outcomes 4035, new, low).
- Japanese Wikipedia: 90 or more hit places above the waterline alone (1602).
- None gives an upper bound, and nothing shows the four are independent counts.

**Kinugasa.** Naval Encyclopedia's four hits (outcomes 4036, low) repeat combinedfleet's (1605).

**Shells fired and hits together.**

- Boise: the Secretary of the Navy's public report of 1943 says she fired 1,000 rounds of 5-inch and 6-inch together (outcomes 4037, new source `navy-report-1943`, low). The calibres cannot be split.
- Shells fired and hits received are now held for one and the same ship for Salt Lake City and Boise: rounds in 1636, 1639, 2517 and 4037; hits in 1587, 1590 to 1592 and 3965.
- They are also held for Aoba, Furutaka and Kinugasa: rounds in 2276 and 2278 to 2281; hits in 1598 to 1605.
- No row gives the American shells fired at Aoba or at Furutaka by ship.

## Not found, and leads for the next pass

**In `gaps.csv`:** warning minutes and fighters in contact by raid over Henderson Field. Every other wanted fact this pass did not find was already a gap.

**Leads not keyed:**

- **The B5N2's radius with bombs, and the stated strike radii of the SBD-3 and TBF-1 (Q1.10, Q1.11).** Not answered by the outside model.
- **The Okumiya transcript (Q4.1)** and Benham's General Information Book (Q1.4). Scans; OCR would read them.
- **Enterprise's landing times on the evening of 24 August.** In her action report.
- **The NHHC Antiaircraft Action Summary.** Named by the answer to Q8, with no address.
- **The NHHC Savo pages and the CINCPAC war diary for August 1942.** These timed out for the outside model and were not tried here. The ONI narrative and Bates's analysis are already sources.
