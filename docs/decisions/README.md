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
| 0005 | Movement: nearest-centre hex, greedy steps, pathfinding with the real map | Confirmed, part not built |
| 0006 | Fuel: burn per mile rises with the square of speed above cruise, never below the cruise rate. Placeholder | Confirmed, part not built |
| 0007 | Truth and belief: `SideView` for players, `umpire()` for tests | Settled |
| 0008 | Orders: checked at intake against what the side can see, kept whether accepted or not | Confirmed |
| 0009 | A scenario file is also the saved-game format, for now | Settled |
| 0010 | Scenario tests live in a `tests` workspace member | Settled |
| 0011 | Cargo handling: berths and lighters by port size, load and unload orders. Placeholder figures | Confirmed |

"Needs review" means the choice affects how the game plays and the designer should confirm it. "Confirmed" means the designer has. "Settled" means it is an engineering choice with no effect on play.
