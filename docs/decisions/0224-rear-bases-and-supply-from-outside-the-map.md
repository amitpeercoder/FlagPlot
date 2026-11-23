# 0224: Rear bases, and supply from outside the map

Status: a plan by the architect, written on Nov 23, 2026, on the designer's question at the table: "Do we have bases with depots? What bases have 'unlimited' supplies and which don't? Is there a system to it?" The answer was that there is none: no base is unlimited, the engine has a supply line from off the map (decision 0196) and a reserve, and no scenario for play uses either. The designer agreed that rear bases are settled first, as the seam an economy later takes over ("OK. Go."). The research is `docs/research/rear-bases-and-supply-1942.md` (rows 14309 to 14477 of `data/oob/place_facts.csv`).

Summary: This is a rule for scenarios, not for the engine. It says, for every base in a scenario, how its stock is replaced: not at all, by a line from off the map, or by ships. It names the test that chooses between them, gives each rear base a reserve, puts the missing rear bases on the maps that have room for them, and says where each figure comes from. It adds no mechanism: the supply line, the reserve, a unit that enters by date, berths and lighters are all built. It then says how an economy takes each piece over.

Standing: Planned on Nov 23, 2026; amended on review; measured; the architect's rulings of Nov 23, 2026 on where the Japanese runs load and on Espiritu Santo's stock given, the fuel left open; nothing built.

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

## The architect's rulings, Nov 23, 2026

The designer's direction, as relayed after the measurement: the Japanese runs load at Rabaul, with no invented stock at Shortland; Espiritu Santo's stock is sized from the loads of the runs. The architect read the rows the research of that day filed (`docs/research/guadalcanal-supply-runs-1942.md`; `troop_lift_facts` 12803 to 12890, `place_facts` 14478 to 14560, `land_unit_facts` 15241 to 15258, and the rows held before them) and rules as follows. Each ruling is data and changes no rule of the game. Each is made in the table's own files of the 39 days and nowhere else.

### Ruling 1: the Japanese runs lie at Rabaul and load there; Shortland holds nothing

**What is ruled.** In the table's own files of the 39 days, each Japanese run that the files start at Shortland (task forces 30 and 32 to 39) starts stopped at Rabaul (base 2) with Rabaul as its home port, and may be used one day earlier than the file has it. What each has aboard at the start is unchanged. Shortland (base 7) keeps no supply and no fuel, as ruling E of decision 0101 left it. Task force 31, Ichiki's, is at Rabaul already. The night force at Shortland (task force 6) is not moved.

**Why.** A run takes supply only from the base it lies at. At Shortland, with nothing, a staff can send a run with what the scenario put aboard and can never load one again; that is the cause the measurement found. The record gives two ways out, and only one rests on rows:

- **No row gives Shortland a stock.** The research looked for one in Morison, Saito, Bullard, Miller, Zimmerman, Hough and the records of about fifty ships and found none: the two entries of `gaps.csv` for the place `shortland-islands` say so. What the rows show there is an anchorage: occupied in March 1942 as a relay base (`place_facts` 14478 to 14480), where the August convoy retired (`troop_lift_facts` 12813, 12815), with a station oiler from 6 September (`place_facts` 14328) and seaplanes from 28 August (`place_facts` 14483 to 14485). A stock there would be invented.
- **The rows put the men and the stores at Rabaul.** The Kawaguchi Detachment was landed at Rabaul on 27 August (`troop_lift_facts` 12829) and its first battalion embarked there on 28 August for the Shortlands (12835). The Aoba Detachment reached Rabaul on 31 August (`land_unit_facts` 15241). Uzuki ran food and ammunition "from Rabaul to Guadalcanal" on 24 to 25 August (`troop_lift_facts` 723). The men of 31 August to 7 September came with about two weeks' rations (12821), which were theirs from where they embarked.

**What the rows say against it, and what the ruling gives up.** Every destroyer run of 29 August to 11 September whose start is stated left the Shortlands, and that is firm. One row has stores loaded into destroyers there on 29 August (`troop_lift_facts` 12837, Hough alone). So the ruling is true of the men and of what they ate, and not of the ships that carried them the last leg. It is a named simplification, and the files' notes say so in these words: the passage from Rabaul to the Shortlands in a transport and the run from the Shortlands in destroyers are one passage by one force. What is lost: Shortland as the place the runs sail from, and a run about twice as long (Rabaul to Lunga is 16 hexes, Shortland to Lunga 8). It is given up because the other way needs a base fed from another base on the map, which this record says waits.

