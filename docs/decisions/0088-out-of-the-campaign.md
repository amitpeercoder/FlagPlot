# 0088: Out of the campaign

Status: a plan, written by the architect on Nov 13, 2026, for the designer's approval. Nothing is built. It reads the yard function of decision 0069, which is being specified for its build; the figures here are placeholders for the example only.

## The gap

The objectives scale counts an enemy ship in two ways: sunk, or crippled, which is afloat at the end with propulsion out or flight deck out (decision 0054). Anything less scores nothing.

That misses the commonest serious result of a battle. At the Eastern Solomons Enterprise took three bombs, had her flight deck back within the hour and steamed away under her own power. On the scale she scores nothing for the Japanese. She was away for 53 days. Saratoga, torpedoed a week later, was away 71 with no system out. Boise after Cape Esperance was away 160.

A commander of 1942 counted such a ship as lost to the campaign, and so should the scale. Until it does, the designer has ruled that a result on the scale is reported and not judged (`docs/step2-spec.md`, "Decided by the designer", 4 and 16).

## What decision 0069 gives

Decision 0069, item 6, works out the days a ship needs in a yard from her damage, with no random draw: a figure for each point of flotation damage, a figure for each system degraded and a larger one for each system out, plus the days of the passage. It was written so that this record could read it: "a ship is out of the campaign if the yard would keep her longer than some number of days".

This record uses the days of repair alone, without the passage. The passage belongs to a port; how badly a ship is hurt does not.

## Decisions proposed

1. **A sixth kind of objective, `ships_out_of_campaign`,** with points for each kind of ship, as `ships_sunk` and `ships_crippled` have.
2. **A ship is out of the campaign** if she is not sunk and either of these is true:
   - at the end of the game, her days of repair by the yard function are at or above a figure, `out_of_campaign_days`;
   - during the game she left for a yard (decision 0069, item 5) with days of repair at or above the figure, whether or not she has come back.
3. **A ship counts once, in the highest class she reaches:** sunk, then out of the campaign, then crippled. A ship out of the campaign is not also crippled.
4. **One figure for every scenario.** `out_of_campaign_days` sits with the repair figures in `data/reference`, not in a scenario: one set of figures serves every battle.
5. **No rule reads it and it adds no state.** Like every objective it is a count the umpire reads from the final truth and the event log (decision 0054, item 5). The second half of item 2 reads the yard-departure event, which must therefore carry each ship's days of repair. That is a requirement on decision 0069's build, and the only one.
6. **A side cannot see it during the game.** Its progress line says so, as it does for crippled ships today: the enemy's damage is not something a side knows. At the end both sides are told the umpire's count, with the rest of the result.
7. **The slice's scenarios take the new kind in place of `ships_crippled`** when the scale is set from the record. Until then no scenario changes, and no result changes.

## A small worked case

Placeholder figures, for the example only: 1 day for each point of flotation damage, 10 days for a system degraded, 30 for a system out; `out_of_campaign_days` 30. Points: a carrier sunk 10, out of the campaign 5, crippled 5; a cruiser 4, 2 and 2.

| Ship at the end | Days of repair | On the scale today | With this rule |
| --- | --- | --- | --- |
| Carrier, flotation 35, flight deck degraded | 35 + 10 = 45 | nothing | out of the campaign: 5 |
| Carrier, flotation 10, flight deck out | 10 + 30 = 40 | crippled: 5 | out of the campaign: 5 |
| Cruiser, flotation 12, fire control degraded | 12 + 10 = 22 | nothing | nothing |
| Cruiser, propulsion out, flotation 0 | 30 | crippled: 2 | out of the campaign: 2 |
| Cruiser, flotation 45, sent to the yard on day 3 of 60 and back on day 58, whole | 45 when she left | nothing | out of the campaign: 2 |
| Destroyer, sunk | | sunk: 1 | sunk: 1 |

