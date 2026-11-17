# Step 3, stage 2b: the proof of command levels

Run on Nov 17, 2026, by the team lead of the proof, to the test plan of decision 0138 (items 8 to 19), written before the run, for the pass marks of decision 0132 ("What it must pass"). Release build, live figures, seeds 1 to 1,000, from main at the merge of pull request 392: state layout `flagplot-state-v38`, rules label 18, doctrine `supply-19`, agent interface 16, export version 6, batch version 6. Nothing was tuned, and no figure, reading, pass mark, intent or file was changed after a run.

**Result: the proof passes as stated.** Every judged condition meets its mark.

| Condition | Value | Mark | Holds |
| --- | --- | --- | --- |
| 3, the battle: every invariant, every seed | 1,000 of 1,000 runs complete | every seed | yes |
| 3: refusals | 0 staff orders refused (0 for the cause of decision 0136, 14; 0 for any other), 0 commands refused, both sides | none but for that cause, counted | yes |
| 3: the same start gives the same game | the two batches byte for byte the same; five exports made twice the same | identical | yes |
| 4, a sighting that reaches one command before another | 233 seeds of 1,000 with a case | 200 or more | yes |
| 5, each can state its reason | 12 of 12 cases right | 10 of 12 | yes |
| 8, speed | 1.05 times (13.6 ms against 12.9 ms, a day early in the week); 1.08 late; 1.06 over six days | no more than 3 times | yes |

Condition 6 (the week), condition 7 (Savo Island), the reported items of item 16 and the model in a seat (item 17) are reported below and not judged.

## The files (item 8)

In `data/scenarios/stage-2b-proof/`, each kept out of the loops that walk `data/scenarios/*.toml` as the other proof folders are (decision 0072). Each was made by a script from the file named, adding a header comment, `command_levels = ...` on the line after `name`, and `reports = "../../reference/reports.toml"` (the live report figures: 0.7 on time, 0.25 a pulse late in every direction; reach 600 nm for a carrier, battleship and cruiser, 300 nm for another ship, 1,000 nm for a base) on the line after the last reference file. Nothing else is changed. No commander is named and no force is silent in any of the four files of item 8.

| File | Made from | Changed |
| --- | --- | --- |
| `05a_eastern_solomons.toml` | `second-proof/05a_eastern_solomons.toml` (both sides from intents) | `command_levels = true`, `reports` |
| `05b_eastern_solomons_off.toml` | the same | `command_levels = false`, `reports` (read by nothing with levels off); its hash on seed 1942 equals `second-proof/05a`'s |
| `07a_week.toml` | `second-proof/07a_week.toml` (the week, days 15 to 21, from intents) | `command_levels = true`, `reports` |
| `04_savo_island.toml` | `04_savo_island.toml` (scripted) | `command_levels = true`, `reports`, and its six data paths moved one folder up (`../` to `../../`) so that they name the same files from the new folder |
| `05c_eastern_solomons_bold.toml` | `05a_eastern_solomons.toml` of this folder | for item 16 only: a `[[commander]]` "Placeholder Bold 1" and "Placeholder Bold 2", bold, for task forces 1 and 2, at the end, each marked a placeholder |

## How each batch was run (item 9)

For each file: `sim-cli batch` twice over seeds 1 to 1,000 (`--label stage-2b-proof`), compared byte for byte; `sim-cli export` of every seed read by the second proof's `measure.py` and `summarise.py` (copied unchanged but for the worktree's path); the exports of seeds 1, 2, 3, 500 and 1,000 made twice and compared. Every pair of batches is byte for byte the same, every pair of exports the same, and every run of every file completed. The scripts and outputs are kept outside the repository.

## Condition 3, the battle (item 10): passes

The Eastern Solomons, both sides from intents, command levels on:

- **Every invariant holds:** all 1,000 runs complete, in both batches and in the 1,000 exports.
- **Refusals,** from the batch's `refusals` by side, and checked against the exports' `order_rejected` and `command_refused` events: 0 staff orders refused because a force was not as the area command held it, 0 other staff orders refused, 0 commands refused, on every seed, for both sides.
- **The same start gives the same game:** the two batch files are byte for byte the same, and so are the five pairs of exports.

