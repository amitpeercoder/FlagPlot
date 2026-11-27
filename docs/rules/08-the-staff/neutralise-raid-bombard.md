# The staff: Neutralising, raiding and bombarding a base

Part of chapter 8, [The staff](README.md). Neutralising a base; raiding a base; bombarding a base (neutralise.rs, raid.rs, bombard.rs).

## Neutralising a base

The end "neutralise a base" and the doctrine at the area's level that serves it. It is run after the hold doctrine and before the cover doctrine, on the same morning (The day's order, below). It forms sub-intents and gives no order. It draws no random number. [0120 items 5 to 8]

**The end.** A neutralise intent names a **target**: a base on the side's map that is not the side's. It names no base of the side's own. It has no date: it stands until withdrawn. It is given to the area. It is a root, stated by the commander with `set_intent` and the kind `neutralise`, and its plan is drafted, made ready and released by the leaves as a hold intent's plan is (Holding a place, above). `set_intent` refuses a target that is not an enemy base on the side's map as "Not an enemy base"; so does `form_intent`. Under it the staff may form a raid task for the same target (Raiding a base, below), a bombard task (Bombarding a base, below) and, since decision 0202, a carrier task for a free carrier force (Carriers serving an order for an end, below), and nothing else. [0120 items 5 and 9; 0202 item 18]

**What it may use.** An intent given to the area may use no air group without a forces limit (The six limits, above); a neutralise intent is the one exception. With no forces limit on it or above it, it may use every base of the side for the forming of raid tasks, and each raid task may then use the air groups at its base. A forces limit that names air groups narrows it, as any does: a base qualifies (below) only where a group the limit names is, and only the groups it names count there. [0120 item 7]

**Each morning, for each neutralise intent in force whose plan is released,** in priority order, the staff reads each base of the side, in base number order, that the intent may use, with the distance from the base's hex to the target's, in nautical miles by air, and the groups the intent may use at it. A group counts whether it is held or not, as the hold doctrine counts one. A **bomber** here is a group that is not a fighter and whose type carries a weapon; its **reach** is its type's strike radius, the distance it can fly on normal reach. A base **qualifies** if it has a bomber whose reach is at least the distance. Then:

1. **No base qualifies.** Where no carrier force serves it either (Carriers serving an order for an end, below), the staff forms nothing and hands in a decision to give no order whose reason gives, for each base, its distance and the reach of each bomber type it has (or that it has no bomber group it may use), and, where the side has a carrier force, what each was (taken by another order, not free, or with no station), with a `cannot_meet` notice resting on it. It does this every morning on which no base qualifies. The standing interrupt `cannot_meet` fires on it (Interrupts, above).
2. **A base qualifies where a raid stands.** Where a raid task from that base at that target is in force under the intent, whoever made it, the staff forms no other. It hands in a decision to give no order whose reason names the raid that stands. It does this every morning after the first on which the raid has been formed.
3. **A base qualifies and no raid stands.** The staff forms one: a `form_intent` command (parent the neutralise intent, kind raid, the base, the target), with a reason that gives the distance and the bombers that reach. It passes intake as any staff command does. The raid doctrine runs for the new sub-intent on the same morning, on the state as the formation will leave it, so its orders are handed in the same morning and name the new intent.

Where the target is not an enemy base on the side's map when the staff looks (it has been taken, or the picture no longer shows it), the staff forms nothing, hands in a decision to give no order that says so, and no notice. A raid task the staff has formed stands until it is withdrawn with the intent above it: the staff does not withdraw it. [0120 items 6 and 7]

**Its measure.** A neutralise intent is met on a day on which no base of the side was raided from the air. The measure is worked by the agent interface from the side's own view and nothing else, stored nowhere (`sim_agent::neutralise_measure`). It gives:

| Part | What |
| --- | --- |
| Days in force | The days from the day the intent was set to the last day resolved, both counted; or, for an intent withdrawn, to the day before it was withdrawn. 0 where the intent was set after the last day resolved |
| Raids flown | The distinct pairs of a day and a pulse, in the days in force, in which a group of the side launched a base strike at the target (from the side's own `BaseStrikeLaunched` events). Two groups launching in one pulse are one raid |
| Claimed on the ground | The aircraft the side claims to have destroyed on the ground, from the air, at the target's hex, in the days in force (the side's `ClaimReceived` events). It is a claim, as the briefing's are: never below what happened and never corrected. Where the target has changed hands, the hex is the side's own base's |
| Raids on its own bases | The air raids on the side's bases in the days in force, summed over every base of the side (`air_raids_on`). A side cannot tell where a raid on it came from, so they all count |
| Days met | Of the days in force, those on which none of the side's bases was raided from the air |

It is shown in `command_state` (as `measure` on the neutralise intent) and in `report` (as `neutralisations`), with words. It changes nothing and no rule reads it. [0120 item 8]

**A proposal.** `propose` for a neutralise intent runs the air doctrine for the tree of its root as if its plan were released (Holding a place, A proposal). [0120]

