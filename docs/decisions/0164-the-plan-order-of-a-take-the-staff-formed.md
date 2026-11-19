# 0164: The plan order of a take the staff formed

Status: settled by the architect on Nov 19, 2026. It widens decision 0159, rule E (items 15 to 17), by one kind of intent. It adds no state, no order and no event. To be built.

Summary: The staff hands in a plan order for each intent of the commander's, and the seat's tool refused every intent of the staff's. A commander who says "control the island" therefore could not ask when the men would be ashore: the take is the staff's. A "take a place" the staff formed now has a plan order of its own, worked as a commander's take is, and the tool answers for it

Standing: Settled by the architect, Nov 19, 2026

## Why

The table draws a plan on the chart, and the designer asked for the day of the landing on it. The day is in the plan order of a take (decision 0159, item 15, "the day it expects"). Under "control an island" (decision 0151) the take is formed by the staff with a plan of its own (0151, item 9), so it has phases, things it waits for, men wanted and a day, and none of them could be asked: the order of the island's control gives the tasks under it and no more, and the tool refused the take's number.

The order of the island itself is not the place for the day. An island has several places and may have several takes, each with its own phases and its own day.

## The rule

1. **A take the staff formed has a plan order.** For each "take a place" in force that the staff made, the staff hands in a plan order, worked exactly as the order of a commander's take is worked (decision 0159, items 15 to 17): its mission, its plan and state, its three phases, what it waits for, the men wanted and to hand, the day it expects to be ashore, the need to know, and the tasks under it.
2. **The orders are given in number order of their intents,** the commander's and the staff's takes together.
3. **The seat's tool answers for it** by its number, as it answers for an intent of the commander's. It still refuses any other intent of the staff's, and an intent the side does not have, with the words it used.
4. **Nothing else of the plan order changes.** The order of the commander's own intent is as it was, and still lists the take among its tasks. It is worked when asked and stored nowhere.

## What it leaves for later

- **The other intents of the staff's** (a hold, a supply, a denial, a carrier task): a hold has phases too, and none of them is asked for yet.
- **A day on the order of the island's control,** the latest of its takes: not needed while each take answers for itself.

## Rule card

- **Reads:** what the plan order of a take reads (decision 0159, rule E): the side's command state and the staff's own sums of the morning.
- **Writes:** nothing.
- **Reuses:** the plan order of decision 0159, whole. No new state, order, event, draw or belief.

## What it costs

- **State:** none. No hash, layout version, saved game version or rules label changes. No game plays differently.
- **The agent interface:** the tool `order` answers where it refused; the shape of its answer is unchanged, so the interface version stays. Its description, the schemas' README and the rulebook say which intents it answers for.
- **Earlier scenarios:** scenario 84's property "A seat's tool gives it" is restated: the tool refuses an intent of the staff's that is not a take.
- **The rulebook:** chapter 8, "The plan order" and the list of the seat's tools.

## The hand calculation

It is in `tests/scenario_87_the_order_of_a_staffs_take.rs`, written before the code. It has no scenario file: it reads the order of the staff's take, intent 2, on the mornings of scenario 76's main game, whose days are already worked by hand. The landing is ready on day 2, there are 2 days to embark and 1 of passage, and the men go ashore on day 4: the order says day 4 on the mornings of days 2, 3 and 4, and no day on the morning of day 5, when they are ashore.