## Condition 4, a sighting that reaches one command before another (item 11): passes, 233 of 1,000

### How the script reads it

Written before any judged run. The script (`c4.py`, kept outside the repository) runs, for each seed, `sim-cli export` of the file (the umpire's log and the state at the end of each step) and asks the tools of decision 0138: `sim-cli events --strike air_group:N --day D --pulse P --json` for every air group alive at the start of each daylight pulse, and `sim-cli events --report R --json` for every sighting report a launch cites. A case is a sighting report R of ships, made by a side in pulse P of day D, and two different commands A and B of that side:

1. **"A holds the report in the pulse it was made, P, as its own or by word."** R is a `report_received` event of the side (a sighting of ships, the event that makes or refreshes a contact; a claim, a ground count and a "nothing there" are not sightings), and P is its day and pulse. A holds it in P when the `--report R` tool's `first_held` for A is that day and pulse, by any road (its own, up, sideways or down).
2. **"B holds it in a later pulse or never."** The `--report R` tool's `first_held` for B is after P, or its `held` is false.
3. **"A strike group of A launches at the contact that report made or refreshed in A's picture, in pulse P or in the next daylight pulse of the same day, while that report is still the contact's last."** A `strike_launched` event in P, or in the next daylight pulse of day D (the morning after a night report, the afternoon after a morning one, none after an afternoon one), whose causes cite event R, by a group whose command at that pulse's strike decisions is A (the `--strike` tool's `command` for that group and pulse). The strike rule puts among a launch's causes the event of the last report that made or refreshed the chosen contact in the deciding command's picture (`contact.last_report_event`, `airstrike.rs`), so a launch citing R is a launch at the contact R made or refreshed there while R was still its last.
4. **"B has a group on the naval strike mission with an aircraft able to fly in pulse P, and the report's hex is within that group's strike radius, at the reach it is set to, from where B is in pulse P."** Some group whose `--strike` answer for (D, P) has `command` B, `mission` "naval strike", `aircraft_able_to_fly` above 0 and `dark` false, and whose `hex` (where the group, and so B, is at that pulse's strike decisions) is within `reach_nm` (the radius at the reach the group is set to, normal or extended) of R's hex (the report's estimated hex, from the event), by the air distance of `Hex::air_distance_nm`, a distance equal to the radius being within it.
5. **"No group of B launches at that hex in pulse P."** No `strike_launched` or `base_strike_launched` event in (D, P) at R's hex by a group whose command at (D, P) is B. A launch of B at another hex in P does not stop the case.

A case is one report, one A and one B; several launches of A at one report make one case. The count is the seeds with at least one case, of either side.

**Readings taken for the script:**

- **A night report gives no case.** No strike is decided in the night and the `--strike` tool refuses a night pulse; no group is able to fly a strike in the dark, so clause 4 is not met. The script counted the night reports that met clauses 1 to 3: there were none on any seed, so the reading changed nothing.
- **The area command is never A or B:** it has no air group, so it can neither launch nor meet clause 4.
- **Held is the tool's,** not the export's. The script also works out from the export's `report_sent` listeners when each command first held each report and lists every disagreement. Over the 1,000 seeds there were 501, every one a command (most often the area command) that the export shows receiving a late passing in the night of day 2, after the one-day game had ended, and that the tool says never held it (its reading 2: a late passing that never lands because the game ended). The tool's reading is used.
- **Clause 5 is read as written:** B launching at another hex in P does not stop a case. Of the 233 seeds, 174 have a case in which B launched nothing at all in P; on the rest B's groups launched in P at another contact, often the same enemy force reported at another hex (case 1 below). This is reported beside the count, which it does not change.

**Tested before the run.** On scenario 62 the script finds no case, as it should: the one launch (air group 3, morning of day 1) is at the contact held at the start (event 1), which is not a report passed between commands, and the one report that travels is a "nothing there", not a sighting. On the Eastern Solomons file, seeds 1 to 4 and 8 were read by hand from `sim-cli run --events` and the two tools, and agree with the script:
- **Seed 1, one case:** report #245 (task force 4's own, the morning, a cruiser at (7,-5)); task force 4's group 19 launched at it in the morning (#250); task force 3 held it only in the afternoon (sent sideways, a pulse late); its group 12, at (8,-11) with 18 aircraft and a reach of 252 nm, was 223 nm from (7,-5); task force 3 launched in the morning at (7,-6), on report #242, and not at (7,-5).
- **Seed 2, none:** each report a launch cites (#232 Allied, #243 Japanese) reached the other force on time.
- **Seed 3, none:** report #246 (task force 4's own) never reached task force 3 sideways, but task force 3 launched at (7,-5), its hex, in the same pulse (clause 5).
- **Seed 4, none:** report #242 reached task force 3 sideways on time; report #282 (Allied, afternoon) reached task force 2 on time and base 1 on time by the passing down.
- **Seed 8, one case:** report #245 (task force 4's own) never reached task force 3 (sideways lost); task force 3's group 12 was in reach and task force 3 launched at another hex.

### The count

**233 of 1,000 seeds have at least one case** (Allied on 148, Japanese on 103, both on 18), against the mark of 200. It passes. There are 267 cases: B held the report a pulse later in 189 and never in 78. By side, A and B: Allied task force 2 before task force 1 in 87 cases, task force 1 before task force 2 in 74; Japanese task force 4 before task force 3 in 80, task force 3 before task force 4 in 26. Henderson Field (base 1) is neither A nor B in any case. The first forty seeds with a case are 1, 8, 24, 30, 36, 41, 51, 57, 76, 78, 79, 82, 83, 85, 87, 96, 97, 98, 100, 101, 105, 116, 123, 132, 135, 136, 141, 143, 144, 145, 154, 156, 159, 160, 163, 166, 173, 178, 180, 181.

## Condition 5, each can state its reason (item 12): passes, 12 of 12

Kept in `crates/sim-cli/tests/recorded_games/legibility_stage_2b/` (the rule, the cases with the tools' answers and the text each reader was given, the key, the answers and the marks).

- **The pick:** the first case, by the report's event number, on each of the first twelve seeds that have one: seeds 1, 8, 24, 30, 36, 41, 51, 57, 76, 78, 79 and 82. Where a seed has several cases on its first report, the first by A's launch, then by B's name; B's group is the lowest-numbered group meeting clause 4. Eight are Allied, four Japanese; in five B held the report a pulse later and in seven never (lost, or a pulse late and due after the game ended).
- **What each reader was given:** the three answers of the tools, as printed: `--strike` for A's group at its launch pulse with `--report R` (why it launched), `--strike` for B's group in P with `--report R` (why it did not launch), and `--report R --command B` (when the report reached B), with the three questions of item 12. The two commands were named in the order of their names, not A first.
- **The readers:** one Haiku model (`claude-haiku-5-5`) per case, run as `claude -p --model haiku --tools "" --strict-mcp-config --setting-sources ""`, so with no tool of any kind; each answered in one turn.
- **The key** was written from the tools' answers before any reader ran. A case is right when all three answers are: the command that held the report; when and by which road the other held it, or that it never did; and that the other did not hold the report at that pulse's strike decision, so it was not a contact in its picture to strike at.
- **The marks:** a Sonnet model with no tool, given the key and the answers, marked every case right on all three answers. The team lead read every answer against the key and agrees.

## Condition 8, speed (item 15): passes

`sim-cli time --machine "Apple M3 Pro, 36 GB, macOS (the architect's laptop)" --slice FILE --early-day 15 --late-day 18 --repeats 5`, release, both staffs at work from day 15, run one after the other on an idle machine: the week with command levels on and the live report figures (`stage-2b-proof/07a_week.toml`) against the same with them off (`second-proof/07a_week.toml`). **Reading taken:** "a staffed day of the week" is read as `time`'s "one day of the slice", the median of five, early (day 16) and late (day 19); the six days in one call from day 15 are given beside it.

| Workload (median of 5) | Levels on | Levels off | On against off |
| --- | --- | --- | --- |
| One day of the slice, early (day 16) | 13.6 ms | 12.9 ms | 1.05 |
| One day of the slice, late (day 19) | 9.44 ms | 8.74 ms | 1.08 |
| Six days in one call from day 15 | 70.8 ms | 67.0 ms | 1.06 |
| A day with every question asked, early | 27.3 ms | 27.0 ms | 1.01 |
| The staff's decisions, longest day: Allied (day 15), Japanese (day 17) | 4.00 ms, 7.26 ms | 3.80 ms, 7.30 ms | 1.05, 0.99 |

The mark is three times; the most is 1.08.

## Condition 6, the week (item 13): reported, not judged

The week file, command levels on, both sides from intents, arrangement (a) of decision 0128, beside its third run (`docs/validation/stage-2-second-proof-third-run.md`). **Reading taken:** decision 0128's comparison against no orders needs arrangements (b) to (e), which item 8 does not make with command levels on, so it is not run; the conditions of arrangement (a) are.

| Condition (decision 0128, the week) | Levels on | Third run (levels off) |
| --- | --- | --- |
| Every invariant, every seed | 1,000 of 1,000 | 1,000 |
| No staff order or command refused; no commander's command of day 15 refused | 0 refused (0 not as held, 0 other); none | 0; none |
| A search and a patrol on 5 of 7 days (Allied, Japanese) | 1,000, 1,000 | 1,000, 1,000 |
| A strike or raid in the week (Allied, Japanese) | 1,000, 1,000 | 1,000, 1,000 |
| Supply landed at its base (Allied, Japanese) | 1,000, 1,000 | 1,000, 1,000 |
| Same start, same game | identical | identical |

Medians, days 15 to 21, arrangement (a):

| | Levels on | Third run |
| --- | --- | --- |
| Allied margin | 53.70 | 53.42 |
| Allied aircraft lost | 24 | 24 |
| Allied ships sunk or out of action | 0 | 0 |
| Supply landed at Henderson Field | 3,220.8 t | 3,220.8 t |
| Henderson Field open to the Allied side (days) | 7 | 7 |
| Japanese aircraft lost | 26 | 26 |
| Japanese ships sunk or out of action | 1 | 1 |
| Japanese supply landed at Taivu and west of the Lunga | 119.08 t | 119.08 t |

The week with command levels on is, in these measures, the week with them off: every condition holds on every seed and every median but the margin is the same. Late and lost reports cost the staffs little here; the week's decisions rest on bases, supply runs and raids, whose targets are known places.

## Condition 7, Savo Island (item 14): reported, not judged

Savo Island as scripted, command levels on, seeds 1 to 1,000 (`savo.py`, outside the repository). The Allied commands are the area command, the southern group (task force 2) and the northern group (task force 3).

- **The Hudson's report** of the cruiser force, the contact the scenario gives the Allied side at the start, is held by every Allied command from the start, on every seed. **Reading taken:** it is a sighting of the cruiser force, so each command first holds a sighting at the start, and the force that fights it holds one before the fight on 1,000 of 1,000 seeds, the southern group and the northern group alike.
- **The first sighting made in play** (an Allied report whose cause names the cruiser force or the part of it sent home), and the pulse each command first holds it:

| Command | Night of day 1 | Morning of day 1 | Never |
| --- | --- | --- | --- |
| The area command | 931 | 68 | 1 |
| Southern group (task force 2) | 1,000 | 0 | 0 |
| Northern group (task force 3) | 997 | 3 | 0 |

- **Before its own fight,** counting a sighting made in play only (held in an earlier pulse, or in the same pulse at an event before the fight's first): the southern group on 0 of 1,000 seeds (its fight is the first, and the first sighting made in play comes out of it); the northern group on 850 of the 999 seeds on which it fights (it fights in the night on 997 and in the morning on 2): the southern group's report of its fight reaches it, sideways or down, before its own fight begins.

## Also reported (item 16)

- **Strikes at ships, levels on against off** (seeds with at least one, the second proof's measure): Allied 818 on, 822 off; Japanese 976 on, 979 off. The median margin, Allied: 3.75 on, 4.825 off. (The off file plays as `second-proof/05a`; since the third run, whose Allied count was 767, decisions 0133 to 0137 have changed the game.)
- **Passings, levels on, the Eastern Solomons over 1,000 seeds:** 106,724 passings to a listener, 74,576 on time (69.9%), 26,865 a pulse late (25.2%), 5,283 never (4.9%). "Never" here is the draw only; a late passing due after the one-day game ends is counted late.
- **The bold commanders** (`05c_eastern_solomons_bold.toml`): the hash differs on every seed (the commanders are in the state), but every margin is the same as with no commander on all 1,000 seeds, and on seeds 1, 2 and 3 every event is the same. A rating is read only in the closing of a carrier force on a contact of its own, weighed on the staff's morning before the first pulse (decision 0137); in this one-day battle no force holds a contact then, so the rating never acts. Strikes at ships: Allied 818, Japanese 976, as with no commander.

## A model in a seat (item 17): reported, not judged

Kept in `crates/sim-cli/tests/recorded_games/stage_2b_seat_sonnet/`. One game of `05a_eastern_solomons.toml` on seed 1, a Sonnet model (`claude-sonnet-5-5`) in the seat of task force 1, Task Force 16 (Enterprise), through `sim-cli mcp ... --seat allied --at task_force:1`, with no tool but the game's; the staff served every other Allied command and the Japanese side. It was told what it commands, that it sees its own units exactly and the enemy only as its own force's contacts, that an order it gives a unit holds that unit for the rest of the game, and to play the day.

- **What it was shown:** one briefing: its own force at (7,-5) and the five groups aboard Enterprise, numbered from 1 in the seat, each at rest (the staff orders its units when the day is resolved); no contact, no base, no objectives ("none given to this seat").
- **What it ordered** (one call): VS-5 and the first VT-3 group to search, each with an arc; VB-6 and the VT-3 attack group to the naval strike mission; VF-6 to the combat air patrol with a share of 0.7. The two arcs were refused (`range_nm` must be text), and it ended the day without reading the refusals.
- **What came of it:** each of its orders held the group against the staff, so Enterprise's groups did not search as the carrier doctrine has them do, and its two search groups, with no arc, flew nothing. Its force held one contact all day, Henderson Field's report of the carrier striking force at (8,-11), 223 nm away and beyond its strike groups' reach, and it launched nothing. The Japanese strikes put Enterprise's flight deck out in the morning (in the staffed game of the same seed, in the afternoon). Read from the export by the scale's counts: Allied about 2.30 points and Japanese about 5.65, against 1.55 and 5.35 in the staffed game, both a minor defeat for the Allied side. The game is one day long, so the model saw nothing after `end_day`, and no result is given to a seat that is not the area's.
- **Was it given anything its command did not hold?** No. Task force 1's picture in the export is empty at the start, and the briefing said "Contacts held: none"; the briefing named only task force 1's own ships and groups.
- **A first attempt was not kept:** the server started by the model's client was stopped before it wrote its saved game and transcript, so the first game's calls were lost. Its account is kept as written (`first_attempt_account.md`), not verifiable and not the game reported; the game was played again from the start with the calls recorded, and the transcript and saved game were made by playing those calls again through the same command.

## Every reading taken

1. Condition 4's five clauses as read above, with: no case from a night report (none arose); the area command never A or B; held as the `--report` tool has it, not the export's listeners; clause 5 as written, a launch of B at another hex not stopping a case.
2. A case is one report, one A and one B.
3. Condition 5's tie-breaks in the pick (A's launch, then B's name; B's lowest-numbered group meeting clause 4), and the two commands named to the reader in the order of their names.
4. Condition 8's "a staffed day" as `time`'s "one day of the slice", early and late.
5. Condition 6 run for arrangement (a) only; the comparison against no orders needs files item 8 does not make.
6. Condition 7: the Hudson's report held at the start is a sighting of the cruiser force; the first sighting made in play is given beside it.
7. The Savo file's six data paths moved one folder up, so that the file names the same data from the new folder.

## The stricter count, made after the run (Nov 17, 2026)

The reading of condition 4 fixed before the run says that no group of the second command launches **at that hex** in the pulse. On 59 of the 233 seeds every case has the second command striking in that pulse at another hex, often the same enemy force reported from another place. The architect, who wrote the reading, judged it looser than he meant, told the designer, and on the designer's word counted again from the outputs kept, with nothing run again: a case counts only where the second command launches nothing at all in that pulse.

**By the stricter count 174 of 1,000 seeds have a case** (193 cases; Allied on 118 seeds, Japanese on 67), against the mark of 200. By the reading fixed before the run it is 233 and the condition passes; by the stricter one it misses by 26 seeds. Both numbers stand in the record. What is made of the difference is the designer's.
