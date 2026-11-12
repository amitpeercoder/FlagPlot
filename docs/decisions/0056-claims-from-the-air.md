# 0056: Claims from the air

Status: confirmed. The designer approved the plan on Nov 12, 2026. The chances are placeholders set by the architect, to be set from the record in step 2.

## The gap

A side sees that it attacked and never what it did (decision 0047). After a surface fight it gets a claim of ships believed sunk (decision 0014, 16). After an air fight, a strike or a raid on a base it gets nothing. At the end of Eastern Solomons neither side can say whether a carrier was sunk or one aircraft shot down. Decision 0047 named the remedy and put it off. Objectives (decision 0054) make it needed: a side cannot see its own progress.

## Decisions

1. **One mechanism, the claim of decision 0014, made wider.** A claim is a report to the claimant's belief of what its own people believe they did. It goes in through `BeliefState::apply_report` like every report, holds no identity of an enemy unit, and never ages. A claim now says, besides ships believed sunk: hits believed scored on ships, the kind of ship believed attacked, aircraft believed shot down, and aircraft believed destroyed on the ground. Each is a count that is zero when the claim is not about it.
2. **One claim for each side for each raid.** When a raid has been resolved (fighters, anti-aircraft fire, the attack), the raiding side gets one claim and the defending side gets one, each only if it fired. A raid that found nothing makes no claim. The claim's event cites every firing event it rests on.
3. **Aircraft shot down.** A side claims every enemy aircraft its fighters and guns really shot down in the raid, and each whole shot of theirs that missed has a chance of being claimed as one more. Fighters (patrol and escort) use `fighter_false_claim_chance`; guns of ships and bases use `anti_aircraft_false_claim_chance`. So overclaiming grows with the number shooting, as it did.
4. **Hits on ships.** The raiding side claims every bomb and torpedo that really hit, and each whole one that missed has `bomb_false_claim_chance` of being claimed as a hit.
5. **Ships sunk from the air** follow the rule of decision 0014, 16, with its figure (`claimed_sunk_fire`): every ship of the attacked force that sank in the raid, and every one left with fire damage at or above the figure. A ship burning from an earlier raid can be claimed again; so it was.
6. **The kind of ship** in a strike's claim is the kind the contact was taken for when the strike was launched. A strike sent at a "carrier" claims hits on a carrier whatever was there. A surface fight's claim keeps no kind.
7. **Aircraft destroyed on the ground.** The raiding side claims every aircraft its bombs really destroyed at the base, ready or damaged, and each other whole bomb dropped has `bomb_false_claim_chance` of being claimed as one more.
8. **No claim for supply destroyed.** Aircrew could not count tons.
9. **Missed shots are counted in whole shots:** the shots of a firing event less its hits, cut to a whole number, never below zero. Each is one draw.
10. **The claim's draws are their own.** They come from a stream keyed for intelligence on the day, pulse and the first firing unit, and are made after the raid is resolved. No draw of the fight itself moves, so no battle result changes.
11. **A claim is never corrected in step 1.** If a ship claimed sunk is sighted again, the contact shows it. Setting claims against later intelligence belongs to a later step.
12. **The figures are the same for both sides for now.** Placeholders in `data/reference/air_combat.toml`: `fighter_false_claim_chance = "0.1"`, `anti_aircraft_false_claim_chance = "0.05"`, `bomb_false_claim_chance = "0.1"`.
13. **A side sees its own claims whole and never the enemy's** (decision 0047, 2, unchanged).
14. **Objectives progress reads claims.** A side's known count of enemy ships sunk, with their kinds where a claim has one, and of enemy aircraft destroyed is the sum of its claims, labelled as claimed. Ships crippled stay not observable: no claim says "crippled". The umpire's result at the end is still from the truth.
15. **The briefing gains "Claims yesterday"**, one line a claim, and `explain` on a claim shows the firing events of the side's own that it rests on.

## Why

- **A chance on each miss, not a multiplier.** A fixed multiplier can be divided out, and the fog becomes a formality. Counting misses also gives the right shape for nothing: twenty guns that hit nothing still claim something.
- **One claim a raid** keeps the log readable. A claim for each burst of fire would double the events of an air battle.
- **The contact's kind** costs nothing and carries a real effect: a force misread before the strike is misreported after it.

## What it costs

- A side that fires a great deal claims a great deal. A base's guns firing at every raid for a month claim aircraft steadily. Step 2 sets the chances against the record of claims and losses.
- A claim of ships sunk from the air can count the same burning ship twice.
- Every scenario with air combat gains claim events and claims in belief, so its daily hashes change. No loss, hit or movement changes; the test of that is in the scenario files.

## Rule card

| | |
| --- | --- |
| Reads | The raid's firing events (shots, hits, aircraft lost); ships sunk in the raid and their fire damage; the contact the strike was sent at; the three chances and `claimed_sunk_fire` |
| Writes | The claimant side's claims in belief; a claim event |

## As built

