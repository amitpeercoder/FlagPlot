# The closing tests of step 2

Three of the lines of `docs/step2-spec.md`, "Done means", as the coordinating architect ruled they are met ("How each line is met"): the hundred-seed soak run by hand, the legibility test on the tuned Eastern Solomons run as predict, run, explain, and the rulebook-only hand calculations defined afresh. Run on Nov 13, 2026 by an Opus team lead for the architect, on commit `b54840b` (`main` after pull request 228, the small-fixes batch). The pull request that carries this record also carries the rulebook and glossary fixes the architect ruled on afterwards (below, "What was changed in this pull request"); it changes no code and no data. No test and no build step reads these files.

The long texts are in the folder `closing-tests/`:

- `legibility-game.md`: what the model was shown of the game (the opening briefing, the orders and their answers, the final briefing).
- `legibility-answers.md`: both prompts and both answers, word for word.
- `legibility-umpire-log.md`: the umpire's log of the same game, which the model never saw.
- `rulebook-sittings.md`: the files each sitting was given, how the figures were altered, and the four prompts, word for word.
- `rulebook-answers-1.md` to `rulebook-answers-4.md`: the four answers of the first run, word for word.
- `rulebook-answers-4-rerun.md`: the control sitting run again after the rulebook fixes, word for word.

| Test | Result |
| --- | --- |
| 1. The hundred-seed soak, with the live determinism and invariant tests | **Pass.** 100 seeds of 180 days, every check holding; nothing else failed |
| 2. Legibility on the tuned Eastern Solomons | **Fails the procedure's rule by one item:** the explanation, the claims and truth and belief pass, but only two of the four predictions do (three are needed). Both misses are of the enemy's movements, which nothing the side is given shows. Twelve findings |
| 3. The rulebook-only hand calculations | **Every number of the step 2 rules is exact**, but for two derived chances of 19a that are the cut where the hand calculation rounds. One number of the step 1 control is wrong, on a term the rulebook does not define, and two more of its derived chances are the cut. Eleven findings. The architect ruled on each; the rulebook and glossary fixes are in this pull request; the control sitting, **run again after those fixes**, got every number exact, in the cut form the new rule of hand calculations asks |

## Test 1: the hundred-seed soak, by hand

The test is `a_hundred_seeds_run_180_days_with_everything_holding` in `tests/scenario_08_soak.rs` (`#[ignore = "a hundred 180-day runs; run by hand"]`, seeds 1 to 100, live figures, live `data/scenarios/08_soak.toml`; decision 0044). Run beside it, all in release: the rest of the soak file, the determinism scenario (`tests/scenario_07_determinism.rs`, eight of its nine tests on the live figures) and the invariants' unit tests in `sim-core`.

```
cargo test --release -p scenario-tests --test scenario_08_soak -- --ignored --nocapture
cargo test --release -p scenario-tests --test scenario_08_soak
cargo test --release -p scenario-tests --test scenario_07_determinism
cargo test --release -p sim-core invariants
```

- Commit: `b54840b` (HEAD `2879bd4`, which adds only the stub of this file). Toolchain cargo and rustc 1.97.0, Apple M3 Pro (arm64).
- Started 15:26:36 UTC, ended 15:27:09 UTC, Nov 13, 2026.
- The hundred seeds: `100 seeds, 1978115 events, 3.8 s`; `test result: ok. 1 passed; 0 failed`. Wall time with the build 15.4 s.
- The rest of the soak: 11 passed, 0 failed, 1 ignored (the hundred seeds, run above).
- The determinism scenario: 9 passed, 0 failed.
- The invariants: 24 passed, 0 failed (23 in `invariants.rs`, 1 in `resolve.rs`).

The invariants themselves (`sim_core::invariants::check`) run at the end of every day of every run above and fail the run on a violation, so the hundred seeds are also 18,000 days of invariant checks.

## Test 2: legibility on the tuned Eastern Solomons

Reading A of the procedure: predict, then run, then explain.

### The inputs

