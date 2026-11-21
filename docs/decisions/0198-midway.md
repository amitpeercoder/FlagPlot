# 0198: Midway, a scenario for play

Status: a plan by the architect, written on Nov 20, 2026, on the designer's word ("We could go for Midway, actually. It's a famous battle so it may create interest and engagement"). Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ). It is for the designer to confirm before any of it is built.

Summary: A third small scenario for play: Midway over four days, 3 to 6 June 1942. Three American carriers and the aircraft of Midway against four Japanese carriers, with a Japanese force bound to land on the atoll. It needs what no scenario has needed: a second map, in another sea. It is built in four stages: the map, the data, the scenario, and whether it is a contest. It uses the rules the Coral Sea called for, which are built.

Standing: Confirmed by the designer on Nov 20, 2026; stage 0, the map, is decision 0199, and stage A, the data, is gathered and ruled on (rules 50 to 72), with a second pass of research and the derivation list to come.

## Why

The designer chose it for a reason no other scenario has: people know it. The research he asked for is in (`docs/research/midway-order-of-battle.md`, 2,118 rows).

It is the Coral Sea's shape again, which the game now plays: carriers on both sides, a force of men that the attacker must put ashore, an island with aircraft of its own. The score for men landed (decision 0186), the waters struck first (0190), a lift turned back (0191) and men ashore (0193, 0194) all apply as they stand.

## The scenario

### What it is

1. **Four days, 3 to 6 June 1942,** by the clock of Midway (zone +12). The game begins at 1800 on 2 June; day 1 is 3 June, when the patrol aircraft found the transports, and day 4 is 6 June, when Mikuma was sunk. Yorktown's loss on 7 June to a submarine is outside it.
2. **A scenario for play** (decision 0187): its file says so and its scoring is its own.
3. **Scenario 114,** four files in `data/scenarios/midway/`, made by a script from one base, as scenarios 103 and 105 are.

### The forces

4. **Allied:** Task Force 16 (Enterprise, Hornet) and Task Force 17 (Yorktown), each a force of its own with its screen; Midway, an Allied base, with its garrison and its aircraft by unit and type.
5. **Japanese:** the First Carrier Striking Force (Akagi, Kaga, Soryu, Hiryu and their screen); the occupation force (the transports with the landing force aboard, and their escort); the close support cruisers (Kumano, Suzuya, Mikuma, Mogami) as a force of their own.
6. **Left out of the first version, each said in the header:** submarines, as everywhere, which here means I-168 and Nautilus; the Main Body with Yamato and the Second Fleet's main body, which did not come within reach; the Aleutians; the seaplane tender group; the flying boats of both sides if their type cannot be rated.
7. **Nothing is invented.** A ship or aircraft type is in the scenario only with figures derived from fact rows. One the rows cannot rate is left out and named, or is a marked placeholder where the scenario cannot do without it (the transports). Midway's airfield had eight types of aircraft; those that cannot be rated are left out, and the header says which squadrons that removes.

### What each side knows at the start

8. **The American side knew.** It had read enough of the Japanese signals to expect the carriers from the north-west and the landing force from the west on these days. The scenario gives the Allied side that as believed forces at the start: where it expected each, as the rows of the estimate of the time give it. The Japanese side believed the American carriers were not near, and is given nothing of them.
9. **Nothing else is given.** Each side finds the other by its own search.

### Where things start, and the clock

10. **Each start is a row's where a row gives one,** and a marked placeholder where none does. The Japanese carrier force has no track in latitude and longitude on file (its report's positions are in a grid with no key): its start is worked from the bearings and distances the American reports give of it on 4 June, back along its course at its speed, written out in the hand calculation and marked a placeholder.
11. **One clock,** Midway's (zone +12), for every time; each row on another clock is converted, and the hand calculation says so for each. The Japanese rows date the same day a day later.

### The objectives, with the reason for each

12. **Both sides:** enemy ships sunk and out of action, aircraft destroyed and men lost, at the figures the other scenarios give them.
13. **Japanese: men landed at Midway,** 1 point for each 100 men, as in scenarios 103 and 105. It is the mission.
14. **The levels are set in the hand calculation,** before any run, for this scenario's size, in terms of the landing force and the carriers, as the Coral Sea's were. The horizon is stated there with its reason.
15. **The band** of decision 0187 is read on a first run of 1,000 games of the file in which both sides are given orders, after one game is played and read.

## What the game does not model, said before a run

