# The Coral Sea: the first run

Scenario 105, decision 0188, file a (`data/scenarios/coral-sea/01a_coral_sea.toml`): both sides given the orders of the hand calculation, staff against staff, on the live figures and the scenario's own scale. Made by the architect on Nov 20, 2026 on the orders as mended after the one game (commit `b7777ea7`), state hash layout `flagplot-state-v49`, scenario file hash `dabc8db327366bd3`.

`cargo run --release -p sim-cli -- batch data/scenarios/coral-sea/01a_coral_sea.toml --out BATCH.json --runs 1000 --first-seed 1 --label coral-sea-first-run`, seeds 1 to 1,000. Every count below is read from the batch file.

This report gives counts. It changes no point, level, figure, force or order.

## The band

A side is ahead in a game when the margin is on its side (decision 0187, 15).

- **The Japanese side is ahead in 982 games, the Allied side in 18.** No margin is 0.
- The band is that neither side is ahead in more than 650. The scenario is far outside it.
- The Allied margin runs from -69.49 to 51.38; the middle game's is -39.86.

**By level, seen from the Allied side:** decisive victory 3, major victory 10, minor victory 5, stalemate 1, minor defeat 6, major defeat 302, decisive defeat 673.

## Why, from the counts

| Count | Games of 1,000 |
| --- | --- |
| All 5,386 men ashore at Port Moresby | 978 |
| Some men ashore, fewer than all | 8 |
| No man ashore | 14 |
| No transport sunk | 988 |
| One transport sunk or more | 12 |
| A surface fight | 1,000 |

Ships sunk, in games of 1,000:

| Japanese | Games | Allied | Games |
| --- | --- | --- | --- |
| Shoho | 912 | Yorktown | 391 |
| Yubari | 726 | Lexington | 0 |
| Aoba | 707 | Hobart | 68 |
| Kinugasa | 420 | Chicago | 63 |
| Kako | 161 | Australia | 51 |
| Shokaku | 45 | | |
| Furutaka | 28 | | |
| Zuikaku | 1 | | |

1. **The convoy lands whole in 978 games,** and that is 53.86 points. Nothing else in the game is worth as much.
2. **A transport is sunk in 12 games.** The Allied carriers sink Shoho in 912 games and a heavy cruiser of her force in most, and almost never touch the convoy. A staff's strike goes at the nearest force it holds a contact of and takes the most important kind of ship first; no order of the engine's side says to put the transports first (decision 0188, the rulings on the one game, 2).
3. **The support group fights in every game** and sinks Yubari in 726, and does not stop the convoy.
4. **No staff turns the convoy back** (decision 0188, amended 31), so it sails on after its escort's flagship and its covering carrier are sunk.
5. **Lexington is never sunk and Yorktown is in 391.** The reason was not looked for.

## What this report does not do

It does not judge what the scenario should be worth to either side and sets no point. It reports no game from a seat and no game between agents.
