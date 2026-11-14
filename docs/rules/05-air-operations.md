# Air operations

This chapter is the rules for air groups: what they are, what they are ordered to do, how many aircraft can fly, how a strike is decided and fought, and what the flying costs afterwards. Quantities are fixed-point numbers, cut as chapter 1, Numbers and chance, says. A figure named as `file.toml: key` is read from that file in `data/reference`. A figure with no file is written into the code, and the file it is in is named.

A day has three pulses: night, morning and afternoon. The night pulse comes first. Air groups do nothing but rest, repair and receive aircraft at night: nothing flies in the dark (decision 0001, 0012). The air rules in the order they run are listed in "The order of the air rules in a day", and chapter 1, The order in which a day resolves, holds the sequence of the whole day.

Several rules apply only when their figures are loaded, which is defined once in chapter 1, Where figures come from and when a rule is loaded: `air_combat.toml` for strikes, patrol, escorts, raids, reach and running out of fuel, `air_upkeep.toml` for wear and repair, `fatigue.toml` for fatigue and rest, and `air_supply.toml` for supply. Where that matters it is stated with the rule.

## The air group

### What an air group is

An air group is one unit of aircraft of one aircraft type. It holds:

- `ready`: aircraft fit to fly.
- `damaged`: aircraft that came back worn and cannot fly until repaired. Every count of what a group can fly counts `ready` only.
- `pilots`: a count, or none. A group whose scenario gives it no pilots does not track them. One pilot stands for the crew of one aircraft.
- Its starting figures (`ready`, `damaged`, `fatigue`, `pilots`, `strength`) are fields of its entry in the scenario file.
- `strength`: the ceiling for replacements, in aircraft (ready and damaged together) and in pilots. It is what the group started with, unless the scenario gives another figure. Arrivals raise it. Nothing else reads it.
- `mission`, and the order that set it.
- `search`: a search arc (from, to, range), and the order that set it.
- `patrol_share`: from 0 to 1, default 1, and the order that set it.
- `fatigue`: from 0 to 100, default 0.
- `rest_above`: the rest threshold, or none, and the order that set it.
- `reach`: normal or extended, default normal, and the order that set it (see "Radius and reach").
- `stood_down`: whether the staff stood the group down today.
- `rebased_day`: the day it last moved to another base by order, or none (see "Rebasing an air group").
- Its place: exactly one base or exactly one ship, or lost (see "A group lost with its base").

[0025, 0035, 0036, 0038, 0041, 0059, 0090, 0091]

A side sees: its own groups whole. The enemy sees nothing of a group except through a sighting report, a claim, or the fight events listed with each rule below.

### Where a group is

A group at a base is at the base's hex. A group aboard a ship is at the position of the ship's task force, and moves with it with no order. A group aboard a ship that is in a port is at the port's hex, and is still aboard the ship. A group aboard a ship that has sunk has no position. A group cannot change between a ship and a base. A group at a base changes base only by a rebase order (see "Rebasing an air group"). Aircraft can come to be at another place also by landing there when their own deck is lost (see "Coming home to no deck"), and they then belong to a group of that place. A group with no aircraft is at its place all along and can be given orders. A group lost with its base has no place at all. A group is never at a base with no airfield (chapter 7, Units that work and what they build): the loader refuses a scenario that puts one there, a rebase order to one is refused, and aircraft that cannot get home do not land at one. [0025, 0041, 0052, 0059, 0090, 0091, 0099]

A side sees: its own group's place. The enemy sees only what a report says of a force.

### Aircraft capacity of a ship

A ship class has an aircraft capacity: a whole number, 0 for a ship that carries none (`ship_classes*.csv: aircraft_capacity`). The ready and damaged aircraft of all the groups aboard a ship together may not exceed it. A scenario that breaks this does not load. The check is also made at the end of every day, and a breach stops the run with an invariant error. Capacity limits nothing else: arrivals are not tested against it (see "Known limits"). Any aircraft type may be based on any ship with the capacity. [0025, 0035]

A side sees: its own capacity and aircraft count. The enemy does not.

### A group lost with its ship

When a ship sinks, every group aboard it loses the aircraft that were aboard her, ready and damaged, and, if its pilots are tracked, their pilots. The aircraft that were in the air in the pulse are not aboard: they are a group's aircraft that flew in the pulse and were not shot down, which are the aircraft of the sorties noted for it (see "What counts as a sortie"). A sinking in the air combat step finds those noted; a sinking in a surface fight in the movement phase finds none, since the pulse's flying is over, and so loses every aircraft. The group keeps the aircraft in the air as ready aircraft, and one pilot with each where pilots are tracked, until the wear phase of the same pulse brings them home (see "Coming home to no deck"). It stays in the state, aboard the sunk ship, with no position. It records `AirGroupLost` (caused by the `ShipSunk` event, giving only the aircraft that were aboard, 0 if there were none) and, if it lost pilots, `PilotsLost` (caused by `AirGroupLost`, giving the pilots left). An order to such a group is rejected with `AirGroupLost`. A group aboard a ship that sinks in the pulse still attacks in that pulse, because every attack is worked out before any damage lands. [0025, 0026, 0035, 0038, 0041, 0059]

A side sees: `AirGroupLost` and `PilotsLost` for its own groups, whole. The enemy sees nothing of them, and learns the ship sank only through a claim or a report.

### A group lost with its base

When a base changes side (chapter 7, A base that changes side), every group located at it is lost whole, in the ground combat phase after the third pulse, so none is in the air. Its ready and damaged aircraft go to 0 and, if its pilots are tracked, its pilots go to 0. Its place becomes `Lost` (the hash code 3): it is at no base and aboard no ship, and has no position. It keeps its mission, and it keeps its entry in the state. `AirGroupLostWithBase` records it (the group, the base and the aircraft lost, caused by `BaseChangedSide`) and, if it had pilots, `PilotsLost` (caused by it, with 0 left). The aircraft are counted in the `aircraft_destroyed` tally of the enemy of the group's side, as aircraft lost with a ship are (chapter 6). [0090 item 20]

From then on the group flies nothing: it has no aircraft and no position, so no search, strike, patrol or escort. It receives no arrival (Aircraft that arrive on a day) and no replacement, it pays and costs no supply, and it is not repaired. An order to it is refused with its own reason, "the air group was lost with its base", and not with the reason for a group lost with its ship (chapter 1, Every reason an order is refused). A group that was flown to another base by a rebase order on an earlier day is at that base and is not lost. The invariants check that a group with this place has no aircraft. [0090 items 20 and 24]

A side sees: `AirGroupLostWithBase` and `PilotsLost` for its own groups, whole. The enemy sees nothing of them. [0090 item 30]

## Missions and orders

### The five missions

A group has one mission at a time. It flies only that mission. A group takes up its mission only if the other rules of this chapter let it fly. A new group's mission is rest. [0026, 0027, 0034]

| Mission | What the group does |
| --- | --- |
| Rest | Stays on the ground. Flies nothing. |
| Search | Flies its search arc in each daylight pulse, in the search step (chapter 3, Air search). |
| Naval strike | Decides in each daylight pulse whether to strike a contact its side holds, of a kind on its target list. |
| Base strike (names an enemy base) | Strikes that base in the first daylight pulse of the day in which it is in reach, once a day. |
| Combat air patrol | Keeps a share of its aircraft over its own hex, and escorts raids with the rest. |

The group's mission order, if the mission was set by an order, is a cause of every event that group's flying produces. A group on a strike mission does not search. A group on search does not strike or patrol. A group on patrol does not strike. [0026, 0027, 0034]

A side sees: its own orders and what they cause, whole. Nothing of the enemy's missions.

### Air group orders

The orders that act on an air group, and every reason one is refused, are in chapter 1, Air group orders and Every reason an order is refused. An accepted order becomes the standing order and stays until another of its kind replaces it. What each means to the rules here:

- A mission order sets the mission (the table above).
- A search arc order sets the arc that the search step flies (chapter 3, Air search).
- A patrol share order sets `patrol_share`, which only The split reads. It does nothing for a group that is not on combat air patrol.
- A rest threshold order sets `rest_above`, which only Stand-down and return reads. It does nothing when the fatigue figures are not loaded.
- A reach order sets `reach` to normal or extended, which the strike decision, the base strike and the escorts read (see "Radius and reach"), and which a sortie beyond the radius pays for (see "Running out of fuel"). It does nothing for a group that does not strike or escort.
- A target priority order sets `target_priority`, a list of kinds of ship, and `target_order`. The strike decision reads it to choose a contact, and the raid reads it to choose what to attack (see "Naval strike: the contact" and "The attack on a ship"). It does nothing for a group that does not strike, and it is accepted for any group.
- A rebase order is not a standing order. It is applied when it is taken in and is then spent (see "Rebasing an air group").

