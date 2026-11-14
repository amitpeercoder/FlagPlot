# Documents housekeeping, Nov 14, 2026

An audit of every document under `docs/` and every README in the repository, made on Nov 14, 2026 against `main` at commit 72e66e4, and the cleanup that could be done while another team edits the rulebook, the glossary, the open-work list, the commands page, the decisions index, the step 3 spec, `CLAUDE.md` and decisions 0095, 0097 and 0099. Those files are not changed here. Section 8 lists the edits proposed for them.

Line numbers are those of commit 72e66e4 unless a line says otherwise. "Checked" means checked against the code, the data, `git log` or `ls`; "read" means taken from the text only. Nothing was run: no test, no scenario, no command but `grep`, `git` and `ls`.

What this pass changed is in section 10.

## 1. Inventory

217 Markdown files under `docs/` and eleven READMEs (one of them, `docs/decisions/README.md`, under `docs/`). Standing: **current** (says what is true or planned now), **historical** (kept as a record of what was done or found), **superseded** (by what), **orphaned** (no other file names it by path or name).

### Top level of `docs/`

| File | What it is for | Standing |
| --- | --- | --- |
| `pillars.md` | What the game is; the five pillars | Current. Its open questions at 114 and 117 are partly answered by the step 3 spec (section 4) |
| `glossary.md` | One meaning for each word | Current; three entries stale (section 5) |
| `open-work.md` | The architect's list of what is open and next | Current; several lines stale (section 8) |
| `commands.md` | What each command does | Current: flags checked against `crates/sim-cli/src/main.rs`. Grown into a changelog (section 9) |
| `step1-spec.md` | The spec of step 1 | Historical |
| `step2-spec.md` | The spec of step 2, closed Nov 13, 2026 | Historical; two status rows corrected here |
| `step3-spec.md` | The spec of step 3, confirmed Nov 14, 2026 | Current |
| `complexity-review-after-scenario-5.md` | The review of Nov 11 that decisions 0032 and 0033 acted on | Historical; named only by 0032:3 and 0033:3 |
| `housekeeping-nov-14.md` | This report | Current |

### `docs/design/`

| File | What it is for | Standing |
| --- | --- | --- |
| `ground-combat.md` | The plan of land units and ground combat, stages 0 to 4 | Current: stages 1 to 3 built, stage 4 (decision 0093) not. Stale lines noted here |
| `planning.md` | Planning an operation and its trial | Current, not built; two stale lines noted here |
| `intelligence.md` | Layers of intelligence | Current, not built; two stale lines noted here |
| `storytelling.md` | The story of the day, giving commands | Current, not built |
| `economy.md` | Materials and economy | Current, not built |

### `docs/engineering/`

All seven are current and named by `CLAUDE.md`: `data-rules.md`, `determinism.md` (checked against `rng.rs` and `hash.rs`), `layout.md` (one claim contradicts `tests/Cargo.toml`, section 4), `roles-and-tests.md` (two lines stale, section 4), `rule-complexity.md`, `traceability.md`, `truth-and-belief.md`.

### `docs/rules/`

`README.md`, the seven chapters and `differences.md`: current, the rulebook. Not edited here. Section 5 has its gaps.

### `docs/decisions/`

`README.md` is the index. Every record below is current: a record is never archived, and a record whose items later records replaced is still the place those items are read. "Not built" records are confirmed plans.

