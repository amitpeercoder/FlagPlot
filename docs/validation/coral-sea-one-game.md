# The Coral Sea: one game before the thousand

Scenario 105, decision 0188, file a (`data/scenarios/coral-sea/01a_coral_sea.toml`): both sides given the orders of the hand calculation, staff against staff, on the live figures. Played once on its own seed (1942) with `cargo run --release -p sim-cli -- run data/scenarios/coral-sea/01a_coral_sea.toml --events`, at commit `aef5f01f` on the branch `coral-sea-scenario`, state hash layout `flagplot-state-v49`. A second run gave the same output, line for line.

This is the report the hand calculation asks for ("One game before the thousand"). It is not a test, and no run of 1,000 has been made. It changes no order, figure or point: what follows is for the architect to read before any batch, and to mend the orders if they leave out something the mission asks for. Event numbers are the game's (`#N`).

A day runs from 1800 the evening before its date: day 1 is 6 May, and its night pulse is the night of 5 to 6 May. So the clock "D4 21:42" is 2142 on 8 May.

## The first thing to read: the six Allied bomber groups never flew

The commander's order to each of the six bomber groups (2, 3, 4, 6, 7 and 8) to strike transports first was accepted, and **each order held its group** (`#20` to `#31`: "Allied holds air_group:2 by order:1", and so on). This is the override of the rulebook (chapter 8, Holds and the override): a commander's order of any kind to an air group that an intent in force may use holds the group, and "a group held as a whole is given no order of any kind by the staff". Every bomber aboard the two carriers is such a group under intent 2.

So the staff gave them nothing for five days. Its reason, every morning, is "air_group:2 is held by the commander: no order; air_group:3 is held by the commander: no order; ..." (`#47` on day 1, `#822` on day 4). They stayed on the mission they started with, rest, and ended the game on it with every aircraft ready. The Allied side flew no search and no strike, held no contact all game ("Allied: no contacts" at the end), and only its two fighter squadrons flew, on combat air patrol.

The order of what to strike first held, in the letter: the six groups had transport, carrier, battleship, cruiser, destroyer, auxiliary on their lists on every morning, and statement 19's test passes. It held because nothing touched the groups at all.

A probe, not kept and not part of the scenario, says what one change would do: the same file with a `release` command for each of the six groups on day 2 (a release lifts the hold, and the list the order set stays). The groups then searched and struck from the morning of day 2 (the first strike, `air_group:3` with 18 aircraft and four more groups, at a contact 120 nm away); the Allied side sank one carrier, and the convoy still landed all its men: Allied 13.25, Japanese 62.06, a decisive Japanese victory. Whether to give the release, or to set the lists another way, is the architect's.

## Day by day

