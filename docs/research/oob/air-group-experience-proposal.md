**Ruled by the architect on Nov 13, 2026: the values wait.** By the spec's condition 4 (`docs/step2-spec.md`, "Experience"), the facts are too thin to rest them on: the Japanese lift rests on one service-wide statement of hours, the US Navy sits at 2 only because no row exists, and the Marine squadrons' two questions are one fact. Air-group experience waits until research closes the gaps in section 4. No value goes into any data file.

# Air-group experience values: a proposal

**Proposal. Nothing reads it.**

Status: proposal, for the designer to see. Date: 2026-11-13. Decision 0064, stage 2, item 9: the architect derives the air groups' values and shows them to the designer before any rule reads them. This page changes no code, no state, no `oob-derive` rule, no derivation file, no `rules.toml` and no scenario. If the designer accepts it, the questions and thresholds below are fixed in a derivation file, and the values are fixed before any batch with the rule on is looked at (0064, item 3).

Sources of the rows: `data/oob/air_unit_facts.csv` ("air_unit_facts" below), `data/oob/training_facts.csv` ("training_facts"), `data/oob/air_units.csv` for the unit entities. The research behind them is `docs/research/oob/experience.md`. Every row id below was read in the file.

## 1. The rule, as decision 0064 sets it

- A value is a whole number 0 to 4. 2 is a group trained to its service's standard, with nothing in the record for or against it.
- Each yes-or-no question a row answers moves the value by 1 from 2, once, however many rows answer it. A question no row answers moves nothing: a missing fact is a gap, not a guess.
- Words such as "veteran", "well trained" or "very inexperienced" answer no question. Nothing is worked back from the result of the battle being modelled: no loss, hit or kill of the battle answers any question.
- The inputs 0064 names: the date the unit first saw action, and the service's flying hours on joining, where a row gives them.

Which rows may answer. Stage 1 (ships) lets only a `preferred` row of `medium` or `high` confidence answer a question, and rows that are not preferred are printed as not read. I keep that rule here, and show the effect of relaxing it in a second column of the table (section 3). `data/oob/README.md` and `data-rules.md` say a lone weak row is `unconfirmed` and a rule may read it only if the rule says so. Nearly all the unit-level rows for the air groups are one Wikipedia sentence, `low`, `unconfirmed`, so this choice decides most of the table. It is the designer's call, and the table shows both.

## 2. The proposed questions

The thresholds are proposed. They are fixed in the derivation file, not here.

**A group made of several units** (scenario 6, group 1: the 4th, Misawa and Kisarazu Air Groups) answers a question only if every unit in it answers the same way. Otherwise it is a gap.

**A first-action date is void for a group** if a row says the group's crews at the scenario are not the crews of that date (for example, the unit was rebuilt). This is why VT-8's Midway date (air_unit_facts 407, 409), VMSB-231's earlier history (air_unit_facts 421) and Kisarazu's 1937 date (air_unit_facts 359, 360, 361) do not answer.

"Saw action" means a fight with the enemy: an interception attempt, escort duty, or being bombed on the ground is not action.

### Q1. `seasoned`, up 1

> Does a row say the group first saw action before 1 January 1942?

- Direction: +1.
- Why this date: the first-action dates the rows give fall in three clusters: 1937 (Kisarazu), 7 and 8 December 1941 (the carrier groups, Ryujo, Tainan), and from 20 February 1942 on (4th Air Group, then 18 April, 4 June, 7 August and later). No row lies between 9 December 1941 and 19 February 1942. Any date in that gap gives the same answers, so the exact date decides nothing. 1 January is the first round date inside it.

### Q2. `untried`, down 1

> Does a row say the group had no action before 7 August 1942, or date its first action on or after that day?

- Direction: -1.
- Why this date: 7 August is the day of the Guadalcanal landings. The latest first-action dates before it are 18 April (6th Air Group, which row 348 itself calls "an interception attempt, not a fight") and 4 June 1942 (VB-3, VT-8, the Midway squadrons). The earliest on or after it are 7 August (2nd Air Group, VF-5) and 8 August. Any date from 5 June to 7 August gives the same answers. 7 August is the event the scenarios are built on.
- A group whose first action falls between Q1's date and Q2's date (the Midway squadrons, the 4th Air Group, the 6th Air Group) answers neither question, and stays at 2 on this leg.

