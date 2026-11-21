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

## The first use, settled for its hand calculation

Written on Nov 21, 2026, from the rows as they stand. Where this and rules 15 to 19 differ, this holds; the hand calculation is scenario 121.

- **The first ship is Toho Maru at the Coral Sea,** alone. The record has her with the Striking Force from 1 May and fuelling it on 6 May (ship_facts 3503; engagement_forces 414, 450). In August the oilers that served the carriers were north of the map on the days played, so the five days wait.
- **Her class is the Toa Maru class,** with Toa Maru and Kyokuto Maru: the American manual groups the three, and one of the two class pages that claim her agrees. A class is filed for them, of the type oiler.
- **Her cargo is read from her sister and not from a ratio.** Rule 18 said her cargo in barrels could not be made tons. The class page gives it in tons itself: 93,000 barrels, "about 12,687 metric tons". That is a stated capacity by weight for Toa Maru, and it is under her deadweight. It is filed as 12,487 long tons, and Toho Maru reads it by the second method, as her sister's.
- **Her own fuel, her radius and her speeds are read:** 1,100 tons, 15,000 miles at 16 knots, 19 knots at most, from the American manual of the war, filed as numeric rows of low confidence. Its tons are read as long tons and its miles as nautical, as an American naval manual's are; a Japanese compilation gives Kyokuto Maru the same 15,000 nautical miles at 16 knots. Where the manual gives 19 knots and a compilation 19.5, the lower is read.
- **Her deadweight is 13,431 tons,** the lower end of the manual's range for the three.
- **Her light displacement is estimated by ratio.** A like class's ratio is its light displacement over the deadweight of its ships on file. The Cimarron class: 7,256 over 18,250. The Patoka class: 5,375 over 12,600. Each division is cut at the sixth place, as the derivation cuts: 0.397589 and 0.426587, their mean 0.412088, times 13,431 is 5,534.753928 tons, and toughness is that over 25, 221.390157. The least and the greatest are 5,340.018 and 5,729.489.
- **An oiler's cargo is read by weight where no row gives it in barrels:** the derivation reads the stated cargo capacity in tons, as decision 0166, rule 3, has it for a transport.


## As built

Built on Nov 21, 2026, on branch `estimates-build`.

