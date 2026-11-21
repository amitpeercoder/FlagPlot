# 0209: A fight costs supply

Status: a plan by the architect, written on Nov 21, 2026. Decision 0208, as its figure was moved on research, left a unit that is cut off from supply fighting at nearly full strength until its men begin to die: nothing models the ammunition of men ashore. The designer asked for it to be planned. It was read by a reviewer who had not seen it written, and this is the plan with that reading folded in. Its hand calculation is written next.

Summary: Today a fight ashore uses nothing. This makes a day's fight charge each side's bases in the hex, by the men who come to grips: first their ammunition, a stock a base already has, then their supply. A side whose stocks cannot cover the fight has its fire cut toward what its men carry. Every landing of men brings days of fighting as ammunition, which the men cannot eat. It runs where a scenario's ground figures give its three figures, so no proof and no recorded game moves; the games the table plays do.

Standing: Planned on Nov 21, 2026; its review folded in; its hand calculation written (`tests/scenario_123_a_fight_costs_supply.rs`); built on Nov 22, 2026 ("As built"), for the architect's reading.

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

## As built

Built on Nov 22, 2026, on the branch `fight-supply-build`, from the hand calculation of `tests/scenario_123_a_fight_costs_supply.rs`. Statements 1 to 7, 10 to 13 and 15 to 19 and the four properties hold as written. Statements 8, 9 and 14 do not hold as written; each is written as it stands, ignored with its reason, and has an `_as_built` test beside it (What did not hold, below). No figure was tuned.

### What was built, and where

