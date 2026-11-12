# Design note: materials and economy

Status: a note for discussion, written by the architect on Nov 12, 2026 at the designer's request. Nothing here is decided or built. The pillars place production and war-wide strategy in step 5. This note says what that could be, what exists to build on, and what should be settled early so later steps do not have to be reworked. Historical statements below are general and unsourced; any figure the game uses must come from sourced facts.

## Why it matters

Japan went to war for oil, rubber, tin, bauxite and ore, and could keep fighting only while those reached the home islands by sea. The American answer was to cut that flow with submarines, mines and aircraft. A game of the whole Pacific without this would miss why the fleets were where they were, and would leave the American side with one way to win where it had two.

It does change the dynamics, and it should. Fuel stops being a number at a base and becomes the reason a fleet can or cannot sail.

## Why it is not in the slice

- The South Pacific from August 1942 to February 1943 holds no oilfield, rubber or ore. The economy is fought on the routes from the southern resource area to Japan, which exist only when the map covers the whole Pacific.
- Its effects build over years, and the slice is six months.
- The kill gate after step 4 asks whether commanding by intent is fun. An economy built before that answer could be wasted.

## What exists to build on

- **Supply and fuel are tons** that are carried as cargo, stocked at bases and spent by sorties, aircraft, repairs and steaming (decisions 0039, 0045, 0046). A base out of supply flies nothing.
- **Aircraft and pilots arrive at a rate by the day** (decisions 0038, 0041). Today the rate is a figure in the scenario file. This is the seam: with an economy the rate is what industry delivered.
- **Convoys already work:** a transport force loads, steams, unloads and can be sunk.
- **Objectives are counts read from state** (decision 0054), so "oil landed in the home islands this month" can be an objective the day the thing exists.

## The shape it could take

One chain, four links.

1. **Sources.** A place yields a material each day: oil, ore, rubber and the rest. Captured or damaged sources yield less. Few kinds, not dozens: the game needs the ones whose loss changed the war.
2. **Carriage.** Materials move only by ship. Merchant shipping is a stock that is used, sunk and slowly built. This link is where the war is fought.
3. **Industry.** What reaches home becomes, after a delay, fuel, aircraft, ships and the training of pilots. Short of a material, industry makes less of what needs it.
4. **Delivery.** What industry makes goes forward to the theatres, again by ship, and appears as the arrival and replacement rates and the base stocks the game already has.

Denial acts on link 2 and on link 1: submarines, mines and air attack on shipping, and raids on the sources. Submarines and mines are out of step 1's scope and come with this.

## Command by intent, not by spreadsheet

The economy is where War in the Pacific asks the most clerical work of its player: tankers routed by hand, factories set one by one. Flag Plot must not repeat that.

- The player states aims: keep the oil moving from this field, accept these losses on that route, give aircraft priority over ships this quarter.
- The staff routes the convoys, assigns escorts from what the player released, and reports the plan against what happened, with an interrupt when they drift.
- The player sees a few figures that matter, such as months of fuel in hand and aircraft a month, each of which can explain itself down to the cargoes that arrived or were sunk.

## How the two sides differ

- For Japan the economy is a clock running down: the question is how long the flow can be kept up and what is spent defending it.
- For the United States production is vast and rising, and the question is where to send it and how fast to strangle the other side.
- Neither is "run a national economy". Both are a handful of decisions with long consequences. The research task is to find which handful.

## What to settle early

These cost little now and are expensive to change later.

- **Objectives must be able to name a flow,** not only a place or a count at the end: "200,000 tons of oil landed at home in March". The staff planner of step 3 should be designed with that kind of objective in mind, even if no scenario has one yet.
- **Arrivals should stay a rate fed from outside the theatre.** As long as aircraft, pilots, supply and fuel enter a scenario as schedules and rates, an economy can later drive them without touching the theatre's rules.
- **Fuel should have one meaning.** Ship fuel is tons now. Aviation fuel is folded into base supply. When oil becomes a material, that folding needs a decision.
- **Merchant ships should be countable.** Transports are ships like any other today. A war on shipping needs a way to say how much shipping a side has without naming every hull, and where the line falls between hulls the player sees and tonnage the staff manages.

## What to learn from War in the Pacific first

Before planning, read its production system and resource rules through `docs/research/witpae-manual-index.md` (the index lists the production system at section 13.0, capturing base resources at 15.6 and cargo kinds at 6.3.2), and note in our own words what it modelled, what the player had to do by hand, and what Flag Plot should do differently. Mechanics and lessons only: none of its tables or figures.

## Open questions for the product managers

- How many materials? One ("resources") is too few to make oil matter; a dozen is a spreadsheet.
- Does the player ever set what industry builds, or only priorities?
- Is the economy played for both sides from the first version, or scripted for one?
- Is merchant shipping a stock of tonnage, a list of ships, or both at different distances from the fighting?
- Does the slice get a taste of it earlier, for instance fuel for Truk and Rabaul arriving by tanker on a schedule that can be attacked, so the idea is tested before step 5?

## A possible order

1. Now: keep the seams above as they are, and this note.
2. With step 3: objectives that name a flow, and a staff that can plan a supply line as a lasting plan.
3. With step 4, if wanted: the taste in the slice, tankers on a schedule.
4. Step 5: sources, industry and the war on shipping, with the whole map.
