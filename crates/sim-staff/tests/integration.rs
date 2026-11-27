//! Every integration test of this package, as one test program (see `Cargo.toml`).
//! Each file is a module; the files are unchanged but for how they reach `support`.

mod available;
mod enters;
mod holds;
mod lines;
mod notices;
mod radio_silence_and_held;
mod source;