| Item | Value |
| --- | --- |
| Scenario | `data/scenarios/05_eastern_solomons.toml`, sha256 `c52b2cf1…a4271`, daily hash of a plain run `0d70319bcaa38438`; the game's own hash `2709b601250eb8dc` |
| Seat and seed | Allied; seed 1942, the scenario's own. No batch was run to find a median seed: the orders are not the scenario's, so a batch of the scenario would not say which seed is typical of them |
| Orders | The two `issue_orders` calls of the recorded Sonnet game, copied as they stand: a probe of the search range (refused: "search range 9999 nm is not above 0 and at most the aircraft's 397 nm") and sixteen orders (searches by VS-5, VS-3 and VMSB-232 to 200 nm, the four bomber and torpedo groups on naval strike, three fighter groups on patrol with VF-5 at a share of 0.5, both task forces at battle readiness) |
| How played | `sim-cli play --game FILE --scenario … --seat allied --seed 1942`: `get_briefing`, the two orders calls, then `end_day` and `get_briefing` after the prediction was in |
| Model | `haiku` (Claude Haiku) by the Agent tool, a fresh session for each, Nov 13, 2026 |
| Session 1 was given | The opening briefing, the orders and the game's answers, `docs/glossary.md`. No tools but reading those three files |
| Session 2 was given | The same, the final briefing and session 1's answer; and the side's tools by a wrapper of `sim-cli play` that refuses everything but `explain`, `get_briefing`, `list_forces`, `list_contacts`, `list_bases`, `get_entity` and `measure`, and runs each call on a copy of the finished game. It made 18 calls: 16 `explain`, `list_contacts`, `get_entity` |
| Not given | `end_day`'s answer, `sim-cli events`, `run --events`, batch files, the viewer, anything under `data/`, the umpire's log |

The procedure step "save the game with `--export-save` and `--export-transcript`" was done once the game was over (decision 0085 refuses the save before); the umpire's log in `closing-tests/legibility-umpire-log.md` is from that save. The save itself is not committed: the record is the briefings, the orders and the log.

### What happened (the umpire's log)

The Japanese carrier force (Ryujo's, task force 4) steamed overnight to (7,-12) and by noon to (7,-9), 280 then 160 miles from the Allied carriers at (7,-5). No morning search found anything: the enemy was beyond the 200-mile arcs. In the afternoon VS-5 sighted it at 160 miles (chance 1) and the four strike groups launched 47 aircraft at it; Ryujo took four bomb hits from VB-6 and sank under VB-3's (umpire events 129 to 162). The same afternoon a Japanese scout found Task Force 11 (taken for transports), and 27 Japanese bombers with 10 escorts struck Task Force 16 at (7,-5): Enterprise took 5 hits from the first raid (flotation 15.8, fire 10.5, flight deck degraded) and none from the second. Henderson was raided by 6 aircraft for no hits. Allied aircraft lost: 6. Japanese destroyed: 57, 24 of them ditched when their deck was gone. Result from the truth: Allied 12.85, Japanese 0.3, margin 12.55, a major Allied victory.

### The judging sheet

| # | Item | Pass when | What the model said, against the log | Result |
| --- | --- | --- | --- | --- |
| 1 | Search prediction | Right on whether and in which pulse the carriers are found, with a reason the rules support | "Possible, probably in the morning pulse, not certain"; VS-3 or VS-5 by bearing. Found in the afternoon by VS-5. Its reason (the arcs cover every bearing north to 200 miles) is sound; the morning was a guess the briefing could not support, as it said ("the briefing gives no Japanese positions") | **Fail** (pulse wrong) |
| 2 | Strike prediction | Right on whether each strike group launches and at which contact; reason names the target list or the radius rule | The four naval strike groups launch at a carrier contact within their radius, carriers first by the default target list; groups 11 and 12, on rest, do not fly. All true; only the pulse is the search's | **Pass** |
| 3 | Enemy strike prediction | Right on whether it comes; names fighters / anti-aircraft / deck as what decides harm | "I cannot say whether one comes." Names the patrol, the guns at full readiness and the damage of a hit. It came. No prediction was made | **Fail** |
| 4 | Losses | Order of magnitude right and causes named | "Perhaps five to fifteen if a strike goes in", to fighters and guns (and wear, which is not in play). The side lost 6, all shot down | **Pass** |
| 5 | Explanation | The chain of causes matches the engine's `--why` chain; every number quoted is one `explain` gave; no invented mechanism | Search (side event 20, 21) → report (22) → four launches (25 to 28) → attacks (39 to 42) → claims (60, 61); the umpire's chain for the sinking is search 89/90 → report 91 → launch 101 → bombs 131 → sank 162. Every figure it quotes (aircraft, 160 miles, 15.78 and 10.52 points, the deck) is one the side was given. It says plainly it cannot see which hit sank her or the chances (the side is shown neither). One loose figure: "well short of the 50 that makes a ship crippled", the glossary's meaning of crippled, not the objective's (finding L9) | **Pass** |
| 6 | Claims vs truth | Says a claim is a belief, never below the truth, possibly above; does not treat it as score | "Claims are never below the truth and rise with the fire fired … never corrected"; compares the 78 aircraft claimed with the 57 the result counts, and the claimed 14 hits with nothing | **Pass** |
| 7 | Truth and belief | Asserts nothing of enemy truth the side could not know | Every statement of the enemy rests on the side's events or the final result; it says the attackers' composition and causes are hidden | **Pass** |
| 8 | Confusions | Each item it names is listed | Listed below, with the judge's own | Listed |

**Verdict by the procedure's rule** (pass if 5, 6 and 7 pass and at least three of 1 to 4): **not passed**, 5, 6 and 7 passing and two of 1 to 4. The architect may weigh that both misses are of where the enemy would be, which nothing the side is given can show (the opening briefing holds no contact and no enemy force; its mission says only "expected to come down from the north"); neither is a misreading of a rule. The procedure's "judge the reason, not the dice" does not reach them: they are not dice but the fog the game is meant to have. The explanation, the part of the line that the spec asks for in its own words ("explains an outcome"), passed.

**The architect's reading** (the coordinating architect, Nov 13, 2026, on the designer's delegation; added beneath the verdict, which stands as recorded): both failed predictions are about where the enemy is, which the opening briefing cannot show; the three items that test legibility itself, the explanation chain, claims against truth, and truth and belief, pass. The test goes to the designer on this page to accept or not. It is not run again, and the verdict rule is not changed after the result.

