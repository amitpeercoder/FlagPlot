# 0105: Who is engaged in a fight ashore, and an attack that fails

Status: a plan by the architect, written on Nov 14, 2026, after the first baseline of the lengthened slice (`docs/validation/full-slice-39-days-baseline.md`), and revised the same day on the designer's answers. He agreed the limit on engaged men (decision 1). He rejected the first draft's rule that an attack breaks on losing a fifth of its men in a day: an attack fails when it does not overcome the defence, and what follows is wear on the men, not a rule that calls them off (decisions 4 and 5). And he said that what men do when their attack is spent depends on the command and on whether they can stay (decisions 6 and 7). Confirmed as revised. Built Nov 14, 2026 (scenario 33); see "As built". Every figure is a placeholder.

Summary: Who is engaged in a fight ashore, and an attack that fails: a side's fire counts no more men than a multiple of the enemy's in the hex; an attack whose weight of fire is the weaker fails that day and costs its men extra fatigue, so the tired stop sooner; the attack order says whether spent attackers hold or fall back, and with no instruction they hold where they can be fed and else fall back. Placeholder figures

Standing: Confirmed by the designer as revised, Nov 14, 2026. Built Nov 14, 2026 (scenario 33)

## What the baseline showed

Decision 0093 said before the run that two things would be wrong. The run shows how wrong:

- **Every man in the hex fires.** When the Ichiki echelon reaches the airfield's hex, some 10,000 Marines fire on 916 men. All 916 are lost in a day in almost every run. In the record one battalion on the river met them. At the ridge the game loses about 3,300 Japanese against a record of 600 to 1,138.
- **A failed attack never stops.** A side is tested for being beaten only when the other side attacks (decision 0090, 15). The Americans stand on the defence, so the attackers are never beaten and fight on until fatigue stops them, by which day none is left.

No figure can be tuned to mend either. The figures are one set for every fight: a hit chance low enough for 10,000 against 900 would make a fight of equals do nothing.

## Decisions

### Who is engaged

1. **A side fires with no more men than can come to grips with the enemy.** In a day's fight a side's firing men, all its units together, count up to `engagement_ratio` times the other side's fighting men ashore in the hex at the start of the fight. If it has more, each of its units' shots are cut in the same proportion. The smaller side always fires in full. *Agreed by the designer, Nov 14, 2026. The other choice was a fixed number of men for any hex.*
2. **The limit is on fire, not on who is hit.** Losses are taken from all the side's units in the hex in proportion, as now (decision 0090, 9). Which battalion stood on the river is not in the game.
3. **Everything else in the fight is as it was:** fed share, fatigue, experience, skills, defences, the chance to hit.

### An attack that fails

4. **An attack fails on a day when the attacker's fire is the weaker.** A side's weight of fire in a day's fight is the sum, over its units, of each unit's shots times its chance to hit, after the limit of decision 1. It is worked out before any die is rolled. A side that attacked and whose weight of fire is below the other side's has failed that day. If both attacked, the weaker fails; if they are equal, neither does. The defender's better chance to hit, its defences and its skill at holding ground are all in its weight, so an attack on a prepared defence needs real superiority.
5. **A failed day wears the attackers out.** Each unit on attack of the side that failed gains `failed_attack_fatigue`, beside the fatigue of a day's attack, up to 100. Nothing else follows from failing. The rule that exists does the rest: a unit whose fatigue is above the limit goes back to defend (decision 0090, 14). So a hopeless attack ends in a day or two, and one that is winning goes on. *The designer's ruling, Nov 14, 2026: losing a fifth of its men need not stop an attack; an attack fails by the odds, and the penalty is on the men's state.*

### When the attack is spent

6. **The attack order says what the unit does when it is spent.** The posture order that sets a unit to attack may carry one of two instructions: hold, or fall back. It is read when the unit goes back to defend because it is tired, in a hex where the enemy has fighting men ashore:
   - **Hold:** it stays in the hex, on the defence.
   - **Fall back:** it falls back by the rule of decision 0104: by marching, to a linked hex with a base of its side, else to one with no enemy fighting men, tired by `beaten_fatigue` and carrying nothing. If there is no such hex it stays. It does not surrender: its side was not beaten.
