# 0057: The state hash at version 2, and results that say what they were run on

Status: settled by the architect. Neither change affects play.

## The gap

Two things stood between step 1 and step 2, which compares thousands of runs with the record.

- **The state hash kept old hashes alive.** A field added since scenario 5 was written only where it was in use, behind one of twelve tag bytes, so that a scenario without the new rule hashed as before. Each tag was a special case and a place for two states to share a hash. Decision 9 of 0055 said the next change would raise the version instead.
- **A batch summary did not say what it was run on.** Step 2 tunes figures by comparing one batch with another. Two files made from different data files, or by different rules, could not be told apart.

## Decisions

1. **The state hash is at version 2.** It opens with `flagplot-state-v2` (`hash::STATE_VERSION`). Every field of every unit is written, always, in one order. The twelve tag bytes are gone. An air group's place is a tag (1 at a base, 2 aboard a ship) and a number, as a ship's is.
2. **The golden files are remade in the same change:** the 53 run baselines and the saved games of the three recorded games. Only hashes changed in them. No outcome, event or hand calculation moved.
3. **The version is raised again whenever what is hashed, or its order, changes.** A new field is added in its place and the golden files are remade. No tag bytes.
4. **A batch summary and a replay carry `inputs`:** the version of the hash's layout, the hash of the scenario file, and each data file the scenario names with its hash. The two file hashes are what a saved game already records (decision 0043), made the same way. The version of the hash's layout is new here: a saved game does not record it.
5. **`batch --label TEXT` records the caller's own name for the engine that ran,** such as a revision. The engine does not know its own revision, and a file that held one would differ from build to build for the same run.
6. **Both file formats keep their version number.** `inputs` and `label` are additions; a reader that does not know them is not misled.

## Why

- **Now is when it is cheapest.** No stage is in flight, and step 2 has pinned nothing yet.
- **A scenario without a rule still runs as it did.** That was the useful half of the old arrangement and it is kept: a rule whose figures file is absent changes no outcome. What is dropped is the promise that the hash's bytes do not move.
- **Inputs, not a promise.** Decision 2 of 0055 says a saved game need not survive a change of rules. `inputs` does not make results comparable across rules; it lets a reader see that they are not.

## What it costs

- Every hash recorded before this change is void. A saved game from before fails to replay at day 1.
- Three tests said that an unused field left the hash alone. They now say the field is hashed.
- `inputs` does not name the code. Two batches with the same inputs and different rules look alike unless the caller passes `--label`. The step 2 harness is to pass it always.

## Left for later

- **Ships are still charged 40 miles a hex step** (decision 0040, "Deferred"). Charging a ship straight-line miles reopens how a force moves and where it is, so it needs a decision the designer confirms and a hand calculation. It is still to be settled before step 2 compares ship movements with the record, and is not part of this change.
