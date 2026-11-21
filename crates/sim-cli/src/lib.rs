//! The parts of the command-line runner that a test may build a game with
//! as the runner builds one: what plays a seat's trials (decision 0178), the
//! opening of a game for play and the saved game (decision 0182). The runner
//! itself is `main.rs`.

pub mod estimates;
pub mod game;
pub mod save;
pub mod seat_trial;
