# 0088: Ships out of action, counted by the time they are away

Status: confirmed. The designer approved the plan on Nov 13, 2026, with the answers at the end. Built on Nov 13, 2026, straight after ship repair (decision 0069), whose yard function it reads, to the architect's specification and hand calculation; see "As built". The figures here are placeholders for the example only.

Summary: Ships out of action, counted by the time they are away: a sixth kind of objective, `ships_out_of_action`; a ship afloat scores her kind's points times her days out over the horizon (never more than 1), where her days out are her days of repair at the end (decision 0069's yard function, without the passage) plus those she left for a yard with; the horizon is the game's length unless the scenario states a longer one; in a scenario that uses it, it takes the place of `ships_crippled`; no state, no draw. Built (scenario 20b)

Standing: Confirmed by the designer, Nov 13, 2026; built Nov 13, 2026

Rewritten the same day. The first version drew one line at about thirty days and called a ship past it "out of the campaign". The designer asked what that means in a game that runs from Pearl Harbor to 1945, where a month away takes no ship out of anything. The line was wrong at that length, and so was the name. This version counts the time a ship is away against the length of what is being fought over. The file keeps its first name so that references to it hold.

## The gap

The objectives scale counts an enemy ship in two ways: sunk, or crippled, which is afloat at the end with propulsion out or flight deck out (decision 0054). Anything less scores nothing.

That misses the commonest serious result of a battle. At the Eastern Solomons Enterprise took three bombs, had her flight deck back within the hour and steamed away under her own power. On the scale she scores nothing for the Japanese. She was away for 53 days of the fight for Guadalcanal. Saratoga, torpedoed a week later, was away 71 with no system out. Boise after Cape Esperance was away 160.

Until the scale counts this, the designer has ruled that a result on it is reported and not judged (`docs/step2-spec.md`, "Decided by the designer", 4 and 16).

## What decision 0069 gives

Decision 0069 works out a ship's **days of repair** from her damage, with no random draw: a figure for each point of flotation damage, one for each system degraded and a larger one for each system out. It does not include the passage to the yard, which belongs to a port and not to the ship. When a force leaves for a yard, the event records each ship's days of repair.

## Decisions proposed

1. **Every result has a horizon,** the number of days it is judged over. It is the length of the game, unless the scenario's objectives state a longer one with `horizon_days`. A campaign needs nothing extra. A tutorial of three days, or a one-day battle kept for validation, states the operation it stands inside (say 60 days), or every damaged ship in it would count as lost for the whole game. The loader refuses a stated horizon shorter than the game.
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

**Why the horizon is the game's length unless stated.** The game that is to be released is campaigns of weeks or more, and a campaign's horizon is its own length: nobody should have to state it. The one-day battles of the slice are fixtures for validation, and a short tutorial is a few days cut from something longer. Those say what they stand inside. One figure for every scenario could not serve both. (The designer, Nov 13, 2026: even a tutorial may need to be a campaign of about three days, and the game is unlikely to ship with many small battles.)

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

## Answered by the designer

On Nov 13, 2026, each as the architect recommended; the second after the designer's own point about tutorials and campaigns, which changed the recommendation.

1. **A damaged ship is counted by the share of a horizon she is away,** not by a line she is past ("Share of a horizon"). Items 1 to 4.
2. **The horizon is the game's length unless the scenario states a longer one** ("Game length, unless stated longer"). Item 1.
3. **She is counted from her damage,** by the yard function, not from whether her side sent her away ("From her damage"). Item 3.
4. **A ship that left for a yard and came back before the end still counts,** for the days of repair she left with ("Yes, she still counts"). Item 3, second part.
5. **It takes the place of "crippled" in a scenario that uses it;** crippled stays in the engine ("Yes, it replaces it"). Item 5.
6. **It is built right after ship repair,** in a pull request of its own, before the scale is set from the record ("Right after ship repair").

Noted by the architect with the plan, and not decided: in a long campaign the fuller measure is the days each ship was in fact missing, counted as the game goes, with objectives that name a flow. This rule is the approximation that needs no state; it is looked at again when long campaigns exist.

## As planned for the build

Written by the architect on Nov 13, 2026, after the designer's answers. It is built on top of ship repair (decision 0069), from which it takes one function and one event. The hand calculation is in `tests/scenario_20b_out_of_action.rs`.

### What it takes from decision 0069

- `repair::days_of_repair(ship, figures) -> u32`: a ship's days of repair from her damage, without the passage.
- The event `ForceLeftForYard`, which lists each ship that left with her days of repair.

Nothing else of ship repair is read, and nothing of it is changed.

### The horizon

`Objectives` gains `horizon_days: u32`, at least 1. The loader sets it: the scenario's `days`, or `horizon_days` from the `[objectives]` table if that is given. The loader refuses a stated horizon that is 0, not a whole number, or less than the scenario's `days`. The engine does not need to know the length of the game: it is handed the horizon with the objectives. Objectives are not state and are not in the hash, so nothing is remade for this.

A scenario may state a horizon and use no `ships_out_of_action` objective. The horizon is then unused, and that is allowed.

### The objective

`Objective::ShipsOutOfAction(ShipPoints)`, added at the end of the enum; its name is `ships_out_of_action`. In a scenario file it is written like the two ship objectives that exist:

```toml
[objectives]
horizon_days = 60          # optional; the scenario's days if left out

[[objectives.allied.objective]]
kind = "ships_out_of_action"
points = { carrier = 5, battleship = 4, cruiser = 2, destroyer = "0.5" }
```

