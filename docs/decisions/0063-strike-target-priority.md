# 0063: Strike target priority as an order

Status: confirmed. The designer approved the plan on Nov 12, 2026, with the recommended answer to each of its three questions. Nothing is built yet. The hand calculation is written before any code is delegated, and this record gains an "As built" section when the rule lands.

## The gap

- **A strike group cannot choose.** It goes for the contact of the most important kind in reach, in a fixed order: carrier, battleship, cruiser, destroyer, transport, auxiliary. A commander who wants the convoy sunk, and not its escort, has no way to say so.
- **A strike group cannot wait.** It launches at any contact in reach. A group kept for the enemy carriers flies at the first destroyer reported, and is on its way home when the carriers are found. Two players of the recorded games said they could not choose targets or hold a strike back (`crates/sim-cli/tests/recorded_games/full_slice_five_days_sonnet/account.md`), and the architecture review asked for the order before step 3 (decision 0055, "Left for later").
- **Ryujo's raid on Henderson Field cannot be flown.** A base strike launches in the morning pulse only (decision 0034, item 3, still marked "needs review"), and in the morning she is out of reach. So her attack unit waits for a ship and her aircraft are aboard when she is hit: in 256 of the 265 baseline runs in which she sinks, 23 of her aircraft go down with her, where the record has about 4 (`docs/validation/baseline-reading.md`).

The spec puts this rule before the air figures are tuned (`docs/step2-spec.md`, "Decided by the designer", 5): what a strike goes for decides which ships are hit.

## Decisions

### The target list

1. **A strike group has a target list, set by order:** kinds of ship, in the order the group goes for them. The default is all six kinds in the order of importance, which is the rule as it is. So no scenario changes until someone gives the order, as with reach (decision 0059).
2. **A kind that is not on the list is never launched at.** This is how a strike is held. A group whose list is "carrier" stays on deck until a carrier is reported in reach. There is no second order for holding.
3. **The contact is chosen as now, with the list in place of the order of importance:** the contact whose estimated kind comes first on the list, then the nearest, then the lowest contact number. It reads the side's belief and nothing else. A report that names the wrong kind sends the strike wrong, as it does today.

### Over the target

4. **A strike that has flown attacks what it finds.** The raid attacks the enemy force at its hex that holds the ship coming first on the list of the raid's first strike group. Each group then attacks the ship afloat in that force that comes first on its own list. Kinds not on the list follow, in the order of importance. So a group sent at a "carrier" that turns out to be a cruiser bombs the cruiser. The other choice, to come home with its bombs, was set aside: the pilots are over the enemy, and the cost of a wrong report is already paid in a strike spent on the wrong ships.
5. **Two groups of one raid may now attack different ships** of the force, each by its own list. Today every group of a raid attacks the same ship.

### One function

6. **One function answers "what would this group strike now",** from the side's belief and the group's standing orders. The strike decision calls it, and it is offered through the side's view, so the briefing can say what a group is waiting for and the staff planner of step 3 can ask what a commander will do (decision 0055).

### A base strike in the afternoon

7. **A base strike launches in the first daylight pulse of the day in which its base is in reach,** still once a day. Today it is the morning pulse or not at all. A group whose base is in reach in the morning flies in the morning as before, so the raids from Rabaul do not change. A carrier that closes during the day can now send its raid in the afternoon. This settles item 3 of decision 0034.

## What a player sees

- An order, `set_target_priority`, with a list of kinds. It is a standing order of its own kind. It is refused for an empty list, a kind named twice, another side's group and a group lost with its ship.
- The briefing's air group line shows the list when it is not the default, and what the group would strike now, or that it is holding.
- A launch cites the target order among its causes when the list is not the default.

## A small worked case

One carrier, three dive bomber groups, two contacts in reach: a "cruiser" at 120 miles and a "transport" at 160.

| Group | List | Today | With the rule |
| --- | --- | --- | --- |
| 1 | The default | Strikes the cruiser | Strikes the cruiser |
| 2 | Transport, then cruiser | Strikes the cruiser | Strikes the transport |
| 3 | Carrier | Strikes the cruiser | Stays on deck. Nothing is recorded |

Groups 1 and 2 fly to different hexes, so they are two raids, and the escorts go with the larger. The hand calculation takes this case through to the hits.

## Why

- **It is the commander's intent, stated once.** The player still does not order each strike; the staff does. The list tells the staff what the commander wants hit.
- **One list does two jobs.** Choosing and holding are the same order, so there is one new field and one new order.
- **Off unless given.** The default list is today's rule.

## What it leaves out

- An order given after a contact is seen. Orders stay standing; search, decision and strike still happen inside the day.
- A share of a group held back, a strike split among several ships, and a named contact as a target.
- Holding by size ("not fewer than four ships").
- Target choice in a surface fight.
- Strikers kept back to defend their ship. Aircraft on deck defend nothing.

## What it costs

- One more field of state for an air group, so the hash version rises and every golden file's hashes are remade.
- The property that every group of a raid attacks one ship goes (decision 5), and the tests that lean on it are taught the rule.
- The Eastern Solomons and the full slice get new orders: Ryujo's attack unit is sent at Henderson Field, and the Japanese carriers' bombers are given the list the record supports. These are scenario inputs, settled by the architect from the fact rows, and the baseline report is run again after them. Whether Ryujo's attack planes reach Henderson Field from where the scenario puts her is checked then; it may need extended reach.
- Any scenario in which a base strike group is out of reach in the morning and in reach in the afternoon changes with decision 7. None is known; the golden files will show it.

## Rule card

| | |
| --- | --- |
| Reads | The group's target list; its side's contacts (estimated kind, hex, age); the group's reach; over the target, the enemy forces at the hex and their ships afloat; for a base strike, whether the group has flown at its base that day |
| Writes | The group's target list and the order that set it; the strike launched (working data of the pulse) |

## Answered by the designer

On Nov 12, 2026, as recommended in each case.

1. **Attack what it finds, or come home?** A strike sent at the wrong kind attacks what is there (decision 4).
2. **Is the list enough to hold a strike?** Yes. There is no separate hold order (decision 2): a second order would be a second way to say the same thing.
3. **The afternoon base strike in this record, or its own?** Here (decision 7), because it is what lets Ryujo's raid fly.
