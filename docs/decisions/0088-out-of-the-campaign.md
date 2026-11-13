# 0088: Ships out of action, counted by the time they are away

Status: a plan, written by the architect on Nov 13, 2026, for the designer's approval. Nothing is built. It reads the yard function of decision 0069, which is being built; the figures here are placeholders for the example only.

Rewritten the same day. The first version drew one line at about thirty days and called a ship past it "out of the campaign". The designer asked what that means in a game that runs from Pearl Harbor to 1945, where a month away takes no ship out of anything. The line was wrong at that length, and so was the name. This version counts the time a ship is away against the length of what is being fought over. The file keeps its first name so that references to it hold.

## The gap

The objectives scale counts an enemy ship in two ways: sunk, or crippled, which is afloat at the end with propulsion out or flight deck out (decision 0054). Anything less scores nothing.

That misses the commonest serious result of a battle. At the Eastern Solomons Enterprise took three bombs, had her flight deck back within the hour and steamed away under her own power. On the scale she scores nothing for the Japanese. She was away for 53 days of the fight for Guadalcanal. Saratoga, torpedoed a week later, was away 71 with no system out. Boise after Cape Esperance was away 160.

Until the scale counts this, the designer has ruled that a result on it is reported and not judged (`docs/step2-spec.md`, "Decided by the designer", 4 and 16).

## What decision 0069 gives

Decision 0069 works out a ship's **days of repair** from her damage, with no random draw: a figure for each point of flotation damage, one for each system degraded and a larger one for each system out. It does not include the passage to the yard, which belongs to a port and not to the ship. When a force leaves for a yard, the event records each ship's days of repair.

## Decisions proposed

1. **A scenario's objectives may state a horizon:** `horizon_days`, the number of days its result is judged over. For a battle it is the length of the operation the battle belongs to. For a long game it is what the scenario's author says it is, most simply the length of the game.
2. **A sixth kind of objective, `ships_out_of_action`,** with points for each kind of ship. The points are what a ship of that kind is worth if she is away for the whole horizon.
3. **A ship's days out** are:
   - her days of repair at the end of the game, if she is afloat and damaged;
   - plus, for each time she left for a yard during the game, the days of repair she left with.

   A ship sunk is not counted here: she is counted as sunk.
4. **Her share is her days out divided by the horizon, and never more than 1.** The side that holds the objective scores her kind's points times her share. Fixed point, cut to six places, as every sum on the scale is.
5. **A ship counts as sunk or as out of action, never both, and not also as crippled.** A scenario that uses `ships_out_of_action` does not also use `ships_crippled`; the loader refuses the pair. `ships_crippled` stays in the engine for a scenario that wants only the state of the fleet on the last day.
6. **No rule reads it and it adds no state.** Like every objective it is worked out from the final truth and the event log when the result is asked for (decision 0054, item 5).
7. **A side cannot see it during the game.** Its progress line says so, as it does for crippled ships today: the enemy's damage is not something a side knows. At the end both sides are told the umpire's count with the rest of the result.
8. **The slice's scenarios take it up when the scale is set from the record,** with their horizons. Until then no scenario changes and no result changes.

## A small worked case

Placeholder figures, for the example only: 2 days for each point of flotation damage, 5 for a system degraded, 30 for a system out. A carrier out for the whole horizon is worth 5 points and a cruiser 2 (their "crippled" values today); a carrier sunk is worth 10.

The same carrier, flotation damage 12 and flight deck degraded: 24 + 5 = 29 days of repair.

| Scenario | Horizon | Her share | Points to the enemy |
| --- | --- | --- | --- |
| A one-day battle in a 60-day operation | 60 | 29 / 60 = 0.483333 | 2.416665 |
| A four-week game | 28 | 1 (29 is more than 28) | 5 |
| A four-year war | 1,300 | 29 / 1,300 = 0.022307 | 0.111535 |

Other ships, with a horizon of 60:

| Ship | Days out | Today | With this rule |
| --- | --- | --- | --- |
| Cruiser, propulsion out, no other damage | 30 | crippled: 2 | 2 x 30 / 60 = 1 |
| Cruiser, flotation 40, every system working | 80 | nothing | 2 x 1 = 2 |
| Cruiser, flotation 6, fire control degraded | 17 | nothing | 2 x 17 / 60 = 0.566666 |
| Cruiser that left for a yard on day 3 with 45 days of repair and came back whole on day 55 | 45 | nothing | 2 x 45 / 60 = 1.5 |
| Destroyer, sunk | | sunk: 1 | sunk: 1 |

The first table is the designer's question answered: the same damage is half a carrier in a two-month fight and next to nothing in a four-year war.

## The choices, and why these

