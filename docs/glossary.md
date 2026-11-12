# Glossary

One meaning for each word the rules use. The complexity review after scenario 5 found these used in more than one sense. Decision records, event texts and briefings follow this page; where an older record says otherwise, this page is right.

| Word | Means | Does not mean |
| --- | --- | --- |
| **Air distance** | How far an aircraft flies between two hexes: the straight line between their centres, in whole miles (decision 0040). Every range, radius and sortie distance, and every tool that measures one, uses it | The number of hexes between them times 40, which is longer off the grid's axes |
| **Sea distance** | How far a ship is charged for steaming between two hexes by the map's shortest sea path: the sum of the charges of its steps. Where the way is clear it is the air distance; where land forces a detour the steps are charged 40 miles until the way is clear (decision 0058) | The hex count of the path times 40 miles |
| **Step charge** | What the step from one hex centre to the next is charged, in miles, 1 to 40: the miles it brings the force closer to its waypoint in a straight line where the way is clear, 40 where it is not (decision 0058) | |
| **Clear way** | From a hex, a sea path to the waypoint with as many steps as the hex distance: no land forces a detour (decision 0058) | |
| **Readiness** | What a force is ordered to be: normal or battle (decision 0021) | Whether it is alert at this moment |
| **Alert** | The state of a force whose lookouts see at full effect. A force at battle readiness is always alert | An order |
| **Ready** | A force's state as a fight begins, kept for the whole fight: at battle readiness, or alert since before the present pulse began. A force that is not ready fires its guns at a share, `unready_gun_share`, in every round (decision 0068) | Alert. A force alerted in this pulse sees at full effect and is not ready |
| **Alerted** | How a force at normal readiness came to be alert: by a fresh contact nearby, by being fired on, or by an alarm | |
| **Alarm** | The roll a force that is not alert makes when a fight with gunfire happens in its hex | |
| **Stance** | A force's ordered way of meeting an enemy: engage or avoid (decision 0014) | |
| **Avoiding for the pulse** | What a force does after it breaks off or slips away: it takes the avoid stance until the pulse ends, whatever its orders (decision 0022) | A change to its orders |
| **Break off** | To end a fight because the share of a force's ships sunk or crippled has reached its break-off level, or because it is avoiding and was fired on | To move. A force that breaks off keeps to its route |
| **Slip away** | To end a fight unseen: an avoiding force that has seen an enemy that has not seen it | |
| **Keep share** | The share of its fit ships a force must keep when it detaches stragglers (decision 0016) | |
| **Patrol share** | The share of its aircraft a fighter group keeps over its own hex (decision 0027) | |
| **Sight** | What an air search does to a force: it sights it, and a report follows | |
| **See** | What a force does in a fight, round by round | |
| **Report** | What a side is told. It can be wrong. Every change to what a side believes is a report | The truth |
| **Contact** | What a side believes about an enemy force: a hex, a count, a kind and an age. Made and refreshed by reports | The force itself |
| **Position** | A force's hex: the nearest hex centre. A group aboard a ship is at its ship's force's position | How far along a leg it is, which is working data of movement |
| **Radius** | How far out an aircraft type goes and comes back. `search_range_nm` is the largest range a search arc may name, and the engine refuses an arc beyond it. `strike_range_nm` is what a group's reach is measured from. Both columns are named range and hold radii | Range |
| **Reach** | How far an air group's strikes and escorts may go: normal, the type's strike radius, or extended, the radius and a share more. Set by order, normal until then (decision 0059) | Radius |
| **Extended reach** | The reach of a group that has been ordered to it: the radius times (1 plus the extended reach share), cut to a whole mile. A sortie beyond the radius may cost aircraft that run out of fuel on the way home | Normal reach |
| **Target list** | The kinds of ship a strike group goes for, in order, set by order. The default is all six kinds in the order of importance. A group is launched only at a contact taken for a kind on its list, and attacks over the target the ship its list ranks first, the kinds not on it following in the order of importance (decision 0063) | Target priority, priorities |
| **In the air** | Describes the aircraft of a group that flew in the pulse and were not shot down, until the wear phase of the pulse brings them home (decision 0059) | Aboard |
| **Landed elsewhere** | Describes aircraft that were in the air when their own deck was lost and landed at another carrier or a base, joining or forming a group there (decision 0059) | Rebased |
| **Ditched** | Describes an aircraft that ran out of fuel on the way home from a sortie beyond its radius, or that found no deck to land on, and came down in the sea. It is gone from the group for good and its crew may be recovered or lost (decision 0059) | Shot down, written off |
| **Range** | How far an aircraft flies one way, as a sourced fact. Only the derivation tool reads it. The engine reads the radius columns, whose names say range (corrected against the code, which reads `search_range_nm`) | |
| **Crippled** | A ship with flotation damage of 50 or more. It counts toward break-off | Slowed |
| **Straggler** | A ship whose propulsion is degraded or out. The straggler instruction acts on it | Crippled |
| **Degraded, out** | The two damaged states of a named system. Degraded halves what the system does; out stops it | |
| **Raid** | The strikes launched in one pulse from one place at one hex, with their escorts | One air group's strike |
| **Mission** | What an air group is ordered to fly: rest, search, naval strike, combat air patrol. A task force also has a mission, which no rule reads yet | |
| **Placeholder** | A figure invented so that the engine runs. Never historical | |
| **Settled** | A choice between sourced figures, made by the architect and written down with its reason | Stated by a source |
| **Pulse** | One of the three parts of a game day, resolved in this order: night, morning, afternoon. A pulse is the same across the whole map (decision 0001) | Darkness. The night pulse is dark, and light is asked of one function |
| **Phase** | A named step the day runs in a fixed order, inside a pulse or after the last pulse. Some phases are empty slots | A pulse |
| **Draw** | One random number taken from a stream keyed on the seed, day, pulse, subsystem, entity and rule. A chance succeeds if its draw is below it (decision 0003) | A number from a generator that advances as the game runs |
| **Hash** | The 64-bit summary of the state at the end of a day. Two runs agree if their daily hashes agree (decision 0004) | A security measure |
| **Saved game** | A file holding the scenario's name and hash, the hash of each data file, the seed, who held each seat, the orders as handed in and the daily hashes. It holds no state (decision 0043) | A snapshot a game can be resumed from |
| **Replay** | Running a saved game again from the start with no player, and failing on the first day whose hash differs | The replay file the viewer reads, which is an exported run |
| **Seat** | A side's place in a game. A person, an agent or the script holds it (decision 0050) | A unit |
| **Unit** | A task force or an air group: what an order is addressed to | A ship |
| **Issuer** | Who gave an order: the player, the staff or the local commander. It is part of the order's origin | Who holds the seat |
| **Origin** | Who issued an order and the intent it serves. Every order carries one (decision 0008) | |
| **Staff** | The planner that gives orders between days. It is not built | The local commander |
| **Local commander** | A rule that gives an order for a unit on the spot, inside a day. Today it is the straggler rule's order to head for the home port (decisions 0016 and 0055) | The staff |
| **Intake** | The step at the start of a day that numbers, checks and applies or refuses the orders waiting (decision 0008) | |
| **Standing order** | An accepted order that stays in force until another order of its slot replaces it | An order carried out and finished |
| **Cause** | An order or an earlier event that an event names as what led to it (decision 0047 cuts them for a side) | |
| **Breakdown** | The factors of a computed quantity or chance, recorded when recording is on. It never changes what happens or the hash. A side sees it as it sees the event, except a claim's, which it is not shown (decision 0060) | |
| **Whole, cut, absent** | How a side is given an event: as the log has it, cut down to what the side may know, or not at all (decision 0047). A cut event never names an enemy ship, task force or air group (decision 0065) | |
| **Hex kind** | One of deep sea, shallow sea, coast and land | |
| **Navigable** | Describes a hex a ship may enter: deep sea, shallow sea or coast that is on the map | Land |
| **Closed side** | A side of a hex a ship cannot cross. A mark on either hex closes it | |
| **Route** | The list of waypoints a force steams to, with the index of the next one | The path between them, which is never stored |
| **Waypoint** | A hex a route names. A force reaches it when it reaches the hex's centre | |
| **Leg** | The step between the centres of two neighbouring hexes, charged `leg_nm` miles (1 to 40), set when it begins. Once begun it is always finished at that charge | |
| **Segment** | A force's steaming from the start of a pulse or its last fight to its next fight or the end of the pulse. It is recorded as one steamed event | A leg |
| **Budget** | The miles a force may steam in a pulse: the lower of its time limit and its fuel limit | |
| **Fuel limit** | How far the fuel of a force's least-fuelled ship lasts at the force's speed | The ship's fuel capacity |
| **Burn** | The fuel a ship uses steaming a distance: the distance times the cruise burn per mile, times the speed multiplier (decision 0006) | Fuel carried as cargo, which cannot be burned |
| **Under way** | The status of a force that has a route to steam. A force dead in the water is still under way | Moving |
| **Arrived** | The status of a force that has reached the centre of its last waypoint | Docked |
| **Out of fuel** | The status of a force whose fuel ran out before its last waypoint | A force with no fuel that is not under way |
| **Stopped** | Not under way: idle, arrived or out of fuel. Refuelling also asks that the force is exactly at a hex centre | |
| **Dead in the water** | Describes a ship whose propulsion is out, or the force it slows. Its speed is 0 | Crippled |
| **Sound ship** | A ship whose propulsion is working | A ship with no damage |
| **Cargo** | Supply or fuel carried in a ship's hold, in tons, up to the class's cargo capacity | The fuel a ship steams on |
| **Stock** | The tons of supply or of fuel a base holds | |
| **Berth** | A place alongside at a base where a ship works cargo at the berth rate. A size 0 port has none (decision 0011) | |
| **Lighter** | The boats that work cargo for a ship with no berth, at the lighter rate | |
| **Arc** | The sector an air group searches: a from bearing, a to bearing and a range in miles | |
| **Bearing** | Degrees clockwise from north, in whole multiples of ten | |
| **Sweep width** | The width of sea one aircraft of a type sees (`aircraft_types*.csv: sweep_width_nm`) | |
| **Starting contact** | A contact a scenario gives a side at the start. It is entered as a report with the stated hex, count and kind, then aged (decision 0021) | A report with errors |
| **Fresh** | Describes a contact whose age is no more than `search.toml: fresh_contact_max_age_pulses`. A fresh contact can be refreshed by a report and can alert a force | Describes a contact that can be struck, which has its own limit |
| **Forgotten** | Describes a contact whose age has reached `search.toml: contact_drop_age_pulses`. It is dropped | |
| **Taken for** | The kind a strike's contact carried when the strike was launched. The raid's claim names it | The kind of ship that was there |
| **Claim** | A report of what a side's own people believe they did to the enemy. It can be wrong and it never ages (decisions 0014 and 0056) | A loss in truth |
| **Fight** | The rounds that two opposing task forces fight when they meet (decision 0014) | |
| **Meeting** | Two opposing forces in one hex, either because one entered it or because both began a pulse there and neither will move. Each meeting is one fight (decision 0022) | |
| **Round** | One step of a fight, `surface_combat.toml: round_minutes` minutes long | |
| **Salvo** | One ship's gun fire or its torpedo fire in one round at one target | |
| **Shot** | One unit of fire that can hit: one gun round, one torpedo tube or one aircraft's weapon. Each whole shot is one draw (decision 0033) | |
| **Hit** | A shot that succeeded | |
| **Damage points** | What one hit adds to a ship's track: 100 times the weapon's damage over the target class's toughness | |
| **Damage track** | One of a ship's two damage numbers, flotation and fire, each from 0 to 100 | |
| **System hit** | A hit that also steps one of a ship's named systems down a level (decisions 0016 and 0033) | |
| **Toughness** | A ship class's figure that divides a weapon's damage into damage points | |
| **Lit** | Describes a ship with fire damage above 0 in a dark fight. It is easier to hit | |
| **Surprised** | Describes a force that has not sighted the enemy in a round when the enemy has, and the enemy has shots to fire. It does not fire | |
| **Effective stance** | A force's stance in a fight: avoid if it broke off or slipped away earlier in the pulse, otherwise its ordered stance (decision 0022) | |
| **Firing event** | An event of a side's own fire in a raid: its escorts' or patrol's fight, its guns, or its attack. A raid's claim rests on them (decision 0056) | |
| **Objective** | A count of something the engine records, taken for one side, with points (decision 0054) | An order |
| **Margin** | A side's points less the enemy's | |
| **Level** | One of three sizes of margin, minor, major and decisive, that the scenario gives | A ship's level of damage |
| **Outcome** | One of seven: a minor, major or decisive victory or defeat, or a stalemate | |
| **Result** | The umpire's points, margin and outcome for each side, worked from the final truth and the event log. It is given when the game ends | What a side knows of its progress |
| **Cargo landed** | For the objective: the tons a side's ships unloaded at a base less the tons they loaded back there, never below 0 (decision 0067) | Tons unloaded |
| **Base open** | A base that ends a day with supply above zero and at least one ready aircraft at it. The objective counts the days (decision 0067) | A base's state at the end of the game |
| **Air group** | One unit of aircraft of one type, at a base or aboard a ship | A raid |
| **Ready aircraft** | Aircraft of a group that are fit to fly | |
| **Damaged aircraft** | Aircraft that came back worn and cannot fly until repaired | Aircraft lost |
| **Strength** | The ceiling an air group's aircraft and pilots are replaced up to | The number of aircraft it has now |
| **Pilots** | A count for an air group. One pilot stands for the crew of one aircraft | People |
| **Sortie** | One kind of flight by one group in one pulse, out and back: a search, a strike, a patrol or an escort | |
| **Wear** | The roll an aircraft that came back makes after a sortie. A worn aircraft leaves ready and joins damaged, or is written off | |
| **Write off** | To lose a worn aircraft for good | Shoot down |
| **Fatigue** | A number from 0 to 100 for an air group or a task force. It rises by flying or by being alert, and falls by rest | |
| **Stood down** | Describes an air group the staff has stopped from flying for the day because its fatigue is above its rest threshold | |
| **Rest threshold** | The fatigue above which the staff stands an air group down (decision 0036) | |
| **Experience** | A whole number from 0 to 4 for a ship's crew, 2 being its service's standard. Set when a scenario loads and never changed by a rule. In a dark surface fight it multiplies a ship's gun and torpedo hit chances, and a force's chance to sight by the mean of its ships' values, by `1 + strength x (experience - 2)` (decision 0064). Not read by day, and not yet by aircraft | Skill, morale, training |
| **Strength of experience** | One of three figures in `surface_combat.toml`, from 0 to 0.5, the same for both sides: how much experience moves the gun, torpedo and sighting chances. At 0 experience does nothing | |
| **Escort** | A fighter sent with a raid. Escorts fire at the patrol the raid meets (decisions 0027 and 0030) | A patrol |
| **Patrol** | Fighters a group keeps over its own hex, which fire at raids that attack there | |
| **Bomber** | An aircraft of a strike group, whether it carries bombs or a torpedo | |
| **Base strike** | A mission to strike a named enemy base, launched once a day, in the first daylight pulse in which the base is in reach (decisions 0034 and 0063) | A naval strike |
| **Arrival** | Aircraft and pilots a scenario delivers to an air group on a named day (decision 0041) | A replacement |
| **Replacement** | Aircraft and pilots a base receives each day from day 2, given to its groups below strength (decision 0038) | An arrival |
| **Out of supply** | Describes a base whose supply stock is exactly zero at the start of a pulse. Its groups fly nothing in that pulse (decision 0039) | |
| **Strike radius** | An aircraft type's `strike_range_nm`: the radius a strike and an escort are measured from on normal reach, and the distance beyond which a sortie may cost aircraft that run out of fuel. The column's name says range, and the glossary's range is the sourced one-way figure | Range |
| **Loaded** | Describes a rule whose figures file the scenario file names. A rule whose file is not named does nothing (chapter 1 of the rulebook, Where figures come from and when a rule is loaded) | Read from the code |
| **Importance** | The order of the kinds of ship, most important first: carrier, battleship, cruiser, destroyer, transport, auxiliary. Rules that pick the most important kind of a set use it | Size or value |
