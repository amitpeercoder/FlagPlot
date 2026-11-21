# 0206: Real oilers

Status: a plan by the architect, written on Nov 21, 2026 on the designer's word "real oilers", for stage B of refuelling at sea (decision 0201, rule 28). It was read by a reviewer who had not seen it written, and this is the plan with that reading folded in. Its hand calculation waits for the research still in hand.

Summary: Refuelling at sea is built on placeholder ships, and no scenario a person can play has an oiler. This makes the American fleet oiler of 1942 a real class, derived from fact rows as a warship is, and puts the oilers the record names into the Coral Sea and the five days of the Eastern Solomons, in scenario files beside the ones the closed proofs play. The Japanese oilers, Tippecanoe and the 39 days are left for later, each for a stated want of facts.

Standing: Planned on Nov 21, 2026; its review and the research's second pass folded in; its hand calculation is scenario 120; waits for its build.

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
8. **Barrels are made tons by the Navy's own factor for its fuel oil, read from a row,** as decision 0166, rule 8, requires, with no other way. The row is on the class: the attribute `fuel_oil_barrel_factor_lt`, 0.1516 long tons a barrel, from the Navy's fuelling manual of 1945, which gives it in so many words (ship_class_facts 14311). It is a later row read as already so: it is the weight of a barrel of the Navy's fuel oil. A file of its own for such factors was wanted and would need the row checker's code changed; the row on the class serves.
9. **Endurance is read as any ship's is,** a distance with its speed, as numeric rows.
10. **A row dated after the scenario is read only by a written exception** that says it was already so, as the data rules have it. There is no reading of "the nearest year".

### How the Cimarron class comes out

Each of these is settled by the architect with its reason on the row or in the list. The hand calculation works them once the research in hand has reported, and any of them may change on what it finds.

11. **Light displacement: 7,256 tons.** The class's two rows are disputed, 7,256 and 7,470, and the official history prints each for a different ship. The 7,256 is read, by an exception; the two differ by under 3 in 100 and toughness is their twenty-fifth part.
12. **Her own fuel: 15,000 barrels; Cimarron's own 14,850 for her.** They are the Navy data book's lines of 1945, read by an exception as already so: they are her hull's bunkers, and no source read says they were altered.
13. **Cargo of oil: the cargo oil of November 1943,** 101,638 barrels for Platte and 103,233 for Cimarron, read by an exception as already so. It is the statement nearest in time that is a capacity, and the loads of July 1942 sit under it: 93,000 barrels taken by Platte, and 12,000 tons of fuel oil each. The Navy's 1945 table of fuel-oil tanks, 68,054 barrels, is not read: by its own footnote it is how the tanks were allotted to products in design, and Platte carried more than it in 1942.
14. **Neosho has no cargo row of her own;** she was lost in May 1942. She reads the least of her sisters' rows, 101,638 barrels, by a rule written in the list: a ship with no row of her own for a figure her sisters differ in reads the least of theirs. Her bunkers read the four ships' 15,000 barrels, filed as a row of the class marked inferred from identical lines.
15. **Endurance: 12,000 nautical miles at 16.5 knots,** the endurance built into the twelve tankers of the class as a feature of national defence (Wildenberg; row 14313), filed by the architect as numeric rows that cite it. The source says "mile"; it is read as nautical, as a naval design figure, and a compilation's 12,100 nautical miles with no speed stands beside it. No source gives what she burned a day. It sets what she burns a mile, 2,274 tons over 12,000 miles, and her cruising speed, 16.5 knots, above a warship's 15.
16. **Cimarron is a class of her own** in the list, since her bunkers and her cargo differ from her sisters'.

### The ships and where they go