### Q3. `hours_high`, up 1

> Does a row give the service's pilots' average flying hours (on joining a unit, or in a first-line unit) as at or above 400, for a period that includes the group's first action or ends no more than three months before it?

- Direction: +1.
- The row must be of the service's pilots (`ijn-aviators`, `ijn-carrier-aviators`, `ijn-land-based-aviators`, `usmc-aviators`, `usn-aviators`, `usaaf-fighter-pilots`), and the period is read from the row's own dates and, where they disagree with the statement, from the statement (see the suspicious rows, section 6).
- Why that link: it ties the hours to the pilots who were in the group when it first fought. A figure for December 1941 is not a figure for a pilot who joined in July 1942, and the rows have none for him (a gap, section 4).

### Q4. `hours_low`, down 1

> The same, as below 400.

- Direction: -1.
- Why 400: the rows give 275 (training_facts 49, 413 in air_unit_facts) for the Marine squadrons, and 500 to 800 and above for the Japanese navy (training_facts 17, 5, 1, 2, 25). No row lies between 276 and 499, so any threshold in that gap gives the same answers. 400 is the middle round number. I say plainly that the threshold sits in the empty gap, so it decides nothing: what separates the groups is what the rows say, not where the line is drawn.
- A dispute inside the rows (training_facts 5, 1, 2 and 25 are all `disputed`) is settled if every figure falls on the same side of the threshold. They do: 650, 700, 800, 800, and the range 500 to 800 (training_facts 17) are all at or above 400. A figure that straddles the threshold would be a gap.

### Questions considered and not proposed

- **"Formed less than N months before."** Formed dates exist for the Marine squadrons (air_unit_facts 410, 415) and the Rabaul groups (air_unit_facts 342, 347, 350, 354, 362), but all but 410 are `low` `unconfirmed` Wikipedia, none for any US Navy squadron or Japanese carrier group (`docs/research/oob/experience.md` section 2 says "not found"), and any threshold I could write would be chosen to fit the dates. Not proposed.
- **"Assembled from the remnants of other groups."** The only row is Captain Davis's report on the Enterprise Air Group (air_unit_facts 394 to 397, `high`, `preferred`): "formed shortly before sailing of hastily collected remnants of squadrons from three previous air groups". It gives no date, so it cannot be put against a time window (stage 1 treats an undated "recently" as answering nothing). The same row says "the majority of the pilots were very inexperienced", which is a judgement written after the battle of 24 August and answers no question. See section 4 for the research that would make it a question.
- **Hours in a flying course (training_facts 12, 13, 46, 52, 54 to 57).** These are syllabus hours or one cadet's recollection, `low`, `unconfirmed` or `disputed`, from a website or a memoir, for 1937 to 1943. They are not hours on joining a unit. If the designer admitted the US Navy's 207 course hours (training_facts 56) plus 75 for carrier qualification (training_facts 52), about 280, every US Navy group would answer Q4 and fall to 1. I do not propose it: it is a nominal 1939 syllabus, not a statement of what a 1942 pilot had flown, and training_facts 59 (the 1942 pattern) gives no hours.

## 3. The table

Scenario 5 starts 24 August 1942, scenario 6 on 21 August 1942 (`engagements.csv` 1000 and 1002). The thresholds are fixed dates, so a group has the same answers in both scenarios.

How to read a cell: Y = the question is answered yes, N = answered no (the rows say the opposite), gap = no row answers it under the rule. "Strict" admits only `preferred`, `medium` or `high` rows. "Low rows too" also admits `low` and `unconfirmed` rows from one source. Value = 2 + each yes of Q1 and Q3 - each yes of Q2 and Q4.

### Scenario 5: Eastern Solomons