| Record | Title | Standing |
| --- | --- | --- |
| 0001-day-boundary | A game day runs 18:00 to 18:00 in the scenario's reference zone | current |
| 0002-fixed-point | Fixed-point numbers | current |
| 0003-random-numbers | Random numbers | current |
| 0004-state-hash | State hash | current |
| 0005-movement | Movement | current |
| 0006-fuel-model | Fuel model | current |
| 0007-truth-and-belief | Truth and belief in the type system | current |
| 0008-orders | Orders | current |
| 0009-scenario-files | Scenario files and saved games | current |
| 0010-test-layout | Where scenario tests live | current |
| 0011-cargo-handling | Cargo handling at ports | current |
| 0012-search-and-contacts | Air search, reports and contacts | current |
| 0013-random-keys-for-pairs | Random number keys for a pair of entities | current |
| 0014-surface-combat | Surface combat | current |
| 0015-order-of-battle-facts-format | Order-of-battle facts: format and rules | current |
| 0016-systems-and-stragglers | Damage to named systems, and stragglers | current |
| 0017-map-and-pathfinding | The real map, and pathfinding | current |
| 0018-replay-viewer | A read-only replay viewer | current |
| 0019-savo-island-choices | Savo Island, the designer's choices | current |
| 0020-figures-from-facts | Game figures for real ships, derived from facts | current |
| 0021-readiness-and-alarm | Readiness, alertness and alarm | current |
| 0022-meeting-and-breaking-off | Meeting in time order, and breaking off | current |
| 0023-savo-island-ships | The Savo Island ships: exceptions and assumptions | current |
| 0024-savo-island-scenario | The Savo Island scenario: what is in it and what is left out | current |
| 0024-scenario-4-savo-island | (no title: two notes of Nov 13, 2026 on scenario 4) | current, no index row, linked by nothing |
| 0025-carrier-air-groups | Air groups on carriers | current |
| 0026-air-strikes | Air strikes on ships, and the flight deck | current |
| 0027-fighters-and-anti-aircraft-fire | Fighters, anti-aircraft fire and aircraft losses | current |
| 0028-a-strike-that-finds-nothing | A strike that finds nothing drops the contact | current |
| 0029-figures-for-carriers-and-aircraft | Figures for carriers, anti-aircraft guns and aircraft, derived from facts | current |
| 0030-escorts-have-a-radius | Escorts have a radius | current |
| 0031-eastern-solomons-scenario | The Eastern Solomons scenario: what is in it and what is left out | current |
| 0032-cuts-after-the-complexity-review | What the complexity review cut and merged | current |
| 0033-hits-revised | Three changes to how hits are worked | current |
| 0034-raids-on-a-base | Raids on a base | current |
| 0035-wear-repair-and-operational-losses | Wear, repair and operational losses | current |
| 0036-fatigue-and-rest | Fatigue and rest | current |
| 0037-fatigue-at-battle-readiness | Fatigue at battle readiness | current |
| 0038-pilots-and-replacements | Pilots and replacements | current |
| 0039-base-supply-consumption | Base supply consumption | current |
| 0040-air-distance-is-a-straight-line | Air distance is a straight line | current |
| 0041-air-groups-that-arrive | Aircraft that arrive on a day | current |
| 0042-rabaul-against-guadalcanal | The Rabaul against Guadalcanal scenario | current |
| 0043-determinism-scenario | The determinism scenario, saved games and replay | current |
| 0044-the-soak-scenario | The soak scenario | current |
| 0045-refuelling-in-port | Refuelling in port | current |
| 0046-fuel-as-a-cargo | Fuel as a cargo | current |
| 0047-what-a-side-may-know | What a side may know of what happened | current |
| 0048-briefing-and-queries | The briefing and the queries | current |
| 0049-measure | Measuring | current |
| 0050-playing-a-side | Playing a side | current |
| 0051-mcp-server-and-scenario-9 | The MCP server and scenario 9 | current |
| 0052-what-step-1-leaves-out | What step 1 leaves out | current |
| 0053-viewer-measuring-and-side-views | The viewer measures, and shows a side's view | current |
| 0054-objectives-and-results | Objectives and results | current |
| 0055-after-the-architecture-review | After the architecture review | current |
| 0056-claims-from-the-air | Claims from the air | current |
| 0057-hash-version-2-and-run-inputs | The state hash at version 2, and results that say what they were run on | current |
| 0058-ships-charged-straight-miles | Ships are charged straight-line miles | current |
| 0059-aircraft-that-cannot-land-where-they-took-off | Aircraft that cannot land where they took off | current |
| 0060-a-claims-breakdown-is-the-umpires | A claim's breakdown is the umpire's | current |
| 0061-a-search-that-sighted-is-cut | A search that sighted is shown without its distance or its chance | current |
| 0062-what-batch-records-for-each-run | What `batch` records for each run | current |
| 0063-strike-target-priority | Strike target priority as an order | current |
| 0064-experience | Experience | current |
| 0065-a-sides-events-name-no-enemy-unit | A side's events name no enemy ship, task force or air group | current |
| 0066-land-units-carried-and-fed | Land units, carried by sea and fed from a base | current |
| 0067-two-objectives-that-could-be-gamed | Two objectives that could be gamed | current |
| 0068-a-force-not-at-battle-readiness-fires-with-part-of-its-guns | A force not at battle readiness fires with part of its guns | current |
| 0069-ship-repair | Ship repair | current |
| 0070-rates-of-fire-as-achieved | Rates of fire as achieved, not as designed | current |
| 0071-a-force-not-ready-fires-part-of-its-torpedoes | A force not ready fires part of its torpedoes | current |
| 0072-cape-esperance-scenario | The Cape Esperance scenario: what is in it and what is left out | current: built and run once (status line still says plan) |
| 0073-eastern-solomons-search-orders | The Japanese search at the Eastern Solomons, from the record | current |
| 0074-torpedo-tubes-that-bear-on-one-side | A class's torpedo tubes are the tubes that bear on one side | current |
| 0075-a-search-radius-of-its-own | An aircraft type has a search radius of its own | current |
| 0076-how-long-surprise-lasts | How long surprise lasts: no rule | current: no rule |
| 0077-which-radar-sets-count-for-sighting-ships-at-night | Which radar sets count for sighting ships at night | current |
| 0078-which-enemy-ship-a-ship-fires-at | Which enemy ship a ship fires at | current |
| 0079-anti-aircraft-fire-and-the-attack | Anti-aircraft fire and the attack | current |
| 0080-whom-the-patrol-fires-at | Whom the patrol fires at | current |
| 0081-eastern-solomons-orders-from-the-record | The Eastern Solomons scenario's remaining orders, from the record | current |
| 0082-rabaul-orders-from-the-record | Rabaul against Guadalcanal: the raid days and the dive bombers' searches from th | current |
| 0083-the-other-sides-torpedo-tubes | The other side's torpedo tubes | current |
| 0084-what-the-agreements-show-and-do-not | What the agreements show, and what they do not | current |
| 0085-what-a-side-is-told-of-a-fight-and-other-leaks | What a side is told of a fight, and other leaks an outside review found | current |
| 0086-numbers-a-side-is-given | The numbers a side is given | current |
| 0087-the-patrols-fire-and-the-attack | The patrol's fire and the attack | current |
| 0088-out-of-the-campaign | Ships out of action, counted by the time they are away | current |
| 0089-after-a-second-outside-review | After a second outside review of the tuning | current |
| 0090-the-fight-ashore-and-a-base-that-changes-side | The fight ashore, and a base that changes side | current |
| 0091-rebasing-an-air-group | Rebasing an air group by order | current |
| 0092-shells-bombs-and-the-enemy-ashore | Shells and bombs on a base and its men, and what a side knows of the enemy ashor | current |
| 0093-the-landings-of-august-1942 | The landings of August 1942 and the first attacks on the airfield, as a scenario | current: confirmed, not built |
| 0094-marching-overland | Marching overland | current |
| 0095-preparing-a-landing | Preparing a landing | current: confirmed, not built |
| 0096-a-force-that-acts-and-steams-on | A force that acts at a place and steams on | current |
| 0097-the-quality-of-a-land-unit | The quality of a land unit, and training | current: confirmed, not built |
| 0098-what-a-land-unit-is-good-at | What a land unit is good at | current: confirmed, not built |
| 0099-units-that-work-and-what-they-build | Units that work, and what they build | current: confirmed, not built |
| 0100-step-3-stage-0 | Step 3, stage 0: the ground the staff stands on | current: confirmed, not built |

### `docs/validation/`

| File | What it is for | Standing |
| --- | --- | --- |
| `step2-close.md` | Step 2's close: counts, named misses, the designer's decisions | Historical record of Nov 13; hand-off lines noted here |
| `baseline-reading.md` | The architect's running reading of every report, Nov 12 and 13 | Historical; title no longer says what it holds (section 9) |
| `savo-island-close.md`, `eastern-solomons-close.md`, `rabaul-guadalcanal-air-close.md` | Each battle's closing report, state version 8 | Current; must not move (`docs/research/oob/history-on-the-scale.md` cites their lines) |
| `cape-esperance-check.md` | The one run of the check, Nov 13 | Current; must not move, same reason |
| `savo-island-baseline.md`, `eastern-solomons-baseline.md`, `rabaul-guadalcanal-air-baseline.md` | The first report of each battle, remade | Superseded by the close reports; kept in place, named by `step2-spec.md` and several decisions |
| `savo-island-r1.md`, `savo-island-r2.md`, `savo-island-after-0083.md`, `eastern-solomons-r4.md`, `rabaul-guadalcanal-air-rabaul-r1.md` | Reports of single rounds | Superseded by the close reports; kept in place, named by `step2-spec.md`, `step2-close.md` or decisions 0083 and 0084 |
| `passages-baseline.md` | The only passages report | Current |
| `closing-tests.md` | The three closing tests of step 2 | Historical |
| `closing-tests/legibility-*.md` (3), `rulebook-sittings.md`, `rulebook-answers-1.md` to `-4.md`, `-4-rerun.md` | The test's game, prompts and answers | Historical. `-2` and `-3` are named only as the range "1 to 4" (`closing-tests.md:11`), not orphans in effect |
| `archive/` (9 files) | Reports of single rounds | Archived here (section 7) |
| `tuning/` (13 files) | Each tuning round's record, and the two pages of where every figure stands | Historical, except `air-figures-closed.md` and `surface-and-search-figures-closed.md`, which are current (values checked against `data/reference/`). None may move: `data/reference/*.toml`, `data/derivation/rules.toml`, scenario 6 and `crates/sim-data/src/reference.rs` name them |

### `docs/research/`

