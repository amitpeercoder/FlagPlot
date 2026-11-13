# Truth and belief

The boundary between the real world of the game and what a side thinks. Moved out of `CLAUDE.md` on Nov 12, 2026, word for word, to keep that file short. `CLAUDE.md` states each rule that fails silently in one line and points here for the rest and the reasons.

- Truth state is the real world. Belief state is what one side thinks.
- A side's own units are known exactly. Enemy units reach a side only as contacts.
- The only way to change a belief state is `BeliefState::apply_report`. Keep it that way: it is what makes "belief is written only from reports" true.
- Never add a method to `SideView` that returns enemy truth.
- The event log is truth. A side gets its events only through `SideView::events()`, which gives each event whole, cut down or not at all (decision 0047). The rule is one exhaustive `match` in `side_events.rs`: a new kind of event does not compile until someone decides who may see it.
- A breakdown is cut with its event, and a whole event's breakdown is shown whole. Before giving an event a breakdown, ask whether a factor says something of the enemy that the event itself does not. A claim's breakdown does (how much of the claim is real), so a side is not shown it (decision 0060). The same question is asked of an event's own fields: a search that sighted is cut of its distance and chance, which say where the force really was (decision 0061).
- No event a side is given names an enemy ship, task force or air group, in a field or in its text (decision 0065). A new kind of event that names one gets a cut form without it. An enemy base may be named, and so may the target a side wrote in its own order.
- Nothing a side is given may change with something it cannot see: not a reason, an hour, a round number or a flag (decision 0085). A fight's ending is told as the side's own force made it, a fight is told from the round the side first knew of it, and the flag of hidden causes is set by the kind of event, never by the event. A new kind of event is added to that kind table. A breakdown never carries a raw draw. Event, order and contact numbers are opaque outside sim-core and read only with the `UmpireKey`, which only a `World` gives; a side's view counts its own, and a side's units are shown numbered among its own (decision 0086). Never give these ids a public way to read or make their number, or `std::hash::Hash`.
- A `SideView` method that takes a unit takes its number and looks it up among the side's own units, never a value the caller built (decision 0085): the view cannot check a value it is handed.
- Anything the engine says when it fails is the umpire's: a player is told one fixed sentence (decision 0085). A saved game holds both sides' orders and is given to a player only when the game is over.
