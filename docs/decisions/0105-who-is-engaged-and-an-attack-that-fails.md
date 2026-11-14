# 0105: Who is engaged in a fight ashore, and an attack that fails

Status: a plan by the architect, written on Nov 14, 2026, after the first baseline of the lengthened slice (`docs/validation/full-slice-39-days-baseline.md`). The designer approved planning these two changes before any tuning. Needs review: the three questions at the end are the designer's. Nothing is built. Every figure is a placeholder.

## What the baseline showed

Decision 0093 said before the run that two things would be wrong. The run shows how wrong:

- **Every man in the hex fires.** When the Ichiki echelon reaches the airfield's hex, some 10,000 Marines fire on 916 men. All 916 are lost in a day in almost every run. In the record one battalion on the river met them. At the ridge the game loses about 3,300 Japanese against a record of 600 to 1,138.
- **A failed attack never stops.** A side is tested for being beaten only when the other side attacks (decision 0090, 15). The Americans stand on the defence, so the attackers are never beaten and fight on until fatigue stops them, by which day none is left.

No figure can be tuned to mend either. The figures are one set for every fight: a hit chance low enough for 10,000 against 900 would make a fight of equals do nothing.

## Decisions

### Who is engaged

1. **A side fires with no more men than can come to grips with the enemy.** In a day's fight a side's firing men, all its units together, count up to `engagement_ratio` times the other side's fighting men ashore in the hex at the start of the fight. If it has more, each of its units' shots are cut in the same proportion. The smaller side always fires in full.
2. **The limit is on fire, not on who is hit.** Losses are taken from all the side's units in the hex in proportion, as now (decision 0090, 9). Which battalion stood on the river is not in the game.
3. **Everything else in the fight is as it was:** fed share, fatigue, experience, skills, defences, the chance to hit.

### An attack that fails

4. **An attack breaks when it costs too much in a day.** After the day's losses, a side that attacked and lost at least `attack_break_share` of the fighting men it had ashore in the hex before the fight has its attack broken. Its units on attack go back to defend, as tired units do (decision 0090, 14), with their posture order spent.
5. **A broken attack that has no base to stand on falls back.** If the side holds no base in the hex, or only a dump (decision 6), its units there fall back by the rule of decision 0104: by marching, to a linked hex with a base of their side, else to one with no enemy fighting men, tired by `beaten_fatigue` and carrying nothing. If there is no such hex they stay where they are, on the defence. They do not surrender: the side was not beaten.
6. **A base formed by a march is a dump, and is marked so.** It holds what the column carried and nothing else (decision 0094, 10). It does not count as a base to stand on in decision 5. In every other way it is a base as before.
7. **A side that holds a real base in the hex stays** when its attack breaks. It goes on the defence beside its base, and may be ordered to attack again.
8. **Being beaten is as it was** (decisions 0090, 15, and 0104). A side whose attack broke may still be beaten the same day if the other side also attacked; then decision 0104 rules, surrender included.

### What a side knows

9. **A side sees its own attack break and its units fall back,** with the losses that broke it. **The enemy is told nothing** of either: it learns the attack has stopped by not being attacked, and that the men have gone by its own men's report of the hex. The cap on its own fire is in the breakdown of its own units' fire, which it sees as it sees that fire today.

## Placeholder figures

In `ground.toml`:

| Key | Value | Frozen step 1 value | What it is |
| --- | --- | --- | --- |
| `engagement_ratio` | `3` | `1000000` | The most firing men a side counts, as a multiple of the other side's fighting men |
| `attack_break_share` | `"0.2"` | `1` | The share of its fighting men an attacker loses in a day at which its attack breaks |

Neither rests on a source. The frozen values make both rules do nothing, so every earlier hand calculation stands: no frozen scenario has a side a million times the other, and at a share of 1 an attack breaks only when every man is lost.

With the live values, and the other figures as they are, the Ichiki echelon (916 men, 412 of them firing) meets fire counted for 2,748 Marines: 27.48 shots at 0.6, about 16 hits, some 330 men lost in the day. That is over a fifth, so the attack breaks and the survivors fall back west. The record has about 800 lost of 916 in one night. The gap is for the tuning round, which now has figures that can close it: the ratio, the men a hit removes, the share.

## What it changes in the August scenario

The attackers at the Tenaru and the ridge lose a share of their men and fall back to (2,-5), which comes first in hex order and holds a Japanese base. They are fed there if its stock allows. The stock of that hex is resized for the men who gather there; that is a data fix in decision 0101 and not part of this rule.

## Why

- **The limit rests on something a commander can reason about:** a small force cannot be shot at by an army.
- **An attack that can fail and be survived** makes attacking a choice with a price, and leaves the attacker something to preserve.
- **Both reuse what exists:** the fire of the fight, the going back to defend, and the falling back of decision 0104.

## What it leaves out

- **Frontage and terrain:** how wide a hex's front is, and ground that narrows it.
- **Which units are engaged.** All fire a share and all take losses in proportion.
- **A defender that counter-attacks by itself** when an attack breaks.
- **Pursuit.** The victor fires on men who have not yet left only if it is ordered to attack (decision 0104, 6).
- **An attack called off by its commander before it breaks:** that is an order, as today.

## What it costs

- **New state:** a base's mark as a dump, in the hash and named beside it; the hash version and the rules label are raised and the generated files remade.
- **Two new events** with their rows in `side_events.rs`; a factor in the fire breakdown, named only where the limit cuts.
- **The live baselines of every scenario with a fight ashore** change, since the live figures change. The frozen ones do not.
- **The rulebook:** chapter 7 ("Shots and chance", "The tired stop attacking", "A side beaten", "Marching overland") and the glossary, in the same pull request.

## Rule card

| | |
| --- | --- |
| Reads | Each side's firing men and fighting men ashore in the hex; the day's losses; each unit's posture; the bases of a side in the hex and their mark; the land links; the two figures |
| Writes | Each unit's shots; a unit's posture, march and fatigue; a base's mark as a dump; the events |

## Questions for the designer

1. **Decision 1:** a side's fire is limited to a multiple of the enemy's men in the hex. The other choice is a fixed number of men who can fire in any one hex, whoever the enemy is.
2. **Decision 4:** an attack breaks on the losses of one day. The other choice is the losses added up over the whole attack, which needs the game to remember when each attack began.
3. **Decision 5:** after a broken attack, men with no base in the hex fall back. The other choice is that they stay in the hex on the defence until ordered away or beaten.
