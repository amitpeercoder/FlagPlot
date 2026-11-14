# Land units

This chapter covers men: the land unit, the troop capacity of a ship, the load order that takes a unit aboard, the landing that puts it ashore and forms a base for it, a unit divided, men lost with a ship, the men's daily supply and the loss of the unfed, the posture of a unit and its order, the day's fight ashore, fatigue, a side beaten in a hex, a base that changes side, men lost to the fire of bombs and shells, and what a side knows of the enemy's men ashore.

Men are worked through the cargo rule of chapter 2 and fed through the base supply rule of chapter 5. This chapter gives only what is new in them. Every quantity is a fixed-point number except men, which are whole numbers. "Cut" means cut toward zero (chapter 1, Numbers and chance).

Where these rules run in a day (chapter 1, The order in which a day resolves, gives the whole of it):

| When | Rule |
| --- | --- |
| Start of the day, with the other orders | A load order with a unit and a posture order are checked and applied (chapter 1, Orders) |
| Night pulse, after the daily supply for aircraft | The men's supply: feeding, the unfed and the loss |
| Loading phase, last in each pulse | Men are taken aboard and landed with the cargo; units are divided and bases formed |
| Whenever a ship sinks | The men aboard are lost with her |
| Night pulse, in the shore bombardment phase, and in the air combat phase of a daylight pulse | Bombs and shells that hit men ashore: men lost to fire (below) |
| After the third pulse, before the day's base count | The ground combat phase: for each hex with men ashore, the fight, fatigue, a side beaten, a base that changes side and the ground reports |

A scenario that names no ground figures has no land units, and every rule here does nothing in it (chapter 1, Where figures come from and when a rule is loaded). [0066, 0090]

## The land unit

### What a land unit is

A land unit is a count of men with a side and a name. It has a number, which is stable and never reused. Its men are ashore in one hex, aboard ships, or part each. The men ashore of one unit are all in one hex. The men aboard a ship are of one unit only, and a unit may have men aboard several ships. A unit is destroyed when it has no men ashore and none aboard any ship (below, Destroyed). It stays in the state with no men and keeps its number. [0066 items 1 and 2]

A land unit's state is: its number, its name, its side, the men ashore and the hex they are in, its posture, the order that set its posture, its fatigue and its fed share. The hex is set exactly when the men ashore are above 0. The posture is defend or attack, defend at the start unless the scenario says otherwise. The fatigue is from 0 to 100 and starts at 0 unless the scenario says otherwise. The fed share is from 0 to 1 and starts at 1. A ship's state gains the men aboard and the unit they belong to (`troops`), at least one man when set. The state hash holds the men, the hexes, the posture, its order, the fatigue and the fed share and not the name (chapter 1, The state hash). [0066, 0090 items 1, 8 and 11]

A side sees: its own units exactly (below, What a side knows and sees).

### The scenario gives the units

A scenario's `[[land_unit]]` entry gives `id`, `name`, `side` and `men`, and either a hex (`q` and `r`) where the men are ashore, or a `task_force` where they are aboard. It may give a `posture`, `"defend"` (the default) or `"attack"`, and a `fatigue`, a whole number or a decimal string from 0 to 100 (0 if not given). The scenario's `ground` key names the ground figures file. The loader refuses a scenario that:

- has a land unit and names no `ground` file;
- gives a unit no men, a number of 0 or of 1,000,000 and above (those are kept for units made in play), or a number twice;
- gives a unit both a hex and a task force, or neither, or only one of `q` and `r`;
- puts a unit ashore off the map or in a hex that is neither coast nor land;
- puts a unit aboard a task force that does not exist or is of the other side;
- puts a unit aboard a force whose ships do not have berths for all its men;
- gives a unit a fatigue below 0 or above 100, or a posture that is neither `defend` nor `attack`.

Units are put aboard in unit order, each over the force's ships in ship order, into a ship that has no men aboard or holds men of this unit, each up to its troop capacity. A unit that is aboard has no men ashore. The game is also refused when it is built (`World::new`) if there are land units and no ground figures, or if the ground figures are wrong (below, Data keys). [0066 items 3 and 4, Settled before the build, 0090 items 1 and 11]

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

The loading phase, last in every pulse, works the cargo of a force that holds a cargo order, is idle or arrived, and is exactly at the centre of a hex (chapter 2, Cargo orders). Besides a force at a base of its own side, two kinds of force work with no base there, if the ground figures are loaded:

