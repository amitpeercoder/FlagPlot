# Savo Island: four proposed choices for scenario 4c

**What was decided is in `docs/decisions/0019-savo-island-choices.md`.** That record is the designer's answer. This file is the research behind it and is kept for the reasoning. Where the two differ, the decision stands.

Status of this file: proposals, written by Claude on 2026-11-10 and revised after review. They do not settle anything in the data. Decision 0019 took the proposals on placements and on displacement as the damage measure. It treats unknown search radar as not fitted, as a named working assumption. It leaves torpedo types, gun marks and ammunition open as facts still being sourced. The figures themselves are put to Amit row by row in `savo-island-settlement.md`.

The evidence is in `savo-island.md` and `conflicts.md`.

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

## 2. Radar where none was found: unknown, with a proposed treatment

**The facts.** What radar Bagley, Patterson, Helm, Wilson, Jarvis and Australia carried on 8 August 1942 is **unknown**. No source read says any of them had a set and none says they had none. The data records it as a gap, not as "none". Canberra's surface set stays disputed between Type 271 and Type 241, on four weak sources.

**Proposal for the scenario.** Give those six ships no radar detection of surface ships, as a stated assumption. Give Chicago and Canberra a search radar, and Vincennes, Quincy and Astoria a fire-control radar that ranges on a target already found.

**What supports it.** One thing only: the Navy's history of Blue says the two pickets were chosen because they had the new search set, which implies most other destroyers present did not.

**What does not support it.** An earlier draft argued that no radar contact by these ships is recorded. That proves nothing: Chicago and Canberra had search sets and also detected nothing, and so did both pickets. The absence of a contact that night is not evidence about what was fitted.

**What it risks, and why the risk is small.** If one of the five destroyers did carry a set, the scenario understates a capability that failed for every ship known to have it. The detection rule has to reproduce "radar near land found nothing" from the ships that did have sets, and should not depend on this assumption.

## 3. Torpedoes with no named type: a proposed reading

**The facts.** No source read names the torpedo carried by Kinugasa, Tenryu, Yubari or Yunagi in August 1942. For Yubari the data is in conflict with itself: a ship row names the Type 93 and the class rows name the 8th Year Type, both by assumption. All three rows are now marked `disputed`, so neither overrides the other. For the US Mark 15, both warhead rows (224 kg and 374 kg) are `disputed`.

**Proposal.** Until a source is found, the combat rule reads these, each flagged as an assumption:

| Ship | Torpedo | Basis |
| --- | --- | --- |
| Kinugasa | 61 cm Type 93 | Her sister Aoba's class fit after the same rebuild. The other three cruisers of her division fired Type 93 that night |
| Tenryu | 53 cm 6th Year Type | The standard 53 cm torpedo when she was built. No source read reports a change |
| Yubari | 61 cm 8th Year Type | The 61 cm torpedo for light cruisers when she was built. No source read reports a change |
| Yunagi | 53 cm 6th Year Type | As for Tenryu |
| US destroyers | Mark 15 with the 224 kg warhead | The first model. Nothing read puts the heavier warhead in service by August 1942 |

**How to carry it.** For the rule to read the Yubari proposal, the scenario has to name the 8th Year Type for her itself. It cannot get it from the facts layer, where the question is open. The same holds for the Mark 15 warhead.

**What it risks.** If Tenryu or Yubari had been re-armed with Type 93, their torpedoes are understated by more than half: 490 kg against about 200 and 345 kg. They fired ten between them, and one compilation credits Tenryu with hits on Quincy. A research pass on this question is under way.

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
- Ammunition carried. Nothing is in the data yet. A research pass is under way.
