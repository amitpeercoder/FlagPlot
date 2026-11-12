# Air operations

This chapter is the rules for air groups: what they are, what they are ordered to do, how many aircraft can fly, how a strike is decided and fought, and what the flying costs afterwards. Quantities are fixed-point numbers, cut as chapter 1, Numbers and chance, says. A figure named as `file.toml: key` is read from that file in `data/reference`. A figure with no file is written into the code, and the file it is in is named.

A day has three pulses: night, morning and afternoon. The night pulse comes first. Air groups do nothing but rest, repair and receive aircraft at night: nothing flies in the dark (decision 0001, 0012). The air rules in the order they run are listed in "The order of the air rules in a day", and chapter 1, The order in which a day resolves, holds the sequence of the whole day.

Several rules apply only when a scenario loads the figures for them. Where that matters it is stated with the rule.

## The air group

### What an air group is

An air group is one unit of aircraft of one aircraft type. It holds:

- `ready`: aircraft fit to fly.
- `damaged`: aircraft that came back worn and cannot fly until repaired. Every count of what a group can fly counts `ready` only.
- `pilots`: a count, or none. A group whose scenario gives it no pilots does not track them. One pilot stands for the crew of one aircraft.
- `strength`: the ceiling for replacements, in aircraft (ready and damaged together) and in pilots. It is what the group started with, unless the scenario gives another figure. Arrivals raise it. Nothing else reads it.
- `mission`, and the order that set it.
- `search`: a search arc (from, to, range), and the order that set it.
- `patrol_share`: from 0 to 1, default 1, and the order that set it.
- `fatigue`: from 0 to 100, default 0.
- `rest_above`: the rest threshold, or none, and the order that set it.
- `stood_down`: whether the staff stood the group down today.
- Its place: exactly one base or exactly one ship.

[0025, 0035, 0036, 0038, 0041]

A side sees: its own groups whole. The enemy sees nothing of a group except through a sighting report, a claim, or the fight events listed with each rule below.

### Where a group is

A group at a base is at the base's hex. A group aboard a ship is at the position of the ship's task force, and moves with it with no order. A group aboard a ship that is in a port is at the port's hex, and is still aboard the ship. A group aboard a ship that has sunk has no position. A group cannot change between a ship and a base, or between bases: there is no rebase order. A group with no aircraft is at its place all along and can be given orders. [0025, 0041, 0052]

A side sees: its own group's place. The enemy sees only what a report says of a force.

### Aircraft capacity of a ship

A ship class has an aircraft capacity: a whole number, 0 for a ship that carries none (`ship_classes*.csv: aircraft_capacity`). The ready and damaged aircraft of all the groups aboard a ship together may not exceed it. A scenario that breaks this does not load. The check is also made at the end of every day, and a breach stops the run with an invariant error. Capacity limits nothing else: arrivals are not tested against it (see "Known limits"). Any aircraft type may be based on any ship with the capacity. [0025, 0035]

A side sees: its own capacity and aircraft count. The enemy does not.

### A group lost with its ship

When a ship sinks, every group aboard it loses all its ready and damaged aircraft and, if its pilots are tracked, all its pilots. The group stays in the state, aboard the sunk ship, with no aircraft and no position. It records `AirGroupLost` (caused by the `ShipSunk` event) and, if it had pilots, `PilotsLost` (caused by `AirGroupLost`). An order to such a group is rejected with `AirGroupLost`. A group aboard a ship that sinks in the pulse still attacks in that pulse, because every attack is worked out before any damage lands. Landing aboard another carrier or ashore is not modelled. [0025, 0026, 0035, 0038, 0041]

A side sees: `AirGroupLost` and `PilotsLost` for its own groups, whole. The enemy sees nothing of them, and learns the ship sank only through a claim or a report.

## Missions and orders

### The five missions

A group has one mission at a time. It flies only that mission. A group takes up its mission only if the other rules of this chapter let it fly. A new group's mission is rest. [0026, 0027, 0034]

| Mission | What the group does |
| --- | --- |
| Rest | Stays on the ground. Flies nothing. |
| Search | Flies its search arc in each daylight pulse, in the search step (chapter 3, Air search). |
| Naval strike | Decides in each daylight pulse whether to strike a contact its side holds. |
| Base strike (names an enemy base) | Strikes that base in the morning pulse. |
| Combat air patrol | Keeps a share of its aircraft over its own hex, and escorts raids with the rest. |

The group's mission order, if the mission was set by an order, is a cause of every event that group's flying produces. A group on a strike mission does not search. A group on search does not strike or patrol. A group on patrol does not strike. [0026, 0027, 0034]

A side sees: its own orders and what they cause, whole. Nothing of the enemy's missions.

### Air group orders

