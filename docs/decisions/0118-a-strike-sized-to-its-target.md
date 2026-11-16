# 0118: A strike sized to its target

Status: settled by the architect on Nov 15, 2026, inside the plan for step 3, stage 2 (decision 0113, decision 13), on the designer's answer to its question 3: "a strike group sends the number of aircraft and ammunition as it sees fit to accomplish the task at hand". It writes the rule out with its rule card and its hand calculation. Built on Nov 16, 2026, with decision 0123 ("As built" below). Amended on Nov 15, 2026, after the research on strike sizes and the designer's second word on it: see "Who decides the size".

Summary: The size of a strike is the launching group's to decide, from the order it was given, what the operation requires and its doctrine; sending everything is one answer. The rule: a standing setting of a naval strike group, off until set, under which it sends at a contact what doctrine says that kind of target needs, less what groups before it have sent there, and everything where doctrine gives no figure

Standing: Settled by the architect, Nov 15, 2026, inside decision 0113. Built

## Why

Every aircraft of a strike group that can fly goes at the first contact on its list, whatever was reported there. Six groups in reach of one destroyer and three transports all go at one of them. The designer's answer is that a group sends what the task needs and nobody sets a number.

A strike is launched inside the day, on a contact of that pulse, so its size is decided then, by a rule, from what the report says. A report that is wrong sends the wrong size, which is the fog the third pillar asks for.

The rule reuses the strike decision as it stands: the same contacts, the same order of a group's target list, the same count of aircraft able to fly and the same share held back (decision 0114). A game in which the setting is given to no group is the game it was, so the battles tuned in step 2, whose scripts sent everything, are unchanged.

## Who decides the size

Added on Nov 15, 2026. The research this record asked for came in the same day (`docs/research/oob/strike-sizes.md`): twenty strikes of 1942 with the force reported before launch and the aircraft sent. At carriers reported, a carrier sent a whole group or all that could fly: 29 to 50 attack aircraft at two carriers. What was held was held for a stated reason: 17 scout bombers kept by Yorktown at Midway for carriers not yet found, 8 kept by Lexington against torpedo planes. Against lesser targets the numbers are smaller and uneven: 34 at two cruisers and two destroyers, 8 at four destroyers. No rule of the time gives aircraft for each ship.

The architect read that as against this record and put it to the designer, who first chose to withdraw it and then, the same day, restored it with the reading it should have had from the start. His answer was that a group sends what the target needs. That does not rule out sending everything. The size is decided where the strike is launched, from three things:

- **The order it was given.** A commander's order to a group, or the task the staff formed, may say to strike with everything or to size the strike.
- **What the operation requires.** Aircraft kept for an enemy not yet found, or for the group's own defence: the held share (decision 0114), which the staff sets from the risk level.
- **Doctrine.** What a target of each kind needs. Against a carrier, the doctrine the record shows is everything.

So the rule below stands, with one change: doctrine may give no figure for a kind, and then the target needs everything.

## The rule

