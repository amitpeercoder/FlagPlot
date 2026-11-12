# 0068: A force not at battle readiness fires with part of its guns

Status: confirmed. The designer approved the plan on Nov 12, 2026, with the recommended answer to each of its six questions. Built; see "As built". It was built after experience (decision 0064), which changes the same file. The hand calculation was written before any code was delegated.

## The gap

At Savo Island the record has the Allied ships fire 471 shells and the Japanese 1,867 (`data/validation/savo_island.toml`, rows 10 and 7). Over 1,000 runs on the figures of Nov 12, 2026 the Allied groups fire about 2,790 a run and the Japanese about 2,170. The Allied count is six times the record. It hid behind a rate: Allied hits for each thousand shells agreed with the record while the shells and the hits were both far too many (`docs/validation/baseline-reading.md`).

Where the shells come from, a run, by a study of 1,000 seeds:

| | Model | Record |
| --- | --- | --- |
| Allied cruisers, at night | 907 | 107 eight-inch, and about 107 from secondary guns the model does not have |
| Allied destroyers, at night | 1,333 | 249 to 266 |
| Allied ships, in daylight fights with Japanese stragglers | 552 | none: no such fight took place |

Why. In the rules every factor of a night fight is the same for both sides. A force that has not sighted the enemy holds its fire, and is hit at one and a half times the chance, for the one round in which it is surprised. From the next round it fires its full broadside. Being at battle readiness changes how well a force's lookouts sight and nothing else (decision 0021).

What the record shows is a different thing. The delay was short: the Allied cruisers answered within two to nine minutes of the first flare, about one round, which the rules already give. What was missing was the weight of fire afterwards. Shells fired against what the rules' full broadside would fire in the time each ship was in action:

| Ship | Shells | Share of a full broadside |
| --- | --- | --- |
| Canberra | 0 eight-inch, about 3 four-inch | 0 to 2 in 100 |
| Chicago | 0 eight-inch, 25 five-inch | 0 to 3 |
| Vincennes | about 33 eight-inch | 5 |
| Quincy | 21 eight-inch | 3 |
| Astoria | about 53 eight-inch | 8, with no rise over 24 minutes |
| Patterson | 33 to 50 | 4 to 21 |
| Bagley | 0 | 0 |
| Helm | 4 | under 1 |
| Wilson | 212 | 14 |

The rows give the first cause in so many words. Every Allied ship was at Condition II, which "means half the armament and all fire control and communications manned" (engagement_events 23). Vincennes and Astoria had two guns of each turret manned, Quincy one forward turret and half the after one, Canberra two turrets of four with all guns empty (rows 23, 25, 27). The captains of all but one were asleep (row 24). The Japanese force had gone to its highest condition at 19:50 and alerted battle stations at 00:45 (row 30).

At Cape Esperance, two months later, the side that was not ready was the Japanese, and the shape is the same: no Japanese ship fired for seven minutes, and then Aoba fired 7 main battery shells, Furutaka between 20 and 40, and Kinugasa, who was not hit, 80 in twenty-five minutes, about a fifth of her broadside (engagement_outcomes 1647 to 1654, 1699, 1700; the Japanese rows are of low confidence).

Experience (decision 0064) does not reach this. It changes how well a ship hits. At its cap it takes about a fifth to a third off the Allied excess, by sinking and silencing Allied ships sooner, and leaves about four times the record. And no figure in the model can do it: the readiness figures act on sighting only. Set to their extremes they bring the Allied shells down only by making the Allied ships unable to see an enemy that the rows say they saw (rows 62, 64, 66, 84). That is a figure standing in for a missing rule, which the spec forbids.

## Decisions

### The rule