- a force with a load order with a unit, which takes men off the shore;
- a force with an unload order, in a coast hex, with men aboard any ship, which lands them.

The hours worked, the rates of the port and the order of forces and ships are those of chapter 2, Working cargo. The port is the port of the hex. [0066 items 6, 7 and 11]

### Berths

Berths are given out before any cargo moves, for each side's landing place in a hex: the lowest-numbered base of the side there, or the one a landing will form. A ship has cargo to move, for the giving of a berth, if it has men to move or tons to move. A load has men to move when the unit has men ashore in the hex and the ship has room for them; an unload has men to move when the hex is a coast hex and the ship carries men. Chapter 2, Berths and lighters, holds the rest. [0066 item 7]

### One weight for a man

A ship's work in a pulse is its rate (berth or lighter) times the hours worked, in tons, as in chapter 2. Each man moved takes `ground.toml: handling_t_per_man` of it. Men move first and in whole men: the most a ship can move is its tons of work over the weight of a man, cut. What is left of the ship's tons moves supply and then fuel, by the rule of chapter 2. One figure serves loading and landing. [0066 item 7]

### Loading men

Ships are taken in ship order. A ship takes the least of the men its tons of work can move, the unit's men still ashore in the hex, and its berths for the unit (its troop capacity less the men aboard, or none if men of another unit are aboard). The unit's men ashore fall by the men taken. A unit with no men left ashore has no hex. Each ship that takes men records `MenEmbarked`: the unit, ship, hex, men, men now aboard the ship and men left ashore. It cites the load order and the force's arrival event. [0066 item 8]

Then the cargo, with the tons left. A load moves the least of the tons left, the ship's room and the stock, by chapter 2. With no base or no cargo named it moves none.

A load order with a unit is complete when no man of the unit is ashore in the hex or no ship has a berth for one, and nothing remains of the named cargo by the rule of chapter 2 (the stock is 0 or every ship is full; with no cargo named, or no base, nothing remains). When a force has moved something this pulse and the order is complete, `UnitLoadingComplete` is recorded in place of `LoadingComplete`: the force, the unit, the base loaded from (none off a bare shore), the cargo (none if none was named) and the hour the slowest ship finished. The hour is the start of work plus the longest of each ship's tons over its rate, counting the tons its men took. It cites the events of the men and the cargo. [0066 Settled before the build]

A side sees: `MenEmbarked` and `UnitLoadingComplete`, whole, for its own unit and force.

### Landing men

An unload order lands men first, then supply, then fuel. Men are landed only on a coast hex. In any other hex they stay aboard, nothing is recorded for them, and the unload counts as complete without them, as cargo does today where there is no base. [0066 items 9 and 10]

A landing happens before any ship of the force moves cargo in the pulse. If any ship of the force can land at least one man in the pulse (its tons of work over the weight of a man, cut, and the men it carries, give at least 1), then, in this order:

1. **Units are divided** (below), in unit order, for each unit that has men to land.
2. **A base is formed** (below) if the side has none in the hex.

Then each ship, in ship order, lands the least of the men its tons of work can move and the men it carries. The men join their unit's men ashore, and the unit's hex is set to the hex. The ship's men aboard fall by the men landed, and a ship with none left carries none. Each ship that lands men records `MenLanded`: the unit, ship, hex, men, men left aboard the ship and men of the unit now ashore. It cites the unload order, the force's arrival event and, in the pulse a base was formed, the `BaseFormed` event, and, for a unit made by division, its `UnitDivided` event. The supply and fuel unloaded in the same phase go into the base, formed or not, with the tons the men left. [0066 items 9 to 11]

An unload order is complete when no ship has supply or fuel aboard and, in a coast hex, no ship has men aboard. Then `UnloadingComplete` is recorded as before, citing the events of the pulse. [0066 item 9]

A side sees: `MenLanded` and `UnloadingComplete`, whole, for its own unit and force.

### A base formed by a landing

When a force lands its first man in a hex where its side has no base, a base of the side is formed there before the men land. It has the next free base number, counted over all the bases of both sides (the highest plus one), the name "Landing at (q,r)" with the hex, the port of the hex, no supply or fuel, no guns, no repairs and no aircraft. It is the side's lowest-numbered base in the hex only if it has no lower. `BaseFormed` records it, with the side, hex and force, citing the unload order and the arrival. A force with no men aboard forms no base, so a supply run to an empty hex changes nothing. A hex that holds a base of the side already, formed or not, forms none. [0066 item 11]

