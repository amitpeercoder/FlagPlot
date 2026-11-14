//! Scenario 38: a side's men report an empty hex. Where a side holds an
//! estimate of the enemy ashore in a hex, has fighting men there, and the
//! enemy has no man there, it is told that its men find none and the
//! estimate is cleared. It is cleared on the day the last enemy man is gone
//! and not before; a side with no fighting men in the hex keeps its old
//! estimate, shown with its day.
//!
//! Proves decision 0110, items 10 and 11, against a hand calculation. The
//! data is placeholder data: see `data/scenarios/38_an_empty_hex.toml`. It
//! runs on the frozen step 1 figures.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. An estimate's size is drawn:
//! the calculation gives the men it is drawn on and the range it must fall
//! in, and the tests read it from the run. Everything else is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! `report_error_low` 0.5, `report_error_high` 1.5: a ground report's
//! estimate is from half to one and a half times the enemy's men ashore in
//! the hex, working men included. From the earlier rules:
//! `daily_supply_t_per_man` 0.005, `unfed_loss_share_per_day` 0.05,
//! `beaten_fatigue` 30, `march_days_per_hex` 3, `captured_stock_share` 0.5.
//!
//! In the ground combat phase the marches come first; then, hex by hex, the
//! fight, fatigue, a side beaten, its bases changing side and its men
//! falling back or surrendering; and last the ground reports of the hex. So
//! a report sees the hex as the day leaves it.
//!
//! ## The rule
//!
//! A ground report is made, as before, for a side with men ashore in a hex
//! where the enemy has men ashore. Besides that: where a side holds an
//! estimate for a hex, has men of a fighting unit ashore there, and the
//! enemy has no man ashore there, it is given a report that its men find
//! none, and the estimate is cleared. With no estimate held, no report is
//! made. Every estimate is shown with the day it was made; an opening
//! estimate is of day 0.
//!
//! ## Inputs
//!
//! A placeholder map. Every hex is a coast hex and clear. The game is 6
//! days.
//!
//! - P at (0,0), with a land link to R at (1,0).
//! - Q at (10,0), S at (20,0) and V at (30,0), with no land link.
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Japanese | P   | 1000 t |
//! | 2    | Japanese | R   | 100 t  |
//! | 3    | Allied   | P   | 1000 t |
//! | 4    | Allied   | Q   | 1000 t |
//! | 5    | Allied   | S   | 1000 t |
//! | 6    | Japanese | V   | 1000 t |
//! | 7    | Allied   | V   | 1000 t |
//!
//! | Unit | Side     | Kind     | Men | Hex | Posture |
//! | ---- | -------- | -------- | --- | --- | ------- |
//! | 1    | Japanese | working  | 300 | P   | defend  |
//! | 2    | Allied   | fighting | 500 | P   | attack  |
//! | 3    | Allied   | fighting | 100 | Q   | defend  |
//! | 4    | Allied   | working  | 100 | S   | defend  |
//! | 5    | Japanese | working  | 200 | V   | defend  |
//! | 6    | Allied   | fighting | 100 | V   | attack  |
//!
//! Opening estimates held by the Allied side: 400 Japanese men at Q, 400
//! at S and 300 at V. It holds none at P. The Japanese side holds none
//! anywhere. There is no order.
//!
//! ## At P: cleared on the day the last man is gone
//!
//! Day 1. The Japanese side has only working men at P: it is beaten without
//! a fight, base 1 becomes Allied, and unit 1 is set to fall back to R,
//! where base 2 is. It is still at P when the reports are made.
//!
//! | Day | Unit 1's men at P when the reports are made | The Allied report at P | The Allied estimate at P |
//! | --- | ------------------------------------------- | ---------------------- | ------------------------ |
//! | 1   | 300                                         | men: from 150 to 450   | that figure, of day 1    |
//! | 2   | 285, after the night's unfed loss of 15     | from 142.5 to 427.5    | that figure, of day 2    |
//! | 3   | 271, after a loss of 14                     | from 135.5 to 406.5    | that figure, of day 3    |
//! | 4   | none: it marched its third day and is at R, with 258 men | its men find none | none |
//! | 5   | none                                        | no report              | none                     |
//!
//! The estimate is not cleared on days 1 to 3, though the garrison is
//! beaten and leaving: its men are still in the hex. On day 4 one report
//! says the hex is empty. On days 5 and 6 the Allied side holds no estimate
//! for P, so no report is made.
//!
//! The Japanese side: on days 1 to 3 its men at P report the Allied men
//! there, 500, as a figure from 250 to 750, replaced each day. From day 4
//! it has no man at P and gets no report: it still holds its estimate of
//! day 3, shown as of day 3, to the end.
//!
//! The Allied side has no man at R and is told nothing of it.
//!
//! ## At Q: an old estimate of men who were never there
//!
//! The Allied side holds an opening estimate of 400 at Q, of day 0, and has
//! fighting men there. No Japanese man is at Q. On day 1 its men report
//! that they find none, and the estimate is cleared. No report is made
//! after.
//!
//! ## At S: working men do not report
//!
//! The Allied side holds an opening estimate of 400 at S and has only a
//! working unit there. No report is made on any day. At the end it still
//! holds 400, shown as of day 0.
//!
//! ## At V: a surrender
//!
//! Day 1. The Japanese side has only working men at V: it is beaten without
//! a fight and base 6 becomes Allied. V has no land link and unit 5
//! surrenders, its 200 men lost, before the reports are made. The Allied
//! side holds an opening estimate of 300 and has fighting men there: its
//! men report that they find none, and the estimate is cleared on day 1.
//!
//! # Properties
//!
//! - A report that a hex is empty is made only where it is true: on no day
//!   is a side told a hex is empty while an enemy man is ashore in it.
//! - A side is never told that a hex is empty where it has no fighting man.
//! - The enemy is told nothing of the report.
//! - With no estimate held anywhere, a scenario gives what it gave before,
//!   apart from the hash version. A scenario whose events change is listed
//!   for the architect before its baseline is remade.
