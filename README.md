# Flag Plot

An operational wargame of the Pacific War where the player commands by intent and a staff executes.

The name comes from the compartment on a flagship where the admiral and staff tracked the battle and issued orders.

## Status

Step 1 of five: a headless, deterministic simulation core. Three of its eight test scenarios are built, and scenario 4 is in progress. There is no game to play yet.

The ships in the repo are placeholders, invented so the engine has something to run. They are not historical. The map of the South Pacific slice, `data/map/south_pacific.csv`, is real geography, generated from public-domain Natural Earth coastlines by `crates/map-tool`; the small maps the early test scenarios use are placeholders.

## Documents

- [Design pillars](docs/pillars.md): what the game is and why.
- [Step 1 spec](docs/step1-spec.md): the simulation core.
- [Decisions](docs/decisions/): design choices the spec left open.
- [CLAUDE.md](CLAUDE.md): the rules the code follows.

## Running it

You need Rust. The toolchain version is pinned in `rust-toolchain.toml`.

```
cargo test --workspace
cargo run -p sim-cli -- run data/scenarios/01_transit.toml --events
```

The second command runs scenario 1 and prints the state hash of each day, the full event log with what caused each event, and where every ship ended up.

`viewer/` holds a read-only replay viewer for debugging: `sim-cli export` writes a run as one file and `viewer/index.html` draws it on the map. See [viewer/README.md](viewer/README.md).
