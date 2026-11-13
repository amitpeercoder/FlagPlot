# Rates of gun fire achieved in the night surface actions of 1942

Research for the build session's request of 13 November 2026: what ships' guns actually fired, and over how many minutes, in the night actions of August to November 1942, against the designed rates the derivation uses. This page says what was read, what the rows hold, the cases where a source gives shells and minutes together, what the sources say governed the rate, and what was not found. It settles nothing: every row is `unconfirmed`.

## What was read

| Action | Sources read |
| --- | --- |
| Savo Island, 8 to 9 August | Chokai's own action report and Kako's own action report (Japanese archive, handwritten, page images); the Naval War College analysis (Bates, 1950); the ONI combat narrative |
| Cape Esperance, 11 to 12 October | Helena's own action report (page images); the October Battle Experience bulletin (reprints the ships' logs and reports); the ONI combat narrative |
| Guadalcanal, 12 to 13 November | The November Battle Experience bulletin (reprints the reports of Helena, Portland, San Francisco, Sterett, O'Bannon, Fletcher, Aaron Ward, Monssen, Cushing and others); Sterett's own report; transcriptions of O'Bannon's and Laffey's reports; four pages of Helena's report; the 4th Destroyer Squadron's action report (Japanese archive); the ONI combat narrative; Lundgren and Sposato's analysis |
| Guadalcanal, 14 to 15 November | South Dakota's own action report (page images); Washington's report as reprinted in the bulletin; Atago's own action report (Japanese archive, ammunition page only); the ONI combat narrative; Lundgren's analyses and a NavWeaps article |
| Tassafaronga, 30 November | The Task Force 67 report with Tisdale's report; the reports of Minneapolis, Honolulu, Drayton and Fletcher (page images); the ONI combat narrative; Tanaka's memoir and a Maury officer's account as excerpted on the web |

## What the rows hold

`engagement_outcomes.csv` rows 2291 to 3151 and `engagement_events.csv` rows 734 to 1096. Three engagements were added: `guadalcanal-13-nov`, `guadalcanal-15-nov`, `tassafaronga`.

| Measure | Meaning |
| --- | --- |
| `main_rounds_fired`, `secondary_rounds_fired`, `starshell_rounds_fired` | A ship's total for the action, as the source gives it. A destroyer's 5-inch is entered as her main battery |
| `rounds_fired_in_window` | A count tied to a stated part of the action or span of time |
| `rounds_fired_all_calibres` | A force total over every calibre |
| `salvos_fired` | A count of salvos, for the action or a stated part |
| `firing_duration_min` | A stated length of firing. Often a span with pauses, not minutes of continuous fire: the note says which |
| `salvo_interval_s`, `rounds_per_gun_per_minute_stated` | An interval or a rate in the source's own figures, never computed here |
| `guns_firing` | Guns in the battery, or the number that fired or could bear |
| `fire_rate_note` | The source's words on the manner of fire; the value is its own phrase |
| `fire_limit_note` | What stopped, slowed or governed fire; the value is one of fifteen categories |
| `fire_doctrine_note` | Orders, plans and recommendations on how fire should be delivered |

Event kinds added beside `open_fire` and `cease_fire`: `check_fire`, `resume_fire`, `salvo` (a numbered or counted salvo with its time) and `order`. The battery is in the `means` column of an event and at the start of the note of an outcome.

How to read a row:

- The `as_stated` column is the source's passage. For text sources it was checked by program as an exact span of the source text (line breaks closed up); machine-reading errors are left as they are. For scanned pages it is a reader's transcription of typed or handwritten text; the rows this page rests on were checked by eye.
- The note says whether the passage is the ship's own report or log, a compiler's or analyst's statement, or a memoir.
- An event whose clock time comes from the log's time column beside the quoted entry, and not from the quoted words, is `low`.
- Japanese times are Tokyo time as written. American times are as each source prints them; the sources disagree by up to three minutes on the same event (the cease-fire order of 13 November is 0151, 0153, 0154 and 0155 in four reports), and both readings are kept.
- Lundgren's salvo times for Kirishima, and the Washington salvos he marks as undocumented, rest on a reload cycle he assumes (30 seconds). They are his reconstruction and were not entered: 52 records left out.

## Cases with shells and minutes together

The figures are the rows'. Any rate worked out from them here is arithmetic of mine and is not a row.

| Ship and gun | What the source gives | Rows |
| --- | --- | --- |
| Chokai, 20 cm, Savo Island | 302 rounds in 38 salvos; twenty minutes from opening gunfire to the end, with fire held between targets. Salvos by target: 5, 8, 9, 4, then 7 in a second firing | outcomes 3130 to 3137; events 1091 to 1096 |
| Kako, 20 cm, Savo Island | 192 rounds; the mountings "fired over thirty minutes" without a breakdown. Full salvo fire "developed the utmost rate" because the enemy was almost stopped | 3138, 3140, 3141 |
| Kako, 12 cm high-angle, Savo Island | 130 rounds. From the 25th salvo supply fell behind: the ready boxes held 24 rounds, and after they were fired the rate of fire fell | 3139, 3142 |
| Asagumo, destroyer guns, 13 November | 88 rounds in about six minutes at 3,000 to 4,000 metres | 3146, 3147 |
| Atago, 20 cm, 15 November | 55 armour-piercing and 6 common rounds; no firing times read firmly. A NavWeaps article has Takao firing 36 rounds between 0102 and 0114 | 3143 to 3145, 2502 |
| Helena, 6-inch, Cape Esperance | 580 rounds, "in continuous fire": 2346 to about 2348.5, 2349 for about two minutes more, then about 0004 (the log says 0010) to 0012. The 5-inch fired about 400 | 2974 to 2982; events 1031 to 1038 |
| Helena, 6-inch, 13 November | "Rapid continuous fire" for about two minutes from 0148; about 125 rounds in one and a half to two minutes from 0204 in full automatic; about 60 rounds in about one minute "at a very high rate of fire" from 0215. The ONI narrative gives 175 rounds in two minutes for the first firing. The bulletin's table gives her fifteen 6-inch guns | 2577 to 2590, 2625 |
| Honolulu, 6-inch, Tassafaronga | 244 rounds; opened about 2325 "with two salvos, then went to continuous rapid fire, checking as necessary when targets were lost"; checked at 2331. Her war diary extract says ceased 2330 | 2906 to 2919; events 1013 to 1021 |
| Boise, Cape Esperance | "Effective continuous fire"; two minutes on a destroyer in radar control; about 120 rounds from the secondary battery. No main-battery count found | 2517 to 2523 |
| Minneapolis, 8-inch, Tassafaronga | 95 rounds in eleven salvos, opened 2320; the torpedoes hit at 2327 (2324 in the radar officer's chronology); fire checked after the fourth and eighth salvos to shift target. The task force report has her firing until power failed about 2333 | 2927 to 2942; events 1009, 1022 |
| Salt Lake City, 8-inch, Cape Esperance | Already on file: 296 rounds in 30 salvos, 2347 to 0016 with pauses (rows 1638, 1704) | |
| Washington, 16-inch, 15 November | 42 rounds from 0016 to 0019; "rapid, on one turret ready light, for about 2 minutes 39 seconds, firing about 39 rounds"; later 36 rounds in 2 minutes 45 seconds; 75 rounds from 0100 to 0107. The bulletin prints "overall SPG PM was 1.30", unexplained on the page | 2642 to 2668 |
| Washington, 5-inch, 15 November | 133 rounds from 0025 to 0034; 107 (another passage: 120; Lundgren: 227) from 0100 to 0107. One group fired "rapid continuous fire"; another "shifted to salvo fire, 4 second interval, to facilitate spotting" | 2643 to 2661 |
| South Dakota, 16-inch, 15 November | 115 rounds. Her log times salvos 2 to 8 at 0019.15, 0020-10, 0020-42, 0021-40, 0022-15, 0023-00 and 0024-18 | 2863; events from 985 (her report) and from Lundgren's table |
| Fletcher, 5-inch, Tassafaronga | 60 rounds, commenced 2323, ceased 2325 when the radar lost the target | 2971; events 1029, 1030 |
| Drayton, 5-inch, Tassafaronga | 141 rounds (98 common, 43 illuminating) in 41 salvos; about twenty rounds in about two minutes between two checks | 2955 to 2969 |
| Sterett, 5-inch, 13 November | 90 rounds for the action; 13 salvos between 0149 and the check at 0151 | 3113 to 3118 |
| Barton, 5-inch, 13 November | About 60 rounds from her two forward guns and about 10 a gun aft, in about seven minutes of continuous firing (as quoted by Lundgren and Sposato) | 2774 to 2794 |
| Walke, 5-inch, 15 November | "Rapid fire" for two minutes at 11,000 yards, then checks to shift target | 2688; events 851 to 853 |
| Wilson, 5-inch, Savo Island | 212 rounds; shifted to continuous fire when she could not spot | 3025, 3073, 3074 |
| Tenryu, Yubari, Yunagi, Savo Island | Yubari 98 rounds of 5.5-inch; Tenryu's figure is lost in the machine text; Tenryu fired four guns at Wilson for about a minute; Yunagi fired on Jarvis for about five minutes, no count | 3008, 3051, 3078 |

Two things stand out, as leads for the architect and not as findings:

- The cases split. Where a ship fired at one target without interruption, the sources' own figures are high: Helena's 6-inch (about 60 rounds in about a minute; about 125 in under two) and Fletcher's 5-inch (60 rounds in two minutes). Where a ship fired by salvo and spotted, they are low: Chokai's 302 rounds in 38 salvos over twenty minutes, Minneapolis's 95 rounds in eleven salvos, Washington's own "1.30".
- The totals for an action are small because the firing was short and broken, not only because the rate while firing was low. Helena's 580 rounds at Cape Esperance came in three firings of about two minutes each.

## What the sources say governed the rate

Counts of `fire_limit_note` rows by category, 336 in all:

| Category | Rows | Typical statement |
| --- | --- | --- |
| gun-casualty | 52 | Mounts and turrets out of action from hits, misfires, loading faults |
| radar | 38 | Fire-control radar lost the target, was shaken out by own gunfire, or could not show splashes |
| flash-or-smoke | 34 | Own gun flash blinded spotters, directors and 5-inch loaders; smoke and splashes hid the target |
| identification, fear-of-hitting-friends, cease-fire-order | 57 | Fire checked or withheld while a target was made out; the cease-fire orders of 11 October and 13 November; range fouled by a friend |
| target-lost | 20 | Fire checked when the target vanished from radar or sight |
| power-failure | 19 | South Dakota, Minneapolis, Vincennes, Astoria |
| train-limit | 17 | Turrets at the stops: Kako after one salvo, Washington's and South Dakota's forward groups |
| readiness | 18 | Guns not manned or not loaded when the action opened |
| illumination, searchlight | 16 | Star shells that failed; Chokai lit her searchlights only while firing |
| spotting | 10 | Waiting on or unable to see the fall of shot; Washington's 4-second salvo fire "to facilitate spotting" |
| ammunition-supply | 9 | Kako's high-angle guns; Pensacola's ready star shell; Kirishima's shells lined up at the hoists |
| other | 46 | |

Doctrine rows bear on the same question. Scott's memorandum before Cape Esperance told the heavy cruisers to use "continuous fire" against small ships at short range "instead of full gun salvos with long intervals". The bulletin credits "our light cruisers which used rapid continuous fire". The Japanese reports and the American observers both describe Japanese heavy-gun fire as deliberate ("not hurried", "slow and deliberately controlled") and Japanese destroyer fire as rapid in the first minutes.

## Disputes kept as rival rows

- Washington's 5-inch rounds from 0100 to 0107: 107, 120 and 227.
- Helena's rounds at Cape Esperance: 580 and about 400 in her report; 560 and 340 in a veterans' pamphlet whose columns are matched to actions by their order only.
- Kirishima's shell counts: the same figures (27 and 22) are attached to different shell types and to different nights in Lundgren's two papers.
- Minneapolis's opening: 2320, about 2320 and a half, 2321, 2323. Vincennes' second salvo: two guns in the narrative's text, nine in its table.
- Drayton: 141 rounds of which 98 common in the gunnery officer's report; "98 rounds of AA common" in the commanding officer's.

## Not found

Listed in `gaps.csv`. The ones that matter most for a rule:

- No case with shells and minutes for the Japanese 14 cm (5.5-inch) gun. Tenryu's round figure is unreadable in the text read and the 18th Cruiser Division's own report was not re-read for it.
- One Japanese destroyer case only (Asagumo). The 4th Destroyer Squadron's report has no table of rounds by ship; Yudachi's own report has figures that could not be read firmly; nothing was found for the 2nd Destroyer Squadron at Tassafaronga.
- No own action reports for Boise, San Francisco or Salt Lake City at Cape Esperance, nor Washington's own report: they are read only as reprinted or tabulated.
- No rate in rounds a gun a minute in any report except Washington's unexplained "1.30" and a later analyst's 1.56 and 1.54.
- No Bureau of Ordnance or fleet document on rates achieved in action, and no Battle Experience bulletin for Savo Island or Tassafaronga.
