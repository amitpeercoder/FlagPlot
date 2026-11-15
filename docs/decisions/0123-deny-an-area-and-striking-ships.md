# 0123: The end "deny an area", and striking ships from a base

Status: settled by the architect on Nov 16, 2026, inside the plan for step 3, stage 2 (decision 0113, the second end and doctrine 5, and step 4 of its seven), written by level as decision 0120 has it. It writes the end and two tasks out with the hand calculation, and says when decision 0118's rule for a strike's size is built. Not built. It is the first half of step 4; surface forces, carriers at sea and fuel as policy are the second half and get their own record.

Summary: The end "deny an area", stated to the area, under which the area's staff gives each of its bases in reach of those waters the task of striking ships and, where the base has no cover, of searching them; the strike doctrine at the base's level, which sets its bombers' standing orders from the risk level; and a strike's size, decided at the group's level from the task it is under

Standing: Settled by the architect, Nov 16, 2026, inside decisions 0113, 0118 and 0120. Not built

## Why

A side that wants the enemy's ships kept out of some waters has no end to state. Its bombers strike ships only where a scenario's order has put them on the strike mission, and then every aircraft goes at the first contact.

## The end: deny an area

1. **A kind of intent: deny an area.** It names a hex and a distance in hexes: the waters within that distance of the hex. It is addressed to the area. It has no date.
2. **Its doctrine, at the area's level.** Each morning, for each such intent whose plan is released, for each base of the side in base number order:
   - where the base has a bomber group whose normal reach touches a hex of the waters, and no strike task from that base is in force under it, it forms one;
   - where the base has a group that can search and whose search range touches a hex of the waters, and no cover of that base is in force, and no search task from that base is in force under it, it forms one.
   Where it forms nothing for any base, it hands in that it cannot meet the intent, with the distances and the reaches.
3. **What it may use:** as "neutralise a base" (decision 0120, 7): with no forces limit, every base of the side for the forming of tasks, and each task the air groups at its base.
4. **Its measure:** the days it has been in force, the contacts the side held in those waters on those days, the strikes launched at them, and the ships the side claims sunk and hit there. It is met on a day on which the side holds no contact in the waters at the day's end. It is belief, and is judged as belief.

## The task: strike ships from a base

5. **A kind of intent: strike ships.** It names a base of the side. It is addressed to the base. It may stand under "deny an area" and under "hold a place", and a commander may state it himself as a root.
6. **It strikes what comes within reach.** The rules of chapter 5 launch a strike at a contact in a group's reach, by its target list. No rule holds a group to an area, and this record adds none: the waters of the end decide which bases are given the task and where the search looks, and a base given the task strikes what its groups can reach.
7. **Its doctrine, at the base's level.** Each morning, over the groups it may use that are at the base and not held, in group number order. A bomber, as decision 0120 has it but with a weapon that can hit a ship, is its to order unless a raid task in force may use it; then the higher in rank of the two has it. For each bomber it sets, where they differ from what the group has:

| Standing order | Preserve the force | Balanced | Press |
| --- | --- | --- | --- |
| Mission | Naval strike | Naval strike | Naval strike |
| The oldest contact it launches on | The figure | The figure | The figure and two pulses more, to 4 at most |
| Target list | Carrier, battleship, cruiser | Every kind | Every kind |
| Reach | Normal | Normal | Extended |
| Share held back | A third | None | None |

   The rows are decision 0113's table. A bomber rests for a day when its fatigue in the morning is above the level its risk gives (30, 45, 60), and is put back on the strike mission when it is not.
8. **Fighters** are the base's cover's, where there is one, and their escorts are what its patrol share leaves. With no cover the task sets them to combat air patrol with a patrol share of a half, since a base that strikes will be struck. Any other group is given no order.

## The size of a strike, at the group's level

9. **The group decides the size of its strike from the task it is under** (decision 0118, "Who decides the size", and the designer's direction of Nov 15, 2026, in `docs/design/command-levels.md`). A group under a task to strike what comes within reach has more to hit than one force: its strikes are sized by doctrine, which sends everything at a kind doctrine gives no figure for. So for each bomber of a strike task the sizing setting of decision 0118 is set on, and its reason says the group decided, from its task.
10. **Decision 0118's rule is built with this record.** Its setting stays an order, so that a commander may override it on a group.

## The task: search waters from a base

11. **A kind of intent: search waters.** It names a base of the side and takes the waters of the end above it. It is addressed to the base. It stands under "deny an area" only.
12. **Its doctrine:** each group it may use that is not a fighter and can search is set to search at its type's range over the arc of decision 0115's rule 9, with the hexes of the waters as the threats; and rests by that decision's rule 8.
13. **Where a cover of the base is in force,** no search task is formed, and the cover's arc takes the hexes of the waters as threats beside those it has: one arc for a base's searchers.

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| Forming strike and search tasks | The deny intent and its plan; the side's bases and their groups; the types' reach and search range; the hexes of the waters | Sub-intents, or the notice |
| The strike doctrine | The task and its limits read up the tree; the groups at the base, their fatigue and standing orders | Orders to the groups; reasons |
| The search doctrine | The task; the waters; the groups that can search, their fatigue | Orders to the groups; reasons |

The one rule of the engine built here is decision 0118's.

## What it leaves out

- **Holding a group to the waters.** Rule 6.
- **Carriers.** A task force as an addressee, with its own patrol, search and strike, is the second half of step 4.
- **A task to destroy one named force,** under which a strike is not sized. It needs a subject that outlasts a contact, and waits for stage 2b, where a superior hands down what bears on an intent.
- **Night.** Interception by night is surface forces', in the second half.

## What it costs

- **State:** three kinds of intent, and decision 0118's setting. The state version rises, and the rules label rises for decision 0118.
- **The staff:** three doctrines, by level; the doctrine's version rises.
- **The rulebook's chapter 8** gains "Denying an area", "Striking ships from a base" and "Searching waters"; chapter 5 gains decision 0118's rule; the glossary gains the waters of an end.

## The hand calculation

`tests/scenario_53_deny_an_area.rs`: one base with three bomber groups, a flying boat group and a fighter group, three contacts in reach and no enemy unit, over 2 days at press, with every order, every launch and its size.
