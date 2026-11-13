# 0066: Land units, carried by sea and fed from a base

Status: confirmed. The designer approved the plan on Nov 12, 2026 (`docs/design/ground-combat.md`, stage 1) and agreed decisions 7, 12 and 14 on Nov 13, 2026. The architect added the points under "Settled before the build" the same day. The hand calculation is in `tests/scenario_21_landing.rs`. The figures are placeholders set by the architect. Built on Nov 13, 2026 (stage 1, scenario 21; see "As built" at the end).

## The gap

- **A transport carries supply and fuel and nothing else.** A reinforcement run has nothing to deliver, so there is no reason to make one and none to stop it (decision 0052, "What it costs").
- **Nobody is ashore.** A base has stocks and aircraft and no men. Supply landed feeds aircraft only, so a side that cannot land supply loses nothing but sorties.
- **A side can use only the bases a scenario gives it.** It cannot come ashore anywhere else.

This record is stage 1 of the plan: the land unit, men as cargo, a landing place, and eating. Men do not fight yet. Stage 2 adds the fight and a base that changes side, with its own record.

## What the facts will bear

The research of stage 0 is in `docs/research/oob/land-forces.md`. What it gives this stage, and what it does not:

- **Stated troop capacity for five American transports,** from 1,150 to 1,644 men, and station lists showing ships sailing above the stated figure. The transports are not yet in `ships.csv` or `ship_classes.csv`, so no class can be derived until they are added.
- **No stated capacity for a destroyer or a destroyer-transport of either navy.** The record gives loads on named runs: 154 to 200 men for a destroyer-transport, and run totals for Japanese destroyers that come to about 150 to 170 men a ship. A derived capacity for those classes must rest on run totals, by a rule that says so.
- **One hint for what a man eats:** 120 tons of rations as about three and a half days for the force ashore, on a row graded low. Nothing for the Japanese inside the period.
- **Nothing on how fast an unfed force wasted** before November 1942.
- **Nothing on how fast men were put ashore** as a rate. The landings of 7 to 9 August and the destroyer runs give totals and dates.

So every figure here is a placeholder, and none is taken from a row. Stage 4 reads the rows.

## Decisions

### The land unit

1. **A land unit is a count of men with a side and a name.** Its men are ashore in one hex, aboard ships, or part each. It has a stable number, and a new kind code for an entity, added at the end. It is destroyed when it has no men anywhere; its number is not reused.
2. **Men ashore of one unit are in one hex.** Men aboard are aboard named ships. A ship carries men of one unit only.
3. **A scenario gives a unit** its number, name, side and men, and either a hex, where they are ashore, or a task force, where they are aboard: spread over the force's ships in ship order, each up to its troop capacity. A scenario whose unit does not fit aboard does not start. A scenario with a land unit must name the ground figures file, or it does not start.

### Troop capacity

4. **A ship class has a troop capacity, in men,** apart from its cargo capacity in tons: men do not take up cargo room and cargo does not take up berths. The column is `troop_capacity_men` in the ship classes file, 0 where it is not given, so no existing file changes. For a real class it is derived from fact rows by a written rule, as every figure of a real ship is.

### Loading and landing

