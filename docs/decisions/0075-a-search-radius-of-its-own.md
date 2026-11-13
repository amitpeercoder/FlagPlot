# 0075: An aircraft type has a search radius of its own

Status: settled by the architect on Nov 13, 2026, on the designer's delegation ("Go ahead and decide all matters that need decisions"). A change to a derivation rule, not to a simulation rule: what the rules do with a search range is unchanged (`docs/rules/03-search-contacts-and-claims.md`, an order may name no more than the type's search range). Built Nov 13, 2026 (see "As built"). The five exceptions in "Disputes" were settled by the architect on Nov 13, 2026, with the reasons written there.

## The gap

`oob-derive` derives one radius for a type and writes it as both its strike range and its search range (`crates/oob-derive/src/aircraft.rs`, "radius, search and strike"; decision 0029, item 6: the sourced combat radius where exactly one can be read, else a third of the range with a normal load). A strike and a search are different flights. A strike radius allows for forming up, a fight over the target at full power and a heavy load; a scout carries a light load and flies out, across and back. So the search ranges the derivation gives are strike radii, and the record flew beyond them as a matter of routine:

| Type | Derived search range | Flown or ordered | Rows |
| --- | --- | --- | --- |
| B5N2 | 175 (528 / 3, row 79) | 250 on 23 and 24 August, one line 300 | engagement_events 366, 369, 427, 1200, 1202, 1203 |
| E13A1 | 253 (760 / 3, row 494) | 330 at most on 24 August; 300 ordered on 23 August; 300 at Midway | engagement_events 370, 1201, 1133, 1134, 1142 |
| SBD-3 | 200 (row 280, itself a distance flown) | 180 to 250 at the Eastern Solomons; 265 and 315 at Midway | engagement_events 125, 144, 157, 1166, 1179; aircraft_type_facts 283, 549 |

These are orders repeated over several days, not a ceiling reached once.

## Decision

1. **A type has a search radius of its own,** derived by one written rule for every type:
   - a sourced scouting radius, where exactly one can be read for the period;
   - else a third of its range in the scouting condition, rounded down to a whole mile. The scouting condition is the row that states the range as a scout, or at maximum fuel with no strike load;
   - a type with no such row takes a third of its stated maximum range, by the same rule;
   - a type with no range row at all keeps its strike radius as its search radius, and is listed as a gap;
   - where the existing preference rules cannot settle a type's scouting or maximum range and no scenario orders that type to search, the type keeps its strike radius as its search radius and is listed in the derivation's output and in `data/oob/gaps.csv` as not settled. No exception is written for it. Only a type that some scenario orders to search gets an exception, and each is reported to the architect before it is settled. This keeps the rule from settling disputes that decide nothing.
2. **The strike radius is as decision 0029 has it.** Nothing in this decision moves a strike radius.
3. **The search radius is a ceiling.** A scenario's search order names the distance the record has flown, which is at or below it.
4. **The frozen figures of step 1 are unchanged.** `data/reference/step1` keeps the search ranges it has; the hand-calculation tests do not move.

## The source of the method

The Bureau of Aeronautics' sheets of aircraft characteristics give a scouting radius beside the combat radius. The TBF-1C sheet (1 July 1943; the note on aircraft_type_facts 555) gives combat radii of 225 to 235 nautical miles with a torpedo or bombs, and scouting radii of 340, 580 and 645 nautical miles at scouting loads, which the sheet calls a third of the range in that condition. The rule takes that method: a third of the scouting range. The sheet is later than the battles, so its figures are not read for 1942; its method is.

## The hand derivation

Each figure is cut to a whole mile. Beside each, every search distance flown or ordered for the type that the rows hold; every one must fall at or inside the radius.

