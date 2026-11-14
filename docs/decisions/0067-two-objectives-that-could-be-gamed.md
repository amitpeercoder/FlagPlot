# 0067: Two objectives that could be gamed

Status: confirmed. The designer approved the plan on Nov 12, 2026, with the recommended answer to each of its three questions. It is built: see "As built" at the end.

Summary: Two objectives that could be gamed: cargo landed becomes tons unloaded less tons loaded back at that base; base open is counted by days and scored by the share of days, from a record of every base at the end of each day. Built Nov 12, 2026; the state hash of every scenario with a base changed, because the hash counts the log's events

Standing: Confirmed

## The gap

The rulebook review of Nov 12, 2026 found two objectives (decision 0054) that reward play their names do not describe. Both were checked against the code.

- **`cargo_landed` can be scored more than once for the same cargo.** It counts every ton in the `CargoUnloaded` events of the side's ships at the named base. A force at a base of its own side may also load from that base's stock (chapter 2, load orders). So a force can unload, load the same tons back and unload them again, and every unloading scores. The full slice pays the Allied side for cargo landed at Henderson Field.
- **`base_open` reads only the last moment of the game.** It is 1 if the base has supply above zero and at least one ready aircraft at the end. A base that was out of supply and empty of aircraft for four weeks scores as open with one ton and one ready aircraft on the last night. A base that flew every day scores as closed if it is empty on the last night. Rabaul against Guadalcanal and the full slice both turn on it, for both sides.

Neither holds up step 2: no measure rests on either, and the result on the objectives scale is reported and not judged (`docs/step2-spec.md`, "Decided by the designer", 4). They matter the moment something plays the game for points, and the baseline already shows one effect: Rabaul against Guadalcanal ends as a major Allied victory in all 1,000 runs, because ten of its points hang on the last night alone.

## Decisions

### Cargo landed

1. **Cargo landed is the tons the side's ships unloaded at the named base, less the tons the side's ships loaded there,** supply and fuel together, and never below nothing. Cargo that is put ashore and taken off again was not landed.
2. **It is still counted from the event log.** `CargoLoaded` names its base as `CargoUnloaded` does, so nothing new is recorded and nothing is added to the state.
3. **A side still knows it exactly.** It sees both events for its own ships whole (decision 0047), so its known progress keeps the basis `exact`.

### Base open

4. **Base open is counted by days.** At the end of each day, which is after the afternoon pulse when the day's strikes have landed and not the night that opens the next day, the base is open if its supply is above zero and at least one ready aircraft of an air group is at it: the same two conditions as now, asked every day and not once.
5. **The points are for the whole game, as now, and are earned by the share of days.** A side holding `base_open` with 10 points for a base that was open on 21 days of 28 scores 7.5. The share is of the days resolved, for a game that ends early. With `closed = true` the share is of the days it was shut. So a scenario's points keep their meaning, and the two sides' objectives on one base still sum to the whole.
6. **The count comes from the event log,** as every other objective's does. At the end of each day the engine records, for every base in the game, its supply and its ready aircraft. The event changes no state. The side that owns the base sees it, and learns nothing it did not know; the other side does not see it.
6a. **Every base, whatever the objectives say.** The first draft recorded only the bases an objective names. The rulebook review found two faults in that. A side given a daily event for one of its bases and no other would learn that the enemy holds an objective on it, and a side is told its own objectives only (decision 0055). And the engine would be reading the objectives to choose what to record, where chapter 6 says no rule reads them. Recording every base closes both.
7. **What a side knows is as now.** For a base of its own, basis `exact`: its own days. For an enemy base, `not_observable`, and the count is 0.

## A small worked case

A game of four days. The Allied side holds `cargo_landed` at base 1 at 0.005 points a ton and `base_open` at base 1 for 10 points.

| | Day 1 | Day 2 | Day 3 | Day 4 |
| --- | --- | --- | --- | --- |
| Unloaded at base 1, tons | 1,000 | 0 | 1,000 | 0 |
| Loaded at base 1, tons | 0 | 1,000 | 0 | 0 |
| Supply at the end of the day | above 0 | 0 | above 0 | above 0 |
| Ready aircraft at the end of the day | 3 | 3 | 0 | 2 |
| Open | yes | no | no | yes |

- Cargo landed today: 2,000 tons, 10 points. By this plan: 2,000 less 1,000 is 1,000 tons, 5 points.
- Base open today: open on the last night, 10 points. By this plan: 2 days of 4, 5 points.

## Why

