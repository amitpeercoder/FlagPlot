# Escorts of the runs of 1942: size, stations, cover and conduct

Status: research report for the architect, read on 2026-11-15. Nothing here is a ruling. Rows are `unconfirmed` unless the row says otherwise; where sources disagree each has its own row. It adds to `docs/research/oob/runs-to-guadalcanal.md` and does not repeat it.

## What was asked, and the short answer

For decision 0113 items 9 and 14 (an escort force that keeps station on what it escorts and fights for it; a covering force that stands off): the screening stations and orders of Task Force 67's Operation Plan 1-42, escorts per run, close escort against covering force, runs turned back, and the Japanese destroyer runs where some carried cargo and some sailed as the screen.

**No rule of the time for the number of escorts was found.** Not for the American side (the one plan read, Task Force 67's, says where the screen stands and what it does, never how many ships it has) and not for the Japanese side. What exists is counts of what sailed. The only stated change of escort strength in a run, Turner's of 12 November, is a decision made on the day: he took almost every escort out of the convoy to make a fighting force, leaving three destroyers. No run was found turned back for want of escorts. Both facts are in `gaps.csv`.

## What was added

| File | Rows | What |
| --- | --- | --- |
| `troop_lift_facts.csv` | 32 (ids 541 to 572) | The Task Force 67 plan (8 rows), Tanaka's and Toyama's accounts of the runs of 25 August and 30 November (8), Hough on the convoy of 14 November and the runs of 12 November, 11 October and December (11), Miller, Cox and the Survey on cover (5) |
| `lift_entities.csv` | 2 (ids 1391, 1392) | Task Force 67; the drum run of 3 December |
| `sources.csv` | 2 (ids 1462, 1463) | The operation plan; H-gram 012 |
| `gaps.csv` | 9 | What was looked for and not found |

How they were read. The operation plan is a 25-page typed scan on destroyerhistory.org, read as page images; machine-read text was checked against the images for the pages cited (2, 13, 14, 20 to 22; page 24 as text only). Two annex D quotations differ from the machine text only where it dropped words, and were checked by eye on the image. Toyama's interrogation is the HyperWar transcription (`IJO-60.html`), not the PDF on upload.wikimedia.org, which was not opened. Tanaka's drum passage is the destroyerhistory.org excerpt. Hough, Miller and the Survey are the HyperWar transcriptions; H-gram 012 is the PDF. Every quotation was checked by a program against the text, with the page from the printed-page markers (a page is the one after the marker). No site refused a plain fetch.

## Escorts per run, with the transports they escorted

`T` is `troop_lift_facts` (rows 541 to 572 are new). A blank is a gap. "Held" marks a fact the earlier note already had, for the table to be whole.

| Run | Transports | Close escort | Cover or covering force | Rows |
| --- | --- | --- | --- | --- |
| 7 to 9 Aug, US landing | the transports and cargo ships of Task Force 62 (T 396) | Task Force 62's cruisers and destroyers | A covering force of five US and two Australian cruisers with destroyers, split into a northern and a southern screening force across Indispensable Strait; three carrier groups | 569; T 105, 106, 396 held |
| 22 Aug, US | Alhena, Fomalhaut | Blue, Helm, Henley (uboat.net) or "destroyers" (Miller) | Smith's fighters | T 392 to 394 held |
| 14 to 18 Sep, US | 6 transports (7th Marines) | 3 cruisers, destroyers, minesweepers | Hornet and Wasp group; Covering Force | T 403 to 408 held |
| 9 to 13 Oct, US | Zeilin, McCawley | 3 destroyers, 3 minelayers | 4 cruisers, 5 destroyers (Scott) off the left flank; station near Rennell Island (Hough) or Russell Island (USSBS); Scott left the station 11 Oct to meet the Express, his orders charging him to protect the transports and to search for and destroy | 567, 568; T 410, 411 held |
| 8 to 12 Nov, US | Turner: 4 transports; Scott: 3 cargo ships | Cox: Turner's group 4 cruisers (San Francisco, Portland, Helena, Juneau) and 10 destroyers; Scott in Atlanta with 3 destroyers; at nightfall 12 Nov Turner kept one damaged and two low-fuel destroyers and gave the rest to Callaghan | Callaghan's 5 cruisers and 10 destroyers "to cover the movement" from Noumea; Kinkaid's carrier, 2 battleships, 2 cruisers, 8 destroyers in support; all South Pacific aircraft | 562 to 566; T 433 to 435, 440 held |
| 19 to 25 Aug, Japanese | Kinryu Maru and 4 old destroyer transports | Jintsu and 3 destroyers (Zimmerman); 4 destroyers with 4 transports (Toyama, the Survey) | 2 carrier groups; 4 more destroyers (Eighth Fleet?) to join as "additional escort"; Chitose | 555, 556; T 2 to 6, 438, 439 held |
| 13 to 15 Oct, Japanese | 6 transports | Akizuki and 7 destroyers under Takama; 11 warships (Miller) | 11th Air Fleet and floatplane fighters | T 413, 417 held |
| 13 to 15 Nov, Japanese | 11 transports | Combinedfleet.com 11 destroyers; Miller 12; Cox "an additional 12" with 7,000 troops; at sundown 14 Nov, Hough: 3 destroyers (Destroyer Division 15) with 4 transports, "four" later | Kondo ordered by Rabaul to "run interference" for Tanaka with Kirishima, 2 heavy cruisers, 2 light cruisers and a destroyer squadron; Abe's bombardment force on the 12th | 557 to 560, 566; T 15, 428 held |
| 30 Nov, Japanese | none (drums) | 8 destroyers; 6 loaded with 200 to 240 drums each, Naganami and Takanami not | none; Takanami a picket on the port bow | 549 to 554 |
| 3, 7, 11 Dec, Japanese | none (drums) | 3 Dec: number not given. 7 Dec: 10 destroyers (Hough) or 11 (Monograph 98). 11 Dec: Teruzuki stood off while the others dropped drums | none | 570 to 572; T 184, 185 held |

## Screening stations and distances (Task Force 67, Operation Plan 1-42)

From Kinkaid's plan of 27 November 1942 (rows 541 to 548). One force order, for the cruisers and destroyers of Task Force 67, written three days before Tassafaronga; no Japanese or earlier American order of the kind was found.

| Situation | Station | Source in the plan |
| --- | --- | --- |
| Task, cruisers and destroyers | "Screen supply and transport movements" (the cruisers also to attack and bombard, the destroyers to attack with torpedoes day or night) | Paragraph 3 (row 541) |
| Escorting a convoy, normally | Cruisers in screening stations about 2,000 yards from the ships of the convoy; destroyers 2,000 to 3,000 yards outside the cruisers in an anti-submarine screen (so 4,000 to 5,000 yards from the convoy) | Annex D para. 4 (row 542) |
| Air attack on a convoy | Destroyers close to stations between the cruisers and about 1,800 yards from the ships of the convoy; the convoy manoeuvred by flag hoist and the screen conforms, with a tight A.A. screen | Annex D para. 5 (row 543) |
| Cruising alone, normally | Cruisers in column at 1,000 yards or in line of divisions; destroyers in an anti-submarine screen about 4,000 yards from the cruisers | Annex D para. 1 (row 544) |
| Air attack, force alone | Destroyers close to 1,800 yards; speed 25 knots; if a cruiser drops out, the two nearest destroyers stay with it | Annex D paras. 2, 3 (row 544) |
| Cruising with transports | Cruisers at the head of each column of transports or between transports and the destroyer screen; destroyers at least 3,000 yards outside the ships screened | Annex E para. 4 (row 545) |
| Day surface action, force alone | Cruisers 1,000 yards on a line of bearing; destroyers at 6,000 yards, 30 degrees on the engaged bow; torpedoes when directed or when opportunity offers; smoke at discretion | Annex F(1) (row 546) |
| Night surface action, force alone | Cruisers 1,000 yards; destroyers 4,000 yards, 30 degrees on the engaged bow; pickets 10,000 yards toward the expected contact; destroyers to attack as soon as possible, early | Annex F(2) (row 547) |
| A covering mission at the Cactus-Ringbolt area | All or a considerable part of the force, three or more cruisers and assigned destroyers | Letter enclosure, para. 3 (row 548) |

The plan has no separate figures for day and night convoy stations, and nothing for the number of destroyers per transport. The two convoy distances differ by purpose: Annex D is a normal station, Annex E a minimum for destroyers.

## What an escort did when an enemy appeared

- **Task Force 67's plan:** says nothing for a convoy meeting a surface force; Annexes F(1) and F(2) are for the force fighting alone (rows 546, 547). For air attack the screen closes on the convoy and fights with guns (row 543).
- **American, 12 Nov:** Turner decided to withdraw the cargo ships "to avoid destruction by the enemy battleships", with destroyers; the fighting ships stayed or returned (Miller, T 435; Hough, row 561). Cox: three destroyers stayed with the convoy (row 565).
- **American, 11 Oct:** the covering force left its station to meet the Japanese run it learned of (row 567).
- **Japanese, 14 Nov:** the three escorting destroyers were ordered into the fight against the American battleships and the transports turned away north (row 559).
- **Japanese, 30 Nov:** Tanaka's order before the run was that fighting came before unloading (row 552); on sighting he ordered "Stop unloading; take battle stations" (row 553); Toyama gives the mass torpedo attack (engagement_outcomes 4027).
- **Japanese, 25 Aug:** the escort took the surviving transports back to the Shortlands (row 556).

## Runs turned back, with the reason

Held in earlier rows and not repeated: 25 Aug, ordered back (Toyama, no air support; T 2, 3, 6); 15 Sep, Turner withdrew in hope of a better opportunity (T 403); 11 Nov, Zeilin back with one destroyer after air damage (T 434); 12 Nov, cargo ships withdrew from enemy battleships (T 435); 12 Oct, a Japanese convoy recalled, no reason (T 420); 7 Dec, Monograph 98 (T 184). New: 14 Nov, the transports turned north when battleships were met (row 559); 7 Dec, Hough's account of the run (row 571); 12 Nov, Scott's group's damaged transport turned back with a destroyer (Cox, row 564).

## The Japanese destroyer runs, loaded and unloaded

Only the run of 30 November has the ships that carried drums and the ones that did not. Tanaka: of eight, six were to be loaded with 200 to 240 drums (row 549), and no drums were loaded on Naganami, his flagship, nor on Takanami, the leader of Destroyer Division 31 (row 550). Toyama: Takanami was on the port bow "as a picket ship" (row 554), with no distance given. The reserve torpedoes of the six were removed (engagement_outcomes 3351). Hough has the flagship standing off on 11 December while the others dropped drums (row 572). The two unloaded ships were not named a screen by Tanaka; Toyama calls only Takanami a picket.

## Where the sources disagree

- **Tanaka's account is not consistent on the landing groups.** Takanami and Naganami carried no drums yet lead the two landing groups he names (rows 550, 551).
- **The destroyers with Tanaka on 14 November.** Hough: three (pp. 355 to 356), later four (p. 357, row 560); combinedfleet.com 11 at the start; Miller and Cox 12.
- **Troops of the 14 November convoy.** Cox 7,000; Miller about 4,000 landed of 10,000; combinedfleet.com about 2,000 by dawn (earlier note).
- **Where Scott's force stood in October.** Hough, near Rennell Island; the Survey, the Russell Islands (rows 567, 568).
- **Who escorted whom on 12 November.** Cox counts Callaghan's cruisers as Turner's escort; Miller counts them as a separate covering group (rows 562, 564).
- **The run of 7 December.** Ten destroyers (Hough) or eleven (Monograph 98, T 184).
- **The 25 August escort.** Jintsu and three destroyers (Zimmerman), four destroyers with four transports (Toyama), with four more of the Eighth Fleet to join (Toyama, row 555).
- **The plan's two convoy distances** (Annex D, Annex E) are a normal station and a minimum; the plan does not reconcile them.

## What is missing

- **A rule for the number of escorts**, American or Japanese, and the number requested against granted for any run. Turner's operation plans of the runs, Task Force 62's orders for 8 to 12 November, and the 2nd Destroyer Squadron war diaries were not read.
- **A day-and-night difference** in screening stations, and a standing-off distance for a covering force from what it covers. Only a tactical position is given for Scott's (Rennell Island).
- **The Standard Cruising Instructions** (Pacific Fleet letter 26CL-42) that the plan adopts in part.
- **Which ships of a drum run carried drums**, except for 30 November.
- **Names:** the four Eighth Fleet destroyers of 25 August; the escorts of Betelgeuse and Alhena in September; the destroyers left with Turner on 12 November.
- **Not opened in this pass:** the Sea Power Centre study of Australian coastal convoys (it concerns Australian waters, not the Solomons runs) and a university thesis on the Tokyo Express. Nothing is taken from either. Toyama's four destroyers and four transports and his "no air support" were already held (rows 5, 6, 3), and the mass torpedo attack was already engagement_outcomes 4027.