1. **A force that is not ready for action when a fight begins fires its guns at a share of their full rate, for the whole of that fight.** Each of its ships' gun shots in a round are multiplied by one figure, `unready_gun_share`. It is one more factor beside the one that already halves or stops a ship's shots when her main battery is damaged, and it is recorded in the same breakdown.
2. **Ready means at battle readiness by order, or alert since before the pulse began.** A force its commander ordered to battle readiness is ready, and pays for it in fatigue as decision 0037 has it. A force that was alerted in an earlier pulse (by a contact, a fight or an alarm) has had time to man its guns and is ready. A force alerted only in this pulse is not: its lookouts are warned, which is what being alert already gives, and its guns are still half manned. So the northern group at Savo Island, alarmed by the gunfire to the south minutes before the searchlights came on, sights better and still fires with part of its guns, as the rows have it ("battery stations not yet manned", row 85).
3. **The share does not rise during the fight.** A fight is at most thirty minutes. Astoria's twelve salvos over twenty-four minutes show no rise to full fire, and no row gives a rate at which guns came into action.
4. **It is the force's condition, not a ship's.** Every ship of the force fires at the share. There is no new state for a ship.
5. **Guns only.** Torpedoes are fired as now. Bagley, who fired no gun, fired four torpedoes five minutes after she sighted.
6. **Nothing else changes.** The one round of surprise, the chance to hit and to be hit, sighting, the alarm and breaking off are as they are.

### The figure

7. **One figure, the same for both sides, from 0.1 to 1.** Its first value is 0.5, which is Condition II as the row states it: half the armament manned. Tuning may move it, toward the record and never for one battle. The frozen step 1 figures hold 1, at which the rule does nothing, so no hand calculation of step 1 moves.
8. **What the figure stands for, said plainly.** Half the guns manned is the part a row states. The record's shares are lower than a half (3 to 21 in 100), and the rest has other causes that this rule does not model one by one: captains called from sleep, ships taken for friends, orders to cease fire, guns loaded and not primed. If tuning takes the figure below 0.5, it is carrying those, and the tuning record says so.

### What a side knows

9. A side sees its own force's readiness, as it does today, and the share among the factors of its own fire. It sees nothing of the enemy's: an event the enemy may see is cut of the factor.

## A small worked case

Three cruisers of eight guns, three rounds a gun a minute, in a round of five minutes: a full salvo is 8 x 3 x 5 = 120 shots a ship. Neither side is surprised or lit; the night chance to hit is the placeholder 0.03; the share is 0.5.

| Force | Ready | Shots a ship | Hits expected a ship |
| --- | --- | --- | --- |
| A, at battle readiness | Yes | 120 | 120 x 0.03 = 3.6 |
| B, at normal readiness, never alerted | No | 120 x 0.5 = 60 | 60 x 0.03 = 1.8 |
| B, alerted in an earlier pulse | Yes | 120 | 3.6 |

The hand calculation takes this case through a whole fight, on scenario 4a's ships.

## What it would do, by arithmetic

These are sums on the round-by-round table of the 1,000 runs, not runs. They hold the length of each fight as it is, which the rule itself will change.

| | Allied shells a run |
| --- | --- |
| Today | about 2,790: 2,240 at night and 550 by day |
| Share 0.5 | about 1,670: 1,120 at night and 550 by day |
| Share 0.25 | about 1,110: 560 at night and 550 by day |
| The record | 471 |

The Japanese, at battle readiness, are untouched. So the rule takes the night's Allied shells most of the way and leaves two things it does not reach, which are named below so that tuning is not asked to close them.

## The test

Cape Esperance is built after Savo Island is tuned and is never tuned on. There the Japanese are the force that was not ready, if the rows on their state stand (see the questions). A share fixed at Savo Island either gives their thin reply or it does not.

Scenario 4a with one force at normal readiness is the rule's own scenario, with the three-part test the spec asks of every new rule: the shots worked by hand at a working value of the figure and read from the breakdown, a property over many seeds, and a pinned outcome for one seed.

## Why

- **It is what the rows say.** Half the armament manned is a statement in the record, not a fit.
- **It makes an order mean something.** Battle readiness costs fatigue today and buys better lookouts. Now it also buys a force's guns. A commander who expects a fight at night orders it and pays; one who does not is caught as the cruisers off Savo Island were. That is command by intent, which is the game.
- **One factor in a place that has one already.** The shots a ship fires are already multiplied by a factor for her battery's state. This is a second factor of the same kind, with no new state, no new draw and no new event.
- **Off unless set.** At 1 the rule does nothing.

## What it leaves out

