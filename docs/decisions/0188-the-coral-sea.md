# 0188: The Coral Sea, a scenario for play

Status: a plan by the architect, written on Nov 20, 2026, at the designer's asking ("Plan the Coral Sea scenario"). Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ). It is for the designer to confirm before any of it is built.

Summary: A second small scenario for play: the Coral Sea over five days, 6 to 10 May 1942. Two American carriers and a cruiser group against two Japanese fleet carriers and a light carrier, with a Japanese convoy bound for Port Moresby that lands on the last days if it is not stopped or turned back. It is built in three stages: the data is settled and derived first, then the scenario with its hand calculation, then its runs and games under the guards of decision 0187. Most of the ships and aircraft it needs the game already has.

Standing: Confirmed by the designer on Nov 20, 2026, with his answers; stage A's table and research are in and ruled on; its derivation list is built ("Stage A as built") and waits for the architect's reading of its exceptions.

## Why

The designer asked on Nov 20, 2026 for a second scenario of about five days, "small, yet interesting", and named the Coral Sea. The research he commissioned is in (`docs/research/coral-sea-order-of-battle.md`, 834 rows in `data/oob`). Decision 0184 gave the first, the Eastern Solomons over five days, and decisions 0186 and 0187 gave what it lacked: a score for men put ashore, and a scale of a scenario's own.

Why it should make a better game than the first. In the Eastern Solomons the convoy landed whatever the Allied side did, and the airfield could not be shut. Here the convoy is the whole question for both sides. It is slow, it must pass the Louisiades within reach of the American carriers, and it is covered by a light carrier that can be sunk. The Japanese commander can turn it back, as he did. Neither side has an airfield at the centre that decides the game by standing still.

## The scenario

### What it is

1. **Five days, 6 to 10 May 1942.** The game begins at 1800 on 5 May; day 1 is 6 May and day 5 is 10 May, the day the landing at Port Moresby was planned for. It leaves out the landing at Tulagi on 3 May and Yorktown's strike there on 4 May: by the evening of 5 May the American forces had joined south of the Solomons, and what followed is the battle.
2. **A scenario for play** (decision 0187): its file says so, its scoring is its own, and it is never a proof's scenario nor compared with the record.
3. **Scenario 105,** four files in `data/scenarios/coral-sea/`, one for each pairing the table offers, made by a script from one base as scenario 103's are. It is added to the table's list of scenarios when it has passed its band.

### The forces

4. **Allied:**
   - **Task Force 17, the carriers:** Yorktown and Lexington with their air groups, and the cruisers and destroyers of the attack group and the air group's screen.
   - **The support group:** Australia, Chicago and Hobart with their destroyers, a force of its own from the start, so that the commander may send it to the Louisiades as Fletcher did or keep it.
   - **The fuelling group:** Neosho and Sims.
   - **Port Moresby,** an Allied base with its garrison ashore.
5. **Japanese:**
   - **The Striking Force:** Shokaku and Zuikaku with their air groups, Myoko and Haguro, and their destroyers.
   - **The Main Force:** Shoho with her air group, Aoba, Kinugasa, Kako and Furutaka, and Sazanami.
   - **The Port Moresby invasion force:** Yubari and her destroyers, and the transports with the men of the South Seas Detachment and the naval landing force aboard.
   - **Rabaul,** with the land bombers and fighters there.
6. **Left out of this version, each said in the scenario's header:** Allied aircraft on land (their strengths are a gap in the facts); submarines, as everywhere; the Japanese seaplanes at Deboyne and Tulagi and the Support Force that set them up; the Tulagi force; the oilers of the Japanese supply group. A later version may add the first and the third when the facts are found.
7. **Nothing is invented.** A ship or an aircraft type is in the scenario only with figures derived from fact rows by the written rules. One for which the rows are too thin is left out and named in the header, or stands as a marked placeholder where the scenario cannot do without it (the transports, as in scenario 103).

### Where things start

8. **The Allied carriers and the support group** start in the hex of 15 00 S, 160 00 E, where the rows of two sources put the joined forces on the morning of 5 May. The fuelling group starts with them.
9. **The Japanese forces** have no stated position for the evening of 5 May: the first firm one is the Striking Force at 12 25 S, 157 35 E at 2400 on 6 May. Each Japanese start is worked back from the rows there are, at the force's speed along its known course, written out in the hand calculation, and marked a placeholder start in the scenario. The convoy starts on its passage from Rabaul, which it left on 4 May.
10. **The clock** is zone -11, the analysis of 1947's, for every time taken from a row; a row on another clock is converted, and the hand calculation says so for each.