The checks are made against the truth, and a refusal cites only what the side can see (chapter 1). [0026, 0027, 0034, 0036, 0050, 0059, 0063, 0091]

A side sees: `OrderAccepted` or `OrderRejected` for its own orders, whole.

Not built: an order to change the repair order (0052).

### Rebasing an air group

A rebase order (`rebase`, with `air_group` and `base`) moves a group at a base to another base of its side. It is given in a scenario file as `kind = "rebase"` with `air_group` and `base`. It is checked and applied at orders intake, at the start of the day, in the same step as a yard order (chapter 1, Orders). It is for one day's move and is spent when it is done. [0091]

**Refusals.** Chapter 1, Every reason an order is refused, lists them in the order they are tried. In short, the order is refused if the group is not the side's own or is lost with its ship or with its base; if it is aboard a ship (away at the yard or not); if it has already moved to another base that day; if the base is not one of the side's own; if the group is already at that base; or if the base is beyond the type's transfer range. A base a landing formed (chapter 7) is a base of the side and may be named, but if it has no airfield the order is refused with "the base has no airfield", tried after the check that the group is not already there and before the range. [0091, 0099]

**The transfer range** is twice the longer of the type's search range (`aircraft_types*.csv: search_range_nm`) and strike radius (`strike_range_nm`), read through `RefData::transfer_range_nm`. The group's reach does not change it. The distance is the air distance between the two bases' hexes (Air distance, above): along an axis it is 40 miles a hex. A distance is within the range if it is less than or equal to it. [0040, 0091]

**What the move does.** In this order:

1. The group's damaged aircraft cannot fly and are written off at the base it leaves. `AircraftWrittenOff` (group, base, aircraft) is recorded, only if there were any, and the group's `damaged` falls to 0.
2. The group's place becomes the new base, and its `rebased_day` is set to the day. Its ready aircraft and all its pilots go with it.
3. `AirGroupRebased` (group, from, to, distance, transfer range, ready aircraft, pilots) is recorded. When breakdowns are on it carries the type's search range and strike radius, the transfer range and the distance.

Both events come straight after the order's `OrderAccepted` and cite the order. Nothing is rolled: no wear, no ditching, no supply for the flight and no fatigue. Written-off aircraft are aircraft lost for good, as aircraft written off by wear are (chapter 6, Aircraft destroyed). [0091]

**The day of the move.** The group flies nothing that day: no search, strike, escort or patrol. This is limit 2 of "The order of the limits". Its aircraft are on the ground at the new base for any raid on it that day. Its mission, search arc, target list, rest threshold, reach, patrol share, stood-down state and fatigue are not changed, and it flies them from the new base from the next day. A group that did not fly gains and loses fatigue as any group that did not fly does. A second rebase order to the group on the day it moved is refused, so a move by way of an intermediate base takes two days. [0091]

**What follows the group.** The move is made before the first pulse, so the night pulse of that day already works on the new base. Arrivals named for the group are delivered to it where it now is (Aircraft that arrive on a day). It leaves its old base's replacement stream and comes under the new base's, if that base has one (Replacements). The new base pays its daily supply for the group's aircraft from that night (Daily supply). Nothing of this needs a rule of its own: each reads the group's place. [0038, 0039, 0041, 0091]

A side sees: its own `AirGroupRebased` and `AircraftWrittenOff`, whole. The enemy sees nothing of them. Both cite only the side's own order. [0091]

Data: no figure of its own. It reads `aircraft_types*.csv: search_range_nm` and `strike_range_nm`.

## What a group can fly in a pulse

### The order of the limits

Search, strike launch and the split of fighters into patrol and escorts all use one number, the aircraft the group is able to fly. It is worked out, for a group, in this order. [0026, 0036, 0038, 0039, 0091]

1. A group at a base that is out of supply at the start of the pulse flies none. A group aboard a ship is never limited this way.
2. A group that moved to another base by order that day flies none (see "Rebasing an air group").
3. Start from the smaller of the group's ready aircraft and its pilots. A group whose pilots are not tracked starts from its ready aircraft.
4. If the group is aboard a ship, multiply by the ship's flight deck factor: 1 if the deck is working, `surface_combat.toml: degraded_factor` if degraded, 0 if out. A group at a base is not multiplied. Without the surface combat figures this step does nothing.
5. Fatigue. If `fatigue.toml` is loaded, a group that is stood down flies none, and any other group is multiplied by (100 minus its fatigue) over 100, so loaded fatigue figures do cut the aircraft. If `fatigue.toml` is not loaded, nothing is multiplied, no group is ever stood down, and fatigue cuts nothing.
6. Cut the result to a whole number once, toward zero. The multiplications of steps 4 and 5 are fixed-point, so no rounding happens before this cut.

The cut is applied once, to the product. Chapter 1, Where figures come from and when a rule is loaded, says what "loaded" means. A group of 6 aircraft with fatigue 1 and a working deck flies 5. A group aboard a carrier with a degraded deck and fatigue 20, 10 ready and 10 pilots, flies 10 times the degraded factor times 0.8: with a degraded factor of 0.5 (an example, not the figure) that is 4. A group's flight deck is worked group by group, not for the ship. [0026, 0036, 0038, 0039, 0091]

A side can ask this choice for its own air groups before it is made (`measure`'s `choices`, `aircraft_able_to_fly`): the answer is the number of aircraft the group is able to fly now, from the group, its base's supply, its ship's flight deck, its fatigue and the day it moved. [0100]

A side sees: the number it flew in its own events (`SearchFlown`, `StrikeLaunched`, `EscortsSent`). It is not told the limits that produced it.

### A base out of supply

A base is out of supply at the start of a pulse if its supply stock is exactly zero then, and the air supply figures are loaded (`air_supply.toml`). This is fixed at the start of every pulse, before any step of it. A base bombed to zero in a pulse still flies that pulse. A base that starts a pulse with any stock flies in full. A base with no stock flies nothing in that pulse: its groups keep their missions and their aircraft are on the ground. Nothing else changes for such a base: its guns still fire, its aircraft are not lost, repairs are limited by stock (see "Repair"), and its groups rest as usual. It flies again in the first pulse that starts with stock. [0039]

Not built: rationing a short stock among groups, supply for ships, a carrier's own supply (0039).

A side sees: `BaseSupplySpent` for its own bases. The enemy sees nothing.

### Missions and light

Search, strike decisions and air combat happen in the two daylight pulses only. Strike decision tests `is_dark` for the group's hex, which is true exactly in the night pulse. A group flies no strike by night. [0001, 0012, 0026]

## Air distance, radius and reach

### Air distance

The air distance between two hexes is the straight line between their centres, in whole miles. With dq and dr the differences of the hexes' q and r coordinates, it is 40 times the square root of (dq squared plus dr squared plus dq times dr), rounded to the nearest mile. It is worked in whole numbers. Along an axis of the grid it is 40 times the number of hexes. Every rule, event and tool that says how far an aircraft flies uses this one function (`Hex::air_distance_nm` in `hex.rs`). A distance is within a radius if it is less than or equal to the radius. [0040]

A ship steams hex by hex but is charged the same straight-line miles where the way is clear, and 40 a step where land forces a detour (chapter 2, What a step is charged). A threshold counted in hexes, such as how near a contact must be to alert a force, stays in hexes. [0040, 0058]

A side sees: the distance in its own events that carry one.

### Radius and reach

A type has a search range (`aircraft_types*.csv: search_range_nm`), a strike radius (`strike_range_nm`) and a sweep width (`sweep_width_nm`). The strike radius is how far out an aircraft goes and comes back, and the same figure is the radius for a strike and for an escort. The derivation sets it from the sourced combat radius where exactly one can be read, and otherwise from a third of the sourced range with a normal load, cut to a whole mile (0029). The search range is the largest range a search arc may name. It is a radius of its own (0075): a sourced scouting radius where exactly one can be read, else a third of the type's range in the scouting condition (as a scout, or at maximum fuel with no strike load), cut to a whole mile; where the rows cannot settle that range, the strike radius stands for it and the type is listed as not settled. A scenario's search order names the distance the record flew, at or below it. [0026, 0029, 0030, 0040, 0059, 0075]

A group has a **reach**, set by order (see "Air group orders"): normal, which is the default, or extended. Reach is how far its strikes and its escorts may go, and it is read through one function, `RefData::reach_nm`, by every rule and tool that asks:

- On normal reach the reach is the type's strike radius.
- On extended reach it is the strike radius times (1 plus `air_combat.toml: extended_reach_share`), cut to a whole mile. With a radius of 250 and a share of 0.25 that is 312.5, cut to 312.

A naval strike group on extended reach strikes a contact within its reach, a base strike group a base within its reach, and a fighter group on extended reach sends its escorts with a raid within its own reach. A distance is within the reach if it is no more than it. The briefing and `measure` give the same reach, because they ask the same function. The price of reach beyond the radius is paid on the way home (see "Running out of fuel").

Known limit: reach is a hard cutoff. A group strikes only if the target is within its reach, and escorts go only if the raid's hex is within theirs. There is no partial effort, no reduced load and no drop-tank order. Decision 0030 lists drop tanks as deferred. The radius of a type with a sourced combat radius and one without are not the same kind of figure (0030). [0059]

### What a type carries

Each aircraft type has these fields in `aircraft_types*.csv`:

- `role`: patrol (search aircraft), bomber or fighter.
- `attack_weapon`: none, bomb or torpedo. A fighter attacks with nothing.
- `attack_damage`: the damage one hit does before the target's toughness. 0 for a type that attacks with nothing.
- `pilot_armour` and `self_sealing_tanks`: yes or no.

A type that attacks with a weapon must have a positive strike radius and attack damage. A type that attacks with nothing cannot be given a strike mission. The derivation of these figures (0029) reads each from sourced facts: the weapon is the load named for the type in the scenario's aircraft list, the damage uses the constants ships already use, the role follows the type's role in the facts, and each protection is a sourced yes or no. A ship class's `anti_aircraft` column is the number of anti-aircraft shots the ship fires, from its heavy guns and light barrels as the derivation sets (0029). [0026, 0029]

## Start of the day: the night pulse

The night pulse runs these air steps first, in this order: repair, arrivals, replacements, daily supply, stand-down. The same order is in the table in "The order of the air rules in a day". Each step reads the state the earlier ones left.

### Repair

At the start of the night pulse, repair runs if `air_upkeep.toml` is loaded. Without that file nothing is repaired.

1. Places are listed in order: every base whose repairs for the night are above 0, by base number, the repairs being the base's `aircraft_repairs_per_day` (a figure of the base's entry in the scenario file, 0 if not given) and one more aircraft for each `work_per_aircraft_repair` of work that land units made on the base's aircraft the day before, cut to a whole aircraft (chapter 7, Units that work and what they build; none on the first day), so work makes a base a place of repair also where its own figure is 0; then, if `air_upkeep.toml: carrier_repairs_per_day` is above 0, every ship afloat that carries at least one group, by ship number. Each place has an allowance: the base's repairs for the night, or the carrier figure for each ship.
2. For each place in order, the groups located there with damaged aircraft are taken lowest group number first. A group's repair is the smaller of the allowance left and its damaged aircraft.
3. At a base, with `air_supply.toml` loaded and `repair_supply_t` above 0, a repair is also limited by stock: the whole number of repairs the stock covers, which is the stock divided by `repair_supply_t`, cut to a whole number. The repair is the smaller of the two. If that is 0 the base stops repairing for the day, and no later group at the base is repaired. A ship pays no supply.
4. Each repaired aircraft moves from `damaged` to `ready`. `AircraftRepaired` is recorded for each group repaired, caused by the group's latest `AircraftWorn` event that left an aircraft damaged. An aircraft damaged from the start has no cause.
5. At a base that paid for repairs, one `BaseSupplySpent` (reason repairs) is recorded after the base's groups, for the repairs times `repair_supply_t`, caused by that base's `AircraftRepaired` events. The stock is taken at each repair, so the payment is whole.

