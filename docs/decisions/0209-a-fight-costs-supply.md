# 0209: A fight costs supply

Status: a plan by the architect, written on Nov 21, 2026. Decision 0208, as its figure was moved on research, left a unit that is cut off from supply fighting at nearly full strength until its men begin to die: nothing models the ammunition of men ashore. The designer asked for it to be planned. It is to be read by a reviewer who has not seen it written before anything is built.

Summary: Today a fight ashore uses nothing. This makes a day's fight charge supply to the bases of each side in the hex, by the men who fire, and cuts the fire of a side whose stock cannot cover it to what its men carry. A landing brings some days of fighting with its rations. Supply stays one stock: the same tons feed the men and feed the guns. It runs where a scenario's ground figures give its three figures, so no game on file moves.

Standing: Planned on Nov 21, 2026; to be reviewed.

## Why

What is so today:

- **A fight ashore charges nothing.** A side's shots are its firing men over `men_per_shot` (rules chapter 7); no stock is read or written.
- **Supply at a base is one stock,** charged each night for the men ashore (`daily_supply_t_per_man`). A base's `ammunition_t` is for ships that rearm in port (decision 0122), is set by the scenario and is no cargo.
- **Since decision 0208's figure moved, a night on nothing costs a unit a tenth of its fire.** So a unit with no supply at all fires nine men in ten for a fortnight.

What the record holds (`docs/research/ammunition-ashore.md`; rows 15082 to 15240 of `land_unit_facts.csv`, all unconfirmed):

- **A force landed with days of fighting, not weeks.** The 1st Marine Division had about four units of fire ashore on 9 August (rows on file, with 15098 and 15101). The Army's manual of 1941 puts a day of attack or defence of a position at 1.5 units of fire on the first day and 1.0 after, and an inactive day at 0.2 (15086, read by the architect on the page image), figures it says rest on the First World War.
- **Men who carried their own had a day or two.** A Japanese rifleman on Tulagi had 120 rounds (15105); one Australian company used 100 rounds a man and 1,200 grenades in a day (15186).
- **The weight is in the guns.** A regiment's small arms and mortars are 72.4 tons to a unit of fire (15083); twelve 105-mm howitzers are 68 tons (15084). On the night of 23 October the 11th Marines fired 6,164 rounds of 75-mm and 2,719 of 105-mm (15123).
- **What was used was far below the manual on most days.** The Marines went 42 days on their four units of fire, through the Tenaru and the ridge, short of particular kinds and not exhausted.
- **A unit short of ammunition rationed its fire and held; a unit out of it withdrew where it could** (15119, 15167, 15187, 15192, 15213, 15226). No row shows a unit attacking with the bayonet for want of ammunition, or surrendering for want of it.
- **The Japanese on Guadalcanal attacked with what they could carry,** hand weapons and machine guns, their guns and shells not having come up (15126), and by December judged attack impossible for want of ammunition (14927, 15148).
- **No count of small-arms use exists for any fight on Guadalcanal,** and no rounds per man for any Japanese force as landed.

## The rules

### Where it runs

1. **The rule runs where a scenario's ground figures give its three figures,** `fight_supply_t_per_man`, `unsupplied_fire_share` and `fight_days_landed`. They are given all or none; the first and the third are 0 or more and the second is from 0 to 1. A file without them plays as today.
2. **The figures go into `data/reference/ground_hunger.toml`,** the file the table's copies name, and its frozen counterpart. The live `ground.toml` and its frozen copy are not changed.

### The charge

3. **A side's want for a day's fight is its firing men times `fight_supply_t_per_man`.** Its firing men are those of its units in the fight, each worked as today from its men ashore, its fighting share and its fatigue (decision 0208, rule 6), before this rule cuts them.
4. **The want is charged to the supply of the side's bases in the hex,** in the order the men's nightly charge takes from them, as far as the stock goes. Both sides are charged on the stocks as they stand at the start of the fight, before any base changes side.
5. **A side's supplied share is what was charged over what was wanted;** one where nothing was wanted, and nothing where the side has no base in the hex.
6. **Each of the side's units in the fight fires its firing men times its fire share, cut to a whole man:** the fire share is `unsupplied_fire_share` plus one less that figure times the supplied share. With the figure at a quarter, a side supplied in full fires every firing man, a side supplied by half fires five in eight, and a side with nothing fires a quarter.
7. **Nothing else of the fight changes:** the shots, the chance to hit, the limit on men engaged, the losses, fatigue, who is beaten and what a beaten side does. A day with no fight charges nothing.

