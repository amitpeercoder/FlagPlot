# 0074: A class's torpedo tubes are the tubes that bear on one side

Status: written by the architect on Nov 13, 2026, as a rule of the derivation; not built. **Held by the coordinating architect on Nov 13, 2026 until Savo Island's first tuning rounds:** it is built only together with a rule for firing the other side's tubes in a later round, which is a plan for the designer, and only if the torpedo measures then call for it. It is not a condition of tuning.

Why it is held. The tubes of one side alone take the five Japanese heavy cruisers from 40 torpedoes to 20, against 32 to 34 fired in the record, and would turn a measure that agrees today (torpedo hits on Allied ships) into a miss by leaving half a rule out. With the second side fired in a later round the totals come back to about what the rules give today, spread over two rounds, at the cost of new state for a ship, a new hash version and a rule for the designer. The gap on the Allied side (4 to 6 torpedoes fired of some 80 tubes) is unreadiness, which decision 0071's share carries, not tubes that bear. So by `docs/engineering/rule-complexity.md` the pair is not worth building before tuning shows whether the torpedo measures need it.

## The gap

A ship in the rules fires her torpedoes once: every tube of her class, in the first round in which she has a target (`docs/rules/04`, "Torpedo shots"). The derivation gives a class the whole number of tubes its ships carry (decision 0020).

Many ships could not do that. A mount on the ship's side fires only to that side, so half the tubes bear on an enemy and the other half point at the empty sea.

| Ship, at Savo Island | Tubes | The rules fire | She fired |
| --- | --- | --- | --- |
| Bagley | 16, in four wing mounts | 16 at once | 4, from her port mounts |
| Chokai | 8, in four twin mounts, two a side | 8 at once | 4 to port, and 4 to starboard ten minutes later |
| Furutaka | 8, one quadruple mount a side | 8 at once | 4 to port, then 1 and 3 to starboard |
| Tenryu | 6, in two centreline mounts | 6 at once | 6 |

The record is in the research of Nov 13, 2026: `data/oob/ship_class_facts.csv` rows 1176 to 1353 (attributes `torpedo_tubes`, `torpedo_tubes_per_broadside`, `torpedo_mount_position`), and `docs/research/oob/torpedoes-fired.md` for what each ship fired.

## Decisions

### The rule

1. **The figure `torpedo_tubes` of a derived class is the number of tubes that bear on one side.** The simulation fires that number, once. Nothing in `sim-core` changes: it already fires the class's figure once and then has no torpedoes ready.
2. **The whole count is read as it is today**, from the ship's or the class's `torpedo_tubes` fit. Call it T. A class with no tubes (T = 0) has the figure 0 and reads nothing more.
3. **The split is read from the class's fact rows, in this order.**
   - A readable row of `torpedo_tubes_per_broadside`: the figure is that number, B.
   - Else a readable row of `torpedo_mount_position`: `centreline` gives T, and `wing` gives T / 2.
   - Else the figure is T, and the class is named in the tool's output as **not split: no row says how its tubes bear**.
4. **What stops the derivation, naming the class and the rows:**
   - B above T, or B of 0 where T is above 0.
   - A position of `wing` where T is odd.
   - A position of `mixed` or `fixed` with no readable row of the number: the position alone does not give it.
   - A readable number and a readable position that disagree (`centreline` with B below T, or `wing` with B other than T / 2).
5. **Readable means what it means for every other fact** (decisions 0020 and 0023): a `preferred` row of medium or high confidence, valid on the list's date, or a row named by an exception in `rules.toml` with its reason. Rows that disagree stop the derivation as they do for any fact.
6. **The frozen step 1 figures and the placeholder classes do not change.** Their tubes are invented and stay as they are.

### A class with no row of the split

7. **It keeps its whole count, and the output says so.** A whole count is what the derivation gives today, so nothing unproven is added: the figure is the most the class could fire and is marked as such. Halving it would be a guess about where its mounts stood, and stopping the derivation would hold back a whole battle for one destroyer.
8. **The gap is the research session's to close.** The classes with no row of the split today are Takao and Atago (as against Chokai and Maya), Myoko, Mogami, Tone, and the mount positions of Fubuki, Mutsuki and Kamikaze. Of these only Kamikaze is in a derived list (Yunagi, at Savo Island).

### The exceptions this rule needs for Savo Island

Most of the new rows of the split rest on one weak source and are `unconfirmed`. Each is read by an exception, written in `rules.toml` when the rule is built, for the reason given here.

| Class | Row | Value | Why it is read |
| --- | --- | --- | --- |
| Takao (Chokai) | 1341 | 4 on a side | Her own action report has her fire 4 to port and 4 to starboard as separate salvos, and says "four firing lines are too few". The row is for Chokai and Maya alone, who kept twin mounts; Chokai is the only ship of the class in a list |
| Aoba | 1343 | 4 on a side | One quadruple mount each side of the catapult after the rebuild. At Savo Island Aoba and Kinugasa fired salvos of four or fewer to one side at a time |
| Furutaka | 1346 | 4 on a side | The same fit. Furutaka fired 4 to port, then 1 and 3 to starboard; Kako 4, 4 and 2 |
| Tenryu | 1323 | 6 on a side | Two centreline triple mounts. She fired 6 |
| Yubari | 1327 | 4 on a side | Two centreline twin mounts |
| Kent subclass of the County class (Canberra) | 1265 | 4 on a side | One quadruple mount each side. She fired none, or two, that night |
| Benham (Wilson) | 1182 | 8 on a side | Four quadruple wing mounts as built. Whether the fit of 1942 was still that is a gap; no source read says it had changed by August 1942 |

