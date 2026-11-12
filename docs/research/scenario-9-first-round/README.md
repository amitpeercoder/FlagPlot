# Scenario 9, the first round of games

On Nov 12, 2026 two Sonnet models each played a game alone with `sim-cli play`, before objectives, claims and the wording fixes existed. Their requests are kept here as they made them (`*.game.jsonl`). They are not replayed by a test: the briefing and the tools have changed since, so the answers they were given cannot be reproduced. The games that are replayed are under `crates/sim-cli/tests/recorded_games/` (the second round).

What follows is the architect's summary of the two accounts, not the players' words.

## Eastern Solomons, Allied side

- The player put two scouting groups on search to the north, fighters on patrol and both forces at battle readiness, and left its bombers on rest "to strike once I had a contact". Its search found the Japanese force; nothing struck it; Enterprise took six hits and lost her flight deck. The game then ended.
- It could not tell what it was meant to achieve, how long the game was, or why it ended after one day. It guessed the search arc convention. It took Henderson Field's sizes of 0 to mean the field was unusable.
- It kept to the rules. All eight orders were accepted.

## The full slice, Allied side, five days

- The player searched and patrolled for five days and never held a contact. Henderson Field was raided on three days. On day 4 Enterprise was hit eight times and left unable to move, which the briefing did not show.
- It found the search arc rules only by being refused twice; both refusals said how to fix the order. It could not tell where raids came from or whether its searches worked. It did not see an arrival come in.
- It kept to the rules.

## What was changed because of them

- The tools now say what each air mission does and that the staff acts inside the day, so missions are set before a contact appears (pull request 62).
- The briefing shows the game's length, flags a force that cannot move, marks enemy units in events, and says when an arrival joins.
- Scenarios have a mission and objectives, and a result at the end (decision 0054).
- A side is told what its own people claim to have done (decision 0056).

The second round, played after these, is the record of scenario 9.