### Findings: what confused the model, with a proposed wording fix for each

The architect's rulings, Nov 13, 2026: L1, L4, L9 and L11 are fixed in the glossary and the rulebook in this pull request (below, "What was changed"); L2, L3, L5, L8 and L12 change served text (the briefing and the tools) and go in a second pull request, cut from `main` after this one, with the agent's goldens and the recorded games remade (made: the briefing now tells two groups of one squadron apart, says a base's fuel is for ships, puts the side's event numbers on the attack-on-a-ship line, says aircraft wear is not in play when it is not, and prints "hit, no change" for a system hit that changed nothing; no save, hash or outcome changed); L6 and L7 are put on `docs/open-work.md` for step 3; L10 needs nothing.

| # | What confused it | Where | Proposed fix |
| --- | --- | --- | --- |
| L1 | Whether a search flies in the night pulse ("the files do not say searches stop at night") | Glossary, Pulse and Search | Glossary, Pulse: add "Searches and strikes fly only in the morning and afternoon pulses; the night pulse has no search." |
| L2 | Whether `air_group:11 "VT-3, attack group"` and `air_group:4 "VT-3"` (and 12 and 8) are the same aircraft counted twice (both sessions) | Briefing, AIR GROUPS | Briefing: a group made from part of a squadron says so, for example "VT-3, attack group (7 aircraft of its own; air_group:4 is a separate group)"; or the glossary's Air group: "Two groups may bear one squadron's name; each holds its own aircraft." |
| L3 | Whether Henderson's "fuel 0 t" grounds its aircraft (both sessions) | Briefing, BASES | Briefing: "fuel 0 t (ships' fuel; aircraft do not draw on it)", or the glossary's Stock: "A base's fuel is for ships; aircraft do not use it." |
| L4 | No sighting chance or hit chance for its own search and attacks: "the breakdowns show factors but no result, so I cannot quote them". By design the side is not shown them (decisions 0061 and 0085), but P2 asks for chances and the glossary says the opposite | Glossary, Breakdown: "A side sees it as it sees the event, except a claim's, which it is not shown" | Glossary, Breakdown: "… except a claim's, and except the factors that would tell where or what an enemy really is: a search that sighted shows no distance or chance, and an attack on an enemy ship shows its aircraft but not its chance." |
| L5 | "Attack on ship:1 … 21 aircraft, 2 attacks" against "event 43, 14 aircraft": the line names no events, and the second attack (event 44, 7 aircraft) was not found | Final briefing, ACTION YESTERDAY | Give the line its event numbers, as every other line has: "21 aircraft, 2 attacks [events 43, 44]". |
| L6 | Why the enemy struck at all: event 43 is shown with its causes hidden, and nothing says the enemy needed a contact | Briefing and glossary | Glossary, Hidden causes (new): "An event caused by the enemy's orders or reports shows no causes; an enemy strike always rests on a contact the enemy held." |
| L7 | What a task force's mission does ("carrier" in the briefing; "no rule reads it yet") (both sessions) | Briefing, TASK FORCES; glossary, Mission | Drop the task force mission from the briefing line until a rule reads it, or print it as "(a label; no rule reads it)". |
| L8 | Expected losses to wear "and the rate is not given": wear is not in play in scenario 5, but the briefing says so only of fatigue and base supply | Briefing, WARNINGS | Add "Aircraft wear: not in play" beside "Air groups tired: fatigue is not in play" when `air_upkeep.toml` is not loaded. |
| L9 | "Crippled" has two meanings: the objective counts "ships left afloat with propulsion or flight deck out" as crippled (the result's "ships crippled"), the glossary says "A ship with flotation damage of 50 or more". The model used the glossary's | Glossary, Crippled; chapter 6 | Glossary, Crippled: "In a fight at sea: flotation damage of 50 or more (it counts toward break-off). In the objectives, 'ships crippled' are ships afloat with propulsion or flight deck out." Or rename the objective "ships out of action". |
| L10 | The search range: it took the orders' 200 miles for the maximum until the refused probe said 397 | Orders (the Sonnet game's reason text), not the game | None needed: the briefing gives "search range 397 miles" and the refusal says it. Listed because the model named it |
| L11 | How a contact's kind is shown on a report, and whether one report can tell a carrier apart (session 1); the contact was later "taken for destroyer", then "battleship" | Glossary | Glossary, Contact: add "Each report gives a kind, which may be wrong; a later report of the same contact may change it." |
| L12 | Judge's own, not named by the model: "main battery working to working" in LOSSES YESTERDAY, for a system hit that changed nothing | Final briefing, Ship damaged | Print a hit that changes no system as "main battery hit, no change", as the umpire's text does, or leave it out of the summary line. |

