# Land units

This chapter covers men: the land unit, the troop capacity of a ship, the load order that takes a unit aboard, the landing that puts it ashore and forms a base for it, a unit divided, men lost with a ship, the men's daily supply and the loss of the unfed, the posture of a unit and its order, marching overland, the day's fight ashore and the limit on the men engaged in it, fatigue, an attack that fails, what a spent attacker does, a side beaten in a hex, a base that changes side, a beaten side's men falling back or surrendering, the day's activity of a unit, units that work and what they build, preparing a landing, the experience of a unit and training, what a land unit is good at (its skills), men lost to the fire of bombs and shells, and what a side knows of the enemy's men ashore.

Men are worked through the cargo rule of chapter 2 and fed through the base supply rule of chapter 5. This chapter gives only what is new in them. Every quantity is a fixed-point number except men, which are whole numbers. "Cut" means cut toward zero (chapter 1, Numbers and chance).

Where these rules run in a day (chapter 1, The order in which a day resolves, gives the whole of it):

| When | Rule |
| --- | --- |
| Start of the day, with the other orders | A load order with a unit, a posture order, a march order, a prepare-landing order and an activity order are checked and applied (chapter 1, Orders) |
| Night pulse, after the daily supply for aircraft | The men's supply: feeding, the unfed and the loss |
| Loading phase, last in each pulse | Men are taken aboard and landed with the cargo; units are divided and bases formed; a force that lands men works at the rate its loading gives, and a unit's first men landing spend its rehearsal (Preparing a landing, below) |
| Whenever a ship sinks | The men aboard are lost with her |
| Night pulse, in the shore bombardment phase, and in the air combat phase of a daylight pulse | Bombs and shells that hit men ashore: men lost to fire (below) |
| Wherever a rule below reads one | A land unit's skills (What a land unit is good at, below): the assault skill in the day's rehearsal, the movement skills in a day's march, the holding skill and weight in the fight, weight in the men's supply and in the supply a march carries |
| After the third pulse, before the day's base count | The ground combat phase: first the marches, then for each hex with men ashore the fight, fatigue, a failed attack, a spent attacker, a side beaten, a base that changes side, the beaten side's men falling back or surrendering and the ground reports; last, once every hex is done, the day's activity: each force's loading, each unit's rehearsal, training or work, and the days of fighting counted (The day's activity, below) |
| The day after a day's work | Work on a port speeds the next day's cargo at the base, and work on aircraft raises the next night's repairs (Units that work, below) |

A scenario that names no ground figures has no land units, and every rule here does nothing in it (chapter 1, Where figures come from and when a rule is loaded). A map with no land links allows no march (chapter 2, Land links). A scenario whose ground file gives the frozen step 1 figures has the rules of preparation and experience switched off by their first values: `unprepared_landing_share` is 1, `unprepared_landing_fatigue` is 0 and `experience_fire_strength` is 0, though loading, rehearsal and the days counted are still kept. A scenario whose ground file gives the frozen figures has every skill switched off too: the four strengths are 0 (What a land unit is good at, below). The seven figures of work and the three of an airfield's damage (The airfield, below) are the same in the frozen file, and no frozen scenario has a working unit. The frozen air combat file gives the share of a hit on a base that lands on its airfield as 0, so no frozen scenario has an airfield damaged. [0066, 0090, 0094, 0095, 0097, 0098, 0099, 0103]

## The land unit

### What a land unit is

A land unit is a count of men with a side and a name. It has a number, which is stable and never reused. Its men are ashore in one hex, aboard ships, or part each. The men ashore of one unit are all in one hex. The men aboard a ship are of one unit only, and a unit may have men aboard several ships. A unit is destroyed when it has no men ashore and none aboard any ship (below, Destroyed). It stays in the state with no men and keeps its number. [0066 items 1 and 2]

A land unit's state is: its number, its name, its side, the men ashore and the hex they are in, its posture, the order that set its posture, its fatigue, its fed share, its march (the hex it marches to, the days marched and the march order, or none), its rehearsal and the hex the rehearsal is for, its activity (ready, train or work, with the job of a work activity, or none) and the order that set it, whether it is a working unit, its experience, its days of training and its days of fighting. The hex is set exactly when the men ashore are above 0. The posture is defend or attack, defend at the start unless the scenario says otherwise. The fatigue is from 0 to 100 and starts at 0 unless the scenario says otherwise. The fed share is from 0 to 1 and starts at 1. The rehearsal is from 0 to 100, starts at 0, and its hex is set exactly when it is above 0. The activity is ready at the start, or work for a working unit. The experience is a whole number from 0 to 4 and starts at 2 unless the scenario says otherwise. The days of training are below `training_days_per_level` and the days of fighting below `fight_days_per_level`, both 0 at the start unless the scenario says otherwise. A ship's state gains the men aboard and the unit they belong to (`troops`), at least one man when set. A task force's state gains its preparation and `landing_at` (Preparing a landing, below). The state hash holds the men, the hexes, the posture, its order, the fatigue, the fed share, the march, the rehearsal and its hex, the activity with its job, its order, the experience, the two counts of days and the working mark, and not the name (chapter 1, The state hash). [0066, 0090 items 1, 8 and 11, 0094, 0095 items 2 and 3, 0097 items 1 and 3]

A land unit's state also holds its skills (What a land unit is good at, below), and its march holds the days counted beside the days marched. The state hash holds both (chapter 1, The state hash). A base's state holds whether it has an airfield, the work done on it, the damage to it, its defences and the day's work (Units that work, below), and the hash holds them. [0098, 0099, 0103]

A side sees: its own units exactly (below, What a side knows and sees).

### The scenario gives the units

A scenario's `[[land_unit]]` entry gives `id`, `name`, `side` and `men`, and either a hex (`q` and `r`) where the men are ashore, or a `task_force` where they are aboard. It may give a `posture`, `"defend"` (the default) or `"attack"`, a `fatigue`, a whole number or a decimal string from 0 to 100 (0 if not given), an `experience`, a whole number from 0 to 4 (2 if not given), `training_days` and `fight_days`, whole numbers of days already counted (0 if not given), and `working`, `true` or `false` (false if not given). No key gives a unit a rehearsal or an activity: a unit starts with 0 rehearsal and ready, or at work if it is a working unit, and a force starts with no preparation. The scenario's `ground` key names the ground figures file. The loader refuses a scenario that:

- has a land unit and names no `ground` file;
- gives a unit no men, a number of 0 or of 1,000,000 and above (the loader keeps that range clear, as it does for the task forces and air groups made in play; a land unit made in play is not numbered in it, see A unit divided), or a number twice;
- gives a unit both a hex and a task force, or neither, or only one of `q` and `r`;
- puts a unit ashore off the map or in a hex that is neither coast nor land;
- puts a unit aboard a task force that does not exist or is of the other side;
- puts a unit aboard a force whose ships do not have berths for all its men;
- gives a unit a fatigue below 0 or above 100, or a posture that is neither `defend` nor `attack`;
- gives a unit an experience above 4, or days of training or of fighting that are not whole numbers or are not below the days a step takes (`training_days_per_level`, `fight_days_per_level`);
- gives a unit a skill that is not a whole number from 0 to 4, or a name that is not one of the eight (below, What a land unit is good at). [0098]
- marks a unit `working` and gives it the posture attack. [0099]

Units are put aboard in unit order, each over the force's ships in ship order, into a ship that has no men aboard or holds men of this unit, each up to its troop capacity. A unit that is aboard has no men ashore. The game is also refused when it is built (`World::new`) if there are land units and no ground figures, or if the ground figures are wrong (below, Data keys). [0066 items 3 and 4, Settled before the build, 0090 items 1 and 11, 0097 items 1 and 3]

### Troop capacity

A ship class has a troop capacity in men, the column `troop_capacity_men` of the ship classes file. Men take no cargo room and cargo takes no berths: the two limits are apart. The column is optional. A file without it gives every class 0, and a file with it needs a whole number in every row. A ship carries men up to the capacity of its class and no more. A ship that carries men of one unit takes none of another. [0066 items 4 and 8]

A side sees: not an event.

## The load order with a unit

The task force order Load takes a unit beside its cargo. The cargo is supply, fuel or none. The order is a standing cargo order like Load, Load fuel and Unload, and replaces them (chapter 2, Cargo orders). It takes the unit's men ashore in the force's hex aboard first, and the named cargo after. With no cargo it takes the men only. A load order with no unit must name supply or fuel, as before. [0066 items 5 and 6, Settled before the build]

It is checked at intake in this order, and the first reason that applies is recorded (chapter 1, Every reason an order is refused):

1. **Under way.** The force's status is under way.
2. **Out of fuel.** The force's status is out of fuel.
3. **Not your land unit.** The unit does not exist or is the enemy's. One reason serves both, so a refusal never shows that an enemy unit exists.
4. **No men ashore here.** The unit has no men ashore, or they are not in the force's position.
5. **No troop capacity.** No ship afloat in the force is of a class with a troop capacity above 0.

No base is needed. A load order with a unit is accepted where the side has no base, which a plain load is not, so men can be taken off a shore that has nothing else. Where the side has no base the order loads men only, whatever cargo it names. Where it has one, the lowest-numbered base of the side in the hex gives the cargo. [0066 items 5 and 6]

A side sees: its own `OrderAccepted` or `OrderRejected`, with the reason, whole. The enemy sees nothing.

## Men worked as cargo

### Who works

The loading phase, last in every pulse, works the cargo of a force that holds a cargo order, is idle, arrived or halted, and is exactly at the centre of a hex (chapter 2, Cargo orders). Besides a force at a base of its own side, two kinds of force work with no base there, if the ground figures are loaded:

- a force with a load order with a unit, which takes men off the shore;
- a force with an unload order, in a coast hex, with men aboard any ship, which lands them.

The hours worked, the rates of the port and the order of forces and ships are those of chapter 2, Working cargo. The port is the port of the hex. [0066 items 6, 7 and 11]

### Berths

Berths are given out before any cargo moves, for each side's landing place in a hex: the lowest-numbered base of the side there, or the one a landing will form. A ship has cargo to move, for the giving of a berth, if it has men to move or tons to move. A load has men to move when the unit has men ashore in the hex and the ship has room for them; an unload has men to move when the hex is a coast hex and the ship carries men. Chapter 2, Berths and lighters, holds the rest. [0066 item 7]

### One weight for a man

A ship's work in a pulse is its rate (berth or lighter) times the hours worked, in tons, as in chapter 2. Each man moved takes `ground.toml: handling_t_per_man` of it. Men move first and in whole men: the most a ship can move is its tons of work over the weight of a man, cut. What is left of the ship's tons moves supply and then fuel, by the rule of chapter 2. One figure serves loading and landing. The breakdown of a `CargoLoaded` or `CargoUnloaded` of a ship that moved men in the pulse has the factor `men_t`, the tons its men took first; it has none where no man moved. [0066 item 7]

### Loading men

Ships are taken in ship order. A ship takes the least of the men its tons of work can move, the unit's men still ashore in the hex, and its berths for the unit (its troop capacity less the men aboard, or none if men of another unit are aboard). The unit's men ashore fall by the men taken. A unit with no men left ashore has no hex. Each ship that takes men records `MenEmbarked`: the unit, ship, hex, men, men now aboard the ship and men left ashore. It cites the load order and the force's arrival event. Its breakdown has the factors `hours_worked`, `on_berth`, `rate_t_per_h`, `capacity_by_time_t`, `handling_t_per_man`, `men_by_time` and `berths` (the least of the berths and the unit's men ashore), and its result is the men taken. [0066 item 8]

Then the cargo, with the tons left. A load moves the least of the tons left, the ship's room and the stock, by chapter 2. With no base or no cargo named it moves none.

A load order with a unit is complete when no man of the unit is ashore in the hex or no ship has a berth for one, and nothing remains of the named cargo by the rule of chapter 2 (the stock is 0 or every ship is full; with no cargo named, or no base, nothing remains). When a force has moved something this pulse and the order is complete, `UnitLoadingComplete` is recorded in place of `LoadingComplete`: the force, the unit, the base loaded from (none off a bare shore), the cargo (none if none was named) and the hour the slowest ship finished. The hour is the start of work plus the longest of each ship's tons over its rate, counting the tons its men took. It cites the events of the men and the cargo. [0066 Settled before the build]

A side sees: `MenEmbarked` and `UnitLoadingComplete`, whole, for its own unit and force.

### Landing men

An unload order lands men first, then supply, then fuel. Men are landed only on a coast hex. In any other hex they stay aboard, nothing is recorded for them, and the unload counts as complete without them, as cargo does today where there is no base. [0066 items 9 and 10]

A landing happens before any ship of the force moves cargo in the pulse. If any ship of the force can land at least one man in the pulse (its tons of work over the weight of a man, cut, and the men it carries, give at least 1), then, in this order:

1. **Units are divided** (below), in unit order, for each unit that has men to land.
2. **A base is formed** (below) if the side has none in the hex.

