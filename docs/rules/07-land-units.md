# Land units

This chapter covers men: the land unit, the troop capacity of a ship, the load order that takes a unit aboard, the landing that puts it ashore and forms a base for it, a unit divided, men lost with a ship, and the men's daily supply and the loss of the unfed. Men do not fight yet. Ground combat, a base that changes side, shore bombardment, bombs on men and a side's knowledge of the enemy ashore are not built (the last section).

Men are worked through the cargo rule of chapter 2 and fed through the base supply rule of chapter 5. This chapter gives only what is new in them. Every quantity is a fixed-point number except men, which are whole numbers. "Cut" means cut toward zero (chapter 1, Numbers and chance).

Where these rules run in a day (chapter 1, The order in which a day resolves, gives the whole of it):

| When | Rule |
| --- | --- |
| Start of the day, with the other orders | A load order with a unit is checked and applied (chapter 1, Orders) |
| Night pulse, after the daily supply for aircraft | The men's supply: feeding, the unfed and the loss |
| Loading phase, last in each pulse | Men are taken aboard and landed with the cargo; units are divided and bases formed |
| Whenever a ship sinks | The men aboard are lost with her |

A scenario that names no ground figures has no land units, and every rule here does nothing in it (chapter 1, Where figures come from and when a rule is loaded). [0066]

## The land unit

### What a land unit is

A land unit is a count of men with a side and a name. It has a number, which is stable and never reused. Its men are ashore in one hex, aboard ships, or part each. The men ashore of one unit are all in one hex. The men aboard a ship are of one unit only, and a unit may have men aboard several ships. A unit is destroyed when it has no men ashore and none aboard any ship (below, Destroyed). It stays in the state with no men and keeps its number. [0066 items 1 and 2]

A land unit's state is: its number, its name, its side, the men ashore and the hex they are in. The hex is set exactly when the men ashore are above 0. A ship's state gains the men aboard and the unit they belong to (`troops`), at least one man when set. The state hash holds the men and the hexes and not the name (chapter 1, The state hash). [0066]

A side sees: its own units exactly (below, What a side knows and sees).

### The scenario gives the units

A scenario's `[[land_unit]]` entry gives `id`, `name`, `side` and `men`, and either a hex (`q` and `r`) where the men are ashore, or a `task_force` where they are aboard. The scenario's `ground` key names the ground figures file. The loader refuses a scenario that:

- has a land unit and names no `ground` file;
- gives a unit no men, a number of 0 or of 1,000,000 and above (those are kept for units made in play), or a number twice;
- gives a unit both a hex and a task force, or neither, or only one of `q` and `r`;
- puts a unit ashore off the map or in a hex that is neither coast nor land;
- puts a unit aboard a task force that does not exist or is of the other side;
- puts a unit aboard a force whose ships do not have berths for all its men.

Units are put aboard in unit order, each over the force's ships in ship order, into a ship that has no men aboard or holds men of this unit, each up to its troop capacity. A unit that is aboard has no men ashore. The game is also refused when it is built (`World::new`) if there are land units and no ground figures, or if the ground figures are wrong (below, Data keys). [0066 items 3 and 4, Settled before the build]

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

A base formed by a landing is not on the enemy's map. It is on its own side's, in every list and answer a side is given of its own bases, and it works as any other base of its side in a rule that reads a base: it holds the stocks unloaded into it, it feeds the men ashore (below), it can be loaded from and unloaded to, and a base count is recorded for it at the end of each day. The other side is told nothing of it: chapter 1, What a side may know, gives the map and the base strike refusal. [0066 item 12]

A side sees: `BaseFormed`, whole, for its own base. The enemy sees nothing.

### A unit divided

A unit is divided when its men would be ashore in two hexes. When a force is about to land men of a unit that has men ashore in a hex other than the force's hex, all the unit's men aboard the force's ships become a new unit first. The new unit has the next free number, counted over all the land units of both sides (the highest plus one), the side of the unit it came from, no men ashore, and the name of that unit with ", detachment" and its own number after it. The men ashore elsewhere keep the unit. It is the new unit that lands. Units are never merged. A unit whose men ashore are in the force's hex, or that has none ashore, is not divided. `UnitDivided` records the unit it came from, the new unit, the force and the men, citing the unload order and the arrival. [0066 item 14, Settled before the build]

A side sees: `UnitDivided`, whole, for its own unit.

## Lost at sea