16. **A carrier caught with her aircraft armed and fuelled on deck.** The game has no rule that makes a carrier more easily destroyed while she is rearming or spotting a strike. That was a great part of how four carriers were lost to three in a morning. Without it the Japanese side will do better than it did. The first run will show by how much; whether to add such a rule is put to the designer with the run, and it is not added to make the band.
17. **Submarines.** Yorktown, damaged, was sunk by one. The scenario ends before it.
18. **What the staffs do** is as the Coral Sea found: a staff sent to stop a landing now strikes the waters it is told to first, and a lift is turned back when it is hurt and a carrier is near.

## Rule card

No rule of the simulation is added by this plan. Stage 0 changes the tool that makes a map and, if the engine reads anything of a map's place on the earth that the present map fixes, how it reads it; that is found in stage 0 and, if it is a rule, is written in a record of its own.

## The stages

### Stage 0: a map for another sea

19. **The map** covers every hex whose centre lies from 23 N to 38 N and from 163 E across the date line to 166 W, on hexes of 40 nautical miles, centred on a point near the middle of it. It holds Midway, Kure and Pearl and Hermes Reef, and French Frigate Shoals. Wake, Pearl Harbor and Japan are off it. It is nearly all deep sea.
20. **The map tool** makes it from the same public source as the present map, with the centre and the extent given as arguments, and with longitude counted across the date line without a break. The present map is made again by the changed tool and must come out byte for byte as it is.
21. **Whatever in the game assumes the present map** is found and listed before anything is changed: the projection's centre in the engine, day and night by longitude, the table's chart, the viewer, the tools that turn a position into a hex. Each either reads the map's own header or is told the map.
22. **Anchorages:** Midway's lagoon, as the present map's anchors are given.

### Stage A: the data

23. **A derivation list, `data/derivation/midway.toml`,** as of 3 June 1942. The team first reports a table: each ship and type, whether its class is derived today, and what the rows lack. The architect then rules ship by ship, as for the Coral Sea, and commissions research for the gaps that stop a derivation.
24. **Known gaps to press:** the Japanese carrier force's track; Enterprise's and Hornet's squadrons on hand; the Japanese carriers' air groups by carrier (the total, 234, is firm; the split is disputed); the men in each transport; the model of dive bomber the American carriers flew.
25. **Settled now by the architect:** times are zone +12. The landing force is a marked placeholder until its number is settled from the three figures on file. The Japanese report's missing tables are sought in its scan.

### Stage B: the scenario

26. **The hand calculation is the architect's,** written when stage A is in, as the doc comment of `tests/scenario_114_midway.rs`: forces, starts, what each side believes, the landing force's passage, the orders a side the engine plays is given, the objectives and the levels.
27. **The orders** are what each mission implies, and they use the staff's rules as they now stand. They are written before any game. The build then plays one game and reports it to the architect before any run of 1,000 is read.

### Stage C: is it a contest

28. **The band,** then four games between agents, then the table's list.

## What it costs

- **Stage 0 is new ground.** Nothing has run on a second map. What assumes the first is not known until it is looked for.
- **Stage A is large:** five carriers new to the game on one side or the other, and eight types of aircraft at Midway.
- **It may not be a contest as the game stands** (rule 16), and the honest mend is a rule, not a figure.
- **Starts that are placeholders,** as in the Coral Sea, for the Japanese carriers above all.

## Questions for the designer

