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

To put a recorded game on the viewer's map: `cargo run --release -p sim-cli -- export crates/sim-cli/tests/recorded_games/NAME/save.json --out /tmp/NAME.json`, then open `viewer/index.html` and pick the file. The replay is the umpire's view, truth and all.

## The games here

Played on Nov 12, 2026 (decision 0051, "The games"): `eastern_solomons_sonnet`, `eastern_solomons_haiku` and `full_slice_five_days_sonnet`. Each has the player's `account.md`. The requests of an earlier round, played before objectives and claims existed, are in `docs/research/scenario-9-first-round/` and are not replayed.

The transcripts were regenerated after the wording pass that followed these games (decision 0051, "The wording pass after the second round"), so they show the answers the tools give now and not word for word what the players read; the requests (`game.jsonl`) and the accounts (`account.md`) are as played. The games themselves did not change: the same orders were accepted and refused and the daily hashes are the same, and each `save.json` differs from the one played only in the hash of the scenario file, which changed when the missions of scenarios 5 and 7 gained a sentence on where the enemy carriers are expected.

The transcripts and saves were regenerated again after decision 0058 (ships are charged straight-line miles, and the state fingerprint is at version 3). The requests (`game.jsonl`) and the players' accounts (`account.md`) are as played, so an account describes the game as it ran before that decision: in the full slice game the convoy's passage and hour of arrival, the tons it landed (746.7 t then, 783.3 t now) and the fuel at Henderson Field differ from what the player wrote, and a briefing's distance for the dry destroyer is 208 miles, not 240. The same orders were accepted and refused. In the two Eastern Solomons games every answer is the same and only the saves differ.

The transcripts and saves were regenerated after decision 0059 stage 1, which added the reach in force to what `measure` answers. The requests and accounts are as played; every `issue_orders`, briefing, list and `end_day` answer is as before, and only the `measure` answers and the saves' hashes differ.

Regenerated again after decision 0059 stage 1b, where a patrol pilot shot down over its own hex is recovered or lost by chance. The two Eastern Solomons games differ only in the saves' data-file hash (no patrol pilots are counted there). In the full slice game seven briefings and one `explain` differ (pilots lost and the shots they cut); every `issue_orders`, `measure`, list and `end_day` answer is as before.

Regenerated after decision 0060, which stops a side being shown the breakdown of its own claim. One `explain` answer in the Haiku game lost its breakdown (the claim of 30 aircraft, which had said 18 were real and 12 false). That player's `account.md` describes the answer as it was. No other answer and no save changed.

Regenerated once more after decision 0059 stage 2, where aircraft in the air when their deck is lost land elsewhere or come down in the sea. The requests and accounts are as played. In the Eastern Solomons games the day's `end_day` answer and one briefing differ (aircraft that landed, groups formed, the aircraft lost); in the full slice game three briefings differ. Every `issue_orders`, list, `measure` and `explain` answer is as before, and the saves differ in hashes only.

Regenerated after decision 0061, which cuts the distance and the chance from a side's view of its own search that sighted. One `explain` answer in the Haiku game changed (the search of event 19: no distance, no chance, and the breakdown keeps the aircraft, the sweep width and the arc). That player's `account.md` describes the answer as it was. No other answer and no save changed.

Regenerated after decision 0059 stage 3, which took Flight 300's scripted arrival (air group 12) out of the full slice. In the full slice game eight briefings differ (the group and the arrival due on day 4 are no longer listed); every `issue_orders`, list, `measure`, `explain` and `end_day` answer is as before, and the save differs in hashes only. The requests (`game.jsonl`) never named air group 12, and the player's `account.md` does not mention Flight 300 or an arrival.

Regenerated after decision 0065, which stops a side's events naming an enemy ship, task force or air group, and makes the agent interface version 2. Every answer of every game differs in its `version`. Beyond that, the briefings after a day of fighting and the `explain` answers that named an enemy unit differ: "at the raid of enemy air_group:12" is "at a raid", an air attack names no ship, a search "sighted a force", and attacks from the air on one ship are one line. The players' `account.md` files describe the answers as they were. No save differs.

Regenerated after the briefing's air group line gained `would_strike` (decision 0063, decision 6): the contact a naval strike group would strike if the decision were made now. One briefing in the Sonnet game of the Eastern Solomons and two in the full slice game gained the words; the Haiku game's transcript and all three saves are as they were. The requests and accounts are as played.