17. **Closed proofs are not touched.** Each scenario's oilers are in a new scenario file beside the old, with a derivation list of its own for the ships added, a class file of its own from it, a sides file of its own, and that class file named beside the scenario's others. A sides file must match its class files id for id, so nothing is added to a list or a sides file a closed proof reads.
18. **The Coral Sea:** Neosho and the destroyer Sims as one force, of the mission transport, with no home port, in the hex of Task Force 17 at the start. Sims is not in the game today; she is derived as her sisters in that scenario are, in the new list. She is in Neosho's own force and not an escort to it, since the staff sends no force to an oiler's force that has an escort. Neosho carries her cargo full, as every ship in that file starts with her own fuel full; her true load on 5 May is a gap and the file says so.
19. **No order is scripted for her.** The Coral Sea's files give no side a scripted order, and one for Neosho would bind the person in the Allied seat. She is the commander's to send, and the staff uses her by decision 0201, rule 20.
20. **The five days of the Eastern Solomons:** Platte and Cimarron as one force, of the mission transport, with no home port and no escort, stopped in the hex of Efate, (7,10), at the start. The record has them at "Roses", which the fleet's own war diary says is Efate, until they sailed at daylight on 23 August, to meet the carriers at 0800 on 25 August at 13-04 S, 164-03.5 E, hex (5,2). No order is scripted for them, as for Neosho. Their escorts are named in no source read, and the file says so.
21. **A force of oilers is not counted by a landing as lift.** The staff counts every transport force for a take whose forces are not named; a force whose every ship has the mark is left out of that count.
22. **The table offers the new files:** a new base and its makers for each scenario, directives for each, and `table/scenarios.json` pointed at them. Their lines on the new game screen say the Allied side has an oiler and the Japanese side none.

23. **The guns.** Neosho in May 1942: one 5-inch/51, three 3-inch/50 and eight 20 mm. Cimarron in August: four 5-inch/38 and eight 20 mm. Platte's fit of August 1942 is disputed between two lines of one source; the lesser is read, by an exception that says so. No source names a radar for any of the three, and the list's assumption that unknown means none stands for them.

## What is left out, and why

- **Tippecanoe.** No endurance and no requirement for her class; and whether she was with the force on 6 May is disputed.
- **Every Japanese oiler.** The research's second pass found their bunkers, their radius at a speed and their deadweight in a wartime American manual, of low confidence. It found no displacement, light or standard, for any of them, and toughness is read from nothing else. A Japanese carrier force therefore still has no oiler to go to. They follow when a displacement is found, or when a rule is made for a hull known only by her tonnage.
- **Sabine, Kaskaskia and Guadalupe at the Eastern Solomons.** Where each was on the days played is disputed or not given.
- **The 39 days of Guadalcanal.** Its oilers shuttled to Noumea and back to reload. Without a staff that sends an oiler to reload (stage D of decision 0201) they would sit empty after the first week.
- **The staff placing an oiler.** Stage D. Until then several short forces may be sent to one oiler on the same morning, each by its own reckoning.

## Rule card

Reads: fact rows of a class and a ship, and a factor. Writes: a derived class file and its sources file. Of the game's rules one line changes (rule 21: what a take counts as lift); an oiler is otherwise a ship of a class with the mark, as stage A built.

## Games on file

None moves. No list, class file or sides file that a scenario on file reads is changed: the new ships are in lists, class files and sides files of their own, named only by the new scenario files. The one change to the staff, rule 21, changes nothing where no ship has the mark, and no ship on file has it but scenario 115's. The build plays every scenario before and after and says what moves.

## What it costs

- **The derivation learns a kind of ship that is not a warship:** a type, two attributes in barrels, a factor in a file of its own, a toughness from light displacement, and no guns by kind. Decision 0166's transports can follow by the same road.
- **Rows the architect files before the build:** her bunkers, her cargo and her endurance as numeric rows citing the statements on file, and the exceptions that read them.
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
- **The research's second pass, Nov 21, 2026** (pull request 632). The Navy's factor is on file in so many words (rule 8). The class's endurance is a stated design figure, 12,000 miles at 16.5 knots (rule 15). Roses is Efate, so the August oilers have a start (rule 20). The guns of 1942 are on file but for Platte's, which is disputed (rule 23). Sims lacks nothing her sisters have. The Japanese oilers have bunkers and radius now and still no displacement.


## As built

Built on Nov 21, 2026, on branch `real-oilers-build`.

