# 0078: Which enemy ship a ship fires at

Status: decided by the architect on the designer's delegation, Nov 13, 2026 ("Go ahead and decide all matters that need decisions"; "Yes, it decides"). Not built. The hand calculations are written by the architect before any code is delegated.

## The gap

In a surface fight a ship fires at the enemy ship in its own place: "Ships are matched in ship ID order. The ship in place i of a force's afloat list takes the enemy ship in place (i modulo the number of enemy ships afloat)" (`docs/rules/04`, "Who fires"; decision 0014). What kind of ship the target is plays no part.

At Savo Island that sends two Japanese heavy cruisers at destroyers by the accident of ship numbers: against the southern group Kako and Yubari fire at Patterson, and Kinugasa and Yunagi at Bagley. Over 1,000 runs on the figures of Nov 13, 2026, 26 in 100 of the Japanese shells fired at Allied ships go at destroyers and 22 in 100 of the hits land on them.

| Gun hits | The record | The model, the middle run |
| --- | --- | --- |
| On the five Allied cruisers | 152 to 218 | 78 |
| On the four Allied destroyers | 1 | 21 |

The record is from `data/oob/engagement_outcomes.csv`: Canberra 24 (rows 19, 20), Chicago 1 (37), Vincennes 57 to 74 (58 to 60), Quincy 36 to 54 (69 to 71), Astoria 34 to 65 (82, 83); Patterson 1 (50), Bagley 0 (49), Helm 0 (93), Wilson 0 (96).

So a cruiser takes about 15 hits in the model, about three fifths of what sinks her, and a destroyer takes five and is sunk by the fourth. Allied cruisers sunk misses low and Allied destroyers sunk misses high, for one cause. No figure of the chance to hit mends it: each was tried alone over 1,000 runs, at its cap or doubled, and cruisers sunk stayed at a median of 1 and destroyers at 3, because every one of them raises the destroyers' hits with the cruisers'.

## The record of who fired at whom

"ev" is `data/oob/engagement_events.csv` and "out" is `engagement_outcomes.csv`.

**Guns, Savo Island.** Every row that names the target of a Japanese heavy cruiser's main battery names a cruiser: Chokai at Astoria, "first main battery salvo about 0151, 500 yd short", with the note "Then KAKO (VINCENNES, 9,400 yd), AOBA (QUINCY, 8,250 yd ...)" (ev 94, medium, preferred); Kako at Canberra "with main and secondary batteries" (ev 1044, unconfirmed); Kinugasa "concentrated all the fire power of both her main battery and her port dual-purpose battery on the VINCENNES" (out 3049, unconfirmed); Furutaka at Canberra and Chicago (ev 58, preferred). "For the first few minutes at least, only one cruiser was firing on each of ours" (out 2378, unconfirmed). The one exception is Furutaka firing with Tenryu at the picket Ralph Talbot at 02:14, battery not stated; she is outside the scenario.

What fired at destroyers was lighter: the light cruisers' guns (Tenryu and Yubari at Patterson, ev 74 and its note; Tenryu "shelled the WILSON with her four 5.5-inch guns for about a minute", out 3078) and the heavy cruisers' secondary guns (Kinugasa's 4.7-inch at Helm, ev 1057). Tenryu also fired at Quincy (ev 1060). The destroyer Yunagi "opened fire on the JARVIS", a destroyer, and at nothing else (ev 1061).

The Allied cruisers that fired their main batteries fired at cruisers. Patterson fired at Tenryu and Yubari (ev 80) and Wilson at Kako (ev 98).

**Torpedoes, Savo Island.** Every Japanese salvo with a named target went at a cruiser but one: Chokai's at Canberra and Chicago and at Vincennes, Furutaka's at Canberra and Chicago (ev 58) and one torpedo at Patterson, Kako's at Chicago and Astoria, Kinugasa's and Aoba's at Canberra, and Tenryu "fired six torpedoes at Quincy" (ev 1106, low, unconfirmed). Bagley's four went at what she took for two light cruisers (out 3205, low, unconfirmed).

**Orders.** At Cape Esperance the American admiral's memorandum is the one stated division of fire in the rows: "destroyers to illuminate after radar contact and fire torpedoes at large ships and shell at destroyers and small craft; heavy cruisers to use continuous fire against small ships at short range" (ev 550, high, preferred). The Japanese order at Savo Island had each captain "operate independently as regards gun and torpedo firings" (out 3172, the clerk's reading of a poor scan, unconfirmed).