A base formed by a landing is not on the enemy's map until it changes side (below, A base that changes side) or a ground report finds it (below, What a side knows of the enemy ashore). It is on its own side's, in every list and answer a side is given of its own bases, and it works as any other base of its side in a rule that reads a base: it holds the stocks unloaded into it, it feeds the men ashore (below), it can be loaded from and unloaded to, and a base count is recorded for it at the end of each day. The other side is told nothing of it until then: chapter 1, What a side may know, gives the map and the base strike refusal. [0066 item 12, 0090 item 18, 0092]

A side sees: `BaseFormed`, whole, for its own base. The enemy sees nothing.

### A unit divided

A unit is divided when its men would be ashore in two hexes. When a force is about to land men of a unit that has men ashore in a hex other than the force's hex, all the unit's men aboard the force's ships become a new unit first. The new unit has the next free number, counted over all the land units of both sides (the highest plus one), the side of the unit it came from, no men ashore, and the name of that unit with ", detachment" and its own number after it. The men ashore elsewhere keep the unit. It is the new unit that lands. Units are never merged. A unit whose men ashore are in the force's hex, or that has none ashore, is not divided. `UnitDivided` records the unit it came from, the new unit, the force and the men, citing the unload order and the arrival. [0066 item 14, Settled before the build]

A side sees: `UnitDivided`, whole, for its own unit.

## Lost at sea

Men aboard a ship that sinks are lost with her, all of them, at the moment she sinks. `MenLostWithShip` records the unit, the ship and the men, citing the `ShipSunk` event. The ship then carries none. A unit left with no men ashore and none aboard any ship is destroyed (below). The side that sank the ship is not told she carried men. [0066 item 15]

A side sees: `MenLostWithShip`, whole, for its own unit. The enemy sees nothing.

## The men's supply

### Feeding

If the ground figures are loaded, the night pulse has a phase of its own for the men's supply, after the daily supply for aircraft and before air group rest (chapter 1, The night pulse). For each hex with men of a side ashore, hexes in hex order (by q, then by r) and, in a hex, sides in the order of their codes (Allied first), the men are the men ashore of all the side's units in the hex, and the bases are every base of the side in the hex. There is no random number. [0066 items 16 to 18, Settled before the build, 0090 item 24a]

The charge is `ground.toml: daily_supply_t_per_man` times the men. The bases are taken in base order. Each base with a stock above 0, while any of the charge is still owed, is asked for what is still owed and pays as much as its stock covers. The payment comes off the one supply stock that pays for aircraft, after the aircraft's charge in the night, so a stock too short for both feeds the aircraft. Each base that pays records its own `BaseSupplySpent` (reason men): the men, the amount asked, the amount paid and the stock left, with no cause. Its breakdown has the factor `paid_by_earlier_bases_t` only where an earlier base of the side paid part of the charge. A base with no stock, or no base, is asked nothing and pays nothing, and no payment is recorded. A stock the men's charge empties makes the base out of supply from the start of the next pulse (chapter 5, A base out of supply). [0066 items 16 to 18, 0090 item 24a]

The men fed are the tons paid by all the bases together over the charge for a man, cut to a whole man and never more than the men. The rest are unfed. `MenFed` is recorded for every side and hex with men ashore, also where nothing was paid: the side, hex, the base (the first base that paid, or, if none paid, the lowest-numbered base of the side in the hex, or none if the side has none there), the men, the fed and the unfed. A hex with one base of the side reads as it always did. It cites each payment there was. Men aboard ships do not eat. [0066 items 17 and 19, 0090 item 24a]

A side sees: `MenFed` for its own men and `BaseSupplySpent` for its own base, whole. The enemy sees nothing.

### The fed share

The fed share of a unit is the men of its side fed in its hex over the men charged there, set at the same moment by `MenFed`: a fixed-point number, the men fed over the men charged cut at six decimals, the same for every unit of the side ashore in the hex. A side with no base in the hex, or with no stock in it, has a fed share of 0 after that night. A unit with men aboard and none ashore at the charge has its share set back to 1, and `FedShareRestored` is recorded for it. A unit destroyed keeps the share it had. A unit that lands after the charge has the share it had, 1 if it never had a charge, until the next night. The fed share is read by the fight (below, Firing men). [0090 item 8]

