# 0224: Rear bases, and supply from outside the map

Status: a plan by the architect, written on Nov 23, 2026, on the designer's question at the table: "Do we have bases with depots? What bases have 'unlimited' supplies and which don't? Is there a system to it?" The answer was that there is none: no base is unlimited, the engine has a supply line from off the map (decision 0196) and a reserve, and no scenario for play uses either. The designer agreed that rear bases are settled first, as the seam an economy later takes over ("OK. Go."). The research is `docs/research/rear-bases-and-supply-1942.md` (rows 14309 to 14477 of `data/oob/place_facts.csv`).

Summary: This is a rule for scenarios, not for the engine. It says, for every base in a scenario, how its stock is replaced: not at all, by a line from off the map, or by ships. It names the test that chooses between them, gives each rear base a reserve, puts the missing rear bases on the maps that have room for them, and says where each figure comes from. It adds no mechanism: the supply line, the reserve, a unit that enters by date, berths and lighters are all built. It then says how an economy takes each piece over.

Standing: Planned on Nov 23, 2026; to be reviewed.

## Why

What is so today (rules chapter 2; decisions 0011, 0112, 0195 and 0196):

- **A base has a stock of supply and of fuel,** and optionally a reserve the staff will not load below. Ships that unload add to it, at the port's berth or lighter rate.
- **A supply line** gives a base tons a day from off the map. No ship carries it; it cannot be found or cut.
- **A force may enter the map on a day** with cargo aboard.
- **No scenario for play has a line or a reserve.** Port Moresby is the Allied side's only base in the Coral Sea game, so an order to hold it has no possible source of supply. Espiritu Santo is the Allied rear base of the 39 days with 960 t and nothing behind it. Rabaul has 10,000 t and 50,000 t of fuel, a placeholder large enough never to run out.

What the record gives (the research):

- **Who fed whom is firm.** Port Moresby from Townsville across the Gulf of Papua; Midway from Pearl Harbor by single ships; Guadalcanal from Espiritu Santo; Espiritu Santo and Efate from Nouméa and the West Coast; Rabaul from Truk and Japan; Wake through Kwajalein.
- **Which legs the enemy struck is firm for the Allied side and partly for the Japanese.** Espiritu Santo to Lunga was hit again and again. Port Moresby was raided throughout. Ships were sunk off Rabaul and near Truk by submarines and aircraft in 1942, without the line being cut. No loss was found on the West Coast to Nouméa or Nouméa to Espiritu Santo legs in July to October 1942, which is silence and not a statement.
- **Ports are described, and some are counted.** Nouméa: one quay of three berths, about 24 ships a month, 5,000 t a day in December. Espiritu Santo and Efate: no piers, everything by lighter. Port Moresby: one ship at a time, slowly. Lunga: 3,000 t a ship as what could be unloaded in a day.
- **Stocks and rates of arrival are mostly not stated.** Firm: Midway's aviation gasoline (543,000 gallons on 3 June, 65,000 a day in use), and the South Pacific's fuel oil as planned (450,000 barrels to Nouméa, then about 225,000 a month; 200,000 every 15 days; 28,000 a day used). Nothing for Rabaul, Truk, Shortland, Port Moresby, Espiritu Santo or Pearl Harbor.

## The rule

### How a base's stock is replaced

1. **Every base in a scenario for play is one of three kinds,** and the scenario's own notes say which and why:
   - **A forward base:** its stock is what it has and what ships bring. Nothing comes from off the map.
   - **A rear base on a line:** it has a supply line from off the map and a reserve.
   - **A rear base fed by ships:** cargo comes to it in forces that enter the map by date, and it has a reserve.
