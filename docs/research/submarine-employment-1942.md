# How submarines were employed and estimated in 1942

Read on 2026-11-23 for the architect. The question is how the Imperial Japanese Navy and the United States Navy decided where to put their own submarines in 1942, and how each side's command worked out where the enemy's boats were. The game's staff will later choose patrol waters for its own boats where the commander has named none, and form an estimate of the waters where enemy boats are working. This note asks of the record what patterns either navy followed, how often a source states them, and how firm they are. It settles nothing and proposes no rule and no figure.

It follows `submarines-in-1942.md` (the attacks, the deployments battle by battle, the sighting reports, anti-submarine practice) and does not repeat it. Rows of that pass are cited as "on file" with their ids.

## How the reading was done

Six Sonnet readers each took a set of saved texts and staged statements with the exact quotation, the text file and the line it starts on. The lead then:

- checked every quotation by program against its saved text (whitespace and case ignored; pieces joined by " ... " checked one by one), and checked that the quotation starts at the line given: all 486 staged rows passed, six after repairing characters a reader had read in the wrong encoding;
- removed six rows: five that repeat a row on file (16196, 16197, 16207, 16232, 16250) and one about an air commander, not a submarine;
- replaced each quotation of under 70 characters by the whole sentence or lines around it as saved, so that the row's text carries its own statement (the note says so);
- graded confidence by source: the Graybook `medium` (an OCR text layer, as most of its rows on file), the Rainbow Five plan, USF 25(A), the 1944 War Instructions, the Cominch bulletins, the ONI narratives and the USSBS interrogations `high`, the Technical Mission's reports, Japanese Monograph 93, Bates, Parker, the Admiralty's Battle Summary and combinedfleet.com `medium`, Gerry `low`.

Every row is an event of the holding engagement `submarine-war-1942` (id 1017), with `kind` `order` for an order and `other` for doctrine, estimates and judgements, and `standing` `unconfirmed`. Each note starts "Says:" with the statement in plain words, gives the figure as read where it is not a plain number, and ends with the heading (1 to 6, below) and the navy it describes. A row dated 1944 or later carries that date: such a row says what doctrine became, not what it was in 1942.

## What the rows hold

| File | Ids | Rows |
| --- | --- | --- |
| `engagement_events.csv` | 16436 to 16915 | 480 |
| `sources.csv` | 16016 to 16021 | 6 |
| `gaps.csv` | the last 48 entries | 48 |

By heading and navy (the navy whose practice the row describes; an American estimate of Japanese boats counts under the Japanese side only where it says what the Japanese did):

| Heading | USN | IJN |
| --- | --- | --- |
| 1. What submarines were for | 60 | 31 |
| 2. Where boats were placed | 59 | 20 |
| 3. Redeployment | 8 | 25 |
| 4. Command and control | 38 | 12 |
| 5. Estimating the enemy's boats | 137 | 29 |
| 6. What a force did on an estimate | 55 | 6 |

New sources: `wppac-46` (the Pacific Fleet's Rainbow Five plan of July 1941), `usf-25a-1944` (Current Doctrine, Submarines, February 1944), `ftp-143a-1944` (War Instructions, 1944), `parker-priceless-advantage` (the NSA history of communications intelligence at the Coral Sea and Midway, 1993), `admiralty-battle-summary-21` (the Admiralty's staff history of the Guadalcanal campaign, 1949), `gerry-nwc-1996` (a Naval War College paper, used once and graded C).

## What was read and what could not be got

Read: the Nimitz Graybook, volumes 1 and 2, in full for submarines (about 260 rows); the Rainbow Five plan (WPPac-46); USF 25(A) chapters I to V; the 1944 War Instructions chapters 1 to 14; Cominch Battle Experience bulletins 1, 3 and 4; OEG 51 for the Pacific; the Technical Mission's S-17 (Japanese submarine operations) in full and S-24 (anti-submarine warfare) for how the Japanese located American boats; USSBS interrogations Nav 56, 63, 72 (Miwa) and 74; Japanese Monograph 93 (Midway); Bates on Midway and the Coral Sea for the Japanese side; Parker; the Admiralty's Battle Summary No. 21; the ONI narratives of the Eastern Solomons and of the miscellaneous actions in the South Pacific; the combinedfleet.com records of movement of 26 Japanese boats; Gerry.