Built in three stages (11a aircraft shot down, 11b strikes, 11c what a side is told). Nothing above this line was changed.

### Shape

- `BattleClaim` (belief) gains `claimed_hits: u32`, `claimed_kind: Option<ShipKind>`, `claimed_shot_down: u32`, `claimed_on_ground: u32` and `from_air: bool`. `ClaimReceived` (the event, one kind, no new kind) gains the same five fields, so `side_events.rs` is unchanged: the claimant sees its claim whole and nobody else sees it (decision 13). A claim holds no id of an enemy unit; the existing test destructures every field of the struct to keep it so.
- `from_air` is a choice the decision did not state. Without it a claim of nothing (a side that fired and claims no aircraft and no hit) cannot be told from the surface fight's claim of no ships. A surface fight's claim sets it false and the other four fields zero or none, and reads, hashes and exports as before.
- The log line is `claim_text` in `events.rs`, shared with `sim-cli`'s end-of-run listing and, through `air_claim_words`, with the briefing: "Allied claim over (7,-5): 12 aircraft shot down" when it says aircraft shot down only, "Allied claim at (9,0): 5 hits on a carrier, 0 sunk" when it names a kind, "Japanese claim at (5,0): 2 aircraft shot down, 5 aircraft destroyed on the ground". Aircraft shot down are told, as none if need be, when the claim says nothing else of what was hit. A surface claim keeps "Allied claims 2 enemy ships sunk at (5,0)".

### Where and when

- All claims of a pulse are recorded at the end of the air combat step, after every raid has been fought and every damage and report of the step has landed, in order of the raids' first strike groups, the raiding side's claim before the defending side's. One claim a side a raid.
- A side fires, and so claims, if it has a firing event of its own in the raid: for the raiders their escorts' fight (`AirFight`) or an attack (`AirAttack`, `BaseAttacked`), for the defenders their patrol's fight, the ships' guns or the base's guns. A claim of nothing is made when a side fired. A raid that finds nothing makes none; neither does a raid whose escorts did not fire and whose bombers were all shot down before they attacked.
- The claim cites every firing event of the claimant's own that it rests on, in the order they were recorded, and nothing else: not the losses, not the damage, not the report.

### The stream

- Subsystem `Intelligence`, `Draw::Claim` (code 1, added at the end of `Draw` as decision 7 of 0055 requires of a new rule), day, pulse, entity = the claimant's first firing unit and other = the raid's first strike group. The first firing unit is `AirGroup(firer)` for fighters (escorts or patrol), `AirGroup(strike group)` for an attack, `TaskForce(force)` for ships' guns and `Base(base)` for a base's. The second part of the key is what makes it unique to the claim: one patrol group can be the first firing unit of the claims of several raids in a pulse. It cannot collide with the surface fight's use of the stream, which is keyed on two task forces.
- The draws, in order: one for each whole shot that missed in each firing event of fighters and guns, in event order, at that event's chance; then, for the raiding side only, one for each whole bomb or torpedo that missed a ship (hits on ships), or one for each bomb dropped on a base that did not destroy an aircraft (aircraft on the ground), at the bomb chance. Each is `roll_shots` on whole shots: one `chance` draw each.
- Real aircraft shot down are those actually taken off by the raid's firing events (the `AircraftLost` aircraft, after the pool limits), not the hits rolled. Missed shots are the whole shots (shots cut to a whole number) less the hits rolled, never below zero. A torpedo counts as a bomb for the chance. Aircraft really destroyed on the ground count ready and damaged aircraft together, as destroyed, not as hit.
- The kind is the first strike group's: the kind the contact was taken for when that group was launched, kept in the strike in flight (transient, not hashed). A raid whose groups were sent at contacts of different kinds claims the first group's.
- Ships sunk are the ships the raid's own attacks sank (distinct ships) plus the ships of the force attacked still afloat at the end of the step with fire damage at `claimed_sunk_fire` or more. The end of the step means fire from every raid of the pulse, so a burning ship is claimed by each raid on its force that pulse; a ship sunk by another raid is not afloat and is not counted twice.
- The claim's breakdown, when breakdowns are on: `real_shot_down`, `fighter_misses`, `fighter_false_claim_chance`, `gun_misses`, `anti_aircraft_false_claim_chance`, `false_shot_down`; for the raiding side on ships also `real_hits`, `missed_bombs`, `bomb_false_claim_chance`, `false_hits`, `real_sunk`, `burning`; on a base `bombs`, `real_on_ground`, `bomb_false_claim_chance`, `false_on_ground`. Its result is the aircraft shot down, hits and aircraft on the ground claimed, added. Recording it changes nothing (tested). Amended by decision 0060: the breakdown is in the log for the umpire and is not shown to the side that made the claim.

### The chances

`fighter_false_claim_chance`, `anti_aircraft_false_claim_chance` and `bomb_false_claim_chance` are in `data/reference/air_combat.toml` as decimal strings with the values of decision 12, and are fields of `AirCombatFigures`. The loader rejects a missing one, a float, and one outside 0 to 1; 0 and 1 are allowed. Tests set them to 0 and to 1 by running a scenario from a copy of the figures written under the build directory.

