# Step 2: what to decide and do first

A list kept by the architect, begun on Nov 12, 2026 as step 1 closed. Step 2 is the validation harness: runs compared with the record, and the model tuned against it (`docs/pillars.md`). Nothing here is decided. Each item becomes a plan, then a decision record with a hand calculation, as in step 1; since Nov 13, 2026 the coordinating architect decides them on the designer's delegation (`docs/step2-spec.md`, "Decided by the designer", 10). Items are added as they are found and struck off as they are done.

## Decisions still to make

| Item | What it is | Why now | Where it came from |
| --- | --- | --- | --- |
| A friendly field beyond the target | A strike that cannot get back to its carrier may land at a base of its own side within reach of the target, and fly from there or return later. Noted, not to be built with the first version of the reach rule | It eased long launches in the record: landing ashore was used twice at Eastern Solomons. It needs air groups that can be away from their carrier, which the game does not have (rebasing is deferred by decision 0050) | The strike-range research; added at the designer's request on Nov 12, 2026 |
| Daylight as a limit on a launch | A strike launched late must be recovered in the dark or not launched. Noted, not to be built with the first version | Dusk limited a launch as much as fuel did, and the longest reaches at Midway ended in a night recovery. Today a strike flies in the morning or afternoon pulse with no regard to the hour within it | The same |
| Experience for air groups | One number for an air group, read by a fighter's and a bomber's chance to hit and the chance of running out of fuel, diluted by replacements (decision 0064, stage 2). **It waits:** worked by hand from the rows, the facts are too thin to rest the values on, and no air group carries a value until research closes the gaps listed | The air figures are tuned meanwhile with every air group alike in experience, a named gap | Decision 0064, "Nov 13, 2026: stage 2 waits"; `docs/research/oob/air-group-experience-proposal.md` |
| Ship repair | Damage is patched forward in weeks or mended in a yard in months. **Decided (decision 0069, confirmed). Not part of step 2's close: it follows step 2, with land units and ground combat, before step 3** (ruled by the coordinating architect on the designer's delegation, Nov 13, 2026) | Without it a long campaign only wears fleets down. The research gives three bands of time away | Decision 0052, item 11; the step 2 research |
| The close of step 2 | The designer accepts the named misses and the legibility test's result, or says which is not acceptable; and decides the three questions the spec left for the close: agreements at the edge, the hand calculations on the tuned figures, the objectives scale | It closes step 2 | `docs/validation/step2-close.md` |
| "Out of the campaign" on the objectives scale | Points for a ship that must leave for repair, not only for one with propulsion or flight deck out at the end. **Not part of step 2's close: it follows step 2 with ship repair** (the same ruling); the result on the scale stays reported and not judged | Enterprise had her deck back within the hour and was away 53 days; the scale gives nothing for that | The step 2 research |

## Done

