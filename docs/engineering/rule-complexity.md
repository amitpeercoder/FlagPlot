# Rule complexity

What keeps the rules something a person and a model can both reason about. Moved out of `CLAUDE.md` on Nov 12, 2026, word for word, to keep that file short. `CLAUDE.md` states each rule that fails silently in one line and points here for the rest and the reasons.

The game must stay something a person and a model can both reason about. A rule set that is correct but cannot be followed is a failure.

- **One mechanism over several.** A new rule reuses or replaces an existing mechanism where it can. Two limits on the same quantity need a reason, written in the decision record.
- **Every decision record that adds a rule carries a rule card:** the state the rule reads and the state it writes. A rule that reads what many other rules write is where interactions hide.
- **Every scenario test has property tests beside its hand calculation.** They state what must always hold: more of a helpful thing never makes the outcome worse, and quantities that should be conserved are conserved.
- **The legibility test, from scenario 3 on.** A cheap model is given only one side's briefing and the event log, and is asked to explain an outcome and predict the next day. If it cannot, the rules or the events need simplifying before the next scenario starts.
- **Complexity review after scenario 5,** the first scenario where several subsystems meet. The rule cards are read together and anything nobody can explain is cut or merged.
- **The rulebook is kept true.** `docs/rules/` states every rule as the code has it. A pull request that adds a rule, changes one or removes one changes the rulebook in the same pull request:
  - a new rule gets its section, in the chapter of the phase it runs in, with the decisions it rests on, its "A side sees" line and its data keys, and its phase is added to the order of a day in `docs/rules/README.md` if it has one of its own;
  - a changed rule has its section rewritten to say what the code now does, and every other section that points to it or repeats a part of it is checked;
  - a removed rule has its section taken out, with the pointers to it, and is not left as history: the decision record keeps that;
  - a new term goes in the glossary, a term whose meaning changed is corrected there, and a term nothing uses any more is removed;
  - a row of `docs/rules/differences.md` that the change settles is removed, and a new difference found on the way is added.
  A change to a figure in a data file needs no change to the rulebook, which names figures by file and key. The reviewer of a pull request that touches `crates/sim-core/src/` checks the rulebook's diff beside the code's, and a pull request that changes a rule with no change to the rulebook is not merged.