Repair is by the day and by place. It uses no random number. A carrier's allowance is per ship. Where the allowance above the base's own figure came from work, the `AircraftRepaired` events also cite the day's `UnitWorked` events at the base. [0035, 0039, 0099]

Not built: an order to change which group is repaired first (0052), repair by type, small airfields that wear aircraft faster (0035).

A side sees: `AircraftRepaired` and `BaseSupplySpent` for its own groups and bases, whole. The enemy sees nothing.

**A group aboard a ship at the yard.** An air group aboard a ship that is away at the yard (chapter 4, Repair) goes and returns with her, with its aircraft, damaged aircraft and pilots as they were. While she is away it flies nothing, is not repaired (a ship at the yard is not a place of the list above), gets no replacement, and its wear and fatigue do not change: it is neither stood down nor returned. An arrival due to it while it is away is not delivered and not kept (below). Its aircraft are not at a base, and a group aboard a ship cannot be rebased (a rebase order to it is refused as away at the yard), so it does not fly ashore. The group is not lost: it is lost only with a sunk ship (`AirGroupLost`). Shipyard repair is for the ship and does not repair the group's damaged aircraft. [0069]

### Aircraft that arrive on a day

A scenario may list arrivals (`[[arrival]]` entries in the scenario file): a day, an air group, a number of aircraft and a number of pilots. After repair and before replacements, on every day including day 1, each arrival due that day, in the order listed, is delivered:

1. If the group does not exist, or was lost with its base, or is aboard a ship that has sunk or is away at the yard, nothing happens and nothing is recorded. The aircraft are not kept for the ship's return.
2. The group's `ready` and `strength` each rise by the arrival's aircraft.
3. If the group's pilots are tracked, its pilots rise by the arrival's pilots. If not, the arrival's pilots are dropped.
4. `AircraftArrived` is recorded, with no cause (nothing orders it), giving the arrival, the group's ready aircraft, pilots and strength after it.

An arrival is delivered to the group wherever it now is, including a base it was rebased to (see "Rebasing an air group"). Arriving aircraft are ready. The flight in costs nothing: no wear, fatigue or supply, and nothing can intercept it. An arrival on day 1 is the same as starting with the aircraft. No random number. [0041, 0091]

A side sees: `AircraftArrived` for its own groups, whole.

### Replacements

A scenario may give a side replacements at a base (`[[replacement]]` entries in the scenario file): for one aircraft type, a number of aircraft a day and a number of pilots a day. From day 2 on, after arrivals, each stream is worked in the order given:

1. If the base is not still held by the stream's side, nothing happens.
2. The groups located at that base, of that side, that fly that type are taken lowest group number first. A group rebased to the base is one of them, and one rebased away from it is not (see "Rebasing an air group").
3. For each, the aircraft it receives are the smaller of the aircraft left in the stream and its `strength` less its ready and damaged aircraft (zero if that is negative). The pilots it receives, if its pilots are tracked, are the smaller of the pilots left in the stream and its strength less its pilots. A group whose pilots are not tracked takes aircraft and no pilots, and the day's pilots pass to the next group.
4. A group that receives anything gets the aircraft as ready and the pilots added. `ReplacementsReceived` is recorded for it, with no cause.
5. What no group can take is lost. Nothing is held over to the next day.

Day 1 has nothing to replace. Replacement pilots are as good as the pilots they replace. Replacements cost the base no supply. No random number. [0038, 0039, 0041]

A side sees: `ReplacementsReceived` for its own groups, whole.

### Daily supply