Men aboard a ship that sinks are lost with her, all of them, at the moment she sinks. `MenLostWithShip` records the unit, the ship and the men, citing the `ShipSunk` event. The ship then carries none. A unit left with no men ashore and none aboard any ship is destroyed (below). The side that sank the ship is not told she carried men. [0066 item 15]

A side sees: `MenLostWithShip`, whole, for its own unit. The enemy sees nothing.

## The men's supply

### Feeding

If the ground figures are loaded, the night pulse has a phase of its own for the men's supply, after the daily supply for aircraft and before air group rest (chapter 1, The night pulse). For each hex with men of a side ashore, hexes in hex order (by q, then by r) and, in a hex, sides in the order of their codes (Allied first), the men are the men ashore of all the side's units in the hex, and the base is the lowest-numbered base of the side in the hex. There is no random number. [0066 items 16 to 18, Settled before the build]

If the side has a base there with a stock above 0, the base is asked for `ground.toml: daily_supply_t_per_man` times the men and pays as much as its stock covers. The payment comes off the one supply stock that pays for aircraft, after the aircraft's charge in the night, so a stock too short for both feeds the aircraft. `BaseSupplySpent` (reason men) records the men, the amount asked, the amount paid and the stock left, with no cause. A base with no stock, or no base, is asked nothing and pays nothing, and no payment is recorded. A stock the men's charge empties makes the base out of supply from the start of the next pulse (chapter 5, A base out of supply). [0066 items 16 to 18]

The men fed are the tons paid over the charge for a man, cut to a whole man and never more than the men. The rest are unfed. `MenFed` is recorded for every side and hex with men ashore, also where nothing was paid: the side, hex, the base asked (none if the side has none there), the men, the fed and the unfed. It cites the payment where there was one. Men aboard ships do not eat. [0066 items 17 and 19]

A side sees: `MenFed` for its own men and `BaseSupplySpent` for its own base, whole. The enemy sees nothing.

### The unfed are lost

Where any man is unfed, a share of them is lost: the unfed times `ground.toml: unfed_loss_share_per_day`, cut to a whole man, and one man at least, and never more than the unfed. The loss is taken from the side's units with men ashore in the hex, in proportion (below). [0066 item 19]

### Taking men in proportion

With n men to take from units of m1, m2 and so on, of M men in all, in unit order, unit i gives n times mi over M, cut to a whole man. The men still to take are then taken one from each unit in unit order, round again if need be, from units that still have men. No unit gives more than it has. This rule serves the unfed here. [0066 item 20]

Each unit that gives men records `MenLostUnfed`: the unit, the hex, the men lost and the men it has left ashore, citing the `MenFed` event. A unit with no men left ashore has no hex.

A side sees: `MenLostUnfed`, whole, for its own unit.

### Destroyed

A unit that has lost its last man, ashore or aboard, by the loss at sea or by the unfed, is destroyed. `LandUnitDestroyed` records it, citing the event that took the last men. The unit keeps its number and stays in the state with no men. [0066 item 1]

A side sees: `LandUnitDestroyed`, whole, for its own unit.

## What a side knows and sees

A side knows its own land units exactly: the men ashore and where, the men aboard and in which ships. Its view lists them and its ships show the men aboard. The enemy's land units are not in the view at all: there is no report of them yet, so there is no belief. A base the enemy formed by a landing is not among the enemy's bases the view lists. [0066 items 21 and 22]

Every event of this chapter is seen whole by its own side and not at all by the enemy: men embarked and landed, a base formed, a unit divided, men lost with a ship, men fed, men lost unfed, a unit destroyed and a unit's loading complete. A side's number for its own land units, and the leak that a gap in its own numbers can show, are in chapter 1, What a side may know. [0066 item 23]

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

The ground file is a TOML file with `placeholder = true` and the three keys, each a whole number or a decimal string. A TOML float, an unknown key and a missing key are refused. The frozen step 1 copy (`data/reference/step1/ground.toml`) holds the same first values. Every figure is a placeholder that rests on no source. [0066, 0057]

## Limits

- **Men do not fight.** Units of both sides in one hex ignore each other. Fighting, fatigue, posture and a base that changes side are stage 2 of the ground combat plan. [0066]
- **Shore bombardment, bombs on men and a side's knowledge of the enemy ashore** are stage 3: the enemy learns of a base formed by a landing only by a report, and there is none. [0066]
- **Men aboard do not eat,** however long the passage. [0066]
- **A second base in the hex does not feed** when the lowest-numbered one is empty. [0066]
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
- Ground combat, shore bombardment and what a side believes of the enemy ashore: not built.