### What a landing brings

8. **The first landing of men at a base puts days of fighting into it with their rations** (decision 0205, rules 4 and 5): the men aboard at that moment, times `fight_days_landed`, times `fight_supply_t_per_man`, as supply in the base's stock. It is given once, by the same mark of the base that the rations are.
9. **A march brings nothing more than it does.** The supply a marching unit puts down where it arrives (`march_supply_days`) is the one stock, and its fights are charged to it.

## Figures

10. All are placeholders, each with what it rests on.

| Figure | Value | Rests on |
| --- | --- | --- |
| `fight_supply_t_per_man` | 0.01 | a judgement inside a wide range: the manual's rates come to about 35 to 50 tons for a thousand men for a day of attack or defence (15083, 15084, 15086), and the nights of heavy firing on Guadalcanal to about 5 to 15 for the men ashore (15113, 15114, 15123). Ten tons for a thousand firing men is two days of their food |
| `unsupplied_fire_share` | 0.25 | a judgement: the weight of fire is in the guns and mortars (15083, 15084), which are what a side without supply loses, and men still have what they carry, a day or so of it (15105, 15186). The record gives no share |
| `fight_days_landed` | 4 | about four units of fire ashore on 9 August (rows on file, 15098), at the manual's unit of fire to a day of fighting after the first (15086) |

## What is recorded and who knows

11. **The charge is recorded** in an event of its own for each side in a fight that wanted supply: the hex, the side, the firing men, the tons wanted and charged, the supplied share and each base's stock after, citing the fight. A side sees its own and nothing of the enemy's.
12. **The breakdown of a unit's firing gains `supplied_share` and `fire_share`** where the figures are given, shown to its own side as its other factors are.
13. **The days of fighting a landing brings are recorded** beside the rations, in an event of its own, citing the landing: the men, the days and the tons.
14. **A side's own count of the men who would fire today** (`SideView::firing_men`) is not cut by this rule: it is what the unit would fire if supplied, since the charge depends on every unit of the side that fights that day.
15. **The table's diary** says plainly what a fight used and where a side fired short: "used 12 t of supply in the fight at Lunga" and "fired short at Lunga: 3 t of 12 t". A base's card is not changed; its stock is already shown.
16. **No new state, so no new field in the hash and no new layout.** The rules label does not rise, since no game on file plays differently.

## Rule card

Reads: each side's firing men in a fight, the supply of its bases in the hex. Writes: those bases' supply; the firing men of the day's fight. Draws no random number of its own.

## Games on file

None moves: the figures are in a file no proof names. The build runs the three 39-day copies the table plays before the change and after it and reports what moves: the men lost in fights and to hunger by side, the supply each side's fights used, the fights in which a side fired short, and who holds what at the end.

## What it does not do

- **It does not keep ammunition apart from food.** Supply is one stock: a force that fights eats its stock down sooner, and a force that does not fight can eat what it landed to fight with. A stock of ammunition of its own would be a new cargo, with its own orders, its own unloading and its own staff work.
- **It does not charge a quiet day,** which the manual puts at a fifth of a day of fighting and the campaign's averages at far less.
- **It does not charge attack and defence differently,** nor the first day of a fight more than the days after.
- **It does not stop a side that is short from attacking.** Its fire is cut, and an attack with a quarter of its fire fails by the rules there are.
- **It does not give the sides different figures,** nor let a side fire captured ammunition except as the stock of a base it has taken, which it already keeps a share of.
- **It does not teach the staff to foresee it.** A staff that counts a base's days of supply counts the nightly charge alone; a dry run plays the fights and so shows the cost.
- **It does not charge the guns of a base that fire on ships or aircraft,** nor bombardment.

## What it costs

- **Three figures, two events, a charge inside the fight and one more thing a landing puts down.**
- **Supply ashore now runs down in a fight.** With these figures a thousand men who fire for a day use what feeds them for two.
- **Rulebook:** chapter 7 (the day's fight, Rations landed, the list of figures, the list of what is left out); chapter 1's list of who sees what; the glossary gains "Supplied share" and "Fire share".

## Questions for the designer

None before the build. The three figures are placeholders to be tuned at the table.