- Ships charged straight-line miles: built (decision 0058). A step is charged the miles it brings the force closer to its waypoint where the way is clear, 40 where land forces a detour.
- The hash version: raised to 2 (decision 0057) and to 3 for the new field (decision 0058).
- The reach rule: built in all its stages (decision 0059). Stage 1: reach by order and ditching for fuel; 1b: the patrol pilot's chance; 2: aircraft whose deck is lost land at another deck or a field, or in the sea; 3: Flight 300 is produced by the rule in the full slice and the soak (its scripted arrival is gone there and stays in scenario 6, which has no carriers), tested in scenario 12d. Reach beyond the radius, ditching, the range policy order (`set_reach`) and the part of "a friendly field beyond the target" that lands aircraft at a field are done by it.
- Ryujo's raid on Henderson Field and the Japanese carriers' target lists: given as scenario orders on Nov 12, 2026 (decision 0063, "As built (the scenario orders)"). Ryujo is in (7,-9) at noon, her raid flies in every run, and none of her aircraft goes down aboard her; the Eastern Solomons baseline report is remade by the second session.
- The scenario with every rule of step 2 acting together (`data/scenarios/16_all_together.toml`, `tests/scenario_16_all_together.rs`; `docs/step2-spec.md`, "Tests while figures move", 8): built on Nov 13, 2026, before any figure is tuned. One night (a night action of unequal crews, one force not ready) and one day (a hold, a strike beyond the radius, a deck lost with aircraft away, a landing at a field and a ditching, the afternoon raid on that field, the day's end count), on an altered copy of the frozen figures. It is step 2's complexity review of the rule cards.
- `batch` records what the measures count for each run (decision 0062), and `sim-cli validate` judges a batch against a battle's measures by the spec's rule of agreement. Ship passages are worked by `validate --passages` and reported, not judged.
- The reports tuning starts from, on batch version 4 after every rule put before tuning, are in `docs/validation/`, with the architect's reading in `baseline-reading.md`: Savo Island misses 5 of 23 judged measures (3 after round 2), the Eastern Solomons 4 of 14, Rabaul against Guadalcanal 6 of 10.
- Strike target priority as an order: built with scenarios 13a and 13b (decision 0063, stage 1).
- Experience for ships: built (decision 0064, stage 1); the values derived from rows and fixed.
- A force not ready fires part of its guns: built (decision 0068).
- Rates of fire as achieved: built in the derivation (decision 0070).
- A force not ready fires part of its torpedoes, and the fights by day with ships left behind looked at: built (decision 0071).
- The Japanese search at the Eastern Solomons from the record: set (decision 0073), with a search radius of its own for each aircraft type in the derivation (decision 0075).
- How long surprise lasts: decided, no rule (decision 0076).
- Which radar sets count for sighting ships at night: built in the derivation, with class ids pinned in each derivation list (decision 0077).
- Which enemy ship a ship fires at: built (decision 0078).
- Savo Island's orders from the record: the northern group's break-off level set at 1, and Yunagi in a task force of her own (decision 0024, the notes of Nov 13, 2026).
- Frozen figures go with frozen scenarios: every test on the frozen figures reads a frozen copy of scenarios 4, 5, 7 and 8 in `data/scenarios/step2-frozen/` (decisions 0073 and 0075; `docs/step2-spec.md`, "Tests while figures move", 9).
- `batch` at version 4, with who sighted first in each surface fight and the rounds each force was surprised (decision 0062).
- Cape Esperance (decision 0072): planned, its ships derived and the class file frozen, and scenario 17 written with a test of the meeting only. The check was run once on Nov 13, 2026, after Savo Island was closed: judged 8, agree 4, at the edge 1, miss 3 (`docs/validation/cape-esperance-check.md`, read in `docs/validation/baseline-reading.md`). Nothing is tuned on it.
- Tuning round 0, finding and surprise: nothing moved (`docs/validation/tuning/r0-finding-and-surprise.md`).
- Tuning round 1: `experience_gun_strength` from 0.25 to 0.5, kept (`docs/validation/tuning/r1-experience-gun-strength.md`).
- `export` takes a saved game as well as a scenario (`docs/commands.md`).
- Tuning round 2: `surprised_target_factor` from 1.5 to 2, kept; the chance to hit is closed (`docs/validation/tuning/r2-surprised-target-factor.md`, report `docs/validation/savo-island-r2.md`).
- Tubes that bear on one side, and the other side fired in the ship's next torpedo round: built (decisions 0074 and 0083); the state hash is at version 7.
- Anti-aircraft fire and the patrol's fire split before and after the attack, and whom the patrol fires at: built (decisions 0079, 0080, 0087). The Eastern Solomons' and Rabaul's orders from the record: set (decisions 0081, 0082).
- What the agreements show and do not, after an outside review: decided (decision 0084), with measures by ship and group.
- What a side is told of a fight, and the numbers a side is given: built (decisions 0085, 0086); the state hash is at version 8. The ignored test of a force made in play is switched on and passes.
- `docs/rules/differences.md`: rows 7 and 27 made, and the batch of small fixes (rows 2 with 38, 15, 17, 24, 25, 32, 34 and 39).
- Savo Island's surface tuning closed after round 2 and decision 0083, with no further round (`docs/validation/tuning/savo-island-closed.md`, report `docs/validation/savo-island-after-0083.md`).
- Air round 3, the fighters' chance to hit: nothing moved; the American losses at the Eastern Solomons named misses (`docs/validation/tuning/r3-fighter-hit-chance.md`, `r3-us-losses-named-misses.md`).
- Air round 4, claims: `fighter_false_claim_chance` from 0.1 to 0.7; the other claim figures stay (`docs/validation/tuning/r4-claims.md`, report `docs/validation/eastern-solomons-r4.md`).
- Air round 5, the guns' chance to hit: looked at, not moved (`docs/validation/tuning/r5-anti-aircraft-hit-chance.md`).
- Rabaul round 1: `[base_hit] aircraft` from 0.3 to 0.06 (`docs/validation/tuning/rabaul-r1-base-hit-aircraft.md`, report `docs/validation/rabaul-guadalcanal-air-rabaul-r1.md`). Rounds 2 (fatigue) and 3 (wear): nothing moved (`rabaul-r2-fatigue.md`, `rabaul-r3-wear.md`).
- A note on every figure of `data/reference` and the derivation parameters (`docs/validation/tuning/air-figures-closed.md`, `surface-and-search-figures-closed.md`).
- The closing tests: the hundred-seed soak, the legibility test and the rulebook-only hand calculations (`docs/validation/closing-tests.md`).
- The closing page: `docs/validation/step2-close.md`.

## Open, named by the work of Nov 13, 2026

Where an item says when it is done, that is the coordinating architect's ruling of Nov 13, 2026, on the designer's delegation.

- One fight for three forces, so that fights can overlap: a plan for the designer at the close, not built in step 2 (decision 0083, "What it carries and what it does not"). Chicago's fate and the northern group's torpedo hits at Savo Island rest on it.
- Ships of a force that do not fire at all: at Savo Island Helm and Wilson fired no torpedo, Wilson's order to attack not received and Helm's target friendly (`engagement_outcomes` 3217 to 3219), where in the rules every ship of a force that has sighted fires. `allied_torpedoes_fired` is a named miss resting on it; it is participation, not the torpedo share (architect's ruling, Nov 13, 2026; `docs/validation/tuning/savo-island-closed.md`).
- Torpedo reloads: each tube fires once in a game, where Kako fired ten torpedoes from eight tubes at Savo Island and the Takao class had a gear for quick reloading (decision 0083, "What it carries and what it does not").
- A raiding force that fights and passes on: no rule has a force retire when it has done what it came for, so `minutes_of_action` misses at Savo Island (decision 0024, the note on break-off levels).
- Secondary guns: a cruiser's secondary battery has no counterpart in the rules (`docs/step2-spec.md`, "Where the model stands today").
- Identification under fire, and a force seen and stalked before the first shot (decision 0076, "The gaps, named").
- Radar fire control, and land behind a target masking a set (decisions 0072 and 0077). No rule of radar fire control before Cape Esperance is run (0072, "Built", 1). It has been run: `gun_hits_on_aoba` misses, and radar fire control is named among its likely causes, as conjecture, beside the short night fight and the rule of targets (`docs/validation/baseline-reading.md`, "Cape Esperance: the check").
- A search flown in one pulse only (decision 0073, "What the rules cannot express").
- One kind for every cruiser, so Tenryu and Yubari fire their guns at cruisers (decision 0078, "What it cannot give").
- Experience for air groups (decision 0064, stage 2; the table above).
- The briefing's guns line after the attack (decision 0079): the briefing lists the guns' fire after the attack as a guns line of its own, not marked as after the attack. A wording item.
- The fuel round leaves out the ditchings of Ryujo's raid on Henderson Field (decision 0084, finding 13): her B5N2s fly 200 miles on a derived radius of 175 whose rows state no load, so their ditchings are not evidence for `ditch_chance_per_radius_beyond`. A radius with a bomb load is asked of research (decision 0081, C5, option 2).
- Aoba's and Furutaka's hits at Cape Esperance: measured ship by ship since Nov 13, 2026, before the check run (decision 0084, finding 14 and its note); the total of 130 to 134 is retired. Aoba is judged at 40, a weak record; Furutaka is reported only, her rows being lower bounds; Kinugasa is judged at 0 to 4. A row that bounds Aoba or Furutaka and arrives before the check is run changes the record; none changes it after. The check has been run (Nov 13, 2026), so the records stand.
- Named by the Cape Esperance check (`docs/validation/baseline-reading.md`), for step 3 and not tuned on: fire on a friend and a ship acting apart (Duncan), behind `us_destroyers_sunk` and the Japanese claims; Task Force 64 fighting Japanese stragglers by day in 771 runs of 1,000, since it is ordered to stay in (2,-5), which no measure counts; and its five destroyers firing every tube, where two attacked, the participation named at Savo Island.

## What the architect settles from the research

- Which figure to prefer where sources disagree: the Japanese aircraft lost at Eastern Solomons (about 40, 62 or 75), and each day's losses over Guadalcanal.
- In the claims round: with no ship burning, a surface claim of ships sunk equals the truth, against what the briefing promises (a claim "can be well above" it). Decision 0085, 9.
- Base sizes for the real map (decision 0052, item 15).
- The points and levels of decision 0054, and history's result on that scale for scenarios 4, 5 and 6.
- The objectives scale stays decision 0054's placeholders at the close of step 2, and the comments in scenarios 4, 5 and 6 that say it is "to be set from the record in step 2" stand until it is set: a scenario's bytes are named by the reports and the frozen copies. Step 2 reports the scale and does not judge it (ruled by the architect, Nov 13, 2026).
- Port rows that no derivation reads: `place_facts.csv` has Noumea's one large berth and 5,000 tons a day in December 1942 (rows 96, 116) and no wharves at Henderson Field or Milne Bay (58, 85). `port_sizes.csv` stays placeholders, read by no battle of the slice.

## Known gaps between the model and the record

Where step 2 ends, battle by battle, with every named miss and agreement at the edge and its cause: `docs/validation/step2-close.md`. The reports are in `docs/validation/`.

## Not step 2, kept here so they are not lost

- Speed made good. A force in the game holds its ordered speed on a straight course. In the record, where a source gives positions and times as well as a speed, a convoy or a ship in transit made good a half to three quarters of its stated speed: it zigzagged, waited, slowed for landfall or towed. Warships on a combat run are the exception (the 6th Cruiser Division from the Shortlands to off Savo Island on 11 October 1942 took what its speeds give for the distance). Noted by the designer on Nov 12, 2026 and not for step 2 (`docs/step2-spec.md`, "Decided by the designer", item 8): taken up when convoys and supply are compared with the record. The simplest answer may be a cruising speed for routine movement below a ship's stated speed, not a new mechanism. The facts are in `data/oob/passages.csv` and `docs/research/oob/passages.md`.
- Before step 3: the staff closing the range on a contact it cannot reach; choices rules make inside a day pulled into functions the planner can ask (decision 0055, "Left for later"); objectives that name a flow (`docs/design/economy.md`); land units and basic ground combat, and timing a trial of a plan by hand (`docs/design/planning.md`).
- With step 2 or 3: coastwatchers and reconnaissance as standing sources (`docs/design/intelligence.md`, layer 2).
- For step 3, named by the American losses at the Eastern Solomons (`docs/validation/tuning/r3-us-losses-named-misses.md`, ruled Nov 13, 2026): bombers' return fire at fighters (deferred since decision 0027); a search that meets enemy fighters; an aircraft written off for combat damage; a carrier's deck closed for a time, with aircraft aloft running out of fuel while it is.
- For step 3, named by Rabaul round 2 (`docs/validation/tuning/rabaul-r2-fatigue.md`, ruled Nov 13, 2026): a strike of a given size, so that a staff can send part of a group as the units at Rabaul sent about 18 bombers a raid whatever their rest; in the rules a group sends every aircraft it can fly.
- For step 3, named by Rabaul round 3 (`docs/validation/tuning/rabaul-r3-wear.md`, ruled Nov 13, 2026): small airfields that wear aircraft faster (left unbuilt by decision 0035), for the American losses on take-off, landing and ferrying that do not go with distance; and a wear that does not rise in a straight line with distance, since the dive bombers' searches and Rabaul's raids want the distance term opposite ways.