## Test 3: the rulebook-only hand calculations

### The form

Defined here, as the spec asks, since the step 1 run left no record. Four sittings, each a fresh Haiku session given only `docs/rules/` (every file), `docs/glossary.md`, two scenario files (comment lines taken out, since they state answers) and the data files they name: the frozen figures (`data/reference/step1`) altered exactly as each scenario's test alters them, and the maps. Where a question needs a share or a count other than the file's, it says so in words, as the tests do by editing text. Scenario 17a has no file; its case 1 text was written out from the test's `scenario_text`. The copies were checked by running them (scenario 16's gives its pinned outcome). The model was not given tests, decisions or code, and each answer lists the files it read, all inside its folder. Details and the prompts: `closing-tests/rulebook-sittings.md`.

Every expected number below is from an existing hand calculation in a test file's doc comment, the architect's.

| Sitting | Scenarios | Rules | Source of the expected numbers |
| --- | --- | --- | --- |
| 1 | 16, 15b | 0058; a force not ready firing guns and torpedoes, with experience: 0064, 0068, 0071 | `scenario_16_all_together.rs` §1–2; `scenario_15b_unready_torpedoes.rs`; `scenario_15a_unready_fire.rs` (the gun shots) |
| 2 | 17a (case 1), 18 | Whom a ship fires at, 0078; the other side's torpedo tubes, 0083 | `scenario_17a_target_choice.rs` cases 1 and 7 and its points table; `scenario_18_torpedo_sides.rs` cases A, A', C, D |
| 3 | 19a, 19b | Guns before and after the attack, 0079; whom the patrol fires at, 0080 | `scenario_19a_guns_and_the_attack.rs` cases 1 and 2; `scenario_19b_whom_the_patrol_fires_at.rs` cases (a), (b), (d), (f) |
| 4 | 19c, 04a | The patrol's fire before and after the attack, 0087 (with 0079 and 0080); step 1 as the control | `scenario_19c_patrol_after_the_attack.rs` cases 1, 2 and 4; `scenario_04a_night_action.rs` (the meeting, who sights whom, a salvo) |

### The marking

Exact: equal to the sixth decimal, by the route the rulebook gives. "Different route": the number differs only in the last digit because the model cut where the hand calculation rounds.

**Sitting 1 (16 and 15b): every number exact.**

