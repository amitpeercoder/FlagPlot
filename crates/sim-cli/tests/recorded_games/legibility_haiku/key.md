# Key to the 24 reasons (DRAFT, by the team lead, for the architect's reading)

Ruling 39 says the key is the architect's, written from each reason before the model is run. This draft was written by the team lead from `reasons.json` alone, before any model saw a reason, and is marked a draft until the architect has read it.

Question 1 passes if the answer names the base and the aim (keep it at so many days of supply, by sending, by waiting, or by saying it cannot). Question 2 passes if it gives a condition that the reason's own "would change" or "binding" part states (any one of those listed below).

| # | Kind | Q1: base and aim | Q2: conditions the reason states |
| --- | --- | --- | --- |
| 1 | order | base:2 at a floor of 30 days of supply, by sending a run of 360 t (forming ship:1 into a force for it) | the reading of day 3 at or above 300 t (no run would be needed); a free ship of the pool earlier in number order would carry it; the floor of 30 days or the interval of 15 days (binding) |
| 2 | order | base:2 at 30 days, by sending a run of 360 t (loading it on ship:1) | as 1 |
| 3 | order | base:2 at 30 days, by sending a run (forming ship:1), its load cut from 36 t to 30 t by the ship's room | the reading of day 3 at or above 30 t; an earlier free ship; the ships' room of 30 t (binding); the floor or interval (binding) |
| 4 | order | base:2 at 30 days, by sending a run of 30 t (loading it), cut by the room | as 3 |
| 5 | order | base:2 (keep it supplied at its floor), by sailing run 1 with its 360 t now that it is loaded | a load not complete; a plan not released |
| 6 | order | as 5 (the unload order of that run) | as 5 |
| 7 | no order | base:2 at 30 days, by waiting: no run needed now (16 days today, run 1 under way); next run expected on day 16 | the reading of day 18 below 300 t (a run on day 16); a larger use at the destination; the floor or interval (binding) |
| 8 | no order | base:2 at 30 days, by waiting: 38 days today, no run needed; next expected day 10 | the reading of day 12 below 30 t; a larger use at the destination; floor or interval (binding) |
| 9 | no order | base:2 at 30 days, by waiting (43 days today); next run expected day 16 | as 7 |
| 10 | no order | base:2 at 30 days, by waiting (37 days); next expected day 10 | as 8 |
| 11 | no order | base:2 at 30 days, by waiting (15 days today, run 1 under way with 280 t) | as 7 |
| 12 | no order | base:2 at 30 days, by waiting (42 days) | as 7 |
| 13 | displaced | base:3 at 30 days; it says it cannot: no ship free until day 5, ship:1 committed to plan:1's run | plan:1's commitment ending / ship:1 free on day 5; the reading of day 7 below 30 t; a larger use at the destination; no ship of the pool free (binding); displaced by plan:1 (binding) |
| 14 | displaced | as 13 (day 2) | as 13 |
| 15 | displaced | as 13 (day 3) | as 13 |
| 16 | displaced | as 13 (day 4) | as 13 |
| 17 | hold | base:2 at a floor of 40 days, by sending a run of 190 t on ship:2, since ship:1 is held | ship:1 released (it would then be free and would have been sent); the reading of day 12 at or above 400 t; an earlier free ship; the hold on ship:1 (binding); the floor of 40 days (binding) |
| 18 | hold | as 17 (the commitment of ship:2) | as 17 |
| 19 | hold | as 17 (forming ship:2) | as 17 |
| 20 | hold | as 17 (loading 190 t) | as 17 |
| 21 | leave | the reason names no base: it is plan:1 of intent:1 made ready (preparing) because leave to prepare is given. An answer cannot name the base from this reason alone, so by the ruling's wording question 1 is expected to fail: a finding about the reason, not the model. Marked against "names the aim: make the plan ready" if the architect rules the base may be omitted where the reason has none | leave to prepare withheld |
| 22 | leave | as 21: plan:1 released because leave to launch is given; no base named | leave to launch withheld |
| 23 | leave | as 21, plan:2 of intent:2 | leave to prepare withheld |
| 24 | leave | as 22, plan:2 | leave to launch withheld |

## Added for the second run (Nov 15, 2026)

Added by the team lead of the mend, from `reasons_second_run.json` alone, before any model saw a second-run reason. The requirement above is not changed: question 1 passes if the answer names the base and the aim; question 2 if it gives a condition the reason's own "would change" or "binding" part states. This adds only the base and aim each of the four leave reasons now states, in its serves part and in its worked part.

| # | Kind | Base and aim the mended reason states | Q2 as above |
| --- | --- | --- | --- |
| 21 | leave | base:2 (Placeholder base T), kept at 30 days of supply, a run every 15 days; plan:1 made ready (preparing) because leave to prepare is given | leave to prepare withheld |
| 22 | leave | base:2 (Placeholder base T), kept at 30 days of supply, a run every 15 days; plan:1 released because leave to launch is given | leave to launch withheld |
| 23 | leave | base:3 (Placeholder base U), kept at 30 days of supply, a run every 15 days; plan:2 made ready (preparing) because leave to prepare is given | leave to prepare withheld |
| 24 | leave | base:3 (Placeholder base U), kept at 30 days of supply, a run every 15 days; plan:2 released because leave to launch is given | leave to launch withheld |
