# Frozen scenarios of step 2

Copies of scenarios as they stood when their hand calculation was written, the counterpart of the frozen figures in `data/reference/step1` (`docs/step2-spec.md`, "Tests while figures move", 9; decision 0073).

- **What is here:** `05_eastern_solomons.toml`, scenario 5 as it stood on main with decision 0063's orders (Nov 12, 2026), before decision 0073 set the Japanese search from the record. The only difference from that file is that the paths to the map and the reference files are one folder deeper.
- **Who reads it:** only hand-calculation tests (`tests/scenario_05_eastern_solomons.rs`). They load it on the frozen figures. The doc comment of every test that reads a copy here says it runs on the frozen copy. Nothing that plays the game, no batch, no baseline and no export reads it; `data/scenarios/*.toml` does not match it, so the scenario loops of CI and the tests skip it.
- **Limits:** one frozen copy for a scenario and no more. A copy changes only with a rule change's new hand calculation, written by the architect. Nothing remakes it otherwise, and a change to the live scenario's data does not reach it.
- **What a test on it proves:** that the rules compute what the hand calculation says, on known orders and known figures. It says nothing about the live scenario: the run baselines (`crates/sim-cli/tests/run_baseline`) and `validate` cover that.
