# Truth and belief

The boundary between the real world of the game and what a side thinks. Moved out of `CLAUDE.md` on Nov 12, 2026, word for word, to keep that file short. `CLAUDE.md` states each rule that fails silently in one line and points here for the rest and the reasons.

- Truth state is the real world. Belief state is what one side thinks.
- A side's own units are known exactly. Enemy units reach a side only as contacts.
- The only way to change a belief state is `BeliefState::apply_report`. Keep it that way: it is what makes "belief is written only from reports" true.
- Never add a method to `SideView` that returns enemy truth.
- The event log is truth. A side gets its events only through `SideView::events()`, which gives each event whole, cut down or not at all (decision 0047). The rule is one exhaustive `match` in `side_events.rs`: a new kind of event does not compile until someone decides who may see it.
