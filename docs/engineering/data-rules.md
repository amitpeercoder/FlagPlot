# Data rules

Where figures come from, the order-of-battle facts, who settles them, and how the reference material may be used. Moved out of `CLAUDE.md` on Nov 12, 2026, word for word, to keep that file short. `CLAUDE.md` states each rule that fails silently in one line and points here for the rest and the reasons.

## Data rules

- **Do not invent historical figures.** Anything made up is marked `placeholder` in its file and uses a name starting with "Placeholder".
- **Nothing from the War in the Pacific: Admiral's Edition database.** Real data comes from open historical sources, with the source recorded.
- Entity IDs in data files are stable. Never renumber or reuse one.

### Order-of-battle facts

`data/oob` holds what sources say about real ships, aircraft, weapons and battles. `data/reference` holds the figures the simulation uses. Decision 0015 and `docs/research/oob/database-design.md` give the full rules. The ones that matter most:

- **No real ship, aircraft or weapon gets an invented rating.** A game figure for a real entity is derived from fact rows by a written rule, and can name the rule and the rows. The derivation now exists: `oob-derive`, with its rules in `data/derivation/rules.toml` and decision 0020. Until it can derive a ship, figures in `data/reference` stay placeholders.
- **One row is one source's statement.** It carries its source, where in the source, the date read, a confidence and a standing. A missing fact is a missing row, listed in `gaps.csv`. Nothing is guessed.
- **Cite what was read.** A figure a website quotes from a book cites the website. `docs/research/oob/source-policy.md` says which sources may be used.
- **A source that disagrees gets its own row.** Rows are never overwritten to settle a disagreement. The architect settles it and writes the reason on the row (see "Who settles data" below).
- **A value has to earn `preferred`.** A lone weak row is `unconfirmed`, and a rule may read it only if the rule says so.
- **Engagement rows are history to test against.** A scenario may take starting positions, readiness and contacts already held from them. Its result is compared with the recorded outcome and is never scripted to match it.
- **Run `oob-check` after any change to `data/oob`.**

### Who settles data

- **The architect settles data, not the designer.** Which of two disputed rows to prefer, which weak row to read by exception, what to assume where the sources are silent: the architect decides these to the best of its knowledge and does not ask Amit to approve them. This replaces the earlier practice of a settlement sheet for his approval (decisions 0015, 0019, 0023).
- **Every such choice is still written down:** the reason on the row, the exception in the rules file, the assumption in the scenario's ship list, and a decision record where a choice shapes a scenario. A choice is marked with who made it, so "settled by the architect" is never confused with "stated by a source".
- **Ask for more research when it is worth it.** If a figure matters to a result and rests on a weak or single source, the architect sends researchers (cheaper agents, or the order-of-battle session) to look for a better one before settling, and says in the record what was looked for and not found.
- **This is about data only.** What the game is, the spec, and rules that change how the game plays are still the product managers' to decide.
- **Nothing here loosens the source rules.** No invented figures, no War in the Pacific data, every row cites what was read, and a missing fact stays a gap.

## Reference material

`references/` is local only and ignored by git. It holds the designer's own copy of War in the Pacific: Admiral's Edition: the manuals as PDF and text, a map image and the installers.

- **Mechanics and lessons, not numbers.** Use the manuals to learn how a subsystem was modelled and where it made the player do clerical work. Do not copy their tables, formulas' constants or data. Our figures come from open historical sources, or are marked `placeholder`.
- **Own words, with section numbers.** Notes go in `docs/research/`, written in our own words and citing manual sections. No copied passages.
- **Read just in time.** Before planning a scenario, read the sections `docs/research/witpae-manual-index.md` lists for it. Report how WitP:AE models it, what the player had to do by hand, and what Flag Plot should do differently.
- **The map image is for looking at only.** Do not trace hexes or take terrain from it. The real map is generated from open coastline data.
- **Never commit anything from `references/`.** Do not run or unpack the installers.
- **`references/natural-earth/`** holds public-domain coastlines (Natural Earth 1:10m land and minor islands, 5.1.1), used only by the map tool, never committed.
