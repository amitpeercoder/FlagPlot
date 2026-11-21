# 0205: Hunger takes time

Status: a plan by the architect, written on Nov 21, 2026 on the designer's question at the table: what "lost 187 men for want of food" means, on the first night his marines were ashore, and his word to plan a gentler rule. To be read by a reviewer who has not seen it written, and amended on that reading, before its hand calculation is written and before any of it is built.

Summary: Today a twentieth of the men a base cannot feed are lost every night, from the first night ashore. The record says otherwise: men landed carrying rations of their own, lived for weeks on short rations with no such loss, and died of hunger only after a long time on very little. This gives a unit ashore the rations it carried, counts the nights it goes truly hungry, and takes men only after many such nights. It runs only where a scenario asks for it, so no game on file moves; the table asks for it.

Standing: Planned on Nov 21, 2026; waits for its review.

## Why

What is so today (rules chapter 7, The men's supply):

- **Each night the men ashore in a hex are charged a daily supply** from the stock of their side's bases there. The men the stock covers are fed and the rest are unfed.
- **A share of the unfed is lost that night:** `unfed_loss_share_per_day`, 0.05, a placeholder that "rests on no source". It applies from the first night.
- **A unit's fed share,** the fed over the charged, lowers the men who fire in a fight and stops training. That already makes a hungry unit weaker.
- **A unit carries nothing of its own.** Men who land where no stock has yet been landed are unfed that night.

What the record holds (`data/oob/land_unit_facts.csv`):

- **Men landed with rations.** A company of the 5th Marines "landed with three days' rations" (row 538). The Kawaguchi Detachment "carried only three day's ration" (867). The 2d Division set out "carrying only five days' rations" (33, 34). The 2d Raider Battalion "initially carried four days of canned rations" (877).
- **Weeks on short rations cost no such loss.** The 1st Marine Division went to two meals a day on 12 August and stayed on them about six weeks (rows 403 to 408, 706 to 709). No source read gives men lost to hunger in that time.
- **Half rations weakened men.** The 2d Division's troops were "weakened on their half ration of rice" in October (31, 32).
- **Hunger that killed came late and on very little:** the Japanese on Guadalcanal in December on "less than one-third the regular daily allowance" (137). No row gives a count of men who died of it or a rate. That is a gap, and it is why the figures below are placeholders.

So a rule that takes a twentieth of the unfed on the first night is wrong in kind, not only in size.

## The rules

### Where it runs

1. **The rule runs where the ground figures give its three figures** (rule 12). A figures file without them plays as today. The frozen figures do not have them.
2. **The live figures gain them only through a scenario's own word:** a scenario key, `hunger_takes_time = true`, without which the three figures are not read. No scenario on file has the key. The table gives it to the games it starts, as it gives the key for stopping inside a day.

### Rations carried

3. **A land unit carries rations of its own,** counted in nights: `rations_carried_nights`. It is state of the unit. It is set full when men of the unit are put ashore by a landing or an unloading and the unit had no man ashore before; a unit that starts a scenario ashore starts with none.
4. **On a night its hex's stock cannot feed every man,** the units there that have rations left eat their own. Each such unit is fed in full, its fed share is 1, and its rations fall by a night. Its men are left out of the charge, and what the bases paid is shared among the men of the other units, by today's rule.
5. **On a night the stock feeds every man,** nobody's rations fall.

### Nights of hunger

6. **A unit is hungry on a night its fed share, after rule 4, is below `hungry_below_share`.** Its nights of hunger, a count that is state of the unit, rise by one. On a night it is not hungry the count falls by one, not below nothing.
7. **Men are lost to hunger only in a unit whose nights of hunger are above `hunger_nights_before_loss`.** The loss is today's: the unit's unfed men times `unfed_loss_share_per_day`, cut to a whole man. Where the rule runs, the loss is worked for each unit from its own unfed men, and no longer for the hex and then shared.
8. **A unit below the line but not yet past the count loses nobody.** It is weaker by its fed share, as today.
9. **Men aboard ships** do not eat and are not hungry, as today. A unit with no man ashore keeps its counts.

### What is recorded and who knows

10. **`MenFed` gains, in its breakdown, the men who ate their own rations.** A unit's rations falling, and its nights of hunger changing, are recorded in one event for the unit and the night, `UnitRations`, citing `MenFed`. `MenLostUnfed` gains the factors of rule 7. Each side sees its own and nothing of the enemy's.
11. **Two fields are new state of a land unit,** hashed in their place and named, with the layout raised and the golden files remade. A side sees them for its own units: the entity answer and the table's card say "Rations of its own: 2 nights" and "Hungry 5 nights".

## Figures

12. All are placeholders in the game's sense, each with what it rests on.

| Figure | Value | Rests on |
| --- | --- | --- |
| `rations_carried_nights` | 3 | the least of four rows: 3, 3, 4 and 5 days (538, 867, 877, 33) |
| `hungry_below_share` | 0.5 | a judgement: half rations weakened men (31, 32); two meals of three cost no loss and are above it |
| `hunger_nights_before_loss` | 14 | a judgement: six weeks on two meals cost no loss; no row dates the first deaths on a third of a ration |
| `unfed_loss_share_per_day` | 0.05, unchanged | rests on no source, as today |

## Rule card

Reads: the men ashore, the bases' stock, each unit's fed share, rations and nights of hunger. Writes: the two new fields, the fed share, the men lost. Draws no random number.

## Games on file

None moves: no scenario on file has the key, and without it the rule is today's. The layout rises, so the golden files and saves are remade for the layout alone. The 39 days and the proofs keep the old rule until the designer says otherwise; that is his call, since their measures count men lost unfed.

## What it does not do

- It does not model sickness. The record's sick lists are far larger than any loss to hunger and are a rule of their own.
- It does not let a unit forage or eat captured stores, which the record says fed the marines in August (rows 405, 707). A base taken with its stock already gives that stock to the taker.
- It does not change the daily supply a man costs, nor the fed share's effect on a fight.
- It does not give a starting unit rations. A scenario that wants a unit ashore with rations lands it.

## What it costs

- **Two fields of state for each land unit,** an event, and the feeding worked unit by unit in a hungry hex where today it is worked for the hex.
- **A table that plays by a rule the proofs do not.** A game at the table and the same scenario in a closed proof will differ ashore until the proofs take the rule.
- **Rulebook:** chapter 7, The men's supply; the glossary's "Fed, unfed"; chapter 1's list of who sees what.

## Questions for the designer

1. **Should the 39 days and the other proofs take this rule?** It would move their measures of men lost. The architect proposes to build it, play it at the table, and bring what would move before any proof is run again.
