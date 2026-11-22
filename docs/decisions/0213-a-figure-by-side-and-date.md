# 0213: A figure that differs by side and by date

Status: a plan by the architect, written on Nov 22, 2026, on the designer's ruling of that day. Asked whether two rules of carrier battle (decisions 0211 and 0212) should run only in the scenarios for play, he said: "It should be applied everywhere, but if there were historical changes in operations doctrines (e.g., carrier air attack doctrines, ship repair squads, firefighting doctrines) that affected these results for either side, that correlate with the results, these could be applied to maybe affect the battles' results. So we have the same rules, but some values may change to set different odds in different battles. Of course, remember we also have the grand campaign." It is to be read by a reviewer who has not seen it written before anything is built.

Summary: Every figure of the game today is one number for both sides and every date. This lets a figure be given, in the file it already lives in, a value for a side from a date, each with the row it rests on. The rule that reads the figure is the same for everyone; the number it reads is the side's own at the scenario's date. A figure given no such value is as it is. It is the way a navy's doctrine and its changes through the war are carried, and it is never keyed to a scenario.

Standing: Planned on Nov 22, 2026; to be reviewed.

## Why

- **The designer's ruling:** one set of rules for every battle, with values that may differ where a navy's doctrine or practice differed and a source says so, and with the whole war in mind.
- **What the game has:** decision 0187 says "the figures of the simulation are one set for every battle and stay so". A figures file gives one value to a name. The one place sides differ is a ship's or a unit's own derived figures. A scenario file has no key for its date; a derivation list has one.
- **What the rows hold** (`docs/research/carrier-doctrine-by-navy-and-date.md`): American carriers drained their gasoline lines before an attack from May 1942 and the Japanese did not; the American navy's firefighting changed in dated steps from 1943; the Japanese order of June 1944 gave each squadron its own target. None of these is a fact about one battle.

## The rules

1. **A figure may be given values by side and date.** In the figures file that holds a figure, a table `[[by_side]]` may give: the figure's name, a side, a date from which it holds, the value, and what it rests on. A figure may have several, for either side and for several dates.
2. **The value a side's units read is the latest that holds:** of the entries for that figure and that side whose date is on or before the scenario's date, the one with the latest date; and where there is none, the figure's plain value, which every figure still has.
3. **A scenario says its date.** A scenario file gains the key `date`, the date of its first day. A scenario without it reads every figure's plain value, as today.
4. **It is resolved when the scenario is read.** The loader gives the simulation each such figure as two numbers, one for each side; the simulation reads no file and no clock, as now. A game is played on the values of its first day. A game that runs across a date at which a value changes is not yet a thing the game has: a campaign will resolve by day, and that is a later record.
5. **Only a figure a decision record names may be given by side.** The loader refuses a `[[by_side]]` entry for any other figure. The first are those of decisions 0211 and 0212.
6. **Every entry says what it rests on,** as every placeholder does, and an entry with no reason is refused. A value by side or date rests on a row of a difference in doctrine or practice. It is not set from the result of a battle, and it is never keyed to a scenario, a ship or a place.
7. **A side is a side of the game,** Allied or Japanese. A navy within a side, the Royal Navy beside the American, is not told apart; the rows gathered are of the American and the Japanese navies.

## What a side learns by it

Nothing. A side is not told the other side's figures, as it is not told any figure.

## Rule card

Reads: the scenario's date; the entries of a figures file. Writes: nothing in a game; the two values of a figure when a scenario is read. Draws: none.

## Games on file

- **A scenario with no date reads the plain values,** so the mechanism alone moves nothing.
- **Every scenario file on file gains its date** with decisions 0211 and 0212, which are to run everywhere: the date is the one its header already states. The frozen scenarios and the frozen figures do not gain the entries, so the hand calculations stand.
- **Saves and recorded games** record the hashes of the files they were made with, and are remade or refuse as at any change to a figures file.

## What it does not do

- **It does not let a figure differ by scenario,** by battle, by ship, by commander or by unit.
- **It does not change a figure during a game.**
- **It does not tell navies of one side apart.**
- **It does not give any figure a value by side.** Each use is a decision record's, with its rows.

## What it costs

- **A table in the figures files, a key in the scenario file, and for each figure so given two values where there was one** in the simulation's figures.
- **Decision 0187's sentence is amended:** the figures are one set of rules for every battle, and a figure may differ by side and date where a row of doctrine supports it.
- **Rulebook:** chapter 1 (where figures come from, the scenario file's keys); the glossary gains "Figure by side".

## Questions for the designer

None. It is his ruling, written down.