Then each ship, in ship order, lands the least of the men its tons of work can move and the men it carries. The men join their unit's men ashore, and the unit's hex is set to the hex. The ship's men aboard fall by the men landed, and a ship with none left carries none. Each ship that lands men records `MenLanded`: the unit, ship, hex, men, men left aboard the ship and men of the unit now ashore. Its breakdown has the same factors as `MenEmbarked`, with `aboard` (the men the ship carried) in place of `berths`, and the factors of the loading rate below where they apply. It cites the unload order, the force's arrival event and, in the pulse a base was formed, the `BaseFormed` event, and, for a unit made by division, its `UnitDivided` event. The supply and fuel unloaded in the same phase go into the base, formed or not, with the tons the men left. [0066 items 9 to 11]

An unload order is complete when no ship has supply or fuel aboard and, in a coast hex, no ship has men aboard. Then `UnloadingComplete` is recorded as before, citing the events of the pulse. [0066 item 9]

**The rate of a landing.** While a force lands men, every ship's rate (berth or lighter) is multiplied by a factor: `ground.toml: unprepared_landing_share` plus the rest of 1 times the force's loading over 100. The force lands men while it holds an unload order in a coast hex and has men aboard any ship, or has begun landing in that hex and its unloading there is not yet complete (`landing_at`, below). The factor covers everything the force unloads in that time: the men, then the supply and the fuel, the berths' work and the hour the unloading completes. Loading counts only if the force's prepare-landing order is for the hex it lands in; at any other hex, or with no order, it counts as 0 and the factor is the share. A force that carries no men, and has not begun a landing, unloads as chapter 2 says, with no factor. A load order has no factor. With the frozen step 1 share of 1 the factor is 1 for every force. [0095 item 5]

**The first men of a unit.** When men of a unit land in a hex and the unit has no man ashore, which includes a unit just made by division, three things follow, each citing that `MenLanded` event: the unit gains landing fatigue and loses its rehearsal, and it leaves its force's roster (Preparing a landing, below). [0095 items 6 and 7]

