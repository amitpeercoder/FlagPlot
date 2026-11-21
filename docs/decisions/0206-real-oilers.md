# 0206: Real oilers

Status: a plan by the architect, written on Nov 21, 2026 on the designer's word "real oilers", for stage B of refuelling at sea (decision 0201, rule 28). To be read by a reviewer who has not seen it written, and amended on that reading, before its hand calculation is written and before any of it is built.

Summary: Refuelling at sea is built on placeholder ships, and no scenario a person can play has an oiler. This makes the American fleet oiler of 1942 a real class, derived from fact rows as a warship is, and puts the oilers the record names into the Coral Sea and the five days of the Eastern Solomons, in scenario files beside the ones the closed proofs play. The Japanese oilers, Tippecanoe and the 39 days are left for later, each for a stated want of facts.

Standing: Planned on Nov 21, 2026; waits for its review, and for two facts the research is still after.

## Why

What is so today:

- **The derivation makes warships only.** It maps six types to four kinds and fails on any other; cargo is fixed at 0 (`crates/oob-derive/src/derive.rs`). Every transport in the game is a hand-written placeholder. Decision 0166 ruled how a transport is to be derived (kind, toughness from light displacement, cargo from a stated capacity, no guns) and that part was never built.
- **Stage A's oiler is a placeholder class** with the mark `fuels_at_sea`. The loader already accepts a class file with no column for the mark.
- **The oilers are named in the record and left out of every scenario.** The Coral Sea file says why: "an oiler has no kind in the simulation".
- **No carrier force in the Coral Sea or the Eastern Solomons has a home port.** By decision 0201, rule 20, the staff gives such a force the order to fuel from an oiler's force when a ship of it falls below a quarter. So once an oiler is on the map the staff uses her unasked.
- **The research of Nov 21, 2026** (`docs/research/real-oilers-1942.md`, pull request 629) gives for the Cimarron class: her own bunkers, 14,850 to 15,000 barrels; light displacement, 7,256 or 7,470 tons; trial speed 18.3 knots; and cargo by six statements that are different quantities. It found no endurance at a speed for any oiler, no factor from barrels to tons stated by the Navy, and for the Japanese oilers no bunkers, displacement or range at all.

## The rules

### The derivation

1. **A class of the type `oiler` is of the kind auxiliary and carries the mark.** The derived class file gains the column `fuels_at_sea`, 1 for such a class and 0 for every other, and only in a list that names an oiler, so that no class file on file changes.
2. **Toughness is read from the light displacement,** as decision 0166, rule 2, has it for a transport, divided by the same figure as a warship's standard displacement.
3. **Guns are left out,** as decision 0166, rule 5, has it: no main battery and no torpedo tubes. An oiler's anti-aircraft guns are left out with them, and that is said in the rules file: in a raid she is a target and no part of the defence.
4. **Speed** is her class's maximum speed row, by the list's ordinary reading of rows.
5. **Her own fuel** is read from a row in barrels, a new attribute for the fuel a ship steams on stated in barrels, converted by the factor of rule 7.
6. **Her cargo of oil** is a new attribute, the fuel oil a ship can carry for others, in barrels, with its year. It is fuel oil alone: diesel and gasoline are other cargoes and are not read. Where rows of more than one year stand, the row nearest the scenario's year is read, and a load carried on one voyage is not read as a capacity (decision 0166, rule 3).
7. **Barrels are made tons by a factor read from a row,** as decision 0166, rule 8, requires: the Navy's own factor for its fuel oil, or, failing a stated factor, one load a source gives in both. The derivation fails, and no oiler is derived, until such a row stands.
8. **Endurance is read as any ship's is,** a distance with its speed. Where a class has no such row, the requirement its design was built to is read in its place, by a written exception for the class that names the row. Where it has neither, the class is not derived.

### How the Cimarron class comes out

These are the rows the rules above would read today. Each is settled by the architect with its reason on the row, and the hand calculation works them.