**Why time away and not a line.** A line says a ship is out or she is not. Where the line should be depends on how long the game is, which is the fault the designer found. A share of the horizon needs no line: it is right at every length, and Shokaku's 125 days score more than Enterprise's 53 without a second rule.

**Why the damage and not the order.** The scale could count a ship only when her side sends her to a yard. Then a side could deny the enemy his points by keeping a wreck in the line to the end. Reading her damage closes that. Decision 0069 said so in advance.

**Why a ship that came back still counts.** Without the second part of item 3, a ship knocked out for 45 days of a 60-day game scores nothing if she returns before the end, and a side would be paid for repairing early. With it, repairing a ship neither gives nor takes points. The player decides about repair on what the ship is worth to him, which is the decision the game wants him to make.

**Why the horizon is the scenario's.** A battle scenario lasts a day, and the ships it damages are missed for the two months that follow. Only the scenario knows what operation it stands in. A long game knows its own length. One figure for every scenario could not serve both.

**Why it takes the place of crippled.** A ship with a system out is already about a month from service, so she is counted here. Two kinds in one scenario would be two names for nearly the same ships. It also removes a small unfairness in the scale today, where a cruiser with propulsion out and one a hit short of sinking score the same.

## What the closing reports would show

Counted from the 1,000 runs of each closing batch, on the placeholder yard figures above. "Crippled" is today's rule. The Rabaul scenario has no ships.

| Battle, side | Ships with 30 days of repair or more, mean a run | 60 or more | 90 or more | Crippled today |
| --- | --- | --- | --- | --- |
| Savo Island, Allied | 2.06 | 1.93 | 1.77 | 0.57 |
| Savo Island, Japanese | 2.35 | 1.08 | 0.68 | 0.16 |
| Eastern Solomons, Allied (Enterprise only) | 0.42 | 0.15 | 0.03 | 0.13 |
| Eastern Solomons, Japanese (Ryujo only) | 0.24 | 0.24 | 0.23 | 0.21 |

Two things to read from it. The rule counts three to fifteen times as many damaged ships as "crippled" does, which is the gap. And Enterprise reaches 30 days of repair in about four runs in ten, against the 36 days she spent in the yard in the record: the scale would at last see her. The counts at Savo Island are swollen by destroyers, which take many flotation points from one hit; that is a matter for the yard figures, which are placeholders no source sizes yet (decision 0069, "The yard function's figures from the record").

## What it costs

- A new kind of objective and the `horizon_days` key in the loader, the tally, the briefing's objectives line and the result. No state, no hash change, no random draw.
- The rulebook's chapter on objectives and results, and the glossary: "Out of action", "Days out", "Horizon".
- A result is no longer a count of ships times whole points. It is a sum with fractions, and the result shown to a player must list each ship with her days out and her share so that it can be checked by eye.
- Until the scale is set from the record, the points, the horizons and the yard figures are all placeholders, as every result on the scale is today.

## What it leaves out

- **The days a ship was actually absent,** counted day by day. A ship kept in the line while damaged is counted by what her repair would take, not by what her side chose. That is intended (see "Why the damage and not the order").
- **Aircraft and air groups.** A carrier out of action takes her deck with her; the scale counts aircraft only as destroyed.
- **Forward repair during the game** lowers a ship's days of repair, and so her share, unless she left for a yard first. A fortnight with a repair ship is a fortnight, and the scale should see the difference.
- **Setting the points, the horizons and the yard figures from the record.** That is the next piece of work: history's result on the scale for the three battles, from the facts on each ship's damage and time away.

## Rule card

| | |
| --- | --- |
| Reads | Each ship's flotation damage and systems at the end; the yard-departure events and the days of repair they carry; the yard function's figures; the scenario's objectives and horizon |
| Writes | Nothing. The count is worked out when the result is asked for |

## Questions for the designer

Each with the architect's recommendation.

1. **Is a damaged ship counted by the share of a horizon she is away, and not by a line she is past?** Recommended: yes (items 1 to 4).
2. **Whose is the horizon?** Recommended: the scenario's, stated in its objectives. The alternatives are one figure for every scenario, which cannot serve a battle and a long war alike, or always the days left in the game, which gives a one-day battle a horizon of nothing.
3. **Is she counted from her damage, by the yard function, and not from whether her side sent her away?** Recommended: yes (item 3).
4. **Does a ship that left for a yard and came back before the end still count?** Recommended: yes (item 3, second part).
5. **Does it take the place of "crippled" in a scenario that uses it?** Recommended: yes; crippled stays in the engine.
6. **Is it built straight after ship repair, before the scale is set from the record?** Recommended: yes. It is small and has no state of its own.
