# 0205: Hunger takes time

Status: a plan by the architect, written on Nov 21, 2026 on the designer's question at the table: what "lost 187 men for want of food" means, on the first night his marines were ashore, and his word to plan a gentler rule. It was read by a reviewer who had not seen it written, and this is the plan with that reading folded in: it is simpler than the first. Its hand calculation is written next.

Summary: Today a twentieth of the men a base cannot feed are lost every night, from the first night ashore. The record says otherwise: men landed carrying food, lived for weeks on short rations with no such loss, and died of hunger only after weeks on almost nothing. This puts the rations men land with into the base their landing forms, counts for each unit the nights it goes hungry, and takes men only from a unit that has been hungry for many nights. It runs where a scenario's ground figures give its figures, so no game on file moves; the table plays copies of its scenarios that have them.

Standing: Planned on Nov 21, 2026; its review folded in; waits for its hand calculation and its build.

## Why

What is so today (rules chapter 7, The men's supply):

- **Each night the men ashore in a hex are charged a daily supply** from the stock of their side's bases there. The men the stock covers are fed and the rest are unfed. Every unit of the side in the hex has the one fed share, the fed over the charged.
- **A share of the unfed is lost that night,** one man at least: `unfed_loss_share_per_day`, 0.05, a placeholder that "rests on no source". It applies from the first night.
- **The fed share already weakens a hungry unit:** it cuts the men who fire and the work done, and stops training.
- **A landing forms a base and puts no food in it.** Men who land where no supply has yet been landed are unfed that night.
- **Men who march by order already carry food:** `march_supply_days`, 5, put down where they arrive. It rests on no source.

What the record holds (`data/oob/land_unit_facts.csv`):

- **Men landed with rations.** A company of the 5th Marines "landed with three days' rations" at Tulagi (row 538). It is the one row of men landing.
- **Men setting out on a march carried three to five days.** The 2d Division "carrying only five days' rations" (33, 34), the 2d Raider Battalion "four days of canned rations" (877), and a force taken to be the Kawaguchi Detachment "only three day's ration" (867, of low confidence). These bear on the march's figure, not on a landing.
- **Weeks on two meals a day cost no such loss.** The 1st Marine Division went to two meals a day in mid August, on the 12th or the 15th by the source, and stayed on them about six weeks (403 to 406). No source read gives men lost to hunger in that time.
- **Half rations weakened men.** The 2d Division's troops were "weakened on their half ration of rice" in October (31).
- **Men died of hunger after about three weeks on almost nothing.** On 5 and 6 October marines found in a Japanese bivouac "many who had died from hunger" (188, low confidence, no count), about three weeks after the fight on the ridge, which its men had begun with three days' rations. In December the Japanese were on "less than one-third the regular daily allowance" (137). The counts of Japanese dead that include hunger (104, 178, 179) mix it with disease and give no rate.

So a rule that takes a twentieth of the unfed on the first night is wrong in kind, not only in size.

## The rules

### Where it runs

1. **The rule runs where a scenario's ground figures give its three figures** (rule 9), as the men a hit removes and the errors of a report already do. A figures file without them plays as today.
2. **The live `ground.toml` does not gain them.** Its bytes are in the saves of recorded games, which refuse to load when it changes. The figures go in a ground file of their own, which a scenario names.
3. **The table plays copies of its scenarios that name that file.** The copies are made by a script from the files on file, changed in that one line, and checked as the generated scenario files are; `table/scenarios.json` points at them. The files the proofs play are not touched.

### Rations landed

4. **A landing that forms a base puts the men's rations into it:** the men aboard the ships of the force that forms the base, at the moment it is formed, times `rations_landed_nights`, times the daily charge for a man, as supply in the stock of the base formed. However many pulses the men take to go ashore, the rations are put down once, then. It is the mechanism of the supply a marching unit puts down where it arrives, used at a landing.
5. **Only the force that forms the base brings rations.** A second force landing in the same pulse, and men landed later at a base that stands, bring none; that base feeds them or it does not. So men cannot be taken off and landed again to make food.
6. **Nothing else of the feeding changes.** The fed share stays one for every unit of the side in the hex, and `MenFed` is as it is.

### Nights of hunger

7. **A unit's nights of hunger are a count, new state of the unit.** On a night its fed share is below `hungry_below_share` the count rises by one. On a night it is fed in full the count falls by one, not below nothing. On any other night it stays. A unit with no man ashore keeps its count; a unit made by dividing another takes the count of the unit it came from. A scenario may give a unit the count it starts with; without it the count is nothing.
8. **Men are lost to hunger only from units whose count is above `hunger_nights_before_loss`.** The counts are brought up for the night first, so with the figure at 14 the first men are lost on a unit's fifteenth hungry night. The loss in a hex is worked as today, on the men of those units alone: their men times one less the fed share, cut, are their unfed; the loss is their unfed times `unfed_loss_share_per_day`, cut, and one man at least where any of them is unfed; it is taken from those units in proportion by today's rule. With no unit past the count nobody is lost.

## Figures

9. All are placeholders in the game's sense, each with what it rests on.

| Figure | Value | Rests on |
| --- | --- | --- |
| `rations_landed_nights` | 3 | one row: three days' rations at a landing (538) |
| `hungry_below_share` | 0.5 | a judgement: half rations weakened men (31); two meals of three are above it and cost no loss (403 to 406) |
| `hunger_nights_before_loss` | 14 | a judgement between two rows: no loss in six weeks on two meals; "many" dead about three weeks after a force with three days' rations broke (188) |
| `unfed_loss_share_per_day` | 0.05, unchanged | rests on no source, as today |

The march's `march_supply_days` stays 5; the rows of men setting out with three, four and five days are noted against it.

## What is recorded and who knows

10. **The rations a landing brings are recorded** with the base formed: the men, the nights and the tons, citing the landing. **A unit's count changing is recorded** in an event for the unit and the night, citing that night's `MenFed`, and only when the count changes. `MenLostUnfed` gains the factors of rule 8. Each side sees its own and nothing of the enemy's.
11. **One field is new state of a land unit,** hashed in its place and named, with the layout raised and the golden files remade. A side sees it for its own units; the table's card says "Hungry 5 nights" where it is above nothing. The rules label does not rise, since no game on file plays differently.

## Rule card

Reads: the men ashore, the bases' stock, each unit's fed share and its nights of hunger. Writes: the stock of a base formed by a landing, the count, the men lost. Draws no random number.

## Games on file

None moves: the figures are in a file no scenario on file names, and the live figures are not changed. The layout rises, so the golden files and the daily hashes are remade for the layout alone, as at every new field. The 39 days and the proofs keep the old rule until the designer says otherwise; that is his call, since their measures count men lost unfed.

## What it does not do

- It does not model sickness. The record's sick lists are far larger than any loss to hunger and are a rule of their own.
- It does not let a unit forage or eat captured stores, which the record says fed the marines in August (405, 707). A base taken with its stock already gives that stock to the taker.
- It does not change the daily supply a man costs, nor the fed share's effect on a fight, on work or on training.
- It does not give rations to a unit that starts ashore, that enters ashore on its day, or that lands at a base which stands.

## What it costs

- **One field of state for each land unit, two events, and a landing that creates supply.** The supply is small: 3,000 men bring 45 tons at the live charge.
- **A table that plays by a rule the proofs do not,** from copies of the scenarios. A game at the table and the same scenario in a closed proof will differ ashore until the proofs take the rule.
- **Rulebook:** chapter 7 (Landing men and a base formed by a landing, The men's supply, The unfed are lost, A unit divided); the glossary's "Fed, unfed"; chapter 1's list of who sees what.

## Questions for the designer

1. **Should the 39 days and the other proofs take this rule?** It would move their measures of men lost. The architect proposes to build it, play it at the table, and bring what would move before any proof is run again.

## How the plan changed on review

A reviewer who had not seen the plan read it against the feeding's code, the bridge, the rows and the engineering pages on Nov 21, 2026.

- **The first plan gave each unit rations of its own and a fed share of its own.** The fight, training, the entity answer and the briefing all read one fed share for a hex and cite the hex's feeding; a share for each unit would have broken each of them. The march already puts carried food down as supply where men arrive. Rations at a landing now go the same way, into the base the landing forms, and the fed share stays one for the hex (rules 4 to 6).
- **The table cannot give a scenario a key:** it passes a scenario's path and nothing more, and its 39-day games are the proof's own files. The rule had also been gated twice, by figures and by a key, where the house has one or the other. It is now gated by figures in a ground file of its own, and the table plays copies (rules 1 to 3).
- **The count could be held at one for ever** by a full meal every other night on a quarter ration. It now falls only on a night fed in full (rule 7).
- **The loss was not defined for a unit, and the one man at least was dropped without a word.** Rule 8 defines it and keeps him.
- **The rows said less than the plan claimed.** One row is of a landing; three are of marches. One row does date deaths from hunger, which the plan had said none did. The figures' reasons are rewritten from the rows as they are.
