# Data rules

Where figures come from, the order-of-battle facts, who settles them, and how the reference material may be used. Moved out of `CLAUDE.md` on Nov 12, 2026, word for word, to keep that file short. `CLAUDE.md` states each rule that fails silently in one line and points here for the rest and the reasons.

## Data rules

- **Do not invent historical figures.** A real ship's figure is read from her rows or, where the record has none, estimated from like ships by a written rule that says so (decision 0207), and never made up. Anything made up is marked `placeholder` in its file and uses a name starting with "Placeholder".
- **Nothing from the War in the Pacific: Admiral's Edition database.** Real data comes from open historical sources, with the source recorded.
- Entity IDs in data files are stable. Never renumber or reuse one. That holds for generated files too: the class ids of a derived ship class file are pinned in its derivation list (`[class_ids]`, each id with its ships), and `oob-derive` refuses a list that generates class files without them (decision 0077).

### Order-of-battle facts

`data/oob` holds what sources say about real ships, aircraft, weapons and battles. `data/reference` holds the figures the simulation uses. Decision 0015 and `docs/research/oob/database-design.md` give the full rules. The ones that matter most:

- **No real ship, aircraft or weapon gets an invented rating.** A game figure for a real entity is derived from fact rows by a written rule, and can name the rule and the rows: read from her rows or, where the record has none, estimated from like ships by a written rule that says so (decision 0207), and never made up. The derivation now exists: `oob-derive`, with its rules in `data/derivation/rules.toml` and decision 0020. Until it can derive a ship, by reading or by an estimate its list enters, figures in `data/reference` stay placeholders.
- **One row is one source's statement.** It carries its source, where in the source, the date read, a confidence and a standing. A missing fact is a missing row, listed in `gaps.csv`. Nothing is guessed: a figure of a real ship is read from her rows or, where the record has none, estimated from like ships by a written rule that says so (decision 0207), never made up, and the gap stays open under it.
- **Cite what was read.** A figure a website quotes from a book cites the website. `docs/research/oob/source-policy.md` says which sources may be used.
- **A source that disagrees gets its own row.** Rows are never overwritten to settle a disagreement. The architect settles it and writes the reason on the row (see "Who settles data" below).
- **A value has to earn `preferred`.** A lone weak row is `unconfirmed`, and a rule may read it only if the rule says so.
- **An exception is one row, one reason and the day it was agreed,** in `data/derivation/rules.toml`: it lets the derivation read a row the preference rules would not, and every figure resting on it says so in the sources file. The tool reads no row that begins after a list's date, so that a later fit does not leak back into an earlier battle. An exception may say `later = true` to read such a row for what did not change, with the reason it was already so; it acts only on a list dated before the row begins, it is refused where a row of the time can be read, and every figure resting on it is marked with the row's date (decision 0200).
- **Engagement rows are history to test against.** A scenario may take starting positions, readiness and contacts already held from them. Its result is compared with the recorded outcome and is never scripted to match it.
- **Run `oob-check` after any change to `data/oob`.**

### A ship that is not a warship

The derivation makes one kind of ship besides the warships: the oiler (decision 0206).

- **A class of the type `oiler` is of the kind auxiliary** and carries the mark that she fuels others at sea. A derived class file has the column `fuels_at_sea` only where its list has an oiler, so no class file of warships changes.
- **No main battery and no torpedo tubes, by her kind,** not by a scenario's assumption: no fit, shell or outfit is read for them. Her anti-aircraft guns and her radar are read as any ship's.
- **Toughness from the light displacement** (`displacement_light_lt`), by the same constant as a warship's standard displacement.
- **Her own fuel and her cargo of oil are read in barrels,** `fuel_capacity_bbl` and `cargo_oil_capacity_bbl`, numeric attributes of a class and of a ship, and made long tons only by the factor of a row, `fuel_oil_barrel_factor_lt` (decision 0166, rule 8). The text notes of the same statements (`fuel_capacity_note`, `cargo_oil_note`) are not read; a numeric row restating one cites the same source and locator. A cargo is a capacity: a load on a voyage (`cargo_oil_load_note`) is never read.
- **Her own fuel and her cargo by weight.** Where no row gives them in barrels for her or her class, and her list estimates neither in barrels, they are read in long tons, `fuel_capacity_lt` and `cargo_capacity_lt`, a capacity by weight (decision 0166, rule 3; decision 0207). Neosho's cargo, which decision 0206 read as the least of her sisters', is an estimate of the second method below.

### A figure estimated from like ships

Decision 0207. A figure of a real ship has one of three standings: read, from her own rows or her class's; estimated, worked by a written rule from the rows of like ships; or placeholder, made up and marked, which a real ship never gets.

