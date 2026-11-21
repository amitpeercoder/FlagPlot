# 0209: A fight costs supply

Status: a plan by the architect, written on Nov 21, 2026. Decision 0208, as its figure was moved on research, left a unit that is cut off from supply fighting at nearly full strength until its men begin to die: nothing models the ammunition of men ashore. The designer asked for it to be planned. It was read by a reviewer who had not seen it written, and this is the plan with that reading folded in. Its hand calculation is written next.

Summary: Today a fight ashore uses nothing. This makes a day's fight charge each side's bases in the hex, by the men who come to grips: first their ammunition, a stock a base already has, then their supply. A side whose stocks cannot cover the fight has its fire cut toward what its men carry. Every landing of men brings days of fighting as ammunition, which the men cannot eat. It runs where a scenario's ground figures give its three figures, so no proof and no recorded game moves; the games the table plays do.

Standing: Planned on Nov 21, 2026; its review folded in; its hand calculation written (`tests/scenario_123_a_fight_costs_supply.rs`).

## Why

What is so today:

- **A fight ashore charges nothing.** A side's shots are its firing men over `men_per_shot` (rules chapter 7); no stock is read or written.
- **Supply at a base is one stock,** charged each night for the men ashore (`daily_supply_t_per_man`). A base's `ammunition_t` is for ships that rearm in port (decision 0122), is set by the scenario and is no cargo.
- **Since decision 0208's figure moved, a night on nothing costs a unit a tenth of its fire.** So a unit with no supply at all fires 89 men in 100 after one night and still 77 after fourteen.

What the record holds (`docs/research/ammunition-ashore.md`; rows 15082 to 15240 of `land_unit_facts.csv`, all unconfirmed):

- **A force landed with days of fighting, not weeks.** The 1st Marine Division had about four units of fire ashore on 9 August (rows 464 and 777, on file), of ten embarked (15098). The Army's manual of 1941 puts a day of attack or defence of a position at 1.5 units of fire on the first day and 1.0 after, and an inactive day at 0.2 (15086; the architect read the table on the page image), figures it says rest on the First World War.
- **Men who carried their own had a day or two.** A Japanese rifleman on Tulagi had 120 rounds (15105); one Australian company used 100 rounds a man and 1,200 grenades in a day (15186).
- **The weight is in the guns.** A regiment's small arms and mortars are 72.4 tons to a unit of fire (15083); twelve 105-mm howitzers are 68 tons (15084). On the night of 23 October the 11th Marines fired 6,164 rounds of 75-mm and 2,719 of 105-mm (15123).
- **Reinforcements brought their own:** the 7th Marines about ten units of fire on 18 September (15122), the 164th Infantry five (357, 712).
- **What was used was far below the manual on most days.** The Marines went 42 days on their four units of fire, through the Tenaru and the ridge, short of particular kinds and not exhausted. (The tons for a thousand men used below are the research's reader's arithmetic on these rows, with strengths that are not in the rows.)
- **A unit short of ammunition rationed its fire and held; a unit out of it withdrew where it could** (15119, 15167, 15187, 15192, 15213, 15226). No row shows a unit attacking with the bayonet for want of ammunition, or surrendering for want of it.
- **The Japanese on Guadalcanal attacked with what they could carry,** hand weapons and machine guns, their guns and shells not having come up (15126), and by December judged attack impossible for want of ammunition (14927, 15148).
- **No count of small-arms use exists for any fight on Guadalcanal,** and no rounds per man for any Japanese force as landed.

## The rules

### Where it runs

1. **The rule runs where a scenario's ground figures give its three figures,** `fight_supply_t_per_man`, `unsupplied_fire_share` and `fight_days_landed`. They are given all or none, and only with the three figures of decision 0205; they do not need the two of decision 0208. The first and the third are 0 or more and the second is from 0 to 1. A file without them plays as today, in every rule below.
2. **The figures go into `data/reference/ground_hunger.toml`,** the file the table's copies name, and its frozen counterpart. The live `ground.toml` and its frozen copy are not changed. Scenario 122's test lists the keys that file may add to the live one; the list gains these three, by the architect's leave.

### The charge

