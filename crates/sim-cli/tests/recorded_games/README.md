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
