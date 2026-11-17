# 0151: Controlling an island: step 8 of stage 3, second part

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 8 of its nine as amended, and the designer's first answer: control of an island means its named places held and their garrisons supplied above their reserve). It writes the area intent stated alone. It adds no rule to the engine. Nothing is built. It is built after decision 0150.

Summary: Step 8 of stage 3, second part: the end "control an island", stated alone, from which the staff draws the tree: "take a place" and "hold a place" for each named place that is not the side's, "hold a place" for each that is, the supply under each hold from a source it chooses, a denial of the island's waters and the neutralising of each enemy base near it; with its measure, the days on which every named place was the side's and supplied above its reserve

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Not built

## Why

The spec's play is one statement: control this island (`docs/step3-spec.md`, "The play it must support"). Every end it implies is built by now: take, hold, keep supplied, deny, neutralise. This record lets a commander state the one and have the staff state the rest, as decision 0116 meant a tree to work. He may still state any of them himself.

## An island

1. **An island is data.** A scenario names it: `[[island]]`, with a number, a name and its places in order, each a base or a coast hex. It is on both sides' maps, as the map is. The loader refuses an island with no place, a place off the map, and a number used twice.
2. **It is not state:** nothing of it changes in a game, and it is not in the hash.

## The end

3. **Control an island** names an island. It has no date. It is an end, addressed to the area, and a root. Its plan is drafted, prepared and released by the leaves, as a hold's is, and stands until the intent is withdrawn.
4. **It needs no forces limit.** With one, the staff's choices are inside it, kind by kind, as decision 0149 has it.
5. **Inside one tree nothing is kept from a sibling.** The rule of what the staff may take (decision 0149, item 5) keeps a unit, a force or a base from an intent of another root that is higher in priority. Under one root the intents share: a unit or a force serves the first intent, in the order of item 6, that takes it that morning, and a ship serves the run or the lift that has it until it is back.

## The tree

Each morning the plan is released, the staff forms what is missing, by `form_intent` under the intent, in this order. An intent of the kind and place already in force anywhere on the side, whoever made it, counts, and none is formed beside it.

6. **In the order of the island's places:**
   - **Take a place,** for each named place where the side holds no base, and no "take a place" of it is in force with a plan that has not ended.
   - **Hold a place,** for each named place with a base on the side's map: of the side's base there, or, where the side has none, of the enemy's lowest-numbered base there, which waits until it is taken (decision 0144). A named place that is a bare hex has its hold formed on the first morning the side has a base there.
   - **Deny the waters** within 3 hexes of the island's first place, once. The 3 hexes are a placeholder.
   - **Neutralise a base,** for each enemy base on the side's map that is in no named place's hex and within 15 hexes of one, in base order. The 15 hexes are the placeholder of decision 0124.
7. **Their limits.** The staff sets no risk: each reads the island intent's. It sets a forces limit only on the holds, by item 8. The takes choose their forces by decision 0149; the denial and the neutralising use what their doctrines find (decisions 0120 and 0123) and say so where they find nothing.
8. **The source of supply** for the holds is the commander's, where his forces limit names one. Else it is the side's base in no named place's hex, that the staff may take, at which a transport force it may take lies stopped, with the most supply above its reserve; then the lower number. The staff sets each hold's forces limit to that source and the transport forces stopped there. With no such base the holds form no supply and say so.
9. **What follows is the built doctrines':** each take is prepared, launched on leave and landed; each hold puts its men on the defence, covers its base and forms its supply; the check that a place can be held is made before a take is launched (decision 0144).

## Its measure

10. **Controlled on a day:** at the end of the day every named place holds a base of the side, and in each the side's supply is above the reserves of its bases there. Worked from the side's own view and events, stored nowhere, as a hold's measure is (decision 0117, item 14).
11. **It gives** the days in force, the days controlled, and for each place whether it is the side's and its days of supply today. It is shown in `command_state` and in `report`, in the words "controlled on N of M days in force".
12. **The intent is never marked complete.** It stands, and what it formed stands, until the commander withdraws it.

## What it leaves for later

- **An island with a place inland,** which waits on the march from a beach (stage 4).
- **The order the places are taken in,** chosen by the staff: they are taken in the island's order, each as its forces come free.
- **Garrisons sized and moved between places,** and defences built under a hold.
- **Leaving formed by the staff** when a place cannot be held.

## Rule card

- **Reads:** the island's places; the side's own bases, forces and units; its map of the enemy's bases; every intent in force and its plan.
- **Writes:** one kind of intent with its island, in the command state; nothing else.
- **Reuses:** `form_intent` and `set_limit`; the ends of decisions 0117, 0120, 0123, 0142, 0144 and 0149 whole.

## What it costs

- **State:** one kind of intent, in the hash and named. The state layout, saved game, agent interface and doctrine versions rise. The rules label does not.
- **Data:** the islands of each scenario that states the intent. The staff's setup of the 39 days names Guadalcanal's places from the rows the slice already rests on.
- **The rulebook's chapter 8** gains "Controlling an island".

## The hand calculation

`tests/scenario_76_controlling_an_island.rs`.
