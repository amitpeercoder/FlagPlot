# Decisions

Design choices the spec left open, one short file each. A decision here can be overturned. When that happens, change the file and say what replaced it, so the history stays readable.

Files are numbered in the order the decisions were made. Add the next number for a new one.

A record that adds a rule to the simulation ends with a rule card: the state the rule reads and the state it writes. Records 0001 to 0010 predate this and gain their cards at the complexity review after scenario 5.

| # | Decision | Status |
| --- | --- | --- |
| 0001 | A game day runs 18:00 to 18:00 in the reference zone (UTC+11 for the slice) | Confirmed |
| 0002 | Fixed-point numbers: 64-bit, six decimals, truncating | Settled |
| 0003 | Random numbers: SplitMix64 mixing, keyed per draw | Settled |
| 0004 | State hash: FNV-1a over a hand-written walk, no dependencies in sim-core | Settled |
| 0005 | Movement: nearest-centre hex, turn back on a re-route. Pathfinding is in 0017 | Confirmed |
| 0006 | Fuel: burn per mile rises with the square of speed above cruise, never below the cruise rate. Placeholder | Confirmed |
| 0007 | Truth and belief: `SideView` for players, `umpire()` for tests | Settled |
| 0008 | Orders: checked at intake against what the side can see, kept whether accepted or not | Confirmed |
| 0009 | A scenario file is also the saved-game format, for now | Settled |
| 0010 | Scenario tests live in a `tests` workspace member | Settled |
| 0011 | Cargo handling: berths and lighters by port size, load and unload orders. Placeholder figures | Confirmed |
| 0012 | Air search by coverage, reports with independent errors, contacts that age and merge. Placeholder figures | Confirmed |
| 0013 | A random number key may name a second entity | Confirmed |
| 0014 | Surface combat: sight first, fire by expected hits, three damage tracks. Scenario 4 built in stages. Placeholder figures | Confirmed |
| 0015 | Order-of-battle facts: sourced rows in CSV, ratings derived by written rules, no invented figures for real ships | Confirmed; standing rules need review |
| 0016 | Damage to named systems in three states, and a standing instruction for stragglers. Replaces the system track of 0014. Placeholder figures | Confirmed |
| 0017 | The slice map generated from Natural Earth coastlines, and shortest-path movement | Confirmed |
| 0018 | A read-only replay viewer, as a debugging tool and not the game's interface | Confirmed |
| 0019 | Savo Island: who was where, standard displacement, and unknown radar treated as not fitted | Confirmed |
| 0020 | Game figures for real ships derived from facts by written rules, with three shared scaling constants | Confirmed; constants are placeholders |
| 0021 | Readiness by order, alertness from belief and events, and an alarm roll when a fight breaks out nearby | Confirmed |
| 0022 | Movement and fights in one time-ordered step; a force that breaks off avoids action for the rest of the pulse | Confirmed |
| 0023 | The Savo Island ships: weak rows read by exception, designed rates of fire by named exception, and ammunition not limiting for three ships | Confirmed |
| 0024 | The Savo Island scenario: one hex, three forces, the pickets and Jarvis left out; what the first runs show | Confirmed |
| 0025 | Air groups aboard ships: aircraft capacity, position from the ship, search from a carrier, lost with the ship | Confirmed |
| 0026 | Air strikes on ships: decided on belief, resolved in the same pulse, one hit mechanism; the flight deck as a fifth system | Confirmed |
| 0027 | Fighters on patrol and escort, anti-aircraft fire, and aircraft lost: escort against patrol one for one, then guns, then the attack | Confirmed |
| 0028 | A strike that finds nothing drops the contact it was sent at | Confirmed |
| 0029 | Figures for carriers, anti-aircraft guns and aircraft from facts: gun weights, strike radius, protection, and ships whose guns are not modelled | Confirmed |
| 0030 | Escorts have a radius: fighters go with a raid only within their type's radius, and otherwise stay on patrol | Confirmed |
| 0031 | The Eastern Solomons scenario: one day, four forces and Henderson Field; positions, air groups and orders | Confirmed |
| 0032 | What the complexity review cut and merged: noticing, contact confidence, two duplicate figures, and clock rounding | Confirmed |
| 0033 | Hits revised: a system hit depends on the damage, each shot is rolled, and a patrol divides among the raids on its hex | Confirmed |
| 0034 | Raids on a base: the base strike mission, the base's guns, and hits on aircraft on the ground and on supply | Confirmed |
| 0035 | Wear, repair and operational losses: damaged aircraft, a wear roll that rises with distance, a share written off, repairs a day at a base | Confirmed; 2 needs review, after the air figures are tuned |
| 0036 | Fatigue and rest: one number for an air group that cuts the aircraft that fly, and a rest threshold the staff applies | Confirmed |
| 0037 | Fatigue at battle readiness: an alert force tires each pulse, and fatigue cuts its chance to sight | Confirmed |
| 0038 | Pilots and replacements: a pilot is lost with an aircraft shot down away from home, and aircraft and pilots arrive at a base at a daily rate | Confirmed |
| 0039 | Base supply consumption: supply spent for each sortie, each aircraft a day and each repair, and a base out of supply flies nothing | Confirmed |
| 0040 | Air distance is a straight line between hex centres, in whole miles, for every rule that asks how far an aircraft flies. Ships are deferred | Confirmed |
| 0041 | Aircraft that arrive on a day: a scenario may deliver aircraft and pilots to an air group on a named day, raising its strength | Confirmed |
| 0042 | The Rabaul against Guadalcanal scenario: who is there, what the aircraft can do, and what is a placeholder | Confirmed |
| 0043 | The determinism scenario, saved games and replay: the full slice, what shuffled order means, `--save`, `replay` and `hash` | Confirmed |
| 0044 | The soak scenario: the full slice carried on for 180 days, with every subsystem in use in each 30 days | Confirmed |
| 0045 | Refuelling in port: a base has a fuel stock, and a force stopped at its own side's base at the start of a day fills up from it | Confirmed |
| 0046 | Fuel as a cargo: a ship carries fuel as cargo beside supply, a load order names its cargo, and fuel landed joins the base's stock | Confirmed |
| 0047 | What a side may know of what happened: which events each side sees and what is cut from them | Confirmed |
| 0048 | The briefing and the queries: a daily situation report for a side, list and detail queries, and explain, built only on a side's view, as JSON with schemas | Confirmed |
| 0049 | Measuring: air distance, sea route, a force's passage with time and fuel, and an air group's reach, each by the engine's own function | Confirmed |
| 0050 | Playing a side: seats, `issue_orders` and `end_day`, saved orders as handed in, the transcript, and `sim-cli serve` | Confirmed |
| 0051 | The MCP server and scenario 9: `sim-cli mcp`, `sim-cli play`, a game as a file of requests, and a model's two recorded games | Confirmed |
| 0052 | What step 1 leaves out: ground combat, land units, shore bombardment and base orders deferred; pilots as a count and weapons as figures recorded as choices | Confirmed |
| 0053 | The viewer measures (air distance and sea route, checked against the engine) and shows a side's view, built from the events the engine gave it | Settled |
| 0054 | Objectives and results: a mission and counted objectives for each side, points, a margin and a level from decisive victory to decisive defeat; no rule reads them | Confirmed |
| 0055 | After the architecture review: the day is the smallest command cycle, saved games need not survive rule changes, own units are known exactly; a player's own count of orders, the local commander as issuer, a rule tag on random keys, every state field named beside the hash | Confirmed (1 to 4 and the pillar wording); settled (5 to 9) |
| 0056 | Claims from the air: one claim a side a raid, of aircraft shot down, hits on ships, ships sunk and aircraft destroyed on the ground; each miss may be claimed | Confirmed |
| 0057 | The state hash at version 2 with no tag bytes, golden files remade; batch summaries and replays record the inputs they were run on, and `batch --label` | Settled |
| 0058 | Ships are charged straight-line miles: a step costs the miles it brings the force closer to its waypoint; 40 a hex only where land forces a detour | Confirmed |
| 0059 | Aircraft that cannot land where they took off: extended reach by order with ditching for fuel, another deck or a field when the deck is lost, and the crew's fate by where it comes down | Confirmed |
| 0060 | A claim's breakdown is the umpire's: a side sees its claim whole and no breakdown of it, because the breakdown says how much of the claim is real | Confirmed |
| 0061 | A search that sighted is shown to its side cut: the group (and, until 0065, the force), without the distance or the chance, which are worked from the force's true position. Its open question, whether an event should name an enemy force by its number, is answered by 0065 | Confirmed |
| 0062 | What `batch` records for each run: counts from the event log and end state for every step 2 measure, header tables, batch file version 2 (version 3, with the shots of each ship, in 0068's change); no rule, no state | Settled |
| 0063 | Strike target priority as an order: a list of kinds a strike group goes for, in order; a kind off the list is never launched at, which holds a strike; a base strike may fly in the afternoon. Built (stage 1) | Confirmed |
| 0064 | Experience: a value from 0 to 4 for a ship and an air group, derived from fact rows by yes or no questions and fixed before tuning; ships first, read at night by the hit and sighting chances; air groups second. Approved as a plan; not built yet | Confirmed |
| 0065 | A side's events name no enemy ship, task force or air group: the cut event has no field for it. A fight holds the side's own force; six kinds that were whole lose only the name. An enemy base and the target of a side's own order are still named. Replaces item 8 of 0047 | Confirmed |
| 0066 | Land units, carried by sea and fed from a base: a count of men, troop capacity, men worked as cargo at a fixed weight, a landing that forms a base, a daily charge for each man and a share of the unfed lost. Stage 1 of the ground combat plan. Placeholder figures | Confirmed as to the plan; 7, 12 and 14 need review. Not built |
| 0067 | Two objectives that could be gamed: cargo landed becomes tons unloaded less tons loaded back at that base; base open is counted by days and scored by the share of days, from a record of every base at the end of each day. Built Nov 12, 2026; the state hash of every scenario with a base changed, because the hash counts the log's events | Confirmed |
| 0068 | A force not at battle readiness fires with part of its guns: one share on its gun shots for the whole of a fight, unless it was ordered to battle readiness or alerted before the pulse began. Found at Savo Island, where the Allied groups fire six times the shells they did. Built. | Confirmed |
| 0069 | Ship repair: light damage mended by the staff at a forward base with a repair figure; a force sent to a yard off the map returns whole on a day worked out from its damage and told beforehand. Not built: it follows tuning | Confirmed |
| 0070 | Rates of fire as achieved, in the derivation: a class fires at the lesser of its sourced rate and the median achieved rate of its kind of battery (salvo, destroyer, continuous), from named cases of shells over a stated span of five minutes or more. Tenryu no longer reads the twin-mount row 432 | Settled by the architect, to be reviewed |
| 0071 | A force not ready fires part of its torpedoes: the readiness of decision 0068 also takes a share of the torpedo shots, `unready_torpedo_share`. Found because at Savo Island Japanese ships left dead in the water, mostly by Allied torpedoes, are caught by day in half the runs. Reverses 0068's answer 4 | Confirmed (decided by the architect on the designer's delegation) |
| 0072 | The Cape Esperance scenario, planned: Task Force 64's nine ships against the bombardment group's five, the reinforcement group left out; both meet in hex (2,-5) at 23:42, the Americans at battle readiness, the Japanese not alert and not ready; class disputes settled, what is left for exceptions, decision 0070 and research; what the rules cannot show, with radar fire control flagged. No batch is run before Savo Island is tuned | Plan; settled by the architect, to be reviewed |
| 0074 | A derived class's torpedo tubes are the tubes that bear on one side: the number a source gives, else all for centreline mounts and half for wing mounts, else the whole count with the class named as not split. A rule of the derivation; nothing in the simulation changes. Firing the other side's tubes later in a fight is left to the designer. Not built: held until Savo Island's first tuning rounds, and built only with a rule for the other side's tubes if the torpedo measures call for it | Written; held |
| 0076 | How long surprise lasts: no rule. In the record a force under fire replied within about one round, which the rules give; the longer interval is the time between being seen and being shot at, outside a fight. A longer surprise for an unready force would charge its unreadiness twice (decision 0068). Three prototypes moved nothing at Savo Island | Decided: no rule |
| 0077 | Which radar sets count for sighting ships at night: a derived class has the radar figure only for a surface-search set whose display has a plan position indicator; an air-search set, a set of type not stated or one with no display row does not count and is named. At Savo Island no ship has such a set; at Cape Esperance Helena and Boise do. A rule of the derivation; no figure moves. Not built: it follows decisions 0073 and 0075 | Decided |

"Needs review" means the choice affects how the game plays and the designer should confirm it. "Confirmed" means the designer has. "Settled" means it is an engineering choice with no effect on play.