| Q | Expected | Model | Mark |
| --- | --- | --- | --- |
| 16: entry into (2,-4) | 180 miles, 05:15 | 180, 05:15 | Exact |
| 16: alert, ready | Allied not alert, not ready; Japanese alert, ready; readiness fixed at the fight's start | Same | Exact |
| 16: experience factors; round 1 sighting | 0.6, 1.4; 0.15, 0.7 | Same | Exact |
| 16: gun shots | Japanese 120; Allied 60; Allied battery degraded 30 | Same | Exact |
| 16: chances, Japanese guns / torpedoes | 0.042, 0.063, 0.084 / 0.056, 0.084 | Same | Exact |
| 16: chances, Allied guns / torpedoes; fire control degraded | 0.018, 0.027, 0.036 / 0.024, 0.036; 0.009 | Same | Exact |
| 16: expected hits a salvo; torpedo shots | Allied 1.08, Japanese 5.04; 8 each | Same | Exact |
| 15b: B ready; gun shots; hits | Not ready; 60; 1.8 | Same | Exact |
| 15b: torpedoes, sound / degraded; afterwards | 4, 0.16 / 2, 0.08; spent | Same | Exact |
| 15b: at battle readiness | 8, 0.32 | Same | Exact |
| 15b: by day, alerted before / not | 8, 0.16 / 4, 0.08 | Same | Exact |

**Sitting 2 (17a and 18): every number and target exact.**

| Q | Expected | Model | Mark |
| --- | --- | --- | --- |
| 17a case 1: guns and torpedoes of the eight ships | C1 5/5, C2 5/5, D3 6/5, D4 7/5, C5 1/1, D6 3/2, D7 4/1, D8 3/2 | Same (counting places from 0; it also gave the from-1 reading) | Exact |
| 17a: a salvo | Cruiser and destroyer guns 120 shots, 6; torpedoes 8, 0.16 | Same | Exact |
| 17a: points a hit (nine) | 5, 20, 6.666666; 1.25, 5, 1.666666; 50, 200, 66.666666 | Same | Exact |
| 17a case 7 | C2 guns C4, C3 guns C5; torpedoes C2→C5, C3→C6; Y's guns C2, C3, C2; torpedoes C2, C3, C2 | Same | Exact |
| 18 A: rounds 1 and 2 | 4 tubes, 4 shots, 0.04, 0.16; then 4, 4, 0.04, 0.16; spent | Same (noting the target may be lit by round 2) | Exact |
| 18 A': surprised | 0.06, 0.24 | Same | Exact |
| 18 C: not ready | 4 tubes, 2 shots, 0.08, each round | Same | Exact |
| 18 D: mounts degraded | 4 tubes, 2 shots, 0.08, each round | Same | Exact |
| 18: a torpedo hit | 50 points, system chance 1 | Same | Exact |

**Sitting 3 (19a and 19b): every number exact but two, right by a different route.**

| Q | Expected | Model | Mark |
| --- | --- | --- | --- |
| 19a: launches and escorts | 12 bombers at the contact; group 3's 4 escort (share 0) | Same | Exact |
| 19a: the rolls | 4 × 0.3 = 1.2; 4, 1.2; 2, 0.6; guns 20, 1.0 | Same | Exact |
| 19a: b = 1, 0.5, 0.33, 0 | Guns 20/0, 10/10, 6/14, 0/20; attack shots 10.4, 10.9, 11.1, 11.4; bomb hits 2.08, 2.18, 2.22, 2.28; lost to guns 1.0 | Same | Exact |
| 19a: a bomb hit | 10 points | 10 | Exact |
| 19a case 2: chance it attacks | 0.358486, 0.598737, 1 | 0.358485, 0.598736, 1 | **Different route**: 0.95^20 = 0.3584859… and 0.95^10 = 0.5987369…; the model cut, the hand calculation rounds |
| 19a case 2: bomb hits; lost | 0.071697, 0.119747, 0.2; 0.641514 | Same | Exact |
| 19b (a), (b), (f), (d) at each s | The tables of the doc comment: D, R, X, the shots each way and their expected hits; (d) shares 5 and 5 | Same, every row | Exact |

**Sitting 4 (19c and 04a): 19c every number exact; 04a every number exact but three: two by a different route, one wrong.**

