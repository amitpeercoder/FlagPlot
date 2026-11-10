# 0015: Order-of-battle facts: format and rules

Status: confirmed by the designer, Nov 10, 2026 (the format). Items 9 and 10 were added in the Savo Island pass and need review.

The full design is `docs/research/oob/database-design.md`. Which sources may be used is `docs/research/oob/source-policy.md`. This record is the short form.

## The gap in the spec

The spec says real ships and aircraft come from open historical sources with the source recorded, and that nothing may be invented. It does not say where that research is kept, in what form, or how a historical fact becomes a game figure.

## Decisions

1. **Two layers.** Facts are sourced historical statements and live in `data/oob/`. Ratings are game figures derived from facts by written rules and live in `data/reference/`. The simulation loads only `data/reference/`.
2. **No real ship, aircraft or weapon gets an invented rating.** Every game figure for a real entity is produced by a written rule from fact rows, and can name the rule and the rows.
3. **CSV files are the only source of truth.** They follow the conventions of decision 0009: `#` comment lines, whole numbers or decimal strings, no floats. A database file may be generated from them for queries and is never committed.
4. **One row is one source's statement of one fact:** an entity, an attribute from the fixed vocabulary in `attributes.csv`, a value in that attribute's one unit, the source's own figure, a date range, a source, a locator, the date read, a confidence, a basis and a standing.
5. **A missing fact is a missing row.** There are no placeholders, defaults or guesses in the facts layer. What was looked for and not found is listed in `gaps.csv`.
6. **A source that disagrees gets its own row.** Rows are never overwritten to settle a disagreement. `docs/research/oob/conflicts.md` lists the disagreements.
7. **Every entity has a whole-number `id` and a readable `key`.** Neither is ever changed or reused. Real entities start at id 1000. The range below is left to placeholders.
8. **A row cites what was read.** A figure a website quotes from a book cites the website, and the note names the book. Nothing comes from any wargame.
9. **A value has to earn `preferred`.** It needs one row at medium or high confidence, or two rows from different sources that agree, at least one from a source not graded C. Anything less is `unconfirmed`. The ratings layer does not read an unconfirmed value unless a derivation rule says it may, and the rating is then flagged.
10. **A dispute is settled by a person, with the reason written on the row.** A primary document of the owning side, or of a technical examination, outranks a compilation, and a compilation outranks a summary. A higher confidence grade alone settles nothing, because a good source can carry a misprint.
11. **Engagement records are history to test against.** Sightings, positions, alert states and outcomes of a battle are recorded as sourced rows. A scenario may take its starting positions and alert states from them. Its result is compared with the recorded outcome and is never scripted to match it.

## Not decided here

- The derivation tool and the rating rules. They are designed with the combat and detection rules, scenario by scenario.
- A permanent checker for the files. The checks are listed in the design and were run with a script that is not in the repo. Their home is `sim-data` or a small tool crate.
- Whether rows that cite enthusiast sites stay. They are capped at low confidence and under item 9 cannot become `preferred` on their own.

This record adds no rule to the simulation, so it has no rule card.
