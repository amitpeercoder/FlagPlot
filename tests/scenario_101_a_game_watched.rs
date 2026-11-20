//! Scenario 101: a game watched. Two commanders are at the seats and a
//! third party, the watcher, holds the clock. He steps it, reads what a
//! commander reads, and can order nothing. The game played is the game
//! the same orders give with no watcher.
//!
//! Proves decision 0182. There is no arithmetic in it: the record changes
//! who moves the clock and nothing that happens when it moves. What
//! follows is stated by the architect before the code, and each statement
//! is a test. If the code disagrees, the code or the rule is wrong: a
//! mismatch goes back to the architect.
//!
//! It runs on scenario 78, orders inside a day, as that scenario's own
//! test runs it, through a session as `sim-cli` opens one. No new data.
//!
//! # The game it is measured against
//!
//! **The plain game:** scenario 78 with the Allied seat played, in which
//! the seat gives the orders scenario 78's hand calculation has it give,
//! at the moments it gives them, by `advance` and `end_day`. Its daily
//! hashes and the answers of its calls are the measure.
//!
//! # Statements
//!
//! 1. **The same orders give the same game.** A watched game with the
//!    Allied seat played and the Japanese seat the script's, in which the
//!    seat makes the plain game's calls in the plain game's order and the
//!    watcher steps whenever the seat is ready: the same daily hashes,
//!    and each of the seat's calls is answered as in the plain game, byte
//!    for byte, but that a call of time is answered when its time has
//!    passed.
//! 2. **Both seats played, the same game.** As 1, with the Japanese seat
//!    played and giving, at each dusk, the orders the script gives it in
//!    scenario 78, then `end_day`: the same daily hashes.
//! 3. **No step past a seat that has not finished.** At the start a
//!    `step` is refused, naming both seats. With the Allied seat ready
//!    and the Japanese not, it is refused naming the Japanese seat, and
//!    the clock has not moved.
//! 4. **Each seat's stop is its own.** The Allied seat calls `advance`
//!    of one pulse and the Japanese seat `end_day`. After one step the
//!    Allied call is answered and the Japanese call is not; a second
//!    step is refused naming the Allied seat; the Allied seat gives an
//!    order at 0600, which is taken in at the start of the morning, and
//!    calls `advance` of two pulses; after two more steps both calls are
//!    answered, at dusk.
//! 5. **A stop on what a side was told.** The Allied seat calls
//!    `advance` of three pulses to stop on the thing scenario 78's hand
//!    calculation has it stop on: its call is answered after the step of
//!    the pulse in which that happens, with the plain game's answer.
//! 6. **A held seat cannot call.** Between a seat's call of time and its
//!    answer, any other call of that seat is refused, saying that it is
//!    ready and waits for the clock; nothing is changed by it.
//! 7. **`run` and `advance` are offered to both seats** in a watched
//!    game, and with two seats played in a game not watched they are
//!    not, as before.
//! 8. **The watcher reads as the seat reads.** For each read tool of
//!    rule 10 the watcher's answer for a side is the seat's own answer
//!    at that moment, byte for byte.
//! 9. **The watcher can change nothing.** Each tool that orders,
//!    commands, previews, tries, acknowledges or moves time is refused
//!    to him by name, as a tool he does not have. After any number of
//!    his reads the hashes, each seat's transcript and the number of the
//!    seat's next preview are what they would have been.
//! 10. **He is told where the game stands:** the day and the moment,
//!     each played seat as ready until a named stop or waited for, and
//!     at the end that the game is over. A scripted seat is always
//!     ready.
//! 11. **He is shown what a seat did:** the seat's calls since its last
//!     stop, each with its tool, its arguments and whether it was
//!     refused; a call's `reason` where it gave one.
//! 12. **At the game's end** every held call is answered, with the
//!     result where the tool gives one; a `step` is refused, the game
//!     being over.
//! 13. **Saved and replayed.** The saved game of 2 replays, with no
//!     watcher, to the same daily hashes.
//! 14. **A day at a step where a scenario has no orders inside a day:**
//!     on scenario 5 watched, with the Allied seat played, a step
//!     resolves a day, `advance` is not offered, and five steps give the
//!     plain game's hashes.
//! 15. **`serve --watched`,** in a test of `sim-cli`: a held call's
//!     answer line comes after the line of the step that made it due,
//!     with the call's own `id`; a request that is none of a seat's
//!     call, a step or a watcher's read is answered with an error and
//!     changes nothing; and without `--watched` a step and a watcher's
//!     read are refused.
//! 16. **A game not watched is as it was:** the recorded games replay
//!     as they did, and every tool listed for them is one listed before.

// The tests are written by the build, to the statements above.
