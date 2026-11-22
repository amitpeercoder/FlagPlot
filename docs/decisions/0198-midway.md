# 0198: Midway, a scenario for play

Status: a plan by the architect, written on Nov 20, 2026, on the designer's word ("We could go for Midway, actually. It's a famous battle so it may create interest and engagement"). Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ). It is for the designer to confirm before any of it is built.

Summary: A third small scenario for play: Midway over four days, 3 to 6 June 1942. Three American carriers and the aircraft of Midway against four Japanese carriers, with a Japanese force bound to land on the atoll. It needs what no scenario has needed: a second map, in another sea. It is built in four stages: the map, the data, the scenario, and whether it is a contest. It uses the rules the Coral Sea called for, which are built.

Standing: Confirmed by the designer on Nov 20, 2026; stage 0, the map, is decision 0199, built; stage A, the data, is gathered, ruled on (rules 50 to 80) and derived (`data/derivation/midway.toml`); stage B's rulings are rules 81 to 102 and its hand calculation is `tests/scenario_114_midway.rs`, written on Nov 22, 2026; stage B is built and its first game played ("Stage B: as built"), the band waiting on the architect's reading of that game.

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

## Stage A: rulings on the second pass

Made on Nov 21, 2026 on the second pass of research and the derivation list (`data/derivation/midway.toml`, 37 classes from 5000, 13 aircraft types from 5100, 28 exceptions). The list is read and stands as written. These rulings amend rules 50 to 72 where the new rows or the tool require it.

