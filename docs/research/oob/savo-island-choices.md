# Savo Island: four proposed choices for scenario 4c

Status: **proposed, not decided.** Written by Claude on 2026-11-10 and revised after review. Amit confirms or overturns each one when the stage 4c plan is put to him. Until then nothing here is settled in the data: every row these proposals touch keeps the standing the sources give it.

These are proposals about how to use the facts where the sources stop. None is a sourced fact and none adds a figure to the facts layer. The evidence is in `savo-island.md` and `conflicts.md`.

## 1. Damage measure: one displacement figure per class

**Proposal.** Use one displacement figure per class as the measure of how much damage a ship can take, as below. The rows stay `disputed` in `ship_class_facts.csv` until Amit confirms.

The figures are not all the same kind of thing. The table says what each one is, in the source's own terms.

| Class | Ships at Savo | Proposed figure (long tons) | What the source calls it | Other source figures |
| --- | --- | --- | --- | --- |
| Takao | Chokai | 11,350 | One website prints it as displacement in "t" without naming the condition; Wikipedia gives the same number in long tons | 9,694: the as-built figure of the website that carries most Japanese class data |
| Aoba | Aoba, Kinugasa | 8,858 | The figure after the 1938 to 1940 rebuild, converted from 9,000 metric tons. The condition was not recorded from the page | 7,100, before the rebuild |
| Furutaka | Furutaka, Kako | 8,563 | The figure after the 1936 to 1939 rebuild, converted from 8,700 metric tons. The condition was not recorded from the page | 8,100, before the rebuild |
| Tenryu | Tenryu | 3,948 | **"Normal", not standard** | 3,886: the same 3,948 treated as metric tons |
| Yubari | Yubari | 2,844 | "Standard", as completed in 1923, converted from 2,890 metric tons. Two rows, one source | 3,387, which that source's page suggests is normal displacement |
| Kamikaze | Yunagi | 1,378 | **"Normal", not standard**, converted from 1,400 metric tons | 1,270 |
| County (Kent) | Canberra, Australia | 9,750 | One website labels it "BRT", a merchant-ship measure it uses for every warship; taken as standard. A second prints the same number in "t" | 9,850 to 10,570 for single ships at other dates |
| Northampton | Chicago | 9,300 | "BRT" on the same website; taken as standard | 9,050 and 9,006 |
| New Orleans | Vincennes, Quincy, Astoria | 9,950 | "BRT" on the same website; Wikipedia gives the same number as standard | 10,136 |
| Bagley | Bagley, Patterson, Helm, Blue, Ralph Talbot | **not settled** | Candidates: 1,624 (two weak sources that may share an origin) and 1,646 (one website, condition not named) | 1,500, a nominal treaty figure |
| Benham | Wilson | **not settled** | Candidates: 1,657 and 1,656 (two weak sources) | 1,500, a nominal treaty figure |

**So the measure is not one definition.** It is standard displacement where a source says so (Yubari, and New Orleans by one source), normal displacement for Tenryu and Yunagi, and a figure whose condition the source does not name for the rest. Normal is a heavier condition than standard, so those two ships are overstated against the others by an amount the sources do not give. Whether the main Japanese source's tons are metric or long is not stated on its pages, and the Tenryu figure suggests long. That is a 1.6 per cent doubt on every Japanese figure.

**Six destroyers have no preferable figure.** No source graded above "low" gives a displacement for the Bagley or Benham class. The candidates differ by under 2 per cent.

**Why displacement at all, and why not full load.** It is the one measure every class has. Full load has no common definition and several classes have no agreed figure.

**How much it matters.** Leaving aside figures that describe a different ship (before a rebuild, or a design figure), the candidates for each class lie within 4 per cent, except Yubari at 19 per cent. A damage rule sensitive to 4 per cent is finer than this data.

**How the picks were made.** The ship as she was in 1942 where a rebuild changed her; then a figure two sources give; then the better-graded source. Decision 0015 says a higher grade alone does not settle a dispute, and three of these picks (Kamikaze, New Orleans, Northampton) rest on little more than that. That is why they are proposals.

## 2. Radar where no set is named: unknown, with a proposed treatment

**The facts.** What radar Bagley, Patterson, Helm, Wilson, Jarvis and Australia carried on 8 August 1942 is **unknown**. No source read names a set for any of them and none says they had none. The data records it as a gap.

Two things are now sourced from the Australian official history:

- **Canberra was using a Type 271 surface warning set that night,** and saw Blue on it at intervals. It had earlier picked up cruisers at 30 miles. That settles her set in the data, against websites that said Type 241.
- **The pickets were chosen because, in their squadron commander's opinion, they had "the most effective radar and radar operating capability of the nine ships in his squadron".** That reads as the other seven destroyers, Bagley, Patterson and Helm among them, having some radar that was less effective or less well worked. The data has low-confidence rows for those three saying "a set, type not stated", marked as an inference. Wilson was in another squadron and nothing was found.