Could not get (each is a gap entry):

- **Japanese Monographs 102, 108, 110 and 111** (submarine operations, December 1941 to March 1943). The HyperWar index now links them, but every link returns "not found"; there is no copy on the Wayback Machine or archive.org. JM 102 is printed in a book of Chihaya's essays, not read.
- **USSBS Nav 107 and 108**, the interrogations on submarine attacks on convoys and on Japanese submarine operations: no copy found.
- **The Sixth Fleet's and the submarine squadrons' own war diaries** for August to November 1942 at JACAR: one web search, not searched in the catalogue itself.
- **SRH-012**, the wartime history of radio intelligence in the naval war, 1942: catalogued, not online.
- **COMSOPAC's and the task forces' war diaries** for August and September 1942, and any CINCPAC or COMSOPAC intelligence bulletin: an NHHC copy of a CINCPAC war diary excerpt refused the fetch; fold3 is paywalled.
- **A 1941 or 1942 edition of Current Doctrine, Submarines, and FTP 143 of 1934**: only the 1944 editions are online. So American submarine doctrine in 1942 rests on Rainbow Five (1941), the Graybook and the Cominch bulletins, and the 1944 books say what it had become.

Not read: the war patrol reports of the American boats; Morison, Roscoe, Blair, Holwitt, Boyd and Yoshida (none was needed to locate a primary statement found here).

## 1. What submarines were for

### United States

- **In the plan of July 1941**, the Pacific Fleet's submarines were to help defend Midway and Wake and "as a habit" to operate offensively in enemy waters (16698); in the first phase, to keep up the strongest practicable patrol against Japanese forces and communications near the homeland (16702, 16700); two boats each at Wake and Midway "for gaining information and for attack on enemy units approaching" (16699). The operation plan of 10 December 1941 repeats the Wake and Midway patrols and the patrol off Japan "at the maximum strength permitted" (16436). Firm: the plan itself and CINCPAC's war diary.
- **Shipping became the stated first object in the spring of 1942.** COMINCH on 2 March: every boat that can be spared from the Mandates to attack the lines of communication where they leave Japan, "at focal or other favorable points" (16486); on 17 April, making "enemy ships and shipping the primary objective" of surface, air and submarine attack (16519). CINCPAC's own statement of 31 October 1942 is the clearest: aircraft scout better, so most boats should stay in focal areas with "information mission secondary to attack" (16643), answering COMINCH's remark that the Navy was not using its boats enough for scouting (16642). Firm: dated dispatches.
- **Special missions** recur but are few: reconnaissance of Makin before the raid (16532, 16533) and the raid itself from submarines (16609), a boat for MacArthur and for munitions to Corregidor (16485), a special mission for Drum (16504). The 1944 doctrine names the tasks as patrol, scouting, screening, mining, reconnaissance, services to aircraft and, exceptionally, escort; the primary task is to sink ships (16712, 16713, 16726).
- **Fleet work at Midway was the exception, not the rule.** Outside Midway the record shows boats sent to the Mandates, Truk, the Bonins, the Empire and the Aleutians as the war on shipping and the watch on bases required (section 2), and the staff judging the boats "not too impressive" (16511) or "ineffective" in the Solomons (16687, 16695).

### Japan

- **Boats as the fleet's scouts and ambushers, saved for the American fleet.** On file: S-17 (16340 to 16344), Miwa (16366 to 16374). New: the investigator's reading that Japanese operations were conceived as a decisive fleet action centred on the battleship, "as the Standard Doctrine in Naval Warfare shows", and that submarine tactics were never changed to fit what happened (16798). Firm as a post-war judgement from Japanese papers; the doctrine text itself was not seen.
- **What the boats were set to do, by period, from the Navy Ministry's own summary** (S-17 Enclosure C): from December 1941 about 27 boats of Squadrons 2 and 3 to watch the American fleet at Hawaii (16796), Squadron 2 there until about the end of January (16797); from April to June raids on shipping off eastern Australia and Fiji by about 13 boats, then 7 of Squadron 3 until early August (16792); in 1943 part of the large boats posted round Hawaii, Fiji and Espiritu Santo to watch and attack the fleet (16795). Harbours from which the American fleet would sortie were reconnoitred before fleet operations (16804) and ports kept under "fairly constant" surveillance (16803); boats picked up airmen on the strike track (16802). Medium: OCR of a 1946 report from Japanese papers.
- **Supply to Guadalcanal from November 1942**, ordered by Yamamoto through the Sixth Fleet (16910), decided by the Naval General Staff (Miwa, 16813) on the Army's request (16814, 16805); CINCPAC knew it within days (16680, 16681, 16683).
- **Against the American line of communication in September and October 1942**, as the Americans and British read it: ONI's "strong submarine offensive" in the Coral Sea between the Solomons and Espiritu Santo (16878), the Admiralty's "well-planned submarine offensive" against the carriers covering the line (16864), boats working as far as 13 S (16868).

