# 0060: A claim's breakdown is the umpire's

Status: confirmed. The designer approved the change on Nov 12, 2026. It amends decision 0047, item 10, and decision 0056.

## The gap

A side learns of its own attacks only as claims, and a claim can be wrong (decision 0056). But a side was shown the claim's breakdown, and the breakdown says how much of the claim is real.

- Decision 0047, item 10, gives a side the whole breakdown of an event it sees whole. A side sees its own `ClaimReceived` whole.
- Decision 0056 makes the breakdown of a raid's claim from `real_shot_down`, `fighter_misses`, `gun_misses`, `false_shot_down`, `real_hits`, `missed_bombs`, `false_hits`, `real_sunk`, `burning`, `bombs`, `real_on_ground`, `false_on_ground` and the three chances.
- So `explain` on a claim told the side the enemy's true losses. In the recorded Eastern Solomons game played by Haiku, the Allied side asked about a claim of 30 aircraft shot down and was told 18 were real and 12 were false.

Breakdown recording is on by default, so every player had this. The fog over claims was only as deep as one question. The two records were each right alone and wrong together: nobody decided that a side should know the truth of its claims. An outside review of the rulebook found it on Nov 12, 2026.

## Decisions

1. **A side is shown no breakdown of a claim.** `ClaimReceived` stays whole for the side that made it: the hex, the counts claimed, the kind and whether it is from the air. Its breakdown is absent for the side.
2. **No factor is kept.** A cut that kept the side's "own" factors would keep nothing safe:
   - the real counts are the enemy's real losses;
   - the false counts are the claim less the real counts;
   - the misses are the side's shots less its real hits, and a side is already not shown its hits (decision 0047 cuts them from its own salvo, air fight and air attack);
   - the chances are figures of the data files and tell a side nothing about this claim;
   - the result is the claimed counts added, which the event already holds.
3. **The causes stay.** The claim still cites the side's own firing events it rests on, each cut as decision 0047 cuts it. `explain` on a claim answers "which of my units fired" and no longer answers "how much of it was true".
4. **The log keeps the breakdown.** The umpire's tools show it as before: `sim-cli events`, the replay viewer, and the validation harness, whose claims measure (claimed over actual) reads the truth.
5. **The rule for every other event stands.** Decision 0047, item 10, is otherwise unchanged. A new kind of event whose breakdown would say something of the enemy that the event itself does not is cut the same way, and that is decided when the event is added.

A claim from a surface fight carries no breakdown, so nothing changes for it.

## Why

- A claim is the one event whose whole purpose is to differ from the truth. Its breakdown is the difference.
- "Every number can be explained" and "enemy truth stays hidden" meet here. The explanation a side is owed stops at what the side could know: its own units fired, and its people reported this. How the report came to be wrong is the umpire's.
- Decision 0056 says a claim is never below what really happened, and that this bounds the truth from above. That bound is all a side is meant to have.

## What it costs

- The legibility test of decision 0051 counted it as a pass that the Haiku player read "a claim's breakdown into real kills and false claims". It was reading something it should not have had. What the test showed about `explain` on the search chance and the attack still holds. The test is run again on the tuned Eastern Solomons in step 2 (`docs/step2-spec.md`, "Done means"), on this rule.
- A player can no longer learn from the game how unreliable claims are. The MCP server's instructions already say that claims can be wrong and are never below the truth.

## Rule card

| | |
| --- | --- |
| Reads | The log's `ClaimReceived` event and the side asking |
| Writes | Nothing. No state, no event, no random draw, no change to the hash |
| Where | `side_events.rs`: `side_breakdown` |

## As built

- `side_breakdown` returns no breakdown for a whole `ClaimReceived`.
- Tests: `a_claim_is_whole_to_its_side_and_its_breakdown_is_not_shown` in `side_events.rs`, and `crates/sim-agent/tests/claims.rs` asserts that `explain` on every claim of scenario 5 has no breakdown and names no real or false count.
- The Haiku game's transcript was regenerated: one `explain` answer lost its breakdown. No other answer, no save and no hash changed.
- Found on the way: the breakdown of a search that sighted gives the true distance to the force, which can differ from the distance to the hex the report names. It is row 40 of `docs/rules/differences.md`.
