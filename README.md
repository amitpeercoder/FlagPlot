# Flag Plot

An operational wargame of the Pacific War where the player commands by intent and a staff executes.

The name comes from the compartment on a flagship where the admiral and staff tracked the battle and issued orders.

## Status

The game is built in five steps. Step 1, a headless, deterministic simulation core, is complete: all nine of its scenarios pass. Step 2, which checks the simulation against the historical record and tunes its figures, was closed on Nov 13, 2026 (`docs/validation/step2-close.md`). The spec of step 3 is confirmed; nothing of it is built. There is no game to play yet.

- **What runs today.** Ships steam, burn fuel and carry cargo. Aircraft search, strike ships and bases, fight in the air, wear out and are repaired. Forces fight at night on the surface. Each side knows the enemy only through sighting reports and claims, which can be wrong. A scripted or an AI player can play a scenario through `sim-cli`.
- **What is real.** The ships and aircraft of Savo Island, the Eastern Solomons and the four weeks over Guadalcanal are historical, with ratings derived by a written rule from sourced facts in `data/oob`. The map of the South Pacific, `data/map/south_pacific.csv`, is real geography, generated from public-domain Natural Earth coastlines by `crates/map-tool`.
- **What is a placeholder.** Every combat, wear, fatigue, supply and claim figure, and every point value, until step 2 sets it from the record. *(Nov 14, 2026: step 2 closed with most of them still placeholders; `docs/validation/tuning/`.)* The small test scenarios use invented ships and maps, each named "Placeholder".
- **What step 1 left out.** Ground combat, land units, shore bombardment, ship repair and base damage, among others. Decision 0052 has the list. *(Nov 14, 2026: since built: land units and ground combat, decisions 0066, 0090 and 0094; shore bombardment, decision 0092; ship repair, decision 0069. Base damage is not built.)*

## Documents

- [Design pillars](docs/pillars.md): what the game is and why.
- [The rules](docs/rules/): every rule as the code has it, in the order a day resolves. [Glossary](docs/glossary.md): one meaning for each word.
- [Decisions](docs/decisions/): each design choice and its reason.
- [Step 1 spec](docs/step1-spec.md) and [step 2 spec](docs/step2-spec.md).
- [Commands](docs/commands.md): what each command does.
- [CLAUDE.md](CLAUDE.md): the rules the code follows, and where everything else is.

## Running it

You need Rust. The toolchain version is pinned in `rust-toolchain.toml`.

```
cargo test --workspace
cargo run -p sim-cli -- run data/scenarios/01_transit.toml --events
```

The second command runs scenario 1 and prints the state hash of each day, the full event log with what caused each event, and where every ship ended up.

To run a historical battle, give it `data/scenarios/04_savo_island.toml` or `data/scenarios/05_eastern_solomons.toml`.

`viewer/` holds a read-only replay viewer for debugging: `sim-cli export` writes a run as one file and `viewer/index.html` draws it on the map. See [viewer/README.md](viewer/README.md).
