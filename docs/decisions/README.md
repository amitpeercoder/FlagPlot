# Decisions

Design choices the spec left open, one short file each. A decision here can be overturned. When that happens, change the file and say what replaced it, so the history stays readable.

Files are numbered in the order the decisions were made. Add the next number for a new one.

| # | Decision | Status |
| --- | --- | --- |
| 0001 | A game day runs 18:00 to 18:00 | Needs review |
| 0002 | Fixed-point numbers: 64-bit, six decimals, truncating | Settled |
| 0003 | Random numbers: SplitMix64 mixing, keyed per draw | Settled |
| 0004 | State hash: FNV-1a over a hand-written walk, no dependencies in sim-core | Settled |
| 0005 | Movement: hex plus progress, greedy steps, no pathfinding yet | Needs review |
| 0006 | Fuel: burn per mile rises with the square of speed. Placeholder | Needs review |
| 0007 | Truth and belief: `SideView` for players, `umpire()` for tests | Settled |
| 0008 | Orders: checked against truth at intake, kept whether accepted or not | Needs review |
| 0009 | A scenario file is also the saved-game format, for now | Settled |
| 0010 | Scenario tests live in a `tests` workspace member | Settled |

"Needs review" means the choice affects how the game plays and the designer should confirm it. "Settled" means it is an engineering choice with no effect on play.