| Type | Row the rule reads | Range in the scouting condition | Search radius | Flown or ordered (rows) | Inside? |
| --- | --- | --- | --- | --- | --- |
| B5N2 | aircraft_type_facts 408 (proposed; see Disputes) | 1,075 nm, maximum range | 1075 / 3 = 358.33, so 358 | 250 (366, 369, 1200, 1202, 1203); 230 and 200 (368); one line of 300 (427) | Yes |
| E13A1 | 493 (proposed) | 1,194 nm at maximum fuel, 297 US gallons, 103 knots | 1194 / 3 = 398 | 330 at most (370); 300 (1201); at Midway 300 with 60 to the left (1133, 1134; 40 in 1142), so the farthest corner is about 306 from the start; a Chikuma aircraft turned back 350 from its start (1136) | Yes |
| SBD-3 | 270, "1370 miles as scout" | 1,191 nm | 1191 / 3 = 397 | 180 and 187 (125, 126); 200 (144); 250 with a 500 lb bomb (157, 283); 265 (1166); 315 at Midway, after which the flight ran out of fuel (1179, 549) | Yes |
| TBF-1 | 290, "1450 miles as scout" | 1,260 nm | 1260 / 3 = 420 | 250 (157, 309) | Yes |
| PBY-5 | 313, "3100 miles patrol range" (proposed) | 2,694 nm (statute miles assumed), 1,478 gallons | 2694 / 3 = 898 | 700 from Midway and from Noumea (1159, 1160, 1182, 1183, 1184; 1152); 600, 550, 425 and 250 (1162 to 1164, 1180, 1181, 1185, 1186) | Yes. With row 215 (2,190), 730, and every line still inside |

For the record, the other distances by type found by the research of Nov 13, 2026 (`docs/research/oob/air-search.md`, last section): the battleships' floatplanes 300 miles and their Type 94s 250 (1201); the Haruna's Type 95 150 and 180 (1137, 1138, 1143); Japanese searches of 200 by sector at the Coral Sea, type not given (1149 to 1151); the SBD at 150 to 250 in all four actions; one B-17 to 800 (1161). None is for a type the scenarios hold beyond those above, and none contradicts them.

What it does to the derived figures of the Eastern Solomons: B5N2 175 to 358, E13A1 253 to 398, SBD-3 200 to 397, TBF-1 190 to 420. The strike ranges are unchanged. The other types the scenarios hold (F4F-4, A6M2, D3A1, G4M1) are derived by the same rule when it is built. None is ordered to search in any scenario, so where their rows dispute the range (the F4F-4's 912 against 1,108, the A6M2's 999, 1,373 and 1,677) they keep their strike radius as search radius and are listed as not settled (decision 1).

## Disputes

Where one row states the range as a scout, that is the row the rule asks for. Where two rows state the same quantity differently, `oob-derive`'s existing preference rules apply, and what they cannot settle is a named exception (decision 0020, 3). The five below were settled by the architect on Nov 13, 2026; each reason is written beside its exception in `rules.toml` when it is built. The flown lines are the check in the table above, not a reason for any choice.

| Type | Row | Against | Reason |
| --- | --- | --- | --- |
| SBD-3 | 270 (1,191 "as scout") | 226 (826, "Maximum Range") | It is the one row stating the range in the scout condition, which is what the rule asks for; row 226 states no condition |
| TBF-1 | 290 (1,260 "as scout") | none in the condition | The only row in the scout condition |
| B5N2 | 408 (1,075, Pacific Wrecks, medium) | 78 (1,075, the same source, low); 413 (1,078, Pacific War Online Encyclopedia, low) | The higher confidence of rows agreeing within 3 miles |
| E13A1 | 493 (1,194, TAIC Manual 1 data sheet, medium) | 501 (1,796, Japanese Wikipedia, low) | It states its condition, maximum fuel, and is the sheet whose normal range (494) the derivation already reads; row 501 states no condition and cites a book not read |
| PBY-5 | 313 (2,694, "patrol range", 1,478 gal) | 215 (2,190, PBY-5A, no condition); 323 (2,694, PBY-5A, the same source as 313) | In the record only: built when a scenario holds a PBY. The statute-mile assumption behind 2,694 is a gap to close first |

## What it costs