**What no row gives:** a stated rule of target choice at Savo Island; how often a heavy cruiser's main battery fired at a destroyer (no named target is one, and no frequency exists); which Allied ship each of Chokai's four numbered targets was; Quincy's target. Nothing here rests on the well-known accounts.

So the rows give: cruisers' main batteries at cruisers; light guns and destroyers at destroyers; torpedoes, from cruisers and destroyers alike, at the big ships.

## Decisions

1. **A ship chooses a target for its guns and a target for its torpedoes,** at the start of the round, among the enemy ships afloat. They need not be the same ship.
2. **The kinds are in the order of importance the rules already have** (decision 0012, which decision 0063's default target list also uses): carrier, battleship, cruiser, destroyer, transport, auxiliary. No second order is added.
3. **Guns: like at like.** A ship fires at enemy ships of its own kind. If none is afloat, it fires at those of the nearest more important kind. If none of those is afloat either, it fires at those of the most important kind afloat. Among them it takes the ship in place (p modulo their number), in ship ID order, where p is the firing ship's place among the ships of its own kind in its force.
4. **Torpedoes: at the most important kind afloat.** Among those ships it takes the one in place (i modulo their number), where i is the firing ship's place in its force.
5. **As now:** every ship afloat counts for its place, armed or not; burning or nearer ships do not draw fire; a ship fires its gun salvo first and then its torpedo salvo; a ship sunk in the round still fires in it and still draws that round's fire.
6. **One rule for both sides, by day and by night.**
7. **Where both forces are of one kind the rule gives exactly the pairs it gave before.** Scenarios 4a, 15a, 15b and 16 are cruisers against cruisers: their events do not change by a byte, and that is the proof that no draw moved.

### The words for the rulebook

`docs/rules/04`, "Who fires", in place of "Ships are matched in ship ID order ...":

> A ship chooses a target for its guns and a target for its torpedoes at the start of the round, among the enemy ships afloat. Kinds are in the order of importance: carrier, battleship, cruiser, destroyer, transport, auxiliary. **Guns:** a ship fires at enemy ships of its own kind; if none is afloat, at those of the nearest more important kind; if none of those, at those of the most important kind afloat. Among them it takes the ship in place (p modulo their number), in ship ID order, where p is the firing ship's place among the ships of its own kind in its force. **Torpedoes:** a ship fires at the enemy ships of the most important kind afloat, taking the ship in place (i modulo their number), where i is the firing ship's place in its force. Every ship afloat counts for its place, armed or not. Burning or nearer ships do not draw fire. A ship fires its gun salvo first and then its torpedo salvo. [0014, 0078]

## Rule card

| | |
| --- | --- |
| Reads | The kinds of the ships afloat in both forces (class data), ship IDs, the order of kinds of decision 0012 |
| Writes | Nothing new. A ship's gun salvo and torpedo salvo may now name different targets; each `Salvo` event already names its own |

No state, no new version of the state hash, no figure, no new draw. Each ship's stream and the order of its draws (the gun salvo, then the torpedo salvo) are unchanged.

## What was tried

Three rules, prototyped in a throwaway copy, 1,000 seeds each on the figures of Nov 13, 2026 with decision 0077 in. Each cell is the 5th to the 95th percentile, then the median.

- **(a)** Every ship fires both weapons at the most important kind afloat.
- **(b)** Like at like for both weapons.
- **(c)** Like at like for guns, the most important kind for torpedoes: this decision.

| Measure | The record | Before | (a) | (b) | (c) |
| --- | --- | --- | --- | --- | --- |
| Allied cruisers sunk | 4 | 0 to 2, 1 | 0 to 5, 2 | 0 to 4, 2 | 0 to 4, 2 |
| Northern group cruisers sunk | 3 | 0 to 2, 0 | 0 to 3, 1 | 0 to 3, 1 | 0 to 3, 1 |
| Allied destroyers sunk | 0 | 1 to 4, 3 | 0 to 0, 0 | 0 to 1, 0 | 0 to 0, 0 |
| Japanese ships sunk | 0 | 0 to 1, 0 | 0 to 1, 0 | 0 to 2, 1 | 0 to 1, 0 |
| Gun hits on Allied cruisers | 152 to 218 | 23 to 114, 78 | 45 to 155, 111 | 51 to 134, 99 | 40 to 130, 95 |
| Gun hits on Allied destroyers | 1 | 12 to 32, 21 | 0 to 3, 0 | 0 to 20, 5 | 0 to 22, 7 |
| Gun hits on Japanese ships | 10 to 17 | 2 to 45, 13 | 8 to 63, 23 | 6 to 62, 21 | 6 to 65, 23 |
| Torpedo hits on Allied ships | 4 to 9 | 1 to 7, 3 | 1 to 7, 3 | 1 to 7, 3 | 1 to 7, 3 |
| Japanese gun hits for each thousand shells | 81 to 117 | 37 to 71, 57 | 41 to 73, 60 | 43 to 72, 59 | 39 to 71, 58 |
| Allied gun hits for each thousand shells | 21 to 36 | 10 to 50, 29 | 19 to 56, 37 | 16 to 57, 37 | 16 to 57, 37 |
| Minutes of action | 20 to 30 | 30 to 60, 35 | 35 to 65, 40 | 35 to 65, 40 | 35 to 65, 40 |
| Allied torpedoes fired | 4 to 6 | 8 to 36, 20 | 16 to 36, 36 | 16 to 36, 32 | 16 to 48, 36 |

**How the rule came to be chosen, said plainly.** It was written after these prototypes had been seen. It rests on the rows of who fired at whom, weapon by weapon: (a) scores a little better on the cruisers and was not chosen, because it has destroyers' guns always on cruisers, against the one order in the rows and against Yunagi, and would leave destroyers untouched until every cruiser was gone; (b) was not chosen because its torpedoes go like at like, and the rows have every torpedo salvo but one at a cruiser.

## What it cannot give, and tuning must not chase

- **Chicago's one hit.** She took one hit because she left the fight. With three or four ships firing at her in the model she takes six.
- **Tenryu and Yubari's guns.** The rules have one kind for every cruiser, so the two light cruisers fire their guns at cruisers, where the rows mostly have them firing at destroyers. A kind for light cruisers would be new class data and is not proposed.
- **Even sharing is an accident of ship order.** Seven cruisers on the northern group's three gives Vincennes three firers and Quincy and Astoria two each. The record has one heavy cruiser on each at first and then ships shifting as targets burned.
- **What gets worse, each with its cause.** The minutes of action (median 40 against 20 to 30): the Japanese ended each fight by passing on, which no rule has. Allied torpedoes fired (36 against 4 to 6): more Allied destroyers live to fire, and the rest is readiness (decision 0071's share) and tubes that do not bear (decision 0074, held). Gun hits on Japanese ships (21 to 23 against 10 to 17) and the Allied hit rate at the top of its record: the Allied ships now live longer and fire longer.
- **The Japanese hit rate.** The rule does not touch the chance to hit. It is still 58 to 60 for each thousand shells against 81 to 117, and it is the next tuning round's, now on fights whose hits fall where the record's did.
- **Four cruisers sunk.** The median becomes 2. The rest is the hit rate and what a hit does.
- **A ship taken for a friend.** Duncan and Farenholt were hit by their own cruisers at Cape Esperance. That is a named gap and not this rule.

## What it costs

- `crates/sim-core/src/surface.rs`: the choice of the two targets, and the in-file tests with forces of mixed kinds.
- A new hand calculation, by the architect: eight cases of the choice (own kind present; no own kind and a more important kind afloat; no own kind and nothing more important; torpedoes at the most important kind while the guns go elsewhere; more firers than targets and fewer; a target sunk in the round, and the places counted again; an unarmed ship counting for its place; mixed kinds against one ship, to show the two salvos fire in the old order on the same streams).
- Scenario 4's hand calculation (`tests/scenario_04_savo_island.rs`) has two tables of who fires at whom in the free round; they are worked again under this rule, and the test says which rule it was worked under. The hand-worked scoring of Savo Island in `tests/scenario_10b_objectives.rs` is worked again for the same reason. No other expected number moves.
- The run baselines of every scenario with a night fight between forces of mixed kinds (4, 7 and 8), and the recorded games that hold one, are remade. The three reports are remade and are the baseline tuning starts from.
- `docs/rules/04` and the glossary; a dated note in decision 0014 pointing here.