| File | What it is for | Standing |
| --- | --- | --- |
| `witpae-manual-index.md` | A map of a game manual's sections, for mechanics only | Historical; holds no figures (checked by grep) |
| `scenario-9-first-round/README.md` | The first round of agent games | Historical; named by decision 0051 and the recorded games' README |
| `oob/achieved-fire-rates.md`, `aircrew-fates.md`, `ditching-and-rescue.md`, `experience.md`, `fuel-fits-and-alarm.md`, `japanese-air-records.md`, `land-forces.md`, `orders-and-conduct.md`, `outside-research-nov-14.md`, `passages.md`, `strike-ranges.md`, `torpedoes-fired.md`, `troop-lift.md`, `us-side-air-losses.md` | Research behind fact rows still read | Current |
| `oob/air-group-experience-proposal.md` | Air groups' experience; ruled to wait | Current (agrees with decision 0064 and `open-work.md:25`) |
| `oob/history-on-the-scale.md` | The record in the objectives scale's terms | Current, and **orphaned**: nothing names it |
| `oob/air-search.md`, `anti-aircraft.md`, `cape-esperance.md`, `conflicts.md`, `database-design.md`, `eastern-solomons.md`, `eastern-solomons-settlement.md`, `japanese-records-2026-11-12.md`, `outside-research-nov-13.md`, `rabaul-guadalcanal-air.md`, `radar.md`, `savo-island.md`, `savo-island-settlement.md`, `source-policy.md`, `step2-record.md` | Research of a battle or a pass, settled into rows and records | Historical |
| `oob/proposals-2026-11-11.md` | Three proposals of Nov 11 | Superseded in part by decision 0073 |
| `oob/savo-island-choices.md` | Four proposed choices | Superseded by decision 0019 (its own first line says so) |
| `oob/damage-control.md`, `gunnery-fire-control.md`, `sonar.md`, `torpedoes.md` | Notes of Nov 10 | **Orphaned** by path (`damage-control.md` and `torpedoes.md` are named only in prose); none is indexed in `data/oob/README.md` |

### READMEs outside `docs/`

| File | What it is for | Standing |
| --- | --- | --- |
| `README.md` | The front page | Current; its status corrected here |
| `crates/sim-agent/schemas/README.md` | The agent interface's schemas | Current; version drift (section 8) |
| `crates/sim-cli/tests/recorded_games/README.md` | The recorded games of scenario 9 | Current; grown into a log of regenerations (section 9) |
| `data/oob/README.md` | The index of fact files | Current index; its status tables of Nov 10 are stale (section 8) |
| `data/reference/step1/README.md` | The frozen figures of step 1 | Current |
| `data/scenarios/step2-frozen/README.md` | The frozen scenario copies | Current |
| `data/validation/README.md` | The measures and their rules | Current (take list checked against `crates/sim-cli/src/validate/take.rs`) |
| `viewer/README.md` | The replay viewer | Current (replay version 3 checked against `crates/sim-cli/src/export/mod.rs:193`) |

## 2. Statuses that are untrue

Fixed in this pass (each with a dated note on the same line, so no line moved; section 10):

| File:line | What it said | What is true |
| --- | --- | --- |
| `docs/decisions/0094-marching-overland.md:3` | "Not built." | Built Nov 14, 2026, scenario 25, pull request 262; its own "As built" at :98 and index row :104 say so (checked) |
| `docs/decisions/0069-ship-repair.md:3`, `:103` | "Nothing is built." | Built Nov 13, 2026, scenario 20a, pull request 235, `crates/sim-core/src/repair.rs` (checked) |
| `docs/decisions/0006-fuel-model.md:3` | Decision 3 "is not built yet" | Built: `speed_multiplier`, `crates/sim-core/src/movement.rs:74` (checked) |
| `docs/decisions/0004-state-hash.md:23`, `:25` | "Nothing enforces this yet"; hashing data files "a later step" | `name_every_field`, `hash.rs:152`, decision 0055 item 8; saved games, batches and replays carry the file hashes, decision 0057 item 4 (checked) |
| `docs/decisions/0040-air-distance-is-a-straight-line.md:3`, `:45` | "ships later"; ships "to be settled before step 2" | Settled by decision 0058 (read) |
| `docs/decisions/0090-the-fight-ashore-and-a-base-that-changes-side.md:3` | Code "waits for" 0066 and 0091 to merge | Both merged; 0090 itself built (checked) |
| `README.md:9`, `:13`, `:14` | Step 2 "is under way"; figures placeholders "until step 2 sets it"; step 1's gaps listed as if open | Step 2 closed Nov 13, 2026; most figures still placeholders; land units, ground combat, shore bombardment and ship repair built since (checked against scenarios 20a to 25) |
| `docs/step2-spec.md:167`, `:168` | Ship repair "not built"; out of the campaign "Not decided" | Built, scenario 20a; decided and built as ships out of action, decision 0088, scenario 20b (checked) |
| `docs/design/ground-combat.md:47`, `:93`, `:119`, `:167` | GroundCombat subsystem unused; the bombardment mission does nothing; no movement overland; "Step 2 is under way" | Used (`ground_combat.rs`); bombardment built (0092, `bombardment.rs`); marching built (0094, `march.rs`); step 2 closed (checked) |
| `docs/design/planning.md:82`, `:85` | Ground combat "to be built"; step 2 "under way" | Stages 1 to 3 built; step 2 closed (checked) |
| `docs/design/intelligence.md:14`, `:33` | "Two kinds of report"; claims "being built" | A third: the ground report and estimate (0092); claims built (0014, 0056) (checked) |
| `docs/validation/step2-close.md:215`, `:242`, `:249`, `:250` | Repair, out of action and land units "built next"; 0066's items "need review" | All built (scenarios 20a, 20b, 21, 23, 24, 25); the designer agreed 0066's items on Nov 13 (checked). The objectives scale is still to be set |
| `docs/validation/tuning/savo-island-closed.md:3`, `:129`; `r1-experience-gun-strength.md:13`; `r2-surprised-target-factor.md:11`, `:30` | Cape Esperance "to be checked"; the scenario "moves out of `not-run/`" | Run once Nov 13 (`cape-esperance-check.md`); moved in commit 897ba4f (checked) |
| `docs/validation/tuning/rabaul-r3-wear.md:177` | The live report "is still" `rabaul-r1.md` | The closing report is `rabaul-guadalcanal-air-close.md` (checked) |
| `docs/research/oob/passages.md:58`; `experience.md:90` | `oob-check` does not read the file | It does: `crates/oob-check/src/passages.rs`, `training.rs` (checked) |
| `docs/research/oob/land-forces.md:118`, `:127` | No checker for the lift files; transports not derived | Rule `lift` checks the lift files (`lift.rs`); `land_units.csv` still has no checker; `troop-lift.md` adds the transports' capacities (checked) |
| `docs/research/oob/history-on-the-scale.md:3`, `:212`, `:308` | Repair and out of action on branches, `horizon_days` and `ships_out_of_action` not built | Merged and built, decisions 0069 and 0088 (checked) |
| `docs/research/oob/savo-island.md:165`; `savo-island-settlement.md:3`; `source-policy.md:3`; `step2-record.md:146`; `database-design.md:354`; `conflicts.md:559`; `proposals-2026-11-11.md:3` | For Amit to confirm; to be made a record; a proposal; the next item; not built; waits for 0070; nothing settled | Decisions 0019, 0023 and 0015; the Japanese records pages; `oob-derive` (0020); 0070 built, rows 480 and 481 standing as preferred and alternative; section 1c replaced by 0073 (checked) |

Found and not fixed:

| File:line | What it says | What is true | Why not fixed |
| --- | --- | --- | --- |
| `docs/decisions/0072-cape-esperance-scenario.md:3` and index row `README.md:82` | "Status: plan" | Built and run once (scenario 17, `step2-close.md`) | The index row says the same; fixing one would make them differ. Both in section 8 |
| `docs/decisions/0035-wear-repair-and-operational-losses.md:3`, `:16`; index `:45` | Decision 2 "needs review, after the air figures are tuned" | The air figures are closed (`tuning/air-figures-closed.md`); no review of decision 2 is recorded (checked by grep) | A ruling for the architect |
| `docs/decisions/0059-aircraft-that-cannot-land-where-they-took-off.md:91`, `:111` | "Stages 1b, 2 and 3 are not built" | All built (its "As built" sections) | Dated notes of a stage-by-stage record; read as history |
| `docs/decisions/0074-torpedo-tubes-that-bear-on-one-side.md:3` | Built, then "held ... until Savo Island's first tuning rounds" | Built with 0083 | The status line already says built first; the held text is the record of the first version |
| `docs/decisions/0088-out-of-the-campaign.md:192` | "its status line still reads as planned" | The status line at :3 says built | Small; left for the architect |
| `docs/decisions/0066-land-units-carried-and-fed.md:201` | "'with their versions' was not applied; the architect to confirm" | Never confirmed in the record; later records keep schema version 3 | A question for the architect |
| `docs/validation/baseline-reading.md:12`, `:14`, `:40`, `:43`, `:224`, `:382`-`:384` | Questions and next steps of Nov 12 (designer to decide; reports to be remade; 0068 and 0063 next; experience to wait) | Each settled or done (`step2-close.md:200`; the page's own later lines; 0063, 0064, 0068) | A running dated record; its later sections say what happened |
| `docs/research/oob/air-group-experience-proposal.md:162` | "The US Navy has no hours figure at all" | `training_facts.csv` has US Navy rows, none for a 1941-42 fleet squadron (read) | Not certain the rows postdate it |
| `docs/research/oob/experience.md:3`; `passages.md:3` | "86 rows"; "76 rows" | 106 and 186 rows (checked) | Counts of a pass, dated |
| `docs/pillars.md:114`, `:117` | Staff planner approach "Still open"; the plan loop "to be designed in step 3" | `step3-spec.md:254` fixes doctrine and trial; the loop is designed, not built (read) | Whether the pillar's question is closed is the designer's |
| `docs/step1-spec.md:26`, `:225`, `:239` | Ground combat and bombardment deferred; three crates; "eight scenarios" | Built; seven crates and `tests`; nine | Historical spec |
| `docs/design/storytelling.md:113` | "Now, at no cost to step 2: write a diary by hand" | Step 2 closed; no diary found (grep) | A suggestion, not a status |
| `data/`, `crates/` READMEs | See section 8 | | Under `data/` and `crates/` |

## 3. Broken or stale cross-references

A script over every relative Markdown link in the repository found no broken link on 72e66e4, nor after this pass. Paths in backticks were checked by a second script; what it found:

| File:line | Reference | Why stale | Fixed? |
| --- | --- | --- | --- |
| `CLAUDE.md:24` | `data/scenarios/not-run/` | The folder does not exist since commit 897ba4f moved Cape Esperance out. The rule still holds if the folder is made again (CI's loop at `.github/workflows/ci.yml:63` walks only `data/scenarios/*.toml`) | No: section 8 |
| `docs/decisions/0072-cape-esperance-scenario.md:221`, `:267`; `0084:25`; `baseline-reading.md:178` | `data/scenarios/not-run/...` | Historical, past tense except 0072:221, which its own :267 corrects | No: history |
| `docs/design/ground-combat.md:145` | "a new `ground_combat.toml`" | Built as `data/reference/ground.toml` (checked) | Yes |
| `docs/decisions/0095-preparing-a-landing.md:106`, `:107` | "decision 4a"; "Decision 6: a new landing place loses all of it" | No decision 4a; after the revision of Nov 14 these are decisions 5, 6 and 8 to 10 (checked) | No: section 8 |
| `docs/decisions/0096-a-force-that-acts-and-steams-on.md:3` | "'What it builds on': needed before stage 2" | The table's row for 0096 (`step3-spec.md:99`) says "Built"; "decided before stage 2" is at `step3-spec.md:272` (checked) | Yes |
| `docs/decisions/0091-rebasing-an-air-group.md:53` | "Decision 0052, item 7, no longer lists rebasing" | 0052:19 still lists it, with a note at :21 that it is built (checked) | No: says what a record does, a wording for the architect |
| `docs/decisions/0062`:59, `0081`:137, `0070`:144 | `tests/batch.rs`, `tests/validate.rs`, `tests/derive.rs` | Crate-relative: `crates/sim-cli/tests/`, `crates/oob-derive/tests/` (checked) | No: correct, only ambiguous |
| `docs/decisions/0086:68`, `0072:227` | "scenario 17a" | A test, `tests/scenario_17a_target_choice.rs`, with no scenario file; 17 was later used for Cape Esperance | No: correct |
| `docs/validation/closing-tests/rulebook-answers-2.md`, `-3.md` | `reference-17a/`, `reference-18/`, `reference-19a/`, `reference-19b/`, `scenarios/17a_target_choice.toml` | Scratch copies of a sitting, named as such at their lines 3, 184 and 238 | No: not a claim about the repository |
| `docs/research/oob/history-on-the-scale.md:16` | "(decision 0024)" | Two files carry 0024 | No: section 6 |
| `docs/engineering/layout.md` (map-tool row) | "nothing depends on it" | `tests/Cargo.toml` has `map-tool` as a dev-dependency (checked) | No: section 4 |
| `docs/validation/tuning/r3-fighter-hit-chance.md:31`, `:52`, `:101`; `r4-claims.md`; `air-figures-closed.md:117`; decisions 0080:84, 0087 | "the air plan (C3, A2, B19)" | No file of that name in the repository or its history (grep) | No: cannot be resolved |
| `crates/sim-cli/src/main.rs:62` | "decision 0062's file, version 2" | Batch files are version 5 (`batch.rs:358`) | No: code |

No decision number in a "replaced by" note points at the wrong record (checked for every such note in 0001 to 0100).

## 4. Contradictions

None is resolved here. "Code" says what the code does where it was checked.

**Between the rulebook and the code** (all checked in the code by reading; nothing was run):

1. State version. `docs/rules/01-the-day-orders-and-what-a-side-knows.md:603`: "begun by the version string `flagplot-state-v14`", and the same paragraph lists version 15. Code: `crates/sim-core/src/hash.rs:144`, `flagplot-state-v15`.
2. The task force's mission. `docs/rules/02-movement-fuel-and-cargo.md:80` "No rule reads it"; `docs/glossary.md:42` "which no rule reads yet"; `docs/rules/06-objectives-and-results.md:11` "no rule reads either"; `docs/rules/differences.md:20` row 12 "No rule reads it" — against `docs/rules/01:146` and `04:285`, "a force on the bombardment mission fires at an enemy base". Code: `crates/sim-core/src/bombardment.rs:205` reads `force.mission`.
3. A men's-supply `BaseSupplySpent`. `docs/rules/05-air-operations.md:236` "Every other `BaseSupplySpent` has 0 men" against `07-land-units.md:225` and `01:416` (a march's payment carries the men). Code: `march.rs:170-176` records reason March with the men marching.
4. What `men_lost` counts. `docs/rules/06-objectives-and-results.md:32` "unfed, lost with a ship or lost in a fight" against `06:45` and `07:327`, which add men lost to fire. Code: `objectives.rs:519-523` counts all four.
5. Who works cargo. `07-land-units.md:70` "is idle or arrived" against `02:313` "idle, arrived or halted". Code: `cargo.rs:163-167` includes halted.
6. Ground file values. `07-land-units.md:386` "each a whole number or a decimal string" against `march_days_per_hex`, read as a whole number only (`crates/sim-data/src/reference.rs:784`; its test at :2261 refuses the string "3").
7. Numbers of units made in play. `07-land-units.md:36` says numbers of 1,000,000 and above are kept for units made in play; `01:373` and the code (`cargo.rs:287-293`) give a divided land unit the highest number plus one, so no land unit made in play is in that series.
8. Fed share restored. `07-land-units.md:144`: a unit with men aboard and none ashore "has its share set back to 1, and `FedShareRestored` is recorded for it". Code: `ground.rs:86` records it only when the share was not already 1.
9. A base formed by a landing. `07-land-units.md:112`: "A hex that holds a base of the side already, formed or not, forms none." Code: `cargo.rs:102-107` finds the base once, when the participants of the phase are built, and `cargo.rs:324` forms one when there was none; two forces of one side landing in one empty coast hex in the same pulse would each form a base. Found by reading only.
10. Order of the sort. `01:318` and `differences.md:22` row 14: "task forces before air groups"; land units also take orders now (`01:136`, `:181-186`). Code: `resolve.rs:309` sorts by `EntityRef`, which puts land units after air groups (`ids.rs:284-292`).
11. Briefing and feeding. `crates/sim-agent/src/briefing.rs:1326-1344` works days of supply for a hex from one base, the lowest-numbered (citing 0066 item 16), where the rule since 0090 item 24a feeds from every base of the side in the hex (`07-land-units.md`, The men's supply). Read, not run.
12. Figures copied into the book. `docs/rules/README.md:9` says the book never copies a value; `07:319`, `:321`, `:335`, `:372`-`:373`, `04:289`-`:291`, `05:435` give values (read).

**Between documents:**

13. `docs/decisions/0094-marching-overland.md:3` "Not built" against its own :100 and index :104 (fixed here).
14. `docs/decisions/0069-ship-repair.md:3` "Nothing is built" against index :79, `0052:49`, `0088:3`, `open-work.md:9` (fixed here).
15. `CLAUDE.md:120` (in the worktree: "Next, before step 3: the objectives scale set from the record, then stage 4 of ground combat") and `open-work.md:5`-`:11` against `step3-spec.md:107` (the scale is "not a condition of stages 0 to 2", frozen before stage 5) and `step3-spec.md:102`-`:105` (0095 to 0099 needed before stage 3). Two lists of what comes next.
16. `docs/decisions/0093-the-landings-of-august-1942.md:87` "No new rule beyond marching (decision 0094)" against 0099:75-77 (working units and the airfield change the August scenario), `0095:57`, and `step3-spec.md:102`-`:105`.
17. Two meanings of "stage 4": ground combat's (`ground-combat.md:161`, `CLAUDE.md:120`, `open-work.md:11`, `:62`, `0093:3`, `0095:57`, `0099:73`) and step 3's, the trial (`step3-spec.md:276`, `0100:91`).
18. `docs/decisions/0024-savo-island-scenario.md:12`, `:13`, `:19` (three forces, one column of eight, all break off at half) and index :34, against `0024-scenario-4-savo-island.md:12`, `:49` (Yunagi as task force 4; the northern group at 1). Code: `data/scenarios/04_savo_island.toml:44`-`:76` has four forces and break-off levels 0.5, 0.5 and 1 (lines 285, 320, 331). A note pointing to the second file is added to the first here.
19. `docs/decisions/0050-playing-a-side.md:38` "The seven tools of the spec" against its own list of nine and the server's nine (read).
20. `docs/decisions/0008-orders.md:17` item 7 (an order may fail in execution) against `differences.md:17` row 9 (no failure in execution exists). Ruled; note added here.
21. `docs/decisions/0001-day-boundary.md:11`, `:37` (a UTC anchor) against `differences.md:18` row 10 (none exists). Ruled; note added here.
22. `docs/decisions/0035:29` item 9 against `differences.md:24` row 16. Ruled; note added here.
23. `docs/decisions/0011:21` item 6 against `differences.md:49` row 41 (read).
24. `docs/engineering/roles-and-tests.md:9` "Product manager: Amit, with the main session" against `CLAUDE.md:107`-`:108` (the main session is the architect).
25. `docs/engineering/layout.md` "map-tool ... nothing depends on it" against `tests/Cargo.toml` (dev-dependency; checked).
26. `data/reference/step1/README.md:3` and `roles-and-tests.md:32` "for the length of step 2" against `step2-spec.md:224` item 15 (the frozen figures stay frozen after the close).
27. `crates/sim-agent/schemas/README.md:8`, `:43` (version 3; `answer.rs:10`) against `:218`-`:219`, `:240`, `:271` ("the version stays 1").
28. `docs/design/ground-combat.md:53` "Units are not split or merged by any rule" against `07-land-units.md` "A unit divided" and the event `LandUnitDivided` (read).
29. `docs/design/intelligence.md:91` "Layer 2 in step 2" against `open-work.md:63` "With step 2 or 3" and `step2-spec.md:20`.
30. `docs/validation/baseline-reading.md:7`-`:9` (19/10/1/8, 14/10/0/4, 10/3/1/6 beside each baseline) against the remade files (`savo-island-baseline.md:15` 14/4/5 of 23; `eastern-solomons-baseline.md:16` 11/1/3 of 15; `rabaul-guadalcanal-air-baseline.md:19` 3/4/3 of 10); and `baseline-reading.md:133` against `savo-island-r2.md:15`. The remakes are told later in the page.
31. `docs/validation/tuning/r5-anti-aircraft-hit-chance.md:52`, `:62` ("consistent") against its own :54 and decision 0089 ruling 3 ("cannot be called consistent").
32. `docs/research/oob/cape-esperance.md:155`-`:160` (torpedoes fired by Duncan, Buchanan, Kinugasa) against `torpedoes-fired.md:82`, `:134` and `0072:151` (no count for any American ship). Not checked against `engagement_outcomes.csv`.
33. `docs/research/oob/land-forces.md:111` (no stated troop capacity for a destroyer-transport) against `troop-lift.md:66` (144 and 200, disputed).
34. `docs/research/oob/torpedoes.md:27` (Type 93 speeds and ranges) against `conflicts.md:12` (settled values). The first is a draft of Nov 10.
35. `docs/decisions/0098-what-a-land-unit-is-good-at.md:3` "The designer agreed decisions 2 and 6" against its :80-83, still phrased as open questions with no "Answered" heading.

Checked and found to agree: the counts of `step2-close.md:26`-`:30` and each close report; the soak's 0.27 seconds (`0100:32`, `0044:32`); what a saved game records (`0100:31`, `save.rs:150`-`:175`).

## 5. The rulebook's gaps

From a reading of decisions 0066, 0090, 0091, 0092, 0094 and 0096 beside the code (`ground.rs`, `ground_combat.rs`, `march.rs`, `cargo.rs`, `bombardment.rs`, `estimate.rs`, `rebase.rs`, `movement.rs`, `resolve.rs`, `invariants.rs`, `side_events.rs`, `refdata.rs`, `rng.rs`, `hash.rs`; `sim-data`'s loader). Checked by reading the code, not by running it.

Matches found (no entry): the formulas and their rounding of firing men, shots, losses, the beaten test, men fed and the unfed loss, captured stock, weighed shots and the report's error; the order of phases and hexes; the refusals and their order for load with a unit, posture, march and rebase; the rebase range (twice the greater of the two radii, equal allowed) and its effects; the march, its days, supply and dump; the halt and steaming on; the events of these rules and who sees them; the draws `GroundFire` 6, `Bombardment` 7, `GroundReport` 8 and the reuse of `Claim`; no new pulse phase.

**What the code does and no chapter says:**

1. `invariants.rs:138`-`:141`, `:470`-`:484`: the invariant that a force is halted exactly when it holds a halt, at a hex centre on a waypoint that is not the last (`TaskForceHaltUnsound`). Chapter 1's invariants table (`01:590`) has no halt row.
2. `state.rs:947`-`:969`, `cargo.rs:306`-`:308`: a unit made by division starts on defend, fatigue 0, fed share 1, with no posture order and no march, whatever its parent's. Chapter 7, A unit divided (`07:118`-`:122`).
3. `ground_combat.rs:125`-`:134`: the causes of `GroundFight` and of `SideBeatenAshore` are posture orders only; a unit whose attack posture comes from its scenario entry adds none. Chapter 7.
4. Breakdown factors not named: `MenFed` and `MenLostUnfed` (`ground.rs:197`-`:213`, `:256`-`:286`), `MenEmbarked` and `MenLanded` and the cargo event's `men_t` (`cargo.rs:507`-`:515`, `:751`-`:783`), `GroundReportReceived` (`estimate.rs:130`-`:150`; chapter 3 at :283 names them in prose only).
5. `bombardment.rs:308`: weighed shots are one `mul_div`; `04:291` gives the formula without the order of operations.
6. The agent interface of these rules (march supply in the briefing, `moved_yesterday`, `transfer_range_nm` in `measure`, the marks shown in `act_at`; `crates/sim-agent/src/briefing.rs:1496`-`:1558`, `lines.rs:250`, `:335`, `measure.rs:336`, `:793`) is recorded only in the "As built" sections of 0091, 0094 and 0096. Whether a chapter should hold it is a placement question.

**What a chapter says and the code does not do:** items 3 to 9 of section 4.

**Tables that leave something out:**

7. `02-movement-fuel-and-cargo.md:73`-`:74` (the task force's state): no row for a route's marks (`state.rs:396`), the status halted (`state.rs:366`) or the halt (`state.rs:553`).
8. `02:12` (where the chapter's rules run in a day): "Stragglers, movement, meetings and fights" leaves out steaming on, which runs first (`movement.rs:491`).
9. `01:392`-`:421` (what a side sees of each event): no rows for `TaskForceHalted` and `TaskForceSteamedOn` (given in prose at `01:308`, `02:232`; `side_events.rs:732`-`:733`).
10. `docs/rules/README.md:60` (chapter 2's contents): no mention of halting at a marked waypoint or steaming on.
11. `docs/rules/README.md` step 17 and `01:98` (air combat) do not mention hits on men ashore (`07:15`, `05:445`); step 13 and `01:85` (shore bombardment) do not mention the men, the sighting report to the base's side or the claim (chapters 3 and 4 have them).
12. `01:556` lists the values of `Draw` without `TorpedoOtherSide` (code 5, `rng.rs:82`); `04:407` has it.
13. `differences.md:19` row 11 lists what runs at the start of the night pulse without the men's supply (`resolve.rs:88`).

**The glossary** (`docs/glossary.md`). Present with one meaning: every land-unit term of chapter 7 (`:88` to `:109`), rebase, transfer range, write off (`:33`, `:34`, `:167`), mark, halt, steam on (`:77` to `:79`), shore bombardment, weighed shot, men lost to fire (`:130` to `:132`), ground report, estimate, opening estimate (`:118` to `:120`). Stale or missing:

14. `:42` Mission: "which no rule reads yet" (contradiction 2).
15. `:68` Route: no mention of marks.
16. `:52` Unit: "A task force or an air group: what an order is addressed to"; a land unit takes orders too, and chapter 7 says "unit" for a land unit throughout.
17. `:121` Fight, `:128` Shot: defined for task forces and guns only; chapter 7 has "the day's fight" ashore and ground shots, a fraction.
18. `:34` Transfer range says "search range and strike radius" where `:27` and `:181` call both columns radii.
19. No entry: the objective kinds `base_held` and `men_lost`, the men's supply, the ground combat phase, the halt's "work" (bombardment or cargo).

**The order of a day.** `docs/rules/README.md:27`-`:53` and chapter 1 list every phase these six rules add (men's supply, shore bombardment, the halt and steaming on, loading with men, marches and the fight in the ground combat phase, the orders applied at intake), every order (`load` with a unit, `set_posture`, `march`, `rebase`, route marks) and every draw, apart from items 9, 11 and 12 above.

## 6. The decisions index

**Missing from `docs/decisions/README.md`:** `0024-scenario-4-savo-island.md`. It has no title and no status: two notes of Nov 13, 2026 (the break-off levels read from the record; Yunagi in a task force of her own), added by commit 318d4ee. Nothing links it. It shares its number with `0024-savo-island-scenario.md`, the record the index row and the scenario files name. A note in the first file now points to it.

**Rows whose status differs from the record** (on 72e66e4):

| Row | Index says | Record says |
| --- | --- | --- |
| 0069 (:79) | Built Nov 13 | "Nothing is built" (fixed here) |
| 0094 (:104) | Built Nov 14 | "Not built" (fixed here) |
| 0072 (:82) | "Plan" | "Plan", and both describe the scenario built and run (section 8) |
| 0035 (:45) | "2 needs review, after the air figures are tuned" | The same; the condition is met and no review is recorded |
| 0040 (:51) | "Ships are deferred" | Settled by 0058 (note added to the record) |
| 0062 (:72) | Batch versions 2 to 4 | The record has version 5 (0062:57) |
| 0047 (:57) | No amendment named | The record names 0065 and 0085; 0060, 0061 and 0086 also amend it |
| 0034, 0041 to 0044, 0048 to 0050 | "Confirmed" | "confirmed by the architect"; the legend at :112 says "Confirmed" means the designer has |
| 0093, 0095, 0097 to 0100 | "Confirmed by the designer" | The records say "a plan ... the designer agreed" or "approved"; the same in substance |
| :7 | Records 0001 to 0010 "gain their cards at the complexity review" | The review is done; 0001, 0005, 0006 and 0008 have cards, 0002, 0003, 0004, 0007, 0009 and 0010 none |

**A proposal for finding 100 records,** with no record moved or renumbered:

1. Keep the numbered table as it is, the one list of every record.
2. Above it, a short table by standing, one line each: **Built**; **Confirmed, not built** (today 0093, 0095, 0097, 0098, 0099, 0100); **No rule** (scenario data and derivation rules: 0019, 0023, 0024, 0031, 0042, 0072, 0073, 0074, 0076, 0077, 0081, 0082, 0084, 0089); **Engineering, no effect on play** (0002 to 0004, 0009, 0010, 0013, 0015, 0018, 0043, 0053, 0057, 0062); **Replaced in part** (a record and the records that replaced items of it: 0012 by 0032 and 0040; 0014 by 0016, 0021, 0022, 0033, 0078; 0027 by 0033, 0079, 0080, 0087; 0047 by 0060, 0061, 0065, 0085, 0086; 0068 by 0071; 0081 by 0084).
3. Then a table by subject, each line naming its rulebook chapter and the records in order: time and orders (0001, 0008, 0055, 0063, 0091, 0096); movement and fuel (0005, 0006, 0017, 0022, 0040, 0045, 0058); cargo and land units (0011, 0046, 0066, 0090, 0092, 0093, 0094, 0095, 0097, 0098, 0099); search, belief and what a side knows (0007, 0012, 0032, 0047, 0048, 0060, 0061, 0065, 0085, 0086); surface combat (0014, 0016, 0021, 0033, 0037, 0068, 0070, 0071, 0074, 0076, 0077, 0078, 0083); air (0025 to 0030, 0034 to 0036, 0038, 0039, 0041, 0059, 0064, 0075, 0079, 0080, 0087); repair (0069, 0088); objectives (0054, 0056, 0067, 0088); scenarios (0019, 0023, 0024, 0031, 0042, 0043, 0044, 0051, 0072, 0073, 0081, 0082); the harness and reviews (0052, 0053, 0057, 0062, 0084, 0089, 0100).
4. Give the second 0024 file a title, a status and a row of its own ("0024, notes"), or merge its two notes into `0024-savo-island-scenario.md` under a dated heading. That choice is the architect's.

## 7. What could be archived

**Archived in this pass** (moved to `docs/validation/archive/`, a dated line at the head of each, every link fixed; nothing under `crates/`, `tests/`, `data/`, `viewer/` or `.github/` named them, checked by grep):

- `eastern-solomons-d0079.md`, `-d0080.md`, `-d0087.md`, `-r3.md`, replaced as the battle's reading by `eastern-solomons-close.md`. Links fixed in decisions 0079:144, 0080:127, 0087:125 and `tuning/r3-fighter-hit-chance.md:105`.
- `rabaul-guadalcanal-air-d0079.md`, `-d0080.md`, `-d0087.md`, `-r3.md`, `-r4.md`, replaced by `rabaul-guadalcanal-air-close.md`. Links fixed in the same three decisions, `tuning/r3-fighter-hit-chance.md:106` and `tuning/r4-claims.md:123`.

**Could be archived, left for the architect:**

- `savo-island-baseline.md`, `savo-island-r1.md`, `savo-island-r2.md`, `eastern-solomons-baseline.md`, `eastern-solomons-r4.md`, `rabaul-guadalcanal-air-baseline.md`, `rabaul-guadalcanal-air-rabaul-r1.md`: superseded by the close reports, but named by `docs/step2-spec.md` (37, 41, 42, 43, 158), `step2-close.md` (27, 28, 292), `baseline-reading.md` and decisions 0080, 0081, 0082, 0083, 0084, 0087. Moving them means editing those lines too. `savo-island-after-0083.md` is the record the Savo close rests on (`step2-close.md:26`); better kept.
- `docs/complexity-review-after-scenario-5.md`: a one-off review, acted on by 0032 and 0033, named only by their status lines. It could go to `docs/archive/`. Left in place: it is a record rather than a superseded page, and nothing gains from moving it.
- `docs/research/oob/savo-island-choices.md`: superseded by 0019, but its reasoning is the only statement of why; better kept.

**Must not be archived:** every decision record; `step2-close.md`; the four close reports and `cape-esperance-check.md` (their lines are cited by `history-on-the-scale.md:218`-`:221`); `baseline-reading.md`; `passages-baseline.md`; `closing-tests.md` and `closing-tests/` (named by `open-work.md:66`, `differences.md:11`, `roles-and-tests.md:29` and `crates/sim-agent/tests/legibility.rs`); everything in `tuning/` (named by data files and `crates/sim-data/src/reference.rs`); every research note (the history of the facts; `history-on-the-scale.md` needs a link, not a move); `step1-spec.md` and `step2-spec.md` (named by `CLAUDE.md` and many decisions); the design notes (still the plans).

One thing to know: `sim-cli validate` writes its report to `docs/validation/<battle>-<label>.md` by default (`crates/sim-cli/src/validate/mod.rs:300`, `:351`), so the folder is both the record and the tool's output.

## 8. Edits waiting for the files in flight

Each to be applied in one pass when the file is free. The words now are quoted from 72e66e4.

**`CLAUDE.md`**

- :24, "A scenario written and not to be run yet goes in `data/scenarios/not-run/`, which no loop walks (decision 0072)" → add "(the folder is made when needed; it is empty and absent since Cape Esperance left it, commit 897ba4f)".
- :120 Status, "Next, before step 3: the objectives scale set from the record, then stage 4 of ground combat" → name what is built (0091 rebasing, 0094 marching, 0096 act and steam on, beside 0066, 0090, 0092), say that the step 3 spec is confirmed (Nov 14, 2026) and stage 0 (decision 0100) is not built, and say "stage 4 of ground combat (decision 0093)" so it is not read as step 3's stage 4. Which comes next, the scale or step 3's stage 0, is the contradiction of section 4, item 15: the architect's to word.
- The "Where to look" row for `docs/design/`, "Think about what is not planned yet": `ground-combat.md` is now mostly built; suggest "(the plan for ground combat, stages 1 to 3 built)".

**`docs/decisions/README.md`**

- :7, "Records 0001 to 0010 predate this and gain their cards at the complexity review after scenario 5" → "Records 0001 to 0010 predate this; 0001, 0005, 0006 and 0008 gained cards at the complexity review after scenario 5."
- :34 (0024), "one hex, three forces" → add "Notes of Nov 13, 2026 in `0024-scenario-4-savo-island.md`: Yunagi as a force of her own, break-off levels from the record."
- :45 (0035), "2 needs review, after the air figures are tuned" → the architect's ruling on decision 2 now that the air figures are closed.
- :51 (0040), "Ships are deferred" → "Ships: decision 0058".
- :57 (0047), add "Amended by 0060, 0061, 0065, 0085 and 0086".
- :72 (0062), add "version 5, with the wave that took part, decision 0081".
- :82 (0072), "Plan; settled by the architect, to be reviewed" → "Built and run once (Nov 13, 2026); settled by the architect, to be reviewed"; and the same word in the record's own :3 at the same time.
- :105, :107, :108, :109: unchanged.
- A table by standing and by subject above the numbered list (section 6).

**`docs/open-work.md`**

- :13, "The step 3 spec is being written by the architect in parallel." → "The step 3 spec is confirmed by the designer (Nov 14, 2026); `docs/step3-spec.md`."
- :11, "Stage 4 of `docs/design/ground-combat.md` is still to come" → add "(decision 0093, confirmed, not built)"; and name 0091 and 0096 as built.
- :62, "choices rules make inside a day pulled into functions the planner can ask (decision 0055, 'Left for later')" → add "(now decision 0100, step 3's stage 0, confirmed, not built)"; "basic ground combat (stage 4; ...)" → "ground combat's stage 4 (decision 0093)".
- :67, L7, "a task force's mission is shown in the briefing and read by no rule" → the bombardment mission is read (0092); the item holds for the other missions only.
- A line for each confirmed and unbuilt record (0093, 0095, 0097, 0098, 0099, 0100), the page's own purpose (:3).
- A link to `docs/research/oob/history-on-the-scale.md` beside item 2, the scale (:10) and :51, so the page is no longer orphaned.
- Experience for air groups appears at :25 and :40, and Savo Island's participation at :32 and :44: one each is enough.

**`docs/glossary.md`**

- :42, "A task force also has a mission, which no rule reads yet" → "A task force also has a mission; only shore bombardment reads it (a force on the bombardment mission)".
- :68 Route: add "and the waypoints marked to act at".
- :52 Unit: "A task force, an air group or a land unit: what an order is addressed to"; or a separate entry, as the rulebook's chapter 7 uses "unit" for a land unit.
- :121 Fight and :128 Shot: add the meaning ashore, or a word of its own for each.
- :34 Transfer range: "search radius and strike radius".
- New entries: `base_held`, `men_lost` (or point to chapter 6), the men's supply, work (at a halt: a bombardment or cargo).

**`docs/rules/`** (each a statement for the architect to word; the code is the authority)

- `01:603`, "`flagplot-state-v14`" → "`flagplot-state-v15`".
- `02:80`, `06:11`, `differences.md:20` row 12: the mission is read by shore bombardment.
- `05:236`, "Every other `BaseSupplySpent` has 0 men" → except a march's payment.
- `06:32`: add men lost to fire.
- `07:70`: "idle, arrived or halted".
- `07:386`: `march_days_per_hex` is a whole number only.
- `07:36`: the series kept for units made in play is not used by a divided land unit.
- `07:144`: `FedShareRestored` only when the share was not already 1.
- `07:112`: two landings in one pulse (section 4, item 9), after the architect rules whether the code or the book is to change.
- `01:590`: the halt invariant. `02:73`-`:74`: the route's marks, the halted status, the halt. `02:12`: steaming on. `01:392`-`:421`: rows for `TaskForceHalted` and `TaskForceSteamedOn`. `01:556`: `TorpedoOtherSide`. README :60, step 13 and step 17 (section 5, items 10, 11).
- `07:118`-`:122`: what a divided unit starts with. Chapter 7: the causes of a fight.
- `differences.md:19` row 11: add the men's supply. Row 14 (`:22`): three kinds of unit. Rows 1, 3, 4, 5, 9, 10, 16: the notes they called for are now in the records (this pass); the rows can say "Done".

**`docs/decisions/0095-preparing-a-landing.md`**

- :106, "Decision 4: preparation speeds the landing and also helps the fight ashore (decision 4a)" → the question's numbers after the revision: decisions 5 and 6.
- :107, "Decision 6: a new landing place loses all of it; a ship joining or a new unit loaded keeps half" → decisions 8 to 10.
- :3 and :13 "Nothing is built": true on `main`; say nothing until the build merges.
- Name its hand-calculation test, `tests/scenario_27_preparing_a_landing.rs` (it exists; its scenario file does not).

**`docs/decisions/0097-the-quality-of-a-land-unit.md`**: name its test, `tests/scenario_28_experience_and_training.rs`. **`0099-units-that-work-and-what-they-build.md`**: none; it names no scenario and has no test yet.

**`docs/step3-spec.md`**: none found wrong. Its four dated "Amended" paragraphs at :3-:11 sit above the contents; the spec now changes only by a decision record, so they could move to the end.

**`docs/commands.md`**: flags all correct; it does not mention the march order (0094) or `act_at` (0096).

**Under `data/`, `crates/` and `viewer/`** (files outside this pass by rule):

- `data/oob/README.md:46`-`:82`: "Status after several passes, read on 2026-11-10" and its counts (for example `ships.csv` 122 rows, now 172; `ship_facts.csv` 612, now 1424; `sources.csv` 66, now 247: `wc -l`). :117 "They need a permanent home in sim-data": `crates/oob-check` is that home. The notes `air-group-experience-proposal`, `anti-aircraft`, `damage-control`, `gunnery-fire-control`, `history-on-the-scale`, `radar`, `savo-island-choices`, `sonar` and `torpedoes` are not in its index.
- `data/scenarios/08_soak.toml:33`: "Ships cannot take on fuel" (refuelling built, 0045).
- `data/validation/passages.toml:11`: "76 rows" (186).
- `data/reference/step1/README.md:3`: "for the length of step 2" (contradiction 26).
- `crates/sim-agent/schemas/README.md:218`-`:219`, `:240`, `:271`: "the version stays 1" (it is 3); its list of objective kinds near :244 lacks `base_held` and `men_lost` (added later at :386).
- `crates/sim-cli/src/main.rs:62`: "version 2" of the batch file (it is 5).
- `crates/sim-cli/tests/recorded_games/README.md:31`-`:121`: about sixty dated paragraphs of regenerations; the games themselves are :33-:35.

## 9. What a new reader would trip on

- **Two stages 4** (section 4, item 17), and step 3's stages 0 to 5 beside ground combat's 0 to 4. A record that says "stage 4" should say whose.
- **Two files numbered 0024** (section 6).
- **`docs/validation/baseline-reading.md`**: titled "The baseline, read", it holds the reading of Nov 12, about twelve dated remakes, the whole reading of the Cape Esperance check (176-217) and the comparison of closing records (278-378). The same Savo Island rounds are told there, in `tuning/r1`, `tuning/r2`, `tuning/savo-island-closed.md` and `step2-close.md:42`-`:73`, with six generations of counts.
- **`docs/commands.md:5`**: one paragraph of about 2,900 words, ordered by decision.
- **`docs/open-work.md:11`**: one dense paragraph with the history of stages 1 to 3 and marching; `CLAUDE.md` asks for the status to be kept short, and this list is its long form.
- **`crates/sim-cli/tests/recorded_games/README.md`** and **`crates/sim-agent/schemas/README.md`**: both have grown into change logs in the order of the decisions.
- **`docs/decisions/0069-ship-repair.md`**: about 45 KB, with "As planned for the build" (101) and the yard function's figures (321) but no "As built"; the build is told in 0088:190 and the code.
- **`docs/decisions/0072-cape-esperance-scenario.md:3`**: a status line of 575 characters telling three stages, which still begins "plan".
- **The engineering pages** each open with "Moved out of `CLAUDE.md` on Nov 12, 2026, word for word"; history in the header of a page that is a rule.
- **`docs/engineering/data-rules.md:15`**: "Until it can derive a ship, figures in `data/reference` stay placeholders": `oob-derive` derives ships since 0020.
- **Rulings that asked for a note and were never given one** (`differences.md` rows 1, 3, 4, 5, 9, 10, 13, 16): added in this pass.
- **Research notes not indexed** in `data/oob/README.md` (section 8).
- **`docs/research/oob/outside-research-nov-13.md:3`** names, in its first paragraph, the outside source of the answers it checks; the house rule (`CLAUDE.md`, and the brief of this pass) is that outside models are not named in the repository. For the architect to look at; not changed here.

## 10. What this pass changed

Commits on branch `docs-housekeeping`, each one kind of change:

1. Status lines made untrue by `main`, corrected with a dated note on the same line: 27 files (section 2, the first table).
2. Nine reports of single rounds moved to `docs/validation/archive/`, with their links in five files (section 7).
3. Dated notes that a later record changed or replaced an item of an earlier one, including the notes `docs/rules/differences.md` called for (rows 1, 3, 4, 5, 9, 10, 13 and 16): 39 records (0001, 0005, 0007, 0008, 0009, 0011, 0012, 0014, 0016, 0017, 0018, 0020, 0021, 0024-savo-island-scenario, 0025, 0026, 0027, 0029, 0031, 0033, 0035, 0036, 0038, 0041, 0043, 0044, 0045, 0047, 0050, 0052, 0054, 0055, 0057, 0063, 0066, 0067, 0069, 0072, 0090). Text is only added.
4. Stale references fixed: `docs/design/ground-combat.md:145` (`ground.toml`), `docs/decisions/0096-a-force-that-acts-and-steams-on.md:3` (the step 3 spec's locator).
5. This report.

Nothing under `crates/`, `data/`, `tests/`, `viewer/` or `.github/` is changed, and none of the files in flight. No ruling, rule, figure, number or decision is changed; no numbered decision or hand calculation of a record is edited. Every relative link in the repository resolves after the pass (checked by a script).