1. **The days.** Four, 3 to 6 June; or three, ending on 5 June, which leaves out the pursuit of the cruisers. The architect proposes four.
2. **What the American side knows.** Given as believed forces at the start (the architect's proposal), or left for the player to find, which makes it another battle.
3. **A carrier caught rearming.** Build the scenario without such a rule and see the run first (the architect's proposal), or plan the rule before the scenario.

## Amended on review

A reviewer who had not seen the plan written read it against the map tool, the staff's rules, the table and the research's rows. Seventeen findings. The plan as written could not have been played: the Japanese side had no base on the map, the atoll would probably not have been land, and what it gave the American side of its intelligence does nothing in a game. Found right: the numbers 0198 and 114 are free; the extent holds every position on file; the American start is a row's; the rows support the forces named; no rule of the engine reads where a map is on the earth; the game has nothing of a carrier caught rearming.

### The map, which becomes a record of its own

29. **Stage 0 is decision 0199, a second map,** written and reviewed before it is built. It amends decision 0017, whose projection was "for the slice only", and it changes what the table's new game screen reads of a scenario (decision 0185). It can be built while stage A is done. What it must settle, from the review:
    - **The south edge is 19 N, to take in Wake.** As planned at 23 N the Japanese side had no base on the map. A lift needs a base of its side to come back to; with none the staff forms no plan for the take, and a lift turned back has nowhere to turn to. Wake is a Japanese base and the landing force's source.
    - **An atoll must be land enough to land on.** Land is sampled at points 2 nm apart, and Midway's two islands are smaller than the square one point stands for: on the present map five small islands of the Solomons sit in hexes read as deep sea. A hex that is deep sea takes no land unit, forms no base when men unload, and is no place a staff will take. The record rules how a named place with land makes its hex coast.
    - **The tool** takes its centre, its extent and its factor for longitude as arguments and counts longitude across the date line without a break; made with its old arguments it gives the present map byte for byte, checked once by hand, since its source is not in the repository.
    - **What assumes the present map,** found by the reviewer: nothing in the engine (darkness is by the clock, positions are stored and not read, distances are in hexes); the tool's `locate` and `water`; the check of passages in `sim-cli`; the viewer's labels; the words "the Solomon Islands" in the tool server's opening; a longitude of 182.6 handed to an agent; and the table, which loads one chart for all scenarios, draws it with no factor for latitude (stretched 16 in 100 at Midway's), letters every meridian east, and has its coast and names files and the script that clips them fixed to the present map.
    - **The table's list** names the map of each scenario.

### The days

30. **Five days, 3 to 7 June,** not four. The transports' passage, at the 8.8 kn the rows imply, brings them to Midway about 1635 on 6 June, an hour and a half before a game of four days ends: whether the landing fell inside the game would have turned on a placeholder speed, the fault of decisions 0184 and 0188. Five days put the planned day of landing, 6 June, a day inside. The game begins at 1800 on 2 June and ends at 1800 on 7 June, so Yorktown's sinking that morning is inside it too, though no submarine is.
31. **A game of three days was never a choice:** it ends before the day of the landing, and the Japanese side could not do what it came for.

### The starts

32. **The Japanese carrier force's start is a row's:** 37 01.5 N, 171 07 E, on course 130, at 1630 on 2 June by Midway's clock, from its own report, an hour and a half before the game begins. Rule 10 would have worked it back from what the Americans reported, which is their belief and not where it was; that is struck.
33. **The transports' start and speed** are worked in the hand calculation from the rows that give them (700 nm bearing 261 from Midway at 0925 on 3 June; 560 nm at about 0115 on 4 June), back to 1800 on 2 June, and their class is a placeholder of its own at the speed those rows imply, marked, as the Coral Sea's is.

### What the American side knows

34. **A believed force gives a side nothing in play.** No rule of the engine reads one, nor the staff, nor the briefing; only a dry run does. Rule 8 is replaced. The American side's knowledge is carried by three things the game does read: **where its carriers start** (the point north-east of Midway where the rows put them, chosen because of what was known); **the orders a side the engine plays is given** (its carriers stationed there, Midway's aircraft searching to the north-west and west, the waters to the north-west denied); and **the directive a person reads at his seat.** The Japanese side's orders and directive know of no American carrier. No contact is held at the start by either side.

### The forces

35. **The Second Fleet's main body is in.** The plan left it out as never within reach; the rows have it closing at 28 kn on 4 June to within 270 nm of the carriers by the early hours of 5 June. It was the night surface threat that turned the American carriers away. Its ships are nearly all of classes the game has figures for. The Main Body with Yamato stays out, and is said to be: it stood 200 to 250 nm from the American carriers at its nearest and her class is new.
36. **The patrol flying boats are not optional.** Without them Midway searches to 180 nm, not the 700 that found both Japanese forces. Rating the PBY is a must of stage A, with research for its disputed ranges, and the scenario waits for it as the Coral Sea waited for its torpedo planes.
37. **New to the game:** the classes of Akagi, Kaga, Soryu and Hiryu, of the cruiser Jintsu and of four destroyers; and eight aircraft types (two models of flying boat, the B-17, the B-26, two Marine types at Midway, a Japanese floatplane and one more). Every American surface ship is of a class the game has.

### The orders, for the hand calculation

38. **The Japanese carriers are named in the forces of the take,** not given an order of their own. A carrier force raids the place only while its task stands under a take; with an order of its own the four carriers would never strike Midway.
39. **The take's risk, its need to know and the estimate it holds are stated.** At balanced it wants twice the men it believes are there; Midway's garrison is some 2,200 on file against a landing force of 2,500 to 5,800, and a staff that wants more than it has does not sail.
40. **Midway's bombers strike ships only under a deny that names them;** under a hold they search and never strike.
41. **One game is played and read before any run,** as for the Coral Sea.