73. **The Japanese carriers' aircraft are restated on a better source** (air unit facts 14200 to 14216, grade B, a count of 27 May): each carrier's own 18 fighters with the 6th Air Group's beside them. Fighters are Akagi 24, Kaga 27, Soryu 21 and Hiryu 21; bombers 18, 18, 16 and 18; attack planes 18, 27, 18 and 18. Kaga's two spare bombers are not flown. Hiryu has no row in that source and keeps rule 50's rows, with the 6th's three fighters added as her sister's are. This replaces rule 50's figures.
74. **Rule 51 is withdrawn: Soryu's bombers are 16.** Two sources now state 16 aboard, one of grade B. The 18 of the striking force's report is an inferred reading of an organization table, which may give the planned strength and not a count; a stated count outweighs it. The row stays, disputed.
75. **Zuiho's group is 6 A5M4, 6 A6M2 and 12 B5N2** (14220 to 14222), the higher grade of source, as the list has it.
76. **The PBY-5 is rated from her own rows** (aircraft type facts 12005, 12006 and 314), grade C, by exception. Rule 59 is withdrawn: an exception cannot read another type's row, and should not.
77. **The PBY-5A waits for a rule of the tool.** The Navy's sheet is of 1943 and the tool reads no row dated after a list's date, rightly: a later fit must not leak back. Reading a later row for what did not change (an airframe's range) needs a way to say so, row by row, with its reason. That is decision 0200, a small change to the derivation, planned next. Until it is built the sixteen PBY-5As are not in the list, and the scenario is not built without them.
78. **The six TBF-1s at Midway are left out,** with the B-26s of rule 60. Their torpedo can be read only by an exception that would change the Eastern Solomons' derived sources, and six aircraft are not worth that.
79. **The transports' close screen fights no surface action** (surface weapons not modelled), with the carriers' screens. Of the night threat of rule 68, the Second Fleet's main body is modelled in full but for Yura and Mikazuki; the four cruisers of the Close Support Group are not, since no row names their torpedo in June 1942. The first game's report says what that took away.
80. **Wake stays out of the scenario file.** The second pass found its air units by day but no garrison figure for June and nothing on its runway. The Japanese forces start at sea, as rule 29 has them; what they do for a port is the hand calculation's to say.

## Stage B: the architect's rulings for the hand calculation

Made on Nov 22, 2026, on a brief of what the repository holds for the scenario, gathered by a team and checked against the staff's code; then read by a reviewer who had not seen them written, and amended on that reading ("How stage B changed on review", at the end). They settle what rules 26 to 49 and 80 left to the hand calculation.

### A base for the Japanese side

81. **Wake is in the scenario, as a Japanese base. Rule 80 is withdrawn.** Rule 29 said a lift needs a base of its side to come back to, and the staff's code agrees: with no Japanese base on the map the take forms no order at all (`sim-staff`, `lifts.rs`, `take.rs`). Wake has its name and place from a row (5107) and nothing else: no aircraft, no airfield, no land unit, no fuel, and 100 t of supply that is a marked placeholder. Its air units of June are on file by day and are left out, their type being in no list; the header says so.
82. **The landing force did not sail from Wake,** but from Saipan and Guam, which are off the map. Wake is where a lift that is turned back goes, and where the staff counts the take's way home from. The header says so. A base there also draws each side's carriers' search toward it, since a carrier force points its search at the enemy's bases; that is a thing of the game and not of the battle, and the first game's report says what it did.

### The starts

83. **The invasion force starts at hex (-17,4),** worked from rows 7699 and 7706 as rule 33 says: 700 miles on bearing 261 from Midway at 0925 on 3 June and 560 miles at 0120 on 4 June are 140 miles in 15 hours 55 minutes, 8.8 kn; back 15 hours 25 minutes from 0925 to 1800 on 2 June is 135.7 nm more, 835.7 nm from Midway on bearing 261. The bearing is laid off in the map's own flat frame, where that point is 607.6 nm west and 146.8 nm south of hex (0,0), in hex (-17,4). The miles are read as nautical. Row 7706 gives no bearing and is read on row 7699's. The pilot's "speed 19" in row 7699 is his estimate and is not the speed made good, which is rule 65's.
84. **The American carriers start together at the rendezvous of 2 June, hex (15,-6)** (rows 5737 to 5739), which is rule 34's point.
85. **Gwin is left out.** Both rows have her joining on 5 June (8582, 8663), and where she was on 2 June is on no row; she came from Pearl Harbor, which is off the map.

### The forces

86. **The Japanese transports, their close screen, the Close Support Group and the Second Fleet's main body are one task force, the Midway Invasion Force,** 46 ships, at the transports' 8.8 kn. They were one command, the Second Fleet's; a row has the Close Support Group leaving the Marianas in company with the transports and keeping 50 to 75 miles ahead (9296), and no row places the main body before noon on 4 June. As forces of their own the staff's rules have nothing for them that the battle had: sent to stand at a station they go at full speed, arrive two days before the transports and lie stopped, and the Close Support Group's destroyers run out of fuel on the way. In one force they are where the record has them, with the transports, and what they add is their guns against aircraft and against any ship that comes to them. The night surface threat of rule 35, the main body closing on the American carriers at 28 kn, is so not in the game, and the header says it.
87. **Zuiho and Mikazuki are left out. Rule 66 is withdrawn for the scenario.** The staff's rules give a two-ship carrier force with one group of attack planes nothing to do but search with them, and wherever it is put it either lies out of all reach or stands in front of the transports as the first target of Midway's bombers. She flew no strike in the battle. The header says she is left out.
88. **The transports are thirteen ships of a placeholder class and are named so,** "Placeholder transport 1" to "13", as the Coral Sea's are: a real ship is not given an invented rating. The header names the thirteen ships they stand for and says that which was the thirteenth is disputed (5880 against 5968).
89. **The two land units are aboard that force,** 4,507 men in thirteen transports of 600 berths; which ship carries which men is not said.
90. **VB-6 flies the SBD-3.** Its rows say SBD-2 and SBD-3 mixed with no split (12600 to 12603), so the group cannot be parted as the squadrons of rules 55 and 57 are; the other three dive bomber squadrons of the two sister carriers flew the SBD-3. The header says the split is a gap.
91. **Tone and Chikuma carry three Type 0 floatplanes each,** by the only rows that give a number carried by type (12466 to 12488, low); a row of higher grade counts four of Chikuma's aircraft of all types in use on 4 June (12412) and does not say of what type. They are the Japanese carrier force's search.
92. **Of VMSB-241's seventeen SB2U-3s twelve are ready and five are damaged,** as the row's own note has five out of commission on 31 May (5751).
93. **Ships' experience is derived if the list can derive it,** by a block for this battle as the Coral Sea's list has one; if the rows on file do not give it, every ship has the value the loader gives where none is set, the header says so, and the build reports which. Air groups have no experience in the game (decision 0064).

### The bases' stocks

94. **Midway's supply is 4,000 t, a marked placeholder.** The rows say the island was heavily stocked with bombs and fuel (6008, 6031) and give no tons. The figure is set so that the stock is not what stops an aircraft. At the game's figures (`air_supply.toml`) five days cost about 2,000 t: upkeep of 111 aircraft at 0.2 t a day, 111 t; the flying boats' search once a day, charged out and back, 14 x (0.5 + 0.25 x 17.96) + 16 x (0.5 + 0.25 x 14.74) = 137 t a day, 685 t; the fighters' patrol about 70 t; the bombers twice a day at about 400 nm, about 1,100 t; the garrison, 54 t. A bomb that hits supply destroys 50 t, and a base with none flies nothing: 4,000 t is twice the five days.
95. **Every ship starts with her fuel full, and each transport carries 41 t of supply,** the Coral Sea's placeholder. No base has fuel for ships and no force has a home port, so no force's fuel is weighed by the staff; the oilers are left out. A destroyer that steams at full speed for long can run dry, and the first game's report says whether any did.

### The orders

96. **The take of Midway is at the staff's own risk, balanced, with no need to know, and the side is given no estimate.** The Japanese sailed with no count of the garrison from the air, and the order says so: a take that need not know goes on no count. No row gives what they believed of the garrison when they sailed: row 6908's 750 is their estimate of the reinforcement sent, by its own note, and row 6909's 5,000 is dated 4 June, after the game begins. With no estimate held a take takes all its ships carry. If the staff comes to hold one from the air, at balanced it wants twice that many men; and a lift that is hurt with a carrier near is turned back (decision 0191), which is what was done on 5 June. Press would have switched that off, and would have set the carriers' own risk with it.
97. **Midway's hold names its garrison, its fighters and its two groups of flying boats.** A hold whose forces limit names only the men leaves the fighters with no patrol, and a flying boat that carries a torpedo is taken for a bomber by any order to strike ships that may use her. Named in the hold, the fighters patrol and the flying boats search. The night torpedo attack of the flying boats on 3 June is so not in the game.
98. **The waters denied are those within 4 hexes of (1,-1), north-west of Midway, and the order names the dive bombers and the B-17s.** A base's search looks toward the waters its side denies. Waters all round Midway would spread the flying boats over the whole circle, where two groups find a force 600 nm out less than one morning in ten; these waters lie from north of Midway round to the west-south-west, where the carriers and the transports came from and where the flying boats in truth were sent. The bombers strike any ship they can reach, carriers first (rule 40).
99. **Each American carrier force is under its own order to operate,** Task Force 16 from Point Luck, (14,-6), and Task Force 17 from the rendezvous, (15,-6).

### The levels

100. **Minor 5, major 15, decisive 30, over a horizon of 66 days.** Five points are a carrier out of action; fifteen are a third of the landing force ashore, or lost; thirty are two thirds. The Coral Sea's minor level was one transport's men; here it is a carrier's, since rule 14 asks for levels in terms of the landing force and the carriers. The horizon is the days from the game's beginning to the landings on Guadalcanal, the next operation the carriers fought in: a carrier out for those days missed it. They are set before any game is played and are not moved to enter the band.

### What to expect, said before any game

101. **The landing is likely in most games, and the game then turns on the carriers.** By the engine's own way the invasion force reaches Midway about 1600 on 6 June, the afternoon of day 4. The staff begins no attack ashore with fewer than three days left, so the men land with no fight and score in full, 45 points, unless the force was turned back or sunk. Nothing on the Allied side is ordered to strike transports before carriers. Four Japanese carriers sunk with their aircraft are about 52. This was put to the designer only as rule 48; it is put to him in full with the first game.
102. **The Japanese carriers will probably be found first.** Under the take the carrier force goes at full speed to a hex about 160 nm north of Midway and is there by the first evening, a day earlier and far closer than it was. Its floatplanes look toward Midway. The American carriers' search looks toward Wake, which from their station lies across the Japanese carriers. This is reasoning from the rules, not a run, and the first game's report says what happened.

### How stage B changed on review

A reviewer read rules 81 to 99 as first written and the hand calculation against the staff's rules and code, the loader, the map and the rows.

- **The take was at risk "press".** Press also sets the carriers' risk (a quarter of their fighters on patrol, strikes at extended reach, closing on any contact) and switches off the lift turned back, the rule that does what was done on 5 June. Its reason did not hold: a take that need not know sails at balanced, the 750 of row 6908 is not an estimate of the garrison, and no row has the failed reconnaissance the rule named. Rule 96.
- **The two surface forces were sent to stand at stations.** They would have gone at full speed, lain stopped for four days off the transports' way, and one would have run its destroyers dry. Rule 86.
- **Zuiho had a force of her own.** It would never have struck, and would have been the bombers' first target. Rule 87.
- **The waters denied were all round Midway.** Rule 98.
- **The transports had real names and a made-up class.** Rule 88.
- **The passage was reckoned on the count of hexes,** 880 nm, where the engine charges 827: the transports arrive on the afternoon of day 4, not in the night of day 5.
- **Midway's stock was 2,000 t on working not shown;** the working comes to 2,000 t for the five days with nothing to spare. Rule 94.
- **Left unsaid for the build:** fuel aboard, supply aboard, that the invasion force starts stopped, Wake's fuel, the order of the air groups. The hand calculation now says each.
- **What to expect was not said.** Rules 101 and 102.

## Stage B: as built

Built on Nov 22, 2026 on the branch `midway-build`, to rules 81 to 102 and the hand calculation in `tests/scenario_114_midway.rs`. No rule of the simulation was added or changed.

**What was built, and where.**

- **The transports' class:** class 19, "Placeholder transport, 8.8 knots", `data/reference/ship_classes_placeholder_transport_8_8_knots.csv`: the Coral Sea's class 14 with a cruise and a top speed of 8.8 kn, 600 berths, marked a placeholder, its header saying that the speed stands on rows 7699 and 7706 alone.
- **The file of sides:** `data/reference/class_sides_midway.csv`, a row for every class of the list (5000 to 5036, Zuiho's, Gwin's and Mikazuki's among them though no ship of the scenario is of them), every type (5100 to 5114) and class 19 (both sides).
- **Ships' experience (rule 93):** derived, by an `[experience]` block in `data/derivation/midway.toml` for the scenario's ships (`ship_experience_midway.ships.csv`), engagement `midway`, windows to 4 June 1942. Worden is left out of the block: her one commissioning row (ship_facts 202) is unconfirmed and gives a year only, and with her in it the tool reports a problem; Farragut was left out of the Coral Sea's block for the same reason. She has the game's own value for a ship not answered, and the header says so.
- **The list's header** on the Japanese carriers' aircraft is corrected to rules 73 and 74 (93, 70 and 81, 244 in all). No generated file changed for it. `oob-derive -- data/derivation/midway.toml --check` passes.
- **Frozen copies** in `data/reference/step1/` of the new class file and the file of sides, and the frozen `aircraft_types_midway.csv` brought up to the live one (types 5113 and 5114), each with its line in that folder's README.
- **The scenario:** `data/scenarios/midway/base_midway.toml` and its four files `01a` to `01d_midway.toml`, made by `scripts/midway/make_files.py [--check]` as the Coral Sea's are. 92 ships, 4 task forces, 2 bases, 3 land units, 32 air groups, ids in the order of the hand calculation. Every ship's fuel is her class's full figure.
- **The tests:** `tests/scenario_114_midway.rs`, a test for each of statements 1 to 26 and the four properties; all pass, none ignored. Scenario 104's statement 23 is widened to these four files and the table's copies of b, c and d: its doc comment gains one sentence in the form of the earlier additions, and its test the new files.
- **The table:** an entry `midway` in `table/scenarios.json`, after the Coral Sea, on `data/map/midway.csv`, with its seven pairings; its directives `table/directives/midway_allied.json` and `midway_japanese.json`; the table's copies of files b, c and d in `data/scenarios/table-hunger/`, by `scripts/table-copies/make_copies.py`; and `table/check_screen.py`'s check of the listed order and counts extended for it. The check passes.
- **The first game:** `docs/validation/midway-first-game.md`.

**Readings of the build, where the hand calculation is silent.** Each follows the Coral Sea's base file, and each is the architect's to overrule.

- The figures files are the Coral Sea's (the live search, surface and air combat, upkeep, fatigue, air supply, ground and trial figures, the port sizes), and `orders_inside_a_day = true`, as in scenarios 103 and 105. Command levels are off.
- Midway has the airfield a base has by default, and 4 aircraft repaired a day, a placeholder: the figure the Coral Sea and the 39 days give Rabaul, their base with aircraft (Port Moresby had no aircraft and no figure). No anti-aircraft figure (a gap, as the hand calculation says).
- Wake has `airfield = false`, for rule 81's "no airfield".
- The ground figures are the live `ground.toml` in the scenario's files, and `ground_hunger.toml` in the table's copies (decision 0205), as for every file with land units.
- The island table names one island, Midway, of base 1, for the table's list.
- The Allied deny's targets limit has no `waters_first`: the hand calculation names none.
- Ships' names: "USS" before the American names and none before the Japanese, as in the Coral Sea's file. The land units are named for their units: "The garrison of Midway", "Ichiki Detachment", "2nd Combined Special Naval Landing Force".
- The history for the end paper: "Akagi, Kaga, Soryu and Hiryu and the cruiser Mikuma were sunk. Yorktown and the destroyer Hammann were lost. The invasion force turned back on 5 June and landed no one at Midway." The missions: Allied "Hold Midway and sink the Japanese carriers."; Japanese "Land the invasion force's men at Midway and sink the American carriers."

**What did not hold.** Every statement holds as written. Four were read, and the reading is the test's: statements 14 and 16 count the steps of the agent's sea route as the hexes after the start; statement 18's "sailed for Midway" is that task force 4 has left (-17,4) and its route's first waypoint is Midway (its route goes on to Wake, the way home); statement 19's "waters denied" is the deny's area, within 4 of (1,-1), where each of the three hexes is exactly 4 away.

**The first game** (file a, seed 1): a decisive Japanese victory, 49.41 to 7.6. All 4,507 men landed at 1559 on 6 June and after, with no fight ashore. No carrier was sunk; Akagi was put out of action by Midway's bombers. The American carriers found the Japanese carriers on 4 June but never struck: they decide to close only at a day's start, on a contact held as a carrier, and closed once, to 208 nm, beyond the SBD-3's 200. The Japanese carriers never found the American carriers: their search looked only toward Midway. Rule 101's landing held to the hour; its "the game turns on the carriers" did not, because there was no carrier battle. Rule 102 held. The band of 1,000 games has not been run: it waits for the architect's reading of this game.

## Stage B: the architect's rulings on the build and the first game

Made on Nov 22, 2026 on the build's report, the first game's report (`docs/validation/midway-first-game.md`) and the scenario's header.

1. **Every statement of the hand calculation holds,** and the build's readings are accepted: the Coral Sea's figures files, orders inside a day, Midway's four aircraft repaired a day as a placeholder, Wake with no airfield, the names, and Worden left at the loader's experience for want of a row that dates her.
2. **The first game is a decisive Japanese victory, 49.41 to 7.6, with no carrier battle.** All 4,507 men landed, to the hour rule 101 gave; no carrier was sunk on either side; the American carriers launched no strike in five days. Rule 102 held in full: the Japanese carriers were at their hex north of Midway by the first evening and were found first, by a search that Wake pointed across them.
3. **Why the American carriers never struck, as the game's own record has it.** The staff is asked once a day and closes only on a contact it then holds as a carrier: the sighting of 4 June came a pulse after that day's asking. On 5 June it closed on a report one hex off and stopped at the first hex within its torpedo planes' 175 nm of that report, 208 nm from where the carriers were, eight miles past its dive bombers' reach. On the other days the search called the carrier force battleships or cruisers, and at balanced it does not close on those.
4. **Nothing is changed for it.** The scenario is merged as it was specified before the game, and the report stands as written. These are things of the staff's rules, not of this scenario's data, and the same rules play the Coral Sea and the Eastern Solomons.
5. **What is put to the designer, with the band:**
   - **The American carriers' station.** The orders hold them at the point where they waited on 2 and 3 June. The rows have them steaming south-west through the night of 3 June to be about 200 nm north of Midway at dawn on the 4th (5743, 5747): an order to operate from there is what they did, and would put them in reach on the morning the battle was fought. It was not in the orders written before the game, and changing an order after a game is seen is for him to allow.
   - **A staff that closes inside a day,** on a sighting of that morning, as a commander would.
   - **A margin on the hex a carrier force closes to,** so that a report a hex off does not leave it just out of reach, and closing by the reach of the aircraft that will strike.
   - **The scoring,** in which the landing alone is a decisive victory.
6. **The band, read on Nov 22, 2026: far outside.** Over seeds 1 to 1,000 of the file in which both sides are given their orders, at the commit of the build, the Japanese side is ahead in 983 games and the Allied side in 17; 947 are decisive Japanese victories. The counts behind it:
   - **The landing:** every man lands in 969 games, some in 12 and none in 19, in which the invasion force is still on its way when the game ends. It is never turned back and never sunk; a transport is sunk in 17 games.
   - **The Japanese carriers:** none sunk in 436 games, one in 535, two in 29, never more. Akagi is the one sunk in 564 games and Kaga in 29; Soryu and Hiryu are never sunk. One or more is out of action and afloat in 787.
   - **The American carriers:** none is ever sunk or put out of action. A group of theirs launches at ships in 780 games.
   - **No Japanese carrier group ever strikes an American carrier force.** Every launch of theirs, in every game, is at Midway.
   - **Aircraft lost, the mean of a game:** 104 Allied and 46 Japanese.
7. **So the first game was the common game in its result and not in its detail:** the American carriers do strike in most games, and sink a carrier in more than half. What does not vary is that the Japanese carriers never look for them, never find them and never strike them, and that the landing is made. The game as the staffs play it is the landing against one carrier.
8. **The scenario is merged and listed at the table all the same, on the designer's word to add Midway.** The Coral Sea was listed only when it was inside the band; this one is listed outside it, and that is said here and to him. A person in the Allied seat plays against these Japanese orders, not against the Allied staff's.
9. **Nothing is changed to enter the band.** What may be done is rule 5's list, to which the band adds one: **the Japanese carriers' search,** which under a take looks only at the place to be taken.

## The American carriers' station moved, on the designer's word

On Nov 22, 2026 the designer, shown the band of 983 to 17 and rule 5's list, said to make Midway a contest as the architect had suggested: the American carriers' station and the Japanese carriers' search.

103. **Task Forces 16 and 17 are both ordered to operate from hex (9,-5). Rule 99 is amended.** It is the hex of 31 30 N, 176 30 W, the position Task Force 17's commander chose for the two forces at dawn on 4 June, 202 miles from Midway on bearing 013 (engagement positions 5744 and 5745). Rule 99 held them where they waited on 2 and 3 June. They start where they did, at the rendezvous, and steam to the station as the staff sends them; they are so at it a day before they were, as the Japanese carriers are at theirs.
104. **The band with that one change, read the same day:** over seeds 1 to 1,000 the Japanese side is ahead in 867 games and the Allied side in 132, and one is level; 724 are decisive Japanese victories. It is still outside the band, and it is now a carrier battle:
   - an American carrier group launches at ships in 999 games, where it did in 780;
   - a Japanese carrier group attacks an American carrier in 454 games, where it never did;
   - a Japanese carrier is sunk in 891 games (one in 763, two in 111, three in 16, four in 1), and an American one in 255 (one in 207, two in 48);
   - of the Japanese carriers sunk, Akagi is the one in 891 games, Kaga in 128, Soryu in 17 and Hiryu in 1.
105. **The Japanese carriers' search is not changed, and the reason is the run.** With the American carriers at their station the Japanese find and strike them in nearly half the games as the rules stand: the station lies inside the arc their floatplanes fly toward Midway. A rule to widen that search would send the result further from the band, not toward it, and is left until the rest is settled.
106. **What the run now points at,** put to the designer:
   - **Nearly every American strike falls on one carrier.** Akagi is sunk seven times as often as Kaga and fifty times as often as Soryu. On the day three squadrons struck three carriers. How a strike's aircraft are shared among the ships of the force they find is to be looked into before anything else, since it is a thing of the engine and not of this battle.
   - **The landing is made whatever happens to the carriers,** and is 45 of the points. On the day it was called off when they were lost (rule 96 leaves that to decision 0191, which turns back a lift that is itself hurt, and it never is).
   - **A carrier caught with her aircraft armed on deck** (rule 16), which the designer said to see the run before deciding.