Regenerated again after the scenario orders of decision 0063 ("As built (the scenario orders)", Nov 12, 2026): the scenarios' orders changed, so the games now meet Ryujo one hex nearer (in (7,-9), 160 miles from the American carriers) with her raid on Henderson Field flying, and the fleet carriers' dive bombers hold their strike for heavy ships. The requests (`game.jsonl`) and accounts (`account.md`) are as played: an account describes the game as it ran before, with Ryujo in (7,-10) 200 miles off.

Regenerated after decision 0067, which records every base at the end of each day and counts `base_open` by days and `cargo_landed` net of what was loaded back. Every answer carries version 3. Each side's event numbers move from day 2 on, because its own bases' counts are among its events; the objective lines say "open at the end of each day" and give days of days; the saves' hashes change, because the hash counts the log's events. The requests and accounts are as played.

Regenerated after decision 0064 stage 1, which adds a ship's experience to the state (the hash is at version 6). The requests and accounts are as played; every answer of every game is as before, and the saves differ in hashes only (the state layout string and the data-file hash of `surface_combat.toml`, which gained three figures; the games' scenarios are not scenario 4, so no result moved).

Regenerated after decision 0068 (a force not ready fires part of its guns). The transcripts are byte for byte as before; each `save.json` differs only in the hash of `surface_combat.toml` among its data files, which gained the key `unready_gun_share`.  Every answer and every daily hash is as before.

Regenerated after decision 0071 (a force not ready fires part of its torpedoes, Nov 13, 2026). The transcripts are byte for byte as before; each `save.json` differs only in the hash of `surface_combat.toml` among its data files, which gained the key `unready_torpedo_share`. Every answer and every daily hash is as before.

Regenerated after decisions 0075 (an aircraft type has a search radius of its own: the SBD-3's is 397, the TBF-1's 420) and 0073 (the Japanese search at the Eastern Solomons from the record: Chikuma's floatplanes search 120 to 180 to 330 miles, Tone's have no order). The requests and accounts are as played. In the two Eastern Solomons games the answers that show a group's search range give the new figures, and a refused search of 9,999 miles is refused against 397 miles, not 200; the opening briefing and the day's briefing differ. In the full slice game an order to search to 250 miles, refused on day 1 as played ("at most the aircraft's 200 nm"), is now queued, so that game no longer runs as the player saw it from day 1 on: eight briefings, one `get_entity`, two `issue_orders` and one `explain` answer differ. The player's `account.md` describes the game as it ran before.

Regenerated after decision 0077 (a derived class has the radar figure only for a surface-search set with a plan position indicator). Every transcript is byte for byte as before; each `save.json` differs only in the data-file hash of `ship_classes_eastern_solomons.csv`. No answer and no daily hash moved.

Regenerated after tuning round 1 (Nov 13, 2026), which set `experience_gun_strength` in `surface_combat.toml` to 0.5. Every transcript is byte for byte as before; each `save.json` differs only in the data-file hash of `surface_combat.toml`. No answer and no daily hash moved: no game here has a night surface fight between ships of unequal experience.

Regenerated after tuning round 2 (Nov 13, 2026), which set `surprised_target_factor` in `surface_combat.toml` to 2. Every transcript is byte for byte as before; each `save.json` differs only in the data-file hash of `surface_combat.toml`. No answer and no daily hash moved: no game here reaches a night surface fight with a surprised force.

Regenerated after the strike report was made to read the force after the damage (rulebook differences, row 7; Nov 13, 2026). Answers that show what the Allied side believes after an attack that sank a ship of the force attacked now differ: the contacts' counts and kinds. In the Eastern Solomons game one answer differs (the briefing after day 1); in the full slice game four (three briefings and a `list_contacts`, after days 4 and 5). No order and no request changed; the daily hashes from the first such attack on differ, because the contacts are in the state.

