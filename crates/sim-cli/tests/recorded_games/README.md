# Recorded games (scenario 9)

Each directory here is one game played through the MCP interface by a player
other than a test (decisions 7 to 11 of `docs/decisions/0051-mcp-server-and-scenario-9.md`).
A directory holds:

- `game.jsonl`: the game as `sim-cli play` keeps it. The first line names the
  scenario, the seat and the seed; each later line is one tool call, `{"tool":
  .., "arguments": ..}`, in the order made.
- `transcript.jsonl`: every call with the whole answer the game gave, one JSON
  line each (`sim-cli play --game game.jsonl --export-transcript ...`).
- `save.json`: the saved game, which `sim-cli replay save.json` runs with no
  player (`--export-save`).
- Optionally the player's own account of what it did and why, as it gave it.

They are made by playing with `sim-cli play --game game.jsonl --scenario S
--seat allied TOOL ARGS` once for each call (`sim-cli play` alone prints how),
then exporting the save and the transcript from the game file.

The test `every_recorded_game_replays_to_its_transcript_and_save` in
`crates/sim-cli/tests/mcp.rs` replays each `game.jsonl` through `sim-cli mcp`
and asserts that every answer is the transcript's, that the regenerated
transcript and save equal these files byte for byte, and that `replay` of
`save.json` succeeds. A directory needs no registration: dropping the files in
is enough. A change to the engine that alters a result makes the test fail,
rightly; the game is then played again or the record marked as of an older
engine. The model's play is not judged here, only that it repeats.

## The games here

Played on Nov 12, 2026 (decision 0051, "The games"): `eastern_solomons_sonnet`, `eastern_solomons_haiku` and `full_slice_five_days_sonnet`. Each has the player's `account.md`. The requests of an earlier round, played before objectives and claims existed, are in `docs/research/scenario-9-first-round/` and are not replayed.

The transcripts were regenerated after the wording pass that followed these games (decision 0051, "The wording pass after the second round"), so they show the answers the tools give now and not word for word what the players read; the requests (`game.jsonl`) and the accounts (`account.md`) are as played. The games themselves did not change: the same orders were accepted and refused and the daily hashes are the same, and each `save.json` differs from the one played only in the hash of the scenario file, which changed when the missions of scenarios 5 and 7 gained a sentence on where the enemy carriers are expected.

The transcripts and saves were regenerated again after decision 0058 (ships are charged straight-line miles, and the state fingerprint is at version 3). The requests (`game.jsonl`) and the players' accounts (`account.md`) are as played, so an account describes the game as it ran before that decision: in the full slice game the convoy's passage and hour of arrival, the tons it landed (746.7 t then, 783.3 t now) and the fuel at Henderson Field differ from what the player wrote, and a briefing's distance for the dry destroyer is 208 miles, not 240. The same orders were accepted and refused. In the two Eastern Solomons games every answer is the same and only the saves differ.

The transcripts and saves were regenerated after decision 0059 stage 1, which added the reach in force to what `measure` answers. The requests and accounts are as played; every `issue_orders`, briefing, list and `end_day` answer is as before, and only the `measure` answers and the saves' hashes differ.
