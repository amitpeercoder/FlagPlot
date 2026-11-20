# The Eastern Solomons over five days: the first run

Scenario 103, decision 0184, file a (`data/scenarios/five-days/01a_eastern_solomons.toml`): both sides given the orders of the hand calculation, staff against staff, on the live figures and the game's one scale of points. Made on Nov 20, 2026 at commit `824d65bb` (main merged in), state hash layout `flagplot-state-v49`, scenario file hash `2eb43713436ba48a`.

This report gives counts. It draws no conclusion about balance and changes no point, level or figure: what follows is for the architect to state and the designer to see before any scale of the scenario's own (decision 0184, "Points are not fitted to a run" and "Confirmed on Nov 20, 2026").

## How it was made

- **The game on its own seed:** `cargo run --release -p sim-cli -- run data/scenarios/five-days/01a_eastern_solomons.toml --events` (seed 1942).
- **The 1,000 games:** `cargo run --release -p sim-cli -- batch data/scenarios/five-days/01a_eastern_solomons.toml --out BATCH.json --runs 1000 --first-seed 1 --label five-days-first-run`, seeds 1 to 1000. The batch gives the result of each run and the ships sunk.
- **The counts behind the points:** the batch does not list every objective's count, so the helper `tests/five_days_first_run.rs` (ignored; its doc comment says how it is run) plays the same seeds the same way and writes, for each, the game's own result (each objective's count and points) and the men the events show landed at Taivu. `python3 scripts/five-days/read_first_run.py BATCH.json COUNTS.jsonl` first checks that the two are the same games (every seed's final hash, Allied margin and outcome agree, all 1,000 do) and then prints the tables below.

## File a on its own seed, day by day

Seed 1942. A day runs from 1800 the evening before its date: day 1 is 23 August, its night pulse the night of 22 to 23 August.

| Day | The convoy | Rabaul's bombers | The carriers | Lunga at the end of the day |
| --- | --- | --- | --- | --- |
| 1, 23 Aug | Sails on the morning of the first command (the hold of Taivu sends it by night, 280 nm at 10 kn, 28 hours): (6,-12) to (3,-8) in the night, (3,-6) in the morning, (3,-5) in the afternoon. Sighted in the morning by VB-6 at 280 nm and VB-3 at 312 nm and again in the afternoon at 208 and 243 nm, each beyond the strike radius; not struck. | 41 G4M1, unescorted, raid Lunga in the morning: 8 hits, 1 fighter destroyed on the ground, 150 t of supply and 30 men lost; 3 bombers lost. | Task force 3 runs to (8,-11) and task force 4 to (7,-9) in the night; task force 2 to (7,-3); task force 1 stays at (6,-3). No contact between them. | 750.98 t, 27 aircraft ready: open |
| 2, 24 Aug | Reaches Taivu (3,-4) at 22:00 in the night and lands all 1,500 men that night (350, 350 and 600 from the transports, 152 from Kagero, 48 from Kawakaze); its 90 t of supply land over the night, morning and afternoon, the unloading done at 14:09. Not attacked. It then lies at Taivu. | 23 raid Lunga in the morning: 3 hits on the airfield (damage 6, mended that evening by the pioneers), 50 t of supply and 17 men lost; 3 bombers lost. | Both sides close in the night (task forces 1 and 2 to (7,-5), 3 and 4 to (7,-7)) and strike each other in the morning at 80 nm: Shokaku's flight deck out and her propulsion degraded; Enterprise's propulsion out and flight deck degraded. The afternoon strikes hit nothing. | 599.01 t, 27 ready: open |
| 3, 25 Aug | At Taivu. Struck by Saratoga's groups at 183 nm: Kagero hit, afloat. | 15 raid Lunga in the morning: 3 hits on nothing, 2 bombers lost. The group's fatigue reaches 65.72. | The Japanese strike of the morning sinks Enterprise at (7,-5), with the 45 aircraft aboard her; task force 1 returns to (6,-3), task force 2 to (7,-3), the Japanese forces to (8,-11) and (7,-9). | 480.345 t, 37 ready: open |
| 4, 26 Aug | At Taivu. Kagero sunk by a torpedo of VT-8 in the morning. | Rest, by the staff's reason: "fatigue 60.72, above the 45 of the balanced risk: it rests". | Shokaku's dive bombers that landed on Ryujo hit Saratoga once in the afternoon (6 days of repair). | 381.645 t, 35 ready: open |
| 5, 27 Aug | At Taivu. Kawakaze sunk in the morning. | Rest. | Task forces 3 and 4 move to (7,-8) in the night. No further hit on a carrier. | 284.63 t, 32 ready: open |

**The result:** Allied 20.94 points (Kagero and Kawakaze sunk 2; Shokaku out of action for the whole 30 days, 5; 77 aircraft 3.85; 9 men 0.09; Lunga open 5 of 5 days, 10), Japanese 16.12 (Enterprise sunk 10; Saratoga out of action 6 of 30 days, 1; 84 aircraft 4.2; 47 men 0.47; Lunga shut 0 of 5 days, 0; 90 t landed at Taivu, 0.45). Margin 4.82: an Allied minor victory.

## The 1,000 games

**Results, seen from the Allied side:** decisive victory 404, major victory 286, minor victory 156, stalemate 128, minor defeat 23, major defeat 3, decisive defeat 0. The Allied side wins 846 games, the Japanese side 26; 128 are stalemates.

**The counts behind the points** (lowest, middle and highest over the 1,000 runs, the middle the value at place 500 of the sorted runs, and the runs in which the count is above zero):