**The derivation** (`crates/oob-derive`). The type `oiler` maps to the kind auxiliary (`derive.rs`, `KIND_FOR_TYPE`). A ship of a kind that is not a warship gets her seven surface weapon figures as 0 by her kind, reads no fit, shell or outfit for them, and says so in the sources file and the class file's header; a list that also names her in `surface_weapons_not_modelled` changes nothing. Her toughness is `displacement_light_lt` over the same 25. An oiler's own fuel is `fuel_capacity_bbl` times `fuel_oil_barrel_factor_lt`, and her cargo `cargo_oil_capacity_bbl` times the same factor, each read as any fact is (ship, else class; exceptions; rows of a later date by `later = true`), the figure citing the barrels and the factor. A list may write `[least_of_sisters]` (`facts`, `reason`): a ship with no row of her own or of her class for one of those facts reads the least of the values her sisters' own rows give, each read as any ship's row is on the list's date (a sister whose row cannot be read is passed over); the note names every row compared. The column `fuels_at_sea` (1 for an oiler, citing her ships and classes rows) is written only where the list has an oiler, so no class file on file changed. Unit tests: `crates/oob-derive/tests/derive.rs`, the tests from `an_oiler_is_an_auxiliary_...` on, and the lists' files against those committed.

**The anti-aircraft rule, a fix.** It refused any row of the heavy_aa or light_aa slot whose weapon is not of kind `aa_gun` or `machine_gun`, whatever its date. It now asks the kind only of a row that holds or may hold on the list's date, so Neosho's 3-inch/23 of 1941 does not stop her; a row that holds is refused as before (unit test).

**The rows and the data** (each settled by the architect, Nov 21, 2026, with the reason on the row, in `data/derivation/rules.toml` under "Decision 0206", or in the list):
- Attributes `fuel_capacity_bbl` and `cargo_oil_capacity_bbl`, for a class and a ship.
- `ship_facts.csv` 14700 to 14704, the bunkers of Cimarron (14,850), Platte, Sabine, Kaskaskia and Guadalupe (15,000), restating rows 14303, 14314, 14325, 14336 and 14347; 14705 to 14708, the cargo oil of November 1943 of Cimarron (103,233), Platte (101,638), Sabine and Guadalupe (104,938), restating 14366 to 14369. `ship_class_facts.csv` 14340, the class's 15,000 barrels, inferred from the four lines; 14341 and 14342, endurance 12,000 nm at 16.5 knots, restating 14313, the mile read as nautical. They were first numbered 14600 to 14608 and 14320 to 14322; the merge of `origin/main` brought rows of those ids from the research of the Japanese oilers (pull request 635), and these, not yet on main, were renumbered.
- `fits.csv` 14320 and 14321, Cimarron's four 5-inch/38 of 1942 and 1941 in the heavy_aa slot, and 14322, those of Platte's second line of 1942, each restating her main battery row with the same source and locator, as the warships' 5-inch/38 are filed in both slots (the architect's ruling 2 on the build).
- `weapons.csv` 1064, the 3-inch/50, made `aa_gun`: a dual-purpose gun, as the 5-inch/38 and 5-inch/25 are filed, and Neosho's captain calls hers anti-aircraft guns (fits 14307). The 3-inch/23 stays `gun`. The reason is in the file's header, which has no note column. Before and after, every list's `--check` passed and every derived file was byte for byte the same (the transports' secondary_battery rows that name it are read by nothing).
- Exceptions: the factor 14311, the bunkers 14700, 14701 and 14340, the cargo 14705 to 14708 (each `later = true`); light displacement 14300; maximum speed 2516 (DANFS, 18 knots); Neosho's 1942 fits 14305 and 14306, read as holding on 6 May; Cimarron's 20 mm 14315 and 5-inch/38 14320; Platte's line A, 14310 and 14311, the lesser by the figure and the guns.

**Readings made in the build.**
- The cargo of 1943 is read for all four sisters with a row, Sabine and Guadalupe too, so that the least of her sisters for Neosho is the least of every row on file; it is Platte's 101,638 either way.
- Neosho's bunkers: the class row 14340 is dated 15 April 1945 as the lines it rests on, so it is read by `later = true` as the ships' own rows are.
- Cimarron's 5-inch/38 heavy_aa row of 1942 is dated to the year only, so an exception judges it to hold in August (rule 23). The 1941 row (14321) does not hold on either list's date and is not read.
- The list of the five days is dated 23 August 1942, day 1 of those files; the Coral Sea's 6 May, as the Coral Sea's own list.
- Sims and Neosho are in the Coral Sea list's `[experience]` table, on the engagement `coral-sea`, as Sims's sisters are; both come out 2.
- The oilers' list of the five days has no experience: no ship of the five days has one.
- The table's line for the five days says the oilers are at Efate, where the file puts them, not with the carriers.

