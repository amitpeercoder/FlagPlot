# Determinism rules

What keeps the same start, orders and seed giving the same state hash on any machine. Moved out of `CLAUDE.md` on Nov 12, 2026, word for word, to keep that file short. `CLAUDE.md` states each rule that fails silently in one line and points here for the rest and the reasons.

The same starting state, orders and seed must give the same state hash on any machine. Breaking any of these rules breaks replays.

- **No floating point in `sim-core`.** Use `fixed::Fixed` for every quantity. `clippy::float_arithmetic` is denied in the crate.
- **No floats on the way in.** Data files give quantities as whole numbers or decimal strings such as `"0.25"`. The loader rejects TOML floats.
- **Ordered collections only.** `BTreeMap` and `BTreeSet`, never `HashMap` or `HashSet`. Clippy rejects the hash types across the workspace.
- **No wall clock, thread timing or locale.** Clippy rejects `SystemTime::now` and `Instant::now`.
- **All randomness through `World::rng`.** Each draw is keyed on seed, day, pulse, subsystem and entity, so results cannot depend on processing order. Never share one stream between entities, and never carry a stream from one pulse to the next. A new rule that draws adds its own value to `rng::Draw`, at the end, and never uses `Draw::Legacy` (decision 0055).
- **A world built from a picture draws in its own domain.** `World::from_picture` takes a seed and a domain (`rng::Domain`: a trial's number and a world's number within it), and the domain is mixed into the key of every draw after everything else the key holds, and only where there is one. A live game has none, so every stream of a live game is byte for byte what it was. The world holds its domain; it is named in `hash::name_every_field` and written last in `hash::hash_world`, and only where there is one, so no live hash moves and `STATE_VERSION` was not raised for it. Nothing in `sim-core` passes a live world's seed to a built one; only the umpire's rebuild (`World::rebuild_from_picture`, which asks for the umpire's key) builds on a live seed with no domain (decision 0170, 12 and 13).
- **Never renumber stable codes.** `Subsystem::code`, `Pulse::code`, `EntityRef::kind_code`, `Side::code` and the like feed the random number keys and the state hash. Add new values at the end.
- **The random number generator's output is pinned** by a test in `rng.rs`. If that test fails, saved games no longer replay. Do not update the pinned numbers without a decision record.
- **New state goes into the hash.** When a field is added to the state, add it to `hash::hash_world` in the same change. `hash::name_every_field` stops the build until the field is named there. A change to what is hashed, or its order, raises `hash::STATE_VERSION` and remakes the golden files in the same change; no tag bytes (decision 0057).
- **Units made in play are numbered from their side's series.** A task force made by a detach and an air group formed by a landing take the next number of their own side's series (Allied from 1,000,001, Japanese from 2,000,001), one counter for each side and kind, kept in the state and hashed. The loader refuses a scenario unit numbered 1,000,000 or above. Draws keyed on a made unit then depend on its own side's history only (decision 0086).
- **Overflow panics in every profile.** `overflow-checks` is on for release builds. Do not turn it off.
- **No `unsafe`.** It is forbidden workspace-wide.

## The browser's build (decision 0229, stage 3)

The same game is built for `wasm32-unknown-unknown`, where `usize` is 32 bits and a panic ends the module. `web/check.mjs` compares it with the native build, call for call and day for day. What is kept so that the two agree:

- **No unstable sort that depends on the size of what it sorts, where equal does not mean the same.** `clippy.toml` disallows `sort_unstable_by`, `sort_unstable_by_key` and the `select_nth_unstable` family (path `slice::...`); `sort_unstable()` on whole values is allowed.
- **A cast to `usize` from a 64-bit value is read.** None is made in the crates the module holds; a number that crosses from a call is read through `u32` first (`take_back`'s `number`).
- **Overflow checks are on in the `web` profile**, so an overflow is a panic in both builds and never a wrapped number in one.