A side sees: its own `StaffReason`, `StaffNotice`, `IntentFormed`, the `CommandTaken` of each staff command and the measure, whole. The enemy sees none of it. [0120]

## Raiding a base

The task "raid a base" and the doctrine at the base's level that serves it. It is run last in the staff's morning, after the cover doctrine (The day's order, below). It draws no random number. A raid is launched, escorted, met and resolved by chapter 5 as it stands: this doctrine only sets the orders of the groups. [0120 items 9 to 16]

**The task.** A raid intent names a base of the side and a target (a base on the side's map that is not the side's). It has no date. It is given to the base. The staff forms it under a neutralise intent naming the same target, and a commander may state it himself by `set_intent` with the kind `raid`, when it is a root with a plan of its own, made ready and released by the leaves. Where it is the staff's, it belongs to its root's plan. A raid task is a sub-intent, so its limits narrow as the tree says (The intent tree, above). [0120 item 9]

**The groups it serves.** Each morning, for each raid intent in force whose plan is released, in priority order, the staff reads the groups that:

- the raid may use (The six limits, above): named in every forces limit on its chain, or, with none, the side's groups at its base;
- are at the raid's base;
- are not held as a whole (Holds and the override, above);
- are not taken by a raid served earlier the same morning; and
- are the raid's by rule 16, below.

They are taken in group number order. A group is a **bomber** if it is not a fighter group, its type carries a weapon, and it can reach the target on normal reach (its type's strike radius at least the distance from the base). A group is a **fighter group** and has a **fatigue** and a number of aircraft **able to fly**, as the cover doctrine reads them (Covering a base from the air, above). [0120 item 10]

**The risk level** is the raid's risk limit read up the tree, balanced where none is set. It gives five figures:

| Set by the risk level | Preserve the force | Balanced | Press |
| --- | --- | --- | --- |
| A group rests above a fatigue of | 30 | 45 | 60 |
| Share of its bombers a group holds back | A third | None | None |
| The least a raid sends | 12 | 6 | 1 |
| A raid without an escort | Does not go | Goes | Goes |
| A group stands down after losing | A tenth of what it sent | A quarter | Never |

The fatigue levels are the cover doctrine's. The first two rows are decision 0113's; the last three are placeholders that rest on no row. [0120 item 14 and the table]

1. **Which bombers go.** A bomber group goes unless its fatigue in the morning is above the level its risk gives (a fatigue equal to it goes), or it stands down by rule 3. [0120 item 11]
2. **Whether the raid goes.** What a group can send is the aircraft it is able to fly less its held share, cut to a whole aircraft: the held share is the one the risk gives this morning, whatever the group has set now. The raid goes on a morning when the bombers that go can send, between them, at least the least weight the risk asks, and more than none, and, at preserve the force, when the escorts the fighters at the base can give this morning are above 0. A fighter the raid has gives every aircraft it is able to fly; a fighter of the base's cover gives what the patrol share the cover wants this morning leaves of the aircraft it is able to fly, cut to a whole aircraft, and none where the cover rests it. At balanced and press the raid goes without escort. Where the raid does not go, every bomber rests. [0120 item 12]
3. **What the last raid cost.** A bomber group stands down for a morning when its last base strike launch was on the day before, and the aircraft it lost in the pulse of that launch, shot down or ditched (the side's own `AircraftLost` and `AircraftDitched` events of that pulse), come to at least the share of the aircraft it sent that the risk gives (a tenth, a quarter). A group that lost less, or that flew its last raid on any earlier day, does not stand down; at press none does. A group stands down on the morning right after that raid only. [0120 item 13]
4. **The orders.** A bomber is set to the strike mission at the target (`BaseStrike`) if it goes and the raid goes, and to rest otherwise. Its held share is set to the share the risk gives, if the group has another. A fighter group the raid has is set to the combat air patrol mission with a patrol share of 0, so that every aircraft it can fly is an escort; it is given no rest order and flies only on a morning a raid goes. A fighter group of the base's cover is given no order. Any other group is given no order, and the reason says so. An order is given only where the group's mission or setting differs from what is wanted, so a morning on which nothing changed has no order for a group. Orders are handed in in group number order, a mission before a setting. [0120 item 10; 0114]
5. **When it cannot.** On a morning when no bomber group the raid may use and is not held can reach the target, or, at preserve the force, the base has no fighter group it may use or draw escorts from (a fighter of the cover counts, though the cover rests it), the staff hands in a decision to give no order whose reason says it cannot meet the intent and why, with a `cannot_meet` notice resting on it. A raid that is not sent for want of weight, fatigue or escort is no failure: its bombers rest and the reason says why. [0120 item 15]
6. **Reasons.** An order's reason says the group, its kind, its fatigue against the level, what the group has and what is wanted, the weight the bombers that go can send against what the risk asks, and the escorts the fighters can give. A decision to give no order is handed in on a morning on which a group got no order, or the raid cannot be met, or the raid has no group at the base that the staff may order: it lists those groups and says whether the raid goes. [0120]

**Whose a group is (rule 16).** A cover and a raid task may both be in force at one base and both may use a group. A group has one owner, decided by its kind and not by the rank of the intents, so that a base both defended and raiding does both:

- a fighter group is the cover's where a cover of its base that may use it is in force, and the raid draws the escorts the cover's patrol share leaves (rule 2); with no such cover it is the raid's and the raid sets it (rule 4);
- a group that can bomb the target is the raid's, though it can also search: the cover leaves it out (Covering a base from the air, A raid's bomber). A bomber a strike-ships task in force may use is that task's where the task ranks above the raid, and the raid leaves it out (Striking ships from a base, above);
- any other group is the cover's if a cover that may use it is in force (it may search), and otherwise no one's;
- rank decides only between two tasks of the same kind that may use one group: a group serves one cover, and one raid, the first in priority order.

[0120 item 16, amending 0115, 7]

A side sees: its own orders, their acceptance or refusal, reasons and notices, whole. The enemy sees none of it. [0120]

## Bombarding a base

The task "bombard a base" and the doctrine at the task force's level that serves it. It runs last in the staff's morning (The day's order, below), after "take a place", whose landing force's due day it reads. It gives orders to a task force and draws no random number. The engine's half is the shore bombardment (chapter 4, Shore bombardment) and the halt at a marked waypoint (chapter 2). [0146]

**The task** (`bombard`, code 14) names a task force of the side and a base on the side's map that is not the side's, the target; intake refuses a base that is not (`not_an_enemy_base`) and a force not the side's. It has no date. It is addressed to the force. A commander may state it as a root; it then needs a plan of its own, made ready and released by the leaves. [0146 item 1]

**Forming.** Under an end whose plan is released, for each task force on the bombardment mission with a ship afloat that the nearest forces limit on the end's chain names, in task force order, where no bombard task for that force is in force anywhere on the side (a force before its day is left out): under **neutralise a base**, at that base; under **take a place**, at the enemy's lowest-numbered base on the side's map in the place's hex, and none where there is none (intake refuses one whose target is not in the place's hex, `cannot_stand_under`). Where one stands, a decision to give no order names it. A force on the bombardment mission is given no stand task. Under "control an island" a neutralising formed under the island takes no force by this rule: the bombardment forces are the island's (Controlling an island, The bombardment forces). [0146 items 2 and 3; 0151 item 8c]

**The force's morning.** Each morning, for each bombard task in force whose plan is released, in priority order, the staff decides at the force's level from the side's view, as the stand doctrine does (Standing at a hex by night, above), with these differences:

1. **The force** gone, before its day, held by the commander, or with no ship afloat: one reason and no order, and for a force gone or with no ship afloat the notice that the intent cannot be met.
2. **The target.** Where the target is not a base of the enemy's on the side's map (it is the side's own now, or gone from the map), the force is given no route, is set to normal readiness where it is not, and the reason says so. No notice is handed in. [0146 item 4]
3. **Radio silence, the home port, away from home and the rules of engagement** are the stand's: silence at preserve the force where reports travel; the base it lies at stopped as its home port where it has none; no order away from its home port; the stance engage and the break-off level of the risk, set once. [0146 items 8 and 9]
4. **Whether it sorties tonight.** As a root or under neutralise: at press and at balanced every night; at preserve the force every night on which the force's own picture holds no contact of ships within 15 hexes of the target that reports more ships than the force has afloat. Under take a place: on the morning of the day the landing force is due at the place, and on no other. That day is, on the morning the landing force is routed (an order of the same morning), that day plus the days of the passage at the cruise speed of its slowest class less one (decision 0142, item 18); while it is under way to the place, the day the engine's forecast brings it there; before it is routed it is not known and the force waits; once it has come to the place there is none. [0146 item 5]
5. **The speed and the route.** From its home port, routed to the target's hex and home, the target's hex marked to act at; at cruise, then full speed, the first that brings it there within the night's 12 hours, ordered where it differs from the force's. Where neither does, there is no sortie and the notice that the intent cannot be met is handed in. [0146 item 6]
6. **Fuel and ammunition first,** by the stand doctrine's rule (the passage out and home, by the engine's measure, times the risk's margin) and decision 0127 (Ammunition as policy, below): a force short of either does not sortie, and its reason gives the day from which it may be used, or none and the notice that the intent cannot be met, on each morning it holds. [0146 item 7]
7. **Readiness:** battle on a morning it sorties, normal on one it does not. The orders are in the stand's order: silence, home port, rules of engagement, readiness, speed, route. [0146 items 8 and 9]
8. **It fires and comes home by the rules as they are:** the force halts in the target's hex, fires in the night's bombardment phase, and steams on at the start of the morning pulse. [0146 item 10]
9. **Reasons** give the target and its distance, the hours to it, the fuel aboard against the fuel needed, each ship's rounds against the level of the risk, and, at preserve the force, the contact that stopped it. [0146 item 11]

A side sees: its own `IntentFormed`, staff reasons, orders and notices, whole. The enemy sees none of it.
