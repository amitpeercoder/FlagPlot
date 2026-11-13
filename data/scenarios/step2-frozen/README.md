# Frozen scenarios of step 2

Copies of scenarios as they stood when their hand calculation was written, the counterpart of the frozen figures in `data/reference/step1` (`docs/step2-spec.md`, "Tests while figures move", 9; decision 0073).

- **What is here:** `04_savo_island.toml`, scenario 4 as it stood on main on Nov 13, 2026, before the northern group's break-off level was set from the record (decision 0024, the note of Nov 13, 2026). The only difference from that file is that the paths to the map and the reference files are one folder deeper, and a comment at its head.
- **Who reads them:** only hand-calculation tests (`tests/scenario_10b_objectives.rs` reads scenario 4's copy, for its Savo Island scoring). They load it on the frozen figures. The doc comment of every test that reads a copy here says it runs on the frozen copy. Nothing that plays the game, no batch, no baseline and no export reads it; `data/scenarios/*.toml` does not match it, so the scenario loops of CI and the tests skip it.
- **Limits:** one frozen copy for a scenario and no more. A copy changes only with a rule change's new hand calculation, written by the architect. Nothing remakes it otherwise, and a change to the live scenario's data does not reach it.
- **What a test on it proves:** that the rules compute what the hand calculation says, on known orders and known figures. It says nothing about the live scenario: the run baselines (`crates/sim-cli/tests/run_baseline`) and `validate` cover that.
