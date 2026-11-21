# 0207: A figure estimated from like ships

Status: a plan by the architect, written on Nov 21, 2026 on the designer's word: staying with history is good and is to be done where it can be, but this is a game, and to block it for want of some pieces may be wrong; extrapolating is valid. It was read by a reviewer who had not seen it written, and this is the plan with that reading folded in.

Summary: The rules say no real ship gets an invented rating, and a ship lacking one fact is left out of the game. Three passes of research on the Japanese oilers ended one fact short each time, so a Japanese carrier force has no oiler at all, which is further from history than an oiler whose range is a tenth out. This lets a missing figure of a real ship be estimated from like ships by a written rule, says so on the figure, keeps the gap open, and never lets an estimate stand as evidence. It is for what a ship is, never for what happened.

Standing: Planned on Nov 21, 2026; its review folded in; built after decision 0206's oilers.

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
   - the attribute the derivation needs has no row at all for the ship or her class, of any standing or date, and a gap row for the ship says it was looked for;
   - at least one like ship has the rows the method needs.
3. **An estimate is for what a ship is, and the line is drawn so:**
   - **may be estimated:** a displacement, her own fuel, her range, her speed, the weight of cargo she can carry;
   - **never estimated, because it is what happened or what was aboard on a day:** her guns and her radar on a date, the aircraft aboard a carrier, the men a transport carried, a crew's experience, where she was, what she had aboard at a scenario's start, what was lost, how strong a unit was.
   What is never estimated stays a sourced fact, an open gap, or an assumption of a scenario's list marked as one, as "radar unknown means none" is today.

### How one is made

4. **Each estimate is an entry in the derivation list of the scenario that needs the ship:** the ship or class, the attribute, the method, and the architect's name and date. It names an attribute of the rows, such as the light displacement, not a figure of the class file. Nothing is estimated without its entry.
5. **There are two methods, and a new one is a change to this decision.**
   - **By ratio.** The attribute is a row of the ship's own, times the mean of the ratio of the same two attributes over every like class on file. A light displacement from her deadweight is the case in hand.
   - **As her sisters.** The attribute is the least of the rows of the ships of her own class, where she has none. It is for sisters by the class the ships' file gives them, and for no one else.
6. **The like classes are not chosen: they are every class of the same type on file that has both rows,** read as the derivation reads any row, with the exceptions that settle their disputes. The entry cannot pick among them. The mean of them all is read, and the check prints the least and the greatest beside it, so that how far the like ships disagree is seen each time.
7. **The ship's own row is a numeric row like any other,** settled where disputed. Where a source gives a range for a group of ships, the row filed for a ship is its lower end, and says so.
8. **An estimate never rests on an estimate.** A class with an estimated attribute is not a like class for that attribute, in any list; the check looks across the lists.

### How it is marked and kept honest

9. **The sources file of a derived class says "estimated"** for a figure that rests on an estimated attribute: the method, the ship's own rows, each like class and its rows, and the least and greatest. It does not print the rule it would print for a figure read.
10. **The class file that games load carries the mark:** a column that names the estimated figures of each class, empty for a class with none, and only in a class file that has an estimate, so that no class file on file changes.
11. **The gap stays open, and a fact drives the estimate out.** When any row of the estimated attribute is filed for the ship or her class, of any standing, the list's check fails until the estimate's entry is taken out.
12. **An estimate is never evidence, and a test holds that:**
    - a measure of the validation may not rest on a scenario that loads a class with the mark, unless the measure's own notes name the class and say it is estimated; a test refuses it otherwise;
    - a disputed row is not settled by its agreement with an estimate, and no figure is tuned toward one;
    - a scenario file that loads such a class says so in its header, and a test refuses one that does not.
13. **The checker counts them.** The derivation's check prints, for each list, how many attributes are estimated and for which ships.

### What the rules pages say

14. **The pages change when this is built, not before,** since until then the tool can estimate nothing. These lines change together, each to say that a real ship's figure is read from her rows or, where the record has none, estimated from like ships by a written rule that says so, and never made up:
    - `docs/engineering/data-rules.md`: "Do not invent historical figures"; "No real ship, aircraft or weapon gets an invented rating"; "Until it can derive a ship, figures stay placeholders"; "Nothing is guessed"; "Nothing here loosens the source rules";
    - the project's instructions: "Do not invent a historical figure" and "A missing fact is a gap, not a guess";
    - `data/oob/README.md`, which gains a line that an estimate is never filed as a row.

