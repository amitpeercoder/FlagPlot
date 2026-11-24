# The reader's prompt, stage 4's legibility test (condition 11)

Fixed before the proof is run. Each reader (a Haiku model) is given one estimate, alone, and these five questions, and nothing else: no rule, no glossary, no other estimate. The estimate is given as the seat's `trial` tool answers it (`estimateN.json`, the whole answer). The marker (a Sonnet model) is held to `key.md`, which the harness wrote from the estimates' own fields before any reader ran; the architect reads the marks.

```
Below is the answer a staff gave a commander when he asked it to try an operation. Read it, and answer five questions about it in your own words, in one or two sentences each, using only what the answer says.

1. What does the operation need?
2. When could it go?
3. In how many of the worlds tried did it come off?
4. What may it cost?
5. What was assumed?

THE ANSWER:
<the estimate, as the seat is answered>
```

An estimate passes when all five answers agree with the key. The mark is six of six. The models' names and versions, this prompt, the answers and the marks are kept in this folder when the readers have run; `key.md` and `estimate1.json` to `estimate6.json` are made by `run.sh harness` (the test `legibility_estimates`) and copied here from `DIR/harness/legibility/` by the architect before any reader runs.