### What the game does not model, added to

42. **Weather.** Fog hid the Japanese carriers on 2 and 3 June. With none in the game and a search of 700 nm they are likely to be found on the first day. This pulls against rule 16: the Japanese side loses its surprise as well as its bad luck.
43. **Daylight.** Night is from 1800 to 0600 everywhere. At Midway in June there were about fourteen hours of light, and the morning's launches and first sightings fell in what the game calls night.
44. **Reach.** The Japanese torpedo plane's radius in the game is 175 nm, against a launch at 240 nm on the day, so the carriers will close further than they did.
45. **And what it does have:** while the Japanese carriers serve the take, their bombers are held to the raid on Midway for the whole of a pulse. That is a part of the bind Nagumo was in, though not the decks full of armed aircraft.

### The scoring, seen before a run

46. **One coin again, unless the figures are looked at.** Thirteen transports are worth 26 points and their men 25 to 58; all four Japanese carriers are worth 40. The size of the landing force swings the result more than every carrier.
47. **The landing force is a dispute and not a gap:** three sources give 2,500, 5,000 and 5,800. It is settled in stage A by the rule for disagreements, not left a placeholder.
48. **Men landed in the last hours score in full with no fight,** since no attack is begun in a game's last days (decision 0194).

### The stages, added to

49. **Stage B's build also needs:** the test that pins which scenario files are for play (scenario 104's) restated by the architect; frozen copies of the new figures files; the class sides file; the islands key and the directives of the table's list; this list's check among the commands before a commit.

### The questions for the designer, as they now stand