- **The three figures (rules 1 and 2, statement 15).** `sim_core::refdata::FightSupplyFigures` (`fight_supply_t_per_man`, `unsupplied_fire_share`, `fight_days_landed`) is `HungerFigures::fight_supply`, an `Option`, so the three can only be given with the three of 0205, and need not have the two of 0208. The loader (`sim-data/src/reference.rs`) refuses one or two without the third ("together, or none of them"), the three without the three of 0205 ("only with"), a per-man figure or days below 0, and a share below 0 or above 1 (`GroundFigures::problem`). `data/reference/ground_hunger.toml` and its frozen counterpart gain the three (0.01, 0.25, 4), each with a PLACEHOLDER comment giving what the table above says it rests on; `data/reference/step1/README.md` says so. The live `ground.toml`, its frozen copy and the files of scenarios 119 and 122 are not changed.
- **The charge (rules 3 to 9, 14 and 15).** In `ground_combat/fight.rs`: the men at grips are worked in the pass that works the limit (the side's firing men, or the limit where that is less, not cut); then `World::charge_fight` charges each side, Allied first, before any die; then each unit's shots after the limit are multiplied by its side's fire share, and only then are the weights of fire worked. `supplied_share` and `fire_share` are pure functions there. The charge records `FightSupplyCharged` (citing the fight), then for each base charged `BaseAmmunitionSpent` and `BaseSupplySpent` with the new `SupplyUse::Fight`, each citing the charge; each unit's `GroundFire` cites its side's charge. The breakdown of `GroundFire` gains `supplied_share` and `fire_share` just before `shots`, where the figures are given.
- **The landing (rules 10 and 16).** `cargo.rs`: `prepare_landing` reads, before any man lands, whether the force begins a landing at the hex (its `landing_at` is not that hex), and `land_ammunition`, called after `land_rations` in both of its branches, puts the men aboard times the days times the per-man figure into the base's `ammunition_t` and records `AmmunitionLanded`, citing what the rations' record cites.
- **The dry run (rule 12, statement 13).** `World::set_supplied_in_full(side)` and `World::supplied_in_full()`, a field of the world named in `hash::name_every_field` as not hashed, like the game's days. `sim_trial::build_world` sets it for the side the picture makes up. `sim_trial::Assumption::EnemySupplied` (rule 12 of decision 0209) is put in the list of what was assumed where the figures are given, and carried to the estimate (`AssumedOf::EnemySupplied`), the seat's trial and the agent's answer (`AssumedThing::Supplied`, kind `enemy_supplied`), whose words are "the enemy's men ashore taken to have all they need to fight, at no cost to his stocks"; the table's paper shows it under what we took the enemy to have, as it shows every assumed line.
- **Who sees what (rule 17).** `side_events.rs`: the charge and the ammunition landed are their side's own, whole, and a base's ammunition spent its base's side's; the enemy is told nothing. The two factors are named among those a side may see of its own unit's fire, with a test (`a_ground_fire_cut_keeps_the_sides_supplied_share_and_fire_share`), and the three events are in the table of who sees what.
- **The agent and the export.** A base's line (`list_bases`) and the briefing's bases gain `ammunition_t`, left out when 0; a base's stocks yesterday gain `used_in_fights_t`; the text adds ", ammunition N t" and "used in fights". The three events are in the export (`fight_supply_charged`, `base_ammunition_spent`, `ammunition_landed`) and the briefing's fold leaves them to the events and explain. The schemas (`defs`, `briefing`, `trial`) and their README gain the fields; the version is not raised.
- **The table (rule 19).** The diary says what a day's fighting used ("The fighting at Lunga used about 7 t of ammunition") and where our troops fired short ("Our troops at ... fired short: the fighting called for 10 t of ammunition and the stocks there gave 5 t of supply"), in whole tons; a base's ammunition spent is folded with the routine. A base's card shows "Ammunition" beside "Supply" where the base has any.
- **No new state (rule 20).** The hash, `STATE_VERSION` and the rules label are not changed; scenario 123's test pins them.
- **Scenario 123.** `data/scenarios/a-fight-costs-supply/123_a_fight_costs_supply.toml` (scenario 122's setting with scenario 119's mark for forces and a line for Q's ammunition, 16 days), its ground figures `data/reference/ground_placeholder_scenario_123.toml` and the frozen copy (the frozen `ground.toml` with the eight figures, `unfed_fighting_share` 0.9), and the test, registered in `tests/Cargo.toml`: 27 tests, three of them ignored.
- **Tests on file.** Scenario 122's statement 18 gains the three keys in its list of what `ground_hunger.toml` may add, by rule 2; scenario 119's property that no game on file moves gains scenario 123's folder and file among those that may give the figures, as it gained 122's; scenario 7's list of kinds not in the full slice gains the three events; scenarios 6 and 6d2 name the new use.
- **The rulebook.** Chapter 7: where it runs, Ammunition landed, What a fight costs, the fire's breakdown and what a side sees, the list of events, the data keys and what is left out (the line that `unfed_fighting_share` stood for ammunition is gone); chapter 1's table of who sees what and its list of factors a side keeps; chapter 9's enemy fighting supplied. The glossary gains "Men at grips", "Supplied share", "Fire share" and "Ammunition landed". `docs/commands.md` says what output changes.

### The readings

1. **Ammunition first means all the side's bases' ammunition, then their supply,** each pass in base order, not each base's ammunition and then its own supply.
2. **A base with nothing of a stock is asked nothing of it and records nothing,** as the night's charge does; a side that wanted something records its charge even with no base in the hex or nothing given (statements 3 and 4).
3. **The supply a fight spends is `BaseSupplySpent` with the reason `fight` and `men` 0;** the men at grips are in the charge. The base events have a breakdown (`wanted_t`, `charged_before_t`, the result the tons asked).
4. **A side taken as supplied in full still has the two factors in its fire's breakdown,** each 1, so the breakdown says the rule ran; it records no charge.
5. **A force begins a landing when it lands men at a hex where its `landing_at` is not that hex.** A landing of several pulses brings ammunition once; the same force landing again after its unloading there is complete brings it again.
6. **The dry run's setting is set by `build_world` for every picture,** and acts only where the figures are given; the assumed line is made only where they are given.
7. **The base line gives ammunition only where above 0,** so every answer of a game whose bases hold none is as it was.

### What did not hold

- **Statements 8 and 9:** a ground file with `engagement_ratio` 0.5 does not load: the loader and `World::new` refuse a ratio below 1 (decision 0105; chapter 7, "so the smaller side is never cut"). Both are written as written and ignored. Beside them, `statement_8_as_built_...` and `statement_9_as_built_...` reach the statements' numbers at the least ratio the loader takes, 1, with V of 500 men and Q at 17.5 t (so the night leaves 15 t, and the limit for B, 500, cuts E's 891): A's men at grips 500, 5 t wanted and charged, Q 10 t after, V's shots 5; B's men at grips 500, 5 t wanted, none charged, E's shots 1.25. A question for the architect: is the setting to be moved so, or the statements to stand ignored.
- **Statement 14:** scenario 122's ground file gives `unfed_fighting_share` 0.5, so E's firing men there are 495 and its shots 4.95, not 891 and 8.91. Run with `--ignored`, V's shots are 10 and E's 4.95. Written as written and ignored; `statement_14_as_built_without_the_three_figures` plays scenario 123's own ground file with the three figures taken out, and every number of the statement holds there: V 10, E 8.91, Q 15 t, nothing recorded of a charge, no factor, and the landing bringing 45 t of supply and no ammunition. The dice of that game fall as they fell before the change (V 5 hits and E 2 on day 1, V 4 and E 4 on day 2, taken from the build before the change).

### Games on file

Every scenario file under `data/scenarios` (`tmp-inside` aside), 200 of them, was run with `run --events` before the change and after it. All but four give the same output to the byte, events and daily hashes. The four are table copies: `table-hunger/02b_eastern_solomons_oilers`, `03b_coral_sea_both_oilers`, `07b_slice` and `07c_slice`. In the first three, once the new events are taken out and the events renumbered, nothing differs: the ammunition landed is in the base's state, so the daily hashes move, and nothing else does. `07_full_slice` does not move at all: no landing there puts men ashore and no fight is fought.

| Copy | Men lost in fights (Allied, Japanese) | Men lost to hunger (Allied, Japanese) | What the fights used | Fights fired short | Bases at the end |
| --- | --- | --- | --- | --- | --- |
| `07_full_slice` | 0, 0 before and after | 0, 231 before and after | nothing: no fight | none | the same: Espiritu Santo Allied, every other base Japanese |
| `07b_slice` | 0, 0 before and after | 0, 0 before and after | nothing: no fight | none | the same: Espiritu Santo Allied, every other base Japanese |
| `07c_slice` | 80, 80 before and after | 0, 993 before and after | Allied 14.22 t of ammunition and no supply; Japanese 4.74 t of supply and no ammunition (two fights on day 1, at Lunga and Tulagi) | none: every side was supplied in full | the same: Lunga, Tulagi, Espiritu Santo and the two landings Allied; Rabaul, Shortland and the two stocks Japanese |

In `07c_slice` the Allied landings of day 1 bring 243 t and 436 t of ammunition, and two later landings 4.92 t and 15.68 t. The Japanese garrisons fight on the supply of the bases they lose that day, so Lunga and Tulagi change side with 1.17 t and 1.2 t less supply than before; that ripples as tons through the Allied stocks and the staff's reasons for the rest of the game, and changes no order, no sortie (550 aircraft sent on sorties before and after), no loss and no holder. No base ran out of supply that had not before (two short charges for men, the same, before and after); no event records a sortie not flown for want of supply, so none is counted, and none differs.

### Not verified

- The diary's two sentences and the card's "Ammunition" row were not seen in a browser; the patterns were checked against the events' text by script, and `python3 table/check_screen.py --port 8796` passes (174 checks), which does not open the diary of a fight or a base card.
- A seat's dry run on a game with the figures was not asked, so the assumed line was not seen on the paper; the chain to it compiles and is exhaustive.

## The architect's rulings

Read on Nov 22, 2026: the charge in full (`sim-core`, `ground_combat/fight.rs`, `charge_fight` and where the fire share meets the shots), the landing's ammunition, the dry run's setting, and the build's account of what moved.

1. **Three statements of the hand calculation were wrong, and the fault was the architect's.** Statements 8 and 9 asked for a limit on engaged men of a half, which no ground file may give (decision 0105): they are restated at a ratio of 1 with V of 500 men, where the limit bites on B and not on A, and the numbers they assert are the ones first written. Statement 14 named scenario 122's ground file, whose `unfed_fighting_share` is another: it is restated on scenario 123's own file with the three figures taken out. Each is one test, not set aside; the build's tests of what the code does became the tests.
2. **The build's seven readings are accepted.** A landing over several pulses brings its ammunition once, and a force that lands again after its unloading is complete brings it again; a side taken as supplied shows both shares as one and records no charge; a base's line shows ammunition only where it has some, so no answer on file changes.
3. **What the rule does to the 39 days, in the table's copies:** no loss, no sortie and no base's holder changes. In the one copy with fights, both on the first day, the Allied side used 14.22 t of the 679 t of ammunition its landings brought, and the Japanese 4.74 t of supply; no side fired short. So the rule is in the game and these games do not yet test it: the landings bring far more than two fights use, and the Japanese counter-attacks that would run a side short do not happen in these files.
4. **A sortie not flown for want of supply is not recorded,** so the plan's ask to count them could not be met; bases that ran out of supply are the same before and after.
5. **Not seen:** the diary's two new sentences and the card's ammunition row in a browser, and the dry run paper's line that the enemy is taken as supplied.
