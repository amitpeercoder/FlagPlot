//! Scenario 102: the enemy's objectives shown. Each side reads, in its
//! briefing, what the other side is scored for: the kinds, the places and
//! what each is worth. It reads nothing of the enemy's mission, progress
//! or state.
//!
//! Proves decision 0183. There is no arithmetic in it. What follows is
//! stated by the architect before the code, and each statement is a test.
//! If the code disagrees, the code or the rule is wrong: a mismatch goes
//! back to the architect.
//!
//! It runs on existing scenarios, through a session as the other tests of
//! the agent's interface open one. No new data.
//!
//! - **ES:** `05_eastern_solomons.toml`: both sides have objectives, none
//!   naming a base.
//! - **FS:** `07_full_slice.toml`: both sides have objectives, some naming
//!   a base (supply landed at Lunga, Lunga open, Lunga and Tulagi held).
//! - **T:** `01_transit.toml`: no objectives.
//! - **CL:** `stage-2b-proof/05a_eastern_solomons.toml`: command levels
//!   on, and objectives.
//!
//! # Statements
//!
//! 1. **The enemy's objectives are listed, as the scenario gives them.**
//!    On FS, at the Allied seat on the evening before day 1: the
//!    briefing's objectives part has, after the side's own objectives,
//!    the Japanese side's, one for one and in the scenario's order, each
//!    with its kind, the base it names where it names one (by the map's
//!    number and name), whether a base is to be open or shut, and its
//!    worth (points; points a ton; points for each kind of ship), equal
//!    to the scenario file's own figures. On FS the Japanese objectives
//!    name Lunga alone, and one is that it be shut. And at the Japanese seat, the Allied
//!    side's, likewise.
//! 2. **What a seat is shown of the enemy's is what the enemy is shown
//!    of his own,** but for progress: for each objective, the kind, the
//!    base, open or shut, and the worth in the Allied briefing's list of the Japanese
//!    objectives equal those in the Japanese briefing's list of its own.
//! 3. **No progress and no state.** An enemy's objective in a briefing
//!    is of a form that has no field for a count, days counted, points
//!    earned, a basis or "open now"; the briefing's schema says so. Played to day 5 on FS with the script's orders handed in for both
//!    sides, so that Lunga changes side on day 1, with fights and landings, the enemy's
//!    list in each side's briefing is, byte for byte, what it was on the
//!    evening before day 1.
//! 4. **The enemy's mission is not shown.** No briefing, as text or as
//!    data, holds any sentence of the other side's mission; and two
//!    worlds that differ only in the enemy's mission give the same
//!    briefing, byte for byte (a case in the agent's source test).
//! 5. **A base is named by the map's number and name.** On FS, in the
//!    Japanese briefing, the Allied objective of supply landed names
//!    Lunga as the Japanese seat's own tools name that base, before it
//!    changes side and after.
//! 6. **The text says whose they are, from the reader's side.** The
//!    briefing as text has the side's own objectives and the line of the
//!    levels as before, then a line "The enemy is scored for:" and the
//!    enemy's, each in words written from the reader's side: "your ships
//!    sunk", "tons the enemy unloads at Lunga", "Lunga is the enemy's at
//!    the end", "days Lunga is shut". No line of the enemy's says "enemy
//!    ships sunk" or "you unload".
//! 7. **A scenario with no objectives shows none:** on T the briefing,
//!    as text and as data, is what it was before this record, byte for
//!    byte but for the version of the agent's interface.
//! 8. **A seat at a command is shown none:** on CL the briefing at a
//!    task force's seat has no enemy objectives, as it has none of its
//!    side's.
//! 9. **A side with objectives whose enemy has none** is told so in one
//!    line. Tested on a copy of ES made by the test in which the
//!    Japanese side keeps its table and its mission and has no objective
//!    entries: the data has the part, empty, and the text the line.
//! 10. **Nothing else changed.** On ES and FS the daily hashes are what
//!     they were; the staffs give the orders they gave; the result at
//!     the end is what it was.
//! 11. **The answers are checked against their schemas,** and the
//!     briefing's schema names the new part.

// The tests are written by the build, to the statements above.