### What each side is to do

11. **Japanese mission:** land the convoy's men at Port Moresby and sink the American carriers.
12. **Allied mission:** stop the landing at Port Moresby and sink the Japanese carriers.
13. **A side the engine plays is given** the orders its mission implies, written in the hand calculation before any run: the Japanese, to take Port Moresby with the convoy, to operate each carrier force, and to neutralise nothing; the Allied, to hold Port Moresby, to operate the carrier force, and to deny the waters south of the Louisiades with the support group. The hand calculation states them in full.

### The objectives, with the reason for each before any run

14. **Both sides:** enemy ships sunk, enemy ships out of action, enemy aircraft destroyed and enemy men lost, each at the figure scenarios 5, 7 and 103 give it (a carrier 10, a cruiser 4, a transport 2; an aircraft 0.05; 1 point for each 100 men). Reason: they are the game's figures for these things, and guard one of decision 0187 asks for an existing figure where there is one.
15. **Japanese: men landed at Port Moresby,** 1 point for each 100 men (decision 0186), the figure scenario 103 gives it. Reason: it is the mission.
16. **No objective for a base open or held.** Port Moresby cannot fall in the days of the game, and no airfield decides it.
17. **What these figures mean, said before a run.** The convoy's men are several thousand: landed, they are worth more to the Japanese side than every carrier in the game; sunk in their transports, they are worth as much to the Allied side, with the transports. A convoy turned back scores nothing for either. That is the battle as it was: the Japanese side sank more and lost, because the convoy turned back.
18. **The levels** are 3, 8 and 16, the levels of scenarios 5 and 103, set now and not moved (decision 0187, rule 15).
19. **The band** of decision 0187 is read on a first run of 1,000 games of the file in which both sides are given orders. If it is outside the band the next means is this scenario's points, once, with a reason; then the designer.

## Rule card

No rule is added. The scenario uses the kinds, intents and doctrines the game has. If the build finds that it cannot be made without a new rule (a landing against a held base that the staff cannot plan, a route the map does not give), it stops and the architect writes a record for that rule.

## The three stages

### Stage A: the data

20. **A derivation list, `data/derivation/coral_sea.toml`,** as of 6 May 1942, with the ships and aircraft of rules 4 and 5.
21. **What the game already derives** for other battles is read again for this date, not copied: Shokaku and Zuikaku, the Lexington class (as Saratoga), the Yorktown class (as Enterprise), the New Orleans class, Portland, Chicago, the Aoba and Furutaka classes, Yubari, several destroyer classes, and the F4F-4, SBD-3, A6M2, D3A1, B5N2 and G4M1. A ship's experience is worked for this date by the rules of decision 0064.
22. **What is new:** Shoho's class, Myoko's class, Hobart, Australia if her class is not Canberra's as derived, Neosho's class, the destroyer classes not yet derived, and the TBD-1, the F4F-3 and the A5M4 where a squadron flew them.
23. **The team reports, before anything is derived, a table:** each ship and type of rules 4 and 5, whether its class is derived today, and whether the fact rows for a new class are enough for every figure the rules need. The architect then settles, in the list and with the reason on each: which rows are read where sources disagree; which ships are left out; and what further research is commissioned.
24. **Settled now, by the architect:**
    - **Air group strengths.** Yorktown's are her own reports'. Lexington's and each Japanese carrier's rest on one low-grade source that quotes a standard work; they are read, marked unconfirmed, and the standard work is named as the thing to find. Shoho's are the Japanese tabular record's.
    - **The convoy's men.** The units are named and no row gives their numbers. The scenario's men are a marked placeholder until a source is read; the research is commissioned as part of this stage (the campaign volume of the strategic bombing survey and the Japanese monographs, which the first pass named and did not reach).
    - **The transports.** Two lists disagree on the names. The transports are of the placeholder transport class, so their names carry no figure; their number is settled in the list from the rows, with the reason.
    - **Disputes that set nothing in a scenario's start** (the size of the strike of 8 May, the hits on Shoho, the losses) are left as they are: rows for a later measure, not for this build.