The first row is Enterprise on 24 August. The fifth is why item 2 has a second half.

## The choices, and why these

**Why the function and not the order.** The scale could count a ship only when her side sends her to a yard. Then a side could deny the enemy his points by keeping a wreck in the line to the end. Reading the damage itself closes that. Decision 0069 said so in advance.

**Why a ship that came back still counts.** Without the second half of item 2, a ship knocked out for 55 days of a 60-day game scores nothing if she returns on day 58. And a side would be paid, in points taken from the enemy, for repairing early. With it, repairing a ship neither gives nor takes points: she counts if she was hurt that badly, whatever her side did next. The player decides about repair on what the ship is worth to him, which is the decision the game wants him to make.

**Why a line and not a slope.** The alternative is points in proportion to the days away, so that Shokaku's 125 days score more than Enterprise's 53. It is finer, and it is closer to the record. It is also harder to read: a result becomes a sum of fractions nobody can check at a glance, and the player and the staff planner of step 3 must reason about days instead of about ships. One line, at about a month, is the simplest rule that says what a commander meant by "out of it". If the scale later needs a second line (out for the season), it is one more figure in the same rule.

**Why it takes the place of crippled in the slice.** A ship with propulsion or flight deck out at the end is almost always out of the campaign as well: one system out is already about a month in a yard. Keeping both in a scenario would be two names for nearly the same ships. `ships_crippled` stays in the engine, for a scenario that wants the state of the fleet on the last day and nothing more.

## What it costs

- A new kind of objective in the loader, the tally, the briefing's objectives line and the result. No state, no hash change, no random draw.
- One figure, a placeholder until the scale is set.
- The yard-departure event of decision 0069 carries each ship's days of repair.
- The rulebook's chapter on objectives and results, and the glossary ("Out of the campaign", and the order of sunk, out of the campaign, crippled).
- Until the scale is set from the record, "out of the campaign" is a statement about placeholder figures, as every result on the scale is today.

## What it leaves out

- **Aircraft and air groups.** A carrier out of the campaign takes her air group's deck with her; the scale counts aircraft only as destroyed.
- **Points that fall with time,** for a ship that will be back before a long game ends. That belongs with objectives that name a flow, which are not planned yet (`docs/design/economy.md`).
- **Setting the points, the levels and the figure from the record.** That is the next piece of work after this and ship repair: history's result on the scale for the three battles, from the facts on each ship's damage and time away.
- **A ship that is forward-repaired during the game** to below the figure does not count, unless she left for a yard first. That is intended: a fortnight with a repair ship is not out of the campaign.

## Rule card

| | |
| --- | --- |
| Reads | Each ship's flotation damage and systems at the end; the yard-departure events and the days of repair they carry; the yard function's figures and `out_of_campaign_days`; the scenario's objectives |
| Writes | Nothing. The count is worked out when the result is asked for |

## What the reports would show

To be added before the designer decides: for Savo Island, the Eastern Solomons and Rabaul, how many ships each side has out of the campaign in the runs of the closing batches, at 30, 60 and 90 days, on the placeholder yard figures, beside what the record has (Chicago after Savo Island; Enterprise after the Eastern Solomons). It is being counted from the batch files.

## Questions for the designer

Each with the architect's recommendation.

1. **Is a ship out of the campaign judged from her damage, by the yard function, and not from whether her side sent her away?** Recommended: yes (item 2).
2. **Does a ship that left for a yard and came back before the end still count?** Recommended: yes (item 2, second half).
3. **One line at a number of days, or points in proportion to the days away?** Recommended: one line, with the number a placeholder of 30 days until the scale is set.
4. **Does it take the place of "crippled" in the slice's scenarios, once the scale is set?** Recommended: yes; crippled stays available in the engine.
5. **Is it built with ship repair, as one piece of work?** Recommended: yes. It is small, it has no state of its own, and its one requirement falls on the yard event.
