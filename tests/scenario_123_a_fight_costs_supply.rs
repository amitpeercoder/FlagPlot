//! Scenario 123: a fight costs supply. A day's fight charges each side's
//! bases in the hex for its men at grips, ammunition first and then
//! supply, and a side that cannot be supplied fires short.
//!
//! Proves decision 0209. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The setting
//!
//! Scenario 122's, in a file of its own: a placeholder scenario on a
//! placeholder map, with no real unit. Its ground figures are in a file of
//! its own, the frozen step 1 figures with eight added: the three of
//! decision 0205 (`rations_landed_nights` 3, `hungry_below_share` 0.6,
//! `hunger_nights_before_loss` 14), the two of decision 0208
//! (`hunger_weakness_per_night` 0.01, `unfed_fighting_share` 0.9), and
//! `fight_supply_t_per_man` 0.01, `unsupplied_fire_share` 0.25 and
//! `fight_days_landed` 4. In the frozen figures the daily supply for a
//! man is 0.005 t, `men_per_shot` is 100, the limit on engaged men is out
//! of reach (`engagement_ratio` 1,000,000) and no unit's weight or skill
//! changes its shots. Every land unit is standard, with no fatigue.
//!
//! Side A has a base Q, with no aircraft, and a land unit V of 1,000 men
//! ashore there, on defence. The men's charge at Q is 5 t a night. Q's
//! supply and ammunition at the start are what each statement gives; the
//! ammunition is nothing unless it says. Side B has a land unit E of
//! 1,000 men in Q's hex, set to attack, with no base there.
//!
//! The first night comes before the first fight, as in scenario 122. On
//! it V is fed in full wherever Q starts with 5 t or more, and E is fed
//! nothing: E's count is 1, its condition 0.99, its share from the meal
//! 0.9, and its firing men are 1,000 times 0.891, 891. V's firing men are
//! 1,000. Every statement below is of the first fight, on day 1, unless
//! it says otherwise.
//!
//! "Shots" are a unit's shots as its fire records them, after the limit
//! on engaged men and after this rule.
//!
//! # Statements: the charge
//!
//! 1. **Supplied in full.** Q starts with 20 t of supply. The night takes
//!    5 and leaves 15. V's men at grips are 1,000 and its want is 1,000
//!    times 0.01, 10 t. All of it is charged, from supply: Q has 5 t
//!    after the fight's charge. The supplied share is 1 and the fire share
//!    1, and V's shots are 1,000 over 100, 10.
//! 2. **Supplied by half.** Q starts with 10 t. The night leaves 5. The
//!    want is 10 t and 5 are charged; Q has nothing. The supplied share
//!    is 0.5, the fire share 0.25 plus 0.75 times 0.5, 0.625, and V's
//!    shots are 6.25. V's firing men are still 1,000.
//! 3. **Nothing to fire with.** Q starts with 5 t. The night takes it all
//!    and V is fed in full. The want is 10 t and nothing is charged. The
//!    supplied share is 0, the fire share 0.25, and V's shots are 2.5.
//!    The charge is still recorded for A: 10 t wanted, none charged.
//! 4. **A side with no base.** In each of statements 1 to 3, E's men at
//!    grips are 891 and its want 8.91 t. B has no base in the hex, so
//!    nothing is charged. E's fire share is 0.25 and its shots are 8.91
//!    times 0.25, 2.2275.
//! 5. **Ammunition first.** Q starts with 20 t of supply and 6 t of
//!    ammunition. The night leaves 15 t of supply. The want of 10 t takes
//!    the 6 t of ammunition and 4 t of supply: Q has no ammunition and 11
//!    t of supply. The supplied share is 1 and V's shots are 10. The
//!    record gives 6 t from ammunition and 4 t from supply.
//! 6. **Ammunition alone.** Q starts with 5 t of supply and 4 t of
//!    ammunition. The night takes the supply. The want is 10 t and 4 are
//!    charged, from ammunition. The supplied share is 0.4, the fire share
//!    0.25 plus 0.75 times 0.4, 0.55, and V's shots are 5.5.
//! 7. **No fight, no charge.** With E not in the scenario, Q starting
//!    with 20 t of supply and 6 t of ammunition has 15 t and 6 t after
//!    day 1, and 10 t and 6 t after day 2. Nothing is recorded of a
//!    fight's charge.
//!
//! # Statements: the limit on engaged men
//!
//! The same scenario on a ground file whose `engagement_ratio` is 0.5.
//!
//! 8. **Only the men at grips are paid for.** The limit for A is 0.5
//!    times E's 1,000 fighting men, 500, below V's 1,000 firing men: A's
//!    men at grips are 500 and its want 5 t. Q starts with 20 t of
//!    supply: 5 are charged and Q has 10 t after the fight's charge. V's
//!    shots are 10 times 500 over 1,000, 5, times a fire share of 1: 5.
//! 9. **The cut is on the shots after the limit.** The limit for B is 0.5
//!    times V's 1,000, 500, below E's 891 firing men: B's men at grips
//!    are 500 and its want 5 t, of which nothing is charged. E's shots
//!    are 8.91 times 500 over 891, 5, times 0.25: 1.25.
//!
//! # Statements: what a landing brings
//!
//! Scenario 119's landing: side A's force T with a land unit U of 3,000
//! men aboard, and a hex P where A has no base. No enemy is at P.
//!
//! 10. **Days of fighting, as ammunition.** When T lands its first man at
//!     P the base formed has 45 t of supply, the rations of decision
//!     0205, and 3,000 times 4 times 0.01 t, 120 t, of ammunition. The
//!     record of the ammunition names 3,000 men, 4 days and 120 t, and
//!     cites what the rations' record cites.
//! 11. **Every landing brings them.** A second force with 1,000 men
//!     landing at P on a later day brings no rations and 1,000 times 4
//!     times 0.01 t, 40 t, of ammunition: P has 160 t of it.
//! 12. **Men do not eat it.** With all of U ashore at P and nothing else
//!     landed, the three nights after the landing are fed in full and the
//!     fourth is not; P's ammunition is 120 t throughout.
//!
//! # Statements: a dry run, and where it runs
//!
//! 13. **A side a dry run makes up is taken as supplied.** With the
//!     world's setting that takes B as supplied, in statement 1: E's fire
//!     share is 1 and its shots 8.91, nothing is charged to B and no
//!     charge is recorded for B. A is charged as in statement 1.
//! 14. **Without the three figures,** on scenario 122's ground file, in
//!     statement 1's setting: V's shots are 10 and E's 8.91, Q has 15 t
//!     after the fight, nothing is recorded of a fight's charge, and the
//!     landing of statement 10 brings 45 t of supply and no ammunition.
//! 15. **The loader.** A ground file with one or two of the three figures
//!     and not all is refused. One with the three and without the three
//!     of decision 0205 is refused. One with the three and the three of
//!     0205 and without the two of 0208 is taken. A
//!     `fight_supply_t_per_man` or `fight_days_landed` below 0 is
//!     refused, and an `unsupplied_fire_share` below 0 or above 1.
//! 16. **The files.** `data/reference/ground_hunger.toml` gives
//!     `fight_supply_t_per_man` 0.01, `unsupplied_fire_share` 0.25 and
//!     `fight_days_landed` 4, and no figure of it that the live
//!     `ground.toml` has differs from it. Its frozen counterpart gives
//!     the three with the same values. Scenario 119's and scenario 122's
//!     own files are as they were.
//!
//! # Statements: what is recorded and who knows
//!
//! 17. **The order and the causes.** In statement 5 the record of A's
//!     charge comes after the fight's event and before any unit's fire,
//!     with Q's ammunition spent and Q's supply spent, each citing the
//!     charge; V's fire cites the charge. A's comes before B's.
//! 18. **Each side sees its own.** In statement 2, A is shown its charge
//!     and, in V's fire, a supplied share of 0.5 and a fire share of
//!     0.625. B is shown none of A's charge, A's stocks spent or those
//!     factors, and A none of B's.
//! 19. **The side's own count of firing men is not cut.** In statement 3,
//!     A is told that 1,000 of V would fire.
//!
//! # Properties
//!
//! - **No proof and no recorded game moves.** Every scenario and recorded
//!   game on file but the table's copies gives the events and the daily
//!   hashes it gave before.
//! - **No new state.** The state's layout version is what it was.
//! - **Nothing is drawn by the rule.** The same game played twice gives
//!   the same events, and in statement 14's setting the dice of the fight
//!   fall as they did before the change.
//! - **Stocks never go below nothing,** and what is charged in a fight is
//!   never more than what was wanted.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]