- **The name is the test.** An objective should pay for what a commander would recognise as the thing: cargo that stayed ashore, a field that was in use.
- **Net tons, not tons traced to their port.** Paying only for cargo that came from another base would need each ship to remember where its cargo was loaded: new state for every ship, in the hash, to close the same hole.
- **A share of days, not all or nothing.** Requiring every day would give the enemy the whole objective for one bad night, which is the fault being mended, turned round.
- **No new mechanism.** Both stay a count read from the log at the end, like every objective.

## What it leaves out

- **A base that is open at dusk and was useless all day,** for want of fuel or with every pilot spent. The two conditions are the ones the objective already has. A better test of a working field is a question for the scale as a whole.
- **"Out of the campaign"** and the other reworking of the scale, which stay on the opening list. *(Nov 14, 2026: "out of the campaign" is built as ships out of action by decision 0088)*
- **Cargo destroyed ashore.** A ton landed and then bombed still counts as landed.
- **Fuel landed and then taken as bunker fuel.** A ship that refuels from the base's stock records `ShipRefuelled`, not `CargoLoaded`, so the fuel still counts as landed. Bunker fuel cannot be unloaded again, so there is no way round the rule here: the rulebook review looked for one.

## What it costs

- The results of the full slice and of Rabaul against Guadalcanal change, and so do the recorded games' scores. Every hand calculation that includes either objective is worked again by the architect before the code.
- One new kind of event, with its line in `side_events.rs` saying who may see it. No new state. It is recorded for every base on every day, so the event numbers move in every golden file and transcript of a scenario that has a base, not only in the two scenarios that hold the objective. Whether an event that changes no state is the right carrier, or the day's count is better kept another way, is the build architect's to settle before the hand calculation, within decision 6.
- Chapter 6 of the rulebook and the glossary change in the same pull request as the code.
- The baseline reports' tallies of outcomes for the two scenarios are remade.

## Rule card

| | |
| --- | --- |
| Reads | The `CargoLoaded` and `CargoUnloaded` events of a side's ships at a named base. At the end of each day, the supply and the ready aircraft of every base |
| Writes | No state. One event a day for every base |

## Answered by the designer

On Nov 12, 2026, as recommended in each case.

1. **Cargo landed: net of what was loaded back at that base, or only cargo that came from another base?** Net (decision 1). The second is closer to the idea and costs new state for every ship.
2. **Base open: the share of days, or open on every day or it does not score?** The share of days (decision 5).
3. **Who builds it?** The rulebook review session, which found the two faults, found the fault in the first draft of decision 6 and reviewed this plan. The build session is on experience (0064) and the plan for a force not at battle readiness (0068).

## As built

Built by the rulebook review session on Nov 12, 2026.

- **The event is the carrier.** `BaseAtDayEnd { base, supply_t, aircraft_ready }` is recorded by a fifth end-of-day phase, the base count, for every base in base order. It belongs to no pulse and has no cause. The test of "open" is one function, `events::base_is_open`, which the log's text, the tally, a side's known progress and the replay export all call.
- **The state hash of every scenario with a base changes,** which this record did not foresee. No state was added, but the hash counts the events of the log, and the state holds event numbers (a force's arrival, a contact's last report, a claim), which move. The hash version stays at 5: no field was added. The run baselines, the old batch fixtures (hashes only, as decision 0063 did) and the recorded games' saves were remade.
- **The count of a base is days,** with the days counted beside it (`days`), in the result and in what a side knows. The points are the objective's points times the days as wanted over the days counted, cut to six places: one day of three for 10 points is 3.333333.
- **A side's known progress reads no state.** It takes the days of its own bases from its own `BaseAtDayEnd` events and its net cargo from its own cargo events. It no longer reads the base's supply or aircraft.
- **The agent interface is version 3.** The meaning of two counts changed, and the schemas' version rule requires it. The briefing says "open at the end of each day" and "3 of 5 days".
- **The scores of the two scenarios did not change.** Henderson Field ends every day open in both (28 of 28 and 14 of 14, seed 1942), and nothing is loaded back there in the full slice, so the points are 10 and 4.8 as before. Other seeds can differ; the baseline reports are the spec session's to remake.
- **Tests.** The worked case of this record is the hand calculation in `objectives.rs` (`the_worked_case_of_decision_0067_scores_net_cargo_and_the_share_of_days` and four beside it). `tests/scenario_10b_objectives.rs` carries each day's supply and ready aircraft at Henderson Field for scenarios 6 and 7 and asserts every day's event against it. Tests that counted a day's events count one more for each base, each with a note.
- **Rulebook.** Chapter 1 (the base count phase, and who sees the event), chapter 6 and the glossary.