3. **A side's men at grips in a day's fight are its firing men, no more than the limit on engaged men** (decision 0105): the sum of the firing men of its units that fire, as the fight works them today, or `engagement_ratio` times the other side's fighting men where that is less. It is a quantity, not cut to a whole man.
4. **A side's want is its men at grips times `fight_supply_t_per_man`.** A unit's weight does not change it.
5. **The want is charged to the side's bases in the hex,** in the order the men's nightly charge takes them: first from the ammunition of each (`ammunition_t`, the stock a base has for ships that rearm), then from the supply of each, as far as they go. Both sides are charged on the stocks as they stand when the fight begins, before any die is rolled and before any base changes side. A side with no base in the hex is charged nothing.
6. **A side's supplied share is what was charged over what was wanted,** a division cut at the sixth place; one where nothing was wanted.
7. **A side's fire share is `unsupplied_fire_share` plus one less that figure times its supplied share.** With the figure at a quarter, a side supplied in full has a fire share of one, a side supplied by half five eighths, and a side with nothing a quarter.
8. **Each unit's shots, after the limit on engaged men, are multiplied by its side's fire share.** Its firing men are not changed and nothing is cut to a whole man again. The rules for everything that follows from the shots are as they are: the weight of fire that decides a failed attack, the dice, the losses, who fired, fatigue, who is beaten.
9. **A day with no fight in a hex charges nothing there.**

### What a landing brings

10. **Each time a force begins to land men at a base of its side, it puts days of fighting into the base's ammunition:** the men aboard its ships at that moment, times `fight_days_landed`, times `fight_supply_t_per_man`. A unit's weight does not change it. It is given at every such landing, not once for the base as rations are: men cannot eat it, and the record has reinforcements bringing their own.
11. **A march brings nothing more than it does.** A unit that marches by order puts down supply where it arrives (`march_supply_days`), and its fights are charged to that. A unit that falls back carries nothing and fires at the floor.

### In a dry run

12. **A side that a dry run makes up is taken as supplied in full, and charged nothing.** A side does not know the enemy's stocks, and a dry run pictures none for the bases his landings and marches formed, so without this every pictured enemy would fire at the floor. It is a setting of the world a dry run builds, not state, and the dry run's paper says it among what was assumed of the enemy.

## Figures

13. All are placeholders, each with what it rests on.

| Figure | Value | Rests on |
| --- | --- | --- |
| `fight_supply_t_per_man` | 0.01 | a judgement inside a wide range. By the research's reader's arithmetic the manual's rates come to about 35 to 50 tons for a thousand men for a day of attack or defence (15083, 15084, 15086) and the nights of heavy firing on Guadalcanal to about 5 to 15 for the men ashore (15113, 15114, 15123); neither is a row. Ten tons for a thousand men at grips is two days of their food |
| `unsupplied_fire_share` | 0.25 | a judgement, and the weakest of the three: the weight of fire is in the guns and mortars (15083, 15084), which is a share of weight and not of effect, and men still have what they carry, a day or so of it (15105, 15186). The record gives no share |
| `fight_days_landed` | 4 | about four units of fire ashore on 9 August (464, 777), read as four days of the landed men all at grips. By the manual a unit of fire is far more than a game's day of fighting costs here, so this is a floor on what was landed, not a measure of it |

## What is recorded and who knows

14. **The charge is recorded** in an event for each side in a fight that wanted anything: the hex, the side, the men at grips, the tons wanted, the tons charged from ammunition and from supply, and the supplied share, citing the fight. Each base charged records its stock spent: its supply by the event there is for supply spent, with a new use, the fight, added at the end; its ammunition by an event of its own. They come after the fight's event and before any unit's fire, the Allied side first, and each unit's fire cites its side's charge.
15. **The breakdown of a unit's fire gains `supplied_share` and `fire_share`** where the figures are given, named among the factors a side may see of its own unit, with a test.
16. **The ammunition a landing brings is recorded** in an event of its own, citing what the rations' record cites: the base, the force, the men, the days and the tons.
17. **A side sees its own of all of these and nothing of the enemy's.**
18. **A side's own count of the men who would fire today** (`SideView::firing_men`) is not changed: this rule cuts shots, not firing men.
19. **The table's diary** says what a fight used and where a side fired short, in plain words and whole tons, and a base's card shows its ammunition beside its supply where it has any.
20. **No new state, so no new field in the hash and no new layout.** The rules label does not rise.

## Rule card

Reads: each side's firing men and the limit on engaged men in a fight, the ammunition and supply of its bases in the hex, the men aboard a force that lands. Writes: those bases' ammunition and supply; the shots of the day's fight. Draws no random number of its own.

