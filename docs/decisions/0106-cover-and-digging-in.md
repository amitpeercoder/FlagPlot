# 0106: Cover, and men who dig in

Status: a plan by the architect, written on Nov 14, 2026, after the designer said that well-prepared defenders were massively bombed and did not suffer heavily for it. Needs review: the three questions at the end are the designer's. Nothing is built. Every figure is a placeholder.

## The gap

A base's defences, built by work (decision 0099, 4), do one thing: they raise the chance to hit of a side that defends there. They do not shelter anyone. A man behind full defences is lost to a bomb, a shell or a rifle as easily as a man in the open (decisions 0092 and 0090). So bombardment before an assault is always worth its shells, a defender has no reason to dig beyond shooting better, and a prepared position cannot be what it was: a place that takes a great weight of fire and is still held when the firing stops.

Two more things are missing. Every defence is one kind, built over weeks by labour at a base: troops cannot scrape holes where they stand. And nothing can be dug where a side has no base.

## Decisions

### Cover

1. **Cover is a number from 0 to 100 for a unit in a hex.** It is the higher of two things: the defences of its side's lowest-numbered base in the hex (decision 0099), and the unit's own fieldworks (decision 5). A unit that is marching, or on attack, has no cover: it is out of its holes.
2. **Cover saves men from bombs and shells.** The men a hit on men removes (decision 0092) are cut for each unit by its cover: the men it would lose, times 1 less its cover over 100 times `cover_from_fire`, cut to a whole man. At full cover and the placeholder figure, one man is lost where ten were.
3. **Cover saves men in the fight.** The men a side's hits remove from a unit that defends (decision 0090) are cut in the same way by `cover_in_fight`. The defences' effect on the defender's own chance to hit stays as it is.
4. **Cover does not shelter aircraft, supply or the airfield.** They are hit as now. Men are what a trench holds.

### Fieldworks

5. **Men dig in where they stand.** A fighting unit on defend that neither marched nor fought nor worked that day gains `fieldworks_per_day` of fieldworks, up to `fieldworks_limit`. It needs no base, no order and no labourers, and it does not stop a unit training or rehearsing: digging a hole is not a day's work. Working units do not dig for themselves; they are sheltered by a base's defences if it has them.
6. **Fieldworks are the unit's and are lost when it leaves them.** They go to 0 when the unit marches, falls back, is taken aboard, or is set to attack. A unit made by division starts with those of the unit it left.
7. **Improvised and prepared are two things.** Fieldworks are quick and shallow: a few days to the limit, and the limit is low. A base's defences are slow and deep: weeks of labour, up to 100, and they stay with the base whoever holds it. A unit is sheltered by the better of the two.

### Wearing cover down

8. **Nothing wears cover down.** Bombs, shells and assault leave a base's defences and a unit's fieldworks as they are. That is the simplest rule, and the record this decision answers to is of positions that were not reduced by bombardment. A rule for breaching and for heavy shells is left for when a battle asks for it.

### What a side knows

9. **A side sees its own units' cover whole.** The enemy sees none of it: a base's defences and a unit's fieldworks are not in what it is told, and a raid's claim says nothing of them. A side learns that an enemy is well dug in the way it learns anything of the enemy ashore: by what its own fire and its own attacks then cost and achieve.

## Placeholder figures

In `ground.toml`:

| Key | Value | Frozen step 1 value | What it is |
| --- | --- | --- | --- |
| `cover_from_fire` | `"0.9"` | `0` | The share of losses to bombs and shells that full cover saves |
| `cover_in_fight` | `"0.5"` | `0` | The share of losses in the fight that full cover saves a defender |
| `fieldworks_per_day` | `10` | `0` | Fieldworks a unit gains in a day of standing on the defence |
| `fieldworks_limit` | `30` | `0` | The most fieldworks a unit can dig for itself |

None rests on a source. The frozen values make the rule do nothing, so every earlier hand calculation stands. With the live values a unit that has stood three days has fieldworks of 30: it loses 27 in 100 fewer men to shelling and 15 in 100 fewer in a fight. Behind a base's full defences it loses a tenth of what shelling would have cost it and half of what an assault would.

## What it changes in the August scenario

The Marines at Lunga dig in from the day the field is theirs and have fieldworks of 30 within three days, before any attack reaches them. The Japanese shellings and raids of the period cost them fewer men. The Japanese garrisons of 7 August have what the scenario gives their bases; no row sizes that, so it stays 0 until research gives one, and the scenario says so.

## Why

- **Bombardment becomes a judgement.** Against men in the open it pays; against a prepared position it buys little, and a commander must decide whether to spend the days and the shells.
- **Time on the defence is worth something by itself,** which is what a commander who gets ashore first has.
- **Labour buys what troops cannot dig:** the deep defences only working units build.
- **It is one number read in two places** that already exist, the hit on men and the losses of the fight.

## What it leaves out

- **Breaching, and shells or bombs heavy enough to reduce defences.**
- **Cover for aircraft, supply, guns and the airfield.**
- **Terrain as cover:** jungle, caves and ridges.
- **Fieldworks carried over** when a unit moves a short way.
- **The enemy learning of defences** by reconnaissance.

## What it costs

- **New state:** a unit's fieldworks, in the hash and named beside it; the hash version and the rules label are raised and the generated files remade.
- **A factor in two breakdowns** (men lost to fire, men lost in the fight), named only where cover acts; an event for fieldworks gained and lost, with its row in `side_events.rs`.
- **A scenario key** for a unit's fieldworks at the start.
- **The live baselines of every scenario with men ashore under fire or in a fight** change.
- **The rulebook:** chapter 7 ("Losses", "Men lost to fire", "Units that work") and chapter 5, and the glossary, in the same pull request. It is built after decision 0105, which changes the same fight.

## Rule card

| | |
| --- | --- |
| Reads | A unit's posture, march, activity and fieldworks; the defences of its side's base in the hex; the men a hit or a fight would remove; the four figures |
| Writes | A unit's fieldworks; the men a unit loses to fire and in the fight; the events |

## Questions for the designer

1. **Decisions 2 and 3:** cover saves men both from bombs and shells and in the fight ashore. The other choice is from bombs and shells only, leaving the fight as it is.
2. **Decision 5:** troops dig shallow fieldworks for themselves, anywhere, up to a low limit. The other choice is that all cover comes from a base's defences, built by work.
3. **Decision 8:** nothing wears cover down for now. The other choice is that hits on men also wear down fieldworks and defences, so a long bombardment slowly tells.
