# Outside answers of 13 November 2026, checked against their sources

On 13 November 2026 the designer put eight research prompts to an outside model (ChatGPT, through Codex). Its answers are tables of claims, each with a source, a page and a quoted sentence. An outside model's answer is a lead, not a source. A row was keyed here only where one of our readers opened the cited document, found the statement in it, and keyed the row from the document's own words. The answer files stay outside the repository.

The reading was done by Sonnet agents, one for each answer file. Each fetched the cited document as raw text (HTML, or PDF through pdftotext; two scanned USSBS interrogations were read by OCR and the key pages by eye), searched it for the quoted words, and read the passage around them. Every staged row's quote was checked by program as a span of the saved text: 86 of 86 passed. The lead re-read the disputed cases (the Cape Esperance table quote, the misdated Saratoga strike, the Gunnery Bulletin figures) in the saved text. This page says what the rows hold, what the answers got wrong and what is still missing. It settles nothing.

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
| **All** | **148** | **129** | **44** | **9** | **9** | **1** | **15 (10%)** |

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
- **What it adds is small.** 44 of the 129 verified claims were already in the database from the same source. The new rows below come mostly from passages our readers found next to the cited quote, not from the answers themselves.

Used this way, as a list of documents worth opening, the channel is useful. Used as a source, it would have entered about one wrong attribution in ten.

## What the rows hold

| File | Ids | Rows | What |
| --- | --- | --- | --- |
| `sources.csv` | 1223 to 1231 | 9 | New sources: Bombing Squadron Six's Midway report; CINCPAC Gunnery Bulletin 2-42; FM 30-58; ShipCraft 5; Benham's Eastern Solomons report; RAN radar notes; the War Damage Reports for San Francisco, South Dakota and Boise |
| `engagement_events.csv` | 1241 to 1279 | 39 | Opening of fire at the Eastern Solomons; Henderson Field's scramble routine; Salt Lake City's firing at Cape Esperance; Savo and 15 November times |
| `engagement_outcomes.csv` | 3942 to 3971 | 30 | Rounds per gun at Coral Sea; Midway bomb hits; hits received by San Francisco, South Dakota, Boise and Salt Lake City; aircraft at Henderson Field |
| `ship_class_facts.csv` | 1375 to 1381 | 7 | Japanese torpedo tubes by class, from a wartime US Army recognition manual, and Myoko as designed |
| `fits.csv` | 758, 759 | 2 | Benham's 16 tubes; Canberra's Type 271 |
| `training_facts.csv` | 98 to 105 | 8 | Japanese carrier pilots' hours and who flew in August 1942, from two USSBS interrogations |
| `gaps.csv` | last line | 1 | Warning minutes and fighters in contact, by raid over Henderson Field |

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

**Q1.8.** The Canberra radar passage in the RAN radar notes is by Lieutenant Commander Gregory, Canberra's officer of the watch, not by Stevens, who compiled the notes. It gives Type 271 (fits 759). It is not independent of the Gregory rows already held. Madsen's sentence is reworded in the answer; it is already held (103, 355).

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

## Not found, and leads for the next pass

**In `gaps.csv`:** warning minutes and fighters in contact by raid over Henderson Field. Every other wanted fact this pass did not find was already a gap.

**Leads not keyed:**

- **The B5N2's radius with bombs, and the stated strike radii of the SBD-3 and TBF-1 (Q1.10, Q1.11).** Not answered by the outside model.
- **The Okumiya transcript (Q4.1)** and Benham's General Information Book (Q1.4). Scans; OCR would read them.
- **Enterprise's landing times on the evening of 24 August.** In her action report.
- **The NHHC Antiaircraft Action Summary.** Named by the answer to Q8, with no address.
- **The NHHC Savo pages and the CINCPAC war diary for August 1942.** These timed out for the outside model and were not tried here. The ONI narrative and Bates's analysis are already sources.
