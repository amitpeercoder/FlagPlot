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

"Needs review" means the choice affects how the game plays and the designer should confirm it. "Confirmed" means the designer has. "Settled" means it is an engineering choice with no effect on play.
