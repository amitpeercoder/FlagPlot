# 0071: A force not ready fires part of its torpedoes

Status: decided. The designer was told on Nov 13, 2026 that this reverses his answer 4 to decision 0068 and delegated the decision to the coordinating architect in these words: 'Go ahead and decide all matters that need decisions'. The architect decided it the same day. Every figure in it is a placeholder.

Summary: A force not ready fires part of its torpedoes: the readiness of decision 0068 also takes a share of the torpedo shots, `unready_torpedo_share`. Found because at Savo Island Japanese ships left dead in the water, mostly by Allied torpedoes, are caught by day in half the runs. Reverses 0068's answer 4. Built (Nov 13, 2026)

Standing: Decided by the architect on the designer's delegation

## The gap

The spec ("Rule work that comes before tuning", the row "Surface fights by day with ships that fell behind") asks that these fights be looked at before the surface figures are tuned. In about half of 1,000 runs of `data/scenarios/04_savo_island.toml`, Japanese ships left behind by the rule for stragglers are caught after dawn. The record has no such fight (rows 14 and 15). Mikawa's force withdrew up the Slot whole. Its only loss on the way home was Kako, sunk by the submarine S-44 on 10 August, which is outside the scope of the measures (row 778). By day the Allied groups are ready and fire every gun, so these fights carry much of the Allied shells and hits that the measures say are too many.

## What the runs show

The runs are 1,000 seeds (1 to 1000) of scenario 4 on `main` at b09753e, batch label `day-fights`. The batch matches the baseline exactly: 518 runs have no fight by day. Each seed was also run with `sim-cli run --events`, and its log was read pulse by pulse. The shots and sinkings read from the logs match the batch in all 1,000 runs. Seeds 1, 2, 4 and 7 have a fight by day; seed 2 has one in the afternoon too. Seeds 8 and 10 send ships home only, and seeds 16 and 17 have no straggler. All of these were read whole, and seed 2 was traced with `events --why`.

### How a fight by day comes about

The steps below follow seed 1, with its event numbers.

1. **Hits at night stop a ship's engines.** At 01:31 the column meets the southern group. A system hit knocks a system down one level (decision 0016).
   - A gun hit does about 4 to 6 points of damage. At 0.06 for each point it has a chance of about 0.25 to 0.35 of a system hit, and 0.2 of system hits land on propulsion.
   - A torpedo hit does 25 to 300 points, so it always hits a system, and 0.8 of those hits land on propulsion.
   - Over the 1,000 runs, a Japanese ship's propulsion went to out 596 times by a torpedo and 354 times by a gun. It went to degraded 1,400 times by a torpedo and 1,301 times by a gun.
