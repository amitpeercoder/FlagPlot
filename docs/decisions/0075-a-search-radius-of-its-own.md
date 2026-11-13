# 0075: An aircraft type has a search radius of its own

Status: settled by the architect on Nov 13, 2026, on the designer's delegation ("Go ahead and decide all matters that need decisions"). A change to a derivation rule, not to a simulation rule: what the rules do with a search range is unchanged (`docs/rules/03-search-contacts-and-claims.md`, an order may name no more than the type's search range). Not built: it changes `data/derivation/rules.toml` and `oob-derive`, which wait until decision 0070 is on main. The five exceptions in "Disputes" were settled by the architect on Nov 13, 2026, with the reasons written there.

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
