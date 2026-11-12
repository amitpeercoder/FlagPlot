# 0055: Claims from the air

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
