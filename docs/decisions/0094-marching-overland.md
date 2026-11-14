# 0094: Marching overland

Status: confirmed as to the need. The designer asked on Nov 14, 2026 for movement overland to be built before the scenario of the August landings (decision 0093), so that a force that landed along the coast marches to the airfield and is not set down beside it. The rule's details are the architect's, of the same date. Not built. Its three figures are placeholders. The hand calculation is written before any code is delegated.

## The gap

Men move only by sea (`docs/design/ground-combat.md`, answer 1). The Japanese who attacked the airfield in August and September 1942 landed a day's or a week's march from it. Without a march they must be landed in the airfield's own hex, and the days and the tiredness of the approach are lost.

## A limit of the map that shapes the rule

A hex is 40 miles across and has no position inside it. On the real map one hex can hold parts of two islands: the hex of Tulagi also holds the stretch of Guadalcanal's coast where Taivu Point lies. All of a side's men in a hex are one body, so men cannot be put on one island of a hex and not the other. The map does not say which hexes are joined by land either: a hex's closed sides are for ships.

So the rule does not work out by itself where men can walk. The map says it.

## Decisions

### Where men can march

1. **A map lists its land links:** pairs of neighbouring hexes that men can march between. A unit marches only along a link. A map with no list has none, so nothing changes for any existing scenario.
2. **For the real map the list is a file made by hand,** one line for a link, each with its reason. It is geography, not a figure: it says that one landmass runs through both hexes and that the game treats the men in each hex as being on that landmass. A hex that holds two islands is linked for one of them only, and the file says which.

### The order and the march

3. **A march order names a land unit and a hex.** `march`, with `land_unit` and the hex. The hex is a neighbour of the unit's hex joined to it by a land link, or the unit's own hex, which stops a march. It stands until the unit arrives or the order is replaced. It is refused at intake if the unit is not the side's own, is destroyed or has no man ashore, or if there is no land link from its hex to the hex named. Each has its own reason.
4. **A march takes `march_days_per_hex` days.** In the ground combat phase, before the day's fights, each unit with a march order has marched one more day. When it has marched the full number, its men ashore are in the new hex, its order is spent, and it takes part in that day's fight there. A new order to another hex starts the count again.
5. **Until it arrives the unit is where it was.** It eats there, can be hit there and fights there as any unit does.
6. **Only men ashore march.** Men of the unit aboard ships stay aboard; if they later land in another hex than the unit's, they become a new unit by the rule of stage 1 (decision 0066, 14).
7. **A unit may march out of a hex where the enemy is.** There is no pursuit and no rear guard.

### Tiredness

8. **A day's march tires a unit.** A unit that marched that day gains `fatigue_per_march_day` in place of resting. A unit that marched and also fired on attack that day gains both. So a column arrives less able to fire, by the rule of stage 2 that fatigue cuts the men who fire.

### Supply on the march

9. **A unit marches with supply.** When it arrives, `march_supply_days` days of the daily charge for its men are taken from its side's bases in the hex it left, in base order, as far as their stock goes, and put into its side's lowest-numbered base in the new hex. Where the side has none there, a base is formed, as a landing forms one: no guns, no repairs, and not on the enemy's map until a report finds it (decisions 0066, 11 and 12, and 0092, 15).
10. **A unit that leaves from a hex with no base of its side, or an empty one, carries nothing** and is unfed where it arrives unless a base is there. No base is formed for a unit that carries nothing.

### What a side knows

11. **A side sees its own marches whole:** the order, each day marched, the arrival, the supply carried and a base formed. The enemy sees nothing of them. It learns that men have come by a ground report, on the first day both sides have men in the hex.

## Placeholder figures

In `ground.toml`, and in the frozen step 1 figures with the same first values:

| Key | Value | What it is |
| --- | --- | --- |
| `march_days_per_hex` | `3` | Days to march from a hex to a neighbour |
| `fatigue_per_march_day` | `10` | Fatigue a unit gains on a day it marches |
| `march_supply_days` | `5` | Days of its daily charge a unit takes with it |

None rests on a source. The record has two marches inside the period, both from Taivu to the airfield: about two days in August and about a week in September through the jungle inland. One figure cannot be both; it is tuned with the others in stage 4.

## A small worked case

Hexes P and Q are joined by a land link. Unit 1, 1,000 men, fed, fatigue 0, is at P, where its side's base holds 100 t. It is ordered on day 1 to march to Q, where its side has no base.

| Day | Ground combat phase | Unit 1 |
| --- | --- | --- |
| 1 | One day marched | At P, fatigue 10 |
| 2 | Two days marched | At P, fatigue 20 |
| 3 | Three days marched: it arrives. 5 days at 1,000 x 0.005 t is 25 t, taken from the base at P and put into a base formed at Q | At Q, fatigue 30. The base at P holds 100 less three nights' charges of 5 t less 25, which is 60 t; the base at Q holds 25 t |
| 4 | Not marching, not attacking: it rests | At Q, fatigue 25. That night the base at Q pays 5 t and holds 20 t |

If it is on attack and the enemy has men at Q, it fires on day 3 with 1,000 x 80/100 = 800 men (its fatigue before the day's change is 20) and gains 20 more for the attack, 50 after the day.

The hand calculation is in `tests/scenario_25_march.rs`.

## Why

- **The map says where men walk.** Working it out from land percentages would link islands that only share a hex, and would be wrong without failing a test.
- **A march is slow and tiring, and it is supplied by what the column carries.** Those three things are why an approach overland mattered to the fights the record names, and each is one figure.
- **A dump is a base.** Supply ashore lives in bases already; a column's dump is one more, and every rule that feeds men from a base feeds them there.

## What it leaves out

- **Terrain and roads.** Every link takes the same days.
- **Routes of more than one hex.** A unit is ordered hex by hex.
- **Pursuit, retreat by rule, and blocking a march.** A unit marches into and out of a hex whatever the enemy has there.
- **Marching part of a unit.**
- **Losses on the march** to sickness or hunger beyond the unfed rule.

## What it costs

- **New state:** a unit's march order, the hex it marches to and the days marched. They go into the hash and are named beside it; the hash version is raised and the golden files remade (decision 0057).
- **The map gains land links,** in the placeholder map format and as a file beside the real map, and the loader reads them. Only `sim-data` reads the file.
- **A new order with its refusals, new events** with their rows in `side_events.rs`, and the agent interface: the order, a unit's march in the briefing and queries, and which neighbours a unit can march to.
- **The rulebook:** chapter 7 gains marching, chapter 2 the land links, chapter 1 the order, its refusals and the events, with the glossary, in the same pull request as the code. The ground combat plan's first answer, that men move only by sea, gets a dated note.

## Rule card

| | |
| --- | --- |
| Reads | A land unit's men ashore, hex, side, fatigue and march; the map's land links; the bases of the side in both hexes and their stock; the three figures and the daily charge |
| Writes | A land unit's hex, fatigue and march; a base's supply; a new base; the events |
