# 0013: Random number keys for a pair of entities

Status: confirmed by the designer, Nov 10, 2026, as part of the scenario 3 plan.

## The gap in the spec

The spec keys each random draw on seed, day, pulse, subsystem and one entity ID. Search needs a draw for a pair: this air group looking for that task force. Keyed on the group alone, the result for one target would depend on which other targets were in the arc. Keyed on the target alone, two groups searching for it would always get the same result.

## Decision

- A key may name a second entity. The key is then seed, day, pulse, subsystem, entity and second entity.
- A key with no second entity gives exactly the numbers it gave before. The five pinned values in `rng.rs` do not change, so saved games still replay.
- A key with a second entity folds it in after the first, with its own tag constant. New pinned values are added for this form.
- The order matters and is fixed by the rule that draws: for search, the air group is the entity and the task force is the second entity.

## Consequences

- Adding or removing a target does not change the result for any other target. Adding or removing a searching group does not change the result for any other group.
- A rule that needs several numbers for one pair takes them from that pair's stream in a fixed order, as decision 0003 says for one entity.

## Rule card

No rule. This changes how random numbers are keyed and nothing else.
