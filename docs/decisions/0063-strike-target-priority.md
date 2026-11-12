# 0063: Strike target priority as an order

Status: confirmed. The designer approved the plan on Nov 12, 2026, with the recommended answer to each of its three questions. Built (stage 1), Nov 12, 2026: see "As built" at the end. The hand calculation was written before any code was delegated.

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

## As built (stage 1)

Built Nov 12, 2026: decisions 1 to 7. What the code does, and the small choices the decision left open.

- **The order.** `AirGroupOrder::SetTargetPriority { kinds: Vec<ShipKind> }`, added at the end of the enum (`orders.rs`). In scenario files, saved games and `issue_orders` it is `set_target_priority` with `kinds` a list of kind names ("carrier", "battleship", "cruiser", "destroyer", "transport", "auxiliary"). A standing order of its own kind: it replaces the list and the order that set it and nothing else. Two new refusal reasons, added at the end of `RejectReason`: `TargetListEmpty` and `TargetKindRepeated(kind)`. They are tried after "not your unit" and "air group lost", which are reused for another side's group and a group lost with its ship, and the empty list is tried before a repeated kind (the first kind named twice, in list order, is named). It needs no air combat figures and is accepted for any group, as the reach order is. In a scenario file an unknown kind name is refused when the file loads; an empty list or a repeated kind loads and is refused at intake.
- **State and hash.** `AirGroup.target_priority` (default every kind in the order of importance, `ShipKind::ALL`), `AirGroup.target_order` (default none) and `AirGroup.last_base_strike_day` (the game day of the group's last base strike launch, default none). Hashed last in the group, after `reach_order`, in that order: the list's length, each kind's code in list order, the order, the day as an optional number. Named in `hash::name_every_field`. `hash::STATE_VERSION` is `flagplot-state-v5`. `AirGroup::target_rank(kind)` is where a kind comes in the group's preference: kinds on the list first, in list order, then the others in the order of importance.
- **The one function.** `contact_to_strike(belief, group, origin, reach_nm, max_age_pulses)` in `airstrike.rs`, returning a `StrikeChoice` (kind, distance, contact, hex, report event). A contact qualifies if it is no older than the figure, within the reach and taken for a kind on the list; of those the best by (rank, distance, contact number). It takes the belief, the group and numbers, so it can read no enemy truth. `decide_strikes` calls it, and `SideView::strike_choice(group)` calls it for a group of the side's own on the naval strike mission (`None` for any other group or side). The briefing does not call it: its "holding" warning is made from the contacts and the group's list directly.
- **Causes.** A group that has a target order cites it, after its mission order and its reach order and before the report, as a cause of `StrikeLaunched`. A group with none cites nothing new, so no log of a game with no target order changed. A group given the default list by order cites it, though the list is the default.
- **Over the target.** `find_target` takes the raid's first strike group and returns the force: the lowest (rank of the best ship afloat by that group's list, force number). `ship_to_attack` gives each group the lowest (rank, ship number) in that force by its own list. Everything is worked on the start-of-step state, as before. `RaidTarget::Ships` now holds the force alone; the ship each group attacks is chosen in `work_out_raid`. Everything keyed on the force (the streams, the guns, the claims, the patrol shares) is as before.
- **The report's true kind.** The sighting report an attack sends keeps its true kind as the most important kind among the ships afloat in the force attacked (chapter 3), whichever ship the group attacked. The decision did not say; this keeps every report as it was, and a group that attacks a destroyer in a force with a cruiser still reports a cruiser force.
- **Base strike.** A base strike launches in any daylight pulse if it has not launched that game day (`last_base_strike_day` is not the day), and the existing conditions hold. The launch sets the day. Reach is measured from where the group is when the pulse begins, as before.
- **What a player sees.** The briefing's air group line gains `target_priority` (a list of kind names) only for a group whose list is not the default. The warnings gain `strike_group_holding`, in the briefing's data and its text only when there is one: a group on naval strike that holds a contact within its reach, none of a kind on its list, with the list, the nearest such contact, the kind it is taken for and its distance. The age of the contact is not looked at, as the out-of-reach warning does not. The decision's "what the group would strike now" on the line is not built; `SideView::strike_choice` is there for it. The MCP description of `issue_orders`, `order.schema.json`, `defs.schema.json` and `briefing.schema.json` say all this. The replay export gives a group `target_priority` only when it is not the default, and a refusal's reason as `target_list_empty` or `target_kind_repeated` (with the kind).
- **Scenarios 13a and 13b** (`data/scenarios/13a_target_priority.toml`, `13b_afternoon_base_strike.toml`) are built to the hand calculations, on placeholder data. 13a is given the figure `strike_contact_max_age_pulses` 1 by its test; with the live figure, 0, a contact the side starts with is too old to strike in the morning, so the run baseline of 13a shows no launch.
- **A mismatch with the hand calculation of 13a.** Its property says that over seeds 1 to 1000 and both pulses group 3 (list: battleship) is never launched. In 111 of 1,000 runs it is launched in the afternoon, at a contact reported as a battleship. A strike that attacked sends a sighting report, and the report's kind goes through the confusion table (`search.toml`): a cruiser force is reported as a battleship at some chance. A contact so reported is a battleship contact, which is on group 3's list, so by decision 3 it launches at it ("a report that names the wrong kind sends the strike wrong"). In the morning, on the starting contacts, group 3 never launches, as the table says. The test asserts what holds: none in the morning, and in the afternoon only at a contact reported as a battleship. The doc comment of the hand calculation is as the architect wrote it; the architect is to say whether the property or the scenario changes.
- **Tests of existing scenarios taught the rule.** `scenario_06c_fatigue_and_rest`: its model of the afternoon is the morning's (`work_the_pulse`), since a group that can fly no aircraft in the morning for its fatigue, and one in the afternoon once the morning's rest has lowered it, now launches its raid in the afternoon (seed 436, day 7, is one: fly(1, 9) is 0, fly(1, 0) is 1); and "the afternoon has no raid" is "one raid a day at most". A note in its doc comment says so; no number of its tables moved. The unit test of the base strike's launch conditions no longer lists the afternoon as a stop, and lists having launched that day. The test of the MCP description (`mcp.rs`) names the new wording of the base strike mission. Three tests that pinned a hash follow the new version: the Eastern Solomons day 1 hash in `scenario_10_objectives` and the hash version in `batch.rs`.
- **Run baselines and other fixtures.** Every golden file's daily hashes changed and nothing else did, with one new golden file pair for each of 13a and 13b. `crates/sim-cli/tests/batch_before/*.json` changed in `state_version` and each run's `final_hash` only. The recorded games' saves changed in their hashes only; the transcripts and requests are as they were.
- **Not built.** What the decision leaves out stays out. The scenario orders (Ryujo's attack unit sent at Henderson Field, the Japanese carriers' bombers' lists) are not given: they are the architect's, from the fact rows. `docs/validation/` still says `flagplot-state-v4` for its three baseline reports, which belong to the second session.
