# Step 2: what to decide and do first

A list kept by the architect, begun on Nov 12, 2026 as step 1 closed. Step 2 is the validation harness: runs compared with the record, and the model tuned against it (`docs/pillars.md`). Nothing here is decided. Each item becomes a plan for the designer's approval, then a decision record with a hand calculation, as in step 1. Items are added as they are found and struck off as they are done.

## Decisions to bring to the designer before tuning starts

| Item | What it is | Why now | Where it came from |
| --- | --- | --- | --- |
| Ships charged straight-line miles | A ship still moves hex by hex, but a run of hexes with no land in the way is charged its straight length in distance, time and fuel. A real detour round land keeps 40 miles a hex | The scenarios' main routes are overcharged 15 to 24 in 100, so ships are slower and shorter-legged than the sources say. Comparing ship movements with the record means nothing until this is right | Decision 0040 deferred it; the analysis of Nov 12, 2026 |
| Reach beyond the radius | A strike group may fly somewhat beyond its type's radius at a price in aircraft written off on the way home, reusing the rule by which wear rises with distance (decision 0035). Today 190 miles flies at full strength and 200 does not fly at all | In a recorded game two torpedo groups stayed on deck 10 miles short of a carrier. A combat radius is a planning figure with reserves, and commanders launched beyond it. The record is now read (`docs/research/oob/strike-ranges.md`, once its pull request is merged): strikes were usually launched at 150 to 210 miles with all types together; Saratoga's torpedo bombers flew against Ryujo, sighted at 198 miles, and none was lost; the cost of reaching far rose past about 175 miles, was paid on the way home, and fell first on fighters | The second round of model games; agreed by the designer on Nov 12, 2026 |
| Ditching: aircraft lost because they cannot reach a deck or a field | A cause of loss apart from shot down, written off, destroyed on the ground and lost with the ship: out of fuel, the carrier not found, no landing in the dark, no deck left. To be built inside the reach rule above, as the price of flying long, and not as a second mechanism. The crew of an aircraft that comes down near its own force or base is usually saved, in open sea or in the dark much less often, over the enemy almost never, which extends the rule for pilots that exists (decision 0038). Aircraft in the air when their carrier sinks are a separate, smaller question: today they are lost with the ship | At the Eastern Solomons it is the largest gap between the rules and the record: 7 of the 17 American carrier aircraft lost ditched out of fuel with their crews saved, 5 Japanese dive bombers of the second wave did not return after dark, and Ryujo's aircraft in the air had no deck. Without it the model reaches the recorded totals only by shooting down too many, which pushes the combat figures the wrong way; the two all-cause loss measures for that battle are not judged until it lands. The chances need rows: the facts hold examples and no rates | Raised by the designer with the spec session and added at his word on Nov 12, 2026; `data/oob/engagement_outcomes.csv` rows 289, 293, 305, 965, 1166, 1169 |
| A range policy order | The player sets each air group to normal or extended reach. It is the "range limit" order the spec lists and decision 0050 deferred | It turns the item above into a choice the commander makes | The same |
| Strike target priority as an order | Which kind of contact a strike group goes for first, and whether it holds its strike. Today the priority is the order the kinds of ship were declared in | Two players said they could not choose targets or hold a strike back; the architecture review asked for the same before step 3 | Decision 0055, "Left for later"; the recorded games |
| A friendly field beyond the target | A strike that cannot get back to its carrier may land at a base of its own side within reach of the target, and fly from there or return later. Noted, not to be built with the first version of the reach rule | It eased long launches in the record: landing ashore was used twice at Eastern Solomons. It needs air groups that can be away from their carrier, which the game does not have (rebasing is deferred by decision 0050) | The strike-range research; added at the designer's request on Nov 12, 2026 |
| Daylight as a limit on a launch | A strike launched late must be recovered in the dark or not launched. Noted, not to be built with the first version | Dusk limited a launch as much as fuel did, and the longest reaches at Midway ended in a night recovery. Today a strike flies in the morning or afternoon pulse with no regard to the hour within it | The same |
| Ship repair | Damage is patched forward in weeks or mended in a yard in months | Without it a long campaign only wears fleets down. The research gives three bands of time away | Decision 0052, item 11; the step 2 research |
| "Out of the campaign" on the objectives scale | Points for a ship that must leave for repair, not only for one with propulsion or flight deck out at the end | Enterprise had her deck back within the hour and was away 53 days; the scale gives nothing for that | The step 2 research |
| The hash version | Raise it, drop the old tags, remake every baseline | Housekeeping the architecture review asked for at the next change of layout. It goes in alone | Decision 0055, 9 |

## What the architect settles from the research

- Which figure to prefer where sources disagree: the Japanese aircraft lost at Eastern Solomons (about 40, 62 or 75), and each day's losses over Guadalcanal.
- The three claim chances of decision 0056 by side, and the surface claim figure of decision 0014, from claims set beside actual losses.
- Base sizes for the real map (decision 0052, item 15).
- The points and levels of decision 0054, and history's result on that scale for scenarios 4, 5 and 6.

## Known gaps between the model and the record

- Savo Island scores as a minor Allied victory; the night was a decisive Japanese one (decisions 0024, 0054).
- The losses of the four weeks over Guadalcanal do not have the shape of the record (decision 0042).
- Eastern Solomons over 1,000 runs, on placeholder points: 54 in 100 stalemates, 36 Allied victories, 9 Allied defeats. The day is usually judged a limited American victory.
- Claims in the model overstate less than the record: about 2.5 to 3 aircraft were claimed for each lost, and 14 ships for 4 at Savo Island.
- The Japanese search at Eastern Solomons is a placeholder arc; the real search orders are now in the facts.

## Not step 2, kept here so they are not lost

- Before step 3: the staff closing the range on a contact it cannot reach; choices rules make inside a day pulled into functions the planner can ask (decision 0055, "Left for later"); objectives that name a flow (`docs/design/economy.md`).
- With step 2 or 3: coastwatchers and reconnaissance as standing sources (`docs/design/intelligence.md`, layer 2).
