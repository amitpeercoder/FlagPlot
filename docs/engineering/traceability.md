# Traceability rules

What lets every outcome say what caused it. Moved out of `CLAUDE.md` on Nov 12, 2026, word for word, to keep that file short. `CLAUDE.md` states each rule that fails silently in one line and points here for the rest and the reasons.

Every outcome must be able to say what caused it. This cannot be added later.

- **Every state change records an event.** If a rule changes truth state and writes no event, that is a bug.
- **Every event names its causes:** the orders and earlier events that led to it. Follow the pattern in `resolve.rs`: movement cites the standing orders, and fuel burn and arrival cite the movement event.
- **Every computed quantity or chance records a breakdown** of its factors when `SimConfig::record_breakdowns` is on. Breakdown recording must never change what happens or the state hash.
- **Every order carries its origin:** who issued it and the intent it serves.