A side sees: `MenFed` as above, and `FedShareRestored` and its own units' fed shares, whole. The enemy sees none of them. [0090 item 29]

### The unfed are lost

Where any man is unfed, a share of them is lost: the unfed times `ground.toml: unfed_loss_share_per_day`, cut to a whole man, and one man at least, and never more than the unfed. The loss is taken from the side's units with men ashore in the hex, in proportion (below). [0066 item 19]

### Taking men in proportion

With n men to take from units of m1, m2 and so on, of M men in all, in unit order, unit i gives n times mi over M, cut to a whole man. The men still to take are then taken one from each unit in unit order, round again if need be, from units that still have men. No unit gives more than it has. This rule serves the unfed and the losses of a fight. [0066 item 20, 0090 item 9]

Each unit that gives men records `MenLostUnfed`: the unit, the hex, the men lost and the men it has left ashore, citing the `MenFed` event. A unit with no men left ashore has no hex.

A side sees: `MenLostUnfed`, whole, for its own unit.

### Destroyed

A unit that has lost its last man, ashore or aboard, by the loss at sea, by the unfed, in a fight or to fire, is destroyed. `LandUnitDestroyed` records it, citing the event that took the last men. The unit keeps its number and stays in the state with no men. [0066 item 1]

A side sees: `LandUnitDestroyed`, whole, for its own unit.

## Posture

### The posture and its order

A land unit is on defend or on attack. It starts on defend, or on the posture its scenario entry gives. The order `set_posture` names a land unit and a posture, `defend` or `attack`. It is a standing order, in a slot of its own, and stands until another replaces it. It is applied when it is taken in, at the start of the day: the unit's posture is set and the order is kept as the order that set it, so the day's fight reads it. [0090 items 1 and 2]

It is checked at intake in this order, and the first reason that applies is recorded (chapter 1, Every reason an order is refused):

1. **Not your land unit.** The unit does not exist or is the enemy's. One reason serves both.
2. **Land unit destroyed.** The unit has no men ashore and none aboard any ship.
3. **No men ashore.** The unit has no man ashore (it has men aboard ships only).

An attack order needs no enemy. A unit on attack in a hex where nothing is to be attacked does nothing, and is not tired by it (below, Fatigue); the one case in which it has an effect with no enemy man in the hex is a base with no men (below, A side beaten). [0090 items 2 and 3]

A side sees: its own `OrderAccepted` or `OrderRejected`, with the reason, whole. The enemy sees nothing. [0090 item 29]

## The fight ashore

### The ground combat phase

If the ground figures are loaded, the ground combat phase runs once a day, after the third pulse and before the day's base count (chapter 1, After the pulses). It takes the hexes that hold men ashore at its start, in hex order (by q, then by r), and finishes one hex before the next. In each hex it does, in this order: the day's fight, if the hex is contested; each unit's fatigue and the tired units back to defend; the test of a side beaten; and the bases of a beaten side changing side. Last, the ground reports of the hex (below, What a side knows of the enemy ashore). [0090 items 4, 12, 14, 15 and 17]

### A hex is contested

A hex is contested on a day when both sides have men ashore in it and at least one unit there is on attack. A hex that is not contested has no fight. [0090 item 4]

A side attacks in the fight if any of its units in the hex is on attack. A side that does not attack defends. All of a side's units in the hex fight as one body: the units on defend of a side that attacks fire with the rest, as attackers. If both sides attack, each fires as an attacker. There is no position inside a hex. [0090 item 5]

`GroundFight` records the hex and which sides attack, citing the posture orders of the units on attack. [0090]

### Firing men

A unit's firing men are its men ashore times its fed share, times 100 less its fatigue over 100, cut to a whole man. The second product is one `mul_div`, so nothing is cut in the middle. Men aboard ships do not fight and are not hit. Hunger and tiredness each cut this one number and nothing else. [0090 items 6 and 13]

### Shots and chance

Each unit in the fight fires, a unit of the side on defend too. Its shots are its firing men over `ground.toml: men_per_shot`, not cut: the part of a shot is rolled at that part of the chance, by the rule that surface combat uses (chapter 4, Each shot is rolled). Each shot hits at `ground.toml: hit_chance` if its side attacks, or at `ground.toml: defender_hit_chance` if it defends. Each unit's shots are rolled on a stream keyed on the day, no pulse, the ground combat subsystem, the draw `GroundFire` (code 6) and the unit, so no stream is shared between units and the order of firing does not matter. [0090 item 7, 0055]

