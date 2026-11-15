# Legibility of the staff's reasons (decision 0107, ruling 39)

Run once, on Nov 15, 2026, by the team lead of step 4 of stage 1. It is not a test CI runs, and this folder holds no game (`regame.py` and the `mcp` tests skip a folder with no `game.jsonl`).

- `pick.py`: the rule that chose the 24 reasons, written before the pick (its docstring), run on the exports of scenarios 36, 40a, 41a and 42b on the frozen figures (`sim-cli export ... --reference data/reference/step1`).
- `reasons.json`: the 24 reasons with their five parts, and `prompt_text`, the only thing each model was given.
- `key.md`: the key, a draft by the team lead written before any model ran, for the architect to read (the ruling makes the key the architect's).
- `answers.md`: each Haiku answer verbatim, one agent per reason, given the reason alone and the two questions.
- `marks.md`: a second model's marks against the key.

Result as marked: 20 of 24 pass both questions; every order, no-order, displaced and hold reason passes; the four leave reasons fail question 1 only, because those reasons name no base. By the ruling's second part (no kind under half) the pass mark is not met. It stands only once the architect has read the key and every fail.