9. **Light displacement: 7,256 tons.** Two sources give it, shipscribe and the official history for Kaskaskia; the 7,470 of a later compilation is left disputed.
10. **Speed: 18.3 knots,** the trial speed in the Navy's own data book, for every ship of the class.
11. **Her own fuel: 15,000 barrels,** the data book's figure for four of the five ships; Cimarron's own 14,850 is read for her alone.
12. **Cargo of oil: the fuel-oil tanks of the Navy's 1945 table,** 68,054 barrels for Platte, Sabine, Kaskaskia and Guadalupe and 87,246 for Cimarron. It is the one statement that is fuel oil alone and a capacity. The loads of July 1942, 12,000 tons and 93,000 barrels, are larger than the first and are loads, with other oil among them; the plan says so and does not read them. Neosho has no row of her own and reads the four ships' figure, by a written exception, as a ship of the class.
13. **Endurance: the General Board's requirement of 1935, 10,000 miles at 16.5 knots,** by rule 8's exception, unless the research in hand finds what she did.

### The ships and where they go

14. **Closed proofs are not touched.** Each scenario gains its oilers in a new file beside the old, as scenario 103 was made beside scenario 5 (decision 0184). The table offers the new files.
15. **The Coral Sea:** Neosho, with the destroyer Sims, as a force of their own beside Task Force 17 at the start. She carries her cargo full, as every ship in that file starts with her own fuel full; her true load on 5 May is a gap and the file says so. Her force is given, as a scenario's order, the passage the record gives her: detached at 1725 on 6 May for the oilers' rendezvous. A commander may order her otherwise.
16. **The five days of the Eastern Solomons:** Platte and Cimarron as one force, starting where the record has them and given as a scenario's order their passage to the meeting of 0800 on 25 August at 13-04 S, 164-03.5 E. Their escorts are not named in any source read, so they sail without, and the file says so.
17. **A fuelling group is of the mission transport,** as stage A's is, and has no home port.
18. **Each new class takes a new id** in its list's `[class_ids]`, and a row in the class sides file of each scenario that names it.

## What is left out, and why

- **Tippecanoe.** No endurance and no requirement for her class; and whether she was with the force on 6 May is disputed.
- **Every Japanese oiler.** No row gives her bunkers, her displacement or her range. A Japanese carrier force therefore still has no oiler to go to. The research is asked for these; they follow when it finds them.
- **Sabine, Kaskaskia and Guadalupe at the Eastern Solomons.** Where each was on the days played is disputed or not given.
- **The 39 days of Guadalcanal.** Its oilers shuttled to Noumea and back to reload. Without a staff that sends an oiler to reload (stage D of decision 0201) they would sit empty after the first week.
- **The staff placing an oiler.** Stage D.

## Rule card

Reads: fact rows of a class and a ship. Writes: a derived class file and its sources file. The game's rules do not change: an oiler is a ship of a class with the mark, as stage A built.

## Games on file

None moves. No class file on file gains a column; the new classes are in lists whose scenarios are new files. Adding a class that no scenario on file names has moved only the class files' hashes in saves before (decision 0166), and the build says what moves.

## What it costs

- **The derivation learns a kind of ship that is not a warship:** a type, two attributes in barrels, a factor, and a toughness from light displacement. Decision 0166's transports can follow by the same road.
- **A one-sided table for now:** an Allied commander has oilers and a Japanese one has none.
- **An endurance that is a requirement and not a performance,** said on the class.
- **Rulebook:** none of the game's rules change. `data/oob/README.md` and `docs/engineering/data-rules.md` gain the new attributes and the oiler's derivation; the scenario files' own headers say what is in and what is left out.

## Questions for the designer

1. **A one-sided start.** The Allied side gets real oilers now and the Japanese side when the facts are found. The architect proposes to go ahead and say so on the new game screen's lines for those scenarios.
