# 0152: Forces with men aboard: what the 39 days ask of steps 7 and 8

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139), after reading the staff's setup of the 39 days against decisions 0142 to 0151. In that setup every man of both sides who is not on Guadalcanal or Tulagi on 7 August is aboard a transport force, at sea or in port, and each force may be used from its own day (decision 0140). Decisions 0149 and 0151 choose among units ashore beside ships, and decision 0143 sends after only what was left ashore. This record widens the three so that the staff can use what the campaign gives it. It adds no rule to the engine. Nothing is built. It is built with decision 0151, after decision 0150.

Summary: A transport force with men aboard is a lift as it stands: "take a place" takes one as its first lift, sends each later one while the place is not taken and men are still wanted, and lands the working men aboard with the rest; a hold sends to its place a force with men aboard that no take needs; and the supply under a hold formed by "control an island" uses whatever transport forces lie empty at its source that morning

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Not built

## Why

The landings of 7 August begin with the men at sea. Every Japanese reinforcement of the 39 days is a run with its men aboard, usable from the day the record first has it. Without this record the staff of either side would find no base with a unit ashore beside a transport force, form nothing, and say it cannot meet the intent. That is a fault of the rules as written and not a finding about the staff.

## A lift as it stands

1. **A lift as it stands** is a transport force the staff may take (decision 0149, item 5) that has men aboard. Its units are the units aboard it, whole. Its source, for its way back and for the check of decision 0144, is its home port, or with none the nearest base of the side by the engine's sea path, the lower number on a tie.
2. **For "take a place" with no forces named** (decision 0149), the lifts as they stand with a fighting unit aboard come before any base: the first lift is the lowest-numbered such force, and the base rule of that decision's item 7 is used only where there is none. The men counted against the men wanted are its fighting men.
3. **Everyone aboard lands.** The unload order lands every man aboard, so working units aboard the lift land with it. They are not units of the operation for the judgement of decision 0143, and the reason names them.
4. **The supply that lands with the men** (decisions 0142, item 16, and 0144, item 13) is reckoned for every man aboard the lift. A force that does not lie at its source can take nothing more: it goes with what is aboard, and the reason gives what it is short.

## Later lifts

5. **Decision 0143's second lift is widened.** While the place is not taken, and no estimate is held or the fit men who can attack are fewer than the men wanted, the staff sends, in this order: each lift as it stands with a fighting unit aboard that no "take a place" higher in priority has taken, in force order, on the first morning the side may use it; then the units left ashore at the source, as that decision has it.
6. **A later lift as it stands** is given a route to the place and back to its source, marked to act at the place, and an unload order, on that morning: it has nothing to load. Its units are added to the forces limit of the task "ready a landing", as decision 0143, item 12, has it. They are not set to attack before they sail: the judgement of each morning sets them.
7. **One force a morning to a place** is not asked: each lift is its own force and goes when it can.

## Men for a place already held

8. **A lift as it stands that no "take a place" in force can use,** because none is in force for a place not yet taken or its men are all working men, is sent by "hold a place": on a morning a hold's plan is released and its base is the side's, each such force, in force order, that no hold higher in priority has taken, is given a route to the base's hex and back to its source, marked to act there, and an unload order. Under "control an island" the holds are in the island's order, so the first place held has them.
9. **A run by night** (decision 0145) covers these runs and the later lifts, as it covers a lift.

## The supply under an island's holds

10. **The source,** where the commander names none, is chosen on the first morning there is a base that fits decision 0151, item 8, with "a transport force it may take lies stopped there with nothing aboard" for that item's test, and kept.
11. **The ships.** Each morning the staff sets the forces limit of each hold it formed to the source and the transport forces it may take that lie stopped at the source with nothing aboard, where that differs from the limit the hold has. So a convoy that may be used from a later day joins the pool on that day.

## What it leaves for later

- **Men moved between places already held,** as the crossings from Tulagi to Guadalcanal were.
- **Which place a lift goes to,** chosen by what each place needs: the first take or hold in priority has it.
- **Where on an island a lift lands:** at the place itself, as before. The choice of a beach is stage 4's.

## Rule card

- **Reads:** the side's own transport forces with the men and supply aboard, their days and home ports; the intents in force and their priority.
- **Writes:** nothing of its own: `set_limit` on intents the staff formed, and route and unload orders.
- **Reuses:** decisions 0142 to 0151 whole.

## What it costs

- **State:** none. The doctrine version rises with decision 0151's.
- **The rulebook's chapter 8:** "Taking a place", "Holding a place" and "Controlling an island" each gain a paragraph.

## The hand calculation

`tests/scenario_77_forces_with_men_aboard.rs`.
