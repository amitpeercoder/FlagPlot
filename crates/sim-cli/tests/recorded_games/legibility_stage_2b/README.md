# Condition 5 of stage 2b: each can state its reason (decision 0138, item 12)

Run once, on Nov 17, 2026, by the team lead of the proof of stage 2b (`docs/validation/stage-2b-proof.md`). It is not a test CI runs, and this folder holds no game (`regame.py` and the `mcp` tests skip a folder with no `game.jsonl`).

- `pick.py`: the rule that chose the twelve cases, in its docstring: the first case, by the report's event number, on each of the first twelve seeds in order that have one, read from the condition 4 script's lines for `data/scenarios/stage-2b-proof/05a_eastern_solomons.toml` on seeds 1 to 1,000. It asks the tools for each case's three answers and writes the text each reader was given.
- `cases.json`: the twelve cases (seed, side, report, the commands A and B, their groups, when B first held the report), the three answers of the tools exactly as printed, and `prompt_text`, the only thing each reader was given.
- `key.md`: the key, written from the measured data (the tools' answers) before any reader ran.
- `answers.md`: each reader's answer verbatim. One Haiku model per case, with no tool of any kind (`claude -p --model haiku --tools "" --strict-mcp-config --setting-sources ""`), given the case's text alone; each answered in one turn.
- `mark_prompt.txt` and `marks.md`: what the marker, a Sonnet model with no tool, was given, and its marks verbatim.

Result: **12 of 12 cases right** (all three answers of every case), against the pass mark of 10 of 12. It passes.