2. **The test between a line and ships is one question: in the period of the scenario, could the enemy forces in the scenario strike the ships that fed the base?** If they could, the cargo comes in ships. If they could not, it is a line. "Could" is read from the record: a leg the enemy struck, or one his forces in the scenario could reach, is ships. The submarine war on shipping far from the scenario's forces is not in the scenario and does not make a leg "ships".
3. **A base is a rear base only where the record says the side fed others from it** in the period. A base that only received is a forward base.
4. **Where the base that fed a forward base lies within the map's extent, it is put on the map** as a base of the side, with its port. Where it lies outside, the cargo enters at the map's edge on the bearing of that base, in ships (rule 2), or the forward base takes the line itself where the test of rule 2 gives a line.
5. **A rear base's reserve** is the stock that feeds its own garrison and aircraft for `rear_reserve_days` (30: see question 1). The staff ships out only what lies above it, as the reserve rule already has it.

### The figures

6. **A stock or a rate the record states is used,** turned into the game's tons by a written rule in the derivation, and cited.
7. **A stock or a rate the record does not state is a placeholder,** marked so in the scenario with what it rests on, and listed as a gap. It is sized by one written rule for every such base, so that no base is set for one scenario: the stock is `rear_stock_days` of what the side's forces in the scenario that draw on the base use in a day at rest, and the line gives that day's use each day.
8. **A port's size is taken from the description where there is no count:** no piers is size 0 (lighters only); one ship at a time is one berth; Nouméa's three berths are three. The berth and lighter rates stay the placeholders of `port_sizes.csv` until a derivation reads the rows (5,000 t a day at Nouméa; 3,000 t a ship in a day off Lunga).

### The three games for play