**The figures.** Neosho (4030) and Platte (2030): auxiliary, 16.5 and 18 knots, 2,274 t of her own fuel, 12,000 nm, 15,408.3208 t of cargo, toughness 290.24, no guns, no radar (assumed none), the mark. Cimarron (2031): 2,251.26 t and 15,650.1228 t. Anti-aircraft: Neosho 5 (three 3-inch/50, fits 14305; eight 20 mm, 14306), Platte 5 (fits 14310, 14311), Cimarron 6 (four 5-inch/38, fits 14320; eight 20 mm, 14315). Sims (4031) has every figure of class 4008.

**Rule 21, changed on the architect's ruling.** As first built to the plan's words, a take left out only a force whose every ship has the mark; Neosho's force holds Sims, so it was counted and statement 10 did not hold. On the architect's ruling of Nov 21, 2026 the rule is: a force that has a ship afloat with the mark is not counted by a take whose forces limit names no task force; a limit that names it counts it (`crates/sim-staff/src/available.rs`, `has_oiler`; chapter 8, "A force with an oiler is no lift"; glossary, "Force with an oiler"). No ship on file but scenario 115's has the mark, and no game on file moved.

**The scenario files.** `scripts/coral-sea/make_oiler_files.py` makes `data/scenarios/coral-sea/base_coral_sea_oilers.toml` from the Coral Sea's base and the four files `02a` to `02d_coral_sea_oilers.toml` from it, with the old script's own orders; `scripts/five-days/make_oiler_files.py` the same for the five days (`base_eastern_solomons_oilers.toml`, `02a` to `02d_eastern_solomons_oilers.toml`). The fuelling group is task force 6 in each: Neosho (ship 54) and Sims (55) at (-3,5); Platte (44) and Cimarron (45) stopped at (7,10). The sides files are `class_sides_coral_sea_oilers.csv` and `class_sides_five_days_oilers.csv`, the files beside them with two rows more. `table/scenarios.json` points the Coral Sea's and the five days' pairings at the new files, with a line each that the Allied side has an oiler and the Japanese side none; the directives are as they were.

**Statements.** All thirteen hold and are tested in `tests/scenario_120_real_oilers.rs`, with the properties (c) and (b); (a) was checked by the build, not by a test: every one of the 174 scenario files on file was run with `run --events` on the live and on the frozen figures before the change and after, and the outputs, hash lines included, are byte for byte the same; and every class, ships, sides and experience file on file is byte for byte as at ae3972c3. Statement 6's test cannot keep that commit (the checks fetch two commits), so it holds what must stay so: no scenario on file names a file of the oilers, and no class, ships or sides file on file has their classes or the column of the mark but scenario 115's. In statement 8 the night's breakdown has the fuelling's hours as 11, the night's twelve less the hour alongside, while the tons take 1.2 hours: the 300 tons pass, as the statement says.

**Other tests that the new files touched.** Scenario 104's statement 23 ("The key") found `for_play` in the eight new files and the two new bases. On the architect's ruling of Nov 21, 2026 its test leaves out the two new bases as it does the old and counts the `02` files, and the statement carries the architect's amendment; no number moved. The frozen figures (`data/reference/step1`) gained copies of the two new class files and the two new sides files, as `crates/sim-data/tests/frozen_figures.rs` asks of every live figures file (its README says so); the frozen sides files are the frozen Coral Sea's and full slice's with the oilers' two rows each.

**Not done.** The steps of `CLAUDE.md` "Before a commit" name each list's `--check`; the two new lists' are not added there (an agent does not edit `CLAUDE.md`): they are run by `crates/oob-derive/tests/derive.rs` and by scenario 120's tests.