## 2. Where boats were placed

### United States

- **Off the enemy's ports, straits and passages.** Rainbow Five ranks seven stations off Japan: Yokohama, Bungo Channel, Kii Channel, Tsushima, Nagasaki, Shimonoseki, Tsugaru (16701). Every one is a port approach or a strait. In 1942 the boats went off the Marshalls (five by 4 January, 16455), Truk and Saipan (16464), north-west of the Bonins against a concentration (16499), Kavieng, St George Channel, Faisi and Bougainville Strait (16633, 16625), the Shortlands and Faisi (16632), Rabaul (16570, 16566), Kiska, Attu and Paramushiro (16562). CINCPAC in August 1942 sought "more submarine hours in major focal points" of Japanese communications (16583); in December the staff's diagnosis of failure was that patrol positions did not concentrate at the two focal points, Rabaul and Buin (16695). Firm and often stated: this is the American pattern.
- **One or two boats as a picket off one's own outlying bases**: two each at Wake and Midway in 1941 (16699), cut to one and then none as the Marshalls needed boats (16438, 16439, 16454); about four off Midway in the May 1942 plan (16548), Gudgeon north-west of Midway on radio intelligence (16844), the search moved 50 miles north-west of the island (16843). Midway's arcs at 200, 150 and 50 miles are on file (16168 to 16175).
- **Spacing and line length**: none stated for 1942. The only American figure is the 1944 doctrine's scouting distance, at most 20 miles between boats on the surface and 12 submerged (16732). The 1944 books describe the choices: a wide line across the advance (few attacks), a line parallel to a narrow approach (an attack each in turn), boats near the enemy's point of departure and at focal points on his route, or concentrated near his destination when he is fast or has several routes, and boats placed on his line of retreat (16715, 16716, 16741, 16742, 16743, 16731).
- **Numbers per task**: 20 fleet boats meant for Australia (16512, 16581, 16615); 24 under Halsey from October, 10 left for the western Pacific (16650, 16652, 16872); COMSOPAC wanting 14 for the Solomons, 7 on patrol (16658, 16660); 16 on patrol over the Solomons on 7 December (16687); 3 boats to a point 75 miles north-east of Santa Isabel (16667); MacArthur asking 30 to 36 (16631, 16676).

### Japan

- **On the expected track, not searching for the force.** S-17: "as a general rule" boats were placed on the expected track of a task force (16773); whenever a move was expected the fleet commander guessed the track and every spare boat was rushed to it (16770). On file: 16 boats on two lines 200 miles apart, boats 30 miles apart, across the expected approach from the south-east in the Solomons (16348), the lines 200 miles apart so a force crossing the first in darkness would meet the second (16772, new). This is stated several times in one source, of one battle; firm as practice, single as a figure.
- **Picket lines near the American approaches in 1942**, from combinedfleet: a picket line in the Hawaii area relieved in January (16913), the Torres Strait (16912), the Sunda Strait (16911), south-east of Guadalcanal (the A group of nine boats, 16905), west of Espiritu Santo (16906), west of San Cristobal against reinforcement (16907), Indispensable Strait with midget carriers (16914); the Admiralty's Forces A and B of six boats each (16869). Six boats for the Coral Sea, which Bates thought too few (16836, on file 16247, 16248).
- **The Midway cordon** was to be between Hawaii and Midway five days before the appointed day (16831); a third line (HEI) was planned and not manned (16835). Its lateness is on file (16196, 16197).
- **1944, as a lesson**: the NA line, seven boats 30 miles apart (16780), "very likely" found near enemy air bases (16787); a flexible sector to be chosen rather than a line (16788). Later doctrine, not 1942.