5. **A load order may name a land unit.** The order is the load order as it is, for supply or for fuel, with a unit beside the cargo. It takes the unit's men ashore in the force's hex first and the named cargo after. Cargo orders stay one kind, so a new one replaces the old.
6. **It is refused at intake** if the unit is not the side's own, if the unit has no men ashore at the force's position, or if no ship of the force has troop capacity. A load order with a unit is accepted where the side has no base, which a plain load order is not: men can be taken off a shore that has nothing else. It then loads men only.
7. **A man is worked as cargo at a fixed weight.** A ship's work in a pulse is its rate, by berth or by lighter, times its hours, in tons, as now. Each man moved takes `ground.toml: handling_t_per_man` of it. Men are moved first and in whole men; what is left of the ship's tons moves supply and then fuel, by the rule as it is. A ship has cargo to move, for the giving of berths, if it has men to move. *Agreed by the designer, Nov 13, 2026. The other choice was two more columns in the port sizes file.*
8. **Loading** takes ships in ship order. A ship takes men up to its troop capacity. A ship with men of another unit aboard takes none.
9. **An unload order lands men first,** then supply, then fuel. Men landed join their unit's men ashore in the force's hex.
10. **Men are landed only on a coast hex.** In any other hex they stay aboard and nothing is recorded, as cargo does today where there is no base.
11. **A landing forms a base where the side has none.** When the first man of a force is landed in a hex that holds no base of its side, a base of that side is formed there, with the next free base number, the port of the hex, no stock, no guns and no repairs. Supply and fuel unloaded in the same phase go into it. A force with no men aboard forms no base, so nothing changes for a supply run to an empty hex.
12. **A base formed in play is not on the enemy's map.** A side is told the number, name, hex and holding side of every base the scenario starts with: that is the map. A base formed by a landing is known to its own side only. An order against it by the other side is refused with the reason given for a base that does not exist, so the refusal tells nothing. The enemy learns of it by a report, which is stage 3. *Agreed by the designer, Nov 13, 2026: without this, a landing would be shown to the enemy the moment the first man was ashore.*

### A unit in two places

13. **A force may sail with part of a unit.** The men ashore stay where they are and the men aboard stay aboard.
14. **A unit is divided when its men would be ashore in two hexes.** When a force begins to land men of a unit that has men ashore in another hex, all of that unit's men aboard the force's ships become a new unit first, with the next free number, and it is the new unit that lands. The men ashore elsewhere keep the unit. Units are never merged. *Agreed by the designer, Nov 13, 2026. The plan said no rule splits a unit. This one does, in the one case where the other choices are to strand men aboard or to keep a unit's men in several hexes. It costs nothing in the fight, where all of a side's units in a hex are to fight as one body (stage 2).*

### Lost at sea

15. **Men aboard a ship that sinks are lost with her,** all of them (the plan, answer 3). The loss is an event caused by the sinking.

### Eating

16. **Men ashore are fed from their side's base in their hex,** each night, after the daily supply for aircraft. For each side and each hex with men of that side ashore, the base is the lowest-numbered base of the side in the hex, as everywhere. It is asked for `daily_supply_t_per_man` times the men, and pays what its stock covers.
17. **The men fed are the tons paid over the charge for a man, cut to a whole man.** The rest are unfed. Where the side has no base in the hex, or the base has no stock, all are unfed and nothing is paid or recorded as paid.
18. **Aircraft are paid for first.** Both charges fall on the one stock, and the men's follows the aircraft's in the night's order. A stock too short for both feeds the aircraft.
19. **A share of the unfed is lost each day:** the unfed men times `unfed_loss_share_per_day`, cut to a whole man, and one man at least where any man is unfed. Men aboard ships do not eat and are not lost.
20. **Men lost in a hex are taken from the side's units there in proportion.** With n men to take from units of m1, m2 and so on, of M in all: unit i gives n times mi over M, cut to a whole man; the men still to take are taken one from each unit in unit order, round again if need be, from units that still have men. This one rule serves the unfed here and will serve the fight and the shells in stages 2 and 3.

### What a side knows

21. **A side knows its own units exactly:** men ashore and where, men aboard and in which ships. Its view lists them and the briefing shows them.
22. **A side knows nothing of the enemy's land units in this stage.** There is no report of them yet, so there is no belief. The ground report and the estimate are stage 3.
23. **Every event of this stage is seen whole by its own side and not at all by the enemy:** men embarked, men landed, a base formed, a unit divided, men lost with a ship, men fed and unfed, men lost unfed, a unit destroyed. The side that sank a ship carrying men is not told she carried them.

## Settled before the build

Choices of detail made by the architect on Nov 13, 2026, when the hand calculation was written.

