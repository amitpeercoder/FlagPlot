# Decision 0156: the proof of orders inside a day

Run on Nov 18, 2026, by the team lead of step 5, and judged by the architect the same day, to the conditions of decision 0156 ("The conditions of its proof") as the architect's brief for step 5 sets out how each is run. Release build unless said, live figures, on the branch `orders-inside-proof` cut from `table-stops` at 1599a679 (steps 1 to 4 of decision 0156): state layout `flagplot-state-v44`, rules label 19, doctrine `supply-34`, agent interface 25, saved game 20. Apple M3 Pro, 11 cores. Nothing was tuned, and no rule, figure, doctrine, reading or condition was changed for or after a run.

**Result: five of the seven judged conditions hold, and two miss.** Nothing changed where the key is off, or where it is on and unused; the hand calculation passes; a saved game replays and a changed key is refused; a seat is told nothing more. Legibility misses: the events of a day do not say what an order changed, when what it changed is a unit's readiness or an air group's mission (4 of 6 by the condition's words, 5 of 6 by the key). Speed misses at the tool on a near-empty day (1.541 against 1.1) and holds in the engine and on a battle. What it is worth, which is not judged: a model in the Allied seat did worse advancing by pulse than by days. The judgement is at the end. Each figure below was checked by the architect against the run's own output.

| Condition | Value | Mark | Holds |
| --- | --- | --- | --- |
| 1a, every generated file as the engine gives it | `regenerate.sh --check`: "every generated file is what the engine gives" | as written | yes |
| 1b, the Eastern Solomons, 1,000 seeds, before and after | the two batch files byte for byte the same (4,336,400 bytes, sha256 d592f3dc...7139641) | identical | yes |
| 2, on and unused: by pulse against by days | 1,000 of 1,000 seeds agree, from the Allied seat and from the Japanese seat; no seed differs | 1,000 | yes |
| 3, the hand calculation of scenario 78 | 9 of 9 tests pass | pass | yes |
| 4, a saved game replays, and refuses a changed key | replays to its three daily hashes; refused with the key changed (words below) | as written | yes |
| 5, a seat is told nothing more | the two source tests pass; at 0600 and 1200, from each seat, `events` counts what the seat's view holds (14, 16; 44, 67), none of a pulse not resolved | as written | yes |
| 6, legible | 5 of 6 cases pass all three questions; (i) and (ii) 6 of 6; one miss, case E, question (iii) | 6 of 6 | **no** |
| 7, what it is worth (reported, not judged) | by days: 30 of 30 over, a strike at ships in 19, median Allied margin 3.3; by pulse: 30 of 30 over, a strike at ships in 13, median 0.675, orders inside the day median 0, most 14 | none | not judged |
| 8, speed: three calls against one | the engine: 0.946 (the Eastern Solomons) and 0.996 (the slice, days 1 to 10); the session: 1.012 and **1.541** | 1.1 | **no**, at the tool on a near-empty day; yes in the engine and on the battle |

## 1. Nothing changed where it is off

**(a)** `sh scripts/regenerate.sh --check` on the branch at 1599a679, before anything of this step was added: "every generated file is what the engine gives", exit 0 (the run baselines, the agent goldens, the viewer's measure fixture, the recorded games, the pinned hashes and the batch fixture). Run again on the branch with this step's files and `origin/main` merged in (b0bdd540): the same words, exit 0.

**(b)** `sim-cli batch data/scenarios/05_eastern_solomons.toml --runs 1000 --first-seed 1 --label proof`, release, each run from the top of its own worktree:

- before: a worktree detached at 2eb55806, the merge of pull request 425, the last commit on main before pull request 426;
- after: this branch at 1599a679.

`cmp` finds the two files the same; both are 4,336,400 bytes with sha256 `d592f3dc6fbe1a1ae9110ff88f274ce71522a919f9626626990da7c517139641`. No version string differs. The scenario file and every data file it reads are the same at both commits (`git diff 2eb55806 1599a679 -- data/` changes only scenario 78, which is new).

## 2. Nothing changed where it is on and unused

`data/scenarios/orders-inside-proof/05_eastern_solomons_on.toml` is scenario 5 with the key on, made by `scripts/orders-inside-proof/make_arrangements.py` (`--check`: as made). The test `condition_2_by_pulse_has_the_hashes_of_by_days` (`crates/sim-agent/tests/orders_inside_proof.rs`, `#[ignore]`), run as `PROOF_SEEDS=1-1000 cargo test --release -p sim-agent --test orders_inside_proof condition_2 -- --ignored --nocapture`, plays on each seed three games: by days with the doctrine's staffs as `sim-cli run` plays it, and through a session advanced one pulse at a time (`advance {"pulses": 1}`) with the script's orders given at each dusk and no order inside the day, once with the Allied seat a player's and once with the Japanese seat a player's. It printed:

> condition 2: seeds 1 to 1000: 1000 of 1000 agree from both seats (Allied seat 1000, Japanese seat 1000); first that does not: none

## 3. The hand calculation

`cargo test -p scenario-tests --test scenario_78_orders_inside_a_day`: 9 passed, 0 failed:

- `the_file_is_scenario_1_with_the_key_on_and_one_order_at_0600`
- `case_a_the_order_at_0600_is_taken_in_the_morning_and_the_force_slows`
- `case_b_the_order_at_1200_is_taken_in_the_afternoon`
- `case_c_with_the_key_on_and_no_order_inside_a_day_every_hash_is_scenario_1s`
- `an_order_of_another_kind_at_0600_is_refused_as_given_at_dusk`
- `the_later_the_order_the_farther_the_force`
- `with_a_staff_an_order_at_0600_holds_the_convoy_from_the_staff`
- `a_replay_of_the_orders_a_saved_game_keeps_gives_the_same_hashes`
- `orders_are_handed_in_inside_a_day_only_at_0600_and_1200`

## 4. A saved game replays, and refuses a scenario whose key was changed

`sim-cli run data/scenarios/78_orders_inside_a_day.toml --save FILE` wrote a saved game of version 20 holding the 8-knot order with `"pulse": "morning"`; `sim-cli replay FILE` printed the three daily hashes (`28b7568688229566`, `25e71ae5c94f37dc`, `88e91e6c85d3f83f`), exit 0.

The save names the scenario by its path from where `replay` runs (`data/scenarios/78_orders_inside_a_day.toml`), with the file's hash. So a copy of `data/map`, `data/reference` and the scenario was made outside the repository under the same paths, and `replay` was run there on the same save:

- the copy unchanged: replays, the same three hashes;
- the key set to `false`: refused, exit 1: "error: data/scenarios/78_orders_inside_a_day.toml: the order on day 1 has a pulse, and orders inside a day need orders_inside_a_day = true at the top of the scenario". The scenario file's own scripted order has a pulse, so the file no longer loads;
- the key line taken out: the same words;
- the key set to `false` and the file's own order with a pulse taken out too, so that the file loads: refused, exit 1: "error: the scenario file data/scenarios/78_orders_inside_a_day.toml has changed since the game was saved (saved hash ff075762fcebdb52, now 561d4360a9b477e9)";
- for comparison, the copy unchanged but for one comment line added: refused with the same words as the last (now f916b868da821545).

So a changed key is refused, either by the loader or by the saved hash of the scenario file; no check reads the key itself.

## 5. A seat is told nothing more

The tests that read `sim-agent`'s source pass: `the_library_code_names_nothing_of_the_truth` and `the_tools_of_a_seat_read_the_world_only_through_the_seats_view` (`crates/sim-agent/tests/leaks.rs`).

Added: `condition_5_events_at_0600_and_1200_are_the_seats_view` (`orders_inside_proof.rs`, run with the rest). On `05_eastern_solomons_on.toml`, from each seat, with the script's orders given at dusk, the session is advanced to 0600 (one pulse) and, in a second game, to 1200 (two). Then `events {}` is called, and the session's world is taken (`Session::into_world`) and the side's own view read (`SideView::events_on_day(1)`). It asserts that `count` is the length of the list, that the numbers listed are the view's numbers in order, and that no event is of a pulse not yet resolved (at 0600 only `night` or none; at 1200 `night`, `morning` or none). It passes:

| Seat | At 0600: `events` count, the view's | At 1200 |
| --- | --- | --- |
| Allied | 14, 14 | 16, 16 |
| Japanese | 44, 44 | 67, 67 |

## 6. Legible: 5 of 6

The six days, each with one order given inside it, played through a session by `condition_6_the_six_days` (`orders_inside_proof.rs`, `#[ignore]`), the Allied seat's events of day 1 taken from `events` at its dusk:

| Case | Game | Order inside the day |
| --- | --- | --- |
| A | scenario 78 with its own order taken out (its case C); at dusk the route to (8,3) at cruise | `set_speed` 8 kn to task force 1 at 0600 |
| B | the same | the same at 1200 |
| C | `36_one_convoy_on.toml` (scenario 36, key on) | `set_speed` 8 kn at 0600 to the convoy the staff formed (the hold) |
| D | `05_eastern_solomons_on.toml`, the script's Allied orders at dusk | `set_readiness` battle to task force 2 (Task Force 11) at 0600 |
| E | the same | `set_air_mission` rest to air group 6 (VB-3, Saratoga, on naval strike) at 1200 |
| F | as A | `set_route` to (5,0) to task force 1 at 1200 |

Cases D, E and F, and their hours, were chosen before the games were played. Each reader's material was that case's events, each event's number, pulse, kind and words, nothing else. The key was written from the material alone before any reader ran. Six Haiku readers, one a case, each read one file holding the material and the three questions; a Sonnet model marked the answers against the key, and the team lead checked the marks. `crates/sim-cli/tests/recorded_games/legibility_orders_inside_a_day/` keeps the material (`cases.json`), the reader's words (`prompt.txt`), the key, the answers and the marks.

**5 of 6 cases pass all three questions.** Every reader gave the right hour and the right pulse.

**The miss: case E, question (iii).** The key: air group 6 did not strike; in the afternoon air groups 7 and 8 launched at contact 2 and air group 6 launched nothing; an answer passes if it says air group 6 did not launch or was kept back from the afternoon's strike. The reader: "The events do not say what it changed. Event 17 only records that order:15 was accepted for air_group:6, and no later event for air_group:6 appears that day." The marker: a fail by the key's wording, and borderline.

Read with the result:

- The words of an `order_accepted` name only the order's number and its unit, never what was ordered. So (iii) is read from what follows: the speed in `task_force_steamed` (A, B), the hold in `held_by_order` (C), the turn in `task_force_turned_back` (F). In D nothing shows Task Force 11's readiness, and in E only an absence shows.
- **D passes (iii) by saying the events do not show the change**, as the key, written before the readers, allows. The architect's reading is below.

## 7. Reported, not judged: what it is worth

**The arrangement.** Stage 2's second proof played "the Allied side from intents" in its arrangement (b), `data/scenarios/second-proof/05b_eastern_solomons.toml`: the Japanese side scripted, the Allied side's intents in the file. Its arrangement (c) is the same with the Allied side given nothing. The two copies made by `make_arrangements.py` are (c) with a header: `05c_allied_by_days.toml` (no key) for the games by days, and `05c_allied_by_pulse.toml` (key on) for the games by pulse.

**The games.** 60 games, seeds 1 to 30 in each arm, game N on seed N. Each was one fresh Haiku subagent, at most 14 at once, given the brief (`scripts/orders-inside-proof/condition-7/`: `brief_common.txt` with `brief_days.txt` or `brief_pulse.txt`, written before the first game) and playing through a wrapper that called `target/release/sim-cli play --game FILE --scenario ... --seat allied --seed N TOOL ARGS`. Both briefs give the mission, say to command by intent (`set_intent`, `set_leave`, leave to launch; the other commands to the staff allowed; no `issue_orders` at dusk), and then: by days, `run {"days": 1}` to the end; by pulse, `advance {"pulses": 3, "stop_on": ["contact", "attack", "loss"]}` and act at each stop, `issue_orders` allowed at 0600 and 1200 for the seven kinds. Each game was read after it ended by `scripts/orders-inside-proof/read_games.py`: `play --export-save` and `--export-transcript`, `sim-cli export` of the save; a strike at ships is a `strike_launched` (not `base_strike_launched`) by a group the export gives as Allied; the margin is the `result.allied.margin` of the last answer; the orders inside the day are the Allied orders the saved game holds with a `pulse`.

| | By days | By pulse |
| --- | --- | --- |
| Games completed (game over) | 30 | 30 |
| Games that did not finish | 0 | 0 |
| A strike at ships by the Allied side | 19 of 30 | 13 of 30 |
| Allied margin, median | 3.3 | 0.675 |
| least, lower quartile, upper quartile, most | -4.3, 0.1625, 10.4, 12.25 | -4.6, 0.1, 7.30, 12.45 |
| Orders inside the day (saved, with a pulse): median, most | 0, 0 | 0, 14 |
| Games with any order inside the day | 0 | 12 |
| Orders inside the day refused (not saved), all games | 0 | 102, in five games |

In the by-pulse arm the first strike of every game but one was in the afternoon (game 8: the morning). The 12 games with orders inside the day all gave them at 1200; of those 12, two struck ships (games 20 and 22).

**Beside them, the staff alone.** From stage 2's second proof, third run (`docs/validation/stage-2-second-proof-third-run.md`, doctrine `supply-17`), over seeds 1 to 1,000:

- both sides from intents (its arrangement (a)), the "staff alone on both sides": the Allied side strikes ships on 767 seeds (table, "A strike at ships on 800 or more seeds: Allied");
- the Allied side from its stated intents against the Japanese script (its (b), the arrangement nearest this one): strikes on 528 seeds ("Why the Allied side's margin rose"), median Allied margin 0.95 ("Against no orders, judged").

Not asked, and run on this branch's engine (`supply-34`) for comparison, the same files by `sim-cli batch`: (a) strikes on 822 of 1,000, median Allied margin 4.825; on seeds 1 to 30, 23 and 9.05. (b) strikes on 544 of 1,000, median 0.95; on seeds 1 to 30, 19 and 3.175.

**What went wrong in the running, in full.**

- **The tool list was emptied for a time.** The wrapper's `play.sh tools` printed a list kept in the games' folder. In the first wave (games 2 to 8 of each arm) the player of game 5 by days ran `play.sh tools > .../tools.txt`, which emptied the file before it was read. Until it was remade and made read-only (with the folders the players could write to), `play.sh tools` printed nothing (by days) or only `events` and `advance` (by pulse). By their own reports, the players of games 7 and 8 by days and 4 and 8 by pulse met the empty list and learned the arguments from the game's refusals. No other game reported it. None was replayed.
- **Players broke the brief's limits.** Several ran commands other than the wrapper (filters of its output; reading a saved tool output; one wrote a file to `/tmp`), and several queued intents by mistake while learning the arguments and withdrew them before the day (the saved games hold both). Each is in the player's own report; none touched another game's file.
- **Several by-pulse players read the clock wrong.** With `pulses: 3`, a game whose first contact, attack and loss came in the afternoon ran to the end in one call. Several players reported this as `advance` failing to stop. It did not: on games 9, 10, 14 and 15 the events `told` were checked, and all are of the afternoon pulse.

## 8. Speed

The test `condition_8_speed` (`orders_inside_proof.rs`, `#[ignore]`), `PROOF_REPEATS=30 cargo test --release -p sim-agent --test orders_inside_proof condition_8 -- --ignored --nocapture`, Apple M3 Pro. Each way 30 times after one unmeasured run, the four ways taken in turn; the figure is the median over the 30 of the days' times summed. Both scenarios read with the key on: the Eastern Solomons from the proof folder (its one day), and the staff's setup of the full slice with the key written into its text in the test (days 1 to 10; the file is not edited). Two measures, since the condition does not say at which level "a call" is:

- **the engine:** a day by one call, `Scenario::next_day_staffed` (which is `World::resolve_day_watched`), against the same day by `open_day`, `resolve_next_pulse` three times and `close_day`, called from outside;
- **the session:** the Allied seat a player's with the script's orders given at each dusk (outside the reading), a day by `end_day` against the same day by `advance {"pulses": 1}` three times.

| | One call | Three calls | Ratio |
| --- | --- | --- | --- |
| Engine, the Eastern Solomons | 2.809 ms | 2.657 ms | 0.946 |
| Engine, the slice, days 1 to 10 | 0.846 ms | 0.842 ms | 0.996 |
| Session, the Eastern Solomons | 2.762 ms | 2.796 ms | 1.012 |
| Session, the slice, days 1 to 10 | 0.852 ms | 1.312 ms | **1.541** |

Run a second time to see it is steady: 0.948, 0.996, 1.017, 1.538. On the slice the session's three `advance` calls cost 0.46 ms more over ten days than `end_day`, about 46 microseconds a day, on days that cost some 85 microseconds each (the staff's setup with no intent: the staff does nothing). Why was not measured. Which measure the condition means is ruled below.

## Before the pull request

On the branch at b0bdd540 (this step's files, with `origin/main` at c6f8fe01, the merge of pull request 429, merged in, which changed no file): `cargo fmt --all -- --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean; `cargo test --workspace` 208 test binaries, none failed; `sh scripts/regenerate.sh --check` "every generated file is what the engine gives". The only edit to a test file after the measuring runs was `cargo fmt` of `orders_inside_proof.rs`, which moves lines and changes nothing they do.

## What this did not do

- Condition 7's games by pulse used the stops the players chose; no game was steered and none replayed. The 60 games are one model's play, read once.
- The cause of the session's cost on the slice (condition 8) was not looked into.
- In condition 4, the refusal of a changed key was not asked of a save whose scenario has no order with a pulse of its own.

## The judgement

By the architect, on the figures above, with nothing changed after the run.

**Conditions 1 to 5 hold.** The two batch files were compared again by the architect and are the same; the lines for conditions 2, 3, 4, 5 and 8 are in the run's outputs as given here. In condition 4 the refusal comes from the loader or from the saved hash of the scenario file, and no check reads the key by itself. That is enough: the key is a line of the file, so a file whose key is changed is a changed file.

**Condition 6 misses.** The condition asks that a reader say what the order changed, six of six. By the key the count is 5 of 6. By the condition's own words it is 4 of 6: in case D the reader said that the events do not show what the order changed, and the key, which the team lead wrote, lets that pass. Saying that the log does not tell is a right answer about the log and not the answer the condition asks for. Both counts miss, and the cause is one: an `order_accepted` event names the order's number and its unit and never what was ordered. A change of speed, a hold and a turn show in what the unit does next. A change of readiness shows nowhere in the day's events, and a group stood down shows only as a launch that did not happen. When and in which pulse an order acted were read right in all six.

**Condition 8 misses, by the stricter reading.** The condition does not say where a call is counted, and a reading is not chosen after the run to make it pass. A seat resolves a day in three calls by `advance`, so the tool's figure counts. In the engine a day in parts costs no more than a day in one (0.946 and 0.996). At the tool it costs 1.012 on the battle and 1.541 on ten near-empty days of the slice. The size of it: about 46 microseconds a day, some 15 a call, on days of 85 microseconds. By reading the code and not by measuring: each `advance` builds the side's view before and after its pulse and its answer, which is the cost of answering three times, not of resolving. It does not grow with what happens in the day, as the battle's figure shows.

**Condition 7, not judged, read plainly.** By days the model struck ships in 19 of 30 games with a median margin of 3.3; by pulse in 13 of 30 with 0.675. The staff from the stated intents, on the same 30 seeds and today's engine, strikes in 19 with 3.175, so the model by days does as the staff does. The 18 games by pulse in which the model gave no order inside the day struck in 11, much as by days. The 12 in which it did give orders, all at 1200, struck in 2. So the loss is in the games where the model used the new orders: an order to a unit takes it out of the staff's hands, and the model took units from a staff that was about to strike. Five games had 102 orders refused between them. This is one model's play, read once, with the faults in the running listed above; it shows no gain from commanding inside a day in a model's hands as the tools stand.

**What follows, for the designer.** The proof is not rerun and nothing is tuned to it. The two misses are his to accept as named or to have mended:

1. Legibility: the event of an order taken in says what was ordered, to the side that gave it (its own order; nothing of the enemy). This changes the words of an event, so the generated files are remade, and the six cases are read again as a check of the mend.
2. Speed at the tool: accept as named. The cost is of answering, is small, and does not grow with the day.

The key stays off for the 39 days. What condition 7 shows is a question of design more than of this build: a hold that takes a unit from the staff is a heavy thing to hand a player in the middle of a day.

## After the judgement: the mend of legibility, and its check

The designer chose on Nov 18, 2026 to have the legibility miss mended. The proof's result stands as judged above; what follows is a check of the mend and not a second run.

**The mend.** The event that says an order was taken in now says what was ordered, to the side that gave it: `order:15 accepted for task_force:2: set_readiness, readiness battle`. Only the words a seat's tools write for that event changed (`sim-agent`, `Ctx::event_text` and `orders::order_words`). The engine, the event log, every hash and the umpire's files are as they were; of the generated files one recorded game's transcript changed, in two lines of that kind.

**The check.** The six days were made again by `condition_6_the_six_days`; in each, only the words of `order_accepted` events differ from the proof's material. The architect wrote a key before any reader ran: (iii) passes only if the answer says what the order set, and an answer that says the events do not tell fails. Six fresh Haiku readers were given the proof's own prompt and one case each, and the architect marked them. **Six of six**, each of the three questions. `crates/sim-cli/tests/recorded_games/legibility_orders_inside_a_day/mended/` keeps the material, the key, the answers and the marks.

The speed miss is left as named.