## 3. What moved boats, and how fast

### United States

- **Few cases and no stated delay.** Three boats ordered north-west of the Bonins on 13 March were on station by 15 March (16499; the gap entry gives the pair). A boat off Kavieng moved north-east of Bougainville Strait (16625); a boat ordered after its attack to intercept a cargo ship on radio intelligence (16636). At Midway all boats were ordered to close the striking force and only Nautilus attacked (16753; on file 16181 to 16193). The 1944 doctrine wants stations shifted "frequently" to follow the shipping and escape anti-submarine effort (16729), and on a contact report a boat gets ahead of the force out of sight (16735). No interval is given anywhere.

### Japan

- **A report, then an order the same day, and boats on the new line in two to three days.** Two hours after the Enterprise raid of 1 February the Sixth Fleet ordered seven boats out (16893); on 9 December 1941 all of Squadron 1 was sent after a carrier I-6 reported (16892); on 20 August 1942 a flying boat's report brought the order east of San Cristobal and I-26 reached the line on 23 August (on file 16265 to 16268, 16281 to 16283); I-17's report of Hornet on 13 October brought a redirect the same day (16903); a floatplane from I-15 found Hornet on 16 October and I-26 was sent (16904); I-174 and I-175 diverted on the way to a line (16908). French Frigate Shoals: I-123 reported the same night and the operation was cancelled two days later (16897, 16898).
- **The record's verdict is that this was usually too late.** S-17: when American forces struck elsewhere than expected the boats were rushed to the actual point, "usually too late and on the surface" (16776); the Japanese themselves listed the straight line and the outguessing of striking forces among their faults (16777); boats diverted late to a threatened point "are not expected to give satisfactory results" (16801, 1944). At Midway the Combined Fleet ordered the boats to a new line on 5 June and they waited 140 miles west of Midway (16829, 16830); Bates calls the late stationing "a major flaw" (16834).
- **The one timed relay from order to boats** is of 1944: the Advance Force's order to shift three boats 60 miles was dated 1542 and the squadron commander passed it to the boats at 1959 (16782, 16783), and the line was moved after the high command learned from American radio traffic that boats had been attacked (16784, 16785).

## 4. Who ordered the boats, how, and when they reported

### United States

- **Rainbow Five sets two classes**: boats for the local defence of a base under the base commander; boats as fleet forces under a fleet task force commander (16708, 16709). The Undersea Force commander keeps everyone informed of his boats' positions and routes (16706); boats routed west cover a wide front (16705).
- **In practice the boats were moved between area commands by dispatch and argument**, not by the task force commander: CINCPAC and CINCAF over the East China Sea boundary (16465, 16469); MacArthur refusing Pacific Fleet boats in his area unless under his control (16616) and asking 30 (16631); the Joint Chiefs putting Task Force 42 under COMSOPAC on 19 October (16639); COMSOPAC then passing his boats good information of enemy movements (16649). Firm: CINCPAC's war diary.
- **Reporting**: Rainbow Five, report important enemy movements only if the attack is not jeopardised (16707); the 1944 War Instructions, radio silence except for vital contact reports because direction finders find the boat quickly (16738), and no report from enemy waters unless worth giving away the position (16739); orders reach boats on prearranged schedules, by low frequency to boats submerged, repeated at night (16719, 16722). At Midway all planes and boats reported on one frequency the ships also heard (16754), and Tambor's report lacked course and speed (16755). The 1944 doctrine names no fixed positions for boats in a fleet disposition: they are coordinated with, not combined with, surface forces (16717, 16723).

### Japan

- **The group commander set the stations and would move the whole line by radio at night** on a notification from the Commander-in-Chief (16771). Boats were tied to their ambush stations and told not to leave them (16775). The Midway plan split operating areas by latitude between squadrons and task forces (16833). In 1944 orders came from four levels with "no coherence" (16774). The Sixth Fleet in 1942 gave orders straight to boats (16895, 16896), and supply runs were ordered by the area command, not the Sixth Fleet (16815).
- **Reporting** is on file: the boat that made contact held its report until clear (16345), neither boat at the Wasp attack reported (16349), reports went to headquarters not to the carriers (16370). New: a boat's last signal was a sighting report (16909); at Midway the American intercept heard Japanese submarine traffic in code amid plain-language air traffic (16846).