| Count | Lowest | Middle | Highest | Runs above zero |
| --- | --- | --- | --- | --- |
| Japanese carriers sunk | 0 | 0 | 2 | 388 |
| American carriers sunk | 0 | 0 | 2 | 355 |
| Japanese carriers afloat and out of action, any days | 0 | 1 | 2 | 752 |
| Japanese carriers afloat and out of action for the whole 30 days | 0 | 1 | 2 | 682 |
| American carriers afloat and out of action, any days | 0 | 0 | 2 | 494 |
| American carriers afloat and out of action for the whole 30 days | 0 | 0 | 2 | 430 |
| Transports sunk | 0 | 0 | 1 | 5 |
| Japanese destroyers sunk | 0 | 1 | 5 | 530 |
| Men landed at Taivu | 1,500 | 1,500 | 1,500 | 1,000 |
| Tons landed at Taivu | 90 | 90 | 90 | 1,000 |
| Japanese men lost | 9 | 9 | 9 | 1,000 |
| Allied men lost | 0 | 27 | 855 | 968 |
| Japanese aircraft lost | 11 | 68 | 153 | 1,000 |
| American aircraft lost | 12 | 47 | 133 | 1,000 |
| Days Lunga was open, of 5 | 3 | 5 | 5 | 1,000 |
| Days Lunga was shut, of 5 | 0 | 0 | 2 | 14 |
| Allied points | 10.94 | 22.44 | 39.82 | 1,000 |
| Japanese points | 1.1 | 8.32 | 29.11 | 1,000 |
| Allied margin | -13.56 | 13.44 | 34.64 | 926 |

**Points by objective** (lowest, middle, highest; runs above zero):

| Objective | Allied | Japanese |
| --- | --- | --- |
| Ships sunk | 0, 2, 20; 717 | 0, 0, 20; 355 |
| Ships out of action | 0, 5, 11; 845 | 0, 3.5, 10; 547 |
| Aircraft destroyed | 0.55, 3.4, 7.65; 1,000 | 0.6, 2.35, 6.65; 1,000 |
| Enemy men lost | 0.09 in every run | 0, 0.27, 8.55; 968 |
| Lunga open (Allied) or shut (Japanese) | 6, 10, 10; 1,000 | 0, 0, 4; 14 |
| Supply landed at Taivu | | 0.45 in every run |

Other counts from the batch: Shokaku is sunk in 116 games, Zuikaku in none, Ryujo in 277; Enterprise in 342, Saratoga in 14. Kagero is sunk in 527 games, Kawakaze 309, Umikaze 156, Suzukaze 58; one transport, in 5. The Allied men lost are lost to fire on Lunga in 968 games (at most 97 in a game) and unfed in 12 (at most 831), the games in which Lunga's stock ran out. The Japanese men lost are the 9 of the 100 at Taivu who go unfed before the convoy lands, in every game; no fight ashore takes place in any game. Rabaul's bombers raid Lunga on day 1 and day 2 in every game, on day 3 in 869, on day 4 in 131 and on day 5 in none.

## What the staffs did that bears on whether it is a contest

Said plainly and left as it is, for the architect to rule on (decision 0184, rule 20 and "Points are not fitted to a run").

1. **The convoy always lands, whole, before it can be struck.** In all 1,000 games all 1,500 men are ashore at Taivu by the end of day 2's night pulse and all 90 t by its afternoon; a transport is sunk in 5 games, after the landing. The Japanese staff's own reason on day 1: the hold "sends it to base:5 ... as it stands ... everyone aboard lands ... 280 nm out at 10 kn = 28 hours ... no contact of ships is held within 15 hexes of (3,-4): the run is not threatened, so it sails alone ... the run goes by night ... arriving in the night with 8 hours of it left". On the Allied side the carriers' searches find it on day 1 at 208 to 312 nm, beyond the 200 and 190 of their strike radius, and Lunga's dive bombers are not on a strike: the cover of Lunga, which the staff forms under the hold, sets VMSB-232 to search ("air_group:10, a search group ... searches") on an arc toward the threat it knows, Rabaul ("threats: base:2 (Rabaul) ... 270 to 340"), and no intent of the Allied side's orders strikes ships from Lunga.
2. **After landing the convoy stays at Taivu and its destroyers are struck there.** Its route ends where it lands: "0 nm back to the source base:5", the hold naming no source, so the staff takes Taivu as the source; on later days "its forces limit names no source, so the staff forms no supply for it". Kagero is sunk in 527 games.
3. **The landing scores little.** The convoy's success is worth 0.45 points (90 t at 0.005 a ton); the men landed score nothing, as no kind of objective counts them (decision 0184, amended 13).
4. **Lunga is almost never shut.** It is shut on any day in 14 games. Rabaul's bombers raid unescorted (the forces limit of "neutralise Lunga" names the bombers alone; the staff's reason, "the fighters can give 0 escorts (it goes without an escort if it must)"), and stop when tired: "air_group:27, a bomber group, has fatigue 60.72, above the 45 of the balanced risk: it rests; the raid on base:1 (Lunga) does not go". The 10 points of "Lunga open" go to the Allied side in full in the middle game, and the 10 of "Lunga shut" to the Japanese side in none.
5. **The carriers meet.** Each side's carrier forces close on a contact and strike in most games (above: a Japanese carrier sunk or out of action in 752 or more, an American one sunk in 355 and out of action in 494).

## What this report does not do

It does not judge balance, set or suggest any point, level or figure, or alter any force. It does not report the four games between agents of decision 0184, rule 19, which are not part of this build.
