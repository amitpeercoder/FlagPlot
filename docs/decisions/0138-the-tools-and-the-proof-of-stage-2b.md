# 0138: The tools and the proof of stage 2b: step 5, and the test plan written before the run

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 2b, that the designer confirmed on Nov 16, 2026 with its pass marks (decision 0132, step 5 of its five and "What it must pass"). It says what tools are built, and then what is run, how each condition is read and what passes, before any of it is run. Nothing is built and nothing is run.

Summary: Step 5 of command levels: the tools that show who knew what (a command's picture and held state, when a report reached a command, why a group did not launch, refusals counted by cause, a seat at any command for a model, the viewer showing one command's picture beside the truth), and the test plan of the stage's proof: the Eastern Solomons from intents with command levels on over a thousand seeds, with the reading of each of the designer's pass marks fixed before the run

Standing: Settled by the architect, Nov 17, 2026, inside decision 0132, whose pass marks the designer confirmed on Nov 16, 2026. Not built. Not run

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