**The day earlier.** One row dates the leg: Sado Maru left Rabaul on 28 August and reached the Shortlands on 29 August (`troop_lift_facts` 12835), a day. So each run moved may be used a day earlier than the file has it. For the run of 29 August that day is the row's own; for the others it is worked out from that row and marked so. No row gives where the Oite's run of 16 August loaded; it is moved with the rest and its note says that.

**This overrides ruling E of decision 0101 in these files only.** That ruling put the runs at Shortland for a game in which each run is ordered once, by the scenario, with what is aboard. It stands in every file where that is so.

### Ruling 2: Espiritu Santo's supply is 2,120 t, a placeholder sized from the stated loads of the runs

**What is ruled.** In the table's own files of the 39 days, Espiritu Santo (base 3) starts with 2,120 t of supply in place of 960 t. It is marked PLACEHOLDER, with this working beside it. It is given no supply line.

**The rule the figure is made by.** The stock is the sum of the loads the rows state for the runs that landed at Guadalcanal or Tulagi on or before the scenario's last day, 14 September 1942, wherever they loaded. A run counts once, at the largest total a row states for it. A run with no stated total counts nothing and stays a gap. Tons are taken as printed.

| Run | Tons | Row | Note |
| --- | --- | --- | --- |
| The four destroyer-transports of 15 August | 120 | `troop_lift_facts` 88 (Dyer) | Loaded at Espiritu Santo |
| Alhena, Fomalhaut and the destroyer-transports, 20 to 23 August | 2,000 | `troop_lift_facts` 60 (Dyer: "over 2,000 tons") | Fomalhaut's about 1,500 t (12869, her own report) and the 120 t of rations of 20 or 21 August (12859, 656) are filed on this run and are inside it, not added |
| **Sum** | **2,120** | | |

Not stated, so nothing counted: Manley and Stringham (16 August), William Ward Burrows (20 August), Burrows and Kopara (29 to 30 August), Betelgeuse (31 August), Fomalhaut (3 September), Fuller and Bellatrix (7 to 8 and 14 September). Not counted because it never landed in the days played: Colhoun's 17 t, aboard when she was sunk on 30 August (`troop_lift_facts` 399, whose note says the sentence does not put her in the run of 29 August); Lakatoi's 400 t, lost on passage (12890); Santa Ana, held back (`place_facts` 14456); the convoy of the 7th Marines, which sailed on the last day and landed on 18 September.

**Why wherever they loaded.** Fomalhaut loaded at Nouméa (12869); Alhena and the destroyer-transports at Espiritu Santo (394, 12853); the research has Burrows and Kopara from Efate, which the architect did not find in a row and does not rest on. Nouméa is not a base of the scenario, and this record says it waits. Espiritu Santo is the one Allied base the runs can load at, so it stands for every port they loaded at, and the files' notes say so.

**How far the rows support it.** They support the figure as a least figure of what was carried, and no more:

- **It is not a stock.** No row gives what lay at Espiritu Santo on any day (`gaps.csv`, the two entries for the place `espiritu-santo`), and the base was "in the earliest throes of being built and stocked" in mid-August (`place_facts` 14491, 14494). The gap stays open under the figure.
- **It is low.** Seven runs are counted at nothing because no row gives their tons.
- **It is all there on the first day,** where the record brought it forward over five weeks. A staff may ship early what was not yet there. A line by date would be closer and needs a rate no row gives.
- **Every row it reads is unconfirmed,** and both are one author, Dyer. No derivation reads them; the figure is a scenario's placeholder and is never evidence.

The architect holds that this is what "sized from the loads of the runs" can honestly mean on the rows as they are, and rules it. Were a larger figure wanted, the rows do not give one.

### Ruling 3: Espiritu Santo's fuel is not ruled

The direction speaks of the runs' loads. No run carried fuel oil for ships: what they carried in drums was aviation gasoline, which is part of the game's supply and is inside ruling 2. The rows give no stock of fuel oil at Espiritu Santo and no load of an oiler there: Sabine fuelled eleven ships from 22 to 28 August (`place_facts` 14435), a count of ships; the planned deliveries of fuel oil this record cites are Nouméa's. So nothing sizes the 600 t, and the architect stops here rather than make a figure. The fuel stays 600 t, a placeholder, and the measurement's finding stands unmended: it is gone on day 6. Owed, as research: the oilers that discharged at Espiritu Santo in August and September 1942 and what each gave.