| Day | The convoy | Task force 1 (Yorktown, Lexington) | Task force 2 (support group) | Striking Force (task force 3) | Main Force (task force 4, Shoho) | Rabaul |
| --- | --- | --- | --- | --- | --- | --- |
| 1, 6 May | Sails at once under the take: the staff finds every unit aboard and its supply aboard, 410 t of the 403.95 t it reckons for 15 days, and sends it, 492 nm at 6.5 kn (`#120`, `#122`). (-8,-9) to (-10,-7) in the night (`#219`), (-11,-6) in the morning, (-12,-5) in the afternoon. | Runs to its station: (-3,5) to (-10,1) in the night at 32.5 kn, arrives at (-12,1) at 0811 (`#176`, `#293`). Fighters on patrol; no search. | Moves on the commander's order at 12 kn (no speed was ordered: its cruise), north and then west round the Solomons side: (-3,0), (-3,-2), (-4,-3) (`#193`, `#294`, `#341`). | Runs to (-4,1), there at 0225 (`#210`). Shokaku's dive bombers search and sight task force 1 at 240 nm, reported as "17 ships, transport" (`#238`). Zuikaku's 22 dive bombers strike at it and find nothing: the force had moved on (`#243`, `#246`). | Runs to (-10,-2), Deboyne, there at 0417 (`#218`), and stays there all game. Shoho's nine B5N1 search; her twelve fighters fly patrol. | The 4th Air Group's G4M1 search (`#240`) and report task force 1 as a carrier force at (-11,1); the Motoyama group's 24 G3M2 strike at 487 nm and find nothing (`#244`, `#247`). The Tainan group flies patrol over Rabaul. |
| 2, 7 May | (-12,-5) to (-12,-3) in the night (`#414`), rounds the tip to (-13,-2) in the morning and (-14,-2) in the afternoon. Not sighted by the Allied side, not struck. | At (-12,1). Sighted by Shokaku's torpedo planes at 320 nm (`#435`), beyond the Striking Force's reach. | (-4,-3) to (-8,-3) in the night, (-10,-3), (-12,-3). Sighted (`#437`); struck by 15 G3M2 in the morning, 3 bomb hits on Australia (`#445` to `#449`), and by 9 in the afternoon, 1 hit (`#513`). Its guns bring down 5 bombers (`#447`, `#515`). | At (-4,1). Holds task force 1 at 320 nm, beyond its bombers' 175 nm. | At Deboyne. | Strikes task force 2 twice (above); the strike task flies at any ship in reach, as the hand calculation says. |
| 3, 8 May | (-14,-2) to (-15,-3) in the night, (-16,-3), (-17,-3). | At (-12,1). Struck in the morning by 54 aircraft of both Japanese carriers at 160 nm (`#664` to `#666`): Yorktown takes 4 and 2 torpedo hits and 5 bomb hits, flotation damage 77.6, propulsion out (`#677` to `#696`). The patrol and the guns bring down 20 attackers, and VF-42 loses 3 fighters (`#670` to `#686`). An afternoon strike of 31 finds nothing (`#760` to `#767`). | Arrives at (-12,-3) at 1905 on 7 May, the night of day 3 (`#617`): about a day and a half after the convoy passed it. It meets nothing and stays there. | Closes on its carrier contact: (-4,1) to (-8,1) in the night (`#587`: "at the balanced risk it closes on a carrier"), then strikes (left). | At Deboyne. | Searches; does not strike. |
| 4, 9 May | Reaches (-17,-4) and halts at 2142 on 8 May (`#884`), the hour of statement 15. Lands every man that night: 600 from each of transports 1 to 8, 86 from transport 9 and 500 from transport 10, 5,386 in all, and 108 t of supply (`#886` to `#909`). Its landing forms a Japanese base, "Landing at (-17,-4)", in the hex of Port Moresby (`#885`). | At (-12,1). | At (-12,-3). Struck again in the morning by Zuikaku's 9 dive bombers (2 hits) and 5 G3M2 (1 hit) (`#923` to `#934`): Australia's flotation damage 53.4. An afternoon strike finds nothing (`#989`). | Closes again, on the support group taken for a carrier: (-8,1) to (-8,-2) in the night (`#837`), strikes it (left). | At Deboyne. | Strikes task force 2 (left). |
| 5, 10 May | Unloading done at 1805 on 9 May (`#1105`); the force steams back east, to (-16,-3) by the end. The men ashore dig in and rest. | At (-12,1); sighted by the Japanese, not struck. | At (-12,-3). | "Holds nothing in reach and nothing to close on", so it runs back to (-4,1) (`#1053`, `#1085`). No strike. | At Deboyne. | Searches. |

## The questions of the hand calculation

**Did the convoy sail, and when?** Yes, on day 1, at the start of the night of 5 to 6 May (`#120`, `#219`). The take sent it as it stood: no estimate held, so the staff took everything the ships carried (`#77`), and the supply aboard was enough (`#120`).

**When and where did it land, and how many men?** At (-17,-4), the hex of Port Moresby, at 2142 on 8 May, in the night pulse of day 4 (`#884`): all 5,386 men (`#886` to `#906`), and over the night and the next day 108 t of the 410 t aboard (`#889` to `#909`, done `#1105`).

**What did the men do ashore, and did Port Moresby fall?** They dug in and rested and did not attack, as the hand calculation expected. Once ashore the men counted the garrison: the side's estimate was 3,444 men, so at balanced the staff wanted 6,888 and had 5,116 fit men: "the men that can attack are too few and none is on attack: the staff waits" (`#1051`). Port Moresby did not fall; the garrison, 3,000 men, lost none. Both sides end with men ashore in the one hex.

