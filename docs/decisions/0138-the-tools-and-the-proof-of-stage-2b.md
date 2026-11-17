# 0138: The tools and the proof of stage 2b: step 5, and the test plan written before the run

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 2b, that the designer confirmed on Nov 16, 2026 with its pass marks (decision 0132, step 5 of its five and "What it must pass"). It says what tools are built, and then what is run, how each condition is read and what passes, before any of it is run. The tools (items 1 to 7) were built on Nov 17, 2026 ("As built" below); the proof (items 8 to 17) was run on Nov 17, 2026, and passes as stated ("As run" below). A stricter count of condition 4, made after the run on the designer's word, gives 174 seeds against the mark of 200: see "The stricter count".

Summary: Step 5 of command levels: the tools that show who knew what (a command's picture and held state, when a report reached a command, why a group did not launch, refusals counted by cause, a seat at any command for a model, the viewer showing one command's picture beside the truth), and the test plan of the stage's proof: the Eastern Solomons from intents with command levels on over a thousand seeds, with the reading of each of the designer's pass marks fixed before the run

Standing: Settled by the architect, Nov 17, 2026, inside decision 0132, whose pass marks the designer confirmed on Nov 16, 2026. Tools built. Proof run Nov 17, 2026: passes as stated. Condition 4 by a stricter count made after the run: 174 of 200, which is the designer's to weigh

## The tools

1. **When a report reached a command.** `sim-cli events` can be asked, for a report and a command, when the command first held it and by which road (its own, up, sideways, down), or that it never did. `--why` on a launch or an order follows a report only along the road that brought it to the command that decided, and no longer lists every passing (decision 0135, "As built", open).
2. **Why a group did not launch.** For a strike group, a day and a pulse, the tools give the answer the rules gave on its command's picture: the contacts that command held then, and for each why it was not struck (not held, too old, out of reach, of a kind not on the list, nothing able to fly). It is the question of decision 0100, asked of a past pulse from the saved game or the log.
3. **A command's picture and held state** in `command_state`: the contacts a command holds with its own numbers, and for the area command each force as last reported with the pulse it is of.
4. **Refusals by cause.** The command report counts the staff orders refused because a force was not as the area command held it (decision 0136, 14) apart from every other refusal.
5. **A seat at any command.** The agent interface can be started in a seat: the area command, a task force or a base. A seat is given its command's picture, its own units exactly, and, for the area, its forces as last reported. A seat that is not the area's may give orders to its own units only, and is refused any other with the reason. Stating intents from a seat that is not the area's is not built (decision 0132, "What it leaves out"). The easier setting of decision 0119, answer 5, is a choice at the area seat: the side's own picture, and its forces exactly.
6. **The viewer** can show, for a scenario exported with command levels on, the picture of one command beside the truth: its contacts in each pulse, and the reports sent with who got each and when. The viewer and the export are the umpire's, as always.
7. **No record is added to a launch.** Which command decided a launch is the command of the group that flew, which the export and the log already give.

## The proof: what is run

8. **Files,** in `data/scenarios/stage-2b-proof/`, each an existing file with command levels on, the live report figures and nothing else changed: the Eastern Solomons with both sides from the intents of decision 0128 (from `second-proof/05a`), the same with command levels off for comparison; the week of the slice from intents (from `second-proof/07a`); and Savo Island as scripted (scenario 4). No commander is named and no force is silent in any of them: the stage is judged without initiative (decision 0132).
9. **Release build, live figures, seeds 1 to 1,000,** each batch run twice.

## The conditions and how each is read

10. **Condition 3 of decision 0132, the battle.** The Eastern Solomons file, command levels on: every invariant holds on every seed; no command and no staff order is refused on any seed but for the cause of decision 0136, 14, and those are counted and reported; the two batches are byte for byte the same. Judged.
11. **Condition 4, a sighting that reaches one command before another.** A case, on a seed, is a sighting report of ships, a command A and a command B of one side, such that:
    - A holds the report in the pulse it was made, P, as its own or by word;
    - B holds it in a later pulse or never;
    - a strike group of A launches at the contact that report made or refreshed in A's picture, in pulse P or in the next daylight pulse of the same day, while that report is still the contact's last;
    - B has a group on the naval strike mission with an aircraft able to fly in pulse P, and the report's hex is within that group's strike radius, at the reach it is set to, from where B is in pulse P;
    - and no group of B launches at that hex in pulse P.

    The count is the seeds with at least one case, of either side. Pass: 200 or more of 1,000. Judged.