7. **With no instruction, the unit holds if it can be fed where it stands, and else falls back.** It can be fed if its side has a base in the hex with supply above 0, a dump its column put down included. *The designer's ruling, Nov 14, 2026: it depends on the plan and the command, and on whether they can stay: reserves that could land, or support from ships or aircraft. Otherwise they march out, tired and carrying nothing, and do not surrender.* Whether reserves or support are coming is the judgement of whoever gives the order. The staff of step 3 sets the instruction from the plan; the engine does not guess it.
8. **Being beaten is as it was** (decisions 0090, 15, and 0104). A side whose attack failed may still be beaten the same day if the other side also attacked; then decision 0104 rules, surrender included.

### What a side knows

9. **A side is told that its attack failed,** which its men know, and sees the fatigue it cost and its units going back to defend or falling back. It is not told the enemy's weight of fire: the comparison is the umpire's. **The enemy is told nothing** of any of it: it learns the attack has stopped by not being attacked, and that the men have gone by its own men's report of the hex. The cut to a side's own fire is in the breakdown of its own units' fire.

## Placeholder figures

In `ground.toml`:

| Key | Value | Frozen step 1 value | What it is |
| --- | --- | --- | --- |
| `engagement_ratio` | `3` | `1000000` | The most firing men a side counts, as a multiple of the other side's fighting men |
| `failed_attack_fatigue` | `30` | `0` | Fatigue a unit on attack gains on a day its side's attack failed |

Neither rests on a source. With the frozen values the limit never cuts and a failed day costs nothing. The instruction of decisions 6 and 7 has no figure: when it is built, every frozen scenario in which a tired attacker goes back to defend with the enemy in the hex is listed, and one whose result changes goes back to the architect before its baseline is remade.

With the live values, and the other figures as they are, the Ichiki echelon reaches the field already tired by its landing and its march. Its 916 men meet fire counted for 2,748 Marines: 27.48 shots at 0.6, a weight of 16.5 against its own of little more than 1. The attack fails, about 330 men are lost, the day costs 20 and 30 of fatigue, and the survivors stop. The record has about 800 lost of 916 in one night. The gap is for the tuning round, which now has figures that can close it: the ratio and the men a hit removes.

## What it changes in the August scenario

The attackers at the Tenaru and the ridge are given the instruction to fall back, as they did. They lose a share of their men and fall back to (2,-5), which comes first in hex order and holds a Japanese base. The stock of that hex is resized for the men who gather there; that is a data fix in decision 0101 and not part of this rule.

## Why

- **The limit rests on something a commander can reason about:** a small force cannot be shot at by an army.
- **Failure is judged by the odds,** which is what a commander weighs before he attacks, and it is paid in the state of the men, which he must then live with.
- **What the men do afterwards is a command,** with a default a player can predict.
- **All of it reuses what exists:** the fire of the fight, fatigue, the tired going back to defend, and the falling back of decision 0104.

## What it leaves out

- **Frontage and terrain:** how wide a hex's front is, and ground that narrows it.
- **Which units are engaged.** All fire a share and all take losses in proportion.
- **Morale and disorder apart from fatigue.**
- **A defender that counter-attacks by itself** when an attack fails.
- **Pursuit.** The victor fires on men who have not yet left only if it is ordered to attack (decision 0104, 6).
- **The engine judging reserves or support.** That is the order's.

## What it costs

- **New state:** the instruction a unit's attack order carries, in the hash and named beside it; the hash version and the rules label are raised and the generated files remade.
- **New events** with their rows in `side_events.rs`; factors in the fire breakdown, named only where the limit cuts.
- **The order gains a field,** in scenario files, the agent interface and its schema.
- **The live baselines of every scenario with a fight ashore** change, since the live figures change.
- **The rulebook:** chapter 7 ("Shots and chance", "Fatigue", "The tired stop attacking", "A side beaten", "Posture") and the glossary, in the same pull request.

