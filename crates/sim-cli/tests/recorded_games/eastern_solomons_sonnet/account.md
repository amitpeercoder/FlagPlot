# The player's account

A Sonnet model, playing alone on Nov 12, 2026 with `sim-cli play` and nothing else. Its report as it gave it; only the game file's path is left out.

## Situation as I understood it

- The game was 1 day long ("Day 1 of 1"). I had two carrier task forces stacked at (7,-5): TF16 with Enterprise and TF11 with Saratoga, plus Henderson Field at (2,-4), which holds a Marine fighter group and a Marine dive-bomber group. No contacts, no enemy bases listed, no arrivals.
- Mission: find and sink the Japanese carriers and keep my own carriers in action. Scoring: points for ships sunk (carrier 10), for ships left crippled (carrier 5) and for enemy aircraft destroyed (0.05 each).
- The briefing said missions are standing orders and that I cannot see a contact and then order a strike, so I had to set everything before the day. I did not know where the enemy was.

## Decisions (all in one issue_orders call, all queued, none refused)

1. I first probed the search range with a deliberately huge value (9999). It was refused with the maximum, 200 nm. That refusal was clear. It was a probe, not a mistake, and it was kept in the game file.
2. Search: VS-5 over bearings 270 to 0 and VS-3 over 0 to 90, both at 200 nm. VMSB-232 (Henderson) searched 300 to 60 at 200 nm. I chose the northern half because I guessed the enemy would come from the north. The briefing gave no direction, so this was a guess. I did not search south.
3. Strike: VB-6, VT-3, VB-3 and VT-8 were set to naval_strike, so they would launch at any contact within their radius (about 190 to 200 nm).
4. Fighters: VF-6, VF-5 and VMF-223 were set to combat air patrol. VF-5 got a patrol share of 0.5 so half of it could escort. VF-6 and VMF-223 stayed at share 1.
5. Both task forces were set to battle readiness. I gave no movement orders: the carriers stayed where they were.
6. I then called end_day. The game ended, since day 1 was the last day.

## What happened (from the final briefing)

- A contact appeared at (7,-10), 5 hexes north, about 200 nm: "4 ships estimated, taken for carrier", later "taken for cruiser".
- VB-6 and VB-3 launched at it and bombed ship:33 with 16 and 14 aircraft. They lost 1 aircraft each. VT-3 and VT-8 did not launch, probably out of range or radius, but the briefing did not say why.
- Two Japanese raids (air_group:12 with 12 aircraft and air_group:15 with 15 aircraft) hit Enterprise. My fighters fired on them and the carrier's anti-aircraft fired too (123 shots per raid). Enterprise took 6 hits, flotation damage, fire damage, and its flight deck went to "out". It was not sunk. My fighters lost 3 aircraft.
- My claims: 2 enemy carriers claimed sunk (20 points), 51 aircraft claimed shot down, worth 2.55 points.

## What I believed at the end, and the result

- Before the end I believed I had probably sunk two Japanese carriers. The briefing's own progress line said "claims can be wrong".
- The final result, counted from the truth: Allied 6.55 points, Japanese 5.25, margin 1.3, labelled "stalemate" for both sides.
  - Ships sunk: 0 (so my two sunk claims were wrong).
  - Ships crippled: 1 carrier, worth 5 points.
  - Aircraft destroyed: 31, worth 1.55 points. The claim of 51 was too high.
- The Japanese side also got 5 points for crippling one carrier, which was my Enterprise, and 0.25 for 5 aircraft.
- The history text said Ryujo was sunk and Enterprise damaged, and that this is usually judged a limited American victory. In this run no ship was sunk.

## Confusing, missing or wrong

- A one-day game gives one set of orders and no feedback before it ends, so it is more a planning exercise than a command game. Nothing at the start said that the enemy was 200 nm north, or that a carrier would be attacked.
- The tools do not say how many days the game has until you read the briefing. They also do not say that the whole game is one end_day. A warning in the tool text would help.
- naval_strike and search both depend on guessing where the enemy is. With no enemy bases listed and no contacts, I had no way to choose search directions except a guess. A hint about the enemy's likely approach would be a fair thing to include in the OBJECTIVES.
- I could not tell why VT-3 and VT-8 did not strike. The briefing lists only the launches that happened. The "air_reach" measure answered "not known" when I gave it a hex-only destination in my first try, with no explanation. It worked later with a base handle.
- The result labelled both sides "stalemate" with margin plus 1.3 and minus 1.3, and "level: null". It was not clear what margin or level threshold makes a win.
- The claims (2 sunk, 51 aircraft) were much higher than the truth (0 sunk, 31 aircraft), and the briefing warned that they can be wrong, which was fair.
- Search arcs and the strike were set without being able to see any intermediate result, so I could not adjust, which is how the game is designed here.
- The CAP share default of 1 means no escort at all; the schema explained it, but I found it counter-intuitive.

## Refusals

One refusal: the probe at search range 9999 nm. The refusal told me the maximum (200 nm), so it told me how to fix it. No other call was refused.

## Rules

I used only `target/debug/sim-cli play` commands, run from the repository directory. I did not read or list any other file, did not use --export options, did not build anything, and did not use the web. The only extra things I did were a mkdir of the game file's directory and shell variable setup to shorten the commands. I also used the --tools flag with the first call, as the usage text shows. In my first attempt I used a shell variable that did not word-split, which failed with "no such file or directory"; it never reached the game, so nothing was added to the game file.