| Q | Expected | Model | Mark |
| --- | --- | --- | --- |
| 19c case 1 | 4 / 4 / 9 shots, 1.2 / 1.2 / 2.7 | Same | Exact |
| 19c p = 1, 0.4, 0 | 9/0, 3/6, 0/9; attack 9.3, 11.1, 12; bombs 1.86, 2.22, 2.4; lost 2.7 | Same | Exact |
| 19c case 2 | 5/0, 2/3, 0/5; attacks 0.16807, 0.49, 1; bombs 0.033614, 0.098, 0.2; lost 0.83193 | Same | Exact |
| 19c case 4 | Patrol 6/0, 2/4, 2/4, 0/6; guns 20/0, 20/0, 0/20, 0/20; attack 9.2, 10.4, 11.4, 12; bombs 1.84, 2.08, 2.28, 2.4 | Same | Exact |
| 04a: the meeting | 180 miles, 11.25 hours, 45 t a cruiser | Same | Exact |
| 04a: sighting | 0.5, 0.25; 0.125, 0.375, 0.125, 0.375 | Same | Exact |
| 04a: never sights | 0.002781 | 0.002780 | **Different route** (0.375^6 = 0.0027809…, cut) |
| 04a: A has a free round | 0.598331 | 0.598331 | Exact |
| 04a: B has a free round | 0.199444 | 0.199443 | **Different route** (0.1994438…, cut) |
| 04a: both fire from the start | 0.199444 | 0.125 | **Wrong.** It read "from the start" as "both sight in round 1" and worked 0.125; the hand calculation means "both sight first in the same round", which the model also worked (0.199443) and set aside. The rulebook defines neither "free round" nor "both fire from the start" (it said so); the terms came into the question from the test. Cause: the rulebook is silent (finding R5), and the question used the test's words |
| 04a: salvoes | 120 shots, 3.6 / 5.4 / 7.2; fire control degraded 1.8; torpedoes 0.32 / 0.48 | Same | Exact |
| 04a: points and system chance | Gun 5 and 0.3; torpedo 50 and 1 | Same | Exact |
| 04a: at least one torpedo hit, free round | 0.390431 | 0.390431 | Exact |

**In all:** every number exact but five: four right to the fifth decimal by cutting where the hand calculation rounds, one wrong. Every number of a step 2 rule is exact but two derived chances of 19a case 2, which differ in the sixth decimal only. The one miss and the other two rounding differences are in the step 1 control.

**Against the procedure's pass rule** ("every listed number matches, and every finding is fixed or ruled"): as the first run came out, not a pass, and it is recorded as it came out. The architect then ruled (R6) that a chance worked by hand over several draws is written cut, as the engine cuts, so the model's four cut values are the right form; had the findings fixed; and had the control sitting run again (below, "The control sitting run again, after the fixes").

### Findings: where the rulebook is unclear or wrong, with a proposed wording fix

"Unclear" means the model had to infer, whether or not its number came out right. The architect's rulings, Nov 13, 2026: R1, R2, R3, R4, R5, R7 and R8 are fixed in this pull request as proposed (below, "What was changed"); R6 is ruled as a rule of the hand calculations, written into `docs/engineering/roles-and-tests.md`; R9, R10 and R11 need nothing.