## 5. How each side estimated the enemy's boats

### United States

- **What the estimate looked like.** No periodic submarine estimate was found. CINCPAC's war diary gives running sentences: "about fifteen" boats from Hawaii to the West Coast (2 January, 16451); three boats near Midway by direction finding (23 January, 16466); one or two near Wake and a concentration forming (23 and 24 February, 16482, 16483); at least two between Pearl and Midway, one at French Frigate Shoals (4 and 5 March, 16493 to 16495); "very possible" boats in the Hawaiian area, "for observation only" (24 April, 16524); "a number of submarines" in the Oahu-Midway area (8 June, 16550); at least two approaching Oahu (July, 16579); at least 17 in the Solomons as an advance screen (14 September, 16613); at least 20 on station and 32 available (1 October, 16628); about 40 available for the South Pacific (23 November, 16682). Estimates of Japanese squadrons in order-of-battle tables (8 to 10 boats of Squadron 1 for Alaska, 16549; Squadron 7 at 12 boats at Rabaul, 16567; one squadron to the southward, 16602).
- **On what evidence.** Direction finding is named at Midway in January and at Wake in February (16466, 16482), off Pago Pago (16502), in the Midway warning of 29 May (Parker, 16842) and in the carrier commander's count of "approximately 9" boats on 7 September, the only estimate in the second volume that states its source (16610). Sightings by aircraft and ships, and attacks suffered, make up most of the rest (16462, 16468, 16481, 16608, 16586, 16587, 16645). From October radio intelligence was blind: a change of call signs made traffic analysis impossible (16626, 16638). The staff discounted reports it could not confirm (16508, 16509, 16523).
- **What the staff expected by habit**: Rainbow Five already expects boats to raid and observe off Oahu and the outlying islands and on the lines of communication, with a strong concentration in the Mandates (16710, 16711); the staff of January 1942 that the only fleet base lets enemy boats concentrate (16446) and that boats would be sent against the Samoan detachment (16457); in August and September, boats on the supply line and off the unloading at Guadalcanal "known to be present" (16589, 16590, 16604, 16607); in October and December, continued attacks on the line of communication (16651, 16685).
- **How right.** CINCPAC estimated twelve boats with the Port Moresby forces against eight (Parker citing Lundstrom, 16849, 16850). In September 1942 the Technical Mission's 16 boats in the Solomons (on file 16348) sit between the carrier commander's 9 and CINCPAC's 17. The October and November totals (20 on station, 40 available) were not checked against a Japanese count here. ONI thought the boats of September "probably not large" in number (16879). In January to March 1942 the boats CINCPAC placed between Pearl and Midway and at French Frigate Shoals tie to the K operations whose existence Rochefort read on 5 March (16838, 16839).

### Japan

- **1942 rests on single cases.** Destroyers ahead of the main body reported five American boats 40 miles off Okinoshima on 29 May (16827); a long emergency signal to Midway on 30 May led them to assume boats near the transports (16828; Parker heard the same, 16845); the Midway convoy turned 50 miles north on a report of boats north-west of Wake (16826) and left Saipan with a deception round Tinian (16825). JM 93 records nine ships sunk near the homeland in May (16824). On file: the staff at Midway counted 14 or 16 American boats (16200 to 16202); Japanese belief of one or two Allied boats in the Bismarcks against four (16257).
- **The late-war method, undated or 1943 to 1945**: the Grand Escort Fleet located American boats "chiefly by statistical data", with interception, direction finding, sightings and ships' reports (16806); it could intercept American submarine signals but not read them (16807); Tokyo kept a plot of the whole Pacific and radioed it to the area commanders (16808, 16819); it predicted when groups of boats would come and go and issued printed bulletins to escorts (16809); direction-finding bearings often arrived too late (16820). Later than 1942 and stated by two sources (S-24 and Nav 74); firm as late-war practice, silent on 1942.

## 6. What a force did on an estimate

### United States