`GroundFire` is recorded for every unit in the fight, also a unit with no firing men: the unit, the hex, the firing men, the shots, the chance and the hits. It cites the fight and the `MenFed` event of the day in that hex, if any. Its breakdown has the factors `men_ashore`, `fed_share`, `fatigue_factor` (100 less fatigue over 100), `firing_men`, `men_per_shot`, `shots` and `chance`, and its result is the shots times the chance. [0090 item 7]

### Losses

Both sides fire on the strengths they had at the start of the fight, and the losses are worked out for both before either is taken. A side's loss in the hex is the hits the enemy scored times `ground.toml: men_per_hit` (a whole number), never more than the side's men ashore in the hex. It is taken from the side's units in the hex in proportion (above, Taking men in proportion). A hit takes the same men whoever fires and whoever is hit. [0090 item 9, 0066 item 20]

Each unit that loses men records `MenLostInFight`: the unit, the hex, the men lost and the men it has left ashore. It cites the fire events of the other side's units. Its breakdown has the factors `loss_in_hex`, `unit_men_ashore`, `men_ashore_in_hex` and `in_proportion`. A unit with no men left ashore has no hex, and a unit with no man anywhere is destroyed (above, Destroyed). [0090 items 9 and 10]

A side sees: `GroundFight`, cut to the hex, for a fight its men are in. `GroundFire` for its own unit, cut to the unit, the hex, the firing men and the shots, with the breakdown factors `men_ashore`, `fed_share`, `fatigue_factor`, `firing_men`, `men_per_shot` and `shots`, and no chance or hits. `MenLostInFight` for its own unit, whole. It is not told its hits, the enemy's units, the enemy's men or how many of them there are. The enemy's own half of the fight is the enemy's. [0090 item 28]

### Fatigue

A land unit has fatigue, from 0 to 100, the scale air groups and task forces use. After the fight (or with no fight), each unit that still has a man ashore, in unit order, is changed once:

- a unit on attack that fired, which is a unit on attack in a fight whose shots are above 0, gains `ground.toml: fatigue_per_attack_day`, up to 100;
- every other unit, a unit on attack that did not fire included, loses `ground.toml: rest_per_day`, down to 0.

A unit with no man ashore keeps its fatigue. `LandUnitFatigue` (gained or rested, and the new fatigue) is recorded only when the value changes. It cites the unit's fire event where there was one. Its breakdown has the factors `fatigue_before` and `fatigue_per_attack_day` or `rest_per_day`. Fatigue cuts the men who fire (above, Firing men) and nothing else. [0090 items 11 to 13]

### The tired stop attacking

After the day's change, a unit on attack with men ashore whose fatigue is above `ground.toml: attack_fatigue_limit` goes back to defend, whether or not it fought. Its attack order is spent: the order that set its posture is no longer kept as the standing one, and the unit stays on defend until a new order sets it. `LandUnitBackToDefend` records it with the fatigue, citing the unit's fatigue event (or its fire event if the fatigue did not change). Its breakdown has the factors `fatigue` and `attack_fatigue_limit`. One figure serves every unit, and no order sets the limit. [0090 item 14]

A side sees: `LandUnitFatigue`, `LandUnitBackToDefend` and its own units' postures, fatigue and fed shares, whole. The enemy sees none of it. [0090 item 29]

## A side beaten

After the day's fight a side is beaten in a hex if the other side attacked there that day, and it has no man ashore in the hex or fewer than `ground.toml: beaten_share` times the other side's men ashore. Men are counted after the day's losses. If both sides attacked, each is tested. Both are beaten at once only if neither has a man left, and then neither is (the loader and `World::new` refuse a `beaten_share` of 1 or more, so otherwise one side at most is beaten). [0090 item 15]

A base with no men falls without a fight. In a hex that is not contested, where a side has a unit on attack and the other side has a base and no man ashore, the other side is beaten in the ground combat phase of that day. One man landed on attack takes an empty airfield. [0090 item 16]

