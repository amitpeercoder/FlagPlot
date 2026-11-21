# 0206: Real oilers

Status: a plan by the architect, written on Nov 21, 2026 on the designer's word "real oilers", for stage B of refuelling at sea (decision 0201, rule 28). It was read by a reviewer who had not seen it written, and this is the plan with that reading folded in. Its hand calculation waits for the research still in hand.

Summary: Refuelling at sea is built on placeholder ships, and no scenario a person can play has an oiler. This makes the American fleet oiler of 1942 a real class, derived from fact rows as a warship is, and puts the oilers the record names into the Coral Sea and the five days of the Eastern Solomons, in scenario files beside the ones the closed proofs play. The Japanese oilers, Tippecanoe and the 39 days are left for later, each for a stated want of facts.

Standing: Planned on Nov 21, 2026; its review folded in; waits for the research in hand, then its hand calculation and its build.

## Why

What is so today:

- **The derivation makes warships only.** It maps six types to four kinds and fails on any other; cargo is fixed at 0 (`crates/oob-derive/src/derive.rs`). Every transport in the game is a hand-written placeholder. Decision 0166 ruled how a transport is to be derived (kind, toughness from light displacement, cargo from a stated capacity, no guns) and that part was never built.
- **Stage A's oiler is a placeholder class** with the mark `fuels_at_sea`. The loader already accepts a class file with no column for the mark.
- **The oilers are named in the record and left out of every scenario.** The Coral Sea file says why: "an oiler has no kind in the simulation".
- **No carrier force in the Coral Sea or the Eastern Solomons has a home port.** By decision 0201, rule 20, the staff gives such a force the order to fuel from an oiler's force when a ship of it falls below a quarter. So once an oiler is on the map the staff uses her unasked.
- **The research of Nov 21, 2026** (`docs/research/real-oilers-1942.md`, pull request 629) gives for the Cimarron class: her own bunkers, 14,850 to 15,000 barrels; light displacement, 7,256 or 7,470 tons; trial speed 18.3 knots; and cargo by six statements that are different quantities. It found no endurance at a speed for any oiler, no factor from barrels to tons stated by the Navy, and for the Japanese oilers no bunkers, displacement or range at all.

## The rules

### The derivation

1. **A class of the type `oiler` is of the kind auxiliary and carries the mark.** Its derived class file has the column `fuels_at_sea`. The loader reads each class file by its own header, so a file of oilers with the column and files of warships without it load together, and no class file on file changes.
2. **Toughness is read from the light displacement,** as decision 0166, rule 2, has it for a transport, divided by the same figure as a warship's standard displacement.
3. **She has no main battery and no torpedo tubes,** by her kind, as decision 0166, rule 5, has it: the derivation writes none for a class of the kind auxiliary and does not look for a fit, a shell or an outfit. This is the ruling of that decision and is not the list's "surface weapons not modelled", which is an assumption of a scenario.
4. **Her anti-aircraft guns are read as any ship's are,** from fits, as the same rule 5 says. Radar is read as any ship's is: a sourced fit, a sourced none, or the list's working assumption that unknown means none.
5. **Speed is the official history's line,** 18 knots, as decision 0166, rule 7, has it where an official line and a later compilation differ, by a written exception for the class. Every speed row of the class is disputed, so nothing is read without one.
6. **Her own fuel** is a new numeric attribute, the fuel a ship steams on stated in barrels, made tons by the factor of rule 8.
7. **Her cargo of oil** is a new numeric attribute, the oil a ship can carry for others, in barrels, made tons by the same factor. It is a capacity: a load carried on one voyage is not read (decision 0166, rule 3).
8. **Barrels are made tons by the Navy's own factor for its fuel oil, read from a row,** as decision 0166, rule 8, requires, with no other way. The factor has a file of its own, `data/oob/unit_facts.csv`, where one row is one source's statement of a factor between units. No oiler is derived until that row stands.
9. **Endurance is read as any ship's is,** a distance with its speed, as numeric rows.
10. **A row dated after the scenario is read only by a written exception** that says it was already so, as the data rules have it. There is no reading of "the nearest year".

### How the Cimarron class comes out

Each of these is settled by the architect with its reason on the row or in the list. The hand calculation works them once the research in hand has reported, and any of them may change on what it finds.

11. **Light displacement: 7,256 tons.** The class's two rows are disputed, 7,256 and 7,470, and the official history prints each for a different ship. The 7,256 is read, by an exception; the two differ by under 3 in 100 and toughness is their twenty-fifth part.
12. **Her own fuel: 15,000 barrels; Cimarron's own 14,850 for her.** They are the Navy data book's lines of 1945, read by an exception as already so: they are her hull's bunkers, and no source read says they were altered.
13. **Cargo of oil: the cargo oil of November 1943,** 101,638 barrels for Platte and 103,233 for Cimarron, read by an exception as already so. It is the statement nearest in time that is a capacity, and the loads of July 1942 sit under it: 93,000 barrels taken by Platte, and 12,000 tons of fuel oil each. The Navy's 1945 table of fuel-oil tanks, 68,054 barrels, is not read: by its own footnote it is how the tanks were allotted to products in design, and Platte carried more than it in 1942.
14. **Neosho has no cargo row of her own;** she was lost in May 1942. She reads the least of her sisters' rows, 101,638 barrels, by a rule written in the list: a ship with no row of her own for a figure her sisters differ in reads the least of theirs. Her bunkers read the four ships' 15,000 barrels, filed as a row of the class marked inferred from identical lines.
15. **Endurance: what the research in hand finds she did.** Failing that, the General Board's requirement of 1935, 10,000 miles at 16.5 knots, filed as numeric rows that say "requirement" and read by an exception. It sets what she burns a mile and her cruising speed, which would then be above a warship's 15 knots; over five days it matters little, and the class says what it rests on.
16. **Cimarron is a class of her own** in the list, since her bunkers and her cargo differ from her sisters'.