9. **Coral Sea.** Townsville is put on the map as the Allied rear base, on a line (the Japanese forces of the scenario did not reach it). Port Moresby is a forward base with one berth, fed from Townsville by ships the staff routes; the hold of Port Moresby then has a source the staff can choose (decision 0222, step 3). Rabaul is a rear base on a line: ships were sunk off it in 1942, and none by the forces of this scenario.
10. **Midway.** Pearl Harbor is outside the map. Midway is a forward base fed by ships that enter by date where the record gives a ship and a date (Kitty Hawk, arrived 26 May, before the scenario's first day: so none enters in the days played). Its stock is set from the gasoline on hand on 3 June. Wake is a forward base.
11. **The 39 days.** Nouméa is put on the map as the Allied rear base on a line, with three berths. Espiritu Santo is a rear base fed by ships from Nouméa, port size 0. Lunga and Tulagi are forward bases. Rabaul is a rear base on a line. Shortland is a forward base.

### What an economy takes over

12. **The line's tons a day** are the one figure an economy later sets. A campaign replaces the scenario's fixed rate with what its shipping delivers; nothing else in the game changes.
13. **The ships that enter by date** become the convoys an economy sails, and the test of rule 2 becomes a question asked of each route.
14. **A port's berths** are the limit an economy pushes against: what it can deliver to a theatre is bounded by what the rear port can unload.
15. **Nothing is "unlimited" anywhere,** so there is no special case for an economy to remove.

## What it does not do

- **No new mechanism.** No depot kind, no aviation gasoline as a stock of its own, no limit on a port's daily traffic beyond its berths, no ship held waiting in a queue.
- **No tankers or oilers counted as a pool.** The fuel line to Nouméa is a line.
- **No Japanese rear base beyond Rabaul** in these three games; Truk is outside every map.
- **No change to a frozen scenario** or to a closed proof's scenario. The play scenarios change; they are measured before and after.

## What it costs

- The three play scenarios move. Each is run before and after on the same seeds, and what moved is reported: how long each base lasts, what the staff ships, what each side loses. Nothing is tuned to the result.
- The Coral Sea and the 39 days gain a base each; the briefing and the table show it.
- The staff's supply doctrine now has work in the Coral Sea, where it had none. That is a larger change than the data suggests, and is why step 3 of decision 0222 goes with it.

## Before it is built

1. **Reviewed** by a fresh reader given this record, the research, chapter 2 and the three scenarios.
2. **The derivation rules** for rules 6 to 8, written by the architect.
3. **The architect's statements** for each scenario: what each base is, what it must hold on its first and last day with no enemy, and that an order to hold Port Moresby is supplied.

## Questions for the designer

1. **The reserve and the placeholder stock** (rules 5 and 7) need a number of days the record does not give. The plan uses 30 days for both, marked as placeholders. Say if another figure is wanted.
2. **Midway gets no supply in the days played** (rule 10), because the record's one ship arrived before them. Say if a supply ship should enter anyway, as a placeholder, to give the Japanese side something to cut.

## Amended on review, Nov 23, 2026

A fresh reader was given this record, the research, the rulebook and the code, and found eight things that had to be mended before a build. The architect accepts them. The plan's central claim, that this is a rule for scenarios and needs no new mechanism, was wrong, and so was what the architect told the designer when the question was asked: that the short games are broken for want of a rear base. Where this section and the rules above differ, this section holds. Rules 9 to 11 are withdrawn as written.

### What the review found

- **In the two five-day games supply does not bind.** Port Moresby's 450 t is 30 days for its men in a game of 5. The Allied side has no transport in the Coral Sea, and Townsville is 16 hexes away: no run could finish in the game. A rear base there would change nothing but what the staff's reason says, and a player who named it as a source would have his hold's plan fail for want of a ship. The reason as decision 0222, rule 1 now gives it ("lives on what it has: 30 days of supply today") is the true and sufficient answer in those games.
- **A scenario cannot give a base a port.** Every hex of the generated maps is port size 0 and a base has no port key. Rule 8 needs a loader change, and it read the port table wrongly: the table is by size, not by berths, and three berths at its rates is 9,000 t a day, far above Nouméa in August and September 1942.
- **A large rear base would be chosen as the source of "control an island"** by the rule of most supply above reserve, with no transport lying at it, and supply to Lunga would stop. The choice rule has to be settled first (decision 0222, step 3).
- **Rules 5 and 7 give nothing where nothing draws on the base:** Townsville, Nouméa and Espiritu Santo have no garrison or aircraft in their scenarios, and a line of 0 is refused.
- **Rule 2's "could" was not defined and was applied as "did" where it suited.** By the rows, carriers were detailed towards Townsville; Meiyo Maru was sunk on the Rabaul approach on day 2 of the 39 days carrying men for Guadalcanal; Shortland fed Guadalcanal and was called forward.
- **The three kinds did not hold together:** there was no kind for a rear base fed from another base on the map, and rule 4 gave a forward base a line.
- **Midway's stock from its gasoline would ground Midway:** 543,000 gallons is about 1,460 t against about 2,000 t the game's own figures use in five days, since aviation fuel is one part of the game's supply and not the whole.
- **The 39 days the table plays are copies of a closed proof's files,** and `07_full_slice.toml` feeds a validation measure, a determinism test, a recorded game and two soak scenarios. The plan named no file.
- **The record does give levels of reserve** (`docs/research/oob/supply-levels-and-convoy-cycles.md`): 120 and 90 days authorised for South Pacific bases from July 1942, as minimum reserves to be drawn on in emergency. They are authorised levels and not stocks on hand. "30 days" was the architect's and rested on nothing.
- **The record was overread** in four places: Port Moresby's Townsville convoys are dated from June 1942, not May; no source says Rabaul's line was not cut; Nouméa's 5,000 t a day is December, after a stevedore battalion came; Lunga's 3,000 t is a limit of 29 September, after the 39 days.

### What stands

1. **Nothing is unlimited, and nothing is changed in the two five-day games.** Their bases are forward bases living on their stock, which is true to the days they cover.
2. **The vocabulary,** corrected: a base is **forward** (its stock and what ships bring), **fed from a base on the map** (ships the staff routes), or **on a line** (tons a day from off the map). A base may be both fed and feeding.
3. **A line is a named simplification, not a finding that the leg was safe.** It is used where the ships of a leg are not in the scenario. Where the rows say the leg was struck in the period, the scenario's notes say so beside the line.
4. **The economy's seam** is the line's tons a day and nothing more is claimed for it here. `docs/design/economy.md` asks for cargo at the map's edge in real ships and for quiet routes as a pool of tonnage; a line is the stand-in until then, and the note is not changed by this record.

### What is done next, in order

1. **Measure before anything is planned further.** The 39 days, staff against staff, on 100 seeds, from a copy of the table's game: each base's supply and fuel by day, the first day each falls to its reserve or to nothing, what the staff shipped and what it could not. The question is whether any base's stock binds, where and when. Nothing is changed by it. If nothing binds but the Japanese hunger at Lunga already named (decision 0159), this record ends here and that miss is taken up on its own.
2. **If a rear base binds,** four pieces of work are owed before a scenario changes, each planned and reviewed in its turn:
   - a scenario key for a base's port, with rates from a derivation that reads the rows by their dates;
   - the staff's choice of a source (decision 0222, step 3), so that a far, full base with no transport is not chosen;
   - a base fed from another base on the map, for fuel as well as supply;
   - the reserve, from the authorised levels, with the ruling that turns a level into a stock.
3. **The table's 39 days get a file of their own** before any of that touches them, so that no closed proof's scenario and no frozen figure moves.
4. **The gaps the research lists stay open:** stocks and rates for Rabaul, Truk, Shortland, Port Moresby and Espiritu Santo.

### The designer's questions, as they stand

Both are withdrawn: no reserve figure is set by this record, and Midway is not changed.

## The measurement, Nov 23, 2026

The 39 days were run staff against staff on seeds 1 to 100, in the proofs' arrangement with both sides from their orders and the table's ground figures (the table has no game with a staff on both sides, so this stands in for it). Each base's stock was read at the end of each day. The seeds hardly differ: the staffs choose the same in all of them. The counts are the measuring team's; the architect checked against the scenario file that Espiritu Santo begins with 960 t and 600 t of fuel and Shortland with nothing.

**Stocks bind. The answer to "does a rear base run short" is yes, on both sides.**

- **Espiritu Santo's supply, 960 t, is one run.** The Allied staff ships all of it on day 24, and from day 26 says of every later run "cannot be met: the source holds 0 t", in all 100 seeds. On day 1 it already says the landing at Lunga needs 1,526 t and the source has 960.
- **Espiritu Santo's fuel, 600 t, is gone on day 6,** taken whole by one transport force, whose seventh ship is left short. Lunga's 200 t of fuel is gone on day 4 the same way. No carrier force takes fuel at any base in any seed, and none comes near empty.
- **Lunga's supply fails late.** Its largest drain is bombs (2,375 t a seed), then the men (1,812 t). The Allied stock at the place is at nothing at a day's end in 35 seeds, from about day 38; its aircraft are short of supply for sorties in 75 seeds from day 27, and its men in 54 from day 30.
- **Shortland holds nothing, and the Japanese runs sail from it.** Every lift from Shortland is "cut by the source's 0 t above its reserve: nothing to take" and carries only what was aboard. The Japanese men at Lunga are unfed from day 12 in all 100 seeds and about 1,000 are lost to hunger. This is the named miss of decision 0159's proof, and here is a cause of it that no mend has addressed: the base the runs leave from is empty, while Rabaul, 10,000 t, is never short.
- **Never near binding:** Rabaul's supply and fuel, Tulagi after day 2, the Allied landing places, and every carrier force's fuel.

**What follows.** The second step of "What is done next" is owed: the 39 days need their rear bases. In order:

1. **The table's 39 days get files of their own,** so that what follows moves no closed proof's scenario.
2. **Shortland's stock** is the smallest change with the largest effect, and is data: what the runs loaded there. The research found no figure for Shortland (a gap), so it is a marked placeholder sized by a written rule, or the runs are made to load at Rabaul, where the stock is. The architect rules which when the rows on where the runs loaded are read.
3. **Espiritu Santo on a line,** for supply and for fuel, with the tons a day from the South Pacific's planned fuel deliveries where the rows give them and a marked placeholder for supply. This needs no port, since Espiritu Santo unloaded by lighter and is port size 0 already, and no choice of source, since it is already the source.
4. **Nouméa, a port for a base, and one base fed from another** are not needed for that, and wait.
5. **Each change is measured against this run** on the same seeds, and nothing is tuned to the result.

Not measured: the table's own pairings with a player in one seat; a balance of each base's stock event by event.