`SideBeatenAshore` records the side, the hex, the side's men and the enemy's men, and whether there was a fight. It is recorded whenever a side is beaten, whether or not it holds a base there. It cites the fight, or, with no fight, the posture orders of the units on attack. Its breakdown has the factors `men_ashore`, `enemy_men_ashore` and `beaten_share`, and its result is the share times the enemy's men. [0090 items 15 and 16]

A beaten side loses every base it holds in the hex (below). Its men who are left stay ashore, unfed while it has no base there, with a fed share of 0 after the next night. They can be taken off by sea and they can attack on a later day. [0090 item 17]

A side sees: nothing of `SideBeatenAshore`: it weighs both sides' men, and it is the umpire's. A side learns it was beaten by losing its base (below). [0090 items 28 and 30]

## A base that changes side

When a side is beaten in a hex, each base of that side in the hex, in base order, changes side to the other side. The rules below run for each base, in this order. [0090 items 17 to 23]

1. **The base.** It keeps its number, name, hex, guns and repair figures. Its side changes and it is on both sides' maps from then on, whatever it was before: a base a landing formed is told to the enemy now (chapter 1, What a side may know). Its supply and its fuel are each cut to `ground.toml: captured_stock_share` of what they were, in one product, and the rest is lost. `BaseChangedSide` records the base, the hex, the side it was and the side it is, the supply and fuel that stay and the supply and fuel that were lost. It cites the `SideBeatenAshore` event and what that event cites (the fight, or the posture orders of the units on attack). Its breakdown has the factors `supply_t`, `fuel_t`, `captured_stock_share` and `fuel_kept_t`, and its result is the supply that stays. [0090 items 18, 19 and 30]
2. **Air groups at it are lost.** Each air group located at the base, in group order, is lost whole: its ready and damaged aircraft go to 0, and its pilots go to 0 where they are tracked. It has the new place `Lost`. It keeps its mission. No group is in the air, because the phase runs after the last pulse. `AirGroupLostWithBase` records the group, the base and the aircraft lost, citing `BaseChangedSide`, and `PilotsLost` is recorded if it had pilots, citing it. The aircraft are counted as aircraft destroyed for the enemy, as a group lost with its ship is (chapter 6). Chapter 5, A group lost with its base, gives what follows the group. A group that was flown out by a rebase order on an earlier day is at another base and is not lost (chapter 5, Rebasing an air group). [0090 item 20]
3. **A ship in port at it is lost,** in ship order, and is sunk and scored as sunk. This is a ship in no task force: a force in the hex is at sea and is not touched. `ShipLostWithBase` records the ship and the base, citing `BaseChangedSide`; it is its own event because `ShipSunk` names a task force. The air groups aboard her are lost as with a ship that sinks (chapter 5, A group lost with its ship), and the men aboard her are lost with her (above, Lost at sea), each citing `ShipLostWithBase`. [0090 item 21, 0025]
4. **A home port is lost.** A force of the beaten side whose home port the base was has no home port. `HomePortLost` records the force and the base, citing `BaseChangedSide`. [0090 item 22]
5. **A strike mission is cleared.** An air group of the winning side whose mission was a base strike on the base has the mission rest and no mission order. `StrikeMissionCleared` records the group and the base, citing `BaseChangedSide`. [0090 item 22]

What needs no rule of its own: the beaten side's replacement stream at the base stops (chapter 5, Replacements); a standing cargo order of the beaten side works no cargo there, as at any hex with no base of its side; a force of the beaten side away at the yard from that port is not touched and returns to its hex as before. From the next day the base refuels, repairs and works cargo for its new side, aircraft of the new side that cannot get home may land there, and the men of the new holder in the hex are fed from it. [0090 item 23]

A side sees: `BaseChangedSide`, both sides, at its place in the log: whole to the side that now holds the base, and cut to the base, the hex and the two sides, without the stocks, to the side that lost it. `AirGroupLostWithBase`, `ShipLostWithBase`, `HomePortLost` and `StrikeMissionCleared`, whole, for its own group, ship or force; `PilotsLost` and the events of the men and groups aboard a ship as for any ship lost. The side that took the base is not told what was lost at it. [0090 item 30]

Events of a base are seen by the side that held it when they were recorded. The holder at the time of each event is worked out from the log: the side the base had before its first change of side, handed on at each `BaseChangedSide` event. A base's past, and the numbers a side was given for it, are therefore not handed to its taker or renumbered. [0090 item 24, 0086]

## Men lost to fire