- **The carriers' box south-east of Guadalcanal.** Ghormley on 27 August: the carrier forces to operate generally south of 10 S unless a promising target was within reach (16875; ONI, 16881); Fletcher's forces in a box from 10 30 to 13 S and 163 30 to 164 30 E (16863); ONI's area of 150 by 60 miles and 12 crossings of one's own track (on file out 16012, 16016). On 4 September COMSOPAC told Task Force 17 to operate south of 12 and "avoid remaining in same locality" (on file 16305); on 7 September the carrier commander, counting about nine boats, called the directive "unduly restrictive", asked for freedom within 300 miles of Guadalcanal and two destroyers for each heavy ship (16610, 16611, 16620, 16621), and COMSOPAC repeated the grant of freedom of movement and suggested moving east at good speed (16612, 16622). On 20 September COMINCH wrote that prolonged operations from static bases in "submarine-infested waters" were increasingly hazardous and asked for reports (16619); COMSOPAC asked to anchor the carriers in a protected anchorage except on specific tasks (16624); in November Halsey would not keep carriers and battleships at sea continuously in such waters (16679). ONI's verdict: the limited field and the need to stay in one area for long periods (16880). The Admiralty's: a well-planned offensive against the carriers' line of communication (16864). This is the best-documented case in either navy; firm.
- **After an attack: clear the area.** Task Force 17 on 6 September cleared the suspected area to the south-west and the next day worked 120 to 180 miles away (16866, 16883), manoeuvring round four doubtful contacts (16867, 16884); destroyers stayed behind to hunt (16603, on file 16306).
- **Routing round reported boats**: in March 1942 Task Force 16 was recalled to Pearl and its oiler turned back because boats concentrated near the rendezvous (16496); in November reinforcement groups routed north and then south of San Cristobal round a cluster of boats (16762 to 16764); Washington sent to fuel at Noumea because of a "known submarine menace" east of Espiritu Santo (16654); Turner's convoy kept east in spite of the submarines to avoid air search (16876).
- **Speed and zigzag**: 23 knots and zigzag all night before Santa Cruz (16757); Enterprise held to 10 knots after a steering casualty "to reduce the submarine hazard" (16890). The 1944 War Instructions: avoid known submarine waters and waters crossed in the last 24 hours, zigzag in submarine waters even when screened (16745, 16746). No 1942 order giving speed, zigzag or a time limit as a rule was found (gap).
- **Patrols**: a continuous air patrol of the entrances at the Guadalcanal landing (16861), an air patrol and destroyers after sinkings off New Caledonia (16536), PC boats asked for the unloading waters (16589), sweeps before each arrival of transports at Lunga (16674), radar-equipped flying boats, floatplanes and PT boats against boats off Guadalcanal (16668), a minefield at French Frigate Shoals "especially" against submarines (16505), and surface vessels sent there on radio intelligence, which made the Japanese boat give up (16840, 16841).

### Japan

- Only the Midway convoy's deception and its 50-mile turn in 1942 (16825, 16826), and the late-war escort practice of changing convoy routes on submarine positions (16810, 16811). Nothing on a fleet force's speed, zigzag or time in an area in 1942 (gap).

## Where sources disagree

- **Boats with the Midway and Aleutian operations**: Parker gives 21 in one passage (16853) and 16 in another (16854); the rows on file give 15 for Midway.
- **The Port Moresby forces**: twelve boats in CINCPAC's estimate against eight in Lundstrom as Parker cites him (16849, 16850).
- **Boats in the Solomons in September 1942**: about 9 (the carrier commander, from direction finding and contacts, 16610), at least 17 (CINCPAC, 16613), 16 on two lines (the Technical Mission, on file 16348), six or eight in the line of 13 September (on file).
- **What the Japanese did in the second half of 1942**: the Navy Ministry's summary has shipping raids east of Australia until early August and then boats placed near Guadalcanal (16792, 16791); Gerry has the Indian Ocean and the South-West Pacific (16837, low).
- **The carriers' southern limit**: 10 S in Ghormley's order of 27 August (16875, 16881) and 12 S in COMSOPAC's of 4 and 9 September (on file 16305, 16622). Two orders in succession, not a dispute; a row that gives "the" limit should say which date.

## Gaps

The last 48 entries of `gaps.csv`. The ones that matter most to a staff's rule:

