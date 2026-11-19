# 0164: The plan order of a take the staff formed

Status: settled by the architect on Nov 19, 2026. It widens decision 0159, rule E (items 15 to 17), by one kind of intent. It adds no state, no order and no event. Built on Nov 19, 2026 (As built, below).

Summary: The staff hands in a plan order for each intent of the commander's, and the seat's tool refused every intent of the staff's. A commander who says "control the island" therefore could not ask when the men would be ashore: the take is the staff's. A "take a place" the staff formed now has a plan order of its own, worked as a commander's take is, and the tool answers for it

Standing: Settled by the architect, Nov 19, 2026. Built on Nov 19, 2026

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

## As built

Built on Nov 19, 2026, as the record says, with one statement of the hand calculation that the engine does not give (reading 1).

- **The staff:** `crates/sim-staff/src/order.rs`. `orders` keeps an intent in force where it is the commander's or a "take a place" the staff made (`has_an_order`), in the number order `intents_in_force` gives, and works each by `order_of` unchanged (items 1, 2 and 4). The doc comments of `Staff::plan_orders` and `PlanOrder` (`crates/sim-core/src/command.rs`) and of `air::plan_orders` say so; no code in `sim-core` changes.
- **The tool:** `crates/sim-agent/src/staffing.rs`, `plan_order`, needs no change of code: it answers for any intent `plan_orders` gives an order for, and refuses the rest with `not_in_force`, the words it used (item 3). Its doc comment, the tool's description (`ORDER_DESCRIPTION` in `crates/sim-cli/src/staff_tools.rs`), the schemas' README (its table and "The plan as an order") and the title and the `intent` description of `plan_order.schema.json` say which intents it answers for. The answer's shape and the interface version are unchanged.
- **The rulebook:** chapter 8, "The plan order" (its opening and a paragraph after the list) and the `order` line in the seat's tools; the glossary's "Plan order".
- **Tests:** `tests/scenario_87_the_order_of_a_staffs_take.rs`, to its doc comment, on scenario 76's main game as scenario 76's test plays it, read at `Moment::Orders` as scenario 84 reads. Scenario 84's harness refuses only the staff's intents that are not a take, and its "A seat's tool gives it" checks that every order answered is of the commander's intent or a take the staff formed.
- **Unchanged:** no state, hash, layout version, saved game version, rules label or agent interface version; `sh scripts/regenerate.sh --check` finds every generated file as it was. The readers of `Staff::plan_orders` are the tool `order` (and through it the table's plan sheet, which asks only for the intent at the root of the sheet, the commander's, and so shows nothing new), scenario 84's test and the 0159 proof helper (whose six cases have no take the staff formed). No recorded game calls the tool.

### Readings for the architect

1. **The need to know of the take on morning 2.** The hand calculation says "No need to know is asked" on each of the four mornings. On morning 2 the engine gives one: asked by the risk, a count (balanced, no knowledge limit), nothing held, none looking, not met (plan 2, intent 2, place (4,0)); the tool's line is "Asked by the risk: a count. Nothing held; none looking. Not met." On mornings 3 to 5 it gives none, the plan being released. The order follows chapter 8's item 6 ("for a take whose plan is preparing, as The need to know gives it"), which puts the need to know on the order of every take preparing whether or not the staff waits on it; the count is not among what it waits for, since nothing can look and the take goes all the same, and the rest of the row (waits for nothing) agrees. The assertions are left failing, last in `morning_2_preparing` and `the_tool_answers_for_the_take`: either the statement is restated or the need to know is to be left off an order where nothing waits on it, which would change scenario 84's orders too.
2. **The mission.** The hand calculation gives no words for it ("the take's statement, as the staff's reasons give it"). The test asserts that the mission on each morning is among the statements of intent 2 that the staff's handed-in reasons give, as `serves` or as a link of a chain.
3. **The plan.** The hand calculation says the take has "a plan of its own" and gives no number. The test asserts the order's plan is the plan the command state gives intent 2 on that morning (plan 2).
4. **The tool's refusals.** Beyond intents 3 and 8 on morning 2, the test asserts that every staff intent in force without an order is refused on every morning, in the words of an intent the side does not have.

## The architect's rulings

1. **The need to know is on the order of a staff's take that is preparing** (Nov 19, 2026). The hand calculation said no need to know is asked on any morning. On the morning of day 2 the take is preparing, and the order of a take that is preparing gives the need to know whether or not the staff waits on it (chapter 8, The plan order): the risk asks a count, none is held, nobody looks, it is not met, and since nothing of the side's can look it is not among what the order waits for. The rule is right and the statement was wrong; the doc comment is restated by the architect.
2. **The readings of "As built" stand as made.**
3. **Asked by the table.** The table asks the order of each take under the plan it draws, and shows the day ashore at the place.