Regenerated after decision 0081 (Nov 13, 2026), which gave the American torpedo bombers of scenarios 5 and 7 new air groups and changed the scenarios' orders (groups 25 and 26 in scenario 5, 125 and 126 in scenario 7, taking aircraft from VT-3 and VT-8, whose ready aircraft fall from 14 to 7 and from 13 to 8). The requests (`game.jsonl`) and accounts are as played; every `issue_orders` answer is as before (the same orders were accepted and refused), but the games no longer run as the players saw them, and every transcript differs. In the two Eastern Solomons games the opening briefing and the group's `get_entity` list the new groups and the smaller VT-3 and VT-8; the day's `end_day` and briefing differ in the fighting (different patrol shots; the attack on the Enterprise is 2 aircraft, not 27, and she is no longer crippled, so the Japanese points for a crippled carrier are gone: the Haiku game ends in a stalemate where it was a minor defeat, the Sonnet game in a major victory where it was a minor victory), and two `explain` answers of the Haiku game differ. In the full slice game eight briefings list groups 125 and 126 and the smaller VT-3 and VT-8; from day 4 the briefings differ in the strike, the contact (now at (7,-9), not (7,-10)) and the renumbered events; one `list_contacts` and two `measure` answers differ (contact:1 is inside the strike radius, 160 and 200 miles with no ditch chance, where it was 200 and 223 miles, outside it); the day 5 briefing shows strikes that found nothing at the contact's place. The players' `account.md` files describe the games as they ran before. Each `save.json` differs in its data-file hashes and daily hashes.

Regenerated again after main (the strike report reading the force after the damage) was merged into decision 0081 (Nov 13, 2026). The Eastern Solomons game played by Sonnet and the full slice game differ from their 0081 versions only in the reports of the Ryujo detachment made after she sinks: they count the ships left afloat (3, then 2) and take their kind from those ships, where they had counted her. The game played by Haiku did not change. No answer to `issue_orders` differs.

Regenerated after the B5N2 exception's reason in `data/derivation/rules.toml` was corrected (decision 0081, C5; Nov 13, 2026): the header of `aircraft_types_eastern_solomons.csv` changed and no figure did, so only the data-file hash in each `save.json` changed. The transcripts are as they were.

Regenerated after decision 0082 (Rabaul against Guadalcanal: the raid days from the record, Nov 13, 2026), which the full slice takes for its days 1 to 14: Rabaul's bombers now raid only on days 5, 6, 9, 10 and 13, and no longer on days 1, 2, 3, 6, 9 and 12. The two Eastern Solomons games are byte for byte as before. In the full slice game seven briefings and one `explain` answer differ: there is no raid on Henderson Field on day 1 and the fights, losses and claims of the later days are those of the new schedule. The requests (`game.jsonl`) and the player's `account.md` are as played; the account describes the game as it ran before.