- **The daylight fights with stragglers, about 550 Allied shells a run.** A Japanese ship with her propulsion out is left in the hex by the rule for stragglers (decision 0016) and shot at in the morning. In the record no Japanese ship was crippled and none was caught at dawn. With the Allied ships hitting less at night there will be fewer cripples and fewer such fights; what is left is looked at after this rule and experience are in. Whether the shells measures count the night alone is the spec session's to say.
- **The rate of fire itself.** A class's rounds a gun a minute is the designed rate. The rows that give an achieved rate are few and of low confidence, and put it at a half or less of the designed one for both navies. The Japanese light cruisers and destroyer fire about six times their record in the model, at battle readiness, which this rule does not touch. That is a derived figure and a question for the derivation, not for this rule.
- **Secondary guns.** The record counts them (about 107 Allied and 654 Japanese shells). The model has one battery to a ship.
- **Ship by ship.** Wilson fired 212 shells and Helm 4 in the same force. The rows separate destroyers from cruisers a little and give nothing a rule could read for one ship.
- **Ships taken for friends, and orders to cease fire.** Both nights have them (Quincy and Helm at Savo Island; the American admiral at Cape Esperance). They are part of what decision 8 names and are not modelled.
- **A share that rises**, and a delay longer than the one round of surprise: the rows support neither.

## What it costs

- One new figure in `surface_combat.toml`, a placeholder at 0.5, and 1 in the frozen file.
- Whether a force was alert before the pulse began must be known when a fight begins. If the state does not already say when a force was alerted, it gains one small field, the hash version rises and the golden files' hashes are remade. The build says which.
- Scenario 4 changes in its results and not in its file: both Allied groups are at normal readiness already. The Savo Island baseline is run again.
- The tests of scenario 4a and 4c that pin shots for a force at normal readiness are taught the rule; on the frozen figure of 1 they do not move.
- It is built after experience (decision 0064), which changes the same file, and before any figure is tuned.

## Rule card

| | |
| --- | --- |
| Reads | A force's readiness and whether it was alert before the pulse began; the figure `unready_gun_share`; a ship's guns, rate of fire and main battery state, as now |
| Writes | Nothing new. The shots of a salvo are fewer, and the salvo's breakdown names the factor. If decision 2 needs it, the pulse in which a force was alerted |

## Answered by the designer

On Nov 12, 2026, as recommended in each case.

1. **What makes a force ready?** Battle readiness by order, or alert since before the pulse began (decision 2).
2. **A share for the whole fight, or one that rises?** For the whole fight (decision 3).
3. **May tuning take the figure below the half that the row states?** Yes, down to 0.1, with the tuning record saying what the figure then carries (decisions 7 and 8).
4. **Torpedoes left as they are?** Yes (decision 5).
5. **The daylight fights with stragglers:** looked at after this rule and experience are in.
6. **Cape Esperance:** the rule does not wait. The architect commissions research on the state of the Japanese ships there before that scenario's readiness is set.

## As built

Decisions 1 to 9 in the simulation, with the architect's settlements of the brief.

