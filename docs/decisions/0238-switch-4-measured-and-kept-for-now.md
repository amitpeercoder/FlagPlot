# 0238: Switch 4 measured, and kept for now

Status: ruled by the architect on Nov 27, 2026, on Amit's delegation of that day ("Don't wait on me. Let the architect make whatever decision."). Written first as the plan to retire the key `staff_chooses_supply_sources`, as decision 0236, ruling 16 asked; the measurement made for that plan changed the case, and the manager asked for the ruling again against Amit's programme, "cheaper to write and maintain without losing anything". This record amends decision 0236's ruling 16. Nothing is built and nothing is to be.

Summary: The key `staff_chooses_supply_sources` is kept for now. Measured, retiring it would restate 29 tests and not three, leave one recorded game of a closed proof unable to replay, and slow a trial of the stage 3 slice about eight times for a cause not traced, to save some 200 to 250 lines. It is ruled again when the slowness is traced and mended and there is a way to keep that recorded game replaying. The slowness is a defect on its own, since the files with the key on already run that code.

Standing: Ruled by the architect on Nov 27, 2026, on Amit's delegation; amends decision 0236, ruling 16; switch 4 kept until the two conditions here are met; nothing to build

## Why it was looked at

Decision 0228 kept the older choice of a source behind a key so that a closed proof's recorded game would replay, and said the old choice goes when that record is marked as of an older engine. The list of old switches (`docs/validation/old-switches.md`, number 4) recommended retiring it at a cost of eight games moved and three hand calculations restated, and decision 0236 ruled it be done by a class 3 record.

## What was measured

On `origin/main` at 72cd3db0, with the three readers of the key made to answer "on" for every side and the hashed byte left as it was, so that a day hash that differs is a state that differs: `sim-cli run --events` on all 263 scenario files, the workspace's tests, and the five recorded games played again from their requests.

1. **Eight games move in what happens,** the eight the list names: `69_taken_and_held`, `79_fed_before_it_is_taken`, `a-limit-binds-what-it-names/130_two_holds`, `five-days/01a`, `01b`, `02a` and `02b`, and `table-hunger/02b`. In each the staff sets the forces limit of a supply it formed under a hold: one command and one amendment of a plan. No ship, ton or man moves.
2. **Twenty-six more move in the words of a reason and in no hash,** which the list did not see, having run without `--events`: `76`, `77`, `86`, `90`, `135`, `136`, `137`, `stage-3-proof/07a`, `07b` and `07c`, `stage-4-proof/case_w3`, `table-hunger/07b` and `07c` (the island's reason of its source); `coral-sea/01a`, `01c`, `02a`, `02c`, `03a`, `03c`, `five-days/01c` and `02c`, `midway/01a` and `01c`, `table-hunger/01c`, `02c` and `03c` (a hold with no source and no base that can spare).
3. **29 tests fail:** 24 of the scenario package (scenarios 36, 69, 74, 76, 77, 79, 86, 101, 106, 130 and 132), one of `sim-core` and four of `sim-cli`. Scenarios 36, 74, 76 and 86 each state something the one rule changes; in scenario 36 a plan whose pool is empty under a limit that names no ship is no longer marked failed, which is a change to a commander's own supply that nobody chose.
4. **`second_proof_week_sonnet` does not replay.** Under the one rule the staff forms a supply for Tulagi under the model's hold on day 15, the model's next intent is numbered 9 and not 8, and its request to limit "intent 8" lands on the staff's supply. 11 of 53 answers differ, the day hashes differ from day 15, and one of its commands is refused. Its requests may not be edited (decision 0228, A). Its save is also read by scenarios 111, 112 and 113. The other four recorded games replay answer for answer.
5. **A trial slows about eight times.** `sim-cli time`, section h, a trial of the Allied side on `stage-3-proof/07a_slice.toml`: 16 worlds for 14 days, 5.7 s as on `main` and 44.2 s with every side on the one rule; 8 worlds for 7 days, 0.9 s and 1.9 s. The test `timing::time_reports_its_sections_and_changes_nothing` had not ended after 38 minutes. Not traced. The lead: the bases are weighed, each with a sea path, on every morning and not only on the morning a source is chosen.

## What was weighed

- **Retire now, marking the recorded game as of an older engine and dropping its save from three tests.** Not done. That is a fixture lost: the one agent-played record of stage 2's second proof would stop being checked on every commit, and three statements would each lose a save they compare. The programme says nothing is to be lost. Against it stand some 200 to 250 lines of code that are stable, tested and touched by nobody.
- **Retire now and edit the game's requests** so that "intent 8" reads "intent 9". Not done: decision 0228 rules the requests are not edited, and the game would differ in substance from the one the proof records.
- **Retire now and accept the slowness,** to be mended after. Not done: the cause is not known, and a rule made universal before its cost is known could put every trial at the table outside its budget.
- **Keep it for good.** Not done: it still keeps a rule the designer has replaced. It is kept until it can go without loss.

## The ruling

1. **Switch 4 is kept.** No file, test, pin or version changes. Decision 0236's ruling 16, "retire, by a record of its own", is amended to: kept, and ruled again when both of these hold.
   - **The slowness is traced and mended** (ruling 3), and section h of `sim-cli time` with every side on the one rule takes what it takes on `main`, give or take a tenth.
   - **There is a way to keep `second_proof_week_sonnet` replaying to its transcript and save,** byte for byte, with its requests as played. The architect knows of one to try and has not tried it: a recorded game's header names the doctrine it was played under, and the older choice kept for a game whose header says so, in `sim-staff` alone, with the key gone from the scenario and the state. Whether that is cheaper than the key is the question for that day. If no way is found, the switch stays.
2. **The cost of that day is already known** and is not three hand calculations: the lists above, and the statements the architect worked out, kept in `as-built/0238-switch-4-measured-and-kept-for-now.md` as not in force. They are read again against the code of that day before they go into any test.
3. **The slowness is a defect whatever becomes of the key,** and is recorded in `docs/open-work.md`. The 20 files with the key on (the table's 19 and scenario 132) run the same code today. Whether a trial at the table pays the eight times is not measured: the measurement was of a proof's file with the key forced on. Tracing it is its own job: measure a trial on a `table-boats` file with the key on and with the line taken out, find where the time goes, and class the mend (class 2 if it changes no order, command, reason's word or hash).
4. **The list is corrected** where it was wrong, in `docs/validation/old-switches.md`.

## Rule card

No rule of the game is added, changed or removed. No state, draw, event, order, figure or version moves.

## Its class

Class 2, with nothing to build: a ruling and three documents. The mend of ruling 3 is classed when it is traced.

## Questions for the designer

None. Told to him in the report: that number 4 of the list he read is kept for now and why, and that the table's trials may be slow for a cause not yet traced.