| # | Where | The line | Unclear or wrong | Proposed fix |
| --- | --- | --- | --- | --- |
| R1 | Ch. 4, Who fires | "Among them it takes the ship in place (p modulo their number) … where p is the firing ship's place among the ships of its own kind in its force" (and the same for i) | Unclear: does a place count from 0 or 1? The model guessed 0, right, and worked both | "… where p is the firing ship's place, counting from 0 in ship ID order, among the ships of its own kind in its force" and "i … its place in its force, counting from 0". |
| R2 | Ch. 4, Ready | "The chance to hit, sighting, surprise, the alarm and breaking off are unchanged." | Unclear: read as saying a force not ready sights as if alert, against chapter 3's `not_alert_sighting_factor` | "The share changes the shots only. The chance to hit, sighting (which reads whether a force is alert, chapter 3, not whether it is ready), surprise, the alarm and breaking off are as they would be without it." |
| R3 | Ch. 3, Starting contacts | "Each entry names a side, a hex, a ship count (at least 1), a kind and an age in pulses." | Unclear: the scenarios give no age; the loader takes 0 (`age_pulses`, default 0) | "… and an age in pulses (`age_pulses`, 0 if not given)." |
| R4 | Ch. 4, Named systems | "each working, degraded or out" | Unclear: the state at the start is not said | Add: "A ship starts with every system working unless the scenario says otherwise." |
| R5 | Ch. 4, Sighting and Surprise | None: "free round" and "both fire from the start" appear nowhere in the book or the glossary | Unclear: the terms of every hand calculation of a night fight are not the book's (the one wrong answer) | Glossary, Free round (new): "A round in which one force fires and the other, surprised, does not." Chapter 4, after Surprise: "A fight therefore opens in one of three ways: one force has the free round; both sight first in the same round and fire from it on ('both fire from the start'); or neither ever sights and they pass unseen." |
| R6 | Ch. 1, Fixed-point numbers, and the hand calculations | "cut … at six decimals" | Not wrong, but the hand calculations round chances worked over several draws (0.358486, 0.598737, 0.002781, 0.199444), which are not engine quantities and which the book's cut does not reach | A ruling, not a rulebook change: either the hand calculations write such chances cut, or the book adds to Fixed-point numbers: "A chance worked over several draws by hand (at least one hit in n, a fight that never opens) is not an engine quantity; the book does not fix its last digit." |
| R7 | Ch. 1, How a chance is rolled | One draw for one roll | Unclear: that several shots are independent draws (so 0.95^k) is inferred from the keyed streams, not said | Add to chapter 4, Each shot is rolled: "Each shot is its own draw, so n shots at chance p miss together with chance (1 − p)^n." |
| R8 | Ch. 5, Coming home to no deck | "A degraded deck takes them" | Unclear to the model whether "takes" means lands them | "A degraded deck still lands them; only a deck that is out does not." |
| R9 | Ch. 1, The clock | "in the scenario's reference zone" | Unclear in the scenarios, which state no zone; `differences.md` row 10 says why. Ruled clear by the book (its differences row) | None; or a pointer from the line to `differences.md` row 10. |
| R10 | Ch. 4, Torpedo shots | "After any other salvo, torpedoes_ready and torpedoes_one_side_fired are both cleared" | Inferred correctly that a class with both figures equal fires once; the book is clear enough | None, or "(a class whose tubes all bear on one side fires them all at once and is then spent)". |
| R11 | Ch. 3, Alert from belief, with ch. 4, Ready | "A force is ready if its readiness is battle … or if it was alert before the present pulse began." | Inferred correctly that staying alert into the next pulse needs a fresh contact near; clear | None. |