A bomb or a shell that hits men ashore removes men. This is the share for men of the table of where a hit on a base lands (chapter 5, The attack on a base; chapter 4, Shore bombardment). [0092]

**When a hit lands on men.** The table's share for men, `air_combat.toml: [base_hit] men` (`0.2`), counts only if the ground figures are loaded with `men_lost_per_base_hit` above 0 and the base's side has men ashore in the base's hex when the attack begins. When it does not, the share lands on nothing, so a raid on a base with no men ashore gives the hits it gave before. [0092]

**What a hit does.** Each hit on men removes `ground.toml: men_lost_per_base_hit` men (`10`), as many as are there, from the base's side's units with men ashore in the base's hex. The men are taken in proportion by the rule of Taking men in proportion. A raid or a bombardment may therefore remove all the men in the hex, and no more. [0092]

**The events.** Each unit that loses men records `MenLostToFire`: the unit, the base, the hex, the men lost and the men left ashore. It cites the raid's `BaseAttacked` or the force's `ShoreBombardment`. Its breakdown has the factors `hits_on_men`, `men_lost_per_base_hit`, `loss_in_hex`, `unit_men_ashore` and `men_ashore_in_hex`, and its result is the men the unit lost. A unit left with no man is destroyed (above, Destroyed). [0092]

**Who sees it.** The unit's own side sees it whole. The raider is told nothing of men hit: the side that bombed or shelled is given neither the hits on men nor the men lost. [0092]

**What counts.** Men lost to fire count in the `men_lost` objective of chapter 6, as men lost by any other cause do, and in the batch record's `men_lost` tally under the cause `fire`. [0092]

A side sees: `MenLostToFire`, whole, for its own unit.

## What a side knows of the enemy ashore

A side holds an estimate of the enemy's men ashore, one for each hex, written only by a ground report (chapter 3, The ground report and the estimate). This section says when the reports are made and what they tell. [0092]

**When.** In the ground combat phase, at the end of each hex's part, after the day's fight, its losses, fatigue and any base that changed side (above). A report is made for a hex if both sides still have men ashore there, whether or not there was a fight. The Allied side's report comes first. Each side is given a report of the other's men: the true count times an error, between `ground.toml: report_error_low` (`0.5`) and `report_error_high` (`1.5`). A report is made only if both figures are given. [0092]

**What it does.** It makes or replaces the side's estimate for the hex: the count, the day and the report's event. It puts on the side's map every enemy base in the hex that is not on it, a base formed by a landing and a base a scenario holds off the map alike, and so a force may then bombard it and a base strike may name it. [0092]

**What a side can judge.** An estimate is not a count: it may be half the truth or one and a half times it, and it is never corrected until the next report replaces it. A side that watches a hex for some days sees roughly where the truth lies, because each day's error is new. A side with no men in a hex gets no report of it, and aircraft report nothing of men. A scenario may give an opening estimate (chapter 3, The ground report and the estimate). [0092]

A side sees: `GroundReportReceived` and `BaseFound`, whole, for its own reports. Its estimates are in its view, by hex, and the true count and the error are not. It never sees the enemy's estimate of its own men.

## What a side knows and sees

A side knows its own land units exactly: the men ashore and where, the men aboard and in which ships, each unit's posture, fatigue and fed share. Its view lists them and its ships show the men aboard. The enemy's land units are not in the view at all. A side knows of the enemy's men ashore only by an estimate (next section). A base the enemy formed by a landing, or that a scenario holds off the map, is not among the enemy's bases the view lists, until a ground report finds it or it is taken: a base that changes side is on both sides' maps (above). [0066 items 21 and 22, 0090 items 18 and 29, 0092]

Every event of this chapter is seen whole by its own side and not at all by the enemy, except those the sections above cut: men embarked and landed, a base formed, a unit divided, men lost with a ship, men fed, men lost unfed, a unit destroyed, a unit's loading complete, a fed share restored, fatigue, a unit back to defend, men lost in a fight, men lost to fire, an air group lost with a base and its strike mission cleared, a ship lost with a base and a home port lost. The fight itself and a unit's fire are cut, and a base changing side is told to both sides. The test of a side beaten is told to neither. A ground report and a base found are seen whole by the side that was given them and by no other. A side's number for its own land units, and the leak that a gap in its own numbers can show, are in chapter 1, What a side may know. [0066 item 23, 0090 items 28 to 30, 0092]