| Group | Name | Unit entity | Q1 first action before 1 Jan 1942 | Q2 no action before 7 Aug 1942 | Q3 hours at or above 400 | Q4 hours below 400 | Value, strict | Value, low rows too | Confidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | VF-6 | vf-6 (1024) | gap | gap | gap | gap | 2 | 2 | Gap on every question. air_unit_facts 399 shows it in action 7-8 Aug, which is not a first-action date. 394 to 398 are words |
| 2 | VB-6 | vb-6 (1022) | gap | gap | gap | gap | 2 | 2 | Gap. air_unit_facts 395 is words |
| 3 | VS-5 | vs-5 (1035) | gap | gap | gap | gap | 2 | 2 | Gap. air_unit_facts 396 is words |
| 4 | VT-3 | vt-3 (1038) | gap | gap | gap | gap | 2 | 2 | Gap. air_unit_facts 397 is words |
| 5 | VF-5 | vf-5 (1023) | gap | gap | gap | gap | 2 | 2 | air_unit_facts 401 (7 Aug, `low`, `unconfirmed`) says in its note that the action "may be earlier": a lower bound, so Q2 cannot be answered even with low rows |
| 6 | VB-3 | vb-3 (1021) | N (low rows only) | N (low rows only) | gap | gap | 2 | 2 | air_unit_facts 404, 405 (4 June 1942, Midway, `low`, `unconfirmed`): falls between the two dates, so neither question moves it |
| 7 | VS-3 | vs-3 (1034) | gap | gap | gap | gap | 2 | 2 | No first-action row at all |
| 8 | VT-8 | vt-8 (1040) | void | void | gap | gap | 2 | 2 | air_unit_facts 407, 408 give 4 June 1942, but 409 says the August squadron was rebuilt on a detachment, so the date is void |
| 9 | VMF-223 | vmf-223 (1026) | N | Y: air_unit_facts 411, 412 (first action 21 Aug; "None" before) | gap | Y: training_facts 49 and air_unit_facts 413 (275 hours) | 0 | 0 | Firm on Q2 (rows `high`, `preferred`, two sources agree); medium on Q4 (`medium`, `preferred`, one source, hours in trainers) |
| 10 | VMSB-232 | vmsb-232 (1030) | N | Y: air_unit_facts 423, 424 (no combat flying before 23 Aug) | gap | Y by training_facts 49 (`medium`, `preferred`; the sentence names both squadrons) | 0 | 0 | Firm on Q2 (`high`, `preferred`). Weak on Q4: training_facts 49's detail and air_unit_facts 425 are about VMF-223, and the unit's own row 426 is `low`, `unconfirmed`. Without Q4 the value is 1 |
| 11 | Shokaku fighter unit | shokaku-fighter-unit (1018) | gap strict; Y low rows: air_unit_facts 366 (7 Dec 1941, the carrier's air group as a whole) | gap | Y: training_facts 17 (`medium`, `preferred`, range 500 to 800, 1941); agree 5, 1, 2, 25 (all `disputed`, all at or above 650) | gap | 3 | 4 | Medium on Q3, low on Q1. See the note on crews after Coral Sea below the table |
| 12 | Shokaku bomber unit | shokaku-bomber-unit (1017) | as 11: air_unit_facts 370 | gap | Y as 11 | gap | 3 | 4 | As 11 |
| 13 | Shokaku attack unit | shokaku-attack-unit (1016) | as 11: air_unit_facts 374 | gap | Y as 11 | gap | 3 | 4 | As 11 |
| 14 | Zuikaku fighter unit | zuikaku-fighter-unit (1044) | as 11: air_unit_facts 378 | gap | Y as 11 | gap | 3 | 4 | As 11 |
| 15 | Zuikaku bomber unit | zuikaku-bomber-unit (1043) | as 11: air_unit_facts 382 | gap | Y as 11 | gap | 3 | 4 | As 11 |
| 16 | Zuikaku attack unit | zuikaku-attack-unit (1042) | as 11: air_unit_facts 386 | gap | Y as 11 | gap | 3 | 4 | As 11 |
| 17 | Chikuma floatplanes | chikuma-air-unit (1051) | gap | gap | gap | gap | 2 | 2 | No first-action or hours row. The hours rows are for fighter and carrier pilots, not cruiser floatplane crews |
| 18 | Ryujo fighter unit | ryujo-fighter-unit (1015) | gap strict; Y low rows: air_unit_facts 390 (8 Dec 1941) | gap | Y: training_facts 17 as above | gap | 3 | 4 | Medium on Q3, low on Q1 |
| 19 | Ryujo attack unit | ryujo-attack-unit (1014) | gap strict; Y low rows: air_unit_facts 392 (8 Dec 1941) | gap | Y: training_facts 17 as above | gap | 3 | 4 | As 18 |
| 20 | Tone floatplanes | tone-air-unit (1050) | gap | gap | gap | gap | 2 | 2 | As 17 |

Note on the Japanese carrier groups (11 to 16, 18, 19). Q3 reads a figure for December 1941, tied to the group by its first action on 7 or 8 December 1941. That assumes the pilots of 24 August 1942 are the pilots of December 1941. The rows do not say so. air_unit_facts 368 (and the copies 372, 376, 380, 384, 388) say Japan lost ninety aircrew killed at Coral Sea, not split by ship, and "whether losses were made good before 24 Aug is not stated". The rows do not say the groups' crews were replaced, so I do not void the date, but the answer is only as good as that assumption. If the designer treats the Coral Sea losses as breaking the link, Q1 and Q3 are both gaps for these groups and they are 2. Their value runs from 2 to 4.

### Scenario 6: Rabaul against Guadalcanal

| Group | Name | Unit entity | Q1 | Q2 | Q3 | Q4 | Value, strict | Value, low rows too | Confidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | Land attack bombers at Rabaul (4th, Misawa and Kisarazu Air Groups) | 4th-ku (1004), misawa-ku (1013), kisarazu-ku (1012) | gap: the three units differ | gap: the three units differ | gap | gap | 2 | 2 | A group of three units with three different answers: 4th Air Group first action 20 Feb 1942 (air_unit_facts 363); Misawa 8 Aug 1942 and "None before" (355, 356); Kisarazu 1937 (359) but a training unit until March 1942 (360, 361). All `low`, `unconfirmed`. Q3 would be Y for the 4th Air Group alone (training_facts 17), gap for Misawa |
| 2 | Tainan Air Group | tainan-ku (1019) | gap strict; Y low rows: air_unit_facts 343 (8 Dec 1941) | gap | Y: training_facts 17, 5, 1, 2 as for the carrier groups | gap | 3 | 4 | Medium on Q3, low on Q1. air_unit_facts 345 (original pilots China veterans, only the most experienced allowed to fly combat) is words and answers nothing |
| 3 | 6th Air Group | 6th-ku (1007) | N (18 Apr 1942 is an interception, not action; first combat found Midway, 4 June) | N: between the two dates | gap | gap | 2 | 2 | air_unit_facts 347, 348, 349, all `low`, `unconfirmed`. If its 18 April date is read as action it still falls between the dates. Either way 2 |
| 10 | VMF-223 | vmf-223 (1026) | N | Y: air_unit_facts 411, 412 | gap | Y: training_facts 49, air_unit_facts 413 | 0 | 0 | As scenario 5 group 9 |
| 11 | VMSB-232 | vmsb-232 (1030) | N | Y: air_unit_facts 423, 424 | gap | Y by training_facts 49 | 0 | 0 | As scenario 5 group 10. Without Q4, 1 |
| 12 | Flight 300 | flight-300 (1010) | gap | gap | gap | gap | 2 | 2 | air_unit_facts 43 to 46 are aircraft and base only. It was flown from Enterprise, but no row says its pilots came from a named squadron, so the Enterprise statements (394 to 397) are not carried over |
| 13 | VMF-224 | vmf-224 (1027) | N | Y strict: air_unit_facts 418 (`medium`, `preferred`, "None before 30-31 Aug"); first-action rows 416 (31 Aug, `medium`, `disputed`) and 417 (2 Sep, `low`, `disputed`) differ by two days and both fall after 7 Aug, so the dispute is settled | gap: no row ties 275 hours to this squadron | gap | 1 | 1 | Medium. training_facts 49 names VMF-223 and VMSB-232 only; I do not carry it to VMF-224 |
| 14 | VMSB-231 | vmsb-231 (1029) | N | gap strict; Y low rows: air_unit_facts 421 (`low`, `unconfirmed`, none as the rebuilt squadron; arrived 30 Aug) | gap | gap | 2 | 1 | Low. 421 also says its earlier personnel flew at Midway as VMSB-241 and the squadron was rebuilt at Ewa |
| 15 | VF-5 | vf-5 (1023) | gap | gap | gap | gap | 2 | 2 | As scenario 5 group 5 |
| 16 | Fighters from Hornet and Wasp | none (VF-71 is in air_units.csv, 1025, but with aircraft counts only, air_unit_facts 105 to 107, 219) | gap | gap | gap | gap | 2 | 2 | No unit entity for the Hornet fighters, and a mixed group of two carriers' squadrons |
| 17 | VS-3 | vs-3 (1034) | gap | gap | gap | gap | 2 | 2 | As scenario 5 group 7 |

### What the table gives, in one line

Strict: two Marine squadrons at 0, VMF-224 at 1, the Japanese carrier groups, Ryujo's groups and Tainan at 3, everything else at 2 (including every US Navy group). With low rows admitted: Japanese carrier groups, Ryujo and Tainan at 4, VMSB-231 at 1, the rest the same.

## 4. Gaps, and the research that would close each

Every group below stays at 2 on the questions named.

**US Navy, all groups (VF-6, VB-6, VS-5, VT-3, VF-5, VB-3, VS-3, VT-8, scenario 6 VF-5, VS-3, Flight 300, Hornet and Wasp fighters): Q3 and Q4.**
- Research: the average flying hours of a US Navy pilot on joining a fleet squadron, 1941 and 1942. `training_facts.csv` has none; `docs/research/oob/experience.md` says "not found anywhere". It is the biggest gap, because the Japanese hours exist and the US figure does not, and it decides whether the two sides differ at all. Where to look: Navy training statistics and reports of the Bureau of Aeronautics or the Chief of Naval Air Training; squadron histories; the published squadron-level accounts of 1942 (the page cites Lundstrom's work in other rows, which no one here has read). Each would be a new `usn-aviators` row of `flying_hours_on_joining`.

**US Navy: Q1 and Q2, first action for VF-6, VB-6, VS-5, VT-3, VS-3 (no row), VF-5 (air_unit_facts 401 is a lower bound), VB-3 and VT-8 (`low`, `unconfirmed`).**
- Research: the date each squadron first fought, from the squadron or carrier war diaries and the action reports (Enterprise: `cv6-action-reports` is already a source). No row gives VF-6 any first-action date. VT-8 needs the date for the rebuilt squadron, not the Midway squadron. A `medium` or `high` source would make each a `preferred` row.

**Enterprise Air Group (VF-6, VB-6, VS-5, VT-3): a possible new question.**
- Research: the date the group was reformed (Captain Davis's "hastily collected remnants", air_unit_facts 394), and which squadrons came from which groups. With a dated row the new question "reformed from other groups' remnants in the three months before" could be asked of these four, and it would use the one `high`-confidence unit-level statement about a US Navy group. Without the date it is words.

**Japanese carrier groups (Shokaku, Zuikaku, Ryujo): continuity of crews after Coral Sea and Dutch Harbor.**
- Research: whether and when the groups' losses were made good (air_unit_facts 368). Japanese carrier division records (JACAR, `jacar-kodochosho-zuikaku` is already a source) give the pilots by name for 24 August; a count of how many were in the group in December 1941 would turn the assumption of the note under the scenario 5 table into a fact.
- Also: the formed date of each carrier group, and a first-action date from a source better than the Wikipedia sentence (air_unit_facts 366 to 393 are one sentence, written three times per carrier).

**Japanese land groups: Misawa, Kisarazu and 4th Air Groups (the Rabaul composite), and the 6th Air Group.**
- Research: a Japanese hours figure for a pilot who joined in 1942. `training_facts.csv` has none for any pilot of mid or late 1942: row 19 (`low`, `unconfirmed`) says experience "fell off gradually through 1942" with no number, and the figures for 1944 and 1945 (rows 3, 4, 26) are out of range. The Japanese Navy's own training records, and the interrogation of Mieno (USSBS Interrogation 444, cited at second hand), are the leads. For the 6th Air Group, a first-fight date that is not the 18 April interception. For the composite group 1, the pilot share of each of its three units on 21 August (none is in the rows).

**Floatplane groups (Chikuma, Tone).**
- Research: any first-action or hours row for cruiser floatplane crews. None exists. These are reconnaissance units; a value of 2 for both is the honest answer.

**US Marine VMF-224, VMSB-231, and the Cactus Air Force squadrons not covered by training_facts 49.**
- Research: pilots' hours for VMF-224 and VMSB-231. training_facts 49 names two squadrons. A squadron history for VMF-224 and VMSB-231 (the `usmc-vma-223-history` source is a Marine Corps history; its sister histories for these squadrons would be the lead) would answer Q4, and a `medium` or better first-action row would settle VMSB-231's Q2.

**VMSB-232: hours specific to the squadron.**
- Research: air_unit_facts 426 is `low`, `unconfirmed` because the sentence's detail is about VMF-223. A squadron-level statement of its pilots' hours would settle it. Until then its value is 0 on a weak leg, or 1 if Q4 is dropped.

**Hornet and Wasp fighters (scenario 6 group 16).**
- Research: which squadrons were meant. The scenario file does not say, and no unit entity matches (VF-71 has aircraft counts only). Either name the squadrons in the scenario or give the group no experience value.

## 5. Can air-group experience rest on these facts?

Spec condition 4 (`docs/step2-spec.md`, "Experience"): if research finds too little, experience waits. My judgement, in plain words: **the facts are too thin for the air groups to carry experience now, and they are thinner than for the ships.** A summary, then why.

What the rows can do.
- They separate two Marine squadrons (VMF-223, VMSB-232) from every other group, by two reasons that agree: a first action only on 21 and 23 August (`high`, `preferred`), and about 275 flying hours, all in trainers (`medium`, `preferred`, `low` for VMSB-232 itself). This is unit-level fact for the first reason and a statement about the squadrons for the second.
- They lift the Japanese carrier groups, Ryujo's groups and Tainan above the rest, by one row: training_facts 17 (the bombing survey's summary, "about 500 to 800 hours" for the first-line Japanese pilot in 1941, `medium`, `preferred`), backed by four disputed rows that all fall on the same side. First-action dates (7 and 8 December 1941) would add another point, but they are single `low`, `unconfirmed` Wikipedia sentences.

What they cannot do.
1. **They separate the two sides by a single service-wide hours statement, which is the weak leg.** The Japanese figure is one statement about a navy at one date (December 1941); it is not a fact about the groups at Eastern Solomons. It says nothing of the men who joined after Coral Sea (training_facts 19 says only that experience fell off). This is the same weak leg as ship question 4, but weaker: ship question 4 had a second independent source (training_facts 87); here the Japanese figures are partly one source relayed several times (training_facts 1 and 25 both relay USSBS reports at second hand, and 2 repeats a figure with no source).
2. **The US Navy has no hours figure at all.** Enterprise's and Saratoga's groups are every one of them at 2 because a row is missing, not because a row says 2. A missing row reads as "trained to its service's standard", while the Japanese groups get a plus on the strength of a row that exists. The effect at Eastern Solomons would be a Japanese advantage of one point built on the fact that the bombing survey interviewed Japanese officers and no one wrote a US counterpart down. A gap is not neutral between two sides when one side's rows exist and the other's do not.
3. **The one `high`-confidence unit-level statement about a US Navy group** (Captain Davis on the Enterprise group: formed from remnants, "majority of the pilots very inexperienced", gunnery and bombing proficiency low, air_unit_facts 394 to 397) answers no question under the rule, because it is words and undated, and it was written after the battle. So the rule gives that group the same 2 as a group with a good record. This is the cost of the rule, and it is the right cost for a model that must not be worked back from the battle. It means the table cannot show the thing the best source says.
4. **The questions double-count a new squadron.** A squadron that has just formed meets both Q2 and Q4. VMF-223 and VMSB-232 are at 0, the lowest value, on two questions that are one fact told twice (the squadron is new and its men are fresh from school). The designer may want a squadron's "new" and "low hours" questions to be one. I have kept them separate as 0064 asks.
5. **Most first-action rows are one `low`, `unconfirmed` Wikipedia sentence** (air_unit_facts 342 to 393, 404 to 409, 415, 420 to 421). Under the stage 1 rule none of them answers. Only the Marine squadrons (VMF-223, VMSB-232, VMF-224), whose histories were read, have `medium` or `high` first-action rows.
6. **Hours rows differ in what they measure.** The Marine 275 is hours in trainers, none in the combat type (training_facts 49); the Japanese figures are total hours at a date, some of them on entering combat (5), some of them averages in a first-line unit (17, 1, 2, 25). Q3 and Q4 read both as "hours" because 0064 says so. They are not the same quantity.

Where this leaves 0064's "if they stand" (item 10): three rules, each with a strength, would read these values. With 0 for four group entries (two Marine squadrons, in each scenario), 3 for nine Japanese group entries and 2 for seventeen, and with a missing US Navy row deciding a point of difference, a strength tuned to match the record would be bending a figure to the one unequal-footed row. I recommend the designer not let the air groups carry a value yet, with two exceptions to consider:
- **Marines at 0**, on the strength of two `high` first-action rows. It is firm enough to rest on. It touches two squadrons at Henderson Field in both scenarios.
- **Everything else at 2** (the experience field exists, with no rule reading a difference).

What would change the answer: a US Navy hours figure for 1941 and 1942 (section 4, first item), and a better first-action source for the Japanese carrier groups. Both are research, not tuning. If the two came back with rows of `medium` confidence the rule above would answer for every group in the two scenarios without change.

## 6. Rows that look suspicious or contradictory

- **training_facts 5 against 17.** Row 5 (650 hours on entering combat, Navy, December 1941) is marked `disputed`, and its note says it "differs from ... the 500-800 range in the same Summary". But 650 lies inside the range 500 to 800 that row 17 gives from the same source. The "dispute" with row 17 is no dispute; only 1 (700) and 2 and 25 (800) are different figures, and they are within the range too. I settle them as agreeing on the side of the threshold, which is all Q3 needs.
- **training_facts 17, dates.** The statement is "average first-line Japanese pilot in 1941", but the row gives `valid_from` 1941 and `valid_to` 1942. I read it as 1941. If its period is read as 1941 to 1942, Q3 would also answer for the the Misawa Air Group (first action August 1942) and for the 6th Air Group, which I think the statement does not support.
- **training_facts 49, 413 (air_unit_facts), 425, 426.** The 275-hour sentence is one sentence about "nearly all" the pilots of two squadrons whose details (ages, ranks, names) are all VMF-223 (the notes of 425 and 426 say so). I answer Q4 for VMSB-232 on training_facts 49 at `medium`, against the squadron's own row 426 at `low`. Row 413 duplicates row 49 for VMF-223, so it is one source, not two.
- **air_unit_facts 366 to 389.** The same Wikipedia sentences are copied to each of the three units of a carrier (366/370/374, 367/371/375 and so on). The notes say "the Shokaku carrier air group as a whole; the fighter unit is not separately described". It is one statement, not three. Counting it for each unit as an answer is a reading, not three sources.
- **air_unit_facts 363 against 364.** The 4th Air Group's first action is 20 February 1942, but the note says it was formed from divisions that fought in the Philippines and the Dutch East Indies. Whether that makes it a seasoned group is not in the rows, and it is why I read the date at its face and do not void it. It matters only inside the composite group 1, which is a gap anyway.
- **air_unit_facts 424.** VMSB-232's aircraft were caught on the ground at Ewa on 7 December 1941. I read "action" as flying against the enemy, as the same row does ("no combat flying before 23 Aug 1942"), so the group is Q2 yes. If the designer reads the Ewa attack as action, Q2 becomes no and VMSB-232 rises to 1 on hours alone.
- **air_unit_facts 359, 360, 361.** Kisarazu's first action is 1937 (China), but it was a training unit from 1940 to March 1942 and the article "does not say the 1942 aircrews were the 1937-40 veterans". The row's own note is the reason I void the date.
- **air_unit_facts 404.** VB-3's 4 June date is from Pacific Wrecks and says the squadron was aboard Yorktown at Midway, while on 7 August it belonged to the Saratoga group (ONI). This is a squadron changing carriers, and the rows do not say whether its crews changed. It falls between the thresholds either way.
- **air_unit_facts 416, 417.** VMF-224's first action, 31 August (`medium`, `disputed`) or 2 September (`low`, `disputed`). Settled for Q2, since both fall after 7 August.
- **training_facts 87 is not used here.** It is the ships' second source (night training) and says nothing on flying.
- **No row is worked back from a battle result.** I read none of: the 61 veteran aircrew lost on 24 August (air_unit_facts 369 and its copies, "after the scenario start"), the 32 pilots killed from August to November (346), the squadron losses of 7 and 8 August (399, 400), or Captain Davis's judgements (394 to 398), as an answer to any question.