**Two slips of the tester's, not the book's.** The altered copies of the frozen figures kept their `# FROZEN` header, so a share of 0.5 sat beside the book's "1 in the frozen step 1 file" and sitting 2 had to say so; the prompt should have said the figures were altered copies. And a question of sitting 1 said "lit (guns only)", where the book's lit factor applies to torpedoes too; the model said so and answered both. Neither changed a number, and neither is run again (the architect's ruling). The 04a question's use of the hand calculations' own terms, which the book did not define, was the tester's too; the book now defines them (R5) and the rerun asked for them as defined.

**Hand calculations that round a derived chance** (R6). They are not edited: nobody changes an expected number, and none of these is asserted by a test; each lies only in a doc comment. Found in the doc comments this test read and in those that repeat them; this is not a search of every test.

| File | Written | Cut |
| --- | --- | --- |
| `tests/scenario_19a_guns_and_the_attack.rs`, case 2 | 0.95^20 = 0.358486; 0.95^10 = 0.598737 | 0.358485; 0.598736 |
| `tests/scenario_04a_night_action.rs`, how a fight opens | 0.375^6 = 0.002781; B's free round, and both from the start, 0.199444 | 0.002780; 0.199443 |
| `tests/scenario_15a_unready_fire.rs`, `tests/scenario_04c_readiness.rs` | 4a's 0.199444 and 0.002781, repeated | 0.199443; 0.002780 |
| `tests/scenario_14a_experience.rs`, how a fight opens | 0.255^6 = 0.000275; both from the start 0.140901 | 0.000274; 0.140900 |

### What was changed in this pull request

Wording only, each saying what the code already does; no rule changes its meaning.

| Finding | File | Change |
| --- | --- | --- |
| L1 | Glossary, Pulse | Air searches and strikes fly only in the morning and afternoon; the night's search phase is empty and no strike is launched in it |
| L4 | Glossary, Breakdown | What a side is shown, as chapter 1 already has it (`side_events.rs`: `own_factors`, `side_breakdown`): a whole event's breakdown less any raw draw; a claim's not at all; of an event cut of more than an enemy's name, only its own factors and no result, with the search, attack, fighters', guns' and salvo cases |
| L9 | Glossary (Crippled, Heavily damaged (new), Break off, Straggler, Dead in the water); chapters 1, 2, 4 and 6; `differences.md` row 3 | One meaning of crippled, the objective's: afloat with propulsion or flight deck out. Flotation damage of `crippled_flotation` or more is now "heavily damaged" |
| L11 | Glossary, Contact | A report's hex, count and kind can each be wrong, and a later report of the contact replaces them |
| R1 | Chapter 4, Who fires | Places p and i are among the ships afloat, in ship ID order, counted from 0, with an example (`surface.rs`: `choose_targets`, over the force's ordered set of ships) |
| R2 | Chapter 4, Ready | Being ready changes the shots only; sighting reads alert, not ready |
| R3 | Chapter 3, Starting contacts | `age_pulses`, 0 when not given (`sim-data`, `ContactSpec`) |
| R4 | Chapter 4, Named systems | A ship starts with every system working unless its `[[ship]]` entry sets one (`sim-data`, `ShipSystems::ALL_WORKING`) |
| R5 | Chapter 4, Surprise; glossary, Free round and Both firing from the start (new) | The free round, and the four ways a fight between two engaging forces opens |
| R6 | `docs/engineering/roles-and-tests.md`, Tests | A chance worked by hand over several draws is written cut to six places |
| R7 | Chapter 4, Each shot is rolled | n shots at p all miss with chance (1 − p)^n |
| R8 | Chapter 5, Coming home to no deck | A degraded deck still lands them |
| L6, L7 | `docs/open-work.md` | One line each, for step 3 |

**Served text that says "crippled" in the old, flotation sense,** which would have to change for the word to have one meaning everywhere (not changed here; listed as the architect asked):

- The break-off event's text, "round N: task_force:X broke off, S of its ships sunk or crippled" (`crates/sim-core/src/events.rs`, the break-off text), shown to a side.
- The break-off breakdown factor `sunk_or_crippled` (`crates/sim-core/src/surface.rs`), shown to the side whose force broke off.
- The figure `surface_combat.toml: crippled_flotation` and its loader check's message "crippled flotation" (`crates/sim-core/src/refdata.rs`): a data key, not served to a player.

The objective `ships_crippled` (the briefing, `crates/sim-agent/src/text.rs`; `briefing.schema.json`) already has the one meaning. The batch file's `crippled_runs` (`sim-cli batch`, the umpire's) was not checked for which meaning it counts.

### The control sitting run again, after the fixes

Run on the architect's ruling, **after the rulebook and glossary fixes above were made**: a fresh Haiku session, the same folder as sitting 4 with `rules/` and `glossary.md` replaced by the fixed ones, and the same questions, but question 7 asks for the free rounds and "both fire from the start", "each as the rulebook and the glossary define these terms". The answer is `closing-tests/rulebook-answers-4-rerun.md`.

| Q | Expected (as the doc comment writes it) | First run | Rerun | Mark of the rerun |
| --- | --- | --- | --- | --- |
| 19c, every question | As the first run's marking | Exact | Every number the same | Exact |
| 04a: the meeting, sighting, salvoes, points, system chances, 0.390431 | As the first run's marking | Exact | Every number the same | Exact |
| 04a: never sights | 0.002781 | 0.002780 | 0.002780 | Exact in the form R6 asks (the doc comment rounds) |
| 04a: A has a free round | 0.598331 | 0.598331 | 0.598331 | Exact |
| 04a: B has a free round | 0.199444 | 0.199443 | 0.199443 | Exact in the form R6 asks |
| 04a: both fire from the start | 0.199444 | 0.125 (wrong) | 0.199443, "both sight in the same round, round 1 or later" | Exact in the form R6 asks |

So after the fixes the control is exact on every number. The rerun named one new place it had to infer, R12. **Closed:** on the architect's ruling the proposed sentence was put into chapter 3, Starting contacts, in the pull request of the briefing's wording fixes.

| # | Where | The line | Unclear | Proposed fix |
| --- | --- | --- | --- | --- |
| R12 | Ch. 3, Starting contacts, and Ageing and dropping | "Events are recorded on day 0, with no pulse", beside "each contact that was not made or refreshed in that pulse gains one pulse of age" | Whether a starting contact is aged at the end of the night pulse like any other. It is: `tests/scenario_19a_guns_and_the_attack.rs` says a contact a side starts holding at age 0 "is aged at the end of the night pulse, so it is 1 pulse old at the morning's decision". The model took age 1 and noted that 0 gave the same answer | Starting contacts: "From then on it is aged like any other contact: one entered at age 0 is 1 pulse old after the night pulse of day 1." |