- **A load order with a unit may name no cargo.** Its cargo is supply, fuel or none. With none it loads men only, at a base or off a bare shore. Without this, taking men off a beach would take their supply with them. A load order with no unit must name supply or fuel, as now. Where the side has no base the order loads men only whatever cargo it names (decision 6).
- **Units go aboard at the start in unit order,** each over the force's ships in ship order, into ships that are empty of men or hold its own (decision 3).
- **Loading is complete** when no man of the unit is ashore in the hex or no ship has a berth for one, and nothing remains of the named cargo by the rule as it is. The hour of a ship's finishing counts the tons its men took.
- **The men's supply is its own phase of the night pulse,** after air supply upkeep and before air group rest. It runs only when the ground figures are loaded. Hexes are taken in hex order and, in a hex, sides in the order of their codes.
- **Whole-number arithmetic.** Men are whole numbers. The men a ship can move are its tons of work over the weight of a man, cut. The men fed are the tons paid over the charge, cut. A unit's part of a loss is the loss times its men, divided by all the men, cut.
- **A unit made by division** takes the name of the unit it left with ", detachment" and its own number after it.
- **The scenario** is number 21, `21_landing.toml`, on a placeholder map of its own, with two placeholder transport classes in a file of their own.

## What a player sees

- A load order with a unit: `load` with a `unit` and a cargo, which may be `none`. The unload order is unchanged.
- A land unit in the list and detail queries: its men ashore and the hex, its men aboard by ship, and whether its men were fed last night.
- A briefing part for land units: each unit's men and place, men landed and embarked yesterday, men lost and to what, and for each hex with men ashore the days of supply its base holds at the present charge, for aircraft and men together.
- A base the side formed appears among its own bases.

## Placeholder figures

In a new file, `ground.toml`, and in the frozen step 1 figures with the same first values:

| Key | Value | What it is |
| --- | --- | --- |
| `handling_t_per_man` | `"0.25"` | The tons of a ship's work in a pulse that moving one man takes |
| `daily_supply_t_per_man` | `"0.005"` | Supply a man ashore costs his base a day, in tons |
| `unfed_loss_share_per_day` | `"0.05"` | The share of unfed men lost in a day |

The plan called the file `ground_combat.toml`. It is `ground.toml`, since these figures are not of combat and stage 2's will join them.

By lighter, at the placeholder rate of 20 tons an hour, a ship lands 80 men an hour. None of the three rests on a source. The record's one hint for the second, 120 tons for about three and a half days, is of rations alone for a force whose size on that day is itself disputed; step 2 sets the figure, and a coarse measure for the third has nothing inside the period to rest on.

## A small worked case

Two ships with a troop capacity of 600 lie stopped off an empty coast hex from the start of day 1, with an unload order. Unit 1, of 1,000 men, is aboard: 600 in ship 1 and 400 in ship 2. Ship 1 also carries 100 tons of supply. The hex has port size 0, so both work by lighter at 20 tons an hour.

| When | What happens | Result |
| --- | --- | --- |
| Day 1, night, feeding | No man is ashore | Nothing |
| Day 1, night, loading: ship 1 | 12 hours at 20 is 240 tons of work. 600 men take 150. The first man forms the base. 90 tons are left: 90 of the 100 tons of supply are landed | 600 ashore, stock 90, 10 tons aboard |
| Day 1, night, loading: ship 2 | 240 tons of work. 400 men take 100 | 1,000 ashore |
| Day 1, morning, loading: ship 1 | The last 10 tons | Stock 100. Unloading complete |
| Day 2, night, feeding | 1,000 men at 0.005 is 5 tons asked and 5 paid. 5 over 0.005 is 1,000 fed | Stock 95, none unfed |

A second unit of 500 men is ashore from the start in another hex, where its side has no base.

| Night of | Unfed | Lost | Left |
| --- | --- | --- | --- |
| Day 1 | 500 | 500 times 0.05 is 25 | 475 |
| Day 2 | 475 | 23.75, cut to 23 | 452 |
| Day 3 | 452 | 22.6, cut to 22 | 430 |

A short stock: 1,000 men of two units, of 700 and 300, at a base with 2 tons. Asked 5, paid 2, stock 0. 2 over 0.005 is 400 fed and 600 unfed. 600 times 0.05 is 30 lost: 21 from the first unit and 9 from the second, with none left over.

The hand calculation in the test takes the first case from a convoy's sailing, with its passage and its arrival hour, through three days ashore.

## The test

One scenario with the next free number, of made-up ships and units marked placeholder and named "Placeholder", on the placeholder map of the supply run. Beside the hand calculation, property tests:

