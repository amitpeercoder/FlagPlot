# 0198: Midway, a scenario for play

Status: a plan by the architect, written on Nov 20, 2026, on the designer's word ("We could go for Midway, actually. It's a famous battle so it may create interest and engagement"). To be read by a reviewer who has not seen it written, and amended on that reading. It is for the designer to confirm before any of it is built.

Summary: A third small scenario for play: Midway over four days, 3 to 6 June 1942. Three American carriers and the aircraft of Midway against four Japanese carriers, with a Japanese force bound to land on the atoll. It needs what no scenario has needed: a second map, in another sea. It is built in four stages: the map, the data, the scenario, and whether it is a contest. It uses the rules the Coral Sea called for, which are built.

Standing: Planned on Nov 20, 2026; waits for its review and for the designer.

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