## Rule card

| | |
| --- | --- |
| Reads | Each side's firing men and fighting men ashore in the hex; each unit's shots and chance; each unit's posture, its order's instruction and its fatigue; the bases of a side in the hex and their supply; the land links; the figures |
| Writes | Each unit's shots; a unit's fatigue, posture and march; the events |

## Questions for the designer

Answered on Nov 14, 2026.

1. **Decision 1:** a side's fire is limited to a multiple of the enemy's men in the hex. Agreed.
2. **Decisions 4 and 5:** the first draft broke an attack on a day's losses. The designer asked for something nearer to how attacks fail: by the odds, with the penalty on the men. Revised and agreed.
3. **Decisions 6 and 7:** the first draft made every broken attacker with no base fall back. The designer ruled that it depends on the command and on whether the men can stay. Revised and agreed.

## As built

Built on Nov 14, 2026 (scenario 33, state version 21, rules label 4; the hand calculation is in `tests/scenario_33_engaged_and_failed.rs`, on the placeholder map `data/map/placeholder_engaged.toml`). Every number of the hand calculation matched the run. The code is in `crates/sim-core/src/ground_combat.rs` (the fight, `tire_or_rest`, `attack_spent`, `fall_back_hex`). Choices the record did not make:

- **The order of the phase** is the one the hand calculation states: the marches, the fight, each unit's fatigue (with the fatigue of a failed day), the tired back to defend, what a spent attacker does (in unit order), the test of a side beaten, the bases changing side and a beaten side's men falling back, the ground reports.
- **The fight is worked in two passes.** First every unit's firing men, shots and chance, for both sides, with no die; then each side's limit and weight of fire; then the dice and the fire events, in side and unit order as before. The limit is `engagement_ratio` times the other side's fighting men ashore in the hex at the start of the fight (working men not counted, as for a contested hex); a side's shots are cut only where its firing men are above it, each unit's by the same proportion (shots times the limit over the side's firing men). The loader refuses a ratio below 1, so the smaller side is never cut. The `GroundFire` event keeps the uncut firing men and gives the cut shots.
- **The failure is an event,** `GroundAttackFailed` (side, hex, both weights), recorded after the day's fire events and before the losses, citing the fight and both sides' fire, with a breakdown of the two weights. It is recorded whatever the figures, so the frozen runs record it too; with `failed_attack_fatigue` at 0 it changes nothing else.
- **A unit whose shots are 0** (fed share or fatigue leaves no firing man): a side that attacked with a weight of 0 has failed if the other's is above 0, as decision 4 reads. The fatigue of a failed day is given to the units on attack that fired (shots above 0), beside the day's attack fatigue: a unit on attack that fired nothing gains neither, as before. The fatigue event cites the failure, and its breakdown names `failed_attack_fatigue`.
- **The instruction in an order** is a field of the posture order: `when_spent = "hold"` or `"fall_back"` in a scenario's `set_posture` order, and the same in the agent's `issue_orders` (schema and MCP text). Only an attack order takes it: a defend order with one is refused by the loader and, from the agent, by the engine (`instruction_needs_attack`). A unit on attack by its scenario entry has none. The state is `LandUnit.when_spent`, hashed after `working` (0 for none). It is spent with the posture order: when the unit goes back to defend, when it falls back beaten, and whenever another posture order replaces it.
- **"The enemy has fighting men ashore in the hex"** is read at the moment the unit goes back to defend, after the day's losses: the men ashore of the enemy's fighting units among those in the hex at the start of the phase (after the marches). A unit already marching is left as it is, as a beaten side's is (decision 0104).
- **Hold** changes nothing: the unit is on the defence already, so it has no event of its own beyond `LandUnitBackToDefend`. **Fall back** takes the hex by decision 0104's rule (`fall_back_hex`, shared with a beaten side), adds `beaten_fatigue`, and records `SpentUnitFellBack` (unit, side, from, to, men, fatigue), citing the unit's going back to defend; the march's cause is that event, so it carries nothing and forms no base. A new event and not `UnitFellBack`, which cites a defeat and which a side's briefing words as one. With no hex to go to the unit stays, on the defence, with no fatigue added and no event; it does not surrender.
- **A choice a side can ask** (decision 0100). Whether a spent attacker holds or falls back reads only the side's own things (its order's instruction and its own bases in its hex and their supply), so it is a function the engine and a side's view both call: `GroundQuestion::AttackSpent { unit, hex }`, answered `hold` or `fall_back` (`when_attack_spent`, `fed_where_it_stands`); the agent's `measure choices` answers it as `attack_spent`. Where the unit falls back to reads where the enemy's men truly are (decision 2 of 0104), so it stays with the rules and is not asked. The question carries the hex the unit stands in when the engine asks, since a unit can march in and attack the same day. One in-step case was found and is named in `tests/choices_asked_and_run.rs`: a column that arrives in the hex in the same step puts down a dump before the fight, so the engine finds a base of the side with supply where the view, before the march, found none (the full slice on days 16 and 37).
- **What a side sees.** The side that attacked is given `GroundAttackFailed` cut to the hex, with no weight and no breakdown; the enemy is given nothing of it, nor of `SpentUnitFellBack`, which its own side is given whole. The fire breakdown names `shots_before_limit`, `side_firing_men` and `engaged_men` only where the limit cuts, and a side is shown them for its own units (decision 9). `engaged_men` is a multiple of the enemy's fighting men; the cut shots already imply it, given the side's own firing men, so the factor shows nothing the event does not. The agent's briefing lists `attacks_failed_yesterday` (place and event), words a spent unit's falling back as "its attack spent" (a `reason` of `beaten` or `attack_spent` on `fell_back_yesterday` and on the unit line's `falling_back`), and shows a standing attack order's `when_spent` on the unit line. The schemas stay at version 3: fields only.
- **An earlier property amended, by the architect, following decision 9.** Scenario 23's property that the enemy sees the same fight whatever the other side's fatigue and units failed on one matched seed (120): the Allied attack failed on day 3 in one world and not in the other, and the attacker is told that its attack failed, which reads the enemy's weight of fire. That is decision 9's disclosure, as a claim is one. The property now holds where the hits are the same and the attacker's attack failed on the same days; its test skips a seed unless those days match. Scenario 33 has the same property (two worlds differing only in the defender's fatigue and units at P), and a test that the attacker is given the hex and the day only, no weight, no men and no unit of the enemy, and the enemy nothing. No property of scenarios 28, 29, 30 or 32 failed.
- **The frozen scenarios checked.** On the frozen figures, a tired attacker goes back to defend with enemy fighting men in its hex in scenarios 23 (unit 2, day 4, at (0,0)), 25 (unit 1, day 4, at (1,0)), 28 (units 3 and 4, day 4), 29 (units 9 and 11, day 4) and 30 (unit 8, day 4). In each its side has a base in the hex with supply above 0, so decision 7's default holds it, as before: no result changes. Scenario 32 has no such unit. No hand calculation changed; every frozen-figure test passes as it was.
- **Regenerated.** Every run baseline (hashes; and in their events and results scenarios 7, 23, 24, 25, 28, 29, 30 and 32, which fight ashore on the live figures), the recorded games (saves only), the batch fixture of scenario 6 and the pinned hashes of scenarios 10 and 17a. The rules label rose to 4 because every live fight ashore changes. The slice's attack orders at the Tenaru and the ridge (the Ichiki echelon, the five Kawaguchi parts and the Oka group) carry `when_spent = "fall_back"`; nothing else in the slice changed. Its batch after this decision is `docs/validation/full-slice-39-days-after-0105.md`.