- **Men are conserved.** For every unit on every day, men ashore, men aboard and men lost by each cause add up to the men it began with, counting a divided unit with the one it came from.
- **More supply never loses more men.** The same scenario with more supply landed, or landed sooner, never ends with fewer men.
- **Men aboard never exceed a ship's troop capacity,** and a ship never holds men of two units.
- **A scenario with no land unit is unchanged:** every existing scenario gives the state it gave before, apart from the hash version.
- **The enemy's view is the same** in two worlds that differ only in a land unit, its landing and the base it formed.

The legibility test asks a cheap model, from one side's briefing and events, why a unit lost men on a given night and how many it will lose on the next.

## Why

- **One mechanism.** Men move through the cargo rule, by the same orders, berths, lighters and hours. Men eat through the base supply rule, from the same stock. Nothing new is invented for either.
- **A base is the landing place.** A hex may already hold bases of both sides, so a beach that destroyers run supply to is a base with a port of size 0 and no stock. No new kind of place is needed, and every rule that works at a base works there.
- **Hunger is the link to the sea.** With men who eat, a supply run that is sunk costs men ashore, and that is the decision at sea this stage exists to create.
- **Nothing changes until a scenario has a land unit.** The ground file is not loaded otherwise, the troop capacity column is 0, and a load order with no unit is the order as it was.

## What it leaves out

- **Fighting, fatigue, posture, and a base that changes side:** stage 2. Until then units of both sides in one hex ignore each other.
- **Shore bombardment, bombs on men, and what a side knows of the enemy ashore:** stage 3.
- **Men aboard do not eat,** however long the passage.
- **A second base in the hex does not feed** when the lowest-numbered one is empty.
- **No ration scale.** Men are fed or unfed; half rations are not a state.
- **A side's hold on its own supply.** Men and aircraft draw on one stock with aircraft first, and no order changes that. Reserve levels stay deferred (decision 0050).
- **Men saved from a sinking ship** (the plan, answer 3), and men carried in ships not in a task force.
- **Loading men and landing them over a beach at different rates.** One figure serves both.
- **Arrivals and replacements of men** by schedule. A scenario's units are those it starts with.

## What it costs

- **New state:** land units; the men aboard each ship and whose they are; whether a base is on the map. Each goes into the hash in its place and is named beside it, the hash version is raised and the golden files are remade (decision 0057).
- **A base can come to exist in play,** which is new, and its number depends on how many bases there are. A side sees the numbers of its own bases, so a gap in them can show that the enemy formed one. It is the leak decision 0047 records for a task force split off and decision 0059 for an air group formed; it is recorded here and not closed.
- **A unit can come to exist in play** by decision 14, with the same remark for its number, which only its own side sees.
- **The map is no longer every base.** Every place that hands a side "the enemy's bases" must leave out those not on the map: the view, the briefing, the queries, the viewer's side view and the intake check of a base strike. The leak tests are extended to it.
- **The supply event gains a reason and a count of men,** and the schemas of the briefing, the queries and the orders change, with their versions.
- **The rulebook:** a new chapter 7 for land units, and changes to chapter 1 (the order of a day, orders and their refusals, what a side may know, what a side sees of an event), chapter 2 (cargo, bases on hexes, troop capacity) and chapter 5 (daily supply), with the glossary, in the same pull request as the code.

## Rule card

| | |
| --- | --- |
| Reads | A land unit's men ashore and its hex; the men aboard each ship and whose they are; a ship class's troop capacity; the cargo order and its unit; the force's position, hours stopped and ships; the hex's kind and port; the bases of the side in the hex and their stock; whether a ship has sunk; the three figures |
| Writes | A land unit's men ashore and its hex; the men aboard a ship; a new land unit; a new base; a base's supply stock; the events of decision 23 |

Three rules now write a base's supply stock in the night: repairs, the aircraft's daily charge and the men's. Their order is fixed (decision 18) and the rulebook's order of a day is given a row for the men's.

## Questions for the designer

Answered on Nov 13, 2026.

1. **Decision 7:** one figure for the weight of a man. Agreed.
2. **Decision 12:** a base formed by a landing is hidden from the enemy until a report shows it. Agreed.
3. **Decision 14:** a unit is divided when its men would be ashore in two hexes. Agreed.

## As built

