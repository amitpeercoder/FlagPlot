# Open work

A list kept by the architect of what is decided and not built, what is open, and what is next. Begun on Nov 12, 2026 as step 2's opening list. Each item becomes a plan and a decision record with a hand calculation before it is built.

## Next, before step 3

Decided by the designer at the close, Nov 13, 2026 ("Reported only; repair first"; `docs/step2-spec.md`, "Decided by the designer", 16), in this order:

1. Ship repair (decision 0069), built (scenario 20a), then ships out of action on the objectives scale (decision 0088, confirmed Nov 13, 2026: counted by the time a ship is away against a horizon; the name "out of the campaign" is dropped), built (scenario 20b); no scenario of the slice uses it yet.
2. The objectives scale set from the record: the points and levels of decision 0054.
3. Land units and basic ground combat. Stage 1, land units carried by sea and fed from a base (decision 0066), is built (scenario 21; the rulebook's chapter 7). Stage 2, the fight ashore and a base that changes side (decision 0090), is built (scenario 23; the rulebook's chapter 7), after rebasing an air group by order (decision 0091, scenario 22). Stage 3, shells and bombs on a base and its men and what a side knows of the enemy ashore (decision 0092: shore bombardment, the share of a hit that lands on men, the daily ground report and the estimate, and an opening estimate), is built (scenario 24; the rulebook's chapters 1, 3, 4, 5 and 7). Marching overland (decision 0094: a march order along the map's land links, its days and tiredness, and the supply a unit takes with it; scenario 25; the rulebook's chapters 1, 2 and 7) is built. It leaves out terrain and roads, routes of more than one hex, pursuit and blocking, marching part of a unit, and losses on the march. The real map has no land links file yet: it is data for the next stage, to be settled by the architect, hex by hex with the landmass and the reason for each link. Stage 4 of `docs/design/ground-combat.md` is still to come: setting the ground figures against the record, the attacks on the airfield (the research of `docs/research/oob/land-forces.md`), which are all placeholders now.

The designer confirmed on Nov 13, 2026 that land units and basic ground combat stay before step 3. The step 3 spec is being written by the architect in parallel.

## Not planned

- Submarines are not modelled and have no plan yet. The designer asked about them on Nov 14, 2026. No rule, ship class or scenario has a submarine, and nothing has been decided of how a side would command one or find one.

## Decisions still to make

| Item | What it is | Why now | Where it came from |
| --- | --- | --- | --- |
| A friendly field beyond the target | A strike that cannot get back to its carrier may land at a base of its own side within reach of the target, and fly from there or return later. Noted, not to be built with the first version of the reach rule | It eased long launches in the record: landing ashore was used twice at Eastern Solomons. It needs air groups that can be away from their carrier, which the game does not have (rebasing, decision 0091, is built, but a group still cannot be away from its carrier and come back to it) | The strike-range research; added at the designer's request on Nov 12, 2026 |
| Daylight as a limit on a launch | A strike launched late must be recovered in the dark or not launched. Noted, not to be built with the first version | Dusk limited a launch as much as fuel did, and the longest reaches at Midway ended in a night recovery. Today a strike flies in the morning or afternoon pulse with no regard to the hour within it | The same |
| Experience for air groups | One number for an air group, read by a fighter's and a bomber's chance to hit and the chance of running out of fuel, diluted by replacements (decision 0064, stage 2). **It waits:** worked by hand from the rows, the facts are too thin to rest the values on, and no air group carries a value until research closes the gaps listed | The air figures are tuned meanwhile with every air group alike in experience, a named gap | Decision 0064, "Nov 13, 2026: stage 2 waits"; `docs/research/oob/air-group-experience-proposal.md` |

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

## Kept so they are not lost

- Speed made good. A force in the game holds its ordered speed on a straight course. In the record, where a source gives positions and times as well as a speed, a convoy or a ship in transit made good a half to three quarters of its stated speed: it zigzagged, waited, slowed for landfall or towed. Warships on a combat run are the exception (the 6th Cruiser Division from the Shortlands to off Savo Island on 11 October 1942 took what its speeds give for the distance). Noted by the designer on Nov 12, 2026 and not for step 2 (`docs/step2-spec.md`, "Decided by the designer", item 8): taken up when convoys and supply are compared with the record. The simplest answer may be a cruising speed for routine movement below a ship's stated speed, not a new mechanism. The facts are in `data/oob/passages.csv` and `docs/research/oob/passages.md`.
- Before step 3: the staff closing the range on a contact it cannot reach; choices rules make inside a day pulled into functions the planner can ask (decision 0055, "Left for later"); objectives that name a flow (`docs/design/economy.md`); basic ground combat (stage 4; stage 3 is built, decision 0092), and timing a trial of a plan by hand (`docs/design/planning.md`).
- With step 2 or 3: coastwatchers and reconnaissance as standing sources (`docs/design/intelligence.md`, layer 2).
- For step 3, named by the American losses at the Eastern Solomons (`docs/validation/tuning/r3-us-losses-named-misses.md`, ruled Nov 13, 2026): bombers' return fire at fighters (deferred since decision 0027); a search that meets enemy fighters; an aircraft written off for combat damage; a carrier's deck closed for a time, with aircraft aloft running out of fuel while it is.
- For step 3, named by Rabaul round 2 (`docs/validation/tuning/rabaul-r2-fatigue.md`, ruled Nov 13, 2026): a strike of a given size, so that a staff can send part of a group as the units at Rabaul sent about 18 bombers a raid whatever their rest; in the rules a group sends every aircraft it can fly.
- For step 3, named by the closing legibility test (`docs/validation/closing-tests.md`, finding L6, ruled Nov 13, 2026): a side shown an enemy strike on its ships with its causes hidden is told nothing of why it came; the briefing could say that an enemy strike always rests on a contact the enemy held, without saying which.
- For step 3, the same test, L7: a task force's mission is shown in the briefing and read by no rule; show it only when a rule reads it, or say there that it is a label.
- For step 3, named by Rabaul round 3 (`docs/validation/tuning/rabaul-r3-wear.md`, ruled Nov 13, 2026): small airfields that wear aircraft faster (left unbuilt by decision 0035), for the American losses on take-off, landing and ferrying that do not go with distance; and a wear that does not rise in a straight line with distance, since the dive bombers' searches and Rabaul's raids want the distance term opposite ways.

## Done, and recorded nowhere else

- The scenario with every rule of step 2 acting together (`data/scenarios/16_all_together.toml`, `tests/scenario_16_all_together.rs`; `docs/step2-spec.md`, "Tests while figures move", 8): built on Nov 13, 2026, before any figure is tuned. One night (a night action of unequal crews, one force not ready) and one day (a hold, a strike beyond the radius, a deck lost with aircraft away, a landing at a field and a ditching, the afternoon raid on that field, the day's end count), on an altered copy of the frozen figures. It is step 2's complexity review of the rule cards.

What step 2 did is in `docs/validation/step2-close.md`, the decision records (`docs/decisions/README.md`) and git.