### What the rulings change in "What follows"

The five steps the measurement gave now read:

1. **The table's 39 days get files of their own. Unchanged, and first.** They are made by a script from the files the table plays today (`data/scenarios/table-hunger/07_full_slice.toml`, `07b_slice.toml`, `07c_slice.toml`), as the files of decision 0225 are, with a check that says whether each file is still the one the script makes. `table/scenarios.json` then names them. No file of a proof, no frozen file and no file a test reads is touched.
2. **Shortland: settled by ruling 1.** No stock is set and no placeholder is sized. The script moves the nine runs to Rabaul, a day earlier.
3. **Espiritu Santo: settled for supply by ruling 2,** as a stock and not a line. **Open for fuel** (ruling 3): nothing changes in its fuel until rows are found.
4. **Nouméa, a port for a base, and one base fed from another still wait.** Ruling 1's simplification and ruling 2's "wherever they loaded" are both there because they wait, and both are taken out when they are built.
5. **Each ruling is measured by itself, then both together, against the run of Nov 23, 2026,** staff against staff on seeds 1 to 100: ruling 1 alone, ruling 2 alone, both. Reported: the day each base first falls to nothing, the days the men of each side at Lunga are unfed and the men lost to hunger, what each staff shipped and what it said it could not. Nothing is tuned to what is found, and neither figure is moved for it.

**The staff's choice of a source** (decision 0222, step 3; decision 0228) is not needed for either ruling: in these files the staffs already choose Rabaul and Espiritu Santo. It is planned beside them because the review of this record asked for it before a rear base is added.

**The architect's statements for the files.** The build holds these, each tested where a test can read it:

1. The script's check passes for each of the three files.
2. Read as data, with every comment set aside, each file differs from the file it is made from in this and nothing else: its `name`; base 3's `supply_t`, 2120 for 960; for each of task forces 30 and 32 to 39, `at_base = 2`, `home_port = 2` and `available_from` one less. Its comments differ in the header and in the notes rulings 1 and 2 ask for, beside base 3, base 7 and each run moved.
3. Base 7 has no supply and no fuel on the first day.
4. On the first morning task force 30 may be used (day 9), it lies stopped at Rabaul's hex with 7 t of supply aboard, as the file it is made from had it at Shortland.
5. No file under `data/scenarios/` outside the new folder, and no recorded game, is changed by the pull request.

**What is not claimed.** That the Japanese men at Lunga are then fed. Decision 0159's named miss has other causes on file (the supply formed late, and a run by night that lands 25 t), and the measurement of step 5 says what moved.

### Amended on review, Nov 23, 2026

A fresh reader checked the rulings against the rows and the files. The architect checked each point and rules:

- **Colhoun's 17 t is out of the sum, which is 2,120 t.** The row's own note says Miller's sentence does not put her in the run of 29 August, and she was sunk with the 17 t aboard, so they were not landed. Ruling 2's rule counted "that ship's" load where a run had no total; that clause is taken out, and its table, its figure and the count of authors are corrected above in place.
- **Statement 2 is restated** above: it forbade the notes that rulings 1 and 2 require. The data may differ only as listed; the comments differ in the header and the notes.
- **Task force 39 has the fuel for the move, at cruising speed.** Its two ships are the placeholder fast transport with cargo (class 12: 500 t of fuel, 4,000 nm, 16 knots cruising and 36 full) with 500 t each. The file's own note at Rabaul gives a fast transport's round trip to Lunga as about 139 t at cruise. At full speed the burn is times (36/16) squared, 5.0625, about 704 t, more than a ship holds. So ruling 1 stands for every run moved, and one thing follows that the ruling did not say: **from Rabaul no run can go at full speed and come home,** where from Shortland it could. A run by night is then the run at cruise where the forecast puts its arrival in the dark, and otherwise the run goes by day at balanced and does not sail at preserve the force (rules chapter 8, A run by night). The measurement of step 5 reports how many runs went by night and by day under ruling 1. Nothing is adjusted for it.

