# 0010: Where scenario tests live

Status: settled.

## The gap in the spec

The spec's repo layout lists a top-level `tests` directory holding the scenarios as automated tests. In a Cargo workspace, a directory of tests needs a package to belong to.

## Decision

- `tests` is a workspace member, a package named `scenario-tests` with no code of its own.
- Each scenario is one file directly in `tests/`, registered as a test target in `tests/Cargo.toml`.
- Unit tests for a module stay inside that module in `sim-core`.

## Adding a scenario

1. Add the data file in `data/scenarios/`.
2. Add `tests/scenario_NN_name.rs`, with the hand calculation in its doc comment.
3. Register it in `tests/Cargo.toml` with a `[[test]]` entry.

## Toolchain

The Rust version is pinned in `rust-toolchain.toml`. A compiler upgrade is a deliberate change with its own commit, because lint behaviour and, in principle, code generation can change between versions.