**The derivation** (`crates/oob-derive`).
- `[[estimate]]` entries of a list (`input.rs`, `parse_estimates`): `ship` or `class`, `attribute`, `method` (`ratio` with `from`, or `sisters`), `by`, `date`, `reason`. Refused at once: an attribute not of rule 3's kinds (the list `ESTIMABLE`: the displacements, her own fuel in tons or barrels, endurance and its speed, maximum speed, cargo in tons or barrels), a ship not in the list, a ratio with no `from` or with `from` the attribute itself, `sisters` for a class or with a `from`, an unknown method, an empty `by` or `reason`, a date that is not one, an attribute entered twice. An entry that names `like_classes` is refused (rule 6); the refusal says, for each class named that has the attribute estimated in any list, that an estimate never rests on an estimate (rule 8).
- Where a figure reads an attribute the list estimates for the ship or her class (`derive.rs`, `fact` and `estimated`), the estimate replaces the reading. It is refused where any row of the attribute is filed for the ship or her class, of any standing or date, with the words "take out the list's [[estimate]] entry" (rule 11), and where no row of `gaps.csv` of the subject's kind names her (rule 2).
- By ratio (`by_ratio`): her own row of `from`, read ship over class with the exceptions, times the mean over every like class of its class row of the attribute over its class row of `from`, each division cut at the sixth place. The like classes are every class in `ship_classes.csv` of her class's type, her own left out, with class rows of both; each is read as any row is; one that cannot be read fails the estimate, and so does one with either attribute estimated in any list of the folder (rule 8); none fails it. As her sisters (`as_sisters`): the least of the readable rows of the other ships of her class in `ships.csv`, a sister whose row cannot be read on the list's date passed over, as decision 0206's rule was.
- The check across the lists: `run` reads the `[[estimate]]` entries of every other `.toml` in the list's folder (`lib.rs`, `sibling_lists`; `input::estimates_of_text`); `derive_among` takes them for a test. An entry no figure reads is a problem.
- Marks: the sources file prints, for a figure on an estimate, a rule beginning "estimated, not read from a row (decision 0207)" in place of the rule of a figure read, and a note with the method, her row, each like class with its rows and ratio, the mean, the least and the greatest (or each sister's row), the entry's author, date and reason. The class file gains a last column `estimated`, naming the figures in column order separated by spaces, only where its list has an entry (`output.rs`, `ESTIMATED_COLUMN`), and a header section "Estimated, not read (decision 0207)". A run and `--check` print the count for every list (rule 13): "estimated (decision 0207): none", or "2 attributes estimated (decision 0207), for toho-maru:" with a line each.
- An oiler's own fuel and cargo are read in barrels where the ship or her class has a row in barrels or the list estimates the attribute in barrels; otherwise by weight, `fuel_capacity_lt` and `cargo_capacity_lt`, and then no barrel factor is read.
- `anti_aircraft_unknown_means_none`: for a ship it names with no fit row of either anti-aircraft slot, her own or her class's, the figure's note and the header say the guns unknown are taken as none, an assumption of the scenario. The figure is 0 in that case with or without the key, as it was; the key only marks it.
- `[least_of_sisters]` is gone; Neosho's cargo is an entry by the second method in `coral_sea_oilers.toml`.
- Unit tests: `crates/oob-derive/tests/derive.rs`, the oilers' tests rewritten for the entry (their numbers unchanged) and twenty new ones from `the_estimate_base_passes_the_checker` on; the three oiler lists against their files.

**sim-data** reads the column (`reference.rs`, `load_ship_classes_marked`, `EstimatedFigures`; a name that is not a figure column is refused) and a loaded scenario answers `Scenario::estimated_figures()`. No rule reads it and it is not in the state.

**Rule 12's tests.** The checks are `crates/sim-cli/src/estimates.rs` (`header_problem`, `measure_problems`), in the runner's library, since `sim-cli validate` is what reads the measures and `sim-data` the class files. `tests/estimates_never_evidence.rs` holds them over every scenario file under `data/scenarios` (a header that says "estimated" and cites decision 0207) and every measures file of `data/validation` (each measure's `reason` naming each marked class and saying "estimated"); scenario 121's statement 12 shows each refusing a file that hides the mark. No measure on file names a scenario with the mark.

**The rows** (each settled by the architect, Nov 21, 2026, with the reason on the row or the exception): the Toa Maru class, `ship_classes.csv` 14303, and the three ships' `class`; `ship_facts.csv` 14709 to 14720 (deadweight 13,431, own fuel 1,100, endurance 15,000 at 16 knots, for Toho, Toa and Kyokuto Maru; Kyokuto Maru's deadweight row disputed, as row 14556 now is); 14721, Toa Maru's cargo of 12,487 long tons; `ship_class_facts.csv` 14343 and 14344, the Cimarron and Patoka classes' deadweight; the attribute `deadweight_lt` for a class; and seven exceptions at the end of `rules.toml`, under "Decision 0207".

**Readings made in the build.**
- The numeric rows are filed for each of the three ships, as the notes they restate are, not for the class.
- The two like classes' deadweight rows are dated to FTP 218, 1 September 1945, as the ships' lines they rest on are, and read by exceptions saying `later = true`, already so as the hulls' deadweight, as decision 0206 read the class's bunkers.
- A like class is read from its class rows only; ship rows of a like class (Kaskaskia's light displacement, the ships' deadweight) are not looked at.
- Rule 2's gap row is any row of `gaps.csv` of the kind `ship` naming her among its entities; Toho Maru's "displacement and fuel burned a day" and Neosho's "own figures" rows serve.
- Her own fuel is read by weight, since no row gives it in barrels (the brief named only the cargo).
- Toho Maru's experience is derived on the Coral Sea's engagement, as her side's ships' are, and comes out 2.
- Her list is `coral_sea_japanese_oilers.toml` (classes 4032), its files `..._japanese_oilers.*`: named with `_oilers` so that scenario 120's statement 6, which passes over the oilers' files by that name, holds unaltered.
- The files of decision 0206's Coral Sea (`base_coral_sea_oilers.toml`, `02a` to `02d`) load Neosho's class, now marked, so by rule 12 their header gains three sentences saying her cargo is estimated; `make_oiler_files.py` makes them so. Nothing else in them changed.
- Scenario 104's statement 23 counts the files with `for_play`: its test now counts the four `03` files, leaves out their base and names the table's `03` copies in place of the `02` ones. Its words are not changed; this reads the architect's amendment for "the files ... with their oilers" as covering them, and is his to confirm.

**The Coral Sea with both oilers.** `scripts/coral-sea/make_both_oilers_files.py [--check]` makes `data/scenarios/coral-sea/base_coral_sea_both_oilers.toml` from the oilers' base and `03a` to `03d_coral_sea_both_oilers.toml` from it: one force more, the Japanese fuelling group (task force 7, Toho Maru, ship 56, class 4032, 1,100 tons of her own fuel and 12,487 of cargo, at (4,-2) with the Striking Force), the class file named after the oilers', and its sides file `class_sides_coral_sea_both_oilers.csv`. The frozen figures gained copies of the new class and sides files, and the frozen copy of the oilers' class file the column `estimated` (`data/reference/step1/README.md`). The table's copies (`scripts/table-copies/make_copies.py`) are made from the `03` files, the `02` Coral Sea copies are gone, and `table/scenarios.json` points the Coral Sea's pairings at them with the line "Each side has an oiler with its carriers".

**Statements.** Scenario 121's sixteen statements and three properties are tested in `tests/scenario_121_a_japanese_oiler_by_estimate.rs`, and all hold. The derivation's figures to the sixth place: the ratios 0.397589 and 0.426587, their mean 0.412088, the light displacement 5,534.753928, toughness 221.390157, the least 5,340.017859 and the greatest 5,729.489997. Statement 5 gives the last two to the third place, one rounded (5,340.018) and one cut (5,729.489); its test holds each within a thousandth. Statement 8's "no class file on file has the column" is read with statement 13: the oilers' class file has it, for Neosho, and no other. Statement 11 is tested as written, a second list whose entry names her class as a like class: rule 6 refuses any entry that names one, and the refusal says the class has its light displacement estimated in her list (rule 8). The case the tool finds for itself, a like class with an estimate in another list, is the unit test `a_like_class_with_an_attribute_estimated_in_another_list_fails_the_estimate`. Statement 12's measure, on file 03a, must also name the Cimarron class, since that file loads Neosho's marked class too.

**Games on file.** No event and no hash moved; see the build's report for how it was checked.

## The architect's rulings

Read on Nov 21, 2026: the estimate in full (`oob-derive`, `derive.rs`: the refusals, the ratio and the sisters), Toho Maru's list and class file, and the build's account of what moved.

1. **The hand calculation holds in every statement,** and the build's eight readings are accepted. The figures are the ones worked by hand: a mean ratio of 0.412088, 5,534.753928 tons light, a toughness of 221.390157, between 5,340.017859 and 5,729.489997.
2. **Statement 8 said no class file on file has the column, and statement 13 gives it to Neosho's.** The two cannot both hold as written; statement 13 is the rule, and the test of statement 8 is right to name the two files that have it.
3. **The Coral Sea files with Neosho say in their headers that her cargo is estimated.** Rule 12 asks it, and the three lines are accepted. Those files' own bytes change and their events do not; a game saved on them before will not load, and none is on file.
4. **Scenario 104's statement 23 covers the Coral Sea with both oilers:** its four files and the table's copies of them carry the key that marks a file for play, as the files they replace at the table did.
5. **Her own fuel is read by weight,** as her cargo is. The brief named only the cargo; the reading is right, since no factor for her oil is on file.
6. **The two lines of `CLAUDE.md` are changed by the architect in this pull request,** as rule 14 says.
7. **Not seen:** a fuelling from Toho Maru played at the table, and her piece on the chart. `sim-cli validate` does not itself refuse a measure that rests on an estimate; the tests do.
