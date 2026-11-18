# The readers' answers, after the mend

Six fresh Haiku readers on Nov 18, 2026, one case each, given the same words as before (`../prompt.txt`) and the case's events with the mended words (`cases.json`, `material`), as one file, told to read that file only. The answers are as given, word for word.

## A
(i) 0600, between the night and the morning pulses. Event 6 (pulse "morning") is the first event of the order, "order:3 accepted for task_force:1: set_speed, knots 8", and it follows the night-pulse events 3 to 5. The log gives no clock time, so this rests on the pulse labels.

(ii) The morning pulse. Event 7 shows task_force:1 steaming "48 nm at 8 kn" in the morning, and events 8 and 9 show the fuel burn at that slower speed (12 t and 6 t).

(iii) It changed task_force:1's speed from cruise to 8 knots. Event 2 set the speed to cruise, and event 3 shows 16 kn in the night pulse. Events 6 and 7 show the new 8 kn speed.

## B
(i) 1200, between the morning and the afternoon pulses. The order is accepted in the afternoon pulse (event 9, "order:3 accepted for task_force:1: set_speed, knots 8"), and the pulses before it are night and morning, with no order between them (events 3 to 8). Events 1 and 2 are the day's start, not this order.

(ii) The afternoon pulse. Events 10 to 12 show it acting there: task_force:1 steamed 48 nm at 8 kn from (8,0) to (8,2), and ship:1 and ship:2 burned 12 t and 6 t.

(iii) It changed the speed of task_force:1 from 16 knots to 8 knots. The morning pulse (event 6) shows 16 kn, and the afternoon movement (event 10) shows 8 kn, with fuel burn halved (ship:1 24 t to 12 t in events 7 and 11; ship:2 12 t to 6 t in events 8 and 12). Order 3 did not change the route.

## C
(i) 1200 is ruled out and 0600 holds: the order was given between the night and the morning pulses. Events 22 to 25 are tagged night, and event 26, order:3 "set_speed, knots 8", is the first event tagged morning, so the order acted in the morning pulse. No event carries the afternoon pulse at all. Events relied on: 22, 25, 26, 27.

(ii) The morning pulse. Event 26 is tagged morning, and event 27 "Allied holds task_force:1 by order:3" is also tagged morning.

(iii) It set task_force:1's speed to 8 knots (event 26) and held task_force:1 (event 27, held_by_order). The log does not say the hold was caused by the speed setting, only that the hold is attributed to order:3. The orders order:1 and order:2 (events 17 and 20) were given at the start of the day, before the night pulse, and are not the order in question.

## D
(i) Between the night and the morning, i.e. at 0600. The one order given inside the day is event 15, "order:15 accepted for task_force:2: set_readiness, readiness battle", tagged pulse "morning"; the day's other orders (events 1 to 14) are tagged null, as start-of-day orders. The log gives no clock times, so 0600 rests on the pulse tag alone.

(ii) The morning pulse. Event 15 is tagged "morning", and the morning events that follow (16 and 17, search_flown) come after it.

(iii) It set task force 2's readiness to "battle". The log records only the acceptance (event 15). No later event names task_force:2 or shows its readiness changing, so the change itself is not confirmed by the events.

## E
(i) 1200, between the morning and the afternoon pulses. The only order given inside the day is order:15 (event 17), logged under the afternoon pulse; orders 1 to 14 (events 1 to 14) were accepted with no pulse, at the day's start.

(ii) The afternoon pulse. Event 17 is tagged afternoon, so the order took effect in that pulse. Relied on: events 17 and 24 to 25 (air_group:6 launched nothing in the afternoon, which fits its new mission).

(iii) It changed air_group:6's standing mission from naval_strike (set by order:8, event 8) to rest. Relied on: events 8 and 17. The log shows only the acceptance, so the effect on air_group:6 is inferred from the mission change and the absence of any launch by it, not from a direct event.

## F
(i) 1200, between the morning and the afternoon pulses. Order:3 was accepted in the afternoon pulse (event 9: "order:3 accepted for task_force:1: set_route, waypoints (5,0)"), after the morning steaming in event 6. Orders 1 and 2 were accepted at the day's start (events 1 and 2, pulse null), so order:3 is the one given inside the day.

(ii) The afternoon pulse. Events 10 and 11 show the task force acting on it: it turned back toward (8,0) and then steamed 96 nm from (8,0) to (6,0), with the fuel burn in events 12 and 13.

(iii) It changed task_force:1's route to waypoints (5,0). The force reversed course to (8,0) and steamed west toward (5,0) instead of continuing on its earlier course. The events do not say whether (5,0) was reached that day.