**Where did each carrier force go, and what did it strike?**
- Task force 1 ran to (-12,1) on the first night and stayed. It struck nothing and searched nothing: its six bomber groups were held (above).
- The Striking Force ran to (-4,1), sat out day 2 with task force 1 at 320 nm, closed on day 3 to (-8,1) and struck task force 1 once (Yorktown out of action, 191 days of repair); closed again on day 4 to (-8,-2) on the support group, which it took for a carrier, and struck it; ran back to (-4,1) on day 5. It was in the fight on days 3 and 4, not, as in the reviewer's trial, only on day 5.
- The Main Force went to Deboyne and stayed. Shoho's nine B5N1 were her force's search and did not strike, as expected.

**What did Rabaul's aircraft do?** Under the deny, the staff formed a task to strike ships and one to search from Rabaul (`#70`, `#73`). The G4M1 group searched every day and never struck. The G3M2 group struck every ship contact in reach: task force 1 on day 1 (nothing found), the support group twice on day 2 and once on day 4 (5 hits on Australia in all). It lost 8 aircraft. The Tainan group flew patrol over Rabaul.

**What did the support group do?** It obeyed the order, at its cruise of 12 kn, by the sea route the game gave it north of the Louisiades, and reached (-12,-3) on the night of day 3, after the convoy had passed. It never sighted the convoy and fought no surface action. It drew Rabaul's bombers and the Striking Force's on days 2 and 4, as the support group drew the land bombers on 7 May, and Australia ended out of action (112 days).

**Did the order of what to strike first hold through the staff's mornings?** Yes in the letter, on every morning, and statement 19 passes; but only because the order held the groups and the staff never gave them anything (the first section).

**What did the Allied strikes hit?** Nothing. There was no Allied strike.

**The result.** Japanese 61.16 points: men landed at Port Moresby 5,386, 53.86; Yorktown and Australia out of action over the 30 days, 5 + 2 = 7; 6 aircraft, 0.3. Allied 2.4: 48 aircraft, 2.4. No ship was sunk, no man lost. Margin 58.76 to the Japanese side, a decisive Japanese victory. The reviewer's trial had the Japanese side win by 41.56 on the landing.

## What was expected and was seen

- The men ashore count the garrison and wait for twice its number, so Port Moresby is not attacked: seen (`#1051`).
- Shoho's nine torpedo planes are her force's search and do not strike: seen.
- The Striking Force closes only on a contact it takes for a carrier: seen, on days 3 and 4 (`#587`, `#837`); this game it closed sooner than in the trial.

## What was not expected

- The six bomber groups held by the commander's order to set their lists, and never flown (the first section).
- The support group's route by the game runs north of the Louisiades, through (-8,-3) and (-10,-3), and at 12 kn it reached the convoy's route a day and a half after the convoy had passed. No speed was given in the order; the hand calculation gives none.

## The game played again, on the orders as the architect mended them

Made by the architect on Nov 20, 2026, after his rulings on the game above (decision 0188, "The architect's rulings on the one game"): the order to the six bomber groups is taken out, and the support group is sent at 20 knots. File a, seed 1942.

- **Day 1.** The Allied groups fly. In the morning 75 bombers of five squadrons go at a contact at (-9,-2), 106 nm from the carriers, by Deboyne.
- **Day 2, night.** The support group meets the convoy at (-12,-3), where it rounds the tip, and fights it for three rounds. Yubari is sunk; Perkins and Walke are sunk; the support group breaks off with three of its five ships sunk or crippled, and arrives in the hex at 23:27. The convoy steams on.
- **Day 2.** The Allied carriers strike the Main Force at Deboyne morning and afternoon: Shoho is sunk in the morning and Aoba in the afternoon.
- **Days 3 and 4.** The Striking Force closes to (-8,1) and (-8,0). The Allied strikes go on at the Main Force. Yorktown, Australia and Chicago end out of action.
- **Day 4, night.** The convoy halts at Port Moresby at 21:42 and lands all 5,386 men.
- **The result.** Allied 23.75 (Shoho, Aoba and Yubari sunk, 18; Kinugasa out of action, 2; 75 aircraft, 3.75). Japanese 66.21 (men landed, 53.86; Perkins and Walke sunk, 2; Yorktown, Australia and Chicago out of action, 9; 27 aircraft, 1.35). A decisive Japanese victory by 42.46.

The orders now do what they are for. No Allied aircraft struck a transport: the strikes went at the Main Force, 106 nm away, on every day, and the convoy passed 144 to 174 nm from the carriers unstruck.
