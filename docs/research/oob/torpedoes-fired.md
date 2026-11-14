# Torpedo tubes that bear on a broadside, and torpedoes fired in the night actions of 1942

Research for the step 2 coordinator's commission of 13 November 2026. Two questions: how many of a class's tubes can fire on one side, and how many torpedoes each ship fired in the night actions of August to November 1942, with the reasons tubes were not fired. This page says what the rows hold, what was found and what was not. It settles nothing.

The reading was done by Sonnet agents: readers quoting from saved pages and scans, clerks keying the records, an auditor checking each staged row against its own quote, and a blind second reading of every scanned page a figure rests on. Every quote from a text source was checked by program as an exact span of the saved text.

## What the rows hold

| File | Rows | What |
| --- | --- | --- |
| `attributes.csv` | three new attributes for a ship class | `torpedo_tubes`, `torpedo_tubes_per_broadside`, `torpedo_mount_position` |
| `ship_class_facts.csv` | 1176 to 1353 | One row for each thing a source's passage states about a class: tubes in all, tubes that bear on one side, where the mounts stand |
| `engagement_outcomes.csv` | 3152 to 3514 | Torpedoes fired, not fired and why, tubes aboard, hits claimed and received, orders and doctrine |
| `engagement_events.csv` | 1097 to 1132 | `torpedo_launch` events where the passage itself prints the time |

Class rows are dated 1 August to 31 October 1942. A passage that is undated or speaks of the class as built is entered only where nothing in it shows a change before August 1942, and its note says so. A broadside figure is `stated` only where the passage prints the number; where it follows from mount positions the passage states (a centreline mount bears on either side, a wing mount on its own side) the basis is `inferred`. Standing follows the checker's rule mechanically: `preferred` where a value is supported, `unconfirmed` where it is not, and every row of a class `disputed` where sources give different values.

Measures for an engagement:

| Measure | Meaning |
| --- | --- |
| `torpedoes_fired` | A count, for one salvo or for the action; the note says which, with side, mount, target and range where the passage gives them |
| `torpedoes_fired_note` | The ship fired torpedoes; no count in the passage |
| `torpedoes_not_fired` | A count of torpedoes that could not be fired |
| `torpedoes_not_fired_note` | The ship fired none, or held fire; the value is the reason, one of fourteen categories |
| `torpedo_tubes_aboard` | Tubes or mounts a battle source gives for a ship |
| `torpedo_tubes_bearing` | Tubes a ship's own report says she could fire at a target (one row: Chokai) |
| `torpedo_hits_claimed`, `torpedo_hits_received` | As claimed or credited; as taken |
| `torpedo_doctrine_note` | Orders, permission and doctrine on firing |

## Tubes that bear on one side, by class

| Class | In all | On one side | Mounts | Standing of the broadside rows |
| --- | --- | --- | --- | --- |
| Bagley, Gridley | 16 | 8 | four quadruple wing mounts, two a side | preferred |
| Mahan | 12 | 8 | one centreline and one on each side; one source prints "8-tube broadside" | preferred |
| Farragut, Porter | 8 | 8 | two centreline quadruple mounts | preferred |
| Sims | 8 | 8 | two centreline mounts after 1939; early units had 12 with wing mounts | preferred; position disputed |
| Benson, Gleaves | 10 | 10 | two centreline quintuple mounts as designed | unconfirmed; see gaps |
| Benham | 16 | 8 | four quadruple wing mounts as built | unconfirmed; see gaps |
| Kent class (Canberra, Australia), Hobart | 8 | 4 | one quadruple mount each side | unconfirmed |
| American heavy cruisers (Pensacola, Northampton, Portland, New Orleans) | 0 | 0 | built without, or removed before the war (the date differs between sources) | unconfirmed |
| Asashio, Kagero, Yugumo, Shiratsuyu | 8 | 8 | two centreline quadruple mounts | unconfirmed |
| Akizuki | 4 | 4 | one centreline quadruple mount | unconfirmed |
| Tenryu | 6 | 6 | two centreline triple mounts | unconfirmed |
| Yubari | 4 | 4 | two centreline twin mounts | unconfirmed |
| Aoba, Furutaka | 8 | 4 | two quadruple mounts, one each side of the catapult, after their rebuilds | unconfirmed |
| Takao class, Chokai and Maya only | 8 | 4 | four twin mounts | unconfirmed |
| Nagato | 0 | 0 | tubes removed in the reconstruction of 1934 to 1936 | preferred |

