# 0115: Covering a base from the air: the first fighting doctrine

Status: settled by the architect on Nov 15, 2026, inside the plan for step 3, stage 2, that the designer confirmed (decision 0113, decisions 1, 4 and 8, and step 2 of its six). It writes the cover intent and its doctrine for the air out, with the hand calculation. Built on Nov 15, 2026, with decision 0117 ("As built" below).

Summary: The first fighting doctrine of the staff: an intent to cover a base, a forces limit that names air groups, and each morning the missions, the patrol share, the search arc and the rest of the groups it is given, with the risk level setting the share kept overhead and the fatigue at which a group rests

Standing: Settled by the architect, Nov 15, 2026, inside decision 0113. Built

## What it adds to the contract

Amended on Nov 15, 2026, by decision 0116: covering a base from the air is a sub-intent the staff forms under the end "hold a place", and a commander may still state it himself. Addressed to a base, it may use the air groups at that base unless a forces limit says otherwise. The doctrine and the hand calculation below are unchanged; in scenario 47 the commander states it directly.


1. **A kind of intent: cover a base.** It names a base of the side. It has no date: it stands until withdrawn. Its plan is drafted, made ready and released as a supply plan is, by the same leaves.
2. **A forces limit may name air groups.** The staff gives an order to an air group only where an intent's forces limit names it, and intake refuses any other, as it does for ships. A group serves one intent: where two name it, the higher in priority has it.
3. **A commander's order to a group in a forces limit overrides,** and holds the group, as an order to a force does (ruling 23 of decision 0107). A hold on a group, or on its search arc, is honoured as stage 1 built it.
4. **The risk limit is read.** With none set it is balanced.

## The doctrine

Each morning, for each cover intent in priority order, over the groups its forces limit names that are at the base and not held, in group number order. An order is given only where what a group has differs from what the doctrine wants, so a morning on which nothing changed has one reason and no order.

5. **Fighters fly the patrol.** A group whose type is a fighter is set to combat air patrol with a patrol share by the risk level: three quarters at preserve the force, half at balanced, a quarter at press. The rest of its aircraft are its escorts, which stay down unless a raid of its own side goes from the base.
6. **The share is raised by a quarter,** and not above 1, on a morning when the base was attacked from the air on any of the three days before.
7. **Groups that can search, search.** A group that is not a fighter and whose type has a search range is set to search, at its type's range, over the arc of decision 9. A group that is neither is given no order by this doctrine, and its reason says so: it is for a strike intent.
8. **Rest.** A group rests for a day, by the rest mission, when its fatigue in the morning is above the level its risk gives: 30 at preserve the force, 45 at balanced, 60 at press. Fighters are taken most tired first, then by number, and **the last fighter group that can fly is never rested**: the base is not left without a patrol while a fighter can fly. A search group has no such exception. The engine's own rest threshold is left unset.
9. **The search arc** points at what the side knows. The threats are the hexes of the enemy bases on the side's map and of the contacts it holds. With none, the arc is the whole circle. Otherwise it is the narrowest arc on the ten-degree grid that holds every threat, by the engine's own test of an arc, taking the lowest starting bearing where two are equally narrow; widened by 30 degrees at each end; and the whole circle if that comes to 360 or more.
10. **When it cannot.** On a morning when no fighter group of the intent has an aircraft able to fly, the staff hands in the notice that it cannot meet the intent, with its reason, and the standing interrupt of stage 1 fires on it.
11. **Reasons** carry the numbers: each group's fatigue against the level, the share and what raised it, the threats and the arc they give.

## What it leaves for later steps

- **Ships covering a place,** and a hex of sea as the subject (step 4).
- **Bombers,** and fighters sent as escort on purpose (step 3, the strike intent).
- **Part of a search group flown.** The engine sends every aircraft of a search group that can fly in both daylight pulses, and a long search tires a group in a day: at 600 nm a group gains 54 fatigue in a day and loses 10 in a day of rest. So one group cannot search every day, and this doctrine does not pretend it can. Sending a share of a search group is a rule to be weighed in step 3, with the strike of a stated size.
- **Where the enemy is not yet seen.** The arc points at what is known. Searching where nothing is known but something is feared is the area limit's to say, and waits for step 4.

## What it costs

- **State:** the new kind of intent and the air groups of a forces limit, in the hash and named beside it. The state version rises. The rules label does not: no game without the intent plays differently.
- **The staff** gives air group orders for the first time; the doctrine's version rises.
- **The tools:** `set_intent` takes the new kind, `set_limit` takes groups, `propose` and `command_state` show the groups, their missions and the arc with its threats.
- **The rulebook's chapter 8** gains "Covering a base from the air"; the glossary gains the cover intent.

## The hand calculation

`tests/scenario_47_cover_from_the_air.rs`: one base, two fighter groups, a flying boat group and a dive bomber group, an enemy base and a contact that ages out, over 12 days, with no enemy unit that moves or flies, so that no die is rolled. It gives every order of every morning and every group's fatigue.

## The proof of the step