After replacements, each base, in base order, pays `air_supply.toml: daily_supply_t_per_aircraft` for each aircraft located at it, ready and damaged together (a rebased group's aircraft are at the new base, see "Rebasing an air group"), as much as its stock covers. The aircraft are counted after the night's repairs and replacements. A base with no stock, or with nothing to pay, pays nothing and records nothing. Otherwise `BaseSupplySpent` (reason upkeep) records the aircraft, the amount asked, the amount paid and the stock left, with no cause. A payment the stock cannot cover takes what is left, and nothing is owed. No random number. [0039]

The men ashore in the base's hex are charged after this, in the men's supply phase of the same night, on the same stock: `ground.toml: daily_supply_t_per_man` for each man. It is recorded as `BaseSupplySpent` with the reason men and a count of men, and the aircraft count 0. A stock too short for both feeds the aircraft: their charge is paid first. Every other `BaseSupplySpent` has 0 men, except a march's payment, which carries the men marching (chapter 7, Marching overland). Chapter 7, The men's supply, gives the rule. [0039, 0066]

A side sees: `BaseSupplySpent` for its own bases, whole.

### Stand-down and return

After the daily supply, if the fatigue figures are loaded, each group with a rest threshold is taken in group order. If its fatigue is above its threshold and it is not stood down, it is stood down. If it is stood down and its fatigue is not above its threshold, it is returned. A group whose state does not change records nothing. `AirGroupStoodDown` or `AirGroupReturned` records the fatigue and the threshold, caused by the rest threshold order.

A group with no threshold is never stood down. A stood-down group flies nothing that day: its able-to-fly number is 0, its aircraft are on the ground, and it keeps its mission and takes it up again when it returns. There is one threshold and no second figure for coming back, so a group goes back to flying on the first night its fatigue is at or under the threshold. No random number. [0036]

This is the staff's rest. The player sets the threshold and the staff applies it. [0036]

A side can ask this choice for its own air groups before it is made (`measure`'s `choices`, `rest`): the answer is whether a tired group would stand down or return now, from its fatigue and threshold. [0100]

A side sees: `AirGroupStoodDown` and `AirGroupReturned` for its own groups, whole.

## The strike decision

### When and who

The strike decision step runs in each daylight pulse, after air search and before air combat. It is worked on the state at the start of the step. Both sides decide before either lands: every decision of the pulse is made before any attack of the pulse is worked out. Groups are taken in group number order. A group decides only if all of these hold, in this order:

1. Its mission is naval strike or base strike, and it has ready aircraft.
2. It has a position (a group aboard a sunk ship has none).
3. It is not dark at its hex (the pulse is not night).
4. Its type attacks with a weapon.
5. The number of aircraft it is able to fly is above 0.

Any failure means the group does nothing, and nothing is recorded. The decision is made on the group's own side's belief and the group's own state. It reads no enemy truth, except that a base strike reads the map's bases (which side holds a base and its hex), which both sides know. [0026, 0034, 0036]

A side sees: nothing of a decision that launched nothing.

### Naval strike: the contact

A naval strike group considers each contact its side holds. A contact qualifies if both hold:

- Its age in pulses is at most `air_combat.toml: strike_contact_max_age_pulses`. A contact is not aged in the pulse it was reported in, so with the figure at 0 a contact reported this pulse or the one before qualifies, and no older one.
- Its estimated hex is within the group's reach (the type's strike radius on normal reach) of the group's hex, by air distance.
- Its estimated kind is on the group's target list. The list is a standing order (see "Air group orders"), and its default is all six kinds in the order of importance (chapter 3, What a report holds). A kind that is not on the list is never launched at, so a short list holds the strike back: a group whose list is "carrier" stays on deck until a carrier is reported in reach.

From the contacts that qualify the group takes the one whose estimated kind comes first on its list, then the nearest by air distance, then the lowest contact number. With none it does nothing, and nothing is recorded. A report that names the wrong kind sends the strike wrong.

This choice is one function, which takes the side's belief and the group's own state and orders and nothing else. The strike decision calls it, and the side's view offers it (`SideView::strike_choice`), so the briefing and a planner can ask what a group would strike now. The view takes the group's number and answers only for the side's own groups. [0085]

It launches every aircraft it is able to fly. It flies to the hex the contact names, not to where the enemy really is. It records `StrikeLaunched` (group, contact, hex, aircraft, one-way air distance), caused by the group's mission order, by the group's reach order if it is on extended reach, by the group's target order if it has one (a group with none cites nothing new), and by the event of the report that last refreshed the contact. A strike in flight is working data of the pulse. It exists from the decision step to the air combat step and is not part of the state hash. [0026, 0032, 0040, 0059, 0063]

This is the choice a side can ask as `measure`'s `choices`, `strike`: the answer is the contact the group would strike now, or none. [0100]

A side sees: `StrikeLaunched` for its own group, whole. The enemy sees nothing at launch.

### Base strike

A base strike group launches in a daylight pulse, morning or afternoon, once a game day: it launches in the first daylight pulse of the day in which the base is within its reach, and not again that day. It needs no contact and reads no belief. It launches if it has not launched at its base that game day, and the base exists, is held by the other side, and is within the group's reach by air distance from where the group is when the pulse begins. A group that can launch in the morning does, so a group whose base is in reach every morning flies as it did. A group whose base comes into reach during the day, as a carrier's does as she closes it, launches in the afternoon. The group keeps the game day of its last base strike (`last_base_strike_day`, in the state hash), and a launch sets it. It launches every aircraft it is able to fly and records `BaseStrikeLaunched` (group, base, hex, aircraft, one-way distance), caused by the group's mission order and, on extended reach, its reach order. [0034, 0059, 0063]

A side can ask this choice for its own air groups before it is made (`measure`'s `choices`, `base_strike`): the answer is whether the group would launch a base strike this pulse, from the group, its target, its reach and the base on the map. [0100]

A side sees: `BaseStrikeLaunched` for its own group, whole.

### A group may fly more than once a day

Nothing in these rules keeps aircraft that flew in the morning from flying in the afternoon, except the losses, wear, fatigue and supply that the morning's flying caused. A naval strike group may launch in both daylight pulses. [0026, 0035, 0036]

## Escorts and patrol

### The split

At the start of the air combat step each fighter group on combat air patrol that has ready aircraft is split. Let F be the aircraft it is able to fly. The patrol is F times the group's patrol share, cut to a whole number. The escorts are F minus the patrol. The patrol stays over the group's own hex. The escorts go with a raid launched from the group's own place, if there is one and it is within the group's reach (see Which raid the escorts join, and the reach). A fighter group that is not on combat air patrol is not split and flies nothing. [0027, 0036]

A side can ask this choice for its own fighter groups before it is made (`measure`'s `choices`, `fighter_split`): the answer is how the group would split into patrol and escorts now, from the group and its patrol share. [0100]

The patrol defends the hex the group is in, against every enemy raid that attacks that side's forces or base there, in a daylight pulse. [0027]

A side sees: nothing of the split itself.

### Raids

The strikes launched in one pulse from one place at one hex are one raid. A place is a ship or a base. The raid's bombers are the aircraft of those strike groups. Strikes at ships and strikes at a base from one place at one hex are two raids; two strikes at a base from one place are one raid only if they name the same base. Raids are ordered by their first strike group, the lowest group number first. A raid with a strike group whose aircraft the fights leave at zero is still a raid. [0027, 0034]

### Which raid the escorts join, and the reach

All the escorts at a place go with one raid from that place: the raid with the most bombers launched, and of those the one whose first group has the lowest number. They look for no nearer raid. The raid is tested for the reach: the escorts go only if the raid's hex is within the fighter group's reach (its type's strike radius on normal reach) of the fighter group's hex, by air distance. If the raid is beyond the reach, the whole group that can fly is its patrol for the pulse: its patrol rises by its escorts and its escorts fall to 0. If no raid launches from the place, its escorts fly nothing and are on the ground. A group that is split has ready aircraft, so it is at a base or aboard a ship that is afloat, and always has a position: a group aboard a sunk ship has no aircraft and is not split. The test is made on the hex the raid is sent to, which is where its side believes the enemy is, so a group whose escorts go with a raid to an empty hex has still sent them. [0027, 0030]

A side can ask this choice for its own fighter groups before it is made (`measure`'s `choices`, `escort`): the answer is which raid the group's escorts would go with, or that they stay, from the side's own raids of the pulse. [0100]

`EscortsSent` is recorded for each fighter group whose escorts go (group, the raid's first strike group, aircraft), caused by the group's mission order, its share order, its reach order if it is on extended reach, and the raid's `StrikeLaunched` or `BaseStrikeLaunched` event. It is recorded only when escorts go. [0027, 0030, 0059]

A side sees: `EscortsSent` for its own group, whole. The enemy sees nothing of it.

### The patrol divided among raids

When more than one raid attacks forces of one side in one hex in a pulse, that side's patrol over the hex is divided among those raids. The patrol is the sum of the patrol aircraft of that side's fighter groups at that hex, over ships and base together. A raid's aircraft are its bombers plus the escorts sent with it. A raid that finds nothing takes no share and does not count. All shares are worked out on the start-of-step counts before any raid is resolved, so the order of raids does not matter.

- A raid's share is the patrol times its aircraft over the aircraft of all the raids there, in whole numbers, cut toward zero.
- The patrol left over, patrol less the shares, goes one fighter each to the raids in order of size, the largest first (aircraft, bombers and escorts), and on a tie the one whose first strike group has the lower number. The shares add up to the patrol.

With one raid the share is the whole patrol. A raid whose share is 0 meets no patrol: its escorts do not fire at the patrol either. [0033]

How a patrol divides between the raids over it, and between escorts and bombers, is made at the point of contact from the true size of each raid, and a side cannot ask it. [0100]

A side sees: nothing of the division. The factors of the patrol's own fire (below) show the side its own patrol and share.

## The raid, step by step

### The order of the air combat step

The air combat step runs in each daylight pulse after the strike decision. Everything is worked out on the state at the start of the step, then recorded and applied. If no strike was launched the step does nothing except note the patrols for the wear step. In order:

1. Group the strikes into raids, split the fighters, send the escorts, count the aircraft on the ground at attacked bases, note the patrols and strikes for wear, and divide the patrol.
2. Work out every raid on the start-of-step counts, whatever the others do. Nothing is written.
3. For each raid in order of its first strike group, record `EscortsSent` for each group that sends escorts, then each fire roll before the attack as an event (`AirFight`, then `AntiAircraftFire` or `BaseAntiAircraftFire`), then take the losses of each roll off the groups and record `AircraftLost` for each group that lost aircraft, and `PilotsLost` where the rule below says.
4. For each strike group in order of group number, record what it found: `StrikeFoundNothing`, `AirAttack` or `BaseAttacked`. A strike group with no bomber left records nothing and sends no report.
4a. For each raid in order of its first strike group, record the patrol's fire at the bombers after the attack, if it has shots (see "The patrol against the escorts and the bombers"), then the guns' fire after the attack, if it has shots (see "Anti-aircraft fire"), each followed by the `AircraftLost` and `PilotsLost` it causes. [0079, 0087]
5. In the same group order, apply each attack: for ships, apply the damage and send the sighting report; for a base, apply the hits on aircraft and supply; for a strike that found nothing, send the report that nothing is there.
6. Record the claims of the pulse's raids (see "The report and the claim").

Both sides' strikes land in this one step. A group aboard a ship that sinks in step 5 still attacked in step 4. [0026, 0027, 0033, 0034, 0056]

### A strike that finds nothing

A raid's target is found on the truth at the start of the step. For a raid at ships, the target is the enemy task force at the raid's hex that holds the best ranked ship afloat by the target list of the raid's first strike group (see "The attack on a ship"), and on a tie the lowest task force number. With the default list that is the force with the most important ship, as it was before target lists. Position is read before the pulse's movement, because movement follows air combat in the daylight pulse. If no enemy task force is at the hex, the raid finds nothing. It does not search neighbouring hexes. A base strike always finds its base, because the launch checked it exists and is held by the other side.

Which enemy force a raid attacks, and whether it finds anything, is made at the point of contact from the enemy forces truly in the hex, and a side cannot ask it. [0100]

A raid that finds nothing meets nobody: no escorts fire, no patrol fires, no guns fire, and it takes no share of a patrol. Every strike group in it records `StrikeFoundNothing` (group, hex), caused by the strike's launch. Then, in group order and with no random number, the side's belief is told nothing is there. If the side still holds the contact the strike was sent at and that contact still names that hex, the contact is dropped and `ContactNotThere` is recorded, caused by that group's `StrikeFoundNothing`. If the contact has gone or a later report has moved it, nothing is dropped and nothing is recorded. A dropped contact's number is not used again. A strike group that has no bomber left does not find nothing: it records nothing, and the contact stays. [0026, 0027, 0028, 0063]

A side sees: `StrikeFoundNothing` for its own group, `ContactNotThere` for its own contact. The enemy sees nothing: it never learns a strike came.

### Escorts against the patrol

If the raid has escorts (with more than 0 aircraft) and meets a patrol (a share above 0), the escorts fire at the patrol. Each escort is one shot. The shots are the raid's escort count, whatever the share. The chance is `air_combat.toml: fighter_hit_chance` times the protection factors of the target (see "Protection"), where the target type is that of the first group in the patrol pool: the lowest-numbered patrol group over the hex.

Each shot is one draw from a stream keyed on (the first escort group, the target: the task force attacked, or the base). A hit is an aircraft lost. The losses are not taken in this raid's turn: the hits of all the raids' escorts are added up per patrol and taken, lowest patrol group first, when the rolls are recorded, each group down to 0. Hits beyond the patrol's aircraft are wasted. The patrol's pool is its aircraft at the start of the step. Records `AirFight` (firer: first escort group, target: first patrol group, firing escorts, at patrol, shots, chance, expected, hits). [0027, 0029, 0033]

A side sees: `AirFight` for its own escorts as the firer, cut to the shots and without the group fired at (decision 0065). The defender sees its own `AircraftLost` and `PilotsLost` only. It does not see the escorts' shots or hits.

### The patrol against the escorts and the bombers

If the raid meets a patrol, the patrol fires from one stream keyed on (the first patrol group over the hex, the raid's first strike group). Its shots are split between the escorts and the bombers. With P the raid's share, E its escorts and B its bombers at the start of the step, the escorts draw off D = the smaller of P and E, times `air_combat.toml: patrol_drawn_off_share`, cut to a whole number. The rest, R = P − D, is split in proportion between the escorts not drawn off and the bombers: X = R × (E − D) / (E − D + B), cut to a whole number, and never more than E − D, so no more shots go at the escorts than there are escorts left. It fires first at the escorts and then at the bombers, each roll a shot per fighter:

1. At the escorts: shots = D + X. Chance: `fighter_hit_chance` times the protection factors of the first escort group's type. Hits come off the escorts, lowest group first, no more than they have. Skipped if the raid has no escorts or the shots are 0.
2. At the bombers: shots = R − X. Skipped if 0. Chance: `fighter_hit_chance` times the protection factors of the first strike group's type. Hits come off the raid's strike groups, lowest group first, no more than they have.

The shots at the bombers are split by `air_combat.toml: patrol_before_attack_share`: the first k = the shots times the share, cut to a whole number, are worked before the attack, and the others after it, in order from the one stream, so the draws and the patrol's hits are the same at every share. The hits before the attack come off the bombers before they attack, and the guns then fire if any bomber is left. The hits after come off the bombers that are left, after their attack is worked out, and whatever the attack did stands. The roll at the escorts is not split. A part with no shots records nothing. The fire after the attack is the same `AirFight` with `after_attack` set, recorded in step 4a before the guns' fire after the attack; it cites the raid's launch and the `AirAttack` or `BaseAttacked` events of its groups. When the share is below 1 the breakdown of each part also names the shots at the bombers in all and the share. [0087]

At a share of 1 the escorts draw the patrol off the bombers one for one, and a raid with no escorts takes its whole share: the rule as it was before decision 0080. Records `AirFight` for each roll (firer: first patrol group, target: first escort or first strike group, firing patrol, at escorts or bombers). The breakdown of a patrol roll lists, first, the whole patrol, the raid's aircraft, the aircraft of all the raids, and the raid's share; when the share drawn off is below 1, also that share, the fighters drawn off (D) and the rest's shots at the escorts (X); then the shots and chance. [0027, 0029, 0033, 0080]

A side sees: `AirFight` for its own patrol, cut to the shots and whether it was after the attack, without the group fired at (decisions 0065, 0087). The attacker sees only its own `AircraftLost`.

### Anti-aircraft fire

The guns of the target fire at the bombers that are left (the strike groups' aircraft after the patrol's hits). Skipped if there are no bombers left or the guns have no shots. Their shots, n, are split by `air_combat.toml: anti_aircraft_before_attack_share`: the first k = n times the share, cut to a whole number, are worked before the attack, and the other n − k after it. Both parts are rolled in order from the one stream below, one draw for each shot, so the draws, the guns' hits in all and each group's losses are the same at every share. The hits of the fire before the attack come off the bombers before they attack. The hits of the fire after come off the bombers the fire before left, lowest group first, after their attack is worked out, and whatever the attack did stands. A part with no shots records nothing, so at a share of 1 the guns fire as they did before the rule. The fire after the attack is the same event with `after_attack` set, recorded in step 4a of the air combat step; it cites the raid's launch and the `AirAttack` or `BaseAttacked` events of the raid's groups, and its `AircraftLost` cite it. When the share is below 1 each part's breakdown also names the guns' shots in all and the share. [0079]

- Raid on ships: the shots are the sum of the `anti_aircraft` of the class of every ship afloat in the task force attacked. Damage does not reduce them. The stream is keyed on (the lowest-numbered ship afloat in that force, the raid's first strike group). One roll covers every ship. Records `AntiAircraftFire`.
- Raid on a base: the shots are the base's `anti_aircraft`, a figure of the base's entry in the scenario file (0 if not given). The stream is keyed on (the base, the raid's first strike group). Records `BaseAntiAircraftFire`. Ships in the hex do not fire at a raid on the base, and the base does not fire at a raid on ships.

The chance is `air_combat.toml: anti_aircraft_hit_chance` times the protection factors of the first strike group's type. Hits come off the strike groups, lowest first, no more than they have. There is no ammunition limit. [0027, 0029, 0034, 0033]

A side sees: `AntiAircraftFire` or `BaseAntiAircraftFire` for its own ships or base, cut to the shots and whether it was after the attack, without the raid's group (decisions 0065, 0079). The attacker sees its own `AircraftLost`.

### Each shot is rolled

Every roll of fighters, guns, bombs and torpedoes has the form of chapter 4, Each shot is rolled: one draw for each whole shot, and one more for a part of a shot at the chance times the part. The event gives the shots, the chance, the expected hits (shots times chance) and the hits. The hits of a roll are never capped by the draws: a cap by the aircraft there happens after, when the hits are taken from a pool. [0033]

### Protection

A shot at aircraft has a chance made in this order, each step cut: the base chance; times `air_combat.toml: pilot_armour_factor` if the target type has pilot armour; times `self_sealing_tanks_factor` if it has self-sealing tanks. The target type is the type of the first group of the pool the hits come from. A pool that holds several types takes its first group's. The breakdown lists the base chance and each factor applied. [0029, 0033]

### Aircraft lost

Hits are taken from the pool: patrol, escorts or bombers, lowest group number first, each group down to 0 before the next. An aircraft lost is removed from the group's `ready` aircraft, never more than it has. `AircraftLost` (group, aircraft, remaining) is recorded, caused by the firing event.

Pilots, where counted:

- An aircraft lost as a bomber or an escort takes its pilot with it, never below 0. `PilotsLost` is recorded right after, caused by `AircraftLost`.
- An aircraft lost as the patrol over its own hex draws for its pilot, where the group's pilots are counted: one draw for each aircraft lost, recovered with `air_combat.toml: crew_recovered_over_own_base` for a group based at a base, and with `crew_recovered_beside_own_force` for a group aboard a ship. Each pilot not recovered is a pilot lost, never below 0, and `PilotsLost` is recorded right after the `AircraftLost`, caused by it, when any is lost. The draws are one stream of the air combat subsystem with its own value of `Draw` (`PatrolPilot`, code 3), keyed on the patrol group and the group of escorts whose fire took the loss, so each firing roll's losses to a patrol group have a stream of their own; they are taken in the order the losses come off the groups. The `AircraftLost` of such a loss carries a breakdown, when breakdowns are recorded, of the aircraft lost, the chance and the number of crews recovered, whether or not a pilot was lost. A group whose pilots are not counted draws nothing. A patrol aircraft lost is the only loss that draws here: only escorts' fire takes a patrol's aircraft. [0038, 0059]
- An aircraft written off after a sortie, or destroyed on the ground, loses no pilot.
- An aircraft that ran out of fuel on the way home (see "Running out of fuel") draws for its crew, and a crew not recovered is a pilot lost.

An aircraft that has been shot down is not worn afterwards: it is removed from the sortie that the wear step rolls. A group whose aircraft were all shot down still flew, for fatigue and supply. [0027, 0035, 0038]

A side sees: `AircraftLost` and `PilotsLost` for its own groups, whole. It never sees the enemy's losses. What it learns of the enemy's is the claim.

### The attack on a ship

Each strike group with bombers left attacks, in group order, each with what it has left. The attack is worked out from one stream keyed on (the air group, the task force attacked), in the air combat subsystem.

**The target ship.** All the aircraft of the group attack one ship: the ship afloat in the task force that ranks first by the group's own target list, and on a tie the lowest ship number. A group ranks ships by its target list first, in the list's order, and then by the kinds not on its list, in the order of importance. So a group sent at a "carrier" that turns out to be a cruiser attacks the cruiser: a strike that has flown attacks what it finds. The force is the one the raid found, the same for every strike group in the raid, but each group picks its own ship in it, so two groups of one raid may attack different ships. With the default list every group attacks the most important ship afloat in the force, as before target lists. Ranks are worked on the ships afloat at the start of the step.

Which ship each strike group attacks is made at the point of contact from the true ships of the force, and a side cannot ask it. [0100]

**The weapon and its chance.** A type's `attack_weapon` sets the chance: `air_combat.toml: bomb_hit_chance` for a bomb, `torpedo_hit_chance` for a torpedo. Each aircraft is one shot, and each shot is rolled.

**What a hit does.** A hit is the hit of chapter 4 (Where a hit lands, Damage). Only these differ:

- The damage points are 100 times the aircraft type's `attack_damage` over the target class's `toughness`, in one multiplication and one division, cut. A gun or torpedo hit in a surface fight uses the class's damage figure in the same formula.
- A bomb draws its track as a gun does, against `surface_combat.toml: gun_hit_flotation`. A torpedo lands on flotation and draws no track.
- The system draw is made for every hit, even at a chance of 0. Its bands are in the order given in chapter 4, Where a hit lands, and use `air_combat.toml: bomb_system_hit` for a bomb, which names the flight deck, and `surface_combat.toml: torpedo_system_hit` for a torpedo. The chance of a system hit per point is the same figure for every weapon.
- The hits carry no round number.

The attack's draws, in order, are one per shot, then per hit its track (a bomb only) and its system, then the sighting report (chapter 3, Reports from a strike). [0026, 0033]

**Applying the damage.** In step 5 of the air combat step the hits go through the damage path of chapter 4 (Applying damage): one `ShipDamaged` per track that took hits, then one `SystemHit` per system hit, each caused by the `AirAttack` event. A ship whose flotation reaches 100 sinks, and its groups are lost (see "A group lost with its ship"). A strike group's hits land in group order, so a later group's hits can land on a ship an earlier group sank in the same step: its damage and system events are recorded and no second sinking happens. [0026, 0033]

**The flight deck.** A deck that is out stops all launches from it (see "What a group can fly in a pulse"). Aircraft that are in the air when their deck is lost, by a sinking or by a hit that puts it out, come home to another deck, a base or the sea (see "Coming home to no deck"). [0026, 0027, 0059]

`AirAttack` is recorded (group, task force, ship, weapon, shots, chance, expected, hits), caused by the strike's launch and by every `AircraftLost` event of that group in the raid.

A side sees: the attacker sees `AirAttack` for its own group, cut to the aircraft and the weapon, without the ship or its force (decision 0065). The ship attacked sees an `AirAttack` against it, cut to the task force, the ship and the shots (one shot for each aircraft that attacked), without the attacking group. It sees the damage to its own ships (`ShipDamaged`, `SystemHit`, `ShipSunk`), whole.

### The attack on a base

Each strike group with bombers left attacks, in group order. The stream is keyed on (the air group, the base). Each bomber drops one bomb at `air_combat.toml: bomb_hit_chance`, whatever weapon its type carries against ships: a torpedo bomber sent at an airfield carries bombs. The size of the bomb does not matter. Each shot is rolled.

Then one draw for each hit decides where it lands, in the order: below `air_combat.toml: [base_hit] aircraft` it lands on aircraft on the ground; else below that plus `[base_hit] supply` it lands on supply; else below that plus `[base_hit] men` it lands on men; else it lands on nothing.

The share for men (`air_combat.toml: [base_hit] men`, `0.2`) counts only if the ground figures are loaded with `men_lost_per_base_hit` above 0 and the base's side has men ashore in the base's hex at the start of the attack. Otherwise it is 0 and the share lands on nothing, so a raid on a base with no men ashore draws and lands its hits as it did before decision 0092. A file that gives no `[base_hit] men` gives 0. The three shares add to 1 at most, and the loader refuses a file in which they add to more. Shells land by the same table (chapter 4, Shore bombardment). The raid's breakdown names the factor `hit_on_men_chance` only when the share counts, so a raid with no men ashore reads exactly as before. [0092]

`BaseAttacked` is recorded (group, base, shots, chance, expected, hits, hits on aircraft, hits on supply, hits on men), caused by the strike's launch and the group's `AircraftLost` events. [0034, 0092]

**Aircraft on the ground.** At the start of the step, for each attacked base, a pool is made. For each group located at the base, in group number order: its damaged aircraft first (whatever the group is doing), then its ready aircraft less the aircraft flying in the pulse. The aircraft flying in the pulse are: for a search group with a search arc and light, the aircraft it is able to fly; for each strike, the aircraft launched; and for a fighter group, its patrol and the escorts that go. So the aircraft a tired group leaves behind, the fighters that neither patrol nor escort, a stood-down group, a resting group and a base strike group in the afternoon are all on the ground. A pool made for a base is shared by all the raids on it in the pulse. [0034, 0035, 0036]

**Hits on aircraft.** A strike group's hits on aircraft destroy aircraft from the pool, lowest group first, a group's damaged aircraft before its ready ones, no more than the pool holds and never more than the group has at that moment. Hits beyond the pool are wasted. Each kind of aircraft a group lost is one event: `DamagedAircraftDestroyedOnGround` then `AircraftDestroyedOnGround`, caused by `BaseAttacked`. Pilots are not lost, and a destroyed aircraft does not reduce the group's pilots. Aircraft worn in the pulse are not on the ground in it, because the wear step follows air combat. [0034, 0035, 0036, 0038]

**Hits on supply.** Each hit on supply destroys `air_combat.toml: base_supply_destroyed_per_hit_t` tons, no more than the base has. If any is destroyed, `BaseSupplyDestroyed` is recorded with the tons destroyed and the stock left, caused by `BaseAttacked`. [0034]

**Hits on men.** Each hit on men removes `ground.toml: men_lost_per_base_hit` men from the base's side's land units in the base's hex. `MenLostToFire` is recorded for each unit that lost men, caused by `BaseAttacked` (chapter 7, Men lost to fire). [0092]

**Hits on nothing** change nothing. A side holds no belief about a base's contents, so no report follows. Nothing in the rules damages a base's port or airfield. [0034, 0052]

A side sees: the attacker sees `BaseAttacked` cut to the aircraft and shots. The side that holds the base sees it with the hits and where they landed (on men too) and without the raiding group (decision 0065), and its own `AircraftDestroyedOnGround`, `DamagedAircraftDestroyedOnGround`, `BaseSupplyDestroyed` and `MenLostToFire`. The raider is told nothing of men hit. [0092]

### The report and the claim

**The sighting report.** After a strike group attacks ships, in step 5 and right after its damage lands, the attacking side receives a sighting report of the force attacked. Chapter 3, Reports from a strike, gives what it holds and how its errors are drawn. The event is `ReportReceived`, caused by the `AirAttack`. A strike at a base sends no report. What the report does to the contact the strike was sent at, and whether the group can strike it again, is in chapter 3, Reports from a strike. It reads the force after the damage. [0026, 0012]

**The claim.** Step 6 makes the claims of the pulse's raids, the raiding side's first and then the defending side's, for each raid in order of its first strike group. A raid that found nothing makes none. Chapter 3, The claim after a raid from the air, gives who claims, what a claim counts and how its false counts are drawn. The claim is recorded as `ClaimReceived`, caused by the claimant's own firing events. [0056]

A side sees: its own `ReportReceived` and `ClaimReceived`, whole. It never sees the enemy's.

### Two carrier forces striking each other in one pulse

Both forces' strikes are launched in the decision step before either lands, each on its own belief. Each raid is worked out on the state at the start of the step. A carrier hit in this pulse still sends its strike, because all attacks are worked out before any damage is applied. In group order damage lands, and the sighting reports follow. The two forces' fights happen in the same step. No rule is specific to this case. [0026, 0027, 0033]

## The end of a daylight pulse for aircraft

The wear phase runs in each daylight pulse after air combat. It works the sorties noted in the pulse, in this order: running out of fuel, coming home to no deck, wear, fatigue, then the supply for sorties.

### What counts as a sortie

When any of the wear, fatigue, air supply or air combat figures are loaded, the search step and the air combat step note the sorties of the pulse. Each notes the group, the kind, the aircraft that flew (`sent`), the aircraft that came back (`sent` less those shot down), the distance, and the events that say it flew.

| Kind | Aircraft | Distance |
| --- | --- | --- |
| Search | Aircraft that searched | Twice the range of the group's search arc |
| Strike (at a contact or a base) | Aircraft launched | Twice the air distance from the group's hex to the hex it was sent to |
| Patrol | The group's patrol (all it can fly where its escorts do not go), noted in every daylight pulse, raid or not | 0 |
| Escort | Escorts that went | Twice the air distance from the group's hex to the raid's hex |

A fighter group with a patrol and escorts has two sorties. A patrol flies in every daylight pulse, whether a raid comes or not. Escorts fly only when sent. Nothing flies where it is dark. [0035, 0036, 0039]

### Running out of fuel

The first part of the wear phase, before any wear is rolled. It runs if `air_combat.toml` is loaded, whether or not the wear figures are. It works the strike, base strike and escort sorties of the pulse in the order the wear step uses: order of air group, a group's sorties in the order noted. A sortie is rolled only if at least one aircraft came back from it and its group has a position (a group aboard a ship that sank in the pulse has none: it rolls nothing, as in wear, and its aircraft are lost with the ship). A search and a patrol never roll.

For each such sortie, with d its one-way distance (half the distance the sortie is noted at, in whole miles) and R the strike radius of the group's type:

1. If d is not above R, nothing is rolled and nothing is recorded. A group that flew within its radius never ditches.
2. The share beyond the radius is (d less R) over R, cut to millionths. The chance is `air_combat.toml: ditch_chance_per_radius_beyond` times the share, cut, and then 1 at most. The figure may be more than 1. With R 250 and d 280 the share is 0.12, and with the figure 2 the chance is 0.24. Each type is measured against its own radius, so fighters, with the shorter radius, are further beyond theirs on the same strike. No rule singles them out.
3. Each aircraft that came back (those sent less those shot down) is rolled on its own at the chance, in the form of chapter 4, Each shot is rolled. An aircraft that fails is **ditched**: it leaves `ready` for good. It also leaves the sortie, so that the wear step below never rolls it. The sortie still counts as flown, on the aircraft sent, for fatigue and for supply.
4. Where the group's pilots are counted, each ditched aircraft then draws for its crew at `air_combat.toml: crew_recovered_short_of_home`. Each crew not recovered is one pilot lost, never below 0, and `PilotsLost` is recorded right after the sortie's event, caused by it, when any is lost. A group whose pilots are not counted draws no crew.

All the draws of a group in a pulse come from one stream of the upkeep subsystem with its own value of `Draw` (`Ditch`, code 2), keyed on the air group, and no other. The stream is taken across the group's sorties in order. For each sortie the ditching draws come first, one for each aircraft that came back, and then the crew draws, one for each aircraft ditched. This stream is not the one wear uses, so a ditching changes no wear draw but the number of aircraft wear rolls.

`AircraftDitched` is recorded for each sortie that rolled, whether or not an aircraft ditched (group, kind of sortie, one-way distance, radius, aircraft rolled, chance, expected, ditched, crews recovered, pilots lost, ready after), caused by the events that say the group flew. Its breakdown lists the aircraft rolled, the distance, the radius, the share beyond the radius, the figure per radius, the chance and the crew figure. For the enemy's `aircraft_destroyed` objective a ditched aircraft counts as destroyed, though no one claims it: no claim counts it (chapter 6, The five kinds). [0059]

A side sees: `AircraftDitched` for its own groups, whole, and `PilotsLost`. The enemy sees nothing of it. [0047, 0059]

Not built: a landing at another deck or a field instead of the sea when the deck is lost, and a different chance for the crew by where the aircraft comes down (decision 0059, stage 2).

### Coming home to no deck

The second part of the wear phase, after running out of fuel and before wear. It runs if `air_combat.toml` is loaded. A group's aircraft are in the air if they flew in the pulse (a search, strike, patrol or escort) and were not shot down or ditched for fuel: they are what its noted sorties still hold. Their own deck cannot take them if the group is aboard a ship that has sunk in the pulse, or whose flight deck is out at this moment. A degraded deck still lands them: only a deck that is out, or a ship that has sunk, does not. A group at a base is never affected: no rule damages a field.

Groups are worked in group number order, each group's aircraft as one body that may be split by room, in this order:

1. **Another carrier of the group's side in the same hex**: the hex of the group's ship, or where it sank (noted when she sank). She must be afloat with her flight deck not out, in any task force of her side; ships are taken in ship number order, lowest first. She takes as many as fit under her aircraft capacity, the ready and damaged aircraft of the groups aboard her counted at that moment, and the rest go on to the next such carrier.
2. **A base of the group's side with an airfield (chapter 7, Units that work and what they build) within the aircraft type's strike radius of that hex**, by air distance (the radius, not the reach). For a type with no strike radius (some scout types have none), the search range stands for it. The nearest base, then the lowest number. A base takes any number.
3. **The sea.** The aircraft left are ditched and `AircraftDitchedNoDeck` is recorded. Where the group's pilots are counted, each crew is recovered with `air_combat.toml: crew_recovered_beside_own_force` if any ship of the group's side is afloat in that hex at that moment (the group's own ship counts if she is afloat with her deck out), and otherwise with `crew_recovered_open_sea`. A crew not recovered is a pilot lost, never below 0, with `PilotsLost` caused by the ditching event. The draws are one stream of the upkeep subsystem with its own value of `Draw` (`NoDeck`, code 4), keyed on the source group, one draw for each aircraft ditched, taken after the group's landings. No other draw is made: the choice of a place is not random.

A side can ask this choice for its own air groups before it is made (`measure`'s `choices`, `landing`): the answer is where a group's aircraft would land now if its deck were gone, from the side's carriers in the hex and its bases in reach. [0100]

Aircraft that land join the lowest-numbered group of their aircraft type and side located at that ship or base, whose ready aircraft and strength rise by them (and its pilots, if they are tracked); if there is none, a new group is formed there: the next air group number of its side's series (the Allied side's from 1,000,001, the Japanese side's from 2,000,001; units made in play are numbered by side, so the number depends on its own side's history only), the same aircraft type and side, a name made of the old one, "landed at" and the place (for example "Placeholder dive bombers landed at ship:6"), mission rest, reach normal, no search arc, the source group's fatigue, strength equal to what it received, and pilots counted if the source group's are, one with each aircraft. The source group keeps what it has aboard, its orders and its number; it loses the aircraft and, with each, a pilot. A sortie goes with its aircraft to the group that has them, so wear, fatigue and supply are worked for that group. `AircraftLandedElsewhere` is recorded for each landing (from group, to group, the ship or base, aircraft, pilots, whether the group was formed), and `AircraftDitchedNoDeck` for the ditching (group, hex, aircraft, whether beside own ships, the chance, crews recovered, pilots lost, ready after, with a breakdown of the aircraft, the chance and whether a ship was in the hex). Both are caused by the event that closed the deck (the `ShipSunk`, or the `SystemHit` that put the deck out) and by the events that say the group flew. No carrier ends a pulse over her aircraft capacity. A ditched aircraft counts for the enemy's `aircraft_destroyed` objective; an aircraft that lands elsewhere does not. [0059, 0086]

A side sees: `AircraftLandedElsewhere` and `AircraftDitchedNoDeck` for its own groups, whole, and the `PilotsLost` that follows. The enemy sees nothing of them. [0047, 0059]

Not built: a landing planned at a field beyond the target, a deck that takes aircraft by night or in the dark, aircraft pushed over the side to clear a deck, and rescue of a pilot beside ships of the other side. A ship's deck that goes out in a surface fight while aircraft are in the air is not met: the pulse's flying is over by then.

### Wear

If `air_upkeep.toml` is loaded, each sortie is rolled in order of air group (a group's patrol before its escorts). The aircraft that rolled are those that came back and did not ditch. A sortie with no aircraft back rolls nothing. A group aboard a sunk ship has no position and rolls nothing.

1. The wear chance is `air_upkeep.toml: wear_base_chance` plus `wear_chance_per_100_nm` times the distance over 100, 1 at most. The distance term is one multiplication and division, cut.
2. Each returning aircraft is rolled on its own at the wear chance. The worn leave `ready`.
3. Each worn aircraft is rolled again at `write_off_share`. A written-off aircraft is lost for good: the operational loss. The rest join `damaged`.

All draws come from one stream of the upkeep subsystem keyed on the air group, taken in order across the group's sorties of the pulse. `AircraftWorn` is recorded for each sortie that rolled, whether or not any aircraft was worn (group, kind, distance, aircraft, chance, expected, worn, written off, ready after, damaged after), caused by the events that say the group flew. Wear takes no pilot. A written-off aircraft saves its pilot. [0035, 0038]

A side sees: `AircraftWorn` for its own groups, whole.

### Damaged aircraft

A damaged aircraft does not fly. It is on the ground at its base whatever its group is doing, so a bomb hit on aircraft can destroy it, and it is destroyed before the group's ready aircraft. It counts toward a ship's aircraft capacity. It costs its base the daily supply. It sinks with its ship. Damaged aircraft are repaired by the day (see "Repair"). Aircraft damaged in combat that get home are not modelled apart: a fighter's or a gun's hit shoots an aircraft down, and the damage that remains is the wear. [0035]

### Fatigue gained and lost

If the fatigue figures are loaded, after wear, each group with a position, in order of group number, is worked.

- A group that flew an aircraft in the pulse (a sortie with aircraft `sent`, whether or not any came back) gains: `fatigue.toml: air_fatigue_per_sortie` plus `air_fatigue_per_100_nm` times the distance of its longest sortie over 100, once, never above 100. For equal distances the first sortie counts. A fighter group that kept a patrol and sent escorts gains once, for the longer of the two.
- A group that did not fly in the pulse loses `air_rest_per_pulse`, never below 0.

`AirGroupFatigue` (gained or rested, the amount, the new fatigue) is recorded only when the fatigue changes, caused by the events that say the group flew, or for a group that rested, its latest `AirGroupStoodDown` if it is stood down, else its mission order. Nothing changes at night. So a group that flies one long sortie in the morning gets part back in the afternoon, and a group that flies both pulses gets nothing back. No random number. [0036]

A side sees: `AirGroupFatigue` for its own groups, whole.

### What fatigue cuts

Fatigue cuts the aircraft a group is able to fly, as limit 5 of "The order of the limits" says, and does nothing else. It changes no chance to hit and no chance of wear. It cuts search, strike, escort and patrol alike. Fatigue gained in the morning cuts what flies in the afternoon. [0036]

Not built: crew experience, morale, rest better at a well-found base, weather (0036, 0052).

### Supply for sorties

If `air_supply.toml` is loaded, after wear and fatigue, each base, in base order, pays once for all the sorties flown from it in the pulse. For each sortie of a group located at the base with aircraft `sent` above 0, the cost is the cost of one aircraft's sortie times `sent`, where the cost of one aircraft's sortie is `sortie_supply_t` plus `sortie_supply_t_per_100_nm` times the distance over 100. It is counted on the aircraft sent, shot down or not. The costs are added and taken from the stock in one payment, as much as the stock covers: a payment the stock cannot cover takes what is left. `BaseSupplySpent` (reason sorties) records the aircraft, the amount asked, the amount paid and the stock left, caused by the events that say its groups flew, with the sorties listed by kind in the breakdown. It is not recorded if the amount asked is 0. A group aboard a ship spends nothing. No random number. [0039]

A side sees: `BaseSupplySpent` for its own bases, whole. The patrol itself has no event of its own with the supply figures alone; the breakdown lists the patrol aircraft.

## The order of the air rules in a day

Chapter 1, The night pulse and The morning and afternoon pulses, hold the sequence. The air rules in that order:

1. Before the pulses, at orders intake: a rebase order is applied (Rebasing an air group; chapter 1, Orders).
2. At the start of every pulse: the grounded bases are fixed (A base out of supply).
3. In the night pulse, first: repair, arrivals, replacements (from day 2), daily supply, the men's supply (chapter 7), stand-down and return.
4. In each daylight pulse: air search (chapter 3), the strike decision, air combat, then the wear phase, which runs running out of fuel, then coming home to no deck, then wear, then fatigue, then supply for sorties, and then movement (chapter 2).

Contacts age at the end of every pulse (chapter 3). The end-of-day phases do nothing for aircraft. [0035, 0036, 0039, 0041, 0059, 0091]

## Known limits

- **A hard reach.** A strike group launches only if its target is within its reach, the type's strike radius or, on extended reach, a share more. There is no partial effort or reduced load. A group on extended reach pays for a sortie beyond its radius only in aircraft that run out of fuel on the way home. There is no daylight limit on a launch and no landing at a field beyond the target. [0030, 0059]
- **One hex.** A strike flies to the hex its contact names and attacks only there. It does not search neighbouring hexes. [0026, 0028]
- **One ship.** All aircraft of a group attack one ship. A strike is not spread, and it attacks nothing but that ship, which is the one its own target list ranks first in the force the raid found. [0026, 0063]
- **Target lists are standing.** An order given after a contact is seen does not send a strike at it that day: search, decision and strike all happen inside the day. A list cannot hold back a share of a group, split a strike among several ships, name a contact, or hold by the size of a force. [0063]
- **No night strikes.** [0026, 0034]
- **Bases.** A base strike launches once a day, in the first daylight pulse in which the base is in reach. A bomb's size does not matter. No runway or port damage. No damage assessment for the raider except a claim. [0034, 0052, 0063]
- **Rebasing is whole and by order.** A group at a base moves by a rebase order, the whole group at once, to a base with an airfield within its transfer range, and a group aboard a ship cannot be rebased or go aboard. A base takes any number of aircraft. There is no loss on the flight and no enemy patrol meets it. A base beyond the range is reached by two orders on two days. [0052, 0091]
- **Landing elsewhere is by deck loss only.** Aircraft in the air when their own deck is lost land at another carrier or a base, or come down in the sea; no group is ordered to move by this rule, and a landing plans no field beyond the target. Aircraft aboard a ship that sinks are lost with her, with their pilots, though most of those men were really saved: a saved pilot with no aircraft has nowhere to go, and there is no roster of pilots. [0025, 0059]
- **Pilots are a count.** No experience, no skills, no pilot wounded. A pilot shot down away from home is not rescued. The crew of an aircraft that ran out of fuel on the way home may be, at one chance whatever the place it came down, and the pilot of a patrol aircraft shot down over its own hex may be, at a chance for a base and one for ships. [0038, 0052, 0059]
- **Arrivals and capacity.** An arrival to a group aboard a ship is not tested against the ship's aircraft capacity. An over-capacity ship is found by the end-of-day check, which stops the run. To change: differences.md row 27, a scenario whose arrival would overfill a ship is to be refused when it loads.
- **Aircraft in reserve, morale, a base's aviation support, ground support, weather** are not built. [0052]
- **Anti-aircraft ammunition** is not limited. [0027]
- **Repair order** is lowest group first and cannot be changed. [0052]

## Not in this chapter

- Orders in general, the order intake, and the rejection reasons for task forces and air groups: chapter 1, Orders.
- Movement, fuel and cargo: chapter 2.
- Air search, the sighting chance, the report error model, contacts and their ageing, claims in belief, and a force's readiness, alert and alarm: chapter 3.
- Damage points, flotation, fire, systems and sinking, and break-off: chapter 4.
- What a side is shown of an event, the state hash, random stream keys and replay: chapter 1.
- The briefing, `explain` and `measure`: not rules.
- Objectives and results: chapter 6.
