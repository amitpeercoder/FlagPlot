# A model in a seat: the proof of stage 2b, item 17 (decision 0138)

Played on Nov 17, 2026, by the team lead of the proof of stage 2b (`docs/validation/stage-2b-proof.md`). Reported and not judged.

**The game.** `data/scenarios/stage-2b-proof/05a_eastern_solomons.toml` (the Eastern Solomons from both sides' intents, command levels on, the live report figures), seed 1, the Allied side, the seat at `task_force:1` (Task Force 16, Enterprise): `sim-cli mcp data/scenarios/stage-2b-proof/05a_eastern_solomons.toml --seat allied --at task_force:1 --seed 1`. The staff served every other Allied command and the whole Japanese side, from the intents in the file.

**The model.** A Sonnet model (`claude-sonnet-5-5`), run as `claude -p --model sonnet --tools "" --strict-mcp-config` with the game's server as its only tools, given `prompt.txt` and nothing else.

**This folder holds no `game.jsonl`,** so `regame.py` and the `mcp` tests do not replay it: neither carries a seat's `--at`. What it holds:

- `prompt.txt`: everything the model was told.
- `calls.jsonl`: the model's calls, as its client sent them (tool and arguments).
- `transcript.jsonl` and `save.json`: the calls played again through `sim-cli mcp` with `--save` and `--transcript` (the same command with those two options), since the server the client started was stopped before it could write them. `sim-cli replay save.json --check-staff` passes.
- `account.md`: the model's account at the end, verbatim.
- `first_attempt_account.md`: the account of a first attempt whose calls were not kept (the server was stopped by the client before writing them); kept as written, not the game reported, and not verifiable.

**What it was shown, ordered and got.** It called `get_briefing` once, `issue_orders` once and `end_day` once. The briefing gave its own force and the five air groups aboard Enterprise, numbered from 1 in the seat, all at rest (the staff's orders are given when the day is resolved), no contact, no base and no objectives ("none given to this seat"). It ordered VS-5 and the first VT-3 group to search, VB-6 and the VT-3 attack group to the naval strike mission, and VF-6 to the combat air patrol with a share of 0.7; its two search arcs were refused ("range_nm" must be text) and it ended the day without reading the refusals, so its two search groups flew nothing. Each order it gave held the group against the staff, so Enterprise's groups did not search as the staff's carrier doctrine has them do. Its force held one contact all day, Henderson Field's report of the carrier striking force at (8,-11), 223 nm away and out of reach, and its strike groups launched nothing. The Japanese strikes put Enterprise's flight deck out in the morning, as in the staffed game of the same seed (where it happens in the afternoon). The game is one day long, so the model saw nothing after `end_day`; no result is given to a seat that is not the area's.

**Was it given anything its command did not hold?** No. Against the export's pictures: `task_force:1`'s picture at the start is empty, and the briefing said "Contacts held: none"; the briefing named only task force 1's own ships and groups; no other force, base or contact of the side appears in anything it was given.
