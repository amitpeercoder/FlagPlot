# 0143: Ashore: the attack on the estimate, and a second lift: step 4 of stage 3, first part

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 4 of its nine, as amended). Step 4 is written in two records: this one, what the men of "take a place" do from the first man ashore until the place is taken; and the next, "hold a place" with men, the supply that follows and the check that the place can be held. It adds no rule to the engine. Nothing is built. It is built after decision 0142.

Summary: Step 4 of stage 3, first part: ashore under "take a place" the staff attacks when the men who can attack are enough against its estimate of the enemy for the risk, stands on the defence and rests when they are not, sends the units a first lift left behind, and attacks again when the men are rested and enough

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Not built

## Why

Decision 0142 lands the men on attack and then gives no order until the place is the side's. Against an empty airfield that is enough. Against a garrison it is not: the engine sends a tired unit back to the defence and nobody sends it forward again, nobody stops an attack that the first day shows to be too weak, and the units left at the source stay there. This record gives the staff those three judgements. They are judgements on the side's estimate of the enemy, which may be wrong, and the reasons say what it was.

## The judgement of each morning

It is made on each morning the plan of a "take a place" is released, the place is not taken and a unit chosen for it has men ashore in the place's hex. It replaces item 24 of decision 0142, and item 22 where they differ.

1. **The units of the operation** are the fighting units chosen (the forces limit of the task formed, decision 0142, item 11) and the units a later lift brought (item 9 below), that have men ashore in the place's hex, that the side may use and the commander does not hold.
2. **A unit can attack** if it is on attack already, or its fatigue is at or below the mark for the risk:

| The risk | The mark |
| --- | --- |
| Preserve the force | A third of `attack_fatigue_limit` |
| Balanced | Two thirds of it |
| Press | The limit itself |

Each is cut to a whole number. The two shares are the staff's and are placeholders.
3. **Its fit men** are its men ashore times 100 less its fatigue over 100, cut to a whole man: what the unit's own side knows of who would fire.
4. **The men wanted** are the side's estimate of the enemy's men in the hex times the figure for the risk of decision 0142, item 10: 3, 2 or 1.
5. **Attack.** Where no estimate is held, or the fit men of the units that can attack are at least the men wanted, each unit that can attack and is not on attack is set to attack, with no instruction for when it is spent: the rule's own default stands (it holds where it can be fed). With no estimate the reason says that it attacks with no count of the enemy.
6. **Wait.** Otherwise each unit of the operation that is on attack is set to defend, and no other order is given. The reason gives the estimate with its day, the men wanted, the fit men and how many are short, each unit's fatigue against the mark, and what is coming (item 9).
7. **A unit the rules sent back to the defence, tired,** is under the same judgement: it is sent forward again only when it can attack by item 2 and item 5 says attack.
8. **A unit that lands** is under the judgement from the first morning it has men ashore in the hex. Item 17 of decision 0142 stands for the first lift: its units are set to attack before they sail.

## A second lift

9. **The units left behind** are the fighting units of the intent's forces limit that were not chosen, and that the side may use, the commander does not hold, and whose men are all ashore in the source's hex.
10. **They are sent** while the place is not taken, on a morning the transport force lies stopped at the source with nothing aboard, where no estimate is held or the fit men of the units of the operation that can attack are fewer than the men wanted. The units are taken in the order and by the ships of decision 0142, items 8 and 9, until the men ashore and the men taken are at least the men wanted or no ship is left; with no estimate, every unit the ships can carry.
11. **The lift** is as the first, by items 16 to 19 of decision 0142: supply to a weight on the first morning (the daily charge of the men of this lift for 15 days), a unit a morning by a load order, then the route to the place and back and the unload order, with an escort as before. Two things differ. No unit of a second lift is set to attack before it sails: the judgement of each morning sets it. And the force goes as it stands, with no landing made ready: the reason says what loading it has.
12. **The units of a second lift are kept** as the forces limit of the task "ready a landing" was: the staff adds them to that limit by `set_limit` on the morning the lift is decided, so the choice is made once and the commander may change it.
13. **Nothing more is sent** once the place is taken, or when no unit is left behind. Where the men are short and nothing is left to send, the reason says so, and the staff waits: leaving the place is the commander's (step 8).

## What it keeps

14. **Holds and leaves** as in decision 0142, items 25 and 26. A unit the commander has given a posture is held by that order, as any order of his holds a unit, and is not counted among the units of the operation.
15. **Reasons** carry the numbers in the order of decision 0139's amendment: the estimate and its day, the men wanted, each unit with its men, its fatigue and its fit men, what is short, what is coming and when.

## What it leaves for later

- **A march from a beach to a place inland,** and with it the choice of the beach: stage 4. Decision 0142 named it for this step; it is moved because a beach is a choice between courses, which is stage 4's, and one fixed rule here would be undone there. A place that is not a coast hex is still refused.
- **Where a unit falls back to, and when the staff pulls men out:** the rules of a beaten side and of a spent attacker stand as they are; pulling out is "leave a place", step 8.
- **Holding the place once taken, feeding the men and the check that they can be fed:** the second part of this step.
- **Bombardment and cover before an attack:** steps 5 and 6.

## Rule card

- **Reads:** the side's own land units (men ashore, fatigue, posture, holds), its estimate of the enemy's men in the place's hex with its day, the ground figure `attack_fatigue_limit`, the transport force and the source, the intent's limits.
- **Writes:** nothing of its own. Posture, load, route and unload orders, and `set_limit` on the task the staff formed.
- **Reuses:** every rule of decision 0142; the posture order; the estimate of decision 0092.

## What it costs

- **State:** none. The doctrine version rises; no other version does. A game with no "take a place" plays as before, and scenario 67 gives the orders it gave: no enemy man is ashore in it, so the judgement says attack and every unit is on attack already.
- **The rulebook's chapter 8,** "Taking a place", gains "Ashore".

## The hand calculation

`tests/scenario_68_ashore.rs`.