12. **Condition 5, each can state its reason.** Twelve cases: the first case, by the report's event number, on each of the first twelve seeds in order that have one. For each, a reader is given only the tools' answers of 1 and 2 above: for A, why its group launched; for B, why its group did not launch in pulse P, and when the report reached B. The reader is a model with no tools and nothing else of the game. It is asked three things: which of the two commands held the sighting in that pulse; when and by which road the other came to hold it, or that it never did; and why the other did not strike in that pulse. The key is written from the measured data before any reader runs. A case is right when all three answers are. Pass: 10 of 12. Judged.
13. **Condition 6, the week.** The week file, command levels on: the conditions of decision 0128 for the week, set beside its third run. Reported and not judged.
14. **Condition 7, Savo Island.** For each Allied command, the pulse in which it first holds a sighting of the Japanese cruiser force, on seeds 1 to 1,000: the share of seeds on which the force that fights it holds one before the fight. Reported and not judged.
15. **Condition 8, speed.** A staffed day of the week with command levels on and the live report figures takes no more than three times what it takes with them off, by `sim-cli time`, the median of five. Judged.
16. **Also reported and not judged:** on how many seeds each side strikes at ships with command levels on and off; how many passings arrive on time, late and never; and the Eastern Solomons run once more with the Allied carrier forces given a bold placeholder commander.
17. **A model in a seat.** One game of the Eastern Solomons with a model in the seat of one Allied carrier force, ordering its own units, the staff in every other seat. Reported and not judged: what it was shown, what it ordered, and whether it was ever given anything its command did not hold.

## What is done with a failure

18. **Nothing is tuned to pass,** no figure is changed, and no reading above is changed after a run. A condition that fails is reported with its seeds and its cause, and goes to the designer. The first run's report is kept whatever follows.

## What was already seen

19. **Before this plan was written,** two things had been measured for information and are disclosed: with command levels on and the live report figures, over 200 seeds of the Eastern Solomons, the Allied side struck at ships on 168 seeds and the Japanese on 194, against 168 and 195 with command levels off; and of 21,106 passings, 14,846 arrived on time, 5,266 a pulse late and 994 never. Neither counts cases as 11 reads them, and no reading above was chosen on them.

## What it costs

- **The agent interface version rises** for the seat. The export format version rises for the pictures.
- **No state** is added by this step.

## As built: the tools (items 1 to 7)

Built on Nov 17, 2026. No rule of the game changed: no event, state, hash or draw; every earlier test passes unchanged and every run baseline is unchanged. The state version stays `flagplot-state-v38`, the rules label 18, the saved game version 13 and the doctrine `supply-19`. The agent interface is version 16, the replay (export) format version 6, the batch format version 6. `docs/commands.md` describes each tool.