25. **The map.** Port Moresby, the Louisiades and every position of the battle are on the present map. The team checks that the sea routes the battle needs exist on it (Rabaul to Port Moresby through the Louisiades; the Coral Sea to the Louisiades) and reports any hex that is wrong. The map is changed only by the architect's word.

### Stage B: the scenario

26. **The hand calculation is the architect's,** written when stage A is in: the forces by ship and group, the starts worked back from the rows, the convoy's passage in hours, the men and how they are carried, the orders of rule 13, and the objectives. It is the doc comment of `tests/scenario_105_the_coral_sea.rs`.
27. **The build** makes the base, the script and the four files, the test under the hand calculation, and a first run of 1,000 games with its report, as scenario 103's was made.

### Stage C: is it a contest

28. **The band** is read (rule 19) and reported with the counts behind the points: carriers sunk each side, transports sunk, men landed, games in which the convoy turned back.
29. **Four games between agents** (decision 0187, guard three), reported beside the staffs'.
30. **Then the table's list** gains the scenario, and the designer plays it.

## What it costs

- **Stage A is the larger part.** A derivation list is days of an Opus lead and Sonnet researchers, and the architect's settlements. The scenario itself is small beside it.
- **Starts that are placeholders.** The Japanese side's are worked back, not recorded. A reader should not take the opening positions as history.
- **A version without Allied aircraft on land** is kinder to the convoy than history was, though those aircraft sank nothing in the battle.
- **The convoy's men are a placeholder** until the research finds them, and they are the largest figure in the scoring.
- **It may show what the engine cannot yet do:** a staff planning a landing against a held base on a clock, or a commander's choice to turn a convoy back. What it shows is reported and not patched inside this build.

## Questions for the designer

1. **The days.** Five days, 6 to 10 May, with the Tulagi strike left out; or seven, 4 to 10 May, with it in. The architect proposes five.
2. **Allied aircraft on land left out of the first version,** to be added when their strengths are found. Or wait for them.
3. **The convoy's men as a marked placeholder** while the research is done, so that the scenario is not held up; or hold the scenario until a source gives them.

## Amended on review

A reviewer who had not seen the plan written read it against the fact rows, the map and the staff's rules, and measured the routes with the game. Fifteen findings. The plan was too hopeful in three ways: about what a staff can do with the convoy, about how much of the data is already there, and about the dates. The architect's rulings follow. Found right: the numbers 0188 and 105 are free; Port Moresby is a coast hex with sea beside it and a sea route from Rabaul; the Allied start on 5 May; that a take can launch a lift already at sea; that a landing in a hex with an enemy base forms the lander's own base there.

### What a staff can and cannot do with the convoy

31. **A staff never turns a convoy back.** Once a take's lift is routed the staff gives it nothing more, and no doctrine recalls it; only a commander can withdraw the order and send it home. So in games of staff against staff the convoy always goes on, and the band is decided by how many transports the Allied carriers sink before it lands. Rule 17's "turned back" is a person's or an agent's choice, and rule 28's count of games in which it turned back is struck from the staffs' run. A doctrine that turns back a threatened convoy would be a new rule; this plan does not add it, and names it as the first thing the scenario may show is wanted.
32. **The Japanese order, stated so that the convoy sails.** Whether the convoy sails, waits or attacks on landing turns on three things the plan left out, and the hand calculation states each before any run:
    - the convoy starts at sea with no route, so that the take can use it as it stands;
    - what the Japanese side believes of Port Moresby's garrison (a believed force, or none), and the risk of the order, since at balanced with an estimate the staff wants twice the estimate and may refuse to sail;
    - the supply aboard, so that the men landed are fed: a stated number of days for the men, a placeholder.
33. **One game before the thousand.** The build's first act with the scenario is one game on its own seed, reported to the architect: did the convoy sail, when did it land, what did the men do ashore. If the orders left out something the mission asks for, the architect mends them then (decision 0187, rule 20), before any run of 1,000 is read.
34. **Port Moresby may be attacked.** With no estimate held the men attack on landing. Rule 16's "cannot fall in the days of the game" was not checked and is struck. There is still no objective for the base: the mission inside five days is the landing. Whether the base falls in any game is reported from the first run.
35. **Rabaul's aircraft are given a task.** With nothing to neutralise and no deny, a staff leaves them idle. The Japanese side is given an order to deny the waters the convoy passes through, its forces the bombers and fighters at Rabaul, so that they strike ships that come there, as they struck the support group on 7 May. The Allied support group's waters are stated in the hand calculation and reach the convoy's route as the game routes it.