Built on Nov 13, 2026, to the sections above and the architect's hand calculation in `tests/scenario_21_landing.rs`. Nothing above this section was changed except the status line. Every choice the build made that the record did not is listed here. The rulebook's chapter 7 states the rules as built.

- **Ids.** The id type `LandUnitId` and the entity kind code 6 (`EntityRef::LandUnit`) already existed, reserved, so no code was added to them and nothing was renumbered.
- **Numbers of units and bases made in play.** A unit made by division and a base formed by a landing take the next free number counted over all units or all bases of both sides (the highest plus one), as decision 14 and 11 say, and not a side's own series of decision 0086. sim-agent shows a side its land units numbered among its own (decision 0086, 3), so the player sees its own count; bases keep their numbers. The leak the record notes in "What it costs" stays.
- **The name of a formed base** is "Landing at (q,r)", with the hex.
- **The state.** `Ship::troops` (the unit and the men), `Base::on_map`, `CargoOrder::unit`, the cargo action `LoadNoCargo` (code 4, added at the end), and `TruthState::land_units`, each unit with its men ashore and hex. Hash version 10. The order is in `hash.rs`: a ship's `troops` after her `system_repair_points`; a base's `on_map` after its `yard_passage_days`; a cargo order's unit after its order; the land units after the arrivals, each as number, side, men ashore and hex (the name is not hashed).
- **New events**, at the end of the enum: `MenEmbarked`, `MenLanded`, `BaseFormed`, `UnitDivided`, `MenLostWithShip`, `MenFed`, `MenLostUnfed`, `LandUnitDestroyed` and `UnitLoadingComplete`. The last is the completion of a load order with a unit, which may have no base and no cargo, so `LoadingComplete` was left as it was. `BaseSupplySpent` gains `men` and the reason `men`; every other reason has 0 men. A `MenFed` event is recorded for every side and hex with men ashore, also where nothing was paid; the charge itself is a `BaseSupplySpent` only where a base with stock paid.
- **Causes.** A landing cites the unload order, the arrival and, in the pulse a base was formed, the `BaseFormed` event, and its unit's `UnitDivided` event. A loss unfed cites the `MenFed` event. Men lost with a ship cite the `ShipSunk`. A unit destroyed cites the loss that emptied it.
- **Berths** are given out by side and hex (the side's landing place) and not by base number. With one lowest-numbered base for each side and hex this is the same as before.
- **The order within a landing.** A landing happens before any ship of the force moves cargo in that pulse: units are divided and the base is formed first if any ship can land at least one man in the pulse, so supply unloaded in the same phase goes into the new base.
- **A hex that is not coast.** Men stay aboard and an unload counts as complete without them.
- **The intake order of the refusals** for a load with a unit: under way, out of fuel, not your land unit (one reason for a unit that does not exist and one of the enemy), no men ashore here, no troop capacity.
- **The loader** refuses a land unit with no ground file, with no men, numbered 0 or 1,000,000 and above, given twice, with both or neither of a hex and a task force, ashore off the map or in a hex that is neither coast nor land, aboard an unknown force or one of the other side, or that does not fit aboard.
- **The ground figures.** The weight of a man and his charge must be above 0, and the share lost from 0 to 1; `World::new` refuses them otherwise.
- **sim-agent.** Land units are a `land_units` section of `list_forces` and `get_entity("land_unit:N")`; a ship's detail shows its `troops`; the briefing has a land units part (`crates/sim-agent/schemas/README.md`, "Land units", lists it); `fed_last_night` has its own words; `rules_in_play.supply` is true when the ground figures are loaded as well. The schema `version` stays 3 because only fields were added, by the crate's version rule. The record's "with their versions" was not applied; the architect to confirm.
- **The viewer** rebuilds land units from the events and shows men ashore and formed bases in the hover readout. It has no map marker and no panel for them.
- **The legibility test** of the record (a cheap model asked why a unit lost men) was not run: the repository's legibility tests are wording tests with no model, and a test that runs a model needs the network.
- **The property "more supply never loses more men"** is tested for the half "more supply", over eight sizes of the load. The half "landed sooner" is not tested.

Differences from the record's rules: none. The points above are the names and orders the record left open.