## Games on file

No proof and no recorded game moves: the figures are in a file none of them names. The nine copies the table plays name it, and they change. The build runs the three 39-day copies before the change and after it and reports what moves: the men lost in fights and to hunger by side, what each side's fights used of ammunition and of supply, the fights in which a side fired short, the air sorties not flown for want of supply, and who holds what at the end.

## What was weighed and not taken

- **Ammunition wholly in the one stock of supply.** The first plan had it so, with the landing's days of fighting put into the stock the men eat. Four days of fighting weigh as much as eight nights of food, so every force that landed and did not fight at once would have gone hungry eight nights later than decision 0205 has it. The base's ammunition is a stock that exists, is hashed, and that no man eats.
- **Ammunition as a cargo of its own.** It would need its own orders, unloading and staff work. Resupply after the landing comes as supply, which the fight falls back on.
- **A side out of ammunition made to fall back,** as the record has units that ran out withdraw. A cut of fire is one rule; a withdrawal for want of ammunition is a second, with its own questions of where to and who decides. A side at the floor loses its fights by the rules there are. It is the first thing to add if play shows cut-off units holding too long.
- **Fatigue-like decay of the floor,** so that what men carry runs out. It needs a count for each unit.

## What it does not do

- **It does not charge a quiet day,** which the manual puts at a fifth of a day of fighting and the campaign's averages at far less.
- **It does not charge attack and defence differently,** nor the first day of a fight more than the days after.
- **It does not stop a side that is short from attacking.**
- **It does not give the sides different figures.**
- **It does not let ammunition be carried by sea as cargo or by a march.** What a march puts down is supply.
- **It does not keep the fight's supply apart from the aircraft's:** a base's supply also pays for the upkeep and sorties of its aircraft, and a fight that falls back on supply competes with them.
- **It does not keep men's ammunition apart from ships':** a ship that rearms at a base draws on the same stock, where the base's port lets her.
- **It does not teach the staff to foresee it.** A staff that counts a base's days of supply counts the nightly charge alone; a dry run plays the fights and so shows the cost to the side that asks.
- **It does not charge the guns of a base that fire on ships or aircraft,** nor bombardment.

## What it costs

- **Three figures, three events and a new use of a fourth, a charge inside the fight, one more thing a landing puts down, and a setting of a dry run's world.**
- **Supply ashore can now run down in a fight,** once the ammunition is gone: a thousand men at grips for a day use what feeds them for two.
- **An attacker who marches fights on what he carried,** about two and a half days of fighting less what he eats, and a unit that has fallen back fires a quarter.
- **Rulebook:** chapter 7 (the day's fight, Rations landed, a base's stocks, the list of figures, the list of what is left out); chapter 1's list of who sees what; the chapter on dry runs for rule 12; the glossary gains "Men at grips", "Supplied share" and "Fire share".

## Questions for the designer

None before the build. The three figures are placeholders to be tuned at the table, and the floor of a quarter is the one to watch.

## How the plan changed on review

A reviewer who had not seen the plan read it against the fight's code, the feeding, the landing, the march, the dry run and the rows on Nov 21, 2026.

- **The want was every firing man of the side.** Ten thousand defenders attacked by nine hundred would have paid for ten thousand, far above anything on record, and the cut could have vanished inside the limit on engaged men. The want is now the men at grips, and the cut is on the shots after the limit (rules 3 and 8).
- **The landing's days of fighting went into the stock the men eat,** which undid decision 0205's three nights. They now go into the base's ammunition (rules 5 and 10).
- **They were given once for a base,** by the rations' mark, so reinforcements brought none. Every landing now brings them (rule 10).
- **A dry run would have been optimistic:** the enemy it pictures has no stock at the bases his landings formed, and would have fired a quarter. Rule 12.
- **A test on file would have failed,** and the plan said no game on file moves where the table's own do. Rule 2 and "Games on file".
- **Left to guess:** a unit's weight, the order of events and their causes, the rounding, what the three figures are given with. Rules 1, 4, 6, 8, 10 and 14.
- **Row 15098 was cited for four units of fire ashore;** it is the ten embarked. The tons for a thousand men were cited as rows and are a reader's arithmetic. A fortnight's fire of a unit on nothing was overstated.
- **Not taken, with the reason given above:** a withdrawal for want of ammunition, and a floor that decays.
