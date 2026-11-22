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

## Reviewed, Nov 22, 2026

Read by a reviewer who had not seen it written. What follows is the plan as it now stands; where it differs from the rules above, this holds.

1. **By side only, for now.** No battle on file is later than 1942, so a value from a date could not be seen in any game. A figure may be given a value for each side; the date of a scenario and values from a date wait for the campaign. When they come the dated values are kept whole and read by the game's own date each day, so a game that crosses a date needs no new reader. Rules 2 to 4 above are withdrawn so far.
2. **A value by side rests on a row of a difference in doctrine or practice between the navies.** It may agree with results across battles. It is not fitted to one battle, it is never keyed to a scenario, a ship or a place, and it does not stand in for something the model lacks that both sides share.
3. **A side is not told the enemy's value.** Figures are common knowledge because they are the same for both sides (decision 0100, item 3); a value by side is not. A side's view gives its own value in both places, and a dry run plays the made-up enemy on the side's own: a commander takes the enemy to do as his own navy does. Chapter 1's "figures, in full" and the engineering page on truth and belief are amended to say so.
4. **The loader refuses** an entry for a figure no decision record names, an unknown side, two entries for one figure and side, an entry for a figure the file does not hold, a value outside the figure's range, a TOML float, and an entry with no reason.
5. **The frozen copies gain no such table,** and the frozen-figures test names it among what they are without.
6. **What it amends, in full:** decision 0187's sentence; `docs/step2-spec.md`, tuning rules 1 and 7, which gain the exception of rule 2 here; chapter 1.
7. **A navy within a side is still not told apart.** The first figure given by side is of fuel handled in American and Japanese carriers; a British carrier on the Allied side would read the American value. No such ship is on file. It is noted for the campaign.
8. **The only figure given by side so far is decision 0212's `aircraft_fire_chance`,** which the designer confirmed on Nov 22, 2026.

## As built, Nov 22, 2026

Built with decisions 0211 and 0212 in one pull request, to "Reviewed". The table is `[[by_side]]` in `air_combat.toml`, with `figure`, `side` (`allied` or `japanese`), `value` and `rests_on`; the loader (`crates/sim-data/src/reference.rs`) keeps the list of figures a record names, `GIVEN_BY_SIDE`, now `aircraft_fire_chance` alone, and refuses what rule 4 of "Reviewed" lists. Resolved, a figure is a `BySide` value in the simulation's figures, read with the side of the unit a rule works on. The other figures files refuse the table as any unknown key. `SideView::air_combat_figures` gives the side's own value in both places, and a dry run (`sim-trial`'s `build_world`) is built on the figures as the side knows them (`RefData::as_seen_by`). The frozen copies have no such table, and `crates/sim-data/tests/frozen_figures.rs` names it among what they are without.