- `oob-derive` gains a second radius and `rules.toml` gains its rule text, a rule for the scouting condition and the exceptions above (four built; the PBY-5's waits for a scenario that holds the type). The derived aircraft files gain new search ranges; their strike ranges, sweep widths and everything else are unchanged.
- A search order may then name the distance flown. Scenario 5, 7 and 8's Chikuma floatplanes go from 253 to 330 (decision 0073). No other order moves: the American search keeps 200 miles (decision 0073, item 4), though the SBD-3 could then be ordered to the 250 of its afternoon search.
- The run baselines of every scenario whose aircraft file changes are remade; their hashes move only where an order moves, since the search range is read only when an order is checked. The frozen figures are untouched, so no hand calculation moves.

## Rule card

| | |
| --- | --- |
| Reads | The fact rows of a type's range in the scouting condition, or its maximum range; a sourced scouting radius where one exists |
| Writes | The type's `search_range_nm` in the derived aircraft file. No state, no event, no draw, no hash change in the simulation |

## As built

Built Nov 13, 2026, on branch `eastern-solomons-search`, after decision 0070 reached main.

- `data/derivation/rules.toml` has a `[search_radius]` block of its own after the `[fire_rate]` block: `divisor_of_scouting_range = 3` and four `[[search_radius.exception]]` entries (SBD-3 row 270, TBF-1 row 290, B5N2 row 408, E13A1 row 493) with the reasons of "Disputes". No PBY-5 entry: no list derives the type.
- `oob-derive` (`crates/oob-derive/src/aircraft.rs`, `input.rs`, `output.rs`) derives `search_range_nm` on its own when the block is present: a readable `scouting_radius_nm`, else `range_max_nm` / 3 in whole numbers, read with the top-level exceptions and the block's own; else the strike radius, with the figure's rule saying "search radius not settled: ...; the strike radius is kept (decision 0075)", the type listed under "Search radius not settled" in the file's header and in the run's output. Without the block the search range is the strike radius, as before. The block's exceptions are read only for the search radius, and are listed in a header section of their own.
- Derived figures: B5N2 358, E13A1 398, SBD-3 397 (in both the Eastern Solomons and the Rabaul files), TBF-1 420. Strike ranges and every other figure are unchanged. Not settled, keeping their strike radius: F4F-4 (rows 242, 243), A6M2 (28, 29, 365, 386, 518), D3A1 (394) and G4M1 (170), each listed in `data/oob/gaps.csv`.
- Tests (`crates/oob-derive/tests/derive.rs`): a preferred maximum range of 1,200 gives 400 against a strike radius of 175; a sourced scouting radius is read first; a range row that is not preferred is read only through an exception of the block (a top-level exception for another fact does not count; one for the range does); rows that disagree with nothing to settle them, and no range row at all, keep the strike radius and say so, and the run succeeds; without the block the search range is the strike radius; an exception naming a missing row or a row of another type or attribute is a problem; the divisor must be a whole number above zero. No existing expected number changed.
- The rulebook (chapter 5, the air group's figures) and the glossary (Radius) say the search range is a radius of its own.

### The frozen figures, and the proof that failed

The first ruling put the new search ranges into the frozen figures of step 1 as well, on the reasoning that `search_range_nm` is read only as the ceiling of a search order and no step 1 order names more than the old figure, so no step 1 result could move. That was to be proved, not assumed: main at ec6e1de, with only the two frozen aircraft files changed, was run whole. No event, outcome, hash, chance or count moved, but six tests failed, because they assert the figure itself or quote it: two of scenario 5's hand calculation (`the_aircraft_table_is_read_from_the_reference_data`, `the_morning_flies_every_search_and_finds_nothing_and_does_nothing`), one of Flight 300's (`seed_28_the_figures_of_the_hand_calculation`), the agent's Eastern Solomons golden briefing ("search range 200 miles" on eight lines), and two wording tests of the agent (the refusal "at most the aircraft's 200 nm" and the air group's line). So the frozen figures stay exactly as they were, search ranges included: the value that leaves step 1 as it was is the old value.

The principle ruled instead: frozen figures go with frozen scenarios, live scenarios with live figures. Every test that runs scenario 5, 7 or 8 on the frozen figures runs its frozen copy in `data/scenarios/step2-frozen/` (decision 0073, "Frozen figures go with frozen scenarios"), so that no live order that names a distance the frozen ceilings refuse is silently dropped. No expected number, golden or wording changed in those tests.

