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
//! It runs on existing scenarios, through a session opened by `sim-cli`'s
//! library on the live figures, as `serve` opens one. No new data.
//!
//! - **S78:** `78_orders_inside_a_day.toml`, three days, the Allied side
//!   alone, orders inside a day on.
//! - **ES:** `orders-inside-proof/05_eastern_solomons_on.toml`, one day,
//!   both sides, orders inside a day on.
//! - **T:** `01_transit.toml`, three days, orders inside a day off.
//!
//! # The games it is measured against
//!
//! **The plain game of a scenario:** that scenario with the Allied seat
//! alone played and no watcher, the seat making a fixed list of calls
//! that the test states. Its daily hashes and the answers of its calls
//! are the measure. For S78 the calls are those of scenario 78's hand
//! calculation.
//!
//! # Statements
//!
//! 1. **The same calls give the same game.** S78 watched, the Allied
//!    seat making the plain game's calls in order and the watcher
//!    stepping whenever the seat is ready: the same daily hashes, and
//!    each call answered as in the plain game, byte for byte.
//! 2. **Which seat speaks first changes nothing.** ES watched with both
//!    seats played, each giving one order at dusk and one at 0600: with
//!    the Allied seat calling first at each stop, and with the Japanese
//!    first, the same hash at the end and the same answers to each seat.
//! 3. **No step past a seat that has not finished.** At the start a
//!    `step` is refused, naming both seats. With the Allied seat ready
//!    and the Japanese not, it is refused naming the Japanese seat, and
//!    the clock has not moved.
//! 4. **Each seat's stop is its own.** On ES the Allied seat calls
//!    `advance` of one pulse and the Japanese seat `end_day`. After one
//!    step the Allied call is answered and the Japanese is not; a second
//!    step is refused naming the Allied seat; the Allied seat gives an
//!    order at 0600, taken in at the start of the morning, and calls
//!    `advance` of two pulses; after two more steps both are answered,
//!    the Allied first.
//! 5. **A stop on what a side was told.** On ES the Allied seat calls
//!    `advance` of three pulses to stop on a contact: it is answered
//!    after the step of the first pulse in which its side is told of
//!    one, with the answer the plain game of ES gives the same call.
//! 6. **A held seat cannot call.** Between a seat's call of time and
//!    its answer, any other call of that seat is refused, saying that it
//!    is ready and waits for the clock; the hash and the seat's queue
//!    are as they were, and the refusal is in the transcript.
//! 7. **A call of time refused for its arguments** (`advance` of no
//!    pulse) is answered at once, and the seat is not ready.
//! 8. **`run` beside a stop inside a day.** On S78 with both seats
//!    played: the Allied seat calls `run` of two days and the Japanese
//!    seat `advance` of one pulse, then at 0600 `end_day`. The Allied
//!    seat's orders queued at the first dusk go in then and not again;
//!    its call is answered at the second dusk with the plain answer of
//!    `run` of two days.
//! 9. **A held `advance` stopped by an interrupt** at a dusk is answered
//!    there, and the other seat's call is not.
//! 10. **`run` and `advance` are offered to both seats** in a watched
//!     game, and with two seats played and no watcher they are not, as
//!     before. A seat at a command has no `run`, watched or not.
//! 11. **The watcher reads as the seat reads.** For each of his reads,
//!     at dusk and at 0600, his answer for a side is the seat's own at
//!     that moment, byte for byte; the briefing is refused him at 0600
//!     as it is the seat.
//! 12. **The watcher can change nothing.** Each tool that orders,
//!     commands, previews, tries, acknowledges or moves time is refused
//!     him by name, as a tool he does not have. After any number of his
//!     reads the hashes, each seat's transcript and the number of the
//!     seat's next preview are what they would have been.
//! 13. **A seat's answers do not read the other seat's queue.** On ES
//!     with both seats played, at dusk: each read the Allied seat has,
//!     asked with the Japanese queue empty and again with it holding
//!     orders and a command, is the same byte for byte; and the same
//!     the other way. If one differs the build stops and reports it.
//! 14. **He is told where the game stands:** the day and the moment,
//!     each played seat as ready until a named stop or waited for, and
//!     at the end that the game is over. A scripted seat is always
//!     ready.
//! 15. **He is shown what a seat did:** its calls since its last stop,
//!     each with its tool, its arguments and whether it was refused; a
//!     call's `reason` where it gave one.
//! 16. **The step's answer** says the day and pulse resolved, where the
//!     clock stands and which seats' stops came.
//! 17. **At the game's end** every held call is answered, with the
//!     result where the tool gives one; a `step` is refused, the game
//!     being over. With the engine stopped on an error, made by the
//!     test, every held call is answered with the words of the stop.
//! 18. **Saved and replayed.** The saved game of 2 replays, with no
//!     watcher, to the same hash. A game left at 0600 is saved as
//!     stopped inside a day.
//! 19. **A day at a step where a scenario has no orders inside a day:**
//!     T watched with the Allied seat played, calling `end_day` each
//!     dusk: a step resolves a day, `advance` is not offered, and three
//!     steps give the plain game's hashes.
//! 20. **`serve --watched`,** in a test of `sim-cli`: a held call's
//!     answer line comes after the line of the step that made it due,
//!     with the call's own `id`; two due on one step come Allied first;
//!     a seat's tool definitions and opening words are answered; a
//!     request that is none of a seat's call, a step or a request of
//!     the watcher's is answered with an error and changes nothing; and
//!     without `--watched` a step and a watcher's request are refused.
//! 21. **A game not watched is as it was:** the recorded games replay
//!     as they did, and every tool listed for them is one listed before.

// The tests are written by the build, to the statements above.
