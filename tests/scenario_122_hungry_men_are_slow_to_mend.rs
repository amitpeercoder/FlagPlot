//! Scenario 122: hungry men are slow to mend. A unit's condition falls
//! with each night of hunger and rises a night at a time once it is fed in
//! full, and last night's meal counts for half.
//!
//! Proves decision 0208. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The setting
//!
//! Scenario 119's, in a file of its own: a placeholder scenario on a
//! placeholder map, with no real unit. Its ground figures are in a file of
//! its own, the frozen step 1 figures with five added: the daily supply
//! for a man is 0.005 t, `rations_landed_nights` is 3,
//! `hungry_below_share` is 0.6, `hunger_nights_before_loss` is 14,
//! `hunger_weakness_per_night` is 0.01 and `unfed_fighting_share` is 0.5.
//! Every land unit has the standard weight and skills, and no fatigue
//! unless a statement gives it.
//!
//! Side A has a base Q, with no aircraft, and a land unit V of 1,000 men
//! ashore there at the start. The men's charge at Q is 5 t a night. Q's
//! stock is what each statement gives it. Side B has nothing ashore unless
//! a statement says so. "After night N" is as in scenario 119: read at the
//! start of day N's morning.
//!
//! The words: a unit's condition is 1 less its nights of hunger times
//! 0.01, not below 0. Its share from last night's meal is 0.5 plus 0.5
//! times its fed share. Its fighting share is the one times the other.
//! "Would fire" is what side A is told of its own unit: the men ashore
//! times the fighting share times 100 less the fatigue over 100, cut to a
//! whole man.
//!
//! "The rule of 0205" is the same scenario on a ground file with the
//! three figures of decision 0205 and without the last two.
//!
//! # Statements: the fall
//!
//! 1. **A night on nothing halves a unit and no more.** With no stock at
//!    Q, after the first night V's fed share is 0 and its count 1: its
//!    condition is 0.99, its share from the meal 0.5, its fighting share
//!    0.495, and 495 men would fire. By the rule of 0205 none would.
//! 2. **Each night takes a hundredth more.** After the fifth night the
//!    count is 5: condition 0.95, fighting share 0.475, and 475 would
//!    fire. After the fourteenth, 0.86 times 0.5, 0.43: 430.
//! 3. **Men still die as before.** On the fifteenth night V loses 50 men
//!    and has 950, as in scenario 119. Its count is 15: 950 times 0.85
//!    times 0.5 is 403.75, and 403 would fire.
//! 4. **Half a ration is hunger, and bites less at once.** With 2.5 t at Q
//!    the first night feeds 500: the fed share is 0.5, below 0.6, and the
//!    count is 1. The share from the meal is 0.75, the fighting share
//!    0.99 times 0.75, 0.7425, and 742 would fire. With 3 t the share is
//!    0.6, not below the mark, and the count stays at nothing: the share
//!    from the meal is 0.8 and 800 would fire. By the rule of 0205 they
//!    are 500 and 600.
//!
//! # Statements: the rise
//!
//! 5. **Five nights hungry, five nights to mend.** With no stock for five
//!    nights and then 5 t reaching Q on each of days 6 to 10, V is fed in
//!    full from the sixth night. After it the count is 4 and 960 would
//!    fire; after the seventh 970, the eighth 980, the ninth 990, and
//!    after the tenth the count is nothing and 1,000 would fire. By the
//!    rule of 0205, 1,000 would fire after the sixth.
//! 6. **A short night does not mend.** With no stock for five nights and
//!    then 3.5 t on day 6 and 5 t on day 7: after the sixth night the fed
//!    share is 0.7 and the count stays 5; the share from the meal is
//!    0.85, the fighting share 0.95 times 0.85, 0.8075, and 807 would
//!    fire. After the seventh the count is 4 and 960 would fire.
//! 7. **Past a hundred nights there is nothing to give back at first.**
//!    With V given a count of 120 at the start and 110 t at Q, V is fed in
//!    full every night. After the first night the count is 119 and no man
//!    would fire; after the twentieth it is 100 and none would; after the
//!    twenty-first it is 99, the condition is 0.01, and 10 would fire.
//!
//! # Statements: the function
//!
//! 8. **Fatigue is taken after.** 1,000 men with a fighting share of
//!    0.475 and a fatigue of 20: 1,000 times 0.475 times 0.8, 380 fire.
//! 9. **The cut is made once, at the end.** 333 men with a fighting share
//!    of 0.475 and no fatigue: 158.175, and 158 fire.
//! 10. **Work.** V is a fighting unit: with a fighting share of 0.475 the
//!     work it makes in a day is 1,000 times 0.475 times 1 times 2 over 2
//!     times 0.5, 237.5. A working unit of 400 men with a count of 5, fed
//!     in full, makes 400 times 0.95, 380. The staff's count of a day's
//!     work for each gives the same figure.
//!
//! # Statements: work, training and what a side is told
//!
//! 11. **A unit fed nothing still works, and is not told it was not
//!     fed.** With V a unit at work on Q's defences and no stock, on the
//!     first day V's work is 1,000 times 0.495 times 0.5, 247.5, and the
//!     briefing gives no reason it did nothing. By the rule of 0205 it
//!     makes none and is told "not fed".
//! 12. **Men who are mending do not train.** With V set to train, hungry
//!     for five nights and fed as in statement 5: V does not train on
//!     days 1 to 9. On each of days 6 to 9 it was fed in full and its
//!     count was above nothing, and the reason A is given is "mending".
//!     It trains on day 10. By the rule of 0205 it trains on day 6.
//! 13. **The fight reads the same share.** V is given a count of 5 at the
//!     start and no stock. Side B has a unit E of 1,000 men in Q's hex,
//!     set to attack, with no base there, so E is fed nothing. After the
//!     first night V's count is 6 and E's is 1. In the first fight V
//!     fires 1,000 times 0.94 times 0.5, 470 men, and E fires 1,000 times
//!     0.99 times 0.5, 495, each with no fatigue. The record of V's fire
//!     gives a fed share of 0, 6 nights of hunger, a condition of 0.94
//!     and a fighting share of 0.47, and A is shown all four. B is shown
//!     none of them for V, and A none of them for E.
//! 14. **A is told its unit's condition, and only where it is below
//!     one.** In statement 5 V's line gives a condition of 0.95 after the
//!     fifth night and 0.96 after the sixth, and gives none after the
//!     tenth.
//!
//! # Statements: where it runs
//!
//! 15. **With the three figures of 0205 and without the two,** every
//!     statement's "by the rule of 0205" holds, and V's line gives no
//!     condition.
//! 16. **With no hunger figures at all,** V with no stock fires none after
//!     the first night, loses 50 men that night, and is whole in its
//!     share the first night it is fed, as today.
//! 17. **The loader.** A ground file with one of the two figures and not
//!     the other is refused. One with the two and without the three of
//!     0205 is refused. One with either below 0 or above 1 is refused. 0
//!     and 1 are taken.
//! 18. **The files.** `data/reference/ground_hunger.toml` gives
//!     `hunger_weakness_per_night` 0.01, `unfed_fighting_share` 0.5 and
//!     `hungry_below_share` 0.6, and no other figure of it differs from
//!     the live `ground.toml`'s. Its frozen counterpart gives the two new
//!     figures with the same values. Scenario 119's own files are as they
//!     were, and its tests pass unchanged.
//!
//! # Properties
//!
//! - **No game on file moves.** Every scenario and recorded game on file
//!   but the table's copies gives the events and the daily hashes it gave
//!   before.
//! - **No new state.** The state's layout version is what it was, and a
//!   saved game of scenario 119 from before the change loads and replays.
//! - **Nothing is drawn.** The same game played twice gives the same
//!   events.
//! - **Each side sees its own.** B's view gives none of A's counts,
//!   conditions or fighting shares, in any event or answer.
//! - **One function.** The fight, the work, the side's count of firing
//!   men, the staff's count of work and the unit's line give the same
//!   fighting share for the same unit on the same morning, in statements
//!   5 and 13.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]
