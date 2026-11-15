# Account

## What I did (call by call)
1. get_briefing: 20-day scenario, bases 2 and 3 each at 10 days, base 1 has 1000 t and two fast transports (ships 1, 2).
2. set_intent base 2, floor 10, interval 15 (became intent 1).
3. set_intent base 3, floor 10, interval 15 (intent 2).
4. set_limit forces intent 1: ships [1,2], source base 1.
5. set_limit forces intent 2: ships [1,2], source base 1 (same pool for both; I guessed the staff would split ships between runs, and it did: ship 1 for T, ship 2 for U).
6. set_leave launch given (so the staff releases plans without waiting for me).
7. set_interrupt base 2 below 3 days; 8. same for base 3 (as safety stops).
9. staff_orders: checked the staff's plan (16 t runs, one ship each, committed days 1-4).
10. run 20 days: the one call that resolved days. It ran to game_over with no interrupt firing.
11. get_briefing and report to confirm the result.

Total: 12 calls to the game; 1 resolved days (20 days in one run). No issue_orders used.

Result: minor victory, 6.2 to 0 (31 t landed at each base). Every briefing line I read said 200 men fed at both bases, 0 unfed, bases at 21 days of supply at the end. I read only the first and last briefings, so I did not see the middle days directly; the interrupts at 3 days never fired, which is my evidence nothing got low.

## What was unclear or missing
- Whether two intents sharing one ship pool would collide. The docs say a forces limit is a pool, but not how competing intents divide it. It worked out, but I guessed.
- "Days of supply" semantics at the interval: the doc says each shipment brings interval_days above the floor, but the staff's load of 16 t (1 t deficit + 15) was far less than what a ship carries; I did not know how much a ship holds.
- The scoring ("unload and do not load back") was clear, but I did not know whether a bigger interval_days would score more points. I chose 15 (the default) rather than optimizing; a larger interval or floor would likely land more tons and more points. Nothing told me whether more points mattered beyond the mission of keeping bases fed.
- The staff-order text mentions commands (add_run, commit_ships) that are not in the tool list; harmless but opaque.
- I could not tell whether the interrupt at 3 days was a sensible threshold; the doc does not say how fast supply can fall in a day beyond planned use.
- The run answer was very long (it repeats the whole first-day staff orders); I tailed it. The structured result (days, stopped) was findable only at the end.
- Whether leave "launch" was needed: I gave it to be safe; without it the run would have stopped on waits_for_leave.