Scenario 6, the air campaign, with the Allied side given one end, "hold Henderson Field" (decision 0117), with no forces limit, both leaves, and no order, against the Japanese side as the scenario scripts it. Over 1,000 seeds, set beside the same scenario with the Allied orders taken out and nothing in their place:

- it passes if the median of Japanese aircraft lost is higher, and the median of Allied aircraft destroyed on the ground is lower, than with no orders;
- every invariant holds and no staff order is refused on any seed;
- the same run beside the scenario's own scripted Allied orders is reported and not judged.

## As built

Built on Nov 15, 2026, with decision 0117, in one step. What follows is what was built and every place the build read the record more closely or differed from it.

**State.** `IntentKind::Cover { base }`, code 2, addressed to the base. A forces limit gains `air_groups` after its task forces, and its `source` becomes optional: a supply intent's forces limit must still name one, and intake refuses it otherwise with a new refusal, "a supply intent's forces limit must name a source base" (`NoSource`); a cover or hold intent's needs none. The risk limit becomes optional, so that one not set is read up the tree (decision 0117, 7) and read as balanced where nothing sets it. A hold may name a whole air group (`HoldTarget::AirGroup`, code 4). The state version is 28 and the rules label 12.

**Intake.** A staff order to an air group is accepted when the intent it serves may use the group: named in the forces limit read up the tree, or, with none on it or above it, based at the base it is addressed to (`World::may_use_group`). Intake does not enforce that a group serves one intent: the staff does, by priority. A commander's order of any kind to a group that some intent in force may use is an override and holds the whole group; it does not look at that intent's plan. The guard refuses every staff order to a group held whole, and an arc or mission order to a group whose arc is held.

**The doctrine** is `crates/sim-staff/src/air.rs`, the doctrine version `supply-8`; the supply doctrine serves keep-supplied intents only and is unchanged. Readings the build took:

- A group held whole or by its arc is left out of the doctrine entirely, so the staff plans without it.
- The raids of item 6 are read from the side's own events by `SideView::air_raids_on`: a pulse in which the base was bombed (the defender's cut `BaseAttacked`). Whether a patrol fought it is a patrol's fire (`AirFight` by the patrol) in the same pulse by a group of the side now based there. The side's events can tell this; nothing of the raiders is read.
- A fighter at or below the level that cannot fly is still set to patrol; only one above the level rests because it cannot fly. The cannot-meet notice is handed in on a morning when no fighter group the intent serves (at the base, not held) can fly, including a morning when it serves none, and rests on that morning's no-order reason.
- A cover root's plan is made ready and released by the leaves as a supply plan is; without leave to launch the no-order reason and the waits-for-leave notice are handed in every morning.
- A contact named as gone in an arc's reason is one the side's own events show dropped since the day of the group's last arc order and no longer held.
- A morning with orders has their reasons and one no-order reason for the groups given none (the dive bomber's "left for a strike intent" is there); a morning with none has one reason.

**What it was checked against.** Scenario 47 (`data/scenarios/47_cover_from_the_air.toml`, placeholder data, run on the frozen figures as scenario 46 is; no frozen copy, being new) came out as the hand calculation has it in every number but one. Every order of the 12 mornings (6, 1, 2, 1, 0, 0, 0, 0, 1, 3, 2 and 1), every morning's fatigue, the end (40, 56, 72, 0), the arcs, the reasons and the variants (preserve, the raid on day 3, no threat, no ready fighter, a held group, an override, the forces limit, the same start, two worlds, `why`) pass. The one that does not, marked ignored: **at press, group 2 is at 50 on the morning of day 12, not 66.** Its mornings from day 9 are 48, 46, 52 and 50. Group 2 has 8 aircraft; above fatigue 50 it can fly 3 of them, and a quarter of 3, cut to a whole aircraft, is none, so it flies no patrol sortie in that pulse ("in each daylight pulse in which that is at least one aircraft", the calculation's own figures) and rests. On day 9 it flies the morning at 48 (4 able, 1 kept, +3) and not the afternoon at 51 (3 able, 0 kept, −5). Group 1 (12 aircraft) is at 66 on day 12 and rests, as the calculation says; the share 0.25 and no rest before day 12 pass. The staff's orders are as the calculation has them; the difference is in the press arithmetic, which goes back to the architect.

**The proof of the step.** Scenario 6 (the live file, live figures) over seeds 1 to 1,000, in release, with the Allied orders taken out:

| Allied side | Japanese aircraft lost, median | of them shot down | Allied aircraft destroyed on the ground, median | Allied aircraft lost, median |
| --- | --- | --- | --- | --- |
| One end, hold Henderson Field, both leaves, no limit, no order | 52 | 32 | 2 | 40 |
| No order and no intent | 25 | 4 | 3 | 3 |
| The scenario's scripted orders (reported, not judged) | 61 | 42 | 2 | 41 |

"Aircraft lost" counts every loss the batch records (shot down, destroyed on the ground, worn out, ditched, lost with a ship). Both conditions hold: 52 is above 25, and 2 is below 3. Every invariant held on every seed (each day is checked and no run failed), and no staff order and no command was refused on any seed (`run --events` on each of the 1,000 seeds, searched for a rejected order or a refused command). Nothing was tuned.