1. **Five days, 3 to 7 June.** The architect proposes it, for the reason of rule 30.
2. **The scoring.** At the game's figures the landing outweighs the four carriers. For the Coral Sea he kept it so. For Midway, the battle everyone knows as four carriers lost: keep the figures as they are and see the run (the architect's proposal), or have him propose, by a rule from the mission, figures in which the carriers count for more.
3. **A carrier caught rearming, and fog.** The game has neither. Build without them and see the run first (the architect's proposal), or plan either before the scenario.
4. **Yamato's Main Body left out.** The architect proposes it.

## Confirmed on Nov 20, 2026

The designer: "Go with your recommendations."

1. **Five days, 3 to 7 June.**
2. **The scoring keeps the game's figures,** the landing outweighing the carriers, and the first run is seen before anything is proposed.
3. **No rule for a carrier caught rearming, and none for fog, before the scenario.** It is built without them, and what the run shows is put to him.
4. **Yamato's Main Body is left out.**

## Stage A: the architect's rulings

Made on Nov 21, 2026 on the data team's report, `docs/research/midway-stage-a.md`, which lists what is derived, what is derivable and what is blocked, and every dispute with its rows. Each ruling says which rows the derivation list or the scenario reads and why. No row is changed by a ruling: the list names its exceptions, as the Coral Sea's does. Seventeen rows on file changed standing in that pass because the checker requires it once a second row agrees or disagrees; no list's derived files changed.

**The Japanese carriers' aircraft.**

50. **By carrier and type, the rows of the order of battle as it stood on the day** (air unit facts 5821 to 5857 and 12434 to 12437): Akagi 24 fighters, 18 bombers and 18 attack planes; Kaga 27, 20 and 27; Hiryu 18, 18 and 18; Soryu 18 fighters and 18 attack planes. They are low rows read by exception. The reason: the other two readings are establishments with spares and name no day, and this reading's bombers come to the 72 of the one grade A total (5864 to 5867).
51. **Soryu's bombers are 18,** not that reading's 16: the striking force's own report counts 18 operational on the morning (12403, high), and a carrier has no more aircraft fit to fly than she has aboard.
52. **The 6th Air Group's fighters are six, nine, three and three** (12425 to 12432), two rows agreeing, against the six in each of the plan of May (12414 to 12424). They are in the fighters of rule 50 as that source has them. The game has no aircraft carried as cargo, so they are fighters of their carrier.
53. **The total is not forced to the grade A figure.** The list states its sum beside the 234 and the difference stays in sight. The gap for a better source by carrier stays open, and a row from one replaces these exceptions.
54. **Soryu's two reconnaissance planes are left out,** with the B-26s and the Type 95 floatplanes (rule 60).

**The American aircraft.**

55. **Midway's dive bombers are 19 SBD-2 and 17 SB2U-3** (5750, 5751, 12660): the grade A study and the air group's daily record agree, against the commanding officer's report.
56. **Hornet's dive bombers are 35,** her own report's figure (12639, high and stated), not the study's 37. No row splits them between her two squadrons, so they are one air group of 35.
57. **Midway's flying boats are two groups by type, 14 PBY-5 and 16 PBY-5A** (5753, 5754). Which squadron had which stays a dispute the game does not need settled.
58. **The PBY-5A is rated from the Navy's sheet** (aircraft type facts 12000 to 12004), dated 1 September 1943 and read for June 1942 by written exception: no sheet of 1942 was found, and the airframe is the same. When her armour and protected tanks were fitted is a gap, and the list says so.
59. **The PBY-5 reads the same sheet, by a second exception.** It is the same aircraft without the wheels; no sheet of her own was found. This is an inference of the architect's and is marked as one. The searches flown to 700 miles (12007, 12008) are the check on both: if the rule's reach falls short of what was flown, that is reported and not fitted.
60. **Left out, as the tool has no role for them or no rows:** the four B-26s (a medium bomber has no role in the derivation, decision 0029), Soryu's two D4Y1-C, and the Type 95 floatplanes of the battleships and cruisers (no armour or tank row). Ten aircraft or so of each side's hundreds; a role for six aircraft is not worth a rule. Tone's and Chikuma's Type 0 floatplanes, already derived, do the Japanese search.
61. **The other squadrons are as the table has them.** Where a count is one of two (Yorktown's torpedo planes, 12 or 13), the list takes the ship's or squadron's own report.

**The men.**

62. **The landing force is two land units of 4,507 men: the Ichiki Detachment, 2,507, and the 2nd Combined Special Naval Landing Force, 2,000.**
    - 2,507 (land unit facts 12800) is the one figure given to a man and is a count on a day, 19 May, not a plan.
    - 2,000 (6947) is the monograph's own figure where it names the two units; its 2,800 is the plan's round figure earlier in the same document, and the 1,500 of the Marine history is the part that was to storm Sand Island.
    - The check: with the 650 construction men on file aboard one transport the force is 5,157, which is the "5,000 troops including an airfield construction unit" of two rows (12803, troop lift 12800). The 2,500 and the 5,800 are the two ends: the assault parts alone, and the plan's round figures added.
    - The construction units are not land units of the game and are not landed.
63. **Midway's garrison is 2,178 fighting men** (6907): the defense battalion and the two raider companies. The sailors, the air units' men and the soldiers of the one source that counts them are not a land unit. Rule 39's "some 2,200" is this figure and means this.
64. **The thirteen transports carry the 4,507 between them,** as a placeholder class; what each carried is a gap but for two ships. Patrol Boats 1, 2 and 34 have no class on file and are left out.
65. **The 8.8 kn of the transports stands on the positions alone** (rule 30), and the scenario file says so beside the figure.

**The ships.**

66. **Zuiho is in, with the Second Fleet's main body,** where two sources put her, one of them the Japanese monograph; and Mikazuki with her. Which of the three readings of her air group the list takes is settled when the list is written, by the rule for disagreements.
67. **The disputes of conversion** (the four carriers' speed, fuel and displacement, given in metric and long tons; Sendai's endurance; Hornet's and Pensacola's light guns) are settled in the list by its rule for disagreements, as every list's are.
68. **The Second Fleet's main body and the Close Support Group fight on the surface,** since they are the night threat of rule 35. So their classes need surface figures. Kongo and Takao have them. The Myoko class's tubes are the 16 of the fits dated from her refit of 1941 (fits 13006 to 13008), not the 8 of the class rows, which name no date. The Tone, Mogami, Nagara and Sendai classes are blocked on the torpedo's model and, for the last two, rounds per gun: a second pass of research is commissioned for those rows alone. A class still blocked after it has `surface_weapons_not_modelled`, and the report of the first game says what that took away.
69. **The American ships are as at the Coral Sea:** carriers and screens with surface weapons not modelled where rounds are missing.

**The bases.**

70. **Wake has no rows but its name and place.** The second pass gathers its airfield, its aircraft and its garrison in June 1942. Until then it is on the map and in no scenario file.

**Corrections to the rules above.**

71. **Rule 37's list is corrected:** new classes are Akagi, Kaga, Soryu, Hiryu, Sendai (Jintsu), Asashio (nine destroyers) and Mutsuki (Mikazuki), and Pensacola, which no list derives. New aircraft types are the F2A-3, the SB2U-3, the B-17E, the PBY-5 and the PBY-5A. The SBD-2, the F4F-3 and the Type 0 floatplane are derived already.
72. **Class ids from 5000 and aircraft type ids from 5100,** pinned in `data/derivation/midway.toml`.