## First use: the Japanese fast tankers

The reviewer worked this from the rows and showed it needs far more than one estimate. It is planned here and built after decision 0206's oilers, whose code it stands on, with a hand calculation of its own.

15. **The ships:** Toho Maru at the Coral Sea; Toho, Toei, Kyokuto and Nippon Maru in August 1942.
16. **What the architect files and settles first,** each with its reason on the row:
    - a class for each ship in the ships' file, Toho Maru's among them, which two sources give differently;
    - numeric rows for her deadweight, her own fuel, her radius and its speed, from the American manual of the war that has them, read as long tons and nautical miles, of low confidence, each the lower end of its group's range;
    - her speed, where two sources differ by half a knot: the lower.
17. **Her light displacement is estimated by ratio** from her deadweight. Today two classes of the type have both rows: the Cimarron class, 7,256 tons light on 18,250, and the Patoka class, 5,375 on 12,600. Their ratios are 0.398 and 0.427 and their mean 0.412, which gives a tanker of 13,431 tons deadweight about 5,535 tons light, between 5,340 and 5,730.
18. **Her cargo of oil is estimated by ratio** from her deadweight too, by weight, from the same classes' cargo over their deadweight. Her cargo in barrels, from a compilation's class page, is not read: 93,000 barrels at the Navy's factor is more than she could carry by her own deadweight, so it is some other measure, and no factor for her oil is on file.
19. **Her guns are not estimated.** No source read gives them. Her list takes the assumption that anti-aircraft guns unknown means none, marked as an assumption of the scenario, as radar is.
20. **Neosho's cargo,** which decision 0206 reads as the least of her sisters', becomes an entry by the second method when this is built, and is marked estimated.
21. **Tippecanoe and decision 0166's transports** follow by the same rules, each in its own list, where the type has like classes on file.

## Rule card

Reads: fact rows, and entries of a derivation list. Writes: a derived class file and its sources file. The game's rules do not change.

## Games on file

None moves. No list on file gains an estimate, and no class file on file gains the column; the first uses are in new lists for new scenario files, as decision 0206's ships are. Neosho's entry changes how her cargo is marked and not its value.

## What it does not do

- It does not estimate anything for a placeholder scenario or a placeholder ship; those are made up and say so.
- It does not let research stop. A gap with an estimate over it is still a gap.
- It does not touch aircraft types or weapons. The same rules would serve them; they are left until a case asks.
- It does not estimate events, positions, loads, losses or strengths.

## What it costs

- **Figures in the game that no source states,** marked as such. A reader of a class file must look at its sources file to know which.
- **Little judgement left in an entry.** The like classes are every class of the type on file, and their mean is read; what is judged is the type a ship is filed under and the rows on file for it. As more classes of a type are filed the estimates move, and are remade.
- **One more thing the derivation does,** with its own check.

## Questions for the designer

None. The designer asked for this; the guard rails are the architect's, and he agreed them on Nov 21, 2026.

## How the plan changed on review

A reviewer who had not seen the plan read it against the derivation, the rows and the rules of tuning on Nov 21, 2026, and was asked for the best case that it was too loose.

- **The first use failed the plan's own test of likeness.** The Cimarron class is a third larger than the Japanese tankers, with turbines and two screws where they have one diesel and one. And the choice of like ship moved the answer by a fifth. So the like ships are no longer chosen: every class of the type on file is read and their mean taken, with the spread printed (rules 5 and 6).
- **Almost any figure could be reached** by choosing the like ship, the rows and the end of a range. Each is now fixed by rule (rules 6 and 7).
- **"Never evidence" could only be kept on a page.** The mark now goes into the class file games load, and tests refuse a validation measure or a scenario header that hides it (rules 10 and 12).
- **The third method, a rate in proportion to size, is cut.** It was not sound and no use needed it. "As a like ship" is narrowed to sisters (rule 5).
- **The line between what a ship is and what happened was not clean.** Guns on a date, aircraft aboard, men carried and a crew's experience are named as never estimated (rule 3).
- **The first use is blocked by much more than a displacement:** no class, no numeric rows, no guns on file, and a cargo in barrels that cannot be made tons. It is now planned in full (rules 15 to 19).
- **More sentences of the rules pages would be false than the one the plan amended.** They are listed, and change when the tool can do it (rule 14).