The breakdowns of `CargoUnloaded` and `MenLanded` gain the factors `port_rate_t_per_h` (the port's rate before the factor), `unprepared_landing_share`, `loading` (the loading counted) and `loading_factor`, only where a landing force works and the share is below 1. With the frozen share of 1 every breakdown is as it was. [0095 item 5]

A side sees: `MenLanded` and `UnloadingComplete`, whole, for its own unit and force.

### A base formed by a landing

When a force lands its first man in a hex where its side has no base, a base of the side is formed there before the men land. It has the next free base number, counted over all the bases of both sides (the highest plus one), the name "Landing at (q,r)" with the hex, the port of the hex, no supply or fuel, no guns, no repairs, no aircraft and no airfield (Units that work, below). It is the side's lowest-numbered base in the hex only if it has no lower. `BaseFormed` records it, with the side, hex and force, citing the unload order and the arrival. A force with no men aboard forms no base, so a supply run to an empty hex changes nothing. A hex that holds a base of the side already, formed or not, forms none. The base is looked for when the landing is prepared, not before, so when two forces of one side land in one empty hex in the same pulse, the force that is taken first forms the base and the second finds it and forms none. [0066 item 11]

A base formed by a landing, and a base formed by a march (below, Marching overland), is not on the enemy's map until it changes side (below, A base that changes side) or a ground report finds it (below, What a side knows of the enemy ashore). It is on its own side's, in every list and answer a side is given of its own bases, and it works as any other base of its side in a rule that reads a base: it holds the stocks unloaded into it, it feeds the men ashore (below), it can be loaded from and unloaded to, and a base count is recorded for it at the end of each day. The other side is told nothing of it until then: chapter 1, What a side may know, gives the map and the base strike refusal. [0066 item 12, 0090 item 18, 0092, 0094 item 9]

A side sees: `BaseFormed`, whole, for its own base. The enemy sees nothing.

### A unit divided

A unit is divided when its men would be ashore in two hexes. When a force is about to land men of a unit that has men ashore in a hex other than the force's hex, all the unit's men aboard the force's ships become a new unit first. The new unit has the next free number, counted over all the land units of both sides (the highest plus one), the side of the unit it came from, no men ashore, and the name of that unit with ", detachment" and its own number after it. The men ashore elsewhere keep the unit. It is the new unit that lands. The new unit takes the experience of the unit it came from and nothing else of what that unit has. It starts on defend with no posture order, fatigue 0, fed share 1, no march, no rehearsal, no days of training or of fighting counted, the activity ready with no order, and no place on a roster, whatever the parent's posture, fatigue or march. A part of a working unit is a working unit: it takes the mark, and its activity is work with no job named. Its number is the highest land unit number plus one, so it is not in the series of 1,000,001 and above that task forces and air groups made in play take (chapter 1, What a side may know). Units are never merged. A unit whose men ashore are in the force's hex, or that has none ashore, is not divided. `UnitDivided` records the unit it came from, the new unit, the force and the men, citing the unload order and the arrival. [0066 item 14, Settled before the build, 0097 item 6]

A side sees: `UnitDivided`, whole, for its own unit.

## Lost at sea

Men aboard a ship that sinks are lost with her, all of them, at the moment she sinks. `MenLostWithShip` records the unit, the ship and the men, citing the `ShipSunk` event. The ship then carries none. A unit left with no men ashore and none aboard any ship is destroyed (below). The side that sank the ship is not told she carried men. [0066 item 15]

A side sees: `MenLostWithShip`, whole, for its own unit. The enemy sees nothing.

## The men's supply

### Feeding

If the ground figures are loaded, the night pulse has a phase of its own for the men's supply, after the daily supply for aircraft and before air group rest (chapter 1, The night pulse). For each hex with men of a side ashore, hexes in hex order (by q, then by r) and, in a hex, sides in the order of their codes (Allied first), the men are the men ashore of all the side's units in the hex, and the bases are every base of the side in the hex. There is no random number. [0066 items 16 to 18, Settled before the build, 0090 item 24a]

The charge is `ground.toml: daily_supply_t_per_man` times the men. The bases are taken in base order. Each base with a stock above 0, while any of the charge is still owed, is asked for what is still owed and pays as much as its stock covers. The payment comes off the one supply stock that pays for aircraft, after the aircraft's charge in the night, so a stock too short for both feeds the aircraft. Each base that pays records its own `BaseSupplySpent` (reason men): the men, the amount asked, the amount paid and the stock left, with no cause. Its breakdown has the factors `men_ashore` and `daily_supply_t_per_man`, and `paid_by_earlier_bases_t` only where an earlier base of the side paid part of the charge. A base with no stock, or no base, is asked nothing and pays nothing, and no payment is recorded. A stock the men's charge empties makes the base out of supply from the start of the next pulse (chapter 5, A base out of supply). [0066 items 16 to 18, 0090 item 24a]

The men fed are the tons paid by all the bases together over the charge for a man, cut to a whole man and never more than the men. The rest are unfed. `MenFed` is recorded for every side and hex with men ashore, also where nothing was paid: the side, hex, the base (the first base that paid, or, if none paid, the lowest-numbered base of the side in the hex, or none if the side has none there), the men, the fed and the unfed. A hex with one base of the side reads as it always did. It cites each payment there was. Its breakdown has the factors `men_ashore`, `paid_t` and `daily_supply_t_per_man`, and its result is the men fed. Men aboard ships do not eat. Where a unit's weight is not the standard, the charge and the men fed are worked as What a land unit is good at says. [0066 items 17 and 19, 0090 item 24a, 0098 item 5]

A side sees: `MenFed` for its own men and `BaseSupplySpent` for its own base, whole. The enemy sees nothing.

### The fed share

The fed share of a unit is the men of its side fed in its hex over the men charged there, set at the same moment by `MenFed`: a fixed-point number, the men fed over the men charged cut at six decimals, the same for every unit of the side ashore in the hex. A side with no base in the hex, or with no stock in it, has a fed share of 0 after that night. A unit with men aboard and none ashore at the charge has its share set back to 1, and `FedShareRestored` is recorded for it, only if the share was not 1 already. A unit destroyed keeps the share it had. A unit that lands after the charge has the share it had, 1 if it never had a charge, until the next night. The fed share is read by the fight (below, Firing men). [0090 item 8]

A side sees: `MenFed` as above, and `FedShareRestored` and its own units' fed shares, whole. The enemy sees none of them. [0090 item 29]

### The unfed are lost

Where any man is unfed, a share of them is lost: the unfed times `ground.toml: unfed_loss_share_per_day`, cut to a whole man, and one man at least, and never more than the unfed. The loss is taken from the side's units with men ashore in the hex, in proportion (below). [0066 item 19]

### Taking men in proportion

With n men to take from units of m1, m2 and so on, of M men in all, in unit order, unit i gives n times mi over M, cut to a whole man. The men still to take are then taken one from each unit in unit order, round again if need be, from units that still have men. No unit gives more than it has. This rule serves the unfed and the losses of a fight. [0066 item 20, 0090 item 9]

Each unit that gives men records `MenLostUnfed`: the unit, the hex, the men lost and the men it has left ashore, citing the `MenFed` event. Its breakdown has the factors `unfed_in_hex`, `unfed_loss_share_per_day`, `loss_in_hex`, `unit_men_ashore`, `men_ashore_in_hex` and `in_proportion` (the loss in the hex times the unit's men over the men in the hex, cut), and its result is the men the unit lost. A unit with no men left ashore has no hex.

A side sees: `MenLostUnfed`, whole, for its own unit.

### Destroyed

A unit that has lost its last man, ashore or aboard, by the loss at sea, by the unfed, in a fight, to fire or by surrender, is destroyed. `LandUnitDestroyed` records it, citing the event that took the last men. The unit keeps its number and stays in the state with no men. [0066 item 1]

A side sees: `LandUnitDestroyed`, whole, for its own unit.

## Posture

### The posture and its order

A land unit is on defend or on attack. It starts on defend, or on the posture its scenario entry gives. The order `set_posture` names a land unit and a posture, `defend` or `attack`. It is a standing order, in a slot of its own, and stands until another replaces it. It is applied when it is taken in, at the start of the day: the unit's posture is set and the order is kept as the order that set it, so the day's fight reads it. [0090 items 1 and 2]

It is checked at intake in this order, and the first reason that applies is recorded (chapter 1, Every reason an order is refused):

1. **Not your land unit.** The unit does not exist or is the enemy's. One reason serves both.
2. **Land unit destroyed.** The unit has no men ashore and none aboard any ship.
3. **A working unit.** An attack order for a working unit: "a working unit does not attack: it is always on defend". An order for defend is accepted.
4. **An instruction on a defend order.** The order is defend and carries `when_spent`: "only an attack order says what the unit does when its attack is spent" (`instruction_needs_attack`).
5. **No men ashore.** The unit has no man ashore (it has men aboard ships only).

An attack order may carry one instruction, `when_spent`, `hold` or `fall_back`: what the unit does when its attack is spent (below, What a spent attacker does). Only an attack order takes it: a defend order with one is refused by the scenario loader and, from a side's agent, by the engine (the reason `instruction_needs_attack`). The instruction is kept with the unit (`LandUnit.when_spent`, 0 for none in the hash) as long as the order that set it is kept. It is spent with the order: when the unit goes back to defend, when it falls back beaten, and whenever another posture order replaces it. A unit that is on attack by its scenario entry has no order and so no instruction. [0105 items 6 and 7]

An attack order needs no enemy. A unit on attack in a hex where nothing is to be attacked does nothing, and is not tired by it (below, Fatigue); the one case in which it has an effect with no enemy man in the hex is a base with no men (below, A side beaten). [0090 items 2 and 3]

A side sees: its own `OrderAccepted` or `OrderRejected`, with the reason, whole. The enemy sees nothing. A unit's line shows the instruction of its standing attack order to its own side. [0090 item 29, 0105 item 9]

## Marching overland

### Where men can march

Men march only along a land link of the map (chapter 2, Land links): a pair of neighbouring hexes the map joins by land. A map with no links allows no march. The map, and so every link, is common knowledge. [0094 items 1 and 2]

### The march order

The order `march` names a land unit and a hex. It is a standing order, in a slot of its own, and stands until the unit arrives or another march order replaces it. The hex is a neighbour of the unit's hex joined to it by a land link, or the unit's own hex. [0094 item 3]

It is checked at intake in this order, and the first reason that applies is recorded (chapter 1, Every reason an order is refused):

1. **Not your land unit.** The unit does not exist or is the enemy's. One reason serves both.
2. **Land unit destroyed.** The unit has no men ashore and none aboard any ship.
3. **No men ashore.** The unit has no man ashore.
4. **No land link.** No land link joins the unit's hex to the hex named, and the hex named is not the unit's own.

An accepted order does this when it is taken in, at the start of the day:

- An order to the unit's own hex stops a march. If the unit has one, it is cleared and `MarchStopped` is recorded with the reason ordered, the hex it was marching to and the days marched. If it has none, nothing is done and nothing is recorded.
- An order to the hex the unit is already marching to keeps the days marched and the days counted.
- An order to any other hex starts a march to it with 0 days marched, replacing any other.

A unit may march out of a hex where the enemy has men. There is no pursuit and no rear guard. A unit that is falling back (A beaten side's men, below) takes a march order as any unit does: it replaces the falling back, and to the hex the unit is already falling back to it keeps the days marched and the days counted. The march is then a march by order, and carries supply (below). [0094 items 3, 4 and 7, 0104 item 5]

A side sees: its own `OrderAccepted` or `OrderRejected`, with the reason, whole, and `MarchStopped`. The enemy sees nothing.

### A day's march

The marches are the first step of the ground combat phase, before any hex is worked. Each unit with a march, in unit order, is taken in this order:

1. **It stops** if it has no man ashore (`MarchStopped`, reason no men ashore), or if its men ashore are in a hex that no land link joins to the hex it marches to, which happens when they went aboard ships and landed elsewhere (`MarchStopped`, reason no land link). The march is cleared and the unit stays where it is. `MarchStopped` cites the march's cause: the march order, or for a falling back (A beaten side's men, below) the `UnitFellBack` event.
2. **Otherwise it marches one more day.** `MarchDay` records the unit, the hex it is in, the hex it marches to, the days marched, the days counted (What a land unit is good at, below: the same as the days marched for a standard unit) and `ground.toml: march_days_per_hex`, citing the march's cause (the march order, or the `UnitFellBack` event). The unit has marched this day for the fatigue step (below).
3. **It arrives** if the days counted now reach `march_days_per_hex` (below; for a standard unit, the days marched now equal it). Otherwise the days marched and the days counted are kept for the next day.

Until it arrives the unit is where it was. It is fed there, can be hit there, and fights there as any unit does. Only men ashore march. Men of the unit aboard ships stay aboard, and if they later land in another hex than the unit's, they become a new unit (above, A unit divided). No part of a unit marches alone. [0094 items 4 to 7]

A side sees: `MarchDay` and `MarchStopped`, whole, for its own unit. The enemy sees nothing.

### Arriving, and the supply carried

When a unit arrives, its men ashore are in the hex it marched to and its march is spent. The unit takes part in that day's fight in the new hex. There is no random number. [0094 item 4]

**The supply is taken.** The unit carries `ground.toml: march_supply_days` days of the daily charge for its men: `march_supply_days` times `ground.toml: daily_supply_t_per_man` times the men ashore. It is taken from the unit's side's bases in the hex it left, in base order. Each base with a stock above 0, while any is still wanted, is asked for what is still wanted and pays as much as its stock covers. Each base that pays records `BaseSupplySpent` with the reason march (a unit whose weight is not the standard carries the days of its own charge, below): the men, the amount asked, the amount paid and the stock left, citing the unit's `MarchDay`. Its breakdown has the factors `men_marching`, `march_supply_days` and `daily_supply_t_per_man`, and `taken_from_earlier_bases_t` where an earlier base gave part. A hex with no base of the side, or only empty ones, gives nothing. [0094 item 9]

**The supply is put down.** What was taken is put into the side's lowest-numbered base in the new hex. If the side has none there and something was carried, a base is formed there first. It has the next free base number counted over both sides, the name "Dump at (q,r)" with the hex, the port of the hex, no guns, no repairs and no aircraft, and it is not on the enemy's map (above, A base formed by a landing, says what that means). `MarchBaseFormed` records it with the side, hex and unit, citing the supply events and the day's march. A unit that carries nothing forms no base, and is unfed where it arrives unless a base is there. A unit that fell back carries nothing (A beaten side's men, below): its march has an event for its cause and no order, and it takes no supply from the bases of the hex it left, forms no base on arrival, and `MarchArrived` records 0 carried and no base. [0094 items 9 and 10]

`MarchArrived` records the unit, the hex it left, the hex it reached, the men, the supply carried and the base it was put into (none if nothing was carried). It cites the unit's `MarchDay`, each `BaseSupplySpent` and any `MarchBaseFormed`. Its breakdown has the factors `men_marching`, `march_supply_days`, `daily_supply_t_per_man` and `wanted_t`, and its result is the supply carried.

A side sees: `MarchArrived`, `MarchBaseFormed` and `BaseSupplySpent` for its own unit and bases, whole. The enemy sees nothing: it learns that men have come by a ground report, and of a dump when a report finds it (below, What a side knows of the enemy ashore). [0094 item 11]

### Tiredness on the march

A day's march tires a unit by the fatigue rule below: it gains `ground.toml: fatigue_per_march_day` in place of resting, and if it also fired on attack that day it gains both. The change is made after the fight, so a column that arrives and fights the same day fires with the fatigue it had before that day's change. A day on which a march stopped is not a march day. [0094 item 8]

A side sees: `LandUnitFatigue`, as in Fatigue below.

## The fight ashore

### The ground combat phase

If the ground figures are loaded, the ground combat phase runs once a day, after the third pulse and before the day's base count (chapter 1, After the pulses). It does the marches first (above, Marching overland). It then takes the hexes that hold men ashore, as they are after the marches, in hex order (by q, then by r), and finishes one hex before the next: a unit that arrived is in its new hex. In each hex it does, in this order: the day's fight, if the hex is contested (with the test of an attack that fails, worked before any die is rolled); each unit's fatigue, with the fatigue of a failed day; the tired units back to defend; what each spent attacker does, in unit order; the test of a side beaten; the bases of a beaten side changing side; and the beaten side's men falling back or surrendering. Last, the ground reports of the hex (below, What a side knows of the enemy ashore). [0090 items 4, 12, 14, 15 and 17, 0094 item 4, 0105]

### A hex is contested

A hex is contested on a day when both sides have men of fighting units ashore in it and at least one unit there is on attack. Working men are not counted (Units that work, above). A hex that is not contested has no fight. [0090 item 4, 0099 ruling 1]

A side attacks in the fight if any of its units in the hex is on attack. A side that does not attack defends. All of a side's units in the hex fight as one body: the units on defend of a side that attacks fire with the rest, as attackers. If both sides attack, each fires as an attacker. There is no position inside a hex. [0090 item 5]

`GroundFight` records the hex and which sides attack, citing the posture orders of the units on attack. A unit on attack because its scenario entry says so has no posture order, and adds no cause; a fight whose only attackers are such units cites nothing. [0090]

### Firing men

A unit's firing men are its men ashore times its fed share, times 100 less its fatigue over 100, cut to a whole man. The second product is one `mul_div`, so nothing is cut in the middle. Men aboard ships do not fight and are not hit. The men of a working unit do not fire: they have no firing men, no shots and no fire event. Hunger and tiredness each cut this one number and nothing else. [0090 items 6 and 13]

### Shots and chance

Each fighting unit in the fight fires, a unit of the side on defend too. A working unit does not. Its shots are its firing men over `ground.toml: men_per_shot`, not cut: the part of a shot is rolled at that part of the chance, by the rule that surface combat uses (chapter 4, Each shot is rolled). Each shot hits at `ground.toml: hit_chance` if its side attacks, or at `ground.toml: defender_hit_chance` if it defends: the side's chance. A unit's chance is the side's times `1 + ground.toml: experience_fire_strength x (the unit's experience - 2)`, kept within 0 and 1 (Experience and training, below). At the frozen strength of 0 the factor is 1. A unit's shots are also times its weight factor, and the chance of a unit whose side defends is also times its holding factor (What a land unit is good at, below) and its side's defences factor (Units that work, above). Each unit's shots are rolled on a stream keyed on the day, no pulse, the ground combat subsystem, the draw `GroundFire` (code 6) and the unit, so no stream is shared between units and the order of firing does not matter. [0090 item 7, 0055]

**The limit on engaged men.** A side fires with no more men than can come to grips with the enemy. The fight is worked in two passes. In the first, with no die, every unit's firing men, shots and chance are worked out for both sides. A side's limit is `ground.toml: engagement_ratio` times the other side's fighting men ashore in the hex at the start of the fight (working men are not counted, as for a contested hex). If a side's firing men, all its units together, are above its limit, each of its units' shots are cut in the same proportion: the unit's shots times the limit over the side's firing men. If they are not above it, nothing is cut. The loader and `World::new` refuse a ratio below 1, so the smaller side is never cut and fires in full. The limit is on fire and not on who is hit: losses are still taken from all the side's units in the hex in proportion (below, Losses). Only then are the dice rolled and the fire events recorded, in side and unit order. [0105 items 1 to 3]

`GroundFire` is recorded for every fighting unit in the fight, also a unit with no firing men: the unit, the hex, the firing men, the shots, the chance and the hits. It cites the fight and the `MenFed` event of the day in that hex, if any. The firing men it records are the unit's own, before any limit; the shots are the shots after it. Its breakdown has the factors `men_ashore`, `fed_share`, `fatigue_factor` (100 less fatigue over 100), `firing_men`, `men_per_shot`, `shots` and `chance`, and its result is the shots times the chance. Where the strength is above 0, the factors `side_chance`, `experience` and `experience_factor` come between `shots` and `chance`; with the strength at 0 the breakdown is as it was. Where the limit cuts the unit's shots, the factors `shots_before_limit`, `side_firing_men` and `engaged_men` (the limit) are added. [0090 item 7, 0097 item 2, 0105 item 1]

### Losses

Both sides fire on the strengths they had at the start of the fight, and the losses are worked out for both before either is taken. A side's loss in the hex is the hits the enemy scored times `ground.toml: men_per_hit` (a whole number), never more than the side's men ashore in the hex. It is taken from the side's units in the hex in proportion (above, Taking men in proportion), working units with the rest. A hit takes the same men whoever fires and whoever is hit. [0090 item 9, 0066 item 20]

Each unit that loses men records `MenLostInFight`: the unit, the hex, the men lost and the men it has left ashore. It cites the fire events of the other side's units. Its breakdown has the factors `loss_in_hex`, `unit_men_ashore`, `men_ashore_in_hex` and `in_proportion`. A unit with no men left ashore has no hex, and a unit with no man anywhere is destroyed (above, Destroyed). [0090 items 9 and 10]

A side sees: `GroundFight`, cut to the hex, for a fight its men are in. `GroundFire` for its own unit, cut to the unit, the hex, the firing men and the shots, with the breakdown factors `men_ashore`, `fed_share`, `fatigue_factor`, `firing_men`, `men_per_shot` and `shots`, and `experience` and `experience_factor` where they are given, and, where the limit cuts, `shots_before_limit`, `side_firing_men` and `engaged_men` (a side's own men and the limit on them: its own units' fire), and no chance or hits (so no side chance either). `MenLostInFight` for its own unit, whole. It is not told its hits, the enemy's units, the enemy's men or how many of them there are. The enemy's own half of the fight is the enemy's. [0090 item 28, 0105 item 9]

### Fatigue

A land unit has fatigue, from 0 to 100, the scale air groups and task forces use. After the fight (or with no fight), each unit that still has a man ashore, in unit order, is changed once:

- a unit that marched a day in this phase (above, Marching overland) gains `ground.toml: fatigue_per_march_day`, up to 100, in place of resting;
- a unit on attack that fired, which is a unit on attack in a fight whose shots are above 0, gains `ground.toml: fatigue_per_attack_day`, up to 100; one that also marched that day gains both, the sum capped once at 100;
- a unit on attack that fired, on a day its side's attack failed (below, An attack that fails), also gains `ground.toml: failed_attack_fatigue`, beside the day's attack fatigue, the sum capped once at 100: a unit on attack that fired nothing gains neither, as before;
- every other unit, a unit on attack that did not fire included, loses `ground.toml: rest_per_day`, down to 0.

A unit also gains fatigue when its first men land (above, Landing men): `ground.toml: unprepared_landing_fatigue` times 100 less its rehearsal for that hex over 100, up to 100. It is a change of its own, made in the loading phase when the men land, and it wears off by the rules above, so men put ashore in a hurry fire worse for some days. [0095 item 6]

A unit with no man ashore keeps its fatigue. `LandUnitFatigue` (gained, rested, marched, marched and attacked, or landed, and the new fatigue) is recorded only when the value changes. It cites the unit's fire event where there was one, its march day event where there was one and, for a failed day, the `GroundAttackFailed` event; a landing's change cites the `MenLanded` event. Its breakdown has the factor `fatigue_before` and then `rest_per_day`, or `fatigue_per_march_day`, `fatigue_per_attack_day` and `failed_attack_fatigue` for the part that applies, or, for a landing, `unprepared_landing_fatigue` and `rehearsal` (the rehearsal counted, 0 if it is for another hex). Fatigue cuts the men who fire (above, Firing men) and nothing else. The step comes after the fight, so the men who fire on the day a unit arrives are worked out from the fatigue it had before that day's change. [0090 items 11 to 13, 0094 item 8, 0095 item 6, 0105 item 5]

### An attack that fails

A side's weight of fire in a day's fight is the sum, over its units, of each unit's shots (after the limit on engaged men) times its chance to hit. It is worked out in the first pass, before any die is rolled. A side that attacked has failed that day if its weight of fire is below the other side's. If both attacked, the weaker fails; if they are equal, neither does; a side that did not attack never fails. A side that attacked with a weight of 0 (its men too hungry or too tired to fire) has failed if the other's is above 0. The defender's better chance to hit, its defences and its skill at holding ground are all in its weight, so an attack on a prepared defence needs real superiority. [0105 item 4]

`GroundAttackFailed` records the side, the hex and both weights, after the day's fire events and before the losses. It cites the fight and both sides' fire events. Its breakdown has the factors `weight` and `enemy_weight`. It is recorded whatever the figures are, the frozen ones included: with `failed_attack_fatigue` at 0 it changes nothing else. A failed day costs the attackers fatigue (above, Fatigue). Nothing else follows from it: the rule that exists does the rest, since a unit whose fatigue is above the limit goes back to defend (below), so a hopeless attack ends in a day or two and one that is winning goes on. [0105 items 4 and 5]

A side sees: `GroundAttackFailed`, for the side that attacked and failed, cut to the hex: it is told that its attack failed and where, which its men know, and not the weights or the breakdown. The enemy is told nothing: it learns the attack has stopped by not being attacked. The comparison of the two weights is the umpire's. [0105 item 9]

### The tired stop attacking

After the day's change, a unit on attack with men ashore whose fatigue is above `ground.toml: attack_fatigue_limit` goes back to defend, whether or not it fought. Its attack order is spent: the order that set its posture is no longer kept as the standing one, and the unit stays on defend until a new order sets it. `LandUnitBackToDefend` records it with the fatigue, citing the unit's fatigue event (or its fire event if the fatigue did not change). Its breakdown has the factors `fatigue` and `attack_fatigue_limit`. One figure serves every unit, and no order sets the limit. [0090 item 14, 0105 item 5]

A side can ask this choice for its own land units before it is made (`measure`'s `choices`, `back_to_defend`): the side may state the fatigue to test, and the answer is whether the unit would go back to defend, from its posture and that fatigue. [0100]

### What a spent attacker does

When a unit goes back to defend because it is tired, its attack is spent. After every unit of the hex has been tired or rested and sent back to defend, each unit that went back, in unit order, does what its attack order said, or the default. This is read only where the enemy has fighting men ashore, counted after the day's losses, among the enemy's units that were in the hex at the start of the phase (after the marches). A unit that is already marching is left as it is, as a beaten side's is. A unit that went back to defend with no enemy fighting man in its hex does nothing more.

- **Its order's instruction** (`when_spent`) is read first: `hold` or `fall_back`.
- **With none, it holds if it can be fed where it stands, and else falls back.** It can be fed if its side has a base in the hex with supply above 0, a dump its column put down included. Whether reserves or support are coming is the judgement of whoever gave the order: the engine does not guess it.
- **Hold** changes nothing: the unit is on the defence already, and has no event of its own beyond `LandUnitBackToDefend`.
- **Fall back** takes the hex by the rule of a beaten side (A beaten side's men, below: the first hex joined by a land link that holds a base of its side, else the first with no man ashore of an enemy fighting unit). The unit gains `ground.toml: beaten_fatigue`, up to 100, and is set to march to the hex with 0 days marched, carrying no supply and forming no base where it arrives. `SpentUnitFellBack` records the unit, its side, the hex it leaves, the hex it goes to, its men and its fatigue afterwards, citing the unit's `LandUnitBackToDefend`; the march's cause is that event. Its breakdown has the factors `fatigue_before` and `beaten_fatigue`. It is not `UnitFellBack`, which cites a defeat. If there is no hex to go to the unit stays, on the defence, with no fatigue added and no event. It does not surrender: its side was not beaten. [0105 items 6 and 7]

A side can ask this choice for its own land units (`measure`'s `choices`, `attack_spent`): the answer is `hold` or `fall_back`, from the unit's instruction and the side's own bases in the hex and their supply, which are all the choice reads. Where the unit falls back to reads where the enemy's men truly are, so it stays with the rules and is not asked. The question carries the hex the unit stands in when the engine asks, since a unit can march in and attack the same day. One case differs: a column that arrives in the hex in the same step puts down a dump before the fight, so the engine finds a base of the side with supply where the side's view, before the march, found none. [0100, 0105]

A side sees: `SpentUnitFellBack`, whole, for its own unit. The enemy is told nothing of it: it learns that the men have gone by its own men's report of the hex. [0105 item 9]

A side sees: `LandUnitFatigue`, `LandUnitBackToDefend` and its own units' postures, fatigue and fed shares, whole. The enemy sees none of it. [0090 item 29]

## A side beaten

After the day's fight a side is beaten in a hex if the other side attacked there that day, and it has no man ashore in the hex or fewer than `ground.toml: beaten_share` times the other side's men ashore. Men are counted after the day's losses, and only the men of fighting units are counted, on both sides (Units that work, above). If both sides attacked, each is tested. Both are beaten at once only if neither has a man left, and then neither is (the loader and `World::new` refuse a `beaten_share` of 1 or more, so otherwise one side at most is beaten). [0090 item 15]

A side whose attack failed that day (above) may still be beaten, if the other side also attacked: the tests are separate. The test counts the men in the hex as they stand, a spent attacker's included, for a unit that falls back marches from the next day. Then the rule of a beaten side's men rules, surrender included. [0105 item 8]

A base with no men, or with only working men, falls without a fight. In a hex that is not contested, where a side has a unit on attack and the other side has a base and no man ashore, the other side is beaten in the ground combat phase of that day. One man landed on attack takes an empty airfield. [0090 item 16]

`SideBeatenAshore` records the side, the hex, the side's men and the enemy's men, and whether there was a fight. It is recorded whenever a side is beaten, whether or not it holds a base there. It cites the fight, or, with no fight, the posture orders of the units on attack (a unit on attack by its scenario entry has none, so adds none). Its breakdown has the factors `men_ashore`, `enemy_men_ashore` and `beaten_share`, and its result is the share times the enemy's men. [0090 items 15 and 16]

A beaten side loses every base it holds in the hex (below). Its men who are left then fall back, or surrender (A beaten side's men, below). Until a unit has left the hex it is in it, unfed while its side has no base there, with a fed share of 0 after the next night, and can be fought. They can be taken off by sea and they can attack on a later day. [0090 item 17, 0104]

A side sees: nothing of `SideBeatenAshore`: it weighs both sides' men, and it is the umpire's. A side learns it was beaten by losing its base (below). [0090 items 28 and 30]

## A base that changes side

When a side is beaten in a hex, each base of that side in the hex, in base order, changes side to the other side. The rules below run for each base, in this order. [0090 items 17 to 23]

1. **The base.** It keeps its number, name, hex, guns and repair figures. Its side changes and it is on both sides' maps from then on, whatever it was before: a base a landing formed is told to the enemy now (chapter 1, What a side may know). Its supply and its fuel are each cut to `ground.toml: captured_stock_share` of what they were, in one product, and the rest is lost. `BaseChangedSide` records the base, the hex, the side it was and the side it is, the supply and fuel that stay and the supply and fuel that were lost. It cites the `SideBeatenAshore` event and what that event cites (the fight, or the posture orders of the units on attack). Its breakdown has the factors `supply_t`, `fuel_t`, `captured_stock_share` and `fuel_kept_t`, and its result is the supply that stays. [0090 items 18, 19 and 30]
2. **Air groups of the beaten side at it are lost.** Each air group of the side the base was, located at the base, in group order, is lost whole: its ready and damaged aircraft go to 0, and its pilots go to 0 where they are tracked. It has the new place `Lost`. It keeps its mission. No group is in the air, because the phase runs after the last pulse. `AirGroupLostWithBase` records the group, the base and the aircraft lost, citing `BaseChangedSide`, and `PilotsLost` is recorded if it had pilots, citing it. The aircraft are counted as aircraft destroyed for the enemy, as a group lost with its ship is (chapter 6). Chapter 5, A group lost with its base, gives what follows the group. A group that was flown out by a rebase order on an earlier day is at another base and is not lost (chapter 5, Rebasing an air group). A group of the side that takes the base, which can only be a group with no aircraft waiting for the base (chapter 5, A group with no aircraft at a base that is not its side's), stays, untouched, and no event names it. [0090 item 20, 0101]
3. **A ship in port at it is lost,** in ship order, and is sunk and scored as sunk. This is a ship in no task force: a force in the hex is at sea and is not touched. `ShipLostWithBase` records the ship and the base, citing `BaseChangedSide`; it is its own event because `ShipSunk` names a task force. The air groups aboard her are lost as with a ship that sinks (chapter 5, A group lost with its ship), and the men aboard her are lost with her (above, Lost at sea), each citing `ShipLostWithBase`. [0090 item 21, 0025]
4. **A home port is lost.** A force of the beaten side whose home port the base was has no home port. `HomePortLost` records the force and the base, citing `BaseChangedSide`. [0090 item 22]
5. **A strike mission is cleared.** An air group of the winning side whose mission was a base strike on the base has the mission rest and no mission order. `StrikeMissionCleared` records the group and the base, citing `BaseChangedSide`. [0090 item 22]

What needs no rule of its own: the beaten side's replacement stream at the base stops (chapter 5, Replacements); a standing cargo order of the beaten side works no cargo there, as at any hex with no base of its side; a force of the beaten side away at the yard from that port is not touched and returns to its hex as before. From the next day the base refuels, repairs and works cargo for its new side, aircraft of the new side that cannot get home may land there, and the men of the new holder in the hex are fed from it. [0090 item 23]

A side sees: `BaseChangedSide`, both sides, at its place in the log: whole to the side that now holds the base, and cut to the base, the hex and the two sides, without the stocks, to the side that lost it. `AirGroupLostWithBase`, `ShipLostWithBase`, `HomePortLost` and `StrikeMissionCleared`, whole, for its own group, ship or force; `PilotsLost` and the events of the men and groups aboard a ship as for any ship lost. The side that took the base is not told what was lost at it. [0090 item 30]

Events of a base are seen by the side that held it when they were recorded. The holder at the time of each event is worked out from the log: the side the base had before its first change of side, handed on at each `BaseChangedSide` event. A base's past, and the numbers a side was given for it, are therefore not handed to its taker or renumbered. [0090 item 24, 0086]

## A beaten side's men

When a side is beaten in a hex, after its bases have changed side, each of its units with men ashore in the hex, fighting or working, in unit order, falls back or surrenders. A unit that is already marching, by any march, when its side is beaten is left as it is: it is not set to fall back and gains no fatigue. [0104 items 1 to 3 and 7]

**Where to.** Of the hexes joined to the hex by a land link (chapter 2, Land links), in hex order: the first that holds a base of the unit's side, read at that moment and whatever else stands there (the bases that changed side have changed, so a base of the beaten side in the hex itself does not count); if none does, the first that holds no man ashore of an enemy fighting unit (a working unit does not count). If there is none, or the map has no land link from the hex, the unit has no hex and surrenders (below). [0104 item 2]

**What a unit that falls back does.** In this order:

1. It gains `ground.toml: beaten_fatigue`, up to 100, on top of its fatigue then. This is after the day's fatigue step, so it does not change that day's fatigue or the units sent back to defend that day.
2. If its posture is attack it goes on the defence, and the posture order that held it on attack is cleared. A unit already on defend is unchanged.
3. It is set to march to the hex it falls back to, with 0 days marched and 0 days counted, by the marching rule (Marching overland, above). Its march has no order: its cause is the `UnitFellBack` event. It marches from the next day's ground combat phase and, being a march like any other, takes the days a march takes, tires the unit by `fatigue_per_march_day` and is fed where it is until it arrives.
4. `UnitFellBack` records the unit, its side, the hex it leaves, the hex it falls back to, its men ashore and its fatigue afterwards. It cites the `SideBeatenAshore` event. Its breakdown has the factors `fatigue_before` and `beaten_fatigue`, and its result is the fatigue afterwards.

**It carries no supply.** Its side has just lost its bases in the hex. The unit takes nothing from any base and forms no base where it arrives, so it is unfed there unless a base of its side is there (above, Arriving, and the supply carried). [0104 item 3]

**What it can do from the next day.** It takes orders as any unit does. A march order replaces the falling back: to another hex it starts a march with 0 days; to the hex it is falling back to it keeps the days marched, and as a march by order it carries supply from the bases of its side in the hex it left. An order to its own hex stops the march. A posture order of attack is taken and the march goes on. While it is still in the hex it can be fought as any unit can: the hex is contested if a unit there is on attack and the enemy has fighting men there, and a victor that goes on attacking fires on men who have not yet left. [0104 items 5 and 6]

**A unit with no hex surrenders** in the same phase, after the units that fall back. `MenSurrenderedAshore` records the side, the hex and the men of all the units that surrendered there, once for the hex, citing the `SideBeatenAshore` event. Then each unit, in unit order, records `UnitSurrendered`: the unit, its side, the hex and its men ashore, citing the hex's `MenSurrenderedAshore`. Its men ashore are lost: they go to 0 and the unit has no hex. If it has no man aboard a ship either, it is destroyed (above, Destroyed), citing `UnitSurrendered`. The men count as men lost on the `men_lost` objective of chapter 6, as the men lost in a fight do. A side that means to take its men off by sea must do it before it is beaten. [0104 item 7]

A side sees: `UnitFellBack` and `UnitSurrendered`, whole, for its own unit; and `MenSurrenderedAshore`, whole, when the surrendered men are its own. The enemy is told nothing of a falling back: it learns that the men have gone as it learns anything of the enemy ashore, by a ground report. Of a surrender it is told one event for the hex, with the hex and the men and no unit and no count of units, so it cannot learn how many units surrendered; it is not shown `UnitSurrendered`. The events that cite the test that beat the side, which no side is given, have that cause dropped as chapter 1 says. [0104 item 8]

## The day's activity

### The activity and its order

A land unit has an activity for the days on which it neither marches nor fights. It is ready, train or work. It is ready at the start, or work for a working unit. No scenario key sets it except the working mark: a scenario order does, as it does any other order. The order `set_activity` names a land unit and an activity, `ready`, `train` or `work`; with `work` it may also name a `job`: `airfield`, `port`, `aircraft` or `defences` (Units that work, above). It is a standing order, in a slot of its own, and stands until another replaces it. It is applied when it is taken in, at the start of the day: the unit's activity is set and the order is kept as the one that set it. [0099 items 3b and 4, 0097 item 4]

It is checked at intake in this order, and the first reason that applies is recorded (chapter 1, Every reason an order is refused):

1. **Not your land unit.** The unit does not exist or is the enemy's. One reason serves both.
2. **Land unit destroyed.** The unit has no men ashore and none aboard any ship.
3. **A working unit works.** The unit is a working unit and the activity is ready or train: "a working unit works: it does not stand ready or train". An order for work, with or without a job, is accepted.

A unit that has men aboard ships only may be set to train, and any other activity: the order needs no man ashore. It gains nothing from training until all its men are ashore. [0099 item 4]

A side sees: its own `OrderAccepted` or `OrderRejected`, with the reason, whole. The enemy sees nothing.

### One activity a day

A unit does one thing in a day, and this table says which. It is worked out once a day, at the end of the ground combat phase, when every hex is done, so it is known whether the unit marched, whether its hex was contested and how it was fed that night. The first row that applies to the unit is its day. A unit with no man ashore and none aboard is not taken. [0099 item 3b]

| In this order | The unit's day is |
| --- | --- |
| It marched a day in this phase, or its hex was contested (it has men ashore in a hex where a fight was held) | None of the rest. It marched or fought. |
| It is a working unit, or its activity is work | A day of work, at its job (Units that work, above) |
| Its activity is train | A day of training, if all its men are ashore at a base of its side and its fed share is 1 (Experience and training, below); otherwise nothing |
| Its activity is ready and it is on the roster of a force that qualified today, and every man of it is ashore in that force's hex or aboard that force's ships | A day of rehearsal for the landing of the force's order (Preparing a landing, below) |
| Its activity is ready in any other case | Nothing |

So rehearsal is what a rostered unit does when it is not told to train. A scenario or a staff schedules a raw unit to train first and to rehearse after by changing its activity. Its place on the roster does not change, so the force's loading is not cut for it. [0099 item 3b]

A side can ask this choice for its own land units before it is made (`measure`'s `choices`, `unit_day`): the side states whether the unit marched and whether its hex is contested, which it cannot know for certain, and the answer is what the unit's day would be. [0100]

A unit set to work, or a working unit, makes no training and no rehearsal that day. [0099 item 3a]

The phase does its parts in this order, once a day. First each force's loading (Preparing a landing, below), which says which forces qualified. Then each unit, in unit order, by the table, and a unit's work is made at that point, after the fights and any base changing side. Last the days of fighting (Experience and training, below). Each gain is given once for a unit for a day. A unit at its ceiling of training, or at 100 of rehearsal, still spends the day. [0099 items 3b and 3d]

A side sees: as the sections that follow say.

## Units that work, and what they build

### The working unit

A scenario may mark a land unit as a working unit with `working = true`. A working unit's men do not fire in a fight. In everything else they are men: they are carried, landed, fed, hit in proportion with the rest, and lost. A working unit is always on defend, and its activity is always work. A scenario that gives one the posture attack is refused, and so is an order that makes it attack, stand ready or train. A unit not marked is a fighting unit, and fights as before; the order `set_activity` may put it to work (above, The activity and its order). A part of a working unit made by division is a working unit, with the activity work and no job named (above, A unit divided). [0099 items 1 and 3a]

A unit's works skill is a whole number from 0 to 4, 2 being the standard for a unit raised to build (above, What a land unit is good at). A fighting unit set to work has the skill its scenario gives it, 2 if none. [0099 items 2 and 3a]

**Only fighting men count in the fight.** Where the rules of the fight count a side's men ashore, they count the men of fighting units and not the men of working units. A hex is contested when both sides have men of fighting units ashore and a unit is on attack. A side is beaten on its fighting men, and the enemy's, as counted for the test. A base whose side has only working men ashore falls without a fight, as a base with no men does. A fighting unit set to work is still a fighting unit and its men count. The ground report counts every man ashore, the working men too. [0099 item 1 and ruling 1]

### The day's work

A unit's day is worked out once a day, at the end of the ground combat phase (above, One activity a day). If its day is work, it makes work then, in unit order, after the marches, the fights and any base changing side. The unit makes work when it has men ashore:

`men ashore x fed share x work_per_man_day x works skill / 2`

and for a fighting unit set to work, times `fighting_unit_work_share`. The fed share is what the night's feeding left it. A unit that marched, or whose hex was contested, makes none: its day is the march or the fight. The work goes to the lowest-numbered base of the unit's side in its hex, as it is at that moment, after the fights and after a base has changed side. A unit in a hex with no base of its side makes none. A day of 0 work records nothing. [0099 items 3 and 3a]

`UnitWorked` records the unit, the base, the job, the work made and a total, citing the order that set the unit's activity, if any, and the `MenFed` event of the night in that hex. The total is, for the airfield, the work done on it; for the port and the aircraft, the day's work so far on that job at the base; for the defences, the base's defences. Its breakdown has the factors `men_ashore`, `fed_share`, `work_per_man_day` and `works_skill`, then `fighting_unit_work_share` for a fighting unit and `work_per_defence_point` for the defences, and its result is the work made. [0099 items 3 and 4]

### The jobs

Each unit has its own job. The order `set_activity` with the activity `work` may name a job, and the job stands until another order replaces it. A unit with no job named works the airfield if its base has none or its airfield is damaged, and else the port. This is chosen each day, so it changes when the field is built, when it is damaged and when it is mended. The work of several units on one job at a base is added. [0099 item 4, 0103 item 8]

| Job | What the day's work does |
| --- | --- |
| `airfield` | Adds to the work done on the base's airfield, until it is built; then mends its damage |
| `port` | The next day, ships at the base work cargo faster |
| `aircraft` | The next night, the base repairs more aircraft |
| `defences` | Adds to the base's defences |

**Airfield.** The day's work of every unit on the airfield is added to the base's work done, and the work done is kept by the base. Once every unit has worked, a base whose work done has reached `airfield_work` has an airfield, and `AirfieldBuilt` records it, citing the day's `UnitWorked` events of that base. Its breakdown has the factors `work_done` and `airfield_work`. The day's work is added whole on the day the airfield is built, however far past the figure it takes the work done, and nothing is added after. A unit with no job named then works the port, unless the field is damaged. [0099 item 5 and ruling 2, 0103]

**Mending.** Work on the airfield of a base whose airfield is built and damaged lowers the damage by the work over `work_per_airfield_damage_point`, never below 0, and the work adds nothing to the work done. Each unit's work is taken in unit order, so a later unit finds the damage the earlier ones left, and the work of a unit that is more than the damage mends only the damage that is there. `AirfieldMended` records the unit, the base, the points mended and the damage after, citing the unit's `UnitWorked` event. Its breakdown has the factors `work`, `work_per_airfield_damage_point` and `damage_before`, and its result is the points mended. Nothing is recorded for a unit that finds no damage. A fighting unit set to the airfield works at `fighting_unit_work_share`, as at any job. Nothing mends the field by itself: a field with no unit on it stays as damaged as it is. The work is done at the end of the day, after the third pulse, so a shelling by night costs the flying of that day and is mended for the next. [0103 items 7 to 10]

**Port.** The day's work on the port is kept for the base for that day only. The next day, every ship's rate at the base, berth or lighter, for men and for cargo alike, is multiplied by `1 + work / work_to_double_unloading`, and by no more than 2. It multiplies the rate before the landing factor of a force's loading (above, Landing men). The breakdown of the cargo events gains `port_work` and `port_work_factor` where there was work, and they cite the day's `UnitWorked` events. There is nothing to read on the first day. [0099 item 4]

**Aircraft.** The day's work on the aircraft is kept in the same way. The next night, the base repairs its own figure of aircraft and one more for each `work_per_aircraft_repair` of work, cut to a whole aircraft. This holds also where the base's own figure is 0, so work makes a base a place of repair (chapter 5, Repair). [0099 item 4 and ruling 4]

**Defences.** The work adds `work / work_per_defence_point` to the base's defences, not cut to a whole number (450 of work is 2.25 points), up to 100. Defences belong to the base. They do not wear away, and a base that changes side keeps them, as it keeps the work done on its airfield. A scenario may give a base defences to start with. While a side defends in a contested hex, each of its fighting units' chance to hit is multiplied by `1 + defences_strength x defences / 100` of its lowest-numbered base in the hex, and then cut to 0 to 1 as before. A side with no base in the hex has no defences. The breakdown of `GroundFire` names `defences` and `defences_factor` only where the factor is not 1. [0099 items 4 and 4a and ruling 3]

### The airfield

A base has an airfield or it does not. Every base a scenario states has one unless the scenario says `airfield = false`, so no earlier scenario changes. A base formed by a landing or a march has none. A scenario may give a base with no airfield `airfield_work`, the work already done on it, and the loader refuses `airfield_work` for a base that has one. Rebasing an air group to a base with no airfield is refused, aircraft that cannot get home do not land at one, and the loader refuses a scenario that puts an air group at one (chapter 5, Rebasing an air group and Landing elsewhere; chapter 1, Every reason an order is refused). A side that lands on a bare shore has no air cover of its own until its engineers have had their days. [0099 items 5 and 6]

**Damage.** A base's airfield has damage, from 0 to 100, 0 at the start. A scenario may give it (`[[base]]` `airfield_damage`, a whole number or a decimal string from 0 to 100) and the loader refuses it for a base with no airfield. A base with no airfield has none. A hit on the airfield adds `airfield_damage_per_hit` (chapter 5, The attack on a base), to 100 at most. Work on the airfield lowers it (Mending, above). The damage is kept by the base and goes with it if it changes side. It changes how many aircraft a group at the base may fly, and at `airfield_landing_refused_at` or more the field takes no aircraft: no rebase to it, no landing on it, and an arrival due there waits (chapter 5, A damaged airfield). It changes nothing else: not the port, the stocks, the guns, the men or the building of a field not yet built. The invariants refuse damage outside 0 to 100, and damage at a base with no airfield. [0103]

### What a side sees

A side sees its own units' works skills, their jobs and their work, and its own bases' airfield, work done, damage, defences and the day's work. `UnitWorked` is seen whole by the side of the unit, `AirfieldBuilt` whole by the side that built it, `AirfieldMended` whole by the side of the unit that mended, and `AirfieldDamaged` whole by the side that holds the base. The enemy sees nothing of any of them. An enemy base on a side's map is shown with no word on its airfield or its damage, and no ground report tells it. The agent's view gives a side's own base a line of the damage (the damage, the share of every 100 aircraft that may fly, whether the field takes aircraft and the level at which it takes none) only while the damage is above 0, a line for each of its own air groups of the aircraft the damage keeps on the ground, and in the briefing the hits, the mending and the arrivals held back of yesterday. [0099 item 7, 0103 item 11]

### The figures

Ten figures in `ground.toml` (the seven of work, and the three of decision 0103), each required, a whole number or a decimal string. The live values are placeholders that rest on no source. The frozen step 1 copy holds the same ten, and no frozen scenario has a working unit or an airfield with damage, so they do nothing there.

| Key | Value | Range | What it is |
| --- | --- | --- | --- |
| `work_per_man_day` | `1` | 0 or more | Work a man of a standard working unit makes in a day |
| `airfield_work` | `20000` | Above 0 | Work to build an airfield |
| `work_to_double_unloading` | `1000` | Above 0 | Work in a day that doubles the next day's unloading at a base |
| `work_per_aircraft_repair` | `200` | Above 0 | Work in a day for one more aircraft repaired the next night |
| `work_per_defence_point` | `200` | Above 0 | Work for one point of a base's defences |
| `defences_strength` | `0.5` | 0 to 1 | How much full defences raise a defender's chance to hit |
| `fighting_unit_work_share` | `0.5` | 0 to 1 | The share of a working unit's work that a fighting unit set to work makes |
| `airfield_damage_per_hit` | `2` | 0 or more | Damage one hit on an airfield adds |
| `work_per_airfield_damage_point` | `50` | Above 0 | Work to mend one point of damage |
| `airfield_landing_refused_at` | `50` | 0 to 100 | Damage at or above which a field takes no aircraft |

With them a standard battalion of 1,000 builds an airfield in 20 days, a battalion of 1,000 mends 20 points of damage a day, and fifty hits on a field close it. The share of hits on a base that land on its airfield is `air_combat.toml: [base_hit] airfield` (`0.2`; `0` in the frozen copy). [0099, 0103]

## Preparing a landing

### The order

The order `prepare_landing` names a task force, a hex and a roster: the land units that are to land from the force. It is a standing order, in a slot of its own, and stands until the force is given another or until the landing it prepares is done. It may be given at sea, and the roster may be empty. [0095 item 1]

It is checked at intake in this order, and the first reason that applies is recorded (chapter 1, Every reason an order is refused). The reasons for a task force order in general (not your unit, away at the yard, no ship afloat) come first, then:

1. **Not your land unit.** A unit on the roster does not exist or is the enemy's. One reason serves both. The first such unit in roster order decides.
2. **Not a coast hex.** The hex is off the map or is not a coast hex: "H is not a coast hex on the map". The map is common knowledge, so the reason shows nothing a side cannot see.
3. **Land unit destroyed.** A unit on the roster has no man ashore and none aboard any ship.

With no ground figures the order is accepted and does nothing. [0095 item 1, 0099 item 3c]

An accepted order does this when it is taken in, at the start of the day. It sets the force's preparation: the hex, the roster, the loading and the order. The loading is what the force had (below), or 0 for a force with none.

- **The same order again** (the same hex and the same roster) changes nothing, and the earlier order stays the one that is cited.
- **A new hex** sets the loading to 0, records `LoadingLost` (new landing place) if it was above 0, and takes away the rehearsal of every unit of the old roster whose rehearsal is for the old hex, each with `RehearsalLost` (new landing place). A unit of the old roster that is not on the new one also records `LeftRoster` (dropped by order).
- **The same hex with another roster.** A unit dropped leaves the roster at no cost, with `LeftRoster` (dropped by order); its rehearsal is kept. If any unit is added, the loading is multiplied once by `ground.toml: preparation_kept_on_change`, however many are added, and `LoadingLost` (unit added) is recorded if the loading changes. A unit added brings the rehearsal it has for that hex, which is none unless it rehearsed it with another force.
- **A unit named on another force's roster** is taken off that roster, with `LeftRoster` (taken by another force's order). That force loses nothing.

A unit is on one roster at a time. [0095 items 8 to 11, 0099 item 3c]

A side sees: its own `OrderAccepted` or `OrderRejected`, with the reason, whole, and the events above. The enemy sees nothing.

### Loading and rehearsal

A force has loading, from 0 to 100. It is the ships: what is stowed where, and the boats and their crews. A land unit has rehearsal, from 0 to 100, for one hex: the men knowing the plan, the boats and the beach. Both start at 0. No scenario key sets either: a scenario cannot start a force loaded or a unit rehearsed. Both are gained by the day, by doing it, in the day's activity (above). [0095 items 2 to 4]

**A force gains loading** `ground.toml: preparation_per_day`, up to 100, if it qualifies. It qualifies if all of these hold when the day is settled:

- its roster is not empty;
- it is idle, arrived or halted at the centre of its hex, and is not away at the yard;
- a base of its side is in its hex;
- it did not arrive or halt at any hour of that day, and was not under way at any hour of it.

`LoadingGained` records the force, the hex of the order and the new loading, when the loading changes, citing the order. Its breakdown has the factors `loading_before` and `preparation_per_day`. A force at sea, or one that came in during the day, gains nothing. Having the order on paper earns nothing: only days in port do. [0095 item 4]

**A unit gains rehearsal** by the same figure times its assault factor (What a land unit is good at, below; 1 for a standard unit), up to 100, on a day that is a day of rehearsal by the table above: it is ready, it is on the roster of a force that qualified, it did not march, its hex was not contested, and every man of it is ashore in the force's hex or aboard the force's ships. A unit aboard the ships of another force does not rehearse, nor does a unit with some men ashore elsewhere. The rehearsal is for the hex of the force's order. If the unit had rehearsal for another hex, that is lost first, with `RehearsalLost` (another landing place), and it starts again from 0. `RehearsalGained` records the unit, the force, the hex and the new rehearsal, when the rehearsal changes, citing the order that set the force's preparation and the unit's activity order if it has one. Its breakdown has the factors `rehearsal_before` and `preparation_per_day`. [0095 item 4, 0099 item 3b]

A side sees: `LoadingGained` and `RehearsalGained`, whole, for its own force and unit. The enemy sees nothing.

### What it buys

**The rate of the landing.** While a force lands men, its ships work at a rate that follows its loading (above, Landing men, The rate of a landing). The factor is `unprepared_landing_share` plus the rest of 1 times the loading over 100, and the loading counts only at the hex of the order. It covers every ton the force unloads from the first man landed until its unloading there is complete, supply and fuel as well as men. It is for every force that lands men, with the order or without it. With the placeholder figures a force with no loading lands at half the port's rate, and one with 100 at the whole of it. [0095 item 5]

**The fatigue on landing.** When a unit's first men land in a hex, the unit gains `unprepared_landing_fatigue` times 100 less its rehearsal over 100, up to 100. The rehearsal counts only if it is for that hex; at any other hex it counts as none. The gain is the change `landed` of `LandUnitFatigue` (below, Fatigue). Fatigue cuts the men who fire and wears off by the day. [0095 item 6]

### Spent by the landing

- **The unit's rehearsal is 0** when its first men land, in any hex. If it had any, `RehearsalLost` (landed) records it, citing the `MenLanded` event.
- **The unit leaves its force's roster** when its first men land from a ship, in any hex: its landing is done. `LeftRoster` (landed) records it, citing the same event. A force whose roster is empty gains no loading, though its order stands and it keeps the loading it has.
- **The force's loading is 0, and its order is done,** when its unloading at the hex of the order is complete. The force's `landing_at` is the hex where it has begun to land men, set when a ship lands one and cleared when the unloading there is complete. When it is cleared, if the force's order is for that hex, `LoadingLost` (spent by the landing) records the loading, also when the loading was 0, and the preparation is gone. A unit still on the roster goes with it, with no `LeftRoster`. If the order is for another hex it stands, with its loading. [0095 item 7]

A side sees: `RehearsalLost`, `LeftRoster` and `LoadingLost`, whole, for its own unit and force. The enemy sees nothing.

### What a change costs

| The change | The loading | A unit's rehearsal |
| --- | --- | --- |
| A new landing place (a new order for another hex) | Lost: 0 | Lost for every unit of the old roster that had it for the old hex |
| A unit added to the roster, also one taken from another force's roster | Multiplied once by `preparation_kept_on_change`, however many are added | The unit keeps what it has for that hex |
| A unit dropped from the roster, or taken from it by another force's order | Nothing | The unit keeps it |
| A force splits, by the detach order or by stragglers | Each part has the loading the force had | Not copied: it is the unit's |
| Forces join, or a ship joins a force | Written and not reachable: nothing in the game does either yet | |
| A change of date | Nothing here | Nothing here |

[0095 items 8 to 13]

**A split.** The part that leaves takes the roster units all of whose men aboard are in its ships, with the hex, the loading and the order of the parent. The rest of the roster stays. The new part has begun landing where the parent had (`landing_at` is copied). `PreparationShared` records the parent, the new force, the hex, the loading and the units, after `ShipsDetached`, citing it, for a detach order and for stragglers alike. [0095 item 12, ruling of Nov 14, 2026]

**A join.** The rules are written as two functions that nothing calls, since no order joins forces or adds a ship yet: forces that join take the lower loading (the other force's counted only if it is for the same hex, else 0), the union of the rosters and the receiving force's hex and order; a ship that joins keeps `preparation_kept_on_change` of the loading. The order that does either is a stage of step 3. [0095 item 12]

### What a side sees

A side sees its own forces' preparation (hex, roster, loading, the share of the port's rate it would land at) and `landing_at`, and its own units' rehearsal, roster place and the fatigue they would land with, exactly. It sees each day's gain and each loss with its reason, each whole to the side that owns the force or the unit. The enemy sees none of it. [0095 item 14]

## Experience and training

### Experience

A land unit has experience, a whole number from 0 to 4, 2 being its service's standard. It is the scale of a ship's crew (decision 0064) and it means the same: 0 is raw and 4 is the best there is. A scenario gives it (the key `experience`), and it is 2 if the scenario does not. Every real unit in the scenarios of the repository is 2: no real unit has a value until the research is done, as for air groups. [0097 item 1, Real units]

**What it does.** It moves the chance a unit's shot hits (above, Shots and chance). The unit's chance is the side's chance times `1 + experience_fire_strength x (experience - 2)`, kept within 0 and 1. With the placeholder strength of 0.25 a raw unit's shots hit half as often as a standard unit's, and a unit of experience 4 half as often again. The loader and `World::new` keep the strength from 0 to 0.5, so the factor is never below 0. One strength serves both sides. At the frozen strength of 0 it does nothing. Nothing else reads experience. [0097 item 2]

### Days of training

A unit set to train (above, The day's activity) has a day of training on a day when all of these hold: its activity is train, it did not march, its hex was not contested, all its men are ashore (it has none aboard), its hex holds a base of its side, and its fed share is 1. The fed share is the one the night's charge set: a unit that landed after the charge keeps the share it had. A unit with its fed share below 1, or at no base, or with men aboard, trains nothing, and nothing is recorded. [0097 items 3 and 4]

The day is counted in the day's activity. `TrainingDay` records the unit, the days counted after the day and the experience, citing the unit's activity order and the day's `MenFed` event for its hex. Its breakdown has the factors `days_before` and `training_days_per_level`. When the days counted reach `ground.toml: training_days_per_level`, the experience rises by one and the count starts again from 0. [0097 item 3]

**The ceiling.** Training cannot take a unit above `ground.toml: training_ceiling`, a whole number from 0 to 4 (2, the standard, with the placeholder figures). A unit at or above it spends the day and counts nothing, and no event is recorded. Training makes raw troops into sound ones and no more. [0097 item 3]

### Days of fighting

A unit fired on a day if its shots in a fight were above 0, whether it attacked or defended. Each such day adds one to its days of fighting, up to the step at `ground.toml: fight_days_per_level`, when the experience rises by one and the count starts again. At experience 4 nothing is counted. A unit with no man left ashore or aboard is not counted. The days are counted in the day's activity, after each unit's rehearsal or training, for every unit that fired. `FightDayCounted` records the unit, the days counted after the day and the experience, citing the unit's `GroundFire` event. Its breakdown has the factors `days_before` and `fight_days_per_level`. [0097 item 5]

Days of fighting are counted with the frozen figures too, so a frozen run of a scenario with a fight ashore records `FightDayCounted` events, and a unit may reach experience 3. At the frozen strength of 0 nothing that happens changes. [0097 item 5]

`ExperienceRaised` records the unit, the new experience and the cause, training or fighting, citing the `TrainingDay` or `FightDayCounted` event of the day. A day of training does not count as a day of rehearsal, and the reverse. [0097 items 3 to 5]

### A unit divided, and losses

A unit made by division takes the experience of the unit it came from, and nothing else of that unit's (above, A unit divided). Nothing adds men to a unit in play, so no men join and dilute it. A unit that loses men keeps its experience: it is not lost with losses. [0097 item 6, What it leaves out]

### What a side sees

A side sees its own units' experience, days of training and days of fighting exactly, and `TrainingDay`, `FightDayCounted` and `ExperienceRaised`, whole. The enemy sees none of it. An estimate of the enemy ashore is of men and says nothing of their quality. A side's own `GroundFire` carries `experience` and `experience_factor` and not the side's chance (above, The fight ashore). [0097 item 7]

## What a land unit is good at

### The skills

A land unit has five skills, each a whole number from 0 to 4, 2 being the standard, on the scale experience has: **assault** (from the sea), **holding** (ground), **weight**, **movement**, which is one number for each of the four kinds of terrain the map has (`clear`, `jungle`, `mountain` and `atoll`), and **works**, eight numbers in all. The works skill is the fifth, added by decision 0099. The list is closed: a sixth skill is a record of its own. A scenario gives a unit its skills and any it does not give is 2. A unit with every skill at 2 is the unit as it was before the rule. A skill never changes in play: no rule raises or lowers one, and training and fighting raise experience only (above, Experience and training). A unit made by division takes the skills of the unit it left, with its experience (above, A unit divided). [0098 items 1, 2, 6 and 7]

### The one formula

A skill's factor is `1 + strength x (skill - 2)`. The strength is a figure of `ground.toml`, one for each of four skills (movement has one for all its terrains), the same for both sides. Each is at least 0 and below 0.5, so that a skill of 0 never makes a factor of 0: a march that never ends, a unit that eats nothing. A skill of 2, or a strength of 0, makes the factor exactly 1, and then the rule it acts on is as it was. A factor is fixed-point and nothing is cut in the middle. [0098 item 3]

| Skill | The strength | The rule it acts on |
| --- | --- | --- |
| Assault | `assault_skill_strength` | The rehearsal a unit gains for a day |
| Movement | `movement_skill_strength` | What a day's march counts as |
| Holding | `holding_skill_strength` | The chance a unit's shot hits, while its side defends |
| Weight | `weight_strength` | The unit's shots, and the supply for each of its men |
| Works | None: the skill over 2 | The work a unit makes for a day (Units that work, above) |

### Assault: the rehearsal a day gives

A unit's rehearsal for a day (Preparing a landing, above) is `preparation_per_day` times its assault factor, up to 100. With the placeholder figures a unit of assault skill 4 gains 7.5 a day against the standard 5, and one of skill 0 gains 2.5. The breakdown of `RehearsalGained` names `assault_skill` and `assault_factor` only where the factor is not 1. [0098 item 2]

### Movement: what a day's march counts as

A day's march counts toward the days a march takes as the unit's movement factor, not always as one. The factor is of the **lower** of the unit's two skills: its movement skill for the terrain of the hex it leaves and for the terrain of the hex it enters. So mountain troops are fast from one mountain hex to the next, ordinary where either hex is clear, and nobody is fast through a hex he is not trained for. A hex the map gives no terrain counts as jungle: the generated map marks every land hex jungle and gives a coast hex no terrain (decision 0052, item 14), so on the real map only the jungle skill can act. [0098 item 4]

A march holds the days marched, which are one for each day and which fatigue counts (Fatigue, above), and the days counted, the sum of the day's factors. A day adds one to the first and the factor to the second, and the unit arrives when the days counted reach `march_days_per_hex` (Marching overland, above): with the placeholder `march_days_per_hex` of 3, a unit of factor 1.5 takes two days where a standard one takes three, and one of factor 0.5 takes six. A day on which a unit marched counts as a march day for tiredness whatever its factor. The days counted are from 0 up to below `march_days_per_hex`, kept for the next day with the days marched, and kept by a march order to the hex already marched to. `MarchDay` carries the days counted beside the days marched; its text adds "counted as" only where they differ. Its breakdown, recorded only where the factor is not 1, names the skill for the terrain of the hex left, that for the hex entered if the terrain differs, `movement_skill` (the lower), `movement_skill_strength`, `movement_factor` and `counted_before`, and its result is the days counted after the day. [0098 item 4]

### Holding: the chance while the side defends

A unit's chance to hit (Shots and chance, above) is the side's chance times its experience factor times its holding factor, cut to 0 to 1. The holding factor is the factor of the unit's holding skill while its side defends, which is to say when none of its units in the hex is on attack; while the side attacks it is 1. The shots are not cut to whole numbers, as before. The breakdown of `GroundFire` names `holding_skill` and `holding_factor` only where the factor is not 1, with `side_chance` beside them, and the strength of experience as it did. [0098 item 2]

### Weight: shots, and each man's daily charge

Weight cuts both ways. A heavy unit fires more for its men and eats more for its men, by the same factor, the factor of its weight skill. It is the better unit where supply can reach it and the worse one where it cannot. [0098 item 5]

- **Shots.** A unit's shots are its firing men over `men_per_shot` times its weight factor. The breakdown of `GroundFire` names `weight` and `weight_factor`, before `shots`, only where the factor is not 1.
- **The daily charge.** A unit's men cost the daily charge times its weight factor each. The charge of a hex (The men's supply, above) is the sum, for each unit of the side ashore there, of its men times `daily_supply_t_per_man` times its factor. The men fed are the tons paid by all the bases together times the men ashore there over that charge, cut to a whole man, never more than the men; for units of the standard weight this is exactly the tons over the charge for a man, as it was. The fed share is common to the side's units in the hex, as before. The unfed loss is by men. Where any unit's weight is not the standard, the breakdowns of the payments name `weighted_men` (the men counted by their factors).
- **The supply a march carries.** A marching unit carries `march_supply_days` of its own daily charge, weight included: `march_supply_days` times `daily_supply_t_per_man` times its weight factor times its men ashore. The breakdowns of the supply taken and of `MarchArrived` name `weight` and `weight_factor` where the factor is not 1.

### The scenario gives the skills

A `[[land_unit]]` entry may give `skills`, an inline table with any of `assault`, `holding`, `weight`, `clear`, `jungle`, `mountain`, `atoll` and `works`, each a whole number from 0 to 4; any not given is 2. An empty table is allowed. The loader refuses a skill that is not a whole number from 0 to 4 and a name that is not one of the eight, saying which. The invariants refuse a skill above 4 in play (chapter 1, Invariants). [0098 item 1]

### The figures

Four figures in `ground.toml`, each a whole number or a decimal string, from 0 up to but not including 0.5, and required. The live values are placeholders that rest on no source. In the frozen step 1 figures every strength is 0, so no skill does anything and the hand calculations of the earlier stages stand.

| Key | Live | Frozen | What it is |
| --- | --- | --- | --- |
| `assault_skill_strength` | `0.25` | `0` | How much a step of the assault skill moves the rehearsal gained in a day |
| `movement_skill_strength` | `0.25` | `0` | How much a step of a movement skill moves a day's march |
| `holding_skill_strength` | `0.15` | `0` | How much a step of the holding skill moves a defender's chance to hit |
| `weight_strength` | `0.25` | `0` | How much a step of weight moves a unit's shots and the supply for each of its men |

At 0.25 a unit of assault skill 4 rehearses a landing in 14 days where a standard unit takes 20, and a unit of skill 0 takes 40. [0098]

### Real units

No real unit has a value yet. A unit's skills are facts about it, with rows and a written rule, as a ship's figures are, and until that research is done every real land unit is 2 in every skill. The tuning rule holds: a unit's value is never changed to improve a report, and only the strengths are tuned. [0098 Real units]

### What a side sees

A side sees its own units' skills exactly, in the unit's line (the three skills, the four movement skills and works), with the days counted beside the days marched of its march, and `counted` in a `MarchDay` event. The enemy sees none of them: there is no line for an enemy unit, and an estimate of the enemy ashore is of men (What a side knows of the enemy ashore, below). The agent's view gives a side two answers of its own units: the factor a day's march of one of them to a neighbouring hex would count as, and the supply its men cost a day, weight included; the briefing's daily supply by hex counts each unit's weight. The replay file carries a unit's skills, left out when all are 2, and `counted`. [0098 item 8]

## Men lost to fire

A bomb or a shell that hits men ashore removes men. This is the share for men of the table of where a hit on a base lands (chapter 5, The attack on a base; chapter 4, Shore bombardment). [0092]

**When a hit lands on men.** The table's share for men, `air_combat.toml: [base_hit] men` (`0.2`), counts only if the ground figures are loaded with `men_lost_per_base_hit` above 0 and the base's side has men ashore in the base's hex when the attack begins. When it does not, the share lands on nothing, so a raid on a base with no men ashore gives the hits it gave before. [0092]

**What a hit does.** Each hit on men removes `ground.toml: men_lost_per_base_hit` men (`10`), as many as are there, from the base's side's units with men ashore in the base's hex. The men are taken in proportion by the rule of Taking men in proportion. A raid or a bombardment may therefore remove all the men in the hex, and no more. [0092]

**The events.** Each unit that loses men records `MenLostToFire`: the unit, the base, the hex, the men lost and the men left ashore. It cites the raid's `BaseAttacked` or the force's `ShoreBombardment`. Its breakdown has the factors `hits_on_men`, `men_lost_per_base_hit`, `loss_in_hex`, `unit_men_ashore` and `men_ashore_in_hex`, and its result is the men the unit lost. A unit left with no man is destroyed (above, Destroyed). [0092]

**Who sees it.** The unit's own side sees it whole. The raider is told nothing of men hit: the side that bombed or shelled is given neither the hits on men nor the men lost. [0092]

**What counts.** Men lost to fire count in the `men_lost` objective of chapter 6, as men lost by any other cause do, and in the batch record's `men_lost` tally under the cause `fire`. Men who surrendered (A beaten side's men, above) count in the objective and in no cause of the batch record's tally, which has no cause for them. [0092, 0104]

A side sees: `MenLostToFire`, whole, for its own unit.

## What a side knows of the enemy ashore

A side holds an estimate of the enemy's men ashore, one for each hex, written only by a ground report (chapter 3, The ground report and the estimate). This section says when the reports are made and what they tell. [0092]

**When.** In the ground combat phase, at the end of each hex's part, after the day's fight, its losses, fatigue and any base that changed side (above). A report is made for a hex if both sides still have men ashore there, whether or not there was a fight. The Allied side's report comes first. Each side is given a report of the other's men: the true count times an error, between `ground.toml: report_error_low` (`0.5`) and `report_error_high` (`1.5`). A report is made only if both figures are given. [0092]

**What it does.** It makes or replaces the side's estimate for the hex: the count, the day and the report's event. It puts on the side's map every enemy base in the hex that is not on it, a base formed by a landing and a base a scenario holds off the map alike, and so a force may then bombard it and a base strike may name it. [0092]

**What a side can judge.** An estimate is not a count: it may be half the truth or one and a half times it, and it is never corrected until the next report replaces it. A side that watches a hex for some days sees roughly where the truth lies, because each day's error is new. A side with no men in a hex gets no report of it, and aircraft report nothing of men. A scenario may give an opening estimate (chapter 3, The ground report and the estimate). [0092]

A side sees: `GroundReportReceived` and `BaseFound`, whole, for its own reports. Its estimates are in its view, by hex, and the true count and the error are not. It never sees the enemy's estimate of its own men.

## What a side knows and sees

A side knows its own land units exactly: the men ashore and where, the men aboard and in which ships, each unit's posture, fatigue, fed share, march, activity, experience, days of training and of fighting, rehearsal and place on a roster, its skills, and its forces' preparation. Its view lists them and its ships show the men aboard. The enemy's land units are not in the view at all. A side knows of the enemy's men ashore only by an estimate (next section). A base the enemy formed by a landing or a march, or that a scenario holds off the map, is not among the enemy's bases the view lists, until a ground report finds it or it is taken: a base that changes side is on both sides' maps (above). [0066 items 21 and 22, 0090 items 18 and 29, 0092, 0094 item 11, 0095 item 14, 0097 item 7, 0098 item 8]

Every event of this chapter is seen whole by its own side and not at all by the enemy, except those the sections above cut: men embarked and landed, a base formed, a base formed by a march, a unit divided, a march day, a march arrived, a march stopped, a unit fell back, a spent attacker fell back, a unit surrendered, men lost with a ship, men fed, men lost unfed, a unit destroyed, a unit's loading complete, a fed share restored, fatigue, a unit back to defend, men lost in a fight, men lost to fire, an air group lost with a base and its strike mission cleared, a ship lost with a base and a home port lost; and, for a force or unit of the side's own, loading gained and lost, rehearsal gained and lost, a unit left a roster, preparation shared, a day of training, a day of fighting counted, experience raised, a day's work, and an airfield built for the side's own base, an airfield mended by the side's own unit and an airfield of the side's own base damaged. The fight itself and a unit's fire are cut, a failed attack is told to the side that failed, cut to the hex, and a base changing side is told to both sides. The test of a side beaten is told to neither. A falling back (after a defeat or of a spent attacker) and a unit's surrender are the side's own and the enemy is told neither; of a surrender it is told the hex and the men, once for the hex. A ground report and a base found are seen whole by the side that was given them and by no other. A side's number for its own land units, and the leak that a gap in its own numbers can show, are in chapter 1, What a side may know. [0066 item 23, 0090 items 28 to 30, 0092, 0094 item 11, 0095 item 14, 0097 item 7]

A side sees: as each section says. The briefing, the list and detail queries and the events show a side its own units, the days of supply the bases of the side in a hex with men ashore hold, the stock of every such base together over the present charge for the aircraft at them and the men, as the men are fed (above, The men's supply), and whether its men were fed last night. A unit's line shows its skills, its activity, its experience, its days of training (the days counted, the days a step takes and the ceiling) and of fighting, and, when they apply, its rehearsal and its place on a roster with the fatigue it would land with. A force's line shows its preparation (the hex, the roster, the loading and the rate it would land at) and `landing_at`. The enemy is shown none of it.

## Data keys

| Where | Key | What it is |
| --- | --- | --- |
| Scenario file | `ground` | The path of the ground figures file. Required if the scenario has a land unit |
| Scenario file | `[[land_unit]]` | `id`, `name`, `side`, `men`, and either `q` and `r` or `task_force` (above, The scenario gives the units) |
| Scenario file | `[[order]]` of kind `load` | `unit` (a land unit's number) and `cargo`, which with a unit may be `"none"`; without a unit the cargo is supply or fuel |
| Ship classes file | `troop_capacity_men` | Men a ship of the class carries. Optional column, after `anti_aircraft`, 0 where not given |
| `ground.toml` | `handling_t_per_man` | The tons of a ship's work in a pulse that moving one man takes. Above 0 |
| `ground.toml` | `daily_supply_t_per_man` | Supply a man ashore costs his base a day, in tons. Above 0 |
| `ground.toml` | `unfed_loss_share_per_day` | The share of unfed men lost in a day. From 0 to 1 |
| `ground.toml` | `men_per_shot` | Firing men to one shot. Above 0 |
| `ground.toml` | `hit_chance` | The chance a shot hits, for a side that attacks. From 0 to 1 |
| `ground.toml` | `defender_hit_chance` | The same, for a side that defends. From 0 to 1 |
| `ground.toml` | `men_per_hit` | Men a hit removes. A whole number, at least 1 |
| `ground.toml` | `fatigue_per_attack_day` | Fatigue a unit gains on a day it fired on attack. From 0 to 100 |
| `ground.toml` | `rest_per_day` | Fatigue a unit ashore loses on any other day. From 0 to 100 |
| `ground.toml` | `attack_fatigue_limit` | Above this a unit on attack goes back to defend. From 0 to 100 |
| `ground.toml` | `beaten_share` | A side attacked is beaten below this share of the attacker's men. From 0 to below 1 |
| `ground.toml` | `captured_stock_share` | The share of a base's supply and fuel its taker keeps. From 0 to 1 |
| `ground.toml` | `beaten_fatigue` | Fatigue a unit gains when its side is beaten and it falls back. From 0 to 100. Required (decision 0104) |
| `ground.toml` | `engagement_ratio` | The most firing men a side counts, as a multiple of the other side's fighting men ashore in the hex. 1 or more. Required (decision 0105) |
| `ground.toml` | `failed_attack_fatigue` | Fatigue a unit on attack that fired gains on a day its side's attack failed. From 0 to 100. Required (decision 0105) |
| `ground.toml` | `men_lost_per_base_hit` | Men a bomb or shell hit on men removes. A whole number, 0 or more. Missing gives 0 (decision 0092) |
| `ground.toml` | `report_error_low`, `report_error_high` | The least and the most a ground report can be, as a share of the truth, `0.5` and `1.5`. Both or neither: with neither no ground report is made, and one alone is refused. The low is 0 or more and not above the high (decision 0092) |
| `air_combat.toml` | `[base_hit] men` | The share of hits on a base that land on men, `0.2`. Missing gives 0 (decision 0092) |
| `air_combat.toml` | `[base_hit] airfield` | The share of hits on a base that land on its airfield, `0.2` (`0` in the frozen copy). Missing gives 0 (decision 0103) |
| Scenario file | `[[base]]` `on_map` | `true` by default. `false` puts the base off the enemy's map at the start (decision 0092) |
| Scenario file | `[[estimate]]` | `side`, `q`, `r` and `men`: an opening estimate (chapter 3, The ground report and the estimate) |
| Scenario file | `[[land_unit]]` `posture`, `fatigue` | The unit's posture at the start (`defend` by default or `attack`) and its fatigue (0 to 100, 0 by default) |
| Scenario file | `[[land_unit]]` `experience`, `training_days`, `fight_days` | The unit's experience (a whole number from 0 to 4, 2 by default) and the days of training and of fighting already counted (whole numbers, 0 by default, below `training_days_per_level` and `fight_days_per_level`) |
| Scenario file | `[[order]]` of kind `set_posture` | `land_unit` (a land unit's number) and `posture` (`attack` or `defend`), and, with `attack` only, an optional `when_spent` (`hold` or `fall_back`; decision 0105) |
| Scenario file | `[[order]]` of kind `march` | `land_unit` (a land unit's number) and `hex` (`[q, r]`) |
| Scenario file | `[[order]]` of kind `prepare_landing` | `task_force` (a force's number), `hex` (`[q, r]`) and `roster` (land units' numbers; may be empty) |
| Scenario file | `[[order]]` of kind `set_activity` | `land_unit` (a land unit's number) and `activity` (`ready`, `train` or `work`) and an optional `job` |
| `ground.toml` | `march_days_per_hex` | Days a march from a hex to a neighbour takes. A whole number, 1 or more. Required |
| `ground.toml` | `fatigue_per_march_day` | Fatigue a unit gains on a day it marches. From 0 to 100. Required |
| `ground.toml` | `march_supply_days` | Days of the daily charge for its men that a unit takes with it when it arrives. 0 or more. Required |
| `ground.toml` | `preparation_per_day` | Loading a force and rehearsal a unit of its roster gain in a day. From 0 to 100. Required |
| `ground.toml` | `unprepared_landing_share` | The share of its rate a force with no loading works at while it lands men. From 0 to 1. Required |
| `ground.toml` | `preparation_kept_on_change` | The share of its loading a force keeps when a unit is added to its roster. From 0 to 1. Required |
| `ground.toml` | `unprepared_landing_fatigue` | Fatigue a unit gains on landing with no rehearsal. From 0 to 100. Required |
| `ground.toml` | `experience_fire_strength` | How much a step of experience moves the chance a shot hits. From 0 to 0.5. Required |
| `ground.toml` | `training_days_per_level` | Days of training for one step of experience. A whole number, 1 or more. Required |
| `ground.toml` | `training_ceiling` | The experience training cannot pass. A whole number from 0 to 4. Required |
| `ground.toml` | `fight_days_per_level` | Days of firing in a fight for one step of experience. A whole number, 1 or more. Required |
| `ground.toml` | `assault_skill_strength`, `movement_skill_strength`, `holding_skill_strength`, `weight_strength` | How much a step of each skill moves what it acts on (What a land unit is good at, above). Each from 0 to below 0.5. Required (decision 0098) |
| Scenario file | `[[land_unit]]` `skills` | An inline table `{ assault, holding, weight, clear, jungle, mountain, atoll, works }`, each optional, a whole number from 0 to 4, 2 by default (decisions 0098, 0099) |
| Scenario file | `[[land_unit]]` `working` | `true` marks a working unit (`false` by default). A working unit may not have the posture attack (decision 0099) |
| Scenario file | `[[base]]` `airfield`, `airfield_work`, `defences` | `airfield` is `true` by default; `false` gives a base with none. `airfield_work` is the work already done on it, 0 or more, only with `airfield = false`. `defences` is from 0 to 100, 0 by default (decision 0099) |
| Scenario file | `[[base]]` `airfield_damage` | The damage to the airfield at the start, from 0 to 100, 0 by default, only for a base with an airfield (decision 0103) |
| Scenario file | `[[order]]` of kind `set_activity`, `activity` `work` | An optional `job` (`airfield`, `port`, `aircraft` or `defences`), only with `work` (decision 0099) |
| `ground.toml` | `work_per_man_day`, `airfield_work`, `work_to_double_unloading`, `work_per_aircraft_repair`, `work_per_defence_point`, `defences_strength`, `fighting_unit_work_share` | The seven figures of work and what it builds (Units that work, above). Required (decision 0099) |
| `ground.toml` | `airfield_damage_per_hit`, `work_per_airfield_damage_point`, `airfield_landing_refused_at` | The three figures of an airfield damaged and mended (The airfield, above). Required (decision 0103) |
| Scenario file | `land_links`, map file `[[land_link]]` | The land links of the map (chapter 2, Land links) |
| Scenario file | objective of kind `base_held` | `base` and `points` (chapter 6) |
| Scenario file | objective of kind `men_lost` | `points_per_100_men` (chapter 6) |

The ground file is a TOML file with `placeholder = true` and the forty keys (the twelve, the three of decision 0094, the eight of decisions 0095 and 0097, the four of decision 0098, the seven of decision 0099, the three of decision 0103, the one of decision 0104 and the two of decision 0105), each a whole number or a decimal string, except `march_days_per_hex`, `training_days_per_level`, `training_ceiling` and `fight_days_per_level`, which are a whole number only (a decimal string for one of them is refused), and may hold the three of decision 0092. A TOML float, an unknown key and a missing one of the forty are refused. The report error figures need the search figures: a world with them and no search figures is refused when it is set up, because belief is written with the search figures. `World::new` and the loader refuse a figure outside the ranges above. The frozen step 1 copy (`data/reference/step1/ground.toml`) holds the same first values. The frozen copy switches the preparation and experience rules off by its first values: `unprepared_landing_share` 1, `unprepared_landing_fatigue` 0 and `experience_fire_strength` 0, and the live file has 0.5, 40 and 0.25; the others are the same in both (`preparation_per_day` 5, `preparation_kept_on_change` 0.5, `training_days_per_level` 30, `training_ceiling` 2, `fight_days_per_level` 10). The frozen copy also switches the skills off: its four strengths are 0, and the live file has 0.25, 0.25, 0.15 and 0.25. The frozen copy also switches the limit on engaged men and the fatigue of a failed day off: its `engagement_ratio` is a figure no fight reaches and its `failed_attack_fatigue` is 0, so with them a fight is as it was before decision 0105 (the `GroundAttackFailed` event is still recorded). The seven figures of decision 0099, the three of decision 0103 and `beaten_fatigue` are the same in both. Every figure is a placeholder that rests on no source. A `unprepared_landing_share` of 0 is within its range, and with no loading a landing then never completes: a figure to rule on when it is tuned. [0066, 0090, 0092, 0094, 0095, 0097, 0098, 0103, 0104, 0105, 0057]

## Limits

- **A side knows the enemy's men ashore only by an estimate.** It is made daily, where both sides have men ashore, and may be wrong by up to half either way. A side with no men in a hex gets no report of it, and there are no patrols or scouts ashore. [0092]
- **A bombardment can close a field, by hits and not by a rule of its own.** It destroys aircraft, supply and men, and a share of its hits lands on the airfield and damages it (The airfield, above). It does not damage the port. [0092, 0103]
- **Engagement is one limit and nothing else.** There is no frontage, no terrain that narrows a front, and no choice of which units are engaged: all of a side's units fire a share and all take losses in proportion. There is no morale or disorder beyond fatigue, no counter-attack by a defender when an attack fails, and no pursuit: the victor fires on men who have not yet left only if it is ordered to attack. The engine does not judge whether reserves or support are coming; the attack order's instruction says it. [0105]
- **Falling back is by land only, to the one hex the rule gives.** A unit that falls back cannot choose where to, beyond a march order the next day; it is not taken off by sea, which only an order does, and a beaten side that means to take its men off by sea must do it before it is beaten. A unit with no hex surrenders: prisoners are men lost and nothing more, with no prisoners held, exchanged or fed. A unit falling back has no more disorder than the fatigue of defeat. A unit already marching when its side is beaten, or when its attack is spent, keeps its march and gains nothing. A spent attacker that falls back does it by the same rule and does not surrender if no hex is given: it stays. [0090, 0094, 0104, 0105]
- **Guns ashore do not fire on ships,** and artillery ashore does not close an airfield: only bombs and shells from outside damage it. [0090, 0103]
- **No terrain in the fight and no kinds of unit.** A man is a man, and the defender's advantage is one figure, raised by the defences of its base; terrain changes only how fast a skilled unit marches, and a unit's skills are numbers and not a kind. [0090, 0098, 0099]
- **A base is not damaged by the fight for it.** It is taken whole, but for its stocks. [0090]
- **Flying out on the day a base falls is not possible.** The fight is after the last pulse and a rebase order is given at the start of a day, so a group must be ordered out a day ahead; one still there is lost whole. [0090, 0091]
- **Men aboard do not eat,** however long the passage. [0066]
- **No ration scale.** Men are fed or unfed. [0066]
- **A side's hold on its own supply.** Men and aircraft draw on one stock, aircraft first, and no order changes that. [0066, 0050]
- **A march is one hex.** There is no road, and terrain changes the days only for a unit whose movement skill is not the standard (decision 0098): for a standard unit every land link takes the same days. A route of more than one hex is ordered hex by hex. There is no pursuit and no blocking of a march, and a part of a unit cannot march. Men lose nothing on the march but what the unfed rule takes. [0094]
- **Men saved from a sinking ship,** and men carried in ships not in a task force, are not built. [0066]
- **Loading and landing are one rate.** Men go over a beach and over a berth at the rate of the port, by the one weight of a man; a landing force's rate is then scaled by its loading. [0066, 0095]
- **No scenario key sets a force's loading or a unit's rehearsal** at the start: both start at 0. [0095]
- **Nothing joins forces or adds a ship to a force yet.** The rules for a join are written and no order reaches them: a join order is a stage of step 3. [0095]
- **Works are built in their smallest form.** Damage to a port and so its repair, a port's berths and guns, a base for ship repair, materials, working men pressed into a fight, defences worn down by shells, bombs or the fight, and more than one airfield and its size are left out. Bombs and shells destroy aircraft, supply and men and damage the airfield, and leave the port and the defences whole. An airfield's damage is one number: there are no craters that slow the building of a field not yet built, no aircraft wrecked by landing on a damaged field below the level that refuses them, and the enemy learns nothing of the state of a field. No real unit has a works skill yet: until research gives one, every real unit is 2. [0099, 0103]
- **A skill at fighting in a terrain is not built,** as against moving through it: terrain does not enter the fight. Weapons, vehicles, artillery and engineers as things, skills for air groups and ships, and the retraining of a unit are left out too: weight stands for the heavy things, and a unit's skills never change. [0098]
- **Experience is not lost with losses.** A unit cut to a tenth of its men keeps its experience and its counts, and nothing in play adds men to a unit or lowers its experience. No real unit differs from the standard until research gives each one a value. Kinds of training, the quality of replacements and experience for air groups are not built. [0097]
- **Preparation is for a landing and nothing else.** Loading and rehearsal are spent by the landing and do not last; they are for one hex, and a bombardment, a strike or a convoy has none. [0095]
- **No arrivals or replacements of men** by schedule. A scenario's units are those it starts with. [0066]
- **A new number can show what the enemy did.** A base formed by a landing or a march and a unit made by division take the next free number counted over both sides, so a gap in the numbers of a side's own bases can show that the enemy formed one. Decision 0047 records the same for a task force split off and 0059 for an air group formed. [0066, 0047, 0059]

## Not in this chapter

- The cargo rule, berths and lighters, and the rates of a port: chapter 2, Cargo.
- Supply stocks, the daily supply for aircraft and a base out of supply: chapter 5.
- The land links a march follows and a beaten side's men fall back along: chapter 2. Orders in general, and what a side may know and see: chapter 1.
- The experience of a ship's crew: chapter 4 (decision 0064). The cargo rule a landing force's rate scales, and the port factor of a day's work: chapter 2. Aircraft repair, rebasing, landing elsewhere, arrivals and the aircraft a damaged airfield lets fly, which read a base's airfield, its damage and work: chapter 5.
- The ground report and the estimate as belief, and the opening estimate: chapter 3. Shore bombardment: chapter 4. The share of a hit that lands on men and the share that lands on the airfield: chapter 5.
- Objectives on a base held and on men lost (which counts men who surrendered): chapter 6. Aircraft lost with a base and a group's refusal of orders: chapter 5.