A side sees: as each section says. The briefing, the list and detail queries and the events show a side its own units, the days of supply its base holds at the present charge for aircraft and men together, and whether its men were fed last night.

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
| `ground.toml` | `men_lost_per_base_hit` | Men a bomb or shell hit on men removes. A whole number, 0 or more. Missing gives 0 (decision 0092) |
| `ground.toml` | `report_error_low`, `report_error_high` | The least and the most a ground report can be, as a share of the truth, `0.5` and `1.5`. Both or neither: with neither no ground report is made, and one alone is refused. The low is 0 or more and not above the high (decision 0092) |
| `air_combat.toml` | `[base_hit] men` | The share of hits on a base that land on men, `0.2`. Missing gives 0 (decision 0092) |
| Scenario file | `[[base]]` `on_map` | `true` by default. `false` puts the base off the enemy's map at the start (decision 0092) |
| Scenario file | `[[estimate]]` | `side`, `q`, `r` and `men`: an opening estimate (chapter 3, The ground report and the estimate) |
| Scenario file | `[[land_unit]]` `posture`, `fatigue` | The unit's posture at the start (`defend` by default or `attack`) and its fatigue (0 to 100, 0 by default) |
| Scenario file | `[[order]]` of kind `set_posture` | `land_unit` (a land unit's number) and `posture` (`attack` or `defend`) |
| Scenario file | objective of kind `base_held` | `base` and `points` (chapter 6) |
| Scenario file | objective of kind `men_lost` | `points_per_100_men` (chapter 6) |

The ground file is a TOML file with `placeholder = true` and the twelve keys, each a whole number or a decimal string, and may hold the three of decision 0092. A TOML float, an unknown key and a missing one of the twelve are refused. The report error figures need the search figures: a world with them and no search figures is refused when it is set up, because belief is written with the search figures. `World::new` and the loader refuse a figure outside the ranges above. The frozen step 1 copy (`data/reference/step1/ground.toml`) holds the same first values. Every figure is a placeholder that rests on no source. [0066, 0090, 0092, 0057]

## Limits

- **A side knows the enemy's men ashore only by an estimate.** It is made daily, where both sides have men ashore, and may be wrong by up to half either way. A side with no men in a hex gets no report of it, and there are no patrols or scouts ashore. [0092]
- **A bombardment does not close a field.** It destroys aircraft, supply and men, and does not damage the runway. [0092]
- **No retreat.** A beaten side's men stay in the hex; no unit moves overland. [0090]
- **Guns ashore do not fire on ships,** and artillery does not close an airfield. [0090]
- **No terrain, no dug-in defence that grows, and no kinds of unit.** A man is a man, and the defender's advantage is one figure. [0090]
- **A base is not damaged by the fight for it.** It is taken whole, but for its stocks. [0090]
- **Flying out on the day a base falls is not possible.** The fight is after the last pulse and a rebase order is given at the start of a day, so a group must be ordered out a day ahead; one still there is lost whole. [0090, 0091]
- **Men aboard do not eat,** however long the passage. [0066]
- **No ration scale.** Men are fed or unfed. [0066]
- **A side's hold on its own supply.** Men and aircraft draw on one stock, aircraft first, and no order changes that. [0066, 0050]
- **Men saved from a sinking ship,** and men carried in ships not in a task force, are not built. [0066]
- **Loading and landing are one rate.** Men go over a beach and over a berth at the rate of the port, by the one weight of a man. [0066]
- **No arrivals or replacements of men** by schedule. A scenario's units are those it starts with. [0066]
- **A new number can show what the enemy did.** A base formed by a landing and a unit made by division take the next free number counted over both sides, so a gap in the numbers of a side's own bases can show that the enemy formed one. Decision 0047 records the same for a task force split off and 0059 for an air group formed. [0066, 0047, 0059]

## Not in this chapter

- The cargo rule, berths and lighters, and the rates of a port: chapter 2, Cargo.
- Supply stocks, the daily supply for aircraft and a base out of supply: chapter 5.
- Orders in general, and what a side may know and see: chapter 1.
- The ground report and the estimate as belief, and the opening estimate: chapter 3. Shore bombardment: chapter 4. The share of a hit that lands on men: chapter 5.
- Objectives on a base held and on men lost: chapter 6. Aircraft lost with a base and a group's refusal of orders: chapter 5.