2. **The stragglers pass runs straight after the fight** (decision 0016, detach).
   - Ships dead in the water (in seed 1, Aoba and Kako, #116) become a new force, "left behind". It is idle, has no route and the avoid stance, and stays in hex (2,-5).
   - Ships with degraded propulsion go home at half speed with escorts (#113). Such a force is always moving, so it never meets anyone at the start of a pulse. It was never caught in any run.
   - A force made during a fight is not on the meeting list of that moment (decision 0022, 7). So the left-behind force does not fight the northern group that night.
3. **The Allied groups stay where they are.** They have no route and are idle in (2,-5) all day. Nothing in the scenario sends them anywhere.
4. **At 06:00 the two meet.** Both forces are in one hex and neither will move, so they meet at the start of the pulse (decision 0022, 3).
5. **The fight lasts one round.**
   - By day both sides sight with a chance of 1, so the force at the avoid stance cannot slip away. It fights back and breaks off.
   - The Allied groups were alerted at night, so they are ready (decision 0068). They fire every gun at the day chance of 0.05, with no experience factor. In seed 1 the destroyers Helm and Wilson fire 300 shots each.
   - Every fight by day ends in round 1, but one of 1,249. It ends by a break-off, often by a force that was over its level before it fired, or with the left-behind force sunk.
6. **The same pair fights again at noon.** A break-off holds only for its pulse, so 293 fights are in the afternoon.
   - Of the 702 Japanese ships left behind, 603 are sunk by day and 99 are afloat at the end.
   - 88 Allied ships are sunk by day, 44 of them Canberra.

### How often each path occurs, over 1,000 runs

| Path | Runs |
| --- | --- |
| No straggler at all | 159 |
| Ships sent home only; none of them was caught | 359 |
| A ship left dead in the water, and so a fight by day | 482 |

- **Ships left behind in a run:** 1 in 312 runs, 2 in 130, 3 in 33, and 4 or more in 7.
- **Fights by day:** 956 in the morning and 293 in the afternoon. 702 were against the southern group and 547 against the northern group.
- **Who is in them:** in every one of the 1,249 fights by day, the Japanese force is a left-behind force.

### What the record says about stragglers

No Japanese ship had her propulsion out or was slowed.

- Chokai, Aoba and Kinugasa were slightly damaged and steamed home (rows 779 to 781).
- Tenryu and Yubari had splinters and scratches only (rows 782 to 784).
- Furutaka was not hit. Her steering casualty at 01:45 was not caused by a hit (rows 147, 785 and 786).
- Yunagi was undamaged (row 787).
- No row gives a torpedo hit on a Japanese ship.

So the record has no straggler. The runs have:

- **Stragglers afloat after the night:** a mean of 2.4 and a median of 2. Only 148 runs have none.
- **Torpedo hits on Japanese ships at night:** a median of 2 and a mean of 2.8, against none.
- **Gun hits on Japanese ships at night:** a median of 42, against 10 to 17.
- **Allied torpedoes fired at night:** a mean of 48. The rows give about 4 to 6 for the ships in the scenario: Bagley 4 (rows 46 and 47), Patterson none (row 57) and Canberra none or two (rows 180 and 30). No row gives Helm's or Wilson's.

Runs with a fight by day, by the night's hits on Japanese ships:

| Torpedo hits \ gun hits | 17 or fewer | 18 to 50 | over 50 |
| --- | --- | --- | --- |
| 0 | 1 of 127 | 5 of 41 | 4 of 10 |
| 1 or 2 | 12 of 85 | 47 of 145 | 69 of 122 |
| 3 or more | 14 of 18 | 90 of 141 | 240 of 311 |

A night like the record's, with no torpedo hit and 17 gun hits or fewer, has a fight by day in 1 run of 127.

### What the fights by day add to each measure

These are counted from the logs, not estimated.

| Measure | Record | Whole run: median (5th to 95th), runs holding | Night only: median (5th to 95th), runs holding |
| --- | --- | --- | --- |
| Allied gun shells, hundreds | 4 | 13 (1 to 32), 6 | 10 (1 to 21), 8 |
| Southern group's, hundreds | 0 | 4 (0 to 13), 373 | 4 (0 to 8), 445 |
| Northern group's, hundreds | 3 to 4 | 8 (1 to 24), 79 | 7 (1 to 16), 110 |
| Japanese gun shells, hundreds | 18 | 24 (10 to 32), 26 | 23 (8 to 31), 35 |
| Gun hits on Japanese ships | 10 to 17 | 53 (2 to 159), 32 | 42 (1 to 108), 34 |
| Gun hits on Allied ships | 153 to 219 | 121 (37 to 175), 176 | 117 (32 to 171), 157 |
| Japanese ships sunk | 0 | 1 (0 to 3), 479 | 0 (0 to 2), 728 |
| Allied cruisers sunk | 4 | 0 (0 to 2), 2 | 0 (0 to 2), 1 |
| Southern group's cruisers sunk | 1 | 342 runs hold | 328 runs hold |
| Japanese ships with propulsion out | 0 | 923 runs hold | 518 runs hold |

- **Shells by day:** the Allied side fires a mean of 1,470 shells a run, 1,091 of them at night, so 25.8 percent come by day. The Japanese side fires 2,252, 2,174 of them at night, so 3.5 percent come by day.
- **The propulsion measure agrees only because of the fights by day.** The left-behind ships are sunk by day, so at the end of the run no Japanese ship is afloat with her propulsion out. Counted after the night alone, only 518 runs hold the record of none.
- **Unchanged:** both rates of hits for each thousand shells, torpedo hits on Allied ships and Allied destroyers sunk are the same within one either way.

## Which cause it is

- **Not the scenario.** The Allied groups never move, and every fight by day is two idle forces meeting at the start of a pulse. That the scenario runs to the end of the day is not at fault: in the rules a ship sinks at once, and the engine resolves whole days.
- **Not a bug.** Each rule on the way does what it was decided to do:
  - the detach instruction leaves a ship that cannot move (decision 0016, 12);
  - idle forces in one hex meet (decision 0022, 3);
  - alerted forces are ready (decision 0068);
  - by day both sides sight.

  A cruiser stopped ten miles off Savo at dawn, with the Allied screen still on station to guard the transports, as it was all through 9 August, would have been fought.
- **The cause is upstream: the cripples should not exist.** They come from the night's hits on Japanese ships, which are too many.
  - The largest single source is Allied torpedoes.
  - Decision 0068 left torpedoes out of the readiness share (its item 5), and no measure judges them.
  - No figure can mend it. `torpedo_hit_chance_night` serves both sides, and Japanese torpedo hits on Allied ships are already low: a median of 3 against a record of 4 to 9.

### Why this reverses answer 4 to decision 0068

Decision 0068, item 5, left torpedoes as they were on one row: Bagley, who fired no gun, fired four torpedoes five minutes after she sighted. The rows taken together say something else.

- **Bagley fired a quarter of her tubes.** Her four torpedoes are a quarter of her sixteen tubes. Her starboard tubes "could not be armed in time", so she fired one port mount only (engagement_events 116).
- **Patterson fired none.** "No torpedoes left the tubes", because the order to fire torpedoes was not heard as the guns opened (row 57).
- **Canberra fired none or two.** Her triggers did not respond (row 180). The 1943 US narrative says she possibly fired two (row 30).

Each of these failures has a cause of unreadiness. And Allied torpedoes are the largest maker of the cripples that the record does not have.

## Options

1. **Change no rule and add measures.**
   - The rules for stragglers and meeting stand: a cripple left in enemy waters at dawn may be fought.
   - The spec session is asked for two measures: torpedo hits on Japanese ships (record 0) and Allied torpedoes fired (about 4 to 6, once Helm and Wilson are settled). Then the source is judged rather than hidden.
   - `surface_fights_by_day` stays judged as the check.
   - It changes no state, no hash and no test.
2. **The readiness share also applies to torpedoes.** This amends decision 0068, item 5, and reuses its mechanism: no new state, a second factor beside the torpedo mount factor.
   - In a scratch experiment outside the repository, not tuning, with the share at 0.5:
     - runs with a fight by day fall from 482 to 346;
     - runs holding `surface_fights_by_day` rise from 518 to 654;
     - runs holding none for `japanese_ships_sunk` rise from 479 to 643;
     - torpedo hits on Japanese ships fall from a mean of 2.8 to 1.4;
     - the Allied shells fired by day fall from 26 to 20 percent.
   - With the gun and torpedo shares both at 0.25, 141 runs have a fight by day and 859 hold the measure.
3. **A force scuttles a ship it leaves dead in the water.** This is a new outcome of the stragglers pass. It ends the fights by day outright, but it keeps the Japanese loss, so `japanese_ships_sunk` still misses. It mends the symptom, and whether to scuttle is a commander's choice, which decision 0016 left to the staff planner.
4. **A meeting rule:** idle forces would meet only if one held a contact of the other. Rejected. The Allied side holds one after the night's report, so it would change little and would weaken a mechanism every battle uses.
5. **Scenario changes.** Rejected.
   - Allied groups at the avoid stance would still fire a round by day, where both sides sight for sure, and they would have engaged a cripple.
   - Ending the scenario at dawn is not possible: the engine runs whole days.
   - The Japanese "retire" instruction also leaves ships dead in the water.

## Decision

Option 2, with option 1's finding.

1. **A force that is not ready when a fight begins fires its torpedoes at a share.** Ready is decision 0068's `ready`, unchanged. The share is `surface_combat.toml: unready_torpedo_share`.
   - Torpedo shots = tubes x torpedo mount factor x the share.
   - The ship's torpedoes are then spent, as now (decision 0016, 6): the tubes that did not fire are not kept for later.
2. **One figure, the same for both sides, from 0.1 to 1.** It is apart from `unready_gun_share` because the torpedo failures in the record are separate statements: tubes not armed in time, an order not heard, triggers that did not respond. Its first value is 0.5, a placeholder. The frozen step 1 figures hold 1, at which the rule does nothing, as decision 0068 did for the guns.
3. **What a side knows.** The factor is in the firing ship's breakdown, named `unready_torpedo_share`, only when the force is not ready. It is cut from what the enemy may see, exactly as the gun share is.
4. **The rules for stragglers and meeting stand.** A cripple left in the enemy's waters at dawn may be fought, at Cape Esperance and by day in any battle. The mend is where the record differs: in how many cripples there are.
5. **Option 1's finding goes to the spec session:** two torpedo measures, and `surface_fights_by_day` read again after the rates of fire (decision 0070) and the shares are set. If it still misses once the night's hits on Japanese ships are near the record, option 3 is the next step.

## What the share carries and what it does not

This section does for the torpedo share what items 7 and 8 of decision 0068 did for the gun share.

- **What it carries:** a force caught unready that fails to get its torpedoes away. Its tubes are not manned or primed, or the order to fire is not heard.
- **What it does not carry.** Tuning must not take the share down to cover any of these:
  - how many tubes bear on one broadside, which is a matter for the derivation, and research is commissioned;
  - how many ships of a force fire at all, and in which round;
  - the range;
  - the chance to hit.
- **Its first value of 0.5 has no row of its own.** It mirrors the gun share's, whose row is Condition II: half the armament manned (engagement_events 23).
- **The one count the rows state is Bagley's:** four torpedoes of sixteen tubes.
- **The share is only part of the gap.** At the baseline the Allied groups fire about 48 torpedoes a run against a record of 4 to 6, a factor of eight or more. This share is one part of that factor. The rest belongs to the causes named above, and tuning is not to push this figure to close it (`docs/step2-spec.md`, "Tuning", rule 7).

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| Torpedo fire (amended; decisions 0014, 0016, 0068) | `ForceFight.ready`, which is already kept; `unready_torpedo_share`; the class's tubes; the torpedo mount state | Nothing new. A salvo's shots are fewer, and its breakdown names the factor |

## What it leaves out

- **Tubes that bear on one broadside.** A class fires all its tubes in one round (the Bagley class fires 16). On many classes about half bear on a side. Chokai fired four to port at 01:38 and four to starboard at 01:48 (rows 110 to 112). This is for the derivation, and the architect is commissioning research.
- **Scuttling a ship left dead in the water** (option 3), and towing.
- **A pursuit**, and a commander's choice to finish off a cripple or leave it.

## Seen and left

Three things were noticed and left with no change:

- A force that is over its break-off level when a fight begins still fires a full round before the check ends the fight: 517 Allied break-offs in the fights by day come in round 1.
- `EngagementBegun` for a meeting at the start of a pulse cites only the two forces' rules-of-engagement orders, as the rulebook says. So `events --why` on a fight by day does not reach the night's damage or the `ShipsDetached` event that left the force there.
- A left-behind force is not met in the pulse it was made in, but is met at the start of the next.

## As built

Built on Nov 13, 2026. The architect wrote the hand calculation before any code was delegated.

- **No new state.**
  - Readiness is read from `ForceFight.ready`, which decision 0068 already keeps.
  - `hash::STATE_VERSION` is unchanged. There is no new draw and no new event.
- **The figure.**
  - `unready_torpedo_share` in `surface_combat.toml`, from 0.1 to 1. The loader refuses a value outside that range, a float, or a missing key.
  - The live file holds "0.5", a placeholder, with the comment of "What the share carries".
  - The frozen file `data/reference/step1/surface_combat.toml` gains the key at "1", where the rule does nothing, as decision 0068 did for the guns. No hand calculation of step 1 moved, and no expected number in any existing test changed.
- **The rule.**
  - `Fight::torpedo_share(force)` is the figure if the force was not ready, else 1.
  - `torpedo_shots` is tubes x the torpedo mount factor x that share, both where the shots are fired and in `has_any_salvo`, the test for who is surprised.
  - `torpedoes_ready` is cleared after the salvo as before, however few tubes fired.
- **The breakdown.**
  - A torpedo salvo's factors are, in order: `tubes`, `rate`, `minutes`, `torpedo_mount_factor`, `unready_torpedo_share`, `base_chance`, `surprised_factor`, `lit_factor`, then `experience_factor` only in a dark fight.
  - The factor is named only when the force was not ready and the figure is below 1.
- **What a side sees.** `side_events.rs` keeps the factor among the firer's own factors of `Salvo`. `SalvoAgainst` has no breakdown, so the enemy never sees it.
- **Scenario 15b** (`data/scenarios/15b_unready_torpedoes.toml`, `tests/scenario_15b_unready_torpedoes.rs`) asserts the hand calculation:

  | Case | Shots | Expected hits |
  | --- | --- | --- |
  | Not ready, mounts sound, night | 4 | 0.16 |
  | Not ready, mounts degraded, night | 2 | 0.08 |
  | At battle readiness by order, night | 8 | 0.32 |
  | 15a's case (c), alerted in the pulse before, fight by day | 8 | 0.16 |
  | Case (c)'s control, not ready, by day | 4 | 0.08 |

  It also asserts:
  - no enemy view carries the factor;
  - with the key at 1 the log is 15a's and 4a's, event for event;
  - torpedo hits are within four sigma of what the chances expect;
  - over 300 seeds, lowering the share never raises the firing force's torpedo hits (B's total: 101 hits at 0.5, 219 at 1).

  A pinned outcome on the live figures, seed 1942, completes the three parts the spec asks for.

  Night torpedo salvos at a lit target never occur (a target is lit only by fire from an earlier round, and a ship fires its torpedoes in its first firing round), so that cell is not asserted.
- **Goldens remade.**
  - The run baselines of 04, 07, 08 and 16 changed in their hashes and logs. Savo Island's day 1 hash, on decision 0070's rates of fire, went from 14d2cad7e9647819 (249 events) to 0d5d6d45f49430a2 (250 events). Scenario 16's went from 0be882cc31372323 (145 events) to 89fa08fb5e2193b4 (146 events), and its test proper, on altered frozen figures, held.
  - The logs of 04a, 04c_in_and_out, 04c_three_forces, 14a and 15a changed only in the torpedo salvos of a force not ready; their hashes are unchanged.
  - The saves of the three recorded games, and the version 1 batch capture of scenario 6, take only the new hash of `surface_combat.toml`. The recorded games' transcripts and daily hashes are byte for byte as before.
- **What Savo Island shows.** 1,000 runs, seeds 1 to 1000, on `main` with decision 0070's rates of fire, and the measures as on `main` with this change's two new measures. Before is the engine with the torpedo share at 1, at which the rule does nothing. After is the share at 0.5. So the two differ by this decision alone. Nothing is tuned.

  | Measure | Record | Before: median (5th to 95th), runs holding | After | Verdict before / after |
  | --- | --- | --- | --- | --- |
  | allied_cruisers_sunk | 4 | 0 (0 to 1), 0 | 0 (0 to 1), 0 | miss / miss |
  | southern_group_cruisers_sunk | 1 | 0 (0 to 1), 264 | 0 (0 to 1), 257 | agrees / agrees |
  | northern_group_cruisers_sunk | 3 | 0 (0 to 0), 0 | 0 (0 to 0), 0 | miss / miss |
  | allied_destroyers_sunk | 0 | 2 (1 to 4), 49 | 3 (1 to 4), 24 | miss / miss |
  | japanese_ships_sunk | 0 | 0 (0 to 2), 647 | 0 (0 to 1), 857 | agrees / agrees |
  | japanese_ships_with_propulsion_out | 0 | 0 (0 to 1), 826 | 0 (0 to 1), 872 | agrees / agrees |
  | japanese_ships_sunk_or_stopped | 0 | 0 (0 to 2), 523 | 0 (0 to 1), 748 | agrees / agrees |
  | gun_hits_on_allied_ships | 153 to 219 | 72 (21 to 101), 0 | 74 (32 to 100), 0 | miss / miss |
  | gun_hits_on_japanese_ships | 10 to 17 | 19 (2 to 55), 210 | 16 (2 to 51), 246 | agrees / agrees |
  | torpedo_hits_on_allied_ships | 4 to 9 | 3 (1 to 7), 426 | 3 (1 to 7), 428 | agrees / agrees |
  | torpedo_hits_on_japanese_ships | 0 | 3 (0 to 7), 122 | 1 (0 to 4), 293 | edge / agrees |
  | allied_torpedoes_fired (reported) | 4 to 6 | 72 (24 to 72), 0 | 32 (12 to 36), 11 | reported |
  | japanese_gun_hits_per_thousand_shells | 81 to 117 | 52 (33 to 67), 1 | 53 (35 to 67), 1 | miss / miss |
  | allied_gun_hits_per_thousand_shells | 21 to 36 | 33 (12 to 53), 464 | 32 (12 to 52), 478 | agrees / agrees |
  | japanese_main_battery_shells_fired_in_hundreds | 10 to 11 | 13 (5 to 18), 207 | 13 (8 to 18), 244 | agrees / agrees |
  | allied_main_battery_shells_fired_in_hundreds | 3 | 5 (1 to 12), 119 | 5 (1 to 11), 144 | agrees / agrees |
  | southern_group_main_battery_shells_fired_in_hundreds | 0 | 0 (0 to 4), 527 | 0 (0 to 3), 614 | agrees / agrees |
  | northern_group_main_battery_shells_fired_in_hundreds | 3 | 4 (1 to 9), 150 | 4 (1 to 8), 170 | agrees / agrees |
  | tenryu_and_yubari_main_battery_shells_fired_in_hundreds | 1 | 2 (1 to 4), 80 | 2 (2 to 3), 37 | edge / miss |
  | minutes_of_action | 20 to 30 | 35 (20 to 65), 360 | 35 (20 to 60), 433 | agrees / agrees |
  | surface_fights_by_day | 0 | 0 (0 to 4), 599 | 0 (0 to 4), 772 | agrees / agrees |
  | japanese_ships_claimed_sunk_per_hundred_sunk (reported) | 350 | 120 (100 to 400), 3 of 968 | 125 (100 to 300), 3 of 981 | reported |
  | allied_ships_claimed_sunk | 0 | 0 (0 to 2), 626 | 0 (0 to 1), 831 | agrees / agrees |

  - **The head line:** judged 21. Before, 14 agree, 2 are at the edge and 5 miss. After, 15 agree, none is at the edge and 6 miss.
  - **Fights by day:** runs with one fall from 401 to 228, and the fights themselves from 1,380 to 820.
  - **Allied torpedoes fired:** a mean of 54.2 a run before and 27.0 after.
  - **The result on the scale:** an Allied defeat of some kind in 713 runs, where it was 541, and a victory in 31, where it was 124. The median margin goes from -3 to -4.
  - **Two measures went the wrong way.**
    - Tenryu and Yubari's shells went from at the edge to a miss: 37 runs hold the record of one hundred, where 80 did.
    - Allied destroyers sunk hold the record of none in 24 runs, where 49 did.

    Both fit fewer Japanese cripples: a column that loses fewer ships breaks off later, and its light cruisers fire more rounds at the destroyers. The rule does not touch the Japanese guns. The light cruisers' rate is decision 0070's, and how long the column fights is for tuning.
- **Still open.** `surface_fights_by_day` agrees, and 228 runs still have a fight by day. It is read again when the shares are tuned. If it still misses once the night's hits on Japanese ships are near the record, option 3 is next.