- the Japanese Monographs on submarine operations and USSBS Nav 108 (both still not got), and the Sixth Fleet's own war diaries (not searched at JACAR): so the Japanese orders of 1942 rest on combinedfleet.com, the Technical Mission and JM 93;
- the positions of the Solomons lines (distance and bearing from Guadalcanal or from the expected track): only the spacing is stated;
- any delay from a report to boats on a new line, for either navy in 1942, beyond the date pairs in section 3;
- an American periodic submarine estimate with its sources, and the means by which CINCPAC's counts of 17, 20, 32 and 40 boats were reached;
- a 1942 order of either navy giving speed, zigzag or a time limit in an area as a rule against submarines;
- a 1941 or 1942 edition of American submarine doctrine, and radio-silence orders of 1942 for either navy;
- a Japanese estimate of American boats in 1942 other than single sightings.

## What a staff's rule could and could not rest on

This section says which patterns the record states often and clearly enough to be made a doctrine value by navy and date, and which are single cases. It designs no rule.

**Stated often and clearly enough.**

- *American boats work off enemy bases, straits and focal points of shipping, not on lines.* Rainbow Five's seven stations off Japan (16701), the patrols off the Marshalls, Truk, Saipan, the Bonins, Rabaul, Kavieng, St George Channel, Faisi, Bougainville Strait, the Shortlands, Kiska and Attu (section 2), CINCPAC's "focal points" of August (16583) and of December (16695). Many rows, from the plan to December 1942. A staff could name waters by "off an enemy base or in the strait leading to it" without a figure. How many boats, and how far off, is not stated (one or two per station is what the rows show).
- *American boats are placed for attack first and information second, from spring 1942.* COMINCH 17 April (16519), CINCPAC 31 October (16643). Two dated primary statements and the pattern of the stations.
- *American pickets of one or two boats off one's own outlying bases*, drawn in when a threat to the base is read: Wake and Midway in 1941 (16699, 16436), Midway in May and June 1942 (16548, 16844, 16843; on file 16168 to 16193). Several cases at one base type.
- *Japanese boats put on the expected track of an American force, in one or two lines across it, set before the force is seen and moved by order after a sighting.* S-17's "as a general rule" (16773, 16770), the two lines 200 miles apart and boats 30 miles apart (on file 16348, 16772), the Midway cordon, the Coral Sea line, the lines east of San Cristobal and south-east of Guadalcanal (section 2). The general pattern is stated by the Technical Mission and borne out by about a dozen 1942 placements. The figures (200 miles between lines, 30 between boats) rest on one source and one battle; 30 miles recurs on the NA line of 1944 (16780).
- *A Japanese report moves boats by order the same day; boats reach the new waters in about two to three days, too late for a moving force.* The 20 August pair and its three days (on file), the same-day orders of 1 February, 13 October and 16 October (16893, 16903, 16904), and S-17's verdict "usually too late" (16776). The direction is well supported; the interval rests on one clean pair and dates in records of movement.
- *A force that stays in one area is found by submarines; the remedy was to move, clear the area after an attack, and route round reported boats.* Cominch on the Coral Sea (on file 16392, 16748), COMSOPAC's order of 4 September (on file 16305), the carrier commander's and COMINCH's dispatches of September (16611, 16619), Halsey's November rule (16679), ONI (16880), the routing of November (16762 to 16764). Many primary statements of one navy in 1942. No source gives a time or a distance: "several days" (on file 16392), "fairly long periods" (16880); the 24-hour rule is of 1944 (16745).
- *American estimates of enemy boats, in 1942, are counts in an area with a basis that is mostly sightings and attacks, with direction finding where it worked, and an expectation by habit of boats off one's own bases and on the supply line.* About 140 rows. The counts are given with the words "at least", "about" and "a number", and the one check against the record (12 against 8 at the Coral Sea) runs high.

**Single cases or later doctrine, not enough by themselves.**

- The American scouting distance of 20 miles surfaced and 12 submerged (16732): 1944 doctrine, no 1942 statement.
- Any distance from one's own objective to a line: Midway's 200, 150 and 50 mile arcs (on file) and the Japanese 140 miles west of Midway (16830) are each one battle.
- The relay from order to boats (4 hours 17 minutes) is a single 1944 case (16783).
- The Japanese estimate of American boats in 1942: single sightings only. The plot, the statistical prediction and the bulletins (16806 to 16809) are late-war.
- The 1944 American rules on avoiding waters crossed in 24 hours, zigzag and screen spacing (16745 to 16747): later doctrine; the Cominch bulletins of 1942 state the danger without the figures.
- How right the American counts were in the South Pacific from October: not checked; radio intelligence was blind on submarines then (16626, 16638).
