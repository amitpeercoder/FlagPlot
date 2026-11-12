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
| 0024 | The Savo Island scenario: one hex, three forces, the pickets and Jarvis left out; what the first runs show | Needs review |
| 0025 | Air groups aboard ships: aircraft capacity, position from the ship, search from a carrier, lost with the ship | Confirmed |
| 0026 | Air strikes on ships: decided on belief, resolved in the same pulse, one hit mechanism; the flight deck as a fifth system | Confirmed |
| 0027 | Fighters on patrol and escort, anti-aircraft fire, and aircraft lost: escort against patrol one for one, then guns, then the attack | Confirmed |
| 0028 | A strike that finds nothing drops the contact it was sent at | Confirmed |
| 0029 | Figures for carriers, anti-aircraft guns and aircraft from facts: gun weights, strike radius, protection, and ships whose guns are not modelled | Confirmed |
| 0030 | Escorts have a radius: fighters go with a raid only within their type's radius, and otherwise stay on patrol | Confirmed |
| 0031 | The Eastern Solomons scenario: one day, four forces and Henderson Field; positions, air groups and orders | Confirmed |
| 0032 | What the complexity review cut and merged: noticing, contact confidence, two duplicate figures, and clock rounding | Confirmed |
| 0033 | Hits revised: a system hit depends on the damage, each shot is rolled, and a patrol divides among the raids on its hex | Confirmed |
| 0034 | Raids on a base: the base strike mission, the base's guns, and hits on aircraft on the ground and on supply | Confirmed; 3 and 6 need review |
| 0035 | Wear, repair and operational losses: damaged aircraft, a wear roll that rises with distance, a share written off, repairs a day at a base | Confirmed; 2, 9 and 12 need review |
| 0036 | Fatigue and rest: one number for an air group that cuts the aircraft that fly, and a rest threshold the staff applies | Confirmed; 4, 5 and 9 need review |
| 0037 | Fatigue at battle readiness: an alert force tires each pulse, and fatigue cuts its chance to sight | Confirmed |
| 0038 | Pilots and replacements: a pilot is lost with an aircraft shot down away from home, and aircraft and pilots arrive at a base at a daily rate | Confirmed; 4, 8 and 10 need review |
| 0039 | Base supply consumption: supply spent for each sortie, each aircraft a day and each repair, and a base out of supply flies nothing | Confirmed; 4, 6 and 8 need review |
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
| 0061 | A search that sighted is shown to its side cut: the group and the force, without the distance or the chance, which are worked from the force's true position. Open: whether an event should name an enemy force by its number | Confirmed; the open question needs review |

"Needs review" means the choice affects how the game plays and the designer should confirm it. "Confirmed" means the designer has. "Settled" means it is an engineering choice with no effect on play.
