# 0051: The MCP server and scenario 9

Status: confirmed by the architect, within the plan for the agent interface the designer approved on Nov 11, 2026. He chose there that scenario 9 is tested by a recorded game and a scripted client, not by a live model in the test run, and that the model plays scenario 5 and also five days of the full slice.

## The gap

A side can be played through `sim-cli serve` (decision 0050), in a protocol of our own. The spec asks for an MCP server, so that any model that speaks that protocol can play without glue written for it, and for scenario 9: a model plays the Allied side of scenario 5 through the MCP server from start to finish with no human help, and the saved game replays to the same hashes.

## Decisions

### The server

1. **`sim-cli mcp` is the game as an MCP server over standard input and output.** It holds one game, with one seat held by the player and the other by the script. It is a thin wrapper: every tool call goes to the session of decision 0050 and comes back unchanged.
2. **It offers the nine tools and nothing else.** Each has a description written for a player who has read nothing else, and an input schema taken from the schema files. The server's instructions say in a paragraph what the game is, what a day is, and the order to work in: read the briefing, ask, measure, give orders, end the day.
3. **A tool's answer is text.** `get_briefing` answers with the briefing as text unless asked for data; the others answer with their JSON. An answer that is an error is marked as one in the protocol's own way.
4. **No resources, no prompts, no sampling, no network.** Tools only.

### Playing without a client

5. **`sim-cli play` makes one call to the MCP server and exits,** so that a player with nothing but a command line can play. It keeps the game as a file of the requests made so far. Each time, it starts the MCP server afresh, sends the requests of the file and then the new one, prints the new answer, and adds the request to the file. The game is the same each time because the game is deterministic: the same requests give the same game.
6. **A game is therefore a file of requests,** and that file is what is kept. From it come the transcript and the saved game, and it can be replayed through the server at any time.

### Scenario 9

7. **A model plays two games, alone:** the Allied side of scenario 5, which is one day, and the Allied side of the first five days of the full slice, so that reading, ordering and ending the day really repeat. It is given the tools' own descriptions and one instruction: command the Allied side to the end of the game. Nobody answers its questions or corrects its orders.
8. **What is kept of each game:** the file of requests, the transcript, the saved game, and the model's own account of what it did and why, as it gave it. They are committed as the record of scenario 9.
9. **The test of scenario 9 runs with no model.** It replays each kept file of requests through the MCP server and asserts that every answer is the one in the transcript, that the saved game is the one kept, and that `replay` of that saved game gives the same daily hashes. A second test drives the MCP server through the protocol with a scripted client: initialise, list the tools, call each, play a game to the end.
10. **The model's play is not judged.** Scenario 9 proves that a model can play through the interface and that what it did is recorded and repeats. Whether it played well is a matter for step 2 and the staff planner.
11. **What the record cannot show is said plainly:** the model was run as an agent with a command line and was told to use only `sim-cli play`. The transcript shows everything the game gave it. It does not prove the model read nothing else on the machine.

### Later, and not without the designer's word

12. **Adding the server to the project's own configuration,** so that the designer can open the game from Claude and play a side himself, is proposed to him with the file to be added, and added only when he says so.

## Why

- **A thin wrapper** keeps one game and one set of tools. Anything the MCP server did differently from `serve` would be a second interface to keep true.
- **A game as a file of requests** is the determinism rules paying off: nothing has to stay running between a model's calls, and nothing about the game lives outside a file that can be read.
- **A recorded game in the tests** is the only form of scenario 9 that can run every time and give the same answer. A live model would make the test run slow, costly and different each time.

## What it costs

- `sim-cli play` replays the whole game for each call. It is milliseconds for these games and would be slow for a long one; a client that holds the server open has no such cost.
- The recorded games are tied to the engine as it was. A change that alters a result means the model's requests no longer give its transcript. The test then fails, rightly, and the games are played again or the record is marked as of an older engine.
- One model, two games, is an anecdote and not a measurement.

## Rule card

No rule of the game is added.

## As built (stage 9e)

Settled by the architect while building, inside the approved plan.

- **Two forms of the protocol.** The specification's current revision, 2026-07-28, has no `initialize`: each request carries the protocol version and the client's capabilities in `params._meta`, and `server/discover` stands for the handshake. The revisions before it open with `initialize`. The specification lets a server serve both, and this one does, choosing by how a request opens. Read on Nov 12, 2026 from `modelcontextprotocol.io/specification/versioning` and `/specification/2026-07-28/basic`. Where decision 1 above says `initialize`, read "either form". `sim-cli play` uses the older form inside itself.
- **Input schemas.** `issue_orders` takes its input schema from the schema files of `sim-agent`, with every reference replaced by what it points at, since a client cannot fetch a neighbouring file. The other eight are written in `mcp.rs`, because the schema files describe answers and not arguments. `measure` is one flat object with a `question`, because some clients refuse a choice of shapes at the top of a tool's input.
- **The briefing is text** unless asked for as data: a call with no `text` is made as `{"text": true}`, and the transcript shows it so.
- **Errors.** A call the game refuses (a bad handle, a bad order document) is a tool answer marked as an error, and is kept in the transcript and the game file. A call the protocol refuses (a tool that does not exist, arguments that are not an object) is a protocol error and is kept in neither. `play` prints a refusal on standard error and exits with 1.
- **The game file** is a first line naming the scenario, the seat and the seed, then one line a call. `play --tools` prints every tool as the server lists it; `--export-save` and `--export-transcript` write the saved game and the transcript from the file.
- **The server asks nothing of the client** (no sampling, no elicitation) and offers tools only.
- **A reason with the orders.** `issue_orders` takes an optional `reason`: the player's own words for why it gives these orders. It is kept in the transcript and the game file, where a reader finds it beside the orders. The game never reads it and it is not in the saved game, so it cannot change a result or a hash. It is the first piece of "why did the player do that" (the engine's own staff will answer the same question through an order's origin and intent, in step 3).
- **What the tools tell, after the first two games a model played** (scenarios 5 and 7, Nov 12, 2026). Words and fields only; no rule, result or hash changes. The server's instructions and the `issue_orders` description now say what each air mission does and that missions must be set before the contact appears (a strike is decided and flown inside the day), the arc convention (0 is north, clockwise, multiples of 10, range at most the type's), that the game has no score and ends after its last day, and what `pilots: null`, a base size of 0 and an arrival's day mean. The briefing's heading gives the last day (the session hands it in; a view does not hold it), a force line and a sixth warning say that a task force cannot move or is slowed, the text calls enemy units in raids and attacks "enemy" (they are still "not known" to `get_entity`, decision 0047, 8), and `issue_orders` counts what was queued and refused. Rule card: reads the side's view and the number of days the session was started with; writes nothing.