### The hash

A claim of the old form hashes as it did. A claim with any air figure (`from_air`, hits, a kind, aircraft shot down or on the ground) adds, after the side's claims, the tag 0xFC, its place in the list, its four counts and `from_air`. So every scenario with no air combat (1 to 4c, Savo Island, 5a) keeps every hash line and its baseline files are unchanged. The tag is kept as built: it was designed before decision 9 of 0055 (the next layout change to the hash raises it to v2 and drops the tags), and this stage is the stage in flight that decision mentions. The version bump follows as its own change. `hash::name_every_field` names the five new fields of a claim.

### What a side is told (stage 11c)

- Known progress. `ships_sunk`: the sum of the claims' ships sunk; the basis is `claims` if any claim sank a ship or names a kind (so a raid that claims none sunk still gives a count of 0), else `not_observable` as before; the points are for the claims that name a kind (ships sunk times the points for that kind), and `None` if none does, as for a surface claim. `aircraft_destroyed`: the aircraft claimed shot down and destroyed on the ground, with the basis `claims` once the side has any claim from the air (a count of 0 included), and points the count times the points for one. `ships_crippled` stays not observable: no claim says it. The result at the end is still from the truth.
- The briefing: the claims list of the losses part is labelled "Claims yesterday" in the text, a line for each, with the fields of the schema added (`claimed_hits`, `taken_for`, `claimed_shot_down`, `claimed_on_ground`, `from_air`). It stays in the losses part: it was not made a part of its own, so the briefing's shape is as it was. The strike reports under an attack no longer carry a claim from the air (the claim is listed once, under claims). `explain` needed no change: a claim is an event with causes, and its causes are the claimant's own firing events.
- The text of objective progress for `claims` says ships or aircraft "claimed" and "(claims can be wrong)". The MCP server's instructions say that a side learns of its own attacks only as claims, what they hold and that they can be wrong; they say a claim is never below what really happened, which is true and bounds the truth from above.
- The viewer's export needed one arm: `claim_received` in `sim-cli/src/export/events.rs` takes the new fields. A claim from the air adds `claimed_hits`, `claimed_shot_down`, `claimed_on_ground` and, if it has one, `claimed_kind`; a surface fight's claim exports as it did. The export of the state at the end (`export/state.rs`) still lists each claim's ships sunk only. `viewer/` was not touched.

### The property that did not hold

"The known count is never above the true count" (stage 10a) does not hold with claims that can be false. The decision asked for it to hold with the three chances at 0, but it does not hold even there for ships sunk: a ship left burning is claimed sunk (decision 5, the rule of decision 0014, 16), so a side that burned an enemy ship and sank none knows 2 of a true 0 (seed 1, Allied, scenario 5). What holds at the three chances 0, over seeds 1 to 200, is: for `aircraft_destroyed` the known count is never above the true count, and for `ships_sunk` the known count is the true count plus the ships claimed only because they were burning (the `burning` of the claims' breakdowns), exactly. The test of stage 10a was replaced by that one.

### Tests and files that changed because claims exist

- `tests/scenario_11_claims.rs` holds the hand calculation and the properties. The logs from before this decision are kept under `tests/claims_before` (the baselines of the day, copied) so that the test of "every event that is not a claim is what it was" can run for ever: it takes the claims out of each log, renumbers the rest and compares each line, text and causes, with the old.
- The scenario tests 5, 5b, 5c, 5d, 6, 6a, 6b, 6c, 6d1 and 6d2 walk the log of each pulse and name every event they expect. Each now leaves the claims from the air out of the events it walks, where it builds them (a filter at that one place, with a note). 6d1's comparison with 6a numbers the events by position and now takes the pilots lost from the numbers and leaves the claims, which both logs have, to leave the same gaps.
- `tests/scenario_10_objectives.rs`: the literal of scenario 5's day hash in its baseline follows the new hash; the test of what each side knows was rewritten for claims; the property above replaced the old one. `crates/sim-agent/tests/objectives.rs` no longer asserts a claim-based count is below the truth; it asserts it is the sum of the side's claims.
- Run baselines under `crates/sim-cli/tests/run_baseline` changed, both the summary and the events file where there is one, for 05, 05b, 05c, 05d, 06, 06a, 06b, 06c, 06d1, 06d2 and 07; for 08_soak the summary only. The briefing goldens of `sim-agent` changed for scenario 5 after day 1 and for the base raid on days 2 and 3: the claims, the objectives' progress, and the side's own event numbers, which now count the claims it sees.
- The before-the-claims comparison (`the_log_without_the_claims_is_the_log_from_before_decision_0056`) now covers the scenarios whose ships decision 0058 does not move (05b, 05c, 05d, 06, 06a). 05 and 07 are out of it, and their two fixtures are deleted: their carrier forces steam off the grid axes and decision 0058 moved them.