### The route and the days

36. **The game routes the convoy round the eastern tip of New Guinea,** west of the Louisiades, 652 nm from Rabaul; not through Jomard Passage. The plan's "pass the Louisiades" is to be read so. The map is not changed for it.
37. **The convoy's start and speed are the rows'.** It left Rabaul at 1600 on 4 May at 6.5 kn (the analysis of 1947), so it is about 170 nm out at 1800 on 5 May, with about 480 to 520 nm to go: about 80 hours at 6.5 kn, a landing on day 4. The placeholder transport's 10 kn would land it a day sooner and halve the days the Allied side has. The convoy sails at the row's speed; how the scenario gives it that speed (a placeholder class for this convoy, marked, with the row cited) is settled in the hand calculation. The game does not turn on an invented speed (decision 0184, amended 11).
38. **The Striking Force's start is a row's too,** not worked back: it rounds the south-east tip of San Cristobal at 1900 on 5 May, about 280 nm from the Allied start. Rule 9 is replaced: each start is a row's where a row gives one, and a marked placeholder only where none does.
39. **"10 May, the day the landing was planned for" rests on no row** and is struck. The planned date is a gap. The game ends on 10 May because five days from the evening of 5 May end there.
40. **The Allied carriers start about 590 to 660 nm from the convoy's route** and must close about 400 nm, some 16 hours at speed, and find it with their own search. That is the first two days of the game, as it was of the battle.

### The data: about half is new, and some cannot be derived today