The orders that act on an air group, and every reason one is refused, are in chapter 1, Air group orders and Every reason an order is refused. An accepted order becomes the standing order and stays until another of its kind replaces it. What each means to the rules here:

- A mission order sets the mission (the table above).
- A search arc order sets the arc that the search step flies (chapter 3, Air search).
- A patrol share order sets `patrol_share`, which only The split reads. It does nothing for a group that is not on combat air patrol.
- A rest threshold order sets `rest_above`, which only Stand-down and return reads. It does nothing when the fatigue figures are not loaded.

The checks are made against the truth, and a refusal cites only what the side can see (chapter 1). [0026, 0027, 0034, 0036, 0050]

A side sees: `OrderAccepted` or `OrderRejected` for its own orders, whole.

Not built: orders to rebase, to set target priority, to set a range limit other than a search arc's range, and to change the repair order (0052).

## What a group can fly in a pulse

### The order of the limits

Search, strike launch and the split of fighters into patrol and escorts all use one number, the aircraft the group is able to fly. It is worked out, for a group, in this order. [0026, 0036, 0038, 0039]

1. A group at a base that is out of supply at the start of the pulse flies none. A group aboard a ship is never limited this way.
2. Start from the smaller of the group's ready aircraft and its pilots. A group whose pilots are not tracked starts from its ready aircraft.
3. If the group is aboard a ship, multiply by the ship's flight deck factor: 1 if the deck is working, `surface_combat.toml: degraded_factor` if degraded, 0 if out. A group at a base is not multiplied. Without the surface combat figures this step does nothing.
4. If the fatigue figures are loaded: a group that is stood down flies none. Otherwise multiply by (100 minus the group's fatigue) over 100.
5. Cut the result to a whole number once, toward zero. The multiplications of steps 3 and 4 are fixed-point, so no rounding happens before this cut.

The cut is applied once, to the product. A group of 6 aircraft with fatigue 1 and a working deck flies 5. A group aboard a carrier with a degraded deck and fatigue 20, 10 ready and 10 pilots, flies 10 times the degraded factor times 0.8: with a degraded factor of 0.5 (an example, not the figure) that is 4. A group's flight deck is worked group by group, not for the ship. [0026, 0036, 0038, 0039]

A side sees: the number it flew in its own events (`SearchFlown`, `StrikeLaunched`, `EscortsSent`). It is not told the limits that produced it.

### A base out of supply

A base is out of supply at the start of a pulse if its supply stock is exactly zero then, and the air supply figures are loaded (`air_supply.toml`). This is fixed at the start of every pulse, before any step of it. A base bombed to zero in a pulse still flies that pulse. A base that starts a pulse with any stock flies in full. A base with no stock flies nothing in that pulse: its groups keep their missions and their aircraft are on the ground. Nothing else changes for such a base: its guns still fire, its aircraft are not lost, repairs are limited by stock (see "Repair"), and its groups rest as usual. It flies again in the first pulse that starts with stock. [0039]

Not built: rationing a short stock among groups, supply for ships, a carrier's own supply (0039).

A side sees: `BaseSupplySpent` for its own bases. The enemy sees nothing.

### Missions and light

Search, strike decisions and air combat happen in the two daylight pulses only. Strike decision tests `is_dark` for the group's hex, which is true exactly in the night pulse. A group flies no strike by night. [0001, 0012, 0026]

## Air distance and radius

### Air distance

The air distance between two hexes is the straight line between their centres, in whole miles. With dq and dr the differences of the hexes' q and r coordinates, it is 40 times the square root of (dq squared plus dr squared plus dq times dr), rounded to the nearest mile. It is worked in whole numbers. Along an axis of the grid it is 40 times the number of hexes. Every rule, event and tool that says how far an aircraft flies uses this one function (`Hex::air_distance_nm` in `hex.rs`). A distance is within a radius if it is less than or equal to the radius. [0040]

Ships do not use it: a ship steams hex by hex and each step is 40 miles (chapter 2, Legs and position). A threshold counted in hexes, such as how near a contact must be to alert a force, stays in hexes. [0040]

A side sees: the distance in its own events that carry one.

### Radius

A type has a search range (`aircraft_types*.csv: search_range_nm`), a strike radius (`strike_range_nm`) and a sweep width (`sweep_width_nm`). The strike radius is how far out an aircraft goes and comes back, and the same figure is the radius for a strike and for an escort. The derivation sets it from the sourced combat radius where exactly one can be read, and otherwise from a third of the sourced range with a normal load, cut to a whole mile (0029). The search range is the largest range a search arc may name. [0026, 0029, 0030, 0040]

Known limit: the radius is a hard cutoff. A group strikes only if the target is within the radius, and escorts go only if the raid's hex is within their radius. There is no partial effort, no reduced load and no drop-tank order. Decision 0030 lists drop tanks as deferred. The radius of a type with a sourced combat radius and one without are not the same kind of figure (0030).

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

1. Places are listed in order: every base whose `aircraft_repairs_per_day` is above 0, by base number; then, if `air_upkeep.toml: carrier_repairs_per_day` is above 0, every ship afloat that carries at least one group, by ship number. Each place has an allowance: the base's own figure, or the carrier figure for each ship.
2. For each place in order, the groups located there with damaged aircraft are taken lowest group number first. A group's repair is the smaller of the allowance left and its damaged aircraft.
3. At a base, with `air_supply.toml` loaded and `repair_supply_t` above 0, a repair is also limited by stock: the whole number of repairs the stock covers, which is the stock divided by `repair_supply_t`, cut to a whole number. The repair is the smaller of the two. If that is 0 the base stops repairing for the day, and no later group at the base is repaired. A ship pays no supply.
4. Each repaired aircraft moves from `damaged` to `ready`. `AircraftRepaired` is recorded for each group repaired, caused by the group's latest `AircraftWorn` event that left an aircraft damaged. An aircraft damaged from the start has no cause.
5. At a base that paid for repairs, one `BaseSupplySpent` (reason repairs) is recorded after the base's groups, for the repairs times `repair_supply_t`, caused by that base's `AircraftRepaired` events. The stock is taken at each repair, so the payment is whole.

Repair is by the day and by place. It uses no random number. A carrier's allowance is per ship. [0035, 0039]

Not built: an order to change which group is repaired first (0052), repair by type, small airfields that wear aircraft faster (0035).

A side sees: `AircraftRepaired` and `BaseSupplySpent` for its own groups and bases, whole. The enemy sees nothing.

### Aircraft that arrive on a day

A scenario may list arrivals: a day, an air group, a number of aircraft and a number of pilots. After repair and before replacements, on every day including day 1, each arrival due that day, in the order listed, is delivered:

1. If the group does not exist, or is aboard a ship that has sunk, nothing happens and nothing is recorded.
2. The group's `ready` and `strength` each rise by the arrival's aircraft.
3. If the group's pilots are tracked, its pilots rise by the arrival's pilots. If not, the arrival's pilots are dropped.
4. `AircraftArrived` is recorded, with no cause (nothing orders it), giving the arrival, the group's ready aircraft, pilots and strength after it.

Arriving aircraft are ready. The flight in costs nothing: no wear, fatigue or supply, and nothing can intercept it. An arrival on day 1 is the same as starting with the aircraft. No random number. [0041]

A side sees: `AircraftArrived` for its own groups, whole.

### Replacements

A scenario may give a side replacements at a base: for one aircraft type, a number of aircraft a day and a number of pilots a day. From day 2 on, after arrivals, each stream is worked in the order given:

1. If the base is not still held by the stream's side, nothing happens.
2. The groups located at that base, of that side, that fly that type are taken lowest group number first.
3. For each, the aircraft it receives are the smaller of the aircraft left in the stream and its `strength` less its ready and damaged aircraft (zero if that is negative). The pilots it receives, if its pilots are tracked, are the smaller of the pilots left in the stream and its strength less its pilots. A group whose pilots are not tracked takes aircraft and no pilots, and the day's pilots pass to the next group.
4. A group that receives anything gets the aircraft as ready and the pilots added. `ReplacementsReceived` is recorded for it, with no cause.
5. What no group can take is lost. Nothing is held over to the next day.

Day 1 has nothing to replace. Replacement pilots are as good as the pilots they replace. Replacements cost the base no supply. No random number. [0038, 0039, 0041]

A side sees: `ReplacementsReceived` for its own groups, whole.

### Daily supply

After replacements, each base, in base order, pays `air_supply.toml: daily_supply_t_per_aircraft` for each aircraft located at it, ready and damaged together, as much as its stock covers. The aircraft are counted after the night's repairs and replacements. A base with no stock, or with nothing to pay, pays nothing and records nothing. Otherwise `BaseSupplySpent` (reason upkeep) records the aircraft, the amount asked, the amount paid and the stock left, with no cause. A payment the stock cannot cover takes what is left, and nothing is owed. No random number. [0039]

A side sees: `BaseSupplySpent` for its own bases, whole.

### Stand-down and return

After the daily supply, if the fatigue figures are loaded, each group with a rest threshold is taken in group order. If its fatigue is above its threshold and it is not stood down, it is stood down. If it is stood down and its fatigue is not above its threshold, it is returned. A group whose state does not change records nothing. `AirGroupStoodDown` or `AirGroupReturned` records the fatigue and the threshold, caused by the rest threshold order.

A group with no threshold is never stood down. A stood-down group flies nothing that day: its able-to-fly number is 0, its aircraft are on the ground, and it keeps its mission and takes it up again when it returns. There is one threshold and no second figure for coming back, so a group goes back to flying on the first night its fatigue is at or under the threshold. No random number. [0036]

This is the staff's rest. The player sets the threshold and the staff applies it. [0036]

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
- Its estimated hex is within the type's strike radius of the group's hex, by air distance.

From the contacts that qualify the group takes the one with the most important estimated kind (carrier, battleship, cruiser, destroyer, transport, auxiliary, in that order), then the nearest by air distance, then the lowest contact number. With none it does nothing.

It launches every aircraft it is able to fly. It flies to the hex the contact names, not to where the enemy really is. It records `StrikeLaunched` (group, contact, hex, aircraft, one-way air distance), caused by the group's mission order and by the event of the report that last refreshed the contact. A strike in flight is working data of the pulse. It exists from the decision step to the air combat step and is not part of the state hash. [0026, 0032, 0040]

A side sees: `StrikeLaunched` for its own group, whole. The enemy sees nothing at launch.

### Base strike

A base strike group launches in the morning pulse only, once a day, with no contact and no belief read. It launches if the base exists, is held by the other side, and is within the type's strike radius by air distance. It launches every aircraft it is able to fly and records `BaseStrikeLaunched` (group, base, hex, aircraft, one-way distance), caused by the group's mission order. In the afternoon pulse a base strike group does nothing. [0034]

A side sees: `BaseStrikeLaunched` for its own group, whole.

### A group may fly more than once a day

Nothing in these rules keeps aircraft that flew in the morning from flying in the afternoon, except the losses, wear, fatigue and supply that the morning's flying caused. A naval strike group may launch in both daylight pulses. [0026, 0035, 0036]

## Escorts and patrol

### The split

At the start of the air combat step each fighter group on combat air patrol that has ready aircraft is split. Let F be the aircraft it is able to fly. The patrol is F times the group's patrol share, cut to a whole number. The escorts are F minus the patrol. The patrol stays over the group's own hex. The escorts go with a raid if one is available. A fighter group that is not on combat air patrol is not split and flies nothing. [0027, 0036]

The patrol defends the hex the group is in, against every enemy raid that attacks that side's forces or base there, in a daylight pulse. [0027]

A side sees: nothing of the split itself.

### Raids

The strikes launched in one pulse from one place at one hex are one raid. A place is a ship or a base. The raid's bombers are the aircraft of those strike groups. Strikes at ships and strikes at a base from one place at one hex are two raids; two strikes at a base from one place are one raid only if they name the same base. Raids are ordered by their first strike group, the lowest group number first. A raid with a strike group whose aircraft the fights leave at zero is still a raid. [0027, 0034]

### Which raid the escorts join, and the radius

All the escorts at a place go with one raid from that place: the raid with the most bombers launched, and of those the one whose first group has the lowest number. They look for no nearer raid. The raid is tested for the radius: the escorts go only if the raid's hex is within the fighter type's strike radius of the fighter group's hex, by air distance. A group with no position is not measured and sends its escorts. If the raid is beyond the radius, the whole group that can fly is its patrol for the pulse: its patrol rises by its escorts and its escorts fall to 0. If no raid launches from the place, its escorts fly nothing and are on the ground. The test is made on the hex the raid is sent to, which is where its side believes the enemy is, so a group whose escorts go with a raid to an empty hex has still sent them. [0027, 0030]

`EscortsSent` is recorded for each fighter group whose escorts go (group, the raid's first strike group, aircraft), caused by the group's mission order, its share order, and the raid's `StrikeLaunched` or `BaseStrikeLaunched` event. It is recorded only when escorts go. [0027, 0030]

A side sees: `EscortsSent` for its own group, whole. The enemy sees nothing of it.

### The patrol divided among raids

When more than one raid attacks forces of one side in one hex in a pulse, that side's patrol over the hex is divided among those raids. The patrol is the sum of the patrol aircraft of that side's fighter groups at that hex, over ships and base together. A raid's aircraft are its bombers plus the escorts sent with it. A raid that finds nothing takes no share and does not count. All shares are worked out on the start-of-step counts before any raid is resolved, so the order of raids does not matter.

- A raid's share is the patrol times its aircraft over the aircraft of all the raids there, in whole numbers, cut toward zero.
- The patrol left over, patrol less the shares, goes one fighter each to the raids in order of size, the largest first (aircraft, bombers and escorts), and on a tie the one whose first strike group has the lower number. The shares add up to the patrol.

With one raid the share is the whole patrol. A raid whose share is 0 meets no patrol: its escorts do not fire at the patrol either. [0033]

A side sees: nothing of the division. The factors of the patrol's own fire (below) show the side its own patrol and share.

## The raid, step by step

### The order of the air combat step

The air combat step runs in each daylight pulse after the strike decision. Everything is worked out on the state at the start of the step, then recorded and applied. If no strike was launched the step does nothing except note the patrols for the wear step. In order:

1. Group the strikes into raids, split the fighters, send the escorts, count the aircraft on the ground at attacked bases, note the patrols and strikes for wear, and divide the patrol.
2. Work out every raid on the start-of-step counts, whatever the others do. Nothing is written.
3. For each raid in order of its first strike group, record `EscortsSent` for each group that sends escorts, then each fire roll as an event (`AirFight`, then `AntiAircraftFire` or `BaseAntiAircraftFire`), then take the losses of each roll off the groups and record `AircraftLost` for each group that lost aircraft, and `PilotsLost` where the rule below says.
4. For each strike group in order of group number, record what it found: `StrikeFoundNothing`, `AirAttack` or `BaseAttacked`. A strike group with no bomber left records nothing and sends no report.
5. In the same group order, apply each attack: for ships, apply the damage and send the sighting report; for a base, apply the hits on aircraft and supply; for a strike that found nothing, send the report that nothing is there.
6. Record the claims of the pulse's raids (see "The report and the claim").

Both sides' strikes land in this one step. A group aboard a ship that sinks in step 5 still attacked in step 4. [0026, 0027, 0033, 0034, 0056]

### A strike that finds nothing

A raid's target is found on the truth at the start of the step. For a raid at ships, the target is the enemy task force with the most important ship that is really at the raid's hex, and on a tie the lowest task force number. Position is read before the pulse's movement, because movement follows air combat in the daylight pulse. If no enemy task force is at the hex, the raid finds nothing. It does not search neighbouring hexes. A base strike always finds its base, because the launch checked it exists and is held by the other side.

A raid that finds nothing meets nobody: no escorts fire, no patrol fires, no guns fire, and it takes no share of a patrol. Every strike group in it records `StrikeFoundNothing` (group, hex), caused by the strike's launch. Then, in group order and with no random number, the side's belief is told nothing is there. If the side still holds the contact the strike was sent at and that contact still names that hex, the contact is dropped and `ContactNotThere` is recorded, caused by that group's `StrikeFoundNothing`. If the contact has gone or a later report has moved it, nothing is dropped and nothing is recorded. A dropped contact's number is not used again. A strike group that has no bomber left does not find nothing: it records nothing, and the contact stays. [0026, 0027, 0028]

A side sees: `StrikeFoundNothing` for its own group, `ContactNotThere` for its own contact. The enemy sees nothing: it never learns a strike came.

### Escorts against the patrol

If the raid has escorts (with more than 0 aircraft) and meets a patrol (a share above 0), the escorts fire at the patrol. Each escort is one shot. The shots are the raid's escort count, whatever the share. The chance is `air_combat.toml: fighter_hit_chance` times the protection factors of the target (see "Protection"), where the target type is that of the first group in the patrol pool: the lowest-numbered patrol group over the hex.

Each shot is one draw from a stream keyed on (the first escort group, the target: the task force attacked, or the base). A hit is an aircraft lost. The losses are not taken in this raid's turn: the hits of all the raids' escorts are added up per patrol and taken, lowest patrol group first, when the rolls are recorded, each group down to 0. Hits beyond the patrol's aircraft are wasted. The patrol's pool is its aircraft at the start of the step. Records `AirFight` (firer: first escort group, target: first patrol group, firing escorts, at patrol, shots, chance, expected, hits). [0027, 0029, 0033]

A side sees: `AirFight` for its own escorts as the firer, cut to the shots. The defender sees its own `AircraftLost` and `PilotsLost` only. It does not see the escorts' shots or hits.

### The patrol against the escorts and the bombers

If the raid meets a patrol, the patrol fires from one stream keyed on (the first patrol group over the hex, the raid's first strike group). It fires first at the escorts and then at the bombers, each roll a shot per fighter:

1. At the escorts: shots = the smaller of the raid's share and the escort count. Chance: `fighter_hit_chance` times the protection factors of the first escort group's type. Hits come off the escorts, lowest group first, no more than they have. Skipped if the raid has no escorts.
2. At the bombers: shots = the share less the shots at the escorts. Skipped if 0. Chance: `fighter_hit_chance` times the protection factors of the first strike group's type. Hits come off the raid's strike groups, lowest group first, no more than they have.

So escorts draw the patrol off the bombers one for one, and a raid with no escorts takes its whole share. Records `AirFight` for each roll (firer: first patrol group, target: first escort or first strike group, firing patrol, at escorts or bombers). The breakdown of a patrol roll lists, first, the whole patrol, the raid's aircraft, the aircraft of all the raids, and the raid's share, then the shots and chance. [0027, 0029, 0033]

A side sees: `AirFight` for its own patrol, cut to the shots. The attacker sees only its own `AircraftLost`.

### Anti-aircraft fire

The guns of the target fire at the bombers that are left (the strike groups' aircraft after the patrol's hits). Skipped if there are no bombers left or the guns have no shots.

- Raid on ships: the shots are the sum of the `anti_aircraft` of the class of every ship afloat in the task force attacked. Damage does not reduce them. The stream is keyed on (the lowest-numbered ship afloat in that force, the raid's first strike group). One roll covers every ship. Records `AntiAircraftFire`.
- Raid on a base: the shots are the base's `anti_aircraft`. The stream is keyed on (the base, the raid's first strike group). Records `BaseAntiAircraftFire`. Ships in the hex do not fire at a raid on the base, and the base does not fire at a raid on ships.

The chance is `air_combat.toml: anti_aircraft_hit_chance` times the protection factors of the first strike group's type. Hits come off the strike groups, lowest first, no more than they have. There is no ammunition limit. [0027, 0029, 0034, 0033]

A side sees: `AntiAircraftFire` or `BaseAntiAircraftFire` for its own ships or base, cut to the shots. The attacker sees its own `AircraftLost`.

### Each shot is rolled

Every roll of fighters, guns, bombs and torpedoes has the form of chapter 4, Each shot is rolled: one draw for each whole shot, and one more for a part of a shot at the chance times the part. The event gives the shots, the chance, the expected hits (shots times chance) and the hits. The hits of a roll are never capped by the draws: a cap by the aircraft there happens after, when the hits are taken from a pool. [0033]

### Protection

A shot at aircraft has a chance made in this order, each step cut: the base chance; times `air_combat.toml: pilot_armour_factor` if the target type has pilot armour; times `self_sealing_tanks_factor` if it has self-sealing tanks. The target type is the type of the first group of the pool the hits come from. A pool that holds several types takes its first group's. The breakdown lists the base chance and each factor applied. [0029, 0033]

### Aircraft lost

Hits are taken from the pool: patrol, escorts or bombers, lowest group number first, each group down to 0 before the next. An aircraft lost is taken off the group's `ready` aircraft, never more than it has. `AircraftLost` (group, aircraft, remaining) is recorded, caused by the firing event.

Pilots, where counted:

- An aircraft lost as a bomber or an escort takes its pilot with it, never below 0. `PilotsLost` is recorded right after, caused by `AircraftLost`.
- An aircraft lost as the patrol over its own hex loses no pilot. The group has one aircraft fewer and keeps the pilot. This holds for a carrier's patrol at sea too: rescue at sea is not modelled.
- An aircraft written off after a sortie, or destroyed on the ground, loses no pilot.

An aircraft that has been shot down is not worn afterwards: it is taken off the sortie that the wear step rolls. A group whose aircraft were all shot down still flew, for fatigue and supply. [0027, 0035, 0038]

A side sees: `AircraftLost` and `PilotsLost` for its own groups, whole. It never sees the enemy's losses. What it learns of the enemy's is the claim.

### The attack on a ship

Each strike group with bombers left attacks, in group order, each with what it has left. The attack is worked out from one stream keyed on (the air group, the task force attacked), in the air combat subsystem.

**The target ship.** All the aircraft of the group attack one ship: the most important ship afloat in the task force, and on a tie the lowest ship number. The ship is the one chosen when the raid's target was found, so it is the same for every strike group in the raid.

**The weapon and its chance.** A type's `attack_weapon` sets the chance: `air_combat.toml: bomb_hit_chance` for a bomb, `torpedo_hit_chance` for a torpedo. Each aircraft is one shot, and each shot is rolled.

**What a hit does.** A hit is the hit of chapter 4 (Where a hit lands, Damage). Only these differ:

- The damage points are 100 times the aircraft type's `attack_damage` over the target class's `toughness`, in one multiplication and one division, cut. A gun or torpedo hit in a surface fight uses the class's damage figure in the same formula.
- A bomb draws its track as a gun does, against `surface_combat.toml: gun_hit_flotation`. A torpedo lands on flotation and draws no track.
- The system draw is made for every hit, even at a chance of 0. Its bands use `air_combat.toml: bomb_system_hit` for a bomb, which names the flight deck, and `surface_combat.toml: torpedo_system_hit` for a torpedo. The chance of a system hit per point is the same figure for every weapon.
- The hits carry no round number.

The attack's draws, in order, are one per shot, then per hit its track (a bomb only) and its system, then the sighting report (chapter 3, Reports from a strike). [0026, 0033]

**Applying the damage.** In step 5 of the air combat step the hits go through the damage path of chapter 4 (Applying damage): one `ShipDamaged` per track that took hits, then one `SystemHit` per system hit, each caused by the `AirAttack` event. A ship whose flotation reaches 100 sinks, and its groups are lost (see "A group lost with its ship"). A strike group's hits land in group order, so a later group's hits can land on a ship an earlier group sank in the same step: its damage and system events are recorded and no second sinking happens. [0026, 0033]

**The flight deck.** A deck that is out stops all launches from it (see "What a group can fly in a pulse"). There is no rule for aircraft whose deck is closed while they are away: they are lost only if the ship sinks. [0026, 0027]

`AirAttack` is recorded (group, task force, ship, weapon, shots, chance, expected, hits), caused by the strike's launch and by every `AircraftLost` event of that group in the raid.

A side sees: the attacker sees `AirAttack` for its own group, cut to the aircraft and the weapon. The ship attacked sees an `AirAttack` against it, cut to the group, the task force, the ship and the shots. It sees the damage to its own ships (`ShipDamaged`, `SystemHit`, `ShipSunk`), whole.

### The attack on a base

Each strike group with bombers left attacks, in group order. The stream is keyed on (the air group, the base). Each bomber drops one bomb at `air_combat.toml: bomb_hit_chance`, whatever weapon its type carries against ships: a torpedo bomber sent at an airfield carries bombs. The size of the bomb does not matter. Each shot is rolled.

Then one draw for each hit decides where it lands, in the order: below `air_combat.toml: [base_hit] aircraft` it lands on aircraft on the ground; else below that plus `[base_hit] supply` it lands on supply; else it lands on nothing.

`BaseAttacked` is recorded (group, base, shots, chance, expected, hits, hits on aircraft, hits on supply), caused by the strike's launch and the group's `AircraftLost` events. [0034]

**Aircraft on the ground.** At the start of the step, for each attacked base, a pool is made. For each group located at the base, in group number order: its damaged aircraft first (whatever the group is doing), then its ready aircraft less the aircraft flying in the pulse. The aircraft flying in the pulse are: for a search group with a search arc and light, the aircraft it is able to fly; for each strike, the aircraft launched; and for a fighter group, its patrol and the escorts that go. So the aircraft a tired group leaves behind, the fighters that neither patrol nor escort, a stood-down group, a resting group and a base strike group in the afternoon are all on the ground. A pool made for a base is shared by all the raids on it in the pulse. [0034, 0035, 0036]

**Hits on aircraft.** A strike group's hits on aircraft destroy aircraft from the pool, lowest group first, a group's damaged aircraft before its ready ones, no more than the pool holds and never more than the group has at that moment. Hits beyond the pool are wasted. Each kind of aircraft a group lost is one event: `DamagedAircraftDestroyedOnGround` then `AircraftDestroyedOnGround`, caused by `BaseAttacked`. Pilots are not lost, and a destroyed aircraft does not reduce the group's pilots. Aircraft worn in the pulse are not on the ground in it, because the wear step follows air combat. [0034, 0035, 0036, 0038]

**Hits on supply.** Each hit on supply destroys `air_combat.toml: base_supply_destroyed_per_hit_t` tons, no more than the base has. If any is destroyed, `BaseSupplyDestroyed` is recorded with the tons destroyed and the stock left, caused by `BaseAttacked`. [0034]

**Hits on nothing** change nothing. A side holds no belief about a base's contents, so no report follows. Nothing in the rules damages a base's port or airfield. [0034, 0052]

A side sees: the attacker sees `BaseAttacked` cut to the aircraft and shots. The side that holds the base sees it whole, with the hits and where they landed, and its own `AircraftDestroyedOnGround`, `DamagedAircraftDestroyedOnGround` and `BaseSupplyDestroyed`.

### The report and the claim

**The sighting report.** After a strike group attacks ships, in step 5 and right after its damage lands, the attacking side receives a sighting report of the force attacked. Chapter 3, Reports from a strike, gives what it holds and how its errors are drawn. The event is `ReportReceived`, caused by the `AirAttack`. A strike at a base sends no report. [0026, 0012]

**The claim.** Step 6 makes the claims of the pulse's raids, the raiding side's first and then the defending side's, for each raid in order of its first strike group. A raid that found nothing makes none. Chapter 3, The claim after a raid from the air, gives who claims, what a claim counts and how its false counts are drawn. The claim is recorded as `ClaimReceived`, caused by the claimant's own firing events. [0056]

A side sees: its own `ReportReceived` and `ClaimReceived`, whole. It never sees the enemy's.

### Two carrier forces striking each other in one pulse

Both forces' strikes are launched in the decision step before either lands, each on its own belief. Each raid is worked out on the state at the start of the step. A carrier hit in this pulse still sends its strike, because all attacks are worked out before any damage is applied. In group order damage lands, and the sighting reports follow. The two forces' fights happen in the same step. No rule is specific to this case. [0026, 0027, 0033]

## The end of a daylight pulse for aircraft

The wear step runs in each daylight pulse after air combat, then fatigue, then the supply for sorties. They are worked on the sorties noted in the pulse.

### What counts as a sortie

When any of the wear, fatigue or air supply figures are loaded, the search step and the air combat step note the sorties of the pulse. Each notes the group, the kind, the aircraft that flew (`sent`), the aircraft that came back (`sent` less those shot down), the distance, and the events that say it flew.

| Kind | Aircraft | Distance |
| --- | --- | --- |
| Search | Aircraft that searched | Twice the range of the group's search arc |
| Strike (at a contact or a base) | Aircraft launched | Twice the air distance from the group's hex to the hex it was sent to |
| Patrol | The group's patrol (all it can fly where its escorts do not go), noted in every daylight pulse, raid or not | 0 |
| Escort | Escorts that went | Twice the air distance from the group's hex to the raid's hex |

A fighter group with a patrol and escorts has two sorties. A patrol flies in every daylight pulse, whether a raid comes or not. Escorts fly only when sent. Nothing flies where it is dark. [0035, 0036, 0039]

### Wear

If `air_upkeep.toml` is loaded, each sortie is rolled in order of air group (a group's patrol before its escorts). The aircraft that rolled are those that came back. A sortie with no aircraft back rolls nothing. A group aboard a sunk ship has no position and rolls nothing.

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

Fatigue cuts the aircraft a group is able to fly, as limit 4 of "The order of the limits" says, and does nothing else. It changes no chance to hit and no chance of wear. It cuts search, strike, escort and patrol alike. Fatigue gained in the morning cuts what flies in the afternoon. [0036]

Not built: crew experience, morale, rest better at a well-found base, weather (0036, 0052).

### Supply for sorties

If `air_supply.toml` is loaded, after wear and fatigue, each base, in base order, pays once for all the sorties flown from it in the pulse. For each sortie of a group located at the base with aircraft `sent` above 0, the cost is the cost of one aircraft's sortie times `sent`, where the cost of one aircraft's sortie is `sortie_supply_t` plus `sortie_supply_t_per_100_nm` times the distance over 100. It is counted on the aircraft sent, shot down or not. The costs are added and taken from the stock in one payment, as much as the stock covers: a payment the stock cannot cover takes what is left. `BaseSupplySpent` (reason sorties) records the aircraft, the amount asked, the amount paid and the stock left, caused by the events that say its groups flew, with the sorties listed by kind in the breakdown. It is not recorded if the amount asked is 0. A group aboard a ship spends nothing. No random number. [0039]

A side sees: `BaseSupplySpent` for its own bases, whole. The patrol itself has no event of its own with the supply figures alone; the breakdown lists the patrol aircraft.

## The order of the air rules in a day

Chapter 1, The night pulse and The morning and afternoon pulses, hold the sequence. The air rules in that order:

1. At the start of every pulse: the grounded bases are fixed (A base out of supply).
2. In the night pulse, first: repair, arrivals, replacements (from day 2), daily supply, stand-down and return.
3. In each daylight pulse: air search (chapter 3), the strike decision, air combat, then the wear phase, which runs wear, then fatigue, then supply for sorties, and then movement (chapter 2).

Contacts age at the end of every pulse (chapter 3). The end-of-day phases do nothing for aircraft. [0035, 0036, 0039, 0041]

## Known limits

- **A hard radius.** A strike group launches only if its target is within the type's strike radius. There is no longer reach at reduced load. [0030]
- **One hex.** A strike flies to the hex its contact names and attacks only there. It does not search neighbouring hexes. [0026, 0028]
- **One ship.** All aircraft of a group attack one ship. A strike is not spread, and it attacks nothing but that ship. [0026]
- **No night strikes.** [0026, 0034]
- **Bases.** A base strike launches once a day, in the morning. A bomb's size does not matter. No runway or port damage. No damage assessment for the raider except a claim. [0034, 0052]
- **No rebase.** A group cannot move between bases or between a ship and a base. [0025, 0041, 0052]
- **No landing elsewhere.** Aircraft are lost with their ship. [0025]
- **Pilots are a count.** No experience, no skills, no pilot wounded. Pilots over the sea are not rescued. [0038, 0052]
- **Arrivals and capacity.** An arrival to a group aboard a ship is not tested against the ship's aircraft capacity. An over-capacity ship is found by the end-of-day check, which stops the run.
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