### The ships and where they go

17. **Closed proofs are not touched.** Each scenario's oilers are in a new scenario file beside the old, with a derivation list of its own for the ships added, a class file of its own from it, a sides file of its own, and that class file named beside the scenario's others. A sides file must match its class files id for id, so nothing is added to a list or a sides file a closed proof reads.
18. **The Coral Sea:** Neosho and the destroyer Sims as one force, of the mission transport, with no home port, in the hex of Task Force 17 at the start. Sims is not in the game today; she is derived as her sisters in that scenario are, in the new list. She is in Neosho's own force and not an escort to it, since the staff sends no force to an oiler's force that has an escort. Neosho carries her cargo full, as every ship in that file starts with her own fuel full; her true load on 5 May is a gap and the file says so.
19. **No order is scripted for her.** The Coral Sea's files give no side a scripted order, and one for Neosho would bind the person in the Allied seat. She is the commander's to send, and the staff uses her by decision 0201, rule 20.
20. **The five days of the Eastern Solomons:** Platte and Cimarron as one force. The record has them leaving "Roses" at daylight on 23 August and meeting the carriers at 0800 on 25 August at 13-04 S, 164-03.5 E, which is on the map. No row places them at the scenario's start or says what Roses was. They go in when the research in hand gives a start; if it gives none, they wait.
21. **A force of oilers is not counted by a landing as lift.** The staff counts every transport force for a take whose forces are not named; a force whose every ship has the mark is left out of that count.
22. **The table offers the new files:** a new base and its makers for each scenario, directives for each, and `table/scenarios.json` pointed at them. Their lines on the new game screen say the Allied side has an oiler and the Japanese side none.

## What is left out, and why

- **Tippecanoe.** No endurance and no requirement for her class; and whether she was with the force on 6 May is disputed.
- **Every Japanese oiler.** No row gives her bunkers, her displacement or her range. A Japanese carrier force therefore still has no oiler to go to. The research is asked for these; they follow when it finds them.
- **Sabine, Kaskaskia and Guadalupe at the Eastern Solomons.** Where each was on the days played is disputed or not given.
- **The 39 days of Guadalcanal.** Its oilers shuttled to Noumea and back to reload. Without a staff that sends an oiler to reload (stage D of decision 0201) they would sit empty after the first week.
- **The staff placing an oiler.** Stage D. Until then several short forces may be sent to one oiler on the same morning, each by its own reckoning.

## Rule card

Reads: fact rows of a class and a ship, and a factor. Writes: a derived class file and its sources file. Of the game's rules one line changes (rule 21: what a take counts as lift); an oiler is otherwise a ship of a class with the mark, as stage A built.

## Games on file

None moves. No list, class file or sides file that a scenario on file reads is changed: the new ships are in lists, class files and sides files of their own, named only by the new scenario files. The one change to the staff, rule 21, changes nothing where no ship has the mark, and no ship on file has it but scenario 115's. The build plays every scenario before and after and says what moves.

## What it costs

- **The derivation learns a kind of ship that is not a warship:** a type, two attributes in barrels, a factor in a file of its own, a toughness from light displacement, and no guns by kind. Decision 0166's transports can follow by the same road.
- **More rows before any of it can be derived:** the factor, the class's guns of 1942, her endurance, Sims, and a start for the two oilers of August. The research in hand is after each.
- **A one-sided table for now:** an Allied commander has oilers and a Japanese one has none.
- **An endurance that is a requirement and not a performance,** said on the class.
- **Rulebook:** rule 21 is a line in chapter 8, where a take counts its lift. No other rule of the game changes. `data/oob/README.md` and `docs/engineering/data-rules.md` gain the new attributes and the oiler's derivation; the scenario files' own headers say what is in and what is left out.

## Questions for the designer

1. **A one-sided start.** The Allied side gets real oilers now and the Japanese side when the facts are found. The architect proposes to go ahead and say so on the new game screen's lines for those scenarios.

## How the plan changed on review

A reviewer who had not seen the plan read it against the derivation, the loader, the rows and the scenario files on Nov 21, 2026.

- **Adding the oilers to the lists on file would have broken the closed proofs.** A sides file must match its class files id for id, and the five days share theirs with the 39 days. Each new scenario now has a list, a class file and a sides file of its own (rule 17).
- **Several readings the plan asked for do not exist.** There is no "nearest year"; a later row is read only by an exception that says it was already so, and an exception reads one row for one ship. The figures are now numeric rows, each read by a named exception or a written rule (rules 6 to 15).
- **The plan under-rated her cargo, for a wrong reason.** The 1945 table of fuel-oil tanks is an allotment in design by its own footnote, and Platte carried more in 1942. The cargo oil of 1943 is read (rule 13).
- **The factor from barrels to tons has no second way.** Decision 0166 asks for the Navy's own factor in a row and nothing else; the plan's fallback is dropped and the factor gets a file (rule 8).
- **Her anti-aircraft guns are read, not left out,** as the decision the plan cited says; and "no guns" is by her kind, in the code, not by a scenario's assumption (rules 3 and 4).
- **A scenario's order has a day and a pulse, not a clock,** and the Coral Sea's files script no order for anyone. Neosho gets none (rule 19). The two oilers of August have no start on record, and wait for one (rule 20).
- **Sims is not in the game** and must be derived; **Cimarron differs from her sisters** and is a class of her own; **a take would have counted the oilers as lift** (rules 16, 18 and 21).