41. **The summary's "most of the ships and aircraft it needs the game already has" is wrong.** Derived today: Shokaku, Zuikaku, the four cruisers of the Main Force, Yubari, Chicago, Portland, New Orleans, Minneapolis, Astoria, Phelps, the Farragut class, and the A6M2, D3A1, B5N2, SBD-3 and G4M1. New, with what the rows hold:
    - **Lexington and Yorktown.** Neither is "as Saratoga" or "as Enterprise": those read their own ships' fit rows. Lexington has no fit rows of her own, and the class rows would give her guns she did not carry in May 1942. Yorktown can be derived from her class rows. Each takes a class id of its own.
    - **Shoho:** her class has no fit rows, so her anti-aircraft guns cannot be derived.
    - **Myoko and Haguro:** no row for the class's endurance, which the fuel rule reads.
    - **Chester:** no class on file. **Perkins:** her class is not derived. **Tsugaru:** no class on file.
    - **The Sims class, Hobart, Australia, Neosho, and Sazanami, Ushio and Akebono:** the rows are there; each needs deriving, and a class id.
    - **The convoy's destroyers** (the Kamikaze and Mutsuki classes): no row gives the rounds a gun carried, the same gap decision 0184 met. Until one is found they stand as "Placeholder destroyers" of a placeholder class, and Yubari alone is real.
    - **Aircraft:** both American fighter squadrons flew the F4F-3, so the F4F-4 is not needed. New: the TBD-1, F4F-3, SBD-2, A5M4, B5N1 and G3M2 (Rabaul's bombers were G3M2 of one group, 24 aircraft, by the rows). That the fleet carriers flew the B5N2 in May rests on undated low rows and is checked.
42. **The land side has no rows at all.** No row for Port Moresby's garrison, for the South Seas Detachment or the 144th Infantry; Port Moresby's port and runway are a recorded gap. So the garrison's men, both sides' land units and their experience, Port Moresby's port size (which sets how fast men land) and the convoy's supply are all placeholders, each marked and named as one, until research finds them. Rule 24 said this of the convoy's men alone.
43. **Research is commissioned in stage A for the gaps that stop a derivation or leave a placeholder at the centre:** Lexington's guns in May 1942; Shoho's anti-aircraft guns; the Myoko class's endurance; Chester's and Tsugaru's classes; rounds for a gun of the Kamikaze and Mutsuki classes; Port Moresby's garrison, port and airfield; the convoy's units, men and supply; the planned date of the landing; Lexington's and the Japanese carriers' squadron strengths from the standard work.
44. **A strength on one low row is unconfirmed, and the derivation does not read such a row by default.** Reading one is an exception written in the list with its reason. For Lexington the strikes of 8 May, from high-grade rows, stand beside it as lower bounds.
45. **A ship that cannot be derived when stage A's research is in is left out and named in the header, or the scenario waits.** The architect rules ship by ship from stage A's table. The carriers are not left out: without Lexington, Yorktown and Shoho there is no scenario, and it waits for them.

### What is left out, and what that does

46. **Without Allied aircraft on land the carriers are the only Allied eyes on the convoy.** In the battle it was Army bombers that found it on 7 May. This is the largest thing rule 6 leaves out. It stays out of the first version because the strengths are a gap; if the first run shows the convoy is seldom found, that is reported and the research is pressed.
47. **The fuelling group is a target and nothing else.** The game refuels only in port, so Neosho fuels nobody at sea, and leaving out the Japanese oilers changes nothing. She and Sims stay in as what they were on 7 May: the force a Japanese search takes for a carrier.
48. **The flying boats at Rabaul and Shortland** are left out with the seaplanes, and said to be. The Japanese side then has no search from land.

### The scoring

49. **It is close to one coin, and the plan says so.** A placeholder transport of 600 men swings the margin 14 points: sunk, 6 for the men and 2 for the ship to the Allied side; landed, 6 to the Japanese. The convoy as a whole is some 50 points either way; a carrier is 10. Carriers decide a game only where the convoy is partly sunk. The figures pass guard one of decision 0187 (each is one the game already has). Whether a game that turns so much on one thing is the game wanted is the designer's question, below.
50. **The levels are not borrowed.** Rule 18 took scenario 103's, where the men were worth 15 points. They are set in the hand calculation, before any run, for this scenario's size and with the reason given there in terms of the convoy: what one transport is worth, what a third of the convoy is worth, what most of it is worth. They wait for the convoy's men.
51. **The horizon** for ships out of action is stated in the hand calculation from the length of the operation (decision 0088).
52. **The band** is read on the sign of the margin over seeds 1 to 1,000 of the file in which both sides are given orders.

### The stages, as they now stand

53. **Before stage B:** decision 0186 and the key of decision 0187 are built and merged. They are being built now.
54. **Stage A gains** the research of rule 43 and ends with the table of rule 23 and the architect's rulings ship by ship.
55. **Stage B's build also makes,** each named in the hand calculation: the class sides file for the new classes; port sizes for Port Moresby and Rabaul; the ground figures; both sides' land units and formations; the believed forces; the islands table the table's list needs; the rules switched on (fatigue, wear, supply, orders inside a day), as scenario 103 has them; the table's directives for each seat.

### The questions for the designer, restated

1. **The days:** five, 6 to 10 May, without the Tulagi strike; or seven with it. The architect proposes five.
2. **Allied aircraft on land** left out of the first version, knowing the carriers are then the only Allied eyes; or wait for their strengths.
3. **Placeholders at the centre.** The convoy's men, the garrison and the port are placeholders until research finds them, and the convoy's men are the largest figure in the scoring. Build on marked placeholders, or hold stage B until the research is in. The architect proposes: begin stage A and its research now, and decide on stage B when its table is in.
4. **One coin.** At the game's existing figures the convoy outweighs everything else. Keep it so, as the battle was; or have the architect propose, by a rule from the mission, a lower worth for men landed and lost in this scenario so that the carriers count for more.
5. **A staff that turns a convoy back** is not a rule today. Without it the engine's Japanese side never does what the Japanese commander did. Plan it as a rule of its own, after this scenario's first run; or leave it to people and agents.

## Confirmed on Nov 20, 2026

The designer answered the questions one by one, taking the architect's proposal on each.

1. **The days:** five, 6 to 10 May, without the Tulagi strike.
2. **Allied aircraft on land** are left out of the first version, and said to be. If the runs show the convoy is seldom found, the research for their strengths is pressed.
3. **Placeholders at the centre:** stage A and its research were begun on his word ("Yes, start stage A"). Whether stage B is built on marked placeholders is decided when stage A's table is in.
4. **The convoy stays decisive.** The scoring keeps the game's existing figures, so that the landing decides the game. If the first runs and the games between agents show the carriers do not matter, the architect proposes a lower worth for the men then, with a reason from the mission.
5. **A staff that turns a convoy back** is planned as a rule of its own after this scenario's first run, from what the run shows the staff does with the convoy. The first version's engine never turns back.

## Stage A: the architect's rulings

Made on Nov 20, 2026 on stage A's table and research (`docs/research/coral-sea-stage-a.md`; 259 new fact and fit rows, each checked against its page by the team). They are the rules the derivation list is built to. Each exception the list then writes names its row and its reason, and the architect reads every one before the list merges.

### How a disagreement is read

1. **A ship's own report of the time is read before any later account.** Then the higher grade of source. Between sources of one grade, the figure a source states plainly is read before one it hedges ("probably", "about").
2. **A row dated only to a year** is read as the existing lists read the same row or its like, where one does. Where none does, it is read if leaving it out would give a figure known to be wrong for the date, and the exception says so and is marked unconfirmed.
3. **A figure that rests on one low row** is read only by a written exception with its reason (`data/oob/README.md`), and the thing to find is named beside it.
4. **Where a row is missing and no rule above reaches it, nothing is made up:** the ship or type is left out and named, or the scenario waits.

### Surface fire

5. **It is modelled where a surface fight is the force's purpose,** and not elsewhere. The support group is sent to stop the convoy, so Australia, Chicago, Hobart and their destroyers, the four cruisers of the Main Force and the convoy's escort have their guns and torpedoes. The carriers and the ships of their screens do not, as in the Eastern Solomons list. Where a ship of the first kind has no rounds for a gun, her ammunition is read as not limiting, as the Savo Island list reads Yubari's.

### Ship by ship

6. **Lexington.** Her 8-inch guns are gone (the rows agree). Her heavy anti-aircraft guns are the twelve 5-inch/25 of the class row, which no row says were touched; the list reads that row for her by an exception, unconfirmed, and a ship row dated to May is still to be found. Her 1.1-inch mounts are read as seven and her 20 mm as 32, the one source that gives counts for April 1942; her own report shows 20 mm aboard on 8 May and names a mount numbered 8, which is noted beside the exception.
7. **Yorktown** is her own report of the battle: eight 5-inch/38, four 1.1-inch quads, 24 of 20 mm, 18 of .50 calibre.
8. **Chester** is read as Northampton class with her own fit; her 20 mm row, dated to the year, falls under ruling 2.
9. **Shoho.** Eight 12.7 cm; her 25 mm are read as 8, which three sources state, against 12 that one states and another calls probable. Her displacement and fuel are settled by ruling 1 in the list. Her air group is 6 A5M4, 6 A6M2 and 9 B5N: the fighters agree across sources, and nine torpedo planes is what the analysis of 1947 and one other give.
10. **Myoko and Haguro:** 7,500 nm at 14 kn, the better source of the two.
11. **Perkins, the Sims class, Australia, and Akebono, Ushio and Sazanami:** each has disputed rows; the list settles them by ruling 1 and the architect reads each.
12. **Left out of the first version, each named in the scenario's header:**
    - **the fuelling group, Neosho and Sims.** An oiler has no kind in the simulation and Neosho has no endurance or fuel in tons. She cannot be rated, and Sims without her has no task. The Japanese search then has no decoy to find, which is a loss to the game and is said;
    - **Tsugaru** (no fuel row), **Tippecanoe, Tangier and Toho Maru** (no class on file).
13. **The convoy's escort** is Yubari as rated, and "Placeholder destroyers" of a placeholder class for the Kamikaze and Mutsuki class ships, since no row gives the rounds their guns carried (decision 0184, amended 18).

### Aircraft

14. **The SBD-3's radius** is the 200 miles of the rows of August, read for May by an exception: the same aircraft with the same load.
15. **The fleet carriers' torpedo bomber is the B5N2,** on the order of battle of December 1941 and two undated rows, by an exception; no source read names the model for May.
16. **The F4F-3's range** is settled by ruling 1.
17. **The SBD-2, TBD-1, A5M4, B5N1 and G3M2 lack a range or a load.** They are researched as part of building the list. A type still without them is not flown: its squadron is left out and named. No type stands in for another.
18. **Strengths.** Yorktown's squadrons are her reports'. Lexington's and the Japanese fleet carriers' are the one low row's, by an exception, with Lexington's launches of 7 May beside them as lower bounds.

### The land side and the convoy, for stage B

19. **The landing was planned for dawn on 10 May:** three sources now say so, and rule 39's "rests on no row" no longer holds.
20. **Port Moresby's garrison** has no figure for May. The scenario's is a placeholder, named one, resting on the 3,000 of January 1942.
21. **The convoy's men** have no count from a good source. The scenario's are a placeholder, named one, resting on the 4,886 of the detachment at Guam in January and the low row for the naval landing force.
22. **The number of transports** is disputed between five and twelve. It is settled in the hand calculation from the men to be carried and the rows, with the reason.
23. **Allied aircraft on land now have a table** (1 May 1942, from the analysis of 1947, checked on the page image). The designer left them out of the first version; the rows are there for the second.

### A fault in the work, recorded

24. **Researchers saved pages and scans to a scratch folder** to search them, against their brief, which allowed reading and not saving. Nothing was committed and the files were deleted. The next brief says it again.

## Stage A as built

Built on Nov 20, 2026 by the list's lead (`data/derivation/coral_sea.toml`, as of 6 May 1942). The architect reads every exception before it merges.

### What was built

- **The list** derives 38 ships in 24 classes (ids 4000 to 4023) and 8 aircraft types (ids 4100 to 4107), and the experience of 32 ships. It writes `ship_classes_coral_sea.csv`, `aircraft_types_coral_sea.csv`, their sources and ships companions and `ship_experience_coral_sea.ships.csv`; the two figures files are also frozen in `data/reference/step1`, since `crates/sim-data/tests/frozen_figures.rs` asks a copy of every live figures file. No other list's output changed.
- **Ships:** Lexington, Yorktown, the attack group (Minneapolis, New Orleans, Astoria, Chester, Portland, Phelps, Dewey, Farragut, Aylwin, Monaghan), Fitch's screen (Morris, Anderson, Hammann, Russell), the support group (Australia, Chicago, Hobart, Perkins, Walke), the Striking Force (Shokaku, Zuikaku, Myoko, Haguro, Akebono, Ushio, Ariake, Yugure, Shigure, Shiratsuyu), the Main Force (Shoho, Aoba, Kinugasa, Kako, Furutaka, Sazanami) and Yubari.
- **Aircraft:** F4F-3, SBD-3, A6M2, D3A1, B5N2, B5N1, G4M1, G3M2.
- **Surface fire** (ruling 5) is modelled for the support group, the Main Force's four cruisers and Yubari. The tool takes `surface_weapons_not_modelled` as a list of ships, so one list does it; the others are in that list. Hobart, Perkins, Walke and Yubari have no rounds-per-gun row and are in `main_gun_ammunition_not_limiting`.
- **Exceptions:** 35, at the end of `rules.toml` under "Decision 0188", each with its row and its reason, dated 2026-11-20. One more is the list's own: the SBD-3's radius reads row 280 (`combat_radius_row`, ruling 14), since an exception cannot make a row dated 24 August hold on 6 May.
- **New rows:** `fits.csv` 1310 to 1323 (loads of the SBD-2, TBD-1, B5N1 and G3M2; the torpedo marks and tubes of the Leander, Mahan and Sims classes; Lexington's 5-inch/25; Chicago's anti-aircraft; Yorktown's 1.1-inch), `aircraft_type_facts.csv` 637 to 639 (ranges of the G3M2, B5N1 and TBD-1), `ship_facts.csv` 3777 (Shoho's completion), `sources.csv` 1539 to 1541, and five entries at the end of `gaps.csv`. Each cites a page read in this pass; the quoted words were checked against the page fetched in memory, and nothing was saved to disk.

### Readings where a ruling did not reach

1. **Lexington (ruling 6).** Her twelve 5-inch/25 are a new ship row (fits 1316) from Navypedia's armament line and its list of changes, which removes them from Saratoga only. `oob-check` makes the row preferred, since the class row 239 agrees, so the exception ruling 6 asked for is not needed.
2. **Yorktown (ruling 7).** Her report's rows (1215 to 1218) are dated 8 May only, and the tool cannot place them on 6 May. She is read from the class's eight 5-inch/38, a new ship row for her four 1.1-inch quadruple mounts (1323, Navypedia's armament line) and Navypedia's 24 of 20 mm (1221) by exception: the figures of her report. Her .50 calibre guns are machine guns, which the rule does not count.
3. **Chicago's anti-aircraft.** Neither she nor her class had a `heavy_aa` or `light_aa` row that holds on 6 May, so her anti-aircraft figure came out 0 with no problem reported, the silent zero the stage A table warned of. Three ship rows were written from Navypedia's Northampton page (1320 to 1322, as Chester's are), the 20 mm of 1942 by exception, as Chester's and Astoria's are.
4. **Australia's 2-pounders (ruling 11).** Navypedia's ship rows (573, 574) give eight barrels in two mounts of one gun; the tool reads one row for a gun, so it would count four. The class row 200 (eight, Wikipedia) is read for its count instead, which the better source's total supports.
5. **Torpedoes of Hobart, Perkins and Walke.** Their tube rows named no torpedo, so no warhead could be read. New class rows name the mark from NavWeaps (Mark IX for Leander and later cruisers; Mark 15 for destroyers) and the tubes from Navypedia: eight for Hobart, twelve for Perkins, and eight for the Sims class from summer 1940, which also settles Walke, whose as-built twelve (272) were no longer aboard. All three are inferred.
6. **Sims and Mahan classes (ruling 11).** Endurance, its speed, fuel and top speed are the Navy's fuel tables (FTP 218, grade A), read as holding on the date as the Farragut and Porter rows of the same tables are; displacement is Navypedia's, which states standard, over uboat.net's figure labelled BRT. Perkins's five 5-inch/38 (547) and seven 20 mm (549) are both read, though the same Navypedia line says most ships lost one gun when the 20 mm came: no row gives four or names her.
7. **Shoho (ruling 9).** Displacement 11,262 tons (Navypedia, a treaty figure in long tons); fuel 2,559 long tons (Nishida's 2,600 metric tons, as Yubari's fuel is read). Her 25 mm read as eight with no exception: the rows of twelve are not preferred and do not hold over the eight.
8. **Chester and Astoria (rulings 8 and 2).** Their 20 mm rows dated only to 1942 are read, as the Eastern Solomons list reads Portland's.
9. **Chicago's tubes.** None, Navypedia's removal in 1931 (327), with which the one grade A row agrees.
10. **The F4F-3's range (ruling 16).** Both rows are grade C and plain; the one read (747 nm) states the speed it is flown at. The radii differ by 4 nm.
11. **Sazanami** is of the Main Force but was Shoho's screen; ruling 5 names its four cruisers, so her surface weapons are not modelled.
12. **Worden** sailed with the fuelling group and is not in rule 4; she is left out with it.
13. **The G4M1** attacks with the 250 kg bomb, as in the Rabaul list; its torpedo row is a lone low row with no exception. **The G3M2** also with the 250 kg bomb, by exception. What each carried against the support group on 7 May is a gap.
14. **Experience** is measured by the tool to the engagement's first day, 4 May, not 6 May; no row falls between. Shokaku, Zuikaku and Shoho are new ships (1); the Japanese cruisers and destroyers take the navy's point (3); the rest are 2. Six ships are left out of the experience table because their one commissioned row is unconfirmed and the new-ship question cannot read it: Farragut, Akebono, Ushio, Sazanami, Shigure and Shiratsuyu. Shoho's date found a second source, so her two rows now stand preferred, as `oob-check` requires of agreeing rows: the one row on file whose standing this pass changed (ship facts 3461).

### Left out, each to be named in the scenario's header

- Neosho and Sims (ruling 12), Worden with them; Tsugaru, Tippecanoe, Tangier and Toho Maru (ruling 12); the Kamikaze and Mutsuki class destroyers, which stand as placeholder destroyers (ruling 13).
- **The SBD-2 (VB-2), the TBD-1 (VT-2 and VT-5) and the A5M4 (Shoho's six A5M4)** (ruling 17). The research found loads for the SBD-2 and TBD-1 (Lexington's report of 7 May) and a second range for the TBD-1, but no page read gives the SBD-2's or the TBD-1's pilot armour, the A5M4's armour or tanks, or the SBD-2's range with a load. Without them Lexington has no dive bombers but VS-2's SBD-3s and neither carrier has torpedo planes: a large change to the battle, for the architect to weigh against waiting for the research (Lundstrom's The First Team, the Bureau of Aeronautics' characteristics sheets, which the American Aviation Historical Society's site refused).

### Not done

- `CLAUDE.md`'s list of commands before a commit does not name this list's `--check`; it is the only place the other lists' checks are run, and was left alone as the brief said.
- No scenario, no class sides file, no change to the simulation's rules.

