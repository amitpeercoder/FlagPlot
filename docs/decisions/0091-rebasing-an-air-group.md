# 0091: Rebasing an air group by order

Status: confirmed as to the need. The designer asked on Nov 13, 2026 for rebasing to be built before the fight ashore (decision 0090), so that a player can fly aircraft out of a base that is about to fall. The rule's details are the architect's, of the same date. Built on Nov 14, 2026 (scenario 22). It has no figure of its own.

## The gap

An air group stays at the base its scenario gives it (decision 0052, item 7). When a base can fall, its groups would be lost with it and the player could do nothing about it.

## Decisions

1. **A rebase order names an air group and a base.** `rebase`, with `air_group` and `base`. It is an order for one day's move, not a standing order: once done it is spent.
2. **It is refused at intake** if the group is not the side's own; if the group is lost, aboard a ship or away at the yard; if the base is not one of the side's own; if the group is already at that base; or if the base is beyond the type's transfer range. Each has its own reason.
3. **The transfer range** is twice the longer of the type's search range and strike range: the distance it flies out and back, flown one way. The distance is between the two bases' hexes. No new figure.
4. **The move is made when the order is accepted.** The group's place becomes the new base. Its ready aircraft and all its pilots go with it. Nothing is rolled: no wear, no ditching, no supply for the flight, no fatigue.
5. **Damaged aircraft cannot fly and are written off** at the old base. An event records them. They count as aircraft lost for good, as aircraft lost to wear do.
6. **The group flies nothing on the day it moves:** no search, strike, escort or patrol. Its aircraft are on the ground at the new base for any raid on it that day. Its mission, target list and rest threshold stand, and it flies them from the new base from the next day.
7. **A group that was stood down stays stood down,** and its fatigue is unchanged.
8. **What follows the group and what does not.** Aircraft the scenario delivers to the group by name follow it (decision 0041). A replacement stream belongs to a base and serves the groups there (decision 0038): the group leaves its old base's stream and comes under the new base's, if there is one. From that night the new base pays its daily supply.
9. **A side sees its own group rebased and its aircraft written off, whole. The enemy sees nothing.**

## A small worked case

Base 1 at (0,0) and base 2 at (5,0), both Allied, 5 hexes or 200 nm apart. Group 1, of the placeholder flying boat (search range 600 nm, so transfer range 1,200 nm), is at base 1 with 6 ready and 2 damaged, on a search mission.

| Day | What happens | Group 1 |
| --- | --- | --- |
| 1 | It flies its search from (0,0) | At base 1: 6 ready, 2 damaged |
| 2 | The rebase order is accepted at the start of the day. 2 damaged aircraft are written off | At base 2: 6 ready, 0 damaged. It flies nothing |
| 3 | It flies its search from (5,0) | At base 2: 6 ready |

Group 2, of the placeholder fighter (strike range 250 nm, so transfer range 500 nm), is at base 1. Base 3, Allied, is 13 hexes or 520 nm from base 1. An order to rebase group 2 to base 3 is refused as beyond its transfer range. An order to rebase it to base 1 is refused as already there.

The hand calculation is in `tests/scenario_22_rebase.rs`.

## Why

- **The cost is the day.** A group that moves does not search, strike or defend that day, and it leaves its damaged aircraft. That is enough to make the move a decision without a new figure or a new roll.
- **Made at acceptance, as a yard order is,** so the briefing of that day shows the group where it will be.
- **One-way range from the figures a type has.** A type has no ferry range, and no source row gives one yet.

## What it leaves out

- **A group aboard a ship.** Flying a group ashore from a carrier, or aboard from a base, stays deferred.
- **Part of a group.** The whole group moves.
- **A field's capacity.** A base takes any number of aircraft.
- **Staging through a base between** to reach one beyond the transfer range in one day. It can be done over two days by two orders.
- **Loss on the flight,** and an enemy patrol meeting it.

## What it costs

- **New state:** the day a group last moved, which stops it flying that day. It goes into the hash and is named beside it; the hash version is raised and the golden files remade (decision 0057).
- **A new order, five refusal reasons and two events,** each event with its row in `side_events.rs`, and the agent interface's order list, briefing and queries.
- **The rulebook:** chapter 5 gains rebasing, the order of the limits on what flies gains the day of the move, and chapter 1 gains the order and its refusals, with the glossary, in the same pull request as the code. Decision 0052, item 7, no longer lists rebasing.

## Rule card

| | |
| --- | --- |
| Reads | The air group's side, place, aircraft and type; the type's search and strike range; the base's side and hex; the day |
| Writes | The air group's place, its damaged aircraft and the day it moved; the two events |

## As built

Built on Nov 14, 2026, to the sections above and the architect's hand calculation in `tests/scenario_22_rebase.rs`. Nothing above this section was changed except the status line. Every choice the build made that the record did not is listed here.

- **The distance** is the air distance between the two bases' hexes, `Hex::air_distance_nm` (decision 0040): along an axis, 40 nm a hex. A distance is within the transfer range if it is less than or equal to it. The range is `RefData::transfer_range_nm`.
- **The refusals** reuse three existing reasons (not your unit, air group lost, not your base) and add five (aboard a ship, away at the yard, already moved today, already at the base, beyond the transfer range). They are tried in this order: not your unit, air group lost, away at the yard, aboard a ship, already moved today, not your base, already at the base, beyond the transfer range. A group aboard a ship that is at the yard is told it is away at the yard. The beyond-range reason carries the distance and the range.
- **A second rebase order on the day of a move is refused** (already moved today). Staging through a base therefore takes two days, as "What it leaves out" says.
- **The events.** `AircraftWrittenOff` (group, base left, aircraft) comes first, only if the group had damaged aircraft; `AirGroupRebased` (group, from, to, distance, transfer range, ready aircraft, pilots) follows. Both cite the order and come right after its `OrderAccepted`. `AirGroupRebased` carries a breakdown of the search range, strike range, transfer range and distance when breakdowns are on.
- **The day of the move** is a limit in `aircraft_able_to_fly`, right after the base out of supply: a group that moved to another base by order that day flies none. It is limit 2 of "The order of the limits" in chapter 5. The normal fatigue rules for a group that did not fly apply that day.
- **Aircraft written off** count for `aircraft_destroyed` as aircraft lost for good, as aircraft written off by wear do, and `sim-cli batch` counts them in `worn_out` of `air_losses`.
- **The agent interface.** The briefing's air group line and `get_entity` gain `moved_yesterday` (true the day after a move, when the group flew nothing). The briefing's losses gain `air_groups_rebased`, and `AircraftWrittenOff` is counted in the briefing's `written_off`. `measure`'s `air_reach` answer gains `transfer_range_nm`. The order is `rebase` with `air_group` and `base`, in a scenario file as `kind = "rebase"`.
- **The new state** is an air group's `rebased_day`, the day it last moved, written in the hash after `last_base_strike_day`. The state version is 11 and the golden files are remade.
- **A base formed by a landing** is a base of the side (decision 0066) and may be named by a rebase order.
- **The rulebook.** Chapter 5 has the section "Rebasing an air group" and the new limit; chapter 1 has the order, its refusals and the events' visibility; chapter 6 counts the written-off aircraft; the glossary has "rebase" and "transfer range".