- **Only where all of these hold:** the record places the ship in a battle the game plays; the attribute has no row at all for the ship or her class, of any standing or date, and a row of `gaps.csv` names her as looked for; and a like ship has the rows the method needs.
- **For what a ship is, never what happened:** a displacement, her own fuel, her range, her speed, the weight of cargo she can carry. Never her guns or radar on a date, the aircraft aboard, the men carried, a crew's experience, where she was, her load at a scenario's start, what was lost or how strong a unit was; those stay a sourced fact, an open gap, or a marked assumption of a scenario's list (`radar_unknown_means_none`, `anti_aircraft_unknown_means_none`).
- **An entry in the list of the scenario that needs her,** `[[estimate]]`: the ship or class, the attribute of the rows, the method and the architect's name and date. Two methods: **by ratio**, her own row of another attribute times the mean of the ratio of the two over every like class; **as her sisters**, the least of the rows of the ships of her own class. A like class is every class of the same type on file with class rows of both, read as any row is with the exceptions that settle their disputes; the entry does not name them, and the check prints the least and the greatest beside the mean.
- **An estimate never rests on an estimate,** across the lists of the folder, and **a fact drives it out:** once any row of the attribute is filed for the ship or her class the list's check fails until the entry is taken out.
- **Marked where it is used:** the sources file says "estimated" with the method, her rows, each like class and its rows and the range; the class file games load names the estimated figures in its column `estimated`, written only where its list has an entry; the check counts the estimates of each list.
- **Never evidence.** A measure of the validation whose scenario loads a class with the mark names the class in its notes and says it is estimated; a scenario file that loads one says so in its header; both are held by `tests/estimates_never_evidence.rs`. A disputed row is not settled by its agreement with an estimate, and no figure is tuned toward one.
- **An estimate is never filed as a row.**

### The file of sides

A scenario may name a file of sides (`class_sides`, decision 0173): a hand-written CSV in `data/reference` that gives each class of ship and type of aircraft its reference files give the side that uses it, `allied`, `japanese` or `both`, with a note on every row. It is a file of its own so that no generated class or type file changes for it.

- **What a row rests on.** A class or type derived from the order of battle takes the side of the ships or aircraft its derivation list gives it, the nation of each in `data/oob/ships.csv` or `data/oob/aircraft_types.csv`, and its note names the list. A placeholder class takes `both` unless its note says why not. The side of a real class is a fact of the navy that built it and needs no figure.
- **Whole and true.** The loader refuses a file with no row for a class or type the scenario's reference files give, a row for one they do not, one given twice, or a ship or air group of the scenario of a class or type given to the other side alone. Where a side cannot be told from the repository's own data, the row is left out and the scenario does not load: a missing side is not guessed.
- **Its frozen copy.** It is a reference file, so `data/reference/step1` holds a copy with rows for exactly the frozen classes and types, which the scenario reads when it is loaded on the frozen figures.

### Who settles data

- **The architect settles data, not the designer.** Which of two disputed rows to prefer, which weak row to read by exception, what to assume where the sources are silent: the architect decides these to the best of its knowledge and does not ask Amit to approve them. This replaces the earlier practice of a settlement sheet for his approval (decisions 0015, 0019, 0023).
- **Every such choice is still written down:** the reason on the row, the exception in the rules file, the assumption in the scenario's ship list, and a decision record where a choice shapes a scenario. A choice is marked with who made it, so "settled by the architect" is never confused with "stated by a source".
- **Ask for more research when it is worth it.** If a figure matters to a result and rests on a weak or single source, the architect sends researchers (cheaper agents, or the order-of-battle session) to look for a better one before settling, and says in the record what was looked for and not found.
- **This is about data only.** What the game is, the spec, and rules that change how the game plays are still the product managers' to decide.
- **Nothing here loosens the source rules.** No invented figures, no War in the Pacific data, every row cites what was read, and a missing fact stays a gap. A real ship's figure is read from her rows or, where the record has none, estimated from like ships by a written rule that says so (decision 0207), and never made up; an estimate is never a row.

## Reference material

`references/` is local only and ignored by git. It holds the designer's own copy of War in the Pacific: Admiral's Edition: the manuals as PDF and text, a map image and the installers.

- **Mechanics and lessons, not numbers.** Use the manuals to learn how a subsystem was modelled and where it made the player do clerical work. Do not copy their tables, formulas' constants or data. Our figures come from open historical sources, or are marked `placeholder`.
- **Own words, with section numbers.** Notes go in `docs/research/`, written in our own words and citing manual sections. No copied passages.
- **Read just in time.** Before planning a scenario, read the sections `docs/research/witpae-manual-index.md` lists for it. Report how WitP:AE models it, what the player had to do by hand, and what Flag Plot should do differently.
- **The map image is for looking at only.** Do not trace hexes or take terrain from it. The real map is generated from open coastline data.
- **Never commit anything from `references/`.** Do not run or unpack the installers.
- **`references/natural-earth/`** holds public-domain coastlines (Natural Earth 1:10m land and minor islands, 5.1.1), used only by the map tool, never committed.
