# 0229: Men ashore on an island's land are fed by the order for the island

Status: a plan by the architect, written on Nov 24, 2026, inside the plan for step 3, stage 4 (decision 0161), from the measurement that record holds ("The men unfed, as measured" and the architect's reading of it). It is to be read by a reviewer who has not seen it written before anything is built. It has one question for the designer.

Summary: Under an order to control an island the staff feeds the side's men at the island's named places and nowhere else. On the 39 days the Japanese garrison of Lunga is beaten on the first day, falls back one hex to its own side's dump, eats the 71 t there and starves, in every game, with ships and a source to hand: about 1,000 to 1,300 men a game, more than every other loss to hunger together. This makes the men of a side who stand at a base of their side on the island's land the island's to feed, by the supply doctrine as it is, after the named places. One rule of doctrine, and three small changes to what the engine's intake accepts so that the doctrine's intent can stand (amended on review).

Standing: Planned by the architect on Nov 24, 2026, inside decision 0161; amended on review the same day; both questions answered by the designer (the garrison's hunger is judged, and counted where a base of the side stands: decision 0161, condition 13). Its statements and hand calculation are written (scenario 135). To be built before stage 4's proof

## Why

Measured on Nov 24, 2026, on the arrangement stage 4's proof will read and on the same with the table's ground figures, seeds 1 to 100 (decision 0161, "The men unfed, as measured"):

- **Who starves.** Japanese units 11, 12 and 13, the naval guard and the two construction units that begin ashore at Lunga (2,-4), 2,040 men. On day 1 they are beaten and fall back by the engine's rule to the first hex joined by a land link that holds a base of their side: (2,-5), base 6, 71 t.
- **How many.** Of about 1,984 men who fall back, about 1,260 a game on average are lost unfed at (2,-5) on the proof's figures and about 1,010 on the table's. It is more than half of all the Japanese men lost unfed in 98 games of 100 and in 100 of 100.
- **What the staff did.** Nothing. In 200 games no ton was unloaded at (2,-5) and no order of the staff named it, while the same staff fed the men it landed at Lunga to about one in twenty.
- **Why.** "Control an island" forms a hold for each named place, and supply under a hold: for the base held, or, while the place is the enemy's, for the side's men in the place's own hex (decision 0159, A). Guadalcanal has one named place. A base of the side one hex away is in no tree.

A commander who orders an island controlled has not said "and let the garrison beside the airfield starve". The men are his, on the island he named, at a base of his side, and the staff has the source and the ships. The Japanese runs of August and September 1942 landed their men and their supply west and east of the airfield and not on it; a staff that can only feed the airfield's own hex cannot do what was done.

## The rule

1. **The island's land** is the hexes of its named places and every hex joined to one of them by land links, through any number of hexes. It is read from the map, which both sides hold. It is not state and is in no hash.
2. **A base beside a place.** Each morning the island's plan is released, after the takes and the holds are formed (rulebook, "Controlling an island", The tree each morning, orders 1 and 2) and before the denial, the staff looks at each base of the side, in base order, that is on the island's land, in no named place's hex, that the staff may take (decision 0149, item 5), and in whose hex the side has men ashore, of any unit, fighting or working, by whatever way they came there.
3. **It is kept supplied.** For each such base with no "keep supplied" for a base in its hex in force anywhere on the side, the staff forms one, under the island, by `form_intent`: with the floor of the island's risk in days and no interval, as a hold's is formed (10 days at preserve, 5 at balanced, 0 at press; the staff's 15 days), and with a plan of its own, made ready and released by the supply doctrine on the leaves, as the "keep supplied" under a hold of the island has. A "keep supplied" for the hex in another tree counts and is named in the reason, as an intent of another tree is named today.
4. **Its source and its ships are the island's.** Its forces limit is set by the staff as each hold's is set under the island: the island's source of supply and the transport forces stopped there with nothing aboard, each morning that differs; for a side whose staff chooses its sources by decision 0228, by that rule, which is not changed. With no source it forms none and says so, as a hold does. A base that is itself the island's source is not looked at.
5. **After the named places.** It is formed after every hold's supply and is served after them. Under one root a ship serves the first intent that takes it; the supply of a named place takes a ship before the supply of a base beside it. Where the pool cannot feed both, the base beside the place goes short and its "keep supplied" hands in `cannot_meet`, as any does.
6. **It ends with its reason.** On the first morning the side has no man ashore in the hex, or the base is not the side's, the staff withdraws the intent it formed, as it withdraws a neutralising when its base is taken. A run under way is ended by the supply doctrine's own rule for a cancelled intent.
7. **It gives no order to the men.** They stay as they are: on the defence where they fell back, or under whatever order they have. Whether they are marched back, joined to a take or taken off is not this rule's.
8. **Men at a hex with no base of the side** cannot be fed: a unit is fed through a base in its hex, and a unit that falls back to a bare hex forms none. For each hex of the island's land with men of the side ashore and no base of the side, the staff gives no order and records one reason, each morning it is so: the hex, the units and their men, that no base of the side stands there to feed them through, and that it would change with a base there or the men moved. No notice and no interrupt is added for it here.
9. **Only under "control an island".** A "take a place" or a "hold a place" stated alone names one place, and the men it feeds are those of decision 0159, A. Nothing of them changes.
10. **What the staff says.** The reason of the `form_intent` names the base, its hex, the men ashore there by unit, the supply the base holds and its days at today's use, the place of the island it is beside, and that it would change with no man there. The plan order of the island lists the bases so fed beside its places.

## What it reads, and what a side sees

The staff reads its own bases, units and intents from its view, and the map. It reads nothing of the enemy: whether the enemy is in the hex, or holds the place beside it, changes nothing in the rule. So two games that differ only in what the side cannot see give the same intents, which the build's test of that kind is to show.

A side sees its own `IntentFormed`, `PlanDrafted`, staff reasons, orders and notices, whole, as for any "keep supplied". The enemy sees none of it. No new kind of event is added, so `side_events.rs` is not touched.

## Determinism

No draw. No field of the state: the intents formed are intents, which are state already. The hash's layout and version do not move for it. The doctrine's label is raised, since a staff's orders change in any game with an island and a base beside a place; saved games of such scenarios under the old label are refused as they are today, and the recorded games and baselines that hold one are remade.

## Rule card

| | |
| --- | --- |
| Reads | The map's land links and an island's places; the side's bases (hex, supply, reserve), its land units ashore by hex, its intents in force and their plans, the island's source of supply and pool as the staff has set them |
| Writes | By the staff's commands only: `form_intent` of a "keep supplied" under the island, `set_limit` on it, `withdraw_intent` of it. Then the supply doctrine's orders, unchanged |
| Reuses | "Keep supplied" and its doctrine whole; the plan of its own of decision 0151, item 9; the source and pool of decisions 0152, 0163 and 0228; the staff's withdrawal of decision 0151, item 8b; the land links of decision 0094 |
| New | The island's land, a reading of the map. One step in the island's morning. One reason for men at a hex with no base |
| Draws | None |

Why a second way of forming supply under an island and not a hold of the base: a hold puts the men on the defence, covers the base and enters the island's measure of control, and none of those is wanted of a dump the garrison fell back to. Why not add the base to the island's places in the scenario: a place is what the island's control is counted by, for both sides, and the other side would be told to take a hex it has no base on its map for.

## What it changes

- **The 39 days, the Japanese side,** in every arrangement with "control an island": a supply of base 6 is formed on the first morning the garrison stands there, and runs go to (2,-5). The pool that feeds the men landed at Lunga is the same pool. The men landed are fed first (rule 5); whether they are fed as well as before is read by the proof and is not known.
- **The Allied side** wherever its men stand at a base of their side on an island's land outside a named place's hex. On the 39 days as measured it has no such men unfed.
- **Other scenarios with an island** and a base of the side beside a place: scenarios 76 and 77 and those that follow them are checked by the build, and each change is listed in "As built".
- **The rulebook:** chapter 8, "Controlling an island" (the island's land; the tree each morning, a row between 2 and 3; the source and its pool) and "Holding a place" (a pointer from "Men fed before it is taken"); the glossary gains "island's land" and "base beside a place".

## The case and its arithmetic

One scenario, the next free number, with a test whose statements and full hand calculation the architect writes in its doc comment before the build is delegated. The case, on the frozen figures (`daily_supply_t_per_man` 0.005):

- An island of one place, the enemy's base A. One hex from it by a land link, the side's base B with 20 t, and a unit of 1,000 men ashore there from the start. A source S of the side, off the island, with 1,000 t and a transport force stopped there with nothing aboard. The side states "control the island", at balanced, with both leaves, and has no unit to take the place with.
- The men eat 1,000 x 0.005 = 5 t a day. B holds 20 / 5 = 4 days.
- **Without the rule** (the game as it is today, kept as the test's first statement so that the difference is shown): no intent names B; the 20 t are gone after the fourth night; from the fifth night the unit is unfed.
- **With it:** on the morning of day 1 a "keep supplied" for B is formed under the island with a floor of 5 days, its limit the source S and the force; B's 4 days are below the floor, so a run is formed that morning by the supply doctrine's own projection, loads and sails; the doc comment works its load, its day of arrival and B's stock each evening, and states that no man of the unit is lost unfed in the game.
- **It ends:** the unit is ordered to march to another hex by a scenario's order on a stated day; on the first morning no man of the side is at B the staff withdraws the intent, and no run is formed after.
- **A bare hex:** a second unit ashore at a hex of the island's land with no base: no intent, one reason each morning, in the words of rule 8.
- **Not the island's:** a third base of the side with men, on land joined to no named place: nothing is formed.

Properties beside it: more supply at the source never leaves B with less; with no man at B nothing is formed; the side's orders are the same in two games that differ only in the enemy's forces and bases off the side's map; a staff with no intent gives no order.

## How it is judged

1. **The case's test passes** as its hand calculation states.
2. **Nothing changed where it is unused:** every scenario with no island, and every island with no base of the side beside a place holding men, plays as before, by the baselines.
3. **The measurement is made again** after the build, by the same reader, on the same two arrangements and seeds 1 to 100, and set beside the first: for each side the men landed lost unfed over the men landed (condition 3's reading, unchanged), and the men of units ashore at the start lost unfed over those men at the start. The reader, `unfed.py` with its count G in place of F as first written, is brought into the repository under `scripts/` with this build, so that the proof's setup can run it.
4. **Nothing is tuned to it.** If the men landed are fed worse than before, that is reported and goes to the architect; the order of rule 5 is not changed after the fact to move a figure.

## What it leaves out

- **Moving the men:** marching a garrison that fell back to a fed place, joining it to a take, or taking it off. The staff feeds them where they are.
- **A base for men at a bare hex,** and a notice or an interrupt for men no order can feed (rule 8 gives a reason only).
- **Choosing where to land.** The staff still lands at the place it is to take; that its own side holds a base one hex away is not weighed.
- **Men of the side on land that no island names,** and men under a take or a hold stated alone.
- **The stock of base 6,** 71 t, a placeholder of the scenario, which is not moved.

## Building steps

One branch and one pull request, after the review of this plan.

1. The architect writes the scenario's statements and hand calculation in the test file's doc comment.
2. The island's land as a reading of the map, in the staff, with its unit tests.
3. Rules 2 to 7 and 10 in the island's doctrine; rule 8's reason; the doctrine's label raised.
4. The scenario and its test; the baselines and recorded games remade, each change listed.
5. The rulebook and the glossary, in the same pull request.
6. The reader brought into `scripts/`, and the measurement made again and written into this record as "As measured after the build".

## Questions for the designer

1. **Is the garrison's hunger judged by stage 4's proof, or reported?** Condition 3 of decision 0161 is of men landed and stays as written. The men who begin ashore are not in it. The architect recommends a condition of its own, stated now, before this is built and before the proof is run: on the 39 days staff against staff, 1,000 seeds, of the men of a side who begin ashore on an island that side is ordered to control, the median lost to hunger is no more than one in twenty, each side by itself. It can fail. The other course is to report the figure beside condition 3 and judge nothing by it.

## Amended on review, Nov 24, 2026

The plan was read by a reviewer who had not seen it written, against the code, and one seed's events were read for the men lost at (2,-4). The reviewer found the case right (units 11 to 13 fall back to base 6 at (2,-5) by the land link of the map's file) and nine things wrong or unsaid. What follows is the architect's, checked in the code named, and replaces the rules above where they differ.

1. **The engine's intake changes, in three places, and the rules label is raised.** The plan said no change in the engine. That was wrong: a "keep supplied" under "control an island" is refused today (`IntentKind::may_stand_under`, `command.rs`), would share the island's plan and so have no runs of its own (`CommandState::forms_own_plan`), and at press its floor of 0 days is refused, being accepted only under a hold (`command/intake.rs`). The three changes, and no other:
   - a "keep supplied" may stand under "control an island", where its base is the side's; a base of the enemy's or of no one is refused as `CannotStandUnder`, as under a hold;
   - so formed, it has a plan of its own, as a "keep supplied" under an island's hold has;
   - its floor may be 0, as under a hold.

   The forces limit of its runs is its own, which the staff sets by `set_limit` on the morning it forms it and each morning the island's source or pool differs (rule 4); intake's reading of a run's forces already takes an intent's own limit first. Whether the base is on the island's land is the staff's reading and is not checked by intake, which keeps the walk of the land links out of `sim-core`. No state, no event, no draw and no hash layout is added. A command the engine refused is now taken, so the rules label is raised, by one from where decision 0220 leaves it, with the doctrine's label. A scenario test inside `sim-core`'s own tests covers the three, each way.
2. **Rule 5 is withdrawn.** It said the named places are served first. The supply doctrine serves intents by the commander's rank, read up the tree, and then by number (`morning.rs`, `priority_order`), which is the command contract's tie-break: at equal priority, the lower number. Under one island every supply has the island's rank, so the supply of a base beside a place, formed in the first days, is served before the supply of men landed later, and a run of it under way keeps its ships until it is back. That stands. A rule that put a named place first would be a second order of service beside the contract's, written after a figure was seen that it would protect; it is not made. What follows from this is said plainly: the two supplies draw on one pool, the garrison's has the earlier claim, and condition 3 of decision 0161, which stood at 0.0505 on 100 seeds against a mark of 0.05, may be the worse for it. The proof reads it. A commander cannot today rank one place of an island above the land beside it; that is left out.
3. **Men on the march to the base count** (rule 2, widened). The staff looks at a base in whose hex the side has men ashore or to whose hex a unit of the side is marching, both read from the side's own units. A beaten unit walks for `march_days_per_hex` days and is on its old hex's record until it arrives; with this the supply is formed on the first morning after it is beaten and not the morning after it arrives.
4. **The day of the first run.** The supply doctrine is asked before the doctrine that forms the island's tree (`SupplyStaff::decide`, `lib.rs`). An intent formed on a morning is first served the next. The case's "a run is formed that morning" is wrong by a day: the intent is formed on day 1 and its first run on day 2.
5. **When it is withdrawn** (rule 6, replaced). On the first morning that all of these hold: no man of the side is ashore in the hex; no unit of the side is marching to it; and no run of its plan is loaded or at sea. Until then it stays in force, so a run under way is brought home by the supply doctrine as any is, and none is left to be freed half way. With no man in the hex the planned use is nothing, the days of supply have no end, and the doctrine forms no new run, by the rule stage 1 fixed for a base with no consumption. The engine marks a released plan of its own complete on withdrawal, not cancelled (`intake.rs`, the withdrawal); the record's "ended by the supply doctrine's own rule for a cancelled intent" is struck.
6. **Leaving and coming back.** The men lost "at (2,-4)" after falling back are not men who went back (item 9, below), so the measured games have no unit that leaves and returns. One could: a unit ordered to attack across the link that falls back again. Then the supply is withdrawn and, on the morning the unit marches back, formed again with a new number and a new plan. That costs an intent a time and is bounded by one a morning; nothing more is built for it. The test has the case.
7. **"Without the rule" is shown by the base that is not the island's,** since a built game has no switch to play without a rule. The case's third base is made the twin of B: the same 20 t and the same 1,000 men, on land joined to no named place. In one game B's men are fed and the twin's are unfed from the fifth night, and the doc comment works both.
8. **Rule 8's reason covers men on the march.** For a unit marching, the reason names the hex it is recorded in, that it carries nothing and no base of the side stands there, and where it marches to. The measured loss on the march is in item 9.
9. **The men lost at (2,-4)** (decision 0161, the architect's reading, 6) are read, from seed 1 of the proof's arrangement: a beaten unit takes three days to walk to (2,-5) and is on (2,-4)'s record until it arrives on day 4; the side lost its base in the hex when it was beaten, and a unit that falls back carries nothing (chapter 7, "A beaten side's men"). On the proof's ground figures an unfed unit loses one man in twenty a night from the first night, so three nights cost 1,984 x (1 less 0.95 x 0.95 x 0.95) = 283 men, which is the 28,177 over 100 seeds. On the table's figures, where hunger takes time, none is lost on the march. **This rule cannot feed them:** no base stands where they walk. It is the engine's rule for a beaten side (decision 0104) and is not changed here.
10. **What moves, named.** The doctrine's label and the rules label in every file that holds them. By the label alone: scenarios 76, 77, 86, 90 and 94, and any other with an island and no base beside a place. By what the staff does: the three arrangements of stage 3's proof (07a, 07b and 07c where a side states an island), and the staffed games of `five-days`, `coral-sea`, `table-hunger`, `table-boats` and `table-rear-bases` wherever an island's land holds a base of the side with men; each is checked and listed in "As built". The five recorded games under `crates/sim-cli/tests/recorded_games/`, pinned to the doctrine's label, are remade. `export_pictures.rs`, `docs/commands.md` and the rulebook's chapters 1 and 8 name the labels or the refusals and are brought up with them. The comparison "worse than given nothing" of the proofs reads arrangement (c), in which the Japanese side has no order: its garrison is fed by no one there, before and after, so that comparison moves toward the side with orders, and the proof's setup says so.
11. **The case's numbers.** The intent is formed on the morning of day 1 and first served on day 2. The load of the first run, the day it arrives and B's stock each evening come from the supply doctrine's projection, and the day of the march that empties B is chosen to fall after a run has landed and while none is at sea; all are worked in the test's doc comment by the architect before the build is delegated, which is step 1 and is not skipped. The first property is tested by raising the source's stock in a variant of the scenario (the scenario's text with the stock changed, loaded by `common::from_text`) and asserting that B's stock at each day's end is no lower.

## Answered by the designer on Nov 24, 2026

The garrison's hunger is judged. It is condition 13 of decision 0161, written there before this is built.

## A second question for the designer

By item 9, on the proof's ground figures a garrison beaten on the first day loses about one man in seven on its three days' march, before it reaches any base, by the engine's rule and whatever a staff does. Condition 13 as answered counts every man who begins ashore, so on those figures it is expected to miss on the Japanese side with this rule built and working. Three ways to read it, to be chosen before the build:

- **(a)** As answered. The miss is expected and its cause is named in advance.
- **(b)** Counted from where a staff can act: the men lost to hunger in a hex where a base of their side stands, over the men who begin ashore; the men lost on the march are reported beside it. The architect recommends this: the condition is there to say whether the staff feeds the men it can feed.
- **(c)** The proof's arrangement is given the ground figures in which hunger takes time. The architect recommends against it: it changes the figures of every condition of the proof to help one.

## Answered by the designer on Nov 24, 2026: the second question

(b). Condition 13 counts the men lost to hunger in a hex where a base of their side stands. The men lost on the march, with no base where they walk, are counted and reported beside it by name, and judged by nothing. Condition 13 in decision 0161 is restated so, before this is built.

## The statements and the hand calculation, Nov 24, 2026

Written by the architect in `tests/scenario_135_men_beside_a_place.rs`, with the scenario `data/scenarios/men-beside-a-place/135_men_beside_a_place.toml` and its map `data/map/placeholder_beside_a_place.toml`. Thirteen tests, each empty and marked "statement written, not built"; the build fills them. The scenario is in a folder of its own and is read by no loop (decision 0072). It replaces "The case and its arithmetic" above where they differ:

- **The island's place is the side's own,** an anchorage with a garrison of 300, and not an enemy's base. An enemy base one hex from the dump would make the run a run by night (decision 0145), and a take with nothing to take with would add its notices; neither is this rule's, and with the place held the game has no die.
- **The numbers.** The intent is formed on day 1 as intent 3 and first served on day 2. Its run is 90 t (25 t of floor less the 10 t read on the day it arrives, and 75 t for 15 days), loaded on day 2, sailed at hour 48 of the game and landed on day 3, 40 t in the morning and 50 t in the afternoon. The dump holds 95 t that evening and 40 t at the end of the 14 days, and no man beside the place is lost. The twin, off the island's land, is fed for four nights and loses 397 of 1,000 men by the end.
- **The march that ends it** is ordered on day 6 and arrives on day 8; the supply is withdrawn on the morning of day 9. Ordered back on day 9, the unit is marching to the dump on the morning of day 10 and the supply is formed again as a new intent.
- **Checked on the engine as it stands.** The supply doctrine's part (the day, the load, the hours, every stock) was worked by hand and then run in a copy of the scenario in which the dump is itself the island's place, where the same doctrine serves a hold's supply; it gave the same figures, and the same for the two marches. What is the rule's own, the intent's number, its plan, and the mornings it is formed and withdrawn, could be checked on nothing and rests on the statement.

**The versions.** Decision 0220 takes rules label 33, doctrine `supply-60`, interface 55 and state v60. This takes rules label 34 and doctrine `supply-61`. The state's layout does not move. If the plan order's answer to a seat gains a field for the bases fed beside a place, the interface is 56, and the build says so.

**A lead from writing it, not this record's.** A supply formed at press has a floor of 0 days, and the doctrine sends a run when a reading is below the floor's stock; a reading is never below 0. If that is how it plays, a hold at press never sends a run and its men starve when the stock is gone. It is how a hold's supply is formed today (decision 0144) and is not changed here; the test asserts only that the intent is formed. A developer is to play scenario 76's hold at press and say what happens; it goes to `docs/open-work.md`.

