# Legibility test of orders inside a day (decision 0156, condition 6 of its proof)

- `cases.json`: the six days, A to F, each with one order given inside it: the game, the order given and the answer the game gave to it (for the record), and `material`, the seat's events of day 1 as the `events` tool gave them at dusk, cut to the number, pulse, kind and words of each. Made by `crates/sim-agent/tests/orders_inside_proof.rs`, `condition_6_the_six_days` (`PROOF_OUT=folder cargo test --release -p sim-agent --test orders_inside_proof condition_6 -- --ignored`), on the live figures.
- `prompt.txt`: the words each reader was given before its case's events.
- `key.md`: the answer key, written by the team lead from the material alone before any reader ran.
- `answers.md`: the answers of six Haiku readers, one case each, word for word.
- `marks.md`: the marks, by a Sonnet model against the key, checked by the team lead.

Nothing here is read by a test or a build step.