1. **A setting of a group: strike sizing,** off until an order sets it (`set_strike_sizing`, on or off). Only the naval strike decision reads it.
2. **The need of a contact** is the ships it reports times the figure for the kind it reports: `air_combat.toml: [strike_aircraft_per_ship]`, one whole number for each of the six kinds. A kind with no figure needs everything: a sized group sends all it can at such a contact, and the contact is never served. With no table in the figures at all, every kind is so, and sizing does nothing.
3. **What has been sent at a contact** is the aircraft launched at it in this pulse by the side's groups before this one in group number order, sized or not.
4. **A sized group walks the contacts it could launch on,** in the order it chooses among them today (the kind first on its target list, then the nearest), and takes the first whose need is above what has been sent. It launches at that contact the smaller of what it can send, which is the aircraft it is able to fly less its held share, and what is still needed.
5. **Where every such contact is served, it does not launch** and records nothing.
6. **Aircraft not sent stay on the ground.** They are not charged for a sortie and may fly in the next pulse.
7. **A group with sizing off** launches as it does today, at the first contact, with all it can send.
8. **Escorts are unchanged:** all the escorts of a place go with its raid of the most bombers.
9. **A base strike is not sized.** It sends all the group can, less its held share.
10. **The launch's breakdown** adds the need of the contact, what had been sent, and what the group sent. The launch is caused, besides what causes it today, by the group's sizing order.
11. **The strike question** (`measure`'s `choices`) answers with the contact and the aircraft this rule gives.

## The figures

| Kind reported | Aircraft for each ship | What it leans on |
| --- | --- | --- |
| Carrier | no figure: everything | Every carrier strike at carriers reported, Coral Sea to Santa Cruz |
| Battleship | no figure: everything | No row. Taken with the carrier as a capital ship |
| Cruiser | 9 | 34 at two cruisers and two destroyers, Midway, 6 June |
| Destroyer | 3 | 8 at four destroyers, 25 August; a Japanese figure of 3 for a screening destroyer, in a compilation of 1945 that is not dated to 1942 |
| Transport | 3 | 25 at a convoy of 12 transports and its escort, 14 November |
| Auxiliary | 3 | No row |

The carrier's entry rests on the record. The others are placeholders, each leaning on one strike or on none, and a strike in the record was often simply all that was on hand. One table serves both sides until the Japanese side's own doctrine is found.

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| A strike sized to its target | The group's sizing setting; the side's contacts, their ships and kinds; the figure for each kind; the aircraft the group is able to fly and its held share; the strikes already launched in the pulse | The contact a group launches at and the aircraft of the strike; the launch's breakdown |

## What the staff does with it

In step 3 the strike doctrine decides, for the groups of a strike task, whether a strike is sized: on where the task has more to hit than one force, so that the groups are spread over what is found, and off where the task is one force to be hit with everything. It sets their held share from the risk level, as decision 0113's table has it. Neither is set by a player, though a player's order to a group may set either, as an override.

## What it leaves out

- **Ammunition by target.** A strike's weapon is its type's. A choice of bomb or torpedo by target is not in the game.
- **A minimum worth sending.** A group that can send 2 where 24 are needed sends 2. Whether to send at all is the standing order's to say, by the target list and the contact's age.
- **Fighters sized to the target's patrol.** The escorts go as they do.
- **A base strike sized** to what is believed at the base. It waits for "neutralise a base", in the same step, which may ask for it.
- **Two reports of one force.** Two contacts are two needs. The merging of reports is chapter 3's, and nothing here adds to it.

## What it costs

- **State:** one setting of a group, in the hash and named beside it; the state version rises. The rules label rises.
- **An order** added at the end of the order kinds, and its event.
- **The rulebook's chapter 5** gains "A strike sized to its target" under the strike decision, and its list of air group orders the new order; the glossary gains the need of a contact.

## The hand calculation

`tests/scenario_49_a_strike_sized_to_its_target.rs`: eight groups at one base and three contacts in reach, a carrier, two cruisers and three transports, with no enemy unit, so that every strike finds nothing and no die is rolled. It gives every launch and its breakdown.

## As built

Built on Nov 16, 2026, with decision 0123, in one step. What follows is what was built and every place the build read the record more closely or differed from it.

**State and order.** An air group has `strike_sizing` (off until set) and `strike_sizing_order`, hashed last in the group after `strike_share_order`. The order is `AirGroupOrder::SetStrikeSizing { on }`, added at the end of the air group orders (`set_strike_sizing` with `on` in scenario files and the agent's orders); it is accepted for any group of the side, with or without the air combat figures. There is no scenario key for it. The state version is 30 (`flagplot-state-v30`, with decision 0123's kinds) and the rules label 13.

**The figures.** `air_combat.toml: [strike_aircraft_per_ship]` is an optional table of whole numbers by kind; a kind left out has no figure, and a file without the table is every kind so (`AirCombatFigures::strike_aircraft_per_ship`, empty). The live file has cruiser 9, destroyer 3, transport 3, auxiliary 3 and no carrier or battleship. **The frozen step 1 figures have the table empty**, as this record says ("get no table"); the test that a frozen file has every key of its live counterpart (`crates/sim-data/tests/frozen_figures.rs`) names this table as one a record leaves out, with its reason. A hand calculation adds its own figures to its copy.

**The rule** is `naval_strike_launch` in `crates/sim-core/src/airstrike.rs`, the one function the strike decision and the side's view call. The candidates are the contacts the group could launch on, in its order (`contacts_to_strike`: kind first on the list, then the nearest, then the contact number; it replaces `contact_to_strike`). What has been sent at a contact is read from the strikes in flight of the side aimed at it (`sent_at_contacts`): those launched earlier in the pulse, sized or not. Sizing off takes the first candidate with all the group can send; sizing on walks them as rule 4 has it, and a kind with no figure takes all the group can send and is never served. The need is the ships reported times the figure, saturating.

**The breakdown.** A sized launch's factors are `aircraft_able_to_fly`, `share_held_back`, `need_of_contact` (only where the kind has a figure), `sent_before`, and the result the aircraft sent. An unsized launch's breakdown is as decision 0114 built it. The sizing order is cited after the share order, wherever the group has one (on or off).

**The strike question.** `AirAnswer::Strike` now carries a `NavalStrike` (the contact, the aircraft, and how a sized strike's size was found). `SideView::naval_strike` answers it as the decision will be made: the side's own groups before the one asked, in group number order, on the naval strike mission with ready aircraft, a position and a weapon, each launch on the same function as if the pulse were daylight (darkness is the same everywhere today), then the group asked. `SideView::strike_choice`, which the briefing's `would_strike` reads, is the contact of that answer; for a group with sizing off it is what it was. The agent's strike choice line gains `aircraft`, and for a sized group `need` (null: everything) and `sent_before`; the agent interface is version 10.

**What it was checked against.** Scenario 49 (`data/scenarios/49_a_strike_sized_to_its_target.toml`, placeholder data, run on the frozen figures with the calculation's table added; sizing set by orders on day 1, the age and the held share by the scenario keys) came out as the hand calculation has it in every number: the morning's launches (10, 10, 4, 9, 10, 5, 18 and none), every breakdown, the causes, the three raids of 24, 24 and 18, the drops by groups 1, 4 and 7, 66 aircraft, and every property (89, 89, 89, 78, 72, 82, too few, a base strike of 9, sorties charged 4 and none, the strike question 4, 18 and none, the refusal). One reading: the variants' totals (89, 78, 72, 82) are the morning's, as "in one raid" says; over the whole day the variants with contacts left unserved launch again in the afternoon.