Regenerated after decision 0083 (a ship fires the tubes that bear on one side, then the other side's in her next torpedo round; with decision 0074 the class files gained `torpedo_tubes_one_side`). The state hash went to version 7. Every transcript is byte for byte as before. Each `save.json` differs only in its daily hashes and in the data-file hashes of the changed class files. No answer moved: no game here has a ship whose tubes on one side are fewer than her tubes.

Regenerated after decision 0079 (anti-aircraft fire and the attack: the live share of the guns' fire worked before the attack is 0, so a raid's bombers attack before the guns' hits come off them), on the scenarios of decisions 0081 to 0083. The requests are as played. Answers that report the day's air fighting differ: in the Eastern Solomons games played by Haiku 3 (a briefing and two `explain` answers) and by Sonnet 1 (a briefing); in the full slice game 3 briefings. Each `save.json` also has the new data-file hash of `air_combat.toml` and, from the first raid that meets guns on, new daily hashes.

Regenerated after decision 0080 (whom the patrol fires at: the live patrol splits its fire between the escorts and the bombers in proportion), on the scenarios of decisions 0081 to 0083 and with decision 0079. The requests are as played. Answers that report the day's air fighting differ: one briefing in the Eastern Solomons game played by Sonnet, none in Haiku's, and three briefings in the full slice game. Each `save.json` also has the new data-file hash of `air_combat.toml` and, from the first raid that meets a patrol on, new daily hashes.

Regenerated after decision 0087 (the patrol's fire before and after the attack: at the live share of 0.4, part of the patrol's fire at the bombers comes after they have attacked). The requests are as played. Answers that report the day's air fighting differ: in the Eastern Solomons games played by Haiku 2 (a briefing and an `explain`) and by Sonnet 1 (a briefing); in the full slice game 3 briefings. Each `save.json` also has the new data-file hash of `air_combat.toml` and, from the first raid that meets a patrol on, new daily hashes.

Decision 0086: handles are numbered per side (each side's own ships, task forces and air groups are 1, 2, 3... among its own, in the order of the scenario's numbers). The requests of the three games were translated by `translate_handles.py`, with the table beside each game in `handles_0086.json` (only `full_slice_five_days_sonnet` has requests that changed; the Eastern Solomons games' Allied groups 25 and 26 become 11 and 12, which no request names), and the transcripts were made again with `regame.py`; `check_translation.py OLD NEW TABLE` shows a new transcript differs from the old only in handles. Orders, results and hashes are unchanged, and every `save.json` is byte for byte as it was. The accounts quote the old numbers.

Regenerated after air round 4 (Nov 13, 2026), which set `fighter_false_claim_chance` in `air_combat.toml` to 0.7. The requests are as played. The answers that report a side's claims differ: one briefing in each Eastern Solomons game and three in the full slice game. Each `save.json` also has the new data-file hash of `air_combat.toml` and, from the first claim on, new daily hashes: claims are in belief. Remade again on main's requests after decision 0086 part 1 (handles per side); the same answers differ.

Regenerated after Rabaul round 1 (Nov 13, 2026), which set `[base_hit] aircraft` in `air_combat.toml` to 0.06. The requests are as played. One answer differs in the Eastern Solomons game played by Haiku (an `explain` of a raid on Henderson Field) and one briefing in the full slice game; the Sonnet Eastern Solomons game differs only in the data-file hash. Each `save.json` has the new hash of `air_combat.toml`. Remade on main's per-side handles after the merge.

Regenerated after decision 0086 part 2 (units made in play are numbered from their side's series), state version 8. Every transcript is byte for byte as before: no answer, handle or report moved, since no game here makes a unit in play whose draws matter to an answer. Each `save.json` differs only in its daily hashes, which cover the new counters and the version.

Regenerated after the batch of small fixes of `docs/rules/differences.md` (Nov 13, 2026), in which a contact no longer alerts or stands down a force at battle readiness (row 2). The requests and accounts are as played. In the full slice game one briefing differs (after day 5): the Allied events it cites are numbered one lower from the morning on, since an alert event of a force at battle readiness is no longer recorded; nothing it reports changed. Its `save.json` differs only in the hash of day 5. The two Eastern Solomons games did not change.

Remade after the placeholder notes of Nov 13, 2026: comments added to `air_upkeep.toml`, `fatigue.toml` and `air_supply.toml` change their data-file hashes in the full slice game's `save.json`. Nothing else changed: no request, answer or daily hash.

Remade after the figure notes of Nov 13, 2026: comments on every figure of `surface_combat.toml`, `search.toml` and `air_combat.toml`, and new headers on `port_sizes.csv`, `ship_classes.csv`, `aircraft_types.csv`, `air_upkeep.toml`, `fatigue.toml` and `air_supply.toml`, change their data-file hashes in each `save.json`. Nothing else changed: no request, answer or daily hash.

Regenerated after the closing legibility test's wording fixes (L2, L3, L5, L8, L12, `docs/validation/closing-tests.md`): the briefing's text now says that a group sharing a squadron's name with another of the side's own is "a separate group from" it (L2), that a base's fuel is "for ships; aircraft do not use it" (L3), gives the side's own event numbers on the attack from the air on a ship (L5), says "Aircraft wear: not in play" beside the fatigue line when wear is not loaded (L8), and reads a system hit that changed nothing as "hit, no change" and not "working to working" (L12). The requests and accounts are as played. Two briefings differ in each Eastern Solomons game (the opening one and the one after day 1) and eight in the full slice game; no other answer moved. The wording is the only change: no order, result or hash differs, and every `save.json` is byte for byte as it was.

Remade after decision 0089 (Nov 13, 2026): reworded notes in `air_combat.toml` change its data-file hash in each `save.json`. Nothing else changed: no request, answer or daily hash.

Regenerated after decision 0069 (ship repair, state version 9; five figures added to `surface_combat.toml`). Every transcript is byte for byte as before, and every `game.jsonl` is unchanged: no request, answer, handle or report moved. The orders in each `save.json` are identical. Each `save.json` differs only in the data-file hash of `surface_combat.toml` (now `9f5a179ed349e925`) and in its daily hashes, which cover the new state fields and the version (the two Eastern Solomons games have one daily hash each, the full slice game five).
