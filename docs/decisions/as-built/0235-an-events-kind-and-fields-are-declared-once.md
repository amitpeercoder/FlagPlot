# 0235, as built: An event's kind and fields are declared once

## Amended on review (Nov 27, 2026, pull request 745)

An independent reading of the record as first written said "must change". What changed, and why:

1. **Two tests read the enum from the source** (ruling 13, P3, P4, pull request 1). `event_kind_names` in `side_events.rs`'s tests and `event_kinds` in `tests/scenario_07_determinism.rs` look for the text `pub enum EventKind {`, which the macro removes, and the record as written forbade the mend. Both now read `EventKind::KINDS`, on the architect's word. The second is under `tests/`, so pull request 1 waits for items 2 and 3 of the programme. The trial of the macro had built the library only, and the record now says so.
2. **The gain was overstated.** 145 of the 179 kinds are fired in the 127 exports; 34 are not, and under ruling 2 keep their arms. The summary, the index and `docs/open-work.md` say about 110 to 120 arms and some 1,400 lines, where they said 140 and 1,700. No proof is added for the kinds not fired.
3. **Ruling 2 gains a third condition.** Identical bytes do not show a branch of an arm that no event took. An arm goes only if it has no such branch beyond the type's one reading, or every side of each is taken in the "before" exports, shown by a script.
4. **P3 names `crates/sim-core/src/lib.rs`,** for the `mod` line of a new file.
5. **Pull request 4 names its three scenarios.**