The rows for tubes in all are `disputed` for the Aoba and Takao classes, because sources that describe the fit as built (12 for Aoba) or the other pair of ships (16 for Atago and Takao) sit beside the 1942 figures. The Aoba and Furutaka figures assume each beam mount fires only to its own side; the passages say "one on each side" and no more. The action record agrees with them: at Savo Island every Japanese heavy cruiser fired her port and starboard tubes as separate salvos of four or fewer (below), and Chokai's own report says of the Type 93 torpedo that "four firing lines are too few" (outcome row with measure `torpedo_tubes_bearing`).

Single ships, read but not entered as rows, because a new fact row for a ship would make the build session's generated experience files stale. They are leads:

- HMAS Australia: "1940 when torpedo tubes were removed and AA armament increased" (naval-history.net). Canberra and Hobart are listed with two quadruple mounts, undated.
- Jintsu: her four twin mounts replaced by two quadruple mounts, one a side, from March to May 1941 (Wikipedia; combinedfleet has the mounts but not the sides). Abukuma alone of the Nagara class received two quadruple mounts in 1941.
- Takao and Atago: four twin mounts replaced by four quadruple mounts in the 1939 refit (Japanese Wikipedia).
- Juneau: a photograph caption of 19 March 1942 shows a port quadruple mount.
- Noa: two triple mounts removed in 1940.
- Kirishima: "The remaining torpedo tubes were also removed" in her second reconstruction (Wikipedia). No class row was written for the Kongo class.

## Torpedoes fired, by ship

Counts are as the sources print them; rival figures are separate rows.

**Savo Island.** Japanese counts are from the Naval War College analysis and, for Chokai and Kako, from the ships' own action reports read twice.

| Ship | Fired | Source |
| --- | --- | --- |
| Chokai | 4 to port at 2337 Tokyo time (aiming range 4500), 4 to starboard at 2347 (3500); 8 in the action | her report; the analysis has the same salvos at 0138 and 0148 |
| Kako | 4 at 2347 (3500), 4 to port at 2359 or 0001, 2 at 0005; 10 in the action, 6 remaining | her report; the analysis has 3 to starboard, 4 to port and 2 more |
| Furutaka | 4 to port, then 1 and 3 to starboard; 8 in all | analysis |
| Kinugasa | 4 to port; 2 in another passage of the same book | analysis |
| Aoba | 3 to starboard, 1 to port | analysis |
| Tenryu | 6 | analysis |
| Yubari | 4, the analyst's inference | not entered as a count |
| Bagley | 4 from her port mounts | analysis and ONI narrative |
| Ralph Talbot | 1, then 4 in all | analysis and ONI narrative |
| Canberra | none (analysis); possibly 2 from port (ONI narrative) | rival rows |
| Helm, Wilson, Patterson, Blue | none | see reasons below |

**Cape Esperance.** No count of torpedoes fired was found for any American ship. Buchanan and Duncan made torpedo attacks and two hits are credited to each. The Japanese counts already on file (Aoba 2, Kinugasa 4, from a staff paper) remain the only ones.