- **No new state.** Whether a force was alert before the pulse began is read from the event that alerted it: `TaskForce.alerted_by` names an event, and an event has a day and a pulse. So no field was added, `hash::STATE_VERSION` is unchanged (`flagplot-state-v6`), no hash and no golden file changes on account of the state, and `name_every_field` has nothing to add. An alerted force with no `alerted_by` (which no rule makes; only a test sets it) counts as alerted before.
- **Ready.** `World::force_is_ready(force, day, pulse)`, called once for each force in `begin_fight` and kept in `ForceFight.ready` for the whole fight. Ready is: readiness `Battle` (by order or from the start), or alerted with an `alerted_by` event of an earlier day, or of an earlier pulse of the same day. A force alerted in the present pulse is not ready, by each route: a contact at the start of the pulse (`alert_from_belief` records its event in that pulse), being sighted or fired on in an earlier fight of the pulse (`ForceBecameAlert`), or the alarm (`AlarmRoll`). A force alerted by a contact in an earlier pulse and still alerted is ready. A normal force never alerted is not ready.
- **The reading of a contact.** A force alerted by a contact at the start of the pulse of its fight is not ready in that pulse: it has been alert since that pulse began, not since before it. Knowing of a contact is not an order to man the guns; battle readiness is.
- **Figure.** `unready_gun_share` in `surface_combat.toml`, from 0.1 to 1 (the loader refuses outside, and a float or a missing key). The frozen file holds "1"; the live file "0.5", a placeholder, with the comment that 0.5 is Condition II as engagement_events row 23 states it and that tuning may move it and then says what it carries (decisions 7 and 8).
- **The rule.** `gun_shots(ship, class, minutes, figures, share)` is `min(full x main_battery_factor x share, ammunition)`: the share is a factor beside the battery factor, before the cap. The share is `Fight::gun_share(force)`: the figure if the force was not ready, else 1. `has_any_salvo` (the test for who is surprised) reads the same share. Torpedo shots, hit chances, sighting, surprise, the alarm and breaking off do not read it. No new draw and no new event.
- **Breakdown.** A gun salvo names `unready_gun_share` straight after `main_battery_factor`, only when the firing force was not ready and the figure is below 1. On the frozen figures no breakdown and no event changes.
- **What a side sees.** In `side_events.rs`, `unready_gun_share` is added to the own factors of `Salvo`, the firer's side (decision 9). `SalvoAgainst` has no breakdown, so the enemy never sees it; no method of `SideView` returns a force's readiness or share.
- **Scenario.** `data/scenarios/15a_unready_fire.toml` is scenario 4a exactly; `tests/scenario_15a_unready_fire.rs` has the architect's hand calculation and the three parts (the shots at the working value 0.5, properties over 1,000 seeds and the cases of a force ordered to battle readiness, alert since an earlier pulse and alarmed in the pulse, and a pinned outcome on the live figures). No other scenario file changes.
- **The batch file** goes to version 3 with `shots_by` (decision 0062, "Version 3"), so that the Savo Island shells measures can be taken by group.
- **What Savo Island shows at the live share of 0.5 (1,000 runs, before and after, on the same seeds; nothing tuned).** Allied gun shells in the middle run fall from 2,515 to 1,378 (the mean of a run from 2,401 to 1,470; the record is 471). The southern group's gun shells in the middle run go from 875 to 438, the northern group's from 1,493 to 870, and the three Japanese light ships' stay at 1,230 (they are at battle readiness). The head line goes from 8 agree, 2 at the edge, 6 miss to 8 agree, 1 at the edge, 7 miss: the Allied shells measure still misses (median 25 to 13 hundred, record 4), the Japanese hits on Allied ships and Allied destroyers sunk are not mended, and gun hits on Japanese ships, at the edge before, now miss (median 96 to 53, record 10 to 17). What the rule leaves out is named above (the daylight fights with stragglers, the rate of fire, secondary guns).
- **The cases tested** (`tests/scenario_15a_unready_fire.rs`, approved by the architect). (b) Force B ordered to battle readiness is ready and its log is scenario 4a's with B at battle readiness. (c) B at normal readiness with a start contact at its hex is alerted in the night pulse, and when A reaches it in the morning (by day) it is ready: 120 shots, sighting 1; without the contact it is not ready: 60 shots, sighting 0.5. A night version was not wanted. (d) A third force C alarmed by the first fight (alarm chance 1 in the altered figures) is not ready in its own fight: 60 shots, sighting 0.5. The test works readiness out from the log alone, as an independent check, over 1,000 seeds, ammunition included.
- **The three groups' gun shells at Savo Island** from `shots_by` (median of 1,000 runs, before and after at 0.5): southern group (ships 9 to 12) 875 to 438, northern group (13 to 17) 1,493 to 870, Tenryu, Yubari and Yunagi 1,230 to 1,230.
- **Still open.** Allied shells are 13 hundreds in the middle run against a record of 4. The Japanese hit rate and the Allied cruisers sunk are not this rule's: it does not touch them.
- **Rule card as built.** Reads: a force's readiness, its `alerted` flag, the day and pulse of the event named by `alerted_by`, the figure, a ship's guns, rate and battery state. Writes: nothing.
