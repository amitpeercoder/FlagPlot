# 0207: A figure estimated from like ships

Status: a plan by the architect, written on Nov 21, 2026 on the designer's word: staying with history is good and is to be done where it can be, but this is a game, and to block it for want of some pieces may be wrong; extrapolating is valid. To be read by a reviewer who has not seen it written, and amended on that reading, before any of it is built.

Summary: The rules say no real ship gets an invented rating, and a ship lacking one fact is left out of the game. Three passes of research on the Japanese oilers ended one fact short each time, so a Japanese carrier force has no oiler at all, which is further from history than an oiler whose range is a tenth out. This lets a missing figure of a real ship be estimated from like ships by a written rule, says so on the figure, keeps the gap open, and never lets an estimate stand as evidence. It is for what a ship is, never for what happened.

Standing: Planned on Nov 21, 2026; waits for its review.

## Why

What is so today:

- **A real ship's figures are derived from fact rows by a written rule,** and a ship the rows cannot rate is not derived (`docs/engineering/data-rules.md`). A made-up figure is a placeholder, and no real ship gets one.
- **The rule has kept real ships out for one fact each.** The Japanese fast tankers have bunkers, range and cargo on file and no displacement; the navy-built oilers have a displacement and no range; Tippecanoe has no endurance; decision 0166's transports wait on fuel and endurance.
- **The lists already estimate, case by case, without the name.** A gun's stated outfit is read for a class that mounts it "where nothing nearer exists" (decision 0166, 9). Neosho reads the least of her sisters' cargo (decision 0206, 14). A class row is filed as "inferred from five identical lines".
- **Research has stopped paying.** The record of a merchant hull requisitioned in 1941 does not hold a displacement, and no further search is likely to find one.

## The rules

### What an estimate is

1. **A figure of a real ship has one of three standings.** Read: from her own rows or her class's. Estimated: worked by a written rule from the rows of like ships. Placeholder: made up, and marked; a real ship still gets none.
2. **An estimate is allowed only where all of these hold:**
   - the record places the ship in a battle the game plays, or her class is needed by one;
   - a figure the derivation needs has no row, and a gap row says what was looked for and where;
   - a like ship has the rows the method needs.
3. **An estimate is for what a ship is:** her displacement, her fuel, her range, her speed, her cargo, her guns' figures. It is never for what happened: where she was on a day, what she carried on a voyage, what was lost, how many men a unit had. Those stay sourced facts or open gaps, and a scenario's own assumptions stay marked as they are today.

### How one is made

4. **Each estimate is an entry in the derivation list,** as an exception is: the ship or class, the figure, the method, the like ship, the rows read on both sides, the reason the two are alike, and the architect's name and date. Nothing is estimated without its entry.
5. **The methods are a short closed list.** A new method is a change to this decision.
   - **By ratio.** The figure is a row of the ship's own times the ratio of the same two quantities in the like ship: a light displacement from her deadweight, by the like ship's light displacement over her deadweight.
   - **As a like ship.** The figure is the like ship's own, where the two are of one design or one type and size: a sister's, or a near-sister's.
   - **In proportion to size.** A rate is the like ship's rate times the ratio of displacements: the fuel burned a mile at a speed.
6. **A like ship is of the same kind and trade, and near in size:** her deadweight or displacement within a quarter of the other's. The entry says in what the two are alike and in what they are known to differ.
7. **An estimate reads rows as the derivation does.** It does not read a disputed row without the exception that settles it, and it does not read another estimate: one ship's estimate never rests on another's.

### How it is marked and kept honest

8. **The sources file of a derived class says "estimated"** for the figure, with the method, the like ship and the rows, in place of the rows it would name for a figure read. A scenario file that has such a ship says so in its header.
9. **The gap stays open.** When a row is filed that gives the figure, the list's check fails until the estimate's entry is taken out, so an estimate cannot outlive the fact that replaces it.
10. **An estimate is never evidence.** It settles no dispute between sources. No measure of the validation rests on one, and no figure is tuned toward one. Where a measure would read a ship that has an estimated figure, the measure's own notes say so.
11. **The checker counts them.** The derivation's check prints, for each list, how many figures are estimated and for which ships, so that the number is seen each time it is run.

### What the rules pages say

12. **`docs/engineering/data-rules.md` and the project's instructions change in one line each:** "No real ship, aircraft or weapon gets an invented rating: its figures are derived from fact rows by a written rule, read from its own rows or, where the record has none, estimated from like ships by a rule that says so (decision 0207)."

## First uses

These are worked in the hand calculation once the plan is reviewed, and each may change there.

13. **The Japanese fast tankers** the record puts with the carriers: Toho Maru at the Coral Sea, and Toho, Toei, Kyokuto and Nippon Maru in August 1942. Each has, in an American manual of the war, her deadweight, her own bunkers, her radius at a speed and her speeds, of low confidence, and her cargo in barrels from a class page. She lacks a light displacement. It is estimated by ratio from the Cimarron class, a fast tanker of her time with both figures on file: 7,256 tons light over 18,250 tons deadweight.
14. **Tippecanoe,** who lacks an endurance, and **decision 0166's transports,** follow by the same rules where a like ship can be named, each in its own list.

## Rule card

Reads: fact rows, and entries of a derivation list. Writes: a derived class file and its sources file. The game's rules do not change.

## Games on file

None moves. No list on file gains an estimate by this decision; the first uses are in new lists for new scenario files, as decision 0206's ships are.

## What it does not do

- It does not estimate anything for a placeholder scenario or a placeholder ship; those are made up and say so.
- It does not let research stop. A gap with an estimate over it is still a gap.
- It does not touch aircraft or weapons yet. The same rules would serve them; they are left until a case asks.
- It does not estimate events, positions, loads, losses or strengths.

## What it costs

- **Figures in the game that no source states,** marked as such. A reader of a class file must look at its sources file to know which.
- **A judgement in every entry:** that two ships are alike. It is written down and signed, and it can be wrong.
- **One more thing the derivation does,** with its own check.

## Questions for the designer

None. The designer asked for this; the guard rails are the architect's, and he agreed them on Nov 21, 2026.