1. **When a report reached a command:** `sim-cli events SOURCE --report EVENT [--command area|task_force:N|base:N] [--json]` (`crates/sim-cli/src/events.rs`, `events/report_road.rs`). On scenario 62, report 71 (task force 1's "nothing there", morning of day 1): "the area command first held report #71 in the morning of day 1: sent up by task_force:1 (event #73), on time." and "task_force:2 never held report #71 (sent sideways by task_force:1, event #73: lost; sent down by the area command, event #74: lost)." `--why` on a launch or an alert follows the cited report only along the road to the command that decided.
2. **Why a group did not launch:** `sim-cli events SOURCE --strike air_group:N --day D --pulse morning|afternoon [--report EVENT] [--json]`, read from `SideView::strike_review` (`crates/sim-core/src/view/strike_review.rs`), asked at that pulse's strike decisions as the game is played again. On scenario 62, group 6 on the morning of day 1: "the rules gave: launch nothing", its one contact "too old: 1 pulse against the limit of 0", and report 71 "not held by task_force:2". The strike rule's own tests are shared, not copied: the per-contact test (`contact_verdict`), the group's gates (`strike_group_gate`, `strike_mission_stop`), the sizing walk (`strike_walk`, `sized_strike`) in `airstrike.rs`, and the walk of the groups before (`SideView::sent_before_group`), each now called by the strike decision and by the review.
3. **`command_state`**, with command levels on, gives `picture` (the area command's contacts, its own numbers, own or told) and `forces_held` (each force and base as held, with `as_of`, `current` and a force's `radio_silence`) (`crates/sim-agent/src/staffing.rs`).
4. **Refusals by cause:** the command report's `refusals` (`staff_orders_not_as_held`, `staff_orders_other`, `commands`; `reporting.rs`) and each batch run's `refusals` by side (`batch.rs`, `batch_record.rs`).
5. **A seat at any command:** `--at SIDE=area|area:easier|task_force:N|base:N` on `serve`, `--at` beside `mcp --seat`, and `play --at` kept in the game file's header; `Session::with_post` and `Post` in `sim-agent`. A seat at a task force or a base reads `SideView::at_seat` (`crates/sim-core/src/view/seat_view.rs`), a view whose own units, orders, events and picture are cut to the command, so the agent's builders run on it unchanged. The easier setting reads the side's view with `with_side_picture`.
6. **The export and the viewer:** `pictures` in `steps` and `final` for a game with command levels on, and each `report_sent` listener as an object with direction, arrival and the day and pulse it arrived (`crates/sim-cli/src/export/`); the viewer's "Command picture" selector and Reports tab (`viewer/index.html`, `viewer/check.js`, `viewer/README.md`). Of 94 scenarios exported before and after, the 90 with command levels off are byte for byte the same but for the version.
7. **No record is added to a launch.** The deciding command is the launching group's command at the pulse's strike decisions, read as the game is played again.

### Readings taken

1. **Held or never** is read from what each command has heard (`CommandPost::heard`), moment by moment as the game is played again; the road is named from the log: its own where the command sent the report up or sideways, or is the area command sending it down with no passing up behind it, or holds it with no passing to it that arrived; otherwise the first passing that arrived (a late one at the start of the next pulse, before an on-time one of that pulse; then by event number). The two readings were checked against each other on scenarios 62 to 64, the Eastern Solomons from intents with command levels and the live figures on seeds 1 to 5, and the week from intents on seed 1, with no disagreement.
2. **A late passing that never lands** is said so: the game ended first, or the command had ceased to be one (`deliver_late_reports` drops it).
3. **A report that is not passed** (a scenario's starting contact, every report with no figures) is refused by `--report` as "not a report passed between commands", and with command levels off or no figures the tool answers in one line.
4. **`--why` on an order:** an accepted order cites only its order in the log, so there is no report to narrow; the narrowing is for a launch and for an alert from belief. It applies only where reports travel; otherwise `--why` is byte for byte as before.
5. **The night pulse** has no strike decisions, so `--strike` refuses it.
6. **The review's verdicts:** the group's gates in the decision's order (not on the mission, no ready aircraft, no position, dark, no weapon, nothing able to fly, the held share leaves none), and "on the base strike mission" as a reading-only cause; a contact that fails its own test shows that test even where the group is also stopped.
7. **A seat's units:** a task force owns itself, its ships (sunk and at a yard too), the groups aboard and the land units aboard; a base owns itself, the ships in its port, the groups at it or aboard them, the land units in its hex or aboard them, and its replacement streams. Numbers are the seat's own, from 1, so a seat is not told how many forces its side has; bases keep their numbers. The person starting the game names the command by the scenario's id.
8. **A seat's events** are those that name one of its units and no other unit of its side, with the "not there" and claim events they caused; reports, contact ageing and drops, report sent and state report events are left out (their contact numbers are the side's). Mission, objectives and the final result are not given at such a seat (`objectives.not_given_to_this_seat`, `result` null).
9. **A seat's order and the staff's:** as for any commander's order (decision 0107): the seat's order to a unit an intent uses holds the unit against the staff, and a seat cannot release the hold. Not changed.
10. **Refusals counted:** staff orders by the issuer of their origin, split by `RejectReason::is_not_as_held`; commands refused whatever the issuer. No built scenario has a staff order refused, so the split is tested on the tally and the end-to-end test counts commands; the proof's batches will be the first to count a refused staff order.
11. **`validate`** reads batch versions 5 and 6 (`crates/sim-cli/src/validate/mod.rs`); `docs/step2-spec.md` still says version 5 only and is left to its keeper.
12. **The export:** a `report_sent` listener was text and is an object; nothing read the text. Each exported contact carries `report_event`. The viewer's command view shows no ground estimates by command (none are exported by command). The Reports tab is the umpire's page only.

### Tests

`crates/sim-cli/tests/events_who_knew.rs` (10) and a test of every road in `events.rs`; `crates/sim-core/src/view/strike_review.rs` (3); `crates/sim-agent/tests/command_picture.rs` (6) and a tally test in `reporting.rs`; `crates/sim-cli/tests/batch.rs` (2 new); `crates/sim-agent/tests/seat.rs` (14), among them every tool's answer at a task force seat and a base seat the same in two worlds that differ only in what the seat's command does not hold (another force's place, fuel and aircraft, a base's stocks, a contact another force saw), at the start and after each day, and the week of the slice at every command; `crates/sim-cli/tests/seat.rs` (6); `crates/sim-cli/tests/export_pictures.rs` (6). Scenarios 62, 63 and 64 are read against their hand calculations by each tool. The recorded games' transcripts changed only in the agent interface version and, in one `report`, `refusals` all 0.

## As run: the proof (items 8 to 17)

Run on Nov 17, 2026, release build, live figures, seeds 1 to 1,000, from main at the merge of pull request 392. The report is `docs/validation/stage-2b-proof.md`; the files are in `data/scenarios/stage-2b-proof/`. Nothing was tuned and no reading was changed after a run. **The proof passes as stated:**

- **Condition 3, the battle:** every invariant on all 1,000 seeds; no staff order and no command refused on any seed, for this record's cause (14 of decision 0136) or any other; both batches byte for byte the same.
- **Condition 4:** 233 of 1,000 seeds have a case (Allied 148, Japanese 103), against 200.
- **Condition 5:** 12 of 12 cases right, against 10 (`crates/sim-cli/tests/recorded_games/legibility_stage_2b/`).
- **Condition 8, speed:** a day of the week with command levels on takes 1.05 times (early) and 1.08 times (late) what it takes with them off, against 3.

Reported: the week with command levels on meets every condition of decision 0128 on every seed, its medians the third run's but for the margin (53.70 against 53.42); at Savo Island every Allied command holds the Hudson's report from the start, and the northern group holds a sighting made in play before its own fight on 850 of 999 seeds; strikes at ships on 818 and 976 seeds (Allied, Japanese) on, 822 and 979 off; of 106,724 passings 69.9% on time, 25.2% late, 4.9% never; the bold commanders change no event (no force holds a contact of its own on the morning the closing is weighed); and a model in the seat of task force 1 was given nothing its command did not hold (`stage_2b_seat_sonnet`).

Readings taken in the run, each in the report: condition 4's clauses as the script reads them (a night report gives no case, and none arose; held is the `--report` tool's; a launch of B at another hex does not stop a case); "a staffed day" as `time`'s one day of the slice; condition 6 for arrangement (a) only; the Hudson's report held at the start as a sighting of the cruiser force.

## The stricter count

The reading of condition 4 fixed before the run says that no group of the second command launches **at that hex** in the pulse. On 59 of the 233 seeds every case has the second command striking in that pulse at another hex, often the same enemy force reported from another place. The architect, who wrote the reading, judged it looser than he meant, told the designer, and on the designer's word counted again from the outputs kept, with nothing run again: a case counts only where the second command launches nothing at all in that pulse.

**By the stricter count 174 of 1,000 seeds have a case** (193 cases; Allied on 118 seeds, Japanese on 67), against the mark of 200. By the reading fixed before the run it is 233 and the condition passes; by the stricter one it misses by 26 seeds. Both numbers stand in the record. What is made of the difference is the designer's.
