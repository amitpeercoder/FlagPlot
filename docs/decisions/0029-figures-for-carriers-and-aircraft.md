# 0029: Figures for carriers, anti-aircraft guns and aircraft, derived from facts

Status: confirmed. The designer approved the five rule choices on Nov 11, 2026, with the plan for stage 5e (decisions 1, 5, 6, 8 and the scope in decision 0030). Everything else here is data, which the architect settles (CLAUDE.md, "Who settles data").

## The gap

Decision 0020 derives a ship class's surface-combat figures from facts. Scenario 5 needs four more things for real ships and aircraft: how many aircraft a ship carries, how much anti-aircraft fire it puts up, what an aircraft type can do, and how hard it is to shoot down. It also meets ships whose guns and torpedoes no source read gives in full, in a battle where no ship fired at another.

## Decisions

### Ships

1. **Anti-aircraft shots = heavy guns x 1 + light barrels x 0.25, rounded down.** Heavy and light are the two slots of the facts, `heavy_aa` and `light_aa`: every heavy gun in the facts is 100 to 127 mm and every light one 12.7 to 28 mm. Counts are barrels. Several weapons in one slot are added. Machine guns, the .50 calibre and the 13 mm, are not counted: their numbers are the most disputed rows in the facts, and at a quarter each they would add one or two shots to a ship. The two weights are model parameters and placeholders, in the rules file.
2. **Aircraft capacity is the class's sourced capacity.** A ship with no such fact carries none. A carrier with no such fact is an error.
3. **Radar is yes if any search radar is fitted on the date,** whichever set it is. Two sources naming different sets do not conflict for this figure.

   *Nov 14, 2026: decision 0077: a derived class has the radar figure only for a surface-search set whose display has a plan position indicator.*

4. **A scenario's ship list may say that named ships' guns and torpedoes are not modelled.** Their main guns, rate of fire, ammunition, gun damage, torpedo tubes and torpedo damage are written as 0, and the sources file says it is an assumption of the scenario and not a fact. It is for a battle in which those ships fought no surface action. Without it the tool would need 81 figures the Eastern Solomons scenario never uses, most of them not in the facts.
5. **An exception may name a row whose dates are given only to the year.** The tool cannot tell whether "1942" covers a day in August 1942. Naming the row in the rules file says the architect has judged that it does, and why.

### Aircraft

6. **Strike and search radius.** The sourced combat radius where exactly one can be read. Otherwise one third of the sourced range with a normal load, rounded down to a whole mile. The fraction is a model parameter and a placeholder.

   *Nov 14, 2026: decision 0075 gives an aircraft type a search radius of its own, apart from the strike radius.*

7. **What an aircraft attacks with** is the load named for the type in the scenario's aircraft list, which must be a load the facts hold for it. The list names it because the facts hold several loads for most types and the choice is the scenario's: a 1,000 lb bomb on a strike, a torpedo for a torpedo bomber.
8. **Damage** uses the constants ships already use: a bomb's weight in kg / the gun constant, a torpedo's warhead in kg x the torpedo constant.
9. **Role** follows the type's role in the facts: fighters are fighters; dive, torpedo, heavy and land attack bombers are bombers; flying boats and amphibians are patrol aircraft.
10. **Sweep width** in search is one model parameter for every type, a placeholder. No source gives it.
11. **How hard an aircraft is to bring down** comes from two sourced yes-or-no facts, pilot armour and self-sealing fuel tanks. Each one present multiplies the chance that a fighter's or a gun's shot brings the aircraft down by a factor in the air combat figures, a placeholder. Where a pool of targets holds several types, the pool's first group sets the factor.
12. **Air group strengths, bases and positions are scenario inputs,** as decision 0015 allows for starting positions and readiness. The scenario file gives them and cites the fact rows. The tool does not generate them.

## What the architect settled for the Eastern Solomons

Every choice is an exception in `data/derivation/rules.toml` with its reason, or an assumption in `data/derivation/eastern_solomons.toml`. The ones that move a figure most:

| Figure | Settled | On what |
| --- | --- | --- |
| Surface weapons of all 36 ships | Not modelled | No ship fired at another; most are not in the facts in full |
| Search radar of 24 ships | Treated as not fitted | No source read says either way |
| F4F-4 radius | 105 miles | The Navy sheet's figure without drop tanks. Nothing read says tanks were carried on the day |
| SBD-3 and TBF-1 radius | 200 and 190 miles | Distances flown on the day. No official radius was found |
| D3A1 and B5N2 radius | 252 and 175 miles | A third of range. No radius was found at all |
| Mark 13 torpedo warhead | 183 kg | The early warhead. Which model was carried in August 1942 rests on the architect's knowledge, not on a dated source |
| Type 91 torpedo warhead | 205 kg | As above |
| Enterprise's 20 mm guns | 38 | The settlement sheet's choice. Six of them may have been added after the battle; 32 is the other reading |
| Shokaku's and Ryujo's 25 mm guns | 48 and 22 | The settlement sheet's choices, on weak sources |
| Gleaves class 5-inch guns | 4 | Sources say 4 or 5; the lower is read |

Worth more research before step 2 compares results with history: the two torpedo warheads on the date, and Enterprise's light guns on 24 August.

## Why

- **Not modelled, by name,** is the same kind of statement as "ammunition not limiting" in decision 0023: the absence of a figure, said out loud, and not an invented one. If two such forces ever met in a scenario they would not fire, and the ship list says so.
- **One third of range** allows for the flight out, the flight back, and a reserve for forming up, searching and landing. It gives the B5N2 175 miles and the D3A1 252, beside about 200 flown on the day.
- **Armour and tanks** are the only sourced facts that separate a Wildcat from a Zero as a target. Which fighter shoots better has no sourced measure, and the rule leaves them equal.
- **The first group's factor** keeps one roll per pool. A raid of mixed types is rare, and the complexity review should look at it.

## Deferred

- Which fighter is better in a fight. Speed, climb and manoeuvre are in the facts and no rule reads them.
- Drop tanks. The F4F-4's radius is its radius without them.
- Aircraft that cannot fly from a deck. The facts hold "carrier capable" for some types only.
- Generating air groups from the facts.
- Floatplanes aboard cruisers and battleships.

## Rule card

| | |
| --- | --- |
| Reads (the tool) | `data/oob`: class facts, fits, weapon facts, aircraft type facts; the rules file; the scenario's ship and aircraft lists |
| Writes (the tool) | The scenario's ship class file and aircraft type file in `data/reference`, with the rule and rows behind every figure |
| Reads (the simulation) | An aircraft type's two protection facts, when a shot is fired at aircraft of that type |
| Writes (the simulation) | Nothing new: the chance of the existing rolls of decision 0027 |
