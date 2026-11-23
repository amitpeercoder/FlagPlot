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