**Proposal for the scenario.** Treat every Allied ship in the fight as carrying a search radar no better than the pickets' set, except Wilson and Jarvis, which get none as a stated assumption. Vincennes, Quincy and Astoria also have a fire-control radar that ranges on a target already found.

**Why this and not "no radar".** An earlier draft gave five destroyers no radar, arguing that no contact by them is recorded. Review showed that was circular: Chicago, Canberra and both pickets had sets and also detected nothing. The official history now points the other way for three of the five.

**What it means for the detection rule.** On this proposal nearly every Allied ship had a search radar and none detected the Japanese column. So the rule must make search radar close to useless in those waters that night, as the pickets' record shows, and the outcome should not turn on which ships are given a set.

## 3. Torpedoes with no named type: a proposed reading

**The facts.** No source read names the torpedo itself for Kinugasa, Tenryu, Yubari or Yunagi in August 1942. A second pass found what their tubes were:

| Ship | What is sourced | What it implies |
| --- | --- | --- |
| Kinugasa | Eight 24-inch tubes (1950 analysis). The post-war US technical mission says the quadruple Type 92 mount, which her class was given in its rebuild, fires the Type 93 | Type 93, by inference from the mount |
| Tenryu | Six 21-inch tubes (1950 analysis) | Not the Type 93, which is a 24-inch torpedo. The 21-inch 6th Year Type by inference |
| Yunagi | Six 21-inch tubes. The technical mission's table of mounts lists the 6th Year twin 21-inch mount for her class | As Tenryu |
| Yubari | Four 24-inch tubes. The technical mission's table lists her with the 8th Year twin mount. One unsourced website says she carried four reserve Type 93 | Open. The mount is the old one. Whether it could fire, or she was issued, the Type 93 was not found |

For the US Mark 15, the Bureau of Ordnance history says the heavier explosive was first used in autumn 1942, and the Navy's torpedo history gives the 374 kg warhead as a later model. Neither says outright what destroyers carried in August.

**Proposal.** The combat rule reads these, each flagged as an inference:

| Ship | Torpedo | Warhead |
| --- | --- | --- |
| Kinugasa | 61 cm Type 93 | 490 kg |
| Tenryu, Yunagi | 53 cm 6th Year Type | about 200 kg |
| Yubari | 61 cm 8th Year Type | about 345 kg |
| US destroyers | Mark 15, first model | 224 kg |

**How to carry it.** The Tenryu, Yunagi and Kinugasa rows are in the data as low-confidence inferences, so a rule that says it may read unconfirmed values gets them from the facts layer. Yubari and the Mark 15 warhead are `disputed` in the data, so the scenario has to name them itself.

**What it risks.** Only Yubari is still a real doubt. If she had Type 93, her four torpedoes are understated by a third.

## 4. The disputed placements: a proposed reading

**The facts.** The 1943 narrative and the 1950 analysis list Australia, Jarvis, Hull and Hobart differently. The rows stay `disputed` in `engagement_forces.csv`.

**Proposal.**

| Ship | Placement at the start of the night action |
| --- | --- |
| Australia | Not with the southern group. Inside the transport screen off Guadalcanal, about 20 miles from Savo. She takes no part |
| Jarvis | Not in any group. Alone, damaged, steaming west about 3,000 yards south of Savo at slow speed, unable to report. In the scenario because the Japanese column sighted her and held fire |
| Wilson | In the northern group, in Jarvis's place |
| Hull | With the transport screen. Not in the fight |
| Hobart | With San Juan in the eastern group. Not in the fight |

**Why.** For Australia and Hobart the two accounts mostly describe different things: one the task organisation on paper, the other where the ship was that night. For Jarvis the 1950 analysis reconstructs her track from Japanese sightings, and the Navy's history has Wilson replace her after the torpedo hit of 8 August. Hull appears in no account of the action.

**The command consequence is part of the placement.** With Australia gone, Chicago's captain was in charge of the southern group and gave no orders, and the northern group's commander did not know he was the senior officer present.

## What these proposals do not touch

- Whether Canberra was torpedoed, and by whom. The scenario should not need to decide.
- When Chokai changed to Type 93 torpedoes. She had them on the night.
- Rate of fire. Every figure is from one website and is the gun's rate, not what crews achieved. The outcome data gives a check: the three northern cruisers fired 107 main-battery rounds between them before they were silenced.
- Ammunition carried. Now in the data for main guns of every class but Tenryu, Yubari and Yunagi, and for torpedo reloads of the Japanese heavy cruisers. See `savo-island.md`.
