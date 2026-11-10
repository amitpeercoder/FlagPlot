# Savo Island: four choices for scenario 4c

Status: decided by Claude on 2026-11-10. Amit delegated these and may overturn any of them. They are choices about how to use the facts, made where the sources stop. None of them is a sourced fact, and none adds a figure to the facts layer.

The evidence behind each is in `savo-island.md` and `conflicts.md`. If the scenario plan records these as a numbered decision, that record replaces this file.

## 1. Damage measure: standard displacement, one figure per class

**Choice.** The measure of how much damage a ship can take is her standard displacement in August 1942, in long tons.

| Class | Ships at Savo | Figure | Next nearest source figure |
| --- | --- | --- | --- |
| Takao | Chokai | 11,350 | 9,694 (the design figure) |
| Aoba | Aoba, Kinugasa | 8,858 | 7,100 (before rebuild) |
| Furutaka | Furutaka, Kako | 8,563 | 8,100 (before rebuild) |
| Tenryu | Tenryu | 3,948 | 3,886 |
| Yubari | Yubari | 2,844 | 3,387 |
| Kamikaze | Yunagi | 1,378 | 1,270 |
| County (Kent) | Canberra, Australia | 9,750 | 9,850 to 10,570 |
| Northampton | Chicago | 9,300 | 9,050 |
| New Orleans | Vincennes, Quincy, Astoria | 9,950 | 10,136 |
| Bagley | Bagley, Patterson, Helm, Blue, Ralph Talbot, Jarvis | 1,624 | 1,646 |
| Benham | Wilson | 1,657 | 1,656 |

**Why standard and not full load.** Standard displacement has one definition for every navy in the list, set by treaty, and every class has a figure for it. Full load has no common definition, and several classes have no agreed figure for it.

**How each figure was picked.** In this order: the figure for the ship as she was in 1942, where a rebuild changed it; then a figure two sources give; then the better-graded source. Each row in `ship_class_facts.csv` now says which it was and why.

**How much it matters.** Apart from the three cases where the rejected figure describes a different ship (design, or before a rebuild), the sources differ by under 4 per cent, except Yubari (19 per cent) and the Kamikaze class (8 per cent). A damage rule that is sensitive to 4 per cent of displacement is too fine for this data.

**Known weakness.** The Japanese figures come from one website that gives no sources, and it is not certain whether its tons are metric or long. The difference is 1.6 per cent. A second weakness: one figure per class hides any difference between sisters.

## 2. Ships with no radar found: treat as having no search radar

**Choice.** Bagley, Patterson, Helm, Wilson and Jarvis are given no search radar on 8 August. Australia is given none either, though she was away from the fight. Vincennes, Quincy and Astoria are given a fire-control radar, which gives a range on a target already found and plays no part in finding one. Chicago and Canberra are given a search radar that detected nothing that night.

**Why.**

- The pickets were chosen because they had the new search set. The Navy's own history of Blue says so. That only makes sense if most of the other destroyers did not have it.
- No source records a radar detection of a ship by any of the five destroyers, by Chicago or by Canberra before the shooting. Whatever they carried, the result was the same as carrying nothing.
- Giving a ship a radar the sources do not show would be inventing equipment. Leaving the ships out of the scenario would remove half the Allied force.

**What it risks.** If one of the five did carry a set, the scenario understates a capability that failed anyway. The detection rule should be able to reproduce "radar near land found nothing" from the two pickets, which did have sets. It should not rely on the other ships having none.

## 3. Torpedoes with no named type: the rule may read the inferred rows

**Choice.** The combat rule may use these, each flagged as resting on an unconfirmed fact:

| Ship | Torpedo | Basis |
| --- | --- | --- |
| Kinugasa | 61 cm Type 93 | Her sister Aoba's class fit after the same rebuild. The other three cruisers of her division fired Type 93 that night |
| Tenryu | 53 cm 6th Year Type | The standard 53 cm torpedo when she was built. No source read reports a change |
| Yubari | 61 cm 8th Year Type | The 61 cm torpedo for light cruisers when she was built. No source read reports a change |
| Yunagi | 53 cm 6th Year Type | As for Tenryu |

US destroyers: the Mark 15, with the 224 kg warhead of the first model. The 374 kg warhead belongs to a later model, and nothing read puts it in service by August 1942.

**Why.** Decision 0015 lets a rule read an unconfirmed value if the rule says so. The alternative is ships with tubes and nothing in them. Tenryu and Yubari fired ten torpedoes between them and Tenryu's are credited with hits on Quincy, so leaving them unarmed would change the result.

**What it risks.** If Tenryu or Yubari had been re-armed with Type 93, their torpedoes are understated: 490 kg against about 200 and 345 kg. The Mark 15 choice matters less. Bagley's four were the only US torpedoes that may have hit anything, and that was Canberra.

## 4. The disputed placements

**Choice.**

| Ship | Placement at the start of the night action |
| --- | --- |
| Australia | Not with the southern group. Inside the transport screen off Guadalcanal, about 20 miles from Savo. She takes no part |
| Jarvis | Not in any group. Alone, damaged, steaming west about 3,000 yards south of Savo at slow speed, unable to report. She is in the scenario because the Japanese column sighted her and held fire |
| Wilson | In the northern group, in Jarvis's place |
| Hull | With the transport screen. Not in the fight |
| Hobart | With San Juan in the eastern group. Not in the fight |

**Why.** For Australia the two official accounts do not disagree about the night: one lists the task organisation and the other says where she was. For Jarvis the 1950 analysis reconstructs her track from three Japanese sightings, and the Navy's history has her replaced by Wilson after the torpedo hit of 8 August. Hull and Hobart appear in no account of the action itself.

**The command consequence is part of the placement.** With Australia gone, Chicago's captain was in charge of the southern group and gave no orders, and the northern group's commander did not know he was the senior officer present. A scenario that puts Australia elsewhere should also start with those two gaps.

## What these choices do not settle

- Whether Canberra was torpedoed, and by whom. The scenario should not need to decide.
- When Chokai changed to Type 93 torpedoes. She had them on the night.
- Rate of fire. Every figure is from one website and is the gun's rate, not what crews achieved that night. The outcome data gives a check: the three northern cruisers fired 107 main-battery rounds between them before they were silenced.