Bagley needs none: row 1177 (8 on a side) is `preferred` at medium confidence.

## The hand derivation

T is the tubes the list derives today. Each line is worked by the rule above and is the expected value of the tool's test.

| Class | T | Rows read | Working | Figure |
| --- | --- | --- | --- | --- |
| Bagley | 16 | 1177: 8 on a side (preferred, medium). 1178: wing | The number is read: 8. The position agrees: 16 / 2 = 8 | **8** |
| Takao, for Chokai | 8 | 1341: 4 on a side, by exception | The number is read: 4. No position row | **4** |
| Tenryu | 6 | 1323: 6 on a side, by exception. 1324: centreline, unconfirmed and not read | The number is read: 6 | **6** |
| Yubari | 4 | 1327: 4 on a side, by exception | The number is read: 4 | **4** |
| Farragut (not in a list; the rule's test alone) | 8 | 1201: 8 on a side (preferred, medium). 1202: centreline (preferred, medium) | The number is read: 8. The position agrees: all 8 | **8** |
| Farragut, with row 1201 taken out | 8 | 1202: centreline | No number; the position gives T | **8** |
| Bagley, with row 1177 taken out | 16 | 1178: wing | No number; the position gives 16 / 2 | **8** |
| Kamikaze, for Yunagi | 6 | none | Not split: T, and the output names the class | **6**, not split |
| Mahan (not in a list; the rule's test alone) | 12 | 1193: 8 on a side (preferred, medium). 1194: mixed | The number is read: 8. Mixed says nothing against it | **8** |
| Mahan, with rows 1193 and 1196 taken out | 12 | 1194: mixed | Stops: a mixed fit with no number | refused |

What the Savo Island list gives after the rule:

| Class | Today | After |
| --- | --- | --- |
| Aoba, Furutaka, Takao | 8 | 4 |
| Tenryu | 6 | 6 |
| Yubari | 4 | 4 |
| Kamikaze (1922) | 6 | 6, not split |
| County, Kent subclass | 8 | 4 |
| Bagley (both rows), Benham | 16 | 8 |
| New Orleans, Northampton | 0 | 0 |

## What it does at Savo Island, by arithmetic

Not runs. Torpedoes a force can fire in the night, every ship firing once.

| | Today | After | The record |
| --- | --- | --- | --- |
| The five Japanese heavy cruisers | 5 x 8 = 40 | 5 x 4 = 20 | 32 to 34: Chokai 8, Kako 9 or 10, Furutaka 8, Kinugasa 4 (2 in another passage), Aoba 4 |
| Tenryu, Yubari, Yunagi | 6 + 4 + 6 = 16 | the same | Tenryu 6, Yubari 4 by the analyst's inference, Yunagi not found |
| A Bagley class destroyer | 16 | 8 | Bagley 4; Helm, Wilson, Patterson and Blue none |

So the rule moves the American destroyers toward the record and takes the Japanese heavy cruisers below it. The heavy cruisers fired both sides, one after the other, as they turned. That is the question this record leaves out, and the Savo Island measure of torpedo hits on Allied ships (4 to 9 in the record, 1 to 7 in the model today, agreeing) is likely to fall when this is built. The report made after the build will say by how much.

## What it leaves out

- **Firing the other side's tubes later in the fight.** Chokai, Kako and Furutaka did, ten to twenty minutes after their first salvo. In the rules a ship's torpedoes are fired once. Letting a ship with wing mounts fire again in a later round is a change of rule, with a rule card, and the designer's to approve. It is not in this decision. Until it is made, a ship with wing mounts fires half her tubes in a fight and no more.
- **Reloads.** The Japanese destroyers and cruisers carried spare torpedoes and reloaded in action (Kako had 6 remaining after firing 10). Nothing is reloaded in the rules.
- **Why a ship did not fire.** Bagley's starboard primers were not in; Helm and Wilson found no target. Decision 0071 (a force not ready fires a share of its torpedoes) deals with readiness and multiplies whatever this rule gives. Its share does not carry the tubes that bear, and this figure does not carry readiness: the two are kept apart.
- **One ship's fit where it differs from her class's.** Australia's tubes were removed in 1940, Jintsu and Abukuma were refitted, Atago and Takao had quadruple mounts. These are in the research as leads, with no fact row for the ship. A ship's own `torpedo_tubes` fit is read before her class's, as today; a ship's own split would need a row the research has not written.
- **The classes of the Eastern Solomons list.** Its surface weapons are not modelled (decision 0029), so its tubes stay 0.

## Why

- **It is a fact of the ship, not a tuning figure.** Where a mount stands is in the sources. No figure in `surface_combat.toml` could stand in for it without being wrong for every centreline ship.
- **It needs no rule in the simulation.** The class's figure already means "the tubes fired"; the derivation was giving it the wrong number for ships with wing mounts.
- **A number where a source gives one, the position where it does not.** The number is the more direct statement, and covers fits like Mahan's that a position cannot. The position covers the classes whose sources say where the mounts stood and print no number.
- **A gap stays a gap.** A class with no row keeps today's figure and is named, and no ship's tubes are halved on a guess.

## What it costs

- `oob-derive`: the reading of two class attributes, the rule, the refusals of decision 4, a line in the output for a class not split, and tests on the hand derivation above.
- `rules.toml`: the rule's text and the seven exceptions.
- The Savo Island class file is generated again. The run baselines, golden files and recorded games of scenarios 4, 7 and 8 are remade, and the Savo Island report is run again.
- `docs/rules/04` and the glossary say that a derived class's `torpedo_tubes` is the tubes that bear on one side. No rule of the simulation changes, so there is no rule card.