The loader refuses a scenario in which any side has `ships_out_of_action` and any side has `ships_crippled`: a scenario counts damaged ships one way.

### The tally

`Tally` gains `out_of_action`: for each enemy ship with days out above zero, in ship number order, her number, her kind, her days at the end and the days she left with.

For each ship of the enemy, read from the final truth and the log:

- **Sunk:** not counted here, whatever came before. She is in `sunk`.
- **Days at the end:** `days_of_repair` of her damage as it stands, if her location is a task force or a port. If her location is the yard, 0: the damage she carries there is the damage she left with, which the departure event already holds, and it must not be counted twice.
- **Days she left with:** the sum, over every `ForceLeftForYard` event in the log that lists her, of the days of repair it gives for her.
- **Days out:** the two added.

`tallies` already takes the reference data, which holds the yard figures. `crippled` is still counted as it is today, for the scenarios that use it.

### The score

For each ship in the tally, in ship number order:

- her **share** is `min(days out, horizon) / horizon`, one fixed-point division, cut to six places;
- her **points** are her kind's points times her share, one fixed-point multiplication, cut to six places.

The objective's points are the sum of the ships' points. Its `count` is the number of ships listed and `by_kind` counts them by kind. `ObjectiveScore` gains `horizon_days: Option<u32>` and `ships_out`: for each ship her number, kind, days at the end, days she left with, days out, share and points, so that a result can be checked line by line.

### What a side sees

- **During the game:** `known_progress` gives nothing for it, on the basis it gives for `ships_crippled` today (not observable: the enemy's damage is not something a side knows). The briefing's objectives part names the objective, the points for each kind "for a ship away for the whole horizon", and the horizon in days.
- **At the end:** the umpire's result lists each ship with her days out, her share and her points, for both sides, as the rest of the result is shown (decision 0054, item 8). The game is over.

Nothing is added to `SideView` that reads the enemy's truth before the game is over.

### Where else it shows

The result in `sim-cli run`, the export's result object and the viewer's check, the agent's final briefing and `end_day` answer, and the briefing's schema each gain the horizon and the list of ships. The batch file records each run's result as it does today; no version change unless a recorded key changes its meaning, and none does.

### No state, no draw

No field of the state is added and the hash is not touched. No random draw is made. With no scenario using the objective, every scenario's result, event log and hash are as before.

### The rulebook and glossary, in the same pull request

Chapter 6 (objectives and results): the sixth kind, the horizon, the order sunk then out of action, the refusal of the pair with crippled, and what a side sees. The glossary: "Out of action", "Days out", "Horizon". `docs/commands.md` where it describes a result.

## As built

Built on Nov 13, 2026, by a team lead with Sonnet developers, to the section above and the architect's hand calculation in `tests/scenario_20b_out_of_action.rs`, whose numbers all came out as written. Nothing above this section was changed, so its status line still reads as planned.

What was built:

- **sim-core** (`objectives.rs`): `Objectives.horizon_days`; `Objective::ShipsOutOfAction(ShipPoints)` at the end of the enum; `Tally.out_of_action`, one `OutOfAction` (kind, days at the end, days left with) for each enemy ship not sunk with days out above 0, keyed by ship number; `ObjectiveScore.horizon_days` and `ObjectiveScore.ships_out`, one `ShipOut` for each ship with her share and points. The departures are read in one pass over the log. `known_progress` gives `not_observable` for it. `World::set_objectives` refuses a horizon of 0. `SideView::horizon_days` gives the scenario's horizon, which is configuration common to both sides like the levels; nothing added to `SideView` reads the enemy's truth.
- **The loader** sets the horizon and refuses what the section above says. It also refuses `ships_out_of_action` in a scenario without surface combat figures, which hold the yard figures; since `surface_combat` is a required key of every scenario this cannot happen from a file, and is a second line only.
- **Where it shows.** `sim-cli run` prints the count "over N days" and one line for each ship. The agent's briefing gives the objective with `horizon_days` and the points for a ship away for the whole horizon; the result in the final briefing and in `end_day` gives `horizon_days` and `ships_out`. All three new fields are left out of an answer when they do not apply, so no earlier answer changed and the agent interface stays at version 3; the briefing schema has them. A side's own ships are named in `ships_out` by its own handles (decision 0086); an enemy ship, for which a side has no handle, is named `enemy_ship:K`, K being her place in that list, from 1, which tells the side nothing beyond the count. The export has no result object and the viewer reads none, so neither changed.
- **Scenario 20b** (`data/scenarios/20b_out_of_action.toml`) on the map of scenario 20a. The frozen ship classes have no battleship, so a new placeholder class file, `ship_classes_placeholder_battleship.csv` (class 9, class 1's figures with the kind battleship), was added to `data/reference` and its frozen copy to `data/reference/step1`, read by scenario 20b alone. A new file, rather than a row in `ship_classes.csv`, leaves the data-file hashes of every saved game and batch as they were.
- **A leak test** in `crates/sim-agent/tests/objectives.rs`: two worlds that differ only in an enemy ship's damage give the side the same briefing, as JSON and as text, and the same known progress on every day of the game; only the result at the end differs.
- **A wording fix** that ship repair left: the hint shown with the refusal "the task force is under way" now names every order that needs a stopped force, `load` (of supply or fuel) and `send_to_yard`. The recorded games were remade: one line of the full slice game's transcript changed, in the hint only; every save is byte for byte as it was.

No state, no hash change, no random draw. Every other scenario's run baseline, event log and hash are as before; only scenario 20b's baselines are new.

Differences from the section above: none in the rule. The names the section leaves open were chosen as described (the enemy ship's name in the result, the class file).
