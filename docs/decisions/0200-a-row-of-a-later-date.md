# 0200: A row of a later date, read by exception

Status: a plan by the architect, written on Nov 21, 2026, for decision 0198's rule 77. It adds to decision 0029, the derivation. To be read by a reviewer who has not seen it written, and amended on that reading, before it is built.

Summary: The derivation tool reads no fact row dated after a list's date, so that a later fit does not leak back into an earlier battle. Some facts do not change with the years, and the only good source for them may be dated later: the Navy's sheet for the PBY-5A is of September 1943, and Midway is June 1942. This lets an exception say, row by row and with its reason, that a later row is read for an earlier date, and marks every figure that rests on one.

Standing: Planned on Nov 21, 2026; waits for its review.

## Why

What is so today:

- **A row holds on a date** when the date lies between its `valid_from` and `valid_to`. A row whose date is known only to the year or month "cannot be told", and an exception that names it says the architect judged that it holds.
- **A row that begins after the list's date does not hold,** and no exception can make the tool read it. An exception that names one changes nothing.
- **An exception is one row, one reason and the day it was agreed,** in `data/derivation/rules.toml`, and applies to every list.
- **Midway's sixteen PBY-5As** (decision 0198, rules 36, 58 and 77) have a patrol range, a radius, armour and tanks only on a sheet of 1 September 1943 (aircraft type facts 12000 to 12004). Without the flying boats Midway searches 180 nm and the scenario is not built.

## The rules

1. **An exception may say `later = true`.** It then names a row whose `valid_from` is after the list's date, and says the architect judged that what the row states was already so on that date. The tool reads the row as it reads a row that holds.
2. **Its reason says why the fact did not change** between the list's date and the row's: for a range or a radius, that the airframe and its tanks were the same; for a fit, the row or rows of the time that support it. The tool cannot check a reason; the architect's reading of the list does.
3. **It is refused where it is not needed or not meant:**
   - on a row that holds on the list's date, or whose date cannot be told (the plain exception is for those);
   - on a row whose `valid_to` is before the list's date: a row that ended is not read forward by this;
   - where a row of the same entity and fact holds on the date and is preferred: a row of the time is read before a later one.
4. **It applies to a list only when the row is later than that list's date.** On a list dated after the row begins, the exception is a plain one or nothing, as today.
5. **Every figure that rests on one is marked** in the derived sources file, as a figure resting on any exception is, with the words "read from a row of a later date" and the row's date.
6. **No list on file changes.** No exception on file says `later`, and the check of every list passes with its derived files as they are.

## What it is used for first

7. **The PBY-5A at Midway:** range, patrol radius and bombing radius from the sheet (12000 to 12002); armour and protected tanks (12003, 12004) with the reason that the sister type's rows of mid-1942 (12005, 12006, grade C) say most had both by then, and that when they were fitted is a gap that stays open. Type id 5113 in `data/derivation/midway.toml`, and the PBY-5 on her own rows takes 5114 (decision 0198, rule 76).

## Rule card

Reads: an exception's new field; a row's dates; the list's date. Writes: nothing in the simulation. The derived figures of a list that names such an exception, and their sources file.

## What it costs

- **A way for a later fact to reach an earlier battle,** which the tool was built to stop. It is held to one row at a time, each with a written reason and a mark on the figure, and is the architect's to grant.
- **Rulebook and documents:** the page that says how exceptions are read; `docs/engineering/data-rules.md` where it says what an exception is; decision 0029 gains a line pointing here.

## Amended on review

A reviewer who had not seen the plan written read it against the tool's code and the rows on Nov 21, 2026. Where this and the text above differ, this holds.

**Where it acts, and where it is refused (replaces rules 1, 3 and 4).**

8. **`later = true` acts wherever the tool tests a row's date:** the reading of an attribute, the choice between a ship's rows and her class's, and the reading of a slot's items. It is allowed on an exception of the search radius block as on any other.
9. **It is judged list by list.** On a list whose date is before the row's `valid_from`, the row is read. On any other list the exception does nothing and the tool says so in its notes; it never acts as a plain exception there. A row of one date (`valid_from` the same as `valid_to`) after the list's date is later, like any other.
10. **It is refused on a list where another row of the same thing can be read and holds on the date,** whether that row is preferred or named by an exception. "The same thing" is the entity and the attribute, and for a fit the entity, the slot and the item. The clause about a row that ended is dropped: such a row is not later.
11. **A row read this way is always listed as an exception,** preferred or not, so that its figure carries the mark of rule 5 and the row's date.

**A tank that is partly protected (new; adds to decision 0029, 11).**

12. **`partial` is a value of `self_sealing_tanks`, and it reads as no.** The tool takes only yes or no today, and every row on file for either PBY says partial: the Navy's sheet has 603 of 1,478 gallons in protected cells. An aircraft with most of her fuel unprotected is rated as one without protection. No list on file reads a row that says partial, so none changes.

**What the PBYs read (replaces rule 7).**

13. **The PBY-5A, type 5113:**
    - range 2,211 nm from the sheet (12000), by a later exception in the search radius block; her search radius is then 737 nm, against the 700 miles (608 nm) flown;
    - armour yes from the sheet (12003), later. The reason is not that armour does not change, for it does: it is that the one page of the time says most of the boats had some by mid-1942 (12005, low and unconfirmed, and of the PBY-5 or of both, the page does not say). It is the weakest reading in the list and is marked so; a gap row for when the PBY-5A was fitted is added;
    - tanks partial from the sheet (12004), later, read as no by rule 12;
    - strike radius 434 nm, the distance flown on the night of 3 June (12009), which holds and is preferred; the sheet's two radii are not read. Her torpedo from the fit on file (6822).
14. **The PBY-5, type 5114,** on her own rows by plain exceptions (decision 0198, rule 76): armour 12005, tanks 12006 (partial, so no), range 2,694 nm (313) in the search radius block, a search radius of 898 nm, and range with bombs 1,245 nm (314), from which the tool's rule gives her strike radius. Both range rows are one low page with no sources; the list says so, and the gap for a sheet of her own stays open.
15. **Each exception says it is settled by the architect,** as the data rules ask.

**Documents the build changes,** beyond those named: `docs/commands.md` on the tool; `docs/research/oob/database-design.md` on dates; decisions 0020 and 0075 gain a pointing line each; the comments in `rules.toml` on the search radius exceptions and in `midway.toml` on the flying boats.