*Note, Nov 14, 2026 (audit of the documents; nothing above is changed).* The paragraph above on Cape Esperance disagrees with `cape-esperance.md` (its table of rounds fired) and with the rows. `data/oob/engagement_outcomes.csv` holds counts for American ships: Duncan 2 (row 1643, high, preferred, the Navy's ship history), Buchanan 5 (row 1719, low, unconfirmed, a magazine article that cites a book not read) and McCalla 0 (row 1642, low, unconfirmed). It also holds two Japanese counts for Kinugasa: 4 (row 2282, medium, a staff table, the one named above) and 16 (row 1654, low, unconfirmed, Japanese Wikipedia alone). So "no count was found for any American ship" is not what the rows say; those rows came from the Nov 12 pass on Cape Esperance, which this page did not use. `cape-esperance.md` follows rows 1643, 1654 and 1719; the rows are current, and where this page and `cape-esperance.md` differ, `cape-esperance.md` and the rows are the ones to read. The two Kinugasa counts remain a dispute; this page's 4 is the better-sourced of the two.

**12 to 13 November.** American counts are from the ships' reports as reprinted in the Battle Experience bulletin, and from Sterett's and O'Bannon's own reports. Japanese counts are from the 4th Destroyer Squadron's report, read twice.

| Ship | Fired |
| --- | --- |
| Sterett | 4 at about 0205 at a battleship, 2 at about 0220 at a destroyer; 6 in all |
| Monssen | 5 at about 0216, 5 more about three minutes later |
| Cushing | 6 by local control at a battleship |
| Fletcher | two half salvos of 5 |
| Barton | 5 (ONI narrative) |
| O'Bannon | 3 in her own report; 4 in the bulletin's summary |
| Laffey | 2, which did not arm |
| Atlanta, Juneau, Aaron Ward | no statement found |
| Murasame | 7 |
| Yudachi | 8 |
| Asagumo | 8, both readers unsure of the figure |

**14 to 15 November.** Atago 19 in the action (8 to starboard, 8 to port, 3 more; her log's times were not read firmly). Asagumo 4. Takao 20, Ayanami 6 and "35 to 40" from Nagara's group come from later analyses only. Gwin fired one by a short circuit; no American destroyer reports a deliberate launch.

**Tassafaronga.** Fletcher 10 at 2322 in two half salvos; Perkins 8; Drayton 2 at 2320 and 4 at about 2337; Maury, Lamson and Lardner none. A second reading of the scanned reports agrees on every count. Japanese counts are from Tanaka's memoir and a later narrative only.

## Why tubes were not fired

Fifty-seven `torpedoes_not_fired_note` rows and nine `torpedoes_not_fired` rows. The reasons, as the sources give them:

| Ship | Reason |
| --- | --- |
| Patterson, Savo Island | the captain ordered torpedoes fired; the torpedo officer did not hear the order |
| Wilson, Savo Island | no reason stated; the analysis says neither she nor Helm fired |
| Helm, Savo Island | her one target proved to be friendly |
| Bagley, Savo Island | starboard tubes: primers were not in before the ship passed the firing bearing; she swung and fired the port tubes |
| Ralph Talbot, Savo Island | a tube hit, a faulty primer, and fire control to the port tubes cut |
| Farenholt, Cape Esperance | torpedo mount jammed by fragments at 2350 |
| Cushing, 13 November | held fire on destroyers crossing ahead when ordered back to the column's course |
| Sterett, 13 November | at 0149 the target was too fine on the bow and too vague; later two torpedoes could not be fired from a damaged mount |
| Maury, Lamson, Lardner, Tassafaronga | no search radar, target not identified |
| Drayton, Tassafaronga | her plot showed the target stopped, so she fired two only |
| Fletcher's division, Tassafaronga | permission to fire was asked at 2316 and given about four minutes later, after the commander queried the range |
| Asagumo, Teruzuki, Samidare | did not fire (no reason; Samidare's target sank first) |

Doctrine rows (65) carry the orders: Scott's instruction before Cape Esperance that destroyers fire torpedoes at large ships, the cease-fire and course orders of 13 November, and the permission exchange at Tassafaronga with its times and ranges.

## Checks

- Text quotes: 575 of 575 from the first pass, and 15 of 15 from a follow-up, matched the saved source text.
- Audit: of 290 staged rows carrying a count, a reason or a claim, 259 passed, 3 were corrected and 28 were rejected (wakes sighted, torpedoes seen or remaining entered as torpedoes fired).
- Scans: the Tassafaronga reports and the Japanese reports were each read twice by readers who did not see the other's reading. A Japanese figure is `medium` only where both readings agree and neither doubted it.
- 19 records marked by a later analyst as his own inference were not entered.

## Not found

In `gaps.csv`. The ones that bear on a rule: tubes a side for Atago and Takao, Myoko, Mogami, Tone and Nagara; mount positions for Fubuki, Mutsuki and Kamikaze; the 1942 fit of Benham, Benson and Gleaves; any count of torpedoes fired at Cape Esperance by an American ship; and Japanese counts for Tassafaronga and for most ships of 12 to 13 November from a Japanese record.

## The Japanese group's readiness at Cape Esperance

Nothing new was read for this. What the rows already hold (from pull requests 153 and 158): battle was ordered at 2143 on sighting, the first enemy fire came at 2146, Furutaka's high-angle guns opened at 2148 and her main battery at 2152, Kinugasa at 2154; Aoba fired 7 main-battery rounds, Furutaka 20, Kinugasa 80. What the guns were loaded with, where they were trained and any recognition signal are still gaps.
